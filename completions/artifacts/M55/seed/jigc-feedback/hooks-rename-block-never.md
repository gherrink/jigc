---
kind: bug
found-in: review:M53-per-axis-rc19/(2,N-2)
about: jigc setup
jigc-version: 1.0.0-rc.19
status: resolved
tier: tier-3
pinned-by: precommit_hook_acceptance::commit_blocks_on_this_commit_oob_rename_of_a_placement_doc
date: 2026-10-03
schema-version: 1
---

# The hook's rename block never fires for a placement doctype

## Description

The installed pre-commit hook's blocking rename backstop could not fire for any placement doctype. A `git mv` of `VISION.md`, `CHANGELOG.md`, `docs/roadmap.md` or `docs/decisions-log.md` committed at exit 0 with *(not staged in this commit; commit not blocked)*, while the same act on a location doctype (`milestone-record`) was blocked at exit 1. `reconciliation.rename` emitted the `mv` revert route the hook pairs on only for a location doctype. A renamed placement doc was reported *missing* and routed at `jigc unmanage`. Every managed singleton a stock corpus ships is a placement doctype. Nothing was destroyed, and `jigc validate` raised the rename and exited non-zero. The covering sentence was inside the cwd arc's own new code; the inertness was older.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git mv VISION.md VISION-OOB.md                        # a placement singleton, renamed out of band
git commit -qm "probe VISION.md"; echo "exit $?"      # 1: the hook blocks the staged rename
git status --porcelain                                # R  VISION.md -> VISION-OOB.md, still staged
```

## Resolution

Fixed in M53's pre-v1 usability batch (`be40738e`), stamped `1.0.0-rc.20`. It was a census bug, not a hook filter: a placement singleton's rename candidates were enumerated at its one declared path, the path a rename empties. The census now widens only when the declared home is absent, gated by an exact content-hash match. The rc.20 re-review recorded it *CLOSED*, with all four placement singletons blocked at exit 1.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`). `git mv VISION.md VISION-OOB.md` and then `git commit` exits 1 with *out-of-band managed-doc rename staged in this commit — a bare `git mv` bypasses jigc identity tracking; use `jigc rename` instead (commit blocked)*, and the rename stays staged.
