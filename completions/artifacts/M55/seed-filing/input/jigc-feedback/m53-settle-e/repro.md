```sh
# a source read, from the jigc checkout: the two mint doors write base.json two ways
sed -n '/^pub fn mint_task(/,/^}/p' crates/engine/src/state.rs | grep -n 'pin_path'
#   std::fs::write(&pin_path, body)               <- a plain write
sed -n '/^pub fn mint_milestone(/,/^}/p' crates/engine/src/milestone.rs | grep -n 'pin_path'
#   crate::state::persist(&pin_path, …)           <- the atomic one
```
