# The stabilization workflow's build — the record

**Written 2026-10-06, on `fix/rc24-tier1` at `00825df3`.** This directory holds what the build of the stabilization workflow produced that no commit of it carries: the two independent reviews of the build and the design of its repair, which until this record existed only in a session scratch directory that does not outlive the session. Nothing here grades a finding or repairs anything. **It is written for a reader with no memory of the session**: start with *Where it stands*.

The workflow's one home is [implementation/stabilization-workflow.md](../../../../implementation/stabilization-workflow.md); the rulings it was built from, and every builder's choices inside them, are `DECISIONS.md` → *2026-10-05 — The stabilization workflow, as ruled* and the build entries above it. This record restates neither.

## Where it stands

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

- **Two questions:** what becomes of the *land a part* exit · how a record commit is gated when the candidate's own gate is red. A third, the order of the repair, was ruled while this record was written.
- **Six readings of the orchestrator's, reported on 2026-10-06 and not yet confirmed** — each stands until the human overturns it.

## Files

| path | what |
|---|---|
| [state-machine.md](state-machine.md) | review A, as the reviewer wrote it |
| [harness-agents-gate.md](harness-agents-gate.md) | review B, the orchestrator's transcription of the reviewer's return |
| [repair-plan.md](repair-plan.md) | the repair's design, the orchestrator's transcription of the planner's return |

**Every report here is a lead, not a measurement**, until the repair's own red tests reproduce it.

**Not copied:** review A's rig builder, its helpers and its rigs (`mkrig`, `lib.sh`, `rig.*`), review B's drive directories and its nextest probe crate, and the scratch copies of the script, the harness and the suite they were minted from. They stayed in the session scratch directory. A repro that matters is rebuilt from its block; review A's opening section says how its rig was minted from committed bytes.

**The copies are sanitized for a public repository, by the rule [the fix pass's record](../fix-pass-rc25/README.md) used**: every absolute host path replaced — a session scratch path by `<scratch>/…`, a system temporary path by `<tmp>`, a path into this repository by its repo-relative form — and nothing else changed. **No replacement was needed: zero over the three files**, which are byte-identical to their sources (`cmp`). Review A names its rigs relative to its own directory; review B's transcription already carried `<scratch>/…` in the two lines that name a scratch path, and names the gate's logs by an environment variable's name. The directory was scanned with `command grep -n` for an absolute host path, the login name, an email address, a home-directory form, a session id, the line the install-line fence counts and a credential-shaped string: none found.

**The guard's two scanners ran, on the staged tree before the commit** ([public-hygiene.md](../../../../implementation/public-hygiene.md) → The guard). `dev/hygiene-scan --tree <denylist> HEAD --not --remotes` — this machine's private denylist over the tracked working tree, the staged files of this record among them, which a control search confirmed it reaches: `hygiene: denylist clean (38 pattern(s))`, exit 0. The branch had no unpushed commit, so that run scanned no commit message; this commit's message was matched against the same patterns before it was written into the commit. gitleaks 8.30.1 with this repository's `.gitleaks.toml`, over this directory (`gitleaks dir`) and over the whole staged diff (`gitleaks git --pre-commit --staged`): no leaks found, exit 0 both times. `dev/gate`'s own hygiene advisory ran again with the commit's gate.
