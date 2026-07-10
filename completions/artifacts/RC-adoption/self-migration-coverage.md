# Self-hosting coverage map — jigc's repo docs → jigc doctypes (2026-07-10)

Commissioned off the adoption trial's doctype-gap feedback ([trial-record.md](trial-record.md) → Doctype gaps): after 1.0 this repo migrates completely under jigc management. Read-only audit of the doc corpus against both packs' schemas + the migrate machinery. Planning input for the post-1.0 **doctype-completeness milestone**.

**Doctype universe:** dev pack `commit, adr, spec, prd, arch-doc, changelog` (frozen); methodology pack `commit`(shadow)`, roadmap, deferral-ledger, decisions-log, completion-record, dogfood-record, vision, research, idea, milestone-record` (freeze-exempt). **Migrate workflows shipped:** only `migrate-{adr,spec,prd,arch-doc,changelog}`; `migrate.rs` resolves `migrate-<doctype>` from the composed workflow set pack-agnostically (`crates/cli/src/migrate.rs:85-100`), so methodology migrate workflows Just Work once authored.

## Coverage table

| Doc family | Count | Target doctype | Fit | Migrate path? | What's missing / lost |
|---|---|---|---|---|---|
| `VISION.md` | 1 | `vision` | **Forced (heavy)** — schema has 3 slots + `grounded-in`; the file has 14 `##` sections (determinism-boundary table, document model, flows, …) | No | Needs a **split**: thesis/invariants/open-questions → `vision`, the rest → `reference` docs. Sits at the placement canonical path already (in-place adopt covers it); `grounded-in` would point at a research corpus this repo doesn't have |
| `DECISIONS.md` | 1 (462 dated entries, 5820 lines) | `decisions-log` | **Forced (structural)** — schema = `{title, date, why-slot}` items; the entries are multi-paragraph essays + framing prose with no slot | No | **Item-explosion blocker** (below). The placement move is the easy part |
| `implementation/roadmap.md` | 1 (39 milestones, 2005 lines) | `roadmap` | **Fit with loss** — milestones map onto `{title, proves, decomposition}`; arc-narrative interstitials have no home | No | Migrate workflow + accepted narrative loss |
| `implementation/decisions-pending.md` | 1 | `deferral-ledger` | **Good** — near 1:1 onto `{title, kind, trigger, date, body-slot}` | No | Migrate workflow |
| `ideas/*.md` | 25 | `idea` | **Good/mild** — internal `##` structure flattens into the one description slot | No | Migrate workflow |
| `design/*.md` | 26 | **none** (nearest: arch-doc) | **No fit** — arch-doc mandates components with *required* `implemented-by` + title-names-symbol; these are concept-structured design-of-record prose | n/a | **New `reference` doctype**. `worked-examples.md` (2558 lines) is the stress case |
| `implementation/` reference docs (parsing, module-layout, language-runtime, doctype-map) | 4 | partial | `module-layout.md` is the one plausible arch-doc; `language-runtime.md` forces into adr; parsing/doctype-map — no fit | partial | Rest → `reference` |
| `implementation/*-workflow.md` | 5 | **none** | Executable content already lives as methodology-pack workflow YAML; the `.md`s are the human rationale/handbook layer | n/a | → `reference` (or consciously retire in favor of the pack + describe, keeping rationale) |
| `completions/artifacts/*/VERDICT.md` | 6 | `completion-record` | **Forced** — findings tables map; "what shipped"/narrative has no slot | No | Migrate workflow + narrative loss. Note: ~33 milestones have **no VERDICT.md** — completion facts live only inside DECISIONS entries (blocker 3) |
| `completions/artifacts/**` other | ~163 | **deliberately none** | Clean as-is — the `owner-artifact` class completion/dogfood records point at | n/a | Stay unmanaged, referenced |
| `WHY-JIGC.md` | 1 | none (nearest: research) | Forced — a living multi-claim dossier vs one investigation | No | → `reference`, or accept the forced research read |
| `QUICKSTART.md` | 1 | none | User handbook | n/a | → `reference` |
| `READINESS-ASSESSMENT.md` | 1 | none (nearest: adr) | Superseded gate-adjudication record | migrate-adr exists | Forced-adr or freeze as unmanaged historical |
| `reviews/codex-2026-05-28/*.md` | 6 | none | Dated cross-model review passes, not milestone-keyed | n/a | Freeze as historical, or `reference` |
| `CHANGELOG.md` | **0 — doesn't exist** | changelog | n/a | moot | jigc shipped rc.1–rc.3 with no changelog; self-hosting **creates** it via `record-change`, not migration |
| milestone-record instances | 0 | milestone-record | n/a — machine-minted by the milestone verbs, never migrated | correct | Nothing |
| Tooling, CLAUDE.md, `.claude/`, TODO.local.md | — | out of scope | scripts / adapter territory / scratch | n/a | Nothing |

## (a) migrate-* workflows to author

1. `migrate-vision` (in-place placement adopt at `VISION.md`)
2. `migrate-roadmap`, `migrate-decisions-log`, `migrate-deferral-ledger` (running singletons; the migrate-changelog precedent generalizes)
3. `migrate-idea` (×25, one task each)
4. `migrate-completion-record` (×6 VERDICTs)
5. `migrate-reference` — for the new doctype; the volume driver (~40 files)
6. Optional: `migrate-research`; probably not `migrate-dogfood-record` (historical study records don't carry the schema's protocol counts — stay owner-artifacts)

No repo doc targets `spec`/`prd`/`commit`.

## (b) new doctypes needed

- **`reference`** — **the dominant gap, ~40 files** (26 design/ + ~7 implementation/ + QUICKSTART + WHY-JIGC + arguably reviews/). Shape: `id-from: title`, `location: reference/` (or per-area), an `overview` slot + a repeatable `topics` section `{title, body-slot}` (per-doc-varying section sets survive schema-driven parse — the prd.requirements pattern) + an **optional** per-topic `cites-code` anchor (unlike arch-doc's required one). No component structure, no title-names-symbol.
- Nothing else is demanded by self-migration. A study/trial-record doctype is not needed — the owner-artifact posture covers `completions/artifacts/**`.

## (c) structural blockers (where one-file→one-doc, one-LLM-task breaks)

1. **DECISIONS.md item-explosion** — 462 entries through per-item write verbs in one task: context-infeasible, fidelity-unreviewable; the `why` slot wants "a sentence or two," the entries are essays. Realistic paths: a deterministic bulk-import mechanism (entries are near-schema-shaped `## date — title` + prose), or **freeze as historical + start `decisions-log` fresh at cutover** (mirrors how M16 was adopted).
2. **VISION.md needs a split, not a migration** — one file → several docs (`vision` + reference docs); F11's re-opened fork, second witness.
3. **Completion-record history trapped in DECISIONS.md** — ~33 milestones have no per-milestone file; one-file→one-doc can't mint a record from a slice. Backfill by hand-driven create (reading DECISIONS as unmanaged context), or accept records only for the 6 VERDICT-bearing milestones.
4. **Deep heading nesting vs slot parse** — design docs use `###`/`####` freely inside conceptual sections; slot prose can't contain headings the schema-driven parse would claim. A reference migration must flatten/re-level — binds hardest on `worked-examples.md`, `validation.md`, `overrides.md`.
5. **Narrative interstitials** in roadmap/decisions-pending have no slot — accepted-drop or relocate to reference.
6. **Placement/location moves ride shipped mechanics but touch many paths** — singletons → `docs/*.md`; composed `[dev ▸ methodology]` applies the dev `docs-root`, so `ideas/` → `docs/ideas/` etc. unless a project-cascade `docs-root` delta pins the current layout (the M39 `config set docs-root` detect+route+move exists for exactly this).

**Bottom line:** ~34 files land cleanly-or-acceptably once ~6 methodology migrate workflows exist; **~40 files (the design/implementation reference corpus — the repo's largest doc surface) have no doctype at all** and hinge on `reference` + `migrate-reference`; two files (DECISIONS.md, VISION.md) are structurally beyond `jigc migrate` as shipped (bulk-import or freeze-and-start-fresh); ~170 artifact files correctly stay unmanaged.
