# Machine setup

**Read once, when setting up a development machine.** Nothing here recurs — the day-to-day build/lint/test commands live in [CLAUDE.md](../CLAUDE.md) → *Build / lint / test*, and the loop that uses them in [dev-workflow.md](dev-workflow.md).

## macOS — exempt the terminal from Gatekeeper assessment

```sh
sudo spctl developer-mode enable-terminal
```

Then enable your terminal under **System Settings → Privacy & Security → Developer Tools**, and restart it. If your terminal is not listed, add it with `+`.

**Why, and why it is worth the two minutes.** macOS assesses every **freshly created** executable on its first run. The test suite creates a lot of them, and each assessment cost a **measured ~44 s** of `/usr/libexec/syspolicyd` — during which the test process sits at 0% CPU doing nothing. Confirmed by re-running an already-assessed binary with no code change: **64.1 s → 19.1 s**.

Before the test targets were consolidated this was catastrophic — 252 binaries × ~44 s ≈ **3 hours per gate run**, against 8 minutes of actual test execution. It was misdiagnosed for a long time as a slow suite ([DECISIONS.md](../DECISIONS.md) → 2026-08-05). The consolidation to twelve `[[test]]` group targets removed most of it; this setting removes the rest.

The trade-off is real and it is yours to make: binaries launched from that terminal are no longer Gatekeeper-assessed. That is normal for a development machine, but it is a genuine reduction in local malware checking, so it is a per-machine choice rather than something the repo can do for you.

**Linux / other:** nothing to do — this is macOS-specific.

## cargo-nextest — the gate's test runner

```sh
cargo install cargo-nextest --locked
```

`dev/gate` runs the suite as `cargo nextest run --workspace --no-fail-fast` and then the doctests with `cargo test --workspace --doc` ([dev-workflow.md](dev-workflow.md) → Gate). Without nextest the gate still runs the full suite — as `cargo test`, and it says so — but slower: measured on 2026-10-02, nextest alone made a full gate 9 % faster ([gate-speed-measurement.md](../completions/artifacts/M55/gate-speed-measurement.md)). Its configuration is [`.config/nextest.toml`](../.config/nextest.toml).

**On macOS, nothing to install for git**, but know what the gate does with it: when `git` resolves to `/usr/bin/git` — an `xcrun` trampoline costing ~11 ms a call, over ~174 k calls a gate — `dev/gate` puts the real git from `xcrun --find git` first on `PATH` and exports `SDKROOT`, and its `gate: git <path>` header line says which git ran. A hand-run `cargo test` gets neither.

## Sanity check

A full gate from a warm build should take **~9 minutes** (measured 532–546 s on a 10-core M1 Max, 2026-10-02 — [gate-speed-measurement.md](../completions/artifacts/M55/gate-speed-measurement.md)):

```sh
dev/gate
```

~~A full gate from a warm build should be **~5 minutes** (measured 4m34s–5m18s), over `cargo fmt --check` · `cargo clippy --all-targets -- -D warnings` · `cargo test` · `cargo build`.~~ **[Superseded 2026-10-02: the suite has grown since that figure; the same four commands measured 19.1 min before the gate-speed levers.]**

**[Retired 2026-09-29 (M54 Increment 2): this block carried a fifth line, the probe prebuild `cargo build --quiet --manifest-path crates/cli/probes/doc-code/Cargo.toml`. The `doc-code` probe runs inside `jigc` by self-exec, so there is no separate probe to build ([module-layout.md](module-layout.md) → Probe boundary).]**

If it runs dramatically longer than that and the machine is otherwise idle, check whether `syspolicyd` is burning CPU (`ps aux | grep syspolicyd`) before assuming the suite is at fault — that is exactly the wrong turn taken in M47.
