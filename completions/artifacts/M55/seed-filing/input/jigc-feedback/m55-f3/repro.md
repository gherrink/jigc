```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow park-idea "first park" | sed -n 's/^task minted: //p')    # no `new: true` on its entry
$JIGC doc create idea --title "Probe idea" --task $T > /dev/null
$JIGC doc set-field idea:probe-idea#trigger --value never --task $T > /dev/null
printf 'The first idea.\n' | $JIGC doc set-slot idea:probe-idea#description --from-file - --task $T > /dev/null
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value ideas --task $T > /dev/null
printf 'park an idea\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'why\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "idea lands: exit $?"                  # 0
U=$($JIGC start --workflow park-idea "second park" | sed -n 's/^task minted: //p')
$JIGC doc create idea --title "Probe: idea" --task $U; echo "exit $?"                # 1, write.title-ignored
#   route: choose a distinct `--title`, or keep this one and pass `--slug <slug>` …   <- not a rename of the committed doc
```
