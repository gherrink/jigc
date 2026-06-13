---
status: accepted
date: 2026-06-13
supersedes: absorb-advances-baseline-on-every-sweep
---

# Absorb advances baseline only at a landed finalize

## Context

The earlier rule advanced the file-state baseline to the observed content on every reconcile
sweep that saw a conformant external edit — including read-only previews such as `task validate`.
That over-absorbed: a previewed-but-never-committed edit silently became the new baseline, so a
later run could no longer tell the edit had never actually been committed. The defect was verified:
an absorbed OOB baseline re-fired in every later task because the advance was either lost (a pure
read verb) or applied without a landed commit behind it.

## Decision

The reconcile sweep still classifies and absorbs a conformant OOB edit in memory, but the
shifted file-state baseline is **persisted durably only at a landed `finalize`**. The preflight
sweep returns its post-sweep record alongside the report; that record is threaded through the
finalize transaction and saved by post-commit **only when the commit actually lands**. A standalone
`task validate` (a read verb) drops the swept record — read verbs stay pure readers. This keeps
the three named re-baseline sites (baseline-adopt / absorb / commit) but binds the absorb advance
to the commit boundary, so an absorbed baseline advances durably exactly once.

## Consequences

An absorbed OOB baseline stops re-firing across later tasks (the verified re-fire defect is fixed),
and a previewed-but-never-committed edit no longer leaks into the baseline. The cost is plumbing:
the post-sweep record must ride from the preflight through the finalize transaction to post-commit,
and the per-run staged working-area keys (`docs/<type>:<slug>.md`) must be stripped before
persistence so only committed-store baselines survive. This is the M17 amendment to the
baseline-persistence rule.
