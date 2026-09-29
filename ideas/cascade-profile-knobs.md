# Cascade profile knobs — declared project facts that fan out to knob bundles

**Status: parked 2026-07-02, unscheduled.** From the KB/research comparison. Indexed from [VISION.md](../VISION.md) → Open questions.

## The shape

The KB's cross-cutting tags (client/own, maturity: prototype→active→released) are **profiles**: one declared fact about the project that flips a coherent *bundle* of applicability decisions. jigc's knob surface is per-key (~28 severity knobs + behavior knobs) — a project wanting "prototype posture" must hand-set each severity; nothing lets a pack say "maturity=released → changelog authoring expected, these severities floor up." Two possible shapes, sharply different in cost: **(a)** a profile-knob primitive that fans out to defaults — a new cascade primitive, and a *conditional* one, brushing against the no-conditionals invariant; **(b)** documented **preset delta-manifests** — pure convention, zero engine work, available today if a project asks. The KB's other two tags are already answered: *family* = the team cascade layer (built), *ai-relevance* = pack choice (deferred with the platform).

Evidence: KB index tags vocabulary + `types/identify.md#apply-the-four-tags`; the per-key knob surface in `crates/cli/packs/dev/config/knobs.yaml`.

## Trigger

A real project asks for a posture bundle (start with shape (b) — presets as convention; escalate to the primitive only if presets recur across projects and their duplication becomes the friction).
