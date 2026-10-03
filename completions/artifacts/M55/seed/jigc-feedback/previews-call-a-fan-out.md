---
kind: bug
found-in: review:M53-per-axis-rc20/(2,A2-2)
about: repo.head-detached
jigc-version: 1.0.0-rc.20
status: resolved
tier: tier-2
pinned-by: validate_previews_posture::an_ordinary_task_in_a_fan_out_worktree_is_previewed_as_the_door_refuses_it
date: 2026-10-03
schema-version: 1
---

# Previews call a fan-out worktree clean while task finalize refuses it

## Description

Inside a jigc-provisioned fan-out worktree, whose `HEAD` is detached, an ordinary task minted with `jigc start` previewed clean. `jigc task validate` exited 0 with no `repo.*` finding, `jigc task finalize --dry-run` exited 0 with *would commit*, and `jigc start`'s orientation carried no `repo.head-detached`. `jigc task finalize` then refused at exit 1 with `repo.head-detached`, routed at `git switch <branch>`. Run there, `git switch main` exits 128, because `main` is checked out in the main worktree. The preview layers exempt a dedicated worktree from the detached-HEAD member, so that jigc's own `--detach` provisioning is not refused, and the commit seam does not take that exemption. Four locked homes say the preview renders the committing door's own finding at its exit code. `HEAD` was unmoved in every refusing cell. It was found independently as `(5, DEFECT 1 · rc.20)`, and its `jigc start` half was inside the usability batch's own code.

## Repro

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

## Resolution

Fixed in M53's last batch (`6281f768`, and `2f62db15` from its review), stamped `1.0.0-rc.21`. The preview takes the seam's subject for an ordinary task in a fan-out worktree. A sub-task keeps the exemption, because its committing door is the milestone boundary. The route became `git switch -c <new-branch>`, driven to land. The suggested *finish it from the main checkout* was refused, because an ordinary task minted in a worktree stages in that worktree's index, so that route would lose the work.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). In `.jigc/worktrees/area-one`, an ordinary `single-task` task has its commit doc filled and `k.txt` staged. `task validate`, `task finalize --dry-run` and `task finalize` each exit 1 with `repo.head-detached`, routed at `git switch -c <new-branch>`. That span, lifted from the emitted route with `<new-branch>` filled as `rescue`, exits 0. The next `task finalize` exits 0 and lands on `rescue`.
