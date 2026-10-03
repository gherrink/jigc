```sh
git config --global --get commit.gpgsign             # true: the precondition
W=$(mktemp -d "${TMPDIR:-/tmp}/signed.XXXXXX"); cd "$W"
git init -q && git config user.name t && git config user.email t@example.com
echo a > a && git add a && git commit -qm one         # as most test repositories commit
git log --format='%G?' -1                             # G: signed with the developer's key
```
