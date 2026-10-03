---
kind: bug
found-in: milestone:M55-planning/F13
about: jigc doc schema
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A same-id built-in doctype demotes a listed pack's doctype without a warning

## Description

A doctype shipped by a project-listed pack is silently demoted when a built-in doctype has the same id. There is no warning, and `jigc doc schema` shows the built-in shape. The doctypes gap-detector drove it with a copy-built binary embedding a `finding` doctype. It decided the namespaced id `jigc-feedback`. `inconsistency` kept its plain id, and an adopter's same-named doctype is shadowed: F13's general defect, true of every built-in doctype id, seeded rather than dodged per doctype (`design/findings-channel.md` → 1). The Settle seeded it open as a declared bound.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. A listed pack shipping its own `inconsistency` schema, with an `owner` field and a `note` slot, composes over `[dev ▸ methodology]`. `jigc doc schema inconsistency` exits 0 with the built-in shape (`kind`, `status`, `sides`, no `owner`), and neither `jigc validate` nor `jigc start` mentions it. The one place the loss surfaces is the `--explain` header, as `collision: doctype:inconsistency → won by methodology/1.0.0-rc.22`, the M49 rule that a listed pack sorts below any doctype a manifest freezes (`design/multi-pack.md`). The control, the same schema under a fresh id `house-note`, wins and shows `owner`.

## Repro

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

## Resolution
