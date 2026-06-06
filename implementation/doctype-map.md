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
| `arch-doc` | living architecture documentation | persisted | M13 | planned |

VISION's starting set is commit · arch-doc · prd · adr · spec ([VISION.md](../VISION.md) → Document model). `commit` + `adr` are built, `spec` is next (M3), `prd` + `arch-doc` await a driving workflow.

## The relations (edges)

Direction reads source → target; cardinality is the rough *intent*, not a locked schema constraint.

- `adr` —**supersedes**→ `adr` (0..1 → 0..1) — **locked, shipped** (M1). The one logic-free task→committed-doc handle the MVP proves ([worked-examples.md](../design/worked-examples.md) → Superseding decision).
- `commit` —**implements**→ `spec` (**0..1** → 0..1) — **activates with M3.** A task's commit references the spec it satisfies; the M3 `spec` worked example walks this edge at finalize. Cardinality is `0..1` on the commit side (not the `1` of first-draft intent), because spec-less tasks (`single-task`, `quick-fix`) still produce a valid `commit` — the edge is optional, set only by the spec-driven workflow.
- `spec` —**decided-by**→ `adr` (n → n) — a spec points at the decisions that shaped it. **Deferred past M3** — nothing in the M3 two-task arc writes or reads it, so it stays named-unscheduled intent until a workflow drives it (a one-line schema delta when it does).
- `prd` —**decomposes-into**→ `spec` (1 → n) — a PRD's requirements fan out into specs.
- `arch-doc` —**cites**→ `adr` (n → n) — architecture docs reference the decisions behind them.

Work-units (`task`, `increment`, `milestone`) are **not doctypes** — they're the anchors docs bind to (`task.commit`, `task.spec`, `task.decision`), carrying engine-native data-value roots, not schemas. The `milestone` work-unit activates in **M7** (the deterministic join core — [roadmap](roadmap.md)); its `{{milestone.tasks}}` data-value root lands in **M8** with the fan-out step that consumes it; `increment` minting stays deferred.

```text
prd ──decomposes-into──▶ spec ──decided-by──▶ adr ◀──supersedes── adr
                          ▲                     ▲
                     implements               cites
                          │                     │
                       commit               arch-doc
```
*(illustrative — edges, not a schema)*

## Unsettled

- **`prd` driver — settled M9.** The new-project idea-development setup workflow drives `prd`, earning its schema from that real creator ([project-setup.md](../design/project-setup.md) → Flow 1; [DECISIONS.md](../DECISIONS.md) 2026-06-06). M9's `prd` uses **fixed prose slots** (single-word section ids), so the `prd —decomposes-into→ spec` edge stays intent (no per-requirement anchors to author from) until a flow needs structured, individually-addressable requirements.
- **`arch-doc` driver — scheduled M13.** *Living architecture documentation* earns its schema from a general architecture-documentation workflow at M13 (doctype & surface completion), which also activates `arch-doc↔code` over M10's probe mechanism ([roadmap.md](roadmap.md) → M13; [DECISIONS.md](../DECISIONS.md) 2026-06-06).
- **doc↔code edges** (`spec`/`arch-doc` → code) are pack-provided validation *probes*, not structural relations — they land after the docs they check exist (post-M3), per [VISION.md](../VISION.md) → principle #6.
