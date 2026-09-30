//! **The one install line, read out of the doc that owns it** (M54 S13;
//! [release.md](../../../../implementation/release.md) → Installing).
//!
//! `crates/cli/guides/QUICKSTART.md` owns `cargo install jigc --version '<req>' --locked`,
//! and every other home either points at it or — the root README, the one allowed copy —
//! carries it byte-identical. So every fence that speaks about the line reads it **from the
//! owner's bytes** through this one extractor, never from a literal respelled in test code:
//! a respelled literal can agree with a fence while the guide an adopter installs says
//! something else.
//!
//! The grammar the extractor holds a doc to is deliberately narrow: under the doc's
//! `## Install` heading, exactly one line inside a ```` ```sh ```` block starts
//! `cargo install`, that block carries that line and nothing else, and the line names its
//! requirement as `--version '<req>'`. A second install line, a line wrapped in a comment
//! telling the reader where to run it, or a line with no requirement is an extraction
//! failure, which is the point — each of those is a doc that no longer has *one* line.

use std::path::PathBuf;

/// The heading the install line lives under, in every doc that carries it.
pub const INSTALL_HEADING: &str = "## Install";

/// The install line as a doc states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallLine {
    /// The line, byte-for-byte as the doc carries it.
    pub line: String,
    /// The `--version` argument with its single quotes removed.
    pub requirement: String,
}

/// The guide that owns the line — the crate-local file `setup` embeds.
pub fn quickstart_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("guides/QUICKSTART.md")
}

/// The install line `QUICKSTART.md` owns.
pub fn quickstart_install_line() -> InstallLine {
    let path = quickstart_path();
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("{} must be readable: {err}", path.display()));
    install_line_in(&text, "QUICKSTART.md")
}

/// The one install line under `markdown`'s `## Install` heading. `origin` names the doc in
/// every panic, so a failure says which home stopped carrying one line.
pub fn install_line_in(markdown: &str, origin: &str) -> InstallLine {
    let section = install_section(markdown, origin);

    let mut found: Vec<Vec<&str>> = Vec::new();
    let mut block: Option<Vec<&str>> = None;
    for line in section.lines() {
        match block.as_mut() {
            None if line == "```sh" => block = Some(Vec::new()),
            None => {}
            Some(_) if line.starts_with("```") => {
                let lines = block.take().expect("inside a block");
                if lines
                    .iter()
                    .any(|l| l.trim_start().starts_with("cargo install"))
                {
                    found.push(lines);
                }
            }
            Some(lines) => lines.push(line),
        }
    }
    assert!(
        block.is_none(),
        "{origin} → {INSTALL_HEADING}: a ```sh block is never closed"
    );
    assert_eq!(
        found.len(),
        1,
        "{origin} → {INSTALL_HEADING} must carry exactly one ```sh block holding the \
         `cargo install` line; found {}: {found:?}",
        found.len(),
    );
    let lines = found.pop().expect("exactly one");
    assert_eq!(
        lines.len(),
        1,
        "{origin} → {INSTALL_HEADING}: the install block carries the one line and nothing \
         else, so the line is copied whole or not at all; it carries: {lines:?}",
    );
    let line = lines[0].to_string();

    let mut tokens = line.split_whitespace();
    let quoted = loop {
        match tokens.next() {
            Some("--version") => break tokens.next(),
            Some(_) => continue,
            None => break None,
        }
    }
    .unwrap_or_else(|| {
        panic!(
            "{origin}'s install line carries no `--version` requirement, so cargo would \
             never select a release candidate: `{line}`"
        )
    });
    let requirement = quoted
        .strip_prefix('\'')
        .and_then(|q| q.strip_suffix('\''))
        .unwrap_or_else(|| {
            panic!(
                "{origin}'s install line must single-quote its requirement, or a shell \
                 reads `^`/`<` itself: `{line}`"
            )
        })
        .to_string();
    InstallLine { line, requirement }
}

/// The text under `## Install`, up to the next `## ` heading.
fn install_section<'a>(markdown: &'a str, origin: &str) -> &'a str {
    let start = markdown
        .lines()
        .scan(0usize, |offset, line| {
            let at = *offset;
            *offset += line.len() + 1;
            Some((at, line))
        })
        .find(|(_, line)| *line == INSTALL_HEADING)
        .map(|(at, line)| at + line.len())
        .unwrap_or_else(|| panic!("{origin} must carry a `{INSTALL_HEADING}` heading"));
    let rest = &markdown[start..];
    let end = rest.find("\n## ").map_or(rest.len(), |at| at + 1);
    &rest[..end]
}
