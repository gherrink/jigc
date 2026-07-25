# What & why

<!-- One or two sentences: what this change does and the problem it solves.
     Link the issue / DECISIONS.md entry it traces to, if any. -->

## Dev-workflow gate

All four must pass locally before merge (CI runs the same four):

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo build`
- [ ] `cargo test`

## Record

- [ ] Any decision made here is recorded in `DECISIONS.md` (dated, ≤1 line of why) — or no decision was made.
