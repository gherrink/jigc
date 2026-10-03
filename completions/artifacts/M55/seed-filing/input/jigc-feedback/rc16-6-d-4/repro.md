```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Empty probe"
$JIGC milestone execute empty-probe > "$RIG/o" 2> "$RIG/e"    # exit 0
grep -c '^Spawn:' "$RIG/o"                                    # 0
wc -c < "$RIG/e"                                              # 0
#   "Spawn a sub-agent per sub-task (one per `Spawn:` line below) …" — and no line below
$JIGC milestone finalize empty-probe                          # exit 3, milestone.zero-contribution
```
