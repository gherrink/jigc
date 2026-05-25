# Workflow dialect

The second dialect over the skeleton: how a workflow is **defined** and **composed** into the instruction set the agent receives. This is the *compose* half of the context compiler — the read path's counterpart to the document dialect's write path.

Builds on [structural-grammar.md](structural-grammar.md) (the skeleton: steps, addressing, `include`, override), [document-type-schema.md](document-type-schema.md) (data-values navigate the document graph), [write-commands.md](write-commands.md) (command-refs route to write ops; the task/staging model), [storage.md](storage.md) (isolated working areas, by-task-id merge, and the md + front-matter file pattern definitions reuse), and [overrides.md](overrides.md) (definitions resolve through the cascade). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**; the emitted-format micro-syntax is an open question.

## Shared grammar, divergent runtime

The "one grammar, two dialects" bet holds where it claimed to — on the *definition grammar* — and was never a claim of runtime unity:

| | shared with the skeleton | workflow-specific |
|---|---|---|
| unit | step — ordered, ID'd, addressed `workflow:single-task#locate` | — |
| composition | `include` by id (used heavily) | — |
| override | `insert-step --after`, `replace`, `remove`, scalar | — |
| leaves | *(extension point)* | `instruction` (static) · `placeholder` (read-path) |
| repetition | *(extension point)* | `fan-out`/`join` marker |
| runtime | — | resolve & **emit** (no write path, no instances, no minting) |

So: **doc = fill & persist instances; workflow = resolve & emit a view.** There are no write-path leaves here — `slot` (LLM-filled, write) and `placeholder` (CLI-filled, read) stay strict opposites, and the workflow dialect has only the read-path kind.

## The composed output is a view

A composed workflow is **ephemeral**: a derived view, re-composed on every `tool workflow <x> --task <id>` call from `definition + cascade + live state`. It is never persisted and never a source of truth (consistent with storage's "composed = derived, rebuildable"). `--explain` re-computes the resolution tree on demand rather than reading a stored one.

A **flow diagram** (mermaid) is one such derived view: the CLI *generates* it from the composition (the ordered includes + `fan-out` markers) and can output it wherever useful — `--explain`, a `--diagram` flag, or leading the composed output. It is **descriptive, never prescriptive** — it reflects the includes, never defines flow — so it can't drift from the source or become a back-door for the banned DAGs/conditionals. Its value is mostly in visualizing `fan-out`; a straight sequence already reads as the ordered includes.

## Composition is substitution, not control flow

The load-bearing line, sharper than "no conditionals":

> A workflow's **structure** is fixed by `definition + cascade` and is **task-independent**. Only the **resolved values** vary by task.

Two tasks of the same type get the same steps in the same order; only the embedded data differs. No step appears or disappears based on runtime state. The composer is a deterministic substitution engine, not a workflow runtime. Adaptation therefore has exactly two homes, neither of them the composer:

- **to a project** → the **cascade** (recorded deltas decide which steps/commands exist);
- **to a situation** → the **agent's reasoning** (the workflow hands it all relevant steps; it skips what doesn't apply).

The **one bounded exception** is `fan-out`, where a runtime-resolved list yields N spawns — the single place runtime state affects structure, which is exactly why it is the single called-out concurrency primitive.

## Leaves: instructions and placeholders

A step's body is a block of two leaf kinds:

- **`instruction`** — static directive prose: "run this exact command," "author this doc slot," "reason about X." (The visual grammar that distinguishes these in the emitted output is the open emitted-format question.)
- **`placeholder`** — read-path, `{{…}}`, CLI-filled before the agent sees it. Three kinds:

| kind | resolves to | notes |
|---|---|---|
| **command-ref** `{{cli.add_phase}}` | a literal CLI invocation, via the cascade | project-overridable; validated by `workflow ↔ references` |
| **data-value** `{{task.spec#criteria}}` | a value or doc-slice from live state, embedded as text | the context-assembly core |
| **include** `{{include: step:validate-refs}}` | a step/block by id, expanded recursively | cycle-checked at validate-time |

Resolution order: includes expand first (pulling in nested placeholders), then data-values and command-refs resolve. An empty resolution yields **empty text** — never conditional inclusion of surrounding prose (that would be control flow).

**Data-values are full graph navigation, no logic** — and the path syntax turns on one rule that disambiguates the connectors:

> **`.` crosses a relation edge** (doc → related doc, *inter*-document); **`#` enters a doc** (doc → section/item/leaf, *intra*-document); **`:` names a doc literally** (`type:name`).

So a data-value is a **start** + `.relation` hops + an optional `#fragment` slice:

```
data-value := head ( "." relation )* ( "#" fragment )?
head       := root | type:name          # a live-state root, or a literal doc id
fragment   := unit ( "/" item )? ( "/" leaf )?   # the addressing fragment
```

`task.spec#criteria` = `task` root → `.spec` relation → `#criteria` section; `task.spec.derived-from#goal` = task → spec → the PRD → `#goal`; `prd:billing#goal` starts from a literal doc; `milestone.tasks` resolves to a **collection** (a `fan-out` source). The **root catalog** is declared, split on the engine/pack seam: engine-native `task` (the current task — its workflow-declared context roles, see [write-commands.md](write-commands.md) → Task origination) and `store` (the managed store, e.g. `store.findings`); pack-provided roots like `milestone`. A path resolves to a **scalar**, a **doc-slice**, or a **collection** — pure navigation, **no filtering or selection** — and one that fails to resolve is caught at validate-time by `workflow ↔ references`, not at runtime. (The `.` here is relation-traversal in placeholders; the dotted keys in cascade manifests like `validation.doc-code.severity` are a different grammar in a different context — no collision.)

## On-disk definition format

A step and a workflow are each **one file**, reusing the same Markdown + front-matter pattern as document instances ([storage.md](storage.md)) — interpreted for definitions:

- **A step** = a file: the YAML **front-matter** is the step's config (e.g. a `fan-out` marker); the **body is the prompt** — instruction prose with `{{placeholders}}`. The **id is the filename** (a frozen slug, like docs and items), so a plain step needs no front-matter at all — it's just a prompt body.
- **A workflow** = a file: front-matter for workflow metadata — including a one-line **`when`** hint the router uses for selection; the **body is the ordered `{{include}}`s** — which *is* the composition and the ordering (principle #2: the workflow file is the ordered list; steps are the ID'd, reusable units it references).

```markdown
# steps/locate.md   — a plain step: id from filename, body is the prompt
Read the spec for this task:
{{ task.spec#criteria }}
```

```markdown
# steps/implement-tasks.md   — a fan-out step: the marker lives in front-matter
---
fan-out:
  over: "{{ milestone.tasks }}"
  run:  workflow:single-task
---
Spawn a sub-agent per task and implement it.
```

```markdown
# workflows/single-task.md   — front-matter carries the when-to-use hint; body is ordered includes
---
when: "Implement one well-scoped change against an existing spec."
---
{{ include: step:locate }}
{{ include: step:implement }}
{{ include: step:finalize }}
```

Because a step is a file with a stable id, **steps resolve through the cascade** ([overrides.md](overrides.md)): a step-id resolves to the highest-precedence layer's file, so overriding a pack step is just "the project ships its own `validate.md`, and project wins" — no special override machinery. Step files are reusable across workflows; that's what `include` references.

(The doc-type *schema* definition format is config-structured — structured YAML, not this pattern — and is specified in [document-type-schema.md](document-type-schema.md).)

## Workflow selection — the router default

Which workflow a task runs is **not** the composer's job (composition is substitution, not control flow) and **not** the CLI's (it can't infer, model-free). It is resolved at the front door ([write-commands.md](write-commands.md) → Task origination):

- `tool start --workflow <X>` composes `X` directly.
- `tool start` with no workflow composes the **router** — the cascade-default workflow whose job is *selection*. The router is an ordinary composed workflow: its body assembles the **catalog** of available workflows with each one's **`when`** hint plus a *recommended* default, and instructs the agent to re-run `tool start --workflow <chosen>`. The agent picks; the CLI never does.

So selection reuses the workflow machinery (no special selection logic), stays model-free, is cascade-overridable (a project can rewrite its routing advice, or set the default to a specific work-workflow), and **never forces** — the worst case is the agent gets routing help. A workflow earns its place in the router by declaring its one-line `when` hint, so adding a workflow surfaces it automatically.

## `fan-out` / `join`

The single bounded concurrency primitive, assembled from the locked concurrency invariant, the blackboard model, and the storage decisions:

- A **`fan-out` step** declares a **list-source** — a data-value resolving to a *collection* (e.g. `{{milestone.tasks}}` from a many-relation, or `{{spec#criteria}}` from a repeatable section's items) — and a **referenced sub-workflow** each spawn runs.
- The **CLI resolves the list and emits dispatch instructions** ("spawn these task-ids running `W`"); the **assistant adapter launches** them. The locked seam holds — *CLI owns the payload, the assistant owns the launch* — and the CLI never spawns (it makes no agent calls).
- Each **sub-agent re-enters the composer** (`tool workflow W --task <sub>`) and gets the *same deterministic composed workflow* it would get as a main agent — **referenced, not inline** (inline would reintroduce a lossy paraphrase). It writes to its isolated `.tool/tasks/<sub>/`, acks `status + task_id`, and writes any detail to CLI state; the main agent **re-derives from the CLI, never trusts the message**. A sub-agent never commits (see the join, below).
- **Sub-task id = the fanned item's id** — the list-source resolves to a collection of managed instances, each carrying a frozen minted id, and that id becomes the sub-task id, so the by-task-id merge is meaningful and deterministic.
- **`join` is a barrier:** all sub-tasks complete, the CLI merges working areas **by task-id order** (never completion order), and the main workflow re-composes its post-join steps against the merged state. Synchronization is through CLI state, not messages.
- **The parent task's `finalize` is the commit boundary** — sub-agents may `validate` their own area for early feedback but **never run git**, so there are no races. The parent's `finalize` validates the merged effective state and emits **one commit per sub-task in task-id order** (squash is a cascade knob), plus the parent's own commit. "One task → one logical commit" is preserved per sub-task, deterministically ordered.
- **Slug collisions resolve in the merge pass** — if two sub-agents mint the same slug, the join suffixes the loser (task-id order) and **rewrites that area's *local* self-references** to match (the CLI owns wiring — the agent placed nothing). Isolation guarantees the only references to a sub-agent's new doc are from its own area, so cross-area rewriting never arises; workflow-provisioned ids are sub-task-derived and can't collide at all.

Two bounds, both consistent with prior decisions:

- **Fan out over the whole collection** — no "fan out over the subset matching X" (selection is logic); each sub-agent's own workflow + reasoning handles "nothing to do here," like the no-conditionals stance.
- **No nested `fan-out`** — parallelism is capped at one level, mirroring the doc dialect's "no repeatable-in-repeatable." A within-task parallel need is the trigger to revisit; tree-parallelism is deferred.

## How a workflow composes

A definition (placeholders unresolved):

```text
# workflow: single-task-execution
## locate
Read the spec for this task:
{{ task.spec#criteria }}
## implement · author the commit prose
Run: {{ cli.set-commit-summary }}
{{ author: commit.summary }}
## finalize
Run: {{ cli.finalize }}
```

Composed for task `add-rate-limiter` — structure identical, values resolved (emitted markers illustrative, pending the format decision):

```text
# single-task-execution · task add-rate-limiter
## locate
Read the spec for this task:
> SPEC auth-flow — criteria
> • Rate limit holds at 100/min → maps-to-test: test/rate_limit_spec.rb#burst

## implement · author the commit prose
Run: tool doc set-slot commit:add-rate-limiter#summary --from-file -
<<author: commit:add-rate-limiter#summary>>

## finalize
Run: tool task finalize add-rate-limiter
```

The data-value resolved to a doc-slice, the command-ref resolved to a literal command through the cascade, and the `author` directive points the agent at a *document* slot to fill via the write path — the workflow dialect itself stores nothing.

## Open questions

- **Emitted-format micro-syntax** — the concrete visual grammar that marks "run this exact command" vs "author this doc slot" vs "reason about X" in the composed output (the underlying slot/placeholder delimiters are settled).
- **Workflow progress / resumption** — whether "where am I" is purely re-derived from accumulated task effects (idempotent re-compose) or lightly tracked. Leaning re-derived, to match the ephemeral + blackboard model; not yet decided.
- **`milestone-execution` orchestration** — how the milestone workflow partitions and recombines worktree-isolated tasks (cross-refs [storage.md](storage.md) → CLI and git).
