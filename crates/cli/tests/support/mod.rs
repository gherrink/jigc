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

pub mod frozen_pack;
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

/// Split an emitted command line into argv **through a real shell**, so a suite that
/// claims to run a route "verbatim" runs the bytes an agent would paste
/// (`design/surface-contract.md` → law 2: a route is followable).
///
/// **The splitter is `sh` itself, and that is the whole point.** The hand-rolled
/// splitters this replaces understood one quoting form and performed no expansion, so
/// every emitted `$`, backtick or `;` was inert *in test* while live in a terminal — a
/// route quoted the pre-M47 way passed the suite and, run for real, renamed a document to
/// something nobody authored (M48 inc-2 triage). `set --` is the shell's own word
/// splitter: whatever `sh` makes of the emitted bytes — expansions, command substitutions
/// and all — is exactly what the caller then executes, so a route that only *looks*
/// followable reddens here.
///
/// `cwd` / `home` are the corpus's, so an expansion that does leak is the corpus's own
/// `$HOME` and a command substitution's side effect lands inside the throwaway repo where
/// the caller can assert on it — never in the developer's tree.
pub fn shell_words(cmd: &str, cwd: &std::path::Path, home: &std::path::Path) -> Vec<String> {
    let script = format!("set -- {cmd}\nfor w in \"$@\"; do printf '%s\\0' \"$w\"; done");
    let out = std::process::Command::new("sh")
        .arg("-c")
        .arg(&script)
        .current_dir(cwd)
        .env("HOME", home)
        .output()
        .expect("spawn sh");
    assert!(
        out.status.success(),
        "the emitted command line must parse as shell words; got `{cmd}`\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let mut words: Vec<String> = out
        .stdout
        .split(|b| *b == 0)
        .map(|w| String::from_utf8_lossy(w).into_owned())
        .collect();
    // The trailing NUL of the last word yields one empty tail element — an *emitted*
    // empty word keeps its own element.
    words.pop();
    words
}
