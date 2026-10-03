```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
T=$($JIGC start --workflow park-idea "park a thought" | sed -n 's/^task minted: //p')   # allows-create: idea only
printf 'A thesis rewritten by a task that may create only an idea.\n' |
  $JIGC doc set-slot vision:vision#thesis --from-file - --task $T; echo "set-slot: exit $?"   # 0
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value ideas --task $T > /dev/null
printf 'park a thought\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'why\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "finalize: exit $?"                     # 0
git show --name-only --format=%s HEAD                                                 # VISION.md
```
