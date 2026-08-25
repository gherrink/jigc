"""Answer what the answer key covers; halt for a human on anything else.

The operator's job in a blind session is small but unforgiving. From
`runbook.md`:

  * *"If the worker asks you anything, tell me before answering. Replies come
    from `answer-key.md`; improvising one is how a session gets contaminated."*
  * *"Never say anything to a worker that names reading, checking or verifying —
    or a synonym. That voids the session's read-back measurement."*
  * every utterance logged verbatim, as it happens, with its justification.

That last one was missed once in the 1.0.0-gate trial and had to be recovered
from a transcript afterwards, and the recovery **changed how a headline finding
read**: an apparent adapter bypass turned out to be operator-approved.

**What this module is, stated exactly.** It is the *mechanism* a driving loop
composes: `answer()` has two branches and no third, `screen()` refuses rather than
warns, and `OperatorLog.record()` writes as it happens. **No such loop ships in
this package** — a headless `-p` turn has no channel to inject a reply into, so
there is nothing here to drive one from, and the composition is deliberately left
until a real trial protocol shapes it.

Read that as the limit it is: nothing in this package currently guarantees that a
reply reached a worker through `answer()`. An operator replying by another route
gets no screen and no log entry. The guarantee is available to a caller that uses
these functions, and it is not a property of the apparatus as a whole.

**What a halt actually looks like, measured.** A headless turn that needs the
operator ends at exit **0**, `subtype: success`, `is_error: false`, with an empty
stderr — every cheap signal says it finished. Only `permission_denials` and the
text of `result` say otherwise. That is why `observe.halted_awaiting_human()`
exists and why the process exit code is never the completion signal.

**The contamination screen is mechanical here, not a discipline.** Every reply —
keyed or human-supplied — is checked against a forbidden vocabulary before it is
sent, and a hit refuses rather than warns. An operator who improvises a reply
naming a read surface voids the measurement, and a screen that can be forgotten
is off wherever somebody forgot it.
"""
from __future__ import annotations

import dataclasses
import datetime
import pathlib
import re
from typing import Iterable, Optional

#: Words that void a read-back measurement if they reach the worker. §3.2's
#: absolute rule: an utterance may not name reading, checking or verifying, or a
#: synonym. Deliberately wider than the trial's own list — over-refusing costs one
#: operator rewrite, under-refusing costs the session's headline number.
FORBIDDEN = (
    "read", "re-read", "reread", "check", "verify", "verified", "confirm",
    "inspect", "review", "look at", "open the", "cat ", "show the", "doc show",
    "validate", "audit", "double-check",
)


@dataclasses.dataclass(frozen=True)
class Answer:
    """One answer-key entry: what it matches, and what to say."""
    key: str
    pattern: re.Pattern
    reply: str
    justification: str = ""


@dataclasses.dataclass(frozen=True)
class Reply:
    """What the driver decided to do about one worker question."""
    matched: Optional[str]
    text: str
    halted: bool
    reason: str


def contaminates(text: str, forbidden: Iterable[str] = FORBIDDEN) -> list[str]:
    """Which forbidden words a reply would put in front of the worker."""
    low = text.lower()
    return [w for w in forbidden if w in low]


def screen(text: str, forbidden: Iterable[str] = FORBIDDEN) -> None:
    """Refuse a contaminating reply rather than warn about it."""
    hits = contaminates(text, forbidden)
    if hits:
        raise SystemExit(
            f"refusing to send a reply naming {hits}: §3.2's contamination rule is "
            "absolute, and a reply that names a read surface voids the session's "
            "read-back measurement. Reword it, or record the session as void."
        )


def answer(question: str, keys: Iterable[Answer]) -> Reply:
    """Answer from the key, or halt. There is no third branch.

    A question the key does not cover is **not** improvised and is not skipped —
    it stops the run and asks a human, which is the one interaction this whole
    directory exists to preserve rather than remove.
    """
    for entry in keys:
        if entry.pattern.search(question):
            screen(entry.reply)
            return Reply(entry.key, entry.reply, False,
                         entry.justification or f"answer-key entry {entry.key!r}")
    return Reply(None, "", True,
                 "no answer-key entry matches; improvising one is how a session "
                 "gets contaminated")


class OperatorLog:
    """Every utterance into a session, written as it happens.

    Append-only and flushed per entry, because the failure this repairs is a log
    that was accurate for everything except the one entry nobody wrote down.
    """

    def __init__(self, path: pathlib.Path, session: str) -> None:
        self.path = path
        self.session = session
        if not path.exists():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(
                f"# Operator log — {session}\n\n"
                "Every utterance into this session, verbatim, as it happened.\n"
                "Written by the driver: §8 rule 2 requires it, and in the 1.0.0-gate\n"
                "trial one entry was missed and had to be recovered from a transcript,\n"
                "which changed how a headline finding read.\n\n"
            )

    def record(self, kind: str, text: str, justification: str) -> None:
        stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        with self.path.open("a") as fh:
            fh.write(f"## {stamp} · {kind}\n\n")
            fh.write("```\n" + text.rstrip() + "\n```\n\n")
            fh.write(f"*Why:* {justification}\n\n")

    def record_halt(self, question: str, reason: str) -> None:
        self.record("HALT — waiting for a human", question, reason)


def load_answer_key(path: pathlib.Path) -> list[Answer]:
    """Parse an answer key written as markdown sections.

    Shape, deliberately close to the hand-written `answer-key.md` it replaces:

        ## <key>
        match: <regex>
        why: <justification>

        > the reply, as a blockquote

    A key whose reply would contaminate is refused **at load time**, not when it
    is first needed — an answer key is written before a trial and read during one,
    and a screen that only fires on use fires in the middle of a session.
    """
    entries: list[Answer] = []
    current: dict = {}
    reply: list[str] = []

    def flush() -> None:
        if not current:
            return
        text = "\n".join(reply).strip()
        if not text:
            raise SystemExit(f"answer-key entry {current.get('key')!r} has no reply")
        screen(text)
        entries.append(Answer(
            key=current["key"],
            pattern=re.compile(current.get("match", re.escape(current["key"])), re.I),
            reply=text,
            justification=current.get("why", ""),
        ))

    for line in path.read_text().splitlines():
        if line.startswith("## "):
            flush()
            current, reply = {"key": line[3:].strip()}, []
        elif line.lower().startswith("match:"):
            current["match"] = line.split(":", 1)[1].strip()
        elif line.lower().startswith("why:"):
            current["why"] = line.split(":", 1)[1].strip()
        elif line.startswith("> "):
            reply.append(line[2:])
    flush()
    if not entries:
        raise SystemExit(f"{path} defines no answer-key entries")
    return entries
