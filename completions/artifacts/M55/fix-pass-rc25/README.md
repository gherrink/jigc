# The rc.24 fix pass — the record

**Written 2026-10-05, at the close of the session that ran the pass, on `fix/rc24-tier1` at `0f34d8f0` — 55 commits over `origin/main` (`13dc5bc7`).** Drafted the day before at `5f5b273a`, while fix round 4 was running, and brought to the branch tip here. This directory holds what the pass produced that no fix commit carries: the index over every finding of the gate ([findings-ledger.md](findings-ledger.md)), sanitized copies of the files it indexes, and the performance analysis of the run. Nothing here grades a finding or proposes a fix. **It is written for a reader with no memory of the session**: start with *The state at the close* and *What is owed next, in order*.

## The state at the close

- **The pass is not finished and has not landed.** `fix/rc24-tier1` is not merged; there is no pull request; `1.0.0-rc.24` is still the published release.
- **No testing round was run on round 4 or on the last fix.** The completion audit ran once, at `5f5b273a`, and came back red. Everything after it — the 35 commits of round 4 and `0f34d8f0`, 36 commits — was read and driven by its own fixer only. Each fixer reported a green full `dev/gate` before each commit; at `0f34d8f0` the fixer's report reads `tests   passed=4635 failed=0  (over 17 test binaries)`, `GATE: PASS`. No auditor, no re-review row, no trial arm and no end-to-end pass has seen those commits.
- **The 1.0.0 call cannot be taken on this state.** It is the human's, and the exit rule (below) asks four instruments of a published candidate; none has run on anything this pass built.
- **What the ledger says, in numbers.** Of the audit's 54 findings, 45 read as closed by a fix commit, 3 are records findings the closing record commit discharges, 6 are open. Beside them the ledger's closing section lists 20 decisions taken or deferred inside the pass that were never put to the human, 3 findings surfaced after the audit and not fixed (one of them a declared behaviour), 69 leads, bounds, residue and wording items, and the review's and the trial's 58 + 21 tier-2 and tier-3 rows.
- **The test step of the gate is at its ceiling.** 590 s at `0f34d8f0` against the 600 s foreground limit of the tool that runs it, as the fixers measured; run the gate in the background and read its own exit code. → [perf/run-performance.md](perf/run-performance.md) — **recommendations only, none taken**.
- **Check that the branch is pushed.** When this record was committed, the local remote-tracking ref `origin/fix/rc24-tier1` stood at `ed783637`, behind the last fix and behind this record.

## What the fix pass is

`jigc 1.0.0-rc.24` did not pass the exit rule as it then stood: the gate before 1.0.0 — the partial per-axis re-review and the blind agent trial, both on the published binary — left **six tier-1 rows, each upheld by an independent adversarial re-drive**: `(R3, F7)` at `task finalize`, `(R6, D-1)` at `milestone finalize`, `(R1, F1)` at `setup`, `(R9, F5)` at `uninstall`, `(R6, D-7)` at `task finalize`, and the trial's `L-22` at the milestone doors and `uninstall`. None is inside M54's code, M55's code or the co-author trailer. By the human's ruling all six are fixed in **one post-review fix pass under M55**, on branch `fix/rc24-tier1` — one fixer per row deriving the class, an independent review over the diff, the record, a new stamp, the affected rows and arms re-driven; no new milestone. `(R6, K-1)`, demoted to tier 2, has its one-probe fix folded in. The rule for the pass's fix choices is the human's: **every change is future-proofed** — when the correct fix is affordable now it is taken even if it is more work. The external-writer race class behind `(R6, K-1)` is tier 2 for 1.x and is not fixed here. *(`DECISIONS.md` → 2026-10-04 — The rc.24 gate; the forks ruled inside the pass, the audit's result, the exit rule's revision and the as-built entries are the entries above it.)*

**How it went.** Rounds 1 and 2 fixed the six rows and the folded-in seventh (ten commits). Round 3 built five further class members the orchestrator queued under the standing rule and one sibling the human had ruled (six commits). The completion audit then came back red in every area; the human chose one bounded round on its findings and, the same day, **revised the exit rule** — *no tier-1 row* carried no reach term and the loop could not converge. Round 4 took 41 of the audit's findings in six areas (35 commits) and returned three as *could-not-fix*: they needed a record the task's working area did not hold. The human ruled on that record on 2026-10-05 (option A, the copied-in witness), and the last fix built it (`0f34d8f0`). Then the session ended.

## The commits, in order

`git log --oneline origin/main..HEAD`, oldest first — **55 commits**: 3 record commits, 10 fixes of rounds 1 and 2, 6 of round 3, 35 of round 4, and the last fix. `#` is the commit's position in that log. What each closed is from its commit message and `DECISIONS.md`; what each *left* is tables D and F of the ledger. The closing record commit that carries this file is the 56th and is not in the tables.

**Record commits before the close (3)**

| # | commit | what it closed |
|---|---|---|
| 1 | `bca470c1` docs(decisions): the rc.24 gate — the result, the fix pass and its first rulings | the record: the gate's result, the fix pass, its first rulings (forks `(R3, F7)` and `(R9, F5)`); the M57 row for the external-writer race class |
| 4 | `fcd9af18` docs(decisions): the rc.24 fix pass — three more rulings, and worktree doc work parked for 1.x | the record: three more rulings (the own-registration fork, the symlink fork, the linked-worktree fork); arm B parked at `ideas/linked-worktree-doc-work.md` |
| 13 | `614d9c99` docs(decisions): the rc.24 fix pass — the fixes as built, two rulings, and what they left for 1.x | the record: the ten fixes as built, two rulings, five further class members queued, eleven M57 rows |

**Rounds 1 and 2 — the six tier-1 rows and `(R6, K-1)` (10)**

| # | commit | what it closed |
|---|---|---|
| 2 | `12398ddc` fix(uninstall): the teardown answers for every byte inside a transient directory | **`(R9, F5)`**; also tier-3 **`(R9, F3)`** |
| 3 | `dc0d7586` fix(setup): a file the install would replace refuses on an unborn HEAD too | **`(R1, F1)`** |
| 5 | `c0c4d88c` fix(create): a create under a new: true entry never reaches the copy-in | **`(R6, K-1)`** (tier 2, folded in) |
| 6 | `3f3a724b` fix(milestone): a created doc never promotes over a file at its home at the milestone boundary | **`(R6, D-1)`** |
| 7 | `9465f9b6` fix(finalize): a managed doc lands as a regular file at its home, never through a link | **`(R6, D-7)`**; `jigc milestone create` folded in |
| 8 | `eb18e5fe` fix(reconcile): a task records what it copies in, and a staged doc with no record is decided by its base pin | **`(R3, F7)`** — the copy-in baseline, the base-pin backstop |
| 9 | `8c159622` fix(milestone): a door removes a git worktree registration only at a path it created, by path | **`L-22`** — no repository-wide `git worktree prune` remains |
| 10 | `ebfc79fb` fix(milestone): a worktree door answers for what git's registration of the path holds | the own-registration fork: `provision`, `discard`, `uninstall` refuse over a registration that holds work |
| 11 | `a88f71cc` fix(validate): a doc a task has both staged and bound is probed at its staged bytes only | a defect outside the review's record, the prerequisite of the next commit; separate so it can be reverted alone |
| 12 | `e3a6ba58` fix(finalize): a linked worktree the user made commits code only | the linked-worktree sibling of `(R3, F7)`, arm A; one new finding code, `finalize.linked-worktree-doc` |

**Round 3 — the five queued class members and one ruled sibling (6)**

| # | commit | what it closed |
|---|---|---|
| 14 | `104a7d4b` fix(setup): a file jigc replaces is written as a regular file, never through a link | the symlink sibling of `(R1, F1)` — queued member 1 |
| 15 | `ef9456b5` fix(setup): an install path is asked what it holds where git status cannot say | the gitignored sibling of `(R1, F1)`, and the index-flag class |
| 16 | `4fa1c0f5` fix(uninstall): the teardown never starts the invocation log it blocks over | left open by `(R9, F5)` — queued member 2 |
| 17 | `8d9c3afb` fix(milestone): a record door with no baseline compares the record against HEAD | left open by `(R3, F7)`: the milestone-record door — queued member 3 |
| 18 | `e842342e` fix(milestone): the boundary refuses before it lands without work a sub-task worktree holds | the fourth worktree door of the own-registration class — queued member 4; a second new finding code, `milestone.unlanded-work` |
| 19 | `5f5b273a` fix(rename): a door that writes a committed doc in place never writes through or moves a link | left open by `(R6, D-7)`: `jigc rename`, the relocation primitive, the record's later writers — queued member 5. **The audit ran here** |

**Round 4, area A — eol comparisons (1)**

| # | commit | what it closed |
|---|---|---|
| 20 | `676eff57` fix(cli): a comparison against git is asked of git, at setup and at both doors | install F1 · reconcile F3 — two regressions, one mechanism |

**Round 4, area B — the linked-worktree guard (6)**

| # | commit | what it closed |
|---|---|---|
| 21 | `55a6e281` fix(finalize): the linked-worktree doc guard binds only beside a real main checkout | linked-worktree F1 · E2E F1 |
| 22 | `ac0f63b1` fix(finalize): the backstop never routes a task at a finalize that would commit the main checkout's own staged work | linked-worktree F4 |
| 23 | `e590806f` fix(finalize): the relocation route drops the task it relocates before it mints | linked-worktree F3 · E2E F6 |
| 24 | `d8f9907d` fix(finalize): the linked-worktree guard's routes say only what is true of the state they print in | linked-worktree F5 |
| 25 | `e0f00278` fix(finalize): from a jigc fan-out worktree the guard's routes put the reader's milestone first | XC-4 |
| 26 | `df58d1dc` fix(validate): the blast-radius walk reads a doc the task has staged at its staged bytes only | linked-worktree F2 |

**Round 4, area C — the milestone boundary (5)**

| # | commit | what it closed |
|---|---|---|
| 27 | `78e8ded1` fix(finalize): a home git holds is occupied, whether or not the file is on disk | CPL-5 |
| 28 | `d724365f` fix(finalize): a fixed-identity doc over an occupied home is routed at exits it has | CPL-3 |
| 29 | `f031d31e` fix(finalize): two promotions of one plan to one destination are refused together | CPL-1 |
| 30 | `048724d0` fix(milestone): a promote never replaces a file a sub-task worktree staged at its home | CPL-2 |
| 31 | `c97b8eb6` fix(rename): the home-shape refusal reaches a JSON driver in the findings envelope, keyed | CPL-7 |

**Round 4, area D — the baseline record (4)**

| # | commit | what it closed |
|---|---|---|
| 32 | `4ba04efc` fix(finalize): a migration source edited after the mint is never replaced or removed | reconcile F4 · E2E F3 |
| 33 | `d24a0c91` fix(reconcile): an unreadable file-state record names the cache and carries a route that runs | reconcile F5 |
| 34 | `e39d9ac9` fix(reconcile): a drifted doc at the pin under a staged copy says what lands, not that an edit was absorbed | reconcile F8 |
| 35 | `b54b58b2` docs(reconcile): the copy-in baseline's two open bounds are stated, and four sentences that claimed them away are corrected | reconcile F6 (three of four sentences) · XC-3 |

**Round 4, area E — setup and uninstall (9)**

| # | commit | what it closed |
|---|---|---|
| 36 | `f0b1120a` fix(setup): where git cannot answer the install guard's question, nothing is installed | install F2 · XC-2 |
| 37 | `33620081` fix(setup): the guide's ownership oracle answers for the whole file, not its body | install F3 |
| 38 | `528d0205` fix(uninstall): the teardown takes only the lines jigc wrote out of a pre-commit hook | install F4 |
| 39 | `55cb29fa` fix(setup): a file the install merges into is followed through a link only to a file the repository can commit | E2E F2 |
| 40 | `1b1d7656` fix(uninstall): the teardown names each work unit whose bookkeeping it takes | install F6 |
| 41 | `cba98c44` fix(setup): the unborn refusal's commit arm says it forfeits the secrets-floor .gitignore | install F8 |
| 42 | `a330b4f9` fix(finalize): a repository that ignores jigc's own paths is stated as unsupported, and its refusal routes at the cause | install F5 — closed by declaration: the layout is stated unsupported |
| 43 | `8b214ad5` docs(uninstall): the bound on "the teardown never starts the log" counts every other jigc run, the pre-commit hook's included | XC-5 |
| 44 | `dcfa40f0` fix(uninstall): the ack says it dropped git's worktree registrations, not that it pruned them | XC-7 |

**Round 4, area F — the worktree doors (10)**

| # | commit | what it closed |
|---|---|---|
| 45 | `894f2234` fix(milestone): every git status jigc runs states its own flags, never the user's status.* config | worktree F1 |
| 46 | `f3186486` fix(milestone): a stale worktree registration is read as stale on a git that prints no prunable line | worktree F2 |
| 47 | `c3f6dcaa` fix(uninstall): a worktree registration is jigc's to drop only if jigc recorded making it | worktree F3 |
| 48 | `ad1d2ef7` fix(milestone): the keep command names a branch that is free in every letter case | worktree F6 |
| 49 | `f1b84ef1` fix(milestone): the boundary's route promises a command only where its line prints one | worktree F5 |
| 50 | `7e5ba186` fix(milestone): in a moved repository the boundary finds the registrations made at its old path | worktree F4 |
| 51 | `685a4c55` fix(relocate): a parked squatter never replaces a file already parked under its name | CPL-4 |
| 52 | `526141e6` fix(migrate-corpus): a relocation destination that is not a regular file blocks, and is left as it is | CPL-6 |
| 53 | `242341bb` fix(config): a refused re-point names the foreign file it parked and did not put back | CPL-8 |
| 54 | `ed783637` docs(milestone): who ruled the boundary's pre-landing refusal is the decision log's to say | worktree F7 (the design doc and rustdoc half) |

**The last fix (1)**

| # | commit | what it closed |
|---|---|---|
| 55 | `0f34d8f0` fix(reconcile): a doc a task holds is decided by what that task itself copied in | reconcile F1, F2, F7 (the crossing cells) · E2E F5 — the copied-in witness, the human's ruling of 2026-10-05, option A |

## The audit's verdict

The completion audit of the pass at `5f5b273a` — six independent code reviewers and one end-to-end tester — returned **red in every area and FAIL end to end: 54 findings, 14 HIGH · 26 MEDIUM · 14 LOW**. What held: all six tier-1 repros and `(R6, K-1)` refuse or hold with the bytes intact, and every route those refusals print ran as printed to a landed end state. What did not: exit-0 losses still reachable beside the fixes — pre-existing, or cells inside a class a fix claimed closed; regressions against rc.24 inside the pass's own new code — the linked-worktree guard where no main checkout exists, the status-blind ask and the base-pin backstop in a line-ending-converting checkout, `milestone provision` on a git older than 2.31; and a record that stopped before round 3. The findings, each with its repro block, are `audit/audit-findings.md`; what became of each is table C of the ledger.

## The exit rule

The human's exit rule, as stated on 2026-10-04:

> "We do not lose files or writhe / update incorrect things. We have a working product others can use and relay on. It is usable by the agents. The planned migration for this project to use jigc will work."

The two sharpenings the human accepted:

1. **The scope of the first clause.** In a healthy repository used as documented — which includes ordinary git configuration (line-ending conversion, `status.showUntrackedFiles`, a symlinked `CLAUDE.md`, linked worktrees) — no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for. Where jigc cannot tell (git fails, the index is unreadable) it refuses before writing. Races against a non-jigc writer inside a millisecond window, and deliberately planted states, are declared bounds, written down with their reach.
2. **One instrument per clause.** The re-review rows and the audit, graded against that scope · no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed · the blind trial arms · a rehearsal of this repository's migration on a copy of it with the candidate binary (which must not become a reason to bend the CLI to the project).

**A finding blocks the 1.0.0 call only if it breaks a clause inside its scope; everything else is recorded with its tier.**

## What is owed next, in order

1. **A re-audit of round 4 and the last fix.** The completion audit's shape — one independent, read-only reviewer per area, one cross-cutting reviewer, one end-to-end tester — over `5f5b273a..HEAD`: 36 fix commits and the record commit. Each finding states its class and carries a repro. The auditors' own `not_examined` lists, and the fixers' `not covered` lists, are where to aim it (ledger → *Open at the close*, (f)).
2. **Triage of what it finds, against the exit rule** — verify-real first, then: *blocks* (it breaks a clause inside its scope), *declared bound* (written down with its reach), or *recorded*. The same triage is owed to the ledger's closing section, which nobody has graded: its group (a) is twenty decisions to put to the human one at a time; its group (b) holds two un-fixed findings and one declared behaviour that touches the rule's first clause.
3. **The pass's fold-back — owed, deliberately not written at this close because the pass has not landed.** In the same motion as the pull request, once the re-audit is green:
   1. **The fix-pass span in `implementation/project-history.md`, with the `crates/cli/tests/foldback_truth.rs` arms re-aimed in the same edit.** The precedent is M53, whose post-review fixes were **not** given a span of their own: they are recorded inside the `**M53 —` span of that file, each citing its verdict addendum (`[VERDICT → Addendum]` … `[VERDICT → Addendum 4]`). M55's span is the newest in the record, so the fences read it — the claim arm, and the built-and-installed version arm against `crates/cli/Cargo.toml`; the doc comments in `foldback_truth.rs` that begin *At M53 Increment 6 (T5)* and *Inverted 2026-09-22* say how the arms moved for that pass, and `DECISIONS.md` → *2026-10-03 — M55 Increment 11 / T9* is how they were aimed at `**M55 —`. The file's own header states the rule for an edit to it.
   2. **An addendum to `completions/artifacts/M55/VERDICT.md` for the post-review fix pass.** The precedent is `completions/artifacts/M53/VERDICT.md`, which carries four: *Addendum (2026-09-22) — the post-review fix, and the second stamp*, and *Addendum 2*, *3* and *4*, one per fix round and stamp. M55's verdict has none yet, and its *What is next* still ends at the trial and the re-review on rc.23.
   3. **The row annotations in the two rc.24 records.** `completions/artifacts/M55/per-axis-review-rc24/README.md` and `completions/artifacts/RC-rc24/README.md` still read every row as open. The rows this pass closed — tables A and B of the ledger say which: the five upheld tier-1 review rows, `(R6, K-1)`, `(R9, F3)`, `L-22` with `O-1` and `O-2`'s driven halves, and the re-drive leads RD-1 and RD-4 — get a dated closing bracket there, and `(R6, K-1)`'s `UNPINNED` note can name the tests that pin it.
   4. **`CLAUDE.md` → *Project state* again, once the next release candidate is published** — it names `1.0.0-rc.24` as current and points at this record for where the pass stands; mind `foldback_truth.rs`'s rule that the file states no built-and-installed version.
   5. **`dev/clean-litter` at the pass's close.** The pass ran about ninety full gates (`perf/run-performance.md` counts 85 completed full runs in the gate logs it read).
4. **The pull request `fix/rc24-tier1 → main`, the human's merge, the release PR, the next release candidate.** `origin/main` is merged into the branch first — a merge commit, never a rebase (`CLAUDE.md` → Branches). The release PR's semver check has not seen the engine `pub` moves the pass made in a published rc crate (`DECISIONS.md`, the 2026-10-05 as-built entries, list them by commit).
5. **The exit rule's instruments on that candidate, scoped to what the pass touched:** the review rows (the instrument is `completions/artifacts/M55/per-axis-review-rc24/instrument/`); the blind trial arms, with an **unambiguous, rehearsed occasion for the inconsistency arm** — rc.24's arm (c) was not reached because its occasion was weak; the regression set — no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed; and **the port rehearsal, which does not exist yet and must be designed** before it can run.
6. **The 1.0.0 call — the human's.**

Before step 1, by the human's decision at the close: **the gate loop is built as a workflow first** — the parked idea is `ideas/gate-loop-workflow.md` — and **the performance levers are decided with the human**; the report carries recommendations only and four of its twelve change a standing rule. Both, and every item above that has a trigger, are rows of `implementation/decisions-pending.md` → *Before the next release candidate — the fix pass's re-audit*.

## Files

| path | what |
|---|---|
| [findings-ledger.md](findings-ledger.md) | the index: six tables (A the review · B the trial · C the completion audit · D the fixers' `left_open` of rounds 1–3 · E planners and advocates · F round 4's and the last fix's `left_open`), the closing list *Open at the close of the session*, and the counts — 487 rows |
| `audit/audit-findings.md` | the seven auditors' structured returns at `5f5b273a`, verbatim — the 54 findings, each with severity, `where:`, `class:` and a repro block; each area's `held` and `not_examined` lists |
| `audit/review-install-teardown.md` · `audit/review-linked-worktree-guard.md` · `audit/review-reconcile-baseline.md` | the three full review reports that were written (the other three reviewers and the E2E tester returned the structured result only) |
| `fixers/fixer-reports.md` | the fixers' reports of rounds 1 and 2 — nine reports over ten commits |
| `fixers/fixer-reports-round3.md` | the six round-3 fixers' reports |
| `fixers/fixer-reports-round4.md` | the six round-4 area fixers' reports: per finding, the gate, the must-not-refuse cells, the pinned surfaces that moved, the routes driven, what was left open |
| `fixers/fixer-report-witness.md` | the last fixer's report, `0f34d8f0` |
| `planning/plan-promote.md` · `plan-install.md` · `plan-worktree.md` · `plan-linked-worktree.md` | the four family plans written before any fix |
| `planning/advocate-install-1.md` · `advocate-install-2.md` · `advocate-promote-1.md` · `advocate-promote-2.md` · `advocate-worktree-1.md` | the five robust-case advocacies, one per fork |
| [perf/run-performance.md](perf/run-performance.md) | where the wall clock and the tokens of the whole gate run went, with twelve ranked levers — **recommendations only**; the next session decides with the human |

**Every report here is a lead, not a measurement.** The fixers' and auditors' returns are copied verbatim; the ledger's `FIXED` was checked against `git show` and, where it names a symbol, the tree, and says *as reported* where it could not be.

**Not copied:** the auditors' and fixers' driver scripts, rigs, logs and binaries. The repro blocks and the reports name them under `<scratch>/…`; they stayed in the session scratchpad, which does not outlive the session. A repro that matters has to be rebuilt from its block.

**The copies are sanitized for a public repository.** Every absolute host path was replaced — a path into this repository by its repo-relative form, a session scratch path by `<scratch>/…`, a system temporary path by `<tmp>` — 61 replacements over fifteen files in the draft (43 · 15 · 3) and 5 more in `fixers/fixer-reports-round4.md`, added at the close (1 · 1 · 3; `fixers/fixer-report-witness.md` needed none); nothing else was changed. The whole directory was then scanned again with `command grep -rn` for an absolute host path, the login name, an email address outside `noreply@anthropic.com` / `example.com` / `example.invalid`, and a credential-shaped string: none found. The `NAME=` assignments left in the copies are command lines with placeholder values, not environment values.

**Committed sources the ledger cites and does not copy:** `completions/artifacts/M55/per-axis-review-rc24/` · `completions/artifacts/RC-rc24/` · `DECISIONS.md` (the 2026-10-04 and 2026-10-05 entries) · `implementation/decisions-pending.md` → *Before the next release candidate* and *M57 — the 1.x fix pass* · `ideas/linked-worktree-doc-work.md` · `ideas/gate-loop-workflow.md`.
