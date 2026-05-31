# Decisions pending

The forward look for the build: decisions each increment will **force**, surfaced now so nothing ambushes a dev-workflow run mid-task. This is a *decision* backlog, not a *task* backlog — tasks are still cut per-increment at pickup ([roadmap.md](roadmap.md)). Entries graduate to [DECISIONS.md](../DECISIONS.md) when made; delete them here once logged.

Two kinds, tagged: **(D)** a genuine open *design* question (a real choice, often already flagged in a doc's Open questions); **(I)** an *implementation* pick (crate/algorithm that falls out naturally at pickup, listed so it's not a surprise).

## Cross-cutting — settled 2026-05-31

Made up front because they shape many signatures; recorded in [DECISIONS.md](../DECISIONS.md). Listed here only as pointers:

- **(I) Error strategy** — `thiserror` (engine, typed) + `anyhow` (cli).
- **(I) Hashing** — `blake3`, one algo for `file-state` + content-drift.
- **(I) Test tooling** — `insta` (snapshot/golden) + `proptest` (property/fuzz).

## Cross-cutting — still open

- **(D) Slug / minting normalization** — case/charset rules + collision-suffix form. Open in [structural-grammar.md](../design/structural-grammar.md) → Open questions. **Decide before increment 3/4** (`create` / `add-item` mint ids). The one real design decision standing between us and the write path.

## Increment 2 — parser/writer

- **(D) Exact canonical byte form** — the precise bytes the canonical writer emits (blank-line placement, `<!-- fields -->` sentinel spacing, field-bullet form). [storage.md](../design/storage.md) / [parsing.md](parsing.md) specify the rules; the golden tests need them pinned to literal bytes.
- **(I) In-memory schema representation** — the Rust types the doctype YAML deserializes into (drives parse + the canonical writer).
- **(I) pulldown-cmark handling** — offset/front-matter edge behavior (the span-precision spike flagged in [parsing.md](parsing.md) → Open questions).

## Increment 3 — compose

- **(D) Task working-area on-disk layout** — the exact files under `.jigc/tasks/<id>/` (base-pin, bound context roles, staged docs). [storage.md](../design/storage.md) is illustrative.
- **(I) Data-value path resolver + command-catalog shell-quoting** — algorithms are specified ([workflow-dialect.md](../design/workflow-dialect.md), [command-catalog.md](../design/command-catalog.md)); this is implementing them.
- **(cleanup) Stale emitted-format open question** — [module-layout.md](module-layout.md) → Renderers still calls the emitted-format micro-syntax "an open question," but it's settled (the four-class format in [workflow-dialect.md](../design/workflow-dialect.md#emitted-format); VISION says settled 2026-05-28). Confirm and remove the stale ref.

## Increment 4 — write + finalize

- **(D) Git invocation** — shell out to the `git` binary vs a Rust lib (`gitoxide` / `git2`). Lean shell-out (respects user hooks; matches "CLI orchestrates, git executes" — [storage.md](../design/storage.md), [finalize.md](../design/finalize.md)), but a real call.
- **(D) Blocked / error payload shape** — the structured error+route the agent receives on a block. Open in [write-commands.md](../design/write-commands.md) → Open questions; touches finalize's hook-output relay too ([finalize.md](../design/finalize.md) → Open questions).

## Increment 5 — persisted ADR + edge index

- **(D) Edge-index storage format + stamp/rebuild** — on-disk shape of the rebuildable edge index and its HEAD/doc-set stamp. [storage.md](../design/storage.md) describes the lifecycle; the concrete format is ours to pick.
- **(I) Reconciliation state machine + rename detection** — specified in [reconciliation.md](../design/reconciliation.md); reuses the `blake3` hash choice.

## Increment 6 — adapter & ship

- **(D) Product name** — still `jigc` placeholder ([VISION.md](../VISION.md)). Shipping wants the real name decided before this increment.
- **(D) `jigc setup` injection mechanism** — idempotent markers for writing the bootstrap line + allowlist into `CLAUDE.md` / `.claude/settings.json` ([assistant-adapter.md](../design/assistant-adapter.md)).
- **(I) Release / cross-compile + quickstart** — packaging the single binary.
