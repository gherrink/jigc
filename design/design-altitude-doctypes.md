# The design-altitude doctypes — `vision` · `research` · `idea` (M37)

The methodology pack's design-altitude trio: the three doctypes that let a project's
*design* work — not just its execution — be a managed, composable surface. **Research
feeds vision-forming; the vision is the compare-against anchor; shaped-but-unscheduled
directions park as ideas.** This is the human's own working practice ([VISION.md](../VISION.md),
[CLAUDE.md](../CLAUDE.md) → conventions, `ideas/`), lifted from unmanaged house convention
into the methodology pack so any project inherits it — driven by the greenfield RC trial,
which consumes it (the purpose map's first driver arrival; [doctype-map.md](../implementation/doctype-map.md)
→ The purpose map; [DECISIONS.md](../DECISIONS.md) → 2026-07-03 M37 charter, 2026-07-04 M37 planning).

Reads after [multi-pack.md](multi-pack.md) (it adds three methodology doctypes + the pack's
first internal managed ref, so the composition surface must be in hand) and before
[measurement.md](measurement.md).

## What M37 settles that the charter left open

The charter flagged five forks and expected *pack-only, zero engine change*. Planning
settled all five and **consciously broke the zero-engine-change expectation** on one — the
human chose the robust vision surface (§4), which is real engine + CLI work. The
"halt-on-engine-need" gate fired as designed ([roadmap.md](../implementation/roadmap.md) → M37;
the settled-going-in rule). The decisions:

| Fork | Decision | §  |
|---|---|---|
| 1 · vision-forming workflow shape | **Shape 2** — a `do-research` workflow authors+commits `research` standalone; a separate `form-vision` workflow reads the *committed* research and sets `grounded-in`. (The one-flow in-task content-slice is **verified broken** against the binary — reading a same-task, uncommitted sibling's content is an unbuilt engine lift; Shape 2 rides the proven committed-read path.) | §3 |
| 2 · vision location / surface | **Full robust** — managed home `docs/vision/vision.md`; a per-doctype **display-title knob** so its H1 reads `# Vision`; a deterministic **finalize render** to a root `VISION.md` (the changelog-render mechanism, [ideas/root-changelog-render.md](../ideas/root-changelog-render.md), built here for `vision`). Engine + CLI work. | §4 |
| 3 · open-questions: prose vs `tracks → idea` edge | **Prose slot** — forced by scope: mechanizing needs a ref *inside a repeatable item*, an engine lift explicitly deferred ([methodology-docs.md](methodology-docs.md) → the repeatable-ref lift). Building it would violate the (already-broken-elsewhere) engine-change bound in a direction the trajectory does not commit to. | §2 |
| 4 · idea de-park trigger; relation to `deferral-ledger` | **Coexist** — `idea` is a standalone doctype with a `trigger` string (the `deferral-ledger.trigger` shape, not a new enum-status); `deferral-ledger.kind=I` is **unchanged**. Two distinct homes: owed *decisions* (ledger) vs shaped *directions* (idea). | §2 |
| 5 · `park-idea` routing | **Router-selectable** (`selectable: true` + a `when:` hint) — parking must cost less than losing the thought, so it must be discoverable, not off-router. | §3 |
| GF5 · methodology `commit` stale fork | **Fix now** — add `optional: true` to `scope` + `body` (the M26 finalize-wall fix the methodology fork missed); graduate [decisions-pending.md](../implementation/decisions-pending.md) → the line-109 entry. `implements→spec` stays dropped (deliberate). | §5 |

**Naming principle binds every name below** ([doctype-map.md](../implementation/doctype-map.md)
→ The purpose map): ecosystem-common terms the AI already knows, never house inventions.
So the `idea` body is `description` (not the house term "shape"), and "de-park trigger"
surfaces as the plain field `trigger`. The retained terms — `thesis`, `invariants`,
`open-questions`, `question`, `findings`, `sources`, `grounded-in`/`grounds` — are all
standard English/technical terms an AI knows for these purposes (§6 records the audit).

## 1. Where the three sit in the methodology pack

All three are **methodology-pack** doctypes (the human's call: forming a vision is
*how-you-work*, not code-specific; and it keeps the `grounded-in` edge **pack-local**,
off the recorded unexercised cross-pack surface). At M37 they sat **outside every freeze**,
the methodology pack shipping no manifest of its own — **that ended at M40**, when the pack
adopted its own `config/schema-manifest.yaml`: all three are frozen there at
`schema-version: 1`, so a shape change to any of them bumps that version and ships a corpus
migration exactly like a dev-pack doctype ([doctype-map.md](../implementation/doctype-map.md)
→ The v1 freeze, scope-pin — the one home for what each pack's manifest governs).

`vision —grounded-in→ research` is the **methodology pack's first internal managed ref**
of any kind. It honors — does not reopen — the M16 "methodology composes alone, no managed
ref" bound ([methodology-docs.md](methodology-docs.md)): that bound's *rationale* was "a
ref would point **outside** the composed schema universe" (to dev-pack `adr`/`spec`).
`grounded-in` is intra-pack (`vision → research`, both methodology) and **doc-level** (a
header field, not a per-repeatable-item ref), so it points *inside* the universe and does
not touch the deferred repeatable-ref lift. The engine edge index / `ref-resolves` walk is
schema-driven and pack-agnostic by construction, so the reuse is sound — but it has never
run in a methodology-pack composition, so the acceptance flow (§7) is its first real driver.

## 2. The three schemas

Section ids are single words or the post-M22 multi-word-safe form (`open-questions` reparses
correctly — the pre-M22 single-word-only constraint in the older methodology schemas'
comments no longer binds). All field types are engine-native + the existing `ref`.

### `vision` — singleton living reference (`docs/vision/vision.md`)

```yaml
type: vision
location: vision/
singleton: true
id-from: title
display-title: Vision   # display-title knob (§4) — the singleton H1 reads `# Vision`
root-render: VISION.md  # finalize render-to-root knob (§4)
description: The project's charter — its thesis, the invariants it holds, and its open questions — grounded in the research it was formed from.
usage: the project's direction is worth a durable, managed anchor that later work compares against, formed from and traceable to the research behind it.
sections:
  - id: meta
    header: true
    fields:
      - { id: grounded-in, type: ref, to: research, card: "0..*", inverse: grounds }
  - id: thesis
    slot: { hint: "The core claim — what this project is and the one idea it descends from." }
  - id: invariants
    slot: { hint: "What must stay true — the boundaries and commitments no change may violate." }
  - id: open-questions
    slot: { hint: "What is deliberately unsettled — the directions still open." }
```

**`singleton: true` fixes the identity, not only the H1.** The `display-title:` knob in the
block above is the *visible* half of one fact about a singleton: both the doc's id and its
heading are the schema's, never the author's. The id half: `vision`'s slug **is** the type id,
so the doctype has exactly one address (`vision:vision`, which the bare head `vision` expands
to), one home, and no `<slug>` for a caller to choose. Every door asks one predicate for this
— `placement.is_some() || singleton` ([storage.md](storage.md) → Placement) — and refuses a
write, rename or read under any other slug with `store.fixed-identity`. Consequence for the
pack's own prose, and the reason it is stated here: `jigc doc schema vision` is a
**type-level** projection, so it advertises `vision:<slug>#…` like every other doctype and is
therefore the authority on this doctype's slots, fields and enum members but **not** on its
address — the `identity` key of the pinned `--format json` projection is (M52 Increment 6; the
fence that holds the author steps to it is `crates/cli/tests/fixed_identity_axis.rs`).

`invariants`/`open-questions` are **prose slots, not repeatable sections**. Unlike the
`prd` fixed→repeatable migration (M25 rework), this carries no debt: methodology is
freeze-exempt (graduating a slot to a repeatable section later needs no version bump), and
**no M37 consumer reads an individual invariant or question**. Prose is the minimal-*correct*
cut, not a cheap one. *(M40 falsifies the no-version-bump half: with the methodology manifest
landed, the deferred slot→repeatable graduation now costs a version bump + corpus migration —
a priced cost of joining the version-gate world; [DECISIONS.md](../DECISIONS.md) → 2026-07-10
M40 Settle.)*

### `research` — one-per-doc, append-only record (`docs/research/<slug>.md`)

**One-per, not a singleton.** The class label "append-only record" pattern-matches to the
methodology singletons (`decisions-log`/`deferral-ledger`), but `grounded-in` is `0..*` —
which is meaningful only if there are *many* research docs. A singleton would collapse the
edge to `0..1` and break the comparison anchor. So `research` mints fresh per document
(`id-from: title`, `location: research/`) — the methodology pack's **first one-per-doc
persisted doctype**. It **must** carry a real `location:` or every `grounded-in` edge
dangles at finalize (a location-less target is always unreachable).

```yaml
type: research
location: research/
id-from: title
description: One investigation and what it found — the evidence a vision or design is formed from.
usage: a question needs evidence gathered and recorded before a vision or decision rests on it, and that record should stay addressable by what it grounds.
sections:
  - id: meta
    header: true
    fields:
      - { id: date, type: date, set: on-create }
  - id: question
    slot: { hint: "The question this research set out to answer." }
  - id: findings
    slot: { hint: "What was found — the evidence and what it shows." }
  - id: sources
    slot: { hint: "Where the evidence came from — links, references, prior art." }
```

### `idea` — one-per-doc, parked shaped direction (`docs/ideas/<slug>.md`)

Coexists with `deferral-ledger.kind=I` (unchanged): the ledger holds owed *decisions*;
`idea` holds shaped *directions*. `trigger` is the de-park condition, the
`deferral-ledger.trigger` shape (a plain string naming what would resurface it) — **not** a
new enum-status axis.

```yaml
type: idea
location: ideas/
id-from: title
description: One shaped-but-unscheduled direction, with the trigger that would bring it back.
usage: a direction is worth keeping but not worth scheduling now, so it needs a durable home cheaper than losing the thought and a note of when to revisit it.
sections:
  - id: meta
    header: true
    fields:
      - { id: trigger, type: string }
      - { id: date, type: date, set: on-create }
  - id: description
    slot: { hint: "The shaped direction — what it is and why it might be worth doing." }
```

## 3. The driving workflows (Shape 2)

Three router-selectable methodology work-workflows (`creates-task: true, selectable: true`,
each with a `when:` hint). Each mints a task and lands one commit through `finalize`.

- **`do-research`** — `allows-create: [{type: research, as: record}]`. Authors a `research`
  doc (question/findings/sources) and finalizes. `when:` "you need to investigate or gather
  evidence before forming a vision or making a decision." Research stands alone and commits
  here, so `form-vision` can read it committed.
- **`form-vision`** — `allows-create: [{type: vision, as: vision}]`. Creates/updates the
  `vision` singleton (idempotent copy-in on re-entry), sets `grounded-in` to the committed
  research it rests on, and authors thesis/invariants/open-questions. `when:` "you're forming
  or revising the project's vision from research."

  **The load-bearing output is the *stored* `grounded-in` anchor** — a `0..*` ref, so a vision
  may be grounded in *several* research docs; every element is `ref-resolves`-validated and the
  `grounds` inverse is derivable. This is fully multi-valued and is what "the compare-against
  anchor" means. Separately, the author step *optionally* reads one grounding research's findings
  into its guidance via the **proven edge-walk slice** `{{@task.vision.grounded-in#findings}}` —
  identical in shape to the shipped `@task.decision.supersedes#decision` superseding-context slice
  (walk a ref edge from the in-task doc to a *committed* target, slice a section). **Two constraints
  the build must honor, both from the superseding precedent, both design-review findings:**
  1. **Ordering (re-compose required).** The slice resolves at *compose* time; at first `jigc start`
     the vision does not exist and `grounded-in` is unset, so it resolves empty. So `form-vision`
     is a **re-entry flow**, exactly like the superseding worked example ([worked-examples.md](worked-examples.md)
     → flow 5, lines 257–285): create vision → `jigc doc set-field …grounded-in` → **re-compose**
     (`jigc start --task <id>`) → author using the now-populated slice. A single create→set→author
     pass would author against an empty read (a vacuous green).
  2. **The content-echo surfaces *all* grounded research.** `walk_edge` fans out to *every* matching
     edge target (`index.rs`), so the slice renders each bound research doc's `findings` as its own
     labelled `> ` blockquote — the full grounding set, not just one. *(This was originally documented
     as a first-only echo, on the basis that a single-source convenience read sufficed and a true
     all-N read was an out-of-scope engine lift. The M39 RC greenfield trial falsified that basis: a
     vision grounded in two research docs must re-compose **both** findings for the compare-against
     read to do its job — so the all-source fan-out is the built behavior, M39 inc-2.)* The stored
     `grounded-in` refs remain the load-bearing anchor; the echo now mirrors them in full. The emitted
     form is byte-identical to the superseding-context precedent when a single research grounds the
     vision (one bare blockquote, N==1); N≥2 labels each source by its `<type>:<slug>` identity.
- **`park-idea`** — `allows-create: [{type: idea, as: idea}]`. Mints one `idea` (description
  + trigger) and finalizes. `when:` "a shaped-but-unscheduled idea occurs mid-work and is
  worth keeping." Router-selectable so it is discoverable (fork 5).

New command-refs in `packs/methodology/config/commands.yaml`: `create-research`,
`create-vision`, `create-idea` (the `{{cli.*}}` catalog the author steps surface).

**Acceptance ordering (Shape 2):** `do-research` (task 1) → commit `research:<slug>` →
`form-vision` (task 2) → set `grounded-in=[research:<slug>]`, author, finalize. This is the
two-task committed-target shape the superseding worked example already proves; the
`grounded-in` edge and the edge-walk slice both resolve against the committed research.

## 4. The vision surface — the engine + CLI work (fork 2, full robust)

> **⚠ Superseded by M38 (2026-07-04).** This section designed the vision's root legibility as a **`root-render` mirror** (managed source at `docs/vision/vision.md`, a regenerated `VISION.md` artifact at root). M38 **retires `root-render`** and makes the vision *managed directly at root `VISION.md`* — one file, proper reconciliation, no mirror — via the placement convention ([storage.md](storage.md) → Placement; [DECISIONS.md](../DECISIONS.md) → 2026-07-04 M38 planned). The `display-title: Vision` knob (§2) survives; the `root-render:` knob, the finalize render, and the bespoke foreign-file guard below are all retired (the general `plan_clobber_guard` inherits the no-data-loss role). Read this section as historical rationale for *why* the vision belongs at root — the *mechanism* is now placement, not render.

Two capabilities, both verified absent today (a singleton's H1 is hardcoded to its lowercase
type-id in `state.rs`; there is no declarative render-to-file knob — the `commit` git-message
sink is bespoke `render_commit_message`). Both are **additive schema-format keys used only by
freeze-exempt methodology doctypes**, so the six frozen dev-pack doctypes' hashes stay
unchanged and the freeze assertion stays green — **but only if the new fields serialize-skip
when absent.** `schema-hash` is `blake3(serde_json(schema))`; a bare `Option<String>` would
serialize `"display-title":null` into every frozen doctype's JSON and change all six hashes.
So both fields **must** carry `#[serde(default, skip_serializing_if = "Option::is_none")]`,
matching the existing `location`/`id-from` pattern (`schema.rs`). Verified, not assumed (§7 arm 5).

- **Display-title knob (`display-title:`).** An optional doctype-level string, **orthogonal to
  `id-from`** (display-only, never an id-source — the name avoids colliding with the `id-from: title`
  token). When present, it overrides the H1 display text — narrowing the `state.rs` singleton
  branch that today forces `title = slug`. `vision` declares `display-title: Vision`, so the
  managed doc reads `# Vision`, not `# vision`. Absent → today's behavior (H1 = id-source / slug),
  so no frozen doctype changes.
- **Root render (`root-render:`).** An optional doctype-level path. At finalize, after the
  managed doc is promoted, the CLI writes the doc's canonical bytes to that repo-root path and
  commits it with the task. The **managed doc stays source of truth**; the root file is a
  **deterministic regenerated artifact** (the [root-changelog-render](../ideas/root-changelog-render.md)
  shape, built here for `vision`) — regenerated each finalize from the managed doc. No
  `location:` change; no bump-computation facet (that stays a changelog-only parked idea).
  - **Non-destructive on first write (no silent data loss — a hard invariant, [VISION.md](../VISION.md)
    principle #3 / [storage.md](storage.md)).** A hand-authored root `VISION.md` is *exactly* what a
    convention-following project already has (this repo does). The render must not clobber it. The
    guard uses the managed doc's existence as the "jigc owns this render" signal: if the managed
    `docs/vision/vision.md` does **not** yet exist (first `form-vision`) **but** a root `VISION.md`
    **does**, finalize **blocks and routes** ("a `VISION.md` exists that jigc did not generate —
    adopt its content into the vision doc or remove it, then re-run"), never overwriting. Once jigc
    manages the vision (managed doc present), the root render is jigc's own regenerated output and
    is overwritten freely — an out-of-band edit to *that* generated file is overwritten at the next
    finalize (edit the managed doc, not the render). This is the greenfield case (no root file → just
    write) and the existing-project case (foreign root file → route), the RC trials' two on-ramps.

**Chosen mechanism: a declarative doctype knob, not a bespoke vision hardcode.** Config-driven
over special-casing is jigc's idiom, and it avoids a second bespoke branch accreting in finalize
next to the commit sink. The `commit` sink stays bespoke because it renders to git, not a file;
a file render generalizes cleanly. **The *engine mechanism* is reusable, but applying it to the
frozen-v1 `changelog` is not free** — `root-render:` in a schema file is a schema-shape change, so
adding it to `changelog.yaml` would cost a v1→v2 bump + corpus migration ([root-changelog-render](../ideas/root-changelog-render.md)
would need re-costing, or `root-render` relocated to the cascade as a knob, if a free changelog
de-park is later wanted). For M37, `vision` is freeze-exempt, so the schema-key placement costs
nothing here. *(The build increment may confirm bespoke-vs-knob against the finalize path; the
knob is the design-of-record.)*

Not reached by a location tweak alone: the idiomatic **uppercase** `VISION.md` filename — slugs
are `[a-z0-9-]`, so the managed home is `vision.md`; the uppercase root name is produced by the
`root-render:` path value (`VISION.md`), which is a literal, not a slug.

## 5. The methodology `commit` fix (GF5)

`packs/methodology/schemas/commit.yaml`: add `optional: true` to the `scope` field and the
`body` slot — the M26 finalize-wall fix the methodology fork missed (without it, `finalize`
hard-blocks an empty `scope`/`body`, the first-run wall the dev pack already removed). Leave
`implements→spec` dropped (a deliberate methodology subset, [multi-pack.md](multi-pack.md)).
The methodology `commit` is shadowed by the dev `commit` under `[dev ▸ methodology]`
composition, so live stakes are low — but the fix is two flags, closes a *known* regression,
and closes [decisions-pending.md](../implementation/decisions-pending.md) → line 109, whose
trigger ("next methodology-pack milestone") has no scheduled successor after M37 (re-defer
would orphan it).

## 6. Naming audit (the principle binds; recorded for the record)

- **`grounded-in` / `grounds`** — kept (in the charter + [DECISIONS.md](../DECISIONS.md) record;
  reads naturally: "this vision is grounded in research X"; "research X grounds these visions").
  Rejected `informed-by`/`informs` (marginal gain, churns the record) and `cites` (implies
  citation, not foundation; already the `arch-doc → adr` relation).
- **`thesis`** (over `overview`) — cleaner semantics for a vision's core claim; a standard term.
- **`invariants`** (over `principles`) — the precise term for "what must not change"; the term
  this project's own VISION uses, and an AI knows it.
- **`open-questions`** — the RFC/design-doc idiom; multi-word-safe post-M22.
- **`question` / `findings` / `sources`** — standard research-record terms (`sources` over
  `references`: cleaner for provenance).
- **`description`** (over the house term "shape") for the `idea` body; **`trigger`** (over a
  literal "de-park trigger") for the revisit condition — both ecosystem-plain.

## 7. Acceptance

A `crates/cli/tests/flow*.rs` suite drives the **real binary** under the **`[dev ▸ methodology]`
composition** (the RC-trial's actual on-ramp, not methodology-alone), following the
`flow19`/`flow20` templates:

1. `do-research` (×2) → author two `research` docs → finalize each → committed
   `docs/research/<slug>.md` with an on-create `date`.
2. `form-vision` → create `vision` → `set-field grounded-in=[research:a, research:b]` (**both**,
   to exercise the multi-valued anchor) → **re-compose** (`jigc start --task <id>`) → author
   thesis/invariants/open-questions → finalize. **Asserts:** *both* `grounded-in` targets resolve
   (the stored anchor is multi-valued, no block); the re-composed `form-vision` guidance contains
   the **first** research's `findings` prose (the edge-walk slice actually read it — not merely
   "resolved", closing the vacuous-green gap); the managed `docs/vision/vision.md` H1 reads
   `# Vision` (display-title knob); a root `VISION.md` is rendered + committed byte-faithful to the
   managed doc (root render); `describe` lists all three doctypes + three workflows.
3. `park-idea` → author `idea` (description + trigger) → finalize → committed
   `docs/ideas/<slug>.md`; and `park-idea` appears in the router selection surface (selectable).
4. **Dangling arm:** a `grounded-in` pointing at a non-existent research **blocks** at finalize
   (`ref-resolves`, per-element).
5. **Freeze arm:** the pack-load freeze assertion stays green — the new `display-title:`/`root-render:`
   keys (serialize-skipped when absent) do not change any frozen dev-pack doctype's hash.
6. **Pre-existing-root arm (no silent data loss):** in a repo that *already* has a hand-authored
   root `VISION.md` and no managed vision, the first `form-vision` finalize **blocks and routes**
   (does not overwrite the foreign file); after the file is removed/adopted, finalize proceeds and
   the render owns the root file.

Fold-back (owed at build/completion): [doctype-map.md](../implementation/doctype-map.md) main
"The doctypes" table + the relations/edges section gain the three rows + the `grounded-in`
edge; the purpose-map rows flip build-now→shipped; [worked-examples.md](worked-examples.md)
gains **flow 25** (vision-forming + park-idea, run under `[dev ▸ methodology]`);
[methodology-docs.md](methodology-docs.md) gains a line noting the first intra-pack doc-level
managed ref; [CLAUDE.md](../CLAUDE.md) reading order gains this doc; the stale consumer framing
in [ideas/derived-doc-staleness.md](../ideas/derived-doc-staleness.md) (still naming adr/spec/prd
consumers) is noted as superseded by the narrowed M37 scope (the hash-pinned staleness mechanic
stays parked, trigger unchanged).
