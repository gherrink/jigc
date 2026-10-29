```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"
before=$(git rev-list --count HEAD)
$JIGC milestone add-task before-the-first-stabilization-run "D0 — write down the rulings"
$JIGC milestone add-task before-the-first-stabilization-run "P0 — generate the overview"
echo $(( $(git rev-list --count HEAD) - before ))     # 2
```
