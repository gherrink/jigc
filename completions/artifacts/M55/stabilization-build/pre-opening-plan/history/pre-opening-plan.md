# Build plan: everything owed before the first stabilization run opens

Read-only. Based on `fix/rc24-tier1` @ `0bd8a5b5`, which is clean. Facts I checked are cited by file and line. Anything I could not check is in the last section.

## 0. Facts the plan is built on

- **The 145 scheduled rows** (`crates/cli/tests/route_catalog/scheduled.tsv`): 144 are blocking `unchanged` sites, plus one `not-counted` dynamic row, `crates/cli/src/author.rs::reject::dynamic#1`. By family (`sites.tsv` command column):

  | Family | Scheduled sites |
  |---|---|
  | `task finalize` | 42 |
  | `milestone` | 23 |
  | `doc write` | 20 |
  | `unassigned` | 19 + 1 dynamic |
  | `config` | 14 |
  | `start` | 9 |
  | `doc read` | 4 |
  | `task discard` | 4 |
  | `ingest` | 3 |
  | `migrate` | 2 |
  | `task bind` | 2 |
  | `validate` | 2 |

  `setup` and `uninstall` have no scheduled rows.
- **What the fence accepts today.** `route_catalog::every_blocking_refusal_site_has_a_case_a_reason_or_a_scheduled_row` (`route_catalog.rs:2850`) passes a site with a case, a `REASONS` entry of any `Why`, or a scheduled row. There are only 9 site-level `Reason {` entries in all the tables: setup 4, task_finalize 2, unassigned 2, config 1. A cause-level `NotConstructible` appears about 154 times, inside `READINGS.without_case` and `READINGS.not_followed`.
- **"No git on PATH" is an example of the frame gap.** `route_catalog/setup.rs:4100-4113` gives the reason: *the frame has no act that changes a variable between the refusing command and the route's*.
- **How big the 2026-10-12 family tasks were** (their `DECISIONS.md` tables, "minutes" = first probe to green, gate not counted):

  | Sites | Cases | Minutes |
  |---|---|---|
  | 7 | 42 | 22 |
  | 7 | 20 | 22 |
  | 2 | 45 | 23 |
  | 8 | 84 | 50 |
  | 3 | 18 | 15 |
  | 7 | 57 | 46 |

  That is about 3–7 minutes a site, and up to 22 for an engine funnel (`store.home-not-regular-file`).
- **`dev/runner-faithful` asserts the opposite of the runner.** Its posture assertion (lines 222-229) requires *no* `/etc/gitconfig`. The CI fixer's variant changes two lines: it hard-codes `repo_root` to a scratch worktree (not to be kept) and adds `docker cp …runner-etc-gitconfig $cid:/etc/gitconfig` (`[safe] directory = *`). The 2026-10-27 entry also notes the git version differs: the runner has 2.56 from the git-core PPA, the image has stock 2.43.
- **`K7` (the trial image's base layer) is not built.** Nothing under `completions/trial-harness/` has changed since M54. `K7b` "calls what `K7` changed" (reshape plan line 334), so `K7` and `M3` are owed even though they were not on your list.
- **The fence that will bite `T8`.** `K1b`'s fence holds `KIT_DRIVES` equal to the harness's `drives`, and holds every kit role to a role the harness launches (DECISIONS.md:15-17). So a kit role `hunt` cannot land before `ROLES.hunt`.
- **The gitleaks hit.** The scratchpad's `gl.json` has one hit over 310 commits: `generic-api-key`, `DECISIONS.md:99` at commit `85661425`. The same JSON shape still stands at HEAD on `DECISIONS.md:7748` and `:7754`. `.gitleaks.toml` exists, with one allowlist entry keyed to a file and a line shape.
- **The version disagreement.** `DECISIONS.md:9535` (ruling 1 of 2026-10-05) says *"`1.0.0` in this run"*. `CLAUDE.md:7` says *"The 1.0.0 call is the human's, and it has not been taken."*
- **The two prompts missing the *Tools* paragraph.** `proposalPrompt` (`stabilize.js:2908`) and `crossModelPrompt` (`:2930`) are the two roles run as `general-purpose` with no definition.
- **`O6` may already be done.** `CLAUDE.md:27` already points at the route catalog through `tooling.md`.

## 1. Lanes, shared files and gates

| Lane | Where | Files it owns |
|---|---|---|
| **L0** | main session, on `fix/rc24-tier1` | `DECISIONS.md` entries recording your rulings, `.claude/agents/`, `CLAUDE.md` |
| **R** | worktree, `fix/rc24-runner-faithful` | `dev/runner-faithful`, the new `tooling-tests/dev_runner_faithful.rs`, `dev-workflow.md` → Gate, its row in `tooling.md` |
| **P** | worktree, `fix/rc24-route-catalog-rest` | `crates/cli/tests/route_catalog*`, `support/route_case.rs`, `pinning.md` §6, `route-catalog-findings.md` and `route-catalog-pilot.md`, the projection's row in `tooling.md` |
| **K** | worktree, `fix/rc24-trial-image-layer` | `completions/trial-harness/` |
| **T** | the loop branch's own checkout | record script, step tool, harness source plus its generated file, simulation and runtime fixture, harness fence, canary, `stabilization-workflow.md`, `decisions-pending.md` (after `D0`) |

- **Writes inside lane T are one at a time.** Two tasks never hold the record script, the step tool, the harness, the simulation or a fence together.
- **Declared overlaps.** `DECISIONS.md` is resolved by `dev/merge-logs` at each merge. `tooling.md` and `tooling-tests/groups/g_tooling.rs` are touched on different rows or lines in R, P and T, and resolved by hand at the merge.
- **Gates.** Only one gate, or one `dev/runner-faithful` run, at a time on the machine. Worktrees use `--private-target`. The orchestrator queues them.
- **Merges into the loop branch** (git agent, `--no-ff`):
  - **M-R**: R → loop. Must come before P's first family task, because each family task's Linux line must come from a faithful image.
  - **M-P0**: P → loop, after `P0`. Must come before `T1`.
  - **M3**: K → loop. Must come before `K7b`.
  - **M-P**: P → loop, after `P-Z`. Must come before `T10` and `J1`.

## 2. Tasks, in build order

### Step 0 (L0)

**D0 — Record what you ruled on 2026-10-27 that is not yet recorded.** Main session.
- **Writes:** one `DECISIONS.md` entry for rulings 2, 3, 4, 6 and ruling 5's leftover; `implementation/decisions-pending.md`.
- **What it records:**
  - The new order: catalog first, then the hunter on top of it, the regression base with the remaining `test`-stage tasks, then `J2` = `wf-test-review-real-run`, then (a)–(c), then the opening.
  - The H1 correction, as a bracketed annotation on the 2026-10-27 *not-comparable* entry. *"Inside by construction"* holds for round 1 only. A break the run's own fixes cause where the previous release is not green is now caught by the regression base.
- **Pending rows:**
  - `wf-route-catalog-rest`: trigger moved to *before the opening*.
  - New row `wf-regression-base`, trigger *before `J2`*.
  - `wf-route-walk`: annotated with ruling 4.
  - `wf-test-review-real-run`: annotated as `J2`.
  - New row `wf-runner-faithful-posture`.
- **Red first:** none; this is a document task.
- **Done when:** `doc_link_fence` is green and a full gate is green.
- **Fence:** `doc_link_fence`.

After `D0`, fork R, P and K from `D0`'s commit. **R, P, K and T then run in parallel; only their gates queue.**

### Lane R

**R1 — `dev/runner-faithful` takes the runner's git posture, and gets a test of its own.** Builder.
- **Writes:** `dev/runner-faithful`, the new `tooling-tests/dev_runner_faithful.rs`, its line in `tooling-tests/groups/g_tooling.rs`, `implementation/dev-workflow.md` → Gate, the `tooling.md` row's *Held by* cell, one `DECISIONS.md` entry.
- **What:**
  - The Dockerfile writes `/etc/gitconfig` = `[safe]\n\tdirectory = *`, as runner-images' `install-git.sh` does.
  - The posture assertion then requires that file and exactly `file:/etc/gitconfig safe.directory=*` from `git config --list --show-origin`, and still no global or XDG file and no `GIT_CONFIG_*`.
  - The header states what the image is faithful to, line by line, with the runner-images source named.
  - Port nothing from the scratch variant except the `/etc/gitconfig` idea. Its `repo_root` is hard-coded.
- **Red first (static, no docker needed):**
  - `the_image_carries_the_runners_system_config_and_no_other`. It parses the heredoc and is red at HEAD, where the image asserts no `/etc/gitconfig`.
  - `the_posture_refuses_an_image_without_the_runners_safe_directory`.
  - `a_usage_error_exits_2_before_docker_is_asked`.
  - `help_prints_the_header`.
- **Ignored arm (docker):** `the_ownership_cell_is_red_at_936c6d54s_parent_and_green_at_it`. This makes the CI fixer's evidence the tool's own.
- **Done when:**
  - The gate is green.
  - The ignored arm has run once, with its lines in the commit.
  - `g_migration` (route catalog and byte table) has run once in the new image: green, or each difference listed.
- **Fence:** the new suite, plus `tooling_index`.
- **Your fork:** whether git comes from the PPA, and whether a runner-side arm checks for drift (fork 7).

### Lane K

**K7 — The trial image's base layer** (reshape plan §5, as written). Builder.
- **Writes:** `completions/trial-harness/`, one `DECISIONS.md` entry.
- **Done when:** `a_second_candidate_with_the_same_lockfile_builds_on_the_kept_layer` and `a_changed_lockfile_or_toolchain_file_builds_a_new_layer`, and the image's seven checks plus the pair check still hold.
- Merged as **M3** before `K7b`.

### Lane P (cases are data; it writes no product source; a red case is listed, never fixed here)

**P0 — The catalog's projection: one generated, committed file, held equal by the suite like `sites.tsv`.** Builder.
- **Writes:** `crates/cli/tests/route_catalog.rs`, a new `route_catalog/projection.tsv` (the name is the builder's), its row in `tooling.md`, one `DECISIONS.md` entry.
- **What the file holds:**
  - A header with the vocabulary: the ten classifications and the three `Why` words, read from `Classification` and `Why`'s `Display`. This settles the review's B3.
  - Per family: its doors (from `FAMILIES` in `refusal_sites.rs`, which settles S8), and every site including claimed, `MINTED_AT` and `IN_PART` dynamic sites (which settles B2).
  - Per site, its answer: each case with its cause; or the reason; or `scheduled`; plus `without_case` and `not_followed` with their `Why`.
  - The residue: dynamic sites no family claims.
- **Red first:**
  - `the_committed_projection_is_the_catalogs_answers`, red against an empty file with the sites named. `UPDATE_GOLDENS=1` regenerates it, and CI refuses that, as with the goldens.
  - `a_claimed_dynamic_site_is_in_its_familys_rows`.
  - `the_vocabulary_header_is_the_frames`.
  - Mutant: a case dropped from a table makes the file differ.
- **Done when:** the gate is green; merged as **M-P0**.
- **Fence:** the new arm.

**P1 — Tighten the fence: a site without a case needs a reason you ruled.** Builder, with one halt to you.
- **Writes:** `route_catalog.rs`, a new `route_catalog/ruled_reasons.tsv` (site · `Why` · the `DECISIONS.md` heading of the ruling), `pinning.md` §6, one `DECISIONS.md` entry.
- **What:** a `REASONS` entry passes only if the ruled list has it and its heading exists in `DECISIONS.md`. The `not-counted` class is refused for any site except the one row standing today, which C17 gives a case.
- **Red first:**
  - `a_reason_nobody_ruled_does_not_answer_a_site`. Red over the 9 existing reasons until they are ruled.
  - `a_not_counted_row_is_refused_for_a_new_site`.
  - `a_ruled_reasons_heading_that_is_gone_is_named`.
- **The halt:** the 9 existing reasons go to you as one batch. The main session records your answer, and the commit cites it.
- **Done when:** the gate is green.
- **Fence:** the catalog's own suite.
- **Your fork:** fork 1 (mechanism) and fork 2 (tighten now or last).

**C1–C18 — The family tasks.** One builder each, serial in lane P.
- **Writes:** that family's file under `route_catalog/` (a new one where none exists, plus its `tables!` line), `scheduled.tsv` (rows removed), `EXPECTED_RED`, `CATALOGUED`, `CLAIMS` and `MINTED_AT` as needed, `projection.tsv` regenerated, `route-catalog-findings.md` (minted keys), one `DECISIONS.md` entry with the 2026-10-12 table (sites · functions read · causes · followed · reasons · cases · red · minutes).
- **Red first:** each case is written, run and seen in its classification before it is listed. Any frame vocabulary a case needs is added red first, against a runner that declares it and does nothing.
- **Done when:**
  - The family's rows are gone from `scheduled.tsv`.
  - The strict expected-red list holds.
  - The projection is regenerated.
  - One `dev/runner-faithful` run of `route_catalog::` is green, with its line in the commit.
  - The named-binary arm has run on rc.24, with its table in the record.
  - The suite's seconds are in the commit.
  - A new site-level reason is a batched halt to you at the end of the task.
  - Any site over about an hour stops the task.
- **Fence:** `every_blocking_refusal_site_has…` (as tightened by P1), `the_expected_red_list_is_strict_three_ways`, the projection arm.

| # | Sites | What |
|---|---|---|
| C1 | 10 | `crates/cli/src/task.rs`: `task discard` 4, `task bind` 2, `doc read` 4 |
| C2 | 7 | `migrate` 2, `validate` 2 (`render.rs`, `engine/probe.rs`), `ingest` 3 |
| C3 | 9 | `start`: `start.rs` 4, `milestone.rs` 1, `engine/compose.rs` 4 |
| C4 | 7 | `config`: the first 7 of `config.rs` |
| C5 | 7 | `config`: the other 3 of `config.rs`, `relocate.rs` 2, `start.rs` 2 |
| C6 | 11 | `doc write`: `doc.rs` |
| C7 | 9 | `doc write`: `task.rs` 2, `engine/state.rs` 5, `engine/write.rs` 2 |
| C8 | 10 | `milestone`: `cli/milestone.rs` 6, 4 of `engine/milestone.rs` |
| C9 | 7 | `milestone`: `engine/milestone.rs` |
| C10 | 6 | `milestone`: the rest of `engine/milestone.rs` |
| C11 | 7 | `task finalize`: `engine/parse.rs` |
| C12 | 7 | `task finalize`: the rest of `engine/parse.rs` |
| C13 | 10 | `task finalize`: `engine/finalize.rs` 8, `index.rs` 1, `target_surface.rs` 1 |
| C14 | 10 | `task finalize`: cli `doc` 2, `orphan` 2, `render` 3, `start` 1, `task` 2 |
| C15 | 8 | `task finalize`: `file_state` 2, `state` 2, `validate` 3, `milestone` 1 |
| C16 | 9 | `unassigned`, cli dynamic sites: `author`, `doc_code_probe`, `ingest`, `migrate_corpus`, `relocate`, `rename`, `render`, `repo`, `rollback` |
| C17 | 6 | `unassigned`, engine dynamic sites: `cascade` 2, `compose` 1, `finding` 3 |
| C18 | 5 | `unassigned`, engine dynamic sites: `milestone`, `override_default`, `store`, `validate`, `write` |

- **Total: 145 sites.** At 3–7 minutes a site this is about 7–17 builder-hours, plus 18 gates and 18 Linux runs.
- **C16–C18 first count the codes each dynamic site mints and report that number before building** (fork 3).
- Families stay whole in their own file; only the commit groups them.

**P-NC — The not-constructible sweep.** Builder; may split into one commit per frame piece.
- **Writes:** `support/route_case.rs`, `route_catalog.rs`, the affected family files, `projection.tsv`, `pinning.md` §6, one `DECISIONS.md` entry each.
- **What:**
  - Read every `NotConstructible` text in the projection.
  - Classify each as *plantable with a frame piece* or *not*, and commit that classification as a table in the entry.
  - For each plantable one, add the frame piece red first together with the case that needs it. The first is a per-command environment: no `git` on `PATH` for the refusing command, and an act that puts `PATH` back, for `setup.rs:4107` and `:3990`.
- **Red first:** per piece, e.g. `the_refusing_command_runs_without_git_on_path_and_the_route_puts_it_back`.
- **Done when:** each plantable cause has its case or the table says why not; the projection is regenerated; the gate and the Linux line are in the commit.
- **Fence:** the frame's arms, plus the projection.

**P-Z — Close the catalog.** Builder.
- **Writes:** `route_catalog.rs` (the `Scheduled` type, `SCHEDULED`, `THE_COUNT` and its arm `a_scheduled_site_is_one_the_count_holds_unchanged` removed), `scheduled.tsv` deleted, `pinning.md` §6, `route-catalog-pilot.md` (§14 numbers: sites, causes, cases, red by classification, reasons by word, the suite's seconds, the rc.24 table), one `DECISIONS.md` entry.
- **Red first:** `the_fence_answers_a_site_only_with_a_case_or_a_ruled_reason`.
- **Done when:** the gate is green and the suite's seconds in the gate are recorded. If `g_migration` goes past CI's bound, its split is in `ci_matrix_fence` in this commit.
- Merged as **M-P**.

### Lane T (one at a time, in this order)

**T0 — The harness's two roles without a definition get the *Tools* paragraph** (ruling 5's leftover). Builder.
- **Writes:** harness source plus the generated file, `tooling-tests/tooling_index.rs`, one `DECISIONS.md` entry.
- **Red first:** `the_prompts_of_roles_without_a_definition_carry_the_tools_paragraph`. It is held to the definitions' paragraph with repository-relative links.
- **Done when:** the gate is green and the harness's self-test is carried.

**RB1 — The regression base, the record script's half.** Additive: a run without the new fact answers exactly as before. Builder.
- **Writes:** `dev/stabilize-record`, `tooling-tests/dev_stabilize_record.rs`, one `DECISIONS.md` entry.
- **What:**
  - A filed verdict may carry `base`: `true`, `false`, `not-comparable` or `none`, together with the base binary's sha256.
  - The base's commit is derived by the script from the round record (fork 4), never passed in.
  - Grade `regression` = previous-release fact `true`, **or** base `true`.
  - `route_of` sends such a finding to `fix`/`regression`.
  - The base gets a cell of its own in the triage record, the view and the ledger.
  - A `base` without its hash is *unknown*.
- **Red first:**
  - `a_finding_green_on_the_base_and_red_on_the_candidate_is_a_regression_of_the_run_and_a_fixers`.
  - `a_not_comparable_finding_red_on_the_base_is_routed_by_its_door`.
  - `a_base_without_its_hash_is_unknown`.
  - `round_one_has_no_base_and_says_none`.
  - `a_run_without_a_base_writes_the_bytes_it_wrote`.
- **Done when:** the gate is green and three mutants are red.

*Wait for **M-P0**.*

**T1 — An item may name a family.** A kind-agnostic optional cell, held to the projection's family words; this is F5 settled. Builder.
- **Writes:** the record script, its suite, one `DECISIONS.md` entry.
- **Red first:** `an_item_names_a_family_the_projection_has`, `a_family_the_projection_lacks_is_refused_and_nothing_is_written`, `the_item_read_prints_the_family`, `the_family_stands_once_a_round_owes_the_item`.
- **Done when:** the gate is green.

**T2 — The hunt writer: `hunt --from FILE`.** Builder.
- **Writes:** the record script, `dev/stabilize-step` (the vet learns the filed class), both suites, one `DECISIONS.md` entry.
- **What:**
  - One row per site of the family.
  - Ids and the partition are held to `projection.tsv` **read at the round's candidate commit**.
  - Words are held to the projection's vocabulary header.
  - The coverage count is computed by the script, never typed by a caller.
  - The report is rendered and filed under the reporter's name.
- **Red first:** the review's seven arms (route-hunter plan H3), re-pointed from `sites.tsv` to the projection, plus `a_claimed_dynamic_site_is_a_row_a_hunt_owes`.
- **Done when:** the gate is green and four mutants are red.

**T3 — The stop's view and the digest show each hunt's count beside its family's listed sites.** Builder; only if you rule fork 5 option A.
- **Writes:** record script and step tool, their suites.
- **Red first:** the three H5 arms.

**O-def — The definitions.** Main session; each edit behind its own gate.
- **What:**
  - `O4` as planned: *What a finding carries* gains a kind's evidence, in both copies.
  - The tester gets the kit sentence and the hunt contract, bound to a new `HUNT:` line (E9 kept): provoke every site and cause; try the projection's not-constructible causes by hand; set a Bash timeout on hanging causes (review N5); one row per site through `T2`'s call; never a site outside its family.
  - `finding-triage.md`: a finding that names a site carries `site`.
  - `stabilize-opening.md`: one `route-hunt` item per family from the projection, with its doors.
  - `build-fixer.md`: a cause a hunt found that can be planted gets its catalog case in the fix, red first, and a new blocking site comes with its case.
- **Must come before T8.**

**T4 — The harness fence's census (arm (t)) descends into a list's `items`** (review S5). Builder.
- **Writes:** `tooling-tests/stabilize_harness_fence.rs`.
- **Red first:** a fixture with an optional field inside the items of `FINDING`.

**T8 — The route hunter as an instrument.** Builder.
- **Writes:** harness source plus generated file, the simulation and `stabilize-runtime.mjs`, the harness fence, the record script (the kit's `hunt` role in the **first commit, together with `ROLES.hunt`**, because of the `K1b` fence), one `DECISIONS.md` entry.
- **What:**
  - `ROLES.hunt` = `milestone-e2e-tester`, `drives: true`.
  - `CHAINS['route-hunt']`, `HUNT_SCHEMA`, `NEEDS.hunt`.
  - `huntPrompt`: kit line, binary, `HUNT:` line, the projection at the candidate's commit, the family, `T2`'s call.
  - Items over 12 sites are split into parts by file (F6).
- **Red first:** the plan's halt *no chain for the kind*, kept as evidence; the five simulation arms of route-hunter plan H4; `kit --role hunt` prints pinning §6 from its file, and exits 27 when the heading has moved.
- **Done when:** the gate is green and the fence's arms (a), (d), (s), (t) and the new `the_files_the_hunters_prompt_names_exist` pass.

*Wait for **M3**.*

**K7b**, then **K6**, as the reshape plan specifies them.

**T9** — the rest of it. Includes `its_doors_are_the_review_rows_the_hunts_and_the_arms`.

**T11**, then **T12**, as specified. `T12` also re-points `runFix`'s calls away from `VERIFY_PASSES` (review N1).

**RB3 — The verifier's definition.** Main session.
- Step 4: drive the base binary whenever the previous release's fact is not `true`.
- Review N4's clause: *the door, or a step of the block, the previous release does not have*.
- Bound on a new `BASE:` line; inert until RB2.

**RB2 — The regression base, the harness's half.** Builder.
- **Writes:** step tool, harness plus generated file, simulation, harness fence, one `DECISIONS.md` entry.
- **What:** in rounds 2 and later, a third held build: the base commit. The verifier's prompt gets the `BASE:` line. `VERDICT_SCHEMA` gains `base`. `NEEDS` rows.
- **Red first:**
  - `in_round_two_a_new_door_broken_by_a_fix_is_a_regression_and_goes_to_a_fixer_unasked`.
  - `a_re_broken_fix_red_on_the_previous_release_and_green_on_the_base_is_a_regression`.
  - `round_one_builds_no_base`.
  - One `NEEDS` void arm per cell.
- **Done when:** the gate is green. The schema form follows fork 6.

**O5** — main session, as planned.

*Wait for **M-P**.*

**T10 — The documents.** Builder; a document task.
- **What:**
  - Everything planned for `T10`.
  - Review N1 (`stabilization-workflow.md:706`) and N2 (`:140`, `:275`).
  - The hunter's sections: an item hunts a *family*; the hunt residue, and that unknown sites are the review rows' and drives' job, are declared bounds; the pricing rule.
  - The regression base.
  - Pronoun lines N8, where they are comments.
- **Pending rows:**
  - Add the five from review S6 (`wf-evidence-held` noted as not needed, by ruling 4).
  - Discharge `wf-route-walk`, `wf-route-catalog-rest` and `wf-regression-base`.
- **Done when:** `doc_link_fence` is green.

**J1** — as planned: `every_expected_red_key_is_a_row_somebody_holds_open`, plus the hunter-paths arm if `T8` did not absorb it.

### The join

**J2 — The review of the whole diff, which is the review of the `test` stage for a real run.**
- The main session launches `milestone-code-reviewer` over the diff from the last re-review of the `test` half to HEAD.
- Fixes go back through the lane that owns each file, in rounds of at most three.
- **O6:** main session. Either confirm `CLAUDE.md:27` covers it and record why, or add the two sentences.
- **Done when:** every finding is fixed or ruled, and `wf-test-review-real-run` is discharged with the record.

## 3. Your decisions, after J2 and before the opening

- **(a) `wf-whole-canary-after-parts`.** The measured hunt (old F7) belongs here: one supervised hunt at a small family in a canary clone, using `T9`'s setup, to price the hunts. A family's rate is a floor, not an estimate (review S8). Hunts' red routes also draw verdicts from the budget (N6).
- **(b) The gitleaks hit.**
  - It only has to be cleared before the first PR to `main`.
  - If you choose a history rewrite, it must happen before the opening: the run's records name commit shas, and a rewrite after the opening invalidates them.
  - The line shape (a regression-set JSON line of shas) will recur in the run's own records. A `.gitleaks.toml` entry keyed to that file and line shape covers that. A `.gitleaksignore` fingerprint covers only the one hit.
- **(c) The version.** The opening writes the target version, so this must be settled before it. Whichever wording stands, the `CLAUDE.md` edit is the main session's.
- **Then the opening.**

## 4. Genuine forks (the robust option first)

1. **How a reason gets ruled.**
   - (A) A ruled-reasons list, each row naming a `DECISIONS.md` heading, held by the fence.
   - (B) Reasons frozen at a count, with new ones held by review only.
2. **When the fence is tightened.**
   - (A) Before the family tasks, with each task's new reasons batched to you as one halt.
   - (B) Last, in `P-Z`.
3. **What answers a dynamic site that mints several codes.**
   - (A) Every code it mints is claimed and given a case.
   - (B) One case per site.
   - The size is unknown until C16–C18 count it.
4. **Which binary is "the candidate its round started from".**
   - (A) In round *n* ≥ 2's `test` stage, round *n*−1's candidate, which is what the landed fixes were made on. In the fix stage's audit, the round's own candidate. Round 1 has none.
   - (B) The literal reading, the round's own candidate. In `test` that binary is the one under test, so nothing can be green there.
5. **Where the hunt count is shown.**
   - (A) Also in the stop's view and the digest (`T3`).
   - (B) Only in the filed report.
6. **The mixed-type schema enum (review N3).**
   - (A) A three-word string enum in the harness's schemas, mapped in `regressionOf`, and the same form for `base`.
   - (B) Keep it, and observe whether the runtime accepts it with a `{probe}` run.
7. **How faithful `dev/runner-faithful` is.**
   - (A) git from the git-core PPA as on the runner, plus an arm that runs only under `GITHUB_ACTIONS` and holds the runner's `git config --system` to the tool's declared posture.
   - (B) Stock git, static checks only.

Already settled, not forks: F1 (E9, the tester), F5, F6, F8 (kit option A).

## 5. What I could not verify

- Whether the 2026-10-12 condition *"held back until the parts have run"* is met for all four parts on record. Part 4's last entry is 2026-10-17; I found no entry reading part 4.
- How many codes each dynamic site mints (this sizes C16–C18).
- Whether `g_migration` stays inside CI's per-job bound after roughly 145 sites' cases are added.
- Whether the step tool's push vet runs gitleaks over the run's own record lines, and would refuse them.
- Whether line 7754 of `DECISIONS.md` would also be flagged by a range that includes its commit.
- Whether the Workflow runtime accepts a type-less, mixed enum.
- What else the runner image configures beyond `safe.directory`, and its exact git version at the time R1 is built.
- That the `K1b` fence turns red on a kit role ahead of `ROLES.hunt`. I read this from the 2026-10-27 entry and did not run it.
- Whether `O6` is already discharged by `CLAUDE.md:27`.

### Critical Files for Implementation
- crates/cli/tests/route_catalog.rs (with `route_catalog/scheduled.tsv`, `support/route_case.rs`)
- dev/stabilize-record
- .claude/workflow-sources/stabilize.js
- dev/runner-faithful
- completions/artifacts/M55/stabilization-build/reshape/plan.md