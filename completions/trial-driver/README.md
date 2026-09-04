# `trial-driver/` — driving and scoring an RC-trial session without an operator

This exists because the 1.0.0-gate trial's instrument failed in a way nobody could
have hit. [cue-card-postmortem.md](../artifacts/RC-1.0-gate/cue-card-postmortem.md)
measured the operator's injection window at **11–19 seconds** across four sessions,
the cue card fired **0 times of 4**, and M48's `jigc doc rename` was reached by
nothing. The rule promoted out of that trial is the one this directory is built to
obey:

> An instrument fires reliably iff its trigger is a **state**, its consequence is
> **re-raised by the product**, and **every worker behaviour maps to a scored
> outcome**.

The driver does not solve the injection problem — see *Bounds*. It removes the
operator from prompt delivery, evidence collection and counting, and it makes
**rehearsal** cheap, which is the act rule 4 says must be paid for before a trial
and which cost a whole session before.

## What is here

| file | what it is |
|---|---|
| `run.py` | the one entry point: `observe`, `test` |
| `driver/channels.py` | protocol §3.3's four channels, as predicates. The *registration* |
| `driver/observe.py` | reads a finished session's channels off its own evidence |
| `driver/cascade.py` | ordered outcomes, first match wins, apparatus failures **voided** |
| `arms/adopt.sh` | adopts a naive corpus using the **container's** jigc, via `--exec` |
| `driver/session.py` | seed a conversation once, freeze it, fork it N times |
| `driver/plants.py` | wait for a plant's state and fire it, from outside the session |
| `driver/interact.py` | answer from the key or halt; the operator log, written as it happens |
| `driver/gate.py` | refuse a round the isolation record does not cover |
| `walk.py` · `arms/walk/` | the operator walk as scripted arms, unrun ones visibly blank |
| `test_*.py` | six suites — `python3 run.py test` |

Driving happens through `../trial-harness/run-session.sh --headless`, not through a
second container path. That is the flag's own stated rule for `--exec`: *"a control
driven by some other mechanism would not validate the mechanism the blind sessions
actually run on."*

## The reader's positive control

```sh
python3 completions/trial-driver/run.py observe --archive
```

Scores the committed 1.0.0-gate evidence and compares against the record's own
hand-derived table. **If it cannot reproduce numbers this repo already settled by
hand, the reader is wrong** and nothing it says about a fresh session should be
believed. It currently reproduces all four blind sessions exactly, including B1's
corrected `3` rather than the running note's `5`.

## What it found, that a human had not

- **`run-session.sh`'s adjacent counter was labelled `(VERB-ADJACENT, §3.3)` and was neither.**
  **Settled and fixed 2026-08-28** by the protocol that owns the question: the counter is aligned to
  §3.3 (`task validate` joins; a `doc list` without `--task` leaves) and demoted to an explicitly
  indicative quick look pointing at `run.py observe`, so one registered measurement has one
  authoritative implementation. The aligned grep reproduces this reader's §3.3 numbers exactly on all
  four archived sessions (2 · 9 · 5 · 6, against the old counter's 0 · 4 · 0 · 1). What it was, kept
  because the wart is why the fix exists — and note the line has since moved off `:174`, which is now
  the `--shell` branch: it
  misses `jigc task validate` (§3.3 lists it) and counts a bare `jigc doc list`
  (§3.3 requires `--task`). Net undercount of 2–5 in all four blind sessions. The
  record was **not** misled — it broke `task validate` into its own column and said
  so beneath the table. This is a wart the humans compensated for by hand, and the
  compensation is what the driver stops needing. `Observation.adjacent_counter_gap`
  still ships and still reports the delta — it is now the *regression* check on an
  answered question rather than an open one, and it goes to zero against the aligned
  counter.
- **`grep -c` prints `0` twice.** It emits `0` *and* exits 1, so the `|| echo 0`
  fallback fires too. **Fixed 2026-08-28** in the same patch: the counter's `count()`
  helper assigns on failure instead of falling through to a second `echo`.

### Found on the RC-m50 trial (2026-09-04) — three more, two of them on the duress cell

- **A Bash read stored the matched *hint* as its path**, so `is_document` tested `.jigc/tasks/` for a
  `.md` suffix and every `cat` of a staged document under the workbench was filed as bookkeeping —
  B3-h2's one read of the planted doc rendered `wkbn`. Fixed: the path is the path read
  (`test_observe.py::ABashReadOfAStagedDocumentIsADocumentRead`).
- **`find <task dir> -type f | xargs … cat {}` scored FILESYSTEM 0** — `find` is not a reader, the `cat`
  stage carries no path, and the `;` inside `sh -c` split the statement before the `cat` was seen.
  B2's duress read, found from the worker's own debrief, not by the reader. Fixed: a `find`/`ls` head
  piped into `xargs` is one read of everything under the head; `find | grep -v` stays the dead false
  positive it was (`test_observe.py::AFindPipedIntoCatIsARead`).
- **`walk.py` lost every arm's stdout across `--only` passes** (exit codes kept, bar lines gone) —
  fixed by persisting `ARM-OUTPUT.txt` beside the evidence; and then **`carry` handed that file on as
  corpus**, the fix's own sibling one layer out, fixed in `_EVIDENCE_NAMES` (`test_walk.py`,
  `test_session.py::CarryLeavesTheWalksOwnOutputBehind`).

The pattern holds: none was found by reading. Two of the three sat on the cell the headline rests
on, and the second was invisible to the reader until a worker said what it had done.

## Bounds — read these before believing a number

**Headless is a different channel from the one every archived trial measured.**
`--fork-session` drops session-scoped permission grants, and `--plugin-dir`,
`--settings`, `--mcp-config` and `--add-dir` are not restored on resume. A headless
result is a fact about the headless channel until something shows the two agree.

**`--strict-permissions` headless reproduces the adopter's condition faithfully —
measured, after the opposite was predicted.** This directory was planned on
`knowledge/17`'s claim that *project allow rules are ignored under `-p`*, which
would have made the strict arm misrepresent the asymmetry rather than merely
flatten it. **Driven on `jigc-gate:rc11`, that is not what happens:** all 13 jigc
invocations executed under `--permission-mode default`, and all **7** permission
denials were file writes (`Write`, and Bash `>` redirects). `Bash(jigc:*)` was
honoured. The prediction was wrong and the arm is usable — recorded here because a
bound that quietly disappears is how a false premise survives.

**Mid-turn injection is not solved, and headless makes it worse.** `-p` runs the
arc to completion with no queued-message channel at all: the 11–19-second window
does not widen, it disappears. The postmortem's answer — plants, whose trigger is a
state and whose consequence the product re-raises — is unchanged. Automation buys
the *driving*, never the *interrupting*.

**A halt looks exactly like success.** A headless turn that ends asking the operator
for something exits **0**, at `subtype: success`, `is_error: false`. Only
`permission_denials` and the text of `result` say it stopped mid-task, which is why
the process exit code cannot be the completion signal. `halted_awaiting_human()` is
the check; observed live on the first strict arm.

**The FILESYSTEM channel is a heuristic; the other two are exact.** VERB and
VERB-ADJACENT come from the invocation log, a product surface with a pinned record
shape. FILESYSTEM comes from the CLI transcript, whose format `knowledge/10` warns
changes between versions — so hits are returned as *evidence for review*, not as a
number. Registration state is not in the transcript at all, so a read of a foreign,
never-adopted file at a managed home matches and should not score; the 1.0.0-gate
record dispositions exactly one such read that way.

**The driver does not grade findings.** It produces counts, timings and scored
session outcomes. Confirming or refuting a claim, writing repro blocks, and citing
`pinned-by:` stay hand work — [pinning.md](../../implementation/pinning.md) §3
refuses that automation by name, on the grounds that *a symbol-existence parser
would be a finder wearing a fence's badge*.

**It was reviewed twice, and the review found more than the driving did.** A
self-review found one defect; an independent cross-model review (Codex) found ten,
five of them able to turn apparatus failure, missing evidence, a failed read or the
wrong fixture tree into a clean product verdict. Every one was checked against the
code before being accepted — one was already fixed in the worktree, and the reviewer
said so itself. The headline five:

- **a dead CLI scored as a product result** — `fork()` printed the arm's exit code
  and never returned it, so cascade row 1 was unreachable through the real path;
- **a missing transcript became `NEITHER`** — a claim that the worker read nothing
  anywhere, made with the FILESYSTEM channel unread;
- **failed reads counted as read-backs** — `rosewater`'s registered 7 contains two
  `doc show --task` calls that exited 1. Both counts now ship, because which one is
  meant is the protocol's call, not the reader's;
- **leading global flags hid every structured channel** — `jigc --format json doc
  show … --task …` is accepted, logged verbatim, and matched nothing;
- **every headless run orphaned a container holding the OAuth token** — `--headless`
  fell through the `else` branch first. Eleven had accumulated.

**Every module here was corrected by driving it.** Not one of the defects below was
found by reading: the shadowed cascade row (by `check_examples()`), the missing
`unmeasured` row and `doc author --help` (by the first live arm), the unreadable
staged transcript, the blind seed-failure guard and the rig's evidence leaking into
the corpus (by the first live chain), the plant's sibling body file (by the first
poller run), `python3` being absent from a node image and a step whose stderr was
never captured (by the first walk arm). The corpus gate's own self-test found, on
its first run, that it had been matching pass labels against failure text and was
reporting every bar dead.

**A green suite is necessary and not sufficient.** Both defects the cascade has
caught so far were caught by *running* it somewhere it had not been run — the
shadowed row by `check_examples()`, and the missing `unmeasured` row by the first
live arm. Neither was found by reading.
