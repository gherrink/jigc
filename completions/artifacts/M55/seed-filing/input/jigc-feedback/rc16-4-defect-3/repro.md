```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
printf 'tasks/\n# my private line\n' > .jigc/.gitignore
cp .jigc/.gitignore "$RIG/ign-pre.txt"
mkdir -p "$RIG/hooks"
printf '#!/bin/sh\necho "the hook says no" >&2\nexit 1\n' > "$RIG/hooks/pre-commit"
chmod +x "$RIG/hooks/pre-commit"; git config core.hooksPath "$RIG/hooks"
$JIGC milestone create 'Amend probe' > "$RIG/o" 2> "$RIG/e"     # exit 1
#   … nothing was committed — the record write and the milestone workbench were both
#   rolled back, so nothing of milestone:amend-probe survives. …
diff "$RIG/ign-pre.txt" .jigc/.gitignore     # 6 lines added: the amend survived
grep -c gitignore "$RIG/o" "$RIG/e"          # 0 0
git status --short                           #  M .jigc/.gitignore
```
