```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC rename vision:vision --to "New Vision"     # exit 1, write.identity-change
#   route: `jigc rename vision:vision --to 'New Vision' --slug vision` keeps the identity that
#     cannot move and rewrites only the title
$JIGC rename vision:vision --to 'New Vision' --slug vision     # the route, verbatim: exit 0
#   renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)
grep -m1 '^# ' VISION.md                         # # New Vision
git log -1 --format=%s                           # rename VISION.md -> VISION.md
$JIGC rename vision:vision --to 'New Vision' --slug vision     # exit 0
#   no-op: vision:vision already holds the title "New Vision" at VISION.md — nothing renamed,
#   nothing committed
```
