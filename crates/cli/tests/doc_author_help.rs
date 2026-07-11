//! M41 Increment 4, T2 — `jigc doc author --help` long help carries a worked
//! payload-shape snippet naming the top-level grammar keys.
//!
//! The `author` payload grammar (`title:` + `sections:` of `{id, set, items}`,
//! each item `{title, set, sections}`) is authoritative in `author.rs` but was
//! invisible from the CLI: an agent had to read the source or fail a write to
//! learn the shape. This drives the REAL binary's long help and asserts the
//! emitted help text carries the worked snippet with its top-level keys.

use std::process::Command;

/// Run `jigc <args>` and return trimmed stdout, panicking with both streams on
/// a non-zero exit. `--help` needs no repo/pack — clap prints before dispatch.
fn help_stdout(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .output()
        .expect("spawn the jigc binary");
    assert!(
        out.status.success(),
        "`jigc {args:?}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn doc_author_help_shows_payload_grammar_keys() {
    let help = help_stdout(&["doc", "author", "--help"]);
    for key in ["title:", "sections:", "set:", "items:"] {
        assert!(
            help.contains(key),
            "`doc author --help` must show payload grammar key `{key}`; got:\n{help}"
        );
    }
}
