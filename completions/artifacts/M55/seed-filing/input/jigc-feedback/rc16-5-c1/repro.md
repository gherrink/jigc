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
