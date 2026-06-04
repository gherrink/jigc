# Finalize transaction

The ordered transaction that turns a task's working area into a committed change. `finalize` is the **only** site that touches the committed store; everything else is staging. Specifying it as a single transaction keeps the rest of the system honest: read paths see committed state, write paths stage into the task area, and the one transition between them happens here.

Builds on [VISION.md](../VISION.md) (the commit boundary; the determinism boundary), [write-commands.md](write-commands.md) (`finalize ≡ validate + commit`; the task as staging unit; the two check times), [validation.md](validation.md) (the validate phase and its probes), [storage.md](storage.md) (the per-task working area, derived caches, "CLI orchestrates, git executes"), [workflow-dialect.md](workflow-dialect.md) (`fan-out`/`join` and sub-agent finalization), and [document-type-schema.md](document-type-schema.md) (the commit doc type). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## What `finalize` is

`finalize ≡ validate(task) + commit`. One task → one logical commit. The shape:

```
jigc task finalize <task-id>
  → validate                (blocking findings abort cleanly)
  → render commit-doc       (→ git message string)
  → promote managed docs    (working area → canonical paths)
  → stage                   (git add: promoted docs + code changes)
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
- The commit doc (`commit:<task-id>`) is **not** promoted — its sink is the git message rendered in phase 3, not a repo file ([write-commands.md](write-commands.md) → Instance provisioning).

Promotion is **copy, not move**, so rollback (if a later phase fails) is removal of the copies — not a complex undo. The working area stays intact until phase 7.

### 5. Stage

`git add` for:

- every promoted doc (canonical paths from phase 4),
- every change in the working tree between base `<A>` and current (the agent's code changes).

The stage set is "managed docs the task produced + every code-change the tree shows," in that order. See [Dirty-tree policy](#dirty-tree-policy) for what "every code-change" includes and excludes.

### 6. Commit

`git commit -F <rendered-message-file>` (a temp file, deleted after). The CLI **never** passes `--no-verify` — the user's `pre-commit` and `commit-msg` hooks are policy and the CLI respects them.

Outcomes:

- **Commit succeeds** → proceed to phase 7.
- **Hook rejects, or other git error** → roll back: `git restore --staged --worktree <promoted-paths>` (restores HEAD content for promoted paths), then delete the promoted copies. The working area is untouched; the agent sees the hook's stderr verbatim and re-runs `finalize` after fixing. The hook output **is** the correction signal — never bypassed.

### 7. Post-commit (best-effort)

After the commit lands, three updates happen — none can affect commit truth, all self-heal if they fail:

- **Invalidate the edge-index stamp** ([storage.md](storage.md) → Derived caches). The next read rebuilds from the new HEAD.
- **Update `file-state` hashes** for every committed managed doc. The next `file-state` probe sees them as in-sync.
- **Remove `.jigc/tasks/<task-id>/`.** The staging area's job is done; it persisted until here so rollback was possible across phases 4–6.

A failure in any of these is **logged**, not raised — the commit is real, the system reads correctly because the edge-index stamp invalidates eventually, and a stale `.jigc/tasks/<id>/` gets cleaned up by `jigc task discard <id>` or the next `finalize` reusing the slot.

## Dirty-tree policy

A task is pinned to base commit `<A>` at start. At `finalize`, **everything in the working tree differing from `<A>`** is the task's work and gets committed in phase 5 — managed docs (via promotion) and code changes (via direct stage).

The rule is simple by design:

- **No "declared touch-set."** The agent doesn't tell the CLI what it plans to edit; the CLI doesn't refuse changes outside a scope.
- **No `--include-all` flag, no "stash unrelated."** A task is a coherent unit of work; the tree's deltas from base define it.
- **Parallel hand-editing** (a human editing other files on the same branch while a task runs) is caught upstream by the base-pin: if the human commits, HEAD moves and the base-mismatch rejection in phase 1 fires; if the human only edits without committing, the changes show in `jigc task diff <id>` before `finalize` so they're visible at preview time.

The cost is honesty: if you started a task on a checkout, the tree-diff from base **is** the task. The benefit is the absence of a hidden allow/deny list the agent would have to reason about.

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

A `fan-out` workflow's parent task's `finalize` is the commit boundary; sub-agents never run git ([workflow-dialect.md](workflow-dialect.md#fan-out--join)). The transaction expands:

1. **Merge sub-task working areas into the parent** — by task-id order; slug collisions resolve via the deterministic suffix ([workflow-dialect.md](workflow-dialect.md#fan-out--join)).
2. **Validate** the merged effective state — one validate run, not N.
3. **Render, promote, stage, commit — gated by the `finalize.fan-out.squash` cascade knob.** M7 shipped only the synthesized single-commit form; **M8 adds per-sub-task authored commit docs and the knob** ([DECISIONS.md](../DECISIONS.md) 2026-06-04 → M8 Settle). The two modes:
   - **`squash: true` (default) — one aggregate commit.** Its message is **synthesized deterministically by the CLI** — a structural projection of the milestone id + its id-ordered sub-task list (no authored prose, so it's *structure* the CLI owns, not a slot; like git's auto-generated merge message). The sub-tasks' own commit docs are not rendered into the message in this mode; the synthesized message is byte-identical across feed orders, which the determinism acceptance requires.
   - **`squash: false` — one commit per sub-task in task-id order, plus the parent's commit.** Each sub-task authored its own `commit` doc (sub-agents author commit prose but never run git); the parent finalize renders each in id order, so the commit *sequence* is deterministic (byte-identical across feed orders) even though each message carries LLM prose — consistent with the determinism boundary (the CLI owns the ordering + placement; the prose is the sub-agent's).
4. **Post-commit** runs once for the whole batch.

Either mode keeps fan-out reproducible: the committed bytes (message(s) + tree) are identical given the same `definition + cascade + live state`, independent of completion/feed order. The default (`true`) is the conservative continuation of M7's one-commit boundary; `false` is the opt-in for projects that want per-sub-task commit history.

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

**Empty commit.** If validate passes but the staged diff (managed docs + code changes) is empty, `finalize` aborts with "task validated but produced no diff — nothing to finalize." No empty commits.

## What `finalize` does NOT do

- **No three-way merge.** A true conflict between the task's working area and an out-of-band edit is blocked, not auto-resolved ([write-commands.md](write-commands.md) → Out-of-band reconciliation); three-way merge is deferred and parallels override-conflict resolution.
- **No schema migration.** A doc-type schema version bump that requires re-rendering existing instances is a separate operation; `finalize` only writes content the task authored against the current schema.
- **No LLM call.** Like the rest of the CLI core, this transaction is deterministic — same `(task working area + base commit + cascade)` in → same commit out ([VISION.md](../VISION.md) principle #1).
- **No `--no-verify`.** Pre-commit and commit-msg hooks are the user's policy. The CLI respects them; bypass would be silent-data-loss territory.

## Open questions

- **Multi-doc promotion ordering** — when a task produces multiple managed docs (e.g., an ADR plus edits to an existing SPEC), the stage set is order-independent for git, and the edge-index updates after commit. Flagged as a non-issue under the current edge-index design; revisit if it surfaces.
- **`finalize --dry-run`** — `jigc task validate <id>` already previews validation; whether to add a dry-run that also walks render/promote/stage (without commit) is pending. `validate` covers most of the value.
- **Commit-msg hook output capture** — git surfaces hook stderr to the terminal; the precise shape the CLI uses to relay it back to the agent (so the agent can act on it) needs design alongside the broader blocked/error-payload question ([write-commands.md](write-commands.md#open-questions)).
