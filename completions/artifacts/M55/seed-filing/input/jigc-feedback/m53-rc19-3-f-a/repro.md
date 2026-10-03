```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
L=$(mktemp -d)/linked; git worktree add -q -b feat "$L"; cd "$L"
$JIGC start --workflow quick-fix "tidy the readme" > /dev/null
$JIGC doc set-field commit:tidy-the-readme#header/type --value fix
printf 'tidy\n' | $JIGC doc set-slot commit:tidy-the-readme#summary --from-file -
echo code > src.txt; git add src.txt
printf 'KEEPHP\n' > "$REPO/.jigc/tasks/tidy-the-readme/notes.txt"   # a byte jigc did not write
$JIGC --format json task finalize tidy-the-readme; echo "exit $?"
#   0; committed.displaced: from ".jigc/tasks/tidy-the-readme/notes.txt"
#                           to   ".jigc/displaced/tidy-the-readme/notes.txt"
cat "$REPO/.jigc/displaced/tidy-the-readme/notes.txt"                # KEEPHP: moved aside, not lost
```
