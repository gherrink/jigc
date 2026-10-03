```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'TRACKED-BYTES-AT-HEAD\n' > keeper.md; git add keeper.md; git commit -qm "chore: keeper"
$JIGC milestone create "Axis three probe" > /dev/null
$JIGC milestone add-task axis-three-probe "first sub" > /dev/null
$JIGC milestone provision axis-three-probe
W=.jigc/worktrees/first-sub
printf 'SUBWORK\n' > "$W/subwork.txt"; git -C "$W" add subwork.txt
rm "$W/keeper.md"                                  # delete a TRACKED file; do not stage it
git -C "$W" status --porcelain                     # " D keeper.md"
$JIGC milestone join axis-three-probe
$JIGC milestone finalize axis-three-probe; echo "exit $?"
#   0; "first-sub: keeper.md (never staged)" under "not committed, not recoverable"
git show HEAD:keeper.md; cat keeper.md             # TRACKED-BYTES-AT-HEAD, twice: the bytes are in git
```
