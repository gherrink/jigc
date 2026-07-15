# sequential milestone — the work-unit for dependency-spined milestones

**Status: parked 2026-07-12.** From the RC implementation-half trial, probe 4 ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → Probe 4). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

The milestone work-unit fans sub-tasks into isolated worktrees pinned to a shared base — by design (the one-bounded-primitive concurrency invariant: fan-out/join, merge by task id). The trial's brownfield milestone was a **linear spine** — increment 3 couldn't compile without increment 2's code — so every worktree would miss its predecessors' work. The agent discovered this only by hitting "pinned to base X but you're on Y", discarded all six sub-tasks, and fell back to flat sequential dev-tasks: the milestone surface's first real use collapsed ~80 seconds after creation, and the execute/join/finalize happy path went unexercised — *"which is itself a signal that the unit didn't fit a very normal kind of work."* Consequence chain: no milestone unit → no milestone-record continuity → no completion gate (the close was skipped entirely).

> The milestone unit assumes independent, parallelizable sub-tasks. A large fraction of brownfield work is sequential-with-dependencies. Either detect that shape, offer a "sequential milestone" mode, or at minimum document the assumption prominently.

## The shape

Three rungs, cheapest honest one first: (1) **disclaim + detect** — the milestone create/add-task surface states the parallel-independent assumption, and the first `start --task` failure routes to "your sub-tasks depend on each other → run them as sequential tasks under the milestone record" instead of a bare base-pin error; (2) **sequential mode** — sub-tasks execute in order *on the main branch* (no worktrees, no join; the milestone-record still tracks membership and the completion gate still binds) — this adds no new concurrency primitive, it *removes* one for the sequential case, so the invariant is arguably untouched; (3) mixed DAGs are **out forever** (explicit non-goal: no DAGs, no conditionals). The design question for Settle: is rung 2 a mode of the milestone unit or just "a milestone-record over ordinary sequential tasks" — the latter may already be most of the machinery.

## Trigger

The milestone-surface wave that also owes the stale-record-on-discard fix and the skippable-completion decision (they're one cluster: the unit must *fit* brownfield work before its close can be made non-skippable).

**2026-07-15 datum — the second real planning session never reached for the milestone unit at all** ([lacon trial](../completions/artifacts/RC-lacon/trial-record.md) → log surprises #4): the Tier-2 session ran `planning`, wrote roadmap/decisions-log/deferral-ledger, then executed three increments as flat `dev-task`/`single-task` starts — zero `milestone` verbs in 294 logged invocations. After rc.5's milestone arc collapsed ~80 s after creation, rc.6's first planning session simply routed around the unit (whether the workflow steered away or the agent did is unasked). Two consecutive trials, zero successful milestone-unit uses on sequential brownfield work — the unit-fit question is no longer hypothetical.
