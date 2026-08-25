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
| `test_observe.py` · `test_cascade.py` | the suites — `python3 run.py test` |

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

- **`run-session.sh:174` is labelled `(VERB-ADJACENT, §3.3)` and is neither.** It
  misses `jigc task validate` (§3.3 lists it) and counts a bare `jigc doc list`
  (§3.3 requires `--task`). Net undercount of 2–5 in all four blind sessions. The
  record was **not** misled — it broke `task validate` into its own column and said
  so beneath the table. This is a wart the humans compensated for by hand, and the
  compensation is what the driver stops needing. Reported by
  `Observation.adjacent_counter_gap`, never silently resolved: which of the two is
  right is a protocol question.
- **`grep -c` prints `0` twice.** It emits `0` *and* exits 1, so the `|| echo 0`
  fallback fires too. Cosmetic; noted so it is not rediscovered.

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

**A green suite is necessary and not sufficient.** Both defects the cascade has
caught so far were caught by *running* it somewhere it had not been run — the
shadowed row by `check_examples()`, and the missing `unmeasured` row by the first
live arm. Neither was found by reading.
