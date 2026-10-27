//! **The tool index, held to the tree** ([tooling.md](../implementation/tooling.md);
//! [DECISIONS.md](../DECISIONS.md) → *2026-10-27 — One index of the repository's tools*).
//!
//! The index is one row per tool and per deterministic instrument: the question it
//! answers, how it is called, the doc that owns it, the test or fence that holds it true.
//! A prose index that no test reads drifts — a tool is added beside it and never listed,
//! a tool is removed and its row still sends an agent to nothing — and an agent that finds
//! no row builds the tool again. So this suite is the index's fence:
//!
//! - **every entry of every home the index's own registry names has a row** — the
//!   registry is the index's table *Where tools live*: a directory and whether one tool
//!   there is a file or a directory, with what in it is no tool and why. The entries are
//!   read off the tree, never listed here, so a file added to `dev/` reddens until a row
//!   links it ([`every_entry_of_a_home_has_a_row`]);
//! - **every path a row or a home names exists**, so a row cannot outlive its tool
//!   ([`every_path_the_index_names_exists`]);
//! - **every row has its five cells**, the first linking the tool
//!   ([`every_row_has_its_five_cells`]);
//! - **the derivation itself names both faults** on a throwaway tree that has one of
//!   each, so the arms above cannot pass by reading nothing
//!   ([`the_derivation_names_an_unlisted_file_and_a_dangling_row`]);
//! - **every agent definition carries the one *Tools* paragraph**, byte for byte, as
//!   [`release_pipeline_fence`](super::release_pipeline_fence) arm (q) holds the *Never*
//!   paragraph, and the paragraph's links and the principle it names exist
//!   ([`every_agent_definition_carries_the_tools_paragraph`],
//!   [`the_tools_paragraph_points_at_what_exists`]).
//!
//! **Why the paragraph's arm is here and not a new arm of `release_pipeline_fence`.** That
//! suite's arm (q) holds what agents may not do — the human's acts. The *Tools* paragraph
//! is a pointer at this index, and the index and every pointer to it are one subject: a
//! change to the index's place or the principle's name reddens this suite alone.
//!
//! **Why `tooling-tests/`.** The index is a record of the repository's own tooling, and
//! none of this drives `jigc`: it belongs with the tooling suites, outside `crates/`, so a
//! change to it is no change to a product path (`tooling-tests/groups/g_tooling.rs`).
//!
//! **What it does not hold:** that a row's question, call or owner is *true* — a row is
//! read for its shape and its paths, and what it says is its owner's to keep right; and the
//! candidates list, whose entries name no path until one is built.

use std::fs;
use std::path::Path;

use crate::support::ci_workflow::repo_root;
use crate::support::root_walk;
use crate::support::scratch::ScratchDir;

/// The index, repository-relative; its links resolve from its own directory.
const INDEX: &str = "implementation/tooling.md";

/// The heading of the registry: the homes whose every entry needs a row.
const HOMES: &str = "## Where tools live";

/// The heading of the candidates list, whose rows are no tools yet.
const CANDIDATES: &str = "## Candidates";

/// A row's cells: the tool, the question it answers, how it is called, the doc that owns
/// it, the test or fence that holds it.
const ROW_CELLS: usize = 5;

/// The agents' definitions, repository-relative.
const AGENT_DEFINITIONS: &str = ".claude/agents";

/// The principles, where the paragraph's named section must stand.
const PRINCIPLES: &str = "implementation/working-principles.md";

/// The paragraph every agent definition carries, verbatim.
const TOOLS_PARAGRAPH: &str = "**Tools: use them, doubt them, hand yours on.** jigc is \
     deterministic, so much of what you establish a script or test can establish too — \
     before you judge, hunt or build, look in \
     [tooling.md](../../implementation/tooling.md) for the tool that answers your question, \
     and use it rather than rebuilding it \
     ([working-principles.md](../../implementation/working-principles.md) → *Deterministic \
     first*). A tool can be wrong: if you suspect a bug in one, investigate it and report it \
     as a finding with its evidence — never work around it silently. And if you wrote a \
     script or probe you ran more than once, or needed a tool that does not exist, say so \
     under **Tools**, the last section of your report, or in your return where you write no \
     report: what it does, the question it answers, and its text if it is short — a scratch \
     path is gone when you return.";

/// One home of the registry: a directory, whether one tool there is a file or a
/// directory, and the names in it that are no tool.
struct Home {
    line: usize,
    dir: String,
    each_directory: bool,
    not_tools: Vec<String>,
}

/// One row of a tool table, as its cells.
struct Row {
    line: usize,
    cells: Vec<String>,
}

/// The index, read: its registry and its rows.
struct Index {
    homes: Vec<Home>,
    rows: Vec<Row>,
}

/// A table line's cells, split on every `|` outside a code span.
fn cells(line: &str) -> Vec<String> {
    let inner = line.trim().trim_start_matches('|').trim_end_matches('|');
    let mut out = vec![String::new()];
    let mut in_code = false;
    for c in inner.chars() {
        match c {
            '`' => {
                in_code = !in_code;
                out.last_mut().expect("one cell").push(c);
            }
            '|' if !in_code => out.push(String::new()),
            _ => out.last_mut().expect("one cell").push(c),
        }
    }
    out.into_iter().map(|cell| cell.trim().to_owned()).collect()
}

/// Every relative link target in `text`, without its `#anchor`.
fn links(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("](") {
        rest = &rest[at + 2..];
        let Some(end) = rest.find(')') else { break };
        let target = &rest[..end];
        rest = &rest[end..];
        if target.contains("://") || target.starts_with('#') || target.starts_with("mailto:") {
            continue;
        }
        let target = target.split('#').next().unwrap_or(target);
        if !target.is_empty() {
            out.push(target.to_owned());
        }
    }
    out
}

/// A link target, resolved from the directory `from` (repository-relative), as a
/// repository-relative path with no trailing slash.
fn resolve(from: &str, target: &str) -> String {
    let mut parts: Vec<&str> = from.split('/').filter(|p| !p.is_empty()).collect();
    for segment in target.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}

/// The index's directory, from which its links resolve.
fn index_dir() -> &'static str {
    INDEX.rsplit_once('/').map_or("", |(dir, _)| dir)
}

/// The names a `Not a tool` cell lists: its code spans.
fn code_spans(cell: &str) -> Vec<String> {
    cell.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// Read the index: the registry's rows as homes, every other table's rows as tools, the
/// candidates' rows as nothing.
fn parse(text: &str) -> Index {
    let mut homes = Vec::new();
    let mut rows = Vec::new();
    let mut section = "";
    let mut table_line = 0usize;
    for (n, line) in text.lines().enumerate() {
        let number = n + 1;
        if line.starts_with("## ") {
            section = line;
            table_line = 0;
            continue;
        }
        if !line.trim_start().starts_with('|') {
            table_line = 0;
            continue;
        }
        table_line += 1;
        // The header and the separator under it.
        if table_line <= 2 {
            continue;
        }
        let cells = cells(line);
        if section.starts_with(HOMES) {
            let dir = links(&cells[0])
                .first()
                .map(|target| resolve(index_dir(), target))
                .unwrap_or_default();
            homes.push(Home {
                line: number,
                dir,
                each_directory: cells.get(1).is_some_and(|c| c == "each directory"),
                not_tools: cells.get(2).map(|c| code_spans(c)).unwrap_or_default(),
            });
        } else if !section.starts_with(CANDIDATES) {
            rows.push(Row {
                line: number,
                cells,
            });
        }
    }
    Index { homes, rows }
}

/// Every entry of every home that no row's first cell links — a file or directory of the
/// home, or a path under it — named repository-relative. A name that opens with `.` is the
/// operating system's or an editor's, never a tool: every tool is called by its name.
fn unlisted(root: &Path, index: &Index) -> Vec<String> {
    let listed: Vec<String> = index
        .rows
        .iter()
        .flat_map(|row| links(&row.cells[0]))
        .map(|target| resolve(index_dir(), &target))
        .collect();
    let mut out = Vec::new();
    for home in &index.homes {
        let dir = root.join(&home.dir);
        let mut entries: Vec<String> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("the home `{}` must be readable: {e}", home.dir))
            .map(|entry| entry.expect("an entry of a home"))
            .filter(|entry| {
                let path = entry.path();
                if home.each_directory {
                    path.is_dir()
                } else {
                    path.is_file()
                }
            })
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| !name.starts_with('.') && !home.not_tools.contains(name))
            .collect();
        entries.sort();
        assert!(
            !entries.is_empty(),
            "the home `{}` holds no tool — a fence over it would check nothing",
            home.dir
        );
        for name in entries {
            let entry = format!("{}/{name}", home.dir);
            let prefix = format!("{entry}/");
            if !listed
                .iter()
                .any(|link| *link == entry || link.starts_with(&prefix))
            {
                out.push(entry);
            }
        }
    }
    out
}

/// Every link of a home or a row that names nothing in the tree, as `line N: path`.
fn dangling(root: &Path, index: &Index) -> Vec<String> {
    let homes = index.homes.iter().map(|home| (home.line, home.dir.clone()));
    let rows = index.rows.iter().flat_map(|row| {
        row.cells
            .iter()
            .flat_map(|cell| links(cell))
            .map(|target| (row.line, resolve(index_dir(), &target)))
    });
    homes
        .chain(rows)
        .filter(|(_, path)| path.is_empty() || !root.join(path).exists())
        .map(|(line, path)| format!("line {line}: {path}"))
        .collect()
}

fn read_index(root: &Path) -> String {
    fs::read_to_string(root.join(INDEX))
        .unwrap_or_else(|e| panic!("`{INDEX}` must be readable: {e}"))
}

fn rel(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[test]
fn every_entry_of_a_home_has_a_row() {
    let root = repo_root();
    let index = parse(&read_index(&root));
    assert!(
        index.homes.iter().any(|home| home.dir == "dev"),
        "the registry names `dev/`, every file of which is a tool"
    );
    let unlisted = unlisted(&root, &index);
    assert!(
        unlisted.is_empty(),
        "every tool of a home the index's registry names must have a row in `{INDEX}` whose \
         first cell links it; these have none: {unlisted:?}. Add the row in the commit that \
         adds the tool — an agent that finds no row builds the tool again"
    );
}

#[test]
fn every_path_the_index_names_exists() {
    let root = repo_root();
    let index = parse(&read_index(&root));
    let dangling = dangling(&root, &index);
    assert!(
        dangling.is_empty(),
        "every path a row or a home of `{INDEX}` links must exist; these do not: \
         {dangling:?}. A row goes in the commit that removes or moves its tool"
    );
}

#[test]
fn every_row_has_its_five_cells() {
    let root = repo_root();
    let index = parse(&read_index(&root));
    assert!(
        index.rows.len() >= 20,
        "the scan found the index's rows: {}",
        index.rows.len()
    );
    let malformed: Vec<String> = index
        .rows
        .iter()
        .filter(|row| {
            row.cells.len() != ROW_CELLS
                || row.cells.iter().any(String::is_empty)
                || links(&row.cells[0]).is_empty()
        })
        .map(|row| format!("line {}: {:?}", row.line, row.cells))
        .collect();
    assert!(
        malformed.is_empty(),
        "every row of `{INDEX}` has {ROW_CELLS} cells — the tool, linked; the question; the \
         call; the owner; what holds it — and none empty: {malformed:#?}"
    );
}

#[test]
fn the_derivation_names_an_unlisted_file_and_a_dangling_row() {
    let scratch = ScratchDir::new("tooling-index");
    let root = scratch.path();
    fs::create_dir_all(root.join("dev")).expect("make dev/");
    fs::create_dir_all(root.join("implementation")).expect("make implementation/");
    for name in ["listed", "unlisted", ".DS_Store"] {
        fs::write(root.join("dev").join(name), "#!/bin/sh\n").expect("write a tool");
    }
    let index = parse(
        "# Tools\n\n\
         ## Where tools live\n\n\
         | Home | One tool is | Not a tool |\n\
         |---|---|---|\n\
         | [dev/](../dev/) | each file | — |\n\n\
         ## Tools\n\n\
         | Tool | Question | Call | Owner | Held by |\n\
         |---|---|---|---|---|\n\
         | [dev/listed](../dev/listed) | q | `dev/listed` | o | h |\n\
         | [dev/removed](../dev/removed) | q | `dev/removed` | o | h |\n\n\
         ## Candidates\n\n\
         | Candidate | Question | Raised by | Triage |\n\
         |---|---|---|---|\n\
         | [a probe](../scratch/gone) | q | r | t |\n",
    );
    assert_eq!(index.rows.len(), 2, "the candidates are no rows");
    assert_eq!(
        unlisted(root, &index),
        ["dev/unlisted"],
        "the file no row links, and not the dotfile"
    );
    assert_eq!(
        dangling(root, &index),
        ["line 14: dev/removed"],
        "the row whose tool is gone, and not the candidate's path"
    );
}

#[test]
fn every_agent_definition_carries_the_tools_paragraph() {
    let root = repo_root();
    let definitions = root_walk::files_in(&root.join(AGENT_DEFINITIONS), root_walk::ext("md"));
    let lacking: Vec<String> = definitions
        .iter()
        .filter(|path| {
            !fs::read_to_string(path)
                .unwrap_or_else(|e| panic!("`{}` must be readable: {e}", path.display()))
                .contains(TOOLS_PARAGRAPH)
        })
        .map(|path| rel(&root, path))
        .collect();
    assert!(
        lacking.is_empty(),
        "every agent definition must carry the Tools paragraph verbatim; these do not: \
         {lacking:?}. A definition is read alone, so the paragraph is how an agent learns \
         that the index exists, that a tool can be wrong, and that its own tools are handed on"
    );
}

#[test]
fn the_tools_paragraph_points_at_what_exists() {
    let root = repo_root();
    for target in links(TOOLS_PARAGRAPH) {
        let path = resolve(AGENT_DEFINITIONS, &target);
        assert!(
            root.join(&path).is_file(),
            "the Tools paragraph links `{target}`, which is `{path}` from a definition — it \
             must exist"
        );
    }
    let named = TOOLS_PARAGRAPH
        .split("→ *")
        .nth(1)
        .and_then(|rest| rest.split('*').next())
        .expect("the paragraph names the principle it rests on");
    let principles = fs::read_to_string(root.join(PRINCIPLES))
        .unwrap_or_else(|e| panic!("`{PRINCIPLES}` must be readable: {e}"));
    assert!(
        principles
            .lines()
            .any(|line| line.starts_with(&format!("## {named}"))),
        "the Tools paragraph names the section *{named}* of `{PRINCIPLES}`; no heading there \
         opens with it"
    );
}
