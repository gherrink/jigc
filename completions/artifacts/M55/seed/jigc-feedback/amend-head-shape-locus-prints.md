---
kind: bug
found-in: review:M53-per-axis-rc20/(5,DEFECT 2 · rc.20)
about: amend.head-shape
jigc-version: 1.0.0-rc.20
status: resolved
tier: tier-3
pinned-by: task_amend::the_head_shape_refusal_names_the_id_the_mint_would_have_taken
date: 2026-10-03
schema-version: 1
---

# Amend head-shape locus prints an undeclared work-unit address

## Description

`amend.head-shape`, a code minted in F-10, printed its locus as `at: work-unit:<id>` when `HEAD` was unborn or a root commit. No grammar in the product declares that spelling. `design/structural-grammar.md` and `design/command-output-contract.md` declare `task:<id>` and `milestone:<id>`, and `jigc doc show work-unit:root-probe` refuses with `store.unknown-type`. The finding projects no key, so only the printed locus was affected. The code's comment gave a reason: the id is one the mint would have taken, so `task:<id>` names a task that does not exist. Nothing was minted.

## Repro

```sh
rig=$(dev/jigc-rig --git-state unborn) || exit; eval "$rig"
$JIGC setup > /dev/null 2>&1                    # births HEAD: a root commit, which amend refuses
$JIGC task amend "root probe"; echo "exit $?"   # 1, amend.head-shape, at: task:root-probe
$JIGC task amend; echo "exit $?"                # 1, amend.head-shape, at: task:amend-<sha7>
ls .jigc/tasks                                  # No such file or directory: nothing minted
```

## Resolution

Fixed in M53's last batch (`8a40a5e2`), stamped `1.0.0-rc.21`. The locus takes the declared `task:<id>` spelling, naming the id the mint would have taken.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). In an unborn repository, after `jigc setup` births a root commit, `jigc task amend "root probe"` exits 1 with `amend.head-shape` at `task:root-probe`, and `jigc task amend` exits 1 with it at `task:amend-<sha7>`. `.jigc/tasks` does not exist.
