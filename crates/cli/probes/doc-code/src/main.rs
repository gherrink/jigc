//! **Transitional shim, for one commit (M54 Inc 2 T4 → deleted at T5).** The `doc-code`
//! probe's sources now live in the `jigc` bin's module tree
//! (`crates/cli/src/doc_code_probe/`), and `jigc` runs them by self-exec. This detached
//! workspace compiles the same module by `#[path]`, so `build.rs`, `setup`'s extract and
//! `doc_code_probe_suite` stay green until T5 retires all four.

#[path = "../../../src/doc_code_probe/mod.rs"]
mod doc_code_probe;

fn main() -> std::process::ExitCode {
    doc_code_probe::run()
}
