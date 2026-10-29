```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Before the first stabilization run opens"       # milestone:before-the-first-stabilization-run
$JIGC milestone add-task before-the-first-stabilization-run "D0 — write down the rulings, the owed rows and the tool candidates"
# added task:d0-write-down-the-rulings
```
