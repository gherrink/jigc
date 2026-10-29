# Route hunter (`wf-route-walk`): build plan

*Read-only draft, 2026-10-27, against `fix/rc24-tier1` @ `495991e3` (tree clean). Nothing is built or ruled here.*

## 0. Facts the plan is cut on

| # | Fact | Where |
|---|---|---|
| A1 | **The site-listing script is already built.** Plan task `S0` built it: a Rust scan on the shared reader, the committed list `sites.tsv` (260 rows, columns `command·file·function·code·ordinal·form·severity·code-written·route·route-text`), and the suite `refusal_site_list::`, which holds the list byte-equal to the scan. The pending row still lists the script as owed, so that part of the row is stale. | `crates/cli/tests/support/refusal_sites.rs`, `crates/cli/tests/route_catalog/sites.tsv`, `crates/cli/tests/refusal_site_list.rs`; `DECISIONS.md` → *2026-10-11 — The product's refusal sites are one scan…* |
| A2 | The `command` column maps a code prefix to a command through `FAMILIES`, and the scan says this is no reachability fact (for example, the `task finalize` family is printed by three doors). Sites whose code is not spelled at the call are `unassigned` (24 blocking). | `refusal_sites.rs` header |
| A3 | The route catalog's fence now covers every family. Every blocking site has exactly one answer: a case, a reason, or a line of `scheduled.tsv`. 144 blocking sites are scheduled (no case yet). Per command, blocking/scheduled: `task finalize` 58/42, `milestone` 31/23, `doc write` 23/20, `config` 19/14, `start` 10/9, `task discard` 4/4, `setup` 23/0, `uninstall` 16/0. | `implementation/pinning.md` §6; `crates/cli/tests/route_catalog/scheduled.tsv` |
| A4 | The reshape plan has a T-task for the hunter, `T8`. Its write-set is `HARN SIM FENCE LOG` (harness, simulation, harness fence, one log entry), it depends on `S0`, `T7` and `O4`, and its done-when names three scenarios plus fence arms (a) and (s). Its prompt is to name `LIST` and the command's family, say a family may list no site, name the catalog so a site and cause that has a case is not hunted again, and ask for the count *(sites listed, provoked, routes followed)*. The fence pair *"the files the hunter's prompt names exist"* is `J1`'s, and `J1` is unbuilt. | `completions/artifacts/M55/stabilization-build/reshape/plan.md` lines 208–212 and 282 |
| A5 | `T1`–`T7` are built: the queue, the budget, `QUEUE_RULINGS`, `queueSaid`, filed verdicts, the stop's view. Not built: `T8`, `O4` (the e2e tester and code reviewer definitions are unchanged since Oct 7), `K1b` (no kit call in `unitPrompt`), `J1`, `T9`–`T12`. | harness source; `ls -la .claude/agents` |
| A6 | An item's kind picks its chain from `CHAINS`, and a kind with no chain halts the stage. A unit is `green` once every step of its chain reported. A result row has only `item·outcome·reason·verdict`, so **it has no field for a count**. | `stabilize.js` 1058–1079, 2106–2113, 4116–4149; `result-set --help` |
| A7 | A ledger row and a triage entry already take `code` and `site` (an id from `sites.tsv`) as plain text, and `wf-recurrence-count` counts by them at every stop. The triage *definition* does not tell triage to carry `site`. | record `--help` `ledger-add`/`triage`; `.claude/agents/finding-triage.md` 51–61 |
| A8 | `kit --role` prints rule text from the file that owns it. Its roles are review, drive, verify, advocate, proposal and triage — there is no hunt role. | `kit --help` |
| A9 | The catalog's words already exist: ten classifications (`not provoked` … `followed, holds, operand implied` …) and three reason words (`not-constructible: …`, `platform: …`, `no-route-printed`). Pinning §6 holds the human's rulings of 2026-10-12 on how a route is followed as printed: span by index, placeholders, cited lines, a command printed without its operand. | `crates/cli/tests/support/route_case.rs` 855–882; `crates/cli/tests/route_catalog.rs` 1590 |
| A10 | The guard suite derives *"types the harness launches"* from `ROLES`. `FENCED` lists nine types, and a new type has to be added to it, named. In the simulation, item reporters are *scripted* by label. `KINDS` gives a play function only to triage, verify, git and the other fixed kinds. | `tooling-tests/dev_stabilize_guard.rs` 63–75 and 1024–1045; `tooling-tests/fixtures/stabilize-runtime.mjs` 701–750 |
| A11 | The plan's §6 told the human that the coverage count would be *"the hunter's own word"*. §7 deferred holding it as a new row, `wf-evidence-held`. **That row is not in `decisions-pending.md`.** The 2026-10-27 ruling *"hunter before the opening, then the stage reviewed"* **is not in `DECISIONS.md` at HEAD.** | `plan.md` 375 and 417; grep |

## 1. Forks — the human's to rule; robust option first

**F1 — Which definition carries the hunter.**
- (A) **A new definition, `route-hunter`**, written by the main session. Its contract is used only by this instrument, so it does not mix with the milestone audit's *"`cargo build`… worked-example flows"*. Costs: one more file under arm (e), the repeated paragraphs copied byte for byte, and a `FENCED` entry.
- (B) **`milestone-e2e-tester`** plus a hunt paragraph bound to a new prompt line. This is the plan's `O4`. It is cheaper, but needs `O4` first, and that definition is shared with milestone builds.
- (C) `finding-verifier`: listed only. Its contract is one key and one filed verdict, which is the wrong shape for a hunt.

**F2 — Where the coverage count lives, and who holds it.**
- (A) **A writer in the record script** (`hunt --from FILE`). The hunter hands over one row per site; the script holds each id to `sites.tsv` and each word to the catalog's vocabulary, holds the partition (every listed blocking site of the family exactly once), **computes** the count, and renders the report from the rows (the triage writer's pattern). The count stops being anyone's word. This goes beyond what §6 told the human.
- (B) A `coverage` field in the return plus a `coverage` cell on the result row (a change to the record script). The harness checks its arithmetic shape only.
- (C) The plan's `T8` as written: the count in the return and the report, its shape checked, *"on record"* meaning the committed report. No change to the record script, and `wf-evidence-held` is added as the row §7 promised.

**F3 — Which sites a hunter is handed.**
- (A) Every blocking site of its family. For a site with a case, it hunts only causes the case does not construct. This needs a **generated answers file** (site → cases with their causes, or reason, or scheduled), held equal by the catalog's fence. It is a product-test change, so it must land before the opening (pending row F15).
- (B) **Only the family's `scheduled.tsv` sites.** A site with a case or a reason is counted *catalogued* and skipped. This is deterministic from two committed files and needs no new file.
- (C) Every site, ignoring the catalog. This repeats what the gate already runs.

**F4 — The 24 unassigned blocking sites.**
- (A) An item of their own, covering registries whole.
- (B) Each hunter is handed them too, and claims one only where the code it saw matches a catalog `MINTED_AT` or claim row.
- (C) Left out, with the count saying how many.

**F5 — How an item names its family.**
- (A) **A `family` cell for kind `route-hunt` in `item-set`**, held to `sites.tsv`'s `command` words and printed by `item`.
- (B) The brief names it; nothing holds it.
- (C) A naming convention on the door (`jigc <family>`). This is fragile because of A2.

**F6 — Large families** (`task finalize` 42 or 58 sites).
- (A) **One item per command, split into parts of at most N sites** (by file). One death then voids less, and a hunt fits an agent.
- (B) One item per command, as the ruling words it.

**F7 — Measuring the hunter before the opening.** The plan's §9 rule needs a rate: an item is written only where the rate times the command's sites is no more than the stage's verifiers.
- (A) **One supervised hunt at `jigc task discard` (4 sites) in a canary clone before the opening.** This is also item 1 of the whole canary's *"still to show"* list.
- (B) Round 1's tuning stop measures it, and the opening prices hunts by estimate.

**F8 — Where the "followed as printed" rules come from.**
- (A) **`kit --role hunt` prints pinning §6 from its file**, so the rules have one home and the hunter's *works/fails* matches the catalog's.
- (B) The definition paraphrases §6, which gives the rules a second home.
- (C) The prompt tells the hunter to read pinning §6.

*Standing, not forks:* a hunter's finding goes through the stage's one triage and a verifier like any other finding. A hunt spends no verdict of the budget. *Holds, operand implied* is counted apart from *holds*, under the rulings of 2026-10-12.

## 2. Tasks, in build order

Every builder task follows the plan's §4 rules: red first, one commit, one full gate in the background, a task carries the assertions it breaks, and the harness is regenerated with `dev/stabilize-harness` with both files committed. **Every task that touches the record script (H1, H2, H3, H5) is cut only after the parallel builder's commits to the record script, the workflow doc, the pending list and `DECISIONS.md` have landed, rebased onto them.** One lane: no two tasks hold the record script, the step tool, the harness, the simulation or the fence at once.

**H0 — Sync and rulings (orchestrator, no files).**
- Read the parallel builder's landed commits.
- Confirm the 2026-10-27 ruling has its entry; if it does not, the first commit below records it.
- Put F1–F8 to the human one at a time, each with its recommendation.
- *Done when:* each fork's answer is recorded in one `DECISIONS.md` entry (the parallel builder's, or H7's first commit).

**H1 — `kit --role hunt` (F8-A).**
- *Files:* `dev/stabilize-record`, `tooling-tests/dev_stabilize_record.rs`, one log entry.
- *Red first:* `the_kit_of_a_hunt_prints_pinning_six_from_its_file_and_the_rig_and_the_report_rules`; `a_kit_for_hunt_whose_section_six_heading_moved_prints_the_word_and_exits_27`; `hunt_is_a_role_the_kit_takes_and_review_still_prints_what_it_printed`.
- *Done when:* green; three mutants are red.
- *Fence:* the kit's own arms; the exit-27 rule is what holds pinning §6 against drift.

**H2 — An item names its family (F5-A; skipped under B or C).**
- *Files:* `dev/stabilize-record` and its suite, one log entry.
- *Red first:* `a_route_hunt_item_names_a_family_the_site_list_has`; `a_family_the_list_lacks_is_refused_and_nothing_is_written`; `another_kind_carries_no_family`; `the_item_read_prints_the_family`; `the_family_stands_once_a_round_owes_the_item` (it joins the four cells that stand).
- *Done when:* green; a run without a hunt item writes the same bytes as before.
- *Fence:* the record suite.

**H3 — The hunt writer (F2-A; under B, the result row's `coverage` cell instead; under C, nothing).**
- *Files:* `dev/stabilize-record`, `tooling-tests/dev_stabilize_record.rs`, `dev/stabilize-step` only if the step tool's vet must learn the file class, one log entry.
- *What:*
  - `hunt --run --round --reporter --attempt --from FILE`, taking a JSON array of per-site rows: `site`; `outcome` (one of the catalog's classifications, or a reason word with its text); `causes` (each with the functions read, as the catalog's `READINGS` has them); `commands`; `route` as printed; and `finding`, the id of the hunter's own finding where the outcome is red.
  - Ids are held to `sites.tsv`; the partition is held to the family (or to `scheduled.tsv` under F3-B); words are held to the catalog's vocabulary.
  - The count is computed, the report rendered and the file filed under the reporter's name. It prints one line `{report, sha256, count}`.
  - It is a filed-file class for `run_file`, `vet` and `set_aside`.
- *Red first:* `a_hunt_leaving_a_listed_site_out_or_naming_one_twice_is_refused_and_nothing_written`; `a_site_the_list_lacks_is_refused`; `an_outcome_outside_the_catalogs_words_is_refused`; `the_count_is_computed_and_no_caller_types_it`; `a_rendered_hunt_is_a_report_like_any_other`; `a_denylisted_term_refuses_the_filing_naming_the_row`; `a_family_with_no_listed_site_is_a_count_of_zero_that_says_so`.
- *Done when:* green; four mutants are red.
- *Fence:* the record suite; the step tool's vet arms if touched.

**O-def — Definition text (main session; subagents may not edit `.claude/agents/`).**
- *Files:*
  - F1-A: a new `.claude/agents/route-hunter.md`, carrying the repeated paragraphs verbatim.
  - F1-B: the hunt paragraph in `milestone-e2e-tester.md`, bound to a new line, and `O4` first.
  - In both cases, `finding-triage.md` gains one sentence: *a finding that names a site id carries it as `site`*.
- *Contract:*
  - **Inputs:** the binary line, the report or hunt line, the kit call (H1), the item read (family and brief), the list, the scheduled list or answers file (F3), and pinning §6 through the kit.
  - **Duties:**
    - Provoke every listed site in every cause it can construct, reading the site's callee for causes and naming the functions read.
    - Run the printed route literally, the operand taken from the printed text by span, with the candidate's binary first on `PATH`.
    - One finding per site and cause that fails, carrying `site`, `code`, and the route line as printed.
    - A site not constructible gets its reason word and the exact reason.
    - States in the planted-state classes of 2026-10-11 are reported and marked.
  - **Return:** the count — listed, catalogued and skipped, provoked, not constructible by reason, causes tried, followed, holds, holds with operand implied, red by classification — through H3's line under F2-A, or as a return field.
  - **Halts when:** the list is missing; the family has no row (under F5-A); the binary's hash does not match.
- *Done when:* each edit has its own full gate. Under F1-A it lands **in the same commit as H4's `ROLES` row**, because the guard suite derives `FENCED` from `ROLES` and fence arm (a) needs the definition present.
- *Fence:* harness fence arms (a), (d), (e); `release_pipeline_fence` (the NEVER paragraph); `dev_stabilize_guard::FENCED`.

**H4 — The instrument in the harness (plan `T8`).**
- *Files:* `.claude/workflow-sources/stabilize.js` and the generated `.claude/workflows/stabilize.js`, `tooling-tests/stabilize_simulation.rs`, `tooling-tests/fixtures/stabilize-runtime.mjs`, `tooling-tests/stabilize_harness_fence.rs`, `tooling-tests/dev_stabilize_guard.rs` (F1-A), one log entry.
- *What:*
  - `ROLES.hunt` (`drives: true`; labels report, binary and the hunt line).
  - `CHAINS['route-hunt'] = [[{as:'hunt', role:'hunt'}]]`.
  - `HUNT_SCHEMA`.
  - `NEEDS.hunt`: the hash missing voids the item; the count or the filing's line missing voids the item.
  - `huntPrompt`, composed only from the three tools' plain calls.
  - The kind is path-independent (`runUnits` is shared), but its scenarios are written on openings with an order.
- *Red first:* a scenario with a `route-hunt` item is run before the chain exists and halts with *no chain for the kind*; that output is kept.
- *Done when:*
  - Simulation: `a_route_hunt_runs_for_a_command_in_the_rounds_scope_and_for_no_other`; `its_coverage_count_is_on_record_with_its_result`; `a_hunter_that_returns_no_count_voids_its_item`; `a_hunters_red_route_reaches_the_one_triage_as_a_numbered_source_and_its_row_is_route_hunt_with_its_site`; `a_hunt_spends_no_verdict_of_the_budget`; under F2-A, `a_hunt_whose_rows_leave_a_site_out_is_refused_and_its_item_is_void`.
  - Self-test: the chain is staffed from the roles; the prompt names the list, the kit call and the family read.
  - Fence: (a) the `ROLES` census; (d) the new label spelled once; (s) `huntPrompt` in the table of composers; (t) `HUNT_SCHEMA`'s optional fields are `NEEDS` rows or `NOT_NEEDED`, and each void cell is a simulation arm; a new arm `the_files_the_hunters_prompt_names_exist` (absorbing `J1`'s half), red against a path the harness constant names and the tree lacks.
  - Simulation runtime: a `KINDS` entry `/:hunt$/` whose play function writes the rows and makes H3's call.
- *Fence:* the simulation, the harness fence and the guard suite on every gate. Drift between the list and the scan is held by `refusal_site_list::`; drift between the list and the catalog by `route_catalog::every_blocking_refusal_site_has_a_case_a_reason_or_a_scheduled_row`.

**H5 — The stop shows the count (F2-A or B; skipped under C).**
- *Files:* `dev/stabilize-record` (view), `dev/stabilize-step` (the digest's counts), both suites, one log entry.
- *Red first:* `the_view_shows_each_hunts_count_beside_its_familys_listed_sites`; `a_hunt_item_with_no_filed_hunt_is_named_as_such`; `the_digest_carries_the_hunts_numbers_and_no_site_id`.
- *Done when:* green; a view with no hunt item is unchanged byte for byte.
- *Fence:* the record and step suites (*the view is a function of the record*).

**H6 — A measured hunt before the opening (F7-A).**
- *Files:* `dev/stabilize-canary`, `tooling-tests/dev_stabilize_canary.rs`, one log entry.
- *Red first:* `a_setup_with_a_hunt_writes_one_route_hunt_item_at_task_discard_and_its_prompt`; `check_says_what_the_hunt_filed`.
- *Then (human-run, not a builder's):* one supervised invocation in the clone. Record tokens and minutes per provoked site, and the count beside the list's 4. Commit the result as the rate the opening prices with.
- *Fence:* the canary suite.

**H7 — The workflow doc, the pending rows, the log (a document task).**
- *Files:* `implementation/stabilization-workflow.md`, `implementation/decisions-pending.md`, `DECISIONS.md`.
- *What:*
  - Doc, *Opening a run*: one `route-hunt` item per command (or part, F6), its family, the clause *every refusal's route works as printed*, and the pricing rule with H6's rate.
  - Doc, *A round's `test`* step 4: hunters among the instruments.
  - Doc, *Orchestration*: the new role or definition, and whether it is in `FENCED`.
  - Doc, *The simulation of a stage*.
  - Doc, *Open questions and declared bounds*: the count, held or the hunter's own word per F2; and what the hunt cannot see (A2, F4).
  - Pending: `wf-route-walk` discharged with its commits; `wf-evidence-held` added if F2-C; `wf-recurrence-count` annotated (site ids now come from hunts); `wf-whole-canary-after-parts` annotated with H6.
- *Done when:* `doc_link_fence` is green; the harness fence's doc arm for the halt list is unchanged.
- *Fence:* `doc_link_fence`; harness fence arm (t), which holds the doc's halt rows.

**H8 — Then the ruling's second half (orchestrator):** the review of the `test` stage as built, for a real run. It is outside this plan.

## 3. How findings flow, as built after H4

1. The hunter's red routes are its unit's findings.
2. They go to the one triage as a numbered source `<item>-hunt`, and triage gives each entry `kind route`, its route line, `code` and `site` (O-def).
3. The first record writes the ledger and `classes.md` with class `route hunt`.
4. The queue ranks them by the opening's order, and verifiers drive them within the budget and file their verdicts.
5. The second record takes the verdicts, the stop's view is rendered, and the recurrence count is printed by site.
6. Route items nobody reached hold the close and end in `rule` (the ruling of 2026-10-11).
7. The hunt's own result is `green` when it ran to its end. What it found blocks the close through the ledger, never through the result.

## 4. Not verified

- What the parallel builder will change in the record script, the doc or the pending list. The tree is clean and no second worktree holds that work.
- Whether 24 to 58 sites fit in one agent's run under the runtime's limits; nothing has timed it.
- Whether harness fence arm (t) walks fields nested inside `FINDING` (for example a new `site` on a finding).
- Whether a new agent type needs an entry in `.claude/settings.json`; the plan's line 248 says no new allow rule.
- Whether the write fence lets a hunter's `dev/jigc-rig` roots under `$TMPDIR` through. The staged canary's verifiers drove rigs, but I did not find a record of that specific case.
- How sites in `crates/engine/src` (the `unassigned` dynamic sites) can be reached by any one command.
- Whether `sites.tsv` in the tree equals the candidate's during a re-run. Record commits touch only the run's directory, so it should, but I did not check.

### Critical Files for Implementation
- `.claude/workflow-sources/stabilize.js`
- `dev/stabilize-record`
- `tooling-tests/stabilize_harness_fence.rs`
- `tooling-tests/fixtures/stabilize-runtime.mjs` (with `tooling-tests/stabilize_simulation.rs`)
- `crates/cli/tests/route_catalog/sites.tsv` (with `crates/cli/tests/route_catalog/scheduled.tsv`, `crates/cli/tests/support/refusal_sites.rs`)