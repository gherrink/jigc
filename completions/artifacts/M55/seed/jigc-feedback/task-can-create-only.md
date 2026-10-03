---
kind: inconvenience
found-in: milestone:M55-planning/F19
about: write.identity-change
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A task can create only one instance of each doctype

## Description

A task creates at most one instance of a doctype: a second create with another title is refused `write.identity-change`. So seeding N per-doc findings needs N tasks. The baseline found it, and the Settle accepted it as a cost of one doc per finding (S1, `design/findings-channel.md` → 1, *Costs carried*). The seed itself pays it, as one fan-out sub-task per row.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**, as the accepted cost. In one `report-jigc-feedback` task, `jigc doc create jigc-feedback --title "First finding"` exits 0 and a second create, `--title "Second finding"`, exits 1 with `write.identity-change`, routed at renaming the first doc or at *a genuinely separate second document is its own task*.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow report-jigc-feedback "two findings" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "First finding" --task $T; echo "exit $?"    # 0
$JIGC doc create jigc-feedback --title "Second finding" --task $T; echo "exit $?"   # 1, write.identity-change
```

## Resolution
