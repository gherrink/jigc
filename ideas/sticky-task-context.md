# Sticky active-task context — `jigc task use <id>`, like a git branch

**Status: parked 2026-07-06, unscheduled.** From RC greenfield trial 1, finding F4 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Every one of the trial's ~30 write commands carried the full `--task <long-id>`. That is a third of the typed surface and a whole class of copy-paste errors — for a value that is constant across an entire session.

## The shape

`jigc task use <id>` records an active task; write verbs default to it, explicit `--task` always overrides. The open fork is **where the stickiness lives**, and it is load-bearing:

- **Store-global** (a file under `.jigc/state`) is the git-branch analogy — but it collides with the concurrency invariant: fan-out sub-agents are isolated *by task ID*, and a repo-global default task would let one writer silently write into another's context. If store-global, it must refuse/ignore while a fan-out is active (the M35 coarse milestone-dir guard precedent).
- **Per-process/session** (`JIGC_TASK` env var) is concurrency-safe by construction (each spawned agent gets its own), at the cost of the adapter having to set it — which the spawn template already parameterizes per task.

Lean env-var-first; the explicit `--task` in *composed* command text stays (compose knows the ID — determinism is free there); stickiness serves the hand-typed/agent-improvised calls.

## Trigger

Next agent-ergonomics milestone, or the adoption trial confirming the tax. Settle the store-global vs env-var fork at pickup, against the fan-out isolation invariant.
