//! `EmbeddedPack` — the MVP `PackSource` impl that serves the built-in dev pack
//! from bytes embedded in the `jigc` binary.
//!
//! Embed mechanism (`rust-embed` vs `include_dir` vs build-script) is an open
//! decision, settled in the increment that loads real pack data. See
//! `implementation/module-layout.md` → The dev pack's home.
