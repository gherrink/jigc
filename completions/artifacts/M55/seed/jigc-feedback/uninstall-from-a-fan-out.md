---
kind: bug
found-in: review:M53-per-axis-rc18/(3,F-3)
about: jigc uninstall
jigc-version: 1.0.0-rc.18
status: resolved
tier: tier-2
pinned-by: cwd_verb_subject::uninstall_removes_the_workbench_home_install_from_every_cwd
date: 2026-10-03
schema-version: 1
---

# Uninstall from a fan-out worktree reports an install it did not remove

## Description

Run inside a provisioned fan-out worktree, `jigc uninstall` exited 0 and acked *repo-local install removed* with seven removal lines. Yet the main checkout's `.jigc/`, its `CLAUDE.md` preload line and `.claude/skills/jigc/SKILL.md` all stood, and `jigc start` still oriented. What it did remove was `.git/hooks/pre-commit`, the one copy every checkout of the repository shares. The door resolved its `.jigc/` from the cwd's checkout, while every other door resolved `jigc_home`, so it never met the dirty-worktree guard the main checkout raised (`uninstall.dirty-worktree`, exit 1). Three clauses of a destroying door's ack were false. No untracked byte died, because the one cell that would have lost one was guarded.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Axis three probe" > /dev/null
$JIGC milestone add-task axis-three-probe "first sub" > /dev/null
$JIGC milestone provision axis-three-probe
cd .jigc/worktrees/first-sub
$JIGC uninstall; echo "exit $?"             # 0, from inside the fan-out worktree
cd "$REPO"
ls -d .jigc .git/hooks/pre-commit           # both gone: the MAIN install was removed
git worktree list | wc -l                   # 1: the worktree registration was pruned
```

## Resolution

Fixed in M53's cwd-dependence arc, which bound `setup` and `uninstall` to `jigc_home` on the human's call, stamped `1.0.0-rc.19`. From inside the worktree the door removes the main install in full, prunes git's worktree registrations, and its site line says the workbench it removed held the worktree the reader is standing in. All four WIP guards fire byte-identically from the root and from the worktree. The rc.19 re-review recorded it *CLOSED*, and rc.20 *still CLOSED*.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). From `.jigc/worktrees/first-sub` of a provisioned one-sub-task milestone, `jigc uninstall` exits 0. It lists `removed .jigc/` and *pruned git's worktree registrations for the fan-out worktrees `.jigc/` held*, and ends *removed at `<repo>` — the main checkout this repository's jigc install and `.jigc/` workbench bind to, and that workbench held the worktree you are standing in, which this removed*. In the main checkout, `.jigc` and `.git/hooks/pre-commit` are both gone, and `git worktree list` prints one line.
