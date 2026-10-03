---
kind: bug
found-in: review:M52-per-axis/(5,C1)
about: jigc start
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Orientation JSON omits its declared next_steps key

## Description

`jigc start`'s two pinned orientation rows in `ENVELOPE_ARMS`, `OrientationView::Clean` and `OrientationView::ActiveTask`, declare `next_steps`, and a reachable composition omits it. `next_steps` is `skip_serializing_if = "Vec::is_empty"`. Its producer returns nothing when the composed pack-set ships neither `planning` nor `ingest-existing`, and the binary loads such a pack-set without complaint. The registry's declared bound covers a **missing row**. Here the row exists, and its declared key set is false for a reachable state. The finding originated in the Codex source pass and was then driven. The M53 re-reviews kept it open at both arms through `1.0.0-rc.20`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. On a `--pack-from-dev` rig the control carries `next_steps` (`ingest-existing`). With `ingest-existing.yaml` moved out of the throwaway pack, the `Clean` arm drives `['header', 'schema_version', 'state', 'workflows']`. With one task minted, the `ActiveTask` arm drives `['header', 'schema_version', 'state', 'tasks', 'workflows']`. Both lack the declared `next_steps`.

## Repro

```sh
rig=$(dev/jigc-rig fresh --pack-from-dev) || exit; eval "$rig"     # exports $JIGC_PACK_DIR
keys() { python3 -c 'import json,sys; print(sorted(json.load(sys.stdin)))'; }
$JIGC --format json start | keys     # control: [..., 'next_steps', ...]
STASH=$(mktemp -d "${TMPDIR:-/tmp}/packstash.XXXXXX")
mv "$JIGC_PACK_DIR/workflows/ingest-existing.yaml" "$STASH/"       # a move, not a delete
$JIGC --format json start | keys     # Clean: ['header', 'schema_version', 'state', 'workflows']
$JIGC start --workflow single-task "c one probe"
$JIGC --format json start | keys     # ActiveTask: ['header', 'schema_version', 'state', 'tasks', 'workflows']
mv "$STASH/ingest-existing.yaml" "$JIGC_PACK_DIR/workflows/"       # restore
```

## Resolution
