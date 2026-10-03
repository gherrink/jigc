```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'compose-embedded-methodology: false\n' > .jigc/config/packs.yaml    # a hand opt-out
git commit -qam "opt out of the methodology pack"
$JIGC doc schema vision > /dev/null 2>&1; echo "vision: exit $?"          # 1: the opt-out holds
$JIGC setup > /dev/null 2>&1; echo "setup: exit $?"                       # 0
cat .jigc/config/packs.yaml                                                # compose-embedded-methodology: true
git show --name-only --format=%s HEAD                                      # the revert, committed as setup's install commit
$JIGC setup --help | grep -ci 'methodology'                               # 0: no opt-out flag
```
