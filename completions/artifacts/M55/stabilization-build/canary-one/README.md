# canary-one — the first stage of the stabilization workflow run with real agents

**Run 2026-10-08, headless, in a throwaway clone; analysed and recorded 2026-10-10.** One `test` stage of the synthetic run `canary-one`: one door (`jigc doc list`), five items (a review row, the cross-cutting reviewer, a scripted trial arm, the gate and the regression set as held checks), one planted false finding. What it was built to show is [the plan](../re-review-test-half/plan.md) → *The small canary*; the harness is `.claude/workflows/stabilize.js` and its doc [stabilization-workflow.md](../../../../../implementation/stabilization-workflow.md). This file is the analysis; every number names the file it was read from, and the files are beside it.

**Nothing here is a ruling.** The one open decision — what to do about the verify fan-out — is laid out under *The options*, and is the human's.

## Summary

- **The stage reached its record and pushed it, with real agents and nobody watching.** One commit (`126a8531` in the clone), vetted and pushed by the step tool to the clone's bare remote; 100 agents launched, **100 returned on their first try**; every relayed digest arrived whole (28 of 28 over both invocations); the gate (4,966 passed, 0 failed), the regression set (22 differences, 0 off the list) and the record's gate were held and judged by the tool. Nothing was published unscanned, and no false cell was found in the record.
- **It took 8 h 28 min against 120–150 minutes expected, and 100 agents against 32.** The expectation priced one triage pass and one verifier. The stage ran four triage passes, 61 verifiers, and an advocate plus an independent drive for one contested finding (138 minutes for that one fork).
- **The main finding: verification multiplies.** Every verifier returns about six things it *left open*; each is triaged like a finding; about half are graded *unclear* and verified in turn. To verify: **10 → 13 → 38 → 117**. The harness verifies three passes per invocation, so the stage returned `triaged` with **118 rows unverified** and `next: triage`. Of 372 ledger entries, **320 are a verifier's left-open items** and **50 name the round's door**.
- **The brief for this record said the second invocation never ran. It did.** It was launched 3 minutes after the first returned, re-graded the 118 rows, launched 118 verifiers, got 19 back — **6 of them confirmed** — and was killed by the account's usage limit after 51 minutes: 99 verifiers died, three tries each (297 dead launches in 64 seconds). It returned `halted`; the session hit the same limit before writing that down. **Its 19 verdicts are on no record**: 22 untracked files in the clone, copied here.
- **The product findings are real, and none is at the round's door.** 12 confirmed in the first attempt — **2 of them regressions against `1.0.0-rc.24`**, both at `jigc setup` — and 6 more in the second. All 18 break the same measure: *every refusal's route works as printed*. They are listed in §5 and, with the 47 *needs-bound* rows, in [findings.md](findings.md).
- **The left-open channel is not noise.** Verifiers confirmed 1 of 10, 3 of 13, 8 of 37 and then 6 of 16: about one in five at every depth. What the fan-out buys is an unbounded hunt outside the round's test set; what it costs is that the stage never ends.
- **What the canary did not show:** an invocation that has nothing to do; the plant (a verifier stopped from outside); that no step asks for a permission — the session ran with permissions bypassed, and the clone, an untrusted workspace, ignored the three committed allow rules; a red gate, a failed push, a refused report, a ruling, a `fix` stage.

## 1. The timeline

Read from the per-agent transcripts' first and last timestamps, ordered by the workflow's journal; UTC. Data: [data/timeline.tsv](data/timeline.tsv), [data/agents-attempt-1.tsv](data/agents-attempt-1.tsv). Session start and end: [evidence/session.out](evidence/session.out).

| phase of the first invocation | start | end | min | share |
|---|---|---|---:|---:|
| state read, attempt marker, first preflight (scope beside it), the two builds | 06:47:35 | 06:50:37 | 3.0 | |
| the candidate's gate, held | 06:50:38 | 07:13:58 | 23.3 | |
| the regression set, held | 07:14:11 | 07:36:27 | 22.3 | |
| second preflight — the trial image | 07:36:27 | 08:01:12 | 24.7 | |
| **before the first instrument** | | | **73.5** | 14% |
| **instruments** — 7 reporters, at most 4 at once | 08:01:13 | 08:45:13 | **44.0** | 9% |
| triage, pass 1 (54 findings in) | 08:45:13 | 09:03:43 | 18.5 | |
| verify, pass 1 (10 verifiers) | 09:03:43 | 09:18:50 | 15.1 | |
| the advocate of the one contested finding | 09:18:50 | 10:42:29 | 83.7 | |
| the independent drive of its proposal | 10:42:29 | 11:37:08 | 54.6 | |
| triage, pass 2 (60 in) | 11:37:08 | 12:03:11 | 26.0 | |
| verify, pass 2 (13) | 12:03:11 | 12:37:15 | 34.1 | |
| triage, pass 3 (88 in) | 12:37:15 | 13:03:12 | 26.0 | |
| verify, pass 3 (38, at most 8 at once) | 13:03:12 | 14:04:27 | 61.3 | |
| triage, pass 4 (235 in; nothing of it verified) | 14:04:27 | 14:37:49 | 33.4 | |
| **triage, four passes** | | | **103.9** | 20% |
| **verify, three passes** | | | **110.5** | 22% |
| **one contested fork** | | | **138.3** | 27% |
| the record: its batch (13.8), its held gate (22.6), commit, push, state (1.4) | 14:37:49 | 15:15:40 | **37.8** | 7% |
| **total** | 06:47:35 | 15:15:40 | **508.0** | |

The parts the estimate priced — everything before the instruments, the instruments, one triage, one verify pass, the record — took **189 minutes** here. The other 319 are passes 2 to 4 (181) and the fork (138).

The session: started 06:47:25, first invocation returned 15:15:40, `first.json` written 15:18:44, second invocation 15:18:54 to 16:10:17, session ended by the limit 16:10:24 — 9 h 23 min.

## 2. The agents

Counted from the two journals (`started`, `result`, `failed` lines) and each agent's own transcript; tokens are the sums of the `usage` blocks, one per model turn. Data: [data/agents-by-role.json](data/agents-by-role.json), the three `agents-attempt-*.tsv`. The setup's line gives 32 agents of which 19 tool steps ([evidence/setup.json](evidence/setup.json)); the split of the other 13 is this record's reading of the plan.

| role (agent definition, model) | expected | first invocation | how they ended | output tokens | second invocation |
|---|---:|---:|---|---:|---|
| tool steps (`build-git`, sonnet) | 19 | 22 | 22 returned the digest | 29 k | 6 returned |
| preflight (`stabilize-preflight`, opus) | 2 | 2 | `ready` | 20 k | 1 |
| scope (`stabilize-scope`) | 1 | 1 | `written` | 54 k | — |
| instrument reporters (2 `milestone-code-reviewer`, 5 `milestone-e2e-tester`) | 7 | 7 | `reported` | 411 k | — |
| triage (`finding-triage`) | 1 | **4** | `graded` | 605 k | 1 (98 k) |
| verifiers (`finding-verifier`) | 1 | **61** | 60 `verified`, 1 `halted` with its report | 3,060 k | **316 launches**: 19 returned (16 `verified`, 3 `halted`); 297 died — 99 verifiers, 3 tries each |
| advocate (`robust-advocate`) | 0 | 1 | `argued` | 195 k | — |
| independent drive (`general-purpose`) | 0 | 1 | `driven` | 100 k | — |
| record executor (`build-executor`) | 1 | 1 | `applied` | 118 k | — |
| **total** | **32** | **100** | 100 of 100 on the first try | **4.59 M** | **324** (1.34 M) |

- **Tokens, all kinds.** First invocation: 4.59 M output, 16.7 M cache writes, 329.0 M cache reads. Second: 1.34 M, 4.2 M, 83.3 M. The runtime's own count, which leaves cache reads out: 14.35 M and 4.45 M ([evidence/first-harness-log.json](evidence/first-harness-log.json), [evidence/then-halted.json](evidence/then-halted.json)). The session's cost record puts both invocations at 311 USD at API prices, 308 of it Opus. **A verifier costs, on average, 10.5 agent-minutes and 50 k output tokens** (median 8.4 minutes, longest 34.1); the 61 of them are two thirds of the stage's tokens.
- **Endings.** First invocation: no agent returned nothing, none threw, none was retried. Second: every agent launched after 16:09:13 failed with the limit's message; the harness read each as a null return, tried each three times within a minute, logged `exhausted 3 attempts` 99 times, and its breaker — two exhausted agents — tripped after 193 of the 297 dead launches; 104 more followed it. Eight of the 99 were at work (2 to 51 model turns) when the limit came; none of the 99 wrote a report.
- **Concurrency.** At most eight agents at once, in both invocations.
- **Tool calls that failed.** 89 of 4,749 tool results are flagged as errors ([data/tool-errors.json](data/tool-errors.json)). 84 are an agent's own command exiting non-zero (80 × 1, 3 × 2, 1 × 98) — a refusal of the product it was driving, a `grep` that matched nothing. **Five were blocked before they ran**, in two texts:
  - 3 × *"This command passes a shell -c script that runs rm, and Claude Code could not check the script for dangerous removals. Approve only if you have read the script."* — three verifiers, each on a driving script of its own.
  - 2 × the shell guard: *"Blocked: `git worktree` writes into a pipe, so the exit status you get back is the last command's, not its own."* — the source reviewer and the driver, each piping a `git worktree add` into `sed`.
  - **No call of `dev/stabilize-step`, `dev/stabilize-record` or `dev/stabilize-probe` was denied, and no call timed out.** But the session ran in `bypassPermissions` (the main transcript's own field), so this says nothing about the allow rules (§7).
- **What an agent typed that its prompt did not spell.** The tool-step agents typed nothing else: 97 calls, every one a `dev/stabilize-step …` line or the structured return. The driving roles' commands are free by design; of note among them:
  - 96 commands name `/tmp` outside the scratch root (verifiers 64, triage 24, scope 5, testers 3); 33 files with such names were still there two days later.
  - 17 commands read the harness's own source — 14 of them triage, looking up the arithmetic its return is held to, `VERIFY_PASSES`, the slug rule and the wording for a missing repro block — and triage read the pending row `wf-r-m5` itself.
  - The second preflight waited for the trial image's build with `until [ -f … ]; do sleep 15; done`; two of those calls ran to the 600-second ceiling. The independent drive waited for its own gate the same way.
  - Nine `sudo -n true` probes and 14 `docker` calls by verifiers looking for a root caller or a Linux binary; the second preflight ran one read-only `git cat-file` against the repository the clone was made from.
  - Four commands contain a `git push` or a `git stash`, and nine a bare `git commit` or `git checkout`. **None reached the clone**: its reflog holds the opening and the stage's one commit, its remote one ref, no stash, no second branch or worktree.

## 3. The attempts

`dev/stabilize-canary check` printed `next: triage`, `attempt: 3` ([evidence/check.json](evidence/check.json)): the number the *next* attempt would get.

| | what it was | why it ended | what it cost | what it reused, what it redid |
|---|---|---|---|---|
| **attempt 1** | the first invocation, whole: preflight, scope, held checks, instruments, four triage passes, three verify passes, the record | at its end, by design: pass 4 is past the three passes the harness verifies (`VERIFY_PASSES = 3`), so its 117 rows stay unverified, and the record is written with `next: triage` | 508 min, 100 agents, 4.59 M output tokens | — |
| **attempt 2** | the second invocation: the lap that finishes the triage — *"118 row(s) are graded and verified now, attempt 2; no instrument runs"* | the account's usage limit, at 16:09:13; the harness halted at its next step (*"the report check could not be read"*, `transient: true`, the breaker named) | 51 min, 324 launches, 1.34 M output tokens; 19 verdicts, **none recorded** | **reused**: the scope, the items' results, the gate, the regression set, the previous release's binary (the hold was answered from the job that stood). **Redid**: the state reads, the attempt marker, the preflight (1.2 min), the candidate's build (30 s — the commit the round tested, with the branch by then one commit past it; the same binary hash), and a triage of the 118 rows (14.5 min) that re-graded one *breaks* as *unclear* |
| **attempt 3** | not run | | | Not established: whether it would take attempt 2's 19 reports or launch all 118 verifiers again. The halt's message says the stage *"reads the state as it stands and works the next attempt"*, and the ledger still has the 19 as unverified |

## 4. The verify fan-out

From each triage's own `counts` and each verifier's `left_open`, as the journal holds their returns. Data: [data/fan-out.json](data/fan-out.json); every entry with its pass, source and grade is [data/entries.tsv](data/entries.tsv).

| triage pass | findings in | from | merged | entries | graded *breaks* or *unclear* | verifiers run | confirmed · refuted · halted | left open by them |
|---|---:|---|---:|---:|---:|---:|---|---:|
| 1 | 54 | 47 findings of the 7 instrument reporters, 6 left open by the scope step, the 1 seeded row | 20 | 34 | **10** | 10 | 1 · 9 · 0 | 40, and 20 more from the advocate (11) and the independent drive (9) |
| 2 | 60 | pass 1's verifiers, the advocate, the drive | 4 | 56 | **13** | 13 | 3 · 10 · 0 | 88 |
| 3 | 88 | pass 2's verifiers | 15 | 73 | **38** | 38 | 8 · 29 · 1 | 238 (235 handed on) |
| 4 | 235 | pass 3's verifiers | 26 | 209 | **117** | 0 — past the third pass | — | — |
| **stage** | **437** | | **65** | **372** | 178 | **61** | **12 · 48 · 1** | 366 |
| lap 2, pass 1 | 118 | the ledger's unverified rows | 0 | 118 | **118** | 19 of 118 | 6 · 10 · 3 | 99 from the 19 |

- **The multiplication.** A verifier leaves 4.0, 6.8, 6.3 and 5.2 items open per pass — 6.0 on average. Triage merges about one in seven and grades 23%, 52% and 56% of what remains as *breaks* or *unclear*. So each pass launches about three times the verifiers of the one before: ×1.3, ×2.9, ×3.1. Had lap 2 finished, its 118 verifiers would have left about 615 items open at the observed rate — about 300 verifiers for the pass after.
- **437 in, 372 entries: the difference is the merges**, 20 + 4 + 15 + 26 = 65. The grades add up too: 147 *no-break* is 23 + 25 + 20 + 79; 47 *needs-bound* is 1 + 18 + 15 + 13; 114 *unclear* is pass 4's 113 and the one whose verifier halted; 12 *confirmed* and 48 *refuted* are the 60 verdicts.
- **Where the entries come from.** 320 of 372 are a verifier's left-open item; 11 the advocate's, 7 the drive's, 6 the scope step's. **27 come from an instrument** — 16 from the review row, 9 from the trial arm, 2 from the cross-cutting reviewer — and 1 from the ledger.
- **The door.** 48 entries name `jigc doc list` exactly and 2 more among other doors: **50 of 372**. By pass: 16, 13, 5, 16. The rest are at `jigc task finalize` (55), `jigc validate` (48), `jigc setup` (31), `jigc uninstall` (19), `jigc doc show` (17) and some twenty other doors; 74 name several doors or none, 7 the trial tooling. Of the 61 verifiers, 9 worked a finding at the round's door, and all 9 refuted it.

**What the 114 *unclear* are.** All are left-open items of a verifier. By a keyword read of their keys — a rough sorting, not a judgment — 67 say that something *was not driven* (a platform, a layout, another door, the previous release), 38 state a behaviour, 9 are about a pin, a test or a sentence of a doc. Ten of them, every eleventh:

| key | door | triage's line |
|---|---|---|
| `r1-p4-not-staged-block-other-unservable-states-not-enumerated` | `jigc doc show` | the other states that reach the same arm without a servable doc were not enumerated |
| `r1-p4-finalize-refuses-tracked-link-home-previous-release-landed` | `jigc task finalize` | over a planted tracked link the candidate exits 3 where the previous release landed; the route was not driven |
| `r1-p4-setup-separate-git-dir-commits-into-enclosing-repository` | `jigc setup` | setup exits 0 and makes its install commit in an enclosing repository; not driven on the previous release |
| `r1-p4-uninstall-refusal-leaves-teardown-half-done` | `jigc uninstall` | a refusal at step 6 leaves the teardown half done; the re-run was not pursued |
| `r1-p4-doc-list-over-orphan-absent-from-work-tree-not-driven` | `jigc doc list` | the second consumer of the orphan walk was not driven under this state |
| `r1-p4-finalize-exit-0-commits-without-doc-lowercase-address` | `jigc task finalize` | finalize exits 0 and commits without the doc the task wrote; validate then exits 1; no byte lost |
| `r1-p4-rename-moves-and-rewrites-unadopted-file` | `jigc rename` | rename moved, rewrote and committed a file jigc never adopted, at exit 0; candidate only |
| `r1-p4-reconciliation-rename-routes-unmanage-of-staged-path` | `jigc task validate`, `jigc task finalize` | the advisory routes `jigc unmanage` of a path this task stages a copy of; the route was not run |
| `r1-p4-finalize-ref-resolves-over-stranded-target-names-no-strand` | `jigc task finalize` | the refusal says the stranded target resolves nowhere and names neither the strand nor its repair |
| `r1-p4-amend-refusal-route-undriven-name-shapes` | `jigc task finalize` | not driven: the original entry and the names git quotes under every setting |

They are leads about the product at other doors — several of them serious if true — and statements of what a verifier did not reach. None restates a finding already on the ledger: triage merged those.

**What the 147 *no-break* are.** 112 are a verifier's left-open items, 12 the advocate's and the drive's, 5 the scope step's, 18 an instrument's findings. By the same rough read: 89 state a behaviour that triage found to break no clause — wording, a statement stronger than the fact, a read that reports less — 40 are about a pin, a test or a doc, 18 are things not driven. Ten, every fourteenth:

| key | door | triage's line |
|---|---|---|
| `r1-doc-list-non-utf8-path-drops-orphan-rows` | `jigc doc list` | a read that reports less than is there; nothing written; exit 0 |
| `r1-arm-evidence-git-dir-touched-after-run` | the trial arm's evidence | nothing the score reads moved; what touched the directory was not established |
| `r1-decisions-pending-d-row-true-one-doc-deep` | none — a row of the pending list | a record that owes a sentence; no clause reads it |
| `r1-uninstall-help-says-touches-nothing-outside` | `jigc uninstall` | a help-truth matter; no byte destroyed and nothing committed |
| `r1-p3-validate-says-no-address-reaches-file-one-does` | `jigc validate` | wording of an advisory whose route works under the true spelling |
| `r1-p4-doc-list-directory-named-md-ends-listing` | `jigc doc list` | a raw OS error and no route over a planted directory, both binaries |
| `r1-p4-setup-writability-route-names-directory-not-parent` | `jigc setup` | the route works; its sentence names the directory where the act is on the parent |
| `r1-p4-migration-review-hold-names-two-spellings-of-one-file` | `jigc task finalize` | wording of the review hold |
| `r1-p4-composed-step-says-nothing-to-recover-from` | `jigc migrate` | a statement stronger than the fact, toward caution |
| `r1-p4-migrate-help-says-ingest-advisory-routes-migrate` | `jigc validate` | two surfaces disagree on the door; the printed route works |

**The grades against the verdicts.** Of the 3 graded *breaks* and verified, 2 were confirmed and 1 — the plant — refuted. Of the 58 graded *unclear* and verified, 10 were confirmed, 47 refuted, 1 halted. **Ten of the twelve confirmed findings, both regressions among them, were graded *unclear*.** And the rough sorting by key does not predict a verdict: 6 of the 31 *not driven* items verified were confirmed, 4 of 18 behaviours, 2 of 12 records.

## 5. What the stage found about the product

On the candidate — the product tree of `7e1b34a1`, binary sha256 `dded1fac…beadfc` — against `1.0.0-rc.24` at `91834b5e`; one macOS arm64 host, a case-insensitive volume. Each row with the verifier's own basis is in [findings.md](findings.md); the repro is the block its report names under [run/r1/reports/test/](run/r1/reports/test/). **This record verified none of them again.**

**The two blockers** — ledger grade `regression`, route `fix`; door `jigc setup`, clause *working-product*:

| key | in one line |
|---|---|
| `r1-setup-install-hook-route-other-causes-unexamined` | a `pre-commit` hook that is a link to a script outside the repository: `setup` refuses, and its route names the script as the link — **done as printed, the script is deleted**, and the re-run still exits 1. `1.0.0-rc.24` exits 0 with no refusal |
| `r1-p3-setup-install-hook-undriven-producers-and-causes` | the same with a relative link; and a hook path that is a directory, or a hook of mode 000 or 200, prints the *make the hooks directory writable* route while it is writable |

**Ten confirmed, no regression** — ledger grade `confirmed`, route `human`; clause *working-product*, each a refusal whose printed route does not work:

| key | door | in one line |
|---|---|---|
| `r1-setup-installs-outside-work-tree` | `jigc setup` | in a worktree of a bare repository, seven files written and `CLAUDE.md` appended in the folder that holds the bare repository, exit 1, the route loops; in a submodule, exit 0 and the superproject's hook rewritten. **Contested** (below) |
| `r1-doc-show-lowercase-address-prints-path-no-file-has` | `jigc doc show` | on a case-insensitive volume `research:upper` opens `UPPER.md`, refuses under a spelling no file has, and the `jigc migrate` it routes dead-ends |
| `r1-adoption-route-for-non-doc-id-name-not-run` | `jigc validate` | one cell: an upper-case file migrated under its title is refused at `doc author`, and the next two refusals each print a rename route that exits 1 |
| `r1-p3-not-staged-route-task-less-read-refuses` | `jigc doc show` | over a readable file that is no managed doc, `store.not-staged` routes at the task-less read, which exits 1 |
| `r1-p3-setup-writability-route-printed-for-other-causes` | `jigc setup` | four causes draw *make the hooks directory writable* while it is writable; the re-run is identical |
| `r1-p3-uninstall-writability-route-not-driven` | `jigc uninstall` | the same route at `uninstall.remove-precommit`, under five causes, on 32 rigs |
| `r1-p3-migrate-case-variant-path-says-git-holds-no-copy` | `jigc migrate` | a path in the other case is refused as untracked; the printed `git add` stages nothing; the re-run is identical |
| `r1-p3-lowercase-address-other-doors-not-driven` | `jigc doc set-slot`, `set-field` | the write is accepted at exit 0, then `task finalize` exits 1 blaming a hook that complained of nothing |
| `r1-p3-doc-show-task-no-committed-copy-of-stranded-doc` | `jigc doc show` | after a `docs-root` change and a hard reset, the task read says *no committed copy* of a doc the task-less read finds; both acts of its route are refused |
| `r1-p3-amend-refusal-route-spells-c-quoted-name-as-path` | `jigc task finalize` | under git's default `core.quotePath` the amend refusal prints a C-quoted name as a path, which matches nothing |

**The contested one, and its fork.** The verifier found that `r1-setup-installs-outside-work-tree` contests a decision still open — the (D) row of the M57 list, *a worktree of a bare repository answers "isn't set up" at every door*. The advocate argued *robust-now* with a proposal of 389 added lines in three files ([evidence/proposal-setup-outside.diff](evidence/proposal-setup-outside.diff), sha256 `6d536acd…ea74d`, as its report names it), spiked and gated green on its own clone; **the independent drive returned `holds: false`** over 25 steps. Both reports are in the run's directory; the fork is the human's and is in `first.json` → `forks`.

**Four graded *breaks* by pass 4, verified by nobody** — ledger grade `breaks`, route `triage`:

| key | door | triage's line |
|---|---|---|
| `r1-p4-setup-hook-linked-to-dev-null-route-names-dev-null` | `jigc setup` | the refusal tells the reader to remove the link at `/dev/null` |
| `r1-p4-migration-of-upper-case-file-stage-failed-route-refuses` | `jigc task finalize` | the migration cannot land and `finalize.stage-failed`'s printed re-run refuses again; both binaries |
| `r1-p4-identity-change-route-doc-rename-exits-1-over-foreign-file` | `jigc doc author` | `write.identity-change`'s printed `jigc doc rename` exits 1; both binaries. Attempt 2's triage graded it *unclear* |
| `r1-p4-not-staged-route-create-or-author-gate-blocked` | `jigc doc show` | `store.not-staged`'s route is printed for a task whose workflow grants no create |

**Six confirmed in attempt 2 — on no record.** No regression; the ledger still has each as *unclear*; their reports are in [run-pending-a2/](run-pending-a2/):

| key | door | in one line |
|---|---|---|
| `r1-p4-unparseable-adoption-route-refuses-untracked-source` | `jigc doc show` | the adoption route `jigc migrate <path> --as research` exits 1 over an untracked foreign file |
| `r1-p4-not-staged-route-mode-000-repro-b-not-redriven` | `jigc doc show` | over a mode-000 file, the route's read exits 1 and routes back at the first read |
| `r1-p4-not-staged-block-other-unservable-states-not-enumerated` | `jigc doc show` | three more states reach the same arm — a foreign root `CHANGELOG.md` among them |
| `r1-p4-nothing-staged-route-git-add-fails-unreadable-file` | `jigc task finalize` | `finalize.nothing-staged` routes at a `git add` that exits 128 |
| `r1-p4-not-found-not-staged-routes-loop-at-resume-door` | `jigc start` | three refusals that route at each other, over a committed doc made unreadable |
| `r1-p4-reconciliation-rename-route-jigc-rename-not-found` | `jigc validate` | after a staged bare `git mv`, the advisory's first command exits 1 in all seven cells; its second works |

**The 57 for the human** are the ten confirmed above and **47 graded *needs-bound* by triage** — a grade no verifier is launched for, so each is one agent's reading of a report. They are [findings.md](findings.md) §6.

**The planted finding went as a plant goes that nobody stops.** `canary-seeded-claim` — *`jigc doc list` deletes an untracked file* — was found again by triage (`found_again: 1`), graded *breaks*, re-driven by a verifier in 2.5 minutes on the block as written and on a rig, and refuted: *does not reproduce*. It left nothing open. The ledger has it `refuted`, `recorded`.

**What the coming run would want seeded.** The two regressions and the ten confirmed, as verified findings outside this canary's test set; the fork, with both reports; the six of attempt 2 and the four *breaks*, as leads with their blocks; the three whose verifier halted for want of a Linux host or a root caller ([findings.md](findings.md) §5), as a statement of what no verifier of this run can drive. The 47 *needs-bound* and the 114 *unclear* are 161 more leads, and what becomes of them is the fan-out's question.

## 6. The held checks

From each job's `job.json`, `exit.json` and `verdict.json` under the stage's scratch root. Data: [data/held.json](data/held.json).

| job | started | ended | seconds | verdict | what the tool read it from |
|---|---|---|---:|---|---|
| `build-c1-a1` — the candidate | 06:49:24 | 06:49:52 | 28 | green | the build act's own line: `jigc 1.0.0-rc.24`, sha256 `dded1fac…` |
| `build-previous-91834b5e011d` | 06:50:04 | 06:50:31 | 26 | green | the same: sha256 `accf3996…` |
| `gate-c1-a1` — the candidate's gate | 06:50:41 | 07:13:52 | 1,390 | green | the gate's totals line: **`tests   passed=4966 failed=0  (over 18 test binaries)`**, exit 0 |
| `regression-set-c1-a1` | 07:14:16 | 07:36:16 | 1,319 | green | the regression tool's line: **22 differences, 0 of them off the list; 11 tests excluded by a row, 0 by none**; the list's sha256 on record |
| `record-test-r1-a1-1` — the record's gate | 14:51:40 | 15:14:09 | 1,348 | green | the totals line: `passed=4966 failed=0` |
| `build-c1-a2` — the candidate again, attempt 2 | 15:20:54 | 15:21:24 | 30 | green | sha256 `dded1fac…`, the same binary |

**No agent's reading entered.** Each job was started by one `hold-start` call and waited for by one agent that called `hold-wait` again — 15 times for the gate, 14 for the regression set, 15 for the record's gate, once for each build — and returned the tool's digest line, which the harness read as `ran` every time. The verdict file is the tool's, and two of them are in the record ([run/r1/checks/](run/r1/checks/)).

## 7. Against what the canary had to show

The plan's six items, then what it said the canary deliberately does not show, then the plan review's `B5` and the build record's list of what only a real invocation can verify ([harness-agents-gate.md](../harness-agents-gate.md)).

| what | shown? | evidence |
|---|---|---|
| **1.** The stage reaches its record and pushes it, with real agents | **shown** | the attempt's marker, scope, instruments, triage, verifiers, the record's gate, one commit `126a8531`, the push (`vetted: 1`), the state read back; `check`: `pushed: true`, `records: 1` |
| … and the second invocation answering from that state | **shown otherwise** | the state said `triage`, so the second invocation was no answer of *nothing to do* but a lap — which is what the plan's item 4 expected only with the plant. **An invocation with nothing to do was not shown** |
| **2.** A commit that is not checked out, built by the step tool | **shown, both halves** | the previous release at `91834b5e`; and in attempt 2 the round's candidate `eeffe347` with the branch at `126a8531` — the same hash as the first build |
| **3.** A reviewer asserting the binary's hash | **shown** — 89 of 89 returns that carry `asserted_sha256` hold the candidate's; `unasserted` is empty | **but not on a pass that drives nothing**: the source reviewer built rigs and drove the binary (151 tool calls, a `git worktree`). Agents asked the tool for the hash 193 times and typed `shasum` 124 times |
| **4.** How an agent really ends | **shown otherwise** | the plant needs a session that can stop a subagent; a headless one cannot. The usage limit did it instead: the runtime hands the script a failure carrying the limit's message, the harness reads a null return, three tries, then `exhausted`; two exhausted agents trip the breaker; **no dead agent wrote a report afterwards**. An agent that *halts* returns `halted` and has written its report: 4 of 4 |
| **5.** The held commands with real agents | **shown** | §6 |
| … and no permission prompt anywhere in the stage | **not shown** | the session ran in `bypassPermissions`; and its first line was *"Ignoring 3 permissions.allow entries from .claude/settings.json: this workspace has not been trusted"* — a fresh clone is untrusted, so the committed rules did not apply there at all. Five calls were blocked all the same (§2). What observes the rules is the probes' second run, in a session the human watched |
| **6.** Scope and preflight side by side | **shown** | both started 06:48:06 |
| … a trial arm's three steps against the ten-minute ceiling | **shown** | 8.4, 7.6 and 7.8 minutes per step; no call of theirs reached 300 s |
| … a role's own commands under the session's permission mode | **shown under bypass only** | |
| *The digest relayed at real size* | **shown** | 28 of 28 relayed lines whole on the first try, 341 to 2,544 bytes, ASCII ([data/relayed-digests.tsv](data/relayed-digests.tsv)); the digest files on disk run to 19.7 KB ([data/digest-files.tsv](data/digest-files.tsv)), the state document to 394 KB |
| *An agent's own return at size* — the plan's open measurement | **shown** | five triage returns of 26, 56, 68, 124 and 82 KB arrived whole on the first try; the scope's 29 KB; the advocate's 29 KB ([data/agent-returns.tsv](data/agent-returns.tsv)) |
| *A payload at size* | **shown** | the record's batch, 232,267 bytes, written by the executor with its file tool and applied on the first call — 13.4 minutes and 118 k output tokens for it |
| *A second triage pass* (declared not to be shown) | **shown** — four | §4 |
| *Two agents that die; a stage killed as a whole* (declared not to be shown) | **shown** | attempt 2 |
| *A report refused for its text* | **not shown** | 96 calls of the report writer, 96 accepted |
| *A push that fails, a red candidate, a record under a red gate, a ruling, a go, a re-run, the check that reads CI, a cross-model pass, the real remote's rules, the `fix` stage, any later round* | **not shown** | none occurred |
| `B5` — what a tuning commit waits for | **partly** | retries needed: 0 in 100; the breaker's threshold against a parallel batch (§8); 15 asks for a 23-minute wait; the cap of eight |
| The build record's list: relay and payload hashing with real agents (2), the `test` half's git steps (3), reporter death — dead, and halted with a report (7), an unfinished triage in `test` (8), preflight on a commit that is not `HEAD` (9), the cap of eight (10) | **shown** | as above |
| … kill and halt at each boundary (4), the three exits (5), a record under a red gate (6), a reporter that wrote and then died (7), close sync (11), a fork followed by a ruling (12) | **not shown** | |

**The 22 pending paths** are attempt 2's: its marker (`attempt.a2.md`), its preflight's report, its triage's report, and the reports of the 19 verifiers that returned. The stage left them because an attempt writes its reports as untracked files and only the record step commits them — and attempt 2 never reached a record. They are complete reports, written through the report writer.

## 8. What the run shows to be wrong with the tooling

Each is a candidate row; the rows are in [decisions-pending.md](../../../../../implementation/decisions-pending.md) → *What the small canary showed*. **Class 1** is the three the human's ruling of 2026-10-07 has repaired before the run's opening — publish unscanned, record or compute falsely, strand state; **class 2** costs time or tokens; **class 3** is cosmetic.

**Nothing of class 1 was observed.** The one push was the step tool's and vetted; all 96 reports went through the scanning writer; the counts of `first.json` add up (§4); the clone holds one commit of the stage and nothing stray; the state answers a next step. Two things could become class 1 and were not seen to:

| key | what | evidence | class |
|---|---|---|---|
| `wf-neutralized-return` | the runtime rewrites an agent's return in place where it matches an *instruction-shaped pattern* (`<` becomes `<\`) before the harness reads it | two log lines of the first invocation (`arm-control:run`: `bypass-permissions`; a verifier: `settings-json`). No altered byte was found in the record: `<\` occurs nowhere under the run's directory | **1, potential** — a finding's text changed between its author and the ledger, seen by nobody |
| (`wf-b4`, a row already) | a lap that dies records nothing | attempt 2: 19 verdicts, 6 confirmed, in 22 untracked files; the ledger says *unclear* of all 19 | 1 if they are lost, 2 if the next attempt re-verifies them — not established |

**The fan-out** — `wf-verify-fan-out` — is §4 and *The options*.

**Class 2:**

| key | what | evidence |
|---|---|---|
| `wf-unverifiable-relaunch` | a finding no verifier can drive is launched again every lap | `r1-p3-discard-uninstall-plants-linux-and-root-not-driven` halted in attempt 1 and again in attempt 2, for the same reason: the verifier is handed two macOS binaries and the finding asks for Linux and a root caller. Two more halted the same way in attempt 2 |
| `wf-breaker-parallel` | the breaker does not stop a parallel batch, and a usage limit is retried at once | 297 dead launches in 64 seconds; the breaker tripped after 193 and 104 followed; each agent tried three times with no pause |
| `wf-fork-serial` | a contested finding stops the triage loop for its advocate and then its drive, each running a full gate of its own | 83.7 + 54.6 minutes for one fork, 27% of the stage; both waited for their gate with a loop they composed |
| `wf-prelude-serial` | the gate, the regression set and the trial image run one after the other before any instrument | 23.3 + 22.3 + 24.7 minutes; the first reviewer started 73.5 minutes in |
| `wf-image-hold` | the trial image's build is not a held command | the second preflight waited with `until … sleep 15`; two calls ran to the 600-second ceiling. In a session that asks, that shape is a prompt |
| `wf-triage-reads-harness` | triage cannot work from its prompt alone | 14 of its commands read `stabilize.js` — for the arithmetic its counts are held to, the slug rule, the pass bound — and it read the row `wf-r-m5`, whose edit was not made before the canary. Its counts balanced on all five passes |

**Class 3:**

| key | what | evidence |
|---|---|---|
| `wf-return-names` | three names in a return read other than they mean | `status: "triaged"` with the triage unfinished; `unverified` lists 1 row beside `state.untriaged.count: 118`; a halt caused by the usage limit says *"the report check could not be read"* |
| `wf-scratch-escape` | agents write outside the scratch root | 96 commands name `/tmp`; 33 files with such names left there |
| `wf-canary-setup` | three things about the canary's own tool | `expect` — 32 agents, 120 to 150 minutes — counts one triage pass, one verifier and no fork; the clone is an untrusted workspace, so the committed allow rules are ignored there; `check` says `returned.then: false` both for an invocation that never ran and for one that returned and was not written down — which is how this record's brief came to say that it never ran |

**Seen, and no defect of the tooling:** the clause table says *working-product: green* beside two confirmed regressions under that clause — a clause's status is its instruments', and `forbids_close.findings: 177` carries the rest (114 + 4 + 47 + 10 + 2). A triage grades one finding differently on a second look.

**Seen on the machine while this record was made — three things, none of them the canary's.**

- **Stand-ins that outlive their tests — a row already, `wf-test-leaks`.** 51 stand-in processes of the step tool's suites, each a shell playing a held `dev/gate` in a loop of short sleeps, were alive on 2026-10-10, up to three days after the test that started them: 41 from the gates of 2026-10-07 and 10 from 2026-10-08 — five of those started inside the canary's session, one during the candidate's gate and four during the gates the advocate and the independent drive ran themselves. Their rigs are gone; the loops go on. On a machine at rest they cost little: sixteen workers spawned 1,160 processes a second with the 44 day-old ones paused and 1,075 with them running. On a throttled one they cost a factor of four: 150 a second against 35.
- **A gate run by a session at background priority goes red, and the red is the machine's — `wf-gate-starved`.** For about three hours of this record's work the session's whole process tree ran at the system's background priority (4, where 31 is normal); why is not established — the machine was in other use — and it returned to normal by itself. There the gate's fast tier took 48 minutes beside the stand-ins and 18 with them paused, where it takes 17 seconds; and a full gate went **red on the tree of this commit as it then stood: 14 tests ended at the 480-second ceiling of a test and one on a lock that timed out after ten seconds**, with nothing wrong in the tree. The same tree was green twenty minutes later at normal priority (`passed=4966 failed=0`, 1,160 seconds). A held gate of a stage would have put that red on record as the candidate's.
- **The denylist scan leaves its work directory — `wf-hygiene-litter`.** 50,824 of them were in the machine's temporary directory, the oldest of 2026-10-03, each holding the scan's copy of the denylist's patterns.

For the two measurements with the stand-ins paused they were sent a stop signal, and afterwards a continue signal; none was killed, and they run as they were found.

## The options for the fan-out — not decided here

Ruling 5 of [DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-05 — The stabilization workflow, as ruled* has two sentences the fan-out is made of: *an independent agent verifies everything graded breaks or unclear*, and *every entry a fixer, auditor or advocate leaves open becomes a ledger row and is triaged like any finding*. The harness applies the second to a verifier too, and bounds the first at three passes per invocation. Each option below is costed **on this run's own numbers**; "found" means confirmed inside the stage.

| option | verifiers on this run | found | left unverified | what it adjusts |
|---|---:|---|---|---|
| **A. As built** — three verified passes per invocation, laps until nothing is unverified | 61, then 118 owed, then about 300 | 12, then 6 of the first 16 of the lap | never fewer: 10 → 13 → 38 → 117 | nothing — and the stage does not end. 11 h of verifier time so far; the lap owed is about 21 more |
| **B. One verify pass in a `test` stage**; what its verifiers leave open is triaged and recorded, and verified in a later round or not at all | 10 | 1 of the 12 — and the plant refuted | 13 of pass 2's 56 entries, and nothing below them exists | the first sentence, for every pass but the first. The stage would have taken 189 minutes, or 327 with the fork. **Both regressions were found in passes 2 and 3** |
| **C. A cap of N verifiers per pass**, the rest recorded unverified and not launched again in the round | 30 at N = 10 (10 + 10 + 10) | not computable without the order — at the observed one in five, about 6 | 3 of pass 2, 28 of pass 3, all of pass 4 | the first sentence, by a number. Without *not launched again* a cap only slows A |
| **D. Verify only *breaks*; *unclear* goes to the human** | 3, on the passes as they happened; 1 if the chain is followed, since pass 1's only *breaks* was the plant | 2 of the 12, neither regression | 58 *unclear* — of which 10 were real and 47 were not | the first sentence. Triage's *breaks* was right 2 times in 3 and caught 2 of 12 |
| **E. Verify only what names a door of the round's test set**; the rest is recorded for the outside-the-test-set list | 9 (4 + 2 + 3), and 10 of pass 4's 117 | 0 — all 9 were refuted | every confirmed finding of this run | both sentences, by ruling 10's scope. Ruling 4 sends *verified* findings outside the test set to the human; these would arrive unverified |
| **F. A verifier's left-open items are rows, not findings**: recorded with the report, triaged in the next round's `test` | 10 in this stage | 1 | 40 rows, and the 20 of the advocate and the drive | the second sentence, for verifiers — it names a fixer, an auditor and an advocate, not a verifier. Close to B; differs in that the items are not graded now |

Three facts the choice turns on, whatever is chosen:

1. **Depth did not thin the findings.** One verifier in five confirmed at every pass. A bound loses real findings in proportion to what it cuts.
2. **The findings are outside the round's scope.** The round was one door; 0 of 18 confirmed findings are at it. The fan-out is how a scoped round became a hunt over the product — which ruling 10 scopes on purpose (*hunt once per change*).
3. **A real round 1 is larger.** This canary had one review row. Seven rows and a seeded ledger start several times wider; at this run's rates the passes would be several times 10, 13, 38 and 117, at 10.5 agent-minutes and 50 k output tokens a verifier, eight at a time.

## What could not be established

- **Whether attempt 3 would reuse attempt 2's 19 reports**, or count them as another attempt's and launch 118 verifiers again; and whether a halt marked `transient` counts toward the bound on attempts. Nothing was invoked for this record.
- **Whether any of the 18 confirmed findings is real** beyond its verifier's word. None was driven again here.
- **What the 114 *unclear* and the 47 *needs-bound* are, one by one.** The sorting in §4 is a keyword read of 372 keys.
- **What the two neutralized returns held** before the runtime changed them.
- **Why the session ran with permissions bypassed** — how it was launched is not in the files this record read.
- **The gate's and the image's wall time under load**: the three gates ran alone; in a real round they would not.

## What is committed here, and what is not

| path | what |
|---|---|
| [findings.md](findings.md) | the product findings by key: the 2 blockers, the 10 confirmed, the 4 *breaks*, attempt 2's 6, the 3 halted, the 47 *needs-bound* |
| `evidence/first.json` | what the first invocation returned, as the session wrote it down |
| `evidence/then-halted.json` | what the second invocation returned — **read from the runtime's task output after the session ended**; the session never wrote it — with its 698 log lines counted by shape |
| `evidence/first-harness-log.json` | the first invocation's 13 log lines |
| `evidence/check.json`, `evidence/setup.json`, `evidence/session.out` | the canary's own check, run on 2026-10-10; the setup's line; the session's start, end and two messages |
| `evidence/proposal-setup-outside.diff` | the advocate's proposal, byte for byte — a diff nobody applied |
| `data/` | the tables this file cites, extracted from the journals and transcripts by script |
| [run/](run/) | **the run's directory as the stage committed it** — `git archive 126a8531`, 90 files, 2.5 MB: opening, ledger, clauses, test set, the round's scope, triage and results, the two held verdicts, 78 reports |
| [run-pending-a2/](run-pending-a2/) | the 22 files attempt 2 left untracked, 0.6 MB |

**Sanitized:** host paths are `<canary>` (the canary's root), `<scratch-root>`, `<session-store>`, `<repo>`, `<home>` and `<tmp>`; the session id is `<session>`; the limit message's reset time is dropped. So the three evidence files that were single lines are not byte-identical to their originals, whose sizes and hashes are in `data/originals.json`. In `check.json` the 372 ledger rows moved to `data/ledger-as-checked.tsv`: the secret scanner reads a JSON pair of the word `key` and a long slug as a credential, twice. The run's directory and the pending reports are unchanged — they were written through the record script, which replaces host paths itself — and passed the denylist and the secret scan as they stand.

**Not committed:** the session's transcript, the two journals and the 424 agent transcripts (146 MB), the stage's scratch root with every agent's working directory, and the clone. No agent id, no session id and no transcript text is here beyond the few lines quoted.
