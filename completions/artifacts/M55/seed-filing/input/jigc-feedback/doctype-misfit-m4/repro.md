```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
m=before-the-first-stabilization-run
$JIGC milestone add-task $m "D0 — write down the rulings"
grep status docs/milestone-records/$m.md        # status: active, for a task nobody started
perl -pi -e 's/^- status: active$/- status:/' docs/milestone-records/$m.md   # the one task's status, emptied
git commit -qam "empty status"
$JIGC validate        # blocking schema-conformance.field-value-conformant
```
