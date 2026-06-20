# M29 — YAML doc↔code measure (the first language to ride the HD1 seam, in the wild)

**Run 2026-06-20.** The M29 done-bar mandated by [worked-examples.md](../../../design/worked-examples.md)
→ flow 31, [validation.md](../../../design/validation.md) → Multi-language resolution,
[DECISIONS.md](../../../DECISIONS.md) → 2026-06-20 M29 planning (forks F4/F5/F8 + the rebuild + re-pin),
[measurement.md](../../../design/measurement.md) (recorded-alongside posture), and
[roadmap.md](../../../implementation/roadmap.md) → M29 Increment 2 (T4). M28 (flow 30) cashed in the
**CSS addressable-unit keystone** (HD1): a CSS *selector* is not an AST named item in the `path#symbol`
sense — it has no `.name` field, so it needed a distinct **extractor** and the per-grammar dispatch, the
seventh grammar. M29 proves that keystone **generalizes cheaply**: a docker-compose **service key** is a
YAML *mapping key* (`block_mapping_pair` / `flow_pair` → the `key` field, quotes stripped), the **first
language to ride the seam CSS forced** (not build it) — it reused the per-grammar dispatch with no
engine/CLI/pack-schema change, the eighth grammar. This artifact records the **three measured facts of
flow 31**, each **observed by running the ACTUALLY-PINNED `cargo install`-built binary** (NOT
`cargo test`) over a YAML/compose-bearing scratch repo with **no environment overrides** — the
production probe-resolution path a real install hits, now with the YAML grammar live.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `c188fd0761ac6889c7734b0364dafe6f6bda0969c273abff5f9808e2fa311f81` |
| `doc-code` probe sha256 | `d99fc650e27ce2d46a469585513c0cabf84477af8b95241034a8a15bf9f01e2e` |
| HEAD commit | Built at `d7ca6e8` (M29 Inc-2 T3 — the eight-grammar/YAML size-guard-comment fix, the last code-bearing commit of the increment; T1 `39e907c` shipped `flow31_acceptance.rs`, T2 `17c8051` folded the YAML addressable-unit model into `validation.md`; the grammar itself landed in Inc-1 `9cae5dd`). This T4 artifact commit is **docs-only** (this file + `evidence/`), so the pinned binary stays **byte-identical to a fresh `cargo install` at final HEAD**. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the **eight-grammar** `doc-code` probe — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP + bash + CSS + **YAML**, eight tree-sitter language crates — and embeds it via `OUT_DIR/doc-code`); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` placed `jigc` at `~/.cargo/bin/jigc`; it was copied to `~/.local/bin/jigc`; a `jigc setup` run from **each** bin dir extracted that dir's `doc-code` sibling from the embedded copy (the heal/upgrade extract overwrote the M28 `527a3f8…` sibling with the new `d99fc65…`). **All four sha256 identical** — `jigc` identical across both dirs (`c188fd0…`), `doc-code` identical across both dirs (`d99fc65…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The relocated `jigc` is **byte-identical** to the freshly-`cargo install`-built `target/release/jigc` (same `c188fd0…`); the extracted `doc-code` siblings are byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` (`d99fc65…`) — i.e. the pinned binaries match a fresh `cargo install` at this HEAD. |
| Size | release `jigc` **13.66 MB** (13 660 856 B), `doc-code` **8.49 MB** (8 486 360 B) — up from M28's 13.46 MB / 8.28 MB by the YAML grammar. The release pair is small; the size guard's **90 MiB** ceiling ([`crates/cli/tests/cargo_install_probe.rs`](../../../crates/cli/tests/cargo_install_probe.rs)) protects the **debug** `CARGO_BIN_EXE_jigc` (**71.71 MiB**, 75 192 608 B, embedding the ~24.98 MB debug eight-grammar probe) — YAML landed as the eighth grammar (~310 KB) and stayed well under (≈1.25× headroom), so a **ninth** grammar or a re-swept probe `target/` embed is what would trip it (the now-corrected T3 comment). |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (**no** `JIGC_PACK_DIR`) and the `doc-code` probe resolved **as the sibling beside the installed binary** (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s both) — the production probe-resolution path (the M20 embed/sibling), exercised live with the M29 YAML grammar. |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — one detached YAML/compose-bearing scratch repo + isolated `$HOME` per arm; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23/M24/M25/M26/M27/M28):
this is an **owner-artifact recording** (the F4/F5 *recorded-alongside* measure run on the
actually-pinned binary), **not** a TDD red→green test — its done-criterion is this committed artifact
plus the clean end-to-end runs on the installed binary. The blocking reds, the per-item
disambiguation, and the pass↔block masking guard are independently **binary-proven** by
[`crates/cli/tests/flow31_acceptance.rs`](../../../crates/cli/tests/flow31_acceptance.rs) (Inc-2 T1)
against this same binary's source. This artifact's job is to show those facts *on the binary a real
install actually resolves*, with the production sibling probe and no overrides. The literal live
PHP/TS/Vite + docker-compose run is owner-driven hardening afterward (F8, the M28 posture), not a
completion gate.

## Corpus — a YAML/compose-bearing scratch repo (mirrors flow 31 / `flow31_acceptance.rs`)

A real `git init` repo carrying a **single `compose.yaml`** with **two service keys** under
`services:`, an `arch-doc` with **two** components, each `implemented-by` anchored at a **compose
service key** in the *same* `compose.yaml`, plus a header `cites → adr` over a **committed-first**
decision:

| | component A | component B |
|---|---|---|
| source file | `compose.yaml` | `compose.yaml` (the **same** file) |
| present service key | `web:` (`image: nginx`, a `block_mapping_pair` key `web`) | `db:` (`image: postgres`, a `block_mapping_pair` key `db`) |
| anchor (`implemented-by`) | `compose.yaml#web` | `compose.yaml#db` |
| item slug | `web` | `database` (from title "Database" — **differs from** the anchor symbol `db`) |
| item address | `arch-doc:gateway#components/web/implemented-by` | `arch-doc:gateway#components/database/implemented-by` |
| "deleted" form (service block dropped, file stays) | drop the `web:` block | drop the `db:` block |

Both anchors cite the **same file** at different service keys, so a deletion is a rewrite of the compose
file that drops one service block and keeps the other — the file always exists, so each block is a
**key-absent** block, never a file-absent one. Component B's item **slug** (`database`) deliberately
differs from its **anchor symbol** (`db`), so the block is proven keyed on the *item address*, not on a
name coincidence with the symbol. That single shared file is the sharpest per-item disambiguation
fixture: the only lever that picks A from B is the per-key YAML **extractor** over the same
`compose.yaml`. Each arm: `jigc setup` → commit the cited `adr:use-a-cache` →
`jigc start --workflow architecture-documentation` → `jigc doc create/set-slot/set-field/add-item`
(the two-component arch-doc) → `jigc --format json task finalize document-the-gateway`. Full logs per
arm in [`evidence/`](evidence/).

## The three measured facts — each observed on the installed binary

All three observed by running `~/.local/bin/jigc` (sha256 `c188fd0…`) from `PATH`, **no
`JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE`**, resolving the `doc-code` sibling (`d99fc65…`) beside it.

### Fact 1 — a YAML citation blocks on a vanished service key ([`evidence/block-web.log`](evidence/block-web.log))

Drop component A's service block (the `web:` block; the `compose.yaml` file stays, now carrying only
`db:`); component B's `db` key stays valid. `jigc --format json task finalize document-the-gateway`:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `compose.yaml#web` resolves to no symbol (`web` is absent from `compose.yaml`)",
"location": { "address": "arch-doc:gateway#components/web/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), `docs/architecture/gateway.md` **ABSENT** (nothing
promoted). The block names **A's** item address and the dangling YAML anchor target; **B's** address
(`#components/database`) never appears. The YAML grammar ran over `compose.yaml` and surfaced the absent
service key — the M17-friction "non-AST symbol silently passes" is **closed for YAML**.

### Fact 2 — a YAML citation blocks symmetrically on the *other* service key ([`evidence/block-db.log`](evidence/block-db.log))

Restore `web:`; drop component B's service block (the `db:` block; the file stays, now carrying only
`web:`); A's `web` key stays valid:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `compose.yaml#db` resolves to no symbol (`db` is absent from `compose.yaml`)",
"location": { "address": "arch-doc:gateway#components/database/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), nothing promoted. The block names **B's** item address
(slug `database`, the item with the now-dangling `compose.yaml#db` anchor) and dangling target; **A's**
address never appears. The extractor picked **B's own** service key out of the same shared compose file
— and the block keyed on the **item address** (`database`), not on the anchor symbol (`db`), proving the
disambiguation is address-keyed, not a name coincidence.

### Fact 3 — both service keys present → exactly one commit ([`evidence/pass.log`](evidence/pass.log))

Both `web:` and `db:` present in `compose.yaml`. `jigc … task finalize`:

```json
"committed": { "files": 1, "hash": "925091d",
  "manifest": [ { "kind": "promoted", "path": "docs/architecture/gateway.md" } ],
  "promoted": [ "docs/architecture/gateway.md" ],
  "subject": "docs(arch-doc): document the gateway" }
```

`FINALIZE_EXIT=0`, HEAD **delta +1** (exactly one commit), `docs/architecture/gateway.md` **PRESENT**,
subject `docs(arch-doc): document the gateway`. The findings carry only the two
`file-state.baseline-adopt` advisories — **zero `doc-code` findings**. Both YAML anchors resolved
through the YAML extractor over the same `compose.yaml`; the probe genuinely ran (the pass↔block contrast
over the same fixture is the masking guard — a silently-skipped enumeration would have landed the
commit in the block arms too).

### Fact 4 (the keystone) — per-item disambiguation over a single shared YAML file (Facts 1 + 2)

The single lever proving the extractor dispatched each anchor to its **own** service key out of the
*same* `compose.yaml`: drop `web:` and **A alone** blocks while B's `db` anchor stays silent (Fact 1,
naming `#components/web/implemented-by`, never `#components/database`); drop `db:` and **B alone** blocks
while A's `web` anchor stays silent (Fact 2, naming `#components/database/implemented-by`, never
`#components/web`). Each item's anchor resolved against its own service key through the per-grammar YAML
extractor, never a clobbered shared one — per-item disambiguation holds **over one shared compose file**,
and because B's slug (`database`) differs from its symbol (`db`), the disambiguation is proven
**address-keyed**, the sharpest possible witness that the addressable-unit keystone (HD1) generalizes to
the first language riding the seam.

### The sibling probe resolved with the eight-grammar set

No arm surfaced a `pack-probe-integrity` meta-finding — the installed `jigc` found a runnable
`doc-code` sibling beside itself (`~/.local/bin/doc-code`) through the production default
(`current_exe().parent()/doc-code`, no override) and ran the **eight-grammar** probe. The YAML grammar
resolved/surfaced a `compose.yaml` service key within the **13.66 MB release `jigc` / 8.49 MB
`doc-code`** size budget.

## Bounds (carried in honestly)

- **n = 3 arms (block-web, block-db, pass)** over one fixture — a done-bar shape check, not a
  distribution. The reds + the byte-stable round-trip are binary-proven by `flow31_acceptance.rs`
  against this same binary; this artifact is the *observed-on-the-installed-binary* shadow.
- **Owner-artifact recording**, not a CLI-emitted metric — the F4/F5 recorded-alongside posture. Only
  the finalize exit/promotion is the binary's own yes/no; the per-item-disambiguation reading is the
  owner's inspection of the rendered `location.address`.
- **Eight grammars, not "all languages"** — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP +
  bash + CSS + **YAML** (eight tree-sitter language crates). YAML closes the **docker-compose/YAML half**
  of the CSS+docker-compose deferral (CSS closed the CSS half at M28); JSON/TOML/HD2/HD3/HD4 stay open
  ([ideas/multi-language-doc-code.md](../../../ideas/multi-language-doc-code.md) tiered roadmap). An
  un-grammared `#symbol` still emits the **unsupported-language advisory** (M27 Inc-3), not a silent
  pass and not a block; that path is binary-proven in the engine/probe tests, not re-measured here.
- **A YAML *mapping key* (a compose service key)**, specifically — the `block_mapping_pair → key`
  extractor. Sequence items, anchors/aliases, and deeper-nested keys are not separately measured here
  (the extractor's exact node-kind coverage is the probe unit tests' job).
- **A `compose.yaml` shape**, not a literal live docker-compose stack — the YAML *citation* validates
  against the *file* (the determinism boundary: the CLI resolves the symbol in the YAML, it does not run
  Docker). The literal PHP/TS/Vite + docker-compose run is owner-driven hardening afterward (F8 / the
  M28 posture), not a completion gate.
- **The scratch repos + `$HOME`s under `/tmp` are detached and discarded**; no developer repo was
  mutated. jigc's own source was not modified.

## Verdict — done-bar met

The `doc-code` differentiator turns on for a **YAML** target — the first language to ride the HD1 seam —
**measured on the actually-pinned, `cargo install`-built binary** (`jigc` `c188fd0…`, `doc-code`
`d99fc65…`, both identical across `~/.local/bin` + `~/.cargo/bin` and matching a fresh build at HEAD),
resolving the probe through the **production sibling path** with **no env overrides**. A YAML citation
(`compose.yaml#web` / `compose.yaml#db`) **genuinely validates against reality**: each **blocks**
finalize (exit 3, HEAD unchanged, nothing promoted) on its vanished service key naming **its own** item
address and never the other's (per-item disambiguation over a **single shared compose file**, proven
address-keyed by B's slug≠symbol), and the **both-present** arm lands **exactly one** `docs(arch-doc):`
commit, promotes to `docs/architecture/gateway.md`, with **zero** `doc-code` findings (the pass↔block
masking guard holds). The sibling probe resolved with the eight-grammar set within a 13.66 MB / 8.49 MB
budget; the size guard still trips at 90 MiB. The **docker-compose/YAML half of the CSS+docker-compose
deferral is closed in the wild**, and the **HD1 addressable-unit keystone is proven to generalize
cheaply** — the first language to ride the seam, not build it. **The M29 YAML done-bar is met.**
