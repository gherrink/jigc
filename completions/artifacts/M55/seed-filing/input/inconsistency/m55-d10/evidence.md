```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
shasum .jigc/state/file-state.json
$JIGC start --workflow park-idea "probe idea" > /dev/null; echo "exit $?"   # exit 0
ls .jigc/tasks                                                 # probe-idea
shasum .jigc/state/file-state.json                             # the same hash
```
