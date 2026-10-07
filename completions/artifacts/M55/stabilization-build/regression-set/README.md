# The regression set's first part — the records of its runs

## The runs, and which is the current evidence

| run | candidate | the list the verdict was given over | verdict | record |
|---|---|---|---|---|
| 1 | `fix/rc24-tier1` at `d423f09b` — the product **before** the eleven pre-opening fixes | a file of the working tree, untracked when the run read it; 21 rows, sha256 `191d0a00…` | green: 21 differences, all 21 listed | this file, below; [result.json](result.json) |
| 2 | `fix/rc24-regression` at `8f13ea7b` — the product as merged at `b33bf39b`, with the eleven fixes | the candidate's commit's; the same 21 rows, sha256 `191d0a00…` | red: 22 differences, 1 not on the list | [run-2.md](run-2.md); [run-2-result.json](run-2-result.json) |
| **3** | `fix/rc24-regression` at `b36b8c93` — the same product; the commit that holds the rows | the candidate's commit's; 33 rows — 22 for a change, 11 exclusion rows — sha256 `bfdad1af…` | **green: 22 differences, all 22 listed; 11 tests fail on their own binary, each by its row** | [run-3.md](run-3.md); [run-3-result.json](run-3-result.json) |

**The current evidence is run 3, for the candidate `b36b8c93`: green** — on the list that commit holds, with `excluded` exactly its eleven exclusion rows. It is evidence for the product as merged at `b33bf39b`; a commit that changes a product path, or the list, is another candidate and wants a run of its own. **Run 2** is what named the one difference the eleven fixes made in the old suite — it belongs to a ruled fix, item 21, `1c7d391c` — and the eleven tests the rows exclude; [run-2.md](run-2.md) has the test, the checks, and what each of the eleven's own failure says. **Run 1's green is no evidence for the tree as it stands**, for two reasons: its candidate is the product before the eleven fixes, and its list was read from a file no commit held — the call it was made with is refused by the tool as it now is (`DECISIONS.md` → *2026-10-07 — The regression tool's green rests on facts of the two commits*).

**Everything below this line is the record of run 1, as it was written.**

**Run 2026-10-07 on macOS (aarch64, 10 cores) — rustc 1.95.0, cargo-nextest 0.9.143, git 2.54.0 — by [`dev/regression-set`](../../../../../dev/regression-set) as commit `d423f09b` holds it.** `<repo>` is the main checkout, `<scratch>` the scratch root of the run; nothing else of the host is named here. The check, what it establishes and what it cannot see: [implementation/stabilization-workflow.md](../../../../../implementation/stabilization-workflow.md) → The regression set. The choices inside the tool: `DECISIONS.md` → *2026-10-06 — The regression set's first part, as a tool*. This record restates neither.

- **Previous release:** `jigc 1.0.0-rc.24` — tag `jigc-v1.0.0-rc.24`, which peels to `91834b5e011de2c36e2be2b79e96c0b9f60a803c`.
- **Candidate:** `fix/rc24-tier1` at `d423f09bf1bdc98fa2169134e607a8c0b203919f` — the commit that added the tool. It changes no product path: the product is the fix pass's, last changed at `0f34d8f0`.
- **The list:** [intended-changes.tsv](intended-changes.tsv), 21 rows, sha256 `191d0a009de2ec86b8d3fb02b02f2487797791f479c5328c83ba25450c9f1424` — untracked in the tree when the run read it, and committed with this record byte for byte.

## The verdict

**Green: every test that passes on the previous release's binary and fails on the candidate's is a row of the list.** 21 differences, 21 on the list, **none off it**; 0 stale rows; no test needed its retry on either binary.

It is a floor and not the clause's instrument whole: about half of the old suite never reaches the binary, the old tests build only the states they build, and the second part of the regression set — the success paths across configurations and layouts — is not built.

## The command

```sh
cd <repo>
dev/regression-set run \
    --previous 91834b5e011de2c36e2be2b79e96c0b9f60a803c \
    --candidate d423f09bf1bdc98fa2169134e607a8c0b203919f \
    --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv \
    --scratch <scratch> > <scratch>/line.json
```

Exit status 0, nothing on stderr. [result.json](result.json) is the one line it printed, with the host's paths replaced by the two placeholders and laid out one field per line; nothing else in it is changed.

What the tool ran, in order — every one from `<scratch>/regression-set.<minted>/`, the cargo commands with the real git first on `PATH` and `SDKROOT` named, as `dev/gate` runs its own:

```sh
git -C <repo> archive --format=tar -o previous.tar 91834b5e011de2c36e2be2b79e96c0b9f60a803c && tar -xf previous.tar -C previous
( cd previous && CARGO_TARGET_DIR=../target-previous cargo nextest run --workspace --locked --no-run )
( cd previous && CARGO_TARGET_DIR=../target-previous cargo nextest list --workspace --locked \
      --list-type binaries-only --message-format json > ../evidence/binaries-metadata.json )
( cd previous && cargo metadata --format-version=1 --all-features --locked > ../evidence/cargo-metadata.json )
git -C <repo> archive --format=tar -o candidate.tar d423f09bf1bdc98fa2169134e607a8c0b203919f && tar -xf candidate.tar -C candidate
( cd candidate && CARGO_TARGET_DIR=../target-candidate cargo build -p jigc --locked )
# sha256 of target-previous/debug/jigc: the previous release's
( cd previous && cargo nextest run --no-fail-fast --retries 1 \
      --binaries-metadata ../evidence/binaries-metadata.json --cargo-metadata ../evidence/cargo-metadata.json \
      --tool-config-file regression-set:../evidence/nextest.toml --profile regression-baseline )
# sha256 again; then target-candidate/debug/jigc copied beside the path and renamed over it; sha256: the candidate's
( cd previous && cargo nextest run --no-fail-fast --retries 1 \
      --binaries-metadata ../evidence/binaries-metadata.json --cargo-metadata ../evidence/cargo-metadata.json \
      --tool-config-file regression-set:../evidence/nextest.toml --profile regression-candidate )
# sha256 again
```

The tool passes absolute paths where this block writes relative ones.

## The numbers

| | tests | passed | failed | seconds |
|---|---|---|---|---|
| the previous release unpacked, its tests built and the build recorded | | | | 123 |
| the candidate unpacked and its binary built | | | | 98 |
| baseline: the old tests on the old binary | 4393 | 4382 | 11 | 783 |
| the same test binaries on the candidate's binary | 4393 | 4361 | 32 | 1033 |

- **Passed on the baseline and failed on the candidate: 21.** On the list: 21. **Not on the list: 0.**
- **Failed on their own binary, and excluded: 11** — of which 0 pass on the candidate's.
- **Rows of the list that matched nothing: 0.**
- **Passed on a retry:** 0 on the baseline, 0 on the candidate.
- **Wall time: 2045 s**, end to end, on a machine other agents were using (load average between forty-five and eighty; the measurement, alone, took about twenty minutes).
- 15 test binaries: `jigc-engine`, the `jigc` library, the `jigc` binary's own tests, and twelve group targets.

## Which binary ran

The path the old tests have compiled in, read out of the recorded build: `<scratch>/regression-set.oq93qke8/target-previous/debug/jigc`. **12 test binaries hold that path in their bytes** — the twelve group targets; the three unit-test binaries do not, and cannot reach the binary.

| sha256 of the file at that path | |
|---|---|
| the previous release's binary, as its tests' build left it | `56d5596e89fe92e6c972fdab8bf0394bf14ed627e5c905483e71958a5111e792` |
| before the baseline | `56d5596e89fe92e6c972fdab8bf0394bf14ed627e5c905483e71958a5111e792` |
| after the baseline | `56d5596e89fe92e6c972fdab8bf0394bf14ed627e5c905483e71958a5111e792` |
| the candidate's binary, as its own build left it | `007110b0386f10b17d4674714ff5eeb1ee66a28d8ad66aaa87072946b38532ab` |
| after the swap, before the second run | `007110b0386f10b17d4674714ff5eeb1ee66a28d8ad66aaa87072946b38532ab` |
| after the second run | `007110b0386f10b17d4674714ff5eeb1ee66a28d8ad66aaa87072946b38532ab` |

The first three are one hash and the last three another: the baseline drove the previous release's binary, the second run the candidate's, and nothing put the old one back. Both binaries print `jigc 1.0.0-rc.24` — no version has been bumped — which is why a hash and not a version is what identifies them.

## The differences, each on the list

| # | binary :: test | the row's pointer |
|---|---|---|
| 1 | `jigc::g_config` :: `flow19_planning_encode::flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize` | ruling: `DECISIONS.md` → *2026-10-05 — rc.24 fix pass, the copied-in witness* |
| 2 | `jigc::g_doc` :: `clap_error_kind_axis::every_producible_kind_holds_with_the_invocation_log_on` | commit `12398ddc` |
| 3 | `jigc::g_doc` :: `clap_error_kind_axis::every_producible_kind_is_reached_by_driving_its_probe` | commit `12398ddc` |
| 4 | `jigc::g_finalize` :: `pre_guard_repair_route::the_migration_source_route_does_not_promise_a_review_it_does_not_get` | commit `4ba04efc` |
| 5 | `jigc::g_finalize` :: `reconciliation_baseline_contrast::a_lost_file_state_baseline_turns_a_conflict_block_into_a_silent_merge` | ruling: `DECISIONS.md` → *2026-10-05 — rc.24 fix pass, the copied-in witness* |
| 6 | `jigc::g_flow` :: `flow48_acceptance::the_guide_artifact_is_installed_replaced_and_never_clobbered` | commit `33620081` |
| 7 | `jigc::g_flow` :: `flow49_acceptance::a_lost_baseline_switches_off_the_never_silently_merged_guarantee` | ruling: `DECISIONS.md` → *2026-10-05 — rc.24 fix pass, the copied-in witness* |
| 8 | `jigc::g_flow` :: `flow52_acceptance::a_refused_transaction_restores_its_config_layer_writes_without_overwriting_a_concurrent_edit` | commit `104a7d4b` |
| 9 | `jigc::g_item` :: `singleton_running_doc::warm_edit_over_an_oob_drifted_singleton_conflict_blocks_at_finalize` | ruling: `DECISIONS.md` → *2026-10-05 — rc.24 fix pass, the copied-in witness* |
| 10 | `jigc::g_migrate` :: `adapter_artifact::a_pristine_stale_stamped_guide_is_replaced_and_restamped` | commit `33620081` |
| 11 | `jigc::g_migrate` :: `config_layer_preimage::a_path_jigc_never_wrote_is_neither_restored_nor_reported` | commit `104a7d4b` |
| 12 | `jigc::g_migrate` :: `config_layer_preimage::every_config_layer_pathspec_carries_a_disposition` | commit `104a7d4b` |
| 13 | `jigc::g_migrate` :: `config_layer_preimage::the_manufactured_shape_space_holds_over_both_files` | commit `104a7d4b` |
| 14 | `jigc::g_migrate` :: `setup::setup_writes_compose_embedded_methodology_marker` | ruling: `DECISIONS.md` → *2026-10-04 — rc.24 fix pass, two rulings after the first fixes* |
| 15 | `jigc::g_migrate` :: `setup_install_pathspec_guard::an_owned_guide_artifact_is_not_a_subject_of_the_guard` | commit `33620081` |
| 16 | `jigc::g_migrate` :: `version_stamp_rollback::a_hook_rejected_finalize_restores_a_staged_blob_at_a_promotion_destination` | commit `78e8ded1` |
| 17 | `jigc::g_milestone` :: `cwd_verb_subject::uninstall_removes_the_workbench_home_install_from_every_cwd` | commit `dcfa40f0` |
| 18 | `jigc::g_milestone` :: `leftover_probe_fail_closed::a_teardown_that_removed_nothing_narrates_nothing` | commit `1b1d7656` |
| 19 | `jigc::g_milestone` :: `milestone::milestone_finalize_warns_on_a_leaked_worktree_but_still_succeeds` | commit `8c159622` |
| 20 | `jigc::g_milestone` :: `milestone_landed_attribution::a_path_landed_by_two_chain_commits_is_one_manifest_entry_owned_by_the_last` | commit `048724d0` |
| 21 | `jigc::g_milestone` :: `milestone_record_stale_base::stale_base_routes_finalize_without_committing` | commit `8d9c3afb` |

What changed, per row, is the list's third field and is not repeated here.

## The list as seeded

The 21 rows are the 21 tests of the measurement of 2026-10-06 — the spike that proved the method, before the tool existed; its record, `pre-opening/regression-spike.md` beside this directory, is on the branch that prepares the run's opening and reaches this one with it. **Each was checked against the record of the fix pass before it was copied**, and three things were checked per row:

1. **The commit is in the range and touches the test's own file** — `git show --stat <commit> -- crates/cli/tests/<suite>.rs` names the file for every row but the two of `clap_error_kind_axis`, whose file the pass did not touch (below).
2. **The fix pass's record names the commit and the behaviour** — `DECISIONS.md`, the entries *rc.24 fix pass: round 3 as built*, *round 4 as built*, *the fixes as built* and *the copied-in witness*, and [the pass's record](../../fix-pass-rc25/README.md). All thirteen commits the rows rest on are named in both.
3. **What the old test asserts is what the record says changed** — read from the measurement's table against those entries.

Where the row's pointer departs from the measurement's table:

- **Five rows point at a ruling of the human's and not at a commit**, because the record has one. Rows 1 to 4 of the measurement — an edit made before a task's first write lands, baseline or no baseline — are consequence 1 of *the copied-in witness* (option A), built at `0f34d8f0`; `setup::setup_writes_compose_embedded_methodology_marker` is item 2 of *two rulings after the first fixes*: the refusal over a pre-seeded `packs.yaml`, built at `dc0d7586` and kept.
- **The two `clap_error_kind_axis` rows are no behaviour difference.** The test compares the binary's `--help` with the render of the previous release's own `cli` library, linked into the test binary; the swap splits the two halves of one parity check. The pass reworded the `setup` and `uninstall` help paragraphs in several commits; the rows point at the first that changed one, `12398ddc`, which reworded `uninstall`'s. The measurement classed them apart from the nineteen; here they are rows like any other, since the tool knows one kind of row.
- **`33620081` rests on a thin record**, as the measurement noted: one sentence in `DECISIONS.md`, and absent from *What newly refuses against rc.24*. The bound its three tests hit is in the commit's message and in `design/assistant-adapter.md`.

## The 11 tests that fail on their own binary

All for one reason: **the previous release is built from `git archive`, and an archive is no repository.** Each asks git about its own source tree or its history. *Reaches the binary* is the measurement's calibration — the old suite run once with a stub at the path: a test that still passed there does not depend on the binary.

| binary :: test | what it asks git for | reaches the binary |
|---|---|---|
| `jigc::g_config` :: `set_kind_vocabulary::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical` | `git ls-tree -r HEAD` in its own source tree | **yes** |
| `jigc::g_config` :: `stated_at_fence::every_declared_where_reachable_row_is_declared_by_the_step_it_names` | `git ls-files` in its own source tree | no |
| `jigc::g_finalize` :: `manifest_freeze_fence::historical::the_fixture_commits_are_reachable_and_a_whole_push_apart` | a commit of this repository's history, unreachable | no |
| `jigc::g_finalize` :: `manifest_freeze_fence::historical::the_head_tilde_one_window_at_the_push_tip_is_clean` | a commit of this repository's history, unreachable | no |
| `jigc::g_finalize` :: `manifest_freeze_fence::historical::the_repin_of_all_sixteen_is_flagged_over_the_pushed_range` | a commit of this repository's history, unreachable | no |
| `jigc::g_finalize` :: `manifest_freeze_fence::record::every_home_states_what_is_now_fenced` | `git ls-files` in its own source tree | no |
| `jigc::g_finalize` :: `manifest_freeze_fence::record::no_home_still_says_the_rule_is_unfenced` | `git ls-files` in its own source tree | no |
| `jigc::g_migrate` :: `dev_gate_report::a_clean_denylist_says_clean` | `dev/gate`'s hygiene scan, which reads git's range and tree | no |
| `jigc::g_migrate` :: `dev_gate_report::a_denylist_hit_is_named_by_location_and_never_by_content` | `dev/gate`'s hygiene scan, which reads git's range and tree | no |
| `jigc::g_migrate` :: `install_line::the_install_line_has_one_owner_one_copy_and_one_derived_file` | `git ls-files` in its own source tree | no |
| `jigc::g_migrate` :: `package_contents::every_tracked_file_under_the_published_dirs_is_published` | `git ls-files` under the published directories | no |

**So the archive costs this check one test that could have seen the candidate**: `set_kind_vocabulary::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical`. In the measurement, which built from a linked worktree, it passed on both binaries.

## What this run did not do

- **It is one run.** Nothing was re-driven on either binary; the tool's one retry is what stood between a flake and a difference, and no test needed it.
- **The rehearsal before it** — the same call on the commit before the tool's, `3129ab91` — gave the same counts in 1,501 s. Its line is not kept: the candidate's binary is the same source.
- **Nothing was fixed and no row was added after the run.** The list here is the list the run read.
- **`dev/hygiene-scan`** was run over this directory with its files staged — the denylist over the tracked tree — and over the commit's range after it; the gate's own scan, gitleaks included, ran on the same tree. No hit.
