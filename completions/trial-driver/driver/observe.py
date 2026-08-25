"""Read a session's primary channels off its own evidence.

This replaces reading `.jigc/logs/invocations.jsonl` by eye, which is how every
count in the 1.0.0-gate record was produced and how one of them came out wrong:
`session-findings.md` carried B1 at five read-backs for a day while the log said
three the whole time. `evidence/README.md` promoted the consequence to a standing
rule — *read the logs, not the record, when the two disagree* — and a reader that
cannot be disagreed with is the only way to keep it.

Two sources, of very different quality, and the difference is stated rather than
smoothed over:

  * **the invocation log** is a *product* surface with a pinned record shape
    (`timestamp, argv, exit_code, duration_ms, output_bytes, binary_version,
    finding_codes, error_code`). VERB and VERB-ADJACENT are read from it and are
    **exact**.
  * **the session transcript** is Claude Code's own store, whose format
    `knowledge/10` warns "changes between versions". FILESYSTEM is read from it
    and is a **heuristic**: `filesystem_reads()` returns the evidence it matched
    so a human can check the call, and `Observation.filesystem_is_heuristic` is
    true so no caller can forget.

Nothing here reaches back into a corpus. Every field is computed at construction
from bytes that are already archived, so re-running the reader over a finished
session yields what it yielded on the day — the property that makes a corrected
rule re-appliable to a bought round instead of forcing another one.
"""
from __future__ import annotations

import dataclasses
import datetime as _dt
import json
import pathlib
import re
from typing import Callable, Iterable, Optional

from . import channels

#: Path fragments that mean "a managed jigc document" for the FILESYSTEM channel.
#: The staged-document workbench first, then the default homes a pack places docs
#: at. Deliberately broad: over-matching produces a FILESYSTEM hit a human then
#: dismisses, while under-matching produces a silent NEITHER — and §3.3's whole
#: point is that the non-VERB column must not be flattered.
MANAGED_PATH_HINTS: tuple[str, ...] = (
    "/.jigc/", ".jigc/",
    "/docs/", "docs/",
    "/decisions/", "decisions/",
    "VISION.md", "CHANGELOG.md", "DECISIONS.md",
)

#: Paths that sit under a hint above but are **not** managed documents, and whose
#: inclusion would flatter the FILESYSTEM column rather than the reverse.
#:
#: `.jigc/worktrees/**` is the fan-out *code* worktree. A worker editing
#: `src/store.ts` inside it is doing exactly the work it was asked to do, through
#: the workspace jigc provisioned for it — that is not the adapter bypass §3.3
#: measures. Left in, it scored the B2 `pinegrove` session at 38 FILESYSTEM reads
#: of which every single one was ordinary code work; the true figure is 0.
#: Found by reading the matched evidence rather than trusting the count.
MANAGED_PATH_EXCLUSIONS: tuple[str, ...] = (
    ".jigc/worktrees",
)

#: Programs that read a file's *contents*. `find`, `ls`, `git status` and
#: `git check-ignore` are deliberately absent: they enumerate or classify names,
#: and §3.3's FILESYSTEM channel is about reading a document's bytes around the
#: CLI, not about noticing that `.jigc/` exists.
_SHELL_READERS = frozenset(
    ("cat", "sed", "head", "tail", "less", "more", "grep", "rg", "awk", "nl", "bat")
)

#: Statement separators, and — separately — the pipe. The two are split in
#: sequence rather than together because position in a *pipeline* decides whether
#: a reader touches a file at all.
_STATEMENT = re.compile(r"(?:\|\||&&|;|\n)")


def _segments(command: str) -> Iterable[tuple[str, str]]:
    """Yield `(program, segment)` for each command that could read a **file**.

    Only the head of a pipeline can. `find … | grep -v "/work/.jigc/worktrees"`
    has `grep` reading *stdin*, and counting it scored a FILESYSTEM read in the
    archive for a command that was filtering a directory listing — the last of
    three false positives this function was rewritten to kill.
    """
    for statement in _STATEMENT.split(command):
        stage = statement.split("|")[0].strip()
        if not stage:
            continue
        words = stage.split()
        while words and words[0] in ("sudo", "command", "time"):
            words = words[1:]
        if not words:
            continue
        yield words[0], stage


@dataclasses.dataclass(frozen=True)
class Invocation:
    """One line of the invocation log, typed."""
    timestamp: _dt.datetime
    argv: tuple[str, ...]
    exit_code: int
    duration_ms: int
    output_bytes: int
    binary_version: str
    finding_codes: tuple[str, ...]
    error_code: Optional[str]

    @property
    def line(self) -> str:
        return " ".join(self.argv)


@dataclasses.dataclass(frozen=True)
class Read:
    """One transcript action the FILESYSTEM heuristic matched, kept as evidence."""
    tool: str
    detail: str
    path: str


@dataclasses.dataclass(frozen=True)
class Observation:
    """A session's channels, flat and complete at construction.

    Flat on purpose. A predicate that could reach back into a corpus would grade
    differently on a re-read than it did on the day, which defeats the reason the
    raw channels are archived at all.
    """
    session: str
    records: int
    verb: int
    adjacent: int
    shipped_adjacent: int
    filesystem: int
    log_missing: bool
    log_unreadable: bool
    transcript_missing: bool
    binary_versions: tuple[str, ...]
    nonzero_exits: int
    error_codes: tuple[str, ...]
    verb_lines: tuple[str, ...]
    adjacent_lines: tuple[str, ...]
    filesystem_reads: tuple[Read, ...]
    filesystem_is_heuristic: bool = True

    #: The CLI itself failed — a non-zero exit from the `claude` process, not from
    #: a jigc invocation inside it. Distinct from `nonzero_exits`, which counts
    #: jigc's own refusals and is frequently a *result* rather than a failure.
    rc_failed: bool = False

    #: Whether this run was supposed to resume a frozen seed. False for a cold
    #: session, where `seed_inherited` is meaningless and must not void anything.
    seed_expected: bool = False

    #: VERB / VERB-ADJACENT calls that actually SUCCEEDED.
    #:
    #: `verb` counts what §3.3 registers — "`jigc doc show … --task …` **appears in
    #: the invocation log**" — which is appearances, not successes. Two archived
    #: sessions contain a failed `doc show … --task` (rosewater ×2, b3-bypass ×2),
    #: and the 1.0.0-gate record counts them, so `verb` must too or the reader stops
    #: reproducing the table it is checked against.
    #:
    #: But a failed read returns no document bytes, so counting it as a read-back is
    #: a different measurement wearing the same name. Both are carried, the gap is
    #: reported, and **the choice is the protocol's, not the reader's** — exactly as
    #: with `adjacent_counter_gap`. A reader that quietly redefined a registered
    #: measurement would be the failure this package exists to prevent.
    verb_succeeded: int = 0
    adjacent_succeeded: int = 0

    #: Successful doc-authoring writes. Zero means **no read-back occasion ever
    #: existed**, which §3.3 keeps distinct from NEITHER: "The session never reached
    #: an authoring step — no occasion existed; unmeasured, not NEITHER." Found
    #: missing by the first live headless arm, whose arc truncated before it
    #: authored anything and which the cascade would otherwise have scored as a
    #: worker that declined to read.
    authoring_writes: int = 0

    #: The turn ended asking the operator for something — in practice a denied
    #: write under the adopter's real permission set. Reported beside the outcome
    #: rather than scored as one: a halt *after* authoring is still measurable, so
    #: this is metadata, not a cascade row.
    halted_for_human: bool = False

    #: Whether the fork actually inherited the seed. The only channel that sees a
    #: `--resume` which silently began a *fresh* conversation: every other channel
    #: looks identical, which is what makes a dead fixture yield a plausible arm.
    seed_inherited: bool = True

    # NOTE: there is deliberately no `outcome` property here.
    #
    # There was one, and it was a genuine defect of exactly the kind `cascade.py`
    # claims to have designed out: "there is no second place where a row can be
    # applied". It WAS that second place — a private re-implementation of the
    # cascade's ordering that knew nothing about `authoring_writes`,
    # `seed_inherited` or `rc_failed`, so it scored the first live strict arm as
    # `NEITHER` (a claim about the worker) while the cascade correctly voided it as
    # `unmeasured — no authoring occasion existed`.
    #
    # An Observation is now facts only. Scoring belongs to `cascade.grade()`, in
    # one list, in the registered order — which is the whole reason that module
    # exists.

    @property
    def failed_reads(self) -> int:
        """VERB calls that appear in the log but returned no document.

        Non-zero means the registered count and the "a read-back happened" reading
        of it disagree for this session. Reported, never resolved here.
        """
        return self.verb - self.verb_succeeded

    @property
    def adjacent_counter_gap(self) -> int:
        """How far `run-session.sh:174` undercounts §3.3's own definition.

        Non-zero means the shipped counter and the protocol disagree about this
        session. Reported, never resolved here: which one is right is a protocol
        question.
        """
        return self.adjacent - self.shipped_adjacent


#: Invocations that put author-supplied prose into a managed doc. A read-back
#: occasion exists only once one of these has succeeded.
_AUTHORING = (
    ("doc", "create"), ("doc", "author"), ("doc", "set-slot"),
    ("doc", "set-field"), ("doc", "add-item"), ("doc", "retitle-item"),
)


def _is_authoring(argv: list[str]) -> bool:
    """A write, not a *question about* a write.

    `jigc doc author --help` matches the verb pair and writes nothing; counting it
    reported an authoring occasion for a session that had none, which is precisely
    the `unmeasured`-vs-`NEITHER` distinction the row exists to preserve. Caught on
    the first live arm this predicate ran against.
    """
    if any(a in ("--help", "-h") for a in argv):
        return False
    v = channels.verb_tokens(argv)
    return any(tuple(v[:2]) == p for p in _AUTHORING)


def _parse_ts(raw: str) -> _dt.datetime:
    return _dt.datetime.strptime(raw, "%Y-%m-%dT%H:%M:%SZ").replace(
        tzinfo=_dt.timezone.utc
    )


def read_log(path: pathlib.Path) -> list[Invocation]:
    """Parse an invocation log. Raises on a malformed line rather than skipping it.

    Skipping is what a tolerant reader would do, and it is wrong here: a line the
    reader cannot parse is a line whose invocation is missing from every count,
    silently, in the direction of a smaller denominator.
    """
    out: list[Invocation] = []
    for n, raw in enumerate(path.read_text().splitlines(), 1):
        raw = raw.strip()
        if not raw:
            continue
        try:
            r = json.loads(raw)
        except json.JSONDecodeError as exc:
            raise ValueError(f"{path}:{n} is not JSON: {exc}") from exc
        out.append(Invocation(
            timestamp=_parse_ts(r["timestamp"]),
            argv=tuple(r["argv"]),
            exit_code=int(r["exit_code"]),
            duration_ms=int(r.get("duration_ms", 0)),
            output_bytes=int(r.get("output_bytes", 0)),
            binary_version=str(r.get("binary_version", "")),
            finding_codes=tuple(r.get("finding_codes") or ()),
            error_code=r.get("error_code"),
        ))
    return out


def _tool_uses(path: pathlib.Path) -> Iterable[tuple[str, dict]]:
    """Every `tool_use` block in a transcript, tolerant of shape drift.

    Tolerant *here* and strict in `read_log`, and the asymmetry is deliberate: the
    transcript is a third-party format that changes between CLI versions, so a
    reader that crashed on an unexpected event would fail a session for the wrong
    reason. The invocation log is ours and is pinned.
    """
    for raw in path.read_text(errors="replace").splitlines():
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            event = json.loads(raw)
        except json.JSONDecodeError:
            continue
        message = event.get("message")
        if not isinstance(message, dict):
            # `{"type":"system","subtype":"permission_denied",...,"message":"..."}`
            # carries `message` as a *string*; `.get` on it raises.
            continue
        content = message.get("content")
        if not isinstance(content, list):
            continue
        for block in content:
            if isinstance(block, dict) and block.get("type") == "tool_use":
                yield str(block.get("name")), (block.get("input") or {})


#: Roughly, a filesystem path inside a shell word. Used to test hints and
#: exclusions PER PATH rather than against a whole command, which is the difference
#: between "this segment mentions a worktree" and "every path in it is a worktree".
_PATHISH = re.compile(r"[^\s'\"|;&<>()]*/[^\s'\"|;&<>()]*")


def _looks_managed(text: str) -> Optional[str]:
    """The hint a managed path in `text` matched, or None.

    Exclusions are applied **per path, not per segment.** Testing the whole string
    meant one excluded path suppressed every other path beside it, so

        grep -n "cap" /work/docs/decisions/x.md /work/.jigc/worktrees/t/src/a.ts

    scored as no managed read at all — a false NEGATIVE on the channel §3.3 says
    must not be flattered. Found by the cross-model review, not by a failing test.
    """
    candidates = [w for w in _PATHISH.findall(text) if w]
    # A bare filename with no slash (`cat CHANGELOG.md`) has no path-ish token, so
    # fall back to the whole string — but only when nothing path-ish was found at
    # all, or the fallback would reintroduce exactly the bug above.
    if not candidates:
        candidates = [text]
    for candidate in candidates:
        if any(x in candidate for x in MANAGED_PATH_EXCLUSIONS):
            continue
        for hint in MANAGED_PATH_HINTS:
            if hint in candidate:
                return hint
    return None


def filesystem_reads(path: pathlib.Path) -> list[Read]:
    """Direct reads of a managed doc or the staged workbench, as §3.3 defines them.

    A heuristic, and returned as evidence rather than as a number so the call can
    be checked by a human. Three exclusions are deliberate, each because leaving it
    in inflated the archive:

      * **a `jigc` invocation is never a bypass.** It is the adapter. Counting one
        because its argument text named a docs path is the inverse of the
        measurement.
      * **`git show` / `git status` and friends** read the committed store through
        git, or read names rather than bytes. Neither is the adapter bypass §3.3
        is about.
      * **`.jigc/worktrees/**`** is the fan-out code worktree — see
        `MANAGED_PATH_EXCLUSIONS`.

    **The bound this heuristic cannot close.** §3.3's FILESYSTEM channel is about
    a **managed** doc. Registration state is not in the transcript, so a read of a
    *foreign, never-adopted* file at a managed home matches here and should not
    score: `.jigc/AGENT.md` explicitly permits reading an unregistered doc before
    adopting it, and the 1.0.0-gate record dispositions exactly one such read that
    way (B3b `rosewater`, the planted foreign ADR). Hits are returned as evidence
    for that reason — the last call is a human's, made against the corpus.
    """
    hits: list[Read] = []
    for tool, inp in _tool_uses(path):
        if tool in ("Read", "Edit", "NotebookEdit"):
            target = str(inp.get("file_path") or "")
            hint = _looks_managed(target)
            if hint:
                hits.append(Read(tool=tool, detail=target, path=target))
        elif tool == "Bash":
            cmd = str(inp.get("command") or "")
            for program, segment in _segments(cmd):
                if program == "jigc" or program not in _SHELL_READERS:
                    continue
                hint = _looks_managed(segment)
                if hint:
                    hits.append(Read(tool="Bash", detail=segment[:200], path=hint))
    return hits


def observe(session: str, log: pathlib.Path,
            transcript: Optional[pathlib.Path] = None,
            *, rc_failed: bool = False, seed_expected: bool = False,
            seed_marker: Optional[str] = None,
            halted_for_human: bool = False) -> Observation:
    """Score one session from its archived channels.

    `seed_marker` is the seed's first turn text. When given, the forked
    transcript is searched for it and `seed_inherited` records the answer —
    the check that separates a real fork from one that began cold.
    """
    log_missing = not log.is_file()
    log_unreadable = False
    records: list[Invocation] = []
    if not log_missing:
        try:
            records = read_log(log)
        except (ValueError, KeyError):
            log_unreadable = True

    verb_recs = [r for r in records if channels.is_verb(list(r.argv))]
    adj_recs = [r for r in records if channels.is_adjacent(list(r.argv))]
    verb_lines = tuple(r.line for r in verb_recs)
    adj_lines = tuple(r.line for r in adj_recs)
    verb_ok = sum(1 for r in verb_recs if r.exit_code == 0)
    adj_ok = sum(1 for r in adj_recs if r.exit_code == 0)
    shipped = sum(1 for r in records if channels.is_shipped_adjacent(list(r.argv)))

    transcript_missing = transcript is None or not transcript.is_file()
    reads = tuple(filesystem_reads(transcript)) if not transcript_missing else ()

    authoring = sum(1 for r in records
                    if _is_authoring(list(r.argv)) and r.exit_code == 0)

    inherited = True
    if seed_expected and seed_marker:
        inherited = (not transcript_missing
                     and seed_marker in transcript.read_text(errors="replace"))

    return Observation(
        verb_succeeded=verb_ok,
        adjacent_succeeded=adj_ok,
        authoring_writes=authoring,
        halted_for_human=halted_for_human,
        rc_failed=rc_failed,
        seed_expected=seed_expected,
        seed_inherited=inherited,
        session=session,
        records=len(records),
        verb=len(verb_lines),
        adjacent=len(adj_lines),
        shipped_adjacent=shipped,
        filesystem=len(reads),
        log_missing=log_missing,
        log_unreadable=log_unreadable,
        transcript_missing=transcript_missing,
        binary_versions=tuple(sorted({r.binary_version for r in records if r.binary_version})),
        nonzero_exits=sum(1 for r in records if r.exit_code != 0),
        error_codes=tuple(sorted({r.error_code for r in records if r.error_code})),
        verb_lines=verb_lines,
        adjacent_lines=adj_lines,
        filesystem_reads=reads,
    )


def halted_awaiting_human(stream: pathlib.Path) -> tuple[bool, str]:
    """Whether a headless turn ended by asking the operator for something.

    A headless `-p` turn has nobody to answer, so a denied tool call ends the arc
    — at `subtype: success`, `is_error: false`, exit 0. Every cheap signal says
    the session finished normally; only the `permission_denials` array and the
    text of `result` say it stopped mid-task.

    That is the shape the driver must halt on rather than score, and it is the
    reason `run-session.sh`'s exit code cannot be the completion signal.

    Returns `(halted, why)`.
    """
    if not stream.is_file():
        return False, "no stream"
    denials, result = [], ""
    for raw in stream.read_text(errors="replace").splitlines():
        raw = raw.strip()
        if not raw.startswith("{"):
            continue
        try:
            event = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if event.get("type") == "result":
            denials = event.get("permission_denials") or []
            result = str(event.get("result") or "")
    if not denials:
        return False, ""
    tools = sorted({str(d.get("tool_name")) for d in denials})
    return True, f"{len(denials)} denial(s) on {', '.join(tools)}: {result[:160]}"


def windows(records: "list[Invocation]",
            opens: "Callable[[Invocation], bool]",
            closes: "Callable[[Invocation], bool]") -> "list[tuple[Invocation, Invocation, float, float]]":
    """Every `(open, close, seconds, longest_silence)` interval in a session.

    `cue-card-postmortem.md` §2 is a table of exactly this shape — the interval
    between a trigger and the `task finalize` that closed the doc's staged life,
    and the longest gap inside it — and it says in its own header that it was
    "reconstructed from `<corpus>/.jigc/logs/invocations.jsonl` in each archived
    corpus", by hand, after the trial.

    That table is what established the instrument was impossible rather than
    mistimed (11-19 seconds, longest silence 4-17). Computing it is four lines and
    the answer decides whether a designed occasion is worth building at all, so it
    should not wait for a post-mortem.

    `longest_silence` is the largest gap between consecutive invocations inside the
    window — the actual room an operator had, which is smaller than the window
    whenever the worker was busy.
    """
    out = []
    for i, rec in enumerate(records):
        if not opens(rec):
            continue
        for j in range(i + 1, len(records)):
            if not closes(records[j]):
                continue
            span = records[i:j + 1]
            gaps = [(b.timestamp - a.timestamp).total_seconds()
                    for a, b in zip(span, span[1:])]
            out.append((rec, records[j],
                        (records[j].timestamp - rec.timestamp).total_seconds(),
                        max(gaps) if gaps else 0.0))
            break
    return out
