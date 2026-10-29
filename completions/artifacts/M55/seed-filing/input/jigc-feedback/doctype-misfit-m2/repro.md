```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC doc schema milestone-record   # tasks: task-id · intent · workflow · status — no spec
$JIGC doc schema roadmap            # decomposition is one prose slot
```
