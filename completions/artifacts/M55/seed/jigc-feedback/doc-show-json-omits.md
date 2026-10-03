---
kind: bug
found-in: milestone:M55-planning/F10
about: jigc doc show
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: doc_show::an_absent_defaulted_field_projects_its_default_on_the_whole_doc_serve
date: 2026-10-03
schema-version: 1
---

# Doc show JSON omits a defaulted field deleted by hand

## Description

A defaulted field deleted by hand was omitted from `jigc doc show --format json`, so a client filtering on it, for example `status == "open"`, silently missed the row. The capabilities gap-detector found it. For the findings channel, whose `status` defaults to `open`, that is a finding dropped from every open-findings query.

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
$JIGC task finalize $T > /dev/null 2>&1; echo "lands: exit $?"            # 0
F=docs/jigc-feedback/probe-finding.md
grep -v '^status: ' $F > $F.new && mv $F.new $F; git commit -qam "hand edit: drop status"
$JIGC doc show jigc-feedback:probe-finding --format json > "$RIG/show.json"; echo "doc show: exit $?"
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["fields"].get("status"))' "$RIG/show.json"   # open
```

## Resolution

Fixed in M55 Increment 6 (`03ecc2ba`, *doc show's whole-doc fields report an absent defaulted field's default*). The whole-doc serve reports an absent defaulted field at its default, and M55 Increment 6's `doc list --format json` rows, which carry fields (`7976d7fa`), do the same.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). With `status:` deleted by hand from a committed `jigc-feedback` doc, `jigc doc show jigc-feedback:probe-finding --format json` exits 0 and its `fields.status` is `open`, and the `jigc doc list jigc-feedback --format json` row also reads `open`.
