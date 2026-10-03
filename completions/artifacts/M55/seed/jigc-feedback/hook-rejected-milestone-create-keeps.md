---
kind: bug
found-in: review:M52-per-axis/(4,DEFECT 3)
about: jigc milestone create
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Hook-rejected milestone create keeps its gitignore amend unsaid

## Description

When a pre-commit hook rejects `jigc milestone create`, the door leaves its `.jigc/.gitignore` amend on disk. It names that amend nowhere and says *nothing … survives*. This is M51 §A's C4, narrowed to what rc.16 still did. `IGNORE_DOORS`' row for this door declares its ack to be *the `minted milestone:` ack itself*. On a rejected run that ack does not exist, so the declared channel never opens while the write survives. The frame composes *nothing … survives* with no *apart from* opener, because the surviving path is outside `ROLLBACK_POPULATIONS`. No data is lost, since the amend is a union and the user's own lines are kept. The guard-refusal arms M52 fixed stay closed.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. The setup is a user-authored `.jigc/.gitignore` and a rejecting hook behind `core.hooksPath`. `jigc milestone create 'Amend probe'` exits 1 and says *nothing was committed — the record write and the milestone workbench were both rolled back, so nothing of milestone:amend-probe survives*. Yet `.jigc/.gitignore` gained six lines (`index/` `state/` `milestones/` `worktrees/` `logs/` `displaced/`), `git status --short` shows ` M .jigc/.gitignore`, and neither stream mentions `gitignore`.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'tasks/\n# my private line\n' > .jigc/.gitignore
cp .jigc/.gitignore "$RIG/ign-pre.txt"
mkdir -p "$RIG/hooks"
printf '#!/bin/sh\necho "the hook says no" >&2\nexit 1\n' > "$RIG/hooks/pre-commit"
chmod +x "$RIG/hooks/pre-commit"; git config core.hooksPath "$RIG/hooks"
$JIGC milestone create 'Amend probe' > "$RIG/o" 2> "$RIG/e"     # exit 1
#   … nothing was committed — the record write and the milestone workbench were both
#   rolled back, so nothing of milestone:amend-probe survives. …
diff "$RIG/ign-pre.txt" .jigc/.gitignore     # 6 lines added: the amend survived
grep -c gitignore "$RIG/o" "$RIG/e"          # 0 0
git status --short                           #  M .jigc/.gitignore
```

## Resolution
