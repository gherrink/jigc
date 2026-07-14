//! Regression: a repeatable-item field declared `set: on-create` must be
//! materialized when `jigc doc add-item` mints the entry — mirroring how a
//! doc-level `set: on-create` field is stamped at `create`.
//!
//! The live example is the methodology pack's `deferral-ledger` (and
//! `decisions-log`): each per-entry block declares `{ id: date, type: date,
//! set: on-create }`. Before the fix, `add-item` minted the entry with no date
//! — the schema's "CLI-set on create" promise was inert for the running-doc
//! entries. This test drives the real binary over the methodology pack and
//! asserts the minted entry carries a populated `date` field.
//!
//! Every assertion runs over the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`): `JIGC_PACK_DIR=<methodology> jigc setup` then
//! `jigc start --workflow planning` (whose create-gate admits `deferral-ledger`)
//! → `doc create` → `doc add-item`, reading back the staged entry.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-add-item-on-create-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        path.push(unique);
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The embedded dev pack tree on disk (selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships) — carries the `changelog` doctype whose
/// `unreleased-changes` repeatable is keyed by an `enum` id-from (`category`).
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
}

/// The staged singleton body for `<type>:<type>.md` in the task working area.
fn staged_singleton(repo: &Path, task: &str, ty: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("{ty}:{ty}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

#[test]
fn add_item_materializes_an_on_create_date_on_the_deferral_ledger_entry() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );

    // The `planning` workflow's create-gate admits `deferral-ledger`.
    let start = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "plan the next milestone"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow planning` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );
    let task = "plan-the-next-milestone";

    let create = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "create",
            "deferral-ledger",
            "--title",
            "Deferral-Ledger",
        ],
    );
    assert!(
        create.status.success(),
        "`jigc doc create deferral-ledger` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr),
    );

    // Mint one entry. `add-item` emits the minted item address on stdout.
    let add = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "add-item",
            "deferral-ledger:deferral-ledger#entries",
            "--title",
            "Spec support deferred",
        ],
    );
    assert!(
        add.status.success(),
        "`jigc doc add-item …#entries` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&add.stderr),
    );

    // Read the staged body back and assert the minted entry carries a populated
    // `date` field — the `set: on-create` promise. Before the fix the entry has
    // no `date:` bullet at all (the absence is the red).
    let staged = staged_singleton(repo.path(), task, "deferral-ledger");
    let entry_date = staged
        .lines()
        .find_map(|l| l.trim_start().strip_prefix("- date:"))
        .map(str::trim)
        .unwrap_or_else(|| {
            panic!("the minted entry must carry a `date:` field; staged:\n{staged}")
        });
    assert!(
        !entry_date.is_empty(),
        "the minted entry's `date` field must be populated on create (set: on-create); \
         staged:\n{staged}",
    );
    // The stamped value is an ISO `YYYY-MM-DD` date (the `date` field shape the
    // engine's value-conformance check accepts).
    let ymd: Vec<&str> = entry_date.split('-').collect();
    assert_eq!(
        ymd.len(),
        3,
        "the on-create date is an ISO `YYYY-MM-DD`; got {entry_date:?}",
    );
    assert!(
        ymd[0].len() == 4 && ymd[0].chars().all(|c| c.is_ascii_digit()),
        "the on-create date's year is four digits; got {entry_date:?}",
    );
}

/// M24 inc-2 T1 — a **top-level** `add-item` whose `--title` re-slugs OUTSIDE the
/// repeatable's `id-from` enum is rejected **at the write verb** (non-zero exit), not
/// deferred to finalize. `Improvements` ∉ {added, changed, …} for the changelog's
/// `unreleased-changes` change-groups, so the mint blocks with the SHARED finalize code
/// `schema-conformance.field-value-conformant` naming the slug-cased id-from address;
/// a valid member `Fixed`→`fixed` passes. Driven over the real binary against the
/// shipped dev pack (the bytes that ship), so the block is the emitted contract.
#[test]
fn top_level_foreign_category_blocks_at_the_add_item_verb() {
    let repo = TempDir::new("toplevel-repo");
    let home = TempDir::new("toplevel-home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );

    // `record-change` is off-router; start it by name (it admits `changelog`).
    let start = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "record-change", "cut the release"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow record-change` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );

    let create = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "create", "changelog", "--title", "Changelog"],
    );
    assert!(
        create.status.success(),
        "`jigc doc create changelog` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr),
    );

    // The foreign category — blocks at the write verb. `--format json` so the emitted
    // finding envelope is parseable.
    let blocked = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "add-item",
            "changelog:changelog#unreleased-changes",
            "--title",
            "Improvements",
            "--format",
            "json",
        ],
    );
    assert!(
        !blocked.status.success(),
        "a foreign id-from enum category must block at the add-item verb (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("stderr is JSON: {e}; got:\n{stderr}"));
    let finding = &report["findings"][0];
    assert_eq!(
        finding["code"], "schema-conformance.field-value-conformant",
        "the block carries the SHARED finalize code; got:\n{stderr}",
    );
    let address = finding["location"]["address"]
        .as_str()
        .unwrap_or_else(|| panic!("the finding is addressed; got:\n{stderr}"));
    // (M42 inc-9 T2) The slug-cased id-from address in **URI normal form** — the bare
    // fragment this pinned before is not a stable key (two docs' foreign categories collided
    // on it); `design/command-output-contract.md` → the `write.*` row.
    assert_eq!(
        address, "changelog:changelog#unreleased-changes/improvements/category",
        "the finding names the slug-cased id-from address, doc-qualified; got {address:?}",
    );
    assert_eq!(
        finding["key"]["target"], address,
        "the stable key.target IS that address; got:\n{stderr}",
    );

    // A valid member passes — `Fixed` re-slugs to the `fixed` enum member.
    let ok = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "add-item",
            "changelog:changelog#unreleased-changes",
            "--title",
            "Fixed",
        ],
    );
    assert!(
        ok.status.success(),
        "a valid enum-member category (`Fixed`→`fixed`) must pass; stderr:\n{}",
        String::from_utf8_lossy(&ok.stderr),
    );
    assert_eq!(
        String::from_utf8_lossy(&ok.stdout).trim_end_matches('\n'),
        "changelog:changelog#unreleased-changes/fixed",
        "the valid mint emits the minted item address",
    );
}
