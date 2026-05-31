//! The `PackSource` provider trait — how a frontend supplies the pack-default
//! cascade layer to the engine, keeping the engine empty of domain content.
//!
//! MVP impl is `EmbeddedPack` (in `cli`); `FilesystemPack` is the post-MVP seam.
//! See `implementation/module-layout.md` → The dev pack's home.
