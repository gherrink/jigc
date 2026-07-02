# Output-language directives — per-doctype natural-language guidance for authored prose

**Status: parked 2026-07-02, unscheduled.** From the KB/research comparison. Indexed from [VISION.md](../VISION.md) → Open questions. **Not** [multi-language-doc-code](multi-language-doc-code.md) (programming-language doc↔code validation) — a previously mis-folded distinction, now split: this is about the *natural language of authored prose*.

## The shape

A German team may want specs/ADRs/PRDs authored in German while agent-context and commits stay English. The decision rule the research settles: **split by reader** — agents count as English-preferring (the Lost-in-the-Mix asymmetry, research/12); client-facing deliverables follow the client. Prose language is LLM-side of the determinism boundary — jigc must never *validate* it (a language-conformance check would grade prose content). What jigc *can* own is the structural delivery of the directive: per-doctype slot-`hint` language via schema shadow (works today, heavy), or the cheap declared seam — a `{{fill: authoring-guidance}}` extension point in the `author-*` steps so a project injects language/register guidance without forking a step.

Evidence: research/12 (English-matrix asymmetry; directive must survive compaction; split-by-reader), KB `language.md` (mechanism verdict: schema-shadow scoping beats path-scoped CLAUDE.md directives).

## Trigger

A real client/non-English project on jigc asks for non-English managed-doc prose.
