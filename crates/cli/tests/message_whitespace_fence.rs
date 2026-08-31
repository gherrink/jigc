//! The **message-whitespace fence** — a wrapped message literal keeps its source
//! indentation, and the fence catches it where every literal in the workspace is
//! decided rather than where someone remembered to look.
//!
//! ## The class this fences
//!
//! Rust has no implicit line-joining inside a string literal. A long message wrapped
//! across two source lines **without a trailing `\`** keeps the newline *and* the
//! continuation line's indentation in its value, so what the author read as
//!
//! ```text
//! …so whether it names task `t-9f2` as a sub-task is unknown…
//! ```
//!
//! reaches the user as fourteen literal spaces mid-sentence. It survives review because
//! the source *looks* right: the indentation is the file's own, and the defect is
//! invisible until the message is emitted. The M49 completion audit found it on the
//! blocking pack-load freeze bail — the one message a project whose every door refuses
//! has to read to get out.
//!
//! ## The domain, and the floor
//!
//! The subject is **every string literal in the workspace**, decoded to the value the
//! binary carries ([`support::rust_source::string_literals`]), because that is where
//! membership is decided ([dev-workflow.md](../../../implementation/dev-workflow.md) →
//! *a grep is not a fence*). Reading source *lines* instead is not the same check and is
//! not honest: over this workspace, with the class below already repaired, a line-level
//! scan for the same run length (`grep -rhoE '[^ ] {9,}[^ ]' --include='*.rs' crates`)
//! still reports **234 matches across 40 files** — aligned doc-comment tables, `//!`
//! prose, `\x20`-anchored fixture bodies and YAML indentation, every one of them correct
//! exactly as written.
//!
//! The predicate is a run of **[`RUN_FLOOR`] or more spaces preceded by a non-space
//! character** inside a literal's value — mid-value, so the leading indentation of a
//! fixture line (which follows a newline) is out by construction.
//!
//! **The floor is declared, not universal.** Runs of two to eight spaces are load-bearing
//! elsewhere and cannot be told apart from this defect by width alone: the frozen
//! `### Title  {#anchor}` heading grammar (two), the `` ` — `` route gutter in composed
//! output (three), the aligned columns of `jigc doc list` and the ingest report, and YAML
//! fixture bodies indented behind a `{placeholder}`. So a wrapped literal indented eight
//! columns or fewer would evade this fence. Every member of the real class sits at 14, 18
//! or 22 — the indentation a message literal actually takes inside a `format!` argument —
//! and at nine and above the workspace contains **no** legitimate run at all, which is
//! what makes the fence quiet enough to be believed.

use crate::support::rust_source::{StringLiteral, rust_files, string_literals};
use std::path::{Path, PathBuf};

/// The narrowest run of spaces that cannot be prose, alignment, or fixture indentation.
/// See the module doc for why the floor sits here and what it concedes.
const RUN_FLOOR: usize = 9;

/// The workspace root — two levels up from `crates/cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root")
}

/// The mid-value space runs of `RUN_FLOOR` or more in one literal, as `(byte offset in
/// the value, run length)`.
fn offending_runs(value: &str) -> Vec<(usize, usize)> {
    let bytes = value.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b' ' {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && bytes[i] == b' ' {
            i += 1;
        }
        let preceded_by_text = start > 0 && !bytes[start - 1].is_ascii_whitespace();
        if preceded_by_text && i - start >= RUN_FLOOR {
            out.push((start, i - start));
        }
    }
    out
}

/// The 1-based line of `offset` in `body`.
fn line_of(body: &str, offset: usize) -> usize {
    body[..offset.min(body.len())].matches('\n').count() + 1
}

/// The offenders in one file, rendered one per line.
fn offenders_in(path: &Path, body: &str) -> Vec<String> {
    let mut out = Vec::new();
    for StringLiteral { offset, value } in string_literals(body) {
        for (at, len) in offending_runs(&value) {
            let lead: Vec<char> = value[..at].chars().rev().take(38).collect();
            let lead: String = lead.into_iter().rev().collect();
            let tail: String = value[at + len..].chars().take(38).collect();
            out.push(format!(
                "{}:{} — {len} spaces after \"…{}\" and before \"{}…\"",
                path.display(),
                line_of(body, offset),
                lead.replace('\n', "\\n"),
                tail.replace('\n', "\\n"),
            ));
        }
    }
    out
}

/// **The fence.** No message literal anywhere in the workspace carries a run of spaces
/// wide enough to be a wrapped source line rather than something the author meant.
///
/// The repair is the one the compiler already offers: end the source line with `\`, which
/// eats the newline and the indentation that follows it, or close the literal and let
/// adjacent literals concatenate.
#[test]
fn no_message_literal_carries_a_wrapped_source_lines_indentation() {
    let root = workspace_root();
    let files = rust_files(&root.join("crates"));

    let mut scanned_literals = 0usize;
    let mut offenders = Vec::new();
    for path in &files {
        let Ok(body) = std::fs::read_to_string(path) else {
            continue;
        };
        scanned_literals += string_literals(&body).len();
        offenders.extend(offenders_in(path, &body));
    }

    // A green here must mean "nothing was wrong", never "nothing was measured"
    // (dev-workflow.md → the vacuous-pass family).
    assert!(
        files.len() > 200 && scanned_literals > 5_000,
        "the fence measured almost nothing: {} files, {scanned_literals} literals under {}",
        files.len(),
        root.display(),
    );

    assert!(
        offenders.is_empty(),
        "{} message literal(s) carry a wrapped source line's indentation. End the \
         source line with `\\` (it eats the newline and the indentation after it) or \
         close the literal — do not reflow the prose:\n{}",
        offenders.len(),
        offenders.join("\n"),
    );
}

/// The fence's own reading, checked against source it is handed rather than the tree it
/// sweeps — so a green above is a measurement and not a scanner that stopped seeing.
///
/// Every sample is assembled at run time from `" ".repeat(n)`, so this file does not
/// itself contain the shapes it plants and the fence above stays true of it.
#[test]
fn the_scanner_sees_the_defect_and_leaves_the_legitimate_shapes_alone() {
    let wide = " ".repeat(18);
    let plain = |s: &str| offenders_in(Path::new("sample.rs"), s).len();

    // The defect, exactly as it is written in a wrapped `format!` argument.
    let planted = format!("fn f() {{ bail!(\"a message that ran on{wide}and kept going\"); }}");
    assert_eq!(plain(&planted), 1, "the planted wrap must be seen");

    // The repair the message says to make: `\` eats the newline and the indentation.
    let repaired =
        format!("fn f() {{ bail!(\"a message that ran on \\\n{wide}and kept going\"); }}");
    assert_eq!(
        plain(&repaired),
        0,
        "a `\\`-continued literal is not an offence"
    );

    // Prose that merely *mentions* the shape, in a comment, is not an instance of it.
    let in_comment = format!("// a message that ran on{wide}and kept going\nfn f() {{}}");
    assert_eq!(
        plain(&in_comment),
        0,
        "a comment carries no message literal"
    );

    // Fixture indentation after a newline is leading, not mid-value.
    let fixture = format!("fn f() {{ let y = \"sections:\\n{wide}- id: releases\"; }}");
    assert_eq!(
        plain(&fixture),
        0,
        "indentation after a newline is not mid-value"
    );

    // A char literal holding a quote must not open a phantom string over real code.
    let quote_char = format!("fn f() {{ if c == '\"' {{ }} }}\n// tail{wide}text\nfn g() {{}}");
    assert_eq!(plain(&quote_char), 0, "`'\\\"'` does not open a string");

    // A raw literal is verbatim, so the defect is visible there too.
    let raw = format!("fn f() {{ let m = r\"ran on{wide}and kept going\"; }}");
    assert_eq!(plain(&raw), 1, "a raw literal is read verbatim");

    // The floor is a floor: one space below it is left alone, exactly at it is caught.
    let below = " ".repeat(RUN_FLOOR - 1);
    let at = " ".repeat(RUN_FLOOR);
    assert_eq!(plain(&format!("fn f() {{ let m = \"a{below}b\"; }}")), 0);
    assert_eq!(plain(&format!("fn f() {{ let m = \"a{at}b\"; }}")), 1);
}
