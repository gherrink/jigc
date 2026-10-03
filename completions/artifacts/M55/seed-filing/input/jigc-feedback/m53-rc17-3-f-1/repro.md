```sh
rig=$(dev/jigc-rig fresh --start quick-fix "tidy the readme") || exit; eval "$rig"
OUT=$(mktemp -d); printf 'TARGET-BYTES\n' > "$OUT/body.md"
A=.jigc/tasks/tidy-the-readme/docs
mkdir -p "$A"
ln -s "$OUT/body.md" "$A/adr:via-symlink.md"
$JIGC task discard tidy-the-readme; echo "exit $?"          # 1, task-discard.foreign-bytes names it
$JIGC doc list --task tidy-the-readme; echo "exit $?"       # 0, adr:via-symlink … managed
$JIGC task discard tidy-the-readme --force; echo "exit $?"  # 0, "dropped staged edits to: adr:via-symlink"
cat "$OUT/body.md"                                          # TARGET-BYTES: the target survives
```
