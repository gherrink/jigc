---
kind: bug
found-in: review:M52-per-axis/(3,A3-3)
about: test:milestone_boundary_displacement
jigc-version: 1.0.0-rc.16
status: resolved
tier: tier-3
pinned-by: milestone_boundary_displacement::a_landed_milestone_boundary_keeps_every_byte_of_its_own_area_it_did_not_write
date: 2026-10-03
schema-version: 1
---

# Boundary displacement suite never tests the milestone's own area

## Description

`milestone_boundary_displacement`'s subject was the sub-task areas only. Every plant in that suite was under `.jigc/tasks/`, and `grep -c milestones` over the file returned 0. So the boundary's `displaced: []` assertion was true of a tree it did not look at. The boundary's own area, `.jigc/milestones/<id>/`, was outside the subject. That is why the tier-1 row `(3, A3-1)`, *milestone finalize destroys its own area's foreign bytes*, was green under the suite that exists to fence this door's displacement. The class is *working areas this door removes*, and its axis has two members. The move-failure cell had no standing test at either `Displace` door.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
export PATH="$(dirname "$JIGC"):$PATH"
jigc milestone create "Area probe"
jigc milestone add-task area-probe one --workflow report-jigc-feedback
jigc milestone provision area-probe; jigc milestone execute area-probe
( cd .jigc/worktrees/one &&
  jigc doc create jigc-feedback --title "Probe row" --task one &&
  jigc doc set-field jigc-feedback:probe-row#meta/kind --value bug --task one &&
  jigc doc set-field jigc-feedback:probe-row#meta/found-in --value trial:probe --task one &&
  jigc doc set-field jigc-feedback:probe-row#meta/jigc-version --value 1.0.0-rc.22 --task one &&
  printf 'Seen.\n' | jigc doc set-slot jigc-feedback:probe-row#description --from-file - --task one )
printf 'mine\n' > .jigc/milestones/area-probe/notes.txt     # a byte jigc did not write
jigc milestone finalize area-probe            # exit 0
#   note: the working area held 1 entry jigc did not write … they were moved aside, not taken:
#     .jigc/milestones/area-probe/notes.txt → .jigc/displaced/area-probe/notes.txt
cat .jigc/displaced/area-probe/notes.txt      # mine
```

## Resolution

Fixed in M53 with the tier-1 row `(3, A3-1)`. The milestone area joined the displacement subject as a registry row beside the task-area and sub-task rows. Its complement is displaced to `.jigc/displaced/<milestone-id>/`, and the suite gained the cells A3-3 said were missing. Those are the boundary's own area (the pinning test) and the move-failure axis (`boundary_areas_*`). Every M53 re-review from `1.0.0-rc.17` through `1.0.0-rc.20` recorded it *CLOSED (behaviourally)*. The suite now names `milestones` 8 times.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`). A foreign `notes.txt` was planted in `.jigc/milestones/area-probe/` of a one-sub-task milestone. `jigc milestone finalize area-probe` exits 0, names the move `.jigc/milestones/area-probe/notes.txt → .jigc/displaced/area-probe/notes.txt`, and the parked file holds the planted bytes.
