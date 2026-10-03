---
kind: bug
found-in: milestone:M55-planning/S13
about: jigc task finalize
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# Two tasks on the ordinary finalize sweep each other's staged files

## Description

The general case of F1, declared a bound by the Settle (S13, `design/findings-channel.md` → 3 and → 6). A task finalized on the ordinary commit model commits the whole git index. So with two tasks open in one checkout, the one finalized first commits the other's staged files under its own message, at exit 0, and the other then fails `finalize.empty-commit`, whose only route is discard. `finalize.carried-staged` catches only paths staged before the first task started. M55's doc-only finalize closes the case for the report and triage workflows (`m55-f1`). It stays open for every workflow on `step:finalize`: two code tasks, or a code task and a code-less one such as `park-idea`, whose suite cell `doc_only_finalize::park_idea_over_the_same_states_behaves_as_today` pins the sweep as unchanged. The fix is per-task path claims, a new mechanism with no cheaper-now argument.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **open**. Two `single-task` tasks are minted, then `one.sh` and `two.sh` are staged, one for each. `jigc task finalize task-one` exits 0 with a commit `feat(probe): task-one lands` holding both files, and `jigc task finalize task-two` exits 3 with `finalize.empty-commit`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
fill_commit() {
  $JIGC doc set-field commit:$1#type --value feat --task $1 > /dev/null
  $JIGC doc set-field commit:$1#scope --value probe --task $1 > /dev/null
  printf '%s\n' "$1 lands" | $JIGC doc set-slot commit:$1#summary --from-file - --task $1 > /dev/null
  printf 'why\n' | $JIGC doc set-slot commit:$1#body --from-file - --task $1 > /dev/null
}
$JIGC start --workflow single-task "task one" > /dev/null
$JIGC start --workflow single-task "task two" > /dev/null
printf 'one\n' > one.sh; git add one.sh           # task-one's code
printf 'two\n' > two.sh; git add two.sh           # task-two's code, staged after both started
fill_commit task-one; fill_commit task-two
$JIGC task finalize task-one > /dev/null 2>&1; echo "task-one: exit $?"   # 0
git show --name-only --format=%s HEAD             # one.sh AND two.sh, under task-one's message
$JIGC task finalize task-two; echo "task-two: exit $?"                    # 3, finalize.empty-commit
```

## Resolution
