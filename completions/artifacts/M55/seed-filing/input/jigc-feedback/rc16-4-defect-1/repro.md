```sh
rig=$(dev/jigc-rig committed-singletons --start quick-fix "axis four probe") || exit; eval "$rig"
export PATH="$(dirname "$JIGC"):$PATH"
jigc doc set-field commit:axis-four-probe#header/type --value fix
printf 'four\n' | jigc doc set-slot commit:axis-four-probe#summary --from-file -
echo w > work.txt; git add work.txt
: > "$(git rev-parse --git-path index.lock)"
jigc task finalize axis-four-probe            # exit 3, finalize.stage-failed
#   route: `jigc task finalize axis-four-probe` once the embedded git failure is resolved
#     (e.g. remove a stale `.git/index.lock`) — the task survives intact, so the same
#     re-run lands the commit
rm -f "$(git rev-parse --git-path index.lock)"
jigc task finalize axis-four-probe            # the route, verbatim: exit 0, the commit lands
```
