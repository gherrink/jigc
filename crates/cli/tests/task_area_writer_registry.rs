//! M52 Increment 4 / T2 — **jigc's own writer set is one registry, and the destroying
//! subject is its complement over the tree** (`completions/artifacts/M52/settle-record.md`
//! → D3.1 as amended by §6 and §18's riders; `gap-findings.md` → G-12, G-13;
//! `design/team-ready-state.md` → The working area's two populations).
//!
//! **The gap this closes.** Five doors destroy a working area, and every one of them took
//! the whole directory: a `NOTES.md` an agent left at `.jigc/tasks/<id>/` and an
//! `analysis/perf.txt` under it died at exit 0, named by nothing
//! ([baseline-destroying.md](../../../completions/artifacts/M52/baseline-destroying.md)
//! §2.1). The subject of that destruction is stated in **no** locked doc (G-12), and the
//! set it ought to have been is owned by nobody (G-13): thirteen filename constants
//! scattered across three modules, eleven private. The baseline's own hand-drawn list
//! drove **short** — it missed `renames.json`, which lands at the ordinary
//! `jigc doc rename --task` door — so a guard cut from it would have called three of
//! jigc's own files foreign and refused every task that had renamed a doc.
//!
//! So the registry is the deliverable, and this suite is the reason it can be trusted.
//! Four arms, each naming the kind of set it iterates:
//!
//! - **(a) the count fence** — a **counted source scan** on `repo_relative_paths.rs`'s
//!   mold (there is no clap tree to fence ⇔ against, G-13). Every production
//!   `<…dir>.join(<name>)` site in **both** crates resolves its `<name>` to a literal and
//!   must land in the registry; the rest are counted per file against a table that carries
//!   a reason each. Mutant-proven **in both directions**: a fourteenth writer const reddens
//!   it, and a member deleted from the registry reddens it.
//! - **(b) the complement over a manufactured shape space** — the baseline's §1.4 plant
//!   (S-a…S-j) plus the three `docs/` cells and the directory wearing a `*.md` name, each
//!   classified foreign, with every registry member beside them classified jigc's. The
//!   space is **manufactured and says so**: the shapes a filesystem admits are not a set
//!   any registry computes.
//! - **(c) the zero-false-fire control** — the whole point. A full lifecycle through the
//!   real binary (`start` on a create-granting workflow → `doc create` → `doc rename
//!   --task` → `task validate`) must leave the complement **empty**, `roles.json` and
//!   `renames.json` included. A registry short by one member is a door that destroys or
//!   refuses over jigc's own file, which is strictly worse than the gap it closes.
//! - **(d) the milestone-area row** — the same control over a real `milestone create` +
//!   `milestone add-task`.
//!
//! **Declared bound.** The fence reads `.join(` sites. A working-area path composed by
//! `format!`, by `PathBuf::push`, or by a helper that takes the filename as an argument is
//! outside its reach — the one such helper today is `state::instance_path`, whose subject
//! is the `docs/` tree the registry covers by rule rather than by name. The fence is a
//! *counted* scan, not a proof of the writer set's completeness; what proves completeness
//! is arm (c), driven through the binary.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::support::rust_source;

// ---------------------------------------------------------------------------
// (a) the count fence
// ---------------------------------------------------------------------------

/// The two crate source trees the registry governs. Both, because the writer set spans
/// them — `source` was written by the CLI and read by the engine, which is exactly how a
/// registry declared in one crate goes wrong.
const CRATE_SRC: &[&str] = &["crates/engine/src", "crates/cli/src"];

/// The **staged doc bodies**: production sites that join a filename **composed at
/// runtime** under a working area's `docs/` member, by file and count, with the reason.
///
/// These are jigc's own writes, and they are guarded — by the registry's *tree* rule
/// rather than by one of its names, because the name is the task's own staged address
/// (`engine::state::TASK_DOCS_FILES`). A fence that demanded a registry *name* here would
/// be demanding the impossible; a fence that ignored them would be silent about the one
/// population inside `docs/` that the rule has to get right.
const DOCS_TREE_JOINS: &[(&str, usize, &str)] = &[
    (
        "crates/engine/src/file_state.rs",
        2,
        "`<task_dir>/docs/<from>.md` — the staged copy probed by the reconciler, twice \
         (placement branch and location branch)",
    ),
    (
        "crates/engine/src/index.rs",
        1,
        "`<task_dir>/docs/<to>.md` — the rename target probed on the task surface",
    ),
    (
        "crates/engine/src/milestone.rs",
        1,
        "`<sub_dir>/docs/<claimed>.md` — the body each provenance entry claims, at the join",
    ),
    (
        "crates/engine/src/validate.rs",
        2,
        "`<dir>/docs/<filename>` — each staged instance read by the task-scope sweep, twice",
    ),
];

/// The **complement**: production sites that join a working-area entry the complement
/// derivation has *already* classified as **not jigc's**, by file and count, with the
/// reason (M52 Increment 4 / T3).
///
/// A third answer, and it has to be its own: the receiver *is* a working area — so
/// [`NON_AREA_JOINS`]' reason would be false of it — while the joined name is a runtime
/// value that must **never** be a registry member, which is the exact inverse of what
/// [`DOCS_TREE_JOINS`] says about its sites. Demanding a registry name here would demand
/// the impossible in the other direction: these are the names the registry excludes.
const COMPLEMENT_JOINS: &[(&str, usize, &str)] = &[(
    "crates/cli/src/task.rs",
    3,
    "`displace_foreign_area`'s `<area>.join(<complement entry>)` — the source of the move \
     that keeps a foreign byte out of the teardown — `foreign_areas`' own \
     `<area>.join(<complement entry>)`, which absolutizes the same entries so the three \
     consenting doors can name them and re-read them after the removal (M52 Increment 4 / \
     T5), and `kept_area_finding`'s, which re-roots the post-unwind re-read's entries so the \
     landed advisory prints them repo-relative through `render::repo_relative` rather than \
     as bare basenames an operator cannot act on (M53 Increment 2 / T3); all three names \
     come from `engine::state::foreign_area_paths`, so they are registry members' \
     complement by construction",
)];

/// **The registry row itself**: production sites that join a name taken from
/// `engine::state::WorkArea::jigc_written()`, by file and count, with the reason (M52
/// Increment 5 / T7).
///
/// A fourth answer, and like the third it has to be its own: the receiver **is** a working
/// area, so [`NON_AREA_JOINS`]' reason is false of it; the joined name is a registry member
/// **by construction**, which is the exact inverse of [`COMPLEMENT_JOINS`]; and it is not a
/// `docs/` body composed at runtime. A fence demanding a registry *literal* here would be
/// demanding a literal where the code has the row — the joined name is every member, in turn.
const ROW_JOINS: &[(&str, usize, &str)] = &[(
    "crates/engine/src/state.rs",
    1,
    "`unwind_area`'s `<area>.join(<row member>)` — the `MintedSet` sink walking the area's \
     own row to remove jigc's own writes before a **non-recursive** `remove_dir`, so what \
     jigc did not write survives by construction",
)];

/// **The registry row, read**: production sites that join **one named member** of
/// `engine::state::WorkArea::jigc_written()` in order to *ask about* the entry rather than
/// write, walk or move it, by file and count, with the reason (M53 Increment 3 / T2).
///
/// A fifth answer, and it is its own for the reason the other four are: the receiver **is**
/// a working area, so [`NON_AREA_JOINS`]' reason is false of it; the joined name is a
/// registry member by construction, so [`COMPLEMENT_JOINS`]' is its inverse; it is no
/// `docs/` body; and it is not [`ROW_JOINS`]' walk over *every* member before a removal —
/// nothing here removes, moves or writes anything. Folding it into `ROW_JOINS` would print
/// *walked* over a site that only stats, and this fence exists so that one word means one
/// answer.
const ROW_READ_JOINS: &[(&str, usize, &str)] = &[(
    "crates/engine/src/state.rs",
    1,
    "`carries_base_pin`'s `<area>.join(<row member 0>)` — the residual rule's one predicate, \
     taking the pin's name off the kind's own row rather than spelling it, so the file it \
     asks about and the file `unwind_area` removes **first** cannot drift apart; it \
     `symlink_metadata`s that path and answers a bool, and writes nothing",
)];

/// Every production `<…dir>.join(<name>)` site whose receiver is **not** a working area,
/// by file, with the count and the reason — the remainder, stated as a number so that
/// nothing goes stale silently (`repo_relative_paths.rs` → the unswept remainder is a
/// count, not a sentence).
///
/// A file may appear here *and* hold guarded sites: `dir` is a name production code gives
/// to plenty of directories that are not working areas, and the fence deliberately reads
/// the **name being joined** rather than guessing at the receiver's meaning.
///
/// **[Corrected 2026-09-22 (M53 completion audit, fix 1).** This note read: *"The name is one
/// row too narrow, and the row that falsifies it says so. M53 Increment 2 / T1 drove
/// `try_execute_finalize_plan`'s `msg_tmp_dir` to be `cleanup_dir` at all three call sites, so
/// its `finalize-message.tmp` is joined onto a working area. It belongs in the remainder all
/// the same, for the reason its row carries: the registry's members are what a door about to
/// **destroy** an area must not take, and a name that is gone before that door ever reads the
/// area is not one of them."* **Falsifying datum, driven on the debug binary at `f664863a`:** a
/// `pre-commit` hook that `chmod 0555`s the task area makes the best-effort
/// `remove_file(&msg_path)` fail, so the transient is **not** gone before the door reads the
/// area — `jigc task finalize` landed at exit 0 naming `.jigc/tasks/<id>/finalize-message.tmp`
/// as *a path jigc did not write*, and `jigc task discard <id>` then refused at exit 1 over
/// jigc's own file. *Gone before the door reads it* is a claim about the ordinary arm, and
/// membership is decided on the faulting one. `engine::state::FINALIZE_MESSAGE_FILE` is now a
/// member of **both** rows, so this table's name is exact again and the row below counts one
/// site, not two.**]
///
/// So the membership rule this table states is *the joined name is no registry member*, and —
/// with the correction above — *the receiver is no working area* is why that holds for every
/// row in it.
const NON_AREA_JOINS: &[(&str, usize, &str)] = &[
    (
        "crates/cli/src/adapter.rs",
        1,
        "the adapter profile's config dir gets a `.gitkeep`, not a working-area file",
    ),
    (
        "crates/cli/src/config.rs",
        4,
        "the project **config layer** — two `steps/<basename>.yaml` writes, one \
         `fills/<id>.md`, one `steps/<step_id>.yaml`; a config dir is no working area",
    ),
    (
        "crates/cli/src/invocation_log.rs",
        1,
        "the invocation log's own file under `.jigc/logs/`, which is no working area",
    ),
    (
        "crates/cli/src/pack.rs",
        2,
        "two `packs.yaml` reads off the project config dir. **[Corrected 2026-09-23 (M53 — \
         the cwd census, the verb class).** This row read `3` and counted a third site, \
         *one `.git` repo-root probe*. That probe was a seventh, inlined copy of \
         `crate::repo::discover_repo_root`, and it decided the project pack-set from the \
         **standing checkout** — so inside a fan-out worktree, which carries no \
         `.jigc/config/`, the whole project layer vanished. `discover_project_config` now \
         resolves `crate::repo::jigc_home`, which joins `.jigc` rather than `.git`.**]",
    ),
    (
        "crates/cli/src/relocate.rs",
        1,
        "a squatter parked into the workbench dir by its own basename (a runtime value, \
         not a declared name)",
    ),
    (
        "crates/cli/src/repo.rs",
        3,
        "`.git` (the root probe), one `HEAD` read and one caller-supplied entry, all off \
         the **git** dir",
    ),
    (
        "crates/cli/src/setup.rs",
        6,
        "two `pre-commit` hook paths off the hooks dir, the `doc-code` probe binary off \
         the bin dir, `packs.yaml` off the config dir, and — since M52 Increment 4 / T5 — \
         the two receivers `workbench_foreign_areas` builds its subject FROM: \
         `.jigc/<tasks|milestones>` and `.jigc/displaced`, which are the directories that \
         *hold* working areas (and the parking home) rather than working areas themselves",
    ),
    (
        "crates/cli/src/rollback.rs",
        1,
        "the conflicted pre-image `park` copies into `.jigc/displaced/<door>/` by the \
         entry's own identity (a runtime value, not a declared name) — the workbench is \
         where bytes are kept OUT of a teardown, never a working area \
         (M52 Increment 5 / T2)",
    ),
    (
        "crates/cli/src/task.rs",
        0,
        "**[Corrected 2026-09-22 (M53 completion audit, fix 1).** This row read `2` and \
         counted `try_execute_finalize_plan`'s `finalize-message.tmp` beside the `.git` \
         probe, on the ground that *it does not outlive the transaction: it is gone before \
         phase 7 reads the area*. **Falsifying datum, driven at `f664863a`:** the removal is \
         `let _ = std::fs::remove_file(…)`, and a `pre-commit` hook that `chmod 0555`s the \
         area makes it fail — the transient survived into the complement, `task finalize` \
         named it as *a path jigc did not write* at exit 0, and `task discard` then refused \
         at exit 1 over it. The name is now `engine::state::FINALIZE_MESSAGE_FILE` on both \
         registry rows, so the site is a guarded member rather than a remainder.**] \
         **[Corrected 2026-09-23 (M53 — the cwd census, the verb class).** The row then \
         read `1`, for *`.git` — the repo-root probe, whose receiver is an ancestor \
         walk*. That probe was one of five byte-identical private copies of \
         `crate::repo::discover_repo_root`; the copies are collapsed onto the one \
         function in `repo.rs`, whose own row already counts its `.git` join, so this \
         file now carries no unregistered site at all.**]",
    ),
    (
        "crates/engine/src/target_surface.rs",
        1,
        "`<repo_root>/<location>/<slug>.md` — a **committed store** location, not a \
         working area",
    ),
];

/// The union of both registry rows plus the `docs/` cell's fixed member — every name the
/// fence accepts as jigc's own.
fn registry_members() -> Vec<&'static str> {
    let mut all: Vec<&'static str> = Vec::new();
    all.extend_from_slice(engine::state::TASK_AREA_FILES);
    all.extend_from_slice(engine::state::TASK_DOCS_FILES);
    all.extend_from_slice(engine::state::MILESTONE_AREA_FILES);
    all.sort_unstable();
    all.dedup();
    all
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// One `.join(<name>)` site the fence found: where it is, and the name it joins as
/// written in the source.
struct JoinSite {
    file: String,
    line: usize,
    arg: String,
}

/// Every `const <NAME>: &str = "<literal>";` in production source, across both crates —
/// the resolver that lets the fence read `task_dir.join(RENAMES_FILE)` as the name it
/// actually is. A writer that joins a *constant* is the shape the registry exists for, so
/// a fence that only understood literals would be blind to exactly the sites it guards.
fn str_consts() -> BTreeMap<String, String> {
    let mut consts = BTreeMap::new();
    for dir in CRATE_SRC {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            let body = fs::read_to_string(&path).expect("read source");
            let code = rust_source::code_and_strings(&body);
            // Regions are brace-matched, so they must be read off the **string-blanked**
            // source — a `{` inside a literal throws the match off, and both blankings
            // preserve byte offsets, so the two readings line up.
            let regions = rust_source::cfg_test_regions(&rust_source::code_only(&body));
            for (at, _) in code.match_indices("const ") {
                if rust_source::is_test_domain(&path, &regions, at) {
                    continue;
                }
                let tail = &code[at + "const ".len()..];
                let Some(colon) = tail.find(':') else {
                    continue;
                };
                let name = tail[..colon].trim();
                if name.is_empty() || !name.chars().all(|c| c.is_ascii_uppercase() || c == '_') {
                    continue;
                }
                let Some(eq) = tail.find('=') else { continue };
                let Some(rest) = tail.get(eq + 1..) else {
                    continue;
                };
                let rest = rest.trim_start();
                if !rest.starts_with('"') {
                    continue;
                }
                let Some(end) = rest[1..].find('"') else {
                    continue;
                };
                consts.insert(name.to_string(), rest[1..1 + end].to_string());
            }
        }
    }
    consts
}

/// Whether `receiver` — the expression text immediately left of a `.join(` — names a
/// directory. The rule is the **last path segment**: an identifier ending in `dir`, or
/// `area`. A call (`temp_dir()`) is not a binding and is not read as one.
fn is_dir_receiver(receiver: &str) -> bool {
    let last = receiver.rsplit('.').next().unwrap_or_default();
    !last.ends_with(')') && (last.ends_with("dir") || last == "area")
}

/// Every production `<…dir>.join(<name>)` site in `path`, including the **second** hop of
/// a chained `<dir>.join(A).join(B)` — `ProvenanceRecord::path_in` is exactly that shape,
/// and a fence blind to it would leave `provenance.json` unguarded.
fn area_join_sites(path: &Path) -> Vec<JoinSite> {
    let body = fs::read_to_string(path).expect("read source");
    let code = rust_source::code_and_strings(&body);
    // `cfg_test_regions` brace-matches, so it reads the **string-blanked** source: a `{`
    // inside a test's literal would otherwise mis-bound the region and leak that test's
    // sites into a production fence. Byte offsets are preserved by both blankings.
    let regions = rust_source::cfg_test_regions(&rust_source::code_only(&body));
    let file = path
        .strip_prefix(workspace_root())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    let mut sites = Vec::new();
    for (at, _) in code.match_indices(".join(") {
        if rust_source::is_test_domain(path, &regions, at) {
            continue;
        }
        let receiver = receiver_text(&code, at);
        if !is_dir_receiver(&receiver) {
            continue;
        }
        let mut open = at + ".join(".len() - 1;
        while let Some(close) = matching_paren(&code, open) {
            sites.push(JoinSite {
                file: file.clone(),
                line: body[..open].lines().count(),
                arg: code[open + 1..close].trim().to_string(),
            });
            // A chained `.join(` right after the close is the same receiver one level
            // deeper — `<task_dir>.join(DOCS_DIR).join(PROVENANCE_FILE)`.
            let tail = &code[close + 1..];
            if tail.starts_with(".join(") {
                open = close + 1 + ".join(".len() - 1;
            } else {
                break;
            }
        }
    }
    sites
}

/// The expression text immediately left of `at` — an identifier path (`self.dir`,
/// `minted.dir`, `task_dir`), read backwards over the characters a path may contain.
fn receiver_text(code: &str, at: usize) -> String {
    let head = &code[..at];
    let start = head
        .rfind(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.' || c == ')'))
        .map(|i| i + 1)
        .unwrap_or(0);
    head[start..].to_string()
}

/// The index of the `)` closing the `(` at `open`, over blanked code so a paren inside a
/// literal cannot throw the match off.
fn matching_paren(code: &str, open: usize) -> Option<usize> {
    let bytes = code.as_bytes();
    let mut depth = 0i32;
    for (i, b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

#[test]
fn every_area_join_name_is_a_registry_member_or_a_counted_remainder() {
    let members = registry_members();
    let consts = str_consts();

    let mut guarded: Vec<String> = Vec::new();
    let mut remainder: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for dir in CRATE_SRC {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            for site in area_join_sites(&path) {
                // Resolve the joined name: a literal is itself; a constant path resolves
                // through its declaration; anything else is unresolvable and counts as
                // remainder, which is the fail-closed direction.
                let literal = if site.arg.starts_with('"') && site.arg.ends_with('"') {
                    Some(site.arg[1..site.arg.len() - 1].to_string())
                } else {
                    consts
                        .get(site.arg.rsplit("::").next().unwrap_or(""))
                        .cloned()
                };
                let where_ = format!("{}:{} — .join({})", site.file, site.line, site.arg);
                match literal {
                    Some(name) if members.contains(&name.as_str()) => guarded.push(where_),
                    _ => remainder.entry(site.file.clone()).or_default().push(where_),
                }
            }
        }
    }

    assert!(
        !guarded.is_empty(),
        "the fence found no guarded site at all — it has stopped reading the source",
    );

    // Each file's remainder is the **sum** of its dispositions — the `docs/` bodies the tree
    // rule covers, the receivers that are no working area at all, the complement entries a
    // door moves rather than writes, the registry row a sink walks, and the one row member a
    // predicate reads. One table for them would be one word for five different answers.
    let mut expected: BTreeMap<String, (usize, Vec<String>)> = BTreeMap::new();
    for (table, kind) in [
        (DOCS_TREE_JOINS, "a staged `docs/` body (the tree rule)"),
        (NON_AREA_JOINS, "no working area"),
        (
            COMPLEMENT_JOINS,
            "a complement entry, moved rather than written",
        ),
        (ROW_JOINS, "the registry row itself, walked"),
        (ROW_READ_JOINS, "the registry row itself, read"),
    ] {
        for (file, count, reason) in table {
            assert!(!reason.trim().is_empty(), "{file}: carries no reason");
            let row = expected.entry((*file).to_string()).or_default();
            row.0 += *count;
            row.1.push(format!("{count} × {kind}: {reason}"));
        }
    }

    let mut offenders: Vec<String> = Vec::new();
    for (file, (count, why)) in &expected {
        let sites = remainder.remove(file).unwrap_or_default();
        if sites.len() != *count {
            offenders.push(format!(
                "  {file}: the tables say {count} unregistered `.join(…)` site(s) — {} — \
                 the source has {}:\n      {}",
                why.join("; "),
                sites.len(),
                sites.join("\n      "),
            ));
        }
    }
    for (file, sites) in &remainder {
        offenders.push(format!(
            "  {file}: {} `.join(<name>)` site(s) on a directory receiver whose name is in \
             NEITHER registry row and in no remainder row — a new working-area writer joins \
             `engine::state::TASK_AREA_FILES` (or its milestone sibling), a site that moves \
             an already-classified foreign entry joins `COMPLEMENT_JOINS`, and anything \
             else joins `NON_AREA_JOINS` with its reason:\n      {}",
            sites.len(),
            sites.join("\n      "),
        ));
    }

    assert!(
        offenders.is_empty(),
        "jigc's own writer set is one registry, and this is the count that keeps it one \
         (M52 Increment 4 / T2):\n{}",
        offenders.join("\n"),
    );
}

// ---------------------------------------------------------------------------
// (b) the complement over the manufactured shape space
// ---------------------------------------------------------------------------

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-area-registry-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // Mode-000 plants would otherwise survive the teardown.
        let _ = Command::new("chmod")
            .args(["-R", "u+rwX", &self.0.to_string_lossy()])
            .output();
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// **The shape space, manufactured and declared so.** S-a…S-j are the baseline's own plant
/// (`baseline-destroying.md` §1.4), driven through `jigc task discard` at `7637a46f` and
/// destroyed at exit 0 to the last one; the final three cells are §6's `docs/` rule and
/// §18's rider (the reachable foreign `docs/*.md` is a **directory** wearing that name).
///
/// It is manufactured because the set of shapes a filesystem admits is not a set any
/// registry or clap tree computes — it is decided, and deciding it wrongly is how L-3
/// shipped.
const FOREIGN_CELLS: &[(&str, &str)] = &[
    ("NOTES.md", "S-c · an ordinary `.md` file at the area root"),
    ("notes.txt", "S-a · a non-`.md` file at the area root"),
    ("docs/notes.txt", "S-b · a non-`.md` file under `docs/`"),
    (
        "analysis",
        "S-d · a nested directory holding a file (`analysis/perf.txt`)",
    ),
    ("empty-dir", "S-e · an empty directory"),
    (".hidden", "S-f · a dotfile"),
    ("link-out", "S-g · a symlink to a file outside the repo"),
    (
        "link-out-dir",
        "S-h · a symlink to a directory outside the repo",
    ),
    ("link-in", "S-i · a symlink into the repo"),
    ("locked.txt", "S-j · a file with mode 000"),
    (
        "docs/fake:thing.md",
        "§18's rider · a DIRECTORY wearing a staged doc's name (L-3's cell at the \
         destroying door's own question)",
    ),
    (
        "docs/agent-notes.md",
        "the `.md` half of §6's `docs/` rule · a plain `.md` FILE under `docs/` whose name \
         is no staged identity — jigc's own writer emits `<type>:<slug>.md` at every site \
         (`engine::state::instance_path`), so a colon-less `.md` is something else's",
    ),
    (
        "docs/sub",
        "§6 · a directory under `docs/` that is no staged identity",
    ),
    (
        "base.json/",
        "shape is membership · a DIRECTORY wearing a member's name",
    ),
];

/// Plant the manufactured space into `area`, beside a jigc-shaped set of every registry
/// member, and return the relative paths planted.
fn plant_shape_space(area: &Path, outside: &Path) -> Vec<PathBuf> {
    fs::create_dir_all(area.join("docs")).expect("docs");
    // Every registry member, in the shape jigc writes it — the control half.
    for member in engine::state::TASK_AREA_FILES {
        if *member == "docs" || *member == "base.json" {
            continue;
        }
        fs::write(area.join(member), b"jigc\n").expect("member");
    }
    for member in engine::state::TASK_DOCS_FILES {
        fs::write(area.join("docs").join(member), b"{}\n").expect("docs member");
    }
    fs::write(area.join("docs").join("adr:a-real-one.md"), b"# A\n").expect("staged doc");

    fs::write(outside.join("outside.txt"), b"outside\n").expect("outside file");
    fs::create_dir_all(outside.join("outsidedir")).expect("outside dir");

    let mut planted = Vec::new();
    for (cell, _) in FOREIGN_CELLS {
        let path = area.join(cell);
        match *cell {
            "analysis" => {
                fs::create_dir_all(&path).expect("nested dir");
                fs::write(path.join("perf.txt"), b"perf\n").expect("nested file");
            }
            "empty-dir" | "docs/fake:thing.md" | "docs/sub" | "base.json/" => {
                fs::create_dir_all(&path).expect("dir cell");
            }
            "link-out" => {
                std::os::unix::fs::symlink(outside.join("outside.txt"), &path).expect("link");
            }
            "link-out-dir" => {
                std::os::unix::fs::symlink(outside.join("outsidedir"), &path).expect("link");
            }
            "link-in" => {
                std::os::unix::fs::symlink(area.join("intent"), &path).expect("link");
            }
            "locked.txt" => {
                fs::write(&path, b"locked\n").expect("locked");
                Command::new("chmod")
                    .args(["000", &path.to_string_lossy()])
                    .output()
                    .expect("chmod");
            }
            _ => {
                fs::write(&path, b"foreign\n").expect("plain cell");
            }
        }
        planted.push(PathBuf::from(cell.trim_end_matches('/')));
    }
    planted.sort();
    planted
}

#[test]
fn the_complement_is_every_cell_of_the_shape_space_and_no_registry_member() {
    let root = TempDir::new("space");
    let area = root.path().join("area");
    let outside = root.path().join("outside");
    fs::create_dir_all(&outside).expect("outside root");
    let planted = plant_shape_space(&area, &outside);

    let foreign = engine::state::foreign_area_paths(&area, engine::state::WorkArea::Task)
        .expect("a readable area enumerates");

    assert_eq!(
        foreign,
        planted,
        "every manufactured cell is foreign and no registry member is — cells:\n{}",
        FOREIGN_CELLS
            .iter()
            .map(|(cell, why)| format!("  {cell}: {why}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    // The members are the control half: named explicitly, so a registry that classified
    // its own writers foreign reddens here with the member's name rather than as a diff.
    // `base.json` is the one exception, and it is the point of its cell — it is planted as
    // a **directory**, and shape is part of membership, so it belongs in the complement
    // exactly as it is asserted to above.
    for member in engine::state::TASK_AREA_FILES {
        if *member == "base.json" {
            continue;
        }
        assert!(
            !foreign.contains(&PathBuf::from(*member)),
            "`{member}` is a registry member, planted in the shape jigc writes it, and must \
             never be in the complement",
        );
    }
}

#[test]
fn an_unreadable_area_is_an_error_never_an_empty_complement() {
    let root = TempDir::new("closed");
    let area = root.path().join("area");
    fs::create_dir_all(area.join("docs")).expect("area");
    Command::new("chmod")
        .args(["000", &area.to_string_lossy()])
        .output()
        .expect("chmod");

    let result = engine::state::foreign_area_paths(&area, engine::state::WorkArea::Task);

    Command::new("chmod")
        .args(["u+rwx", &area.to_string_lossy()])
        .output()
        .expect("chmod back");

    assert!(
        result.is_err(),
        "a present-but-unreadable area must be an `Err` — enumerating nothing and finding \
         nothing are the same empty vector to a door about to delete, and only one of them \
         is safe (`staged_task_prose`'s discipline)",
    );

    // Absence is the other half of the same rule, and it is the *only* empty answer.
    assert_eq!(
        engine::state::foreign_area_paths(
            &root.path().join("never-existed"),
            engine::state::WorkArea::Task,
        )
        .expect("an absent area is not an error"),
        Vec::<PathBuf>::new(),
    );
}

// ---------------------------------------------------------------------------
// (c) + (d) the zero-false-fire controls, driven through the real binary
// ---------------------------------------------------------------------------

fn git(repo: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
}

fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// A real install over a one-commit repo — the state every driven arm starts from.
fn installed(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write README");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    (repo, home)
}

#[test]
fn a_full_task_lifecycle_leaves_an_empty_complement() {
    let (repo, home) = installed("lifecycle");
    let task = "record-a-decision-about-caching";

    // `record-decision` grants the ADR create, so the lifecycle reaches `roles.json`.
    ok(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "record-decision",
            "record a decision about caching",
        ],
        "jigc start",
    );
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache the thing",
            "--task",
            task,
        ],
        "jigc doc create",
    );
    // The rename is what writes `renames.json` — the member the baseline's hand-drawn
    // list missed, and the one that would have made this guard refuse every task that
    // renamed a doc.
    ok(
        repo.path(),
        home.path(),
        &[
            "doc",
            "rename",
            "adr:cache-the-thing",
            "--to",
            "Cache the other thing",
            "--task",
            task,
        ],
        "jigc doc rename",
    );
    // `task validate` is the door that can land the doc↔code probe snapshots.
    jigc(repo.path(), home.path(), &["task", "validate", task]);

    let area = repo.path().join(".jigc").join("tasks").join(task);
    let on_disk: Vec<String> = fs::read_dir(&area)
        .expect("read the area")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    assert!(
        on_disk.contains(&"roles.json".to_string())
            && on_disk.contains(&"renames.json".to_string()),
        "the lifecycle must actually reach `roles.json` and `renames.json`, else this \
         control proves nothing; the area holds: {on_disk:?}",
    );

    let foreign = engine::state::foreign_area_paths(&area, engine::state::WorkArea::Task)
        .expect("the area enumerates");
    assert_eq!(
        foreign,
        Vec::<PathBuf>::new(),
        "jigc's own nine files must never trip the guard — a registry short by one member \
         is a door that destroys or refuses over jigc's own file, which is worse than the \
         gap it closes; the area holds: {on_disk:?}",
    );
}

#[test]
fn a_minted_milestone_area_leaves_an_empty_complement() {
    let (repo, home) = installed("milestone");
    ok(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
        "jigc milestone create",
    );
    ok(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Warm the read cache",
        ],
        "jigc milestone add-task",
    );

    let area = repo
        .path()
        .join(".jigc")
        .join("milestones")
        .join("cache-rework");
    let on_disk: Vec<String> = fs::read_dir(&area)
        .expect("read the area")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();

    let foreign = engine::state::foreign_area_paths(&area, engine::state::WorkArea::Milestone)
        .expect("the area enumerates");
    assert_eq!(
        foreign,
        Vec::<PathBuf>::new(),
        "the milestone row's four files — base pin, task list, staged snapshot, record \
         commit message — are jigc's own; the area holds: {on_disk:?}",
    );

    // …and the row still classifies: a foreign byte beside them is found.
    fs::write(area.join("scratch.txt"), b"mine\n").expect("plant");
    assert_eq!(
        engine::state::foreign_area_paths(&area, engine::state::WorkArea::Milestone)
            .expect("the area enumerates"),
        vec![PathBuf::from("scratch.txt")],
    );
}
