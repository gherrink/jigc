---
kind: code-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# The registration checklist names none of the census fences

## Sides

### implementation/doctype-authoring.md  {#side1}

The checklist a new or reshaped doctype is walked through. At planning it named none of the hand-listed tests that a registration turns red.

### crates/cli/tests/  {#side2}

The hand-listed fences that registering a methodology doctype turns red: 14 measured on the planning spike and 19 at the gate-record (R5). Among them are `registry_seam`, `count_fences`, `doctype_map_versions`, `roundtrip_registry_fence` and `item_slot_ceiling_axis`.

## Description

`implementation/doctype-authoring.md` is the checklist that a new or reshaped doctype is walked through. At planning it named none of the hand-listed tests that registering a methodology doctype turns red, so an author following it met each one at the gate. The doctypes gap-detector's census measured 14 real reds on a spike, and the gate-record re-measured 19 (R5).

## Evidence

The census in `gap-list.md` and `planning-gate-record.md` → row 3. Re-read at `a7a742d3`: `doctype-authoring.md` → *Registration-census fences*, against the fences `design/findings-channel.md` → 1.7 lists.

## Resolution

Fixed while the new doctypes were registered, as planned. M55 Increment 7 (`582f045a`) added *Registration-census fences — the hand-listed tests a registration turns red*: a doctype half, a workflow half, a step-text half, and the obligations no list fence names. Increment 8 (`524f7dec`) added what registering the four workflows measured. UNPINNED: the checklist is prose, and no test holds it to the fences it lists.

Re-driven at `a7a742d3`: the section is present, and it names each fence that `findings-channel.md` → 1.7 lists, `item_slot_ceiling_axis`'s two tests and `schema_load_strictness` among them.
