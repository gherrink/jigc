# Coverage — every surface M48 changed, in exactly one column

Required by [protocol.md](protocol.md) §6. Derived 2026-08-18 from the repo, not from a summary:
the diff range is **`8979f16..9cb9b78`** (`8979f16` = the last pre-M48 commit, the sha §5 arm 2 pins
as the rc.10 baseline; `9cb9b78` = `chore(release): 1.0.0-rc.11`) — 698 files, 12 increments plus
four audit fixes.

---

## Method, and the two errors it is written against

M47's first derivation **ranked files**, which called `rename` untouched while `rename.rs` really was
unchanged and the *verb* had gained an error-code-registry member. So the unit here is a **surface** —
a verb, a flag, a finding code, a route, a fence, a contract key, a composed pack statement — and a
surface is placed by what it *is*, never by which file carries it. `rename` appears three times below
under three different increments; `rename.rs` is cited nowhere.

M47's first derivation also used a two-way split with no way to say *"fenced by a test, not reached
by the trial"*, which left ~27% of changed lines unexamined. The three columns fix that, under one
**precedence rule, stated so it cannot be applied after the fact**:

> **trial-reached > test-fenced > neither.** A surface the trial drove is listed as trial-reached
> even when a suite also fences it (most are fenced too). *Test-fenced* means the trial did **not**
> reach it and a standing test does. *Neither* means nothing reached it, and it must be explained.

**Every cited suite was verified by reading what its test asserts**, per the caution in §6's brief.
`verb_suite_coverage` was **not** used as evidence: its green means *named by a suite*, which this
project has misread five times. Where a citation's strength is weaker than its name suggests, the row
says so.

**Evidence tiers inside the trial-reached column**, marked per row so the strength is visible:

| tag | meaning |
|---|---|
| **[B]** | a blind session drove it — argv or `finding_codes` in that corpus's `.jigc/logs/invocations.jsonl`, or a committed artefact the session produced |
| **[W]** | the operator-scripted walk drove it — verbatim output in [v1-walk.md](v1-walk.md) |
| **[R]** | the operator's **rehearsal** drove it on the live rc.11 binary — verbatim output in [cue-cards.md](cue-cards.md) / [pre-trial-findings.md](pre-trial-findings.md), but on the host, and the session it was built for never fired |

The **[R]** tier is separated rather than blended because it is the weakest of the three, and §6's
answer changes if the human reads "the trial" as *sessions + walk only*. That consequence is
quantified at the bottom; the short version is that all five [R] rows are also fenced, so none of
them moves to *neither*.

---

## Column 1 · Trial-reached (35 surfaces)

### Increment 1 — the destroying-door guard

| # | Surface | Evidence |
|---|---|---|
| T1 | `milestone provision` refuses a non-empty, non-registered leftover — `milestone.leftover-holds-work`, naming the path, enumerating its 11 items, `--force` as the consent | **[W]** arm 1, verbatim; all four §5 criteria met; `--force` then run verbatim and destroyed the work as consented (exit 0) |
| T2 | `milestone discard` refuses the same state — `milestone.dirty-worktree` | **[W]** arm 1 |
| T3 | `jigc uninstall` refuses the same state — `uninstall.dirty-worktree` — and the new `--force` flag clears it | **[W]** arm 1 (refusal + consent); arm 3 drove the clean exit-0 path |
| T4 | The classifier's **unverifiable** verdict (`git rev-parse` non-zero) — *"git cannot read a repository there, so nothing can say those bytes are disposable"* | **[W]** arm 1, that clause is in the quoted refusal |
| T5 | An **empty** leftover at a real sub-task path is still reused, not refused — the idempotent case survives the widened guard | **[W]** arm 1, exit 0 |
| T6 | The uninstall route's **no-abandon** branch — on a path this repo has registered nowhere, the route offers `--force` only | **[W]** arm 1; the quoted route carries no `milestone discard` arm |

### Increment 2 — the write path stops lying about identity

| # | Surface | Evidence |
|---|---|---|
| T7 | `write.identity-change` pre-check firing at **both** minting verbs (`doc create`, `doc author`) across `adr` / `research` / `spec`, each with an argv-complete route, **and the routed argv run verbatim to exit 0** | **[R]** cue-cards.md:88/242/329/413 + its verification list |
| T8 | `write.title-ignored` at the create door on a fixed-title singleton (`decisions-log`) — *"would be dropped silently"* | **[R]** cue-cards.md:123 |
| T9 | `jigc doc rename` — staged re-slug **and** the same-slug retitle ack (`— the id is unchanged`), on `adr` / `research` / `spec` / `arch-doc`, with slots, a repeatable item and its `{#id}` anchor surviving; `doc show --task` then serving the renamed staged copy | **[R]** cue-cards.md §0.7 + the independent review's live repro at :508-511 |
| T10 | `doc rename` refusing a **committed** identity, routing at `jigc rename` | **[R]** cue-cards.md:65 |
| T11 | `doc rename` refusing a singleton outright | **[R]** cue-cards.md:116 |
| T12 | `doc create` / `doc author` long help stating the title contract before the write | **[B]** all four sessions ran `doc author --help` (B1, B2, B3a, B3b) |
| T13 | The methodology `create-ledger` / `create-log` command-refs retitled to `Deferral Ledger` / `Decisions Log` — the pack's own consequence of the new title guard | **[B]** B2 ran both composed refs verbatim, exit 0 |

### Increment 3 — the read-back fence (the wave's claim)

| # | Surface | Evidence |
|---|---|---|
| T14 | The read-back instruction the fence forces into every write-soliciting step — *"Read your write back before you move on — with `--task` the read serves THIS task's staged copy"* + the exact command | **[B] 4/4 sessions**, unprompted: `doc show … --task` ×3 (B1), ×4 (B2), ×6 (B3a), ×7 (B3b). **The cue card never fired in any session**, so this is the fence's text alone |
| T15 | `jigc doc show` joining both packs' command catalogs — the menu `describe` projects | **[B]** B1 `describe` ×2, B3a `describe`, B3b `describe --format json` |
| T16 | `doc list --task <id>` — listing what a task stages | **[B]** B2, twice |
| T34 | `locate-from-spec`'s prose edit — the one write-soliciting step deliberately **outside** the fence family (it solicits through literal argv, so no structural signal reaches it) | **[B]** B3a and B3b both ran `implement-from-spec`, which composes it. The bound is that it declares no code, not that it went unread |

### Increment 4 — one foreign file, one code, one route at every door

| # | Surface | Evidence |
|---|---|---|
| T17 | The **task-scope** gate converging a foreign file on `schema-conformance.unadopted-instance` with the adoption route — the cell M42's sweep reached store-side and never here | **[B]** B3b: the code appears in `finding_codes` on `task validate` **and** `task finalize`, and the worker followed the route to `ingest` → `migrate … --as adr`, both exit 0 |

### Increment 5 — the install commit carries what it claims

| # | Surface | Evidence |
|---|---|---|
| T18 | The install-commit pathspec **derived** from where the hook landed — `core.hooksPath` shape, hook committed | **[B]** B1's in-session `setup` commit `087f0ce` carries `.githooks/pre-commit` (53 lines). Recorded independently as PT-4's drain |
| T19 | The same derivation on the **default `.git/hooks`** shape — the hook correctly **absent** from the install commit | **[B]** B2, B3a, B3b and arm 0 all ran on `.git/hooks`; all four install commits are the 8-path jigc-owned set with no hook path |

### Increment 6 — the cascade's read rung, and read intents that route to reads

| # | Surface | Evidence |
|---|---|---|
| T20 | `jigc config get <key>` — resolved value + the layer it came from | **[W]** arm 4: `config get docs-root` → `docs-root = docs/  (pack-default)` |
| T21 | `jigc config list` | **[W]** arm 4 |
| T22 | jigc rendering the unknown-subcommand block itself — a read intent answered by a **read** verb, clap's contradicting suggestion gone | **[W]** arm 4: `jigc doc read` → a tip naming `doc list` and `doc show <address> --task` |

### Increment 7 — the additive-key window closes

| # | Surface | Evidence |
|---|---|---|
| T23 | `doc schema`'s pinned projection at **contract-version 5**, including the id-source `write-key` marker | **[B]** B2 (`research`, `roadmap`, `deferral-ledger`, `decisions-log`), B3a and B3b (`changelog`, `spec`, `arch-doc`, `adr`) — 11 calls; B3a then drove `doc add-item --title` ×7, the verb the marker names |
| T24 | `describe --format json`'s `router_hidden` key — the suppression carried structurally instead of only in prose | **[B]** B3b ran `describe --format json`; and two sessions independently reported the router-hidden gap, so the fact the key carries is the one workers needed |

### Increment 8 — the two behavioural smalls

| # | Surface | Evidence |
|---|---|---|
| T25 | `describe --workflows` — the menu asked for the part it needs | **[W]** arm 4 |
| T26 | `describe` still refusing a positional (`jigc describe <id>` is not built) — the boundary the new filter had to hold | **[B]** B2 ran `describe --help`, then `describe milestone` → **exit 2**. Not anticipated by the protocol; see the note below |

### Increment 9 — the tier-2 truth batch

| # | Surface | Evidence |
|---|---|---|
| T27 | **F7** — the `adr` `options` hint stops instructing an omission (*"leave it empty when the call was obvious; the heading renders either way"*) | **[B]** B1's composed `record-decision` carried it, and B1 wrote `#options` with `set-slot` rather than omitting it |
| T28 | **F9** — the seam-generated id-source statement naming the literal hyphen as a word boundary, rendered in every `{{schema:}}` payload skeleton | **[B]** B2, B3a and B3b authored every `--from-file` payload against that skeleton |
| T29 | **F13** — the pre-commit header states the intent, not the deed, on a **rejected** finalize | **[B]** B1 hit `finalize.commit-rejected` at exit 1 (the planted `core.hooksPath` hook) — the exact site |
| T30 | **F14** — `single-task`'s catalog entry names the changelog gate as a discriminating axis | **[B]** B3a and B3b both ran bare `start` (the router catalog); B3a then went on to hit the changelog gate |
| T36 | **F15** — planning's finalize naming `jigc milestone create` and `jigc milestone add-task`, with the `add-from-spec` bound stated | **[B]** B2, and it worked: `task finalize roadmap-…` → `milestone create "Bound series cardinality at ingest"` → `milestone add-task <id> "<intent>"` ×2, in that order, with `add-from-spec` never attempted. The trial's cleanest instance of a tier-2 wording fix changing what an agent does |

### Increment 10 — the adapter's first owned artifact

| # | Surface | Evidence |
|---|---|---|
| T31 | `setup` installs the version-stamped guide at `.claude/skills/jigc/SKILL.md` and **commits it** | **[B]** all four blind corpora + arm 0 carry it in the install commit (251 lines); **[W]** arm 2 installed it onto a corpus authored on rc.10 |
| T32 | Refuse-to-clobber a user-edited guide — `adapter-guide.user-modified`, advisory + route, `setup` not blocked | **[W]** arm 3 |
| T33 | `uninstall` removes the guide **while it is jigc's**, and keeps a user-edited copy with the ownership statement | **[W]** arm 3, both branches |

### Cross-cutting

| # | Surface | Evidence |
|---|---|---|
| T35 | The **rc.10 → rc.11 upgrade path** over a corpus authored end-to-end on rc.10 — `validate`'s binary-mismatch advisory, `doc show` reading what rc.10 wrote, `migrate-corpus` reporting nothing to do, `finalize`, and `rename`'s two in-flight refusals whose routes were each followed to success | **[W]** arm 2. Covered by nothing in the suite — the fixture builder constructs every state with the *current* binary |

---

## Column 2 · Test-fenced (29 surfaces)

The trial did not reach these. Each names the suite **and what that suite asserts**.

### Increment 1

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F1 | The leftover classifier's **full three-verdict axis** × the three destroying doors (the walk drove one verdict at all three doors) | `provision_leftover_guard::every_verdict_refuses_a_non_empty_leftover_and_leaves_the_planted_bytes_intact` — constructs all three verdicts, asserts refusal **and** that the planted bytes are still on disk afterwards; `every_verdict_clears_and_provisions_under_force` is its consent complement |
| F2 | `provision`'s refusal is **transactional** — the audit MEDIUM, where both original fixtures planted at the first path and masked it | `provision_leftover_guard::a_refusal_leaves_every_path_unprovisioned_wherever_the_leftover_sits` — plants the leftover at the first *and* at a later sub-task path, and asserts **no** path was provisioned in either case |
| F3 | `uninstall.staged-prose` — an open task's authored prose that no object DB has a copy of blocks the teardown; `--force` is the consent | `uninstall_worktree_guard::uninstall_refuses_while_an_open_tasks_authored_prose_lives_only_in_the_workbench` — drives the ordinary `setup → start → set-slot` loop, asserts the refusal keys on the staged `*.md` **set** and not on directory non-emptiness, then drives `--force` to completion; `..._refuses_a_pristine_skeleton_without_claiming_prose_it_cannot_see` is the law-1 complement |
| F4 | The uninstall route's **abandon** branch — offered only where this repo registered the path | `uninstall_worktree_guard::uninstall_route_keeps_the_abandon_arm_where_this_repo_registered_the_path` + `..._offers_no_abandon_arm_where_no_abandon_can_clear_the_block` (the walk drove the second branch only) |
| F5 | The corrected long help at `uninstall` and `milestone discard` | `uninstall_worktree_guard::uninstall_long_help_states_the_refusal_and_names_its_escape_hatch`; `milestone_discard::discard_long_help_states_the_widened_refusal_and_what_force_really_does`. **No session or walk arm ran either `--help`** — B2 ran five other `--help` forms |

### Increment 2

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F6 | The rename census over the **whole doctype registry**, and every fixed-identity refusal | `doc_rename_in_task::the_rename_census_partitions_the_whole_doctype_registry` (no doctype unclassified) + `every_fixed_identity_doctype_refuses_the_rename` + `a_location_bearing_singleton_refuses_the_rename` + `the_milestone_record_refuses_through_the_shipped_machine_maintained_guard` |
| F7 | The re-slug destination guard answering over **both homes**, and the partial write behind it (audit LOW) | `doc_rename_in_task::the_reslug_destination_guard_answers_over_both_homes` — iterates `Occupant::ALL`; both occupied cells block with `write.already-present` keyed at the destination, stage nothing there, and leave the source doc **byte-untouched**, i.e. the refused rename is not a partial one |
| F8 | An orphaned role binding is not an incumbent (audit HIGH — M48's centrepiece creating a blocking dead end whose route could not run) | `write_title_divergence::a_binding_whose_doc_is_gone_is_not_an_incumbent_at_any_minting_verb` — asserts the retry lands at **both** minting verbs, the binding is re-pointed, and the complement holds (a real staged incumbent still blocks a divergent title) |
| F9 | The fixed-title axis at the write door, the CRLF incumbent, and the routes surviving a real shell over the metachar axis | `write_title_divergence::every_fixed_title_doctype_refuses_a_title_it_would_drop`, `a_crlf_committed_incumbent_takes_its_own_title_and_still_gates_a_divergent_one`, `the_identity_divergence_route_survives_a_real_shell_over_the_metachar_axis` |

### Increment 3

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F10 | The **pack-load fence** itself — the wave's mechanism (not a runtime surface; it carries no verb, finding or route a trial can drive) | `read_back_fence::the_dev_owe_set_is_exactly_its_read_back_declarers` — the owe-set computed from the tree's own catalog + steps **equals** the declaring set, both directions; `every_write_soliciting_dev_step_is_fenced_at_pack_load` — withdraws each member's declaration in turn from a `JIGC_PACK_DIR` copy and requires the composing binary to block. Methodology twins alongside |
| F11 | `doc list`'s empty-set line, its route to the staged read, and the staging-task-count branch | `doc_list::a_task_less_listing_routes_at_the_staged_read` — the advisory rides **stderr** so the listing stays byte-identical, the route is lifted from the emitted bytes and run verbatim, and with nothing staged there is no advisory at all. Circumstantially the trial may have used it (B2 ran `doc list` then `doc list --task`) but stderr was not captured, so it is scored here, not above |

### Increment 4

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F12 | The **managed** cell's stamp-keyed route (below-version/absent → migrate-corpus; at-version → the hand-repair sanction) | `foreign_at_both_doors::the_managed_advisory_routes_on_the_stamp_never_at_adoption` — iterates all four stamp states end to end through the real binary. Unreachable by this trial: every blind corpus was authored on rc.11, and walk arm 2's rc.10 corpus was already current |

### Increment 5

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F13 | The **local-only clause** in the setup summary and the `hook_committed` JSON key — the audit LOW, on the shape most adopters are on | `setup::setup_commits_the_pre_commit_hook_iff_it_is_a_working_tree_file` — over the whole hooks-dir axis: commit membership **equals** committability, nothing inside git's control dir is ever staged, the rest of the commit is exactly `INSTALL_COMMIT_BASE_PATHS`, and a second `setup` mints no second commit. The *behaviour* was trial-reached on both shapes (T18/T19); the *clause* was printed only to the operator running setup and nobody recorded reading it |

### Increment 6

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F14 | `config.undeclared-key` — a routed rejection | `config_read::config_get_of_an_undeclared_key_is_a_routed_rejection` — non-zero, stable code, routes to the verb that enumerates the surface. §5 arm 4 chartered a typo'd `config get`; the walk record does not carry it |
| F15 | `config list` emitting exactly the declared knob set; `config get` naming the project layer and a soft-rejected set's floor | `config_read::config_list_emits_exactly_the_declared_knob_set` (iterated from the loaded knob set, so a new knob joins with no edit), `config_get_names_the_project_layer_for_an_applied_override`, `config_get_names_the_attempted_value_and_the_floor_of_a_soft_rejected_set`. The walk drove the pack-default arm only |
| F16 | The read-intent rule over the **whole verb axis** — the universality claim a curated table could not deliver | `unknown_subcommand_tip::no_read_intent_is_answered_with_a_write_verb` — for every `(parent, read-shaped guess)` enumerated from the clap tree, **every** verb reference in the emitted stderr classifies `VerbKind::Read`, at least one read verb is named, and once per parent the first backticked span is run verbatim; `cli_parse::every_leaf_verb_is_classified` fences `VERB_KINDS` total against the tree. The walk drove one guess |

### Increment 7

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F17 | The standing **text/JSON parity fence** — the pre-1.0 additive-key window's closure | `text_json_parity_axis::the_parity_registry_bijects_the_clap_leaf_verbs` + `every_fenced_renderer_carries_on_the_wire_what_its_text_prints` — the witness value is destructured **exhaustively**, so the compiler is the field enumerator and a new field cannot be printed-but-withheld without breaking the build. Judgment-tier prose is a one-time census, not this fence |
| F18 | All six `ConfigAck` variants stating the uncommitted write in text **and** on the wire | `config_ack_uncommitted::every_config_ack_states_its_uncommitted_write_in_text_and_on_the_wire` + `config_ack_all_bijects_against_the_clap_config_verbs` (so a seventh variant cannot ship silently) |
| F19 | `setup --format json`'s `hook_file` | `setup::setup_json_names_the_installed_hook` / `setup_json_names_the_core_hookspath_hook`. No trial participant ran `setup --format json` |

### Increment 8

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F20 | An **idempotent** `rename` acking the no-op instead of dressing git's empty commit as a hook rejection — swept over `COMMITTING_DOORS` × the empty-commit outcome | `flow37_rename::idempotent_retitle_acks_the_no_op_and_never_claims_a_rejection` (reads the invocation-log record for the argv) + `commit_rejected_axis::no_committing_door_dresses_an_empty_commit_as_a_rejection` — every code-side committing door driven into the empty-commit state **with no hook installed anywhere**. Walk arm 2 ran `rename` three times, none of them idempotent |
| F21 | `describe --doctypes` / `--commands` and the combination rule | `describe::describe_kind_filter_selects_which_entries_the_menu_returns`. The walk drove `--workflows` only |

### Increment 9

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F22 | **F7**'s step-body half over both packs | `optional_slot_guidance::no_step_paragraph_guiding_a_persisted_optional_slot_instructs_an_omission` — the *subject* is derived from the loaded model (which paragraph, in which step, follows from the slot ids), the *predicate* is a closed six-phrase vocabulary. The suite's own doc comment calls this **a probe, not an axis-complete fence**; the residue is N1 below |
| F23 | **F11** — `migrate-corpus`'s headline over the whole run-mode axis | `corpus_migration::migrate_corpus_headline_states_its_run_mode_over_the_whole_axis` — 3 run modes × 2 corpus states, each headline byte-exact. Walk arm 2 rendered only the *already current* cell, which is the branch F11 did **not** change |
| F25 | **F13**'s second site — the pre-commit header on a finalize that is *not* rejected | `finalize_message_truth::both_pre_commit_headers_state_the_intent_and_a_rejected_finalize_never_reads_as_done` — both present-tense sites, and the rejected path never reads as done. B1 reached the rejected site only (T29) |

### Increment 10

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F26 | `jigc upgrade`'s guide reading, over its three states | `adapter_artifact::upgrade_reports_a_user_modified_guide_and_writes_nothing` — names the artifact, exits 0, leaves the artifact **and** the whole `.jigc/config/` project layer byte-identical (upgrade is a `VerbKind::Read`); plus `upgrade_over_a_pristine_guide_names_it_in_the_clean_line` and `upgrade_with_no_installed_guide_claims_no_guide_check`. §5 arm 3 **deliberately** steers away from `upgrade`, so nothing in the trial could reach it |
| F27 | Stale-stamp replacement, a profile with no guide target, every not-jigc artifact shape, `--force` removal, and the fail-closed `setup.write-guide` / `setup.guide-target` reads | `adapter_artifact::a_pristine_stale_stamped_guide_is_replaced_and_restamped`, `a_profile_with_no_guide_target_installs_nothing_and_exits_zero`, `every_shape_of_a_not_jigc_artifact_survives_setup_byte_identical`, `force_removes_a_user_modified_guide_with_the_rest_of_the_install`, `the_installed_guide_carries_no_in_repo_relative_link` |

### Increment 11

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F28 | The **manifest-hash freeze fence** + the `Manifest-Repin: <entity>` escape | **§6 pre-states this as not trial-testable, and that stands**: it fires in CI over a pushed range and carries no verb, finding or route. `manifest_freeze_fence` fences it in three layers — the verdict axis (`hash_moved_version_equal_is_a_violation` … `the_move_product_is_a_table_with_no_unnamed_cell`, fail-closed on an unparseable manifest at either end), the escape (`the_named_entity_is_excused_and_its_neighbour_is_not`), and the **window** (`the_repin_of_all_sixteen_is_flagged_over_the_pushed_range` against `the_head_tilde_one_window_at_the_push_tip_is_clean` — the measurement that falsified the settled shape). The live arm is `#[ignore]`d and invoked by a named CI step, itself fenced by `the_ci_step_is_named_and_runs_the_ignored_arm_verbatim` |

### Increment 12

| # | Surface | Fence — and what it actually asserts |
|---|---|---|
| F29 | Flow 48, the 612-golden regeneration from an emptied root, the registration fence, and the fold-back truth checks | Build fences carrying no verb, finding or route — nothing for a trial arm to drive. `flow48_acceptance` (six arms, eight tests, each enumerating its class axis from a code-side registry); `test_target_registration::every_group_root_is_declared_as_a_test_target` + `every_aggregated_submodule_is_declared_by_its_aggregator` (reads `Cargo.toml`, not the disk); `foldback_truth::the_pack_step_count_is_stated_once_and_names_its_measurement_point` |

---

## Column 3 · Neither (3 surfaces) — each explained

The column is not empty, and each member is here because it is genuinely reached by nothing — not
because it was overlooked.

**N1 · F7's residue: a novel omission phrasing outside the closed six-phrase vocabulary.**
*Increment 9, and the wave's own declared bound.* The step-body probe (F22) derives its **subject**
from the loaded model but keeps a **closed** predicate vocabulary, so a guidance paragraph that
instructs an omission in words the vocabulary does not contain passes. Nothing reached it, and
nothing can: a trial cannot drive the absence of a phrasing, and a fence over the general case would
be the grep the M42 lesson forbids. It is carried in [VERDICT.md](../M48/VERDICT.md) → Honest bounds
2, and this row is its coverage disposition rather than a new finding.

**N2 · `milestone provision`'s mid-mutation failure.** *Increment 1 + the audit MEDIUM fix.* The
refusal is now two-phase and transactional (F2), but a `git worktree add` that fails **inside** phase
2 still leaves earlier paths provisioned, and nothing rolls that back. No suite constructs it — F2
fences the *refusal*'s transactionality, not a failure of the mutation phase — and no trial arm can
force `git worktree add` to fail partway. Recorded as VERDICT bound 7 and stated in both design
homes. An idempotent re-run reuses the provisioned paths, so the state is recoverable; that is why
it was taken as a bound rather than a defect.

**N3 · The guide artifact's missing delta discipline.** *Increment 10.* Refuse-to-clobber (T32) is
an untracked-fork **detector**; the cascade's *"all customization is a recorded delta against a known
base"* invariant has no instance here, so there is no delta to test and no route to drive. Recorded
as a declared deviation from principle #5 with its own trigger — the discipline is owed when a
second adapter artifact appears. Listed rather than dropped because M48 changed the adapter's
ownership model and this is the half of that change nothing checks.

---

## Counts

| Column | Surfaces |
|---|---|
| **Trial-reached** | **36** — 18 **[B]** blind · 13 **[W]** walk · 5 **[R]** rehearsal (see below); T31 was reached by both blind and walk and is counted once, as blind |
| **Test-fenced** | **28** |
| **Neither** | **3**, each explained above |
| **Total** | **67** |

Per increment, at least one surface is trial-reached in **10 of 12**. The two exceptions are
Increment 11 (§6 pre-states it as not trial-testable) and Increment 12 (build fences only — the
wave's own roadmap records that Increment 11 deliberately has no flow-48 arm *"because a CI build
fence carries no verb, finding or route for a done-picture walk to reach"*, and the same reasoning
covers Increment 12's own deliverables).

**The [R] tier, and what changes if it is disallowed.** Five rows — T7 through T11, the whole of
Increment 2's runtime surface — rest on the operator's cue-card rehearsal on the live rc.11 host
binary, with verbatim output, and **not** on a blind session or a walk arm. The precedent for
counting them is PT-1, which this trial already treats as a finding *"found by an operator rehearsing
a plant rather than by a blind worker"*. If the human reads "the trial" as sessions + walk only,
those five move to **test-fenced** — F6, F7, F8 and F9 already assert every one of them over its
class axis — and the counts become **31 / 33 / 3**. **No row moves to *neither* under either
reading**, which is the load-bearing fact.

---

## Five things the derivation found that the protocol did not anticipate

**1 · The whole of Increment 2 escaped both blind sessions and the walk.** `doc rename` appears
**zero** times across all four blind invocation logs and the control, and `write.identity-change` /
`write.title-ignored` appear in **no** session's `finding_codes`. The reason is structural: §3.2's
mid-task correction was the designed occasion for exactly that path, and [session-findings.md](session-findings.md)
records it **never fired in any session** — *"the correction path is untested, not passed."* So the
protocol's own instrument failure and this coverage gap are the same event, and the wave's
second-largest increment is carried by tests and a rehearsal.

**2 · §5 arm 4 chartered two probes the walk record does not carry.** `doc rename` on a **committed**
identity (*"expect the refusal, not exit 0"*) and a typo'd `jigc config get`. [v1-walk.md](v1-walk.md)
arm 4 records `config get`, `config list`, `describe --workflows` and `jigc doc read` only. Arm 2's
three `jigc rename` attempts are the **top-level** verb, a different surface from `jigc doc rename`.
Both chartered probes land in the test-fenced column (F6/F7 and F14).

**3 · B2 read `describe --help` and then reached for the form the help says is not built.** The
sequence is `describe --help` (exit 0) → `describe milestone` (**exit 2**). The help it had just read
names `--workflows` / `--doctypes` / `--commands` **and** states *"The single-item form
(`jigc describe <id>`) is not built"*, and it also names `jigc doc schema` — the very capability B1
reported as missing in D-1. Two independent instances, in one trial, of a capability named on a
surface the worker demonstrably rendered and did not take. That is the discoverability lens with a
sharper edge than the sessions' own accounts give it, and it is evidence *about* D-1 rather than a
separate finding.

**4 · The install-commit pathspec was exercised on both hooks-dir shapes, by accident of the rig.**
B1 ran on `core.hooksPath` (hook committed); B2, B3a, B3b and arm 0 all ran on the default
`.git/hooks` (hook correctly absent). The protocol charters neither — it is a by-product of B1's plant
— but it means Increment 5's *behaviour* has field evidence on both shapes, which is more than the
wave itself claimed. Only the audit LOW's summary **clause** (F13) stayed unread.

**5 · A tier-2 wording fix demonstrably changed what an agent did — and this derivation nearly
recorded the opposite.** Increment 9's F15 added `jigc milestone create` and `jigc milestone add-task`
to planning's finalize because RC-pre-1.0's planner *"committed a roadmap entry and believed it had
opened a milestone."* B2's log reads: `task finalize roadmap-…` → `milestone create "Bound series
cardinality at ingest"` → `milestone add-task <id> "<intent>"` ×2, in the fix's own order, with the
tempting-but-wrong `add-from-spec` never attempted. The first pass of this table filed F15 as
*unreached*, on B2's earlier `start --workflow planning` exiting 1 — a call that failed for want of
an intent, not for want of the workflow, which the router then selected on the very next line. The
correction is recorded rather than quietly applied, because it is the same failure mode M47's
derivation was faulted for: reading a surface's fate from one adjacent signal instead of following
what the agent actually did next.

---

## What the protocol asked for and the trial did not produce

**Whether the guide artifact was *used*.** §4 requires recording *"whether it is **used**"* — not
found, since the harness auto-advertises `.claude/skills/`. **No feedback report mentions the skill,
the guide, or `SKILL.md` at all** (grep over all four `feedback-*.md`: zero hits). Installation and
teardown are covered (T31/T32/T33); **use is unmeasured**, and should be reported as unmeasured
rather than as a null. This is not a coverage gap in the M48 diff — the artifact's surfaces are all
placed above — but it is an unanswered question the protocol asked in writing.
