```sh
src=$PWD                                              # the jigc checkout
rig=$("$src"/dev/jigc-rig fresh) || exit; eval "$rig"
printf '{{ include: step:no-such-step }}\n' > .jigc/broken.yaml
$JIGC config insert-step --workflow amend --after amend-message .jigc/broken.yaml; echo "insert-step: exit $?"   # 0
git add -- .jigc/config && git commit -qm "chore: a project structural delta"
$JIGC validate --format json; echo "validate: exit $?"           # 0, "findings": []
$JIGC task amend "repair the message"; echo "task amend: exit $?"   # 1, blocking workflow-refs.include-resolves
# the control: the same include in a project whole-file shadow of `amend` is reported
mkdir -p .jigc/config/workflows
{ cat "$src"/crates/cli/packs/dev/workflows/amend.yaml; printf '{{ include: step:no-such-step }}\n'; } > .jigc/config/workflows/amend.yaml
git add -- .jigc/config && git commit -qm "chore: a project workflow shadow"
$JIGC validate --format json; echo "validate: exit $?"           # 0, workflow-refs.include-resolves at workflow:amend
```
