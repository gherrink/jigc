```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Quebec probe" > /dev/null
$JIGC milestone add-task quebec-probe "area one" > /dev/null   # not provisioned
mkdir -p .jigc/worktrees
printf 'LEFTOVER-FILE-BYTES\n' > .jigc/worktrees/area-one        # a FILE at the worktree path
$JIGC milestone provision quebec-probe; echo "exit $?"          # 1, refusal arm: "the file itself"
$JIGC milestone provision quebec-probe --force; echo "exit $?"  # 0
#   warning: removing the leftover file .jigc/worktrees/area-one discards work that is not in git:
#       area-one          <- the path's own basename, in the child position
```
