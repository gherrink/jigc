# doc search — a managed-corpus search surface

**Status: parked 2026-07-12.** From the human's trial scratch notes + the RC implementation-half trial's read-path observations ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Agents fall back to `grep`/direct file reads to *find* the relevant managed doc — the trial's agents read committed docs directly during orientation, hand-grepped for prose mentions, and twice invoked a nonexistent `jigc doc list`. The read surface answers "show me doc X" (`doc show`) and "what shape is doctype Y" (`doc schema`) but not "which doc talks about Z" — so discovery, the step before every read, happens outside the tool. If jigc is the sole channel for reading managed docs, it has to be good at *finding* them, or grep stays the path of least resistance.

## The shape

Layered, cheapest first: (1) `jigc doc list [--type T]` — enumerate managed instances (type, slug, title, home); this is pure index projection and the trial demanded it verbatim twice; (2) `jigc doc search <term>` — deterministic text/title match across managed docs, returning addresses (not content) so the result feeds `doc show`; (3) anything smarter (ranking, embeddings) is out — judgment lives with the agent, the CLI returns matches. Search over *code* too was floated in the notes; that's the code-intel tools' job — jigc's edge index already knows doc→code anchors, which `search` could expose (which docs anchor to this file/symbol — the read-side inverse of [symbol-mention-sweep](symbol-mention-sweep.md)).

## Trigger

`doc list` is fix-shaped (chartered with the rc.6 wave); the `search` layer parks until an adopter corpus is big enough that list+show stops being enough — or the adapter-adherence data shows grep-for-discovery is what pulls agents off the managed path.
