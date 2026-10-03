```sh
gh api repos/gherrink/jigc/environments/release/deployment-branch-policies \
  --jq '.total_count, (.branch_policies[] | "\(.id):\(.type):\(.name)")'   # 1, then 61391261:branch:main
grep -c 'policy 61391261' implementation/release.md                         # 1
```
