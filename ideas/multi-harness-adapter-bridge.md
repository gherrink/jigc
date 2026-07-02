# Multi-harness adapter bridge — one canonical agent-context layer, thin per-tool wrappers

**Status: parked 2026-07-02, unscheduled.** From the KB/research ideas harvest. Indexed from [VISION.md](../VISION.md) → Open questions. Distinct from the deferred *multi-assistant adapter profiles* milestone ([decisions-pending](../implementation/decisions-pending.md)) — that generalizes the *install/profile model*; this shapes the *context-file bridging* those installs share.

## The shape

jigc already is, architecturally, the "canonical semantic layer" the research names — one source of truth, thin per-tool surfaces. When a second agent harness (Codex, Cursor, Copilot, Gemini) joins a jigc-managed repo, jigc deterministically emits and maintains the bridge artifacts: an `AGENTS.md` (the Linux-Foundation cross-tool standard, 20+ native consumers) as the canonical file, plus each tool's native wrapper (`CLAUDE.md` whose first line is `@AGENTS.md`; Codex reads `AGENTS.md` natively). The CLI owns *which* files exist and *what's in them*; wrappers reference, never duplicate — "one canonical context file, bridged not duplicated" holds by construction. It's `jigc setup` machinery pointed at N harnesses.

Evidence: research/01 §8 (AGENTS.md standard + bridge mechanics), research/09 §3.1 (one canonical file per scope, others as thin bridges), KB `claude-md.md#agents-md-deferred` (bridge when a second tool actually joins — not preemptively; that ruling is why this parks rather than ships).

## Trigger

A second agent harness actually operates in a jigc-managed repo, or an adopter runs a mixed-harness team. Pairs with the multi-assistant-profile milestone when both fire.
