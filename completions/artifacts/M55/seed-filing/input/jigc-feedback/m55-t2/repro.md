```sh
# from the jigc checkout
cargo test -p jigc --test g_doc child_stdin_feed::      # 2 passed
grep -rn 'write_all' crates/cli/tests --include='*.rs' | grep -c 'expect("write stdin")'   # 2: both in doc comments
```
