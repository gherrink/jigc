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

The pack-default `single-task` include list is `[locate, implement, superseded-context, finalize]`. The project wants its own `implement` step that augments the pack's with a house lint reminder — without forking the pack step (it re-includes it).

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

The adapter launches them as **concurrent Task-tool sub-agents**. Each sub-agent runs `jigc workflow sub-task --task <sub>` — which provisions its write-ready area on first entry — implements, authors its own `commit` doc, creates any ADR through the create-gate, and writes only into its own `tasks/<sub>/` area (the **write-time `--task` barrier** refuses anything else). A sub-agent never runs git. It acks `status + task_id`; the orchestrator re-derives all state from the CLI, never the message (the blackboard). Then the barrier-join-finalize sequence from flow 9 runs, gated by `finalize.fan-out.squash` (default `true` → one aggregate commit).

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
3. **Genuinely spawn ≥2 concurrent sub-agents.** The orchestrator's **Task tool launches the sub-agents concurrently**, one per emitted directive. Each sub-agent re-enters via `jigc workflow sub-task --task <sub>` (which provisions its write-ready `tasks/<sub>/` area on first entry), implements, authors its own `commit` doc, creates any ADR through the create-gate, and **writes only through `jigc` into its own area** — the write-time `--task` barrier refuses anything else. A sub-agent never runs git and never reads a sibling's area; it acks `status + task_id` only.
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
