```sh
rig=$(dev/jigc-rig committed-singletons --start quick-fix "empty commit probe") || exit; eval "$rig"
$JIGC doc set-field commit:empty-commit-probe#header/type --value fix
printf 'empty\n' | $JIGC doc set-slot commit:empty-commit-probe#summary --from-file -
$JIGC task validate empty-commit-probe; echo "exit $?"                 # 0, validates clean
$JIGC task finalize empty-commit-probe --dry-run; echo "exit $?"       # 3, finalize.empty-commit
$JIGC task finalize --help | grep -c 'the empty-commit guard (`finalize.empty-commit`'       # 1
$JIGC task finalize --help | grep -c 'The gates named above are \*\*outside\*\* that set'   # 1
```
