//! **Every door that ensures `.jigc/.gitignore` says what it changed there** (M51
//! Increment 4 / T2 — EC-18's law-1 half; `completions/artifacts/M51/settle-record.md`
//! → §6 *"with the ack naming the content change"* · `design/surface-contract.md` → law
//! 1 · `completions/artifacts/M51/gap-capabilities.md` → G10 item 3).
//!
//! **The base-red, driven at `db70f1e3`** (the amend-to-union writer, T1's commit). A
//! repo whose committed `.jigc/.gitignore` predates an [`ENTRIES`] addition — the
//! ordinary *upgrade jigc → run any door* sequence — and carries the user's own two
//! lines:
//!
//! ```text
//! jigc setup            → EXIT=0, the summary lists five install targets, `.jigc/.gitignore`
//!                         is not among them, and `displaced/` was appended and committed
//! jigc milestone create → EXIT=0, `minted milestone:… / record: … / next: …`, and nothing
//!                         anywhere names the file jigc just wrote into
//! ```
//!
//! The amend is no longer a loss (T1), but it is still jigc writing into a file the user
//! legitimately co-owns — and law 1 is that the door says what it did. The finalize
//! manifest is the closest anything came: it prints `modified .jigc/.gitignore`, naming
//! the **file** and not the **change**, which is exactly the gap the baseline recorded
//! (`completions/artifacts/M51/baseline-committing-doors.md` → §6).
//!
//! **Two arms, on the `mint_doors.rs` mold, because the axis is *which doors exist*.**
//!
//! 1. **The completeness fence** — the production call-site set of
//!    [`cli::gitignore::ensure`] across both crates must equal [`IGNORE_DOORS`]' declared
//!    `site` set, read off source with comments and string literals blanked and
//!    `#[cfg(test)]` bodies excluded (the shared [`support::rust_source`] scanner). A grep
//!    is what *found* these four; it cannot stop the fifth
//!    (`implementation/dev-workflow.md` → *a grep is not a fence*), and a fifth door that
//!    wrote into the user's file silently is precisely this task's defect returning.
//! 2. **One driven cell per member**, through the real binary over a planted, **committed**
//!    under-set `.jigc/.gitignore`: the door's ack must name the entries it appended, and
//!    the same door re-run — with nothing left to append — must say nothing at all. A
//!    member with no cell is a **hard panic**, not a skip, and the declared [`Ack`] is
//!    checked against what the cell observed, so an `Exempt` row's silence is a
//!    disposition with a stated reason rather than an oversight.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use cli::gitignore::{Ack, ENTRIES, IGNORE_DOORS};

use crate::support;
use support::rust_source::{cfg_test_regions, code_only, enclosing_fn, is_test_domain, rust_files};
use support::trial_corpus::{State, TrialCorpus};

// ───────────────────────── (a) the completeness fence ─────────────────────────

/// The workspace root — two levels up from `crates/cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root")
}

/// Every production call site of `gitignore::ensure` in `code`, as
/// `<rel-path>::<enclosing fn>`.
///
/// *Production* is everything outside the test domain — a `tests/` tree, or a
/// `#[cfg(test)]` module body (`relocate.rs`'s call sits in one) — read off source with
/// comments and string literals blanked, so a doc-comment naming the writer is not a
/// call.
///
/// A match counts when it is **qualified** (`…gitignore::ensure(`) or lives in the
/// writer's own module, which is where an unqualified call could appear; the function's
/// own `fn ensure(` header is skipped by the token before it, and a match glued to an
/// identifier (`reensure(`) is not a call to this one.
fn ensure_sites_in(rel: &str, code: &str, regions: &[(usize, usize)], path: &Path) -> Vec<String> {
    let own_module = rel.ends_with("crates/cli/src/gitignore.rs");
    let mut sites = Vec::new();
    for (at, _) in code.match_indices("ensure(") {
        if is_test_domain(path, regions, at) {
            continue;
        }
        let head = &code[..at];
        // `fn ensure(…)` is the definition, not a call site.
        if head.trim_end().ends_with("fn") {
            continue;
        }
        // A suffix of some longer identifier is a different function.
        if head
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        if !head.ends_with("gitignore::") && !own_module {
            continue;
        }
        let owner = enclosing_fn(code, at)
            .unwrap_or_else(|| panic!("a call to `gitignore::ensure` sits inside some fn ({rel})"));
        sites.push(format!("{rel}::{owner}"));
    }
    sites
}

/// Every production call site of the writer across both member crates.
fn production_ensure_sites() -> BTreeSet<String> {
    let root = workspace_root();
    let mut sites = BTreeSet::new();
    let mut files = 0usize;
    for crate_dir in ["crates/engine", "crates/cli"] {
        for path in rust_files(&root.join(crate_dir)) {
            // The `doc-code` probe is a detached workspace that cannot depend on `cli`.
            if path.components().any(|c| c.as_os_str() == "probes") {
                continue;
            }
            files += 1;
            let body = fs::read_to_string(&path).expect("read a workspace source");
            let code = code_only(&body);
            let regions = cfg_test_regions(&code);
            let rel = path
                .strip_prefix(&root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            sites.extend(ensure_sites_in(&rel, &code, &regions, &path));
        }
    }
    assert!(
        files > 100,
        "the sweep must find the workspace sources; it saw only {files}",
    );
    sites
}

/// The declared site set — every production writer the registry names.
///
/// Duplicate `site` values would let one entry stand for two real calls with two
/// different dispositions, so the set must be as large as the table.
fn declared_sites() -> BTreeSet<String> {
    let declared: BTreeSet<String> = IGNORE_DOORS.iter().map(|d| d.site.to_string()).collect();
    assert_eq!(
        declared.len(),
        IGNORE_DOORS.len(),
        "each `IGNORE_DOORS` entry names its own call site; the table has {} entries but \
         {} distinct sites",
        IGNORE_DOORS.len(),
        declared.len(),
    );
    declared
}

/// **(a)** The registry is the production writer set — no undeclared door, no phantom entry.
#[test]
fn ignore_doors_enumerates_every_production_gitignore_writer() {
    let found = production_ensure_sites();
    let declared = declared_sites();

    let undeclared: Vec<&String> = found.difference(&declared).collect();
    let phantom: Vec<&String> = declared.difference(&found).collect();

    assert!(
        undeclared.is_empty(),
        "every production call to `cli::gitignore::ensure` writes into a file the user \
         co-owns, so it is a door and owes `cli::gitignore::IGNORE_DOORS` an entry \
         stating how it acks that write — named on its own surface, or exempt with a \
         reason the driven arm asserts. Undeclared:\n  {}",
        undeclared
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  "),
    );
    assert!(
        phantom.is_empty(),
        "`IGNORE_DOORS` names a call site that no longer exists — a moved or deleted \
         writer leaves the table describing a door nobody can reach. Phantom:\n  {}",
        phantom
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  "),
    );
}

/// The site detector really discriminates — the fence would catch the mutant it claims to.
#[test]
fn the_ensure_site_detector_separates_a_call_from_a_definition() {
    let sample = "\
pub fn ensure(jigc_root: &Path) -> io::Result<Ensured> { todo!() }

fn a_fifth_door(root: &Path) {
    crate::gitignore::ensure(&root.join(\".jigc\"))?;
    reensure(root);
}

#[cfg(test)]
mod tests {
    fn helper() {
        let _ = crate::gitignore::ensure(&root);
    }
}
";
    let code = code_only(sample);
    let regions = cfg_test_regions(&code);
    let path = Path::new("crates/cli/src/example.rs");
    let sites = ensure_sites_in("crates/cli/src/example.rs", &code, &regions, path);

    assert_eq!(
        sites,
        vec!["crates/cli/src/example.rs::a_fifth_door".to_string()],
        "the detector must find the qualified production call, skip the definition, the \
         glued identifier and the `#[cfg(test)]` one",
    );
    assert!(
        !declared_sites().contains(&sites[0]),
        "the mutant site must be one the registry does not declare, or the red proof \
         would be vacuous",
    );
}

/// Every production call of a named function in `code`, as `<enclosing fn> -> count`.
///
/// Same reading as [`ensure_sites_in`]: production only, comments and string literals
/// blanked, and the function's own `fn <name>(` header is not a call.
fn call_counts(needle: &str, code: &str, regions: &[(usize, usize)], path: &Path) -> Vec<String> {
    let mut owners = Vec::new();
    for (at, _) in code.match_indices(&format!("{needle}(")) {
        if is_test_domain(path, regions, at) {
            continue;
        }
        let head = &code[..at];
        if head.trim_end().ends_with("fn") {
            continue;
        }
        if head
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        if let Some(owner) = enclosing_fn(code, at) {
            owners.push(owner.to_owned());
        }
    }
    owners
}

/// **The finalize row's *other two doors*, fenced rather than asserted.**
///
/// `IGNORE_DOORS`' finalize entry names three doors over one writer: `jigc task finalize`
/// and both `jigc milestone finalize` arms all reach
/// `task::try_execute_finalize_plan`, which is where the amend happens and which prints
/// nothing itself — each caller emits on its own surface. The driven cell above reaches
/// the per-task door; a fourth caller, or an arm that dropped the emitter, would leave
/// the registry claiming coverage nothing checks, and the (a) fence cannot see it
/// (it enumerates *writer* sites, and this is a caller of the door that owns one).
///
/// So the pairing itself is the claim: **in every production function that runs the
/// shared executor, the executor is called exactly as many times as the ack is emitted.**
/// A milestone arm that lost its `emit_ack` reddens here — the count in that one function
/// drops from two to one — and a new executor caller reddens on its first compile-clean
/// commit.
///
/// **Proven red by an applied mutant**: deleting the `emit_ack` call from the
/// `squash: false` chain arm reddens this with *"`crates/cli/src/milestone.rs::
/// run_milestone_finalize` runs the shared finalize executor 2 time(s) and emits the
/// `.jigc/.gitignore` amend ack 1 time(s)"*; restoring it greens.
#[test]
fn every_finalize_executor_call_is_paired_with_the_amend_ack() {
    let root = workspace_root();
    let mut checked = 0usize;
    for path in rust_files(&root.join("crates/cli")) {
        let body = fs::read_to_string(&path).expect("read a cli source");
        let code = code_only(&body);
        let regions = cfg_test_regions(&code);
        let runs = call_counts("try_execute_finalize_plan", &code, &regions, &path);
        if runs.is_empty() {
            continue;
        }
        let acks = call_counts("emit_ack", &code, &regions, &path);
        let rel = path.strip_prefix(&root).unwrap_or(&path).to_string_lossy();
        for owner in BTreeSet::from_iter(runs.iter().cloned()) {
            let ran = runs.iter().filter(|o| **o == owner).count();
            let acked = acks.iter().filter(|o| **o == owner).count();
            assert_eq!(
                ran, acked,
                "`{rel}::{owner}` runs the shared finalize executor {ran} time(s) and \
                 emits the `.jigc/.gitignore` amend ack {acked} time(s) — every door that \
                 reaches the writer owes its own surface the ack \
                 (`cli::gitignore::IGNORE_DOORS`, the finalize row)",
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 2,
        "the sweep must find the executor's production callers; it saw {checked}",
    );
}

// ───────────────────────── (b) one driven cell per member ─────────────────────────

/// The two lines the planted file carries that jigc has no opinion about — the user's
/// own, and what makes this a file the door has to answer for.
const PRIVATE: &str = "# my private stuff\nbuild-cache/\n";

/// The canonical set's **last** entry — the one every cell's planted file lacks, read
/// from [`ENTRIES`] so no fixture transcribes the constant.
fn missing_entry() -> &'static str {
    ENTRIES.lines().next_back().expect("ENTRIES is non-empty")
}

/// Plant a **committed** under-set `.jigc/.gitignore` carrying [`PRIVATE`] — the real
/// upgrade shape: the file jigc's own older build wrote and committed, one entry short
/// of today's set. Committed, because an *uncommitted* modification to it is a different
/// state that `setup`'s install-commit guard refuses outright (`setup.dirty-install-path`),
/// which would measure the guard rather than the ack.
fn plant_committed(corpus: &TrialCorpus) {
    let last = missing_entry();
    let body: String = ENTRIES
        .lines()
        .filter(|line| *line != last)
        .map(|line| format!("{line}\n"))
        .collect::<String>()
        + PRIVATE;
    fs::write(corpus.repo().join(".jigc").join(".gitignore"), body).expect("plant the .gitignore");
    corpus.git(&["add", ".jigc/.gitignore"]);
    corpus.git(&["commit", "-q", "-m", "the entry set an older jigc wrote"]);
}

/// Whether `output` carries a line that names **both** the file and the entry jigc
/// appended to it — the ack under test, wherever the door chooses to print it.
///
/// Deliberately not an equality against the renderer's own output: a test that rebuilds
/// the line from the code under test asserts nothing about what the door printed.
fn names_the_amend(output: &str) -> bool {
    output
        .lines()
        .any(|line| line.contains(".jigc/.gitignore") && line.contains(missing_entry()))
}

/// Assert the door's first run named the amend and its second run said nothing.
fn assert_ack(first: &str, second: &str, door: &str) -> bool {
    assert!(
        !names_the_amend(second),
        "[{door}] a re-run appends nothing, so it must say nothing about \
         `.jigc/.gitignore`; got:\n{second}",
    );
    names_the_amend(first)
}

/// **Cell — `jigc setup`** (`adapter.rs::init_project_layer`).
fn cell_setup() -> bool {
    let corpus = TrialCorpus::build(State::Fresh);
    plant_committed(&corpus);
    let first = corpus.jigc_ok(&["setup"]);
    let second = corpus.jigc_ok(&["setup"]);
    assert_ack(&first, &second, "jigc setup")
}

/// **Cell — `jigc task finalize <id>`** (`task.rs::try_execute_finalize_plan`, inside the
/// commit transaction — the site the two milestone-finalize arms reach too).
fn cell_task_finalize() -> bool {
    let corpus = TrialCorpus::build(State::Fresh);
    plant_committed(&corpus);
    let mut acks = Vec::new();
    for tag in ["one", "two"] {
        let task = corpus.start_workflow("single-task", &format!("Change {tag}"));
        let file = format!("work-{tag}.txt");
        fs::write(corpus.repo().join(&file), "work\n").expect("write the code change");
        corpus.git(&["add", &file]);
        acks.push(corpus.finalize(&task, "core", &format!("change {tag}"), false));
    }
    assert_ack(&acks[0], &acks[1], "jigc task finalize")
}

/// **Cell — `jigc milestone create "<title>"`** (`milestone.rs::run_create`).
fn cell_milestone_create() -> bool {
    let corpus = TrialCorpus::build(State::Fresh);
    plant_committed(&corpus);
    let first = corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    let second = corpus.jigc_ok(&["milestone", "create", "Cache polish"]);
    assert_ack(&first, &second, "jigc milestone create")
}

/// **Cell — `jigc milestone provision <id>`** (`milestone.rs::run_provision`) — the
/// caller that never commits, so what it writes here lives only in the worktree.
fn cell_milestone_provision() -> bool {
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&["milestone", "add-task", "cache-rework", "Area zed"]);
    plant_committed(&corpus);
    let first = corpus.jigc_ok(&["milestone", "provision", "cache-rework"]);
    let second = corpus.jigc_ok(&["milestone", "provision", "cache-rework"]);
    assert_ack(&first, &second, "jigc milestone provision")
}

/// **(b)** Every `IGNORE_DOORS` member has a driven cell, and each cell's observed ack is
/// the disposition the registry declares.
///
/// The dispatch is exhaustive **by panic**: a member the match below does not name aborts
/// the test rather than passing silently. And an `Exempt` member's silence is checked
/// against its *stated* reason being present, so the exemption dimension can never be
/// left open the way a missing row would leave it.
#[test]
fn every_ignore_door_names_what_it_appended() {
    for door in IGNORE_DOORS {
        let observed = match door.site {
            "crates/cli/src/adapter.rs::init_project_layer" => cell_setup(),
            "crates/cli/src/task.rs::try_execute_finalize_plan" => cell_task_finalize(),
            "crates/cli/src/milestone.rs::run_create" => cell_milestone_create(),
            "crates/cli/src/milestone.rs::run_provision" => cell_milestone_provision(),
            other => panic!(
                "`IGNORE_DOORS` member `{other}` ({}) has no driven cell — a new \
                 `.jigc/.gitignore` writer owes one here, or the registry is a \
                 remembered list again",
                door.door,
            ),
        };
        let names = match door.ack {
            Ack::Names(surface) => {
                assert!(
                    !surface.trim().is_empty(),
                    "`{}` declares `Names` with no surface named",
                    door.site,
                );
                true
            }
            Ack::Exempt(reason) => {
                assert!(
                    !reason.trim().is_empty(),
                    "`{}` declares `Exempt` with no reason — an exemption is a stated \
                     disposition, never an absence",
                    door.site,
                );
                false
            }
        };
        assert_eq!(
            observed,
            names,
            "`{}` declares `{}` but the door {} what it appended to `.jigc/.gitignore`",
            door.site,
            if names { "Names" } else { "Exempt" },
            if observed { "named" } else { "did not name" },
        );
    }
}

// ─── (c) an amend only a success needs is made only by a success (M52 Inc 5 / T9) ───

/// The plant both refusal cells write: the canonical set's **first** entry plus the
/// user's own lines, so every other [`ENTRIES`] line is missing and an amend cannot be
/// mistaken for a no-op. Read from the constant, so no fixture transcribes the set.
fn plant_under_set(corpus: &TrialCorpus) -> (PathBuf, String) {
    let first = ENTRIES.lines().next().expect("ENTRIES is non-empty");
    let body = format!("{first}\n{PRIVATE}");
    let path = corpus.repo().join(".jigc").join(".gitignore");
    fs::write(&path, &body).expect("plant the under-set .gitignore");
    (path, body)
}

/// Everything the door said, both streams — a refusal renders on stderr.
fn both_streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Assert `door` refused with `code` and left the planted `.gitignore` byte-identical.
fn assert_refusal_amends_nothing(
    out: &std::process::Output,
    planted: &(PathBuf, String),
    code: &str,
    door: &str,
) {
    let said = both_streams(out);
    assert!(
        !out.status.success() && said.contains(code),
        "[{door}] the cell must reach the `{code}` refusal; got ({}):\n{said}",
        out.status,
    );
    let after = fs::read_to_string(&planted.0).expect("read the .gitignore back");
    assert_eq!(
        after, planted.1,
        "[{door}] a door that refused needs no ignore entry, so it must write none — \
         `.jigc/.gitignore` is the user's file too and the failed run leaves it exactly \
         as it found it (M52 Increment 5 / T9, C4)",
    );
}

/// **`jigc milestone provision <unknown-id>`** refuses without amending — and the same
/// door, on the success it *does* need the entry for, still amends and still says so.
///
/// Driven red at `8f0fb833`: the refusal exited 1 with `milestone.unknown` and five
/// canonical entries appended to the planted file on the way out.
#[test]
fn a_refused_provision_amends_nothing_and_a_successful_one_still_does() {
    let corpus = TrialCorpus::build(State::Fresh);
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);
    corpus.jigc_ok(&["milestone", "add-task", "cache-rework", "Area zed"]);

    let planted = plant_under_set(&corpus);
    let refused = corpus.jigc(&["milestone", "provision", "nonexistent-milestone"]);
    assert_refusal_amends_nothing(
        &refused,
        &planted,
        "milestone.unknown",
        "jigc milestone provision",
    );

    // The control, on the same planted file: the success still needs `worktrees/` ignored,
    // so it still appends it and still names what it appended.
    let ok = corpus.jigc_ok(&["milestone", "provision", "cache-rework"]);
    assert!(
        names_the_amend(&ok),
        "the successful provision still names the amend it made; got:\n{ok}",
    );
    let after = fs::read_to_string(&planted.0).expect("read the .gitignore back");
    for entry in ENTRIES.lines() {
        assert!(
            after.lines().any(|line| line == entry),
            "the successful provision amends to the canonical union; `{entry}` is \
             missing from:\n{after}",
        );
    }
    assert!(
        after.contains(PRIVATE.trim_end()),
        "the amend preserves the user's own lines; got:\n{after}",
    );
}

/// **`jigc milestone create "<a title a record already owns>"`** refuses without
/// amending — the `guard_record_free` arm of the same class.
#[test]
fn a_refused_create_amends_nothing_and_a_successful_one_still_does() {
    let corpus = TrialCorpus::build(State::Fresh);
    // The first create lands the committed record that makes the slug taken.
    corpus.jigc_ok(&["milestone", "create", "Cache rework"]);

    let planted = plant_under_set(&corpus);
    let refused = corpus.jigc(&["milestone", "create", "Cache rework"]);
    assert_refusal_amends_nothing(
        &refused,
        &planted,
        "milestone.record-exists",
        "jigc milestone create",
    );

    // The control: a create that mints still writes into `.jigc/milestones/`, so it still
    // appends the entry and still names it.
    let ok = corpus.jigc_ok(&["milestone", "create", "Cache polish"]);
    assert!(
        names_the_amend(&ok),
        "the successful create still names the amend it made; got:\n{ok}",
    );
}
