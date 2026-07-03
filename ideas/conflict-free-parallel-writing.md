# Conflict-free parallel writing — isolate concurrent writers by branch/worktree, dogfood jigc's own model

**Status: parked 2026-07-03, unscheduled.** Surfaced from the M36 build retrospective (a `build-fixer`'s `git add -A` swept the orchestrator's in-flight completion-doc edits into its commit; it self-caught and reverted). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

When more than one agent — or an agent *and* the human-led orchestrator — writes to the repo at the same time, they share **one mutable working tree** with no isolation. Interference modes seen or latent:

- **`git add -A` sweep** — a committing agent grabs another writer's uncommitted edits (the M36 trigger; now blunted by the [build-executor](../.claude/agents/build-executor.md)/[build-fixer](../.claude/agents/build-fixer.md) scoped-staging rule, but that is discipline, not isolation).
- **Concurrent same-file edits** — two writing agents touch one file; today avoided only by *manually* assigning disjoint file sets (as the M36 planning design-writers were), which the harness does not enforce.
- **Mid-write commit** — one agent commits while another is mid-edit, capturing a half-written tree.

The current defense is **serial execution + manual disjoint assignment + staging discipline**. That holds for the common case but is unenforced and breaks exactly when we lean on parallelism (the place we reach for it most — fan-out recon, parallel design-writers, triage fixers running while the orchestrator writes the completion record).

## The shape

Two levels, kept distinct.

**1 · Harness/orchestration isolation (the immediate, buildable half).** Give every concurrent *writing/committing* agent its own **branch + git worktree**, and let the orchestrator **land deterministically** (cherry-pick / merge in a fixed order, or apply disjoint diffs). The primitives already exist in the Claude Code harness — the Agent tool's `isolation: "worktree"`, `EnterWorktree`/`ExitWorktree`, and the Workflow per-agent `isolation`. What's missing is *using them systematically* for parallel writers plus a defined **land protocol** (order, conflict policy, who commits). This is an orchestration-tool concern and may be as rich as it needs to be.

**2 · The jigc synergy ("conflict-free writing in combination with jigc") — the dogfood.** jigc's product **already embodies** this exact model, one level down: M31 isolates each fan-out sub-agent's *code* in its own worktree keyed by task-id; the CLI merges at a **join by task-id, not completion order**; and structured **managed docs are never git-text-merged** — they ride the CLI's by-task-id join (the 2026-06-04 invariant, reaffirmed M30/M31), because the CLI owns placement. That *is* a conflict-free-writing discipline: **owner (CLI/tool) owns placement · isolate-by-directory/worktree · deterministic merge by stable id · never text-merge structured content.** So:

- The harness's parallel-writing need is the **same problem jigc solves**, one altitude up — building the harness isolation *with* jigc's discipline is the purest dogfood (the harness that builds jigc should write the way jigc writes).
- It **pressures jigc's deferred concurrency opens** and would sharpen them: *multi-process concurrency — no store locking* ([decisions-pending.md](../implementation/decisions-pending.md): two writers can share `.jigc/state` with no lock), *concurrent-OOB-during-a-task* (reconciliation), and the single-fan-out-per-milestone bound. If real parallel writers ever share one jigc store, those bite — and this is where they'd first bite in anger.

## The caution — do not violate the one-bounded-primitive invariant

jigc's *product* concurrency is deliberately **one bounded primitive**: fan-out/join, isolate-by-directory, merge by task-id — **no DAGs, conditionals, cross-agent messaging, or runtime** (CLAUDE.md invariant). The harness-level isolation (worktrees + branches for arbitrary parallel agents) is allowed to be richer than that, but the two must **stay separate**: the synergy to import into the *product* is the **principle** (owner owns placement, isolate + deterministic-merge-by-id, never text-merge structured docs), **not** the harness's orchestration richness. A "general multi-agent worktree-graph runtime" folded into jigc would break the invariant — that is the failure mode to guard against. The line: harness may branch/worktree freely; jigc's product concurrency stays the one bounded primitive.

## Triggers

- Parallel-agent interference recurs and costs **real rework** (M36 self-corrected; a future run may not) — or we routinely fan out *writing/committing* agents (today the build agents are serial; the M36 parallel writers were a hand-assigned batch).
- The *multi-process concurrency — no store locking* trigger fires (two agents genuinely share one jigc store — [decisions-pending.md](../implementation/decisions-pending.md)); this idea is where the isolation answer lives.
- We choose to **dogfood jigc's concurrency model on the harness** (pairs with [self-hosting.md](../design/self-hosting.md) — the methodology pack encoding the build loop).
