---
kind: doc-doc
status: open
date: 2026-10-03
schema-version: 1
---

# The worked-examples index lists 16 of its 54 flows

## Sides

### design/worked-examples.md:7  {#side1}

*The flows:* an index of 16 entries, renumbered 1 to 16. Its last entry is flow 38.

### design/worked-examples.md:24  {#side2}

The flow sections themselves, `## 1.` through `## 54.`, with no gap.

## Description

`design/worked-examples.md` opens with *The flows:*, an index that lists 16 of the doc's 54 flows and renumbers them 1 to 16. Nothing says the index is a selection. The docs gap-detector found it.

Re-driven at `a7a742d3`: **still open**. The index still has 16 entries, and the sections run from `## 1.` to `## 54.` with no gap.

## Evidence

```sh
sed -n '/^The flows:/,/^## 1\./p' design/worked-examples.md | grep -c '^[0-9]*\. \['   # 16
grep -c '^## [0-9]*\.' design/worked-examples.md                                     # 54
```

## Resolution
