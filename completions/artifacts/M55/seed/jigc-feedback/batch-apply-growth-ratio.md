---
kind: bug
found-in: milestone:M55-planning/T5
about: test:author_batch_scaling
jigc-version: 1.0.0-rc.22
status: open
date: 2026-10-03
schema-version: 1
---

# The batch-apply growth-ratio test flakes under full-suite load

## Description

`author_batch_scaling::the_batch_apply_growth_ratio_stays_within_its_stated_bound` measured a growth ratio of 6.16 against its 5.5 bound under full-suite load, on the R5 fold's first gate run on 2026-10-02. It passed alone and on the re-run. The bound, `MAX_GROWTH_RATIO = 5.5`, is a wall-clock ratio, so a loaded machine can inflate it.

Re-driven on this build (the checkout at `982f910f`): **still open**, on one datum. It did not recur: the full gate run of this re-drive passed it under load, it passed three runs alone, and six concurrent copies of it all passed. Neither the test nor its bound has changed since the red run (`git log` on the file ends at `6fccfbc3`), so the next loaded red is a second datum, not a new finding.

## Repro

```sh
# from the jigc checkout
cargo test -p jigc --test g_doc author_batch_scaling::     # passes alone
grep -n 'const MAX_GROWTH_RATIO' crates/cli/tests/author_batch_scaling.rs   # 5.5
# the red was a full-suite run under load: `dev/gate`, and read its test step's failures
```

## Resolution
