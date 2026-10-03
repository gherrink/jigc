```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC task amend ""; echo "exit $?"          # 1, write.unslugable-title; the route names the bare door
$JIGC task list                              # no active tasks: nothing minted
$JIGC task amend > /dev/null; echo "exit $?" # 0, the exit the route names
$JIGC task list                              # amend-<sha7>
$JIGC milestone create ""; echo "exit $?"    # 1, the control: no fallback there, and the route says so
```
