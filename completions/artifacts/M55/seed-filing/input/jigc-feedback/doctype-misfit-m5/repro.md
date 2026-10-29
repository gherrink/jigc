```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
$JIGC milestone add-task before-the-first-stabilization-run "U0 — give CI room" --workflow builder
# blocking workflow-refs.unknown-workflow — nothing minted
```
