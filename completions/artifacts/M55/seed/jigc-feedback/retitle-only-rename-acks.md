---
kind: bug
found-in: review:M52-per-axis/(8,N-2)
about: jigc rename
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# A retitle-only rename acks and commits a no-op path move

## Description

Every retitle-without-reslug at `jigc rename` acks a no-op and commits one, while the binary knows the title it just moved. M52 Increment 6 minted `write.identity-change`, whose route is a specific argv whose stated effect is a title rewrite. Run, that argv rewrites the title. It reports `renamed vision:vision -> vision:vision (VISION.md -> VISION.md), repointed 0 referrer(s)` and commits under the subject `rename VISION.md -> VISION.md`. The title, the only thing that moved and the thing the route promised, appears on neither surface, nor in the permanent git record. The binary has the datum: re-run the same argv and the no-op arm prints the title. So the ack for a failure to act is fully informative, and the ack for a successful act is not. The review also drove it on a slugged doctype.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `jigc rename vision:vision --to "New Vision"` exits 1 with `write.identity-change`, routing at `` `jigc rename vision:vision --to 'New Vision' --slug vision` ``. That argv, run verbatim, exits 0. `VISION.md`'s H1 becomes `# New Vision`, and the ack and the commit subject are those above. A second run prints *no-op: vision:vision already holds the title "New Vision" at VISION.md*.

## Repro

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

## Resolution
