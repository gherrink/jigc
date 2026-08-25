"""Refuse to measure unless the rig was verified for THIS image and THIS binary.

`verify-image.sh` and `verify-pair.sh` exist, work, and are run by operator
discipline. The rig this directory harvests from has a blunt note about exactly
that shape:

    `verify_isolation.py` was described as the step that gates everything and was
    invoked by nothing: the runner never read its record, so the gate was operator
    discipline of exactly the kind that let two apparatus defects reach
    publication.

So the record is *read*, and a round that is not covered by one is refused.

**Keyed on the image ID, never the tag.** A tag is mutable; a rebuild under the
same tag is a different image, and `build-image.sh` bakes `JIGC_SHA` precisely
because `jigc --version` cannot tell two builds apart — two commits in this
repository's history both stamp `1.0.0-rc.10`, one pre-M48 and one containing all
of it.

**Checked again afterwards.** A CLI that self-updates mid-round means the runs are
not one measurement, and the eval rig aborts on that rather than reporting it.
"""
from __future__ import annotations

import dataclasses
import json
import pathlib
import subprocess
from typing import Optional


@dataclasses.dataclass(frozen=True)
class Identity:
    """What a round is running on, read from the image rather than declared."""
    tag: str
    image_id: str
    jigc_sha: str
    jigc_version: str
    cli_version: str

    def as_dict(self) -> dict:
        return dataclasses.asdict(self)


def _docker(*args: str) -> str:
    got = subprocess.run(["docker", *args], capture_output=True, text=True)
    if got.returncode != 0:
        raise SystemExit(f"docker {' '.join(args)} failed: {got.stderr.strip()[:200]}")
    return got.stdout.strip()


def identity(tag: str) -> Identity:
    """Read the identity out of the image. Nothing here is taken on trust."""
    image_id = _docker("image", "inspect", "--format", "{{.Id}}", tag)
    env = _docker("image", "inspect", "--format",
                  "{{range .Config.Env}}{{println .}}{{end}}", tag)
    jigc_sha = ""
    for line in env.splitlines():
        if line.startswith("JIGC_SHA="):
            jigc_sha = line.split("=", 1)[1].strip()
    if not jigc_sha:
        raise SystemExit(
            f"refusing: {tag} carries no JIGC_SHA — it was not built by "
            "build-image.sh, and `jigc --version` cannot tell two builds apart.")
    jigc_version = _docker("run", "--rm", "--entrypoint",
                           "/usr/local/bin/jigc", tag, "--version")
    cli_version = _docker("run", "--rm", "--entrypoint", "claude", tag, "--version")
    return Identity(tag, image_id, jigc_sha, jigc_version, cli_version)


def write_record(path: pathlib.Path, ident: Identity, *,
                 checks: dict, note: str = "") -> None:
    """Persist what was verified, against what. `checks` is the verifier's own output."""
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps({
        "identity": ident.as_dict(),
        "checks": checks,
        "note": note,
    }, indent=2) + "\n")


def gate(record: pathlib.Path, tag: str) -> Identity:
    """Refuse a round the record does not cover. Returns the identity it certified.

    Three separate refusals, because each has its own failure story:

      * **no record** — the verifier was never run, or its output was never kept;
      * **a different image** — the tag was rebuilt, so every check in the record
        describes a tree that is no longer there;
      * **a failed check** — the record exists and says the rig did not verify,
        which is the case a discipline-based gate is most likely to walk past.
    """
    if not record.is_file():
        raise SystemExit(
            f"refusing: no isolation record at {record}. Run the verifier and keep "
            f"its output:\n  ./completions/trial-harness/verify-image.sh {tag} <version>")
    kept = json.loads(record.read_text())
    live = identity(tag)
    was = kept.get("identity", {})

    problems = []
    if was.get("image_id") != live.image_id:
        problems.append(
            f"the record was made against image {was.get('image_id', '?')[:19]}… and "
            f"{tag} is now {live.image_id[:19]}… — a tag is mutable, so every check "
            "in that record describes a different tree")
    if was.get("jigc_sha") != live.jigc_sha:
        problems.append(f"jigc sha {was.get('jigc_sha', '?')[:12]} != {live.jigc_sha[:12]}")
    failed = [name for name, ok in (kept.get("checks") or {}).items() if not ok]
    if failed:
        problems.append(f"the record itself reports failed checks: {failed}")
    if not kept.get("checks"):
        problems.append("the record carries no checks — a gate that passes on "
                        "nothing is not a gate")

    if problems:
        raise SystemExit("the isolation record does not cover this round:\n  "
                         + "\n  ".join(problems))
    return live


def assert_unchanged(before: Identity, tag: str) -> None:
    """After the round: the same image, or the runs were not one measurement."""
    after = identity(tag)
    if after.image_id != before.image_id or after.cli_version != before.cli_version:
        raise SystemExit(
            "the image changed under the round:\n"
            f"  before: {before.image_id[:19]}… / {before.cli_version}\n"
            f"  after : {after.image_id[:19]}… / {after.cli_version}\n"
            "These runs are not one measurement; discard and re-run.")
