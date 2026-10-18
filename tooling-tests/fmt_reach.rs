//! **The gate's `fmt` step reaches every Rust file of the repository** (2026-10-19 —
//! [DECISIONS.md](../DECISIONS.md) → *The gate's `fmt` step reached none of the files a
//! macro declared*; the step's home is [dev-workflow.md](../implementation/dev-workflow.md)
//! → Gate).
//!
//! `cargo fmt --check` formats a crate's root files and every module rustfmt finds from
//! them through a `mod` declaration it can read. A declaration inside a macro — in a
//! `macro_rules!` body or in the tokens of a macro's call — is no item rustfmt reads, so
//! the file it names is formatted by nothing, and the step exits 0 over any diff in it.
//! Until this fence the byte table's four family files and the route catalog's nine
//! command files were declared that way, and 160 diffs stood in the first four unseen.
//!
//! So the fence asks the step itself which files it reached — `cargo fmt --check` with
//! rustfmt's `--verbose`, which names each file it formats — and holds that set to every
//! `.rs` file under the two Rust homes, `crates/` and `tooling-tests/`. A file that only a
//! macro declares, or that nothing declares, is named. The same derivation is run first
//! on a throwaway crate that declares one module inside a macro and one outside, so the
//! fence is shown to see the shape it exists for.

use crate::support::ci_workflow::repo_root;
use crate::support::root_walk;
use crate::support::scratch::ScratchDir;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The directories that hold the repository's Rust: the workspace's crates, and the
/// tooling suites a group root under `crates/cli` compiles.
const RUST_HOMES: [&str; 2] = ["crates", "tooling-tests"];

/// **The files `cargo fmt --check` reaches from the package at `root`**, each as its
/// canonical path. A diff makes the step exit 1, which is still an answer about reach;
/// any other exit is a fault of the run, and panics with what it printed.
fn reached(root: &Path) -> BTreeSet<PathBuf> {
    let out = Command::new(env!("CARGO"))
        .args(["fmt", "--check", "--", "--verbose"])
        .current_dir(root)
        .output()
        .expect("run cargo fmt");
    let code = out.status.code();
    assert!(
        matches!(code, Some(0 | 1)),
        "`cargo fmt --check -- --verbose` in {} came to {code:?}:\n{}\n{}",
        root.display(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let reached: BTreeSet<PathBuf> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|line| line.strip_prefix("Formatting "))
        .map(|path| {
            fs::canonicalize(path.trim())
                .unwrap_or_else(|e| panic!("rustfmt named `{path}`, which is no file: {e}"))
        })
        .collect();
    assert!(
        !reached.is_empty(),
        "`cargo fmt --check -- --verbose` in {} named no file it formats — rustfmt's \
         verbose line is `Formatting <path>`; if its wording moved, teach `reached()` the \
         new one rather than leaving a fence that checks nothing",
        root.display(),
    );
    reached
}

/// **Every `.rs` file under `homes` of `root` that the step does not reach**, as a reader
/// would type it: relative to `root`.
fn unreached(root: &Path, homes: &[&str]) -> Vec<String> {
    let reached = reached(root);
    let mut out = Vec::new();
    for home in homes {
        for file in root_walk::files(&root.join(home), root_walk::ext("rs")) {
            let canonical = fs::canonicalize(&file).expect("a walked file canonicalizes");
            if !reached.contains(&canonical) {
                out.push(
                    file.strip_prefix(root)
                        .unwrap_or(&file)
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    out
}

#[test]
fn the_fmt_step_reaches_every_rust_file_and_none_a_macro_declares() {
    // The shape, on a crate of its own: one module declared inside a macro, one beside it.
    let scratch = ScratchDir::new("fmt-reach");
    let root = scratch.path();
    fs::create_dir_all(root.join("src/kept")).expect("make the crate's directories");
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"fmt-reach\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
         [workspace]\n",
    )
    .expect("write the manifest");
    fs::write(
        root.join("src/lib.rs"),
        "macro_rules! declared {\n    ($(#[path = $file:literal] mod $name:ident;)*) => {\n        \
         $(#[path = $file] mod $name;)*\n    };\n}\n\ndeclared! {\n    \
         #[path = \"kept/inside.rs\"]\n    mod inside;\n}\n\n#[path = \"kept/beside.rs\"]\n\
         mod beside;\n",
    )
    .expect("write the crate's root");
    for name in ["inside", "beside"] {
        fs::write(root.join(format!("src/kept/{name}.rs")), "pub fn f() {}\n")
            .expect("write a module");
    }
    assert_eq!(
        unreached(root, &["src"]),
        vec!["src/kept/inside.rs".to_string()],
        "a module a macro declares is the one file the step does not reach"
    );

    // The repository: every file of both homes.
    let wrong = unreached(&repo_root(), &RUST_HOMES);
    assert!(
        wrong.is_empty(),
        "`cargo fmt --check` — the gate's fmt step, and CI's — never formats these files, \
         so a diff in them passes the gate unseen:\n\n  {}\n\nrustfmt reaches a module \
         only through a `mod` declaration it can read: declare each beside the macro that \
         lists it, never inside one, and declare a file that nothing declares or remove it",
        wrong.join("\n  "),
    );
}
