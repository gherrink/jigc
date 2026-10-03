---
kind: inconvenience
found-in: review:M52-per-axis/(6,D-4)
about: jigc milestone execute
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Empty milestone execute walk never states it is empty

## Description

A legitimately empty `jigc milestone execute` walk does not state its empty case. Over a milestone with no sub-tasks, the composed walk says *Spawn a sub-agent per sub-task (one per `Spawn:` line below)*. It carries zero `Spawn:` lines, and nothing says so. The sibling empty enumeration, `implement-from-spec`'s spec list, got exactly that clause at M52, and this one did not. The terminal is safe, which bounds the severity: the walk's last verb, `milestone finalize`, refuses with a code and a two-armed route. The M53 re-reviews kept it open through `1.0.0-rc.19`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `jigc milestone execute empty-probe` exits 0 with zero `Spawn:` lines and an empty stderr, and the composed text names no empty case. `jigc milestone finalize empty-probe` then exits 3 with `milestone.zero-contribution` and a route naming `provision` or `discard`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC milestone create "Empty probe"
$JIGC milestone execute empty-probe > "$RIG/o" 2> "$RIG/e"    # exit 0
grep -c '^Spawn:' "$RIG/o"                                    # 0
wc -c < "$RIG/e"                                              # 0
#   "Spawn a sub-agent per sub-task (one per `Spawn:` line below) …" — and no line below
$JIGC milestone finalize empty-probe                          # exit 3, milestone.zero-contribution
```

## Resolution
