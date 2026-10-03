```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig bare) || exit; eval "$rig"
export JIGC_PACK_DIR=$RIG/pack; cp -R "$src"/crates/cli/packs/dev "$JIGC_PACK_DIR"
# the v2 adr: `cites-code` a string, and the `options` slot required
sed -e 's/{ id: cites-code, type: code-anchor }/{ id: cites-code, type: string }/' \
    -e 's/slot: { optional: true, hint: "Alternatives/slot: { hint: "Alternatives/' \
    "$JIGC_PACK_DIR/schemas/adr.yaml" > "$JIGC_PACK_DIR/schema-snapshots/adr.v2.yaml"
sed -i.bak '/- type: adr/{n;s/schema-version: 2/schema-version: 3/;}' "$JIGC_PACK_DIR/config/schema-manifest.yaml"
mkdir -p .jigc/config docs/decisions
printf -- '---\nstatus: accepted\ndate: 2026-06-25\ncites-code: the session cache module\nschema-version: 2\n---\n\n# Alpha decision\n\n## Context\n\nC.\n\n## Options\n\nO.\n\n## Decision\n\nD.\n\n## Consequences\n\nQ.\n' > docs/decisions/alpha.md
git add docs && git commit -qm "a v2 adr"
$JIGC migrate-corpus; echo "migrate-corpus: exit $?"   # 0: 1 migrated, 0 blocked
grep '^schema-version' docs/decisions/alpha.md         # schema-version: 3
$JIGC validate                                          # blocking (gates at finalize) doc-code.symbol-exists
```
