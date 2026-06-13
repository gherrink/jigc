---
supersedes: adr:absorb-advances-baseline-on-every-sweep
---

# absorb-advances-the-file-state-baseline-only-at-a

## Context

When the reconcile sweep absorbs a conformant out-of-band edit to a committed managed
document, the file-state baseline — the recorded raw-byte hash the next drift check
compares against — must eventually advance to the absorbed content, or the same external
edit is re-detected on every later run.

The earlier decision advanced that baseline on *every* sweep that saw a conformant edit,
including read-only previews such as `task validate`. That had a latent flaw the team
flagged: a non-committing sweep (a preview) would silently make a previewed-but-never-
committed edit the new baseline, so a later run could no longer tell the edit had never
actually landed — a verified re-fire / over-absorb defect.

## Decision

The absorb sweep advances the file-state baseline durably only at a **landed** `finalize`.
The per-task preflight sweep still classifies and routes drift in memory (so `task validate`
previews exactly what `finalize` gates on), but the post-sweep file-state record it produces
is persisted only by post-commit, and only when the commit actually lands. A standalone
`task validate` and any other non-committing sweep drop the record — read verbs stay pure
readers. This replaces the earlier "advance on every sweep" behavior.

## Consequences

An absorbed out-of-band baseline advances exactly once, at the commit that lands it, so a
previewed-but-never-committed edit can no longer silently become the baseline — the re-fire
defect is closed. The cost is that the absorbed-but-not-yet-committed baseline lives only in
the preflight's in-memory record between sweeps, so a conformant external edit is re-detected
and re-absorbed (harmlessly, advisory) on each preview until a commit lands. The post-commit
record advance is best-effort and self-heals: the commit is already truth, and the file-state
record is a rebuildable cache. Milestone-boundary finalizes run no sweep and pass no
post-sweep record, loading the durable record as before.
