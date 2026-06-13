---
cites: [adr:out-of-band-edits-are-detected-and-routed, adr:absorb-advances-baseline-on-every-sweep, adr:absorb-advances-the-file-state-baseline-only-at-a]
---

# reconciliation-and-file-state-subsystem

## Overview

The reconciliation / file-state subsystem keeps the human-editable, git-tracked store and
the CLI's view of it honest with each other. Managed documents are plain files a human may
review and edit through git at any time, so an edit made outside the CLI (out-of-band) must
never be silently lost, silently merged, or forbidden. The subsystem detects such edits and
routes each to a deterministic verdict.

The model is a single raw-byte content hash per managed file — the **baseline** — recorded
in a rebuildable `path -> hash` map under `.jigc/state/file-state.json`. Every drift check
compares a file's current raw bytes against its recorded baseline. Three states follow:
UNKNOWN (no recorded hash — first encounter, a fresh checkout), IN_SYNC (the hash matches),
and DRIFTED (the hash differs). UNKNOWN baseline-adopts (the current content becomes the
baseline; absent-hash is not drift). DRIFTED is routed by the full reconciliation classifier.

A drifted committed document is routed on two axes. If the active task also staged that same
document, both sides moved — a conflict, blocked at file granularity with an explicit-discard
route and never silently merged. Otherwise the on-disk bytes are re-parsed and schema-checked:
a clean edit is **absorbed** (the edge index folds in any new forward edges and the baseline
advances), while a malformed edit is **conformance-blocked** (the baseline is pinned and
nothing is auto-repaired). A separately-handled case is a tracked path gone missing on disk:
the rename detector emits a strong signal (an untracked file with the same content hash — a
suspected `git mv`) or a weak signal (simply gone — restore it), always routing to a
human-side revert and never rewriting referrer references.

The recorded baseline advances at exactly three sites — baseline-adopt, absorb, and commit.
Crucially, an absorb's baseline advance is persisted durably only at a landed finalize: the
per-task preflight sweep classifies and routes in memory so `task validate` previews exactly
what finalize gates on, but only a committed finalize threads the post-sweep record through
to disk. The engine layer is pure of I/O and of any shell-out — it is handed the bytes and
returns findings plus in-place mutations to the record and edge index; the CLI layer owns the
filesystem walk, the git facts, and persistence.

## Components

### Raw-byte drift hash  {#raw-byte-drift-hash}

The drift hash hashes a file's raw bytes to a lowercase-hex blake3 digest. Hashing raw
bytes rather than the canonicalized form is deliberate: a conformant no-op read/write does
not register as drift, and a first-touch canonicalization re-baselines cleanly. It is the
single primitive every drift comparison and rename check is built on.

<!-- fields -->
- implemented-by: crates/engine/src/file_state.rs#hash_bytes

### File-state baseline record  {#file-state-baseline-record}

The baseline record is the recorded last-known-good state: a path -> hex-hash map persisted
as byte-stable JSON at .jigc/state/file-state.json. It is a path-sorted BTreeMap so the
serialized bytes are deterministic, and it carries no schema version of its own — it is a
rebuildable cache, re-derivable from the committed docs at any time, hence gitignored. A
missing file is the first-encounter case and loads as an empty record, never an error.

<!-- fields -->
- implemented-by: crates/engine/src/file_state.rs#FileStateRecord

### The file-state probe  {#the-file-state-probe}

The engine-native file-state probe classifies each (path, bytes) pair against the recorded
baseline into the three MVP states. UNKNOWN baseline-adopts (records the hash, emits an
advisory baseline-adopt finding — absent-hash is not drift); a matching hash is IN_SYNC and
emits nothing; a differing hash is drift — a blocking finding carrying a reconcile route.
The probe does no I/O of its own (the caller supplies the bytes) and never advances the
recorded hash on drift: re-baselining happens only at the three named sites.

<!-- fields -->
- implemented-by: crates/engine/src/file_state.rs#file_state

### OOB reconciliation classifier  {#oob-reconciliation-classifier}

The reconciliation classifier is the full out-of-band state machine over one committed
managed document. UNKNOWN baseline-adopts; IN_SYNC is a clean no-op; DRIFTED + task-touched
is a conflict (both sides moved) — a blocking conflict-block with an explicit-discard route,
never a silent merge. DRIFTED + untouched routes to the parse classifier: a clean re-parse +
schema check absorbs (re-hashes the baseline forward and incrementally folds the doc's new
forward edges into the committed edge index, emitting an advisory absorb finding), while a
parse or schema failure conformance-blocks (baseline pinned, index untouched, no auto-repair).

<!-- fields -->
- implemented-by: crates/engine/src/file_state.rs#reconcile_committed

### Committed-store sweep  {#committed-store-sweep}

The committed-store sweep is the command-surface wiring of the classifier and rename
detector over every tracked committed managed doc. For each persisted (location-bearing)
schema it walks <location>/*.md, derives each doc's type:slug identity and record key, and
runs the classifier — marking task_touched when the active task stages the same identity.
A recorded path no longer present on disk is routed to rename detection. It reads the .md
bytes but shells out for nothing (the engine stays shell-free); findings aggregate in a
stable type-then-path-sorted order. This is the sweep task validate and finalize's preflight
both share.

<!-- fields -->
- implemented-by: crates/engine/src/file_state.rs#reconcile_committed_store

### Rename detection  {#rename-detection}

Rename detection is the separate classifier for a tracked managed-doc path gone missing on
disk — the state machine alone cannot catch a git mv, since it sees the old path as absent
and the moved file as a fresh untracked file. Given the missing path, its identity, its
recorded hash, and the untracked candidates, it emits one blocking rename finding: a strong
signal (an untracked path with the same recorded content hash — a suspected git mv, routed
to revert the move) or a weak signal (no match — the file is gone, routed to restore). It
rewrites no referrer refs and mutates no edges: a path rename is an identity change handed
back to the human.

<!-- fields -->
- implemented-by: crates/engine/src/file_state.rs#detect_rename

### Baseline persistence at landed finalize  {#baseline-persistence-at-landed-finalize}

Baseline persistence is the CLI-side post-commit step that advances the recorded baseline
durably — and only at a landed finalize. The per-task preflight runs the sweep and produces
a post-sweep record in memory, but that record is threaded through to disk only when the
commit actually lands; a standalone task validate (and any non-committing sweep) drops it,
so read verbs stay pure readers. On a landed commit the post-sweep record is the base the
commit's own hash updates land on — so an absorbed OOB baseline the commit never touched
persists too — with per-run staged working-area keys stripped first. The advance is
best-effort and self-heals, since the commit is already truth and the record is rebuildable.

<!-- fields -->
- implemented-by: crates/cli/src/task.rs#advance_file_state
