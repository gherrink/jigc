# The second repair of the `test` half — the plan

*Written 2026-10-07 by a planning agent, on `fix/rc24-tier1` at `cbb3d736`. It read both re-review reports beside this file whole ([records-state.md](records-state.md), ids `R1`–`R16`; [harness-doc.md](harness-doc.md), ids `R-H…`, `R-M…`, `R-L…`), the first review's two reports and the first repair's plan one directory up, and the code at the tip. **Nothing here is built, and nothing here is decided that is the human's**: §6 lists what is. `F…`/`S…`, `H…`/`M…`/`L…` and `C0`–`C6` are the first review's and the first plan's ids, used as they use them; a line number of the workflow doc is its line at `cbb3d736`, before this record's commit touched that file; this plan's own are `P1`–`P6` (the structural changes), `T1`–`T12` (a builder's task), `O1`–`O5` (the orchestrating session's own step) and `Fork 1`–`5`.*

**In one paragraph.** Both reviewers say the same: the `test` half is fit for a canary run and not for the real run. Every HIGH and MEDIUM finding was driven again for this plan, at the tip, and **none was refuted**. The thirty-six findings and the leads come down to **six causes**, and four of the six are one sentence of the first repair's own plan that was built for less than its class: *an act is repeatable and every invocation reconciles first* (`C2`, planned for the `fix` half — the `test` half's record step needs it now), *nothing decisive is only in an agent's hands* (`C6`, built for the fork and for nothing else), *every ending of every agent has one row* (the decision of 2026-10-06, built for one dead agent), and *a write is taken only where the state asks for it* (`C5`, built as a census of names). This plan is those changes, ten builder tasks behind the full gate, five small steps that are the orchestrating session's, a canary at the real run's shape in three parts, and five questions for the human.

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

**What the probes of this plan's own showed.** (1) A stage stopped after its preflight and scope leaves four untracked files, and the same stage invoked again reaches its record as attempt 2 — the first of the five things the doc's builder found (the build record's README) is closed by the pending-writes rule. (2) The harness invoked in a repository that has no such run halts at its first step, `wrong-branch`, with one agent launched, no commit and a clean tree — which is what §5 rests the canary's safety on.

**Qualifications to the reviewers' claims** — none changes a finding:

- `R-H1`'s grade rests on an inference (a relay of that size fails). What is certain without it: the document goes back to the main session whole, where ruling 17 has the main session only orchestrate.
- `R6` and `R9` are a direct caller's: the harness composes no `item-set`, and no stage starts before the opening is done.
- `R8`'s reach is first-run teething under a red candidate. It does not matter whether the harness passes `--round` there: with no gate on record `known` is empty either way.
- `R5` is graded MEDIUM by its reviewer, and both reviewers' summaries say nothing found records anything false. `R5` does: it is the one finding of the round whose end state is a wrong `close`.

## 2. The causes, and the six changes

### P1 — The record's commit is one repeatable act, and every invocation reconciles first

**The cause.** A record step is six sub-steps — apply, gate, `git add`, `git commit`, `settle`, push — and three predicates read what they leave behind (`admitted`, `live`, `pushed_is_not_ahead` in `dev/stabilize-step`). None of the three enumerates the states *between* the sub-steps, so each gap is a state the act itself then refuses, or that reads as finished.

**What it closes.** `R2` (both ways in), `R3`, `R4`, `R7` ≡ `R-M1`, `R-L1`, `R-L5` for the `record` step, the lead *a commit made on the loop branch while a stage runs*, the first review's `L2` (the part left open) and the `test` member of `H5`'s class; and the doc's lines 100, 213–215, 314 and 417.

**The change.** One table in `dev/stabilize-step`, read by `git-state` and by `record`, over four facts it reads itself: the journal (none · a write killed between its files · a batch) × the batch's files (unstaged · staged · every one in `HEAD` at the hash the journal holds · altered) × the loop branch against its remote (equal · ahead by commits that touch only the run's directory · ahead by anything else · behind) × what else the tree holds (nothing · the script's own temporaries · anything else). **Every cell has one answer**: `ready`; a step the tool finishes itself and says it did (`recover`, the settling of a batch that is in a commit, the push that is owed); `pending` (a batch to gate and commit); or the human's. And the acts become repeatable: `record` admits its own staged lines, treats a batch that is already in `HEAD` as recorded, and holds the head to the commit the stage began on; `apply` refuses a batch that would change nothing; `discard` becomes an act of the step tool, which can ask git, and refuses a batch whose files are committed.

**Why one change closes the class.** The class is *the states the record step can leave*. It is finite and it is enumerable mechanically: the act runs a fixed list of child commands and the script a fixed list of file operations. **What would show it has not:** the test that kills the act before *every* child command and the script before *every* file operation — counted by an instrumented dry run, never listed by hand — and asserts that `git-state`, and what it names, at most twice, ends at `ready` with the remote at the local head, a clean tree and the state document of the uninterrupted control. A kill point that ends at the human, or at a different state, is a hole; a sub-command added later is covered without a new test.

**This is `C2`, moved.** The first plan put *acts bracketed by an intent and a done record, reconciled first — the tree is clean or exactly a journaled batch, unpushed means push owed, each open intent names its resume step* under the `fix` half. The journal already is the intent and `settle` the done; what was not built is the reconcile, and the window it leaves is the `test` stage's own record step (both reviewers say so independently: A's table of the record step's four bold rows, B's `R-M1`). **It moves to the `test` half now, for the record act and the push.** What stays with the `fix` half is `C2`'s other half — the `fixes` and `acts` rows and the one landing predicate — and it is built as rows of this table, not as a second mechanism (§7, row 1).

### P2 — What is committed is what the script would have written: one check at the commit boundary

**The cause.** The record script scans at the *write*; the step tool commits by *name*. Between the two a file is whatever its last writer left.

**What it closes.** `R1` (HIGH), `R13`, `R15`'s second bullet, and the doc's lines 108 and 287.

**The change.** The script's write-time checks — the hygiene scan, a host path, the fenced line, the end marker — become one read (`vet`, over paths and a subject), the same functions its writers call. The record act runs it over every path it is about to add and over the commit's subject, holds the staged blobs to the bytes it vetted, and on a hit commits nothing.

**Why one change closes the class.** The class is *bytes that reach the remote without the scan*. After the change the run's files are added by one command, and that command is preceded by the scan of exactly those bytes — whoever wrote them, and whatever their name. A manifest of what the script wrote (the reviewer's other option) would close *changed after it was written* and still trust each writer's scan; the boundary check needs no trust. **What would show it has not:** a test that takes each refusal the script has at a write, times each kind of written-once file (a `test` report, a fix cycle's report, a round's scope) and the subject, and asserts a refusal at the commit step with the remote unmoved; and the mutant that removes the call, red.

### P3 — Nothing a script can read or decide goes through an agent's hands

**The cause.** The harness has no filesystem, so what it computes on comes through an agent. The first repair made every *git act* one command with a hashed line (`C0`) and put *the fork* on record (`C6`); it left the state document inside the line, a check's verdict in the preflight's reading of a brief, and three facts in a return and nowhere else.

**What it closes.** `R-H1` (HIGH), `R-M8`, `R-L3`, `R-L4`, the lead *the gate comparison holds names, never counts*, `R-L7`'s wording, and the doc's lines 133 and 386.

**The change, in five parts.** (a) The `state` act's line is a projection with **no cell of free text** — the position, `next`, the lists by key, the items by id — and names the file the document is in and its hash; an agent that needs a row or its doors reads it through the record script, by run, round and item; a stage returns the file's path and never the document. (b) A scripted check is an act of the step tool that starts it, is asked again in slices and relays the tool's own line — so green, red and void are the script's mapping, and the two commits, the list's hash and what was excluded are fields the harness holds (**Fork 1**). (c) The doors of a round that no item reaches are computed where the scope is written. (d) The previous release's binary is held to the path the prompt named, its hash is not the candidate's, and it is a fact of the round's record. (e) A record's gate is held to the candidate's by count as well as by name.

**Why one change closes the class, and where the class ends.** The class is *a fact computable from the disk or from git that the harness takes from an agent*. Its boundary is an agent's own judgment — a finding, a grade, a verdict, a scope's derivation — which is the agent's to return. **What would show it has not:** a size test of the relayed line at 120 rows and 300 doors with every string in it held to a slug, a sha, a path or a word of a fixed list, and the mutant that puts the ledger back, red. **What it does not reach, and the canary measures:** the triage return and the record's payload, which grow with the number of findings in one stage and travel through an agent by construction (§5; `T11`).

**This is `C6`, extended.** `C6` was *nothing decisive lives only in a return*. It was built for the contested fix. Its `test` members that are left are (b), (c), (d) and the failed push whose only record was the return (`P1`).

### P4 — Every way an agent can end is one row, and after the instruments start no single return halts the stage

**The cause.** *How an agent can end* is a table in the harness's header and in the doc; in the code each cell is decided where it happens. So four endings are decided in the code and stated in neither table, two are decided late, and two contradict the decision the table records — *a dead reporter voids what it was launched for and never the stage* (`DECISIONS.md`, 2026-10-06).

**What it closes.** `R-H2` (HIGH), `R-M2`, `R-M3`, `R-M4`, the first review's `M9` (the part left open) and the regression the repair of `M7` brought; the doc's lines 113–124, 343 and 353; and the paths of the `test` half that no test executes (review B's list).

**The change.** One table in the harness — role × ending → what the stage does — that the code consults and the simulation enumerates, with one rule above it: **from the moment an instrument is launched, the stage reaches its record**, and what a single agent failed to deliver is void, unverified or not launched, each with its reason. In it: a field the stage reads is `required` in the role's schema, and its absence is the row's outcome and never a halt (no hash → the item is void; `confirmed` with no regression fact → the finding is unverified); a hash that is *returned and wrong* halts, as ruling 11 has it; the reports of the reporters a stage stands on are checked before anything is launched on what they established; after the breaker trips no role agent is launched and the stage goes on to its report check and its record; a finding graded again drops what the earlier pass established for it.

**Why one change closes the class.** The class is the cells of that table. The simulation runs one scenario per row and fails on a row without one, so a cell cannot be added, or left, without being driven. **What would show it has not:** a `halted` result from any scenario in which an instrument was launched, other than for a wrong hash that was returned or a tool step of the record itself that failed — the simulation asserts that set, by phase.

**A reading this plan takes, open to overturn.** Ruling 11 says *every agent asserts the hash before it drives anything*. An agent that returns no hash has shown nothing about what it drove, so nothing it drove is evidence: its item is void and is run again. That is what the halt did to that item, without doing it to every other one.

### P5 — The derivation's inputs are held where they are written

**The cause.** `C1` made a clause's status a derivation. Two of its inputs can still be bent after the fact (an item's row), and the table that says where a writer is taken (`C5`) is a census of names: what a writer *does* is still written by hand in each subcommand, and a cell's grammar is written once for the writer and once for the reader.

**What it closes.** In `T4`: `R5`, `R6` and the lead *a round's gate and its candidate are not held to one commit*. In the part that waits (**Fork 3**): `R9`, `R12`, `R14`.

**The change.** An item's clause is a clause of the census, in both orders of writing; an item that has a result keeps the four cells the derivation reads; a round's candidate is the commit its gate ran on. And, behind the fork: the writers' table becomes the code path — no writer runs but through it — and each cell has one grammar that its writer and its reader share.

**Why this is not a third patch.** `R5` and `R6` are the two *inputs* of the derivation the first repair left unheld — its reviewer enumerated the four cells and found two unheld and two replaceable — and holding them is the end of that enumeration. `R9`, `R12` and `R14` are instances of a class whose closure is the table as code; patching the two instances of `R12` would be the patch the lesson warns of, so they are **declared until the table is rebuilt** and not touched before.

### P6 — The regression tool's green rests on facts of the two commits, and on nothing it was handed

**The cause.** The tool proves which binary ran. What a green *rests on* beside that is taken as an argument: the list's file, the two commits, and which tests may drop out.

**What it closes.** `R-M6`, `R-M7`; with `P3`(b), `R-M8`.

**The change.** The list is a path *in the candidate's commit*; two commits that are one are refused; a test that fails on its own binary is excluded only by a row that names it and says why — the tolerance goes; the previous release is built from a clone with no checkout, which is a repository and no registration in this one's, so that the eleven tests that ask git about their tree are measured again instead of excluded; and the line's list hash and exclusions reach the round's record through `P3`(b).

**Alone, as a group.** These three share nothing with the other five causes but the caller. **What would show it has not closed:** review B's two blocks, each answering with a refusal; and one real run whose `excluded` is exactly the rows of the list.

**Three builder's choices are overturned by it** — the list as a file the tool is handed, the tolerance, the archive — each recorded as a builder's in `DECISIONS.md` → *The regression set's first part*, items 4, 7 and 8. None is a ruling of the human's.

### The findings that are genuinely alone

`R8` (what a record is held to where its round has no gate — **Fork 2**) · `R10` (the `fix` half's) · `R11`, `R-L2` and `R-L7` (declared) · `R-M5` (one sentence of a definition that asks for what the harness, rightly, refuses — the orchestrator's step) · `R-L6` and `R-L10` (a sentence of the doc each) · `R-L8` (**Fork 4**) · `R-L9` (the orchestrator's step).

## 3. Every finding, and its home

*Task* — a task of this plan. *Declared* — a bound written into the workflow doc by `T10`, with its reach. *`fix`* — the `fix` half's repair, with its row in §7. A finding with two homes has two parts.

**Review A — the record script and the state machine**

| | Finding | Home |
|---|---|---|
| `R1` | unscanned text on the remote | `T3` (`P2`) |
| `R2` | a batch that outlives its commit | `T1`, `T2` (`P1`) |
| `R3` | a failed commit leaves a staged tree | `T1` (`P1`) |
| `R4` | the temporaries of a killed write | `T1` (`P1`) |
| `R5` | an item whose clause the census lacks | `T4` (`P5`) |
| `R6` | `item-set` replaces an item that has results | `T4` (`P5`) |
| `R7` | a record committed and not pushed | `T1`, `T2` (`P1`) |
| `R8` | a record with no gate on record, under a red candidate | `T5` — as **Fork 2** is ruled |
| `R9` | the script takes the whole run while the opening is not done | **Fork 3**: `fix` (§7, row 4), declared until then — the doc's bound at line 460 is widened to what `R9` drove |
| `R10` | after a dropped round only an agent decides that no hunt runs | `fix` (§7, row 3) |
| `R11` | at a clause that is the human's the position hands `test` round R+1 | declared — reach: an orchestrator that answers `rule` with `test`; `next` never asks for it, and the state is already on the doc's list *Open, the human's to rule* (*the states with no step inside a round*) |
| `R12` | two inputs taken, or refused in the wrong class | **Fork 3**: `fix` (§7, row 4), declared until then — reach: a direct call, or an agent's text of exactly `-`; the state then stops reading, loudly |
| `R13` | the subject of a record commit is not scanned | `T3` (`P2`) |
| `R14` | the writers' table is a census of names | **Fork 3**: `fix` (§7, row 4) |
| `R15` | what the header and the doc claim and the code does not do | each bullet by the task that makes it true (`T1`, `T3`, `T4`); what is left, `T10` |
| `R16` | what no arm of the suites drives | bullet 1: `T1`, `T3`; bullet 2 (`S1`, `S2`): `fix` (§7, row 2); bullet 3: `T10` renames nothing — the test that holds the go is named in the doc |
| lead | a round's gate and its candidate are not held to one commit | `T4` |
| lead | which reporters an attempt launched is in no committed table | declared — once the batch is committed git holds the reports themselves |

**Review B — the harness, the agents, the gate, the regression tool, the doc**

| | Finding | Home |
|---|---|---|
| `R-H1` | the state document relayed whole | `T7` (`P3`); the record's payload and the triage return: the canary, then `T11` |
| `R-H2` | one field left out halts the stage | `T6` (`P4`); the reviewer's definition, `O2` |
| `R-M1` | after a rejected push the same invocation pushes nothing | `T1`, `T2` (`P1`); its four sites in `runFix`: `fix` (§7, row 6) |
| `R-M2` | two dead reporters halt the stage | `T6` (`P4`) |
| `R-M3` | the stood-on reporters checked late | `T6` (`P4`) |
| `R-M4` | a second pass keyed to a confirmed finding | `T6` (`P4`) |
| `R-M5` | the triage definition against the arithmetic | `O1`; the arithmetic stays — it is what keeps a finding from being lost |
| `R-M6` | green over a list no commit holds | `T8` (`P6`); *who adds a row*: declared, and `fix` (§7, row 5) |
| `R-M7` | one in a hundred drops out | `T8` (`P6`) |
| `R-M8` | a check's verdict is an agent's reading | `T9` — as **Fork 1** is ruled; the definition, `O3` |
| `R-L1` | `stopAfter` over a pending batch commits and pushes | `T2` (`P1`) |
| `R-L2` | the candidate's gate runs on a tree with the run's pending files | declared (the bound at the doc's line 474, sharpened to say what a file can turn red and where that goes); the canary reads it (§5, item 6); `T11` if it shows red |
| `R-L3` | the previous release's binary known by a version string | `T7` (`P3` d) |
| `R-L4` | a door no item reaches is known from a return alone | `T7` (`P3` c) |
| `R-L5` | a one-command step wrapped in a retry that says look around | `record`: `T2` (a repeatable step is run again, and its retry says so); `begin`: declared (the bound at line 461 has it), and the canary shows how often |
| `R-L6` | the self-test passes a harness that runs the cross-model pass everywhere | declared — a sentence of the doc: the self-test checks the script's tables, and what a stage does is the simulation's |
| `R-L7` | `dev/gate --quick` prints `GATE: PASS` | declared — the build harness's check is written against it, and both scripts that read a gate demand the totals line beside it; `T7` words the two prompts as *the totals line and the verdict line* |
| `R-L8` | nothing in an invocation names the repository | **Fork 4**; declared whichever way it is ruled |
| `R-L9` | three definitions still name the `probe` step | `O4` |
| `R-L10` | the doc names three of six rulings about the run | `T10` |
| the doc's sixteen sentences | false, or lacking what the code does | each by the task that changes the code under it; the rest and the six things *an orchestrator would get wrong*, `T10` — its *done when* is the table, row by row |
| lead | a report written for an attempt nobody began counts as one and is committed | declared — the bound at line 459 says half of it; `T10` adds that it counts |
| lead | the gate comparison holds names, never counts | `T5` (`P3` e) |
| lead | a commit made on the loop branch while a stage runs | `T1` (`P1`: the head is held) |
| lead | a human's scope on a second attempt is not applied | declared — a scope is written once; `T10` says what the orchestrator does instead |
| lead | agent-written text in other agents' prompts | declared — none reaches a shell; kept from the first review |
| lead | `resumeFromRunId` after a kill | declared — an invocation is never resumed by its run id; the canary's last item |

**The first review's findings that are still open**

| | Finding | Home |
|---|---|---|
| `F2`, `S1`, `S2` | a fix on the ledger with no counted cycle; the suite certifies it | `fix` (§7, row 2) — and until then `test` is sound for round 1 and what follows it without a fix, as review A says |
| `F10` | `admitted` is not inherited | declared (it is, at line 473) |
| `F13` | the header claims what the code does not do | `T10`, with `R15` |
| `H1`–`H4`, `M1`, `M12` | the landing, the part, the triage inside a running `fix` | `fix` — unchanged, unreachable behind its refusal |
| `H5` | `landed-before` reports landed, unchecked and unpushed | `fix` (§7, row 1); its `test` member is `R-M1` |
| `M8` | the path rule's limits | the last task of the whole repair, as the first plan has it. Its bearing on `test` is declared by `T10`: a merge on the loop branch that is not a round's — a merge of `main` among them — halts every later `test` |
| `M9` (in part) | two dead agents | `T6` |
| `L2` (in part) | *the same args* after a push halt | `T2` |
| `L5` (in part) | `raise` beside the recorded `cycles` | `fix` (§7, row 7) |
| `L8` | the `probe` sentence | `O4` for the three a run launches; the other five keep their row in `decisions-pending.md` |
| `L12` | a run slug ending `-r<N>` | declared (it is) |
| leads | a new finding keyed as found again; the door text triage returns against the reporter's | declared — each needs the first run; `R-M4` is the one member that was driven, and `T6` takes it |
| the doc builder's first item | a stage stopped after its scope step, and the next `git-state` | **closed** — driven for this plan (§1) |

**Refuted: none.**

## 4. The tasks

**Every task is one `build-executor` behind the full `dev/gate`, on the tree of its own commit.** They run **in the order given, one at a time**: the write sets below show why — `dev/stabilize-step` is written by five tasks, `dev/stabilize-record` by four, the harness by five, and every task appends to `DECISIONS.md` and corrects the sentences of the workflow doc that it makes false. `T8` alone writes none of the three scripts; it could run beside another task only if the orchestrator took both logs' entries itself.

**Write sets.** `REC` = `dev/stabilize-record`, `tooling-tests/dev_stabilize_record.rs` · `STEP` = `dev/stabilize-step`, `tooling-tests/dev_stabilize_step.rs` · `HARN` = `.claude/workflows/stabilize.js`, `tooling-tests/stabilize_simulation.rs`, `tooling-tests/stabilize_harness_fence.rs`, `tooling-tests/fixtures/stabilize-runtime.mjs`, `tooling-tests/fixtures/stabilize-test-stage.trace` · `REG` = `dev/regression-set`, `tooling-tests/dev_regression_set.rs`, `completions/artifacts/M55/stabilization-build/regression-set/` · `DOC` = `implementation/stabilization-workflow.md` · `LOG` = `DECISIONS.md`.

**Edits under `.claude/`.** The harness is under `.claude/workflows/`, and five tasks have it as their subject: they are a builder's, as every harness task of the first repair was (lesson 6: *a task whose subject is a harness or a definition is accepted*). **The agent definitions and the settings are not**: every edit to `.claude/agents/*.md` and to `.claude/settings.json` is cut out of the tasks as a step of the orchestrating session's own, `O1`–`O5`, each small, each its own gated commit. **No task and no step edits `CLAUDE.md`.**

| # | Goal | Writes | Done when | Must not touch | Size |
|---|---|---|---|---|---|
| **`T1`** | **`P1`, the two scripts.** `git-state` and `record` answer from one table of the states a record step can leave; the acts are repeatable; a push that is owed is made first; `discard` asks git | `STEP`, `REC`, `DOC`, `LOG`; an assertion of `stabilize_simulation.rs` only where it pins the old answer | `dev_stabilize_step::a_record_step_killed_before_any_of_its_commands_converges` — every child command of `record` and `push` and every file operation of `apply`, counted by an instrumented run; after each kill `git-state` and what it names, at most twice, end at `ready`, remote at the local head, clean tree, the state document of the control. Four mutants, each red, named in the commit: the settling by what `HEAD` holds, the staged lines, the temporaries, the push that is owed. And by name: a commit that failed is asked for again and commits (`R3`); a batch that changes nothing is refused at `apply` (`R2`); `discard` refuses a batch that is in a commit; a loop branch ahead by anything but record commits is the human's; a temporary is no report (`R4`); a branch that moved under the stage is refused. Review A's blocks `R2`, `R3`, `R4`, `R7` answer as it expected | the harness — every flag it passes today keeps working, a new one is optional until `T2`; what `state` computes; the `fix` acts beyond a helper they share | L |
| **`T2`** | **`P1`, the harness.** A stage starts from that answer; `stopAfter` is read before a pending record is finished; a halt after a commit says what the next invocation does; a tool step's retry runs the one command again | `HARN`, `DOC`, `LOG` | simulation, by name: a push that failed after the record is made by the next invocation, which then answers from the state (`R-M1`); `stopAfter` over a pending batch moves neither head (`R-L1`); a commit that failed is finished by the next invocation; a ruling sent twice is refused and leaves no batch; a commit step whose agent died once records on its retry — the stand-in learns *dies once* | the two scripts; `runFix` | M |
| **`T3`** | **`P2`.** Nothing is committed that the script would not have written | `REC` (the read), `STEP` (the act), one simulation scenario, `DOC`, `LOG` | `dev_stabilize_step::nothing_is_committed_that_the_record_script_would_not_have_written` — each refusal of a write × a `test` report, a fix cycle's report, a round's scope, and the subject; a staged blob that is not the vetted bytes; the mutant without the call, red. Review A's blocks `R1` and `R13` end in a refusal with the bare remote unmoved | what a writer checks at a write | S–M |
| **`T4`** | **`P5`, the item's row.** An item judges a clause of the census; an item with a result keeps what the derivation reads; a round's candidate is the commit its gate ran on | `REC`, `DOC`, `LOG` | `dev_stabilize_record::an_item_judges_a_clause_of_the_census_or_the_run_is_not_ready` (both orders of writing), `::an_item_that_has_a_result_keeps_what_the_derivation_reads` (each of the four cells refused, a brief taken, `next` still `retest`), `::a_rounds_candidate_is_the_commit_its_gate_ran_on`; three mutants, each red. Review A's blocks `R5` and `R6` answer as it expected; the header's two sentences (`R15`, first bullet) are true | the writers' table and its suite's table (**Fork 3**); the harness | S–M |
| **`T5`** | **`R8`, as Fork 2 is ruled — option A unless ruled otherwise — and the gate's counts.** A record in a round with no gate on record carries the candidate's own gate; a record's gate that ran fewer tests than the candidate's is refused | `REC`, `HARN`, `DOC`, `LOG` | simulation: a ruling recorded before a round's gate is on record, under a red candidate, ends `ruled`; `dev_stabilize_record::a_records_gate_that_ran_fewer_tests_than_the_candidates_is_refused`. Review A's block `R8` ends `recorded`; review B's gate lead answers `ok: false` | the ruling of 2026-10-06 itself: what is red and known is still accepted | S–M |
| **`T6`** | **`P4`.** The endings as one table the code consults and the simulation enumerates; the stage reaches its record | `HARN`, `DOC`, `LOG` | `stabilize_simulation::every_ending_of_every_reporter_is_a_row_and_is_driven` — one scenario per row, a row without one fails; `::once_an_instrument_is_launched_the_stage_reaches_its_record` — the set of halts by phase; by name: no hash is a void item (`R-H2`), a verdict without its regression fact is unverified, a stood-on reporter with no report halts before anything is launched (`R-M3`), two dead verifiers are two unverified findings on record (`R-M2`), a second pass under both keyings records (`R-M4`), a triage that does not balance still halts. The stand-in learns to leave a field out, to leave something open, and to die for one label after another | the scripts; the arithmetic of triage; the rule for a hash that is returned and wrong; the numbers of retries and of the breaker (`T11`) | L |
| **`T7`** | **`P3` a, c, d.** The state's line holds no free text and names its file; a stage returns the path; doors by a read of the record script; the doors nobody reaches computed; the previous binary held and on record | `STEP`, `REC` (two reads, one fact), `HARN`, `DOC`, `LOG` | `dev_stabilize_step::the_state_line_holds_no_free_text_and_stays_small` — 120 rows, 300 doors, 13 items: every string a slug, a sha, a path or a listed word, and the line under a bound the task states and justifies; the mutant that puts the ledger back, red. Simulation: a stage's return holds no `state`; a previous binary at another path, or with the candidate's hash, halts before the instruments. Review B's `f_size` measurement is flat over its four sizes | what `dev/stabilize-record state` prints — the document is unchanged; the record's payload (`T11`) | L |
| **`T8`** | **`P6`.** The regression tool reads its list from the candidate's commit, refuses one commit twice, excludes by row, builds the previous release from a clone | `REG`, `DOC`, `LOG` | `dev_regression_set`, by name: a list that is not in the candidate's commit is refused; two commits that are one are refused; a test that fails on its own binary and is on no row is void. Review B's two blocks end in refusals. **One real run** (about thirty-five minutes), its line committed with the record: which of the eleven still fail on their own binary, each of those a row | the swap and its proof; the harness | M |
| **`T9`** | **`P3` b, as Fork 1 is ruled.** *Option B:* a scripted check is an act — started, asked again in slices, its line relayed; the mapping of its exit status is the tool's; the harness holds the two commits, the status, the list's hash and the exclusions; the second preflight's prompt names the previous release. *Option A:* the brief, written once in the doc's section and copied by an opening; the second preflight's prompt | B: `STEP`, `HARN`, `DOC`, `LOG` · A: `HARN` (one prompt), `DOC`, `LOG` | B: `dev_stabilize_step`, by name: a check that is still running answers `running` and is not started twice; each exit status of the regression tool maps to its word; a line whose commits are not the run's is void. Simulation: a green whose list hash is on the round's record. A: the brief is in the doc, and a simulation scenario hands the second preflight a check | the regression tool; the gate | B: M–L · A: S |
| **`T10`** | **The doc and the headers.** Every sentence of review B's table and of `R15` is true or gone; the six things an orchestrator would get wrong are said where an orchestrator reads; the bounds §8 lists are declared with their reach; the canary's list is the union of §5 | `DOC`; the header comments of the three scripts and of the harness; `LOG` | the commit's own table: each of the sixteen rows, each bullet of `R15`, `R-L10` and each of the six, against *made true by `T<n>`* or *the sentence as it now reads*. `doc_link_fence` green. No sentence of the doc says `test` is fit | any behaviour | M |

**Then, in this order:** the re-review of `T1`–`T10` — focused on the six changes, with a mutant for each *done when* — · the canary's parts B and C (§5) · and the tasks whose cut waits for it:

| # | Goal | Cut after the canary because |
|---|---|---|
| **`T11`** | What the runtime showed: the number of retries and the breaker's threshold; the retried `begin`; whether a schema's `required` is asked for again; the record's payload and the triage return at the real run's size — and, **only if a payload failed**, entries that reach the disk through the record script from the agent that made them, so that the batch names them instead of carrying them; a trial arm past the ten-minute ceiling; the cap on concurrent agents; `R-L2` if a file of the run turned the candidate's gate red | each is a number or a design that the canary measures; built before it, it is a guess |
| **`T12`** | The doc's list *To be shown by the self-test under the runtime and by the canary*, item by item, from the canary's record | it is the canary's result |
| — | The record that the `test` half is repaired and re-reviewed — a `DECISIONS.md` entry, the orchestrator's | it lifts the rule that keeps the real run from starting, and it is the last thing |

**The steps that are the orchestrating session's own** — small, each its own gated commit:

| # | Edit | Closes | When |
|---|---|---|---|
| **`O1`** | `.claude/agents/finding-triage.md`: a missing repro block is said in the entry's `why` and in triage's report, and is no entry; the return names `new`, as the schema does | `R-M5` | before the canary |
| **`O2`** | `.claude/agents/milestone-code-reviewer.md`: the hash is read and returned whether or not the pass drives anything | `R-H2`, the definition's half | before the canary, with `T6` |
| **`O3`** | `.claude/agents/stabilize-preflight.md`: how a scripted check is run, as Fork 1 is ruled — the act and its slices, or the brief's pattern for a command of thirty-five minutes | `R-M8` | before the canary, with `T9` |
| **`O4`** | the three definitions a run launches that still name the `probe` step | `R-L9`, `L8` in part | any time |
| **`O5`** | `.claude/settings.json`: an allow rule for `dev/stabilize-step`, `dev/stabilize-record` and `shasum` — **only if the canary's part 0 shows a prompt, and only with the human's word**: it is a permission | the canary's item 9 | after the canary's part 0 |

**The bound on the repair** is the one the orchestrator reported on 2026-10-06 — at most three repair-and-review cycles, then back to the human. This is the second repair; the re-review after `T10` is the third review of this half.

## 5. The canary

**Why not as first sketched.** One door and one planted finding relays a state of three kilobytes, launches no reviewer on a source pass, runs no check of thirty-five minutes, leaves nothing open for a second triage pass and never fails a push. It would pass, and the real run's first stage would then meet each of those for the first time.

**What only a real invocation can show** — the union of both reports and of the builders' list (the workflow doc → *To be shown by the self-test under the runtime and by the canary*), in the order of what a failure would cost the real run:

1. That the runtime accepts the script; that an agent runs a step's one command and relays its line whole — **at the real run's size**, three times a stage; and how large a stage's return to the main session is.
2. Whether a subagent's first `dev/stabilize-step`, `dev/stabilize-record` and `shasum` each run without a permission prompt; and that the session stands in the repository it is meant to.
3. That a schema's `required` is enforced, and what the script is handed when a return lacks a field; that a reviewer on a pass that drives nothing returns the hash; a verifier's `confirmed` on a door the previous release does not have.
4. The first preflight's whole turn in one agent — two builds, the candidate's gate, the regression set, the packaged-tarball install — and a preflight on a commit that is not `HEAD`.
5. A verifier that leaves something open, so that a second triage pass runs; the triage return and the record's payload at the size the stage produces.
6. What the candidate's gate sees of the run's own files — the marker and the scope in the tree while it runs.
7. A report the record script refuses for its text, with the real scanners, and whether its agent rewords it and converges — or writes the file itself, which `P2` must then stop at the commit.
8. A push that fails after the record's commit, and the invocation after it; a candidate whose own gate is red, and its record commit.
9. How an agent really ends: what the runtime hands the script for one that died, after how many tries; whether one it gave up on writes its report afterwards; `begin` with a real agent, and its retry.
10. The human's side: an orchestrator that reads `next` and `human_stages`, brings a fork from the list with the return lost, records a ruling on a finding and a go; a granted re-run; the invocation that finishes a triage.
11. The cap on concurrent agents; a trial arm past the ten-minute tool ceiling; the preflight's status assert beside the scope step's writes; a cross-model pass with its real tool (**Fork 5**).
12. A stage killed from outside, and the fresh invocation after it. An invocation is never resumed by its run id.

**What a clone with a bare remote cannot show at all:** the check that reads CI for the candidate's commit. Its first run is the real run's, and the item is void in the canary by construction.

**Its shape — three parts, on the code the real run will use.**

- **Part 0 — minutes, and it can run today, beside `T1`.** In the clone: the self-test under the runtime, then `test` with `stopAfter: 'state'`. It launches two agents and shows items 1 and 2. Run now, on the tip, it measures the relay *before* `T7` changes it — which says whether `R-H1` is the HIGH its reviewer inferred — and it decides `O5`.
- **Part B — one whole `test` stage, round 1.** The opening as the real run's will be drafted: the stop mode `every-round`, the previous release and its commit, the scope `delta` — so that the scope step derives the real delta since `1.0.0-rc.24` and the scope is the real run's size by itself — the clauses of the exit rule, and **one item of every kind the harness has a chain for** (`review-row`, `audit-area`, `audit-cross-cutting`, `audit-drive`, `trial-arm`) beside the checks (the gate, the regression set, the tarball; CI, void). The ledger is seeded at the real run's size — the fix pass's ledger file has some 560 table rows — **with rows that are already settled**, so that the document is full-size and triage is handed only the planted ones. Planted, each as a *state* set from outside and never as a sentence to an agent: a commit on the canary's loop branch before the opening that adds one failing tooling test, so the candidate's gate is red by one named test (item 8); a hook in the canary's bare remote that rejects the first push (item 8); a synthetic denylist whose one term stands in a file an item's reviewer will quote (item 7); an untriaged row on a door that `1.0.0-rc.24` does not have (item 3); an untriaged row whose repro block shows a second defect (item 5 — likely, not certain).
- **Part C — the invocations after it.** The same arguments again after the rejected push; the finishing triage if `next` asks for it; a ruling on a finding; the go; a granted re-run if an item came back void; and one stage killed during its instruments, then invoked again.

**What must exist before parts B and C.** `T1`–`T10` landed and re-reviewed, `O1`–`O3` made. A check item for the regression set that a preflight can run — the act (`T9`, option B) or the brief (option A): **without one, the canary cannot show item 4**, and the check alone is about thirty-five minutes. The opening's data as files: the items, the seeded ledger, the clauses. The trial image buildable on the machine. `node`. And a session whose working directory is the clone (**Fork 4**).

**How it is kept from this repository's remote.** (1) The clone's only remote is a bare repository inside the canary's own root; that is asserted before every part — `git remote -v` names one remote, a path under that root, and no `url.*` rewrite is configured. (2) **The canary's run slug and loop branch exist in the clone only.** A stage launched from the wrong directory then halts at its first step — `wrong-branch`, one agent, nothing written: driven for this plan (§1). (3) `gh` resolves no repository from a local remote. (4) Nothing of the canary's tree comes back. What it showed is written afterwards as a record on this branch, aggregated, with no transcript.

**What it costs.** Part 0: about ten minutes, two agents. Part B: about two and a half to three hours of wall time — the first preflight alone is some seventy to eighty minutes (two release builds, a gate of thirteen to twenty minutes, the regression set, the tarball), then the instruments side by side, triage and its verifiers, and the record's own gate — and some fifteen to twenty-five agents, a handful of them long. Part C: four or five short invocations, each as long as one full gate — an hour to an hour and a half. **Tokens were not estimated.** The machine runs a full gate at least six times over the canary, so no builder's gate runs beside it without both slowing.

**Before it, and after it.** Before: `T1`–`T10`, `O1`–`O4`, and the re-review — the canary is worth its hours only on the code the real run will use. Part 0 is the exception and wants nothing. After: `T11`, `T12`, `O5`, and the closing record.

## 6. The forks — each is the human's

**Fork 1 — how a scripted check is run and judged.** The regression set is a `check` item, handed to the preflight *by its brief, with no change to the harness* (the workflow doc → The regression set). No brief exists. The run is about thirty-five minutes inside one agent's turn. The mapping of its exit status to green, red and void is the agent's reading, and the tool's proof — the two commits, the list's hash, the hashes around each run, what was excluded — reaches no script and no table (`R-M8`).
- **A — the brief.** Written once in the doc, copied by an opening. Cost: about an hour, no task of size. The verdict stays an agent's; the canary shows whether a preflight holds the run.
- **B — an act of the step tool.** It starts the check, is asked again in slices, relays the tool's line; the mapping is the script's and the harness holds the proof's fields. Cost: one task, `T9`, M–L, and the sentence *with no change to the harness* is given up. CI and the tarball stay briefs.
- **C — A now, B if the canary shows trouble.** Cost: the canary's most expensive item is then run on a path the real run may replace, and that path gets a canary of its own.

*Recommendation: B.* It is the third lesson of the first repair, applied to the one check whose whole point is a proof.

**Fork 2 — what a record commit is held to where its round has no gate on record.** The ruling of 2026-10-06 accepts a record commit whose gate turns nothing red that *the candidate's own gate* did not show. Before a round's first record there is no such gate on record, so under a red candidate nothing can be recorded there — the human's grant of one more attempt among it (`R8`).
- **A — the candidate's gate is run there.** The record step gates the tree before it applies its batch, and the batch carries that gate. Cost: one more full gate, thirteen to twenty minutes, in a state that is rare. The ruling holds as worded.
- **B — the gate of the candidate the run tested last.** No cost in time; before round 1 has a record there is none, and the state stays unrecordable exactly where it was found.
- **C — declared.** The way out is a batch composed by hand.

*Recommendation: A.* `T5` builds it unless ruled otherwise.

**Fork 3 — the writers' table as the code path, now or with the `fix` half.** `R9`, `R12` and `R14` are LOW, each on the safe side or a direct caller's. Their class closes by rebuilding how the record script takes a write: one table the code goes through, one grammar per cell. That is a large task in the file and the suite the first repair worked hardest on.
- **A — now**, as a task after `T4`. Cost: L, and the fixtures of an 11,500-line suite. The doc's bounds at lines 460 and 466 close.
- **B — with the `fix` half**, which adds writers and facts to that table anyway (`C2`'s `fixes` and `acts`). Cost: the three stay declared through the first real `test` stage.
- **C — patch the two instances of `R12` now.** This is the third patch on the same logic.

*Recommendation: B.* Not C.

**Fork 4 — where the canary's session runs.** Nothing in an invocation names the repository: a stage runs wherever its session stands (`R-L8`).
- **A — a second session, started by the human in the clone**, with a prompt the orchestrator hands him. Cost: his few minutes, and he relays what it returns. No mechanism.
- **B — a `root` argument to the harness.** The step tool finds its repository from where it lies, so a git step named by an absolute path is safe. A role agent is not: its shell starts in the session's directory at every call, and one that forgets to change it reviews, gates or commits in this repository. Cost: a task, and a guarantee that is not one.

*Recommendation: A.* `R-L8` is then declared, with the rule that a canary's slug and loop branch exist in its clone only.

**Fork 5 — a cross-model pass in the canary.** Its real tool on one named item is on the canary's list. The human's standing word is that the tool is used only when he asks.
- **A — named on one item of part B.** It shows the pass and each of its ways to fail.
- **B — left out.** The pass stays shown by the simulation alone until a run names it.

*Recommendation: none — it is his word either way.* The plan needs it for nothing.

## 7. What the `fix` half's repair owes — the rows this plan adds

1. **Its git acts are rows of `P1`'s table.** `land`, `carry`, `open-round` and the part's revert each get their intent, their done and what the reconcile finishes, and the test that kills before every command enumerates them. `landed-before` is derived there (`H5`).
2. **`F2`, with the suite that certifies it.** The three truth-table tests *require* the close over a fix with no counted cycle (review A's mutant `m18`): the repair turns the cell and replaces the oracle that review A cites at `dev_stabilize_record.rs:5838` and `:7239` (`S1`, `S2`) — lines this plan did not read.
3. **A dropped round** is said where a stage reads its work from, the scope step is handed no door list of fixes that never landed, and whether a hunt runs is the script's (`R10`).
4. **The writers' table as the code path, and one grammar per cell** (`R9`, `R12`, `R14`) — if Fork 3 is ruled B.
5. **The audit of a fix diff reads the diff of the regression set's list**: a row nobody ruled is held to nothing else (`R-M6`).
6. **The four sites of `runFix` that say *was not pushed***, and the sentence its halts end with (`R-M1`'s class; the doc's bound at line 469).
7. **`raise` beside the recorded `cycles`** (`L5`), and **a fixer's fork has no row** — both owed already, restated so that the list is whole.

## 8. Declared, not fixed — the bounds `T10` writes into the workflow doc

- At a clause that is the human's, the position still hands a `test` stage the next round; `next` does not ask for it (`R11`).
- The candidate's gate runs on a tree that holds the attempt's marker and may hold the round's scope and its report (`R-L2`).
- A stage runs in whatever repository its session stands in; a canary's slug and loop branch exist in its clone only (`R-L8`).
- The self-test checks the script's tables; what a stage does is the simulation's to show (`R-L6`).
- `dev/gate --quick` prints a gate's verdict line and no totals line (`R-L7`).
- A step that begins an attempt, run twice, counts the attempt (`R-L5`, for `begin`).
- While the opening is not done a direct caller can write a whole run; a writer can take a cell its reader refuses; a writer's place is written by hand (`R9`, `R12`, `R14`) — until Fork 3's task.
- A report written for an attempt nobody began counts as an attempt and is committed with the next record; which reporters an attempt launched is in no committed table; a scope is written once, and a human's scope on a second attempt is not applied.
- A row of the regression set's list is held to a pointer that exists, and to nothing that says who ruled it.
- How many findings one stage's record can carry is not measured before the canary.
- What keeps a real `test` stage from starting is the record alone: the harness refuses `fix` and not `test`.
- Agent-written text reaches other agents' prompts and no shell; an invocation is never resumed by its run id.

## 9. What this plan did not do

- **It built nothing and ran no agent.** Every *done when* above is a test that does not exist yet; its name is the plan's proposal, and a builder that finds a better seam keeps the fact and may change the name.
- **It did not measure the tasks.** The sizes are a reading of the code, not an estimate anyone timed; ten tasks are at least ten full gates.
- **It did not verify the canary's plants.** That a seeded row whose block shows a second defect makes a verifier leave something open is likely, not certain; that the trial image builds on this machine was not checked.
- **It read the harness in the parts the findings cite**, about a fifth of it, and the record script likewise. A cause that lives only in what was not read is not in §2.
- **Three LOW findings were not checked** (`R16`, `R-L2`, `R-L6`), and one traced case of `R-M4` was not driven.
