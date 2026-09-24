//! The **mint-door axis** — every production call that opens a working area, and what
//! it does about the carryover gate's staged snapshot
//! (`engine::state::MINT_DOORS`; `design/surface-contract.md` → The carryover gate).
//!
//! ## The class this closes
//!
//! `engine::state::write_staged_snapshot`'s doc-comment said the snapshot is *"written
//! at every task-minting door"* and then named three. Five production sites mint a
//! working area. The two it never named — `engine::milestone::add_task` and
//! `engine::milestone::reseed_sub_task_areas` — are sub-task mints, and a sub-task's
//! area really does carry no `staged-snapshot.json`: driven with a foreign file staged
//! before the mint, `jigc milestone add-task` leaves no snapshot behind while the
//! `jigc start` control leaves one and blocks `finalize.carried-staged` on it.
//!
//! That is an **exemption, not a hole** — the per-task finalize refuses a sub-task
//! before any gate runs (`finalize.milestone-sub-task`), so no committing door consumes
//! a sub-task's snapshot, and the aggregate boundary reads the *milestone* area's. But
//! the difference between an exemption and an oversight is that an exemption is
//! **stated**, and this one was not: the M49 Settle refuted the carryover standing
//! order and recorded the residue as *no `MINT_DOORS` enumeration fence*
//! (`completions/artifacts/M49/settle-record.md` → Corrections this Settle put on the
//! record).
//!
//! ## Two arms, and why neither is a list
//!
//! **(a) The completeness fence** reads both crates' **production** source and asserts
//! the call-site set of `engine::state::mint_task` ∪ `engine::milestone::mint_milestone`
//! equals `MINT_DOORS`' declared `site` set — the `temp_mint_fence.rs` mold, over the
//! shared scanner in [`support::rust_source`](crate::support::rust_source). A grep is
//! what *found* these five; it cannot stop the sixth
//! (`implementation/dev-workflow.md` → *a grep is not a fence*), so membership is
//! checked where membership is decided.
//!
//! **(b) One driven cell per member**, through the real binary over a planted pre-staged
//! `foreign.txt`: a `Written` member's area must carry `staged-snapshot.json` and its
//! boundary must block `finalize.carried-staged` naming that path; an `Exempt` member's
//! stated premise must hold when driven. A member with no cell is a **hard panic**, not
//! a skip — a registry whose new member silently ran no test would be the remembered
//! list again, wearing a table's clothes.

use crate::support::rust_source::{cfg_test_regions, code_only, enclosing_fn, is_test_domain};
use engine::state::{MINT_DOORS, Snapshot};
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// ───────────────────────── (a) the completeness fence ─────────────────────────

/// The workspace root — two levels up from `crates/cli`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("canonicalize the workspace root")
}

/// The two mints whose call sites the registry must enumerate.
const MINTS: [&str; 2] = ["mint_task", "mint_milestone"];

/// Every production call site of [`MINTS`] in `code`, as `<rel-path>::<enclosing fn>`.
///
/// *Production* is everything outside the test domain — a `tests/` tree, or a
/// `#[cfg(test)]` module body — read off source with comments and string literals
/// blanked, so a doc-comment naming the mint is not a call and a `{` inside a string
/// cannot throw the module brace-matching off.
///
/// The mint's **own definition** is not a call: `fn mint_task(` is skipped by the token
/// before it, which is also what keeps the fence honest if the definitions move.
fn mint_sites_in(rel: &str, code: &str, regions: &[(usize, usize)], path: &Path) -> Vec<String> {
    let mut sites = Vec::new();
    for mint in MINTS {
        let needle = format!("{mint}(");
        for (at, _) in code.match_indices(&needle) {
            if is_test_domain(path, regions, at) {
                continue;
            }
            // The definition itself, `fn mint_task(…)`, is not a call site.
            if code[..at].trim_end().ends_with("fn") {
                continue;
            }
            let owner = enclosing_fn(code, at)
                .unwrap_or_else(|| panic!("a call to `{mint}` sits inside some fn ({rel})"));
            sites.push(format!("{rel}::{owner}"));
        }
    }
    sites
}

/// Every production call site of the two mints across both member crates.
fn production_mint_sites() -> BTreeSet<String> {
    let root = workspace_root();
    let mut sites = BTreeSet::new();
    let mut files = 0usize;
    for crate_dir in ["crates/engine", "crates/cli"] {
        for path in crate::support::rust_source::rust_files(&root.join(crate_dir)) {
            // The `doc-code` probe is a detached workspace that cannot depend on `engine`.
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
            sites.extend(mint_sites_in(&rel, &code, &regions, &path));
        }
    }
    assert!(
        files > 100,
        "the sweep must find the workspace sources; it saw only {files}",
    );
    sites
}

/// The declared site set — every production mint the registry names.
///
/// Duplicate `site` values would let one entry stand for two real calls with two
/// different dispositions, so the set must be as large as the table.
fn declared_sites() -> BTreeSet<String> {
    let declared: BTreeSet<String> = MINT_DOORS.iter().map(|d| d.site.to_string()).collect();
    assert_eq!(
        declared.len(),
        MINT_DOORS.len(),
        "each `MINT_DOORS` entry names its own call site; the table has {} entries but \
         {} distinct sites",
        MINT_DOORS.len(),
        declared.len(),
    );
    declared
}

/// **(a)** The registry is the production mint set — no undeclared door, no phantom entry.
///
/// Proven red by an applied mutant: a sixth production `state::mint_task(…)` call with
/// no table entry reddens this as `undeclared`, which is the only thing that makes the
/// table load-bearing rather than decorative.
#[test]
fn mint_doors_enumerates_every_production_working_area_mint() {
    let found = production_mint_sites();
    let declared = declared_sites();

    let undeclared: Vec<&String> = found.difference(&declared).collect();
    let phantom: Vec<&String> = declared.difference(&found).collect();

    assert!(
        undeclared.is_empty(),
        "every production call to `engine::state::mint_task` / \
         `engine::milestone::mint_milestone` opens a working area, so it is a mint door \
         and owes `engine::state::MINT_DOORS` an entry stating its snapshot disposition \
         — written, or exempt with a reason the driven arm asserts. Undeclared:\n  {}",
        undeclared
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  "),
    );
    assert!(
        phantom.is_empty(),
        "`MINT_DOORS` names a call site that no longer exists — a moved or deleted mint \
         leaves the table describing a door nobody can reach. Phantom:\n  {}",
        phantom
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  "),
    );
}

/// The site detector really discriminates — the fence would catch the mutant it claims to.
///
/// A fence whose detector saw nothing, or that counted the mint's own `fn` header as a
/// call, would pass forever. Both directions are flipped here against source text rather
/// than assumed.
#[test]
fn the_site_detector_separates_a_call_from_a_definition() {
    let sample = "\
pub fn mint_task(jigc_root: &Path) -> Result<MintedTask, Finding> { todo!() }

fn a_sixth_door(root: &Path) {
    let minted = state::mint_task(root, \"\", \"commit\", \"single-task\", base, None);
}

#[cfg(test)]
mod tests {
    fn helper() {
        let _ = state::mint_task(root, \"\", \"commit\", \"single-task\", base, None);
    }
}
";
    let code = code_only(sample);
    let regions = cfg_test_regions(&code);
    let path = Path::new("crates/cli/src/example.rs");
    let sites = mint_sites_in("crates/cli/src/example.rs", &code, &regions, path);

    assert_eq!(
        sites,
        vec!["crates/cli/src/example.rs::a_sixth_door".to_string()],
        "the detector must find the production call, skip the definition, and skip the \
         `#[cfg(test)]` one",
    );
    assert!(
        !declared_sites().contains(&sites[0]),
        "the mutant site must be one the registry does not declare, or the red proof \
         would be vacuous",
    );
}

// ───────────────────────── (b) one driven cell per member ─────────────────────────

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-mint-doors-{tag}-{}-{:?}",
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
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// A real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The planted foreign path — staged **before** every mint under test.
const FOREIGN: &str = "foreign.txt";

/// Plant `foreign.txt` and stage it, so every mint below runs over a genuinely
/// pre-staged foreign entry.
fn plant_foreign(repo: &Path) {
    fs::write(repo.join(FOREIGN), "not this work-unit's work\n").expect("write the foreign file");
    git(repo, &["add", FOREIGN]);
}

/// The blocked run's pinned findings envelope (stdout, `--format json`), asserting the
/// validation exit (3).
fn blocked_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    assert_eq!(
        out.status.code(),
        Some(3),
        "`{what}` must block with the validation exit (3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` blocks with the pinned findings envelope on stdout ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// Assert the envelope blocks on exactly one `finalize.carried-staged`, keyed at the
/// planted path — the `Written` disposition's observable consequence.
fn assert_blocks_on_the_plant(out: &std::process::Output, what: &str) {
    let findings = blocked_findings(out, what);
    let carried: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| f["code"] == "finalize.carried-staged")
        .collect();
    assert_eq!(
        carried.len(),
        1,
        "`{what}` blocks on the one planted carried path; got:\n{findings:#?}"
    );
    assert_eq!(
        carried[0]["severity"], "blocking",
        "the carryover refusal is blocking; got: {}",
        carried[0]
    );
    assert_eq!(
        carried[0]["key"]["target"], FOREIGN,
        "the refusal names the planted path; got: {}",
        carried[0]
    );
}

/// The `staged-snapshot.json` a `Written` door leaves behind, parsed.
fn snapshot_at(area: &Path) -> serde_json::Value {
    let body = fs::read_to_string(area.join("staged-snapshot.json")).unwrap_or_else(|e| {
        panic!(
            "a `Written` mint door must leave a staged snapshot in `{}` ({e})",
            area.display()
        )
    });
    serde_json::from_str(&body).expect("the snapshot parses")
}

/// Assert an area carries a snapshot naming the planted path.
fn assert_snapshot_names_the_plant(area: &Path) {
    let snapshot = snapshot_at(area);
    assert!(
        snapshot["entries"].get(FOREIGN).is_some(),
        "the snapshot must carry the pre-mint staged entry; got:\n{snapshot:#?}"
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    for (field, value) in [("type", "feat"), ("scope", "gate")] {
        ok(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#{field}"),
                "--value",
                value,
            ],
            "doc set-field",
        );
    }
    for (slot, prose) in [
        ("summary", &b"enumerate the mint doors\n"[..]),
        ("body", &b"An M49 change.\n"[..]),
    ] {
        let addr = format!("commit:{task}#{slot}");
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", &addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    }
}

/// The `[dev ▸ methodology]` compose marker — a milestone boundary needs the record home.
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Stage a persisted doc + its provenance bit into a sub-task's area — the milestone's
/// real work, without which the zero-contribution refusal blocks ahead of the gate under
/// test (the `carryover_gate.rs` fixture's lesson).
fn stage_subtask_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write staged body");

    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize manifest"),
    )
    .expect("write provenance manifest");
}

/// The authored sub-task ADR body the milestone boundary lands.
const SUB_TASK_ADR: &str = "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# Warm policy\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n";

/// `.jigc/tasks/<id>` under `repo`.
fn task_area(repo: &Path, id: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(id)
}

/// Whether the working area at `area` carries a staged snapshot — the on-disk
/// measurement each cell returns, and what the driver checks the declared
/// [`Snapshot`] disposition against.
fn carries_a_snapshot(area: &Path) -> bool {
    area.join("staged-snapshot.json").exists()
}

/// **Cell — `jigc start "<intent>"`** (`Written`): the minted task area carries the
/// snapshot, and `jigc task finalize` blocks `finalize.carried-staged` on the plant.
fn cell_start(repo: &Path, home: &Path) -> bool {
    init_repo(repo);
    ok(repo, home, &["setup"], "jigc setup");
    plant_foreign(repo);

    let task = "enumerate-the-mint-doors";
    ok(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "enumerate the mint doors",
        ],
        "jigc start",
    );
    let area = task_area(repo, task);
    assert_snapshot_names_the_plant(&area);

    fs::write(repo.join("feature.rs"), "pub fn work() {}\n").expect("write the task edit");
    git(repo, &["add", "feature.rs"]);
    fill_commit(repo, home, task);
    assert_blocks_on_the_plant(
        &jigc(
            repo,
            home,
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged plant)",
    );
    carries_a_snapshot(&area)
}

/// **Cell — `jigc task amend ["<intent>"]`** (`Written`): the amend task's area carries the
/// snapshot, and the snapshot names the pre-mint plant.
///
/// **It does not drive the carryover refusal, and that is the cell's point.** The amend arm
/// is the one `Written` door whose finalize is *exempt* from the carryover gate — it stages
/// nothing, and its own `finalize.amend-index-dirty` refusal is stricter (it refuses the
/// whole index, declared or not, with no `--carry-staged` to wave it through). So what this
/// proves is exactly what `Snapshot::Written` claims — the snapshot is on disk and names the
/// plant — while the refusal over that same plant is driven where it lives, on the finalize
/// arm's own axis (`crates/cli/tests/task_amend.rs`).
fn cell_amend(repo: &Path, home: &Path) -> bool {
    init_repo(repo);
    ok(repo, home, &["setup"], "jigc setup");
    plant_foreign(repo);

    ok(
        repo,
        home,
        &["task", "amend", "repair the mint-door commit message"],
        "jigc task amend",
    );
    let area = task_area(repo, "repair-the-mint-door-commit");
    assert_snapshot_names_the_plant(&area);
    carries_a_snapshot(&area)
}

/// **Cell — `jigc migrate <path> --as <doctype>`** (`Written`): the migration task's own
/// area carries the snapshot, and its finalize refuses the carryover independently of
/// the fidelity `--approve`.
fn cell_migrate(repo: &Path, home: &Path) -> bool {
    init_repo(repo);
    fs::write(
        repo.join("HISTORY.md"),
        "# Changelog\n\n## [0.1.0] - 2021-03-09\n### Added\n- First public release.\n",
    )
    .expect("write HISTORY.md");
    git(repo, &["add", "HISTORY.md"]);
    git(repo, &["commit", "-q", "-m", "track HISTORY.md"]);
    ok(repo, home, &["setup"], "jigc setup");
    plant_foreign(repo);

    ok(
        repo,
        home,
        &["migrate", "HISTORY.md", "--as", "changelog"],
        "jigc migrate HISTORY.md --as changelog",
    );
    let task = "migrate-changelog-history-3268e06b69e1";
    let area = task_area(repo, task);
    assert_snapshot_names_the_plant(&area);

    let payload = "title: Changelog\nsections:\n  - id: releases\n    items:\n      - title: 0.1.0\n        set:\n          date: 2021-03-09\n        sections:\n          - id: changes\n            items:\n              - title: Added\n                set:\n                  notes: \"<<- First public release.>>\"\n";
    let out = jigc(
        repo,
        home,
        &[
            "doc",
            "author",
            "changelog",
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(payload.as_bytes()),
    );
    assert!(
        out.status.success(),
        "`jigc doc author changelog` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // `--approve` declares the fidelity diff and nothing else — the carryover refusal
    // is the gate under test, reached past the review hold.
    assert_blocks_on_the_plant(
        &jigc(
            repo,
            home,
            &["task", "finalize", task, "--approve", "--format", "json"],
            None,
        ),
        "jigc task finalize --approve (pre-mint staged plant)",
    );
    carries_a_snapshot(&area)
}

/// The milestone fixture every milestone cell shares: a `[dev ▸ methodology]` repo whose
/// milestone was created **after** the plant was staged, with one sub-task and its
/// authored doc as the boundary's real work (a boundary that would land nothing is
/// refused ahead of the gate under test).
fn milestone_fixture(repo: &Path, home: &Path) {
    init_repo(repo);
    write_compose_marker(repo);
    plant_foreign(repo);
    ok(
        repo,
        home,
        &["milestone", "create", "Cache rework"],
        "jigc milestone create",
    );
    ok(
        repo,
        home,
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Warm the read cache",
        ],
        "jigc milestone add-task",
    );
    stage_subtask_doc(repo, "warm-the-read-cache", "adr:warm-policy", SUB_TASK_ADR);
}

/// The milestone's own working area.
fn milestone_area(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join("cache-rework")
}

/// **Cell — `jigc milestone create "<title>"`** (`Written`): the milestone area carries
/// the snapshot, and `jigc milestone finalize` — the boundary that consumes it — blocks
/// `finalize.carried-staged` on the plant.
fn cell_milestone_create(repo: &Path, home: &Path) -> bool {
    milestone_fixture(repo, home);
    assert_snapshot_names_the_plant(&milestone_area(repo));
    assert_blocks_on_the_plant(
        &jigc(
            repo,
            home,
            &["milestone", "finalize", "cache-rework", "--format", "json"],
            None,
        ),
        "jigc milestone finalize (pre-create staged plant)",
    );
    carries_a_snapshot(&milestone_area(repo))
}

/// Assert an exempt sub-task area's stated premise, through the binary: the per-task
/// finalize refuses it as a sub-task *before any gate could consume a snapshot*. That
/// refusal is the whole exemption — where the gating boundary reads its snapshot from
/// is each cell's own second half.
fn assert_sub_task_exemption_holds(repo: &Path, home: &Path, sub: &str) {
    let area = task_area(repo, sub);
    assert!(
        area.is_dir(),
        "the sub-task area must exist at `{}`",
        area.display()
    );

    // The premise: no committing door consumes a sub-task's snapshot, because the
    // per-task finalize refuses a sub-task first.
    let out = jigc(
        repo,
        home,
        &["task", "finalize", sub, "--format", "json"],
        None,
    );
    let findings = blocked_findings(&out, "jigc task finalize <sub-task>");
    assert!(
        findings
            .iter()
            .any(|f| f["code"] == "finalize.milestone-sub-task"),
        "the per-task finalize must refuse a sub-task as such — that refusal IS the \
         exemption's premise; got:\n{findings:#?}"
    );
}

/// **Cell — `jigc milestone add-task`** (`Exempt`): the sub-task area carries no
/// snapshot, and the stated reason holds when driven.
fn cell_add_task(repo: &Path, home: &Path) -> bool {
    milestone_fixture(repo, home);
    assert_sub_task_exemption_holds(repo, home, "warm-the-read-cache");
    // The second half of the stated reason: the boundary that DOES gate reads the
    // milestone area's snapshot, which is what makes the sub-task's absence a
    // disposition rather than a gap.
    assert_snapshot_names_the_plant(&milestone_area(repo));
    carries_a_snapshot(&task_area(repo, "warm-the-read-cache"))
}

/// **Cell — the fresh-clone re-seed** (`Exempt`): a milestone op that rebuilds a
/// sub-task area from the committed record mints through the same exempt path, so the
/// rebuilt area carries no snapshot either — and the same premise holds over it.
fn cell_reseed(repo: &Path, home: &Path) -> bool {
    milestone_fixture(repo, home);

    // The fresh clone: the gitignored workbench is gone, the committed record remains.
    fs::remove_dir_all(repo.join(".jigc").join("tasks")).expect("drop the task areas");
    fs::remove_dir_all(repo.join(".jigc").join("milestones")).expect("drop the milestone cache");
    assert!(!task_area(repo, "warm-the-read-cache").exists());

    // A milestone op that *operates* re-seeds both halves from the committed record.
    ok(
        repo,
        home,
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Evict cold entries",
        ],
        "jigc milestone add-task (fresh clone)",
    );
    stage_subtask_doc(repo, "warm-the-read-cache", "adr:warm-policy", SUB_TASK_ADR);
    assert_sub_task_exemption_holds(repo, home, "warm-the-read-cache");
    // This cell's second half differs from `add-task`'s, and honestly: the milestone
    // area's snapshot went with the gitignored workbench, so the re-seed rebuilds a
    // milestone that has none. That is the **declared fail-open bound**, not a
    // regression, and it is already pinned as such
    // (`carryover_gate.rs::milestone_missing_snapshot_fails_open`) — asserted here only
    // to keep this cell from claiming the `add-task` cell's stronger fact.
    assert!(
        !carries_a_snapshot(&milestone_area(repo)),
        "a fresh clone's rebuilt milestone area carries no snapshot — the declared \
         fail-open bound"
    );
    carries_a_snapshot(&task_area(repo, "warm-the-read-cache"))
}

/// **(b)** Every `MINT_DOORS` member has a driven cell, and each cell's on-disk result
/// is the disposition the registry declares.
///
/// The dispatch is exhaustive **by panic**: a member the match below does not name
/// aborts the test rather than passing silently, which is the difference between a
/// registry and the remembered list it replaces. And the declared `Snapshot` is checked
/// against what the cell measured on disk, so a mis-declared disposition reddens here
/// rather than reading as documentation.
#[test]
fn every_mint_door_is_driven_over_a_pre_staged_foreign_file() {
    for (index, door) in MINT_DOORS.iter().enumerate() {
        let repo = TempDir::new(&format!("cell{index}"));
        let home = TempDir::new(&format!("home{index}"));
        let observed = match door.site {
            "crates/cli/src/start.rs::mint_in_repo" => cell_start(repo.path(), home.path()),
            "crates/cli/src/start.rs::mint_migration_in_repo" => {
                cell_migrate(repo.path(), home.path())
            }
            "crates/cli/src/start.rs::mint_amend_in_repo" => cell_amend(repo.path(), home.path()),
            "crates/cli/src/milestone.rs::run_create" => {
                cell_milestone_create(repo.path(), home.path())
            }
            "crates/engine/src/milestone.rs::add_task" => cell_add_task(repo.path(), home.path()),
            "crates/engine/src/milestone.rs::reseed_sub_task_areas" => {
                cell_reseed(repo.path(), home.path())
            }
            other => panic!(
                "`MINT_DOORS` member `{other}` ({}) has no driven cell — a new mint door \
                 owes one here, or the registry is a remembered list again",
                door.door,
            ),
        };
        let declared = matches!(door.snapshot, Snapshot::Written);
        assert_eq!(
            observed,
            declared,
            "`{}` declares `{}` but the area it minted {} a staged snapshot on disk",
            door.site,
            if declared { "Written" } else { "Exempt" },
            if observed { "carries" } else { "carries no" },
        );
    }
}
