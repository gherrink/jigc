# Write-command vocabulary

The verbs through which the agent (and humans) put content *into* document instances via the CLI — the write side of [VISION.md](../VISION.md) principle #3 (the CLI is the only interface for reads *and* writes). The addressing, unit kinds, override ladder, and validation framework these verbs build on are dialect-neutral and specified in [structural-grammar.md](structural-grammar.md); the `slot`/`field`/`relation` leaves they target are in [document-type-schema.md](document-type-schema.md). **Read those first.**

Scope: the **write layer** only. The override layer (customizing a *type*) is its own future part-doc. For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**; the on-disk format and the form-marker syntax are open questions.

## Write layer vs override layer

Two layers share the address model but have different verbs and act on different things:

- **Write** fills *instances* — `slot`/`field` values, and items in repeatable sections.
- **Override** customizes *types* — inserting / replacing / removing sections in the schema ([structural-grammar.md](structural-grammar.md#override)).

`remove`-ing a section from a *type* hits every instance; you can't remove a schema-defined section from one instance without breaking the contract. So the layers stay distinct. The only per-instance structural variation is repeatable-section items — which is why those are write-layer verbs.

The guarantee that makes this safe: **the agent addresses content by name; the CLI does the placement.** Placement-safety comes from address-keying, not from how coarse or fine the verbs are — so the vocabulary can be ergonomic without ever letting the LLM place anything.

## The verbs

**Write primitives** — `tool doc <verb> <addr>`:

| verb | targets | content handoff |
|---|---|---|
| `create <type> --<id-source> …` | — (mints the instance) | id-source inline |
| `set-slot <addr>` | a `slot` leaf | **stdin** (`--from-file -`, or a path) |
| `set-field <addr>` | a `field` leaf | **inline** (`--value …`); validated |
| `add-item <section-addr> --<id-source> …` | a repeatable section | id-source inline → returns the item address |
| `remove-item <item-addr>` | an item | — |
| `reorder <section-addr> --order …` | the item ordering list | deterministic edit of the ordered list |

**Lifecycle** — a task is born at `tool start` (see [Task origination](#task-origination)) and then managed with `tool task <verb> <id>`: `diff` (see the working changeset) · `validate` (check it) · `finalize` (validate + commit) · `discard` (abandon).

A **fillable form** — `tool doc edit` emits the whole instance with slots marked, the agent fills in place, the CLI extracts-by-marker and places — is **deferred sugar** that compiles to a transactional batch of these primitives. It pays off only for multi-slot docs and carries the real risk (form corruption, handled like an out-of-band edit), so it lands when multi-slot flows do. The primitives are the MVP surface.

**Post-MVP verbs — pinned, not designed in detail:**

- **`tool doc rename <addr> --to <new-addr>`** — a path rename of a managed doc is an identity change ([storage.md](storage.md) → Identity), and this op handles it atomically: re-key the `file-state` hash · rewrite every referrer ref across the committed store · commit as one logical change (transactional like `finalize`). MVP detects OOB renames and routes the human to revert in git ([reconciliation.md](reconciliation.md) → Rename detection); the explicit CLI op ships when the verb is needed.
- **`tool doc delete <addr>`** — confirms a deletion (the file is gone, the human meant it). Removes the `file-state` record and surfaces dangling referrer refs as integrity findings. Same deferral shape — MVP detects deletions and routes to restore.

## Content handoff

Splits by leaf kind, falling straight out of the slot/field distinction:

- **Fields** (short, adjudicable) → **inline** `--value …`. No escaping pain.
- **Slots** (multi-line prose) → **stdin** `--from-file -` (or a file path). Never inline — prose through a shell arg is unreadable and escape-fragile.

`add-item` follows the same rule: the id-source field is supplied inline so the slug can mint, the call returns the new item's address, and the item's prose slots are filled by follow-up `set-slot <item-addr>#<leaf>`. One slot per call; bulk fill is the deferred form's job, not a back-door all-at-once `add-item`.

## Task origination

The front door is `tool start`. The agent — knowing only the bootstrap ([bootstrap.md](bootstrap.md)) — runs it; the CLI does the rest. Four forms, all deterministic:

- **bare `tool start`** — **orients** (read-only): project state, in-progress tasks, the workflow catalog with each workflow's `when` hint, recent finalizations. Never mints, never composes a side-effectful workflow. Adapter `SessionStart` hooks call this ([assistant-adapter.md](assistant-adapter.md)).
- **`tool start "<intent>"`** — composes the cascade's **default workflow** (the cascade knob `default-workflow: <id>`, [overrides.md](overrides.md)) with `{{task.intent}}` = `<intent>`. Whether this mints a task is the workflow's own **`creates-task`** declaration ([workflow-dialect.md](workflow-dialect.md)). The two knobs are orthogonal — any workflow can be the default, any workflow can mint or not.
  - MVP: `default-workflow: single-task` (a `creates-task: true` work-workflow) → one call mints.
  - Post-MVP: cascade flips `default-workflow` to a `router` (`creates-task: false`) → composing it presents the catalog and emits the explicit `--workflow X` call, which is what mints.
- **`tool start --workflow <X> "<intent>"`** — composes `X` explicitly, bypassing the cascade default. Mints iff `X` declares `creates-task: true`. The id is a **slug from `<intent>`** (frozen, collision-suffixed under the same discipline as artifacts); pinned to base = HEAD; opens `.tool/tasks/<id>/`. Tasks are first-class identities in the **work-unit family** ([structural-grammar.md](structural-grammar.md#work-units-and-runtime-identity); planned siblings: `milestone`, `increment`). This is the agent's "I know what I want" path, and what the router's output names.
- **`tool start --task <id>`** — resumes an existing task: loads its working area, shows where it is in its workflow. No minting.

Minting is **structure declared by the workflow**, never inferred — the CLI reads `creates-task` and acts. The cascade decides the default, the router (when present) presents options, the agent's reasoning chooses, and a human can pre-pick via a catalog-generated adapter command ([assistant-adapter.md](assistant-adapter.md)). The agent supplies the *intent* (content); the CLI mints the *id* and opens the area (structure) — the determinism boundary holds.

**Task-id collision & resume — serial vs parallel.** The same slug can arrive twice; policy splits by *how*:

- **Active task already exists with the slugged id** (serial retry of `tool start "<intent>"` or `--workflow X "<intent>"`) → **reject**, with the existing task's status surfaced. The agent uses `tool start --task <id>` to resume or `tool task discard <id>` to abandon and retry. *Never silently suffixed, never silently reused* — a re-issued intent that lands on the same slug is almost always a forgotten resume, and silent suffixing would strand the original work.
- **Base mismatch on an existing task** (the task is pinned to commit `<A>`, you're on `<B>`) → blocked with the same divergence-routing prompt as [out-of-band reconciliation](#out-of-band-reconciliation): switch back to `<A>`, or `discard`. Applies to both serial collision and explicit `--task <id>` resume — the CLI never operates a task off its pinned base.
- **`--task <id>` for a nonexistent id** → reject with "no task `<id>`". The in-progress catalog lives in bare `tool start` orientation output, so the agent has a path to discover live ids.
- **Finalized task, slug reusable** → after `finalize` the working area is gone (`.tool/tasks/<id>/` removed), so the task-id slot is free; a re-issued intent slugs to the same id and mints normally. Any *managed-artifact* ids the prior task minted (`commit:<slug>`, `adr:<slug>`) live in committed storage and follow the artifact minting rule's collision-suffix discipline if the new task tries to mint the same name ([structural-grammar.md](structural-grammar.md#ids-provenance-and-minting)).
- **Parallel collision under `fan-out`** (two sub-agents mint the same slug simultaneously) → the by-task-id merge applies the deterministic collision-suffix and rewrites *local* self-references ([workflow-dialect.md](workflow-dialect.md#fan-out--join)). This is the **only** place the suffix runs at task scope; the serial cases above all reject.

The principle: **suffix only in the deterministic parallel case** (where both works are legitimate and the join needs to disambiguate); **reject in the serial case** (where it's almost certainly a forgotten resume the agent should see).

A task carries **context roles** its workflow declares (e.g. `spec`); the agent **binds** them explicitly (at `start`, via a `locate` step, or — for an agent-created doc — at `create` through the gate's `as:` form, see [The create-gate](#the-create-gate)), and `task.<role>` ([workflow-dialect.md](workflow-dialect.md)) navigates the bound doc. The CLI never infers a binding.

## Staging and the transaction model

**The staging unit is the task.** The per-task isolated working area — already required for sub-agent concurrency ([VISION.md](../VISION.md) → Parallelism) — *is* the staging area. There is no separate proposal object: every write lands in the current task's working area immediately (no `begin`), and "propose / review / apply" is just `write / validate / finalize` over that area. The task id is the changeset name.

**Two check times** ([VISION.md](../VISION.md): integrity holds at `finalize`, not per-write):

- **Write-time — local adjudication.** A `set-field` rejects a malformed date or a non-member enum *now*; a mint rejects a bad slug. Fast, local feedback.
- **Finalize-time — integrity the task can fix.** Forward-ref resolution (dangling refs), required-slot presence, and unreconciled drift run at `finalize` — a ref may dangle in the working area until then, because its target can be created in the *same* task. **Finalize blocks only on what the task can fix itself**; inverse/minimum-cardinality ("a PRD needs a SPEC") depends on *other* tasks and is **completeness, not a per-task gate** (see [document-type-schema.md](document-type-schema.md), [validation.md](validation.md)). This split is what stops the bootstrap from deadlocking.

**`finalize` ≡ `validate` + commit.** Finalize has no private check path — it runs `validate` and commits only if it passes, so what `validate` reports and what `finalize` blocks on can never diverge. One task → one logical commit. The **ordered transaction** (preflight → validate → render → promote → stage → commit → post-commit), atomicity rules, rollback discipline, dirty-tree policy, and the commit-doc → git-message mapping are specified in [finalize.md](finalize.md).

- `validate` is **scope-flexible** — a task (the working area; what the hard-block uses), a single doc, or the whole store — one engine ([structural-grammar.md](structural-grammar.md#validation); probe split per [VISION.md](../VISION.md) principle #6).
- Findings are **severity-tagged**: *blocking* (integrity must hold) stops `finalize`; *advisory* is surfaced but doesn't. **Which severity a check carries is a cascade setting** — a project can promote or demote, say, `doc↔code` staleness without touching code.
- `finalize` **defaults to autonomous**; an opt-in cascade setting can require human confirmation. The durable correction point is the git/PR review regardless.

## Instance provisioning

A working instance must exist before you can `set-slot` into it. The CLI **always** owns the structural act — mint the id (slug from the type's id-source field) and place it at the schema-defined location ([structural-grammar.md](structural-grammar.md#ids-provenance-and-minting)). There are two triggers:

- **Workflow-provisioned** (deterministic) — the composed workflow creates the instances the task obviously needs (e.g. the task's commit doc) and fills the id-source field deterministically, yielding a task-derived id like `commit:add-rate-limiter`. The agent only fills slots.
- **Agent-initiated** (judgment) — `tool doc create adr --title "…"` when the agent decides a *new* doc is warranted mid-task. The CLI mints + places; the agent supplied only content.

This keeps the determinism boundary intact: **deciding a doc is warranted is reasoning (agent); creating and placing it is structure (CLI).** Agent-initiated `create` is **workflow-gated** — the catalog of types creatable in a given context is structure (workflow/cascade-owned); *choosing* among them is reasoning.

### The create-gate

The gate lives on the workflow's front-matter as **`allows-create: [<doctype-id>, ...]`** ([workflow-dialect.md](workflow-dialect.md) → On-disk definition format). **Default is empty — no agent-initiated creates allowed unless the workflow explicitly opts in.** Workflow-provisioned instances (e.g. the task's commit doc) bypass the gate; they're not shell-visible and are declared by the workflow's own structure.

**Two entry forms.** Each `allows-create` entry is either a bare doctype id (`adr`) or an object `{type: <doctype-id>, as: <role>}`. The object form additionally **declares a context role and binds the created instance to it**: creating that doctype under the gate makes the new instance reachable as `task.<role>` ([workflow-dialect.md](workflow-dialect.md)) with no separate bind step. MVP `single-task` ships `allows-create: [{type: adr, as: decision}]`, so an agent-created ADR is reachable as `task.decision` — the surface the superseding-decision context-slice composes from (`{{@task.decision.supersedes#decision}}`, see [worked-examples.md](worked-examples.md) → Superseding decision). The bare form grants create permission without declaring a role.

**Enforcement at every `tool doc create <type> --<args>`:**

1. Identify the active task — `.tool/tasks/<id>/` from cwd, or `--task <id>` explicit. **No active task** → reject: `"no active task — start one with \`tool start\`"`.
2. Identify the task's workflow and resolve its effective `allows-create` through the cascade ([overrides.md](overrides.md) → Resolution algorithm).
3. **Unknown doctype** (`<type>` not in the cascade-resolved schema set) → reject: `"unknown doctype \`<type>\`"`. (This is a separate failure mode from the gate; it fires before the gate check.)
4. `<type>` ∈ `allows-create` → proceed: mint id, place per the schema's `location:`, **bind it to the entry's `as:` role if the entry declares one**, return the new address.
5. `<type>` ∉ `allows-create` → reject with the structured gate-block error:

```text
error: workflow 'single-task' does not allow `tool doc create spec` in-task.
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

## Out-of-band reconciliation

Humans edit files directly, and because the **files are the source of truth** ([storage.md](storage.md)), the system **honors a clean edit, not merely tolerates it** ([VISION.md](../VISION.md) principle #3). The CLI never forbids file edits and never silently discards them — every drift is detected, classified, and routed.

The deterministic classifier (states, transitions, parse classification, hash re-baselining, MVP vs post-MVP scope) is specified in [reconciliation.md](reconciliation.md). Three outcomes the agent sees: **absorb** (clean edit accepted; hash advances), **conformance-block** (parse / schema failure surfaces precisely; never auto-repaired in MVP), **conflict-block** (both sides changed; explicit discard, no silent default; three-way merge deferred). `finalize` blocks on the latter two and absorbs the first.

## Worked example — the MVP write loop

```text
# task add-rate-limiter; the workflow has provisioned commit:add-rate-limiter (empty)

tool doc set-field commit:add-rate-limiter#type    --value feat
tool doc set-slot  commit:add-rate-limiter#summary --from-file -      # prose piped in
tool task validate add-rate-limiter                                   # blockers? (preview)
tool task finalize add-rate-limiter                                   # validate + commit
```

Agent-initiated create within the same task:

```text
tool doc create adr --title "Rate-limit at the gateway"   # → adr:rate-limit-at-the-gateway
tool doc set-field adr:rate-limit-at-the-gateway#status  --value accepted
tool doc set-slot  adr:rate-limit-at-the-gateway#context --from-file -
```

## Open questions

- **Form-marker syntax** — how the deferred fillable form delimits slots for unambiguous extraction (the slot/placeholder delimiters themselves are settled — `<<…>>` vs `{{…}}`).
- **`import` three-way merge** — the serialization round-trips (single-file Markdown, [storage.md](storage.md)), so `import` is unblocked; the remaining open part is the both-sides-changed three-way merge, shared with override conflict resolution ([overrides.md](overrides.md)).
- **Blocked/error payload** — the concrete shape a sub-agent writes to task state on a block ("import-vs-discard pending", "validation failed") and how it surfaces through the propose-to-human path.
