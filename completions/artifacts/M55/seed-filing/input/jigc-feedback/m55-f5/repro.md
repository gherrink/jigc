```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git switch -q -c milestone/x/main
T=$($JIGC start --workflow single-task "record the cache decision" | sed -n 's/^task minted: //p')
$JIGC doc create adr --title "Single-node cache" --task $T > /dev/null
for s in context decision consequences; do
  printf 'The %s.\n' "$s" | $JIGC doc set-slot adr:single-node-cache#$s --from-file - --task $T > /dev/null; done
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value cache --task $T > /dev/null
printf 'record the cache decision\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "finalize on the branch: exit $?"    # 0
git switch -q main                                        # the ADR lives on the branch only
$JIGC validate; echo "validate: exit $?"                  # 0, advisory reconciliation.rename, routed at the branch switch
$JIGC validate 2>&1 | grep -c 'jigc unmanage'             # 0
```
