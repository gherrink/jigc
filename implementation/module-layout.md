# Module layout

How the locked architecture — engine / CLI / domain pack / assistant adapter — becomes **real Rust crates and modules**. The *architecture* (the engine/CLI/pack separation, the determinism boundary, "engine ships empty," "MCP could be another frontend") is settled in [VISION.md](../VISION.md) and [CLAUDE.md](../CLAUDE.md); this doc is purely its physical realization in code. The language and single-binary goals are in [language-runtime.md](language-runtime.md); the parse module's internals are in [parsing.md](parsing.md). For the *why*, see [DECISIONS.md](../DECISIONS.md).

## Crate topology

**A two-crate workspace: `engine` (lib) + `cli` (bin), `cli → engine`.**

- **`engine`** — the neutral, **empty** core library: cascade resolution, the document/schema model, parsing & serialization, the doc registry, workflow composition, the validation engine, task/staging state, the edge index. It depends on no frontend and no domain content, and makes **no LLM calls**. Because it's a standalone library, a later **`mcp` bin** is just a third crate over the same engine — the one boundary that carries architectural weight.
- **`cli`** — the `jigc` binary frontend: argument parsing, command dispatch, the three renderers, adapter generation, and cascade-layer *location*. Depends on `engine`.

Rejected: a **single crate** (engine not separately consumable → "MCP later" becomes a refactor, and the frontend boundary blurs); **many micro-crates** (a shared-types crate + version churn + compile-graph overhead, premature for the MVP). Engine internals are **modules**, split into sub-crates only if compile times or reuse later force it — `parse` is the natural first split-out.

## The dev pack's home — embedded

The development pack (doc-type schemas, workflows, steps, default config) is **data, not logic**. The **engine consumes the pack-default layer through a `PackSource` provider trait** — the named, specified form of the previously hand-wavy "source abstraction" — so it stays empty of domain content. Minimal shape (illustrative):

```rust
// in `engine` — the provider boundary; the engine knows only this
trait PackSource {
    fn pack_version(&self) -> Version;                          // built-in: = binary version (override-reconciliation, below)
    fn list(&self, kind: PackResourceKind) -> Vec<ResourceId>;  // schemas | workflows | steps | config
    fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Bytes>;
}
```

`PackSource` is **how `cli` provides the pack-default cascade layer** to the engine — pack-default is just one located layer ([the I/O boundary](#the-io-boundary--cli-locates-engine-resolves)), not a special case. Two impls: **`EmbeddedPack`** (MVP — bytes embedded via `rust-embed`/`include_dir`, so `jigc` is one self-contained artifact, the single-binary value from [language-runtime.md](language-runtime.md)) and **`FilesystemPack`** (post-MVP — installable third-party packs read from a path). The dev pack versions *with* the release, which is exactly what override-reconciliation needs (built-in pack-default version = binary version, [overrides.md](../design/overrides.md)).

**Where the built-in bytes live — embed in `cli` now, relocate when a second frontend lands.** For the MVP's single frontend the `EmbeddedPack` bytes live embedded in the `cli` binary. The moment a second frontend exists (the post-MVP `mcp` bin), the embedded built-in pack + its `EmbeddedPack` impl **relocate to a shared `pack-builtin` data-crate** both bins depend on — so neither re-embeds the pack nor depends on the other's packaging, and **the engine never changes** (it only knows the `PackSource` trait). The crate is *not* created now: for one frontend it would be the premature ceremony [Crate topology](#crate-topology) already declined; the forward-binding rule makes MCP a mechanical move, not a retrofit. Installing the built-in pack to a filesystem path was likewise rejected for now — it only adds path-discovery + a second install step for a single built-in — and arrives later as `FilesystemPack` **through the same trait**, a free extension. Adapter profiles follow the same pattern (embedded now; relocate with the pack if a second frontend shares them).

This preserves the locked boundary as **internal discipline, not a public API** ([CLAUDE.md](../CLAUDE.md) non-goals): the engine loads pack content generically; domain specifics never leak into engine code. A second domain is what would turn the boundary into a real API — not yet.

## The I/O boundary — CLI locates, engine resolves

A clean split that keeps the engine neutral *and* testable:

- **`cli` does bootstrap & presentation I/O** — it locates the three cascade layers (pack-default **embedded** · team **external** `~/.config/jigc/` · project **in-repo** config dir, per [overrides.md](../design/overrides.md)), finds the repo root, and hands the engine its run context; then it renders the engine's results.
- **`engine` does logic & managed I/O** — it resolves the cascade and owns all managed content I/O: the documents, the `.jigc/` tree (committed config + gitignored caches/staging), the edge index ([storage.md](../design/storage.md)).

So the engine is fed its layers and asked for results — *feed layers in, assert results out* — which is what makes the deterministic core directly testable.

## Renderers (in `cli`)

Presentation is strictly **downstream of the deterministic engine result** — the engine never formats for a surface.

- **The engine's public result types are the renderer contract** — `ComposedWorkflow`, `DocView`, `Vec<Finding>`, `TaskStatus`, `Catalog`, … all `Serialize`. **JSON is then generic** (`serde_json` over any result, no per-type code); **agent-text and human are per-type** rendering in a `cli::render` layer. These types are a **versioned semantic API, not incidental `serde` output**: their JSON projection is part of the contract (explicit serde attributes + a schema-version marker), and changes are intentional and versioned — so the renderers *and* any external JSON / future `mcp` consumer bind to a stable surface, not to whatever `#[derive(Serialize)]` happens to emit. MVP keeps this as internal discipline (one frontend, [CLAUDE.md](../CLAUDE.md) non-goals); locking the stance now means a non-CLI consumer is zero-rework rather than silently broken by a field rename.
- **Format selection:** default **agent-text** (the primary consumer is an agent reading piped, non-TTY stdout); **interactive TTY → human-pretty**; `--format=agent|json|human` overrides. MVP human-pretty is *agent-text + light styling*; the **`ratatui` TUI is an additive `render::tui` module, post-MVP**.
- **The non-interactive floor** is intrinsic: the agent path never blocks on a prompt. It's trivially met at MVP (no interactive prompts — `finalize` is autonomous); the post-MVP TUI must preserve it (every interactive affordance keeps a flag/JSON twin).
- **Honest dependency:** the agent-text renderer *for composed workflows* is gated on the **emitted-format micro-syntax**, an open question in [workflow-dialect.md](../design/workflow-dialect.md). The seam exists (a `render` over `ComposedWorkflow`), but that specific output finalizes only once the micro-syntax is decided; other agent-text outputs don't block.

## Adapter (in `cli`)

- **Profiles are embedded data** (Claude Code in-box; more installable later), same pattern as the pack. `jigc setup` / `jigc adapter install --assistant claude-code` **generates** the adapter from `profile + engine catalog` and **regenerates on upgrade**, so it can't rot into a static pile ([assistant-adapter.md](../design/assistant-adapter.md)).
- The generator **writes into the host project's assistant files** — the bootstrap static line into `CLAUDE.md`, the `jigc` allowlist into `.claude/settings.json`, and catalog-derived per-workflow launchers (slash commands), each just `jigc start --workflow X`.
- **MVP scope:** the static-line **floor** + the **allowlist** (the path-of-least-resistance the bootstrap depends on). The **hook** (primary injection) and the **spawn binding** are post-MVP — the spawn payload is composed by the *engine* and rendered through the profile's launch template, but it rides on fan-out (post-MVP).

## Probe boundary (in `engine`)

- A **`Probe` trait** (`check(target, ctx) -> Vec<Finding>`, read-only); the engine owns scope→target resolution, scheduling, severity-via-cascade, aggregation, and the `finalize` gate (the fat-engine/thin-probe split, [validation.md](../design/validation.md)).
- **Two implementations:** **in-process** (engine-native `workflow-refs`, `file-state`, `override-default`-at-contract) compiled in; **subprocess** (pack probes, JSON-in/out) — the only place a non-Rust, possibly-untrusted program runs. The **determinism contract the subprocess impl must satisfy** is locked in [validation.md](../design/validation.md) → Pack-probe determinism contract: six rules (no network, no model, no time/random, read-only fs outside scratch, bounded resources, JSON-only IO) + four meta-finding failure modes (timeout / crash / malformed-output / sandbox-violation, all intrinsic blocking). OS-level sandboxing (seccomp, Landlock, equivalents) is deferred with the subprocess impl but cannot ship without satisfying the contract.
- **MVP ships the trait + in-process impls only.** The subprocess invoker and JSON contract type are *designed for* but not built — the trait must admit the subprocess impl with **zero engine change**. What secures that claim is the trait's `ctx`: it is **effective-state the engine can serialize**, so the in-process and subprocess impls consume the same logical input (a live graph vs its read-only snapshot + path-ref) — the [wire-contract sketch](../design/validation.md#the-wire-contract--request--response) in validation.md. **Probe executables live outside the workspace** (any language); the dev pack's `doc-code` (post-MVP) is a separate program, wired in because the schema's typed leaves imply it (`code-anchor` ⇒ `doc-code`, [document-type-schema.md](../design/document-type-schema.md)).

## The dependency graph

```
cli (bin) ──depends──▶ engine (lib)
  ├─ embeds: dev-pack data (EmbeddedPack) + adapter profiles
  ├─ provides cascade layers: pack-default (PackSource::EmbeddedPack) · team (~/.config/jigc) · project (in-repo)
  ├─ render: agent-text · json · [tui post-MVP]   (over engine result types)
  └─ adapter generation (profile + engine catalog → host project files)

engine (lib) ──depends──▶ (no frontend, no domain content)
  ├─ modules: cascade · schema · parse · doc-registry · compose · validate · state · index
  ├─ PackSource trait: EmbeddedPack (MVP) + FilesystemPack seam (post-MVP installable)
  ├─ Probe trait: in-process impls (MVP) + subprocess seam (post-MVP)
  └─ result types (Serialize, versioned API) = the renderer / JSON / MCP contract

post-MVP:  mcp (bin) ──▶ engine          probe executables (external, any lang) ◀── subprocess Probe
           cli + mcp ──▶ pack-builtin (shared EmbeddedPack data-crate)   # built-in pack relocates here when a 2nd frontend lands
```

## Open questions

- **Engine internal module → crate splits** — kept as modules now; revisit if compile times or cross-frontend reuse demand crates (`parse` first).
- **Embedded-resource mechanism** — `rust-embed` vs `include_dir` vs build-script for `EmbeddedPack`'s bytes. (The provider boundary itself is now specified — the [`PackSource` trait](#the-dev-packs-home--embedded) with `EmbeddedPack` / `FilesystemPack` impls; what stays open is only the embed mechanism.)
- **Subprocess probe implementation** — the OS-level sandboxing tech (seccomp, Landlock, equivalents) and the *concrete* request/response schema (versioning policy, finding envelope, snapshot format), deferred with the pack-probe API. The determinism contract **and the wire seam** the implementation must satisfy are locked in [validation.md](../design/validation.md) → Pack-probe determinism contract (the [wire-contract sketch](../design/validation.md#the-wire-contract--request--response) secures the trait's zero-engine-change claim).
