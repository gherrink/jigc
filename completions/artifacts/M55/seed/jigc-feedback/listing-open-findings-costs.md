---
kind: inconvenience
found-in: milestone:M55-planning/F18
about: jigc doc list
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: doc_list_triage::one_listing_groups_the_open_findings_by_found_in
date: 2026-10-03
schema-version: 1
---

# Listing open findings costs a show per doc and a title parse

## Description

`jigc doc show --format json` on a per-instance doc had no title key, and `jigc doc list` carried no field values. So *all open findings* cost a `doc list`, then one `doc show` per doc, then parsing the markdown for each title. The doctypes gap-detector found it, and the Settle took it as the read surface for the new doctypes (S10).

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
$JIGC doc show jigc-feedback:probe-finding --format json > "$RIG/show.json"
python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["title"])' "$RIG/show.json"   # Probe finding
$JIGC doc list jigc-feedback --format json > "$RIG/list.json"
python3 -c 'import json,sys; r=json.load(open(sys.argv[1]))["docs"][0]; print(r["title"], r["fields"]["status"], r["fields"]["found-in"])' "$RIG/list.json"
#   Probe finding open task:probe        <- one listing, no show per doc
```

## Resolution

Fixed in M55 Increment 6: `489353a3` (*doc show --format json carries the doc's H1 as a top-level title*) and `7976d7fa` (*every doc list --format json row carries title and fields*). One `jigc doc list <ty> --format json` now answers *all open findings* grouped by `found-in`, with no per-doc read (`design/findings-channel.md` → 5).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). For a committed `jigc-feedback` doc, `jigc doc show --format json` carries `"title": "Probe finding"`, and the `jigc doc list jigc-feedback --format json` row carries the title and the fields, `status` `open` and `found-in` `task:probe` among them.
