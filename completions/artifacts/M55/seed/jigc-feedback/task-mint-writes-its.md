---
kind: feedback
found-in: milestone:M53-settle/(D)(e)
about: jigc start
jigc-version: 1.0.0-rc.16
status: open
date: 2026-10-03
schema-version: 1
---

# The task mint writes its base pin non-atomically

## Description

`engine::state::mint_task` writes `base.json` with a plain `std::fs::write`, while `engine::milestone::mint_milestone` writes the same pin through the atomic `crate::state::persist`. M53 Increment 3's residual rule keys on the pin's existence, so it is unaffected, which is why the asymmetry survived that pass. But a predicate keyed on the pin's parse would have a torn-read window at the task door and none at the milestone door. It was read, not driven, and no racer was driven either: `design/storage.md` → *Concurrent writers* names the hook as the designed racer, and no hook runs inside `mint_task`. No defect has been driven, so its kind is `feedback`. What is owed is the two mint doors writing the same file the same way. The Settle's trigger here is the first predicate that parses the base pin rather than asking whether it exists.

Re-driven on this build, by reading the source: **still open**. `mint_task` still writes the pin with `std::fs::write(&pin_path, body)`, and `mint_milestone` with `crate::state::persist(&pin_path, …)`.

## Repro

```sh
# a source read, from the jigc checkout: the two mint doors write base.json two ways
sed -n '/^pub fn mint_task(/,/^}/p' crates/engine/src/state.rs | grep -n 'pin_path'
#   std::fs::write(&pin_path, body)               <- a plain write
sed -n '/^pub fn mint_milestone(/,/^}/p' crates/engine/src/milestone.rs | grep -n 'pin_path'
#   crate::state::persist(&pin_path, …)           <- the atomic one
```

## Resolution
