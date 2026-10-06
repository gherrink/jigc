# Stabilization workflow (open → test → rule → fix → close)

The loop for **taking a build to a version ready to release**. It takes a build and a **closing condition**, finds and fixes until the condition holds, records everything else, and ends with one release ([DECISIONS.md](../DECISIONS.md) → *The stabilization workflow, as ruled*, ruling 1 — the entry every ruling number below refers to). It is the workflow form of a loop that was run by hand on one release candidate — test, fix, re-test — and did not converge: [the record of that pass](../completions/artifacts/M55/fix-pass-rc25/README.md), and the parked description this doc replaces, [ideas/gate-loop-workflow.md](../ideas/gate-loop-workflow.md).

**Where it sits.** The five other loops bracket the `milestone > increment > task` hierarchy: [design](design-workflow.md) governs a *decision*, [dev](dev-workflow.md) a *task*, [milestone planning](milestone-planning-workflow.md) the *opening of a milestone*, [increment](increment-workflow.md) an *increment*, [milestone completion](milestone-completion-workflow.md) a milestone's *close*. This one stands outside that hierarchy. Its unit is a **run** — one build, whatever milestones and fix passes it carries, held against one closing condition — and a run is made of **rounds**. It is the customer of two of the others and restates neither: the completion workflow's audit, verify-real and fix lane, and the dev workflow's loop and gate. It is prose (this doc), a harness script and agent definitions, **not a jigc pack workflow** — that migration is later (ruling 1).

**This doc is the one home of how a run is operated**: what the orchestrator does at each step and on each answer. A rule another doc owns is pointed at, and a mechanism the harness or the record script owns is named with its purpose and left where it is — **Who owns which rule**, below, is the map.

## What is built, and what is not

**Built and committed:** the gate's tiers and its pre-check ([`dev/gate`](../dev/gate)); the record script ([`dev/stabilize-record`](../dev/stabilize-record)) and its suite; the agent definitions a run calls (`.claude/agents/`); the harness ([`.claude/workflows/stabilize.js`](../.claude/workflows/stabilize.js)) and the fence over its source.

**Built is not yet fit for use.** An independent, read-only review of the build came back red: it found mechanisms of the record script and of the harness defective, and they are under repair — the list is under **Open questions and declared bounds** → *Found defective by the review of the build*. **The workflow is not to be used on a real run until the repair and its re-review are recorded** in [DECISIONS.md](../DECISIONS.md). Where this doc says what a step does, it says what the rulings require of that step; the repair's commits correct this doc as they go.

**Not built, or not yet true** — each is taken up where it bites, and listed whole under **Open questions and declared bounds**:

- **Two of the instruments a closing condition can name do not exist: the scripted regression set and the port rehearsal** (ruling 16). A run whose condition has a clause only one of them could judge cannot close until that instrument is built and has its row in the test set (→ Close).
- **The checks on the release PR's head are a procedure the orchestrator runs, not a stage of the harness**, and only one of its three checks is a command today (→ Close).
- **When this doc was written, no stage had been driven by the Workflow runtime.** What stood behind the harness then: the fence over its source, its own self-test — which launches no agent — and one simulation outside the repository in which code played every agent; that simulation is not committed and is not a test ([DECISIONS.md](../DECISIONS.md) → *The stabilization harness, as built* → *Verified*). The self-test under the runtime and a **canary run** — one planted finding through `test`, a ruling, `fix` and a landing, and a second round that is dropped, in a throwaway clone with a bare remote — come after this doc. They establish what nothing had shown: that the runtime accepts the script, that each git step's commands do what its prompt says, that an agent returns what a schema asks, whether an agent can hold a trial arm past the ten-minute tool ceiling, and how the runtime's cap on concurrent agents is enforced. **What they showed is recorded in [DECISIONS.md](../DECISIONS.md) by the build's closing record, never here**; until that record exists, no sentence below about what a stage does is *observed in a run*: each is what the rulings require and what the source was read to do.
- **The `stabilize/` branch type is not applied.** Runs use names the branch rule accepts today (→ Branches).
- **The step in which the human rules on findings found outside the test set is written to be replaceable by an agent** — and is the human's until the human says otherwise (→ Triage, the human's list and the forks).

## Who owns which rule

A reader who wants to change the workflow changes the owner, and this doc only where *what the orchestrator does* changes with it.

| What | Its home |
|---|---|
| The rulings, and every choice a builder made inside them — each marked as open to being overturned | [DECISIONS.md](../DECISIONS.md): *The stabilization workflow, as ruled*, and the build entries above it |
| The stages and the order of their steps; the arguments of an invocation; every branch name (`branchName`); which paths are product paths (`PRODUCT_PATHS`); which agents run an instrument item of a given kind (`CHAINS`); the bound on fix cycles (`DEFAULT_CYCLES`); every git step's command list; the sentence for each word a stage is refused with (`REFUSALS`) | [`.claude/workflows/stabilize.js`](../.claude/workflows/stabilize.js) — its header is the summary |
| A run's files; the vocabulary of grades, dispositions and clause statuses; where a finding is routed; what the next step is, and the order in which that is decided; which round, cycle and attempt a stage works on; which items a round runs; what a writer does to a text before it is written | [`dev/stabilize-record`](../dev/stabilize-record) — its header, which `--help` prints |
| What each role may touch, must return and halts on | the role's definition under `.claude/agents/`; which definition serves which role is the harness's `ROLES` table |
| That the harness and the definitions agree, and that the script's decisions hold | `crates/cli/tests/stabilize_harness_fence.rs` · `crates/cli/tests/dev_stabilize_record.rs` · `crates/cli/tests/merge_logs_fence.rs` (the sync step's log list) |
| The gate | [dev-workflow.md](dev-workflow.md) → Gate |
| Verify-real, and a repro block for every verdict | [milestone-completion-workflow.md](milestone-completion-workflow.md) → The loop, step 2 · [pinning.md](pinning.md) → §3 |
| The fix lane — the finding and never its boundary, the class with how its count was derived, one finding per commit | [milestone-completion-workflow.md](milestone-completion-workflow.md) → The loop, step 3; the round's brief a fixer carries is named by `.claude/agents/build-fixer.md` |
| The branch model | [CLAUDE.md](../CLAUDE.md) → Branches |
| What agents may not do, and the one merge this workflow adds to what they may | [release.md](release.md) → What agents may not do |
| How a pull request reaches `main` | [dev-workflow.md](dev-workflow.md) → Before a pull request to `main` |
| How a version is proposed, published and proved | [release.md](release.md) → Versioning · The release PR · Publishing · Verifying a publish |
| What a report or a record may say in a public repository | [public-hygiene.md](public-hygiene.md) |
| What was deferred, each with its trigger | [decisions-pending.md](decisions-pending.md) → *The stabilization workflow — what its settling deferred* · *The exit rule* |

## The loop (one run)

1. **Open** — a human-led step in the main session fixes what the run is: the closing condition, the candidate and the previous release, the test set, the declared bounds, the stop mode and the bounds. One commit (→ Opening a run).
2. **Test** — one invocation of the harness's `test` stage on the current candidate: the deterministic checks in full, the model-driven instruments over what changed, triage of every finding, verify-real, one record commit (→ A round's `test`).
3. **Rule** — where the state asks for the human: the list of verified findings found outside the test set, a fork, a clause that is still not green after its re-run, a stop. Always **between** two invocations, never inside one (→ Triage, the human's list and the forks).
4. **Fix** — one invocation of the `fix` stage: the human's rulings recorded, fixers, the audit of the fix diff, at most three such cycles, then the round **lands** — or the stage returns at a bound with the round unlanded (→ A round's `fix` · Bounds and exits).
5. **Again** — a landed round leaves a new candidate that nobody has tested, so the next round's `test` comes first, over what the fixes reach. The loop ends when the state answers `close`.
6. **Close** — the closing acts, one pull request, the human's merge, the checks on the release PR's head, the human's merge of the release PR, the publish, the registry proof (→ Close).

**Between any two steps the orchestrator does one thing: it reads `next`** — which a stage that finished returns, and which `dev/stabilize-record state --run <run>` prints at any time, for the branch that is checked out — and does what **Reading the next step** says for that value. It never decides the step itself, and never carries a round, a cycle or a verdict from one invocation to the next: each invocation reads the committed state (ruling 9).

## Opening a run

A run opens with a **human-led step in the main session**, as milestone planning does ([milestone-planning-workflow.md](milestone-planning-workflow.md) → Orchestration; ruling 14). Forks go to the human one at a time; reading, checking and writing go to subagents (ruling 17).

**What it decides, and writes as the opening record** — `opening.md` in the run's directory, prose that the record script neither writes nor reads:

- the **closing condition**, by reference to the decision that states it, clause by clause, each with its scope and its instrument;
- the **candidate** and the **previous release** the run measures against, and the **release being prepared**;
- the **test set** per instrument — fixed before round 1 (ruling 4);
- the **declared bounds** — each the human's ruling, with its reach, and pinned by a test or said to be unpinned (ruling 5);
- the **stop mode** and the **bound across rounds** (ruling 6: the workflow's first run stops after every round, so that it can be tuned; a later run stops at the bound, three rounds unless its opening writes another number), and the **default scope** of a round;
- the **ledger's opening rows**: what is already known against the candidate.

**The run's slug** names its directory under `completions/artifacts/` and, through the harness's one name function, its branches — so it is chosen together with the loop branch (→ Branches).

**What goes in as data, and by which command.** Everything a later step computes from is a row the record script wrote; nothing is read out of the opening record's prose. The script's `--help` owns each call's fields.

| Fact | Call | What computes from it |
|---|---|---|
| the stop mode, the bound across rounds, the previous release's version and the commit it was released from, the default scope | `dev/stabilize-record run-set` | where the run stops for the human; the second binary of every round and what *a regression of the run* is measured against; the scope of a round whose invocation names none |
| one row per clause of the closing condition, each `void` and without a commit: *not yet run* | `dev/stabilize-record clause-set` | closing — a table with no row is a run that is not ready, and a clause without a row is a clause nothing would ever ask about |
| the test set, one row per instrument item: its kind, the clause it judges, whether it runs on every candidate or only in scope, the doors and registries it covers, its brief | `dev/stabilize-record item-set` | which items a round runs; which agents run each |
| the declared bounds | `dev/stabilize-record bound-set` | whether a grade of *out of scope* may stand |
| the opening rows of the ledger, as round 0 | `dev/stabilize-record ledger-add` | the first triage that runs is handed them |

Three things about those calls that the opening has to get right:

- **The script creates neither the run's directory nor the opening record.** Both exist before the first call; a mistyped run name is refused and mints nothing.
- **Every write is scanned**, so the machine needs gitleaks and the private denylist ([public-hygiene.md](public-hygiene.md) → The guard), beside python3, git and bash. Without them nothing is written, and that is reported as *did not run*, never as a pass. [machine-setup.md](machine-setup.md) does not list them yet.
- **An item's kind decides who runs it.** The harness has a chain of agents for a review row, a trial arm and the three kinds of the audit (an area, the cross-cutting pass, the drive); an item of kind `check` is a deterministic check, run by the preflight from the item's brief; any other kind halts the stage that would run it. The names are the keys of the harness's `CHAINS` table, which is the list — this sentence is not.

**Then one commit, behind the full gate, pushed by name.** The opening is ready when `dev/stabilize-record state --run <run>` prints `"not_ready": []` and `"next": "test"`; until then `next` is `not-ready`, `not_ready` names what is owed, and both stages are refused. From this commit on, the harness holds the loop branch to its path rule (→ Branches).

**Each run's opening is its own decision.** This doc opens none, and the first run's is recorded where it is taken.

## A round's `test`

```text
Workflow({ name: 'stabilize', args: { stage: 'test', run: '<run>', scratch: '<absolute dir outside the repository>' } })
```

The loop branch is checked out, clean, and **pushed** — the stage does not push before its checks, and the check that reads CI needs a run for exactly the candidate's commit. The candidate is the loop branch's tip at the invocation; its label and the round's number are derived, never passed. What the machine needs beyond the gate's tools the preflight asserts, and halts naming what is missing: the record script's scanners, a container runtime where a trial arm runs, `gh` where a check reads CI.

**What the stage does, in order** (the harness's header is the home of the list; each role's contract is its definition):

1. **The git state** — and the path rule of → Branches, asserted.
2. **Preflight, beside the scope step.** *Preflight* asserts the environment, builds **one** release binary from the candidate's commit and the previous release's binary from its recorded commit — and one trial image, in a round that runs a trial arm — and runs the deterministic checks: the test set's items of kind `check`. *Scope* resolves what this round tests into **doors**, from the code and the docs as they stand: the doors the change reaches and the doors it does not, each with its derivation, written once. Where the consumers of a change cannot be enumerated to an end, the whole registry is inside (ruling 10).
3. **The instruments** the round's doors select, in parallel: the record script computes which items run — an every-candidate item always, an in-scope item when the round's doors reach it. Every driving agent is handed the one binary and returns the hash it asserted; the harness compares the two (ruling 11).
4. **The reports, checked**: exactly one per agent launched. Every report reaches the repository through the record script and no other way (ruling 12).
5. **Triage and verify-real**, alternating (→ Triage, the human's list and the forks).
6. **The record step**: the ledger rows, the round's triage record, a clause row per clause an item of this round judged, the round's facts — then the full gate and one commit on the loop branch, pushed.
7. **The state, read back**, and `next` returned with it.

**What the orchestrator may pass**, beside the three required arguments — the harness's header, *Usage*, is the home of each:

- **`scope`** — the human widens or narrows a round: everything, a commit range, or named doors. Absent, the run's default scope applies. A round's scope is written once, so this is decided before the invocation.
- **`clause`** — one clause's instrument alone, over the doors of the latest round, named again. It is what `next: retest` asks for.
- **`crossModel`** — the items whose source pass is *also* read by a model of another family, **named one by one**. No value means every item; with none named, no stage calls, names or needs that tool, and a machine without it runs the whole workflow. By the human's ruling a named item's pass that could not run is void for that pass only and fails nothing else, and the round's record says which items got one. It is a human-approved suggestion and never auto-run ([milestone-planning-workflow.md](milestone-planning-workflow.md) → the loop's Review step); naming the item is the approval. *Limited and rare, per item, never required* — the human's ruling ([DECISIONS.md](../DECISIONS.md) → *The stabilization harness, as built*).
- **`stopAfter`** — return after a named step, for tuning (mind the bound on it under **Halt and resume**).

**What comes back.** `status: 'triaged'` with the round, the candidate and its binary's hash, the counts, `human_list`, `forks`, what forbids closing, and `next`. Two things the orchestrator reads beside `next`: `counts.voided` — the items that did not run to their end — and `scope.uncovered` and `scope.reached_but_excluded`: the doors of the round's test set that no item reaches, and the doors the scope step's derivation reaches that a scope set by the human leaves out. Neither is a finding, and neither is in a table of the run: they are in this return and in the scope step's report. The second is what the human needs at the next stop; the first is a door of the round that nobody tested.

**A clause row says how its instrument *ran*, never what it found**: void when an item of the clause did not run to its end, red when a deterministic check of it is red, green otherwise. What a hunting instrument found is the ledger's, and forbids closing there. Ruling 16 has a void or missing row forbid closing; how a clause's evidence is kept when several instruments judge it, over several rounds, is under repair (→ Open questions).

## Reading the next step

`next` is computed by the record script from the ledger, the clause table, the rounds' records and the run's facts. **When each value holds, and the order in which they are tried, is the script's** — its header, *THE NEXT STEP* and *THE STOP* — and is held to a truth table by its suite. What follows is only what the orchestrator does.

| `next` | The orchestrator |
|---|---|
| `not-ready` | finishes the opening: the state's `not_ready` names what is owed (→ Opening a run). No stage starts. |
| `test` | starts `test`. There is no round yet, the latest round's `test` never reached its record, or a round's fix stage is over — landed, a part landed, or dropped — and what it left is tested by nobody. |
| `triage` | invokes the stage whose `position` carries `triage: true` — with no `scope`, `clause` or `crossModel`. It grades, verifies and records the findings the state lists as `untriaged`, and runs no instrument. A finding that leaves that invocation still without a verdict has no further step in the state and no way to the human: under repair (→ Open questions). |
| `rule` | takes it to the human: the findings of `human_list`, each with why it is the human's, and the clauses of `human_clauses` (→ Triage, the human's list and the forks). |
| `fix` | starts `fix` — at once, when `test` left nothing to rule on (ruling 9). |
| `retest` | starts `test` with `clause` set to the **first** clause the state's `retest` names. Each re-run is a round of its own and runs one clause; the next state names the rest. An invocation without `clause` would start a whole round. |
| `unsettled` | takes it to the human. Nothing is open, and a clause is not green — or green from before a fix round — while **no item of the test set judges it**: there is no instrument to run again, and the rulings name no step. The state's `unsettled` names the clause. Its two ways out are under → Close. |
| `close` | closes the run (→ Close) — an answer that closes nothing until the repair is recorded. The return carries each clause's evidence and the close's first git step. |
| `stop` | returns to the human, with `stop.why` and `stop.then` — the step that follows the go (→ Bounds and exits). |

**A stage can answer with something other than a finished stage**, and each has one meaning:

- `refused` — nothing ran beyond, at most, two reads. Either the arguments were malformed, or the state's `position` refuses this stage now, in one word with one sentence of what to do instead; that refusal carries `next`.
- `halted` — a step did not do what the stage needs. Committed work stands (→ Halt and resume). Three halts are not faults but the human's turn: `halted.phase` is `bound`, `fork`, or `fix` where no fixer fixed anything (→ Bounds and exits · Triage, the human's list and the forks · A round's `fix`).
- `ruled` — an invocation that carried rulings about the run recorded them and started nothing.
- `landed` · `dropped` — how a `fix` stage ended a round. `cycle` · `nothing-to-fix` — a `fix` stage that had nothing it could do; `next` says what is owed.
- `stopped` — a `stopAfter` return.

## Triage, the human's list and the forks

**Every finding is graded, and nothing leaves the list unseen** (ruling 5). Triage grades every finding a stage produced — a lead, and every entry an agent *left open*, included — and keys each to one ledger row for the life of the run. An independent verifier re-drives everything graded as breaking or unclear, with a brief to refute it, and establishes on the previous release's binary whether it is a regression. The rule is the completion workflow's verify-real ([milestone-completion-workflow.md](milestone-completion-workflow.md) → The loop, step 2); what this workflow adds is who may decide what:

- **Whether a finding is inside the round's test set is computed** — its door, by exact text, against the round's committed door list. No agent supplies it.
- **Whether it is a regression is a fact** — green on the previous release's binary, red on the candidate.
- **A grade of *out of scope* must cite a declared bound** on the run's list; one that cannot goes to the human.
- **What an agent leaves open is triaged like any finding**, never queued for a fixer on its own. So triage and verification alternate — as many times as the harness's `VERIFY_PASSES` says — and what is still unverified after that leaves the round's triage *unfinished*: `next` is `triage`. Nothing bounds how often — the fourth item of the review's list.

The vocabulary of grades, verdicts and dispositions, and the route each combination takes, are the record script's (its header: `ledger-set`, *THE ROUTE*).

**What is fixed without asking, and what is the human's** (ruling 4). A verified finding **inside** the test set, and a **regression** wherever it was found, is routed to a fixer. A verified finding that breaks the condition but was found **outside** the test set is the human's, as a list, at the end of the round. Per item the human gives one of three rulings:

- **admit** it to this run — it is then fixed like any blocker;
- **declare a bound**, with its reach, where the human ruled it, and the test that pins it or the word *unpinned*;
- **record it for a later release**.

**How a ruling is recorded.** The orchestrator passes the human's rulings to the next `fix` invocation as `rulings`; one step records them before anything is fixed, and the same invocation goes on to fix what is then routed to a fixer. An item left unruled keeps `next` at `rule`, and nothing is fixed beside it. The shapes are the harness header's (*Usage* → `rulings`).

**This step is written to be replaceable.** Its input is the state's `human_list`; its output is the `rulings` argument. An agent that took the list over would change neither — and would still not declare a bound, which stays the human's ruling. Whether and under what rule an agent takes it is deferred, with no trigger but the human's word ([decisions-pending.md](decisions-pending.md) → *Handing the ruling of the outside-the-test-set list to an agent*).

**A clause that is the human's.** A clause that is not green gets its instrument run again once, automatically (`retest`). Still not green after that, it is in `human_clauses`, and the **one** ruling with a recorded form is one more re-run: `rulings: [{ rerun: '<clause>' }]`, on either stage, with the loop branch checked out. Dropping or replacing a clause or an item is no ruling inside a round — it changes the closing condition or the test set, which the opening fixed, and so is a new opening.

**A fork.** A fix that is contested — the verifier found the finding to argue against a settled decision — or that needs a new mechanism — a fixer halted on it — goes to the human with a `robust-advocate`'s case, **and the proposal is driven before the human sees it**: by the advocate, and then by an agent that is not its author ([milestone-planning-workflow.md](milestone-planning-workflow.md) → the loop's Settle step, the claim-driven rule). Forks come back in a stage's `forks`, each with the case and the independent drive; a `fix` stage that met one returns `halted` with `phase: 'fork'`, the round unlanded. The orchestrator brings each to the human (→ Orchestration). The human's answer is recorded as a ruling on that finding — admitted, with a note that reaches the fixer; a bound; or later.

**A red deterministic check.** The rulings give a check three answers — green, red, void — and say what a row that is not green does to closing. They do not say how a red check reaches a fixer, and nothing in the harness hands one to triage. A candidate whose *gate* is red meets a second gap first: the stage's own record step runs that gate before its commit. Both are under **Open questions**.

## A round's `fix`

```text
Workflow({ name: 'stabilize', args: { stage: 'fix', run: '<run>', scratch: '<dir>', rulings: [ … ] } })
```

`rulings` only when the human ruled. The stage starts from the loop branch or from the round's own branch, whichever is checked out.

**What the stage does**:

1. **The git state and the run's state**; the round's branch found, if it exists.
2. **The human's rulings recorded**, by one step.
3. **A cycle**, while a finding is routed to a fixer and the bound allows:
   - the **round branch** is opened from the loop branch, or switched to;
   - **fixers**, one per area — the blockers partitioned by the registry their door comes from — **serial, in the main tree, one finding per commit**, each commit behind the full gate. The lane is the completion workflow's; a fixer's return also carries the doors its commits can change, which the next round's scope step starts from. A fix that needs a new stored file, registry, flag or finding code, or that would revise a settled decision, is that fixer's halt for that finding and never its build decision;
   - the round branch is pushed and **the round tip's binary** built;
   - **the audit of the fix diff**, as exactly that diff over the product paths: a review and a drive, in parallel;
   - triage and verify-real of what the audit found and of what the fixers left open; the record step on the **round branch**; the push.
4. **Land** — only when the audit of the round's fix diff is green and nothing of the round is left open (ruling 7): the round branch is merged into the loop branch with a merge commit, and the loop branch is pushed. **What lands must be what was audited.** The step checks that the merged tree's product paths are exactly the round's; that the last audit read the round's tip as it then stands is the half of the requirement the review found unheld (→ Open questions).

**A cycle in which a fixer halted on a fork, or in which nothing was fixed, is not counted against the bound**, so that the same fixer is not sent at the same finding until a bound is reached. A fork stops the stage for the human; a cycle that fixed nothing returns `halted` with each finding the fixer could not fix and its notes. What ruling 7 requires beside that: a fix another fixer committed in such a cycle is part of the round's fix diff all the same, and is audited before the round lands — the first item of the review's list.

**What comes back from a landing**: `status: 'landed'`, the new candidate — the merge commit — the cycles it took, and `doors_for_retest`. `next` is then `test`, or `stop` where the run stops after every round.

**The `fix` stage's records ride the round branch** and reach the loop branch when the round lands (adjustment iv). So the state is the checked-out branch's: on the loop branch, a round that is out still shows its blockers open.

## Bounds and exits

**Inside a round: at most three fix → audit cycles** (ruling 6). The next one is not started; the stage returns `halted` with `phase: 'bound'`, **the round unlanded**, the ledger as it stands, and the three exits spelled as invocations. Only the human takes one:

| Exit | The next `fix` invocation carries | What happens |
|---|---|---|
| **continue** | `raise: { cycles: N }` | the bound for this round is N. Nothing remembers it: it is passed again on every later `fix` of that round. |
| **drop the round** | `exit: 'drop'` | the round's record says so; every row its fixes had closed is open again, because those commits never land; and exactly the round's record commits are carried over to the loop branch, so the round keeps its record. |
| **land a part** | `exit: { part: [ <the fix commits to keep> ] }` | the fix commits the human keeps, and the round's record commits, are re-cut as a branch of their own from the loop branch's tip, then gated and **audited once more as exactly that diff before it lands**. The fixes left out never land, so their findings are open again, and a part that lands ends the round. This exit is on the review's list (→ Open questions). |

**Across rounds: the stop mode, written at the opening.**

- **A run that stops after every round** returns `next: 'stop'` with `stop.why: 'every-round'` once a round is over — whatever follows, `close` included. The human's go is a recorded fact: `rulings: [{ go: true }]` on either stage, loop branch checked out. That invocation records the go, starts nothing, and returns the `next` the stop was holding. A go costs a record step, so a full gate.
- **A run that stops at the bound** returns `stop.why: 'round-bound'` when one more fix round would start and as many rounds have a fix stage on record as the bound allows. **The bound counts fix rounds, never test stages**: the `test` after the last allowed fix round always runs, and so do a finished triage, the human's ruling on a list, and a clause's re-run. Only the human raises it: `rulings: [{ rounds: N }]`, which is the go.

A ruling about the run — a go, a re-run, a raised bound — is its own invocation and is never mixed with a ruling on a finding.

**At the first stop of the workflow's first run** the human also looks again at the performance levers that were left open, on that run's own numbers ([decisions-pending.md](decisions-pending.md) → *The performance levers left for the first stop*). Until the human rules otherwise, the full gate precedes every commit.

## Branches

The model, its names and its rules are [CLAUDE.md](../CLAUDE.md) → Branches; this section says only what a run adds, and spells no name. **The harness mints every branch name of a run in one function, `branchName`, from the run's slug.**

- **The loop branch** carries the run. **A round is its own branch**, opened from the loop branch, pushed by name, and landed with a merge commit only when the audit of its fix diff left nothing open (ruling 7). **A part** re-cut at a bound is a third branch. No step deletes one.
- **The invariant is by path** (adjustment ii). **Product paths — the paths the published crates carry, and their tests — move only through an audited round's merge. Every other path** — the run's records, `.claude/`, `dev/`, `implementation/`, the logs — **commits directly on the loop branch behind the gate.** Which paths are product paths is one list in the harness. Every stage opens with a check of the loop branch's history since the opening record's commit: a commit that changed a listed path directly, or a merge that is not a round's by its subject, halts the stage. That check is a tripwire for direct commits; it shows nothing about whether a merged round was audited.
- **An agent may land a round branch** — the one merge this workflow adds to what agents may do ([release.md](release.md) → What agents may not do; ruling 8). Everything else there stands: no agent pushes to or merges into `main`, force-pushes, merges a pull request or pushes a tag.
- **The `stabilize/` branch type is not applied.** The branch rule refuses that prefix today, and the names the harness mints are of a type the rule accepts — checked with [`dev/branch-name`](../dev/branch-name) for a loop, a round and a part. The type of its own is owed as **one commit — the last on the first run's loop branch before its pull request**: the branch rule and its fence, the model's table, the release doc, the fenced *Never* paragraph in every agent definition, and `branchName` ([decisions-pending.md](decisions-pending.md) → *The `stabilize/` branch type*). When it lands, the two sentences of this doc that say *not applied* are what changes: nothing here spells a name.

## The run's record

One directory per run under `completions/artifacts/`, named for the run. The list of its files and who writes each is the record script's header; what the orchestrator needs to know of them:

- **The ledger — one row per finding**, for the life of the run (ruling 13). Every finding of every stage gets a row, whatever its grade; a finding found again keeps its key and inherits the human's ruling on it, but not a fix that did not hold — that finding is open again. Keys follow the grammar of the [seed ledger](../completions/artifacts/M55/seed-ledger.md), so that an open row can later be filed as one jigc doc ([findings-channel.md](../design/findings-channel.md)).
- **The clause table — one row per clause** of the closing condition: the instrument, the commit it last ran on, its scope, and green, red or void (ruling 16). **Closing is computed from this table and the ledger, and remembered by nobody.** That one row has to hold the evidence of every instrument that judges the clause is the third item of the review's list.
- **The declared-bounds list and the test set** are tables of their own, which the opening record names.
- **Per round**: its scope — the doors on both sides — its triage record, its facts, and every report of every agent, by stage, cycle and attempt.

**Nobody writes any of it by hand** — neither an agent nor the harness nor the orchestrator. The script decides the path, replaces host paths with the public placeholders, stops hard on the public-hygiene scan — a report is public on the next push — and refuses a line that would redden the one fence that reads every markdown file. A table that is not the script's own, or a cell outside its vocabulary, is refused as corrupt the next time the state is read.

## Close

**`close` is computed, never judged** (ruling 16). What the rulings require of it: a tested candidate — the latest round tested it, and nothing was fixed since; nothing open in the ledger; and every clause's instrument green on record, *with that evidence still good for this candidate* — a deterministic check's green counts only for the candidate it ran on, a hunt's green from before a landing only while every fix round since was followed by a round that resolved a scope of its own and none of those scopes reaches an item of the clause. `forbids_close` lists what the script sees in the way. **The review of the build constructed states in which the script answers `close` against those requirements** — over an instrument that never ran to its end, and over a fix no audit read. Until the repair is recorded, an answer of `close` closes nothing.

**What cannot close.** A clause that no item of the test set judges stays void and answers `unsettled` once nothing else is open. That is where a run stands whose condition names the regression set or the port rehearsal while they do not exist. The two ways out: the instrument is designed with the human, built, and takes its row in the test set — `item-set`, after which the state asks for the clause's run; or the closing condition changes, which is a new opening.

**The order**, once `next` is `close`:

1. **The human closes, with the evidence in front of them.** The return's `evidence` gives, per clause, the round and the commit its instrument last ran on and how many **fix rounds** behind the candidate that is. The human may order one more `test` with `scope: 'everything'` instead of closing — the answer to the one dissent the rulings record (→ Why this shape).
2. **The closing acts.** On the loop branch, each commit behind the gate: the run's closing record and its fold-back, and — on the first run — the `stabilize/` branch type. And [`dev/clean-litter`](../dev/clean-litter), which commits nothing. What a run's fold-back holds is not ruled in general; for the first run it is listed by [the pass's record](../completions/artifacts/M55/fix-pass-rc25/README.md) → *What is owed next, in order*, step 3.
3. **The sync, the gate, the push, the pull request** — [dev-workflow.md](dev-workflow.md) → Before a pull request to `main`. The harness returns the first of these as `close.sync`: the orchestrator spawns it verbatim, on the model it names; then the gate on the merged tree, the push by name, and the pull request. **The sync is the last thing the harness has a hand in**: its merge is not a round's, so — as the path rule's step is worded — a stage invoked after it halts there.
4. **The human merges the pull request.** release-plz puts the version on the release PR ([release.md](release.md) → The release PR).
5. **The checks on the release PR's head — before the human merges it** (adjustment iii). This is a procedure the orchestrator runs with subagents, and no stage of the harness:
   - **the packaged-tarball install** on that head — `dev/runner-faithful --commit <sha> tarball` ([release.md](release.md) → Verifying a publish);
   - **the packaged file set compared with the tested commit**, the expected differences being both crates' versions, the engine pin, the lock file and the changelogs — **no script does it today**;
   - **one upgrade arm from the previous release to the new version**, which no local candidate can exercise: what an upgrade triggers fires only when the version differs, and every local candidate carries the previous release's. The trial harness holds what such an arm is built from — two pinned images, and the check that they are two different trees ([completions/trial-harness/](../completions/trial-harness/README.md)) — and **nothing of this workflow composes them**.

   The release is of two crates, each versioned on its own ([release.md](release.md) → Versioning).
6. **The human sets the version where the derived one differs from the planned one, and merges the release PR.** Both are the human's alone; merging it is the go, and the publish follows ([release.md](release.md) → The release PR · Publishing).
7. **The registry proof** — `dev/runner-faithful registry '<requirement>'` with the install line's requirement ([release.md](release.md) → Verifying a publish · Installing). After the publish it is the only proof left.

## Halt and resume

**A stage that returned is never resumed.** Halted, stopped at a bound or finished, the next invocation is a fresh one and reads the state as it stands; it works the next attempt, whose number the state gives, so a re-run's reports never collide with a stale one. `resumeFromRunId` is for a **killed** run only, and then with identical arguments.

**A halt names its phase and its reason**, and says whether it was transient — an agent that returned nothing after retries, which is infrastructure and no verdict about the run. The harness's message says to send the same invocation again once the cause is dealt with. **That holds for a halt before the stage changed anything.** What the record and the branches look like after a halt in the middle of a git act or of a record step is on the review's list; until its repair is recorded, such a halt goes to the human with the tree, the branches and the state as they stand. **Clearing a halt: the orchestrator decides, a subagent acts** (ruling 17). Where a dead agent left the tree dirty, the harness says what it may have left — uncommitted edits, commits on the round branch — and decides nothing about them; the build loop's classification of such a tree is the precedent ([increment-workflow.md](increment-workflow.md) → Halt and resume, the three tree states), and nothing rules that it binds here.

What a halt leaves, as the source has it — none of it observed in a run yet:

- **A reporter that left no report halts the stage before triage.** The reports already written stay untracked under the run's directory; the next record step commits them as an earlier attempt's, and nothing removes them.
- **A `test` stage that stops after its scope step leaves the round's scope file untracked** — after a halt, or after a `stopAfter` later than `state`. The git step that opens every stage admits untracked files only where they are reports. **So, as that step's rule is worded, the next invocation halts on the scope file** — which contradicts the harness's own *invoke the stage again*. Read off the source, not driven; the canary is where it shows, and what is done about the file until then is the human's.
- **Two different agents exhausting their retries** trips a breaker and the stage spawns nothing more: that is the shape of a rate limit, and the answer is to wait.

## Orchestration

**The vehicle matches the work**: the opening and every ruling are human-led, in the main session; a stage is a workflow that runs without the human and returns. The main session only orchestrates (ruling 17): it starts a stage, reads `next`, brings the human what is the human's, and passes the answer on. Long reports are read by subagents that return a summary; a fork is put to the human one at a time, in plain text with lettered options.

**One role, one definition** — the harness's `ROLES` table says which: the preflight (`stabilize-preflight`), the scope step (`stabilize-scope`), triage (`finding-triage`), verify-real (`finding-verifier`), the reviews (`milestone-code-reviewer`), every drive (`milestone-e2e-tester`), the advocate (`robust-advocate`), the fixers (`build-fixer`), the record step (`build-executor`), and every git step and every read of the record (`build-git`, on Sonnet, like every git step of the build harness — [increment-workflow.md](increment-workflow.md) → Orchestration). Two roles have no definition and a prompt that is their whole contract: the independent drive of an advocate's proposal, and the cross-model pass. A definition for the first is the human's to order.

**The contracts are inert outside a run.** Each contract a run adds binds on a labelled line of the prompt that the build harness does not send, so the same definitions serve a milestone build as they did; the fence holds each label to one spelling, in the script and in every definition that binds on it.

**Independence is what agents do not share**: a scratch root of their own under the invocation's, rigs of their own, no sight of another's report unless the prompt hands it over. What they do share is the one binary per candidate — built once, read-only, its hash asserted by every agent that drives it. It is a release binary, so the debug-only route fences are not in it; the gate is what runs them.

**Checking the harness without a run**: `args: { selfTest: true }` launches no agent and checks the script's own logic; the fence runs the same wherever `node` is installed.

## Why this shape

- **Candidates are local** (ruling 2; adjustments i and iii). A candidate is a commit on the loop branch, built from that commit: inside the loop there is no version, no tag and nothing that needs the human's merge. The rulings state that property and its costs, and no sentence of why beyond the entry's premise: a loop that runs without the main session, which comes in only for decisions. The costs, each answered: every local candidate prints the previous release's version, so evidence is keyed by commit and by the binary's hash; a loop that closes on one machine would meet a red required check at the pull request, so the candidate's CI result is read on every candidate; a trial arm runs on an image built from the candidate's commit and no longer on a published version, so the proof a registry install gave moves to the end; and what only a version change exercises moves to the release PR's head.
- **Scope shrinks** (ruling 10, and the human's answer to the dissent). Round 1 tests what the candidate changed against the previous release, and each later round only what the previous round's fixes reach — because **a repeated hunt over unchanged code finds new things by sampling**, and a loop that hunts everything every time does not end. The pass this workflow came from is the evidence: six blocking rows became fifty-seven commits, and each door studied hard yielded another finding ([ideas/gate-loop-workflow.md](../ideas/gate-loop-workflow.md) → *What made the rc.24 loop not converge*). **The dissent stands on record**: a cross-model reviewer held that scoping by door is not safe without one full run of every model-driven instrument before closing. The human ruled that out; what was worked in instead is that a scope defaults to *inside* where it cannot be bounded, and that the close shows how old each clause's evidence is — in fix rounds, where the dissent was promised commits (→ Open questions).
- **A hunt runs once per change, a check always** (ruling 10). A model-driven instrument samples, so its second pass over the same code is a new sample and not a confirmation. A deterministic check is cheap and is evidence about exactly the candidate it ran on, so it runs in full on every one — the gate, into which every confirmed and refuted repro is pinned, among them.
- **Findings outside the test set go to the human** (rulings 4 and 5). The test set is fixed before round 1 so that the loop has an end: in the pass before, each door studied hard yielded another finding, and nothing said which of them the run owed. And the list is the human's, whole, because triage is where a list shrinks unseen — an *out of scope* nobody ruled, a severity used as a filter — which is why *inside* is computed and a bound is always the human's.
- **Close is computed** (ruling 16; [DECISIONS.md](../DECISIONS.md) → *The decision table's holes, closed*, decision 1). *Cannot close before each instrument ran* was stated and enforced by nothing, with no word for *could not run*. So closing is a function of the clause table and the ledger, a void or missing row forbids it — and it is reachable only from a tested candidate, since a landing leaves something no instrument has seen.
- **Two stages, each a fresh invocation** (ruling 9). A workflow takes no input while it runs, so every point where the human may rule lies between two invocations — a stop is a return, never an agent waiting. And each invocation reads committed state, because the loop this replaces lived in one orchestrator's head.
- **Bounded, with the exits named** (ruling 6). The ruling sets the bounds and gives no reason. The increment loop's bound of the same size does: thrash is a signal for a human, not for a fourth round ([increment-workflow.md](increment-workflow.md) → The loop, step 4).
- **It is the customer of the workflows beside it.** A fix is a dev-workflow task, a verdict is the completion workflow's verify-real, a pull request is the dev workflow's step. Cross-reference, never restate.

## Open questions and declared bounds

The union of what the rulings deferred, what each builder left open, what an independent review of the build found, and what writing this doc found. A builder's choice the human may still overturn is listed in its own entry and not again here — and so are the readings the step table rests on: the decisions that settled what `next` answers are, all but one, the orchestrator's readings of the rulings, reported to the human and standing unless overturned ([DECISIONS.md](../DECISIONS.md) → *The run's decision table, settled* · *The decision table's holes, closed*).

**Found defective by the review of the build, and under repair.** Each is a mechanism that does not do what the rulings require of it. **Until the repair and its re-review are recorded in [DECISIONS.md](../DECISIONS.md), the workflow is not to be used on a real run.**

- **Landing, and what it requires of the audit.** A round may land only when the audit of its fix diff is green. A fix commit can reach the loop branch that no audit read: one made in a cycle that was not audited — beside another fixer's fork, or before a halt — followed by a ruling that leaves nothing open. And the state can answer `close` over such a fix before it has landed at all.
- **The *land a part* exit.** A part can land while a finding of its own audit is still unverified. And a part exit that halts after its branch is cut forgets which fixes it left out, so a finding stays recorded as fixed by a commit that never lands.
- **A clause's evidence when several instruments judge it.** A clause has one row, rewritten by whichever of its items a round ran. So an instrument that never ran to its end is turned green by a later round that ran the others, and the run closes; and a re-run of one clause can stand in for the scope a landing owes.
- **The unverifiable finding.** A finding no verifier can drive keeps the step at `triage`, invocation after invocation, with no bound and no way to the human. A triage finished inside a running `fix` invocation has its findings verified on a commit from before the fixes, and the preflight's contract refuses the commit a finishing `test` invocation asks it to build.
- **Records around a halted git step.** A landing that halted after its merge is reported as landed by the next invocation, unchecked and unpushed. A drop and a part write the round's outcome before the git act it describes, so a halt between the two leaves both stages refused. And the advice the harness gives after every halt — the same arguments again — is wrong once rulings are recorded, a part is cut or a drop is recorded.
- **A record step under a red gate.** The step writes its tables, then gates, then commits. A red gate leaves the tables modified in the tree, and the step that opens every later invocation halts on a tree that is not clean — so a candidate whose gate is red cannot have its `test` stage recorded at all.

The review's further findings are the closing records' to list, with its reports. The repair's commits correct this doc as they go.

**To be shown by the self-test under the runtime and by the canary** — unverified until the build's closing record says otherwise:

- that the runtime accepts the script, that a git step's commands do what its prompt says, that an agent returns what a schema asks;
- whether a harness-launched agent can hold a blind trial session past the ten-minute tool ceiling; how the runtime's cap on concurrent agents is enforced; the replay of a resumed, killed run;
- the untracked scope file after a stopped `test` stage (→ Halt and resume), and whether a preflight call of the same stage reads that file as a tree that is not clean;
- a granted re-run and a raised bound driven through the harness — the simulation drove only the go.

**Not built:**

- **The scripted regression set and the port rehearsal** — designed with the human, one at a time, then built by subagents ([decisions-pending.md](decisions-pending.md) → *The exit rule*). Open with them: which kind each takes — a `check` runs with no change to the harness, any other kind needs its chain — and how *over the whole delta since the previous release*, the one stated exception to scoping, is given to an item, which today is handed the round's doors.
- **A script for the release PR head's file-set comparison, and the composition of its upgrade arm** (→ Close).
- **The `stabilize/` branch type** (→ Branches).
- **A clause's evidence age in commits.** The dissent was promised that number at closing. The state gives fix rounds, and the two commits a git step would count between; no step of the harness counts.
- **A definition for the independent drive of a proposal**; and a per-finding check of fix against finding in the audit of a fix diff, if the first run shows that it wants one.

**Open, the human's to rule:**

- **What a run's close writes, in general**: its closing record and its fold-back have a list for the first run only (→ Close).
- **Which paths the path rule should spare.** A test file is a product path, so a tuning commit to the harness's own fence — or to any fence a doc owes a row in — made directly on the loop branch halts the next stage. The list is one constant.
- **How a red deterministic check reaches a fixer.** Nothing hands one to triage. The pieces that exist: a ledger row added through the script is graded by the next triage that runs, as an opening row is. Whether that is the route is not ruled.
- **A fork has no row.** It is in the stage's return and in the advocate's and the independent drive's reports, and in no table the state is read from — so nothing the state computes waits for the human's answer to it, and a session that loses the return reads the fork back from those reports.
- **A red check on the pull request after the sync.** As the path rule's step is worded, no stage starts once the sync's merge is on the loop branch, so nothing says how such a fix is audited.
- **Whether a dropped round should count toward the bound and be followed by a `test`** — as built it does both, though it landed nothing.
- **The two states with no step inside a round**: a clause that is the human's when the human wants anything but one more re-run, and a clause no item judges. Each is a new opening today.
- **The performance levers left for the first stop**, and **handing the human's list to an agent** — each with its row in [decisions-pending.md](decisions-pending.md).

**Declared bounds — true as built, and to be known by whoever runs one:**

- **A round's record does not say that its branch was merged.** On the round's own branch, once a cycle is recorded and nothing is open, the state reads as after the landing; only `position.fix.land` says that the landing is owed.
- **A re-run of one clause runs over the latest round's doors, on whatever the loop branch's tip is** — a record commit moves that tip without changing a product path.
- **A row the ledger hands a fix cycle's triage is verified on a binary of the round branch**, not on the candidate the round tested.
- **A dropped round's record commits reach the loop branch by cherry-pick**, behind no gate of their own. **A part's kept fixes get new commits.**
- **A disposition is the human's by vocabulary only**: the script cannot know that a human ruled. Of a declared bound it refuses a row that does not say where the human ruled it, and no more.
- **What a round found again is as its triage record stands**: an entry recorded again in the same round replaces its row.
- **The cycle bound is the harness's constant and an argument**; the bound across rounds and the stop mode are the record's. A candidate's label is in no record: it is derived from the round.
- **The record script refuses the one line a standing fence reads, by a list of one.** A second fence over every markdown file owes it a row; until then the stage's own gate is what catches it.
- **The gate's fast tier is cut by this machine's last measurement** ([dev-workflow.md](dev-workflow.md) → Gate). [`dev/gate --fast`](../dev/gate) is a pre-check and never a gate.
- **Three definitions a run launches still describe the gate as running a step that retired** — a stale sentence in the definitions, with no effect on what the gate runs.
