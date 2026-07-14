# Worked examples

End-to-end walkthroughs of the milestone-critical flows, with cross-refs to the canonical spec for every surface they touch. This doc is **last in the reading order**; each example references prior part-docs without re-stating their content. Notation is **illustrative** — the docs cited are the source of truth for shape, error format, and edge cases.

The flows:

1. [Spec-less single-task with optional ADR create](#1-spec-less-single-task-with-optional-adr-create) — the MVP write loop
2. [OOB edit reconciliation](#2-oob-edit-reconciliation) — absorb, conformance-block, conflict-block
3. [Override application at compose time](#3-override-application-at-compose-time) — a project-level delta shifting the composed output
4. [Finalize-to-git](#4-finalize-to-git) — the seven phases producing one commit
5. [Superseding decision](#5-superseding-decision--context-slice--edge-integrity) — context-slice over a persisted ADR + forward-ref integrity
6. [Spec-driven planning](#6-spec-driven-planning--the-two-task-arc) — the M3 arc: a `plan` task authors a spec, a later task binds and implements it
7. [Upgrade reconciliation](#7-upgrade-reconciliation--clean--conflict--orphaned) — the M5 arc: `jigc upgrade` re-classifies recorded deltas against a new pack
8. [Severity tuning & demotion-lock](#8-severity-tuning--demotion-lock) — the M6 arc: a project tunes a check's severity through the cascade; an intrinsic demotion is floor-rejected
9. [Multi-pack composition](#17-multi-pack-composition--dev--methodology-co-composed-m14) — the M14 arc: dev + methodology co-composed, collisions resolved by precedence, includes pack-local *(flow 17)*
10. [Planning encode](#19-planning-encode--jigc-composes-its-own-milestone-planning-spine-and-maintains-its-running-docs-m16) — the M16 planning half: jigc composes its own milestone-planning spine (the Settle checkpoint) and authors-and-maintains its three running working-docs across two milestone runs *(flow 19)*
11. [Completion encode](#20-completion-encode--jigc-composes-its-own-milestone-completion-spine-and-the-5-owner-artifact-gate-m16) — the M16 completion half: jigc composes its own milestone-completion spine (the audit/triage/fix/re-verify halts as checkpoints), create-fresh authors the per-milestone completion-record, appends the decisions-log, and `finalize` promotes the owner-artifact in-transaction + fires the #5 presence gate *(flow 20)*
12. [Measured run](#21-measured-run--capture-live-on-a-twin-seeds-planted-mid-run-the-record-authored-through-the-binary-m17) — the M17 capture arc: hooks + pinned binary on a twin, both seeded failures planted mid-run after the first promoting finalize, the facts tallied from the raw log, and `record-dogfood` authoring the per-run `dogfood-record` through the binary *(flow 21)*
13. [Three-arm comparison](#22-three-arm-comparison--the-pre-registered-thesis-protocol-m17-sessions-pending) — the M17 thesis comparison protocol: pre-registration first, three twins from one baseline (jigc · control · static-methodology), one matched intent, a rubric'd judgment naming ≥1 non-seeded observation — authored protocol; the sessions themselves are the P1 phase, not yet run *(flow 22)*
14. [Compose-both-at-setup](#23-compose-both-at-setup--the-new-project-methodology-default-m21) — the M21 new-project default: `jigc setup` writes the marker, the embedded `[dev ▸ methodology]` pair composes **dev-highest**, and roadmap/`planning` work out of the box — the same mechanism as flow 17 with the opposite precedence *(flow 23)*
15. [Changelog authoring](#24-changelog-authoring--cold-create--warm-append-a-multi-level-singleton-doctype-expansion) — the doctype-expansion arc: a new project authors a managed `changelog` singleton with a **nested** repeatable (release → change-group), cold-created then warm-appended a new release byte-stable, exercising the four engine lifts *(flow 24)*
16. [Vision-forming from research + park-idea](#38-vision-forming-from-research--park-idea--the-design-altitude-doctypes-at-the-rc-trial-composition-m37) — the M37 design-altitude arc: two `research` docs ground a `vision` singleton across a re-compose, the multi-valued `grounded-in` anchor + edge-walk findings-echo resolve, the display-title H1 and byte-faithful root `VISION.md` render, a shaped `idea` parks router-selectable, and the dangling / freeze / pre-existing-root arms hold — all under `[dev ▸ methodology]` *(flow 38)*

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

Run: `jigc doc set-field commit:add-rate-limiter#header/type --value feat`
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

The **M4 acceptance flow** — a project-level delta of each kind shifts the composed output, with no override leaking the no-delta byte-stable baseline. This section walks `structural-op` (`replace-step`) in full, then `scalar-set` and `slot-fill` compactly; `tracked-fork` applies exactly as a phase-2 file shadow (its recorded base-hash matters only at the **(M5)** reconciliation). The shapes cited are canonical in [overrides.md](overrides.md); notation is illustrative.

### 3a · `structural-op` — `replace-step`

The pack-default `single-task` include list is `[locate, implement, record-changelog, superseded-context, finalize]`. The project wants its own `implement` step that augments the pack's with a house lint reminder — without forking the pack step (it re-includes it).

The project authors the delta through the verb:

```text
$ jigc config replace-step workflow:single-task#implement ./project-implement.yaml
> replace-step recorded: single-task#implement → step:project-implement   (project layer)
```

That writes the `replace` delta into `.jigc/config/manifest.yaml` and the native step (id = filename basename) at `.jigc/config/steps/project-implement.yaml`:

```yaml
deltas:
  - kind: replace-step
    target: workflow:single-task#implement
    with: step:project-implement      # id from the file basename
```

```markdown
# .jigc/config/steps/project-implement.yaml — re-includes the pack step, adds a house rule
{{ include: step:implement }}

Before you finalize, run the project lint probe and fix any findings.
```

When the agent runs `jigc start "..."`, the composer runs the [9-phase resolution algorithm](overrides.md#resolution-algorithm):

- **Phase 2** (by-id shadowing): `step:implement` resolves to pack-default; `step:project-implement` resolves to the project layer.
- **Phase 4** (structural deltas): `replace-step` swaps `implement` for `project-implement` in the include list, now `[locate, project-implement, superseded-context, finalize]`.
- **Phase 6** (cycle detection): `project-implement` includes `implement` (a different id) — no cycle.
- **Phase 7** (include expansion): `project-implement`'s body expands, pulling in the pack `implement` body followed by the house-rule line.

Same workflow id, same composed shape — the agent never knows an override happened.

`jigc start --explain add-rate-limiter` ([workflow-dialect.md](workflow-dialect.md#--explain-output-contract)) shows the resolution tree with `project-implement` named as a project-layer override:

```text
workflow:single-task    (pack-default · dev/v0.3.0)
  overrides applied: 1 (replace-step at #implement)
  includes:
    step:locate              (pack-default · dev/steps/locate.yaml)
    step:project-implement   (project · .jigc/config/steps/project-implement.yaml
                              ← replaces step:implement at position 2)
      {{include: step:implement}}  → (pack-default · dev/steps/implement.yaml)
    step:record-changelog    (pack-default · dev/steps/record-changelog.yaml)
    step:superseded-context  (pack-default · dev/steps/superseded-context.yaml)
    step:finalize            (pack-default · dev/steps/finalize.yaml)
  findings (workflow-refs): 0
```

### 3b · `scalar-set` — a typed knob

The project pins the cascade default away from the router to `single-task` (a one-workflow team that wants the bare-intent path to mint directly):

```text
$ jigc config set default-workflow single-task
> scalar-set recorded: default-workflow = single-task   (project layer)

$ jigc config set default-workflow typo-workflow
> error: wrong-type value for default-workflow
>   knob default-workflow is enum of [router, single-task, quick-fix, plan, implement-from-spec]
>   resolution: pick a declared workflow id
```

The first `set` records `scalar: { default-workflow: single-task }` in `.jigc/config/manifest.yaml`; the value is adjudicated at write time via `check_value` against the `config/knobs.yaml` declaration ([overrides.md](overrides.md) → Scalar knobs). On the next `jigc start "<intent>"`, phase-3 resolves `default-workflow` to `single-task` (project wins over the pack default `router`), so bare-intent mints a `single-task` directly — the cascade *applying* the delta, not just locating the layer. The undeclared/wrong-type `set` is rejected before it touches the manifest (closed surface).

### 3c · `slot-fill` — fill a `{{fill:}}` extension point

The pack `implement` step ships a `{{fill:}}` extension point the pack leaves empty (the M4 pack deliverable that makes slot-fill demonstrable):

```markdown
# pack steps/implement.yaml (excerpt)
Implement the change directly in the working tree. When done, stage the commit prose:
...
{{fill: extra-guidance}}
```

The project injects a house rule without forking the step (a *lighter* touch than 3a's `replace-step` — no re-include, just fill the anticipated point):

```text
$ jigc config fill step:implement#extra-guidance --from-file - <<'TXT'
Confirm a changelog entry exists for any user-facing change before finalizing.
TXT
> slot-fill recorded: step:implement#extra-guidance   (project layer)
```

This records a `slot-fill` delta + `.jigc/config/fills/extra-guidance.md`. At **phase 5** the resolver replaces `{{fill: extra-guidance}}` with that content (before expansion/placeholder resolution), so `implement`'s emitted body now carries the house rule. An **unfilled** `{{fill:}}` would have emitted the pack default (here, nothing) — never a finding; a `slot-fill` aimed at a `<fill-id>` no body declares is a blocking `workflow-refs` finding ([overrides.md](overrides.md#the-fill-placeholder--slot-fill-targets)). Same workflow, same step id, project-specific content.

## 4. Finalize-to-git

The seven phases of `finalize` ([finalize.md](finalize.md)), producing exactly one commit:

```text
$ jigc task finalize add-rate-limiter

[1 preflight]  task exists; base a3f9c2 = HEAD ✓; git committable ✓
[2 validate]   no blocking findings (1 advisory: subject 71 chars)
[3 render]     commit message rendered: feat + summary + body
[4 promote]    adr:rate-limit-at-the-gateway → decisions/adr-rate-limit-at-the-gateway.md
[5 stage]      git add decisions/adr-rate-limit-at-the-gateway.md   (jigc stages its promoted doc)
               (the agent's code edits were already `git add`ed under the agent-stage contract)
[6 commit]     git commit <the index — staged code + promoted doc> -F <message-file>
               → b8e2d4 "feat: add per-client rate limit at the gateway"
[7 post]       edge-index stamp invalidated
               file-state hashes updated (1 doc)
               .jigc/tasks/add-rate-limiter/ removed
```

A pre-commit hook rejection at phase 6 would have rolled back phases 4–5: `git restore --staged --worktree` on the promoted ADR path, the ADR copy deleted from `decisions/`, the working area `.jigc/tasks/add-rate-limiter/` intact ([finalize.md](finalize.md) → Rollback discipline). The agent sees the hook's stderr verbatim and re-runs `finalize` after fixing the issue. **Never `--no-verify`** — hooks are user policy.

The [dirty-tree policy](finalize.md#dirty-tree-policy) applies: the commit carries the agent's **staged** code edits (the agent `git add`ed them under the agent-stage contract) alongside jigc's promoted ADR — it commits the **git index**, not a working-tree sweep, so any unstaged or untracked WIP is left out and surfaced rather than swept in. One task → one commit. The base-pin check in phase 1 would have caught a "you switched branches mid-task" accident before any staging happened.

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
If your decision supersedes an earlier one, set `supersedes` on the ADR; the
superseded decision then appears below for reference, so your consequences can
explain what changes (nothing appears if it supersedes none).
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
Run: `jigc task bind spec <SPEC_ID> implement-gateway-rate-limiting`   # ← agent fills <SPEC_ID> = spec:gateway-rate-limiting; the task id is CLI-resolved
Run: `jigc start --task implement-gateway-rate-limiting`   # re-compose to pick up the bound slice
{{ @task.spec#criteria }}                     # empty on this first compose — nothing bound yet
```

The bindable-spec menu is `{{store.specs}}` — the `store` root enumerating the committed `spec` instances as a collection ([workflow-dialect.md](workflow-dialect.md) → data-value roots), emitted as a readable Content list. The agent picks an id, binds, then runs the emitted re-compose directive — the same deferred-bind-then-resume path flow 5 uses, except here the step **emits** the `Run: jigc start --task <id>` so an agent following only the machine markers can't bind-then-forget-to-reread:

```text
$ jigc task bind spec spec:gateway-rate-limiting implement-gateway-rate-limiting
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
Run: `jigc doc set-field commit:implement-gateway-rate-limiting#header/implements --value spec:gateway-rate-limiting`
Run: `jigc doc set-field commit:implement-gateway-rate-limiting#header/type --value feat`
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

## 7. Upgrade reconciliation — clean / conflict / orphaned

The M5 flow that proves principle #5's *inherit-upstream* half: a project's recorded overrides ([flow 3](#3-override-application-at-compose-time)) are **re-classified against a new pack** so every divergence surfaces at one known moment. Reuses the recorded-delta substrate (M4) + the `file_state` clean/conflict/block shape; the new surface is the **`override-default` probe + `jigc upgrade`** ([overrides.md](overrides.md) → Upgrade reconciliation). Notation illustrative.

### Setup — a project overrides pack `dev/0.3.0`

```text
$ jigc config set default-workflow single-task          # scalar-set
$ jigc config replace-step workflow:single-task#implement ./project-impl.yaml   # replace (records base-hash of pack's implement)
$ jigc config fork workflow:single-task#finalize         # tracked-fork (records base-hash of pack's finalize)
$ jigc config fork workflow:single-task#locate           # tracked-fork (records base-hash of pack's locate)
$ jigc config fill step:locate#hints --from-file -        # slot-fill on the pack's {{fill: hints}} point
# plus one replace authored UNDER M4 (before M5's substrate fix), so its manifest entry has NO base-hash:
#   - kind: replace-step   target: workflow:single-task#superseded-context   with: step:project-super
```

Each content-bearing delta pins the **blake3 of the pack-default unit it sits on** at `base-version: 0.3.0` — re-read pack-direct, *bypassing the fork's own shadow* ([overrides.md](overrides.md) → Per-kind base-hash basis).

### The upgrade — pack `dev/0.4.0` ships; `implement` + `finalize` rewritten, `locate` untouched, the `{{fill: hints}}` point dropped

In production this is *installing a newer binary*; the e2e drives it through the **`FilesystemPack`** seam — record under `JIGC_PACK_DIR=<v1>`, then:

```text
$ JIGC_PACK_DIR=<v2> jigc upgrade
Pack: dev/0.4.0 · reconciling 6 recorded deltas against the current pack

  ✓ clean          scalar-set default-workflow            (knob still declared)
  ✓ clean          tracked-fork workflow:single-task#locate   (pack `locate` unchanged — current-hash == base-hash; your fork still tracks it)
  ✗ conflict       replace workflow:single-task#implement
                     pack changed `implement` since you overrode it at dev/0.3.0
                     route: review — keep your replacement / re-target / drop, then re-run
  ✗ conflict       tracked-fork workflow:single-task#finalize
                     pack changed `finalize` since you forked it at dev/0.3.0
                     route: review — keep your fork / re-fork / drop, then re-run
  ✗ orphaned       slot-fill step:locate#hints
                     the `{{fill: hints}}` point no longer exists in pack `locate`
                     route: drop this delta, or re-target a current {{fill:}} point
  ✗ needs-rebasing replace workflow:single-task#superseded-context
                     authored before M5 recorded a base-hash — cannot tell if it changed
                     route: re-record via `jigc config replace-step …` to pin a basis

4 blocking findings — resolve and re-run `jigc upgrade`.
```

- **clean** (`default-workflow`; the `locate` fork): existence holds and — for the unchanged fork — `current-hash == recorded base-hash`, so you keep your override *and* inherit every other v0.4.0 improvement free.
- **conflict** (`replace #implement`; the `finalize` fork): `current-hash ≠ recorded base-hash` — the pack unit moved on. The **fork** conflict is the load-bearing case: the probe re-reads the *pack's* `finalize`, **not** the fork's shadow copy, so a genuinely-changed upstream unit fires (a shadow-aware read would compare the fork to itself and falsely say `clean`). It **blocks with a review route**; M5 surfaces the divergence, it does **not** auto-merge (3-way merge deferred — [overrides.md](overrides.md)).
- **orphaned** (`slot-fill #hints`): the target is gone → loud failure with a remove/re-target route.
- **needs-rebasing** (the M4-authored `replace #superseded-context`): no recorded base-hash, so the probe *cannot* tell whether the unit changed — it surfaces loudly rather than silently assuming `clean`. The single most likely classification on a project's *first* upgrade after M5 ships.

### Resolve — re-run the `jigc config` verbs (report-only upgrade writes nothing)

```text
$ jigc config replace-step workflow:single-task#implement ./project-impl.yaml          # conflict → re-pins to dev/0.4.0's implement (you confirmed your replacement still fits)
$ jigc config fork workflow:single-task#finalize                                       # conflict → re-forks against dev/0.4.0's finalize
$ jigc config replace-step workflow:single-task#superseded-context ./project-super.yaml # needs-rebasing → re-record pins a basis
$ # (drop the orphaned slot-fill by hand-editing .jigc/config/manifest.yaml — the sanctioned delta-form edit)

$ JIGC_PACK_DIR=<v2> jigc upgrade
Pack: dev/0.4.0 · reconciling 5 recorded deltas against the current pack
  ✓ clean   (all 5) — no upstream change unaccounted for
```

`jigc upgrade` is **report-and-route only** — it never mutates the manifest; the human re-pins via the `jigc config` verbs (re-recording a delta pins its basis against the now-current pack), and a clean re-run is the verification. **No upstream change silently lost; no override silently broken** — the milestone's headline, proven through the real binary.

*(Spiked during planning: the `override-default` classifier is purely content-stateless — a stale recorded base-hash yields `conflict`, the matching hash `clean`, a missing target `orphaned` — and the in-process `FakePack`/`FilesystemPack` two-version seam makes the genuine `v1 → v2` drivable; see [DECISIONS.md](../DECISIONS.md) 2026-06-03.)*

## 8. Severity tuning & demotion-lock

The M6 flow that proves principle #6's promise *"severity is a cascade setting, never a code change"* end-to-end. The headline is the **`override-default` retrofit**: the M5 probe — which shipped with hardcoded blocking severities on a bespoke path ([flow 7](#7-upgrade-reconciliation--clean--conflict--orphaned)) — is now on the **non-task `Probe` seam** and tunes through the cascade like any other ([validation.md](validation.md) → The non-task `Probe` seam, Severity assignment — the M6 post-pass). The tunable/intrinsic/byte-identical checks are the supporting assertions. Notation illustrative.

### Headline — demote an `override-default` conflict from blocking to warning

A project carries a deliberate `replace #implement` it intends to keep across upgrades; it does not want a pack change to that step to *block* `jigc upgrade`, only to *warn*. It tunes the conflict check through the cascade:

```text
$ jigc config set validation.override-default.target-unchanged.severity warning
recorded scalar-set (project): validation.override-default.target-unchanged.severity = warning

$ JIGC_PACK_DIR=<v2> jigc upgrade            # same v2 as flow 7: implement changed
Pack: dev/0.4.0 · reconciling recorded deltas against the current pack

  ⚠ warning  override-default.target-unchanged  replace workflow:single-task#implement
               pack changed `implement` since you overrode it at dev/0.3.0
               route: review — keep your replacement / re-target / drop

0 blocking findings — upgrade is clean to proceed (1 warning surfaced).
$ echo $?
0
```

The conflict is **surfaced but no longer blocks** (exit 0). The proof the retrofit is real: the demotion flows through the **post-pass over aggregated findings**, the *same* mechanism that tunes `validate_task`'s inline probes — `override-default` is not a special case, it rode the seam in ([validation.md](validation.md) → The non-task `Probe` seam). Pre-M6 this severity was hardcoded `blocking` and no `scalar-set` could touch it.

### Supporting — a tunable check stops blocking; the no-override path is byte-identical

```text
$ jigc config set validation.file-state.hash-matches.severity advisory   # tunable (no floor)
$ jigc task validate <id>        # an OOB-drifted file that would have blocked
  ℹ advisory  file-state.hash-matches  drift on docs/decisions/cache.md  (route: reconcile)
0 blocking findings.
```

With **no** severity deltas recorded, every finding keeps its emitted default — the composed/validated output is **byte-identical** to pre-M6 (the post-pass overrides *only* on an explicit `scalar-set`; the determinism boundary's #1 risk, held — [validation.md](validation.md) → Severity assignment — the M6 post-pass).

### Supporting — an intrinsic demotion is floor-rejected (logged, not applied)

`workflow-refs.placeholder-resolves` is intrinsic — its knob carries `floor: blocking` ([overrides.md](overrides.md) → Locked keys). A project tries to demote it anyway:

```text
$ jigc config set validation.workflow-refs.placeholder-resolves.severity advisory
recorded scalar-set (project): validation.workflow-refs.placeholder-resolves.severity = advisory
# (write-time records the delta; the floor is enforced at resolution, where the whole cascade is known)

$ jigc start --explain
Pack: dev/0.4.0 · Project config: .jigc/config · Branch: main (HEAD a1b2c3d)
workflow:single-task                              (pack-default · dev/v0.4.0)
  overrides applied: 0
  rejected (below floor): 1
    validation.workflow-refs.placeholder-resolves.severity
      attempted: advisory (project) · floor: blocking · NOT applied — check is intrinsic
  …
```

The demoting delta is **soft-rejected**: logged on the resolution tree, *not applied* — `placeholder-resolves` stays `blocking`, so a dangling placeholder still bricks composition as before. Resolution **does not abort** (an *undeclared* key still would — that's a typo, not a locked-key demotion). The determinism boundary cannot be weakened from config; the floor is the lock ([validation.md](validation.md) → The two-tier rule).

*(Spiked during planning: severity is assigned by one engine post-pass over aggregated findings — synthetic categories, inline `workflow-refs`/`schema-conformance` emissions, and the non-task `override-default` all tune through it because each emits a `Finding` carrying `(probe, check)`; the seam is built minimally to fit `override-default` without rewriting the byte-stable `validate_task` path; see [DECISIONS.md](../DECISIONS.md) 2026-06-04.)*

## 9. Milestone execution — the deterministic join (M7)

The M7 flow that proves the locked concurrency invariant: **the join is a pure function of the *set* of sub-task working areas — same set in, byte-identical committed state out, regardless of completion order** ([storage.md](storage.md#the-by-task-id-join-m7); CLAUDE.md → "merges at a join ordered by task ID, not completion order"). This is the **engine-genuine** proof: the sub-task areas are provisioned and populated as fixtures, then fed to the join in deliberately-scrambled orders. The *real agent spawn* (the adapter launching sub-agents through the Task tool) is **M8** ([flow 10](#10-milestone-execution--the-genuine-spawn-m8); the `milestone-execution` workflow end-to-end). Proving determinism by **permutation over a fixed area set** is stronger than a single real spawn — it cannot be faked by a sequential-in-process loop (the hollow-spawn trap), and it directly exercises the property a real launch can only sample once. Notation illustrative.

### Setup — a milestone with overlapping-by-design sub-tasks

```text
$ jigc milestone create "Cache hardening"
minted milestone:cache-hardening (base a1b2c3d)

$ jigc milestone add-task cache-hardening "add an LRU eviction ADR"      # → task:add-an-lru-eviction-adr
$ jigc milestone add-task cache-hardening "add a cache-strategy ADR"     # → task:add-a-cache-strategy-adr
$ jigc milestone add-task cache-hardening "document the cache strategy"  # → task:document-the-cache-strategy
```

The milestone pins **one shared base**; each sub-task gets an isolated `tasks/<sub>/` area (separate dirs). In M7 the areas are populated as **fixtures** and isolation is enforced **at the join** (the write-time `--task`-scoped barrier is M8). The three areas are populated to **deliberately exercise every contention path** the disjoint partition is supposed to make rare: two stage a `created` doc that slugs the same (`adr:cache-strategy`), one of which **references its own slug**; one types a ref into a sibling's area.

### Headline — permutation determinism (the #7-failure-class guard)

The same populated area set is finalized under **≥3 divergent feed orders** — task-id order, **reverse** task-id order, and a **seed-shuffled** order — and the committed bytes (message *and* tree) are asserted identical. (`jigc milestone join` *reports* the merge; milestone `finalize` materializes the merged bodies, synthesizes the message, and commits.)

```text
$ jigc milestone finalize cache-hardening   # internally: join (feed order id) → materialize → commit C
$ jigc milestone finalize cache-hardening   # (feed order reverse)                              → commit C  (identical)
$ jigc milestone finalize cache-hardening   # (feed order shuffled:42)                          → commit C  (identical)

assert byte-identical(C_id, C_reverse, C_shuffled)   # message + tree
```

The **commit message is CLI-synthesized** — a structural projection of the milestone id + its **id-ordered** sub-task list (no authored prose, no commit doc), so it is byte-identical across feed orders just like the tree ([DECISIONS.md](../DECISIONS.md) 2026-06-04 → the inc-4 fork). Reverse order is mandatory: an id-ordered fixture where completion-order trivially equals id-order would pass even a completion-ordered (broken) merge. The merge enumerates sub-areas by **sorted task id** and every accumulator it touches (staged-doc set, edge overlay, finding list) is order-keyed (`BTreeMap`/sorted `Vec`, **never** a `HashMap` whose iteration could leak into output) — see [Validation hardening #7](../implementation/increment-workflow.md) (single-execution determinism trust).

### Supporting — colliding new instances get a task-id-ordered suffix; self-refs rewritten

Two sub-tasks each *create* `adr:cache-strategy`. They are distinct decisions, not a clash — the join disambiguates deterministically:

```text
join: adr:cache-strategy — collision across {task:add-a-cache-strategy-adr, task:document-the-cache-strategy}
      task:add-a-cache-strategy-adr (lower id)  → adr:cache-strategy     (bare slug kept)
      task:document-the-cache-strategy          → adr:cache-strategy-2   (suffixed, task-id order)
      rewrote 1 intra-document self-reference in adr:cache-strategy-2  (…supersedes: adr:cache-strategy-2)
```

The renamed instance's **own** self-reference is rewritten in lockstep (a fixture whose colliding docs referenced only *siblings* would pass a broken self-rewrite — so the fixture references its own slug on purpose).

### Supporting — cross-area ref rejected, not silently resolved

Sub-task B typed `supersedes: adr:lru-eviction` guessing sub-task A's slug. Even though that doc *byte-exists* in A's area on disk at join time, the ref resolves against `committed ∪ B's own area` only:

```text
  ✗ blocking  schema-conformance.cross-area-ref  task:add-a-cache-strategy-adr
                supersedes → adr:lru-eviction  resolves only in a sibling sub-task's area
                route: order the tasks so the target commits first / move the creation here / drop the ref
```

A naïve overlay that unioned every sub-area's `docs/` would have resolved this *clean* — the bug the per-`from` narrowing prevents ([validation.md](validation.md) → Fan-out cross-area refs).

### Supporting — same pre-existing doc edited by two sub-tasks → blocking clash

If two sub-tasks both stage an **`edited-from-base`** write to the *same committed-at-base slug* (a partition violation, not a coincidental new-slug collision), the join blocks rather than blind-merging. The `created`-vs-`edited-from-base` provenance recorded at stage time, plus the milestone's shared base, make this decidable without a filesystem race:

```text
  ✗ blocking  join.same-doc-clash  decisions/eviction-policy.md   (existed at milestone base)
                edited-from-base by {task:tune-eviction-thresholds, task:document-eviction-policy}
                route: the fan-out partition must be disjoint — re-partition or sequence these
```

No section-merge, no last-writer-win (rejected in planning — [DECISIONS.md](../DECISIONS.md) 2026-06-04); overlap of a shared target is an error the human routes. The **mixed case** — one sub-task `created` a slug another `edited-from-base` — is also a blocking clash, never a suffix.

## 10. Milestone execution — the genuine spawn (M8)

Where flow 9 proved the **join** is order-invariant over a fixed *fixture* area set, flow 10 proves the **control plane**: real sub-agents, launched through the assistant's Task tool, populate those areas through the real `--task`-scoped write path, and the M7 join recombines them into one reproducible commit. M8's headline — *parallel sub-agent work stays reproducible under the blackboard model end-to-end* — and the one M7 could not reach (it never spawned). Because the launch is, by the determinism boundary, **not engine-guaranteed** ("the CLI owns the payload; the assistant owns the launch"), the acceptance is deliberately **two halves**, neither of which fakes a spawn.

### The flow

```text
$ jigc milestone create "Cache hardening"            # → milestone:cache-hardening (shared base)
$ jigc milestone add-from-spec cache-hardening spec:cache-hardening   # one sub-task per criterion
$ jigc milestone execute cache-hardening              # compose the fan-out over the milestone
```

The `milestone-execution` workflow's `fan-out` step resolves `{{milestone.tasks}}` and emits **one `Spawn:` directive per sub-task**, each rendered through the adapter's spawn template:

```text
Spawn: `jigc workflow sub-task --task add-lru-eviction-adr`
Spawn: `jigc workflow sub-task --task add-cache-strategy-adr`
Spawn: `jigc workflow sub-task --task document-the-cache-strategy`
```

The adapter launches them as **concurrent Task-tool sub-agents**. Each sub-agent runs `jigc workflow sub-task --task <sub>` — which provisions its write-ready area on first entry — implements, authors its own `commit` doc, creates any ADR through the create-gate, **writes its managed docs only into its own `tasks/<sub>/` area** (the **write-time `--task` barrier** refuses anything else), and `git add`s its code in its **own git worktree** (`.jigc/worktrees/<sub>`). A sub-agent **never commits** — the parent's `finalize` is the only commit boundary, combining the worktree-isolated code and joining the `tasks/<sub>/` docs by task-id ([finalize.md](finalize.md#fan-out-finalize)). It acks `status + task_id`; the orchestrator re-derives all state from the CLI, never the message (the blackboard). Then the barrier-join-finalize sequence from flow 9 runs, gated by `finalize.fan-out.squash` (default `true` → one aggregate commit).

### Half A — the automated determinism + payload gate (a `cargo test`)

Runs in CI against the real binary; the "sub-agents" are the test invoking the CLI N times as separate processes — **honest**, because `jigc workflow --task` is *exactly* what a real sub-agent invokes:

1. **Deterministic emit.** Composing `milestone-execution` over a fixed task list emits the N `Spawn:` directives byte-identically across runs; the rendered launch line is extracted and **executed as a process**, asserting it resolves to the real `jigc workflow … --task` verb (guards the L1 landmine — a template naming a nonexistent command).
2. **Real write→join.** Each `jigc workflow sub-task --task <sub>` invocation provisions + writes a `created` doc (and, in the edit-staging case, an `edited-from-base` doc via copy-on-first-touch) into its sub-area through the real verbs — producing exactly the `docs/<addr>.md` + `provenance.json` the join consumes (no hand-staging, unlike flow 9's fixtures).
3. **Byte-identical finalize.** Feeding those real-binary-produced areas to `jigc milestone finalize` under ≥3 divergent orders yields byte-identical committed bytes — the flow-9 determinism assertion, now over real-write inputs.

This half proves the CLI payload + the write→join path. It **cannot** prove the assistant's launch primitive — by construction its "sub-agents" are CLI calls, so it can never witness a real Task-tool spawn or a blackboard violation.

### Half B — the recorded genuine spawn (a milestone-completion audit artifact, **not** a CI gate)

Half B is the once-per-milestone proof that the real assistant launch reaches the CLI. It is an **orchestrator-driven, recorded artifact**, not an automated gate: by the determinism boundary the launch is assistant-owned, and a headless Workflow/Task subagent *cannot itself spawn the Task tool* — so this half has a named **main-session/orchestrator owner** and runs in the milestone's **completion-audit** phase, never inside an increment's build loop. The generic spawn-class artifact spec — what it must assert, why no subagent can run it, the hollow-spawn trap — is owned by [milestone-completion-workflow.md](../implementation/milestone-completion-workflow.md) → *the spawn-class artifact*; this section is its concrete, runnable instantiation for flow 10.

**Owner & timing.** The orchestrator runs this once, during the milestone's completion audit (step 1's third artifact), **after** Half A is green. The e2e-tester subagent, being headless, does *not* run it — it must flag in its summary that this genuine-spawn artifact is the orchestrator half it could not run.

**The same fixture, byte-for-byte.** Half B drives the **identical fixture milestone** Half A's T3 golden was captured over — the same `milestone create` + `add-from-spec` inputs producing the same sub-task set — so the committed tree is directly comparable. Any divergence in the fixture invalidates the tree-hash match.

**The procedure** (orchestrator, main session, in a throwaway repo):

1. **Stand up the same fixture.** In a fresh throwaway repo, run the Half-A T3 fixture's setup verbatim: `jigc milestone create "<same intent>"` then `jigc milestone add-from-spec <milestone> <same spec>`, yielding the same ≥2 sub-tasks. Confirm the sub-task id set equals Half A's before spawning.
2. **Compose the fan-out and read the emitted directives.** `jigc milestone execute <milestone>` composes `milestone-execution`; its `fan-out` emits one `Spawn: \`jigc workflow sub-task --task <sub>\`` directive per sub-task (the same lines Half A extracts). The orchestrator launches from **these emitted lines**, not hand-written commands — the rendered launch line is the contract.
3. **Genuinely spawn ≥2 concurrent sub-agents.** The orchestrator's **Task tool launches the sub-agents concurrently**, one per emitted directive. Each sub-agent re-enters via `jigc workflow sub-task --task <sub>` (which provisions its write-ready `tasks/<sub>/` area on first entry), implements, authors its own `commit` doc, creates any ADR through the create-gate, **writes its managed docs only through `jigc` into its own `tasks/<sub>/` area** (the write-time `--task` barrier refuses anything else), and `git add`s its code in its **own git worktree**. A sub-agent **never commits** and never reads a sibling's area; it acks `status + task_id` only.
4. **Re-derive, join, finalize.** The orchestrator re-derives all state **from the CLI**, never from the ack messages (the blackboard), then runs the barrier → join → `jigc milestone finalize` (gated by `finalize.fan-out.squash` default `true` → one aggregate commit).
5. **Assert the tree-hash match.** Capture the committed tree-hash (`git rev-parse HEAD^{tree}`) and assert it equals **Half A's T3 golden tree-hash byte-for-byte**. A match proves the real launch reached the same CLI payload + join the sim exercises.
6. **Witness the blackboard invariant from the transcript.** Record the run transcript and confirm each sub-agent reached state **via `jigc` calls only** — no direct read of a sibling's `tasks/<other>/` area, no out-of-band file access — a property no permutation test can see (its "sub-agents" are CLI calls by construction).

**The shippability gate.** The milestone is **not shippable** until this recorded artifact exists *and* its tree-hash matches *and* its transcript witnesses the blackboard invariant. A missing artifact is a **blocking completion finding** — never an implicit pass on Half A alone (the **hollow-spawn trap**: passing off the N-process sim as the genuine-spawn proof). Because a headless subagent cannot produce it, this gate is tracked to its named owner — the orchestrator at milestone completion — and never punted to "a later task."

The split is the point: Half A is the fast, deterministic regression gate; Half B is the once-per-milestone proof that the real launch reaches the CLI. Neither is a sequential-in-process loop masquerading as concurrency.

## 11. New project + idea development (M9)

The M9 greenfield acceptance: `jigc` wires into a **brand-new** repo and the agent develops a raw idea into the project's first managed document — a `prd`. Proves the new-project on-ramp end-to-end over the proven create-gate → code-less-finalize → promote substrate ([project-setup.md](project-setup.md) → Flow 1; the `prd` doctype earns its schema from this, its real driver). The new surface over flow 6 (`plan` authoring a `spec`) is the **idea-development altitude** (a `prd` above specs) and the **command→workflow handoff** (install, then orient-to-the-workflow). Notation illustrative.

### Setup — install into a fresh repo

```text
$ git init my-product && cd my-product
$ jigc setup
  bootstrap reference → CLAUDE.md
  jigc allowlist      → .claude/settings.json
  (SessionStart hook installed · .jigc/config/ initialized)
```

Before `setup`, bare `jigc start` renders the **unset-project** orientation (`This project isn't set up. Run: jigc setup`). After `setup`, the project reads as **clean** ([bootstrap.md](bootstrap.md) → orientation states; the discriminator is `.jigc/config/` presence) and the catalog lists `project-setup` among the workflows.

### The flow — develop the idea, author the prd, finalize

```text
$ jigc start --workflow project-setup "a CLI that compiles context for coding agents"
# composes project-setup (creates-task: true) — mints the task, emits:
#   step:develop-idea  — reason about vision / requirements / context (prose)
#   step:author-prd    — Run: jigc doc create prd --title "..." --task <id>
#                        <<author: prd:<slug>#vision>>  /  #requirements  /  #context
#   step:project-finalize — Run: jigc task finalize <id>

$ jigc doc create prd --title "Context Compiler" --task <id>
  → prd:context-compiler   (staged at .jigc/tasks/<id>/docs/prd:context-compiler.md)
$ jigc doc set-slot prd:context-compiler#vision       --from-file - --task <id>
$ jigc doc set-slot prd:context-compiler#requirements --from-file - --task <id>
$ jigc doc set-slot prd:context-compiler#context      --from-file - --task <id>
$ jigc task finalize <id>
  → docs(prd): add the Context Compiler product brief        # one commit
  → promoted: prds/context-compiler.md
```

### What it asserts (the acceptance bar)

1. **The command→workflow handoff.** `jigc setup` flips orientation unset→clean; `project-setup` is then a composable catalog workflow (it composes cleanly on the already-"clean" project — nothing assumes "set up" is terminal).
2. **`prd` is authored end-to-end as pure pack data.** `create prd` → three `set-slot` writes (fixed prose slots, single-word section ids) → finalize, with **no engine change** (the schema is `schemas/prd.yaml` only). The create-gate admits `prd` (in `allows-create`) and rejects any doctype not gated.
3. **Code-less finalize lands one `docs(prd)` commit** promoting `prds/<slug>.md` (the promoted doc is the non-empty diff; empty-commit guard satisfied) — the flow-6 spec-only finalize shape, now for `prd`.

## 12. Existing project — bounded detect-and-route ingestion (M9)

The M9 brownfield acceptance: `jigc` ingests an **existing** repo whose docs are in inconsistent states, classifying each candidate against the managed schemas and **routing the verdict** — adopting what conforms, routing what doesn't to a human, ignoring the unmanaged. Proves the existing-project on-ramp **without auto-migration** (the research-grade core, deferred — [project-setup.md](project-setup.md) → Flow 2). The new surface is **repo-wide discovery + the N-candidate classifier** over the proven `parse_sections` + `schema_conformance` substrate, and **closing the silent `baseline-adopt` hole** (adopt only what is schema-checked). Notation illustrative — the exact `jigc ingest` verb surface + adopt-confirmation are pinned at the build's acceptance-flow spike against the built command grammar.

### Setup — an existing repo with docs in mixed states

```text
my-legacy-repo/
  CLAUDE.md                       # pre-existing house rules (must survive setup)
  .claude/settings.json           # a pre-existing hook (must survive setup)
  decisions/rate-limit.md         # a CONFORMANT adr at its location → adoptable
  decisions/auth-choice.md        # a NON-CONFORMANT near-miss in decisions/ → needs-reconcile
  docs/old-adr.md                 # a CONFORMANT adr at the WRONG location → needs-reconcile
  docs/notes.md                   # freeform, outside every location dir → unmanaged
$ jigc setup                      # merges into CLAUDE.md / settings.json, never clobbers
```

### The flow — scan, classify, route

*(The `jigc ingest` verb + its output shape are **illustrative and net-new** — pinned at the build's acceptance-flow spike against the built command grammar, the way flow 10 handled unbuilt surfaces. `jigc setup` / `jigc start` are built.)*

```text
$ jigc start --workflow ingest-existing "bring this repo under jigc management"
# composes ingest-existing (creates-task: false) — orient + route; emits:
#   Run: jigc ingest        — scan, classify, report the triage verdicts
#   <review the verdicts; adopt the conformant; route the non-conformant to a human>

$ jigc ingest
  ingest — 4 candidates classified  (sorted — deterministic report order)

  adoptable       decisions/rate-limit.md   → adr   (conformant, at location)
  needs-reconcile decisions/auth-choice.md  → adr   · routed (non-conformant in decisions/)
    > reconciliation · blocking
    > section heading "Why" does not match required section `consequences`
  needs-reconcile docs/old-adr.md           → adr   · routed (conformant, WRONG location)
    > reconciliation · blocking
    > conformant adr outside decisions/ — relocate to adopt (jigc never auto-moves)
  unmanaged       docs/notes.md             → (parses against no schema — left untouched)
```

### What it asserts (the acceptance bar)

1. **Repo-wide discovery + location-aware N-candidate classification.** All four candidates are discovered (beyond the declared `location:` dirs, sorted for deterministic report order) and classified against every persisted schema (`adr`/`spec`/`prd`) via `parse_sections` + `schema_conformance` — a **binary** parses-conformant-or-not verdict with **location as the discriminator**: `adoptable` requires conformant *and* already at the schema's `location:`; a conformant-but-misplaced doc is `needs-reconcile`, not silently adopted in place (which would make it invisible to every later store sweep). No fuzzy mapping.
2. **Adopt is net-new + schema-gated; the `baseline-adopt` hole is closed.** Adopting `rate-limit.md` **parses + conformance-gates + populates the edge index (`index.absorb_doc`) + records the file-state hash** — *not* reconciliation's silent `baseline-adopt` (which schema-checks nothing and indexes nothing). It is **register-only — no file is moved or rewritten**. The non-conformant `auth-choice.md` and the misplaced `old-adr.md` are **routed, not adopted**; `notes.md` is **left untouched** — *nothing is adopted without a schema check at its correct location*.
3. **No auto-migration, no clobber, idempotent setup.** `jigc ingest` **rewrites no prose** (detect-and-route only). The **binary-level irreversibility test**: seed `CLAUDE.md` with prior house rules + `.claude/settings.json` with a pre-existing hook, run `jigc setup` **twice**, and assert (a) the human content is preserved **verbatim** (structure-aware merge, never clobber), and (b) the second run's tree hash equals the first's (idempotent no-op). The `needs-reconcile` rows route to a human exactly as an OOB conflict does ([reconciliation.md](reconciliation.md)).

## 13. doc↔code validation — the integration advantage (M10)

The M10 acceptance: **documentation drift caught deterministically at the task boundary** — a `code-anchor` that no longer resolves to real code **blocks `finalize`**. This is VISION principle #6's headline (*validate against reality*) and the differentiator deferred since the MVP shipped only the engine-native probes. The new surface is the first **pack-provided probe** (`doc-code`, a subprocess behind the [determinism contract](validation.md#pack-probe-determinism-contract)) and the first **pack-declared field type** (`code-anchor` — [document-type-schema.md](document-type-schema.md#pack-declared-field-types-m10)). The **floor** is symbol/file existence (`adr.cites-code`); the **headline** is criterion→test mapping (`spec` `maps-to-test`). The proof is the **blocking** case — a happy-path-only acceptance would be a masking test ([validation.md](validation.md#the-doc-code-probe-m10) → Blocking semantics). Notation illustrative; the command spellings below were **spiked against the built grammar** at planning ([DECISIONS.md](../DECISIONS.md) 2026-06-06) — the *new* surfaces (`doc-code`, code-anchor resolution) are pinned at the build's acceptance spike, the scaffolding (`plan` / `implement-from-spec` / `bind` / `finalize`) is built.

### What `doc-code` checks, and how each anchor gets authored

`doc-code` enumerates `code-anchor` leaves over the task's **effective-state docs** — docs the task **created/edited** (working deltas) ∪ docs **bound** into its read roles — and resolves each against the working tree ([validation.md](validation.md#the-doc-code-probe-m10) → Target surface; a code-anchor is a probe-checked *field, not an edge*, so it is reached by direct enumeration, never the edge-index blast-radius). Both M10 targets are placed to sit in that surface:

- **Headline — the `spec` criterion** is a **bound** read-role doc. Its `maps-to-test` anchor lives in the `criteria` repeatable block and is authored through the **editable channel** (raw git edit → reconcile absorb, the M3 flow-6 precedent) — `add-item` / in-CLI repeatable authoring stays deferred ([decisions-pending.md](../implementation/decisions-pending.md)). Seeded *before* the work task is minted (the resume re-compose pins to base; committing after mint advances HEAD and blocks — the spiked constraint).
- **Floor — the `adr`** is **created in-task** via the work workflow's create-gate (`allows-create: [{type: adr, as: decision}]`), so it is a **working delta** in scope. Its `cites-code` header anchor is authored in-CLI via `jigc doc set-field` — which needs the small wiring of the existing `insert_front_matter_field` engine primitive to the verb (an absent optional header field is not settable today — the planning spike's writer-limitation finding; this is scoped into M10's build, distinct from the deferred `add-item`).

Either way the **probe, not a write-time gate, is the adjudicator** (at finalize) — so an editable-channel anchor is caught exactly as a CLI-authored one is, which *is* the reconcile-then-validate property worth proving. The `path#symbol` value is write-safe (only control chars are rejected).

### Setup — a `spec` whose criterion carries a `maps-to-test`, committed *before* the work task

```text
# (1) author + commit a spec via the plan workflow
$ jigc start --workflow plan "the rate-limiter spec"
$ jigc doc create spec --title "Gateway rate limiting" --task <plan-id>
$ jigc doc set-slot "spec:gateway-rate-limiting#goal"    --from-file - --task <plan-id>
$ jigc doc set-slot "spec:gateway-rate-limiting#context" --from-file - --task <plan-id>
$ jigc doc set-field "commit:<plan-id>#type" --value docs --task <plan-id>   # + scope/summary/body
$ jigc task finalize <plan-id>                          # → specs/gateway-rate-limiting.md

# (2) editable channel: add a criterion carrying a maps-to-test anchor — committed on the
#     base branch BEFORE minting the work task (the spiked pin-to-base constraint).
#  specs/gateway-rate-limiting.md  ← append under ## criteria:
#     ### Burst limit  {#burst-limit}
#     Requests beyond 100/min are rejected.
#     - maps-to-test: `crates/engine/tests/rate_limit.rs#burst_rejected`
$ git add -A && git commit -m "seed criterion with maps-to-test anchor"
```

### The passing walk — both anchors resolve, finalize commits

```text
$ jigc start --workflow implement-from-spec "implement the rate limiter"
$ jigc task bind spec spec:gateway-rate-limiting <impl-id>     # ROLE ADDR ID — spec enters effective state
$ jigc start --task <impl-id>                                  # {{@task.spec#criteria}} resolves
# … agent implements; creates an adr recording the choice, citing the code it affects:
$ jigc doc create adr --title "Token-bucket limiter" --task <impl-id>     # create-gate → working delta
$ jigc doc set-field "adr:token-bucket-limiter#cites-code" \
        --value "crates/engine/src/limiter.rs#TokenBucket" --task <impl-id>
# … the test rate_limit.rs#burst_rejected and the symbol limiter.rs#TokenBucket both exist …
$ jigc task finalize <impl-id>
  ✓ doc-code · symbol-exists            adr:token-bucket-limiter#cites-code → resolves     (created doc)
  ✓ doc-code · criterion-maps-to-test   spec:…#criteria/burst-limit/maps-to-test → resolves (bound doc)
  → one commit lands (code + the promoted adr).
```

### The blocking walk — a dangling anchor blocks (the headline proof)

```text
# the agent renames the test away (or never writes it): rate_limit.rs#burst_rejected no longer exists
$ jigc task finalize <impl-id>
  ✗ doc-code · criterion-maps-to-test · blocking
    target:  spec:gateway-rate-limiting#criteria/burst-limit/maps-to-test
    message: criterion maps to no test — `crates/engine/tests/rate_limit.rs#burst_rejected`
             resolves to no test in the working tree
    route:   add the test, re-point the anchor, or drop it
  finalize blocked — no commit created.
```

…and symmetrically for the floor: the created `adr:token-bucket-limiter#cites-code` points at `limiter.rs#TokenBucket`; rename the symbol away and `finalize` blocks with `doc-code · symbol-exists`. The block shape mirrors flow-5/6's `ref-resolves` (probe · check · blocking · target · message · route; non-zero exit; `git rev-list --count HEAD` unchanged).

### What it asserts (the acceptance bar)

1. **The blocking case is the proof.** `finalize` **blocks** on a *dangling* `code-anchor` (present, resolves to no file/symbol/test) and **passes** when it resolves — the `ref-resolves` "blocks when it dangles" pattern, now over real code. A happy-path-only test is rejected as masking.
   - **The check must be asserted to have *run*** — not merely that finalize blocked. The passing walk asserts the report contains the `doc-code.symbol-exists` (on the created adr) and `doc-code.criterion-maps-to-test` (on the bound spec) findings *resolving*; a report with **zero `doc-code` findings** is the masking failure (the target-surface enumeration silently skipped the docs) and fails the bar as hard as a missed block.
2. **The contract is satisfied, not waived.** The `doc-code` subprocess runs under the [six rules](validation.md#the-six-rules); a probe that **times out / crashes / returns malformed output** surfaces as an **intrinsic-blocking `pack-probe-integrity.*` meta-finding**, not a silent pass. Assert at least the timeout + malformed-output meta-findings fire (e.g. a probe stub that sleeps past budget / emits non-JSON). OS-level sandboxing + `sandbox-violation` are out (trusted-pack).
3. **Both extension axes are real.** `code-anchor` is **pack-declared** (the dev pack supplies the type + binds `doc-code` as its adjudicator — the engine ships no pack field type), and `doc-code` is a **pack-provided subprocess probe** reached over the reshaped, serializable `Probe` seam. Asserting the floor (`adr`) + headline (`spec` criterion) exercises both a header-field anchor and a repeatable-block-leaf anchor.
4. **Reads working-tree code + the task's effective-state docs.** The probe resolves anchors against the **working tree** (the bytes about to be committed); the docs carrying those anchors come from the task's effective state — the **bound** `spec` (committed store) and the **created** `adr` (the task's working area). The combination closes the "resolved at compose, code changed before finalize" gap by construction.
5. **Severity tunes through the cascade.** Demoting `validation.doc-code.criterion-maps-to-test.severity: warning` makes the dangling criterion **surface but not block** (the M6 post-pass, no code change); the intrinsic `pack-probe-integrity.*` meta-findings **cannot** be demoted below blocking (floor-rejected at resolution).

## 14. describe — the self-description surface (M11)

The M11 acceptance: **the document model applied to jigc describing itself** — `jigc describe` projects the **resolved** definitions (`project > team > pack-default`) into discursive prose, the *menu* of what can be composed here and how it's used ([introspection.md](introspection.md)). The proof is **not** "describe prints the workflows" — a clean machine-readable list would be a *parseable catalog*, the failure M11 forbids. The two load-bearing proofs are: a **project override visibly changes the projection** (it reflects the cascade — it can't drift, generated from the same definitions that drive composition), and the output is **provably non-contractual** (discursive prose, hostile to parsing, so nothing depends on it — dissolving the "not a public API" non-goal). The new substrate is the first **human-authored prose carried on definitions** (`description:`/`usage:`) — a third prose category, neither write-path slot nor read-path placeholder. Notation illustrative; `jigc describe` is net-new, so the command spelling + output shape are pinned at the build's acceptance spike. The cascade-resolve + whole-file-shadow scaffolding is proven (à la the step shadow path); the **free-prose renderer and the format-property predicate are net-new** — order them first in the build, since the renderer's output shape *is* what the predicate (bar #2) tests.

### Setup — authored prose on the real pack definitions + a project override

```text
# pack-default: the shipped dev-pack definitions carry authored description/usage
#  crates/cli/pack/workflows/single-task.yaml  (front-matter):
#     when: "implement one scoped change end-to-end"
#     description: |
#       single-task takes one well-scoped change all the way from intent to a
#       committed result — compose, implement, validate, finalize, one commit.
#     usage: |
#       Reach for it when the work is a single coherent change you can hold in
#       your head: a bug fix, one feature, a focused refactor. Not for multi-part
#       efforts (those plan first) or trivial edits (quick-fix).
#  crates/cli/pack/schemas/adr.yaml  (top-level, siblings of type/location):
#     description: |
#       An architecture decision record — one decision, its context, and consequences.
#     usage: |
#       Created in-task when a change embeds a decision worth keeping; later work can
#       supersede it. Persisted to decisions/.

$ jigc setup                       # fresh repo + embedded pack

# project override: shadow ONE whole workflow definition at the project layer,
# changing only its usage prose (whole-file shadow — overrides.md, no field-merge).
#  .jigc/config/workflows/single-task.yaml  ← a full copy with an edited usage:
#     usage: |
#       Our house rule makes single-task the default for any change under ~200
#       lines; anything larger goes through plan first. (project-authored)
#   (note: authored prose must not BEGIN a line with a colon-term — see bar #2)
```

### The walk — pack-default projection, then the override changes it

```text
$ jigc describe
  This project can compose a handful of workflows, manage a few document types,
  and run them through jigc. Here's what's on the menu and when each is for.

  Among the workflows, single-task takes one well-scoped change all the way from
  intent to a committed result — reach for it when the work is a single coherent
  change you can hold in your head, like a bug fix, one feature, or a focused
  refactor, rather than a multi-part effort (those plan first) or a trivial edit
  (quick-fix). The router helps you pick when you're unsure; quick-fix … plan …
  implement-from-spec … milestone-execution each get their own paragraph.

  The document types it manages include the adr — an architecture decision record
  capturing one decision, its context, and consequences, which you create in-task
  when a change embeds a decision worth keeping and later work can supersede. The
  commit and spec types each get the same treatment.

  As for commands, jigc start orients you and mints a task, jigc task finalize
  validates and commits, and so on — the authored command-ref hints, woven in as
  prose rather than listed.

# now with the project override in place:
$ jigc describe
  … single-task … Our house rule makes single-task the default for any change
  under ~200 lines; anything larger goes through plan first. …    ← project prose wins
```

The override’s prose **replaces** the pack’s for that one definition (whole-file shadow, `file_owner` picks the project layer); every other definition still shows pack-default prose. A definition that carries **no** `description:`/`usage:` is simply **not narrated** (optional, skip-on-absent — not an error).

### What it asserts (the acceptance bar)

1. **Cascade-reflection is the headline — proven by a visible change, not by listing.** With a project-layer **whole-file shadow** of `single-task.yaml` carrying edited `usage:`, `jigc describe` output **contains the project prose and not the pack prose** for that definition, while unshadowed definitions still show pack prose. A test that only asserts "describe mentions single-task" is rejected as masking — it must assert the *override changed the output*. The override-proof shadows a **workflow or doctype** file (the cascade-reflecting surfaces), **never a command-ref** — command-ref `hint` is projected pack-only in M11 (the catalog read path is pack-only; catalog override-deltas are unbuilt), so a hint override is *not* reflected and is not asserted. (Acceptance is `project > pack-default` only; the team layer is unfed in production.)
2. **Non-contractual is enforced by format — a precise positive predicate, not just a snapshot.** The output (a) is **not valid JSON** and **does not parse as a top-level YAML mapping or sequence** (a bare-scalar parse is fine — prose *is* a YAML scalar); (b) has **no key-shaped line** (no line matches `^\s*[\w-]+:\s`) and **no bullet row** (`^\s*[-*]\s`) — authored prose may contain mid-line colons but must not *begin a line* with a colon-term; (c) exposes **no per-definition extractable key/delimiter** (a consumer cannot pull `single-task`'s `usage` out as a field) — *light unkeyed prose section grouping is permitted* as a non-binding reading aid; (d) meets a **prose-density floor** (reads as paragraphs). A byte-snapshot is kept *additionally* but is necessary-but-insufficient — a snapshot of a bulleted list passes a snapshot while failing this predicate. This is the M11 analogue of M10's "contract satisfied, not waived": the milestone is not built if describe emits a parseable catalog. The predicate must pass on legitimate colon-bearing prose and fail on a structured catalog — both directions are asserted.
3. **The fields are authored-on-definitions, assembled — never generated.** describe makes **no LLM call**; the prose comes verbatim (assembled, cascade-resolved) from `description:`/`usage:` fields on the workflow/doctype definitions + the existing command-ref `hint`. Assert the rendered prose contains the authored strings.
4. **Facts-not-advice is authoring discipline, not a CLI gate — stated honestly.** The CLI cannot mechanically tell facts from advice (the prose is human-authored), so this constraint is enforced at authoring/review, not by the binary. The acceptance asserts what the binary *can*: the projection is non-contractual and cascade-reflecting. (The shipped pack prose is itself held to facts-not-advice / usage-not-mechanism at review.)
5. **The floor is workflows + doctypes; the stretch stayed cut.** describe projects every **workflow** (the unfiltered set — not the `creates-task && selectable` catalog) and every **doctype**, plus command-ref `hint`s. **Data-value roots are absent** (no definition substrate — cut at planning); a describe that tried to narrate `{{store.*}}` / `{{catalog}}` roots would be out of scope. `jigc describe` is **whole-menu** (no positional `describe <id>` form).
6. **Realistic definitions, not renderer-shaped fixtures.** The acceptance authors prose on the **real shipped pack definitions** and asserts over their projection — not a fixture pack shaped to whatever the projector happens to render (the fixture-topology masking face, [increment-workflow.md](../implementation/increment-workflow.md)).

## 15. self-hosting — the methodology pack dogfooded on a fresh project (M12, exploratory)

The M12 acceptance: **jigc composes its own development methodology and runs it on a fresh, unlike-jigc project.** The methodology pack encodes the [dev-workflow](../implementation/dev-workflow.md) **reduced-linear** (scope → implement → gate → commit as flat prose steps), composes as the **sole pack** (it *subsumes* the dev surfaces — vendors a `commit` schema + the full intrinsic-knob surface), and drives a real task end-to-end on a `/tmp` copy of a TypeScript project. The proof is **workflow-composition fidelity** — jigc owns the *structure* (compose deterministically, place the writes, walk the finalize gate), the agent fills the *judgment* (scope-restatement, test-first discipline, running the project's gate). It is **not** the quantitative build-health comparison (that tally lives in the external orchestration harness, not the binary — deferred). And it is honest about what it does **not** prove: the dev-workflow's test-first ordering + the mechanized gate degrade to **un-enforced prose** on a foreign project — that degradation is the milestone's primary finding, the named shape of the dialect-extension milestone, *not* a defect to hide. Notation illustrative; the pack spelling + acceptance flow are pinned at the build's spike (a planning spike, 2026-06-07, already ran this end-to-end on a non-Rust repo — [self-hosting.md](self-hosting.md) → Acceptance flow). The methodology pack is **pure pack authoring** (workflows + steps + schemas + cascade config); **zero engine/dialect code** rides in M12.

### Setup — author the methodology pack, copy the dogfood project to /tmp

```text
# the methodology pack (pure pack data — no engine changes):
#  packs/methodology/config/defaults.yaml : pack-id: methodology, default-workflow: dev-task (creates-task: true)
#  packs/methodology/config/knobs.yaml    : the full intrinsic-check severity surface (floored at blocking) + default-workflow enum
#  packs/methodology/config/commands.yaml : the command-refs the steps use (set-commit-summary, finalize-task)
#  packs/methodology/schemas/commit.yaml  : VENDORED (finalize hardcodes COMMIT_TYPE="commit")
#  packs/methodology/workflows/dev-task.yaml : body = flat {{include: step:scope/implement/gate/finalize}}
#  packs/methodology/steps/scope.yaml     : restate {{task.intent}} + an observable done-criterion; stop-and-check the human if scope drifted
#  packs/methodology/steps/implement.yaml : write the FAILING TEST FIRST (confirm it fails for the right reason), THEN minimal green, THEN refactor — PROSE the agent self-polices
#  packs/methodology/steps/gate.yaml      : "run your project's configured test + lint + build gate; all pass or the task isn't done" — PROSE, no hardcoded `cargo`
#  packs/methodology/steps/finalize.yaml  : set-field type + scope, set-slot summary + body (a command-ref EACH — the commit schema requires all four), then {{cli.finalize-task}}
#   (all step ids single-word — the multi-word section-id defect)
#   (every {{…}} placeholder must be the SOLE content of its line — inline-in-a-sentence emits the literal {{…}})

# the dogfood project — NEVER touched in place:
$ cp -r ~/Projects/gherrink-galey /tmp/m12-dogfood    # a TypeScript pnpm monorepo (unlike jigc)
$ cd /tmp/m12-dogfood && git init -q && git add -A && git commit -qm "baseline"   # if not already a repo

$ JIGC_PACK_DIR=<pack> jigc setup        # step 0 — start hard-fails without a .jigc/config/ cascade layer
$ git add -A && git commit -qm "chore: jigc setup"     # commit setup artifacts BEFORE the work, so the work commit is code-only
```

**`JIGC_PACK_DIR` stays the explicit/dogfood channel — unchanged by M21.** This walk composes the methodology pack **alone** by pointing `JIGC_PACK_DIR` at the pack tree; M21's setup-written `compose-embedded-methodology: true` marker ([flow 23](#23-compose-both-at-setup--the-new-project-methodology-default-m21)) is a *different* channel that wires the embedded `[dev ▸ methodology]` pair. The two never fight: when both are present `JIGC_PACK_DIR` **supersedes the marker** (the T2a precedence — [multi-pack.md](multi-pack.md#embedded-second-pack--setup-auto-wiring-m21)), so the methodology-alone / methodology-primary dogfood ordering this flow drives is **untouched by M21** — the explicit override always wins.

### The walk — compose the dev-workflow, work the task, finalize one commit

```text
$ JIGC_PACK_DIR=<pack> jigc start "add a greeting helper to the utils package"
  # composes the methodology dev-task workflow → emits the flat prose spine:
  Scope — restate the intent and state an observable done-criterion.
    The intent is: add a greeting helper to the utils package
    … stop and check with the human if the restatement reveals a different problem.
  Implement — write the failing test FIRST, confirm it fails for the right reason,
    then the minimal change to make it pass, then refactor while green.
  Gate — run your project's configured test + lint + build gate; all pass or the task isn't done.
  Finalize — <<author: the commit summary>> then run: jigc task finalize add-a-greeting-helper…

# determinism: recomposing the SAME task is byte-identical (test via --task, not re-mint):
#   <first-capture> = the stdout of the ORIGINAL successful `start` above — NOT a re-mint
#   (re-running `start "<intent>"` correctly fails "already active"; don't diff against that error).
$ JIGC_PACK_DIR=<pack> jigc start --task add-a-greeting-helper… | diff - <first-capture>   # empty diff

# the agent does the TS work: writes a failing vitest test, implements, runs `pnpm test && pnpm biome` by hand.
# finalize RENDERS an already-filled commit doc — it does NOT fill the fields. The agent fills all four first:
$ JIGC_PACK_DIR=<pack> jigc doc set-field commit:add-a-greeting-helper…#type  --value feat
$ JIGC_PACK_DIR=<pack> jigc doc set-field commit:add-a-greeting-helper…#scope --value utils
$ JIGC_PACK_DIR=<pack> jigc doc set-slot  commit:add-a-greeting-helper…#summary --from-file -
$ JIGC_PACK_DIR=<pack> jigc doc set-slot  commit:add-a-greeting-helper…#body    --from-file -
  #  note: type/scope are FIELDS (set-field --value); summary/body are SLOTS (set-slot). The commit
  #  schema requires all four non-empty, so the methodology `finalize` step must wire a command-ref
  #  (or spell the literal call) for each — not just summary as the embedded pack does.
$ JIGC_PACK_DIR=<pack> jigc task finalize add-a-greeting-helper…
  # validates, renders the (now-filled) commit doctype → git message, lands ONE commit. Git-only — no cargo.
$ git log --oneline -1
  feat(utils): add greet() helper          ← exactly one new commit, code-only
```

### What it asserts (the acceptance bar)

1. **Composition fidelity is the headline — proven by a real end-to-end run, not by "the pack loads."** On a `/tmp` copy of a **non-Rust** project, the methodology pack composes the dev-workflow **deterministically** (same pack + cascade → byte-identical workflow, tested via the `--task` recompose path) and drives a real task `setup → start → finalize → exactly one git commit`, **git-only** (no `cargo`/`rustc` invoked — verified the finalize path is stack-free). A test that only asserts "the methodology workflow composes" is rejected as masking — it must assert a *task ran through to one commit on the foreign project*.
2. **The judgment slots stay genuinely exercised, not hollowed — and the prose-washed mechanizables are NAMED, not dropped.** Read the encoded steps: scope-restatement, test-first discipline, and the gate are **agent-authored prose**; the encode mechanized **nothing** that is judgment (the hollow-encode failure). *And* the encode mechanized **nothing** it merely *renamed* as structure: the dev-workflow's test-first **ordering** + the **gate** are mechanizable-but-unencodable-today, so they ride as prose — and that degradation is **recorded as a dialect-extension trigger** with the failure-class each would prevent ([self-hosting.md](self-hosting.md) → named triggers), never silently presented as faithful structure (the prose-wash failure). Both edges of the acceptance bar are asserted.
3. **The pack composes ALONE and is pure pack data.** The methodology pack is the **sole** composed pack (`JIGC_PACK_DIR`, no second pack layered — that is M14); it **subsumes** the dev surfaces by vendoring the `commit` schema + the full intrinsic-knob surface; and it ships **zero engine/dialect changes** (asserted: the M12 diff touches no `crates/*/src`). The portable gate composed as plain prose — the binary has no opinion about gate content — and `{{task.intent}}` resolved in the composed output.
4. **The working docs stay plain markdown (the M13 seam holds).** The encoded dev-workflow only **reads** project/methodology docs at fixed paths; it authors **no** managed methodology doctype and places **no** jigc write into the roadmap/ledger/decisions-log. (If a faithful dogfood had needed the Scope step to mechanically consume a *managed* deferral-ledger, that one doctype would promote into M12 — it did not; the dev-workflow only reads.)
5. **The doc↔code probe is gated off, honestly.** The methodology pack ships **no `code-anchor`** field, so the M10 `doc-code` probe never fires — correct, since it is Rust-grammar-only and would false-block on a TypeScript project (at M12; generalized at M27 — the probe now spans six languages, so a TypeScript anchor resolves rather than false-blocks, [validation.md](validation.md) → Multi-language resolution). A dogfood that wanted to *prove* doc↔code on a foreign project is out of the reduced slice's scope (named, not attempted).
6. **Realistic project, not a jigc-shaped toy.** The dogfood runs on a **real, unlike-jigc** project (a TypeScript monorepo) so jigc-shaped gates can't flatter a jigc-shaped target — the portability test the methodology pack exists to pass. The original project is **never modified** (the run is on a `/tmp` copy).

## 16. arch-doc↔code — architecture prose validated against the code it describes (M13)

The M13 acceptance: **the richest doc↔code case activates** — a task documents a part of jigc's *own* (Rust) architecture as an `arch-doc`, and `finalize` blocks when a documented component's symbol has vanished. This is the doctype-starting-set terminus (`arch-doc` is the last unbuilt member — [doctype-map.md](../implementation/doctype-map.md)) and M10's `doc-code` differentiator turned on its hardest target: unlike `adr` (one header `cites-code`) or `spec` (one `maps-to-test` per criterion), an `arch-doc` carries **one `implemented-by` anchor per component** — a *repeatable*-item code-anchor — so the proof is **per-item disambiguation**: each component's anchor must resolve against *its own* authored value, never a clobbered shared one. The new engine concept is the per-field-type predicate **`check:` selector** (position no longer chooses the `doc-code` predicate, since a repeatable `components` anchor wants `symbol-exists`, not `criterion-maps-to-test`); the new edge is `arch-doc —cites→ adr` (n→n, header-level). The design of record is [architecture-documentation.md](architecture-documentation.md) → The acceptance flow; the workflow, schema, `check:` selector, and item-scoped setters live there and are not restated here. Notation illustrative; the flow below is the shape the acceptance test ([arch-doc↔code](architecture-documentation.md#the-acceptance-flow-flow-16--the-bar)) drives end-to-end through the built binary against the real `doc-code` subprocess.

### Setup — a committed `adr` for the `cites` edge to resolve against

```text
# author + finalize the decision the PASS half will cite (its canonical decisions/ home):
$ jigc start --workflow plan "record the cache decision"      # (or any workflow with the adr create-gate)
$ jigc doc create adr --title "Use a cache" --task <plan-id>
$ jigc doc set-slot "adr:use-a-cache#context"      --from-file - --task <plan-id>
$ jigc doc set-slot "adr:use-a-cache#consequences" --from-file - --task <plan-id>
$ jigc task finalize <plan-id>                                # → decisions/use-a-cache.md
```

### The walk — author two components, block on a vanished symbol + a dangling cite, fix → one commit

```text
$ jigc start --workflow architecture-documentation "document the index layer"
$ jigc doc create arch-doc --title "Index layer" --task <id>          # mints arch-doc:index-layer (bound task.arch-doc)
$ jigc doc set-slot  "arch-doc:index-layer#overview" --from-file - --task <id>

# the doc-level n→n cites — pointed (for now) at a NON-EXISTENT adr:
$ jigc doc set-field "arch-doc:index-layer#cites" --value "adr:no-such-decision" --task <id>

# per component: add-item materializes the ## Components home, then set the leaves.
# the FIRST add-item materializes `## Components` at its schema-ordered position (after ## Overview).
$ jigc doc add-item  "arch-doc:index-layer#components" --title "Edge index"     --task <id>   # → #components/edge-index
$ jigc doc set-slot  "arch-doc:index-layer#components/edge-index/description"   --from-file - --task <id>
$ jigc doc set-field "arch-doc:index-layer#components/edge-index/implemented-by" \
        --value "crates/engine/src/index.rs#rebuild_committed" --task <id>
$ jigc doc add-item  "arch-doc:index-layer#components" --title "Target surface" --task <id>   # → #components/target-surface
$ jigc doc set-slot  "arch-doc:index-layer#components/target-surface/description" --from-file - --task <id>
$ jigc doc set-field "arch-doc:index-layer#components/target-surface/implemented-by" \
        --value "crates/engine/src/target_surface.rs#collect_repeatable" --task <id>
#  two DIFFERENT, independently-resolving anchors — the per-item disambiguation fixture.

# the agent deletes/renames component A's symbol (rebuild_committed) while B's (collect_repeatable) stays valid:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:index-layer#components/edge-index/implemented-by      ← A's item address, NOT B's
    message:          `crates/engine/src/index.rs#rebuild_committed` resolves to no symbol in the working tree
  ✗ schema-conformance · ref-resolves · blocking
    target:  arch-doc:index-layer#cites → adr:no-such-decision    ← the dangling cite
  finalize blocked — HEAD unchanged, nothing promoted.
```

The `symbol-exists` block naming **A's** address while **B's** anchor (un-deleted) stays silent is the per-item disambiguation proof: were the item-leaf setters clobbering one shared value, B's deletion-free anchor could not stay green while A's named-and-deleted one blocks. Now fix both — restore A's symbol (or re-point its anchor) and re-point `cites` at the committed adr:

```text
$ jigc doc set-field "arch-doc:index-layer#cites" --value "adr:use-a-cache" --task <id>
# … A's symbol is back in the working tree …
$ jigc task finalize <id>
  ✓ doc-code · symbol-exists   (edge-index, target-surface both resolve)
  ✓ schema-conformance · ref-resolves   (cites → adr:use-a-cache resolves)
  → one docs(arch-doc): commit lands; arch-doc:index-layer promotes to architecture/index-layer.md.
$ git log --oneline -1
  docs(arch-doc): document the index layer        ← exactly one commit; the doc at the fourth location:
```

### What it asserts (the acceptance bar)

1. **The blocking case is the proof, named to A's item address.** `finalize` **blocks** on `doc-code.symbol-exists` for component A's *vanished* anchor while component B's *present* anchor stays silent, and the rendered report's `location.address` is **A's** item address (`#components/edge-index/implemented-by`), **never** B's — the per-item disambiguation proof over a repeatable-item anchor (hardening #5: the two-component A-deleted-B-valid fixture is what forces it; a single-component or clobbered fixture proves nothing). A happy-path-only acceptance is rejected as masking — the **block** direction is the mandated half (the PASS for one present item anchor is already proven at increment 4).
2. **The `## Components` home materializes schema-ordered.** The first `add-item` materializes `## Components` at its schema-ordered position (after `## Overview`) — the create→first-`add-item` path the engine has not produced before, asserted as a named check (T1) so the snapshot-spawn over a repeatable item rides a correctly-placed section.
3. **The first real finalize snapshot-spawn over a repeatable-item anchor.** The orchestrator that writes the `doc-code` snapshot and spawns the subprocess during `finalize` — golden-tested for shape but unexercised for an *item* anchor until now ([architecture-documentation.md](architecture-documentation.md) → Honest caveats) — is exercised end-to-end against the real Rust-grammar probe, the anchors pointed at jigc's own codebase (sidestepping the probe's Rust-only limit cleanly, as for M12).
4. **The n→n `cites` edge walks at finalize.** A **dangling** `cites` target blocks `finalize` on `schema-conformance.ref-resolves` (the proven `supersedes`/`implements` walk, now for the M13 edge); re-pointing it at a committed `adr` passes. The block names the dangling target.
5. **The `check:` selector is real, both predicates preserved.** The repeatable `components` anchor gets `symbol-exists` (not the test predicate) via the explicit per-field-type `check:` selector, while `spec.criteria/maps-to-test` keeps `criterion-maps-to-test` — both flows (13 and 16) are the selector increment's green-bar, not flow 16 alone ([architecture-documentation.md](architecture-documentation.md#the-per-field-type-predicate-selector-check--the-m13-engine-concept)).
6. **Fix → exactly one commit at a fourth `location:`.** After fixing both blocks, `finalize` lands **exactly one** `docs(arch-doc):` commit (code-less promotion via the generic `plan_promotions` loop), promotes the doc to `architecture/index-layer.md` (the fourth managed `location:`, joining `decisions/`/`specs/`/`prds/`), and cleans the working area.

## 17. multi-pack composition — dev + methodology co-composed (M14)

The M14 acceptance: **one project composes the embedded dev pack *and* the M12 methodology pack at once**, with the subsume's two collision *kinds* resolving on their two distinct axes: the **top-level** collisions (`commit` doctype, `default-workflow` knob) resolve **by precedence** (highest pack's whole definition wins), while the **same-id body-references** (steps `implement`/`finalize`, the `{{cli.X}}` command-refs, and schema field-types) resolve **pack-locally** (each definition gets its *own* pack's). Each pack's *distinct* surfaces co-compose on top. This is the generalization of *engine-neutral, packs supply content* from internal discipline to architecture at scale ([VISION.md](../VISION.md) → Open questions → Multi-pack composition, de-parked here). The design of record is [multi-pack.md](multi-pack.md); the pack-set model, the precedence-override collision table, and the pack-local body-reference rule live there and are not restated. Notation illustrative. The headline split: the **single-pack floor stays byte-identical to today**, and *on top of that* the **two-pack composition** behaves as designed — both halves are the bar.

### Setup — the single-pack floor, then add the second pack

```text
# the second pack already exists (M12 minted it): packs/methodology/ — VENDORS commit + the knob surface,
#   default-workflow enum rewritten to [dev-task] (the subsume; see self-hosting.md). It STILL composes alone.

# (a) the cold-start floor — NO listed packs ⇒ pack-set is [base] ⇒ byte-identical to the single-pack path.
#   This is an IN-BINARY equivalence (a cross-binary `diff` against a pre-M14 capture is NOT a clean oracle —
#   task-id/branch/HEAD/provenance vary). The real check: Composite([base]) composes byte-for-byte the same as the
#   old SinglePackSource(base) — asserted as a compose-level golden with all non-pack variables pinned (the
#   existing no-delta `start_compose` goldens, now driven through the composite-of-one path):
$ JIGC_PACK_DIR=<base> jigc start --task <fixed-id>   # composite-of-one output == the single-pack golden (empty diff)

# (b) compose the two packs: methodology listed (highest), the embedded dev pack the implicit base (lowest):
$ cat .jigc/config/packs.yaml
  packs:
    - /abs/path/to/packs/methodology     # earlier = higher precedence; embedded dev is the implicit base below
```

### The walk — collisions resolve by precedence; distinct surfaces co-compose; includes stay pack-local

```text
$ jigc start                                  # bare start — no --workflow
  Pack: methodology/0.1.0 ▸ dev/0.0.0         ← provenance names BOTH composed packs, highest-first
  # default-workflow resolves to dev-task (methodology's knobs.yaml wins the whole-file shadow):
  Scope — restate the intent and state an observable done-criterion …      ← methodology's dev-task spine
  Implement — write the FAILING TEST FIRST, confirm it fails for the right reason …   ← methodology's test-first implement

# pack-local body-references (the correctness proof): dev ALSO ships a step:implement (direct-edit, no test-first),
# and methodology's catalog has only 5 command-refs (dev's has 11 — NOT mutual supersets).
# methodology's dev-task composed METHODOLOGY's implement + command-refs, NOT dev's — each resolved in its own pack.

# both packs' DISTINCT surfaces co-compose — dev's workflows + doctypes are available alongside methodology's:
$ jigc start --workflow single-task "add a thing"     # dev's single-task is in the union catalog → composes fine
  # its {{cli.create-adr}} resolves against DEV's own commands.yaml (pack-local) — methodology's catalog lacks it,
  # but that never matters: a workflow's command-refs resolve in the pack that defines the workflow.
$ jigc describe | grep -E "dev-task|single-task|router|adr|spec|arch-doc"
  dev-task        ← methodology         single-task / router / adr / spec / arch-doc   ← dev   (the UNION menu)

# the colliding knob + doctype, shown resolving deterministically (provenance glyphs illustrative, not a byte-contract):
$ jigc start --explain | grep -E "default-workflow|schema:commit"
  default-workflow = dev-task   (pack-default ◂ methodology/0.1.0, shadowing dev/0.0.0)   ← knob collision winner
  schema:commit                 (pack-default ◂ methodology/0.1.0, shadowing dev/0.0.0)   ← doctype collision winner
$ jigc start --explain | grep -E "Pack input"
  Pack input: methodology/0.1.0 = /abs/.../methodology  (blake3 a3f9…) ◂ dev/0.0.0 = <embedded> (blake3 71c2…)
  #  --explain names each pack's PATH + content-hash — id/version alone is not the identity (dir contents can change)

# dev doctypes don't just LIST — they LOAD + VALIDATE (assertion 4), proving schema-pack-local field-type resolution:
$ jigc start --workflow architecture-documentation "document the index layer"     # dev workflow composes (its steps + cli.X pack-local)
$ jigc doc create arch-doc --title "Index layer" --task <id2>                      # dev doctype mints
$ jigc doc set-field "arch-doc:index-layer#components/edge-index/implemented-by" \
        --value "crates/engine/src/index.rs#rebuild_committed" --task <id2>
  #  the implemented-by `code-anchor` field RESOLVED — dev's field-type came from DEV's field-types.yaml,
  #  though methodology (the precedence winner) ships none. Without schema-pack-local resolution this would
  #  fail "unknown field type: code-anchor". That is the field-type half of pack-local body-references.

# determinism: recomposing the SAME task is byte-identical (the bar every prior flow asserts):
$ jigc start --task <id> | diff - <first-capture>     # empty diff
```

### What it asserts (the acceptance bar)

1. **The single-pack floor — `Composite([base])` is byte-identical to the single-pack path, in the same binary.** With `.jigc/config/packs:` absent the pack-set is `[base]`, and composing through the new composite-of-one source produces output **byte-identical** to the old single-pack path — asserted as a compose-level golden with non-pack variables pinned (the existing no-delta `start_compose` goldens, now driven through the composite), **not** a fragile `diff` against a pre-M14 binary (HEAD/branch/task-id/provenance vary). The whole multi-pack machinery adds **zero** observable change to a one-pack project — and the methodology pack's M12 *composes-alone* regression tests stay green untouched. A build that perturbs the one-pack path is rejected (the [M13 cold-start trap](../implementation/milestone-planning-workflow.md)).
2. **Both top-level collisions resolve deterministically, and the winner is visible.** `default-workflow` resolves to `dev-task` (methodology's whole `knobs.yaml` wins the precedence shadow), so a bare `jigc start` composes the methodology loop; **and** `schema:commit` resolves to methodology's — *both* named in `--explain` as the collision winner (the determinism contract — [overrides.md](overrides.md#the-cascade) — extended to N packs). Both choices were proven authoring-compatible at Settle, so the pick is determinism-policy, not correctness ([multi-pack.md](multi-pack.md#why-the-commit-doctype-collision-is-safe-either-way)).
3. **Body-references are pack-local — the correctness proof.** Both packs ship a `step:implement` with **divergent** bodies (methodology test-first; dev direct-edit), and their command catalogs are **not mutual supersets** (methodology 5 refs, dev 11). Methodology's `dev-task` composes **methodology's** implement and command-refs, never dev's; dev's `single-task` composes **dev's** `{{cli.create-adr}}` even though methodology's catalog lacks it — because a workflow's `{{include:}}` *and* `{{cli.X}}` resolve within its **own** pack, *independent* of which pack wins a top-level id. A composition that injected dev's implement into methodology's workflow (or stripped dev's `single-task` of `create-adr`) is the M3-class silent corruption this rule exists to kill ([multi-pack.md](multi-pack.md#pack-local-body-reference-resolution-a-correctness-rule-not-a-policy)).
4. **Both packs' distinct surfaces co-compose — and dev's doctypes LOAD + VALIDATE, not just list.** The selectable-workflow catalog is the **union** — dev's `single-task`/`router`/… are invocable via `--workflow` (each resolving its own pack's steps + command-refs). Crucially, dev's `adr`/`spec`/`arch-doc` don't just appear in `describe` — a dev workflow **creates** one and **sets a `code-anchor` field on it**, which only resolves because a schema's field-types come from **its own** pack (dev's `field-types.yaml`), though methodology — the precedence winner — ships none. A `describe | grep`-only assertion is rejected as masking; the bar is a dev doctype that *resolves its pack-field-type and validates*. This is the *co-composition* proof M12 explicitly deferred to M14 (M12 ran the methodology pack **alone**).
5. **Provenance names every composed pack — by path + content, not just id/version.** The orientation header shows the pack-set highest-first (`Pack: methodology/0.1.0 ▸ dev/0.0.0` — glyphs illustrative), and `--explain` surfaces each pack's **resolving directory path + a content hash** — because a `.jigc/config/packs:` entry is a literal directory whose `id/version` is not a sufficient identity (two dirs can share it; contents mutate). The resolved cascade stays a fully-declared, *inspectable* input under composition.
6. **Determinism holds under composition.** Recomposing the same task (`--task`) is byte-identical — the pre-merge collision resolution is a pure function of the pack-set, same packs in → same composed output, the [VISION principle #1](../VISION.md) claim at pack scale.

## 18. increment-workflow encode — a multi-phase, human-gated workflow composed through jigc (M15)

The M15 acceptance: jigc composes its **own harder development workflow** — the [increment-workflow](../implementation/increment-workflow.md) (plan → execute → validate → fix) — as a methodology-pack definition, the deepest self-hosting proof beyond M12's reduced-linear dev-workflow slice. The one new dialect primitive is the [`checkpoint` step kind](workflow-dialect.md#the-checkpoint-step-kind-m15): the harness's halt points (a new fork at Plan, a blocked task at Execute, still-blocking after the fix-round cap) become **structural** halt directives rather than buried prose. The design of record is [workflow-dialect.md](workflow-dialect.md#the-checkpoint-step-kind-m15); the no-runtime carve-out (a checkpoint *halts*, never *branches*) lives there and in [VISION.md](../VISION.md) → Non-goals, not restated. Notation illustrative.

**The honest bound (the falsifiable edge, per [self-hosting.md](self-hosting.md#what-m15-built-of-these-the-increment-workflow-encode-2026-06-09)).** jigc composes the **single-agent composable spine** — the phases one agent walks, with the checkpoint halts made structural. The increment-workflow's **multi-agent orchestration** (one agent per phase, an *independent* read-only validator, the bounded ≤3-round fix re-run) stays **orchestration-level** (the build harness, `.claude/workflows/milestone-build.js`), exactly as M12 bounded its reduced-linear encode and M8 carved out the genuine spawn. This flow proves the *composition*, not the *loop*; presenting the composed spine as the whole orchestration is the hollow-dogfood trap ([self-hosting.md](self-hosting.md) → the two-half pattern).

### Setup — the methodology pack gains an `increment` workflow + checkpoint steps

```text
# extend packs/methodology/ (M12's pack) — ADD, do not touch the existing prose-degraded dev-task steps
#   (the methodology_honesty_artifact tripwire pins those; structuralizing them would go red — out of scope here):
#
#   workflows/increment.md         creates-task: false   # the OUTER loop — operates on an increment, mints no task
#     {{include: step:plan}}              # Reason prose: cut single-concern tasks from the roadmap increment
#     {{include: step:plan-gate}}         # checkpoint  reason: new-fork-at-plan
#     {{include: step:execute}}           # Reason prose: run each task through the dev-workflow, serially
#     {{include: step:execute-gate}}      # checkpoint  reason: blocked-task
#     {{include: step:validate}}          # Reason prose: independently check the increment against the roadmap
#     {{include: step:fix-gate}}          # checkpoint  reason: fix-rounds-exhausted (halt-pending-fix)
#
#   steps/plan-gate.md, execute-gate.md, fix-gate.md   — each: ---\ncheckpoint:\n  reason: <slug>\n--- + halt-prose body
#
# CONSTRAINT (spiked at planning against the real binary): increment is creates-task:false, so its steps must be
#   TASK-REF-FREE — a {{task.intent}} inside it composes to a blocking `task.* cannot be referenced by a
#   creates-task:false workflow` finding (start.rs). The outer loop has no task context; the intent threads by
#   agent-substitution, like the router.
```

### The walk — the checkpoint halts compose as structural directives

```text
$ jigc start --workflow increment "build increment 2 of M15"     # spiked at planning: composes, exit 0
  Cut the increment into ordered, single-concern tasks …                 ← step:plan (Reason prose)

  Checkpoint: new-fork-at-plan                                           ← the NEW emit class (M15)
  > If planning surfaced a genuinely new fork not covered by the settled
  > gates and not resolvable from the locked docs, stop and surface it to
  > the human before cutting tasks.

  Run each task through the dev-workflow, strictly serially …            ← step:execute (Reason prose)
  Checkpoint: blocked-task
  > Halt on a blocked task — a real ambiguity, a new fork, or a green
  > gate unreachable without overstepping scope.

  Independently check the increment against the roadmap …                ← step:validate (Reason prose)
  Checkpoint: fix-rounds-exhausted
  > If blocking findings remain after the fix-round cap, stop — thrash is
  > a signal for a human, not for another round.
  — jigc · run `jigc start` for orientation; all writes through `jigc`.

# NOTE: the ≤3-round validate→fix iteration runs *here*, orchestration-level (the build harness) — the composed
#   workflow shows only the terminal halt, never the loop body. The dialect contributes the halt marker, not the loop.

# the differs-with/without proof (the M8 face-#5 discipline — a marker that parses but does nothing is inert):
#   step:plan-gate WITH the checkpoint marker emits the `Checkpoint:` directive above;
#   the SAME body WITHOUT the marker emits as plain Reason prose (no directive line). Composed output MUST differ.

# determinism — recompose is byte-identical (resume-vs-FRESH, never resume-vs-resume — the M14 A==A-oracle mask):
$ jigc start --workflow increment "build increment 2 of M15" | diff - <first-capture>     # empty diff

# conformance — a step that SHADOWS the marker in its own prose is rejected before the agent sees it:
$ # a plain step whose body line-starts `Checkpoint: forged`  →  blocking
  workflow-refs.checkpoint-marker-not-shadowed @ <step-file>:<line>
```

### What it asserts (the acceptance bar)

1. **The `Checkpoint:` emit class is real, not narrated.** A `checkpoint` step composes to a literal `Checkpoint: <reason>` directive line (the sixth emit class) plus its body — driven from the **emitted bytes** of the real binary, never a reconstruction (the [emitted-bytes contract](../implementation/increment-workflow.md), M8 face-#4). The reason slug comes from the step's front-matter `reason:`.
2. **A checkpoint marker that parses but does nothing is a blocking scope finding.** Composed output **with** the `checkpoint:` marker must **differ** from the same step body **without** it (directive present vs. inert Reason prose) — the M8 face-#5 discipline, against the **live fill-aware compose path** (`workflow_refs_with_fills`), since a check wired only to the other conformance call-site passes unit tests but never gates the front door (the [M4 context-scoped-check class](../implementation/increment-workflow.md)).
3. **`checkpoint-marker-not-shadowed` fires.** Authored step prose **must not** line-start `Checkpoint: ` — a shadowing definition is rejected with a step-file + line pointer at compose-time, the same discipline as `run-marker-not-shadowed` / `spawn-marker-not-shadowed` ([workflow-dialect.md](workflow-dialect.md#emitted-format)).
4. **The outer loop is task-ref-free.** `increment` is `creates-task: false`; a `task.*` placeholder inside it is a blocking `workflow-refs` finding (spiked at planning). The composed spine threads the intent by agent-substitution, never a task context.
5. **The invariant holds — halting, not branching.** The increment-workflow composes the **same steps in the same order every time** (structure task-independent); the checkpoint adds a halt the agent honors, no step appears/disappears on runtime state, and resumption is the existing **stateless restart-from-scratch** re-compose (`start --task` re-derives from disk — no progress cursor, no runtime). A build that introduces *any* mid-workflow progress state to "remember it halted" breaks restart-from-scratch and is rejected (the forward-review *resume-vs-fresh* probe).
6. **The structural deliverable is the halts-as-directives, not the prose.** The encode's *structural* content is the three `checkpoint` directives — deterministically composed, cascade-overridable (a project can insert/override a halt), and shadow-protected — over otherwise-prose phases. The advance over M12 is precise and gradeable: the M12 dev-workflow had its Settle-halt *only as buried prose* ([self-hosting.md](self-hosting.md)); M15 makes the halts **machine-recognizable structure** an orchestrator can extract and act on. That — not "three prose files" — is what composing this buys.
7. **The honest bound is observed, not over-claimed.** The flow proves jigc *composes* the multi-phase human-gated spine; the **bound (≤3 rounds), the per-phase-agent independence, the independent read-only validate, and the re-run** stay orchestration-level (the build harness), unchanged by M15. A milestone record presenting the composed spine as the whole orchestration loop fails the scope-honesty bar ([self-hosting.md](self-hosting.md) → the two-half pattern).

*(Spiked at planning against the real binary: `jigc start --workflow <X>` composes a `creates-task:false` workflow (exit 0); a `{{task.intent}}` inside it is a blocking finding; and a `checkpoint:` marker on a step today composes as **inert Reason prose** — `StepFrontMatter` does not reject unknown keys — which is exactly the "before" state this flow's assertion 2 converts to "after." The `Checkpoint:`-directive emission is the deliverable itself, so its assertion is the build's **red obligation**, provable only once the step kind is built.)*

## 19. Planning encode — jigc composes its own milestone-planning spine and maintains its running docs (M16)

The M16 planning half ([methodology-docs.md](methodology-docs.md#acceptance-flows) → Acceptance flows, Flow 19): jigc composes its **own milestone-planning workflow** (scope → detect-gaps → settle → review → decompose) as a methodology-pack definition, **and** authors-and-maintains the three running working-docs that loop produces — the `roadmap`, the `deferral-ledger`, and the `decisions-log` — as managed doctypes. This closes the planning machinery of the self-hosting loop the [increment encode](#18-increment-workflow-encode--a-multi-phase-human-gated-workflow-composed-through-jigc-m15) (flow 18) and M12's reduced dev-workflow opened. The doctype shapes, the singleton/own-location decisions, and the engine deltas (repeatable-section conformance, idempotent-create) are the design of record in [methodology-docs.md](methodology-docs.md) and are not restated here. Notation illustrative.

The `planning` workflow is **`creates-task: true, selectable: false`** — the `sub-task` precedent ([methodology-docs.md](methodology-docs.md#the-authoring-spine--creates-task-true-selectable-false-the-sub-task-precedent)): it **mints a task** (so the running-doc authoring rides the proven create-gate + finalize-promote spine) yet ships **no `when`** and stays out of the methodology pack's `default-workflow.of` enum, so it is **off-router** (invoked directly, never offered by the model-free selection catalog). It declares `allows-create:` the three doctypes (`{type: roadmap, as: roadmap}`, `{type: deferral-ledger, as: ledger}`, `{type: decisions-log, as: log}`).

**The honest bound (per [methodology-docs.md](methodology-docs.md#what-m16-proves-and-what-it-does-not), the single-agent-spine bound — sharper than M15).** The real milestone-planning loop is **multi-agent orchestration** — it fans out `capability-auditor`/`gap-detector`/`design-reviewer` recon and an independent design review. jigc composes the **single-agent linear spine** — the phases one agent walks, with the human-gated **Settle** made a structural `checkpoint` halt. The fanned recon and the independent review stay **orchestration-level** (the build harness) and irreducible judgment. The **build-time honesty watch** ([methodology-docs.md](methodology-docs.md#what-m16-proves-and-what-it-does-not), the `methodology_honesty_artifact` bar): the gap-detection and Settle phases stay **pure agent-judgment prose** — never a `gap-count ≥ N` lint or a structured checklist; the checkpoint **halts, it never scores**. And M16 proves the machinery on **fresh instances**, not jigc's own historical 800-line `roadmap.md`/`DECISIONS.md` (the in-place migration stays for v1; the roadmap's `decomposition` is a prose slot, the deferred one-level bound).

### The walk — compose the planning spine, author the three running docs, finalize (run 1: cold-create)

```text
$ jigc setup                                                # installs the methodology pack
$ jigc start --workflow planning "M-Alpha"                  # off-router, invoked directly; mints a task
  Scope the milestone against the roadmap …                          ← step:plan-scope (Reason prose)
  Detect the gaps — capability/coverage holes …                      ← step:detect-gaps (Reason prose, NOT a lint)

  Checkpoint: settle                                                 ← the human-gated halt, made structural (M15 kind)
  > Settle the gaps — the human-in-the-loop gate. … A gap consciously
  > deferred is logged on the deferral ledger, keyed to the milestone
  > that will own it. The human owns this gate.

  Independently review the settled scope …                           ← step:plan-review (Reason prose)
  Cut the milestone into ordered increments …                        ← step:decompose (Reason prose, one level)
  Author the roadmap milestone entry …                               ← step:author-roadmap
  Author the deferral-ledger entries …                               ← step:author-ledger
  Author the decisions-log entry …                                   ← step:author-decisions
  — jigc · all writes through `jigc`.

# author the running docs through the task-scoped create-gate — idempotent-create then add-item/set-slot/set-field.
# COLD (no committed singleton yet): `create` mints the fixed-slug instance (singleton ⇒ slug == type id).
$ jigc doc create roadmap          --title Roadmap          # → roadmap:roadmap.md  (mints; cold)
$ jigc doc create deferral-ledger  --title Deferral-Ledger  # → deferral-ledger:deferral-ledger.md
$ jigc doc create decisions-log    --title Decisions-Log    # → decisions-log:decisions-log.md

# one roadmap milestone entry — add-item returns the item address; author BOTH slots (the two-slot item case):
$ jigc doc add-item roadmap:roadmap#milestones --title "M-Alpha"     # → roadmap:roadmap#milestones/<id>
$ jigc doc set-slot <item>/proves         --from-file -             # "what M-Alpha proves"
$ jigc doc set-slot <item>/decomposition  --from-file -             # the increments, AS PROSE (one-level bound)

# a deferral-ledger entry — the kind enum (D/I) + the trigger milestone string + the body slot (date is set on-create):
$ jigc doc add-item deferral-ledger:deferral-ledger#entries --title "defer the alpha cleanup"
$ jigc doc set-field <entry>/kind     --value D                     # D = deferred decision, I = parked idea
$ jigc doc set-field <entry>/trigger  --value M-Beta                # the milestone that resurfaces it (plain string)
$ jigc doc set-slot  <entry>/body     --from-file -

# a decisions-log entry — the why slot (date set on-create):
$ jigc doc add-item decisions-log:decisions-log#entries --title "chose the alpha shape"
$ jigc doc set-slot <entry>/why --from-file -

$ jigc task finalize m-alpha
> validate: clean
> promote: roadmap/roadmap.md · ledger/deferral-ledger.md · decisions-log/decisions-log.md
> commit:  docs(planning): plan a milestone   ← one commit, the three running singletons + the code
```

Each singleton promotes to **its own `location:` subdir** (`roadmap/`, `ledger/`, `decisions-log/` — never a shared root, the reconciliation-sweep + `identity_of` reason in [methodology-docs.md](methodology-docs.md#the-four-doctypes-earned-from-their-drivers)), and the committed bytes are **byte-stable** against the staged promote source.

### Run 2 (a second milestone) — warm-append: idempotent-create copies-in, re-promotes byte-stable

```text
$ jigc start --workflow planning "M-Beta"                   # a second milestone, a second task
# WARM (the singletons are already committed): `create` on an existing committed slug COPIES THE COMMITTED DOC IN
#   (the read_or_copy_in path) — NOT a blank mint, NOT a clobber. The prior M-Alpha entry survives; provenance
#   records `edited-from-base`. This is the create-or-update mechanism — the workflow always says "create the
#   roadmap," safe whether or not it exists (a deterministic state-dependent write, not a workflow conditional).
$ jigc doc create roadmap --title Roadmap                   # copy-in: the M-Alpha entry is preserved
$ jigc doc add-item roadmap:roadmap#milestones --title "M-Beta"   # append the new milestone
$ jigc doc set-slot <item>/proves        --from-file -
$ jigc doc set-slot <item>/decomposition --from-file -
#   … likewise warm-create + append the ledger and decisions-log entries …
$ jigc task finalize m-beta
> promote: roadmap/roadmap.md (re-promoted byte-stable, BOTH M-Alpha + M-Beta present) · …
```

The warm-promoted `roadmap/roadmap.md` carries **both** milestone entries; the ledger and log carry both runs' entries — the running singletons are **demonstrated maintained over time**, not only cold-created (the two-run bar, [methodology-docs.md](methodology-docs.md#acceptance-flows)). The copy-in keys on the schema's `singleton: true` flag, so non-singleton `create` (the MVP `single-task` ADR flow) keeps its mint-or-reject semantics unchanged.

### The three reds — each fires-and-blocks on real input, not a fixture

```text
# RED 1 — a half-authored roadmap entry BLOCKS at validate (Increment 1's per-item conformance fires on THIS schema):
$ jigc doc add-item roadmap:roadmap#milestones --title "M-Half"
$ jigc doc set-slot <item>/proves --from-file -            # author `proves` ONLY; leave `decomposition` empty
$ jigc task finalize m-half
> BLOCK schema-conformance.required-slot-present @ roadmap:roadmap … #milestones/<id>/decomposition
#   the gate names the genuinely-empty `decomposition` slot; non-zero exit, NO commit (the
#   vacuously-green-managed-doc-gate guard — a real empty leaf, not a malformed fixture).

# RED 2 — `planning` is ABSENT from the router catalog (selectable:false, off-router — the M8 catalog-leak class):
$ jigc start                                               # bare orientation, the live front-door path
> … (no catalog line names `planning`) …
$ jigc start --format json | jq '.workflows[].id'         # the JSON `workflows` array carries no `planning` id

# RED 3 — a baseline-recorded-then-OOB-drifted warm-append CONFLICT-BLOCKS at finalize-preflight:
#   run 1's finalize records the committed baseline FIRST; the committed roadmap is then drifted out-of-band
#   (a hand-edit + raw git commit advances HEAD without touching the recorded baseline → DRIFTED). A warm task
#   copies-in + edits the roadmap (TOUCHED). reconcile_committed_store sees DRIFTED+TOUCHED → no silent merge.
$ jigc task finalize m-drift
> BLOCK reconciliation.conflict-block @ roadmap/roadmap.md      # non-zero exit, NO commit
#   (recording-first is what makes this a genuine red — an unrecorded baseline would baseline-adopt vacuously.)
```

### What it asserts (the acceptance bar)

1. **jigc composes its own planning spine.** `jigc start --workflow planning "<milestone>"` mints a task (off-router) and composes the five phases — scope / detect-gaps / settle / review / decompose — with **Settle** emitted as a structural `Checkpoint: settle` directive (the M15 `checkpoint` kind), driven from the real binary's emitted bytes.
2. **The three running docs are authored and maintained across two milestone runs.** Run 1 cold-creates the `roadmap`/`deferral-ledger`/`decisions-log` singletons via idempotent-create + `add-item`/`set-slot`/`set-field`, finalize promotes each to its own `location:` subdir byte-stable; run 2 warm-re-creates (copy-in preserves the prior milestone, provenance `edited-from-base`), appends, and re-promotes byte-stable with **both** runs present. The roadmap milestone entry authors **both** the `proves` and the prose `decomposition` slots (the two-slot repeatable-item case).
3. **The conformance gate fires on a real empty leaf.** A half-authored roadmap entry — `proves` authored, `decomposition` genuinely empty — **blocks** finalize at `schema-conformance.required-slot-present` naming the empty `decomposition` slot, non-zero exit with no commit (Increment 1's per-item conformance over this schema's leaves, the vacuously-green-managed-doc-gate guard).
4. **`planning` is off-router.** Bare `jigc start` lists no `planning` catalog line and the `--format json` `workflows` array carries no `planning` id — `selectable: false`, the M8 catalog-leak class, asserted on the live front-door bytes.
5. **A drifted warm-append conflict-blocks, never silently merges.** With the committed baseline recorded by run 1's finalize, an out-of-band edit to the committed roadmap (DRIFTED) plus a warm task that touches it (TOUCHED) **conflict-blocks** at finalize-preflight with `reconciliation.conflict-block`, non-zero exit, no commit (the storage-is-human-editable invariant: OOB conflicts route to a human, never silent merge).
6. **The honest bound is observed, not over-claimed.** The flow proves jigc *composes* the single-agent planning spine and *maintains* its three running docs on **fresh instances**; the fanned recon, the independent design review, and jigc managing its *own* historical two-level roadmap stay out of scope ([methodology-docs.md](methodology-docs.md#what-m16-proves-and-what-it-does-not)). The build-honesty watch holds: gap-detect and Settle stay agent-judgment prose, never a gap-count lint (the `methodology_honesty_artifact` bar).

## 20. Completion encode — jigc composes its own milestone-completion spine and the #5 owner-artifact gate (M16)

The M16 completion half ([methodology-docs.md](methodology-docs.md#acceptance-flows) → Acceptance flows, Flow 20): jigc composes its **own milestone-completion workflow** (audit → triage → fix → re-verify) as a methodology-pack definition, **create-fresh authors** the per-milestone `completion-record` (the audit `verdict` + the `owner-artifact` owned-location path + the triaged `findings`), **appends** the running `decisions-log` with the triage decisions, and `finalize` **promotes the owner-artifact in the same transaction** and runs the intrinsic **#5 owner-artifact presence gate**. With [flow 19](#19-planning-encode--jigc-composes-its-own-milestone-planning-spine-and-maintains-its-running-docs-m16) (the planning half) this closes the self-hosting *machinery* loop. The doctype shape (the `meta` header's `verdict`/`owner-artifact`, the repeatable `findings`), the create-fresh `id-from: title` decision, and the engine deltas (the #5 presence gate, repeatable-section conformance) are the design of record in [methodology-docs.md](methodology-docs.md) and are not restated here. Notation illustrative.

The `completion` workflow is **`creates-task: true, selectable: false`** — the same `sub-task` precedent as `planning` ([methodology-docs.md](methodology-docs.md#the-authoring-spine--creates-task-true-selectable-false-the-sub-task-precedent)): it **mints a task** (so the record authoring + the append ride the proven create-gate + finalize-promote spine) yet ships **no `when`** and stays out of the methodology pack's `default-workflow.of` enum, so it is **off-router**. It declares `allows-create:` the per-milestone `completion-record` (`as: record`) and the running `decisions-log` (`as: log`).

**The honest bound (per [methodology-docs.md](methodology-docs.md#what-m16-proves-and-what-it-does-not), the single-agent-spine bound — sharper than M15).** The real milestone-completion loop is **multi-agent orchestration** — parallel `milestone-code-reviewer`/`milestone-e2e-tester` audits plus per-finding fix agents. jigc composes the **single-agent linear spine** — the phases one agent walks, with the human-gated **triage** and **fix-round-cap** halts made structural `checkpoint` directives, and the #5 presence gate made a finalize check. The independent audit passes — and the **green/red verdict** they produce — stay **orchestration-level** (the build harness) and irreducible judgment: the spine *records* the verdict and findings, it does **not** certify them. The **build-time honesty watch** ([methodology-docs.md](methodology-docs.md#what-m16-proves-and-what-it-does-not), the `methodology_honesty_artifact` bar): the audit/triage phases stay **pure agent-judgment prose** — never a structured checklist or count threshold; the checkpoints **halt, they never score**. And the **#5 gate is presence, not judgment** — it asserts a file *exists* at the named owned path, never reads its bytes (a content-reading check would have crossed into the audit judgment the gate only brackets — the determinism boundary). It is a **backstop**: you cannot finalize a completion without recording *something* at the named path; it does not prove the audit was genuine — that stays the orchestrator's recorded responsibility ([methodology-docs.md](methodology-docs.md#the-engine-work-bounded-risk-first), item 3, the anti-vacuity statement).

### The walk — compose the completion spine, author the record, append the log, finalize

```text
$ jigc setup                                                # installs the methodology pack
$ jigc start --workflow completion "M16"                    # off-router, invoked directly; mints a task
  Audit the assembled milestone … {{task.intent}} = M16 …            ← step:audit (Reason prose, NOT a checklist)

  Checkpoint: triage-gate                                            ← the human-gated triage halt (M15 kind)
  > Triage each finding — verify it is real … the human gate fires
  > for exactly two cases: a fix too big for the lane, and a contested
  > finding. … The human owns this gate.

  Checkpoint: fix-rounds-exhausted                                   ← the fix-round-cap halt (bounded to three)
  > For each blocking finding, run a dev-workflow fix task … bounded
  > to three rounds. If blocking findings remain, stop and surface it.

  Re-verify — the full gate green AND re-run the affected audit slice … ← step:re-verify (Reason prose)
  Record the completion audit on the per-milestone completion-record … ← step:author-completion-record
  Record each triage decision on the running decisions log …          ← step:author-decisions
  — jigc · all writes through `jigc`.

# create-fresh the per-milestone record — `id-from: title` mints `completions/M16.md` (the proven adr/spec
# mint, NOT the running-doc copy-in). Author the `meta` header (the audit verdict + the owner-artifact path):
$ jigc doc create completion-record --title "M16"           # → completion-record:m16  (mints; create-fresh)
$ jigc doc set-field completion-record:m16#meta/verdict        --value green
$ jigc doc set-field completion-record:m16#meta/owner-artifact --value completions/artifacts/M16/audit.md
#   the owner-artifact MUST live under the owned home `completions/artifacts/<milestone>/` — write the genuine
#   -audit file there + `git add` it (the orchestrator's same-transaction recording), then name the path here.

# one triaged finding per audit finding — title + severity/disposition enums + an `evidence` STRING
# (a plain `set-field` string: the file:line / repro / trace — NOT a managed ref, the methodology pack
# composes alone, so an adr/spec target would point outside the composed schema universe):
$ jigc doc add-item completion-record:m16#findings --title "A confirmed finding"   # → …#findings/<id>
$ jigc doc set-field <item>/severity    --value blocking
$ jigc doc set-field <item>/disposition --value fixed                  # fixed / deferred / contested
$ jigc doc set-field <item>/evidence    --value engine/src/foo.rs:42

# the both-halves driver — append the triage decisions to the RUNNING decisions-log singleton (create-or-update:
# safe whether or not it exists; an existing committed log is copied-in for append, NOT clobbered):
$ jigc doc create decisions-log --title "Decisions-Log"
$ jigc doc add-item decisions-log:decisions-log#entries --title "chose to fix-now the finding"
$ jigc doc set-slot <entry>/why --from-file -

$ jigc task finalize m16
> validate: clean (the #5 owner-artifact.present gate satisfied — the artifact is durably staged)
> promote: completions/m16.md · completions/artifacts/M16/audit.md · decisions-log/decisions-log.md
> commit:  chore(completion): close the milestone   ← one commit: the record, the owner-artifact, the log, the code
```

The `completion-record` promotes to its own `completions/` `location:` subdir, the owner-artifact lands under `completions/artifacts/M16/`, and the appended `decisions-log` re-promotes to `decisions-log/` byte-stable — all in **one commit** (the writes-are-transactional-at-finalize invariant). The audit `verdict` is an **authored** field: a **`red`** verdict finalizes exactly as readily as a `green` one (the engine records the agent's judgment, it does not certify the audit — the single-agent-spine bound).

### The #5 gate — the owner-artifact presence assertion at finalize

The gate is an **intrinsic** (floored-blocking) finalize-time check on the completion task: it reads the `meta/owner-artifact` field, asserts the named path is **safe** (repo-relative, under the owned home, no `..`/symlink escape) and **durably present** (staged/committed), and emits the `owner-artifact.present` finding when it is not. It **never reads the artifact's bytes** — presence, not content ([methodology-docs.md](methodology-docs.md#the-engine-work-bounded-risk-first), item 3).

```text
# PASS — the artifact is durably staged under the owned home (written + `git add`ed BEFORE finalize):
$ git add completions/artifacts/M16/audit.md                # the same-transaction recording, made explicit
$ jigc task finalize m16
> validate: clean   (owner-artifact.present satisfied)      ← finalize lands; the gate is silent
```

### The reds — the gate fires-and-blocks on a non-durable recording, not a vacuous path-exists

```text
# RED a — ABSENT: a well-shaped owned-home path whose file was never written:
$ jigc doc set-field completion-record:m16#meta/owner-artifact --value completions/artifacts/M16/audit.md
$ jigc task finalize m16                                     # the file at that path does not exist
> BLOCK owner-artifact.present @ completion-record:m16 … completions/artifacts/M16/audit.md   # non-zero, NO commit

# RED b — UNSAFE: an absolute path (or `..`/symlink escape) the gate rejects outright:
$ jigc doc set-field completion-record:m16#meta/owner-artifact --value /etc/passwd
$ jigc task finalize m16
> BLOCK owner-artifact.present @ … /etc/passwd  (not a repo-relative owned-home path)        # non-zero, NO commit

# RED c — PRESENT-BUT-UNTRACKED: the file exists on disk under the owned home but is NOT `git add`ed
#   (presence on disk ≠ a durable recording — the M3 lesson: a file not git-tracked before validate is
#   not durably recorded, so the PASS path's `git add` is load-bearing, not incidental):
$ jigc task finalize m16
> BLOCK owner-artifact.present @ … completions/artifacts/M16/audit.md  (present, untracked)   # non-zero, NO commit
```

Each red exits finalize **non-zero** with **no commit**, surfacing `owner-artifact.present` — proving the gate **fires on a real non-durable recording**, not a vacuous always-pass (the *vacuously-green-managed-doc-gate* guard; the present-but-untracked case is the anti-vacuity teeth that prove the PASS path's staging is genuinely load-bearing).

### What it asserts (the acceptance bar)

1. **jigc composes its own completion spine.** `jigc start --workflow completion "<milestone>"` mints a task (off-router) and composes the audit → triage → fix → re-verify phase walk, with **triage** and the **fix-round cap** emitted as structural `Checkpoint: triage-gate` / `Checkpoint: fix-rounds-exhausted` directives (the M15 `checkpoint` kind), driven from the real binary's emitted bytes.
2. **The per-milestone completion-record is create-fresh authored.** `jigc doc create completion-record --title "<milestone>"` mints `completions/<milestone>.md` (`id-from: title`, the proven mint — *not* a running-doc copy-in); the agent authors the `meta` header's `verdict` + `owner-artifact` and one `findings` entry per triaged finding (`severity`/`disposition` enums + a plain-string `evidence`, never a managed ref), and finalize promotes the record to `completions/` byte-stable.
3. **The #5 owner-artifact gate fires-and-blocks on a non-durable recording.** Finalize **blocks** with `owner-artifact.present` (non-zero exit, no commit) when the named path is absent, unsafe (absolute / `..` / symlink escape), or present-but-untracked, and **passes** when the artifact is durably staged under `completions/artifacts/<milestone>/` — the gate asserting presence, never the artifact's bytes (the determinism boundary).
4. **The decisions-log is appended by this half too.** A triage decision rides into the running `decisions-log` via create-or-update + `add-item`/`set-slot`, re-promoting `decisions-log/decisions-log.md` in the same commit — flow 20 is a both-halves driver of the running singleton.

## 21. Measured run — capture live on a twin, seeds planted mid-run, the record authored through the binary (M17)

The M17 measured-run arc ([measurement.md](measurement.md) → Acceptance flows, Flow 21): a twin with the capture apparatus on and a pinned binary does real work through the methodology, both **seeded failures** plant mid-run (after the first promoting finalize, by construction — a fresh twin has no committed managed doc to edit), the facts are assembled from the capture, and the `record-dogfood` workflow authors the per-run `dogfood-record` **through the binary** (the recording itself dogfoods the machinery) — finalize promoting record + owner-artifact in one commit. The **fact definitions** (units, numerators, the dedup rule), the **doctype shape**, and the **seeded-failure obligation** are the design of record in [measurement.md](measurement.md#the-three-facts--operational-definitions) and are not restated here; the **apparatus mechanics** (the hook set, the pinned v1 JSONL log schema, the tally rules) live at [implementation/dogfood/README.md](../implementation/dogfood/README.md). Notation illustrative; the spine is sealed end-to-end through the real binary by `crates/cli/tests/flow21_measured_run.rs` — composed authoring lines extracted and run verbatim, the real hook logger and real tally closing the capture loop.

The `record-dogfood` workflow is **`creates-task: true, selectable: false`** — the [flow 19](#19-planning-encode--jigc-composes-its-own-milestone-planning-spine-and-maintains-its-running-docs-m16)/[20](#20-completion-encode--jigc-composes-its-own-milestone-completion-spine-and-the-5-owner-artifact-gate-m16) authoring-spine precedent: it **mints a task** (so the record rides the proven create-gate + finalize-promote spine) yet ships **no `when`** and stays out of the methodology pack's `default-workflow.of` enum — **off-router**, invoked directly when a measured run completes. It declares `allows-create: [{type: dogfood-record, as: record}]`.

**The honest bounds (per [measurement.md](measurement.md#the-capture-substrate), stated here because the walk leans on them).** The record's fact fields are **typed-but-transcribed**: collection is mechanical (hooks + emissions), form is engine-validated (`int`/enum), the *values* are transcribed by the recording agent — auditable against the committed raw hook log in the owner-artifact, never CLI-sourced. And the honesty watch extends to this workflow: the `judgment` slot stays **pure prose** and the `verdict` stays **authored** judgment over the counts — never a scoring lint, never computed (the engine never opines on whether jigc is helping; an engine opinion on the thesis would be the thesis inverted).

### The walk — apparatus on, the seeds, the tally, the record

```text
# twin + apparatus on + pinned binary (implementation/dogfood/README.md → Install):
$ export JIGC_DOGFOOD_HOME=<jigc-checkout>/implementation/dogfood
$ export JIGC_DOGFOOD_LOG=/tmp/jigc-dogfood/pilot/hook-log.jsonl    # the log lives OUTSIDE the twin repo
#   merge hooks.json into the twin's .claude/settings.json; record the pinned binary's sha
$ jigc setup                                                        # installs the methodology pack

# real task work through the methodology — the FIRST PROMOTING FINALIZE lands the committed managed docs:
$ jigc start "…"                                                    # dev-task … finalize lands, docs promote

# mid-run seed 1 — the OOB edit: a conformant edit on a now-COMMITTED managed doc (target class: the run's
# promoted methodology singleton), made OUTSIDE the observed tools (human/sed in Bash, not Write/Edit, not
# jigc — pinning the seed to the absorb channel):
$ sed -i 's/…/…/' decisions-log/decisions-log.md
# the next LANDED finalize emits reconciliation.absorb in its envelope EXACTLY ONCE and advances the
# file-state baseline; a subsequent validate sights ZERO absorb (the re-fire defect stays fixed):
$ jigc task validate <next> --format json | jq '[.findings[] | select(.code=="reconciliation.absorb")] | length'
> 0

# mid-run seed 2 — the bad-finalize attempt: a staged integrity violation on a later task; finalize must
# block with the validation-blocked exit code (3 — distinct from operational error 1, usage error 2):
$ jigc task finalize <staged>; echo $?
> BLOCK …                                                           # non-zero, NO commit
> 3

# remaining work … then assemble the facts from the capture (one --managed-prefix per active location: dir):
$ python3 "$JIGC_DOGFOOD_HOME"/tally.py "$JIGC_DOGFOOD_LOG" --managed-prefix decisions-log/ --managed-prefix dogfood/ …
> { "adapter-writes": …, "oob-edits": …, "drift-caught": …, "validate-blocks": …, … }   # totals + per-event detail;
#   raw invocation counts ride as telemetry, never the headline; seeded attribution is transcription protocol —
#   the orchestrator attributes the seeded entries against the run protocol, the tally never opines.

# record the run — off-router, through the binary:
$ jigc start --workflow record-dogfood "Pilot Run"                  # mints a task; composes:
  Record the measured run on the per-run dogfood-record …                ← step:author-dogfood-record
  Run: jigc doc create dogfood-record --title <…> --task <id>            ← {{cli.create-dogfood-record}}
  jigc doc set-field dogfood-record:<slug>#meta/case … /binary-sha …     ← the run identity
  jigc doc set-field … the eight ORGANIC fact ints …                     ← transcribe the tally VERBATIM
  jigc doc set-field … /seeded-oob … /seeded-blocks …                    ← each must be exactly 1
  jigc doc set-field … /verdict … /owner-artifact …
  jigc doc set-slot  dogfood-record:<slug>#judgment …                    ← prose: what the counts mean
  Run: jigc task finalize <id>                                           ← step:finalize

$ jigc doc create dogfood-record --title "Pilot Run" --task <id>    # → dogfood-record:pilot-run (create-fresh
#   `id-from: title` mint to `dogfood/` — NOT `completions/`, the shared-location cross-block fact)
# … transcribe the facts; write the owner-artifact (transcript + raw hook log/tally exported UNCHANGED +
#   a content-hash manifest) under the owned home, then stage it durably:
$ git add completions/artifacts/pilot-run/capture.md
$ jigc task finalize <id>
> validate: clean (owner-artifact durably staged)
> promote: dogfood/pilot-run.md · completions/artifacts/pilot-run/capture.md
> commit: …                                                         ← ONE commit: the record + the artifact
# … export: record + artifacts copied as plain committed files into jigc's repo under
#   completions/artifacts/M17/<case>/ (a copy, not a conversion — the storage invariant).
```

### The reds — the red obligations of measurement.md's flow 21

```text
# RED 1 — the seeded edit counts EXACTLY once post-dedup: the corroborating absorb sightings across
#   blocked + landed envelopes collapse to one (path × finalize window) event in the tally.
#   Zero = the substrate is blind; two+ = the re-fire defect lives or the dedup rule fails. Either FAILS THE RUN.

# RED 2 — the seeded block keys via exit 3 on FINALIZE (a validate exit 3 keys the paired, non-headline
#   validate-blocks count instead — previewing is the designed loop, not a caught escape).

# RED 3 — a half-transcribed record BLOCKS at finalize (the run cannot land with a fact missing):
$ jigc task finalize <id>                                           # oob-edits never transcribed
> BLOCK schema-conformance.required-field-present @ dogfood-record:pilot-run … #meta/oob-edits   # exit 3, NO commit

# RED 4 — the omitted owner-artifact BLOCKS (the flow-20 #5 gate, on this doctype's owned-location field):
$ jigc task finalize <id>                                           # fact fixed; owner-artifact never set/staged
> BLOCK … owner-artifact …                                          # exit 3, NO commit — names ONLY the artifact
#   (the absent/unsafe/present-but-untracked reds are the gate's own — flow 20, not restated here)
```

### What it asserts (the acceptance bar)

1. **`record-dogfood` composes off-router through the real binary.** `jigc start --workflow record-dogfood "<run>"` mints a task and composes the authoring spine (create-fresh mint → identity → organic facts → seeded checks → verdict/owner-artifact → judgment) + the `step:finalize` tail; bare `jigc start` lists no `record-dogfood` catalog line and the `default-workflow.of` enum carries no `record-dogfood` (the M8 catalog-leak class, asserted on the live front-door bytes).
2. **The seeds follow the first promotion by construction and register exactly once each.** The seeded OOB edit (outside the observed tools, pinned to the absorb channel) yields `reconciliation.absorb` exactly once in the next landed finalize's emitted envelope, advances the baseline (zero re-fire at a subsequent validate), and tallies as one post-dedup (path × window) event; the seeded bad finalize exits **3**.
3. **The record cannot land half-recorded.** Finalize blocks (exit 3, no commit) on a missing fact field — `required-field-present` naming the field — and on the omitted owner-artifact; fully authored + durably staged, **one** landed commit promotes the record to `dogfood/` and the artifact under `completions/artifacts/`.
4. **The real capture loop closes.** Every jigc invocation rides through the real hook logger (`log-event.py`, the pinned v1 JSONL), and the real `tally.py` derives the facts — logical mutations grouped per (doc address × finalize window), invocations as telemetry never headline, exit-3 keying splitting finalize from validate.
5. **The honesty watch holds.** The `judgment` slot stays prose and the `verdict` stays authored — no scoring lint, no engine opinion on the thesis; the seeded counts live in their own `seeded-*` fields and never enter the organic facts.

## 22. Three-arm comparison — the pre-registered thesis protocol (M17, sessions pending)

The M17 thesis comparison ([measurement.md](measurement.md#the-comparison-protocol-pilot-only--three-arms) → The comparison protocol + Acceptance flows, Flow 22). This flow is **authored protocol, not a recorded run**: the comparison sessions themselves are the milestone's P1 protocol phase, executed *after* the flow-21 apparatus lands — **every line below is illustrative by construction; no session has run**. The arm definitions, the pre-registration rule, the counting protocol, and the judge rubric are the design of record in [measurement.md](measurement.md#the-comparison-protocol-pilot-only--three-arms) and are not restated here.

### The protocol walk (illustrative — sessions not yet run)

```text
# 0 · pre-registration — WRITTEN FIRST, into the comparison artifact (the carryover mitigation: the
#   orchestrator learns the task from arm 1 and would otherwise unintentionally improve arm 2; with n=1
#   that can dominate): the exact intent text · the acceptance checks · the allowed human interventions ·
#   the stop condition · the judge rubric. Arm order then fixed by coin-flip equivalent and RECORDED.
#   Arm C's frozen static-methodology CLAUDE.md is curated ONCE, before any arm runs — itself a recorded artifact.

# 1 · three twins from ONE pinned baseline commit of gherrink-galey (the originals never touched):
#   A — jigc:               `jigc setup` + the methodology pack (the 4-line CLAUDE.md delta — A and B
#                            differ by precisely the adapter)
#   B — control:             galey's own existing static CLAUDE.md, untouched (the product baseline)
#   C — static-methodology:  the methodology content hand-frozen into a static CLAUDE.md, no jigc —
#                            isolating dynamic composition + the write channel from methodology CONTENT
#   the SAME hooks run on every arm — the Write|Edit channel is the one observation identical across arms.

# 2 · the matched intent — the same pre-registered REAL galey task, issued identically to all arms; same
#   model, same session shape, one arm per session. The jigc arm runs it as ONE dev-task workflow (the
#   closest session-shape match, so ceremony cost compares like-for-like) — the full-methodology spine is
#   the surrounding pilot run (flow 21), measured separately, NOT part of the comparison.

# 3 · comparison-arm counting — PROTOCOL-COUNTED, labeled as such: defects reaching the commit = post-hoc
#   review of each landed commit against the pre-registered acceptance + the project's own gate, counted
#   by the judge; plus each arm's hook log (Write|Edit record).

# 4 · the rubric'd judgment — the human (LLM-judge assist permitted) applies the pre-registered rubric:
#   correctness of the landed change · doc/commit-message quality · drift left behind · ceremony cost
#   (counted on EVERY arm — jigc's overhead is data, not noise). The verdict must name AT LEAST ONE
#   NON-SEEDED thesis observation — a gate passed on apparatus-planted events alone has validated the
#   instrument and measured nothing (the pilot-gate criterion).

# 5 · the artifacts — the comparison artifact (pre-registration + arm transcripts + counts + the rubric'd
#   judgment) and the jigc arm's dogfood-record carrying the comparative verdict in its judgment slot.
#   The comparison arms get NO dogfood-record — no jigc in their loop makes the jigc facts N/A, not zero;
#   their counts live in the comparison artifact's prose (measurement.md → the comparison-arm asymmetry).
```

### What it asserts (the acceptance bar — judged at the pilot gate, not by the engine)

1. **Pre-registration precedes any arm.** Intent, acceptance, interventions, stop condition, and rubric are committed to the comparison artifact before a single arm session starts; arm order is recorded.
2. **Three arms isolate the variable.** Without arm C, an A-beats-B result cannot say whether *jigc* or merely *written-down methodology* did the work — C separates dynamic composition + the write channel from methodology content.
3. **The verdict names ≥1 non-seeded thesis observation** — an organic event or judged difference, never an apparatus-planted one. With flow 21's substrate checks, this is what unlocks cases 2+3 at the pilot gate.
4. **The honest bounds ride along, named.** The judge is unblinded (transcripts self-identify; rubric + recorded arm order + committed hook logs are the mitigations); control-arm counts are protocol-counted and labeled; n=1 per case — a structured pilot study, not a statistics claim ([measurement.md](measurement.md#honest-bounds)).
5. **The verdict is recorded, not certified.** A `red` verdict finalizes exactly as readily as a `green` one — the engine promotes the authored judgment without adjudicating it; the genuine green/red audit stays orchestration-level (the single-agent-spine bound, the build-honesty watch — the checkpoints halt and the gate asserts presence, neither scores).

## 23. compose-both-at-setup — the new-project methodology default (M21)

The M21 acceptance: **a fresh project gets the methodology pack composed *with* dev on a clean `cargo install` — no staged pack tree, no hand-written `packs.yaml`.** Where flow 17 composes a **`FilesystemPack` (absolute path)** that the operator stages and lists by hand, M21 embeds the methodology tree **in the binary** as a second `EmbeddedPack` and has `jigc setup` wire it by writing one marker. The result is the **dev-primary** composition: the embedded `[dev ▸ methodology]` pair composed **dev-highest**, the *inverse* of flow 17's listed-pack-highest ordering. The design of record is [multi-pack.md](multi-pack.md#embedded-second-pack--setup-auto-wiring-m21); the embed/extract mechanism, the marker semantics, and the dev-highest precedence table live there and are not restated. Notation illustrative.

### Setup — `jigc setup` writes the marker; both packs ride in-binary

```text
# a fresh, unconfigured project — nothing staged, no JIGC_PACK_DIR:
$ cd /tmp/fresh-project && git init -q && git add -A && git commit -qm "baseline"

$ jigc setup                                   # writes the .jigc/config/ cascade AND the compose marker:
$ cat .jigc/config/packs.yaml
  compose-embedded-methodology: true           # the marker — no `packs:` list, no path (both packs are in-binary)
$ git add -A && git commit -qm "chore: jigc setup"

# absent the marker the pack-set is exactly [dev] — byte-identical to today (the single-pack floor, flow 17 assertion 1).
# the marker is the ONLY thing that fires the two-embedded-pack composition.
```

### The walk — dev wins the collisions; methodology's distinct surfaces co-compose

```text
$ jigc start --explain | grep -E "Pack:|collision"
  Pack: dev/0.0.0 ▸ methodology/0.1.0          ← dev-highest (the inverse of flow 17's methodology-first header)
  collision: default-workflow → won by dev     ← dev's knobs win → default-workflow = router
  collision: doctype:commit    → won by dev     ← composed commit KEEPS implements→spec (methodology's is a subset)

# because dev wins `default-workflow`, a bare `jigc start` lands on the ROUTER selection menu (the MVP on-ramp),
# NOT methodology's dev-task — contrast flow 17, where methodology wins and a bare start composes dev-task:
$ jigc start                                   # → the router catalog (project-setup / single-task / …)
  # the milestone half of the on-ramp is orientation route-prose, NOT the router menu (planning is off-catalog) —
  # see multi-pack.md → Embedded second pack; project-setup.md → Flow 2 hardening.

# methodology's DISTINCT surfaces co-compose (union, not shadowed) — they work out of the box:
$ jigc doc create roadmap --title "v1 roadmap"           # methodology's roadmap doctype mints
$ jigc start --workflow planning "plan milestone M1"     # methodology's planning workflow composes (off-router, by name)

# determinism holds under composition (the bar every flow asserts):
$ jigc start --task <id> | diff - <first-capture>        # empty diff
```

### What it asserts (the acceptance bar)

1. **The marker is the whole on-ramp — and absent, the floor is byte-identical to today.** `jigc setup` writes `compose-embedded-methodology: true`; the factory reads that key pre-cascade (the existing `discover_pack_list` walk, no new discovery) and composes `[dev ▸ methodology]` from the two in-binary trees. With the marker absent the pack-set is exactly `[dev]` — every dev-only test and the flow-17 single-pack byte-identity floor stay green untouched ([multi-pack.md](multi-pack.md#embedded-second-pack--setup-auto-wiring-m21)).
2. **Dev wins both collisions — the new-project default ordering, the same mechanism as flow 17 with opposite precedence.** `default-workflow → router` (so a bare start lands on the selection menu, the MVP on-ramp) and `doctype:commit` keeps `implements→spec` (a spec-driven task can still bind the edge), each surfaced in `--explain` as `won by dev`. Flow 17's **methodology-primary, listed-pack-highest** ordering and this **dev-primary, embedded-pair** ordering are **both valid compositions of the one precedence mechanism** — only the order differs, a deliberate per-configuration choice. Methodology-primary stays the self-hosting dogfood's ordering ([flow 15](#15-self-hosting--the-methodology-pack-dogfooded-on-a-fresh-project-m12-exploratory)), unchanged.
3. **Methodology's distinct surfaces co-compose — roadmap + `planning` work out of the box.** `roadmap`/`planning`/`decisions-log`/… don't collide with dev, so they all compose under the union; `jigc doc create roadmap` and `jigc start --workflow planning` succeed on a fresh project with no pack staging. (`planning` is **off-router** by design, so it is reached by name, not from the bare-start menu.)
4. **`JIGC_PACK_DIR` stays the explicit override and supersedes the marker.** The marker is the *new-project default* channel; `JIGC_PACK_DIR` remains the explicit/dogfood channel and **wins** when both are present (the T2a precedence — [flow 15](#15-self-hosting--the-methodology-pack-dogfooded-on-a-fresh-project-m12-exploratory) note), so the methodology-alone / methodology-primary path is untouched by M21.

## 24. Changelog authoring — cold-create → warm-append a multi-level singleton (doctype-expansion)

The doctype-expansion arc ([changelog.md](changelog.md) → Acceptance): a **new** project authors a managed `changelog` singleton *through* jigc — the first dev-pack doctype with a **nested repeatable** (release → change-group), cold-created then warm-appended a new release byte-stable, the same two-run shape as [flow 19](#19-planning-encode--jigc-composes-its-own-milestone-planning-spine-and-maintains-its-running-docs-m16) now one level deeper. The doctype shape, the singleton/edge-free decisions, the driver, and the four engine lifts are the design of record in [changelog.md](changelog.md) and are not restated here. Notation illustrative; the exact verb sequence is **spiked against the real binary at build** (the multi-level authoring path is net-new engine).

The `record-change` workflow is **`creates-task: true, selectable: false`** (the off-router authoring spine, [methodology-docs.md](methodology-docs.md#the-authoring-spine--creates-task-true-selectable-false-the-sub-task-precedent)), dev-pack, `allows-create: [{type: changelog, as: changelog}]`. The everyday `single-task` also gains `{type: changelog, as: change}` beside its `{type: adr, as: decision}`, so a coding task appends an unreleased entry through the same create-gate (the fold-in — not walked here, asserted in bar 5).

### The walk — cut a release, then warm-append a second (run 1: cold-create)

```text
$ jigc setup                                                  # dev pack (changelog ships in it)
$ jigc start --workflow record-change "cut 1.0.0"            # off-router, by name; mints a task
  Scope the change …                                                 ← step:scope (Reason prose)
  Author the changelog — create-or-update the singleton, add the release + its change-groups …  ← step:author-change
  — jigc · all writes through `jigc`.

# COLD (no committed changelog yet): idempotent-create mints the fixed-slug singleton (singleton ⇒ slug == type id).
$ jigc doc create changelog --title Changelog               # → changelog:changelog.md (mints; cold)

# add a release item (the level-1 repeatable; `date` is item-level set:on-create → stamped automatically).
# add-item PRINTS the minted address — the slugger drops dots, so `1.0.0` mints id `100`. Drive the
# PRINTED address verbatim downstream (never re-spell the version):
$ jigc doc add-item  changelog:changelog#releases --title "1.0.0"       # → changelog:changelog#releases/100
$ jigc doc set-field changelog:changelog#releases/100/link --value https://example.com/compare/0.9.0...1.0.0  # OPTIONAL field — may be omitted

# add NESTED change-groups under the release (the level-2 repeatable — the Leaf::Repeatable path).
# the minted group address is SECTION-QUALIFIED (carries the `changes` segment) — the form set-slot accepts:
$ jigc doc add-item  changelog:changelog#releases/100/changes --title added    # → changelog:changelog#releases/100/changes/added
$ jigc doc set-slot  changelog:changelog#releases/100/changes/added/notes --from-file -   # "- OAuth device-code flow"
$ jigc doc add-item  changelog:changelog#releases/100/changes --title fixed    # → …/releases/100/changes/fixed
$ jigc doc set-slot  changelog:changelog#releases/100/changes/fixed/notes --from-file -

$ jigc task finalize cut-100
> validate: clean
> promote: changelog/changelog.md
> commit:  docs(changelog): cut 1.0.0           ← one commit, the singleton + the code
```

The `## Unreleased Changes` section (a **multi-word section id** — the multi-word-section-id fix, not a route-around) round-trips; the committed bytes are **byte-stable** against the staged promote source, nested change-groups included.

### Run 2 (a second release) — warm-append: idempotent-create copies-in, re-promotes byte-stable

```text
$ jigc start --workflow record-change "cut 1.1.0"
# WARM (changelog is committed): create copies the committed doc in (read_or_copy_in) — prior 1.0.0 release survives.
$ jigc doc create changelog --title Changelog               # copy-in: the 1.0.0 release is preserved
$ jigc doc add-item changelog:changelog#releases --title "1.1.0"               # → changelog:changelog#releases/110
$ jigc doc add-item changelog:changelog#releases/110/changes --title changed   # → …/releases/110/changes/changed
$ jigc doc set-slot changelog:changelog#releases/110/changes/changed/notes --from-file -
#   (no `link` authored this release — the OPTIONAL field is left absent)
$ jigc task finalize cut-110
> promote: changelog/changelog.md (re-promoted byte-stable, BOTH 1.0.0 + 1.1.0 present, nested groups intact)
```

### The reds — each fires on real input, not a fixture

```text
# RED 1 — a half-authored NESTED entry BLOCKS at validate (recursive repeatable conformance):
$ jigc doc add-item changelog:changelog#releases/110/changes --title removed   # → …/releases/110/changes/removed
#   leave the nested `notes` slot empty
$ jigc task finalize cut-110
> BLOCK schema-conformance.required-slot-present @ changelog:changelog … #releases/110/changes/removed/notes
#   the gate names the genuinely-empty NESTED leaf at its SECTION-QUALIFIED address; non-zero exit, NO commit.
#   (the nested-leaf address depends on the parent-scoped path locator — confirmed at the build spike, B1/S1.)

# RED 2 — the OPTIONAL `link` field, absent, finalizes CLEAN (the optional-field lift):
$ jigc task finalize cut-110     # with no `link` on 1.1.0 → no required-field-present finding for it
> validate: clean                  # an absent OPTIONAL field does not block; an absent REQUIRED field/slot still does
```

### What it asserts (the acceptance bar)

1. **A new project authors a managed `changelog` through jigc.** `jigc start --workflow record-change` mints a task (off-router), idempotent-create mints the `changelog` singleton, and finalize promotes it to `changelog/changelog.md` byte-stable — Flow-A authoring reach extended to a real-project doc.
2. **The nested repeatable round-trips (the `Leaf::Repeatable` lift).** A release item carries a nested `changes` repeatable of change-groups (`#### Added`/`#### Fixed`), authored via `add-item`/`set-slot` on the extended `#section/item/subitem/leaf` address, rendered at the depth grammar (`##`/`###`/`####`), round-tripping byte-stable cold and warm.
3. **Maintained over time across two runs.** Run 1 cold-creates and cuts 1.0.0; run 2 warm-re-creates (copy-in preserves 1.0.0, provenance `edited-from-base`), appends 1.1.0, and re-promotes byte-stable with **both** releases and their nested groups present.
4. **The conformance gate fires on a real empty NESTED leaf**, and the **optional `link` field, absent, finalizes clean** (with no stray fields-block line) — recursive `required-slot-present` over nested items + the `optional:` lift, each on real input (the vacuously-green-managed-doc-gate guard).
5. **The multi-word section id round-trips** (`## Unreleased Changes` — the parser fix, not a route-around), and **the `single-task` fold-in** (`allows-create:[{changelog, as: change}]`) lets an everyday task append an unreleased entry through the same create-gate it uses for an ADR.
6. **The honest bound is observed.** The flow proves **Flow-A authoring** of a new changelog on a fresh project; an existing foreign `CHANGELOG.md` is migrated by [flow 25](#25-changelog-migration--rewrite-a-foreign-changelogmd-to-conformant-shape-and-adopt-it-auto-migration-g1) (M23), and nesting beyond 2 levels is supported-but-unexercised.

## 25. Changelog migration — rewrite a foreign `CHANGELOG.md` to conformant shape and adopt it (auto-migration G1)

The auto-migration arc ([auto-migration.md](auto-migration.md) → Acceptance): an **existing** project's non-conformant foreign `CHANGELOG.md` is **rewritten into conformant `changelog` shape and adopted** — the **transform** arm flow 12's detect-and-route never had. The mechanism (the `jigc migrate` verb, the CLI-owned source seam, the review gate, the retire step, Framing A) is the design of record in [auto-migration.md](auto-migration.md) and is not restated here. Notation illustrative; the net-new surface (verb / seam / review gate / retire) is **red-proven at build**, the reused author + adopt spine is [flow 24](#24-changelog-authoring--cold-create--warm-append-a-multi-level-singleton-doctype-expansion)'s.

The pre-state is the honest one (gap-detector-verified, *not* what older docs claimed): a foreign `CHANGELOG.md` **at repo root** classifies `Unmanaged` — no finding, no route, no hook — so `jigc ingest` leaves it untouched. Only an explicit verb naming the path can reach it.

### The walk — migrate, review, approve, adopt

```text
$ cat CHANGELOG.md            # the foreign original — Keep-a-Changelog-ish, NOT canonical jigc shape
  # Changelog
  All notable changes to this project will be documented in this file.    ← preamble (no schema home → dropped)
  ## [1.2.0] - 2026-03-01
  ### Added
  - OAuth device-code flow
  ### Performance            ← a NON-KaC category (outside the `category` enum)
  - Cut cold-start 40%
  ## [1.1.0] - 2026-01-15
  ### Fixed
  - Token refresh race

$ jigc migrate CHANGELOG.md --as changelog      # the new verb: mints an OFF-ROUTER task, stages the foreign bytes
  Migrating CHANGELOG.md → changelog (task migrate-changelog) …
  Read the source below and author the canonical changelog through `jigc doc …`:   ← step:author-change
  {{source}}        ← the foreign content, surfaced by the CLI-owned source seam (deterministic context, read-only)
  — jigc · all writes through `jigc`.

# the LLM rewrites the foreign prose into canonical shape through the EXISTING write verbs (flow 24's spine):
$ jigc doc create changelog --title Changelog                                    # → changelog:changelog.md
$ jigc doc add-item  changelog:changelog#releases --title "1.2.0"                # → …#releases/120
$ jigc doc set-field changelog:changelog#releases/120/date --value 2026-03-01    # HISTORICAL date overwrites the on-create stamp
$ jigc doc add-item  changelog:changelog#releases/120/changes --title added      # → …/changes/added
$ jigc doc set-slot  changelog:changelog#releases/120/changes/added/notes --from-file -   # "- OAuth device-code flow"
#   the foreign `Performance` category is mapped by the LLM to a valid enum member (`changed`) — NOT coerced by the CLI:
$ jigc doc add-item  changelog:changelog#releases/120/changes --title changed     # → …/changes/changed
$ jigc doc set-slot  changelog:changelog#releases/120/changes/changed/notes --from-file -   # "- Cut cold-start 40%"
$ jigc doc add-item  changelog:changelog#releases --title "1.1.0"                # → …#releases/110  (… etc)

$ jigc task finalize migrate-changelog            # WITHOUT --approve: renders the diff, exits non-zero, commits nothing
> validate: clean
> REVIEW the migration before it commits:                       ← the review gate (a net-new finalize --approve gate)
    CHANGELOG.md (foreign)        →   changelog/changelog.md (canonical)
    [a fidelity diff: foreign source-seam bytes vs the task working-area rendered doc — what was preserved, reordered, DROPPED (the preamble is gone)]
  Approve? the strict parse guarantees STRUCTURE, never content-faithfulness — this is the only fidelity check.
$ jigc task finalize migrate-changelog --approve
> promote: changelog/changelog.md                               ← the canonical doc written at its home
> retire:  CHANGELOG.md (removed — the foreign original)        ← jigc's first byte-destructive op, gated by approval
> adopt:   changelog/changelog.md (parsed, conformance-gated, indexed, baselined)
> commit:  docs(changelog): migrate CHANGELOG.md to managed shape   ← write + retire + adopt, one transaction
```

After approval the foreign `CHANGELOG.md` is **gone**, `changelog/changelog.md` is the managed singleton, and a follow-up `jigc ingest` reports it `adoptable … (adopted)` — it is now tracked.

### The reds — each fires on real input

```text
# RED 1 — a rewrite whose category stays NON-CONFORMANT blocks at the strict gate (enum-on-id-from enforcement):
$ jigc doc add-item changelog:changelog#releases/120/changes --title Performance   # foreign category; mints id slug `performance`
$ jigc task finalize migrate-changelog --approve
> BLOCK schema-conformance.field-value-conformant @ …#releases/120/changes/performance/category
#   the `category` enum is now enforced even on the id-from field, compared by RE-SLUG: `Performance`→`performance` ∉ enum → BLOCK,
#   whereas a foreign `Fixed`/`Added` re-slugs to the valid member `fixed`/`added` and PASSES (no coercion by the CLI).
#   the finding names the item at its slug-cased address (exact code/leaf-suffix pinned at the build spike); non-zero exit, NO commit, NO retire.

# RED 2 — the human REJECTS the fidelity diff at the review gate:
$ jigc task finalize migrate-changelog          # (a conformant rewrite, but the human judges it lost content)
> REVIEW … Approve? → rejected
> task discarded — CHANGELOG.md left untouched, nothing adopted.
#   rejection is byte-safe: the foreign original survives, the managed doc is not written, the edge index untouched.
```

### What it asserts (the acceptance bar)

1. **The transform arm exists (Framing A).** `jigc migrate CHANGELOG.md --as changelog` reaches a root `Unmanaged` foreign file, surfaces its content via the CLI-owned source seam, and the LLM rewrites it through the existing write verbs — the determinism boundary intact (LLM proposes prose; the CLI strict-parses + places every structural act). No CLI fuzzy heading-mapping.
2. **Strict-parse + adopt iff conformant.** A conformant rewrite is parsed, conformance-gated, written at `changelog/changelog.md` byte-stable, and adopted (indexed + baselined); a non-conformant rewrite blocks and is never adopted (RED 1).
3. **The `enum` is enforced on the `id-from` field.** A foreign category outside the `category` enum (`Performance`) is **rejected** at conformance — the migration's "adopted iff conformant" guarantee is real, not hollow (RED 1). The LLM must map foreign categories to valid members.
4. **Historical dates survive.** A release's `date` is set from the foreign file's historical date (`set-field` overwrites the `set: on-create` today-stamp), not stamped to today.
5. **The review gate is the fidelity check, and it is byte-safe both ways.** Approval is required before commit; the foreign original is **retired only on approval** (the first byte-destructive op, CLI-owned, transactional); rejection leaves the foreign original untouched and adopts nothing (RED 2).
6. **The honest bounds hold.** Changelog only (**M25** generalizes to `adr`/`spec`/`prd`; **M24** first hardens this changelog reference — flow 26); the doc preamble / per-release summary / `[Unreleased]` compare-link have no schema home and are **dropped** (accepted, mostly boilerplate); a foreign changelog whose versions collide under dot-dropping slugify (`1.2.0`/`1.20`→`120`) **blocks loudly** (accept-and-block), never silently suffixed. The migration-quality measure (a small real corpus + round-trip-conformance / fidelity-acceptance / content-preservation) is the milestone's done-bar.

## 26. Changelog migration, hardened — the full real-repo live migration through the batch path (M24)

The M24 hardening of flow 25 ([auto-migration.md](auto-migration.md) → Hardening; [DECISIONS.md](../DECISIONS.md) → 2026-06-16 M24 planning). Flow 25 proved the migrate spine on a small synthetic file; flow 26 is the **done-bar** — the **full** live migration of two real, dateless repos (`project-delta` ≈ 12 releases, `project-gamma` ≈ 50, on throwaway clones), authored through the **declarative batch** so it isn't hundreds of round-trips, and measured (incl. **agent-call count**). The spine (verb / seam / review gate / retire / adopt) is flow 25's and is not restated; flow 26 exercises the seven hardening behaviours. Notation illustrative; each behaviour is red-proven at build.

### The walk — batch authoring at scale, no fabricated dates

```text
$ git clone ~/Projects/project-delta /tmp/jigc-m24-project-delta && cd /tmp/jigc-m24-project-delta
$ jigc setup && jigc ingest                 # root CHANGELOG.md classifies Unmanaged (left untouched)
$ jigc migrate CHANGELOG.md --as changelog  # mints OFF-ROUTER task `migrate-changelog` (NOT named `changelog` — #9b), stages bytes
  Read the source and author the canonical changelog in ONE payload via `jigc doc author`:
  {{source}}                                ← the 127-line foreign content, source seam, read-only

# the LLM authors the WHOLE changelog as ONE declarative payload — the CLI applies every leaf atomically (#1):
$ jigc doc author changelog --from-file - <<'EOF'
releases:
  - version: "1.4.0"            # NO date: key — the foreign source is dateless, so NONE is stamped (#6)
    changes:
      - category: changed       # foreign "Improvements" + "Changes" MERGED onto one member (#7) — two groups can't share an id
        notes: |
          - <<the merged prose the LLM mapped from both foreign categories>>
      - category: fixed
        notes: |
          - <<…>>
  - version: "1.3.2"            # … all 12 releases in one payload, ~1 agent call instead of ~80
    changes: [ … ]
EOF
> applied: create changelog + 12 releases + 27 change-groups + 27 notes slots — one staged buffer, byte-stable.

$ jigc task finalize migrate-changelog       # WITHOUT --approve → the review gate
> validate: clean
> REVIEW the migration before it commits:
>   CHANGELOG.md (foreign)  →  changelog/changelog.md (canonical)
>   [the raw fidelity diff, AND a structural release-delta summary (#5):]
>   ── source releases: 1.4.0, 1.3.2, 1.3.1, … (12)   rewrite releases: 1.4.0, 1.3.2, … (12)
>   ── source releases absent from the rewrite: (none)            ← the reviewer needn't eyeball 12 releases
> Approve? structure is guaranteed; content-faithfulness is your call.
$ jigc task finalize migrate-changelog --approve
> promote changelog/changelog.md · retire CHANGELOG.md · adopt · commit (auto-provisioned commit doc, #4)
>   docs(changelog): migrate CHANGELOG.md to managed shape        ← formulaic message, CLI-provided, NOT authored by the agent
> committed: D CHANGELOG.md  A changelog/changelog.md  A .jigc/config/…  A .jigc/.gitignore   ← the migration set + jigc's own tracked config; unrelated user WIP stays out — the general change-set scoping (finalize commits a scoped set, not a working-tree sweep), here the migration variant (#9a)
```

The committed `changelog/changelog.md` carries all 12 releases, dateless ones rendered **with no date line** (no fabricated history), byte-stable; `jigc ingest` reports it adopted. The project-gamma run is the same path at ~50 releases.

### The hardening reds — each fires on real input

```text
# #3 write-time enum reject — fires at the AUTHORING point, not finalize:
$ jigc doc add-item changelog:changelog#releases/140/changes --title Improvements   # ∉ enum
> BLOCK schema-conformance.field-value-conformant @ …/changes/improvements   ← rejected NOW (re-slug membership), not at finalize

# #2 item-field value reject at write time (the closed parity gap):
$ jigc doc set-field changelog:changelog#releases/140/date --value "March 2026"     # not ISO
> BLOCK schema-conformance.field-value-conformant — date "March 2026" is not an ISO date   ← at write, not finalize

# #1 remove-item retracts a mis-authored NESTED change-group (top-level remove_item couldn't reach it):
$ jigc doc remove-item changelog:changelog#releases/140/changes/changed     # removes the nested group + its notes
> removed.

# #8 in-location squatter now authors end-to-end (was the M23 clean-fail):
$ jigc migrate changelog/changelog.md --as changelog   # a non-conformant file AT the canonical path
> working area seeded BLANK over the occupied canonical path (source-path == destination) — authors clean, no Frankenstein doc.
```

### What it asserts (the M24 acceptance bar)

1. **Real-repo scale, measured.** The full project-delta + project-gamma changelogs migrate end-to-end; the four facts are recorded (round-trip conformance · fidelity-acceptance · content-preservation · **agent-call count**), the call-count cut being the batch path's payoff over per-leaf authoring.
2. **The declarative batch authors the whole doc atomically (#1)**, byte-stable, doctype-general, without touching the source seam (boundary intact: agent authors the payload, CLI places every leaf).
3. **Dateless sources don't fabricate history (#6)** — a release with no foreign date renders with no date, not today's.
4. **Write-time validation gives immediate feedback (#2/#3)** — id-from enum and item-field values are rejected at the authoring point, not deferred to finalize.
5. **`remove-item` retracts a mistake, including a nested change-group (#1)** — no whole-task discard needed.
6. **The migration commit doc is auto-provided (#4)**, the review gate carries a structural release-delta summary (#5), categories map-and-merge onto the enum (#7), the in-location squatter authors end-to-end (#8), and finalize commits only the migration set {promote + retire + jigc's own tracked config}, leaving unrelated user WIP out. Since M30 this is the **general** finalize contract — the change-set scoping commits a scoped set, never a working-tree sweep ([finalize.md](finalize.md) → Dirty-tree policy) — of which the migration's fixed set is one variant, not a migration-only special case (#9a).
7. **No new schemas, Framing A intact.** Every hardening fix is CLI/engine/guidance over the existing `changelog`/`commit` schemas; the strict parser stays the sole structural authority.

## 27. Migration generalized — a cross-referencing ADR corpus + a spec + a prd, one file at a time (M25)

The M25 generalization of the corrected changelog pattern ([auto-migration.md](auto-migration.md) → Generalizing to adr/spec/prd; [DECISIONS.md](../DECISIONS.md) → 2026-06-17 M25 planning) to the **location-bearing, multi-instance** doctypes. The spine (verb / source seam / review gate / retire / batch) is flows 25–26's and is not restated; flow 27 exercises the *shape differences* — **N-instance per-file migration**, **per-title-slug minting at the location**, **edge-wiring across a migrated set with the dependency-ordering contract**, the **write-time ref-shape check**, **doc-level date-suppression**, and the **prd repeatable-requirements** shape. The done-bar is the **real corpus** measured on the M23/M24 facts: a real **MADR/Nygard ADR** set (headline — with supersession chains), a **real-but-idiosyncratic spec**, and a **synthesized-and-labeled prd**. Notation illustrative; each behaviour is red-proven at build.

### The walk — migrate ADRs one file at a time, supersession targets first

```text
$ git clone <real-MADR-repo> /tmp/jigc-m25-adrs && cd /tmp/jigc-m25-adrs
$ jigc setup && jigc ingest        # docs/adr/0001-*.md … classify Unmanaged (left untouched, off-canonical path)

# ADR-0001 first (it is a supersession TARGET of 0007) — the ordering contract (one-file-one-task, sequential):
$ jigc migrate docs/adr/0001-use-mysql.md --as adr   # mints OFF-ROUTER task `migrate-adr-0001-use-mysql` (per-FILE id, not the singleton `migrate-adr`)
  Read the source and author the canonical ADR in ONE payload via `jigc doc author`:
  {{source}}                                          ← the foreign ADR content, source seam, read-only

$ jigc doc author adr --from-file - <<'EOF'
title: Use MySQL
sections:
  - id: status
    set:
      status: superseded          # foreign "Rejected"/"Deprecated" → mapped onto the 3-enum; here it was later superseded
      # NO date: key → the foreign ADR is dateless, so NO today-stamp is fabricated (doc-level date-suppression)
  - id: context
    set: { context: "<<why a datastore choice was needed>>" }
  - id: decision
    set: { decision: "<<we chose MySQL>>" }
  - id: consequences
    set: { consequences: "<<the tradeoffs>>" }
EOF
> applied: create adr:use-mysql + status header + 3 slots — one staged buffer, byte-stable.
$ jigc task finalize migrate-adr-0001-use-mysql --approve
> promote decisions/use-mysql.md · retire docs/adr/0001-use-mysql.md · adopt · commit   ← per-title slug at decisions/

# ADR-0007 supersedes 0001 — now 0001 is COMMITTED, so the forward-ref resolves against the store:
$ jigc migrate docs/adr/0007-use-postgres.md --as adr
$ jigc doc author adr --from-file - <<'EOF'
title: Use PostgreSQL
sections:
  - id: status
    set:
      status: accepted
      supersedes: "[adr:use-mysql]"     # BRACKET-list (0..* widened) — a real ADR may supersede several; comma form is the footgun
  - id: context
    set: { context: "<<…>>" }
  - id: decision
    set: { decision: "<<…>>" }
  - id: consequences
    set: { consequences: "<<…>>" }
EOF
$ jigc task finalize migrate-adr-0007-use-postgres --approve
> validate: clean — supersedes adr:use-mysql resolves in the committed store   ← edge integrity across the migrated set
> promote decisions/use-postgres.md · retire docs/adr/0007-use-postgres.md · adopt · commit
```

A spec and a prd migrate the same one-file-one-task way:

```text
$ jigc migrate docs/specs/auth.md --as spec      # → specs/auth.md ; criteria with NO maps-to-test finalize clean (optional anchor)
$ jigc migrate docs/PRD.md --as prd              # → prds/<title-slug>.md ; requirements author as a REPEATABLE section (per-requirement items)
$ jigc doc author prd --from-file - <<'EOF'
title: Product Brief
sections:
  - id: vision
    set: { vision: "<<the product vision>>" }
  - id: requirements                              # repeatable now — each foreign requirement is an individually-addressable item
    items:
      - title: Account signup
        set: { statement: "<<a user can create an account>>" }
      - title: Password reset
        set: { statement: "<<a user can reset a forgotten password>>" }
  - id: context
    set: { context: "<<constraints>>" }
EOF
```

### The reds — each fires on real input

```text
# write-time ref-shape check — a bare slug is rejected NOW, not at finalize with a misleading "resolves in neither":
$ jigc doc set-field adr:use-postgres#status/supersedes --value "use-mysql"   # missing the adr: type prefix
> BLOCK write.malformed-value — ref "use-mysql" must be of the form <type>:<slug> (expected type: adr)   ← immediate

# the ordering contract enforced by reality — supersede a NOT-YET-migrated sibling → forward-ref dangles:
$ jigc task finalize migrate-adr-0007-use-postgres          # 0001 not yet migrated/committed
> BLOCK schema-conformance.ref-resolves — target 'adr:use-mysql' resolves in neither the committed store nor this task's working area

# foreign status outside the enum must be mapped, not passed through:
$ jigc doc author adr --from-file - <<'EOF'   # status: Rejected   (∉ {proposed, accepted, superseded})
> BLOCK write.malformed-value — "Rejected" is not a member of enum "status"   ← guidance maps foreign status onto the 3-enum

# human-reject of the fidelity diff leaves the foreign original untouched (the spine's invariant, byte-safe):
$ jigc task finalize migrate-adr-0001-use-mysql           # WITHOUT --approve → renders the raw diff, exits non-zero, commits nothing
```

### What it asserts (the M25 acceptance bar)

1. **A real ADR corpus migrates end-to-end, one file at a time** — each foreign ADR → a per-title-slug managed doc at `decisions/`, retired + adopted, byte-stable; the per-file migration task id (`migrate-<doctype>-<slug>`) lets a corpus of N ADRs migrate sequentially (the singleton `migrate-<doctype>` id collision is fixed). Measured on round-trip conformance · fidelity-acceptance · content-preservation · agent-call count.
2. **Edge-wiring across the migrated set holds** — `adr.supersedes` (widened `0..*`, bracket-list) resolves against the committed store; the **dependency-ordering contract** (migrate + finalize a supersession target first) is what makes forward-ref integrity hold under the one-file-one-task model, and a forward-ref to a not-yet-migrated sibling **blocks** (no cross-task transaction machinery). A `supersedes` whose target is **outside the migration set** (a prior the operator won't migrate) is **dropped to prose**, never authored as a dangling ref.
3. **The write-time ref-shape check gives immediate feedback** — a bare slug or wrong-type ref is rejected at the authoring point as `write.malformed-value`, not deferred to a misleading finalize dangle.
4. **Dateless ADRs don't fabricate history** — the doc-level on-create date-suppression renders no date when the foreign source has none (the M24 item-level fix, now threaded to the doc-level header path adr's `date` uses).
5. **spec and prd migrate cleanly** — spec criteria without `maps-to-test` finalize clean (optional anchor); **prd requirements author as a repeatable section** (per-requirement items, pack-only — no `decomposes-into` edge).
6. **The honest bounds hold** — adr accepted-drops (Considered Options / Related ADRs / Deciders / out-of-enum status mapped-or-dropped); a **finalize-promote clobber-guard** blocks any create-provenance doc whose destination already holds a committed managed doc (covering the in-location squatter *and* a title-slug collision across the corpus); migrated titles render as the **slug in the H1** (`# use-mysql`, not `# Use MySQL`) — an accepted pre-existing bound, not data loss; prd's acceptance arm is a **synthesized, labeled** PRD; the review gate falls back to the raw foreign-vs-canonical diff (no per-doctype structural summary). Framing A intact throughout — LLM proposes prose, the CLI strict-parses + places every structural act.

## 28. Migration's last doctype — a foreign architecture doc with code anchors and decision citations (M26)

The M26 close of the migration arc ([auto-migration.md](auto-migration.md) → Generalizing to arch-doc; [DECISIONS.md](../DECISIONS.md) → 2026-06-18 M26 planning) — copying the proven pattern one final time to `arch-doc`, the last and hardest target because its repeatable `components` carry **both a `description` slot and an `implemented-by` code anchor**, and those anchors must resolve against real code. The spine (verb / source seam / review gate / retire / batch / clobber guard) is flows 25–27's and is not restated; flow 28 exercises arch-doc's *shape differences* — the **slot+field-group component** authored byte-stable, **doc↔code on a migrated doc** (the first migration to author a resolving code anchor), and **`cites → adr` across a migrated set**. The migration arm is **two pack yaml files, no engine/CLI/schema change**. M26 migrates a foreign architecture *document*; it does **not** infer one from code (the north-star, out). The corpus is **hybrid**: a real foreign arch-doc + a synthesized-over-jigc-Rust arch-doc. Notation illustrative; each behaviour is red-proven at build.

### The walk — migrate the real arm, then the code-checked synthetic arm

```bash
# --- Real arm: a foreign ARCHITECTURE.md (arc42/C4/README-section), non-Rust or no code pointers ---
$ jigc migrate docs/ARCHITECTURE.md --as arch-doc          # mints task migrate-arch-doc-architecture, surfaces the foreign bytes
# the LLM rewrites the foreign prose through ONE declarative batch (Framing A):
$ jigc doc author arch-doc --from-file - --task migrate-arch-doc-architecture <<'EOF'
> # overview slot ← foreign "Overview"/"Introduction"; components ← each module section
> # (a component with no foreign prose gets a SYNTHESIZED one-line responsibility — description is required)
> # implemented-by OMITTED where no concrete path#symbol exists; cites authored ONLY for in-store adrs
> EOF
$ jigc task finalize migrate-arch-doc-architecture --approve   # promotes architecture/<slug>.md, retires the foreign file, adopts byte-stable
                                                               # file-only anchors accepted (non-Rust → symbol-exists degrades to file-exists)

# --- Synthetic arm over jigc's own Rust: the code-checked mandate (C2) ---
# precondition: a committed adr to cite (jigc ships none) — migrate or create+finalize one first:
$ jigc start --workflow ... && jigc doc create adr --title "..." && jigc task finalize ... --approve   # → decisions/<slug>.md
$ jigc migrate fixtures/parser-subsystem.md --as arch-doc
$ jigc doc author arch-doc --from-file - --task migrate-arch-doc-parser-subsystem <<'EOF'
> # ≥2 components, each implemented-by a REAL, independently-resolving jigc symbol
> #   e.g. components/lexer → crates/engine/src/parse.rs#scan_blocks
> #        components/writer → crates/engine/src/write.rs#render_item_at
> # cites: [adr:<a-committed-jigc-adr>]
> EOF

# the BLOCKING red — doc↔code on a migrated doc, per-item disambiguation:
# (delete/rename component-A's anchored symbol — a FIXTURE/clone target, never a live jigc dep — leave B's intact)
$ jigc task finalize migrate-arch-doc-parser-subsystem --approve
> BLOCK doc-code.symbol-exists — anchor resolves to no symbol
>       location arch-doc:parser-subsystem#components/lexer/implemented-by   ← A's item address, NOT B's

# the cites red — a dangling decision citation blocks (committed-first ordering):
$ jigc task finalize migrate-arch-doc-parser-subsystem --approve   # cites an adr not yet committed
> BLOCK schema-conformance.ref-resolves — target 'adr:some-decision' resolves in neither the committed store nor this task's working area

# fix both (anchor a present symbol, cite a committed adr) → finalize promotes architecture/parser-subsystem.md, one docs(arch-doc): commit
```

### What it asserts (the M26 acceptance bar)

1. **A foreign arch-doc migrates end-to-end, hybrid corpus** — both a real foreign arch-doc (in-the-wild fidelity, anchors accepted file-only) and a synthesized-over-jigc-Rust arch-doc → a per-title-slug managed doc at `architecture/`, retired + adopted, byte-stable. Measured on round-trip conformance · fidelity-acceptance · content-preservation · agent-call count (the batch keeps it at one authoring call; the commit is auto-provisioned).
2. **doc↔code on a migrated arch-doc holds — the genuinely-new combination** — the synthetic arm migrates ≥2 components with real, **independently-resolving** `implemented-by` anchors over jigc's Rust; the blocking red fires on a deleted symbol **naming A's specific item address** (`arch-doc:<slug>#components/<a-id>/implemented-by`) while B stays valid, proving each item's anchor resolves against its own authored value. The sibling `doc-code` probe is found beside the installed `jigc` from a non-jigc repo.
3. **`cites → adr` across the migrated set holds** — authored as a **bracketed list**, each element resolved at finalize against the committed store; a dangling citation **blocks** at `ref-resolves`; the **committed-first ordering contract** (migrate the cited ADRs before the arch-doc) is what makes forward-ref integrity hold; an out-of-store decision reference **drops to prose**, never a dangling ref.
4. **The slot+field-group component is byte-stable** — a component (`description` slot + `implemented-by` field) round-trips `render(parse(x)) == x` on the **Increment-1 substrate** (the empty-slot canonicalization), including the minted-but-unfilled intermediate the per-leaf batch produces.
5. **The required `description` is honoured** — a migrated component with a code pointer but no foreign prose gets a **synthesized one-line responsibility** (a required slot cannot drop-to-prose); an unfilled one would block at `required-slot-present`.
6. **The honest bounds hold** — the real arm's anchors are **file-only** where the target is non-Rust (`symbol-exists` degrades to file-exists, the Rust-only probe — recorded, not data loss) (at M26; generalized at M27 — the probe now resolves symbols in six languages, so a non-Rust anchor no longer degrades, flow 29 / [validation.md](validation.md) → Multi-language resolution); a malformed `implemented-by` dangles at finalize rather than rejecting at write (the write-time code-anchor shape check is **not built**, C3); migrated titles render as the **slug in the H1** (now keyed); the synthetic arm is **labeled synthetic**. Framing A intact throughout — LLM proposes prose, the CLI strict-parses + places every structural act.

## 29. doc↔code on a polyglot repo — a TypeScript and a Python citation validated against the code (M27)

The M27 acceptance: **the `doc-code` differentiator turns on for a non-Rust target.** M10/M13 proved doc↔code over Rust by pointing the anchors at jigc's own codebase (sidestepping the Rust-only grammar); M27 generalizes the probe Rust→six languages ([validation.md](validation.md) → Multi-language resolution; [DECISIONS.md](../DECISIONS.md) → 2026-06-19 fork F7), so a citation into a **TypeScript** or a **Python** file genuinely resolves against reality instead of silently passing. This flow mirrors flow 16 — an `arch-doc` with **two** components, each carrying an independently-resolving `implemented-by` anchor, the per-item-disambiguation A-deleted/B-valid proof — but on a **polyglot** repo: component A's anchor at a real **TypeScript** symbol, component B's at a real **Python** symbol. The grammar generalization is **probe-internal** — no engine, CLI, or pack-schema change; the `code-anchor` field type and the workflow are language-blind. The design of record is [validation.md](validation.md#multi-language-resolution-m27); the dispatch table, the per-language node-kind allowlist, and the truth tables live there and are not restated. Notation illustrative; the flow below is the shape the acceptance test (`crates/cli/tests/flow29_acceptance.rs`) drives end-to-end through the built binary against the real `doc-code` subprocess, resolved as a **sibling of `jigc`** (no `JIGC_DOC_CODE_PROBE` override — the production path a real install hits; the measured-on-the-installed-binary proof is M27 Increment 4).

### The walk — author two components over a polyglot tree, block on each vanished symbol, fix → one commit

```text
# a polyglot repo: src/api.ts carries a real TS symbol, services/limiter.py a real Python symbol.
$ jigc start --workflow architecture-documentation "document the gateway"
$ jigc doc create arch-doc --title "Gateway" --task <id>             # mints arch-doc:gateway
$ jigc doc set-slot  "arch-doc:gateway#overview" --from-file - --task <id>
$ jigc doc set-field "arch-doc:gateway#cites" --value "adr:use-a-cache" --task <id>   # a committed adr

# component A → a TypeScript symbol; component B → a Python symbol (two languages, two files):
$ jigc doc add-item  "arch-doc:gateway#components" --title "Edge router"  --task <id>   # → #components/edge-router
$ jigc doc set-field "arch-doc:gateway#components/edge-router/implemented-by" \
        --value "src/api.ts#RateRouter" --task <id>
$ jigc doc add-item  "arch-doc:gateway#components" --title "Token limiter" --task <id>  # → #components/token-limiter
$ jigc doc set-field "arch-doc:gateway#components/token-limiter/implemented-by" \
        --value "services/limiter.py#TokenLimiter" --task <id>

# delete component A's TypeScript symbol (RateRouter) while B's Python symbol (TokenLimiter) stays valid:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/edge-router/implemented-by     ← A's item address, NOT B's
    message:          `src/api.ts#RateRouter` resolves to no symbol in the working tree
  finalize blocked — HEAD unchanged, nothing promoted.

# symmetrically: restore A, delete component B's Python symbol (TokenLimiter) while A's TS symbol stays valid:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/token-limiter/implemented-by    ← B's item address, NOT A's
    message:          `services/limiter.py#TokenLimiter` resolves to no symbol in the working tree

# restore both → finalize PASSES: one docs(arch-doc): commit; arch-doc:gateway promotes to architecture/gateway.md.
$ jigc task finalize <id>
$ git log --oneline -1
  docs(arch-doc): document the gateway        ← exactly one commit; both polyglot anchors resolved
```

The `symbol-exists` block naming **A's** address (a TypeScript symbol) while B's Python anchor stays silent — and the symmetric block naming **B's** address while A's TypeScript anchor stays silent — is the per-item disambiguation proof carried **across two languages**: each item's anchor resolves against its own authored value through its own grammar, never a clobbered shared one. The passing walk lands the commit only because the probe genuinely ran the TS grammar over `src/api.ts` and the Python grammar over `services/limiter.py` and both symbols resolved — the masking guard is the pass↔block contrast over the same fixture (a silently-skipped enumeration would land the commit in every arm).

### What it asserts (the M27 acceptance bar)

1. **A TypeScript citation validates against reality.** Deleting component A's TypeScript symbol (the file stays, the symbol is renamed away) **blocks** `finalize` on `doc-code.symbol-exists`, the rendered report's `location.address` is **A's** item address (`arch-doc:gateway#components/edge-router/implemented-by`) and **never** B's, HEAD is unchanged, nothing is promoted. The TS grammar resolved a present symbol and surfaced an absent one — the M17-friction "non-Rust symbol silently passes" is closed for TypeScript.
2. **A Python citation validates against reality, symmetrically.** Deleting component B's Python symbol while A's TypeScript symbol stays valid blocks `finalize` on `doc-code.symbol-exists` naming **B's** item address (`#components/token-limiter/implemented-by`) and **never** A's. Per-item disambiguation holds **across two grammars** — the single lever that proves the probe dispatched each anchor to its own language.
3. **Both anchors present → exactly one commit.** With the TypeScript and Python symbols both present, `finalize` validates clean, lands **exactly one** `docs(arch-doc):` commit, promotes the doc to `architecture/gateway.md`, and cleans the working area. Zero `doc-code` findings on the passing arm is the masking failure the pass↔block contrast guards against — the block arms (one per language) prove the enumeration reached each anchor-bearing component independently, so the clean pass is "the probe ran and both symbols resolved," not "the probe never ran."
4. **The sibling probe resolves with the larger grammar set.** The `doc-code` subprocess is found beside the running `jigc` (`<bin-dir>/doc-code`, **no** `JIGC_DOC_CODE_PROBE` override) — the production path a real install hits — and runs with the six-grammar set the M27 generalization links in. (The measured-on-the-`cargo install`-installed-binary proof + the size budget are M27 Increment 4.)

## 30. doc↔code on a CSS target — two selector citations validated against the stylesheet (M28)

The M28 acceptance: **the `doc-code` differentiator turns on for a CSS target.** Flow 29 generalized the probe Rust→six **AST** languages, where a citable `#symbol` is an AST *named item* (a class, a function). A CSS *selector* is not — it is an addressable unit of a different model, so it needed a **per-grammar extractor** (the HD1 keystone: read the addressable-unit name per node-kind, not a single `.name` field) rather than the unsupported-language advisory CSS took until now ([validation.md](validation.md#multi-language-resolution-m27) → Multi-language resolution; [DECISIONS.md](../DECISIONS.md) → 2026-06-20 M28 planning, forks F4/F5). This flow mirrors flow 29 — an `arch-doc` with **two** components, each carrying an independently-resolving `implemented-by` anchor, the per-item-disambiguation A-deleted/B-valid proof — but anchored at real **CSS classes** in a single `styles.css`: component A's anchor at `styles.css#card`, component B's at `styles.css#title`. The CSS extractor is **probe-internal** — no engine, CLI, or pack-schema change; the `code-anchor` field type and the workflow stay language-blind. The design of record is [validation.md](validation.md#multi-language-resolution-m27); the extractor's node-kind→name table and the CSS honest bounds live there and are not restated. Notation illustrative; the flow below is the shape the acceptance test (`crates/cli/tests/flow30_acceptance.rs`) drives end-to-end through the built binary against the real `doc-code` subprocess, resolved as a **sibling of `jigc`** (no `JIGC_DOC_CODE_PROBE` override — the production path a real install hits; the measured-on-the-installed-binary proof is M28 Increment 2).

### The walk — author two components over one stylesheet, block on each vanished selector, fix → one commit

```text
# a single styles.css carries two real CSS classes: `.card` and `.title`.
$ jigc start --workflow architecture-documentation "document the gateway"
$ jigc doc create arch-doc --title "Gateway" --task <id>             # mints arch-doc:gateway
$ jigc doc set-slot  "arch-doc:gateway#overview" --from-file - --task <id>
$ jigc doc set-field "arch-doc:gateway#cites" --value "adr:use-a-cache" --task <id>   # a committed adr

# component A → the `.card` selector; component B → the `.title` selector (one file, two selectors):
$ jigc doc add-item  "arch-doc:gateway#components" --title "Card"  --task <id>   # → #components/card
$ jigc doc set-field "arch-doc:gateway#components/card/implemented-by" \
        --value "styles.css#card" --task <id>
$ jigc doc add-item  "arch-doc:gateway#components" --title "Title" --task <id>   # → #components/title
$ jigc doc set-field "arch-doc:gateway#components/title/implemented-by" \
        --value "styles.css#title" --task <id>

# delete component A's CSS class (drop the `.card` rule) while B's `.title` rule stays:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/card/implemented-by     ← A's item address, NOT B's
    message:          `styles.css#card` resolves to no symbol in the working tree
  finalize blocked — HEAD unchanged, nothing promoted.

# symmetrically: restore `.card`, delete component B's `.title` rule while A's `.card` stays valid:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/title/implemented-by    ← B's item address, NOT A's
    message:          `styles.css#title` resolves to no symbol in the working tree

# restore both → finalize PASSES: one docs(arch-doc): commit; arch-doc:gateway promotes to architecture/gateway.md.
$ jigc task finalize <id>
$ git log --oneline -1
  docs(arch-doc): document the gateway        ← exactly one commit; both CSS anchors resolved
```

The two components anchor into the **same file** at different selectors, so the only lever that picks A from B is the per-selector extractor over the shared `styles.css` — the sharpest per-item-disambiguation fixture. The `symbol-exists` block naming **A's** address while B's selector stays silent — and the symmetric block naming **B's** address while A's stays silent — is the proof that each item's anchor resolves against its own authored selector through the CSS extractor, never a clobbered shared one. The passing walk lands the commit only because the probe genuinely ran the CSS grammar over `styles.css` and both selectors resolved — the masking guard is the pass↔block contrast over the same fixture (a silently-skipped enumeration, or CSS still taking the `unsupported-language` advisory, would land the commit in every arm).

### What it asserts (the M28 acceptance bar)

1. **A CSS selector citation validates against reality.** Deleting component A's CSS class (the file stays, the `.card` rule is dropped) **blocks** `finalize` on `doc-code.symbol-exists`, the rendered report's `location.address` is **A's** item address (`arch-doc:gateway#components/card/implemented-by`) and **never** B's, HEAD is unchanged, nothing is promoted. The CSS extractor resolved a present selector and surfaced an absent one — the CSS half of the parked "CSS + docker-compose" deferral is closed.
2. **A second CSS citation validates against reality, symmetrically.** Deleting component B's `.title` rule while A's `.card` stays valid blocks `finalize` on `doc-code.symbol-exists` naming **B's** item address (`#components/title/implemented-by`) and **never** A's. Per-item disambiguation holds **over the same stylesheet** — the single lever that proves the extractor picked each anchor's own selector out of the shared file.
3. **Both selectors present → exactly one commit.** With the `.card` and `.title` rules both present, `finalize` validates clean, lands **exactly one** `docs(arch-doc):` commit, promotes the doc to `architecture/gateway.md`, and cleans the working area. Zero `doc-code` findings on the passing arm is the masking failure the pass↔block contrast guards against — the block arms prove the enumeration reached each anchor-bearing component independently, so the clean pass is "the probe ran and both selectors resolved," not "the probe never ran (or advised them un-grammared)."
4. **The sibling probe resolves with the CSS grammar.** The `doc-code` subprocess is found beside the running `jigc` (`<bin-dir>/doc-code`, **no** `JIGC_DOC_CODE_PROBE` override) — the production path a real install hits — and runs with the seven-grammar set the M28 CSS extractor links in. (The measured-on-the-`cargo install`-installed-binary proof + the re-run size budget are M28 Increment 2.)

## 31. doc↔code on a YAML target — two compose service-key citations validated against `compose.yaml` (M29)

The M29 acceptance: **the `doc-code` differentiator turns on for a YAML target — the first language to *ride* the HD1 seam, not build it.** Flow 30 cashed in the CSS addressable-unit keystone: a per-grammar extractor that reads the addressable-unit name per node-kind rather than a single `.name` field. M29 proves that keystone **generalizes cheaply** — a YAML *mapping key* (a docker-compose service key) is not an AST named item in the `path#symbol` sense, but it slots into the same dispatch with its own small extractor (`block_mapping_pair` / `flow_pair` → the `key` field, surrounding quotes stripped) and **no engine, CLI, or pack-schema change** ([validation.md](validation.md#multi-language-resolution-m27) → Multi-language resolution; [DECISIONS.md](../DECISIONS.md) → 2026-06-20 M29 planning, forks F4/F5/F7). This flow mirrors flow 30 — an `arch-doc` with **two** components, each carrying an independently-resolving `implemented-by` anchor, the per-item-disambiguation A-deleted/B-valid proof — but anchored at real **compose service keys** in a single `compose.yaml`: component A's anchor at `compose.yaml#web`, component B's at `compose.yaml#db`. The YAML extractor is **probe-internal**; the `code-anchor` field type and the workflow stay language-blind. The design of record is [validation.md](validation.md#multi-language-resolution-m27); the extractor's node-kind→name table and the YAML honest bounds live there and are not restated. Notation illustrative; the flow below is the shape the acceptance test (`crates/cli/tests/flow31_acceptance.rs`) drives end-to-end through the built binary against the real `doc-code` subprocess, resolved as a **sibling of `jigc`** (no `JIGC_DOC_CODE_PROBE` override — the production path a real install hits; the measured-on-the-installed-binary proof is M29 Increment 2).

### The walk — author two components over one compose file, block on each vanished service key, fix → one commit

```text
# a single compose.yaml carries two real service keys under `services:`: `web` and `db`.
$ jigc start --workflow architecture-documentation "document the gateway"
$ jigc doc create arch-doc --title "Gateway" --task <id>             # mints arch-doc:gateway
$ jigc doc set-slot  "arch-doc:gateway#overview" --from-file - --task <id>
$ jigc doc set-field "arch-doc:gateway#cites" --value "adr:use-a-cache" --task <id>   # a committed adr

# component A → the `web` service key; component B → the `db` service key (one file, two keys):
$ jigc doc add-item  "arch-doc:gateway#components" --title "Web"      --task <id>   # → #components/web
$ jigc doc set-field "arch-doc:gateway#components/web/implemented-by" \
        --value "compose.yaml#web" --task <id>
$ jigc doc add-item  "arch-doc:gateway#components" --title "Database" --task <id>   # → #components/database
$ jigc doc set-field "arch-doc:gateway#components/database/implemented-by" \
        --value "compose.yaml#db" --task <id>

# delete component A's service key (drop the `web:` block) while B's `db:` block stays:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/web/implemented-by      ← A's item address, NOT B's
    message:          `compose.yaml#web` resolves to no symbol in the working tree
  finalize blocked — HEAD unchanged, nothing promoted.

# symmetrically: restore `web`, delete component B's `db:` block while A's `web:` block stays valid:
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/database/implemented-by ← B's item address, NOT A's
    message:          `compose.yaml#db` resolves to no symbol in the working tree

# restore both → finalize PASSES: one docs(arch-doc): commit; arch-doc:gateway promotes to architecture/gateway.md.
$ jigc task finalize <id>
$ git log --oneline -1
  docs(arch-doc): document the gateway        ← exactly one commit; both YAML anchors resolved
```

The two components anchor into the **same file** at different service keys, so the only lever that picks A from B is the per-key extractor over the shared `compose.yaml` — the sharpest per-item-disambiguation fixture. Component B's item slug (`database`) deliberately differs from its anchor symbol (`db`), so the block is proven keyed on the **item address**, not on a name coincidence with the symbol. The `symbol-exists` block naming **A's** address while B's key stays silent — and the symmetric block naming **B's** address while A's stays silent — is the proof that each item's anchor resolves against its own authored key through the YAML extractor, never a clobbered shared one. The passing walk lands the commit only because the probe genuinely ran the YAML grammar over `compose.yaml` and both keys resolved — the masking guard is the pass↔block contrast over the same fixture (a silently-skipped enumeration, or YAML still taking the `unsupported-language` advisory, would land the commit in every arm).

### What it asserts (the M29 acceptance bar)

1. **A YAML mapping-key citation validates against reality.** Deleting component A's service key (the file stays, the `web:` block is dropped) **blocks** `finalize` on `doc-code.symbol-exists`, the rendered report's `location.address` is **A's** item address (`arch-doc:gateway#components/web/implemented-by`) and **never** B's, HEAD is unchanged, nothing is promoted. The YAML extractor resolved a present key and surfaced an absent one — the docker-compose/YAML half of the parked "CSS + docker-compose" deferral is closed.
2. **A second YAML citation validates against reality, symmetrically.** Deleting component B's `db:` block while A's `web:` block stays valid blocks `finalize` on `doc-code.symbol-exists` naming **B's** item address (`#components/database/implemented-by`) and **never** A's. Per-item disambiguation holds **over the same compose file** — the single lever that proves the extractor picked each anchor's own key out of the shared file.
3. **Both keys present → exactly one commit.** With the `web:` and `db:` blocks both present, `finalize` validates clean, lands **exactly one** `docs(arch-doc):` commit, promotes the doc to `architecture/gateway.md`, and cleans the working area. Zero `doc-code` findings on the passing arm is the masking failure the pass↔block contrast guards against — the block arms prove the enumeration reached each anchor-bearing component independently, so the clean pass is "the probe ran and both keys resolved," not "the probe never ran (or advised them un-grammared)."
4. **The sibling probe resolves with the YAML grammar.** The `doc-code` subprocess is found beside the running `jigc` (`<bin-dir>/doc-code`, **no** `JIGC_DOC_CODE_PROBE` override) — the production path a real install hits — and runs with the eight-grammar set the M29 YAML extractor links in. (The measured-on-the-`cargo install`-installed-binary proof + the re-run size budget are M29 Increment 2.)

## 32. WIP left out and surfaced — `finalize` commits the declared change-set, names the rest (M30)

The M30 acceptance: **per-task `finalize` commits exactly the *declared* change-set — the agent's git index + jigc's promoted docs + the first-commit config layer — and *surfaces* everything else rather than sweeping it in.** M30 inverted the per-task stage from "`git add --all` the whole dirty tree" to "honor the agent's existing index" ([finalize.md](finalize.md#dirty-tree-policy) → Dirty-tree policy, revised M30; [DECISIONS.md](../DECISIONS.md) → 2026-06-20 M30 planning, G1/G2/G3/G6). Two consequences fall out, and this flow proves both end-to-end on the **production path** a real install hits — the cargo-built binary with the **embedded** dev pack and the `doc-code` probe resolved as a **sibling of `jigc`** (no `JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE` override; the flow30/31 real-binary idiom). The agent's half of the contract is G5: the workflow now tells the agent to `git add` its task's code edits before finalize, and the Claude Code adapter permits `git add *` ([assistant-adapter.md](assistant-adapter.md); the pack steps + adapter permit land in M30 Increment 4). Notation illustrative; the flow below is the shape the acceptance test (`crates/cli/tests/flow32_acceptance.rs`) drives end-to-end through the built binary.

### The walk — stage your change, leave the rest; a cited-but-unstaged symbol blocks

```text
# the agent stages its task edit; an unrelated untracked file and an unrelated unstaged
# tracked edit sit alongside it in the dirty tree:
$ jigc start --workflow single-task "scope the set"
$ git add feature.rs                          # the agent's own task edit — staged (G5)
# scratch.txt        — unrelated, untracked   (never `git add`ed)
# README.md          — unrelated, modified    (tracked, unstaged)
$ jigc task finalize <id>
  finalized <hash> — feat(cache): scope the declared change set
    modified feature.rs
    1 file(s) committed
    left-out (unstaged/untracked — git add to include):
      scratch.txt
      README.md
$ git show --name-only HEAD
  feature.rs                                  ← ONLY the staged edit (+ jigc's own files)
$ git status --porcelain
  ?? scratch.txt                              ← still untracked, left behind
   M README.md                                ← still modified, left behind

# a doc citing a code symbol the agent WROTE but did NOT stage — the finalize-scope doc-code
# probe validates the materialized git INDEX, so the symbol is absent → blocks:
$ jigc start --workflow architecture-documentation "document the gateway"
$ jigc doc create   arch-doc --title "Gateway"
$ jigc doc add-item "arch-doc:gateway#components" --title "Widget"     # → #components/widget
$ jigc doc set-field "arch-doc:gateway#components/widget/implemented-by" \
        --value "widget.rs#render_widget"
# agent appends `pub fn render_widget() {}` to the tracked widget.rs but never `git add`s it
$ jigc --format json task finalize <id>
  ✗ doc-code · symbol-exists · blocking
    location.address: arch-doc:gateway#components/widget/implemented-by
    message:          `widget.rs#render_widget` resolves to no symbol in the index
  finalize blocked — HEAD unchanged, nothing promoted.
```

The commit carries only what the agent declared (the index) plus jigc's own promoted/config files — never the ambient WIP — and the left-out residual is the post-commit `git status --porcelain` worktree column, the same set the `--dry-run` forecast predicts (the included/left-out split, M30 G3). The block on the unstaged citation is the sharp half: the agent *wrote* the symbol, so a working-tree read would pass it; the index read catches it, because the index is exactly what is about to be committed. Validated reality == committed reality.

### What it asserts (the M30 acceptance bar)

1. **The commit is scoped to the staged index.** A finalize with a staged task edit, an unrelated untracked file, and an unrelated unstaged-modified tracked file commits **only** the staged edit (plus jigc's promoted/config files); `git show --name-only HEAD` carries the staged edit and **neither** unrelated file, and the unrelated local edit never reaches the committed bytes.
2. **The rest is surfaced, not swept.** Both unrelated changes **remain** uncommitted in the working tree post-commit (`git status --porcelain` still shows the untracked file and the modified tracked file), and the emitted finalize output **names the left-out set** — the untracked file and the unstaged-tracked file — so the agent can `git add` them on a follow-up rather than discovering them silently committed.
3. **A citation the agent wrote but did NOT stage blocks.** A component anchoring a Rust symbol appended to a tracked file but never `git add`ed makes `finalize` block on `doc-code.symbol-exists` naming the citing item's anchor address (`arch-doc:gateway#components/widget/implemented-by`) and the dangling target (`widget.rs#render_widget`); HEAD is unchanged, nothing is promoted. The finalize-scope `doc-code` probe validates the materialized git **index** (M30 Increment 3, G4), so an unstaged symbol is absent reality — the keystone the index-as-change-manifest model rests on.
4. **The production path resolves.** The whole loop runs on the cargo-built `jigc` against the **embedded** dev pack with the `doc-code` probe found beside the binary (no `JIGC_PACK_DIR`, no `JIGC_DOC_CODE_PROBE` override) — the path a real install hits; a missing sibling would surface as a `pack-probe-integrity` meta-finding, asserted absent.

## 33. The combine keystone — a `squash: true` fan-out folds N worktree-staged code-sets into one commit, drops none (M31)

The M31 acceptance: **a `squash: true` fan-out `finalize` combines every sub-agent's worktree-staged code-set into one commit — disjoint-apply in task-id order, block + route a same-file collision, never `git merge` — committing every sub-agent's code and dropping none.** M31 isolated each fanned-out sub-agent's code in its own git worktree (`.jigc/worktrees/<sub-id>`, [storage.md](storage.md) → the third combine-mode). That isolation re-opened a data-loss hole: the old `squash: true` boundary `git add --all`ed the **main checkout**, where the worktree-isolated code never lives — so it committed **zero** sub-agent code. The combine channel closes it: at finalize the CLI reads each worktree's `git diff --cached`, computes a **rename-aware block-set** (the union of {staged path, rename old-path, delete path}), and — absent a cross-worktree intersection — builds the combined tree **off-line via a temp index** (`GIT_INDEX_FILE` at a throwaway path: `read-tree` the base, `git apply --cached` each worktree's patch in task-id order, `write-tree`), then commits that tree from a **clean dedicated worktree** — where `git commit -F` (never `--no-verify`) runs the user's `pre-commit`/`commit-msg` hooks against the combined tree (M31 Inc 5; [finalize.md](finalize.md#fan-out-finalize)) — and **fast-forwards main only after the hook-running commit lands**. The live main worktree/index is **never mutated until that clean fast-forward**, so a blocked or failed combine (or a hook rejection) needs no destructive reset — the M30 `git reset --hard` rollback hazard (which would wipe unrelated main-checkout WIP) is rejected ([DECISIONS.md](../DECISIONS.md) → 2026-06-20 M31 planning, the combine + off-substrate gate-review S1/S2; 2026-06-21 M31 Inc 5, the hook restoration). Notation illustrative; the flow below is the shape the acceptance test (`crates/cli/tests/flow33_acceptance.rs`) drives end-to-end through the built binary with the **embedded** dev pack.

### The walk — two sub-agents on disjoint files combine; a same-file pair blocks

```text
# a milestone fans out to two sub-agents, each editing DISJOINT code in its own worktree:
$ jigc milestone create "Cache rework"
$ jigc milestone add-task cache-rework "Area low"     # → sub-task area-low
$ jigc milestone add-task cache-rework "Area zed"     # → sub-task area-zed
$ jigc milestone provision cache-rework               # N base-pin worktrees
# area-low stages src/low.rs in .jigc/worktrees/area-low; area-zed stages src/zed.rs in its own
# each sub-area also stages a disjoint persisted ADR (adr:low-policy, adr:zed-policy)
$ jigc milestone finalize cache-rework
  finalized <hash> — Finalize milestone cache-rework (2 sub-tasks)
$ git show --name-only HEAD
  src/low.rs                ← area-low's worktree code (dropped none)
  src/zed.rs                ← area-zed's worktree code (dropped none)
  docs/decisions/low-policy.md
  docs/decisions/zed-policy.md
$ git status --porcelain    ← clean: no ` D` drift after the combine + worktree teardown

# two sub-agents touching the SAME file — one of them via a RENAME of it — BLOCK:
# area-low edits shared.txt; area-zed `git mv shared.txt moved.txt` (its old-path collides)
$ jigc milestone finalize cache-rework
  ✗ combine.code-collision · blocking
    code collision — `shared.txt` (sub-tasks [area-low, area-zed]) staged by more than one
    worktree; the combine disjoint-applies code and never text-merges a shared file
    route: have the contending sub-tasks touch distinct files, or combine by hand
  # HEAD unchanged, nothing promoted; the off-line build never touched the live checkout,
  # so unrelated main-checkout WIP survives untouched.
```

The disjoint case is the data-loss fix: both sub-agents' code rides the one commit because the combine reads it from the worktrees, not from a sweep of the (empty) main checkout. The collision case is the never-blind-merge discipline ([storage.md](storage.md) → the by-task-id join) applied to the code substrate — a shared file is routed to a human, never text-merged, and the rename-aware block-set catches the case where one sub-agent *moves* the contended path. Because the combine builds off-line in a temp index, a blocked finalize is non-destructive: the main checkout's index, working tree, and unrelated WIP are byte-identical afterward.

### What it asserts (the M31 acceptance bar)

1. **Disjoint code + docs, one commit, drops none.** A `squash: true` fan-out with two sub-agents on disjoint files commits **both** sub-agents' staged code (`src/low.rs`, `src/zed.rs`) **and** the merged docs in **one** commit — the data-loss repro (RED before the combine: the `git add --all` sweep committed zero sub-agent code).
2. **The main checkout is clean post-commit.** `git status --porcelain` is empty after the combine + worktree teardown — no ` D` drift.
3. **The commit is order-invariant.** The combined commit's **tree hash** and **message** are byte-identical across divergent sub-agent feed/completion orders (the recorded `tasks.json` order *and* the code-staging order varied) — tree+message, not the timestamped commit SHA (Validation hardening #7 extended to the code substrate).
4. **A same-file collision blocks (incl. a rename).** Two sub-agents staging the same path — one of them via a rename of it — block with a routed `combine.code-collision` naming the contended path; HEAD is unchanged and nothing is applied or promoted.
5. **Unrelated main-checkout WIP survives a blocked combine.** An untracked file and an unstaged tracked edit in the main checkout are byte-identical after a blocked finalize — the off-line temp-index build never touches the live checkout (review S2).
6. **An advanced main blocks finalize.** When main advances past the milestone's pinned base, the `base == main-HEAD` preflight (review S1) blocks with the base-mismatch finding; HEAD is unchanged and nothing is applied — the precondition for a clean disjoint-apply.

## 34. The freeze enforced — an un-migrated schema-shape change is blocked at pack-load (M33)

The M33 acceptance: **the frozen-v1 doctype set self-enforces.** A versioned/hashed doctype-set manifest (`config/schema-manifest.yaml`) enumerates each frozen doctype's `schema-version` + `schema-hash`; the engine recomputes each shipped doctype's hash and requires an exact match. The enforcement gate fires at **pack-load** — the productive compose front door — not at the report-only `jigc validate` store sweep ([corpus-migration.md](corpus-migration.md) → The freeze, declared *and* enforced; review Finding 3). So a schema edit that changes a doctype's *shape* without bumping its version + shipping the M34 migration is **blocked**, loudly, rather than caught by review. The build-time sibling pins it for the bytes we ship; this flow is the runtime proof on a customer-shaped on-disk pack (`JIGC_PACK_DIR`). Notation illustrative; the flow below is the shape the acceptance test (`crates/cli/tests/freeze_enforcement.rs`) drives end-to-end through the built binary.

### The walk — drift a schema, get blocked; drop the manifest, get through

1. **Copy the dev pack to a directory pack and run clean.** `JIGC_PACK_DIR=<copy> jigc start --workflow single-task "<intent>"` composes normally — the copy's six schema shapes match the manifest hashes it shipped.
2. **Drift one schema's shape, leave the manifest unbumped.** Edit `<copy>/schemas/adr.yaml` (e.g. relocate it, `location: decisions/` → `location: adr-records/`) — a hash-affecting change — without touching `schema-version` or the manifest's `schema-hash`.
3. **Re-run — blocked at pack-load.** `jigc start …` now exits **non-zero** before composing, naming the breach: `pack-load freeze check failed: doctype 'adr': schema-hash mismatch (manifest declares '…', recomputed '…')`. The fix is to bump the version + ship the migration (M34), or revert.
4. **Drop the manifest — the gate goes inert.** Remove `<copy>/config/schema-manifest.yaml` and re-run: the same drifted pack composes clean. A pack that ships **no** manifest is unchecked (the field-types-absent precedent), so seeded / composed / methodology packs that never froze stay inert — never an error.

### What it asserts (the M33 acceptance bar)

1. **A shape change with no version bump is blocked, naming the doctype + the mismatch.** The drifted-`adr` copy makes `jigc start` exit non-zero; stderr names `schema-hash mismatch` on `adr` (`schema_shape_drift_without_manifest_bump_is_blocked`).
2. **The unmutated copy composes clean.** A faithful dev-pack copy matches its shipped manifest, so the gate is inert and `start` exits 0 (`unmutated_pack_copy_composes_clean`) — the control that proves the gate isn't a blanket reject.
3. **A manifest-less pack is unaffected.** The *same* drift composes clean once `schema-manifest.yaml` is dropped (`manifest_less_pack_is_unaffected`) — the omitting context: the freeze records what a pack *declares* frozen, and a pack that declares none is never gated.
4. **The freeze is composition-independent.** The gate checks the manifest-owning pack's *own* shipped shapes in isolation, so a higher-precedence pack that shadows a frozen doctype with a divergent shape (the embedded methodology pack's own `commit`) does not perturb the dev pack's freeze — every methodology / multi-pack composition still composes clean.

## 35. The corpus migrated — a stranded v0 corpus is detected, stamped, and re-validated clean (M34)

The M34 acceptance: **a managed corpus at an old schema version is detected, migrated, and made conformant — the full detect→block→migrate loop, end-to-end, deterministic and CLI-owned.** Flow 34 froze the v1 shape and made an un-migrated schema change *blocked* at pack-load; M34 builds the **corpus-side** that lets a *blocked* change be *migrated* rather than only blocked. The one genuinely pending shape change post-freeze is the **schema-version stamp** itself: the engine injects an `added-optional-field` stamp declaration into every frozen persisted doctype, so a committed corpus authored before the stamp existed (a **v0 corpus**) is a real `added-field` migration target — the live dogfood, not synthetic coverage. `jigc migrate-corpus` runs the real schema-diff classifier over the genuine corpus and applies the engine transform's `added-optional-field` branch byte-stable, per-doc gated, **the stamp flipped last** ([corpus-migration.md](corpus-migration.md) → Acceptance flows + the stamp-flips-last rule; the determinism boundary: no LLM in the structural path). The verb is disjoint from `jigc migrate` (foreign-doc *adoption*, LLM re-author) and `jigc upgrade` (config-delta reconciliation). Notation illustrative; the loop below is the shape the acceptance test (`crates/cli/tests/corpus_migration.rs`) drives end-to-end through the built binary, with the header-less fence-introducing case + the combined stamp-flips-last case covered over the verb core (`crates/cli/src/migrate_corpus.rs`).

### The walk — detect the v0 corpus, migrate it, re-validate clean

```
# a committed ADR authored before the schema-version stamp existed (the v0 corpus state):
# docs/decisions/alpha-decision.md carries `status:` + `date:` in its header, NO `schema-version:`.

# 1. DETECT — the store-scope sweep reports the stranded doc, routed `migrate` (report-only, exit 0):
jigc validate
#   schema-conformance.field-value-conformant — docs/decisions/alpha-decision.md …
#     route: migrate — … carries no schema-version stamp; run the corpus migration …

# 2. MIGRATE — the verb stamps the corpus byte-stable via the real added-optional-field transform:
jigc migrate-corpus
#   corpus migration: 1 migrated, 0 already current, 0 blocked
#     migrated   docs/decisions/alpha-decision.md
# the ADR now carries `schema-version: 1` spliced into its existing header; status/date + body unchanged.

# 3. RE-VALIDATE — the corpus is conformant + stamped v1: no schema-conformance finding, exit 0:
jigc validate
#   (no schema-conformance findings, no migrate route)
```

The **stamp-flips-last** rule, on a combined change (the stamp **plus** a new required slot): the migration mints the empty slot and the per-doc conformance gate **fails**, so the whole doc rolls back — it stays byte-identical v0, **unstamped**, and is routed to the agent (Framing A) to author the prose. The schema-version stamp therefore never lands mid-migration; once the prose is authored and the doc gates clean, a re-run flips the stamp. An already-stamped (current) doc is skipped byte-untouched (the false-positive guard).

### What it asserts (the M34 acceptance bar)

1. **A v0 corpus is detected, then migrated, then re-validates clean.** The unstamped ADR is reported `route: migrate`; `jigc migrate-corpus` splices `schema-version: 1` into its header byte-stable (prior fields + body preserved); a re-validate surfaces no schema-conformance finding (`migrate_corpus_stamps_the_v0_dogfood_then_revalidates_clean`).
2. **An already-current corpus is a clean no-op.** A doc stamped at the current version is left byte-identical — the migration migrates nothing (`migrate_corpus_is_a_no_op_on_an_already_current_corpus`; verb-core `already_current_doc_is_skipped_byte_untouched`).
3. **The header-less case introduces the fence byte-stable.** A doctype that renders no front-matter gains a `---` block carrying the stamp, its body slots preserved (`header_less_doc_migration_introduces_the_fence_byte_stable`).
4. **The stamp flips last.** A combined stamp + prose-needing change leaves the doc byte-identical v0 and unstamped until the prose is authored, then flips the stamp on a clean re-run (`combined_change_stamp_flips_only_after_prose_authored`) — the omitting context that proves the stamp never lands mid-migration.

## 36. The corpus migrated, structurally — a v1→v2 fixed-slot→repeatable reshape sourced from the snapshot store (M34 Inc 4)

Flow 35's dogfood was the v0→v1 **stamp** (an `added-optional-field` whose prior shape is derivable in-process, `from = strip_stamp(current)`). M34 Increment 4 makes the **structural** transform branches — built + unit-proven since Inc 2 but engine-substrate-only — reachable through the shipped `jigc migrate-corpus` verb, by giving it the one thing a genuine v1→v2 structural change needs and the current schema cannot supply: the **actual prior shape**. The pack stores prior shapes as **versioned snapshots** (`schema-snapshots/<type>.v<N>.yaml`, a dedicated pack resource kind), and the verb sources `from` per committed doc keyed on its schema-version stamp ([corpus-migration.md](corpus-migration.md) → Prior-schema sourcing). The headline is the **reconstructed M25 `prd.requirements` fixed-slot→repeatable reshape** (the riskiest net-new primitive), run end-to-end through the **compiled binary** over a `FilesystemPack` fixture (`JIGC_PACK_DIR`): the current `prd` at manifest version 2 (the repeatable shape) + `schema-snapshots/prd.v1.yaml` (the fixed-slot prior shape) + a committed corpus of v1-stamped fixed-slot prd docs. The migration diffs the two **real declared schemas**, applies the structural splice (the old slot prose preserved verbatim as the default first item), **value-bumps** the stamp `1→2` (a present-field `set_field` splice, distinct from the v0→v1 add), gates each doc on conformance, and writes it back byte-stable — **deterministic, CLI-owned, no LLM in the structural path**. Notation illustrative; the loop below is the shape the acceptance test (`crates/cli/tests/flow36_corpus_structural.rs`) drives through the built binary, with the below-version snapshot + value-bump cases also covered over the verb core (`crates/cli/src/migrate_corpus.rs`).

### The walk — detect the below-version corpus, migrate it structurally, re-validate clean

```
# a committed v1 corpus: docs/prds/cache-prd.md carries `schema-version: 1` and the OLD
# fixed-slot `## Requirements` prose, below the bumped manifest version (prd at v2).

# 1. DETECT — the version-aware store sweep routes each below-version, non-conformant prd
#    `migrate` (report-only, exit 0):
jigc validate
#   schema-conformance … — docs/prds/cache-prd.md …  route: migrate — …

# 2. MIGRATE — the verb sources prd.v1 from the snapshot store, reshapes, value-bumps the stamp:
jigc migrate-corpus
#   corpus migration: 3 migrated, 0 already current, 0 blocked
#     migrated   docs/prds/cache-prd.md
# the requirements slot becomes a repeatable section; the old prose is its default first item;
# the stamp is value-bumped `schema-version: 1` → `2`. vision/context prose unchanged.

# 3. RE-VALIDATE — the migrated corpus is v2-conformant: no schema-conformance finding, exit 0:
jigc validate
#   (no schema-conformance findings, no migrate route)
```

A **widened-cardinality** secondary rides the same run: `adr.supersedes` `0..1`→`0..*` (snapshot `adr.v1.yaml` at the narrower card, the current `adr` at the wider). A committed v1 adr migrates **byte-identical-except-stamp** — the widening is a byte no-op (the existing value stays valid under the wider cardinality), so only the stamp value-bumps. **Detector and verb agree**: the docs the version-aware detector routes `migrate` are *migrated* by the verb, never reported `already-current` ([DECISIONS.md](../DECISIONS.md) → 2026-06-25 audit Finding 2). A below-version stamped doc whose prior-shape snapshot is **not shipped** is **blocked** with a route naming the missing snapshot (never a silent `already-current`).

### What it asserts (the M34 Inc-4 acceptance bar)

1. **A v1→v2 structural reshape migrates byte-stable through the real binary.** The fixed-slot `prd` is reshaped to the repeatable form with the old slot prose preserved as the default item, the stamp value-bumped `1→2`, v2-conformant (a re-validate finds no schema-conformance finding), and **idempotent** — a re-run is a byte-untouched no-op (`structural_v1_to_v2_migration_runs_through_the_real_binary`).
2. **Deterministic across commit orders.** The same two-prd corpus committed in id-order and in **reverse** migrates to byte-identical output — the verb keys output on the path-sorted corpus, never on commit order (`structural_migration_is_byte_identical_across_commit_orders`; increment-workflow #7).
3. **A widened-cardinality migration is byte-identical-except-stamp.** `adr.supersedes` `0..1`→`0..*` rewrites no instance bytes; only the stamp value-bumps (same test, the secondary doctype).
4. **Detector and verb agree on the below-version case.** `jigc validate` routes the v1 prd docs `migrate`; `jigc migrate-corpus` migrates those same docs (never `already-current`) — the Finding-2 fix (same test; the missing-snapshot block + value-bump covered over the verb core in `migrate_corpus.rs`).
5. **The Inc-3 v0→v1 stamp dogfood stays green** (`migrate_corpus_stamps_the_v0_dogfood_then_revalidates_clean`, flow 35) — the add-field path is unchanged by the snapshot-sourcing rework.

## 37. The CLI-owned rename — one command re-slugs a decision and repoints every referrer atomically (M35)

The cost-win differentiator's deterministic core, separate from its post-build cost+completeness study (canonical record: [DECISIONS.md](../DECISIONS.md) → 2026-06-28 M35 planning; the verb spec: [write-commands.md](write-commands.md) → `jigc rename`). A committed `adr` is renamed through one verb; the CLI walks the target's **inverse edges** and rewrites every persisted referrer's structured ref-field old→new, rewrites the moved doc's H1, `git mv`s, and commits — **one atomic transaction with its own rollback** ([write-commands.md](write-commands.md) → `jigc rename`). The store-wide rename detection is the **backstop** under it: a bare `git mv` committed *without* the verb is detected and **blocked** ([reconciliation.md](reconciliation.md) → Rename detection, the A+B decision). Notation illustrative; the loop below is the shape the acceptance test (`crates/cli/tests/flow37_rename.rs`) drives through the built binary.

### The walk — rename the decision, every referrer follows; collisions and mid-fan-out and OOB moves all block

```
# a committed store: docs/decisions/single-node-cache.md (adr:single-node-cache),
# superseded-by docs/decisions/distributed-cache.md (adr:distributed-cache#supersedes),
# cited by docs/architecture/cache-layer.md (arch-doc:cache-layer#cites).

# 1. RENAME — one call: re-slug from the new title, repoint both referrers, git mv, commit:
jigc rename adr:single-node-cache --to "Local in-process cache"
#   renamed adr:single-node-cache → adr:local-in-process-cache
#     repointed  adr:distributed-cache#supersedes
#     repointed  arch-doc:cache-layer#cites
#     moved      docs/decisions/single-node-cache.md → docs/decisions/local-in-process-cache.md
#     committed  rename adr:single-node-cache → adr:local-in-process-cache
#   reported (not rewritten): "single-node-cache" mentioned in prose at
#     docs/architecture/cache-layer.md (slot prose) · README.md · src/cache.rs (comment)

# 2. RE-VALIDATE — no dangling refs; the store is clean (exit 0):
jigc validate
#   (no schema-conformance.ref-resolves findings)

# 3. COLLISION — renaming onto an existing slug blocks (an identity refactor, never a silent suffix):
jigc rename adr:distributed-cache --to "Local in-process cache"
#   error: adr:local-in-process-cache already exists — choose a free slug (rename blocks on collision)

# 4. MID-FAN-OUT — forbidden while a milestone is in-flight (coarse guard):
jigc rename adr:distributed-cache --to "Distributed cache v2"
#   error: a milestone (m:cache-overhaul) is in-flight — rename is forbidden mid-fan-out

# 5. OOB BACKSTOP — a bare git mv, committed without the verb, is detected and BLOCKED at the hook:
git mv docs/decisions/distributed-cache.md docs/decisions/dist-cache.md && git commit -am "rename"
#   pre-commit: blocked — adr:distributed-cache appears renamed via a bare `git mv`
#     adopt it:  jigc rename adr:distributed-cache --to "<New Title>"
#     or revert: git mv docs/decisions/dist-cache.md docs/decisions/distributed-cache.md
```

### What it asserts (the M35 acceptance bar)

1. **One command repoints every persisted referrer atomically.** `jigc rename` walks the inverse edges and rewrites `adr.supersedes` *and* `arch-doc.cites` (both list-valued — the one element is swapped, the canonical whole-list re-emitted) old→new, rewrites the H1, `git mv`s, and commits as **one** commit; a re-validate finds zero dangling refs. A renamed `prd` likewise repoints `spec.derived-from` (the third persisted ref-field). `git log --follow` survives the move.
2. **The determinism boundary holds — prose/unmanaged mentions are reported, never rewritten.** Old-slug occurrences in managed-doc prose and unmanaged files (README, code) are *listed* for the agent; the CLI rewrites only structured ref-fields.
3. **Collision on `--to` blocks** — renaming onto an existing committed slug is refused (an identity refactor of an existing doc, never the by-task-id join's silent suffix).
4. **Forbidden mid-fan-out** — `jigc rename` blocks whenever any milestone is in-flight (the coarse milestone-dir guard), because a rename changes the by-task-id join's same-doc-clash key.
5. **The transaction rolls back on partial failure** — a forced failure after the `git mv` (e.g. a rejecting git hook, or a referrer-rewrite error) restores every captured pre-image and reverses the `git mv`, leaving the store byte-identical to pre-rename — no half-moved slug, no dangling refs.
6. **The OOB backstop blocks (A+B), scoped to *this commit's* moves.** A bare `git mv` of a managed doc, committed without `jigc rename`, is caught by the store-scope rename classifier (firing before `ref-resolves`, which is scope-subtracted so the move surfaces as **one** `reconciliation.rename`, not N dangling-ref findings) and **blocks the commit** via the pre-commit hook, routing to adopt-or-revert — including the referrer-less case no dangling-ref check would catch.
7. **The hook does NOT block on pre-existing drift (no masking-trap regression).** An unrelated commit made while a *pre-existing* OOB-moved doc sits in the tree (its move not staged in this commit) **passes** — the hook intersects rename hits with this commit's staged `git diff --cached` R/D/A set and blocks only on a rename this commit makes (`flow37_rename.rs`: an unrelated commit with a prior unstaged OOB move does not block).
8. **List-valued sibling integrity.** An `arch-doc` citing `[adr:a, adr:old, adr:c]`, on renaming `old→new`, yields exactly `[adr:a, adr:new, adr:c]` — siblings `a`/`c` byte-identical and order preserved (the whole-list re-emit swaps only the one element).

## 38. Vision-forming from research + park-idea — the design-altitude doctypes at the RC-trial composition (M37)

The M37 arc: three methodology-pack **design-altitude** doctypes (`research` · `vision` · `idea`) drive the *how-you-work* level of authoring — research is gathered, a `vision` is formed from it and compared-against, and a shaped-but-unscheduled `idea` is parked. The done-picture is walked here under the **`[dev ▸ methodology]`** composition — the RC-trial's actual on-ramp, not methodology-alone. The three schemas, the two additive engine knobs M37 shipped (`display-title:` and its render-to-root knob — **retired at M38**, `vision` now *managed directly* at root `VISION.md` via the `placement:` model, [storage.md](storage.md) → Placement; the corrected end-to-end layout is walked in flow 39), the driving workflows, and the six acceptance arms are the design of record in [design-altitude-doctypes.md](design-altitude-doctypes.md) (§§2–4, §7) and are not restated here; the exact verb sequence is driven against the real binary by the `crates/cli/tests/flow*.rs` M37 suite (`flow_do_research`, `flow_form_vision`, `flow_park_idea`, and the consolidated §7 suite). Notation illustrative.

`vision —grounded-in→ research` is the **methodology pack's first internal managed ref** ([design-altitude-doctypes.md](design-altitude-doctypes.md) → §1): intra-pack and doc-level, so it points *inside* the composed schema universe and honors — does not reopen — the M16 "methodology composes alone" bound. `form-vision` is a **re-entry flow** in the exact shape of the superseding worked example ([flow 5](#5-superseding-decision--context-slice--edge-integrity)): the edge-walk slice resolves at *compose* time, so the vision must be created and grounded *before* the author step reads it, or the read is a vacuous green.

### The walk — research ×2, then form the vision across a re-compose, then park an idea

```text
$ jigc setup                                                # [dev ▸ methodology] composed dev-highest (flow 23)

# ARM 1 — two research docs, each its own task, each committed (the grounding targets).
$ jigc start --workflow do-research "benchmark the cache"   # off no-router menu / by name; mints a task
$ jigc doc create research --title "Cache benchmark" --task <t1>   # create-gate grants research
$ jigc doc set-slot research:cache-benchmark#question  --from-file -
$ jigc doc set-slot research:cache-benchmark#findings   --from-file -   # "Redis wins at p99 …"
$ jigc doc set-slot research:cache-benchmark#sources    --from-file -
$ jigc task finalize <t1>
> promote: research/cache-benchmark.md        # on-create `date` stamped; committed grounding target
# … a second do-research task commits research/session-store-survey.md the same way.

# ARM 2 — form the vision from BOTH research, across a REQUIRED re-compose.
$ jigc start --workflow form-vision "form the project vision"     # first compose: grounded-in unset →
#   the edge-walk slice {{@task.vision.grounded-in#findings}} resolves EMPTY (no findings echoed yet).
$ jigc doc create vision --title Vision --task <t2>              # → vision:vision (singleton fixed slug)
$ jigc doc set-field vision:vision#meta/grounded-in --value "[research:cache-benchmark, research:session-store-survey]"
$ jigc start --task <t2>                                         # RE-COMPOSE: the slice now reads
#   BOTH grounding research docs' `findings` into the guidance (walk_edge fans out to every target —
#   each rendered as its own `<type>:<slug>`-labelled blockquote for N≥2; the stored refs stay the anchor).
$ jigc doc set-slot vision:vision#thesis         --from-file -
$ jigc doc set-slot vision:vision#invariants     --from-file -
$ jigc doc set-slot vision:vision#open-questions --from-file -
$ jigc task finalize <t2>
> validate: clean                              # BOTH grounded-in targets resolve — no ref-resolves block
> promote:  VISION.md                          # managed directly at root (placement knob, M38); H1 `# Vision` (display-title), not `# vision`
> commit:   one commit, the managed root doc + the code

# ARM 3 — park a shaped idea; park-idea is router-selectable (discoverable), not off-menu.
$ jigc start --workflow park-idea "a public pack platform, someday"   # selectable: true — appears in the menu
$ jigc doc create idea --title "Public pack platform" --task <t3>
$ jigc doc set-field idea:public-pack-platform#meta/trigger --value "an external domain earns a stabilized pack API"
$ jigc doc set-slot  idea:public-pack-platform#description --from-file -
$ jigc task finalize <t3>
> promote: ideas/public-pack-platform.md
```

### The reds — each fires on real input, not a fixture

```text
# ARM 4 — DANGLING: a grounded-in pointing at a non-existent research BLOCKS at finalize (per-element):
$ jigc doc set-field vision:vision#meta/grounded-in --value "[research:cache-benchmark, research:does-not-exist]"
$ jigc task finalize <t2>
> BLOCK schema-conformance.ref-resolves @ vision:vision#meta/grounded-in → research:does-not-exist
#   the resolvable sibling does not rescue the dangling one — non-zero exit, NO commit.

# ARM 6 — PRE-EXISTING ROOT (no silent data loss): a hand-authored root VISION.md, no managed vision yet →
$ jigc task finalize <t2>     # first form-vision finalize, foreign root file present
> BLOCK: a VISION.md exists that jigc did not generate — adopt its content into the vision doc or remove it, then re-run
#   the render never overwrites the foreign file; once removed/adopted, finalize proceeds and owns the root file.
```

### What it asserts (the M37 acceptance bar — the six §7 arms)

1. **The two-task committed-read spine holds under `[dev ▸ methodology]`.** `do-research` commits a `research` doc (on-create `date` stamped) so `form-vision` reads it committed — the superseding-flow shape, now at the design-altitude level in a methodology composition (the `grounded-in` edge-walk's first real driver).
2. **The multi-valued anchor + the findings-echo both resolve.** A `vision` grounded in **both** research docs finalizes clean (every element `ref-resolves`-validated, no block), and the re-composed `form-vision` guidance actually *reads* **both** grounding research docs' `findings` prose into itself — the edge-walk slice fans out to every bound target, not merely "resolved" (closing the vacuous-green gap); each source renders as its own `<type>:<slug>`-labelled blockquote for N≥2 (the all-source content-echo, §3 constraint 2).
3. **The two engine knobs land.** The managed root `VISION.md` H1 reads `# Vision` (the `display-title:` knob narrows the singleton `title = slug` branch), and the doc is *managed directly at repo-root* — one file, proper reconciliation — via the `placement:` knob (M38 retired M37's render-to-root mechanism, [storage.md](storage.md) → Placement) — `describe` lists all three doctypes + three workflows.
4. **Parking is a first-class, discoverable move.** `park-idea` mints an `idea` (description + `trigger`) and finalizes to `ideas/<slug>.md`, and it is **router-selectable** — it appears in the selection surface, so a shaped mid-work direction is kept without leaving the loop.
5. **The dangling arm blocks per-element.** A `grounded-in` with one non-existent target blocks at finalize on `ref-resolves` — the resolvable sibling does not rescue it (no commit).
6. **The freeze stays green and the managed root file is non-destructive.** The additive `display-title:`/`placement:` schema keys serialize-skip when absent, so no frozen dev-pack doctype's `schema-hash` changes and the pack-load freeze assertion stays green ([design-altitude-doctypes.md](design-altitude-doctypes.md) → §4, superseded by M38's placement model); and in an existing project whose root `VISION.md` is hand-authored (no managed vision yet), the first `form-vision` finalize **blocks and routes** (the inherited `plan_clobber_guard`) rather than overwriting — the greenfield and existing-project on-ramps, the RC trials' two entry points.

## 39. The placement convention end-to-end — root `VISION.md` + root `CHANGELOG.md` under `[dev ▸ methodology]` (M38)

The **placement convention** ([storage.md](storage.md) → Placement) kills the one-file-in-a-folder shortcut (`docs/vision/vision.md`, `docs/changelog/changelog.md`): a singleton + ecosystem-idiomatic doctype is **managed directly** at its literal repo-root home, a singleton + internal doctype at a direct `docs/*.md` file. This flow walks the whole convention **end-to-end** in a *single* repo, composed the way an RC trial actually composes — the on-disk **methodology** pack listed in `packs.yaml` over the embedded **dev** base (`[dev ▸ methodology]`) — so the two placement doctypes that live at the repo root (`vision`, methodology; `changelog`, dev) coexist in one tree beside a `docs/*.md` methodology singleton (`roadmap`) and the sibling root non-doctype files (`README.md`, `CLAUDE.md`) an agent must *not* sweep in. The engine model (the `placement: { file: … }` key, exact-path identity, the census sites), the freeze-gated `changelog` v1→v2 relocation, and the retirement of M37's render-to-root mechanism are the design of record in [storage.md](storage.md) (→ Placement) and [corpus-migration.md](corpus-migration.md) (→ Relocation) and are not restated here; the exact verb sequence is driven verbatim against the real binary by `crates/cli/tests/flow39_placement_layout.rs`. Notation illustrative.

This is the RC-trial layout: the flagship `vision` and the `CHANGELOG` live **where an agent and a human look for them** (repo root), managed with proper reconciliation — not a `docs/`-buried source plus a generated mirror. No prior flow put both root singletons in one repo; this is the composite that proves the convention holds at the composed surface.

### The walk — three managed docs, three homes, one `[dev ▸ methodology]` repo

```text
# A real git repo with `.jigc/config/packs.yaml` naming the on-disk methodology pack over the
# embedded dev base ([dev ▸ methodology]); two sibling root files already committed, unmanaged.
$ cat README.md CLAUDE.md          # `# Readme` / `# Claude` — plain root markdown, no doctype

# ── vision → the literal root `VISION.md` (a methodology placement doctype). ──
$ jigc start --workflow form-vision "form the project vision"
$ jigc doc create vision --title Vision --task form-the-project-vision   # → vision:vision (fixed slug)
$ jigc doc set-slot vision:vision#thesis         --from-file -
$ jigc doc set-slot vision:vision#invariants     --from-file -
$ jigc doc set-slot vision:vision#open-questions --from-file -
$ jigc task finalize form-the-project-vision
> promote: VISION.md               # managed directly at root (placement); H1 `# Vision` (display-title)
#   NO docs/vision/vision.md one-file-folder mirror — one file, at the literal home.

# ── changelog → the literal root `CHANGELOG.md` (a dev placement doctype, frozen v2). ──
$ jigc start --workflow record-change "cut the first release"
$ jigc doc create changelog --title Changelog                           # → changelog:changelog (fixed slug)
$ jigc doc add-item changelog:changelog#releases --title "1.0.0"        # emitted address drives downstream
$ jigc doc add-item <release>/changes --title added
$ jigc doc set-slot  <group>/notes --from-file -                        # "- OAuth device-code flow"
$ jigc task finalize cut-the-first-release
> promote: CHANGELOG.md            # managed directly at root (placement, schema-version 2); H1 `# Changelog`
#   NO docs/changelog/changelog.md one-file-folder mirror.

# ── roadmap → a direct `docs/roadmap.md` file (an internal methodology singleton). ──
$ jigc start --workflow planning "plan the first milestone"
$ jigc doc create roadmap --title Roadmap --task plan-the-first-milestone
$ jigc doc add-item roadmap:roadmap#milestones --title M-One --task plan-the-first-milestone
$ jigc doc set-slot <item>/proves        --from-file -
$ jigc doc set-slot <item>/decomposition --from-file -
$ jigc task finalize plan-the-first-milestone
> promote: docs/roadmap.md         # the docs/*.md placement layer — NO docs/roadmap/roadmap.md mirror

# ── The census owns exactly the three literal homes; siblings stay unmanaged. ──
$ jigc ingest
> VISION.md          adopted
> CHANGELOG.md       adopted
> docs/roadmap.md    adopted
> README.md          unmanaged     # a literal placement home owns ONE path — not a dir-glob sweep
> CLAUDE.md          unmanaged
```

### The reds — an OOB nonconformant edit to each root home is detected + routed

```text
# A human drops a required section heading in each managed root file, out of band, and commits.
$ sed -i 's/## Thesis/## Thesisz/'     VISION.md      # nonconformant edit
$ sed -i 's/## Releases/## Releasesz/' CHANGELOG.md   # nonconformant edit
$ git add VISION.md CHANGELOG.md && git commit -m "human edits the root docs out of band"

$ jigc ingest
> VISION.md          needs-reconcile   # a managed root file's OOB drift is detected + routed, like any managed doc
> CHANGELOG.md       needs-reconcile
> README.md          unmanaged         # the re-route does NOT sweep an unrelated sibling root .md into management
> CLAUDE.md          unmanaged
```

### What it asserts (the placement-convention acceptance bar — flow39_placement_layout.rs)

1. **The root singletons land at their literal homes.** `vision` is managed at the literal root `VISION.md` (H1 `# Vision`, display-title) and `changelog` at the literal root `CHANGELOG.md` (H1 `# Changelog`, schema-version 2) — each carrying its authored prose, and **neither** leaves a `docs/<x>/<x>.md` one-file-folder mirror (asserted absent at HEAD).
2. **The `docs/*.md` layer holds.** A methodology singleton (`roadmap`) is managed at the direct file `docs/roadmap.md` (no `docs/roadmap/roadmap.md` mirror) — the internal-singleton placement home, composition-invariant (the literal path is the home, `docs-root` never re-prepended).
3. **The two root placement doctypes coexist under one composition.** Both land in the *same* `[dev ▸ methodology]` repo (methodology `vision` + dev `changelog`), no collision — the RC-trial layout the greenfield trial runs on.
4. **Exact-path ownership — a literal home is not a glob.** `jigc ingest` reports the three managed docs `adopted` and the sibling root `README.md`/`CLAUDE.md` `unmanaged`: a placement home owns exactly its one declared `file:`, so a neighboring root `.md` is never vacuumed into management (rebutting the M16 B-1 dir-glob objection).
5. **A managed root file reconciles like any managed doc.** An out-of-band nonconformant edit to **each** root home (`VISION.md`, `CHANGELOG.md`) is detected + routed `needs-reconcile` by the census, while the unmanaged siblings stay untouched — the placement branch the M38 reconciliation sweep learned (Inc 2).

## 40. The M39 RC-findings wave, end-to-end — read surface · team-ready milestone record · relocation floor · slug + advisory (M39)

The M39 wave answers the RC greenfield trial's live findings ([completions/artifacts/RC-greenfield/trial-record.md](../completions/artifacts/RC-greenfield/trial-record.md)) — four features whose per-feature reds land in their own increments; this flow is the **composite acceptance** that ties them into one done-picture over the real binary (`crates/cli/tests/flow40_acceptance.rs`), composed the RC way (the on-disk **methodology** pack over the embedded **dev** base, `[dev ▸ methodology]`). The designs of record are elsewhere and not restated here: the read surface in [doc-read-surface.md](doc-read-surface.md), the milestone record + the `.jigc`-is-the-workbench principle in [team-ready-state.md](team-ready-state.md), the freeze-exempt relocation floor in [storage.md](storage.md) (→ What M39 closes) + [corpus-migration.md](corpus-migration.md) (→ The freeze-exempt sibling), and the slug mint in [structural-grammar.md](structural-grammar.md) (→ Open questions). Notation illustrative.

### The walk — four arms, one wave

```text
# ── F1 · the read surface: a fresh session reads + revises a committed vision, all groundings in view. ──
$ jigc doc create research --title "Cache Benchmarks"   # → research:cache-benchmarks   (+ finalize)
$ jigc doc create research --title "Sharded Writes"     # → research:sharded-writes     (+ finalize)
$ jigc start --workflow form-vision "form the project vision"
$ jigc doc create vision --title Vision --task form-the-project-vision   # → vision:vision
$ jigc doc set-field vision:vision#meta/grounded-in "[research:cache-benchmarks, research:sharded-writes]"
$ jigc start --task form-the-project-vision            # RE-COMPOSE
> …A single node caps throughput under contention.     # BOTH groundings' findings render, not just the first
> …Sharding removes the write-contention ceiling.      #   (the cardinality-n edge-walk fix — the trial's #1 gap)
$ jigc task finalize form-the-project-vision           # → promote VISION.md (both grounded-in targets resolve)
$ jigc doc show vision:vision                           # the M39 read surface — the committed doc, byte-exact
> # Vision …thesis… grounded-in: [research:cache-benchmarks, research:sharded-writes]

# ── F1/team-ready · the milestone record: create → add-task ×2 → join, then a FRESH CLONE continues. ──
$ jigc milestone create "Cache rework"                  # → milestone:cache-rework  (base pin recorded)
$ jigc milestone add-task cache-rework "Warm the read cache"    # append a tasks item (status: active)
$ jigc milestone add-task cache-rework "Evict cold entries"     # append a second (each a path-scoped record commit)
$ jigc milestone finalize cache-rework                  # join: flip every item + header status → joined, byte-stable
> commit docs/milestone-records/cache-rework.md         #   the record's status-flip folds INTO the join commit

# … a teammate clones; NO `.jigc/` working state travels …
$ rm -rf .jigc                                          # simulate the fresh clone — drop ALL workbench state
$ jigc doc show milestone-record:cache-rework --format json
> { "type": "milestone-record", "slug": "cache-rework", "status": "joined",
>   "sections": { "tasks": [ { "task-id": "warm-the-read-cache", "intent": "Warm the read cache", … },
>                            { "task-id": "evict-cold-entries",  … } ] } }   # the pinned 1.0 shape
$ jigc milestone list-tasks cache-rework                # continue: re-derives the demoted cache from the record
> warm-the-read-cache   evict-cold-entries              #   .jigc/milestones/cache-rework/{base,tasks}.json re-seeded

# ── the relocation floor: a stranded freeze-exempt instance is detected + moved, never silently lost. ──
$ git mv research/cache-benchmarks.md docs/legacy-research/cache-benchmarks.md   # a pre-convention strand
$ jigc relocate research --from docs/legacy-research/
> 1 moved   docs/legacy-research/cache-benchmarks.md -> research/cache-benchmarks.md   # byte-preserving git mv

# ── slug + advisory: a long intent caps readably; --slug overrides; empty research nudges do-research. ──
$ jigc start --workflow single-task "move the session cache to a shared redis cluster"
> task: move-the-session-cache-to                       # ≤5-word word-boundary cap (was mid-word truncation)
$ jigc start --workflow single-task "move the session cache …" --slug redis-cache
> task: redis-cache                                     # --slug sets identity verbatim
$ jigc start --workflow form-vision "form the project vision"   # over an EMPTY research store
> consider running `do-research` first — a vision grounds in research   # advisory, NON-blocking
> Run: jigc doc create vision …                          #   form-vision still composes fully
```

### What it asserts (the M39-wave acceptance bar — flow40_acceptance.rs)

1. **The read surface serves + re-composes every grounding (F1).** After grounding a `vision` in **two** committed `research` docs, the resume re-compose renders **both** groundings' findings into the guidance (the cardinality-n edge-walk, not just the first), finalize resolves both `grounded-in` targets in one commit, and `jigc doc show vision:vision` serves the committed doc byte-exact carrying the thesis + both targets.
2. **The team-ready arc joins byte-stable and continues from a fresh clone.** `create → add-task ×2 → finalize` yields the committed `docs/milestone-records/cache-rework.md` with the base pin + both sub-tasks; `join` flips every item's + the header `status` to `joined` (no `active` survives), folded into the join commit. After `rm -rf .jigc`, `jigc doc show milestone-record:cache-rework --format json` returns the pinned 1.0 shape and the next milestone op re-derives `.jigc/milestones/<id>/{base,tasks}.json` from the committed record — the source-of-truth demotion made true.
3. **The freeze-exempt relocation floor moves a stranded instance.** A committed `research` instance `git mv`'d to a prior home is detected + moved back to its current schema home by `jigc relocate research --from <prior>` (reported `1 moved`, on disk, byte-preserving) — never silently stranded.
4. **Slug capping + `--slug` + the empty-research advisory.** A 9-word intent mints the `≤5`-word word-boundary-capped slug; `--slug` sets identity verbatim; and `form-vision` composed against an **empty** research store surfaces the non-blocking `do-research` advisory while still composing fully.

## 41. The M40 rc.4 wave, end-to-end — transactional finalize · ingest visibility · retitle-item · `doc schema` · a methodology migration · the stamped corpus (M40)

The M40 wave answers the RC adoption trial's (migration-half) verified findings ([completions/artifacts/RC-adoption/trial-record.md](../completions/artifacts/RC-adoption/trial-record.md)) — increments 1–7 prove each feature per-feature; this flow is the **composite acceptance** tying them into six done-picture arms over the real binary (`crates/cli/tests/flow41_acceptance.rs`). The designs of record are elsewhere and not restated here: the finalize transaction in [finalize.md](finalize.md) + [auto-migration.md](auto-migration.md) (→ the transaction mechanism); item identity + the reslug guards in [write-commands.md](write-commands.md) (→ `jigc doc retitle-item` · Placement singletons · Milestone-record reslug) and [storage.md](storage.md) (→ Identity); ingest + the adoption advisories in [project-setup.md](project-setup.md) (Flow-2) + [validation.md](validation.md); the `doc schema` contract in [doc-read-surface.md](doc-read-surface.md) (the third surface); methodology schema versioning in [corpus-migration.md](corpus-migration.md) (→ The freeze-exempt sibling, the M40 revision) + [doctype-map.md](../implementation/doctype-map.md) (→ the scope pin); the seven migrate workflows in [auto-migration.md](auto-migration.md). Arms 1–4 and 6 run the embedded dev pack; arm 5 composes `[dev ▸ methodology]`. Notation illustrative.

### The walk — six arms, one wave

```text
# ── Arm 1 (F7) · a pre-staged `git rm` migration finalize lands ONE clean commit. ──
$ jigc migrate HISTORY.md --as changelog               # foreign Keep-a-Changelog, tracked
$ jigc doc author changelog --from-file - --task migrate-changelog-history
$ git rm HISTORY.md                                    # the USER pre-stages the retirement
$ jigc task finalize migrate-changelog-history --approve
> commit …                                             # no stage-phase fatal — the retirement
$ git show --name-status HEAD                          #   pathspec is discriminated on the INDEX
> A  CHANGELOG.md                                      # exactly ONE whole-index commit carries the
> D  HISTORY.md                                        #   promoted doc AND the pre-staged deletion

# ── Arm 2 (F8+F4) · gitignored trees never poison ingest; a hollow adopt is annotated. ──
$ jigc ingest        # tree carries node_modules/pkg/README.md (gitignored) + a hollow docs/roadmap.md
> …                  # NO node_modules row — candidates come from `git ls-files --exclude-standard`
> docs/roadmap.md   roadmap   adoptable → adopted — structurally empty: 0 milestones
>                    # the pinned repeatable-populated triage annotation; the verdict never flips

# ── Arm 3 (F5/F6) · a committed component retitles; the {#id} anchor and every inbound address survive. ──
$ jigc doc retitle-item arch-doc:cache-layer#components/session-store --title "Session vault"
# staged diff vs committed: EXACTLY the heading-title bytes —
#   "### Session store  {#session-store}"  →  "### Session vault  {#session-store}"
$ jigc doc set-slot arch-doc:cache-layer#components/session-store/description --from-file -
> ok                                                   # the SAME item address still lands post-retitle
$ jigc task finalize retitle-the-store                 # re-commits the doc, anchor frozen

# ── Arm 4 (F1) · `doc schema` returns the separately-pinned contract shape (contract-version 2). ──
$ jigc doc schema adr --format json
> { "contract-version": 2, "type": "adr", "schema-version": 2,
>   "fields": [ { "id": "status", "of": ["proposed","accepted","superseded"], …, "section": "status" },
>               …, { "id": "schema-version", "author-required": false, …, "section": "status" } ],
>   "sections": [ { "id": "context", … }, { "id": "options", "optional": true, … },
>                 { "id": "decision", … }, { "id": "consequences", … } ] }

# ── Arm 5 (F2) · a methodology migration lands the root VISION.md.  [dev ▸ methodology] ──
$ jigc migrate old-vision.md --as vision               # the composed guidance carries `doc author vision --from-file`
$ jigc doc author vision --from-file - --task migrate-vision-old-vision
> vision:vision                                        # the singleton mints at the fixed slug = the type id
$ jigc task finalize migrate-vision-old-vision --approve
> commit …                                             # ONE commit: A VISION.md (root placement home,
>                                                      #   `# Vision` display-H1) + D old-vision.md

# ── Arm 6 (A1) · an unstamped methodology corpus is detected, stamped, and re-validates CLEAN. ──
$ jigc validate                                        # over a committed v0 (no schema-version) research doc
> … schema-conformance … route: migrate                # detected + routed, report-only exit 0
$ jigc migrate-corpus
> corpus migration: 1 migrated, 0 already current, 0 blocked
>   migrated   docs/research/cache-strategy.md         # appends `schema-version: 1`, bytes otherwise untouched
$ jigc validate
> clean                                                # no schema-conformance finding, no migrate route left
```

### What it asserts (the M40-wave acceptance bar — flow41_acceptance.rs)

1. **The pre-staged `git rm` finalize lands one clean commit (F7).** A user who pre-staged the foreign original's retirement before finalize still lands the approved migration: the retirement pathspec is discriminated on the **index**, not HEAD, so the stage phase never fatals — exactly one whole-index commit carries the promoted `CHANGELOG.md` **and** the pre-staged `D HISTORY.md`.
2. **Gitignored trees never enter the funnel; a hollow adopt is annotated (F8+F4).** A gitignored `node_modules/**.md` is absent from the triage report (the candidate set is `git ls-files --cached --others --exclude-standard -- '*.md'`), while a structurally hollow roadmap at its placement home still **adopts** — its row carrying the pinned `repeatable-populated` annotation (`adopted — structurally empty: 0 milestones`), never a flipped verdict, never silence.
3. **Retitle-item round-trips with identity intact (F5/F6).** The staged copy differs from the committed bytes in exactly the heading-title bytes (the `{#id}` anchor byte-frozen), a follow-up `set-slot` at the **same** item address lands (every inbound address survives), and finalize re-commits the retitled doc clean.
4. **`doc schema` serves the pinned contract (F1).** `jigc doc schema adr --format json` returns contract-version 2 (the M41 rc.5 bump adding per-field `of` enum members + the field→section mapping) with the doctype's frozen `schema-version: 2`, the loader-injected `schema-version` stamp field rendered `author-required: false`, and the four adr sections in schema order with `options` flagged optional. (The byte-verbatim goldens live in `doc_schema.rs`.)
5. **A methodology migration lands the root vision (F2/F10).** `jigc migrate old-vision.md --as vision` under `[dev ▸ methodology]` composes the batch-author guidance; the approved finalize lands exactly one commit writing the managed singleton at the repo-root literal `VISION.md` (the placement home, `# Vision` display-H1) and retiring the foreign original.
6. **The stamped corpus validates clean (A1).** A committed unstamped (v0) methodology doc — frozen by the M40 methodology manifest — is detected + routed `migrate` by `jigc validate`; `jigc migrate-corpus` appends exactly the `schema-version: 1` stamp line (bytes otherwise unchanged); the re-validate runs clean — no `schema-conformance` finding, no `migrate` route left.

## 42. The M41 rc.5 wave, end-to-end — folded-slot fidelity · the driver contract · the stable finding key · schema read · a value-remap · the first real-binary Vue proof · the optional-scalar clear (M41)

The M41 rc.5 wave answers the RC adoption rerun trial's (rc.4) verified findings ([completions/artifacts/RC-adoption/rerun-rc4/trial-record.md](../completions/artifacts/RC-adoption/rerun-rc4/trial-record.md)) — increments 1–8 prove each feature per-feature; this flow is the **composite acceptance** tying them into seven done-picture arms over the real binary (`crates/cli/tests/flow42_acceptance.rs`). Every arm re-exercises a design of record owned elsewhere and not restated here: the fold-safe migration slot templates in [auto-migration.md](auto-migration.md) (V1); the pinned `--format json` of composed output, write-acks, and findings — task id, decomposed `target`, findings-as-data, and the stable `(code, target)` key + the advisory-route floor — in [command-output-contract.md](command-output-contract.md) (F1, F2/finding-key) and [validation.md](validation.md) (→ the advisory-route floor); the schema projection in [doc-read-surface.md](doc-read-surface.md) (V3/V6, the `doc schema` surface joined by the enum-members + field→section keys); the enum-member value-remap kind in [corpus-migration.md](corpus-migration.md) (F4); symbol-granular Vue in the code-anchor gate in [validation.md](validation.md) (→ Multi-language resolution / Vue addressable units) + [ideas/multi-language-doc-code.md](../ideas/multi-language-doc-code.md) (F3); the `--unset` verb in [write-commands.md](write-commands.md) (V5). The **Vue arm is the wave's one genuinely first-covered-here proof** — the prior increments proved the SFC `<script>` extract at the probe-unit level, and this is its first drive through the real finalize gate. Arms V1, F1, F2, V3/V6, F3, V5 run the embedded dev pack; arm F4 composes `[dev ▸ methodology]`. Notation illustrative.

### The walk — seven arms, one wave

```text
# ── Arm V1 · a fold-safe migration slot template round-trips multi-line prose intact. ──
$ jigc migrate docs/adr/0001-fidelity.md --as adr     # the composed `doc author` skeleton's slots are `|-` block scalars
$ jigc doc author adr --from-file - --task migrate-adr-docs-adr-0001   # a multi-paragraph + bulleted body filled in
$ jigc task finalize migrate-adr-docs-adr-0001 --approve
# the committed decisions/…md keeps every `\n` — a folding flow scalar would collapse the bullets to one line

# ── Arm F1 · the driver contract: `start` carries `task`; a write-ack decomposes `target`. ──
$ jigc start --format json "add rate limiter"         # the router arm (creates-task: false)
> { "task": null, "text": … }                          # `task` present-and-null, no id minted
$ jigc start --format json --workflow single-task "add rate limiter"
> { "task": "add-rate-limiter", "text": … }            # a work-mint carries the id structurally
$ jigc doc set-field commit:add-rate-limiter#type --value feat --format json
> { "op": "set-field", "target": { "doctype": "commit", "slug": "add-rate-limiter",
>     "section": "header", "leaf": "type" }, "value": "feat", "findings": [] }

# ── Arm F2 (finding-key) · a `0..*` two-dangling-ref sweep fans two uniquely-keyed findings. ──
$ jigc validate --format json    # over a committed adr with `supersedes: [adr:ghost-one, adr:ghost-two]`
> { "findings": [
>     { "code": "schema-conformance.ref-resolves",
>       "key": { "code": "…ref-resolves", "target": "adr:shared-redis-session-cache#supersedes/ghost-one" },
>       "route": "fix the reference, create the target in this task, or drop the field", … },
>     { …, "key": { …, "target": "adr:shared-redis-session-cache#supersedes/ghost-two" }, "route": … } ],
>   "report_only": true }                              # two DISTINCT keys; EVERY finding routes (never null); exit 0

# ── Arm V3/V6 · the schema read surface projects enum members + field→section. ──
$ jigc doc schema adr --format json
> { "contract-version": 2, …,
>   "fields": [ { "id": "status", "of": ["proposed","accepted","superseded"], …, "section": "status" }, … ] }

# ── Arm F4 · the first methodology v1→v2 value-remap, byte-faithful.  [dev ▸ methodology] ──
$ jigc migrate-corpus       # over a committed v1 docs/deferral-ledger.md carrying `kind: D` + `kind: I`
> corpus migration: 1 migrated …                       # `D`→`Decision`, `I`→`Idea`, stamp 1→2, every other byte preserved —
>                                                       #   byte-faithful to the real v2 canonical oracle

# ── Arm F3 · the first real-binary Vue proof: a fabricated `.vue` script symbol blocks. ──
$ jigc doc set-field arch-doc:app-shell#components/layout/implemented-by --value src/AppLayout.vue#DoesNotExist
$ jigc task finalize document-the-app-shell
> … doc-code.symbol-exists … BLOCKED                   # the SFC's <script setup> is parsed under the vendored TS grammar
$ jigc doc set-field arch-doc:app-shell#components/layout/implemented-by --value src/AppLayout.vue#useCounter
$ jigc task finalize document-the-app-shell            # a real composable resolves — the SAME finalize passes

# ── Arm V5 · `--unset` clears an optional scalar and the doc re-conforms. ──
$ jigc doc set-field adr:anchor-decision#status/cites-code --unset --format json
> { "op": "set-field", "unset": true, … }              # the `cites-code:` line is gone; a follow-up write re-populates it
```

### What it asserts (the M41-wave acceptance bar — flow42_acceptance.rs)

1. **Folded-slot template fidelity (V1).** A foreign ADR migrates through `jigc migrate --as adr`; the composed author skeleton's `|-` block-scalar slots keep a multi-paragraph + bulleted body's line breaks through the `jigc doc author` → finalize round-trip — the committed prose carries the body verbatim (a folding flow scalar would collapse the breaks). The task id is parsed from the emitted `--task <id>` verb, never reconstructed.
2. **The driver contract (F1).** `jigc start --format json` carries `task` — present-and-`null` on the `creates-task: false` router arm, the minted slug on a work-workflow — and a `jigc doc set-field --format json` write-ack decomposes its address into `op` + `target{doctype, slug, section, leaf}` + a literal empty `findings` envelope.
3. **The stable finding key + the advisory-route floor (F2/finding-key).** A committed adr whose `0..*` `supersedes` carries two dangling targets fans exactly two `schema-conformance.ref-resolves` findings with **distinct** `(code, target)` keys (`adr:<slug>#supersedes/<to-slug>` — one keyed finding per dangling target), and **every** finding the store sweep emits carries a non-null `route` (the V15 / Fork-2 floor). Store-scope `validate` is report-only, so it exits 0 with the blocking content findings present.
4. **The schema read surface (V3/V6).** `jigc doc schema adr --format json` (contract-version 2) projects the enum members (`of`) on the `status` field and the top-level field→owning-`section` mapping.
5. **The first methodology v1→v2 value-remap (F4).** `jigc migrate-corpus` remaps a committed v1 `deferral-ledger`'s `kind: D` → `kind: Decision` (and `I` → `Idea`) byte-faithful to a real v2 canonical oracle — the enum rename + the stamp bump `1`→`2` are the whole delta; every other byte is preserved.
6. **The first real-binary Vue proof (F3).** A committed arch-doc component whose `implemented-by` anchor names a **fabricated** `.vue` script symbol **blocks** at the finalize gate (`doc-code.symbol-exists`, exit non-zero — the SFC's `<script setup>` is extracted and parsed under the vendored TypeScript grammar); re-pointed at a **real** composable (`useCounter`), the same finalize passes.
7. **The optional-scalar clear (V5).** `jigc doc set-field <addr> --unset` clears an optional adr header scalar (`cites-code`) — the field line is gone and the ack names the clear (`unset: true`) — and the doc re-conforms: a follow-up `set-field` over the now-absent field lands.
