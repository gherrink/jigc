```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
append_entry() {   # append_entry <title> <id>: one task that appends a decisions-log entry and lands it
  local t; t=$($JIGC start --workflow single-task "log $1" | sed -n 's/^task minted: //p')
  $JIGC doc add-item decisions-log:decisions-log#entries --title "$1" --task $t > /dev/null
  printf 'Because %s.\n' "$1" | $JIGC doc set-slot "decisions-log:decisions-log#entries/$2/why" --from-file - --task $t > /dev/null
  $JIGC doc set-field commit:$t#type --value docs --task $t > /dev/null
  $JIGC doc set-field commit:$t#scope --value log --task $t > /dev/null
  printf 'log %s\n' "$1" | $JIGC doc set-slot commit:$t#summary --from-file - --task $t > /dev/null
  $JIGC task finalize $t > /dev/null 2>&1; echo "append $2: exit $?"
}
git switch -q -c side; append_entry "Alpha choice" alpha-choice      # 0
git switch -q main;    append_entry "Beta choice" beta-choice        # 0
git merge -q side > /dev/null 2>&1; echo "merge: exit $?"            # 1: both appended at the end
git show :2:docs/decisions-log.md > "$RIG/ours"; git show :1:docs/decisions-log.md > "$RIG/base"
git show :3:docs/decisions-log.md > "$RIG/theirs"
git merge-file --union "$RIG/ours" "$RIG/base" "$RIG/theirs"; cp "$RIG/ours" docs/decisions-log.md   # the naive union
git add docs/decisions-log.md; git commit -qm "merge side"
grep -c '^- date:' docs/decisions-log.md            # 1: two entries, one date
$JIGC validate; echo "validate: exit $?"            # 0, one advisory file-state.hash-matches
```
