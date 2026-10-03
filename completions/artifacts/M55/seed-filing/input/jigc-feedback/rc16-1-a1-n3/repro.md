```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC start --workflow single-task "axis one"
$JIGC doc set-field "roadmap:roadmap#milestones/m-beta/title" --value X --task axis-one --format json
#   exit 1  {"schema_version": 3, "findings": [{"code": "write.not-present", "key": {…}, …}]}
A=$(printf 'a%.0s' $(seq 1 300))
$JIGC doc set-field "roadmap:$A#milestones/m-alpha/title" --value X --task axis-one --format json
#   exit 1  {"error": "blocking · write.slug-name-ceiling — the `<slug>` head of address … is
#            300 bytes — over the 165-byte ceiling …\n  at: …\n  route: …"}
```
