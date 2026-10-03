---
kind: bug
found-in: milestone:M55-completion/F21
about: jigc validate
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# Validate misses a workflow-ref break a project structural delta introduces

## Description

`jigc validate`'s store-scope `workflow-refs` check reads each workflow's definition without the project layer's `structural-op` deltas (`jigc config replace-step` / `insert-step` / `remove-step`). `enumerate_store_workflows` hands the raw cascade-resolved bytes to the engine's `workflow_refs_store`, which expands that include list, never the post-delta one. So a broken or circular include, or a bad ref inside an inserted native step, is never reported. Compose refuses the same workflow, so the report misses a finding but no wrong commit lands. The fix needs an engine API change, because `workflow_refs_store` takes workflow bytes, not a resolved include list. The M55 completion fix `1306d891` surfaced it, when it made every compose door apply the deltas. The row is F21 of the M55 findings register, recorded after the seed was generated.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `92f279c3`): **open**. In a fresh rig, `jigc config insert-step --workflow amend --after amend-message` records a native step whose body is `{{ include: step:no-such-step }}` at exit 0. With the delta committed, `jigc validate --format json` exits 0 with `"findings": []`, while `jigc task amend` refuses at exit 1 with a blocking `workflow-refs.include-resolves`. The control: the same include placed in a project whole-file shadow of `amend` is reported by `validate` as `workflow-refs.include-resolves` at `workflow:amend`. So the check runs at store scope and misses only the delta path.

## Repro

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

## Resolution
