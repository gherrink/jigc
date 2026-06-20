# Write-command vocabulary

The verbs through which the agent (and humans) put content *into* document instances via the CLI — the write side of [VISION.md](../VISION.md) principle #3 (the CLI is the only interface for reads *and* writes). The addressing, unit kinds, override ladder, and validation framework these verbs build on are dialect-neutral and specified in [structural-grammar.md](structural-grammar.md); the `slot`/`field`/`relation` leaves they target are in [document-type-schema.md](document-type-schema.md). **Read those first.**

Scope: the **write layer** only. The override layer (customizing a *type*) is its own future part-doc. For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**; the on-disk format is an open question (the form-marker syntax open was resolved at M24 — the batch path is declarative whole-doc, below).

## Write layer vs override layer

Two layers share the address model but have different verbs and act on different things:

- **Write** fills *instances* — `slot`/`field` values, and items in repeatable sections.
- **Override** customizes *types* — inserting / replacing / removing sections in the schema ([structural-grammar.md](structural-grammar.md#override)).

`remove`-ing a section from a *type* hits every instance; you can't remove a schema-defined section from one instance without breaking the contract. So the layers stay distinct. The only per-instance structural variation is repeatable-section items — which is why those are write-layer verbs.

The guarantee that makes this safe: **the agent addresses content by name; the CLI does the placement.** Placement-safety comes from address-keying, not from how coarse or fine the verbs are — so the vocabulary can be ergonomic without ever letting the LLM place anything.

## The verbs

**Write primitives** — `jigc doc <verb> <addr>`:

| verb | targets | content handoff |
|---|---|---|
| `create <type> --<id-source> …` | — (mints the instance) | id-source inline |
| `set-slot <addr>` | a `slot` leaf | **stdin** (`--from-file -`, or a path) |
| `set-field <addr>` | a `field` leaf | **inline** (`--value …`); validated |
| `add-item <section-addr> --<id-source> …` | a repeatable section | id-source inline → returns the item address |
| `remove-item <item-addr>` | an item | — |
| `reorder <section-addr> --order …` | the item ordering list | deterministic edit of the ordered list |

**Lifecycle** — a task is born at `jigc start` (see [Task origination](#task-origination)) and then managed with `jigc task <verb> <id>`: `bind` (bind an existing committed doc to a context role — see [Binding a context role](#binding-a-context-role)) · `diff` (see the working changeset) · `validate` (check it) · `finalize` (validate + commit) · `discard` (abandon).

**Batch authoring — `jigc doc author <doctype> --from-file <payload>` (built at M24; the per-leaf primitives above are the floor).** Authoring a large instance one leaf at a time is hundreds of round-trips at scale (a 50-release changelog migration); the batch path collapses it. The chosen shape is a **declarative whole-doc** payload, not an emit-fill-extract form: the **agent authors one structured payload** describing the whole instance (its sections, items, and slot/field values — owning the prose), and the **CLI applies the equivalent `create` + N `add-item` + `set-slot`/`set-field` sequence over a single in-memory buffer**, persisting **once** at the end (rollback = discard the buffer; persist only on full success). **Implementation is pinned to the chain-the-existing-primitives path** ([DECISIONS.md](../DECISIONS.md) → 2026-06-16, review B1): the create runs through the same `create_gated` path (so the **create-gate is enforced** and the **in-location-squatter blank-seed fix applies** — both live in `state::create`), then each `add-item`/`set-*` splice primitive (each a pure source→source transform) is chained over the running buffer with **no persist between leaves**. This is exactly the per-verb chain M23 proved byte-stable, **minus the intermediate persists (which are byte no-ops)** — so byte-stability, the create-gate, and the squatter seam are all inherited unchanged. It deliberately does **not** build an `Instance` and `render` it (that canonicalizing re-emit would bypass the splice primitives' byte-preservation *and* the create-gate / squatter seams). The determinism boundary is unmoved: the **agent authors the payload** (the prose + which-content-goes-where), the **CLI places every leaf** (the agent still places nothing). The payload's slot/field values use the settled `<<slot>>`/scalar conventions; a corrupt payload is rejected whole (atomic), never half-applied. This **supersedes** the earlier "fillable form (`jigc doc edit` emit-fill-extract)" sketch — that was scoped for multi-*slot* single docs and had no story for repeatable-item expansion (adding the Nth item to a section), which is exactly the migration-at-scale need ([DECISIONS.md](../DECISIONS.md) → 2026-06-16 M24 planning). The batch is **doctype-general**; migration is its first heavy consumer.

**Post-MVP verbs — pinned, not designed in detail:**

- **`jigc doc rename <addr> --to <new-addr>`** — a path rename of a managed doc is an identity change ([storage.md](storage.md) → Identity), and this op handles it atomically: re-key the `file-state` hash · rewrite every referrer ref across the committed store · commit as one logical change (transactional like `finalize`). MVP detects OOB renames and routes the human to revert in git ([reconciliation.md](reconciliation.md) → Rename detection); the explicit CLI op ships when the verb is needed.
- **`jigc doc delete <addr>`** — confirms a deletion (the file is gone, the human meant it). Removes the `file-state` record and surfaces dangling referrer refs as integrity findings. Same deferral shape — MVP detects deletions and routes to restore.

## Content handoff

Splits by leaf kind, falling straight out of the slot/field distinction:

- **Fields** (short, adjudicable) → **inline** `--value …`. No escaping pain.
- **Slots** (multi-line prose) → **stdin** `--from-file -` (or a file path). Never inline — prose through a shell arg is unreadable and escape-fragile.

`add-item` follows the same rule: the id-source field is supplied inline so the slug can mint, the call returns the new item's address, and the item's prose slots are filled by follow-up `set-slot <item-addr>#<leaf>`. One slot per call; bulk fill is the **batch path's** job (`jigc doc author --from-file`, above), not a back-door all-at-once `add-item`.

## Task origination

The front door is `jigc start`. The agent — knowing only the bootstrap ([bootstrap.md](bootstrap.md)) — runs it; the CLI does the rest. Four forms, all deterministic:

- **bare `jigc start`** — **orients** (read-only): project state, in-progress tasks, the workflow catalog with each workflow's `when` hint, recent finalizations. Never mints, never composes a side-effectful workflow. Adapter `SessionStart` hooks call this ([assistant-adapter.md](assistant-adapter.md)).
- **`jigc start "<intent>"`** — composes the cascade's **default workflow** (the cascade knob `default-workflow: <id>`, [overrides.md](overrides.md)) with `{{task.intent}}` = `<intent>`. Whether this mints a task is the workflow's own **`creates-task`** declaration ([workflow-dialect.md](workflow-dialect.md)). The two knobs are orthogonal — any workflow can be the default, any workflow can mint or not.
  - MVP: `default-workflow: single-task` (a `creates-task: true` work-workflow) → one call mints.
  - M2 onward: cascade flips `default-workflow` to a `router` (`creates-task: false`) → composing it presents the catalog (the engine-native `catalog` root, [workflow-dialect.md](workflow-dialect.md) → Workflow selection) and emits the explicit `--workflow X` call, which is what mints.
- **`jigc start --workflow <X> "<intent>"`** — composes `X` explicitly, bypassing the cascade default. Mints iff `X` declares `creates-task: true`. The id is a **slug from `<intent>`** (frozen, collision-suffixed under the same discipline as artifacts); pinned to base = HEAD; opens `.jigc/tasks/<id>/`. Tasks are first-class identities in the **work-unit family** ([structural-grammar.md](structural-grammar.md#work-units-and-runtime-identity); planned siblings: `milestone`, `increment`). This is the agent's "I know what I want" path, and what the router's output names. `--workflow` combines with the `<intent>` positional and is **mutually exclusive with `--task`**; an unknown `<X>` (not in the catalog) is **rejected** with a routed finding.
- **`jigc start --task <id>`** — resumes an existing task: loads its working area, recomposes the task's own persisted workflow, shows where it is. No minting. *(A fanned sub-agent enters its sub-task differently — `jigc workflow <W> --task <id>`, which composes an explicitly-named sub-workflow and provisions the sub-area; see [Sub-agent re-entry](#sub-agent-re-entry--jigc-workflow-w---task-id-m8).)*

Minting is **structure declared by the workflow**, never inferred — the CLI reads `creates-task` and acts. The cascade decides the default, the router (when present) presents options, the agent's reasoning chooses, and a human can pre-pick via a catalog-generated adapter command ([assistant-adapter.md](assistant-adapter.md)). The agent supplies the *intent* (content); the CLI mints the *id* and opens the area (structure) — the determinism boundary holds.

**Composing a `creates-task: false` workflow** (the router is the canonical case) takes **no task context**: the CLI does not mint, open a working area, provision a commit doc, or bind any context role. Such a workflow therefore may not reference `task.*` — a `task`-rooted placeholder in a `creates-task: false` workflow is a `workflow-refs` conformance error at validate-time ([validation.md](validation.md)). It threads the user's intent forward only through an **agent-substitution marker** in its emitted re-run command (`jigc start --workflow <chosen> "<intent>"`), never a resolved data-value. The `{{task.intent}}` binding above applies only on the minting path.

**Task-id collision & resume — serial vs parallel.** The same slug can arrive twice; policy splits by *how*:

- **Active task already exists with the slugged id** (serial retry of `jigc start "<intent>"` or `--workflow X "<intent>"`) → **reject**, with the existing task's status surfaced. The agent uses `jigc start --task <id>` to resume or `jigc task discard <id>` to abandon and retry. *Never silently suffixed, never silently reused* — a re-issued intent that lands on the same slug is almost always a forgotten resume, and silent suffixing would strand the original work.
- **Base mismatch on an existing task** (the task is pinned to commit `<A>`, you're on `<B>`) → blocked with the same divergence-routing prompt as [out-of-band reconciliation](#out-of-band-reconciliation): switch back to `<A>`, or `discard`. Applies to both serial collision and explicit `--task <id>` resume — the CLI never operates a task off its pinned base.
- **`--task <id>` for a nonexistent id** → reject with "no task `<id>`". The in-progress catalog lives in bare `jigc start` orientation output, so the agent has a path to discover live ids.
- **Finalized task, slug reusable** → after `finalize` the working area is gone (`.jigc/tasks/<id>/` removed), so the task-id slot is free; a re-issued intent slugs to the same id and mints normally. Any *managed-artifact* ids the prior task minted (`commit:<slug>`, `adr:<slug>`) live in committed storage and follow the artifact minting rule's collision-suffix discipline if the new task tries to mint the same name ([structural-grammar.md](structural-grammar.md#ids-provenance-and-minting)).
- **Parallel collision under `fan-out`** (two sub-agents mint the same slug simultaneously) → the by-task-id merge applies the deterministic collision-suffix and rewrites *local* self-references ([workflow-dialect.md](workflow-dialect.md#fan-out--join)). This is the **only** place the suffix runs at task scope; the serial cases above all reject.

The principle: **suffix only in the deterministic parallel case** (where both works are legitimate and the join needs to disambiguate); **reject in the serial case** (where it's almost certainly a forgotten resume the agent should see).

### Minting a milestone + its task list (M7)

The `milestone` work-unit is the **fan-out container** ([structural-grammar.md](structural-grammar.md#work-units-and-runtime-identity)) — `milestone > task` for M7. Three verbs, all on the same work-unit-family minting discipline as `jigc start`:

- **`jigc milestone create "<title>"`** — mints a milestone work-unit (id = frozen slug from the title), opens its area with an empty task list, and **pins one shared base** (= HEAD) that every sub-task inherits — so the join's "present at the milestone base" test is a deterministic lookup against a frozen commit ([storage.md](storage.md#the-by-task-id-join-m7)). No sub-task minted yet.
- **`jigc milestone add-task <milestone-id> "<intent>"`** — mints a sub-task under the milestone (a task work-unit: id = slug from intent, pinned to the **milestone's** base, isolated `tasks/<sub>/` area) and appends it to the milestone's task list. Incremental — called once per sub-task; a serial slug collision within one milestone **rejects** (the deterministic suffix runs only at the join, never at incremental add).
- **`jigc milestone add-from-spec <milestone-id> <spec-addr>`** — the convenience populator (requested alongside incremental add): seeds the task list from a committed spec by **enumerating its repeatable `criteria` items** (the parse-items read path — `parse_sections` → the section's `items`, *not* the `jigc task bind` slice path) and minting one sub-task per criterion, the criterion's text as that sub-task's intent. A spec with **zero criteria items** is a blocking *"nothing to seed from"* — never a silent empty milestone.

The task list is what the by-task-id join enumerates ([storage.md](storage.md#the-by-task-id-join-m7)). M7 builds the work-unit, these verbs, and the join; the `{{milestone.tasks}}` data-value root that exposes the list to the read path, and the `milestone-execution` *workflow* that fans out over it, are M8.

**The sub-task's recorded workflow (M8).** M7's `add-task`/`add-from-spec` hard-recorded each sub-task's workflow as `single-task`; M8 gives both a **`--workflow <id>` argument (default `sub-task`)** — the finalize-less sub-workflow real sub-agents run — and **records it** in the sub-task area. This is what makes the re-entry equality guard meaningful: the `milestone-execution` fan-out's `run: workflow:<W>` must equal the value `add-task` recorded, so `jigc workflow <W> --task <id>` asserting `<W>` == the recorded workflow (below) is a genuine guard, not a tautology — a stale template or a fan-out naming a workflow the sub-tasks weren't seeded for **fails loudly** instead of silently composing the wrong thing.

**Executing the milestone — `jigc milestone execute <id>` (M8).** The milestone-level counterpart to the sub-agent re-entry verb: it composes the **`creates-task: false`** `milestone-execution` workflow over the milestone work-unit `<id>`, feeding the milestone's id-sorted task list into `{{milestone.tasks}}` so the `fan-out` step resolves it — **minting nothing** (the milestone and its sub-tasks already exist). It lives in the `jigc milestone` verb family (`create`/`add-task`/`add-from-spec`/`execute`/`join`/`finalize`), *not* on `jigc start` — `start --workflow` stays mutually exclusive with `--task` and `start --task` stays top-level-task resume (the invocation-surface fork settled 2026-06-05, [DECISIONS.md](../DECISIONS.md)): a milestone is a distinct work-unit kind, so executing it is its own verb rather than an overload of `--task`. (Which workflow `execute` composes is the pack's single milestone work-workflow today, `milestone-execution`; a cascade knob can choose among several when a second one earns it — the same evolution `default-workflow`/the router followed.)

### Sub-agent re-entry — `jigc workflow <W> --task <id>` (M8)

A fan-out spawns sub-agents; each re-enters the CLI to get its instructions. The re-entry verb is **`jigc workflow <W> --task <id>`** — a top-level verb parallel to `jigc start`, the surface the adapter's launch template renders ([assistant-adapter.md](assistant-adapter.md) → Bind the spawn mechanism). It is the design's named contract, distinct from `jigc start --task <id>`:

- **`jigc start --task <id>`** *resumes a top-level task* and recomposes the task's **own persisted minting workflow** (read from `.jigc/tasks/<id>/workflow`); it takes no workflow argument (`--workflow` is mutually exclusive with `--task` on `start`). This is the human/main-agent "pick up where I left off" path.
- **`jigc workflow <W> --task <id>`** *enters a milestone sub-task as a fanned sub-agent*: it composes the **explicitly-named sub-workflow `<W>`** (the fan-out step's `run:` workflow, which the launch template fills as `{{workflow}}`) for sub-task `<id>`. `<W>` is **explicit, not redundant**: a sub-task minted by `jigc milestone add-task` records its sub-workflow, and re-entry asserts `<W>` equals it (a mismatch is rejected — the fan-out and the recorded mint must agree), so the launch line is self-describing and a stale template that names the wrong workflow fails loudly rather than silently composing the wrong thing.

**Re-entry provisions the write-ready area.** `jigc milestone add-task` mints a sub-task area with `base`/`intent`/`workflow` but **no `docs/`** — it is resume-*composable* but not write-*ready*. The first `jigc workflow <W> --task <id>` re-entry **provisions** the sub-workflow's deterministic instances (the commit doc, any workflow-provisioned docs) into the sub-area — mirroring how `jigc start` provisions at mint time, just deferred to the sub-agent's first entry rather than at `add-task` (so areas that are never spawned are never provisioned). After re-entry the sub-agent writes through the normal `jigc doc` verbs, scoped to its sub-task (below).

### The write-time `--task`-scoped barrier (M8)

Under a fan-out, multiple sub-task working areas coexist, so a write must name *which* area it targets. Every `jigc doc <verb>` gains a **`--task <id>`** selector (the create-gate enforcement already anticipates "`.jigc/tasks/<id>/` from cwd, or `--task <id>` explicit" — [The create-gate](#the-create-gate)); the **active-task resolution** becomes: explicit `--task <id>` wins; else the single active task; else (zero, or **more than one** with no `--task`) reject. The barrier scopes **staging writes** — a `create` / `set-slot` / `set-field` / `add-item` / the copy-in whose **staged-doc destination path** would land outside `tasks/<id>/` is rejected at write-time, not deferred to the join. It does **not** restrict reads of committed state: `jigc task bind <role> <addr> <id>` targets a *committed* doc by design ([Binding a context role](#binding-a-context-role)) — it stages nothing into any area, so the barrier's "destination outside the sub-area" rule never applies to it. This is the *write-time* complement to the M7 *join-time* isolation check ([storage.md](storage.md#the-by-task-id-join-m7)): join-time catches a mis-attributed staged doc after the fact; the barrier refuses the mis-directed staging write up front, so a sub-agent physically cannot stage into a sibling's area.

**Editing a base-committed doc — copy-on-first-touch (M8).** A sub-agent that edits a doc which existed at the milestone's shared base must record `edited-from-base` provenance for the join's clash rule ([storage.md](storage.md#the-by-task-id-join-m7); [workflow-dialect.md](workflow-dialect.md#fan-out--join)). The first write to such a doc **copies the committed body into the sub-area** (`copy_in` → `edited-from-base`), then splices; a `create` of a new doc stages `created` as today (`provision_doc`). **Provenance is recorded once, at first touch, keyed on whether the slug existed in the committed store at the milestone base** — `created` (absent at base) is **sticky** across later edits in the same area (a created-then-edited doc stays `created`, never flips); only a doc present at base records `edited-from-base`. This is what makes the join's clash discriminator stable (the mixed-case `created` × `edited-from-base` block keys on base-membership, not on "did this area edit it"). It makes both provenance bits — and therefore the `same-doc-clash` block — reachable through the real `--task`-scoped write path, where pre-M8 only `created` was reachable and only for a single active task. (`provision_commit_doc` likewise records `created` provenance under M8 — a per-sub-task commit doc must appear in the provenance manifest the join consumes; this is **collision-safe by construction**, because `commit:<sub-id>` is sub-task-id-derived and therefore unique per sub-area, so per-sub-task commit docs never reach the join's same-slug suffix/clash rules.)

A task carries **context roles** its workflow declares; `task.<role>` ([workflow-dialect.md](workflow-dialect.md)) navigates the bound doc, and **the CLI never infers a binding** — the agent binds explicitly, by one of two routes depending on where the doc comes from:

- **agent-created, same task** — `jigc doc create <type>` under a create-gate entry whose object form declares `as: <role>` binds the new instance in one step ([The create-gate](#the-create-gate)).
- **an existing committed doc** — `jigc task bind <role> <addr>` ([Binding a context role](#binding-a-context-role)) binds a doc a *prior* task committed; this is how the spec → implementation arc threads a `spec` written by one task into the task that implements it.

A role the workflow declares but the agent has not yet bound resolves to **empty** (the slice is absent, not an error); a `task.<role>` whose role the workflow does **not** declare is a `workflow-refs` conformance error ([validation.md](validation.md)).

## Staging and the transaction model

**The staging unit is the task.** The per-task isolated working area — already required for sub-agent concurrency ([VISION.md](../VISION.md) → Parallelism) — *is* the staging area. There is no separate proposal object: every write lands in the current task's working area immediately (no `begin`), and "propose / review / apply" is just `write / validate / finalize` over that area. The task id is the changeset name.

**Two check times** ([VISION.md](../VISION.md): integrity holds at `finalize`, not per-write):

- **Write-time — local adjudication.** A `set-field` rejects a malformed date or a non-member enum *now*; a mint rejects a bad slug; an `add-item` rejects an id-source value outside its field's enum (re-slug membership) *now*. Fast, local feedback. *(M24 closed the gap here: pre-M24 only section-level `set-field` adjudicated at write time — `add-item`'s id-from enum and item/nested-item field values were caught only at finalize. M24 extends the write-time check to `add-item` id-from and item-field values via a **single shared slug+enum-membership helper called by both the write-time path and finalize's `check_id_from_enum`** — one function, so the two cannot disagree by construction (not a mirrored copy, which would drift when M25 generalizes it 4×). The finding **code** is shared; the emitted **address** leaf-suffix may differ by call site (at `add-item` time there is no parsed item yet); [DECISIONS.md](../DECISIONS.md) → 2026-06-16.)*
- **Finalize-time — integrity the task can fix.** Forward-ref resolution (dangling refs), required-slot presence, and unreconciled drift run at `finalize` — a ref may dangle in the working area until then, because its target can be created in the *same* task. **Finalize blocks only on what the task can fix itself**; inverse/minimum-cardinality ("a PRD needs a SPEC") depends on *other* tasks and is **completeness, not a per-task gate** (see [document-type-schema.md](document-type-schema.md), [validation.md](validation.md)). This split is what stops the bootstrap from deadlocking.

**`finalize` ≡ `validate` + commit.** Finalize has no private check path — it runs `validate` and commits only if it passes, so what `validate` reports and what `finalize` blocks on can never diverge. One task → one logical commit. The **ordered transaction** (preflight → validate → render → promote → stage → commit → post-commit), atomicity rules, rollback discipline, the change-set scoping (the commit is the agent-staged git index plus jigc's promoted docs/config, not a working-tree sweep), and the commit-doc → git-message mapping are specified in [finalize.md](finalize.md).

- `validate` is **scope-flexible** — a task (the working area; what the hard-block uses), a single doc, or the whole store — one engine ([structural-grammar.md](structural-grammar.md#validation); probe split per [VISION.md](../VISION.md) principle #6).
- Findings are **severity-tagged**: *blocking* (integrity must hold) stops `finalize`; *advisory* is surfaced but doesn't. **Which severity a check carries is a cascade setting** — a project can promote or demote, say, `doc↔code` staleness without touching code.
- `finalize` **defaults to autonomous**; an opt-in cascade setting can require human confirmation. The durable correction point is the git/PR review regardless.

## Instance provisioning

A working instance must exist before you can `set-slot` into it. The CLI **always** owns the structural act — mint the id (slug from the type's id-source field) and place it at the schema-defined location ([structural-grammar.md](structural-grammar.md#ids-provenance-and-minting)). There are two triggers:

- **Workflow-provisioned** (deterministic) — the composed workflow creates the instances the task obviously needs (e.g. the task's commit doc) and fills the id-source field deterministically, yielding a task-derived id like `commit:add-rate-limiter`. The agent only fills slots.
- **Agent-initiated** (judgment) — `jigc doc create adr --title "…"` when the agent decides a *new* doc is warranted mid-task. The CLI mints + places; the agent supplied only content.

This keeps the determinism boundary intact: **deciding a doc is warranted is reasoning (agent); creating and placing it is structure (CLI).** Agent-initiated `create` is **workflow-gated** — the catalog of types creatable in a given context is structure (workflow/cascade-owned); *choosing* among them is reasoning.

### The create-gate

The gate lives on the workflow's front-matter as **`allows-create: [<doctype-id>, ...]`** ([workflow-dialect.md](workflow-dialect.md) → On-disk definition format). **Default is empty — no agent-initiated creates allowed unless the workflow explicitly opts in.** Workflow-provisioned instances (e.g. the task's commit doc) bypass the gate; they're not shell-visible and are declared by the workflow's own structure.

**Two entry forms.** Each `allows-create` entry is either a bare doctype id (`adr`) or an object `{type: <doctype-id>, as: <role>}`. The object form additionally **declares a context role and binds the created instance to it**: creating that doctype under the gate makes the new instance reachable as `task.<role>` ([workflow-dialect.md](workflow-dialect.md)) with no separate bind step. MVP `single-task` ships `allows-create: [{type: adr, as: decision}]`, so an agent-created ADR is reachable as `task.decision` — the surface the superseding-decision context-slice composes from (`{{@task.decision.supersedes#decision}}`, see [worked-examples.md](worked-examples.md) → Superseding decision). The bare form grants create permission without declaring a role.

**Enforcement at every `jigc doc create <type> --<args>`:**

1. Identify the active task — `.jigc/tasks/<id>/` from cwd, or `--task <id>` explicit. **No active task** → reject: `"no active task — start one with \`jigc start\`"`.
2. Identify the task's workflow and resolve its effective `allows-create` through the cascade ([overrides.md](overrides.md) → Resolution algorithm).
3. **Unknown doctype** (`<type>` not in the cascade-resolved schema set) → reject: `"unknown doctype \`<type>\`"`. (This is a separate failure mode from the gate; it fires before the gate check.)
4. `<type>` ∈ `allows-create` → proceed: mint id, place per the schema's `location:`, **bind it to the entry's `as:` role if the entry declares one**, return the new address.
5. `<type>` ∉ `allows-create` → reject with the structured gate-block error:

```text
error: workflow 'single-task' does not allow `jigc doc create spec` in-task.
       allowed doctypes: [adr]
       to loosen: set `workflows.single-task.allows-create` in project config
```

The rejection carries a **`run-command` route** the agent can act on (the cascade-set command, formatted via the [command catalog](command-catalog.md)). Plain-text format above is the human surface; the structured payload shape rides on the broader blocked/error-payload open question (below).

**Cascade override.** Loosening or tightening is a standard delta:

```yaml
# project config
scalar:
  workflows.single-task.allows-create: [adr, spec]   # also allow agent to create SPECs
```

Resolves through the [9-phase algorithm](overrides.md#resolution-algorithm) — phase 3 (scalar deltas), project wins because it applies last. A structural-op replacing the workflow's front-matter wholesale works equivalently.

**`fan-out` sub-agents.** Each sub-agent runs its own sub-workflow, and *that* sub-workflow's `allows-create` applies. A sub-agent inherits no permissions from the parent. (Revisit if a real `fan-out` pattern demands parent → child gate inheritance.)

A worked payoff: the **commit message is just a doc type.** Its format — a `type` enum, a `scope` field, a `summary` slot, a `body` slot, a repeatable `trailers` section, a mandated issue-ref — is the commit type's schema plus cascade overrides. A project gets *any* commit convention it requires (conventional commits, custom trailers, …) with no special-casing; the same machinery that structures an ADR structures the commit. The agent writes the prose; the CLI guarantees the required format.

### Binding a context role

`jigc task bind <role> <addr> <id>` binds an **already-committed** document (`<addr>`) to one of task `<id>`'s declared context roles ([workflow-dialect.md](workflow-dialect.md) → `reads`), so `task.<role>` resolves to it. The `<id>` is the task positional, consistent with the `jigc task <verb> <id>` family (`validate`/`finalize`/`discard`). It is the counterpart to the create-gate's `as:` form: the gate binds a doc the task *creates*; `bind` binds a doc a *prior* task already committed — the only way a managed doc crosses a task boundary into a later task's context.

This is the **spec → implementation seam**: a `plan` task authors and commits a `spec`; a later `implement-from-spec` task runs `jigc task bind spec <spec-id> <task-id>`, and its `locate` step's `{{@task.spec#criteria}}` then resolves over that committed file.

**Why a verb, not a `start` flag.** Binding is a per-task write like any other, kept *off* `jigc start` so origination stays the single simple sentence the agent learns from the bootstrap. The composed workflow routes the agent to it — a `locate` step emits `` Run: `jigc task bind spec <SPEC_ID> <task-id>` `` (the `<SPEC_ID>` is a `<NAME>` agent-substitution marker, the trailing task id is CLI-resolved from `{{task.id}}` so the emitted line is runnable as-is, [command-catalog.md](command-catalog.md)) — exactly as it routes every other structural op back to the CLI.

**Resolution timing.** Binding happens *after* compose, so the bound slice is empty on the first composition and resolves on the **resume re-compose** (`jigc start --task <id>`) — the same deferred-bind-then-resume path the superseding-decision flow already uses for `{{@task.decision.supersedes#decision}}` ([worked-examples.md](worked-examples.md) → Superseding decision).

**Enforcement at every `jigc task bind <role> <addr> <id>`:**

1. Resolve task `<id>`. **No such task** → reject: `"no task \`<id>\` — start one with \`jigc start\`"`.
2. `<role>` ∉ the workflow's declared read-roles → reject, listing the declared roles.
3. `<addr>` does not resolve in the **committed store** → reject: `"no such doc \`<addr>\`"`. `bind` targets committed docs only — a doc the *same* task creates uses the create-gate's `as:` form instead.
4. The target's doctype ≠ the role's declared `type` → reject with the mismatch.
5. Otherwise record the binding in the task's `roles` and return the bound address. Re-binding a role overwrites (last-write-wins, surfaced in `task diff`).

The agent supplies only the *which-doc* choice; recording the binding and resolving it stay the CLI's — the determinism boundary holds.

## Out-of-band reconciliation

Humans edit files directly, and because the **files are the source of truth** ([storage.md](storage.md)), the system **honors a clean edit, not merely tolerates it** ([VISION.md](../VISION.md) principle #3). The CLI never forbids file edits and never silently discards them — every drift is detected, classified, and routed.

The deterministic classifier (states, transitions, parse classification, hash re-baselining, MVP vs post-MVP scope) is specified in [reconciliation.md](reconciliation.md). Three outcomes the agent sees: **absorb** (clean edit accepted; hash advances), **conformance-block** (parse / schema failure surfaces precisely; never auto-repaired in MVP), **conflict-block** (both sides changed; explicit discard, no silent default; three-way merge deferred). `finalize` blocks on the latter two and absorbs the first.

## Worked example — the MVP write loop

```text
# task add-rate-limiter; the workflow has provisioned commit:add-rate-limiter (empty)

jigc doc set-field commit:add-rate-limiter#type    --value feat
jigc doc set-slot  commit:add-rate-limiter#summary --from-file -      # prose piped in
jigc task validate add-rate-limiter                                   # blockers? (preview)
jigc task finalize add-rate-limiter                                   # validate + commit
```

Agent-initiated create within the same task:

```text
jigc doc create adr --title "Rate-limit at the gateway"   # → adr:rate-limit-at-the-gateway
jigc doc set-field adr:rate-limit-at-the-gateway#status  --value accepted
jigc doc set-slot  adr:rate-limit-at-the-gateway#context --from-file -
```

## Open questions

- ~~**Form-marker syntax**~~ — *resolved at M24 (2026-06-16):* the batch path is **declarative whole-doc** (`jigc doc author --from-file <payload>`, above), not an in-place emit-fill-extract form — so there is no in-document fill-marker to delimit; the open question collapses into the **payload format** (the structured input the CLI parses into a leaf-write sequence), elaborated at build. The slot/placeholder delimiters stay settled (`<<…>>` vs `{{…}}`).
- **`import` three-way merge** — the serialization round-trips (single-file Markdown, [storage.md](storage.md)), so `import` is unblocked; the remaining open part is the both-sides-changed three-way merge, shared with override conflict resolution ([overrides.md](overrides.md)).
- **Blocked/error payload** — the concrete shape a sub-agent writes to task state on a block ("import-vs-discard pending", "validation failed") and how it surfaces through the propose-to-human path.
