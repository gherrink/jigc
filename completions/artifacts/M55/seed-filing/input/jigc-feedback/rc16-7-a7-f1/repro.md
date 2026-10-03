```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git rev-parse --short HEAD
rm CHANGELOG.md                              # uncommitted, unstaged worktree deletion
$JIGC validate                               # exit 1
#   … route: `jigc unmanage CHANGELOG.md`                                  (fine)
#   … route: the removal is not committed — restore it: `git -C … checkout -- CHANGELOG.md` (fine)
#   out-of-band rename detected — a structural-identity change this commit introduced; the
#   sweep exits non-zero (revert the `git mv` or adopt it via `jigc rename`).
git rev-parse --short HEAD                   # unchanged
git status --short                           #  D CHANGELOG.md
git log -1 --name-status --diff-filter=R     # no rename rows
```
