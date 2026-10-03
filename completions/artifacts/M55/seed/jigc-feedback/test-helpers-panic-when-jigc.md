---
kind: bug
found-in: milestone:M55-planning/T2
about: test:trial_corpus
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: child_stdin_feed::a_child_that_exits_without_reading_its_stdin_is_judged_on_its_exit
date: 2026-10-03
schema-version: 1
---

# Test helpers panic when jigc exits before reading its stdin

## Description

`trial_corpus.rs:520` called `expect` on writing jigc's stdin, so it panicked with `BrokenPipe` whenever jigc exited before reading. The same pattern stood at 143 sites, and faster runs triggered it more often. The gate-speed measurement found it.

## Repro

```sh
# from the jigc checkout
cargo test -p jigc --test g_doc child_stdin_feed::      # 2 passed
grep -rn 'write_all' crates/cli/tests --include='*.rs' | grep -c 'expect("write stdin")'   # 2: both in doc comments
```

## Resolution

Fixed by the gate-speed PR (#8, `8ec4943f`, *one stdin feeder that tolerates a child exiting before it reads*). `support::child_stdin::feed` is the one stdin writer. It judges a child that exits without reading on its exit status, not on the broken pipe.

Re-driven on this build (the checkout at `982f910f`). `child_stdin_feed::` passes 2 of 2, 149 suite files call the feeder, and the only `.write_all(…).expect("write stdin")` left in the tests are the two doc comments that describe the old pattern.
