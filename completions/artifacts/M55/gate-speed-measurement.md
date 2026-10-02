# Local gate speed — measurement (2026-10-02)

Measured before the M55 build, on a fresh worktree of `origin/main` at `62c76009` (branch `work/gate-speed`), M1 Max, 10 cores, private target dir, every run a full `dev/gate` after touching both crates' `lib.rs`. Load average at run start 7–43 (mostly the previous run). End-of-day baseline 1156 s matched the morning's — no drift.

## Where the time went

- Steps: fmt 2 s · clippy 6–7 s · build 3 s · **test 1124–1138 s (98%)**.
- Groups (s): g_flow 221 · g_milestone 182 · g_finalize 137 · g_migrate 123 · g_doc 109 · g_methodology 81 · g_compose 74 · g_config 74 · g_item 49 · g_migration 30 · g_solo_trial_corpus 29 · rest < 5.
- CPU: user ≈ 4330 s, **sys ≈ 3080 s (40% kernel)**; ~1.13 M new PIDs per run; direct counts ~174k `git` and ~25k `jigc` invocations.
- Gatekeeper is no longer a cost (0–24 ms first-run per binary; syspolicyd ~5 s CPU per gate). The product never fsyncs (write-temp-then-rename).

## Levers (wall s, run 1 / run 2; baseline mean 1151 s; 4222 tests + 2 ignored in every row)

| Lever | Wall | Δ | Note |
|---|---|---|---|
| baseline | 1141 / 1155 (end 1156) | — | |
| nextest, one pool + `cargo test --doc` | 1064 / 1024 | −9% | 0 doctests either way |
| Developer Tools exemption | not toggled | ≤1% expected | needs the human (Ghostty) |
| `GIT_TEST_FSYNC=0` | 1170 / 1156 | 0 | |
| fresh on-disk TMPDIR | 1165 / 1152 | 0 | |
| RAM-disk TMPDIR | 1133 / 1153 | −1% | |
| clippy overlapping the test build | build phases 17 vs 18–20 | ≤3 s | cargo's lock serializes them |
| **real git instead of `/usr/bin/git` (xcrun trampoline)** | **833 / 798** | **−29%** | sys 3080 → 1660 s; needs `SDKROOT` exported or tree-sitter's C build fails |
| ambient-git-config mask | 1140 / 1128 | −1% | |
| **deps at `opt-level = 2`** | **960 / 968** | **−16%** | +40 s cold, +2.5 s incremental; `debug-assertions`/`overflow-checks` stay on |
| real git + nextest | 706 / 711 | −38% | |
| **real git (+SDKROOT) + nextest + deps opt 2** | **540 / 532** | **−53%** | recommended |
| real git + nextest + everything opt 1 | 510 / 493 | −56% | workspace code harder to debug |

## Decision

Land real git + nextest + deps `opt-level = 2`, with the two flaky tests and the TMPDIR leak fixed alongside (recorded in the M55 planning register as T1–T4), as the `work/gate-speed` PR before the M55 build (the human, 2026-10-02). The full suite stays the done-check of every task.
