# Override system

How a project customizes the pack without forking it into bit-rot. This is [VISION.md](../VISION.md) principle #5 made concrete: **the cascade** (layered config resolution) and **the override ladder** (recorded deltas) are *one system* — the cascade is the ordered application of recorded deltas over a versioned base.

Builds on [structural-grammar.md](structural-grammar.md) (override operates on skeleton units by address), [document-type-schema.md](document-type-schema.md) (knobs reuse the field model), [write-commands.md](write-commands.md) (the propose/confirm loop; the workflow create-gate is a cascade scalar at `workflows.<id>.allows-create`), [storage.md](storage.md) (config locations, base-hash, derived caches), [validation.md](validation.md) (`override-default` is a probe; findings + routes), [workflow-dialect.md](workflow-dialect.md) (definitions resolve through the cascade), and [command-catalog.md](command-catalog.md) (command-ref entries resolve through the same cascade — `insert`/`replace`/`remove` deltas apply uniformly). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

> **Build-scope split — M4 (apply) vs M5 (reconcile).** This doc describes one system, but it ships in two milestones ([roadmap.md](../implementation/roadmap.md), [DECISIONS.md](../DECISIONS.md) 2026-06-03). **M4 · override application** builds the *apply* path: the cascade wired **live** into compose, the four delta kinds (`scalar-set` · `structural-op` · `slot-fill` · `tracked-fork`) authored via the `jigc config` verbs (or by hand in delta form), the typed knob surface, and the `{{fill:<id>}}` extension point — so a project composes differently from the pack default. **M5 · upgrade reconciliation** builds the *inherit-upstream* path: the [`override-default` probe + `jigc upgrade`](#upgrade-reconciliation--override-default-m5), the stateless `base-hash` **compare**, and a `conflict` that **blocks with a review route** (keep / re-target / drop). **M6 · severity tuning** then adds cascade-assigned severities (`override-default` ships blocking-by-default in M5) + the **demotion-lock floor enforcement**. The true **3-way-merge authoring** on `conflict` is **deferred past both** (the stateless design keeps only the base-*hash*, not the base-*content* a merge needs — [DECISIONS.md](../DECISIONS.md) 2026-06-03). The split rule for content below: M4 *records* every content-bearing delta's `base-version`+`base-hash` (so the data is there); M5 is the only consumer that *reads* them. Sections marked **(M5)** ship in M5; severity-floor material is **(M6)**.

## The cascade

Three layers over a versioned base, ordered by **specificity — most-specific wins**:

| layer | scope | lives | precedence |
|---|---|---|---|
| **project** | this repo | committed config dir (in-repo, diff-reviewed) | **highest — wins** |
| **team** | cross-project / team-general | external `~/.config/jigc/` | middle |
| **pack-default** | universal (the pack's baseline) | ships with the installed pack | base |

Project wins because it is the layer that *differs most* — the deliberate, specific decisions of the one repo you're in must override your general cross-project settings, not the reverse. `team` is **not per-developer**: what this system configures is *governance* (doc types, workflows, conventions, severities), which is a team/project concern, not a personal preference — so a per-developer layer has nothing to hold (deferred; see Open questions).

Resolution applies the base, then `team` deltas, then `project` deltas (project last, so it wins). The in-repo `project` layer lives in committed `.jigc/config/`; the rest of `.jigc/` is gitignored derived/transient state ([storage.md](storage.md)).

**Determinism.** Composition is a pure function of the *resolved cascade* — a declared, inspectable input, never hidden variance. The practical rule: **anything that must be reproducible from the repo alone belongs at project level** (committed), which is exactly where governance naturally lives.

**Cascade provenance is visible on every long-lived surface.** The resolved cascade is a declared input to determinism, so the agent and the human must *see* what cascade is in effect — hidden cascade variance is the failure mode this discipline exists to prevent. `jigc start` (orientation — see [bootstrap.md](bootstrap.md) → Orientation output examples), `jigc task validate`, and `--explain` ([workflow-dialect.md](workflow-dialect.md) → `--explain` output contract) all show a one-line header: `Pack: <pack-id>/<version> · Project config: <path> · Branch: <branch> (HEAD <short>)`. When the external team layer at `~/.config/jigc/` is populated, a `Team config: <path>` segment is added. `--explain` extends this with the per-definition resolved layer as resolution-tree layer 1.

## The ladder — deltas, not forks

The enemy is not forking; it is the *untracked* fork that discards its ancestor and makes upgrades a guess. So **every customization is a recorded delta against a known base version**, and the ladder is just the lightest delta-kind that fits:

1. **`scalar-set`** — set a config knob. Most customization lives here.
2. **`structural-op`** — `insert` / `replace` / `remove` a unit, targeted **by address** (`workflow:single-task#locate`), never by position.
3. **`slot-fill`** — fill a named extension point; never the surrounding prose. The extension point is an inline **`{{fill:<id>}}`** placeholder a pack step/section body declares (a read-path placeholder, CLI-filled — see [the fill placeholder](#the-fill-placeholder--slot-fill-targets)); a `slot-fill` delta targets `step:<id>#<fill-id>` and supplies the content, higher layer winning.
4. **`tracked-fork`** — copy a unit, last resort, recording the base hash so a 3-way merge can detect upstream conflicts. (At compose time a fork is just a shadowed file — phase 2 applies it; the recorded base-hash is read only by the **(M5)** reconciliation.)

These *are* the cascade's delta vocabulary — there is no separate "config system" and "override system."

## Delta representation

A layer's deltas live in a **config-format manifest**; content lives in **native-format files**, never inline:

- `scalar-set` → inline in the manifest (`key: value`).
- `structural-op` / `slot-fill` / `tracked-fork` → an operation entry referencing a native file (a step file in workflow-definition form, prose as Markdown). No prose-in-config.
- Every **content-bearing** delta records its **base-version** and the target's **base-hash**, which makes reconciliation stateless (below). *(Shipped for `tracked-fork` in M4; extended to `replace`/`remove` in M5 — old manifests lacking it reconcile as `needs-rebasing`.)*

```yaml
# project config — deltas against pack-default v1
scalar:
  validation.doc-code.severity: advisory   # M4 accepts this; the demotion-lock floor is (M6)

deltas:
  - kind: insert-step
    target: workflow:single-task
    after: locate
    content: steps/team-lint          # a native step file in this layer

  - kind: tracked-fork
    target: workflow:single-task#validate
    base-version: v1
    base-hash: "a3f9…"
    content: steps/validate-forked
```

Hand-editing is allowed **only in delta form** — edit the manifest or a native file, never fork a whole base definition into an untracked copy.

## Scalar knobs are config-level fields

A scalar knob is a **pack-declared, typed field** — `enum` / `string` / `bool` / `int` with a default — so it reuses the [field model](document-type-schema.md) entirely: a `scalar-set` is type-checked exactly like `set-field` (write-time adjudication) and reconciled like any delta. The knob surface is **closed**: only declared keys are settable; an undeclared `scalar-set` is an error, a wrong-type value is rejected. This carries the same "anticipated customization" discipline as slots — *knobs exist only where the pack anticipates tuning*. Anything beyond the declared surface that changes the *shape* of a definition is a `structural-op` by address.

**On-disk declaration — `config/knobs.yaml`.** The pack declares its closed knob surface in a `config/knobs.yaml` resource: one entry per key, reusing the document-type [`FieldType`](document-type-schema.md) vocabulary so adjudication is the *same* `check_value` the doc write path uses — no second type system.

```yaml
# pack/config/knobs.yaml — the closed, typed knob surface
default-workflow:
  type: enum
  of: [router, single-task, quick-fix, plan, implement-from-spec]
  default: router
validation.override-default.target-unchanged.severity:   # tunable (no floor)
  type: enum
  of: [blocking, warning, advisory]
  default: blocking
validation.workflow-refs.placeholder-resolves.severity:  # intrinsic (M6)
  type: enum
  of: [blocking, warning, advisory]
  default: blocking
  floor: blocking             # demotion-lock: may not be set below blocking
```

The loader builds the pack-default layer's scalar surface from this file — both the **closed key set** (what `scalar-set` may target) and each knob's **default value**. A knob's default is **materialized by the resolver** seeding the resolved map from `knobs.yaml` before applying any delta; this is the knob's own mechanism and is *independent* of the doc-instance `Field.default` (which a created document does not yet materialize — an orthogonal write-path defect, [DECISIONS.md](../DECISIONS.md) 2026-06-03).

**What migrates, and what doesn't.** Only the keys that must **resolve through the cascade** become knobs: `default-workflow` and the `validation.*.severity` keys move to `knobs.yaml` with declared types; the live `serde_yaml_ng::get("default-workflow")` read in the compose path is replaced by `resolved.scalar("default-workflow")`. **`pack-id` is not a knob** — it is the pack *naming itself* (read for the provenance header), not a project-overridable value — so it stays a pack-identity field (the retained `config/defaults.yaml`, or a pack manifest), read directly, never through the cascade. "Subsumed" means the *settable* surface moves to `knobs.yaml`, not that `defaults.yaml` is deleted out from under the `pack-id` read.

**Read-side determinism invariant.** Routing a compose-path read through `resolved.scalar(k)` is byte-safe **only** if `k` is declared in `knobs.yaml` and thus seeded into the base map. So the rule is two-sided: `resolve` already rejects a `scalar-set` to an *undeclared* key (write side); M4 adds that **`resolved.scalar(k)` returning `None` for a key the compose path reads is a hard error, not a silent fallback** (read side). The no-override path must stay **byte-identical** to today's output — proven by a no-delta golden over the existing `start_compose` fixtures, *plus* a test that a read of an undeclared knob fails loudly rather than resolving to the old raw value ([Resolution algorithm](#resolution-algorithm)).

An open surface is rejected for the same reason untracked forks are: a silent, unvalidatable, unreconcilable typo'd key is exactly the failure mode the whole system exists to kill.

**Locked keys (enforcement: M6).** Some scalar keys are declared but **demotion-locked** — their value can be set above a floor but not below it. The mechanism (M6) is a **`floor:` field on the knob's `KnobDecl`** (declared by the pack in `config/knobs.yaml`, alongside `type`/`of`/`default`): `floor: blocking` means the key may be set to `blocking` but not below. The validation severity inventory ([validation.md](validation.md) → Severity inventory) is where the 11 intrinsic checks get `floor: blocking` on their `validation.<probe>.<check>.severity` keys. **Intrinsic-ness is expressed as "the knob carries `floor: blocking`"** — but, so a mis-declared pack cannot silently un-lock the determinism boundary, the **engine asserts at pack-load that its known intrinsic check-id set is each floored**, failing loudly on an unfloored intrinsic check. The assertion is *assertion-only* (it never assigns severity, never ships pack content) — the engine-empty invariant holds; the engine is merely checking that its own load-bearing invariants are declared.

**Soft-rejection, not abort.** Severities are totally ordered `blocking > warning > advisory`; a `floor: X` key may be set to `X` or stricter, never below. A `scalar-set` attempting to set a key below its floor is **soft-rejected at cascade resolution**: the offending delta is **dropped (logged, not applied)** and **resolution continues** — *as if that delta were absent*, so the value resolves from the remaining layers (it does **not** snap to a synthetic "floor literal"; for an all-intrinsic key with no other delta that means the pack default `blocking` stands). This is deliberately distinct from setting an *undeclared* key (a typo), which stays a **hard** resolution error (`UndeclaredScalar`). So resolution gains a third outcome beyond "applied" / "hard error": "declared-but-below-floor → recorded as a rejected delta." The rejected deltas ride on the resolved cascade (a new surface on `Resolved`, consumed by `--explain`) and surface via **`--explain`** (a new line distinct from the *applied* overrides — [workflow-dialect.md](workflow-dialect.md) → `--explain` output contract). Same delta machinery — just with a per-key floor declared by the pack. M4 ships the severity *knobs themselves* as ordinary `scalar-set`-able typed keys; the **cascade-assigned severities + floor enforcement** ride with **M6 · severity tuning**, alongside the first probe with tunable checks (`override-default`, which itself ships blocking-by-default in M5). M6 also moves `knobs.yaml` from M4's two **per-probe** severity keys to the full **per-check** surface (per-probe key retained as a default — additive, never a rename, so an M4-authored manifest still resolves; an M4-authored demotion of an all-intrinsic probe like `workflow-refs` is now correctly soft-rejected by the floor).

## The `{{fill:}}` placeholder — slot-fill targets

A `slot-fill` delta needs something to fill. That something is an inline **`{{fill:<id>}}`** placeholder a pack author writes into a step (or doc-type section) body to mark an *anticipated extension point* — "a project may inject content here without forking the step." It is a **read-path placeholder** ([workflow-dialect.md](workflow-dialect.md#leaves-instructions-and-placeholders) → the fourth placeholder kind): CLI-filled deterministically, syntactically `{{…}}`, the strict opposite of the write-path `<<author:>>` slot (which the LLM fills per-instance). The two must not be confused — `{{fill:}}` is *config-time* content chosen by the cascade; `<<author:>>` is *task-time* prose authored by the agent.

```markdown
# pack steps/implement.yaml — declares one extension point, default empty
Implement the change directly in the working tree.
{{fill: extra-guidance}}
```

A `slot-fill` delta supplies its content, addressed `step:<id>#<fill-id>`:

```yaml
# project .jigc/config/manifest.yaml
deltas:
  - kind: slot-fill
    target: step:implement#extra-guidance
    content: fills/extra-guidance.md     # a native Markdown file in this layer
```

Resolution applies slot-fills at **phase 5** — *before* include expansion (7) and placeholder resolution (8) — so the filled content's `{{include:}}`, `{{cli.…}}`, and `{{@…}}` placeholders resolve in the later phases exactly as if the pack had written them inline. **One exception — no nested fills:** fill content (and a `{{fill:}}` point's default body) **may not contain another `{{fill:}}`**, because phase 5 does not re-run — a nested `{{fill:}}` would survive to phase 8 unresolved. The `config fill` verb rejects fill content containing `{{fill:}}` at write time, and `workflow-refs` flags a surviving `{{fill:}}` at resolution.

An **unfilled** `{{fill:<id>}}` resolves to the pack's default body (empty if none) — never a finding; an absent extension point is the common case. Higher layer wins for the same `<fill-id>` (phase-5 ordering).

**Orphan detection is M4, not deferred.** A `slot-fill` targeting a `<fill-id>` that **no resolved body declares** is a blocking **`workflow-refs`** finding at compose/resolution — symmetric with the undeclared-`scalar-set` rejection, and closing the same closed-surface hole for *both* authoring paths (the `config fill` verb's write-time check and a hand-edited manifest). This is engine-native M4 validation; it is *not* left to the (M5) `override-default` probe (which adds the *upgrade-time* re-classification of the same condition). A silent, inert, typo'd slot-fill is exactly the failure mode the closed surface exists to kill, so M4 owns the check.

## Delta targets — addressing a definition

A delta names *what* it operates on. The target grammar is **not** the content [`Address`](structural-grammar.md#addressing) (which is instance-scoped — `type:slug#unit/item/leaf`, a slice *inside a persisted document*). A `structural-op` / `slot-fill` targets a **definition's list-entry or extension point**, a distinct namespace:

| delta kind | target form | operates on |
|---|---|---|
| `scalar-set` | a knob key (`default-workflow`, `validation.doc-code.severity`) | the resolved scalar map |
| `structural-op` | `workflow:<id>` + `#<step-id>` / `after:` / `before:` anchor; `schema:<id>#<section-id>` | the definition's **include list** (`workflow.includes`) or **sections list** (`schema.sections`) |
| `slot-fill` | `step:<id>#<fill-id>` | a `{{fill:<id>}}` extension point in a body |
| `tracked-fork` | `workflow:<id>#<step-id>` / `schema:<id>#<section-id>` | the forked unit's file (shadowed whole) |

The load-bearing distinction: `workflow:single-task#validate` here means *"the entry `validate` in `single-task`'s include list"* — `single-task` is a **workflow id**, not a doc slug; `validate` is a **step-id list entry**, not a doc section. This needs its own target resolver over `WorkflowDef.includes` / `Schema.sections`; reusing the content-`Address` parser would conflate the definition and document namespaces (the two slugs live in different spaces). The `structural-op` verbs spell the anchor explicitly (`--after <id>` / `--before <id>`) rather than overloading `#`; the manifest mirrors that with `after:` / `before:` keys (the `#<step-id>` form is the `replace`/`remove` target, which needs no anchor).

## Authoring deltas — the `jigc config` verbs

Deltas may be authored two co-equal ways: **by hand** (edit `manifest.yaml` + drop a native file — sanctioned for config because governance is human/team-owned, the one place direct editing is first-class; never a forked base, only delta form) or through the **`jigc config` verb family**, which records the same manifest+native-file shape the loader reads. One verb per rung:

| verb | records | validation at write time |
|---|---|---|
| `jigc config set <key> <value>` | a `scalar-set` | closed-surface + `check_value` typed adjudication (undeclared key / wrong type rejected) |
| `jigc config insert-step --workflow <id> (--after\|--before) <step-id> <file>` | an `insert` `structural-op` + the native step file | anchor step-id present in the resolution *as of this edit* |
| `jigc config replace-step <workflow:id#step-id> <file>` | a `replace` `structural-op` + native file; **(M5)** also records `base-version`+`base-hash` of the *replaced* unit | target step-id present as of this edit |
| `jigc config remove-step <workflow:id#step-id>` | a `remove` `structural-op`; **(M5)** also records `base-version`+`base-hash` of the *removed* unit | target step-id present as of this edit |
| `jigc config fill <step:id#fill-id> --from-file <file>` | a `slot-fill` + native fill file | the `{{fill:<fill-id>}}` point exists in the resolved step body |
| `jigc config fork <workflow:id#step-id>` | a `tracked-fork` — copies the resolved unit into a native file, records `base-version` + `base-hash` | target resolves as of this edit |

**Native-file id = filename basename.** `insert-step`/`replace-step` take a *source file*; the native step they register takes its **id from the file's basename** — the same "a step's id is its filename" rule the pack uses ([workflow-dialect.md](workflow-dialect.md) → On-disk definition format). So `jigc config insert-step --workflow single-task --after implement ./project-validate.yaml` writes `.jigc/config/steps/project-validate.yaml`, the `insert` delta references `step:project-validate`, and phase-2 shadows that file. A name collision with an existing step id is a write-time error.

**`tracked-fork` hash basis (pinned now; read in M5).** The recorded `base-hash` is the **blake3** ([decisions-pending.md](../implementation/decisions-pending.md) → Hashing) of the **resolved native step/section file bytes** — the post-shadow, pre-expansion body, the exact bytes phase 2 would load. A fork addressed `workflow:single-task#validate` shadows the **step file** `validate` (`steps/validate.yaml`), not the workflow; the workflow `#<step-id>` form just names which unit to copy. M4 records this; only the (M5) reconciliation compares it — but the basis is pinned in M4 so M5's stateless compare (`v2 hash ≠ recorded hash`) can't silently break on a basis mismatch.

**Write-time vs resolve-time split.** A verb does the *cheap, local* checks at write time (key declared, type valid, anchor present **in the resolution as of this edit** — a snapshot) so the human gets an immediate error; the *whole-cascade* consequences (cycles a delta introduces, a later same-manifest delta orphaning an earlier one — e.g. a `remove-step` that drops the anchor a later `insert-step --after` needs) depend on the *full delta set*, so they surface at **resolution time** through `workflow-refs`, consistent with the within-layer manifest order ([Resolution algorithm](#within-layer-manifest-order)). This mirrors the document write path: `set-field` adjudicates the value at write time; cross-doc integrity waits for `validate`/`finalize`. The verbs write **only** the project (or, with a flag, team) layer's `manifest.yaml` + native files — never a base definition, never an untracked fork.

## Resolution algorithm

Every cascade-resolved definition — a workflow, a doc-type schema, or any future managed-config artifact — goes through the same deterministic algorithm. Workflows have one extra phase (include expansion); schemas skip it.

This is what makes [VISION.md](../VISION.md) principle #4's claim concrete: **same `(definition + cascade)` in → same composed output**. The phase order matters; deviating produces different outputs from the same inputs.

### The nine phases

```
1. Resolution — input gathering
   Walk pack-default → team → project. For each layer collect:
     · the workflow / schema file (if this layer ships or overrides one)
     · the step files (each step id resolves to the highest-precedence layer)
     · the deltas manifest

2. Cascade merge of files — by-id shadowing
   For each id (workflow id, step id, schema id):
     pick the highest-precedence layer that has a file for it. Project > team > pack-default.
     Shadowing is atomic at file level — no field-level merge across layers.

3. Apply scalar deltas
   Walk `scalar-set` deltas in order pack → team → project.
   Within a layer, deltas apply in manifest order.
   Project's scalar wins for the same key because it applies last.

4. Apply structural deltas — on the include list / sections list
   Walk `insert` / `replace` / `remove` deltas in order pack → team → project.
   Each operates on the IDs of the structural list (workflow.includes or
   schema.sections), addressed by id (`workflow:single-task#locate`).
   This phase runs BEFORE include expansion.

5. Apply slot-fills
   Walk `slot-fill` deltas, applied to step / section bodies.
   Higher layer wins for the same slot id.

6. Cycle detection
   Walk the resolved include graph (workflow → steps → any sub-includes).
   Cycle → compose-time error from `workflow-refs` (validation.md).

7. Include expansion                                                  (workflow-only)
   Recursively expand each `{{include: step:foo}}` in the include list
   and step bodies. Result: a flat composition.

8. Placeholder resolution                                             (workflow-only)
   Resolve `{{cli.…}}` command-refs and `{{…}}` / `{{@…}}` data-values
   per workflow-dialect.md → Leaves.

9. Emit                                                               (workflow-only)
   Render to the four-class format (Run / Content / Author / Reason)
   per workflow-dialect.md → Emitted format.
```

The load-bearing rule: **phases 1–6 produce a fully-resolved, cycle-free composition tree *before* any include expansion or placeholder resolution runs.** That ordering is what makes the determinism boundary hold at compose time.

### Why structural deltas precede expansion

An `insert-step --after locate` operates on the workflow's **include list** — the ordered step references — not on already-expanded content. If `locate` itself was `replace`d earlier in the same cascade, the `insert` sees the replaced reference because phase 4 applies deltas in order. A reader scanning the manifest can predict the post-cascade include list without reasoning about what each step expands to.

The alternative — applying deltas to already-expanded content — would have made `insert-step --after locate` land somewhere inside `locate`'s expanded body if the expansion happened to contain another step's name. That's exactly the kind of structural non-determinism the system exists to kill.

### Within-layer manifest order

Within a single layer's deltas manifest, deltas apply in **file order**. This matters when a manifest has multiple deltas targeting the same id — e.g., `replace-step target: locate` followed by `remove-step target: locate`. The replace runs first (operating on the original `locate`), then the remove (operating on the result of the replace). A later delta targeting a now-removed id surfaces an `orphaned` finding from `workflow-refs`.

The rule keeps the manifest's intent surface-visible: the order you write deltas is the order they apply.

### `replace` vs `tracked-fork` — precisely

These were under-specified before; locking them here:

- **`replace`** swaps which step / section id appears at a position in the structural list. Targets the list entry, addressed by id. The replacement is **another step / section id**, not inline content — when an override layer wants to ship inline content, it ships a step / section file and references its id.
- **`tracked-fork`** copies a unit's body into a new file at the overriding layer, recording `base-version` and `base-hash`. At compose-time it's just another shadowed file (phase 2 sees it like any other). The recorded base-hash matters only at upgrade-reconciliation time (`override-default` probe — see [Upgrade reconciliation](#upgrade-reconciliation--override-default)).

### What the algorithm does NOT do

- **No partial field-level merge across layers.** A doc / step file shadows or it doesn't. The granularity for "change this one thing" is a `scalar-set` (knob) or `structural-op` (insert / replace / remove); never an implicit YAML-deep-merge.
- **No expansion before delta application.** Phases 4–5 always precede phase 7.
- **No silent cycle handling.** Phase 6 surfaces a cycle as a `workflow-refs` finding; cycles are never broken automatically.
- **No LLM call.** The algorithm is deterministic — same `(files + deltas)` in, same composition tree out ([VISION.md](../VISION.md) principle #1).

## Upgrade reconciliation — `override-default` (M5)

> **(M5)** — everything in this section ships in **M5 · upgrade reconciliation**, not M4. M4 records the `base-version` + `base-hash` for **`tracked-fork` only**; M5 *extends* recording to `replace`/`remove` (old manifests lack it → `needs-rebasing`, below) and is the consumer that re-applies and compares them.

When pack-default goes `v1 → v2`, a guarded `jigc upgrade` re-applies each recorded delta and classifies it. This is the engine-native **`override-default` probe** ([validation.md](validation.md)); it asks up to **three** deterministic questions per delta against v2:

1. **Does the target still exist?** No → **`orphaned`** (loud failure).
2. **Is this a content-bearing delta with no recorded base-hash** (an `replace`/`remove` authored under M4)? Yes → **`needs-rebasing`** (re-record to pin a basis).
3. **Did the target change** (`current-pack hash ≠ recorded base-hash`)? Yes → **`conflict`** (review). No → **`clean`** (you inherit every other v2 improvement free).

A delta conflicts *only when it depends on content that changed upstream*:

| delta kind | depends on | outcomes |
|---|---|---|
| `scalar-set` | a key's existence | clean / orphaned |
| `insert` | the anchor's *existence* | clean / orphaned (never conflicts) |
| `replace` / `remove` | the target's *content* | clean / **conflict** / orphaned / **needs-rebasing** |
| `slot-fill` | the slot's existence | clean / orphaned |
| `tracked-fork` | the forked target's *content* | clean / **conflict** / orphaned |

Because every content-bearing delta carries its target's **base-hash**, change-detection is a pure compare (`current-pack hash ≠ recorded base-hash`) — **stateless**, no old pack kept around.

**Per-kind base-hash basis (M5 substrate fix).** M4 recorded `base-version`+`base-hash` only for `tracked-fork`; M5 extends it to the other content-bearing kinds so the conflict column above is real. For **all three** the basis is the **blake3 of the *pack-default* unit's resolved native bytes** — the bytes the override sits on, recorded at authoring time and re-read by the probe from the current pack:

| delta | base-hash is the blake3 of … |
|---|---|
| `tracked-fork workflow:W#s` | the forked step `s`'s pack-default bytes (already recorded by M4) |
| `replace workflow:W#s` | the **replaced** step `s`'s pack-default bytes (the pack unit being swapped out) |
| `remove workflow:W#s` | the **removed** step `s`'s pack-default bytes (the pack unit being dropped) |

**The probe re-reads the *pack-default* unit (`PackSource::read`), never the cascade-resolved owner.** This matters most for `tracked-fork`, which *shadows the same id* (`steps/s.yaml`): a shadow-aware re-resolution would return the fork's **own** copy, which always equals the recorded base-hash → every fork would falsely classify `clean` and the fork-conflict path would be silently dead. So the basis — at both record time (`config fork`/`replace-step`/`remove-step` already read `pack.read(Steps, s)` pack-direct) and probe time — is the pack's own bytes, deliberately bypassing any project shadow.

**On-disk vs in-memory representation.** The basis is written as `base-version:`/`base-hash:` keys on the *same* `replace-step`/`remove-step` manifest entry (where `config fork` already writes them for a fork). In memory it rides in a **separate M5 recording keyed by the delta's target**, **not** as new fields on the compose-facing `StructuralDelta` — so the phase-4 compose path and its byte-identical goldens are untouched (the basis is read only by `override-default`, never by compose).

So `config replace-step`/`remove-step` gain the same record `config fork` already writes (reusing the resolve-then-hash path). **Backward-compat:** an M4-written `replace`/`remove` delta has **no** recorded base-hash, so M5 parses the keys as **optional** for these kinds; a content-bearing delta with no recorded base-hash classifies as **`needs-rebasing`** (route: re-record it via `jigc config replace-step …`, which pins the basis against the now-current pack) — it is **never** re-derived from the current pack, which would compare equal and silently mask every conflict. A *recorded* basis that simply matches is `clean`; a *missing* basis is `needs-rebasing` — the two are distinct (a present-but-equal hash is never confused with an absent one).

The probe emits validation findings with routes: `orphaned` → a route to remove or re-target the delta; `conflict` → **blocks with a review route** naming what changed and the options (keep your override / re-target / drop). Both ship **blocking-by-default** in M5 (cascade-tunability of these severities is M6 · severity tuning). The upgrade is *guarded by the report*: re-apply, classify, resolve, complete — "no upstream change silently lost; no override silently broken."

**3-way-merge authoring is deferred (past M5 and M6).** The fuller story — *the agent drafts a 3-way-merge proposal through the CLI, the human confirms, the CLI never calls a model* — needs the base-*content* (the v1 ancestor) to merge against, but the stateless design keeps only the base-*hash*. So M5 **detects and surfaces** the conflict (the load-bearing value: every divergence surfaces at a known moment) and routes it to human review, exactly as `file_state` blocks an OOB conflict today without merging ([reconciliation.md](reconciliation.md)); building the diff3/merge surface (and whatever base-content retention it needs) is a separable later step, not part of proving the inherit-upstream half.

### The `jigc upgrade` command — report, route, resolve

`jigc upgrade` is the guarded entry point. It is **report-and-route only** — it never mutates the manifest:

1. **Re-apply** — load the resolved cascade and walk every recorded delta.
2. **Classify** — run `override-default` per delta against the *current* pack (the questions above), identifying each delta in its finding by its **target** (`workflow:single-task#implement`) — a delta has no stable id; its target is its identity.
3. **Report** — emit one finding per non-`clean` delta through the standard findings+routes renderer ([validation.md](validation.md) → Findings); `orphaned`/`conflict`/`needs-rebasing` are **blocking-by-default**, `clean` deltas emit nothing. The route is a **human-readable string** (keep / re-target / drop), mirroring `file_state`'s conflict-block prose — not the `run-command`/`reconcile` tagged union (that promotion is a separate cross-cutting result-contract change, out of M5).
4. **Resolve** — the human acts on each route by **re-running the `jigc config` verbs** (drop an orphaned delta, re-record a conflicted/needs-rebasing one — which re-pins its base-hash to the now-current pack). `jigc upgrade` itself writes nothing, so it needs no guarded-write transaction or rollback; re-running it after the human's edits is the verification.

**Trigger & versioning — hash-driven, not version-gated.** The compare is purely `current-pack-hash ≠ recorded-base-hash`, so reconciliation needs **no** version diff: `jigc upgrade` is a **manual command that re-checks every delta unconditionally** against whatever pack the binary currently carries. There is **no per-project "last-reconciled version" record** — the per-delta `base-version` (recorded `= CARGO_PKG_VERSION` at authoring time) is **narrative only** (shown in the conflict route: *"pack `dev/0.4.0` changed `implement` since you overrode it at `dev/0.3.0`"*), never a classification input. An "upgrade" in production is simply *installing a newer binary* (whose embedded pack carries a bumped `CARGO_PKG_VERSION`) and running `jigc upgrade`; when nothing changed, every delta is `clean` and the command is a clean no-op.

**Testing an upgrade — the `FilesystemPack` seam.** The production pack is compiled in (`include_dir!`), so a single binary embeds exactly one pack version — an e2e cannot otherwise drive a *genuine* `v1 → v2`. M5 adds a **`FilesystemPack`** `PackSource` (a long-named-but-unbuilt seam) selected by a **`JIGC_PACK_DIR`** env var via one pack-source factory. The load-bearing requirement isn't a site count — it's that **every production pack-source construction goes through the factory**, so the *recording* path (the `jigc config` verbs) and the *upgrade* path read the **same** env-selected pack; a basis recorded against one pack and compared against another would mismatch spuriously. A `FilesystemPack` answers `pack_version()` from a **`version` key in its `config/defaults.yaml`** (the file that already carries `pack-id`); absent → a fixed `fs-local` sentinel. Because `base-version` is narrative-only, the sentinel is harmless — but a test that wants the flow-7 `0.3.0`/`0.4.0` route text sets `version:` in each pack dir's `defaults.yaml`. A test records deltas with `JIGC_PACK_DIR=<v1>` then runs `jigc upgrade` with `JIGC_PACK_DIR=<v2>` — a real two-pack reconciliation through the built binary. The seam is independently useful (project-local packs), and `EmbeddedPack` stays the default when the env var is unset.

## Open questions

- **Per-developer `local` layer** — deferred; if added it sits on top (`local > project`), restricted to non-structural / tighten-only (a dev may self-impose stricter checks, never weaken a team standard).
- **Team-layer distribution** — how a team *shares* its `team` config (each member installs the same external layer); a mechanics detail.
- ~~**Committed config dir layout**~~ — *settled (M4 planning, 2026-06-03):* `.jigc/config/manifest.yaml` (`scalar:` map + `deltas:` list) + `steps/<id>.yaml` (structural-op / tracked-fork native files) + `fills/<id>.md` (slot-fill content); the pack-default knob surface in `config/knobs.yaml`. Formalized in [storage.md](storage.md) → Config layout.
