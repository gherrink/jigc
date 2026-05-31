# Decisions pending

The forward look for the build: decisions each increment will **force**, surfaced now so nothing ambushes a dev-workflow run mid-task. This is a *decision* backlog, not a *task* backlog — tasks are still cut per-increment at pickup ([roadmap.md](roadmap.md)). Entries graduate to [DECISIONS.md](../DECISIONS.md) when made; delete them here once logged.

Two kinds, tagged: **(D)** a genuine open *design* question (a real choice, often already flagged in a doc's Open questions); **(I)** an *implementation* pick (crate/algorithm that falls out naturally at pickup, listed so it's not a surprise).

## Cross-cutting — settled 2026-05-31

Made up front because they shape many signatures; recorded in [DECISIONS.md](../DECISIONS.md). Listed here only as pointers:

- **(I) Error strategy** — `thiserror` (engine, typed) + `anyhow` (cli).
- **(I) Hashing** — `blake3`, one algo for `file-state` + content-drift.
- **(I) Test tooling** — `insta` (snapshot/golden) + `proptest` (property/fuzz).

## Cross-cutting — still open

- *(none — slug / minting normalization **settled 2026-05-31**: lowercase ASCII kebab-case + transliterate non-ASCII + numeric collision suffix in task-id merge order; see [DECISIONS.md](../DECISIONS.md). The write path is unblocked.)*

## Increment 3 — compose

- **(cleanup) Stale emitted-format open question** — [module-layout.md](module-layout.md) → Renderers still calls the emitted-format micro-syntax "an open question," but it's settled (the four-class format in [workflow-dialect.md](../design/workflow-dialect.md#emitted-format); VISION says settled 2026-05-28). Confirm and remove the stale ref.

## Increment 4 — write + finalize

- *(both **settled 2026-05-31** — git invocation: shell out to the `git` binary; blocked/error payload: reuse the `finding` shape (severity + located message + `route`). See [DECISIONS.md](../DECISIONS.md).)*

## Increment 5 — persisted ADR + edge index

- *(reconciliation OOB state machine + rename detection both **settled 2026-05-31** — `engine::file_state::reconcile_committed` (absorb / conformance-block / conflict-block over a committed doc) and `engine::file_state::detect_rename` (strong/weak signal for a missing tracked path, routed to git-revert, no ref rewrite); **wired to the command surface 2026-05-31** via `engine::file_state::reconcile_committed_store` inside `validate_task` (the `task validate` / `finalize`-preflight full sweep); see [DECISIONS.md](../DECISIONS.md).)*

## Increment 6 — adapter & ship

- **(D) Product name** — *settled 2026-05-31: ship the MVP as `jigc` (adopt the placeholder as the name). See [DECISIONS.md](../DECISIONS.md).*
- *(release / quickstart **settled 2026-05-31** — a clean `cargo build --release -p cli` produces `target/release/jigc`; `crates/cli/tests/release_smoke.rs` verifies the built binary's `--version` + `jigc setup`; `QUICKSTART.md` documents the loop, referenced from CLAUDE.md. Cross-compile is out of MVP scope. See [DECISIONS.md](../DECISIONS.md).)*
