# M27 — multi-language doc↔code measure (the polyglot differentiator, in the wild)

**Run 2026-06-19.** The M27 done-bar mandated by [worked-examples.md](../../../design/worked-examples.md)
→ flow 29, [validation.md](../../../design/validation.md) → Multi-language resolution,
[DECISIONS.md](../../../DECISIONS.md) → 2026-06-19 fork F7 + the M27-planning rebuild + re-pin,
[measurement.md](../../../design/measurement.md) (recorded-alongside posture), and
[roadmap.md](../../../implementation/roadmap.md) → M27 Increment 4 (T4). M10/M13 proved the
`doc-code` differentiator over **Rust only** (the anchors pointed at jigc's own codebase, sidestepping
the Rust-only grammar); M27 generalizes the probe Rust→six languages, so a citation into a
**TypeScript** or a **Python** file genuinely validates against reality instead of silently passing.
This artifact records the **four measured facts of flow 29**, each **observed by running the
ACTUALLY-PINNED `cargo install`-built binary** (NOT `cargo test`) over a polyglot scratch repo with
**no environment overrides** — the production probe-resolution path a real install hits.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `be7244296adb1fbc4f991c437dde9b99fd538ecaf83c7b6afea981ed2eecc6ca` |
| `doc-code` probe sha256 | `cebfaaaccbfaf5e4ef7a2d7d13a27d3c7fcfdb7f0900976e5c079ec246eebdc7` |
| HEAD commit | `a261708` (Inc-4 T3, the last prose-qualification commit; clean tree) |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the **six-grammar** `doc-code` probe — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP + bash, six tree-sitter crates / seven language variants — and embeds it via `OUT_DIR/doc-code`); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` relocated `jigc` to `~/.cargo/bin/jigc`; it was copied to `~/.local/bin/jigc`; a `jigc setup` run from **each** bin dir extracted that dir's `doc-code` sibling from the embedded copy. **All four sha256 identical** — `jigc` identical across both dirs (`be72442…`), `doc-code` identical across both dirs (`cebfaaa…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The relocated `jigc` is **byte-identical** to the freshly-`cargo install`-built `target/release/jigc` (same `be72442…`); the extracted `doc-code` siblings are byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` (`cebfaaa…`) — i.e. the pinned binaries match a fresh `cargo install` at this HEAD. |
| Size | release `jigc` **13.4 MB** (13 355 496 B), `doc-code` **7.9 MB** (8 181 016 B). The release pair is small; the size guard's **90 MB** ceiling ([`crates/cli/tests/cargo_install_probe.rs`](../../../crates/cli/tests/cargo_install_probe.rs), tightened from 150 MB at Inc-4 T2) protects the **debug** `CARGO_BIN_EXE_jigc` (~71 MB with the embedded multi-grammar debug probe + debuginfo) so future grammar bloat or a re-swept `target/` embed trips it. |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (**no** `JIGC_PACK_DIR`) and the `doc-code` probe resolved **as the sibling beside the installed binary** (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s both) — the production probe-resolution path (the M20 embed/sibling), exercised live with the M27 grammar set. |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — one detached polyglot scratch repo + isolated `$HOME` per arm; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23/M24/M25/M26):
this is an **owner-artifact recording** (the F7 *recorded-alongside* measure run on the actually-pinned
binary), **not** a TDD red→green test — its done-criterion is this committed artifact plus the clean
end-to-end runs on the installed binary. The blocking reds, the per-item disambiguation, and the
pass↔block masking guard are independently **binary-proven** by
[`crates/cli/tests/flow29_acceptance.rs`](../../../crates/cli/tests/flow29_acceptance.rs) (Inc-4 T1)
against this same binary's source. This artifact's job is to show those facts *on the binary a real
install actually resolves*, with the production sibling probe and no overrides.

## Corpus — a polyglot scratch repo (mirrors flow 29 / `flow29_acceptance.rs`)

A real `git init` repo carrying **two languages**, an `arch-doc` with **two** components, each
`implemented-by` anchored at a symbol in a different language, plus a header `cites → adr` over a
**committed-first** decision:

| | component A (TypeScript) | component B (Python) |
|---|---|---|
| source file | `src/api.ts` | `services/limiter.py` |
| present symbol | `export class RateRouter { … }` (a `class_declaration`) | `class TokenLimiter: …` (a `class_definition`) |
| anchor (`implemented-by`) | `src/api.ts#RateRouter` | `services/limiter.py#TokenLimiter` |
| item address | `arch-doc:gateway#components/edge-router/implemented-by` | `arch-doc:gateway#components/token-limiter/implemented-by` |
| "deleted" form (symbol renamed away, file stays) | `export class RouterRenamed { … }` | `class LimiterRenamed: …` |

Each arm: `jigc setup` (extract the sibling probe + tracked config) → commit the cited `adr:use-a-cache`
→ `jigc start --workflow architecture-documentation` → `jigc doc create/set-slot/set-field/add-item`
(the two-component arch-doc) → `jigc --format json task finalize document-the-gateway`. The deletion
(when any) renames the symbol away **before** finalize, so the file stays — a **symbol-absent** block,
not a file-absent one. Full logs per arm in [`evidence/`](evidence/).

## The four measured facts — each observed on the installed binary

All four observed by running `~/.local/bin/jigc` (sha256 `be72442…`) from `PATH`, **no
`JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE`**, resolving the `doc-code` sibling (`cebfaaa…`) beside it.

### Fact 1 — a TypeScript citation blocks on a vanished symbol ([`evidence/block-ts.log`](evidence/block-ts.log))

Delete component A's TypeScript symbol (`RateRouter` → `RouterRenamed`, the `.ts` file stays);
component B's Python symbol stays valid. `jigc --format json task finalize document-the-gateway`:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `src/api.ts#RateRouter` resolves to no symbol (`RateRouter` is absent from `src/api.ts`)",
"location": { "address": "arch-doc:gateway#components/edge-router/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), `docs/architecture/gateway.md` **ABSENT** (nothing
promoted). The block names **A's** item address and the dangling TS anchor target; **B's** address
(`#components/token-limiter`) never appears. The TS grammar ran over `src/api.ts` and surfaced the
absent symbol — the M17-friction "non-Rust symbol silently passes" is **closed for TypeScript**.

### Fact 2 — a Python citation blocks on a vanished symbol, symmetrically ([`evidence/block-py.log`](evidence/block-py.log))

Restore A; delete component B's Python symbol (`TokenLimiter` → `LimiterRenamed`, the `.py` file
stays); A's TypeScript symbol stays valid:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `services/limiter.py#TokenLimiter` resolves to no symbol (`TokenLimiter` is absent from `services/limiter.py`)",
"location": { "address": "arch-doc:gateway#components/token-limiter/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), nothing promoted. The block names **B's** Python item
address and dangling target; **A's** address never appears. The Python grammar ran over
`services/limiter.py` — the friction is **closed for Python**.

### Fact 3 — both anchors present → exactly one commit ([`evidence/pass.log`](evidence/pass.log))

Both the TypeScript and Python symbols present. `jigc … task finalize`:

```json
"committed": { "files": 1, "hash": "f26ac10",
  "manifest": [ { "kind": "promoted", "path": "docs/architecture/gateway.md" } ],
  "promoted": [ "docs/architecture/gateway.md" ],
  "subject": "docs(arch-doc): document the gateway" }
```

`FINALIZE_EXIT=0`, HEAD **delta +1** (exactly one commit), `docs/architecture/gateway.md` **PRESENT**,
subject `docs(arch-doc): document the gateway`. The findings carry only the two `file-state.baseline-adopt`
advisories — **zero `doc-code` findings**. Both polyglot anchors resolved through their own grammar;
the probe genuinely ran (the pass↔block contrast over the same fixture is the masking guard — a
silently-skipped enumeration would have landed the commit in the block arms too).

### Fact 4 — per-item disambiguation across two grammars (Facts 1 + 2 over the same fixture)

The single lever proving the probe dispatched each anchor to its **own** language: delete component
A's TypeScript symbol and **A alone** blocks while B's Python anchor stays silent (Fact 1, naming
`#components/edge-router/implemented-by`, never `#components/token-limiter`); delete component B's
Python symbol and **B alone** blocks while A's TypeScript anchor stays silent (Fact 2, naming
`#components/token-limiter/implemented-by`, never `#components/edge-router`). Each item's anchor
resolved against its own authored value through its own grammar, never a clobbered shared one —
per-item disambiguation holds **across two languages**.

### The sibling probe resolved with the larger grammar set

No arm surfaced a `pack-probe-integrity` meta-finding — the installed `jigc` found a runnable
`doc-code` sibling beside itself (`~/.local/bin/doc-code`) through the production default
(`current_exe().parent()/doc-code`, no override) and ran the **six-grammar** probe. The TS grammar
resolved/surfaced a `src/api.ts` symbol and the Python grammar a `services/limiter.py` symbol within
the **13.4 MB release `jigc` / 7.9 MB `doc-code`** size budget.

## Bounds (carried in honestly)

- **n = 3 arms (block-TS, block-Py, pass)** over one fixture — a done-bar shape check, not a
  distribution. The reds + the byte-stable round-trip are binary-proven by `flow29_acceptance.rs`
  against this same binary; this artifact is the *observed-on-the-installed-binary* shadow.
- **Owner-artifact recording**, not a CLI-emitted metric — the F7 recorded-alongside posture. Only
  the finalize exit/promotion is the binary's own yes/no; the per-item-disambiguation reading is the
  owner's inspection of the rendered `location.address`.
- **Six grammars, not "all languages"** — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP +
  bash (six tree-sitter crates / seven language variants). An un-grammared `#symbol` still emits the
  **unsupported-language advisory** (M27 Inc-3), not a
  silent pass and not a block; that path is binary-proven in the engine/probe tests, not re-measured
  here.
- **The scratch repos + `$HOME`s under `/tmp` are detached and discarded**; no developer repo was
  mutated. jigc's own source was not modified.

## Verdict — done-bar met

The `doc-code` differentiator turns on for a **non-Rust** target, **measured on the actually-pinned,
`cargo install`-built binary** (`jigc` `be72442…`, `doc-code` `cebfaaa…`, both identical across
`~/.local/bin` + `~/.cargo/bin` and matching a fresh build at HEAD `a261708`), resolving the probe
through the **production sibling path** with **no env overrides**. A TypeScript citation
(`src/api.ts#RateRouter`) and a Python citation (`services/limiter.py#TokenLimiter`) **genuinely
validate against reality**: each **blocks** finalize (exit 3, HEAD unchanged, nothing promoted) on its
vanished symbol naming **its own** item address and never the other's (per-item disambiguation across
two grammars), and the **both-present** arm lands **exactly one** `docs(arch-doc):` commit, promotes
to `docs/architecture/gateway.md`, with **zero** `doc-code` findings (the pass↔block masking guard
holds). The sibling probe resolved with the six-grammar set within a 13.4 MB / 7.9 MB budget; the
size guard now trips at 90 MB. The M17-friction **"doc-code non-Rust silent symbol pass" is closed in
the wild** for TypeScript and Python. **The M27 multi-language done-bar is met.**
