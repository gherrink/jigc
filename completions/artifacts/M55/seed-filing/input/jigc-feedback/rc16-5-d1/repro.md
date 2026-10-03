```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC --format json start --explain > "$RIG/o" 2> "$RIG/e"    # exit 0, stderr empty
python3 -c 'import json,sys; print(sorted(json.load(open(sys.argv[1]))))' "$RIG/o"
#   ['collision_winners', 'overrides_applied', 'pack_inputs', 'schema_version', 'steps',
#    'workflow', 'workflow_layer']
# ENVELOPE_ARMS' `start` rows (crates/cli/src/render.rs): OrientationView::UnsetProject,
#   OrientationView::Clean, OrientationView::ActiveTask, Composed — none is this arm
```
