# M33 — the schema/format freeze gate (the frozen-v1 doctype set self-enforces)

**Run 2026-06-24 (re-pinned at HEAD `3815413`).** This artifact is the **productive-go G1
retirement record** for [roadmap.md](../../../implementation/roadmap.md) → M33 Increment 4. M33
declares the **six dev-pack doctypes** (`commit`, `adr`, `spec`, `prd`, `arch-doc`, `changelog`)
+ the schema-definition format **frozen v1** and makes the freeze **machine-checkable, not prose
discipline**: a pack-shipped manifest (`crates/cli/pack/config/schema-manifest.yaml`) enumerates
each frozen doctype's `schema-version` (v1) + `schema-hash`, and an **engine pack-load assertion**
recomputes each shipped doctype's hash and **blocks loudly** on any un-migrated shape change — the
intrinsic-floor-assertion sibling, *not* the report-only `validate` sweep
([corpus-migration.md](../../../design/corpus-migration.md) → The freeze, declared *and*
enforced; [doctype-map.md](../../../implementation/doctype-map.md) → The v1 freeze;
[worked-examples.md](../../../design/worked-examples.md) → flow 34;
[CLAUDE.md](../../../CLAUDE.md) → the frozen-v1 invariant).

This records the **three measured facts**, each **observed by running the ACTUALLY-PINNED
`cargo install`-built binary** (NOT `cargo test`) over real scratch git repos: the conformant
production path runs the **embedded** dev pack with **no** `JIGC_PACK_DIR` / `JIGC_DOC_CODE_PROBE`;
the blocked path necessarily expresses a schema-shape mutation on an on-disk dev-pack copy via
`JIGC_PACK_DIR` (the embedded pack is conformant by build — a shape change is only expressible on
a copy), exactly as the T3 acceptance (`crates/cli/tests/freeze_enforcement.rs`) does.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `5327c5336eeedc2f1ffdd8cae0d0f1921b4ac580b2dad44019679606e817482d` |
| `doc-code` probe sha256 | `b0a413a1cb8fa7fff6ceb85e3fdebc304508c42d76c57447112f4f208ea74304` |
| HEAD commit | Built at `3815413` (`docs(freeze): declare dev-pack-6 frozen v1 + add freeze-discipline invariant` — the M33 Inc-4 T4 doc fold-back, the last commit before this re-pin). The prior pins were **stale and divergent** (`jigc` differed across the two bin dirs — `dc5ae93…` in `~/.cargo/bin` vs `5c4a15a6…` in `~/.local/bin` — and `doc-code` too: `530dd89…` vs `b0a413a1…`); this re-pin rebuilds the M33 binary and re-copies both siblings into both dirs so all four match a fresh build. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the eight-grammar `doc-code` probe and embeds it via `OUT_DIR/doc-code` for the `jigc setup` extract path, plus the embedded dev pack — incl. the M33 `config/schema-manifest.yaml`); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` placed `jigc` at `~/.cargo/bin/jigc` (byte-identical to the `target/release/jigc` it co-produced); `jigc` was copied to `~/.local/bin/jigc` and the `doc-code` sibling (the `build.rs` `OUT_DIR/doc-code`, byte-identical to `target/release/doc-code`) into both dirs. **All four sha256 identical** — `jigc` identical across both dirs (`5327c533…`), `doc-code` identical across both dirs (`b0a413a1…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The pinned `jigc` IS the output of this `cargo install` at `3815413` — byte-identical to the `target/release/jigc` it co-produced (same `5327c533…`); the `doc-code` sibling is byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` and to `target/release/doc-code` (`b0a413a1…`). (Release `jigc` is **not** stripped, so a *separate* later rebuild yields a different sha — the pin is the bytes this install wrote, not a reproducibility claim.) |
| Size | release `jigc` **13.29 MiB** (13 933 728 B), `doc-code` **8.10 MiB** (8 489 160 B). |
| Invocation | `jigc` from `PATH`, with the **embedded** dev pack (**no** `JIGC_PACK_DIR`, except the blocked arm B which needs an on-disk copy to mutate) and the `doc-code` probe resolved **as the sibling beside the installed binary** (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s both) — the production probe-resolution path. |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — a self-cleaning scratch git repo + isolated `$HOME` per arm; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23–M32): this
is an **owner-artifact recording** (the recorded-alongside measure run on the actually-pinned
binary), **not** a TDD red→green test — its done-criterion is this committed artifact plus the
clean end-to-end runs on the installed binary. The freeze gate's behaviour is independently
**binary-proven** by [`crates/cli/tests/freeze_enforcement.rs`](../../../crates/cli/tests/freeze_enforcement.rs)
(`schema_shape_drift_without_manifest_bump_is_blocked`, `unmutated_pack_copy_composes_clean`,
`manifest_less_pack_is_unaffected`) and by the build-time sibling
`pack::tests::shipped_schema_manifest_matches_the_frozen_doctype_set` against this binary's
source. This artifact's job is to show those facts *on the binary a real install actually
resolves*, with the production sibling probe and no overrides.

## The three measured facts

### Fact 1 — the manifest enumerates exactly the six frozen-v1 doctypes

The embedded freeze manifest enumerates `commit`, `adr`, `spec`, `prd`, `arch-doc`, `changelog`,
**each at `schema-version: 1`** — the productive adopter's whole persisted corpus
([`evidence/manifest-enumeration.log`](evidence/manifest-enumeration.log); the embedded source is
`crates/cli/pack/config/schema-manifest.yaml`, ridden into the binary via `include_dir!`). The
build-time gate asserts **exactly** these six names at version 1 and drives
`engine::manifest::check` over the production loader, so the enumeration is locked, not
incidental. The methodology self-host doctypes are deliberately **out** of the v1 freeze (M33
Inc-1 scoped their byte-stability out); the manifest mechanism is general (an absent manifest =
unchecked — fact 3 below).

### Fact 2 — a shape change with no version bump is blocked at pack-load

Arm B copies the dev pack to a `JIGC_PACK_DIR`, drifts the `adr` schema's **shape**
(`location: decisions/` → `location: adr-records/`) **without bumping its `schema-version`**, and
runs the pinned `jigc start --workflow single-task` from inside a real repo. It exits **non-zero
(1)** before composing, and stderr names the breach
([`evidence/blocked.log`](evidence/blocked.log)):

```
pack-load freeze check failed: doctype `adr`: schema-hash mismatch (manifest declares `4882af1b…`, recomputed `64d9c428…`)
```

The un-migrated, version-unbumped schema-shape change the freeze forbids is **blocked loudly at
pack-load** — the productive compose front door, not the report-only `validate` sweep. The fix is
to bump the version + ship the M34 corpus migration (or revert).

### Fact 3 — a conformant pack composes clean; a manifest-less pack is inert

Arm A is the **production path**: the pinned `jigc start` over the **embedded** dev pack with **no
`JIGC_PACK_DIR`**. The freeze gate runs over the embedded six-doctype manifest, every shipped hash
matches, and the binary **composes the workflow, exit 0**
([`evidence/conformant.log`](evidence/conformant.log)). Arm C is the omitting context: the **same**
`adr` shape drift as arm B, but with `config/schema-manifest.yaml` **dropped** — the pack is
**unchecked**, so the identical drift **composes clean, exit 0**
([`evidence/manifest-less.log`](evidence/manifest-less.log)). The manifest *is* the gate: a pack
that declares nothing frozen is never gated (never errors), while the pack that declares the six
is held to them.

| Arm | Pack | Mutation | Manifest | Exit | Observed |
|---|---|---|---|---|---|
| A · conformant | embedded (no `JIGC_PACK_DIR`) | none | present (6 doctypes) | **0** | composes |
| B · blocked | on-disk copy (`JIGC_PACK_DIR`) | `adr` shape drift, no version bump | present | **1** | `schema-hash mismatch` on `adr` |
| C · manifest-less | on-disk copy (`JIGC_PACK_DIR`) | same `adr` drift | dropped | **0** | composes (unchecked) |

## G1 retired

The doctype set + schema-definition format are declared **frozen v1** with a machine-checkable
manifest; a schema-shape change that bumps no version is **blocked at pack-load** on the shipped
binary. Productive-readiness gate **G1** is retired. (The *means* to satisfy a blocked change — the
corpus migration that bumps the version and rewrites the corpus — is M34.) The milestone-completion
audit runs separately after this increment.
