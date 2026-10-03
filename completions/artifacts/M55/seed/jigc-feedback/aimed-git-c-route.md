---
kind: bug
found-in: review:M53-per-axis-rc18/(2,F-1)
about: repo.operation-in-progress
jigc-version: 1.0.0-rc.18
status: resolved
tier: tier-2
pinned-by: git_span_aim::the_fan_out_posture_site_keys_repo_relative_and_routes_absolute
date: 2026-10-03
schema-version: 1
---

# An aimed git -C route does not run from the checkout that printed it

## Description

The post-review fix `3c71da87` routed a fan-out worktree's un-concluded git operation at `git -C <worktree> …`, through `crate::repo::aim_at`, and spelt the worktree repo-relative. From any cwd but the repository root, the emitted span failed: from `docs/deep`, `git -C .jigc/worktrees/cw-area bisect reset` exited 128 with *cannot change to '.jigc/worktrees/cw-area'*. Both callers were affected: `BreachSite::aim`, at `milestone finalize` and `task validate <sub>`, and `milestone::held_here`, at `milestone discard`, `uninstall` and `milestone provision`. The fix's own rationale in `design/finalize.md` reads *a route the caller cannot run from where they are standing is not a route*. Every door refused, so nothing was lost. It was inside M53's own new code.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC milestone create "Cwd wave" > /dev/null
$JIGC milestone add-task cwd-wave "cw area" > /dev/null
$JIGC milestone provision cwd-wave
W=$REPO/.jigc/worktrees/cw-area
git -C "$W" bisect start; git -C "$W" bisect bad
mkdir -p "$REPO/docs/deep"; cd "$REPO/docs/deep"          # an ordinary subdirectory
$JIGC milestone finalize cwd-wave; echo "exit $?"         # 3, repo.operation-in-progress
#   route: … abandon it with `git -C <absolute repo>/.jigc/worktrees/cw-area bisect reset` …
span=$($JIGC milestone finalize cwd-wave 2>&1 | grep -o 'git -C [^`]*bisect reset' | head -1)
eval "$span"; echo "exit $?"                              # the emitted span, run here: 0
```

## Resolution

Fixed in M53's cwd-dependence arc (`65de53f5`, *every operator-facing `git` span names the checkout it runs in*), stamped `1.0.0-rc.19`. `engine::finding::git_at` is the one home of the runnable spelling, `git -C <absolute checkout> …`, and the absolute rides the route only: the message, the `at:` locus and the `(code, target)` key stay repo-relative. The rc.19 re-review recorded it *CLOSED*, with the span run verbatim from five cwds including `/` and under a repository root holding a space. The rc.20 re-review recorded it *still CLOSED*.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). From `docs/deep`, `jigc milestone finalize cwd-wave` exits 3 with `repo.operation-in-progress` at `.jigc/worktrees/cw-area`, routed at `git -C <absolute repo>/.jigc/worktrees/cw-area bisect reset`. That span, lifted from the emitted route and run from the same cwd, exits 0 with *HEAD is now at …*.
