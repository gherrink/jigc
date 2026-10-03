---
kind: code-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# The project guide miscounts the methodology schemas

## Sides

### CLAUDE.md:16  {#side1}

The methodology manifest *freeze-asserts all ten shipped schemas at pack-load*. At planning it said ten, and since M55 Increment 7 it says thirteen.

### crates/cli/packs/methodology/config/schema-manifest.yaml  {#side2}

Lists every shipped methodology schema: eleven at planning, and thirteen since `jigc-feedback` and `inconsistency` were registered.

## Description

`CLAUDE.md` said the methodology manifest freeze-asserts *all ten shipped schemas*, but the manifest listed eleven. The count is prose that no fence reads. The docs gap-detector found it, and the registration census in `gap-list.md` names it again.

## Evidence

Read at planning. Re-read at `a7a742d3`: `CLAUDE.md:16`, the `- type:` entries of the methodology `schema-manifest.yaml` (13), and `crates/cli/packs/methodology/schemas/` (13 files).

## Resolution

Corrected in `CLAUDE.md` by M55 Increment 7 (`9dcb664d`), the commit that registered the two findings doctypes. The sentence now reads *all thirteen shipped schemas*. UNPINNED: no test reads `CLAUDE.md`'s count. `count_fences::the_manifest_listed_doctype_counts_are_the_manifests_own` fences the same figure in `corpus-migration.md` only.

Re-driven at `a7a742d3`: the sentence says thirteen, and the manifest lists thirteen. The *ten schemas listed* at `CLAUDE.md:69` is M40's dated count and is still true of M40.
