```sh
rig=$(dev/jigc-rig --git-state unborn) || exit; eval "$rig"
$JIGC setup > /dev/null 2>&1                    # births HEAD: a root commit, which amend refuses
$JIGC task amend "root probe"; echo "exit $?"   # 1, amend.head-shape, at: task:root-probe
$JIGC task amend; echo "exit $?"                # 1, amend.head-shape, at: task:amend-<sha7>
ls .jigc/tasks                                  # No such file or directory: nothing minted
```
