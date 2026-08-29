# Changelog doctype — the first of the project-doctype expansion (+ the four engine lifts it earns)

**Status: settled at the doctype-expansion milestone's planning (2026-06-14); the locked spec the milestone decomposes against.** This is the first member of the **doctype-expansion** track (G2) split out of the paired post-M21 milestone ([DECISIONS.md](../DECISIONS.md) → 2026-06-14 split; auto-migration G1 is a *separate later* milestone). It ships one new managed doctype — **`changelog`** — driven by a real workflow, and **builds the four latent engine capabilities a faithful changelog earns** (rather than routing around them as every prior doctype did). The deliberate call ([DECISIONS.md](../DECISIONS.md) → this milestone's planning): stop deferring the capabilities; the point at which a real doctype needs them has arrived.

Reading-order note: this reads after [project-setup.md](project-setup.md) (it earns from a **Flow A** new-project authoring workflow that doc frames) and before [worked-examples.md](worked-examples.md) (whose **flow 24** consolidates its acceptance). It presupposes the schema mechanism ([document-type-schema.md](document-type-schema.md)), the create-gate + item-authoring verbs ([write-commands.md](write-commands.md)), and conformance validation ([validation.md](validation.md)); it **lifts** three opens those docs flag as deferred (multi-level repetition, optional slots/fields, doc-level `set: on-create`) and one parser defect ([parsing.md](parsing.md) — the multi-word section-id round-trip).

## What this milestone proves, and what it does not

**Proves:** a **new** project authors a managed, validated **`changelog`** *through* jigc — a singleton maintained over time, round-tripped byte-stable, validated and promoted at finalize — extending **Flow A's authoring reach** beyond the spine + methodology doctypes to a doc every real project keeps. And it lands the four engine capabilities the changelog's faithful shape genuinely needs, **each with an in-milestone consuming target** (so none is speculative generality — the bar every prior deferral was held to):

| Capability (latent defect lifted) | In-milestone target |
|---|---|
| **Multi-level repetition** (`Leaf::Repeatable`) | a `release → change-group` nesting (the core changelog shape) |
| **Multi-word section-id round-trip** (parser fix) | the changelog's `## Unreleased Changes` section (a genuine multi-word id) |
| **Optional slot/field** (`optional:` flag) | a release's optional `summary` slot (not every release has a headline) |
| **Doc-level `set: on-create`** (+ doc-level `default:`) materialization | honor `adr`'s long-broken doc-level `date` (set-on-create) **and** `status: proposed` (default) promises |

**Does not prove (honest bounds, recorded so the completion audit can't charge them):**
- **Flow-A only (for M22) — existing `CHANGELOG.md` files are not tracked by *this* doctype alone.** The `changelog` doctype helps a project that *authors its changelog through jigc from the start*. As of M22 a foreign project's existing `CHANGELOG.md` was **not** adoptable as-is: at repo root it classified `unmanaged` (outside every `location:` dir, conformant against nothing — left untouched, *not* `needs-reconcile`). **Making a foreign `CHANGELOG.md` trackable is auto-migration (G1), shipped as M23** ([auto-migration.md](auto-migration.md); [project-setup.md](project-setup.md) → Flow 2 hardening; [DECISIONS.md](../DECISIONS.md) → 2026-06-14 split + M23 planning) — the `jigc migrate` verb rewrites it to conformant shape (Framing A, boundary intact) and adopts it. **Post-M38 (placement, v2):** root `CHANGELOG.md` is now the doctype's own **canonical managed destination** (`placement: { file: CHANGELOG.md }`), so a foreign root `CHANGELOG.md` is an **in-location squatter at the canonical path** and, once conformant, **adopts in place** (seed/adopt, no retire — the M24 in-location-squatter path; [storage.md](../design/storage.md) → Placement, [DECISIONS.md](../DECISIONS.md) → 2026-07-04 M38), rather than classifying left-untouched-unmanaged.
- **The depth cap is exercised exactly, with nothing held behind it in reserve.** `Leaf::Repeatable` is built as **general recursion with a depth cap** (below), and the changelog uses precisely the two repeatable levels that cap admits — a schema declaring a third nesting level is **refused at pack-load** with a typed error, because the binding ceiling is the **address hop budget**, derived and stated once in [structural-grammar.md](structural-grammar.md) → Repetition. So a third level is deferred work behind a wider address grammar ([decisions-pending.md](../implementation/decisions-pending.md)), never an idle capacity this doctype declines to use (stated, not over-claimed).
- **Entries are prose, not addressable records.** A change-group's individual bullet lines live in a `notes` **slot** (prose), not a third repeatable level — faithful to Keep-a-Changelog (headings + bullets), and it keeps the nesting at the one level the milestone proves. Individually-addressable per-entry records stay future work.

## The `changelog` doctype

A **dev-pack** doctype (project-domain, beside `adr`/`commit`/`spec`/`arch-doc` — *not* methodology), **singleton** (one `CHANGELOG` per repo, fixed slug `changelog` homed at root `CHANGELOG.md` via `placement: { file: CHANGELOG.md }` — M38 relocation from the old one-file-in-a-folder home under `docs/changelog/`; the premise idempotent warm-append rests on), **edge-free** (see Relations). It is **below VISION's named starting set**, exactly as the methodology + `dogfood-record` doctypes are ([doctype-map.md](../implementation/doctype-map.md) — the "below the starting set" framing applies unchanged; no new VISION document-model claim).

**Shape (intent-level; the exact YAML is authored with the driver at build — the earn-from-driver discipline, [doctype-map.md](../implementation/doctype-map.md)).** Two sections, following the Keep-a-Changelog convention:

- **`## Unreleased Changes`** — a **multi-word section id** (`unreleased-changes`), the staging area for changes not yet cut into a version. A **single-level repeatable** of change-groups: each item is a `category` (enum: added / changed / deprecated / removed / fixed / security) carrying a `notes` slot (one bullet per change). *This section is the deliberate target for the multi-word-section-id fix* — it does not route around the defect as prior doctypes did; it fixes it.
- **`## Releases`** — a **two-level repeatable**: each item is a cut version (free-text item title = the version string, e.g. `1.2.0`; an item-level `date` `set: on-create`; an **optional `link` field** — the KaC comparison/diff URL, the optional-field target), and **each release nests a repeatable `changes`** of change-groups (same `category` enum + `notes` slot). *This nesting is the `Leaf::Repeatable` target.* The `date`'s `set: on-create` stamps **today** when *authoring* a new release (`record-change`); when **migrating** a *dateless* foreign changelog, the stamp is suppressed in migration mode so the release renders with no date rather than fabricating the migration day (no schema change — an absent `set: on-create` date finalizes clean; M24, [auto-migration.md](auto-migration.md) → Hardening).

**No leading prose slot on a release item — deliberately (review finding B1).** A release item holds **only scalar fields then the nested `changes` repeatable** — no bare-prose leaf. The earlier sketch's optional `summary` *slot* is dropped: a leading bare-prose slot's span runs to the next `###`/section boundary in the current parser, so it would **swallow** the nested `#### Added`/`#### Fixed` groups — an undefined parse boundary at the exact crux of the `Leaf::Repeatable` lift. Scalar fields avoid this entirely (they render in the `<!-- fields -->` block *before* any nested content), so the optional-field target (`link`) exercises the optional lift with **zero** prose-vs-nested contention. "Leading prose then nested items" (a depth-aware bare-prose boundary) stays a deferred capability ([decisions-pending.md](../implementation/decisions-pending.md)); the changelog does not need it.

Illustrative render of one release (notation illustrative — [document-type-schema.md](document-type-schema.md) is the source of truth for the on-disk grammar):

```text
## Releases

### 1.2.0  {#1-2-0}

<!-- fields -->
- date: 2026-06-14
- link: https://example.com/compare/1.1.0...1.2.0     ← the OPTIONAL `link` field (line omitted when absent)
<!-- /fields -->

#### Added  {#added}

- OAuth device-code flow

#### Fixed  {#fixed}

- session fixation on logout
```

Because a release item carries **no slot leaf** (only fields + the nested repeatable), there is **no `####` contention** between multi-slot leaf sub-headings (the M16 `#### <Leaf-Title>` rendering) and nested-repeatable item titles. Mixing multi-slot leaves *and* a nested repeatable in one item block — and the leading-prose-then-nested boundary — are deferred combinations the changelog does not need (stated below).

*The change-group shape (`category` enum + `notes` slot) is identical in both sections — `unreleased-changes` (single-level) and `releases.changes` (nested). The duplication is deliberate: the engine has no schema-fragment-reuse mechanism, and the staging-vs-released regions are genuinely domain-distinct (review finding S3).*

## The engine work (the four lifts; risk-first, the M13/M16 cold-start discipline)

Each lift carries the forward-review **red obligation** — the new behaviour must **fire-and-block / round-trip** on deliberately-exercised input before its green test (the *vacuously-green* failure class) — and spikes the **cold/empty** state, not a populated fixture.

1. **`Leaf::Repeatable` — multi-level repetition (the big one; build first).** Today `schema.rs` has `Leaf = Slot | Field` with no nested-repeatable variant ([decisions-pending.md](../implementation/decisions-pending.md); confirmed at planning). Add a `Leaf::Repeatable(Repeatable)` variant and make the consumers **recurse**, with a **heading-depth grammar**: a repeatable's items render at heading level `2 + nesting-depth` — section `##`, first-level items `###`, nested items `####` — capped at **2 nesting levels** (a documented cap, not silent truncation). *(M22 set that cap against the heading ceiling; since M49 it is derived from the **address hop budget** instead — the binding constraint — and stated once, in [structural-grammar.md](structural-grammar.md) → Repetition.)* The sites the planning audit mapped, each made depth-aware:
   - **parse** — `ItemTemplate::from`, the `###`-keyed item scanner (`parse_items`), `next_section_heading` (today only `##` bounds a section), and `is_reserved_depth` (today `H2|H3`): the item-region scanner must treat a deeper heading as a *sub-item* boundary within its parent item's region, recursing per the schema block.
   - **render** — `render_item` emits headings at the schema-derived depth and recurses into nested blocks; a nested item's `{#id}` anchor is deterministic from its `id-from` (a change-group's `category` enum value → `{#added}`, stable, one per parent).
   - **validate** — `check_repeatable` recurses, running `required-slot-present`/`required-field-present`/`field-value-conformant` over each nested item's leaves (exempting the `id-from` heading field), at every level.
   - **schema loader** — parse a nested `repeatable:` under a block leaf.
   - **addressing — a parent-scoped path locator, NOT a vocabulary extension (review finding S1).** Today `write::locate_item_block` matches an item **only at `###`, by anchor alone, globally over the whole source**, bounding at the next `### | ##` — so it cannot find a nested `####` item, would be **ambiguous** across parents (two releases each with an `#added` group), and would mis-bound the span. The lift **replaces it with a parent-scoped path locator** that resolves `item/subitem` by walking each path segment within its parent's byte region; `set_item_slot`/`set_item_field`/`add_item` and the `target_surface.rs` address builder all consume it, so `#section/item/subitem/leaf` (e.g. `changelog:changelog#releases/1-2-0/changes/added/notes`) resolves unambiguously. This is a first-class part of the LARGE lift, not a one-liner.
   - **anchor uniqueness is parent-scoped (review finding C1).** The duplicate-anchor `seen` check must reset **per parent item** during recursion: two releases may each legitimately carry `#added`, while a single release with two `#added` groups still blocks.
   - **Reds:** a hand-malformed nested entry (empty required `notes` slot under a change-group) **blocks** at `schema-conformance.required-slot-present` naming the **nested** leaf address; two `#added` groups **within one release** block (parent-scoped uniqueness) while `#added` **across two releases** passes; a two-level release **round-trips byte-stable** cold and warm.
   - This **graduates** the multi-level-repetition deferral ([decisions-pending.md](../implementation/decisions-pending.md)) and flips the [structural-grammar.md](structural-grammar.md) "one bounded level" open.

2. **Multi-word section-id round-trip (the ~1-line parser fix).** `parse::heading_matches` flat-compares the rendered heading against the raw section id, so a hyphenated id (`unreleased-changes` → heading "Unreleased Changes") fails and aborts the **whole-doc** parse ([decisions-pending.md](../implementation/decisions-pending.md); [parsing.md](parsing.md)). Fix: re-slug the heading text before comparing (`slugify(text) == section_id`), symmetric with the title-casing writer; no writer change. *(Free-text item titles are unaffected — they were never matched against a section id.)* **Red:** a `## Unreleased Changes` section round-trips; the pre-fix behaviour is pinned as the regression. **Graduates** the multi-word-section-id deferral, and **narrows** the deferred `prd` repeatable-requirements blocker (which awaited this fix).

3. **Optional slot / field (`optional:` flag).** `Slot`/`Field` carry no optionality; `check_slot_present`/`is_author_required` treat every declared slot (and every non-`set`/non-optional-ref field) as finalize-required ([decisions-pending.md](../implementation/decisions-pending.md)). Add `optional: bool` (serde default false, skip-serialize-on-false to keep existing schema goldens byte-stable, mirroring `singleton`) to **both** `Slot` and `Field`, and skip the requiredness check when set, at the enforcement sites (simple + repeatable arms, slot + field). **Target:** the release **`link` field** (the doctype-consumed exercise of the optional-*field* arm — a slot-arm red/green covers the optional-*slot* arm, same flag). **Reds:** an absent optional `link` finalizes **clean** while an absent required field/slot still **blocks**; the absent `link` renders with **no stray fields-block line** (byte-stable absent form — review finding C2). **Graduates** the optional-slot/field deferral.

4. **Doc-level `set: on-create` (+ doc-level `default:`) materialization.** `empty_instance` seeds **no** header/simple-section field values, so a doc-level `set: on-create` date or a `default:` value is never materialized at create — `adr`'s `date` (set-on-create) and `status: proposed` (default) are both silent no-ops today, the ADR's date a hardcoded test fixture ([decisions-pending.md](../implementation/decisions-pending.md)). Fix mirrors the proven **item-level** path: a CLI-side `on_create_doc_fields(schema)` at create walks header/simple fields for `type: date, set: on-create` (stamp `today_iso`) and `default:` (stamp the value), seeding the instance before render — **the engine stays clock-free** (the CLI owns the clock, exactly as item-level dating does). **Target:** `adr` (its `date` + `status` now materialize). **Cost flagged:** many tests pin empty front-matter on `create`; they update with this lift (the deferral predicted this churn). **Red:** a freshly-created `adr` carries today's `date` + `status: proposed`; an `adr` create round-trips with materialized header. **Graduates** the doc-level `set: on-create` deferral *and* honors the `adr` doc-level date/default promise. **Decompose this as its own isolated increment (review finding S2)** — its target is `adr` (not the changelog), and its predicted front-matter-test churn must land *in isolation*, never interleaved with the LARGE `Leaf::Repeatable` work, so the milestone keeps a clean single verdict.

*Changelog dates its releases at the **item** level (`set: on-create` on the `releases` item `date`), which is already built and byte-stable — lift #4 is earned by `adr`, not by the changelog.*

## The driver — standalone maintenance workflow **+** the single-task fold-in

The changelog's driver is the sharp earn-from-driver question ([doctype-map.md](../implementation/doctype-map.md)). Two complementary paths, both **pack-local to the dev pack** (no cross-pack create):

- **Standalone `record-change` workflow** (dev pack, **`creates-task: true, selectable: false`** — the proven off-router authoring spine, [methodology-docs.md](methodology-docs.md) → the sub-task precedent): `allows-create: [{type: changelog, as: changelog}]`. Mints a task → **idempotent-create** the changelog singleton (cold mint or warm copy-in, the proven `roadmap`/`decisions-log` mechanism) → `add-item` a release (or an unreleased change-group) + its nested change-groups → `set-slot` the notes → finalize promotes byte-stable. Invoked `jigc start --workflow record-change "<what changed>"`. This is the maintenance/release flow — the deliberate "cut a release / record a change" path.
- **The `single-task` fold-in (pack-local, mirrors the proven `adr` pattern exactly).** The dev-pack `single-task` workflow gains `{type: changelog, as: change}` in its `allows-create` list — beside its existing `{type: adr, as: decision}`. So an everyday coding task that lands a user-facing change appends a changelog entry through the **same create-gate** it already uses for an ADR (idempotent-create → one batch `doc author` payload into `## Unreleased Changes`, its shape generated by the step's `{{schema:changelog}}` seam — M47 N9), and finalize promotes it in the task's commit. This is what keeps the changelog **maintained as work lands** (the M17 lesson: a managed doc only an explicitly-invoked workflow touches risks going stale). Additive to a pinned workflow — regression watch that the existing `single-task` ADR-create + superseding-decision acceptance still pass.

*(The cross-pack stretch — folding `allows-create:[changelog]` into the **methodology** pack's `dev-task` under `[dev ▸ methodology]` composition — is **deferred**: the pack-local `single-task` fold-in covers the dev-pack on-ramp, and cross-pack `allows-create` resolution is an unexercised M14 surface. It fires if a real composed-project flow needs methodology's dev-task to author a dev-pack doctype; logged to [decisions-pending.md](../implementation/decisions-pending.md).)*

## Relations (edges) — none managed, by deliberate bound

The changelog is **edge-free** (mirrors the methodology running-singletons' bound, [methodology-docs.md](methodology-docs.md) → Relations). The tempting edge — a changelog entry citing the `commit`/`task` that produced it — **fails the same way the methodology per-entry refs did**: (a) `commit` is **transient** (no `location:`, never persisted — there is no target for `ref-resolves` to walk), and (b) a per-entry managed ref inside a repeatable item needs the deferred `index.rs` repeatable-edge lift. So entries cross-reference in prose; the differentiator this milestone proves is **Flow-A authoring + maintained-over-time append + multi-level conformance**, not an edge-walk — a legitimate, different proof from the MVP's `supersedes`. Stated so the completion audit can't charge under-design.

## The candidate doctype set (design-only forward map)

This milestone **builds `changelog` only**; the rest of "a good doctype set for a real project" is **mapped, not built** — each future doctype earns its schema from its own driving workflow when that workflow is built ([doctype-map.md](../implementation/doctype-map.md), the earn-from-driver discipline). The map (recorded in [doctype-map.md](../implementation/doctype-map.md) → the candidate set):

| Candidate | Disposition | Why |
|---|---|---|
| **`changelog`** | **build now** | real driver (record-change + single-task fold-in); genuine structure; maintained over time |
| `contributing` | map-only (unscheduled driver) | plausibly a one-shot project-setup-class authoring flow; thin structure, weakly "maintained" |
| `runbook` | map-only (unscheduled driver) | a real ops/incident driver is plausible (procedures→steps, would want multi-level), but no workflow needs it yet |
| `SECURITY` / `license` / `code-of-conduct` | **out** | boilerplate or verbatim/selected text — no authoring *judgment*, no schema to earn (a selector, not a doctype) |
| `api-docs` | **out** | tooling-generated from code, not LLM-prose-in-slots |
| `release-notes` | **out** (fold into changelog) | duplicates `changelog`'s release prose unless a distinct-audience driver proves otherwise |
| **`README`** | **out (charter-locked)** | bespoke front-page prose, no schema-able recurring structure, no composing workflow — the canonical "no schema to earn" |

The honest line for the charter's "everything we need for a real project": **most real-project files are not managed doctypes** (they are boilerplate, generated, or bespoke) — saying so explicitly is the anti-over-inclusion discipline.

## Acceptance

[worked-examples.md](worked-examples.md) **flow 24** — cold-create → warm-append, the same two-run byte-stable shape as flow 19 (planning encode), now exercising the nested repeatable: run 1 creates the changelog and authors a release with nested change-groups; run 2 warm-re-creates (copy-in preserves the prior release) and appends a **new** release; reds cover (a) a half-authored nested entry blocking at conformance, (b) the optional `summary` finalizing clean when absent, (c) the multi-word `## Unreleased Changes` section round-tripping. The exact verb sequence is **spiked against the real binary at build** (the M16 discipline — exercise, don't infer), since the multi-level authoring path is net-new engine.

## What stays deferred (logged to [decisions-pending.md](../implementation/decisions-pending.md))

- **Auto-migration (G1)** — rewriting an existing `CHANGELOG.md` to conformant shape; **ships as M23** ([auto-migration.md](auto-migration.md), Framing A, changelog-first).
- **3rd+ repeatable level / individually-addressable entries** — the engine caps nesting at 2 nesting levels (the **address hop budget**; [structural-grammar.md](structural-grammar.md) → Repetition), which is exactly what `changelog` exercises, so a third level needs the address grammar widened before any schema edit; entries stay prose.
- **Multi-slot leaves + a nested repeatable in one item block** — the `####`-depth combination the changelog avoids (one slot per release).
- **The methodology `dev-task` cross-pack fold-in** — needs cross-pack `allows-create` resolution (M14 surface), unexercised.
- **The mapped candidates** (`contributing`, `runbook`) — each earns its schema when its driver is built.
