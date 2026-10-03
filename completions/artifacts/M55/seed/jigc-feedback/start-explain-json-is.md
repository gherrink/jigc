---
kind: bug
found-in: review:M52-per-axis/(5,D1)
about: jigc start
jigc-version: 1.0.0-rc.16
status: open
tier: tier-3
date: 2026-10-03
schema-version: 1
---

# Start explain JSON is an undeclared envelope arm

## Description

`jigc start --explain` emits a production `--format json` stdout arm that `ENVELOPE_ARMS` does not carry. This is the class of M51's DEFECT B, and it survived both passes. `Command::Start { explain: true, .. }` short-circuits into `run_explain` before compose-or-orient, and that prints the JSON of `engine::result::ResolutionTree`. `ENVELOPE_ARMS` carries exactly four `start` rows, the three `OrientationView` variants and `Composed`, and none of them is this arm. The driver's sweep never reached it, because `MINIMAL_ARGV` for `start` is the orient form. The finding originated with the reconciler. The rc.20 re-review found the arm's key set is not even fixed: `scalar_overrides` appears once a scalar override exists.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a768fe75`): **still open**. `jigc --format json start --explain` exits 0 with 939 bytes on stdout and none on stderr. Its top-level keys are `['collision_winners', 'overrides_applied', 'pack_inputs', 'schema_version', 'steps', 'workflow', 'workflow_layer']`. `crates/cli/src/render.rs`'s `ENVELOPE_ARMS` still carries only the four `start` rows.

## Repro

```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC --format json start --explain > "$RIG/o" 2> "$RIG/e"    # exit 0, stderr empty
python3 -c 'import json,sys; print(sorted(json.load(open(sys.argv[1]))))' "$RIG/o"
#   ['collision_winners', 'overrides_applied', 'pack_inputs', 'schema_version', 'steps',
#    'workflow', 'workflow_layer']
# ENVELOPE_ARMS' `start` rows (crates/cli/src/render.rs): OrientationView::UnsetProject,
#   OrientationView::Clean, OrientationView::ActiveTask, Composed — none is this arm
```

## Resolution
