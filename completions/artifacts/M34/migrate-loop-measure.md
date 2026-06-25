# M34 — the corpus-migration detect→block→migrate loop (the dogfood, end-to-end)

**Run 2026-06-25 (pinned at HEAD `c01eea7`).** This artifact is the **productive-go G2
retirement record** for [roadmap.md](../../../implementation/roadmap.md) → M34 Increment 3. M34
adds the **per-doc `schema-version` stamp** (schema-declared, engine-valued) to the frozen-v1
persisted doctypes, the **version-aware migrate-vs-corrupt routing** in the store-scope conformance
detector, and the user-facing **`jigc migrate-corpus`** verb that stamps a stranded corpus
byte-stable through the **real `added-optional-field` transform** — closing the loop the M33 freeze
gate (G1) only declared the *means* for: a committed corpus that predates a shape change is
**detected**, **blocked** (a store-scope conformance break), and **migrated** to the current schema
version ([corpus-migration.md](../../../design/corpus-migration.md) → The stamp / Acceptance flows;
[storage.md](../../../design/storage.md) + [document-type-schema.md](../../../design/document-type-schema.md)
→ the field; [validation.md](../../../design/validation.md) → version-aware routing;
[worked-examples.md](../../../design/worked-examples.md) → flow 35).

This records the **measured facts of the full loop**, each **observed by running the
ACTUALLY-PINNED `jigc` binary** (NOT `cargo test`) over a real dev-pack corpus in a scratch git
repo, with the **embedded** dev pack (no `JIGC_PACK_DIR`) and the `doc-code` probe resolved as the
**sibling beside the installed binary** (no `JIGC_DOC_CODE_PROBE`) — the production path.

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `e2b482045698a52de7a76c75c821be1682f9a421b83139858f242621efd8b360` |
| `doc-code` probe sha256 | `b0a413a1cb8fa7fff6ceb85e3fdebc304508c42d76c57447112f4f208ea74304` (unchanged — the probe was untouched across M34; the milestone added no grammar/probe code) |
| HEAD commit | **`c01eea7`** (`feat(cli): add the jigc migrate-corpus verb + the live v0->v1 stamp dogfood` — the M34 Inc-3 T4 verb). T5 adds only this `completions/artifacts/M34/` record on top, so the shipped binary equals the increment's code HEAD. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the eight-grammar `doc-code` probe and embeds it via `OUT_DIR/doc-code` for the `jigc setup` extract path, plus the embedded dev pack incl. `config/schema-manifest.yaml` re-frozen at the M34 stamp); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/`. `cargo install` placed `jigc` at `~/.cargo/bin/jigc` (byte-identical to the `target/release/jigc` it co-produced); both `jigc` and its `doc-code` sibling (the `build.rs` `OUT_DIR/doc-code`, byte-identical to `target/release/doc-code`) were copied into both dirs. **All four sha256 identical** — `jigc` identical across both dirs (`e2b48204…`), `doc-code` identical across both dirs (`b0a413a1…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The pinned `jigc` IS the output of this `cargo install` at `c01eea7` — byte-identical to the `target/release/jigc` it co-produced (same `e2b48204…`); the `doc-code` sibling is byte-identical to the `build.rs`-embedded `OUT_DIR/doc-code` and to `target/release/doc-code` (`b0a413a1…`). (Release `jigc` is **not** stripped, so a *separate* later rebuild yields a different sha — the pin is the bytes this install wrote, not a reproducibility claim.) |
| Size | release `jigc` **13.41 MiB** (14 060 376 B), `doc-code` **8.10 MiB** (8 489 160 B). |
| Invocation | `jigc` from `PATH`, embedded dev pack (**no** `JIGC_PACK_DIR`), `doc-code` resolved as the installed sibling (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s it) — the production probe-resolution path. |
| Driver | [`evidence/drive.sh`](evidence/drive.sh) — a self-cleaning scratch git repo + isolated `$HOME` per corpus; logs in [`evidence/`](evidence/). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23–M33): this is
an **owner-artifact recording** (the recorded-alongside measure run on the actually-pinned binary),
**not** a TDD red→green test — its done-criterion is this committed artifact plus the clean
end-to-end loop on the installed binary. The loop's behaviour is independently **binary-proven** by
[`crates/cli/tests/corpus_migration.rs`](../../../crates/cli/tests/corpus_migration.rs)
(`migrate_corpus_stamps_the_v0_dogfood_then_revalidates_clean`,
`migrate_corpus_is_a_no_op_on_an_already_current_corpus`) and the store-scope detector's
version-aware routing by `crates/engine`'s conformance tests. This artifact's job is to show those
facts *on the binary a real install actually resolves*, with the production sibling probe and no
overrides.

## The measured loop — one v0 corpus, detect → migrate → re-detect

A committed `adr` at its canonical `docs/decisions/alpha-decision.md`, conformant under the pack
`adr` schema but **carrying no `schema-version` stamp** — the v0 state every dev-pack corpus
written before M34 is in ([`evidence/adr-before.log`](evidence/adr-before.log)).

### Step 1 — DETECT: the stranded v0 doc is blocked + routed `migrate`

`jigc validate` over the v0 corpus reports a **`schema-conformance` break** on the stamp field,
explicitly routed **`migrate`** (below the current schema-version 1), report-only at store scope
(**exit 0**) ([`evidence/detect.log`](evidence/detect.log)):

```
blocking · schema-conformance.field-value-conformant — field `schema-version` is absent; the committed doc predates the schema-version stamp (below the current schema-version 1)
  route: migrate — `docs/decisions/alpha-decision.md` carries no schema-version stamp; run the corpus migration to stamp and upgrade it to schema-version 1
```

This is the **version-aware routing** (Inc-3 T3): `below-version ⇒ migrate` (this case) vs
`at-version + non-conformant ⇒ corrupt`. The detector tells the operator the stranded doc is
*upgradable*, not corrupt. (The `file-state.un-baselined` advisory beside it is orthogonal — the
ADR was committed by hand into the scratch repo, never CLI-baselined; it is unaffected by the
migration and stays advisory throughout.)

### Step 2 — MIGRATE: the verb stamps v1 byte-stable via the real transform

`jigc migrate-corpus` walks the committed store, builds each frozen persisted doctype's v0→v1 job
from the pack, and per-doc applies the **real `added-optional-field` schema-diff transform** —
splicing `schema-version: 1` into the existing header — writing back only on a clean conformance
gate against v1 (**exit 0**) ([`evidence/migrate.log`](evidence/migrate.log)):

```
corpus migration: 1 migrated, 0 already current, 0 blocked
  migrated   docs/decisions/alpha-decision.md
```

The stamp landed **byte-stable**: the prior header fields (`status`, `date`) and the full body
prose are preserved verbatim; only the stamp line is added
([`evidence/adr-after.log`](evidence/adr-after.log)). This is the **live `add-field` e2e** — the
schema-diff classifier run over the genuine corpus, the one true pending shape change of the M34
branch-split, not a synthetic fixture.

### Step 3 — RE-DETECT: the migrated corpus is conformant + stamped v1

`jigc validate` re-run over the now-stamped corpus surfaces **no `schema-conformance` finding** and
**no `migrate` route** (**exit 0**) ([`evidence/revalidate.log`](evidence/revalidate.log)). The
only remaining finding is the unrelated `file-state.un-baselined` advisory. The loop is closed: the
corpus that was stranded below v1 now carries its authored-against version.

### Guard — the no-op on an already-current corpus

A separate corpus whose `adr` already carries `schema-version: 1` is a clean **no-op**: `jigc
migrate-corpus` reports `0 migrated, 1 already current, 0 blocked` and leaves the doc
**byte-identical** (sha256 unchanged before/after) ([`evidence/noop.log`](evidence/noop.log)). A
current doc is never rewritten — the false-positive guard the detector's version-awareness buys.

| Step | Corpus | `jigc` command | Exit | Observed |
|---|---|---|---|---|
| 1 · detect | v0 ADR (unstamped) | `validate` | **0** | `schema-conformance` break, `route: migrate` |
| 2 · migrate | v0 ADR (unstamped) | `migrate-corpus` | **0** | `1 migrated`, `schema-version: 1` spliced byte-stable |
| 3 · re-detect | v1 ADR (stamped) | `validate` | **0** | no `schema-conformance` finding, no `migrate` route |
| 4 · no-op | v1 ADR (stamped) | `migrate-corpus` | **0** | `1 already current`, doc byte-identical |

## G2 retired

The full **detect→block→migrate** loop runs end-to-end on the shipped binary: a corpus stranded
below the current schema version is detected and routed `migrate`, the verb stamps it v1 byte-stable
through the real added-optional-field transform, and a re-detect finds it conformant. Every
persisted frozen doctype now carries its authored-against `schema-version`. Productive-readiness
gate **G2** is retired. (G1 — the freeze gate that *blocks* an un-migrated shape change — was
retired in [M33](../M33/freeze-gate-measure.md); G2 supplies the *migration* that satisfies such a
block.) The milestone-completion audit runs separately after this increment.
