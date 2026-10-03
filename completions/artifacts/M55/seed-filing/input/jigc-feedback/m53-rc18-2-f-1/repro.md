```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC milestone create "Cwd wave" > /dev/null
$JIGC milestone add-task cwd-wave "cw area" > /dev/null
$JIGC milestone provision cwd-wave
W=$REPO/.jigc/worktrees/cw-area
git -C "$W" bisect start; git -C "$W" bisect bad
mkdir -p "$REPO/docs/deep"; cd "$REPO/docs/deep"          # an ordinary subdirectory
$JIGC milestone finalize cwd-wave; echo "exit $?"         # 3, repo.operation-in-progress
#   route: … abandon it with `git -C <absolute repo>/.jigc/worktrees/cw-area bisect reset` …
span=$($JIGC milestone finalize cwd-wave 2>&1 | grep -o 'git -C [^`]*bisect reset' | head -1)
eval "$span"; echo "exit $?"                              # the emitted span, run here: 0
```
