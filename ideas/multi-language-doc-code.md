# Multi-language doc↔code — reaching every language a project uses

**Status: shaped, unscheduled.** Hard design settled 2026-06-19 ([DECISIONS.md](../DECISIONS.md) → 2026-06-19 multi-language doc↔code north-star); the first six languages' `symbol-exists` shipped in **M27**, and **CSS shipped in M28** (the first breadth target — the HD1 addressable-unit keystone proven on the most-different model; [validation.md](../design/validation.md) → Multi-language resolution → CSS addressable units), and **YAML/docker-compose shipped in M29** (the second breadth target — the first language to *ride* the finished seam, mapping keys as addressable units; [validation.md](../design/validation.md) → Multi-language resolution → YAML addressable units), and **the Vue SFC tier-1 slice shipped in M41** (the first *envelope* target — a `.vue` component's `<script>` symbols + a filename-component unit, its SFC extract seam authored **fresh** since no prior HD tier covered an envelope format; [validation.md](../design/validation.md) → Multi-language resolution → Vue addressable units). The note stays open for the rest (JSON/TOML, HD2/HD3/HD4). Indexed from [VISION.md](../VISION.md) → Open questions. **Promote into `design/validation.md` (and the reading order) when scheduled** — milestone by milestone, as each tier earns it.

## Why this is existential, not a feature

The product's headline differentiator is [VISION.md](../VISION.md) principle #6 — *validate docs against the actual code*. **A tool that only does that for Rust is a Rust tool.** For "a context compiler for **coding agents**" to be true, doc↔code validation must reach every language a real project actually uses — Python, PHP, bash, CSS, TypeScript, JavaScript, and onward (Go/Java/Ruby/…). M27 proved the **extension model** (grammar-by-extension → per-language node-kind allowlist → static tree-sitter parse, probe-internal, determinism-safe, grammars out of the engine lock graph). The hard architecture is done; this note shapes the path from "six languages" to "every language."

## The trap: two capabilities that scale very differently

`doc-code` carries two distinct checks. Conflating them — e.g. asking "is-a-test" of CSS — is the mistake to avoid:

- **`symbol-exists`** — "does the cited code thing exist?" (`arch-doc.implemented-by`, `adr.cites-code`). The **dominant, foundational** check and the heart of "validate against reality." Generalizes to *anything* (HD1).
- **`criterion-maps-to-test`** — "is this symbol actually a *test*?" (`spec.maps-to-test`). **Narrower, harder, only meaningful where tests exist.** CSS has no tests; most backend languages do (HD2/HD3).

## HD1 — the keystone: generalize "symbol" → "addressable unit"

**Problem.** `symbol-exists` reads the `name` *field* of declaration nodes. CSS/YAML/HTML have no name-field declarations — CSS has **selectors** (`.btn`, `#header`), **custom properties** (`--color`), **`@keyframes` names**; YAML has **keys**.

**Design.** Generalize the per-language allowlist from *"(node-kind, read `.name`)"* to *"(node-kind → an **extractor** yielding a unit name)"*. A language's **addressable units** = the `(kind, name)` set its extractors produce:
- Programming languages → the current `.name`-field read (now just *one* extractor kind — fully backward compatible; the M27 allowlists are unchanged).
- CSS → class / id / keyframe / custom-property names, **sigils stripped** (`.btn`→`btn`, `#header`→`header`, `@keyframes spin`→`spin`); compound selectors (`.card > .title`) contribute *all* their names.
- YAML / JSON / TOML → mapping keys (or key-paths).

The address stays **`path#name`** = "any addressable unit named `name`" — the exact "any declaration named X" semantics, over a richer unit set. A typed **`path#kind:name`** qualifier (to distinguish a class `.x` from an id `#x`) is a **deferred escape hatch**, built only when real ambiguity bites (no premature generality).

**Why it's the keystone:** `symbol-exists` becomes "**addressable-unit-exists**" with *the same check-id, the same `code-anchor` field-type, zero schema change* — CSS rides the existing path; the probe just gains a CSS extractor. It preserves M27's "probe-internal only" property and turns "support language N" into one uniform move (define N's addressable units) instead of N special cases.

## HD2 — call-based tests (JS/TS): label-addressing

**Problem.** A Jest/Vitest test is `test("renders empty", fn)` — a *call with a string label*, not a named symbol. `path#symbol` has nothing to anchor.

**Not a determinism-boundary issue** (a correction to an early framing): resolving a label is still a static parse, fully deterministic. What it bumps is the **stable-ID philosophy** — a test *label* is mutable prose, less stable than a symbol. A *quality* tension, not a hard invariant.

**Design (full label-addressing — settled to build, not defer).** `maps-to-test` on a JS/TS file supports:
- `path` (bare file) → the test file exists (the stable floor).
- **`path#test:"label"`** → an exact match against a `test`/`it`/`describe` call whose first string argument equals `label`. **Exact match → blocking-eligible** (the label is either present or absent — not a heuristic), with the **mutability caveat documented** and **demotable to advisory via the cascade knob** for projects that prefer it. Camp-A languages keep the stable **symbol-based** is-a-test (named test fns).

## HD3 — is-a-test confidence tiers (cross-language honesty)

**Problem.** is-a-test is a *guarantee* in Rust (`#[test]` is a language attribute) but a *heuristic* elsewhere (pytest's `test_*` naming is convention, configurable — false positives like a `test_helper`, false negatives for renamed conventions). Running the test runner — the only exact answer — is forbidden by the determinism contract.

**Design — two tiers by how the language marks tests:**
- **Attribute-based** (Rust `#[test]`, PHP `#[Test]`) → high-confidence, **blocking-eligible**.
- **Name-convention** (pytest `test_*`, shunit2) → heuristic, **advisory only** — `symbol-exists` stays the hard floor (a vanished test symbol still blocks), and is-a-test merely *warns* "symbol exists but doesn't match the test convention." The convention pattern can become a cascade knob if projects diverge.

This keeps the hard gate exact and refuses to dress a heuristic up as a guarantee.

## The unified address & severity taxonomy

Two orthogonal axes — *what is addressed* and *confidence* — collapse to one rule: **exact resolution blocks; heuristic resolution advises.**

| resolution | example | severity |
|---|---|---|
| addressable unit exists | `file.ts#parseConfig`, `styles.css#btn`, `compose.yaml#web`, `AppLayout.vue#AppLayout` | **blocking** (exact) |
| attribute-marked test | Rust `#[test]`, PHP `#[Test]` | **blocking** (exact) |
| exact test-label match | JS/TS `app.test.ts#test:"renders empty"` | **blocking** (exact; knob-demotable, label-fragile) |
| name-convention test | pytest `test_*`, shunit2 `test*` | **advisory** (heuristic) |
| un-grammared file | `notes.md#foo`, any language with no shipped grammar | **advisory** (`unsupported-language`, M27) |

## The tiered roadmap

Ordered by value × tractability; each tier is one or more milestones, promoted into `design/validation.md` as it ships:

1. **Breadth of `symbol-exists`** — the high-value, proven-tractable tier. **CSS first — ✓ shipped M28** ([validation.md](../design/validation.md) → CSS addressable units) (already parked, most-different model — proves the HD1 addressable-unit generalization; the live PHP/TS/Vite project needs it), then **YAML/compose — ✓ shipped M29** ([validation.md](../design/validation.md) → YAML addressable units) (the first language to *ride* the seam CSS forced — mapping keys, proof the keystone generalizes cheaply), then **Vue SFC — ✓ shipped M41** ([validation.md](../design/validation.md) → Vue addressable units) (the first *envelope* target — a `.vue`'s `<script>` symbols resolved under vendored `tree-sitter-typescript` unioned with a filename-component unit; the SFC extract seam is authored **fresh** — no existing HD tier covered an envelope format, so it was originated at M41, Settle fork 3, not de-parked from an existing tier), then Go/Java/Ruby/… as adopters arrive. Each = a grammar + an addressable-unit definition.
2. **Camp-A is-a-test (advisory)** — Python/PHP/bash/… name-convention predicates, advisory over the symbol-exists floor (HD3 tier 2); attribute-based (PHP `#[Test]`) blocking-eligible (tier 1).
3. **JS/TS label-addressing** — the `path#test:"label"` form (HD2), exact-match blocking + knob.
4. **Grammar scaling (HD4) — only when it bites.** Bundle-all today (one probe, one embed-extract; raise the size guard per-milestone as M27 did — fine to ~two dozen grammars, most are small). Keep `grammar_for` + the allowlist as **one centralized registry seam** so pluggability stays possible later. **Defer** dynamic/pluggable grammars — loading a tree-sitter `.so` is native-code execution, a determinism/trust wrinkle the compiled-in model cleanly avoids — until binary size actually forces it (~>30 MB probe).

## Honest bounds (carry into every tier)

- **No running anything.** The determinism contract forbids build/run/network, so every predicate is a **static heuristic** — "this selector is declared," "this fn matches the test convention" — never "this test passes." Exact for *existence*; the honest ceiling for *is-a-test*.
- **Label fragility (JS/TS).** `#test:"label"` anchors break when a test label is reworded — documented, knob-demotable, the price of call-based frameworks.
- **Deferred until earned:** the `#kind:name` qualifier (HD1), pluggable grammars (HD4), and each language beyond the live adopters' set — no generality for a single use until a real project earns it (the same need-driven discipline as the M17-frictions cluster).

**2026-07-22 — the HD2/HD3 trigger FIRED** (project-alpha-3.0 — the first real `maps-to-test` campaign, on the Pest corpus; no prior trial wrote the field — [RC-alpha3/findings-verification.md](../completions/artifacts/RC-alpha3/findings-verification.md) → §3-C): the first real `maps-to-test` authoring campaign found Pest `it(...)` closures unanchorable (call expressions are in no allowlist — per-criterion anchoring impossible; file-only anchors are the discovered fallback), which is HD2's label-addressing shape in PHP plus HD3's is-a-test tier. Two riders regardless of when the tier lands: the M42-added `<path>#<test-fn>` prompt hint is the misleader on closure frameworks (reword + name the file-only fallback), and the `unsupported-language` message lies on the resolved-symbol/is-a-test-unverifiable fork ("no grammar" when the grammar resolved the symbol — `probes/doc-code/src/main.rs:399-402`). On the **M45 Settle agenda** (decisions-pending → the rc.9 wave, fork 9).
