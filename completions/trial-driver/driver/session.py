"""Record a conversation once, freeze it, and fork it as many times as needed.

This is the mechanism harvested from `~/skills/agentic-coding/evals/lib/seedrun.py`,
and the first correction to make is that **nothing is synthesised**. A real
multi-turn conversation is driven, the CLI's own transcript is frozen, and every
later arm resumes *that file*:

  1. turn 1 runs with `--session-id <uuid>`, which fixes the id up front so
     nothing has to be parsed back out;
  2. turns 2..n run with `--resume <uuid>` and **no** `--fork-session`, so they
     extend the one conversation;
  3. the transcript, the project tree and a manifest are frozen;
  4. a measured run stages the transcript back into the CLI's store and resumes
     with `--resume <uuid> --fork-session`, which mints a new id and rewrites the
     whole ancestry under it.

**Why it is worth having here.** `cue-card-postmortem.md`'s recommended instrument
(design E, the abandoned task) needs a worker handed a doc *already staged by
someone who left*. Today that is a setup script driving the binary. A seed chain
makes the same trick available one layer up, on **conversational** state: drive an
arc to a chosen point, freeze it, and hand N arms an identical mid-arc starting
position. It also makes rehearsal cheap, which is the act rule 4 says must be paid
for before a trial and which previously cost a whole session.

**Every turn is a fresh container**, so the transcript has to travel between turns
or there is no conversation at all. On a host that copy is redundant; here it is
the entire mechanism.

**The failure that manufactures a result out of nothing.** `--resume` on a session
the CLI has never seen does **not** fail — it silently starts a fresh conversation.
Every other channel looks identical to a real fork. `seed_marker` plus
`Observation.seed_inherited` is the only thing that sees it, and `freeze()` refuses
to write a fixture it could not verify.
"""
from __future__ import annotations

import dataclasses
import hashlib
import json
import pathlib
import shutil
import subprocess
import uuid
from typing import Optional, Sequence

#: Claude Code's per-directory key: the absolute path with `/` replaced by `-`.
#: The container always works at `/work`, so its store is always `-work`.
CONTAINER_WORKDIR = "/work"


def project_slug(path: str) -> str:
    return str(path).replace("/", "-")


CONTAINER_SLUG = project_slug(CONTAINER_WORKDIR)

REPO = pathlib.Path(__file__).resolve().parents[3]
RUN_SESSION = REPO / "completions" / "trial-harness" / "run-session.sh"


@dataclasses.dataclass(frozen=True)
class Frozen:
    """A frozen conversation: the transcript, the tree it happened in, and a manifest."""
    root: pathlib.Path

    @property
    def manifest(self) -> dict:
        return json.loads((self.root / "manifest.json").read_text())

    @property
    def transcript(self) -> pathlib.Path:
        return self.root / "session.jsonl"

    @property
    def project(self) -> pathlib.Path:
        return self.root / "project"

    @property
    def session_id(self) -> str:
        return self.manifest["session_id"]

    @property
    def seed_marker(self) -> str:
        return self.manifest["seed_marker"]

    def verify(self) -> list[str]:
        """Refuse a fixture that has drifted. Existence was never enough."""
        bad = []
        if not self.transcript.is_file():
            return [f"{self.transcript} is missing — there is nothing to resume"]
        got = hashlib.sha256(self.transcript.read_bytes()).hexdigest()[:16]
        if got != self.manifest.get("session_sha"):
            bad.append(f"transcript sha {got} != frozen {self.manifest.get('session_sha')}")
        if self.seed_marker not in self.transcript.read_text(errors="replace"):
            bad.append("the frozen transcript does not contain its own seed marker")
        return bad


def _sha16(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()[:16]


def _stage_home(stage: pathlib.Path, session_id: str,
                transcript: pathlib.Path) -> pathlib.Path:
    """Build the `.claude` tree that has to be inside the container before the CLI starts.

    Refuses a missing transcript rather than staging an empty store: `--resume`
    would then resume nothing, begin cold, and produce a plausible arm.
    """
    if not transcript.is_file():
        raise SystemExit(
            f"refusing: {transcript} is not a file, so `--resume {session_id}` would "
            "resume nothing and the fork would silently begin cold.")
    project = stage / ".claude" / "projects" / CONTAINER_SLUG
    project.mkdir(parents=True, exist_ok=True)
    shutil.copy2(transcript, project / f"{session_id}.jsonl")

    # `docker cp` preserves the HOST uid (501 here); the container user is `node`
    # (1000), and the image's entrypoint chowns `/work` but not `/home/node`. So a
    # staged transcript lands as `uid 501, -rw-------` and the CLI cannot read it —
    # it then reports "No conversation found with session ID" and the turn dies.
    #
    # Fixed with modes rather than ownership because ownership cannot be set here:
    # the container is not running yet (create → cp → start), so there is no
    # `docker exec` to chown from, and teaching the entrypoint would mean rebuilding
    # a pinned image — which invalidates every isolation record keyed on its digest.
    # The container is single-use and destroyed at the end of the turn.
    for path in [stage, *stage.rglob("*")]:
        path.chmod(0o777 if path.is_dir() else 0o666)
    return stage


#: What `run-session.sh` writes into an out-dir alongside the corpus. When a turn's
#: out-dir becomes the next turn's corpus, these must NOT travel with it — they are
#: the rig's own evidence, and copying them into `/work` would plant apparatus
#: artefacts in the tree under test, in every turn after the first.
_EVIDENCE_NAMES = frozenset(
    (".session-transcript", "PROVENANCE.txt", "stream.jsonl", "stderr.txt")
)


def carry_forward(out: pathlib.Path, dest: pathlib.Path) -> pathlib.Path:
    """The corpus a turn left behind, without the rig's own droppings.

    `.git` travels: the tree under test has history, and the next turn must see it.
    """
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    for child in out.iterdir():
        if child.name in _EVIDENCE_NAMES:
            continue
        if child.is_dir():
            shutil.copytree(child, dest / child.name, symlinks=True)
        else:
            shutil.copy2(child, dest / child.name)
    return dest


def _find_transcript(out: pathlib.Path, session_id: str) -> Optional[pathlib.Path]:
    """The transcript `run-session.sh` copied out, for exactly this session id."""
    root = out / ".session-transcript"
    if not root.is_dir():
        return None
    hits = list(root.rglob(f"{session_id}.jsonl"))
    return hits[0] if hits else None


def _drive(corpus: pathlib.Path, out: pathlib.Path, prompt: str, *,
           tag: str, extra: Sequence[str], home: Optional[pathlib.Path],
           strict: bool) -> int:
    """One headless turn, through `run-session.sh` — never around it."""
    prompt_file = out.parent / f"{out.name}.prompt.txt"
    prompt_file.parent.mkdir(parents=True, exist_ok=True)
    prompt_file.write_text(prompt)
    argv = [str(RUN_SESSION), "--headless", "--prompt-file", str(prompt_file)]
    if strict:
        argv.append("--strict-permissions")
    if home is not None:
        argv += ["--home", str(home)]
    for value in extra:
        argv += ["--arg", value]
    argv += [str(corpus), str(out), tag]
    rc = subprocess.run(argv, cwd=str(REPO)).returncode
    if rc != 0:
        return rc
    # `run-session.sh` exits 0 even when the driven arm exits non-zero — deliberately,
    # because for a scripted arm that "is data, not necessarily failure". For a seed
    # turn it is neither: a failed turn leaves the next `--resume` with nothing. So the
    # arm's OWN exit code is read back out of the provenance it writes.
    provenance = out / "PROVENANCE.txt"
    if provenance.is_file():
        for line in provenance.read_text().splitlines():
            if line.startswith("exit-code"):
                return int(line.split()[1])
    return rc


def seed(corpus: pathlib.Path, turns: Sequence[str], frozen: pathlib.Path, *,
         tag: str = "jigc-gate:rc11", work: Optional[pathlib.Path] = None,
         strict: bool = False, force: bool = False) -> Frozen:
    """Drive `turns` as one conversation, then freeze it.

    Seeded **once**, because a conversation regenerated per repetition is a
    different test case each time and any variation later credited to the product
    may have originated in the seed.
    """
    if frozen.exists() and not force:
        raise SystemExit(f"{frozen} exists — pass force=True to re-seed (and record why)")
    if not turns:
        raise SystemExit("refusing: a seed with no turns is not a conversation")

    work = work or frozen.parent / f"{frozen.name}-work"
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)

    session_id = str(uuid.uuid4())
    marker = turns[0].strip()
    current_corpus = corpus
    transcript: Optional[pathlib.Path] = None

    for i, text in enumerate(turns, 1):
        out = work / f"turn{i:02d}"
        stage = None
        if i == 1:
            extra = ["--session-id", session_id]
        else:
            extra = ["--resume", session_id]
            stage = _stage_home(work / f"stage{i:02d}", session_id, transcript)

        rc = _drive(current_corpus, out, text, tag=tag, extra=extra,
                    home=stage, strict=strict)
        if rc != 0:
            raise SystemExit(f"seed turn {i} failed (rc={rc}); the seed is void, not weak")

        found = _find_transcript(out, session_id)
        if found is None:
            # Loudly, and at the turn that lost it. The next turn would resume
            # nothing — which does not fail — so the seed would silently become a
            # sequence of unrelated one-turn conversations.
            raise SystemExit(
                f"turn {i} produced no transcript for {session_id} under {out}. "
                "The next turn would resume nothing and the seed would silently "
                "become a sequence of unrelated one-turn conversations.")
        transcript = found
        # The tree the next turn works in is the tree this one left behind — minus
        # the rig's evidence, which would otherwise be planted in /work.
        current_corpus = carry_forward(out, work / f"corpus{i + 1:02d}")

    assert transcript is not None
    if frozen.exists():
        shutil.rmtree(frozen)
    frozen.mkdir(parents=True)
    shutil.copy2(transcript, frozen / "session.jsonl")

    # `.git` is deliberately not frozen: git objects are mode 0444, and copying
    # them in makes the tree unrestorable on the second restore. History is rebuilt
    # from the template instead.
    (frozen / "project").mkdir()
    for src in sorted(current_corpus.rglob("*")):
        if src.is_file() and ".git" not in src.parts:
            rel = src.relative_to(current_corpus)
            dst = frozen / "project" / rel
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, dst)

    (frozen / "manifest.json").write_text(json.dumps({
        "session_id": session_id,
        "project_slug": CONTAINER_SLUG,
        # So a later round can prove it is resuming the same conversation, not
        # merely one with the same id. Existence was the only check before.
        "session_sha": _sha16((frozen / "session.jsonl").read_bytes()),
        "seed_marker": marker,
        "turns": len(turns),
        "corpus_src": str(corpus),
        "tag": tag,
        "permission_mode": "default" if strict else "bypassPermissions",
        "transport": "container",
    }, indent=2) + "\n")

    fixture = Frozen(frozen)
    problems = fixture.verify()
    if problems:
        raise SystemExit("the seed was written but does not verify:\n  "
                         + "\n  ".join(problems))
    return fixture


def fork(fixture: Frozen, prompt: str, out: pathlib.Path, *,
         corpus: pathlib.Path, tag: str = "jigc-gate:rc11",
         strict: bool = False) -> pathlib.Path:
    """Resume the frozen conversation and drive one measured turn.

    `corpus` is restored by the caller — the fixture's `project/` holds the tree
    the seed left behind, and history is rebuilt from the template, because the
    frozen copy carries no `.git`.
    """
    problems = fixture.verify()
    if problems:
        raise SystemExit("refusing to fork an unverified fixture:\n  "
                         + "\n  ".join(problems))
    stage = _stage_home(out.parent / f"{out.name}-stage",
                        fixture.session_id, fixture.transcript)
    rc = _drive(corpus, out, prompt, tag=tag,
                extra=["--resume", fixture.session_id, "--fork-session"],
                home=stage, strict=strict)
    if rc != 0:
        print(f"  (the forked turn exited {rc} — that is data, not necessarily failure)")
    return out


def forked_id(stream: pathlib.Path) -> Optional[str]:
    """The id `--fork-session` minted, read off the first event that carries one."""
    if not stream.is_file():
        return None
    for raw in stream.read_text(errors="replace").splitlines():
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            event = json.loads(raw)
        except json.JSONDecodeError:
            continue
        got = event.get("session_id")
        if got:
            return str(got)
    return None
