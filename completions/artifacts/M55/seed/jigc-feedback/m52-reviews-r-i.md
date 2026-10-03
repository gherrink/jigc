---
kind: feedback
found-in: milestone:M53-settle/(D)(f)
jigc-version: 1.0.0-rc.16
status: open
date: 2026-10-03
schema-version: 1
---

# The M52 review's R-I loss evidence cannot show that bytes are gone

## Description

`completions/artifacts/M52/per-axis-review/axis-3.md` §3 R-I shows that the bytes are gone from disk with `grep -rl '<marker>' .` lines. In the harness the review ran in, `grep` is a shell function over `ugrep --ignore-files`, which honours `.gitignore`, and `.jigc/`'s working areas are gitignored (`.jigc/.gitignore` lists `tasks/`, `milestones/` and the rest). So that search answers *not found* there whether the bytes are present or not. The row's other leg, `git grep -l '<marker>' HEAD`, is sound, but it answers a different question: whether the bytes were committed, which was never in doubt. The verdict stands: two M53 auditors hit the same instrument independently and re-proved every loss with `command grep` plus a before-control, `git log -S` and `git fsck`. What is owed is the recorded evidence, not the finding: R-I's repro block re-driven and replaced in place, with the instrument note beside it, because a repro nobody can re-run makes a true verdict unfalsifiable. It is a finding about a review record, not the binary, so its kind is `feedback` and it names no `about`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. §3 R-I still carries its three `grep -rl` evidence lines, and no instrument note. In a fresh rig, a marker planted in `.jigc/tasks/` sits at a path `git check-ignore` reports as ignored, so a `.gitignore`-honouring search skips it, and `command grep -rl` finds it.

## Repro

```sh
src=$PWD                                          # the jigc checkout
sed -n '/^### R-I /,/^### R-J /p' "$src"/completions/artifacts/M52/per-axis-review/axis-3.md \
  | grep -c 'grep -rl'                            # 3: the evidence lines are unchanged
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
mkdir -p .jigc/tasks; printf 'PRECIOUS-MARKER\n' > .jigc/tasks/plant.txt
git check-ignore .jigc/tasks/plant.txt            # .jigc/tasks/plant.txt: ignored, so a search that
                                                  # honours .gitignore (ugrep --ignore-files, rg) skips it
command grep -rl PRECIOUS-MARKER .                # ./.jigc/tasks/plant.txt: the bytes are there
```

## Resolution
