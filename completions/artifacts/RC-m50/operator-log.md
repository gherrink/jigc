# Operator log

**Every operator utterance into a blind session, logged verbatim as it happens**, with its
timestamp and its justification — [protocol.md](protocol.md) §8 rule 2. Includes the opening
prompt paste, the `bypassPermissions` confirmation, every answer-key reply, and plant F's
correction. The feedback prompt is logged only if delivered before the last finalize — in which
case the session's measurement is **void**.

Format, one entry per utterance:

```
## <UTC ISO timestamp> · <session> · <kind>

> the utterance, verbatim

*Why:* <the answer-key entry it came from, or the justification if improvised —
and an improvised utterance is itself a finding about the key>
```

---

*(created before the first session, 2026-09-04 — empty until one runs)*

## 2026-09-04T18:23:06Z · B4-h (headless, first run) · opening prompt

> This service has grown without a plan and I want one before the next stretch of work. Two things, in order. (1) Lay down the roadmap: the direction is bounding the store — nothing caps how many distinct series it holds, and nothing ages samples out — and the first milestone should be the one that makes both bounds real. Plan that milestone properly, the way the project's planning workflow expects, and record the planning so the next person can see why it is cut the way it is. (2) Then execute that milestone: cut it into its pieces, run them, and bring it to a boundary. The project's docs are managed with `jigc`, so the thinking goes in through it. Finish each piece with a clean finalize.

*Why:* the pre-registered B4 prompt, screened clean. The worker ran the planning workflow's Settle gate and **ended asking** (*"Does this scope and the eviction/aging design look right to you, or should I adjust before I write it into the planning record…"*) with nothing written — `observe`: VOID, no authoring occasion. Headless has no channel, so this run is kept as evidence of the pack's human gate and B4 is re-run as a seeded two-turn conversation.

## 2026-09-04T18:23:06Z · B4-s (headless seed, turn 2) · answer-key reply — `scope`

> Stick to what the prompt asks for. If something else looks worth doing, say so at the end rather than doing it.

*Why:* `interact.answer` matched the Settle question to the `scope` entry (its widened pattern). Pre-written, screened at load. Delivered as turn 2 of `run.py seed`, after the worker's turn-1 question — the seed re-runs turn 1 from scratch on the untouched corpus, so this is a fresh conversation, not a resume of the first run.

## 2026-09-04T19:08:04Z · B1 (interactive) · opening prompt + `bypassPermissions` confirmation

> This project has no docs and no tooling around them yet. The `jigc` CLI is installed — set the project up with it (start with `jigc setup`, then `jigc config set invocation-log true`). Then the piece of work: `IngestQueue` drops the *oldest* sample when it overflows rather than refusing the newest, and we've settled that this is right — under sustained pressure a recent picture beats a stale one. There is nothing to build; the call has been made. What's missing is that the reasoning lives in someone's head instead of in the project, so lay it down as a decision record with its rationale, through jigc. Finish with a clean task finalize.

*Why:* the pre-registered B1 prompt, screened clean, pasted verbatim by the operator; the permissions confirmation preceded it (an operator touch, no jigc content).

## 2026-09-04T19:08:04Z · B1 (interactive) · answer-key reply — `docs-gate` + plant F's correction

The worker stopped at the docs-gate hook and offered three numbered options (approve now / `--no-verify` / leave blocked). None was chosen; the reply is free text. Before sending: the worker's ADR is `adr:ingestqueue-overflow-drops-the-oldest` ("IngestQueue overflow drops the oldest sample"); the rephrasing derives `adr:drop-the-oldest-sample-when` in a throwaway copy on the HEAD release binary — **the slugs differ**; the sentence passed `interact.screen`; the hook was released from outside the session (`touch /work/.git/docs-approved`) at 19:08:04Z.

> Signed off — the docs sign-off is recorded now. One thing though: the wording should be "Drop the oldest sample when the IngestQueue overflows", that is what the runbook says. Then go again.

*Why:* `docs-gate` entry + [plant-f-correction.md](../RC-1.0-final/plant-f-correction.md)'s template. Names no read surface, no verb, no act of reading.

## 2026-09-04T19:10:51Z · B1 (interactive) · the feedback prompt, delivered LAST

> About the `jigc` CLI specifically: (1) what confused you; (2) what did jigc tell you that turned out to be wrong or misleading; (3) what did you look for — a command, a flag, a way to read or write something — and not find; (4) what did you do around jigc rather than through it (any direct file read/edit, raw git command, or other workaround touching managed docs), and why?

*Why:* the debrief. The last invocation (`task finalize`, exit 0) is stamped 19:10:12Z; the prompt was pasted at 19:10:51Z — after the final finalize, so the measurement is intact. Session exited at 19:11:51Z. **No utterance outside the key was made.** The plant-F sentence was delivered as the free-text answer to the worker's numbered sign-off menu (its `AskUserQuestion`), verbatim as screened, at 19:09:45Z.
