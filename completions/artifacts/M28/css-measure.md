# M28 — CSS doc↔code measure (the addressable-unit keystone, in the wild)

**Run 2026-06-20.** The M28 done-bar mandated by [worked-examples.md](../../../design/worked-examples.md)
→ flow 30, [validation.md](../../../design/validation.md) → Multi-language resolution,
[DECISIONS.md](../../../DECISIONS.md) → 2026-06-20 M28 planning (forks F4/F5 + the rebuild + re-pin),
[measurement.md](../../../design/measurement.md) (recorded-alongside posture), and
[roadmap.md](../../../implementation/roadmap.md) → M28 Increment 2 (T4). M27 (flow 29) generalized
the `doc-code` probe Rust→six AST grammars and proved a **TypeScript** and a **Python** citation
resolve against reality. M28 cashes in the **CSS addressable-unit keystone** (HD1): a CSS *selector*
is **not** an AST named item in the `path#symbol` sense — it has no `.name` field, so it needed a
distinct **extractor** (`class_selector` → `class_name`), per-grammar dispatch, the seventh grammar.
This artifact records the **three measured facts of flow 30**, each **observed by running the
ACTUALLY-PINNED `cargo install`-built binary** (NOT `cargo test`) over a CSS-bearing scratch repo with
**no environment overrides** — the production probe-resolution path a real install hits, now with the
CSS grammar live.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `1bd7b4074404488f82e7189d1e3b0bcc3e05b987e0c7dfb70df783fc3de9244d` |
| `doc-code` probe sha256 | `527a3f8ab1f0ccb794c33bab9b51b2b68e7b20473ae0bdee70a1fc07a629138a` |
| HEAD commit | Built at `0f45748` (M28 Inc-2 T3 — the seven-grammar/CSS size-guard-comment fix, the last code-bearing commit of the increment; T1 `902d05b` shipped `flow30_acceptance.rs`, T2 `4408e4e`/`a261708`-era folded the HD1 model into `validation.md`). This T4 artifact commit is **docs-only** (this file + `evidence/`), so the pinned binary stays **byte-identical to a fresh `cargo install` at final HEAD**. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the **seven-grammar** `doc-code` probe — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP + bash + **CSS**, seven tree-sitter language crates — and embeds it via `OUT_DIR/doc-code`); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` placed `jigc` at `~/.cargo/bin/jigc`; it was copied to `~/.local/bin/jigc`; a `jigc setup` run from **each** bin dir extracted that dir's `doc-code` sibling from the embedded copy (the heal/upgrade extract overwrote the M27 `746357a…` sibling with the new `527a3f8…`). **All four sha256 identical** — `jigc` identical across both dirs (`1bd7b40…`), `doc-code` identical across both dirs (`527a3f8…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The relocated `jigc` is **byte-identical** to the freshly-`cargo install`-built `target/release/jigc` (same `1bd7b40…`); the extracted `doc-code` siblings are byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` (`527a3f8…`) — i.e. the pinned binaries match a fresh `cargo install` at this HEAD. |
| Size | release `jigc` **13.4 MB** (13 458 616 B), `doc-code` **8.2 MB** (8 284 112 B) — up from M27's 13.36 MB / 8.18 MB by the CSS grammar. The release pair is small; the size guard's **90 MB** ceiling ([`crates/cli/tests/cargo_install_probe.rs`](../../../crates/cli/tests/cargo_install_probe.rs)) protects the **debug** `CARGO_BIN_EXE_jigc` (**71.45 MB**, 74 921 440 B, embedding the ~24.7 MB debug seven-grammar probe) — CSS landed as the seventh grammar and stayed well under (≈1.26× headroom), so an **eighth** grammar or a re-swept probe `target/` embed is what would trip it (the now-corrected T3 comment). |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (**no** `JIGC_PACK_DIR`) and the `doc-code` probe resolved **as the sibling beside the installed binary** (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s both) — the production probe-resolution path (the M20 embed/sibling), exercised live with the M28 CSS grammar. |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — one detached CSS-bearing scratch repo + isolated `$HOME` per arm; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23/M24/M25/M26/M27):
this is an **owner-artifact recording** (the F4/F5 *recorded-alongside* measure run on the
actually-pinned binary), **not** a TDD red→green test — its done-criterion is this committed artifact
plus the clean end-to-end runs on the installed binary. The blocking reds, the per-item
disambiguation, and the pass↔block masking guard are independently **binary-proven** by
[`crates/cli/tests/flow30_acceptance.rs`](../../../crates/cli/tests/flow30_acceptance.rs) (Inc-2 T1)
against this same binary's source. This artifact's job is to show those facts *on the binary a real
install actually resolves*, with the production sibling probe and no overrides.

## Corpus — a CSS-bearing scratch repo (mirrors flow 30 / `flow30_acceptance.rs`)

A real `git init` repo carrying a **single stylesheet** with **two class rules**, an `arch-doc` with
**two** components, each `implemented-by` anchored at a **CSS class** in the *same* `styles.css`, plus
a header `cites → adr` over a **committed-first** decision:

| | component A | component B |
|---|---|---|
| source file | `styles.css` | `styles.css` (the **same** file) |
| present selector | `.card { border: 1px solid black; }` (a `class_selector` → `class_name` `card`) | `.title { font-weight: bold; }` (a `class_selector` → `class_name` `title`) |
| anchor (`implemented-by`) | `styles.css#card` | `styles.css#title` |
| item address | `arch-doc:gateway#components/card/implemented-by` | `arch-doc:gateway#components/title/implemented-by` |
| "deleted" form (class rule dropped, file stays) | drop the `.card` rule | drop the `.title` rule |

Both anchors cite the **same file** at different selectors, so a deletion is a rewrite of the sheet
that drops one class rule and keeps the other — the file always exists, so each block is a
**selector-absent** block, never a file-absent one. That single shared file is the sharpest per-item
disambiguation fixture: the only lever that picks A from B is the per-selector CSS **extractor** over
the same `styles.css`. Each arm: `jigc setup` → commit the cited `adr:use-a-cache` →
`jigc start --workflow architecture-documentation` → `jigc doc create/set-slot/set-field/add-item`
(the two-component arch-doc) → `jigc --format json task finalize document-the-gateway`. Full logs per
arm in [`evidence/`](evidence/).

## The three measured facts — each observed on the installed binary

All three observed by running `~/.local/bin/jigc` (sha256 `1bd7b40…`) from `PATH`, **no
`JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE`**, resolving the `doc-code` sibling (`527a3f8…`) beside it.

### Fact 1 — a CSS citation blocks on a vanished selector ([`evidence/block-card.log`](evidence/block-card.log))

Drop component A's CSS class (the `.card` rule; the `styles.css` file stays, now carrying only
`.title`); component B's `.title` selector stays valid. `jigc --format json task finalize
document-the-gateway`:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `styles.css#card` resolves to no symbol (`card` is absent from `styles.css`)",
"location": { "address": "arch-doc:gateway#components/card/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), `docs/architecture/gateway.md` **ABSENT** (nothing
promoted). The block names **A's** item address and the dangling CSS anchor target; **B's** address
(`#components/title`) never appears. The CSS grammar ran over `styles.css` and surfaced the absent
selector — the M17-friction "non-AST symbol silently passes" is **closed for CSS**.

### Fact 2 — a CSS citation blocks symmetrically on the *other* selector ([`evidence/block-title.log`](evidence/block-title.log))

Restore `.card`; drop component B's CSS class (the `.title` rule; the file stays, now carrying only
`.card`); A's `.card` selector stays valid:

```json
"severity": "blocking",
"probe": "doc-code",
"check": "symbol-exists",
"code": "doc-code.symbol-exists",
"message": "anchor `styles.css#title` resolves to no symbol (`title` is absent from `styles.css`)",
"location": { "address": "arch-doc:gateway#components/title/implemented-by", "line": 1, "col": 1 }
```

`FINALIZE_EXIT=3`, HEAD unchanged (delta 0), nothing promoted. The block names **B's** item address
and dangling target; **A's** address never appears. The extractor picked **B's own** selector out of
the same shared sheet.

### Fact 3 — both selectors present → exactly one commit ([`evidence/pass.log`](evidence/pass.log))

Both `.card` and `.title` present in `styles.css`. `jigc … task finalize`:

```json
"committed": { "files": 1, "hash": "3863dcf",
  "manifest": [ { "kind": "promoted", "path": "docs/architecture/gateway.md" } ],
  "promoted": [ "docs/architecture/gateway.md" ],
  "subject": "docs(arch-doc): document the gateway" }
```

`FINALIZE_EXIT=0`, HEAD **delta +1** (exactly one commit), `docs/architecture/gateway.md` **PRESENT**,
subject `docs(arch-doc): document the gateway`. The findings carry only the two
`file-state.baseline-adopt` advisories — **zero `doc-code` findings**. Both CSS anchors resolved
through the CSS extractor over the same `styles.css`; the probe genuinely ran (the pass↔block contrast
over the same fixture is the masking guard — a silently-skipped enumeration would have landed the
commit in the block arms too).

### Fact 4 (the keystone) — per-item disambiguation over a single shared CSS file (Facts 1 + 2)

The single lever proving the extractor dispatched each anchor to its **own** selector out of the
*same* `styles.css`: drop `.card` and **A alone** blocks while B's `.title` anchor stays silent
(Fact 1, naming `#components/card/implemented-by`, never `#components/title`); drop `.title` and **B
alone** blocks while A's `.card` anchor stays silent (Fact 2, naming `#components/title/implemented-by`,
never `#components/card`). Each item's anchor resolved against its own selector through the per-grammar
CSS extractor, never a clobbered shared one — per-item disambiguation holds **over one shared
stylesheet**, the sharpest possible witness that the addressable-unit keystone (HD1) works on the
most-different language model.

### The sibling probe resolved with the seven-grammar set

No arm surfaced a `pack-probe-integrity` meta-finding — the installed `jigc` found a runnable
`doc-code` sibling beside itself (`~/.local/bin/doc-code`) through the production default
(`current_exe().parent()/doc-code`, no override) and ran the **seven-grammar** probe. The CSS grammar
resolved/surfaced a `styles.css` selector within the **13.4 MB release `jigc` / 8.2 MB `doc-code`**
size budget.

## Bounds (carried in honestly)

- **n = 3 arms (block-card, block-title, pass)** over one fixture — a done-bar shape check, not a
  distribution. The reds + the byte-stable round-trip are binary-proven by `flow30_acceptance.rs`
  against this same binary; this artifact is the *observed-on-the-installed-binary* shadow.
- **Owner-artifact recording**, not a CLI-emitted metric — the F4/F5 recorded-alongside posture. Only
  the finalize exit/promotion is the binary's own yes/no; the per-item-disambiguation reading is the
  owner's inspection of the rendered `location.address`.
- **Seven grammars, not "all languages"** — Rust + TypeScript/TSX + JavaScript/JSX + Python + PHP +
  bash + **CSS** (seven tree-sitter language crates). CSS closes the **CSS half** of the
  CSS+docker-compose deferral; YAML/HD2/HD3/HD4 and the docker-compose half stay open
  ([ideas/multi-language-doc-code.md](../../../ideas/multi-language-doc-code.md) tiered roadmap). An
  un-grammared `#symbol` still emits the **unsupported-language advisory** (M27 Inc-3), not a silent
  pass and not a block; that path is binary-proven in the engine/probe tests, not re-measured here.
- **A CSS *class* selector**, specifically — the `class_selector → class_name` extractor. Id selectors,
  type selectors, and at-rules are not separately measured here (the extractor's exact node-kind
  coverage is the probe unit tests' job).
- **The scratch repos + `$HOME`s under `/tmp` are detached and discarded**; no developer repo was
  mutated. jigc's own source was not modified.

## Verdict — done-bar met

The `doc-code` differentiator turns on for a **CSS** target — the most-different addressable-unit
model — **measured on the actually-pinned, `cargo install`-built binary** (`jigc` `1bd7b40…`,
`doc-code` `527a3f8…`, both identical across `~/.local/bin` + `~/.cargo/bin` and matching a fresh build
at HEAD), resolving the probe through the **production sibling path** with **no env overrides**. A CSS
citation (`styles.css#card` / `styles.css#title`) **genuinely validates against reality**: each
**blocks** finalize (exit 3, HEAD unchanged, nothing promoted) on its vanished selector naming **its
own** item address and never the other's (per-item disambiguation over a **single shared stylesheet**),
and the **both-present** arm lands **exactly one** `docs(arch-doc):` commit, promotes to
`docs/architecture/gateway.md`, with **zero** `doc-code` findings (the pass↔block masking guard holds).
The sibling probe resolved with the seven-grammar set within a 13.4 MB / 8.2 MB budget; the size guard
still trips at 90 MB. The **CSS half of the CSS+docker-compose deferral is closed in the wild**, and
the **HD1 addressable-unit keystone is proven on the most-different model**. **The M28 CSS done-bar is
met.**
