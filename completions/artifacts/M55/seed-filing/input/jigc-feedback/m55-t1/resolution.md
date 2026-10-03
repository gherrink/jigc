Fixed by the gate-speed PR (#8, `559d8489`, *the no-`cd` assertion reads a command, not a short SHA ending in `cd`*). The assertion goes through `offers_a_cd`, which counts a `cd ` only when no word character, `-`, `.` or `/` precedes it, and the pinning test feeds it the flaking shape.

Re-driven on this build (the checkout at `982f910f`). The pinning test passes, and the full gate run of this re-drive passed `validate_previews_posture::` with it.
