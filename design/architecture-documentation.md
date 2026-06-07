# Architecture documentation — the `arch-doc` doctype and its driving workflow

**Status: settled for M13 (2026-06-07).** `arch-doc` is the last unbuilt member of VISION's named doctype *starting set* (`commit · arch-doc · prd · adr · spec`); every other member ships. This doc is the design of record M13 decomposes against — the doctype schema, the workflow that creates and maintains it, the `arch-doc↔code` activation over M10's probe, and the `arch-doc —cites→ adr` edge. It earns its schema **from its driving workflow** (the M3-`spec` / M9-`prd` discipline — [doctype-map.md](../implementation/doctype-map.md): a doctype's schema is set by the workflow that creates and reads it, never defined ahead of one).

Reading-order note: this reads after [finalize.md](finalize.md) (it rides the create→author→code-less-finalize→promote spine) and after [validation.md](validation.md) (it activates the `doc-code` probe and adds the per-field-type predicate selector), and before [introspection.md](introspection.md). The end-to-end acceptance is [worked-examples.md](worked-examples.md) → flow 16.

## What `arch-doc` is

**Living architecture documentation**: a persisted doc describing one part of the system — its overview, the components that compose it (each tied to the code that implements it), and the decisions behind it. Per [VISION.md](../VISION.md) → Document model, it is a managed document with a single clear purpose and explicit cross-reference obligations; the file mass is a non-problem because the CLI assembles views on read and places content on write.

The "richest doc↔code case" the roadmap names ([roadmap.md](../implementation/roadmap.md) → M13): unlike `adr` (one optional `cites-code` anchor in a header) or `spec` (one `maps-to-test` per criterion), an `arch-doc` carries **one code-anchor per component** — every architectural component points at its implementing code, and `finalize` blocks if that code has vanished. That is principle #6's *validate against reality* turned on architecture prose: the diagram can't quietly drift from the code it claims to describe.

## The schema

`arch-doc` is a **persisted** doctype (`location: architecture/`), `id-from: title` (the H1, never a declared field — exactly like `adr`/`spec`). Its shape:

```yaml
type: arch-doc
location: architecture/
id-from: title
description: Living architecture documentation for one part of the system — its overview, its components and the code that implements them, and the decisions behind it.
usage: a part of the system is worth a durable description that stays honest against the code, so a later reader (or agent) can orient without reverse-engineering it.

sections:
  - id: meta
    header: true
    fields:
      - { id: cites, type: ref, to: adr, card: "0..*", inverse: cited-by }
  - id: overview
    slot: { hint: "What this part of the system is and the boundary it owns." }
  - id: components
    repeatable:
      id-from: title
      block:
        - { id: title, type: string }
        - { id: description, slot: { hint: "The component's responsibility, in prose." } }
        - { id: implemented-by, type: code-anchor, check: symbol-exists }
```

Three things make this shape work on the built substrate, each **exercised at planning** (2026-06-07 spike, HEAD `e983dde`) rather than assumed:

- **`cites → adr` lives in the header (a Simple section), n→n.** A list-valued `ref` (`card: "0..*"`) parses to an ordered list and the edge index emits **one edge per element**; `ref-resolves` walks each at finalize, blocking the dangling target and passing the present one. A ref in a *repeatable* section emits no edge (an MVP non-target) — so `cites` stays doc-level, where the walk is the proven `supersedes`/`implements` path. Section ids stay **single-word** (`meta`/`overview`/`components`), sidestepping the latent multi-word-heading round-trip defect ([decisions-pending.md](../implementation/decisions-pending.md) — still deferred; arch-doc does not trip it).
- **`components` is a repeatable section**, each item addressed `arch-doc:<slug>#components/<id>/…`. This is the first doctype to *author* repeatable items through the binary — the capability `spec.criteria` deferred. It forces the item-authoring surface below (`add-item` + item-leaf setters).
- **`implemented-by` is a per-component `code-anchor` with an explicit `check: symbol-exists`.** This is the one genuinely-new engine concept M13 introduces — see next.

## The per-field-type predicate selector (`check:`) — the M13 engine concept

M10 chose the `doc-code` predicate **by position**: a `code-anchor` in a Simple section gets `symbol-exists` (does the cited code exist?); one in a repeatable section gets `criterion-maps-to-test` (does it resolve to a real `#[test]`?). That was sufficient when the only repeatable anchor was `spec.criteria`, which genuinely wants the test predicate. **arch-doc breaks the coincidence**: its repeatable `components` carry anchors that cite *implementation* code, which want `symbol-exists`. Position no longer discriminates.

The fix keeps the engine empty and the predicate **pack-declared**, on the same axis the adjudicator already rides. The resolution is **pinned, not a build-time call** (the absent-default is the trap — pick it wrong and shipped `spec.criteria` silently loses `criterion-maps-to-test`, the VISION headline check):

- The pack field-type declaration carries the type's **check** explicitly (no implicit engine default): `{ name: code-anchor, adjudicator: doc-code, check: symbol-exists }` in `pack/config/field-types.yaml`.
- A schema **field** may carry an optional `check:` that **overrides** the type's check for that one field.
- **Resolution order:** `field.check` if present, else the resolved field-type's `check`. `target_surface.rs` reads the check id from this resolution at **both** enumeration sites (`collect_simple`, `collect_repeatable`) — the two positional constants (`SYMBOL_EXISTS` / `CRITERION_MAPS_TO_TEST`) are deleted.

So: `adr.cites-code` and `arch-doc.components/implemented-by` are bare `code-anchor` → inherit `symbol-exists`; **`spec.criteria/maps-to-test` declares an explicit `check: criterion-maps-to-test`** to keep its shipped predicate. Because position no longer chooses, the selector increment **must edit `spec.yaml` to carry that override**, and its green-bar is **both** [worked-examples.md](worked-examples.md) flow 13 (spec — `criterion-maps-to-test` preserved, blocks a deleted *test*) **and** flow 16 (arch-doc — `symbol-exists` on a repeatable anchor), not flow 16 alone. Rejected alternatives: a pure positional flip (breaks `spec.criteria`), and keying the predicate on doctype (hardcodes domain knowledge into the engine, violating engine-empty, and breaking the moment one doctype wants two predicates). This is **real engine work**, not free reuse — `PackFieldType` and the schema `Field` each widen by one optional field and the two enumeration sites change their predicate source; see [validation.md](validation.md) → The `doc-code` probe (M10) and [document-type-schema.md](document-type-schema.md) → Pack-declared field types for the fold-back.

## The architecture-documentation workflow

A `creates-task: true` work-workflow, listed by the router with a `when` hint, that authors an arch-doc through the create-gate and finalizes it code-less:

```yaml
---
when: document the architecture of a part of the system, tying its components to the code that implements them
description: Authors living architecture documentation for one part of the system and commits it.
usage: a part of the system needs a durable, code-checked description so a later reader can orient without reverse-engineering it.
creates-task: true
allows-create: [{type: arch-doc, as: arch-doc}]
---
{{ include: step:author-arch-doc }}
{{ include: step:finalize }}
```

The `author-arch-doc` step walks the agent through: `jigc doc create arch-doc --title "…"` (mints `arch-doc:<slug>`, bound to `task.arch-doc` by the `as:` entry); fill `overview` and the doc-level `cites`; then, **per component**, `jigc doc add-item arch-doc:<slug>#components --title "…"` followed by `set-slot …#components/<id>/description` and `set-field …#components/<id>/implemented-by --value <path>#<symbol>`. The step is principle-first prose (it instructs the agent to add one component per architectural piece and anchor each to its implementing code) — the structural ops are the agent running the named CLI commands, per the determinism boundary; the CLI owns placement and the code-anchor adjudication.

`finalize` is the proven code-less path: it validates (required slots/fields present, `ref-resolves` over `cites`, `doc-code` over each `implemented-by`), renders the `commit` doc to the git message, **promotes** `arch-doc:<slug>` to `architecture/<slug>.md`, and lands one `docs(arch-doc):` commit — the generic `plan_promotions` loop that already serves `decisions/`, `specs/`, `prds/` (a fourth `location:`, zero new finalize code).

### "Living" = create-only authoring + the git-editable channel

A workflow run **creates** a fresh arch-doc; it does not re-open a committed one. "Living" is honoured two ways without a new control-flow shape: the committed file is plain human-editable markdown, so the **OOB-reconcile channel** ([reconciliation.md](reconciliation.md)) absorbs hand-edits (and `arch-doc↔code` re-checks them at the next task touching the doc), and re-documenting a changed part of the system is a fresh authoring pass. The create-or-*update*-via-bind path (re-open a committed arch-doc, re-slot, re-promote) is deliberately **out of M13** — it is an unexercised shape (the persisted-doc pattern has only ever *created*; re-promoting an already-committed doc through finalize is unverified), and pulling it in would add a front-matter combination (`allows-create` + a required `reads:` self-role) that no shipped workflow carries. Deferred, with its trigger, to [decisions-pending.md](../implementation/decisions-pending.md). This keeps M13 on the exercise-verified spine.

## `arch-doc↔code` — the richest doc↔code case

Each component's `implemented-by` is enumerated into the task's effective-state target surface, materialized into the `doc-code` probe snapshot with `check_id: symbol-exists`, and resolved against the working tree by the real `doc-code` subprocess. The **enumeration and probe ends were each exercised at planning** (enumeration reaches the repeatable item; the real `doc-code` subprocess checks an item anchor and blocks a dangling `path#symbol`, passes a present one) — the **full `finalize` snapshot-spawn over a repeatable-item anchor is the acceptance increment's first real proof** (the orchestrator that writes the snapshot and spawns the subprocess during `finalize` is golden-tested for the shape but unexercised for an item anchor specifically). The probe is Rust-grammar-only ([validation.md](validation.md)); M13's acceptance points arch-doc's anchors at **jigc's own Rust codebase**, which sidesteps that limit cleanly (a non-Rust target is out of scope, as for M12). No new probe, snapshot, or enumeration work — only the `check:` selector above, so a `components` anchor gets `symbol-exists` rather than the test predicate.

## The relations triage (surface completion = nodes *and* edges)

M13 closes the named doctype graph, so the still-open edges are triaged under the **no-edge-without-a-driving-workflow** gate ([doctype-map.md](../implementation/doctype-map.md)):

- **`arch-doc —cites→ adr` (n→n) — IN.** Driven by this workflow (authored as the doc-level `cites` field), walked at finalize. The one edge M13 adds.
- **`spec —decided-by→ adr` (n→n) — OUT (re-deferred).** No workflow in M13's scope writes or reads it; a one-line `spec` schema delta when a spec-authoring flow earns it. Triaging it in would be the single-use generality the gate forbids.
- **`prd —decomposes-into→ spec` (1→n) — OUT (hard).** No driver, and structurally blocked: it needs per-requirement anchors (the deferred `prd` repeatable-requirements shape), which itself awaits the item-authoring surface and the multi-word-heading fix. Stays intent.

## What M13 builds (cross-ref the decomposition)

The repeatable-component shape pulls a bounded engine surface into M13 — all sized by the planning spike, decomposed risk-first in [roadmap.md](../implementation/roadmap.md) → M13:

1. **The per-field-type `check:` selector** (engine; cross-cutting — touches the `doc-code` predicate, must keep `spec.criteria` green; retired first in isolation).
2. **Item-scoped engine setters** `set_item_slot(schema, source, section_id, item_id, prose)` / `set_item_field(schema, source, section_id, item_id, field_key, value)`. **The contract is pinned** because the existing setters target the *wrong item* for a repeatable section: `locate_field_value` scans **globally** and returns the *first* matching `key:` line (so every component's identically-keyed `implemented-by` would resolve to component #1), and `locate_slot_span` finds only a *section-level* `slot`, never a per-item one. The new setters **(a)** locate the item via the existing item-scoped `locate_item_block(source, item_id)` region, **(b)** restrict the field/slot scan to *that byte region only* (thread `item_id`→region into the locator — `field_value_in_lines` already takes a region bound, so this is threading, not new parsing), reusing the span-generic `splice`. `set_field` / `set_slot` / `set_field_validated` are **not** reused as-is for item leaves (their global scan / section-only slot lookup is the bug). Green-bar: a **multi-item golden** where item B's leaf is set and item A's identical-keyed leaf is provably untouched.
3. **CLI item-leaf addressing + the `add-item` verb** — wiring over the proven engine `add_item`; `set-slot`/`set-field` learn the `#section/item/leaf` fragment.
4. **The pack + design** — the `arch-doc` schema, the `architecture-documentation` workflow + `author-arch-doc` step, the `create-arch-doc` command-ref row, this doc, and the doctype-map/validation/VISION fold-back.
5. **Acceptance** — [worked-examples.md](worked-examples.md) → flow 16.

## The acceptance flow (flow 16 — the bar)

A single task documents one part of jigc's own architecture: create an `arch-doc`, author the `overview`, then add two `components` carrying **two *different*, independently-resolving** `implemented-by` anchors (e.g. `crates/engine/src/index.rs#…` and `crates/engine/src/target_surface.rs#…`), and cite a committed `adr`. The first `add-item` must materialize `## Components` at its **schema-ordered position** (after `## Overview`) — a named acceptance check (the create-path the engine has not produced before). Then the **blocking** proof (a happy-path-only acceptance would mask): delete **component A's** symbol while component B's stays valid → `finalize` blocks on `doc-code.symbol-exists` **naming A's specific item address** (`arch-doc:<slug>#components/<a-id>/implemented-by`), proving each item's anchor resolves against *its own* authored value, not a clobbered shared one; and the `cites` lists a **dangling** adr → `finalize` blocks on `ref-resolves`. Fix both (anchor a present symbol, cite a real adr) → `finalize` promotes `architecture/<slug>.md` and lands one `docs(arch-doc):` commit. This walks every M13 surface: repeatable-item authoring with per-item disambiguation, the `check: symbol-exists` selector on a repeatable anchor, the n→n `cites` edge, and code-less promotion to a fourth `location:`.

## Honest caveats / carried unverified-reuse (retire in-build)

- **Provisioning an empty repeatable home.** `create arch-doc` must yield a doc that `add-item` can append into; the engine `add_item` self-materializes the `## Components` section home, so the risk is low — but verify the create→add-item handoff before relying on it (M10's provisioning-default-materialization note).
- **Finalize snapshot-spawn for an item anchor end-to-end.** Both seam ends are exercised (enumeration output, real probe I/O); the orchestrator that writes the snapshot file and spawns the subprocess during `finalize` is golden-tested for the shape but unexercised for a repeatable-item anchor specifically — confirm in the acceptance increment.
- **`add-item` ergonomics** — mint-empty-then-fill (less new surface) vs inline title+slot handoff; a build-time call.
- **Multi-word section ids** stay deferred — arch-doc's single-word section ids route around the latent parser defect, exactly as `prd` did. (Multi-word *component titles* on `###` item headings are fine — free text, not matched against a section id.)
