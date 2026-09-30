# CI runtime — M54 Increment 8

This file records the **before run**, read back by its id. S8's bound is measured against it, and T3 adds the per-leg proxy here later. Cross-ref [roadmap.md](../../../implementation/roadmap.md) → Milestone 54 → Increment 8; [planning-gate-record.md](planning-gate-record.md) → row 17; [DECISIONS.md](../../../DECISIONS.md) → *M54 Inc 2 T1* (the run id) and *M54 Increment 8 planning*.

## The before run — `36549870099`

This is the single-job workflow as it stood before S8. It ran on the push of `d7d3e1ce`, Increment 1's tip, before Increment 2 put tree-sitter into `jigc`. Read on 2026-09-30.

### The commands

Every value below is the output of one of these commands, cited by its id. Each was run for this record, with `gh` authenticated against `gherrink/jigc`. The log is saved to a scratch file first and read from there, never through a pipe whose exit status is read.

```sh
L=$(mktemp -d)/run.log
# C0: the log. 4,742 lines; two fetches were byte-identical (cmp → 0).
gh run view 36549870099 --log > "$L"
# C1: the run
gh run view 36549870099 --json databaseId,workflowName,headBranch,headSha,event,status,conclusion,attempt --jq '"\(.databaseId) \(.workflowName) \(.headBranch) \(.headSha) \(.event) \(.status) \(.conclusion) attempt=\(.attempt)"'
# C2: the job and its runner
gh api repos/gherrink/jigc/actions/jobs/109345136499 --jq '"\(.id) \(.name) labels=\(.labels|join(",")) group=\(.runner_group_name) runner=\(.runner_name) \(.status) \(.conclusion)"'
# C3: the job wall clock
gh run view 36549870099 --json jobs --jq '.jobs[] | "\(.databaseId) \(.startedAt) \(.completedAt) \((.completedAt|fromdate) - (.startedAt|fromdate))s"'
# C4: the run wall clock
gh api repos/gherrink/jigc/actions/runs/36549870099 --jq '"\(.run_started_at) \(.updated_at) run_attempt=\(.run_attempt)"'
# C5: every step
gh run view 36549870099 --json jobs --jq '.jobs[0].steps[] | "\(.number)\t\(.name)\t\(.startedAt)\t\(.completedAt)\t\((.completedAt|fromdate) - (.startedAt|fromdate))s\t\(.conclusion)"'
# C6: the runner image
command grep -A2 'Runner Image$' "$L"
# C7: nproc: the line after the `Run nproc` group closes
awk '/##\[group\]Run nproc/{f=1} f&&/##\[endgroup\]/{getline; print; exit}' "$L"
# C8: the per-binary test results
command grep -E '(     Running |   Doc-tests |test result: )' "$L"
# C9: the cache and compile state
command grep -E 'No cache found|Finished `|Failed to save' "$L"
# C10: the fresh fetches
command grep -c 'Downloaded ' "$L"
command grep -E 'auto-installed|Updating crates.io index' "$L"
```

`gh run view --log` labels every line of this run `UNKNOWN STEP`. So a log value is tied to its step by the `##[group]Run <command>` marker, and by its timestamp falling inside that step's window in C5. GitHub keeps run logs for 90 days by default, so C0 works until about 2026-12-28. The lines C6–C10 print are quoted below so this record outlives the log.

### The entry gate

| Check | Required | Read | Evidence |
|---|---|---|---|
| Run concluded | `completed` / `success` | `completed` / `success` | C1 → `36549870099 CI main d7d3e1cea9de32efa7eae971b6ae5a1bb9bfedc4 push completed success attempt=1` |
| Standard public runner | `ubuntu-latest` on `GitHub Actions` | `ubuntu-latest` on `GitHub Actions` | C2 → `109345136499 dev-workflow gate labels=ubuntu-latest group=GitHub Actions runner=GitHub Actions 1000000396 completed success` |
| CPUs | `nproc` = 4 | **4** | C7 → `… 2026-09-29T09:33:06.4464128Z 4`. This is step 6, `CPU count`, which ran at 09:33:06Z and whose `run:` is `nproc` (`d7d3e1ce:.github/workflows/ci.yml:96-97`) |

**No halt condition holds, so the gate passes.**

### Identity and runner

| Value | Read | From |
|---|---|---|
| Run id · workflow · branch | `36549870099` · `CI` · `main` | C1 |
| Head sha | `d7d3e1cea9de32efa7eae971b6ae5a1bb9bfedc4` | C1 |
| Event · attempt | `push` · `1` (not a re-run) | C1 |
| Job | `109345136499` `dev-workflow gate`, the run's only job | C2; C3 prints one line |
| Labels · group · runner | `ubuntu-latest` · `GitHub Actions` · `GitHub Actions 1000000396` | C2 |
| Image | `ubuntu-24.04`, version `20260920.314.1` | C6 → `##[group]Runner Image` / `Image: ubuntu-24.04` / `Version: 20260920.314.1` |
| Toolchain | `1.95.0-x86_64-unknown-linux-gnu`, installed by the `Show toolchain` step | C10 → ``warn: the missing active toolchain `1.95.0-x86_64-unknown-linux-gnu` has been auto-installed`` |

### Wall clock

| Value | Read | From |
|---|---|---|
| **Job wall clock** | **2020 s = 33 min 40 s** (09:32:54Z → 10:06:34Z) | C3 → `109345136499 2026-09-29T09:32:54Z 2026-09-29T10:06:34Z 2020s` |
| Run, created → updated | 09:32:51Z → 10:06:35Z | C4 → `2026-09-29T09:32:51Z 2026-09-29T10:06:35Z run_attempt=1` |

### Every step

C5 prints this table:

| # | Step | Start | End | Duration | Conclusion |
|---|---|---|---|---|---|
| 1 | Set up job | 09:32:55Z | 09:32:56Z | 1s | success |
| 2 | Checkout | 09:32:56Z | 09:33:05Z | 9s | success |
| 3 | hygiene range | 09:33:05Z | 09:33:05Z | 0s | success |
| 4 | gitleaks | 09:33:05Z | 09:33:06Z | 1s | success |
| 5 | denylist | 09:33:06Z | 09:33:06Z | 0s | success |
| 6 | CPU count | 09:33:06Z | 09:33:06Z | 0s | success |
| 7 | Show toolchain | 09:33:06Z | 09:33:15Z | 9s | success |
| 8 | Cache cargo dependencies | 09:33:15Z | 09:33:15Z | 0s | success |
| 9 | probe prebuild (doc-code) | 09:33:15Z | 09:33:28Z | 13s | success |
| 10 | cargo fmt --check | 09:33:28Z | 09:33:31Z | 3s | success |
| 11 | cargo clippy --all-targets -- -D warnings | 09:33:31Z | 09:34:07Z | 36s | success |
| 12 | cargo build | 09:34:07Z | 09:34:23Z | 16s | success |
| 13 | cargo test | 09:34:23Z | 10:06:30Z | **1927s = 32 min 7 s** | success |
| 14 | manifest-freeze fence | 10:06:30Z | 10:06:30Z | 0s | success |
| 27 | Post Cache cargo dependencies | 10:06:30Z | 10:06:33Z | 3s | success |
| 28 | Post Checkout | 10:06:33Z | 10:06:33Z | 0s | success |
| 29 | Complete job | 10:06:33Z | 10:06:33Z | 0s | success |

The timestamps are whole seconds, so each duration is ±1 s. The steps sum to 2018 s, against the job's 2020 s.

**Non-cargo step time**, the figure T3's per-leg budget subtracts from 15 min:

- Steps 1–8 and 27–29 sum to **23 s**.
- The steps a cargo-only job would repeat sum to **22 s**. Those are steps 1, 2, 6, 7, 8, 27, 28 and 29, which leaves out the three hygiene steps.

### Per-test-binary run times (the `cargo test` step)

C8 prints each binary's header and its `test result:` line. The `finished in` value is libtest's own time to run that binary's tests. It does not include compilation.

| Binary (as named at `d7d3e1ce`) | Tests | `finished in` |
|---|---|---|
| `tests/groups/g_flow.rs` | 331 passed | 380.19s |
| `tests/groups/g_doc.rs` | 257 passed | 275.43s |
| `tests/groups/g_milestone.rs` | 303 passed | 275.08s |
| `tests/groups/g_finalize.rs` | 369 passed, 1 ignored | 182.96s |
| `tests/groups/g_migrate.rs` | 392 passed, 1 ignored | 169.56s |
| `tests/groups/g_compose.rs` | 203 passed | 168.51s |
| `tests/groups/g_config.rs` | 208 passed | 151.80s |
| `tests/groups/g_methodology.rs` | 138 passed | 97.51s |
| `tests/groups/g_item.rs` | 173 passed | 85.67s |
| `tests/groups/g_solo_trial_corpus.rs` | 14 passed | 44.46s |
| `tests/groups/g_migration.rs` | 88 passed | 42.12s |
| `engine` unittests `src/lib.rs` | 982 passed | 4.39s |
| `tests/groups/g_solo_store_sweep.rs` | 3 passed | 2.33s |
| `cli` unittests `src/lib.rs` | 534 passed | 1.28s |
| `jigc` unittests `src/main.rs` | 0 passed | 0.00s |
| Doc-tests `cli` | 0 passed | 0.00s |
| Doc-tests `engine` | 0 passed | 0.00s |

These sum to **1881.29 s**. C8 prints one more pair: `g_finalize` again, with `1 passed; … 369 filtered out; finished in 0.01s`. That pair is step 14, the manifest-freeze fence's single `--exact` test at 10:06:30Z, and it is not part of `cargo test`. The packages are named `cli` and `engine` because the run predates the `jigc`/`jigc-engine` rename.

### Cache and compile state: the run was cold

**The planning note that this run was *warm* does not hold.** The Actions cache missed, and the registry and the toolchain were fetched fresh. So every compile in the job started from nothing except the job's own earlier steps. C9 prints these lines (shown here after their timestamps):

| Time | Line | Step |
|---|---|---|
| 09:33:15.93Z | `No cache found.` | 8, Cache cargo dependencies (restore key `v0-rust-gate-Linux-x64-0f490d39`) |
| 09:33:28.19Z | ``Finished `dev` profile … in 12.20s`` | 9, probe prebuild (doc-code, a separate target dir) |
| 09:34:07.55Z | ``Finished `dev` profile … in 35.71s`` | 11, clippy |
| 09:34:23.28Z | ``Finished `dev` profile … in 15.65s`` | 12, build |
| 09:35:07.86Z | ``Finished `test` profile … in 44.52s`` | 13, cargo test |
| 10:06:30.38Z | ``Finished `test` profile … in 0.05s`` | 14, manifest-freeze fence |
| 10:06:33.34Z | `Failed to save: Unable to reserve cache with key v0-rust-gate-Linux-x64-0f490d39-02ccd49b, another job may be creating this cache.` | 27, Post Cache |

C10 counts `93` crates downloaded fresh, and it prints two `Updating crates.io index` lines.

The `test` profile's 44.52 s was warm only **within the job**. It reused the dependency artifacts that step 12 (`cargo build`) had just compiled into the same `target/`. No cache ever warmed it.

So the run's cargo compile time is a **cold-cache** figure: the four compiles sum to 12.20 + 35.71 + 15.65 + 44.52 = **108.08 s**. The 1927 s `cargo test` step splits into 44.52 s of compile and about 1882 s of test execution, which matches the 1881.29 s the binaries sum to. The before run is therefore comparable to T3's cold proxy, which the plan already treats as *the bound a cache-miss push must meet too*.
