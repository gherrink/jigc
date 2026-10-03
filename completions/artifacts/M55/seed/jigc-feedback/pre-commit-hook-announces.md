---
kind: bug
found-in: review:M53-per-axis-rc19/(2,N-1)
about: jigc setup
jigc-version: 1.0.0-rc.19
status: resolved
tier: tier-3
pinned-by: precommit_hook_acceptance::commit_deleting_a_managed_placement_doc_announces_no_rename
date: 2026-10-03
schema-version: 1
---

# The pre-commit hook announces a rename on a commit that has none

## Description

After a committed `git rm VISION.md`, the installed pre-commit hook printed *an out-of-band managed-doc rename exists in the committed tree … (not staged in this commit; commit not blocked)*, on that commit and on every commit after it. There was no rename. A staged `git mv` of a placement doc was also told it was *not staged in this commit*. The cause was the cwd arc's own widening. `65de53f5` gave every operator-facing git span a `git -C` prefix, so the hook's extraction grep was widened from `git mv` to `git -C`. That matched every routed span (`home-vacated`'s `show`, `owner-artifact`'s `add`), and the warn branch printed whenever the grep matched anything. No commit was wrongly blocked, and `jigc validate` caught the real condition. It was found independently as `(6, A6-R1)`, and it was inside the cwd arc's own new code.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -qm "probe"; echo "exit $?"   # 0, silent
git rm -q VISION.md                                   # a deletion; no rename anywhere
git commit -qm "probe: delete a managed singleton"; echo "exit $?"                   # 0, silent
#   rc.19 printed "an out-of-band managed-doc rename exists … (not staged in this commit …)"
printf 'b\n' > f2.txt; git add f2.txt; git commit -qm "unrelated 2"; echo "exit $?"     # 0, silent
```

## Resolution

Fixed in M53's pre-v1 usability batch (`be40738e`), stamped `1.0.0-rc.20`. The hook's awk is now its single decision point, with a three-way verdict (block, warn, silent), so its two printed branches cannot disagree about what a move is. The rc.20 re-review recorded it *CLOSED*: a committed `git rm VISION.md` exits 0 and the hook prints nothing.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). In a `committed-singletons` rig, an ordinary commit, a commit of `git rm VISION.md`, and an unrelated commit after it all exit 0, and the hook prints nothing on any of them.
