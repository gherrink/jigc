# Worked examples

End-to-end walkthroughs of four MVP-critical flows, with cross-refs to the canonical spec for every surface they touch. This doc is **last in the reading order**; each example references prior part-docs without re-stating their content. Notation is **illustrative** — the docs cited are the source of truth for shape, error format, and edge cases.

The flows:

1. [Spec-less single-task with optional ADR create](#1-spec-less-single-task-with-optional-adr-create) — the MVP write loop
2. [OOB edit reconciliation](#2-oob-edit-reconciliation) — absorb, conformance-block, conflict-block
3. [Override application at compose time](#3-override-application-at-compose-time) — a project-level delta shifting the composed output
4. [Finalize-to-git](#4-finalize-to-git) — the seven phases producing one commit
5. [Superseding decision](#5-superseding-decision--context-slice--edge-integrity) — context-slice over a persisted ADR + forward-ref integrity
6. [Spec-driven planning](#6-spec-driven-planning--the-two-task-arc) — the M3 arc: a `plan` task authors a spec, a later task binds and implements it

## 1. Spec-less single-task with optional ADR create

The MVP flow: the agent reads its intent + the codebase, implements, optionally writes an ADR documenting a decision, finalizes.

```text
# orientation — read-only
$ jigc start
> ... (clean project; see bootstrap.md → Orientation output examples 2) ...

# start the task; mints commit:add-rate-limiter, opens .jigc/tasks/add-rate-limiter/
$ jigc start "add per-client rate limit at the gateway"
> task: add-rate-limiter · workflow: single-task · base: a3f9c2

# the composed workflow emits its steps using the four-class emitted format
## locate
Reason about the change. The intent is:
> add per-client rate limit at the gateway

The relevant code paths likely involve the gateway middleware. Inspect the
codebase to confirm scope before implementing.

## implement
Implement the change directly in the working tree. When done, stage the commit prose:

Run: `jigc doc set-field commit:add-rate-limiter#type --value feat`
Run: `jigc doc set-slot commit:add-rate-limiter#summary --from-file -`
<<author: commit:add-rate-limiter#summary>>

If a decision is warranted (e.g., choosing a rate-limit algorithm), create an ADR:

Run: `jigc doc create adr --title <TITLE>`
Run: `jigc doc set-field adr:<id>#status --value accepted`
Run: `jigc doc set-slot adr:<id>#context --from-file -`
<<author: adr:<id>#context>>
   (... similar for #decision, #consequences)

## finalize
Run: `jigc task finalize add-rate-limiter`
```

What runs where:

- `jigc start` orients ([bootstrap.md](bootstrap.md) → State-aware front door).
- `jigc start "<intent>"` composes `single-task` via the cascade default ([write-commands.md](write-commands.md) → Task origination); `creates-task: true` mints.
- The composed workflow uses the [emitted format](workflow-dialect.md#emitted-format) — `Run:`, `<<author:>>`, blockquoted Content, plain Reason prose.
- `jigc doc create adr` is permitted because `single-task` declares `allows-create: [adr]` ([write-commands.md](write-commands.md) → The create-gate).
- `<TITLE>` is the agent-substitution marker from the [command catalog](command-catalog.md); the agent fills it before running the command.
- `finalize` runs the [seven phases](finalize.md) — see flow 4.
- `single-task` also includes a `superseded-context` step; in this non-superseding task its `{{@task.decision.supersedes#decision}}` slice resolves to empty text and emits nothing ([workflow-dialect.md](workflow-dialect.md#leaves-instructions-and-placeholders) → empty vs unresolvable). It carries weight only in [flow 5](#5-superseding-decision--context-slice--edge-integrity).

## 2. OOB edit reconciliation

The human edits `decisions/adr-cache-policy.md` directly between two task steps.

### Absorb (the clean path)

```text
# task add-rate-limiter is mid-flight; the agent runs a read
$ jigc doc read adr:cache-policy
> external edit absorbed: adr:cache-policy
>
> (... the doc content as-edited ...)
```

`file-state` probe fired on read; classifier ran ([reconciliation.md](reconciliation.md) → State machine). Pair `(DRIFTED, UNTOUCHED)` → OOB-edit branch. Parse + schema-validate succeeded → **absorb**: re-hash, update edge index for cross-ref changes, surface the message. State returns to `IN_SYNC`.

### Conformance-block

A nonconformant edit — the human dropped a `{#id}` anchor from a repeatable item:

```text
$ jigc doc read adr:cache-policy
> error: nonconformant edit on adr:cache-policy
>   file: decisions/adr-cache-policy.md, line 18
>   expected: {#id} anchor on item heading "### Memory pressure response"
>   resolution: add the anchor back, or revert the edit
```

State stays `DRIFTED`; the task is blocked from progressing on this doc until the human fixes it. The MVP does no auto-repair ([reconciliation.md](reconciliation.md) → Auto-repair scope).

### Conflict-block

The human edited the same doc the task is also writing to:

```text
$ jigc doc set-slot adr:cache-policy#consequences --from-file -
> error: conflict on adr:cache-policy
>   external edit since 2026-05-28T14:03:00Z + task add-rate-limiter has staged changes.
>   resolution:
>     - jigc task discard-write adr:cache-policy   # drop the task's changes
>     - revert the file on disk                    # drop the human's edit
```

File-level block; three-way merge is [deferred](reconciliation.md#mvp-scope-vs-post-mvp) (parallels override-conflict resolution).

## 3. Override application at compose time

The pack-default `single-task` workflow uses `validate` as its third step. The project wants a stricter version that also runs a custom lint probe.

Project's `.jigc/config/manifest.yaml`:

```yaml
deltas:
  - kind: replace-step
    target: workflow:single-task#validate
    with: step:project-validate
```

Project's `.jigc/config/steps/project-validate.md`:

```markdown
Run before finalize: validation + the project's lint probe.

Run: `jigc task validate {{task.id}}`
Run: `jigc task validate {{task.id}} --probe lint`
```

When the agent runs `jigc start "..."`, the composer runs the [9-phase resolution algorithm](overrides.md#resolution-algorithm):

- **Phase 2** (by-id shadowing): `step:validate` resolves to pack-default; `step:project-validate` resolves to the project layer.
- **Phase 4** (structural deltas): `replace-step` swaps `validate` for `project-validate` in `single-task`'s include list. The list is now `[locate, implement, project-validate]`.
- **Phase 6** (cycle detection): no cycles.
- **Phase 7** (include expansion): `step:project-validate.md`'s body expands.
- **Phase 8** (placeholders): `{{task.id}}` resolves to `add-rate-limiter`.

The emitted workflow's third step is now the project's validate. Same workflow id, same address, different content — and the agent doesn't need to know it happened.

`jigc start --explain add-rate-limiter` ([workflow-dialect.md](workflow-dialect.md#--explain-output-contract)) shows the resolution tree with `project-validate` named as a project-layer override of pack-default's `validate`:

```text
workflow:single-task    (pack-default · dev/v0.3.0)
  overrides applied: 1 (replace-step at #validate)
  includes:
    step:locate              (pack-default · dev/steps/locate.md)
    step:implement           (pack-default · dev/steps/implement.md)
    step:project-validate    (project · .jigc/config/steps/project-validate.md
                              ← replaces step:validate at position 3)
      {{cli.validate-task}}  → jigc task validate add-rate-limiter
      {{cli.lint-task}}      → jigc task validate add-rate-limiter --probe lint
  findings (workflow-refs): 0
```

## 4. Finalize-to-git

The seven phases of `finalize` ([finalize.md](finalize.md)), producing exactly one commit:

```text
$ jigc task finalize add-rate-limiter

[1 preflight]  task exists; base a3f9c2 = HEAD ✓; git committable ✓
[2 validate]   no blocking findings (1 advisory: subject 71 chars)
[3 render]     commit message rendered: feat + summary + body
[4 promote]    adr:rate-limit-at-the-gateway → decisions/adr-rate-limit-at-the-gateway.md
[5 stage]      git add decisions/adr-rate-limit-at-the-gateway.md
               git add (code changes between a3f9c2 and HEAD-tree)
[6 commit]     git commit -F <message-file>
               → b8e2d4 "feat: add per-client rate limit at the gateway"
[7 post]       edge-index stamp invalidated
               file-state hashes updated (1 doc)
               .jigc/tasks/add-rate-limiter/ removed
```

A pre-commit hook rejection at phase 6 would have rolled back phases 4–5: `git restore --staged --worktree` on the promoted ADR path, the ADR copy deleted from `decisions/`, the working area `.jigc/tasks/add-rate-limiter/` intact ([finalize.md](finalize.md) → Rollback discipline). The agent sees the hook's stderr verbatim and re-runs `finalize` after fixing the issue. **Never `--no-verify`** — hooks are user policy.

The [dirty-tree policy](finalize.md#dirty-tree-policy) applies: the code changes the agent made directly in the working tree (between base `a3f9c2` and HEAD-tree) are committed alongside the ADR — one task → one commit. The base-pin check in phase 1 would have caught a "you switched branches mid-task" accident before any staging happened.

After phase 7, `jigc start` returns to the "clean project" orientation ([bootstrap.md](bootstrap.md) → Orientation output examples 2) — the task is gone, the recent-finalizations list ticks up by one, the cascade is unchanged, the next `jigc start "<intent>"` mints the next task.

## 5. Superseding decision — context-slice + edge integrity

The flow that proves three differentiators at once: a later task records a decision that **supersedes** a committed ADR, so the loop composes a slice of that persisted ADR, walks the edge index at finalize, and round-trips the committed file on read. `supersedes` is the only logic-free handle from live task state to a committed doc in a spec-less MVP ([CLAUDE.md](../CLAUDE.md) → MVP scope), which is why this single path is the mandated differentiator proof.

### Setup — task 1 commits the ADR that will be superseded

```text
$ jigc start "cache sessions in a single in-memory node"
> task: cache-sessions · workflow: single-task · base: a3f9c2

$ jigc doc create adr --title "Single-node session cache"   # → adr:single-node-cache (allows-create binds it to task.decision)
$ jigc doc set-field adr:single-node-cache#status   --value accepted
$ jigc doc set-slot  adr:single-node-cache#decision --from-file -
   (... prose ...)
$ jigc task finalize cache-sessions
> b8e2d4 "feat: cache sessions in a single in-memory node"
```

`decisions/single-node-cache.md` is now committed; its `file-state` baseline is recorded (finalize phase 7, [finalize.md](finalize.md)).

### Task 2 — supersede it, and the workflow surfaces the prior decision

```text
$ jigc start "move the session cache to a shared redis cluster"
> task: shared-redis-session-cache · workflow: single-task · base: b8e2d4

# the agent decides this replaces the earlier decision and creates the new ADR
$ jigc doc create adr --title "Shared Redis session cache"      # → adr:shared-redis-session-cache, bound to task.decision
$ jigc doc set-field adr:shared-redis-session-cache#status            --value accepted
$ jigc doc set-field adr:shared-redis-session-cache#status/supersedes --value adr:single-node-cache
```

The agent **re-composes** to pick up context now that the edge exists ([workflow-dialect.md](workflow-dialect.md) → Open questions: re-derived progress):

```text
$ jigc start --task shared-redis-session-cache
...
## superseded-context
If your decision supersedes an earlier one, here is that decision for reference —
make your consequences explain what changes:
> A single in-memory node keeps session lookups sub-millisecond and avoids a
> network hop; acceptable because sessions are cheap to reconstruct on a cold node.

## finalize
Run: `jigc task finalize shared-redis-session-cache`
```

- The `{{@task.decision.supersedes#decision}}` placeholder resolved: `task.decision` → `adr:shared-redis-session-cache` (bound at create), `.supersedes` → `adr:single-node-cache`, `#decision` → its decision slice, emitted as a `> ` **Content** blockquote ([workflow-dialect.md](workflow-dialect.md#emitted-format)).
- Reaching that slice **re-read the committed `decisions/single-node-cache.md`** and parsed it losslessly to extract the section — the round-trip on a committed, human-editable file ([implementation/parsing.md](../implementation/parsing.md) → Round-trip guarantees).
- Before the edge existed — and in any task that creates no ADR — the same step's placeholder resolved to **empty text**: no finding, no output ([workflow-dialect.md](workflow-dialect.md#leaves-instructions-and-placeholders) → empty vs unresolvable).

The agent writes `#consequences` referencing what changes, then finalizes.

### Finalize — the edge index walk

```text
$ jigc task finalize shared-redis-session-cache
[2 validate]   forward-ref: adr:shared-redis-session-cache#status/supersedes → adr:single-node-cache ✓ (committed store)
...
> c1a9f7 "feat: move the session cache to a shared redis cluster"
```

Forward-ref resolution walks the overlaid edge index and finds the target in the committed store — the **two reachable surfaces** are committed store + this task's working area ([validation.md](validation.md) → Forward-ref resolution). The superseded ADR's read view now derives `superseded-by: adr:shared-redis-session-cache` (the inverse, never stored — [document-type-schema.md](document-type-schema.md) → Bidirectional).

### The dangling variant — finalize blocks

Had the agent pointed `supersedes` at an ADR that exists in neither surface:

```text
$ jigc doc set-field adr:shared-redis-session-cache#status/supersedes --value adr:typo-nonexistent
$ jigc task finalize shared-redis-session-cache
> error: forward-ref integrity — adr:shared-redis-session-cache#status/supersedes
>   target adr:typo-nonexistent resolves in neither the committed store nor this task's working area
>   resolution:
>     - fix the reference to an existing ADR
>     - create the target in this task
>     - drop the supersedes field
```

Blocking integrity error at finalize ([validation.md](validation.md) → Forward-ref resolution); the task fixes it and re-runs. Cross-task forward-refs (the target is a *different* task's planned ADR) are not supported in MVP — same policy, same three routing options (Pass 3 #6, [DECISIONS.md](../DECISIONS.md)).

## 6. Spec-driven planning — the two-task arc

The M3 flow that proves the doc-creation differentiator *beyond* `adr` and the **intent → spec → implementation** arc. Two tasks: a `plan` task authors and commits a `spec`; a later `implement-from-spec` task **binds** that committed spec and implements against it, reading `{{@task.spec#criteria}}` over the persisted file and recording `commit —implements→ spec` at finalize. The new surface over flow 5 is **cross-task binding** — task B reading a doc task A committed, via `jigc task bind` rather than an in-task create ([write-commands.md](write-commands.md) → Binding a context role). Everything else (the persisted-slice read, the resume re-compose, the finalize edge-walk) reuses the flow-5 machinery.

### Task 1 — `plan` authors and commits the spec

```text
$ jigc start "<intent: define what the rate-limiter must do>"
> the router lists single-task · quick-fix · plan · implement-from-spec with their `when` hints
> (the agent picks `plan` — the intent is "define the what", not "implement")

$ jigc start --workflow plan "rate-limit the gateway per client"
> task: spec-rate-limit-gateway · workflow: plan · base: a3f9c2

# plan's create-gate (`allows-create: [{type: spec, as: spec}]`) lets the agent author the spec
$ jigc doc create spec --title "Gateway rate limiting"        # → spec:gateway-rate-limiting, bound to task.spec
$ jigc doc set-slot  spec:gateway-rate-limiting#goal    --from-file -
$ jigc doc add-item  spec:gateway-rate-limiting#criteria --title "per-client limit"
$ jigc doc set-slot  spec:gateway-rate-limiting#criteria/per-client-limit#statement --from-file -
   (... more criteria ...)

$ jigc task finalize spec-rate-limit-gateway
[4 promote]    spec:gateway-rate-limiting → specs/gateway-rate-limiting.md
[5 stage]      git add specs/gateway-rate-limiting.md
[6 commit]     → d4f1a0 "docs: spec gateway rate limiting"
```

- `plan` is a `creates-task: true` work-workflow; it provisions a `commit` doc like any task, so the spec-authoring task still produces one git commit. Its commit `type` is `docs` (a spec-only change — no code).
- The promoted spec is a non-empty diff, so the [empty-commit guard](finalize.md) ([finalize.md](finalize.md)) is satisfied even though no code changed.
- `specs/gateway-rate-limiting.md` is now committed; its `file-state` baseline is recorded.

### Task 2 — `implement-from-spec` binds the committed spec and implements it

`implement-from-spec`'s body is `[locate-from-spec, implement, finalize]` — note it includes a **distinct** `step:locate-from-spec`, *not* the spec-less `step:locate` that `single-task`/`quick-fix` share (a step id resolves to one file pack-wide, so the spec-driven locate must be its own step).

```text
$ jigc start --workflow implement-from-spec "implement gateway rate limiting"
> task: implement-gateway-rate-limiting · workflow: implement-from-spec · base: d4f1a0

## locate-from-spec
Pick the spec this work implements from the committed specs, bind it, then re-run to
read its criteria:
> committed specs:
> • spec:gateway-rate-limiting — "Gateway rate limiting"
> • spec:auth-token-rotation — "Auth token rotation"
Run: `jigc task bind spec <SPEC_ID>`         # ← agent fills <SPEC_ID> = spec:gateway-rate-limiting
Run: `jigc start --task implement-gateway-rate-limiting`   # re-compose to pick up the bound slice
{{ @task.spec#criteria }}                     # empty on this first compose — nothing bound yet
```

The bindable-spec menu is `{{store.specs}}` — the `store` root enumerating the committed `spec` instances as a collection ([workflow-dialect.md](workflow-dialect.md) → data-value roots), emitted as a readable Content list. The agent picks an id, binds, then runs the emitted re-compose directive — the same deferred-bind-then-resume path flow 5 uses, except here the step **emits** the `Run: jigc start --task <id>` so an agent following only the machine markers can't bind-then-forget-to-reread:

```text
$ jigc task bind spec spec:gateway-rate-limiting
> bound: task.spec → spec:gateway-rate-limiting

$ jigc start --task implement-gateway-rate-limiting
## locate-from-spec
Pick the spec this work implements from the committed specs, bind it, then re-run to
read its criteria:
> committed specs: ...
> SPEC gateway-rate-limiting — criteria
> • per-client limit — holds at 100 req/min per client key, burst 20
> • ... (the criteria slice, re-read from the committed specs/ file)

## implement
Implement the change directly in the working tree. When done, stage the commit prose
and record which spec it implements:
Run: `jigc doc set-field commit:implement-gateway-rate-limiting#implements --value spec:gateway-rate-limiting`
Run: `jigc doc set-field commit:implement-gateway-rate-limiting#type --value feat`
Run: `jigc doc set-slot  commit:implement-gateway-rate-limiting#summary --from-file -`
<<author: commit:implement-gateway-rate-limiting#summary>>

## finalize
Run: `jigc task finalize implement-gateway-rate-limiting`
```

- `jigc task bind spec <addr>` is gated by `implement-from-spec`'s `reads: [{role: spec, type: spec}]` declaration ([write-commands.md](write-commands.md) → Binding a context role); an unknown role, a non-spec target, or a target absent from the committed store all reject.
- `{{@task.spec#criteria}}` resolves over the **committed** `specs/gateway-rate-limiting.md` — the persisted-slice read flow 5 already proves, now reached through a *directly-bound* role rather than a relation hop.
- The `implements` field is a `ref` on the (transient) `commit` doc, `card: "0..1"` — set here, absent on spec-less tasks.
- **If the agent never binds** (the role is optional), `{{@task.spec#criteria}}` stays empty and the task finalizes as a degenerate spec-less implementation — intended graceful degradation for M3, no gate (the "did you mean `single-task`?" advisory is deferred).

### Finalize — the `implements` edge walk

```text
$ jigc task finalize implement-gateway-rate-limiting
[2 validate]   forward-ref: commit:implement-gateway-rate-limiting#implements → spec:gateway-rate-limiting ✓ (committed store)
...
> e7c3b9 "feat: implement gateway rate limiting"
```

Forward-ref resolution walks the overlaid edge index: the `implements` edge originates on the **transient commit doc** in the task's working area (the first edge whose *source* is transient) and its target resolves in the committed store — passing exactly as flow 5's `supersedes` does ([validation.md](validation.md) → Forward-ref resolution; [storage.md](storage.md) → Edge-index lifecycle). The edge is validated at finalize but not persisted past it (the commit doc is the git message, never a repo file). Had `implements` pointed at a non-existent spec, finalize would block with the same three routing options as flow 5's dangling variant.
