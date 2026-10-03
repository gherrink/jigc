---
kind: bug
found-in: milestone:M55-planning/F11
about: jigc doc schema
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# A card on a non-ref field loads clean and does nothing

## Description

`card:` declared on a field that is not a `ref` was accepted at pack-load and ignored. On a `code-anchor` field, `jigc doc schema --format json` showed no card, and a list payload was rejected `expected a string`. The capabilities gap-detector called it a loader gap: `card` belongs to a `ref`, and on any other type the loader neither refuses it nor gives it a meaning.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `982f910f`): **still open**. An `adr` schema whose `cites-code` code-anchor carries `card: "0..*"`, installed with `dev/jigc-rig fresh --schema adr`, loads clean: `jigc doc schema adr` exits 0 and its text and JSON projections show no card. `engine::schema::Field` documents `card` as a `ref` key, and nothing checks the type it sits on. Two halves of the original evidence no longer separate the cases. On this build the schema JSON projects no card for a `ref` either (`supersedes`, declared `0..*`), and `jigc doc author` refuses a list payload for `supersedes` with the same `expected a string`. What stands is the loader accepting a key it gives no meaning.

## Repro

```sh
src=$PWD                                             # the jigc checkout
sed 's/{ id: cites-code, type: code-anchor }/{ id: cites-code, type: code-anchor, card: "0..*" }/' \
  "$src"/crates/cli/packs/dev/schemas/adr.yaml > "${TMPDIR:-/tmp}/adr-card.yaml"
rig=$("$src"/dev/jigc-rig fresh --schema adr "${TMPDIR:-/tmp}/adr-card.yaml") || exit; eval "$rig"
grep -c 'code-anchor, card: "0..\*"' "$JIGC_PACK_DIR/schemas/adr.yaml"   # 1: the pack declares it
$JIGC doc schema adr; echo "doc schema: exit $?"     # 0: loads clean, and no card on cites-code
$JIGC validate > /dev/null; echo "validate: exit $?" # 0: nothing names the stray key
```

## Resolution
