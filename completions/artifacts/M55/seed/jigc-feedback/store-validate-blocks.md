---
kind: bug
found-in: milestone:M55-planning/F5
about: jigc validate
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: l2_branch_switch::after_a_branch_switch_both_scopes_report_the_same_advisory_row
date: 2026-10-03
schema-version: 1
---

# Store validate blocks on a doc that lives on another branch

## Description

After a branch switch, store-scope `jigc validate` exited 1 with a blocking `reconciliation.rename … is missing` for a doc that exists only on the other branch, and routed `jigc unmanage`, which drops the doc from the index. Task scope downgraded the same case to advisory. The split was a deliberate M45 boundary (`file_state.rs:986-992`, pinned by `store_scope_stays_blocking_where_task_scope_is_advisory`), kept because only the task path had been measured. CLAUDE.md says store-scope validate exits 0 for content findings. It is the baseline ledger's **L2** (`design/findings-channel.md` → 6), and it hits the branch-per-milestone model directly.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git switch -q -c milestone/x/main
T=$($JIGC start --workflow single-task "record the cache decision" | sed -n 's/^task minted: //p')
$JIGC doc create adr --title "Single-node cache" --task $T > /dev/null
for s in context decision consequences; do
  printf 'The %s.\n' "$s" | $JIGC doc set-slot adr:single-node-cache#$s --from-file - --task $T > /dev/null; done
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value cache --task $T > /dev/null
printf 'record the cache decision\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "finalize on the branch: exit $?"    # 0
git switch -q main                                        # the ADR lives on the branch only
$JIGC validate; echo "validate: exit $?"                  # 0, advisory reconciliation.rename, routed at the branch switch
$JIGC validate 2>&1 | grep -c 'jigc unmanage'             # 0
```

## Resolution

Fixed in M55 Increment 5 (`6ad92379`, *a dangling baseline routes at the branch switch, never at unmanage*, and `edaa428c`, *store scope grades a history-less baseline as the task gate does*). The history predicate of M45's Decision 7 now runs through the read-only store twin, so a recorded path that is missing and has no history at `HEAD` is advisory at both scopes. The one producer's route names the branch switch and never offers `jigc unmanage`. A committed deletion with history still blocks (`design/findings-channel.md` → 6, L2).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). After an ADR is finalized on `milestone/x/main` and the checkout switches back to `main`, `jigc validate` exits 0. It reports one advisory `reconciliation.rename` at `docs/decisions/single-node-cache.md`, routed *nothing on this checkout needs to change — a branch switch left this baseline behind, … switch back to that branch to work on it again*, and names no `jigc unmanage`.
