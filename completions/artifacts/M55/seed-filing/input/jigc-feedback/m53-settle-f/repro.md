```sh
src=$PWD                                          # the jigc checkout
sed -n '/^### R-I /,/^### R-J /p' "$src"/completions/artifacts/M52/per-axis-review/axis-3.md \
  | grep -c 'grep -rl'                            # 3: the evidence lines are unchanged
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
mkdir -p .jigc/tasks; printf 'PRECIOUS-MARKER\n' > .jigc/tasks/plant.txt
git check-ignore .jigc/tasks/plant.txt            # .jigc/tasks/plant.txt: ignored, so a search that
                                                  # honours .gitignore (ugrep --ignore-files, rg) skips it
command grep -rl PRECIOUS-MARKER .                # ./.jigc/tasks/plant.txt: the bytes are there
```
