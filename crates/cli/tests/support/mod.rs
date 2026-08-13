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

/// The `--title` a `jigc doc create <doctype>` must carry against the **shipped** packs.
///
/// A `placement` / `display-title` singleton's `# H1` is the schema's own, so since M48 a
/// divergent `--title` is **refused** rather than silently dropped (`write.title-ignored`;
/// `design/write-commands.md` → The four-way write). A sweep that mints many doctypes
/// under one label therefore has to ask the schema rather than assume — and asking keeps
/// the sweep correct when a doctype joins either pack. Every doctype whose title is the
/// author's returns `otherwise`, unchanged.
pub fn create_title(doctype: &str, otherwise: &str) -> String {
    use engine::packsource::{PackResourceKind, PackSource, ResourceId};
    let pack = cli::pack::CompositePack::new(vec![
        Box::new(cli::pack::EmbeddedPack::new()),
        Box::new(cli::pack::EmbeddedPack::methodology()),
    ]);
    pack.read(PackResourceKind::Schemas, &ResourceId::from(doctype))
        .ok()
        .and_then(|bytes| cli::pack::load_pack_schema(&pack, &bytes).ok())
        .and_then(|schema| schema.fixed_title())
        .unwrap_or_else(|| otherwise.to_string())
}

/// Split an emitted command into argv the way a shell would — honouring the double
/// quotes a route puts around a multi-word value (a title, an intent), so a suite that
/// runs a route **verbatim** runs the *emitted bytes* rather than a whitespace-split
/// approximation of them (`design/surface-contract.md` → law 2: a route is followable).
pub fn shell_split(cmd: &str) -> Vec<String> {
    let mut argv = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut started = false;
    for ch in cmd.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
                started = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if started {
                    argv.push(std::mem::take(&mut current));
                    started = false;
                }
            }
            c => {
                current.push(c);
                started = true;
            }
        }
    }
    if started {
        argv.push(current);
    }
    argv
}
