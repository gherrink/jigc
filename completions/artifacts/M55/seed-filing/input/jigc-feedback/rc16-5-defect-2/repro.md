```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'id: probe-step\ntitle: Probe\nbody: |\n  Probe.\n' > "$RIG/probe-step.yaml"
$JIGC config insert-step --workflow single-task --before finalize "$RIG/probe-step.yaml"   # exit 0
$JIGC --format json start --explain --workflow single-task   # .steps[].id carries probe-step
$JIGC config remove-step 'workflow:single-task#probe-step'   # exit 1
#   blocking · config.anchor-absent — no step `probe-step` body to fork
#     route: name a step id present in the workflow's resolved include list, then re-run
printf 'id: probe-step\ntitle: Probe2\nbody: |\n  Probe two.\n' > "$RIG/repl.yaml"
$JIGC config replace-step 'workflow:single-task#probe-step' "$RIG/repl.yaml"   # exit 1, the same
$JIGC config remove-step 'workflow:single-task#implement'    # control: exit 0
```
