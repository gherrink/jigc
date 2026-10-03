---
kind: bug
found-in: milestone:M55-planning/F1
about: jigc task finalize
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: doc_only_finalize::every_flow_b_state_lands_the_report_doc_alone
date: 2026-10-03
schema-version: 1
---

# A code-less finalize commits another task's staged code

## Description

A code-less task's `jigc task finalize` committed code that another open task staged *after* the code-less task started. It exited 0 with no warning, under the code-less task's commit message, and the code task then failed `finalize.empty-commit`, whose only route is discard. The M55 planning spike drove it over three states, a dev task open beside a `park-idea` report: state 3 swept `README.md` and `greet.sh` into `docs(ideas): …`. `finalize.carried-staged` catches only paths staged *before* the code-less task started.

## Repro

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start --workflow single-task "code work" > /dev/null                 # a code task, open
T=$($JIGC start --workflow report-jigc-feedback "file a finding" | sed -n 's/^task minted: //p')
printf 'echo hi\n' > greet.sh; git add greet.sh                           # its code, staged AFTER the report started
$JIGC doc create jigc-feedback --title "Probe finding" --task $T > /dev/null
$JIGC doc set-field jigc-feedback:probe-finding#meta/kind --value bug --task $T
$JIGC doc set-field jigc-feedback:probe-finding#meta/found-in --value task:probe --task $T
$JIGC doc set-field jigc-feedback:probe-finding#meta/jigc-version --value 1.0.0-rc.22 --task $T
printf 'Observed.\n' | $JIGC doc set-slot jigc-feedback:probe-finding#description --from-file - --task $T
$JIGC doc set-field commit:$T#type --value docs --task $T
$JIGC doc set-field commit:$T#scope --value findings --task $T
printf 'file a probe finding\n' | $JIGC doc set-slot commit:$T#summary --from-file - --task $T
printf 'A probe.\n' | $JIGC doc set-slot commit:$T#body --from-file - --task $T
$JIGC task finalize $T; echo "exit $?"           # 0, `left-out … greet.sh`
git show --name-only --format=%s HEAD           # docs/jigc-feedback/probe-finding.md alone
git diff --cached --name-only                   # greet.sh: still staged for the code task
```

## Resolution

Fixed for the report and triage workflows in M55 Increment 1 (`d07d4df4`, *the doc-only finalize commits a task's own docs, path-scoped*), with its narration in `3b570c20` and `0f32c485`. Those workflows compose `step:finalize-doc-only`, whose finalize stages and commits only the task's own docs and the owner-artifacts they record, never the index. Anything else staged stays staged and is named in the ack's `left-out` section. `design/findings-channel.md` → 3 is the design. The general case is declared out of this fix and filed as its own row, `m55-bound-two-code-tasks`: a workflow on the ordinary `step:finalize`, a code task or a code-less one such as `park-idea`, still commits the whole index. `doc_only_finalize::park_idea_over_the_same_states_behaves_as_today` pins that omitting context.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`). With a `single-task` open and its `greet.sh` staged after a `report-jigc-feedback` task was minted, the report's `jigc task finalize` exits 0. Its commit holds `docs/jigc-feedback/probe-finding.md` alone, its ack names `greet.sh` under `left-out`, and `greet.sh` is still staged afterwards.
