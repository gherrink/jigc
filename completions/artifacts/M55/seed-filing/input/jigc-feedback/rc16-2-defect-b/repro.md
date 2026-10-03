```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
git checkout -qb cb; echo b > conf.txt; git add conf.txt
git -c core.hooksPath=/dev/null commit -qm b
git checkout -q main; echo m > conf.txt; git add conf.txt
git -c core.hooksPath=/dev/null commit -qm m
git merge --squash cb                          # exit 1, CONFLICT (add/add)
ls "$(git rev-parse --git-dir)"                # MERGE_MSG SQUASH_MSG, no MERGE_HEAD
git ls-files -u | wc -l                        # 2
$JIGC milestone create CS1                     # exit 1
#   blocking · repo.operation-in-progress — a squash merge is staged and not committed —
#     the repository is not in a committable state
```
