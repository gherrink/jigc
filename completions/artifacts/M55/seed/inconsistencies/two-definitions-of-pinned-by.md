---
kind: doc-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# Two definitions of pinned-by

## Sides

### implementation/pinning.md:51  {#side1}

`pinned-by:` is `<module>::<test_name>`, or `UNPINNED: <why>`: a plain string. §3 refuses a `pinned-by:` symbol parser.

### ideas/finding-doctype.md:23  {#side2}

A verified fact's record carries a code anchor that points at its pinning test, so a fact whose anchor dangles goes red.

## Description

The same field had two shapes. `implementation/pinning.md` §3 defines `pinned-by` as a plain string naming a test, and refuses a symbol parser. The parked finding-doctype idea makes the pin a code anchor, which `validate` would report when its target vanished. The `jigc-feedback` doctype needed one of them. The planning register recorded it.

## Evidence

Read at planning (`pinning.md:51,106`). Re-read at `a7a742d3`: `pinning.md:51` and `:56`, `ideas/finding-doctype.md:5` and `:23`, and the `pinned-by` leaf in `crates/cli/packs/methodology/schemas/jigc-feedback.yaml:42`.

## Resolution

Settled at the M55 Settle (S6, `9fd644b7`): `pinned-by` is a string in pinning.md's grammar, and the code anchor stays parked. M55 Increment 7 (`9dcb664d`) registered it on `jigc-feedback` as `{ id: pinned-by, type: string, optional: true }`. `pinning.md` → §3 names the doctype as the grammar's second consumer and stays its one home. The idea file's 2026-10-02 head note says that the anchor is what stays parked, and why, so the corrected side is the idea file, by its head note. Its body is kept as the parking record. UNPINNED: the two texts are prose. The leaf's shape is held by the methodology manifest's `schema-hash` at pack-load, which is not a test of this disagreement.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a7a742d3`): `jigc doc schema jigc-feedback` prints `pinned-by: string (section: meta)`.
