# Doctype map

A **planning aid**, not a schema spec: the eventual managed doctypes, each with a one-line purpose, the cross-reference edges between them, and which milestone is expected to *drive* (and so define) each. This is step 2 of the product-build loop ([milestone-planning-workflow.md](milestone-planning-workflow.md) → Where this sits), consulted by that workflow's **detect gaps** phase to see what doctypes a milestone will need.

**Intent level, deliberately.** This map names doctypes and relations so milestone sequencing has a target picture. It does **not** define fields, slots, or validation — a doctype's schema is set by the workflow that creates and reads it, in the milestone that ships that workflow ([VISION.md](../VISION.md) → Document model for the principle, [design/document-type-schema.md](../design/document-type-schema.md) for the schema mechanism). Defining a schema before its driving workflow is generality for a single-use thing ([PRINCIPLES](../PRINCIPLES.md) → Keep it minimal).

## The doctypes

| Doctype | Purpose (one line) | Persistence / sink | Driver | Status |
|---|---|---|---|---|
| `commit` | the change description for one task | transient → git commit message | M1 | ✅ shipped |
| `adr` | one architecture decision | persisted → `decisions/` | M1 | ✅ shipped |
| `spec` | the "what" a task implements | persisted → `specs/` | M3 | planned |
| `prd` | product requirements above specs | persisted → `prds/` | M9 | planned |
| `arch-doc` | living architecture documentation | persisted → `architecture/` | M13 | ✅ shipped |
| `roadmap` | the milestone spine + per-milestone decomposition | persisted (running singleton) | M16 | ✅ shipped |
| `deferral-ledger` | the forward look — keyed deferred decisions | persisted (running singleton) | M16 | ✅ shipped |
| `decisions-log` | the running log of what was decided + why | persisted (running singleton) | M16 | ✅ shipped |
| `completion-record` | one milestone's audit → findings → verdict | persisted → `completions/` | M16 | ✅ shipped |
| `dogfood-record` | one measured jigc run's case, fact counts, seeded checks, verdict, judgment | persisted → `dogfood/` | M17 | ✅ shipped |
| `changelog` | the running record of user-facing changes per version | persisted (running singleton), `changelog/` | doctype-expansion | planned |

`changelog` is the first member of the **doctype-expansion** track (G2, post-M21 — [decisions-pending.md](decisions-pending.md); design [changelog.md](../design/changelog.md)). Like the methodology + `dogfood-record` doctypes it sits **below VISION's named starting set** (a real-project doc graduating to managed, earned from a driver) — no new VISION document-model claim. It is a **dev-pack** doctype, singleton, edge-free, and is the consuming target that earns the four engine lifts (multi-level repetition, multi-word section-id, optional slot/field, doc-level `set: on-create`). The candidate **set** beyond it is mapped below.

VISION's starting set (commit · arch-doc · prd · adr · spec — [VISION.md](../VISION.md) → Document model) is **complete**: every member ships. `arch-doc` was the last, driven at M13 by the architecture-documentation workflow ([architecture-documentation.md](../design/architecture-documentation.md); [roadmap.md](roadmap.md) → M13). The four **methodology working-doc** doctypes below the starting set are not part of VISION's named set — they are the methodology's *own* documents graduating from plain markdown to managed doctypes at M16 (the self-hosting fold-back), each earned from its driving workflow (planning drives `roadmap` + `deferral-ledger`; completion drives `decisions-log` + `completion-record`), schemas in the **methodology pack** ([methodology-docs.md](../design/methodology-docs.md); [roadmap.md](roadmap.md) → M16). `dogfood-record` (M17) is also below the starting set but is **net-new, not a graduation** — the per-run measurement record, earned from the `record-dogfood` workflow that authors it ([measurement.md](../design/measurement.md)).

## The relations (edges)

Direction reads source → target; cardinality is the rough *intent*, not a locked schema constraint.

- `adr` —**supersedes**→ `adr` (0..1 → 0..1) — **locked, shipped** (M1). The one logic-free task→committed-doc handle the MVP proves ([worked-examples.md](../design/worked-examples.md) → Superseding decision).
- `commit` —**implements**→ `spec` (**0..1** → 0..1) — **activates with M3.** A task's commit references the spec it satisfies; the M3 `spec` worked example walks this edge at finalize. Cardinality is `0..1` on the commit side (not the `1` of first-draft intent), because spec-less tasks (`single-task`, `quick-fix`) still produce a valid `commit` — the edge is optional, set only by the spec-driven workflow.
- `spec` —**decided-by**→ `adr` (n → n) — a spec points at the decisions that shaped it. **Deferred past M3** and **re-triaged OUT at M13 planning** (2026-06-07): no workflow in M13's arch-doc scope writes or reads it, so it stays named-unscheduled intent until a workflow drives it (a one-line schema delta when it does).
- `prd` —**decomposes-into**→ `spec` (1 → n) — a PRD's requirements fan out into specs. **Triaged OUT at M13 planning**; **re-confirmed OUT at M25 planning** (2026-06-17). The structural blocker (the `prd` repeatable-requirements shape) is now **built at M25** (per-requirement items — pack-only), but the edge itself stays deferred: it needs a *new header section* on `prd` (which has only slots/repeatable today) and re-incurs the ref-shape landmine, with no migration driver demanding it.
- `arch-doc` —**cites**→ `adr` (n → n) — architecture docs reference the decisions behind them. **Landed M13** ([architecture-documentation.md](../design/architecture-documentation.md)): authored as a doc-level (header) n→n `ref` field by the architecture-documentation workflow and walked at finalize via `ref-resolves` (pass when the cited adr exists, block when it dangles).

Work-units (`task`, `increment`, `milestone`) are **not doctypes** — they're the anchors docs bind to (`task.commit`, `task.spec`, `task.decision`), carrying engine-native data-value roots, not schemas. The `milestone` work-unit activates in **M7** (the deterministic join core — [roadmap](roadmap.md)); its `{{milestone.tasks}}` data-value root lands in **M8** with the fan-out step that consumes it; `increment` minting stays deferred.

```text
prd ──decomposes-into──▶ spec ──decided-by──▶ adr ◀──supersedes── adr
                          ▲                     ▲
                     implements               cites
                          │                     │
                       commit               arch-doc
```
*(illustrative — edges, not a schema)*

## The project-doctype candidate set (forward map — doctype-expansion track)

Settled at the doctype-expansion milestone's planning (2026-06-14): **build `changelog` only; map the rest.** Each mapped candidate earns its schema *when its driver is built* — defining one now is generality for a single use ([PRINCIPLES](../PRINCIPLES.md); design [changelog.md](../design/changelog.md) → candidate set). The honest line for "everything a real project needs": **most real-project files are not managed doctypes** — they are boilerplate, generated, or bespoke.

| Candidate | Disposition | Why |
|---|---|---|
| `changelog` | **build now** | real driver (record-change + single-task fold-in); genuine structure; maintained over time |
| `contributing` | map-only (driver unscheduled) | plausibly a one-shot project-setup-class authoring flow; thin structure, weakly "maintained" |
| `runbook` | map-only (driver unscheduled) | a plausible ops/incident driver (procedures→steps, would want multi-level), but no workflow needs it yet |
| `SECURITY` / `license` / `code-of-conduct` | **out** | boilerplate or verbatim/selected text — no authoring *judgment*, no schema to earn |
| `api-docs` | **out** | tooling-generated from code, not LLM-prose-in-slots |
| `release-notes` | **out** (fold into `changelog`) | duplicates `changelog`'s release prose unless a distinct-audience driver proves otherwise |
| `README` | **out (charter-locked)** | bespoke front-page prose, no schema-able recurring structure, no composing workflow — the canonical "no schema to earn" |

## Unsettled

- **`changelog` driver — settled at the doctype-expansion milestone's planning (2026-06-14).** A standalone `record-change` workflow (off-router authoring spine) + a `single-task` `allows-create` fold-in, both dev-pack-local; schema earned there ([changelog.md](../design/changelog.md)). Edge-free (a changelog→commit edge fails — `commit` is transient). The milestone builds the four engine lifts the faithful shape earns.
- **`prd` driver — settled M9; repeatable-requirements added at M25.** The new-project idea-development setup workflow drives `prd`, earning its schema from that real creator ([project-setup.md](../design/project-setup.md) → Flow 1; [DECISIONS.md](../DECISIONS.md) 2026-06-06). M9's `prd` used **fixed prose slots**; **M25's prd-migration driver** (2026-06-17) converts `requirements` to a **repeatable section** (per-requirement `title` field + `statement` slot, pack-only — `add-item`/multi-word-heading cleared the blockers), so a migrated foreign PRD's requirements become individually-addressable. The `prd —decomposes-into→ spec` edge stays deferred (above).
- **`arch-doc` driver — settled at M13 planning (2026-06-07).** *Living architecture documentation* earns its schema from the **architecture-documentation workflow**, designed in [architecture-documentation.md](../design/architecture-documentation.md): a persisted doctype (`location: architecture/`) with a doc-level `overview` slot, a doc-level n→n `cites → adr` header ref, and a **repeatable `components` section** (each component a `description` slot + an `implemented-by` `code-anchor`). This activates `arch-doc↔code` over M10's probe with a **per-component `symbol-exists`** check — which forces the one engine concept M13 adds: a **per-field-type predicate selector** (`check:` on the pack field-type decl), since position no longer discriminates the `doc-code` predicate (`spec.criteria` wants `criterion-maps-to-test`, arch-doc's `components` want `symbol-exists`). The repeatable shape also pulls in the item-authoring surface (`add-item` + item-leaf `set-slot`/`set-field`). **Half #2** (graduating the methodology's own working docs — roadmap / deferral-ledger / decisions-log + completion-record — to managed doctypes) is **settled at M16 planning** (2026-06-09): the planning + completion workflows are encoded (`creates-task: true, selectable: false`) and each drives its doctype in one earn-from-driver motion ([methodology-docs.md](../design/methodology-docs.md); [roadmap.md](roadmap.md) → M16), proven on fresh instances in the methodology pack — jigc's own files migrate at v1.
- **doc↔code edges** (`adr`/`spec`/`arch-doc` → code) are pack-provided validation *probes*, not structural relations — so they add **no structural edge** here. **Landed M10** for `adr` (a `cites-code` code-anchor in the `status` header — the floor) and `spec` (a `maps-to-test` code-anchor in the `criteria` block — the headline), via the first pack-provided probe (`doc-code`) + the first pack-declared field type (`code-anchor`); `arch-doc↔code` rides the same mechanism at M13 with a **per-component `symbol-exists`** anchor (`implemented-by`), which is what forces the per-field-type `check:` predicate selector ([architecture-documentation.md](../design/architecture-documentation.md); [roadmap.md](roadmap.md) → M10/M13; [validation.md](../design/validation.md) → The `doc-code` probe; [DECISIONS.md](../DECISIONS.md) 2026-06-06). Per [VISION.md](../VISION.md) → principle #6.
