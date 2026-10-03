//! The crate README crates.io shows for `jigc` is a **generated** file, and every link in
//! it lands where the root README's link lands on GitHub (M55 S15;
//! [findings-channel.md](../../../design/findings-channel.md) → 8, 10 *README fence*).
//!
//! crates.io rewrites a README's relative links against the package's `path_in_vcs`, so the
//! root README published as-is broke four of its six link targets on `1.0.0-rc.22`'s page
//! ([publish-proof.md](../../../completions/artifacts/M54/publish-proof.md) → The README as
//! crates.io renders it). [`dev/crate-readme`](../../../dev/crate-readme) writes
//! `crates/cli/README.md` from the root with each relative link made absolute at
//! `<repository>/blob/HEAD/<root path>` — the base crates.io itself uses. Two fences hold it:
//!
//! - **the byte fence**: the committed file is exactly what the script prints for today's
//!   root, so an edit to either file without a regeneration is red;
//! - **the link fence**: an independent reading of both files' links, not the script's —
//!   the crate README carries no relative link, its links are the root's in order with each
//!   relative one rewritten, and each root target exists in the tree.
//!
//! Proved red on applied mutants (DECISIONS.md → *M55 Increment 10 / T1*): one byte changed in the
//! crate README, and one byte changed in the root README's prose.

use std::fs;

use toml::Table;

use crate::support::crate_readme::{SCRIPT, crate_readme_path, generated_crate_readme, repo_root};

/// The root README, read as it stands.
fn root_readme() -> String {
    let path = repo_root().join("README.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{} must be readable: {e}", path.display()))
}

/// The committed crate README.
fn crate_readme() -> String {
    let path = crate_readme_path();
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "{} must exist and be readable — run `{SCRIPT}` to generate it: {e}",
            path.display()
        )
    })
}

/// The workspace `repository` URL, from the root `Cargo.toml`'s `[workspace.package]`.
fn repository() -> String {
    let path = repo_root().join("Cargo.toml");
    let body = fs::read_to_string(&path).expect("the root Cargo.toml is readable");
    let table: Table = body.parse().expect("the root Cargo.toml parses");
    table["workspace"]["package"]["repository"]
        .as_str()
        .expect("[workspace.package] carries a `repository` string")
        .trim_end_matches('/')
        .to_owned()
}

/// Every inline link or image target outside a fenced code block, in order: the text
/// after each `](` up to the first `)` or whitespace. A lexer, not a markdown parser — the
/// same reading the generator is held to, implemented here a second time on purpose.
fn inline_link_targets(markdown: &str) -> Vec<String> {
    let mut targets = Vec::new();
    let mut fence: Option<(char, usize)> = None;
    for line in markdown.lines() {
        let trimmed = line.trim_start_matches(' ');
        let indent = line.len() - trimmed.len();
        let run = |c: char| trimmed.chars().take_while(|&x| x == c).count();
        match fence {
            Some((c, n)) => {
                if indent <= 3 && run(c) >= n && trimmed.trim_start_matches(c).trim().is_empty() {
                    fence = None;
                }
                continue;
            }
            None if indent <= 3 && (run('`') >= 3 || run('~') >= 3) => {
                let c = if run('`') >= 3 { '`' } else { '~' };
                fence = Some((c, run(c)));
                continue;
            }
            None => {}
        }
        let mut rest = line;
        while let Some(at) = rest.find("](") {
            rest = &rest[at + 2..];
            let end = rest
                .find(|c: char| c == ')' || c.is_whitespace())
                .unwrap_or(rest.len());
            targets.push(rest[..end].to_owned());
            rest = &rest[end..];
        }
    }
    targets
}

/// A target that names a path in this repository: no scheme, no leading `/`, not only an
/// `#anchor`.
fn is_relative(target: &str) -> bool {
    let scheme = target.split_once(':').is_some_and(|(s, _)| {
        s.starts_with(|c: char| c.is_ascii_alphabetic())
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c))
    });
    !(scheme || target.starts_with('/') || target.starts_with('#'))
}

#[test]
fn the_crate_readme_is_the_generated_transform_of_the_root() {
    let committed = crate_readme();
    let generated = generated_crate_readme();
    assert!(
        committed == generated,
        "crates/cli/README.md is not the transform of the root README.md — it is generated, \
         never edited: edit the root README.md and run `{SCRIPT}` to regenerate it.\n\
         --- committed ---\n{committed}\n--- generated ---\n{generated}",
    );
}

#[test]
fn every_crate_readme_link_lands_on_its_root_target() {
    let base = format!("{}/blob/HEAD/", repository());
    let root_links = inline_link_targets(&root_readme());
    let crate_links = inline_link_targets(&crate_readme());

    let relative: Vec<&String> = root_links.iter().filter(|t| is_relative(t)).collect();
    assert!(
        !relative.is_empty(),
        "the root README.md carries no relative link, so this fence checks nothing — the \
         link reader has lost the README's links",
    );

    let stray: Vec<&String> = crate_links.iter().filter(|t| is_relative(t)).collect();
    assert!(
        stray.is_empty(),
        "crates/cli/README.md carries relative links {stray:?}: crates.io resolves them \
         against crates/cli/, not the root — regenerate with `{SCRIPT}`",
    );

    let expected: Vec<String> = root_links
        .iter()
        .map(|t| {
            if is_relative(t) {
                format!("{base}{t}")
            } else {
                t.clone()
            }
        })
        .collect();
    assert_eq!(
        crate_links, expected,
        "crates/cli/README.md's links must be the root README.md's, in order, each relative \
         one made absolute at `{base}`",
    );

    let root = repo_root();
    let missing: Vec<&str> = relative
        .iter()
        .map(|t| t.split('#').next().unwrap_or(t))
        .filter(|path| !root.join(path).exists())
        .collect();
    assert!(
        missing.is_empty(),
        "the root README.md links to {missing:?}, which do not exist in the tree — the \
         crate README sends crates.io readers to the same missing targets",
    );
}

#[test]
fn any_other_argument_is_a_usage_error() {
    let out = std::process::Command::new(repo_root().join(SCRIPT))
        .arg("--write-somewhere-else")
        .output()
        .unwrap_or_else(|e| panic!("run {SCRIPT}: {e}"));
    assert_eq!(
        out.status.code(),
        Some(2),
        "{SCRIPT} takes no argument or `--stdout`; anything else is a usage error, exit 2",
    );
}
