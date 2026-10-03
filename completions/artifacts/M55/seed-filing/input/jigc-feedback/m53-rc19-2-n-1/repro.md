```sh
rig=$(dev/jigc-rig committed-singletons) || exit; eval "$rig"
printf 'ordinary\n' > ord.txt; git add ord.txt; git commit -qm "probe"; echo "exit $?"   # 0, silent
git rm -q VISION.md                                   # a deletion; no rename anywhere
git commit -qm "probe: delete a managed singleton"; echo "exit $?"                   # 0, silent
#   rc.19 printed "an out-of-band managed-doc rename exists … (not staged in this commit …)"
printf 'b\n' > f2.txt; git add f2.txt; git commit -qm "unrelated 2"; echo "exit $?"     # 0, silent
```
