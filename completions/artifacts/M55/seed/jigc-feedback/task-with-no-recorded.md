---
kind: bug
found-in: milestone:M53-settle/(D)(c)
about: jigc task bind
jigc-version: 1.0.0-rc.16
status: open
date: 2026-10-03
schema-version: 1
---

# A task with no recorded workflow is refused without a code

## Description

A pinned task area whose `workflow` file is absent is refused at two producers without a code. `jigc task bind` answers *task at "<host-absolute path>" has no recorded workflow — discard it and re-start with `jigc start`*, a bare `anyhow` through `cli::task`'s `workflow_def`, which also prints the area host-absolute with `{:?}`. The create-gate doors, through `cli::doc`'s `workflow_gate` (reached by `doc create`, `doc author` and the copy-in role binding), answer *the active task has no recorded workflow — …*, code-less, and flattened to `{"error": …}` on `--format json`. The plain write verbs are not gated: `jigc doc set-slot --task <id>` over the same area exits 0, so the Settle's wording, *the `doc` write verbs*, was wider than the binary. M53 Increment 3 closed the residual half, an area with no base pin. Nothing is lost. What is owed is an identity at both producers, and `task bind`'s path rendered through `render::repo_relative`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `47e6d8a9`): **still open**. With `.jigc/tasks/probe-bind/workflow` removed, `jigc task bind decision adr:x probe-bind` exits 1 with no code: *task at "<absolute repo>/.jigc/tasks/probe-bind" has no recorded workflow — discard it and re-start with `jigc start`*. `jigc --format json doc create adr --title "X" --task probe-bind` exits 1 with a flattened `{"error": …}` whose text is *the active task has no recorded workflow*, followed by its discard-and-restart advice. `jigc doc set-slot commit:probe-bind#summary --task probe-bind` exits 0.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons --start single-task "probe bind") || exit; eval "$rig"
rm .jigc/tasks/probe-bind/workflow                  # a pinned area with no recorded workflow
$JIGC task bind decision adr:x probe-bind; echo "exit $?"
#   1, no code: task at "<absolute repo>/.jigc/tasks/probe-bind" has no recorded workflow …
$JIGC --format json doc create adr --title "X" --task probe-bind; echo "exit $?"
#   1, {"error": "the active task has no recorded workflow — …"}: no code, no key
printf 'p\n' | $JIGC doc set-slot commit:probe-bind#summary --from-file - --task probe-bind; echo "exit $?"
#   0: the plain write verbs are not gated, so only the create-gate doors refuse
```

## Resolution
