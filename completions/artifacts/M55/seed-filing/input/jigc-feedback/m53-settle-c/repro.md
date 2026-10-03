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
