```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start --workflow single-task "posture parity"
git bisect start
$JIGC start                                   # exit 0; findings: 3 blocking, 1 advisory
#   blocking · repo.operation-in-progress — a bisect is in progress — …
$JIGC --format json start                     # tasks[].findings[].code carries
#   repo.operation-in-progress
$JIGC task validate posture-parity            # exit 1, the same code
```
