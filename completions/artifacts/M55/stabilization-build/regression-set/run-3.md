# The regression set's first part — run 3: green on the list the candidate holds

**Run 2026-10-07 on macOS (aarch64, 10 cores) — rustc 1.95.0, cargo-nextest 0.9.143, git 2.54.0 — by [`dev/regression-set`](../../../../../dev/regression-set) as commit `b36b8c93` holds it.** `<repo>` is the checkout the run was made from, `<scratch>` the scratch root of the run; nothing else of the host is named here. The check, what it establishes and what it cannot see: [implementation/stabilization-workflow.md](../../../../../implementation/stabilization-workflow.md) → The regression set. What the tool holds since run 1: `DECISIONS.md` → *2026-10-07 — The regression tool's green rests on facts of the two commits*. This record restates neither. The index of the runs: [README.md](README.md).

- **Previous release:** `jigc 1.0.0-rc.24` — tag `jigc-v1.0.0-rc.24`, which peels to `91834b5e011de2c36e2be2b79e96c0b9f60a803c`.
- **Candidate:** `fix/rc24-regression` at `b36b8c93c714bb07531af8e82e9c0268329ba0d2` — the commit that holds the rows, and the tool that reads a test out of the comparison only by a row. It changes no product path: **the product is the one merged at `b33bf39b`**, run 2's.
- **The list:** [intended-changes.tsv](intended-changes.tsv) **as the candidate's commit holds it**: 33 rows, sha256 `bfdad1af90dace76d92ee5bebfe807797d92ca894f89f0d4e9aa4f52a5c0c20c` — 22 for a change (16 by a commit, 6 by a ruling) and 11 exclusion rows.

## The verdict

**Green: every test that passes on the previous release's binary and fails on the candidate's is a row of the list, and every test that fails on its own binary is an exclusion row of it.** 22 differences, 22 on the list, **none off it**; 11 tests failed on their own binary, **the 11 exclusion rows and no other**; no test on no row; 0 stale rows; no test needed its retry on either binary.

It is a floor and not the clause's instrument whole: about half of the old suite never reaches the binary, the old tests build only the states they build, a test that is a row is not looked at again, and the second part of the regression set is not built.

## What is green that run 2 was not

- **The one difference run 2 named off the list has its row**: `jigc::g_migrate` :: `setup::setup_commits_the_pre_commit_hook_iff_it_is_a_working_tree_file`, pointing at the human's ruling on the fix pass's item 21 — `DECISIONS.md` → *2026-10-06 — `jigc setup` records the settings entries it adds* — and checked against the fix's commit, `1c7d391c`, in [run-2.md](run-2.md). It fails on the candidate's binary here as it did there.
- **The eleven tests that fail on their own binary are stated rows.** In runs 1 and 2 they dropped out under a tolerance and by no row. Here each is excluded by its row — the line's `excluded` carries the row beside the test — and the tool would have answered void for one that had none. **`excluded` is exactly the eleven exclusion rows**: compared by binary and name against the list after the run; `excluded_by_no_row` is empty, and no exclusion row is stale.

## The command

```sh
cd <repo>
dev/regression-set run \
    --previous 91834b5e011de2c36e2be2b79e96c0b9f60a803c \
    --candidate b36b8c93c714bb07531af8e82e9c0268329ba0d2 \
    --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv \
    --scratch <scratch> > <scratch>/line.json
```

Exit status 0, nothing on stderr. [run-3-result.json](run-3-result.json) is the one line it printed, with the host's scratch root replaced by `<scratch>` and laid out one field per line; nothing else in it is changed. What the tool ran, in order, is the block of run 1's record with this run's two commits, after the list was read out of the candidate's commit and recorded.

## The numbers

| | tests | passed | failed | seconds |
|---|---|---|---|---|
| the previous release unpacked, its tests built and the build recorded | | | | 40 |
| the candidate unpacked and its binary built | | | | 26 |
| baseline: the old tests on the old binary | 4393 | 4382 | 11 | 722 |
| the same test binaries on the candidate's binary | 4393 | 4360 | 33 | 687 |

- **Passed on the baseline and failed on the candidate: 22.** On the list: 22. **Not on the list: 0.**
- **Failed on their own binary: 11. Excluded by a row: 11. By no row: 0** — of the 11, 0 pass on the candidate's.
- **Rows of the list that matched nothing: 0** — of either kind.
- **Passed on a retry:** 0 on the baseline, 0 on the candidate.
- **Wall time: 1,480 s**, end to end, on a machine other agents were using (load average between thirty and seventy-five).
- 15 test binaries, as in runs 1 and 2. The 22 differences and the 11 tests that fail on their own binary are run 2's, test for test.

## Which binary ran

The path the old tests have compiled in, read out of the recorded build: `<scratch>/regression-set.2l1c260t/target-previous/debug/jigc`. **12 test binaries hold that path in their bytes.**

| sha256 of the file at that path | |
|---|---|
| the previous release's binary, as its tests' build left it | `bc7e27abd229e50b77d922c43343607860def08ebd6800cb6e926c8d56b52099` |
| before the baseline | `bc7e27abd229e50b77d922c43343607860def08ebd6800cb6e926c8d56b52099` |
| after the baseline | `bc7e27abd229e50b77d922c43343607860def08ebd6800cb6e926c8d56b52099` |
| the candidate's binary, as its own build left it | `e0373134f7164552087eede4ccd46ec85f7739560b403c9fc4db5283f1b4a629` |
| after the swap, before the second run | `e0373134f7164552087eede4ccd46ec85f7739560b403c9fc4db5283f1b4a629` |
| after the second run | `e0373134f7164552087eede4ccd46ec85f7739560b403c9fc4db5283f1b4a629` |

The first three are one hash and the last three another: the baseline drove the previous release's binary, the second run the candidate's, and nothing put the old one back.

## The 22 differences and the 11 excluded

Each is a row of [the list](intended-changes.tsv), which says what changed or why the test fails on its own binary, and is not repeated here. The 21 of run 1's table are there unchanged; the twenty-second is the one above; the eleven are run 2's table, each row pointing at [run-2-result.json](run-2-result.json) — the line of the run whose baseline report they were read from.

**One of the eleven is coverage this check does not have**, and its row says so: `jigc::g_config` :: `set_kind_vocabulary::no_schema_hash_moved_and_the_pinned_projection_is_byte_identical` reaches the binary and fails on its own only because the previous release is built from an archive. What building it from a clone would take, and that nobody has driven it: `DECISIONS.md`, the entry above, item 11.

## What this run did not do

- **It is one run.** Nothing was re-driven on either binary; the tool's one retry is what stood between a flake and a difference, and no test needed it.
- **Nothing was fixed and no row was added after the run.** The list here is the list the run read out of its candidate's commit.
- **It did not look again at a test that is a row**: 22 old tests fail on the candidate's binary and are taken as the changes their rows name. What a later fix changed inside one of them is not seen.
- **Who may add a row is held by nothing but review**: a row is held to a pointer that resolves, and for an exclusion row to a run's own line.
- **`dev/hygiene-scan`** was run with this directory's files staged — the denylist over the tracked tree, and over every commit not yet on a remote — and the gate's own scan, gitleaks included, ran on the same tree. No hit.
