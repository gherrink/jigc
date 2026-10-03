```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
B=$(mktemp -d)                                   # a second repository
git -C "$B" init -q -b main
printf 'one\n' > "$B/one.txt"; git -C "$B" add one.txt; git -C "$B" commit -qm one
$JIGC milestone create "Foreign wave" > /dev/null
$JIGC milestone add-task foreign-wave "fw area" > /dev/null
W=$REPO/.jigc/worktrees/fw-area                  # not provisioned: B's own worktree parks there
git -C "$B" worktree add -q "$W" -b fwbranch
printf 'B-SECRET-PAYLOAD\n' > "$W/bsecret.txt"; git -C "$W" add bsecret.txt
$JIGC milestone join foreign-wave; echo "exit $?"       # 0
$JIGC milestone finalize foreign-wave; echo "exit $?"   # 0, "sub-tasks: fw-area: 1 code file"
git cat-file -p HEAD:bsecret.txt                         # B-SECRET-PAYLOAD, committed into this repo
```
