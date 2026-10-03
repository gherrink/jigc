```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
OUT=$(mktemp -d); printf 'OUTSIDE2-KEEP\n' > "$OUT/keep.txt"
mkdir -p .jigc/tasks; ln -s "$OUT" .jigc/tasks/ghost        # a symlink, not a directory
$JIGC task discard ghost; echo "exit $?"
#   1, finalize.no-task — no task `ghost`: `.jigc/tasks/ghost` is a directory carrying no base pin …
$JIGC task validate ghost; echo "exit $?"                   # 1, the byte-identical sentence
cat "$OUT/keep.txt"                                         # OUTSIDE2-KEEP: nothing destroyed
mkdir .jigc/tasks/ghost-dir                                 # the control: a real directory
$JIGC task discard ghost-dir; echo "exit $?"                # 1, the same sentence, true here
```
