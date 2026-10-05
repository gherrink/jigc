# The gate loop — test, fix and re-test a release candidate until the exit rule holds

**Status: parked 2026-10-05.** Written at the close of the session that ran the gate before 1.0.0 on
`1.0.0-rc.24` by hand ([DECISIONS.md](../DECISIONS.md) → *2026-10-05 — The session closes without a
testing round and without the call*). Indexed from [VISION.md](../VISION.md) → Open questions; the
trigger row is [decisions-pending.md](../implementation/decisions-pending.md) → *Before the next
release candidate — the fix pass's re-audit*. *Trigger:* the next session — before the next gate round
on the rc.24 fix pass — turns this into a real workflow out of the mechanisms and rules that already
exist.

> **Notation is illustrative.** This is a parked idea, not a workflow: the workflow docs it names own
> their rules, and nothing here restates them.

## Why this exists

The gate on rc.24 was orchestrated ad hoc: nine separate workflow runs and a dozen single agents over
about two days, a fix pass that grew from six rows to more than fifty commits, a completion audit that
came back red on the pass's own new code, and a loop that had no written stop until the human wrote
one. Every phase reused something that exists — the per-axis instrument, the trial harness, the
completion workflow's audit and fix lane — but the *loop around them* lived in one orchestrator's
head. The record of that run is the fix pass's directory,
[completions/artifacts/M55/fix-pass-rc25/](../completions/artifacts/M55/fix-pass-rc25/README.md) —
its [README](../completions/artifacts/M55/fix-pass-rc25/README.md), its
[findings ledger](../completions/artifacts/M55/fix-pass-rc25/findings-ledger.md) and
[perf/run-performance.md](../completions/artifacts/M55/fix-pass-rc25/perf/run-performance.md); this
file is what that run says the loop should be.

## The loop in one picture

```text
candidate ──▶ 1 TEST ──▶ 2 VERIFY-REAL ──▶ 3 TRIAGE against the exit rule
                                              │
                    nothing blocks ◀──────────┤──────────▶ something blocks
                          │                                      │
                    7 RECORD                              4 FIX round
                          │                                      │
                 the human takes                          5 AUDIT the fix diff
                 (or declines) the call                          │
                                              audit green ◀──────┴──────▶ audit red
                                                   │                        │
                                             7 RECORD ──▶ 8 HAND OFF     back to 3
                                             (PR · release candidate)   (bounded: 6)
                                                   │
                                        next candidate ──▶ 1 TEST, scoped to what was touched
```

## What the loop tests against

The exit rule the human set on 2026-10-04 (its home is [DECISIONS.md](../DECISIONS.md) → *2026-10-04 —
The exit rule, revised*; the owed-and-when half is
[decisions-pending.md](../implementation/decisions-pending.md) → *The exit rule*): four clauses, each
with a scope and one instrument. A finding blocks only if it breaks a clause inside its scope; everything else is recorded.

| Clause | Instrument the loop runs |
|---|---|
| no lost files, no incorrect writes — in a healthy repository used as documented | the per-axis review rows and the audit, graded against that scope |
| a working product others can rely on | a regression set: what worked on the previous candidate still works; every refusal's route works as printed |
| usable by agents | the blind trial arms |
| this repository's migration onto jigc will work | a port rehearsal on a copy of this repository — **does not exist yet** |

## The phases

Each phase: what it does · the existing mechanism it should be built from · what it hands on · where it halts.

**0 · Preflight.** Assert before anything runs: the container runtime is up; the trial harness's and
driver's own checks pass; the Codex quota and the session token are available (by name, never by
value); the candidate's identity (published version or branch tip); the default tags and paths the
tooling still carries are overridden; a durable record directory exists and the branch is pushed.
*Why:* this run lost time to a stopped runtime, an exhausted Codex quota, a broken `seed`, and
defaults three candidates stale — each found mid-phase.

**1 · Test — the instruments, in parallel, scoped.** The first candidate gets the full set; every
later candidate gets only the rows and arms its fixes touched, plus the regression set.
- *Per-axis review:* the instrument at `completions/artifacts/M55/per-axis-review-rc24/instrument/`
  (driver, unseeded Codex pass, reconciler, assembler) — rows chosen from the ledger.
- *Blind trial arms:* `completions/trial-harness/`, `completions/trial-driver/`,
  `completions/trial-corpus-template/`; outcome classes fixed before the run; an occasion that is
  unambiguous (this run's inconsistency arm was not) and **rehearsed** first.
- *End-to-end:* the `milestone-e2e-tester` over the standing fixture states and a fresh clone.
- *Regression set:* the previous candidate's happy paths and every route a refusal prints.
- *Port rehearsal:* to be designed (below).

**2 · Verify-real.** Every candidate blocker gets one independent adversarial re-drive on a fresh
rig: both halves shown, declared-or-not, reach, the class, the code site. A report is a lead until
this has run. ([milestone-completion-workflow.md](../implementation/milestone-completion-workflow.md)
→ The loop → 2. Plan (triage).)

**3 · Triage against the exit rule.** Each verified finding is one of: *blocks* · *declared bound*
(written down with its reach) · *recorded*. A fix that is contested, or needs a new mechanism, is a
halt to the human — presented one at a time, as short plain text with lettered options and a
recommendation, the robust case argued by the `robust-advocate`, and the proposal driven through the
other party's next step before the human sees it.

**4 · Fix round.** `build-fixer` per finding or class, briefed with the finding and never its
boundary; partitioned by projected write-set — worktree fan-out where disjoint, serial in the main
tree where not, said up front. The brief that this run converged on:
- reproduce red first; the smallest correct change that closes the class;
- a new stored file, registry, flag or finding code is a halt, not a build decision;
- where git already answers a question, ask git — never re-derive it from bytes;
- every guard gets *must-not-refuse* cells beside its *must-refuse* cells, over git configuration ×
  repository layout;
- every route a refusal prints is run as printed on a built binary;
- docs in the same commit as the rule; the full gate before each commit.

**5 · Audit the fix diff.** The completion audit, on the diff only: `milestone-code-reviewer` per
disjoint area plus one cross-cutting reviewer, and the `milestone-e2e-tester` — read-only, parallel,
on one shared prebuilt binary. Each finding states its class and carries a repro.

**6 · Loop control.** Audit findings go back to phase 3, not straight to a fixer. A regression the
pass introduced is always fixed. A pre-existing finding outside the classes in hand is triaged
against the exit rule like any other — it is not fixed merely because it was found. After a set
number of fix rounds without a green audit, the loop halts to the human with the ledger.

**7 · Record — every round, not only at the end.** A record commit opens and closes each fix round,
so the decision log never lags the code (it did, twice). Every finding of every phase gets one ledger
row — source, door, where its repro lives, disposition — including what fixers, planners and
advocates *left open*. Nothing lives only in a scratch directory; the branch is pushed at each phase
boundary.

**8 · Hand off.** The pull request, the human's merge, the release PR, the published candidate — all
as [milestone-completion-workflow.md](../implementation/milestone-completion-workflow.md) → Close and
[release.md](../implementation/release.md) already say. Then phase 1 again, scoped.

## What made the rc.24 loop not converge — and the rule each lesson became

| What happened | The rule |
|---|---|
| "No tier-1 row" had no reach term; each door studied hard yielded another exit-0 loss | the exit rule's scope and declared bounds (phase 3) |
| Wide fixes brought new code; the audit found four regressions in exactly that code | smallest correct change · ask git · must-not-refuse axis · new mechanism halts (phase 4) |
| ~130 items fixers "left open", thirteen of them marked the human's call, were triaged by nobody | left-open lists are ledger rows and go through phase 3 after each round |
| The decision log said "not built" over six built fixes | a record commit opens and closes every round (phase 7) |
| The pass's own findings lived in a temp directory | durable-first, pushed early (phases 0 and 7) |
| A two-turn tool had been broken for weeks; the Codex quota ran out mid-run | preflight (phase 0) |
| The trial's inconsistency arm had a weak occasion; its rehearsal was skipped | rehearse every arm's occasion (phase 1) |
| A fork's proposed route was half-broken and nobody had run it | drive the proposal through the next step before the human sees it (phase 3) |
| About 35 full gates in one round; six auditors each built a binary | see [perf/run-performance.md](../completions/artifacts/M55/fix-pass-rc25/perf/run-performance.md) — recommendations only; the levers there need the human's ruling where they touch a standing rule |

## What exists to build it from

| Need | Mechanism |
|---|---|
| Halts returned to the orchestrator, resume by run id, git steps on a smaller model | `.claude/workflows/milestone-build.js` |
| Fix lane, audit, verify-real, the fan-out fence | `implementation/milestone-completion-workflow.md` |
| Test-first loop and the gate | `implementation/dev-workflow.md` · `dev/gate` |
| Agents | `build-fixer` · `build-executor` · `build-git` · `milestone-code-reviewer` · `milestone-e2e-tester` · `robust-advocate` · `capability-auditor` · `increment-validator` |
| Rigs and CI-faithful runs | `dev/jigc-rig` · `dev/runner-faithful` |
| Review and trial instruments | `completions/artifacts/M55/per-axis-review-rc24/instrument/` · `completions/trial-harness/` · `completions/trial-driver/` · `completions/trial-corpus-template/` |
| The fix round as a real fan-out | `crates/cli/packs/methodology/steps/fix-gate.yaml` — once this repository is on jigc |

## What it must not become

- A new orchestration engine. It is a script over the existing agents and the existing docs' rules.
- A way around a standing rule. Where a performance lever touches one (the full gate before every
  commit, one finding per commit), that is the human's ruling, made once and recorded.
- A requirements source for the product. The port rehearsal tests the documented adoption path; it
  does not bend the CLI to this repository.

## Open questions for the session that builds it

1. One workflow with halts, or a chain of small ones the orchestrator stitches? (This run used nine;
   the orchestrator's context filled from reading their journals — phases should return a compact
   summary and write the full report to a file.)
2. The round bound in phase 6, and who may raise it.
3. The port rehearsal: which steps, on what copy, and what "works" means for a corpus this size.
4. How a fix commit is mapped to the instrument rows it obliges phase 1 to re-run.
5. How auditors stay independent while sharing one prebuilt binary.
6. The ledger's form until the port (a markdown table today) and after it (`jigc-feedback` and
   `inconsistency` docs filed through the channel).
7. Which of the performance report's levers the human admits — it carries recommendations only, and
   four of its twelve (2, 3, 10 and 12) change a standing rule.

## Where the rc.24 run stands, for whoever picks this up

The fix pass's [README](../completions/artifacts/M55/fix-pass-rc25/README.md) and
[ledger](../completions/artifacts/M55/fix-pass-rc25/findings-ledger.md) say what is on the branch, what
the audit found, what is fixed, and what is owed next — in order: the re-audit of the pass's last
round and its last fix, the pull request, the next candidate, then phase 1 scoped to what the pass
touched. No testing round ran on that last round, so the loop resumes at phase 5, not at phase 7.
