# Language & runtime

The first **implementation** decision: what we build the engine and CLI in. This directory is the counterpart to [`design/`](../design/) — `design/` specifies *what the system is* (frontend- and language-neutral); `implementation/` records *how we build it*. Implementation docs cross-reference *into* the design and the thesis, never the reverse, so the design stays language-neutral.

For the *why* in one line, see [DECISIONS.md](../DECISIONS.md). Sibling docs (written as their topics are discussed): [`parsing.md`](parsing.md), [`module-layout.md`](module-layout.md).

## Decision

**The engine and the CLI are written in Rust.**

One sentence: the project's entire value proposition is **exhaustive structural correctness** — every cross-reference resolves, every finding is routed, every delta is classified clean/conflict/orphaned, every leaf / placeholder / field / delta *kind* is handled — and the design is built from at least five major tagged unions plus enums throughout. Rust turns *"did we handle every case?"* from a discipline into a **compile error**, which is precisely the class of silent bug the whole system exists to kill ([VISION.md](../VISION.md) → the determinism boundary). A forgotten `route` or `delta` variant is exactly the failure mode we cannot afford.

## Constraints that drove it

These are derived from the locked design, not generic CLI wisdom — they are what made the choice.

1. **Per-invocation cold start.** The thesis routes *every* read and write through `jigc`, just-in-time, with no daemon — a fresh process per call ([VISION.md](../VISION.md) principle #3; [bootstrap.md](../design/bootstrap.md)). Startup latency is paid dozens of times per task, and adoption is explicitly bet on `jigc` being the *path of least resistance* over grepping. Fast start is a first-class requirement, heavier here than for a normal CLI.
2. **Single-binary distribution.** The adapter installs `jigc` and allowlists it on every agent/human machine ([assistant-adapter.md](../design/assistant-adapter.md)). A self-contained static binary (download, run, trivial cross-compile) beats any runtime-dependency install story. *(Held literally again since M54 — settled 2026-09-28, landed 2026-09-29 in Increment 2: the bundled `doc-code` probe, a second executable beside `jigc` from M10 to rc.21, runs **inside** it by self-exec, so an install is one file again ([module-layout.md](module-layout.md) → Probe boundary). The price is paid at build time, not run time: the probe's tree-sitter grammars are C, so building `jigc` from source needs a C compiler, and the binary is ~15 MB (measured on the Settle's spike), not the 2–5 MB the table below cites for a Rust CLI ([release.md](release.md) → Installing).)*
3. **Lossless, schema-driven Markdown round-trip — the #1 technical risk.** The store is canonical Markdown that must re-serialize diff-clean and absorb out-of-band human edits ([storage.md](../design/storage.md), [write-commands.md](../design/write-commands.md) → reconciliation). Needs a source-span-aware parser so edits are surgical. *(Approach → [`parsing.md`](parsing.md).)*
4. **A union-heavy structural domain.** Leaf kinds (`slot`/`field`), placeholder kinds (command-ref/data-value/include), delta kinds (scalar/structural/slot-fill/fork), the finding `route` union, field types — the design is sum types end to end. Compiler-enforced exhaustiveness is the differentiator.
5. **The CLI core makes zero LLM calls** ([VISION.md](../VISION.md) non-goals). Counterintuitive but load-bearing: it kills the "it's an AI tool, use Python" reflex — the core is a deterministic file/graph/text engine, so AI-SDK maturity is irrelevant to it.

Plus two from the human/agent dual surface ([discussed here](module-layout.md)): the CLI must serve **both** agents (plain text / JSON on stdout) and humans (eventually a TUI), and **the non-interactive path is always the floor** — the agent path must never block on an interactive prompt (generalizes [write-commands.md](../design/write-commands.md): `finalize` defaults to autonomous).

*Non-discriminating* (all candidates handle these — they did not sway the call): shelling out to git, the rebuildable on-disk cache (no DB, modest scale), the library-first engine/CLI/MCP module split, snapshot testing. Even fan-out concurrency barely counts — the merge is a deterministic, ordered, in-CLI pass and the assistant does the spawning, so Go's concurrency story is not the asset it appears.

## Candidates considered — and why not

| candidate | verdict |
|---|---|
| **Python** | **Eliminated.** Its one advantage (AI ecosystem) is moot under constraint #5, while its two weaknesses (startup, distribution) hit our two hardest constraints (#1, #2) head-on. |
| **TypeScript on Bun/Deno** | **Eliminated.** The plan rested on Bun neutralizing startup/distribution and on a Markdown edge — research collapsed both: a *compiled* Bun binary starts in tens of ms (sometimes slower than Node), not the single-digit ms of `bun run`; and the Markdown advantage dissolves into a language-agnostic strategy (see below). The dual-surface constraint is fatal: the best TS TUI (`ink`) is effectively Node-bound, so **Bun and a good TUI are mutually exclusive**. TS only survives as Ink-on-Node, which forfeits the startup advantage that justified TS at all. |
| **Go** | **Close runner-up.** Best-in-class TUI dual-mode ergonomics (`bubbletea` v2 `WithoutRenderer`), simplest cross-compile, fast iteration. Lost on the one axis that matters most here: **no sum types, no exhaustiveness checking** — precisely where the design is densest. Mitigable with linters + discipline, but discipline is the thing the project exists to replace with guarantees. |
| **Rust** | **Chosen.** Exhaustive `enum` + `match` is a native fit for the union-heavy domain; single-digit-ms start; the **smallest** binary (~2–5 MB); cleanest Markdown byte-offset story (`pulldown-cmark`); pure-Rust toolchain (no C-dep cross-compile friction) for every library we need. Its usual cost — iteration speed — is unusually low for this workload (per-invocation parse → transform → serialize, mostly-owned data, no shared-mutable concurrency). |

The deciding weighting was explicit: **compiler-enforced exhaustive correctness (Rust) over iteration speed + TUI ergonomics (Go)** — correct for a system whose soul *is* exhaustive correctness.

## Corollaries (detailed in their own docs)

These follow from the Rust decision and are recorded where they live, not here:

- **TUI** → `ratatui` + `crossterm`, post-MVP, with the non-interactive / JSON path as the floor → [`module-layout.md`](module-layout.md).
- **Markdown** → `pulldown-cmark` with an **offset-splice / never-re-stringify / re-parse-to-validate** strategy (every AST stringifier reformats; the diff-clean path is splicing edits into the original byte buffer in reverse source order) → [`parsing.md`](parsing.md).
- **Polyglot seam unchanged** → the core stays one language/one binary; the only cross-language boundary is **pack probes across a JSON process contract**, post-MVP ([validation.md](../design/validation.md)) → referenced from [`module-layout.md`](module-layout.md). *(Since M54 — landed 2026-09-29, Increment 2 — the one shipped probe is Rust compiled into `jigc` and spawned as `jigc` itself, but it still crosses that JSON process contract with its own wire types, so the seam stays language-blind for third-party probes.)*

## Evidence basis

Decided 2026-05-25 after three parallel research passes (runtime/distribution, Markdown round-trip, TUI ecosystem). Recorded so the reasoning is auditable — and because the research **reversed an initial TS-on-Bun lean**. Load-bearing findings:

- **Compiled Bun startup ≠ `bun run` startup.** Real Bun-compiled CLIs measured ~80–104 ms (one *slower* than Node's ~64 ms), vs single-digit ms for Go/Rust; Bun binaries are ~60–116 MB vs Rust's ~2–5 MB. Bun's distribution story is solid (Claude Code ships as a `bun build --compile` binary) — but that's packaging, not cold start.
- **`ink` is effectively Node-bound.** "Claude Code uses Bun" is build/packaging only; running it on Bun was closed *not planned*. Ink's interactive raw-mode on Bun is unsupported/best-effort (real Windows raw-mode bugs); on Deno it's a community fork; and Ink adds Yoga-WASM startup latency.
- **Markdown round-trip is solved by approach, not language.** All AST stringifiers normalize formatting; the diff-clean technique (offset-splice + validate) is the same in any language, so Markdown stopped being a TS differentiator.

Key sources: Bun executables docs (bun.com/docs/bundler/executables); Tigris Node→Bun CLI migration (tigrisdata.com/blog/using-bun-and-benchmark); claude-code#3108 (run-on-Bun "not planned"); markdownlint `--fix` surgical-edit model (DavidAnson/markdownlint); `pulldown-cmark` `OffsetIter`; `ratatui`, `bubbletea` v2.

## Deferred

- **Exact crate set** beyond the load-bearing ones (`clap`, `serde`, `pulldown-cmark`, `ratatui`/`crossterm`) — chosen as modules are planned.
- **Build / lint / test commands** — land in [CLAUDE.md](../CLAUDE.md) when implementation actually begins (per its standing note).
- **Minimum supported Rust version & edition** — pinned at project setup.
