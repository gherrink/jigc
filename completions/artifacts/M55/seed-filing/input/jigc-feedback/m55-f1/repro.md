```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start --workflow single-task "code work" > /dev/null                 # a code task, open
T=$($JIGC start --workflow report-jigc-feedback "file a finding" | sed -n 's/^task minted: //p')
printf 'echo hi\n' > greet.sh; git add greet.sh                           # its code, staged AFTER the report started
$JIGC doc create jigc-feedback --title "Probe finding" --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/kind --value bug --task $T
$JIGC doc set-field jigc-feedback:probe-finding#meta/found-in --value task:probe --task $T
$JIGC doc set-field jigc-feedback:probe-finding#meta/jigc-version --value 1.0.0-rc.22 --task $T
printf 'Observed.\n' | $JIGC doc set-slot jigc-feedback:probe-finding#description --from-file - --task $T
$JIGC doc set-field commit:$T#type --value docs --task $T
$JIGC doc set-field commit:$T#scope --value findings --task $T
printf 'file a probe finding\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T
printf 'A probe.\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T
$JIGC task finalize $T; echo "exit $?"           # 0, `left-out … greet.sh`
git show --name-only --format=%s HEAD           # docs/jigc-feedback/probe-finding.md alone
git diff --cached --name-only                   # greet.sh: still staged for the code task
```
