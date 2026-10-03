```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
fill_commit() {
  $JIGC doc set-field commit:$1#type --value feat --task $1 > /dev/null
  $JIGC doc set-field commit:$1#scope --value probe --task $1 > /dev/null
  printf '%s\n' "$1 lands" | $JIGC doc set-slot commit:$1#summary --from-file - --task $1 > /dev/null
  printf 'why\n' | $JIGC doc set-slot commit:$1#body --from-file - --task $1 > /dev/null
}
$JIGC start --workflow single-task "task one" > /dev/null
$JIGC start --workflow single-task "task two" > /dev/null
printf 'one\n' > one.sh; git add one.sh           # task-one's code
printf 'two\n' > two.sh; git add two.sh           # task-two's code, staged after both started
fill_commit task-one; fill_commit task-two
$JIGC task finalize task-one > /dev/null 2>&1; echo "task-one: exit $?"   # 0
git show --name-only --format=%s HEAD             # one.sh AND two.sh, under task-one's message
$JIGC task finalize task-two; echo "task-two: exit $?"                    # 3, finalize.empty-commit
```
