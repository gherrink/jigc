---
kind: bug
found-in: review:M52-per-axis/(1,A1-N2)
about: config.repoint-failed
jigc-version: 1.0.0-rc.16
status: open
tier: tier-2
date: 2026-10-03
schema-version: 1
---

# A dash root-knob value breaks the door's own git mv

## Description

A `ROOT_KNOBS` value the door accepts makes the door's own `git mv` unparseable, and the refusal's route says to re-run the same command. `jigc config set placement-root -` builds `git mv docs/decisions-log.md -/decisions-log.md` with no `--` separator, so git reads `-/` as a switch. The transaction is sound: the knob is unchanged and the docs are back. But the route asks the operator to fix what the message names and re-run, and the value is itself the fault, so the re-run fails again. Whether the knob is admitted also depends on the corpus's shape rather than on a value rule. The same `config set docs-root -` lands at exit 0 on a corpus with no `location:` instance to move.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. There is one `config.repoint-failed` per placement doc the re-point would move (`docs/decisions-log.md` and `docs/roadmap.md`). Each embeds `error: unknown switch '/'`, and each routes at `re-run jigc config set placement-root -`. The exit is 1, and `config get placement-root` still reads the pack default.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC config set placement-root -             # exit 1
#   blocking · config.repoint-failed — `placement-root` was not set to `-`:
#     `git mv docs/decisions-log.md -/decisions-log.md` failed: error: unknown switch `/'
#   route: `placement-root` is unchanged … Fix what this message names, then re-run
#     `jigc config set placement-root -`
#   (the same again for docs/roadmap.md)
$JIGC config get placement-root               # unchanged: the pack default
```

## Resolution
