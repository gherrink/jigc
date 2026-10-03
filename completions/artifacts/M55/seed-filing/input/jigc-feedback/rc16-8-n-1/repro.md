```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC uninstall --help     # about: "Four states it refuses … `--force`, which deletes all four"
#                            --force: "… Inert when all three guards are already clean"
mkdir -p .jigc/displaced/u-four; printf 'mine\n' > .jigc/displaced/u-four/notes.txt
$JIGC uninstall            # exit 1, uninstall.foreign-bytes  .jigc/displaced/u-four/notes.txt
$JIGC uninstall --force    # exit 0 — not inert
#   warning: removing the relocation workbench .jigc/displaced discards work that is not in git:
#   note: the relocation workbench is the only copy of these bytes — they are not recoverable.
```
