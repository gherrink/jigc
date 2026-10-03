```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC rename vision --to "New Vision"                 # exit 1, no code
#   `vision` is not a `<type>:<slug>` address — e.g. `adr:single-node-cache`
#     route: run `jigc describe` for the doctype surface
$JIGC --format json rename vision --to "New Vision"   # {"error": "`vision` is not a …"}
$JIGC --format json doc show nosuchtype               # {"error": "malformed address …"}
$JIGC doc show vision                                 # control: exit 0, the bare form is accepted
```
