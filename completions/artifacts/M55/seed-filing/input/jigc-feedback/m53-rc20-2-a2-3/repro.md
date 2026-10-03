```sh
src=$PWD                                          # the jigc checkout
rig=$("$src"/dev/jigc-rig committed-singletons) || exit; eval "$rig"
$JIGC --format json task amend "json probe" > amend.json; echo "exit $?"   # 0
grep -c '"task"' amend.json                       # 1: the {task, text} arm
grep -Eo '[0-9a-f]{7,40}' amend.json | wc -l      # 0: no sha in the envelope, by declaration
grep -c 'carries the pinned sha under `text`, no new key needed — \*\*verify\*\*.*~~' \
  "$src"/completions/artifacts/M53/f10-amend-settle.md           # 1: the settle row is struck
```
