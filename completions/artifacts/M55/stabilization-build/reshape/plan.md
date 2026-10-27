# The reshaped `test` stage and the route catalog's pilot — build plan

*First cut 2026-10-10 on `fix/rc24-tier1`; **revised 2026-10-11 against an independent review** (4 blocking, 12 should-fix — §12 is the disposition, finding by finding, each checked at the lines it cites) and against a third count of the canary, of what its agents spent their turns on. The record this plan is held to is committed and pushed (`d591f656`). A plan: nothing is built, nothing is ruled, no gate, test or stage was run for it, and it restates no decision — every task cites the ruling or the row it builds. Its links are written for a home beside the count's files, `completions/artifacts/M55/stabilization-build/reshape/plan.md`.*

**[Noted 2026-10-12: by the human's ruling of that day the reshaped stage is run for real in four parts before §9's one whole canary, which stays owed, and the tasks that remain are built in the order those parts need — the plan for the parts, approved the same day and held to beside this one, is [staged-canary.md](staged-canary.md) ([DECISIONS.md](../../../../../DECISIONS.md) → *2026-10-12 — The reshaped `test` stage is run for real in parts*).]**

**What it builds.** Two things ruled on 2026-10-10 ([DECISIONS.md](../../../../../DECISIONS.md) → *The reshaped `test` stage, corrected by a count the same day*, rulings 1–3; the entry below it, ruling 4 only):

- **Stream T** — the stabilization workflow's `test` stage as ruled: the harness, the two scripts, the definitions, the workflow doc, the fences (rows `wf-verify-fan-out`, `wf-route-walk`).
- **Stream P** — the route catalog's pilot in the product's tests, `setup` and `uninstall` (row `wf-route-catalog`).

**What changed in this revision, in five lines.** (1) A row that appears after the one triage — a red check, a seed — is graded by the stage that meets it, and has its arms (`B1`). (2) What makes a filed verdict count is stated before the task that builds it (`B2`). (3) How follow-ups enter the queue is no longer a premise: it is the human's second question, with four options costed on the canary's tables (`B3`). (4) The canary is redone so that it can show what it must (`B4`). (5) The site scan is Rust on the existing source reader with a committed, fence-held list — no python in a product suite, no new allow rule (`S11`) — and the human's questions are three, not eleven. (6) **Seven pre-built pieces are scheduled** (§5), by the human's ruling of 2026-10-11 on the third count: each removes something the canary's agents did by hand, and names the number the first supervised run checks it against. (7) **After a second review:** the cut is re-ordered so that every commit leaves the gate green — the scripts grow beside what they answer today, the harness grows a new path beside the old, and one task at the end deletes the old (§4, *The migration order*); and the image's build becomes a held command (`K7b`), because the session that drives a stage may ask for permissions.

**Abbreviations for write-sets.** `HARN` `.claude/workflows/stabilize.js` · `REC` `dev/stabilize-record` and `tooling-tests/dev_stabilize_record.rs` · `STEP` `dev/stabilize-step` and `tooling-tests/dev_stabilize_step.rs` · `SIM` `tooling-tests/stabilize_simulation.rs` and `tooling-tests/fixtures/stabilize-runtime.mjs` · `FENCE` `tooling-tests/stabilize_harness_fence.rs` · `CAN` `dev/stabilize-canary` and `tooling-tests/dev_stabilize_canary.rs` · `DOC` `implementation/stabilization-workflow.md` · `PEND` `implementation/decisions-pending.md` · `LOG` one appended entry of `DECISIONS.md` · `LIST` `crates/cli/tests/route_catalog/sites.tsv`.

---

## 1. The facts the plan is cut on

| # | Fact | Where |
|---|---|---|
| F1 | The verify loop is `triagePasses`: a triage per wave, everything graded `breaks` or `unclear` verified in one `parallel` batch, `VERIFY_PASSES = 3`, a contested verdict stops the loop for `driveFork`. `runFix` calls the same functions; `fix` is refused today (`NOT_FIT`), so those calls are unreachable and untested | `HARN` 532, 3028–3130, 3139, 3239, 3777–3885 |
| F2 | **Nothing a `test` stage finds is on record before its one record step at the end.** Entries, rows and verdicts live in the script's memory until `stageRecordCommands`. The lap that finishes a triage grades its rows **again** before it verifies | `HARN` 3239–3263, 3564–3610 |
| F3 | A ledger row has no kind and no parent; a triage row has no word for *why* a finding was refuted | `REC` 1293–1309, 2976–3063 |
| F4 | `route_of` sends an ungraded row, and every row graded `breaks`/`unclear` without a verdict, to `triage`; `next_of` answers `triage` while one exists; left so twice (`AUTOMATIC = 2`) it is the human's. A budget is nowhere | `REC` 3099–3135, 3360–3420, 3765–3778 |
| F5 | **A red result is filed as a ledger row by the record call itself** — `red-<item>`, ungraded and open, *so that the next triage grades it* — in the batch that follows triage | `REC` 143–149, 2561–2567; `HARN` 3603 |
| F6 | The run's facts are `stop · rounds · cycles · previous · previous-commit · scope · clauses · findings`; the run rulings are `go · rerun · rounds · cycles · reverify · again`; **`args.rulings` is a list**, so many findings are ruled by key in one invocation — what is missing is a ruling by class | `REC` 1354–1363; `HARN` 914–958 |
| F7 | A verifier's `left_open` is a list of plain strings; triage returns `source` as free text | `HARN` 1589–1678, 2982–2987 |
| F8 | **The account's usage limit reaches the script as a null return** — 297 `null return on verify:<key>` lines in the canary's second invocation, no `transient failure`. `agentR` retries a null twice at once and asks the breaker only before a call's first try. One observation | canary `evidence/then-halted.json`; `HARN` 2705–2734 |
| F9 | The candidate's build, its gate and every held check are **named by label and attempt** (`build-c1-a2`, `gate-c1-a2`, `<item>-c1-a2`), so a stage killed before its record runs all of them again — 45 minutes of gate and regression set on the canary's numbers; the previous release's build is named by commit and answered from the job that stands | `HARN` 2797–2804, 3461, 3500 |
| F10 | A verdict counts today **only with its verifier's report**; reports of another attempt are tolerated by the report check; a held check's verdict is a written-once, vetted file of the run under `r<N>/checks/` | `HARN` 3026–3027, 3094; `REC` 1235, 2620, 3878–3912, 4174 |
| F11 | The state digest carries lists of ids up to 200 | `STEP` 2029–2058 |
| F12 | Every product suite drives the binary cargo compiled in. The one tool that runs built tests on another binary is `dev/regression-set`, which goes the other way | `crates/cli/tests/support/trial_corpus.rs` 458, 504 |
| F13 | **`setup` and `uninstall` codes are not all minted in `crates/cli/src/setup.rs`.** 45 constructor calls stand there; `setup.dirty-install-path` is minted through a `match` in `crates/engine/src/finalize.rs` (2144), `setup.unverified-install-path` is a constant there (2640), `uninstall.staged-prose` is also minted in `crates/cli/src/task.rs` (1329). A family is its codes' prefix, and a scan of constructor calls alone misses the first | as named |
| F14 | A shared source reader exists and warns against a second copy (`crates/cli/tests/support/rust_source.rs`, 8–16); the precedent fence scans with it for one namespace (`finalize_family_registry.rs`) | as named |
| F15 | The path-class check reads `opening..loop`: product tests committed before the opening record trip nothing; after it, `crates/` moves only through a round's merge | `STEP` 1243–1247; `DOC` → Branches |
| F16 | The six roles that hand over a report hold `Write` and **no edit tool**; the paragraph every one of them repeats says *write the whole report with your file tool*; a report handed over a second time is refused `exists` | `.claude/agents/*.md` line 5; `REC` 2441 |
| F17 | The fence pins the sentence *the round's triage is not finished* in `finding-verifier.md` | `FENCE` 1320–1328 |

---

## 2. Where a ruling cannot be built as worded, or two collide

Twelve claims. The review found nine true as written and three overstated; those three are corrected here (3, 5, 11), and two more are sharpened (1, 7).

1. **"A verdict is recorded when it returns" collides with "one record step at the stage's end" (F2).** A record is one batch behind one full gate, and a verdict needs a ledger row to belong to. *What can be built:* the stage records **twice** — after triage, and after verification — and between the two a verifier **files** its verdict through the record script. A filed verdict is on this machine's disk, uncommitted and unpushed, until the second record: a stage that is killed loses nothing; a disk that is lost loses what was filed. → `Q1`.
2. **"The first message of the account's usage limit halts the stage" cannot be seen (F8).** The script is handed nothing, not a message. Settled as an engineer (§6, `E7`): a wave in which two verifier calls return nothing is the last wave; no verifier call is tried a second time. Every other role still retries as today, bounded by the breaker, which is asked before every try.
3. **"A command's refusal sites" is not derivable from source.** No data maps a code to a door. What a scan can list is the construction sites of the **code families** a committed table assigns to a command, plus the sites whose code is not a literal at the call. *And the table is not clean even for the pilot* (F13): one `setup` code is minted in the engine through a `match`, and is found only by holding the code vocabulary, not the call sites. → `S0`, `P2`.
4. **"One case per refusal site and cause": a fence can hold sites, never causes.** *Causes per site* is a hand count with its derivation.
5. **"The same case on the previous release's binary says whether it is a regression."** No suite can do it (F12). The regression tool's swap is the general road and goes the other way; the pilot adds a narrow one for this suite. So the ruling's sentence becomes **a table run once, by hand, at the pilot's end** — and the pilot computes no word *regression* at all: by source the previous release did not leave the outside script intact either (it wrote through the link), so any rule the pilot fixed beforehand would be a guess about a binary nobody drove for this plan (`S1`).
6. **"Ties a red case to its ledger row" — no ledger exists before the opening.** An expected-red entry names a key, and a fence holds the key to a findings file. **Before the opening that tie certifies itself** where the builder minted the key; it becomes a real one when the opening seeds the rows.
7. **"A reproduction identical to one on record inherits that row, by hash", and "a lead already listed is not listed again".** Identity without a model is identity of bytes, and no reproduction is a script. That it would almost never fire is this plan's inference — the census counts shapes, not sameness — so **neither is built**: a duplicate is verified, or listed, twice, at most about one in seven by the rate at which triage merged. A row (`wf-same-evidence`).
8. **"In a later round the replay tool runs its reproduction" has no instrument.** What the build does in a later round is what the record script does today: a finding an instrument finds again is graded again and, graded `breaks` or `unclear`, verified again in that round. Whether that stands is the human's, **before round 2's `test`** — a row of its own (`wf-round-two-queue`), not parked under `wf-replay`.
9. **The advocate leaves `test` for a stage that is not fit for use.** Ruled (`wf-verify-fan-out`: *the advocate out of `test`*). Until the `fix` half is repaired a contested finding reaches the human with its verifier's basis and no case. A declared bound, said to the human with the questions, and no question.
10. **"One budget per stage" collides with the lap and with `AUTOMATIC` (F4).** Settled as an engineer (`E3`): the budget is the round's `test` stage's, counted on record across its invocations; a filed verdict spends one, a launch that left nothing does not; spent, the step is the human's.
11. **"A route item nobody reached holds the close until verified or ruled" has no ruling by class.** Many findings can be ruled by key in one invocation (F6) — a list the orchestrator composes from the stop's view. What is missing is *the unreached route items of round N*, as one ruling. A row due at the first stop (`wf-rule-a-class`). How long that list is depends on `Q2`.
12. **A hit rate per kind is honest only if the kind is on record before the verdict.** Built: the kind is written with the row and is refused a change once the row has a verdict.

---

## 3. The shape, in one place

The plan's proposal inside the rulings. `Q1` and `Q2` are where it needs the human's word; the tasks are cut on the recommended answers, and §6 says what moves on another.

```text
state → attempt marker → preflight · builds · candidate's gate · held checks (all named by commit) · scope
  → instruments (review rows, trial arms, audit — and one route hunter per command in scope)
  → triage, ONCE: every finding, every red result, every row nobody graded — keys, merges, grades, a kind each
  → RECORD 1: reports, results, rows with kind and origin, grades            ← findings on record
  → verify, in waves, while the budget lasts:
        the record script's queue (ranked) → verifiers, one finding each
        a verifier FILES its verdict and what it hands on, through the record script
        what it hands on enters the queue as Q2 rules; a kind is held to its evidence
        a finding nobody here can drive is marked once; a wave with two silent calls is the last
  → RECORD 2: the filed verdicts, the handed-on rows, the stop's view         ← hit rates, four lists
  → next:  verify  (a ranked row is unverified and budget is left — the next invocation continues)
           triage  (a row nobody graded and no triage saw — the next invocation grades it first)
           rule    (budget spent with route items unreached · undrivable · contested · needs-bound)
           fix / close / stop / retest, as today
```

**An invocation that continues** runs no instrument. It grades **only rows no triage has graded and that are not handed-on items** — a seed added since, a red row a killed batch left — and no graded row a second time; then it reads the queue, takes the verdicts already filed, and launches only what is missing.

**A class** is what the queue is ranked by and what a hit rate is given per: *kind* (`route` — the item quotes the route a refusal printed, with the refusal's code where it has one · `behaviour` — it carries the commands that show the misbehaviour · `lead` — neither) × *origin* (`hunt` — an instrument's own finding, a red check, the scope step's · `seed` · `under-confirmed` · `under-not-a-break` · `under-no-repro`). **The order of classes is data of the opening record** and is the human's to set there: the ruling fixes that route items come first, those under a confirmed parent ahead of the rest, then behaviour items, and that nothing under a parent that did not reproduce is verified. Inside that, the count's own rates are what the opening has to go on — route under a confirmed parent 7 of 10, route under a parent that was not a break 7 of 12, anything straight from the hunt 1 of 8. The build ships no default; a run without an order is not ready.

---

## 4. The tasks

**How every builder task runs** ([dev-workflow.md](../../../../../implementation/dev-workflow.md); lessons 8, 16, 24, 26): red first — the brief says so before anything else and asks for the red output back; where a test followed its code the builder says which, shows the suite red against the parent and its mutants killed, and the commit says so. One commit, one full `dev/gate` on that commit's tree, in the background, read unpiped, with `--private-target` in a worktree. `git status` and the diff's stat are read after the last edit and again before the commit. Builder tasks are `build-executor` on Opus; git steps on Sonnet.

**Three parts:** a prelude on `fix/rc24-tier1` (`S0`) → the two streams, each in a worktree on a branch of its own forked after `S0` → a join (`J1`, `J2`). The stream branches fork from the loop branch and not from `origin/main`; the precedent is `fix/rc24-core-review`, merged at `724e4ce7`. Names are the orchestrator's.

**Write-sets are disjoint between the streams, with one declared overlap** — `DECISIONS.md`, appended by every task and resolved by `dev/merge-logs` at the join — **and one declared dependency**: the hunter's prompt (`T8`) names two files of stream P's, `LIST` (which `S0` lands before the fork) and the catalog's table (which exists after `P1`); `J1` holds that both exist. Stream P writes nothing under `.claude/`, `dev/`, `tooling-tests/` or `implementation/` but `implementation/pinning.md`; stream T writes nothing under `crates/`. Neither writes `CLAUDE.md`. **A third lane** runs beside both — `K7` (`completions/trial-harness/`) and `K8a` (`dev/regression-set` and its suite) — writes no file either stream writes, and is merged into stream T by `M3` before `K8b`, `K7b` and `K6`, which use what it built.

### The prelude

**`S0` — the list of refusal sites: one scan, one committed list.** *For:* ruling 2 (*lists that command's refusal sites from the source*), ruling 3 (*a fence that reads the refusal sites from source*); §2.3.
- *Writes:* `crates/cli/tests/support/refusal_sites.rs` (new) and its line in `crates/cli/tests/support/mod.rs`; `crates/cli/tests/refusal_site_list.rs` (new) and its line in one group root; `LIST` (new, generated and committed); `LOG`.
- *What:* a scan **built on `support::rust_source`**, never a copy of it: every `Finding::graded(` · `::block(` · `::blocking(` call and `Finding { … }` literal in production code of both crates, each a **site** with a stable id (file · enclosing function · code · ordinal in the function — never a line), its severity, its code (a literal, a resolved constant, or `dynamic`), the route's kind as written and its text where it stands at the call, and its **family** by a table from code prefix to command. `LIST` is that scan written out, one site a row, for every family; the suite holds the file equal to the scan and says how to regenerate it (a variable CI refuses, as the goldens have it).
- *Done when* — `refusal_site_list::`, red first: `the_committed_list_is_the_scan` (red against an empty file, with the sites named); `a_site_inside_an_interleaved_test_module_is_not_listed_and_one_after_it_is`; `a_code_behind_a_constant_is_resolved_and_one_behind_an_expression_is_dynamic`; `a_site_keeps_its_id_when_lines_above_it_move`; `a_dynamic_site_is_listed_unassigned_and_never_dropped`; `on_this_tree_the_finalize_namespace_is_the_registrys` — the scan's literal-coded `finalize.` set equals the literal-coded members of `cli::render::FINALIZE_FAMILY`, the one family that already has a scanner. The counts for `setup` and `uninstall` are printed in the commit, not pinned.
- *Must not:* scan anything under `crates/*/src` by a second reader; decide what is a *refusal* beyond severity and route as written; add a case (stream P's).
- *Depends on:* nothing. *Who:* a builder.

### Stream T — the tooling

**The migration order — every commit leaves the full gate green.** The second review found that the first revision's cut did not: the simulation runs 43 scenarios through the committed harness and the real scripts on every gate, the canary's suite opens a run through the real record script, and a script task that changed an answer, or a refusal of the whole stage, turned them red (`HARN` 2682–2685; `SIM` 462–500; `tooling-tests/dev_stabilize_canary.rs` 808–818). No path is gate-exempt and a builder cannot commit on red. So the build moves in three phases, and one rule holds in all of them.

- **Phase A — the scripts grow, and nothing they answer today changes.** The switch is data: *a run whose opening holds an `order`*. In a run without one, every call the harness makes today is answered as today and the run's directory holds no byte it did not hold; every new writer and read is one the harness does not call yet. The simulation and the canary's suite open their runs without an order, so they do not move.
- **Phase B — the harness grows the new path beside the old**, chosen by what the state's digest says of the run (it has a queue, or it has none). A scenario opened without an order runs the old path, unchanged. Each harness task writes its new scenarios on an opening with an order and **moves** the old scenarios whose subject it rebuilds. **No stage is refused as not fit**; on the new path, a step that is not built yet is a refusal *of that path*, which the task's own scenario asserts.
- **Phase C — the old is deleted**, once no scenario opens without an order: the order and the budget become owed, the old answers and the old path go, and every fixture that opens a run gains the facts in that commit.
- **The rule: a task carries the assertions it breaks.** Where a change moves a text that `SIM`, `FENCE`, the canary's suite, `STEP`'s suite or the probe's suite pins, the task changes that assertion and nothing else of the file, and its commit names each. Stream T is one lane, so no two tasks hold one of those files at once. A builder finds them by running the `g_tooling` target before the change is finished, not by reading.

| Task | What the simulation and the canary's suite do at its commit | Why the full gate is green |
|---|---|---|
| `T1` | run as they do, on the old harness, over openings without an order | the new cells are optional; the arm `a_run_without_an_order_holds_the_bytes_it_held` |
| `T2a` | the same | the three facts are optional and owed by nothing yet; without an order the queue is empty and the digest's new fields are empty — `STEP`'s digest arms are carried |
| `T2b` | the same; the old harness never meets `verify` | the new routes and `next` are computed only in a run with an order; the old table arm stands, unmodified, beside the new one |
| `T3a`, `T3b` | the same | a writer and a file class nobody calls; `apply` reads filed verdicts only where there are some |
| `T4` | the same | the view is rendered only in a run with an order, so no record's file set moves |
| `K2`, `K3`, `K1a` | the same | writers and reads nobody calls; `K1a`'s fact is optional for good |
| `T5a` | old scenarios unchanged; new ones on an opening with an order | the new triage call is on the new path only |
| `T5b` | old scenarios: the names of the gate's and the held checks' jobs change, and those assertions are carried; new ones run to the first record, assert `hunted` and `next: verify`, and assert that a continuation is refused **on that path** as not built | the shared change is a name; nothing is refused for a run without an order |
| `K8b` | the regression set's stand-in is handed one more flag | carried in `SIM` |
| `T6a` | new scenarios run through the second record | the new path only |
| `T6b` | the continuation's scenarios; `T5b`'s refusal goes; the old scenarios about a lap, an unverified finding and an advocate are **moved** | each moved scenario is green on the new path in this commit; the others still run the old |
| `T7` | the scenarios that pin a return's shape are moved | both paths return; the new path's words are the new ones |
| `K1b` | every stand-in runs the kit's call — the prompts are shared by both paths | carried in `SIM` |
| `T8` | a new kind of item; nothing old moves | a new chain |
| `K7b`, `K6` | two held kinds are added; the scenarios with a trial arm are moved to the image's hold | carried |
| `T9` | the canary's suite: the tool writes the facts and the suite reads them back; `not_ready` is still `[]` | the facts are still optional |
| `T11` | every scenario still on an opening without an order is moved, about ten a commit | each commit moves scenarios only, and both paths exist |
| `T12` | the fixture's opening holds the facts for every scenario; the canary's tool writes them since `T9` | its first check is that no scenario opens without an order; then the old path, the old answers and the option go, and the facts are owed |

Order, with the pre-built pieces of §5 (`K…`) where they stand: `O0` → `O1` → `T1` → `T2a` → `T2b` → `T3a` → `T3b` → `T4` → `K2` → `K3` → `K1a` → `O2` → `T5a` → `T5b` → `M3` → `K8b` → `O3` → `T6a` → `T6b` → `T7` → `K1b` → `O4` → `T8` → `K7b` → `K6` → `T9` → `T11` → `T12` → `O5` → `KR` → `T10`. The `K` tasks are specified in §5; a task here that carries the harness half of one says so.

**`T1` — a row can carry what the queue is ranked by.** *For:* ruling 1; `wf-verify-fan-out` (*kind and parent on every entry*); §2.12; `B1`. *Phase A.*
- *Writes:* `REC`, `LOG`; carried assertions.
- *What:* a row may carry `kind`, `origin` and `parent` — where they are held, in the ledger's own table or in a table beside it, is the builder's, under the phase's condition. A row of kind `route` **quotes the route line the refusal printed**, with the code where the refusal has one; one of kind `behaviour` carries its commands; a row given a kind without its evidence is entered as a `lead`, and says so. `origin` of a handed-on item is **computed** from its parent's row. **A red result's row is filed with its class** — `behaviour`, `hunt` — by the call that files it today, its evidence the tool's verdict file for a held check and the preflight's report for one that is not held. A triage row may say **why a finding was refuted**, in one of three words, and there is a verdict for *nobody here can drive it* (`undrivable`, with a reason word). One row is read whole by key.
- *Done when* — `dev_stabilize_record::`, red first: `a_kind_without_its_evidence_is_a_lead`; `a_route_item_quotes_its_printed_route_with_or_without_a_code`; `a_red_results_row_has_its_class_when_it_is_filed`; `a_handed_on_items_origin_is_its_parents_verdict_and_no_caller_says_it`; `a_kind_is_refused_a_change_once_its_row_has_a_verdict`; `a_refutation_says_which_of_three_it_is`; `an_undrivable_finding_is_the_humans_at_once`; `a_row_is_read_whole_by_its_key`; **`a_run_without_an_order_holds_the_bytes_it_held`** — one whole stage's calls, as the harness makes them today, against the files they left before this task. Three mutants, each red.
- *Must not:* touch the harness, the queue or `next`; refuse a row for lacking a kind (`T2a`); hold a code to any vocabulary (a row, §7).
- *Depends on:* `O1`. *Who:* a builder.

**`T2a` — the opening's numbers, and the queue.** *For:* ruling 1. *Phase A.*
- *Writes:* `REC`, `STEP` (the digest's shapes), the probe's suite where a case is built on the digest's shape, `LOG`; carried assertions.
- *What:* three run facts — `budget`, `order` and, under `Q2`'s recommended answer, `hand-on` — **optional until `T12`**; a run that holds an order is *a run with a queue*, and says so in its state. In such a run a row without a kind is refused. **The queue** is the script's: the unverified rows whose class is on the order, ranked by it, ties by the ledger's order; the digest carries its first keys and its length by class.
- *Done when* — `dev_stabilize_record::`: `a_run_with_an_order_says_that_it_has_a_queue_and_one_without_answers_as_it_did`; `in_a_run_with_an_order_a_row_without_a_kind_is_refused`; `the_queue_is_the_order_of_the_opening_record_and_a_class_off_it_is_not_in_it`; `an_item_under_a_parent_that_did_not_reproduce_is_never_queued`; `a_lead_is_in_no_queue`. `dev_stabilize_step::the_state_digest_carries_the_queue_as_keys_and_counts`.
- *Must not:* change a route or `next` (`T2b`); make a fact owed (`T12`). *Depends on:* `T1`. *Who:* a builder.

**`T2b` — what follows from the queue and the budget.** *For:* ruling 1 (*what holds the close*); §2.10; `B1`. *Phase A.*
- *Writes:* `REC`, `STEP`, `LOG`; carried assertions.
- *What:* **in a run with a queue**, `route_of`, `next_of` and `position_of` as §3 has them; in any other run, as today. **Spent** is counted by the script: a verdict filed or recorded in the round's `test` stage counts, `undrivable` among them; a launch that left nothing does not. `next` is `verify` while a ranked row is unverified and budget is left; `triage` **only** for a row no triage has graded and that was not handed on; `rule` once the budget is spent with route items unreached. *Left unverified twice* counts launches that left no verdict. A budget and an order the human changes are facts a ruling writes.
- *Done when* — `dev_stabilize_record::`, each on a run with an order: `a_spent_budget_ends_in_the_humans_step_and_never_in_verify`; `budget_left_and_a_ranked_row_unverified_is_verify`; `a_row_nobody_graded_is_triage_whatever_the_queue_holds` — a red row filed after a triage, and a seed added between two invocations, each end in a step that grades it (`B1`); `a_row_nobody_reached_is_not_counted_as_left_unverified`; `the_budget_is_counted_across_the_invocations_of_one_stage`; `a_budget_the_human_raises_reopens_the_queue`. **The table arm of today's two functions stands unmodified**; a second table, for a run with a queue, stands beside it in the header and in the suite.
- *Must not:* touch the harness; change an answer for a run without an order. *Depends on:* `T2a`. *Who:* a builder.

**`T3a` — a verdict is filed when it returns: the filing, and what makes it count.** *For:* ruling 4; §2.1; `B2`. *Phase A.*
- *Writes:* `REC`, `LOG`.
- **The acceptance, stated before the task.** A filed verdict counts when all of these hold, and the script holds each: **(i)** the report of the same reporter and attempt is on disk in the round's report directory, **and the filing stores that report's sha256** — a dead attempt's report is held by no check today (it is `other_attempts`), so this is what holds it; **(ii)** the file names its reporter and its attempt, and the reporter's name is the key's (`verify-<key>`); **(iii)** the attempt is one the round has begun on record; **(iv)** the candidate's hash it asserted is the round's, and a regression fact stands only with the previous binary's hash behind it — else the fact is *unknown*; **(v)** *once* is once per key per round. **What the script cannot hold is who called it** — a declared bound, the same as for a report; the harness narrows it by taking a verdict of this invocation only when the return's word and path agree with the filing's digest.
- **Its home, named.** A filed verdict is a written-once file of the round in a directory of its own, `r<N>/verdicts/`, named for its reporter and attempt. It cannot lie beside the reports — the report check counts a file there without a report's name as `extra` and fails (`REC` 3890–3899) and `run_file` refuses it (4171–4172) — nor among the held checks' verdicts, which the result of an item is read from. `run_file` learns the class, the vet covers it at the three boundaries, and `set_aside` takes it as it takes a report or a scope.
- **A handed-on item's key is minted by the script**, from what the verifier proposes: unique in the ledger and short enough that `verify-<key>` is still a slug of at most 100 characters.
- *What:* the writer — a verifier hands over a file it wrote: the key, the verdict, the refutation's word or the regression fact, the hashes it asserted, and what it hands on as structured entries. The state reads filed files no record holds: their findings are verified, the budget is spent, the handed-on items are in the queue.
- *Done when* — `dev_stabilize_record::`: `a_filing_without_its_report_is_refused`; `a_filing_keeps_its_reports_hash_and_a_report_that_changed_since_is_no_verdict`; `a_filing_under_a_reporter_that_is_not_the_keys_is_refused`; `a_filing_for_an_attempt_the_round_has_not_begun_is_refused`; `a_second_filing_for_a_key_in_the_round_is_refused`; `a_verdict_for_a_key_the_ledger_lacks_is_refused`; `a_verdict_on_another_binary_is_no_verdict_and_a_confirmed_one_without_the_previous_hash_is_unknown`; `a_filed_verdict_counts_before_any_record_holds_it`; `a_handed_on_item_gets_a_key_its_verifiers_name_can_carry`; `a_handed_on_item_enters_the_queue_with_its_class_and_no_grade`; `an_item_handed_on_beyond_the_openings_number_keeps_its_kind_and_is_in_no_queue` (`Q2`); `a_text_with_a_denylisted_term_refuses_that_filing_and_no_other`; `a_file_among_the_filed_verdicts_without_a_verdicts_name_is_refused`.
- *Must not:* touch the record's batch (`T3b`) or the harness. *Depends on:* `T2b`, `Q1`, `Q2`. *Who:* a builder.

**`T3b` — the record takes what was filed.** *For:* ruling 4; the verdict half of `wf-b4`. *Phase A.*
- *Writes:* `REC`, `STEP` (the reconciling table's rows; the vet at the report check, the commit and every push), `LOG`.
- *What:* the record's batch takes verdicts and handed-on rows from the filed files, **and holds each to the report hash its filing stored**. The table of states between two commands gains those between a filing and the record that takes it.
- *Done when* — `dev_stabilize_record::the_record_takes_its_verdicts_from_the_files_and_from_no_caller`; `::a_record_refuses_a_filed_verdict_whose_report_is_gone_or_changed`. `dev_stabilize_step::a_stage_killed_between_a_filing_and_its_record_ends_in_one_of_two_states`; `::a_filed_verdict_overwritten_by_hand_is_set_aside_and_its_finding_is_unverified_again`; `::a_filed_file_is_vetted_at_the_commit_and_at_the_push`. Four mutants, each red.
- *Must not:* touch the harness. *Depends on:* `T3a`. *Who:* a builder.

**`T4` — what a stop shows.** *For:* ruling 1. *Phase A.*
- *Writes:* `REC`, `STEP` (counts in the digest), `LOG`.
- *What:* **in a run with a queue**, a rendered view of the round, derived and written by every record that changes it: the hit rate per class — verified, confirmed, refuted by each of the three words, undrivable — and four lists by key: confirmed inside the round's scope · confirmed outside it · ranked and not reached, route items first · leads, by door. The digest carries counts, the file's path and its hash.
- *Done when* — `dev_stabilize_record::`: `the_stops_view_is_a_function_of_the_record`; `the_hit_rate_is_per_class_and_adds_up_to_the_verdicts`; `the_four_lists_hold_every_row_once_or_say_why_not`; `a_view_edited_by_hand_is_set_aside`; `a_run_without_an_order_has_no_view`.
- *Must not:* take a number from a caller; render rates over the whole run (a row, §7). *Depends on:* `T3b`. *Who:* a builder.

**`T5a` — the harness: the new path's triage.** *For:* `wf-verify-fan-out`; `B1`; the arithmetic half of `wf-triage-reads-harness`. *Phase B.*
- *Writes:* `HARN` (its self-test with it), `SIM`, `FENCE`, `LOG`.
- *What:* the branch — a run with a queue takes the new path from the triage on. There the one triage is handed, beside the reporters' findings and the rows nobody graded, **every red result of the invocation as a source, each under the key the record call will file it with** (`red-<item>`): triage returns it under that key, found again, and the batch files the row before it grades it. Every handed finding has a number in the prompt; triage hands over its entries as one file (`K2`), each saying which numbered findings it folds; the return is the digest and a path.
- *Done when* — simulation, red first, on an opening with an order: `a_red_held_check_is_graded_by_the_stages_one_triage_under_its_own_key` (`B1`); `a_red_check_that_is_not_held_is_graded_too_and_its_evidence_is_the_preflights_report`; `triage_that_leaves_a_handed_finding_in_no_entry_or_in_two_halts_by_name`; `an_entry_without_its_kinds_evidence_is_recorded_as_a_lead`. Self-test: the triage prompt names every source kind. Fence, arm (r).
- *Must not:* change what a run without an order does. *Depends on:* `T1`–`T4`, `K2`, `O2`. *Who:* a builder.

**`T5b` — the harness: the record before verification, and nothing built twice.** *For:* ruling 4; §2.1, §2.9; `S12`. *Phase B.*
- *Writes:* `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* on the new path: instruments → the one triage → **the first record**, composed by the tool (`K3`) and pushed; the stage returns `hunted` with `next: verify`; a contested verdict is recorded as contested and is the human's, with no advocate. **A continuation on the new path is refused as not built** until `T6b` — a refusal of that path, never of the stage. *For both paths:* the two builds are named by commit; **the candidate's gate is named by its commit, and a held check by its commit and the item's own attempt in the round** — so a stage killed before its first record asks for the same names and is answered by the jobs that stand, and an item's automatic re-run (a void or a dead check: `REC` 3217) asks for the next. **A job that stands is taken only while it runs or where its verdict is green**; one that ended red, void or dead is passed over for the next name, as a record's gate already is (`HARN` 2762) — a starved red gate is measured again, never answered forever.
- *Done when* — simulation: `a_stage_records_its_findings_before_anything_is_verified`; `a_stage_killed_before_its_first_record_holds_its_green_gate_and_its_regression_set_once`; `a_held_check_that_left_no_verdict_is_started_under_its_next_name_in_its_rerun`; `a_gate_that_stands_red_is_not_the_answer_to_a_stage_started_again`; `a_contested_verdict_is_the_humans_and_no_advocate_is_launched_on_the_new_path`; `a_continuation_is_refused_on_the_new_path_until_it_is_built`. The old scenarios that pin a job's name are carried. Fence, arm (t): the `advocate` and `proposal` rows of `NEEDS` are still reached, by the old path.
- *Must not:* touch the scripts; refuse a stage of a run without an order; make a held gate's verdict stand for a tree that is not its commit's (`wf-held-gate-tree` stays for the tool's own check).
- *Depends on:* `T5a`, `K3`. *Who:* a builder.

**`T6a` — the harness: the verify phase.** *For:* ruling 1; ruling 4 (*marked so once*); `wf-unverifiable-relaunch`. *Phase B.*
- *Writes:* `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* waves: one tool step reads the queue's digest → verifiers, at most eight and never more than the budget has left → each is handed a **key** and the read that prints its row, the two binaries, and the one plain call that files its verdict, composed with the two hashes as plain words → the wave's report check → the queue again. A verifier's return is a word and a path, held to the filing's digest; one that returned and filed nothing left no verdict. `undrivable` is a filed verdict. Then the second record, composed by the tool (`K3`).
- *Done when* — simulation: `handed_on_items_are_verified_in_the_stage_that_found_them_and_the_stage_ends_at_its_budget`; `an_item_under_a_confirmed_parent_overtakes_an_older_item_of_a_lower_class`; `a_lead_and_an_item_under_a_parent_that_did_not_reproduce_are_never_launched`; `a_finding_nobody_here_can_drive_is_not_launched_again_once_that_is_filed`; `a_verifier_that_returns_a_verdict_and_filed_none_left_no_verdict`. Fence, arm (r); arm (s); arm (k): **the pinned sentence of the verifier's definition moves to the one `O3` added** (`S6`).
- *Must not:* read a row into the script; launch a verifier for a key the queue did not print. *Depends on:* `T5b`, `O3`. *Who:* a builder.

**`T6b` — the harness: an invocation that continues, and a wave that is the last.** *For:* ruling 4; `wf-breaker-parallel` in part; §2.2; `B1`. *Phase B.*
- *Writes:* `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* the continuation of §3, on the new path — no instrument; a triage only over rows nobody graded; the filed verdicts taken; only what is missing launched. A verifier call is never tried twice; a wave in which two calls returned nothing is the last; `agentR` asks the breaker before every try. A stage whose record step then returns nothing halts as *possibly the account's limit*, naming the filed verdicts. `T5b`'s refusal goes. **The old scenarios about a lap, an unverified finding, a second pass and an advocate in `test` are moved to an opening with an order**, each rewritten for the new path or retired with its cell's new home named.
- *Done when* — simulation: `a_verdict_filed_by_an_invocation_that_died_is_not_verified_again`; `a_stage_killed_after_its_first_record_is_continued_without_an_instrument`; `a_continuation_grades_a_row_nobody_graded_and_no_row_a_second_time`; `a_rerun_inside_a_round_hunts_its_due_items_grades_once_and_spends_the_rounds_own_budget`; `one_silent_verifier_call_leaves_its_finding_unverified_and_the_phase_goes_on`; `two_silent_calls_in_a_wave_make_it_the_last_and_none_is_tried_again`; `a_stage_whose_record_step_is_silent_after_that_halts_naming_the_filed_verdicts`.
- *Must not:* change how any other role is retried; delete the old path (`T12`). *Depends on:* `T6a`. *Who:* a builder.

**`T7` — the harness: what a stage returns, and the rulings about the run.** *For:* ruling 1; `wf-return-names`. *Phase B.*
- *Writes:* `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* on the new path the status says what happened — `hunted`, `verified`; the return carries the budget (limit, spent), the queue's length by class, the hit rate's counts and the stop view's path and hash, and no list. Run rulings `budget`, `order` and `hand-on` take the one road the others take. The old scenarios that pin a return's shape are moved.
- *Done when* — self-test: the three among the refusals that precede every agent; simulation: `a_budget_ruled_at_a_stop_is_recorded_and_starts_nothing`; `the_return_names_what_happened_and_holds_no_list`; `a_halt_after_silent_calls_says_that_it_may_be_the_limit`; fence, arm (f).
- *Depends on:* `T6b`. *Who:* a builder.

**`T8` — the route hunter, as an instrument.** *For:* ruling 2; `wf-route-walk`. *Phase B.*
- *Writes:* `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* one kind of item, `route-hunt`, a chain of one driving step; an item names its command as its door, so the round's scope selects it. Its prompt names **the committed list** (`LIST`) and the command's family; says that a family may list no site where its codes are built through a helper, and that the count then says so; names the catalog's table, so that a site and cause that has a case is not hunted again; and asks for the coverage count — sites listed, provoked, routes followed — kept with the item's result.
- *Done when* — simulation: `a_route_hunt_runs_for_a_command_in_the_rounds_scope_and_for_no_other`; `its_coverage_count_is_on_record_with_its_result`; `a_hunter_that_returns_no_count_voids_its_item`; fence, arms (a) and (s).
- *Must not:* hold the count to anything but its shape. *Depends on:* `S0`, `T7`, `O4`. *Who:* a builder.

**`T9` — the small canary's setup follows.** *For:* §9; the first third of `wf-canary-setup`. *Phase B.*
- *Writes:* `CAN`, `LOG`.
- *Done when* — `dev_stabilize_canary::`: `a_fresh_setup_is_ready_with_the_budget_the_order_and_the_hand_on_it_was_given`; **`its_seeded_rows_carry_the_grade_and_the_class_the_canary_plants`** — a seed is graded at the opening, so that no triage decides whether a plant is launched; `its_doors_are_the_review_rows_the_hunts_and_the_arms`; `its_expectation_names_what_it_prices`.
- *Depends on:* `K6`. *Who:* a builder.

**`T11` — the scenarios that are left move.** *Phase B's end.* *Writes:* `SIM`, `LOG`. Every scenario still opened without an order — the records, the commits, the pushes, the held commands, the halts — is moved to an opening with one, about ten a commit, each commit behind its gate; a scenario whose subject the new path no longer has is retired with the arm that took its cell named. *Done when:* no scenario opens without an order, by the suite's own count, printed in the last commit. *Must not:* touch the harness. *Depends on:* `T9`. *Who:* a builder.

**`T12` — the old is deleted.** *Phase C.* *For:* `wf-verify-fan-out` (*`VERIFY_PASSES` gone*); `E8`.
- *Writes:* `HARN`, `REC`, `STEP`, `SIM`, `FENCE`, the probe's suite, `LOG`.
- *What:* the order and the budget are owed by an opening; the answers for a run without an order go; in the harness the old loop, `VERIFY_PASSES` and the lap that grades again go, and `runFix`'s unreachable calls point at the new functions. Every fixture that opens a run, in every suite, gains the facts.
- *Done when:* `dev_stabilize_record::an_opening_without_a_budget_or_an_order_is_not_ready`; the old table arm is gone and the new one is the only one; fence: no function of the harness names a pass, and arm (t) holds every cell to an arm of the one path; the self-test's count is printed. The gate's `passed=` is the count before the task, less the retired arms, each named.
- *Depends on:* `T11`. *Who:* a builder.

**`T10` — the doc says what the code does, and the rows are moved.** A document task: its check is the commit's own table and the link fence, and nothing is red first.
- *Writes:* `DOC`; the header comments of `HARN`, `REC`, `STEP`; `PEND`; `LOG`.
- *What:* the sections *A round's `test`*, *Reading the next step*, *Triage, the human's list and the forks*, *The record step*, *Opening a run* (**the opening writes the order, the budget, the hand-on number, and one `route-hunt` item for each command it wants hunted**), *Close*, *Halt and resume*, *The small canary* and *Open questions and declared bounds* say the stage as built; the bounds gain §2's 2, 3, 7, 8, 9 and §7's. In `PEND`: `wf-verify-fan-out` and `wf-route-walk` discharged with their commits; `wf-unverifiable-relaunch`, `wf-return-names` and `wf-image-hold` discharged; `wf-breaker-parallel` annotated — built for verifiers, bounded by the breaker for every other role; `wf-fork-serial` rewritten as the `fix` half's; `wf-triage-reads-harness`, `wf-b4` and `wf-return-size` annotated; the rows of §7 added, the six `wf-pre-…` rows among them.
- *Done when:* every sentence of those sections that named a pass, an advocate in `test`, *verifies everything graded breaks or unclear* or `status: triaged`, against *made true by `T<n>`* or *as it now reads*; every row of §7 found in `PEND` by its key; `doc_link_fence` green.
- *Depends on:* the stream. *Who:* a builder.

**The orchestrating session's own steps** (lesson 23: every edit under `.claude/agents/`, of the settings and of `CLAUDE.md` is its own, each small, each behind its own full gate. Lesson 25: a paragraph several definitions repeat is changed in every copy or in none. **And: before a sentence is removed from a definition, `FENCE` is searched for it; a sentence a fence pins is removed only after the builder's task that moves the pin.**)

| # | Edit, or act | When |
|---|---|---|
| — | The record the plan is held to | **done**: `d591f656` |
| `O0` | **The human's ruling of 2026-10-11 on the pre-built pieces is recorded** — one entry of `DECISIONS.md`, with this plan and the third count committed beside the first two counts — so that a builder in a worktree can read what it builds | before any `K` task; it can go with the plan's own commit |
| `O1` | Nothing is edited: `Q1`–`Q3` are put to the human, one at a time, with §6's bounds told beside them | before `T1`; `S0`, `P1`, `K7` and `K8a` wait on none of them |
| `O2` | `finding-triage.md`: the entries are written once, to a file, each naming the numbered findings it folds (`K2`); the kit's call in *Read first* (`K1a`); a kind with its evidence on every entry and the rule that assigns it; red results and ungraded rows as sources, a red one under its own key; *a finding without a block is graded unclear* becomes *one that quotes no printed route and carries no commands is a lead*; the sentence `wf-r-m5` owes | before `T5a` — additions, inert until then |
| `M3` | The third lane (`K7`, `K8a`) is merged into stream T's branch, by the git agent, `--no-ff` | before `K8b` |
| `O3` | `finding-verifier.md`: the verdict and what is handed on go into a file, filed by the call the prompt spells, **after the report**; a handed-on item is an entry with its kind's evidence, in the order the verifier would have them verified; the refutation's word and `undrivable`; the kit's call. **The paragraph the fence pins (F17) is left standing**; the new sentence is added beside it | before `T6a` |
| `O4` | `milestone-e2e-tester.md` and `milestone-code-reviewer.md`: the paragraph *What a finding carries in a stabilization run* — it stands in exactly these two — gains the kind's evidence; the kit's call; the tester's gains the hunt's contract, bound on the line `T8` adds | before `T8` |
| `O5` | `finding-verifier.md`: the pinned paragraph goes, now that `T6a` moved the pin. `robust-advocate.md`: its stabilization paragraph names the contested fix. `stabilize-scope.md`: its left-open entries carry a kind's evidence. `stabilize-preflight.md`: it builds no image and waits for none (`K7b`). `build-git.md` is read, and edited only where it names a step that moved | after `T12` |
| `KR` | the six definitions that hold `Write` gain the edit tool, and the report paragraph changes in all eight of its copies (§5) | after `O5` |
| `O6` | `CLAUDE.md`: one sentence each for the committed site list and for the route catalog | after the join |

No edit of `.claude/settings.json` is owed: the hunter reads a committed file, and every new call is a call of a tool that is on the allow list already.

### Stream P — the product's tests

Order: `P1` → `P2` → `P3` → `P4` → `P5`. **It writes no product source**; a case that cannot be written without a change under `crates/*/src` is a halt. **A red case is not fixed here.**

**`P1` — the frame, proved on the regression the existing test misses.** *For:* ruling 3.
- *Writes:* `crates/cli/tests/route_catalog.rs` (new), `crates/cli/tests/support/route_case.rs` (new) and its line in `support/mod.rs`, the suite's line in one group root, `LOG`.
- *What:* a case is data — the site's id as `LIST` holds it, the cause in words, the state that provokes it (built from a bare repository by the binary under test and by file acts), the refusing argv, the acts that follow the route, the outcome that must hold, and the files outside the repository whose bytes must not change. **An act's operand is a span of the printed route, by index.** An act on an object the route does not print is a second, marked form — **and the runner refuses it wherever the printed route holds a path span**, so it cannot stand in for the printed operand. The runner isolates `HOME` and git's configuration as [dev-workflow.md](../../../../../implementation/dev-workflow.md) → Gate has the faithful mask, places a file it will execute as that section's Linux rule asks, and classifies: *not provoked* · *provoked, no route printed* · *followed, holds* · *followed, holds, operand implied* · *followed, the same refusal again* · *followed, a witness changed* · *followed, another failure*. **An expected-red entry is (case, key, the classification expected)**, strict both ways: a listed case that is not red fails, a red case that is not listed fails, **and a listed case whose classification moved fails** (`S3`); one arm reports every difference.
- *Done when* — `route_catalog::`, red first: the first case is **`setup.install-hook`, the hook a link to an existing script outside the repository** — written, run and seen red (*a witness changed*) before the list holds it; then listed under the canary's key for it with that classification. Its control, the dangling link, is *followed, holds*. `no_act_takes_a_path_and_the_marked_form_is_refused_where_a_path_is_printed` — over the suite's own source and one planted case. One mutant, red: the first case given the link's own path by a constant — the existing test's step — turns it green, and the strict list fails it.
- *Must not:* touch an existing suite; drive the compiled-in binary through `support::trial_corpus`; remove a path built from variables.
- *Depends on:* `S0`. *Who:* a builder.

**`P2` — the fence.** *For:* ruling 3; §2.3; `S5`.
- *Writes:* `crates/cli/tests/route_catalog.rs`, `LOG`.
- *What:* for the catalogued families — `setup`, `adapter-guide`, `uninstall` — every blocking site `LIST` holds has a case, a stated reason (`not-constructible: …` · `platform: …` · `no-route-printed`) or a row on an **owed** list that names its task. **And the vocabulary is held, not only the call sites:** every code-shaped literal with one of those prefixes anywhere in production source is the code of a listed site, or is claimed by a row that names the `dynamic` site that mints it, or carries a reason why it is no finding's code. That is what finds `setup.dirty-install-path`, which no constructor call spells (F13).
- *Done when* — `every_refusal_site_of_the_catalogued_families_has_a_case_a_reason_or_an_owed_row` (red against an empty table; the output kept); `every_code_of_a_catalogued_prefix_is_a_listed_sites_or_is_claimed` (red on this tree until the engine's site is claimed); `a_row_for_a_site_that_is_gone_is_named`; `the_fence_is_not_vacuous`.
- *Depends on:* `P1`. *Who:* a builder.

**`P3` — the cases of `setup`.** **`P4` — the cases of `uninstall`.** *For:* ruling 3; §2.4.
- *Writes:* `crates/cli/tests/route_catalog.rs` (and a sibling per family if the table outgrows one file), `LOG`.
- *What:* per site, every cause the builder can construct, found by reading the site's callee; each site's row says which functions were read and how many causes they hold. A red case is listed **only with a key**: the canary's, where its findings file has the finding; else one the builder mints and writes, with the classification, into the pilot's findings file. A doubtful reading of a prose route is not decided: the case follows the printed words most literally, is marked `doubt`, and is listed for the human.
- *Done when:* the owed list shrinks by the family's sites and, after `P4`, is empty and deleted; the strict list holds; **one run of the suite in `dev/runner-faithful` on the committed revision** is green and its line is in the commit (lessons 4 and 14); the suite's seconds in the gate are in the commit. **`P3` reports after its first ten sites** — cases, minutes, the Linux run — before the next ten are briefed; a site that takes more than about an hour stops the task, and that number goes to the human as the pilot's first result. Each task is cut into commits of about ten sites, each behind its own gate.
- *Must not:* assert a route's wording; skip a cause silently.
- *Depends on:* `P2`. *Who:* a builder each.

**`P5` — the previous release's table, and the pilot's numbers.** *For:* ruling 3; `wf-route-catalog`; §2.5.
- *Writes:* `crates/cli/tests/route_catalog.rs`; `completions/artifacts/M55/stabilization-build/reshape/route-catalog-pilot.md` and `route-catalog-findings.md` (new); `implementation/pinning.md` (one section); `LOG`.
- *What:* one ignored arm that runs every case with the binary an environment variable names — the states built by that same binary — and prints each case's classification there. It asserts only that it ran, and **prints the two classifications side by side with no word *regression***. The record: **the frame's fixed cost (`S0`, `P1`, `P2`) apart from the cost per case**; cases, sites, causes per site with their derivations; reasons by word; cases red by classification; routes that print no operand, and of those how many hold when the natural act is done; cases marked `doubt`; the suite's seconds; and, from the orchestrator, minutes and output tokens per case. **It decides nothing about the other families**, and nothing about what *works as printed* means for a route that prints no operand — that number is what the human decides it on.
- *Done when:* the arm was run against a `1.0.0-rc.24` binary built from its commit in a scratch root, and its table is in the record; every number names the command or the file it was read from.
- *Depends on:* `P4`. *Who:* a builder; the usage numbers are the orchestrator's.

### The join

**`J1` — what the two streams owe each other.** *Writes:* `tooling-tests/route_catalog_expected_red.rs` (new) and its line in `tooling-tests/groups/g_tooling.rs`; `LOG`. *Done when:* `every_expected_red_key_is_a_row_somebody_holds_open` — in the canary's findings file, in the pilot's, or a row of an opened run's ledger that is not `fixed`; a key whose row is `fixed` while its case is listed is named; `the_files_the_hunters_prompt_names_exist`. *Who:* a builder.

**`J2` — the record.** *Writes:* `PEND` (`wf-route-catalog`: the pilot landed, its numbers by pointer, the decision owed), `completions/artifacts/M55/stabilization-build/README.md`, the lessons list, `LOG`. With it `O6`, and **a `milestone-code-reviewer` over the build's whole diff before any stage is run with it.** *Who:* a builder for the record; the review is the orchestrator's to launch.

---

## 5. The pre-built pieces — taken into this build

The third count (`prebuild-census.md`, beside this plan) ranked what the canary's agents spent their turns on. **The human ruled on 2026-10-11 (option A): seven of its candidates join this build**; the rest are rows (§7). `O0` records that ruling where a builder can read it. Each task below says what of that count it is expected to remove, **so that the first supervised run can check the saving** — the figures are the count's, for the canary's first invocation.

**Where they stand.** The script half of each comes before the harness task that uses it, and is of phase A — a writer or a read nobody calls yet. The harness halves ride the task that opens the same lines. Two — the image's build and the regression tool — write files no other task writes and run in **a third lane**, merged into stream T by `M3` before the first task that needs them.

**`K2` — triage types each entry once (candidate 2), the script's half.** *Removes:* the second typing of every entry. The count's 129 k output tokens is **for the canary's four passes and their superseded writes together — about 25 to 35 k a triage**, and a stage now has one; the larger gain is that triage's return becomes a digest, which closes `wf-return-size` for it. *Phase A.*
- *Writes:* `REC`, `LOG`.
- *What:* a writer for triage: it hands over **one file of entries** — key, new or found again, doctype, door, clause, repro, grade, kind with its evidence, why, and **which handed findings it folds, each named by its source and its number** — and the script renders the triage's report from it, files it as that reporter's report, and holds the partition: the caller passes, as plain words, how many findings each source handed, the prompt numbers them 1 to that count, and every number of every source is in exactly one entry. Its line is a digest: keys, grades, classes, counts.
- *Done when* — `dev_stabilize_record::`, red first: `triage_hands_over_its_entries_once_and_its_report_is_rendered_from_them`; `a_numbered_finding_in_no_entry_or_in_two_is_refused_and_nothing_is_written`; `a_rendered_report_is_a_report_like_any_other`; `an_entrys_text_with_a_denylisted_term_refuses_the_filing_and_names_the_entry`.
- *Its harness half is a named part of `T5a`;* its definition half is `O2`.
- *Depends on:* `T4`. *Who:* a builder.

**`K3` — the record's batch is composed by a tool (candidate 3), the scripts' half.** *Removes:* 118 k output tokens and 13.4 minutes of wall time **a record** — the executor re-typed its 232 KB prompt, verbatim — and under `Q1`'s recommended answer a stage has two. *Phase A.*
- *Writes:* `REC`, `STEP`, `LOG`.
- *What:* a `test` stage's record is composed by the script from what is already filed with it — the reports on disk, triage's entries (`K2`), the held checks' verdict files, the filed verdicts (`T3b`) — and from **plain words** for the rest: an item's status with, for a void, a reason of the shape a held check's has (`<word>` or `<word>:<reporter>`); the round's facts; the reporters the stage launched. **Those words are held to a hash, as the typed batch is today** (`HARN` 2090; `REC` 820, 4579): the harness computes a sha256 over their canonical form and passes it; the script computes it again over what it was handed and refuses a difference — a step agent that typed `green` for `void` is caught, not recorded. **The list of launched reporters stays the harness's**: a list derived from the disk would make *missing* and *extra* mean nothing. A red check that is not held brings no free text: its door is the item's own, its repro the preflight's report by path.
- **What it does not cover:** a record of the human's rulings, which carries free text — that record keeps its executor; and every record of `fix`.
- *Done when* — `dev_stabilize_record::a_test_stages_batch_is_composed_from_what_is_filed_and_from_plain_words`; `::words_that_differ_from_their_hash_are_refused_and_nothing_is_written` (one status changed, one reporter dropped, one fact changed); `::a_composed_batch_is_what_the_typed_one_was` — on the suite's fixtures, the same tables as today's batch for the same stage; `dev_stabilize_step::a_stage_killed_while_its_batch_is_composed_ends_in_one_of_two_states`.
- *Its harness half is a named part of `T5b`* (the first record) *and of `T6a`* (the second): on the new path the record step is tool steps only — simulation: `a_test_stages_records_are_composed_by_the_tool_and_no_executor_is_launched`; `a_step_that_types_one_status_wrong_is_refused_by_the_hash`; fence, arm (n), for a record with no executor.
- *Depends on:* `K2`. *Who:* a builder.

**`K1a` — the lookup kit (candidate 1), the script's half.** *Removes:* up to 254 k output tokens (a ceiling), 31.3 M cache-read tokens and 54.8 agent-minutes — 283 of the verifiers' 2,095 turns did nothing but look up the report's rules (97 agents, 196 calls), the opening (95), the repro format (85), the rig's help (81), the exit rule's text (76). *Phase A.*
- *Writes:* `REC`, `LOG`.
- *What:* one read that prints, for a role, what every agent of it looked up: the report's rules, the rig's states and how a binary is named to it, the repro block's format, the slug rule — and **the closing condition's own text, read from the entry the opening names**. Ruling 14 has the opening hold the condition by reference, and it stays so: the opening may name that entry's heading as a fact, and the kit prints the entry from the log by its heading; where the fact is absent the kit says where the opening points. Nothing is copied into the run's record. The prompt keeps what the harness holds — the two binaries with their hashes, the scratch root, the key — and names this call as the first. *Why a call and not the text in the prompt:* the harness holds no file, and the rules written into the script would be their second home.
- *Done when* — `dev_stabilize_record::the_kit_prints_what_a_reporter_looks_up_and_nothing_of_a_finding`; `::the_kits_parts_are_read_from_their_homes` — the report rules from the script's own help, the repro format from the section of `pinning.md` that owns it, the condition from the log, each by its heading, and a heading that moved is named; `::a_run_that_names_no_condition_is_told_where_the_opening_points`.
- *Depends on:* `K3`. *Who:* a builder. **It stands before `O2`, so that each definition's *Read first* paragraph is edited once.**

**`K1b` — the lookup kit, the harness's half — and the report line of `KR`.** *Phase B.* *Writes:* `HARN`, `SIM`, `FENCE`, `LOG`. Every driving role's prompt names the kit as its first call. **And the harness's own report line changes with the definitions' paragraph** (`HARN` 1095: *write your report, whole … WITH YOUR FILE TOOL*): the report is written once, when the driving is done, and corrected with the edit tool. *Done when* — fence: `every_driving_roles_prompt_names_the_kit_as_its_first_call`; arm (s) holds it as one plain invocation; arm (r): the kit's text is in no prompt; arm (d): the report line and the paragraph that binds on it agree; the self-test's report-line check is carried; simulation: the stand-ins run the call. *Checked at the first supervised run:* a verifier's turns before its first state or drive (median 6.5 of 32.5), and its turns of nothing but look-ups (13.5%). *Depends on:* `T7`. *Who:* a builder.

**`K8a` — the regression set's baseline is kept (candidate 8), the tool.** *Removes:* 10.9 of the regression set's 22.3 minutes on every run after the first against one previous release — 653 of 1,319 seconds depend on that release's commit alone. **Third lane.**
- *Writes:* `dev/regression-set`, `tooling-tests/dev_regression_set.rs`, `LOG`.
- *What:* the previous release's unpacked tree, its built tests and its baseline report are kept in a directory the caller names and used again. **It is of the class that records falsely** (lesson 11), and **the guard is this task's and nobody else's**: a kept baseline is used only when all of these are what it was kept with — the previous commit; the list's exclusion rows; **the tool's own bytes**; the test runner's version; **the compiler as resolved** (what it prints of itself, not the toolchain file); the host's git; and the hash of every test binary and of the baseline's own `jigc`. Anything else is refused and built afresh, and the line says which.
- *Done when* — `dev_regression_set::`, red first: `a_kept_baseline_is_used_and_the_previous_release_is_not_built_again`; `a_kept_baseline_is_refused_for_each_thing_it_was_kept_with` — one arm a member of the key, eight in all; `a_kept_baseline_whose_binary_was_replaced_is_refused`; and **one real run kept and one unkept, whose lines are equal** — both in the record.
- *Must not:* change the swap or its proof. *Who:* a builder.

**`K8b` — the stage names the kept baseline.** *Phase B.* *Writes:* `STEP` (one flag of the hold), `HARN` (the flag), `SIM`, `LOG`. *Done when* — `dev_stabilize_step::the_regression_hold_passes_the_kept_baseline`; simulation: `a_second_stage_against_one_previous_release_names_the_same_baseline`. *Depends on:* `T5b`, `M3`. *Who:* a builder.

**`K7` — a base layer for the trial image (candidate 7).** *Removes:* about 21 of 24.7 minutes of wall time for every candidate with a trial arm after the first — 1,256 of the build's 1,285 seconds are the same for one toolchain file and one lockfile. **Third lane.**
- *Writes:* `completions/trial-harness/` — the image's build, its file, the two verifiers; `LOG`.
- *Done when* — red first, in the harness's own checks: `a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer` (its compile step timed, in the record); `a_changed_lockfile_or_toolchain_file_builds_a_new_layer`; the seven checks of the image and the check of a pair still hold, and the candidate's binary in the image is still the commit's, by hash.
- *Who:* a builder.

**`K7b` — the image's build is a held command.** *For:* `wf-image-hold`, whose trigger has come: its row is due *before the run's opening if its session asks for permissions*, the session the human drives may ask, and round 1's trial arms need the image whatever the canary does. A preflight that waits for the build with a loop of its own is a prompt there. *Phase B.*
- *Writes:* `STEP`, `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* a kind of the step tool's hold that builds the trial image of a commit and verifies it — the tool's verdict is the tag and that the seven checks hold; the stage starts it and waits for it by tool steps, as it does a build, and the second preflight builds no image and waits for none.
- *Done when* — `dev_stabilize_step::the_verdict_of_an_image_build_is_the_tag_and_its_verification`; `::an_image_build_that_fails_or_is_not_verified_is_void_by_a_word`; simulation: `the_trial_image_is_held_by_acts_and_no_agent_composes_a_wait`; `a_stage_without_an_image_voids_its_arms_and_goes_on`; fence, arm (s): no prompt of a preflight names the image's build. `O5` follows in `stabilize-preflight.md`.
- *Depends on:* `T8`, `M3` (it calls what `K7` changed). *Who:* a builder.

**`K6` — the scripted trial arm is a held check (candidate 6).** *Removes:* 102 k output tokens, 23.8 agent-minutes and three agents for every scripted arm. *Phase B.*
- *Writes:* `STEP`, `REC` (the held kinds), `HARN`, `SIM`, `FENCE`, `LOG`.
- *What:* a kind of the step tool's hold that runs a scripted arm in the trial image and reads the scorer's own verdict; an item of that kind is run by no chain. The blind arms stay agents.
- *Done when* — `dev_stabilize_step::the_verdict_of_a_scripted_arm_is_the_scorers_and_is_read_by_the_tool` (green, red, a run that left no evidence: void by a word); `::a_scripted_arm_writes_only_under_the_scratch_root`; simulation: `a_scripted_arm_is_held_and_no_agent_runs_it`; fence, arm (s) and the held kinds' table.
- *Depends on:* `K7b` — **the arm runs in the image `K7` builds and `K7b` holds**, so it comes after both. *Who:* a builder.

**`KR` — a reporter corrects its report instead of retyping it.** *Removes:* about 231 k output tokens in the canary's first invocation — 31 of 80 verifiers wrote their whole report more than once, 603 k superseded characters.
- **Why reports are retyped — established.** It is **not** that a refused report can only be rewritten whole: the record script refused none of the 80. Three things, read from the definitions, the harness and the script: **(1)** the six roles hold `Write` and no edit tool (F16), so the only way to change a file they wrote is to write it whole; **(2)** the paragraph all of them repeat, and the harness's own report line, say *write the whole report with your file tool*, and nothing about when; **(3)** 28 of the 32 repeated writes followed further commands — the report was written before the driving was done. A handed-over report cannot be corrected at all (`exists`) — which is right and is not the cause: every rewrite preceded the hand-over.
- *Two halves.* The orchestrating session's: the tool line of the six definitions, and the repeated paragraph in all eight of its copies. The human's ruling of 2026-10-08, which gave those roles `Write` because a role that hands over a file must be able to write one, covers the tool that corrects it; nothing is asked. The harness's: the report line, in `K1b`.
- *Done when:* fence, arm (e) — the paragraph is identical wherever it stands; arm (d) — it agrees with the harness's line; arm (a). *Checked at the first supervised run:* superseded report writes per verifier, read from the transcripts — no test can hold what an agent types.

## 6. The questions

### For the human — three, in the order they depend on each other

**`Q1` — May a `test` stage record twice, so that a verdict is kept the moment it returns?**
- *The fact that forces it.* Today nothing a stage finds is on record until its one record at the end; that is why the canary's second invocation lost 19 verdicts, 6 of them confirmed. A record is one commit behind one full gate (22.6 minutes on the canary), so it cannot be made per verdict — and a verdict needs a ledger row to belong to, which today does not exist until that record.
- *The options.* **(a)** Two records a stage: one after triage, one after verification; between them each verifier files its verdict through the record script. A killed stage loses nothing; a continuation costs only what is missing. Cost: one more gate a stage, about 25 minutes, and one more push. A filed verdict is on this machine's disk, not committed, until the second record. **(b)** One record, verdicts filed to disk all the same: no extra gate — but a stage killed before its record has no rows, so the instruments and triage run again and the filed verdicts match nothing. That is things running twice. **(c)** Rows written early and left uncommitted until the end: no extra gate, and a new in-between state in the tool that reconciles a killed write — the class the scale ruling repairs before first use.
- *Recommendation:* **(a)**. With `K3`, which the human has taken, neither record is typed by an agent any more: a record is its gate and about two minutes.

**`Q2` — What becomes of what a verifier leaves open?** *(Depends on `Q1`: the filing is how it arrives.)*
- *The fact that forces it.* The ruling says follow-ups are verified inside the budget and that triage runs once — and the correction took away the replay tool, the only filter it had for follow-ups. In the canary every follow-up went through a triage first: it merged one in seven and sent on 23%, 52% and 56% of the rest. Every rate the ruling rests on (15 of 30 route items confirmed; 9 of 18 under a confirmed parent) was measured **after** that filter. A verifier leaves six items; by the blind word count about 35% name a route (118 of 338 follow-up entries), and triage sent on 53% of those (62).
- *The options, for a budget of 30 on the canary's rates.*
  - **A — a small triage after every wave**, over only the items that could enter the ranked queue. **A triage pass cost about the same whatever it was handed**: 118 k, 158 k, 169 k and 160 k output tokens and 18.5, 25.8, 25.7 and 33.0 minutes for 54, 60, 88 and 235 findings, and 98 k and 14.5 minutes for 118 rows read back. Four waves a stage is therefore about **400–650 k output tokens and 60–130 minutes, in sequence** — a third to a half of what the stage's thirty verifiers cost — less about 30 k a pass with `K2`. It keeps the filter the rates were measured behind: about one route item enters per verdict, some 10–30 are left unreached at the stop, and what triage grades *needs a bound* comes to the human as today (13% in the canary). It is "triage once" given up.
  - **B — no triage; the script ranks on the kind the verifier backs with evidence.** No cost. About 1.8 route items enter per verdict: some 35 unreached at the stop of a canary-shaped stage, and **60–90 in a real round 1**, where the hunt alone fills the budget — the reviewer's estimate, which the tables bear out. Each holds the close until verified or ruled. Roughly half of what is then verified is what triage would have dismissed: up to about nine verifiers, 400 k tokens, a stage.
  - **C — B, and a verifier may hand on to the queue at most N items, each with the route line it saw printed or its commands.** No cost. The verifier, which has just driven the thing, is the filter. **What it noticed beyond N is recorded as what it is** — a route item stays a route item — **and is in no queue**: whether such an item holds the close is `Q3`. The queue's unreached list has a ceiling by construction — N for each verdict, 60 at N = 2, about 30–50 after the stage's own verifications. Not measured: whether a verifier's own two are the ones that confirm. Duplicates are verified twice, at most one in seven.
  - **D — nothing a verifier leaves open is verified in the stage it arises in; it waits for the next stage's one triage.** The measured rates hold, since that triage is the canary's. But the canary's findings were at the first to third hop, both regressions among them: each hop waits a stage, with its gates and records. It contradicts *follow-ups are verified inside the budget*.
- *What each does to "a route item nobody reached holds the close".* A: a list of 10–30 and the human's bounds. B: a list that grows by about two with every verdict. C: a ranked list with a ceiling the opening sets, and beside it the route items a verifier noticed beyond N — at the blind count's 35% of six, up to about one more for every verdict — which hold the close or do not as `Q3` rules. D: no list in the stage, and the findings a round late.
- *Recommendation:* **C, with N = 2 to begin with**, the number in the opening record beside the budget. The first stop's hit rate for the handed-on classes says whether the verifier's own choice confirms at the measured rate; if it is well below one in three, option A is the row to build (`wf-triage-per-wave`).

**`Q3` — Does a route item that is never to be verified hold the close?** *(Depends on `Q2`: it sets how many there are.)*
- *The fact that forces it.* The ruling says two things that meet: *a route item nobody reached holds the close until verified or ruled*, and *a follow-up of a finding that did not reproduce is never verified* (0 of 11 confirmed in the canary, 0 of 6 among route items). The second kind can never be reached. The same holds for any class the human takes off the order, and — under `Q2`'s C — for a route item a verifier noticed beyond its N.
- *The options.* **(a)** Only a route item in the queue holds the close; one that is never to be verified is shown at every stop and at the close, by kind, with its count, and the human may put any of them into the queue by key. **(b)** Every unverified route item holds the close: each of the never-verified ones needs a ruling by key. About one parent in seven did not reproduce, and under `Q2`'s C up to one more for every verdict is beyond N: some 8 to 40 rulings a stage of 30, and more under B.
- *Recommendation:* **(a)**.

**Told with the questions, and no question** — each follows from a ruling that stands or from a fact of the code:

- *A contested finding found in `test` reaches the human with its verifier's basis and no advocate's case* until the `fix` half is repaired (§2.9; ruled).
- *The account's limit is never seen by the script*, so the stage stops verifying after a wave with two silent calls (§2.2).
- *The route hunter's coverage count is the hunter's own word*: the ruling asks for it in the hunter's report, and nothing checks it but its shape.
- *Triage's grade still decides what is verified of what triage saw*: an instrument's finding, a red check or a seed that triage grades *no break*, *out of scope* or *needs a bound* is not queued, as today (ruling 5 of 2026-10-05, which the new rulings adjust by rank and budget only). Only what a verifier hands on enters without a grade.
- *A filed verdict is on this machine's disk until the stage's second record*; and who filed it is not something the script can know.
- *After the opening, every edit of the route catalog — a case, a reason, an expected-red entry, the site list — moves through a round*, because it is a product test (ruling 7, adjustment ii). A case cannot be tuned between rounds by a direct commit.
- *The unreached route items are ruled by key, many in one invocation*; a ruling over a class is a row due at the first stop.
- *Whether a finding an instrument finds again in a later round is verified again, and whether round N's unreached items compete for round N+1's budget*, are theirs before round 2's `test`; the build does what the record script does today — yes to both.
- *The order of classes is theirs at the opening*, with the count's rates beside it.

### Settled as an engineer — and how

| # | Question | How it is settled |
|---|---|---|
| `E1` | Where the site scan lives | Rust, on `support::rust_source`, with a committed list its own suite holds equal to the scan (`S0`). The hunter reads the list. No python in a product suite; no tool for a stage to call; no allow rule. The review's `S11`, adopted |
| `E2` | A site's identity and its command | File, enclosing function, code, ordinal. A command is its codes' prefixes; a `dynamic` site is unassigned, and for a catalogued family the code vocabulary is held as well (`P2`) |
| `E3` | Whose the budget is, and what spends it | The round's `test` stage's — the ruling's *per stage*; an invocation is not a stage, or a continuation would refill it. A filed verdict spends one; a launch that left nothing does not. After an interruption the launches can exceed the budget by what was in flight; the verdicts cannot |
| `E4` | Who says a kind, and what holds it | The row `wf-verify-fan-out` gives it to triage for what triage sees; for a handed-on item the verifier backs it with the printed route line or the commands, and the script enters it as a lead without them. It is frozen once the row has a verdict. No code is held to a vocabulary |
| `E5` | Does triage's grade still gate what triage saw | Yes: ruling 5 of 2026-10-05 stands for it, adjusted only by rank and budget. What is handed on is never graded in the stage it arises in (`Q2`) |
| `E6` | Who files a verdict, and what makes it count | The verifier, after its report, under the acceptance of `T3a`. Who called the script is not holdable — a declared bound, the same as for a report |
| `E7` | What stops verifying, the limit being invisible | A wave with two silent verifier calls is the last; no verifier call is tried twice. One stray death costs one unverified finding and no stop — a stop costs a record and a restart, about 35 minutes. At a real limit every call of the wave is silent |
| `E8` | `runFix` calls what `test` loses | One shape in one script: its unreachable calls point at the new functions; a row says its own verify shape is its repair's |
| `E9` | Which agent hunts routes | `milestone-e2e-tester`, with a paragraph bound on a line of the prompt |
| `E10` | The expected-red entry | (case, key, classification), strict three ways, in the suite; tied to a findings file by a tooling fence at the join |
| `E11` | A case on another binary | The suite's own runner, in one ignored arm; states built by the same binary; a table and no verdict |
| `E12` | What an act may be | A small vocabulary whose operand is a span's index; the marked form for an unprinted object is refused where a path is printed and is counted |
| `E13` | A route that prints no operand | Not decided before the pilot: *the natural act holds* is its own classification, green in the gate and counted apart; *the same refusal again* is red whatever the route printed |
| `E14` | A cause that needs another platform or a root caller | A stated reason, counted |
| `E15` | Group root; the wave; the stop's view | From the gate's timing record; at most eight, the queue read after each; a rendered file of the round |
| `E16` | The word for *an invocation continues* | `verify`, beside `triage` for a row nobody graded: the misnaming `wf-return-names` names is not made again |

**What moves if the human answers otherwise.** `Q1` (b) or (c): `T3a`, `T3b`, `T5b` and `K3` are recut and §3 changes — nothing before `T3a` moves. `Q2` A: `T6a` gains a triage call between waves and `O2` its prompt; B: `hand-on` and its arm fall away; D: `T6a` verifies only what the one triage sent on, and the continuation's triage takes the handed-on rows. `Q3` (b): one line of `T2b`.

---

## 7. What this build deliberately does not do

| Not done | Stays under |
|---|---|
| The replay tool | `wf-replay` |
| *Identical evidence inherits its row*; *a lead already listed is not listed again* (§2.7) | a new row, `wf-same-evidence` — due at the first stop, on what its ledger shows of duplicates |
| What a later round does with a finding found again, and with earlier rounds' unreached items (§2.8) | a new row, `wf-round-two-queue` — **due before round 2's `test`** |
| A ruling over a class of unreached items | a new row, `wf-rule-a-class` — due at the first stop |
| A triage per wave | a new row, `wf-triage-per-wave` — if the first stop's hit rate says so (`Q2`) |
| A code held to the scan's vocabulary; the hunter's count held to the list | a new row, `wf-evidence-held` — neither publishes unscanned, records a false verdict or strands state |
| Hit rates over the whole run; a budget lowered below what is spent | a new row, `wf-stop-view-later` |
| A usage limit met by a role that is no verifier: each call in flight is still tried up to three times before the breaker ends it | `wf-breaker-parallel`, annotated |
| A held gate's verdict held to its commit by the tool itself | `wf-held-gate-tree` — the harness's name now carries the commit; the tool's check is still owed |
| The other route families; what *works as printed* means for an unprinted operand | `wf-route-catalog` — the human's, on `P5`'s numbers |
| A fix of any defect a catalog case shows | the first run's rounds; seeded at its opening (`wf-canary-seed`) |
| Seeding the canary's findings, and how many of its leads | `wf-canary-seed` — the opening's; a seeded lead costs nothing |
| The drive that sends an instruction-shaped text through a return | `wf-neutralized-return` — **left, and still due before the opening**; the build shrinks its surface, since no verdict's text is in a return |
| Agents writing outside the scratch root | `wf-scratch-escape` — left: no ruling names it, and it is a sentence in every role's prompt |
| The gate, the regression set and the image side by side; a second record's gate beside the verify phase | `wf-prelude-serial`; a new row, `wf-record-twice` |
| The preflight's other two readings as held commands — CI's result for the commit, the packaged install | `wf-r-m8` — the image's build is taken (`K7b`), these two are not |
| The advocate and the drive beside a triage; the `fix` stage's verify shape and budget | `wf-fork-serial`, rewritten as the `fix` half's; a new row, `wf-fix-verify-shape` |
| The instruments' findings on record as they are found | `wf-b4` — the verdict half is built |
| The canary tool's other two faults; a fence that holds a definition to its prompt | `wf-canary-setup`; `wf-defs-fence` |
| A driver kit — twin rigs, cells with captured streams, a tree manifest around each (the third count's candidate 4: ≤ 145 k tokens typed by 40 of 61 verifiers) | a new row, `wf-pre-driver-kit` — **due at the first stop, with that round's numbers** |
| Evidence by reference in a report (candidate 5: ≤ 114 k) | a new row, `wf-pre-evidence-by-reference` — the same due point |
| A short instruction file for stage agents in place of the project's (candidate 9: 36–44 M cache-read) | a new row, `wf-pre-stage-instructions` — the same due point; whether an agent can be started without the project file is not established |
| The finding handed as its own excerpt (candidate 10) | a new row, `wf-pre-finding-excerpt` — the same due point |
| A byte matrix over the write doors for the exit rule's first clause; the regression set's second part (candidates 12 and 11) | two new rows, `wf-pre-byte-matrix` and `wf-pre-regression-part-two` — **parked until the route pilot's numbers** |

**The smaller rows.** *Taken, because "nothing runs twice" names it and it is the same code:* `wf-unverifiable-relaunch` (`T1`, `T6a`), `wf-return-names` (`T7`, `E16`). *Taken in part:* `wf-breaker-parallel` (verifiers; the breaker asked before every try — `T6b`), `wf-triage-reads-harness` (`T5a`, `O2`), `wf-fork-serial` (the removal — `T5b`). *Taken since the second review, because its trigger has come:* `wf-image-hold` (`K7b`). *Left a row:* `wf-neutralized-return`, `wf-scratch-escape`, `wf-prelude-serial`.

---

## 8. How it is verified before a human drives a real stage with it

Nothing here launches a stage. The human drives every run.

1. Every commit's own full gate, and CI's `ci-ok` read for each pushed tip (lesson 4).
2. The harness's self-test **by the script's path** (lesson 19): `Workflow({ scriptPath: '<the repository's root>/.claude/workflows/stabilize.js', args: { selfTest: true } })`. No agent is launched; the count it prints is above today's.
3. The fences and the simulation, in the gate: every scenario §4 and §5 name; arms (a), (d), (e), (f), (k), (n), (r), (s), (t). After `T12` the simulation holds one path, and every scenario of it opens a run as a real opening will.
4. A review of the build's diff by an agent that built none of it (`J2`), each finding checked before it is worked in.
5. A small canary of the reshaped stage (§9).

---

## 9. The small canary, redone

One `test` stage in a throwaway clone with a bare remote of its own, set up by `dev/stabilize-canary` after the join; the invocation is printed by the tool and made by the human in a session of their own.

**Two doors.** `jigc setup`, where the review row and the plants are; and **a command the pilot does not catalogue and that lists few sites** — `jigc task discard`, four blocking sites by the count, unless `S0`'s list says otherwise — for the route hunt. A hunter at `jigc setup` would find its sites catalogued and have nothing to do. **And one scripted trial arm, as a held check (`K6`)** — so that the one kind of hold nothing has run for real is run once where a mistake costs nothing; it needs the trial image, **which the stage holds as a command (`K7b`): no agent waits for it with a loop of its own, so a session that asks for permissions is not stopped by it**; its first build pays the whole 25 minutes (`K7` saves on the second).

**Budget 10, hand-on 2, and an order for the canary only:** handed-on route items under a confirmed parent, then seeded route items, then the hunt's route items, then the rest. The first wave is eight; the seeds are in it whatever the hunt returns; two verdicts are left for what overtakes.


**Every seed is graded at the opening** — `unclear`, graded by the human, as the ledger takes it — so that a plant is launched whatever the stage's triage would have made of it: the triage decides only what it is handed ungraded.
| Plant, a state seeded before the stage | Class | What must happen |
|---|---|---|
| the known regression at `setup.install-hook` | `route/seed` | wave 1; confirmed; **what its verifier hands on is verified in wave 2, ahead of every hunt item still waiting** |
| a claim about `jigc setup` that is false, quoting a route line | `route/seed` | wave 1; refuted as `does-not-reproduce`; what its verifier hands on is never launched |
| a claim that needs a Linux host and a root caller | `route/seed` | wave 1; `undrivable`; **no launch after that is filed** — a verifier of it that was in flight at the interruption is launched once more; on the human's list |
| a prose claim with no route and no command | `lead/seed` | no verifier, ever; on the leads list |

**One interruption, by the human:** when at least two verdicts of the first wave are filed and others are in flight, the session is ended and the stage invoked again.

**What it must show.** The first record's commit is on the clone's branch before the first verifier starts. One triage in the first invocation, none in the second. **Never more than ten verdicts filed over both invocations** — the launches may exceed ten by what was in flight at the interruption, at most six. The second invocation launches no reporter of an item, takes the filed verdicts, and launches no verifier for a key whose verdict is filed. The hunter's count beside the list's for its command. The return: `next: rule`, the budget spent, the hit rate per class, the four lists in a file. No advocate, no independent drive. **No record executor, and no agent for the scripted arm.**

**What the run is checked against, piece by piece** — read from its transcripts afterwards by a subagent, as the third count was made:

| Piece | The canary's first invocation | What this run should show |
|---|---|---|
| `K2` triage's entries once | each entry typed in the report and again in the return; 1.4 k output tokens an entry | one file of entries; the return a digest |
| `K3` the record composed | one executor, 118 k tokens, 13.4 minutes a record | no executor; a record is its gate and about two minutes |
| `K1` the kit | 13.5% of a verifier's turns only look things up; 6.5 turns before its first state or drive | one kit call; no grep for the exit rule, the report rules or the repro format |
| `KR` the report corrected | 31 of 80 verifiers wrote the whole report twice or more | no report written whole a second time |
| `K6` the arm held | three agents, 102 k tokens | none |
| `K8` the baseline kept | 22.3 minutes, every time | the first run pays in full; a second stage on the same scratch root is about 11 minutes shorter |
| `K7` the base layer | 24.7 minutes an image | the first build pays in full; shown at the next candidate |

**What it is expected to cost.** The first cut's table summed to 174–223 minutes, 50 agents and 0.93–1.09 M output tokens for what it priced, on the first canary's rates (a tool step 1.3 k; a reporter 59 k; a triage pass 151 k and 18.5 minutes; a verifier 50 k and 10.5 agent-minutes; a gate 23 minutes; a record's gate 22.6). **It left out three things, and the range with them is 190–330 minutes and 1.1–2 M output tokens** — about 60 to 105 USD at the first canary's price per output token, cache traffic included:

- the verifiers relaunched after the interruption: up to six, 0.15–0.45 M, a second wave of 10–30 minutes;
- **the hunter, which nobody has measured** and which may be the largest term: one verifier needed 17 rigs for one site;
- the human's own time between the two invocations.

The range prices the stage **as the first canary's agents worked**. With the pieces of §5 the two record executors fall away (about 0.1 M and 25 minutes), triage types less, and the trial image and its arm add 25 to 40 minutes of wall time and almost no tokens — so the minutes stay in the range and the tokens should come out in its lower half. That is an expectation, and the table above is how it is checked.

**How the hunter is measured early.** That is why the canary's hunt is at a command of four sites: its transcript gives output tokens and minutes per site provoked, at a cost bounded by four. Before round 1's opening writes a `route-hunt` item for a command, the orchestrator multiplies that rate by the command's sites in the list; a command whose product is more than the stage's verifiers together gets no item in round 1 and is said so at the opening.

**What to watch, and when to interrupt.**

- *A prompt for a permission:* stop. A clone is an untrusted workspace and ignores the committed allow rules; the human chooses the session's mode knowing that.
- *A verifier before the first record is pushed:* interrupt.
- *A second triage in the first invocation, any triage in the second, an advocate or a drive:* interrupt.
- *An eleventh verdict filed, or a verifier for the lead:* interrupt. **A launch beyond the tenth after the interruption is expected.**
- *After the restart, a reporter of an item, or a verifier for a key whose verdict is filed:* interrupt — something runs twice.
- *A second verifier for the Linux-and-root plant:* note it; the stage may finish.
- *The hunter past forty minutes or past 200 k tokens on four sites:* note the number; it is the measurement.
- *A line of the runtime saying it changed a return:* note the agent and the pattern.
- *Past five and a half hours, or past eighty agents:* interrupt.

---

## 10. Risks — where this plan is most likely wrong, and what would show it early

1. **The queue's arithmetic (`Q2`).** Every number for the options is the first canary's — one run, one door, a blind word count for *names a route*. *Shows:* the first stop's view; it is recorded before anything is tuned.
2. **The filing's trust (`T3a`).** The script cannot know its caller. *Shows:* the acceptance's eleven arms; the canary's restart.
3. **`T5a`–`T12` rewrite the most-patched function of the harness** (lesson 2), with two paths in one script from `T5a` to `T12`. *Shows:* each task's first scenario, red on the parent for the right reason; `T11`'s count of scenarios still on the old opening, which must reach zero before `T12` is briefed; a builder who cannot land an old arm's cell halts.
3a. **Phase A's promise — a run without an order holds the bytes it held — may be broken by a cell nobody thought of.** *Shows:* at once, in the same commit's gate: the simulation and the canary's suite are exactly that check, run unmodified.
4. **Two records double the exposure to a record step's halts.** *Shows:* the simulation's record scenarios against both; the canary.
5. **A verifier may not fill a structured file reliably.** *Shows:* a refused filing is repairable and sent again (`T3a`); the canary's first wave.
6. **Stream P's size is the pilot's own question** — some forty sites, causes unknown but for one; I/O failures behave differently for a root caller and on Linux; a planted hook git executes is the *text file busy* class. *Shows:* `P3`'s report after ten sites.
7. **The scan sees four constructor shapes.** *Shows:* `S0`'s cross-check on the one family that has a scanner; `P2`'s vocabulary arm, which is red on this tree until the engine's `setup` site is claimed.
8. **A strict expected-red list is a rule a fixer does not know.** *Shows:* the sentence in the doc and in `pinning.md`; the fixer's brief is the `fix` half's.
9. **The join.** `DECISIONS.md` is appended by every task; the hunter's prompt names stream P's files. *Shows:* the merge, and `J1`'s arm.

---

## 11. What this plan did not establish

- Whether the tree is green: no gate was started for it.
- What `1.0.0-rc.24` does over a hook linked to an outside script — read from the count's sentence (*it exited 0 and wrote through the link*), driven by nobody for this plan; `P5` drives it.
- The bodies of `build-git.md`, `stabilize-scope.md`, `stabilize-preflight.md`, `robust-advocate.md`, `milestone-code-reviewer.md` and `milestone-e2e-tester.md` beyond their tool lines and the paragraphs named here.
- Which other sentences of a definition the fence pins — one is known (F17); the rule in §4 covers the rest.
- The workflow doc sentence by sentence; `T10` owns it.
- How the report check treats a third kind of file beside the reports — `T3a`'s builder reads `check_reports` and `run_file` first.
- What a triage of only a wave's handed-on items would cost: the canary's five passes each cost 98–169 k whatever they were handed, and none was that small.
- How many handed-on items quote a route: the 35% is two words searched for in keys and in triage's lines, not a reading.
- The code prefix of `jigc task discard`'s family, and whether it has four sites; `S0`'s list says.
- What a route hunter costs; what the runtime does at a usage limit beyond one observation.
- Whether the pre-built pieces' savings hold outside the first canary — the third count's own bounds.

---

## 12. The review, finding by finding

Each was checked at the lines it cites before it was worked in.

| Finding | What the code or the record says | Disposition | Where |
|---|---|---|---|
| `B1` a row that appears after the one triage has no grader | **True.** `result_set` files `red-<item>` ungraded and open, *so that the next triage grades it* (`REC` 143–149, 2561–2567), in the batch after triage; ungraded routes to `triage` (`REC` 3120). The first cut's continuation ran no triage | **Accepted.** Red results are a source of the one triage and their rows are filed with a class; `next: triage` is kept for exactly a row nobody graded, and the continuation grades those and no other; three arms, a re-run's among them | F5; §3; `T1`, `T2b`, `T5a`, `T6b` |
| `B2` nothing says what makes a filed verdict count | **True.** A verdict counts only with its report today (`HARN` 3026–3027, 3094); the first cut had no arm for a filing without one, and `check_reports` had not been read | **Accepted.** The acceptance is stated before the task — report first, the reporter's name is the key's, the attempt begun, the round's binary, once per round — with an arm each; what cannot be held is named. `T3` is cut in two at the line between the filing and the record | `T3a`, `T3b`, `E6` |
| `B3` follow-ups skip triage, as a premise | **True**, and the first cut's `H2(a)` was false for follow-ups: triage merged one in seven and sent on 23%, 52%, 56% (the canary's record, §4). Counted for this revision: 118 of 338 follow-up entries name a route by the blind words, 62 of them sent on. The 60–90 holds for a round 1 whose hunt fills the budget; a canary-shaped stage gives about 35 | **Accepted** — the human's `Q2`, four options, each with its cost and its effect on the close | §6 `Q2`; `T2a`, `T3a` |
| `B4` the canary cannot show what it must | **True**, all three: seeds ranked below the hunt; relaunches after a kill against *never more than eight*; a hunter told to skip what the pilot covers at the pilot's own door | **Accepted.** An order with the seeds first and two verdicts left for what overtakes; verdicts counted, not launches; the hunt at a door the pilot does not cover | §9 |
| `S1` `H10(a)` misclassifies its own first case | The count says of `1.0.0-rc.24`: *it exited 0 (and wrote through the link)*. Not driven here either | **Accepted.** The pilot prints both classifications and no word *regression*; the question is gone | §2.5; `P5` |
| `S2` `H11(c)` would un-red a confirmed class | The canary's record, §5: *four causes draw the writability route while it is writable; the re-run is identical* | **Accepted.** *The same refusal again* is red whatever was printed; only *the natural act holds* is counted apart. Decided after the pilot, not before | `P1`, `E13` |
| `S3` the expected-red entry hides a worsening | True of (case, key) | **Accepted**: the entry carries the classification; that the tie certifies itself before the opening is said | `P1`, §2.6 |
| `S4` *route item* narrowed to *names a code* | The count, §2: about 17 free-text next steps, 3 hand-written route lines and 12 usage tips are refusals with a route and no code | **Accepted.** The evidence is the printed route line; the code where there is one. The vocabulary check the first cut promised and did not test is a row | §3; `T1`, `E4`, §7 |
| `S5` the fence misses `setup.dirty-install-path` | **True**: `crates/engine/src/finalize.rs` 2144, through a `match`; also `setup.unverified-install-path` (2640) and `uninstall.staged-prose` at `crates/cli/src/task.rs` 1329. The first cut's F12, *one module*, was false | **Accepted.** The code vocabulary of a catalogued prefix is held, not only the call sites | F13; `P2` |
| `S6` `O3`'s own gate goes red | **True**: `FENCE` 1320–1328 pins the sentence | **Accepted.** `O3` adds and leaves the paragraph; `T6a` moves the pin; `O5` removes it; a rule for every removal | §4, the orchestrator's steps |
| `S7` no step commits the record | Was true; done since — `d591f656` | **Accepted, discharged** | header |
| `S8` *verified once in a run* is the human's | `REC` 274–279: a finding found again is recorded again | **Accepted.** A row due before round 2; the build keeps today's behaviour and says so | §2.8, §7 |
| `S9` only ranked route items hold the close | True of the first cut's `T2`, and the ruling says both things | **Accepted** — the human's `Q3` | §6 |
| `S10` the order was labelled *as ruled* | The ruling fixes four things; the census has route under a not-a-break parent at 7 of 12 | **Accepted.** No default ships; the order is the opening's, with the rates beside it | §3 |
| `S11` a non-product file inside the product gate's verdict | `support/rust_source.rs` 8–16 warns against a second reader; the first cut's fence ran a `dev/` script | **Accepted — adopted whole.** A Rust scan on that reader, a committed list its suite holds, the hunter reads the file; the allow-rule question and the python arm are gone | `S0`, `P2`, `T8`, `E1` |
| `S12` two places where nothing-runs-twice is not built | **True**: `HARN` 3461, 3500 name the gate and the held checks by attempt | **Accepted in part.** The names carry the commit (`T5b`, with its arm). *No retry at once* for other roles is not built: bounded by the breaker, annotated on its row | F9; `T5b`, `T6b`, §7 |
| §2, claim 1 | True; a filed verdict is uncommitted | **Accepted**: the durability is said, in the claim and in `Q1` | §2.1 |
| §2, claim 3, *the table is clean* | Overstated — `S5` | **Accepted** | §2.3 |
| §2, claim 5, *no road* | Overstated: `dev/regression-set` swaps a binary under built tests | **Accepted**: reworded; the ruling's sentence is a table run once by hand | §2.5 |
| §2, claim 7 | The census counts shapes, not sameness; the first cut built the mechanism anyway | **Accepted.** Said as this plan's inference; the mechanism is a row and not a task | §2.7, §7 |
| §2, claim 11 | Overstated: `args.rulings` is a list (`HARN` 914–958) | **Accepted**: what is missing is a ruling by class | F6; §2.11 |
| `H1`–`H11`, the review's read | — | **Accepted.** Eleven became three; the rest are settled as an engineer or told as bounds, each with how. `H7` gains the two-in-a-wave option and takes it; `H8` is ruled; `H9` falls with `S11` | §6 |
| *Decided silently:* `E8`, `E16` | The ruling asks for the count in the hunter's report; ruling 7 puts product tests behind a round | **Accepted in part.** Neither is a question — each follows from a standing ruling — and both are now told to the human in so many words | §6, *Told with the questions* |
| §3, *a lead already listed* has no task; who writes the hunt items | True | **Accepted.** One meaning of *lead* — a row; the rule is named as not built; the opening writes the items | §2.7; `O3`; `T10` |
| §3, rows instead of tasks | — | **Accepted**: identity of evidence, run-wide rates, the lowered budget, the code list. None guards the three classes | §7 |
| §4, the cut | `T2`, `T3`, `T5` too large; `P1`'s arm cannot be a test; `T9`, `T10` name none; the branches' precedent | **Accepted.** Cut lines fixed in the plan; `P1`'s arm is a scan of the suite's own source; `T9` names its arms and `T10` is said to be a document task; `724e4ce7` cited | §4 |
| §5, stream P | The frame's fixed cost apart from a case's; forbid the marked form where a path is printed | **Accepted** | `P1`, `P5` |
| §6, the estimate | The first cut's rows sum as the review says; three terms left out | **Accepted**: 190–330 minutes, 1.1–2 M, with what it leaves out and how the hunter is measured early | §9 |
| Notes: `next: triage` meaning *verify only*; a hunter with zero listed sites | True of both | **Accepted**: `verify` is its own word; the hunter's prompt says a family may list none | `E16`; `T8` |
| Notes: F15, *no tooling test names `cli::` today* | True, and moot: the scan left the tooling suites | **Accepted** | `S0` |

**Rejected: none.** One finding is narrower than it reads: `B3`'s 60–90 is the size of the list in a real round 1 and not in a stage of the canary's shape, where the same tables give about 35 — both numbers are in `Q2`.

### The second review — of the revision

One new blocking problem and a set of should-fixes; each checked at its lines.

| Finding | What the code or the record says | Disposition | Where |
|---|---|---|---|
| **Blocking (a)** — a stage refused as not fit turns the simulation red | **True.** The refusal returns before any agent (`HARN` 2682–2685), and all 43 scenarios run the committed harness through `test` | **Accepted.** No stage is refused: the new path is built beside the old, chosen by the state, and a step not built yet is a refusal of that path which its own scenario asserts | §4, *The migration order*; `T5b` |
| **Blocking (b)** — red from `T1` on | **True.** The simulation opens a run through the real `run-set` and asserts `not_ready: []` (`SIM` 462–500); so does the canary's suite (808–818); `KNOWN_NEXT` is `fix`, `rule`, `close` (`HARN` 528); the script tasks' write-sets held neither suite | **Accepted.** Phase A: every new answer is computed only in a run whose opening holds an order, every new field is optional, and a task carries the assertions it breaks. Phase C makes the facts owed and moves every fixture in one commit. The table in §4 says, task by task, what the two suites do and why the gate is green | §4; `T1`–`T4`, `T11`, `T12` |
| `B1` residual — the red key; a red check that is not held | `result_set` mints `red-<item>` (`REC` 2564); a check the preflight reads has no verdict file | **Accepted.** The red source is handed under that key and returned under it; the evidence of one that is not held is the preflight's report; an arm each | `T1`, `T5a` |
| `B2` residual — a filed verdict has no home | **True.** `check_reports` counts it as `extra` (`REC` 3890–3899), `run_file` refuses it (4171–4172), `set_aside` takes a report or a scope only (4474–4476) | **Accepted.** A directory of its own, `r<N>/verdicts/`; `run_file`, the vet and `set_aside` learn the class | `T3a`, `T3b` |
| `B2` residual — a dead attempt's report is held by nothing | **True**: it is `other_attempts` (`REC` 3894–3895) | **Accepted.** The filing stores the report's sha256 and the record holds it | `T3a`, `T3b` |
| `B4` residual — a seed may never launch | True under `E5`: a seed triage grades *needs a bound* is not queued | **Accepted.** The canary's seeds are graded at the opening; `T9`'s arm reads grade and class | §9; `T9` |
| `B4` residual — *no second launch* | False if the verifier is in flight at the interruption | **Accepted**: none after `undrivable` is filed | §9; `T6a` |
| `B4` residual — the arm brings back the image's composed wait | True, and it is `wf-image-hold`'s own trigger; round 1's arms need the image whatever the canary does | **Accepted — settled by building it**: the image's build is a held command | `K7b` |
| `K3` — the words are unhashed | **True**: today the batch is held to the hash of the text the harness composed (`HARN` 2090; `REC` 820, 4579) | **Accepted.** A sha256 over the canonical words, computed by the harness and again by the script; an arm for one word typed wrong. The launched list stays the harness's. What `K3` does not cover is said: a rulings record, and `fix` | `K3` |
| `K2` — a partition needs ids; the saving | The count's 129 k is for four passes and their superseded writes (`prebuild-census.md`, candidate 2) | **Accepted in part.** Counts per source are enough **because the prompt numbers every handed finding** and an entry names source and number — said, and held by an arm. The saving is corrected to 25–35 k a triage | `K2`, `T5a` |
| `K8` — the guard, the key, the merge | True of all three | **Accepted.** The guard is `K8a`'s; the key gains the tool's bytes, the runner's version, the compiler as resolved and the host's git; `M3` merges the lane before `K8b` | `K8a`, `M3` |
| `KR` — the harness's own line | **True**: `HARN` 1095 says *write your report, whole … WITH YOUR FILE TOOL* | **Accepted.** A harness half, in `K1b`, with the fence's arm (d). No question to the human: the ruling that gave `Write` covers the tool that corrects | `K1b`, `KR` |
| `K6` consumes what `K7` changes | True | **Accepted**: `K6` after `K7b`, after `M3` | §5 |
| The ruling on the pre-built pieces is in no file | True | **Accepted**: `O0`, before any `K` task | §4, the orchestrator's steps |
| `Q2`, option A understated; *no pass alone* false | **True.** `canary-one/data/agents-attempt-1.tsv`: 118 k / 18.5 min, 158 k / 25.8, 169 k / 25.7, 160 k / 33.0; the second invocation's 98 k / 14.5 | **Accepted**: 400–650 k and 60–130 minutes a stage; §11 corrected | §6 `Q2`; §11 |
| `Q2`, option C records a kind falsely | True of the first revision: a route item beyond N was entered as a lead | **Accepted.** It keeps its kind and is in no queue; `Q3` asks whether it holds the close | §6 `Q2`, `Q3`; `T3a` |
| Still silent: `E5`; the clauses copied | True of both; ruling 14 holds the condition by reference | **Accepted.** `E5` is told to the human; the kit reads the condition from the entry the opening names and copies nothing | §6; `K1a` |
| Broken by the revision: held checks named by commit defeat a re-run | **True**: a start is answered by the job that is there (`STEP` 725), and an item's automatic re-run (`REC` 3217) would meet the same dead job | **Accepted.** A held check is named by its commit and the item's own attempt; a job that stands is taken only while it runs or where it is green, else the next name is asked, as a record's gate is (`HARN` 2762) | `T5b` |
| Broken by the revision: a handed-on item's key | True: it had left `T3a`, and `verify-<key>` must be a slug of at most 100 characters (`REC` 1228) | **Accepted**: the script mints it, with its arm | `T3a` |

**Rejected: none.** One accepted in part (`K2`: the numbering makes counts sufficient; ids need not travel).

