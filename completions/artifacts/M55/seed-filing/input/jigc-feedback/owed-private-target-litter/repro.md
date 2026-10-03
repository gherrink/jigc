```sh
# a source read, from the jigc checkout
grep -n 'jigc-gate-target-XXXXXX' dev/gate          # the default: a fresh mktemp -d per run
grep -c 'jigc-gate-target' dev/clean-litter         # 0: the reset never looks there
ls -d "${TMPDIR:-/tmp}"/jigc-gate-target-* 2>/dev/null | wc -l   # the trees that have piled up
```
