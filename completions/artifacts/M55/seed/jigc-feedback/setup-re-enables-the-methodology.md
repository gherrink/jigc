---
kind: bug
found-in: milestone:M55-planning/F15
about: jigc setup
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# Setup re-enables the methodology pack over a hand opt-out

## Description

`jigc setup` reverts a hand opt-out of the methodology pack: it always rewrites `compose-embedded-methodology` to `true` in `.jigc/config/packs.yaml`, and there is no opt-out flag. The baseline router auditor found it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. With `compose-embedded-methodology: false` committed, `jigc doc schema vision` is refused, as the opt-out intends. A second `jigc setup` exits 0 and commits `.jigc/config/packs.yaml` back to `compose-embedded-methodology: true`, as its own install commit, `chore(jigc): install jigc workspace config`. Its ack names neither the file nor the methodology pack. `jigc setup --help` names no methodology flag.

## Repro

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

## Resolution
