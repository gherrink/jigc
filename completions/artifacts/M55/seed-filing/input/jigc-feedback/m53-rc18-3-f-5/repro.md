```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Axis three probe" > /dev/null
$JIGC milestone add-task axis-three-probe "first sub" > /dev/null
$JIGC milestone add-task axis-three-probe "second sub" > /dev/null
$JIGC milestone provision axis-three-probe
for s in first-sub second-sub; do
  printf '%s\n' "$s" > ".jigc/worktrees/$s/$s.txt"; git -C ".jigc/worktrees/$s" add "$s.txt"
done
SAVE=$(mktemp -d); cp -R .jigc/tasks/second-sub "$SAVE/"     # a faithful backup, pin and all
$JIGC milestone join axis-three-probe
$JIGC milestone finalize axis-three-probe; echo "exit $?"    # 0
cp -R "$SAVE/second-sub" .jigc/tasks/second-sub              # restored, as a backup would be
$JIGC task list; echo "exit $?"                              # 0, "1 active task(s)": second-sub
$JIGC task discard second-sub; echo "exit $?"                # 1, milestone.terminal
$JIGC task discard second-sub --force; echo "exit $?"        # 1, milestone.terminal
$JIGC task finalize second-sub; echo "exit $?"               # 3, routed at the refusing --force
```
