# Managed-doc enforcement hook — a blocking `PreToolUse` guard on direct edits

**Status: parked 2026-07-02, DEFERRED with a live trigger** ([decisions-pending](../implementation/decisions-pending.md) → (D) Managed-docs enforcement hook). Indexed from [VISION.md](../VISION.md) → Open questions.

## The shape

The adapter boundary is honestly "adapter-enforced, not sandboxed" — an agent that ignores the adapter can still edit managed files directly (VISION principle #3: ergonomics, not prevention). Hooks are the one deterministic enforcement mechanism the harness offers (research/04 §5: `PreToolUse`, `exit 2` blocks). A jigc-emitted hook matching `Edit|Write` against the managed docs-root would block direct edits and route to `jigc` — converting part of the ergonomics bet into actual enforcement for the managed set, without sandboxing.

## Why deferred, and what decides it

This engages two records at once: the deliberate ergonomics bet (VISION #3) and the [cost-of-enforcement](cost-of-enforcement.md) evidence — the block→re-orient→recover loop is the *measured* cost driver on capable models (long-horizon study: 3–6× static on Opus), and M35's verdict ("own the verb, don't police the mistake") points the other way for differentiators. A blocking hook is policing. The counter-evidence: the replication showed instruction alone does not route weaker models (Sonnet bypassed at 19% drift; the gate arm held 0%).

**The decision data is coming:** M36's opt-in invocation log makes RC-trial drive-arounds measurable. *Trigger:* the RC trials' adherence data shows bypasses at a rate that costs correctness — then decide with evidence, weighing the block-loop tax against the drift caught.

**2026-07-15 adherence datum** ([lacon trial](../completions/artifacts/RC-lacon/trial-record.md) → Adherence): write-side adherence was strong — 35 of 38 trial commits through `finalize`, and of the 3 direct-git commits two were legitimately unmanaged surfaces and one was a doctype gap (no fitting home), not a drive-around. Read-side is the weak flank again: 25 migration `doc author`s with **zero** `doc show`, and the task-4 tester read `docs/roadmap.md` with a raw Read — corroborating rc.5's note. Same conclusion as before, now twice-evidenced: the read path must be the path of least resistance (AGENT.md never *names* `doc show`/`doc list` — fix-shaped) before a read-side block could be anything but a tax; the write side shows no correctness-costing bypass to enforce against.
