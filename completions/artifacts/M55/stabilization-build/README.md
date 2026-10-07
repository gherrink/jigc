# The stabilization workflow's build — the record

**Written 2026-10-06, on `fix/rc24-tier1` at `00825df3`.** This directory holds what the build of the stabilization workflow produced that no commit of it carries: the two independent reviews of the build and the design of its repair, which until this record existed only in a session scratch directory that does not outlive the session. Nothing here grades a finding or repairs anything. **It is written for a reader with no memory of the session**: start with *Where it stands*.

The workflow's one home is [implementation/stabilization-workflow.md](../../../../implementation/stabilization-workflow.md); the rulings it was built from, and every builder's choices inside them, are `DECISIONS.md` → *2026-10-05 — The stabilization workflow, as ruled* and the build entries above it. This record restates neither.

## Where it stands

**[2026-10-07 — the list below is the record's as it was written on 2026-10-06. Since then the `test` half was repaired along the plan and re-reviewed by two independent reviewers: fit for a canary run, not yet fit for the real run. Where that stands is the section *The re-review of the `test` half* below, and the directory it points at.]**

- **The workflow is built, and it is not fit for a real run.** Two independent reviews of the build came back **red**. **No stage of it is used on a real run until that stage's repair and re-review are recorded in `DECISIONS.md`** — the `test` half first, by the human's ruling on the order.
- **The repair is designed and not started** ([repair-plan.md](repair-plan.md)). Its order is ruled; two questions about it are open with the human, and six readings the orchestrator took are reported to him and not yet confirmed (→ *Open with the human*).
- **No run is opened, and no stage has been driven by the Workflow runtime.** The self-test under the runtime and the canary run — the build's own verification, `DECISIONS.md` → the rulings entry → *The build* → *Verification*, items 1 and 3 — have not run; the plan puts the canary in the repair's last task.
- **`CLAUDE.md` → *Project state* still gives the order the stabilization rulings replaced**, and its list of the workflow docs names five of six. Neither is corrected by this record's commit: its builder is a subagent and does not edit that file on an agent's brief. Both are handed back to the orchestrator (`DECISIONS.md` → the entry of 2026-10-06, *Not done by this commit*).
- **The fix pass this workflow was built for stands where its own record left it**: un-landed, its last round unaudited ([the pass's record](../fix-pass-rc25/README.md)).

## What was built

`git log --oneline 357ab425..HEAD`, oldest first — ten commits, each sha and subject checked against that log for this record. *Task* is the build's own numbering (`DECISIONS.md` → the rulings entry → *The build* → *The order*); *added after N* is a task the build gained while it ran, and the opening record is no task. *Full gate* is the `passed=` total of the full `dev/gate` **as its builder reported it**; none was re-run for this record.

| commit | task | what | full gate, as reported |
|---|---|---|---|
| `fa2ac313` | — | the opening record: the rulings, and the build's shape | 4635 |
| `967ca491` | 1 | the tiered gate: `dev/gate` stops at the first red stage, runs the suite in two tiers, keeps its timings | 4645 |
| `d3f2445c` | 2 | the record script, `dev/stabilize-record`, and its suite | 4678 |
| `22bcc734` | added after 2 | the run's state as data: `state`, and where a finding goes and what a round does next computed in the script | 4688 |
| `f729e52f` | 3 | the agent definitions: four new, six contracts changed | 4688 |
| `914e61c8` | 4, first commit | the record script extended for the harness: the test set as rows, the round record's facts, the position of each stage | *see below* — 4692 |
| `5146d080` | 4 | the harness, `.claude/workflows/stabilize.js`, and the fence over its source | 4702 |
| `3dbc5612` | added after 4 | the decision table's six cells that answered `unsettled` | 4708 |
| `f6918aaa` | added after 4 | close only from a tested candidate; the decision table's holes | 4712 |
| `00825df3` | 5 | the workflow's doc | 4712 |

**One commit reached the branch without a gate on its own tree.** `914e61c8` was committed together with `5146d080` and gated only as part of it, because a `git stash` to gate it alone was denied. It was gated afterwards on its own tree, in a separate worktree: 4692 passed, 0 failed.

## What the two reviews found

Both are independent and read-only, and both read `357ab425..f6918aaa` — nine of the ten commits. **The workflow's doc (`00825df3`) was written while they ran and was read by neither.** No finding is restated here; the reports are the findings.

| | file | verdict | what it holds |
|---|---|---|---|
| **A** — the state machine: the record script and its suite | [state-machine.md](state-machine.md), written by the reviewer | **red** | 13 findings, `F1`–`F13`: 3 HIGH · 4 MEDIUM · 6 LOW; three more on the suite, `S1`–`S3`; one nextest mark looked into and not confirmed; a crash-consistency table; what was constructed and found correct; what was not examined |
| **B** — the harness, the agent definitions, the gate tooling | [harness-agents-gate.md](harness-agents-gate.md), **the orchestrator's transcription** of the reviewer's return — that reviewer writes no file | **red** | 29 findings: 5 HIGH (`H1`–`H5`) · 12 MEDIUM (`M1`–`M12`) · 12 LOW (`L1`–`L12`); four leads; what was checked and found correct; what was not examined; twelve things only a real invocation can verify |

The repair plan qualifies four of the reviewers' claims and refutes none (its §7).

## Found while the workflow's doc was written — read off the source, not driven

The builder of the doc (`00825df3`) reported five places where what is built contradicts what is recorded. **The doc itself states each one**, so each line is a pointer into [stabilization-workflow.md](../../../../implementation/stabilization-workflow.md) and restates nothing:

1. **A `test` stage stopped after its scope step, and the next invocation's git-state step** → *Halt and resume*, the list *What a halt leaves*; and *Open questions and declared bounds* → *To be shown by the self-test under the runtime and by the canary*. Named by neither review and by no task of the plan.
2. **A red deterministic check and the way to a fixer** → *Triage, the human's list and the forks* → *A red deterministic check*; *Open, the human's to rule*. Review A leaves it as a lead under *Not examined*; the plan lists it with the leads it fixes.
3. **A fork has no row in any table** → *Open, the human's to rule* → *A fork has no row*. Review B's `M3`.
4. **The close's sync merge and the path rule's subject check** → *Close*, step 3; *Open, the human's to rule* → *A red check on the pull request after the sync*. Review B's `M8`.
5. **The retired `probe` gate step, still named by three definitions a run launches** → *Declared bounds*, the last item. Eight definitions carry the sentence in all (`DECISIONS.md` → *2026-10-05 — The agents of a stabilization run* → *Not built*); review B's `L8`.

**And one fact about the doc's own commit, as its builder noted it:** the doc's registration in `crates/cli/tests/doc_link_fence.rs` is a change under `crates/` committed directly on the loop branch. It is harmless only because no run is opened yet, and gone as a class once the tooling suites move out of `crates/` (→ *The repair*).

## The repair

[repair-plan.md](repair-plan.md) is the design, returned by a read-only planning agent and **transcribed by the orchestrator**; nothing in it was run. It holds seven structural changes (`C0`–`C6`), a disposition for every finding of both reviews, eleven states the suite's truth tables lack, fourteen tasks (`0`–`13`) with their order, three ways to treat the *land a part* exit, what the planner would cut, and four decisions it returned as the human's. **The repair works from that plan, from both review reports, and from the five lines of the section above** — the first of which the plan does not reach.

What was decided about the repair since the plan was returned — the human's ruling that the tooling's test suites move to a home of their own outside `crates/`, which is the repair's first task; his ruling on the order, the `test` half first; and the orchestrator's readings — is `DECISIONS.md` → *2026-10-06 — The stabilization workflow's build, reviewed red*. What the plan sets aside rather than repairs has its rows at `implementation/decisions-pending.md` → *The stabilization workflow — what its settling deferred*.

## Open with the human

`DECISIONS.md` → the entry of 2026-10-06 named above is the home of both lists; in one line each:

- **Two questions:** what becomes of the *land a part* exit · how a record commit is gated when the candidate's own gate is red. A third, the order of the repair, was ruled while this record was written. **[The second was ruled 2026-10-06, after this record: a record commit is accepted if its gate turns nothing red that the candidate's own gate did not already show red — `DECISIONS.md` → *2026-10-06 — A stabilization run's record commit under a red candidate*. Not built. *Land a part* was ruled the same day, in the same entry: a part lands by revert on the round branch, the plan's option C. Not built. Nothing is open with the human.]**
- **Six readings of the orchestrator's, reported on 2026-10-06 and not yet confirmed** — each stands until the human overturns it.

## The re-review of the `test` half, and the plan of its second repair

**Recorded 2026-10-07, on `fix/rc24-tier1` at `cbb3d736`.** The repair of the `test` half is the eighteen commits after this record's own (`git log 603018ac..cbb3d736`), and the `DECISIONS.md` entries of 2026-10-06 from *The tooling's own test suites live in `tooling-tests/`* to *The gate, repaired*; the last two commits of the range added the regression set's first part and the record of its first run. It was then re-reviewed by two independent, read-only reviewers who did not see each other's report, over that range. Their reports and the plan that follows them are in [re-review-test-half/](re-review-test-half/plan.md):

| | file | verdict | what it holds |
|---|---|---|---|
| **A** — the record script and the state machine | [re-review-test-half/records-state.md](re-review-test-half/records-state.md), written by the reviewer | **fit for a canary run of `test`; not yet fit for the real run** | 16 findings, `R1`–`R16`: 1 HIGH · 7 MEDIUM · 8 LOW; where each of the first review's thirteen findings stands (ten closed, one partly, two not); eighteen mutants, all killed; a table of what a fresh invocation reads after a kill at each point of a record step; its helpers, verbatim |
| **B** — the harness, the agents, the gate, the regression tool, and the workflow doc read whole for the first time | [re-review-test-half/harness-doc.md](re-review-test-half/harness-doc.md), written by the reviewer | **the same** | 20 findings: 2 HIGH (`R-H1`, `R-H2`) · 8 MEDIUM (`R-M1`–`R-M8`) · 10 LOW (`R-L1`–`R-L10`); a disposition for each of the first review's twenty-nine (sixteen closed, three in part, ten not — seven of them the `fix` half's); the reporter × ending table; which lines of the harness the simulation executes; sixteen sentences of the doc that are false or lack what the code does; six leads |
| the plan | [re-review-test-half/plan.md](re-review-test-half/plan.md), written by a planning agent and **revised the same day** after its review and under the human's ruling on scale | — | every HIGH and MEDIUM finding driven again at the tip (eighteen confirmed, none refuted); six causes; the disposition of each finding of the plan's review; **the core** — nine builder tasks (`K0`–`K8`), five small optional ones, and the orchestrating session's own steps; the runtime probes as a step; the small canary; what a round-1 stage can still do wrong; every finding with its home or its row; what is left with the human; what the `fix` half's repair is owed in addition |
| the plan's review | [re-review-test-half/plan-review.md](re-review-test-half/plan-review.md), **the orchestrator's transcription** of a `design-reviewer`'s return — that reviewer writes no file | **build after the named changes** | ten blocking findings (`B1`–`B10`), eight advisory points, a reading of each of the five forks, seven decisions it found buried as settled |
| the runtime probes | [re-review-test-half/probes/](re-review-test-half/probes/README.md): the four judged results as `dev/stabilize-probe` wrote them, and a README | — | run 2026-10-07 at `1d21024e`: a relayed line never came back whole, at any size; a payload of 300 entries was written whole; a return that omits a required field throws, three tries of three; a command of thirty-five minutes was held by one agent, and outlived its starter. And three things no probe was built for: a workflow invoked by name runs a stale copy, the hold probe asked the human for a permission, and a reviewer left four LOW findings on the probe tool |

**Together: 3 HIGH · 15 MEDIUM · 18 LOW, and nothing built on them yet.** Both reviewers say that no defect found records something false and that each halts on the safe side; the plan qualifies that for one finding (`R5`, whose end state is a wrong `close`). The plan is a design: nothing of it was built, and its *done when* facts are tests that do not exist.

**The plan was reviewed before anything was built from it, and the human then ruled its scale (2026-10-07).** The review's ten blocking findings were each checked — three driven again — and all ten accepted, one of them in part; the first cut's canary and three of its five forks did not survive it. The human's ruling, relayed by the orchestrator, is **(B)**: the plan's core now, the runtime probes, a small canary that only shows one `test` stage running end to end with real agents, then the run's opening — round 1 of the real run is the tuning round, and everything else from the two reviews and the plan's review is a recorded row, fixed between rounds. The plan as committed is the revision under that ruling; its first cut is this file's history (`a7d15955`). **Amended once more the same day, after its first two tasks had landed and the probes had run**: two tasks are promoted into the core — the harness reads a small digest and never a document; a long command is started, waited for and judged by the tool — and the order is recut so that what the harness reads changes once (`DECISIONS.md` → *2026-10-07 — The runtime probes, run*). `DECISIONS.md` → *2026-10-07 — The second repair of the `test` half, reviewed and ruled on scale*.

**The copies are sanitized by the rule this record states under *Files*.** Review A's report is byte-identical to its source (`cmp`). Review B's differs in one place: its one relative link to the first review's report, which lies one directory up from its new home, was retargeted (`harness-agents-gate.md` → `../harness-agents-gate.md`). The plan's review is byte-identical to the transcription the orchestrator handed over (`cmp`). Neither holds an absolute host path, a login name, a session or agent id, or a credential; the one term that looks like a denylist entry (`zzqprivatetermzz`, in review A's blocks) is the synthetic denylist of its rigs. Their rigs, scenario scripts and outputs stayed in the session scratch directory: a repro that matters is rebuilt from its block, and review A's appendix holds its helpers whole.

## Files

| path | what |
|---|---|
| [state-machine.md](state-machine.md) | review A, as the reviewer wrote it |
| [harness-agents-gate.md](harness-agents-gate.md) | review B, the orchestrator's transcription of the reviewer's return |
| [repair-plan.md](repair-plan.md) | the repair's design, the orchestrator's transcription of the planner's return |
| [pre-opening/](pre-opening/README.md) | **added 2026-10-06, after this record, and no part of the build's record:** the human's rulings ahead of the run's opening, the files they were taken from, and the orchestrator's lessons of the build and repair, folded into no doc yet — kept beside the build's record, with a README of its own. **Extended 2026-10-07:** the ruling on the port rehearsal's shape with the facts it was taken from ([port-rehearsal-facts.md](pre-opening/port-rehearsal-facts.md)), four more lessons, and the orchestrator's log of the fixes built from those rulings — each fixer's result, the human's rulings on the four items returned to him, one of them pending, and what each fixer left open ([pre-opening-fixes-log.md](pre-opening/pre-opening-fixes-log.md)) |

**Every report here is a lead, not a measurement**, until the repair's own red tests reproduce it.

**Not copied:** review A's rig builder, its helpers and its rigs (`mkrig`, `lib.sh`, `rig.*`), review B's drive directories and its nextest probe crate, and the scratch copies of the script, the harness and the suite they were minted from. They stayed in the session scratch directory. A repro that matters is rebuilt from its block; review A's opening section says how its rig was minted from committed bytes.

**The copies are sanitized for a public repository, by the rule [the fix pass's record](../fix-pass-rc25/README.md) used**: every absolute host path replaced — a session scratch path by `<scratch>/…`, a system temporary path by `<tmp>`, a path into this repository by its repo-relative form — and nothing else changed. **No replacement was needed: zero over the three files**, which are byte-identical to their sources (`cmp`). Review A names its rigs relative to its own directory; review B's transcription already carried `<scratch>/…` in the two lines that name a scratch path, and names the gate's logs by an environment variable's name. The directory was scanned with `command grep -n` for an absolute host path, the login name, an email address, a home-directory form, a session id, the line the install-line fence counts and a credential-shaped string: none found.

**The guard's two scanners ran, on the staged tree before the commit** ([public-hygiene.md](../../../../implementation/public-hygiene.md) → The guard). `dev/hygiene-scan --tree <denylist> HEAD --not --remotes` — this machine's private denylist over the tracked working tree, the staged files of this record among them, which a control search confirmed it reaches: `hygiene: denylist clean (38 pattern(s))`, exit 0. The branch had no unpushed commit, so that run scanned no commit message; this commit's message was matched against the same patterns before it was written into the commit. gitleaks 8.30.1 with this repository's `.gitleaks.toml`, over this directory (`gitleaks dir`) and over the whole staged diff (`gitleaks git --pre-commit --staged`): no leaks found, exit 0 both times. `dev/gate`'s own hygiene advisory ran again with the commit's gate.
