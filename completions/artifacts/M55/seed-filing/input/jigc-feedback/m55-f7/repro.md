```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
PATH=$(dirname "$JIGC"):$PATH                         # the Spawn: line runs `jigc` off PATH
$JIGC milestone create "Probe wave" > /dev/null
$JIGC milestone add-task probe-wave "fix the greeting" --workflow single-task > /dev/null
$JIGC milestone provision probe-wave > /dev/null
spawn=$($JIGC milestone execute probe-wave | sed -n 's/^.*Spawn: `\(.*\)`.*$/\1/p' | head -1)
eval "$spawn" > "$RIG/composed.txt"; echo "spawn: exit $?"            # the emitted line, verbatim: 0
grep 'jigc task finalize' "$RIG/composed.txt"; echo "grep: exit $?"  # 1: no per-task door
grep -o 'jigc milestone finalize probe-wave` is its only commit boundary' "$RIG/composed.txt"
```
