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

The composed output is plain Markdown, but the **agent must distinguish at a glance** between four line-classes — and reliably, because this is the surface where the structural-determinism bet either holds or leaks. Four classes, four conventions:

| class | how it's written in the emitted text | source |
|---|---|---|
| **Run** — execute exactly | `` Run: `<cmd>` `` at line-start, command in backticks | resolved `{{cli.…}}` command-ref |
| **Content** — read this material | a Markdown blockquote (`> `) | resolved `{{@…}}` data-value |
| **Author** — write into this slot | `<<author: <address>>` on its own line | the only surviving slot-author directive |
| **Reason** — think about this | plain prose, no marker | static `instruction` text |

The two "machine" classes (**Run**, **Author**) carry markers; the two "human-ish" classes (**Content**, **Reason**) lean on existing Markdown semantics or no marker at all. **Reasoning is the default** — anything that isn't one of the other three is reasoning, with no overhead.

The four rules:

1. **`Run: ` is reserved.** A line whose left margin starts with `Run: ` is a directive the composer emits, never authored by step prose. The command always follows in backticks: `` Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -` ``. Backticks make the command both visually distinct *and* machine-extractable by a strict line-pattern (`` ^Run: `(.+)`$ ``).
2. **`> ` blockquote is content** — emitted around the resolution of a `{{@…}}` data-value. A multi-line doc-slice is a multi-line blockquote. Renders cleanly in any Markdown viewer; the agent reads it as "this is material I was given," not "this is something to run."
3. **`<<author: <address>>` is the only thing the agent originates.** Its rules are settled in [Leaves](#leaves-instructions-and-placeholders): exactly one address parameter; the address is the doc slot the agent fills via the write path; the `<<…>>` wrapper survives composition unchanged.
4. **Everything else is reasoning prose** — no marker, no special handling, no overhead. The agent treats it as instruction it should think with.

**Compose-time conformance.** Because **Run** is the load-bearing marker, instruction prose in a step definition **must not** start a line with `Run: ` — that prefix is the composer's. The `workflow-refs` probe ([validation.md](validation.md)) checks this at compose-time as part of its existing scope: the same probe that validates every placeholder/include/command-ref resolves also validates that `Run: ` at line-start in a step body comes only from a resolved `{{cli.…}}`. A definition that shadows the marker is rejected with a precise pointer (step file + line) before the composed output ever reaches the agent. `> ` and `<<author:` need no such check — blockquotes are legitimately part of reasoning prose, and `<<…>>` is grammatically slot-syntax (already off-limits to definition prose by principle #6).

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
- **A workflow** = a file: front-matter for workflow metadata — a one-line **`when`** hint the router uses for selection, a **`creates-task`** boolean (default `true`) declaring whether running this workflow mints a task ([write-commands.md](write-commands.md) → Task origination), an optional **`allows-create: [<doctype-id>, ...]`** list naming the doctypes the agent may `jigc doc create` during the task (default empty — no agent-initiated creates allowed; [write-commands.md](write-commands.md) → Instance provisioning), and an optional **`reads: [{role: <role>, type: <doctype-id>}, ...]`** list declaring **context roles bound from existing committed docs** (default empty); the **body is the ordered `{{include}}`s** — which *is* the composition and the ordering (principle #2: the workflow file is the ordered list; steps are the ID'd, reusable units it references).

  `reads` is the dual of `allows-create`'s object form: `allows-create … as:` declares a role bound to a doc the task *creates*; `reads` declares a role bound to a doc a *prior* task committed, via `jigc task bind <role> <addr>` ([write-commands.md](write-commands.md) → Binding a context role). Both make `task.<role>` a **declared** root for `workflow-refs` — a `task.<role>` whose role appears in neither is a conformance error; a declared-but-unbound role resolves to empty text. A `reads` role is **always optional** in the M3 surface (unbound = empty, no finalize gate) — there is no required-read-role cardinality yet, so `reads` carries no `card`; the rule for a required read-role (block finalize when unbound) is deferred until a workflow earns one. The spec-driven `implement-from-spec` workflow declares `reads: [{role: spec, type: spec}]` so its `locate-from-spec` step's `{{@task.spec#criteria}}` validates and resolves over the bound spec.

```markdown
# steps/locate.md   — a plain step: id from filename, body is the prompt
Read the spec for this task:
{{ task.spec#criteria }}
```

```markdown
# steps/superseded-context.md   — a plain step: surfaces the decision this task supersedes, if any
If your decision supersedes an earlier one, here is that decision for reference —
make your consequences explain what changes:
{{ @task.decision.supersedes#decision }}
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

**Workflow body is include-only at top level.** A workflow file's body may contain only `{{include: step:X}}` lines, blank lines, and HTML comments (`<!-- ... -->`, for human notes about why a step is included). Prose, ATX headings, other placeholders, or any other content at body top-level is a **conformance error** caught at compose-time by `workflow-refs.body-include-only` ([validation.md](validation.md) → Severity inventory). Step files carry the prose; the workflow file is purely composition. The physical-order serialization of the include list satisfies [VISION.md](../VISION.md) principle #2: "never positions" bans IDs that *encode* order, not physical-order serialization of a list whose items carry their own non-positional IDs — a line move is a reorder, nothing renumbers, cross-refs to `workflow:single-task#locate` keep resolving regardless of include position.

Because a step is a file with a stable id, **steps resolve through the cascade** ([overrides.md](overrides.md) → Resolution algorithm): a step-id resolves to the highest-precedence layer's file (atomic file-level shadowing in phase 2), so overriding a pack step is just "the project ships its own `validate.md`, and project wins" — no special override machinery. Step files are reusable across workflows; that's what `include` references. Structural deltas (`insert-step`, `replace-step`, `remove-step`) operate on the workflow's **include list** in phase 4, *before* include expansion in phase 7 — same `(definition + cascade)` in → same composition.

(The doc-type *schema* definition format is config-structured — structured YAML, not this pattern — and is specified in [document-type-schema.md](document-type-schema.md).)

## Workflow selection — the router default

Which workflow a task runs is **not** the composer's job (composition is substitution, not control flow) and **not** the CLI's (it can't infer, model-free). It is resolved at the front door ([write-commands.md](write-commands.md) → Task origination):

- `jigc start --workflow <X>` composes `X` directly.
- `jigc start` with no workflow composes the **router** — the cascade-default workflow whose job is *selection*. The router is an ordinary composed workflow: its body interpolates the engine-native **`catalog`** root — the live list of **selectable work-workflows** (those declaring `creates-task: true`) with each one's **`when`** hint, so adding a work-workflow surfaces it automatically and the router never lists itself or any other `creates-task: false` workflow — and instructs the agent to re-run `jigc start --workflow <chosen> "<intent>"` (an **agent-substitution** pattern, not a resolved value — the agent re-supplies its own intent). The agent picks; the CLI never does. *(A workflow-level **recommended** default is post-M2: with two `when`-hinted options the agent chooses unaided, and surfacing a recommendation would need its own cascade knob — `default-workflow` now names the router — which is generality for a single use until earned.)*

So selection reuses the workflow machinery (no special selection logic), stays model-free, is cascade-overridable (a project can rewrite its routing advice, or set the default to a specific work-workflow — as the **MVP** does, defaulting to `single-task` until ≥2 work-workflows exist, see [CLAUDE.md](../CLAUDE.md) → MVP scope), and **never forces** — the worst case is the agent gets routing help. A workflow earns its place in the router by declaring its one-line `when` hint, so adding a workflow surfaces it automatically.

## `fan-out` / `join`

The single bounded concurrency primitive, assembled from the locked concurrency invariant, the blackboard model, and the storage decisions.

> **Built across two milestones (split 2026-06-04, [DECISIONS.md](../DECISIONS.md)).** The **data plane is M7**: the `milestone` work-unit (engine state) + its task-list verbs, the isolated sub-task area dirs + the **join-time** isolation check, and the **by-task-id join merge** itself ([storage.md](storage.md#the-by-task-id-join-m7)) — proven deterministic by permutation tests over real multi-area fixtures, without any real spawn. The **control plane is M8**: the `fan-out`/`join` **step kinds** + the spawn-instruction emit class (a new emitted-format directive), the **`{{milestone.tasks}}` data-value root** the fan-out step consumes, the **adapter spawn binding** + the `jigc workflow W --task <id>` re-entry + the **write-time `--task`-scoped barrier**, and the **`milestone-execution` workflow** that wires it end-to-end. The bullets below describe the whole primitive; each clause is tagged M7 (data) or M8 (control) only where it isn't obvious.

- A **`fan-out` step** declares a **list-source** — a data-value resolving to a *collection* (e.g. `{{milestone.tasks}}` from a many-relation, or `{{spec#criteria}}` from a repeatable section's items) — and a **referenced sub-workflow** each spawn runs.
- The **CLI resolves the list and emits dispatch instructions** ("spawn these task-ids running `W`"); the **assistant adapter launches** them. The locked seam holds — *CLI owns the payload, the assistant owns the launch* — and the CLI never spawns (it makes no agent calls).
- Each **sub-agent re-enters the composer** (`jigc workflow W --task <sub>`) and gets the *same deterministic composed workflow* it would get as a main agent — **referenced, not inline** (inline would reintroduce a lossy paraphrase). It writes to its isolated `.jigc/tasks/<sub>/`, acks `status + task_id`, and writes any detail to CLI state; the main agent **re-derives from the CLI, never trusts the message**. A sub-agent never commits (see the join, below).
- **Sub-task id = the fanned item's id** — the list-source resolves to a collection of managed instances, each carrying a frozen minted id, and that id becomes the sub-task id, so the by-task-id merge is meaningful and deterministic.
- **`join` is a barrier:** all sub-tasks complete, the CLI merges working areas **by task-id order** (never completion order), and the main workflow re-composes its post-join steps against the merged state. Synchronization is through CLI state, not messages.
- **The parent task's `finalize` is the commit boundary** — sub-agents may `validate` their own area for early feedback but **never run git**, so there are no races. The parent's `finalize` validates the merged effective state and emits **one commit per sub-task in task-id order** (squash is a cascade knob), plus the parent's own commit. "One task → one logical commit" is preserved per sub-task, deterministically ordered.
- **Slug collisions resolve in the merge pass.** Two sub-agents minting the same slug → the join suffixes the loser (task-id order) and **rewrites that area's *local* self-references** to match (the CLI owns wiring — the agent placed nothing). Workflow-provisioned ids are sub-task-derived and can't collide at all.
- **Cross-area refs are rejected at join, not silently rewritten.** Filesystem isolation prevents a sub-agent from *writing into* a sibling's area, but it does not prevent the sub-agent from *typing* a plausible sibling slug — e.g. sub-task B writes `supersedes: adr:cache` guessing sub-task A's ADR id. The join validates each sub-task's outgoing refs against `(committed store ∪ this sub-task's own working area)` only — **same surface as the cross-task forward-ref policy** ([validation.md](validation.md) → Forward-ref resolution) — and rejects cross-area refs as blocking integrity errors. The dangling ref surfaces; the human routes. Coordinating creates that need to reference each other belong in **sequential steps** before the fan-out splits; ID reservation or provisional refs would land as a separate post-MVP feature with its own design, not as a quiet retarget.
- **Never-started sub-tasks surface as a distinct outcome.** The join derives every sub-task's state from the CLI ("messages are notifications, CLI state is truth" — [VISION.md](../VISION.md) → Sub-agents). A sub-task with **no recorded CLI activity** — no reads, no writes, no working-area presence — is treated as **never-started**, distinct from "started and failed." This is the runtime backstop for adapter-template corruption: if the assistant's launch template is stale or broken and the sub-agent never invokes `jigc workflow --task <id>`, the join surfaces "never-started" for routing rather than waiting forever or producing a quiet zero-output success ([assistant-adapter.md](assistant-adapter.md) → Bind the spawn mechanism).

Two bounds, both consistent with prior decisions:

- **Fan out over the whole collection** — no "fan out over the subset matching X" (selection is logic); each sub-agent's own workflow + reasoning handles "nothing to do here," like the no-conditionals stance.
- **No nested `fan-out`** — parallelism is capped at one level, mirroring the doc dialect's "no repeatable-in-repeatable." A within-task parallel need is the trigger to revisit; tree-parallelism is deferred.

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

- **Workflow progress / resumption** — whether "where am I" is purely re-derived from accumulated task effects (idempotent re-compose) or lightly tracked. Leaning re-derived, to match the ephemeral + blackboard model. **Deferred past M7/M8** (2026-06-04): a partial fan-out restarts from scratch; no mid-fan-out resume policy until a workflow earns one.
- ~~**`milestone-execution` orchestration**~~ — *settled (2026-06-04):* the partition **is** the milestone's task list (`{{milestone.tasks}}`), and recombination **is** the by-task-id join ([storage.md](storage.md#the-by-task-id-join-m7)). Isolation is by directory (`tasks/<sub>/`), **not** git worktrees, for managed docs. The `milestone-execution` workflow itself lands in **M8**.
- **The spawn-instruction emit class (M8)** — the `fan-out` step emits a *fifth* directive shape ("spawn these task-ids running `W`") beyond the four-class [emitted format](#emitted-format); its exact marker convention + `workflow-refs` shadowing rule are settled when M8 builds the step kind.
