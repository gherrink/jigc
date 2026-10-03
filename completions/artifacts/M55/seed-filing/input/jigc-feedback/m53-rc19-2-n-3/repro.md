```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC milestone create "Reuse wave"                  # "… (shared base <pin>)"
$JIGC milestone add-task reuse-wave "ru one" > /dev/null
printf 'x\n' > extra.txt; git add extra.txt; git -c core.hooksPath=/dev/null commit -qm "chore: advance"
git worktree add -q --detach "$REPO/.jigc/worktrees/ru-one"   # parked by hand at HEAD, not the pin
git -C .jigc/worktrees/ru-one rev-parse --short HEAD # the commit after the pin
$JIGC milestone provision reuse-wave; echo "exit $?" # 0, "… at base <pin> (ru-one)"
git -C .jigc/worktrees/ru-one rev-parse --short HEAD # unchanged: not the pin the ack names
$JIGC milestone finalize reuse-wave; echo "exit $?"  # 3, finalize.base-mismatch, "cut from <pin>"
```
