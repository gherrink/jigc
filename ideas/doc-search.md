# doc search — a managed-corpus search surface

**Status: parked 2026-07-12.** From the human's trial scratch notes + the RC implementation-half trial's read-path observations ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Agents fall back to `grep`/direct file reads to *find* the relevant managed doc — the trial's agents read committed docs directly during orientation, hand-grepped for prose mentions, and twice invoked a nonexistent `jigc doc list`. The read surface answers "show me doc X" (`doc show`) and "what shape is doctype Y" (`doc schema`) but not "which doc talks about Z" — so discovery, the step before every read, happens outside the tool. If jigc is the sole channel for reading managed docs, it has to be good at *finding* them, or grep stays the path of least resistance.

## The shape

Layered, cheapest first: (1) `jigc doc list [--type T]` — enumerate managed instances (type, slug, title, home); this is pure index projection and the trial demanded it verbatim twice; (2) `jigc doc search <term>` — deterministic text/title match across managed docs, returning addresses (not content) so the result feeds `doc show`; (3) anything smarter (ranking, embeddings) is out — judgment lives with the agent, the CLI returns matches. Search over *code* too was floated in the notes; that's the code-intel tools' job — jigc's edge index already knows doc→code anchors, which `search` could expose (which docs anchor to this file/symbol — the read-side inverse of [symbol-mention-sweep](symbol-mention-sweep.md)).

**2026-07-15 extension — the inbound-reference query joins layer 2** ([lacon trial](../completions/artifacts/RC-lacon/trial-record.md), migration report #5; verified [findings-verification](../completions/artifacts/RC-lacon/findings-verification.md) → A5): the migration tester asked for `jigc references <doc>` — inbound path-links across the repo — after hand-repointing ~40 links post-slug-drift. No such verb exists, but the machinery half-does: `jigc rename`'s repo-wide word-boundary prose-mention scan (M35, `rename.rs:461-478`) reports every `path:line` it can't rewrite — inseparable from executing a rename today. Exposing that scan read-only (plus the edge index's managed→managed inbound edges) is the layer-2 shape: "which docs/files reference this doc" — the read-side inverse of [symbol-mention-sweep](symbol-mention-sweep.md), and most valuable in the [bulk-onboarding-flow](bulk-onboarding-flow.md).

## Trigger

`doc list` is fix-shaped (chartered with the rc.6 wave); the `search` layer parks until an adopter corpus is big enough that list+show stops being enough — or the adapter-adherence data shows grep-for-discovery is what pulls agents off the managed path.
