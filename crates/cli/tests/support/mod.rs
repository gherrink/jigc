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

pub mod committing_doors;
pub mod frozen_pack;
pub mod goldens;
pub mod leaf_argv;
pub mod rust_source;
pub mod shape_space;
pub mod trial_corpus;
pub mod write_miss_cells;

/// Seed the **ambush-class declarer** a hand-built fixture pack owes since M51
/// Increment 8 T2, as one step named `finalize-contracts`.
///
/// The stated-at fence's structural tier keys on the constituents that **ship
/// steps**, not the ones that ship a manifest (`design/surface-contract.md` → The
/// stated-at fence; `cli::pack::step_shipping_constituents`), so a seeded pack with
/// one step of its own owes the four statements too — its `creates-task: true`
/// workflow mints a task that is finalized through `jigc task finalize`, where all
/// four contracts bind, whatever the pack's own steps happen to name.
///
/// **The body states the contracts; it is not a bare receipt.** The named-fact tier
/// (`cli::pack::CONSTRAINT_REQUIRED_TOKENS`) checks only manifest-shipping packs, so
/// nothing would catch a fixture that declared the codes over silence — which is
/// exactly the shape that tier exists to kill, and seeding it into every fixture pack
/// would teach it by example. The prose below is the shipped `finalize` /
/// `migration-finalize` statement, condensed and unchanged in substance.
///
/// Call it from a fixture pack's own seeder, beside the steps under test.
pub fn seed_ambush_class_declarer(pack: &std::path::Path) {
    let steps = pack.join("steps");
    std::fs::create_dir_all(&steps).expect("mk the fixture pack's steps/");
    std::fs::write(
        steps.join("finalize-contracts.yaml"),
        "---\n\
         states-constraints: [finalize.left-out, finalize.nothing-staged, \
         finalize.carried-staged, finalize.promote-clobber]\n\
         ---\n\
         Finalize commits only the staged set plus the docs it manages; unstaged\n\
         edits and untracked files are left out, and with nothing staged over a\n\
         dirty tree it refuses. Anything still staged from BEFORE this task was\n\
         minted makes finalize refuse too (one blocking finding per carried path):\n\
         unstage it, or pass `--carry-staged` to declare the carryover deliberate.\n\
         \n\
         Promoting a created doc blocks (finalize.promote-clobber) when a file\n\
         appeared at its destination after the doc was created: resolve that\n\
         collision, or retitle the doc so it slugs differently, then finalize again.\n\
         A migration finalize holds for review first — a plain finalize renders the\n\
         fidelity diff and commits nothing; re-run it with `--approve` to write the\n\
         canonical doc and retire the foreign original.\n",
    )
    .expect("seed the fixture pack's ambush-class declarer");
}

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
