# CI runtime — M54 Increments 8 and 9

This file records the **before run**, read back by its id. S8's bound is measured against it, and the per-leg proxy (T3) follows it. Increment 9's entry gate adds **the provisional after run** at the end ([planning-gate-record.md](planning-gate-record.md) → row 18). Cross-ref [roadmap.md](../../../implementation/roadmap.md) → Milestone 54 → Increment 8; [planning-gate-record.md](planning-gate-record.md) → row 17; [DECISIONS.md](../../../DECISIONS.md) → *M54 Inc 2 T1* (the run id) and *M54 Increment 8 planning*.

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

## The per-leg proxy (T3)

Each leg T4 pins was run as its own `dev/runner-faithful --cpus 4 --commit 2ce0434e26a9d104b20363a4ceec3ffca7dfa646 cargo <argv>` invocation, one at a time, from 2026-09-30 09:58:45Z to 10:26:00Z. Nothing else ran on the docker host meanwhile. Every run was cold: a fresh container, the target directory inside it, and no cache. So each leg downloaded its crates again (65 to 121 per leg) and compiled from nothing, the way a cache-miss push does.

### The platform, and the bound on what it proves

| Value | Read |
|---|---|
| Commit | `2ce0434e26a9d104b20363a4ceec3ffca7dfa646`, T2's tip (the container's `commit` header line) |
| Host | Apple M1 Max, macOS 26.6.2, colima (Ubuntu 24.04.4 LTS, kernel 6.8.0-117-generic, 8 CPUs) |
| Container | `linux/aarch64`, `rustc 1.95.0 (59807616e 2026-04-14)`, `git version 2.43.0` |
| CPUs | `cpus      4` in every leg's log, which the tool asserts via `--cpuset-cpus 0-3` |

**The proxy is not the runner.** It runs on arm64 here, where the runner is x86_64. It keeps no cache, which matches a cache miss but not a warm push. Its cores are M1 Max cores, not the runner's vCPUs. The before run is the only runner measurement, and it predates tree-sitter in `jigc`. **The platform gap, measured:** for the same group, the before run's libtest `finished in` time is 1.67× (`g_methodology`) to 2.10× (`g_doc`) this proxy's. `g_flow` is 380.19 s there against 184.22 s here, a ratio of 2.06×. That ratio also absorbs Increment 2's removal of the ~39 nested probe builds the before run still ran inside its tests, so it is not a clean platform factor.

### The budget

A leg's budget is **15 min minus the before run's non-cargo step time: 900 − 22 = 878 s.** The 22 s is what a cargo-only job repeats: Set up job, Checkout, CPU count, Show toolchain, the cache restore, Post cache, Post checkout and Complete job (see *Non-cargo step time* above). Counting the hygiene steps too gives 23 s and 877 s. No leg's verdict changes either way.

### The table

*Wall clock* is `date +%s` before and after the invocation, measured outside the tool. It includes the tool's own set-up: the image cache check, the git bundle, `docker create`/`cp` and the clone. *Compile* is cargo's last `Finished` line. *Tests* is libtest's `finished in`. Both are read from the leg's log.

| Leg | Argv (`cargo …`) | Wall clock | Exit | CPUs | Compile · tests |
|---|---|---|---|---|---|
| `fmt` | `fmt --check` | 10 s | 0 | 4 | — |
| `clippy` | `clippy --all-targets -- -D warnings` | 38 s | 0 | 4 | 29.26 s |
| `build` | `build` | 33 s | 0 | 4 | 22.78 s |
| `g_flow` | `test -p jigc --test g_flow --no-fail-fast` | **221 s** | 0 | 4 | 28.97 s · 184.22 s (333 passed) |
| `g_doc` | `test -p jigc --test g_doc --no-fail-fast` | 167 s | **101** ‡ | 4 | 26.45 s · 131.39 s (258 passed, 1 failed) |
| `g_milestone` | `test -p jigc --test g_milestone --no-fail-fast` | 174 s | 0 | 4 | 28.24 s · 137.34 s (303 passed) |
| `g_finalize` | `test -p jigc --test g_finalize --no-fail-fast` | 137 s | 0 | 4 | 27.90 s · 101.35 s (381 passed, 1 ignored) |
| `g_migrate` | `test -p jigc --test g_migrate --no-fail-fast` | 121 s | 0 | 4 | 27.97 s · 83.50 s (413 passed, 1 ignored) |
| `g_compose` | `test -p jigc --test g_compose --no-fail-fast` | 120 s | 0 | 4 | 26.27 s · 85.64 s (203 passed) |
| `g_config` | `test -p jigc --test g_config --no-fail-fast` | 118 s | 0 | 4 | 26.22 s · 83.28 s (236 passed) |
| `g_methodology` | `test -p jigc --test g_methodology --no-fail-fast` | 92 s | 0 | 4 | 25.11 s · 58.36 s (138 passed) |
| `g_item` | `test -p jigc --test g_item --no-fail-fast` | 76 s | 0 | 4 | 25.51 s · 41.91 s (173 passed) |
| `g_solo_trial_corpus` | `test -p jigc --test g_solo_trial_corpus --no-fail-fast` | 59 s | 0 | 4 | 24.41 s · 26.07 s (14 passed) |
| `g_migration` | `test -p jigc --test g_migration --no-fail-fast` | 55 s | 0 | 4 | 24.82 s · 21.22 s (88 passed) |
| `g_solo_store_sweep` | `test -p jigc --test g_solo_store_sweep --no-fail-fast` | 35 s | 0 | 4 | 24.09 s · 1.77 s (3 passed) |
| `unit` | `test --workspace --lib --bins --no-fail-fast` + `test --workspace --doc --no-fail-fast` | 82 s (47 + 35) † | 0, 0 | 4, 4 | 34.15 s · 535 (`cli` lib) + 96 (`jigc` bin) + 991 (`jigc_engine` lib) passed; 24.37 s · 0 doctests in `cli` and `jigc_engine` |
| `manifest-freeze` | `test -p jigc --test g_finalize manifest_freeze_fence::live -- --ignored --exact` | 38 s | 0 | 4 | 28.57 s · 1 passed |
| `publish-dry-run` | `publish --workspace --dry-run` | 45 s | 0 | 4 | 14.66 s + 35.90 s (the two verify builds) |
| `lock-current` | `metadata --locked --format-version 1` (stdout, 539,040 bytes, to a file) | 14 s | 0 | 4 | — |

† `unit`'s two commands ran as two invocations, and each recompiled cold, so their sum **over-estimates** a job that runs both in one checkout.

‡ **A defect, reported rather than recorded as a clean time.** `g_doc` exited 101 in **3 of 4** cold runs at this commit. The first run exited 101, and the re-runs exited 0, 101 and 101, taking 167 s, 168 s and 167 s. Every failure is the same test: `doc_code_probe::jigc_refuses_the_probe_argv_of_another_build` panics at `crates/cli/tests/doc_code_probe.rs:1209` with `write the request: Os { code: 32, kind: BrokenPipe }`. The cause is a race in the test's helper `run_jigc_with_stdin`, which `.expect()`s its `write_all` to the child's stdin. On a skewed `--build`, `jigc` refuses before it reads stdin (`probe_intercept`, `crates/cli/src/main.rs:116-125`) and exits, so the write can find the pipe closed. Production is not affected, because its invoker already discards that write's result (`let _ = stdin.write_all(request)`, `crates/cli/src/invoke.rs:230`). The helper arrived in Increment 2 (`6dfacc4d`), after the before run, so no CI runner has run it yet. As long as it stands, a per-group `g_doc` job can redden a green push. The time still counts for the verdict: the leg's wall clock stayed at 167–168 s across all four runs, passing or failing, so one test's outcome does not move it.

**Verdict: no leg over budget → per-leg compile.** The slowest leg is `g_flow`, at 221 s against the 878 s budget. Scaled whole by the largest runner-to-proxy ratio above (2.10×), it would take about 464 s, which is still inside the budget.

## The provisional after run — `36720872051` (Increment 9's entry gate)

This is S8's nine-job workflow (Increment 8 T4) on its first push. It ran on the push of `d4554b86`, Increment 8's tip. Read on 2026-09-30 for [planning-gate-record.md](planning-gate-record.md) → row 18. It is *provisional* because Increment 12 reads the final after run at Increment 11's tip against the same bound.

### The commands

Each value below is the output of one of these commands, cited by its id, and each was run twice for this record with the same output. `gh` was authenticated against `gherrink/jigc`. Logs and API bodies went to scratch files first and were read from there.

```sh
D=$(mktemp -d)
# A1: the run
gh run view 36720872051 --json databaseId,workflowName,headBranch,headSha,event,status,conclusion,attempt --jq '"\(.databaseId) \(.workflowName) \(.headBranch) \(.headSha) \(.event) \(.status) \(.conclusion) attempt=\(.attempt)"'
# A2: the run wall clock
gh api repos/gherrink/jigc/actions/runs/36720872051 --jq '"\(.run_started_at) \(.updated_at) run_attempt=\(.run_attempt)"'
# A3: every job's wall clock
gh run view 36720872051 --json jobs --jq '.jobs[] | "\(.databaseId)\t\(.name)\t\(.startedAt)\t\(.completedAt)\t\((.completedAt|fromdate) - (.startedAt|fromdate))s\t\(.conclusion)"'
# A4: every job's labels, runner group and runner
for j in $(gh run view 36720872051 --json jobs --jq '.jobs[].databaseId'); do
  gh api repos/gherrink/jigc/actions/jobs/$j --jq '"\(.id) \(.name) labels=\(.labels|join(",")) group=\(.runner_group_name) runner=\(.runner_name) \(.status) \(.conclusion)"'
done
# A5: every job's log, whole, to a file; then nproc, the line after the `Run nproc` group closes
for j in $(gh run view 36720872051 --json jobs --jq '.jobs[].databaseId'); do
  gh api --allow-escape-sequences repos/gherrink/jigc/actions/jobs/$j/logs > "$D/$j.log"
  printf '%s ' "$j"; awk '/##\[group\]Run nproc/{f=1} f&&/##\[endgroup\]/{getline; print; exit}' "$D/$j.log"
done
# A6: the cache state and the runner image, per job log
command grep -hE 'No cache found|Cache restored from key' "$D/<id>.log"
command grep -A2 'Runner Image$' "$D/<id>.log"
# A7: the slowest job's steps, compile and test result
gh run view 36720872051 --json jobs --jq '.jobs[] | select(.databaseId==109905363354) | .steps[] | "\(.number)\t\(.name)\t\((.completedAt|fromdate) - (.startedAt|fromdate))s\t\(.conclusion)"'
command grep -E 'Finished `|test result: ' "$D/109905363354.log"
# A8: the bound, over all jobs
gh run view 36720872051 --json jobs --jq '[.jobs[] | ((.completedAt|fromdate) - (.startedAt|fromdate))] | "max=\(max) over900=\(map(select(.>900))|length) n=\(length)"'
```

**Why A5 is the jobs-logs API, not `gh run view --log --job`.** The plan named `gh run view --log --job <id>`. Redirected to a file, it exits 0 but **drops whole steps** for some jobs, and the result is the same on a second fetch (`cmp` → 0). For `unit`, `publish-dry-run` and `test (g_doc)` it has no `CPU count` step at all: `g_doc`'s file holds only 77 `Checkout` lines, against 664 lines from the API. So an nproc read that way silently reports 17 jobs as 20. `gh api …/jobs/<id>/logs` refuses the body's terminal escape sequences unless it is given `--allow-escape-sequences`. Redirected to a file, nothing reaches a terminal, so the flag is safe here. With it, every job's log is whole. GitHub keeps run logs for 90 days by default, so A5 works until about 2026-12-29. The lines it printed are quoted below so this record outlives the log.

### The entry gate

| Check | Halt if | Read | Evidence |
|---|---|---|---|
| Run concluded | not `completed` | `completed` / `success` | A1 → `36720872051 CI main d4554b868e433d3f7daaf0e1955b24bf3172fda6 push completed success attempt=1` |
| Every job inside 15 min | any job over 900 s | **max 448 s**, no job over 900 s | A8 → `max=448 over900=0 n=20` |
| Trusted-publisher configs | either absent, or naming another repository, workflow or environment | both present and matching | T1 → T2 below |
| No `CARGO_REGISTRY_TOKEN` | any such secret exists | none | T4 below |

**No halt condition holds, so the gate passes.** The proxy's verdict stands: no leg is over budget on the runner either.

### Identity

| Value | Read | From |
|---|---|---|
| Run id · workflow · branch | `36720872051` · `CI` · `main` | A1 |
| Head sha | `d4554b868e433d3f7daaf0e1955b24bf3172fda6` (Increment 8's tip) | A1 |
| Event · attempt | `push` · `1` (not a re-run) | A1 |
| Run, started → updated | 13:20:02Z → 13:27:35Z, **453 s = 7 min 33 s**, against the before run's 2024 s = 33 min 44 s (C4) | A2 → `2026-09-30T13:20:02Z 2026-09-30T13:27:35Z run_attempt=1` |
| Runner, every job | labels `ubuntu-latest`, group `GitHub Actions`, 20 distinct runners (`GitHub Actions 1000000397`–`1000000416`) | A4 |
| Image | `ubuntu-24.04`: version `20260920.314.1` on 15 jobs, `20260927.320.1` on 5 (`fmt`, `publish-dry-run`, `g_milestone`, `g_config`, `g_solo_store_sweep`) | A6 |
| Cache | **cold on every cargo job**: `No cache found.` in all 19 that restore one. `hygiene` restores none | A6 |

So this is a **cold-cache** run, like the before run and T3's proxy. It is the bound a cache-miss push must meet.

### Every job

A3 prints the wall clock, A4 the labels and group, A5 the `nproc` line. Rows are sorted by wall clock. Every job concluded `success`, on `ubuntu-latest` in group `GitHub Actions`, and every `nproc` is **4**. The proxy column is T3's wall clock for the same leg, above.

| Job | Id | Start → end | Wall clock | nproc (A5, its line's time) | Proxy · runner ÷ proxy |
|---|---|---|---|---|---|
| `test (g_flow)` | 109905363354 | 13:20:06Z → 13:27:34Z | **448 s** | 4 (13:20:18.84Z) | 221 s · 2.03× |
| `test (g_doc)` | 109905363824 | 13:20:43Z → 13:26:45Z | 362 s | 4 (13:20:57.85Z) | 167 s · 2.17× |
| `test (g_milestone)` | 109905363665 | 13:20:42Z → 13:26:21Z | 339 s | 4 (13:20:54.92Z) | 174 s · 1.95× |
| `test (g_finalize)` | 109905363567 | 13:20:06Z → 13:24:33Z | 267 s | 4 (13:20:20.23Z) | 137 s · 1.95× |
| `test (g_migrate)` | 109905363547 | 13:20:07Z → 13:24:05Z | 238 s | 4 (13:20:20.90Z) | 121 s · 1.97× |
| `test (g_compose)` | 109905363579 | 13:20:08Z → 13:23:59Z | 231 s | 4 (13:20:22.09Z) | 120 s · 1.93× |
| `test (g_config)` | 109905363749 | 13:20:07Z → 13:23:02Z | 175 s | 4 (13:20:19.93Z) | 118 s · 1.48× |
| `test (g_item)` | 109905363500 | 13:20:07Z → 13:22:40Z | 153 s | 4 (13:20:22.49Z) | 76 s · 2.01× |
| `test (g_methodology)` | 109905363905 | 13:20:06Z → 13:22:01Z | 115 s | 4 (13:20:18.33Z) | 92 s · 1.25× |
| `test (g_solo_trial_corpus)` | 109905363108 | 13:20:07Z → 13:21:57Z | 110 s | 4 (13:20:18.09Z) | 59 s · 1.86× |
| `test (g_migration)` | 109905363849 | 13:20:06Z → 13:21:30Z | 84 s | 4 (13:20:18.84Z) | 55 s · 1.53× |
| `unit` | 109905363203 | 13:20:07Z → 13:21:26Z | 79 s | 4 (13:20:20.05Z) | 82 s † |
| `clippy` | 109905363223 | 13:20:06Z → 13:21:18Z | 72 s | 4 (13:20:19.22Z) | 38 s |
| `publish-dry-run` | 109905363338 | 13:20:06Z → 13:21:13Z | 67 s | 4 (13:20:19.83Z) | 45 s |
| `manifest-freeze` | 109905363406 | 13:20:06Z → 13:21:09Z | 63 s | 4 (13:20:19.95Z) | 38 s |
| `build` | 109905363175 | 13:20:07Z → 13:21:03Z | 56 s | 4 (13:20:20.21Z) | 33 s |
| `test (g_solo_store_sweep)` | 109905363809 | 13:20:06Z → 13:20:56Z | 50 s | 4 (13:20:16.31Z) | 35 s |
| `fmt` | 109905362831 | 13:20:07Z → 13:20:39Z | 32 s | 4 (13:20:19.93Z) | 10 s |
| `lock-current` | 109905363465 | 13:20:07Z → 13:20:37Z | 30 s | 4 (13:20:20.76Z) | 14 s |
| `hygiene` | 109905363422 | 13:20:06Z → 13:20:21Z | 15 s | 4 (13:20:18.52Z) | — |

† T3's `unit` figure is two cold invocations summed, which it flagged as an over-estimate. The job runs both in one checkout.

The timestamps are whole seconds, so each wall clock is ±1 s. The `nproc` times are the output line's own timestamp, inside that job's `CPU count` step.

**The runner-to-proxy ratio** for the twelve `test` rows runs from 1.25× (`g_methodology`) to **2.17× (`g_doc`)**. That top is above the 2.10× T3 used to scale `g_flow` to about 464 s, but `g_flow` itself came in at 2.03×, at 448 s. The ratio includes each side's set-up (the runner's checkout and toolchain install, the tool's clone), so it is not a clean platform factor.

### The slowest job: `test (g_flow)`

A7 prints its steps: Set up job 1 s · Checkout 10 s · CPU count 0 s · Show toolchain 8 s · Cache cargo dependencies 1 s · **`cargo test --test g_flow` 422 s** · Post Cache 4 s · Post Checkout 0 s · Complete job 0 s. Inside the cargo step, ``Finished `test` profile [unoptimized + debuginfo] target(s) in 34.90s`` and `test result: ok. 333 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 386.97s`. Its headroom to 900 s is **452 s**. The lever held in reserve if a later run crosses the bound is S8's: nextest partitioning of the group.

### Trusted Publishing, the `release` environment, the secrets

```sh
UA='jigc-release-check (https://github.com/gherrink/jigc)'
# T1: jigc; T2: jigc-engine. The token comes from ~/.cargo/credentials.toml inside the
# command substitution and is never printed. Without it the API answers 403 (T3).
curl -sS -o "$D/tp-<crate>.json" -w '%{http_code}' -A "$UA" \
  -H "Authorization: $(sed -n 's/^token *= *"\(.*\)"/\1/p' ~/.cargo/credentials.toml)" \
  "https://crates.io/api/v1/trusted_publishing/github_configs?crate=<crate>"
jq -c '.github_configs[] | {id,crate,repository_owner,repository_name,workflow_filename,environment}' "$D/tp-<crate>.json"
jq '.github_configs|length' "$D/tp-<crate>.json"
# T3: the same request without the Authorization header
curl -sS -o /dev/null -w '%{http_code}' -A "$UA" "https://crates.io/api/v1/trusted_publishing/github_configs?crate=jigc"
# T4: the secrets
gh secret list --repo gherrink/jigc
gh secret list --repo gherrink/jigc --env release
gh secret list --repo gherrink/jigc --app dependabot
# T5: the environment, and its deployment policies
gh api repos/gherrink/jigc/environments/release --jq '"\(.name) can_admins_bypass=\(.can_admins_bypass) reviewers=\([.protection_rules[]|select(.type=="required_reviewers")|.reviewers[]|"\(.type):\(.reviewer.login)"]|join(",")) branch_policy=\(.deployment_branch_policy|tostring)"'
gh api repos/gherrink/jigc/environments/release/deployment-branch-policies --jq '"total=\(.total_count) " + ([.branch_policies[]|"\(.id):\(.type):\(.name)"]|join(","))'
```

| Value | Read | From |
|---|---|---|
| `jigc` | HTTP 200, one config: `{"id":22294,"crate":"jigc","repository_owner":"gherrink","repository_name":"jigc","workflow_filename":"release.yml","environment":"release"}` | T1 |
| `jigc-engine` | HTTP 200, one config: `{"id":22295,"crate":"jigc-engine","repository_owner":"gherrink","repository_name":"jigc","workflow_filename":"release.yml","environment":"release"}` | T2 |
| Unauthenticated | HTTP 403 | T3 |
| Repository secrets | `JIGC_DENYLIST` only (created 2026-09-29T09:12:50Z) | T4 |
| `release` environment secrets · Dependabot secrets | none · none | T4 |
| `release` environment | `can_admins_bypass=false`, reviewers `User:gherrink`, `branch_policy={"custom_branch_policies":true,"protected_branches":false}` | T5 |
| Deployment policies | `total=1 61391261:branch:main` | T5 |

**No `CARGO_REGISTRY_TOKEN` exists** at any scope an Actions job can read. The owner is a user account, so there are no organisation secrets. **The deployment policy's id is `61391261`**, as the Increment 9 planning read found. [release.md](../../../implementation/release.md) → Publishing and [planning-gate-record.md](planning-gate-record.md) → row 18 still cite `61320798`. The policy itself, branch `main` only, is what they state. Only the id is stale. This task commits only records, so the citation is left for the task that edits release.md's Publishing pins.
