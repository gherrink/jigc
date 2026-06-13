---
status: accepted
date: 2026-05-31
---

# Drift hash is over raw bytes

## Context

The file-state baseline is a content hash the next drift check compares against. If the hash were
taken over a canonicalized form of the document, a conformant no-op read/write that re-serialized
the file would register as drift, and first-touch canonicalizations could never re-baseline cleanly.

## Decision

The drift hash is computed over the file's **raw bytes** (`blake3`, lowercase-hex), never over a
canonicalized form. A conformant no-op read/write does not drift, and the record stores `path → hex`
in a path-sorted map so its on-disk JSON is byte-stable across runs.

## Consequences

Drift detection is exact and cheap, and the record is a rebuildable cache (re-derivable from the
committed docs, gitignored). The tradeoff is that any byte-level change to a managed file — even a
semantically-equivalent reformat — counts as drift and is routed through reconciliation.
