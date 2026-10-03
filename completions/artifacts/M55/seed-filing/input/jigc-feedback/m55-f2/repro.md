```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
fill_commit() {
  $JIGC doc set-field commit:$1#type --value docs --task $1 > /dev/null
  $JIGC doc set-field commit:$1#scope --value probe --task $1 > /dev/null
  printf '%s\n' "$1 lands" | $JIGC doc set-slot commit:$1#summary --from-file - --task $1 > /dev/null
  printf 'why\n' | $JIGC doc set-slot commit:$1#body --from-file - --task $1 > /dev/null
}
T=$($JIGC start --workflow report-jigc-feedback "first report" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "Probe finding" --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/kind --value bug --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/found-in --value task:probe --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/jigc-version --value 1.0.0-rc.22 --task $T > /dev/null
printf 'The first finding.\n' | $JIGC doc set-slot jigc-feedback:probe-finding#description --from-file - --task $T > /dev/null
fill_commit $T; $JIGC task finalize $T > /dev/null 2>&1; echo "first lands: exit $?"     # 0
U=$($JIGC start --workflow report-jigc-feedback "second report" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "Probe finding" --task $U; echo "exit $?"        # 1, create.already-exists
$JIGC doc create jigc-feedback --title "Probe: finding" --task $U; echo "exit $?"       # 1, the same refusal
git status --short docs                                                                 # nothing staged
```
