---
kind: bug
found-in: milestone:M55-planning/F9
about: jigc validate
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A required on-create date deleted by hand draws no finding

## Description

Deleting a `set: on-create` `date:` by hand and committing drew no conformance finding: `jigc validate` reported only `file-state.hash-matches`, while deleting `trigger:` from the same doc was reported. The decisions gap-detector found it. The Settle seeded it open as a declared bound.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`) on a `jigc-feedback` doc, whose `date` the schema marks required and `set: on-create`: **still open**. With `date:` deleted by hand and committed, `jigc validate` exits 0 and reports one advisory `file-state.hash-matches`, routed *the baseline lags `HEAD`; absorbed at the next finalize*. That is M55 Increment 4's store arm, which runs the conformance check before it absorbs and finds nothing to stop. The control, `found-in:` deleted the same way, is reported `schema-conformance.required-field-present`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow report-jigc-feedback "file it" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "Probe finding" --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/kind --value bug --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/found-in --value task:probe --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/jigc-version --value 1.0.0-rc.22 --task $T > /dev/null
printf 'Observed.\n' | $JIGC doc set-slot jigc-feedback:probe-finding#description --from-file - --task $T > /dev/null
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value findings --task $T > /dev/null
printf 'file it\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'why\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "lands: exit $?"                  # 0
F=docs/jigc-feedback/probe-finding.md
grep -v '^date: ' $F > $F.new && mv $F.new $F; git commit -qam "hand edit: drop date"
$JIGC validate; echo "validate: exit $?"         # 0, only an advisory file-state.hash-matches
grep -v '^found-in: ' $F > $F.new && mv $F.new $F; git commit -qam "hand edit: drop found-in"
$JIGC validate 2>&1 | grep -c 'required-field-present'          # 1: the control is reported
```

## Resolution
