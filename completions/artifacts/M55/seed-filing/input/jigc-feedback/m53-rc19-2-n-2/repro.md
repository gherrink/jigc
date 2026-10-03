```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
git mv VISION.md VISION-OOB.md                        # a placement singleton, renamed out of band
git commit -qm "probe VISION.md"; echo "exit $?"      # 1: the hook blocks the staged rename
git status --porcelain                                # R  VISION.md -> VISION-OOB.md, still staged
```
