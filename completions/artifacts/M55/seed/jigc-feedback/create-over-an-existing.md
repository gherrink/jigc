---
kind: bug
found-in: milestone:M55-planning/F2
about: jigc doc create
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: create_only_gate::a_reused_title_is_refused_by_both_doors_with_nothing_staged
date: 2026-10-03
schema-version: 1
---

# A create over an existing slug silently overwrites the committed doc

## Description

`jigc doc create <ty> --title X`, where a committed doc already carries X's slug, acked `(already existed — copied in for update)` at exit 0, and the next `set-slot` overwrote the earlier doc. `jigc validate` stayed clean. The M55 baseline drove it in a fan-out, and the decisions gap-detector drove it sequentially, over two `park-idea` tasks. For a finding store, where every report is a new doc, that is a silent loss of the earlier report.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
fill_commit() {
  $JIGC doc set-field commit:$1#type --value docs --task $1 > /dev/null
  $JIGC doc set-field commit:$1#scope --value probe --task $1 > /dev/null
  printf '%s\n' "$1 lands" | $JIGC doc set-slot commit:$1#summary --from-file - --task $1 > /dev/null
  printf 'why\n' | $JIGC doc set-slot commit:$1#body --from-file - --task $1 > /dev/null
}
T=$($JIGC start --workflow report-jigc-feedback "first report" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "Probe finding" --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/kind --value bug --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/found-in --value task:probe --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/jigc-version --value 1.0.0-rc.22 --task $T > /dev/null
printf 'The first finding.\n' | $JIGC doc set-slot jigc-feedback:probe-finding#description --from-file - --task $T > /dev/null
fill_commit $T; $JIGC task finalize $T > /dev/null 2>&1; echo "first lands: exit $?"     # 0
U=$($JIGC start --workflow report-jigc-feedback "second report" | sed -n 's/^task minted: //p')
$JIGC doc create jigc-feedback --title "Probe finding" --task $U; echo "exit $?"        # 1, create.already-exists
$JIGC doc create jigc-feedback --title "Probe: finding" --task $U; echo "exit $?"       # 1, the same refusal
git status --short docs                                                                 # nothing staged
```

## Resolution

Fixed in M55 Increment 2 (`e9f7f081`, *an allows-create entry carrying `new: true` refuses an id already on disk*). A create-gate entry may carry `new: true`, and both report workflows' entries do. Under it, a create whose minted id is already on disk at the doctype's home is refused `create.already-exists` before anything is copied in, at `jigc doc create` and `jigc doc author` alike, routed at a distinct `--title` or a `--slug`. An entry without the key keeps the create-or-update default, on which planning's idempotent singleton create depends (`design/findings-channel.md` → 4).

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). With `jigc-feedback:probe-finding` committed, a second `report-jigc-feedback` task's `jigc doc create jigc-feedback --title "Probe finding"` exits 1 with `create.already-exists`, routed *choose a distinct `--title`, or keep this one and pass `--slug <slug>`*. A different title that slugs onto the same id, `Probe: finding`, is refused the same way, and nothing is staged.
