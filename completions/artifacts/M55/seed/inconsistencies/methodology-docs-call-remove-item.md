---
kind: code-doc
status: open
date: 2026-10-03
schema-version: 1
---

# Methodology docs call remove-item unwired

## Sides

### design/methodology-docs.md:112  {#side1}

*What stays deferred*: the `remove-item`/`reorder` CLI verbs, because *the engine `remove_item` exists, unwired*.

### crates/cli/tests/doc_remove_item.rs  {#side2}

`jigc doc remove-item` ships. It was added in `b6f6843d` on 2026-06-16, and this suite drives it over top-level and nested items.

## Description

`design/methodology-docs.md` lists the `remove-item` and `reorder` CLI verbs under *What stays deferred*, and says the engine's `remove_item` exists but is unwired. `jigc doc remove-item` has shipped since `b6f6843d`, a week after the M16 planning that wrote the line. The section is maintained: its multi-level-repetition bullet was updated when that capability shipped. The decisions gap-detector found it.

Re-driven on this build (`jigc 1.0.0-rc.22`, the debug binary built from `a7a742d3`): **still open**. The line moved from `:108` to `:112` and is unchanged. `jigc doc remove-item` removes an item at exit 0. `reorder` is still no verb (`jigc doc reorder --help` exits 2), so that half of the bullet stands.

## Evidence

```sh
rig=$(dev/jigc-rig fresh) || exit; eval "$rig"
T=$($JIGC start --workflow report-inconsistency "probe sides" | sed -n 's/^task minted: //p')
$JIGC doc create inconsistency --title "Probe sides" --task $T > /dev/null
$JIGC doc add-item inconsistency:probe-sides#sides --title a.md --slug side1 --task $T > /dev/null
$JIGC doc add-item inconsistency:probe-sides#sides --title b.md --slug side2 --task $T > /dev/null
$JIGC doc remove-item inconsistency:probe-sides#sides/side2 --task $T; echo "exit $?"   # removed item …, exit 0
$JIGC doc show inconsistency:probe-sides --task $T | grep -c side2                   # 0
```

## Resolution
