```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC doc schema vision --format json        # fields[]: grounded-in, required: false,
#                                              set-field: vision:<slug>#meta/grounded-in
$JIGC doc show 'vision:vision#meta/grounded-in'      # exit 1
#   blocking · store.no-such-leaf — `vision:vision#meta/grounded-in` names no leaf
#     `grounded-in` in section `meta`
$JIGC doc show 'vision:vision#meta/schema-version'   # control: exit 0
```
