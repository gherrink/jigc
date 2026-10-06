# Regression spike: the previous release's test binaries against the candidate's binary

Run 2026-10-06 on macOS (aarch64, 10 cores), rustc 1.95.0, cargo-nextest 0.9.143.
`<W>` is the scratch root (`<scratch>/regr-spike.7OpaVI`); `<repo>` is the main checkout.
Nothing in `<repo>`'s working tree was touched; no commit was made.

- **Previous release:** `jigc 1.0.0-rc.24`, tag `jigc-v1.0.0-rc.24` → `91834b5e`.
- **Candidate:** `fix/rc24-tier1` at `b913e602` (product code last changed at `0f34d8f0`).
  78 commits since the tag, 51 of them touching `crates/cli/src` or `crates/engine/src`.
  `crates/cli/packs` and `crates/cli/adapters` are byte-identical between the two.

## Verdict

**The method works**, with one trap that produces a false green if it is not handled
(step 5), and it is cheap: about 20 minutes end to end, of which the two suite runs are 18.
It found 21 old tests that pass on rc.24's binary and fail on the candidate's; 19 are
behaviours the fix pass changed on purpose and recorded, 2 are an artefact of the swap,
none is an unrecorded change. **It is a lower bound, not a proof**: about half the suite
cannot see the binary at all, and it sees only states the old suite happened to pin
(11 of the pass's 51 product commits surfaced).

## Numbers

| | tests run | passed | failed | skipped | wall |
|---|---|---|---|---|---|
| old test build (cold, `cargo nextest run --no-run`) | | | | | 41 s |
| candidate binary build (cold, `cargo build -p jigc`) | | | | | 26 s |
| baseline: old tests, old binary | 4393 | 4392 | 1 | 2 | 511 s |
| swap: old tests, candidate binary | 4393 | 4372 | 21 | 2 | 589 s |
| calibration: old tests, a stub that exits 97 | 4393 | 2230 | 2163 | 2 | 69 s |
| re-check of the 22 differing tests, per binary | 22 | | | | 5 s each |

- Old binary sha256: `a906cd49003ebedd95f0cb96f3fd06d591b7561150e01a042909b0da2fc9835f`
- Candidate binary sha256: `7fdc1355e52a5be2e5a489c4535cac0239d3eeda08d91f8c1fca9c6afeb737da`
- Both print `jigc 1.0.0-rc.24` (no version bump has happened; see *Cannot see*, 6).
- The file at the baked path hashed to the candidate's sha256 both before and after the
  swap run (`<W>/logs/sha-before-spike-swap.txt`, `sha-after-spike-swap.txt`).
- 15 test binaries: `jigc-engine` (1009 tests), `jigc` lib (543), `jigc::bin/jigc` (96),
  and twelve group targets (2745).
- Disk: old target dir 2.0 GB, candidate's 637 MB.

**Pass → fail: 21. Fail → pass: 1. Fail → fail: 0.** The 21 fail again on a second run
against the candidate and all 22 pass against the restored old binary, so the set is
deterministic.

## Step 2: how the tests find the binary

- 533 occurrences in 347 suite files plus 3 support modules, **every one**
  `env!("CARGO_BIN_EXE_jigc")`: compile time. No `option_env!`, no `cargo run`, no `which`,
  no run-time `target/debug/jigc` path, no `assert_cmd`.
- The baked string is `<W>/target-old/debug/jigc`, the uplifted path, never
  `deps/jigc-<hash>` (checked with `strings` on the group binaries: 35 to 45 hits each for
  the first, 0 for the second; 0 for both in the three unit-test binaries).
- Three suites copy the binary out of the baked path before running it
  (`setup.rs`, `uninstall.rs`, `cargo_install_probe.rs`): the swap reaches them.
- `package_contents.rs` runs `cargo package` over the old **source**: the swap cannot
  reach it. `probe_self_image.rs` is `cfg(target_os = "linux")`: not compiled here.
- 238 suite files set `JIGC_PACK_DIR` to the old tree's on-disk pack
  (`CARGO_MANIFEST_DIR`, also compile time), so in those the candidate binary reads the
  **old** pack sources. Harmless today (the packs did not change); see *Cannot see*, 3.
- On this filesystem `target/debug/jigc` and `deps/jigc-<hash>` are two inodes, not a
  hard link. That is what makes step 5's trap bite.

## The method as run, command by command

Environment for every cargo command below (as `dev/gate` does on macOS):

```sh
PATH="$(dirname "$(xcrun --find git)"):$PATH"; export PATH
SDKROOT="${SDKROOT:-$(xcrun --show-sdk-path)}"; export SDKROOT
```

1. Scratch root and worktrees.

```sh
W=$(mktemp -d "<scratch>/regr-spike.XXXXXX")
git -C <repo> worktree add --detach "$W/old" 91834b5e011de2c36e2be2b79e96c0b9f60a803c
git -C <repo> worktree add --detach "$W/new" "$(git -C <repo> rev-parse fix/rc24-tier1)"
mkdir -p "$W/target-old" "$W/target-new" "$W/logs"
```

2. A nextest tool config, so the store and the JUnit reports land under `<W>` (nextest
   keeps its store at `<workspace>/target/nextest` whatever `CARGO_TARGET_DIR` says).
   Each profile inherits the old tree's `[profile.default]`.

```toml
# <W>/spike-nextest.toml
[store]
dir = "<W>/nextest-store"
[profile.spike-base.junit]
path = "junit.xml"
[profile.spike-swap.junit]
path = "junit.xml"
[profile.spike-swap2.junit]
path = "junit.xml"
```

3. Build the old tests, **and record the build before anything is swapped**.

```sh
cd "$W/old"
CARGO_TARGET_DIR="$W/target-old" cargo nextest run --workspace --no-run
CARGO_TARGET_DIR="$W/target-old" cargo nextest list --workspace \
    --list-type binaries-only --message-format json > "$W/old-binaries-metadata.json"
cargo metadata --format-version=1 --all-features > "$W/old-cargo-metadata.json"
```

4. Build the candidate's binary (same profile, debug).

```sh
cd "$W/new" && CARGO_TARGET_DIR="$W/target-new" cargo build -p jigc
shasum -a 256 "$W/target-old/debug/jigc" "$W/target-new/debug/jigc"
```

5. Baseline.

```sh
cd "$W/old"
CARGO_TARGET_DIR="$W/target-old" cargo nextest run --workspace --no-fail-fast \
    --tool-config-file "spike:$W/spike-nextest.toml" --profile spike-base \
    > "$W/logs/baseline.log" 2>&1; echo "rc=$?"
cp "$W/nextest-store/spike-base/junit.xml" "$W/logs/baseline-junit.xml"
```

6. Swap. A new inode by temp sibling and rename, never `cp` over the file in place
   (overwriting a signed Mach-O in place is the known way to get the next exec killed).

```sh
cp -p "$W/target-old/debug/jigc" "$W/old-jigc.bak"
cp "$W/target-new/debug/jigc" "$W/target-old/debug/jigc.swap-tmp" \
    && mv -f "$W/target-old/debug/jigc.swap-tmp" "$W/target-old/debug/jigc"
shasum -a 256 "$W/target-old/debug/jigc"        # must be the candidate's
```

7. Re-run the same test binaries **without cargo** (see step 5 below for why).
   `--binaries-metadata` cannot be combined with `--workspace` or any other build flag,
   and `CARGO_TARGET_DIR` is not needed: the metadata carries the absolute paths.

```sh
cd "$W/old"
cargo nextest run --no-fail-fast \
    --binaries-metadata "$W/old-binaries-metadata.json" \
    --cargo-metadata "$W/old-cargo-metadata.json" \
    --tool-config-file "spike:$W/spike-nextest.toml" --profile spike-swap \
    > "$W/logs/spike-swap.log" 2>&1; echo "rc=$?"
cp "$W/nextest-store/spike-swap/junit.xml" "$W/logs/spike-swap-junit.xml"
shasum -a 256 "$W/target-old/debug/jigc"        # must still be the candidate's
```

8. Compare per test from the two JUnit files (`<W>/junit_outcomes.py` turns one into
   `binary<TAB>test<TAB>outcome<TAB>seconds` rows; the two test sets were identical,
   4393 keys each).

9. Confirm the differing set on both binaries (5 s each): the same reuse-build
   invocation with `-E 'test(=a) | test(=b) | …'`, once with the candidate at the path
   and once with `old-jigc.bak` moved back.

10. Calibration (optional, 69 s): put `#!/bin/sh\nexit 97` at the baked path and run the
    suite once more. A test that still passes does not depend on the binary.

11. Clean up: `find "$W/old/target" -mindepth 1 -delete` (nextest leaves a
    `CACHEDIR.TAG` there), `git -C <repo> worktree remove "$W/old"`, the same for
    `new`, `find "$W/target-old" -mindepth 1 -delete`, the same for `target-new`.

The three scripts as run are `<W>/phase1.sh` (steps 3 to 5), `<W>/phase2.sh` (6 and 7),
`<W>/phase3.sh` (10). Phase timestamps: `<W>/logs/phases.txt`.

## Step 5: what it took

**A plain `cargo nextest run` after the swap silently puts the old binary back.**
Measured: after the swap the file hashed to the candidate's sha256; then
`cargo nextest run --workspace --no-run` printed only `Finished … in 0.07s`, with no
`Compiling` line, and the file hashed to the **old** sha256 again. Cargo considers the
unit fresh and re-uplifts `deps/jigc-<hash>` over `target/debug/jigc`. A script that
swaps and then runs the usual command would run the baseline twice and report zero
regressions.

What works: nextest's reuse-build path, `--binaries-metadata` plus `--cargo-metadata`
recorded before the swap. It invokes no cargo at all, so nothing rebuilds or re-links.
No archive and no mtime trick was needed.

Proof the candidate is what ran: its sha256 at the baked path before and after the run;
21 tests whose outcome differs, for example
`setup::setup_writes_compose_embedded_methodology_marker`, which fails with the
candidate's own `setup.dirty-install-path` refusal, a code path rc.24 does not take
there; and the stub run, where 2163 tests fail at the same path.

A script should assert the hash at the baked path immediately before and after the
swapped run and fail closed if either is not the candidate's.

## Step 6: every test that passed on the baseline and failed after the swap

Classes: **(a)** changed on purpose and recorded, **(b)** changed and unrecorded,
**(c)** not a behaviour difference, **(d)** could not tell.
**Count: (a) 19, (b) 0, (c) 2, (d) 0.**

"At tip" is how `fix/rc24-tier1` treats the same test. All 19 class (a) tests live in a
file the pass modified; the two class (c) tests live in the one file it did not touch.

| # | Suite :: test | What the old test asserts | What the candidate did | Class, commit, record | At tip |
|---|---|---|---|---|---|
| 1 | `flow19_planning_encode` :: `flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize` | A singleton hand-edited before the task's first write, under a held baseline, blocks `task finalize` non-zero | Exit 0, advisory `reconciliation.absorb`, `finalized … promoted docs/roadmap.md` | (a) `0f34d8f0`. *The copied-in witness*, consequence 1: an edit made before the first write is carried and lands | Body changed |
| 2 | `singleton_running_doc` :: `warm_edit_over_an_oob_drifted_singleton_conflict_blocks_at_finalize` | Same shape on `docs/runlog/runlog.md` | Exit 0, `reconciliation.absorb`, promoted | (a) `0f34d8f0`, same consequence | Body changed |
| 3 | `reconciliation_baseline_contrast` :: `a_lost_file_state_baseline_turns_a_conflict_block_into_a_silent_merge` | Arm A (baseline held, edit before first touch) exits 3 with `reconciliation.conflict-block`; arm B (baseline lost) merges silently at exit 0 | Arm A exits 0; findings `file-state.staged-copy`, `reconciliation.absorb`, `changelog-recording.gate-granted-unused` | (a) `0f34d8f0` (renamed first by `eb18e5fe`). The old test pins the rc.24 defect itself: one sequence, two outcomes, decided by a cache | Deleted; replaced by `the_order_of_the_edit_decides_between_a_block_and_a_merge_not_the_baseline` |
| 4 | `flow49_acceptance` :: `a_lost_baseline_switches_off_the_never_silently_merged_guarantee` | Same two-arm contrast | Arm A exits 0 with the same three findings | (a) `0f34d8f0` (`eb18e5fe`) | Deleted |
| 5 | `pre_guard_repair_route` :: `the_migration_source_route_does_not_promise_a_review_it_does_not_get` | A migration whose source was edited after the mint is routed at the migration-source exit (`jigc unmanage <source>`) | Route is `jigc task discard <id> --force` then `jigc migrate … --as spec`, or undo the edit; "Nothing replaces or removes the file while it differs from the recorded source" | (a) `4ba04efc`. Round 4, Area D, *the migration-source correction*; *newly refuses*: a migration whose source was edited after the mint (0 or 4 → 3) | Deleted (file +376 −78) |
| 6 | `flow48_acceptance` :: `the_guide_artifact_is_installed_replaced_and_never_clobbered` | A guide whose `jigc-version:` line alone was moved to `0.0.1 #…` is still jigc's: `setup` replaces it byte for byte | `setup` left the copy as it was | (a) `33620081`. Round 4, Area E: the guide's ownership oracle answers for the whole file. **Thinly recorded**: see note 1 | Fixture changed to `support::older_guide::as_an_older_build_wrote_it` |
| 7 | `adapter_artifact` :: `a_pristine_stale_stamped_guide_is_replaced_and_restamped` | A hand-assembled stale guide (`jigc-version: 0.0.1-old`, any body, a true body hash) is replaced and re-stamped at `1.0.0-rc.24` | `setup` exits clean and leaves it: the stamp still reads `0.0.1-old` | (a) `33620081`, note 1 | Fixture changed |
| 8 | `setup_install_pathspec_guard` :: `an_owned_guide_artifact_is_not_a_subject_of_the_guard` | A tracked guide with only the stamp line moved is jigc's own, so the install replaces it | Not replaced | (a) `33620081`, note 1 | Fixture changed |
| 9 | `flow52_acceptance` :: `a_refused_transaction_restores_its_config_layer_writes_without_overwriting_a_concurrent_edit` | With `.jigc/version` holding `0.0.0-flow52-stale-stamp` and a hook rejecting the commit, **two** pre-images are parked under `.jigc/displaced/` | One pre-image (`.jigc/.gitignore`); the stamp was never rewritten, so it has none | (a) `104a7d4b`. Round 3: at `task finalize` the stamp is refreshed only where absent or a regular file holding the one `jigc-version:` line; anything else is left unwritten and unstaged | Body same; fixture constant changed to jigc's stamp shape |
| 10 | `config_layer_preimage` :: `a_path_jigc_never_wrote_is_neither_restored_nor_reported` | Exactly one rollback conflict, the stamp's | Zero conflicts | (a) `104a7d4b`, same | Body same; `STALE_STAMP` changed |
| 11 | `config_layer_preimage` :: `every_config_layer_pathspec_carries_a_disposition` | The stage's config-layer pathspecs are exactly the dispositioned rows, `.jigc/version` among them | `git add -- :(literal).jigc/config :(literal).jigc/.gitignore`: the stamp is not staged | (a) `104a7d4b`, same | Body same; `STALE_STAMP` changed |
| 12 | `config_layer_preimage` :: `the_manufactured_shape_space_holds_over_both_files` | Cell Stage/ConcurrentEdit parks both pre-images | One parked | (a) `104a7d4b`, same | Body same; `STALE_STAMP` changed |
| 13 | `setup` :: `setup_writes_compose_embedded_methodology_marker` | `jigc setup` over a pre-seeded, untracked `.jigc/config/packs.yaml` in a repository with no commit exits 0 | Blocking `setup.dirty-install-path` naming `.jigc/config/packs.yaml`; nothing installed | (a) `dc0d7586`. *Now refuses*: `setup` on an unborn `HEAD` over an untracked file at a replaced member, `packs.yaml` among them; the human's ruling 2 of 2026-10-04 keeps it | Body changed |
| 14 | `version_stamp_rollback` :: `a_hook_rejected_finalize_restores_a_staged_blob_at_a_promotion_destination` | `task finalize` reaches the commit phase (the hook is the rejector) with a blob staged in the index at the promotion destination | Blocked earlier: `finalize.promote-clobber`, "missing from the worktree but not from git … minted as a new doc" | (a) `78e8ded1`. Round 4, Area C: an identity git holds is occupied; *newly refuses*: a `created` doc over a home git holds (`task finalize` 0 → 3) | Body changed |
| 15 | `cwd_verb_subject` :: `uninstall_removes_the_workbench_home_install_from_every_cwd` | The ack says it **pruned** the fan-out worktrees | The ack says "dropped git's registrations of the fan-out worktrees `.jigc/` held" | (a) `dcfa40f0` (with `8c159622`). Round 4, Area E: the ack says *dropped*, not *pruned*. Wording only | Body changed |
| 16 | `leftover_probe_fail_closed` :: `a_teardown_that_removed_nothing_narrates_nothing` | An `uninstall` that fails on an unwritable `.jigc/` claims no removal at all on stderr | Prints "warning: removing `.jigc/` also removes jigc's own bookkeeping for 2 open work unit(s)" and then blocks under `uninstall.remove-jigc` | (a) `1b1d7656`. Round 4, Area E: the teardown names each work unit whose bookkeeping it takes. The commit message names this very test as going red and being right to | Body changed: held per path, not per run |
| 17 | `milestone` :: `milestone_finalize_warns_on_a_leaked_worktree_but_still_succeeds` | The teardown warning names the `git worktree prune` remedy | The remedy is `git worktree unlock <path>` then `git worktree remove --force <path>`, "it removes this one registration and no other" | (a) `8c159622`. `L-22`: no repository-wide prune remains, and the one printed remedy that sent the reader to it is converted. No exit code moved | Body changed |
| 18 | `milestone_landed_attribution` :: `a_path_landed_by_two_chain_commits_is_one_manifest_entry_owned_by_the_last` | `milestone finalize` exits 0 where a sub-task's doc promotes to a path a sub-task worktree also staged | Exit 3, `finalize.promote-clobber` keyed at `docs/decisions/low-policy.md` | (a) `048724d0`. Round 4, Area C; *newly refuses*: a worktree-staged destination at the boundary (0 → 3) | Deleted |
| 19 | `milestone_record_stale_base` :: `stale_base_routes_finalize_without_committing` | In a fresh-clone simulation with the record's base rewritten on disk and **uncommitted**, `milestone finalize` blocks with the routed stale-base finding | Blocks with `reconciliation.conflict-block` on the record: "differs from what `HEAD` holds" | (a) `8d9c3afb`. Round 3: a record door with no baseline compares the record against `HEAD`. A refusal either way; the code and the route changed | Body same; the fixture now commits the rewritten record |
| 20 | `clap_error_kind_axis` :: `every_producible_kind_is_reached_by_driving_its_probe` | For each clap error kind disposed *clap stands*, the binary's bytes equal `err.render()` computed **in the test process** from the linked `cli` library | `jigc --help` differs from the old library's render: the `setup` and `uninstall` about-lines were reworded | (c). The oracle is rc.24's own `cli` crate, linked into the test binary; the swap splits the two halves of one parity check. The candidate's binary equals its own render (the test is green and unmodified at tip) | Unchanged |
| 21 | `clap_error_kind_axis` :: `every_producible_kind_holds_with_the_invocation_log_on` | The same parity with the invocation log on | Same | (c), same | Unchanged |

**Note 1, the one place class (a) rests on a thin record.** `DECISIONS.md` carries
`33620081` as one sentence and it is absent from *What newly refuses against rc.24*.
The bound the three tests hit is stated in the commit message, the round-4 fixer report,
the ledger and `design/assistant-adapter.md`: a copy whose front matter or opening
sentence differs from what the running build would assemble reads as the adopter's and
is left alone and reported, where rc.24 replaced any copy whose body hash was true. The
commit states that an install written by the published rc.24 binary is still replaced on
upgrade, driven with that binary; this spike did not re-drive that.

**Direction.** The clause is "no command that works on the previous release stops
working". By that reading:

- **Newly refuses where rc.24 went on** (4 tests, 4 commits): 13 (exit 0 → refusal),
  18 (0 → 3), 14 (blocked before the commit phase it used to reach), 5 (0 or 4 → 3 by
  the record; the old test pins the route, not the exit). These are the clause's subject.
- **Newly succeeds where rc.24 refused** (4 tests, 1 commit): 1 to 4.
- **Same exit class, different action or text** (11 tests): 6 to 12 (a file no longer
  rewritten), 15 to 17 (wording, remedy), 19 (refusal code).

## Step 7: the reverse direction

One test: `format_json_success_axis::task_discard_names_the_commit_it_landed` failed on
the baseline and passed after the swap. It is a flake, not a behaviour: the panic is in
the fixture builder (`support/trial_corpus.rs:1074`, `copy_tree`: `read
…/repo/.git/objects/1b: No such file or directory`), a directory that vanished between
`read_dir` and the read while a corpus was being copied under a 16-thread pool. It
passed 3 of 3 alone on the old binary and again in the old-binary re-check. So: one
flake in 4393 on a loaded machine, in the baseline of all places. The re-check of the
differing set on both binaries (step 9, 10 s) is what separates a flake from a finding,
and a script needs it.

## What does not work, and what the method cannot see

1. **Half the suite cannot see the swap.** With a stub at the baked path, 2230 of 4393
   tests still pass: all 1648 unit tests (`jigc-engine` 1009, `jigc` lib 543, bin 96)
   and 582 tests inside the group targets (source and doc fences, registries, tests
   that call `cli::` in process). The swap reaches **2163** tests. Any behaviour the old
   suite pins only in process is invisible, and so is everything the engine's own unit
   tests pin. (Useful side effect: the swapped run could be filtered to the 2163.)
2. **It sees only states the old suite built.** The pass's record lists many more new
   refusals than the 4 found here, and none of these surfaced because rc.24's suite has
   no test in that state: `uninstall` over the invocation log or foreign bytes
   (`12398ddc`), the linked-worktree guard (`e3a6ba58`), the symlink refusals
   (`104a7d4b`, `9465f9b6`, `5f5b273a`), the own-registration guard (`ebfc79fb`),
   `milestone.unlanded-work` (`e842342e`), `status.showUntrackedFiles=no` (`894f2234`),
   the refusal-code change of `c0c4d88c`. 11 of 51 product commits surfaced. A green
   result means "nothing the old suite pinned through the binary moved", never
   "nothing that worked stopped working".
3. **The pack under test is the old one in 238 suite files.** They set `JIGC_PACK_DIR`
   to the old tree's on-disk pack, so a candidate that changed a workflow or a schema
   would be run against rc.24's pack there, masking the change, and a candidate that
   needs a new pack key would fail for a reason that is not a regression. Not exercised
   by this spike: the packs are identical.
4. **Parity tests whose oracle is the old library** fail on any change to the compared
   surface (class (c) above). They are false positives for the clause but true signals
   of a changed surface; here, two reworded `--help` paragraphs.
5. **Old tests that pin a defect, or a fixture that is unfaithful**, fail when the defect
   is fixed (rows 3, 4, 6 to 12). The method cannot tell a fix from a regression; a
   person or the record has to. The mechanical signal that held for all 21: a failing
   old test whose file the candidate's tree **did not** modify is the suspicious one.
6. **The version string.** Both binaries print `1.0.0-rc.24`. 12 old suite files read
   `env!("CARGO_PKG_VERSION")` (their own, compile time) and 89 binary-reaching tests
   live in them; row 7 compares it with the stamp the binary wrote. Once the candidate
   is bumped, some of those will fail as class (c). Run the method before the bump, or
   expect that set.
7. **Cargo undoes the swap** unless the run goes through the reuse-build path (step 5).
8. **One platform, one profile.** macOS only; the Linux-only suite did not compile; 2
   tests are `#[ignore]`. The candidate was a debug build, where debug-only panics
   exist. Nothing ties the method to that: the swap is one file, so a release build or
   the published binary can sit at the path instead.
9. **Text assertions are as tolerant as their authors made them.** A `contains` check
   passes over a reworded message; a byte-for-byte golden fails on it. Coverage of
   wording changes is uneven and unmeasured.
10. **`package_contents`** packages the old source and cannot see the candidate.

## State left behind

- Both worktrees removed with `git worktree remove` (no `--force`, no refusal);
  `git -C <repo> worktree list` shows the main tree only.
- `<W>/target-old` and `<W>/target-new` emptied. The backup of the old binary deleted.
- Kept under `<W>`: `logs/` (the three run logs, the three JUnit files, the per-test
  outcome tables, `pass-to-fail.txt`, one file per failure under `logs/fail/` with the
  old and tip bodies of each test, `phases.txt`, the hash files), the three phase
  scripts, `junit_outcomes.py`, `spike-nextest.toml`, the two metadata files, and the
  slice of `DECISIONS.md` read for the classification.
