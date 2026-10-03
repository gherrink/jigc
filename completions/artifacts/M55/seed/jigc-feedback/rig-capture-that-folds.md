---
kind: inconvenience
found-in: milestone:M53-post-review/rig-eval-capture
about: dev:jigc-rig
jigc-version: 1.0.0-rc.21
status: open
date: 2026-10-03
schema-version: 1
---

# A rig capture that folds stderr in half-applies its assignments

## Description

`dev/jigc-rig` prints its assignments as text the caller must `eval`. Twice in one M53 session a subagent captured it as `rig=$(dev/jigc-rig … 2>&1)`. The rig's construction log, which goes to stderr by design, was folded into the captured assignments, `$REPO` came out wrong, and the probe's next `git -C "$REPO" commit` landed in this repository. Both commits were reset before any push. The candidate fix is that the rig writes its assignments to a file under its own `mktemp -d` root and prints one line, `source <path>`, so a corrupted capture fails to evaluate instead of half-evaluating. `dev_rig_parity.rs`'s scan of the emitted script would move with it. The row's trigger is the next wave that touches `dev/`, or a third occurrence.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. `dev/jigc-rig fresh` still prints `export …` lines and a `cd` for the caller to evaluate. Captured with `2>&1`, the capture exits 0, the `eval` of it exits 1 on the construction log's first line, `$REPO` is empty, and the shell is still in the jigc checkout. A `git -C "$REPO"` run next would act on that checkout.

## Repro

```sh
src=$PWD                                         # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh 2>&1); echo "capture: exit $?"   # 0
eval "$rig" > /dev/null 2>&1; echo "eval: exit $?"               # 1: the construction log is not shell
echo "REPO=[$REPO]"                              # REPO=[]
test "$PWD" = "$src" && echo "still in the jigc checkout"        # git -C "$REPO" … would act here
```

## Resolution
