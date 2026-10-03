```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
mkdir -p "$RIG/house/schemas"                                     # an adopter's own pack
printf 'type: inconsistency\nlocation: house-notes/\nid-from: title\ndescription: A house note.\nusage: a house note.\nsections:\n  - id: meta\n    header: true\n    fields:\n      - { id: owner, type: string }\n  - id: note\n    slot: { hint: "The note." }\n' > "$RIG/house/schemas/inconsistency.yaml"
sed 's/^type: inconsistency$/type: house-note/' "$RIG/house/schemas/inconsistency.yaml" > "$RIG/house/schemas/house-note.yaml"
printf 'compose-embedded-methodology: true\npacks:\n  - %s\n' "$RIG/house" > .jigc/config/packs.yaml
$JIGC doc schema inconsistency; echo "exit $?"                 # 0: the built-in shape, no `owner`
$JIGC validate 2>&1 | grep -ci 'inconsistency'                 # 0: no warning
$JIGC doc schema house-note | grep -c 'owner'                  # 1: the control, a fresh id, wins
$JIGC start --workflow single-task "probe" --explain | grep 'doctype:inconsistency'
#   collision: doctype:inconsistency → won by methodology/1.0.0-rc.22   <- the one place it surfaces
```
