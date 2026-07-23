//! Shared test support for the `cli` integration suites — the repo's **first**
//! shared test module ([pinning.md](../../../../implementation/pinning.md) §4).
//!
//! Until M45 every one of the 211 integration suites re-implemented its own
//! isolation preamble inline, and the adoption was uneven: 207/211 used the
//! pid+nanos tempdir, 199/211 set `$HOME`, but only **38/211** scrubbed
//! `JIGC_PACK_DIR` — and a suite that inherits a developer's `JIGC_PACK_DIR`
//! reads a different pack than the one it claims to sweep. This module is the
//! **strict** version of that pattern, in one place.
//!
//! Files under `tests/support/` are not cargo test targets, so a consumer suite
//! reaches this module with `mod support;`. A helper that some consumer does not
//! call would otherwise trip `dead_code` under `clippy --all-targets -D warnings`,
//! hence the crate-wide allow below — it is a property of the *shared-module*
//! shape, not a licence for dead code in the suites themselves.
#![allow(dead_code)]

pub mod goldens;
pub mod trial_corpus;
