```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow report-jigc-feedback "two findings" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "First finding" --task $T; echo "exit $?"    # 0
$JIGC doc create jigc-feedback --title "Second finding" --task $T; echo "exit $?"   # 1, write.identity-change
```
