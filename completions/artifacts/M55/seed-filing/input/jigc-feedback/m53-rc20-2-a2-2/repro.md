```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "cwd wave" > /dev/null
$JIGC milestone add-task cwd-wave "area one" > /dev/null
$JIGC milestone provision cwd-wave
cd .jigc/worktrees/area-one                                       # detached HEAD, jigc-provisioned
$JIGC start --workflow single-task "ordinary in worktree" > /dev/null   # an ORDINARY task
$JIGC doc set-field commit:ordinary-in-worktree#header/type --value fix --task ordinary-in-worktree
printf 'k\n' | $JIGC doc set-slot commit:ordinary-in-worktree#summary --from-file - --task ordinary-in-worktree
echo k > k.txt; git add k.txt
$JIGC task validate ordinary-in-worktree; echo "exit $?"          # 1, repo.head-detached
$JIGC task finalize ordinary-in-worktree --dry-run; echo "exit $?" # 1, the same finding
$JIGC task finalize ordinary-in-worktree; echo "exit $?"          # 1, the same finding
span=$($JIGC task finalize ordinary-in-worktree 2>&1 | grep -o 'git switch -c [^`]*' | head -1)
eval "${span/<new-branch>/rescue}"; echo "exit $?"                # the emitted route, run here: 0
$JIGC task finalize ordinary-in-worktree; echo "exit $?"          # 0, it lands on `rescue`
```
