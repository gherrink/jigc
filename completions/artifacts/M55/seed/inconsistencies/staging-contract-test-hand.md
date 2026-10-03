---
kind: code-doc
status: open
date: 2026-10-03
schema-version: 1
---

# The staging-contract test hand-lists four committing workflows

## Sides

### crates/cli/tests/methodology_staging_contract.rs:131  {#side1}

`COMMITTING_WORKFLOWS`, documented as *the four methodology work-workflows that carry a task to a commit — every one of them composes `step:finalize`*: `dev-task`, `decided-task`, `planning`, `completion`.

### crates/cli/packs/methodology/workflows/  {#side2}

Eight workflows compose `step:finalize` and carry a task to a commit: the four, plus `park-idea`, `form-vision`, `do-research` and `record-dogfood`. `planning` composes it through `step:planning-finalize`.

### implementation/doctype-authoring.md:112  {#side3}

The list holds *the methodology workflows that carry a task to a code commit*. It has no red, so a new committing workflow escapes it silently.

## Description

`methodology_staging_contract`'s `COMMITTING_WORKFLOWS` hand-lists four workflows, and its doc comment calls them *the four methodology work-workflows that carry a task to a commit — every one of them composes `step:finalize`*. More than four methodology workflows compose `step:finalize`. So the comment's claim is false, and the fence covers only the four. The docs gap-detector found it at `:131`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a7a742d3`): **still open**. The list still holds four. Eight methodology workflows compose `step:finalize`, and the composed text of each carries the `git add` contract and its consequence, *only what you have staged*: the four, plus `park-idea`, `form-vision`, `do-research` and `record-dogfood`. M55 Increment 7 (`582f045a`) recorded the list in doctype-authoring.md's checklist as *the methodology workflows that carry a task to a code commit*. That reading leaves the four code-less ones out by design, but the test's comment does not say so, and nothing reddens when a committing workflow is missing from the list.

## Evidence

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
for w in dev-task decided-task planning completion park-idea form-vision do-research record-dogfood; do
  printf '%s ' $w; $JIGC workflow $w --preview | tr -s ' \n' '  ' | grep -c 'only what you have staged'
done                                   # 1 for each of the eight
```

## Resolution
