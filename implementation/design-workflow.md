# Design workflow (collaboration)

The loop for **design decisions and doc changes** — developing and recording the *what* and *why* of jigc's design. Distinct from the [dev workflow](dev-workflow.md) (the test-first loop for code): this loop produces decisions and documents, that one produces tested code. The design is largely settled ([VISION.md](../VISION.md), the [design/](../design/) part-docs); this governs changes to it, not a from-scratch phase.

Maurice develops the idea; Claude is a constructive-critical partner, not a code monkey. For design work the **default output is discussion, not artifacts** — nothing is written until an explicit "write it."

## The loop

0. **Scope** — Maurice says roughly what's next. Before developing, restate in one line *what* we're deciding and *at what depth*; if ambiguous, wait for a nod.
1. **Develop & present in chat** — options, tradeoffs, a recommendation with reasoning. One decision at a time; never a batched "whole design" dump.
2. **Discuss & iterate** — push back, name risks, flag honest boundaries. Constructive-and-critical is the standing stance, not a per-request ask.
3. **Converge** — write **only** on an explicit "write it." Never infer convergence from enthusiasm.
4. **Write** — into the agreed home at the agreed granularity (route by home — see [CLAUDE.md](../CLAUDE.md) → Conventions).
5. **Review, then commit** — Maurice reviews the written file; commit (conventional message, one logical change) only after he's satisfied.
