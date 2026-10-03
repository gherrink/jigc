---
kind: code-doc
status: open
date: 2026-10-03
schema-version: 1
---

# Methodology docs show the deferral-ledger kind as two letters

## Sides

### design/methodology-docs.md:30  {#side1}

The doctype table: a deferral-ledger entry is *title + `kind`(enum D/I) + `trigger`(string) + `date` + `body` slot*.

### crates/cli/packs/methodology/schemas/deferral-ledger.yaml:37  {#side2}

`{ id: kind, type: enum, of: [Decision, Idea] }`. M41 F4 renamed the members from `D` and `I`, at deferral-ledger schema-version 2.

## Description

`design/methodology-docs.md`'s doctype table shows a deferral-ledger entry's `kind` as `enum D/I`. The members have been `Decision` and `Idea` since M41 renamed them, which bumped the schema to version 2. The docs gap-detector found it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a7a742d3`): **still open**. The table row moved from `:31` to `:30` and is unchanged, and `jigc doc schema deferral-ledger` prints `kind: enum [Decision|Idea]`.

## Evidence

```sh
grep -n 'enum D/I' design/methodology-docs.md        # 30, read in the checkout
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema deferral-ledger | grep 'kind:'      # kind: enum [Decision|Idea] …
```

## Resolution
