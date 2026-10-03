```sh
# from the jigc checkout: one full gate in a fresh TMPDIR, then what it left
T=$(mktemp -d "${TMPDIR:-/tmp}/gate-tmpdir.XXXXXX")
TMPDIR="$T/" dev/gate; echo "gate: exit $?"           # GATE: PASS
ls -A "$T" | wc -l                                    # 10, five of them the gate's own logs
```
