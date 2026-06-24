# Schema freeze & managed-corpus migration (M33 + M34)

The **productive-readiness pair**. M33 declares the doctype set + schema-definition format a **frozen v1** and makes the freeze *enforced, not asserted*; M34 builds the **automated path to migrate a managed-document corpus** across a v1→v2 schema change, so the lock-in cost a productive adopter inherits is *bounded* rather than open-ended. They are planned together because they are coupled at the seam: M34's transform migrates *onto* the v1 shape M33 declares. They **build as two sequential milestones, M33 first**: M33 completes + freezes the v1 shape and enforces it **dev-side** (a schema change can't silently ship — the manifest + pack-load assertion, self-contained); M34 then builds the **corpus-side** machinery (detector + transform + stamp) that lets a *blocked* change be migrated rather than only blocked ([DECISIONS.md](../DECISIONS.md) → 2026-06-24 M33 + M34; charter [roadmap.md](../implementation/roadmap.md) → M33/M34).

This doc owns the **version stamp**, the **store-scope conformance detector**, the **deterministic transform**, and the **freeze declaration + discipline**. It is the managed-corpus sibling of [auto-migration.md](auto-migration.md) (foreign-doc *adoption*) — and the distinction is load-bearing, below.

## Why this is net-new, not a reuse (the baseline audit's correction)

The original M34 charter assumed *"reconciliation already provides the detect half; M34 adds the transform arm."* The baseline audit (2026-06-24, exercised on the real binary) **refuted that**: store-scope `jigc validate` runs four families — doc↔code, workflow↔refs, file↔CLI-state (**hash-only**), cross-doc ref-resolves — and **none re-parses a committed doc against its schema**. A doc made non-conformant by a *schema* change while its **bytes are unchanged** passes `validate` clean (exit 0, verified). Schema-conformance runs only at **task scope** (`finalize` / `jigc task validate`), only over docs a task touches.

So the genuine value-flow — *a managed corpus at v1 → a schema-shape change ships → the now-non-conformant corpus is detected, blocked, and migrated* — is broken at **step one** today. Both halves are net-new:

| Piece | State today | M33/M34 builds |
|---|---|---|
| Store-scope **schema-conformance detector** | absent (4 families, none conformance) | the 5th `validate` family ([validation.md](validation.md)) |
| Per-doc **schema-version stamp** | absent (only result-JSON `schema_version`) | a schema-declared, engine-valued front-matter field |
| **Schema-diff** | absent (`Schema` derives `PartialEq` only) | a v1→v2 delta classifier |
| Deterministic **transform producer** | absent (`migrate` is LLM foreign-adoption) | schema-diff-driven byte-stable splices |

## The determinism boundary holds (the auto-migration distinction)

[auto-migration.md](auto-migration.md) (`jigc migrate`) adopts a **foreign** document into a **fresh** managed doc: the LLM rewrites un-CLI-verifiable prose, the CLI strict-parses + conformance-gates + adopts, with a human fidelity gate. It is one-file-one-`--as`, never re-opening a managed doc.

M34's corpus migration is the **opposite shape**: a *conformant managed v1 doc* → a *structurally re-rendered v2 doc*, over **N instances of a type at once**. Its source is already canonical (no fidelity gate needed), and the transform is **largely deterministic and CLI-owned**:

- **Structural changes are CLI-owned, no LLM** — *add optional field*, *widen cardinality* (`0..1`→`0..*`), *fixed-slot→repeatable-with-default*. These are computed from the schema-diff and applied as **byte-stable splices** over the proven `write.rs` primitives. The prose is **unchanged**; routing a structural-only bump through an LLM would be lossy, non-deterministic, *and a determinism-boundary breach* (the CLI owns structure — [VISION.md](../VISION.md) → the determinism boundary). This is why reusing `migrate`'s re-author path was **rejected**.
- **Prose-needing changes route to the agent** (Framing A) — a *new required slot* has no deterministic default; the CLI mints the empty slot and the migration workflow routes it to the agent to author, exactly as a `create` flow does. The CLI never authors the prose; it owns only placement and the conformance gate.

The boundary is the same one [auto-migration.md](auto-migration.md) draws; only the *source* (managed-canonical vs foreign) and the *deterministic fraction* (high vs low) differ.

## The schema-version stamp — schema-declared, engine-valued

A managed doc must record **which schema version it was authored against**, so the detector can tell a *known-old-version* doc (migrate it) from a *genuinely-corrupt* one, and so the transform knows its starting point. The stamp is **per-doc, recording the doc's *own doctype* version** (not a global doctype-set version), matched against that doctype's entry in the manifest.

**Shape: a field declared in each doctype's schema, with an engine-supplied value** — the `status`/`date` model (in-schema field, engine-set value). The engine injects the declaration uniformly across all doctypes (no N hand-edits), and a new **"current active schema version" deriver** supplies the value at create time. The crucial reason it is **in-schema, not out-of-schema** (review Finding 1): the migration transform is driven by a **schema-diff over two `Schema` values**, so an out-of-schema engine/manifest field would produce *no* `added-field` classification and the schema-diff classifier — the riskiest net-new code — would get only synthetic coverage. In-schema makes the stamp a *real* `added-field` migration. The spike that shaped the rest: `spec`/`prd`/`changelog` render **zero front-matter** (no `header:` section), so the stamp **introduces a `---` block that never existed** for those three — a real v0→v1 shape change.

> **Two senses of "stamp" — keep them distinct.** This **schema-version stamp** is a durable, in-doc front-matter field. It is *not* the edge-index **cache-stamp** (`storage.md` → the index's HEAD-sha marker), which is a rebuildable-cache freshness marker carrying no schema version. The migration reads and writes the former; the latter stays a cache concern.

Two consequences the build must carry:

- **The deriver is net-new.** Today the on-create deriver produces a value only for `Date + set:on-create` (the clock) else a literal `default`; there is no "live schema version" deriver. ([storage.md](storage.md) → front-matter; [document-type-schema.md](document-type-schema.md) → the stamp field.)
- **Stamping the existing corpus is the first migration — and a genuine one.** Adding the in-schema field is an `added-field` schema-diff the transform classifies and applies; it churns every doc's `file-state` hash and *introduces front-matter* to the header-less three. **Run as the dogfood case of M34's own add-field transform** (build-order: the transform exists before the stamp lands), so the first real migration proves the transform on a real case (the cold-start spike discipline — [milestone-planning-workflow.md](../implementation/milestone-planning-workflow.md) → cold-start state).

**The stamp flips *last*, never mid-migration (review Finding 2).** A migration that includes a *prose-needing* change mints an empty required slot and routes it to the agent; if the stamp were bumped to v2 *before* that slot is authored, the conformance detector would read the in-flight doc as `at-version + missing-required-slot` ⇒ **corrupt**. So the stamp flips to the new version **only after** every structural splice *and* every prose-pending slot is authored and the migration-upgrade gate passes the doc as conformant. An in-flight doc therefore always reads `below-version ⇒ migrate`, never `corrupt`.

## The store-scope conformance detector — the 5th `validate` family

The detect-half. A 5th store-scope family that re-parses **every committed doc against the current resolved schema** and reports each non-conformant instance — the analog the four existing families never covered for *schema* shape. Designed in [validation.md](validation.md) → Store-scope schema-conformance (the fifth family). Key properties, pinned:

- **Verified clean lift.** `conformance_for` / `schema_conformance` (`validate.rs`) are **pure functions** over `(filename, schemas, rel_key, source)` — no working-area, no `reconcile_committed_store`, no task context (confirmed by spike). Fed the committed docs + the current schemas, they fire `schema-conformance.{required-slot,required-field,field-value,ref-resolves}` over stale docs. The committed-doc walk already exists (`file_state::detect_committed_store`); the family reuses **that** doc-walk, *not* the anchor-scoped `enumerate_committed_surface`.
- **Scope pin (M4 lesson — state the surface the check fires against).** The family fires over the **whole committed store, every instance**, and is **report-only at store scope** (exit 0), exactly like the other four content families — `jigc validate` stays a read command that never gates a transaction ([validation.md](validation.md) → Exit semantics). A version-mismatch (doc stamped < manifest v1) is surfaced as the *migrate* route; a conformance break with no version explanation is surfaced as *corrupt*.
- **No new severity surface** — `schema-conformance.*` check ids already exist (the M20/2026-06-23 pattern: same id, store/read-only scope, no knob growth).

The **blocking** counterpart is the **migration-upgrade gate** (below), which fires at the *transform transaction boundary*, not at `validate`.

## The deterministic transform — schema-diff → byte-stable splice

1. **Schema-diff.** A v1→v2 delta classifier over two `Schema` values, classifying each change by kind: `added-optional-field`, `widened-cardinality`, `fixed-slot→repeatable-with-default`, and **`prose-needing`** (a new *required* slot/field with no deterministic default — the Framing-A escape hatch). Net-new (`Schema` derives `PartialEq` only; no comparison code exists).
2. **Transform driver.** Consumes the classified diff and emits a sequence of byte-stable splices over `write.rs` primitives (`splice`, `set_field`, `add_item`, `nested_repeatable`). The one **net-new primitive** is **fixed-slot→repeatable-with-default**: promote a physical `<<slot>>` into a repeatable item-block carrying the old slot content as the default first item (`add_item` assumes the section is *already* repeatable; the promotion is unexercised today).
3. **Prose routing.** A `prose-needing` change mints the empty slot and routes it to the migration workflow's agent step (Framing A); the CLI conformance-gates the result.

## Completing v1 — the five schema-engine opens

The freeze is a **one-way door**: anything not in v1 returns only through a breaking migration. So the robust cut is a *complete, correct* v1 — all five deferred schema-engine opens are **built into v1**, not deferred past the freeze (the cheap path that guarantees a future migration). The structural ones are *also* the real schema changes the transform proves on — which is what makes the M34 transform exercised on genuine cases rather than fixtures.

| Open | What v1 gains | Migration case |
|---|---|---|
| **prd→spec (`decomposes-into`)** | the doctype-graph edge the [doctype-map](../implementation/doctype-map.md) already intends; needs a new `prd` **header section** to carry the ref | a real `add-section` + `add-field` change existing PRDs migrate across — the headline transform proof |
| **repeatable-item managed-ref edges** | a `ref` field on a repeatable item resolves through the edge index (today `index.rs` skips non-simple sections — silently inert); the byte-neutral substrate for prd→spec's per-requirement route | byte-neutral to build (edge-extractor branch); a schema change only when a doctype declares such a ref |
| **inverse/min-cardinality** | enforced completeness ("a PRD must have ≥1 spec") over the store-sweep substrate; pairs with prd→spec | settles the v1 schema metadata (`inverse-card` declarations — a schema-hash change, byte-neutral to instances) |
| **in-prose mentions** | a lighter `#ref` check inside slot prose | additive validation feature — no instance-byte change; in v1 for completeness |
| **schema-fragment `include`** | de-duplicates the `changelog` change-group block; a clean frozen schema-definition format | additive format feature — no instance-byte change; in v1 so the frozen format is clean |

The first three change *shape or schema metadata* and so genuinely belong *before* the freeze; the last two are additive-later (no migration penalty to defer) but are built into v1 anyway for a clean frozen baseline ([DECISIONS.md](../DECISIONS.md) → 2026-06-24 M33 + M34).

## The freeze — declared *and* enforced

M33's deliverable is **declaration + enforcement**, not prose discipline:

- **Declare the doctype set + schema-definition format v1-complete.** The format (`id-from`/`card`/`set`/`of`/`header`/`to`/`inverse`/`inverse-card`) is already **shipped + golden-locked** ([document-type-schema.md](document-type-schema.md)) — M33 *formalizes* it, and corrects the stale "placeholders pending implementation" line as the act of declaring. The frozen *set* is the persisted doctypes enumerated in [doctype-map.md](../implementation/doctype-map.md).
- **A versioned, hashed doctype-set manifest.** An enumerated `doctype → schema-version + schema-hash` artifact, so the freeze can *self-enforce*: a schema-shape change that bumps no version + ships no M34 migration is **blocked**, rather than caught by review. This is the net-new artifact that makes the discipline mechanical.
- **The enforcement gate fires at pack-load / build time, not at `validate` (review Finding 3 — scope pin).** It is an **engine assertion** that recomputes each shipped doctype's `schema-hash` and compares it to the manifest, failing **loudly** on a mismatch — the sibling of the existing intrinsic-floor assertion ([validation.md](validation.md) → Severity inventory), *not* the report-only `validate` store sweep (which exits 0 and would never gate). A hash mismatch with no bumped manifest version is the un-migrated schema change the freeze forbids; the fix is to bump the version + ship the M34 migration (or revert). This firing surface is **pinned now** even though the manifest's on-disk home/format is settled at the increment that builds it.
- **Convergence is evidence, not a promise.** Beyond the gate, the freeze is *proven* by a defined run of milestones with zero schema-shape commits — tracked after the declaration, not gated inside M33.

## Property census (every property this pair establishes → every path → acceptance)

The required Settle artifact ([milestone-planning-workflow.md](../implementation/milestone-planning-workflow.md) → the census). A property asserted on one path and silent on its sibling is an unscoped fork the build resolves by reaching for the old pattern.

| Property | Every path that must uphold it | Acceptance per path |
|---|---|---|
| **Byte-stability** | each of the 6 persisted doctypes (incl. the 4 newly-fuzzed: prd/spec/changelog/arch-doc); the stamp-injection render; **each** transform kind (add-field, widen-card, slot→repeatable) | `arb_doc()` proptest extended to all 6 → no-op write byte-identical; each transform output round-trips byte-identical |
| **Transform determinism** | add-optional-field · widen-cardinality · fixed-slot→repeatable-with-default | same schema-diff in → same bytes out, **no LLM call** in the structural path |
| **No-data-loss** | structural transform (all prose preserved) · prose-needing change (routed to agent, never silently dropped) · the v0→v1 stamp dogfood (no instance content lost) | per-kind: a migrated doc retains every pre-migration slot/field value; a prose-needing change *blocks* until authored, never drops |
| **Migration atomicity / WIP-safety** (CLAUDE.md "writes are transactional") | the N-doc corpus transform passes through a mixed v1/v2 state (interrupted by Framing-A prose authoring) · the abort path after doc *k* of *N* · the per-doc stamp-flip boundary | transaction granularity is **per-doc, gated** — each doc commits only when conformant + stamp-flipped (above); an abort mid-corpus leaves already-migrated docs at v2 and the rest at v1, both conformant-and-detectable (the detector routes the un-migrated remainder as `migrate`); no doc is left half-transformed or wrongly stamped. *(Granularity + the exact rollback inventory — paths, file-state, index — pinned at the increment that builds the transform, the M35-rename-charter "rollback inventory" discipline.)* |
| **Conformance-detection completeness** | every doctype × every required-leaf kind: missing slot · missing field · invalid enum/value · dangling ref · version-mismatch | the 5th family fires the right `schema-conformance.*` finding on each break shape over the committed store |
| **Freeze self-enforcement** | a schema-shape change with no version bump + no migration | the manifest-hash check **blocks** it |

## Acceptance flows (spiked against the real binary)

The engine-behaviour claims these flows encode were **exercised at Scope** (the baseline audit drove the real binary): store-scope `validate` runs four families exit 0; `conformance_for` is a pure task-less function; `write.rs` primitives are byte-stable; `migrate` is foreign-adoption (not corpus-upgrade). The acceptance flows therefore rest on verified, not assumed, behaviour.

- **The detect flow** — a committed corpus at v1; a schema-shape change ships; `jigc validate` now **reports** every stranded instance at store scope (today: silent, exit 0). The headline regression-retirement.
- **The migrate flow** — the same stranded corpus run through `jigc <migrate-corpus>`: add-field/widen-card/slot→repeatable applied deterministically byte-stable; a prose-needing change routed to the agent; the result conformance-gated and adopted; the manifest re-stamped v2.
- **The dogfood flow** — the v0→v1 stamp migration *is* the first add-field transform over the existing corpus.
- **The freeze-enforcement flow** — a schema edit with no version bump is blocked by the manifest check.

These land in [worked-examples.md](worked-examples.md) when the increments build them.

## Open questions

- **Manifest home + format** — where the versioned doctype-set manifest lives (a `.jigc/` artifact vs a pack-shipped declaration) and how `jigc upgrade` relates to it is settled at the increment that builds it.
- **Layout/dialect changes** — this pair scopes *schema-shape* migration; a *directory-layout* or *workflow-dialect* breaking change is a sibling migration class, deferred until a real driver forces it (the `docs-root` move showed layout can shift, but a pinned `docs-root` already mitigates it — [READINESS-ASSESSMENT.md](../READINESS-ASSESSMENT.md)).
