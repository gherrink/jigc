---
kind: bug
found-in: milestone:M55-planning/F3
about: write.title-ignored
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: create_only_gate::without_new_a_different_title_onto_a_committed_id_routes_at_a_distinct_identity
date: 2026-10-03
schema-version: 1
---

# A slug collision is routed at renaming the existing doc

## Description

A different title that slugs to an existing committed doc's id was refused `write.title-ignored`, which is right, but the route told the reporter to `jigc doc rename` the **existing** doc: someone else's work. The route the reporter needed was a distinct title, or a `--slug` beside the existing doc. The M55 doctypes gap-detector found it.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow park-idea "first park" | sed -n 's/^task minted: //p')    # no `new: true` on its entry
$JIGC doc create idea --title "Probe idea" --task $T > /dev/null
$JIGC doc set-field idea:probe-idea#trigger --value never --task $T > /dev/null
printf 'The first idea.\n' | $JIGC doc set-slot idea:probe-idea#description --from-file - --task $T > /dev/null
$JIGC doc set-field commit:$T#type --value docs --task $T > /dev/null
$JIGC doc set-field commit:$T#scope --value ideas --task $T > /dev/null
printf 'park an idea\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T > /dev/null
printf 'why\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T > /dev/null
$JIGC task finalize $T > /dev/null 2>&1; echo "idea lands: exit $?"                  # 0
U=$($JIGC start --workflow park-idea "second park" | sed -n 's/^task minted: //p')
$JIGC doc create idea --title "Probe: idea" --task $U; echo "exit $?"                # 1, write.title-ignored
#   route: choose a distinct `--title`, or keep this one and pass `--slug <slug>` …   <- not a rename of the committed doc
```

## Resolution

Fixed in M55 Increment 2 (`415efc9a`, *`write.title-ignored` over a committed doc routes at a distinct identity*, and `fc307600`, *the distinct-identity route follows where the id comes from*). Over a committed doc the refusal now routes at a distinct `--title` or a `--slug` beside the existing doc, and over a doc the task itself stages it still routes at the in-task `jigc doc rename`. Inside a report task the case cannot arise, because the create-gate refuses `create.already-exists` first (`design/findings-channel.md` → 4).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). With `idea:probe-idea` committed, a second `park-idea` task's `jigc doc create idea --title "Probe: idea"` exits 1 with `write.title-ignored`, routed *choose a distinct `--title`, or keep this one and pass `--slug <slug>` to mint beside the existing doc*, with the runnable create beside it. It no longer names `jigc doc rename`.
