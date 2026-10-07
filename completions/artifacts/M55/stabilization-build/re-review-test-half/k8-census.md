# The census of the second repair — every finding, and where it went

*Written 2026-10-08 by the builder of the plan's task `K8`, on `fix/rc24-tier1` at `6d30208f`. It is the table the plan asks that task for ([plan.md](plan.md) → §4, `K8`, *done when*): what was found, against **closed by a commit**, **a sentence of the workflow doc as it now reads**, **a declared bound**, or **a row with its due point**. It restates no finding: an id is the report's, and its repro is the block in that report.*

## How this was counted

**What was read, line by line:** every finding, lead and closing section of [records-state.md](records-state.md) (review A) and [harness-doc.md](harness-doc.md) (review B); [plan-review.md](plan-review.md) whole; the plan's §7, §9, §10 and §11; [probes/README.md](probes/README.md); and, for every commit of the core — `1d21024e` `K0` · `e76ef470` `K1` · `b34fb073` `K2` · `3c6a889d` `K4` · `8f13ea7b`, `22b0be47`, `b36b8c93`, `86784c28` `K5` and `K6` · `8c0513e5` `K7` · `c2169e19` `K9` · `bbe873eb` `K10` · `a99b7639` `K3` · `a182c92e` `K11` · `083d4745`, `6d30208f` the definitions · `106c5179` `X1` — its message and the *Left open* paragraph of its `DECISIONS.md` entry, and the three merges `b33bf39b`, `249161b5`, `047a8ab6` for what each says read stale after it.

**What one line of the tables is:** one finding, one bullet of a finding that has several, one lead, one entry of a closing list of a report, or one item of a builder's *left open* list. A finding both reviewers made is two lines, one per report. A line whose subject is another line's says so (`=`).

**The last column begins with one word**, and the totals at the end count that word:

- `closed` — the defect is gone, or the question is answered, by the commit named. Nothing is owed.
- `said` — nothing in the code was wrong or changed here; a sentence of the workflow doc was false or missing and reads true since this task's commit.
- `bound` — declared: the number is its place in `implementation/stabilization-workflow.md` → *The declared bounds of the first run*. A bound that also has a row names it.
- `row` — a row of `implementation/decisions-pending.md` → *The `test` half's second repair — what it left as rows*, by its key, with its due point: **C** before a stage runs with real agents, the canary included · **12** between rounds 1 and 2 · **S** the first stop · **F** with the `fix` half's repair · **L** a later release's run.
- `canary` — a fact only a run with real agents shows; it is on the workflow doc's list *To be shown by the canary, and by round 1*, and is no defect to fix.
- `human` — a question that is the human's, listed in the workflow doc under *Open, the human's to rule*.
- `none` — no finding: a statement of a review's scope, a thing checked and found correct, or a point a ruling settled. The reason is given.

**What was not verified by this task** is said where it applies, and collected at the end.

## 1. The workflow doc — review B's sixteen sentences, review A's `R15`, `R-L10`, and the six things

*The doc line is its line at `cbb3d736`, as review B cites it.*

| | The sentence, or the point | Where it went |
|---|---|---|
| 417 | *Nothing on it is the `test` stage's own* | closed `b34fb073`, `a99b7639` — a record that is committed and not pushed is pushed by the next invocation's first read; the sentence is gone (*Where the two halves stand*) |
| 314 | *send the same invocation again … holds for a halt of a record step* | closed `b34fb073`, `a99b7639`; the paragraph now reads *A halt says what the next invocation does*, with the one case where the same arguments are refused |
| 100 | *checked out, clean, and pushed* | closed `b34fb073` — the first read makes the owed push, and refuses over a commit that is no record's; the sentence was rewritten with it |
| 131 | `stopAfter`: *No record step runs and nothing is committed* | closed `a99b7639` for `state`, which only looks; said for a later stop: it reconciles first, and commits a batch that is pending |
| 113–124 | the table *How an agent can end*, and *a hash … halts the stage* | closed `106c5179` for a hash that is absent or wrong; the rows of the two preflights and the scope step gained what the report check finds late; the second death is *Not on the list* |
| 108 | the first report check is where the stood-on reports are first looked for | said, in that table; row `wf-r-m3` (12) |
| 133 | *What comes back* — and the whole state document | closed `a99b7639`; the paragraph was rewritten: counts and ids, and the file for the rest |
| 106 | *the candidate's own full gate, once* — on a tree with the run's pending files | bound 13; row `wf-r-l2` (L) |
| 172, 177 | *left open, included* · *alternate … `VERIFY_PASSES`* — no test runs a second pass | said, under *The simulation of a stage*; row `wf-r-m4` (12) |
| 343 | *an agent of every kind that dies, halts, writes and dies, or returns with no report* | said: *in at least one of its endings … not every ending of every kind* |
| 353 | *What it does not establish* lacks three things | closed `a99b7639` for a relay that fails, which the stand-in now does; said for the other two; rows `wf-r-m4`, `wf-r-m2` |
| 375 | *a committed file that the tool reads* | closed `8f13ea7b`; the paragraph was rewritten |
| 382 | *An archive is no repository … excluded, named* | closed `b36b8c93` — a row each, no tolerance; said: the number, and that one reaches the binary; row `wf-regression-clone` (12) |
| 386 | *about twenty-five minutes* | said: twenty-one to thirty-four minutes, from the three records |
| 386 | *the two commits and the scratch root are in the preflight's prompt already* | closed `bbe873eb`, `a182c92e` — no brief and no preflight: a held check |
| 13 | *(`go`, `rerun`, `rounds`)* — `R-L10` | said: the six |
| `R15`·1 | the header: *nothing removes an item of the test set* · *anywhere: add work or record a ruling* | closed `3c6a889d` in what the script does; the header's two sentences were not re-read by this task |
| `R15`·2 | *nobody writes any of it by hand … stops hard on the scan* | closed `e76ef470`, which added the sentence that follows it |
| `R15`·3 | *pending, never a dirty tree* | closed `b34fb073` — temporaries and the commit step's own index have their rows |
| `R15`·4 | *invoke the stage again … commits a batch that is pending* | closed `b34fb073`, `a99b7639` |
| `R15`·5 | the bound on the opening is wider than stated; *the step suite drives it* | said: the bound's bullet now says any round's, a fix cycle and a bound; `b34fb073` gave `record` its own tests in that suite, `begin`'s were not looked for; row `wf-writers` (F) |
| `R15`·6 | *close is reachable only from a tested candidate* | row `wf-f2` (F); *Close* says the second state is not repaired |
| six·1 | after a halt at `push` they would read `refused` and take the record for pushed | closed `b34fb073`, `a99b7639` |
| six·2 | they would open the canary with one planted finding | said: *The small canary* replaces that description; sizes were the probes' |
| six·3 | they would write the regression set's item with a brief of their own | closed `bbe873eb`, `a182c92e` — kind `held-regression`, no brief |
| six·4 | they would use `stopAfter: 'state'` to look, and commit and push | closed `a99b7639` |
| six·5 | they would expect a second dead verifier to be one more unverified finding | said, in two places; row `wf-r-m2` (12) |
| six·6 | they would read the stage's return whole | closed `a99b7639` |

## 2. Review A — the record script and the state machine

| | Finding | Where it went |
|---|---|---|
| `R1` | a file at a report's path is committed and pushed unscanned | closed `e76ef470` |
| `R2` | an applied batch that outlives its commit | closed `b34fb073`, `a99b7639` |
| `R3` | a failed commit leaves a staged tree | closed `b34fb073` |
| `R4` | the temporaries of a killed write | closed `b34fb073` |
| `R5` | an item whose clause the census lacks | closed `3c6a889d` |
| `R6` | `item-set` replaces an item that has results | closed `3c6a889d` |
| `R7` | a record committed and not pushed | closed `b34fb073`, `a99b7639` |
| `R8` | no gate on record, under a red candidate | row `wf-r8` (12) |
| `R9` | the script takes a whole run while the opening is not done | row `wf-writers` (F); bound 15 |
| `R10` | after a dropped round only an agent decides that no hunt runs | row `wf-r10` (F) |
| `R11` | at a clause that is the human's the position hands `test` round R+1 | bound 12; row `wf-declared-later` (L) |
| `R12` | two inputs taken, or refused in the wrong class | row `wf-writers` (F); bound 15 |
| `R13` | a record commit's subject is not scanned | closed `e76ef470` |
| `R14` | the writers' table is a census of names | row `wf-writers` (F); bound 15 |
| `R16`·1 | no kill of the commit step, no failed commit, no batch that changes nothing, no changed report | closed `e76ef470`, `b34fb073` |
| `R16`·2 | `S1` and `S2` stand: the suite requires the `F2` close | row `wf-f2` (F) |
| `R16`·3 | the test named for the go stays green when a go stands for ever | none — as the report says, the go is held by `a_stop_is_lifted_by_its_go_and_by_no_other_write`; named here, as the plan asks |
| lead | a round's gate and its candidate are not held to one commit | row `wf-r8` (12) |
| lead | which reporters an attempt launched is in no committed table | bound 14; row `wf-declared-later` (L) |
| rests·1 | the path rule's defects bear on `test` | row `wf-m8` (F) |
| rests·2 | every round after the first: `F2` | row `wf-f2` (F); bound 22 |
| rests·3 | the reconcile that `R7` lacks | closed `b34fb073` |
| `F2` | a fix with no counted cycle leaves the candidate current | row `wf-f2` (F) |
| `F10` | `admitted` is not inherited | row `wf-declared-later` (L); declared in the older list, *true as built* |
| `F13` | the header claims what the code does not do | = `R15`, above |
| crash rows | the cycle record, the drop, the part | row `wf-fix-acts` (F) |
| n.e.·1 | the `fix` half's acts: read, not driven | row `wf-fix-acts` (F) |
| n.e.·2 | the harness, the definitions, the gate, the regression tool | none — review B's part |
| n.e.·3 | no real gitleaks and no real denylist | closed `e76ef470` in part: one arm runs the machine's own gitleaks over a real range; the denylist in the suites is one synthetic term |
| n.e.·4 | concurrency: two writers, a reader during a write | canary — and bound 7: `parallel` is serial in the simulation. Nothing has raced them |
| n.e.·5 | `placed_executable` was not run in a Linux container | none — not examined since; CI's Linux job is what runs it |
| n.e.·6 | the golden trace was not adjudicated; the nextest mark | none — the trace gained eight lines at `a182c92e`, each adjudicated in that entry |
| n.e.·7 | `scope.md` as a member of `R1`'s class; a sweep for `R12`'s class | closed `e76ef470` for the scope; row `wf-writers` (F) for the sweep |

## 3. Review B — the harness, the agents, the gate, the regression tool

| | Finding | Where it went |
|---|---|---|
| `R-H1` | the state document relayed whole — observed: never whole | closed `c2169e19`, `a99b7639`; an agent's own return at size: row `wf-return-size` (12) |
| `R-H2` | one field left out halts the stage | closed `106c5179` for the hash, `6d30208f` for the verifier's definition; rows `wf-x1b` (12), `wf-o2` (C) |
| `R-M1` | after a rejected push the same invocation pushes nothing | closed `b34fb073`, `a99b7639`; the `fix` stage's four sites: row `wf-runfix` (F) |
| `R-M2` | two dead reporters halt the stage | row `wf-r-m2` (12); bound 11 |
| `R-M3` | the stood-on reporters are checked late | row `wf-r-m3` (12); bound 11 |
| `R-M4` | a second pass keyed to a verified finding halts at the record | row `wf-r-m4` (12); bound 11 |
| `R-M5` | the triage definition against the arithmetic | row `wf-r-m5` (C) |
| `R-M6` | green over a list no commit holds, and over one commit twice | closed `8f13ea7b`; who adds a row: row `wf-r-m6` (F), bound 16 |
| `R-M7` | one test in a hundred drops out | closed `b36b8c93`; the clone: row `wf-regression-clone` (12) |
| `R-M8` | a check's verdict is the preflight's reading | closed `bbe873eb`, `a182c92e` for the gate and the regression set; row `wf-r-m8` (12), bound 3 |
| `R-L1` | `stopAfter` over a pending batch commits and pushes | closed `a99b7639` for `state`; said for a later stop |
| `R-L2` | the candidate's gate runs on a tree with the run's pending files | bound 13; row `wf-r-l2` (L) |
| `R-L3` | the previous release's binary is known by a version string | closed `bbe873eb`, `a182c92e` — a build is a held command, and its path, hash and version are the tool's |
| `R-L4` | a door no item reaches is known from a return alone | closed `3c6a889d`, `c2169e19`, `a99b7639` |
| `R-L5` | a one-command step wrapped in a retry that says look around | closed `a99b7639` for `record`; `begin`: bound 14, row `wf-declared-later` (L) |
| `R-L6` | the self-test passes a harness the simulation catches | bound 23; row `wf-declared-later` (L) |
| `R-L7` | `dev/gate --quick` prints `GATE: PASS` | closed `a182c92e` for what an agent reads; the line: bound 23, row `wf-declared-later` (L) |
| `R-L8` | nothing in an invocation names the repository | bound 9; row `wf-declared-later` (L); the canary's guards: `8c0513e5` |
| `R-L9` | three definitions still name the `probe` step | row `wf-r-l9` (C) — a note on the older row *Eight agent definitions still describe the gate as running a `probe` step* |
| `R-L10` | the doc names three of six rulings about the run | said — section 1 |
| lead | a report written for an attempt nobody began | bound 14; row `wf-declared-later` (L) |
| lead | the gate comparison holds names, never counts | bound 17; row `wf-declared-later` (L) |
| lead | a commit made on the loop branch while a stage runs | closed `b34fb073`, `a99b7639` — `head-moved` |
| lead | a human's scope on a second attempt is not applied | bound 14; row `wf-declared-later` (L) |
| lead | agent-written text in other agents' prompts | bound 23; row `wf-declared-later` (L) |
| lead | `resumeFromRunId` after a kill | bound 21; row `wf-declared-later` (L) |
| `H1`–`H4`, `M1`, `M12` | the landing, the part, the triage inside a running `fix` | row `wf-fix-half` (F) |
| `H5` | `landed-before` reports landed, unchecked and unpushed | row `wf-fix-acts` (F) |
| `M7` | reviewers get no binary — closed, and worse in one respect | = `R-H2` |
| `M8`, `L12` | the path rule's limits; a run slug ending `-r<N>` | row `wf-m8` (F) |
| `M9` | two dead agents | = `R-M2` |
| `M10` | no test executes `runFix` | row `wf-runfix` (F) — the simulation asserts its refusal and nothing else |
| `L2` | `halt()` always says *the same args* | closed `a99b7639` |
| `L5` | `raise` is not a recorded fact | row `wf-fix-half` (F) |
| `L6` | two concurrent invocations — closed by reading, not driven | none — not driven since |
| `L8` | definitions still name a `probe` step | = `R-L9` |
| cells·1 | a second preflight that returned and left no report halts | said, in the table; row `wf-r-m3` (12) |
| cells·2 | an absent hash halts | closed `106c5179` |
| cells·3 | a confirmed verdict without its regression fact halts | row `wf-x1b` (12); it is a row of the stated list |
| cells·4 | the second death of an invocation halts | = `R-M2` |
| cells·5 | an agent that returns twice has no cell | none — the runtime hands one return, and its two shadows are refused where they could occur |
| sim·1 | never taken: a relay that fails | closed `a99b7639` |
| sim·2 | a call after the breaker tripped; an agent call that throws | rows `wf-r-m2`, `wf-b4` (12) |
| sim·3 | a second triage pass | row `wf-r-m4` (12) |
| sim·4 | a push that fails after a commit; a state not read back after one | closed `a99b7639` for the push; the read-back is a row of the stated list |
| sim·5 | a report nobody launched, inside a stage | row `wf-b4` (12) |
| sim·6 | a record whose executor halts | row `wf-b4` (12) |
| sim·7 | a preflight that returns the wrong thing | closed `a182c92e` — a binary's facts are the tool's, and the preflight returns none |
| sim·8 | triage's counts, a key that is no slug, a verifier without its fact, an advocate's hash | closed `106c5179` for every hash; rows `wf-x1b`, `wf-b4` (12) for the rest |
| sim·9 | a second round | canary — no stage of any suite has a round before it; bound 22 |
| sim·10 | an unknown kind, a name that cannot be made, `stopAfter` at `instruments` and `triage` | row `wf-b4` (12) for the name; the others not examined since |
| sim·11 | a scripted reporter always returns the hash; every relayed line is small | closed `1d21024e`, `106c5179`, `a99b7639` |
| must·1 | a state and a payload at the real run's size | closed — the probes of `1d21024e`, run and recorded |
| must·2 | a reviewer on a pass that drives nothing; a `confirmed` on a door the previous release lacks | canary; `6d30208f` for the verifier's sentence |
| must·3 | a `check` run from its brief by a real preflight; its whole turn | closed `bbe873eb`, `a182c92e` for the long commands; the rest canary |
| must·4 | a verifier that leaves something open | row `wf-r-m4` (12) — the small canary does not show it |
| must·5 | a push that fails after the record's commit | closed `b34fb073`, `a99b7639` in the suites — the small canary does not show it |
| must·6 | what the gate sees of the run's own files | row `wf-r-l2` (L) |
| must·7 | a report the record script refuses for its text, with the real scanners | row `wf-b4` (12) — first met in round 1 |
| must·8 | the size of what a stage returns | closed `a99b7639` |
| must·9 | whether a subagent's first call of each tool runs without a permission prompt | rows `wf-o5`, `wf-probes-again` (C) |
| must·10 | where the canary runs | closed `8c0513e5` |
| must·11 | a clone cannot show the check that reads CI | bound 3 — its first run is the real run's |
| reg·1 | green when the old binary ran: no | none — checked and correct |
| reg·2 | green over two commits that are one | closed `8f13ea7b` |
| reg·3 | green over a list that is not the committed one | closed `8f13ea7b` |
| reg·4 | tests silently dropped at the baseline | closed `b36b8c93` |
| reg·5 | the exclusion is not named in a round's record | closed `bbe873eb` — the verdict's file is a file of the record |
| reg·6 | the real run was not driven | closed `22b0be47`, `86784c28` |
| gate·1 | `--keep-going` green over a red step: no | none — checked and correct |
| gate·2 | a pre-check mistaken for a gate by an agent | = `R-L7` |
| gate·3 | a red build is still a stop | none — the workflow doc says so under *The record step*, *Its bounds* |
| n.e. | the eight things review B did not examine | none, but two: `runFix`'s body, row `wf-runfix` (F); the regression tool's real run, closed `22b0be47`, `86784c28`. The gate's nextest path, three fence arms against the first review's mutants, and the doc's *Close* past its third step were not examined since |
| real·7 | `begin` with a real agent and its retry; a commit that is not `HEAD` | canary |
| real·8 | a cross-model pass with its real tool | canary — round 1's, the small canary names none |
| real·9 | how an agent really ends | canary — its one plant |
| real·10 | an orchestrator bringing a fork with the return lost | canary |
| real·11 | the cap on concurrent agents; a trial arm past the ceiling | canary |
| real·12 | a kill at each boundary; the replay of a resumed run | closed `b34fb073` for the record step's kills; bound 21 for the replay |

*Review B's `real`·1 to 6 are its own findings and `must` items again, and are counted there.*

## 4. The plan's review

| | Finding | Where it went |
|---|---|---|
| `B1` | the grant's gate collides with the granted stage's | row `wf-r8` (12) |
| `B2`, `B3` | the push is the irreversible act; a vet hit has no exit | closed `e76ef470` |
| `B4` | findings live in returns until the batch | row `wf-b4` (12); bound 11; `106c5179` states the list and removes the hash from it |
| `B5`, `B9` | the first canary: masked plants, no author | row `wf-canary-large` (L); the small canary is `8c0513e5` |
| `B6` | a tuning commit as unreviewed code | row `wf-t11` (S) |
| `B7` | tasks cut on unseen runtime facts | closed `1d21024e`, and the probes' run |
| `B8` | four *done when* defects | closed `b34fb073` (a, b), `bbe873eb` (c), `6d30208f` (d) |
| `B10` | a mistyped door | bound 1; named by `3c6a889d`, in every return by `a99b7639`; row `wf-never-selected` (12) |
| adv·1 | an index lock; an owed push under a look; an unpushed tuning commit | closed `b34fb073`, `a99b7639`; bounds 5 and 6 |
| adv·2 | the table keyed so that the `fix` half extends it | closed `b34fb073`; row `wf-fix-acts` (F) |
| adv·3 | refusal lines and red lists are relayed free text | closed `c2169e19` |
| adv·4 | a failed relay, a thrown call, a report nobody launched, an executor that halts | row `wf-b4` (12) |
| adv·5 | the count rule refuses every later record | none — the rule was dropped by the plan; what it answered is bound 17 |
| adv·6 | two runs, and a form for an exclusion row | closed `8f13ea7b`, `b36b8c93`, and the two records |
| adv·7 | when the allow rules are due | row `wf-o5` (C) |
| adv·8 | the harness's reads rewritten after tasks built on the old contract | closed `a99b7639` — the order was recut so that it changes once |
| fork 1 | how a scripted check is run | closed `bbe873eb`, `a182c92e` — the hold probe and the human's ruling settled it |
| fork 2 | what a record is held to where its round has no gate | = `B1` |
| fork 3 | the writers' table now, or with the `fix` half | row `wf-writers` (F) — the ruling on scale answers it |
| fork 4 | a `root` argument; a setup run from the wrong directory | closed `8c0513e5`; the session: row `wf-canary` (C) |
| fork 5 | a cross-model pass in the canary | none — the small canary names none |
| buried·1 | the canary's size | none — ruled: (B) |
| buried·2 | what becomes of a product finding the canary makes | row `wf-canary` (C) |
| buried·3 | the opening drafted before the canary | none — fell away with the plan that was not chosen |
| buried·4 | the automatic push of an owed record | closed `b34fb073` — built as the planner's choice, open to overturn |
| buried·5 | a wrong hash halting *as ruling 11 has it* | closed `106c5179` — it voids, and the attribution is withdrawn |
| buried·6 | resume by run id, narrowed to never | bound 21 |
| buried·7 | accepting the bounds for a public first run | human |
| n.c. | what the plan's review did not check | none — a statement of scope |

## 5. The probes, and what they found beside what they were built for

| | Finding | Where it went |
|---|---|---|
| probe | a workflow invoked by its name runs the copy the session loaded first | said — *Invoking the harness*; bound 9; row `wf-by-name` (C, L) |
| ruling | a step of a stage asked the human for a permission | closed `bbe873eb`, `a182c92e`, `083d4745`, `6d30208f` in what a prompt names; rows `wf-o5`, `wf-probes-again` (C) for what shows it |
| `SP-1` | a failure that is no refusal prints a traceback | row `wf-sp` (L) |
| `SP-2` | only the scratch root is resolved | closed `8c0513e5`, `bbe873eb` for the class; the probe tool itself: row `wf-sp` (L) |
| `SP-3` | two verdicts judged on the last try only | row `wf-sp` (L) |
| `SP-4` | the header names three of four copied files | row `wf-sp` (L) |
| `SP-L1` | a `held` in the microseconds before a hold's record is written | row `wf-sp` (L) |
| left·1 | whether a line of pure ASCII is relayed whole | closed — the relay probe on the digest, whole twelve of twelve, recorded with `bbe873eb` |
| left·2 | the size of an agent's own return | row `wf-return-size` (12); bound 18 |
| left·3 | how an agent that dies reaches the script | canary — its one plant |
| left·4 | whether an allow rule stops the prompt | rows `wf-o5`, `wf-probes-again` (C) |
| left·5 | a payload written with the file tool | row `wf-probes-again` (C) |

## 6. What each builder left open

| | Left open | Where it went |
|---|---|---|
| `K0`·1 | that an agent of each role takes a probe's prompt | closed — the probes ran |
| `K0`·2 | whether the probe tool runs in a subagent without a permission prompt | row `wf-o5` (C) |
| `K0`·3 | a detached hold asked for a second time while it runs | closed — the `hold` probe's second arm, 22 reads |
| `K0`·4 | a hold killed and not reaped reads as `running` | row `wf-step-edges` (L) |
| `K0`·5 | the durations of `relay` and `payload` are not measured | said — the probes' table says *not measured* |
| `K1`·1 | a push made by hand | bound 4 |
| `K1`·2 | what a push would publish is read off the remote-tracking refs | row `wf-step-edges` (L) |
| `K1`·3 | a file that changes and changes back | row `wf-step-edges` (L) |
| `K1`·4 | setting aside an earlier attempt's only file lowers the attempt | row `wf-step-edges` (L) |
| `K1`·5 | four harness edits; `discard` does not ask git | closed `a99b7639`, `b34fb073` |
| `K2`·1 | a kill inside a git child beyond the lock | bound 6 |
| `K2`·2 | the `fix` half's acts and its owed pushes | row `wf-fix-acts` (F) |
| `K2`·3 | the kept answer is lost when the scratch root changes | bound 20; row `wf-step-edges` (L) |
| `K2`·4 | a straggler's report after the commit refuses `dirty` | row `wf-step-edges` (L) |
| `K2`·5 | `begin` run twice counts the attempt | bound 14 |
| `K2`·6 | the harness uses none of the new flags | closed `a99b7639` |
| `K4`·1 | the bound, and the stricter rule the task's brief asked for | bound 1; human; row `wf-never-selected` (12) |
| `K4`·2 | a door that differs by a space; an item deleted by hand | row `wf-step-edges` (L) |
| `K4`·3 | two tables changed by hand consistently | none — declared in the older list, one member closed |
| `K4`·4 | no return names either list | closed `a99b7639` |
| `K5`·1 | the previous release is still built from an archive | row `wf-regression-clone` (12) |
| `K5`·2 | who may add a row is held by nothing | row `wf-r-m6` (F); bound 16 |
| `K5`·3 | what the two docs must now say | said — this task; the stale rows were corrected with it |
| `K5`·4 | a probe arm that compared `1.0` with `1` | closed `c2169e19` |
| `K7`·1 | an invocation's `scope` is not held by the harness | said, in three places; bound 10 |
| `K7`·2 | who starts the canary's session | human; row `wf-canary` (C) |
| `K7`·3 | what the tasks then in flight change in the tool | closed `bbe873eb`, `a182c92e` |
| `K7`·4 | nothing verified with a real agent; the wall time is an estimate | row `wf-canary` (C) |
| `K9`·1 | whether a real agent relays a digest whole | closed — `left`·1 |
| `K9`·2 | thirteen of twenty-six refusals through the flag; the shapes printed by no read | row `wf-step-edges` (L) |
| `K9`·3 | a blocker's row has no read | row `wf-runfix` (F) |
| `K9`·4 | the probe tool keeps its four LOW findings | row `wf-sp` (L) |
| `K10`·1 | the canary's default opening cannot run until the harness holds a command | closed `a182c92e` |
| `K10`·2 | whether an allow rule stops the prompt, and a file tool needs none | rows `wf-o5`, `wf-probes-again` (C) |
| `K10`·3 | a dead job's command may run on; two gates contend; a work directory stays | bound 19; row `wf-step-edges` (L) |
| `K10`·4 | a held gate needs an opened run; five void reasons are not recorded lines; four error paths have no arm; a range's vet holds a verdict to its name | row `wf-step-edges` (L) |
| `K10`·5 | `item-set` can change a held kind; a caller can compose a verdict's file; `state --scratch` | row `wf-writers` (F) |
| `K3`·1 | a check that was void in its round is not run again inside it | bound 2; rows `wf-held-void` (12), `wf-m8` (F) |
| `K3`·2 | whether a real agent runs the read its prompt names | canary; bound 7 |
| `K3`·3 | five sentences of five definitions | closed `083d4745` |
| `K3`·4 | the executor's `refused` is an agent's reading; the probe tool's lines are no digests | row `wf-step-edges` (L) |
| `K3`·5 | a red check's row no longer names its first door | row `wf-keys` (12) |
| `K3`·6 | what a stage tells an agent to run | closed `a182c92e` |
| `K3`·7 | the `fix` stage is not converted | row `wf-runfix` (F) |
| `K11`·1 | the record script's void for a held check; a dead held check uncounted | row `wf-held-void` (12); bound 19 |
| `K11`·2 | a stale name and a dead start at once; no arm makes the tool lie about a build; a wait's second lap | row `wf-step-edges` (L) |
| `K11`·3 | six definitions grant no tool that writes a file | closed `6d30208f` |
| `K11`·4 | the cross-model pass, a proposal's build and a role's own craft can meet a prompt | bound 8 |
| `K11`·5 | the `fix` half's round-tip binary and its fixers' gates | row `wf-runfix` (F) |
| `K11`·6 | the definitions and the settings | closed `083d4745`, `6d30208f` for the definitions; row `wf-o5` (C) for the settings |
| `O7`·1 | whether the file tool's write under a scratch root asks for a permission | row `wf-probes-again` (C) |
| `O7`·2 | no fence holds a definition's wording to its prompt | row `wf-defs-fence` (12) |
| `X1`·1 | `X1b`, and a stand-in that leaves out a field beneath an object | row `wf-x1b` (12) |
| `X1`·2 | the definitions' halves: a reviewer's hash, a verifier's | row `wf-o2` (C); closed `6d30208f` for the verifier |
| `X1`·3 | a name that cannot be made halts after the held checks; findings still live in returns | row `wf-b4` (12) |
| `X1`·4 | a verifier's `contested` left out reads as not contested | bound 23 |
| `X1`·5 | the `fix` stage's two reads of a hash still halt | row `wf-runfix` (F) |
| merge·1 | `b33bf39b`: the regression set's row said *Neither is built* beside *the first is built*; its part 1 row still owed the build | said — both rows corrected by this task |
| merge·2 | `b33bf39b`: the row of the eleven fixes counted what was built before the merge | said — the heading and its last note corrected by this task |
| merge·3 | `b33bf39b`'s fixes log: the note on line 4, that the fold stops at its first blocked doc | closed `0ad081cd` — struck through, corrected and discharged before this task; read again, nothing changed |
| merge·4 | `249161b5`: the workflow doc's *The regression set* described the tool as it was | said — the section was rewritten |
| merge·5 | `047a8ab6`: the canary's header and entry named the digest as still to come | closed `bbe873eb`, `a182c92e`, which rewrote the header's *What it stands on* |
| merge·6 | `047a8ab6`: the workflow doc did not point at the canary's tool | said — *The small canary* |
| cost | the stabilization suites in the gate; a stand-in that outlives its test | rows `wf-gate-cost`, `wf-test-leaks` (S) |

## 7. The plan's own lists

**§7, every row that is not a core task's:** each is a line of sections 1 to 5 above under its finding's id — the plan's table is the union of the reports. Its one row with no finding behind it, *the doc builder's first item* (a stage stopped after its scope step), was closed before the plan and driven by it.

**§9, what the `fix` half's repair owes** — nine items: 1 `wf-fix-acts` · 2 `wf-f2` · 3 `wf-r10` · 4 `wf-writers` · 5 `wf-r-m6` · 6 and 7 and 8 `wf-runfix` · 9 `wf-fix-half`.

**§10, declared, not fixed** — twenty-two bullets, each a bound of the workflow doc's list or a part of one: a mistyped door 1 · a check that is no held command 3 · the fence's domain 8 · invoked by name 9 · `R11` 12 · `R-L2` 13 · `R-L8` 9 · a tuning commit 5 · git's lock 6 · a push by hand 4 · the self-test 23 · `dev/gate --quick` 23 · `begin` twice 14 · `R9`, `R12`, `R14` 15 · reports across attempts and a scope written once 14 · a row of the list 16 · names, never counts 17 · sizes 18 · the probe tool's findings 23 · resume by run id 21 · what keeps `test` from starting 22 · agent-written text 23. **Four bounds of the doc's list are not in §10**, and came from the builders: 2 (`K3`'s item 13, accepted by the orchestrator), 10 (`K7`), 19 (`K10`) and 20 (`K2`); and 7 and 11 state what the plan says in its §6.

**§4, the orchestrating session's steps:** `O1` row `wf-r-m5` · `O2` row `wf-o2` · `O4` row `wf-r-l9` · `O5` row `wf-o5` · `O6` and `O7` closed `083d4745`, `6d30208f` · the relay probe again, closed · the two probes again, row `wf-probes-again` · the bounds, put to the human: the workflow doc's list · the canary: row `wf-canary`. **The three optional tasks** `X3`, `X4`, `X5` are not built: rows `wf-r-m2`, `wf-r-m4`, `wf-r-m3`.

**The focused review of the core's diff** (`F1` to `F8`) is on another branch and is one row, `wf-core-review`, which that branch's merge discharges. It is not counted here.

## The totals

**245 lines** in sections 1 to 6. By the first word of the last column:

| | Lines |
|---|---|
| closed by a commit | 90 |
| a row | 79 |
| a declared bound | 23 |
| said — a sentence of the workflow doc, true since this task | 18 |
| canary | 10 |
| none, with its reason | 16 |
| the human's | 2 |
| the same as another line (`=`) | 7 |

*A line that is closed in part and has a row for the rest is counted once, by the first word of its last cell; the row is written all the same. The count was made by a script over the six tables, not by hand: 90 + 79 + 23 + 18 + 10 + 16 + 2 + 7 = 245.*

**The rows, by due point** — thirty-seven keys, the older row that `wf-r-l9` is a note on among them, and `wf-core-review` too: before a stage runs with real agents, **eight** (`wf-core-review`, `wf-o5`, `wf-probes-again`, `wf-canary`, `wf-r-m5`, `wf-o2`, `wf-by-name`, `wf-r-l9`); between rounds 1 and 2, **thirteen** (`wf-defs-fence`, `wf-b4`, `wf-r-m2`, `wf-r-m3`, `wf-r-m4`, `wf-x1b`, `wf-held-void`, `wf-r8`, `wf-r-m8`, `wf-return-size`, `wf-never-selected`, `wf-keys`, `wf-regression-clone`); the first stop, **three** (`wf-t11`, `wf-gate-cost`, `wf-test-leaks`); with the `fix` half's repair, **eight** (`wf-fix-acts`, `wf-f2`, `wf-r10`, `wf-writers`, `wf-r-m6`, `wf-runfix`, `wf-fix-half`, `wf-m8`); a later release's run, **five** (`wf-declared-later`, `wf-step-edges`, `wf-sp`, `wf-canary-large`, `wf-r-l2` — the last sooner if the canary or round 1 shows it).

## What this task did not verify

- **No command was driven.** Every *closed* above rests on the closing commit's own message and its `DECISIONS.md` entry — red first, its mutants, the reviews' blocks run again — and on this task's reading of the code at `6d30208f` where the workflow doc states behaviour. Nothing was re-run.
- **The scripts' and the harness's headers were not edited** — they are not this task's files — and two of their sentences are known stale: the harness's two usage lines invoke by name (`wf-by-name`), and the canary's header says the committed settings carry the allow rules (`wf-o5`). `R15`'s first bullet, two sentences of the record script's header, was not re-read.
- **Three numbers are from the orchestrator's brief and from no committed record:** a gate's slow tier of 1,678 s, that the step suite's stand-in leaks polling processes, and the count of the focused review's findings.
- **The simulation's arms were searched, not read**, for the two roads this task says no arm scripts — a call after the breaker tripped, and a second triage pass.
