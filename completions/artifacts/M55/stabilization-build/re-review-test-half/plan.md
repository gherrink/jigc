# The second repair of the `test` half — the plan

*Written 2026-10-07 by a planning agent, on `fix/rc24-tier1` at `cbb3d736`, and **revised the same day at `a7d15955`** after an independent review of it ([plan-review.md](plan-review.md): build after the named changes, ten blocking findings — §2 is the disposition of each). It rests on both re-review reports beside this file ([records-state.md](records-state.md), ids `R1`–`R16`; [harness-doc.md](harness-doc.md), ids `R-H…`, `R-M…`, `R-L…`), the first review's two reports and the first repair's plan one directory up, and the code at the tip — the scripts, the harness and the suites are the same bytes at `a7d15955` as at `cbb3d736`. **Nothing here is built, and nothing here is decided that is the human's.** `F…`/`S…`, `H…`/`M…`/`L…` and `C0`–`C6` are the first review's and the first plan's ids; `B1`–`B10` are the plan review's; a line number of the workflow doc is its line at `cbb3d736`. The first cut's tasks `T1`–`T4` and `T8` are `K2`, `K3`, `K1`, `K4` and `K5` with `K6` here, and the plan review speaks of them by their first ids. This plan's own: `P1`–`P6` (the structural changes), `K0`–`K6` (the core's tasks, a builder's), `X1`–`X5` (five small optional tasks), `T5`–`T12` (the tasks of the first cut, which the ruling makes rows), `O1`–`O6` (the orchestrating session's own steps).*

**How this revision is laid out, and the ruling it was finished under.** The first cut was ten tasks, a canary at the real run's shape and a third review. Its review found that four of the tasks, the canary and three of the five forks could not be built as cut — and that three tasks were cut on facts about the runtime that minutes would show. The revision put one question to the human: the whole plan, or its core. **He ruled on it while the revision was written (2026-10-07, relayed by the orchestrator): (B).** The core now; the runtime probes; a small canary that only shows one `test` stage running end to end with real agents; then the run's opening. **Round 1 of the real run is the tuning round**, and everything else from the two reviews and the plan review becomes recorded rows, fixed between rounds. In his words as relayed: later test rounds — a 1.1, for instance — can be used to improve the workflow; it does not have to be fully automated right at the beginning.

So the plan is:

- **The core (§4), complete and ready to build**: the record's commit as one repeatable, reconciling act (`P1`); the record script's checks at the commit boundary *and at the push* (`P2`); the derivation's inputs held (`P5`); the regression tool's green resting on committed facts (`P6`) — with the task that builds the runtime probes first, the small canary's setup and the doc last, and five small optional tasks after it.
- **The runtime probes (§5)**, which the orchestrating session runs as soon as their task has landed: about an hour, most of it waiting.
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
| `R-H1` HIGH | the state document is relayed whole, 75–150 KB at the real run's size | measured (four sizes) and the return driven | **confirmed as measured** — 3,179 · 25,324 · 74,956 · 142,906 bytes (seven more than the report's, the rig's path); a stage's return over a ledger of sixty rows is 38,609 bytes, 37,686 of them `state`. **Not shown by anyone:** that a real agent's relay of such a line fails its hash — that is the canary's, and the grade rests on it |
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

- `R-H1`'s grade rests on an inference (a relay of that size fails). What is certain without it: the document goes back to the main session whole, where ruling 17 has the main session only orchestrate.
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
| `B8` | three *done when* and write-set defects, and one of `T6` | (a) review A's own table: a kill before the journal ends at the state before the batch; (b) `git_state`, `record` and `push_branch` read: the branch is asked first today, and nothing pins it; (c) `dev/stabilize-record:2772` (a green row's detail is `-`), `:1021` (no fact of a round for a list's hash); (d) `finding-verifier.md:19,27` and `stabilize.js:2026` | **accepted — (d) in part** | (a), (b): `K2`'s *done when* (§4). (c): `T9`'s row needs the record script. (d): the definition does return `regression: false` for a door the previous release lacks; what it need not return is the previous binary's hash, which the harness demands with every `confirmed` — so the fix is the review's, a step of the orchestrator's on the verifier's definition (`O6`), and the first cut's *no regression fact → unverified* goes with `P4` |
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

Thirty-six findings and the leads come down to six causes. Four are one sentence of the first repair that was built for less than its class: *an act is repeatable and every invocation reconciles first* (`C2`, planned for the `fix` half), *nothing decisive is only in an agent's hands* (`C6`, built for the fork alone), *every ending of every agent has one row* (the decision of 2026-10-06, built for one dead agent), and *a write is taken only where the state asks for it* (`C5`, built as a census of names). `P1`, `P2`, `P5` and `P6` are the core and are stated whole; `P3` and `P4` are stated as far as their cause and their class: under the ruling they are rows, fixed between rounds.

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

**What it closes.** `R-M6`, `R-M7`. Its caller, `R-M8`, is `T9`'s row.

**The change.** The list is a path *in the candidate's commit*; two commits that are one are refused; a test that fails on its own binary is excluded only by a row that names it and says why — the tolerance of one in a hundred goes; and the previous release is built from a clone with no checkout, which is a repository and no registration in this one's, so that the eleven tests that ask git about their tree are measured again. **An exclusion row** is a row of the same list and of its four fields, with a pointer of a kind of its own that resolves to the committed record of the run that showed the test failing on its own binary; a row for a test that passes there is reported as stale, as the tool reports a stale row today.

**It takes two real runs, of about thirty-five minutes each.** The first, with no exclusion row, names the tests that fail on their own binary — eleven, fewer, or none. Only then can their rows be written; and only a run on a commit that *holds* those rows can show that what is excluded is exactly what is listed. So the change is two tasks, and the second exists only if the first names a test.

**Alone, as a group.** These share nothing with the other causes but the caller. **Three builder's choices are overturned by it** — the list as a file the tool is handed, the tolerance, the archive — each recorded as a builder's in `DECISIONS.md` → *The regression set's first part*, items 4, 7 and 8. None is a ruling of the human's.

### P3 and P4 — rows; their causes and their classes

**`P3` — nothing a script can read or decide goes through an agent's hands** (`C6`, extended). The harness has no filesystem, so what it computes on comes through an agent: the state document inside a relayed line, a check's verdict in the preflight's reading of a brief, three facts in a return and nowhere else. It closes `R-H1` (HIGH), `R-M8`, `R-L3`, `R-L4`, `R-L7`'s wording. Its class is *a fact computable from the disk or from git that the harness takes from an agent*, and its boundary is an agent's own judgment.

**`P4` — every way an agent can end is one row of one table.** *How an agent can end* is prose in two places and code in twenty. It closes `R-H2` (HIGH), `R-M2`, `R-M3`, `R-M4`, the first review's `M9` in part. Its class is the cells of that table — and, as the plan review showed, the first cut's headline rule over it was not what its task built.

Under the ruling both are rows (§7); §6 keeps what the plan review found about each, so that the task that takes a row up starts from it.

### The findings that are genuinely alone

`R8` (what a record is held to where its round has no gate — `T5`'s row) · `R10` (the `fix` half's) · `R11`, `R-L2` and `R-L7` (declared) · `R-M5` (one sentence of a definition that asks for what the harness, rightly, refuses — `O1`) · `R-L6` and `R-L10` (a sentence of the doc each) · `R-L8` (declared; §8) · `R-L9` (`O4`).

## 4. The core — ready to build

**Every task is one `build-executor` behind the full `dev/gate`, on the tree of its own commit; they run in the order given, one at a time.** The write sets show why: `dev/stabilize-step` is written by two of them, the record script by three, the harness by four and by every optional task, and each appends to `DECISIONS.md` and corrects the sentences of the workflow doc that it makes false. `K5` and `K6` write none of the three scripts; they could run beside another task only if the orchestrator took both logs' entries itself.

**Write sets.** `REC` = `dev/stabilize-record`, `tooling-tests/dev_stabilize_record.rs` · `STEP` = `dev/stabilize-step`, `tooling-tests/dev_stabilize_step.rs` · `HARN` = `.claude/workflows/stabilize.js`, `tooling-tests/stabilize_simulation.rs`, `tooling-tests/stabilize_harness_fence.rs`, `tooling-tests/fixtures/stabilize-runtime.mjs`, `tooling-tests/fixtures/stabilize-test-stage.trace` · `REG` = `dev/regression-set`, `tooling-tests/dev_regression_set.rs`, `completions/artifacts/M55/stabilization-build/regression-set/` · `DOC` = `implementation/stabilization-workflow.md` · `LOG` = `DECISIONS.md`.

**Edits under `.claude/`.** The harness is under `.claude/workflows/`, and four core tasks have it as their subject or touch one prompt of it: they are a builder's, as every harness task of the first repair was. **The agent definitions and the settings are not**: each such edit is a step of the orchestrating session's own, small, its own gated commit. **No task and no step edits `CLAUDE.md`.**

| # | Goal | Writes | Done when | Must not touch | Size |
|---|---|---|---|---|---|
| **`K0`** | **The runtime probes, as invocations** (§5). The harness takes `probe: <name>` with a scratch root and nothing else, and runs one probe through its own steps, prompts and schemas; `dev/stabilize-probe` is what the probes' agents run | `HARN`; `dev/stabilize-probe` and `tooling-tests/dev_stabilize_probe.rs` (new, registered in `tooling-tests/groups/g_tooling.rs`); `DOC`, `LOG` | `dev_stabilize_probe::`, by name: a throwaway run under the scratch root whose `state` line is at least the size asked, at four sizes; a batch of N entries with its sha256; a file's hash as one hashed line; a hold that writes its last line only after its seconds, started once and answered `running` within a slice; **every act refuses a scratch root inside the repository and changes no file outside its own** (the tree and `git status` before and after). Simulation: each of the four probes returns `probed` with one result per case under the stand-in; a probe beside `stage` or `run` is refused before any agent; no step of a probe names a branch. The stand-in learns to leave a field out | any stage's logic; the two record scripts | S–M |
| **`K1`** | **`P2`.** `vet`, and its three boundaries with their exits — on the acts as they are today | `REC` (the read), `STEP` (`check-reports`, `record`, `push`), `HARN` (the report check's prompt carries the scratch root; a refused report is read as a missing one), `DOC`, `LOG` | `dev_stabilize_step::nothing_is_published_that_was_not_vetted` — the plan review's `B2` block: `push` refuses, the bare remote unmoved; the mutant without the check at the push, red. `::nothing_is_committed_that_the_record_script_would_not_have_written` — each refusal of a write × a `test` report, a fix cycle's report, a round's scope, and the subject. `::a_report_the_script_would_not_have_written_is_no_report` — the file is in the scratch, the tree does not hold it, the check names it. `::a_vet_hit_at_the_commit_leaves_a_tree_the_next_attempt_starts_from` — after it `git-state` is `ready` and the position is the next attempt. `::a_batch_whose_report_is_gone_is_not_committed`. `::a_scanner_that_cannot_run_is_a_refusal_at_every_boundary`. Simulation: a reviewer's report overwritten by hand voids its item, the stage reaches its record, and neither the commit nor the remote holds the text. Review A's blocks `R1` and `R13` end in refusals | what a writer checks at a write; what the `fix` acts do beyond the shared push | M |
| **`K2`** | **`P1`, the two scripts.** The table of §3, as data; `git-state` and `record` answer from it; the acts repeatable; `discard` an act that asks git; a read that prints the table; a look that finishes nothing | `STEP`, `REC`, `DOC`, `LOG`; an assertion of `stabilize_simulation.rs` only where it pins the old answer | `dev_stabilize_step::a_record_step_killed_between_its_commands_ends_in_one_of_two_states` — every child command of `record` and `push` and every file operation of `apply`, counted by an instrumented run and read from the table's print; each kill ends within two reads in **the state before the batch** (the journal not yet written) or **the state of the uninterrupted control** with the remote at the local head — never a third, never the human's. `::the_wrong_branch_is_refused_before_any_write_fetch_or_push` — with temporaries, a batch that could be settled and a push that is owed all planted: the journal, the tree and the remote unchanged, and the git commands the tool ran, logged by a shim, are reads of the local repository only. `::a_lock_git_left_is_named_and_not_removed`. `::looking_finishes_nothing_and_says_what_is_owed`. `::a_commit_that_is_no_records_is_not_pushed_by_the_tool`. By name, too: a commit that failed is asked for again and commits (`R3`); a batch that changes nothing is refused at `apply` (`R2`); `discard` refuses a batch that is in a commit; a temporary is no report (`R4`); a branch that moved under the stage is refused. Four mutants, each red: the settling by what `HEAD` holds, the staged lines, the temporaries, the push that is owed. Review A's blocks `R2`, `R3`, `R4`, `R7` answer as it expected | the harness — every flag it passes today keeps working, a new one is optional until `K3`; what `state` computes; the `fix` acts beyond a helper they share — `push`'s behaviour for a branch that is no loop branch is as `K1` left it | L |
| **`K3`** | **`P1`, the harness.** A stage starts from that answer; an invocation that only looks finishes nothing; a halt after a commit says what the next invocation does; a tool step's retry runs the one command again | `HARN`, `DOC`, `LOG` | simulation, by name: a push that failed after the record is made by the next invocation, which then answers from the state (`R-M1`); `stopAfter: 'state'` over a pending batch, and over an owed push, moves neither head and returns what is owed (`R-L1`); a commit that failed is finished by the next invocation; a ruling sent twice is refused and leaves no batch; a commit step whose agent died once records on its retry — the stand-in learns *dies once* | the two scripts; `runFix`; what the harness reads of the state document | M |
| **`K4`** | **`P5`.** An item judges a clause of the census; an item with a result keeps what the derivation reads; an in-scope item no tested round selected is in the state and in every return | `REC`, `HARN` (the return, and what `close` hands back), `DOC`, `LOG` | `dev_stabilize_record::an_item_judges_a_clause_of_the_census_or_the_run_is_not_ready` (both orders of writing), `::an_item_that_has_a_result_keeps_what_the_derivation_reads` (each of the four cells refused, a brief taken, `next` still `retest`), `::an_in_scope_item_no_tested_round_selected_is_named`; three mutants, each red. Simulation: a stage whose test set holds such an item names it in its return. Review A's blocks `R5` and `R6` answer as it expected; the plan review's `B10` block names the item while `next` is `close` — which is the bound, and the doc says so | the writers' table and its suite's table (a row, due with the `fix` half); whether an item nobody selected forbids closing — ruling 10 says it owes nothing | S–M |
| **`K5`** | **`P6`, the tool and the first run.** The list read from the candidate's commit; one commit twice refused; an exclusion only by a row; the previous release built from a clone | `REG`, `DOC`, `LOG` | `dev_regression_set`, by name: a list that is not in the candidate's commit is refused; two commits that are one are refused; a test that fails on its own binary and is on no row is void, and the line names it; an exclusion row for a test that passes there is reported stale. Review B's two blocks end in refusals. **One real run**, the candidate the task's parent commit — which holds the list — with no exclusion row: its line is committed with the record, and it names every test that fails on its own binary | the swap and its proof; the harness | M |
| **`K6`** | **`P6`, the rows and the second run — only if `K5`'s run named a test.** Two commits, each behind its gate: the exclusion rows; then the line of a run whose candidate is the commit that holds them | `REG`'s list and record, `LOG` | the second run is green and its `excluded` is exactly the rows; both lines are in the record | the tool | S |
| **`K7`** | **The small canary's setup, and its opening data** (§6). `dev/stabilize-canary` builds the clone, its bare remote and a synthetic opening, and checks them; the regression set's brief is written once, in the workflow doc, for an opening to copy | `dev/stabilize-canary` and `tooling-tests/dev_stabilize_canary.rs` (new, registered in `tooling-tests/groups/g_tooling.rs`); `DOC`, `LOG` | `dev_stabilize_canary::`, by name, on a small source repository of the suite's own: a setup under a root outside the repository leaves a clone at the commit named, **one remote, a bare repository under that root**, the loop branch and the run's slug, an opening written through the record script, committed and pushed there, and `state` answering `not_ready: []`, `next: test`; it prints the invocation's arguments; it **refuses a root inside the repository, and a slug or a loop branch the source repository holds**; the source repository's refs, tree and configuration are the same before and after; `check` refuses a clone with a second remote or a `url.*` rewrite. And, read off the harness and said in the doc: whether an invocation's `scope` bounds a round to the doors it names — the canary's cost rests on it | the scripts of a run; the harness | S–M |
| **`K8`** | **The doc says what the code does, and the rows are on record.** Every sentence of review B's table and of `R15` is true or gone; the six things an orchestrator would get wrong are said where an orchestrator reads; the bounds of §10 are declared as the human accepted them; every row of §7 that is due later has its row in `decisions-pending.md` | `DOC`; the header comments of the three scripts and of the harness; `implementation/decisions-pending.md`; `LOG` | the commit's own table: each of the sixteen rows, each bullet of `R15`, `R-L10` and each of the six, against *made true by `K<n>`*, *the sentence as it now reads* or *a row, due …*. Every row of §7 with a due date is found in `decisions-pending.md` by its key. `doc_link_fence` green | any behaviour | M |

**Optional, after the core — five small tasks, each droppable by the human or the orchestrator.** Each closes **one cell** of what `P4` and `P3` would close as a class, and is cut because what it costs in round 1 is a stage's instruments — hours — against about an hour of building and one gate. Each is a builder's, writes `HARN`, `DOC` and `LOG`, and they run one at a time in the order of what they are worth. **None waits for a probe's result**: none adds a `required` to a schema, which is the one thing probe 1 decides. `X1`, `X3` and `X4` need the stand-in that `K0` teaches to leave a field out; `X1` needs `O6`.

| # | Goal | Closes one cell of | Done when (simulation, by name) |
|---|---|---|---|
| **`X1`** | An agent that leaves a field out voids what it was launched for: no hash → its item is void, with that reason; `confirmed` with no regression fact, or with no hash of the previous binary → the finding is unverified. No schema changes | `R-H2` — the likeliest way a first stage ends with nothing recorded | a reviewer that returns no hash voids its item and the stage records; a confirmed verdict without its regression fact is unverified and the stage records; a hash that is returned and wrong still halts |
| **`X2`** | A stage returns the path of the state's file and never the document | `R-H1`, the half that is certain: round 1 stops after the round, and a stage whose `next` is `stop` hands the whole document to the main session | a stage's return over a ledger of sixty rows holds no `state` and names the file |
| **`X3`** | Once the breaker has tripped, the stage still checks its reports and records: what was not launched is unverified, with that reason | `R-M2` | two dead verifiers are two unverified findings on record, and `next` is `triage` |
| **`X4`** | A finding graded again drops what the earlier pass established for it | `R-M4` — it halts at the record, and the next attempt can halt the same way | a second triage pass records under both keyings of what a verifier left open |
| **`X5`** | The reports of the reporters a stage stands on are checked before an instrument is launched | `R-M3` | a preflight that left no report halts the stage with no instrument in its trace |

**These five are patches, and are named so.** The lesson of the first repair is that a third patch on one logic is the signal to redesign; the redesign here is `P4`'s table, which the ruling makes a row. Each of the five is taken only because its cell is cheap, likely, and costs a stage — and each is one the table would have held anyway.

**The steps that are the orchestrating session's own:**

| # | Edit, or act | Closes | When |
|---|---|---|---|
| **the probes** | the invocations of §5, and their record — a short file beside this plan and a `DECISIONS.md` entry | `B7` | after `K0` |
| **the canary** | the steps of §6, in a session that stands in the clone, and its record | the build's own verification, never run | after `K7` and the probes |
| **the bounds** | §10, put to the human before `K8` writes them into the doc | — | before `K8` |
| **`O1`** | `.claude/agents/finding-triage.md`: a missing repro block is said in the entry's `why` and in triage's report, and is no entry; the return names `new`, as the schema does | `R-M5` | any time before a stage runs with real agents |
| **`O2`** | `.claude/agents/milestone-code-reviewer.md`: the hash is read and returned whether or not the pass drives anything | `R-H2`, the definition's half | the same |
| **`O6`** | `.claude/agents/finding-verifier.md`: with every `confirmed` the previous release's binary is hashed and its hash returned — also where the door does not exist there and the block cannot run | `B8`(d); `R-H2`'s third case | the same |
| **`O4`** | the three definitions a run launches that still name the `probe` step | `R-L9`, `L8` in part | any time |
| **`O5`** | `.claude/settings.json`: an allow rule for `dev/stabilize-step`, `dev/stabilize-record`, `dev/stabilize-probe` and `shasum` — **only if a probe shows a permission prompt, and only with the human's word** | — | after the probes |

`O3` (the preflight's definition) belongs to `T9`, which is a row.

**The order.** `K0` → the probes, beside `K1` → `K2` → `K3` → `K4` → `K5` (→ `K6`) → `K7` → the optional tasks that are kept → the bounds → `K8` → the canary → the record that the `test` stage is used for round 1 → the run's opening, which is the human-led step.

## 5. The runtime probes — a step the orchestrating session runs

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

## 6. The ruling on scale — (B) — and what follows from it

**The ruling (the human's, 2026-10-07, relayed by the orchestrator).** The core now; the runtime probes; a small canary that only shows one `test` stage running end to end with real agents; then the run's opening. Round 1 of the real run — which stops after the round by his own ruling — is the tuning round, and everything else from the two reviews and the plan review becomes recorded rows, fixed between rounds. Later test rounds can be used to improve the workflow; it does not have to be fully automated at the beginning.

**What was not chosen — (A), in short.** The whole plan as first cut and then revised: `P3` and `P4` rebuilt as classes before any real stage, a canary at the real run's shape — the real delta, a ledger at full size, a plant per failure — a third review, and only then the opening. *Why not, as far as this plan can say it:* it was four or five more tasks, three of them large, a canary of a day's wall time whose opening had no author and whose plants masked each other (`B5`, `B9`), and the last review the bound of three cycles allows — all of it spent before the workflow had run once on anything real. What (A) would have bought is that the failures of §6's table were closed before round 1 instead of met in it. Nothing of (A) is lost: each of its parts is a row of §7, and what the plan review found about each is kept below.

### The small canary

**What it is.** One `test` invocation in a clone of this repository with a bare remote of its own, from the stage's first step to its pushed record — then the same arguments once more, to see the answer of an invocation that has nothing to do. It plants one thing, and only where the session allows it (below).

**Its opening — synthetic, written by `K7`'s script, never the human-led opening.** The smallest test set that still has **one item of each kind the real round 1 will have**. The real opening is not written yet, so the kinds are read off the exit rule's instruments (`DECISIONS.md` → *2026-10-04 — The exit rule, revised*): the review rows → one `review-row` on one door; the trial arms → one `trial-arm`; the regression set → one `check`, run from the brief `K7` writes; and the candidate's gate → one `check`. The port rehearsal has no instrument, and so no item. `dev/stabilize-canary` takes the kinds as an argument with these four as its default: **if the real opening is to hold an audit kind, the orchestrator adds it** — that is the one thing about this opening that depends on the real one. Beside the items: the four clauses, the stop mode `every-round`, the previous release and its commit, **one seeded ledger row that is not triaged**, so that triage and one verifier run, and an invocation whose `scope` names the one door, so that the scope step derives nothing over the real delta.

**What it must show.**

1. **The stage reaches its record and pushes it, with real agents**: the attempt's marker, the scope, the instruments, triage, a verifier, the record's own gate, the one commit, the vetted push to the clone's remote, the state read back — and the second invocation answering from that state.
2. **The preflight building a commit that is not the one checked out**: in every stage, the previous release's binary, from its commit, by `git archive`, with nothing run on the tree; and, where 4 below leaves a triage unfinished, the candidate itself — the lap that finishes a triage builds the commit the round tested, which the record's commit has by then moved the branch past.
3. **A reviewer asserting the binary's hash**: the `review-row`'s source pass drives nothing, and its return carries the hash or does not — with `O2` made, and with or without `X1`.
4. **How an agent really ends.** The one plant: **whoever runs the session stops the seeded row's verifier from outside, each time it is tried.** That shows what the runtime hands the script for an agent that is gone and after how many tries, whether such an agent writes its report afterwards, and it leaves the finding unverified — so `next` is `triage`, and the invocation after it is the lap of 2. *If the session cannot stop a subagent* — a headless one cannot — this is not shown, 2's second half is shown only if a triage happens to be left unfinished, and both stay rows.
5. **The first preflight's whole turn in one agent**: two release builds, the candidate's gate, and the regression set from its brief — about thirty-five minutes on its own.
6. In passing, because a stage cannot run without them: that the runtime's concurrency lets a scope step and a preflight run side by side; a trial arm's three steps against the ten-minute tool ceiling; a subagent's first `dev/stabilize-step` and `shasum` without a permission prompt.

**What it deliberately does not show.** A state or a payload at the real run's size — the probes measure those. A push that fails, a candidate whose gate is red, a report the script refuses, a second triage pass, two agents that die, a stage killed as a whole. A ruling, a go, a re-run. The check that reads CI, which is void in a clone. A cross-model pass. The real remote's rules, the real denylist, the real checkout's permission state. Any round after the first. **Each of those is first met in round 1, or is a row.**

**Who does what.** `K7`'s script builds the clone and the opening, from this checkout, naming the clone by its absolute path. **Its session stands in the clone**: a stage runs wherever its session stands (`R-L8`), so either the human starts a session there with a prompt the orchestrator hands him, or the orchestrator launches a headless one there and reads its results from files under the canary's root — whether the workflow runs headless is not known, and is a minute to find out. Afterwards the orchestrator writes what it showed as a record on this branch, aggregated, with no transcript; nothing of the clone's tree comes back.

**How it is kept from this repository's remote.** The clone's one remote is a bare repository under the canary's root, and `dev/stabilize-canary check` asserts it before the session starts. The canary's slug and loop branch exist in the clone only — the setup refuses otherwise — so a stage launched from the wrong directory halts at its first step with nothing written (driven for the first cut, §1), and `K2` pins that order with a test. `gh` resolves no repository from a local remote.

**What it costs.** About two to two and a half hours of wall time: the preflight alone is seventy to eighty minutes with the regression set in it, then the instruments, triage, and the record's gate. Some ten agents. Tokens not estimated. Without the trial arm and the regression item it is about an hour and proves less of what round 1 does first; the orchestrator may cut either, and says so in its record.

### What a round-1 `test` stage can still do wrong, with the core landed — and what each costs

| What happens | Why it is still there | What it costs |
|---|---|---|
| a relayed state line fails its hash three times | `P3` is a row (`R-H1`) | at the first read: minutes. At the second: the preflight — two builds, the gate, the regression set, over an hour — and the attempt. At the third: nothing but the return; the record is committed and pushed. **Probe 2 says how likely, before the opening** |
| the state document comes back to the main session whole | `R-H1` — certain, at every stop | context, not correctness. `X2` closes it |
| a reviewer returns no hash; a verifier confirms and leaves out a field | `P4` is a row (`R-H2`); `O2` and `O6` make it less likely | the stage halts after its instruments: **the instruments are run again**, and the attempt is counted. `X1` closes it |
| a second agent of one invocation dies | `R-M2` | the same: nothing recorded, the instruments again. `X3` closes it |
| a second triage pass keys what was left open to a confirmed finding | `R-M4` | a halt at the record, the instruments again — **and the next attempt can do the same**, after which the stage is the human's. `X4` closes it |
| the preflight or the scope step left no report | `R-M3` | the same halt, found after the instruments instead of before. `X5` closes it |
| one finding's text is refused by the record script — a term of the denylist, a cell its reader refuses | `B4`, `B5`, `R12` | a halt at the record, the instruments again, and it too can repeat. No optional task closes it: its fix is `P4`'s second design |
| triage itself fails — no result, a halt, counts that do not balance | `B4` | the same; `O1` removes the one cause that was a definition's |
| a record cannot be committed before its round has a gate, under a red candidate | `T5` is a row (`R8`, `B1`) | only with a red candidate and two halted attempts; the way out is by hand |
| the triage return or the record's payload is too large to arrive | unmeasured — probe 3 measures the second | a halt at the record, the instruments again |
| **a check's verdict is wrong** — the preflight reads the regression tool's line, or CI, or the tarball's, and returns another word | `T9` is a row (`R-M8`) | **a false cell on record.** `K5` makes the tool's own line sound; the line still passes through an agent. Until `T9`: at the round's stop the orchestrator has a subagent read the tool's line from the file the preflight's report names — a check by hand, and the row says so |
| **an item is never selected because its door is mistyped** | it cannot be held (`B10`) | a clause green without that item. `K4` names it in the return of every stage; whether anyone reads the name is not the script's |

**A probe that fails moves its row.** If probe 2 shows a line of the real run's size coming back broken, or probe 3 a payload, the row is no longer *between rounds 1 and 2*: round 1 could not record at all, and its fix is due before the opening. That is the orchestrator's to bring to the human with the probes' record.

**The two sentences the ruling rests on.**

- ***Never an unscanned push* — true once `K1` and `K2` have landed, for every push the step tool makes**, and the step tool makes every push of a stage. It would be false if: somebody pushes the loop branch by hand (CI's scan is hard, and runs after); `vet`'s scanners were not the two CI runs (`K1` uses the function the script's writers already call, which runs those two); a scanner that cannot run were read as a pass (`K1` has the opposite as a named test). It says nothing of a commit outside the run's directory: the tool does not push one.
- ***Never a false record* — not true without two exceptions.** What the script *derives* is sound for round 1 and what follows it without a fix: review A killed eighteen mutants on it, and `K4` closes the two inputs that could still be bent. **What is handed to the script can be wrong**: a check's verdict, which an agent reads; and a door's name, which nothing can hold — the last two rows above. Beside them stand an agent's own judgments — a verifier's verdict, a key triage gives — which no plan of this kind reaches, and the first review's `F2`, unreachable for as long as `fix` refuses to start.

**So what the ruling buys and what it pays.** Everything in the table but its last two rows costs at most *a late halt and a second run of the instruments* — hours, a counted attempt, and after two the human's grant. Round 1 stops after the round whatever happens, so each such halt is seen by the human before anything is built on it. The last two rows are the price: one is closed by `T9` and held by a check by hand until then; the other is declared.

### What each row's fix must hold when it is taken up — the plan review's findings, kept

- **`P3`** (`T7`). Its size and urgency are probe 2's to say (`B7`). It must cover the refusal lines and `record`'s lists of red, which are relayed free text too, and not the `state` act alone. The doors nobody reaches (its part (c)) must *forbid* or be named at the close, as `K4` names the items nobody selected (`B10`). It is cut before anything that rebuilds what the harness reads of the state.
- **`P4`** (`T6`). **Two designs, and this plan chooses neither:**
  - **Design 1 — the table, and the rule with its real exceptions.** One table of role × ending that the code consults and the simulation enumerates. Once an instrument is launched the stage reaches its record **except**: any ending of triage; a batch the record script refuses for one entry's text; a relay or a tool step of the record that fails; and — unless the table makes it a void — a hash that is returned and wrong. *Cost:* one large task on the harness and its simulation; `X1`, `X3`, `X4`, `X5` are four of its cells, built early. *It leaves:* findings living only in returns until the batch, so each exception still loses the stage's instruments, and two of them fail again on the next attempt.
  - **Design 2 — findings reach the disk through the record script, from the agent that made them.** A reviewer, triage and a verifier each write what they found as it is found; the script scans it at that write, where its author can still reword it; the record's batch names what is staged and carries none of it. A triage that fails then leaves the instruments' findings on disk, the next invocation finishes the triage and runs no instrument, and the payload no longer grows with the findings. *Cost:* new writers in the record script — in the file and the table the writers' fork is about — every reporting definition edited (the orchestrator's steps), the harness's triage rebuilt, the simulation's scenarios rewritten: two large tasks and a review of its own. *It leaves:* the endings table still wanted, and small.
  - *What decides:* probe 3 (does a payload at size arrive), probe 1 (what a schema's `required` buys), and what round 1 shows. In either design a failed relay, a thrown call, a report nobody launched and an executor that halts each need a row — the first cut claimed them closed and gave them none.
- **`T5`** (`R8`). As first cut it cannot be built (`B1`). It holds: where a record's round has no gate on record, the record step gates the tree before it applies its batch and **hands that file to the commit step as what it is held to**; no `gate.md` is written by a record that tests nothing, so the round's gate stays the gate of the candidate it tests; its *done when* drives the granted stage through its own record — `B1`'s block ending `recorded` twice. No rule on counts. The lead *a round's gate and its candidate are not held to one commit* is decided with it.
- **`T9`** (`R-M8`; `O3`). Not cut before probe 4. Its options are four: a brief the preflight reads — which is how round 1 runs it; an act the harness drives, started and asked again, which needs a process to outlive the agent that started it; the preflight holding it by the gate's own pattern with a script reading the result from the file; and the check run outside a stage if neither holds. Whichever: it needs the record script — a green row's detail is `-` today, no fact of a round holds a list's hash, and the harness cannot tell a scripted check from a brief (`B8`).
- **`T11`** — what a run shows about numbers. **It may hold numbers and nothing else**: how many retries, the breaker's threshold, a slice's length, a size that is declared (`B6`). A change of design is a task of its own, with its own review.
- **A canary at the real run's shape**, if one is ever wanted: plants that cannot mask each other; a ledger whose *untriaged* rows are at the real size; a dead agent or two, and a record gate that is newly red with the pending batch after it; an honest count of invocations; an author for its opening; and its gaps said (`B5`, `B9`).

## 7. Every finding — its home, or its row

**This is the table the run's ledger can be seeded from at the opening.** A finding that the core closes has its task. Every other finding is a **row**: what it costs if it happens in round 1, and when it is due. A row's key is `wf-` and the finding's id in lower case (`wf-r-h1`, `wf-b4`); its repro is the block in the report that found it. `K8` writes each row that has a due date into `decisions-pending.md`.

*Due:* **core** — a task of §4 · **before a stage** — an orchestrator's step or `K8`, before any stage runs with real agents · **optional** — one of `X1`–`X5`, before the opening if it is kept, else *1→2* · **1→2** — between rounds 1 and 2 of the first run · **`fix`** — with the `fix` half's repair (§9) · **later** — a later release's run · **declared** — a bound of §10, revisited when a run shows it bites.

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
| `R-H1` | the state document relayed whole | `X2` for the return; row (`T7`) for the relay | halt at a state read — minutes at the first, over an hour at the second, nothing lost at the third; and the document in the main session at every stop | optional; 1→2 — **before the opening if probe 2 fails** |
| `R-H2` | one field left out halts the stage | `O2`, `O6`; `X1`; row (`P4`) | halt | before a stage; optional; 1→2 |
| `R-M1` | after a rejected push the same invocation pushes nothing | `K2`, `K3`; its four sites in `runFix`: row | — | core; `fix` |
| `R-M2` | two dead reporters halt the stage | `X3`; row (`P4`) | halt | optional; 1→2 |
| `R-M3` | the stood-on reporters checked late | `X5`; row (`P4`) | halt | optional; 1→2 |
| `R-M4` | a second pass keyed to a confirmed finding | `X4`; row (`P4`) | halt, **and the next attempt can halt the same way** | optional; 1→2 |
| `R-M5` | the triage definition against the arithmetic | `O1`; the arithmetic stays | — | before a stage |
| `R-M6` | green over a list no commit holds | `K5`; *who adds a row*: declared, and row | — | core; `fix` |
| `R-M7` | one in a hundred drops out | `K5`, `K6` | — | core |
| `R-M8` | a check's verdict is an agent's reading | the brief: `K7`; a check by hand at the round's stop; row (`T9`, `O3`) | **worse than a halt: a false cell on record** — a word the tool did not print; or a check the preflight could not hold, which is void and makes its clause the human's | 1→2 |
| `R-L1` | `stopAfter` over a pending batch commits and pushes | `K3` | — | core |
| `R-L2` | the candidate's gate runs on a tree with the run's pending files | declared; the canary and round 1 read it | a red of the run's own files on record as the candidate's, and then accepted of every record of the round | later — 1→2 if it shows |
| `R-L3` | the previous release's binary known by a version string | row (with `T7`) | a regression fact against the wrong binary: a regression goes to the human instead of to a fixer — the safe side | 1→2 |
| `R-L4` | a door no item reaches is known from a return alone | `K4` names the items nobody selected; the doors: row (with `T7`) | read from the return, or missed | core; 1→2 |
| `R-L5` | a one-command step wrapped in a retry that says look around | `record`: `K3`; `begin`: declared | a retried `begin` counts the attempt | core; later |
| `R-L6` | the self-test passes a harness that runs the cross-model pass everywhere | `K8`, a sentence; declared | nothing: the simulation catches it | before a stage |
| `R-L7` | `dev/gate --quick` prints `GATE: PASS` | declared | nothing: both scripts that read a gate refuse a pre-check's output | later |
| `R-L8` | nothing in an invocation names the repository | declared; `K7`'s guards | a stage launched in the wrong repository halts at its first step, nothing written | later |
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
| `B7` | tasks cut on unseen runtime facts | `K0`, the probes | — | core |
| `B8` | *done when* defects | (a), (b): `K2`; (c): row with `T9`; (d): `O6` | — | core; 1→2; before a stage |
| `B10` | a mistyped door | `K4` names it; declared | **worse than a halt: a clause green without that item** — seen only by whoever reads the names at the stop | core; declared |
| advisory | an index lock; an owed push under a look; an unpushed tuning commit | `K2`, `K3`; declared | a refusal that names its way out | core |
| advisory | the table keyed so the `fix` half extends it | `K2`; §9, row 1 | — | core |
| advisory | refusal lines and `record`'s red lists are relayed free text | row, with `R-H1` | as `R-H1`, at a few kilobytes | 1→2 |
| advisory | a failed relay, a thrown call, a report nobody launched, an executor that halts have no row | row (`P4`) | halt | 1→2 |
| advisory | `T8` needs two runs and a form for an exclusion row | `K5`, `K6` | — | core |

**Refuted: none — of the reviewers' findings, and of the plan review's.**


## 8. The forks as they now stand, and what was buried as settled

**The question on scale is ruled: (B)** (§6). Of the first cut's five forks, under that ruling:

**What is left with the human — three things, none of them a design.**

1. **Whether the core's diff is reviewed before round 1.** The rule of 2026-10-06 is his: a half is used on a real run once its repair *and the re-review of that repair* are recorded. The ruling on scale names the core, the probes, a small canary and the opening, and no review. *Options:* (a) a focused, independent review of `K1`–`K4` — the code that stands between a run and a public push — about what one review of four tasks costs, and it is the third review of this half, the last the bound of three cycles allows; (b) none: round 1 is the tuning round for this too, and the rule is set aside for this half by his word. *Recommendation: (a)* — of everything in this plan, `K1` and `K2` are what the sentence *never an unscanned push* rests on.
2. **The bounds of §10, accepted for a public first run.** A declared bound is his by the exit rule. Two of them are not on the safe side and are named as such: a check's verdict read by an agent, and a mistyped door.
3. **The canary's session — a request.** A stage runs wherever its session stands, so the canary needs a session that stands in its clone: he starts one there with a prompt the orchestrator hands him — which is also the only kind of session that can stop a subagent from outside, the canary's one plant — or the orchestrator launches a headless one, if a minute's trial shows the workflow runs that way.

*And two that are a permission or a drop, whenever they come up:* `O5`, an allow rule in the settings, only if a probe shows a prompt; and any of `X1`–`X5`, which he or the orchestrator may drop.

**What falls away under the ruling.**

- **How a scripted check is run and judged** (the first fork). Round 1 runs the regression set from its brief, with a check by hand at the stop; `T9` is a row due between rounds, and its four options (§6) are weighed then, with probe 4's result. That a script should read the verdict, and no agent, was never a fork: `C6` answers it.
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
7. **Every push of the `fix` stage is vetted by `K1`'s check already** — the round branch's among them, over product code; the `fix` half's repair says what a hit means there, where the commit is a fixer's.
8. **`raise` beside the recorded `cycles`** (`L5`), and **a fixer's fork has no row** — both owed already, restated so that the list is whole.

## 10. Declared, not fixed

*`K8` writes these into the workflow doc with their reach — after they have been put to the human (§8).*

- **A door mistyped in an item's row reads as a door the round did not reach.** The item is never selected, owes nothing, and its clause can be green without it. The state and every stage's return name the in-scope items no tested round selected (`B10`; `K4`).
- **A check's verdict is the preflight's reading** of a brief and of a tool's line, until `T9` (`R-M8`).
- At a clause that is the human's, the position still hands a `test` stage the next round; `next` does not ask for it (`R11`).
- The candidate's gate runs on a tree that holds the attempt's marker and may hold the round's scope and its report (`R-L2`).
- A stage runs in whatever repository its session stands in; a canary's slug and loop branch exist in its clone only, and its setup names the clone by its absolute path (`R-L8`).
- The step tool pushes no commit that is not a record's: a tuning commit on the loop branch is its author's to push, and a stage refuses to start over one that is not.
- git's own lock is named and never removed; what a kill inside a git child leaves beyond it is the human's.
- A push made by hand is not vetted by anything but CI.
- The self-test checks the script's tables; what a stage does is the simulation's to show (`R-L6`).
- `dev/gate --quick` prints a gate's verdict line and no totals line (`R-L7`).
- A step that begins an attempt, run twice, counts the attempt (`R-L5`, for `begin`).
- While the opening is not done a direct caller can write a whole run; a writer can take a cell its reader refuses; a writer's place is written by hand (`R9`, `R12`, `R14`) — until the `fix` half's repair.
- A report written for an attempt nobody began counts as an attempt and is committed with the next record; which reporters an attempt launched is in no committed table; a scope is written once, and a human's scope on a second attempt is not applied.
- A row of the regression set's list is held to a pointer that exists, and to nothing that says who ruled it.
- A record's gate is held to the candidate's by the names of what is red, never by how many tests ran.
- How many findings one stage's record can carry is not measured before probe 3, and the triage return not before a stage.
- The replay of a killed run resumed by its run id is unexamined; a fresh invocation after a kill is what is known to work.
- What keeps a real `test` stage from starting is the record alone: the harness refuses `fix` and not `test`.
- Agent-written text reaches other agents' prompts and no shell.

## 11. What this plan did not do

- **It built nothing and ran no agent.** Every *done when* is a test that does not exist yet; its name is the plan's proposal, and a builder that finds a better seam keeps the fact and may change the name.
- **It did not measure the tasks.** The sizes are a reading of the code; nine core tasks are at least ten full gates, each optional task one more, and `K5` and `K6` wait on real runs of thirty-five minutes.
- **It did not run the probes**, and designed them without having seen the runtime: that the harness can take a `probe` argument beside its stages, and that a role agent can be told to leave a field out, are `K0`'s to find out — and a probe that cannot be built as designed is a finding of `K0`, reported, not worked around.
- **It did not recut what the ruling makes a row**: §6 says what each fix must hold and designs none. It did not verify the small canary: that a session can stop a subagent from outside, that an invocation's `scope` bounds a round, and that a trial arm fits a stage are each said to be unknown where they are used.
- **It read the harness in the parts the findings cite**, about a fifth of it, and the record script likewise. A cause that lives only in what was not read is not in §3.
- **Three LOW findings were not checked** (`R16`, `R-L2`, `R-L6`), one traced case of `R-M4` was not driven, and of the plan review's `B5` the count of invocations was taken as reported.
