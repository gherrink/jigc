# Override system

How a project customizes the pack without forking it into bit-rot. This is [VISION.md](../VISION.md) principle #5 made concrete: **the cascade** (layered config resolution) and **the override ladder** (recorded deltas) are *one system* — the cascade is the ordered application of recorded deltas over a versioned base.

Builds on [structural-grammar.md](structural-grammar.md) (override operates on skeleton units by address), [document-type-schema.md](document-type-schema.md) (knobs reuse the field model), [write-commands.md](write-commands.md) (the propose/confirm loop), [storage.md](storage.md) (config locations, base-hash, derived caches), [validation.md](validation.md) (`override-default` is a probe; findings + routes), and [workflow-dialect.md](workflow-dialect.md) (definitions resolve through the cascade). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## The cascade

Three layers over a versioned base, ordered by **specificity — most-specific wins**:

| layer | scope | lives | precedence |
|---|---|---|---|
| **project** | this repo | committed config dir (in-repo, diff-reviewed) | **highest — wins** |
| **team** | cross-project / team-general | external `~/.config/<tool>/` | middle |
| **pack-default** | universal (the pack's baseline) | ships with the installed pack | base |

Project wins because it is the layer that *differs most* — the deliberate, specific decisions of the one repo you're in must override your general cross-project settings, not the reverse. `team` is **not per-developer**: what this system configures is *governance* (doc types, workflows, conventions, severities), which is a team/project concern, not a personal preference — so a per-developer layer has nothing to hold (deferred; see Open questions).

Resolution applies the base, then `team` deltas, then `project` deltas (project last, so it wins). The in-repo `.tool/` dir holds **no config** — it stays purely derived/transient ([storage.md](storage.md)).

**Determinism.** Composition is a pure function of the *resolved cascade* — a declared, inspectable input, never hidden variance. The practical rule: **anything that must be reproducible from the repo alone belongs at project level** (committed), which is exactly where governance naturally lives.

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

## Upgrade reconciliation — `override-default`

When pack-default goes `v1 → v2`, a guarded `tool upgrade` re-applies each recorded delta and classifies it. This is the engine-native **`override-default` probe** ([validation.md](validation.md)); it asks two deterministic questions per delta against v2:

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
