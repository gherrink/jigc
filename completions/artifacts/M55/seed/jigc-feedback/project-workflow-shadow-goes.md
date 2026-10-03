---
kind: bug
found-in: milestone:M55-planning/F16
about: jigc validate
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A project workflow shadow goes stale with nothing to say so

## Description

A project-layer workflow shadow, `.jigc/config/workflows/<id>.yaml`, is a whole-file copy with no recorded base, and no CLI verb writes it. When the pack's workflow moves on underneath it, the shadow goes stale silently. That breaks the invariant that every customization is a recorded delta against a known base version, never an untracked fork. Steps have `jigc config fork`, which records `base-version` and `base-hash` for the upgrade reconciliation to read. Workflows have no counterpart. The baseline router auditor found it. It bears on the project-pack idea the Settle parked (S3).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. On a `--pack-from-dev` rig, a committed project shadow of `single-task` composes as `workflow:single-task (project · dev/fs-local)` in the `--explain` header. After the pack's own `single-task.yaml` gains a step, the two files differ, `jigc validate` exits 0 with *no findings*, and `.jigc/config/` holds no base for the shadow. `jigc config fork` takes only a `workflow:<id>#<step-id>` unit.

## Repro

```sh
rig=$(dev/jigc-rig fresh --pack-from-dev) || exit; eval "$rig"     # exports $JIGC_PACK_DIR
mkdir -p .jigc/config/workflows                                    # a project shadow: a whole-file copy
cp "$JIGC_PACK_DIR/workflows/single-task.yaml" .jigc/config/workflows/single-task.yaml
git add .jigc/config/workflows && git commit -qm "shadow single-task"
printf '{{ include: step:locate }}\n' >> "$JIGC_PACK_DIR/workflows/single-task.yaml"   # the pack moves on
cmp -s "$JIGC_PACK_DIR/workflows/single-task.yaml" .jigc/config/workflows/single-task.yaml; echo "cmp: exit $?"   # 1
$JIGC validate; echo "validate: exit $?"                           # 0, no findings
$JIGC start --workflow single-task "probe" --explain | grep 'workflow:single-task'   # (project · …): the shadow wins
ls .jigc/config                                                    # no recorded base for it
```

## Resolution
