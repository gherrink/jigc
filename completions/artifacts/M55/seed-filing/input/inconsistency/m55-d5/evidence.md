```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow report-inconsistency "probe sides" | sed -n 's/^task minted: //p')
$JIGC doc create inconsistency --title "Probe sides" --task $T > /dev/null
$JIGC doc add-item inconsistency:probe-sides#sides --title a.md --slug side1 --task $T > /dev/null
$JIGC doc add-item inconsistency:probe-sides#sides --title b.md --slug side2 --task $T > /dev/null
$JIGC doc remove-item inconsistency:probe-sides#sides/side2 --task $T; echo "exit $?"   # removed item …, exit 0
$JIGC doc show inconsistency:probe-sides --task $T | grep -c side2                   # 0
```
