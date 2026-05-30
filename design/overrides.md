# Override system

How a project customizes the pack without forking it into bit-rot. This is [VISION.md](../VISION.md) principle #5 made concrete: **the cascade** (layered config resolution) and **the override ladder** (recorded deltas) are *one system* — the cascade is the ordered application of recorded deltas over a versioned base.

Builds on [structural-grammar.md](structural-grammar.md) (override operates on skeleton units by address), [document-type-schema.md](document-type-schema.md) (knobs reuse the field model), [write-commands.md](write-commands.md) (the propose/confirm loop; the workflow create-gate is a cascade scalar at `workflows.<id>.allows-create`), [storage.md](storage.md) (config locations, base-hash, derived caches), [validation.md](validation.md) (`override-default` is a probe; findings + routes), [workflow-dialect.md](workflow-dialect.md) (definitions resolve through the cascade), and [command-catalog.md](command-catalog.md) (command-ref entries resolve through the same cascade — `insert`/`replace`/`remove` deltas apply uniformly). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## The cascade

Three layers over a versioned base, ordered by **specificity — most-specific wins**:

| layer | scope | lives | precedence |
|---|---|---|---|
| **project** | this repo | committed config dir (in-repo, diff-reviewed) | **highest — wins** |
| **team** | cross-project / team-general | external `~/.config/jigc/` | middle |
| **pack-default** | universal (the pack's baseline) | ships with the installed pack | base |

Project wins because it is the layer that *differs most* — the deliberate, specific decisions of the one repo you're in must override your general cross-project settings, not the reverse. `team` is **not per-developer**: what this system configures is *governance* (doc types, workflows, conventions, severities), which is a team/project concern, not a personal preference — so a per-developer layer has nothing to hold (deferred; see Open questions).

Resolution applies the base, then `team` deltas, then `project` deltas (project last, so it wins). The in-repo `.jigc/` dir holds **no config** — it stays purely derived/transient ([storage.md](storage.md)).

**Determinism.** Composition is a pure function of the *resolved cascade* — a declared, inspectable input, never hidden variance. The practical rule: **anything that must be reproducible from the repo alone belongs at project level** (committed), which is exactly where governance naturally lives.

**Cascade provenance is visible on every long-lived surface.** The resolved cascade is a declared input to determinism, so the agent and the human must *see* what cascade is in effect — hidden cascade variance is the failure mode this discipline exists to prevent. `jigc start` (orientation — see [bootstrap.md](bootstrap.md) → Orientation output examples), `jigc task validate`, and `--explain` ([workflow-dialect.md](workflow-dialect.md) → `--explain` output contract) all show a one-line header: `Pack: <pack-id>/<version> · Project config: <path> · Branch: <branch> (HEAD <short>)`. When the external team layer at `~/.config/jigc/` is populated, a `Team config: <path>` segment is added. `--explain` extends this with the per-definition resolved layer as resolution-tree layer 1.

## The ladder — deltas, not forks

The enemy is not forking; it is the *untracked* fork that discards its ancestor and makes upgrades a guess. So **every customization is a recorded delta against a known base version**, and the ladder is just the lightest delta-kind that fits:

1. **`scalar-set`** — set a config knob. Most customization lives here.
2. **`structural-op`** — `insert` / `replace` / `remove` a unit, targeted **by address** (`workflow:single-task#locate`), never by position.
3. **`slot-fill`** — fill a named extension point; never the surrounding prose.
4. **`tracked-fork`** — copy a unit, last resort, recording the base hash so a 3-way merge can detect upstream conflicts.

These *are* the cascade's delta vocabulary — there is no separate "config system" and "override system."

## Delta representation

A layer's deltas live in a **config-format manifest**; content lives in **native-format files**, never inline:

- `scalar-set` → inline in the manifest (`key: value`).
- `structural-op` / `slot-fill` / `tracked-fork` → an operation entry referencing a native file (a step file in workflow-definition form, prose as Markdown). No prose-in-config.
- Every **content-bearing** delta records its **base-version** and the target's **base-hash** (generalizing `tracked-fork`), which makes reconciliation stateless (below).

```yaml
# project config — deltas against pack-default v1
scalar:
  validation.doc-code.severity: advisory

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

An open surface is rejected for the same reason untracked forks are: a silent, unvalidatable, unreconcilable typo'd key is exactly the failure mode the whole system exists to kill.

**Locked keys.** Some scalar keys are declared but **demotion-locked** — their value can be set above a floor but not below it. The validation severity inventory ([validation.md](validation.md) → Severity inventory) names the intrinsic checks whose `validation.<probe>.<check>.severity` keys cannot be demoted below `blocking`. A `scalar-set` attempting to demote a locked key is rejected at cascade resolution as a config-conformance error (the `scalar-set` is logged, not applied; visible via `--explain`). Same delta machinery — just with a per-key floor declared by the pack.

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

## Upgrade reconciliation — `override-default`

When pack-default goes `v1 → v2`, a guarded `jigc upgrade` re-applies each recorded delta and classifies it. This is the engine-native **`override-default` probe** ([validation.md](validation.md)); it asks two deterministic questions per delta against v2:

1. **Does the target still exist?** No → **`orphaned`** (loud failure).
2. **Did the target change, *and does this delta depend on that content*?** Yes → **`conflict`** (review). No → **`clean`** (you inherit every other v2 improvement free).

A delta conflicts *only when it depends on content that changed upstream*:

| delta kind | depends on | outcomes |
|---|---|---|
| `scalar-set` | a key's existence | clean / orphaned |
| `insert` | the anchor's *existence* | clean / orphaned (never conflicts) |
| `replace` / `remove` | the target's *content* | clean / **conflict** / orphaned |
| `slot-fill` | the slot's existence | clean / orphaned |
| `tracked-fork` | the forked target's *content* | clean / **conflict** / orphaned |

Because every content-bearing delta carries its target's **base-hash**, change-detection is a pure compare (`v2 hash ≠ recorded base-hash`) — **stateless**, no old pack kept around.

The probe emits validation findings with routes: `orphaned` → a `run-command` route to remove or re-target the delta; `conflict` → the merge path — **the agent drafts a 3-way-merge proposal through the CLI, the human confirms, the CLI never calls a model**. `orphaned`/`conflict` default to blocking (cascade-tunable). The upgrade is *guarded by the report*: re-apply, classify, resolve, complete — "no upstream change silently lost; no override silently broken."

## Open questions

- **Per-developer `local` layer** — deferred; if added it sits on top (`local > project`), restricted to non-structural / tighten-only (a dev may self-impose stricter checks, never weaken a team standard).
- **Team-layer distribution** — how a team *shares* its `team` config (each member installs the same external layer); a mechanics detail.
- **Committed config dir layout** — the concrete name/structure of the project config location (tracks the undecided product name).
