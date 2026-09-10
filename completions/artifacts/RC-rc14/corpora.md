# The corpora — how each was built, and its frozen state

**Order is fixed and was followed: instantiate → gate → adopt → plant.** Gate first, plant
second, always: plant E and B1's two plants deliberately break bars a naive-corpus gate is built
to enforce, so the gate must run while it still means something.

All from [trial-corpus-template](../../trial-corpus-template/) with `--clean-prose`, one product
name each, **no `rc`/`trial`/`probe`/`gate` token** in any name a blind worker can see. Adoption
runs through **the container's own binary** (`run-session.sh --exec arms/adopt.sh …
jigc-gate:rc14`), never the host's, so the state under test is produced by the exact build the
sessions run.

## The template, as this trial uses it

**Changed since RC-m50, and declared as a bound** ([protocol.md](protocol.md) §2.2): PT-D is
closed. `Router` now buffers into `IngestQueue` and `main()` ticks, so the queue's overflow
policy — **plant E's entire subject** — is on the live write path instead of being reachable
only from unit tests. The suite moves 23 → 24 and the gate gains a 12th bar that *drives* the
path rather than grepping for it.

`src/ingest.ts` is byte-untouched: its doc-comment (*"Overflow drops the \*oldest\* sample, not
the newest"*) is the committed naming authority plant E's own bar greps verbatim.

## Gate results — every corpus, before anything else touched it

`check-corpus.sh <dir> --clean-prose`, **12 passed / 0 failed** on all eight:

| corpus | product | arm | shape |
|---|---|---|---|
| `wickfield` | Wickfield | **B1** | naive + hook plant + carryover plant |
| `thornbury` | Thornbury | **B2** | adopted + plant E |
| `marlowe` | Marlowe | **B3** | adopted + plant E + foreign-ADR poller |
| `oakhurst` | Oakhurst | **B3-h2** | adopted + plant E |
| `redbourne` | Redbourne | **B4-h** | adopted, no plant |
| `elmsworth` | Elmsworth | **R3** | adopted + plant E (rehearsal) |
| `clayforth` | Clayforth | **R4** | naive + hook plant + carryover plant (rehearsal) |
| `walk-rc14` | Walkfield | the walk | naive; each arm adopts what it needs |

The walk corpus may say what it is; the blind corpora may not.

## Adoption

`arms/adopt.sh` through the container: `jigc setup` → `jigc config set invocation-log true` →
one commit. **7 template commits + setup's install commit + the adopt commit = 9**, clean tree,
adapter surfaces present (`CLAUDE.md`, `.jigc/AGENT.md`), `invocation-log = true (project)`.
Driven on all five adopted corpora; each carried out with `run.py carry` and re-checked:
**9 commits, 0 dirty**.

The log is pre-enabled by the arm rather than by the prompt, so §3.3's primary channel does not
depend on the worker running a command, and the blind prompt loses a line that would be operator
instruction rather than task. **B1's prompt names it**, because B1 installs jigc itself.

## Plant E — by reference, never copied

Source: [RC-1.0-final/plants/e-abandoned-task.sh](../RC-1.0-final/plants/e-abandoned-task.sh).
**md5 `e8bcbee6ad930eeb65a2f9869c738add`**, recomputed live 2026-09-09 and matching the value
[RC-m50/plants/README.md](../RC-m50/plants/README.md) recorded. There is exactly one copy in the
repo, which is the strongest form of *byte-identical*: RC-m50's own note gives the reason —
*"copying them here would make two files that can drift."*

**Watch item:** `driver/plants.py:fire()` copies the plant's **whole directory** into the
container, so anything added to `RC-1.0-final/plants/` rides along into every firing. It holds
one file today.

Fired through `run-session.sh --exec` on `thornbury`, `marlowe`, `oakhurst`, `elmsworth`:

```
thornbury   arm=0  bars OK=11 FAIL=0  PLANT-E-OK  task=record-the-ingest-queue-overflow  addr=adr:reject-the-newest-sample-when
marlowe     arm=0  bars OK=11 FAIL=0  PLANT-E-OK  …
oakhurst    arm=0  bars OK=11 FAIL=0  PLANT-E-OK  …
elmsworth   arm=0  bars OK=11 FAIL=0  PLANT-E-OK  …
```

**Bar 9 holds on rc.14** — *`validate` says NOTHING about status; the discrepancy is
read-back-only* — which is what keeps 3B a measurement of *use* rather than of *invocation*. It
is the bar most exposed to a validate-sweep change and M50 changed that sweep's callers, so it
was checked rather than assumed ([rehearsal-R3.md](rehearsal-R3.md)).

**After the carry**, each planted corpus shows: the task open, `adr:reject-the-newest-sample-when.md`
and `commit:…md` staged, **tree clean**, and the invocation log **0 bytes** — the plant's last
act, so settlement 3 has nothing to exclude.

The staged doc's front matter, as a worker meets it:

```
---
status: superseded
date: 2026-08-14
```

— a `superseded` status on a doc nothing supersedes, under a title
(*"Reject the newest sample when the ingest queue is full"*) that its own `## Decision` slot and
the committed `src/ingest.ts` both contradict. Two independent instruments; renaming the doc
preserves the planted status, so a worker can fix the title and still miss the discrepancy.

## B1's plants (also on `clayforth`, for R4)

`b1-hook.sh` then `b1-staged.sh`, on the host corpus before `run-session.sh` copies it in:

- a **rejecting `pre-commit`** under `core.hooksPath=.githooks`, refusing any commit touching
  `docs/`; release marker `.git/docs-approved` **absent**; commit back-dated;
- **two paths staged before the first mint**, of two different index shapes:
  `A scripts/retention-sweep.sh` and `M src/router.ts`.

Both verified on `wickfield` and `clayforth`: `hooksPath=.githooks`, staged set exactly those
two, working tree otherwise clean.

**Plant F rides the hook's pause** — the correction is appended to the docs-gate answer-key
reply, never scheduled. Whether that pause reliably exists on rc.14 is what R4 measures.
