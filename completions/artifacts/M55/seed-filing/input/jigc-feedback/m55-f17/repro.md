```sh
rig=$(dev/jigc-rig fresh --pack-from-dev) || exit; eval "$rig"     # exports $JIGC_PACK_DIR
cat .jigc/config/packs.yaml                                        # compose-embedded-methodology: true
$JIGC doc schema vision > /dev/null; echo "exit $?"                # 1, store.unknown-type
(unset JIGC_PACK_DIR; $JIGC doc schema vision > /dev/null; echo "control: exit $?")   # 0
unset JIGC_PACK_DIR                                                # the other route: a listed pack
mkdir -p "$RIG/house/schemas" "$RIG/house/workflows"
printf 'type: house-note\nlocation: house-notes/\nid-from: title\ndescription: A house note.\nusage: a house note.\nsections:\n  - id: note\n    slot: { hint: "The note." }\n' > "$RIG/house/schemas/house-note.yaml"
printf -- '---\nwhen: write a house note\ndescription: Write one house note.\nusage: a house note is due.\ncreates-task: true\nallows-create: [{type: house-note, as: note}]\n---\n{{ include: step:author-commit }}\n{{ include: step:finalize }}\n' > "$RIG/house/workflows/note-it.yaml"
printf 'compose-embedded-methodology: true\npacks:\n  - %s\n' "$RIG/house" > .jigc/config/packs.yaml
$JIGC start --workflow note-it "a note"; echo "exit $?"            # 1, pack.resource-missing: config/commands
```
