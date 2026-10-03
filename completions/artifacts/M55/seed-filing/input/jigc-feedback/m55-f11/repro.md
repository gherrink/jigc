```sh
src=$PWD                                             # the jigc checkout
sed 's/{ id: cites-code, type: code-anchor }/{ id: cites-code, type: code-anchor, card: "0..*" }/' \
  "$src"/crates/cli/packs/dev/schemas/adr.yaml > "${TMPDIR:-/tmp}/adr-card.yaml"
rig=$("$src"/dev/jigc-rig fresh --schema adr "${TMPDIR:-/tmp}/adr-card.yaml") || exit; eval "$rig"
grep -c 'code-anchor, card: "0..\*"' "$JIGC_PACK_DIR/schemas/adr.yaml"   # 1: the pack declares it
$JIGC doc schema adr; echo "doc schema: exit $?"     # 0: loads clean, and no card on cites-code
$JIGC validate > /dev/null; echo "validate: exit $?" # 0: nothing names the stray key
```
