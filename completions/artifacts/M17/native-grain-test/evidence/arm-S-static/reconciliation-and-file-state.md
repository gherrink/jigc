---
cites: out-of-band-edits-are-detected-and-routed, absorb-advances-baseline-only-at-a-landed-finalize, drift-hash-is-over-raw-bytes
---

# Reconciliation and file-state

## Overview

This subsystem keeps the managed-document store human-editable without losing integrity. Humans
review and edit managed docs through git regardless of the CLI, so an edit made outside the CLI
(out-of-band, OOB) must be **detected and routed** — never silently lost, silently merged, or
forbidden. Detection compares each managed file's current content against a recorded **baseline**
(a raw-byte hash kept in `.jigc/state/file-state.json`); routing classifies the difference and emits
a finding that either honors the edit (absorb), blocks it (conflict / conformance), or adopts a
fresh baseline.

The boundary it owns is the `file ↔ CLI-state` relationship for committed managed docs: deciding
whether on-disk content matches what the CLI last recorded, and what to do when it doesn't. It does
**not** own the working-area writes of an active task (reconciled elsewhere), nor does it execute any
remediation — it routes, surfacing a route the human or a later command runs.

The baseline model is "last-known-good committed state": a `path → hex-hash` map, path-sorted so the
JSON is byte-stable, treated as a rebuildable cache (a missing record is the first-encounter case,
not an error). The recorded hash advances at exactly three sites — **baseline-adopt**, **absorb**,
and **commit** — and the absorb advance is durable **only at a landed `finalize`** (see the cited
decisions). Drift alone never advances the baseline.

The routing outcomes:

- **Baseline-adopt** — no recorded hash for the path (first run / fresh checkout): the current
  content *is* the baseline. Advisory finding, no route.
- **In-sync** — recorded hash matches: nothing to do.
- **Absorb** — drifted, the task did not touch it, and the edited bytes re-parse and pass schema
  conformance: the edit is honored, the baseline re-hashed forward, and the edge index incrementally
  updated. Advisory finding.
- **Conformance-block** — drifted and the edited bytes fail parse or schema conformance: blocking
  finding naming the first precise error; baseline and index left untouched (no auto-repair).
- **Conflict-block** — drifted *and* the active task also staged the same doc: both sides moved.
  Blocking finding with an explicit-discard route; never a silent merge.
- **Rename** — a recorded path is missing on disk: a separate classifier routes a strong-signal
  suspected `git mv` (an untracked file carries the same recorded hash) or a weak-signal restore
  (no content match). Never rewrites referrer refs or the edge index.

## Components

### Raw-byte drift hash
Hashes a managed file's raw bytes to its lowercase-hex `blake3` digest — the unit of comparison for
all drift detection. Over raw bytes (never a canonicalized form), so a conformant no-op read/write
does not register as drift.
implemented-by: hash_bytes

### File-state baseline record
The `path → hex-hash` map persisted at `<jigc_root>/state/file-state.json`. A path-sorted `BTreeMap`
so the serialized bytes are deterministic; load/save treat it as a rebuildable cache, with a missing
file yielding an empty record rather than an error.
implemented-by: FileStateRecord

### File-state probe
The engine-native probe for the commit-only loop: classifies each `(path, bytes)` against the
recorded baseline into baseline-adopt (advisory, records the hash), in-sync (no finding), or drift
(blocking, carrying a `reconcile` route). Drift does not advance the baseline.
implemented-by: file_state

### OOB reconciliation classifier
The full state machine over a single committed managed doc: routes the `(committed-state,
task-touched)` pair to baseline-adopt, in-sync, conflict-block, or — for a drifted, untouched doc —
the parse classifier, which absorbs a conformant edit (re-hashing the baseline and folding the doc's
forward edges into the index) or conformance-blocks a non-conformant one. Mutates the record and the
edge index only on baseline-adopt and absorb.
implemented-by: reconcile_committed

### Committed-store sweep
The command-surface wiring that runs the classifier (and rename detection) over every tracked
committed managed doc — the full sweep `task validate` previews and `finalize`'s preflight gates on.
Walks each persisted schema's `location:` directory, derives each doc's identity and record key,
reconciles present docs, and routes recorded-but-missing paths to rename detection.
implemented-by: reconcile_committed_store

### Rename detection
The separate classifier for a tracked path gone missing on disk, which the state machine alone
cannot catch (it sees the old path as missing and the moved file as fresh untracked content). Emits
a strong-signal finding (an untracked file with the same recorded hash → suspected `git mv`, routed
to revert) or a weak-signal finding (no match → routed to restore). Pure of I/O and of any
edge/referrer mutation.
implemented-by: detect_rename

### Landed-finalize baseline persistence
The post-commit step that persists the shifted file-state baseline durably and exactly once. The
preflight sweep's post-sweep record rides through the finalize transaction and is saved **only when
the commit lands** — read verbs (a standalone `task validate`) drop it. The committed working set is
re-hashed on top, and per-run staged working-area keys are stripped before persistence so only
committed-store baselines survive.
implemented-by: advance_file_state
