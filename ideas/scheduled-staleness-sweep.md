# Scheduled staleness sweep — `jigc validate` on a clock, not only at task time

**Status: parked 2026-07-02, unscheduled (thin).** From the KB/research ideas harvest. Indexed from [VISION.md](../VISION.md) → Open questions. Pairs with [derived-doc-staleness](derived-doc-staleness.md): that idea supplies *what* to detect; this supplies *when*.

## The shape

Drift detection today fires reactively — at task/finalize time, or when someone runs `jigc validate` by hand. The ecosystem has standardized *time-triggered* maintenance (cron Routines, scheduled triage; "scheduled triage closes the loop" — research/07 §1/§4), and jigc's store sweep is exactly the shape to run on a schedule: read-only, report-only, deterministic, exit-0. The idea is the **integration seam**, not a runtime: an externally-scheduled (cron / CI / Routine) standing `jigc validate` whose report is the artifact — surfacing doc↔code and doc↔doc drift proactively instead of only when a task happens to touch the area. **No runtime enters jigc** — the scheduler stays outside; jigc remains a one-shot deterministic command, so the no-runtime invariant holds by construction.

## Trigger

Post-go-live, a real project's store-scope drift accumulates between tasks and reactive task-time detection proves insufficient (the same driver class `derived-doc-staleness` waits on).
