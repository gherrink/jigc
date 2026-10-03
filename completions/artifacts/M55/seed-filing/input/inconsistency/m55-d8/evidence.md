```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
for w in dev-task decided-task planning completion park-idea form-vision do-research record-dogfood; do
  printf '%s ' $w; $JIGC workflow $w --preview | tr -s ' \n' '  ' | grep -c 'only what you have staged'
done                                   # 1 for each of the eight
```
