```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema milestone-record   # meta: base · status; tasks: task-id · intent · workflow · status — no place for a finding
```
