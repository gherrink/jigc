```sh
rig=$(dev/jigc-rig fresh --pack-from-dev) || exit; eval "$rig"     # exports $JIGC_PACK_DIR
mkdir -p .jigc/config/workflows                                    # a project shadow: a whole-file copy
cp "$JIGC_PACK_DIR/workflows/single-task.yaml" .jigc/config/workflows/single-task.yaml
git add .jigc/config/workflows && git commit -qm "shadow single-task"
printf '{{ include: step:locate }}\n' >> "$JIGC_PACK_DIR/workflows/single-task.yaml"   # the pack moves on
cmp -s "$JIGC_PACK_DIR/workflows/single-task.yaml" .jigc/config/workflows/single-task.yaml; echo "cmp: exit $?"   # 1
$JIGC validate; echo "validate: exit $?"                           # 0, no findings
$JIGC start --workflow single-task "probe" --explain | grep 'workflow:single-task'   # (project · …): the shadow wins
ls .jigc/config                                                    # no recorded base for it
```
