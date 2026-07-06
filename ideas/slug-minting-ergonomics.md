# Slug minting ergonomics — word-boundary mint, shorter cap, `--slug` override

**Status: parked 2026-07-06, unscheduled.** From RC greenfield trial 1, finding F3 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions. The concrete arrival of [structural-grammar.md](../design/structural-grammar.md) → Open questions "minting mechanics (slug normalization, collision-suffix form)".

## The gap

The mint's hard length cap cuts mid-word/mid-phrase: `verify-whether-rovo-glean-and-onyx-enforce-per`, `revise-the-vision-ground-it-in-the-two-new`. The resulting IDs are simultaneously too long to type and too truncated to read — and they are *stable identity* (the invariant), so a bad mint is carried forever or costs a `jigc rename` (M35) that a better mint would never have needed.

## The shape

- **Mint at word boundaries** with a shorter cap (~5 words) — a pure normalization change inside the CLI's minting authority; collision-suffix policy unchanged.
- **Accept `--slug`/`--id` overrides** at mint time (`start`, `create`) — the human/agent proposes identity, the CLI still validates the grammar + owns collision handling. An override at mint is strictly cheaper than a rename after.

Bound: this changes *minting* only — existing committed IDs never move (that is `rename`'s job), and the stable-ID invariant is untouched.

## Trigger

Next milestone touching the mint path, or the adoption trial reproducing unreadable IDs. Mind the freeze: mint policy is CLI behavior, not doctype schema shape, so no version-gate rides on it — verify that claim at pickup.
