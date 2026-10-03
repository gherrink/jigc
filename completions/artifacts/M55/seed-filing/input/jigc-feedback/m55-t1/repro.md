```sh
# from the jigc checkout
cargo test -p jigc --test g_finalize validate_previews_posture::the_cd_detector_reads_a_command_not_a_short_sha   # 1 passed
grep -n 'fn offers_a_cd' crates/cli/tests/validate_previews_posture.rs   # the detector the assertion now reads
```
