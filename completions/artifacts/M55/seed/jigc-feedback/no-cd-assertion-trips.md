---
kind: bug
found-in: milestone:M55-planning/T1
about: test:validate_previews_posture
jigc-version: 1.0.0-rc.22
status: resolved
pinned-by: validate_previews_posture::the_cd_detector_reads_a_command_not_a_short_sha
date: 2026-10-03
schema-version: 1
---

# A no-cd assertion trips on a short SHA ending in cd

## Description

`validate_previews_posture.rs:602` asserted that stderr carries no `"cd "`, so any short SHA ending in `cd` followed by a space tripped it, about 1 run in 256. The gate-speed measurement caught it as a nextest failure on `base d80f6cd but`.

## Repro

```sh
# from the jigc checkout
cargo test -p jigc --test g_finalize validate_previews_posture::the_cd_detector_reads_a_command_not_a_short_sha   # 1 passed
grep -n 'fn offers_a_cd' crates/cli/tests/validate_previews_posture.rs   # the detector the assertion now reads
```

## Resolution

Fixed by the gate-speed PR (#8, `559d8489`, *the no-`cd` assertion reads a command, not a short SHA ending in `cd`*). The assertion goes through `offers_a_cd`, which counts a `cd ` only when no word character, `-`, `.` or `/` precedes it, and the pinning test feeds it the flaking shape.

Re-driven on this build (the checkout at `982f910f`). The pinning test passes, and the full gate run of this re-drive passed `validate_previews_posture::` with it.
