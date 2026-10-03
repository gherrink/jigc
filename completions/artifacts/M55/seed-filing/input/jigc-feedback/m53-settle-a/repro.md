```sh
rig=$(dev/jigc-rig committed-singletons --start single-task "probe titles") || exit; eval "$rig"
$JIGC doc add-item decisions-log#entries --title "!!!"; echo "exit $?"   # 1, write.unslugable-title
$JIGC doc add-item decisions-log#entries --title ""; echo "exit $?"
#   1, schema-conformance.field-value-conformant — the same condition, a second code, the same door
$JIGC doc create adr --title ""; echo "exit $?"                          # 1, create.empty-title, a third
git status --porcelain                                                    # empty: nothing written
```
