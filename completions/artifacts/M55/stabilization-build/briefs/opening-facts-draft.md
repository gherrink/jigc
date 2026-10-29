# The opening's fact sheet — the first stabilization run

*Gathered 2026-10-27 by the read-only `stabilize-opening` definition, on the loop branch `fix/rc24-tier1` at `495991e32bd40d1be3859569c5a0e7b42c076be9`, the tree clean before and after. Its outline is `implementation/stabilization-workflow.md` → *Opening a run* — the list *What it decides* and the table *What goes in as data* — as that commit holds it. Every fact names its source and how it is known: **exercised** — a `git`, `gh`, checksum or record-script read-only call run for this sheet, at this commit; **read** — the code or a doc says so at this commit (a count taken off a committed file is *read, counted here*); **recorded** — a record says so and this sheet did not re-establish it. Paths are relative to the repository's root. This sheet holds no recommendation, ranking or preference: each question's recommendation is the main session's, and every decision is the human's.*

---

## 0. What is owed before the opening

*Added at the coordinator's request, same rules: facts only, each with its source and how it is known. "Struck/discharged" means the row is struck through or carries a dated discharge mark; "built but unstruck" means the code or a commit that builds it is shown here, exercised or read; "open" means nothing found builds it.*

### 0.1 The enumeration

**The commands.**

- `implementation/decisions-pending.md`: `command grep -n -i -E "opening|real run|before round 1"` — **61 hits**. Kept: every line whose trigger or stated condition is *before the run's opening*, *the run's opening* / *at its opening*, *before round 1's `test` stage*, *the first real run*, or a condition that makes it due before the opening — **30 rows**, listed below. Dropped: 31 hits where *opening* is no trigger of this run's opening — the word in other senses (*a wave opening `doc author`*, *opening the repo*, *the artifact's opening sentence*), triggers of other events (*the re-audit*, *the pass's close*, *the next candidate's instruments*, *the re-review of the `test` half*, *between rounds 1 and 2*, *M57*, *the first run's close, for the next run's opening*), and section headings. — *exercised*.
- `implementation/stabilization-workflow.md`: `command grep -n -i -E "before the run's opening|before the opening|before a real run|on a real run|the real run|before round 1|before a run is opened|at the run's opening|at its opening|before (it|a run) opens|ahead of the opening"` — **14 hits** (lines 13, 93, 545, 566, 577, 609, 629, 639, 657, 669, 679, 703, 706, 716). Kept: the sentences that state a condition or a trigger — **7 items** below. Dropped: 566 (a fact about where the guard binds), 609 (a run described as *before the eleven fixes built ahead of the opening*), 629 (the reason for ruling 4), 679 (a struck, dropped item). — *exercised*.

**`decisions-pending.md` — 30 rows.** Line numbers are of the file at `495991e3`.

| # | line | key or heading | trigger | state |
|---|---|---|---|---|
| 1 | 74 | eleven fixes, 1 · item 6 — `milestone finalize` refuses over a discarded sub-task's unstaged and untracked files | before the run's opening | **discharged** (marked 2026-10-06, not struck through) — `855fef44`, an ancestor of the tip |
| 2 | 75 | eleven fixes, 2 · item 17 — `setup`'s own code where git cannot answer | same | **discharged** — `90fee322` |
| 3 | 76 | eleven fixes, 3 · items 7 and 8 — one code for *this path is not an ordinary file* | same | **discharged** (after a return to the human and a ruling) — `23dfe3e8` |
| 4 | 77 | eleven fixes, 4 · item 9 — `migrate-corpus` and a symlinked home | same | **discharged** (ruled a third time 2026-10-07) — `0ad081cd` |
| 5 | 78 | eleven fixes, 5 · item 18 — `setup` follows a link to an ignored file | same | **discharged** — `aac460ea` |
| 6 | 79 | eleven fixes, 6 · item 15 — the exit after `jigc unmanage` | same | **discharged** (after a return to the human) — `a2ccdac0` |
| 7 | 80 | eleven fixes, 7 · item 1 — `setup` after a deleted tracked install file | same | **discharged** — `4f680249` |
| 8 | 81 | eleven fixes, 8 · item 19 — the hook's end marker | same | **discharged** — `dafafb95` |
| 9 | 82 | eleven fixes, 9 · item 12 — the record commands' refusal names the exit | same | **discharged** — `30eb446c` |
| 10 | 83 | eleven fixes, 10 · item 21 — `setup` records the permission entries it added | same | **discharged** — `1c7d391c` |
| 11 | 84 | eleven fixes, 11 · item 3 — the invocation-log refusal says the order | same | **discharged** — `cea9755a` |
| 12 | 85 | *What the run's opening record carries from the decisions taken ahead of it* | the run's opening | **open** — an act of the opening; no opening record exists (1.7) |
| 13 | 86 | *What the fixers ahead of the opening left open — rows of the run's ledger at its opening* | the run's opening | **open** — no ledger exists (7.13) |
| 14 | 89 | *A long list of findings has no shape a model can work with* | the run's opening (an opening row) | **open** |
| 15 | 91 | *A hold of `dev/stabilize-probe` ended 50 ms early under load* | the run's opening (an opening row) | **open** |
| 16 | 96 | `wf-route-catalog-merge` | before the stabilization run's opening | **struck** — *DISCHARGED 2026-10-27*, merged as `e771c46d` (an ancestor of the tip; the branch's tip too — 1.6) |
| 17 | 100 | *What the repair's design sets aside — its *record, not now* items* | the first real run | **open** |
| 18 | 102 | *What the move of the tooling's test suites left behind* | before the repair's next task, and in any case before a run is opened | **struck** — *DISCHARGED 2026-10-06* (`7d732172` and the commit that carries the mark) |
| 19 | 112 | `wf-canary` | before the run's opening | **struck** — *DISCHARGED 2026-10-10*: the canary ran 2026-10-08 |
| 20 | 118 | `wf-verify-fan-out` | before the run's opening | **built but unstruck** — in part, on the path of a run with an order (0.2) |
| 21 | 119 | `wf-route-walk` | before the run's opening | **open** — nothing in the code (0.2) |
| 22 | 122 | `wf-canary-seed` | the run's opening | **open** — an act of the opening |
| 23 | 123 | `wf-neutralized-return` | before the run's opening — *"of the class that records falsely"* | **open** — no commit message, `DECISIONS.md` line or harness line names it (`git log -i --grep="neutraliz"`: 0; `command grep -i "neutraliz"` over `DECISIONS.md`, the workflow doc and the harness's source: no hit naming this row) |
| 24 | 128 | `wf-image-hold` | between rounds 1 and 2 — *and before the run's opening if its session asks for permissions* | **open**, conditional — the step tool's held kinds are `gate` and `regression` only (4.2); the run is in `bypassPermissions` by the ruling of 2026-10-16 (the workflow doc, line 679's bracket) |
| 25 | 152 | `wf-regression-not-comparable` | `T10`, and before the run's opening | **open** — the two texts still differ at this commit (section 4, second disagreement) |
| 26 | 164 | `wf-x1b` | that merge; before the run's opening if it is cheap | **struck** — *DISCHARGED 2026-10-08*, the commit that finished `X1b` |
| 27 | 168 | `wf-return-size` | between rounds 1 and 2 — *and before the run's opening if the opening seeds far more than a hundred untriaged rows* | **open**, conditional on the opening's seed count |
| 28 | 169 | `wf-never-selected` | the run's opening, with the declared bounds | **open** — the human's to rule |
| 29 | 275 | *The regression set, part 1* | before round 1's `test` stage | build **discharged** 2026-10-08 (`8f13ea7b`, `b36b8c93`, ancestors of the tip); still owed by the row and **open**: *"the test-set row that runs it, written at the run's opening"* |
| 30 | 278 | *The declared bounds, as one list with reach* | (the row) *"The one list at the run's opening is still owed"* | **open** |

Counts: **struck/discharged 15** (rows 1–11, 16, 18, 19, 26) · **built but unstruck 1** (row 20, in part) · **open 14** (rows 12–15, 17, 21–25, 27–30; row 29 open for its remaining item). The eleven fixes' heading (line 73) reads *"all eleven built"*; their eleven commits are each an ancestor of the tip (`git merge-base --is-ancestor`) — *exercised*.

**`stabilization-workflow.md` — 7 items.**

| # | line(s) | the sentence | state |
|---|---|---|---|
| A | 13, 639, 669 | *"no stage is used on a real run until the repair of its half and the re-review of that repair are recorded in DECISIONS.md — the `test` half first"*; *"Nothing refuses the `test` stage on that ground: that it is not used on a real run before its record says so is the orchestrator's rule to keep."*; *"The rule binds as before: `test` is used on a real run once this second repair and the review of it are recorded in DECISIONS.md — that record is the orchestrator's to write, and this doc does not anticipate it."* | **open** — no record found (0.3) |
| B | 93 | *"the test set per instrument — fixed before round 1 (ruling 4)"* | **open** — an act of the opening |
| C | 545 | the `payload` probe: *"a failure makes that row due before the opening"* | **condition not met** — both recorded runs of the probe are *whole, four of four* (20, 60, 120 and 300 entries): `re-review-test-half/probes/payload.json` and `payload-run2.json`, read here — *read* |
| D | 577 | *"One `test` stage with real agents, once, before a run is opened"* — the small canary | **discharged** — it ran 2026-10-08 (`wf-canary`, struck; row 19) |
| E | 657 | *"what can publish unscanned, record falsely or strand state is repaired before the run's opening; round 1 of the real run is the tuning round"* | **built** — the core's tasks `K1` `e76ef470`, `K2` `b34fb073`, `K4` `3c6a889d`, `K5`/`K6` `8f13ea7b` `b36b8c93`, `K9` `c2169e19`, `K10` `bbe873eb`, and the core review's fix pass merged by `6e78975b`, each an ancestor of the tip (*exercised*); the same section lists *"What is still defective in it — each a row with its due point"* |
| F | 703, 716 | *"The declared bounds of the first run — the list below, theirs to accept at the run's opening"* | **open** — an act of the opening |
| G | 706 | *"Its build is owed before the run's opening (`wf-verify-fan-out`, `wf-route-walk`, `wf-route-catalog`)"* | mixed — `wf-verify-fan-out` **built but unstruck** in part (0.2); `wf-route-walk` **open**; `wf-route-catalog` **struck**, discharged 2026-10-12 (`decisions-pending.md` line 120) |

Counts: **discharged 1** (D) · **built 1** (E) · **condition not met 1** (C) · **mixed 1** (G) · **open 3** (A, B, F).

### 0.2 The reshaped `test` stage — what is in the code

Read in `.claude/workflow-sources/stabilize.js` (the source; the invoked file is generated from it) and `dev/stabilize-record`, at `495991e3` — *read*.

| piece | in the code? | where |
|---|---|---|
| the queue | **yes** | record script: `has_queue` (line 3705), `known_queue` (3712), `queue_of(order, rows)` (5697), `RUN_QUEUE = ("budget", "hand-on", "order", "queued")` (2553); harness: `hasQueue(state)` (1686), `queueSaid` (1900), and the branch `if (hasQueue(state)) {` (≈5065) — *"EVERY WAY OUT OF THIS BLOCK IS A RETURN: nothing below it runs for a run with a queue"* |
| the budget | **yes** | record script: `RUN_FACTS` (2544), `RUN_NUMBERS` (2570); harness: `QUEUE_RULINGS = ['order', 'queued', 'budget']` (1283); a wave is *"never larger than what the budget has left"* (780), `WAVE = 8` (781) |
| the hand-on | **yes** | record script: `RUN_FACTS` (2544), the filed verdict's `hand_on` (575); harness: `hand-on` written once (1301), `queue.hand_on` in the digest (1906), `verdictPrompt(…, handOn)` (2840) |
| the order | **yes** | record script: `RUN_QUEUE`, `RUN_LISTS` (2573), `queue_of(order, rows)`; harness: the `order` ruling in `QUEUE_RULINGS` |
| the route hunter (`wf-route-walk`) | **no** | `command grep -c -i "hunter"` over the harness's source: 0; no `CHAINS` kind for it (4.2); no definition under `.claude/agents/` whose name holds `route` (`ls`: 0) — *exercised* |
| `wf-verify-fan-out`'s owed pieces | **in part** | triage's kind with its evidence on every new entry — `.claude/agents/finding-triage.md` lines 49–51; one triage in a run with a queue — `triageOnce` (4255–4264); verification in waves out of the queue — `verifyWaves` (4313); the hit rate per class at every stop — record script `view` (header ≈1060), `stop_view` (5750); the leads list — `ledger-add` prints `leads` (header 156, 180); the advocate out of `test` — on the queue path only (header line 255: *"NO ADVOCATE IS LAUNCHED ON THAT PATH"*); **`VERIFY_PASSES` is not gone**: `const VERIFY_PASSES = 3` (776), read by `triagePasses` (4181–4249), the path of a run without an order (4685) |

**The commits that built the queue path**, each an ancestor of the tip (*exercised*): `fbb17209` (2026-10-12 06:27, *the harness's new path, its triage*), `adcd6935` (09:33, *records before it verifies*), `bc8fbe85` (14:51, *verifies in waves what the queue prints*), `196a4de7` (16:06, *continues a stage*), `b717fb96` (21:21, *takes the rulings about the queue*).

**Which source is stale, as a fact.** Line 706 of the workflow doc — *"**Ruled 2026-10-10 and not built yet:** the `test` stage this doc describes is the one that stands in the code"* — was last written by `d591f656` (2026-10-11 00:13; `git blame -L 706,706`), before the five commits above; first written by `95f77b57` (2026-10-10). Its *"not built yet"* is stale against the code for the queue, the budget, the hand-on, the order and the verify fan-out's waves; it is still true of the route hunter, and of `VERIFY_PASSES`, which stands on the path without an order. The `DECISIONS.md` entries of 2026-10-12 match the code. — *exercised*; *read*. (This settles section 4's first disagreement, kept there as it was found.)

### 0.3 The condition of a real run — whether any record satisfies it

**The condition**, quoted: *"**The rule binds as before**: `test` is used on a real run once this second repair and the review of it are recorded in DECISIONS.md — that record is the orchestrator's to write, and this doc does not anticipate it."* (line 669); and *"no stage is used on a real run until the repair of its half and the re-review of that repair are recorded in DECISIONS.md — the `test` half first"* (line 13); `NOT_FIT` names only `fix` (harness, `NOT_FIT`). — *read*.

**The searches, and what they found** — *exercised*:

- `git log -i --grep="repaired and re-reviewed"`: **0**. `git log -i --grep="fit for a real run"`: **0**. `git log -i --grep="fit for the real run"`: **1** — `a7d15955` (2026-10-07), *"re-reviewed — fit for a canary run and not yet for the real run"*.
- `git log -i --grep="second repair"`: **22** — the second repair's builds of 2026-10-07/08 and `c89d2cab` (2026-10-07, *"the second repair of the `test` half, reviewed and ruled on scale"* — a review of the repair's **plan**, before it was built). None says the built repair was reviewed fit for a real run.
- `git log -i --grep="real run"`: **16**; none states that `test` may be used on a real run.
- `command grep -n -i -E "fit for (the|a) real run|used on (the|a) real run|for the real run|repaired and re-reviewed" DECISIONS.md`: hits at lines 8416 and 8420 (the 2026-10-07 re-review: *"not yet fit for the real run"*), 9037–9043 (the `fix` stage's refusal: the commit that deletes `NOT_FIT`'s line records *"the `fix` half's re-review"*), 9121–9129 (2026-10-06, *reviewed red*). None records the `test` half's second repair as reviewed and fit.
- `command grep -rn -i -E "fit for (the|a) real run|used on (the|a) real run|repaired and re-reviewed" completions/artifacts/M55/stabilization-build/`: 7 lines in 5 files — the build's `README.md` (lines 9, 11, 78), `re-review-test-half/records-state.md` (14) and `harness-doc.md` (5), both *"not yet fit for the real run"*, `re-review-test-half/plan-review.md` (49), and `re-review-test-half/plan.md` line 518, which put to the human: *"Whether the core's diff is reviewed before round 1 … (a) a focused, independent review … (b) none: round 1 is the tuning round for this too, and the rule is set aside for this half by their word. Recommendation: (a)."*

**Reviews that exist after the 2026-10-07 re-review** — *read*:

- *2026-10-07 — The core's script-side tasks, reviewed for three dangers* (`DECISIONS.md` line 7944; report `re-review-test-half/core-review.md`): one reviewer, `K1`, `K2`, `K4`, `K5`/`K6`, `K9`, `K10` at `bbe873eb`, *"for three dangers and for nothing else"*; verdict *"no finding blocks the small canary"*. It does not name `K3` or `K11`, and states nothing about a real run.
- The scoped review `R0` before part 1 of the staged canary (2026-10-12/13; its fixes `c120cf44`, `7725f5d6`, `c36d3316`), *"for three dangers"*, recorded as *"said to be a deviation, the human's to overrule"* (`reshape/staged-canary.md` line 613). It states nothing about a real run.

**Found:** no record in a `DECISIONS.md` body, a review file under `completions/artifacts/M55/stabilization-build/`, or a commit message that states the `test` half's second repair as recorded and reviewed for a real run.

### 0.4 `wf-whole-canary-after-parts` — whether its trigger has passed

- **The row**: *"after part 4, the human's decision how much of that canary is still run — on the parts' own records …"*; trigger *the reading of part 4*; unstruck. — `decisions-pending.md` line 155 — *read*.
- **Part 4 ran**: *"Part 4 of the fifth root ran as written (`staged-canary.md` → Part 3, the note of 2026-10-18)."* — `DECISIONS.md` line 1683, the entry *2026-10-18 — A stage's return is read from its notification only*; line 1673 names *"The fifth canary root, part 4 act 1"*. The note it points at, added by `93310ff8` (2026-10-18), begins *"[Noted 2026-10-18, after the fifth root: this is what happened."* — *read*; *exercised* (`git show`).
- **A reading of part 4, or the decision it owes**: `command grep -n -i "whole canary\|wf-whole-canary"` over `DECISIONS.md`, `implementation/*.md` and `staged-canary.md` — hits only in the entry of 2026-10-12 (lines 4660–4726), the pending row itself, and the plan's own text (`staged-canary.md` lines 3, 36, 40, 406, 463, 534–546, 616–617), all written before part 4 ran. `git log -i --grep="whole canary\|wf-whole-canary\|reading of part 4"`: **1** — `a6bb93e5` (2026-10-12), the ruling that made the row. `command grep -rn -i "part 4" completions/artifacts/M55/stabilization-build/`: 36 lines, all in `staged-canary.md`, none a reading of part 4's results or a decision on the whole canary. — *exercised*.
- **So**: the event its trigger waits on — part 4 run — happened by 2026-10-18 on the records; no committed record of the *reading* of part 4, nor of the human's decision the row owes, was found. The row's owed decision is **open**.

---

## 1. The run's slug, and with it its loop branch

**Facts**

- **1.1** The harness mints every branch name of a run in one function, `branchName(run, round, cut)`: the loop branch is `fix/<run>`, a round's branch `fix/<run>-r<N>`, a part `fix/<run>-r<N>-part<k>`; the run's directory is `completions/artifacts/<run>` (`RUNS_ROOT`, `runDir`). — `.claude/workflow-sources/stabilize.js`, `branchName` and `runDir` (the function's own comment says the `stabilize/` type is "this function's change and no other's") — *read*.
- **1.2** So the slug whose loop branch is `fix/rc24-tier1` is `rc24-tier1`, and its round branches would be `fix/rc24-tier1-r<N>`. — the same function — *read*.
- **1.3** The rulings named this run's branches: *"This run: loop branch `fix/rc24-tier1`, rounds `fix/rc24-tier1-r<N>` — both pass `dev/branch-name` today, so no rule or tooling changes for it."* — `DECISIONS.md` → *2026-10-05 — The stabilization workflow, as ruled*, ruling 7 — *read*.
- **1.4** `fix/rc24-tier1` exists locally and on the remote, both at `495991e3` (`git ls-remote --heads origin` and the local ref agree). — *exercised*.
- **1.5** No branch named `fix/rc24-tier1-r…` exists, locally or on the remote. — `git branch -a --list '*rc24*'` — *exercised*.
- **1.6** Five other branches carry the prefix `fix/rc24-`: `fix/rc24-canary`, `fix/rc24-core-review`, `fix/rc24-pre-opening`, `fix/rc24-regression`, `fix/rc24-route-catalog`; each remote tip is an ancestor of `495991e3`, none ahead of it. `fix/rc24-route-catalog` is checked out in a second worktree. — `git merge-base --is-ancestor`, `git rev-list --count` — *exercised*.
- **1.7** No directory `completions/artifacts/rc24-tier1` exists, and the record script answers `{"run": "rc24-tier1", "opened": false}` for that slug. — `ls`; `dev/stabilize-record state --run rc24-tier1` — *exercised*.
- **1.8** A run's slug follows the key grammar of the seed ledger: `a-z0-9` words joined by single `-`, at most 100 characters; anything else is refused (`bad-id`). The record script creates neither the run's directory nor `opening.md`; a run is opened when `completions/artifacts/<run>/opening.md` is a regular file. — `dev/stabilize-record --help`, *Identifiers* and the file list — *exercised* (the help was printed for this sheet).
- **1.9** `dev/branch-name` accepts `fix/<slug>` with `<slug>` = `[a-z0-9]+(-[a-z0-9]+)*`; it refuses any other prefix, `stabilize/` included. — `dev/branch-name`, lines 41–42 — *read*.
- **1.10** On the path rule's list of defects: *"a run slug ending `-r<N>` reads as a round's branch"*. — `implementation/stabilization-workflow.md` → *Open questions and declared bounds*, *The path rule* — *read*.
- **1.11** `origin/main` is at `34d72e13` (a merge of 2026-10-15); `HEAD..origin/main` is empty — `main` is merged into the loop branch — and the loop branch is 319 commits ahead of `origin/main`. — `git ls-remote origin refs/heads/main`, `git rev-list --count` — *exercised*.
- **1.12** The write guard binds where the session's project has a run's loop branch or round branch checked out and that run is opened; in the real run the guarded repository is the main working tree. `fix/rc24-tier1` is the branch the main checkout stands on now. — `implementation/stabilization-workflow.md` → *The write guard*; `git branch --show-current` — *read*; *exercised*.

**What does not exist yet**

- The run's directory under `completions/artifacts/` (1.7).
- Any round branch of the run (1.5).
- The branch type `stabilize/`: designed (`stabilize/<slug>/main`, `stabilize/<slug>/r<N>`), not applied; owed as *"one commit — the last on the first run's loop branch before its pull request"*. — `implementation/stabilization-workflow.md` → *Branches*; `implementation/decisions-pending.md` → *The `stabilize/` branch type* — *read*.

**Where sources disagree** — none found for this section.

**What the decision is between, as the doc and the code frame it**

- The opening writes a slug; the loop branch is not written separately — it is `fix/<slug>` by `branchName` (1.1). The record script takes no call for the slug: it is the run's directory, made before the first call, and the `--run` of every call (the workflow doc's data table, its first row).
- Values the code takes: any slug of the grammar (1.8, 1.9). Values the rulings name for this run: the loop branch `fix/rc24-tier1` (1.3), which `branchName` gives for the slug `rc24-tier1` (1.2).

**What exists beyond the listed options**

- The canary's run used the slug `canary-one`, in a clone, with a loop branch that existed there only. — `completions/artifacts/M55/stabilization-build/canary-one/run/run.md` — *read*.
- `dev/stabilize-canary setup` refuses a run or a loop branch this repository holds. — `implementation/stabilization-workflow.md` → *The small canary* — *read*.

---

## 2. The closing condition — by reference, clause by clause, each with its scope and its instrument

**Facts**

- **2.1** The entry that states it has the heading, whole: `2026-10-04 — The exit rule, revised: four clauses, each with a scope and one instrument — a finding blocks the call only if it breaks a clause inside its scope (the human's)`. Exactly one line of `DECISIONS.md` is `## ` followed by that text. — `DECISIONS.md` line 9611; `grep -c` of the whole heading line — *exercised*.
- **2.2** The rule, in the human's words as quoted there: *"We do not lose files or writhe / update incorrect things. We have a working product others can use and relay on. It is usable by the agents. The planned migration for this project to use jigc will work."* — the same entry — *read*.
- **2.3** The four clauses, as `decisions-pending.md` names them: *nothing is lost and nothing incorrect is written · a working product others can use and rely on · usable by agents · this repository's planned migration onto jigc will work*. — `implementation/decisions-pending.md` → *The exit rule, revised 2026-10-04 — the rule now in force* — *read*.
- **2.4** The scope of the first clause (a sharpening the human accepted): in a healthy repository used as documented — including line-ending conversion, `status.showUntrackedFiles`, a symlinked `CLAUDE.md`, linked worktrees — no jigc command at exit 0 destroys bytes no git object holds or commits content the user did not ask for; where jigc cannot tell it refuses before writing; races against a non-jigc writer inside a millisecond window, and deliberately planted states, are declared bounds. — the 2026-10-04 entry, sharpening 1 — *read*.
- **2.5** One instrument per clause (the second sharpening): *the re-review rows and the audit, graded against that scope · no command that works on rc.24 in a supported layout stops working, and every refusal's route works as printed · the blind trial arms · a rehearsal of this repository's migration on a copy of it with the candidate binary*. — the same entry — *read*.
- **2.6** *"A finding blocks the 1.0.0 call only if it breaks a clause inside its scope; everything else is recorded with its tier."* — the same entry — *read*.
- **2.7** No slug for any clause is ruled for the real run. The canary's synthetic opening used four: `no-lost-files`, `working-product`, `usable-by-agents`, `migration-works`. — `completions/artifacts/M55/stabilization-build/canary-one/run/run.md` — *read*.
- **2.8** The human has ruled on how a printed route is read, in five entries the route catalog follows: three kinds of planted state are declared bounds of the route catalog (2026-10-11, *Which states a route must work in*); a command a route cites for what it prints is held to its words, not to exit 0; a placeholder for a value of the reader's own is followed filled; a span that names a command without its operand is followed with the operand the route speaks of; the route is held to any file in the checkout and owes every value that is not the reader's own (all 2026-10-12). — `DECISIONS.md`, the headings at lines 6209, 3766, 3579, 3310, 3441 — *read*.
- **2.9** Rules of judgment that bind triage and the orchestrator in reading a clause: *neither the doc nor the code is the oracle — the behaviour that makes sense wins, and whichever is wrong changes*. — `implementation/working-principles.md`; `DECISIONS.md` → 2026-10-14 and 2026-10-19 — *read*.

**What does not exist yet**

- A ruled slug for each clause (2.7).
- The fourth clause's instrument (section 4, 4.20).

**Where sources disagree** — none found for this section.

**What the decision is between, as the doc and the code frame it**

- `run-set` `condition` — *optional, for good*: the heading of exactly one `DECISIONS.md` entry, written once; or none (`dev/stabilize-record --help`, `run-set`).
- `run-set` `clauses` — *the census*: a non-empty array of slugs, written once; it must name every clause an item of the test set already judges, or the call is refused whole (same).
- The opening record (prose) holds the condition by reference, clause by clause, each with its scope and its instrument (the workflow doc, *What it decides*).

**What exists beyond the listed options**

- `kit` prints the entry from the log by the `condition` heading at every call (`dev/stabilize-record --help`, `run-set: condition`) — *exercised* (help).
- The two earlier exit-rule entries, superseded in their criterion: *2026-09-21 — The exit rule, and the rc.17 fix pass (M53) chartered* and *2026-09-22 — The exit rule's outside-new-code case* (the latter stands) — *read*.

---

## 3. The candidate, the previous release, and the release being prepared

**Facts — the previous release**

- **3.1** The tag `jigc-v1.0.0-rc.24` is annotated and peels to `91834b5e011de2c36e2be2b79e96c0b9f60a803c`, a commit of 2026-10-03, *chore(release): prepare release jigc v1.0.0-rc.24*; it is an ancestor of `origin/main` and of the loop branch's tip, 327 commits behind the tip. — `git ls-remote --tags origin`, `git log`, `git merge-base --is-ancestor`, `git rev-list --count` — *exercised*.
- **3.2** At `91834b5e`, `crates/cli/Cargo.toml` says `version = "1.0.0-rc.24"` and pins `jigc-engine` `=0.1.0-rc.2`; `crates/engine/Cargo.toml` says `0.1.0-rc.2`. — `git show 91834b5e:<file>` — *exercised*.
- **3.3** The tag `jigc-engine-v0.1.0-rc.2` peels to `bd6e1a44` (the release of `1.0.0-rc.23`); `crates/engine` is byte-identical between `bd6e1a44` and `91834b5e`. — `git ls-remote --tags`, `git diff --quiet` — *exercised*.
- **3.4** The byte table's previous binary: `1.0.0-rc.24` rebuilt from `91834b5e` with `cargo build --release` in a target directory of its own, sha256 `accf3996…ab5d`, *"never an installed `jigc`"*. The same hash is stated by the route catalog's pilot record and the byte table's findings, and by the small canary's own build of `91834b5e` (`build-previous-91834b5e011d`). — `completions/artifacts/M55/stabilization-build/reshape/byte-table-record.md` (its paragraph on the committed tables); `route-catalog-pilot.md` line 347; `canary-one/README.md` line 202 — *recorded* (no binary was built or hashed for this sheet).

**Facts — the candidate**

- **3.5** A candidate is a commit on the loop branch, built locally from that commit; inside the loop there is no version, no tag and nothing that needs the human's merge; evidence is keyed by commit and binary hash. — ruling 2; the workflow doc → *Why this shape* — *read*.
- **3.6** The tip is `495991e3`. The product paths (`Cargo.toml`, `Cargo.lock`, `crates`) are identical between `e771c46d` (the merge of the byte table) and the tip. — `git diff --quiet` — *exercised*.
- **3.7** What the binary is built from — every product path but `crates/*/tests` — is identical between the tip and each of: `b36b8c93` (the regression set's run 3), `7e1b34a1` (the small canary's candidate tree) and `09270673` (the byte table's latest whole-table print). The tests differ (39 files between `b36b8c93` and the tip). The last commit to change product source, packs, guides or adapters is `0ad081cd` (2026-10-07). — `git diff --quiet` with `:!crates/cli/tests :!crates/engine/tests`; `git log -1 --` — *exercised*.
- **3.8** Since `91834b5e`: 189 commits touch product paths; 63 non-merge commits touch `crates/*/src`, packs, guides or adapters; 149 files of product paths changed. Since `0f34d8f0` (the fix pass's last commit) 11 non-merge commits touch product source — the eleven pre-opening fixes, `cea9755a` … `0ad081cd`. — `git rev-list --count`, `git diff --stat`, `git log` — *exercised*.
- **3.9** `crates/engine/src` changed in 9 files since `91834b5e`, and `crates/engine/Cargo.toml` is unchanged (still `0.1.0-rc.2`); the tip's `jigc` still says `1.0.0-rc.24`. — `git diff --stat`, `git show HEAD:<file>` — *exercised*.
- **3.10** Thirteen commits of the fix pass moved `pub` items of `jigc-engine`, a crate published at `0.1.0-rc.2`; the release PR's semver check has not seen them; owed: the version decision for `jigc-engine` at the next release PR. — `implementation/decisions-pending.md` → *The engine's public API moved in a published rc crate* — *recorded*.
- **3.11 CI on the loop branch.** 95 runs are listed for `fix/rc24-tier1`: one green (`5f5b273a`, 2026-10-04); the other 94, every run since (2026-10-05 onward), are red. The tip's run (`495991e3`) completed **red**: `test (g_migrate)` and `ci-ok` failed, every other job green. — `gh run list --branch fix/rc24-tier1 --limit 100`; `gh run view 40544364786` — *exercised*.
- **3.12** The failing test, in the tip's run and in eight earlier red runs read for this sheet (the two oldest red, of 2026-10-05, and six of 2026-10-19 to 2026-10-27): `setup_install_pathspec_guard::where_git_cannot_answer_setup_refuses_before_its_first_write`, `461 passed; 1 failed` in that leg (`467 passed` in the two oldest). The failing cell is `OwnershipRefused` (git's ownership refusal): `setup` refuses under `setup.dirty-install-path` where the test requires its own code (`setup.unverified-install-path` since `90fee322`). — `gh run view --log-failed`; `crates/cli/tests/setup_install_pathspec_guard.rs` lines 3225–3280 — *exercised*; *read*.
- **3.13** Of the 93 red runs before the tip's, 92 had `test (g_migrate)` failed (the job list of one, `37436058722`, answered HTTP 502); 51 of them also had `test (g_tooling)` failed or cancelled, one also `hygiene` failed, and one had seven other jobs cancelled. The failing test itself was read only in the nine runs of 3.12. — `gh run view --json jobs` over every red run — *exercised*.
- **3.14** The test was added by `f0b1120a` (2026-10-05, round 4's fix *where git cannot answer the install guard's question, nothing is installed*); `91834b5e` has no such test. — `git log -S`; `git grep` at `91834b5e` — *exercised*.
- **3.15** `DECISIONS.md` recorded the red on 2026-10-05: *"Not diagnosed here. It enters the run's ledger at its opening, as a regression of the pass."* — `DECISIONS.md` line 9470 — *read*.
- **3.16** The last local full gate on record for the merged tree: `passed=5349 failed=0` at `e771c46d`. — `implementation/decisions-pending.md` → `wf-route-catalog-merge` (struck, discharged 2026-10-27) — *recorded*.
- **3.17** Adjustment (i): the candidate's `ci-ok` is read for its commit on every candidate; *"where it cannot be read, the gate runs under `dev/runner-faithful`"*; without it *"the loop closes on macOS and the pull request's required check is red"*. — the rulings entry, adjustment (i) — *read*.

**Facts — the release being prepared**

- **3.18** Ruling 1: *"its outcome is a version ready to release — `1.0.0` in this run, a `1.1.0` or a `1.1.0-rc.1` in a later one."* — `DECISIONS.md` → *2026-10-05 — The stabilization workflow, as ruled* — *read*.
- **3.19** *"`jigc` is on the `1.0.0-rc.N` track until the 1.0.0 call … Leaving the track — `release-plz set-version` by hand to `1.0.0` — **is** the human's 1.0.0 call, and nothing automated performs it."* `jigc-engine` is versioned on its own, 0.x, bumped only by changes to its own paths. — `implementation/release.md` → *Versioning* — *read*.
- **3.20** *"The 1.0.0 call is the human's, and it has not been taken."* — `CLAUDE.md` → *Project state* — *read*.
- **3.21** At the close the human sets the version where the derived one differs from the planned one, and merges the release PR; the release is of two crates, each versioned on its own. — the workflow doc → *Close*, steps 5–6 — *read*.

**What does not exist yet**

- A built binary of the tip, and its hash (none was built for this sheet; the byte table's print names *this tree's debug build* without a hash in the lines read).
- A green CI run on the loop branch since `5f5b273a` (3.11).
- A diagnosis of the Linux-only red (3.15: *"Not diagnosed"*).
- A script for the release PR head's file-set comparison, and the composition of an upgrade arm from the previous release to the new version. — the workflow doc → *Close*, step 5; *Not built* — *read*.

**Where sources disagree**

- **What version this run prepares.** Ruling 1 (2026-10-05): *"`1.0.0` in this run"* (3.18). `CLAUDE.md` → *Project state* (current at this commit): *"The 1.0.0 call is the human's, and it has not been taken"*, and its order, bracketed as superseded on 2026-10-05, ran *"the next release candidate → the exit rule's instruments on it → the call"*; `release.md` makes leaving the rc track the human's call alone (3.19); the fix pass's record, under a 2026-10-05/06 bracket, still lists *"the next release candidate"* as its step 4. The ruling is the newest of the four statements that name a version for the run's outcome; `CLAUDE.md`'s sentence on the call is current text at this commit.

**What the decision is between, as the doc and the code frame it**

- `run-set` `previous` — a version (`1.2.3`, `1.2.3-rc.4`) — and `previous-commit` — a full sha; each written once. Values the facts above give: `1.0.0-rc.24` and `91834b5e011de2c36e2be2b79e96c0b9f60a803c` (3.1).
- The candidate is no `run-set` fact: a round's `round-set` writes it (`candidate`, a full sha, written once; *"A round whose record names its candidate is TESTED"*). The opening record names it in prose (`dev/stabilize-record --help`, `round-set`).
- The release being prepared is prose of the opening record only; no call takes it.

**What exists beyond the listed options**

- `dev/runner-faithful [--commit <rev>] cargo <args…>` runs one cargo command on a clone of a committed revision in a runner-shaped Linux image (`CLAUDE.md` → *Build / lint / test*) — *read*. Docker answers on this host (`command -v docker`) — *exercised*.
- `completions/trial-harness/build-image.sh <sha>` pins a binary by commit into a trial image; `--registry <version>` installs a published one — `CLAUDE.md`, trial tooling paragraph — *read*.

---

## 4. The test set, per instrument — fixed before round 1

**Facts — what an item is, and who runs it**

- **4.1** An item row is: `item` (slug) · `kind` · `clause` (a clause of the census) · `runs` (`every-candidate` | `in-scope`) · `brief` (text, not blank) · `doors` · `registries`; an in-scope item names at least one door or registry. From the first round that ran it or owes it a run, its clause, runs, doors and registries stand. — `dev/stabilize-record --help`, `item-set` — *exercised* (help).
- **4.2** Kinds the harness runs: the keys of `CHAINS` — `review-row` (source pass, driver, optional cross-model, then a reconciler), `trial-arm` (rehearse, run, score, on the trial image), `audit-area`, `audit-cross-cutting`, `audit-drive`; `check` (a deterministic check the preflight runs from the item's brief); `held-<id>`, where the step tool's held kinds are `gate` and `regression`. Any other kind halts the stage that would run it. — `.claude/workflow-sources/stabilize.js`, `CHAINS`, `CHECK_KIND`; `dev/stabilize-step` header, `hold-start` — *read*.
- **4.3** Ruling 10: round 1 tests what the candidate changed against the previous release; model-driven instruments run only over what changed; the deterministic checks run in full on every candidate — the full `dev/gate`, CI's `ci-ok` for the candidate's commit, the packaged-tarball install, and the scripted regression set. — the rulings entry, ruling 10 — *read*.
- **4.4** Round 1's hunts, ruled 2026-10-06 (question 21, option A): *the review rows and one cross-cutting reviewer — no per-area audit*. The seven rows (question 22, option A): 1 install members · 2 teardown · 3 worktree registrations · 4 home occupancy · 5 home shape · 6 reconciliation · 7 the linked-worktree guard, each deriving its doors from the registries the proposal names. — `DECISIONS.md` → *2026-10-06 — Ahead of the run's opening*, 1 and 2; `completions/artifacts/M55/stabilization-build/pre-opening/review-rows-proposal.md` — *read*.
- **4.5** A stated limit accepted with the table: *"no row reads the guides and the help text as a whole"* — to go into the opening record as a limit of round 1's test set. — the same entry, 2 — *read*.
- **4.6** Cross-model passes in round 1 (question 23, option B): rows 6 and 3, named per item in the `test` invocation's `crossModel`. — the same entry, 3 — *read*.
- **4.7** The seven rows' commit lists cover `13dc5bc7..0f34d8f0` (the proposal: *"The seven rows' commit lists are exactly those 52"*). The eleven pre-opening fixes (`cea9755a` … `0ad081cd`) came after `0f34d8f0` and appear in no row's commit list. — `review-rows-proposal.md`; the entry's item 1; `git log 0f34d8f0..HEAD` — *read*; *exercised*.
- **4.8** The proposal's own *Not determined* list: grep counts include definitions and in-file tests; *"whether a candidate binary exists"*; the container's git version. — `review-rows-proposal.md` → *Not determined* — *read*.

**Facts — clause 1's instrument (the review rows and the audit)**

- **4.9** The rc.24 review's instrument — ten axis prompts and ten driver scopes — is committed at `completions/artifacts/M55/per-axis-review-rc24/instrument/` (21 files). The seven rows descend from its axes 1, 2, 3, 5 and 6 (row 3 and row 7 from none). — `git ls-files`; `review-rows-proposal.md` — *exercised*; *read*.
- **4.10** The byte table — 30 rows, 298 lines, 782 cells, 602 of them in the gate in four arms, 180 on demand — is the suite `crates/cli/tests/byte_table.rs` (registered in `g_migration`), with three ignored tests run by hand; its red cells are listed under keys in `EXPECTED_RED`. — `byte-table-record.md` §1 (*recorded*); `crates/cli/tests/groups/g_migration.rs` (*read*).

**Facts — clause 2's instrument (no command that works on rc.24 stops working; every refusal's route works as printed)**

- **4.11** The regression set's first part is built: `dev/regression-set run --previous … --candidate … --list … --scratch …`, run by a stage as an item of kind `held-regression`; *"No run has an item for it yet: that row is written at a run's opening."* — the workflow doc → *The regression set* — *read*.
- **4.12** The list the tool reads is `completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv` (`REGRESSION_LIST`); sha256 `bfdad1af…` at the tip and at `b36b8c93`; 33 rows — 16 `commit:`, 6 `ruling:`, 11 `excluded:`. — `shasum -a 256` of the tree's file and of `git show b36b8c93:<file>`; an `awk` count — *exercised*.
- **4.13** Run 3 is green for the candidate `b36b8c93` (22 differences, all listed; 11 tests failing on their own binary, each by its row); *"a commit that changes a product path, or the list, is another candidate and wants a run of its own"*. Three recorded runs took 21 to 34 minutes. — `regression-set/README.md`, the first table; the workflow doc — *recorded*.
- **4.14** What a green of part 1 cannot see: tests that never reach the binary (about half the old suite); states no old test built; one platform, one profile. — the workflow doc → *The regression set* — *read*.
- **4.15** The route catalog — `crates/cli/tests/route_catalog.rs` (registered in `g_migration`), cases per refusal site and cause, its `EXPECTED_RED` naming 43 distinct ledger keys across 49 entries. The pilot covers `setup` and `uninstall` (54 sites, 353 cases, recorded); the other families were taken in for the sites the fix pass touched. — `crates/cli/tests/route_catalog.rs` lines 402–868, counted here; `decisions-pending.md` → `wf-route-catalog` (struck) — *read, counted here*; *recorded*.
- **4.16** *"After the opening every edit of the route catalog moves through a round, because it is a product test."* — `DECISIONS.md` → *2026-10-11 — The reshaped `test` stage's three open questions, ruled* — *read*.
- **4.17** `dev/gate` runs fmt, clippy, build and the tests (nextest plus doctests); a held gate is `dev/gate --keep-going` (`hold-start --kind gate`); the byte table's and the route catalog's gate arms run in it. — `CLAUDE.md`; `dev/stabilize-step` header — *read*.

**Facts — clause 3's instrument (the blind trial arms)**

- **4.18** The trial arms ruled for round 1: the fan-out arm and the amend arm as on rc.24, the inconsistency arm in two turns scored separately, no fourth blind arm; the fan-out arm scores *told to file after landing* as the expected outcome. The rehearsal comes before any blind session (ruling 7 of that entry). What passes: the fan-out and amend arms each reach their best outcome class, and in the asked turn the agent files through the findings channel; an arm whose occasion did not fire is void, not failed. — `decisions-pending.md` → *A rehearsed, unambiguous occasion for the blind trial's inconsistency arm* and *What the run's opening record carries* — *read*.
- **4.19** Tools that exist: `completions/trial-harness/` (`build-image.sh`, `verify-image.sh`, `verify-pair.sh`, `run-session.sh`, a `Dockerfile`); `completions/trial-driver/` (`run.py observe`/`seed`/`fork`/`plant`/`gate`, `walk.py`, walk arms `00`–`23`); `completions/trial-corpus-template/`; the rc.24 trial's protocol at `completions/artifacts/RC-rc24/protocol.md`. — `git ls-files`; `CLAUDE.md` — *exercised*; *read*.

**Facts — clause 4's instrument (the port rehearsal)**

- **4.20** The rehearsal's shape is ruled (question 27, option B, 2026-10-06): a plain `git clone` of this repository with its remote removed, an isolated `$HOME`, the candidate's binary; a deterministic walk (`jigc setup`, ingest of the seed docs, one doc task with fixed text, a simulated fan-out with each `Spawn:` line run verbatim); and one real migration by an agent — `implementation/decisions-pending.md` as the deferral ledger; a finding is either *the product fails its documented path* (can block) or *this project would have to change* (never does). **Not built.** Trigger: *before the first stabilization run can close*; its build *"while round 1's `test` runs"*. — `decisions-pending.md` → *The port rehearsal — a scripted walk on a clone, and one real migration by an agent*; `DECISIONS.md` → *2026-10-07 — The port rehearsal is a scripted walk …* — *read*.

**Facts — what the doc says a run does about an instrument that does not exist**

- **4.21** *"A clause that no item of the test set judges stays void and answers `unsettled` once nothing else is open … The two ways out: the instrument is designed with the human, built, and takes its row in the test set — `item-set`, after which the state asks for its run, as an attempt of the latest tested round; or the closing condition changes, which is a new opening."* — the workflow doc → *Close*, *What cannot close* — *read*.
- **4.22** Ruling 16: round 1's `test` starts with what exists; the missing instruments are designed with the human, one at a time, built by subagents, and run on the next candidate over the whole delta since the previous release — *"the one stated exception to ruling 10"*. How that delta is given to an item, which today is handed the round's doors, is open. — the rulings entry, ruling 16; the workflow doc → *Not built* — *read*.

**What does not exist yet**

- A committed brief for any of the seven review rows (the item's `brief` cell may be text — 4.1). No file under `completions/` or `.claude/` other than the proposal and the rulings names the rows' briefs. — `git grep -l "install members"`; `git ls-files` — *exercised*.
- A route hunter: ruled 2026-10-10 (ruling 2), owed *before the run's opening* (`wf-route-walk`, unstruck); no role, chain or site-listing script for it is in the harness. — `decisions-pending.md` → `wf-route-walk`; `grep` of the harness — *read*.
- The regression set's second part (the nine layouts as fixtures; none exists). Trigger: *before the first stabilization run can close*; `wf-pre-regression-part-two`'s cut: *before the port (M56's planning), after the byte table's pilot*. — `decisions-pending.md` — *read*.
- The port rehearsal (4.20).
- The inconsistency arm's occasion, its rehearsal and its class table split by turn: *"Ruled 2026-10-06, and not built"*. — `decisions-pending.md` — *read*.
- The trial image's build as a held command (`wf-image-hold`, trigger *between rounds 1 and 2 — and before the run's opening if its session asks for permissions*). — `decisions-pending.md` — *read*.
- `wf-byte-table-rest` (the byte table's six unscheduled write leaves, twelve needs, the plan's unbuilt items, two plant kinds, the regression set part 2's cut): trigger *M56's planning, before the port*. — `decisions-pending.md` → `wf-byte-table-rest`; `byte-table-record.md` §5 — *read*.
- `wf-route-catalog-rest` (every blocking site the fix pass did not touch): trigger *before the port (M56) begins*. — `decisions-pending.md` — *read*.
- A CI job of the byte table's own (*"owed at the join with the tooling lane"*). — `wf-byte-table-rest` — *read*.

**Where sources disagree**

- **Whether the reshaped `test` stage is built.** The workflow doc, *Open, the human's to rule*: *"What becomes of the verify fan-out … **Ruled 2026-10-10 and not built yet:** the `test` stage this doc describes is the one that stands in the code"* (line 706 of the doc). The same doc elsewhere describes *a run with a queue* (lines 97, 111, 301), the harness's source names that path 46 times (*"A RUN WITHOUT AN ORDER TAKES THE PATH ABOVE, UNCHANGED"*), and `DECISIONS.md` records its build on 2026-10-12 (the verify phase, the record before verification, the new path's triage). `decisions-pending.md` keeps `wf-verify-fan-out` and `wf-route-walk` unstruck with the trigger *before the run's opening*. All are at this commit; the `DECISIONS.md` entries of 2026-10-12 are newer than the 2026-10-10 ruling the doc's sentence names. *Settled against the code in 0.2: the doc's line 706 is the stale one, but for the route hunter and `VERIFY_PASSES`.*
- **What a verifier says of a door the previous release did not have.** `.claude/agents/finding-verifier.md`, step 4: *"A block that cannot run on the previous release at all, because the door did not exist, is `regression: false`, with *not comparable* said in the basis."* `dev/stabilize-record`, header line 377: *"ABSENT, OR null, WHERE THE VERIFIER DOES NOT KNOW: … the door is one that release does not have … an unknown fact is never written as `no`."* Both at this commit; the row `wf-regression-not-comparable` owes *one of the two, in the definition and in the header together*, trigger *`T10`, and before the run's opening*.

**What the decision is between, as the doc and the code frame it**

- One `item-set` row per instrument item (4.1), each of a kind in 4.2, its clause one of the census (section 2). The kinds the rulings and the canary point at, item by item: seven `review-row` items and one `audit-cross-cutting` (4.4); `trial-arm` items (4.18); `held-gate` and `held-regression` (4.11, 4.17); `check` items by brief — CI's `ci-ok`, the packaged-tarball install (4.3), whose verdicts are the preflight's reading (declared bound 3 of the doc's list).
- Per item: `runs` `every-candidate` or `in-scope`; `crossModel` is no item fact — it is named per invocation (4.6).

**What exists beyond the listed options**

- The canary's synthetic test set, one item of each kind round 1 has (`row-doc-list` review-row, `cross-cutting`, `arm-control` trial-arm on walk arm 00, `gate` held-gate, `regression-set` held-regression). — `canary-one/run/test-set.md` — *read*.
- `dev/stabilize-canary setup --parts` writes an opening for the staged canary with four items, a budget, a hand-on number and an order. — `DECISIONS.md` → *2026-10-12 — The canary's setup opens a run with a queue* — *read*.
- `completions/workflow-eval/`, the older per-workflow agent-run eval. — `CLAUDE.md` — *read*.
- The `audit-area` and `audit-drive` chains exist and were not ruled into round 1 (4.4). — `CHAINS` — *read*.

---

## 5. The declared bounds — each the human's ruling, with its reach, pinned or *unpinned*

**Facts**

- **5.1** The workflow doc lists *"The declared bounds of the first run — what the human is asked to accept at its opening"*: 23 numbered bounds, each with its reach and its pin or *unpinned*; *"Two are not on the safe side — 1 and 3 can put a wrong word on record."* Then a second list, *"true as built, and to be known by whoever runs one"* (20 bullets). — `implementation/stabilization-workflow.md` → *Open questions and declared bounds* — *read*.
- **5.2** Bound 1 of that list: *"A mistyped door answers `close`, with both names shown"* (not on the safe side); bound 3: *"Some verdicts are still an agent's reading: CI's `ci-ok` for the candidate's commit, the packaged-tarball install, that the trial image is verified, the preflight's environment asserts, whether the cross-model tool answers — and any `check` item that is a brief"* (unpinned, not on the safe side). — same — *read*.
- **5.3** The human's product bounds of 2026-10-06 (5c): (1) content staged for a doc a task is updating, while the working file does not carry it, is replaced at finalize; (2) an uncommitted edit before a task's first write is carried and replaced by that task's write to the same part; (3) while the invocation log is on, any jigc run brings the log back; (4) a repository that ignores jigc's own committed files is unsupported. Each was written into the design doc that owns its door (`design/finalize.md`, `design/reconciliation.md`, `design/measurement.md`, `design/validation.md`). — `DECISIONS.md` → *2026-10-06 — Ahead of the run's opening*, 5c; `decisions-pending.md` → *The declared bounds, as one list with reach* — *read*.
- **5.4** Bound (2) was **retired** by the human on 2026-10-19: a `doc` write that would replace a part holding an uncommitted hand edit refuses at the write; *"the product change and the sentences of `design/write-commands.md` and `design/reconciliation.md` … are a finding of the run's ledger; the list owes three bounds."* — `decisions-pending.md`, the same row's last bracket; `DECISIONS.md` line 898 — *read*.
- **5.5** The byte table's suite takes bounds 1, 3 and 4 (`byte_table::DECLARED_BOUNDS`); one cell is marked *bound 1* (`task-finalize-a-change-staged-at-the-updated-doc-and-not-in-the-file`). — `byte-table-record.md` §1 and §3.3; `DECISIONS.md` line 914 — *read*.
- **5.6** The route catalog's bound: a state only `JIGC_ADAPTERS_DIR` reaches is a declared bound of the catalog (a red case standing on one stays red and does not block); `$HOME` taken away is no bound. — `route-catalog-findings.md`, its header; `DECISIONS.md` → 2026-10-11, *Which states a route must work in* — *read*.
- **5.7** Other bounds the exit rule's row names as scattered: the external-writer race class and its measured windows; the layouts whose home is no work tree; a working area minted before the copied-in witness; the bounds each design doc states under *Declared bounds*. — `decisions-pending.md` → *The declared bounds, as one list with reach* — *read*.
- **5.8** `wf-never-selected` — whether an in-scope item no tested round selected should forbid `close` — trigger *"the run's opening, with the declared bounds; the form, if they ask for it, between rounds 1 and 2"*. — `decisions-pending.md` — *read*.
- **5.9** A row of `bounds.md` is taken only as the record of a human's ruling: it needs its reach, where the human ruled it (`--ruling`), and the pin or `unpinned` (`--pin`); none blank. A grade of *out of scope* must cite such a row, or the finding goes to the human (ruling 5). — `dev/stabilize-record --help`, `bound-set`; the rulings entry, ruling 5 — *exercised* (help); *read*.

**What does not exist yet**

- *"The one list at the run's opening is still owed."* — `decisions-pending.md` → *The declared bounds, as one list with reach* — *read*.

**Where sources disagree** — none found: the four-bound note (2026-10-06) and its retirement of bound (2) (2026-10-19) stand in the same row, the later bracket saying the list owes three.

**What the decision is between, as the doc and the code frame it**

- One `bound-set` row per bound the human accepts (5.9); the doc's list of 23 (5.1) is framed as *"what is put to them"*, each to accept or not; plus the human's product bounds that stand (5.3–5.4).

**What exists beyond the listed options**

- The fix pass's open lists name further bound-shaped items no ruling has made a bound — e.g. `dc0d7586`-5 = PI-1 and `ebfc79fb`-12, *"two declared bounds"* stated by fixers, and `4F-2`'s worktree-registration bounds. — `completions/artifacts/M55/fix-pass-rc25/findings-ledger.md` → *Open at the close*, (e) — *read*.

---

## 6. The stop mode, the bound across rounds, the default scope — and the bound on fix cycles

**Facts**

- **6.1** Ruling 6: inside a round at most three fix → audit cycles; *"Across rounds: the first run stops after each; later runs default to three rounds, written in the opening record. Only the human raises either, per run."* — the rulings entry — *read*.
- **6.2** 2026-10-07, the human's ruling (B): *"round 1 of the real run is the tuning round"*; everything else from the reviews is a recorded row, fixed between rounds. — `DECISIONS.md` → *2026-10-07 — The second repair of the `test` half, reviewed and ruled on scale* — *read*.
- **6.3** At the first stop of the first run the human looks again at the performance levers left open (2 and 3, 5, 10 and 11), on the run's own numbers; until then *the full gate precedes every commit*. Further rows fall due at the first stop: `wf-pre-driver-kit`, `wf-pre-evidence-by-reference`, `wf-pre-stage-instructions`, `wf-pre-finding-excerpt`, `wf-triage-per-wave`, `wf-recurrence-count`, `wf-hold-no-stop`, `wf-kit-no-filing`, `wf-t11`, `wf-gate-cost`, `wf-test-environment`, and *the stabilization tool's questions to the human are thinner than the orchestrator's* (beside T-a and T-b). — `decisions-pending.md`, their triggers — *read*.
- **6.4** The harness's `DEFAULT_CYCLES` is 3, `MAX_CYCLES` 99, `VERIFY_PASSES` 3. — `.claude/workflow-sources/stabilize.js` lines 764–776 — *read*.
- **6.5** **Fitness of the stages.** `NOT_FIT` holds `fix`: an invocation of `stage: 'fix'` returns `refused` before any agent runs, but for rulings about the run. *"Nothing refuses the `test` stage on that ground: that it is not used on a real run before its record says so is the orchestrator's rule to keep."* The `test` half was re-reviewed on 2026-10-07: *"fit for a canary run, not yet for the real run"*; *"The rule binds as before: `test` is used on a real run once this second repair and the review of it are recorded in DECISIONS.md — that record is the orchestrator's to write"*. And: *"the `test` stage is sound for round 1 and for what follows it without a fix, and no further"*. — the harness, `NOT_FIT`; the workflow doc → *What is built, and what is not* and *Where the two halves stand* — *read*.
- **6.6** No `DECISIONS.md` heading carries the words *"the `test` half of the stabilization workflow, repaired and re-reviewed"* or *"fit for the real run"* beyond the 2026-10-07 re-review's *"not yet"*. — `grep` of the headings — *exercised*.
- **6.7** Whole stages run under the runtime: the small canary's `test` stage (2026-10-08, headless, 100 agents, 8.5 hours, triage unfinished); the staged canary's parts in throwaway clones, driven by the human in a session of their own (*"Part 4 of the fifth root ran as written"*). No stage of a real run. `wf-whole-canary-after-parts` — *how much of the whole canary is still run* — is due *at the reading of part 4* and is unstruck. — the workflow doc → *The small canary*; `DECISIONS.md` lines 1683 and 4670; `decisions-pending.md` — *read*; *recorded*.
- **6.8** Default scope values: `delta` (round 1 tests what the candidate changed against the previous release) or `everything`; an invocation may also name `{ range: '<sha>..<sha>' }` or `{ doors: [ … ] }`; a round's scope is written once. — `dev/stabilize-record --help`, `run-set: scope`; the harness's *Usage*, `scope` — *read*.
- **6.9** The dissent on record: a cross-model reviewer held that scoping by door is not safe without one full run of every model-driven instrument before closing; the human ruled that out; the close shows each clause's evidence age in fix rounds. — the workflow doc → *Why this shape* — *read*.

**What does not exist yet**

- The record in `DECISIONS.md` of the `test` half's second repair and its review that 6.5 names as the condition for a real run (6.6).
- A definition of the tuning notes T-a and T-b in the repository: `decisions-pending.md` names them once (*"beside the tuning notes T-a and T-b"*) and nowhere says what they are. — `git grep` — *exercised*. The orchestrating session's own handover notes, outside the repository, describe them: *T-a* `dev/stabilize-step hold-start --wait` — one agent starts or finds a held job and waits in 90-second slices, its digest carrying `fresh`; *T-b* `push --read-state`, about two to three launches a stage — *recorded* (not in the repository).
- A clause's evidence age in commits (the dissent was promised it; the state gives fix rounds). — the workflow doc → *Not built* — *read*.

**Where sources disagree** — none found for this section.

**What the decision is between, as the doc and the code frame it**

- `run-set` `stop`: `every-round` | `at-the-bound` — written once.
- `run-set` `rounds`: a positive integer (required where the mode is `at-the-bound`: `no-round-bound`), raised only at its own stop.
- `run-set` `cycles`: three where unwritten; written at the opening, or raised above the bound that holds at its own stop.
- `run-set` `scope`: `delta` | `everything` — written once.
- The opening is ready when `state` prints `"not_ready": []` and `"next": "test"`; `not_ready` can hold `no-clause-row`, `no-stop-mode`, `no-round-bound`, `no-previous-release`, `no-default-scope`. — `dev/stabilize-record --help`, `run-set` and `state` — *exercised* (help).

**What exists beyond the listed options**

- `stopAfter` — return after a named step, for tuning; no record step runs (the harness's *Usage*) — *read*.
- The staged canary's precedent: the tool run *in parts*, each in a throwaway clone driven by the human in a session of their own — `DECISIONS.md` → *2026-10-12 — The reshaped `test` stage is run for real in parts* — *read*.

---

## 7. The ledger's opening rows — what is already known against the candidate

**Facts — the byte table**

- **7.1** `byte-table-record.md` §2 lists **69 live keys** — counted here: 69 lines of the form `` - `key` ``, no key twice; **46** with a red count, **23** marked *no cell*. Four keys are retired and not listed. The record says 69, 46 and 23. — `byte-table-record.md` lines 54–200 — *read, counted here*.
- **7.2** Regressions — *a harm class on this tree that the previous release did not show for the same cell*: six keys marked **yes, 1** and one marked **traded, 2** (`r1-config-set-refuses-and-leaves-the-directories-its-re-point-made`) — **8 cells under 7 keys**, as the record says. — same — *read, counted here*.
- **7.3** Two of those regression cells name `5f5b273a` as their cause: `config-set-placement-root-a-link-to-nothing-at-the-roadmaps-new-home` under `r1-config-set-parks-an-untracked-occupant-of-a-new-home` (*"`5f5b273a` (the findings' §25) — the one cell red here and clean there"*), and `relocate-a-link-to-nothing-at-the-notes-new-home` under `r1-relocate-parks-an-untracked-occupant-of-a-new-home` (*"the parking of an occupant at a new home, `5f5b273a`"*). — `byte-table-record.md` §2 — *read*.
- **7.4** §27, key `r1-milestone-execute-prints-a-spawn-line-the-adapters-launch-template-never-renders` (one of the 69, *no cell*): the human's rule of 2026-10-26 — *"jigc never launches a sub-agent itself. It prints the launch, rendered through the adapter's template, as an instruction the coding agent carries out with its own primitive"*; the product's `Spawn:` line is bare and the template is checked at install and never rendered; wrong: the code, and the doc owes the rule as a sentence; open inside it: which profile renders where no adapter is installed or a second one is. — `byte-table-findings.md` §27 — *read*.
- **7.5** §29, key `r1-a-write-refuses-an-unreadable-file-state-record-with-a-route-that-deletes-the-baseline` (one of the 69, *no cell*): the refusal over an unreadable `.jigc/state/file-state.json` routes `rm <the record, absolute>` and says *"the record is a rebuildable cache and holds nothing of yours"*; by the human's ruling of 2026-10-26 the record is the baseline; wrong: both the product's words and `design/reconciliation.md`; *"Robust: for the run to decide, not this lane"*; not a regression (rc.24 printed no route). — `byte-table-findings.md` §29 — *read*.
- **7.6** The keys are not held by any ledger yet: *"No ledger holds these keys"* is said of the route catalog's; the byte table's §2 is *"the keys, as the run's ledger will take them"*. — `route-catalog-findings.md`; `byte-table-record.md` §2 — *read*.

**Facts — the route catalog and the small canary**

- **7.7** The route catalog's `EXPECTED_RED` names **43** distinct keys; none is among the byte table's 69; **7** are keys of the small canary's findings. — the suite's list, resolved and intersected here — *read, counted here*.
- **7.8** The small canary's findings file holds **72** keyed rows in its tables — 2 confirmed regressions (both at `jigc setup`, a pre-commit hook linked outside the repository), 10 confirmed, 4 *breaks* verified by nobody, 6 confirmed in attempt 2 and on no record, 3 whose verifier halted (Linux host or root caller), 47 *needs-bound* — and its data file **372** entries: 2 regression · 10 confirmed · 4 breaks · 47 needs-bound · 114 unclear · 48 refuted · 147 no-break. — `canary-one/findings.md`; `canary-one/data/entries.tsv` — *read, counted here*.
- **7.9** The canary's candidate was the product tree of `7e1b34a1` (binary `dded1fac…`), whose non-test product paths are identical to the tip's (3.7). — `canary-one/README.md` line 143; `git diff --quiet` — *recorded*; *exercised*.
- **7.10** `wf-canary-seed`: *"the confirmed as findings to verify on the run's own candidate, the rest as leads; how many of the 161 leads are seeded is `wf-verify-fan-out`'s to say"*; trigger *the run's opening*. — `decisions-pending.md` — *read*.

**Facts — the fix pass's ledger**

- **7.11** `completions/artifacts/M55/fix-pass-rc25/findings-ledger.md` holds **487** rows over six tables, counted here and matching its own counts: A 88 · B 77 · C 54 · D 132 · E 66 · F 70. Its closing section *Open at the close of the session*: (a) 20 decisions — ruled by the human on 2026-10-06; (b) 3 new findings — ruled 2026-10-06 (item 21 fixed before the opening; items 22 and 23 declared bounds); (c) 6 audit findings no round took and one sub-item (XC-6, CPL-9, CPL-10, E2E F7, E2E F8, E2E F9 — six riders — and XC-1's comment); (d) by reference, the review's 58 open rows and the trial's 7 tiered, 14 untiered, 11 `O-`, 32 tooling and 6 drift rows — *"filed through jigc at the port (M56)"* and *"have not been graded against the revised exit rule"*; (e) 69 leads, bounds, residue, wording and coverage entries; (f) what nobody examined, by reference. — the ledger's tables and *Counts* — *read, counted here* (the tables); *read* (the groups).
- **7.12** *"Every `FIXED` of round 4 and the last fix is unaudited"* (36 commits). — the ledger; the pass's `README.md` → *The state at the close* — *read*.

**Facts — the pre-opening fixes and other owed rows**

- **7.13** `pre-opening/pre-opening-fixes-log.md` lists **22** numbered *left open* lines, **2** of them marked *"This line becomes no ledger row"* — **20** standing, each to be *a ledger row at the opening*. — counted here — *read, counted here*.
- **7.14** The CI red of 3.11–3.15: one test, Linux only, red on every run since 2026-10-05; `DECISIONS.md` says it *"enters the run's ledger at its opening, as a regression of the pass"*. No ledger key for it exists in any record read. — *exercised*; *read*.
- **7.15** The timing flake: `dev_stabilize_probe::a_hold_ends_only_after_its_seconds_is_started_once_and_is_answered_within_a_slice` measured 1.95 s against its 2.0 s floor in a loaded gate on 2026-10-18 and passed on re-run; *"nobody has looked"*; trigger *the run's opening (an opening row of its ledger)*. — `decisions-pending.md` → *A hold of `dev/stabilize-probe` ended 50 ms early under load* — *recorded*.
- **7.16** Large lists: *"A long list of findings has no shape a model can work with"* — the human's ruling of 2026-10-20: jigc's own output first, a product design item settled through the design workflow; the repository's tooling later, at the first stop; trigger *the run's opening (an opening row of its ledger)*; the design is settled *before the port*. — `decisions-pending.md` — *read*.
- **7.17** Thin questions: *"The stabilization tool's questions to the human are thinner than the orchestrator's"* — owed in the `fix` half's repair; trigger *the run's first stop (beside the tuning notes T-a and T-b)*. — `decisions-pending.md` — *read*.
- **7.18** Retired bound (2)'s product change (5.4) is *"a finding of the run's ledger"*. — `decisions-pending.md` — *read*.
- **7.19** Ledger keys follow the seed ledger's grammar so that an open row can later be filed as one jigc doc (`jigc-feedback` or `inconsistency`). — the workflow doc → *The run's record*; ruling 13 — *read*.

**What does not exist yet**

- The run's ledger (`ledger.md`) and any key for the CI red (7.14) or the timing flake (7.15).
- A ledger row for the route catalog's 43 keys (7.6).
- A decision on how many of the canary's leads are seeded (7.10).

**Where sources disagree**

- **How many of the pre-opening fixers' left-open lines stand.** `decisions-pending.md` → *What the fixers ahead of the opening left open* (added at `3ab2c895`, 2026-10-07 02:22): *"five lines from the setup and uninstall fixes, two from the repair of the tooling suites' Linux failures, seven from the record of the settings entries, four from the last two fixes. Two of the eighteen are closed since … The other sixteen are not restated here."* `pre-opening-fixes-log.md` as committed by `0ad081cd` (2026-10-07 08:12, newer) holds a further block, *"Left open by this fix — ledger rows at the opening"*, of four lines after item 9's build: 22 lines listed, 2 struck, 20 standing (7.13).
- **Whether the byte table's merge is owed.** `byte-table-record.md` §11 (written on `fix/rc24-route-catalog` before the merge): *"This branch, `fix/rc24-route-catalog`, is to be merged into the loop branch … before the stabilization run opens there."* `decisions-pending.md` → `wf-route-catalog-merge` (struck at `e5e323fc`, newer): *"DISCHARGED 2026-10-27 — merged as `e771c46d`"*; the branch's tip is an ancestor of the loop branch (1.6). §11 also asked for *"CI read for its tip"*: the merge's CI run is red (3.11).

**What the decision is between, as the doc and the code frame it**

- `ledger-add` rows, round 0: `key`, `doctype` (`jigc-feedback` | `inconsistency`), `round`, `source`, `door`, `clause`, `repro` required; `grade`, `graded_by`, `bound`, `disposition`, `detail` optional; a class (`kind` `route` | `behaviour` | `lead`, with `route` or `commands`; `origin` `hunt` | `seed`, or a `parent`; `code`, `site`) optional — **required on every row in a run that holds an `order`** (section 8). A key the ledger holds is not appended again. — `dev/stabilize-record --help`, `ledger-add` — *exercised* (help).
- *"A list a record already keeps is pointed at by its path and its row count"* (the definition of this sheet); the doc: the first triage that runs is handed the opening rows.

**What exists beyond the listed options**

- `wf-return-size`: *"before the run's opening if the opening seeds far more than a hundred untriaged rows"*; the canary measured five triage returns of 26 to 124 KB arriving whole. — `decisions-pending.md` — *read*.
- The M55 seed ledger (`completions/artifacts/M55/seed-ledger.md`) — the key grammar and the rows the port adopts first. — the workflow doc; `CLAUDE.md` — *read*.

---

## 8. The queue — the budget, the hand-on, the order

**Facts**

- **8.1** All three are optional; *"A run that holds an `order` is a run with a queue"*; the two numbers alone make none. `hand-on` is written once; the budget is raised only at its own stop; another order is the human's ruling at any step. An order is first taken only while every ledger row carries a class — so it is written before the ledger's first row without a kind. In a run with an order, `ledger-add` refuses a row without a kind. — `dev/stabilize-record --help`, `run-set` and `ledger-add` — *exercised* (help).
- **8.2** An order is a non-empty list of classes `<kind> <origin>`, kind `route` | `behaviour`, origin `hunt` | `seed` | `under-confirmed` | `under-not-a-break`; a lead and `… under-no-repro` are refused by name. The script ships no order. — same; `DECISIONS.md` → *2026-10-11 — The opening's numbers, and the queue* — *read*.
- **8.3** The human's rulings: one budget per stage, **30** to begin with, follow-ups inside it; the queue ranked by what an item is about — route items first, those under a confirmed parent ahead of the rest; then behaviour items with commands; a follow-up of a finding that did not reproduce is never verified; *"The order stands in the run's opening record"* (2026-10-10, ruling 1). A verifier hands on at most **N = 2** to begin with; *"N stands in the run's opening record beside the budget"*; only a queued route item holds the close (2026-10-11, rulings 2 and 3). — `DECISIONS.md` lines 7831 and 7585 — *read*.
- **8.4** The canary's numbers behind the ruling: 15 of 30 items naming a printed route confirmed, 3 of 50 others; 17 of 18 confirmed were a verifier's follow-up. — `DECISIONS.md` line 7831 — *recorded*.
- **8.5** *"A RUN WITHOUT AN ORDER TAKES THE PATH ABOVE, UNCHANGED"* — the harness has two paths; the one without an order is the path the small canary ran, where verification multiplied (10 → 13 → 38 → 117 items to verify over four triage passes). — the harness header; the workflow doc → *The small canary* — *read*.
- **8.6** The staged canary's setup under `--parts` opened a run with a budget of 9, a hand-on of 2, and an order that left the queue empty. — `DECISIONS.md` line 4562 — *read*.
- **8.7** In a run with a queue a contested verdict reaches the human with its verifier's basis and no advocate's case until the `fix` half is repaired. — `DECISIONS.md` → 2026-10-11, the three questions, *Told with the questions* — *read*.

**What does not exist yet**

- An order, a budget or a hand-on number written for any real run (none exists).
- The rows `wf-verify-fan-out` and `wf-route-walk` are unstruck with trigger *before the run's opening* (section 4, the first disagreement).

**Where sources disagree** — the reshaped stage's build status (section 4) bears on this section too; nothing further.

**What the decision is between, as the doc and the code frame it**

- No queue: none of the three written (the path of 8.5).
- A queue: `order` written (a list of classes, 8.2), with `budget` and `hand-on` each a positive whole number or absent.

**What exists beyond the listed options**

- `queued` — the rows the human puts into the queue by key, at any step (`run-set`, `dev/stabilize-record --help`) — *exercised* (help).
- At every stop the state prints how many agents reported a row carrying each `code` and `site` (`state: view`, `named`), from which nothing follows (`wf-recurrence-count`, the human's idea of 2026-10-11, due at the first stop). — same; `decisions-pending.md` — *read*.

---

## Also a fact of the opening as a whole

- **9.1** Every write of a run is scanned: the machine needs gitleaks and the private denylist, beside python3, git and bash; *"`machine-setup.md` does not list them yet"*. On this host: `gitleaks`, `python3`, `cargo-nextest` and `docker` resolve on `PATH`, and the denylist file named by `implementation/public-hygiene.md` exists and is not empty (its content not read). `implementation/machine-setup.md` names neither gitleaks nor a denylist. — the workflow doc → *Opening a run*; `command -v`; `test -s` — *read*; *exercised*.
- **9.2** The opening is one commit behind the full gate, pushed by name; from that commit on the harness holds the loop branch to its path rule: a commit that changed a product path directly, or a merge that is not a round's by its subject, halts every later stage. — the workflow doc → *Opening a run*, *Branches* — *read*.
