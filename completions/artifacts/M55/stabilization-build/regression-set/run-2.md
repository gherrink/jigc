# The regression set's first part — run 2: the first run on the merged tree

**Run 2026-10-07 on macOS (aarch64, 10 cores) — rustc 1.95.0, cargo-nextest 0.9.143, git 2.54.0 — by [`dev/regression-set`](../../../../../dev/regression-set) as commit `8f13ea7b` holds it.** `<repo>` is the checkout the run was made from, `<scratch>` the scratch root of the run; nothing else of the host is named here. The check, what it establishes and what it cannot see: [implementation/stabilization-workflow.md](../../../../../implementation/stabilization-workflow.md) → The regression set. What the tool holds since run 1: `DECISIONS.md` → *2026-10-07 — The regression tool's green rests on facts of the two commits*. This record restates neither. The index of the runs, and which of them is the current evidence: [README.md](README.md).

- **Previous release:** `jigc 1.0.0-rc.24` — tag `jigc-v1.0.0-rc.24`, which peels to `91834b5e011de2c36e2be2b79e96c0b9f60a803c`.
- **Candidate:** `fix/rc24-regression` at `8f13ea7b356ee955325bfe81c9c2bc35da6b7620` — the commit that made the tool read its list out of the candidate's commit. It changes no product path. **The product is the one merged at `b33bf39b`: run 1's candidate plus the eleven pre-opening fixes** ([pre-opening-fixes-log.md](../pre-opening/pre-opening-fixes-log.md)).
- **The list:** [intended-changes.tsv](intended-changes.tsv) **as the candidate's commit holds it** — the tool read it with `git cat-file blob 8f13ea7b…:<path>`, not from the working tree: 21 rows, sha256 `191d0a009de2ec86b8d3fb02b02f2487797791f479c5328c83ba25450c9f1424`, byte for byte the list of run 1.

## The verdict

**Red: one test that passes on the previous release's binary and fails on the candidate's is not a row of the list.** 22 differences, 21 on the list, **1 off it**; 0 stale rows; no test needed its retry on either binary.

**It was expected not to be green.** The list was seeded for the candidate before the eleven fixes, and has no row for what they changed on purpose. Naming that is this run's job; the row is the next commit's, and the run on the commit that holds it is [run 3](README.md).

## The one difference that is not on the list

| binary :: test | the fix it belongs to | the ruling |
|---|---|---|
| `jigc::g_migrate` :: `setup::setup_commits_the_pre_commit_hook_iff_it_is_a_working_tree_file` | `1c7d391c` — *the teardown removes the settings entries setup recorded, and leaves an identical entry that was already there* (the fix pass's item 21) | the human's, 2026-10-06, option B: `DECISIONS.md` → *2026-10-06 — `jigc setup` records the settings entries it adds* |

**How it fails**, on both tries, at `crates/cli/tests/setup.rs:1420` of the previous release: *the install commit must carry exactly the install footprint* — the commit `jigc setup` makes carries nine paths where the old test expects eight. The ninth is `.jigc/settings-entries.json`.

**Checked against the fix before it was called intended** — the three checks of run 1's record:

1. **The commit is in the range and touches the test's own file.** `git show --stat 1c7d391c -- crates/cli/tests/setup.rs` names the file; the change there is this test's own constant, `INSTALL_COMMIT_BASE_PATHS`, from eight paths to nine, the added one `.jigc/settings-entries.json`, with a comment that names the ruling.
2. **The record names the commit and the behaviour.** The `DECISIONS.md` entry above (*the record is `.jigc/settings-entries.json` … committed with the install*); [pre-opening-fixes-log.md](../pre-opening/pre-opening-fixes-log.md) → *Second tree — item 21* (`1c7d391c`, *the eleventh install member*); and the ruling as the human gave it, [decisions.md](../pre-opening/decisions.md), the table's row for item 21 — *`setup` records which entries it added … the record is committed with the install* — and, on the fixer's halt, option B in his words in the fixes log.
3. **What the old test asserts is what the record says changed.** It asserts the exact set of paths in the install commit; the ruling adds one committed file to every install.

**No difference is left that no ruled fix stands behind.**

## What the other ten fixes moved in the old suite: nothing this check can see

The run names no second difference: **of the old tests that still passed on run 1's candidate, the ten other fixes moved none.** That is a statement about the old suite and not about the fixes — this check is a floor, and the second part of the regression set is not built. And it has one blind spot of its own, named here because several of the fixes touch files whose old tests are rows already: **a test that is a row fails either way and is not looked at again**, so what a later fix changed inside one of the 21 is not seen by this run.

## The command

```sh
cd <repo>
dev/regression-set run \
    --previous 91834b5e011de2c36e2be2b79e96c0b9f60a803c \
    --candidate 8f13ea7b356ee955325bfe81c9c2bc35da6b7620 \
    --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv \
    --scratch <scratch> > <scratch>/line.json
```

Exit status 1, nothing on stderr. [run-2-result.json](run-2-result.json) is the one line it printed, with the host's scratch root replaced by `<scratch>` and laid out one field per line; nothing else in it is changed. What the tool ran, in order, is the block of run 1's record with this run's two commits — the steps did not change — after one step more at the start: the list read out of the candidate's commit, and its path and hash written to `run.json`.

## The numbers

| | tests | passed | failed | seconds |
|---|---|---|---|---|
| the previous release unpacked, its tests built and the build recorded | | | | 73 |
| the candidate unpacked and its binary built | | | | 33 |
| baseline: the old tests on the old binary | 4393 | 4382 | 11 | 636 |
| the same test binaries on the candidate's binary | 4393 | 4360 | 33 | 546 |

- **Passed on the baseline and failed on the candidate: 22.** On the list: 21 — the list's 21 rows, each matched. **Not on the list: 1.**
- **Failed on their own binary, and excluded: 11** — of which 0 pass on the candidate's.
- **Rows of the list that matched nothing: 0.**
- **Passed on a retry:** 0 on the baseline, 0 on the candidate.
- **The steps sum to 1,288 s**; the wall time was not taken apart from them. The machine was one other agents were using, at a load average between fifty and sixty.
- 15 test binaries, as in run 1.

## Which binary ran

The path the old tests have compiled in, read out of the recorded build: `<scratch>/regression-set.d12bhw72/target-previous/debug/jigc`. **12 test binaries hold that path in their bytes.**

| sha256 of the file at that path | |
|---|---|
| the previous release's binary, as its tests' build left it | `00d3975a2702ee7703153feafa0b386f5869b79d5a934888d7c5d65f7a377a04` |
| before the baseline | `00d3975a2702ee7703153feafa0b386f5869b79d5a934888d7c5d65f7a377a04` |
| after the baseline | `00d3975a2702ee7703153feafa0b386f5869b79d5a934888d7c5d65f7a377a04` |
| the candidate's binary, as its own build left it | `20b3bbcf214b1969f212e66dce2dcda63513332ec7310c7f3768a064a9bd2b6f` |
| after the swap, before the second run | `20b3bbcf214b1969f212e66dce2dcda63513332ec7310c7f3768a064a9bd2b6f` |
| after the second run | `20b3bbcf214b1969f212e66dce2dcda63513332ec7310c7f3768a064a9bd2b6f` |

The first three are one hash and the last three another. The previous release's binary does not hash as it did in run 1: it was built in another directory, and a build embeds its path — which is why the tool now refuses two commits that are one instead of trusting two hashes to differ.

## The 11 tests that fail on their own binary

The same eleven as in run 1, for the same one reason, **read this time out of the baseline's own report**: the previous release is built from `git archive`, and an archive is no repository.

| binary :: test | what its failure on its own binary says |
|---|---|
| `jigc::g_config` :: `set_kind_vocabulary::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical` | `git ls-tree -r --name-only -z HEAD` in its own source tree: *not a git repository* |
| `jigc::g_config` :: `stated_at_fence::every_declared_where_reachable_row_is_declared_by_the_step_it_names` | `git ls-files --cached --others --exclude-standard -z` in its own source tree: *not a git repository* |
| `jigc::g_finalize` :: `manifest_freeze_fence::historical::the_fixture_commits_are_reachable_and_a_whole_push_apart` | the commit `2c5eee5` of this repository's history *is unreachable in this clone* |
| `jigc::g_finalize` :: `manifest_freeze_fence::historical::the_head_tilde_one_window_at_the_push_tip_is_clean` | the commit `7ada302` *is unreachable in this clone* |
| `jigc::g_finalize` :: `manifest_freeze_fence::historical::the_repin_of_all_sixteen_is_flagged_over_the_pushed_range` | the commit `2c5eee5~1` *is unreachable in this clone* |
| `jigc::g_finalize` :: `manifest_freeze_fence::record::every_home_states_what_is_now_fenced` | `git ls-files` in its own source tree: *not a git repository* |
| `jigc::g_finalize` :: `manifest_freeze_fence::record::no_home_still_says_the_rule_is_unfenced` | `git ls-files` in its own source tree: *not a git repository* |
| `jigc::g_migrate` :: `dev_gate_report::a_clean_denylist_says_clean` | the old `dev/gate`'s hygiene scan, run in the unpacked tree, does not say it ran: it reads git's range and tree |
| `jigc::g_migrate` :: `dev_gate_report::a_denylist_hit_is_named_by_location_and_never_by_content` | the same scan reports no hit in the tracked tree: there is no tracked tree |
| `jigc::g_migrate` :: `install_line::the_install_line_has_one_owner_one_copy_and_one_derived_file` | `git ls-files`: *not a git repository* |
| `jigc::g_migrate` :: `package_contents::every_tracked_file_under_the_published_dirs_is_published` | `git ls-files` under the published directories: *not a git repository* |

**They are excluded here by the tolerance the tool still has** — at most one test in a hundred — **and by no row.** The tool's next change ends that: each of the eleven becomes a row of the list that says why and points at [run-2-result.json](run-2-result.json), and a test that fails on its own binary with no such row is no green. One of the eleven reaches the binary — `set_kind_vocabulary::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical`, by run 1's calibration — and is still what the archive costs this check.

## What this run did not do

- **It is one run.** Nothing was re-driven on either binary; the tool's one retry is what stood between a flake and a difference, and no test needed it.
- **Nothing was fixed and no row was added.** The list here is the list the run read, and it is run 1's.
- **The reach of the eleven tests was not measured again**: which of them reaches the binary is run 1's calibration, taken as recorded.
- **`dev/hygiene-scan`** was run with this directory's files staged — the denylist over the tracked tree, and over every commit not yet on a remote — and the gate's own scan, gitleaks included, ran on the same tree. No hit.
