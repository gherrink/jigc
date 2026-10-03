```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
report() {   # report <title> <slug> <intent>: one report-jigc-feedback task, filed and finalized
  local t; t=$($JIGC start --workflow report-jigc-feedback "$3" | sed -n 's/^task minted: //p')
  $JIGC doc create jigc-feedback --title "$1" --task $t > /dev/null
  $JIGC doc set-field jigc-feedback:$2#meta/kind --value bug --task $t > /dev/null
  $JIGC doc set-field jigc-feedback:$2#meta/found-in --value task:probe --task $t > /dev/null
  $JIGC doc set-field jigc-feedback:$2#meta/jigc-version --value 1.0.0-rc.22 --task $t > /dev/null
  printf 'Observed.\n' | $JIGC doc set-slot jigc-feedback:$2#description --from-file - --task $t > /dev/null
  $JIGC doc set-field commit:$t#type --value docs --task $t > /dev/null
  $JIGC doc set-field commit:$t#scope --value findings --task $t > /dev/null
  printf 'file %s\n' "$2" | $JIGC doc set-slot commit:$t#summary --from-file - --task $t > /dev/null
  printf 'why\n' | $JIGC doc set-slot commit:$t#body --from-file - --task $t > /dev/null
  $JIGC task finalize $t
}
git worktree add -q "$RIG/wt"; cd "$RIG/wt"                         # a user-created worktree
report "First worktree report" first-worktree-report first > /dev/null 2>&1; echo "first: exit $?"   # 0
$JIGC doc show jigc-feedback:first-worktree-report; echo "doc show: exit $?"                   # 1, store.not-found
report "Second worktree report" second-worktree-report second; echo "second: exit $?"         # 3, reconciliation.rename
```
