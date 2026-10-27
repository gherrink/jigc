// THE SOURCE OF THE HARNESS. Builders edit this file; the one a session invokes,
// .claude/workflows/stabilize.js, is generated from it without its comments by
// `dev/stabilize-harness` — run it after every edit, and commit both files
// (implementation/stabilization-workflow.md -> The harness's source).
//
// stabilize — the harness of the stabilization workflow: one script, two stages, each a
// fresh invocation (DECISIONS.md -> 2026-10-05, "The stabilization workflow, as ruled";
// what this script settles inside those rulings is the entry of 2026-10-06, "The
// stabilization harness, as built", and what the entry "The decision table's holes,
// closed" changed in it). It takes a run that the human opened — a build and a
// closing condition — and works one round of it: `test` finds and grades, `fix` fixes,
// audits the fix diff and lands. Every point where the human may rule lies BETWEEN two
// invocations, so a stop is a return, never an agent waiting.
//
// WHAT THIS SCRIPT DECIDES, AND WHAT IT DOES NOT. It has no shell, no file system and no
// clock: it sees what an agent returns to it. The run's committed state is read by
// `dev/stabilize-record state`, and what this script ACTS ON of it is relayed by a git step
// as the DIGEST its command prints (below, WHAT THIS SCRIPT READS) and checked here against
// the hash that line ends with — and whether a finding is inside
// the round's test set, where it is routed, which items a round runs, which round, cycle and
// attempt an invocation works on, and what happens next are computed THERE, where a suite
// holds them to truth tables on every gate. This script computes none of them again. `next`
// is returned to the orchestrator verbatim; `unsettled`, and any value this script does not
// know, is a return that names the file the state document lies in — never a guess and never
// a loop. Nothing a previous invocation knew is used: an invocation starts from the state
// and from git.
//
// WHAT THIS SCRIPT READS (the second repair plan's task K3; DECISIONS.md -> 2026-10-07, "What
// the harness reads, changed once"). Observed on 2026-10-07: a line relayed through an agent
// never arrives byte for byte once it holds a character outside ASCII, and from about 80 KB
// it does not arrive at all. So EVERY LINE OF A STEP THIS SCRIPT HOLDS TO A HASH IS A DIGEST
// (`dev/stabilize-step <act> --digest <scratch>`; that tool's header: THE DIGEST): ids,
// words of a fixed list, counts, hashes and paths of plain segments — printable ASCII by
// construction, a few kilobytes — and `readDigest` refuses a line that is anything else: one
// that holds a backslash or a character outside ASCII, one that names no file for what it
// leaves out (the line the tool prints unasked, the state document in it), and one whose
// `unfit` says that a value did not fit its shape. NO STATE DOCUMENT, NO LEDGER ROW, NO
// BRIEF, NO DOOR LIST AND NO REFUSAL'S PROSE PASSES THROUGH THIS SCRIPT. Where an agent
// needs one, its prompt names THE READ and the agent runs it: an item's row with its brief
// (`dev/stabilize-record item`), the doors of a round an item covers (`item-doors`), the
// ledger's rows whose triage is not finished (`untriaged`), a pending batch's subject
// (`pending`). Where the orchestrator needs one, the return names THE FILE: the state
// document (`named.document`), a refused step's own line with its halt report (`step.file`)
// — each by its path and its sha256, read by a subagent. What a return holds of the state is
// what the digest holds: words, ids and counts. What an AGENT returns of its own judgment —
// a finding, a grade, a verdict, a scope's derivation, its own halt report — is no line of a
// tool, is held to no hash, and is passed on as it came.
// A DIGEST IS COPIED BY A MODEL, AND A COPY SLIPS WHERE THE LINE REPEATS ITSELF (the staged
// canary's part 1, 2026-10-16: one `retry` of the state's clause table came back as the next
// rows' word, on three tries of three, and the stage halted). So a line that does not hash is
// put back where ONE of its values, replaced by another value the line holds under the same
// name, makes it hash (`mendSlip`) — every act's line, at every read: the line that hashes is
// the tool's, and nothing an agent said is believed for it. Any other alteration is what it
// was: the state read is asked for again, twice, and every other step halts. A probe reads
// the relay exactly, and mends nothing.
//
// WHAT AN AGENT OF A STAGE TYPES (the second repair plan's task K11; DECISIONS.md ->
// 2026-10-07, "What the harness tells an agent to run"). The human's ruling of 2026-10-07:
// an agent this script launched, told to start a command "in the background" and wait "in
// slices", composed a waiting shell no permission rule can name and stopped on a prompt —
// "there should be tooling for this". So EVERY COMMAND A PROMPT OF THIS SCRIPT NAMES IS ONE
// PLAIN INVOCATION OF ONE OF THREE COMMITTED TOOLS — `dev/stabilize-step`,
// `dev/stabilize-record`, and in a probe `dev/stabilize-probe` — rendered by one function
// (`typed`): no pipe, no redirect, no `&&`, no substitution, no loop, no second command,
// and no other program. The gate, the regression set, cargo, git and a hash are reached
// THROUGH the step tool's acts. A LONG COMMAND IS HELD BY THE TOOL (`hold`): one step
// starts it, one step asks after it — its agent asks again while the tool says `running`,
// and composes nothing — and the tool judges it. WHERE AN AGENT HANDS A WRITER A TEXT — its
// report, the round's scope, a record's batch — it writes a file WITH ITS FILE TOOL, under
// the scratch root, at the path this script names, and the writer is called with that path
// (`--from`). No prompt tells an agent to wait, to send anything to the background, to
// redirect, to pipe or to poll by its own means; tooling-tests/stabilize_harness_fence.rs,
// arm (s), composes every prompt and holds each to that, and names its exceptions: the
// independent drive of a proposal (a patched tree is no commit, and no held kind builds
// one), the cross-model pass (a tool of another family, launched only for an item the
// invocation names), and the re-cut of a part (the `fix` half's, about to go). WHAT A ROLE
// RUNS OF ITS OWN CRAFT — a reviewer's rig, a driver's commands on the binary, the
// preflight's read of CI, the trial image — is named by no prompt and held by no fence.
//
// THE BOUND ACROSS ROUNDS AND THE STOP MODE (ruling 6) are the record script's too: the
// run's opening writes them (`dev/stabilize-record run-set`), `next` is `stop` where the
// human is asked before the step that follows, and the position refuses a stage of a run
// whose opening is not done (`not-ready`), a round after a stop the human has not lifted
// (`stopped`) and one more fix round once the bound is spent (`round-bound`). This script
// holds neither number and starts no stage the position refuses.
//
// WHAT THE HUMAN RULES ABOUT THE RUN is recorded like every ruling: by the one step that
// records `args.rulings`. A go after a stop, one more re-run of a clause that is still not
// green, and a raised bound are facts of the record (`round-set`, `run-set`) — an invocation
// that carries them records them on the loop branch, starts nothing, and returns the state's
// `next`, which then names the step the stop was holding. SO ARE THE RULINGS ABOUT THE QUEUE
// OF A RUN THAT HAS ONE (the reshape's plan, T7; QUEUE_RULINGS): ANOTHER ORDER of its
// classes, THE ROWS THE HUMAN PUTS INTO IT BY KEY, and ITS BUDGET, RAISED — each one call of
// `run-set`, in that order whichever was said first, in the batch of the one step. The record
// script holds each to the state as it stands before its call, a call of a batch reads what
// the calls before it wrote, and a batch is written whole or not at all: a raise is taken
// only while the budget is spent and a row of the queue is unreached, an order or a key may
// be what puts a row there, and a raise refused on the state they leave refuses them with it.
// After such a ruling `next` is `verify` wherever a row of the queue has budget left — also
// where the run was stopped after a round whose rows were all off the order: the round is
// open again, no go is owed, and the invocation that follows continues the stage. THE NUMBER
// A VERIFIER HANDS ON IS NO RULING: that script writes it once, and an invocation that
// carries one is refused before any agent (ONCE).
//
// AN UNFINISHED TRIAGE (`next: 'triage'`) is finished by the stage that left it: the position
// marks that stage `triage: true`, and its next invocation grades, verifies and records the
// rows the state lists as untriaged and runs NO instrument — every report of the round is on
// record already. And every stage's triage is handed those rows beside its own reporters'
// findings (`ledgerSource`): a row seeded at the opening, and an entry an earlier stage left
// without a verdict, never wait for a stage of their own.
//
// A RUN WITH A QUEUE TAKES ANOTHER PATH FROM ITS TRIAGE ON (the build plan of the reshaped
// `test` stage, completions/artifacts/M55/stabilization-build/reshape/plan.md, §4, Phase B;
// DECISIONS.md -> 2026-10-12, "The harness: the new path's triage"). A run whose opening
// holds an `order` has a queue, and its state's digest says so (`queue`; `hasQueue`, the ONE
// place the two kinds of run are told apart). Up to its instruments such a run's `test`
// stage is the stage above. Then THE STAGE HAS ONE TRIAGE (`triageOnce`), and it is handed
// EVERY SOURCE, EACH FINDING UNDER A NUMBER: what the reporters returned and what the scope
// step left open; EVERY RED RESULT OF THE INVOCATION, as a source of its own under the key
// the record script files its row with (`red-<item>` — `redSources`), graded from the file
// the tool kept its verdict in or, for a check an agent read, from that preflight's report;
// and the rows nobody graded, by the read that prints them. ITS ENTRIES NEVER PASS THROUGH
// THIS SCRIPT: the triage writes them to ONE file and hands it over by the call its prompt
// spells whole (`dev/stabilize-record triage`, one `<source>=<count>` word a source), the
// record script holds the partition — every number of every source in exactly one entry —
// and renders the triage's report from the file. What comes back is that call's line, as
// much of it as this script holds: the path, the hash, and what each source handed
// (`filedFault`). THEN THE STAGE'S FIRST RECORD, BEFORE ANYTHING IS VERIFIED (that plan's
// T5b; the ruling of 2026-10-11, 1: a stage records twice — until then nothing a stage found
// was on record before its one record at the end, and a stage that died while it verified
// lost every row). THE RECORD IS COMPOSED BY THE TOOL, AND NO AGENT TYPES A BATCH
// (`composeRecord`; that plan's K3): one step — `dev/stabilize-step compose` — is handed
// PLAIN WORDS (`recordWords`: the reporters that left a report, the commit, the gate's job,
// what each item did, the round's facts) and THE SHA256 OF THEIR CANONICAL FORM, which this
// script computes (`wordsText`: the run, the round, the attempt and the subject, a line each,
// then a line a word, each ended by a line feed) and the record script computes again over
// what the step typed. EVERY ROW IS THAT SCRIPT'S, read from what was filed: the entries, a
// held check's verdict, a red check's report. A VOID SAYS WHY IN ONE WORD (VOID_WORDS), never
// in a sentence. Then the record's gate, its one commit and its push, as above, and the stage
// returns `hunted` with the state's `next`. HOW THE COMPOSING STEP CAN BE REFUSED is one
// table (COMPOSE_REFUSALS): `words` — a word was altered on its way through the step's agent
// — is asked for again, exactly, once; every other word halts the stage at its record with
// nothing written. THEN THE STAGE VERIFIES, IN WAVES, IN THE SAME INVOCATION (that plan's T6a;
// the rulings of 2026-10-10, 1 and 4, and of 2026-10-11, 1 and 2). WHAT IS VERIFIED, IN WHICH
// ORDER AND HOW MUCH OF IT IS THE RECORD SCRIPT'S, read as the state's digest after the first
// record and AGAIN AFTER EVERY WAVE (`verifyWaves`): a wave is the keys the queue prints
// (`queue.first`), in its rank — AT MOST EIGHT (WAVE), NEVER MORE THAN THE BUDGET HAS LEFT
// (`queue.left`), and never a key this invocation launched already: a verdict is filed under
// the reporter `verify-<key>`, and a reporter is launched once an attempt. This script keeps
// no order, no budget and no list of what is verified; it launches a verifier for NO KEY THE
// QUEUE DID NOT PRINT, and reads NO ROW: a verifier is handed ITS KEY and the read that prints
// its row (`dev/stabilize-record row`), the two binaries, the lookup kit's call, which is its
// first read (`kit`), how many items it may hand on to the queue — the opening's number, as
// the digest has it — and THE ONE PLAIN CALL THAT FILES ITS VERDICT, spelled whole
// (`verdictPrompt`: `dev/stabilize-record verdict`, the run, the round, the reporter, the
// attempt, the file it reads). THE TWO HASHES ARE MEMBERS OF THAT FILE AND NO WORD OF THE CALL
// — the plan had them as plain words; the committed call takes none. A VERDICT COUNTS FROM
// THE MOMENT IT IS FILED, AND NOT ONE WORD OF IT PASSES THROUGH THIS SCRIPT: what a verifier
// returns is the path and the word its call printed, and `contested` as that line has it,
// HELD TO WHAT THIS SCRIPT CAN HOLD (`verdictFault`: the one path a filing of that reporter,
// attempt and round has; a word a filing says; a truth value; the hash it asserted) — AND THEN
// TO THE QUEUE, READ AGAIN: a finding the queue prints again after its wave HAS NO VERDICT,
// WHATEVER ITS VERIFIER RETURNED, and is named so; a return that is not held, of a finding
// the queue does not print again, is named too, and decides nothing — the filed file does.
// `undrivable` is a filed verdict like any: the queue does not print its key again, and no
// code here knows the word. WHAT A VERIFIER HANDS ON is a row of the state from the moment
// its filing is taken, and is in the next read of the queue. A CONTESTED VERDICT IS THE
// HUMAN'S BY ITS VERIFIER'S OWN FILING: nothing is launched and nothing typed for it. After
// each wave THE REPORTS ARE CHECKED, as after every other launch. THEN THE STAGE'S SECOND
// RECORD, where a verifier was launched: `composeRecord` again, with a subject and a
// directory of its own, over EVERY REPORTER OF THE ATTEMPT THAT LEFT A REPORT — the record
// script holds a record's report check to the whole attempt — and no other word: the script
// takes the filed verdicts, the handed-on rows and what a stop shows itself. REFUSED AS ONE
// THAT CHANGES NOTHING (`no-change`), IT IS NO HALT: nothing was filed, and the stage returns
// as it hunted. The stage returns `verified` where its second record was made, else
// `hunted`, with the state's `next` and WHAT THE PHASE DID AS COUNTS (`verify`) — how many
// verdicts the state counts since the first record among them, the record script's number
// and never this script's. No key is in the return: each finding a launch left without a
// verdict, each return that was not held and each that says contested is A LINE OF THE LOG,
// by its key, with why.
// AN INVOCATION THAT CONTINUES (that plan's §3 and T6b; the rulings of 2026-10-10, 4: nothing
// runs twice). A stage that returned or died after its first record leaves the round where
// the state's position says `verify` — a row of the queue is unverified, and budget is left
// — or `triage` — a row nobody graded is left, which no triage saw: a red row a batch filed
// under its own key, a seed added since — or `filed`: A FILED VERDICT COUNTS AND NO RECORD
// TOOK IT, which is all a stage leaves that died after its last verifier filed, or after the
// wave that spent its budget: the state then hands no fixer, no ruling and no close before
// the stage's record (dev/stabilize-record: A FILED VERDICT IS RECORDED BEFORE ANYTHING
// FOLLOWS FROM IT). THE INVOCATION THAT FOLLOWS CONTINUES THE STAGE
// (`continuing`, `continueTest`), FROM THE COMMITTED STATE AND FROM GIT: its two reads — a
// record that was applied and not committed is finished by the first, and by nothing else
// of that invocation (`finishRecord`) — its attempt begun on record, on the candidate the
// round tested, and the two binaries, answered by the builds that stand. NO INSTRUMENT RUNS:
// no preflight, no scope step, no gate, no held check, no reporter of an item. A TRIAGE RUNS
// ONLY OVER ROWS NOBODY GRADED — one source, the read that prints them — AND NO ROW IS GRADED
// A SECOND TIME; what it graded is recorded before anything is verified, composed by the
// tool from that attempt's rendering. THE FILED VERDICTS ARE TAKEN AS THEY STAND — the state
// counted each from the moment it was filed, so the queue prints none of their findings, and
// `undrivable` is one of them — AND ONLY WHAT IS MISSING IS LAUNCHED: the same phase
// (`verifyThenRecord`), under the continuation's own attempt, so that a finding a launch
// left without a verdict gets a verifier whose report and whose filing are names the record
// script takes, and whose launch that script counts — left so twice, the finding is the
// human's. ITS RECORD AFTER THE PHASE takes what its verifiers filed, what an earlier
// invocation's had — THE STATE'S POSITION SAYS THAT ONE IS OWED A RECORD (`filed`), and with
// that word alone the invocation launches nobody and records, ITS REPORT CHECK FIRST (below:
// REPORTS) — and every report an earlier invocation left pending. AND WHERE IT HAS NOTHING
// TO RECORD — no
// verifier of it left a report, no filed verdict is owed — IT COMPOSES NONE: its attempt is
// then one that left its marker and no record, the record script counts those, and after
// two in a row the stage is the human's. A finding whose verifier leaves nothing at all has
// no other counter.
// A WAVE THAT IS THE LAST (that plan's §2.2 and E7). The account's usage limit never reaches
// this script as a message — it is handed nothing. So A VERIFIER CALL IS NEVER TRIED TWICE
// (`agentR`, `once`): one that returns nothing leaves ITS finding unverified, named, and the
// phase goes on; A WAVE IN WHICH TWO CALLS RETURNED NOTHING (SILENT_LAST) IS THE LAST — its
// reports are checked, the queue is read, the stage records what was filed, and no further
// wave is launched. AND A STAGE WHOSE STEPS THEN RETURN NOTHING TOO HALTS AS WHAT MAY BE THE
// ACCOUNT'S LIMIT, NAMING THE FILED VERDICTS (`limitSaid`): by the path of each one this
// invocation's verifiers filed, by the state's count, by the directory they lie in and by
// the state document that lists them. THE HALT IS HANDED WHICH STEP HALTED (LIMIT_STEPS: the
// wave's report check, the read of the queue, the composing step, the record's gate, its
// commit, its push, the read-back), AND SAYS ONLY WHAT IS SO AT THAT STEP: before a read
// after the wave, that the document it names predates the wave, and the one plain call that
// prints the state now; that the filed verdicts are in no commit only while none can hold
// them — where the commit step itself returned nothing it says that it cannot tell, and from
// the push on it names the commit that took them; and that the stage is simply continued
// only at the two steps before its record — after them the halt of the record step, of the
// push and of the read-back already says what the next invocation does first. EVERY OTHER
// ROLE IS RETRIED AS IT WAS, bounded by the breaker — which is asked before every try, and
// no longer only before a call's first.
// WHAT AN INVOCATION OF THAT PATH RETURNS (that plan's T7; the row `wf-return-names`: three
// names in a stage's return read other than they meant). ITS STATUS IS ONE OF THREE WORDS,
// EACH FOR WHAT THE INVOCATION ITSELF PUT ON RECORD (`statusOf`): `hunted` — a stage whose
// findings are on record, and which made no record of a verify phase: it had nothing to
// verify, or its verifiers left nothing; `verified` — the record of the verify phase was made,
// by a stage or by an invocation that continued one; `continued` — an invocation that
// continued a stage and made no such record: it hunted nothing, and does not say that it
// did. `next` IS THE STATE'S, VERBATIM, and `verify` is a value this script knows: the `test`
// stage again, continued. BESIDE `next` EVERY RETURN OF SUCH A RUN — a stage's, a
// continuation's, a ruling's, a halt's as the state was last read — SAYS WHAT THE RECORD
// SCRIPT COUNTS, AS THE STATE'S DIGEST HAS IT (`queueSaid`): THE BUDGET (its limit, what is
// spent, what is left), THE QUEUE'S LENGTH, in all and class by class, THE HIT RATE and THE
// STOP'S VIEW by its path and its hash, with how the file on disk stands to that hash — and
// WHAT AN INVOCATION OF `test` CONTINUES THE STAGE FOR (`continues`): `triage` or `verify` —
// grading or verification is owed — or `filed` and nothing else — ONLY A RECORD IS OWED, and
// the invocation that follows launches nobody — or nothing. None of it is counted here.
// AND NO LIST OF THIS SCRIPT'S OWN MAKING: what a stage launched, what its agents returned,
// the items it voided and the doors nobody covers are HOW MANY in a return, and names in the
// log and on record; the lists a return holds are the step tool's — a digest's, each bounded
// by its shape — and the position's words. A HALT IS THE EXCEPTION, AND SAYS SO: it names the
// reporters it launched, and the limit's halt the paths of what was filed — what must not be
// lost before the stage is invoked again. WHAT NO STEP PRINTS IS IN NO RETURN: which findings
// are the human's and which of them a verifier contests are the state document's and the
// view's file's, each named by its path and its hash.
// NO ADVOCATE IS LAUNCHED ON THAT PATH: a contested fix is the `fix` half's. A RUN WITHOUT AN
// ORDER TAKES THE PATH ABOVE, UNCHANGED.
//
// WHAT A STAGE HOLDS IS NAMED BY ITS COMMIT, AND NOTHING IS BUILT TWICE (that plan's T5b; its
// review's S12 — for BOTH kinds of run). The two builds, the candidate's gate and every held
// check were named by the stage's ATTEMPT, so a stage that ended before its record ran all
// of them again: forty-five minutes of gate and regression set on the canary's numbers. A
// build is named by its commit, the candidate's gate by the candidate's commit, and a held
// check by that commit and THE ITEM'S OWN RUN IN THE ROUND — so a stage started again asks
// for the names the stage before it asked for and is answered by the jobs that stand, and an
// item's re-run, on the same candidate, asks for the next. A JOB THAT STANDS UNDER A NAME IS
// TAKEN ONLY WHILE IT RUNS OR WHERE ITS VERDICT IS GREEN (`hold`, `stands`): one that ended
// red, void or dead is PASSED OVER for the next name — `<name>-2`, `<name>-3`, … — as a
// record's gate always was: a red gate a starved machine left is measured again, and never
// answered for ever. THE ANSWER'S OWN WORD DECIDES THAT, WHATEVER THE START STEP DID
// (`passedOver`): the tool says `started` to the one call that started a job and nothing of
// who started one that is there, so a job that answers a start as ENDED is never taken for
// one this invocation must run itself — not on the step's retry either. What a name CANNOT
// hold is that a gate's verdict is of the tree its
// commit names: the gate reads the working tree, and the tool's own check of that is owed
// (the row `wf-held-gate-tree`).
//
// AND WHAT THE PREVIOUS RELEASE'S COMMIT ALONE DECIDES IS BUILT ONCE (that plan's K8b — for
// BOTH kinds of run). About half of a regression set is the previous release's: its tree, its
// built tests, their run on their own binary. The regression tool keeps those in a directory
// its caller names (dev/regression-set: A KEPT BASELINE), and THE STAGE NAMES IT: a held
// regression set is started with `--baseline`, a path below the scratch root that is the
// previous release's commit's and nothing else's (`baselineOf`) — no candidate, no round, no
// attempt, no run and no job is in it. So a stage started again, an item's re-run, the next
// round's candidate and another run under the same scratch root all name ONE directory, and
// it lies in no job's own. NAMING IT IS NO KEY: that tool uses what is kept there only while
// its own key holds — eight members, held one by one — and builds afresh otherwise, in the
// same run and with the same verdict. What became of the baseline is in that tool's line —
// in the verdict's file, and on record with the check — and no step of this script reads it.
//
// A STAGE THAT IS NOT FIT FOR USE REFUSES TO START (the human's ruling of 2026-10-06 on the
// order of the repair: the `test` half first). The build was reviewed red, and the `fix`
// stage is named in NOT_FIT: an invocation of it returns `refused` before any agent runs,
// whatever its arguments — an exit at a bound included — until its own repair and the
// re-review of that repair are recorded. The one invocation that is not refused starts no
// stage: one that only records what the human ruled about the run (below).
//
// THE STAGES (ruling 9).
//   test   git state (the path-class assert) -> state -> THE ATTEMPT BEGUN ON RECORD ->
//          (preflight -> HELD: the candidate's binary, the previous release's, the
//          candidate's gate) ∥ scope -> state -> HELD: the round's held checks, one after
//          another -> the round's instruments, in parallel -> the reports checked -> per
//          pass: triage -> verify-real -> per fork an advocate and an independent drive ->
//          the reports checked -> the record step (its batch applied from a file; HELD:
//          the full gate) -> the record's one commit on the loop branch, held to the
//          candidate's gate -> push -> state.
//          Returns {status: 'triaged', round, candidate, counts, human_list, forks, next}.
//   fix    git state -> state -> the round's branch found -> [the human's rulings
//          recorded] -> per cycle: the round branch opened -> fixers, one area at a time,
//          serial, one finding per commit -> push -> the round-tip binary -> the audit of
//          the fix diff, in parallel -> triage -> verify-real -> the record step on the
//          round branch -> push -> state -> land when nothing is left open, else the next
//          cycle while blockers remain and the bound allows.
//          Returns {status: 'landed', candidate, cycles, counts, doors_for_retest}; at the
//          bound {status: 'halted', halted: {phase: 'bound'}}, unlanded; on a fixer's
//          new-mechanism halt {status: 'halted', halted: {phase: 'fork'}} with the
//          advocate's case and the independent drive.
//
// THE BOUND AND ITS EXITS (ruling 6). A round has at most DEFAULT_CYCLES fix -> audit
// cycles; the next one is not started, and the stage returns `phase: 'bound'` with the
// round unlanded. The human's exits, each an invocation of `fix`:
//   continue      args.raise = { cycles: N }   — N above DEFAULT_CYCLES, passed again on every
//                                                later `fix` of that round (nothing is
//                                                remembered between invocations)
//   drop          args.exit  = 'drop'          — the round's record says so, its fixes are
//                                                marked open again, and exactly its record
//                                                commits are carried over to the loop branch
//   land a part   args.exit  = { part: [sha…] } — the fix commits the human keeps are re-cut
//                                                as a branch tip of their own, gated and
//                                                audited once more as exactly that diff,
//                                                and landed if that audit finds no new blocker
//
// THE GIT ACTS. Every git act of a run — and every read of its record — is ONE command of
// `dev/stabilize-step`, which does the act, checks it and prints ONE line — asked for its
// digest, as every step of a stage asks: a git step's
// prompt is that command and "relay its line", and `readDigest` holds the relayed line to the
// hash it ends with, and to being a digest, before anything here reads it. A step whose
// agent returned nothing is asked for again as THE ONE COMMAND, RUN AGAIN: the tool says
// itself what an earlier run of it left. What an act runs and what it refuses is
// that tool's (its header); which act a stage asks for, in which order and with which names,
// is this script's. ONE step is still a list of commands the agent follows: the re-cut of a
// part (`carryPrompt` with a part), because that exit is ruled to be rebuilt as a revert
// (DECISIONS.md -> 2026-10-06) and a mechanism about to go is not ported.
//
// BRANCHES (ruling 7). Every branch name is minted in ONE function, `branchName`. A round's
// branch is opened from the loop branch, pushed by name and landed `--no-ff`; product paths
// (PRODUCT_PATHS) move only through such a merge, and a git step asserts it by path class.
// The `test` stage's records land on the loop branch; the `fix` stage's ride the round
// branch and reach the loop branch when the round lands, or by the carry-over when it is
// dropped (adjustment iv). No step force-pushes, rebases or touches `main`.
//
// REPORTS (ruling 12). Every reporter writes its report through `dev/stabilize-record`,
// under the name this script hands it on its `REPORT:` line. The script keeps the list of
// every reporter it launched and has `check-reports` run on that list — after the
// instruments, after every triage pass, and inside the record's batch. A file nobody
// launched stops the stage. A LAUNCHED REPORTER WITH NO REPORT DOES NOT: it is taken off
// the list, and what it was launched for is recorded as not done (below).
// AND IN A RUN WITH A QUEUE EVERY RECORD THIS SCRIPT COMPOSES OR APPLIES IS PRECEDED, IN THE
// SAME INVOCATION, BY THE REPORT CHECK (the scoped review before the parts, its finding 2
// here): it is the step that SETS ASIDE WHAT THE TOOLS CAN SET ASIDE — a file a hand, an
// editor or a file browser left among a round's filed verdicts or beside a stage's reports
// — and a record that meets such a file first is refused over it, at every invocation, or
// takes its batch back at its commit, a gate spent. After the instruments, a triage or a
// wave of verifiers the check is that step's own. A RECORD COMPOSED WITH NO WAVE BEHIND IT —
// an invocation that continues a stage for its record alone (`filed`) and launches nobody —
// asks for it first, over the attempt's own reporters (`verifyThenRecord`). AND THE BATCH OF
// THE HUMAN'S RULINGS is preceded by THE CHECK'S FIRST HALF ALONE, THE VET (`ruleTheRun`;
// `checkReportsPrompt` without names): such a record names no stage, no attempt and no
// reporter, so nothing of it could be held to a list. What either of those two checks set
// aside is a line of the log (`asideSaid`). A RUN WITHOUT AN ORDER RECORDS ITS RULINGS AS IT
// DID: nothing is filed there, and the record script names no stray file of such a run as
// pending.
//
// AN ATTEMPT BEGINS ON RECORD. The first step of an attempt that will launch an agent is
// `dev/stabilize-step begin`: it holds the round and the attempt to the state's position
// once more, and writes the attempt's marker — a report of this script's own, under the
// reporter ATTEMPT. So an attempt whose agents all died before one of them wrote is still an
// attempt the record script counts, and after two that reached no record the next one is
// the human's to grant. An invocation that is refused, or returns before that step, began
// no attempt.
//
// HOW AN AGENT CAN END, AND WHAT FOLLOWS (the harness review's M5 and M9). An agent returns
// a result, halts with a report, halts without one, returns nothing after its retries, or
// writes its report and dies. WHETHER A REPORT IS THERE is never taken from a return: it is
// what `check-reports` says. A result counts only with its report on disk; a report with no
// result stays on the list and is committed, and counts for nothing. Per reporter:
//   preflight (first), scope    no result, or no report: the stage halts — nothing can be
//                               asserted or selected without them, and nothing is built
//                               after a preflight that did not assert. The attempt is counted.
//   preflight (second)          no result, or no verified image: the trial arms and the
//                               checks it was asked for are VOID with that reason, and the
//                               stage goes on.
//   a step of an item's chain   no result, or no report: the item is VOID with the step and
//                               the reason, the rest of its chain is not launched, and
//                               every other item goes on. The state then asks for the
//                               item's re-run. A finding it did return, with its report, is
//                               triaged all the same.
//   the cross-model pass        ANY of these: that pass is void FOR THAT PASS, recorded as
//                               not run, and its item's own passes go on. It never halts
//                               the stage and never voids its item (the human's rulings of
//                               2026-10-06: limited and rare, never required).
//   triage                      no result, or no report: the stage halts — a finding nobody
//                               graded has no row a later stage could read. Counted.
//   a verifier                  no verdict, a verdict for another finding, or no report:
//                               the finding stays UNVERIFIED; the record script counts the
//                               passes that left it so, and after one more it is the human's.
//   an advocate, the            no case, no drive, or a report missing: THE FORK IS NOT ON
//   independent drive           RECORD and its finding stays unverified, as above — the next
//                               triage verifies it and drives the fork again.
// A RETURN THAT LACKS A FIELD THE STAGE NEEDS OF THAT ROLE (the second repair plan's task
// X1; DECISIONS.md -> 2026-10-07, "An absent field voids its item"). Observed on 2026-10-07:
// a return that omits a field its schema REQUIRES does not come back partial — the call
// fails, three whole runs of the agent. So the fields below stay OPTIONAL in every schema of
// a stage, the script is handed the return without one, and the absence is THAT REPORTER'S
// FACT — the row above for that reporter, with a reason that names the field and the
// reporter — never the stage's (NEEDS, the one table; the fence holds it to the schemas):
//   a step of an item's chain   no `asserted_sha256`: the item is VOID, the rest of its
//     (a reviewer, a driver)    chain is not launched, and the state asks for its re-run.
//                               WHAT IT FOUND IS TRIAGED ALL THE SAME: a finding's text and
//                               repro do not rest on the hash, and whatever is graded a
//                               break is driven again by a verifier on the binary it asserts.
//   a verifier                  no `verdict`, or no `asserted_sha256`: the finding stays
//                               UNVERIFIED. What it left open is triaged all the same.
//   an advocate, the            no `asserted_sha256`: THE FORK IS NOT ON RECORD, the finding
//   independent drive           stays unverified, and the proposal of a case argued on a
//                               binary nobody asserted is not driven.
//   a verifier, `confirmed`     no `regression`, or no hash of the previous release's binary
//                               (`ran_on.previous`) — or one that is another: THE VERDICT
//                               STANDS, AND ITS REGRESSION FACT IS UNKNOWN (`regressionOf`).
//                               The record takes `confirmed` with the field ABSENT — never
//                               `false` for a fact nobody established, and never an
//                               unverified finding — and the state names the finding
//                               (`regression_unknown`), which every return counts. A door
//                               the previous release does not have is no such gap: it is
//                               `not-comparable`, a fact, and goes as it is.
// AND A HASH THAT CAME BACK AND IS NOT THE BINARY'S IS THE SAME CELL as one that did not come
// back — a step's item void, a finding unverified, a fork not on record — with both hashes in
// the reason, and the return naming the reporter (`unasserted`). Until this task it halted
// the stage. THE ARGUMENT: a definition tells a driving agent to HALT where the binary it is
// handed does not hash to its line, and that halt voids its item and nothing else; the
// comparison this script makes is that halt, made for an agent that did not make it. So the
// same fact — this one agent drove something else — costs the same whether the agent
// noticed. And it is that agent's alone: every other agent's evidence is held to its own
// assertion, the binary is the tool's and placed read-only, and a binary swapped under a
// whole stage shows as every later reporter void, on record, by name — where a halt records
// nothing and runs every instrument again, to meet the same agent.
//
// WHAT STILL HALTS A `test` STAGE ONCE AN INSTRUMENT IS LAUNCHED is ONE LIST (HALTS_AFTER),
// and a halt in that stretch names its row (`haltAfter`): a reporter name that cannot be
// made (`names`) · the report check, unread or naming a file nobody launched
// (`report-check`) · the round's scope set aside (`scope-aside`) · a reporter the stage
// stands on that left no report (`stood-on`) · ANY ending of triage but its grades with its
// report (`triage`) · the record step
// (`record`), its push (`push`), and the state read back after it (`read-back`) — these
// three with the batch applied or the record committed, and nothing lost. Nothing else
// halts there: a list that grows is a row added, or the fence is red.
//
// HOW A STEP CAN END, AND WHAT FOLLOWS (the second repair plan's task K3). A step of the
// tool ends with its status, or refuses with ONE WORD — and this script has a row for every
// word (STEP_REFUSALS: whose the state is, and what leaves it; the fence holds the table to
// the tool's words), so no word falls into a sentence that says nothing. The rows a stage
// meets, by the step:
//   the first read (`git-state`)  RECONCILES FIRST: a killed write is finished, a batch whose
//                               commit is made is settled, and a record that was committed
//                               and not pushed IS PUSHED — the stage then starts from that
//                               answer, and its return names what was met (`arrived`).
//     an applied batch            the invocation gates and commits it, pushes, and starts
//                               nothing else (`finishRecord`).
//     under stopAfter: 'state'    THE READ ONLY LOOKS (`--look`): nothing is finished, no
//                               head moves, and what the next invocation would finish is
//                               returned as `owed` — a pending batch and an owed push among
//                               them — with status `stopped`. (A filed verdict no commit
//                               holds is owed to the stage, and finished by no read: the
//                               look goes on to the state, and says what the stage is
//                               continued for.)
//     locked                      the orchestrator's: git's own lock stands. No git process
//                               running, the file the step's line names is removed, and
//                               the stage is invoked again.
//     foreign-commit              ITS AUTHOR'S, never the human's and never this script's:
//                               a commit outside the run's directory that the remote lacks
//                               is pushed by name, `git push origin <branch>`, and the
//                               stage is invoked again.
//     unvetted (the owed push)    the human's: a local commit that would not have been
//                               written is his to take back. Nothing is pushed.
//   the report check              a file that is no report is SET ASIDE (`aside`), and its
//                               reporter has left none: the row of that reporter above.
//                               The round's scope set aside: the round has no scope, and
//                               the stage halts there.
//   the record's batch (`apply`)  refused `no-change` — the same ruling, sent twice: the
//                               invocation returns `refused`, nothing is written, NO BATCH
//                               IS LEFT and no discard is asked for.
//   the record's commit (`record`), each a halt that says what the next invocation does:
//     gate-red                    the batch stays applied: the next invocation gates it
//                               again and commits it, or `dev/stabilize-step discard`
//                               takes it back.
//     unvetted                    THE BATCH IS TAKEN BACK ALREADY, and what was no report
//                               left the tree: the next invocation works the next attempt.
//     head-moved                  somebody committed while the stage ran: the batch stays
//                               applied, and the stage is invoked again.
//     recorded, asked again       a commit step whose agent died is asked for again — the
//                               ONE command, run again — and answers what it answered.
//   a record's push               that failed: THE PUSH IS OWED, and the next invocation's
//                               first read makes it. Refused `unvetted`: the human's, and
//                               never the push that is owed — it is refused again until the
//                               commit is taken back. Refused `foreign-commit`: its author's.
//   a held command (`hold`: one step starts it, one asks after it; read by `heldOf`). WHAT
//   A STAGE HOLDS, AND WHICH ENDS IT TAKES, is one table (HELD_TAKES); every other end is a
//   HALT that names the job — but for a held check, which is then VOID BY A WORD:
//                    green        red          void         dead         running | unread
//     a build        the binary:  HALT         HALT         HALT         HALT — the job
//                    path, hash,  (`build`)                              is named, and
//                    version are                                         the one call that
//                    the tool's                                          answers once it
//     the candidate's a fact; the  a fact; the  HALT         HALT        is over; the next
//       gate         file is on   red is on    (`gate`): no record       attempt starts
//                    record       record       could be held to it      its own
//     a held check   ITS RESULT IS THE FILE THE TOOL KEPT   VOID BY A WORD (`heldVoid`):
//                    ITS VERDICT IN, and no word: red is    the stage goes on and records
//                    filed as a finding by the record       it void, a run that counts; the
//                    script, void is a void run with `why`  state asks for its re-run where
//                                                           it owes one; the return names it
//     a held gate    answered from the candidate's own gate, which is not run twice; in a
//       of the set   re-run NOT RUN — it reads the tree — and it is void by a word (below)
//     a record's     the ONE commit step, handed the file   HALT (`record`): nothing is
//       gate         the tool kept the output in, and held  committed, THE BATCH STAYS
//                    to the candidate's gate there          APPLIED, and the next
//                                                           invocation gates it again
//                                                           under the next name — or,
//                                                           where NO NAME IS LEFT for a
//                                                           pending batch's gate, the
//                                                           halt says so and names the
//                                                           batch taken back (`no-name`)
//   AND WHO ASKS AFTER IT:
//     the start's agent died      the start is asked for again — its ONE command — and the
//                               tool answers with the job that is there: nothing starts twice.
//     the asking agent died       THE WAIT IS TAKEN OVER: the step is asked for again, and
//                               its one command is the wait, never the start. The command,
//                               which outlives every agent, ran once.
//     a turn ended on `running`   the next agent takes the wait over, HOLD_LAPS in turn.
//     a name that was there       (a record's gate) a job an earlier invocation started
//       before the stage asked    under the name is that invocation's, a verdict of the
//                               tree as it stood then: the next name is asked.
//                               (a build, the candidate's gate, a held check — named by
//                               their commit) it is TAKEN while it runs or where it is
//                               green; red, void or dead it is passed over for the next.
//                               WHICH JOB WAS THERE IS THE TOOL'S ANSWER, NEVER THE COUNT
//                               OF THE START STEP'S TRIES (`passedOver`): `started` is this
//                               call's; an END is never the answer to a job the invocation
//                               must run itself — on the step's retry either, where it may
//                               be a dead try's own job that ended, at the cost of one more
//                               gate; only a record's gate that still RUNS is told by the
//                               tries — an earlier invocation's on the first, the dead
//                               try's own on a retry.
//
// A FORK IS ON RECORD (the harness review's M3). A verdict the verifier found to contest a
// settled decision is recorded with that verdict, in the round's triage record: its kind,
// the verdict of the advocate's case, and whether the independent drive holds. The record
// script routes such a finding to the human until the human rules it, so `next` is `rule`
// and never `fix` or `close`; the ruling is one of the three dispositions, written by the
// one step that records `args.rulings`.
//
// ONE BINARY PER CANDIDATE (ruling 11). THE STEP TOOL BUILDS IT, from the commit and never
// from the working tree, as a held command (`buildBoth`), and its path, its hash, the
// version it prints and that a bare `jigc` resolves to it are the tool's facts, read off
// its verdict — no agent's reading. Every driving agent is handed the path and the hash on
// a `BINARY:` line and returns the hash it asserted; this script compares the two itself,
// and what an agent drove on anything else — or on a binary it did not assert — is evidence
// about nothing: its item is void, its finding unverified, its fork none (above: A RETURN
// THAT LACKS A FIELD).
//
// A ROUND TESTS ONE CANDIDATE, AND EVERY RESULT OF IT NAMES THAT COMMIT (ruling 11; the
// orchestrator's ruling of 2026-10-07 on the core review's F3: a result recorded on another
// commit made `close` computable over greens of a tree the round never tested). The round a
// `test` stage begins has no candidate on record: it is the branch's tip, which the first
// read returned, and the stage's record names it. A RE-RUN is an attempt inside a round
// that has one, and by then the round's record commits lie on top of it — so the tip is
// another commit: a re-run builds the round's candidate from its commit, as the state's
// digest names it, hands every driving agent that binary, begins its attempt on that
// commit and names it in every result. The tip is named to ONE step — the record's commit,
// which is held to the commit the stage began on (`--head`) — and to no writer. AND A CHECK
// THE PREFLIGHT RUNS READS THE WORKING TREE, which in a re-run is not the candidate: it is
// not asked for there — its definition halts on such a list — and is recorded VOID with
// that reason, never run on the tip and recorded as the candidate's. So a check that did
// not run in its round's `test` stage is not run again inside that round; the clause is
// the human's after the re-run. ACCEPTED AS BUILT, AS A DECLARED BOUND FOR THE FIRST RUN
// (the orchestrator's ruling of 2026-10-07): the tip differs from the candidate only by
// record commits, but proving that is the path rule's, which is the `fix` half's repair;
// until then a void check is settled by the next round, whose candidate is the tip, and
// such a round ends at `rule`, not `close`. A HELD CHECK THAT NAMES ITS COMMITS IS WHERE
// THAT ENDS: the regression set is started with the two commits it compares and is held
// again in a re-run, on the round's candidate. A HELD GATE IS NOT: it reads the working
// tree, so it is not held again inside its round — it is recorded VOID BY A WORD
// (`off-tree`), the one word the record script takes of a held check: a run that counts,
// so `next` leaves `retest` and the clause is the human's after it, as after a check's.
//
// THE LABELS. The contracts of the agent definitions bind on lines of the prompt — LABELS,
// below. Each is spelled here exactly as the definitions spell it, and
// tooling-tests/stabilize_harness_fence.rs holds the two together.
//
// THE RECORD STEP (the repair's change C4; DECISIONS.md -> 2026-10-06, "A stabilization
// run's record commit under a red candidate"). A stage's tables are written in ONE place and
// in three acts. (1) A `build-executor` writes the record's calls to a file with its file
// tool and applies them as ONE batch (`dev/stabilize-record apply --from FILE --sha256
// HASH`): the writer holds the file to the hash this script composed, every table is
// written or none is, and the record script holds the batch as pending. (2) THE FULL GATE
// on the tree with the records is a command the step tool holds — started by one step,
// asked after by another, under a name of the record's own — and the tool keeps its whole
// output. (3) A git step makes the one commit
// (`dev/stabilize-step record`): it holds that gate to the candidate's own — which the
// tool held too, and whose red steps and tests the batch itself puts on record — and commits
// when nothing is red that the candidate's gate did not show red. On a green candidate that
// is the rule that a commit follows a green gate. Red that is new is a halt that names it,
// with the batch still applied: THE NEXT INVOCATION OF EITHER STAGE FINDS THE PENDING BATCH,
// holds the gate on it again — no executor runs: nothing is applied — and commits it, and
// does nothing else (`finishRecord`). What is
// accepted as recorded is the commit step's own line, held to its hash: the commit, the gate
// check that holds, and as many calls and checks as this script composed.
//
// RESUMING. A stage that returned — halted, stopped at a bound, or finished — is not
// resumed: the next invocation is a fresh one and reads the state as it stands. What a
// halted stage wrote through the record script and no record step committed — its reports,
// the round's scope — is pending, not a dirty tree, and stays where it is; the next
// attempt's reports carry the next attempt number, which the state document gives. `resumeFromRunId` is for a KILLED run
// only, and then with the SAME args: every agent call's cache key is its prompt.
//
// THE RUNTIME PROBES (the second repair plan's task K0 and its §5; DECISIONS.md -> 2026-10-07,
// "The runtime probes of the stabilization harness"). An invocation with `probe` works on NO
// RUN: it observes ONE fact about the runtime this script runs in — with the roles, the
// prompts and the schemas a stage uses — and has `dev/stabilize-probe` judge what it saw.
//   required  two reviewers under a schema that REQUIRES the hash: (a) is told to leave the
//             field out, (b) is asked for a source pass that drives nothing. What comes back —
//             the object without it, the field, nothing, a throw — and after how many tries.
//   relay     the ONE line of a throwaway run's state, at four sizes, each relayed three
//             times by a git step under the stage's own step prompt — AS ITS DIGEST, the
//             line the step tool prints when asked for one (`--digest`), which is what a
//             stage is to read (the second repair plan's task K9).
//   relay-document  the same, of the line WITH THE STATE DOCUMENT IN IT — what a stage
//             reads today, and what this probe's first run never got back whole (2026-10-07:
//             twelve of twelve). Kept to compare.
//   payload   a record's batch of 20, 60, 120 and 300 entries, composed by the functions a
//             stage composes one with and written by the executor from its prompt.
//   hold      one long command: (a) HELD AS A STAGE HOLDS ITS GATE — started through the
//             step tool by one step, and asked after by ONE agent, again and again inside
//             its turn: the same two prompts, so that this case under the runtime is the
//             test that a held command asks the human for nothing; (b) started by one git
//             step through the probe tool and asked for by later ones, a slice each.
// A probe names no run, no round and no branch; its steps are commands of that tool — and,
// for the hold probe's case (a), the two of the step tool that hold a command; and it
// reaches no code of a stage: the script returns from `runProbe`. What it
// returns is `status: 'probed'` with the tool's judged `cases` — one per case, each with its
// `verdict` — the file they were written to under the scratch root, and what this script
// itself observed of each call (`observed`); or `status: 'halted'` with the step that failed.
// A PROBE'S AGENTS ARE MEANT TO FAIL: how each try of a watched call ended is the answer, so
// the retries are counted, and the breaker two exhausted calls trip is put back.
//
// Usage — BY THE SCRIPT'S PATH, NEVER BY ITS NAME: a session loads a workflow it invokes by
// name once and runs that copy for as long as it lives, after the file is edited too
// (observed 2026-10-07; implementation/stabilization-workflow.md -> Invoking the harness):
//         Workflow({ scriptPath: '<the repository's root>/.claude/workflows/stabilize.js', args: { stage: 'test', run: '<run>', scratch: '<dir>' } })
//         Workflow({ scriptPath: '<the repository's root>/.claude/workflows/stabilize.js', args: { probe: 'relay', scratch: '<dir>' } })
//   stage     — REQUIRED: 'test' or 'fix'.
//   run       — REQUIRED: the run's slug. Its directory is completions/artifacts/<run>/ and
//               its branches are named from it (`branchName`).
//   scratch   — REQUIRED: an absolute directory, outside the repository, that every agent of
//               the invocation works under and passes to the record script as --scratch.
//   scope     — OPTIONAL, `test` only: the scope the round is started with — 'delta',
//               'everything', { range: '<sha>..<sha>' } or { doors: [ … ] }. Absent: the
//               run's default scope, a fact of its record. A round's scope is written once.
//   clause    — `test` only, and REQUIRED EXACTLY WHEN THE STATE ASKS FOR A RE-RUN (`next` is
//               `retest`): the first clause its `retest` names. The invocation runs the
//               items of that clause the state lists as due — its position's `rerun` — and
//               nothing else, as ONE MORE ATTEMPT INSIDE THE ROUND THAT SELECTED THEM, over
//               that round's doors AND ON THAT ROUND'S CANDIDATE, never the branch's tip
//               (above: A ROUND TESTS ONE CANDIDATE): no round is begun, no scope resolved, and the record is
//               one result per item (the human's ruling of 2026-10-06: that instrument is
//               run again once; the record script counts the attempts, and takes a re-run
//               only where its state asks for one). A `clause` the state does not ask for,
//               and a `retest` answered without it, are refused before any agent runs.
//               Never together with `scope` or `crossModel`.
//   crossModel — OPTIONAL, `test` only: [ '<item>', … ] — the items of the test set whose
//               source pass is ALSO read by a model of another family, named one by one.
//               There is no value that means every item: heavy use is a deliberate act,
//               item by item. Absent, no stage names, calls or needs that tool, and a
//               machine that lacks it runs the whole workflow. A named item whose pass
//               could not run is returned and recorded as void for that pass; the source
//               pass a definition of this repository runs is never failed by it. (The
//               human's rulings of 2026-10-06; a cross-model second opinion is a human-
//               approved suggestion, never auto-run — implementation/milestone-planning-
//               workflow.md -> Review — and naming the item is the approval.)
//   rulings   — OPTIONAL: the human's rulings, recorded by ONE step.
//               On a finding or a bound:
//               [{ key, ruling: 'admitted' | 'later', note? } | { key, ruling: 'bound',
//               bound, reach, where, pin } | { bound, reach, where, pin }] — `where` is where
//               the human ruled it, `pin` the test that pins the bound or the word `unpinned`.
//               On `fix` they are recorded before anything is fixed, and the stage goes on.
//               On `test` the invocation records them on the loop branch and starts nothing
//               — the road they take while the `fix` stage refuses to start.
//               About the run, either stage — the invocation records them and starts nothing:
//               [{ go: true } | { rerun: '<clause>' } | { rounds: N } | { cycles: N }
//               | { reverify: '<key>' } | { again: 'test' | 'fix' }
//               | { order: ['<kind> <origin>', …] } | { queued: ['<key>', …] } | { budget: N }]
//               — the go after the
//               stop the state names (`stop.why: 'every-round'`), which answers that stop and
//               no later one; one more re-run of a clause the state lists in `human_clauses`;
//               the bound across rounds, raised (the go after `stop.why: 'round-bound'`), and
//               the bound on a round's fix cycles, raised (after `stop.why: 'cycle-bound'`);
//               one more triage of a finding the state's `human_list` names as still ungraded
//               or unverified after its retry; one more attempt of a stage the state lists in
//               `human_stages`; and, of a run with a queue, another order of its classes,
//               the rows put into its queue by key, and its budget, raised — recorded in
//               that order by one record (above: WHAT THE HUMAN RULES ABOUT THE RUN). The
//               record script takes each only while its state asks for it. `hand-on` is no
//               ruling: it is written once, at the opening. The two kinds are two invocations.
//   raise     — OPTIONAL, `fix` only: { cycles: N }, the human's raise of the cycle bound.
//   exit      — OPTIONAL, `fix` only: 'drop' or { part: [ … ] }, the human's exit at a bound.
//   stopAfter — OPTIONAL: return after a named step (STOPS), for tuning. NO RECORD STEP RUNS
//               and nothing is committed — but what the stopped stage wrote through the
//               record script is on disk and stays: after `state`, nothing at all — THE
//               INVOCATION ONLY LOOKS: its first read finishes nothing, no batch is gated or
//               committed and no push is made, and what the next invocation would finish
//               is returned as `owed`; after a
//               later step of `test`, the attempt's marker, every report written so far
//               and the round's scope, which is written once and stands. Such an attempt
//               counts as one that reached no record, and after two in a row the next is
//               the human's to grant. On `fix`: after `rulings` the rulings ARE recorded,
//               committed and pushed; after `fixers` the fixers' commits are on the round
//               branch; after `audit` the round branch is pushed as well.
//   selfTest  — OPTIONAL: true runs zero agents and checks this script's own logic.
//   probe     — INSTEAD OF A STAGE: 'required', 'relay', 'relay-document', 'payload' or 'hold' — one runtime
//               probe (above). It takes `scratch` — a directory that is there, outside the
//               repository, and fresh: a probe's root under it is made once — and nothing
//               else: no stage, no run.
//   seconds   — OPTIONAL, the `hold` probe only: how long its command holds. Absent, twelve
//               minutes; 2100 is the regression set's thirty-five.
//   model     — OPTIONAL: the model every agent call but the git steps is pinned to ('opus').

export const meta = {
  name: 'stabilize',
  description: 'One stage of a stabilization run. test: preflight, scope, the round\'s instruments, triage, verify-real, the record. fix: the human\'s rulings, fixers, the audit of the fix diff (at most 3 cycles), land — or the exits at a bound: continue, drop, land a part. Reads the round, the cycle and the next step from dev/stabilize-record state. Args: { stage, run, scratch, scope?, clause?, crossModel?, rulings?, raise?, exit?, stopAfter?, selfTest?, model? }. Or one runtime probe, on no run: { probe, scratch, seconds? }.',
  phases: [
    { title: 'Probe' },
    { title: 'State' },
    { title: 'Preflight and scope' },
    { title: 'Instruments' },
    { title: 'Fix' },
    { title: 'Triage' },
    { title: 'Verify' },
    { title: 'Record' },
    { title: 'Land' },
  ],
}

// ---- constants ----
const STAGES = ['test', 'fix']
const ARGS = ['stage', 'run', 'scratch', 'scope', 'clause', 'crossModel', 'rulings', 'raise', 'exit', 'stopAfter', 'selfTest', 'model', 'probe', 'seconds']
// The steps a stage can be stopped after, for tuning.
const STOPS = { test: ['state', 'preflight', 'instruments', 'triage'], fix: ['state', 'rulings', 'fixers', 'audit'] }
const SLUG_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/
const SLUG_MAX = 100
const SCRATCH_RE = /^(\/[A-Za-z0-9._-]+)+$/
const SHA_RE = /^[0-9a-f]{40}$/
const SHORT_SHA_RE = /^[0-9a-f]{7,40}$/
const SHA256_RE = /^[0-9a-f]{64}$/
const RANGE_RE = /^[0-9a-f]{7,40}\.\.[0-9a-f]{7,40}$/
const MODEL_RE = /^[a-z][a-z0-9.-]*$/
// Ruling 6: at most three fix -> audit cycles in a round. Only the human raises it, per run.
const DEFAULT_CYCLES = 3
const MAX_CYCLES = 99
const MAX_ROUNDS = 99
// The values of the state document's `next` this script knows what to say about. Any other
// value — the script's own `unsettled` among them — goes back to the orchestrator with the
// state attached. The set grows in dev/stabilize-record, never here first. `verify` is the
// step of a run with a queue (that script's second table): the `test` stage again, which an
// invocation CONTINUES — and what it continues the stage for goes back beside it (`queueSaid`).
const KNOWN_NEXT = ['fix', 'rule', 'close', 'verify']
// What an agent leaves open is triaged like any finding (ruling 5), and what triage sends on
// is verified. A verifier leaves things open too, so the two alternate — this many times with
// verification, then once more to give every entry its row.
const VERIFY_PASSES = 3
// THE MOST VERIFIERS ONE WAVE LAUNCHES, IN A RUN WITH A QUEUE (the reshape's plan, T6a): as
// many keys as the state's digest prints of the queue (dev/stabilize-step: THE STATE'S DIGEST,
// `queue.first` — "at most eight, as many as one wave of verifiers launches"; the fence holds
// the two numbers together). A wave is never larger than what the budget has left, either.
const WAVE = 8
// HOW MANY VERIFIER CALLS OF ONE WAVE MAY RETURN NOTHING BEFORE THAT WAVE IS THE LAST, IN A
// RUN WITH A QUEUE (the reshape's plan, T6b; its §2.2 and E7). The account's usage limit never
// reaches this script as a message: observed once, 2026-10-10, it was handed NOTHING, 297
// times. One call that returns nothing is one finding left unverified, and no stop — a stop
// costs a record and a restart; at a real limit every call of the wave is silent.
const SILENT_LAST = 2
// The verdicts a filing says (dev/stabilize-record: verdict), one of which the call that
// files it prints as `word`. The fence holds the list to that script's.
const FILED_WORDS = ['confirmed', 'refuted', 'undrivable']
const GIT_MODEL = 'sonnet'
// The one tool a git step runs a command of, and relays the line of.
const STEP_TOOL = 'dev/stabilize-step'
// THE TOOLS AN AGENT TYPES A COMMAND OF — and there are three (the second repair plan's task
// K11; the human's ruling of 2026-10-07: no step of a stage may need his permission): the
// step tool above, the record script, and — in a probe — the probe tool. The gate, the
// regression set, cargo, git and a hash are reached THROUGH the step tool's acts, and never
// typed by an agent: the full gate of a run — `--keep-going`, so that what it names red is
// all that is red — is the command of that tool's held kind `gate`.
const RECORD_TOOL = 'dev/stabilize-record'
// typed — THE ONE FUNCTION THAT RENDERS A COMMAND AN AGENT RUNS: one plain invocation of a
// tool, as a code span — the tool, and words a shell reads as words and as nothing else. No
// pipe, no redirect, no second command, no substitution: tooling-tests/
// stabilize_harness_fence.rs, arm (s), composes every prompt this script can compose and
// holds each command in it to that shape, and the committed settings allow exactly the
// three tools by a command's first word.
function typed(tool, words) {
  return '`' + tool + (words ? ' ' + words : '') + '`'
}
// Where the regression set's list lies: a path the CANDIDATE'S COMMIT holds — that tool
// reads the list out of the commit it tests (dev/regression-set) — and what a held
// `regression` is started with. The fence holds it to the file the commit holds.
const REGRESSION_LIST = 'completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv'
// baselineOf — WHERE THE PREVIOUS RELEASE'S BASELINE IS KEPT, below the scratch root (the
// header: AND WHAT THE PREVIOUS RELEASE'S COMMIT ALONE DECIDES IS BUILT ONCE): named by that
// release's commit as its build and its binary are (`build-previous-<12>`, `bin/previous-<12>`)
// and by nothing else, beside `hold/` and in no job's directory. The twelve digits are a
// name and no identity: the regression tool holds what is kept there to the whole commit.
function baselineOf(previous) {
  return 'baseline/previous-' + previous.slice(0, 12)
}
// regressionFlags — what a held `regression` is started with (dev/stabilize-step: HOLDS):
// the two commits it compares, the list, and the previous release's kept baseline.
function regressionFlags(previous, candidate) {
  return '--previous ' + previous + ' --candidate ' + candidate + ' --list ' + REGRESSION_LIST + ' --baseline ' + baselineOf(previous)
}
// A HELD COMMAND IS ASKED AFTER BY ONE AGENT AT A TIME, which asks again for as long as its
// turn lasts (observed 2026-10-07: one agent held thirty-five minutes inside its turn, and a
// read per agent cost 26 agents). Where a turn ends on `running`, the next agent takes the
// wait over — at most this many in turn; then the stage halts and names the job.
const HOLD_LAPS = 3
// A NAME IS ONE COMMAND'S, AND IS NEVER STARTED UNDER AGAIN (dev/stabilize-step: A HELD
// COMMAND). A record's gate that an earlier invocation started under the name this one
// would mint is that invocation's — a verdict of a tree as it stood then — so the next
// name is asked: `<name>-1`, `<name>-2`, … up to this many.
const HOLD_NAMES = 9
// HOW OFTEN THE COMPOSING STEP IS ASKED FOR AGAIN where the record script refused its words
// as not the ones that were hashed (`words`): a word altered on its way through the step's
// agent is that agent's slip, and the step typed as it stands composes the record. Refused
// so once more, the stage halts at its record: two agents in a row did not type forty words.
const WORDS_AGAIN = 1
const RUNS_ROOT = 'completions/artifacts'
// Adjustment (ii): the paths the published crates carry, and their tests. They move only
// through an audited round's merge; every other path commits directly behind the gate. The
// acts that assert it are handed the list.
const PRODUCT_PATHS = ['Cargo.toml', 'Cargo.lock', 'crates']
// The append-only logs a merge may conflict in — milestone-build.js's list, held equal by
// tooling-tests/merge_logs_fence.rs. `dev/merge-logs` resolves exactly these; the two acts
// that merge are handed the list.
const SYNC_LOGS = ['DECISIONS.md', 'implementation/project-history.md']
// The lines the agent definitions' contracts bind on, spelled as they spell them.
const LABELS = { report: 'REPORT:', binary: 'BINARY:', area: 'AREA:', record: 'RECORD STEP', branch: 'BRANCH:' }
// Ruling 4's three: the dispositions that are the human's to give.
const HUMAN_RULINGS = ['admitted', 'bound', 'later']
const PAYLOAD_ENDS = 'STABILIZE_PAYLOAD'
// THE RUNTIME PROBES. The tool a probe's steps are commands of, and which judges what a
// probe observed; the probes; and their cases — the ledger rows of the relay probe's four
// throwaway runs and how often each one's line is relayed, and the entries of the payload
// probe's four batches. dev/stabilize-probe has the same lists, and
// tooling-tests/stabilize_harness_fence.rs holds the two together.
const PROBE_TOOL = 'dev/stabilize-probe'
const PROBES = ['required', 'relay', 'relay-document', 'payload', 'hold']
const PROBE_ROWS = [1, 20, 60, 120]
const PROBE_RELAYS = 3
const PROBE_ENTRIES = [20, 60, 120, 300]
// The hold probe's command holds twelve minutes — past the ten a call of an agent's shell
// tool may take — unless `args.seconds` says otherwise; and a read of a detached hold waits
// a slice that is under that tool's DEFAULT timeout of two minutes, so that a git step needs
// to be told nothing about timeouts.
const PROBE_HOLD = 720
const PROBE_HOLD_MAX = 3600
const PROBE_SLICE = 90
// The name under which a triage is handed the rows of the ledger that still await it. No
// reporter can carry it: an item's reporters are `<item>-<step>`.
const LEDGER_SOURCE = 'ledger'
// The key a red result's row is filed under, before the item's id (dev/stabilize-record
// result-set: A RED RUN IS A FINDING, `red-<item>`) — and so the name the one triage of a run
// with a queue is handed that result under, as a source. The fence holds it to that script's.
const RED_PREFIX = 'red-'
// The reporter an attempt's marker is written under (`dev/stabilize-step begin`). No agent
// can carry it: an item's reporters are `<item>-<step>`, and no role is named so.
const ATTEMPT = 'attempt'
// STEP_REFUSALS — every word `dev/stabilize-step` refuses an act with (its STATUS), as what
// this script SAYS of it: whose the state is, and what leaves it. A step's digest carries
// the word and never the prose — the tool's own halt report is in the file the digest names
// — so the sentence is this script's, and tooling-tests/stabilize_harness_fence.rs holds the
// table to the tool's words in both directions: a word the tool gains has no row until it is
// given one here, and no row outlives its word. What the NEXT INVOCATION does after a
// refusal of a record's commit or of its push is `recordThen` and `pushThen`, below.
const STEP_REFUSALS = {
  'usage': { whose: 'the harness\'s', leaves: 'the command this script composed is not one the tool takes, or names a scratch root the tool cannot use: nothing ran. It is a defect of the harness or of the invocation\'s `scratch`, and the step\'s own line says which argument' },
  'wrong-branch': { whose: 'the orchestrator\'s', leaves: 'the branch the step names is not the one checked out: nothing was read past its name, and nothing was written, fetched or pushed. The branch the step names is checked out, and the stage invoked again' },
  'dirty': { whose: 'the human\'s', leaves: 'the tree holds what is no pending write of the run — a tracked file changed by hand, a staged path that is no open record\'s, a stray file — or a file of an applied batch that a hand changed: the human reconciles it, and no step is repeated on a guess. For a changed batch `dev/stabilize-step discard` takes the batch back; the step\'s own line names the file' },
  'remote-ahead': { whose: 'the human\'s', leaves: 'the pushed branch has commits the local one lacks: the human reconciles the two — no step pulls, merges or rebases' },
  'product-path': { whose: 'the human\'s', leaves: 'a commit changed a product path directly on the loop branch: a product path moves only through an audited round\'s merge' },
  'foreign-merge': { whose: 'the human\'s', leaves: 'a merge on the loop branch is not a round\'s: only a round\'s branch is merged into it' },
  'merge-in-round': { whose: 'the human\'s', leaves: 'a round\'s branch holds a merge: its commits are a fixer\'s and a record\'s, one by one' },
  'conflict': { whose: 'the human\'s', leaves: 'a merge stopped on a path that is no append-only log: it was aborted, and the conflict is the human\'s' },
  'merge-logs': { whose: 'the human\'s', leaves: '`dev/merge-logs` could not resolve the logs a merge stopped on: the merge was aborted' },
  'unresolved': { whose: 'the human\'s', leaves: 'a merge is left in progress with a path still conflicted: it was not aborted, and the human finishes or aborts it' },
  'merge-differs': { whose: 'the human\'s', leaves: 'the merge commit is not the merge that was asked for — its parents, its tree or what it moved: nothing was undone' },
  'pick-stopped': { whose: 'the human\'s', leaves: 'a cherry-pick stopped, and was aborted' },
  'carry-differs': { whose: 'the human\'s', leaves: 'what was carried over is not exactly the commits named: nothing was undone' },
  'push-rejected': { whose: 'the orchestrator\'s', leaves: 'the remote did not take the push — or none was made, because a tag or another ref answers to the branch\'s name: nothing is lost — the commit is local — and the push is owed' },
  'remote-differs': { whose: 'the orchestrator\'s', leaves: 'the remote, or the branch, did not stand where the push held it. One of them moved while the range was vetted, and nothing was pushed; or the commit that was vetted IS published — by its sha, and nothing later — and the remote stands elsewhere, or a commit that landed on the branch meanwhile is not on the remote. The push is owed, and the next first read looks again' },
  'record': { whose: 'the orchestrator\'s', leaves: '`dev/stabilize-record` refused, or its answer could not be read: the step\'s own line holds that script\'s refusal — the tree is as that script left it, which is whole or untouched' },
  'git': { whose: 'the orchestrator\'s', leaves: 'a git command of the step failed: the step\'s own line holds the command and what it printed, and nothing after it ran' },
  'gate-red': { whose: 'the orchestrator\'s', leaves: 'the gate that ran on the tree with the records shows red that the candidate\'s own gate did not: nothing was committed or undone, and the batch stays applied — the step\'s own line names what is new' },
  'no-batch': { whose: 'the orchestrator\'s', leaves: 'the run has no applied batch of the calls and checks this script composed — none was applied, or the one that is there is another\'s: nothing was committed' },
  'position': { whose: 'the orchestrator\'s', leaves: 'the run\'s state does not hand the stage this round and this attempt — another invocation holds it, or the state moved since it was read: nothing was written, and the stage invoked again reads the state as it stands' },
  'unvetted': { whose: 'the human\'s at a push; at a record\'s commit, the next attempt\'s', leaves: 'a file, a subject or a commit is not what `dev/stabilize-record` would have written. At a record\'s commit nothing is committed and the batch is taken back; at a push nothing is pushed, and the commit is local and the human\'s to take back or rewrite — `dev/stabilize-record vet --range -- <branch> --not --remotes=origin` names every commit and place again' },
  'did-not-run': { whose: 'the orchestrator\'s', leaves: 'a scanner could not run — gitleaks, or the denylist: that is a refusal and never a pass, and everything is as it was. The scanner is provided, and the stage invoked again' },
  'locked': { whose: 'the orchestrator\'s', leaves: 'git\'s own lock stands in the repository, and the tool never removes one. With no git process running there, the orchestrator removes the file the step\'s own line names and invokes the stage again; while one runs, once it is done' },
  'fence-moved': { whose: 'the human\'s', leaves: 'THE REPOSITORY WAS WRITTEN PAST THE WRITE GUARD while the stage ran — its configuration, a hook, an info file, a local ref or a reflog\'s newest entry is not what the stage\'s first read saw, and no step of a stage writes any of them before its record: nothing was committed or undone, and the batch stays applied. The human finds what moved — `git reflog -5`, `git config --local --list`, the hooks directory, `git for-each-ref` — puts back what must be put back, and invokes the stage again: its first read takes the guarded state as it then stands' },
  'head-moved': { whose: 'the orchestrator\'s', leaves: 'the branch is not at the commit the stage began on — somebody committed while it ran: nothing was committed or undone, and the batch stays applied. The stage is invoked again: its first read meets that commit, then gates the batch on the tree as it now stands and commits it there' },
  'foreign-commit': { whose: 'its author\'s', leaves: 'the loop branch is ahead of its remote by a commit that changes a path outside the run\'s directory, or by a merge: some task\'s commit and no record\'s. It is NEITHER THE HUMAN\'S NOR A STAGE\'S: its author — or the orchestrator, for that task — pushes it by name, `git push origin <branch>`, behind that task\'s own gate, and the stage is invoked again. Nothing was pushed, not the records beside it either' },
  'committed': { whose: 'the orchestrator\'s', leaves: 'the applied batch is in a commit already and is not taken back: the next first read settles it' },
  'taken': { whose: 'the harness\'s', leaves: 'a held command of that name was started with other arguments, or its work directory holds a build: a name is minted once per attempt' },
  'missing': { whose: 'the harness\'s', leaves: 'no held command of that name was started under this scratch root, or the file asked for is not there' },
  'build': { whose: 'the orchestrator\'s', leaves: 'the commit did not build, or its binary is not of the version asked: no binary was written' },
}
// stepRefusal — a step's refusal as this script says it: the word, whose it is, what leaves it.
function stepRefusal(word, branch) {
  const row = STEP_REFUSALS[word]
  return row ? '`' + word + '` — ' + row.whose + ': ' + row.leaves.replace('<branch>', branch || '<branch>') : '`' + word + '`, a word of `' + STEP_TOOL + '` this script has no row for'
}
// THE HELD CHECKS. A check that is one long command is an item whose kind says so:
// `held-<id>`, `<id>` a kind of the step tool's held commands (its HOLDS; dev/stabilize-record
// reads the same form). The tool starts it, waits for it and JUDGES it; what this script
// reads of it is `heldOf`. A `test` stage starts each one itself, after the candidate's gate
// and never beside it (`hold`), and a held gate is answered from the candidate's own.
const HELD_PREFIX = 'held-'
const HELD_CHECKS = ['gate', 'regression']
function heldKind(kind) {
  const id = typeof kind === 'string' && kind.startsWith(HELD_PREFIX) ? kind.slice(HELD_PREFIX.length) : null
  return id && HELD_CHECKS.includes(id) ? id : null
}
// HELD_ENDS — how a held command can stand when it is asked for, and what follows: the
// header's table (HOW A STEP CAN END), as the words `heldOf` answers with.
const HELD_ENDS = {
  running: 'every agent that asked after it in turn ended its turn while the command still ran',
  green: 'the command ran to its end, and the tool read its verdict green',
  red: 'the command ran to its end, and the tool read its verdict red',
  void: 'the command ran to its end, and its output holds no verdict the tool takes',
  dead: 'the command\'s process is gone and nothing says how it ended: it has no verdict, whatever its output holds, and is never started again under its name',
  unread: 'the step that asked did not answer with a job of the tool\'s, and nothing is known of how the command stands',
}
// DEAD_WHY — WHAT `dead` SAYS WHERE THE TOOL'S OWN WORD FOR IT SAYS SOMETHING ELSE THAN THE
// SENTENCE ABOVE (`heldEnded`). A NAME WHOSE START LEFT NO RECORD (`no-record`; dev/stabilize-
// step: A JOB'S DIRECTORY WITHOUT ITS RECORD IS NO JOB THAT IS READ) is dead by that tool's
// answer, and ITS COMMAND'S PROCESS MAY WELL LIVE: a start killed after its fork leaves a
// supervisor that runs the command to its end, unrecorded. Whether it lives is the answer's
// `supervised` — in the step's own line, in the file its digest names, and in no digest: this
// script acts on nothing of it, and says of such a name what the answer says and no more.
const DEAD_WHY = {
  'no-record': 'the name holds no record of the tool\'s — a start under it was killed before it wrote one, or no start made its directory: nothing says what was asked under it, so nothing of it is read and it has no verdict, and nothing is ever started under it again. A command MAY STILL RUN under it, unrecorded — the step\'s own line says whether its supervisor lives (`supervised`)',
}
// heldEnded — how a held command stands, as the sentence a halt and a log say of it.
function heldEnded(job) {
  return (job.ends === 'dead' && DEAD_WHY[job.why]) || HELD_ENDS[job.ends] || 'it ended `' + job.ends + '`'
}
// HELD_TAKES — WHAT A STAGE HOLDS, AND WHICH ENDS IT TAKES (the header: HOW A STEP CAN END, a
// held command). Any other end is a HALT that names the job — but for a held check, which
// is then VOID BY A WORD (`heldVoid`): the stage goes on and records it, a run that counts.
const HELD_TAKES = {
  build: ['green'],
  gate: ['green', 'red'],
  check: ['green', 'red', 'void'],
  record: ['green', 'red'],
  probe: ['green'],
}
// heldVoid — WHY A HELD CHECK LEFT NO VERDICT, as the ONE WORD the record script takes of a
// held check (dev/stabilize-record result-set: `{item, outcome: void, reason}`, the reason of
// a declared shape — `<word>` or `<word>:<key>`, a slug and the held job's name — which
// HELD_VOID_RE is; the fence holds the two patterns equal). The word is the one this script
// already reads: `off-tree` for a held gate that is not run where the tree is not the
// candidate; the tool's own word where it REFUSED the start; else how the job stood when
// the stage stopped asking — `dead`, `running`, `unread`. Every part is a slug by
// construction: no sentence, no path and no agent's prose reaches the record through it.
const HELD_VOID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*(?::[A-Za-z0-9._-]+)?$/
function heldVoid(job) {
  const word = job.ends === 'not-run' ? 'off-tree' : job.refused || job.ends
  return word + (job.name ? ':' + job.name : '')
}
// mayRun — whether a held command may STILL RUN when the stage stops asking after it.
function mayRun(job) {
  return !!job && !!job.name && (job.ends === 'running' || (job.ends === 'unread' && job.started === true))
}
// heldFault — a held command that did not end as the stage needs it, as the halt says it.
function heldFault(what, job) {
  // (No name was left to hold it under: every step that asked ANSWERED — with a job that was
  // there already — so it is not said to be a step that did not answer.)
  if (job.exhausted) return what + ' was not held: ' + job.why + '.'
  return what + (job.name ? ' (the held command `' + job.name + '`)' : '') + ' did not end as the stage needs it: ' + heldEnded(job) + (job.why ? ' — `' + job.why + '`' : '') + '.'
}
// heldThen — what a halt over a held command says is to be done, where the command MAY
// STILL RUN: it is nobody's to end, the one call that answers once it is over is named, and
// the stage is invoked again after that. Null where it does not run: the halt's own
// sentence stands, and the next attempt names and starts the next.
function heldThen(scratch, job) {
  if (!mayRun(job)) return null
  return ' THE COMMAND MAY STILL RUN, in a session of its own, and nothing ends it but its own end: ' + typed(STEP_TOOL, 'hold-wait --scratch ' + scratch + ' --name ' + job.name) + ' answers once it is over' + (job.output ? ', and its whole output is ' + job.output : '') + '. Invoke the stage again AFTER THAT, with the same args — two gates at once contend for one lock: the next attempt names and starts its own command, and takes nothing over from this one.'
}
// heldSaid — a held command as a return names it: its job, how it ended, and nothing else.
function heldSaid(job) {
  return { name: job.name || null, kind: job.kind || null, ends: job.ends, why: job.why || null, output: job.output || null }
}
// The paragraph every agent definition carries, verbatim (tooling-tests/
// release_pipeline_fence.rs holds it there) — for the two roles that have no definition and
// so no paragraph of their own. The fence holds this copy to the definitions' bytes.
const NEVER = "**Never push to or merge into `main`, force-push any branch, merge a pull request, push a tag, approve or reject a deployment, or yank a crate.** Those acts are the human's ([release.md](../../implementation/release.md) → *What agents may not do*). You may merge an increment branch into its milestone branch locally, and push `milestone/*`, `fix/*` and `work/*` branches — always by name (`git push origin <branch>`), never a bare `git push`, and never forced (no `--force`, `--force-with-lease`, `-f` or `+` refspec). `.claude/settings.json` denies the commands that perform the human's acts; a denial is the answer, never something to route around."

// THE WRITE GUARD, AS A PROMPT SAYS IT (DECISIONS.md -> 2026-10-16, "The write guard of a
// stabilization run"; dev/stabilize-guard). Every agent of a run but its git steps and its
// fixers is fenced by a hook: a file-tool write into the repository is refused, a call of
// the three tools is taken only as one plain invocation typed alone from the repository's
// root, and every other command runs under the write fence. The hook says so where it
// refuses; this sentence says it before, in every role's prompt (`opening`, and the two
// prompts without a definition), so that the call is typed in the shape that is taken the
// first time — the staged canary's verifiers typed a tool's call after a change of
// directory and before an echo of its status, every time.
const GUARD = 'THIS RUN\'S WRITE GUARD: no agent of it but its git steps and its fixers writes this repository or its git directory. A file you write goes under your scratch root, and what reaches the run\'s record reaches it through the record script\'s one plain call. A call of ' + STEP_TOOL + ', ' + RECORD_TOOL + ' or ' + PROBE_TOOL + ' is taken only as ONE plain invocation, typed by itself, from the repository\'s root, exactly as this prompt spells it — nothing before it and nothing after it on the line, its exit status its own; any other command of yours runs under the write fence, where every write under this repository is refused.'

// ---- the roles: who is launched, and which labelled lines its definition binds on ----
// `drives` is a role that is handed the binary and returns the hash it asserted — every
// role whose definition binds on the binary line (the fence holds the two sets equal), and
// the one prompt-only role that drives: a reviewer verifies its findings on the candidate's
// own binary too, never on a build of the working tree (the harness review's M7).
// `proposal` drives an advocate's proposal independently of the advocate. No definition
// fits that role (the entry of 2026-10-06 says what each lacks), so it has none, and its
// prompt is its whole contract. `crossModel` is the source pass by a model of another
// family: it has no definition either, and is launched only for an item args.crossModel names.
const ROLES = {
  preflight: { agentType: 'stabilize-preflight', labels: ['report'], drives: false },
  scope: { agentType: 'stabilize-scope', labels: ['report'], drives: false },
  review: { agentType: 'milestone-code-reviewer', labels: ['report', 'binary'], drives: true },
  drive: { agentType: 'milestone-e2e-tester', labels: ['report', 'binary'], drives: true },
  triage: { agentType: 'finding-triage', labels: ['report'], drives: false },
  verify: { agentType: 'finding-verifier', labels: ['report', 'binary'], drives: true },
  advocate: { agentType: 'robust-advocate', labels: ['report', 'binary'], drives: true },
  proposal: { agentType: 'general-purpose', labels: [], drives: true },
  crossModel: { agentType: 'general-purpose', labels: [], drives: false },
  fixer: { agentType: 'build-fixer', labels: ['report', 'area', 'branch'], drives: false },
  record: { agentType: 'build-executor', labels: ['record', 'branch'], drives: false },
}

// ---- the chains: per KIND of instrument item, the fixed chain of agents that runs it ----
// A chain is a list of steps run in order; a step is the roles run in parallel. `as` names
// the reporter (`<item>-<as>`), `hands` the earlier steps whose reports the role is handed.
// An item whose kind has no chain here halts the stage and names the kind: the test set is
// data (dev/stabilize-record item-set), and a kind is added by adding its row here.
// `check` has no chain of its own: a deterministic check is run by the preflight.
const CHECK_KIND = 'check'
// The doctype a red deterministic check is filed under: the candidate fails a check.
const RED_DOCTYPE = 'jigc-feedback'
// The id under which the preflight returns whether the cross-model pass's tool answers.
const CROSS_CHECK = 'cross-model-tool'
const CHAINS = {
  'review-row': [
    [{ as: 'source', role: 'review', task: 'the SOURCE PASS of this review row' }, { as: 'driver', role: 'drive', task: 'DRIVE this review row\'s table' }, { as: 'crossmodel', role: 'crossModel', crossModel: true }],
    [{ as: 'reconciler', role: 'drive', task: 'RECONCILE this review row: the table another agent drove, against the source pass — a claim of one that the other cannot reproduce is a lead, and you drive every lead', hands: ['source', 'driver', 'crossmodel'] }],
  ],
  'trial-arm': [
    [{ as: 'rehearse', role: 'drive', task: 'REHEARSE this trial arm: prove its occasion fires, before the arm is run', image: true }],
    [{ as: 'run', role: 'drive', task: 'RUN this trial arm', hands: ['rehearse'], image: true }],
    [{ as: 'score', role: 'drive', task: 'SCORE this trial arm from the evidence its run left', hands: ['rehearse', 'run'], image: true }],
  ],
  'audit-area': [[{ as: 'review', role: 'review', task: 'the audit of this AREA' }]],
  'audit-cross-cutting': [[{ as: 'review', role: 'review', task: 'the CROSS-CUTTING pass of the audit' }]],
  'audit-drive': [[{ as: 'drive', role: 'drive', task: 'the audit\'s DRIVE: the doors of this unit, through the binary' }]],
}
// chainOf — a kind's chain as ONE item runs it: a step marked `crossModel` is in it only
// when the invocation named that item for the cross-model pass (and the pass can run), and
// nobody is handed the report of a step that is not.
function chainOf(kind, crossModel) {
  const chain = (CHAINS[kind] || []).map((stepList) => stepList.filter((s) => !s.crossModel || crossModel === true))
  const present = chain.reduce((all, stepList) => all.concat(stepList.map((s) => s.as)), [])
  return chain.map((stepList) => stepList.map((s) => (s.hands ? Object.assign({}, s, { hands: s.hands.filter((h) => present.includes(h)) }) : s)))
}
// The audit of a round's fix diff — not an item of the test set, the `fix` stage's own.
const FIX_AUDIT = [[{ as: 'review', role: 'review', task: 'the review of this round\'s FIX DIFF' }, { as: 'drive', role: 'drive', task: 'the drive of this round\'s FIX DIFF: every door it can change' }]]

// ---- what a stage needs of a return, and what the absence of it does ----
// NEEDS — per ROLE THAT DRIVES, the fields a `test` stage needs of its return THAT ITS
// SCHEMA DOES NOT REQUIRE, and what follows where one is not there (`then`; the header: HOW
// AN AGENT CAN END, a return that lacks a field). NO SCHEMA OF A STAGE REQUIRES ONE: a
// return that omits a required field is a call that FAILED — three whole runs of the agent,
// and a count toward the breaker (the `required` probe, 2026-10-07) — so the script is
// handed the return without it, and the absence is THAT REPORTER'S FACT, never the stage's:
//   void        the item whose chain the step belongs to is VOID, with the reporter and the
//               field in its reason; the rest of its chain is not launched; what the step
//               found is triaged all the same — a finding is verified by another agent
//   unverified  the finding stays UNVERIFIED, as after a verifier that died
//   no-fork     half a fork is no fork: nothing of it is on record, the finding unverified
//   unknown     THE VERDICT STANDS AND ITS REGRESSION FACT IS UNKNOWN: recorded as confirmed
//               with `regression` ABSENT (`regressionOf`, `triageRecord`) — never `false`,
//               and never an unverified finding — and named by the state
//   halts       THE STAGE STILL HALTS — a cell owed to a task, by name, in
//               tooling-tests/stabilize_harness_fence.rs (OWED), and a row of HALTS_AFTER.
//               NO CELL HOLDS IT TODAY: the last two were turned to `unknown`
// `with` is the verdict the need holds under. The fence, arm (t), holds this table to the
// schemas — every optional field of a driving role's schema has a row here or a reason
// there — and every cell that is not `halts` to an arm of the simulation.
const ASSERTED = 'the hash it asserted for the binary it drove: without it nothing it drove is evidence about this candidate'
const NEEDS = {
  review: [{ field: 'asserted_sha256', then: 'void', why: ASSERTED }],
  drive: [{ field: 'asserted_sha256', then: 'void', why: ASSERTED }],
  verify: [
    { field: 'verdict', then: 'unverified', why: 'what it found: a verifier that returns none verified nothing' },
    { field: 'asserted_sha256', then: 'unverified', why: ASSERTED },
    { field: 'regression', with: 'confirmed', then: 'unknown', why: 'whether the finding it confirmed is a regression: the verifier\'s fact, and nobody else\'s' },
    { field: 'ran_on.previous', with: 'confirmed', then: 'unknown', why: 'the hash it asserted for the previous release\'s binary: what a regression fact is backed by' },
  ],
  advocate: [{ field: 'asserted_sha256', then: 'no-fork', why: ASSERTED }],
  proposal: [{ field: 'asserted_sha256', then: 'no-fork', why: ASSERTED }],
}
// HALTS_AFTER — WHAT STILL HALTS A `test` STAGE ONCE AN INSTRUMENT IS LAUNCHED: the whole
// list, each row with the phase its halt names. From the stage's first held check on — and,
// in the lap that finishes a triage, from its triage on — a halt goes through `haltAfter`
// and names its row, and the fence, arm (t), is red on one that does not. EVERY OTHER
// ENDING OF EVERY AGENT VOIDS WHAT THAT AGENT WAS LAUNCHED FOR, AND THE STAGE REACHES ITS
// RECORD. What a halt here costs is the stage: its instruments ran, nothing is recorded,
// the attempt is counted. `record`, `push` and `read-back` cost less — the batch is applied,
// or the record is committed — and each says what the next invocation finishes.
const HALTS_AFTER = {
  'names': { phase: 'state', when: 'a reporter name cannot be made for a step of an item the round runs: the item\'s id is too long to carry its step inside a slug' },
  'report-check': { phase: 'reports', when: 'the report check after the instruments could not be read — its step died, refused, or the breaker is tripped — or it names a file no launched reporter wrote. IN A RUN WITH A QUEUE, the same of the report check after a wave of verifiers: the first record is committed, and what was filed is on disk — and of the report check that comes before a record no wave came before, in an invocation that continues a stage only for its record: nothing was composed' },
  'scope-aside': { phase: 'scope', when: 'the round\'s scope was set aside by the report check: the file at its path is not what the record script would have written' },
  'stood-on': { phase: 'reports', when: 'a reporter the stage stands on left no report: the attempt\'s marker, the first preflight, the scope step, a second preflight that provided, the preflight of a finishing lap' },
  'triage': { phase: 'triage', when: 'ANY ending of triage but its grades with its report — no result, a halt, counts that do not balance, a key that is none or is given twice, no report — and, inside a pass, a verifier\'s name that cannot be made and a report check that could not be read. IN A RUN WITH A QUEUE, any ending of the one triage but its entries handed over, with their rendered report: a halt, a return that names another path than that report\'s or no hash, a call typed with other counts than the stage handed — and the same of the triage an invocation that continues such a stage launches over the rows nobody graded' },
  'record': { phase: 'record', when: 'the record step: the executor applied nothing or its batch was refused, the record\'s gate left no verdict, or the commit step refused — the batch stays applied where it was, and the next invocation finishes it. IN A RUN WITH A QUEUE, where the tool composes the batch: the composing step returned nothing, or was refused — by a word of the record script (COMPOSE_REFUSALS; `words` only once it was asked for again), with nothing written, or by a word of the tool. Of such a run\'s SECOND record the same, but for `no-change`, which is no halt: nothing was filed, and there is nothing to record. Of the records of an invocation that continues the same: of what it graded as of a first, of what its verifiers filed as of a second' },
  'push': { phase: 'push', when: 'the push of the record\'s commit failed or was refused: the record is committed, the push is owed, and the next invocation\'s first read makes it' },
  'read-back': { phase: 'state', when: 'the state could not be read back once the record was committed and pushed: nothing is lost, and the next invocation reads it. IN A RUN WITH A QUEUE, also the read of the queue after a wave of verifiers: what was filed is on disk, and the state the next invocation reads counts it' },
}

// VOID_WORDS — WHY AN ITEM DID NOT RUN TO ITS END, AS ONE WORD: what the record of a run with
// a queue says of a void item, in the shape the record script takes of every void there
// (`<word>` or `<word>:<key>`, HELD_VOID_RE — dev/stabilize-record compose: `item=<item>:
// void:<word>[:<key>]`, "never a sentence"). The key is THE REPORTER the word is about — one
// that left no report among them, which is then no `reporter=` word of the record — or, for
// a held check, the job (`heldVoid`, whose words are the tool's and HELD_ENDS's and are not
// repeated here). The sentence is this table's, for whoever reads the word; the path of a
// run without an order records its own sentences, as it did.
const VOID_WORDS = {
  'silent': 'a step of the item\'s chain returned nothing, after its retries',
  'halted': 'a step of the item\'s chain halted',
  'unasserted': 'a step of the item\'s chain drove, and did not return the hash of the candidate\'s binary as the one it asserted',
  'unreported': 'a step of the item\'s chain returned, and left no report',
  'set-aside': 'the file at the report\'s path of a step of the item\'s chain was no report, and was set aside',
  'chain-lost': 'the item\'s chain did not run: nothing came back of it',
  'no-image': 'a trial arm, and the second preflight provided no verified trial image',
  'off-tree': 'a check that reads the working tree, in a re-run: the tree is no longer the round\'s candidate, and the check was not run',
  'other-commit': 'the check ran on another commit than the candidate, or on one the preflight did not name',
  'could-not-run': 'the preflight could not run the check, and says so',
  'not-provided': 'a check the scope selected, and the second preflight did not provide it',
  'unanswered': 'the preflight returned nothing for the check',
}
// voidWord — a void's reason as the record of a run with a queue takes it: the word, and the
// key it is about.
function voidWord(word, key) {
  return word + (key ? ':' + key : '')
}
// COMPOSE_REFUSALS — every word the record script refuses a composed record with that the
// step tool has a sentence for (its COMPOSE_THEN; the fence holds the two tables together),
// as what this script DOES about it and what the halt says is done next. `again`: the step
// is asked for again, exactly (WORDS_AGAIN). Every other word halts the stage at its record,
// and NOTHING WAS WRITTEN by a step refused so.
const COMPOSE_REFUSALS = {
  'words': { again: true, says: 'the words the step typed are not the words this script hashed: one was altered, dropped, added or moved on its way through the step\'s agent — also when the step was asked for again', then: null },
  'check': { again: false, says: 'a check of the batch does not hold — a launched reporter\'s report is missing, a file nobody launched lies among the reports, a key is no row — or a file among the filed verdicts is no verdict. The stage\'s report check came before this step, so something changed between the two', then: null },
  'pending': { again: false, says: 'another batch is applied and not committed — one this stage did not compose: it appeared while the stage ran', then: 'pending' },
  'no-change': { again: false, says: 'every file the record would write is what the run holds already — and an attempt\'s first record always writes that the attempt reached it', then: ' Nothing was written and NO BATCH IS LEFT. No discard is asked for.' },
  'hygiene': { again: false, says: 'the public-hygiene scan stopped a file the record would write — gitleaks found a secret-shaped string in it, or the denylist matched: a credential is never committed, even a revoked one, and the scan is never widened to let one through', then: ' What was committed stands, and the reports the stage filed are still untracked under the run\'s directory. What wrote the string is changed — a report its reporter rewrites, a field a tool names — and the stage is invoked again; the step\'s own line holds the record script\'s whole refusal.' },
}
// scanHits — what the public-hygiene scan stopped, as the composing step's digest names it
// (dev/stabilize-step compose: `hits`): each file by its path in the repository and the
// rule — gitleaks' by its id, the denylist by that word — and never what matched. '' where
// the step named none: the step's own line holds the refusal whole.
function scanHits(hits) {
  const named = Array.isArray(hits) ? hits.filter((hit) => plain(hit) && isText(hit.rule)) : []
  return named.length ? '. It stopped ' + named.map((hit) => (hit.rule === 'denylist' ? 'a match of the denylist' : 'gitleaks\' rule `' + hit.rule + '`') + (Number.isInteger(hit.line) ? ' at line ' + hit.line : '') + ' of ' + (isText(hit.path) ? '`' + hit.path + '`' : 'a file the step\'s line names')).join('; ') : ''
}

// ---- pure helpers (everything from here to `selfTest` runs no agent) ----

// branchName — THE one place a branch name of a stabilization run is minted (ruling 7).
// `round` absent: the loop branch. With a round: its branch; `cut` above 1 is a part of the
// round re-cut as a tip of its own (ruling 6). `round` as '' gives the prefix every branch
// of every round shares, for the git steps that list them. The branch type of its own
// (`stabilize/<run>/main`, `stabilize/<run>/r<N>`) is this function's change and no other's.
function branchName(run, round, cut) {
  const loop = 'fix/' + run
  if (round == null) return loop
  return loop + '-r' + round + (cut > 1 ? '-part' + (cut - 1) : '')
}

function runDir(run) {
  return RUNS_ROOT + '/' + run
}

// sha256 — of a text's UTF-8 bytes, as `shasum -a 256` prints it. What an agent relays —
// the state document — and what an agent is told to write — a record step's payload — is
// held to a hash, so that a line retyped with one character changed is caught here and not
// three steps later.
function sha256(text) {
  const bytes = []
  for (let i = 0; i < text.length; i++) {
    let c = text.charCodeAt(i)
    if (c >= 0xd800 && c < 0xdc00 && i + 1 < text.length) c = 0x10000 + ((c - 0xd800) << 10) + (text.charCodeAt(++i) - 0xdc00)
    if (c < 0x80) bytes.push(c)
    else if (c < 0x800) bytes.push(0xc0 | (c >> 6), 0x80 | (c & 63))
    else if (c < 0x10000) bytes.push(0xe0 | (c >> 12), 0x80 | ((c >> 6) & 63), 0x80 | (c & 63))
    else bytes.push(0xf0 | (c >> 18), 0x80 | ((c >> 12) & 63), 0x80 | ((c >> 6) & 63), 0x80 | (c & 63))
  }
  const bits = bytes.length * 8
  bytes.push(0x80)
  while (bytes.length % 64 !== 56) bytes.push(0)
  for (let shift = 56; shift >= 0; shift -= 8) bytes.push(shift >= 32 ? Math.floor(bits / 4294967296) >>> (shift - 32) & 255 : bits >>> shift & 255)
  // The first 32 bits of the fractional parts of the square and cube roots of the primes.
  const h = []
  const k = []
  for (let n = 2; k.length < 64; n++) {
    let prime = true
    for (let d = 2; d * d <= n; d++) if (n % d === 0) prime = false
    if (!prime) continue
    if (h.length < 8) h.push(Math.floor((Math.sqrt(n) % 1) * 4294967296))
    k.push(Math.floor((Math.cbrt(n) % 1) * 4294967296))
  }
  const rotr = (x, n) => (x >>> n) | (x << (32 - n))
  for (let at = 0; at < bytes.length; at += 64) {
    const w = []
    for (let i = 0; i < 16; i++) w.push((bytes[at + 4 * i] << 24) | (bytes[at + 4 * i + 1] << 16) | (bytes[at + 4 * i + 2] << 8) | bytes[at + 4 * i + 3])
    for (let i = 16; i < 64; i++) {
      const s0 = rotr(w[i - 15], 7) ^ rotr(w[i - 15], 18) ^ (w[i - 15] >>> 3)
      const s1 = rotr(w[i - 2], 17) ^ rotr(w[i - 2], 19) ^ (w[i - 2] >>> 10)
      w.push((w[i - 16] + s0 + w[i - 7] + s1) | 0)
    }
    let [a0, b0, c0, d0, e0, f0, g0, h0] = h
    for (let i = 0; i < 64; i++) {
      const t1 = (h0 + (rotr(e0, 6) ^ rotr(e0, 11) ^ rotr(e0, 25)) + ((e0 & f0) ^ (~e0 & g0)) + k[i] + w[i]) | 0
      const t2 = ((rotr(a0, 2) ^ rotr(a0, 13) ^ rotr(a0, 22)) + ((a0 & b0) ^ (a0 & c0) ^ (b0 & c0))) | 0
      h0 = g0; g0 = f0; f0 = e0; e0 = (d0 + t1) | 0; d0 = c0; c0 = b0; b0 = a0; a0 = (t1 + t2) | 0
    }
    const add = [a0, b0, c0, d0, e0, f0, g0, h0]
    for (let i = 0; i < 8; i++) h[i] = (h[i] + add[i]) | 0
  }
  return h.map((x) => (x >>> 0).toString(16).padStart(8, '0')).join('')
}

function isText(value) {
  return typeof value === 'string' && value.trim() !== '' && !/[\r\n]/.test(value)
}
function isSlug(value) {
  return typeof value === 'string' && value.length <= SLUG_MAX && SLUG_RE.test(value)
}
function shq(text) {
  return "'" + String(text).replace(/'/g, "'\\''") + "'"
}
function plain(value) {
  return value && typeof value === 'object' && !Array.isArray(value)
}
function distinct(list) {
  return new Set(list).size === list.length
}

// validateRulings — the `rulings` argument, or why it is refused. A ruling is one of the
// human's three dispositions on a finding, or a declared bound; nothing else is one.
// runRuling — whether a ruling is about the RUN and not about a finding or a bound: the go
// after a stop, one more re-run of a clause, one more triage of a finding, one more attempt
// of a stage, and either bound raised. AND, OF A RUN WITH A QUEUE (the reshape's plan, T7;
// dev/stabilize-record run-set: A BUDGET AND AN ORDER THE HUMAN CHANGES ARE FACTS A RULING
// WRITES; `queued`), the three in QUEUE_RULINGS: another order, the rows the human puts into
// the queue by key, and the budget raised.
// QUEUE_RULINGS IS THE ORDER THEIR CALLS ARE MADE IN, whichever was said first
// (`rulingsRecordPrompt`): that script holds a raise to the state as it stands before its
// call — the budget spent, and a row of the queue nobody reached — and another order, or a
// row put in by key, may be what puts a row there. A call of a batch reads what the calls
// before it wrote, so the raise is judged on what the other two leave.
// THE NUMBER A VERIFIER HANDS ON IS NO RULING (`hand-on`, ONCE): that script writes it once,
// at the run's opening, and refuses a second write (`exists`) — the plan names it a third
// ruling about the run, and the committed script has none. An invocation that carries one is
// refused before any agent, with that reason.
const QUEUE_RULINGS = ['order', 'queued', 'budget']
const RUN_RULINGS = ['go', 'rerun', 'rounds', 'cycles', 'reverify', 'again'].concat(QUEUE_RULINGS)
const ONCE = '`hand-on` is no ruling: how many items a verifier hands on to the queue is written ONCE, at the run\'s opening, and `' + RECORD_TOOL + ' run-set` refuses a second write of it (`exists`) — no ruling changes it, and nothing was recorded. What the human rules about a run\'s queue is its budget, raised (`budget`), another order of its classes (`order`), and the rows he puts into it by key (`queued`)'
// A number of the record script's: a positive whole number of at most six digits (its NUMBER).
const MAX_NUMBER = 999999
// A class, as an order names one: `<kind> <origin>`, two words. WHICH kinds and origins there
// are is the record script's (its CLASS), and its refusal of one it does not know is the
// rulings' batch's own — as a ruling on a key the ledger lacks is.
const CLASS_RE = /^[a-z]+(-[a-z]+)* [a-z]+(-[a-z]+)*$/
function runRuling(r) {
  return plain(r) && RUN_RULINGS.some((name) => r[name] != null)
}
function validateRulings(rulings) {
  if (!Array.isArray(rulings) || rulings.length === 0) return 'args.rulings must be a non-empty list of rulings'
  const keys = []
  const about = []
  for (const r of rulings) {
    if (!plain(r)) return 'a ruling must be an object, not ' + JSON.stringify(r)
    if (r['hand-on'] != null) return ONCE
    if (runRuling(r)) {
      const named = Object.keys(r)
      if (named.length !== 1) return 'a ruling about the run names one thing — ' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + ' — and nothing beside it: ' + JSON.stringify(r)
      if (r.go != null && r.go !== true) return 'the human\'s go is `go` as true, or is not passed at all: ' + JSON.stringify(r)
      if (r.rerun != null && !isSlug(r.rerun)) return '`rerun` names the clause that is granted one more re-run, a slug as the clause table spells it: ' + JSON.stringify(r)
      if (r.rounds != null && !(Number.isInteger(r.rounds) && r.rounds >= 1 && r.rounds <= MAX_ROUNDS)) return '`rounds` is the bound across rounds, raised: a whole number from 1 to ' + MAX_ROUNDS + ', not ' + JSON.stringify(r.rounds)
      if (r.cycles != null && !(Number.isInteger(r.cycles) && r.cycles > DEFAULT_CYCLES && r.cycles <= MAX_CYCLES)) return '`cycles` is the bound on a round\'s fix cycles, raised: a whole number above ' + DEFAULT_CYCLES + ' and at most ' + MAX_CYCLES + ', not ' + JSON.stringify(r.cycles)
      if (r.reverify != null && !isSlug(r.reverify)) return '`reverify` names the finding that is granted one more triage, by its ledger key: ' + JSON.stringify(r)
      if (r.again != null && !STAGES.includes(r.again)) return '`again` names the stage that is granted one more attempt — ' + STAGES.join(' or ') + ': ' + JSON.stringify(r)
      if (r.budget != null && !(Number.isInteger(r.budget) && r.budget >= 1 && r.budget <= MAX_NUMBER)) return '`budget` is how many verdicts a round\'s `test` stage may spend, raised: a whole number from 1 to ' + MAX_NUMBER + ', not ' + JSON.stringify(r.budget)
      if (r.order != null && !(Array.isArray(r.order) && r.order.length > 0 && r.order.every((entry) => typeof entry === 'string' && CLASS_RE.test(entry)) && distinct(r.order))) return '`order` is the order of classes the queue is ranked by: a non-empty list of classes, each `<kind> <origin>` and none twice, not ' + JSON.stringify(r.order)
      if (r.queued != null && !(Array.isArray(r.queued) && r.queued.length > 0 && r.queued.every(isSlug) && distinct(r.queued))) return '`queued` is the rows the human puts into the queue: a non-empty list of ledger keys, none twice, not ' + JSON.stringify(r.queued)
      about.push(named[0] + ':' + (r.rerun || r.reverify || r.again || ''))
      continue
    }
    const declares = r.bound != null
    if (r.key == null && !declares) return 'a ruling names a finding by `key`, or declares a bound by `bound`: ' + JSON.stringify(r)
    if (r.key != null) {
      if (!isSlug(r.key)) return 'a ruling\'s key ' + JSON.stringify(r.key) + ' is not a ledger key'
      if (!HUMAN_RULINGS.includes(r.ruling)) return 'ruling ' + JSON.stringify(r.ruling) + ' on ' + r.key + ' is not one of the human\'s: ' + HUMAN_RULINGS.join(', ')
      if (declares !== (r.ruling === 'bound')) return 'ruling on ' + r.key + ': `bound` names the declared bound of a `bound` ruling, and of no other'
      if (r.note != null && (declares || !isText(r.note))) return 'ruling on ' + r.key + ': `note` is one line of text, for `admitted` and `later`'
      keys.push(r.key)
    } else if (r.ruling != null || r.note != null) {
      return 'a declared bound without a finding carries no `ruling` and no `note`: ' + JSON.stringify(r)
    }
    if (declares) {
      if (!isSlug(r.bound)) return 'the bound ' + JSON.stringify(r.bound) + ' is not a slug'
      for (const cell of ['reach', 'where', 'pin']) if (!isText(r[cell])) return 'the bound ' + r.bound + ' needs `' + cell + '` as one line of text (`pin`: the test that pins it, or the word unpinned)'
    }
    const known = r.key != null ? (declares ? ['key', 'ruling', 'bound', 'reach', 'where', 'pin'] : ['key', 'ruling', 'note']) : ['bound', 'reach', 'where', 'pin']
    const unknown = Object.keys(r).filter((name) => !known.includes(name))
    if (unknown.length) return 'a ruling has no field ' + unknown.join(', ') + ': ' + JSON.stringify(r)
  }
  if (!distinct(keys)) return 'args.rulings rules on a finding more than once'
  if (!distinct(about)) return 'args.rulings says the same thing about the run more than once'
  if (about.length && about.length !== rulings.length) return 'args.rulings mixes rulings about the run (' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + ') with rulings on a finding or a bound: the first are recorded on the loop branch by an invocation that starts nothing, the others ride the round — two invocations'
  return null
}

// NOT_FIT — the stages that refuse to start, each with why. The build of this workflow was
// reviewed red; a stage is used on a real run only once the repair of its half and the
// re-review of that repair are recorded in DECISIONS.md, the `test` half first (the human's
// ruling of 2026-10-06 — DECISIONS.md, "The stabilization workflow's build, reviewed red",
// ruling 3). LIFTING A REFUSAL IS ONE EDIT: the stage's line is deleted from this table, by
// the commit that records that re-review under a DECISIONS.md heading with the words
// "the `<stage>` half of the stabilization workflow, repaired and re-reviewed" —
// tooling-tests/stabilize_harness_fence.rs, arm (m), takes the deletion in a tree that has
// that heading and in no other, and neither it nor the self-test is edited with it.
const BUILD_RECORD = 'completions/artifacts/M55/stabilization-build/README.md'
const NOT_FIT = {
  fix: 'its repair and the re-review of that repair are not recorded',
}
// notFit — why an invocation's stage refuses to start, or null. Asked before any agent runs.
// An invocation that only records what the human ruled ABOUT THE RUN (RUN_RULINGS) starts
// no stage — it runs two reads and the one record step, on the loop branch,
// exactly as the other stage's does — and is not refused. A ruling on a finding, or a
// declared bound, is the stage's own first step and is refused with it.
function notFit(a) {
  if (!NOT_FIT[a.stage]) return null
  if (a.rulings != null && a.rulings.every(runRuling)) return null
  return 'the `' + a.stage + '` stage is NOT FIT FOR USE and refuses to start: ' + NOT_FIT[a.stage] + '. The build of the stabilization workflow was reviewed red — its record is ' + BUILD_RECORD + ' — and a stage is used only once its half is repaired and re-reviewed. Nothing was run: no agent, no read. What is taken meanwhile: ' + STAGES.filter((stage) => !NOT_FIT[stage]).map((stage) => 'the `' + stage + '` stage').concat(['what the human rules about the run (args.rulings: ' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + '), which either stage records and which starts nothing']).join('; and ') + '. A ruling on a finding or a declared bound is this stage\'s own first step and waits with it — until then an invocation of `test` that carries only such rulings records them on the loop branch, and starts nothing'
}

// scratchFault — why a value is no scratch root, or null.
function scratchFault(value) {
  return typeof value !== 'string' || !SCRATCH_RE.test(value) || /\/\.\.?(\/|$)/.test(value) ? 'args.scratch ' + JSON.stringify(value) + ' is not an absolute directory path of plain segments ([A-Za-z0-9._-]): every agent works under it, and it reaches a shell' : null
}
// probeFault — why an invocation that names a probe is refused, or null. A probe works on no
// run: it takes the scratch root, and nothing a stage takes. Whether that root is there, a
// directory and outside the repository is a file system's to say, and the probe's first step
// asks the tool, which refuses before anything is written.
function probeFault(a) {
  if (!PROBES.includes(a.probe)) return 'args.probe ' + JSON.stringify(a.probe) + ' is not a runtime probe: ' + PROBES.join(', ')
  const beside = Object.keys(a).filter((name) => !['probe', 'scratch', 'seconds'].includes(name))
  if (beside.length) return 'a probe works on no run and no stage: it takes `scratch` — and `seconds`, for `hold` — and nothing else, not ' + beside.join(', ')
  if (scratchFault(a.scratch)) return scratchFault(a.scratch)
  if (a.seconds != null && (a.probe !== 'hold' || !Number.isInteger(a.seconds) || a.seconds < 1 || a.seconds > PROBE_HOLD_MAX)) return 'args.seconds is how long the `hold` probe holds its command — a whole number from 1 to ' + PROBE_HOLD_MAX + ' — and no argument of another probe: not ' + JSON.stringify(a.seconds)
  return null
}

// validateArgs — every refusal that precedes the first agent. Returns the message of the
// refusal, or null.
function validateArgs(a) {
  if (!plain(a)) return 'args must be an object: { stage, run, scratch, … }'
  const unknown = Object.keys(a).filter((name) => !ARGS.includes(name))
  if (unknown.length) return 'unknown arg(s) ' + unknown.join(', ') + ' — the args are ' + ARGS.join(', ')
  if (a.selfTest != null && typeof a.selfTest !== 'boolean') return 'args.selfTest must be true or false'
  if (a.selfTest) return null
  if (a.probe != null) return probeFault(a)
  if (!STAGES.includes(a.stage)) return 'args.stage ' + JSON.stringify(a.stage) + ' is not one of ' + STAGES.join(', ')
  if (!isSlug(a.run)) return 'args.run ' + JSON.stringify(a.run) + ' is not a slug: lowercase a-z0-9 words joined by single dashes, at most ' + SLUG_MAX + ' characters — it names the run\'s directory and its branches'
  if (scratchFault(a.scratch)) return scratchFault(a.scratch)
  if (a.seconds != null) return 'args.seconds is the `hold` probe\'s, and no argument of a stage'
  if (a.model != null && (typeof a.model !== 'string' || !MODEL_RE.test(a.model))) return 'args.model ' + JSON.stringify(a.model) + ' is not a model name'
  if (a.stopAfter != null && !STOPS[a.stage].includes(a.stopAfter)) return 'args.stopAfter ' + JSON.stringify(a.stopAfter) + ' is not a step of `' + a.stage + '`: ' + STOPS[a.stage].join(', ')
  const only = (name, stage) => (a[name] != null && a.stage !== stage ? 'args.' + name + ' belongs to the `' + stage + '` stage' : null)
  const misplaced = only('scope', 'test') || only('clause', 'test') || only('crossModel', 'test') || only('raise', 'fix') || only('exit', 'fix')
  if (misplaced) return misplaced
  if (a.crossModel != null && !(Array.isArray(a.crossModel) && a.crossModel.length > 0 && a.crossModel.every(isSlug) && distinct(a.crossModel))) return 'args.crossModel names the items of the test set that get a cross-model source pass, one by one: [ \'<item>\', … ] — not ' + JSON.stringify(a.crossModel) + '. There is no value that turns it on for every item'
  if (a.clause != null) {
    if (!isSlug(a.clause)) return 'args.clause ' + JSON.stringify(a.clause) + ' is not a clause of the closing condition: a slug, as the clause table spells it'
    if (a.scope != null || a.crossModel != null) return 'args.clause runs the items of one clause again, inside the round that selected them and over that round\'s doors: it takes no args.scope and no args.crossModel'
  }
  if (a.scope != null) {
    const s = a.scope
    const named = plain(s) ? Object.keys(s) : []
    const ok = s === 'delta' || s === 'everything'
      || (named.length === 1 && named[0] === 'range' && typeof s.range === 'string' && RANGE_RE.test(s.range))
      || (named.length === 1 && named[0] === 'doors' && Array.isArray(s.doors) && s.doors.length > 0 && s.doors.every(isText) && distinct(s.doors))
    if (!ok) return 'args.scope must be \'delta\', \'everything\', { range: \'<sha>..<sha>\' } or { doors: [ … ] }, not ' + JSON.stringify(s)
  }
  if (a.rulings != null) {
    const why = validateRulings(a.rulings)
    if (why) return why
    const run = a.rulings.every(runRuling)
    if ((run || a.stage === 'test') && ['scope', 'clause', 'crossModel', 'raise', 'exit', 'stopAfter'].some((name) => a[name] != null)) return 'an invocation that only records rulings — about the run, on either stage; on a finding or a bound, on `test` — records them and starts nothing: it takes no scope, clause, crossModel, raise, exit or stopAfter — invoke the step the returned `next` names afterwards'
  }
  if (a.raise != null) {
    const named = plain(a.raise) ? Object.keys(a.raise) : []
    if (named.length !== 1 || named[0] !== 'cycles' || !Number.isInteger(a.raise.cycles) || a.raise.cycles <= DEFAULT_CYCLES || a.raise.cycles > MAX_CYCLES) return 'args.raise must be { cycles: N } with N above the bound of ' + DEFAULT_CYCLES + ' (and at most ' + MAX_CYCLES + '), not ' + JSON.stringify(a.raise)
  }
  if (a.exit != null) {
    const e = a.exit
    const part = plain(e) && Object.keys(e).length === 1 && Array.isArray(e.part) && e.part.length > 0 && e.part.every((s) => typeof s === 'string' && SHORT_SHA_RE.test(s)) && distinct(e.part)
    if (e !== 'drop' && !part) return 'args.exit must be \'drop\' or { part: [<the fix commits to keep>] }, not ' + JSON.stringify(e)
    if (a.raise != null) return 'args.exit and args.raise are two different exits at a bound: pass one'
  }
  return null
}

// reporterName — the name a reporter writes its report under: a slug, or null when the
// parts do not make one (the caller halts, naming them — never a truncated name).
function reporterName(parts) {
  const name = parts.join('-')
  return isSlug(name) ? name : null
}

// The reporters of one stage of one attempt: every name this script hands out, once.
function launcher(ctx) {
  const names = []
  return {
    names,
    add(parts) {
      const name = reporterName(parts)
      if (!name) throw new Error('no reporter name can be made of ' + JSON.stringify(parts) + ': it must be a slug of at most ' + SLUG_MAX + ' characters')
      if (names.includes(name)) throw new Error('the reporter ' + name + ' would be launched twice')
      names.push(name)
      return name
    },
    // A reporter that was launched and left no report — `check-reports` said so: the
    // record is not held to a report that does not exist.
    drop(name) {
      if (names.includes(name)) names.splice(names.indexOf(name), 1)
    },
    line(name) {
      return reportLine(ctx, name)
    },
  }
}

function stageFlags(ctx) {
  return '--run ' + ctx.run + ' --round ' + ctx.round + ' --stage ' + ctx.stage + (ctx.stage === 'fix' ? ' --cycle ' + ctx.cycle : '')
}
// A REPORT REACHES ITS WRITER FROM A FILE (dev/stabilize-record: A WRITER FED FROM A FILE): the
// reporter writes it with its file tool to the path this script names, under the scratch
// root, and hands it over by ONE plain call — the labelled line carries both.
function reportFile(ctx, name) {
  return ctx.scratch + '/report/r' + ctx.round + '/' + ctx.stage + (ctx.stage === 'fix' ? '.c' + ctx.cycle : '') + '/' + name + '.a' + ctx.attempt + '.md'
}
function reportCall(ctx, name) {
  return typed(RECORD_TOOL, 'report ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + ' --from ' + reportFile(ctx, name))
}
function reportLine(ctx, name) {
  return LABELS.report + ' ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' — write your report, whole, to the file `' + reportFile(ctx, name) + '` WITH YOUR FILE TOOL, and hand it over by ONE plain call, typed as it stands: ' + reportCall(ctx, name) + ' (with one more `--scratch` for each scratch root of your own outside that one).'
}
// below — a path under the scratch root, as the step tool takes one: without the root.
function below(scratch, path) {
  return path.startsWith(scratch + '/') ? path.slice(scratch.length + 1) : path
}
// `scratch`: the invocation's scratch root, under which the binaries lie — and with it THE
// ONE PLAIN CALL THAT ASSERTS THE HASH, spelled whole, so that no driving agent computes a
// hash by a command of its own or works out a path below the root. A probe's reviewer, which
// drives nothing, is handed none.
function binaryLine(built, withImage, scratch) {
  return LABELS.binary + ' candidate `' + built.candidate.binary + '` sha256 ' + built.candidate.sha256 + ' (commit ' + built.candidate.sha + ', label ' + built.candidate.label + ')'
    + (built.previous ? '; previous release `' + built.previous.binary + '` sha256 ' + built.previous.sha256 + ' (version ' + built.previous.version + ')' : '')
    + (withImage && built.image ? '; trial image `' + built.image.tag + '`' : '')
    + '.' + (scratch ? ' ASSERT THE CANDIDATE\'S HASH BEFORE YOU DRIVE ANYTHING, by ONE plain call typed as it stands, and never by a hash command of your own: ' + typed(STEP_TOOL, 'hash --scratch ' + scratch + ' --file ' + below(scratch, built.candidate.binary)) + ' prints ONE line of JSON whose `content_sha256` must be that sha256' + (built.previous ? ' — and the same call with `' + below(scratch, built.previous.binary) + '` gives the previous release\'s' : '') + '.' : '')
    + ' Return the sha256 you asserted for the candidate as `asserted_sha256`.'
}
function branchLine(branch, loop) {
  return LABELS.branch + ' ' + branch + (branch === loop ? '' : ' (forked from ' + loop + ')') + ', already checked out by the harness. Before you commit, the branch checked out must be exactly `' + branch + '` — if it is not, commit nothing and halt. Never create, switch, merge or push a branch; the harness\'s git steps own them.'
}

// hashMismatch — ruling 11's comparison, made here and by no agent: the reporters that
// drove something and did not return the candidate's hash as the one they asserted.
function hashMismatch(expected, returns) {
  return returns.filter((r) => r.drives && r.result && r.result.status !== 'halted' && r.result.asserted_sha256 !== expected).map((r) => ({ reporter: r.name, asserted: r.result.asserted_sha256 == null ? null : String(r.result.asserted_sha256) }))
}

// lacks — the rows of NEEDS that a role's return does not meet: a needed field that is not
// there, where the verdict the need holds under is the return's.
function lacks(role, result) {
  const held = (path) => path.split('.').reduce((at, name) => (at == null ? undefined : at[name]), result)
  return (NEEDS[role] || []).filter((n) => (!n.with || result.verdict === n.with) && held(n.field) == null)
}
// lacksWhy — a needed field that is not there, as the reason its cell is recorded with: THE
// REPORTER AND THE FIELD, by name, and why the stage needs it.
function lacksWhy(reporter, need) {
  return '(' + reporter + ') returned no `' + need.field + '` — ' + need.why
}
// unasserted — why a reporter's return is NO EVIDENCE ABOUT THE CANDIDATE, or null: it
// drove, and the hash it returns as the one it asserted is not there (NEEDS) or IS ANOTHER.
// THE TWO ARE ONE CELL. A definition tells a driving agent to HALT where the hash of the
// binary it is handed is not the one on its line, and such a halt voids its item and
// nothing else; a returned hash that is another is that same fact, found by the comparison
// this script makes where the agent did not make it — so it is that agent's, and no other
// agent's evidence, each held to its own assertion, is touched by it. What comes back is
// printed only where it is a sha256: a return's text is an agent's.
function unasserted(role, reporter, result, expected) {
  const gap = lacks(role, result).find((n) => n.field === 'asserted_sha256')
  if (gap) return lacksWhy(reporter, gap)
  if (!ROLES[role].drives || result.asserted_sha256 === expected) return null
  return '(' + reporter + ') returned ' + (SHA256_RE.test(String(result.asserted_sha256)) ? 'the `asserted_sha256` ' + result.asserted_sha256 : 'an `asserted_sha256` that is no sha256') + ', which is not the candidate\'s binary (' + expected + '): nothing it drove is evidence about this candidate'
}

// regressionOf — THE REGRESSION FACT OF A CONFIRMED VERDICT, OR null WHERE NOBODY KNOWS IT
// (NEEDS: `unknown`; the orchestrator's ruling of 2026-10-07, built as the second repair
// plan's X1b). The fact is what the verifier returned — true, false, or NOT_COMPARABLE where
// the door is one the previous release does not have (dev/stabilize-record: triage-set,
// `regression`) — AND is backed by the previous release's binary: the hash it returned for
// that binary is there and is the one it was handed. Anything else — no `regression`, one
// that is none of the three, no `ran_on.previous`, or another hash — is a fact nobody
// established: the verdict stands, the fact is unknown, and `unknown` says why in the
// stage's log.
const NOT_COMPARABLE = 'not-comparable'
function regressionOf(reporter, result, previous) {
  const gap = lacks('verify', result).find((n) => n.then === 'unknown')
  const ran = result.ran_on || {}
  const why = gap ? lacksWhy(reporter, gap)
    : typeof result.regression !== 'boolean' && result.regression !== NOT_COMPARABLE ? '(' + reporter + ') returned a `regression` that is neither true, false nor `' + NOT_COMPARABLE + '`'
      : ran.previous !== previous ? '(' + reporter + ') returned ' + (SHA256_RE.test(String(ran.previous)) ? 'the hash ' + ran.previous : 'a hash that is no sha256') + ' for the previous release\'s binary, which is not the one it was handed (' + previous + '): what its regression fact rests on was not that release'
        : null
  return why ? { regression: null, unknown: why } : { regression: result.regression, unknown: null }
}

// reportsRead — what a `check-reports` step established, or why it established nothing:
// the launched reporters that left NO REPORT (`missing` — each is then taken off the list,
// and what it was launched for is recorded as not done), or the files nobody launched
// (`extra` — how many: their names are in the file the step's digest names), which stop the
// stage: a report nobody asked for, a stray file. And WHAT WAS SET ASIDE (`aside`): a file at
// a report's path that the record script would not have written is no report — the check
// moved it out of the tree, its reporter is among the `missing`, and `why` is the word of
// what refused it.
function reportsRead(step) {
  const seen = step && step.status === 'checked' && plain(step.check) ? step.check : null
  if (!seen || !Array.isArray(seen.missing) || !Number.isInteger(seen.extra) || !Array.isArray(step.aside)) return { fault: 'the report check could not be read' + (step && step.refused ? ': it refused ' + stepRefusal(step.refused) : ''), transient: !step, file: fileOf(step) }
  if (seen.extra) return { fault: 'the stage\'s report directory holds ' + seen.extra + ' file(s) no launched reporter wrote — a file nobody launched is nobody\'s report; the step\'s own line names each', file: fileOf(step) }
  return { missing: seen.missing, aside: step.aside }
}
// fileOf — the file a step's digest names for what it leaves out — the line the tool prints
// unasked: a refusal's halt report, the names behind a count — by its path under the
// scratch root and its sha256; or null.
function fileOf(step) {
  return step && isText(step.file) && SHA256_RE.test(String(step.file_sha256)) ? { file: step.file, sha256: step.file_sha256 } : null
}
// setAside — why a reporter's report was set aside, as the word the check gives; or null.
// A report's file is `<reporter>.a<attempt>.md`.
function setAside(aside, name) {
  const moved = (aside || []).find((a) => plain(a) && typeof a.path === 'string' && a.path.slice(a.path.lastIndexOf('/') + 1).replace(/\.a[0-9]+\.md$/, '') === name)
  return moved ? String(moved.why) : null
}
// endingOf — how a reporter that did not report ended, in words a void's reason carries.
function endingOf(result) {
  return result ? 'halted' + (result.halt && result.halt.root_cause ? ' (' + result.halt.root_cause + ')' : '') : 'returned nothing'
}

// unreported — a unit's reporters whose report is NOT THERE, taken out of what the unit
// established: a result nobody can read a report for is no result. A step of the unit's own
// chain voids the unit; the cross-model pass voids that pass, and nothing else.
function unreported(ran, missing, aside) {
  for (const unit of ran) {
    for (const r of unit.reporters.filter((x) => missing.includes(x.name))) {
      // A report that was set aside is a report its reporter did not leave: the file at
      // its path was not what the record script would have written.
      const moved = setAside(aside, r.name)
      if (r.crossModel) unit.crossModel = 'void'
      else if (unit.status !== 'void') Object.assign(unit, { status: 'void', reason: 'its `' + r.as + '` step ' + (moved ? 'left a file at its report\'s path that is no report (`' + moved + '`): it was set aside, and the step left none' : r.result && r.result.status === 'reported' ? 'returned and left no report' : endingOf(r.result) + ', and left no report'), word: voidWord(moved ? 'set-aside' : !r.result ? 'silent' : r.result.status === 'reported' ? 'unreported' : 'halted', r.name) })
      r.result = null
      r.report = null
    }
  }
}

// outcomeOf — `next` exactly as the state document gives it, and whether this script knows
// the value. It knows nothing else about it — but `rule` is the human's step for a finding
// on the human's list (one that is still ungraded or unverified after its retry among them:
// its `why` says so), for a clause that is still not green after its one re-run, and for a
// stage whose attempts did not reach their record, so all three go back with it: how many
// findings the human's list holds — the list itself is in the state document's file, which
// every return names — the keys of it that may be granted one more triage, and the clauses
// and the stages by name.
function outcomeOf(state) {
  const out = { next: state.next, known: KNOWN_NEXT.includes(state.next) }
  if (out.next === 'rule') out.rule = { findings: state.human_list || 0, reverify: state.reverify || [], clauses: state.human_clauses || [], stages: state.human_stages || [] }
  return out
}

// refusalOf — the word the state's position refuses a stage with, as what the orchestrator
// does about it. The words are dev/stabilize-record's (its header: THE POSITION), and
// tooling-tests/stabilize_harness_fence.rs holds this table to them.
const REFUSALS = {
  'not-ready': 'the run\'s opening is not done, and no stage starts before it is — the state\'s `not_ready` names what it owes, each a fact of the run: the clauses of the closing condition, the stop mode, the bound across rounds, the previous release and the default scope (`dev/stabilize-record run-set`)',
  'no-round': 'there is no tested round to fix — the `test` stage comes first, and records its triage',
  'not-tested': 'the round\'s `test` stage has not reached its record — it is run again first, as the next attempt',
  'round-open': 'the round is tested and a finding of it is still open — run `fix`, record the human\'s rulings with it, or drop the round (args.exit = \'drop\'); where the state\'s `next` is `triage`, the stage whose position says `triage` finishes the round\'s triage first',
  'round-over': 'the round is over — the next stage is `test`',
  'cycle-bound': 'the bound on a round\'s fix cycles is spent — as many fix -> audit cycles are recorded in this round as it allows — and the next step is the human\'s: one more cycle is allowed by the raised bound, passed as a ruling about the run (args.rulings, `cycles`), which records it (`dev/stabilize-record run-set`) and starts nothing; the other exits are to rule on what is still open, or to leave the round',
  'attempts-spent': 'this stage has left reports and no record in this round as many times in a row as are tried without the human — the state\'s `human_stages` names it — and one more attempt is the human\'s to grant: pass it as a ruling about the run (args.rulings, `again`), which records it and starts nothing; what the earlier attempts halted on is in their returns and their reports',
  'round-bound': 'the bound across rounds is spent — as many rounds have a fix stage on record as it allows — and one more fix round is the human\'s to allow: pass the raised bound as a ruling about the run (args.rulings, `rounds`), which records it (`dev/stabilize-record run-set`) and starts nothing',
  'stopped': 'the run stops after every round, and the round is over: the next one waits for the human\'s go — pass it as a ruling about the run (args.rulings, `go`), which records it and starts nothing; the state\'s `stop.then` names the step that follows',
}
function refusalOf(at) {
  return REFUSALS[at.refused] || 'the record script refuses it with a word this script has no sentence for'
}

// runRulingsFault — why the state does not ask the human for what the rulings about the run
// say, or null. The record script refuses the same (its `round-set`: go, granted); asked
// here first, so that a ruling nobody was asked for costs no record step.
function runRulingsFault(rulings, state) {
  const facts = state.facts || {}
  // WHAT THE HUMAN RULES ABOUT THE QUEUE, held to the state's digest of it (`queue`). A run
  // without an order has no queue, and its opening is where one is written. A RAISE THAT
  // STANDS ALONE is held to what the digest shows of the budget's own stop — nothing left of
  // it, and a row in the queue — and refused IN THE RECORD SCRIPT'S OWN WORDS (its `run-set`:
  // budget), with the three numbers that script would name. BESIDE ANOTHER ORDER OR A ROW PUT
  // IN BY KEY IT IS NOT ASKED HERE: those calls are made first, this script cannot read the
  // state they leave, and the raise's own call is the check — refused, it refuses the batch
  // whole, and nothing of the order is written either.
  const queue = hasQueue(state) ? state.queue : null
  const moves = rulings.some((r) => r.order != null || r.queued != null)
  for (const r of rulings) {
    const about = QUEUE_RULINGS.find((name) => r[name] != null)
    if (about && !queue) return '`' + about + '` rules on the queue of a run, and this run has no queue: its opening holds no order. The order, the budget and the number a verifier hands on are written at a run\'s opening (`' + RECORD_TOOL + ' run-set`)'
    if (r.budget != null && queue.budget != null && r.budget <= queue.budget) return 'the budget is ' + queue.budget + ': `budget` raises it, and ' + r.budget + ' does not'
    if (r.budget != null && queue.budget != null && !moves && !(queue.left === 0 && queue.length > 0)) return 'the budget: the state does not ask for it — it is written at the run\'s opening, and raised only while it is spent and the queue holds a row nobody reached (' + queue.spent + ' of ' + queue.budget + ' spent, ' + queue.length + ' in the queue)'
    if (r.go != null && !(state.next === 'stop' && state.stop && state.stop.why === 'every-round')) return 'the run is not stopped after a round for the human\'s go — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': a go is recorded for that stop and for no other state'
    if (r.rerun != null && !(state.human_clauses || []).some((c) => c.clause === r.rerun)) return 'the clause `' + r.rerun + '` is not the human\'s: one more re-run is granted to a clause that is still not green after its re-run — the state\'s `human_clauses` names ' + ((state.human_clauses || []).map((c) => c.clause).join(', ') || 'none')
    if (r.rounds != null && facts.rounds != null && r.rounds <= facts.rounds) return 'the bound across rounds is ' + facts.rounds + ': `rounds` raises it, and ' + r.rounds + ' does not'
    if (r.rounds != null && (state.rounds || []).length && !(state.not_ready || []).length && !(state.stop && state.stop.why === 'round-bound')) return 'the run is not stopped at its bound across rounds — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': once a round is begun that bound is raised at its own stop, and at no other state'
    if (r.cycles != null && !(state.next === 'stop' && state.stop && state.stop.why === 'cycle-bound')) return 'the run is not stopped at the bound on a round\'s fix cycles — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': that bound is raised at its own stop, and at no other state'
    if (r.cycles != null && facts.cycles != null && r.cycles <= facts.cycles) return 'the bound on a round\'s fix cycles is ' + facts.cycles + ': `cycles` raises it, and ' + r.cycles + ' does not'
    if (r.reverify != null && !(state.reverify || []).includes(r.reverify)) return 'the finding `' + r.reverify + '` is not the human\'s to grant one more triage: that is granted to a finding the state\'s `human_list` names as still ungraded or unverified after its retry — its `reverify` names ' + ((state.reverify || []).join(', ') || 'none')
    if (r.again != null && !(state.human_stages || []).some((h) => h.stage === r.again)) return 'the `' + r.again + '` stage is not the human\'s to grant one more attempt: that is granted to a stage the state\'s `human_stages` names — it names ' + ((state.human_stages || []).map((h) => h.stage).join(', ') || 'none')
  }
  return null
}

// WHETHER A RULING NAMES A ROW THE LEDGER HOLDS is not asked here: the ledger is no part of
// what this script reads, and the rulings' own batch holds every key (`check-ledger`, and
// `ledger-set` itself, which names a key the ledger lacks) — a ruling on no row refuses that
// batch, whole, and nothing is written. Whether a ruling is the right one is the human's;
// the record script takes the three dispositions on any row (its header: WHAT THIS SCRIPT
// CANNOT KNOW).

// ledgerSource — the rows of the ledger whose triage is not finished (the state's
// `untriaged`: nobody graded them, or nobody verified them), as one more source EVERY stage's
// triage is handed beside its own reporters' findings. A row seeded at the opening, and an
// entry an earlier stage left without a verdict, are graded and verified by the next triage
// that runs — never left for a stage of their own, and never for the human as if verified.
// THE ROWS THEMSELVES ARE NOT HANDED OVER BY THIS SCRIPT, which reads how many there are and
// nothing of them: the source names THE READ that prints them whole, and triage runs it.
function ledgerSource(run, state) {
  const awaiting = plain(state.untriaged) && Number.isInteger(state.untriaged.count) ? state.untriaged.count : 0
  if (!awaiting) return []
  return [{ reporter: LEDGER_SOURCE, report: runDir(run) + '/ledger.md', count: awaiting, read: typed(RECORD_TOOL, 'untriaged --run ' + run), findings: [] }]
}
// findingsOf — how many findings a source hands triage: its own lines, or — for the source
// that names a read — as many rows as the state counts.
function findingsOf(source) {
  return source.read ? source.count : source.findings.length
}

// hasQueue — WHICH PATH A `test` STAGE TAKES FROM ITS TRIAGE ON: whether the run has a queue,
// which is what its state's digest says of it (`queue`: null for a run without an order —
// dev/stabilize-step: THE STATE'S DIGEST). The ONE place the two kinds of run are told apart.
function hasQueue(state) {
  return plain(state.queue)
}
// testReport — where a `test` report of a reporter lies in the run's directory, as the record
// script lays it out (its header: `r<N>/reports/test/<reporter>.a<A>.md`; the fence holds the
// two together). It is where that script renders the entries a triage hands over — so what
// the one triage's return is held to — and where a preflight's report lies.
function testReport(ctx, name) {
  return runDir(ctx.run) + '/r' + ctx.round + '/reports/test/' + name + '.a' + ctx.attempt + '.md'
}
// The file the one triage of a run with a queue writes its entries to before it hands them over.
function entriesFile(ctx) {
  return ctx.scratch + '/triage/r' + ctx.round + '.a' + ctx.attempt + '.json'
}
// redSources — EVERY RED RESULT OF AN INVOCATION AS A SOURCE OF THE STAGE'S ONE TRIAGE, IN A
// RUN WITH A QUEUE (the reshape's plan, T5a; its review's B1: the record call files a red
// result as a row, ungraded, in the batch that FOLLOWS the triage — so no triage of the
// stage graded it, and the round's triage was left unfinished by every red check). Each is
// a source of ONE finding under THE KEY ITS ROW IS FILED WITH (`red-<item>`): triage returns
// it under that key, found again, and the stage's first record files the row and then
// grades it. `reds` is what the stage knows of each: the item, its clause, and WHERE ITS
// EVIDENCE LIES — `verdict`, the file the tool kept a held check's verdict in (with
// `output`, the command's whole output); or `report`, the report of the preflight that read
// the check red. A path either way: no word a preflight returned is handed on.
function redSources(reds) {
  return reds.map((r) => ({ reporter: RED_PREFIX + r.item, red: r, findings: ['the check `' + r.item + '` is red · clause: ' + r.clause + ' · the door its row is filed with: the check `' + r.item + '`'] }))
}
// filedFault — WHY THE RETURN OF THE ONE TRIAGE IS NOT TAKEN, or null: what that return is
// held to, which its prompt says in the same words (the arithmetic half of the row
// `wf-triage-reads-harness`). The call that hands the entries over prints ONE line, and the
// return carries three things of it: `report`, which must be the path a report of that
// reporter and attempt has — the script renders the entries there and nowhere else; `sha256`,
// a hash; and `handed`, what the call was typed with for each source — which must be the
// sources and the counts this stage handed, each by name, BECAUSE THE PARTITION IS HELD TO
// THE COUNTS OF THE CALL: typed with a smaller one, a finding is in no entry and nothing
// refuses. NO ENTRY IS READ HERE, and none is in the return. What comes back is printed only
// where it is a count: a return's text is an agent's.
function filedFault(ctx, name, sources, t) {
  if (t.report !== testReport(ctx, name)) return 'triage returned ' + (isText(t.report) ? 'a path' : 'no path') + ' that is not where its entries are rendered — `' + testReport(ctx, name) + '`, the path a report of `' + name + '`, attempt ' + ctx.attempt + ', has: the call was typed under another name, round or attempt, or was not made'
  if (!SHA256_RE.test(String(t.sha256))) return 'triage returned no sha256 of its rendered entries — the hash its call printed beside their path: nothing says which text it handed over'
  const said = Array.isArray(t.handed) ? t.handed.filter(plain) : []
  const wrong = sources.filter((s) => !said.some((h) => h.source === s.reporter && h.count === findingsOf(s))).map((s) => {
    const typedWith = said.find((h) => h.source === s.reporter)
    return '`' + s.reporter + '`: handed ' + findingsOf(s) + ', and the call ' + (typedWith && Number.isInteger(typedWith.count) ? 'was typed with ' + typedWith.count : 'named no count for it')
  })
  const strangers = said.filter((h) => !sources.some((s) => s.reporter === h.source)).length
  if (wrong.length || strangers) return 'the call triage handed its entries over by was not typed with the sources and counts this stage handed it — ' + wrong.concat(strangers ? [strangers + ' source(s) of the call that nobody handed'] : []).join('; ') + '. The record script holds the partition to the counts of the call, so a finding of a source named here may be in no entry: nothing is recorded on that'
  return null
}

// verdictFile — WHERE THE RECORD SCRIPT FILES THE VERDICT OF A REPORTER (its header: verdict,
// `r<N>/verdicts/<reporter>.a<A>.json`; the fence holds the two together): the ONE path a
// filing of that reporter, that attempt and that round is written at — and so what the
// return of a verifier that files is held to.
function verdictFile(ctx, name) {
  return runDir(ctx.run) + '/r' + ctx.round + '/verdicts/' + name + '.a' + ctx.attempt + '.json'
}
// The file a verifier of a run with a queue writes its filing to before it files it.
function filingFile(ctx, name) {
  return ctx.scratch + '/verdict/r' + ctx.round + '/' + name + '.a' + ctx.attempt + '.json'
}
// verdictFault — WHY THE RETURN OF A VERIFIER THAT FILES IS NOT HELD, or null (the reshape's
// plan, T6a: "a verifier's return is a word and a path, held to the filing's digest"). The
// call that files a verdict prints ONE line, and the return carries three things of it:
// `filing`, which must be the path a verdict of that reporter and attempt is filed at — the
// script writes it there and nowhere else; `word`, a verdict a filing says; and `contested`,
// a truth value. And `asserted_sha256`, as every driving role returns it. THAT IS ALL THIS
// SCRIPT CAN HOLD OF A RETURN: it has no file system, and no step relays a filed file — the
// state's digest holds how many verdicts count and which keys the queue prints, never a
// filing. SO A RETURN DECIDES NOTHING: whether a verdict was filed, which word it says and
// whether it is contested are the filed file's, read by the record script; a held return of
// a finding the queue prints again left no verdict (`verifyWaves`), and a return that is not
// held is named and nothing else. What comes back is printed only where it is a hash: a
// return's text is an agent's.
function verdictFault(ctx, name, r, expected) {
  if (r.filing !== verdictFile(ctx, name)) return 'returned ' + (isText(r.filing) ? 'a path' : 'no path') + ' that is not where a verdict of `' + name + '`, attempt ' + ctx.attempt + ', is filed — `' + verdictFile(ctx, name) + '`: the call was typed under another name, round or attempt, or was not made'
  if (!FILED_WORDS.includes(r.word)) return 'returned ' + (r.word == null ? 'no word' : 'a word') + ' that is no verdict a filing says — ' + FILED_WORDS.join(', ') + ' — which the call prints as `word`'
  if (typeof r.contested !== 'boolean') return 'did not return `contested` as true or false: whether its filing gave a basis, as the call prints it'
  if (r.asserted_sha256 !== expected) return 'returned ' + (r.asserted_sha256 == null ? 'no `asserted_sha256`' : SHA256_RE.test(String(r.asserted_sha256)) ? 'the `asserted_sha256` ' + r.asserted_sha256 : 'an `asserted_sha256` that is no sha256') + ', which is not the candidate\'s binary (' + expected + ')'
  return null
}
// continuing — WHAT OF A `test` STAGE AN INVOCATION CONTINUES, IN A RUN WITH A QUEUE (the
// reshape's plan, §3 and T6b): the words the state's position says of the stage that left the
// round open, in the order they are worked — `triage`: a row nobody graded is left, one no
// triage saw; `verify`: a row of the queue is unverified, and budget is left; `filed`: A
// FILED VERDICT COUNTS AND NO RECORD TOOK IT (CONTINUES, with what each says in a log and
// in a refusal; dev/stabilize-record: A FILED VERDICT IS RECORDED BEFORE ANYTHING FOLLOWS
// FROM IT) — the stage's record is owed, whatever else is left: with that word alone the
// invocation launches nobody and records. None: the invocation begins a round or runs an
// item again — or the run has no queue, and what its position says is the other path's lap.
const CONTINUES = { triage: 'a row nobody graded is left', verify: 'a row of the queue is unverified, and budget is left', filed: 'a filed verdict is in no record' }
function continuing(state, at) {
  return hasQueue(state) ? Object.keys(CONTINUES).filter((word) => at[word] === true) : []
}
// continueFault — why an invocation that continues takes nothing of what it was passed, or
// null. It runs no instrument and resolves no scope, so it takes no scope, no clause and no
// cross-model pass; and it has no preflight, no instrument and no step of its own to be
// stopped after — but its first read, which only looks.
function continueFault(a) {
  const passed = ['scope', 'clause', 'crossModel'].filter((name) => a[name] != null).concat(a.stopAfter != null && a.stopAfter !== 'state' ? ['stopAfter: \'' + a.stopAfter + '\''] : [])
  return passed.length ? 'this invocation CONTINUES the round\'s `test` stage — it runs no instrument, resolves no scope and grades only rows nobody graded — so it takes no scope, clause or crossModel, and can be stopped after nothing but its first read (stopAfter: \'state\'): it was passed ' + passed.join(', ') : null
}
// limitSaid — WHAT A HALT SAYS, AND NAMES, WHERE IT MAY BE THE ACCOUNT'S USAGE LIMIT (the
// reshape's plan, T6b; its §2.2). The limit never reaches this script as a message, so the
// halt says what it saw and never what it is: `last` is the wave that was made the last —
// its number, and how many of its verifier calls returned nothing — and the step that halted
// returned nothing too. AND IT NAMES THE FILED VERDICTS, because until the stage's record
// takes them they are files of this machine's disk and of no commit: `counted`, how many
// verdicts the state counts in the round, as it was last read — on record or filed;
// `files`, the path of each one a verifier of THIS invocation filed — held by its return and
// by the queue, which did not print the finding again; `unconfirmed`, the paths returned as
// filed in a wave whose report check or whose read of the queue did not come back, which
// nothing held; and `directory`, where every filed verdict of the round lies. Which of them
// count is the state document's `filed`, in the file every return names (`named.document`):
// no step relays a filing, and this script reads none.
// THE HALT IS HANDED WHICH STEP HALTED (`step`, a row of LIMIT_STEPS; the scoped review before
// the parts, its finding 3 in this script: the same sentences were appended at every step,
// and said what was not so at most of them). What differs from step to step is three facts,
// and each sentence is said only where it is true:
//   `read`    whether the state was read AFTER the wave that was made the last. Where its
//             report check or the read of the queue is the step that returned nothing, IT WAS
//             NOT (`before`): the count is of before that wave, the state document the return
//             names is that read's and lists nothing the wave filed — and the halt says so,
//             and names THE ONE PLAIN CALL THAT PRINTS THE STATE AS IT STANDS NOW.
//   `commit`  what became of the record's commit: `none` — no commit holds what was filed;
//             `unknown` — the commit step itself returned nothing, and may have made it;
//             `made` — committed, and local until the push that is owed; `pushed`. From
//             `made` on the halt names the commit (`record`) and never says *in no commit*.
//   WHAT THE NEXT INVOCATION DOES is said by this halt only where nothing else says it — the
//   two steps before the record, where the stage is simply continued. From the composing step
//   on, the record step's, the push's and the read-back's own sentence says it, and the halt
//   carries that sentence (`recordThen`, `pushThen`, `halt`): there the first read finishes a
//   batch or makes a push before anything is continued.
const LIMIT_STEPS = {
  'report-check': { step: 'the report check after that wave', read: 'before', commit: 'none' },
  'queue': { step: 'the read of the queue after that wave', read: 'before', commit: 'none' },
  'compose': { step: 'the step that composes the record', read: 'after', commit: 'none' },
  'gate': { step: 'a step of the record\'s gate', read: 'after', commit: 'none' },
  'commit': { step: 'the record\'s commit step', read: 'after', commit: 'unknown' },
  'push': { step: 'the push of the record', read: 'after', commit: 'made' },
  'read-back': { step: 'the read of the state after the record', read: 'after', commit: 'pushed' },
}
// (`record`: the commit of the record this invocation made after its waves, where it made one.)
function limitSaid(ctx, last, counted, files, unconfirmed, step, record) {
  const at = LIMIT_STEPS[step]
  const directory = runDir(ctx.run) + '/r' + ctx.round + '/verdicts'
  const committed = at.commit === 'made' || at.commit === 'pushed'
  const named = (files.length ? ' — of this invocation\'s verifiers: ' + files.join(', ') : ' — no verifier of this invocation filed one') + (unconfirmed.length ? '; returned as filed in the wave whose report check or whose read of the queue did not come back, and held by nothing: ' + unconfirmed.join(', ') : '')
  const lies = 'each verdict that is filed is a file under ' + directory + '/ on this machine\'s disk' + named + '.'
  const filed = committed ? ' WHAT WAS FILED IS NOT LOST, AND IS IN A COMMIT: the record this invocation made (' + record + ') took it' + (at.commit === 'pushed' ? ', and is pushed' : ' — a local commit until the push that is owed is made') + '. In that commit, ' + lies
    : at.commit === 'unknown' ? ' WHAT WAS FILED IS NOT LOST: ' + lies + ' WHETHER A COMMIT HOLDS IT THIS SCRIPT CANNOT SAY — the commit step may have made its commit before it returned nothing: the next invocation\'s first read says which, and finishes the record either way.'
      : ' WHAT WAS FILED IS NOT LOST, AND IS IN NO COMMIT YET: ' + lies
  const state = at.read === 'before' ? ' THE STATE WAS LAST READ BEFORE THAT WAVE: it counted ' + counted + ' verdict(s) in round ' + ctx.round + ' then, and the state document this return names is that read\'s — it lists nothing wave ' + last.wave + ' filed. ' + typed(RECORD_TOOL, 'state --run ' + ctx.run) + ' prints the state as it stands now, on one line: every filed verdict, with whether it counts (`filed`).'
    : ' The state counts ' + counted + ' verdict(s) in round ' + ctx.round + ', and the state document this return names lists every filed verdict with whether it counts (`filed`)' + (at.commit === 'none' ? '' : ' — as it was read before that record\'s commit: what it says of which record took a verdict (`recorded`) is of then') + '.'
  const then = at.read === 'before' ? ' Wait for the window, then invoke the stage again with the same args: where the state\'s position still hands it the stage, it CONTINUES — no instrument, the filed verdicts taken as they stand, and only what is missing launched; where it does not, its return says why, and whichever record the run makes next takes what is filed.'
    : ' Wait for the window, then invoke the stage again with the same args: what that invocation does first is said above, with this halt.'
  return {
    more: { limit: { possible: true, wave: last.wave, silent: last.silent, step }, filed: { counted, read: at.read + '-the-wave', commit: at.commit, record: committed ? record : null, files, unconfirmed, directory } },
    says: ' THIS MAY BE THE ACCOUNT\'S USAGE LIMIT — this script cannot tell: a limit never reaches it as a message, it is handed nothing. ' + last.silent + ' verifier call(s) of wave ' + last.wave + ' returned nothing, so that wave was made the last, and then ' + at.step + ' returned nothing too.' + filed + state + then,
  }
}

// previousOf — the release the run measures against, as its record names it: what the
// second binary is built from, and what "a regression of the run" is green on.
function previousOf(state) {
  const facts = state.facts || {}
  return { version: facts.previous, commit: facts['previous-commit'] }
}

// evidenceOf — per clause, what the record script derived for it: its status, the round and
// the commit of the latest run among its items, how many fix rounds behind the candidate
// that is, and where each item's own run stands. It goes back with `close`, so that the
// human closes with that number in front of them.
function evidenceOf(state) {
  // An item's entry holds, of where its own run stands, what is not null: a digest prints
  // no null it can leave out.
  const at = (value) => (value == null ? null : value)
  return (state.clauses || []).map((c) => ({ clause: c.clause, status: c.status, round: c.round, commit: c.commit, behind: c.behind, items: (state.items || []).filter((i) => i.clause === c.clause).map((i) => ({ item: i.item, round: at(i.round), attempt: at(i.attempt), standing: at(i.standing), why: at(i.why) })) }))
}

// namedOf — what every return names of the state beside `next`: the in-scope items no tested
// round's scope has selected, by id; how many doors of the round in hand no item of the test
// set names; HOW MANY FINDINGS ARE CONFIRMED WITH THEIR REGRESSION FACT UNKNOWN and not yet
// recorded (`regression_unknown`: the digest lists their keys, and the document says which);
// and the file the state document lies in — its path and its sha256 — where the
// doors are named and everything else this script does not read can be read. None of the
// three forbids closing: the human reads them at every stop, and at the close.
function namedOf(state, round) {
  if (!state) return null
  const within = round == null ? state.round : round
  const bare = (state.uncovered || []).find((u) => u.round === within)
  return { never_selected: state.never_selected || [], uncovered: { round: within == null ? null : within, doors: bare ? bare.doors : 0 }, regression_unknown: (state.regression_unknown || []).length, document: state.document || null }
}

// queueSaid — WHAT EVERY RETURN SAYS OF A RUN WITH A QUEUE, BESIDE `next` (the reshape's plan,
// T7; the row `wf-return-names`): the budget — its limit, what is spent, what is left — the
// queue's length, in all and class by class, with the number a verifier hands on, and THE
// STOP'S VIEW: its round, the file it lies in, the sha256 that file must have, how the file
// on disk stands to it (`on_disk` — it is the stop's view only where that says `written`),
// the hit rate in all and class by class, and how long each of its lists is. EACH IS THE
// STATE'S DIGEST'S, AS THE STEP TOOL PRINTS IT (dev/stabilize-step: THE STATE'S DIGEST,
// `queue` and `view`): nothing here is counted, added up or ranked by this script, and NO KEY
// IS IN IT — the queue's first keys are what a wave is launched from, and stay there.
// AND WHAT AN INVOCATION OF `test` CONTINUES THE STAGE FOR, AS THE STATE STANDS (`continues`,
// the position's words: CONTINUES) — which is how a reader of a return whose `next` is
// `verify` tells VERIFICATION THAT IS OWED (`triage`, `verify`) from A RECORD THAT IS OWED AND
// NOTHING ELSE (`filed` alone: the invocation that follows launches nobody and records), and
// both from a stage of which nothing is left (none).
// WHAT IT DOES NOT SAY, because no step prints it: which findings are the human's — the
// digest counts his list (`for_the_human`, and `rule.findings` beside `rule`) — and which of
// them a verifier contests: the digest says nothing of that at all. Both are in the state
// document the return names, and in the view's file; a return's `verify.contested` counts
// what the verifiers' RETURNS of that invocation said, which nothing holds.
// A run without an order says none of it: its returns are what they were.
function queueSaid(state) {
  if (!state || !hasQueue(state)) return {}
  const queue = state.queue
  return {
    continues: continuing(state, state.position.test),
    budget: { limit: queue.budget, spent: queue.spent, left: queue.left },
    queue: { length: queue.length, hand_on: queue.hand_on, classes: queue.classes },
    view: state.view || null,
  }
}
// statusOf — WHAT AN INVOCATION OF THE `test` STAGE OF A RUN WITH A QUEUE SAYS HAPPENED (that
// plan's T7; the row `wf-return-names`: `triaged` was said by a stage whose triage was not
// finished). ONE OF THREE WORDS, EACH FOR WHAT THE INVOCATION ITSELF PUT ON RECORD:
//   hunted     A STAGE — it began a round, or ran an item again: its instruments ran, its one
//              triage graded, and its findings are on record (`record`). It made no record of
//              a verify phase: the queue printed no key or the budget had nothing left
//              (`verify.waves` is 0 — it had nothing to verify), or every verifier it
//              launched left neither a report nor a verdict.
//   verified   THE RECORD OF THE VERIFY PHASE WAS MADE BY THIS INVOCATION (`verify.record`) —
//              a stage's second record, or the record of an invocation that continued one:
//              what was filed is in a commit. `verify.verdicts` says how many verdicts the
//              state counts since the phase began, and may say none: a verifier's report
//              with no verdict is recorded too, and its launch counted; and a continuation
//              that was handed only a filed verdict (`continued`: `filed`) launched nobody.
//   continued  AN INVOCATION THAT CONTINUED A STAGE AND MADE NO RECORD OF A VERIFY PHASE. It
//              hunted nothing — no instrument ran — so it does not say that it did: it
//              graded rows nobody had graded and recorded those (`record`), or its verifiers
//              left nothing, or it had nothing to record at all.
// `words` is what the invocation continued the stage for (none: it is a stage); `second`,
// the commit of the record it made after its verify phase, or null.
function statusOf(words, second) {
  return second ? 'verified' : words.length ? 'continued' : 'hunted'
}

// readStep — a git step's return as the object its command printed. The relayed line must
// end with the sha256 of itself without that field, as the tool writes it (dev/stabilize-step:
// "The line"), parse as JSON and be the line of the act that was asked for; then it is what
// the step says, a refusal included (`status: 'halted'`, with the word it refused with). A
// line that is not the command's is a halted step too, and `relay` says why — and never
// repeats the line: what came back is not passed on. A step whose agent halted without a
// line is passed on as that halt.
const STEP_SHA_RE = /, "sha256": "([0-9a-f]{64})"\}$/
// lostStep — a step whose line is not one this script reads, as a halted step that says why.
function lostStep(r, why) {
  return { status: 'halted', relay: why, halt: { root_cause: why, evidence: typeof r.line === 'string' ? '(a line of ' + r.line.length + ' characters came back; it is not passed on)' : '(the step returned no line)', tree_state: '(not read: the step\'s line was not the command\'s)', recommendation: 'read the tree and the branches before the step is asked for again: its command may have run' } }
}
// `exact`: the line is read only as it came back, never mended (`mendSlip`) — what a probe
// measures is the relay itself.
function readStep(r, act, exact) {
  return stepRead(r, act, exact).said
}
function stepRead(r, act, exact) {
  if (!r) return { said: null }
  if (r.status !== 'ran') return { said: { status: 'halted', halt: r.halt || null } }
  const lost = (why) => ({ said: lostStep(r, why) })
  if (typeof r.line !== 'string') return lost('the step returned no line')
  const held = asPrinted(r.line, exact)
  if (!held) return lost('the relayed line does not end with the sha256 of itself, as `' + STEP_TOOL + '` prints it: it was altered on its way, or is not the command\'s')
  let said = null
  try {
    said = JSON.parse(held.line)
  } catch (e) {
    return lost('the relayed line is not JSON: ' + ((e && e.message) || e))
  }
  if (!plain(said) || said.act !== act || typeof said.status !== 'string') return lost('the relayed line is not the line of `' + STEP_TOOL + ' ' + act + '`')
  return { said, line: held.line, mended: held.mended }
}
// asPrinted — the relayed line AS THE TOOL PRINTED IT, or null: the line itself where it
// hashes to the sha256 it ends with; else, unless `exact`, the line with ONE value put back
// (`mendSlip`) where that makes it hash — and then `mended` says which. Null where neither
// holds: the line is not the command's, and nothing of it is read.
function asPrinted(relayed, exact) {
  const line = relayed.replace(/\n$/, '')
  const tail = STEP_SHA_RE.exec(line)
  if (!tail) return null
  const body = line.slice(0, tail.index)
  if (sha256(body + '}') === tail[1]) return { line, mended: null }
  const mended = exact ? null : mendSlip(body, tail[1])
  return mended ? { line: mended.body + line.slice(tail.index), mended: mended.field } : null
}
// mendSlip — A RELAYED LINE WITH ONE VALUE SLIPPED, PUT BACK BY ITS OWN HASH (the staged
// canary's part 1, 2026-10-16). A line is relayed by an agent that copies it, and a copy
// slips where the line repeats itself: in four rows of the state's clause table, identical
// for sixty characters before each one's `retry`, the second row's `never-selected` came back
// as the `no-instrument` of the rows below it — on three tries of three, the same slip, so a
// read asked for again failed alike and the stage halted. So: FOR EACH FIELD OF THE LINE THAT
// HOLDS A PLAIN VALUE, EACH OTHER VALUE THE LINE HOLDS UNDER THAT FIELD'S NAME is put in its
// place, and the line that hashes to the sha256 it ends with is the line. NOTHING AN AGENT
// SAID IS TRUSTED BY IT: the line that hashes IS the tool's — a line of other bytes with that
// hash is not to be found — so a mended line is read exactly as one that came back whole.
// WHAT IT DOES NOT MEND, and leaves to the read asked for again and then the halt: two
// slips; a value that stands nowhere else in the line under its field's name; a name, a
// bracket, a hash or a character struck or added. At most MEND_TRIES lines are hashed.
const MEND_TRIES = 4096
function mendSlip(body, want) {
  const fields = Array.from(body.matchAll(/"([^"\\]+)":("[^"\\]*"|-?\d+|true|false|null)(?=[,}\]])/g), (m) => ({ key: m[1], value: m[2], at: m.index + m[0].length - m[2].length }))
  const under = {}
  for (const f of fields) {
    if (!Object.prototype.hasOwnProperty.call(under, f.key)) under[f.key] = []
    if (!under[f.key].includes(f.value)) under[f.key].push(f.value)
  }
  let tries = 0
  for (const f of fields) {
    for (const value of under[f.key]) {
      if (value === f.value) continue
      if (++tries > MEND_TRIES) return null
      const text = body.slice(0, f.at) + value + body.slice(f.at + f.value.length)
      if (sha256(text + '}') === want) return { body: text, field: { key: f.key, relayed: f.value, printed: value } }
    }
  }
  return null
}
// readDigest — a step of a STAGE, read: `readStep`, and then THE LINE IS A DIGEST or it is
// not read (the header: WHAT THIS SCRIPT READS). A digest is printable ASCII and holds no
// backslash — the tool prints it without one escape, so a relay has nothing to decode — and
// it says what it leaves out: `unfit`, the fields whose value did not fit its shape, and
// `file`, where the rest lies. So the line the tool prints UNASKED — the state document or a
// refusal's prose in it — is refused here even where it hashes, as a line that is not the
// one asked for; and a digest with a field struck out is a step this script does not act
// on: by then the act may have run, and its own line, in the file, says what it did.
const DIGEST_RE = /^[\x20-\x7e]*$/
// A digest whose one value was put back (`mendSlip`) is read, and a step's `label` has the
// log say which field came back as what.
function readDigest(r, act, label) {
  const read = stepRead(r, act)
  const said = read.said
  if (!said || r.status !== 'ran' || said.relay) return said
  const line = read.line
  if (!Array.isArray(said.unfit) || !('file' in said) || !('file_sha256' in said)) return lostStep(r, 'the relayed line is no digest of `' + STEP_TOOL + ' ' + act + '` — it names neither what did not fit nor the file that holds the rest: it is the line the tool prints unasked, or another\'s')
  if (!DIGEST_RE.test(line) || line.includes('\\')) return lostStep(r, 'the relayed line holds a backslash or a character outside printable ASCII, and a digest of `' + STEP_TOOL + '` holds neither: it was altered on its way, or is not the digest that was asked for')
  // (Both values are the digest's own: the one that came back stands in it under the same name.)
  if (read.mended && label) log('the relayed line of `' + label + '` came back with ONE value altered — `' + read.mended.key + '` as ' + read.mended.relayed + ' where the tool printed ' + read.mended.printed + ' — and is read as the tool printed it: that value put back, the line hashes to the sha256 it ends with')
  if (said.unfit.length) return Object.assign(lostStep(r, 'the digest of `' + STEP_TOOL + ' ' + act + '` could not print ' + said.unfit.filter((name) => typeof name === 'string' && /^[a-z0-9_.-]+$/.test(name)).join(', ') + ': a value did not fit the shape its field declares, and this script acts on no line with a field struck out. THE ACT MAY HAVE RUN — its own line, whole, is in the file the digest names'), { unfit: true, said: said.status, file: said.file, file_sha256: said.file_sha256 })
  return said
}
// heldOf — what a `hold-start` or a `hold-wait` step's digest says of a held command, as one
// of HELD_ENDS: `running`; `green`, `red` or `void` — the tool's verdict of a command that
// ran to its end, with the file the tool kept it in, by its path and its hash, and `why`
// where it is void; `dead`; or `unread`, where the step did not answer with a job. THE
// VERDICT IS THE TOOL'S: nothing here judges, and no word of an agent is read.
function heldOf(step, scratch) {
  // (`silent`: the step returned NOTHING — what a halt that may be the account's limit is
  // said of, and what a step that refused, or a command that still runs, is not.)
  if (!step) return { ends: 'unread', why: 'the step returned no result', silent: true }
  if (step.status === 'halted') return { ends: 'unread', why: step.refused ? stepRefusal(step.refused) : step.relay || 'the step halted', refused: step.refused || null }
  // `fresh`: THIS call started the command — the tool answers `started` once per name, and
  // any other answer of a start is a job that was there before it asked.
  const job = { name: step.name, kind: step.kind, output: isText(step.output) ? scratch + '/' + step.output : null, fresh: step.status === 'started' }
  if (step.status === 'started' || step.status === 'running') return Object.assign({ ends: 'running' }, job)
  if (step.status === 'dead') return Object.assign({ ends: 'dead', why: step.why || null }, job)
  if (step.status === 'done' && ['green', 'red', 'void'].includes(step.verdict) && isText(step.verdict_file) && SHA256_RE.test(String(step.verdict_sha256))) return Object.assign({ ends: step.verdict, why: step.why || null, verdict: scratch + '/' + step.verdict_file, verdict_sha256: step.verdict_sha256, facts: plain(step.facts) ? step.facts : {} }, job)
  return Object.assign({ ends: 'unread', why: 'the step said `' + step.status + '` and named no verdict of the tool\'s' }, job)
}
// passedOver — WHETHER THE JOB THAT ANSWERED A START IS PASSED OVER FOR THE NEXT NAME — and by
// which of the two reasons — or null, where it is the job this invocation holds (`hold`).
// WHAT A START'S ANSWER SAYS OF WHO STARTED THE JOB (dev/stabilize-step: `hold-start`):
// `started` — THIS call started it (`fresh`) — and with every other answer NOTHING: the job
// was there, and the line names no call, no invocation and no moment this script could hold
// it to (the job's record holds when its start wrote it; no line prints that, and this
// script has no clock). Until the scoped review before the parts (its finding 1 in this
// script) THE COUNT OF THE STEP'S TRIES stood in for that: what answered a start's RETRY was
// taken as the job a dead first try of this invocation had started. It is that only
// sometimes. Under a name two invocations share — a record's gate of the rulings of one
// round (`record-rulings-r<N>`), of a pending batch (`record-pending`), and whatever stands
// under a commit's name — it is as well AN EARLIER INVOCATION'S, and then a record was
// committed and pushed behind a gate that never ran on its tree, or a round's record named a
// red nobody measured in that attempt. SO THE ANSWER'S OWN WORD DECIDES, AND THE TRIES ONLY
// IN THE ONE CELL WHERE NO WORD CAN:
//   started (`fresh`)                this call's: the answer.
//   an END — green, red, void, dead  NEVER the answer to a job the invocation must run itself,
//                                    WHATEVER THE TRIES (`ended`): for a record's gate
//                                    (`sequence`) every end; for what is named by its commit
//                                    (`stands`) every end BUT GREEN — a green build, gate or
//                                    check of that commit is the commit's, whoever asked.
//   running                          `stands`: the answer — its wait is taken over.
//                                    `sequence`: on the start's FIRST try an earlier
//                                    invocation's, and passed over (`running`); on a RETRY the
//                                    job the dead try started, and the answer — the cell the
//                                    tries still decide, so that a gate of this tree that runs
//                                    is not started a second time.
//   unread                           no job answered: nothing is passed over.
// THE COST is one more gate, build or check where a dead try's own job had ENDED before its
// step was asked again — and a gate that ends inside a retry is one that did not run. WHAT
// THE ONE CELL LEAVES OPEN: a gate of an earlier invocation that STILL RUNS under a shared
// name when a start's first try returns nothing is taken as this one's. Under
// `record-pending` that is a gate of the same applied batch; under `record-rulings-r<N>` it
// takes a batch that was taken back while its gate ran.
function passedOver(job, at, tries) {
  if (at.ends === 'unread' || at.fresh) return null
  if (at.ends === 'running') return job.sequence && tries < 2 ? 'running' : null
  if (job.sequence) return 'ended'
  return job.stands && at.ends !== 'green' ? 'ended' : null
}

// resultRows — what a `test` stage records of its items: ONE RESULT PER ITEM it ran, and
// nothing about a clause — a clause's status is derived by the record script from these
// rows, and no code of this script composes one. `void` with its reason when the item did
// not run to its end; `red` for a deterministic check that is red — which the record script
// files as a finding in the same call, so every red brings what that ledger row needs: a
// doctype, the door it stands at — the check's own name: which door of the round a red
// check stands at is triage's to say, from the item's row, and no text of that row passes
// through this script — and where the
// evidence lies; else `green`: the item ran, on this commit, over the round's doors. What a
// hunting item FOUND is the ledger's, and forbids closing there. A HELD CHECK'S ROW IS THE
// FILE THE TOOL KEPT ITS VERDICT IN, AND NO WORD (`unit.verdict`, from `heldOf`): the record
// script reads green, red or void out of that file, and refuses an outcome a caller names.
function resultRows(units) {
  return units.map((unit) => {
    if (unit.verdict) return { item: unit.item, verdict: unit.verdict }
    if (unit.status === 'void') return { item: unit.item, outcome: 'void', reason: unit.reason || 'it did not run to its end' }
    if (unit.status === 'red') return { item: unit.item, outcome: 'red', doctype: RED_DOCTYPE, door: 'the check `' + unit.item + '`', repro: unit.evidence || 'the check returned red, and no evidence beside it' }
    return { item: unit.item, outcome: 'green' }
  })
}

// classifyCommits — a round's commits, by what a carry-over may take: a commit confined to
// the run's directory is a record; one that touches nothing in it is a fix; one that does
// both, or neither, is neither — and stops the step that asked.
function classifyCommits(all, inside, outside) {
  const records = []
  const fixes = []
  const mixed = []
  for (const sha of all) {
    const i = inside.includes(sha)
    const o = outside.includes(sha)
    if (i && !o) records.push(sha)
    else if (o && !i) fixes.push(sha)
    else mixed.push(sha)
  }
  return { records, fixes, mixed }
}
function sameCommit(x, y) {
  return typeof x === 'string' && typeof y === 'string' && x.length >= 7 && y.length >= 7 && (x.startsWith(y) || y.startsWith(x))
}
// reopenPatches — the ledger patches of a dropped round, or of the fixes a part leaves out:
// a row whose `fixed` names one of `gone` is open again, because that commit never lands.
function reopenPatches(ledger, gone) {
  return ledger.filter((row) => row.disposition === 'fixed' && gone.some((sha) => sameCommit(sha, row.detail))).map((row) => ({ key: row.key, disposition: 'open' }))
}
// repointPatches — a part's kept fixes were cherry-picked, so their rows name the new commits.
function repointPatches(ledger, picked) {
  const out = []
  for (const row of ledger) {
    const pick = row.disposition === 'fixed' ? picked.find((p) => sameCommit(p.from, row.detail)) : null
    if (pick) out.push({ key: row.key, disposition: 'fixed', detail: pick.to })
  }
  return out
}

// areasOf — lever 8, how `fix` partitions: the blockers by the registry their door comes
// from in the round's scope, in the scope's order; a door the scope does not list is an
// area of its own. One fixer per area, serial.
function areasOf(blockers, doors) {
  const rows = doors ? doors.included.concat(doors.excluded) : []
  const areas = []
  for (const finding of blockers) {
    const at = rows.find((d) => d.door === finding.door)
    const registry = at ? at.registry : '(a door the round\'s scope does not list)'
    let area = areas.find((x) => x.registry === registry)
    if (!area) areas.push(area = { registry, findings: [] })
    area.findings.push(finding)
  }
  return areas.map((area, i) => Object.assign({ n: i + 1 }, area))
}

// currentCut — which of a round's branches is its branch now: the highest cut that exists.
function currentCut(run, round, local) {
  let cut = 0
  for (let c = 1; c <= MAX_CYCLES; c++) if (local.includes(branchName(run, round, c))) cut = c
  return cut
}

// What a stage's triage becomes in the record: the new rows, and the round's triage entries.
function triageRecord(ctx, entries) {
  return {
    rows: entries.filter((e) => e.new).map((e) => ({ key: e.key, doctype: e.doctype, round: ctx.round, source: e.source, door: e.door, clause: e.clause, repro: e.repro })),
    triage: entries.map((e) => {
      const t = { key: e.key, grade: e.grade }
      if (e.grade === 'out-of-scope') t.bound = e.bound
      if (e.verdict) t.verdict = e.verdict
      // A regression fact that is UNKNOWN is the field LEFT OUT — the record's cell for a
      // confirmed finding nobody could say it of — and never `false`; NOT_COMPARABLE, a door
      // the previous release does not have, is a fact and goes as it is.
      if (e.verdict === 'confirmed' && (typeof e.regression === 'boolean' || e.regression === NOT_COMPARABLE)) t.regression = e.regression
      // A fork rides on the verdict that raised it, and is a row a later invocation reads.
      if (e.verdict && e.fork) t.fork = e.fork
      return t
    }),
  }
}

// recordThen — what a halt of a record step says is to be done, in place of the sentence
// every other halt ends with: WHAT THE NEXT INVOCATION DOES, by how the step ended. `word`
// is what the commit step refused with, or null where the step never answered — the
// executor's, or the commit step's own agent: then nothing is known of the batch, and the
// sentence is true whether or not it was applied.
function discardCommand(run, branch) {
  return '`' + STEP_TOOL + ' discard --branch ' + branch + ' --run-dir ' + runDir(run) + '`'
}
function recordThen(run, branch, word, discarded) {
  const stands = ' What was committed stands.'
  const takeBack = ' To take the batch back instead — the orchestrator\'s decision, a subagent\'s act — ' + discardCommand(run, branch) + ' asks git first, puts every table back as it was before the record step, and keeps the reports and the round\'s scope.'
  if (word === 'gate-red') return stands + ' THE BATCH IS STILL APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree. Invoke the stage again: that invocation runs the full gate on the batch once more, commits and pushes it, and does nothing else.' + takeBack
  if (word === 'unvetted') return stands + (discarded === true ? ' THE BATCH IS TAKEN BACK ALREADY, and a report or a scope that was no such file has left the tree: nothing is pending, and no discard is asked for. Invoke the stage again: it reads the state as it stands and works the next attempt.' : ' The batch was NOT taken back — a file of it is in a commit already — and it is the human\'s: the step\'s own line says which file.')
  if (word === 'fence-moved') return stands + ' Nothing was committed or undone: THE BATCH IS STILL APPLIED — AND SOMETHING WROTE THE REPOSITORY PAST THE WRITE GUARD while the stage ran: its configuration, a hook, an info file, a local ref or a reflog\'s newest entry moved. It is the human\'s: what moved is found and put back first. Then invoke the stage again: its first read takes the repository as it then stands, gates the batch and commits it.' + takeBack
  if (word === 'head-moved') return stands + ' Nothing was committed or undone: THE BATCH IS STILL APPLIED. Invoke the stage again: its first read meets the commit somebody made while the stage ran — one the remote lacks is its author\'s to push — then gates the batch on the tree as it now stands, and commits it there.' + takeBack
  if (word === 'no-gate') return stands + ' THE BATCH IS STILL APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree: its gate left no verdict, and no commit step ran. Invoke the stage again: that invocation holds the full gate on the batch once more, under the next name, commits and pushes it, and does nothing else.' + takeBack
  // NO NAME IS LEFT FOR THE GATE OF A PENDING BATCH (`hold`: `exhausted`). Such a gate is held
  // under `record-pending-<n>` whichever batch it is, a name is one command's for the life of
  // a scratch root, and there are HOLD_NAMES of them: an invocation that finds them all taken
  // would find them taken again. So this halt does NOT say that the next invocation gates the
  // batch under the next name — there is none — and names the exit that works.
  if (word === 'no-name') return stands + ' THE BATCH IS STILL APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree — AND NO NAME IS LEFT TO HOLD ITS GATE UNDER: the gate of a pending batch is held under one of ' + HOLD_NAMES + ' names for the life of a scratch root, a name is one command\'s, and every one of them holds a command already. INVOKING THE STAGE AGAIN CHANGES NOTHING: that invocation asks for the same names, and halts here. THE EXIT THAT WORKS is the batch taken back — the orchestrator\'s decision, a subagent\'s act: ' + discardCommand(run, branch) + ' asks git first, puts every table back as it was before the record step, and keeps the reports and the round\'s scope — and a filed verdict, which is no file of a batch. The stage invoked after that reads the state as it stands: what the batch recorded is recorded again by the step that owes it — a stage\'s record under its attempt\'s own name, the human\'s rulings by the invocation that carries them.'
  if (word === 'did-not-run' || word === 'locked') return stands + ' Everything is as it was: THE BATCH IS STILL APPLIED. Once what the refusal names is dealt with, invoke the stage again: that invocation gates the batch once more, commits and pushes it, and does nothing else.' + takeBack
  return stands + ' A record step that stopped may have left its batch APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree. Invoke the stage again: its first read says what is there. Where a batch is pending, that invocation runs the full gate on it once more, commits and pushes it, and does nothing else; where the commit was made and nothing else, it settles and pushes it; where none is, it reads the state as it stands and works the next attempt.' + takeBack
}
// The word the record script refused a batch with that means THE SAME BATCH, SENT TWICE:
// every file it would write is what the run holds already. Nothing is written, no journal
// exists, and there is nothing to take back.
const NO_CHANGE = 'no-change'
// pushThen — what a halt says after a record's push that failed: the record is committed,
// and WHO MAKES THE PUSH. It is owed, and the next invocation's first read makes it — but a
// push the tool refused `unvetted` is the human's and is never made by a later read, and
// one refused `foreign-commit` waits for a commit that is its author's to push.
function pushThen(branch, word) {
  if (word === 'unvetted') return ' What was committed stands, AND THE PUSH IS NOT OWED TO ANY LATER INVOCATION: a commit it would publish is not what `dev/stabilize-record` would have written, so every push of it is refused again — a stage\'s first read included — until the human has taken that commit back or rewritten it. `dev/stabilize-record vet --range -- ' + branch + ' --not --remotes=origin` names every commit and place.'
  if (word === 'foreign-commit') return ' What was committed stands. The record is not pushed beside a commit that is no record\'s: its author pushes that commit by name — `git push origin ' + branch + '` — and the stage is invoked again; its first read then makes the push that is owed.'
  return ' What was committed stands, and THE PUSH IS OWED: invoke the stage again with the same args — its first read finds the record the remote lacks, vets it and pushes it, and the stage goes on from the state as it then stands.'
}
// What a retry is told. An agent that works in the tree looks at what a dead try left
// before it goes on. A STEP OF THE TOOL IS ONE COMMAND, AND ITS RETRY IS THAT COMMAND, RUN
// AGAIN: the tool reconciles — it finishes what an earlier run of it left, answers again
// what it answered, or refuses and says whose the state is — and the agent looks at nothing.
const LOOK_AGAIN = 'RETRY after a transient failure of an earlier attempt at this same call. Before anything else look at what that attempt left — the tree\'s status, the last commit, and whether your report already stands at its path — and go on from it: never clean the tree (the untracked files under the run\'s directory are other agents\' reports), never make a commit or hand over a report that exists already. If you cannot tell what the dead attempt did, halt and say so.'
const RUN_AGAIN = 'RETRY: an earlier try of this same step returned nothing. Run the ONE command above again, exactly as it is written, and relay its line: the command says itself what an earlier run of it left — it finishes that, answers again what it answered, or refuses — and you look at nothing else and repair nothing.'
// What a state read is told when the line an earlier try relayed was not the command's.
function relayAgain(n) {
  return 'AGAIN (' + n + '): the line an earlier attempt returned was not the line the command printed. Copy the line from the command\'s output character for character.'
}

// ---- structured-output schemas ----
const HALT = {
  type: 'object',
  description: 'the halt report — fill every field, so that nobody needs your transcript',
  properties: {
    root_cause: { type: 'string' },
    evidence: { type: 'string', description: 'the command and its full output, or the refusal line verbatim' },
    tree_state: { type: 'string', description: 'the tree\'s status, the branch checked out, and which commits landed' },
    recommendation: { type: 'string' },
  },
}
const STRINGS = { type: 'array', items: { type: 'string' } }
// What a git step returns: the ONE line its command printed. What the line holds is the
// tool's to say (dev/stabilize-step: its header); `readStep` reads it.
const STEP_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['ran', 'halted'] },
    halt: HALT,
    line: { type: 'string', description: 'the ONE line the command printed on stdout, whole and as printed: every character copied, nothing re-ordered, summarised, re-indented or re-escaped' },
  },
}
// The one step that is still a list of commands: the re-cut of a part.
const CARRY_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['carried', 'halted'] },
    halt: HALT,
    branch: { type: 'string' },
    head: { type: 'string' },
    remote_head: { type: 'string' },
    picked: { type: 'array', description: 'one entry per commit carried, in order', items: { type: 'object', required: ['from', 'to'], properties: { from: { type: 'string', description: 'the full sha the step named' }, to: { type: 'string', description: 'the full sha of the commit the cherry-pick made' } } } },
  },
}
const PREFLIGHT_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['ready', 'halted'] },
    halt: HALT,
    image: { type: 'object', properties: { tag: { type: 'string' }, verified: { type: 'boolean' }, failed: STRINGS } },
    checks: { type: 'array', items: { type: 'object', required: ['check', 'status'], properties: { check: { type: 'string', description: 'the id of the item the prompt listed the check under' }, status: { type: 'string', enum: ['green', 'red', 'void'] }, commit: { type: 'string', description: 'the full sha the check ran on: a green or a red is evidence about that commit and no other' }, evidence: { type: 'string' } } } },
    report: { type: 'string', description: 'the path `dev/stabilize-record report` printed' },
  },
}
const DOORS = { type: 'array', items: { type: 'object', required: ['door', 'registry', 'derivation'], properties: { door: { type: 'string' }, registry: { type: 'string' }, derivation: { type: 'string' } } } }
const SCOPE_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['written', 'stands', 'halted'] },
    halt: HALT,
    base: { type: 'string', description: 'the full sha the round\'s change is measured from' },
    tip: { type: 'string' },
    included: DOORS,
    excluded: DOORS,
    registries: { type: 'array', items: { type: 'object', properties: { registry: { type: 'string' }, read_at: { type: 'string' }, doors: { type: 'number' }, included: { type: 'number' }, excluded: { type: 'number' }, whole: { type: 'boolean' } } } },
    by_unit: { type: 'array', items: { type: 'object', properties: { unit: { type: 'string' }, doors: STRINGS } } },
    uncovered: STRINGS,
    reached_but_excluded: STRINGS,
    left_open: STRINGS,
    scope: { type: 'string' },
    report: { type: 'string' },
  },
}
const FINDING = {
  type: 'object',
  required: ['id', 'title', 'door', 'clause', 'repro'],
  properties: {
    id: { type: 'string', description: 'your own id for it, unique in your report' },
    title: { type: 'string' },
    severity: { type: 'string', description: 'information for triage, never a filter' },
    door: { type: 'string', description: 'in the exact words of the door list you were handed, or named plainly and marked unlisted' },
    clause: { type: 'string' },
    class: { type: 'string' },
    count_derivation: { type: 'string' },
    repro: { type: 'string', description: 'the heading of its repro block in your report' },
    lead: { type: 'boolean', description: 'true for what you noticed and did not pursue' },
  },
}
const UNIT_SCHEMA = {
  type: 'object',
  required: ['status', 'findings'],
  properties: {
    status: { type: 'string', enum: ['reported', 'halted'] },
    halt: HALT,
    findings: { type: 'array', items: FINDING },
    doors_affected: Object.assign({ description: 'when you were handed a fix diff: every door it can change' }, STRINGS),
    asserted_sha256: { type: 'string', description: 'when your prompt carries a binary line: the sha256 you asserted for the candidate before you drove it' },
    report: { type: 'string', description: 'the path `dev/stabilize-record report` printed' },
    summary: { type: 'string', description: '2-4 sentences' },
  },
}
// The `required` probe's: a unit's return, with the hash REQUIRED — the one thing about a
// schema the probe is there to see the runtime's answer to. No stage uses it.
const PROBE_UNIT_SCHEMA = Object.assign({}, UNIT_SCHEMA, { required: UNIT_SCHEMA.required.concat(['asserted_sha256']) })
const TRIAGE_SCHEMA = {
  type: 'object',
  required: ['status', 'entries', 'counts'],
  properties: {
    status: { type: 'string', enum: ['graded', 'halted'] },
    halt: HALT,
    entries: {
      type: 'array',
      items: {
        type: 'object',
        required: ['key', 'new', 'doctype', 'source', 'door', 'clause', 'repro', 'grade'],
        properties: {
          key: { type: 'string' },
          new: { type: 'boolean', description: 'true for a key you minted; false for a finding found again, whose row the ledger holds' },
          doctype: { type: 'string', enum: ['jigc-feedback', 'inconsistency'] },
          source: { type: 'string' },
          door: { type: 'string' },
          clause: { type: 'string' },
          repro: { type: 'string' },
          grade: { type: 'string', enum: ['breaks', 'unclear', 'no-break', 'out-of-scope', 'needs-bound'] },
          bound: { type: 'string', description: 'with out-of-scope only: the row of the declared-bounds list it cites' },
          why: { type: 'string' },
        },
      },
    },
    to_verify: { type: 'array', items: { type: 'object', required: ['key'], properties: { key: { type: 'string' }, redrive: { type: 'string', description: 'what the verifier must re-drive' } } } },
    counts: {
      type: 'object',
      required: ['findings_in', 'entries', 'merged'],
      properties: {
        findings_in: { type: 'array', items: { type: 'object', required: ['reporter', 'count'], properties: { reporter: { type: 'string' }, count: { type: 'number' } } } },
        entries: { type: 'number' },
        new: { type: 'number' },
        found_again: { type: 'number' },
        merged: { type: 'number', description: 'findings folded into another entry, each merge named in your report' },
      },
    },
    report: { type: 'string' },
  },
}
// What THE ONE TRIAGE OF A RUN WITH A QUEUE returns: no entry — its entries are one file, handed
// to the record script by the call its prompt spells — and of the ONE line that call printed,
// what `filedFault` holds. Nothing but the status is required: a return that omits a
// required field is a call that failed, and what is missing here is that triage's ending.
const FILED_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['filed', 'halted'] },
    halt: HALT,
    report: { type: 'string', description: 'the path the call printed as `triage`: your report, rendered from your entries' },
    sha256: { type: 'string', description: 'the hash that call printed beside the path, as `sha256`' },
    handed: { type: 'array', description: 'what that call printed as `handed`: one entry a source, with the count the call named for it', items: { type: 'object', required: ['source', 'count'], properties: { source: { type: 'string' }, count: { type: 'number' } } } },
  },
}
const VERIFY_SCHEMA = {
  type: 'object',
  required: ['status', 'key'],
  properties: {
    status: { type: 'string', enum: ['verified', 'halted'] },
    halt: HALT,
    key: { type: 'string' },
    verdict: { type: 'string', enum: ['confirmed', 'refuted'] },
    regression: { enum: [true, false, 'not-comparable'], description: 'with confirmed only: true where the repro is green on the previous release, false where it is red there too, not-comparable where it cannot run there because its door is one that release does not have' },
    basis: { type: 'string' },
    contested: { type: 'boolean' },
    asserted_sha256: { type: 'string', description: 'the sha256 you asserted for the candidate before you drove it' },
    ran_on: { type: 'object', properties: { candidate: { type: 'string', description: 'the sha256 you asserted for the candidate' }, previous: { type: 'string', description: 'the sha256 you asserted for the previous release, where step 4 ran' } } },
    repro: { type: 'string' },
    pinnable: { type: 'boolean' },
    left_open: STRINGS,
    report: { type: 'string' },
  },
}
// What A VERIFIER OF A RUN WITH A QUEUE returns: NO VERDICT — its verdict is one file, filed by
// the call its prompt spells — and of the ONE line that call printed, what `verdictFault`
// holds. Nothing but the status is required, and NO FIELD OF IT IS NEEDED BY THE STAGE (the
// fence, arm (t)): what is filed decides, and a return that lacks a field is named and
// nothing else.
const VERDICT_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['filed', 'halted'] },
    halt: HALT,
    filing: { type: 'string', description: 'the path the call that filed your verdict printed as `verdict`' },
    word: { type: 'string', description: 'the verdict that call printed as `word`: confirmed, refuted or undrivable' },
    contested: { type: 'boolean', description: 'what that call printed as `contested`: true where your filing gave a basis, false where it gave none' },
    asserted_sha256: { type: 'string', description: 'the sha256 you asserted for the candidate before you drove it' },
    report: { type: 'string', description: 'the path the call that handed your report over printed' },
  },
}
const DRIVEN = { type: 'array', items: { type: 'object', required: ['step', 'command', 'result'], properties: { step: { type: 'string' }, command: { type: 'string' }, result: { type: 'string' } } } }
const ADVOCATE_SCHEMA = {
  type: 'object',
  required: ['status', 'verdict', 'case'],
  properties: {
    status: { type: 'string', enum: ['argued', 'halted'] },
    halt: HALT,
    verdict: { type: 'string', enum: ['robust-now', 'cheap-cut-is-correct'] },
    case: { type: 'string', description: 'the case, whole: the robust position, the tells, the priced cost, the robust scope' },
    proposal: { type: 'string', description: 'what you propose the human accept, as a change somebody else could apply to a clone of the candidate\'s commit' },
    driven: DRIVEN,
    undriven: STRINGS,
    asserted_sha256: { type: 'string' },
    left_open: STRINGS,
    report: { type: 'string' },
  },
}
const PROPOSAL_SCHEMA = {
  type: 'object',
  required: ['status', 'holds', 'steps'],
  properties: {
    status: { type: 'string', enum: ['driven', 'halted'] },
    halt: HALT,
    holds: { type: 'boolean', description: 'true only when every next step ran as the proposal says it does' },
    steps: { type: 'array', items: { type: 'object', required: ['step', 'command', 'result', 'agrees'], properties: { step: { type: 'string' }, command: { type: 'string' }, result: { type: 'string' }, agrees: { type: 'boolean', description: 'whether what you observed is what the advocate reported for this step' } } } },
    undriven: STRINGS,
    asserted_sha256: { type: 'string' },
    left_open: STRINGS,
    report: { type: 'string' },
  },
}
const FIXER_SCHEMA = {
  type: 'object',
  required: ['status', 'entries'],
  properties: {
    status: { type: 'string', enum: ['worked', 'halted'] },
    halt: HALT,
    entries: {
      type: 'array',
      items: {
        type: 'object',
        required: ['key', 'status'],
        properties: {
          key: { type: 'string' },
          status: { type: 'string', enum: ['fixed', 'could-not-fix', 'halted'] },
          commit: { type: 'string', description: 'the full sha' },
          gate: { type: 'string', description: 'the totals line and the GATE: PASS of the gate that preceded that commit' },
          notes: { type: 'string', description: 'on could-not-fix: the trace showing the finding is not real, or what stood in the way' },
          class: { type: 'string' },
          count_derivation: { type: 'string' },
          doors_affected: STRINGS,
          left_open: STRINGS,
          halt: HALT,
        },
      },
    },
    report: { type: 'string' },
  },
}
// What the record step's executor returns: that the batch is applied. It runs no gate,
// makes no commit and returns no evidence of its own: the gate is a command the step tool
// holds, and the commit step reads the batch and the gate's output itself.
const RECORD_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['applied', 'halted'] },
    halt: HALT,
    refused: { type: 'string', description: 'when the batch of step 2 was refused: the ONE word that follows `refused` on the line `dev/stabilize-record` printed on stderr — `no-change`, `check`, `hygiene`, …' },
  },
}

// ---- the git steps (build-git) — each ONE command of dev/stabilize-step, its line relayed ----
const STEP_RULES = 'Run exactly the ONE command below, from the repository\'s root, typed as it stands, and nothing else: no command before it, after it or around it, no branch, no commit, no file edit, no stash, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. The command does the whole step and checks it. It prints ONE line of JSON on stdout — also when it refuses, which it says with an exit status that is not 0 and one line on stderr. A refusal is the step\'s answer: you never repair what it names, never run the command a second time, and never do by hand what it did not do.'
// ASK_RULES — the rules of THE ONE STEP THAT RUNS ITS COMMAND MORE THAN ONCE: the step that
// asks what became of a held command (`hold-wait`). The tool answers within a time of its
// own, shorter than the two minutes an agent's shell gives a call, and says `running` while
// the command has not ended: the agent ASKS AGAIN — that same plain call, and nothing in
// between — for as long as its turn lasts. It composes no shell around it and keeps no
// time of its own: what an agent improvised there is what stopped a stage on a prompt
// nobody was watching (the human's ruling of 2026-10-07).
const ASK_RULES = 'Run exactly the ONE command below, from the repository\'s root, typed as it stands, and nothing else: no command before it, after it or around it, no branch, no commit, no file edit, no stash, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. It asks the tool what became of a long command the tool itself started and holds, and it answers by itself within a minute and a half, with ONE line of JSON on stdout — also when it refuses, which it says with an exit status that is not 0 and one line on stderr. WHILE THAT LINE\'S `status` IS `running`, RUN THE SAME COMMAND AGAIN, typed as it stands, as often as it takes: the long command can take the better part of an hour, and asking again is the whole of this step — nothing else is run in between, and you keep no time of your own. The first line whose `status` is anything else — `done`, `dead`, or a refusal — is the step\'s answer: you never repair what it names, and never start the long command yourself.'
// AS_PRINTED — how a line is relayed, in the words every step that relays one is told.
const AS_PRINTED = 'the ONE line the command printed on stdout, WHOLE and AS PRINTED, whatever it exited with: every character copied, no key re-ordered or dropped, nothing summarised, no `\\u…` escape rewritten into its character'
function stepPrompt(what, act, flags, asks) {
  return [
    'GIT STEP — ' + what + '. ' + (asks ? ASK_RULES : STEP_RULES),
    '1. ' + typed(STEP_TOOL, act + ' ' + flags),
    '2. Report status = ran and line = ' + AS_PRINTED + (asks ? ' — of the LAST run of it: the first line whose `status` is not `running`' : '') + '. The harness holds the line to the hash it ends with, so a line that is not the command\'s is caught. Only when the command printed no such line: status = halted, and the halt report (root_cause, evidence = the command and everything it printed, tree_state = the tree\'s status and the branch checked out).',
  ].join('\n')
}
// THE HELD COMMANDS — a long command is STARTED by one step and ASKED AFTER by another: two
// plain calls of the tool, each its own agent's, so that the agent that asks never starts
// and a step asked for again never starts twice. `flags` are what the kind takes
// (dev/stabilize-step: `hold-start`).
function holdStartPrompt(v, name, kind, flags, what) {
  return stepPrompt('START ' + what + ' as a command the tool holds (`' + name + '`): the tool starts it in a session of its own and answers at once — the command runs on, and another step asks after it', 'hold-start', '--scratch ' + v.scratch + ' --name ' + name + ' --kind ' + kind + ' ' + flags + ' ' + digestFlag(v.scratch))
}
function holdWaitPrompt(v, name, what) {
  return stepPrompt('what became of ' + what + ', a command the tool holds (`' + name + '`)', 'hold-wait', '--scratch ' + v.scratch + ' --name ' + name + ' ' + digestFlag(v.scratch), true)
}
// A list the harness owns, as the flags that hand it to an act.
function listFlags(flag, values) {
  return values.map((value) => '--' + flag + ' ' + value).join(' ')
}
// digestFlag — what makes a step's ONE line its digest, and names the root the file with the
// rest is kept under: the invocation's scratch root. Every step of a stage carries it.
function digestFlag(scratch) {
  return '--digest ' + scratch
}
// `look`: the invocation only looks (`stopAfter: 'state'`) — the read finishes nothing, and
// says what is owed.
function gitStatePrompt(v, look) {
  return stepPrompt((look ? 'LOOKED AT ONLY — nothing is finished, and what is owed is said: ' : '') + 'the state of the stabilization run `' + v.run + '` before its `' + v.stage + '` stage: the branch, the tree, the pushed loop branch, and the path-class assert', 'git-state', '--stage ' + v.stage + ' --loop ' + branchName(v.run) + ' --rounds ' + branchName(v.run, '') + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + (look ? ' --look' : '') + ' ' + digestFlag(v.scratch))
}
function statePrompt(v, tag) {
  return stepPrompt('read the record of the stabilization run `' + v.run + '` (read ' + tag + ')', 'state', '--run ' + v.run + ' --scratch ' + v.scratch + ' --tag ' + tag + ' ' + digestFlag(v.scratch))
}
// The BEGIN step: an attempt of a stage put on record before any agent of it is launched.
function beginPrompt(ctx, sha) {
  return stepPrompt('attempt ' + ctx.attempt + ' of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', begun on record: the state must hand the stage exactly this round and attempt, and the attempt\'s marker is written', 'begin', stageFlags(ctx) + ' --attempt ' + ctx.attempt + ' --reporter ' + ATTEMPT + ' --commit ' + sha + ' --scratch ' + ctx.scratch + ' ' + digestFlag(ctx.scratch))
}
// `names` null: THE VET ALONE — the report check's first half (dev/stabilize-step:
// `check-reports`, WITHOUT A STAGE): every file of the run that is written once and in no
// commit is vetted, and one that the record script would not have written is moved out of the
// tree. What an invocation asks for before a record that names no stage's reporters — the
// batch of the human's rulings. The call then names the run and the scratch root, and no
// round, no stage, no attempt and no reporter: `ctx` is the invocation's arguments.
function checkReportsPrompt(ctx, names) {
  const vet = names == null
  return stepPrompt(vet ? 'what of the stabilization run `' + ctx.run + '` is written once and in no commit, VETTED before a record that names no stage\'s reporters — the batch of the human\'s rulings: a file there that the record script would not have written leaves the tree' : 'the reports of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', held to the reporters that were launched', 'check-reports', (vet ? '--run ' + ctx.run : stageFlags(ctx) + ' --attempt ' + ctx.attempt) + ' --scratch ' + ctx.scratch + ' ' + digestFlag(ctx.scratch) + (vet ? '' : ' -- ' + names.join(' ')))
}
function findRoundPrompt(v, round) {
  return stepPrompt('find the branch of round ' + round + ' of `' + v.run + '`', 'find-round', '--prefix ' + branchName(v.run, round) + ' ' + digestFlag(v.scratch))
}
function openRoundPrompt(v, round, branch) {
  return stepPrompt('the branch `' + branch + '` of round ' + round + ' of `' + v.run + '`: switch to it, or open it from `' + branchName(v.run) + '`', 'open-round', '--loop ' + branchName(v.run) + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + digestFlag(v.scratch))
}
// `record`: the push of a RECORD on the loop branch — held to publishing records and nothing
// beside them (`--run-dir`): a commit outside the run's directory is its author's to push.
function pushPrompt(v, branch, record) {
  return stepPrompt((record ? 'a RECORD\'S push — what it publishes is the run\'s records and nothing beside them: ' : '') + 'push `' + branch + '` of the stabilization run `' + v.run + '` by name', 'push', '--branch ' + branch + (record ? ' --run-dir ' + runDir(v.run) : '') + ' ' + digestFlag(v.scratch))
}
// The RECORD step's commit: the applied batch committed as ONE commit, when the gate that
// ran on the tree with the records shows nothing red that the candidate's own gate did not.
// The act is handed how many calls and checks the batch was composed of, and refuses a
// batch that is another's; the scratch root, under which a file that is no report is set
// aside; `head`, THE COMMIT THE STAGE BEGAN ON — the record is committed on it or not at
// all, and it is what lets the step, asked again, answer what it answered; and `fence`, THE
// GUARDED STATE as the stage's first read printed it (dev/stabilize-step: THE GUARDED STATE) —
// the write guard's backstop: what no step of a stage writes before this commit, and what a
// write past the guard moves, so that the record is refused (`fence-moved`) and halts the
// stage instead of committing beside it.
function recordCommitPrompt(v, branch, gate, expect, head, fence) {
  return stepPrompt('the ONE commit of a record step of `' + v.run + '` on `' + branch + '`: the applied batch, held to the gate that ran on it', 'record', '--branch ' + branch + ' --run-dir ' + runDir(v.run) + ' --gate ' + gate + ' --calls ' + expect.calls + ' --checks ' + expect.checks + ' --scratch ' + v.scratch + (head ? ' --head ' + head : '') + (fence ? ' --fence ' + fence : '') + ' ' + digestFlag(v.scratch))
}
// wordsText — THE CANONICAL FORM OF A COMPOSED RECORD'S PLAIN WORDS, as the record script
// defines it (dev/stabilize-record compose: THE CANONICAL FORM — its ONE definition; this is
// a caller's computation of it, and the simulation holds the two to each other on every
// record a stage composes): the run, the round, the attempt and the subject, a line each,
// then one line a word IN THE ORDER HANDED — nothing sorted, folded or trimmed — each line
// ended by ONE line feed, the last too. `sha256` hashes its UTF-8 bytes.
function wordsText(run, round, attempt, subject, words) {
  return ['run=' + run, 'round=' + round, 'attempt=' + attempt, 'subject=' + subject].concat(words).join('\n') + '\n'
}
// recordWords — THE PLAIN WORDS OF A `test` STAGE'S FIRST RECORD, IN A RUN WITH A QUEUE: what
// the record script cannot read off what was filed (its header: compose, THE WORDS), in the
// order the stage means them — which is the order of the report check's reporters and of
// the results' rows. `rec` is { reporters, commit, gate, did, facts }:
//   reporter=<name>   EVERY REPORTER THE STAGE LAUNCHED THAT LEFT A REPORT — the launcher's
//                     list, off which the report check took each one that left none
//   commit=<sha>      the commit the items ran on, which the gate is of: THE CANDIDATE
//   gate=<job>        the candidate's gate, by the name of the job the tool holds it under
//   item=<item>:…     what each item did: `held:<job>` for a held check that left a verdict
//                     — green, red or void, which the script reads out of the job's file;
//                     `red:<reporter>` for a check an agent read red — the preflight whose
//                     report says so, and which is the evidence, by path; `void:<word>
//                     [:<key>]` (VOID_WORDS, `heldVoid`); else `green`
//   fact=<name>:<v>   the round's facts, a list one word an item — and no word for an
//                     empty list
// NO ROW, NO ENTRY, NO GRADE AND NO VERDICT IS A WORD: the script reads those itself.
function recordWords(rec) {
  const words = rec.reporters.map((name) => 'reporter=' + name).concat(['commit=' + rec.commit], rec.gate ? ['gate=' + rec.gate] : [])
  for (const u of rec.did) words.push('item=' + u.item + ':' + (u.job ? 'held:' + u.job : u.status === 'void' ? 'void:' + u.word : u.status === 'red' ? 'red:' + u.by : 'green'))
  for (const [name, value] of rec.facts) for (const one of Array.isArray(value) ? value : [value]) words.push('fact=' + name + ':' + one)
  return words
}
// The COMPOSE step: a `test` stage's record composed BY THE TOOL from what was filed and
// from the plain words — held to the sha256 of their canonical form, computed here — and
// applied. `subject` is the commit's whole subject: one word of the call, quoted.
function composePrompt(ctx, branch, subject, words) {
  return stepPrompt('the record of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', attempt ' + ctx.attempt + ', COMPOSED BY THE TOOL from what the stage\'s agents filed and from the ' + words.length + ' plain word(s) after the `--` — and applied as ONE batch. THE WORDS ARE HELD TO THE HASH THE COMMAND NAMES: one word typed otherwise — a status, a name, their order — and the tool refuses, with nothing written', 'compose', '--branch ' + branch + ' --run-dir ' + runDir(ctx.run) + ' --round ' + ctx.round + ' --attempt ' + ctx.attempt + ' --subject ' + shq(subject) + ' --sha256 ' + sha256(wordsText(ctx.run, ctx.round, ctx.attempt, subject, words)) + ' --scratch ' + ctx.scratch + ' ' + digestFlag(ctx.scratch) + ' -- ' + words.join(' '))
}
// What the composing step is told when it is asked for again: the tool refused the words an
// earlier step typed as not the ones that were hashed.
function wordsAgain(n) {
  return 'AGAIN (' + n + '): an earlier step ran this command and the tool refused it — a word of it was not typed as it stands, so the words did not hash to the sha256 the command names, and nothing was written. Run the ONE command above typed as it stands, word for word and in its order: copy it, and change nothing in it.'
}
// The LAND step: a round's branch merged `--no-ff` into the loop branch and the loop branch
// pushed, as one act.
function landPrompt(v, round, branch) {
  const loop = branchName(v.run)
  return stepPrompt('land round ' + round + ' of `' + v.run + '`, ONE act: merge `' + branch + '` into `' + loop + '` with a merge commit, then push `' + loop + '` by name. The audit of its fix diff left nothing open', 'land', '--loop ' + loop + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + ' ' + listFlags('log', SYNC_LOGS) + ' ' + digestFlag(v.scratch))
}
function roundCommitsPrompt(v, round, branch) {
  return stepPrompt('the commits of round ' + round + ' of `' + v.run + '` (`' + branch + '`), and which of them touch the run\'s directory', 'round-commits', '--loop ' + branchName(v.run) + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + digestFlag(v.scratch))
}
// The CARRY step: named commits cherry-picked, one by one. Onto the loop branch — a dropped
// round keeps its record, adjustment iv — it is the tool's act. Onto a part's own branch
// (ruling 6) it is NOT: that exit is ruled to be rebuilt as a revert, so its re-cut stays the
// list of commands it was, to the byte, until that rebuild deletes it.
const CARRY_RULES = 'Run exactly the commands below, in order, and nothing else: the only commits you make are the cherry-picks this step names, one by one. No other branch, no file edit, no `git stash`, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. Any check that fails, or any command that fails, is a HALT: stop, leave everything as the step says, and fill the halt report (root_cause = which check, evidence = the command and its output, tree_state = `git status` and `git branch --show-current`).'
function carryPrompt(v, round, shas, part) {
  const loop = branchName(v.run)
  if (!part) return stepPrompt('round ' + round + ' of `' + v.run + '` is dropped: carry exactly its record commits over to `' + loop + '`, so that the round keeps its record', 'carry', '--loop ' + loop + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + ' ' + digestFlag(v.scratch) + ' -- ' + shas.join(' '))
  return [
    'GIT STEP — re-cut a part of round ' + round + ' of `' + v.run + '` as a branch of its own, `' + part + '`, from the tip of `' + loop + '`: the fix commits the human keeps, and the round\'s record commits. ' + CARRY_RULES,
    '1. `git status --porcelain --untracked-files=all` must print nothing.',
    '2. `git rev-parse --verify --quiet refs/heads/' + part + '` must FAIL and `git ls-remote --exit-code --heads origin ' + part + '` must exit 2 — the part\'s branch does not exist yet. `git switch ' + loop + '`, then `git switch --no-track -c ' + part + ' ' + loop + '`; pre = `git rev-parse HEAD`.',
    '3. Each of these ' + shas.length + ' commit(s), in this order, with `git cherry-pick -x <sha>` — each must exit 0; one that stops is `git cherry-pick --abort`, then HALT:',
  ].concat(shas.map((sha) => '   - ' + sha)).concat([
    '4. `git rev-list --count pre..HEAD` must print ' + shas.length + '.',
    '5. `git push origin ' + part + '` — the branch named in full; a rejected push is a HALT, never forced. Then `git ls-remote --exit-code --heads origin ' + part + '` must report the sha `git rev-parse ' + part + '` prints.',
    '6. Report status = carried, branch = `git branch --show-current`, head = `git rev-parse HEAD`, remote_head, and picked: for each commit named in step 3, in order, `from` = that sha and `to` = the sha of the commit its cherry-pick made (`git log --reverse --format=%H pre..HEAD` prints them in the same order).',
  ]).join('\n')
}
// The close's SYNC step — the fixed step before any pull request to main
// (implementation/dev-workflow.md -> Before a pull request to main). Returned to the
// orchestrator with `next: 'close'`, never run here: the close's own acts come first.
function syncMainPrompt(run, scratch) {
  const m = branchName(run)
  return stepPrompt('merge `origin/main` into `' + m + '` before its pull request to `main` is opened (the close of the stabilization run `' + run + '`). Nothing is pushed and no pull request is opened: the gate runs on the merged tree first', 'sync-main', '--loop ' + m + ' ' + listFlags('log', SYNC_LOGS) + ' ' + digestFlag(scratch))
}

// ---- the agents' prompts — thin: the role and its contract live in the definition ----
function opening(v) {
  return 'The run\'s opening record is `' + runDir(v.run) + '/opening.md`; its state is ' + typed(RECORD_TOOL, 'state --run ' + v.run) + '. ' + GUARD
}
// THE PREFLIGHT BUILDS NOTHING AND HOLDS NO GATE (the second repair plan's task K11): the
// two binaries, the candidate's gate and every held check are commands `dev/stabilize-step`
// holds, started and asked after by steps of this script. What is left to the role is what
// only an agent can do: the environment asserts, the trial image, and the checks that are a
// brief. Its prompt names no command at all — how it asserts is its definition's.
function preflightPrompt(ctx, launch, name, plan) {
  const steps = ['the environment asserts — the tree may hold untracked files under `' + runDir(ctx.run) + '/` and nothing else: what the record script wrote and no record step has committed yet — reports, the round\'s scope, its journal' + (plan.tested ? '. THE CANDIDATE IS NOT `HEAD` HERE: it is the commit the round tested, and the round\'s record commits lie on top of it — commit ' + plan.sha + ' must be an ancestor of `HEAD`, and nothing this call covers reads the working tree' : '')]
  const asked = plan.crossModel ? crossModelAssert() : null
  if (plan.image) steps.push('the trial image, built and verified from the candidate\'s commit')
  if (asked) steps.push(asked)
  if (plan.checks.length) steps.push('the deterministic checks, one per item below, each returned in `checks` under `check` = the item\'s id; a CI run that is still in progress 60 minutes on is void:\n' + plan.checks.map((c) => '   - ' + c.item + ': its brief is the `brief` of the item\'s row — ' + typed(RECORD_TOOL, itemRead(ctx.run, c.item))).join('\n'))
  return [
    'PREFLIGHT — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage. ' + opening(ctx),
    launch.line(name),
    'Candidate: label ' + plan.label + ', commit ' + plan.sha + ' — ' + (plan.tested ? 'the commit round ' + ctx.round + ' tested, an ancestor of' : 'the tip of') + ' `' + plan.branch + '`, which is checked out. Scratch root: `' + ctx.scratch + '`.',
    'NO BUILD AND NO GATE IS YOURS, whatever your definition says of either: the candidate\'s binary, the previous release\'s, the candidate\'s full gate and every scripted check are commands the harness has `' + STEP_TOOL + '` start, hold and judge, by steps of its own. You build nothing, run no gate, and return no binary.',
    'The steps this call covers, in this order, and nothing else:',
  ].concat(steps.map((s2, i) => (i + 1) + '. ' + s2)).join('\n')
}
function scopeText(scope, fallback) {
  if (scope == null) return 'the run\'s default scope, a fact of its record: ' + (fallback === 'everything' ? 'everything — every door of every registry is inside' : 'the derived delta')
  if (typeof scope === 'string') return scope === 'everything' ? 'everything — set by the human: every door of every registry is inside' : 'the derived delta'
  return scope.range ? 'the commit range ' + scope.range + ' — set by the human' : 'the named doors, set by the human: ' + scope.doors.join(' · ')
}
// The file the scope step writes the round's scope to before it hands it over.
function scopeFile(ctx) {
  return ctx.scratch + '/scope/r' + ctx.round + '.a' + ctx.attempt + '.json'
}
function scopePrompt(ctx, launch, name, plan) {
  return [
    'SCOPE — stabilization run `' + ctx.run + '`, round ' + ctx.round + '. ' + opening(ctx),
    launch.line(name),
    'The round\'s change lies between: base = ' + (plan.base ? plan.base + ' (the candidate round ' + (ctx.round - 1) + ' tested)' : plan.previous.commit + ' (the previous release, ' + plan.previous.version + ', as the run\'s record names it)') + '; tip = ' + plan.sha + ' (label ' + plan.label + ').',
    'The scope this stage was started with: ' + scopeText(plan.scope, plan.fallback) + '.',
    plan.earlier ? 'The door lists of round ' + (ctx.round - 1) + '\'s fixers and fix-diff auditors — inputs, never the result: the `doors_affected` of every report under `' + runDir(ctx.run) + '/r' + (ctx.round - 1) + '/reports/fix/`, which this prompt hands you.' : 'There is no earlier round whose door lists could be an input.',
    'The units of the test set are the rows of `' + runDir(ctx.run) + '/test-set.md` (the state document\'s `items`).',
    'Write the round\'s scope — ONE JSON object, as that script\'s help has it — to the file `' + scopeFile(ctx) + '` WITH YOUR FILE TOOL, and hand it over, once, by ONE plain call, typed as it stands: ' + typed(RECORD_TOOL, 'scope-set --run ' + ctx.run + ' --round ' + ctx.round + ' --scratch ' + ctx.scratch + ' --from ' + scopeFile(ctx)) + '.',
  ].join('\n')
}
function doorList(doors) {
  return doors.length ? doors.map((d) => '   - ' + d.door + '   [' + d.registry + ']').join('\n') : '   (none)'
}
// WHAT AN AGENT READS BY KEY (dev/stabilize-record: `item`, `item-doors`). This script hands
// an agent no text of the record: a unit's prompt names the read that prints its item's row,
// brief included, and the read that prints the doors of the round its item covers — the
// selection this script made itself until 2026-10-07 is that read's (`unitDoors` there),
// from the round it is asked for, which for a re-run is the round the position gives.
function itemRead(run, item) {
  return 'item --run ' + run + ' --item ' + item
}
function doorsRead(run, round, item) {
  return 'item-doors --run ' + run + ' --round ' + round + ' --item ' + item
}
// kitRead / rowRead — the two reads a verifier of a run with a queue is handed (dev/stabilize-
// record: `kit`, `row`): what every agent of its role looks up, in one read — its FIRST — and
// its finding's row, whole, by its key. No text of either passes through this script.
function kitRead(run, role) {
  return 'kit --run ' + run + ' --role ' + role
}
function rowRead(run, key) {
  return 'row --run ' + run + ' --key ' + key
}
// briefLine / doorLines — a unit's brief and its door list, as its prompt says them: by the
// read, for an item of the test set; as given, for the one unit that is this script's own
// (the audit of a fix diff, whose brief is this script's sentence).
function briefLine(ctx, unit) {
  return unit.brief != null ? 'Your brief: ' + unit.brief : 'Your brief is the `brief` of your item\'s row in the test set — read it: ' + typed(RECORD_TOOL, itemRead(ctx.run, unit.item)) + '.'
}
function doorLines(ctx, unit) {
  return Array.isArray(unit.doors) ? 'The door list — a finding names its door in these exact words:\n' + doorList(unit.doors) : 'The door list — a finding names its door in these exact words — is what ' + typed(RECORD_TOOL, doorsRead(ctx.run, unit.round, unit.item)) + ' prints: ' + (unit.doors == null ? 'the doors' : unit.doors + ' door(s)') + ' of round ' + unit.round + '\'s test set that your item covers, each with its registry and its derivation.'
}
// forkNames / unitNames — the reporter names a fork and a list of units will ask for, or
// why one cannot be made: found before anything is launched, never inside a running chain.
function forkNames(tag, key) {
  const names = [['verify', tag, key], ['advocate', tag, key], ['proposal', tag, key]]
  return names.every((parts) => reporterName(parts)) ? names : null
}
function unitNames(units, taken) {
  const names = taken.slice()
  for (const unit of units) {
    for (const stepList of unit.chain) {
      for (const step of stepList) {
        const name = reporterName([unit.item, step.as])
        if (!name || names.includes(name)) return { fault: 'no reporter name of its own can be made for `' + unit.item + '`, ' + step.as + (name ? ' (`' + name + '` is taken)' : ' (not a slug of at most ' + SLUG_MAX + ' characters)') }
        names.push(name)
      }
    }
  }
  return { names }
}
function unitPrompt(ctx, launch, name, step, unit, built, handed) {
  const role = ROLES[step.role]
  const lines = [
    'STABILIZATION RUN `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage — ' + step.task + ': `' + unit.item + '`. ' + opening(ctx),
    launch.line(name),
  ]
  if (role.drives) lines.push(binaryLine(built, step.image, ctx.scratch))
  lines.push(briefLine(ctx, unit))
  lines.push('Candidate: label ' + built.candidate.label + ', commit ' + built.candidate.sha + '. Scratch root: `' + ctx.scratch + '` — mint your own directory under it.')
  if (unit.range) lines.push('The unit is the diff `' + unit.range + '` over the product paths (' + PRODUCT_PATHS.join(', ') + ').')
  lines.push(doorLines(ctx, unit))
  if (handed.length) lines.push('This prompt hands you these reports of the same item, and no other:\n' + handed.map((h) => '   - ' + h.as + ': ' + (h.report ? '`' + h.report + '`' : '(it left no report)')).join('\n'))
  return lines.join('\n')
}
function triagePrompt(ctx, launch, name, sources, pass) {
  const others = launch.names.filter((n) => n !== name)
  return [
    'TRIAGE — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage, pass ' + pass + '. ' + opening(ctx),
    launch.line(name),
    'The reporters this stage has launched so far, each with one report of attempt ' + ctx.attempt + ' — a report of the stage beside these is the partial stage you halt on: ' + (others.join(' · ') || '(none)') + '.' + (pass > 1 ? ' This is pass ' + pass + ': it grades only what the agents of the pass before left open; every other finding of the stage was graded then, and is not handed to you again.' : ''),
    'Grade every finding below — ' + sources.reduce((n, s) => n + findingsOf(s), 0) + ' in all, from ' + sources.length + ' reporter(s). The reports are the files named; each list is that reporter\'s structured return, and an entry marked `left open` is what an agent left open, triaged like any finding.' + (sources.some((s) => s.reporter === LEDGER_SOURCE) ? ' The source `' + LEDGER_SOURCE + '` is no reporter of this stage: its entries are rows the ledger already holds, whose triage nobody finished. Grade each under its own key, found again — from the report its repro names, which this prompt hands you with it — and count the source in `findings_in` like any other.' : ''),
  ].concat(sources.map((s) => '- ' + s.reporter + ' — report: ' + (s.report ? '`' + s.report + '`' : '(none)') + ' — ' + findingsOf(s) + ' finding(s):' + (s.read ? ' the rows ' + s.read + ' prints — each whole, under its key, with why its triage is not finished (`ungraded`: nobody graded it; `unverified`: graded, and nobody verified it). Read them there: this prompt carries none of them.' : '\n' + s.findings.map((f) => '   - ' + f).join('\n')))).join('\n')
}
// oneTriagePrompt — THE PROMPT OF THE STAGE'S ONE TRIAGE, IN A RUN WITH A QUEUE (the reshape's
// plan, T5a). The definition's paragraph for it binds on THE CALL, SPELLED WHOLE
// (finding-triage.md: "Where your prompt spells a `dev/stabilize-record triage` call"): the
// run, the round, the reporter and the attempt, the file it reads, and one `<source>=<count>`
// word for every source — so this prompt spells it so, and NUMBERS EVERY HANDED FINDING, 1
// to its source's count, under the source's name: the record script holds the partition to
// those numbers. A SOURCE IS ONE OF THREE: a reporter (its findings, or what it left open);
// A RED RESULT (`redSources`), under the key its row is filed with and with where its
// evidence lies; and THE ROWS NOBODY GRADED (`ledgerSource`), which this prompt carries none
// of — it names the read, and the rows are numbered in the order that read prints them. THE
// `REPORT:` LINE STAYS, and says what it is for here: the call renders the report of a
// triage that grades, so the line's own call is A HALT'S ROAD and no other. AND THE LAST
// LINE SAYS WHAT THE RETURN IS HELD TO (`filedFault`) — the canary's triage read this
// script's source to find that out.
function oneTriagePrompt(ctx, launch, name, sources) {
  const others = launch.names.filter((n) => n !== name)
  const said = (s) => (s.red
    ? '- ' + s.reporter + ' — A RED RESULT, and no reporter: the check `' + s.red.item + '` of this invocation came back red. The stage\'s record files its row under the key `' + s.reporter + '`, in the batch that records your grade — so it gets ONE entry, under that key, found again; you mint no key for it. Grade it from ' + (s.red.verdict ? 'the tool\'s verdict of the held command, the file `' + s.red.verdict + '`' + (s.red.output ? ' (the command\'s whole output is `' + s.red.output + '`)' : '') : 'the report of the preflight that read it red, `' + s.red.report + '`') + ' — 1 finding:'
    : s.read ? '- ' + s.reporter + ' — THE ROWS NOBODY GRADED, and no reporter of this stage: rows the ledger already holds — ' + findingsOf(s) + ' finding(s): the rows ' + s.read + ' prints, NUMBERED 1 TO ' + findingsOf(s) + ' IN THE ORDER THAT READ PRINTS THEM — each whole, under its key. Each is found again, under its own key, graded from the report its repro names. Read them there: this prompt carries none of them.'
      : '- ' + s.reporter + ' — report: ' + (s.report ? '`' + s.report + '`' : '(none)') + ' — ' + findingsOf(s) + ' finding(s):')
    + (s.read ? '' : '\n' + s.findings.map((f, i) => '   ' + (i + 1) + '. ' + f).join('\n'))
  return [
    'TRIAGE — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage: THE STAGE\'S ONE TRIAGE. ' + opening(ctx),
    launch.line(name) + ' UNDER THIS PROMPT THAT IS A HALT\'S ROAD, AND YOU TAKE IT ONLY WHERE YOU HALT: where you grade, the `triage` call below writes the file at the path a report of your name and attempt has — your report, rendered from your entries — and a report handed over beside it for the same attempt would be refused.',
    'The reporters this stage has launched so far, each with one report of attempt ' + ctx.attempt + ' — a report of the stage beside these is the partial stage you halt on: ' + (others.join(' · ') || '(none)') + '.',
    'Grade every finding below — ' + sources.reduce((n, s) => n + findingsOf(s), 0) + ' in all, from ' + sources.length + ' source(s). EVERY FINDING HAS A NUMBER, from 1 to its source\'s count, under its source\'s name, and an entry says what it folds by the source and those numbers. A reporter\'s report is the file named, its list that reporter\'s structured return, and an entry marked `left open` is what an agent left open, triaged like any finding.',
  ].concat(sources.map(said)).concat([
    'YOUR ENTRIES ARE ONE FILE, HANDED OVER ONCE. Write them — a JSON array, one object an entry — to the file `' + entriesFile(ctx) + '` WITH YOUR FILE TOOL, and hand it over by ONE plain call, typed as it stands: ' + typed(RECORD_TOOL, 'triage --run ' + ctx.run + ' --round ' + ctx.round + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + ' --from ' + entriesFile(ctx) + ' -- ' + sources.map((s) => s.reporter + '=' + findingsOf(s)).join(' ')) + '. Its last words say how many findings each source above handed you, and the record script holds every number of every source to being in exactly one entry.',
    'WHAT YOUR RETURN IS HELD TO, and it holds no entry. That call prints ONE line of JSON. Return status = filed with three things of it: `report`, the path it printed as `triage` — held to `' + testReport(ctx, name) + '`, the path a report of your name and attempt has; `sha256`, the hash it printed beside that path — held to being one; and `handed`, what it printed under that name, one entry a source with its count — held to the sources and the counts of the call above, each by name: a call typed with another count held another partition, and the stage halts on it. Nothing else is read from your return: no entry, no grade and no key is typed a second time, and you name nothing to verify — what is verified is the record script\'s queue. Where you halt, return status = halted with the halt report, a refusal\'s line verbatim in it.',
  ]).join('\n')
}
function verifyPrompt(ctx, launch, name, entry, redrive, built) {
  return [
    'VERIFY-REAL — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', ONE finding. ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false, ctx.scratch),
    'The finding: ledger key `' + entry.key + '` · door: ' + entry.door + ' · the clause it is said to break: ' + entry.clause + ' · triage\'s grade: ' + entry.grade + ' · its repro block: ' + entry.repro + '.',
    redrive ? 'What triage asks you to re-drive: ' + redrive : '',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].filter(Boolean).join('\n')
}
// THE RULE OF A PRINTED ROUTE, beside both branches of the hand-on line (DECISIONS.md ->
// 2026-10-18, "A verifier hands on every printed route it did not run"; the human's ruling,
// option A): a route is confirmed only by being run as printed, so one a verifier saw printed
// and did not run is an entry however right it looks — the fifth canary root's verifier kept
// one back as "plausibly right".
const PRINTED_ROUTE = 'A ROUTE YOU SAW PRINTED AND DID NOT RUN AS PRINTED IS AN ENTRY OF `hands_on`, whether or not it looks right to you: a route is confirmed only by being run as printed, and one you ran as printed belongs to your verdict.'
// verdictPrompt — THE PROMPT OF A VERIFIER THAT FILES ITS VERDICT, IN A RUN WITH A QUEUE (the
// reshape's plan, T6a). The definition's paragraph for it binds on THE CALL, SPELLED WHOLE
// (finding-verifier.md: "Where your prompt spells a `dev/stabilize-record verdict` call"): the
// run, the round, the reporter (`verify-<key>`), the attempt and the file it reads — so this
// prompt is what switches that paragraph on. IT HANDS THE VERIFIER ITS KEY AND NOTHING OF ITS
// ROW: the read that prints the row is named, and the verifier runs it. THE KIT'S CALL STANDS
// FIRST of the calls it is to make of the record script, as the definition has it its first
// read. THE `REPORT:` AND `BINARY:` LINES STAY: the report comes before the filing, a halt is
// reported by it, and the two hashes a filing holds — `candidate`, `previous` — are the ones
// asserted under the binary line: MEMBERS OF THE FILE, AND NO WORD OF THE CALL, which takes
// none. `handOn` is the opening's number, as the state's digest has it: how many of what the
// verifier hands on enter the queue — or null, where the opening wrote none and nothing does.
// AND THE LAST LINE BUT ONE SAYS WHAT THE RETURN IS HELD TO (`verdictFault`), and what counts.
function verdictPrompt(ctx, launch, name, key, built, handOn) {
  return [
    'VERIFY-REAL — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', ONE finding: the row `' + key + '` of the run\'s ledger. ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false, ctx.scratch),
    'YOUR FIRST READ, before anything else: ' + typed(RECORD_TOOL, kitRead(ctx.run, 'verify')) + ' — it prints what every verifier looks up: the rules of a report, the slug rule, the rig and how a binary is named to it, the format of a repro block, and the run\'s closing condition. Read them there, and look none of them up again.',
    'THE FINDING IS A ROW, AND YOU READ IT BY ITS KEY: ' + typed(RECORD_TOOL, rowRead(ctx.run, key)) + ' prints it whole — its door, the clause it is said to break, its repro, its grade where triage graded it, its class, and for an item a verifier handed on, who handed it on and under which finding. This prompt carries none of it.',
    'YOUR VERDICT IS ONE FILE, AND YOU FILE IT ONCE — after your report is handed over, and before you return. Write it — ONE JSON object, its members as your definition has them — to the file `' + filingFile(ctx, name) + '` WITH YOUR FILE TOOL, and file it by ONE plain call, typed as it stands: ' + typed(RECORD_TOOL, 'verdict --run ' + ctx.run + ' --round ' + ctx.round + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + ' --from ' + filingFile(ctx, name)) + ' (with one more `--scratch` for each scratch root of your own outside that one). THE TWO HASHES ARE MEMBERS OF THAT FILE, and no word of the call: `candidate` is the sha256 you asserted for the candidate\'s binary under the `' + LABELS.binary + '` line, and `previous` the one you asserted for the previous release\'s, wherever you drove it.',
    (handOn == null
      ? 'WHAT YOU HAND ON ENTERS NO QUEUE: the run\'s opening names no number of items a verifier hands on, so every entry of `hands_on` is recorded as what it is, in no queue, and the human may put one in by its key.'
      : 'WHAT YOU HAND ON — AT MOST ' + handOn + ' ENTER THE QUEUE: that is the number of the run\'s opening. `hands_on` is in the order you would have them verified: the first ' + handOn + ' that carry the evidence of their kind are handed on to the queue, and a later one is recorded as what it is, in no queue.')
      + ' ' + PRINTED_ROUTE,
    'WHAT YOUR RETURN IS HELD TO, and it holds no verdict. That call prints ONE line of JSON. Return status = filed with three things of it: `filing`, the path it printed as `verdict` — held to `' + verdictFile(ctx, name) + '`, the one path a verdict of your name and attempt is filed at; `word`, the verdict as it printed it — held to ' + FILED_WORDS.join(', ') + '; and `contested`, true or false as it printed it. Nothing else of your verdict is read from your return: no basis, no block and no item you hand on is typed a second time. WHAT COUNTS IS WHAT IS FILED: once your wave is over the harness reads the run\'s queue again, and a finding the queue prints again has no verdict, whatever its verifier returned. Where you halt, return status = halted with the halt report, a refusal\'s line verbatim in it — and file nothing.',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].join('\n')
}
function advocatePrompt(ctx, launch, name, fork, built) {
  return [
    'A FORK of the stabilization run `' + ctx.run + '`, round ' + ctx.round + ' — finding `' + fork.key + '` (' + fork.kind + '). ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false, ctx.scratch),
    'The finding: door: ' + fork.door + ' · clause: ' + fork.clause + ' · repro: ' + fork.repro + '.',
    'What raised the fork: ' + fork.statement,
    'The cheap cut is what that statement would leave as it is; argue the robust case, and drive what you propose. Return `proposal` as a change somebody else can apply to a clone of commit ' + built.candidate.sha + ': another agent drives it after you, without your spike.',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].join('\n')
}
// reportHandOver — how a role that has no definition is told to write its report: the file,
// by its file tool, and the ONE plain call that hands it over. A role with a definition is
// told by its `REPORT:` line and by that definition.
function reportHandOver(ctx, name) {
  return 'Write it, whole, to the file `' + reportFile(ctx, name) + '` WITH YOUR FILE TOOL — its last line is `<!-- end of report -->` — and hand it over by ONE plain call, typed as it stands: ' + reportCall(ctx, name) + '; return the path it prints as `report`. A refusal about your text you repair in the file, and hand it over again; any other refusal you return as your halt, verbatim. Never remove, recursively, a path built from variables, and read every exit status from the command itself, bare.'
}
// The independent drive of an advocate's proposal. Its agent has no definition, so this
// prompt is its whole contract — and so it is long where the others are thin. ITS `HOW` IS
// THE ONE ROW THAT IS NOT REPLACEABLE (DECISIONS.md -> 2026-10-07, "A long command is started,
// waited for and judged by the tool", item 12, row 12): a proposal is a PATCHED tree, which
// is no commit, and the step tool builds a commit and nothing else — so the drive clones,
// patches and builds by a shell of its own, under the session's permission mode. What CAN
// be a plain call is one: the hash it asserts is asked of the tool, on a line of its own.
function proposalPrompt(ctx, name, fork, advocate, built) {
  return [
    'You drive a PROPOSAL you did not write. A fork of the stabilization run `' + ctx.run + '` (round ' + ctx.round + ', finding `' + fork.key + '`) goes to the human with an advocate\'s case; a proposal is a claim, and before the human sees it somebody other than its author runs it end to end — through every OTHER party\'s next step (implementation/milestone-planning-workflow.md, Settle: the claim-driven rule). You are that somebody. Your brief is to REFUTE it: assume a step the advocate reports as passing does not pass.',
    'The proposal, as its author returned it: ' + (advocate.proposal || '(none returned — read it from the report)'),
    'The advocate\'s report, handed to you: ' + (advocate.report ? '`' + advocate.report + '`' : '(none)') + '. The steps it says it drove:\n' + ((advocate.driven || []).map((d) => '   - ' + d.step + ' — ' + d.command + ' — ' + d.result).join('\n') || '   (none)'),
    'HOW. In a directory of your own, minted with `mktemp -d` under `' + ctx.scratch + '`: clone commit ' + built.candidate.sha + ' (`git clone` this repository there and check that commit out, detached — never the working tree, never a branch of this repository), apply the proposal as written, build it there (`cargo build --release --locked`, a target directory of its own), and run every next step the proposal names, each as printed, each exit status read bare. Where the proposal names a step you cannot run, say so in `undriven`, with why.',
    'THE BEFORE of each step is the candidate itself, and its hash is ASKED OF THE TOOL, by ONE plain call typed as it stands, and never computed by a command of your own: ' + typed(STEP_TOOL, 'hash --scratch ' + ctx.scratch + ' --file ' + below(ctx.scratch, built.candidate.binary)) + ' prints ONE line of JSON whose `content_sha256` must be ' + built.candidate.sha256 + ', and you return that hash as `asserted_sha256`.',
    'WHAT YOU RETURN. One entry of `steps` per next step: the command, what it printed and exited, and `agrees` — whether that is what the advocate reported. `holds` is true only when every step ran as the proposal says. What you hit that is not this proposal goes in `left_open`.',
    'WHAT YOU MAY TOUCH. Nothing in this repository: no edit, no commit, no branch, no push, no install. One file reaches it — your report. ' + reportHandOver(ctx, name) + ' ' + GUARD,
    NEVER,
  ].join('\n')
}
// The cross-model pass — everything this script says about it is in these three functions,
// and each is called only where the invocation's opt-in is tested
// (tooling-tests/stabilize_harness_fence.rs holds both).
function crossModelTool() {
  return 'codex'
}
function crossModelAssert() {
  return 'whether the cross-model pass\'s tool answers, returned in `checks` under `check` = `' + CROSS_CHECK + '`: green when `command -v ' + crossModelTool() + '` answers and `' + crossModelTool() + ' --version` exits 0, void when either does not — with what it printed. It is NOT one of the environment asserts: void is an answer, never a halt, and nothing is installed'
}
function crossModelPrompt(ctx, name, unit) {
  const tool = crossModelTool()
  return [
    'A CROSS-MODEL SOURCE PASS of review row `' + unit.item + '` — stabilization run `' + ctx.run + '`, round ' + ctx.round + '. This invocation named this item for it (args.crossModel): a cross-model second opinion is a human-approved suggestion, never auto-run (implementation/milestone-planning-workflow.md, Review), and naming the item was the approval. You relay what a model of another family claims about this row\'s source; you judge none of it and drive none of it.',
    'HOW. `command -v ' + tool + '` must answer — if it does not, halt and say so: nothing is installed. In a directory of your own, minted with `mktemp -d` under `' + ctx.scratch + '`, write the pass\'s prompt to a file: the row\'s brief (the `brief` of the item\'s row, which ' + typed(RECORD_TOOL, itemRead(ctx.run, unit.item)) + ' prints — read it, and hand over what it says to review), the door list below, and the instruction to return each claim with its `file:line` and the door it stands at. Then run `' + tool + ' review - < <the prompt file> > <an output file> 2>&1`, its exit status read bare, and read the output file.',
    'WHAT ITS CLAIMS ARE. Leads, never findings: a claim one model makes that nobody reproduced is a lead, and the row\'s reconciler drives every one. Return each as a finding with `lead` set to true, its door in the exact words of the list below (or named plainly and marked unlisted), the clause it would break as the claim reads, and `repro` = the heading you give the claim in your report. A pass that returned nothing usable is a halt, with its output as the evidence.',
    doorLines(ctx, unit),
    'WHAT YOU MAY TOUCH. Nothing in this repository: no edit, no commit, no branch, no push, no install. One file reaches it — your report, the pass\'s output whole and your list of leads. ' + reportHandOver(ctx, name) + ' ' + GUARD,
    NEVER,
  ].join('\n')
}
function fixerPrompt(ctx, launch, name, area, branch) {
  return [
    'FIX — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', cycle ' + ctx.cycle + '. ' + opening(ctx),
    launch.line(name),
    branchLine(branch, branchName(ctx.run)),
    LABELS.area + ' ' + area.registry + ' — ' + area.findings.length + ' finding(s), in this order, each by its ledger key:',
  ].concat(area.findings.map((f) => '- `' + f.key + '` · door: ' + f.door + ' · clause: ' + f.clause + ' · repro: ' + f.repro + (f.detail ? ' · the human\'s ruling: ' + f.detail : ''))).join('\n')
}

// A record step's payload: ONE line, which its executor writes to a file WITH ITS FILE TOOL —
// no here-document, no redirect — and which the writer it is handed to holds to this hash
// (`apply --from FILE --sha256 HASH`): a batch that is not the text this script composed is
// refused there, and nothing is written. The text stands between two marker lines, so that
// what is data is told from what is an instruction.
function payload(dir, name, value) {
  const text = JSON.stringify(value)
  const file = dir + '/' + name
  return {
    file,
    sha256: sha256(text + '\n'),
    write: 'Write the file `' + file + '` WITH YOUR FILE TOOL — the tool that writes a file, and no shell: its content is exactly the ONE line that stands between the two marker lines below — neither marker, nothing before it, and ONE newline after it.\n' + PAYLOAD_ENDS + '\n' + text + '\n' + PAYLOAD_ENDS,
  }
}
// RECORD_RUNS — how the executor is told to run a record's steps.
const RECORD_RUNS = 'Run these from the repository\'s root, in this order, each exactly as written and each exit status read bare. A refusal is a halt — its one line the evidence — and nothing it names is repaired by hand:'
const RECORD_RETURNS = 'YOU MAKE NO COMMIT, stage nothing, and RUN NO GATE — whatever your definition says of one: the harness holds the full gate itself, as a command of `' + STEP_TOOL + '`, by steps of its own that follow this one, and then makes the one commit. Return status = applied as soon as step 2 printed its line. A halt is a refusal of step 2 — and with it return refused = the ONE word that follows `refused` on the line it printed on stderr. One refusal is yours to repair, once: `bad-value`, naming the file as not the batch that was hashed — write the file again, exactly, and run step 2 again; refused a second time, it is your halt.'
// subjectOf — the subject of a record's commit, after its `docs(record): `: plain ASCII,
// with nothing a shell reads, because it is a word of the ONE plain call that applies the
// batch.
function subjectOf(run, round, what) {
  return run + ' r' + round + ' - ' + what
}
// call — one call of a record's batch: a subcommand of the record script with its flags, as
// its argument list, and what it reads on stdin.
function call(flags, more, stdin) {
  const made = { argv: flags.split(' ').concat(more || []) }
  if (stdin !== undefined) made.stdin = JSON.stringify(stdin)
  return made
}
function isCheck(made) {
  return made.argv[0].startsWith('check-')
}
// recordPrompt — a record step's executor: the calls as ONE batch, written to a file and
// applied from it, AND NOTHING ELSE — the gate that follows is a command the step tool
// holds. Returns the prompt, the name that gate is held under — one per record, from the
// directory this script names the record by — and what the commit step is held to.
function recordHold(dir) {
  return 'record-' + dir.slice(dir.lastIndexOf('/') + 1)
}
function recordPrompt(v, what, branch, dir, round, calls, subject) {
  const batch = payload(dir, 'batch-' + sha256(JSON.stringify(calls) + '\n').slice(0, 12) + '.json', calls)
  const text = [
    LABELS.record + ' — stabilization run `' + v.run + '`: ' + what + '.',
    branchLine(branch, branchName(v.run)),
    RECORD_RUNS,
    '1. ' + batch.write,
    '2. ' + typed(RECORD_TOOL, 'apply --run ' + v.run + (round ? ' --round ' + round : '') + ' --subject ' + shq('docs(record): ' + subject) + ' --scratch ' + v.scratch + ' --from ' + batch.file + ' --sha256 ' + batch.sha256) + ' — the ' + calls.length + ' call(s) of this record as ONE batch, read from the file of step 1 and held to its hash: every table is written, or none is. It prints one line of JSON.',
    RECORD_RETURNS,
  ].join('\n')
  return { text, hold: recordHold(dir), expect: { calls: calls.length, checks: calls.filter(isCheck).length } }
}
// pendingRecord — the record step of a batch an earlier invocation applied and did not
// commit: NO EXECUTOR RUNS — nothing is applied and no table written — and what is left of
// the step is its gate, held by the tool, and its one commit.
// (`pending`: the gate of such a record is held under the ONE name every pending batch shares
// — `recordStep` says so where no name is left of it.)
function pendingRecord(dir, pending) {
  return { text: null, hold: recordHold(dir), pending: true, expect: { calls: pending.calls, checks: pending.checks } }
}
// stageRecordCommands — a stage's record, as the calls of its ONE batch: the reports checked
// first, then the candidate's gate (a `test` stage's), then what each item did, then the
// rows, then the ledger checked. Every row is composed here, from structured returns. THE
// RESULTS COME BEFORE THE LEDGER'S ROWS: the record script takes a re-run only while its
// state asks for one, and a finding the re-run found would end that. THE DISPOSITIONS ARE WRITTEN
// BEFORE THE TRIAGE: a round's triage records the disposition a row carried when it was
// found, and a fix cycle's audit finds a finding AFTER that cycle's fix — written the other
// way round, a fix that did not hold would be read as a finding followed by its fix.
function stageRecordCommands(ctx, rec) {
  const calls = []
  if (rec.reporters.length) calls.push(call('check-reports ' + stageFlags(ctx) + ' --attempt ' + ctx.attempt + ' --', rec.reporters))
  if (rec.gate) calls.push(call('gate-set --run ' + ctx.run + ' --round ' + ctx.round + ' --commit ' + rec.gate.commit + ' --summary ' + rec.gate.file + ' --scratch ' + ctx.scratch))
  if (rec.results && rec.results.rows.length) calls.push(call('result-set --run ' + ctx.run + ' --round ' + ctx.round + ' --commit ' + rec.results.commit + ' --scratch ' + ctx.scratch, [], rec.results.rows))
  if (rec.rows.length) calls.push(call('ledger-add --run ' + ctx.run + ' --scratch ' + ctx.scratch, [], rec.rows))
  if (rec.patches.length) calls.push(call('ledger-set --run ' + ctx.run + ' --scratch ' + ctx.scratch, [], rec.patches))
  if (rec.triage.length) calls.push(call('triage-set --run ' + ctx.run + ' --round ' + ctx.round, [], rec.triage))
  if (rec.facts) calls.push(call('round-set --run ' + ctx.run + ' --round ' + ctx.round, [], rec.facts))
  if (rec.keys.length) calls.push(call('check-ledger --run ' + ctx.run + ' --', rec.keys))
  return calls
}
// rulingsRecordPrompt — THE one step through which the human's rulings reach the record
// (ruling 4): the three dispositions that are the human's, and the rows of the declared-
// bounds list. No other function of this script composes either call with those values.
function rulingsRecordPrompt(v, round, branch, rulings, ran, at) {
  const calls = []
  // What the human ruled about the run: the go after the stop that follows `round`; one
  // more re-run of a clause — a fact of every round that selected an item of it whose
  // re-run is spent; one more triage of a finding — a fact of the latest round, whose
  // record counts them; one more attempt of a stage — a fact of the round it is spent in;
  // and either bound, raised. The record script takes each only while the state asks for it.
  const about = rulings.filter((r) => runRuling(r))
  for (const r of about.filter((x) => !QUEUE_RULINGS.some((name) => x[name] != null))) {
    const facts = r.go != null ? [{ value: { go: true }, call: 'round-set --run ' + v.run + ' --round ' + round }]
      : r.rerun != null ? ran[r.rerun].map((spentIn) => ({ value: { granted: r.rerun }, call: 'round-set --run ' + v.run + ' --round ' + spentIn }))
        : r.reverify != null ? [{ value: { reverify: r.reverify }, call: 'round-set --run ' + v.run + ' --round ' + at.round }]
          : r.again != null ? [{ value: { again: r.again }, call: 'round-set --run ' + v.run + ' --round ' + at.stages[r.again] }]
            : r.cycles != null ? [{ value: { cycles: r.cycles }, call: 'run-set --run ' + v.run }]
              : [{ value: { rounds: r.rounds }, call: 'run-set --run ' + v.run }]
    for (const fact of facts) calls.push(call(fact.call, [], fact.value))
  }
  // WHAT THE HUMAN RULED ABOUT THE QUEUE — another order, the rows put in by key, the budget
  // raised: ONE CALL EACH, IN THAT ORDER, WHICHEVER WAS SAID FIRST (QUEUE_RULINGS). The record
  // script holds each to the state as it stands before ITS call, and a call of a batch reads
  // what the calls before it wrote: the raise is judged on the queue the order and the keys
  // leave. A batch is written whole or not at all, so a raise refused there refuses the
  // order beside it.
  for (const name of QUEUE_RULINGS) for (const r of about.filter((x) => x[name] != null)) calls.push(call('run-set --run ' + v.run, [], { [name]: r[name] }))
  const bounds = rulings.filter((r) => r.bound != null)
  for (const r of bounds) calls.push(call('bound-set --run ' + v.run + ' --bound ' + r.bound + ' --scratch ' + v.scratch, ['--reach', r.reach, '--ruling', r.where, '--pin', r.pin]))
  const patches = rulings.filter((r) => r.key != null).map((r) => {
    const patch = { key: r.key, disposition: r.ruling }
    if (r.ruling === 'bound') patch.detail = r.reach
    else if (r.note != null) patch.detail = r.note
    return patch
  })
  if (patches.length) {
    calls.push(call('ledger-set --run ' + v.run + ' --scratch ' + v.scratch, [], patches))
    calls.push(call('check-ledger --run ' + v.run + ' --', patches.map((p2) => p2.key)))
  }
  return recordPrompt(v, 'the human\'s rulings — ' + patches.length + ' on a finding, ' + bounds.length + ' declared bound(s), ' + about.length + ' about the run. They are the human\'s, relayed: record them as given, and judge none of them', branch, v.scratch + '/record/rulings-r' + round, round, calls, subjectOf(v.run, round, 'the rulings of the human'))
}

// ---- the runtime probes: what a probe's agents are told, and what it hands the tool ----
// probeStep — a probe's step that is ONE command of the probe tool: THE HARNESS'S OWN STEP
// PROMPT, word for word, with that tool where the step tool stands. What a probe's git step
// is told — one command, its line relayed whole — is what a stage's is told.
function probeStep(what, act, flags) {
  return stepPrompt('a PROBE of the stabilization harness, of no run — ' + what, act, flags).replace('1. `' + STEP_TOOL + ' ', '1. `' + PROBE_TOOL + ' ')
}
// The `required` probe's reviewer: a source pass that drives nothing, under the binary line a
// stage's reviewer is handed — and, for case a, told in so many words to leave the hash out.
function probeReviewPrompt(id, binary) {
  const lines = [
    'A PROBE of the stabilization harness\'s runtime — the `required` probe, case ' + id + '. It is no stage of any run: there is no round, no test set and NO REPORT — you write no file, and you run no command of `dev/stabilize-record`.',
    binaryLine({ candidate: { binary: binary.file, sha256: binary.sha256, sha: '0'.repeat(40), label: 'probe' } }, false),
    'Your brief: one review row\'s SOURCE PASS, and a small one — read the comment block `' + PROBE_TOOL + '` opens with, and return what you find wrong in it as findings. None is expected, and an empty `findings` is an answer. YOU DRIVE NOTHING: the file on the line above is a stand-in and no build of any commit — the commit that line names is forty zeros and names nothing — and no command of it is run.',
    'The door list — a finding names its door in these exact words:\n' + doorList([{ door: PROBE_TOOL, registry: 'tooling' }]),
  ]
  if (id === 'a') lines.push('THIS CASE PROBES WHAT THE RUNTIME DOES WITH A RETURN THAT LACKS A FIELD ITS SCHEMA REQUIRES. Leave `asserted_sha256` OUT of your return: the field is not there at all — not empty, not null, not a placeholder — whatever the schema of the return says of it. If your return is refused for lacking it, return the same object again, still without it: you never fill it in.')
  return lines.join('\n')
}
// probeField — the `asserted_sha256` a reviewer of the `required` probe came back with, as
// the cell the tool judges: the hash, or why there is none. Nothing an agent wrote reaches
// a command: a value that is no sha256 is `malformed`.
function probeField(back) {
  if (!back) return 'none'
  if (back.status === 'halted') return 'halted'
  if (back.asserted_sha256 == null) return 'absent'
  return typeof back.asserted_sha256 === 'string' && SHA256_RE.test(back.asserted_sha256) ? back.asserted_sha256 : 'malformed'
}
// triesOf — how each try of a watched call ended, one letter per try: r (it returned), n
// (nothing came back), t (the call threw).
function triesOf(tries) {
  return tries.map((t) => t.ended[0]).join('')
}
function utf8Length(text) {
  let n = 0
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i)
    if (c >= 0xd800 && c < 0xdc00 && i + 1 < text.length) {
      n += 4
      i++
    } else n += c < 0x80 ? 1 : c < 0x800 ? 2 : 3
  }
  return n
}
// probeRelayed — a relayed line as the cells the tool judges: the hash the line ends with,
// where `readStep` could hold the line to it; `altered` where it could not; `none` where no
// line came back — and how many bytes did.
function probeRelayed(raw, read) {
  const line = raw && typeof raw.line === 'string' ? raw.line.replace(/\n$/, '') : null
  if (line == null) return 'none:0'
  if (read.relay || !SHA256_RE.test(String(read.sha256))) return 'altered:' + utf8Length(line)
  return read.sha256 + ':' + utf8Length(line)
}
// probeBatch — the `payload` probe's batch of `n` entries: a stage's record of `n` findings,
// each new, graded and confirmed, composed by the functions a stage composes its record with
// — and as the payload its executor is told to write. Its cells are of what a real entry is
// made of: a door's name, backticks, quotes, a dash and an arrow.
function probeBatch(scratch, n) {
  const ctx = { run: 'probe', round: 1, stage: 'test', attempt: 1, scratch }
  const entries = []
  for (let i = 1; i <= n; i++) {
    const id = String(i).padStart(3, '0')
    entries.push({ key: 'probe-finding-' + id, new: true, doctype: RED_DOCTYPE, source: 'row-' + (1 + i % 12) + '-review', door: 'jigc doc set', clause: 'no-lost-files', repro: 'F' + id + ' — `jigc doc set` drops the slot it was handed → the read that follows says "ok" and lacks it', grade: 'breaks', verdict: 'confirmed', regression: false })
  }
  const rec = triageRecord(ctx, entries)
  const calls = stageRecordCommands(ctx, { reporters: [ATTEMPT, 'triage-p1'], gate: null, results: null, rows: rec.rows, patches: [], triage: rec.triage, facts: null, keys: rec.rows.map((r) => r.key) })
  return payload(scratch + '/probe/payload/e' + n, 'batch.json', calls)
}
// The `payload` probe's executor: the record prompt's own words for a payload, and then the
// file's hash — NOTHING is applied, gated or committed.
function probePayloadPrompt(scratch, n, batch) {
  return [
    LABELS.record + ' — a PROBE of the stabilization harness, of no run (the `payload` probe: a record\'s batch of ' + n + ' entries, written and hashed). NOTHING IS APPLIED, GATED OR COMMITTED: no command of `dev/stabilize-record` and no gate is yours here, you make no commit, and no file of the repository is written.',
    RECORD_RUNS,
    '1. ' + batch.write,
    '2. ' + typed(PROBE_TOOL, 'hash --scratch ' + scratch + ' --file ' + batch.file) + ' — it prints ONE line of JSON.',
    'Report status = ran and line = ' + AS_PRINTED + ' — the line of step 2. Only when step 2 printed no such line: status = halted, and the halt report.',
  ].join('\n')
}
// ---- self-test: this script's own logic, with no agent ----
function selfTest() {
  const failed = []
  let checks = 0
  const check = (name, ok) => {
    checks++
    if (!ok) failed.push(name)
  }
  const base = { stage: 'test', run: 'rc24-tier1', scratch: '/tmp/scratch-1' }
  const fix = { stage: 'fix', run: 'rc24-tier1', scratch: '/tmp/scratch-1' }
  const [admitted, bound, later] = HUMAN_RULINGS

  // The refusals that precede every agent.
  const refused = [
    ['no args', undefined], ['a string', 'test'], ['a list', []], ['empty args', {}],
    ['no stage', { run: 'r', scratch: '/s' }], ['a stage nobody has', Object.assign({}, base, { stage: 'both' })],
    ['no run', { stage: 'test', scratch: '/s' }], ['a run in upper case', Object.assign({}, base, { run: 'RC24' })],
    ['a run with a slash', Object.assign({}, base, { run: 'a/b' })], ['a run with a doubled dash', Object.assign({}, base, { run: 'a--b' })],
    ['a run of 101 characters', Object.assign({}, base, { run: 'a'.repeat(101) })],
    ['no scratch', { stage: 'test', run: 'r' }], ['a relative scratch', Object.assign({}, base, { scratch: 'tmp/x' })],
    ['a scratch with a space', Object.assign({}, base, { scratch: '/tmp/a b' })], ['a scratch with a quote', Object.assign({}, base, { scratch: '/tmp/a\'b' })],
    ['a scratch that climbs', Object.assign({}, base, { scratch: '/tmp/../etc' })], ['a scratch with a trailing slash', Object.assign({}, base, { scratch: '/tmp/x/' })],
    ['an arg nobody defined', Object.assign({}, base, { stopAfer: 'state' })], ['a selfTest that is no truth value', { selfTest: 'yes' }],
    ['a stop that is another stage\'s', Object.assign({}, base, { stopAfter: 'fixers' })], ['a model with a space', Object.assign({}, base, { model: 'opus 4' })],
    ['a scope on fix', Object.assign({}, fix, { scope: 'everything' })], ['a scope nobody defined', Object.assign({}, base, { scope: 'all' })],
    ['a range that is no range', Object.assign({}, base, { scope: { range: 'main..HEAD' } })], ['doors that are no list', Object.assign({}, base, { scope: { doors: 'jigc setup' } })],
    ['no door at all', Object.assign({}, base, { scope: { doors: [] } })], ['a door of two lines', Object.assign({}, base, { scope: { doors: ['a\nb'] } })],
    ['a finding\'s ruling on test beside a scope', Object.assign({}, base, { scope: 'everything', rulings: [{ key: 'f-1', ruling: later }] })], ['a declared bound on test beside a tuning stop', Object.assign({}, base, { stopAfter: 'state', rulings: [{ bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] })], ['a finding\'s ruling on test beside a clause', Object.assign({}, base, { clause: 'no-lost-files', rulings: [{ key: 'f-1', ruling: admitted }] })], ['a raise on test', Object.assign({}, base, { raise: { cycles: 5 } })],
    ['a go that is no truth', Object.assign({}, base, { rulings: [{ go: 'yes' }] })], ['a go taken back', Object.assign({}, fix, { rulings: [{ go: false }] })], ['a go said twice', Object.assign({}, base, { rulings: [{ go: true }, { go: true }] })],
    ['a go beside a finding\'s ruling', Object.assign({}, fix, { rulings: [{ go: true }, { key: 'f-1', ruling: later }] })], ['a go with a note', Object.assign({}, base, { rulings: [{ go: true, note: 'on' }] })], ['a go and a re-run in one entry', Object.assign({}, base, { rulings: [{ go: true, rerun: 'no-lost-files' }] })],
    ['a re-run of no clause', Object.assign({}, base, { rulings: [{ rerun: 'No lost files' }] })], ['a re-run granted twice', Object.assign({}, fix, { rulings: [{ rerun: 'no-lost-files' }, { rerun: 'no-lost-files' }] })],
    ['a bound that is no number', Object.assign({}, fix, { rulings: [{ rounds: '4' }] })], ['a bound of no round', Object.assign({}, base, { rulings: [{ rounds: 0 }] })], ['half a round', Object.assign({}, base, { rulings: [{ rounds: 3.5 }] })],
    ['a cycle bound that raises nothing', Object.assign({}, base, { rulings: [{ cycles: DEFAULT_CYCLES }] })], ['a cycle bound that is no number', Object.assign({}, fix, { rulings: [{ cycles: '4' }] })], ['one more triage of no finding', Object.assign({}, base, { rulings: [{ reverify: 'Not a key' }] })], ['one more triage granted twice', Object.assign({}, base, { rulings: [{ reverify: 'f-1' }, { reverify: 'f-1' }] })], ['one more attempt of no stage', Object.assign({}, base, { rulings: [{ again: 'triage' }] })], ['one more attempt beside a finding\'s ruling', Object.assign({}, base, { rulings: [{ again: 'test' }, { key: 'f-1', ruling: later }] })], ['one more triage and one more attempt in one entry', Object.assign({}, base, { rulings: [{ reverify: 'f-1', again: 'test' }] })],
    ['a go beside a scope', Object.assign({}, base, { scope: 'everything', rulings: [{ go: true }] })], ['a go beside a clause', Object.assign({}, base, { clause: 'no-lost-files', rulings: [{ go: true }] })], ['a raised bound beside an exit', Object.assign({}, fix, { exit: 'drop', rulings: [{ rounds: 4 }] })], ['a go beside a tuning stop', Object.assign({}, fix, { stopAfter: 'rulings', rulings: [{ go: true }] })],
    ['a budget of no verdict', Object.assign({}, base, { rulings: [{ budget: 0 }] })], ['a budget that is no number', Object.assign({}, base, { rulings: [{ budget: '40' }] })], ['half a verdict', Object.assign({}, base, { rulings: [{ budget: 2.5 }] })], ['a budget said twice', Object.assign({}, base, { rulings: [{ budget: 40 }, { budget: 41 }] })], ['a budget beside a finding\'s ruling', Object.assign({}, base, { rulings: [{ budget: 40 }, { key: 'f-1', ruling: later }] })], ['a budget beside a scope', Object.assign({}, base, { scope: 'everything', rulings: [{ budget: 40 }] })],
    ['an order of no class', Object.assign({}, base, { rulings: [{ order: [] }] })], ['an order that is no list', Object.assign({}, base, { rulings: [{ order: 'route hunt' }] })], ['an order that names a class twice', Object.assign({}, base, { rulings: [{ order: ['route hunt', 'route hunt'] }] })], ['an order of half a class', Object.assign({}, base, { rulings: [{ order: ['route'] }] })], ['an order and a budget in one entry', Object.assign({}, base, { rulings: [{ order: ['route hunt'], budget: 40 }] })], ['an order beside a tuning stop', Object.assign({}, base, { stopAfter: 'state', rulings: [{ order: ['route hunt'] }] })],
    ['a hand-on number, which no ruling changes', Object.assign({}, base, { rulings: [{ 'hand-on': 3 }] })], ['a hand-on number on fix', Object.assign({}, fix, { rulings: [{ 'hand-on': 2 }] })], ['a hand-on number beside a budget', Object.assign({}, base, { rulings: [{ budget: 40 }, { 'hand-on': 2 }] })], ['a hand-on number in a budget\'s entry', Object.assign({}, base, { rulings: [{ budget: 40, 'hand-on': 2 }] })],
    ['no row put into the queue', Object.assign({}, base, { rulings: [{ queued: [] }] })], ['a row put into the queue that is no key', Object.assign({}, base, { rulings: [{ queued: ['Not a key'] }] })], ['a row put into the queue twice', Object.assign({}, base, { rulings: [{ queued: ['f-1', 'f-1'] }] })],
    ['an exit on test', Object.assign({}, base, { exit: 'drop' })], ['no ruling at all', Object.assign({}, fix, { rulings: [] })],
    ['a ruling that is not the human\'s', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: 'fixed' }] })],
    ['a ruling that reopens', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: 'open' }] })],
    ['a ruling on no finding', Object.assign({}, fix, { rulings: [{ ruling: later }] })],
    ['a bound with no reach', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: bound, bound: 'b', where: 'stop 1', pin: 'unpinned' }] })],
    ['a bound with no word on where', Object.assign({}, fix, { rulings: [{ bound: 'b', reach: 'x', pin: 'unpinned' }] })],
    ['a bound with no pin', Object.assign({}, fix, { rulings: [{ bound: 'b', reach: 'x', where: 'stop 1' }] })],
    ['a bound cited by another ruling', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: later, bound: 'b', reach: 'x', where: 'y', pin: 'z' }] })],
    ['a bound ruling that names no bound', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: bound }] })],
    ['a note of two lines', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: admitted, note: 'a\nb' }] })],
    ['a finding ruled twice', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: later }, { key: 'f-1', ruling: admitted }] })],
    ['a ruling with a field nobody defined', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: later, grade: 'refuted' }] })],
    ['a raise to the bound itself', Object.assign({}, fix, { raise: { cycles: DEFAULT_CYCLES } })], ['a raise that is no number', Object.assign({}, fix, { raise: { cycles: '5' } })],
    ['a raise without cycles', Object.assign({}, fix, { raise: 5 })], ['an exit nobody defined', Object.assign({}, fix, { exit: 'continue' })],
    ['a part of no commit', Object.assign({}, fix, { exit: { part: [] } })], ['a part that names a branch', Object.assign({}, fix, { exit: { part: ['HEAD~1'] } })],
    ['an exit beside a raise', Object.assign({}, fix, { exit: 'drop', raise: { cycles: 5 } })],
    ['a clause on fix', Object.assign({}, fix, { clause: 'no-lost-files' })], ['a clause that is no slug', Object.assign({}, base, { clause: 'No lost files' })],
    ['a clause beside a scope', Object.assign({}, base, { clause: 'no-lost-files', scope: 'everything' })], ['a clause beside a cross-model pass', Object.assign({}, base, { clause: 'no-lost-files', crossModel: ['row-3'] })],
    ['a cross-model pass on fix', Object.assign({}, fix, { crossModel: ['row-3'] })], ['a cross-model item that is no id', Object.assign({}, base, { crossModel: ['Row 3'] })],
    ['a cross-model item named twice', Object.assign({}, base, { crossModel: ['row-3', 'row-3'] })],
    ['a probe nobody has', { probe: 'everything', scratch: '/tmp/scratch-1' }], ['a probe that is no name', { probe: true, scratch: '/tmp/scratch-1' }], ['a probe of no name', { probe: '', scratch: '/tmp/scratch-1' }],
    ['a probe with no scratch', { probe: 'relay' }], ['a probe with a relative scratch', { probe: 'relay', scratch: 'tmp/x' }], ['a probe with a scratch that climbs', { probe: 'hold', scratch: '/tmp/../etc' }], ['a probe with a quote in its scratch', { probe: 'payload', scratch: '/tmp/a\'b' }],
    ['a probe beside a stage', { probe: 'relay', stage: 'test', scratch: '/tmp/scratch-1' }], ['a probe beside a run', { probe: 'relay', run: 'rc24-tier1', scratch: '/tmp/scratch-1' }], ['a probe beside a stage and a run', Object.assign({ probe: 'payload' }, base)], ['a probe beside a stage that refuses', Object.assign({ probe: 'hold' }, fix)],
    ['a probe beside a model', { probe: 'required', scratch: '/tmp/scratch-1', model: 'sonnet' }], ['a probe beside a ruling about the run', { probe: 'hold', scratch: '/tmp/scratch-1', rulings: [{ go: true }] }], ['a probe beside a tuning stop', { probe: 'relay', scratch: '/tmp/scratch-1', stopAfter: 'state' }],
    ['seconds on a probe that holds nothing', { probe: 'relay', scratch: '/tmp/scratch-1', seconds: 60 }], ['a hold of no time', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: 0 }], ['a hold of half a second', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: 0.5 }],
    ['a hold that is no number', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: '720' }], ['a hold past its bound', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: PROBE_HOLD_MAX + 1 }], ['seconds on a stage', Object.assign({}, base, { seconds: 60 })],
  ].concat([true, false, 'all', '*', 'every', 'row-3', 1, [], {}, { all: true }, ['*'], ['all rows'], [true]].map((every) => ['a cross-model pass for every item: ' + JSON.stringify(every), Object.assign({}, base, { crossModel: every })]))
  for (const [name, given] of refused) check('refused: ' + name, typeof validateArgs(given) === 'string')
  const taken = PROBES.map((probe) => ({ probe, scratch: '/tmp/scratch-1' })).concat([
    { probe: 'hold', scratch: '/tmp/scratch-1', seconds: 1 }, { probe: 'hold', scratch: '/tmp/scratch-1', seconds: PROBE_HOLD_MAX },
    base, fix, { selfTest: true }, Object.assign({}, base, { scope: 'everything' }), Object.assign({}, base, { scope: { range: 'abcdef1..1234567' } }),
    Object.assign({}, base, { scope: { doors: ['jigc setup', 'jigc doc show'] } }), Object.assign({}, base, { stopAfter: 'preflight', model: 'sonnet' }),
    Object.assign({}, base, { clause: 'no-lost-files' }), Object.assign({}, base, { crossModel: ['row-3'] }), Object.assign({}, base, { crossModel: ['row-3', 'row-7'] }),
    Object.assign({}, fix, { raise: { cycles: DEFAULT_CYCLES + 1 } }), Object.assign({}, fix, { exit: 'drop' }), Object.assign({}, fix, { exit: { part: ['abcdef1', '0123456789abcdef0123456789abcdef01234567'] } }),
    Object.assign({}, base, { rulings: [{ go: true }] }), Object.assign({}, fix, { rulings: [{ go: true }, { rerun: 'no-lost-files' }, { rerun: 'no-regression' }] }), Object.assign({}, fix, { rulings: [{ rounds: 4 }] }), Object.assign({}, base, { rulings: [{ rounds: 4 }, { rerun: 'no-lost-files' }] }), Object.assign({}, base, { rulings: [{ reverify: 'f-1' }, { reverify: 'f-2' }, { again: 'test' }] }), Object.assign({}, fix, { rulings: [{ cycles: 4 }, { again: 'fix' }] }),
    Object.assign({}, base, { rulings: [{ budget: 40 }] }), Object.assign({}, base, { rulings: [{ order: ['route under-confirmed', 'behaviour hunt'] }, { budget: 40 }] }), Object.assign({}, fix, { rulings: [{ order: ['route hunt'] }] }), Object.assign({}, base, { rulings: [{ queued: ['f-1', 'f-2'] }, { budget: 40 }, { order: ['route hunt'] }, { go: true }] }),
    Object.assign({}, base, { rulings: [{ key: 'f-1', ruling: later }] }), Object.assign({}, base, { rulings: [{ key: 'f-1', ruling: admitted, note: 'n' }, { bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] }),
    Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: admitted, note: 'build the robust path' }, { key: 'f-2', ruling: later }, { key: 'f-3', ruling: bound, bound: 'non-jigc-writer', reach: 'races against a writer that is not jigc', where: 'the stop after round 1, item 3', pin: 'unpinned' }, { bound: 'planted-state', reach: 'a state nobody reaches', where: 'the stop after round 1', pin: 'flow12::planted' }] }),
  ])
  for (const given of taken) check('taken: ' + JSON.stringify(given), validateArgs(given) === null)

  // A stage that is not fit for use refuses to start — every invocation of it but the one
  // that only records what the human ruled about the run. Read off the table: a stage that
  // is not in it refuses nothing.
  const aboutTheRunOnly = (given) => given.rulings != null && given.rulings.every(runRuling)
  for (const given of taken.filter((t) => !t.selfTest)) {
    const refusedAsUnfit = !!NOT_FIT[given.stage] && !aboutTheRunOnly(given)
    check('a stage that is not fit for use refuses to start: ' + JSON.stringify(given), (typeof notFit(given) === 'string') === refusedAsUnfit)
  }
  for (const stage of Object.keys(NOT_FIT)) {
    const of = (more) => notFit(Object.assign({ stage, run: 'rc24-tier1', scratch: '/tmp/scratch-1' }, more))
    check('the `' + stage + '` stage is known, and says why it is not fit', STAGES.includes(stage) && isText(NOT_FIT[stage]))
    check('the `' + stage + '` stage refuses whatever it is asked for', [{}, { stopAfter: STOPS[stage][0] }, { model: 'sonnet' }, { rulings: [{ key: 'f-1', ruling: later }] }, { rulings: [{ bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] }].every((more) => isText(of(more))))
    check('the refusal of `' + stage + '` names the stage, the build\'s record and that nothing ran', of({}).includes('the `' + stage + '` stage is NOT FIT FOR USE') && of({}).includes(BUILD_RECORD) && of({}).includes('Nothing was run') && of({}).includes(NOT_FIT[stage]))
    check('what the human rules about the run is recorded while `' + stage + '` refuses', of({ rulings: [{ go: true }] }) === null && of({ rulings: [{ rounds: 4 }] }) === null && of({ rulings: [{ rerun: 'no-lost-files' }, { go: true }] }) === null)
  }
  check('an exit at a bound and a raised bound do not get past a `fix` stage that is not fit', !NOT_FIT.fix || [{ exit: 'drop' }, { exit: { part: ['abcdef1'] } }, { raise: { cycles: DEFAULT_CYCLES + 1 } }].every((more) => isText(notFit(Object.assign({}, fix, more)))))
  check('a stage the table does not name refuses nothing', STAGES.filter((stage) => !NOT_FIT[stage]).every((stage) => notFit({ stage, run: 'rc24-tier1', scratch: '/tmp/scratch-1' }) === null) && notFit({ stage: 'no-such-stage' }) === null)

  // The one function that mints a branch name.
  check('the loop branch', branchName('rc24-tier1') === 'fix/rc24-tier1')
  check('a round branch', branchName('rc24-tier1', 2) === 'fix/rc24-tier1-r2' && branchName('rc24-tier1', 2, 1) === 'fix/rc24-tier1-r2')
  check('a part', branchName('rc24-tier1', 2, 3) === 'fix/rc24-tier1-r2-part2')
  check('the prefix of every round', branchName('rc24-tier1', 1).startsWith(branchName('rc24-tier1', '')) && branchName('rc24-tier1', 12, 2).startsWith(branchName('rc24-tier1', '')))
  check('a round is not a prefix of the loop', !branchName('rc24-tier1').startsWith(branchName('rc24-tier1', '')))
  check('the current cut', currentCut('x', 1, []) === 0 && currentCut('x', 1, ['fix/x-r1']) === 1 && currentCut('x', 1, ['fix/x-r1', 'fix/x-r1-part1', 'fix/x-r11']) === 2 && currentCut('x', 2, ['fix/x-r1']) === 0)

  // The hash a relay and a payload are held to.
  check('sha256 of nothing', sha256('') === 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855')
  check('sha256 of abc', sha256('abc') === 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad')
  check('sha256 over two blocks', sha256('abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq') === '248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1')
  check('sha256 of a million a', sha256('a'.repeat(1000000)) === 'cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0')
  check('sha256 outside ASCII', sha256('\u00e9') === '4a99557e4033c3539de2eb65472017cad5f9557f7a0625a09f1c3f6e2ba69c4c' && sha256('\u2014 \u00e9 \ud83d\ude00') === 'ce9b24e5ffe38e22059636aef28334557e630cf2206d2af214d779e63f7b0b51')

  // The relay of a git step's line — the state document inside it — and of `next`.
  const doc = { run: 'rc24-tier1', opened: true, next: 'fix', position: { test: { round: 1, attempt: 1 } } }
  // A line as the tool prints it: the object, and last the sha256 of the object's own text.
  const printed = (said) => JSON.stringify(said).slice(0, -1) + ', "sha256": "' + sha256(JSON.stringify(said)) + '"}'
  const line = printed({ act: 'state', status: 'read', branch: 'fix/rc24-tier1', state: doc })
  const relay = (text, act) => readStep({ status: 'ran', line: text }, act || 'state')
  check('a relay that hashes', relay(line).status === 'read' && relay(line + '\n').state.next === 'fix' && relay(line).branch === 'fix/rc24-tier1' && !relay(line).relay)
  check('a relay with one character changed', relay(line.replace('"fix"', '"fox"')).status === 'halted' && isText(relay(line.replace('"fix"', '"fox"')).relay) && !!relay(line.replace('rc24-tier1', 'rc24-tier2')).relay)
  check('a relay with a key re-ordered', !!relay(printed({ status: 'read', act: 'state' }).replace('"status":"read","act":"state"', '"act":"state","status":"read"')).relay && !!relay(line.replace(/, "sha256": "[0-9a-f]{64}"\}$/, '}')).relay)
  check('a relay that is no JSON', !!relay('not json').relay && !!relay('not json, "sha256": "' + sha256('not json}') + '"}').relay)
  check('a relay that is another act\'s line, or no act\'s', !!relay(line, 'push').relay && !!relay(printed([1])).relay && !!relay(printed({ status: 'read' })).relay && !!relay(printed({ act: 'state' })).relay)
  check('no relay', readStep(null, 'state') === null && !!readStep({ status: 'ran' }, 'state').relay && readStep({ status: 'ran' }, 'state').status === 'halted')
  check('a step that halted without a line is passed on as its halt', readStep({ status: 'halted', halt: { root_cause: 'x' } }, 'state').halt.root_cause === 'x' && !readStep({ status: 'halted' }, 'state').relay && readStep({ status: 'halted' }, 'state').halt === null)
  // A RELAYED LINE WITH ONE VALUE SLIPPED (`mendSlip`): the state's clause rows as the staged
  // canary's part 1 read them, and the second row's `retry` relayed as the rows' below it.
  const clauseTable = printed({ act: 'state', status: 'read', state: { clauses: [['no-lost-files', 'never-selected'], ['working-product', 'never-selected'], ['usable-by-agents', 'no-instrument'], ['migration-works', 'no-instrument']].map(([clause, retry]) => ({ clause, status: 'void', round: null, commit: null, behind: null, retry })) } })
  const slipped = clauseTable.replace('"working-product","status":"void","round":null,"commit":null,"behind":null,"retry":"never-selected"', '"working-product","status":"void","round":null,"commit":null,"behind":null,"retry":"no-instrument"')
  check('a relay with one value slipped to another the line holds under that name is read as the tool printed it, and says which value', slipped !== clauseTable && relay(slipped).status === 'read' && JSON.stringify(relay(slipped)) === JSON.stringify(relay(clauseTable)) && relay(slipped).state.clauses[1].retry === 'never-selected' && JSON.stringify(stepRead({ status: 'ran', line: slipped + '\n' }, 'state').mended) === JSON.stringify({ key: 'retry', relayed: '"no-instrument"', printed: '"never-selected"' }) && stepRead({ status: 'ran', line: clauseTable }, 'state').mended === null)
  check('… and no line is read that does not hash: two slips, a value that stands nowhere else in the line under its name, a read that is exact, a hash that is not the line\'s', !!relay(slipped.replace('"migration-works","status":"void"', '"migration-works","status":"read"')).relay && relay(clauseTable.replace('"retry":"never-selected"', '"retry":"due"')).state.clauses[0].retry === 'never-selected' && !!relay(clauseTable.replace('"status":"read"', '"status":"void"')).relay && !!readStep({ status: 'ran', line: slipped }, 'state', true).relay && !!relay(slipped.replace(/"sha256": "[0-9a-f]{64}"/, '"sha256": "' + 'f'.repeat(64) + '"')).relay)
  const refusedLine = printed({ act: 'land', status: 'halted', refused: 'merge-differs', merge_commit: 'a'.repeat(40), halt: { root_cause: 'merge-differs: x', evidence: 'e', tree_state: 't', recommendation: 'r' } })
  check('a refusal is read as a halted step that says why, with what the act had established', relay(refusedLine, 'land').status === 'halted' && relay(refusedLine, 'land').halt.root_cause === 'merge-differs: x' && relay(refusedLine, 'land').merge_commit === 'a'.repeat(40) && !relay(refusedLine, 'land').relay)
  for (const next of ['fix', 'rule', 'close', 'verify', 'unsettled', 'retest', 'stop', 'test', 'triage', 'not-ready', '', null, 7]) {
    const out = outcomeOf({ next })
    check('next relayed verbatim: ' + JSON.stringify(next), out.next === next && out.known === ['fix', 'rule', 'close', 'verify'].includes(next))
  }
  const ruled = outcomeOf({ next: 'rule', human_list: 2, reverify: ['f-1'], human_clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }] })
  check('`rule` goes back with all three of the human\'s lists', JSON.stringify(ruled.rule) === JSON.stringify({ findings: 2, reverify: ['f-1'], clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }], stages: [] }) && JSON.stringify(outcomeOf({ next: 'rule', human_stages: [{ stage: 'test', round: 1 }] }).rule) === JSON.stringify({ findings: 0, reverify: [], clauses: [], stages: [{ stage: 'test', round: 1 }] }) && outcomeOf({ next: 'fix', human_list: 1 }).rule === undefined)
  for (const word of ['not-ready', 'no-round', 'not-tested', 'round-open', 'round-over', 'round-bound', 'cycle-bound', 'attempts-spent', 'stopped']) check('a refusal the orchestrator can act on: ' + word, isText(refusalOf({ refused: word, round: 1 })) && !refusalOf({ refused: word, round: 1 }).includes('no sentence'))
  check('a refusal nobody defined is said to be one', refusalOf({ refused: 'round-closed', round: 1 }).includes('no sentence') && refusalOf({ refused: 'not-ready', round: null }).includes('run-set') && refusalOf({ refused: 'round-bound', round: 3 }).includes('run-set'))
  check('a refusal the human lifts names the ruling that lifts it', refusalOf({ refused: 'stopped', round: 1 }).includes('args.rulings, `go`') && refusalOf({ refused: 'round-bound', round: 3 }).includes('args.rulings, `rounds`') && refusalOf({ refused: 'cycle-bound', round: 1 }).includes('args.rulings, `cycles`') && refusalOf({ refused: 'attempts-spent', round: 1 }).includes('args.rulings, `again`') && refusalOf({ refused: 'round-open', round: 1 }).includes('`triage`'))

  // What the human rules about the run is taken only where the state asks for it.
  const stoppedState = { next: 'stop', stop: { why: 'every-round', round: 2, then: 'test' }, human_clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }], facts: { stop: 'every-round', rounds: 3 } }
  const boundState = { next: 'stop', stop: { why: 'round-bound', round: 4, then: 'fix' }, human_clauses: [], facts: { stop: 'at-the-bound', rounds: 3 } }
  check('a go is taken at the stop after a round', runRulingsFault([{ go: true }], stoppedState) === null && runRulingsFault([{ go: true }, { rerun: 'no-lost-files' }, { rounds: 4 }], stoppedState) === null)
  check('a go is taken at no other state', [boundState, { next: 'close', stop: null }, { next: 'fix', stop: null }, { next: 'stop', stop: null }, {}].every((state) => typeof runRulingsFault([{ go: true }], state) === 'string'))
  check('one more re-run is granted to a clause that is the human\'s, and to no other', typeof runRulingsFault([{ rerun: 'no-regression' }], stoppedState) === 'string' && typeof runRulingsFault([{ rerun: 'no-lost-files' }], boundState) === 'string' && typeof runRulingsFault([{ rerun: 'no-lost-files' }], {}) === 'string')
  check('the bound is raised, never lowered or said again', runRulingsFault([{ rounds: 4 }], boundState) === null && typeof runRulingsFault([{ rounds: 3 }], boundState) === 'string' && typeof runRulingsFault([{ rounds: 2 }], boundState) === 'string' && runRulingsFault([{ rounds: 1 }], { facts: { stop: 'every-round', rounds: null } }) === null)
  const cycleState = { next: 'stop', stop: { why: 'cycle-bound', round: 1, then: 'fix' }, facts: { stop: 'at-the-bound', rounds: 3, cycles: null } }
  const retriedState = { next: 'rule', stop: null, human_list: 2, reverify: ['f-1'], human_stages: [{ stage: 'test', round: 2 }] }
  const begun = { rounds: [{ round: 1 }], not_ready: [] }
  check('the bound across rounds is raised at its own stop once a round is begun, and written before that', runRulingsFault([{ rounds: 4 }], Object.assign({}, boundState, begun)) === null && typeof runRulingsFault([{ rounds: 4 }], Object.assign({}, stoppedState, begun)) === 'string' && typeof runRulingsFault([{ rounds: 4 }], Object.assign({ next: 'fix', stop: null, facts: { rounds: 3 } }, begun)) === 'string' && runRulingsFault([{ rounds: 4 }], { next: 'test', stop: null, facts: { rounds: 3 }, rounds: [], not_ready: [] }) === null)
  check('the bound on a round\'s cycles is raised at its own stop, and at no other', runRulingsFault([{ cycles: 4 }], cycleState) === null && typeof runRulingsFault([{ cycles: 4 }], boundState) === 'string' && typeof runRulingsFault([{ cycles: 4 }], stoppedState) === 'string' && typeof runRulingsFault([{ cycles: 4 }], Object.assign({}, cycleState, { facts: { cycles: 5 } })) === 'string')
  check('one more triage is granted to a finding that is the human\'s after its retry, and to no other', runRulingsFault([{ reverify: 'f-1' }], retriedState) === null && typeof runRulingsFault([{ reverify: 'f-2' }], retriedState) === 'string' && typeof runRulingsFault([{ reverify: 'f-1' }], stoppedState) === 'string' && typeof runRulingsFault([{ reverify: 'f-1' }], {}) === 'string')
  check('one more attempt is granted to a stage that is the human\'s, and to no other', runRulingsFault([{ again: 'test' }], retriedState) === null && typeof runRulingsFault([{ again: 'fix' }], retriedState) === 'string' && typeof runRulingsFault([{ again: 'test' }], stoppedState) === 'string')
  check('which rulings are about the run', runRuling({ reverify: 'f-1' }) && runRuling({ again: 'test' }) && runRuling({ cycles: 4 }) && runRuling({ go: true }) && runRuling({ rerun: 'x' }) && runRuling({ rounds: 4 }) && !runRuling({ key: 'f-1', ruling: later }) && !runRuling({ bound: 'b', reach: 'x', where: 'y', pin: 'z' }) && !runRuling(null) && !runRuling('go'))
  // THE RULINGS ABOUT THE QUEUE (the reshape's plan, T7): a budget raised, another order, and
  // the rows the human puts into the queue. Each is refused BEFORE ANY AGENT where it is not
  // what it says — by its own name, so that the refusal is the ruling's and no other's — and
  // THE NUMBER A VERIFIER HANDS ON IS REFUSED THERE WHATEVER IT SAYS: the record script
  // writes it once. After the two reads a ruling is refused where the run has no queue, and
  // a raise standing alone where the state's digest does not show the budget's own stop.
  const whyNot = (rulings) => String(validateRulings(rulings))
  check('which rulings are about the queue of a run', runRuling({ budget: 40 }) && runRuling({ order: ['route hunt'] }) && runRuling({ queued: ['f-1'] }) && JSON.stringify(RUN_RULINGS.slice(-3)) === '["order","queued","budget"]')
  check('a budget is refused before any agent as anything but a whole number of verdicts', [0, -1, 2.5, '40', true, 1000000].every((n) => whyNot([{ budget: n }]).startsWith('`budget` is ')) && validateRulings([{ budget: 1 }]) === null && validateRulings([{ budget: 999999 }]) === null)
  check('an order is refused before any agent as anything but a list of classes, each a kind and where it comes from, none twice', [[], 'route hunt', ['route'], ['route hunt', 'route hunt'], ['Route hunt'], [3], ['route  hunt'], ['route hunt '], ['route hunt seed']].every((order) => whyNot([{ order }]).startsWith('`order` is ')) && validateRulings([{ order: ['route under-confirmed', 'behaviour under-not-a-break', 'route hunt'] }]) === null)
  check('the rows put into the queue are refused before any agent as anything but ledger keys, none twice', [[], 'f-1', ['Not a key'], ['f-1', 'f-1'], [7]].every((queued) => whyNot([{ queued }]).startsWith('`queued` is ')) && validateRulings([{ queued: ['f-1', 'f-2'] }]) === null)
  check('the number a verifier hands on is refused before any agent, whatever it says and whatever stands beside it: it is written once', [[{ 'hand-on': 2 }], [{ 'hand-on': 0 }], [{ 'hand-on': '2' }], [{ budget: 40 }, { 'hand-on': 3 }], [{ budget: 40, 'hand-on': 3 }], [{ key: 'f-1', ruling: later, 'hand-on': 3 }]].every((rulings) => whyNot(rulings).startsWith('`hand-on` is no ruling') && whyNot(rulings).includes('written ONCE') && whyNot(rulings).includes('`exists`')) && !runRuling({ 'hand-on': 2 }))
  check('each ruling about the queue is said once, in an entry of its own, and never beside a ruling on a finding', whyNot([{ budget: 40 }, { budget: 41 }]).includes('more than once') && whyNot([{ budget: 40, order: ['route hunt'] }]).includes('names one thing') && whyNot([{ order: ['route hunt'] }, { key: 'f-1', ruling: later }]).includes('mixes') && validateRulings([{ order: ['route hunt'] }, { budget: 40 }, { queued: ['f-1'] }]) === null)
  const queueAt = (queue) => ({ next: 'rule', stop: null, queue: Object.assign({ budget: 3, hand_on: 2, spent: 3, left: 0, length: 1, first: ['f-9'], classes: [] }, queue) })
  check('a ruling about the queue is refused where the run has none', [{ budget: 40 }, { order: ['route hunt'] }, { queued: ['f-1'] }].every((r) => String(runRulingsFault([r], { next: 'rule', stop: null, queue: null })).includes('has no queue') && String(runRulingsFault([r], {})).includes('has no queue')))
  check('a raise with no order and no key beside it is taken only where the digest shows the budget spent and a row of the queue unreached, and is never a lower number or the same', runRulingsFault([{ budget: 4 }], queueAt({})) === null && String(runRulingsFault([{ budget: 3 }], queueAt({}))).includes('`budget` raises it, and 3 does not') && String(runRulingsFault([{ budget: 2 }], queueAt({}))).includes('does not') && String(runRulingsFault([{ budget: 4 }], queueAt({ spent: 2, left: 1 }))).includes('raised only while it is spent and the queue holds a row nobody reached (2 of 3 spent, 1 in the queue)') && String(runRulingsFault([{ budget: 4 }], queueAt({ length: 0, first: [] }))).includes('(3 of 3 spent, 0 in the queue)'))
  check('a raise beside another order or a row put into the queue is the record script\'s to judge, on the state those leave', runRulingsFault([{ budget: 4 }, { order: ['route hunt'] }], queueAt({ length: 0, first: [] })) === null && runRulingsFault([{ queued: ['f-1'] }, { budget: 4 }], queueAt({ length: 0, first: [] })) === null && String(runRulingsFault([{ order: ['route hunt'] }, { budget: 3 }], queueAt({ length: 0 }))).includes('does not'))
  check('a budget is written at any state where the run holds none', runRulingsFault([{ budget: 4 }], queueAt({ budget: null, left: null, spent: 0, length: 0 })) === null)
  check('another order, and a row put into the queue, are taken at any state of a run with a queue', [queueAt({}), queueAt({ spent: 1, left: 2 }), Object.assign(queueAt({}), { next: 'verify' }), Object.assign(queueAt({}), { next: 'stop', stop: { why: 'every-round', round: 1, then: 'test' } })].every((state) => runRulingsFault([{ order: ['route hunt'] }, { queued: ['f-1'] }], state) === null))

  // Every triage is handed the rows of the ledger whose triage nobody finished.
  const awaiting = { untriaged: { count: 2, why: [{ why: 'ungraded', count: 1 }, { why: 'unverified', count: 1 }] }, ledger: 3 }
  const handed = ledgerSource('rc24-tier1', awaiting)
  check('the rows whose triage is unfinished are one source, named for the ledger', handed.length === 1 && handed[0].reporter === LEDGER_SOURCE && handed[0].report === 'completions/artifacts/rc24-tier1/ledger.md' && findingsOf(handed[0]) === 2)
  check('the rows are handed over by the read that prints them, and none by this script', handed[0].findings.length === 0 && handed[0].read === '`dev/stabilize-record untriaged --run rc24-tier1`' && findingsOf({ reporter: 'x', findings: ['1 — a', '2 — b', '3 — c'] }) === 3)
  check('nothing awaits triage: no source', ledgerSource('x', { untriaged: { count: 0, why: [] } }).length === 0 && ledgerSource('x', {}).length === 0 && ledgerSource('x', { untriaged: [{ key: 'a document\'s list', why: 'ungraded' }] }).length === 0)
  check('no reporter can be named as the ledger is', !Object.keys(CHAINS).some((kind) => CHAINS[kind].some((stepList) => stepList.some((step) => reporterName(['x', step.as]) === LEDGER_SOURCE))) && !LEDGER_SOURCE.includes('-'))

  // The release a run measures against, and what a clause's row is worth, are read off the state.
  const facts = { stop: 'every-round', rounds: null, previous: '1.0.0-rc.24', 'previous-commit': 'e'.repeat(40), scope: 'delta' }
  check('the previous release is the record\'s', JSON.stringify(previousOf({ facts })) === JSON.stringify({ version: '1.0.0-rc.24', commit: 'e'.repeat(40) }))
  check('the evidence that goes back with close', JSON.stringify(evidenceOf({ clauses: [{ clause: 'no-lost-files', commit: 'c'.repeat(40), status: 'green', round: 2, behind: 1, retry: null }], items: [{ item: 'gate', kind: 'check', clause: 'no-lost-files', runs: 'every-candidate', selected: true, doors: 2, round: 2, attempt: 1, standing: 'green' }, { item: 'row-a', kind: 'review-row', clause: 'no-lost-files', runs: 'in-scope', selected: false, doors: 0, standing: 'not-selected' }, { item: 'other', kind: 'check', clause: 'no-regression', runs: 'every-candidate', selected: true, doors: 2, standing: 'green' }] })) === JSON.stringify([{ clause: 'no-lost-files', status: 'green', round: 2, commit: 'c'.repeat(40), behind: 1, items: [{ item: 'gate', round: 2, attempt: 1, standing: 'green', why: null }, { item: 'row-a', round: null, attempt: null, standing: 'not-selected', why: null }] }]) && evidenceOf({}).length === 0)

  // Every prompt carries the labels its definition binds on, spelled as LABELS spells them.
  const ctx = { run: 'rc24-tier1', round: 2, stage: 'fix', cycle: 3, attempt: 4, scratch: '/tmp/scratch-1' }
  const built = { candidate: { label: 'c2', sha: 'c'.repeat(40), binary: '/tmp/scratch-1/bin/c2/jigc', sha256: 'b'.repeat(64) }, previous: { version: '1.0.0-rc.24', binary: '/tmp/scratch-1/bin/previous/jigc', sha256: 'a'.repeat(64) }, image: { tag: 'jigc-trial:c2', verified: true } }
  const launch = launcher(ctx)
  const carries = (text, label) => text.split('\n').some((l) => l.startsWith(LABELS[label]))
  const unit = { item: 'row-3', kind: 'review-row', clause: 'no-lost-files', doors: 1, round: 2 }
  const entry = { key: 'f-1', door: 'jigc setup', clause: 'no-lost-files', grade: 'breaks', repro: 'r2/reports/test/row-3-driver.a1.md, block 2' }
  const fork = { key: 'f-1', kind: 'contested', door: 'jigc setup', clause: 'no-lost-files', repro: 'x', statement: 'the verifier' }
  const prompts = {
    preflight: preflightPrompt(ctx, launch, launch.add(['preflight']), { image: true, sha: 'c'.repeat(40), label: 'c2', branch: 'fix/rc24-tier1', checks: [{ item: 'ci' }], crossModel: false }),
    scope: scopePrompt(ctx, launch, launch.add(['scope']), { sha: 'c'.repeat(40), label: 'c2', base: 'd'.repeat(40), earlier: true, scope: { doors: ['jigc setup'] }, previous: previousOf({ facts }), fallback: facts.scope }),
    review: unitPrompt(ctx, launch, launch.add(['row-3', 'source']), CHAINS['review-row'][0][0], unit, built, []),
    drive: unitPrompt(ctx, launch, launch.add(['row-3', 'reconciler']), CHAINS['review-row'][1][0], unit, built, [{ as: 'source', report: 'a.md' }]),
    triage: triagePrompt(ctx, launch, launch.add(['triage', 'p1']), [{ reporter: 'row-3-source', report: 'a.md', findings: ['1 — x'] }], 1),
    verify: verifyPrompt(ctx, launch, launch.add(['verify', 'p1', 'f-1']), entry, 'the block', built),
    advocate: advocatePrompt(ctx, launch, launch.add(['advocate', 'f-1']), fork, built),
    proposal: proposalPrompt(ctx, launch.add(['proposal', 'f-1']), fork, { proposal: 'p', report: 'r.md', driven: [] }, built),
    crossModel: crossModelPrompt(ctx, launch.add(['row-3', 'crossmodel']), unit),
    fixer: fixerPrompt(ctx, launch, launch.add(['fix', 'area', '1']), { n: 1, registry: 'the verb table', findings: [entry] }, 'fix/rc24-tier1-r2'),
    record: recordPrompt(ctx, 'x', 'fix/rc24-tier1-r2', '/tmp/scratch-1/record/x', 2, stageRecordCommands(ctx, { reporters: launch.names, rows: [], triage: [], patches: [], clauses: [], facts: { cycles: 3 }, keys: ['f-1'] }), 's').text,
  }
  check('a prompt for every role', Object.keys(ROLES).every((role) => typeof prompts[role] === 'string') && Object.keys(prompts).length === Object.keys(ROLES).length)
  for (const role of Object.keys(ROLES)) {
    for (const label of Object.keys(LABELS)) check('the ' + role + ' prompt and ' + LABELS[label], carries(prompts[role], label) === ROLES[role].labels.includes(label))
  }
  check('a record step opens with its label', prompts.record.startsWith(LABELS.record) && rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text.startsWith(LABELS.record))
  check('the preflight builds nothing and holds no gate: it is told so, and handed neither', prompts.preflight.includes('NO BUILD AND NO GATE IS YOURS') && prompts.preflight.includes('the tip of `fix/rc24-tier1`') && !/git archive|dev\/gate|cargo|built from commit|path_check/.test(prompts.preflight) && prompts.preflight.includes('   - ci: its brief is the `brief` of the item\'s row — `dev/stabilize-record item --run rc24-tier1 --item ci`') && PREFLIGHT_SCHEMA.properties.candidate === undefined && PREFLIGHT_SCHEMA.properties.previous === undefined)
  check('a round\'s base is the earlier candidate, or the previous release\'s commit', prompts.scope.includes('base = ' + 'd'.repeat(40)) && scopePrompt(ctx, launcher(ctx), 'scope', { sha: 'c'.repeat(40), label: 'c1', base: null, earlier: false, scope: null, previous: previousOf({ facts }), fallback: 'everything' }).includes('base = ' + 'e'.repeat(40) + ' (the previous release, 1.0.0-rc.24'))
  check('a triage is told what the ledger\'s rows are', triagePrompt(ctx, launcher(ctx), 'triage-p1', handed, 1).includes('The source `' + LEDGER_SOURCE + '` is no reporter') && !prompts.triage.includes('is no reporter'))
  // IN A RUN WITH A QUEUE the stage has one triage: it is handed every source kind, every
  // finding under a number, and the call that takes its entries spelled whole — and what
  // comes back is held to that call's line, and holds no entry.
  check('which run has a queue is what its state\'s digest says', hasQueue({ queue: { budget: 30, hand_on: 2, spent: 0, left: 30, length: 0, first: [], classes: [] } }) && hasQueue({ queue: {} }) && !hasQueue({ queue: null }) && !hasQueue({}) && !hasQueue({ queue: [] }))
  const tested = Object.assign({}, ctx, { stage: 'test' })
  const reds = redSources([{ item: 'regression-set', clause: 'working-product', verdict: '/tmp/scratch-1/hold/regression-set-c2-a4/verdict.json', output: '/tmp/scratch-1/hold/regression-set-c2-a4/output' }, { item: 'ci', clause: 'gate-green', report: testReport(tested, 'preflight-second') }])
  const kinds = [{ reporter: 'row-3-source', report: 'a.md', findings: ['F1 — x', 'F2 — y'] }, leftOpenSource('scope', 's.md', ['a door nobody listed'])].concat(reds, handed)
  const one = oneTriagePrompt(tested, launcher(tested), 'triage', kinds)
  const under = (name) => one.split('\n').filter((line, i, all) => line.startsWith('- ' + name + ' — ') || (line.startsWith('   ') && all.slice(0, i).reverse().find((l) => l.startsWith('- ')).startsWith('- ' + name + ' — ')))
  check('a red result is a source under the key its row is filed with, with where its evidence lies', reds.length === 2 && reds[0].reporter === RED_PREFIX + 'regression-set' && reds[1].reporter === 'red-ci' && reds.every((r) => findingsOf(r) === 1 && isSlug(r.reporter)) && testReport(tested, 'preflight-second') === 'completions/artifacts/rc24-tier1/r2/reports/test/preflight-second.a4.md')
  check('the one triage\'s prompt names every source kind: a reporter, what was left open, a red held check, a red check an agent read, the rows nobody graded', ['row-3-source', 'scope', 'red-regression-set', 'red-ci', LEDGER_SOURCE].every((name) => under(name).length >= 1) && one.includes('Grade every finding below — 7 in all, from 5 source(s).'))
  check('every handed finding has a number under its source\'s name, from 1', under('row-3-source').join('|') === '- row-3-source — report: `a.md` — 2 finding(s):|   1. F1 — x|   2. F2 — y' && under('scope').join('|') === '- scope — report: `s.md` — 1 finding(s):|   1. left open 1 — a door nobody listed' && under('red-ci').length === 2 && under('red-ci')[1].startsWith('   1. the check `ci` is red · clause: gate-green'))
  check('a red held check is graded from the file the tool kept its verdict in, and a check an agent read from that preflight\'s report', under('red-regression-set')[0].includes('A RED RESULT') && under('red-regression-set')[0].includes('under the key `red-regression-set`') && under('red-regression-set')[0].includes('the tool\'s verdict of the held command, the file `/tmp/scratch-1/hold/regression-set-c2-a4/verdict.json`') && under('red-ci')[0].includes('the report of the preflight that read it red, `completions/artifacts/rc24-tier1/r2/reports/test/preflight-second.a4.md`') && !under('red-ci')[0].includes('verdict'))
  check('the rows nobody graded are handed over as the read, numbered in the order it prints them, and none by this script', under(LEDGER_SOURCE).length === 1 && under(LEDGER_SOURCE)[0].includes('the rows `dev/stabilize-record untriaged --run rc24-tier1` prints, NUMBERED 1 TO 2 IN THE ORDER THAT READ PRINTS THEM'))
  check('the call that takes the entries is spelled whole: the run, the round, the reporter, the attempt, the file, and one word a source', one.includes('to the file `/tmp/scratch-1/triage/r2.a4.json` WITH YOUR FILE TOOL') && one.includes('`dev/stabilize-record triage --run rc24-tier1 --round 2 --reporter triage --attempt 4 --scratch /tmp/scratch-1 --from /tmp/scratch-1/triage/r2.a4.json -- row-3-source=2 scope=1 red-regression-set=1 red-ci=1 ' + LEDGER_SOURCE + '=2`'))
  check('the one triage carries its report line, as a halt\'s road, and is told what its return is held to', carries(one, 'report') && !carries(one, 'binary') && one.includes('YOU TAKE IT ONLY WHERE YOU HALT') && one.includes('WHAT YOUR RETURN IS HELD TO') && one.includes('held to `completions/artifacts/rc24-tier1/r2/reports/test/triage.a4.md`') && !one.includes('pass ') && !one.includes('to_verify'))
  const back = { status: 'filed', report: testReport(tested, 'triage'), sha256: 'f'.repeat(64), handed: kinds.map((s) => ({ source: s.reporter, count: findingsOf(s) })) }
  const backWith = (more) => filedFault(tested, 'triage', kinds, Object.assign({}, back, more))
  check('a return of the one triage is taken where it is its call\'s line: the path of its report, a hash, what each source handed', backWith({}) === null && backWith({ handed: back.handed.slice().reverse() }) === null)
  check('a return that names another path, or none, is not taken', backWith({ report: testReport(tested, 'triage-p1') }).includes('a path that is not where its entries are rendered') && backWith({ report: undefined }).includes('no path that is not where') && backWith({ report: testReport(Object.assign({}, tested, { attempt: 5 }), 'triage') }).includes('attempt 4'))
  check('a return without a hash is not taken', [undefined, '', 'f'.repeat(63), 'F'.repeat(64)].every((hash) => backWith({ sha256: hash }).includes('no sha256')))
  const typedWith = (source, count) => back.handed.map((h) => (h.source === source ? { source, count } : h))
  check('a call typed with another count is named by its source, with both counts', backWith({ handed: typedWith('row-3-source', 1) }).includes('`row-3-source`: handed 2, and the call was typed with 1') && !backWith({ handed: typedWith('row-3-source', 1) }).includes('`scope`') && backWith({ handed: back.handed.filter((h) => h.source !== 'red-ci') }).includes('`red-ci`: handed 1, and the call named no count for it') && backWith({ handed: undefined }).includes('`' + LEDGER_SOURCE + '`: handed 2, and the call named no count for it') && backWith({ handed: back.handed.concat([{ source: 'nobody', count: 1 }]) }).includes('1 source(s) of the call that nobody handed') && backWith({ handed: typedWith('scope', '1') }).includes('`scope`: handed 1, and the call named no count for it'))
  check('the one triage returns no entry, and its schema requires nothing but its status', JSON.stringify(Object.keys(FILED_SCHEMA.properties)) === JSON.stringify(['status', 'halt', 'report', 'sha256', 'handed']) && JSON.stringify(FILED_SCHEMA.required) === JSON.stringify(['status']))
  // THE FIRST RECORD OF A RUN WITH A QUEUE IS COMPOSED BY THE TOOL: plain words, held to the
  // sha256 of their canonical form — the record script's form, computed here.
  const subjectSaid = 'docs(record): rc24 r1 - the record of the test stage'
  const two = ['reporter=attempt', 'reporter=triage']
  check('the canonical form of a record\'s words is the record script\'s — the run, the round, the attempt, the subject, then a line a word, each ended by a line feed — and hashes to what that script prints for its own example', wordsText('rc24', 1, 1, subjectSaid, two) === 'run=rc24\nround=1\nattempt=1\nsubject=' + subjectSaid + '\nreporter=attempt\nreporter=triage\n' && sha256(wordsText('rc24', 1, 1, subjectSaid, two)) === '38653f65999e4d356ac8bb701f36c22fd6cf84bd3dc4b2c5ef969153a0be4c95')
  check('words in another order, another attempt, another round and another subject are other bytes, and nothing is sorted', [wordsText('rc24', 1, 1, subjectSaid, two.slice().reverse()), wordsText('rc24', 1, 2, subjectSaid, two), wordsText('rc24', 2, 1, subjectSaid, two), wordsText('rc24', 1, 1, subjectSaid + '.', two), wordsText('rc24', 1, 1, subjectSaid, two.concat(['reporter=scope']))].every((text, i, all) => sha256(text) !== sha256(wordsText('rc24', 1, 1, subjectSaid, two)) && all.indexOf(text) === i) && wordsText('rc24', 1, 1, subjectSaid, []).endsWith('subject=' + subjectSaid + '\n'))
  const did = [{ item: 'area-a', status: 'green' }, { item: 'gate', status: 'green', verdict: '/tmp/scratch-1/hold/gate-0123456789ab/verdict.json', job: 'gate-0123456789ab' }, { item: 'regression-set', status: 'red', verdict: '/tmp/scratch-1/hold/regression-set-0123456789ab-a1/verdict.json', job: 'regression-set-0123456789ab-a1' }, { item: 'ci', status: 'red', evidence: 'a sentence of the preflight\'s', by: 'preflight-second', word: voidWord('unanswered', 'preflight-second') }, { item: 'arm-a', status: 'void', reason: 'no trial image: a sentence', word: voidWord('no-image', 'preflight-second') }, { item: 'held-b', status: 'void', reason: 'dead:held-b-0123456789ab-a1', word: heldVoid({ ends: 'dead', name: 'held-b-0123456789ab-a1' }) }]
  const said = recordWords({ reporters: ['attempt', 'scope', 'triage'], commit: 'c'.repeat(40), gate: 'gate-0123456789ab', did, facts: [['candidate', 'c'.repeat(40)], ['binary', 'b'.repeat(64)], ['cross-model', ['row-a', 'row-b']], ['cross-model-void', []]] })
  check('a record\'s words say the reporters, the commit, the gate\'s job, what each item did and the round\'s facts, in that order — a held check by its job whatever its verdict, a red check by the preflight that read it, a void by its word, a list one word an item and an empty list none', JSON.stringify(said) === JSON.stringify(['reporter=attempt', 'reporter=scope', 'reporter=triage', 'commit=' + 'c'.repeat(40), 'gate=gate-0123456789ab', 'item=area-a:green', 'item=gate:held:gate-0123456789ab', 'item=regression-set:held:regression-set-0123456789ab-a1', 'item=ci:red:preflight-second', 'item=arm-a:void:no-image:preflight-second', 'item=held-b:void:dead:held-b-0123456789ab-a1', 'fact=candidate:' + 'c'.repeat(40), 'fact=binary:' + 'b'.repeat(64), 'fact=cross-model:row-a', 'fact=cross-model:row-b']))
  check('a re-run\'s words name the commit, and no gate and no fact of the round', JSON.stringify(recordWords({ reporters: ['attempt'], commit: 'c'.repeat(40), gate: null, did: [did[2]], facts: [] })) === JSON.stringify(['reporter=attempt', 'commit=' + 'c'.repeat(40), 'item=regression-set:held:regression-set-0123456789ab-a1']))
  check('no sentence is a word: a void says why in one word of the shape the record script takes, and no word of a record holds a space', Object.keys(VOID_WORDS).length === 12 && Object.keys(VOID_WORDS).every((word) => isSlug(word) && HELD_VOID_RE.test(voidWord(word)) && HELD_VOID_RE.test(voidWord(word, 'row-3-reconciler')) && VOID_WORDS[word].split(' ').length >= 6) && voidWord('silent', 'area-a-review') === 'silent:area-a-review' && voidWord('off-tree') === 'off-tree' && said.every((word) => /^[A-Za-z0-9_.:=-]+$/.test(word)) && !said.join(' ').includes('sentence'))
  const composing = composePrompt(tested, 'fix/rc24-tier1', subjectSaid, said)
  check('the composing step is ONE command of the tool: the branch, the run\'s directory, the round, the attempt, the subject as one quoted word, the sha256 of the words\' canonical form, and the words after `--`', composing.includes('\n1. `dev/stabilize-step compose --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --round 2 --attempt 4 --subject \'' + subjectSaid + '\' --sha256 ' + sha256(wordsText('rc24-tier1', 2, 4, subjectSaid, said)) + ' --scratch /tmp/scratch-1 --digest /tmp/scratch-1 -- ' + said.join(' ') + '`\n') && composing.startsWith('GIT STEP — ') && !composing.includes(LABELS.record) && !composing.includes(PAYLOAD_ENDS))
  check('a composing step that is asked for again is told that the tool refused the words, and to type the command as it stands', wordsAgain(1).startsWith('AGAIN (1): ') && wordsAgain(1).includes('typed as it stands') && wordsAgain(1).includes('nothing was written') && !wordsAgain(1).includes('`'))
  check('of the words a composed record is refused with, `words` is asked for again — once — and every other halts, each saying what it means', WORDS_AGAIN === 1 && Object.keys(COMPOSE_REFUSALS).filter((word) => COMPOSE_REFUSALS[word].again).join() === 'words' && ['words', 'check', 'pending', 'no-change', 'hygiene'].every((word) => COMPOSE_REFUSALS[word] && COMPOSE_REFUSALS[word].says.split(' ').length >= 10) && COMPOSE_REFUSALS['no-change'].then.includes('NO BATCH IS LEFT') && COMPOSE_REFUSALS.pending.then === 'pending')
  // THEN THE STAGE VERIFIES, IN WAVES: a verifier is handed its key and the read that prints
  // its row, the kit's call first, and the ONE call that files its verdict, spelled whole —
  // and what comes back is held to that call's line, as far as this script can hold one.
  const filer = 'verify-f-1'
  const files = verdictPrompt(tested, launcher(tested), filer, 'f-1', built, 2)
  const [kitCall, rowCall, filingCall] = ['`dev/stabilize-record kit --run rc24-tier1 --role verify`', '`dev/stabilize-record row --run rc24-tier1 --key f-1`', '`dev/stabilize-record verdict --run rc24-tier1 --round 2 --reporter verify-f-1 --attempt 4 --scratch /tmp/scratch-1 --from /tmp/scratch-1/verdict/r2/verify-f-1.a4.json`']
  check('a wave is as many verifiers as the digest prints keys of the queue, and a filing says one of three verdicts', WAVE === 8 && FILED_WORDS.join() === 'confirmed,refuted,undrivable')
  check('a verdict is filed where the record script files one, and written first to a file under the scratch root', verdictFile(tested, filer) === 'completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json' && filingFile(tested, filer) === '/tmp/scratch-1/verdict/r2/verify-f-1.a4.json' && verdictFile(Object.assign({}, tested, { attempt: 5 }), filer) !== verdictFile(tested, filer))
  check('a verifier that files is handed the kit\'s call, the read of its row and the call that files its verdict, each spelled whole and in that order', [kitCall, rowCall, filingCall].every((call) => files.includes(call)) && files.indexOf(kitCall) < files.indexOf(rowCall) && files.indexOf(rowCall) < files.indexOf(filingCall) && files.includes('to the file `/tmp/scratch-1/verdict/r2/verify-f-1.a4.json` WITH YOUR FILE TOOL'))
  check('it is handed its key, and nothing of its row: no door, no clause, no grade and no repro is in the prompt', files.includes('ONE finding: the row `f-1` of the run\'s ledger') && files.includes('This prompt carries none of it.') && !files.includes(entry.door) && !files.includes(entry.clause) && !files.includes(entry.repro) && !files.includes('triage\'s grade'))
  check('it carries its report line and its binary line with both binaries, and the two hashes are members of its file and no word of its call', carries(files, 'report') && carries(files, 'binary') && files.includes('; previous release `/tmp/scratch-1/bin/previous/jigc`') && files.includes('THE TWO HASHES ARE MEMBERS OF THAT FILE, and no word of the call') && !filingCall.includes('b'.repeat(64)) && !filingCall.includes('a'.repeat(64)))
  check('it is told how many of what it hands on enter the queue — the opening\'s number — or that none does where the opening names none', files.includes('AT MOST 2 ENTER THE QUEUE') && files.includes('the first 2 that carry the evidence of their kind') && !files.includes('ENTERS NO QUEUE') && verdictPrompt(tested, launcher(tested), filer, 'f-1', built, null).includes('WHAT YOU HAND ON ENTERS NO QUEUE') && !verdictPrompt(tested, launcher(tested), filer, 'f-1', built, null).includes('AT MOST') && verdictPrompt(tested, launcher(tested), filer, 'f-1', built, 0).includes('AT MOST 0 ENTER THE QUEUE'))
  check('it is told what its return is held to, and that what counts is what is filed', files.includes('WHAT YOUR RETURN IS HELD TO') && files.includes('held to `completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json`') && files.includes('held to confirmed, refuted, undrivable') && files.includes('a finding the queue prints again has no verdict') && !files.includes('left_open') && !files.includes('pass '))
  const filedBack = { status: 'filed', filing: verdictFile(tested, filer), word: 'confirmed', contested: false, asserted_sha256: 'b'.repeat(64) }
  const heldWith = (more) => verdictFault(tested, filer, Object.assign({}, filedBack, more), 'b'.repeat(64))
  check('a return of a verifier that files is held where it is its call\'s line: the path a filing of its name has, a verdict a filing says, `contested` as a truth value, and the hash it was handed', heldWith({}) === null && FILED_WORDS.every((word) => heldWith({ word }) === null) && heldWith({ contested: true }) === null && heldWith({ report: 'anything', basis: 'anything' }) === null)
  check('a return that names another path, or none, is not held, and says which path a filing has', heldWith({ filing: verdictFile(tested, 'verify-f-2') }).includes('a path that is not where a verdict of `verify-f-1`, attempt 4, is filed — `completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json`') && heldWith({ filing: undefined }).startsWith('returned no path that is not where') && isText(heldWith({ filing: verdictFile(Object.assign({}, tested, { attempt: 5 }), filer) })) && isText(heldWith({ filing: testReport(tested, filer) })))
  check('a return whose word is no verdict a filing says is not held, and what came back is not printed', [undefined, '', 'verified', 'Confirmed', 'regression', true].every((word) => isText(heldWith({ word })) && heldWith({ word }).includes('no verdict a filing says') && !heldWith({ word }).includes('`' + word + '`')) && heldWith({ word: undefined }).startsWith('returned no word'))
  check('a return that does not say `contested` as true or false is not held', [undefined, null, 'true', 'a basis', 1].every((contested) => isText(heldWith({ contested })) && heldWith({ contested }).includes('`contested` as true or false')))
  check('a return without the hash it asserted, or with another, is not held', heldWith({ asserted_sha256: undefined }).startsWith('returned no `asserted_sha256`') && heldWith({ asserted_sha256: 'a'.repeat(64) }).includes('the `asserted_sha256` ' + 'a'.repeat(64) + ', which is not the candidate\'s binary (' + 'b'.repeat(64) + ')') && heldWith({ asserted_sha256: 'a sentence' }).includes('an `asserted_sha256` that is no sha256') && !heldWith({ asserted_sha256: 'a sentence' }).includes('a sentence'))
  check('a verifier that files returns no verdict, and its schema requires nothing but its status', JSON.stringify(Object.keys(VERDICT_SCHEMA.properties)) === JSON.stringify(['status', 'halt', 'filing', 'word', 'contested', 'asserted_sha256', 'report']) && JSON.stringify(VERDICT_SCHEMA.required) === JSON.stringify(['status']) && VERDICT_SCHEMA.properties.word.enum === undefined && !('verdict' in VERDICT_SCHEMA.properties) && !('left_open' in VERDICT_SCHEMA.properties))
  // AN INVOCATION THAT CONTINUES, AND A WAVE THAT IS THE LAST: where a stage is continued and
  // what such an invocation takes; how many silent calls end the phase; and what the halt
  // after them says and names.
  const withQueue = { queue: { budget: 30, hand_on: 2, spent: 1, left: 29, length: 1, first: ['f-1'], classes: [] } }
  check('an invocation continues a stage where the run has a queue and the position says `triage`, `verify` or `filed` — worked in that order — and nowhere else', JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, verify: true })) === '["verify"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, verify: true, triage: true })) === '["triage","verify"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, triage: true })) === '["triage"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, filed: true })) === '["filed"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, filed: true, verify: true, triage: true })) === '["triage","verify","filed"]' && continuing({ queue: null }, { round: 2, attempt: 2, filed: true }).length === 0 && continuing(withQueue, { round: 3, attempt: 1 }).length === 0 && continuing(withQueue, { round: 2, attempt: 2, rerun: { clause: 'c' } }).length === 0 && continuing({ queue: null }, { round: 2, attempt: 2, triage: true }).length === 0 && continuing({}, { round: 2, attempt: 2, verify: true }).length === 0)
  check('an invocation that continues takes no scope, clause or cross-model pass, and is stopped after nothing but its first read', continueFault(base) === null && continueFault(Object.assign({}, base, { stopAfter: 'state' })) === null && [['scope', 'everything'], ['clause', 'no-lost-files'], ['crossModel', ['row-3']]].every(([name, value]) => isText(continueFault(Object.assign({}, base, { [name]: value }))) && continueFault(Object.assign({}, base, { [name]: value })).endsWith('it was passed ' + name)) && ['preflight', 'instruments', 'triage'].every((stop) => continueFault(Object.assign({}, base, { stopAfter: stop })).endsWith('it was passed stopAfter: \'' + stop + '\'')))
  check('two verifier calls of one wave that return nothing make it the last', SILENT_LAST === 2)
  // THE LIMIT'S HALT, AT EACH OF THE SEVEN STEPS IT CAN BE OF (LIMIT_STEPS): what it names, and
  // that every sentence it says is one that is so at that step — and no sentence that is not.
  const limitAt = (step, record) => limitSaid(tested, { wave: 3, silent: 2 }, 7, [verdictFile(tested, filer)], [verdictFile(tested, 'verify-f-2')], step, record || null)
  const limit = limitAt('compose')
  check('a halt that may be the account\'s limit says what it saw and never that it is one, and names the filed verdicts: the state\'s count, each path, what nothing held, and where they lie', JSON.stringify(limit.more) === JSON.stringify({ limit: { possible: true, wave: 3, silent: 2, step: 'compose' }, filed: { counted: 7, read: 'after-the-wave', commit: 'none', record: null, files: ['completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json'], unconfirmed: ['completions/artifacts/rc24-tier1/r2/verdicts/verify-f-2.a4.json'], directory: 'completions/artifacts/rc24-tier1/r2/verdicts' } }) && limit.says.includes('THIS MAY BE THE ACCOUNT\'S USAGE LIMIT') && limit.says.includes('2 verifier call(s) of wave 3 returned nothing') && limit.says.includes('The state counts 7 verdict(s) in round 2') && limit.says.includes('held by nothing: completions/artifacts/rc24-tier1/r2/verdicts/verify-f-2.a4.json') && limitSaid(tested, { wave: 1, silent: 8 }, 0, [], [], 'queue', null).says.includes('no verifier of this invocation filed one') && !limitSaid(tested, { wave: 1, silent: 8 }, 0, [], [], 'queue', null).says.includes('held by nothing'))
  check('the limit\'s halt can be of seven steps, each with whether the state was read after the wave and what became of the record\'s commit', JSON.stringify(Object.keys(LIMIT_STEPS)) === JSON.stringify(['report-check', 'queue', 'compose', 'gate', 'commit', 'push', 'read-back']) && JSON.stringify(Object.keys(LIMIT_STEPS).map((step) => [LIMIT_STEPS[step].read, LIMIT_STEPS[step].commit])) === JSON.stringify([['before', 'none'], ['before', 'none'], ['after', 'none'], ['after', 'none'], ['after', 'unknown'], ['after', 'made'], ['after', 'pushed']]) && Object.keys(LIMIT_STEPS).every((step) => isText(LIMIT_STEPS[step].step) && limitAt(step, 'c'.repeat(40)).says.includes('and then ' + LIMIT_STEPS[step].step + ' returned nothing too.')))
  const stateNowCall = '`dev/stabilize-record state --run rc24-tier1` prints the state as it stands now'
  check('before a read after the wave the halt says that the named document predates the wave, and names the one plain call that prints the state now — and after one it does not', ['report-check', 'queue'].every((step) => limitAt(step).says.includes('THE STATE WAS LAST READ BEFORE THAT WAVE: it counted 7 verdict(s) in round 2 then') && limitAt(step).says.includes('it lists nothing wave 3 filed') && limitAt(step).says.includes(stateNowCall) && !limitAt(step).says.includes('lists every filed verdict') && limitAt(step).more.filed.read === 'before-the-wave') && ['compose', 'gate', 'commit', 'push', 'read-back'].every((step) => !limitAt(step, 'c'.repeat(40)).says.includes('BEFORE THAT WAVE') && !limitAt(step, 'c'.repeat(40)).says.includes(stateNowCall) && limitAt(step, 'c'.repeat(40)).says.includes('the state document this return names lists every filed verdict with whether it counts (`filed`)') && limitAt(step, 'c'.repeat(40)).more.filed.read === 'after-the-wave'))
  check('the halt says *in no commit yet* only where no commit can hold what was filed; where the commit step returned nothing it says that it cannot tell; and from the push on it names the commit', ['report-check', 'queue', 'compose', 'gate'].every((step) => limitAt(step).says.includes('WHAT WAS FILED IS NOT LOST, AND IS IN NO COMMIT YET: ') && limitAt(step).more.filed.commit === 'none' && limitAt(step).more.filed.record === null) && !limitAt('commit').says.includes('IN NO COMMIT YET') && !limitAt('commit').says.includes('IS IN A COMMIT') && limitAt('commit').says.includes('WHETHER A COMMIT HOLDS IT THIS SCRIPT CANNOT SAY') && limitAt('commit').more.filed.commit === 'unknown' && limitAt('commit', 'c'.repeat(40)).more.filed.record === null && ['push', 'read-back'].every((step) => !limitAt(step, 'c'.repeat(40)).says.includes('IN NO COMMIT YET') && limitAt(step, 'c'.repeat(40)).says.includes('WHAT WAS FILED IS NOT LOST, AND IS IN A COMMIT: the record this invocation made (' + 'c'.repeat(40) + ') took it') && limitAt(step, 'c'.repeat(40)).more.filed.record === 'c'.repeat(40)) && limitAt('push', 'c'.repeat(40)).says.includes('a local commit until the push that is owed is made') && !limitAt('push', 'c'.repeat(40)).says.includes('and is pushed') && limitAt('read-back', 'c'.repeat(40)).says.includes(', and is pushed.') && ['commit', 'push', 'read-back'].every((step) => limitAt(step, 'c'.repeat(40)).says.includes('as it was read before that record\'s commit')) && ['compose', 'gate'].every((step) => !limitAt(step).says.includes('as it was read before that record\'s commit')))
  check('the halt says that the stage is continued only at the two steps before its record: from the composing step on, what the next invocation does first is the record step\'s, the push\'s and the read-back\'s own sentence', ['report-check', 'queue'].every((step) => limitAt(step).says.includes('it CONTINUES') && limitAt(step).says.endsWith('whichever record the run makes next takes what is filed.')) && ['compose', 'gate', 'commit', 'push', 'read-back'].every((step) => !limitAt(step, 'c'.repeat(40)).says.includes('it CONTINUES') && limitAt(step, 'c'.repeat(40)).says.endsWith('what that invocation does first is said above, with this halt.')))
  // WHICH JOB ANSWERS A START (`passedOver`): the whole table — what the tool's answer says, by
  // what is held, by how often the start step was tried.
  const aRecordsGate = { sequence: true }
  const byItsCommit = { stands: true }
  const oneName = {}
  const answered = (ends, fresh) => ({ ends, fresh: fresh === true })
  const overOf = (job, ends, tries, fresh) => passedOver(job, answered(ends, fresh), tries)
  check('a job that answers a start as ENDED is never the answer for a record\'s gate, whatever the tries; for what is named by its commit only green is', ['green', 'red', 'void', 'dead'].every((ends) => [1, 2, 3].every((tries) => overOf(aRecordsGate, ends, tries) === 'ended')) && ['red', 'void', 'dead'].every((ends) => [1, 2, 3].every((tries) => overOf(byItsCommit, ends, tries) === 'ended')) && [1, 2, 3].every((tries) => overOf(byItsCommit, 'green', tries) === null))
  check('a job that still runs is passed over for a record\'s gate on the start\'s first try, is the dead try\'s own on a retry, and is taken over for what is named by its commit', overOf(aRecordsGate, 'running', 1) === 'running' && overOf(aRecordsGate, 'running', 2) === null && overOf(aRecordsGate, 'running', 3) === null && [1, 2].every((tries) => overOf(byItsCommit, 'running', tries) === null))
  check('the job this call started, and a start that was not read, are never passed over — and a hold of one name passes nothing over', [aRecordsGate, byItsCommit, oneName].every((job) => [1, 2].every((tries) => overOf(job, 'running', tries, true) === null && overOf(job, 'unread', tries) === null)) && ['running', 'green', 'red', 'void', 'dead'].every((ends) => [1, 2].every((tries) => overOf(oneName, ends, tries) === null)))
  check('a name without a record is said to be what the tool\'s answer says of it — never that its command\'s process is gone', heldEnded({ ends: 'dead', why: 'no-record' }).includes('MAY STILL RUN') && !heldEnded({ ends: 'dead', why: 'no-record' }).includes('process is gone') && heldEnded({ ends: 'dead', why: 'no-record' }).includes('`supervised`') && heldEnded({ ends: 'dead', why: 'no-exit' }) === HELD_ENDS.dead && heldEnded({ ends: 'dead', why: 'killed' }) === HELD_ENDS.dead && heldEnded({ ends: 'red' }) === HELD_ENDS.red && heldFault('the candidate\'s binary', { name: 'build-c1', ends: 'dead', why: 'no-record' }).includes('MAY STILL RUN') && JSON.stringify(Object.keys(DEAD_WHY)) === '["no-record"]')
  check('a step that returned nothing is told from one that answered: only the first is what the limit\'s halt is said of', heldOf(null, '/tmp/scratch-1').silent === true && heldOf({ status: 'halted', refused: 'taken' }, '/tmp/scratch-1').silent === undefined && heldOf({ status: 'running', name: 'g', kind: 'gate' }, '/tmp/scratch-1').silent === undefined)
  const noNameSaid = recordThen('rc24-tier1', 'fix/rc24-tier1', 'no-name')
  check('where no name is left for a pending batch\'s gate the halt says so, never that the next invocation gates it under the next name, and names the exit that works', noNameSaid.includes('NO NAME IS LEFT TO HOLD ITS GATE UNDER') && noNameSaid.includes('one of ' + HOLD_NAMES + ' names') && noNameSaid.includes('INVOKING THE STAGE AGAIN CHANGES NOTHING') && noNameSaid.includes('`dev/stabilize-step discard --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1`') && !noNameSaid.includes('under the next name') && !noNameSaid.includes('once more') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'no-gate').includes('under the next name') && heldFault('the record\'s gate', { name: 'record-pending-9', ends: 'unread', exhausted: true, why: 'none is left' }) === 'the record\'s gate was not held: none is left.' && pendingRecord('/tmp/scratch-1/record/pending', { calls: 2, checks: 1 }).pending === true && pendingRecord('/tmp/scratch-1/record/pending', { calls: 2, checks: 1 }).hold === 'record-pending')
  const vetText = checkReportsPrompt(base, null)
  check('the vet before a record of rulings is the report check without a stage: one plain call that names the run and the scratch root, and no round, stage, attempt or reporter', vetText.includes('1. `dev/stabilize-step check-reports --run rc24-tier1 --scratch /tmp/scratch-1 --digest /tmp/scratch-1`') && !vetText.includes('--round') && !vetText.includes('--stage') && !vetText.includes('--attempt') && !vetText.includes(' -- '))
  const finishing = preflightPrompt(ctx, launcher(ctx), 'preflight', { image: false, sha: 'c'.repeat(40), label: 'c2', branch: 'fix/rc24-tier1', checks: [], crossModel: false, tested: true })
  check('a finishing preflight is told that the candidate is the commit the round tested, and not HEAD', finishing.includes('the commit round 2 tested, an ancestor of `fix/rc24-tier1`') && finishing.includes('THE CANDIDATE IS NOT `HEAD` HERE') && finishing.includes('commit ' + 'c'.repeat(40) + ' must be an ancestor of `HEAD`') && !finishing.includes('`git ') && !prompts.preflight.includes('NOT `HEAD`'))
  check('a fork is recorded with the verdict it rides on, and with no other entry', JSON.stringify(triageRecord(ctx, [{ key: 'f-1', new: false, grade: 'breaks', verdict: 'confirmed', regression: false, fork: { kind: 'contested', case: 'robust-now', drive: 'holds' } }, { key: 'f-2', new: false, grade: 'breaks', fork: { kind: 'contested', case: 'robust-now', drive: 'holds' } }]).triage) === JSON.stringify([{ key: 'f-1', grade: 'breaks', verdict: 'confirmed', regression: false, fork: { kind: 'contested', case: 'robust-now', drive: 'holds' } }, { key: 'f-2', grade: 'breaks' }]))
  check('a report line gives the flags', launch.line('scope') === reportLine(ctx, 'scope') && reportLine(ctx, 'scope').startsWith('REPORT: --run rc24-tier1 --round 2 --stage fix --cycle 3 --reporter scope --attempt 4 ') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').startsWith('REPORT: --run rc24-tier1 --round 2 --stage test --reporter x --attempt 4 ') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').includes('to the file `/tmp/scratch-1/report/r2/test/x.a4.md` WITH YOUR FILE TOOL') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').includes('`dev/stabilize-record report --run rc24-tier1 --round 2 --stage test --reporter x --attempt 4 --scratch /tmp/scratch-1 --from /tmp/scratch-1/report/r2/test/x.a4.md`') && reportFile(ctx, 'scope') === '/tmp/scratch-1/report/r2/fix.c3/scope.a4.md')
  check('a binary line gives the path and the hash, and the one plain call that asserts it', binaryLine(built, true, '/tmp/scratch-1').includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc` prints ONE line of JSON whose `content_sha256` must be that sha256') && binaryLine(built, true, '/tmp/scratch-1').endsWith(' Return the sha256 you asserted for the candidate as `asserted_sha256`.') && !binaryLine(built, true).includes('stabilize-step') && prompts.review.includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc`') && prompts.verify.includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc`') && binaryLine(built, true).startsWith('BINARY: candidate `/tmp/scratch-1/bin/c2/jigc` sha256 ' + 'b'.repeat(64)) && binaryLine(built, true).includes('jigc-trial:c2') && !binaryLine(built, false).includes('jigc-trial:c2'))
  check('every chain is staffed from the roles', Object.keys(CHAINS).concat(['fix-diff']).every((kind) => (CHAINS[kind] || FIX_AUDIT).every((stepList) => stepList.every((s) => ROLES[s.role] && isSlug(s.as) && !s.as.includes('-') && (s.hands || []).every((h) => (CHAINS[kind] || FIX_AUDIT).some((earlier) => earlier.some((e) => e.as === h)))))))

  // The cross-model pass runs for the items an invocation names, and for no other: an
  // item that is not named has no such step, nobody is handed its report, and no prompt
  // names its tool. (That no argument value names every item is among the refusals above.)
  const names = new RegExp(crossModelTool(), 'i')
  const asOf = (chain) => chain.reduce((all, stepList) => all.concat(stepList.map((s) => s.as + (s.hands ? '<' + s.hands.join('+') : ''))), []).join(' ')
  for (const kind of Object.keys(CHAINS)) {
    for (const given of [undefined, null, false, 'true', 1, [], ['row-3'], 'all']) check('no cross-model step in ' + kind + ' under ' + JSON.stringify(given), chainOf(kind, given).every((stepList) => stepList.every((s) => !s.crossModel && ROLES[s.role] !== ROLES.crossModel && !(s.hands || []).includes('crossmodel'))))
  }
  check('the review row without the opt-in, and with it', asOf(chainOf('review-row', false)) === 'source driver reconciler<source+driver' && asOf(chainOf('review-row', true)) === 'source driver crossmodel reconciler<source+driver+crossmodel' && chainOf('no-such-kind', true).length === 0)
  // Every git step is ONE command of the tool, and its prompt is that command and "relay
  // its line" — all but the re-cut of a part, which is the list of commands it was.
  const steps = { begin: beginPrompt(Object.assign({}, ctx, { stage: 'test' }), 'c'.repeat(40)), 'git-state': gitStatePrompt(base), state: statePrompt(base, 't'), 'find-round': findRoundPrompt(fix, 1), 'open-round': openRoundPrompt(fix, 1, 'fix/rc24-tier1-r1'), push: pushPrompt(fix, 'fix/rc24-tier1-r1'), land: landPrompt(fix, 1, 'fix/rc24-tier1-r1'), 'round-commits': roundCommitsPrompt(fix, 1, 'fix/rc24-tier1-r1'), carry: carryPrompt(fix, 1, ['a'.repeat(40)], null), 'sync-main': syncMainPrompt('rc24-tier1', '/tmp/scratch-1'), 'check-reports': checkReportsPrompt(ctx, ['x']), record: recordCommitPrompt(fix, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }) }
  const recut = carryPrompt(fix, 1, ['a'.repeat(40)], 'fix/rc24-tier1-r1-part1')
  for (const act of Object.keys(steps)) {
    const numbered = steps[act].split('\n')
    check('the `' + act + '` step is one command of the tool, and its line relayed', numbered.length === 3 && numbered[0].startsWith('GIT STEP — ') && numbered[0].endsWith(STEP_RULES) && numbered[1].startsWith('1. `' + STEP_TOOL + ' ' + act + ' ') && numbered[1].endsWith('`') && numbered[2].startsWith('2. Report status = ran and line = ') && !steps[act].includes('`git '))
    check('the `' + act + '` step asks for its digest, once, under the scratch root — and before any list of names', numbered[1].split(' --digest /tmp/scratch-1').length === 2 && (!numbered[1].includes(' -- ') || numbered[1].indexOf(' --digest ') < numbered[1].indexOf(' -- ')))
  }
  check('the names a step hands the tool are the harness\'s', steps['git-state'].includes('`' + STEP_TOOL + ' git-state --stage test --loop fix/rc24-tier1 --rounds fix/rc24-tier1-r --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --digest /tmp/scratch-1`') && steps.land.includes(' land --loop fix/rc24-tier1 --branch fix/rc24-tier1-r1 --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --log DECISIONS.md --log implementation/project-history.md --digest /tmp/scratch-1`') && steps.carry.endsWith(STEP_RULES + '\n1. `' + STEP_TOOL + ' carry --loop fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --digest /tmp/scratch-1 -- ' + 'a'.repeat(40) + '`\n' + steps.carry.split('\n')[2]) && steps['sync-main'].includes(' sync-main --loop fix/rc24-tier1 --log DECISIONS.md --log implementation/project-history.md --digest /tmp/scratch-1`') && steps.state.includes(' state --run rc24-tier1 --scratch /tmp/scratch-1 --tag t --digest /tmp/scratch-1`') && steps['find-round'].includes(' find-round --prefix fix/rc24-tier1-r1 --digest /tmp/scratch-1`') && steps.push.includes(' push --branch fix/rc24-tier1-r1 --digest /tmp/scratch-1`') && steps.record.includes(' record --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --gate /tmp/scratch-1/record/x/gate.txt --calls 3 --checks 2 --scratch /tmp/scratch-1 --digest /tmp/scratch-1`'))
  check('an attempt is begun under the script\'s own reporter, on the commit it runs on', steps.begin.includes('`' + STEP_TOOL + ' begin --run rc24-tier1 --round 2 --stage test --attempt 4 --reporter ' + ATTEMPT + ' --commit ' + 'c'.repeat(40) + ' --scratch /tmp/scratch-1 --digest /tmp/scratch-1`') && beginPrompt(ctx, 'c'.repeat(40)).includes(' begin --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 --reporter '))
  check('no agent can be named as the attempt\'s marker is', isSlug(ATTEMPT) && !ATTEMPT.includes('-') && ATTEMPT !== LEDGER_SOURCE && !Object.keys(CHAINS).some((kind) => CHAINS[kind].some((stepList) => stepList.some((step) => reporterName(['x', step.as]) === ATTEMPT))) && !['preflight', 'scope'].includes(ATTEMPT))
  check('the re-cut of a part is still the list of commands it was', recut.startsWith('GIT STEP — re-cut a part of round 1 of `rc24-tier1` as a branch of its own, `fix/rc24-tier1-r1-part1`') && recut.includes(CARRY_RULES) && recut.includes('`git cherry-pick -x <sha>`') && recut.includes('`git push origin fix/rc24-tier1-r1-part1`') && !recut.includes(STEP_TOOL) && recut.split('\n').length === 8)
  const gitPrompts = Object.keys(steps).map((act) => steps[act]).concat([recut])
  check('no prompt names the tool without the opt-in', Object.keys(prompts).filter((role) => names.test(prompts[role])).join(' ') === 'crossModel' && !names.test(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text) && !names.test(gitPrompts.join('\n')))
  const asserts = (crossModel) => preflightPrompt(ctx, launcher(ctx), 'preflight', { image: false, sha: 'c'.repeat(40), label: 'c2', branch: 'b', checks: [], crossModel })
  check('the preflight is asked about the tool only when an item is named', names.test(asserts(true)) && asserts(true).includes('`' + CROSS_CHECK + '`') && !names.test(asserts(false)) && !names.test(asserts(undefined)) && !asserts(false).includes(CROSS_CHECK))
  const dropping = launcher(ctx)
  dropping.add(['row-3', 'source'])
  dropping.drop(dropping.add(['row-3', 'crossmodel']))
  check('a cross-model reporter that left no report is not held to one', dropping.names.join(' ') === 'row-3-source')
  check('a round with no scope of its own takes the run\'s default, as its record names it', scopeText(undefined, 'delta').endsWith('the derived delta') && scopeText(null, 'everything').includes('everything') && scopeText('delta', 'everything') === 'the derived delta')
  check('a driving chain role is handed the binary', unitPrompt(ctx, launcher(ctx), 'x', CHAINS['trial-arm'][1][0], unit, built, []).includes('jigc-trial:c2'))

  // How an agent can end: what the report check says decides, never what a return claims.
  const checked = (check, aside) => reportsRead({ status: 'checked', check, aside: aside || [], file: 'lines/check-reports.0123456789abcdef.json', file_sha256: 'f'.repeat(64) })
  check('a report check that holds names nothing missing', JSON.stringify(checked({ ok: true, missing: [], extra: 0 })) === '{"missing":[],"aside":[]}')
  check('a launched reporter with no report is named, and is no fault', JSON.stringify(checked({ ok: false, missing: ['verify-p1-f-1'], extra: 0 })) === '{"missing":["verify-p1-f-1"],"aside":[]}')
  check('a file nobody launched is a fault that says how many, and names the file that names them — never the names', isText(checked({ ok: false, missing: [], extra: 1 }).fault) && checked({ ok: false, missing: ['a'], extra: 2 }).fault.includes('2 file(s)') && checked({ ok: false, missing: ['a'], extra: 1 }).missing === undefined && checked({ ok: false, missing: [], extra: 1 }).file.file === 'lines/check-reports.0123456789abcdef.json')
  check('a report check that cannot be read is a fault, and transient only when nothing came back', reportsRead(null).transient === true && isText(reportsRead(null).fault) && reportsRead({ status: 'halted' }).transient === false && isText(reportsRead({ status: 'checked', check: { ok: true }, aside: [] }).fault) && isText(reportsRead({ status: 'checked', check: 'ok', aside: [] }).fault) && isText(reportsRead({ status: 'checked', check: { ok: true, missing: [], extra: [] }, aside: [] }).fault) && isText(reportsRead({ status: 'checked', check: { ok: true, missing: [], extra: 0 } }).fault) && reportsRead({ status: 'halted', refused: 'did-not-run' }).fault.includes('`did-not-run`'))
  const moved = [{ path: 'completions/artifacts/rc24-tier1/r2/reports/test/a-source.a1.md', why: 'hygiene' }, { path: 'completions/artifacts/rc24-tier1/r2/scope.md', why: 'host-path' }]
  check('a report that was set aside is its reporter\'s, by the file\'s name, with the word of what refused it', JSON.stringify(checked({ ok: false, missing: ['a-source'], extra: 0 }, moved)) === JSON.stringify({ missing: ['a-source'], aside: moved }) && setAside(moved, 'a-source') === 'hygiene' && setAside(moved, 'a') === null && setAside(moved, 'scope') === null && setAside(undefined, 'a-source') === null)
  check('how a reporter that did not report ended', endingOf(null) === 'returned nothing' && endingOf({ status: 'halted', halt: { root_cause: 'no rig' } }) === 'halted (no rig)' && endingOf({ status: 'halted' }) === 'halted')
  const twoUnits = () => [
    { item: 'a', status: 'green', reason: null, crossModel: 'ran', reporters: [{ as: 'source', name: 'a-source', crossModel: false, result: { status: 'reported', findings: [1] }, report: 'a.md' }, { as: 'crossmodel', name: 'a-crossmodel', crossModel: true, result: { status: 'reported', findings: [2] }, report: null }] },
    { item: 'b', status: 'void', reason: 'its `drive` step returned nothing', crossModel: null, reporters: [{ as: 'drive', name: 'b-drive', crossModel: false, result: null, report: null }] },
  ]
  const lostCross = twoUnits()
  unreported(lostCross, ['a-crossmodel', 'b-drive'])
  check('a cross-model pass with no report is void for that pass, and never voids its item', lostCross[0].status === 'green' && lostCross[0].crossModel === 'void' && lostCross[0].reporters[1].result === null && lostCross[0].reporters[0].result.findings.length === 1 && lostCross[1].reason === 'its `drive` step returned nothing')
  const lostSource = twoUnits()
  unreported(lostSource, ['a-source'])
  check('a step of an item\'s own chain with no report voids the item, and its result counts for nothing', lostSource[0].status === 'void' && lostSource[0].reason === 'its `source` step returned and left no report' && lostSource[0].reporters[0].result === null && lostSource[0].crossModel === 'ran')
  const asideSource = twoUnits()
  unreported(asideSource, ['a-source'], moved)
  check('a step whose report was set aside voids its item, and the reason says that the file was no report', asideSource[0].status === 'void' && asideSource[0].reason === 'its `source` step left a file at its report\'s path that is no report (`hygiene`): it was set aside, and the step left none' && asideSource[0].reporters[0].result === null)
  const kept = twoUnits()
  unreported(kept, [])
  check('a reporter whose report is there keeps what it returned', JSON.stringify(kept) === JSON.stringify(twoUnits()))
  check('the roles with no definition are handed the paragraph every definition carries', prompts.proposal.endsWith('\n' + NEVER) && prompts.crossModel.endsWith('\n' + NEVER) && NEVER.startsWith('**Never push to or merge into `main`') && Object.keys(prompts).filter((role) => prompts[role].includes(NEVER)).join(' ') === 'proposal crossModel')
  check('a role is handed the binary exactly when it is held to the hash it asserts', Object.keys(ROLES).every((role) => ROLES[role].drives === (ROLES[role].labels.includes('binary') || role === 'proposal')) && ROLES.review.drives && carries(prompts.review, 'binary'))

  // The reporters: every name once, and the check names every one of them.
  check('the reporter list', launch.names.join(' ') === 'preflight scope row-3-source row-3-reconciler triage-p1 verify-p1-f-1 advocate-f-1 proposal-f-1 row-3-crossmodel fix-area-1')
  let twice = false
  try { launch.add(['scope']) } catch (e) { twice = true }
  let tooLong = false
  try { launch.add(['verify', 'p1', 'k'.repeat(100)]) } catch (e) { tooLong = true }
  check('a reporter launched twice, or with no name', twice && tooLong && launch.names.length === 10 && reporterName(['a', 'B']) === null)
  // A record step: its calls as ONE batch, the gate, and no commit of the executor's.
  const batchOf = (text) => JSON.parse(text.split('\n').find((l) => l.startsWith('[{"argv"')))
  const spelled = (made) => made.argv.join(' ')
  const recordCalls = batchOf(prompts.record)
  check('the check names every reporter launched', launch.names.every((n) => checkReportsPrompt(ctx, launch.names).includes(' ' + n)) && checkReportsPrompt(ctx, launch.names).includes('`' + STEP_TOOL + ' check-reports --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 --scratch /tmp/scratch-1 --digest /tmp/scratch-1 -- ' + launch.names.join(' ') + '`') && spelled(recordCalls[0]) === 'check-reports --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 -- ' + launch.names.join(' '))
  check('the ledger check names every key', spelled(recordCalls[recordCalls.length - 1]) === 'check-ledger --run rc24-tier1 -- f-1')
  check('a record\'s calls are one payload, written by a file tool and held to its hash by the writer', prompts.record.includes(' --sha256 ' + sha256(JSON.stringify(recordCalls) + '\n') + '`') && prompts.record.includes('\n1. Write the file `/tmp/scratch-1/record/x/batch-' + sha256(JSON.stringify(recordCalls) + '\n').slice(0, 12) + '.json` WITH YOUR FILE TOOL') && prompts.record.includes('\n' + PAYLOAD_ENDS + '\n' + JSON.stringify(recordCalls) + '\n' + PAYLOAD_ENDS + '\n') && !/cat |mkdir|shasum|<<|here-document/.test(prompts.record) && spelled(recordCalls[1]) === 'round-set --run rc24-tier1 --round 2' && recordCalls[1].stdin === '{"cycles":3}' && recordCalls.length === 3)
  check('a record step applies its calls as one batch from a file, RUNS NO GATE, and makes no commit', prompts.record.includes('\n2. `dev/stabilize-record apply --run rc24-tier1 --round 2 --subject \'docs(record): s\' --scratch /tmp/scratch-1 --from /tmp/scratch-1/record/x/batch-' + sha256(JSON.stringify(recordCalls) + '\n').slice(0, 12) + '.json --sha256 ' + sha256(JSON.stringify(recordCalls) + '\n') + '`') && !prompts.record.includes('\n3. ') && !prompts.record.includes('dev/gate') && prompts.record.endsWith(RECORD_RETURNS) && RECORD_RETURNS.includes('RUN NO GATE') && RECORD_RETURNS.includes('Return status = applied') && JSON.stringify(RECORD_SCHEMA.properties.status.enum) === JSON.stringify(['applied', 'halted']) && RECORD_SCHEMA.properties.gate === undefined && !/`git (add|commit)/.test(prompts.record))
  const stageRecord = recordPrompt(ctx, 'x', 'fix/rc24-tier1', '/tmp/scratch-1/record/z', 0, stageRecordCommands(Object.assign({}, ctx, { stage: 'test' }), { reporters: ['a'], gate: { commit: 'c'.repeat(40), file: '/tmp/scratch-1/gate/c2.a4.txt' }, results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }, { item: 'it\'s-b', status: 'void', reason: 'it\'s chain died' }]) }, rows: [], triage: [], patches: [], facts: { candidate: 'c'.repeat(40) }, keys: [] }), 's')
  check('what the commit step is held to is what was composed', JSON.stringify(stageRecord.expect) === JSON.stringify({ calls: 4, checks: 1 }) && stageRecord.hold === 'record-z' && stageRecord.gate === undefined && !stageRecord.text.includes(' --round 0') && JSON.stringify(recordPrompt(ctx, 'x', 'b', '/d', 2, recordCalls, 's').expect) === JSON.stringify({ calls: 3, checks: 2 }))
  check('a test stage\'s record puts the candidate\'s gate on record, from the file the tool kept its output in', spelled(batchOf(stageRecord.text)[1]) === 'gate-set --run rc24-tier1 --round 2 --commit ' + 'c'.repeat(40) + ' --summary /tmp/scratch-1/gate/c2.a4.txt --scratch /tmp/scratch-1' && !recordCalls.some((made) => made.argv[0] === 'gate-set'))
  check('what each item did is one call, its rows data whatever they hold', spelled(batchOf(stageRecord.text)[2]) === 'result-set --run rc24-tier1 --round 2 --commit ' + 'c'.repeat(40) + ' --scratch /tmp/scratch-1' && batchOf(stageRecord.text)[2].stdin === JSON.stringify([{ item: 'a', outcome: 'green' }, { item: 'it\'s-b', outcome: 'void', reason: 'it\'s chain died' }]))
  const resumed = pendingRecord('/tmp/scratch-1/record/pending', { calls: 5, checks: 2, files: 4, round: 1 })
  check('a pending batch is gated again by the tool: no executor runs, and nothing is applied', resumed.text === null && resumed.hold === 'record-pending' && JSON.stringify(resumed.expect) === JSON.stringify({ calls: 5, checks: 2 }))
  const cycleRecord = stageRecordCommands(ctx, { reporters: [], rows: [{ key: 'f-2' }], triage: [{ key: 'f-1', grade: 'breaks' }], patches: [{ key: 'f-1', disposition: 'fixed', detail: 'c'.repeat(40) }], clauses: [], facts: null, keys: ['f-1', 'f-2'] }).map((made) => made.argv[0])
  const at = (name) => cycleRecord.indexOf(name)
  check('a cycle\'s fixes are on the rows before its audit\'s triage reads them', at('ledger-add') >= 0 && at('ledger-add') < at('ledger-set') && at('ledger-set') < at('triage-set') && at('triage-set') < at('check-ledger'))

  // What is accepted as recorded: the commit step's own line, with one result per check.
  const held = { status: 'recorded', commit: 'a'.repeat(40), gate: { ok: true, verdict: 'pass', red: 0, known: 0, new: 0 }, applied: { calls: 4, checks: 2, failed: 0 } }
  const composed = { calls: 4, checks: 2 }
  const lacking = (change) => recordFault(Object.assign({}, held, change), composed)
  check('a record whose line holds is recorded', recordFault(held, composed) === null)
  check('a record step that returns nothing, or halts, is not recorded', isText(recordFault(null, composed)) && recordFault({ status: 'halted', halt: { root_cause: 'gate-red: x' } }, composed) === 'gate-red: x' && isText(recordFault({ status: 'halted' }, composed)))
  check('a record without a commit, or without a gate check that holds, is not recorded', isText(lacking({ commit: 'abc' })) && isText(lacking({ gate: undefined })) && isText(lacking({ gate: { ok: false, new: 1 } })) && isText(lacking({ gate: 'GATE: PASS' })))
  check('a record that returns no evidence of its checks is not recorded', isText(lacking({ applied: undefined })) && isText(lacking({ applied: { calls: 4 } })) && isText(lacking({ applied: { calls: 4, checks: 0, failed: 0 } })) && isText(lacking({ applied: { calls: 4, checks: 1, failed: 0 } })) && isText(lacking({ applied: { calls: 3, checks: 2, failed: 0 } })) && isText(lacking({ applied: { calls: 4, checks: [{ check: 'reports', ok: true }, { check: 'ledger', ok: true }] } })))
  check('a record one of whose checks does not hold is not recorded', isText(lacking({ applied: { calls: 4, checks: 2, failed: 1 } })) && isText(lacking({ applied: { calls: 4, checks: 2 } })) && isText(lacking({ applied: { calls: 4, checks: 2, failed: null } })))
  check('a commit step that refused is said in the sentence of its word', recordFault({ status: 'halted', refused: 'gate-red', branch: 'fix/rc24-tier1' }, composed).includes('refused `gate-red` — the orchestrator\'s: ') && recordFault({ status: 'halted', relay: 'the relayed line is no digest' }, composed) === 'the relayed line is no digest')

  // The candidate's gate is run by the first preflight of a `test` stage, and by no other.
  const heldStart = holdStartPrompt(base, 'gate-c2-a4', 'gate', '--run rc24-tier1', 'the full gate')
  const heldWait = holdWaitPrompt(base, 'gate-c2-a4', 'the full gate')
  check('the gate of a run is a command the tool holds, started by one plain call and asked after by another', heldStart.includes('\n1. `dev/stabilize-step hold-start --scratch /tmp/scratch-1 --name gate-c2-a4 --kind gate --run rc24-tier1 --digest /tmp/scratch-1`\n') && heldStart.split('\n')[0].endsWith(STEP_RULES) && heldWait.includes('\n1. `dev/stabilize-step hold-wait --scratch /tmp/scratch-1 --name gate-c2-a4 --digest /tmp/scratch-1`\n') && heldWait.split('\n')[0].endsWith(ASK_RULES) && ASK_RULES.includes('RUN THE SAME COMMAND AGAIN') && !STEP_RULES.includes('RUN THE SAME COMMAND AGAIN') && STEP_RULES.includes('never run the command a second time') && heldWait.includes('of the LAST run of it') && !heldStart.includes('of the LAST run of it'))

  // Ruling 11's comparison.
  const good = { status: 'reported', asserted_sha256: 'b'.repeat(64) }
  check('the hash comparison', hashMismatch('b'.repeat(64), [{ name: 'a', drives: true, result: good }, { name: 'b', drives: false, result: { status: 'reported' } }, { name: 'c', drives: true, result: { status: 'halted' } }, { name: 'd', drives: true, result: null }]).length === 0)
  check('a hash that differs, and one never returned', JSON.stringify(hashMismatch('b'.repeat(64), [{ name: 'a', drives: true, result: { status: 'reported', asserted_sha256: 'c'.repeat(64) } }, { name: 'b', drives: true, result: { status: 'reported' } }, { name: 'c', drives: true, result: good }])) === JSON.stringify([{ reporter: 'a', asserted: 'c'.repeat(64) }, { reporter: 'b', asserted: null }]))
  check('a needed field that is not there is the row of its role, and one that is there is none', JSON.stringify(lacks('review', { status: 'reported', findings: [] }).map((n) => n.field + ':' + n.then)) === JSON.stringify(['asserted_sha256:void']) && lacks('review', good).length === 0 && lacks('crossModel', { status: 'reported' }).length === 0 && JSON.stringify(lacks('verify', { status: 'verified', key: 'k' }).map((n) => n.field)) === JSON.stringify(['verdict', 'asserted_sha256']) && JSON.stringify(lacks('verify', { status: 'verified', key: 'k', verdict: 'confirmed', asserted_sha256: 'b'.repeat(64), ran_on: { candidate: 'b'.repeat(64) } }).map((n) => n.field + ':' + n.then)) === JSON.stringify(['regression:unknown', 'ran_on.previous:unknown']) && lacks('verify', { status: 'verified', key: 'k', verdict: 'refuted', asserted_sha256: 'b'.repeat(64) }).length === 0)
  check('why a return is no evidence about the candidate: no hash, another hash, and text that is none', unasserted('drive', 'drive-a-drive', { status: 'reported' }, 'b'.repeat(64)) === '(drive-a-drive) returned no `asserted_sha256` — ' + ASSERTED && unasserted('drive', 'd', { status: 'reported', asserted_sha256: 'c'.repeat(64) }, 'b'.repeat(64)) === '(d) returned the `asserted_sha256` ' + 'c'.repeat(64) + ', which is not the candidate\'s binary (' + 'b'.repeat(64) + '): nothing it drove is evidence about this candidate' && unasserted('drive', 'd', { status: 'reported', asserted_sha256: 'x`; rm -r /' }, 'b'.repeat(64)).includes('an `asserted_sha256` that is no sha256') && !unasserted('drive', 'd', { status: 'reported', asserted_sha256: 'x`; rm -r /' }, 'b'.repeat(64)).includes('rm -r') && unasserted('drive', 'd', good, 'b'.repeat(64)) === null && unasserted('crossModel', 'x', { status: 'reported' }, 'b'.repeat(64)) === null)
  check('every cell of what a stage needs is one of five words, and every row of what still halts names its phase', Object.values(NEEDS).every((rows) => rows.every((n) => ['void', 'unverified', 'no-fork', 'unknown', 'halts'].includes(n.then) && isText(n.field) && isText(n.why))) && Object.keys(NEEDS).every((role) => ROLES[role] && ROLES[role].drives) && Object.values(HALTS_AFTER).every((row) => isText(row.phase) && isText(row.when)) && Object.values(NEEDS).some((rows) => rows.some((n) => n.then === 'halts')) === ('regression-fact' in HALTS_AFTER))

  // The human's rulings enter in one step.
  const rulingsCalls = batchOf(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text)
  const rulingsText = rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text
  const aboutTheRun = rulingsRecordPrompt(base, 2, 'fix/rc24-tier1', [{ go: true }, { rerun: 'no-lost-files' }, { rounds: 4 }], { 'no-lost-files': [1, 2] })
  const aboutCalls = batchOf(aboutTheRun.text)
  check('the rulings step writes the go, the granted re-run — in every round an item of the clause is spent in — and the raised bound', JSON.stringify(aboutCalls) === JSON.stringify([{ argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"go":true}' }, { argv: ['round-set', '--run', 'rc24-tier1', '--round', '1'], stdin: '{"granted":"no-lost-files"}' }, { argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"granted":"no-lost-files"}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"rounds":4}' }]) && aboutTheRun.text.includes(' apply --run rc24-tier1 --round 2 --subject '))
  const granted = batchOf(rulingsRecordPrompt(base, 3, 'fix/rc24-tier1', [{ reverify: 'f-1' }, { again: 'test' }, { cycles: 4 }], {}, { round: 2, stages: { test: 1 } }).text)
  check('the rulings step writes one more triage in the latest round, one more attempt in the round the stage is spent in, and the raised cycle bound', JSON.stringify(granted) === JSON.stringify([{ argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"reverify":"f-1"}' }, { argv: ['round-set', '--run', 'rc24-tier1', '--round', '1'], stdin: '{"again":"test"}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"cycles":4}' }]))
  check('a ruling about the run writes no bound and no disposition', !aboutCalls.some((made) => ['bound-set', 'ledger-set'].includes(made.argv[0])) && aboutTheRun.text.includes('3 about the run') && rulingsText.includes('0 about the run') && !rulingsCalls.some((made) => ['round-set', 'run-set'].includes(made.argv[0])) && JSON.stringify(aboutTheRun.expect) === JSON.stringify({ calls: 4, checks: 0 }))
  check('the rulings step writes the bounds and the dispositions', JSON.stringify(rulingsCalls[0].argv) === JSON.stringify(['bound-set', '--run', 'rc24-tier1', '--bound', 'non-jigc-writer', '--scratch', '/tmp/scratch-1', '--reach', 'races against a writer that is not jigc', '--ruling', 'the stop after round 1, item 3', '--pin', 'unpinned']) && spelled(rulingsCalls[1]).startsWith('bound-set --run rc24-tier1 --bound planted-state ') && rulingsCalls[2].stdin === JSON.stringify([{ key: 'f-1', disposition: admitted, detail: 'build the robust path' }, { key: 'f-2', disposition: later }, { key: 'f-3', disposition: bound, detail: 'races against a writer that is not jigc' }]) && spelled(rulingsCalls[3]) === 'check-ledger --run rc24-tier1 -- f-1 f-2 f-3' && rulingsCalls.length === 4)
  check('a quote in a ruling cannot leave its argument', shq('it\'s a "bound" $(x) `y`') === '\'it\'\\\'\'s a "bound" $(x) `y`\'')
  // WHAT THE HUMAN RULES ABOUT THE QUEUE IS ONE RECORD, ITS CALLS IN ONE ORDER (the reshape's
  // plan, T7): another order first, then the rows put in by key, then the raise — each a
  // call of `run-set`, judged on the state the one before it leaves — whichever was said
  // first. A raise is taken only where a row of the queue is unreached, and an order or a
  // key may be what puts one there.
  const ofTheQueue = (rulings) => rulingsRecordPrompt(base, 1, 'fix/rc24-tier1', rulings, {}, { round: 1, stages: {} })
  const raisedAndReordered = [{ budget: 40 }, { order: ['route under-confirmed', 'route hunt'] }]
  check('a budget and an order ruled in one invocation are ONE record of two calls: the order, and then the raise', JSON.stringify(batchOf(ofTheQueue(raisedAndReordered).text)) === JSON.stringify([{ argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"order":["route under-confirmed","route hunt"]}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"budget":40}' }]) && JSON.stringify(ofTheQueue(raisedAndReordered).expect) === JSON.stringify({ calls: 2, checks: 0 }))
  check('the record is the same, byte for byte, whichever ruling was said first', ofTheQueue(raisedAndReordered).text === ofTheQueue(raisedAndReordered.slice().reverse()).text && ofTheQueue([{ queued: ['f-2', 'f-1'] }, { order: ['route hunt'] }, { budget: 40 }]).text === ofTheQueue([{ budget: 40 }, { queued: ['f-2', 'f-1'] }, { order: ['route hunt'] }]).text)
  check('the rows put in by key stand between the two, and the calls of the other rulings about the run before all three', JSON.stringify(batchOf(rulingsRecordPrompt(base, 2, 'fix/rc24-tier1', [{ budget: 40 }, { queued: ['f-2', 'f-1'] }, { go: true }, { rounds: 4 }, { order: ['route hunt'] }], {}, { round: 2, stages: {} }).text)) === JSON.stringify([{ argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"go":true}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"rounds":4}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"order":["route hunt"]}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"queued":["f-2","f-1"]}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"budget":40}' }]) && ofTheQueue(raisedAndReordered).text.includes('2 about the run'))

  // What a test stage records of its items: one result each, and nothing about a clause.
  const results = (units) => JSON.stringify(resultRows(units))
  check('a result per item, as it ran', results([{ item: 'a', status: 'green', reason: 'r' }, { item: 'b', status: 'green' }]) === JSON.stringify([{ item: 'a', outcome: 'green' }, { item: 'b', outcome: 'green' }]) && results([]) === '[]')
  check('a void item says why', results([{ item: 'a', status: 'void', reason: 'a step of its chain did not report' }, { item: 'b', status: 'void' }]) === JSON.stringify([{ item: 'a', outcome: 'void', reason: 'a step of its chain did not report' }, { item: 'b', outcome: 'void', reason: 'it did not run to its end' }]))
  check('a red check brings what its finding needs', results([{ item: 'gate', status: 'red', reason: 'r', door: 'jigc setup', evidence: 'two tests red' }, { item: 'ci', status: 'red', evidence: null }]) === JSON.stringify([{ item: 'gate', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `gate`', repro: 'two tests red' }, { item: 'ci', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `ci`', repro: 'the check returned red, and no evidence beside it' }]))
  check('no result names a clause, and no call sets one', !resultRows([{ item: 'a', clause: 'x', status: 'green' }]).some((row) => 'clause' in row) && !stageRecordCommands(ctx, { reporters: [], results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }]) }, rows: [], triage: [], patches: [], facts: null, keys: [] }).some((made) => made.argv[0].startsWith('clause')))
  const reRecord = stageRecordCommands(Object.assign({}, ctx, { stage: 'test' }), { reporters: ['a'], gate: null, results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }]) }, rows: [{ key: 'f-1' }], triage: [{ key: 'f-1', grade: 'breaks' }], patches: [], facts: null, keys: ['f-1'] })
  check('a re-run\'s record is its results before anything of the ledger, and no fact of the round', JSON.stringify(reRecord.map((made) => made.argv[0])) === JSON.stringify(['check-reports', 'result-set', 'ledger-add', 'triage-set', 'check-ledger']))

  // A dropped round, and a part.
  const classes = classifyCommits(['a1', 'b2', 'c3', 'd4', 'e5'], ['b2', 'd4', 'e5'], ['a1', 'c3', 'e5'])
  check('a round\'s commits by class', JSON.stringify(classes) === JSON.stringify({ records: ['b2', 'd4'], fixes: ['a1', 'c3'], mixed: ['e5'] }) && classifyCommits(['z9'], [], []).mixed.length === 1)
  const ledger = [{ key: 'f-1', disposition: 'fixed', detail: 'aaaaaaa1' }, { key: 'f-2', disposition: 'fixed', detail: 'bbbbbbb2' + '0'.repeat(32) }, { key: 'f-3', disposition: 'open', detail: null }, { key: 'f-4', disposition: 'fixed', detail: 'ccccccc3' }]
  check('a dropped round\'s fixes are open again', JSON.stringify(reopenPatches(ledger, ['aaaaaaa1' + '0'.repeat(32), 'bbbbbbb2' + '0'.repeat(32)])) === JSON.stringify([{ key: 'f-1', disposition: 'open' }, { key: 'f-2', disposition: 'open' }]) && reopenPatches(ledger, []).length === 0)
  check('a part\'s kept fixes name their new commits', JSON.stringify(repointPatches(ledger, [{ from: 'aaaaaaa1' + '0'.repeat(32), to: 'd'.repeat(40) }])) === JSON.stringify([{ key: 'f-1', disposition: 'fixed', detail: 'd'.repeat(40) }]))
  check('two commits are one by their prefix only', sameCommit('abcdef1', 'abcdef1234') && !sameCommit('abcdef1', 'abcdef2') && !sameCommit('abc', 'abcdef1') && !sameCommit('abcdef1', null))

  // The areas of a fix cycle.
  const areas = areasOf([{ key: 'f-1', door: 'jigc setup' }, { key: 'f-2', door: 'nowhere' }, { key: 'f-3', door: 'jigc doc show' }, { key: 'f-4', door: 'finalize.dirty' }], { included: [{ door: 'jigc setup', registry: 'verbs' }, { door: 'finalize.dirty', registry: 'codes' }], excluded: [{ door: 'jigc doc show', registry: 'verbs' }] })
  check('the areas', areas.map((x) => x.n + ':' + x.findings.map((f) => f.key).join('+')).join(' ') === '1:f-1+f-3 2:f-2 3:f-4' && areasOf([], null).length === 0 && areasOf([{ key: 'f', door: 'd' }], null).length === 1)
  check('a unit is handed its brief and its doors as the reads that print them, and no text of either', prompts.review.includes('\nYour brief is the `brief` of your item\'s row in the test set — read it: `dev/stabilize-record item --run rc24-tier1 --item row-3`.\n') && prompts.review.includes('is what `dev/stabilize-record item-doors --run rc24-tier1 --round 2 --item row-3` prints: 1 door(s) of round 2\'s test set') && !prompts.review.includes('Your brief: ') && !prompts.review.includes('   - ') && prompts.crossModel.includes('`dev/stabilize-record item --run rc24-tier1 --item row-3`') && prompts.crossModel.includes('`dev/stabilize-record item-doors --run rc24-tier1 --round 2 --item row-3`') && doorsRead('rc24-tier1', 1, 'row-3').endsWith(' --round 1 --item row-3'))
  check('the one unit that is this script\'s own is handed its brief and its doors as given', unitPrompt(ctx, launcher(ctx), 'x', FIX_AUDIT[0][0], { item: 'audit', brief: 'The fix diff.', doors: [{ door: 'jigc setup', registry: 'verbs' }], range: 'a..b' }, built, []).includes('\nYour brief: The fix diff.\n') && briefLine(ctx, { item: 'audit', brief: '' }) === 'Your brief: ' && doorLines(ctx, { item: 'a', doors: [] }).endsWith('\n   (none)') && doorLines(ctx, { item: 'a', doors: null, round: 3 }).includes('prints: the doors of round 3\'s test set'))
  check('a check is listed for the preflight by its item and the read that prints its brief', prompts.preflight.includes('\n   - ci: its brief is the `brief` of the item\'s row — `dev/stabilize-record item --run rc24-tier1 --item ci`'))
  check('triage is handed the ledger\'s rows as the read that prints them, held to their count', triagePrompt(ctx, launcher(ctx), 'triage-p1', handed.concat([{ reporter: 'row-3-source', report: 'a.md', findings: ['1 — x'] }]), 1).includes('Grade every finding below — 3 in all, from 2 reporter(s).') && triagePrompt(ctx, launcher(ctx), 'triage-p1', handed, 1).includes('\n- ledger — report: `completions/artifacts/rc24-tier1/ledger.md` — 2 finding(s): the rows `dev/stabilize-record untriaged --run rc24-tier1` prints') && prompts.triage.includes('\n- row-3-source — report: `a.md` — 1 finding(s):\n   - 1 — x'))

  // WHAT THIS SCRIPT READS: a step's line is a digest, or it is not read.
  const digest = (said) => {
    const body = JSON.stringify(Object.assign({}, said, { unfit: said.unfit || [], file: said.file === undefined ? 'lines/' + said.act + '.0123456789abcdef.json' : said.file, file_sha256: 'f'.repeat(64) }))
    return body.slice(0, -1) + ', "sha256": "' + sha256(body) + '"}'
  }
  const asDigest = (text, act) => readDigest({ status: 'ran', line: text }, act || 'state')
  const stateDigest = digest({ act: 'state', status: 'read', branch: 'fix/rc24-tier1', state: { run: 'rc24-tier1', opened: true, next: 'fix', ledger: 3 }, file: 'state/test-1.json' })
  check('a digest is read: what the tool printed, and the file that holds the rest', asDigest(stateDigest).status === 'read' && asDigest(stateDigest + '\n').state.ledger === 3 && asDigest(stateDigest).file === 'state/test-1.json' && !asDigest(stateDigest).relay && JSON.stringify(fileOf(asDigest(stateDigest))) === JSON.stringify({ file: 'state/test-1.json', sha256: 'f'.repeat(64) }) && fileOf({ file: null }) === null && fileOf(null) === null)
  check('the line the tool prints unasked is no digest, though it hashes: the document is not read', relay(line).status === 'read' && asDigest(line).status === 'halted' && asDigest(line).relay.includes('is no digest') && asDigest(refusedLine, 'land').relay.includes('is no digest') && !JSON.stringify(asDigest(line)).includes('rc24-tier1'))
  const dashed = printed({ act: 'state', status: 'read', state: { run: 'a — b' }, unfit: [], file: 'state/t.json', file_sha256: 'f'.repeat(64) })
  const escaped = printed({ act: 'state', status: 'read', state: { run: 'a \\ b' }, unfit: [], file: 'state/t.json', file_sha256: 'f'.repeat(64) })
  check('a line with a character outside ASCII, or a backslash, is no digest, though it hashes', relay(dashed).status === 'read' && asDigest(dashed).relay.includes('outside printable ASCII') && relay(escaped).status === 'read' && asDigest(escaped).relay.includes('a backslash') && !!asDigest(stateDigest.replace('"fix"', '"fox"')).relay && !!asDigest(stateDigest.replace('rc24-tier1', 'rc24—tier1')).relay)
  const struck = asDigest(digest({ act: 'push', status: 'ready', branch: null, head: null, unfit: ['branch', 'head'] }), 'push')
  check('a digest with a field struck out is a step that is not acted on, and names the file its own line is in', struck.status === 'halted' && struck.unfit === true && struck.said === 'ready' && struck.relay.includes('could not print branch, head') && struck.relay.includes('THE ACT MAY HAVE RUN') && struck.file === 'lines/push.0123456789abcdef.json')
  check('what is no line at all, and a step whose agent halted, are read as before', readDigest(null, 'state') === null && !!readDigest({ status: 'ran' }, 'state').relay && readDigest({ status: 'halted', halt: { root_cause: 'x' } }, 'state').halt.root_cause === 'x' && !!asDigest(stateDigest, 'push').relay && !JSON.stringify(relay(line.replace('"fix"', '"fox"'))).includes('fox'))
  const waited = digest({ act: 'hold-wait', status: 'done', name: 'gate-c1-a1', facts: { steps: [{ step: 'build', exit: 0 }, { step: 'test', exit: 1 }, { step: 'fmt', exit: 0 }] } })
  check('a digest of any act with one value slipped is read as the tool printed it', asDigest(waited.replace('"step":"fmt","exit":0', '"step":"fmt","exit":1'), 'hold-wait').status === 'done' && asDigest(waited.replace('"step":"fmt","exit":0', '"step":"fmt","exit":1'), 'hold-wait').facts.steps[2].exit === 0 && !!asDigest(waited.replace('"step":"test","exit":1', '"step":"test","exit":0'), 'hold-wait').relay)

  // HOW A STEP CAN END: every word has its row, and says whose the state is.
  check('every refusal of the tool has a row that says whose it is and what leaves it', Object.keys(STEP_REFUSALS).length === 30 && Object.keys(STEP_REFUSALS).every((word) => /^[a-z][a-z-]*$/.test(word) && isText(STEP_REFUSALS[word].whose) && isText(STEP_REFUSALS[word].leaves) && stepRefusal(word).startsWith('`' + word + '` — ' + STEP_REFUSALS[word].whose + ': ')))
  check('a word the table lacks is said to be one', stepRefusal('no-such-word').includes('has no row') && !stepRefusal('locked').includes('has no row'))
  check('a commit that is no record\'s is its author\'s, a lock the orchestrator\'s, and what would not have been written the human\'s', STEP_REFUSALS['foreign-commit'].whose === 'its author\'s' && stepRefusal('foreign-commit', 'fix/rc24-tier1').includes('`git push origin fix/rc24-tier1`') && STEP_REFUSALS.locked.whose === 'the orchestrator\'s' && STEP_REFUSALS.locked.leaves.includes('never removes') && STEP_REFUSALS['head-moved'].whose === 'the orchestrator\'s' && STEP_REFUSALS.unvetted.whose.startsWith('the human\'s at a push') && stepRefusal('unvetted', 'fix/rc24-tier1').includes('`dev/stabilize-record vet --range -- fix/rc24-tier1 --not --remotes=origin`'))
  const discards = '`dev/stabilize-step discard --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1`'
  check('a record step that stopped says what the next invocation does, and names the act that takes a batch back', [null, 'gate-red', 'head-moved', 'did-not-run', 'locked', 'no-batch'].every((word) => recordThen('rc24-tier1', 'fix/rc24-tier1', word).includes(discards) && !recordThen('rc24-tier1', 'fix/rc24-tier1', word).includes('stabilize-record discard')) && recordThen('rc24-tier1', 'fix/rc24-tier1', 'gate-red').includes('THE BATCH IS STILL APPLIED') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'head-moved').includes('its first read meets the commit') && recordThen('rc24-tier1', 'fix/rc24-tier1', null).includes('may have left its batch APPLIED AND NOT COMMITTED'))
  check('a commit refused as unvetted has taken its batch back, and asks for no discard', recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', true).includes('THE BATCH IS TAKEN BACK ALREADY') && !recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', true).includes(discards) && !recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', true).includes('pending, and no dirty tree') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', false).includes('was NOT taken back') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted').includes('was NOT taken back'))
  check('a push that failed is owed to the next first read — but never one refused as unvetted, and not beside a commit that is no record\'s', pushThen('fix/rc24-tier1', null).includes('THE PUSH IS OWED') && pushThen('fix/rc24-tier1', 'push-rejected').includes('its first read finds the record the remote lacks') && pushThen('fix/rc24-tier1', 'unvetted').includes('THE PUSH IS NOT OWED TO ANY LATER INVOCATION') && !pushThen('fix/rc24-tier1', 'unvetted').includes('THE PUSH IS OWED:') && pushThen('fix/rc24-tier1', 'foreign-commit').includes('`git push origin fix/rc24-tier1`') && !pushThen('fix/rc24-tier1', 'foreign-commit').includes('THE PUSH IS OWED:'))
  check('a step that returned nothing is asked for again as its one command, and no agent of a step is told to look around', RUN_AGAIN.includes('Run the ONE command above again') && !RUN_AGAIN.includes('git status') && !RUN_AGAIN.includes('git log') && LOOK_AGAIN.includes('the tree\'s status') && !LOOK_AGAIN.includes('`git ') && RUN_AGAIN !== LOOK_AGAIN)
  check('an invocation that only looks says so to its first read, and to no other step', gitStatePrompt(base, true).includes(' --product crates --look --digest /tmp/scratch-1`') && !steps['git-state'].includes('--look') && Object.keys(steps).every((act) => !steps[act].includes('--look')))
  check('a record\'s push on the loop branch publishes records only, and its commit is held to the commit the stage began on', pushPrompt(base, 'fix/rc24-tier1', true).includes(' push --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --digest /tmp/scratch-1`') && !steps.push.includes('--run-dir') && recordCommitPrompt(base, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }, 'c'.repeat(40)).includes(' --calls 3 --checks 2 --scratch /tmp/scratch-1 --head ' + 'c'.repeat(40) + ' --digest /tmp/scratch-1`') && !steps.record.includes('--head') && recordCommitPrompt(base, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }, 'c'.repeat(40), 'd'.repeat(64)).includes(' --head ' + 'c'.repeat(40) + ' --fence ' + 'd'.repeat(64) + ' --digest /tmp/scratch-1`') && !steps.record.includes('--fence') && STEP_REFUSALS['fence-moved'].whose === 'the human\'s' && recordThen('rc24-tier1', 'fix/rc24-tier1', 'fence-moved').includes('PAST THE WRITE GUARD'))

  // What every return names of the state: what nothing can hold, and the file.
  const namedState = { round: 2, never_selected: ['row-b'], uncovered: [{ round: 1, doors: 3 }, { round: 2, doors: 1 }], document: { file: '/tmp/scratch-1/state/test-2.json', sha256: 'f'.repeat(64) }, ledger: 60, blockers: 2, human_list: 5, untriaged: { count: 1, why: [{ why: 'ungraded', count: 1 }] } }
  check('every return names the items no round selected, the doors of the round no item names, and the file', JSON.stringify(namedOf(namedState)) === JSON.stringify({ never_selected: ['row-b'], uncovered: { round: 2, doors: 1 }, regression_unknown: 0, document: namedState.document }) && namedOf(Object.assign({}, namedState, { regression_unknown: ['f-1', 'f-2'] })).regression_unknown === 2 && namedOf(namedState, 1).uncovered.doors === 3 && namedOf(namedState, 3).uncovered.doors === 0 && namedOf(null) === null && JSON.stringify(namedOf({ opened: false })) === JSON.stringify({ never_selected: [], uncovered: { round: null, doors: 0 }, regression_unknown: 0, document: null }))
  check('what a return holds of the state is counts and words, never a list of rows', attached(namedState).ledger === 60 && attached(namedState).blockers === 2 && attached(namedState).human_list === 5 && attached(namedState).untriaged.count === 1 && !('document' in attached(namedState)) && !('items' in attached(namedState)))
  // WHAT A RETURN SAYS OF A RUN WITH A QUEUE (the reshape's plan, T7; the row
  // `wf-return-names`): the budget, the queue's length by class and the stop's view AS THE
  // STATE'S DIGEST HAS THEM, what an invocation of `test` continues the stage for — and no
  // key of the queue; and the three words for what an invocation did.
  const shown = { round: 1, file: 'completions/artifacts/rc24-tier1/r1/view.md', sha256: 'c'.repeat(64), on_disk: 'written', verified: 3, confirmed: 2, refuted: 1, undrivable: 0, rates: [{ kind: 'route', origin: 'hunt', verified: 3, confirmed: 2, refuted: 1, undrivable: 0, queued: 2 }], inside: 1, outside: 1, unreached: 2, listed: [{ why: 'lead', rows: 4 }], other: 6, codes: 1, sites: 0 }
  const queuedState = { next: 'verify', queue: { budget: 30, hand_on: 2, spent: 3, left: 27, length: 2, first: ['f-1', 'f-2'], classes: [{ kind: 'route', origin: 'hunt', length: 2 }] }, view: shown, position: { test: { round: 1, attempt: 2, verify: true, filed: true } } }
  check('a return of a run with a queue says the budget, the queue by class and the stop\'s view as the digest has them, and what the stage is continued for', JSON.stringify(queueSaid(queuedState)) === JSON.stringify({ continues: ['verify', 'filed'], budget: { limit: 30, spent: 3, left: 27 }, queue: { length: 2, hand_on: 2, classes: [{ kind: 'route', origin: 'hunt', length: 2 }] }, view: shown }))
  check('no key of the queue is in it, and a run without an order says none of it', !JSON.stringify(queueSaid(queuedState)).includes('f-1') && JSON.stringify(queueSaid({ next: 'fix', queue: null, position: { test: { refused: 'round-open', round: 1 } } })) === '{}' && JSON.stringify(queueSaid(null)) === '{}')
  check('only a record is owed where the stage is continued for a filed verdict and nothing else, and nothing where the position hands no continuation', JSON.stringify(queueSaid(Object.assign({}, queuedState, { position: { test: { round: 1, attempt: 2, filed: true } } })).continues) === '["filed"]' && JSON.stringify(queueSaid(Object.assign({}, queuedState, { position: { test: { refused: 'round-open', round: 1 } } })).continues) === '[]' && queueSaid(Object.assign({}, queuedState, { view: undefined })).view === null)
  check('what an invocation says happened: it hunted, it verified, or it continued a stage and recorded no verdict', statusOf([], null) === 'hunted' && statusOf([], 'a'.repeat(40)) === 'verified' && statusOf(['verify'], 'a'.repeat(40)) === 'verified' && statusOf(['filed'], 'a'.repeat(40)) === 'verified' && statusOf(['verify'], null) === 'continued' && statusOf(['triage', 'verify'], null) === 'continued')

  // A held command, as this script reads it: the tool's words, and the tool's verdict.
  const job = { act: 'hold-wait', name: 'gate-c1-a1', kind: 'gate', output: 'hold/gate-c1-a1/output' }
  const doneAs = (verdict, why) => heldOf(Object.assign({ status: 'done', verdict, why: why || null, verdict_file: 'hold/gate-c1-a1/verdict.json', verdict_sha256: 'e'.repeat(64) }, job), '/tmp/scratch-1')
  check('a held check is an item whose kind names a held command of the tool', heldKind('held-regression') === 'regression' && heldKind('held-gate') === 'gate' && heldKind('held-build') === null && heldKind('held-') === null && heldKind('check') === null && heldKind('regression') === null && heldKind(null) === null && HELD_CHECKS.every((id) => !CHAINS[HELD_PREFIX + id] && HELD_PREFIX + id !== CHECK_KIND))
  const startedWith = (previous, candidate) => holdStartPrompt(base, 'regression-set-' + candidate.slice(0, 12) + '-a1', 'regression', regressionFlags(previous, candidate), 'the regression set')
  check('a held regression set is started with the two commits, the list and the previous release\'s kept baseline — a path below the scratch root named by that release\'s commit and by nothing else: two candidates name one, two previous releases two, and it lies in no job\'s directory', startedWith('c'.repeat(40), 'd'.repeat(40)).includes('\n1. `dev/stabilize-step hold-start --scratch /tmp/scratch-1 --name regression-set-dddddddddddd-a1 --kind regression --previous ' + 'c'.repeat(40) + ' --candidate ' + 'd'.repeat(40) + ' --list ' + REGRESSION_LIST + ' --baseline baseline/previous-cccccccccccc --digest /tmp/scratch-1`\n') && startedWith('c'.repeat(40), 'e'.repeat(40)).includes(' --baseline baseline/previous-cccccccccccc ') && startedWith('f'.repeat(40), 'd'.repeat(40)).includes(' --baseline baseline/previous-ffffffffffff ') && baselineOf.length === 1 && !baselineOf('c'.repeat(40)).startsWith('hold/') && !baselineOf('c'.repeat(40)).startsWith('/') && baselineOf('c'.repeat(40)).split('/').every((part) => /^[A-Za-z0-9_][A-Za-z0-9._-]*$/.test(part)))
  check('a held command that still runs is asked for again', heldOf(Object.assign({ status: 'running' }, job), '/tmp/scratch-1').ends === 'running' && heldOf(Object.assign({ status: 'started' }, job, { act: 'hold-start' }), '/tmp/scratch-1').ends === 'running' && heldOf(Object.assign({ status: 'running' }, job), '/tmp/scratch-1').output === '/tmp/scratch-1/hold/gate-c1-a1/output' && heldOf(Object.assign({ status: 'started' }, job), '/tmp/scratch-1').fresh === true && heldOf(Object.assign({ status: 'running' }, job), '/tmp/scratch-1').fresh === false && doneAs('green').fresh === false)
  check('a held command that ran to its end is the tool\'s verdict, by the file the tool kept it in', ['green', 'red', 'void'].every((verdict) => doneAs(verdict).ends === verdict && doneAs(verdict).verdict === '/tmp/scratch-1/hold/gate-c1-a1/verdict.json' && doneAs(verdict).verdict_sha256 === 'e'.repeat(64)) && doneAs('void', 'no-verdict').why === 'no-verdict' && doneAs('green').why === null)
  check('a held command that is dead has no verdict, and one that is done without the tool\'s verdict is not read', heldOf(Object.assign({ status: 'dead', why: 'no-exit' }, job), '/tmp/scratch-1').ends === 'dead' && heldOf(Object.assign({ status: 'dead', why: 'killed', verdict: 'green', verdict_file: 'hold/x/verdict.json', verdict_sha256: 'e'.repeat(64) }, job), '/tmp/scratch-1').verdict === undefined && doneAs('passed').ends === 'unread' && heldOf(Object.assign({ status: 'done', verdict: 'green' }, job), '/tmp/scratch-1').ends === 'unread' && heldOf(null, '/tmp/scratch-1').ends === 'unread' && heldOf({ status: 'halted', refused: 'missing' }, '/tmp/scratch-1').refused === 'missing' && heldOf({ status: 'halted', relay: 'altered' }, '/tmp/scratch-1').why === 'altered')
  check('every way a held command can stand has its sentence', JSON.stringify(Object.keys(HELD_ENDS)) === JSON.stringify(['running', 'green', 'red', 'void', 'dead', 'unread']) && Object.keys(HELD_ENDS).every((ends) => isText(HELD_ENDS[ends])))
  check('a held check that left no verdict is void by ONE word of the shape the record script takes: the tool\'s own word where it refused the start, how the job stood otherwise, and the job\'s name', heldVoid({ ends: 'not-run', kind: 'gate', why: 'off-tree' }) === 'off-tree' && heldVoid({ ends: 'dead', why: 'no-exit', name: 'regression-set-c1-a1' }) === 'dead:regression-set-c1-a1' && heldVoid({ ends: 'running', name: 'reg-c1-a2' }) === 'running:reg-c1-a2' && heldVoid({ ends: 'unread', refused: 'taken', name: 'reg-c1-a1' }) === 'taken:reg-c1-a1' && heldVoid({ ends: 'unread', why: 'the step returned no result', name: 'reg-c1-a1' }) === 'unread:reg-c1-a1' && heldVoid({ ends: 'unread', kind: 'regression', why: 'no name of a held command can be made' }) === 'unread' && Object.keys(HELD_ENDS).filter((ends) => !HELD_TAKES.check.includes(ends)).concat(['not-run']).every((ends) => HELD_VOID_RE.test(heldVoid({ ends, name: 'x'.repeat(SLUG_MAX) })) && heldVoid({ ends, name: 'x'.repeat(SLUG_MAX) }).length <= 200) && Object.keys(STEP_REFUSALS).every((word) => HELD_VOID_RE.test(heldVoid({ ends: 'unread', refused: word, name: 'a-b' }))) && !HELD_VOID_RE.test('it did not run to its end') && !HELD_VOID_RE.test('dead:/tmp/x') && JSON.stringify(resultRows([{ item: 'reg', status: 'void', reason: heldVoid({ ends: 'dead', name: 'reg-c1-a1' }) }])) === JSON.stringify([{ item: 'reg', outcome: 'void', reason: 'dead:reg-c1-a1' }]))
  const knows = { status: 'verified', verdict: 'confirmed', regression: true, asserted_sha256: 'b'.repeat(64), ran_on: { candidate: 'b'.repeat(64), previous: 'c'.repeat(64) } }
  const without = (drop) => { const r = JSON.parse(JSON.stringify(knows)); drop(r); return r }
  check('a regression fact is taken only as a boolean with the previous binary\'s hash behind it, and is unknown otherwise — never false', JSON.stringify(regressionOf('v', knows, 'c'.repeat(64))) === JSON.stringify({ regression: true, unknown: null }) && regressionOf('v', without((r) => { r.regression = false }), 'c'.repeat(64)).regression === false && [without((r) => { delete r.regression }), without((r) => { delete r.ran_on.previous }), without((r) => { delete r.ran_on }), without((r) => { r.regression = 'no' }), without((r) => { r.ran_on.previous = 'd'.repeat(64) }), without((r) => { r.regression = null })].every((r) => regressionOf('v', r, 'c'.repeat(64)).regression === null && isText(regressionOf('v', r, 'c'.repeat(64)).unknown)) && regressionOf('verify-p1-k', without((r) => { delete r.regression }), 'c'.repeat(64)).unknown.startsWith('(verify-p1-k) returned no `regression` — ') && regressionOf('v', without((r) => { delete r.ran_on.previous }), 'c'.repeat(64)).unknown.startsWith('(v) returned no `ran_on.previous` — ') && regressionOf('v', without((r) => { r.ran_on.previous = 'x`; rm -r /' }), 'c'.repeat(64)).unknown.includes('a hash that is no sha256') && !regressionOf('v', without((r) => { r.ran_on.previous = 'x`; rm -r /' }), 'c'.repeat(64)).unknown.includes('rm -r'))
  check('a confirmed finding whose regression fact is unknown is recorded with the field LEFT OUT, and one that is known with its boolean', JSON.stringify(triageRecord(ctx, [{ key: 'f-1', new: false, grade: 'breaks', verdict: 'confirmed', regression: null }, { key: 'f-2', new: false, grade: 'breaks', verdict: 'confirmed', regression: false }, { key: 'f-3', new: false, grade: 'breaks', verdict: 'confirmed' }, { key: 'f-4', new: false, grade: 'breaks', verdict: 'refuted', regression: true }]).triage) === JSON.stringify([{ key: 'f-1', grade: 'breaks', verdict: 'confirmed' }, { key: 'f-2', grade: 'breaks', verdict: 'confirmed', regression: false }, { key: 'f-3', grade: 'breaks', verdict: 'confirmed' }, { key: 'f-4', grade: 'breaks', verdict: 'refuted' }]))
  check('a door the previous release does not have is NOT COMPARABLE: taken with that binary\'s hash behind it, unknown without, and recorded as it is', regressionOf('v', without((r) => { r.regression = NOT_COMPARABLE }), 'c'.repeat(64)).regression === NOT_COMPARABLE && regressionOf('v', without((r) => { r.regression = NOT_COMPARABLE; delete r.ran_on.previous }), 'c'.repeat(64)).regression === null && regressionOf('v', without((r) => { r.regression = 'not comparable' }), 'c'.repeat(64)).regression === null && JSON.stringify(triageRecord(ctx, [{ key: 'f-1', new: false, grade: 'breaks', verdict: 'confirmed', regression: NOT_COMPARABLE }]).triage) === JSON.stringify([{ key: 'f-1', grade: 'breaks', verdict: 'confirmed', regression: NOT_COMPARABLE }]))
  check('what a stage holds takes the ends its table names, and no other', JSON.stringify(HELD_TAKES) === JSON.stringify({ build: ['green'], gate: ['green', 'red'], check: ['green', 'red', 'void'], record: ['green', 'red'], probe: ['green'] }) && Object.keys(HELD_TAKES).every((use) => HELD_TAKES[use].every((ends) => HELD_ENDS[ends])) && !Object.keys(HELD_TAKES).some((use) => ['running', 'dead', 'unread'].some((ends) => HELD_TAKES[use].includes(ends))))
  check('a held command that may still run is named with the one call that answers once it is over', mayRun({ name: 'gate-c1-a1', ends: 'running' }) && mayRun({ name: 'gate-c1-a1', ends: 'unread', started: true }) && !mayRun({ name: 'gate-c1-a1', ends: 'unread', started: false }) && !mayRun({ name: 'gate-c1-a1', ends: 'dead' }) && !mayRun({ name: 'gate-c1-a1', ends: 'void' }) && !mayRun({ ends: 'running' }) && heldThen('/tmp/scratch-1', { name: 'gate-c1-a1', ends: 'running', output: '/tmp/scratch-1/hold/gate-c1-a1/output' }).includes('`dev/stabilize-step hold-wait --scratch /tmp/scratch-1 --name gate-c1-a1`') && heldThen('/tmp/scratch-1', { name: 'gate-c1-a1', ends: 'running' }).includes('MAY STILL RUN') && heldThen('/tmp/scratch-1', { name: 'gate-c1-a1', ends: 'dead' }) === null)
  check('a held command that did not end as the stage needs it is said by its job and the tool\'s word', heldFault('the candidate\'s gate', { name: 'gate-c1-a1', ends: 'void', why: 'no-verdict' }).includes('`gate-c1-a1`') && heldFault('the candidate\'s gate', { name: 'gate-c1-a1', ends: 'void', why: 'no-verdict' }).includes('`no-verdict`') && heldFault('x', { ends: 'unread' }).includes(HELD_ENDS.unread) && JSON.stringify(heldSaid({ name: 'n', kind: 'gate', ends: 'dead', why: 'no-exit', output: '/o', verdict: '/v', facts: {} })) === JSON.stringify({ name: 'n', kind: 'gate', ends: 'dead', why: 'no-exit', output: '/o' }))
  check('a command an agent types is rendered by one function, as one code span', typed(STEP_TOOL, 'table') === '`dev/stabilize-step table`' && typed(RECORD_TOOL, '') === '`dev/stabilize-record`' && RECORD_TOOL === 'dev/stabilize-record' && isText(REGRESSION_LIST) && !REGRESSION_LIST.startsWith('/') && HOLD_LAPS >= 1 && HOLD_NAMES >= 2)
  check('the subject of a record is plain words, and so is every word of the call that applies it', /^[A-Za-z0-9 _.\/:=@%+,()-]+$/.test('docs(record): ' + subjectOf('rc24-tier1', 2, 'the record of the test stage')) && subjectOf('rc24-tier1', 2, 'a re-run of no-lost-files') === 'rc24-tier1 r2 - a re-run of no-lost-files' && /--subject 'docs\(record\): rc24-tier1 r1 - the rulings of the human' /.test(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', [{ go: true }], {}, { round: 1, stages: {} }).text))
  check('the scope and a report reach their writer from a file the agent writes with its file tool', prompts.scope.includes('to the file `/tmp/scratch-1/scope/r2.a4.json` WITH YOUR FILE TOOL') && prompts.scope.includes('`dev/stabilize-record scope-set --run rc24-tier1 --round 2 --scratch /tmp/scratch-1 --from /tmp/scratch-1/scope/r2.a4.json`') && prompts.proposal.includes(reportCall(ctx, 'proposal-f-1')) && prompts.crossModel.includes(reportCall(ctx, 'row-3-crossmodel')) && [prompts.proposal, prompts.crossModel, prompts.scope, prompts.review].every((text) => !text.includes('here-document')))
  check('the one driving role with no definition asks the tool for the binary\'s hash', prompts.proposal.includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc`') && prompts.proposal.includes('`content_sha256` must be ' + 'b'.repeat(64)) && !prompts.proposal.includes('shasum') && below('/tmp/scratch-1', '/tmp/scratch-1/bin/c2/jigc') === 'bin/c2/jigc' && below('/tmp/scratch-1', '/elsewhere/jigc') === '/elsewhere/jigc')
  check('a held check\'s result is the file of the tool\'s verdict, and no word', results([{ item: 'regression-set', status: 'green', verdict: '/tmp/scratch-1/hold/regression-c1-a1/verdict.json' }, { item: 'gate', status: 'red', evidence: 'e' }]) === JSON.stringify([{ item: 'regression-set', verdict: '/tmp/scratch-1/hold/regression-c1-a1/verdict.json' }, { item: 'gate', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `gate`', repro: 'e' }]))
  check('a refusal of a batch as one that changes nothing is the record script\'s word', NO_CHANGE === 'no-change' && RECORD_SCHEMA.properties.refused.type === 'string' && !RECORD_SCHEMA.required.includes('refused') && RECORD_RETURNS.includes('return refused = the ONE word'))

  // The runtime probes: what their agents are told, and what is handed to the tool.
  const probeBegin =probeStep('its first act', 'begin', '--scratch /tmp/scratch-1 --probe relay')
  check('a probe\'s step is the harness\'s own step prompt, with the probe tool where the step tool stands', probeBegin === stepPrompt('a PROBE of the stabilization harness, of no run — its first act', 'begin', '--scratch /tmp/scratch-1 --probe relay').split(STEP_TOOL).join(PROBE_TOOL) && probeBegin.includes('\n1. `dev/stabilize-probe begin --scratch /tmp/scratch-1 --probe relay`\n') && probeBegin.includes(STEP_RULES) && probeBegin.includes(AS_PRINTED) && !probeBegin.includes(STEP_TOOL))
  check('the field a reviewer came back with, as a cell — and nothing an agent wrote', probeField(null) === 'none' && probeField({ status: 'halted', asserted_sha256: 'a'.repeat(64) }) === 'halted' && probeField({ status: 'reported', findings: [] }) === 'absent' && probeField({ status: 'reported', asserted_sha256: null }) === 'absent' && probeField({ status: 'reported', asserted_sha256: 'a'.repeat(64) }) === 'a'.repeat(64) && probeField({ status: 'reported', asserted_sha256: 'x\'; rm -r /' }) === 'malformed' && probeField({ status: 'reported', asserted_sha256: 'A'.repeat(64) }) === 'malformed' && probeField({ status: 'reported', asserted_sha256: 7 }) === 'malformed')
  check('how the tries of a call ended', triesOf([{ ended: 'nothing' }, { ended: 'threw', message: 'm' }, { ended: 'returned' }]) === 'ntr' && triesOf([{ ended: 'returned' }]) === 'r' && triesOf([{ ended: 'nothing' }, { ended: 'nothing' }, { ended: 'nothing' }]) === 'nnn')
  check('the bytes of a text', utf8Length('') === 0 && utf8Length('a\u00e9\u2014\ud83d\ude00') === 1 + 2 + 3 + 4)
  const probeLine = printed({ act: 'state', status: 'read', state: { run: 'probe', door: 'a \u2014 b' } })
  const probeOwn = sha256(JSON.stringify({ act: 'state', status: 'read', state: { run: 'probe', door: 'a \u2014 b' } }))
  const relayed = (text) => probeRelayed({ status: 'ran', line: text }, readStep({ status: 'ran', line: text }, 'state', true))
  check('a relayed line that holds is its own hash, and its bytes', relayed(probeLine) === probeOwn + ':' + utf8Length(probeLine) && relayed(probeLine + '\n') === probeOwn + ':' + utf8Length(probeLine))
  check('a relayed line that does not hold is altered, and its bytes', relayed(probeLine.replace('probe', 'prob')) === 'altered:' + (utf8Length(probeLine) - 1) && relayed('not json') === 'altered:8' && relayed(probeLine.replace('\u2014', '-')) === 'altered:' + (utf8Length(probeLine) - 2))
  check('a probe reads the relay exactly: a line with one value slipped is altered there, though a stage reads it', relayed(slipped) === 'altered:' + utf8Length(slipped) && relay(slipped).status === 'read')
  check('no line came back', probeRelayed(null, null) === 'none:0' && probeRelayed({ status: 'ran' }, readStep({ status: 'ran' }, 'state')) === 'none:0' && probeRelayed({ status: 'halted', halt: { root_cause: 'x' } }, readStep({ status: 'halted', halt: { root_cause: 'x' } }, 'state')) === 'none:0')
  const probeReview = (id) => probeReviewPrompt(id, { file: '/tmp/scratch-1/probe/required/bin/jigc', sha256: 'c'.repeat(64) })
  check('a probe\'s reviewer is handed the binary line a stage\'s reviewer is, and no report', probeReview('b').includes('\nBINARY: candidate `/tmp/scratch-1/probe/required/bin/jigc` sha256 ' + 'c'.repeat(64) + ' (commit ' + '0'.repeat(40) + ', label probe). Return the sha256 you asserted for the candidate as `asserted_sha256`.\n') && !probeReview('b').includes('\n' + LABELS.report) && !probeReview('a').includes('\n' + LABELS.report))
  check('only case a is told to leave the field out', probeReview('a').includes('Leave `asserted_sha256` OUT of your return') && !probeReview('b').includes('OUT of your return') && probeReview('a').replace('case a', 'case b').startsWith(probeReview('b') + '\n'))
  check('the probe\'s schema requires the hash, and a stage\'s does not', PROBE_UNIT_SCHEMA.required.includes('asserted_sha256') && !UNIT_SCHEMA.required.includes('asserted_sha256') && PROBE_UNIT_SCHEMA.properties === UNIT_SCHEMA.properties)
  const probed = PROBE_ENTRIES.map((n) => probeBatch('/tmp/scratch-1', n))
  const probedCalls = (batch) => JSON.parse(batch.write.split('\n')[2])
  check('a probe\'s batch is a stage\'s record of that many findings, under the scratch root', probed.every((batch, i) => batch.file === '/tmp/scratch-1/probe/payload/e' + PROBE_ENTRIES[i] + '/batch.json' && JSON.stringify(probedCalls(batch).map((made) => made.argv[0])) === JSON.stringify(['check-reports', 'ledger-add', 'triage-set', 'check-ledger']) && JSON.parse(probedCalls(batch)[1].stdin).length === PROBE_ENTRIES[i] && JSON.parse(probedCalls(batch)[2].stdin).length === PROBE_ENTRIES[i] && probedCalls(batch)[3].argv.length === 4 + PROBE_ENTRIES[i]))
  check('a probe\'s batch is held to the hash of its text, and grows with its entries', probed.every((batch) => batch.sha256 === sha256(batch.write.split('\n')[2] + '\n') && batch.write.split('\n')[1] === PAYLOAD_ENDS && batch.write.split('\n')[3] === PAYLOAD_ENDS && batch.write.startsWith('Write the file `' + batch.file + '` WITH YOUR FILE TOOL')) && probed.every((batch, i) => i === 0 || batch.write.length > probed[i - 1].write.length) && distinct(probed.map((batch) => batch.sha256)))
  const probePayload = probePayloadPrompt('/tmp/scratch-1', 20, probed[0])
  check('a probe\'s executor writes the payload as a record step does, hashes it, and applies nothing', probePayload.startsWith(LABELS.record + ' — a PROBE') && probePayload.includes('\n' + RECORD_RUNS + '\n1. ' + probed[0].write + '\n2. `dev/stabilize-probe hash --scratch /tmp/scratch-1 --file /tmp/scratch-1/probe/payload/e20/batch.json`') && !probePayload.includes('stabilize-record apply') && !probePayload.includes('dev/gate') && !probePayload.includes('shasum') && !probePayload.includes(LABELS.branch))
  const probeHeld = holdStartPrompt({ scratch: '/tmp/scratch-1' }, 'a', 'probe', '--seconds 720', 'the hold probe\'s long command, case a, of 720 seconds')
  check('a probe\'s long command is held as a stage\'s gate is: started by one plain call of the step tool, and asked after by another', probeHeld.includes('\n1. `dev/stabilize-step hold-start --scratch /tmp/scratch-1 --name a --kind probe --seconds 720 --digest /tmp/scratch-1`\n') && !probeHeld.includes('\n' + LABELS.report) && !/background|slices|2>&1/.test(probeHeld) && holdWaitPrompt({ scratch: '/tmp/scratch-1' }, 'a', 'x').includes('`dev/stabilize-step hold-wait --scratch /tmp/scratch-1 --name a --digest /tmp/scratch-1`'))
  check('a slice of a read is under the tool\'s default timeout, and a hold past its ceiling', PROBE_SLICE < 120 && PROBE_HOLD > 600 && PROBE_HOLD <= PROBE_HOLD_MAX)
  check('no prompt of a probe names a run, a round or a branch', [probeBegin, probeReview('a'), probePayload, probeHeld].every((text) => !text.includes('fix/') && !text.includes(RUNS_ROOT) && !text.includes('--run ') && !text.includes('--round ') && !text.includes('--branch ') && !text.includes('--loop ')))

  return { status: failed.length ? 'self-test-failed' : 'self-test-passed', checks, failed }
}

// ---- args: parsed, and refused before any agent ----
// args sometimes arrives JSON-stringified (milestone-build.js's M24 guard): a string that
// parses to an object is that object.
let parsedArgs = args
if (typeof args === 'string') {
  const s = args.trim()
  if (s.startsWith('{')) {
    try {
      parsedArgs = JSON.parse(s)
    } catch (_) { /* not JSON: refused below, as a string */ }
  }
}
const refusal = validateArgs(parsedArgs)
if (refusal) {
  return { status: 'refused', message: refusal + '. Nothing was run. Usage: Workflow({ scriptPath: \'<the repository\'s root>/.claude/workflows/stabilize.js\', args: { stage: \'test\' | \'fix\', run: \'<run>\', scratch: \'<absolute dir>\' } }) — by the script\'s path, never by its name; the script\'s header has the rest.' }
}
if (parsedArgs.selfTest) return selfTest()
const unfit = notFit(parsedArgs)
if (unfit) {
  return { status: 'refused', stage: parsedArgs.stage, run: parsedArgs.run, not_fit: { stage: parsedArgs.stage, record: BUILD_RECORD }, message: unfit + '.' }
}
const v = parsedArgs
const model = v.model ? String(v.model) : 'opus'
const loopBranch = branchName(v.run)
const cycleLimit = v.raise ? v.raise.cycles : DEFAULT_CYCLES

// ---- running agents ----
// agentR — an agent() call retried on a transient failure (a throw, or a null return: a
// subagent that died on a terminal API error), with milestone-build.js's rate-limit breaker:
// once two DIFFERENT calls have exhausted their retries the run stops spawning — a call that
// is between two of its own tries included. A retry is
// told what a dead attempt may have left; it is never told to clean the tree, because the
// untracked files under the run's directory are other agents' reports.
const TRANSIENT_RETRIES = 2
let exhaustedLabels = []
let breakerTripped = false
// How each try of a call ended, kept only while a probe watches one (`watched`): null in
// every stage, where nothing reads it.
let triesSeen = null
// `seen`, where a caller hands one: how often the call was tried (`tries`) — what tells a
// held command this invocation's own dead try started from one that was there before it.
// THE BREAKER IS ASKED BEFORE EVERY TRY, not only before a call's first (the reshape's plan,
// T6b; the row `wf-breaker-parallel`, in part): a call that is between two tries when the
// breaker trips — beside others, in a `parallel` — is not spawned again.
// `once`: A CALL THAT IS NEVER TRIED TWICE — a verifier's, in a run with a queue (that plan's
// §2.2 and E7). What it returned nothing for is ONE FINDING LEFT UNVERIFIED, which its wave
// counts (`verifyWaves`: two in a wave make it the last) — so such a call exhausts nothing
// and feeds no breaker: two stray deaths in two waves must not end every step that follows,
// the stage's record among them. EVERY OTHER ROLE IS RETRIED AS IT WAS.
async function agentR(prompt, opts, again, seen, once) {
  const lbl = (opts && opts.label) ? opts.label : 'agent'
  const tries = once ? 1 : TRANSIENT_RETRIES + 1
  let lastErr
  for (let attempt = 0; attempt < tries; attempt++) {
    if (breakerTripped) {
      log('rate-limit breaker is tripped — NOT spawning ' + lbl + (attempt ? ' again (try ' + (attempt + 1) + ' of ' + tries + ')' : ''))
      return null
    }
    const note = attempt === 0 ? '' : '\n\n' + (again || LOOK_AGAIN)
    if (seen) seen.tries = attempt + 1
    try {
      const result = await agent(prompt + note, Object.assign({ model }, opts))
      if (triesSeen) triesSeen.push({ ended: result != null ? 'returned' : 'nothing' })
      if (result != null) return result
      lastErr = new Error('agent returned null')
      log('null return on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + tries + ')')
    } catch (e) {
      lastErr = e
      if (triesSeen) triesSeen.push({ ended: 'threw', message: String((e && e.message) || e) })
      log('transient failure on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + tries + ') — ' + ((e && e.message) || e))
    }
  }
  if (once) return null
  log('exhausted ' + (TRANSIENT_RETRIES + 1) + ' attempts on ' + lbl + ' (' + ((lastErr && lastErr.message) || lastErr) + ')')
  if (!exhaustedLabels.includes(lbl)) exhaustedLabels.push(lbl)
  if (exhaustedLabels.length >= 2 && !breakerTripped) {
    breakerTripped = true
    log('TWO different agents exhausted their retries (' + exhaustedLabels.join(', ') + ') — the shape of an account-level rate limit, not of a fault in the run. No further agent is spawned.')
  }
  return null
}
// Every git step, and every read of the record, is a build-git call on Sonnet.
function gitStep(label, phaseTitle, prompt, schema, again, seen) {
  return agentR(prompt, { label: 'git:' + label, phase: phaseTitle, agentType: 'build-git', schema, model: GIT_MODEL }, again, seen)
}
// toolStep — a git step that is ONE command of the tool: what the command `act` printed,
// read off the relayed line — its DIGEST, or it is not read. Null when the agent returned
// nothing, after the command was asked for again.
async function toolStep(label, phaseTitle, act, prompt, seen) {
  return readDigest(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA, RUN_AGAIN, seen), act, 'git:' + label)
}
// hold — ONE LONG COMMAND, HELD BY THE TOOL: started by one step, asked after by another, and
// judged by the tool, whose verdict comes back as what `heldOf` reads. `job` is { name, kind,
// flags, what, phase } and, for a record's gate, `sequence`; for what is named by its commit
// — a build, the candidate's gate, a held check — `stands`.
//   THE START is one plain call, and IDEMPOTENT BY NAME: a start step whose agent died is
//   asked for again as its one command, and the tool answers with the job that is there —
//   nothing is started twice. WHICH ANSWER OF A START IS THE JOB THIS INVOCATION HOLDS, AND
//   WHICH IS PASSED OVER FOR THE NEXT NAME, is one table (`passedOver`). With `sequence` the
//   name is `<name>-<n>`: a job that answers a start as one that was there is an EARLIER
//   INVOCATION'S — a gate of the tree as it stood then — and the next name is asked. With
//   `stands` the name is the commit's, and A JOB THAT WAS THERE IS THE ANSWER while it runs
//   — its wait is taken over — or where its verdict is green: nothing is built, gated or
//   checked twice for one commit. One that ended red, void or dead IS PASSED OVER, and the
//   next name is asked — `<name>-2`, `<name>-3`, … — so that a red a starved machine left
//   is measured again and a dead job is never the answer for ever. `flags` may then be a
//   function of the name's number: what is written under a name is written once.
//   THE WAIT is one plain call an agent runs again while the tool says `running`. A WAIT
//   WHOSE AGENT DIED IS TAKEN OVER, never started again: the step's retry is the wait, and
//   the command, which outlives every agent, ran once. An agent whose turn ended on
//   `running` is followed by the next, HOLD_LAPS in turn.
// It returns how the command stands — `running` and `unread` among them, with `started`:
// whether a command of this name is there, and may still run; and `exhausted` where every
// name the job may be held under answered as taken already: none is left under this root.
async function hold(job) {
  let started = null
  let name = job.name
  let asked = 0
  for (let n = 1; n <= (job.sequence || job.stands ? HOLD_NAMES : 1) && !started; n++) {
    name = job.sequence || (job.stands && n > 1) ? job.name + '-' + n : job.name
    asked = n
    if (!isSlug(name)) return { ends: 'unread', kind: job.kind, why: 'no name of a held command can be made of `' + name + '`: it must be a slug of at most ' + SLUG_MAX + ' characters' }
    const seen = { tries: 0 }
    const at = heldOf(await toolStep('hold-start:' + name, job.phase, 'hold-start', holdStartPrompt(v, name, job.kind, typeof job.flags === 'function' ? job.flags(n) : job.flags, job.what), seen), v.scratch)
    const over = passedOver(job, at, seen.tries)
    if (over) {
      log(over === 'running'
        ? 'the held command `' + name + '` was running before this invocation asked for it — an earlier invocation\'s: it is not read as this one\'s, and the next name is asked'
        : 'the held command `' + name + '` answered its start as one that had ended already (' + at.ends + (at.why ? ', `' + at.why + '`' : '') + ') — this call did not start it, and the tool does not say which call did: ' + (job.sequence ? 'it is not read as the gate of this record\'s tree' : 'it is passed over — a job that stands is the answer only while it runs or where it is green') + ', and the next name is asked')
      continue
    }
    started = at
  }
  if (!started) return { ends: 'unread', name, kind: job.kind, exhausted: true, why: 'every name from `' + job.name + (job.sequence ? '-1' : '') + '` to `' + name + '` answered its start with a command that was there already under this scratch root' + (job.stands ? ', each ended red, void or dead' : '') + ': a name is one command\'s, and none is left' }
  // A start the tool REFUSED started nothing; one whose agent returned nothing, or whose
  // line did not come back, may have.
  if (started.ends === 'unread') return Object.assign({ name, kind: job.kind }, started, { started: !started.refused })
  let now = started
  for (let lap = 1; now.ends === 'running' && lap <= HOLD_LAPS; lap++) now = Object.assign({ name, kind: job.kind, output: started.output }, heldOf(await toolStep('hold-wait:' + name + (lap > 1 ? ':lap' + lap : ''), job.phase, 'hold-wait', holdWaitPrompt(v, name, job.what)), v.scratch))
  return Object.assign(now, { started: true, asked })
}
// buildOf — the `jigc` binary of a commit, BUILT BY A HELD COMMAND and read off the tool's
// verdict: its path, its hash, the version it prints and that a bare `jigc` resolves to it
// are the tool's facts, held here to what was asked, and no agent's reading (ruling 11; the
// re-review's R-L3). Anything but a green build halts the stage: no instrument runs
// without the binary.
// A BUILD IS NAMED BY ITS COMMIT (`whose`: `candidate`, or `previous`), so the binary of a
// commit that is built under this scratch root is the one the tool answers with — and where
// a job of that name stands red, void or dead, the next name is asked, and writes its
// binary at a path of its own: a binary is written once.
async function buildOf(whose, sha, version, what, phaseTitle) {
  const short = sha.slice(0, 12)
  const at = (n) => 'bin/' + whose + '-' + short + (n > 1 ? '-' + n : '') + '/jigc'
  const job = await hold({ name: 'build-' + whose + '-' + short, stands: true, kind: 'build', flags: (n) => '--commit ' + sha + ' --to ' + at(n) + (version ? ' --version ' + version : ''), what, phase: phaseTitle })
  if (!HELD_TAKES.build.includes(job.ends)) return { fault: heldFault(what, job) + ' No instrument runs without the binary.', phase: 'build', held: heldSaid(job), transient: job.ends === 'unread' || job.ends === 'running', then: heldThen(v.scratch, job) }
  const to = at(job.asked)
  const f = job.facts
  if (f.commit !== sha || f.binary !== to || !SHA256_RE.test(String(f.content_sha256 || '')) || f.resolves !== true || (version && f.version !== version)) return { fault: what + ' is not the binary that was asked for: the tool\'s verdict names ' + JSON.stringify({ commit: f.commit, binary: f.binary, version: f.version, resolves: f.resolves }), phase: 'build', held: heldSaid(job) }
  return { binary: v.scratch + '/' + to, sha256: f.content_sha256, version: f.version }
}
// buildBoth — the ONE binary a candidate is driven as, and the previous release's, which
// every regression fact is measured against: each from its commit, never from the working
// tree. EACH IS NAMED BY ITS COMMIT, so that one that is built already under this scratch
// root — by an earlier attempt, a re-run, a lap — is the one the tool answers with.
async function buildBoth(label, sha, previous, phaseTitle) {
  const built = await buildOf('candidate', sha, null, 'the candidate\'s binary, label ' + label, phaseTitle)
  if (built.fault) return built
  const before = await buildOf('previous', String(previous.commit), previous.version, 'the previous release\'s binary, version ' + previous.version, phaseTitle)
  if (before.fault) return before
  return { candidate: { label, sha, binary: built.binary, sha256: built.sha256 }, previous: { version: before.version, binary: before.binary, sha256: before.sha256 }, image: null }
}
// (`once`: a call that is never tried twice — `agentR`.)
function roleStep(role, label, phaseTitle, prompt, schema, once) {
  return agentR(prompt, { label, phase: phaseTitle, agentType: ROLES[role].agentType, schema }, null, null, once)
}

// halt — every way a stage stops short, as one shape. Committed work stands; the next
// invocation is a fresh one. A halt of a run with a queue carries what every return of such a
// run carries (`queueSaid`) — AS THE STATE WAS LAST READ, which after a wave whose check or
// whose read did not come back is before that wave.
function halt(phaseName, why, more) {
  const transient = !!(more && more.transient)
  return Object.assign({
    status: 'halted',
    stage: v.stage,
    run: v.run,
    halted: Object.assign({ phase: phaseName, reason: why, transient }, more || {}),
    arrived,
    named: namedOf(lastState),
    message: 'stabilize ' + v.stage + ' of `' + v.run + '` HALTED at ' + phaseName + ': ' + why + (breakerTripped ? ' The rate-limit breaker is tripped (' + exhaustedLabels.join(', ') + '): wait for the window before invoking again.' : transient ? ' An agent returned no result after retries: that is infrastructure, not a verdict about the run.' : '') + ((more && more.then) || ' What was committed stands; a report a reporter wrote and no record step committed is still untracked under ' + runDir(v.run) + '/ and stays there. Once the cause is dealt with, invoke the stage again with the same args: it reads the state as it stands and works the next attempt.'),
  }, queueSaid(lastState))
}
// haltAfter — a halt of a `test` stage ONCE AN INSTRUMENT IS LAUNCHED: it names its row of
// HALTS_AFTER, and that row's phase is the phase the halt carries.
function haltAfter(row, why, more) {
  return halt(HALTS_AFTER[row].phase, why, more)
}
// gitHalt — a git step that did not do what the stage needs: it returned nothing, it
// halted, or it reported something else than the step's end state.
// A REFUSAL OF THE TOOL IS ITS WORD: the digest carries that and no prose, this script says
// what the word means from its own table (STEP_REFUSALS), and the tool's halt report — the
// file, the commit, the command that leaves the state — is in the file the digest names,
// which the halt returns by its path and its hash (`step`). An agent's own halt report, of
// a step whose command printed no line, is passed on as it came (`halt`).
function gitHalt(phaseName, r, what, more) {
  if (!r) return halt(phaseName, what + ': the step returned no result', Object.assign({ transient: true }, more || {}))
  const word = typeof r.refused === 'string' ? r.refused : null
  const said = word ? 'the step refused ' + stepRefusal(word, (more && more.branch) || r.branch || loopBranch) : r.relay || (r.halt && r.halt.root_cause) || 'the step reported ' + JSON.stringify({ status: r.status, branch: r.branch, head: r.head, remote_head: r.remote_head, merge_commit: r.merge_commit })
  return halt(phaseName, what + ': ' + said, Object.assign({ refused: word, whose: word && STEP_REFUSALS[word] ? STEP_REFUSALS[word].whose : null, step: stepFile(r), halt: word || r.relay ? null : r.halt || null }, more || {}))
}
// stepFile — the file a step's digest names, by its path under the invocation's scratch root.
function stepFile(r) {
  const kept = fileOf(r)
  return kept ? { file: v.scratch + '/' + kept.file, sha256: kept.sha256 } : null
}
// A branch step ended where the stage needs it: on `branch`, and — for a push — with the
// remote at the local head.
function onIt(r, branch, pushed) {
  return !!r && r.status === 'ready' && r.branch === branch && SHA_RE.test(String(r.head || '')) && (!pushed || r.remote_head === r.head)
}

// readState — the state, as its digest: what this script acts on of the document, relayed
// by a git step as the ONE line its command prints and held to that line's hash and to
// being a digest. The document itself stays in the file the digest names, and every return
// names that file (`namedOf`). A relay that does not hash, is no digest, or holds no state
// is asked for again, twice; then there is no state, and the stage halts.
let stateReads = 0
// The state as it was last read, for what every return names of it; and what the first read
// of the invocation met on arrival — `found`, `finished`, `owed`, as the tool names them.
let lastState = null
let arrived = null
async function readState() {
  const tag = v.stage + '-' + (++stateReads)
  let why = 'the step returned no result'
  for (let n = 0; n < 3; n++) {
    const again = n === 0 ? '' : '\n\n' + relayAgain(n)
    const r = await toolStep('state:' + tag + (n ? ':again' + n : ''), 'State', 'state', statePrompt(v, tag) + again)
    if (!r) return { error: why, transient: true }
    if (!r.relay) {
      if (r.status !== 'read') return { error: r.refused ? 'the step refused ' + stepRefusal(r.refused) : (r.halt && r.halt.root_cause) || 'the step halted', halt: r.refused ? null : r.halt, step: stepFile(r) }
      if (plain(r.state) && typeof r.state.opened === 'boolean') {
        // Where the document lies: the one thing of the state this script adds to it.
        lastState = Object.assign(r.state, { document: stepFile(r) })
        return { state: lastState, branch: r.branch }
      }
    }
    why = r.relay || 'the relayed line holds no state'
    log('state relay rejected (' + why + ') — asking again')
  }
  return { error: why }
}

// settleReports — the stage's reports, held to every reporter launched so far, by the one
// check that can see a file. A launched reporter with no report is TAKEN OFF THE LIST and
// named in `missing`: the record is not held to a report that does not exist, and the
// caller says what follows for that reporter. A file nobody launched is a fault.
async function settleReports(ctx, launch, label, phaseTitle) {
  const read = reportsRead(await toolStep(label, phaseTitle, 'check-reports', checkReportsPrompt(ctx, launch.names)))
  for (const name of read.missing || []) launch.drop(name)
  return read
}
// asideSaid — WHAT A REPORT CHECK SET ASIDE, AS LINES OF THE LOG: each file that left the tree
// because the record script would not have written it there, by its path and the word of
// what refused it — a file a hand, an editor or a file browser left among a round's filed
// verdicts or beside a stage's reports, where the check comes before a record and no
// reporter of the invocation is there for the caller to say it of.
function asideSaid(aside) {
  for (const moved of aside) {
    if (!plain(moved) || typeof moved.path !== 'string') continue
    log('SET ASIDE by the report check: `' + moved.path + '` (`' + String(moved.why) + '`) — the file is not what `' + RECORD_TOOL + '` would have written at its path, and it left the tree for the scratch root: the step\'s own line says where it lies now')
  }
}

// recordFault — why a record is NOT accepted as recorded, or null. What is accepted is the
// commit step's own line — held to its hash by `readStep` — and nothing an agent says of
// it: the commit, the gate check that holds, and ONE RESULT PER CHECK THIS SCRIPT COMPOSED,
// each of which holds, in a batch of as many calls as were composed. A step that returns no
// such evidence did not record.
function recordFault(r, expect) {
  if (!r) return 'the record\'s commit step returned no result'
  if (r.status !== 'recorded') return r.refused ? 'the record\'s commit step refused ' + stepRefusal(r.refused, r.branch) : r.relay || (r.halt && r.halt.root_cause) || 'the record\'s commit step halted'
  if (!SHA_RE.test(String(r.commit || ''))) return 'the record\'s commit step returned no commit sha'
  if (!plain(r.gate) || r.gate.ok !== true) return 'the record\'s commit step returned no gate check that holds'
  const applied = r.applied
  if (!plain(applied) || applied.calls !== expect.calls || applied.checks !== expect.checks) return 'the batch that was committed is not the one this stage composed: ' + expect.calls + ' call(s) and ' + expect.checks + ' check(s) were composed, and the commit step read ' + JSON.stringify(applied == null ? null : { calls: applied.calls, checks: applied.checks })
  if (applied.failed !== 0) return 'a check of the record does not hold: ' + JSON.stringify(applied.failed == null ? null : applied.failed) + ' of ' + applied.checks + ' — the step\'s own line names each'
  return null
}
// pushHalt — a record's push that did not end with the remote at the local head.
function pushHalt(pushed, branch, what) {
  return gitHalt('push', pushed, what, { branch, then: pushThen(branch, pushed && typeof pushed.refused === 'string' ? pushed.refused : null) })
}
// The commit the stage began on, as its first read returned it — and, after a record of
// this invocation, the head its push returned: what a record's commit step is held to
// (`--head`). Null in the `fix` stage's own records, which follow its fixers' commits: which
// head those are held to is that half's repair.
let stageHead = null
// THE GUARDED STATE the stage began with, as its first read printed it, and after a record of
// this invocation as that record's commit step printed it: what the next record's commit
// step is held to (`--fence`; dev/stabilize-step: THE GUARDED STATE). Null where a read
// printed none, and in the `fix` stage's own records, as `stageHead` is.
let stageFence = null
// recordStep — a record step, whole, in three acts: (1) the executor applies the batch — or
// none runs, where an earlier invocation applied it (`rec.text` is null); (2) THE FULL GATE
// on the tree with the records, a command the tool holds (`hold`, under a name of its own);
// (3) the ONE commit, a git step, held to that gate's output as the tool kept it. `rec` is
// what `recordPrompt` or `pendingRecord` returned. Returns the commit step's digest as
// `result` — or `fault`, with `then`: what the next invocation does, and `unchanged` where
// the batch was refused as one that changes nothing.
async function recordStep(label, branch, rec) {
  if (rec.text) {
    const applied = await roleStep('record', 'record:' + label, 'Record', rec.text, RECORD_SCHEMA)
    if (!applied) return { fault: 'the record step returned no result', result: null, transient: true, then: recordThen(v.run, branch, null) }
    if (applied.status !== 'applied') return { fault: (applied.halt && applied.halt.root_cause) || 'the record step halted', result: null, halt: applied.halt || null, unchanged: applied.refused === NO_CHANGE, then: applied.refused === NO_CHANGE ? ' Nothing was written and NO BATCH IS LEFT: the record script refused the batch as one that changes nothing (`' + NO_CHANGE + '`) — every file it would write is what the run holds already. No discard is asked for.' : recordThen(v.run, branch, null) }
  }
  // A RECORD'S GATE WITH NO VERDICT — void, dead, or nobody left to ask after it: nothing
  // is committed, THE BATCH STAYS APPLIED, and the next invocation gates it again, under
  // the next name — the name of a pending batch's gate (`finishRecord`). WHERE THIS IS THAT
  // GATE AND NO NAME IS LEFT OF IT (`exhausted`), there is no next name, and the halt says so
  // and names the exit that works (`recordThen`: `no-name`).
  // (`at`, `silent`: which step of the record it was, and whether that step RETURNED NOTHING
  // — what a halt that may be the account's limit is said of, `limitSaid`.)
  const gate = await hold({ name: rec.hold, sequence: true, kind: 'gate', flags: '--run ' + v.run, what: 'the full gate on the tree with the records', phase: 'Record' })
  if (gate.exhausted) return { fault: heldFault('the record\'s gate', gate) + ' Nothing was committed.', result: null, transient: false, at: 'gate', silent: false, held: heldSaid(gate), then: recordThen(v.run, branch, rec.pending ? 'no-name' : 'no-gate') }
  if (!HELD_TAKES.record.includes(gate.ends)) return { fault: heldFault('the record\'s gate', gate) + ' Nothing was committed.', result: null, transient: gate.ends === 'unread' || gate.ends === 'running', at: 'gate', silent: gate.silent === true, held: heldSaid(gate), then: recordThen(v.run, branch, 'no-gate') + (heldThen(v.scratch, gate) || '') }
  const r = await toolStep('record:' + label, 'Record', 'record', recordCommitPrompt(v, branch, gate.output, rec.expect, stageHead, stageFence))
  const found = recordFault(r, rec.expect)
  if (!found) {
    if (stageFence) stageFence = SHA256_RE.test(String(r.fence)) ? String(r.fence) : null
    return { result: r }
  }
  const fault = found.replace('<branch>', branch)
  const word = r && typeof r.refused === 'string' ? r.refused : null
  return { fault, result: r, transient: !r, at: 'commit', silent: !r, refused: word, step: stepFile(r), gate: r && plain(r.gate) ? r.gate : null, then: recordThen(v.run, branch, word, r ? r.discarded : null) }
}
// composeRecord — THE FIRST ACT OF A RECORD THE TOOL COMPOSES, IN A RUN WITH A QUEUE: ONE
// step of the tool (`compose`), handed the plain words and the sha256 of their canonical
// form, in place of the executor that typed a batch into a file. It returns `rec` — what
// `recordStep` is then handed: NO TEXT, so no executor runs, the name the record's gate is
// held under, and what the commit step is held to, WHICH IS WHAT THE COMPOSING STEP SAID IT
// APPLIED — or `fault`, with `then`. A STEP REFUSED `words` IS ASKED FOR AGAIN, exactly
// (COMPOSE_REFUSALS, WORDS_AGAIN): a word altered on its way is its agent's slip, and the
// same words compose the same batch. Every other refusal is returned, with the record
// script's own word (`refusal`) beside the tool's (`refused`); nothing was written by a step
// the record script refused. A step that answered nothing, or with a line that is not read,
// MAY HAVE APPLIED ITS BATCH: the next invocation's first read says so, and finishes it. It
// halts nothing itself.
async function composeRecord(label, branch, ctx, dir, subject, words) {
  let c = null
  for (let n = 0; n <= WORDS_AGAIN; n++) {
    c = await toolStep('compose:' + label + (n ? ':again' + n : ''), 'Record', 'compose', composePrompt(ctx, branch, subject, words) + (n ? '\n\n' + wordsAgain(n) : ''))
    const row = c && c.status === 'halted' && c.refused === 'record' ? COMPOSE_REFUSALS[c.refusal] : null
    if (!row || !row.again) break
    log('the composing step was refused `' + c.refusal + '` — ' + row.says + (n < WORDS_AGAIN ? ': the step is asked for again, exactly' : ': it was asked for again already'))
  }
  if (!c) return { fault: 'the step that composes the record returned no result', transient: true, then: recordThen(v.run, branch, null) }
  if (c.status === 'applied' && plain(c.applied) && Number.isInteger(c.applied.calls) && Number.isInteger(c.applied.checks)) return { rec: { text: null, hold: recordHold(dir), expect: { calls: c.applied.calls, checks: c.applied.checks } } }
  const word = typeof c.refused === 'string' ? c.refused : null
  const said = word === 'record' && typeof c.refusal === 'string' ? c.refusal : null
  const row = said ? COMPOSE_REFUSALS[said] || { says: 'a word of `' + RECORD_TOOL + '` this script has no row for: the step\'s own line holds that script\'s refusal', then: null } : null
  if (row) return { fault: 'the record was not composed: `' + RECORD_TOOL + '` refused the composing step `' + said + '` — ' + row.says + scanHits(c.hits) + '. Nothing was written', refused: word, refusal: said, step: stepFile(c), then: row.then === 'pending' ? recordThen(v.run, branch, null) : row.then }
  if (word) return { fault: 'the step that composes the record refused ' + stepRefusal(word, branch), refused: word, refusal: null, step: stepFile(c), then: recordThen(v.run, branch, null) }
  return { fault: 'the step that composes the record did not answer with a batch it applied: ' + (c.relay || (c.halt && c.halt.root_cause) || 'it said `' + c.status + '`'), refused: null, refusal: null, step: stepFile(c), halt: c.relay ? null : c.halt || null, then: recordThen(v.run, branch, null) }
}
// recordHalt — a record step that did not record, as the halt of the stage: why, what the
// tool established (`gate`: the counts of what is red; `step`: the file its own line is in),
// the executor's own halt report where it halted, and what the next invocation does.
function recordHalt(phaseName, recorded, more) {
  return halt(phaseName, recorded.fault, Object.assign({ transient: !!recorded.transient, refused: recorded.refused || null, whose: recorded.refused && STEP_REFUSALS[recorded.refused] ? STEP_REFUSALS[recorded.refused].whose : null, step: recorded.step || null, gate: recorded.gate || null, held: recorded.held || null, halt: recorded.halt || null, then: recorded.then }, more || {}))
}
// pushRecord — the push of a record this invocation committed on `branch`, and then the
// head the next record of the invocation is held to. A record on the loop branch is pushed
// as a record's (`--run-dir`).
async function pushRecord(label, branch) {
  const pushed = await toolStep(label, 'Record', 'push', pushPrompt(v, branch, branch === loopBranch))
  if (onIt(pushed, branch, true) && stageHead) stageHead = String(pushed.head)
  return pushed
}
// finishRecord — an invocation that finds a batch a record step applied and did not commit:
// the gate on it again, the one commit, the push, the state — and NOTHING ELSE of a stage.
async function finishRecord(gs) {
  phase('Record')
  const pending = gs.pending
  log('a record step left its batch applied and not committed on ' + gs.branch + ' — ' + pending.calls + ' call(s), ' + pending.files + ' file(s): this invocation gates and commits it, and starts nothing')
  const recorded = await recordStep('pending', gs.branch, pendingRecord(v.scratch + '/record/pending', pending))
  if (recorded.fault) return recordHalt('record', recorded, { pending })
  const pushed = await pushRecord('push:pending', gs.branch)
  if (!onIt(pushed, gs.branch, true)) return pushHalt(pushed, gs.branch, 'the pending record is committed on ' + gs.branch + ' (' + recorded.result.commit + ') and the branch was not pushed')
  const read = await readState()
  if (!read.state) return halt('state', 'the pending record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient, step: read.step || null })
  return Object.assign({ status: 'recorded', stage: v.stage, run: v.run, pending, record: recorded.result.commit, gate: recorded.result.gate, arrived, message: 'a record step of an earlier invocation had applied its batch and not committed it: this invocation ran the gate on it, committed and pushed it, and did nothing else — `next` names the step.' }, nextOf(read.state))
}

// beginAttempt — AN ATTEMPT BEGINS ON RECORD: before any agent of it is launched, one step
// holds its round and its number to the state's position once more and writes the
// attempt's marker, under a reporter of this script's own. Returns the halt, or null.
async function beginAttempt(ctx, launch, sha) {
  launch.add([ATTEMPT])
  const begun = await toolStep('begin', 'State', 'begin', beginPrompt(ctx, sha))
  if (!begun || begun.status !== 'begun' || String(begun.attempt) !== String(ctx.attempt)) return gitHalt('begin', begun, 'attempt ' + ctx.attempt + ' of the `' + ctx.stage + '` stage, round ' + ctx.round + ', was not put on record, and no agent of it was launched')
  return null
}

// findingLines — a reporter's structured findings as the lines triage is handed.
function findingLines(findings) {
  return findings.map((f) => f.id + ' — ' + f.title + ' · door: ' + f.door + ' · clause: ' + f.clause + (f.severity ? ' · severity: ' + f.severity : '') + (f.lead ? ' · a lead' : '') + ' · repro: ' + f.repro)
}
function leftOpenSource(reporter, report, leftOpen) {
  return { reporter, report, findings: (leftOpen || []).map((text, i) => 'left open ' + (i + 1) + ' — ' + text) }
}

// runUnits — the chains of a list of units, in parallel under the runtime's cap. Returns,
// per unit, its status (`green` once every step of its chain reported; `void`, with the
// step and how it ended, when one did not) and every reporter it launched with what that
// reporter returned. Whether a reporter's REPORT is there is not known here: the caller
// has it checked (`settleReports`), and a result with no report counts for nothing.
async function runUnits(ctx, launch, built, units, phaseTitle) {
  const out = await parallel(units.map((unit) => async () => {
    const done = []
    let reason = null
    let word = null
    let crossModel = null
    for (const stepList of unit.chain) {
      const named = stepList.map((step) => ({ step, name: launch.add([unit.item, step.as]) }))
      const promptOf = (step, name) => (step.crossModel ? crossModelPrompt(ctx, name, unit) : unitPrompt(ctx, launch, name, step, unit, built, (step.hands || []).map((h) => ({ as: h, report: (done.find((d) => d.as === h) || {}).report }))))
      const results = await parallel(named.map(({ step, name }) => () => roleStep(step.role, unit.item + ':' + step.as, phaseTitle, promptOf(step, name), UNIT_SCHEMA)))
      named.forEach(({ step, name }, i) => {
        const r = results[i]
        const reported = !!r && r.status === 'reported'
        // A STEP THAT REPORTED AND DID NOT ASSERT THE CANDIDATE'S BINARY — no hash, or
        // another — voids its item like a step that did not report (NEEDS): what it drove
        // is evidence about nothing. What it FOUND stays with it, and is triaged.
        const unheld = reported ? unasserted(step.role, name, r, built.candidate.sha256) : null
        // The cross-model pass that did not run is void FOR THAT PASS, however it ended:
        // the unit goes on with its own passes, and nothing of the stage waits for it.
        if (step.crossModel) crossModel = reported ? 'ran' : 'void'
        else if (!reported && !reason) [reason, word] = ['its `' + step.as + '` step ' + endingOf(r), voidWord(r ? 'halted' : 'silent', name)]
        else if (unheld && !reason) [reason, word] = ['its `' + step.as + '` step ' + unheld, voidWord('unasserted', name)]
        done.push({ as: step.as, name, drives: ROLES[step.role].drives, crossModel: !!step.crossModel, result: reported || !step.crossModel ? r : null, report: r && r.report ? r.report : null })
      })
      if (reason) break
    }
    return { item: unit.item, clause: unit.clause, status: reason ? 'void' : 'green', reason, word, crossModel, reporters: done }
  }))
  return units.map((unit, i) => out[i] || { item: unit.item, clause: unit.clause, status: 'void', reason: 'its chain did not run', word: voidWord('chain-lost'), crossModel: null, reporters: [] })
}
// triagePasses — triage of every finding, verify-real on everything graded breaks or
// unclear, and per contested finding an advocate and an independent drive; then again over
// what those agents left open. Returns the entries by key, with the verdicts beside them.
// AFTER EVERY PASS THE REPORTS ARE CHECKED, and a verdict, a case or a drive counts only
// with its report: the next pass's triage is handed the reporters that have one.
async function triagePasses(ctx, launch, built, sources, forksIn) {
  const entries = []
  const forks = forksIn.slice()
  const unverified = []
  const unreportedBy = []
  let pending = sources.filter((s) => findingsOf(s))
  let handed = 0
  for (let pass = 1; pending.length; pass++) {
    const count = pending.reduce((n, s) => n + findingsOf(s), 0)
    handed += count
    const tName = launch.add(['triage', 'p' + pass])
    log('triage pass ' + pass + ': ' + count + ' finding(s) from ' + pending.length + ' reporter(s)')
    const t = await roleStep('triage', 'triage:p' + pass, 'Triage', triagePrompt(ctx, launch, tName, pending, pass), TRIAGE_SCHEMA)
    if (!t || t.status !== 'graded') return { fault: t ? 'triage halted: ' + ((t.halt && t.halt.root_cause) || 'no reason given') : 'triage returned no result', transient: !t, halt: t ? t.halt : null }
    const counted = (t.counts.findings_in || []).reduce((n, c) => n + c.count, 0)
    if (counted !== count || t.entries.length + t.counts.merged !== count) return { fault: 'triage pass ' + pass + ' was handed ' + count + ' finding(s) and accounts for ' + counted + ' in, ' + t.entries.length + ' entries and ' + t.counts.merged + ' merged: a finding in no entry and in no named merge is lost, and nothing is recorded on that' }
    const strangers = t.entries.filter((e) => !isSlug(e.key)).map((e) => JSON.stringify(e.key))
    if (strangers.length) return { fault: 'triage returned the key ' + strangers.join(', ') + ', which is not a ledger key' }
    // One finding, one key, one entry: two entries under one key are two findings merged
    // without a word, or one finding graded twice — and nothing says which.
    const twice = t.entries.map((e) => e.key).filter((key, i, all) => all.indexOf(key) !== i)
    if (twice.length) return { fault: 'triage pass ' + pass + ' returned more than one entry under the key ' + twice.filter((key, i) => twice.indexOf(key) === i).join(', ') + ': one finding has one entry, and a merge is named — nothing is recorded on that' }
    for (const e of t.entries) {
      const held = entries.find((x) => x.key === e.key)
      if (held) Object.assign(held, e, { new: held.new })
      else entries.push(Object.assign({}, e))
    }
    pending = []
    const verified = []
    const driven = []
    const toVerify = pass > VERIFY_PASSES ? [] : t.entries.filter((e) => e.grade === 'breaks' || e.grade === 'unclear')
    if (pass > VERIFY_PASSES) log('NOT VERIFIED: pass ' + pass + ' is past the ' + VERIFY_PASSES + ' passes that are verified — its entries get their rows, and those graded breaks or unclear stay unverified: the state\'s `next` then says that the round\'s triage is not finished (`triage`), and names them')
    if (toVerify.length) {
      const hints = {}
      for (const h of t.to_verify || []) hints[h.key] = h.redrive
      const unnamed = toVerify.filter((e) => !forkNames('p' + pass, e.key)).map((e) => e.key)
      if (unnamed.length) return { fault: 'no reporter name can be made for the verifier of ' + unnamed.join(', ') + ': the key is too long to carry a prefix inside ' + SLUG_MAX + ' characters' }
      const named = toVerify.map((e) => ({ e, name: launch.add(['verify', 'p' + pass, e.key]) }))
      log('verify-real, pass ' + pass + ': ' + named.length + ' finding(s)')
      const verdicts = await parallel(named.map(({ e, name }) => () => roleStep('verify', 'verify:' + e.key, 'Triage', verifyPrompt(ctx, launch, name, e, hints[e.key], built), VERIFY_SCHEMA)))
      named.forEach(({ e, name }, i) => {
        const r = verdicts[i]
        const noVerdict = r && r.status === 'verified' ? lacks('verify', r).find((n) => n.field === 'verdict') : null
        if (!r || r.status !== 'verified' || noVerdict || r.key !== e.key) {
          unverified.push({ key: e.key, why: 'its verifier ' + (!r || r.status !== 'verified' ? endingOf(r) : noVerdict ? lacksWhy(name, noVerdict) : 'returned a verdict for `' + r.key + '`, another finding') })
          return
        }
        // A VERDICT ON A BINARY NOBODY ASSERTED — no hash of the candidate, or another — is
        // no verdict about this candidate (NEEDS): the finding stays unverified, with the
        // reporter and the field in the reason, and nothing of the stage waits on it. Its
        // report, and what it left open, are taken below as any verifier's are.
        const unheld = unasserted('verify', name, r, built.candidate.sha256)
        verified.push({ e, name, r, unheld })
      })
      for (const { e, r } of verified.filter((v) => !v.unheld && v.r.contested)) driven.push(await driveFork(ctx, launch, built, { key: e.key, kind: 'contested', door: e.door, clause: e.clause, repro: e.repro, statement: 'the verifier found the finding to contest a settled decision — ' + (r.basis || '(no basis returned)') }, 'p' + pass))
    }
    // THE REPORTS OF THIS PASS, CHECKED — before anything it established is taken.
    const settled = await settleReports(ctx, launch, 'check-reports:p' + pass, 'Triage')
    if (settled.fault) return { fault: settled.fault, transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null }
    for (const name of settled.missing) unreportedBy.push(name)
    if (settled.missing.includes(tName)) return { fault: 'triage pass ' + pass + ' returned its grades and left no report: nothing is recorded on a grader\'s word with no report behind it' }
    for (const { e, name, r, unheld } of verified) {
      const fork = driven.find((d) => d.fork.key === e.key)
      // A fork is whole or it is none: the advocate's case, the independent drive, both
      // reports, and each on the binary it asserted. Where one is missing the finding
      // stays unverified, and the next triage verifies it and drives the fork again.
      const lacking = settled.missing.includes(name) ? 'its verifier returned a verdict and left no report'
        : unheld ? 'its verifier ' + unheld
          : !fork ? null
            : !fork.argued || settled.missing.includes(fork.aName) ? 'it is contested, and its advocate ' + (fork.argued ? 'left no report' : endingOf(fork.fork.advocate))
              : fork.unheld ? 'it is contested, and ' + fork.unheld
                : !fork.drove || settled.missing.includes(fork.pName) ? 'it is contested, and the independent drive of the advocate\'s proposal ' + (fork.drove ? 'left no report' : endingOf(fork.fork.independent_drive))
                  : null
      // What an agent left open is triaged whatever became of the verdict beside it.
      if (!settled.missing.includes(name) && r.left_open && r.left_open.length) pending.push(leftOpenSource(name, r.report, r.left_open))
      for (const source of fork ? fork.leftOpen.filter((x) => !settled.missing.includes(x.reporter)) : []) pending.push(source)
      if (lacking) {
        unverified.push({ key: e.key, why: lacking })
        continue
      }
      const entry = entries.find((x) => x.key === e.key)
      entry.verdict = r.verdict
      // The previous release's binary is driven with every confirmed verdict — that is
      // where the regression fact comes from, true or false — so the fact is taken only
      // with that binary's hash behind it. WITHOUT IT THE VERDICT STANDS AND THE FACT IS
      // UNKNOWN (NEEDS: `unknown`): the finding is confirmed, NOT unverified, and nothing
      // of the stage waits on it.
      if (r.verdict === 'confirmed') {
        const fact = regressionOf(name, r, built.previous.sha256)
        entry.regression = fact.regression
        if (fact.unknown) log('REGRESSION UNKNOWN: `' + e.key + '` is confirmed, and its verifier ' + fact.unknown + '. It is recorded as confirmed with that fact unknown — never as not a regression — and the state names it')
      }
      entry.basis = r.basis
      if (fork) {
        entry.fork = { kind: fork.fork.kind, case: fork.fork.advocate.verdict, drive: fork.fork.independent_drive.holds ? 'holds' : 'differs' }
        forks.push(fork.fork)
      }
    }
    if (pass > VERIFY_PASSES) break
  }
  for (const left of unverified) log('NOT VERIFIED: `' + left.key + '` — ' + left.why + '. It stays unverified: the state counts the pass, and after one more the finding is the human\'s')
  return { entries, forks, handed, unverified, unreportedBy }
}

// triageOnce — THE STAGE'S ONE TRIAGE, IN A RUN WITH A QUEUE (the header: A RUN WITH A QUEUE
// TAKES ANOTHER PATH). `sources` is every source of the stage; the ones that hand a finding
// are handed on. No source hands a finding: no triage is launched, and there is nothing to
// hand over. Else the triage is launched under the reporter `triage`, its return is held
// (`filedFault`) BEFORE the reports are checked, and
// THE REPORT CHECK SAYS WHETHER ITS RENDERED ENTRIES ARE THERE — never the return: a
// rendering that is no rendering was set aside, and its triage has left none. It returns
// `triage` — the reporter, the path and the hash — and how many findings were handed; or
// `fault`, which the caller halts on. It halts nothing itself, and reads no entry.
async function triageOnce(ctx, launch, sources) {
  const handed = sources.filter((s) => findingsOf(s))
  const count = handed.reduce((n, s) => n + findingsOf(s), 0)
  if (!handed.length) return { triage: null, handed: 0 }
  const tName = launch.add(['triage'])
  log('the stage\'s one triage: ' + count + ' finding(s) from ' + handed.length + ' source(s) — ' + handed.map((s) => s.reporter + '=' + findingsOf(s)).join(' '))
  const t = await roleStep('triage', 'triage', 'Triage', oneTriagePrompt(ctx, launch, tName, handed), FILED_SCHEMA)
  if (!t || t.status !== 'filed') return { fault: t ? 'triage halted: ' + ((t.halt && t.halt.root_cause) || 'no reason given') : 'triage returned no result', transient: !t, halt: t ? t.halt : null }
  const unheld = filedFault(ctx, tName, handed, t)
  if (unheld) return { fault: unheld }
  const settled = await settleReports(ctx, launch, 'check-reports:triage', 'Triage')
  if (settled.fault) return { fault: settled.fault, transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null }
  if (settled.missing.includes(tName)) return { fault: 'triage returned the path and the hash of its rendered entries, and the report check finds no report of it there' + (setAside(settled.aside, tName) ? ' — the file at that path is not the text its entries render (`' + setAside(settled.aside, tName) + '`), and was set aside' : '') + ': nothing is recorded on a return with no file behind it' }
  return { triage: { reporter: tName, report: t.report, sha256: t.sha256 }, handed: count }
}

// verifyWaves — THE VERIFY PHASE OF A `test` STAGE, IN A RUN WITH A QUEUE (the header: THEN THE
// STAGE VERIFIES, IN WAVES). `from` is the state as it was read back after the stage's first
// record. PER WAVE: the keys the queue prints (`queue.first`), in its rank, that this
// invocation has not taken already — AT MOST `WAVE`, AND NEVER MORE THAN THE BUDGET HAS LEFT
// (`queue.left`; a run that holds no budget bounds nothing) — a verifier for each, in
// parallel, handed its key (`verdictPrompt`); THE REPORTS CHECKED; and THE QUEUE READ AGAIN,
// which says what the wave left. It ends where the queue prints no key that is still to be
// launched, or the budget has nothing left. A KEY IS LAUNCHED ONCE AN INVOCATION, whatever
// its verifier left: its verdict is filed under the reporter `verify-<key>`, and a reporter
// reports once an attempt — so a finding a launch left without a verdict is the next
// invocation's, and the record script counts the launch by its report. A key no verifier can
// be named for is passed over, and named.
// WHAT A WAVE LEFT IS READ OFF THE QUEUE, AND NEVER OFF A RETURN: a finding the queue prints
// again has no verdict that counts (`unverified`, with how its verifier ended — a return that
// names a filing among them); a return that is not a filing's (`verdictFault`), a halt or no
// return at all, of a finding the queue does NOT print again, is named (`unheld`) and decides
// nothing — the state says where the finding stands. `contested` is the keys whose verifier's
// held return says so: WHETHER THE FILING GAVE A BASIS IS THE FILED FILE'S, which this script
// cannot read — the state's list of what is the human's names a contested finding with its
// basis, and a key named here that it does not list filed none and is routed by its verdict.
// EACH OF THE THREE IS A LINE OF THE LOG, by its key and with why; what goes back is HOW MANY.
// A VERIFIER CALL IS NEVER TRIED TWICE, AND A WAVE IN WHICH `SILENT_LAST` OF THEM RETURNED
// NOTHING IS THE LAST (the reshape's plan, T6b; its §2.2 and E7). The account's usage limit
// reaches this script as nothing at all, so one silent call is its finding's alone — named,
// unverified, and the phase goes on — and two in one wave end the phase: that wave's reports
// are still checked and the queue still read, and no further wave is launched (`last`: the
// wave, and how many of its calls were silent; `silent`: how many in the whole phase). WHAT
// THIS INVOCATION'S VERIFIERS FILED is kept as the one path a filing of each has
// (`filings`) — a return that is held, of a finding the queue does not print again — for the
// halt that names them where the steps after such a wave return nothing too; `unconfirmed`
// is the same of a wave whose check or read did not come back, held by the returns alone.
// It returns the state as it was last read, and `fault` where the report check (`unread`
// absent) or the read of the queue (`unread`) could not be made. It halts nothing itself.
async function verifyWaves(ctx, launch, built, from) {
  const out = { state: from, waves: 0, launched: 0, unverified: 0, unheld: 0, contested: 0, silent: 0, last: null, filings: [], unconfirmed: [] }
  const seen = []
  const unverified = (key, why) => {
    out.unverified++
    log('NOT VERIFIED: `' + key + '` — ' + why + '. It awaits its verdict again, and nothing of the budget was spent on it')
  }
  for (;;) {
    const queue = out.state.queue
    const room = queue.left == null ? WAVE : Math.min(WAVE, queue.left)
    const named = []
    for (const key of queue.first || []) {
      if (named.length >= room) break
      if (seen.includes(key)) continue
      seen.push(key)
      const name = reporterName(['verify', key])
      if (name && !launch.names.includes(name)) named.push({ key, name: launch.add(['verify', key]) })
      else unverified(key, 'no verifier can be named for it: a verdict is filed under the reporter `verify-<key>`, which for this key is no slug of at most ' + SLUG_MAX + ' characters, or is a name this attempt has launched already')
    }
    if (!named.length) return out
    out.waves++
    out.launched += named.length
    log('verify-real, wave ' + out.waves + ': ' + named.length + ' of the ' + queue.length + ' finding(s) the queue holds — ' + (queue.left == null ? 'the run holds no budget' : queue.left + ' verdict(s) of the budget left'))
    const back = await parallel(named.map(({ key, name }) => () => roleStep('verify', 'verify:' + key, 'Verify', verdictPrompt(ctx, launch, name, key, built, queue.hand_on), VERDICT_SCHEMA, true)))
    // HOW MANY CALLS OF THE WAVE RETURNED NOTHING — each was tried once: with SILENT_LAST of
    // them the wave is the last, whatever its check and the queue then say.
    const silent = back.filter((r) => r == null).length
    out.silent += silent
    if (silent >= SILENT_LAST) out.last = { wave: out.waves, silent }
    const returned = named.filter(({ name }, i) => back[i] && back[i].status === 'filed' && !verdictFault(ctx, name, back[i], built.candidate.sha256)).map(({ name }) => verdictFile(ctx, name))
    // THE REPORTS OF THIS WAVE, CHECKED — and then THE QUEUE, which says what the wave left.
    const settled = await settleReports(ctx, launch, 'check-reports:wave' + out.waves, 'Verify')
    if (settled.fault) return Object.assign(out, { fault: settled.fault, transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null, unconfirmed: returned })
    const read = await readState()
    if (!read.state) return Object.assign(out, { unread: true, fault: read.error, transient: !!read.transient, step: read.step || null, unconfirmed: returned })
    out.state = read.state
    named.forEach(({ key, name }, i) => {
      const r = back[i]
      const filed = !!r && r.status === 'filed'
      const moved = setAside(settled.aside, name)
      const ended = !filed ? endingOf(r)
        : settled.missing.includes(name) ? (moved ? 'left a file at its report\'s path that is no report (`' + moved + '`), which was set aside' : 'returned and left no report')
          : verdictFault(ctx, name, r, built.candidate.sha256)
      if ((out.state.queue.first || []).includes(key)) unverified(key, 'its verifier ' + (ended || 'returned the path and the word of a filing') + ', and the queue prints the finding again: no verdict of it counts')
      else if (ended) {
        out.unheld++
        log('NOT HELD: `' + key + '` — its verifier ' + ended + ' — and the queue does not print the finding again: the state says where it stands, and no return decides it')
      } else {
        out.filings.push(verdictFile(ctx, name))
        if (r.contested) {
          out.contested++
          log('CONTESTED, BY ITS VERIFIER\'S RETURN: `' + key + '` — where its filing gave a basis the finding is the human\'s, and the state\'s list of what is the human\'s names it with that basis; where that list does not name it so, the filing gave none, and the finding is routed by its verdict. This script reads no filing, and cannot tell the two apart')
        }
      }
    })
    if (out.last) {
      log('THE LAST WAVE: ' + silent + ' verifier call(s) of wave ' + out.waves + ' returned nothing — each was tried once, and none is tried again. That is how the account\'s usage limit shows, which never reaches this script as a message, and it is also what ' + silent + ' stray deaths look like: no further wave is launched, though the queue holds ' + out.state.queue.length + ' finding(s) and ' + (out.state.queue.left == null ? 'the run holds no budget' : out.state.queue.left + ' verdict(s) of the budget are left') + '. What was filed is recorded now, and the invocation that follows continues')
      return out
    }
  }
}

// verifyThenRecord — THE VERIFY PHASE AND THE RECORD THAT FOLLOWS IT, IN A RUN WITH A QUEUE:
// what a stage does after its first record, and what an invocation that continues one does
// after its reads (the header: THEN THE STAGE VERIFIES, IN WAVES; AN INVOCATION THAT
// CONTINUES). `from` is the state as it was last read. `made` says what THIS INVOCATION has
// recorded before the phase: `commit` — a stage's first record, or a continuation's record
// of the rows it graded — or null, where it has made none: a continuation that had nothing
// to grade; `owed`, for that one: whether the state's position says that a filed verdict is
// in no record (`filed`); and `says`, the fact as a halt says it.
//   THE RECORD is `composeRecord` over EVERY REPORTER OF THE ATTEMPT THAT LEFT A REPORT — the
//   last report check took every other off the list — and no other word: the record script
//   takes what was filed itself. AFTER A RECORD OF THIS INVOCATION'S OWN it is composed where
//   a verifier was launched, and REFUSED AS ONE THAT CHANGES NOTHING IT IS NO HALT: nothing
//   was filed since. WHERE THE INVOCATION HAS MADE NONE it would be the first of its attempt
//   — the one that says the attempt reached a record, which always changes something — so
//   IT IS COMPOSED ONLY WHERE THERE IS SOMETHING TO RECORD: a verifier of the invocation left
//   a report — its verdict, or its launch, which the record script counts by that report —
//   or an earlier invocation's filed verdict is owed a record. WHERE NEITHER HOLDS NOTHING IS
//   RECORDED, and the attempt is one that left its marker and no record: the record script
//   counts those, and after two in a row the stage is the human's. That is the ONE EXIT of a
//   finding whose verifier leaves nothing at all — no report, so no launch anybody counts:
//   a record of nothing but its own attempt would make every such invocation one that
//   reached its record, and the loop would have no counter. Every other refusal halts the
//   stage at its record.
//   THE REPORT CHECK COMES BEFORE THE RECORD: a wave's own, after every wave — and, where no
//   wave ran and a record is composed all the same, one asked for here, before the composing
//   step (the header: REPORTS).
//   A HALT AFTER A WAVE THAT WAS THE LAST FOR ITS SILENCE, OF A STEP THAT RETURNED NOTHING
//   TOO, SAYS THAT IT MAY BE THE ACCOUNT'S LIMIT, AND NAMES THE FILED VERDICTS (`limitSaid`):
//   the wave's own report check, the read of the queue, the composing step, the record's
//   gate and its commit, the push, the read-back — AND IS HANDED WHICH OF THE SEVEN IT WAS,
//   so that what it says is so at that step. A step that REFUSED is no such halt, and
//   neither is a gate that still runs: both answered.
// It returns `halted` — the halt, whole — or the state as it was last read, `second`: the
// commit of the record it made, or null; and `verify`: WHAT THE PHASE DID, AS COUNTS.
async function verifyThenRecord(ctx, launch, built, from, made) {
  phase('Verify')
  const spent = from.queue.spent
  const ver = await verifyWaves(ctx, launch, built, from)
  // `limited` — A HALT OF THE PHASE OR OF ITS RECORD, AS ONE THAT MAY BE THE ACCOUNT'S LIMIT:
  // only after a wave that was made the last for its silence, and only where THE STEP THAT
  // HALTED RETURNED NOTHING (`silent` — a step that refused, and a gate that still runs,
  // answered). It is handed WHICH STEP that was (`step`, a row of LIMIT_STEPS) and, from the
  // push on, the commit the record made: what the halt says of the filed verdicts, of the
  // state document and of the next invocation is true at that step, or is not said.
  const limited = (h, step, silent, record) => {
    if (!ver.last || !silent) return h
    const said = limitSaid(ctx, ver.last, ver.state.queue.spent, ver.filings, ver.unconfirmed, step, record || null)
    return Object.assign(h, { halted: Object.assign(h.halted, said.more), message: h.message + said.says })
  }
  if (ver.fault && ver.unread) return { halted: limited(haltAfter('read-back', made.says + ', wave ' + ver.waves + ' of its verifiers ran, and the queue could not be read after it: ' + ver.fault + '. What a verifier filed is on disk, and the state counts it', { transient: !!ver.transient, step: ver.step, launched: launch.names }), 'queue', !!ver.transient) }
  if (ver.fault) return { halted: limited(haltAfter('report-check', 'after wave ' + ver.waves + ' of the stage\'s verifiers ' + ver.fault + '. ' + made.says.charAt(0).toUpperCase() + made.says.slice(1) + ', and what a verifier filed is on disk', { transient: !!ver.transient, step: ver.step, launched: launch.names }), 'report-check', !!ver.transient) }
  let state = ver.state
  let second = null
  const records = made.commit ? ver.launched > 0 : made.owed || launch.names.some((name) => name !== ATTEMPT)
  if (!records && !made.commit) log('NOTHING TO RECORD: no verifier of this invocation left a report, and no filed verdict is owed a record — so no record is composed, and attempt ' + ctx.attempt + ' is one that left its marker and no record. The record script counts those: after two in a row the stage is the human\'s, who grants one more')
  if (records) {
    phase('Record')
    // THE REPORT CHECK COMES BEFORE THE RECORD, IN THIS INVOCATION (the header: REPORTS; the
    // scoped review before the parts, its finding 2 in this script). After a wave it is that
    // wave's own. WHERE NO WAVE RAN — an invocation that continues a stage for its record
    // alone (`filed`): the queue printed no key, or the budget has nothing left — nothing of
    // this invocation had checked the reports, and the composing step was the first to meet
    // a file the tools did not write: refused (`check`), at this and at every later
    // invocation, until the stage's attempts were spent. So the check is asked for HERE, over
    // the attempt's own reporters: what the tools can set aside leaves the tree first.
    if (!ver.waves) {
      const settled = await settleReports(ctx, launch, 'check-reports:filed', 'Record')
      if (settled.fault) return { halted: limited(haltAfter('report-check', 'before the record of what was filed — no verifier was launched, so no wave\'s report check had come before it — ' + settled.fault + '. Nothing was composed, and what was filed is on disk', { transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null, launched: launch.names }), 'report-check', false) }
      asideSaid(settled.aside)
    }
    const again = await composeRecord('verdicts:r' + ctx.round, loopBranch, ctx, v.scratch + '/record/verdicts-r' + ctx.round + '-a' + ctx.attempt, 'docs(record): ' + subjectOf(v.run, ctx.round, 'the verdicts of the test stage'), launch.names.map((name) => 'reporter=' + name))
    if (again.fault && again.refusal !== NO_CHANGE) return { halted: limited(recordHalt('record', again, { refusal: again.refusal || null, launched: launch.names, first: made.commit }), 'compose', !!again.transient) }
    if (again.fault) log('NOTHING TO RECORD after the verifiers: the record script refused the record as one that changes nothing (`' + NO_CHANGE + '`) — no verdict was filed since the record before it, and no batch is left. That is no halt')
    else {
      const taken = await recordStep('verdicts:r' + ctx.round, loopBranch, again.rec)
      if (taken.fault) return { halted: limited(recordHalt('record', taken, { launched: launch.names, first: made.commit }), taken.at, taken.silent === true) }
      const published = await pushRecord('push:verdicts', loopBranch)
      if (!onIt(published, loopBranch, true)) return { halted: limited(pushHalt(published, loopBranch, 'the record of what the verifiers filed is committed on ' + loopBranch + ' (' + taken.result.commit + ') and the branch was not pushed'), 'push', !published, taken.result.commit) }
      const after = await readState()
      if (!after.state) return { halted: limited(haltAfter('read-back', 'the record of what the verifiers filed is committed and pushed (' + taken.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, step: after.step || null }), 'read-back', !!after.transient, taken.result.commit) }
      state = after.state
      second = taken.result.commit
    }
  }
  return { state, second, verify: { waves: ver.waves, launched: ver.launched, verdicts: state.queue.spent - spent, record: second, unverified: ver.unverified, unheld: ver.unheld, contested: ver.contested, silent: ver.silent, last: ver.last ? ver.last.wave : null } }
}

// continueTest — AN INVOCATION THAT CONTINUES A `test` STAGE, IN A RUN WITH A QUEUE (the
// header: AN INVOCATION THAT CONTINUES; the reshape's plan, §3 and T6b). `words` is what the
// state's position says is left of the stage (`continuing`). IT STARTS FROM THE COMMITTED
// STATE AND FROM GIT, and from nothing an earlier invocation knew:
//   NO INSTRUMENT RUNS — no preflight, no scope step, no gate of the candidate's, no held
//   check and no reporter of an item: every report of the round is on record, the round's
//   scope stands, and what the stage holds is the commit's. THE TWO BINARIES are asked for
//   under their commits' names, and answered by the builds that stand (`buildBoth`).
//   THE ATTEMPT IS BEGUN ON RECORD, ON THE CANDIDATE THE ROUND TESTED — the round's record
//   commits lie on top of it — and every reporter of the invocation reports under that
//   attempt: so a verifier that is launched for a finding a second time has a name the
//   record script takes, and the launch is counted by its report.
//   A TRIAGE ONLY OVER ROWS NOBODY GRADED (`triage`): the one triage, handed ONE source —
//   the rows the state lists as untriaged, by the read that prints them: in a run with a
//   queue those are the rows no triage saw and no verifier handed on — a red row a batch
//   filed under its own key, a seed added since. No reporter's finding, no red result and
//   no graded row is handed over, so NO ROW IS GRADED A SECOND TIME. Then ITS RECORD OF WHAT
//   IT GRADED, composed by the tool from that attempt's rendering, so that the rows are in
//   the queue the phase reads.
//   THE FILED VERDICTS ARE TAKEN AS THEY STAND: the state counted each from the moment it was
//   filed, so the queue does not print its finding, and no verifier is launched for it —
//   `undrivable` among them. ONLY WHAT IS MISSING IS LAUNCHED: what the queue prints, while
//   the budget has verdicts left (`verifyThenRecord`), and then the record — which takes
//   what this invocation's verifiers filed, what an earlier one's had, and every report an
//   earlier invocation left pending; or none, where there is nothing to record.
// It returns the stage's return, a refusal, or a halt.
async function continueTest(ctx, state, words) {
  const label = 'c' + ctx.round
  const said = '(`next` is `' + state.next + '`): ' + words.map((word) => CONTINUES[word]).join(', and ')
  const why = continueFault(v)
  if (why) return { status: 'refused', stage: 'test', run: v.run, message: 'round ' + ctx.round + ' is tested and its `test` stage is not finished ' + said + ' — ' + why + '. Nothing was run beyond the two reads.', next: state.next, untriaged: state.untriaged, position: state.position.test, named: namedOf(state), arrived }
  // THE CANDIDATE THE ROUND TESTED: its findings are about that commit, and the round's
  // record commits have moved the branch on since.
  const tested = String((state.rounds.find((r) => r.round === ctx.round) || {}).candidate || '')
  if (!SHA_RE.test(tested)) return halt('state', 'the state says that round ' + ctx.round + '\'s `test` stage is to be continued and names no candidate for that round: a verdict is of the candidate its round tested, and of no other commit')
  log('round ' + ctx.round + ', attempt ' + ctx.attempt + ' of the test stage: A CONTINUATION ' + said + ' — on ' + tested + ', the candidate the round tested. No instrument runs; the verdicts that are filed stand, and only what is missing is launched')
  if (v.stopAfter === 'state') return { status: 'stopped', after: 'state', stage: 'test', run: v.run, round: ctx.round, attempt: ctx.attempt, continues: words, candidate: { label, sha: tested }, owed: arrived.owed, state: attached(state), named: namedOf(state), arrived }
  const launch = launcher(ctx)
  const unbegun = await beginAttempt(ctx, launch, tested)
  if (unbegun) return unbegun
  const built = await buildBoth(label, tested, previousOf(state), 'State')
  if (built.fault) return halt(built.phase, built.fault, { transient: !!built.transient, held: built.held || null, then: built.then })
  // WHETHER A FILED VERDICT IS OWED A RECORD IS THE STATE'S WORD (`filed`), never the first
  // read's: that read names a file no COMMIT holds, and a verdict filed between a record's
  // batch and its commit rides that commit with no table holding it — owed all the same.
  let made = { commit: null, owed: words.includes('filed'), says: 'the stage\'s first record stands, committed by an earlier invocation' }
  let once = { triage: null, handed: 0 }
  if (words.includes('triage')) {
    phase('Triage')
    once = await triageOnce(ctx, launch, ledgerSource(v.run, state))
    if (once.fault) return haltAfter('triage', once.fault, { transient: !!once.transient, halt: once.halt || null, step: once.step || null })
    if (once.triage) {
      phase('Record')
      const composed = await composeRecord('graded:r' + ctx.round, loopBranch, ctx, v.scratch + '/record/graded-r' + ctx.round + '-a' + ctx.attempt, 'docs(record): ' + subjectOf(v.run, ctx.round, 'the rows nobody had graded'), launch.names.map((name) => 'reporter=' + name))
      if (composed.fault) return recordHalt('record', composed, { refusal: composed.refusal || null, launched: launch.names, triage: once.triage })
      const recorded = await recordStep('graded:r' + ctx.round, loopBranch, composed.rec)
      if (recorded.fault) return recordHalt('record', recorded, { launched: launch.names, triage: once.triage })
      const pushed = await pushRecord('push:graded', loopBranch)
      if (!onIt(pushed, loopBranch, true)) return pushHalt(pushed, loopBranch, 'the record of the rows this invocation graded is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
      const read = await readState()
      if (!read.state) return haltAfter('read-back', 'the record of the rows this invocation graded is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient, step: read.step || null })
      state = read.state
      made = { commit: recorded.result.commit, says: 'the record of the rows this invocation graded is committed and pushed (' + recorded.result.commit + ')' }
    }
  }
  const closing = await verifyThenRecord(ctx, launch, built, state, made)
  if (closing.halted) return closing.halted
  state = closing.state
  // WHAT IT RETURNS: what it did, in one word (`statusOf`: `verified` where it made the
  // record of its verify phase, else `continued` — never `hunted`, which it did not); the
  // record it made of what it graded (`record`: null where it graded nothing); what the
  // phase did, as counts; `continued`: what of the stage it was handed to finish — and,
  // beside `next`, what every return of a run with a queue says (`queueSaid`). No list.
  return Object.assign({
    status: statusOf(words, closing.second),
    stage: 'test',
    run: v.run,
    round: ctx.round,
    continued: words,
    candidate: built.candidate,
    record: made.commit,
    triage: once.triage,
    verify: closing.verify,
    counts: { reporters: launch.names.length, findings_in: once.handed, blockers: state.blockers, for_the_human: state.human_list },
    forbids_close: state.forbids_close,
  }, nextOf(state))
}

// driveFork — ruling 5: a contested fix, or one that needs a new mechanism, goes to the
// human with a robust-advocate's case, and the proposal is driven before the human sees it —
// by the advocate, as its definition has it, and then by an agent that is not the advocate.
// `argued` and `drove` say whether each of the two returned what a fork is made of, and
// `unheld` why what one of them drove is no evidence about the candidate (`unasserted`) —
// then the fork is none. The proposal of a case argued on a binary nobody asserted is not
// driven: there is no fork it could complete.
async function driveFork(ctx, launch, built, fork, tag) {
  const aName = launch.add(['advocate', tag, fork.key])
  const adv = await roleStep('advocate', 'advocate:' + fork.key, 'Triage', advocatePrompt(ctx, launch, aName, fork, built), ADVOCATE_SCHEMA)
  const out = { fork: Object.assign({}, fork, { advocate: adv, independent_drive: null, driven_by: null }), aName, pName: null, argued: !!adv && adv.status === 'argued', drove: false, leftOpen: [], unheld: null }
  if (!out.argued) return out
  if (adv.left_open && adv.left_open.length) out.leftOpen.push(leftOpenSource(aName, adv.report, adv.left_open))
  const noCase = unasserted('advocate', aName, adv, built.candidate.sha256)
  if (noCase) {
    out.unheld = 'its advocate ' + noCase
    return out
  }
  out.pName = launch.add(['proposal', tag, fork.key])
  const drive = await roleStep('proposal', 'proposal:' + fork.key, 'Triage', proposalPrompt(ctx, out.pName, fork, adv, built), PROPOSAL_SCHEMA)
  out.fork.independent_drive = drive
  out.fork.driven_by = ROLES.proposal.agentType
  out.drove = !!drive && drive.status === 'driven'
  if (out.drove) {
    const noDrive = unasserted('proposal', out.pName, drive, built.candidate.sha256)
    if (noDrive) out.unheld = 'the independent drive of the advocate\'s proposal ' + noDrive
    if (drive.left_open && drive.left_open.length) out.leftOpen.push(leftOpenSource(out.pName, drive.report, drive.left_open))
  }
  return out
}

// preflightOf — one preflight call: the environment asserts, the trial image where it is
// asked for, and the checks that are a brief. IT BUILDS NOTHING: the binaries are the
// tool's (`buildBoth`).
async function preflightOf(ctx, launch, parts, plan, phaseTitle) {
  const name = launch.add(parts)
  const r = await roleStep('preflight', name, phaseTitle, preflightPrompt(ctx, launch, name, plan), PREFLIGHT_SCHEMA)
  if (!r || r.status !== 'ready') return { name, fault: r ? 'the preflight halted: ' + ((r.halt && r.halt.root_cause) || 'no reason given') : 'the preflight returned no result', transient: !r, halt: r ? r.halt : null }
  if (plan.image && !(r.image && r.image.verified && r.image.tag)) return { name, fault: 'the trial image is not verified: ' + JSON.stringify(r.image || null) }
  return { name, result: r }
}

// attached — what a return holds of the state where the orchestrator is handed it: what the
// digest holds — words, ids and counts — and never the document, which lies in the file
// `named.document` names. `blockers`, `human_list`, `untriaged` and `ledger` are HOW MANY.
function attached(state) {
  return { next: state.next, stop: state.stop, not_ready: state.not_ready, candidate: state.candidate, fix_rounds: state.fix_rounds, evidence: evidenceOf(state), ledger: state.ledger, blockers: state.blockers, human_list: state.human_list, reverify: state.reverify, untriaged: state.untriaged, retest: state.retest, human_clauses: state.human_clauses, human_stages: state.human_stages, unsettled: state.unsettled, forbids_close: state.forbids_close, position: state.position }
}
// What a stage returns of `next`: the value itself, always; what the digest holds of the
// state beside it when this script does not know the value — and the file the document lies
// in, never the document; and with `close` the step the close owes first, and
// per clause how far behind the candidate its last evidence is. AND WITH EVERY VALUE, WHAT
// NOTHING CAN HOLD (`named`): the in-scope items no tested round has selected, and how many
// doors of the round no item names — at every stop and at the close, where the human reads
// them (dev/stabilize-record: WHAT CANNOT BE HELD IS NAMED).
function nextOf(state) {
  const o = outcomeOf(state)
  const named = namedOf(state)
  // … AND, OF A RUN WITH A QUEUE, WITH EVERY VALUE: the budget, the queue, the stop's view,
  // and what the stage is continued for (`queueSaid`).
  const queued = queueSaid(state)
  if (!o.known) return Object.assign({ next: o.next, returned_to_orchestrator: true, state: attached(state), named, arrived }, queued)
  if (o.rule) return Object.assign({ next: o.next, rule: o.rule, named, arrived }, queued)
  return Object.assign(o.next === 'close' ? { next: o.next, close: closeOf(), evidence: evidenceOf(state), named, arrived } : { next: o.next, named, arrived }, queued)
}

// rulingsStep — the ONE call of the step that records what the human ruled: its record, the
// push of the branch it is on, and the state read back.
async function rulingsStep(round, branch, ran, at) {
  phase('Record')
  const ruled = await recordStep('rulings:r' + round, branch, rulingsRecordPrompt(v, round, branch, v.rulings, ran, at))
  // The same rulings, sent twice: the record holds them already. A refusal that leaves no
  // batch — never a halt that asks for a discard.
  if (ruled.unchanged) return { halted: { status: 'refused', stage: v.stage, run: v.run, refused: { refused: NO_CHANGE }, halt: ruled.halt, arrived, named: namedOf(lastState), message: 'the rulings are on record already: the record script refused their batch as one that changes nothing (`' + NO_CHANGE + '`).' + ruled.then + ' Nothing was recorded, committed or pushed.' } }
  if (ruled.fault) return { halted: recordHalt('rulings', ruled) }
  const pushed = await pushRecord('push:rulings', branch)
  if (!onIt(pushed, branch, true)) return { halted: pushHalt(pushed, branch, 'the rulings are recorded on ' + branch + ' (' + ruled.result.commit + ') and the branch was not pushed') }
  const read = await readState()
  if (!read.state) return { halted: halt('state', 'the rulings are recorded (' + ruled.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient, step: read.step || null }) }
  return { state: read.state, record: ruled.result.commit }
}

// ruleTheRun — an invocation that ONLY RECORDS RULINGS: what the human ruled about the run
// (a go after a stop, one more re-run of a clause, the bound raised), which either stage
// takes — or, on the `test` stage, what the human ruled on findings and bounds. They are
// recorded on the loop branch by the one step that may write them, NOTHING is started, and
// the state's `next` goes back — which now names the step the human was asked about.
async function ruleTheRun(state, checkedOut) {
  const told = { status: 'refused', stage: v.stage, run: v.run, next: state.next, stop: state.stop, human_clauses: state.human_clauses, human_stages: state.human_stages, reverify: state.reverify, named: namedOf(state), arrived }
  if (checkedOut !== loopBranch) return Object.assign(told, { message: 'an invocation that only records rulings records them on the loop branch ' + loopBranch + ', and `' + checkedOut + '` is checked out. Nothing was run beyond the two reads.' })
  const about = v.rulings.every(runRuling)
  // Whether a ruling on a finding names a row is the batch's own to hold (`check-ledger`).
  const why = about ? runRulingsFault(v.rulings, state) : null
  if (why) return Object.assign(told, { message: why + '. Nothing was recorded.' })
  // The rounds in which a clause has an item whose re-run is spent: where a grant is a fact.
  const ran = {}
  for (const c of state.clauses || []) ran[c.clause] = (state.items || []).filter((i) => i.clause === c.clause && i.rerun === 'spent').map((i) => i.round).filter((n, at, all) => all.indexOf(n) === at)
  // The round a finding's passes are counted in, and the round each stage is spent in.
  const stages = {}
  for (const h of state.human_stages || []) stages[h.stage] = h.round
  // THE REPORT CHECK'S VET COMES BEFORE THE BATCH OF THE RULINGS, IN A RUN WITH A QUEUE (the
  // header: REPORTS; the scoped review before the parts, its finding 2 in this script). Every
  // batch takes what was filed first — a ruling's too — so a file a hand left among a round's
  // filed verdicts refused the batch where it is applied (`check`), at every invocation; and
  // one beside a stage's reports, a pending path of such a run, was staged by the commit step
  // and refused by its vet: the batch taken back, one whole gate spent, and the ruling to be
  // sent again. A record of rulings names no stage's reporters, so what it is preceded by is
  // the check's first half alone: what the tools can set aside leaves the tree before
  // anything is applied. A RUN WITHOUT AN ORDER RECORDS ITS RULINGS AS IT DID: nothing is
  // filed there, and the record script names no stray file of it as pending.
  if (hasQueue(state)) {
    phase('Record')
    const vetted = await toolStep('check-reports:rulings', 'Record', 'check-reports', checkReportsPrompt(v, null))
    if (!vetted || vetted.status !== 'vetted' || !Array.isArray(vetted.aside)) return gitHalt('reports', vetted, 'the rulings were not recorded: the report check\'s vet, which comes before their batch, did not answer with what it set aside — nothing was applied')
    log('the report check\'s vet, before the batch of the rulings: ' + (Number.isInteger(vetted.vetted) ? vetted.vetted : 'the') + ' pending file(s) of the run vetted, ' + vetted.aside.length + ' set aside')
    asideSaid(vetted.aside)
  }
  const ruled = await rulingsStep((about && state.stop ? state.stop.round : state.round) || 0, loopBranch, ran, { round: state.round, stages })
  if (ruled.halted) return ruled.halted
  return Object.assign({ status: 'ruled', stage: v.stage, run: v.run, rulings: v.rulings, record: ruled.record, message: 'the human\'s rulings ' + (about ? 'about the run' : 'on the round\'s findings and bounds') + ' are recorded, and nothing was started: `next` names the step.' }, nextOf(ruled.state))
}

// finishTriage — a round's triage that a stage left unfinished (`next: 'triage'`), finished
// by that stage's next invocation: the rows the state lists as untriaged are graded, verified
// and recorded, and NO INSTRUMENT RUNS — every report of the round is on record already.
// Which stage, which round and which attempt is the position's (`triage: true`), read from
// committed state; nothing here is an argument. `tip` is the commit the verifiers drive.
async function finishTriage(ctx, launch, state, tip, branch) {
  phase('Triage')
  const sources = ledgerSource(v.run, state)
  log('round ' + ctx.round + ': the ' + ctx.stage + ' stage left its triage unfinished — ' + findingsOf(sources[0] || { findings: [] }) + ' row(s) are graded and verified now, attempt ' + ctx.attempt + '; no instrument runs')
  const pre = await preflightOf(ctx, launch, ['preflight'], { image: false, sha: tip.sha, label: tip.label, branch, checks: [], crossModel: false, tested: tip.tested }, 'Triage')
  if (pre.fault) return { halted: halt('preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, branch }) }
  const built = await buildBoth(tip.label, tip.sha, previousOf(state), 'Triage')
  if (built.fault) return { halted: halt(built.phase, built.fault, { transient: !!built.transient, held: built.held || null, branch, then: built.then }) }
  const tri = await triagePasses(ctx, launch, built, sources, [])
  if (tri.fault) return { halted: haltAfter('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, step: tri.step || null, branch }) }
  // The lap stands on its preflight as a stage does: binaries with no report behind them
  // are no evidence, whatever was verified on them.
  if (tri.unreportedBy.includes(pre.name)) return { halted: haltAfter('stood-on', 'the preflight of this lap returned and left no report: what was driven on its binaries has nothing on record behind it', { branch, launched: launch.names }) }
  phase('Record')
  const rec = triageRecord(ctx, tri.entries)
  const dir = v.scratch + '/record/triage-r' + ctx.round + (ctx.stage === 'fix' ? '-c' + ctx.cycle : '') + '-a' + ctx.attempt
  const commands = stageRecordCommands(ctx, { reporters: launch.names, rows: rec.rows, triage: rec.triage, patches: [], clauses: [], facts: null, keys: tri.entries.map((e) => e.key) })
  const recorded = await recordStep('triage:r' + ctx.round, branch, recordPrompt(v, 'round ' + ctx.round + '\'s triage, finished — ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s) graded; no instrument ran', branch, dir, ctx.round, commands, subjectOf(v.run, ctx.round, 'the triage of the round, finished')))
  if (recorded.fault) return { halted: recordHalt('record', recorded, { launched: launch.names, branch, forks: tri.forks }) }
  const pushed = await pushRecord('push:triage', branch)
  if (!onIt(pushed, branch, true)) return { halted: pushHalt(pushed, branch, 'the triage is recorded on ' + branch + ' (' + recorded.result.commit + ') and the branch was not pushed') }
  const after = await readState()
  if (!after.state) return { halted: haltAfter('read-back', 'the triage is recorded and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, step: after.step || null, branch }) }
  return { state: after.state, entries: tri.entries, forks: tri.forks, unverified: tri.unverified, handed: tri.handed, record: recorded.result.commit, reporters: launch.names, candidate: built.candidate }
}
function closeOf() {
  return {
    order: ['the close of the run, as the workflow\'s doc has it', 'sync: close.sync, spawned verbatim on model close.sync.model', 'dev/gate green on the merged tree', 'git push origin ' + loopBranch, 'gh pr create --base main --head ' + loopBranch],
    sync: { agentType: 'build-git', model: GIT_MODEL, label: 'git:sync-main', prompt: syncMainPrompt(v.run, v.scratch), schema: STEP_SCHEMA, reads: 'its `line` is the ONE line `' + STEP_TOOL + ' sync-main` printed, a digest — JSON: status `merged` or `up-to-date` with head, origin_main and resolved_logs, or `halted` with the word it refused with (`refused`); and `file`, the path under ' + v.scratch + ' of the line the tool prints unasked, its halt report in it' },
  }
}

// ---- the runtime probes ----
// watched — a call of a probe, and how each try of it ended. A PROBE'S AGENTS ARE MEANT TO
// FAIL — that is what it observes — so two calls that exhausted their retries are its answer
// and no sign of a rate limit: the breaker is put back, and the step that judges still runs.
async function watched(call) {
  triesSeen = []
  const back = await call()
  const tries = triesSeen
  triesSeen = null
  exhaustedLabels = []
  breakerTripped = false
  return { back, tries }
}
// probeAct — a probe's step that is ONE command of the probe tool, on the git steps' model:
// the return as it came back, and what `readStep` makes of its line — EXACTLY as it came
// back: a probe measures the relay, so a line with a value slipped is altered here, and is
// never mended (`mendSlip`).
async function probeAct(label, act, what, flags) {
  const raw = await gitStep('probe:' + label, 'Probe', probeStep(what, act, flags), STEP_SCHEMA)
  return { raw, read: readStep(raw, act, true) }
}
// faultOf — why a step's line is not an answer, in a few words, or null: never the line.
function faultOf(read) {
  if (!read) return 'nothing came back'
  if (read.relay) return read.relay
  return read.status === 'halted' ? (read.halt && read.halt.root_cause) || 'the step halted' : null
}
// runProbe — ONE probe, whole: its root made by the tool, its cases observed with the roles
// a stage launches, and what was observed handed to the tool as tokens — letters, digits
// and hashes this script computed, never a word an agent wrote — which it judges against
// what it knows itself, writes under the scratch root and prints.
async function runProbe() {
  phase('Probe')
  const s = v.scratch
  const stopped = (at, why, more) => Object.assign({ status: 'halted', probe: v.probe, scratch: s, halted: Object.assign({ phase: at, reason: why }, more || {}), message: 'the `' + v.probe + '` probe HALTED at ' + at + ': ' + why + '. No file of the repository was written; what the probe left lies under ' + s + '/probe/' + v.probe + '/. Invoke it again under a FRESH scratch root: a probe\'s root is made once.' })
  const begun = (await probeAct('begin', 'begin', 'its first act: the root of the `' + v.probe + '` probe under the scratch root, and what the probe stands on', '--scratch ' + s + ' --probe ' + v.probe)).read
  if (!begun || begun.status !== 'begun') return stopped('begin', 'the probe was not begun: ' + faultOf(begun), { refused: (begun && begun.refused) || null, halt: (begun && begun.halt) || null })
  const observed = []
  const cases = []
  if (v.probe === 'required') {
    for (const id of ['a', 'b']) {
      const { back, tries } = await watched(() => roleStep('review', 'probe:required:' + id, 'Probe', probeReviewPrompt(id, begun.binary), PROBE_UNIT_SCHEMA))
      observed.push({ case: id, tries, returned: back })
      cases.push(id + ':' + triesOf(tries) + ':' + probeField(back))
    }
  }
  if (v.probe === 'relay' || v.probe === 'relay-document') {
    for (const rows of PROBE_ROWS) {
      for (let n = 1; n <= PROBE_RELAYS; n++) {
        const tag = 'r' + rows + '-' + n
        const { back, tries } = await watched(() => probeAct('state:' + tag, 'state', 'the ONE line of the state of a throwaway run of ' + rows + ' ledger row(s), relayed — relay ' + n + ' of ' + PROBE_RELAYS, '--scratch ' + s + ' --probe ' + v.probe + ' --rows ' + rows + ' --tag ' + tag))
        const cell = probeRelayed(back.raw, back.read)
        observed.push({ case: tag, tries, back: cell, fault: faultOf(back.read) })
        cases.push(tag + ':' + triesOf(tries) + ':' + cell)
      }
    }
  }
  if (v.probe === 'payload') {
    for (const n of PROBE_ENTRIES) {
      const batch = probeBatch(s, n)
      const { back, tries } = await watched(() => roleStep('record', 'probe:payload:e' + n, 'Probe', probePayloadPrompt(s, n, batch), STEP_SCHEMA))
      const read = readStep(back, 'hash', true)
      const relayed = read && read.status === 'hashed' && SHA256_RE.test(String(read.file_sha256)) ? read.file_sha256 : 'none'
      observed.push({ case: 'e' + n, tries, composed: { file: batch.file, sha256: batch.sha256 }, relayed, fault: faultOf(read) })
      cases.push('e' + n + ':' + triesOf(tries) + ':' + batch.sha256 + ':' + relayed)
    }
  }
  if (v.probe === 'hold') {
    const seconds = v.seconds || PROBE_HOLD
    // (a) HELD AS A STAGE HOLDS ITS GATE (the second repair plan's task K11): one step
    // starts the command THROUGH THE STEP TOOL — its held kind `probe`, under the name the
    // probe tool judges — and ONE agent asks after it, again and again, inside its turn:
    // the same two prompts a stage's held command gets, read by the same reader. So this
    // case, run under the runtime, is the test that no step of a held command asks the
    // human for anything.
    const begunA = heldOf(await toolStep('hold-start:a', 'Probe', 'hold-start', holdStartPrompt(v, 'a', 'probe', '--seconds ' + seconds, 'the hold probe\'s long command, case a, of ' + seconds + ' seconds')), s)
    const a = await watched(() => toolStep('hold-wait:a', 'Probe', 'hold-wait', holdWaitPrompt(v, 'a', 'the hold probe\'s long command, case a')))
    const held = heldOf(a.back, s)
    const line = HELD_TAKES.probe.includes(held.ends) && held.kind === 'probe' && held.name === 'a' ? 'held' : a.back && !a.back.relay && held.name === 'a' ? 'other' : 'none'
    observed.push({ case: 'a', seconds, started: begunA.ends, tries: a.tries, line, ends: held.ends, why: held.why || null })
    cases.push('a:' + triesOf(a.tries) + ':' + line)
    // (b) One step starts it and returns; later steps ask for it, a slice each, until it
    // has ended or as many slices as it could take are spent.
    const started = (await watched(() => probeAct('hold:b', 'hold', 'the long command, STARTED in a session of its own: this step returns at once, and the command runs on for ' + seconds + ' seconds', '--scratch ' + s + ' --name b --seconds ' + seconds + ' --detach'))).back.read
    const most = Math.ceil(seconds / PROBE_SLICE) + 2
    let reads = 0
    let last = 'none'
    while (reads < most && last !== 'done' && last !== 'dead') {
      reads++
      const answer = (await watched(() => probeAct('held:b:' + reads, 'held', 'what became of the long command another step started, asked within a slice of ' + PROBE_SLICE + ' seconds — read ' + reads, '--scratch ' + s + ' --name b --slice ' + PROBE_SLICE))).back.read
      last = answer && ['done', 'running', 'dead'].includes(answer.status) ? answer.status : 'none'
      // A hold nobody started is asked for once: the tool's refusal is the answer.
      if (answer && answer.refused) break
    }
    observed.push({ case: 'b', seconds, started: started ? started.status : null, reads, last, fault: faultOf(started) })
    cases.push('b:' + reads + ':' + last)
  }
  const asked = '--scratch ' + s + ' --probe ' + v.probe + ' --sum ' + sha256(cases.join(' ')) + ' -- ' + cases.join(' ')
  const judged = (await probeAct('verdict', 'verdict', 'the judgement: what the harness observed of each case, held by the tool to what it knows itself, and written under the scratch root', asked)).read
  // What was observed is not lost with the step that should have judged it: the command is
  // returned, and whoever runs it as it stands gets the verdict.
  if (!judged || judged.status !== 'judged' || !Array.isArray(judged.cases)) return stopped('verdict', 'what the probe observed was not judged: ' + faultOf(judged) + '. The observations stand: `halted.command`, run as it is from the repository\'s root, judges them', { refused: (judged && judged.refused) || null, halt: (judged && judged.halt) || null, command: PROBE_TOOL + ' verdict ' + asked, observed })
  return { status: 'probed', probe: v.probe, scratch: s, result: judged.file, cases: judged.cases, observed, message: 'the `' + v.probe + '` probe ran: `cases` is what ' + PROBE_TOOL + ' judged, one entry per case with its `verdict`, as it wrote it to ' + judged.file + '; `observed` is what this script saw of each call. What a verdict decides is implementation/stabilization-workflow.md -> The runtime probes.' }
}

// ---- the `test` stage ----
async function runTest() {
  phase('State')
  // AN INVOCATION THAT ONLY LOOKS finishes nothing: its first read says what is owed.
  const looks = v.stopAfter === 'state'
  const gs = await toolStep('state', 'State', 'git-state', gitStatePrompt(v, looks))
  if (!onIt(gs, loopBranch, false)) return gitHalt('git', gs, 'the tree and the branches are not in the state the `test` stage starts from — the loop branch ' + loopBranch + ' checked out, holding nothing but what the record script wrote and no commit holds yet, no product change outside a round\'s merge, and no commit the remote lacks that is no record\'s')
  // EVERY INVOCATION RECONCILES FIRST: what the read met, what it finished — a killed write,
  // a batch whose commit was made, the push that was owed — and what it left.
  arrived = { found: gs.found || [], finished: gs.finished || [], owed: gs.owed || [] }
  if (arrived.finished.length) log('the first read finished what an earlier invocation left: ' + arrived.finished.join(', '))
  stageHead = String(gs.head)
  stageFence = SHA256_RE.test(String(gs.fence)) ? String(gs.fence) : null
  // … BUT A FILED VERDICT NO COMMIT HOLDS IS NOTHING THE FIRST READ FINISHES: it is owed to
  // the stage, the state is read across it, and the look goes on to say what the stage is
  // continued for (`continues`, with `filed` among it where a record is owed).
  if (looks && arrived.owed.some((word) => word !== 'filed')) return { status: 'stopped', after: 'state', stage: 'test', run: v.run, owed: arrived.owed, pending: gs.pending || null, head: gs.head, remote_head: gs.remote_head || null, arrived, message: 'this invocation only looked (stopAfter: \'state\'), and FINISHED NOTHING: no write was finished, no batch gated or committed, nothing pushed, and the state was not read across it. What an invocation without that stop finishes before it does anything else is `owed` — ' + arrived.owed.join(', ') + ' — as `' + STEP_TOOL + ' table` names each.' }
  if (gs.pending) return await finishRecord(gs)
  const first = await readState()
  if (!first.state) return halt('state', 'the run\'s state could not be read: ' + first.error, { transient: !!first.transient, halt: first.halt || null, step: first.step || null })
  let state = first.state
  if (!state.opened) return halt('state', 'the run `' + v.run + '` has no opening record (' + runDir(v.run) + '/opening.md): a run opens with the human-led step, and `test` does not start before it')
  if (v.rulings) return await ruleTheRun(state, gs.branch)
  const at = state.position.test
  if (at.refused) return { status: 'refused', stage: 'test', run: v.run, refused: at, message: 'the `test` stage is refused (' + at.refused + ', round ' + at.round + '): ' + refusalOf(at) + '. Nothing was run beyond the two reads.', next: state.next, not_ready: state.not_ready, stop: state.stop, named: namedOf(state), arrived }
  const ctx = { run: v.run, round: at.round, stage: 'test', attempt: at.attempt, scratch: v.scratch }
  const label = 'c' + ctx.round
  // IN A RUN WITH A QUEUE AN INVOCATION THAT FINDS THE ROUND'S `test` STAGE UNFINISHED
  // CONTINUES IT (the header: AN INVOCATION THAT CONTINUES; the reshape's plan, T6b): such a
  // run stands, after the invocation that recorded its findings, where a row nobody graded
  // is left (`triage`), a row of its queue is unverified and budget is left (`verify`), or a
  // filed verdict is in no record (`filed`).
  // The lap below is the other path's — it would grade every row a second time and launch
  // an advocate — and is never entered for a run with a queue: every way out of
  // `continueTest` is a return.
  const continues = continuing(state, at)
  if (continues.length) return await continueTest(ctx, state, continues)
  // The round's triage is not finished, and this stage left it: finish it, and run nothing.
  if (at.triage) {
    if (v.scope != null || v.clause != null || v.crossModel != null) return { status: 'refused', stage: 'test', run: v.run, message: 'round ' + at.round + '\'s triage is not finished (`next` is `' + state.next + '`): this invocation finishes it and runs no instrument, so it takes no scope, clause or crossModel. Nothing was run beyond the two reads.', next: state.next, untriaged: state.untriaged, named: namedOf(state), arrived }
    // The verifiers drive THE CANDIDATE THE ROUND TESTED: its findings are about that
    // commit, and the round's record commits have moved the branch on since.
    const tested = String(state.rounds.find((r) => r.round === at.round).candidate)
    const lap = launcher(ctx)
    const unbegun = await beginAttempt(ctx, lap, tested)
    if (unbegun) return unbegun
    const done = await finishTriage(ctx, lap, state, { sha: tested, label, tested: true }, loopBranch)
    if (done.halted) return done.halted
    return Object.assign({ status: 'triaged', stage: 'test', run: v.run, round: ctx.round, triage_only: true, candidate: done.candidate, record: done.record, counts: { reporters: done.reporters.length, findings_in: done.handed, entries: done.entries.length, blockers: done.state.blockers, for_the_human: done.state.human_list }, forks: done.forks, unverified: done.unverified, forbids_close: done.state.forbids_close }, nextOf(done.state))
  }
  // A RE-RUN is the position's, never the invocation's to decide: the state asks for it
  // (`rerun`) and names the clause, the round that selected its due items, and the attempt
  // each one's run will be. `args.clause` says that the orchestrator read the same answer.
  const rerun = at.rerun || null
  if (rerun && v.clause !== rerun.clause) return { status: 'refused', stage: 'test', run: v.run, message: 'the state asks for a re-run (`next` is `' + state.next + '`): the items of clause `' + rerun.clause + '` that are due in round ' + rerun.round + ' — ' + rerun.items.map((i) => i.item + ' (attempt ' + i.attempt + ')').join(', ') + '. Invoke `test` with clause: \'' + rerun.clause + '\'; an invocation without it would begin a round nobody asked for. Nothing was run beyond the two reads.', next: state.next, retest: state.retest, position: at, named: namedOf(state, rerun.round), arrived }
  if (!rerun && v.clause != null) return { status: 'refused', stage: 'test', run: v.run, message: 'args.clause asks for a re-run, and the state does not (`next` is `' + state.next + '`): an item is run again only where the record script lists it as due, as an attempt of the round that selected it. Nothing was run beyond the two reads.', next: state.next, retest: state.retest, named: namedOf(state), arrived }
  // THE COMMIT THIS INVOCATION TESTS, AND NAMES IN EVERY RESULT — A ROUND TESTS ONE
  // CANDIDATE (ruling 11; the orchestrator's ruling of 2026-10-07 on the core review's F3).
  // A round that is begun here has none on record yet: its candidate is the branch's tip,
  // which the first read returned. A RE-RUN IS AN ATTEMPT INSIDE A ROUND THAT HAS ONE — and
  // by then the round's record commits lie on top of it, so the tip is another commit: the
  // re-run builds, drives and records THE ROUND'S CANDIDATE, as the state names it, and
  // never the tip.
  const sha = rerun ? String((state.rounds.find((r) => r.round === rerun.round) || {}).candidate || '') : String(gs.head)
  if (!SHA_RE.test(sha)) return halt('state', 'the state asks for a re-run inside round ' + at.round + ' and names no candidate for that round: a result is of the candidate its round tested, and of no other commit')
  // Whether the candidate is still the tree checked out. In a re-run it is not: what is
  // built is built from the commit, and NO CHECK THAT READS THE WORKING TREE IS ASKED FOR —
  // a tree that is not the candidate is evidence about no commit (stabilize-preflight.md).
  const moved = sha !== String(gs.head)
  const launch = launcher(ctx)
  const items = state.items || []
  // The items this invocation names for a cross-model source pass — none, unless it names them.
  const crossNamed = v.crossModel || []
  const crossStrangers = crossNamed.filter((id) => !items.some((i) => i.item === id && chainOf(i.kind, true).some((stepList) => stepList.some((s) => s.crossModel))))
  if (crossStrangers.length) return halt('state', 'args.crossModel names ' + crossStrangers.join(', ') + ', which is no item of the test set whose chain has a cross-model pass — the items that have one are ' + (items.filter((i) => chainOf(i.kind, true).some((stepList) => stepList.some((s) => s.crossModel))).map((i) => i.item).join(', ') || '(none)'))
  const runs = (i) => (rerun ? rerun.items.some((x) => x.item === i.item) : i.selected === true)
  log('round ' + ctx.round + ', attempt ' + ctx.attempt + ' of the test stage: ' + (rerun ? 'A RE-RUN inside it, on ' + sha + ' — ' + rerun.items.map((i) => i.item + ' (its attempt ' + i.attempt + ')').join(', ') + ' of clause `' + rerun.clause + '`, and nothing else' : 'candidate ' + label + ' = ' + sha + '; ' + items.length + ' item(s) in the test set'))
  if (looks) return { status: 'stopped', after: 'state', stage: 'test', run: v.run, round: ctx.round, attempt: ctx.attempt, candidate: { label, sha }, owed: arrived.owed, state: attached(state), named: namedOf(state, rerun ? rerun.round : null), arrived }
  const unbegun = await beginAttempt(ctx, launch, sha)
  if (unbegun) return unbegun

  // The preflight, then what the tool holds — the two builds, and the candidate's gate —
  // beside the scope step: neither reads what the other writes. THE PREFLIGHT COMES FIRST:
  // an environment assert that fails stops the stage before anything is built. A RE-RUN
  // has no scope step — the round's scope stands, and its doors are what the items run
  // over again — and no gate of the candidate's to hold: its round has one on record, and
  // the tree is no longer that candidate.
  phase('Preflight and scope')
  const earlier = (state.rounds || []).find((r) => r.round === ctx.round - 1)
  const always = items.filter((i) => i.kind === CHECK_KIND && runs(i))
  const scopeName = rerun ? null : launch.add(['scope'])
  const steps = [async () => {
    const asserted = await preflightOf(ctx, launch, ['preflight'], { image: false, sha, label, branch: loopBranch, checks: moved ? [] : always, crossModel: crossNamed.length > 0, tested: moved }, 'Preflight and scope')
    if (asserted.fault) return asserted
    const binaries = await buildBoth(label, sha, previousOf(state), 'Preflight and scope')
    if (binaries.fault) return Object.assign({ name: asserted.name }, binaries)
    // THE CANDIDATE'S OWN GATE, ONCE: a command the tool holds, whose whole output the
    // tool keeps — the record's batch puts what it shows red on record, a record commit is
    // held to exactly that, and a held gate of the test set is answered from this run of
    // it. Green and red are both FACTS about the candidate; a gate that left no verdict
    // leaves nothing a record could be held to, and the stage halts there.
    const gate = rerun ? null : await hold({ name: 'gate-' + sha.slice(0, 12), stands: true, kind: 'gate', flags: '--run ' + v.run, what: 'the full gate of the candidate, label ' + label, phase: 'Preflight and scope' })
    if (gate && !HELD_TAKES.gate.includes(gate.ends)) return { name: asserted.name, fault: heldFault('the candidate\'s gate', gate) + ' A record commit is held to the candidate\'s own gate, so nothing of this attempt could be recorded against one.', phase: 'gate', held: heldSaid(gate), transient: gate.ends === 'unread' || gate.ends === 'running', then: heldThen(v.scratch, gate) }
    return { name: asserted.name, result: asserted.result, built: binaries, gate }
  }]
  if (!rerun) steps.push(() => roleStep('scope', 'scope', 'Preflight and scope', scopePrompt(ctx, launch, scopeName, { sha, label, base: earlier && earlier.candidate ? earlier.candidate : null, earlier: !!earlier, scope: v.scope, previous: previousOf(state), fallback: state.facts.scope }), SCOPE_SCHEMA))
  const both = await parallel(steps)
  const pre = both[0] || { fault: 'the preflight returned no result', transient: true }
  const sc = rerun ? { status: 'stands' } : both[1]
  if (pre.fault) return halt(pre.phase || 'preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, held: pre.held || null, then: pre.then })
  if (!sc || (sc.status !== 'written' && sc.status !== 'stands')) return halt('scope', sc ? 'the scope step halted: ' + ((sc.halt && sc.halt.root_cause) || 'no reason given') : 'the scope step returned no result', { transient: !sc, halt: sc ? sc.halt : null })
  const built = pre.built
  const second = await readState()
  if (!second.state) return halt('state', 'the run\'s state could not be read after the scope step: ' + second.error, { transient: !!second.transient, halt: second.halt || null, step: second.step || null })
  state = second.state
  if (!rerun && (!state.doors || state.round !== ctx.round)) return halt('scope', 'the scope step reported ' + sc.status + ', and the state holds no doors for round ' + ctx.round)
  // The round whose doors the items run over: this round's own scope — or, for a re-run,
  // the round its position names. WHICH doors is no read of this script's: a unit's prompt
  // names the read that prints them, from that round (`doorsRead`).
  const doorsOf = rerun ? rerun.round : ctx.round
  const selected = state.items.filter(runs)
  // A held check is run by no chain and by no preflight: the tool holds its command.
  const held = selected.filter((i) => heldKind(i.kind))
  const hunting = selected.filter((i) => i.kind !== CHECK_KIND && !heldKind(i.kind))
  const late = selected.filter((i) => i.kind === CHECK_KIND && !always.some((x) => x.item === i.item))
  const unknownKinds = hunting.filter((i) => !CHAINS[i.kind]).map((i) => i.item + ' (' + i.kind + ')')
  if (unknownKinds.length) return halt('state', 'this round runs item(s) of a kind the harness has no chain for: ' + unknownKinds.join(', ') + ' — a kind is added by adding its chain to CHAINS, never by guessing one', { launched: launch.names })
  // THE HELD CHECKS — each ONE long command the tool starts, holds and JUDGES: one after
  // another, after the candidate's gate and never beside it. A held gate is answered from
  // the candidate's own gate, which is not run twice — and in a re-run not at all. What
  // comes back is how each command ended (`heldOf`): a verdict of the tool's is the
  // check's result, by its file; any other end leaves the check VOID BY A WORD, and the
  // stage goes on. A HELD CHECK IS NAMED BY THE CANDIDATE'S COMMIT AND THE ITEM'S OWN RUN IN
  // THE ROUND (`runOf`): its first, in the stage that begins the round — whichever attempt
  // of that stage this is — and in a re-run the run the state's position names.
  const runOf = (i) => (rerun ? (rerun.items.find((x) => x.item === i.item) || {}).attempt : 1)
  const heldRan = []
  for (const i of held) {
    const kind = heldKind(i.kind)
    if (kind === 'gate') {
      heldRan.push(Object.assign({ item: i.item }, pre.gate || { ends: 'not-run', kind, why: 'off-tree' }))
      continue
    }
    heldRan.push(Object.assign({ item: i.item }, await hold({ name: i.item + '-' + sha.slice(0, 12) + '-a' + runOf(i), stands: true, kind, flags: regressionFlags(previousOf(state).commit, sha), what: 'the held check `' + i.item + '`: the regression set, the previous release against the candidate', phase: 'Preflight and scope' })))
  }
  for (const h of heldRan.filter((x) => !HELD_TAKES.check.includes(x.ends))) log('VOID BY A WORD: the held check `' + h.item + '` — ' + (h.ends === 'not-run' ? 'a gate reads the working tree, which is no longer the round\'s candidate: it is not run again inside its round' : heldEnded(h) + (h.why ? ' (`' + h.why + '`)' : '')) + '. It left no verdict, and is recorded void — `' + heldVoid(h) + '` — a run that counts')
  const needsImage = hunting.some((i) => chainOf(i.kind, false).some((stepList) => stepList.some((s) => s.image)))
  let checks = pre.result.checks || []
  // The second preflight: the trial image, and the checks the scope selected. One that did
  // not provide them halts nothing — what it was asked for is VOID with that reason: a
  // trial arm is not launched without an image, and every other item goes on.
  let secondName = null
  let unprovided = null
  if (needsImage || late.length) {
    const more = await preflightOf(ctx, launch, ['preflight', 'second'], { image: needsImage, sha, label, branch: loopBranch, checks: late, crossModel: false, tested: moved }, 'Preflight and scope')
    secondName = more.name
    if (more.fault) {
      unprovided = 'the second preflight did not provide it — ' + more.fault
      log('NOT PROVIDED by the second preflight (' + more.fault + '): ' + (needsImage ? 'no trial arm runs, and ' : '') + late.length + ' check(s) are void — every other item goes on')
    } else {
      built.image = needsImage ? more.result.image : null
      checks = checks.concat(more.result.checks || [])
    }
  }
  const imageless = (i) => !!unprovided && chainOf(i.kind, false).some((stepList) => stepList.some((s) => s.image))
  log('scope ' + sc.status + ': ' + (rerun ? 'the doors of round ' + doorsOf : state.doors.included + ' door(s)') + ' inside; ' + hunting.length + ' of ' + state.items.filter((i) => i.kind !== CHECK_KIND).length + ' hunting item(s) run ' + (rerun ? 'again' : 'this round') + ', and ' + (always.length + late.length) + ' check(s)')
  if (sc.uncovered && sc.uncovered.length) log('NOT COVERED: ' + sc.uncovered.length + ' door(s) of the round\'s test set are reached by no item — ' + sc.uncovered.join(' · '))
  if (v.stopAfter === 'preflight') return { status: 'stopped', after: 'preflight', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, previous: built.previous, gate: pre.gate ? heldSaid(pre.gate) : null, held: heldRan.map((h) => Object.assign({ item: h.item }, heldSaid(h))), checks, scope: { status: sc.status, uncovered: sc.uncovered || [], reached_but_excluded: sc.reached_but_excluded || [] }, reporters: launch.names, named: namedOf(state, doorsOf), arrived }

  // The round's instruments.
  phase('Instruments')
  const crossTool = crossNamed.length > 0 && (pre.result.checks || []).some((c) => c.check === CROSS_CHECK && c.status === 'green')
  const crossVoid = crossNamed.filter((id) => !crossTool || !hunting.some((i) => i.item === id)).map((id) => ({ item: id, why: crossTool ? 'the item does not run in this round' : 'the tool of the cross-model pass did not answer on this machine' }))
  for (const lost of crossVoid) log('CROSS-MODEL PASS VOID for `' + lost.item + '`: ' + lost.why + ' — its other passes run regardless')
  // How many doors of that round a unit covers is the digest's; the doors are the read's.
  const covers = (i) => (rerun ? (rerun.items.find((x) => x.item === i.item) || {}).doors : i.doors)
  const units = hunting.filter((i) => !imageless(i)).map((i) => ({ item: i.item, kind: i.kind, clause: i.clause, chain: chainOf(i.kind, crossTool && crossNamed.includes(i.item)), doors: covers(i), round: doorsOf, range: null }))
  const nameable = unitNames(units, launch.names)
  if (nameable.fault) return haltAfter('names', nameable.fault)
  const ran = await runUnits(ctx, launch, built, units, 'Instruments')
  const reporters = ran.reduce((all, u) => all.concat(u.reporters), [])
  // WHO DROVE AND DID NOT ASSERT THE CANDIDATE'S BINARY — no hash returned, or another.
  // Each one's item is void already, with the reporter and the field in its reason
  // (`runUnits`, by NEEDS), and nothing of the stage waits on it: the return names them.
  const unassertedBy = hashMismatch(built.candidate.sha256, reporters)
  // THE REPORTS, CHECKED: what is on disk decides, never what an agent returned. The
  // reporters the stage stands on — its marker, the preflight, the scope step — left one, or
  // it halts; an instrument's step that left none voids its item, and the stage goes on.
  const launchedSoFar = launch.names.slice()
  const settled = await settleReports(ctx, launch, 'check-reports', 'Instruments')
  if (settled.fault) return haltAfter('report-check', settled.fault, { transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null, launched: launchedSoFar })
  // THE ROUND'S SCOPE, SET ASIDE: a file at its path that the record script would not have
  // written is no scope, and it has left the tree — so the round has none, and nothing an
  // instrument found over its doors can be recorded against one.
  const scopeFile = runDir(v.run) + '/r' + ctx.round + '/scope.md'
  const noScope = settled.aside.find((a) => plain(a) && a.path === scopeFile)
  if (noScope) return haltAfter('scope-aside', 'the round\'s scope (' + scopeFile + ') was set aside by the report check: the file at its path is not what `dev/stabilize-record` would have written (`' + noScope.why + '`), so round ' + ctx.round + ' has no scope any more, and what this attempt\'s instruments found cannot be recorded against one', { launched: launchedSoFar, aside: settled.aside })
  const stoodOn = [ATTEMPT, pre.name, scopeName].concat(secondName && !unprovided ? [secondName] : []).filter((name) => settled.missing.includes(name))
  if (stoodOn.length) return haltAfter('stood-on', 'a reporter this stage stands on ' + (stoodOn.some((name) => setAside(settled.aside, name)) ? 'left a file at its report\'s path that is no report, and it was set aside' : 'returned and left no report') + ': ' + stoodOn.join(', ') + ' — what it established has nothing on record behind it', { launched: launchedSoFar, aside: settled.aside })
  unreported(ran, settled.missing, settled.aside)
  for (const u of ran) {
    if (u.status === 'void') log('VOID: `' + u.item + '` — ' + u.reason + '. Its result is recorded as void, and the state asks for its re-run')
    if (u.crossModel === 'void') crossVoid.push({ item: u.item, why: 'the pass was launched and did not report' })
  }
  const crossRan = ran.filter((u) => u.crossModel === 'ran').map((u) => u.item)
  if (v.stopAfter === 'instruments') return { status: 'stopped', after: 'instruments', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, units: ran.map((u) => ({ item: u.item, status: u.status, findings: u.reporters.reduce((n, r) => n + (r.result && r.result.findings ? r.result.findings.length : 0), 0) })), reporters: launch.names }

  // WHAT EACH ITEM THIS INVOCATION RAN DID — one result per item, which the record step
  // records. It is read off what the instruments, the preflights and the held commands left,
  // and it stands BEFORE the triage: in a run with a queue every red one is a source of it.
  // BESIDE EACH VOID'S SENTENCE STANDS ITS WORD (`word`: VOID_WORDS, `heldVoid`), beside a
  // check an agent read THE PREFLIGHT THAT READ IT (`by`), and beside a held check that left
  // a verdict ITS JOB (`job`): what the record of a run with a queue is composed from.
  const unitStatus = ran.map((u) => ({ item: u.item, status: u.status, reason: u.reason, word: u.word })).concat(hunting.filter(imageless).map((i) => ({ item: i.item, status: 'void', reason: 'no trial image: ' + unprovided, word: voidWord('no-image', secondName) })))
  // A check that is due again once the round's record lies on its candidate was not asked
  // for (above): it is void, with that reason — never run on the tip and recorded as the
  // candidate's.
  const offTree = moved ? 'the round\'s candidate ' + sha + ' is no longer the tree checked out — the round\'s record commits lie on it — and a check reads the working tree: it was not run, because what it would show is evidence about no commit of this round' : null
  for (const item of always.concat(late)) {
    const c = offTree ? null : checks.find((x) => x.check === item.item)
    // A green or a red is evidence about exactly the commit the check ran on: one that
    // ran on another commit, or does not say which, did not run on the candidate.
    const elsewhere = c && c.status !== 'void' && c.commit !== sha ? 'the check ran on ' + (c.commit ? 'the commit ' + c.commit : 'a commit the preflight did not name') + ', not on the candidate' : null
    // The preflight that read it: the second's for a check the scope selected.
    const by = late.includes(item) ? secondName : pre.name
    unitStatus.push({ item: item.item, status: !c || elsewhere ? 'void' : c.status, reason: elsewhere || offTree || (c ? c.evidence || 'the check could not run, and the preflight returned no reason' : late.includes(item) && unprovided ? unprovided : 'the preflight returned nothing for it'), evidence: c && c.evidence ? c.evidence + ' — the preflight\'s report: ' + pre.result.report : null, by, word: offTree ? voidWord('off-tree') : voidWord(elsewhere ? 'other-commit' : c ? 'could-not-run' : late.includes(item) && unprovided ? 'not-provided' : 'unanswered', by) })
  }
  // A HELD CHECK'S RESULT IS THE FILE THE TOOL KEPT ITS VERDICT IN, and no word: green, red
  // or void as that file says. ONE THAT LEFT NO VERDICT IS VOID BY A WORD — the one word a
  // caller may say of a held check — so that it has a run on record and the state can
  // leave `retest`: dead, its start refused, still running when the stage stopped asking,
  // or a gate that is not held off the candidate's tree.
  for (const h of heldRan) unitStatus.push(HELD_TAKES.check.includes(h.ends) ? { item: h.item, status: h.ends, verdict: h.verdict, job: h.name } : { item: h.item, status: 'void', reason: heldVoid(h), word: heldVoid(h) })

  // Triage of every finding; verify-real; the forks.
  phase('Triage')
  const sources = reporters.filter((r) => r.result && r.result.findings).map((r) => ({ reporter: r.name, report: r.report, findings: findingLines(r.result.findings) }))
  if (sc.left_open && sc.left_open.length) sources.push(leftOpenSource(scopeName, sc.report, sc.left_open))
  // A RUN WITH A QUEUE TAKES ITS OWN PATH FROM HERE (the header: A RUN WITH A QUEUE TAKES
  // ANOTHER PATH; the reshape's plan, T5a and T5b). THE STAGE HAS ONE TRIAGE, handed every
  // source: the reporters' findings and what the scope step left open (above), EVERY RED
  // RESULT OF THIS INVOCATION under the key the record files its row with — a held check's
  // evidence the file the tool kept its verdict in, any other's the report of the preflight
  // that read it — and the rows nobody graded. THEN THE STAGE'S FIRST RECORD, BEFORE ANYTHING
  // IS VERIFIED: composed by the tool from what was filed and from plain words — the red
  // item's result and the triage's reporter in ONE batch, so that the row a red result is
  // filed as is graded by the entry that names it — then gated, committed and pushed as
  // every record is. NO BATCH IS TYPED, NO EXECUTOR IS LAUNCHED, AND NOTHING IS VERIFIED
  // BEFORE THAT RECORD. THEN THE VERIFY PHASE, AND THE SECOND RECORD (`verifyWaves`): no pass
  // of a triage and no advocate. EVERY WAY OUT OF THIS BLOCK IS A RETURN: nothing below it
  // runs for a run with a queue.
  if (hasQueue(state)) {
    const reds = unitStatus.filter((u) => u.status === 'red').map((u) => {
      const h = heldRan.find((x) => x.item === u.item)
      return Object.assign({ item: u.item, clause: (selected.find((i) => i.item === u.item) || {}).clause }, h ? { verdict: h.verdict, output: h.output } : { report: testReport(ctx, u.by) })
    })
    const once = await triageOnce(ctx, launch, sources.concat(redSources(reds), ledgerSource(v.run, state)))
    if (once.fault) return haltAfter('triage', once.fault, { transient: !!once.transient, halt: once.halt || null, step: once.step || null })
    if (v.stopAfter === 'triage') return { status: 'stopped', after: 'triage', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, triage: once.triage, findings_in: once.handed, reporters: launch.names, named: namedOf(state, doorsOf), arrived }
    // THE FIRST RECORD. Its words: the reporters that left a report — the last report check
    // took every other off the list — the candidate, its gate's job, what each item did, and
    // the round's facts; a re-run writes no fact of the round and names no gate.
    phase('Record')
    const facts = rerun ? [] : [['candidate', sha], ['binary', built.candidate.sha256]].concat(sc.base && SHORT_SHA_RE.test(sc.base) ? [['base', sc.base]] : [], crossNamed.length ? [['cross-model', crossRan], ['cross-model-void', crossVoid.map((x) => x.item)]] : [])
    const words = recordWords({ reporters: launch.names, commit: sha, gate: pre.gate ? pre.gate.name : null, did: unitStatus, facts })
    const composed = await composeRecord('test:r' + ctx.round, loopBranch, ctx, v.scratch + '/record/test-r' + ctx.round + '-a' + ctx.attempt, 'docs(record): ' + subjectOf(v.run, ctx.round, rerun ? 'a re-run of ' + rerun.clause : 'the findings of the test stage'), words)
    if (composed.fault) return recordHalt('record', composed, { refusal: composed.refusal || null, launched: launch.names, triage: once.triage })
    const recorded = await recordStep('test:r' + ctx.round, loopBranch, composed.rec)
    if (recorded.fault) return recordHalt('record', recorded, { launched: launch.names, triage: once.triage })
    const pushed = await pushRecord('push', loopBranch)
    if (!onIt(pushed, loopBranch, true)) return pushHalt(pushed, loopBranch, 'the stage\'s first record is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
    const last = await readState()
    if (!last.state) return haltAfter('read-back', 'the stage\'s first record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + last.error, { transient: !!last.transient, step: last.step || null })
    state = last.state
    // THEN THE STAGE VERIFIES, IN WAVES, AND RECORDS A SECOND TIME (`verifyThenRecord`; the
    // header; the reshape's plan, T6a): what the queue prints, while the budget has verdicts
    // left — the queue read again after every wave — and then what was filed.
    const closing = await verifyThenRecord(ctx, launch, built, state, { commit: recorded.result.commit, says: 'the stage\'s first record is committed and pushed (' + recorded.result.commit + ')' })
    if (closing.halted) return closing.halted
    const second = closing.second
    state = closing.state
    // WHAT THE STAGE RETURNS (the reshape's plan, T7): WHAT IT DID, IN ONE WORD (`statusOf`) —
    // `hunted`: its findings are on record, graded, and it made no record of a verify phase;
    // `verified`: its second record was made. `record` is the first record's commit, and
    // `verify` what the phase did, AS COUNTS: how many waves and launches, HOW MANY VERDICTS
    // THE STATE COUNTS SINCE THE FIRST RECORD (the script's count, never this one's), the
    // second record's commit, HOW MANY findings a launch left without a verdict, how many
    // returns were not held and how many say contested — each of those a line of the log,
    // by its key — how many calls returned nothing, and the wave their silence made the
    // last. `next` is the state's, verbatim, and BESIDE IT what the record script counts, as
    // the digest has it (`nextOf`, `queueSaid`): the budget, the queue's length class by
    // class, the hit rate and the stop's view by its path and its hash, and what the stage
    // is continued for.
    // AND NO LIST OF THIS SCRIPT'S OWN MAKING: no entry, no key, no grade, no verdict and no
    // basis — and WHAT IT LAUNCHED, WHAT ITS AGENTS RETURNED AND WHAT IT VOIDED ARE HOW MANY:
    // the items it voided and the reporters that did not assert the binary are named in the
    // log, each with its word, and are on record as void results; how each held check ended
    // is the log's and the record's; the doors nobody covers and the doors reached and left
    // out are the round's scope's; a re-run names its clause, its round and how many items;
    // and a cross-model pass is counted. The lists a return holds are the step tool's — a
    // digest's, each bounded by its shape — and the position's words.
    const voided = unitStatus.filter((u) => u.status === 'void')
    if (voided.length) log('VOID, AND ON RECORD AS VOID: ' + voided.map((u) => '`' + u.item + '` (`' + (u.word || u.reason) + '`)').join(', ') + ' — the state asks for the re-run of each where it owes one')
    if (heldRan.length) log('the held checks: ' + heldRan.map((h) => '`' + h.item + '` ' + (h.ends === 'not-run' ? 'not run' : 'ended ' + h.ends) + (h.name ? ' (the held command `' + h.name + '`' + (h.output ? ', its output ' + h.output : '') + ')' : '')).join('; '))
    return Object.assign({
      status: statusOf([], second),
      stage: 'test',
      run: v.run,
      round: ctx.round,
      rerun: rerun ? { clause: rerun.clause, round: rerun.round, items: rerun.items.length } : null,
      cross_model: { named: crossNamed.length, ran: crossRan.length, void: crossVoid.length },
      candidate: built.candidate,
      record: recorded.result.commit,
      triage: once.triage,
      verify: closing.verify,
      counts: { items: state.items.length, items_run: hunting.length + always.length + late.length + held.length, reporters: launch.names.length, findings_in: once.handed, blockers: state.blockers, for_the_human: state.human_list, voided: voided.length, unasserted: unassertedBy.length, held: heldRan.length },
      scope: { status: sc.status, uncovered: (sc.uncovered || []).length, reached_but_excluded: (sc.reached_but_excluded || []).length },
      forbids_close: state.forbids_close,
    }, nextOf(state), rerun ? { named: namedOf(state, rerun.round) } : {})
  }
  // And the rows whose triage nobody finished — seeded at the opening, or left without a
  // verdict by an earlier stage: this triage is the next one that runs.
  for (const source of ledgerSource(v.run, state)) sources.push(source)
  const tri = await triagePasses(ctx, launch, built, sources, [])
  if (tri.fault) return haltAfter('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, step: tri.step || null })
  if (v.stopAfter === 'triage') return { status: 'stopped', after: 'triage', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, entries: tri.entries, forks: tri.forks, reporters: launch.names, named: namedOf(state, doorsOf), arrived }

  // The record step: the reports checked, what each item did, the rows, the round's facts.
  // ONE RESULT PER ITEM THIS INVOCATION RAN — the record script derives every clause's
  // status from them, and files a red check as a finding in the same call.
  phase('Record')
  const rec = triageRecord(ctx, tri.entries)
  // A re-run writes no fact of the round: the round is tested, and its record stands.
  const facts = rerun ? null : { candidate: sha, binary: built.candidate.sha256 }
  if (crossNamed.length) Object.assign(facts, { 'cross-model': crossRan, 'cross-model-void': crossVoid.map((x) => x.item) })
  if (facts && sc.base && SHORT_SHA_RE.test(sc.base)) facts.base = sc.base
  const dir = v.scratch + '/record/test-r' + ctx.round + '-a' + ctx.attempt
  const commands = stageRecordCommands(ctx, { reporters: launch.names, gate: pre.gate ? { commit: sha, file: pre.gate.output } : null, results: { commit: sha, rows: resultRows(unitStatus) }, rows: rec.rows, triage: rec.triage, patches: [], facts, keys: tri.entries.map((e) => e.key) })
  const recorded = await recordStep('test:r' + ctx.round, loopBranch, recordPrompt(v, rerun ? 'the record of a re-run inside round ' + ctx.round + ' — ' + rerun.items.length + ' item(s) of clause `' + rerun.clause + '` run again, ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s)' : 'the record of round ' + ctx.round + '\'s test stage — ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s)', loopBranch, dir, rerun ? state.candidate.round : ctx.round, commands, subjectOf(v.run, ctx.round, rerun ? 'a re-run of ' + rerun.clause : 'the record of the test stage')))
  if (recorded.fault) return recordHalt('record', recorded, { launched: launch.names, forks: tri.forks })
  const pushed = await pushRecord('push', loopBranch)
  if (!onIt(pushed, loopBranch, true)) return pushHalt(pushed, loopBranch, 'the record is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
  const last = await readState()
  if (!last.state) return haltAfter('read-back', 'the record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + last.error, { transient: !!last.transient, step: last.step || null })
  state = last.state

  const out = nextOf(state)
  const byGrade = {}
  for (const e of tri.entries) byGrade[e.verdict || e.grade] = (byGrade[e.verdict || e.grade] || 0) + 1
  return Object.assign({
    status: 'triaged',
    stage: 'test',
    run: v.run,
    round: ctx.round,
    rerun,
    cross_model: { named: crossNamed, ran: crossRan, void: crossVoid },
    candidate: built.candidate,
    record: recorded.result.commit,
    counts: { items: state.items.length, items_run: hunting.length + always.length + late.length + held.length, reporters: launch.names.length, findings_in: tri.handed, entries: tri.entries.length, by_grade: byGrade, blockers: state.blockers, for_the_human: state.human_list, voided: unitStatus.filter((u) => u.status === 'void').map((u) => u.item) },
    forks: tri.forks,
    unverified: tri.unverified,
    // Who drove and did not assert the candidate's binary: each one's item is void.
    unasserted: unassertedBy,
    // How each held check ended — the tool's word, the job and the file its output is in.
    held: heldRan.map((h) => Object.assign({ item: h.item }, heldSaid(h))),
    // What the scope step's agent read; what the record script computed is `named`.
    scope: { status: sc.status, uncovered: sc.uncovered || [], reached_but_excluded: sc.reached_but_excluded || [] },
    forbids_close: state.forbids_close,
  }, out, rerun ? { named: namedOf(state, rerun.round) } : {})
}

// ---- the `fix` stage ----
// WHAT OF IT IS REACHED TODAY is its first lines, down to `ruleTheRun`: the stage refuses to
// start (NOT_FIT), and the one invocation of it that is let through records what the human
// ruled about the run. Those lines read digests, as every step does.
// OWED TO THE `fix` HALF'S REPAIR (the second repair plan, section 9, row 7) — and said here
// so that nobody reads the rest as converted: BELOW `ruleTheRun` THIS FUNCTION STILL READS
// THE STATE DOCUMENT IT IS NO LONGER HANDED. Its steps are asked for their digests like
// every other, so it parses the lines it parsed; but `state.ledger` (the rows of the
// blockers, of a dropped round, of a part), `state.blockers` and `state.human_list` as
// lists, `state.doors.included` / `.excluded` as lists, `rounds[].facts.cycles`, a fixer's
// findings by their door and repro (`fixerPrompt`, `areasOf`), `landed.moved_outside` as a
// list, and a record's own `--head` are what its repair reads by key or from the digest — a
// blocker's row has no read yet (DECISIONS.md -> 2026-10-07, "A line holds what the harness
// acts on", item 6). AND ITS ROUND-TIP BINARY IS STILL ASKED OF A PREFLIGHT, which builds
// none any more (the second repair plan's task K11): that binary, its fixers' gates and the
// fixer's prompt are its repair's to hold by the tool's acts. Nothing below is reachable
// until that repair lifts the refusal.
async function runFix() {
  phase('State')
  const gs = await toolStep('state', 'State', 'git-state', gitStatePrompt(v))
  if (!gs || gs.status !== 'ready') return gitHalt('git', gs, 'the tree or the branches are not in the state the `fix` stage starts from')
  arrived = { found: gs.found || [], finished: gs.finished || [], owed: gs.owed || [] }
  stageHead = String(gs.head)
  stageFence = SHA256_RE.test(String(gs.fence)) ? String(gs.fence) : null
  if (gs.pending) return await finishRecord(gs)
  let read = await readState()
  if (!read.state) return halt('state', 'the run\'s state could not be read: ' + read.error, { transient: !!read.transient, halt: read.halt || null, step: read.step || null })
  let state = read.state
  if (!state.opened) return halt('state', 'the run `' + v.run + '` has no opening record (' + runDir(v.run) + '/opening.md)')
  if (v.rulings && v.rulings.every(runRuling)) return await ruleTheRun(state, gs.branch)
  // The `fix` stage's own records follow its fixers' commits: which head each is held to is
  // its repair's, and none is named here.
  stageHead = null
  stageFence = null
  if (state.position.fix.refused) return { status: 'refused', stage: 'fix', run: v.run, refused: state.position.fix, message: 'the `fix` stage is refused (' + state.position.fix.refused + ', round ' + state.position.fix.round + '): ' + refusalOf(state.position.fix) + '. Nothing was run beyond the two reads.', next: state.next, not_ready: state.not_ready }
  const round = state.position.fix.round

  // The round's branch: the highest cut that exists, or none yet.
  const found = await toolStep('find:r' + round, 'State', 'find-round', findRoundPrompt(v, round))
  if (!found || found.status !== 'ready') return gitHalt('git', found, 'the round\'s branches could not be listed')
  const local = found.local || []
  const stray = (found.remote || []).filter((b) => currentCut(v.run, round, [b]) && !local.includes(b))
  if (stray.length) return halt('git', 'origin has a branch of round ' + round + ' that is not here: ' + stray.join(', ') + ' — fetch it; this stage does not guess which branch the round is on')
  let cut = currentCut(v.run, round, local)
  let branch = cut ? branchName(v.run, round, cut) : loopBranch
  let checkedOut = found.branch
  let tipSha = String(found.head || '')
  // onBranch — the round's branch checked out: switched to, or opened from the loop branch.
  async function onBranch(target) {
    if (checkedOut === target) return null
    const ob = await toolStep('open:' + target, 'State', 'open-round', openRoundPrompt(v, round, target))
    if (!onIt(ob, target, false)) return gitHalt('git', ob, 'the branch ' + target + ' could not be checked out')
    checkedOut = target
    tipSha = String(ob.head)
    return null
  }
  if (checkedOut !== branch) {
    if (!cut) return halt('git', 'round ' + round + ' has no branch yet, and `' + checkedOut + '` is checked out — not the loop branch ' + loopBranch)
    const stopped = await onBranch(branch)
    if (stopped) return stopped
    read = await readState()
    if (!read.state) return halt('state', 'the run\'s state could not be read on ' + branch + ': ' + read.error, { transient: !!read.transient })
    state = read.state
    if (state.position.fix.refused) return { status: 'refused', stage: 'fix', run: v.run, refused: state.position.fix, message: 'on `' + branch + '` the `fix` stage is refused (' + state.position.fix.refused + ').', next: state.next }
  }
  if (v.stopAfter === 'state') return { status: 'stopped', after: 'state', stage: 'fix', run: v.run, round, branch, state: attached(state) }

  // The human's rulings: ONE step, before anything is fixed.
  if (v.rulings) {
    const ruled = await rulingsStep(round, branch, {})
    if (ruled.halted) return ruled.halted
    state = ruled.state
  }
  if (v.stopAfter === 'rulings') return { status: 'stopped', after: 'rulings', stage: 'fix', run: v.run, round, branch, state: attached(state) }

  const report = { stage: 'fix', run: v.run, round }
  const blockersOf = (s) => s.ledger.filter((row) => s.blockers.includes(row.key))
  const cyclesDone = () => state.rounds.find((r) => r.round === round).facts.cycles
  const doorsForRetest = []
  const forks = []
  const couldNot = []

  // The exit `drop`: the round's record says so, its fixes are open again, and its record
  // commits — exactly those — are carried over to the loop branch.
  async function drop() {
    phase('Record')
    let commits = { records: [], fixes: [], mixed: [] }
    if (cut) {
      const listed = await toolStep('commits:r' + round, 'Record', 'round-commits', roundCommitsPrompt(v, round, branch))
      if (!listed || listed.status !== 'listed') return gitHalt('git', listed, 'the round\'s commits could not be listed')
      commits = classifyCommits(listed.all || [], listed.inside || [], listed.outside || [])
      if (commits.mixed.length) return halt('git', 'commit(s) of the round touch the run\'s directory and other paths at once, or nothing: ' + commits.mixed.join(', ') + ' — no carry-over can take them, and none is guessed')
    }
    const ctx = { run: v.run, round, stage: 'fix', cycle: state.position.fix.cycle, attempt: state.position.fix.attempt, scratch: v.scratch }
    const patches = reopenPatches(state.ledger, commits.fixes)
    const commands = stageRecordCommands(ctx, { reporters: [], rows: [], triage: [], patches, clauses: [], facts: { outcome: 'dropped' }, keys: patches.map((p) => p.key) })
    const recorded = await recordStep('drop:r' + round, branch, recordPrompt(v, 'round ' + round + ' is DROPPED, by the human\'s ruling at a bound — ' + patches.length + ' finding(s) whose fix does not land are open again', branch, v.scratch + '/record/drop-r' + round, round, commands, v.run + ' r' + round + ' — the round is dropped'))
    if (recorded.fault) return halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null })
    phase('Land')
    let carried = []
    if (cut) {
      const pushedRound = await toolStep('push:' + branch, 'Land', 'push', pushPrompt(v, branch))
      if (!onIt(pushedRound, branch, true)) return gitHalt('push', pushedRound, 'the dropped round\'s branch was not pushed')
      const take = commits.records.concat([recorded.result.commit])
      const carry = await toolStep('carry:r' + round, 'Land', 'carry', carryPrompt(v, round, take, null))
      if (!carry || carry.status !== 'carried' || carry.branch !== loopBranch || carry.remote_head !== carry.head || (carry.picked || []).length !== take.length) return gitHalt('carry', carry, 'the dropped round\'s record commits were not carried over to ' + loopBranch)
      carried = carry.picked
    } else {
      const pushed = await toolStep('push', 'Land', 'push', pushPrompt(v, loopBranch))
      if (!onIt(pushed, loopBranch, true)) return gitHalt('push', pushed, 'the drop is recorded on ' + loopBranch + ' and the branch was not pushed')
    }
    const after = await readState()
    if (!after.state) return halt('state', 'round ' + round + ' is dropped and its record is on ' + loopBranch + ', and the state could not be read back: ' + after.error, { transient: !!after.transient })
    return Object.assign({ status: 'dropped', dropped_branch: cut ? branch : null, reopened: patches.map((p) => p.key), carried, blockers: after.state.blockers, human_list: after.state.human_list }, report, nextOf(after.state))
  }
  if (v.exit === 'drop') return await drop()

  // The second half of a cycle, from the fixers' return on: the round's tip pushed and
  // built; per fork of the fixers an advocate and an independent drive — or, when there is
  // none, the audit of the fix diff; then triage, verify-real, the record on the round's
  // branch, the push, and the state read back. A cycle with a fork, and a cycle that fixed
  // nothing, is not audited in this invocation and is not counted: there is no new diff.
  async function secondHalf(ctx, launch, fixerSources, patches, fixerForks, doors, isPart) {
    phase('Fix')
    const head = await toolStep('push:' + branch + ':c' + ctx.cycle, 'Fix', 'push', pushPrompt(v, branch))
    if (!onIt(head, branch, true)) return { halted: gitHalt('push', head, 'the round\'s branch ' + branch + ' was not pushed') }
    // The fixers' own, and the rows whose triage nobody finished: this triage is the next.
    const sources = fixerSources.concat(ledgerSource(v.run, state))
    const driven = []
    const range = loopBranch + '..' + branch
    const audited = !fixerForks.length && (isPart || patches.length > 0)
    let built = null
    if (audited || fixerForks.length || sources.length) {
      const label = 'r' + round + 'c' + ctx.cycle
      const binary = v.scratch + '/bin/' + label + '.a' + ctx.attempt + '/jigc'
      const pre = await preflightOf(ctx, launch, ['preflight'], { build: true, image: false, sha: head.head, label, branch, binary, checks: [], crossModel: false, previous: previousOf(state) }, 'Fix')
      if (pre.fault) return { halted: halt('preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, branch }) }
      built = { candidate: { label, sha: head.head, binary, sha256: pre.result.candidate.sha256 }, previous: pre.result.previous, image: null }
    }
    if (fixerForks.length) {
      log('cycle ' + ctx.cycle + ': ' + fixerForks.length + ' fork(s) of the fixers — the fix diff is NOT audited in this invocation, and the cycle is not counted')
      for (const fork of fixerForks) {
        const d = await driveFork(ctx, launch, built, fork, 'fix')
        if (d.unheld) return { halted: halt('binary', 'the fork of `' + fork.key + '`: ' + d.unheld, { branch }) }
        driven.push(d.fork)
        for (const source of d.leftOpen) sources.push(source)
      }
    } else if (!audited) {
      log('cycle ' + ctx.cycle + ': NOTHING WAS FIXED — there is no new diff to audit, and the cycle is not counted')
    } else {
      const unit = { item: 'audit', clause: null, chain: FIX_AUDIT, doors, range, brief: 'The fix diff of round ' + round + ' as it stands at ' + head.head + ': every commit of `' + range + '` over the product paths. Whether each fix closes its finding\'s class, and what it breaks beside it.' }
      const ran = (await runUnits(ctx, launch, built, [unit], 'Fix'))[0]
      const wrong = hashMismatch(built.candidate.sha256, ran.reporters)
      if (wrong.length) return { halted: halt('binary', 'a driving agent of the audit did not assert the round tip\'s binary (' + built.candidate.sha256 + '): ' + wrong.map((w) => w.reporter + ' asserted ' + JSON.stringify(w.asserted)).join('; '), { mismatched: wrong, branch }) }
      if (ran.status !== 'green') return { halted: halt('audit', 'the audit of the fix diff did not run to its end (' + (ran.reporters.filter((r) => !r.result || r.result.status !== 'reported').map((r) => r.name).join(', ') || 'no reporter returned') + '): a round is not landed, and a cycle is not counted, on half an audit', { transient: ran.reporters.some((r) => !r.result), launched: launch.names, branch }) }
      for (const r of ran.reporters) {
        sources.push({ reporter: r.name, report: r.report, findings: findingLines(r.result.findings) })
        for (const d of r.result.doors_affected || []) if (!doorsForRetest.includes(d)) doorsForRetest.push(d)
      }
    }
    if (v.stopAfter === 'audit') return { halted: Object.assign({ status: 'stopped', after: 'audit', branch, candidate: built ? built.candidate : null, reporters: launch.names, sources: sources.map((x) => ({ reporter: x.reporter, findings: x.findings.length })), forks: driven }, report) }
    phase('Triage')
    const tri = await triagePasses(ctx, launch, built, sources, [])
    if (tri.fault) return { halted: halt('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, branch }) }
    phase('Record')
    const rec = triageRecord(ctx, tri.entries)
    const keys = tri.entries.map((e) => e.key)
    for (const patch of patches) if (!keys.includes(patch.key)) keys.push(patch.key)
    const commands = stageRecordCommands(ctx, { reporters: launch.names, rows: rec.rows, triage: rec.triage, patches, clauses: [], facts: audited ? { cycles: ctx.cycle } : null, keys })
    const recorded = await recordStep('fix:r' + round + ':c' + ctx.cycle, branch, recordPrompt(v, 'the record of round ' + round + '\'s fix cycle ' + ctx.cycle + ' — ' + launch.names.length + ' report(s), ' + patches.length + ' ledger patch(es), ' + tri.entries.length + ' finding(s) triaged' + (audited ? '' : '; the fix diff was not audited, and the cycle is not counted'), branch, v.scratch + '/record/fix-r' + round + '-c' + ctx.cycle + '-a' + ctx.attempt, round, commands, v.run + ' r' + round + ' c' + ctx.cycle + ' — the fix cycle\'s record'))
    if (recorded.fault) return { halted: halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null, launched: launch.names, branch }) }
    const pushed = await toolStep('push:' + branch + ':c' + ctx.cycle + ':record', 'Record', 'push', pushPrompt(v, branch))
    if (!onIt(pushed, branch, true)) return { halted: gitHalt('push', pushed, 'cycle ' + ctx.cycle + ' is recorded on ' + branch + ' (' + recorded.result.commit + ') and the branch was not pushed') }
    const after = await readState()
    if (!after.state) return { halted: halt('state', 'cycle ' + ctx.cycle + ' is recorded (' + recorded.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, branch }) }
    state = after.state
    for (const fork of driven.concat(tri.forks)) forks.push(fork)
    return { audited, fixerForks: driven, record: recorded.result.commit }
  }

  // land — the round's branch merged into the loop branch `--no-ff`, and the loop branch pushed.
  async function land(extra) {
    phase('Land')
    const landed = await toolStep('land:r' + round, 'Land', 'land', landPrompt(v, round, branch))
    const merged = !!landed && landed.status === 'merged' && SHA_RE.test(String(landed.merge_commit || '')) && landed.remote_head === landed.merge_commit
    const before = !!landed && landed.status === 'landed-before' && SHA_RE.test(String(landed.head || ''))
    if (!merged && !before) return gitHalt('land', landed, landed && landed.merge_commit ? 'round ' + round + ' IS merged into ' + loopBranch + ' locally (' + landed.merge_commit + '), and what followed the merge stopped' : 'round ' + round + ' was not landed; it is unlanded on ' + branch)
    const after = await readState()
    if (!after.state) return halt('state', 'round ' + round + ' is landed (' + (landed.merge_commit || landed.head) + '), and the state could not be read back: ' + after.error, { transient: !!after.transient })
    return Object.assign({ status: 'landed', candidate: { label: 'c' + (round + 1), sha: merged ? landed.merge_commit : landed.head }, landed_before: before, landed_branch: branch, cycles: after.state.rounds.find((r) => r.round === round).facts.cycles, counts: { blockers: after.state.blockers.length, for_the_human: after.state.human_list.length, forks: forks.length }, doors_for_retest: doorsForRetest, tip_moved: !!landed.tip_moved, resolved_logs: landed.resolved_logs || [], moved_outside: landed.moved_outside || [], forks }, extra || {}, report, nextOf(after.state))
  }
  // unlanded — a stage that stops with the round not landed: at the bound, or on a fork.
  function unlanded(phaseName, why) {
    return Object.assign(halt(phaseName, why, { branch, cycles: cyclesDone(), limit: cycleLimit, blockers: state.blockers, human_list: state.human_list, forks, could_not_fix: couldNot }), { round }, phaseName === 'bound' ? { exits: { continue: 'args.raise = { cycles: ' + (Math.max(cycleLimit, cyclesDone()) + 1) + ' } — or more; passed on every later `fix` of this round', drop: 'args.exit = \'drop\'', part: 'args.exit = { part: [<the fix commits of ' + branch + ' to keep>] }' } } : {})
  }

  // The exit `part`: the fix commits the human keeps, and the round's record commits, re-cut
  // as a branch tip of their own; then gated — the record step's gate runs on exactly that
  // tree — and audited once more as exactly that diff, and landed if that audit is clean.
  if (v.exit) {
    if (!cut) return halt('git', 'round ' + round + ' has no branch: there is nothing a part could be cut from')
    phase('Fix')
    const listed = await toolStep('commits:r' + round, 'Fix', 'round-commits', roundCommitsPrompt(v, round, branch))
    if (!listed || listed.status !== 'listed') return gitHalt('git', listed, 'the round\'s commits could not be listed')
    const commits = classifyCommits(listed.all || [], listed.inside || [], listed.outside || [])
    if (commits.mixed.length) return halt('git', 'commit(s) of the round touch the run\'s directory and other paths at once, or nothing: ' + commits.mixed.join(', ') + ' — no part can be cut around them, and none is guessed')
    const strangers = v.exit.part.filter((sha) => commits.fixes.filter((f) => sameCommit(f, sha)).length !== 1)
    if (strangers.length) return halt('git', 'args.exit.part names ' + strangers.join(', ') + ', which is not exactly one fix commit of `' + branch + '` — its fix commits are: ' + (commits.fixes.join(', ') || '(none)'))
    const kept = commits.fixes.filter((f) => v.exit.part.some((sha) => sameCommit(f, sha)))
    const left = commits.fixes.filter((f) => !kept.includes(f))
    const take = (listed.all || []).filter((sha) => kept.includes(sha) || commits.records.includes(sha))
    const part = branchName(v.run, round, cut + 1)
    const carry = await gitStep('cut:' + part, 'Fix', carryPrompt(v, round, take, part), CARRY_SCHEMA)
    if (!carry || carry.status !== 'carried' || carry.branch !== part || carry.remote_head !== carry.head || (carry.picked || []).length !== take.length) return gitHalt('carry', carry, 'the part was not cut as ' + part)
    // What the part changes in the ledger: a kept fix names its new commit; a fix left out
    // never lands, so its finding is open again. Those are the human's choice, and are not
    // what the part's audit is asked about: a blocker it must not add is one that is new.
    const reopen = reopenPatches(state.ledger, left)
    const patches = repointPatches(state.ledger, carry.picked).concat(reopen)
    const known = state.blockers.concat(reopen.map((p) => p.key))
    const doors = (state.doors ? state.doors.included.concat(state.doors.excluded) : []).filter((d) => state.ledger.some((row) => row.door === d.door && (row.disposition === 'fixed' || state.blockers.includes(row.key))))
    cut = cut + 1
    branch = part
    checkedOut = part
    const ctx = { run: v.run, round, stage: 'fix', cycle: state.position.fix.cycle, attempt: state.position.fix.attempt, scratch: v.scratch }
    const done = await secondHalf(ctx, launcher(ctx), [], patches, [], doors, true)
    if (done.halted) return done.halted
    const fresh = state.blockers.filter((key) => !known.includes(key))
    const cutAs = { branch: part, kept, left, reopened: reopen.map((p) => p.key) }
    if (fresh.length || state.human_list.length || forks.length) return Object.assign(unlanded('bound', 'the part was audited as exactly its diff, and the audit is not clean: ' + fresh.length + ' new blocker(s)' + (fresh.length ? ' (' + fresh.join(', ') + ')' : '') + ', ' + state.human_list.length + ' item(s) for the human, ' + forks.length + ' fork(s) — it is unlanded, on ' + part), { part: Object.assign({ new_blockers: fresh }, cutAs), record: done.record }, nextOf(state))
    const closing = stageRecordCommands(ctx, { reporters: [], rows: [], triage: [], patches: [], clauses: [], facts: { outcome: 'part' }, keys: [] })
    const said = await recordStep('part:r' + round, part, recordPrompt(v, 'a PART of round ' + round + ' lands, by the human\'s ruling at a bound — ' + kept.length + ' fix commit(s) kept, ' + left.length + ' left out and their finding(s) open again', part, v.scratch + '/record/part-r' + round, round, closing, v.run + ' r' + round + ' — a part of the round lands'))
    if (said.fault) return halt('record', said.fault, { transient: !said.result, halt: said.result ? said.result.halt : null, branch: part })
    return await land({ part: cutAs })
  }

  // The cycles.
  let finished = false
  for (;;) {
    const at = state.position.fix
    if (at.refused) return Object.assign({ status: 'refused', refused: at, message: 'the `fix` stage is refused (' + at.refused + ', round ' + at.round + '): ' + refusalOf(at) + '.' }, report, nextOf(state))
    const open = { branch: cut ? branch : null, cycles: cyclesDone(), blockers: state.blockers, human_list: state.human_list, untriaged: state.untriaged, forks }
    // The round's triage is not finished, and this stage left it: finish it — once — and
    // go on from the state that leaves. No fixer and no auditor runs for it.
    if (at.triage) {
      if (finished) return Object.assign({ status: 'cycle', message: 'round ' + round + '\'s triage was finished in this invocation and the state still names findings without a grade or a verdict: it is not run again on a guess — `next` says what is owed.' }, open, report, nextOf(state))
      if (!cut) return halt('git', 'the state says that the `fix` stage left round ' + round + '\'s triage unfinished, and the round has no branch: there is no tip a verifier could drive')
      const stopped = await onBranch(branch)
      if (stopped) return stopped
      const lap = { run: v.run, round, stage: 'fix', cycle: at.cycle, attempt: at.attempt, scratch: v.scratch }
      const done = await finishTriage(lap, launcher(lap), state, { sha: tipSha, label: 'r' + round + 'c' + at.cycle, tested: false }, branch)
      if (done.halted) return done.halted
      state = done.state
      for (const fork of done.forks) forks.push(fork)
      finished = true
      continue
    }
    if (at.land) {
      if (!cut) return Object.assign({ status: 'nothing-to-fix', message: 'round ' + round + ' has nothing open and no branch: nothing is fixed and nothing lands.' }, open, report, nextOf(state))
      const stopped = await onBranch(branch)
      if (stopped) return stopped
      return await land()
    }
    if (!state.blockers.length) return Object.assign({ status: cut ? 'cycle' : 'nothing-to-fix', message: 'no finding of round ' + round + ' is routed `fix`' + (cut ? ', and the round is not landed: something of it is still open, and it is not a fixer\'s' : '') + ' — `next` says what is owed.' }, open, report, nextOf(state))
    if (state.next !== 'fix') return Object.assign({ status: 'cycle', message: 'round ' + round + ' has ' + state.blockers.length + ' blocker(s), and the state\'s `next` is not `fix`: what it names comes first.' }, open, report, nextOf(state))
    if (at.cycle > cycleLimit) return unlanded('bound', 'round ' + round + ' has ' + cyclesDone() + ' recorded fix -> audit cycle(s) and ' + state.blockers.length + ' blocker(s) left: the bound of ' + cycleLimit + ' is reached, and the round is unlanded' + (cut ? ' on ' + branch : '') + ' (ruling 6)')

    // Cycle `at.cycle`: the round's branch, then the fixers — one area at a time, serial.
    phase('Fix')
    if (!cut) {
      cut = 1
      branch = branchName(v.run, round, cut)
    }
    const stopped = await onBranch(branch)
    if (stopped) return stopped
    const ctx = { run: v.run, round, stage: 'fix', cycle: at.cycle, attempt: at.attempt, scratch: v.scratch }
    const launch = launcher(ctx)
    const blockers = blockersOf(state)
    const unnamed = blockers.filter((b) => !forkNames('fix', b.key)).map((b) => b.key)
    if (unnamed.length) return halt('state', 'no reporter name can be made for a fork of ' + unnamed.join(', ') + ': the key is too long to carry a prefix inside ' + SLUG_MAX + ' characters', { branch })
    const areas = areasOf(blockers, state.doors)
    log('round ' + round + ', cycle ' + ctx.cycle + ' of at most ' + cycleLimit + ' (attempt ' + ctx.attempt + '): ' + blockers.length + ' blocker(s) in ' + areas.length + ' area(s), on ' + branch)
    const fixerSources = []
    const patches = []
    const fixerForks = []
    for (const area of areas) {
      const name = launch.add(['fix', 'area', String(area.n)])
      const r = await roleStep('fixer', 'fix:r' + round + ':c' + ctx.cycle + ':area' + area.n, 'Fix', fixerPrompt(ctx, launch, name, area, branch), FIXER_SCHEMA)
      if (!r || r.status !== 'worked') return halt('fix', r ? 'the fixer of area ' + area.n + ' (' + area.registry + ') halted: ' + ((r.halt && r.halt.root_cause) || 'no reason given') : 'the fixer of area ' + area.n + ' (' + area.registry + ') returned no result — what it left is UNCOMMITTED in the tree, and what it committed is on ' + branch, { transient: !r, halt: r ? r.halt : null, branch, launched: launch.names })
      const handedKeys = area.findings.map((f) => f.key)
      const returned = r.entries.map((e) => e.key)
      if (returned.length !== handedKeys.length || !distinct(returned) || returned.some((key) => !handedKeys.includes(key))) return halt('fix', 'the fixer of area ' + area.n + ' was handed ' + handedKeys.join(', ') + ' and returned entries for ' + (returned.join(', ') || '(none)') + ' — one entry per finding handed, and no other', { branch, launched: launch.names })
      const leftOpen = []
      for (const e of r.entries) {
        const finding = area.findings.find((f) => f.key === e.key)
        if (e.status === 'fixed') {
          if (!SHA_RE.test(String(e.commit || ''))) return halt('fix', 'the fixer of area ' + area.n + ' reported `' + e.key + '` fixed and named no commit by its full sha', { branch, launched: launch.names })
          patches.push({ key: e.key, disposition: 'fixed', detail: e.commit })
        } else if (e.status === 'could-not-fix') {
          couldNot.push({ key: e.key, cycle: ctx.cycle, notes: e.notes || null })
        }
        if (e.status === 'halted') {
          fixerForks.push({ key: e.key, kind: 'new-mechanism', door: finding.door, clause: finding.clause, repro: finding.repro, statement: 'the fixer halted on it — ' + ((e.halt && e.halt.root_cause ? e.halt.root_cause + (e.halt.recommendation ? ' · ' + e.halt.recommendation : '') : '') || '(no halt report returned)') })
        }
        for (const d of e.doors_affected || []) if (!doorsForRetest.includes(d)) doorsForRetest.push(d)
        for (const text of e.left_open || []) leftOpen.push('`' + e.key + '`: ' + text)
      }
      if (leftOpen.length) fixerSources.push(leftOpenSource(name, r.report, leftOpen))
    }
    if (v.stopAfter === 'fixers') return Object.assign({ status: 'stopped', after: 'fixers', branch, fixed: patches, forks: fixerForks, reporters: launch.names }, report)

    const fixDoors = (state.doors ? state.doors.included.concat(state.doors.excluded) : []).filter((d) => blockers.some((b) => b.door === d.door) || doorsForRetest.includes(d.door))
    const done = await secondHalf(ctx, launch, fixerSources, patches, fixerForks, fixDoors, false)
    if (done.halted) return done.halted
    if (!patches.length && !done.fixerForks.length) return Object.assign(unlanded('fix', 'no fixer fixed anything in cycle ' + ctx.cycle + ' of round ' + round + ' (' + couldNot.filter((c) => c.cycle === ctx.cycle).map((c) => c.key).join(', ') + ' could not be fixed) — the same fixers on the same findings are not sent again; each `could_not_fix` entry carries its fixer\'s notes, and the cycle is not counted'), { record: done.record }, nextOf(state))
    if (done.fixerForks.length) return Object.assign(unlanded('fork', 'a fixer halted on a fix that needs a new mechanism, or that would revise a settled decision: ' + done.fixerForks.map((f) => f.key).join(', ') + ' — each comes with an advocate\'s case and an independent drive of the proposal. The human rules; the next `fix` carries the ruling in args.rulings'), { fixed: patches.map((p) => p.key), record: done.record }, nextOf(state))
  }
}

return await (v.probe ? runProbe() : v.stage === 'test' ? runTest() : runFix())
