# M34 — the v1→v2 structural corpus migration (the real reshape, end-to-end)

**Run 2026-06-25 (pinned at HEAD `cf2b340`).** This artifact is the **productive-go G2
retirement record** for [roadmap.md](../../../implementation/roadmap.md) → M34 Increment 4 — the
increment that retires G2 on a **real v1→v2 structural** migration, not only the v0→v1 stamp the
Inc-3 dogfood proved ([migrate-loop-measure.md](migrate-loop-measure.md), flow 35). Increment 4
supplies the one thing a genuine structural change needs and the current schema cannot give the
verb: the **actual prior shape**. The pack stores prior shapes as **versioned snapshots**
(`schema-snapshots/<type>.v<N>.yaml`, a dedicated pack resource kind), and `jigc migrate-corpus`
sources `from` **per committed doc keyed on its `schema-version` stamp** — so the structural
transform branches (built + unit-proven since Inc 2, but engine-substrate-only) become reachable
through the shipped binary ([corpus-migration.md](../../../design/corpus-migration.md) → Prior-schema
sourcing; [worked-examples.md](../../../design/worked-examples.md) → flow 36;
[validation.md](../../../design/validation.md) → version-aware routing / detector-verb agreement).

This records the **measured facts of the headline structural loop**, each **observed by running the
ACTUALLY-PINNED `jigc` binary** (NOT `cargo test`) over a real dev-pack corpus in a scratch git
repo, with the `doc-code` probe resolved as the **sibling beside the installed binary** (no
`JIGC_DOC_CODE_PROBE`) — the production path. The one override is `JIGC_PACK_DIR`, pointing at a
**`FilesystemPack` fixture** that ships the versioned snapshot store (the seam Inc-4 adds): a clone
of the embedded dev pack + `schema-snapshots/{prd,adr}.v1.yaml` + the `prd`/`adr` manifest versions
bumped to **2**, so the committed v1-stamped docs read as below-version. (The embedded pack has no
snapshot store yet — the snapshots are a per-migration authoring artifact, sourced through the
documented `JIGC_PACK_DIR` seam exactly as a real adopter would supply them.)

## Binary under test

| | |
|---|---|
| `jigc` sha256 | `cfd176a41f2c3748c41c5ebf0193223de3fbc83afeccbac6064330cda5e6b11d` |
| `doc-code` probe sha256 | `b0a413a1cb8fa7fff6ceb85e3fdebc304508c42d76c57447112f4f208ea74304` (unchanged across all of M34 — the milestone added no grammar/probe code; byte-identical to the Inc-3 pin) |
| HEAD commit | **`cf2b340`** (`test(cli): real-binary v1->v2 structural corpus migration acceptance (flow 36)` — the M34 Inc-4 T3 acceptance, the increment's code HEAD). T4 adds only this `completions/artifacts/M34/` record on top, so the shipped binary equals the increment's code HEAD. |
| Built | `cargo install --path crates/cli --force` (release; `build.rs` builds the eight-grammar `doc-code` probe and embeds it via `OUT_DIR/doc-code` for the `jigc setup` extract path, plus the embedded dev pack incl. `config/schema-manifest.yaml`); `rustc 1.95.0` |
| Pinned to | `jigc` **and** its `doc-code` probe sibling to **both** `~/.local/bin/` and `~/.cargo/bin/` (+ `target/release/`). `cargo install` placed `jigc` at `~/.cargo/bin/jigc` (byte-identical to the `target/release/jigc` it co-produced); both `jigc` and its `doc-code` sibling were copied into both bin dirs. **All sha256 identical** — `jigc` identical across both dirs + `target/release` (`cfd176a4…`), `doc-code` identical across both dirs + `target/release` (`b0a413a1…`). `which jigc` → `~/.local/bin/jigc`, `which doc-code` → `~/.local/bin/doc-code` (a pinned dir). |
| Fresh-build cross-check | The pinned `jigc` IS the output of this `cargo install` at `cf2b340` — byte-identical to the `target/release/jigc` it co-produced (same `cfd176a4…`). (Release `jigc` is **not** stripped, so a *separate* later rebuild yields a different sha — the pin is the bytes this install wrote, not a reproducibility claim.) |
| Size | release `jigc` **13.41 MiB** (14 057 936 B), `doc-code` **8.10 MiB** (8 489 160 B). |
| Invocation | `jigc` from `PATH`, `doc-code` resolved as the installed sibling (**no** `JIGC_DOC_CODE_PROBE` — the driver `unset`s it); `JIGC_PACK_DIR` → the snapshot-bearing `FilesystemPack` fixture (the prior-schema sourcing seam). |
| Driver | [`evidence/structural-drive.sh`](evidence/structural-drive.sh) — a self-cleaning scratch git repo + isolated `$HOME` + fixture pack; logs in [`evidence/`](evidence/) (`structural-*`). |

## Honest posture — protocol-observed, not engine-emitted

Per [measurement.md](../../../design/measurement.md) and the settled posture (M17/M23–M33): this is
an **owner-artifact recording** (the recorded-alongside measure run on the actually-pinned binary),
**not** a TDD red→green test — its done-criterion is this committed artifact plus the clean
end-to-end loop on the installed binary. The loop's behaviour is independently **binary-proven** by
[`crates/cli/tests/flow36_corpus_structural.rs`](../../../crates/cli/tests/flow36_corpus_structural.rs)
(`structural_v1_to_v2_migration_runs_through_the_real_binary`,
`structural_migration_is_byte_identical_across_commit_orders`) and the prior-schema sourcing /
value-bump / missing-snapshot-block paths over the verb core in
[`crates/cli/src/migrate_corpus.rs`](../../../crates/cli/src/migrate_corpus.rs). This artifact's job
is to show those facts *on the binary a real install actually resolves*, with the production sibling
probe.

## The measured loop — one v1 corpus, detect → migrate → re-validate

A committed v1 corpus in a scratch repo: two **fixed-slot** `prd` docs at their canonical
`docs/prds/<slug>.md` (`cache-prd`, `queue-prd`), each stamped `schema-version: 1` and carrying the
**original M9-era fixed `## Requirements` slot** — the byte form a committed v1 prd had before the
M25 reshape ([`evidence/structural-prd-before.log`](evidence/structural-prd-before.log)) — plus one
v1 `adr` (`alpha-decision`, the widened-cardinality secondary,
[`evidence/structural-adr-before.log`](evidence/structural-adr-before.log)). The fixture's manifest
declares `prd` + `adr` at version **2**, so all three docs read below-version.

### Step 1 — DETECT: every below-version doc is routed `migrate`

`jigc validate` over the v1 corpus reports a **`schema-conformance.field-value-conformant` break** on
each below-version `schema-version` stamp, explicitly routed **`migrate`**, report-only at store
scope (**exit 0**) ([`evidence/structural-detect.log`](evidence/structural-detect.log)):

```
blocking · schema-conformance.field-value-conformant — field `schema-version` is schema-version 1, below the current schema-version 2
  route: migrate — `docs/prds/cache-prd.md` is stamped schema-version 1, below the current 2; run the corpus migration to upgrade it
```

The version-aware routing fires on the **stamp value** (`below-version ⇒ migrate`), so **all three**
below-version docs — both prds *and* the adr — are routed `migrate` (3 `route: migrate` lines). The
adr's *body* is conformant under the widened card, but its stamp is below current, so the detector
flags it too; this is exactly the detector/verb agreement the verb honours in step 2 (every routed
doc is migrated, never reported `already-current` — [DECISIONS.md](../../../DECISIONS.md) → 2026-06-25
audit Finding 2). *(The worked-examples flow-36 narrative describes the adr as "not flagged here"
focusing on its byte no-op body; the observed binary flags it on the stamp value, which is the
stronger, fully-agreeing behaviour — `>= 2` routes, as the acceptance test asserts.)*

### Step 2 — MIGRATE: the verb reshapes structurally + value-bumps the stamp

`jigc migrate-corpus` sources each doc's prior shape from the snapshot store keyed on its stamp
(`prd.v1.yaml` fixed-slot, `adr.v1.yaml` narrower card), diffs it against the current v2 schema,
applies the structural splice, gates each doc on v2 conformance, and writes back byte-stable
(**exit 0**) ([`evidence/structural-migrate.log`](evidence/structural-migrate.log)):

```
corpus migration: 3 migrated, 0 already current, 0 blocked
  migrated   docs/decisions/alpha-decision.md
  migrated   docs/prds/cache-prd.md
  migrated   docs/prds/queue-prd.md
```

**The prd reshape landed byte-stable** ([`evidence/structural-prd-after.log`](evidence/structural-prd-after.log)):
the fixed `## Requirements` slot became a **repeatable section** whose old slot prose is preserved
verbatim as the **default first item** (`### requirements  {#requirements}` + the original
sentence), the `## Vision` / `## Context` prose unchanged, and the stamp **value-bumped `1` → `2`**
(a present-field `set_field` splice — distinct from the v0→v1 *add*). This is the
`fixed-slot→repeatable-with-default` promotion (the riskiest net-new primitive) run over **two real
declared schemas**, no LLM in the structural path.

**The widened-cardinality adr migrated byte-identical-except-stamp**
([`evidence/structural-adr-after.log`](evidence/structural-adr-after.log)): `adr.supersedes`
`0..1`→`0..*` rewrites no instance bytes (the existing absence stays valid under the wider card), so
**only** the stamp value-bumped `1` → `2` — `adr_after == adr_before.replace("schema-version: 1",
"schema-version: 2")`.

### Step 3 — RE-VALIDATE: the migrated corpus is v2-conformant

`jigc validate` re-run surfaces **no `schema-conformance` finding** and **no `migrate` route**
(**exit 0**) ([`evidence/structural-revalidate.log`](evidence/structural-revalidate.log)). The only
remaining findings are unrelated advisories (`file-state.un-baselined` — the docs were committed by
hand, never CLI-baselined; `schema-completeness.inverse-cardinality` — the prds have no specs). The
loop is closed: the corpus stranded below v2 now carries its authored-against version in its v2
shape.

### Guard — idempotent re-run

A second `jigc migrate-corpus` is a clean **no-op**: the now-v2 docs read at-version and are skipped
(`0 migrated`), leaving every migrated doc **byte-identical** (sha256 unchanged before/after for both
the reshaped prd and the bumped adr) ([`evidence/structural-idempotent.log`](evidence/structural-idempotent.log)).

### Guard — deterministic across commit orders (increment-workflow #7)

The same two-prd corpus seeded in **id-order** (`cache-prd` then `queue-prd`) and in **reverse**
(`queue-prd` then `cache-prd`) migrates to **byte-identical** output — same sha256 for each doc under
both orders — and the two distinct docs migrate to **genuinely distinct** content (the fixture forces
real overlap, not two empty strings) ([`evidence/structural-determinism.log`](evidence/structural-determinism.log)).
The verb keys output on the path-sorted corpus, never on commit order.

| Step | Corpus | `jigc` command | Exit | Observed |
|---|---|---|---|---|
| 1 · detect | v1 (2 fixed-slot prd + 1 adr) | `validate` | **0** | 3 × `schema-conformance` break, all `route: migrate` |
| 2 · migrate | v1 (below-version) | `migrate-corpus` | **0** | `3 migrated`; prd slot→repeatable (prose as default item), adr byte-identical-except-stamp, stamp value-bumped `1`→`2` |
| 3 · re-validate | v2 (migrated) | `validate` | **0** | no `schema-conformance` finding, no `migrate` route |
| 4 · idempotent | v2 (migrated) | `migrate-corpus` | **0** | `0 migrated`, every doc byte-identical |
| 5 · determinism | v1, id-order vs reverse | `migrate-corpus` ×2 | **0** | byte-identical output across commit orders |

## G2 retired

The full **detect→migrate→re-validate** loop runs end-to-end on the shipped binary for a **real
v1→v2 structural change**: a fixed-slot `prd` corpus stranded below the bumped schema version is
detected and routed `migrate`, the verb sources each prior shape from the versioned snapshot store
and applies the `fixed-slot→repeatable-with-default` reshape byte-stable (the old prose preserved as
the default item, the stamp value-bumped, v2-conformant, idempotent, deterministic across commit
orders), and a widened-cardinality secondary migrates byte-identical-except-stamp. The structural
transform branches built in Inc 2 are now reachable through the compiled `jigc migrate-corpus`; the
detector and verb agree on every below-version doc. Productive-readiness gate **G2** is retired on a
real corpus migration — the bar the roadmap set ("G2 retires once the structural verb path lands —
Inc 4"). (G1 — the freeze gate that *blocks* an un-migrated shape change — was retired in
[M33](../M33/freeze-gate-measure.md); the Inc-3 stamp dogfood is recorded in
[migrate-loop-measure.md](migrate-loop-measure.md).) The milestone-completion audit runs separately
after this increment.
