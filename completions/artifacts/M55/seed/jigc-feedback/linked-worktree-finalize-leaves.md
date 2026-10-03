---
kind: bug
found-in: review:M53-per-axis-rc19/(2,N-4)
about: file-state.hash-matches
jigc-version: 1.0.0-rc.19
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# A linked-worktree finalize leaves main a false file-state diagnosis

## Description

A promoting `jigc task finalize` run from a branch-attached linked worktree lands its doc on that branch, and writes the new baseline into the shared `.jigc/state/`. The main checkout's next `jigc validate` then reported *blocking (gates at finalize) · file-state.hash-matches — on-disk content of `docs/decisions-log.md` differs from the recorded state*, routed at *review the out-of-band edit … and re-author it through the owning workflow*. There was no out-of-band edit: jigc wrote that baseline from another checkout, at exit 0. The rc.19 driver filed it tier 2, and its reconciler corrected it to tier 3, because main's next `task finalize` absorbs the drift (`reconciliation.absorb`) and lands. So the residual is the diagnosis, not a dead end. The rc.20 re-review kept it open, exactly as corrected.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open, with a different false diagnosis**. M55 Increment 4 (`2c284c48`) grades a drift whose on-disk bytes equal `HEAD`'s blob as advisory. So main's `jigc validate` now exits 0 with *advisory · file-state.hash-matches*, routed *the baseline lags `HEAD`; absorbed at the next finalize*. It no longer names an out-of-band edit, but its new reason is false of this state too. The baseline holds the bytes of the `lwbr` commit, and main's `HEAD` is an ancestor of `lwbr`, so the baseline is ahead of `HEAD`, not behind it.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
L=$(mktemp -d)/lw; git worktree add -q -b lwbr "$L"; cd "$L"
$JIGC start "record the linked decision" --workflow decided-task > /dev/null
$JIGC doc add-item decisions-log#entries --title "Linked branch decision"
printf 'Why.\n' | $JIGC doc set-slot decisions-log#entries/linked-branch-decision/why --from-file -
$JIGC doc set-field commit:record-the-linked-decision#header/type --value docs
printf 'record it\n' | $JIGC doc set-slot commit:record-the-linked-decision#summary --from-file -
$JIGC task finalize record-the-linked-decision; echo "exit $?"   # 0, lands on lwbr, not main
cd "$REPO"
git merge-base --is-ancestor HEAD lwbr && echo "main's HEAD is an ancestor of lwbr"
$JIGC validate; echo "exit $?"
#   0; advisory · file-state.hash-matches — on-disk content of `docs/decisions-log.md` differs
#   from the recorded state;  route: the baseline lags `HEAD`; absorbed at the next finalize
#   (the baseline holds lwbr's bytes, a commit AHEAD of this HEAD: it does not lag it)
```

## Resolution
