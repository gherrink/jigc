```sh
# from the jigc checkout
cargo test -p jigc --test g_doc author_batch_scaling::     # passes alone
grep -n 'const MAX_GROWTH_RATIO' crates/cli/tests/author_batch_scaling.rs   # 5.5
# the red was a full-suite run under load: `dev/gate`, and read its test step's failures
```
