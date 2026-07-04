# Finalize transaction

The ordered transaction that turns a task's working area into a committed change. `finalize` is the site that turns **task** work into a commit — everything on the task path is staging, and the one transition between staged and committed happens here. Specifying it as a single transaction keeps the rest of the system honest: read paths see committed state, write paths stage into the task area, and the one transition between them happens here. *(Two task-less direct-store ops touch the committed store outside this boundary, each with its **own** transaction + rollback: `migrate-corpus` ([corpus-migration.md](corpus-migration.md)) and `jigc rename` ([reconciliation.md](reconciliation.md) → Rename detection, the transaction). They are not staged-then-promoted — they mutate committed paths directly — so they define their own atomicity rather than riding finalize's.)*

Builds on [VISION.md](../VISION.md) (the commit boundary; the determinism boundary), [write-commands.md](write-commands.md) (`finalize ≡ validate + commit`; the task as staging unit; the two check times), [validation.md](validation.md) (the validate phase and its probes), [storage.md](storage.md) (the per-task working area, derived caches, "CLI orchestrates, git executes"), [workflow-dialect.md](workflow-dialect.md) (`fan-out`/`join` and sub-agent finalization), and [document-type-schema.md](document-type-schema.md) (the commit doc type). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## What `finalize` is

`finalize ≡ validate(task) + commit`. One task → one logical commit. The shape:

```
jigc task finalize <task-id>
  → validate                (blocking findings abort cleanly)
  → render commit-doc       (→ git message string)
  → promote managed docs    (working area → canonical paths)
  → stage                   (git add: promoted docs + first-commit config, into the agent-staged index)
  → git commit              (pre-commit hook may reject)
  → post-commit             (invalidate caches, update hashes, cleanup)
```

The transaction is **atomic up through `git commit`**: any failure before that point rolls back to "as if `finalize` was never called" — the working area is intact, canonical paths unchanged. After `git commit` succeeds, post-commit steps are **best-effort** with cache stamps absorbing partial failures (a failed hash update only means the next read rebuilds the cache).

## The seven phases

### 1. Preflight

Cheap rejections before any expensive work:

- **Task exists.** `.jigc/tasks/<task-id>/` must exist; otherwise reject with "no task `<id>`".
- **Base pin matches HEAD.** The task was started at base `<A>`; if HEAD ≠ `<A>`, reject with the divergence-routing prompt ("switch back to `<A>` or discard") — same philosophy as [out-of-band reconciliation](write-commands.md#out-of-band-reconciliation). The CLI never operates a task off its pinned base.
- **Git in a committable state.** No in-progress merge/rebase/bisect. The CLI does *not* require a clean working tree (see [Dirty-tree policy](#dirty-tree-policy)).

### 2. Validate

Runs `validate(task)` ([validation.md](validation.md)). Three outcomes:

- **Blocking findings present** → abort with the finding list. No side effects. Agent fixes and re-runs `finalize`.
- **Advisory findings only** → surface them in the output, continue.
- **No findings** → continue silently.

`finalize` has no private check path: what `validate` reports is what `finalize` blocks on. They cannot diverge ([write-commands.md](write-commands.md) → Staging and the transaction model).

### 3. Render the commit message

The commit doc instance (`commit:<task-id>`) is rendered into a git-message string. The renderer is **schema-driven** and **cascade-overridable**; the pack-default for the development pack follows Conventional Commits — full mapping in [Commit-doc rendering](#commit-doc-rendering) below.

This phase has **no disk side effects** — it returns a string. A missing required slot would have surfaced at phase 2 (`validate` blocks on required-slot presence); the renderer trusts validate.

### 4. Promote managed docs

For each managed doc instance in the task's working area (`.jigc/tasks/<task-id>/*.md`):

- **Copy** (not move) the file from the working area to its canonical path. The path comes from the doc type's `location:` field ([document-type-schema.md](document-type-schema.md) → On-disk definition format): an ADR lands in `decisions/`, a SPEC in `specs/`, and so on.
- **Placement doctypes promote to their literal home (M38).** A doctype declaring `placement: { file: … }` has `location: None`, so it promotes to its exact `placement.file` — a repo-root literal (`VISION.md`, `CHANGELOG.md`) or a direct `docs/roadmap.md` — never `<location>/<slug>.md` ([storage.md](storage.md) → Placement). `plan_promotions` computes this branch on both the single-task and fan-out-join paths. This is how M37's vision-render mechanism is **retired**: the `vision` is *promoted (managed) at root* `VISION.md` — one file, proper reconciliation — not regenerated as a mirror of a docs-buried source.
- The commit doc (`commit:<task-id>`) is **not** promoted — its sink is the git message rendered in phase 3, not a repo file ([write-commands.md](write-commands.md) → Instance provisioning).

Promotion is **copy, not move**, so rollback (if a later phase fails) is removal of the copies — not a complex undo. The working area stays intact until phase 7.

### 5. Stage

The agent has already `git add`ed its own code edits as it worked (the **agent-stage contract** — [Dirty-tree policy](#dirty-tree-policy)). This phase stages **jigc's own** managed artifacts into that existing index:

- every promoted doc (canonical paths from phase 4),
- on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`).

The commit set is therefore **the git index** — the agent's staged code plus jigc's just-staged docs/config — committed whole (no curated pathspec, so an agent-staged owner-artifact like `completions/artifacts/**` rides along). finalize **never** runs `git add --all`: untracked and unstaged working-tree changes stay out of the commit and are surfaced (see [Dirty-tree policy](#dirty-tree-policy)).

### 6. Commit

`git commit -F <rendered-message-file>` (a temp file, deleted after). The CLI **never** passes `--no-verify` — the user's `pre-commit` and `commit-msg` hooks are policy and the CLI respects them.

Outcomes:

- **Commit succeeds** → **relay any hook output to the agent**, then proceed to phase 7. A successful `git commit` can still produce hook output (a *non-blocking* `pre-commit`/`commit-msg` hook that warns but exits 0 — e.g. the M19 doc↔code backstop, below); the CLI surfaces that captured stdout/stderr to the agent on success, not only on rejection (M19 — resolves the [open question](#open-questions) below). Without this the warning is silently swallowed and the agent never sees it. **Placement + scope (M19 review S1):** the relay is **general** — git exposes one combined hook stream, so the CLI cannot single out jigc's own backstop from a user's linter/formatter hook; *all* non-blocking hook output is relayed. It is emitted in a **clearly delimited section after** the success result and **never inside** the routing footer or the `--format json` envelope (an agent parsing structured output must not have it corrupted). This is a deliberate behavior change: a verbose user hook that previously had its success output swallowed by finalize now surfaces it — the honest fix (an agent committing through jigc *should* see its hooks speak), recorded here so it isn't a surprise.
  - **Fan-out finalize relays every fan-out commit (M19 review B1; rewired M31 Inc 5).** Both fan-out commit paths run the user's hooks and relay their output. Under **`squash: false`** there are **N+1** commits — one per *code-carrying* sub-task (each carrying that sub-task's own worktree code) plus the aggregate `git_commit` that promotes the merged docs + config; the user's `pre-commit`/`commit-msg` hooks fire on **each** and the success-relay surfaces **each** commit's output. Under **`squash: true`** there is one aggregate commit — the off-line combined tree committed with hooks running from a dedicated worktree ([§ `fan-out` finalize](#fan-out-finalize)) — and its hook output is relayed. A warn-only doc↔code backstop therefore runs the whole-store sweep once per fan-out commit (redundant across the N+1 `squash: false` commits but harmless; warn-only never aborts). *(A future optimization could skip the sweep on an unchanged diff; named so it isn't silently assumed.)*
- **Hook rejects, or other git error** → roll back: `git restore --staged --worktree <promoted-paths>` (restores HEAD content for promoted paths), then delete the promoted copies. The working area is untouched; the agent sees the hook's stderr verbatim and re-runs `finalize` after fixing. The hook output **is** the correction signal — never bypassed.

**The M19 doc↔code backstop fires here (and is warn-only by construction).** `jigc setup` can install an assistant-neutral `pre-commit` hook that runs `jigc validate` (the store-wide doc↔code sweep) — so it fires on *this* commit too. It is **warn-only** (always exits 0, surfacing stale-anchor findings but never rejecting): a *blocking* store-sweep here would reject `finalize`'s commit for **unrelated pre-existing drift the task never touched** — re-introducing the masking-trap M18 designed `jigc validate` to avoid ([validation.md](validation.md) → Store-scope re-validation), and self-gating `finalize` on whole-store drift. So the backstop's output reaches the agent via the success-relay above, never as a `finalize` block. ([assistant-adapter.md](assistant-adapter.md) → neutral install; [roadmap.md](../implementation/roadmap.md) → M19.)

### 7. Post-commit (best-effort)

After the commit lands, three updates happen — none can affect commit truth, all self-heal if they fail:

- **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches). The next read rebuilds from the new HEAD.
- **Update `file-state` hashes** for every committed managed doc. The next `file-state` probe sees them as in-sync.
- **Remove `.jigc/tasks/<task-id>/`.** The staging area's job is done; it persisted until here so rollback was possible across phases 4–6.

A failure in any of these is **logged**, not raised — the commit is real, the system reads correctly because the edge-index stamp invalidates eventually, and a stale `.jigc/tasks/<id>/` gets cleaned up by `jigc task discard <id>` or the next `finalize` reusing the slot.

## Dirty-tree policy

A task is pinned to base commit `<A>` at start. **The commit set is the git index, not a sweep of the working tree** ([DECISIONS.md](../DECISIONS.md) 2026-06-20 → M30; this inverts the pre-M30 "everything differing from `<A>`" sweep). Under the **agent-stage contract**, the agent `git add`s its own code edits as it works; at `finalize` the CLI stages **its own** managed artifacts into that same index — the promoted docs (phase 4) and, on a first commit, the project config layer (`.jigc/config`, `.jigc/.gitignore`) — then commits the whole index as one logical change. The agent stages code; the CLI commits the index; neither places what the other owns ([write-commands.md](write-commands.md) → Staging and the transaction model).

What this means for the working tree:

- **No `git add --all`.** Untracked files and unstaged tracked edits are **left out** of the commit — they are not the task's work until the agent stages them. The index *is* the declared touch-set, expressed by `git add`, not inferred from the tree's delta from base.
- **Left-out WIP is surfaced, not committed (and not refused)** — a landed `finalize` names what it left behind (see *Surfaced, not prevented*, below), so a stray `scratch.txt` stands out rather than riding the commit silently.
- **Block on nothing staged.** If the narrowed commit set is empty while the tree is dirty, `finalize` blocks (*"you staged nothing — `git add` your changes"*) rather than sweeping unrelated WIP in to avoid an empty commit (see [Empty commit](#commit-doc-rendering), below).
- **Parallel hand-editing** (a human editing other files on the same branch while a task runs) is caught upstream by the base-pin: if the human commits, HEAD moves and the base-mismatch rejection in phase 1 fires; if the human only edits without committing, the changes show in `jigc task diff <id>` before `finalize` and stay out of the commit unless the agent stages them.

  **Amended at M17 planning (2026-06-12) — phase 1 gains a re-pin path.** The unconditional rejection proved wrong for serial-task shapes the methodology itself mandates: a completion task minted at audit-start finalizes *after* its fix commits land, so its base has moved **by design**, and discard-and-reauthor was the only route (verified by exercise on a foreign repo). Phase 1 now **auto-re-pins** when the moved history is disjoint from the task's work: re-pin to the new HEAD iff *(paths changed in commits between the recorded base and HEAD)* ∩ *(currently-dirty working-tree paths ∪ the task's promote destinations)* = ∅, then re-run the preflight sweep against the new base. Any overlap keeps the block — now with a conflict route naming the overlapping paths — which is exactly the parallel-hand-editing case this bullet exists to catch. ([DECISIONS.md](../DECISIONS.md) 2026-06-12; the M17 pre-fix set.)

The benefit is precision: the commit carries exactly what the agent staged plus the docs/config jigc manages — no hidden allow/deny list the agent must reason about, and no unrelated working-tree WIP silently riding along.

**The staging policy — per-task, migration, and fan-out combine (M30/M31).** The index-honoring scope above is the **per-task** finalize policy (`StagePolicy::IndexHonoring`; a migration task uses the fixed `MigrationFixed` narrowing). The **milestone fan-out** isolates each sub-agent's code in its **own git worktree** (M31; [storage.md](storage.md#cli-and-git) → the third combine mode), so a fan-out commit reads sub-agent code **from the worktrees**, never from a `git add --all` sweep of the shared checkout — the whole-tree `Sweep` is **retired** (under worktree isolation it dropped every line of sub-agent code, the data-loss the redesign fixes). The two squash modes diverge: **`squash: true`** folds the N worktrees' staged code-sets into one combined tree **off-line** (`StagePolicy::Combine`), then commits that tree from a clean dedicated worktree so the user's hooks run against it WIP-safely ([§ `fan-out` finalize](#fan-out-finalize)); **`squash: false`** commits each sub-task's worktree code per-sub-task (hooks run + relay on each) and then promotes the merged docs through `IndexHonoring` over the now-clean index. So the staging policy is three-way — per-task non-migration → `IndexHonoring`, per-task migration → `MigrationFixed`, `squash: true` fan-out → `Combine` ([DECISIONS.md](../DECISIONS.md) 2026-06-20 → M30, the SPLIT + G1; 2026-06-21 → M31 Inc 5).

**Surfaced, not prevented (B1, [DECISIONS.md](../DECISIONS.md) 2026-06-19; narrowed at M30).** A landed `finalize` emits a **change-set manifest** — the committed set (the index: the agent's staged code + the promoted docs + first-commit config), each path tagged by how it entered (promoted / modified / deleted / added — a deliberately-staged new file), **plus a distinct `left-out` list** naming what the commit excluded (untracked files + unstaged tracked edits) so a stray `scratch.txt` stands out as left-behind rather than swept in. A staged-then-further-modified path appears in **both** lists — its staged bytes commit, its later worktree delta is left out. `jigc task finalize <id> --dry-run` prints that manifest and stops — committing nothing, no destructive side effect (and needing no `--approve` on a migration task). This *surfaces* the set; it does not refuse anything beyond the empty/nothing-staged block.

## Rollback discipline

Per-phase failure inventory:

| phase | failure → | rollback |
|---|---|---|
| 1 preflight | reject before side effects | none needed |
| 2 validate | blocking findings | none needed |
| 3 render | rendering error | none needed (no disk writes yet) |
| 4 promote | copy fails (disk full, permissions) | delete copies made so far; working area intact |
| 5 stage | `git add` fails (rare) | unstage; delete promoted copies; working area intact |
| 6 commit | hook rejects, git error | `git restore --staged --worktree` promoted paths; delete promoted copies; working area intact |
| 7 post-commit | hash update / cleanup fails | **commit is real;** log, self-heal via cache stamps |

**Before phase 6, all-or-nothing.** After phase 6, the commit defines truth and any post-commit residue is a chore, not a transaction issue.

## `fan-out` finalize

A `fan-out` workflow's parent task's `finalize` is the commit boundary; sub-agents **stage code in their own worktree and author their managed docs, but never commit** ([workflow-dialect.md](workflow-dialect.md#fan-out--join)). The two substrates isolate differently (M31; [storage.md](storage.md#cli-and-git) → the three combine modes): a sub-agent's **code** lives in its own git worktree (`.jigc/worktrees/<sub-id>`, `git add`ed there), while its **managed docs** are written into its directory-isolated `tasks/<sub>/` area. The transaction expands:

1. **Combine the substrates into the parent.** **Docs** merge by the deterministic by-task-id join — by task-id order; slug collisions resolve via the deterministic suffix ([workflow-dialect.md](workflow-dialect.md#fan-out--join), [storage.md](storage.md#the-by-task-id-join-m7)). **Code** is combined by reading each worktree's staged set in task-id order with **disjoint-apply + block-on-collision** (a cross-worktree same-file write blocks + routes the named paths, never `git merge`; [storage.md](storage.md#cli-and-git) → the third combine mode) — checked **up front**, before any commit.
2. **Validate** the merged effective state — one validate run, not N.
3. **Render, promote, stage, commit — gated by the `finalize.fan-out.squash` cascade knob.** M7 shipped only the synthesized single-commit form; **M8 added per-sub-task authored commit docs and the knob** ([DECISIONS.md](../DECISIONS.md) 2026-06-04 → M8 Settle); **M31 made both paths carry the worktree-isolated code and run the user's hooks** ([DECISIONS.md](../DECISIONS.md) 2026-06-21 → M31 Inc 5). The two modes:
   - **`squash: true` (default) — one aggregate commit.** The CLI folds the N worktrees' staged code-sets into one combined tree **off-line** (a throwaway index), overlays the promoted docs + first-commit config, and commits that tree from a **clean dedicated worktree** — where `git commit -F` (never `--no-verify`) runs the user's `pre-commit`/`commit-msg` hooks against the combined tree — then **fast-forwards main only after the commit lands**. The main checkout is never mutated until that clean fast-forward, so a blocked combine or a hook rejection lands nothing and leaves unrelated main-checkout WIP intact (WIP-safe). The message is **synthesized deterministically by the CLI** — a structural projection of the milestone id + its id-ordered sub-task list (no authored prose, so it's *structure* the CLI owns, not a slot; like git's auto-generated merge message); it is byte-identical across feed orders, which the determinism acceptance requires.
   - **`squash: false` — one commit per code-carrying sub-task in task-id order, then the parent's aggregate commit.** Each per-sub-task commit applies **that one worktree's** staged code onto the checkout and `git commit -F`s the sub-task's own authored `commit` doc — `git commit` (never `--no-verify`) runs the user's hooks on **each**, and **each** commit's hook output is relayed. A sub-task that staged no code contributes no commit. The **merged docs ride the parent's aggregate commit** (the by-task-id doc-join is a cross-sub-task merge with suffix resolution — [storage.md](storage.md#the-by-task-id-join-m7) — not cleanly partitionable, so docs are a milestone-level artifact, not a per-sub-task lie). The parent renders each sub-task's commit doc in id order, so the commit *sequence* is deterministic (byte-identical across feed orders) even though each message carries LLM prose — consistent with the determinism boundary (the CLI owns ordering + placement; the prose is the sub-agent's).
4. **Post-commit** runs once for the whole batch.

Either mode keeps fan-out reproducible: the committed bytes (message(s) + tree) are identical given the same `definition + cascade + live state`, independent of completion/feed order. The default (`true`) is the conservative continuation of M7's one-commit boundary; `false` is the opt-in for projects that want per-sub-task commit history. Both honor finalize.md's never-`--no-verify` contract — the user's hooks run on every fan-out commit ([What `finalize` does NOT do](#what-finalize-does-not-do)).

## Commit-doc rendering

The commit doc's schema (pack-default for the development pack — Conventional Commits) is:

| schema element | git-message slot | rule |
|---|---|---|
| `type` field (enum) | subject `<type>` | required |
| `scope` field (string, optional) | subject `(<scope>)` | omitted entirely if empty (no empty parens) |
| `summary` slot | subject line text after `: ` | required |
| `body` slot (optional) | body paragraph(s) | preceded by one blank line; section skipped if empty |
| `trailers` repeatable section | footer `key: value` lines | preceded by one blank line; section skipped if empty |

The rendered shape:

```
<type>(<scope>): <summary>

<body>

<trailer-key>: <trailer-value>
<trailer-key>: <trailer-value>
```

**Line limits.** Conventional Commits suggests 50/72 for subject/body. These ship as `commit-rendering.line-limit-subject` and `commit-rendering.line-limit-body`, both **advisory at pack-default severity** ([validation.md](validation.md) → Severity inventory). A project can promote to blocking via the cascade if it requires hard enforcement (`validation.commit-rendering.line-limit-subject.severity: blocking`).

**Cascade customization.** The commit doc's schema lives in the pack and is overridable like any doc type. A project that rejects Conventional Commits ships its own commit schema in its project layer; the renderer reads the schema, so no code changes. Trailer fields are particularly common to extend (e.g., `Co-Authored-By:`, `Refs:`, `Signed-off-by:`); these are added as `field` declarations under the `trailers` repeatable section.

**Empty commit.** If validate passes but the narrowed commit set (`git diff --cached` + the promoted docs + first-commit config) is empty, `finalize` aborts — no empty commits. The CLI selects the message by *why* it is empty: a **dirty tree with nothing staged** blocks with *"you staged nothing — `git add` your changes"* (the agent has work but never staged it); a **genuinely clean** tree aborts with *"task validated but produced no diff — nothing to finalize."* This narrowed empty-check is **per-task only**. The milestone fan-out paths have no whole-tree empty-check: under `squash: false` a sub-task that staged no code simply contributes no commit, and under `squash: true` the combine reads staged code from the worktrees (an unprovisioned / empty milestone degrades to a docs-only aggregate, [§ `fan-out` finalize](#fan-out-finalize)).

## What `finalize` does NOT do

- **No three-way merge.** A true conflict between the task's working area and an out-of-band edit is blocked, not auto-resolved ([write-commands.md](write-commands.md) → Out-of-band reconciliation); three-way merge is deferred and parallels override-conflict resolution.
- **No schema migration.** A doc-type schema version bump that requires re-rendering existing instances is a separate operation; `finalize` only writes content the task authored against the current schema.
- **No LLM call.** Like the rest of the CLI core, this transaction is deterministic — same `(task working area + base commit + cascade)` in → same commit out ([VISION.md](../VISION.md) principle #1).
- **No `--no-verify`.** Pre-commit and commit-msg hooks are the user's policy. The CLI respects them; bypass would be silent-data-loss territory.

## Open questions

- **Multi-doc promotion ordering** — when a task produces multiple managed docs (e.g., an ADR plus edits to an existing SPEC), the stage set is order-independent for git, and the edge-index updates after commit. Flagged as a non-issue under the current edge-index design; revisit if it surfaces.
- **`finalize --dry-run`** — **resolved (B1 / M30):** `jigc task finalize <id> --dry-run` walks the transaction without committing and prints the **change-set manifest** — what would be committed (the index) and what would be left out (untracked / unstaged WIP) — alongside the validation preview `jigc task validate <id>` already gives. See [Surfaced, not prevented](#dirty-tree-policy).
- **Commit-msg hook output capture** — **resolved (M19):** the CLI relays the captured `git commit` stdout/stderr to the agent **on success** (not only on rejection — phase 6), so a non-blocking hook's warning (e.g. the M19 doc↔code backstop) reaches the agent instead of being swallowed. The richer structured blocked/error-payload shape ([write-commands.md](write-commands.md#open-questions)) stays the broader open question; M19 settles the success-relay specifically.
