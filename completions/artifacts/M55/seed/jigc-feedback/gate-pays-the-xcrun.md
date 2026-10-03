---
kind: inconvenience
found-in: milestone:M55-planning/T4
about: dev:gate
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: dev_gate_report::the_gate_names_the_git_the_suite_runs_under
date: 2026-10-03
schema-version: 1
---

# The gate pays the xcrun git trampoline on every git call

## Description

On macOS, `/usr/bin/git` is an `xcrun` trampoline that costs about 11 ms a call, and a full gate makes about 174k git calls. Putting the real git first on `PATH` broke tree-sitter's C build unless `SDKROOT` was exported. The gate-speed measurement found it.

## Repro

```sh
# from the jigc checkout
dev/gate --quick | grep -E '^gate: (git|       SDKROOT)'
#   gate: git    <the real git, not /usr/bin/git>
#   gate:        SDKROOT … -- the real git, not the /usr/bin/git xcrun trampoline
cargo test -p jigc --test g_migrate dev_gate_report::the_gate_names_the_git_the_suite_runs_under   # 1 passed
```

## Resolution

Fixed by the gate-speed PR (#8, `56d9307a`, *dev/gate runs the suite on nextest under the real git*). When `git` resolves to the trampoline, `dev/gate` puts `dirname $(xcrun --find git)` first on `PATH`, exports `SDKROOT` from `xcrun --show-sdk-path` unless one is set, and names the git it runs under in a `gate: git` header line. The PR measured −29 % from this lever alone. A bare `cargo test` outside `dev/gate` still runs under whatever git is first on `PATH`.

Re-driven on this build (the checkout at `982f910f`). The full gate run of this re-drive printed `gate: git` naming the CommandLineTools git 2.54.0, not `/usr/bin/git`, and the `SDKROOT` line. Its test step took 448 s and passed.
