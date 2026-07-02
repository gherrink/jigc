# Glossary doctype + selective term injection — managed ubiquitous language

**Status: parked 2026-07-02, unscheduled.** From the KB/research ideas harvest. Indexed from [VISION.md](../VISION.md) → Open questions.

## The shape

A `glossary` doctype (ubiquitous-language / termbase) as a managed artifact, whose entries the CLI *selectively injects* into a composed workflow's context slice — loaded only for the compositions that need them (domain/deliverable work), never dumped into always-on context. Rides existing machinery end-to-end: the doctype + repeatable entries are ordinary schema; *which terms load into which composition* is a structural/placement decision (CLI-side); the LLM only consumes them. Terminology injection is a measured win for specialized-domain output (WMT24), and "selective, not whole-termbase" is the stated best practice — which maps exactly onto jigc's per-workflow slice model.

Evidence: research/12 §5 (tiered termbase, selective inclusion, WMT24 measurement), research/09 §3.6 (GLOSSARY as versioned repo file), KB `language.md#glossary-terms-quoted`.

Sits beside, not inside, [output-language-directives](output-language-directives.md) (a language *setting*) and [multi-language-doc-code](multi-language-doc-code.md) (programming-language validation) — three different axes.

## Trigger

A domain-heavy pack or a client-deliverable workflow lands where terminology consistency matters.
