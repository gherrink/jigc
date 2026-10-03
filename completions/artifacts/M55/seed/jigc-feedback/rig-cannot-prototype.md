---
kind: inconvenience
found-in: milestone:M55-planning/F17
about: dev:jigc-rig
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# The rig cannot prototype a doctype composed with both packs

## Description

A methodology doctype cannot be prototyped in a rig composed with the dev pack. `JIGC_PACK_DIR`, which every `--pack-from-dev`, `--schema` and `--workflow` rig sets, supersedes the `compose-embedded-methodology` marker (`pack.rs:2183`), so the methodology pack does not compose at all. The other route, a listed project pack, has to copy `commands.yaml` and the finalize, author-commit and migration-finalize steps. The capabilities gap-detector found it, and it feeds the project-pack idea.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open** on both halves. On `dev/jigc-rig fresh --pack-from-dev`, with the marker `true`, `jigc doc schema vision` exits 1 with `store.unknown-type`, and with `JIGC_PACK_DIR` unset it exits 0. A listed pack carrying only a `house-note` schema and a `note-it` workflow that includes `step:author-commit` and `step:finalize` fails `jigc start --workflow note-it` at exit 1 with `pack.resource-missing`, *no composed pack ships `config/commands`*, searching only the listed pack.

## Repro

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

## Resolution
