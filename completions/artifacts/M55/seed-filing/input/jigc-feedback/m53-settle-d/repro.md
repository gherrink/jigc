```sh
# a source read, from the jigc checkout: the walk's only exit is a free path
sed -n '/^fn free_displacement_path/,/^}/p' crates/cli/src/task.rs
#   let mut n = 2u32;
#   loop { … if std::fs::symlink_metadata(&candidate).is_err() { return candidate; } n += 1; }
```
