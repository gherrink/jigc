---
kind: bug
found-in: review:M53-per-axis-rc20/(2,A2-1)
about: jigc task finalize
jigc-version: 1.0.0-rc.20
status: resolved
tier: tier-3
pinned-by: dry_run_findings_equal_set::the_forecast_names_every_gate_it_refuses_on_that_the_preview_does_not_report
date: 2026-10-03
schema-version: 1
---

# Dry-run help says its findings are task validate's set, and they are not

## Description

`jigc task finalize --help` said that `--dry-run` *refuses on three gates* and that *its `findings` are the set `jigc task validate <id>` reports*. Two of the four gates the forecast refused on were outside that set: an amend task's `finalize.base-mismatch` after `HEAD` moved, and an ordinary task's `finalize.empty-commit` with nothing staged. In both cells `task validate` exited 0 clean while `--dry-run` exited 3. The forecast's silence on the base pin is a decision (the M47 Settle's Decision 1). What was false was the help text: its count and its parity sentence. Every refusal's route was runnable, and `HEAD` did not move. The class is the whole set `--dry-run` refuses on, and it predates F-10.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons --start quick-fix "empty commit probe") || exit; eval "$rig"
$JIGC doc set-field commit:empty-commit-probe#header/type --value fix
printf 'empty\n' | $JIGC doc set-slot commit:empty-commit-probe#summary --from-file -
$JIGC task validate empty-commit-probe; echo "exit $?"                 # 0, validates clean
$JIGC task finalize empty-commit-probe --dry-run; echo "exit $?"       # 3, finalize.empty-commit
$JIGC task finalize --help | grep -c 'the empty-commit guard (`finalize.empty-commit`'       # 1
$JIGC task finalize --help | grep -c 'The gates named above are \*\*outside\*\* that set'   # 1
```

## Resolution

Fixed in M53's last batch (`f57698fb`, and `40046113` from its review), stamped `1.0.0-rc.21`. The help names every gate decided before the transaction, and says which are outside `task validate`'s set by design. The batch's review found six out-of-set codes where the first fix named two, so the set is now derived from its refusal producers and count-fenced against their source text.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). In a `quick-fix` task with its commit doc filled and nothing staged, `task validate empty-commit-probe` exits 0 with *no findings — the task validates clean*, and `task finalize empty-commit-probe --dry-run` exits 3 with `finalize.empty-commit`. The help names *the empty-commit guard (`finalize.empty-commit`, …)* among the gates, and says *The gates named above are **outside** that set by design*.
