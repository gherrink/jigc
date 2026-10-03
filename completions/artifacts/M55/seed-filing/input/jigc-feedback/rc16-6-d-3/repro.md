```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start
#   Preview: `jigc workflow <id> --preview`   — … a workflow that mints nothing has no
#     preview — `jigc start --workflow <id>` composes it directly, and mints nothing either
$JIGC describe --workflows      # … Neither reaches a workflow whose line below says it is
#                                 reached only through a verb … both of these refuse it by name.
$JIGC start --workflow milestone-execution     # exit 1, workflow.verb-routed
$JIGC start --workflow router                  # control: exit 0
```
