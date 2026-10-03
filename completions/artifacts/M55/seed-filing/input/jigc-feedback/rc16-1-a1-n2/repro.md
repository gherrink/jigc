```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC config set placement-root -             # exit 1
#   blocking · config.repoint-failed — `placement-root` was not set to `-`:
#     `git mv docs/decisions-log.md -/decisions-log.md` failed: error: unknown switch `/'
#   route: `placement-root` is unchanged … Fix what this message names, then re-run
#     `jigc config set placement-root -`
#   (the same again for docs/roadmap.md)
$JIGC config get placement-root               # unchanged: the pack default
```
