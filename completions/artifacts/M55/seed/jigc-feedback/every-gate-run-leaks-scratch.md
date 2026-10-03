---
kind: bug
found-in: milestone:M55-planning/T3
about: dev:gate
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: UNPINNED: a measurement over one full gate run; no test counts what a run leaves in $TMPDIR, and support::scratch::ScratchDir's drop is what removes it
date: 2026-10-03
schema-version: 1
---

# Every gate run leaks scratch roots into TMPDIR

## Description

Every gate run leaked about 249 entries into `$TMPDIR`, `jigc-trial-rig-fence*` the most of them: 54k of about 111k entries, about 38 GB accumulated. The gate-speed measurement found it.

## Repro

```sh
# from the jigc checkout: one full gate in a fresh TMPDIR, then what it left
T=$(mktemp -d "${TMPDIR:-/tmp}/gate-tmpdir.XXXXXX")
TMPDIR="$T/" dev/gate; echo "gate: exit $?"           # GATE: PASS
ls -A "$T" | wc -l                                    # 10, five of them the gate's own logs
```

## Resolution

Fixed by the gate-speed PR (#8, `8ddb260d`, *suites remove the scratch roots they mint, so a gate run leaves no TMPDIR litter*). `support::scratch::ScratchDir` removes its root on drop, `dev_gate_report` runs every gate it drives with `$TMPDIR` inside one, and the probe's `temp_root` returns a dropping guard. The commit names five entries a run that it leaves by design or did not cover.

Re-driven on this build (the checkout at `982f910f`). One full `dev/gate` in a fresh `$TMPDIR` passed (4332 passed, 0 failed) and left 10 entries: the gate's own five logs (`jigc-gate`, `jigc-gate-steps`, `jigc-gate-hygiene`, `jigc-gate-gitleaks`, `jigc-hygiene`) and the five the fix names (`jigc-migrate-source-absolute`, `jigc-migrate-source-symlink`, `jigc-retire-sink-absolute`, `jigc-trial-pre-dispatch-empty-pack`, `jigc-trial-fresh-copy`). No `jigc-trial-rig-fence*` entry remained.
