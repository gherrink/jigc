# reference doctype — the handbook/topic doc with optional code anchors

**Status: parked 2026-07-10.** From the RC adoption trial's doctype-gap feedback, finding D2 ([trial-record](../completions/artifacts/RC-adoption/trial-record.md) → Doctype gaps) — and independently confirmed the same day by the [self-migration coverage audit](../completions/artifacts/RC-adoption/self-migration-coverage.md): **~40 of jigc's own files** (all of `design/`, most of `implementation/`, QUICKSTART, WHY-JIGC) fit no existing doctype. The dominant doctype gap on both corpora. Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Every real project carries durable topic prose — stack notes, conventions, testing guidance, structure walkthroughs, design-of-record part-docs. In the trial, ~1,300 lines of true, useful GSD reference content (STACK/CONVENTIONS/TESTING/STRUCTURE) matched no schema and was dropped as "CLAUDE.md's territory" — which the trial agent itself named a dodge: CLAUDE.md is unmanaged, unvalidated, invisible to `validate`. `arch-doc` is close but wants "one part of the system and its components" with a *required* `implemented-by` anchor per component; conventions and stack notes are not component-structured.

## The shape

A topic doc: `id-from: title`, `location: reference/`, an `overview` slot + a **repeatable `topics` section** `{title, body-slot}` (per-doc-varying section sets survive the schema-driven parse — the `prd.requirements` pattern) + an **optional** per-topic `cites-code` anchor — so a testing-guide topic *can* anchor to the code it describes and go stale loudly, without arch-doc's required-anchor tax. No component structure, no `title-names-symbol`.

What it buys: reference prose kept honest against the repo instead of rotting in an unvalidated file — the doc↔code drift promise extended to the largest doc class projects actually have. For jigc's own self-migration it is the volume driver (`migrate-reference` over ~40 files); the deep-heading-nesting flattening problem is recorded as structural blocker 4 in the coverage audit.

## Trigger

The post-1.0 **doctype-completeness milestone** — the self-migration's anchor doctype; plan it first among the doctype candidates.
