```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
$JIGC start --workflow fix-task "fix the thing"      # exit 0, task minted: fix-the-thing
#   line 25: Never `git commit` and never `jigc task finalize` here. …
$JIGC doc set-field commit:fix-the-thing#header/type --value fix
printf 'fix the thing\n' | $JIGC doc set-slot commit:fix-the-thing#summary --from-file -
echo f > fixed.txt; git add fixed.txt
$JIGC task finalize fix-the-thing                    # exit 0 — the door line 25 forbids
git log -1 --format=%s                               # fix: fix the thing
```
