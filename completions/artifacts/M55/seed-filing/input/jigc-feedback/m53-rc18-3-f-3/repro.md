```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Axis three probe" > /dev/null
$JIGC milestone add-task axis-three-probe "first sub" > /dev/null
$JIGC milestone provision axis-three-probe
cd .jigc/worktrees/first-sub
$JIGC uninstall; echo "exit $?"             # 0, from inside the fan-out worktree
cd "$REPO"
ls -d .jigc .git/hooks/pre-commit           # both gone: the MAIN install was removed
git worktree list | wc -l                   # 1: the worktree registration was pruned
```
