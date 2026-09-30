//! The **link fence** — every link and every repository path a live doc names resolves
//! (M54 S15, completed by the human's O1 answer; [DECISIONS.md](../../../DECISIONS.md) →
//! *M54 settled* → S15).
//!
//! Nothing fenced a doc link before M54, and M54 moves every path the census found cited:
//! the packs into `crates/cli/packs/`, the guides into `crates/cli/guides/`, the probe into
//! the binary. A doc that still says `packs/methodology/…` after that is a doc an agent
//! follows to nothing. So this fence reads a **declared** list of live docs and fails on
//! any link or path token in them that does not resolve in the working tree. Offline: it
//! reads files and nothing else.
//!
//! **The docs.** [`LIVE_DOCS`] names every live doc; [`EXCLUDED`] names the dated records,
//! each with its reason. Neither is derived from the tree, and
//! [`every_markdown_file_under_the_homes_is_listed_or_excluded`] walks the [`HOMES`] so a
//! new doc cannot fall outside the fence without someone deciding it should.
//!
//! **What is checked.**
//!
//! - Every **relative markdown link** `[label](target)`, resolved from the doc's own
//!   directory. A target with a scheme, or one that is only an `#anchor`, is not relative.
//!   A target that climbs out of the repository is an offence: it resolves on one machine
//!   and not on the next.
//! - Every **path token**: the content of an inline code span, or the plain text of a link
//!   label, whose first word starts with a declared top-level directory
//!   ([`TOP_LEVEL_DIRS`]) or is a declared root file ([`ROOT_FILES`]). A token is
//!   repository-root-relative. Both lists are declared, never read off the tree, so a
//!   directory that moved away (`packs/`) still makes a stale mention a token.
//!
//! **The grammar of a token** (S15 and O1):
//!
//! - a `:line`, `#anchor` or `::symbol` suffix is stripped (everything from the first `:`
//!   or `#`);
//! - a `{a,b}` brace form expands into each path, and each must resolve;
//! - a token carrying a `<…>` placeholder is not a path;
//! - a `*` segment is a glob, and resolves when it matches at least one entry;
//! - text inside `~~…~~`, `[Superseded …]` or `[Corrected …]` is exempt — that is where a
//!   doc keeps the history of a path it no longer claims.
//!
//! Two arms settle the non-path tokens the first sweep's sizing found, each recorded in
//! DECISIONS.md → *M54 Inc 3 T9*: `dev/<semver>` is the `<pack-id>/<version>` notation
//! `jigc` prints (`Pack input: dev/1.0.0-rc.21`), not a path under `dev/`; and
//! `.claude/skills/` is the tree `jigc setup` installs into an **adopter's** repository —
//! S15's adopter-path class, beside `.jigc/…` and `docs/…` — which this repository does
//! not track ([`the_adopter_install_tree_is_not_tracked_here`] keeps that arm honest).
//!
//! **The pending list** (O1). A doc may name a path a later increment of this milestone
//! creates. [`PENDING`] lists each, with the increment, and each is asserted **absent**
//! ([`every_pending_path_is_still_absent`]), so the increment that creates it must also
//! delete its entry. The list must be empty at M54's close (Increment 12). A path no
//! increment creates is a defect to fix, never a pending entry.
//!
//! **Declared bounds.** The scan is a lexer, not a markdown parser. A code span is read on
//! one line: a backtick run pairs with the next run of the same length on that line, so a
//! fence line (```` ``` ````) pairs with nothing and the backticked tokens *inside* a
//! fenced block are read like any other. A code span or a link broken across two lines is
//! not read. The fence reads paths, never line numbers: `pack.rs:1824` resolves when
//! `pack.rs` does, whatever line 1824 now holds.

use std::fs;
use std::path::{Path, PathBuf};

use crate::support::root_walk;

/// Every live doc the fence reads, repository-relative. Declared, never derived.
const LIVE_DOCS: &[&str] = &[
    "CLAUDE.md",
    "VISION.md",
    "WHY-JIGC.md",
    "crates/cli/guides/MIGRATING.md",
    "crates/cli/guides/QUICKSTART.md",
    "design/architecture-documentation.md",
    "design/assistant-adapter.md",
    "design/auto-migration.md",
    "design/bootstrap.md",
    "design/changelog.md",
    "design/command-catalog.md",
    "design/command-output-contract.md",
    "design/corpus-migration.md",
    "design/design-altitude-doctypes.md",
    "design/doc-read-surface.md",
    "design/document-type-schema.md",
    "design/finalize.md",
    "design/introspection.md",
    "design/measurement.md",
    "design/methodology-docs.md",
    "design/multi-pack.md",
    "design/overrides.md",
    "design/project-setup.md",
    "design/reconciliation.md",
    "design/self-hosting.md",
    "design/storage.md",
    "design/structural-grammar.md",
    "design/surface-contract.md",
    "design/team-ready-state.md",
    "design/validation.md",
    "design/worked-examples.md",
    "design/workflow-dialect.md",
    "design/write-commands.md",
    "implementation/decisions-pending.md",
    "implementation/design-workflow.md",
    "implementation/dev-workflow.md",
    "implementation/doctype-authoring.md",
    "implementation/doctype-map.md",
    "implementation/dogfood/README.md",
    "implementation/increment-workflow.md",
    "implementation/language-runtime.md",
    "implementation/machine-setup.md",
    "implementation/milestone-completion-workflow.md",
    "implementation/milestone-planning-workflow.md",
    "implementation/module-layout.md",
    "implementation/parsing.md",
    "implementation/pinning.md",
    "implementation/public-hygiene.md",
    "implementation/release.md",
    "ideas/adapter-permission-model.md",
    "ideas/adr-lifecycle-extensions.md",
    "ideas/assumption-register.md",
    "ideas/batch-authoring-ergonomics.md",
    "ideas/brownfield-baseline-capture.md",
    "ideas/bulk-onboarding-flow.md",
    "ideas/cascade-profile-knobs.md",
    "ideas/cli-owned-rename.md",
    "ideas/commit-scope-vocabulary.md",
    "ideas/composed-context-token-budget.md",
    "ideas/compounding-lessons-grounding.md",
    "ideas/conflict-free-parallel-writing.md",
    "ideas/cost-of-enforcement.md",
    "ideas/derived-doc-staleness.md",
    "ideas/differentiator-pilot.md",
    "ideas/doc-read-surface.md",
    "ideas/doc-search.md",
    "ideas/finding-doctype.md",
    "ideas/flow-doctype.md",
    "ideas/form-vision-research-routing.md",
    "ideas/glossary-term-injection.md",
    "ideas/issue-tracker-integration.md",
    "ideas/managed-doc-enforcement-hook.md",
    "ideas/methodology-kb-pack.md",
    "ideas/migration-content-coverage.md",
    "ideas/monorepo-submodule-support.md",
    "ideas/multi-harness-adapter-bridge.md",
    "ideas/multi-language-doc-code.md",
    "ideas/output-language-directives.md",
    "ideas/pack-doctype-visibility.md",
    "ideas/postmortem-and-runbook-doctypes.md",
    "ideas/reference-doctype.md",
    "ideas/root-changelog-render.md",
    "ideas/scheduled-staleness-sweep.md",
    "ideas/sequential-milestone.md",
    "ideas/slug-minting-ergonomics.md",
    "ideas/spec-criterion-status.md",
    "ideas/spec-open-decisions.md",
    "ideas/spec-router-matching.md",
    "ideas/state-aware-compose.md",
    "ideas/sticky-task-context.md",
    "ideas/symbol-mention-sweep.md",
    "ideas/task-record-graduation.md",
    "ideas/team-ready-state-externalization.md",
];

/// The markdown files under the [`HOMES`] the fence does not read, each with its reason.
const EXCLUDED: &[(&str, &str)] = &[
    (
        "DECISIONS.md",
        "a dated record: each entry states what was true on its date, and rewriting an old \
         path in it would falsify the log rather than repair a claim",
    ),
    (
        "implementation/project-history.md",
        "a dated record: one fold-back per milestone, as it stood when that milestone closed",
    ),
    (
        "implementation/roadmap.md",
        "excluded whole (O1): its plans name paths ahead of the increments that create them, \
         and its closed milestones are dated records",
    ),
];

/// Where live docs live. The root is read one level deep; the others recursively.
/// `completions/` is not a home: it is dated records throughout.
const HOMES: &[&str] = &["", "crates/cli/guides", "design", "implementation", "ideas"];

/// A token whose first word starts with one of these is a repository path. Declared, not
/// read off the tree: `packs/` stays although M54 moved the methodology pack out of it,
/// so a stale `packs/methodology/…` is still read and still fails.
const TOP_LEVEL_DIRS: &[&str] = &[
    ".claude/",
    ".config/",
    ".github/",
    "completions/",
    "crates/",
    "design/",
    "dev/",
    "ideas/",
    "implementation/",
    "packs/",
];

/// A token that **is** one of these names a root file: the files this repository keeps at
/// its root. The guides left the list when M54 moved them into `crates/cli/guides/`. A
/// bare `QUICKSTART.md` is the guide's *name*, not a root path: the guides name each other
/// that way in the bytes `jigc setup` installs into an adopter's repository, where no
/// `crates/cli/guides/` exists. A stale root *link* to a guide is still a link, and fails.
const ROOT_FILES: &[&str] = &[
    ".gitignore",
    ".gitleaks.toml",
    "CLAUDE.md",
    "Cargo.lock",
    "Cargo.toml",
    "DECISIONS.md",
    "LICENSE-APACHE",
    "LICENSE-MIT",
    "VISION.md",
    "WHY-JIGC.md",
    "rust-toolchain.toml",
    "rustfmt.toml",
];

/// Paths a later increment of M54 creates, each with that increment. Each must be absent
/// today; the increment that creates one deletes its entry. Empty at M54's close.
const PENDING: &[(&str, &str)] = &[(
    "dev/runner-faithful",
    "Increment 7, *a clean machine installs it* — the runner-shaped container that installs \
     the packaged crates (S10)",
)];

/// The tree `jigc setup` installs into an adopter's repository (S15's adopter-path class).
const ADOPTER_INSTALL_TREE: &str = ".claude/skills/";

/// The exempt bracket forms (S15). Each opens with one of these and closes at its matching
/// `]`.
const EXEMPT_BRACKETS: &[&str] = &["[Superseded", "[Corrected"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// The tree a token resolves against
// ---------------------------------------------------------------------------

/// What a token or link resolves against: the working tree, or a fixture set in the
/// grammar arms.
trait Tree {
    /// `rel` (repository-relative, `/`-separated, no trailing `/`) names a file or a
    /// directory.
    fn exists(&self, rel: &str) -> bool;
    /// The entries directly under the directory `rel` (`""` is the root).
    fn children(&self, rel: &str) -> Vec<String>;
}

struct WorkingTree(PathBuf);

impl Tree for WorkingTree {
    fn exists(&self, rel: &str) -> bool {
        self.0.join(rel).exists()
    }
    fn children(&self, rel: &str) -> Vec<String> {
        fs::read_dir(self.0.join(rel))
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// `*` matches any run of characters inside one path segment.
fn wildcard(pattern: &str, name: &str) -> bool {
    match pattern.split_once('*') {
        None => pattern == name,
        Some((head, tail)) => {
            let Some(rest) = name.strip_prefix(head) else {
                return false;
            };
            (0..=rest.len())
                .filter(|&i| rest.is_char_boundary(i))
                .any(|i| wildcard(tail, &rest[i..]))
        }
    }
}

/// A path with a `*` segment resolves when it matches at least one entry.
fn glob_resolves(tree: &dyn Tree, path: &str) -> bool {
    let mut current = vec![String::new()];
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        let join = |dir: &str, name: &str| {
            if dir.is_empty() {
                name.to_string()
            } else {
                format!("{dir}/{name}")
            }
        };
        current = if segment.contains('*') {
            current
                .iter()
                .flat_map(|dir| {
                    tree.children(dir)
                        .into_iter()
                        .filter(|name| wildcard(segment, name))
                        .map(|name| join(dir, &name))
                        .collect::<Vec<_>>()
                })
                .collect()
        } else {
            current
                .iter()
                .map(|dir| join(dir, segment))
                .filter(|rel| tree.exists(rel))
                .collect()
        };
    }
    !current.is_empty()
}

fn resolves(tree: &dyn Tree, path: &str) -> bool {
    let path = path.trim_end_matches('/');
    if path.contains('*') {
        glob_resolves(tree, path)
    } else {
        tree.exists(path)
    }
}

// ---------------------------------------------------------------------------
// The scan
// ---------------------------------------------------------------------------

/// One link or token that does not resolve.
#[derive(Debug, PartialEq)]
struct Offence {
    line: usize,
    what: String,
    why: &'static str,
}

#[derive(Default)]
struct Scan {
    offences: Vec<Offence>,
    links: usize,
    tokens: usize,
}

/// `text` with every exempt region blanked to spaces (newlines kept, so line numbers
/// hold), plus an offence for an exempt form that never closes inside its paragraph —
/// an unclosed form would otherwise exempt the rest of the doc.
fn mask_exempt(text: &str) -> (String, Vec<Offence>) {
    let mut out: Vec<char> = text.chars().collect();
    let mut offences = Vec::new();
    let line_of =
        |chars: &[char], at: usize| chars[..at].iter().filter(|&&c| c == '\n').count() + 1;
    let blank = |out: &mut Vec<char>, from: usize, to: usize| {
        for c in &mut out[from..to] {
            if *c != '\n' {
                *c = ' ';
            }
        }
    };
    let chars: Vec<char> = text.chars().collect();
    let starts_at = |at: usize, needle: &str| {
        needle
            .chars()
            .enumerate()
            .all(|(k, n)| chars.get(at + k) == Some(&n))
    };
    let ends_paragraph = |at: usize| chars[at] == '\n' && chars.get(at + 1) == Some(&'\n');

    // The bracket forms: balanced `[`…`]` from the opener.
    let mut i = 0;
    while i < chars.len() {
        if !EXEMPT_BRACKETS.iter().any(|b| starts_at(i, b)) {
            i += 1;
            continue;
        }
        let mut depth = 0usize;
        let mut end = None;
        for (j, &c) in chars.iter().enumerate().skip(i) {
            if ends_paragraph(j) {
                break;
            }
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
                if depth == 0 {
                    end = Some(j + 1);
                    break;
                }
            }
        }
        match end {
            Some(end) => {
                blank(&mut out, i, end);
                i = end;
            }
            None => {
                offences.push(Offence {
                    line: line_of(&chars, i),
                    what: chars[i..(i + 12).min(chars.len())].iter().collect(),
                    why: "an exempt bracket that does not close inside its paragraph",
                });
                i += 1;
            }
        }
    }

    // `~~…~~`, paired in order over what the brackets left.
    let masked: Vec<char> = out.clone();
    let mut open: Option<usize> = None;
    let mut j = 0;
    while j + 1 < masked.len() {
        if masked[j] == '~' && masked[j + 1] == '~' {
            match open.take() {
                None => open = Some(j),
                Some(from) => blank(&mut out, from, j + 2),
            }
            j += 2;
            continue;
        }
        if let Some(from) = open
            && ends_paragraph(j)
        {
            offences.push(Offence {
                line: line_of(&chars, from),
                what: "~~".to_string(),
                why: "a `~~` strike that does not close inside its paragraph",
            });
            open = None;
        }
        j += 1;
    }
    if let Some(from) = open {
        offences.push(Offence {
            line: line_of(&chars, from),
            what: "~~".to_string(),
            why: "a `~~` strike that does not close inside its paragraph",
        });
    }
    (out.into_iter().collect(), offences)
}

/// `{a,b}` expands into each path; several brace groups expand as a product.
fn expand_braces(token: &str) -> Vec<String> {
    let Some(open) = token.find('{') else {
        return vec![token.to_string()];
    };
    let Some(close) = token[open..].find('}').map(|k| open + k) else {
        return vec![token.to_string()];
    };
    token[open + 1..close]
        .split(',')
        .flat_map(|alt| expand_braces(&format!("{}{alt}{}", &token[..open], &token[close + 1..])))
        .collect()
}

/// `true` when `rest` is a semantic version — the `dev/<semver>` pack-version notation.
fn is_semver(rest: &str) -> bool {
    let core = rest.split_once('-').map_or(rest, |(core, _)| core);
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

enum Token {
    /// Not a repository path (not under a declared root, a placeholder, or an arm).
    NotAPath,
    /// A repository path, before brace expansion.
    Path(String),
}

/// Classify the first word of a code span or a plain link label.
fn classify(raw: &str) -> Token {
    let Some(word) = raw.split_whitespace().next() else {
        return Token::NotAPath;
    };
    let path = word
        .split([':', '#'])
        .next()
        .unwrap_or_default()
        .trim_end_matches([',', ';', ')']);
    let declared =
        TOP_LEVEL_DIRS.iter().any(|dir| path.starts_with(dir)) || ROOT_FILES.contains(&path);
    if !declared || (word.contains('<') && word.contains('>')) {
        return Token::NotAPath;
    }
    if path.strip_prefix("dev/").is_some_and(is_semver) || path.starts_with(ADOPTER_INSTALL_TREE) {
        return Token::NotAPath;
    }
    Token::Path(path.to_string())
}

/// Lexically resolve `target` from the directory `doc_dir`; `None` when it climbs out of
/// the repository.
fn resolve_relative(doc_dir: &str, target: &str) -> Option<String> {
    let mut parts: Vec<&str> = doc_dir.split('/').filter(|s| !s.is_empty()).collect();
    for segment in target.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

fn pending(path: &str) -> bool {
    let path = path.trim_end_matches('/');
    PENDING.iter().any(|(p, _)| *p == path)
}

/// Every link and path token in one doc that does not resolve against `tree`.
fn scan(doc: &str, text: &str, tree: &dyn Tree) -> Scan {
    let (masked, mut offences) = mask_exempt(text);
    let mut scan = Scan::default();
    let doc_dir = doc.rsplit_once('/').map_or("", |(dir, _)| dir);

    let check_token = |scan: &mut Scan, offences: &mut Vec<Offence>, line: usize, raw: &str| {
        let Token::Path(path) = classify(raw) else {
            return;
        };
        scan.tokens += 1;
        for each in expand_braces(&path) {
            if !pending(&each) && !resolves(tree, &each) {
                offences.push(Offence {
                    line,
                    what: format!("`{}`", raw.trim()),
                    why: "a repository path that does not resolve",
                });
            }
        }
    };

    for (index, line) in masked.lines().enumerate() {
        let line_no = index + 1;
        let chars: Vec<char> = line.chars().collect();

        // Code spans: a backtick run pairs with the next run of the same length.
        let mut i = 0;
        while i < chars.len() {
            if chars[i] != '`' {
                i += 1;
                continue;
            }
            let run = chars[i..].iter().take_while(|&&c| c == '`').count();
            let body_from = i + run;
            let mut k = body_from;
            let mut close = None;
            while k < chars.len() {
                if chars[k] == '`' {
                    let other = chars[k..].iter().take_while(|&&c| c == '`').count();
                    if other == run {
                        close = Some(k);
                        break;
                    }
                    k += other;
                } else {
                    k += 1;
                }
            }
            match close {
                Some(close) => {
                    let body: String = chars[body_from..close].iter().collect();
                    check_token(&mut scan, &mut offences, line_no, &body);
                    i = close + run;
                }
                None => i = body_from,
            }
        }

        // Links: `[label](target)`.
        let mut from = 0;
        while let Some(at) = line[from..].find("](").map(|k| from + k) {
            from = at + 2;
            let Some(len) = line[from..].find(')') else {
                continue;
            };
            let target = line[from..from + len]
                .split_whitespace()
                .next()
                .unwrap_or_default();
            // The label: back to the matching `[`.
            let head = &line[..at];
            let mut depth = 0usize;
            let label_start = head.char_indices().rev().find_map(|(k, c)| match c {
                ']' => {
                    depth += 1;
                    None
                }
                '[' if depth == 0 => Some(k + 1),
                '[' => {
                    depth -= 1;
                    None
                }
                _ => None,
            });
            if let Some(start) = label_start {
                let label = head[start..].trim_matches(['*', '_', ' ']);
                if !label.contains('`') {
                    check_token(&mut scan, &mut offences, line_no, label);
                }
            }
            if target.is_empty()
                || target.starts_with('#')
                || target.contains("://")
                || target.starts_with("mailto:")
            {
                continue;
            }
            let path = target.split('#').next().unwrap_or_default();
            scan.links += 1;
            match resolve_relative(doc_dir, path) {
                None => offences.push(Offence {
                    line: line_no,
                    what: format!("({target})"),
                    why: "a relative link that climbs out of the repository",
                }),
                Some(rel) if !pending(&rel) && !resolves(tree, &rel) => {
                    offences.push(Offence {
                        line: line_no,
                        what: format!("({target})"),
                        why: "a relative link that does not resolve",
                    });
                }
                Some(_) => {}
            }
        }
    }
    scan.offences = offences;
    scan
}

// ---------------------------------------------------------------------------
// The fence over the tree
// ---------------------------------------------------------------------------

#[test]
fn every_markdown_file_under_the_homes_is_listed_or_excluded() {
    let root = repo_root();
    let mut found = Vec::new();
    for home in HOMES {
        let md = root_walk::ext("md");
        let files = if home.is_empty() {
            root_walk::files_in(&root, md)
        } else {
            root_walk::files(&root.join(home), md)
        };
        found.extend(files.into_iter().map(|path| {
            path.strip_prefix(&root)
                .expect("a walked path sits under the repo root")
                .to_string_lossy()
                .replace('\\', "/")
        }));
    }
    let excluded: Vec<&str> = EXCLUDED.iter().map(|(doc, _)| *doc).collect();
    let undecided: Vec<&String> = found
        .iter()
        .filter(|doc| !LIVE_DOCS.contains(&doc.as_str()) && !excluded.contains(&doc.as_str()))
        .collect();
    assert!(
        undecided.is_empty(),
        "a markdown file under the live-doc homes is neither in LIVE_DOCS nor in EXCLUDED \
         with a reason — decide which: {undecided:#?}",
    );
    let missing: Vec<&str> = LIVE_DOCS
        .iter()
        .chain(excluded.iter())
        .copied()
        .filter(|doc| !found.iter().any(|f| f == doc))
        .collect();
    assert!(
        missing.is_empty(),
        "a listed or excluded doc is not under the homes any more — a list entry that names \
         nothing is a stale one: {missing:#?}",
    );
}

#[test]
fn every_link_and_path_token_in_a_live_doc_resolves() {
    let root = repo_root();
    let tree = WorkingTree(root.clone());
    let mut offences = Vec::new();
    let (mut links, mut tokens) = (0usize, 0usize);
    for doc in LIVE_DOCS {
        let text = fs::read_to_string(root.join(doc))
            .unwrap_or_else(|e| panic!("live doc `{doc}` must be readable: {e}"));
        let scan = scan(doc, &text, &tree);
        links += scan.links;
        tokens += scan.tokens;
        offences.extend(
            scan.offences
                .into_iter()
                .map(|o| format!("{doc}:{} {} — {}", o.line, o.what, o.why)),
        );
    }
    assert!(
        links > 1000 && tokens > 1000,
        "the scan read {links} links and {tokens} path tokens — it regressed and would pass \
         having checked nothing",
    );
    assert!(
        offences.is_empty(),
        "{} link(s) or path token(s) in the live docs do not resolve. Fix the path, or, when \
         the doc keeps it as history, move it inside `~~…~~`, `[Superseded …]` or \
         `[Corrected …]`:\n{}",
        offences.len(),
        offences.join("\n"),
    );
}

#[test]
fn every_pending_path_is_still_absent() {
    let root = repo_root();
    for (path, increment) in PENDING {
        assert!(
            !root.join(path).exists(),
            "`{path}` is on the link fence's pending list ({increment}) but now exists — the \
             increment that created it deletes its PENDING entry",
        );
    }
}

#[test]
fn the_adopter_install_tree_is_not_tracked_here() {
    assert!(
        !repo_root().join(ADOPTER_INSTALL_TREE).exists(),
        "`{ADOPTER_INSTALL_TREE}` exists in this repository, so the arm that reads it as an \
         adopter's install tree would now hide paths this repository tracks — drop the arm",
    );
}

// ---------------------------------------------------------------------------
// The grammar, on fixtures
// ---------------------------------------------------------------------------

/// A fixture tree: the listed files, and every directory above them.
struct Fixture(&'static [&'static str]);

impl Tree for Fixture {
    fn exists(&self, rel: &str) -> bool {
        self.0
            .iter()
            .any(|f| *f == rel || f.starts_with(&format!("{rel}/")))
    }
    fn children(&self, rel: &str) -> Vec<String> {
        let prefix = if rel.is_empty() {
            String::new()
        } else {
            format!("{rel}/")
        };
        let mut out: Vec<String> = self
            .0
            .iter()
            .filter_map(|f| f.strip_prefix(prefix.as_str()))
            .map(|rest| rest.split('/').next().unwrap_or_default().to_string())
            .collect();
        out.dedup();
        out
    }
}

const FIXTURE: Fixture = Fixture(&[
    "design/storage.md",
    "design/validation.md",
    "crates/cli/packs/dev/config/knobs.yaml",
    "crates/cli/packs/methodology/config/knobs.yaml",
    "crates/cli/src/pack.rs",
    "CLAUDE.md",
]);

fn offences(doc: &str, text: &str) -> Vec<String> {
    scan(doc, text, &FIXTURE)
        .offences
        .into_iter()
        .map(|o| o.what)
        .collect()
}

#[test]
fn a_broken_relative_link_and_a_stale_token_are_offences() {
    let doc =
        "see [storage](storage.md), [gone](gone.md) and `packs/methodology/config/knobs.yaml`";
    assert_eq!(
        offences("design/x.md", doc),
        ["`packs/methodology/config/knobs.yaml`", "(gone.md)"],
    );
}

#[test]
fn a_link_resolves_from_its_own_directory_and_may_not_climb_out() {
    assert!(offences("design/x.md", "[v](../design/validation.md)").is_empty());
    assert_eq!(
        offences("design/x.md", "[v](design/validation.md)"),
        ["(design/validation.md)"]
    );
    assert_eq!(
        offences("design/x.md", "[p](../../PRINCIPLES.md)"),
        ["(../../PRINCIPLES.md)"]
    );
}

#[test]
fn suffixes_are_stripped_and_anchors_and_schemes_are_not_paths() {
    let doc = "`crates/cli/src/pack.rs:1824` `crates/cli/src/pack.rs::seam` \
               `design/storage.md#placement` [a](#top) [b](https://x.y/z) [c](storage.md#p)";
    assert!(offences("design/x.md", doc).is_empty());
}

#[test]
fn a_brace_form_expands_and_each_path_must_resolve() {
    assert!(
        offences(
            "x.md",
            "`crates/cli/packs/{dev,methodology}/config/knobs.yaml`"
        )
        .is_empty()
    );
    assert_eq!(
        offences("x.md", "`crates/cli/packs/{dev,gone}/config/knobs.yaml`"),
        ["`crates/cli/packs/{dev,gone}/config/knobs.yaml`"],
    );
}

#[test]
fn a_placeholder_token_is_not_a_path() {
    assert!(offences("x.md", "`crates/<dir>/CHANGELOG.md`").is_empty());
}

#[test]
fn a_glob_resolves_when_it_matches() {
    assert!(
        offences(
            "x.md",
            "`crates/cli/packs/*/config/knobs.yaml` `design/*.md`"
        )
        .is_empty()
    );
    assert_eq!(offences("x.md", "`design/*.yaml`"), ["`design/*.yaml`"]);
}

#[test]
fn text_in_the_exempt_forms_is_not_read() {
    let doc = "~~`packs/methodology/`~~ **[Superseded 2026-09-29: `crates/cli/pack/` and \
               [x](gone.md)]** [Corrected: `packs/dev/`] but `packs/stale/`";
    assert_eq!(offences("x.md", doc), ["`packs/stale/`"]);
}

#[test]
fn an_exempt_form_must_close_inside_its_paragraph() {
    let doc = "[Superseded 2026: `packs/a/`\n\nnext paragraph";
    assert_eq!(
        scan("x.md", doc, &FIXTURE)
            .offences
            .iter()
            .map(|o| o.why)
            .collect::<Vec<_>>(),
        [
            "an exempt bracket that does not close inside its paragraph",
            "a repository path that does not resolve",
        ],
    );
    let strike = "~~`packs/a/`\n\n`packs/b/`~~";
    assert_eq!(
        scan("x.md", strike, &FIXTURE).offences.len(),
        4,
        "an unclosed strike exempts nothing and is itself an offence",
    );
}

#[test]
fn a_token_outside_the_declared_roots_is_not_a_path() {
    let doc =
        "`docs/roadmap.md` `.jigc/state/x.json` `~/.claude/x` `cargo test -p jigc` `README.md`";
    assert!(offences("x.md", doc).is_empty());
}

#[test]
fn the_two_settled_arms_read_as_not_a_path() {
    assert!(
        offences(
            "x.md",
            "`dev/0.3.0` `dev/1.0.0-rc.21` `.claude/skills/jigc/SKILL.md`"
        )
        .is_empty()
    );
    assert_eq!(offences("x.md", "`dev/0.3`"), ["`dev/0.3`"]);
}

#[test]
fn a_pending_path_is_accepted_while_absent() {
    assert!(
        offences(
            "design/x.md",
            "`dev/runner-faithful` [r](../dev/runner-faithful)"
        )
        .is_empty()
    );
}

#[test]
fn a_plain_link_label_naming_a_path_is_a_token() {
    assert_eq!(
        offences("design/x.md", "[crates/cli/pack/x.yaml](storage.md)"),
        ["`crates/cli/pack/x.yaml`"],
    );
}
