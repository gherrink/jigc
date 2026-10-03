---
kind: inconvenience
found-in: milestone:M53-post-review/private-target-litter
about: dev:gate
jigc-version: 1.0.0-rc.21
status: open
date: 2026-10-03
schema-version: 1
---

# Gate private targets pile up where clean-litter never looks

## Description

`dev/gate --private-target` mints a fresh `$TMPDIR/jigc-gate-target-*` build tree on every run, and `dev/clean-litter` never sees them. By 2026-09-26 there were 82 such trees, and a full gate took about 2 hours, against 16 minutes after the reset. The candidate fix reuses one directory by default, which `JIGC_GATE_TARGET` already supports, and has `clean-litter` list the stray trees with their sizes. It lists them and never removes them, per the rule the tool is built on. The row's trigger is the next wave that touches `dev/`.

Re-driven on this build (`jigc 1.0.0-rc.22`, the checkout at `982f910f`), by reading the two tools: **still open**. Without `JIGC_GATE_TARGET`, `dev/gate --private-target` still sets `CARGO_TARGET_DIR=$(mktemp -d "$tmp_root/jigc-gate-target-XXXXXX")` and prints how to reuse it next run. `dev/clean-litter` runs `cargo clean` over the workspace's target tree and names no `jigc-gate-target-*` tree. On the machine the re-drive ran on, `$TMPDIR` held 103 of them.

## Repro

```sh
# a source read, from the jigc checkout
grep -n 'jigc-gate-target-XXXXXX' dev/gate          # the default: a fresh mktemp -d per run
grep -c 'jigc-gate-target' dev/clean-litter         # 0: the reset never looks there
ls -d "${TMPDIR:-/tmp}"/jigc-gate-target-* 2>/dev/null | wc -l   # the trees that have piled up
```

## Resolution
