---
kind: bug
found-in: milestone:M55-planning/F8
about: jigc validate
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A union-resolved singleton loses a field and validates clean

## Description

Two branches each appended an item to one singleton, and the conflict was resolved with a naive union. One item's `date` was lost, because its trailing fields block went to one side, and `jigc validate` exited 0. `design/storage.md` is silent on managed docs merged by git across branches. It is the baseline ledger's **S3**, and one reason the findings channel took one doc per finding rather than a singleton ledger (`design/findings-channel.md` → 1). The baseline concurrency auditor drove it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. Two `single-task` tasks each append one `decisions-log` entry, `Alpha choice` on a side branch and `Beta choice` on `main`, and each lands. `git merge` conflicts, and a `git merge-file --union` resolve leaves `Beta choice` with no `<!-- fields -->` block: the two identical `- date:` lines collapse into one, under `Alpha choice`. The merge commits, and `jigc validate` exits 0 with one advisory `file-state.hash-matches`, routed *the baseline lags `HEAD`; absorbed at the next finalize*. A missing `set: on-create` date draws no conformance finding (`m55-f9`), so nothing reports the loss.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
append_entry() {   # append_entry <title> <id>: one task that appends a decisions-log entry and lands it
  local t; t=$($JIGC start --workflow single-task "log $1" | sed -n 's/^task minted: //p')
  $JIGC doc add-item decisions-log:decisions-log#entries --title "$1" --task $t > /dev/null
  printf 'Because %s.\n' "$1" | $JIGC doc set-slot "decisions-log:decisions-log#entries/$2/why" --from-file - --task $t > /dev/null
  $JIGC doc set-field commit:$t#type --value docs --task $t > /dev/null
  $JIGC doc set-field commit:$t#scope --value log --task $t > /dev/null
  printf 'log %s\n' "$1" | $JIGC doc set-slot commit:$t#summary --from-file - --task $t > /dev/null
  $JIGC task finalize $t > /dev/null 2>&1; echo "append $2: exit $?"
}
git switch -q -c side; append_entry "Alpha choice" alpha-choice      # 0
git switch -q main;    append_entry "Beta choice" beta-choice        # 0
git merge -q side > /dev/null 2>&1; echo "merge: exit $?"            # 1: both appended at the end
git show :2:docs/decisions-log.md > "$RIG/ours"; git show :1:docs/decisions-log.md > "$RIG/base"
git show :3:docs/decisions-log.md > "$RIG/theirs"
git merge-file --union "$RIG/ours" "$RIG/base" "$RIG/theirs"; cp "$RIG/ours" docs/decisions-log.md   # the naive union
git add docs/decisions-log.md; git commit -qm "merge side"
grep -c '^- date:' docs/decisions-log.md            # 1: two entries, one date
$JIGC validate; echo "validate: exit $?"            # 0, one advisory file-state.hash-matches
```

## Resolution
