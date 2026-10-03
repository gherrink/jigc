---
kind: bug
found-in: review:M52-per-axis/(4,DEFECT 1)
about: finalize.stage-failed
jigc-version: 1.0.0-rc.16
status: resolved
tier: tier-2
pinned-by: migrate_rollback::stage_phase_git_failure_yields_a_routed_finding_and_rolls_back
date: 2026-10-03
schema-version: 1
---

# Stage-failed route at task finalize is not copy-runnable

## Description

`finalize.stage-failed`'s route named `jigc task finalize` with no `<ID>`. Pasted as printed, it exited 2 with clap's *the following required arguments were not provided: <ID>*. Yet `stage_failed_finding(task_id, git_error)` takes the task id and spends it on the locus. The same door's hook-rejection frame prints `re-run jigc task finalize <id>`. The finding's `gate_coverage` row is `FinalizeOnly`, so the route reaches a reader at the finalize door only.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons --start quick-fix "axis four probe") || exit; eval "$rig"
export PATH="$(dirname "$JIGC"):$PATH"
jigc doc set-field commit:axis-four-probe#header/type --value fix
printf 'four\n' | jigc doc set-slot commit:axis-four-probe#summary --from-file -
echo w > work.txt; git add work.txt
: > "$(git rev-parse --git-path index.lock)"
jigc task finalize axis-four-probe            # exit 3, finalize.stage-failed
#   route: `jigc task finalize axis-four-probe` once the embedded git failure is resolved
#     (e.g. remove a stale `.git/index.lock`) — the task survives intact, so the same
#     re-run lands the commit
rm -f "$(git rev-parse --git-path index.lock)"
jigc task finalize axis-four-probe            # the route, verbatim: exit 0, the commit lands
```

## Resolution

Fixed in M53's pre-v1 usability batch (DECISIONS.md → *2026-09-23 — the pre-v1 usability batch: six surface rows*). The route is a `Route::mechanical`, whose debug fence parses the argv against the real CLI. It names the task, and it echoes the run's own `--approve` and `--carry-staged`. Without that echo, a migration finalize's re-run parses and then stops at exit 4 at the review gate the blocked run had already passed.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`). A stale `index.lock` makes `jigc task finalize axis-four-probe` exit 3 with `finalize.stage-failed`, and its route reads `` `jigc task finalize axis-four-probe` once the embedded git failure is resolved ``. After the lock is removed, the backticked span, run verbatim, exits 0 and lands the commit. The pinning test lifts the span out of the emitted `route:` line, asserts its argv is `jigc task finalize <id> --approve`, and runs it to a landed commit.
