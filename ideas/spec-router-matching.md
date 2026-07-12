# spec matching in the router — "this looks like spec X — bind it?"

**Status: parked 2026-07-12.** From the RC implementation-half trial, probes 2+3 ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

On a brownfield corpus with 11 committed specs, `jigc start "<intent>"` routed across ten workflows without ever checking whether a committed spec already covered the intent. Probe 2's agent implemented a spec'd task through `dev-task` and found the governing spec *by accident, late* — nearly re-deriving it from scratch and nearly walking into the exact trap the spec documented. Probe 3 proved `implement-from-spec` works when the prompt names a spec — the workflow isn't broken, it's **invisible**: two-for-two sessions never reached it unprompted.

> The router should match intent against the committed spec corpus and say "this looks like spec X — bind it?" before offering dev-task. On a brownfield repo, that's the question that matters most.

## The shape

The tension to resolve at pickup: intent→spec *matching* is a judgment call, and the CLI makes no LLM calls. Two boundary-respecting shapes: (a) **surface, don't match** — the router's composed output lists the committed specs (title + slug, cheap since `doc show`/the index already know them) with "if one of these covers this intent, bind it" — the *agent* judges, the CLI only guarantees the specs are in view at routing time (a context-compiler move, no matching logic); (b) deterministic *lexical* overlap scoring as a hint ranking, still agent-judged. Start with (a); it's pack YAML + a data-value root.

## Trigger

The next workflow/pack wave, or a second trial/adopter session that re-derives a committed spec.
