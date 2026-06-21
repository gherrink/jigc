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

A composed workflow is **ephemeral**: a derived view, re-composed on every `jigc workflow <x> --task <id>` call from `definition + cascade + live state`. It is never persisted and never a source of truth (consistent with storage's "composed = derived, rebuildable"). `--explain` re-computes the resolution tree on demand rather than reading a stored one.

A **flow diagram** (mermaid) is one such derived view: the CLI *generates* it from the composition (the ordered includes + `fan-out` markers) and can output it wherever useful — `--explain`, a `--diagram` flag, or leading the composed output. It is **descriptive, never prescriptive** — it reflects the includes, never defines flow — so it can't drift from the source or become a back-door for the banned DAGs/conditionals. Its value is mostly in visualizing `fan-out`; a straight sequence already reads as the ordered includes.

### `--explain` output contract

The resolution tree shows exactly **what the composer did to get from definition to emitted text** — five layers, in resolution order, so a reader can debug cascade, include, and placeholder problems without reading internal state:

1. **Workflow & cascade provenance** — the workflow file, the layer that won (pack-default / team / project), any scalar-key overrides applied with their source layer, and (M6) any **rejected** scalar-sets — a demotion soft-rejected by a knob's `floor` (logged, not applied), shown distinctly from applied overrides with the attempted value, the floor, and its source layer ([overrides.md](overrides.md) → Locked keys).
2. **Include expansion tree** — each step pulled in by id, with its source file and source layer; nested includes shown under their parent so a cycle or unexpected pull is visible in the shape.
3. **Command-ref resolution** — for each `{{cli.…}}`, the cascade path walked (which layers had an entry, which won), the template invocation at that layer, and the final substituted result.
4. **Data-value path resolution** — for each `{{path}}` / `{{@path}}`, the parsed path (head + `.relation` hops + `#fragment`), the live-state root it started from, what it resolved to (**scalar** / **address** / **content** / **collection**), and — for `@` — the address it dereferenced.
5. **Compose-time findings** — anything `workflow-refs` flagged: dangling include, dangling placeholder, `@` on a scalar (a conformance error), include cycle. With addresses.

Illustrative shape (exact framing settles with the emitted-format micro-syntax, see [open questions](#open-questions)):

```text
workflow:single-task                              (pack-default · dev/v0.3.0)
  overrides applied: none
  includes:
    step:locate                                   (project · ./steps/locate.md  ← overrides pack-default)
      {{@task.spec#criteria}}
        path:     task → .spec → #criteria
        @-deref:  spec:auth-flow#criteria → <doc-slice, 17 lines>
    step:implement                                (pack-default · dev/steps/implement.md)
      {{cli.set-commit-summary}}
        cascade:  project (—) → team (—) → pack-default (matched)
        template: "jigc doc set-slot {{addr}} --from-file -"
        result:   "jigc doc set-slot commit:add-rate-limiter#summary --from-file -"
      {{task.commit#summary}}
        path:     task → .commit → #summary
        resolved: commit:add-rate-limiter#summary (address)
  findings (workflow-refs): 0
```

The tree is **derived** like the composed workflow itself — re-computed each call, never persisted; the same `definition + cascade + live state` always yields the same tree.

## Composition is substitution, not control flow

The load-bearing line, sharper than "no conditionals":

> A workflow's **structure** is fixed by `definition + cascade` and is **task-independent**. Only the **resolved values** vary by task.

Two tasks of the same type get the same steps in the same order; only the embedded data differs. No step appears or disappears based on runtime state. The composer is a deterministic substitution engine, not a workflow runtime. Adaptation therefore has exactly two homes, neither of them the composer:

- **to a project** → the **cascade** (recorded deltas decide which steps/commands exist);
- **to a situation** → the **agent's reasoning** (the workflow hands it all relevant steps; it skips what doesn't apply).

The **one bounded exception** is `fan-out`, where a runtime-resolved list yields N spawns — the single place runtime state affects structure, which is exactly why it is the single called-out concurrency primitive.

**A `checkpoint` step halts execution; it does *not* branch composition (M15).** The invariant above bars a *conditional* — a step that appears or disappears on runtime state. A [`checkpoint` step](#the-checkpoint-step-kind-m15) is the opposite shape: it **always composes** (every workflow of a type emits the same checkpoint steps in the same order — structure stays task-independent) and emits a reserved `Checkpoint:` directive the agent/orchestrator **honors by stopping** to surface a decision to the human. Whether the run then proceeds is the human's call, *outside* jigc — not a composer branch. This is the generalization of `finalize`'s existing blocking gate (`has_blocking()` stops the commit) lifted to an arbitrary point in a workflow: an **execution gate**, never a step-selection conditional. The composer remains a pure substitution engine emitting the whole workflow text; the binary grows no loop, no progress cursor, no runtime — resumption after a halt is the existing **stateless restart-from-scratch** re-compose (`jigc start --task <id>` re-derives the view from accumulated task effects; see [Open questions](#open-questions)). So a checkpoint adds *halting*, which the invariant never forbade, without adding *branching*, which it does.

## Leaves: instructions and placeholders

A step's body is a block of two leaf kinds:

- **`instruction`** — static directive prose: "run this exact command," "author this doc slot," "reason about X." (The visual grammar that distinguishes these in the emitted output is the open emitted-format question.)
- **`placeholder`** — read-path, `{{…}}`, CLI-filled before the agent sees it. Four kinds:

| kind | resolves to | notes |
|---|---|---|
| **command-ref** `{{cli.add_phase}}` | a literal CLI invocation, resolved against the [command catalog](command-catalog.md) via the cascade | typed args (literal / `from:` / `agent:`); project-overridable; validated by `workflow-refs.command-ref-resolves` |
| **data-value** `{{path}}` / `{{@path}}` | by default, the path's **address**; with `@`, the **content** at that address (see below) | the context-assembly core |
| **include** `{{include: step:validate-refs}}` | a step/block by id, expanded recursively | cycle-checked at validate-time (phase 6 of [resolution algorithm](overrides.md#resolution-algorithm)) |
| **fill** `{{fill: extra-guidance}}` | the content a `slot-fill` delta supplies for this extension point, or the pack default body (empty if none) | a pack-declared override point; applied at phase 5 *before* expansion; **no nested `{{fill:}}`**; orphan + survivor flagged by `workflow-refs` ([overrides.md](overrides.md#the-fill-placeholder--slot-fill-targets)) |

**Address vs content — the `@` marker.** A data-value path resolves to the **address** of its target by default; prefix with `@` to dereference and get the **content** at that address. One uniform rule across all depths:

| form | resolves to |
|---|---|
| `{{task.commit}}` | `commit:add-rate-limiter` (the address) |
| `{{@task.commit}}` | the commit doc's content |
| `{{task.spec#criteria}}` | `spec:auth-flow#criteria` (the address) |
| `{{@task.spec#criteria}}` | the criteria section's content (doc-slice) |
| `{{task.intent}}` | the scalar string (no address/content distinction) |

Mnemonic: bare path = the **reference**; `@` = at that reference (= the content), like `@username` ("at the user") or `*ptr` ("dereference"). `@` on a path that resolves to a scalar (`task.intent`) is a **conformance error** caught at compose-time by `workflow-refs`.

**Slot-author directives — same syntax in definition and emitted.** An `instruction` to "author this doc slot" is written in workflow definitions with the same `<<author: <address>>` slot syntax used in emitted output — kept strict opposites with `{{…}}` per principle #6 (an author directive is *never* a placeholder). The `<address>` parameter is a data-value path inside `{{…}}` (which, per the rule above, resolves to an address by default): `<<author: {{task.commit#summary}}>>` composes to `<<author: commit:add-rate-limiter#summary>>`. The `<<…>>` wrapper survives composition unchanged; only the embedded `{{…}}` resolves.

Resolution order: includes expand first (pulling in nested placeholders), then data-values and command-refs resolve. An empty resolution yields **empty text** — never conditional inclusion of surrounding prose (that would be control flow).

**Empty vs unresolvable — a sharp line.** A path that is *structurally valid* but currently resolves to nothing — an **unbound optional context role** (nothing bound to `task.decision` because the agent created no ADR) or an **unset `0..1` ref** (`supersedes` not set) — yields **empty text**, not a finding. Only a *structurally invalid* path — an undeclared root, role, relation, section, or leaf — is a `workflow-refs` error. This is what lets the always-present `superseded-context` step ([On-disk definition format](#on-disk-definition-format)) compose cleanly in the common case: the placeholder is valid, the value is absent, the line emits nothing. (The `@`-on-a-scalar case stays a conformance error — that's a malformed path, not an absent value.)

## Emitted format

The composed output is plain Markdown, but the **agent must distinguish at a glance** between five line-classes — and reliably, because this is the surface where the structural-determinism bet either holds or leaks. Five classes, five conventions:

| class | how it's written in the emitted text | source |
|---|---|---|
| **Run** — execute exactly | `` Run: `<cmd>` `` at line-start, command in backticks | resolved `{{cli.…}}` command-ref |
| **Content** — read this material | a Markdown blockquote (`> `) | resolved `{{@…}}` data-value |
| **Author** — write into this slot | `<<author: <address>>` on its own line | the only surviving slot-author directive |
| **Spawn** — launch a sub-agent for this id | `` Spawn: `<launch-line>` `` at line-start, command in backticks, **one line per fanned id** | a resolved `fan-out` step over a `{{…}}` collection (M8) |
| **Checkpoint** — stop here and surface to the human | `Checkpoint: <reason>` at line-start, the reason slug bare (no backticks) | a `checkpoint` step kind (M15) |
| **Reason** — think about this | plain prose, no marker | static `instruction` text |

The four "machine" classes (**Run**, **Author**, **Spawn**, **Checkpoint**) carry markers; the two "human-ish" classes (**Content**, **Reason**) lean on existing Markdown semantics or no marker at all. **Reasoning is the default** — anything that isn't one of the other five is reasoning, with no overhead.

The six rules:

1. **`Run: ` is reserved.** A line whose left margin starts with `Run: ` is a directive the composer emits, never authored by step prose. The command always follows in backticks: `` Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -` ``. Backticks make the command both visually distinct *and* machine-extractable by a strict line-pattern (`` ^Run: `(.+)`$ ``).
2. **`> ` blockquote is content** — emitted around the resolution of a `{{@…}}` data-value. A multi-line doc-slice is a multi-line blockquote. Renders cleanly in any Markdown viewer; the agent reads it as "this is material I was given," not "this is something to run."
3. **`<<author: <address>>` is the only thing the agent originates.** Its rules are settled in [Leaves](#leaves-instructions-and-placeholders): exactly one address parameter; the address is the doc slot the agent fills via the write path; the `<<…>>` wrapper survives composition unchanged.
4. **`Spawn: ` is reserved (M8).** A line whose left margin starts with `` Spawn: `` is a fan-out dispatch directive the composer emits, never authored by step prose — exactly the `Run: ` discipline applied to the launch class. A `fan-out` step resolves its `over:` collection (`{{milestone.tasks}}`) and emits **one `` Spawn: `<launch-line>` `` per fanned id**, where `<launch-line>` is the sub-agent launch rendered through the adapter's spawn template (`jigc workflow <run-workflow> --task <sub-id>`, [assistant-adapter.md](assistant-adapter.md) → Bind the spawn mechanism). The command sits in backticks, machine-extractable by the same strict line-pattern as `Run`. The CLI resolves the list and emits the directives deterministically; the adapter performs the launch — *CLI owns the payload, the assistant owns the launch*.
5. **`Checkpoint: ` is reserved (M15).** A line whose left margin starts with `Checkpoint: ` is a halt directive the composer emits, never authored by step prose — exactly the `Run: ` / `Spawn: ` discipline applied to the halt class. A [`checkpoint` step](#the-checkpoint-step-kind-m15) emits **one `Checkpoint: <reason>` line**, where `<reason>` is the step's front-matter `reason` slug (bare, not backticked — it is a label, not a command). **The directive is emitted at the head of the step's text, *before* the body prose** — unlike `Spawn:`, which appends *after* the body, because the halt must be read before the prose that explains when to honor it. The agent/orchestrator reads it as "stop here and surface to the human"; the binary emits it unconditionally and runs no logic on it (recognition is convention, not parser-enforcement — the **Honest boundary** note below applies as it does to the other markers).
6. **Everything else is reasoning prose** — no marker, no special handling, no overhead. The agent treats it as instruction it should think with.

**Compose-time conformance.** Because **Run**, **Spawn**, and **Checkpoint** are load-bearing markers, instruction prose in a step definition **must not** start a line with `Run: `, `Spawn: `, or `Checkpoint: ` — those prefixes are the composer's. The `workflow-refs` probe ([validation.md](validation.md)) checks this at compose-time as part of its existing scope: the same probe that validates every placeholder/include/command-ref resolves also validates that a `Run: ` line-start comes only from a resolved `{{cli.…}}` (`run-marker-not-shadowed`), a `Spawn: ` line-start comes only from a resolved `fan-out` step (`spawn-marker-not-shadowed`, M8), and a `Checkpoint: ` line-start comes only from a resolved `checkpoint` step (`checkpoint-marker-not-shadowed`, M15). All three fire against the **de-included step body *before* directive emission** (the same `step.body` surface `run-marker-not-shadowed` already scopes — placeholders unresolved, the composer's own `Run:`/`Spawn:`/`Checkpoint:` directives not yet emitted). For `checkpoint-marker-not-shadowed` this scope is load-bearing: a checkpoint step's *own* emitted `Checkpoint:` directive is legitimate, so the check must read the pre-emit body, never the post-emit text — otherwise every real checkpoint step would self-trip. A definition that shadows any marker is rejected with a precise pointer (step file + line) before the composed output ever reaches the agent. `> ` and `<<author:` need no such check — blockquotes are legitimately part of reasoning prose, and `<<…>>` is grammatically slot-syntax (already off-limits to definition prose by principle #6).

**Routing footer (compaction resilience).** Every composed workflow output in agent-text and human-pretty format ends with a one-line routing footer:

```
— jigc · run `jigc start` for orientation; all writes through `jigc`.
```

The footer reinforces the bootstrap routing pointer on every CLI emission, so context compaction can't strip the agent's a-priori knowledge that the CLI is the interface ([bootstrap.md](bootstrap.md) → Context compaction resilience). It is CLI-emitted at phase 9 of the [resolution algorithm](overrides.md#resolution-algorithm), after step content; a step body never authors it. JSON-format output (consumed by tooling, not the agent's reading flow) carries no footer. Stays within the *routing, not content* discipline: names the entrypoint, embeds no rules.

**Honest boundary.** Recognition of these four markers is **convention**, not parser-enforced on the agent side — agents read the emitted text as humans do, and the markers' job is to be unambiguous *to a reader*, not to be machine-parsed downstream. Consistent with [VISION.md](../VISION.md) principle #3 (compliance is by ergonomics, not sandbox). The CLI guarantees the *shape* of the emitted text; reading it as intended is the agent's responsibility.

**Data-values are full graph navigation, no logic** — and the path syntax turns on one rule that disambiguates the connectors:

> **`.` crosses a relation edge** (doc → related doc, *inter*-document); **`#` enters a doc** (doc → section/item/leaf, *intra*-document); **`:` names a doc literally** (`type:name`).

So a data-value is a **start** + `.relation` hops + an optional `#fragment` slice:

```
data-value := "@"? head ( "." relation )* ( "#" fragment )?
head       := root | type:name          # a live-state root, or a literal doc id
fragment   := unit ( "/" item )? ( "/" leaf )?   # the addressing fragment
```

`task.spec#criteria` = `task` root → `.spec` relation → `#criteria` section; `task.spec.derived-from#goal` = task → spec → the PRD → `#goal`; `prd:billing#goal` starts from a literal doc; `milestone.tasks` resolves to a **collection** of the milestone's sub-task work-unit ids (a `fan-out` source). The **root catalog** is declared, split on the engine/pack seam — but all current roots are **engine-native**: `task` (the current task — its workflow-declared context roles, see [write-commands.md](write-commands.md) → Task origination), `store` (the managed store, e.g. `store.findings`), `catalog` (the live catalog of **selectable work-workflows** with their `when` hints — the router's input, see [Workflow selection](#workflow-selection--the-router-default)), and the **work-unit root `milestone`** — like `task`, an engine-native live-state root over the work-unit family ([structural-grammar.md](structural-grammar.md#work-units-and-runtime-identity)), *not* pack content; its `.tasks` resolves to the milestone's sub-task collection. **The `milestone` *data-value root* lands in M8**, with its only consumer — the `fan-out` step kind that reads the collection; in M7 the `milestone` work-unit exists as engine state and the join reads its task list directly (no placeholder), so the read-path root waits for the step that needs it. `catalog` is engine-native machinery, not pack content — it enumerates whatever work-workflows the cascade supplies, so the "engine ships empty" invariant holds; it resolves deterministically for a given cascade (same cascade → same catalog). `store.<doctype-id>` enumerates the committed instances of that doctype as a **collection** of addresses (e.g. `store.specs` → every committed `spec` — pure navigation, no filtering); a collection interpolated in an ordinary (non-`fan-out`) step emits as a readable **Content** list, which is how `implement-from-spec`'s `locate-from-spec` step surfaces the bindable specs before `jigc task bind` ([worked-examples.md](worked-examples.md) → Spec-driven planning). A path resolves to one of: a **scalar** (`task.intent`); an **address** (any other path, bare); the **content at that address** (any other path with `@`); or a **collection** of addresses (a `fan-out` source, or a Content list in a plain step) — pure navigation, **no filtering or selection** — and one that *structurally* fails to resolve (an undeclared root/role/relation/fragment — distinct from a valid path whose value is merely absent, which yields empty text, see [Leaves](#leaves-instructions-and-placeholders)) is caught at validate-time by `workflow-refs`, not at runtime. (The `.` here is relation-traversal in placeholders; the dotted keys in cascade manifests like `validation.doc-code.severity` are a different grammar in a different context — no collision.)

## On-disk definition format

A step and a workflow are each **one file**, reusing the same **shape** as document instances — a Markdown body under a `---`-fenced front-matter block ([storage.md](storage.md)) — but **not the same front-matter grammar**: a definition's front-matter is **config-family YAML** (nested keys like `fan-out.over`, `allows-create: [{type, as}]`), parsed by the config parser, *not* the flat `key: value` field block used for document instances ([parsing.md](../implementation/parsing.md) → Front-matter; [overrides.md](overrides.md)). Interpreted for definitions:

- **A step** = a file: the YAML **front-matter** is the step's config (e.g. a `fan-out` marker); the **body is the prompt** — instruction prose with `{{placeholders}}`. The **id is the filename** (a frozen slug, like docs and items), so a plain step needs no front-matter at all — it's just a prompt body.
- **A workflow** = a file: front-matter for workflow metadata — a one-line **`when`** hint the router uses for selection, a **`creates-task`** boolean (default `true`) declaring whether running this workflow mints a task ([write-commands.md](write-commands.md) → Task origination), a **`selectable`** boolean (default `true`) governing **router-catalog membership only** — whether the router's `{{catalog}}` root lists this workflow as a selectable work-option. ([`describe`](introspection.md) is unaffected: it narrates the **unfiltered** workflow set, gated on `description`/`usage` presence, never on `selectable`.) `selectable` is **orthogonal to `creates-task` and never gates minting or provisioning** — those gate on `creates-task` alone — so a `creates-task: true, selectable: false` workflow mints a task and provisions its commit doc like any other task, yet stays **off** the router catalog (the fan-out `sub-task`, the methodology authoring workflows). Coupling `selectable` into a minting or provisioning gate is a bug, not a symmetry: the two concerns are independent by design. An optional **`allows-create: [<doctype-id>, ...]`** list naming the doctypes the agent may `jigc doc create` during the task (default empty — no agent-initiated creates allowed; [write-commands.md](write-commands.md) → Instance provisioning), and an optional **`reads: [{role: <role>, type: <doctype-id>}, ...]`** list declaring **context roles bound from existing committed docs** (default empty), and optional **`description`** / **`usage`** authored-prose fields the [`describe`](introspection.md) self-description surface projects (default absent — describe omits an undescribed workflow; `description` = what it *is*, `usage` = when/why to reach for it, never mechanism); the **body is the ordered `{{include}}`s** — which *is* the composition and the ordering (principle #2: the workflow file is the ordered list; steps are the ID'd, reusable units it references).

  `reads` is the dual of `allows-create`'s object form: `allows-create … as:` declares a role bound to a doc the task *creates*; `reads` declares a role bound to a doc a *prior* task committed, via `jigc task bind <role> <addr>` ([write-commands.md](write-commands.md) → Binding a context role). Both make `task.<role>` a **declared** root for `workflow-refs` — a `task.<role>` whose role appears in neither is a conformance error; a declared-but-unbound role resolves to empty text. A `reads` role is **always optional** in the M3 surface (unbound = empty, no finalize gate) — there is no required-read-role cardinality yet, so `reads` carries no `card`; the rule for a required read-role (block finalize when unbound) is deferred until a workflow earns one. The spec-driven `implement-from-spec` workflow declares `reads: [{role: spec, type: spec}]` so its `locate-from-spec` step's `{{@task.spec#criteria}}` validates and resolves over the bound spec.

```markdown
# steps/locate.md   — a plain step: id from filename, body is the prompt
Read the spec for this task:
{{ task.spec#criteria }}
```

```markdown
# steps/superseded-context.md   — a plain step: surfaces the decision this task supersedes, if any
If your decision supersedes an earlier one, set `supersedes` on the ADR; the
superseded decision then appears below for reference, so your consequences can
explain what changes (nothing appears if it supersedes none).
{{ @task.decision.supersedes#decision }}
```

```markdown
# steps/implement-tasks.md   — a fan-out step: the marker lives in front-matter
---
fan-out:
  over: "{{ milestone.tasks }}"
  run:  workflow:sub-task
---
Spawn a sub-agent per task and implement it.
```

```markdown
# steps/join-tasks.md   — a join step: the barrier, also a front-matter marker
---
join: {}
---
All sub-tasks are complete and merged by task-id order. Continue.
```

A **`join` step kind** is the **barrier**: the CLI blocks composition of every following step until the fanned sub-tasks complete, merges their areas by task-id order ([storage.md](storage.md#the-by-task-id-join-m7)), and re-composes the post-join steps against the merged state. A workflow with a `fan-out` step **must** carry a later `join` step — `join` with no preceding `fan-out`, or a `fan-out` with no following `join`, is a `workflow-refs` conformance error. These step kinds are parsed from front-matter exactly like any other config; the **id is still the filename**, so a `fan-out`/`join` step is reusable across workflows like a plain step. (The third kind, [`checkpoint`](#the-checkpoint-step-kind-m15), is parsed the same way — M15.) *(The `fan-out`/`join` front-matter must be **parsed and honored**, not stripped — M8 ([DECISIONS.md](../DECISIONS.md) 2026-06-04): pre-M8 `load_step_def` discards all step front-matter, so a `fan-out:` marker today silently composes as inert prose.)*

```markdown
# workflows/single-task.md   — front-matter carries the when-to-use hint + create-gate; body is ordered includes
---
when: "Implement one well-scoped change against an existing spec."
allows-create: [{type: adr, as: decision}]   # agent may `jigc doc create adr ...`; the created ADR binds to task.decision
---
{{ include: step:locate }}
{{ include: step:implement }}
{{ include: step:superseded-context }}
{{ include: step:finalize }}
```

A workflow that **doesn't** mint a task — the router is the canonical case — declares `creates-task: false`:

```markdown
# workflows/router.md   — composes selection guidance; routes to an explicit --workflow call
---
when: "Help me pick a workflow."
creates-task: false
---
{{ include: step:present-catalog }}
{{ include: step:route-to-workflow }}
```

The **`milestone-execution`** workflow (M8) is the canonical `fan-out`/`join` composition — its body pairs a `fan-out` step with a later `join` step (the `fan-out-join-paired` conformance rule), so the pairing is visibly satisfied:

```markdown
# workflows/milestone-execution.md   — provisions worktrees, fans out over the task list, joins, then finalizes
---
when: "Execute a planned milestone's tasks in parallel."
creates-task: false   # operates on an existing milestone work-unit; mints no task
---
{{ include: step:provision-worktrees }} # Run: provision N detached worktrees at the milestone base pin (one per sub-task, gitignored)
{{ include: step:implement-tasks }}   # fan-out: over {{milestone.tasks}}, run workflow:sub-task (each sub-agent in its own worktree)
{{ include: step:join-tasks }}         # join: barrier — merge managed docs by task-id, then continue
{{ include: step:milestone-finalize }} # parent finalize: combine the worktree-staged code (disjoint-apply, id order) + promote docs — the commit boundary
```

The leading **`provision-worktrees`** step is an ordinary `Run:` step ([Emitted format](#emitted-format)) — it composes to `` Run: `jigc milestone …` `` and provisions one ephemeral, gitignored git worktree per sub-task at the milestone base pin, so each fanned sub-agent has its own index/HEAD to `git add` code into ([storage.md](storage.md#cli-and-git) → the three combine modes). The worktrees are torn down at finalize.

The fanned **`sub-task`** workflow (the `run:` target) is **fan-out-free by construction** — `{{ include: step:locate }}` / `step:implement` / `step:author-commit`, no `finalize`, no `fan-out` step — which is what structurally guarantees the no-nested-`fan-out` rule (below): a sub-agent's workflow can never itself fan out.

**Workflow body is include-only at top level.** A workflow file's body may contain only `{{include: step:X}}` lines, blank lines, and HTML comments (`<!-- ... -->`, for human notes about why a step is included). Prose, ATX headings, other placeholders, or any other content at body top-level is a **conformance error** caught at compose-time by `workflow-refs.body-include-only` ([validation.md](validation.md) → Severity inventory). Step files carry the prose; the workflow file is purely composition. The physical-order serialization of the include list satisfies [VISION.md](../VISION.md) principle #2: "never positions" bans IDs that *encode* order, not physical-order serialization of a list whose items carry their own non-positional IDs — a line move is a reorder, nothing renumbers, cross-refs to `workflow:single-task#locate` keep resolving regardless of include position.

Because a step is a file with a stable id, **steps resolve through the cascade** ([overrides.md](overrides.md) → Resolution algorithm): a step-id resolves to the highest-precedence layer's file (atomic file-level shadowing in phase 2), so overriding a pack step is just "the project ships its own `validate.md`, and project wins" — no special override machinery. Step files are reusable across workflows; that's what `include` references. Structural deltas (`insert-step`, `replace-step`, `remove-step`) operate on the workflow's **include list** in phase 4, *before* include expansion in phase 7 — same `(definition + cascade)` in → same composition.

(The doc-type *schema* definition format is config-structured — structured YAML, not this pattern — and is specified in [document-type-schema.md](document-type-schema.md).)

## Workflow selection — the router default

Which workflow a task runs is **not** the composer's job (composition is substitution, not control flow) and **not** the CLI's (it can't infer, model-free). It is resolved at the front door ([write-commands.md](write-commands.md) → Task origination):

- `jigc start --workflow <X>` composes `X` directly.
- `jigc start` with no workflow composes the **router** — the cascade-default workflow whose job is *selection*. The router is an ordinary composed workflow: its body interpolates the engine-native **`catalog`** root — the live list of **selectable work-workflows** (those declaring `creates-task: true` **and** `selectable: true`, the default — `selectable` is the catalog-membership flag, orthogonal to minting and provisioning; see [On-disk definition format](#on-disk-definition-format)) with each one's **`when`** hint, so adding a work-workflow surfaces it automatically and the router never lists itself, any other `creates-task: false` workflow, or a `selectable: false` one — and instructs the agent to re-run `jigc start --workflow <chosen> "<intent>"` (an **agent-substitution** pattern, not a resolved value — the agent re-supplies its own intent). The agent picks; the CLI never does. *(A workflow-level **recommended** default is post-M2: with two `when`-hinted options the agent chooses unaided, and surfacing a recommendation would need its own cascade knob — `default-workflow` now names the router — which is generality for a single use until earned.)*

So selection reuses the workflow machinery (no special selection logic), stays model-free, is cascade-overridable (a project can rewrite its routing advice, or set the default to a specific work-workflow — as the **MVP** does, defaulting to `single-task` until ≥2 work-workflows exist, see [CLAUDE.md](../CLAUDE.md) → MVP scope), and **never forces** — the worst case is the agent gets routing help. A workflow earns its place in the router by declaring its one-line `when` hint, so adding a workflow surfaces it automatically.

## `fan-out` / `join`

The single bounded concurrency primitive, assembled from the locked concurrency invariant, the blackboard model, and the storage decisions.

> **Built across two milestones (split 2026-06-04, [DECISIONS.md](../DECISIONS.md)).** The **data plane is M7**: the `milestone` work-unit (engine state) + its task-list verbs, the isolated sub-task area dirs + the **join-time** isolation check, and the **by-task-id join merge** itself ([storage.md](storage.md#the-by-task-id-join-m7)) — proven deterministic by permutation tests over real multi-area fixtures, without any real spawn. The **control plane is M8**: the `fan-out`/`join` **step kinds** + the spawn-instruction emit class (a new emitted-format directive), the **`{{milestone.tasks}}` data-value root** the fan-out step consumes, the **adapter spawn binding** + the `jigc workflow W --task <id>` re-entry + the **write-time `--task`-scoped barrier**, and the **`milestone-execution` workflow** that wires it end-to-end. The bullets below describe the whole primitive; each clause is tagged M7 (data) or M8 (control) only where it isn't obvious.

- A **`fan-out` step** declares a **list-source** — a data-value resolving to a *collection* (e.g. `{{milestone.tasks}}` from a many-relation, or `{{spec#criteria}}` from a repeatable section's items) — and a **referenced sub-workflow** each spawn runs.
- The **CLI resolves the list and emits dispatch instructions** ("spawn these task-ids running `W`"); the **assistant adapter launches** them. The locked seam holds — *CLI owns the payload, the assistant owns the launch* — and the CLI never spawns (it makes no agent calls).
- Each **sub-agent re-enters the composer** (`jigc workflow W --task <sub>`) and gets the *same deterministic composed workflow* it would get as a main agent — **referenced, not inline** (inline would reintroduce a lossy paraphrase). It writes its **managed docs** to its isolated `.jigc/tasks/<sub>/` area and `git add`s its **code** in its own worktree index (the two staging seams — [storage.md](storage.md#cli-and-git)), acks `status + task_id`, and writes any detail to CLI state; the main agent **re-derives from the CLI, never trusts the message**. A sub-agent never commits (see the join, below).
- **Sub-task id = the fanned item's id** — the list-source resolves to a collection of managed instances, each carrying a frozen minted id, and that id becomes the sub-task id, so the by-task-id merge is meaningful and deterministic.
- **`join` is a barrier:** all sub-tasks complete, the CLI merges working areas **by task-id order** (never completion order), and the main workflow re-composes its post-join steps against the merged state. Synchronization is through CLI state, not messages.
- **The parent task's `finalize` is the commit boundary** — sub-agents author their own commit doc, `git add` their code in their own worktree, and may `validate` their own area for early feedback, but **never commit**, so there are no races. The parent's `finalize` validates the merged effective state and commits per the **`finalize.fan-out.squash`** cascade knob (M8): `true` (default) → **one aggregate commit** whose message is CLI-synthesized from the id-ordered sub-task list; `false` → **one commit per sub-task in task-id order** rendering each sub-task's own authored commit doc, plus the parent's. Either way "one task → one logical commit" is preserved and the committed bytes are deterministic — byte-identical across feed orders ([finalize.md](finalize.md#fan-out-finalize)).
- **Slug collisions resolve in the merge pass.** Two sub-agents minting the same slug → the join suffixes the loser (task-id order) and **rewrites that area's *local* self-references** to match (the CLI owns wiring — the agent placed nothing). Workflow-provisioned ids are sub-task-derived and can't collide at all.
- **Cross-area refs are rejected at join, not silently rewritten.** Filesystem isolation prevents a sub-agent from *writing into* a sibling's area, but it does not prevent the sub-agent from *typing* a plausible sibling slug — e.g. sub-task B writes `supersedes: adr:cache` guessing sub-task A's ADR id. The join validates each sub-task's outgoing refs against `(committed store ∪ this sub-task's own working area)` only — **same surface as the cross-task forward-ref policy** ([validation.md](validation.md) → Forward-ref resolution) — and rejects cross-area refs as blocking integrity errors. The dangling ref surfaces; the human routes. Coordinating creates that need to reference each other belong in **sequential steps** before the fan-out splits; ID reservation or provisional refs would land as a separate post-MVP feature with its own design, not as a quiet retarget.
- **Never-started sub-tasks surface as a distinct outcome — *deferred past M8* (2026-06-04).** The intended runtime backstop: the join derives every sub-task's state from the CLI ("messages are notifications, CLI state is truth" — [VISION.md](../VISION.md) → Sub-agents), so a sub-task with **no recorded CLI activity** (no reads, no writes, no working-area presence) is treated as **never-started**, distinct from "started and failed" — catching the case where a launch template *passes* install-time validation but still misfires at runtime. **M8 does not build this** ([DECISIONS.md](../DECISIONS.md) 2026-06-04): install-time template schema validation ([assistant-adapter.md](assistant-adapter.md) → Bind the spawn mechanism) is M8's only line of defence against a broken launch; the *runtime* misfire of a schema-valid template is an accepted, unsurfaced gap until a workflow earns the backstop. Until then a never-started sub-area simply contributes nothing to the join (the missing sub-task shows as absent, not as a routed outcome).

Two bounds, both consistent with prior decisions:

- **Fan out over the whole collection** — no "fan out over the subset matching X" (selection is logic); each sub-agent's own workflow + reasoning handles "nothing to do here," like the no-conditionals stance.
- **No nested `fan-out`** — parallelism is capped at one level, mirroring the doc dialect's "no repeatable-in-repeatable." A within-task parallel need is the trigger to revisit; tree-parallelism is deferred.

## The checkpoint step kind (M15)

The third step kind (after `fan-out`/`join`): a **structural human-gate** that halts a workflow's execution to surface a decision to the human, then resumes from where the prior steps' effects left off. It is the dialect's encoding of the **halt points** the harder harness workflows are built around — planning's *Settle*, the [increment workflow](../implementation/increment-workflow.md)'s three halts (a new fork at Plan, a blocked task at Execute, still-blocking after the fix-round cap), and the **fix-loop's halt-pending-fix** (validate → if it blocks, stop and fix, then re-run). It is the **expected M12 dialect-extension outcome**, triggers #2 + the structural half of #4 ([self-hosting.md](self-hosting.md#the-named-dialect-extension-trigger-the-expected-m12-outcome)), built as **one** primitive because every one of those halts is the same shape — *stop and hand the decision to the human* — differing only in *which* halt it is.

**On-disk format — a front-matter marker, like `fan-out`/`join`:**

```markdown
# steps/plan-gate.md   — a checkpoint step: the marker lives in front-matter
---
checkpoint:
  reason: new-fork-at-plan
---
If planning surfaced a genuinely new fork not covered by the settled gates
and not resolvable from the locked docs, stop and surface it to the human
before cutting tasks.
```

- **`checkpoint:` carries one field, `reason`** — a stable slug naming *which* halt point this is (`new-fork-at-plan`, `blocked-task`, `fix-rounds-exhausted`, …). The slug is a label for the human and the orchestrator, not a condition the CLI evaluates. The **id is the filename** (a frozen slug), so a checkpoint step is reusable across workflows like any other step.
- **The body is the prose** — *what to evaluate and when to halt*. The condition ("if a new fork surfaced", "if blocking findings remain after 3 rounds") is **agent judgment, stated as prose** — never a dialect conditional. This is the deliberate line: the dialect supplies the *halt point* (structure); the agent supplies the *halt decision* (judgment). The same sort the rest of the methodology already draws ([self-hosting.md](self-hosting.md) → the three-way determinism cut).

**How it composes.** A checkpoint step **always composes** — it emits its `reason` as a `Checkpoint: <reason>` directive line (the [Checkpoint emit class](#emitted-format)) followed by its body prose. Composition is unconditional and task-independent: every workflow of a type emits the same checkpoint steps in the same order, so the structural-substitution invariant holds (see [Composition is substitution, not control flow](#composition-is-substitution-not-control-flow) — a checkpoint *halts*, it does not *branch*). The agent/orchestrator **honors** the directive by stopping; what the human then decides is outside jigc. There is **no pairing rule** (unlike `fan-out`↔`join`) — a checkpoint is standalone, valid anywhere in a workflow body.

**Resumption is the existing stateless restart.** After a halt is cleared, the workflow resumes via `jigc start --task <id>`, which **re-composes from scratch** and re-derives the view from the task's accumulated effects on disk — no progress cursor, no checkpoint state, no runtime (see [Open questions](#open-questions)). The fix-loop rides this for free: re-running after a fix re-derives the now-passing state. **The loop's *bound* (≤N rounds) and its *re-run* stay orchestration-level** — they live in the build harness that drives the workflow, never in the dialect or the binary. The dialect contributes only the *halt signal*; "repeat until clean, ≤3 times" is not a dialect construct (that would be a loop = control flow = the barred runtime).

**Engine reality (not a free variant).** Admitting a third step kind re-opens the closed `(fan-out, join)` parse: `parse_step_kind` becomes a three-marker mutual-exclusivity check (a step is exactly one kind), `StepKind` gains a `Checkpoint { reason }` variant, the emit dispatch gains the `Checkpoint:` arm, and `checkpoint-marker-not-shadowed` is added to the `workflow-refs` conformance pass (at **both** its call sites — the fill-aware one is the live compose path). A `checkpoint:` marker authored *without* the engine support composes as **inert Reason prose** (front-matter does not reject unknown keys), so the build's done-criterion must assert composed output **differs** with vs. without the marker — the [M8 face-#5 discipline](../implementation/increment-workflow.md) (a marker that parses but does nothing is a parsed-but-ignored flag).

**The honest encode bound.** jigc composes a **single-agent linear workflow**; the increment-workflow it encodes is a **multi-agent orchestration loop** (one agent per phase, an *independent* read-only validator, the bounded fix-loop). M15 encodes the **single-agent composable spine** — the phases one agent walks, with the checkpoint halts made structural — while the multi-agent orchestration (per-phase-agent independence, the independent validate, the bound + re-run) stays **orchestration-level**, exactly as M12's reduced-linear encode bounded itself and M8 carved out the genuine spawn ([self-hosting.md](self-hosting.md#the-named-dialect-extension-trigger-the-expected-m12-outcome)). This bound is the deliverable's falsifiable edge, recorded so a faithful-looking encode can't quietly over-claim that jigc owns the loop.

## How a workflow composes

**Conceptual shape** (placeholders unresolved). *The actual on-disk workflow file uses `{{include: step:X}}` lines at body top-level per [On-disk definition format](#on-disk-definition-format); the expanded shape below is shown for clarity — each `##` step heading and its content lives in its own step file.*

```text
# workflow: single-task-execution
## locate
Read the spec for this task:
{{ @task.spec#criteria }}
## implement · author the commit prose
Run: {{ cli.set-commit-summary }}
<<author: {{task.commit#summary}}>>
## finalize
Run: {{ cli.finalize }}
```

Composed for task `add-rate-limiter` — structure identical, values resolved (using the four-class [emitted format](#emitted-format) above):

```text
# single-task-execution · task add-rate-limiter
## locate
Read the spec for this task:
> SPEC auth-flow — criteria
> • Rate limit holds at 100/min → maps-to-test: test/rate_limit_spec.rb#burst

## implement · author the commit prose
Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -`
<<author: commit:add-rate-limiter#summary>>

## finalize
Run: `jigc task finalize add-rate-limiter`
```

The **`@`-marked data-value** resolved to a doc-slice and emitted as a `> ` blockquote (Content class); the command-ref resolved through the cascade and emitted as a `` Run: `<cmd>` `` directive (Run class); the **`<<author: …>>` slot-author directive** survived composition with its embedded `{{task.commit#summary}}` placeholder resolved to the doc address — it points the agent at a *document* slot to fill via the write path (Author class). The reasoning-prose lines ("Read the spec for this task:", "author the commit prose") carry no marker — that's the Reason class, the default. The workflow dialect itself stores nothing.

*This illustrative example shows the **spec-driven** shape (general case). The **MVP** `single-task` is spec-less — its `locate` reads `{{task.intent}}` + the codebase, not a SPEC; see [CLAUDE.md](../CLAUDE.md) → MVP scope.*

## Open questions

- **Workflow progress / resumption** — whether "where am I" is purely re-derived from accumulated task effects (idempotent re-compose) or lightly tracked. Leaning re-derived, to match the ephemeral + blackboard model. **Deferred past M7/M8** (2026-06-04, reconfirmed at M8 Settle): a partial fan-out restarts from scratch; no mid-fan-out resume policy until a workflow earns one. *(M15 confirms **re-derived**: the `checkpoint` step kind's halt→re-run rides this stateless restart with no progress state added — `jigc start --task` re-composes from disk effects after a halt is cleared. The structural primitive named as dialect-extension trigger #4 is the checkpoint, built halt-only; the mid-fan-out resume policy stays deferred.)*
- ~~**`milestone-execution` orchestration**~~ — *settled (2026-06-04), reopened + revised (M31):* the partition **is** the milestone's task list (`{{milestone.tasks}}`); **managed docs** recombine by the by-task-id join ([storage.md](storage.md#the-by-task-id-join-m7)) and isolate by directory (`tasks/<sub>/`), while **code** isolates by **worktree** and recombines by deterministic disjoint-apply (the [third combine mode](storage.md#cli-and-git)) — never git-merged. The `milestone-execution` workflow itself lands in **M8**.
- ~~**The spawn-instruction emit class (M8)**~~ — *settled at M8 Settle (2026-06-04):* the fifth emit class is **`Spawn: `** — a reserved line-start marker mirroring `Run: `, one `` Spawn: `<launch-line>` `` per fanned id, the launch rendered through the adapter spawn template; its shadowing rule is `workflow-refs.spawn-marker-not-shadowed` (intrinsic, scoped to composed step bodies). See [Emitted format](#emitted-format).
