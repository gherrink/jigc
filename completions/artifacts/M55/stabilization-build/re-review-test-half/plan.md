# The second repair of the `test` half — the plan

*Written 2026-10-07 by a planning agent, on `fix/rc24-tier1` at `cbb3d736`, and **revised the same day at `a7d15955`** after an independent review of it ([plan-review.md](plan-review.md): build after the named changes, ten blocking findings — §2 is the disposition of each). It rests on both re-review reports beside this file ([records-state.md](records-state.md), ids `R1`–`R16`; [harness-doc.md](harness-doc.md), ids `R-H…`, `R-M…`, `R-L…`), the first review's two reports and the first repair's plan one directory up, and the code at the tip — the scripts, the harness and the suites are the same bytes at `a7d15955` as at `cbb3d736`. It was **amended at `e76ef470`**, after its first two tasks had landed and the runtime probes had run ([probes/](probes/README.md)): §4 is recut, and §5 says what the probes showed. **Of this plan `K0` and `K1` are built; nothing else is, and nothing here is decided that is the human's.** `F…`/`S…`, `H…`/`M…`/`L…` and `C0`–`C6` are the first review's and the first plan's ids; `B1`–`B10` are the plan review's; a line number of the workflow doc is its line at `cbb3d736`. The first cut's tasks `T1`–`T4` and `T8` are `K2`, `K3`, `K1`, `K4` and `K5` with `K6` here, and the plan review speaks of them by their first ids. This plan's own: `P1`–`P6` (the structural changes), `K0`–`K11` (the core's tasks, a builder's), `X1` (core since the probes), `X3`–`X5` (three small optional tasks; `X2` is absorbed by `K3`), `T5`–`T12` (the tasks of the first cut, which the ruling makes rows), `O1`–`O7` (the orchestrating session's own steps).*

**How this revision is laid out, and the ruling it was finished under.** The first cut was ten tasks, a canary at the real run's shape and a third review. Its review found that four of the tasks, the canary and three of the five forks could not be built as cut — and that three tasks were cut on facts about the runtime that minutes would show. The revision put one question to the human: the whole plan, or its core. **He ruled on it while the revision was written (2026-10-07, relayed by the orchestrator): (B).** The core now; the runtime probes; a small canary that only shows one `test` stage running end to end with real agents; then the run's opening. **Round 1 of the real run is the tuning round**, and everything else from the two reviews and the plan review becomes recorded rows, fixed between rounds. In his words as relayed: later test rounds — a 1.1, for instance — can be used to improve the workflow; it does not have to be fully automated right at the beginning.

So the plan is:

- **The core (§4)**: the record's commit as one repeatable, reconciling act (`P1`); the record script's checks at the commit boundary *and at the push* (`P2`, built); the derivation's inputs held (`P5`); the regression tool's green resting on committed facts (`P6`) — **and, since the probes, two tasks more**: the harness reads a small digest and never a document, and a long command is started, waited for and judged by the tool. The order is recut so that what the harness reads changes once.
- **The runtime probes (§5)**: their design, and — run on 2026-10-07 — what they showed, with the judged results committed beside this plan.
- **What (B) means (§6)**: the small canary, cut concretely; what a round-1 stage can still do wrong and what each case costs; and a short note of what was not chosen.
- **Every finding, with its home or its row (§7)** — the table the run's ledger can be seeded from at the opening — the forks that are left (§8), what the `fix` half is owed (§9), what is declared (§10).

## 1. What was verified, and how

Everything ran in throwaway roots minted with `mktemp -d` under one session scratch root, on the committed bytes of `cbb3d736` (the working tree was clean at that commit). Nothing ran in the repository's working tree but four read-only `dev/regression-set` calls (`check-list` twice, `verdict` twice, their output in the scratch root) and reads of git objects, nothing was pushed to its remote, and no file of it was changed.

- **Review A's blocks** were run from its own helpers (`lib.sh`, `killat.py`, `killstep.py` — the report's appendix, verbatim), under `bash`, one block per finding, each in a fresh light rig or a fresh clone with a bare remote of its own.
- **Review B's scenarios** were run against the committed harness under the committed runtime stand-in, in rigs built as the report's *The rig* says; the one scenario that needs a changed stand-in (`R-H2`) used a copy with the one line the report names. The regression tool's blocks were rebuilt from the report's text — the list files and the evidence directory were written anew.
- **Three probes of this plan's own**: a stage stopped after its preflight and then invoked again; the harness invoked in a repository that holds no such run; and the neighbouring case of `R-M4`.

| | Finding | How it was checked here | Result |
|---|---|---|---|
| `R1` HIGH | a file at a report's path is committed and pushed unscanned | block run | **confirmed** — the script refuses the text (`hygiene`, 11); the same text, written by a redirect, is on the bare remote's branch three steps later |
| `R-H1` HIGH | the state document is relayed whole, 75–150 KB at the real run's size | measured (four sizes) and the return driven | **confirmed as measured** — 3,179 · 25,324 · 74,956 · 142,906 bytes (seven more than the report's, the rig's path); a stage's return over a ledger of sixty rows is 38,609 bytes, 37,686 of them `state`. **Since observed (§5):** a real agent's relay of that line fails its hash at *every* size — at 3 KB as at 160 — so the grade no longer rests on an inference, and the finding is larger than it was reported |
| `R-H2` HIGH | one field left out of one return halts the stage after the instruments ran | two of its three cases driven | **confirmed** — no `asserted_sha256` → `halted`, `binary`, the position at attempt 2; `confirmed` with no `regression` → `halted`, `binary`. The third case (no `ran_on.previous`) cannot be scripted under the stand-in and was read (`stabilize.js:2026`) |
| `R2` | an applied batch that outlives its commit | both ways in run | **confirmed** — a kill before `settle`: `git-state` → `ready`, `pending: null`, the next `apply` refused `pending`, and `discard` reverts committed tables; the same ruling sent twice: `no-batch`, the journal keeps the batch, every later `apply` refused |
| `R3` | a commit that fails leaves a staged tree no act leaves | block run | **confirmed** — `halted git`; with the hook gone the commit step and `git-state` both refuse `dirty` |
| `R4` | the temporaries of a killed write make the tree dirty | block run | **confirmed** — eight temporaries, `dirty`, *the human reconciles it*; `recover` → `ready`; a killed reporter's temporary is `extra` to `check-reports` (20) |
| `R5` | an item whose clause the census does not name judges nothing | block run | **confirmed** — `item-set` exit 0, `not_ready = []`; after the round `next = "close"` over a void item. **This one is no halt on the safe side**: a typed letter at an opening closes a run over an instrument that never ran to its end |
| `R6` | `item-set` replaces an item whole after it has results | block run | **confirmed** — `retest` becomes `close` |
| `R7` | a record committed and not pushed is said by nothing | block run | **confirmed** — one commit ahead, `git-state` → `ready`, `position.test` = round 2 |
| `R8` | a record in a round with no gate on record cannot be committed under a red candidate | block run | **confirmed** — `gate-red`, `known = []`; a direct `gate-set` leaves a tree the commit step refuses |
| `R-M1` | after a rejected push the same invocation is refused and pushes nothing | scenario run | **confirmed** — invocation 2: `refused` (`stopped`), two agents, local and origin still apart. The same finding as `R7`, from the harness's side |
| `R-M2` | two dead verifiers halt the stage | scenario run, with the one-death control | **confirmed** — one dead: recorded, `unverified`; two dead: `halted`, `triage`, no ledger row; again: `attempts-spent` |
| `R-M3` | the stood-on reporters are checked after the instruments | both preflights driven | **confirmed** — eleven agents before the halt, both times |
| `R-M4` | a second pass that keys what was left open to a confirmed finding halts at the record | both keyings driven | **confirmed** — a new key: recorded, two rows; the same key graded `no-break`: `halted`, `record`, the ledger empty. The neighbouring case (graded `unclear`, its verifier dies) was **not driven**: this plan's scenario for it was scripted wrong, and it stands as the reviewer traced it |
| `R-M5` | the triage definition asks for an entry the arithmetic refuses | scenario run; the sentence read | **confirmed** — `halted`, `triage`, *1 in, 2 entries and 0 merged* |
| `R-M6` | the regression tool is green over a list no commit holds, and over two commits that are one | three calls run; `act_run` read | **confirmed** — `git cat-file -e d423f09b:<the list>` exits 128 (control: `cbb3d736` exits 0); `check-list` over an uncommitted list with a row nobody ruled → `listed`; over one commit twice → `listed` |
| `R-M7` | one test in a hundred drops out with the verdict green | evidence built, two calls | **confirmed** — two of 200: `green`, `excluded: 2`; three: `void`, `baseline` (13). The clone in place of the archive was driven by nobody |
| `R-M8` | a check's verdict is the preflight's reading of a brief | read | **confirmed by reading** — nothing to drive: no run has the item, no brief exists, the definition's whole word on it is *once your prompt names one*, the harness reads `status` and `commit` of what the agent returns, and the second preflight's prompt names no previous release |

**Eighteen of eighteen HIGH and MEDIUM confirmed; none refuted; none that could not be checked.** Of the eighteen LOW: **driven** — `R9`, `R10`, `R11`, `R12`, `R13` (each block run, each as reported) and `R-L1` (run: `recorded`, local and origin both moved); **confirmed by reading the lines cited** — `R14`, `R15`, `R-L3`, `R-L4`, `R-L5`, `R-L7`, `R-L8`, `R-L9`, `R-L10`; **not checked here** — `R16` (the mutant `m18` was not re-run), `R-L2` (traced by its reviewer, driven by nobody) and `R-L6` (the mutant was not re-run). The first review's `F2` was driven and still answers `close` on the pre-fix candidate.

**What the probes of this plan's own showed.** (1) A stage stopped after its preflight and scope leaves four untracked files, and the same stage invoked again reaches its record as attempt 2 — the first of the five things the doc's builder found (the build record's README) is closed by the pending-writes rule. (2) The harness invoked in a repository that has no such run halts at its first step, `wrong-branch`, with one agent launched, no commit and a clean tree — which is what §6 rests the canary's safety on.

**Qualifications to the reviewers' claims** — none changes a finding:

- `R-H1`'s grade rested on an inference (a relay of that size fails) when this was written. **The relay probe has since observed it, and more**: the line fails at every size, for a reason that has nothing to do with size (§5).
- `R6` and `R9` are a direct caller's: the harness composes no `item-set`, and no stage starts before the opening is done.
- `R8`'s reach is first-run teething under a red candidate. It does not matter whether the harness passes `--round` there: with no gate on record `known` is empty either way.
- `R5` is graded MEDIUM by its reviewer, and both reviewers' summaries say nothing found records anything false. `R5` does: it is the one finding of the round whose end state is a wrong `close`.

**Driven again for the revision** (§2 has what each showed): the plan review's `B1` — the grant, and then the granted stage through its own record — in a fresh clone; its `B2` in a fresh clone; its `B10` and the denylist path of its `B5` in fresh light rigs. Its other findings were read against the lines it cites.


## 2. The plan's review, and what this revision does with each finding

*Accepted* — the finding is right and the plan is changed. *In part* — right in substance, with a qualification stated. **None is refuted.** Three were driven again for this revision, in fresh rigs and a fresh clone under the scratch root, as review A's helpers build them; the others were checked against the source lines the review cites.

| | The review's finding | Checked how | Disposition | Where it went |
|---|---|---|---|---|
| `B1` | Fork 2's option A makes the stage it grants unrecordable | **driven**: the grant recorded with a `gate-set` for the commit the tree stood on → `recorded`, pushed, `next = test`, attempt 3 begun on the new tip; that stage's own batch → *refused `exists` … `r1/gate.md` holds the gate of … — a round tests one candidate*, exit 6 | **accepted** | Fork 2 is withdrawn as a fork (§8). `T5` is a row, kept with the mechanics the review names — the baseline as a file the commit step is handed, and no `gate.md` written by a record that tests nothing — and a *done when* that drives the granted stage through its own record (§6) |
| `B2` | `P2` scans at the commit; the irreversible act is the push | **driven**: a report with the denylisted term and a host path, committed under the run's directory by a plain `git add` and `git commit` with the term in its subject → `git-state` `ready`, `push` `ready`, and the bare remote holds the text and the subject | **accepted** | `P2` now has three boundaries — the report check, the commit, **every push** — and `K1` lands before `K2` builds the cell that makes an owed push, so that cell never admitted anything unvetted (§3, §4) |
| `B3` | a vet hit has no exit | read: a report is written once (the script refuses `exists`); the journal keeps the batch's `check-reports` result (`dev/stabilize-record`, `apply`); `record` reads that stored result | **accepted** | three exits, one per boundary (§3 `P2`): at the report check the file is no report and leaves the tree; at the commit the batch is taken back and the next attempt starts clean; at the push it is the human's, named |
| `B4` | `P4`'s rule is not what `T6` builds; *as ruling 11 has it* is wrong | read: `stabilize.js:2342` (every ending of triage halts), `:2316-2317` (a wrong hash), `:1902-1904` (a refused batch); ruling 11's text says *every agent asserts the hash before it drives anything*, and nothing of a halt | **accepted** | `P4` is a row, kept with **both designs stated and neither chosen** (§6). The attribution to ruling 11 is withdrawn (§8) |
| `B5` | the canary cannot show what `T11` waits for, and its plants mask each other | the denylist path **driven**: a ledger row that quotes the term → `apply` refused `hygiene`, exit 11, nothing of the batch written — so that plant halts the stage at its record, before the rejected push and the red candidate are reached. The count of part C's invocations was not driven again | **accepted** | the canary of the first cut is withdrawn, and what one at that shape would have to hold is kept (§6); the small canary plants one thing, a stopped verifier, and nothing that can mask anything |
| `B6` | `T11` is unreviewed code on the real run | reasoned against the rule of 2026-10-06 and the bound of three cycles | **accepted** | `T11` may hold numbers and nothing else; anything more is a fourth cycle, which the bound returns to the human (§6) |
| `B7` | three tasks are cut on runtime facts that minutes would show | reasoned; `stabilize.js:1793-1811` read (a return that never arrives is three tries and a count toward the breaker) | **accepted** | the four probes are designed as a step (§5), built by the core's first task `K0`, and each names the task its result changes |
| `B8` | three *done when* and write-set defects, and one of `T6` | (a) review A's own table: a kill before the journal ends at the state before the batch; (b) `git_state`, `record` and `push_branch` read: the branch is asked first today, and nothing pins it; (c) `dev/stabilize-record:2772` (a green row's detail is `-`), `:1021` (no fact of a round for a list's hash); (d) `finding-verifier.md:19,27` and `stabilize.js:2026` | **accepted — (d) in part** | (a), (b): `K2`'s *done when* (§4). (c): since the probes it is `K10`'s — a scripted check's verdict file is a file of the record. (d): the definition does return `regression: false` for a door the previous release lacks; what it need not return is the previous binary's hash, which the harness demands with every `confirmed` — so the fix is the review's, a step of the orchestrator's on the verifier's definition (`O6`), and the first cut's *no regression fact → unverified* goes with `P4` |
| `B9` | the canary's opening has no author | the order of 2026-10-06 read (ruling 3: the half re-reviewed and given a canary run, *then* the run is opened) | **accepted** | the small canary's opening is synthetic and written by `K7`'s script; the question of drafting the real opening first went with (A) |
| `B10` | `R5`'s class is not closed: a mistyped door | **driven**: one letter wrong in an item's door → `item-set` exit 0, `selected: false`, `next = "close"`, the clause green; the control, spelled right, → `retest`, the clause void | **accepted** | it cannot be told from an item no scope was ever meant to reach, so it is **declared**, and `K4` makes the script compute it and the stage's return name it (§3 `P5`, §4, §10) |

**The advisory findings** — each accepted:

| The review's point | Where it went |
|---|---|
| a kill *during* a git child leaves an index lock: the class is not finite as claimed | `P1` no longer claims it. A lock is a cell: named, never removed by the tool (`K2`); what a kill inside a git child can leave beyond the lock is declared |
| `stopAfter: 'state'` over an owed push | a cell: an invocation that only looks finishes nothing and says what is owed (`K2`, `K3`) |
| *ahead by anything else → the human's* turns every unpushed tuning commit into a human halt | a cell of its own: refused, with the orchestrator's way out — it is some task's commit, behind that task's gate, and its author's to push (`K2`); declared |
| the table has four facts and no intent; the `fix` half's acts need dimensions it lacks | `P1`'s table is keyed by **act and phase** (§3), printed as data, and the test that kills reads the print — so the `fix` half adds an act and the same test covers it (§9, row 1) |
| `P3`(a) covers one act: refusal lines and `record`'s red lists are relayed free text too | kept with `P3`'s row (§6) |
| `P4` over-claims closing the never-executed lines: a failed relay, a thrown call, a report nobody launched, an executor that halts | kept with `P4`'s row: each needs a row of the table in either design (§6) |
| `T5`'s count rule refuses every later record once a tuning commit removes a test | the count rule is dropped; the lead it answered found no mechanism, and stays declared (§10) |
| `T8` needs two real runs, and an exclusion row has no form | `P6` is cut as `K5` and `K6`: the tool and the first run; the rows and the second (§4) |
| `O5` is *after part 0* in one place and *after the canary* in another | after the probes; it is a permission and needs the human's word (§4) |
| `T7` rewrites what the harness reads after `T2`, `T5` and `T6` built on the old contract | kept with `P3`'s row (§6); `K3` reads the step tool's own line and no state document, so the core does not build on what that row will change |

## 3. The causes, and the changes

Thirty-six findings and the leads come down to six causes. Four are one sentence of the first repair that was built for less than its class: *an act is repeatable and every invocation reconciles first* (`C2`, planned for the `fix` half), *nothing decisive is only in an agent's hands* (`C6`, built for the fork alone), *every ending of every agent has one row* (the decision of 2026-10-06, built for one dead agent), and *a write is taken only where the state asks for it* (`C5`, built as a census of names). `P1`, `P2`, `P5` and `P6` are the core and are stated whole; `P3` joined the core after the probes; `P4` is stated as far as its cause and its class and is a row, fixed between rounds.

### P1 — The record's commit is one repeatable act, and every invocation reconciles first *(core)*

**The cause.** A record step is six sub-steps — apply, gate, `git add`, `git commit`, `settle`, push — and three predicates read what they leave behind (`admitted`, `live`, `pushed_is_not_ahead` in `dev/stabilize-step`). None enumerates the states *between* the sub-steps, so each gap is a state the act itself then refuses, or that reads as finished.

**What it closes.** `R2` (both ways in), `R3`, `R4`, `R7` ≡ `R-M1`, `R-L1`, `R-L5` for the `record` step, the lead *a commit made on the loop branch while a stage runs*, the first review's `L2` (the part left open) and the `test` member of `H5`'s class; the doc's lines 100, 213–215, 314 and 417.

**The change: one table, keyed by act and phase.** An *act* is a step that changes git or the record; its *phases* are its sub-steps in order; each phase has a predicate that says, **from git and the journal and never from a flag alone**, whether it is done, and the one step that resumes it. The journal names the act that is open. Today there is one act with phases, `record`: *applied* (the journal holds the batch) → *committed* (every file of the batch is in `HEAD` at the hash the journal holds) → *settled* (the journal is cleared) → *pushed* (the remote stands at the local head). Reconciling is: for the open act, the first phase that is not done — finish it, or say who does. The table is data in `dev/stabilize-step`, printed by a read, and `git-state` and `record` both answer from it:

| What the tool reads, in this order | Its answer |
|---|---|
| the branch checked out is not the one the act names | refused `wrong-branch` — **before any write, fetch or push** |
| git's own lock stands in the repository (`index.lock`) | refused, naming the file: the tool never removes a lock. No git process holding it, the orchestrator removes it and invokes again |
| a write of the record script killed between its files | finished: `recover` |
| the script's own temporaries and no journal — a write killed before its journal | finished: `recover` removes them; the state is the one before the batch |
| `record`, *applied*: the batch's files in the tree, staged or not, each as the batch wrote it | `pending` — a batch to gate and commit; the commit step is repeatable |
| `record`, *applied*, a file of the batch changed since | the human's, as today |
| `record`, every file of the batch in `HEAD` | finished: `settle` |
| a batch that would change nothing | refused at `apply`, before a journal exists |
| no open act; the loop branch ahead of its remote by commits that touch only the run's directory and that pass `P2`'s check | finished: the push that is owed |
| ahead, and a commit of the range fails that check | the human's, naming the commit and the place — never pushed |
| ahead by a commit that touches anything else | refused, and not the human's: it is some task's commit behind its own gate, which its author pushes; the tool pushes no commit that is not a record's |
| the remote ahead, or the two apart | the human's, as today |
| anything else in the tree | the human's, as today |
| an invocation that only looks (`stopAfter: 'state'`) | every answer above that would *finish* something is reported as owed, and nothing is finished |

And the acts become repeatable: `record` admits its own staged lines, takes a batch that is already in `HEAD` as recorded, and holds the head to the commit the stage began on; `discard` becomes an act of the step tool, which can ask git, and refuses a batch whose files are committed.

**Why one change closes the class, and where the class ends.** The class is *the states a record step can be found in between two of its own commands* — a fixed list of child commands and of file operations, enumerable by an instrumented run. **What would show it has not closed:** the test that kills before each of them and asserts that the tool ends, within two reads, in **one of two states and no third**: where the journal was not yet written, the state *before* the batch — the tables as they were, the reports and the scope still pending, the attempt counted, nothing to push; where it was, the state of the uninterrupted control, with the remote at the local head and a clean tree. **Where the class ends:** a kill *inside* a git child. Its one common trace, the index lock, has a cell; that git updates a ref whole or not at all is relied on and not tested; anything else such a kill leaves is the last row of the table.

**This is `C2`, moved — and keyed so it is moved once.** The first plan put *acts bracketed by an intent and a done record, reconciled first* under the `fix` half. The window it leaves is the `test` stage's own record step, as both reviewers found independently. It is built now, for `record`. The `fix` half's acts — `land`, `carry`, `open-round`, the part's revert — are added as acts of this table with their own phases, and the test that kills reads the table's print, so an act added is an act killed at every phase without a new test (§9, row 1).

### P2 — What is published is what the script would have written: its checks at the report check, at the commit, and at every push *(core)*

**The cause.** The record script scans at the *write*; the step tool commits by *name* and pushes by *branch*. Between a write and the remote, a file is whatever its last writer left, and a commit is whoever made it.

**What it closes.** `R1` (HIGH), `R13`, `R15`'s second bullet, the doc's lines 108 and 287 — and the plan review's `B2` and `B3`.

**The change.** The script's write-time checks — the two scanners CI runs over a push, a host path, the fenced line, the end marker — become one read, `vet`, the same functions its writers call. It runs at three boundaries, and **each has its exit**:

| Boundary | What is vetted | A hit |
|---|---|---|
| the report check, the first of a stage and every later one | every report a launched reporter left, and the round's scope | **the file is no report.** The check names it as refused, the step moves it out of the tree into the invocation's scratch, and its reporter has left none — which the stage already has a row for: the item is void, or, for a reporter the stage stands on, the stage halts there. The stage goes on |
| the commit | every path the act is about to add, the subject, the staged blobs against the bytes vetted, and the batch's stored report check against the disk | **nothing is committed and the batch is taken back**: the file leaves the tree as above, `discard` restores the tables, and the next invocation works the next attempt from a tree it can start from. It is reached only by a file changed in the minutes between the report check and the commit |
| every push the step tool makes | the subject of every commit the push would publish, and what each adds or changes | **nothing is pushed, and it is the human's**: the commit is local, and taking a commit back is his to confirm. The refusal names the commit and the place, never the text |

A scanner that cannot run — no denylist, no `gitleaks` — is a refusal at each boundary, never a pass; the script has that word already (`did-not-run`).

**Why one change closes the class.** The class is *bytes that reach the remote without the scan*. The step tool makes every push of a run, and after the change every such push is preceded by the scan of exactly what it would publish — whoever committed it, and under whatever name. The two earlier boundaries are not what makes that true; they are what turns a hit from a stage lost into an item void. **What would show it has not closed:** `B2`'s block ending in a refusal with the remote unmoved; a test that takes each refusal the script has at a write, times each kind of written-once file and the subject, at each boundary; and the mutant without the check at the push, red. **What it does not cover, and nothing in a script can:** a push somebody makes by hand.

### P5 — The derivation's inputs are held where they are written *(core, in part)*

**The cause.** `C1` made a clause's status a derivation. Two of its inputs can still be bent after the fact — an item's row — and the table that says where a writer is taken (`C5`) is a census of names.

**What it closes.** In `K4`: `R5` and `R6`. Declared and named by `K4`: the plan review's `B10`. Rows, due with the `fix` half: `R9`, `R12`, `R14`.

**The change.** An item's clause is a clause of the census, in both orders of writing; an item that has a result keeps the four cells the derivation reads. **And what cannot be held is named**: an in-scope item that no tested round's scope has selected is computed by the script, is in the state, and is in every stage's return — beside the doors of the round that no item reaches, which the scope step returns today.

**Where the class ends — `B10`.** Review A enumerated the four cells of an item's row that the derivation reads: the clause, when it runs, its doors, its registries. The clause can be held, because the census is a closed list. **A door cannot**: a round's doors are derived afresh by the scope step, there is no list a door's name could be held to, and a door mistyped by one letter reads exactly as a door this round's change did not reach — an item that owes nothing (ruling 10). Driven: the run answers `close` over it. So this member of the class is **declared** (§10), and the bound's reach is what `K4` makes visible: the human reads the items that were never selected at every stop, and at the close.

**Why this is not a third patch.** `R5` and `R6` are the end of an enumeration — of four cells, two were unheld or replaceable, one can be held and is, and one cannot and is named. `R9`, `R12` and `R14` are instances of a class whose closure is the writers' table as the code path; patching the two instances of `R12` would be the patch the lesson warns of, so they stay declared until that table is rebuilt.

### P6 — The regression tool's green rests on facts of the two commits, and on nothing it was handed *(core)*

**The cause.** The tool proves which binary ran. What a green *rests on* beside that is taken as an argument: the list's file, the two commits, and which tests may drop out.

**What it closes.** `R-M6`, `R-M7`. Its caller, `R-M8`, is `K10` and `K11` since the probes.

**The change.** The list is a path *in the candidate's commit*; two commits that are one are refused; a test that fails on its own binary is excluded only by a row that names it and says why — the tolerance of one in a hundred goes; and the previous release is built from a clone with no checkout, which is a repository and no registration in this one's, so that the eleven tests that ask git about their tree are measured again. **An exclusion row** is a row of the same list and of its four fields, with a pointer of a kind of its own that resolves to the committed record of the run that showed the test failing on its own binary; a row for a test that passes there is reported as stale, as the tool reports a stale row today.

**It takes two real runs, of about thirty-five minutes each.** The first, with no exclusion row, names the tests that fail on their own binary — eleven, fewer, or none. Only then can their rows be written; and only a run on a commit that *holds* those rows can show that what is excluded is exactly what is listed. So the change is two tasks, and the second exists only if the first names a test.

**Alone, as a group.** These share nothing with the other causes but the caller. **Three builder's choices are overturned by it** — the list as a file the tool is handed, the tolerance, the archive — each recorded as a builder's in `DECISIONS.md` → *The regression set's first part*, items 4, 7 and 8. None is a ruling of the human's.

### P3 — nothing a script can read or decide goes through an agent's hands *(core, since the probes)*

**The cause.** The harness has no filesystem, so what it computes on comes through an agent: the state document inside a relayed line, a check's verdict in the preflight's reading of a brief, a command's waiting in a shell the agent composes. The first repair made every *git act* one command with a hashed line (`C0`) and put *the fork* on record (`C6`); it left the document in the line and the long commands in prose.

**What it closes.** `R-H1` (HIGH) — observed, and wider than reported: every line of the step tool that holds one character outside ASCII fails its hash on the way back (§5). `R-M8`, for the checks that are held commands. `R-L3`, `R-L4` and `R-L7`'s wording. And the human's ruling of 2026-10-07 that no step of a stage may need his permission.

**The change** is §4's two rules: **a line holds what the harness acts on and nothing else**, ASCII by construction, and everything else is read from files by key (`K9`, `K3`); **a long command is started, waited for and judged by the tool**, and every command a prompt names is one plain invocation of a committed tool (`K10`, `K11`).

**Why one change closes the class, and where the class ends.** The class is *a fact computable from the disk or from git that the harness takes from an agent, and a shell an agent composes because the harness asked for one*. Its boundary is an agent's own judgment — a finding, a grade, a verdict, a scope's derivation — and a role's own craft, which no prompt names. **What would show it has not closed:** a line of any act that is not of its declared shape; a prompt that carries a ledger row's text; a prompt the fence finds a pipe in; and, outside any test, **the relay probe run again on the digest** — the only instrument that has ever seen the runtime's side of this.

**This is `C6`, extended — and `X2` and the first cut's `T7` and `T9`, promoted.** The ruling on scale made them rows on the reading that `R-H1` was an inference and the check's verdict a risk to carry through round 1. The probes showed that the first state read of any real stage fails, and the human ruled on the permission prompt: neither is something round 1 could be tuned through.

### P4 — a row; its cause and its class

**Every way an agent can end is one row of one table.** *How an agent can end* is prose in two places and code in twenty. It closes `R-H2` (HIGH), `R-M2`, `R-M3`, `R-M4`, the first review's `M9` in part. Its class is the cells of that table — and, as the plan review showed, the first cut's headline rule over it was not what its task built. Under the ruling it is a row (§7); **one of its cells, `X1`, is core since the probes**, and three more are optional tasks. §6 keeps what the plan review found about it.

### The findings that are genuinely alone

`R8` (what a record is held to where its round has no gate — `T5`'s row) · `R10` (the `fix` half's) · `R11` and `R-L2` (declared) · `R-M5` (one sentence of a definition that asks for what the harness, rightly, refuses — `O1`) · `R-L6` and `R-L10` (a sentence of the doc each) · `R-L8` (declared; §8) · `R-L9` (`O4`).

## 4. The core — as it stands after the probes

**Landed:** `K0`, the runtime probes (`1d21024e`) · `K1`, `P2` (`e76ef470`). The probes ran (§5), and with a ruling of the human's they change the core in four ways: **two tasks are promoted into it** — the harness reads a digest and never a document (`K9`, with `K3`), and a long command is started, waited for and judged by the tool (`K10`, with `K11`); **`X1` is decided and is core**; **`X2` is absorbed**; and **the order is recut so that what the harness reads changes once**.

**The cut.** Every task that changes what a script *prints* or *takes* comes first and leaves the harness as it is: a script's new line is a second form of the old one, asked for by a flag; a writer's new way in is a flag beside the old one. Then **one** task changes what the harness reads (`K3`) and deletes the old forms; then one changes what the harness tells an agent to run (`K11`); then `X1`, which touches neither contract. `K1`'s builder left four harness edits owed (`e76ef470`'s message): they are `K3`'s.

**Every task is one `build-executor` behind the full `dev/gate`, on the tree of its own commit; they run in the order of the list at the end of this section, one at a time.**

**Write sets.** `REC` = `dev/stabilize-record`, `tooling-tests/dev_stabilize_record.rs` · `STEP` = `dev/stabilize-step`, `tooling-tests/dev_stabilize_step.rs` · `PROBE` = `dev/stabilize-probe`, `tooling-tests/dev_stabilize_probe.rs` · `HARN` = `.claude/workflows/stabilize.js`, `tooling-tests/stabilize_simulation.rs`, `tooling-tests/stabilize_harness_fence.rs`, `tooling-tests/fixtures/stabilize-runtime.mjs`, `tooling-tests/fixtures/stabilize-test-stage.trace` · `REG` = `dev/regression-set`, `tooling-tests/dev_regression_set.rs`, `completions/artifacts/M55/stabilization-build/regression-set/` · `DOC` = `implementation/stabilization-workflow.md` · `LOG` = `DECISIONS.md`.

**Edits under `.claude/`.** The harness is a builder's where it is a task's subject, as in the first repair. **The agent definitions and the settings are the orchestrating session's own steps**, each small, each its own gated commit. **No task and no step edits `CLAUDE.md`.**

### The digest — what `K9` and `K3` build

**The rule.** A line a tool hands the harness holds **what the harness acts on and nothing else**: ids, states, words of a fixed list, counts, hashes. It is printable ASCII by construction — each field of each act has a declared shape, and a value that does not fit it cannot be printed — so it holds no free text, no escape and no backslash, and there is nothing for a structured return to decode. It is held to a hash, as today. **Everything else an agent reads from the files itself**, by a key the line or the prompt gives it. This is every line of the step tool, not the state's alone: a refusal's prose and a commit's subject fail the relay exactly as a ledger row does (§5).

**What the harness uses of the state today, read off its source** (`stabilize.js`, every `state.` it reads), and what the digest holds for it:

| The harness reads | For | In the digest |
|---|---|---|
| `opened`, `next`, `not_ready`, `stop` (why, round, then), `round`, `fix_rounds`, `candidate.round` | every decision of where a stage stands; every return | as they are — words and numbers |
| `position.test` / `position.fix`: round, attempt, cycle, `refused`, `triage`, `rerun` (clause, round, each item and its attempt) | which attempt begins, or why none does | as they are; **a re-run's doors are not in it** — read by key |
| `facts.previous`, `facts['previous-commit']`, `facts.scope` | the preflight's and the scope step's prompts | a version, a sha, a word |
| `rounds[…].facts.candidate` of the round in hand and the one before | the commit a finishing triage builds; the scope's base | the two shas |
| `items`: id, kind, clause, when it runs, `selected` — **and brief, doors, registries** | which chains run; each unit's prompt | id, kind, clause, runs, `selected`, and **how many** doors it has this round. Brief and doors: by key |
| `doors.included`, `doors.excluded` — each door with its registry and its derivation | each unit's doors; a log line | **counts** — included, excluded, reached by no item |
| `clauses`: status, round, commit, behind; each item's round, attempt, standing, why | the evidence a `close` hands back; which re-runs are spent | as they are — words, numbers and shas |
| `untriaged` (key, why) joined to **`ledger` rows** (door, clause, repro, grade) | the lines triage is handed | **a count**, and a count per *why*. The rows: by key |
| `ledger.some(key …)` | whether a ruling names a row | not in it: the batch's own `check-ledger` says so, as it does today |
| `blockers`, `human_list` | counts in a `test` stage's return; and the lists themselves, returned to the orchestrator | **counts.** The lists stay in the document's file, which the return names |
| `human_clauses`, `human_stages`, `retest`, `forbids_close`, `unsettled`, `never_selected` (`K4`) | what a refusal and a return say | as they are — short lists of ids and words |
| — | — | the document's path **relative to the scratch root**, and its sha256 |

**What replaces each piece of free text the harness passes on to an agent today:**

| Free text in a prompt or a return today | Replaced by |
|---|---|
| an item's brief, in its unit's prompt and in the preflight's list of checks | a read the prompt names: the item's row, by run and item |
| an item's doors — name, registry, derivation — for the round | a read by run, round and item. The script does the selection the harness does today (`unitDoors`); a re-run names the round its position gives |
| the untriaged ledger rows, as lines in triage's prompt | a read by run: the untriaged rows whole. The harness keeps the count, which triage's arithmetic is held to |
| the human's list and the blockers, in a stage's return | counts, and the state file's path: the orchestrator has a subagent read it |
| the whole state document, in a return whose `next` the script does not know | the file's path (`X2`, absorbed) |
| a refusal's cause, evidence and recommendation, in its line | the refusal's **word** in the line; the prose in a file beside the state's, named in the line by its relative path and its hash. The harness says what a word means from a table of its own, held to the tool's words as its table of positions is today |
| a pending batch's subject; the paths of a record; the lists of red in a gate check | counts and flags in the line; the lists in the same file |

**What it does not touch:** what an agent *returns* of its own judgment — a finding, a grade, a verdict, a scope's derivation. That is a structured return, held to no hash, and the write direction is sound (§5). Its size is a row (§7).

### A held command — what `K10` and `K11` build

**The rule (the human's ruling of 2026-10-07, §5).** No agent of a stage composes a shell. A long command is **started by one plain act** of the step tool, detached, its output in a file the tool names from a flag; **waited for by one plain act** that returns within a bounded slice and answers `running` or `done`; and **judged by the tool**, which reads the command's own last lines and prints the verdict as a digest. One agent waits by asking again — the cheap form; because the command outlives its starter (§5), another agent can take the wait over, and the record of what was started is the tool's file and no agent's memory.

**What it covers — a table in the step tool, one row per kind**: the candidate's gate and a record's gate (judged from the gate's own two lines, by the reader the record script already has); **the regression set** (judged from the tool's own line: its status, and its two commits held to the run's); **the build of a binary** from a commit — the candidate's and the previous release's (judged by the hash, the version it prints and the path a bare `jigc` resolves to); and the hold probe's case (a). *The build is this plan's widening of what was asked*, for one reason: the preflight's prompt names `git archive`, a pipe and `cargo build` today, and the fence below would be red on it. It can be dropped — then the fence's domain says so, and that pipe stays a place a prompt can appear.

**With it, every writer an agent calls takes its text from a file named by a flag** — a report, a scope, a record's batch with its hash — where today a prompt says *as a single here-document*. The agent writes the file with its file tool; the command is one plain invocation.

**A scripted check is a kind of item.** An item whose check is a held command says so by its kind and names the command by its id; the result of such an item is written from the tool's verdict file, which reaches the record as a file written once — so the two commits, the list's hash and what was excluded are on record, and **no agent returns the word**. This settles the fork on how a scripted check is run.

**And a fence, over what the harness composes.** Every command a prompt tells an agent to run is rendered by one function, and is **one plain invocation of a tool the commit holds under `dev/`**: no pipe, no redirect, no `&&`, no `$(…)`, no loop, no `sleep`, no `nohup`, no `&`. The fence reads the prompts the simulation's scenarios and the self-test's table actually compose — not the source — and prints the set of tools it found, which is what `O5`'s rules are written from. **Its domain is what the harness names.** What a role runs of its own craft — a reviewer's rig, a driver's commands on the binary, the preflight's read of CI, the tarball install, the trial image — is not named by a prompt, is not held by it, and is listed in §10.

**Which verdicts are still an agent's reading afterwards:** CI's `ci-ok` for the candidate's commit · the packaged-tarball install · that the trial image is verified · whether the cross-model tool answers · the preflight's environment asserts · and any `check` item that is a brief and no held command. And, as before and by their nature, an agent's own judgments: a finding, a grade, a verifier's verdict, a scope. **No longer an agent's:** the candidate's gate and a record's gate — the red list was a script's already, the starting and the waiting were not — the regression set, and a binary's hash and path.

### The tasks

| # | Goal | Writes | Done when | Must not touch | Size |
|---|---|---|---|---|---|
| **`K2`** | **`P1`, the two scripts** — as cut (§3): the table keyed by act and phase, as data; `git-state` and `record` answer from it; the acts repeatable; `discard` an act that asks git; a read that prints the table; a look that finishes nothing. It builds on `K1`: the push it makes when one is owed is `K1`'s vetted push, and `K1`'s refusal `unvetted` is a cell | `STEP`, `REC`, `DOC`, `LOG`; an assertion of `stabilize_simulation.rs` only where it pins the old answer | `dev_stabilize_step::a_record_step_killed_between_its_commands_ends_in_one_of_two_states` — every child command of `record` and `push` and every file operation of `apply`, counted by an instrumented run and read from the table's print; each kill ends within two reads in **the state before the batch** (the journal not yet written) or **the state of the uninterrupted control** with the remote at the local head — never a third, never the human's. `::the_wrong_branch_is_refused_before_any_write_fetch_or_push` — temporaries, a batch that could be settled and an owed push all planted: journal, tree and remote unchanged, and the git commands the tool ran, logged by a shim, are local reads. `::a_lock_git_left_is_named_and_not_removed`. `::looking_finishes_nothing_and_says_what_is_owed`. `::a_commit_that_is_no_records_is_not_pushed_by_the_tool`. By name, too: a commit that failed is asked for again and commits (`R3`); a batch that changes nothing is refused at `apply` (`R2`); `discard` refuses a batch that is in a commit; a temporary is no report (`R4`); a branch that moved under the stage is refused. Four mutants, each red. Review A's blocks `R2`, `R3`, `R4`, `R7` answer as it expected | the harness — every flag it passes and every line it reads today stays; what `state` computes; the `fix` acts beyond a helper they share | L |
| **`K4`** | **`P5`, in the record script.** An item judges a clause of the census; an item with a result keeps what the derivation reads; an in-scope item no tested round selected is in the state | `REC`, `DOC`, `LOG` | `dev_stabilize_record::an_item_judges_a_clause_of_the_census_or_the_run_is_not_ready` (both orders of writing), `::an_item_that_has_a_result_keeps_what_the_derivation_reads` (each of the four cells refused, a brief taken, `next` still `retest`), `::an_in_scope_item_no_tested_round_selected_is_named`; three mutants, each red. Review A's blocks `R5` and `R6` answer as it expected; the plan review's `B10` block names the item while `next` is `close` — the bound, and the doc says so | the harness — that a stage's return names the item is `K3`'s; the writers' table; whether such an item forbids closing | S–M |
| **`K5`** | **`P6`, the tool and the first run — on the tree as merged.** The list read from the candidate's commit; one commit twice refused; an exclusion only by a row; the previous release built from a clone | `REG`, `DOC`, `LOG` | `dev_regression_set`, by name: a list that is not in the candidate's commit is refused; two commits that are one are refused; a test that fails on its own binary and is on no row is void, and the line names it; an exclusion row for a test that passes there is reported stale. Review B's two blocks end in refusals. **One real run**, the candidate the task's parent commit, the list as that commit holds it: its line is committed with the record. *It is expected not to be green*: the green on record is for the candidate **before** the eleven pre-opening fixes were merged (`b33bf39b`), and the list has no row for what they changed on purpose. The line names every difference and every test that fails on its own binary | the swap and its proof; the harness; the list's rows | M |
| **`K6`** | **`P6`, the rows and the second run.** Two commits, each behind its gate: the rows — an exclusion row for each test `K5`'s run showed failing on its own binary, and **a row for each difference that is an intended change of the eleven fixes, pointing at its ruling** (`pre-opening/decisions.md` and the `DECISIONS.md` entries it cites); then the line of a run whose candidate is the commit that holds them | `REG`'s list and record, `LOG` | the second run is green and its `excluded` is exactly the exclusion rows; both lines are in the record. **A difference no ruling covers gets no row**: it is a finding, the task halts on it and hands it to the orchestrator | the tool | S–M |
| **`K9`** | **The digest, in the scripts.** Every act of the step tool prints, when asked by a flag, a line of declared shape; the state's line is the table above; a file beside it holds what the line leaves out; the record script answers an item's row, an item's doors for a round, and the untriaged rows, each by key. The relay probe is pointed at it | `STEP`, `REC`, `PROBE`; of `HARN` the relay probe's one prompt and its check, and nothing a stage reads; `DOC`, `LOG` | `dev_stabilize_step::every_line_is_a_digest` — every act, on success and on each refusal the suite's table names: printable ASCII, no backslash, every value of its field's declared shape, under a byte bound the task states, ending in its hash; the mutant that lets one prose field through, red. `::the_state_digest_does_not_grow_with_the_ledger_or_the_doors` — at 1 · 20 · 60 · 120 rows over 2 · 40 · 150 · 300 doors the line differs only in digits; and a ledger row with a dash, an arrow and a typographic quote changes no byte of it. `::what_a_line_leaves_out_is_in_a_file_it_names` — by relative path and hash; a refusal's prose is there. `dev_stabilize_record::an_agent_reads_its_brief_its_doors_and_the_untriaged_rows_by_key` — the doors read answers what the harness's own selection answers today, on every fixture of the suite. `dev_stabilize_probe`: the relay cases are built on the digest. Without the flag every line is byte for byte what it was | what a stage reads — the simulation's golden trace is untouched; what `dev/stabilize-record state` prints: the document is unchanged | L |
| **`K10`** | **A held command, and writers fed from files, in the scripts.** `hold-start` and `hold-wait` with the kinds above; the tool's verdict per kind; `hash` for a file; a flag on `report`, `scope-set` and `apply` that names the file the text is read from — `apply` with the batch's hash; a scripted check's result written from its verdict file, which is a written-once file of the run and is vetted as one | `STEP`, `REC`, `DOC`, `LOG` | `dev_stabilize_step`, by name: `::a_held_command_is_started_once_and_outlives_its_starter`; `::a_wait_answers_within_its_slice` — `running`, then `done`, and a slice that ends inside a tool call's default time limit; `::the_verdict_of_a_held_command_is_read_by_the_tool` — a gate from its two lines, a pre-check's output refused; the regression set from each status and exit its tool has, a line whose commits are not the ones asked void; a build from its hash, its version and its path, under the suite's stand-in for cargo; a command that died without its last line void; `::a_held_command_writes_only_under_the_scratch_root_it_resolved` — **every path it writes is resolved, and a link planted beneath the root is refused** (`SP-2`'s class). `dev_stabilize_record::a_writer_takes_its_text_from_a_file` — the three writers; a batch whose hash is not the one named is refused and nothing is written; `::a_scripted_checks_result_is_the_tools_verdict` — no outcome word is taken from the caller. Every line is a digest from its first commit | the harness; the regression tool; the gate | L |
| **`K3`** | **What the harness reads, changed once.** A stage starts from `K2`'s answer; an invocation that only looks finishes nothing; every line it reads is a digest, and the old forms are deleted; it hands an agent keys and never text; a stage returns the state file's path and never the document; a halt after a commit says what the next invocation does; a tool step's retry runs the one command again; **`K1`'s four**: the scratch root in the report check's and the commit step's prompts, a report set aside read as a missing one, the two refusals `unvetted`, of which the push's is never the push that is owed; **`K4`'s**: a stage's return names the items nobody selected | `HARN`; of `STEP` the deletion of the old line; `DOC`, `LOG` | simulation, by name: **the stand-in relays as the runtime does** — it decodes a line's escapes — and a whole stage records; a brief, a door's derivation and a ledger row's repro, each planted with a marker and a character that is not ASCII, are in **no prompt and no return**; a stage's return over a ledger of sixty rows holds no document and names the file; the mutant in which the harness takes the document, red. And the ones cut before: a push that failed after the record is made by the next invocation (`R-M1`); `stopAfter: 'state'` over a pending batch, and over an owed push, moves neither head and returns what is owed (`R-L1`); a commit that failed is finished by the next invocation; a ruling sent twice is refused and leaves no batch; a commit step whose agent died once records on its retry; a report set aside voids its item; a push refused `unvetted` halts as the human's | the two record scripts beyond that deletion; `runFix` beyond what keeps it parsing the same lines — its own reads are the `fix` half's (§9) | L |
| **`K11`** | **What the harness tells an agent to run.** The candidate's gate, a record's gate and the regression set are started and waited for by tool steps; the two binaries are built by a held command; a scripted check's verdict is the tool's; reports, the scope and the batch are written from files; a hash is asked of the tool. **And the fence** | `HARN`, `DOC`, `LOG` | `stabilize_harness_fence::every_command_a_prompt_names_is_one_plain_invocation_of_a_tool` — over every prompt the simulation's scenarios and the self-test's table compose: each command is rendered by the one function, names a file the commit holds under `dev/`, and its arguments are plain words; no prompt holds a redirect, a pipe, `&&`, `$(`, a here-document, `sleep`, `nohup`, a trailing `&` or the words *in the background*; the tools found are printed. Simulation, by name: the candidate's gate is held by acts and no agent composes a wait; **a regression set whose tool printed `red` is recorded red while the preflight's stand-in says `green`**; its verdict file is among the record's paths; a waiting agent that dies is followed by another that takes the wait over, and the command ran once | the scripts; the stage's order of steps beyond what the held commands move — the regression set still runs after the gate, never beside it | M–L |
| **`X1`** | **An agent that leaves a field out voids what it was launched for** — decided by the probe: no schema of a stage gains a `required` field, since a return that lacks one is a call that failed, three times over (§5). No hash → its item is void, with that reason; `confirmed` with no regression fact, or with no hash of the previous binary → the finding is unverified. A hash that is returned and wrong still halts | `HARN`, `DOC`, `LOG` | simulation, by name: a reviewer that returns no hash voids its item and the stage records; a confirmed verdict without its regression fact is unverified and the stage records; a wrong hash halts. And the fence arm `K0` added still holds: no schema of a stage requires the hash | the schemas; the scripts | S |
| **`K7`** | **The small canary's setup, and its opening data** (§6). `dev/stabilize-canary` builds the clone, its bare remote and a synthetic opening, and checks them. Its regression item is a scripted check, so no brief is written | `dev/stabilize-canary` and `tooling-tests/dev_stabilize_canary.rs` (new, registered in `tooling-tests/groups/g_tooling.rs`); `DOC`, `LOG` | `dev_stabilize_canary::`, by name, on a small source repository of the suite's own: a setup under a root outside the repository leaves a clone at the commit named, **one remote, a bare repository under that root**, the loop branch and the run's slug, an opening written through the record script, committed and pushed there, and the state answering `not_ready: []`, `next: test`; it prints the invocation — **by `scriptPath`, the clone's own harness**; it refuses a root inside the repository, and a slug or a loop branch the source repository holds; **every path it writes is resolved, and a link planted beneath its root is refused** (`SP-2`'s class); the source repository's refs, tree and configuration are the same before and after; `check` refuses a clone with a second remote or a `url.*` rewrite. And, read off the harness and said in the doc: whether an invocation's `scope` bounds a round to the doors it names | the scripts of a run; the harness | S–M |
| **`K8`** | **The doc says what the code does, and the rows are on record.** Every sentence of review B's table and of `R15` is true or gone; the six things an orchestrator would get wrong are said where an orchestrator reads; **every invocation the doc shows is by `scriptPath`, with the reason**; the bounds of §10 are declared as the human accepted them; every row of §7 that is due later has its row in `decisions-pending.md` — **and the rows the merge left stale there are corrected**: the regression set's row that says *neither is built* beside *the first is built*, the row that still owes part 1's build, and the row of the eleven fixes, whose count of what is built is the count before the merge | `DOC`; the header comments of the scripts and of the harness; `implementation/decisions-pending.md`; `LOG` | the commit's own table: each of the sixteen rows, each bullet of `R15`, `R-L10` and each of the six, against *made true by `K<n>`*, *the sentence as it now reads* or *a row, due …*. Every row of §7 with a due date is found in `decisions-pending.md` by its key; the three stale rows read as the tree stands. `doc_link_fence` green | any behaviour | M |

**Optional, after `X1` — three small tasks, each droppable by the human or the orchestrator.** Each closes one cell of what `P4`'s table would close as a class; each is a builder's, writes `HARN`, `DOC` and `LOG`, and costs about an hour and a gate. They are patches, and are named so.

| # | Goal | Closes one cell of | Done when (simulation, by name) |
|---|---|---|---|
| **`X3`** | Once the breaker has tripped, the stage still checks its reports and records: what was not launched is unverified, with that reason | `R-M2` | two dead verifiers are two unverified findings on record, and `next` is `triage` |
| **`X4`** | A finding graded again drops what the earlier pass established for it | `R-M4` — it halts at the record, and the next attempt can halt the same way | a second triage pass records under both keyings of what a verifier left open |
| **`X5`** | The reports of the reporters a stage stands on are checked before an instrument is launched | `R-M3` | a preflight that left no report halts the stage with no instrument in its trace |

`X2` is no task any more: `K3` holds it.

**The steps that are the orchestrating session's own:**

| # | Edit, or act | Closes | When |
|---|---|---|---|
| **the relay probe, again** | `{ probe: 'relay', scratch }`, **by `scriptPath`**, on the digest: whole, three of three, at each of the four sizes — or `K3` is not started | what §5 leaves unsettled | after `K9`, before `K3` |
| **`O1`** | `.claude/agents/finding-triage.md`: a missing repro block is said in the entry's `why` and in triage's report, and is no entry; the return names `new`, as the schema does | `R-M5` | before a stage runs with real agents |
| **`O2`** | `.claude/agents/milestone-code-reviewer.md`: the hash is read and returned whether or not the pass drives anything | `R-H2`, the definition's half — one sample of the probe did so unasked | the same |
| **`O6`** | `.claude/agents/finding-verifier.md`: with every `confirmed` the previous release's binary is hashed and its hash returned — also where the door does not exist there | `B8`(d); `R-H2`'s third case | the same |
| **`O7`** | the definitions follow `K10` and `K11`, in one commit: the preflight builds through the tool and holds no gate and no regression set; the record's executor writes the batch's file and applies it, and runs no gate; every reporter writes its report to a file and names it; a hash is asked of the tool, not of `shasum` | the ruling on permission prompts, the definitions' half | with `K11` |
| **`O4`** | the three definitions a run launches that still name the `probe` step | `R-L9`, `L8` in part | any time |
| **`O5`** | **the allow rules for the tools the fence prints** — one rule per tool, in the settings' own syntax for a command by its first word. *Where they live is the human's question* (§8), and the step is cut for both answers. **In the committed settings:** an edit of `.claude/settings.json`, the orchestrator's gated commit, made on his word; the rules then travel into every clone, the canary's among them, and into every contributor's session. **In his local settings:** no commit — the orchestrator hands him the lines and he adds them himself; they do not travel, so the canary's clone needs them at the user level or again in the clone, and the fence's printed list is what he checks them against | the ruling on permission prompts | after `K11` |
| **two probes, again** | `hold`, its arm (a) now through the acts — **no prompt appears**, which is what shows `O5` works; and `payload`, the batch now written with the file tool | what §5 leaves unsettled | after `K11` and `O5` |
| **the bounds** | §10, put to the human before `K8` writes them into the doc | — | before `K8` |
| **the canary** | the steps of §6, in a session that stands in the clone, every invocation by `scriptPath`; and its record | the build's own verification, never run | after `K7` |

`O3`, a brief for the preflight, falls away: a scripted check has no brief.

**The order from here.** `K2` → `K4` → `K5` → `K6` → `K9` → *the relay probe, again* → `K10` → `K3` → `K11` (with `O7`) → `O5` → *two probes, again* → `X1` → `K7` → the optional tasks that are kept → `O1`, `O2`, `O6`, `O4` (any time before the canary) → the bounds → `K8` → the canary → the record that the `test` stage is used for round 1 → the run's opening, which is the human-led step.

*Why this order.* `K2` and `K4` are the scripts' part of what was already cut, and change no line the harness reads. `K5` and `K6` wait on two real runs and write nothing the others write, so they go early and `K10`'s regression kind is built against the tool as it will be. `K9` then converts every line there is — `K2`'s new answers among them — once. `K10`'s acts are born as digests. Only then does the harness move: its read contract in `K3`, what it asks of agents in `K11`, and `X1` last because it touches neither.

## 5. The runtime probes — their design, and what they showed

*The design below is as it was cut, before `K0` built the probes and before they ran; the results follow it. The judged results are committed at [probes/](probes/README.md).*

**Why first.** Three tasks of the first cut were cut on what the runtime does, and nobody has seen it do anything: no stage, and not even the self-test, has run under it. Each probe below is minutes; one waits half an hour by its nature. **None touches a run, a branch or the remote**: a probe's agents run `dev/stabilize-probe` and nothing else, and that script writes only under the scratch root it is handed and refuses one inside the repository. So they run from the main checkout, in the orchestrating session, with no clone and no second session.

**What is invoked.** The `stabilize` workflow, from the repository's root, with `args` as below; `<scratch>` is a directory minted with `mktemp -d` under the session's scratch root. Probe 0 exists today; probes 1 to 4 exist once `K0` has landed.

| | `args` | What its agents do | What it shows | Which task or row the result changes |
|---|---|---|---|---|
| **0** | `{ selfTest: true }` | none is launched | that the runtime accepts the script at all — the build's own first verification, never run | everything: a script the runtime refuses is the first repair |
| **1 — a return that lacks a required field** | `{ probe: 'required', scratch }` | two reviewers (`milestone-code-reviewer`), each under a schema that requires the hash. (a) is told in so many words to leave the field out of its return; (b) is handed a file and its hash on the labelled line and asked for a source pass that drives nothing | (a) what the script is handed when a return lacks a required field: the object without it, an object with the field made up, nothing, or a throw — and after how many tries. (b) whether a reviewer that drives nothing returns the hash unasked | `P4`'s row and the choice between its two designs. If a violation comes back as *nothing*, `required` must **not** be added to any field whose absence should cost one item: it would cost three full runs of a reviewer and a count toward the breaker. If the field is *made up*, `required` is worse than no check. `X1` is untouched by the answer — it adds no `required`. `O2`, if (b) leaves the hash out |
| **2 — a relay at the real run's size** | `{ probe: 'relay', scratch }` | a first step builds four throwaway runs under the scratch root — 1 · 20 · 60 · 120 ledger rows over 2 · 40 · 150 · 300 doors — and relays their paths; then, three times per size, a git step (`build-git`, on the model the harness's git steps use, under the harness's own step prompt) runs that run's `dev/stabilize-step state` and relays its one line, which the harness holds to its hash | how often a line of 3, 25, 75 and 143 KB comes back whole — the inference `R-H1`'s grade rests on, observed | `R-H1`'s row. Whole every time at every size: `R-H1` is the document in the orchestrator's return and nothing more, which `X2` closes. A failure at a size: the row is **due before the opening** — round 1's state reaches that size |
| **3 — a here-document payload at size, hashed** | `{ probe: 'payload', scratch }` | the harness composes a record's batch of 20 · 60 · 120 · 300 entries with the functions a stage uses and hands each to the role that writes it (`build-executor`) under the record prompt's own words — the file goes under the scratch root, and nothing is applied or gated; the agent returns the file's hash line | whether a payload of that many findings reaches the disk as the bytes the harness composed | `P4`'s row. Whole at 300: one stage's record can carry what a first round finds, and that number is declared. A failure: findings must reach the disk through the record script from the agent that made them — design 2 of §6 — and the row is **due before the opening** |
| **4 — a long command held in slices** | `{ probe: 'hold', scratch }` | (a) one preflight agent (`stabilize-preflight`) starts `dev/stabilize-probe hold` for twelve minutes in the background and waits for its last line in slices of under five, as its definition has it wait for the gate. (b) one agent starts the same hold and returns; a **second** agent is asked for its line in slices. A third argument, `seconds`, runs either for the regression set's thirty-five minutes | (a) whether one agent holds a command past the ten-minute tool ceiling inside its turn; (b) whether a process outlives the agent that started it | `T9`'s row — **and round 1 itself**, which runs the regression set from a brief inside the preflight's turn. (a) holds: the brief can work, and later a script reads the result. (a) fails: the brief cannot work, and in round 1 the check runs outside the stage — by the orchestrator, recorded as the human rules; (b) then says whether the harness could drive it instead |

**What it costs.** Probe 0: seconds. Probes 1 to 3: some twenty-five short agents, most of them the git steps' model, about twenty minutes. Probe 4: twelve minutes of waiting for each arm, thirty-five where the full length is asked. **About an hour of wall time; tokens not estimated.**

**What they do not show.** How an agent *dies* — nothing here kills one. Two agents on one tree at once. A stage. And probe 1(a) shows what happens when an agent is *told* to leave a field out, which is not the same as one forgetting it; (b) is the nearer case, and is one sample.

### What the probes showed — run 2026-10-07, at `1d21024e`

| Probe | Result | What it settles | What it moves |
|---|---|---|---|
| **0 — the self-test under the runtime** | passed — and counted 373 checks when invoked by name, 421 by `scriptPath`, after `K0` had landed in the same session | the runtime takes the script. **And a workflow invoked by its name runs the copy the session loaded first** | every invocation in a session that has edited the harness goes by `scriptPath`: a sentence of `K8`, a line of the canary's steps, a row (§7) |
| **1 — `required`** ([required.json](probes/required.json)) | (a) a return that omits a required field **throws**, three tries of three — nothing partial comes back. (b) a reviewer on a pass that drives nothing **returned the hash** it was handed | `required` buys nothing a stage wants: an omitted field would cost three whole runs of the agent and a count toward the breaker. One sample says the reviewer returns the hash unasked | **`X1` is decided — fields stay optional, an absent one voids its item — and is core.** `P4`'s row: neither design may add a `required` to a field whose absence should cost one item |
| **2 — `relay`** ([relay.json](probes/relay.json)) | **never whole**, twelve of twelve. 3 KB and 26 KB: altered, every time. 81 KB: missing twice, altered once. 159 KB: missing three times | two causes, by a diff of the smallest case against what was sent. (i) the tool prints what is not ASCII as `\uXXXX`, the agent's structured return decodes it, and the byte hash fails — **at every size**, whenever a record holds a dash, an arrow or a typographic quote. (ii) from about 80 KB the line does not come back at all: the output is too large to be handed to the agent whole, a safety classifier stopped two copies, and the other agents declined to return a line that was not the command's | **`R-H1` is observed, and is not a matter of size: the first state read of any real stage fails**, and so does any other line that carries a subject or a refusal's prose. `P3` leaves the rows and is core: `K9`, `K3` |
| **3 — `payload`** ([payload.json](probes/payload.json)) | **whole**, four of four, 8 KB to 120 KB at 300 entries | the write direction needs no change | `P4`'s second design loses the one reason that would have made it due before the opening. One stage's record carries 300 findings; the bound is declared at that number |
| **4 — `hold`** ([hold.json](probes/hold.json)) | (a) one agent **held** 2,100 s inside its turn. (b) the process **outlived its starter**: 22 reads by separate agents, the last `done` — at 26 agents and about 835,000 tokens for the one wait | both ways hold a command of the regression set's length. One agent asking again is the cheap form; a read per agent is not a design. That the command outlives its starter is what lets a wait be taken over | the first fork (§8) is settled: `K10`, `K11` |
| **— a permission prompt** | the hold probe's agent, told to start a command *in the background* and wait *in slices*, composed a waiting shell that asked the human for permission | **the human's ruling, 2026-10-07:** *"Hold used an script bash command that needed my permission that should not happen because then it can not run autonomous there should be tooling for this."* The preflight holds the gate the same way and would hold the regression set the same way; the committed settings have no allow rule for any tool of the workflow | `K10`, `K11`, the fence, `O7`, `O5` |
| **— the reviewer of 1(b)** | four LOW findings and a lead on `dev/stabilize-probe` itself (`SP-1`–`SP-4`, `SP-L1`; [probes/README.md](probes/README.md)) | — | rows (§7). `SP-2`'s class — only the root is resolved, so a link beneath it carries writes elsewhere — is in `K7`'s and `K10`'s *done when* |

**What they leave unsettled.** Whether a line that is pure ASCII and a few kilobytes long is relayed whole — every failure seen is explained, and no such line was sent: **the relay probe runs again on the digest, after `K9` and before `K3` is built on it.** Whether the allow rules stop the prompt, and whether a batch written with the file tool arrives as a here-document did: `hold` and `payload` again, after `K11` and `O5`. How an agent that *dies* reaches the script: nothing here killed one; it is the canary's one plant. And **the size of an agent's own return**: a structured return of 26 KB arrived, of 81 KB one in three — and a triage's return grows with the findings it is handed. Nothing measured one (§7, the row).

## 6. The ruling on scale — (B) — and what follows from it

**The ruling (the human's, 2026-10-07, relayed by the orchestrator).** The core now; the runtime probes; a small canary that only shows one `test` stage running end to end with real agents; then the run's opening. Round 1 of the real run — which stops after the round by his own ruling — is the tuning round, and everything else from the two reviews and the plan review becomes recorded rows, fixed between rounds. Later test rounds can be used to improve the workflow; it does not have to be fully automated at the beginning.

**What was not chosen — (A), in short.** The whole plan as first cut and then revised: `P3` and `P4` rebuilt as classes before any real stage, a canary at the real run's shape — the real delta, a ledger at full size, a plant per failure — a third review, and only then the opening. *Why not, as far as this plan can say it:* it was four or five more tasks, three of them large, a canary of a day's wall time whose opening had no author and whose plants masked each other (`B5`, `B9`), and the last review the bound of three cycles allows — all of it spent before the workflow had run once on anything real. What (A) would have bought is that the failures of §6's table were closed before round 1 instead of met in it. Nothing of (A) is lost: each of its parts is a row of §7, and what the plan review found about each is kept below.

### The small canary

**What it is.** One `test` invocation in a clone of this repository with a bare remote of its own, from the stage's first step to its pushed record — then the same arguments once more, to see the answer of an invocation that has nothing to do. It plants one thing, and only where the session allows it (below).

**Its opening — synthetic, written by `K7`'s script, never the human-led opening.** The smallest test set that still has **one item of each kind the real round 1 will have**. The real opening is not written yet, so the kinds are read off the exit rule's instruments (`DECISIONS.md` → *2026-10-04 — The exit rule, revised*): the review rows → one `review-row` on one door; the trial arms → one `trial-arm`; the regression set → one scripted check, a held command (`K10`, `K11`); and the candidate's gate, which every stage holds. The port rehearsal has no instrument, and so no item. `dev/stabilize-canary` takes the kinds as an argument with these four as its default: **if the real opening is to hold an audit kind, the orchestrator adds it** — that is the one thing about this opening that depends on the real one. Beside the items: the four clauses, the stop mode `every-round`, the previous release and its commit, **one seeded ledger row that is not triaged**, so that triage and one verifier run, and an invocation whose `scope` names the one door, so that the scope step derives nothing over the real delta.

**What it must show.**

1. **The stage reaches its record and pushes it, with real agents**: the attempt's marker, the scope, the instruments, triage, a verifier, the record's own gate, the one commit, the vetted push to the clone's remote, the state read back — and the second invocation answering from that state.
2. **The preflight building a commit that is not the one checked out**: in every stage, the previous release's binary, from its commit, by `git archive`, with nothing run on the tree; and, where 4 below leaves a triage unfinished, the candidate itself — the lap that finishes a triage builds the commit the round tested, which the record's commit has by then moved the branch past.
3. **A reviewer asserting the binary's hash**: the `review-row`'s source pass drives nothing, and its return carries the hash or does not — with `O2` made, and with or without `X1`.
4. **How an agent really ends.** The one plant: **whoever runs the session stops the seeded row's verifier from outside, each time it is tried.** That shows what the runtime hands the script for an agent that is gone and after how many tries, whether such an agent writes its report afterwards, and it leaves the finding unverified — so `next` is `triage`, and the invocation after it is the lap of 2. *If the session cannot stop a subagent* — a headless one cannot — this is not shown, 2's second half is shown only if a triage happens to be left unfinished, and both stay rows.
5. **The held commands with real agents**: the two builds, the candidate's gate, the regression set and the record's gate, each started by one act and waited for by one agent that asks again — **and no permission prompt anywhere in the stage**, which is what the human's ruling asks.
6. In passing, because a stage cannot run without them: that the runtime's concurrency lets a scope step and a preflight run side by side; a trial arm's three steps against the ten-minute tool ceiling; a role's own commands — a reviewer's rig, a driver's on the binary — under the session's permission mode, which no fence holds.

**What it deliberately does not show.** A state or a payload at the real run's size — the probes measured those. A push that fails, a candidate whose gate is red, a report the script refuses, a second triage pass, two agents that die, a stage killed as a whole. A ruling, a go, a re-run. The check that reads CI, which is void in a clone. A cross-model pass. The real remote's rules, the real denylist, the real checkout's permission state. Any round after the first. **Each of those is first met in round 1, or is a row.**

**Who does what.** `K7`'s script builds the clone and the opening, from this checkout, naming the clone by its absolute path. **Every invocation is by `scriptPath`, the clone's own harness** — a workflow invoked by its name runs the copy the session loaded first (§5). **Its session stands in the clone**: a stage runs wherever its session stands (`R-L8`), so either the human starts a session there with a prompt the orchestrator hands him, or the orchestrator launches a headless one there and reads its results from files under the canary's root — whether the workflow runs headless is not known, and is a minute to find out. Afterwards the orchestrator writes what it showed as a record on this branch, aggregated, with no transcript; nothing of the clone's tree comes back.

**How it is kept from this repository's remote.** The clone's one remote is a bare repository under the canary's root, and `dev/stabilize-canary check` asserts it before the session starts. The canary's slug and loop branch exist in the clone only — the setup refuses otherwise — so a stage launched from the wrong directory halts at its first step with nothing written (driven for the first cut, §1), and `K2` pins that order with a test. `gh` resolves no repository from a local remote.

**What it costs.** About two to two and a half hours of wall time: the two builds, the gate and the regression set are seventy to eighty minutes between them, then the instruments, triage, and the record's gate. Some ten agents. Tokens not estimated. Without the trial arm and the regression item it is about an hour and proves less of what round 1 does first; the orchestrator may cut either, and says so in its record.

### What a round-1 `test` stage can still do wrong, with the core landed — and what each costs

| What happens | Why it is still there | What it costs |
|---|---|---|
| a relayed line fails its hash | observed: with the harness as it stands, **every** state read fails (§5) | **closed by `K9` and `K3`** — and shown closed only by the relay probe run again on the digest. If that run is not whole, no stage can start, and it is back with the human |
| the state document comes back to the main session whole | `R-H1`, the other half | closed by `K3` |
| a reviewer returns no hash; a verifier confirms and leaves out a field | `R-H2` | closed by `X1`, which is core since the probes: the item is void, or the finding unverified, and the stage records |
| a second agent of one invocation dies | `R-M2` | a halt after the instruments: nothing recorded, **the instruments are run again**, an attempt counted. `X3` closes it |
| a second triage pass keys what was left open to a confirmed finding | `R-M4` | a halt at the record, the instruments again — **and the next attempt can do the same**, after which the stage is the human's. `X4` closes it |
| the preflight or the scope step left no report | `R-M3` | the same halt, found after the instruments instead of before. `X5` closes it |
| one finding's text is refused by the record script — a term of the denylist, a cell its reader refuses | `B4`, `B5`, `R12` | a halt at the record, the instruments again, and it too can repeat. No optional task closes it: its fix is `P4`'s second design |
| triage itself fails — no result, a halt, counts that do not balance | `B4` | the same; `O1` removes the one cause that was a definition's |
| a record cannot be committed before its round has a gate, under a red candidate | `T5` is a row (`R8`, `B1`) | only with a red candidate and two halted attempts; the way out is by hand |
| a triage's return is too large to arrive | the probes: a payload of 120 KB is written whole; **a return of 81 KB came back one time in three** — and a triage's return grows with what it is handed | a halt at triage, the instruments again, and it repeats for as long as triage is handed as much. It bites only if the opening seeds far more than a hundred untriaged rows; what avoids it without a redesign is triage in portions |
| **a check's verdict is wrong** — the preflight reads CI's result, or the tarball install's, or says the trial image is verified, and returns another word | `K10` and `K11` take the gate and the regression set out of an agent's hands; these three stay in them (§4) | **a false cell on record**, for an item whose check is one of the three. Until each is a held command: at the round's stop the orchestrator has a subagent read the evidence the preflight's report names — a check by hand, and the row says so |
| **an item is never selected because its door is mistyped** | it cannot be held (`B10`) | a clause green without that item. `K4` names it in the return of every stage; whether anyone reads the name is not the script's |

**A probe that fails moves its row — and one did.** The relay probe showed that round 1 could not have read its own state, so `R-H1`'s row left *between rounds 1 and 2* and is `K9` and `K3`; the payload probe held, so nothing else moved for that reason. The human's ruling on the permission prompt moved the scripted check the same way.

**The two sentences the ruling rests on.**

- ***Never an unscanned push* — true once `K1` and `K2` have landed, for every push the step tool makes**, and the step tool makes every push of a stage. It would be false if: somebody pushes the loop branch by hand (CI's scan is hard, and runs after); `vet`'s scanners were not the two CI runs (`K1` uses the function the script's writers already call, which runs those two); a scanner that cannot run were read as a pass (`K1` has the opposite as a named test). It says nothing of a commit outside the run's directory: the tool does not push one.
- ***Never a false record* — not true without two exceptions.** What the script *derives* is sound for round 1 and what follows it without a fix: review A killed eighteen mutants on it, and `K4` closes the two inputs that could still be bent. **What is handed to the script can be wrong**: the verdict of a check that is not a held command — CI's, the tarball's, the trial image's — which an agent reads; and a door's name, which nothing can hold — the last two rows above. Beside them stand an agent's own judgments — a verifier's verdict, a key triage gives — which no plan of this kind reaches, and the first review's `F2`, unreachable for as long as `fix` refuses to start.

**So what the ruling buys and what it pays.** Everything in the table that is still open, but its last two rows, costs at most *a late halt and a second run of the instruments* — hours, a counted attempt, and after two the human's grant. Round 1 stops after the round whatever happens, so each such halt is seen by the human before anything is built on it. The last two rows are the price: one is narrowed to three checks and held by a check by hand; the other is declared.

### What each row's fix must hold when it is taken up — the plan review's findings, kept

- **`P3`** is core since the probes (§4). What the plan review found about it is in `K9`'s and `K3`'s cut: every line of the step tool and not the state's alone; the doors nobody reaches as a count in the digest and a name at the close.
- **`P4`** (`T6`). **Two designs, and this plan chooses neither:**
  - **Design 1 — the table, and the rule with its real exceptions.** One table of role × ending that the code consults and the simulation enumerates. Once an instrument is launched the stage reaches its record **except**: any ending of triage; a batch the record script refuses for one entry's text; a relay or a tool step of the record that fails; and — unless the table makes it a void — a hash that is returned and wrong. *Cost:* one large task on the harness and its simulation; `X1`, `X3`, `X4`, `X5` are four of its cells, built early. *It leaves:* findings living only in returns until the batch, so each exception still loses the stage's instruments, and two of them fail again on the next attempt.
  - **Design 2 — findings reach the disk through the record script, from the agent that made them.** A reviewer, triage and a verifier each write what they found as it is found; the script scans it at that write, where its author can still reword it; the record's batch names what is staged and carries none of it. A triage that fails then leaves the instruments' findings on disk, the next invocation finishes the triage and runs no instrument, and the payload no longer grows with the findings. *Cost:* new writers in the record script — in the file and the table the writers' fork is about — every reporting definition edited (the orchestrator's steps), the harness's triage rebuilt, the simulation's scenarios rewritten: two large tasks and a review of its own. *It leaves:* the endings table still wanted, and small.
  - *What decides, after the probes:* a payload at size does arrive, so design 2 is not forced by the write; a schema's `required` buys nothing, so neither design may lean on it; **an agent's return at size is the open measurement** — it is what design 2 removes and design 1 does not — and round 1 shows the rest. In either design a failed relay, a thrown call, a report nobody launched and an executor that halts each need a row — the first cut claimed them closed and gave them none.
- **`T5`** (`R8`). As first cut it cannot be built (`B1`). It holds: where a record's round has no gate on record, the record step gates the tree before it applies its batch and **hands that file to the commit step as what it is held to**; no `gate.md` is written by a record that tests nothing, so the round's gate stays the gate of the candidate it tests; its *done when* drives the granted stage through its own record — `B1`'s block ending `recorded` twice. No rule on counts. The lead *a round's gate and its candidate are not held to one commit* is decided with it.
- **`T9`** is settled by probe 4 and the human's ruling, and is `K10` and `K11` (§4): the tool starts the command, one agent waits by asking again, the tool reads the verdict; the verdict file is a file of the record, which is where the list's hash and the exclusions go. What is left of it as rows: CI's result, the tarball install and the trial image as held commands of their own.
- **`T11`** — what a run shows about numbers. **It may hold numbers and nothing else**: how many retries, the breaker's threshold, a slice's length, a size that is declared (`B6`). A change of design is a task of its own, with its own review.
- **A canary at the real run's shape**, if one is ever wanted: plants that cannot mask each other; a ledger whose *untriaged* rows are at the real size; a dead agent or two, and a record gate that is newly red with the pending batch after it; an honest count of invocations; an author for its opening; and its gaps said (`B5`, `B9`).

## 7. Every finding — its home, or its row

**This is the table the run's ledger can be seeded from at the opening.** A finding that the core closes has its task. Every other finding is a **row**: what it costs if it happens in round 1, and when it is due. A row's key is `wf-` and the finding's id in lower case (`wf-r-h1`, `wf-b4`); its repro is the block in the report that found it. `K8` writes each row that has a due date into `decisions-pending.md`.

*Due:* **core** — a task of §4 · **before a stage** — an orchestrator's step or `K8`, before any stage runs with real agents · **optional** — one of `X3`–`X5`, before the opening if it is kept, else *1→2* · **1→2** — between rounds 1 and 2 of the first run · **`fix`** — with the `fix` half's repair (§9) · **later** — a later release's run · **declared** — a bound of §10, revisited when a run shows it bites.

*In round 1:* **halt** — a late halt, the stage's instruments run again, an attempt counted, and after two the human's grant. Anything worse than that is written out.

**Review A — the record script and the state machine**

| | Finding | Home | In round 1 | Due |
|---|---|---|---|---|
| `R1` | unscanned text on the remote | `K1` | — | core |
| `R2` | a batch that outlives its commit | `K2`, `K3` | — | core |
| `R3` | a failed commit leaves a staged tree | `K2` | — | core |
| `R4` | the temporaries of a killed write | `K2` | — | core |
| `R5` | an item whose clause the census lacks | `K4` | — | core |
| `R6` | `item-set` replaces an item that has results | `K4` | — | core |
| `R7` | a record committed and not pushed | `K2`, `K3` | — | core |
| `R8` | a record with no gate on record, under a red candidate | row (`T5`) | only under a red candidate and after two halted attempts: the human's grant cannot be recorded, and the way out is a batch composed by hand | 1→2 |
| `R9` | the script takes the whole run while the opening is not done | row; declared | nothing in a stage — a direct caller's | `fix` |
| `R10` | after a dropped round only an agent decides that no hunt runs | row | unreachable while `fix` refuses | `fix` |
| `R11` | at a clause that is the human's the position hands `test` round R+1 | declared | only if the orchestrator answers `rule` with `test`: a round that no bound counts | later |
| `R12` | two inputs taken, or refused in the wrong class | row; declared | halt, and it can repeat — reached by an agent's text of exactly `-`; from a direct call the state stops reading, loudly | `fix` |
| `R13` | the subject of a record commit is not scanned | `K1` | — | core |
| `R14` | the writers' table is a census of names | row | nothing | `fix` |
| `R15` | what the header and the doc claim and the code does not do | each bullet by the task that makes it true; the rest `K8` | an orchestrator misled | before a stage |
| `R16` | what no arm of the suites drives | bullet 1: `K1`, `K2`; bullet 2 (`S1`, `S2`): row; bullet 3: `K8` names the test that holds the go | nothing | `fix` |
| lead | a round's gate and its candidate are not held to one commit | row, decided with `T5` | nothing in a stage | 1→2 |
| lead | which reporters an attempt launched is in no committed table | declared | nothing: once committed, git holds the reports | later |

**Review B — the harness, the agents, the gate, the regression tool, the doc**

| | Finding | Home | In round 1 | Due |
|---|---|---|---|---|
| `R-H1` | the state document relayed whole — **observed: every line with a character outside ASCII fails, at any size** | `K9`, `K3`; an agent's own return at size: row (`P4`) | — once the relay probe is whole on the digest | core; 1→2 |
| `R-H2` | one field left out halts the stage | `X1` — core since the probes; `O2`, `O6`; the table: row (`P4`) | — | core; before a stage; 1→2 |
| `R-M1` | after a rejected push the same invocation pushes nothing | `K2`, `K3`; its four sites in `runFix`: row | — | core; `fix` |
| `R-M2` | two dead reporters halt the stage | `X3`; row (`P4`) | halt | optional; 1→2 |
| `R-M3` | the stood-on reporters checked late | `X5`; row (`P4`) | halt | optional; 1→2 |
| `R-M4` | a second pass keyed to a confirmed finding | `X4`; row (`P4`) | halt, **and the next attempt can halt the same way** | optional; 1→2 |
| `R-M5` | the triage definition against the arithmetic | `O1`; the arithmetic stays | — | before a stage |
| `R-M6` | green over a list no commit holds | `K5`; *who adds a row*: declared, and row | — | core; `fix` |
| `R-M7` | one in a hundred drops out | `K5`, `K6` | — | core |
| `R-M8` | a check's verdict is an agent's reading | the gate and the regression set: `K10`, `K11`; CI's result, the tarball install, the trial image: row, with a check by hand at the round's stop | for those three, **worse than a halt: a false cell on record** — a word the evidence does not bear | core; 1→2 |
| `R-L1` | `stopAfter` over a pending batch commits and pushes | `K3` | — | core |
| `R-L2` | the candidate's gate runs on a tree with the run's pending files | declared; the canary and round 1 read it | a red of the run's own files on record as the candidate's, and then accepted of every record of the round | later — 1→2 if it shows |
| `R-L3` | the previous release's binary known by a version string | `K10`, `K11`: a build is a held command, and its hash, version and path are the tool's — if the build kind is kept; else row | — | core |
| `R-L4` | a door no item reaches is known from a return alone | `K4` names the items nobody selected; `K9` counts the doors no item reaches, in the digest | — | core |
| `R-L5` | a one-command step wrapped in a retry that says look around | `record`: `K3`; `begin`: declared | a retried `begin` counts the attempt | core; later |
| `R-L6` | the self-test passes a harness that runs the cross-model pass everywhere | `K8`, a sentence; declared | nothing: the simulation catches it | before a stage |
| `R-L7` | `dev/gate --quick` prints `GATE: PASS` | `K10`: the tool reads a gate's two lines and no agent waits for *a verdict line*; the line itself: declared | nothing: every script that reads a gate refuses a pre-check's output | core; later |
| `R-L8` | nothing in an invocation names the repository | declared; `K7`'s guards; every invocation by `scriptPath` | a stage launched in the wrong repository halts at its first step, nothing written | later |
| `R-L9` | three definitions still name the `probe` step | `O4` | — | before a stage |
| `R-L10` | the doc names three of six rulings about the run | `K8` | — | before a stage |
| the doc's sixteen sentences | false, or lacking what the code does | the task that changes the code under each; the rest, and the six things *an orchestrator would get wrong*, `K8` | an orchestrator misled | before a stage |
| lead | a report written for an attempt nobody began counts as one and is committed | declared | an attempt counted before a stage ran | later |
| lead | the gate comparison holds names, never counts | declared — the first cut's rule on counts is dropped | none found: nothing was shown to cut a keep-going gate short | later |
| lead | a commit made on the loop branch while a stage runs | `K2` | — | core |
| lead | a human's scope on a second attempt is not applied | declared | the round's scope stands, and the return says so in one word | later |
| lead | agent-written text in other agents' prompts | declared | none reaches a shell | later |
| lead | resume by run id after a kill | declared: its replay is unexamined | a fresh invocation is what is known to work | later |

**The first review's findings that are still open**

| | Finding | Home | In round 1 | Due |
|---|---|---|---|---|
| `F2`, `S1`, `S2` | a fix on the ledger with no counted cycle; the suite certifies it | row | unreachable while `fix` refuses; `test` is sound for round 1 and what follows it without a fix | `fix` |
| `F10` | `admitted` is not inherited | declared (it is, at line 473) | the safe side | later |
| `F13` | the header claims what the code does not do | `K8`, with `R15` | — | before a stage |
| `H1`–`H4`, `M1`, `M12` | the landing, the part, the triage inside a running `fix` | row | unreachable behind the refusal | `fix` |
| `H5` | `landed-before` reports landed, unchecked and unpushed | row — an act of `P1`'s table; its `test` member is `R-M1`, `K2` | — | `fix` |
| `M8` | the path rule's limits | row — the last task of the whole repair | a merge on the loop branch that is not a round's halts every later `test`; none is due in round 1 | `fix` |
| `M9` (in part) | two dead agents | `X3`; row | halt | optional; 1→2 |
| `L2` (in part) | *the same args* after a push halt | `K3` | — | core |
| `L5` (in part) | `raise` beside the recorded `cycles` | row | — | `fix` |
| `L8` | the `probe` sentence | `O4` for the three a run launches; the other five keep their row | — | before a stage |
| `L12` | a run slug ending `-r<N>` | declared (it is) | the first run's slug does not end so | later |
| leads | a new finding keyed as found again; the door text triage returns against the reporter's | declared | round 1 is what shows them; `R-M4` is the member that was driven | 1→2 |
| the doc builder's first item | a stage stopped after its scope step, and the next `git-state` | **closed** — driven (§1) | — | — |

**The plan review's findings**

| | Finding | Home | In round 1 | Due |
|---|---|---|---|---|
| `B1` | the grant's gate collides with the granted stage's | row, with `R8` — `T5`'s mechanics in §6 | as `R8` | 1→2 |
| `B2`, `B3` | the push is the irreversible act; a vet hit has no exit | `K1` | — | core |
| `B4` | findings live only in returns until the batch | row (`P4`, two designs in §6) | halt, for every ending of triage and for one entry's refused text — and those repeat | 1→2 |
| `B5`, `B9` | the first cut's canary: masked plants, no author | not chosen with (A); kept in §6 | — | later |
| `B6` | `T11` as unreviewed code | `T11` holds numbers only | — | 1→2 |
| `B7` | tasks cut on unseen runtime facts | `K0`, the probes — run; §5 | — | done |
| `B8` | *done when* defects | (a), (b): `K2`; (c): `K10` — the verdict file is a file of the record; (d): `O6` | — | core; before a stage |
| `B10` | a mistyped door | `K4` names it; declared | **worse than a halt: a clause green without that item** — seen only by whoever reads the names at the stop | core; declared |
| advisory | an index lock; an owed push under a look; an unpushed tuning commit | `K2`, `K3`; declared | a refusal that names its way out | core |
| advisory | the table keyed so the `fix` half extends it | `K2`; §9, row 1 | — | core |
| advisory | refusal lines and `record`'s red lists are relayed free text | `K9`: every line is a digest, and the prose is in a file it names | — | core |
| advisory | a failed relay, a thrown call, a report nobody launched, an executor that halts have no row | row (`P4`) | halt | 1→2 |
| advisory | `T8` needs two runs and a form for an exclusion row | `K5`, `K6` | — | core |

**What the probes added**

| | Finding | Home | In round 1 | Due |
|---|---|---|---|---|
| probe | a workflow invoked by its name runs the copy the session loaded first | `K8`, a sentence; the canary's steps and `K7`'s printed invocation; row `wf-by-name` | a stage runs a harness that is not the tree's — silently — in any session that edited it | before a stage; later for a guard |
| ruling | a step of a stage asks the human for a permission | `K10`, `K11`, the fence, `O7`, `O5` | a stage that waits on a prompt nobody is watching | core |
| probe | an agent's own return does not arrive at size — 81 KB, one in three | row `wf-return-size`, with `P4`'s second design; triage in portions avoids it | halt at triage, and it repeats | 1→2 — **before the opening if it seeds far more than a hundred untriaged rows** |
| `SP-1` | a failure of `dev/stabilize-probe` that is no refusal prints a traceback and exits 1 | row | nothing: no stage runs that tool | later |
| `SP-2` | only the scratch root is resolved: a link planted beneath it carries writes elsewhere | the class: `K7`, `K10`; the probe tool itself: row | nothing in a stage; a canary's setup is where it would bite, and `K7` holds it | core; later |
| `SP-3` | two verdicts of `required` judged on the last try only | row | nothing | later |
| `SP-4`, `SP-L1` | the header names three of four copied files; a `held` in the microseconds before a hold's record is written | row | nothing | later |
| merge | three rows of `decisions-pending.md` are stale since `b33bf39b`; the regression set's green on record is for the candidate before the eleven fixes | `K8`; `K5`, `K6` | an orchestrator misled; a check that would be red on rows nobody wrote | before a stage; core |
| `K1` | four harness edits owed | `K3` | — | core |

**Refuted: none — of the reviewers' findings, and of the plan review's.**


## 8. The forks as they now stand, and what was buried as settled

**The question on scale is ruled: (B)** (§6). Of the first cut's five forks, under that ruling:

**What is left with the human — four things, none of them a design.**

1. **Where the allow rules live** — new since the probes. His ruling is that no step of a stage may need his permission; `K10` and `K11` make every command a prompt names one plain invocation of a tool under `dev/`, and the fence prints which tools. A rule per tool then has to stand somewhere. *Options:* (a) **the committed settings** (`.claude/settings.json`) — the orchestrator's gated commit on his word; the rules travel into every clone, the canary's among them, and into the session of anyone who clones a public repository — for scripts that repository itself holds and reviews; (b) **his local settings** — he adds the lines himself, nothing is committed, and they do not travel: the canary's clone needs them at the user level or once more. *Recommendation: (a)* — a run that is to be autonomous in a clone is autonomous only if the rules are in the clone, and the fence already holds the list they are written from. Either way it is his: a permission is never an agent's to grant, and `O5` is cut for both.
2. **Whether the core's diff is reviewed before round 1.** The rule of 2026-10-06 is his: a half is used on a real run once its repair *and the re-review of that repair* are recorded. The ruling on scale names no review. *Options:* (a) a focused, independent review — of `K1`–`K4` as first proposed, **and now of `K9`, `K10`, `K3` and `K11` too**, which rebuild every line the harness reads and every command it names: larger than it was, and the third review of this half, the last the bound of three cycles allows; (b) none: round 1 is the tuning round for this too, and the rule is set aside for this half by his word. *Recommendation: (a).*
3. **The bounds of §10, accepted for a public first run.** A declared bound is his by the exit rule. Two are not on the safe side and are named as such: the verdict of a check that is no held command, and a mistyped door.
4. **The canary's session — a request.** A stage runs wherever its session stands, so the canary needs a session that stands in its clone: he starts one there with a prompt the orchestrator hands him — which is also the only kind of session that can stop a subagent from outside, the canary's one plant — or the orchestrator launches a headless one, if a minute's trial shows the workflow runs that way.

*And a drop, whenever it comes up:* any of `X3`–`X5`, and the build as a held command (§4), which he or the orchestrator may drop.

**What falls away under the ruling.**

- **How a scripted check is run and judged** (the first fork). **Settled, by probe 4 and the human's ruling on the permission prompt**: the tool starts it, one agent waits by asking again, the tool reads the verdict (`K10`, `K11`). Of its four options that is the third — the command held by the gate's pattern and judged by a script — with the starting and the waiting taken out of the agent's hands as well.
- **What a record is held to where its round has no gate** (the second). Withdrawn: not the human's. Running the candidate's gate there is his ruling of 2026-10-06 as worded; the mechanics first proposed were wrong (`B1`, driven), and the right ones are with `T5`'s row.
- **The writers' table as the code path, now or with the `fix` half** (the third). The ruling answers it: a row, due with the `fix` half — and never a patch of `R12`'s two instances. `K1`, `K2` and `K4` each add a read or a refusal to that script by hand, which the rebuilt table later takes over.
- **A `root` argument for the harness** (the fourth, as a fork). Rejected, as the plan review confirms: a role agent's shell starts in the session's directory at every call. What is left of it is the request above.
- **A cross-model pass in the canary** (the fifth). The small canary names none; a pass is named when he asks for one.

**Decisions the review found buried as settled** — each said to be the human's or not, with one line of why:

| The decision | Whose | Why |
|---|---|---|
| the canary's size, and its departure from the one-door shape that was approved | **the human's — ruled: (B)** | the build's verification was approved as he ruled it; the small canary is back near that shape |
| what becomes of a finding the canary makes about the product | the orchestrator's, open to overturn — the question in its large form went with (A) | the small canary reviews one real door, and a reviewer may find something real there: it is seeded into the real run's ledger at the opening as a lead, and nothing else of the canary comes back |
| the opening drafted before the canary | fell away with (A) | the small canary's opening is synthetic, written by `K7`'s script; the human-led opening stays after the canary, as he ruled on 2026-10-06 |
| whether the core is reviewed before round 1 | **the human's** — open (the first item above) | the rule of 2026-10-06 is his, and asks for a re-review of a repair |
| accepting the bounds of §10 for a public first run | **the human's** | a declared bound is his by the exit rule, and the record script takes no bound's row that does not say where he ruled it |
| resume by run id, narrowed to *never* | **the human's — and withdrawn** | ruling 9 keeps resume by run id for a killed run. The first cut declared it never used; this revision declares only that its replay is unexamined, and recommends a fresh invocation after a kill until it is shown |
| the automatic push of a record that is owed | not the human's — the planner's, open to overturn | it is the push a stage already makes after every record, made by the next invocation instead; it is restricted to commits that touch only the run's directory, and since `B2` it is vetted first |
| a hash that is returned and wrong halts the stage *as ruling 11 has it* | not the human's — **and the attribution is withdrawn** | ruling 11 says every agent asserts the hash before it drives anything; halting is the builders' choice of 2026-10-06. Whether a wrong hash halts or voids is `P4`'s row to decide — either way what that agent drove is no evidence |
| an agent that returns no hash voids its item | not the human's — `X1` if it is kept, else `P4`'s row | the same |
| three choices of the regression tool's builder overturned | not the human's — reported | each is recorded as a builder's (items 4, 7, 8 of that entry) |

## 9. What the `fix` half's repair owes — the rows this plan adds

1. **Its git acts are acts of `P1`'s table.** `land`, `carry`, `open-round` and the part's revert each get their phases, each phase its predicate from git and its resume step, and the journal names the one that is open. The test that kills reads the table's print and covers them without being rewritten. `landed-before` is a phase that is done, read there (`H5`).
2. **`F2`, with the suite that certifies it.** The three truth-table tests *require* the close over a fix with no counted cycle (review A's mutant `m18`): the repair turns the cell and replaces the oracle that review A cites at `dev_stabilize_record.rs:5838` and `:7239` (`S1`, `S2`) — lines this plan did not read.
3. **A dropped round** is said where a stage reads its work from, the scope step is handed no door list of fixes that never landed, and whether a hunt runs is the script's (`R10`).
4. **The writers' table as the code path, and one grammar per cell** (`R9`, `R12`, `R14`) — a row the ruling on scale puts here.
5. **The audit of a fix diff reads the diff of the regression set's list**: a row nobody ruled is held to nothing else (`R-M6`).
6. **The four sites of `runFix` that say *was not pushed***, and the sentence its halts end with (`R-M1`'s class; the doc's bound at line 469).
7. **`runFix` reads the digest, and its long commands are held.** `K3` keeps it parsing the lines it parses today and no more; what a `fix` stage reads by key — the blockers, the fixes' doors — and its own gates as held commands are its repair's.
8. **Every push of the `fix` stage is vetted by `K1`'s check already** — the round branch's among them, over product code; the `fix` half's repair says what a hit means there, where the commit is a fixer's.
9. **`raise` beside the recorded `cycles`** (`L5`), and **a fixer's fork has no row** — both owed already, restated so that the list is whole.

## 10. Declared, not fixed

*`K8` writes these into the workflow doc with their reach — after they have been put to the human (§8).*

- **A door mistyped in an item's row reads as a door the round did not reach.** The item is never selected, owes nothing, and its clause can be green without it. The state and every stage's return name the in-scope items no tested round selected (`B10`; `K4`).
- **The verdict of a check that is no held command is the preflight's reading** — CI's `ci-ok`, the packaged-tarball install, that the trial image is verified, whether the cross-model tool answers, its environment asserts (`R-M8`, what `K10` and `K11` leave).
- **The fence holds what the harness names, and no more.** What a role runs of its own craft — a reviewer's rig, a driver's commands on the binary, the preflight's read of CI, the tarball, the trial image — is run under the session's permission mode, and a prompt can appear there.
- **A workflow invoked by its name runs the copy the session loaded first.** Every invocation is by `scriptPath`; nothing in the harness can know that it is stale.
- At a clause that is the human's, the position still hands a `test` stage the next round; `next` does not ask for it (`R11`).
- The candidate's gate runs on a tree that holds the attempt's marker and may hold the round's scope and its report (`R-L2`).
- A stage runs in whatever repository its session stands in; a canary's slug and loop branch exist in its clone only, and its setup names the clone by its absolute path (`R-L8`).
- The step tool pushes no commit that is not a record's: a tuning commit on the loop branch is its author's to push, and a stage refuses to start over one that is not.
- git's own lock is named and never removed; what a kill inside a git child leaves beyond it is the human's.
- A push made by hand is not vetted by anything but CI.
- The self-test checks the script's tables; what a stage does is the simulation's to show (`R-L6`).
- `dev/gate --quick` prints a gate's verdict line and no totals line (`R-L7`); no agent reads either line any more once `K11` has landed.
- A step that begins an attempt, run twice, counts the attempt (`R-L5`, for `begin`).
- While the opening is not done a direct caller can write a whole run; a writer can take a cell its reader refuses; a writer's place is written by hand (`R9`, `R12`, `R14`) — until the `fix` half's repair.
- A report written for an attempt nobody began counts as an attempt and is committed with the next record; which reporters an attempt launched is in no committed table; a scope is written once, and a human's scope on a second attempt is not applied.
- A row of the regression set's list is held to a pointer that exists, and to nothing that says who ruled it.
- A record's gate is held to the candidate's by the names of what is red, never by how many tests ran.
- One stage's record carries 300 findings, as the payload probe measured, and no number above that is known. **An agent's own return is not measured at all**: of a relayed line of 81 KB one in three arrived, and a triage's return grows with what it is handed.
- `dev/stabilize-probe` has four LOW findings of its own (`SP-1`–`SP-4`); no stage runs it.
- The replay of a killed run resumed by its run id is unexamined; a fresh invocation after a kill is what is known to work.
- What keeps a real `test` stage from starting is the record alone: the harness refuses `fix` and not `test`.
- Agent-written text reaches other agents' prompts and no shell.

## 11. What this plan did not do

- **It built nothing and ran no agent.** Every *done when* is a test that does not exist yet; its name is the plan's proposal, and a builder that finds a better seam keeps the fact and may change the name.
- **It did not measure the tasks.** The sizes are a reading of the code; from here the core is ten tasks and at least eleven full gates — four of the ten are large — each optional task one more, and `K5` and `K6` wait on real runs of thirty-five minutes.
- **It did not run the probes**; the orchestrating session did, and §5 is that session's results as handed over and as the four judged files hold them. The diff that found the relay's first cause, the token count of the hold's second arm and the count of the self-test by name are the orchestrator's observations, taken as reported; of the workflow journals this plan read the reviewer's findings and the words of the agents that returned no line.
- **It designed the digest from the harness's reads of the state, by a search of its source for them** — a read that goes through a helper under another name is not in §4's table, and `K9`'s builder is to hold the table against the code, not the code against the table.
- **It did not verify the settings' rule syntax, nor that a rule stops the prompt the hold probe met**: `O5` is followed by the probe that shows it.
- **It did not recut what the ruling makes a row**: §6 says what each fix must hold and designs none. It did not verify the small canary: that a session can stop a subagent from outside, that an invocation's `scope` bounds a round, and that a trial arm fits a stage are each said to be unknown where they are used.
- **It read the harness in the parts the findings cite**, about a fifth of it, and the record script likewise. A cause that lives only in what was not read is not in §3.
- **Three LOW findings were not checked** (`R16`, `R-L2`, `R-L6`), one traced case of `R-M4` was not driven, and of the plan review's `B5` the count of invocations was taken as reported.
