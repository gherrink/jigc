//! M23 Increment 4, T1 — the flow-25 marquee: the full migrate → review →
//! `--approve` → adopt path proven end to end against the **rebuilt** `jigc`
//! binary, over `git init` temp repos against the **shipped** dev pack.
//!
//! This is the milestone's headline e2e ([worked-examples.md](../../../design/worked-examples.md)
//! → flow 25; [auto-migration.md](../../../design/auto-migration.md) → Acceptance). It
//! consolidates the whole transform arm into one binary walk plus the two reds, none
//! of which the Inc 1–3 slice tests cover through the full migrate→author→finalize path:
//!   1. the **consolidated walk** — `jigc migrate HISTORY.md --as changelog` over a
//!      real multi-release foreign file, the author spine (create / add-item / set-field
//!      historical date / set-slot), then `finalize` WITHOUT `--approve` blocks (exit
//!      `EXIT_REVIEW_PENDING` = 4, renders the fidelity diff, no commit, foreign
//!      byte-intact, nothing adopted), then `finalize --approve` writes
//!      `CHANGELOG.md` byte-stable, retires the foreign original (gone), lands
//!      exactly ONE commit carrying the added doc + the deletion, and a follow-up
//!      `jigc ingest` reports it adopted;
//!   2. **RED 1** — a `Performance` change-group is rejected **at the `add-item` write
//!      verb** (M24 inc-2 moved the id-from enum to write time): non-zero exit, the
//!      finding code `schema-conformance.field-value-conformant` naming the slug-cased
//!      `…/changes/performance/category` address, with no commit / retire / adopt. The
//!      reject fires before the bad group is ever authored, so finalize is never reached
//!      (the finalize-time enum check still backstops an out-of-band foreign category —
//!      the engine `validate.rs` unit tests cover that path);
//!   3. **RED 2** — `finalize` without `--approve` leaves the foreign original
//!      byte-intact AND the persisted edge index unchanged (the byte-safe, state-safe
//!      human-reject path).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow25marquee-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
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
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`.
fn run_jigc(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The off-router migration task id — `migrate` mints a per-file `migrate-<doctype>-<slug(path)>` (the empty
/// intent slugs the `migrate-` id-source fallback), keeping the bare `changelog`
/// namespace free (`auto-migration.md` -> Hardening #9).
const TASK: &str = "migrate-changelog-history-3268e06b69e1";

/// The shipped changelog schema, loaded for the byte-stable round-trip assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped changelog schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// A real multi-release foreign Keep-a-Changelog file (the headline migration input).
const FOREIGN: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2023-01-15
### Added
- Device-code OAuth flow.
### Fixed
- Session fixation on logout.

## [1.1.0] - 2022-08-01
### Changed
- Bumped the default timeout to 30s.
";

/// `doc set-field <addr> --value <value>` against the migration task.
fn set_field(repo: &Path, home: &Path, pack: &Path, addr: &str, value: &str) {
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "set-field", addr, "--value", value, "--task", TASK],
            None,
        ),
        "set-field",
    );
}

/// `doc set-slot <addr> --from-file -` against the migration task, piping `prose`.
fn set_slot(repo: &Path, home: &Path, pack: &Path, addr: &str, prose: &[u8]) {
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "set-slot", addr, "--from-file", "-", "--task", TASK],
            Some(prose),
        ),
        "set-slot",
    );
}

/// `doc add-item <parent> --title <title>` against the migration task, driving the
/// emitted address verbatim.
fn add_item(repo: &Path, home: &Path, pack: &Path, parent: &str, title: &str) -> String {
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "add-item", parent, "--title", title, "--task", TASK],
            None,
        ),
        "add-item",
    )
}

/// Add a release + author its **historical** date (OVERWRITING the on-create
/// today-stamp), returning the emitted release address.
fn add_release(repo: &Path, home: &Path, pack: &Path, version: &str, date: &str) -> String {
    let release = add_item(repo, home, pack, "changelog:changelog#releases", version);
    set_field(repo, home, pack, &format!("{release}/date"), date);
    release
}

/// Add a nested change-group under `parent` (`<release>/changes`) + author its notes.
fn add_group(repo: &Path, home: &Path, pack: &Path, parent: &str, category: &str, notes: &[u8]) {
    let group = add_item(repo, home, pack, parent, category);
    set_slot(repo, home, pack, &format!("{group}/notes"), notes);
}

/// Fill every author-required field/slot of the provisioned commit doc for the task.
fn make_commit_conformant(repo: &Path, home: &Path, pack: &Path) {
    set_field(repo, home, pack, &format!("commit:{TASK}#type"), "feat");
    set_field(
        repo,
        home,
        pack,
        &format!("commit:{TASK}#scope"),
        "changelog",
    );
    set_slot(
        repo,
        home,
        pack,
        &format!("commit:{TASK}#summary"),
        b"adopt the migrated changelog\n",
    );
    set_slot(
        repo,
        home,
        pack,
        &format!("commit:{TASK}#body"),
        b"Migrate the foreign HISTORY.md into managed shape.\n",
    );
}

/// Write + **commit** the foreign original (so its retirement lands as a tracked
/// deletion), then drive the migrate + author spine to a conformant staged
/// `changelog:changelog` over [`FOREIGN`] plus a conformant commit doc. Returns the
/// emitted address of the `1.2.0` release (RED 1 hangs its `Performance` group off it).
fn drive_conformant_migration(repo: &Path, home: &Path, pack: &Path) -> String {
    // Track the foreign original FIRST — the realistic flow-25 scenario (a pre-existing
    // committed `HISTORY.md`), so `--approve`'s retire surfaces a real deletion.
    fs::write(repo.join("HISTORY.md"), FOREIGN).expect("write foreign HISTORY.md");
    git(repo, &["add", "HISTORY.md"]);
    git(repo, &["commit", "-q", "-m", "track foreign changelog"]);

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");

    // The migrate verb mints the off-router task + composes the foreign content into the
    // emitted view through the source seam.
    let composed = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate HISTORY.md --as changelog",
    );
    assert!(
        composed.contains(FOREIGN.trim_end()),
        "the composed migrate workflow surfaces the foreign content through the source seam; \
         stdout:\n{composed}",
    );

    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                TASK,
            ],
            None,
        ),
        "doc create changelog",
    );

    // Author each release oldest-to-newest with its historical date + nested groups.
    let rel_110 = add_release(repo, home, pack, "1.1.0", "2022-08-01");
    add_group(
        repo,
        home,
        pack,
        &format!("{rel_110}/changes"),
        "Changed",
        b"Bumped the default timeout to 30s.\n",
    );

    let rel_120 = add_release(repo, home, pack, "1.2.0", "2023-01-15");
    add_group(
        repo,
        home,
        pack,
        &format!("{rel_120}/changes"),
        "Added",
        b"Device-code OAuth flow.\n",
    );
    add_group(
        repo,
        home,
        pack,
        &format!("{rel_120}/changes"),
        "Fixed",
        b"Session fixation on logout.\n",
    );

    make_commit_conformant(repo, home, pack);
    rel_120
}

/// The persisted committed edge index's **edge set** (`.jigc/index/edges.json` →
/// `edges`), defaulting to empty when no index file exists. Comparing this across a
/// blocked review proves no migration edge leaked into the committed index — the
/// semantic "unchanged", not the cache file's mere materialization (validation may
/// lazily rebuild the empty index keyed to the unchanged HEAD).
fn committed_edges(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("index").join("edges.json");
    let Ok(bytes) = fs::read(&path) else {
        return Vec::new();
    };
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("edges.json is valid JSON");
    json["edges"].as_array().cloned().unwrap_or_default()
}

/// The marquee: migrate → review-block → `--approve` → adopt, one binary walk.
#[test]
fn flow25_migrate_review_approve_adopt_walk() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    drive_conformant_migration(repo.path(), home.path(), &pack);

    // ── Review gate: finalize WITHOUT --approve blocks (exit 4), commits nothing ──
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let foreign_before = fs::read(repo.path().join("HISTORY.md")).expect("read foreign before");

    let gated = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );
    assert_eq!(
        gated.status.code(),
        Some(4),
        "finalize without --approve on a migration task exits 4 (review-pending); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&gated.stdout),
        String::from_utf8_lossy(&gated.stderr),
    );
    let diff = format!(
        "{}{}",
        String::from_utf8_lossy(&gated.stdout),
        String::from_utf8_lossy(&gated.stderr),
    );
    assert!(
        diff.contains("## [1.2.0] - 2023-01-15"),
        "the fidelity diff surfaces the foreign source bytes; got:\n{diff}"
    );
    assert!(
        diff.contains("CHANGELOG.md"),
        "the fidelity diff names the canonical rewrite destination; got:\n{diff}"
    );
    assert!(
        diff.contains("--approve"),
        "the block tells the human how to approve; got:\n{diff}"
    );
    // No commit on the gated branch; foreign byte-intact; nothing adopted.
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "a blocked review leaves HEAD unchanged"
    );
    assert_eq!(
        foreign_before,
        fs::read(repo.path().join("HISTORY.md")).expect("read foreign after block"),
        "a blocked review leaves the foreign original byte-intact"
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "a blocked review adopts nothing (no canonical doc on disk)"
    );

    // ── Approve: write byte-stable, retire foreign, ONE commit, ingest adopted ──
    let approved = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        approved.status.success(),
        "finalize --approve on a conformant migration lands (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approved.stdout),
        String::from_utf8_lossy(&approved.stderr),
    );

    // Exactly ONE new commit (the all-or-nothing transaction).
    let count_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count_after,
        count_before + 1,
        "the approved migration lands exactly ONE commit"
    );

    // The canonical managed doc is on disk + committed + round-trips BYTE-STABLE.
    let canonical = repo.path().join("CHANGELOG.md");
    let committed = fs::read_to_string(&canonical).expect("the canonical changelog is on disk");
    let schema = shipped_changelog_schema(&pack);
    let parsed = engine::write::instance_from_source(&schema, &committed)
        .expect("the committed changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        committed,
        "the committed changelog is byte-stable across parse -> render:\n{committed}",
    );
    // The historical dates survived migration (both releases carry their foreign date).
    assert!(
        committed.contains("2023-01-15") && committed.contains("2022-08-01"),
        "both historical release dates survive the migration:\n{committed}",
    );
    assert!(
        git(repo.path(), &["cat-file", "-t", "HEAD:CHANGELOG.md"]).contains("blob"),
        "the canonical changelog is committed in HEAD"
    );

    // The foreign original is GONE, and the SAME commit carries its deletion + the add.
    assert!(
        !repo.path().join("HISTORY.md").exists(),
        "the approved migration retires the foreign original from disk"
    );
    let name_status = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.contains("D\tHISTORY.md"),
        "the finalize commit carries the foreign deletion:\n{name_status}"
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md"),
        "the SAME commit carries the added managed doc:\n{name_status}"
    );

    // A follow-up `jigc ingest` reports the managed changelog ADOPTED.
    let ingest = ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["ingest"], None),
        "jigc ingest",
    );
    let row = ingest
        .lines()
        .find(|l| l.contains("CHANGELOG.md"))
        .unwrap_or_else(|| panic!("ingest reports the managed changelog:\n{ingest}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the managed changelog ingests as adopted:\n{row}"
    );
    assert!(
        !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed changelog is neither unmanaged nor needs-reconcile:\n{row}"
    );
}

/// RED 1: a `Performance` change-group (a foreign category outside the `category`
/// enum) is rejected **at the `add-item` write verb** (M24 inc-2 moved the id-from
/// enum to write time) — non-zero exit, finding `schema-conformance.field-value-conformant`
/// naming the slug-cased `…/changes/performance/category` address, with no commit /
/// retire / adopt. The reject fires before the bad group is authored, so the migration
/// never reaches finalize (the finalize-time enum check still backstops an out-of-band
/// foreign category — the engine `validate.rs` unit tests cover that path).
#[test]
fn flow25_red1_performance_group_blocks_at_enum_conformance() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    let rel_120 = drive_conformant_migration(repo.path(), home.path(), &pack);

    // The id-from slug = `performance`; the finding address is rooted at the release address
    // the binary minted (driven verbatim, not hand-built) — in **URI normal form** since
    // M42 inc-9 T2 (`design/command-output-contract.md` → the `write.*` row: a bare fragment
    // is not a stable key), so the minted address prefixes it whole.
    let expected_address = format!("{rel_120}/changes/performance/category");

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // A NON-KaC category outside the enum — rejected at the add-item write verb, before
    // any byte is staged. `--format json` so the emitted finding envelope is parseable.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "--format",
            "json",
            "doc",
            "add-item",
            &format!("{rel_120}/changes"),
            "--title",
            "Performance",
            "--task",
            TASK,
        ],
        None,
    );

    // The write verb blocks (non-zero exit) — the bad category never reaches the staged
    // doc, so finalize is never invoked.
    assert!(
        !out.status.success(),
        "a foreign category blocks at the add-item write verb (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The structured finding the agent consumes: the exact (code, address) pin.
    let report: engine::result::ValidationReport = serde_json::from_slice(&out.stderr)
        .unwrap_or_else(|e| {
            panic!(
                "the block renders a JSON validation report on stderr ({e}); stderr:\n{}",
                String::from_utf8_lossy(&out.stderr)
            )
        });
    let enum_block = report
        .findings
        .iter()
        .find(|f| {
            f.code == "schema-conformance.field-value-conformant"
                && f.location.as_ref().and_then(|l| l.address.as_deref()) == Some(&expected_address)
        })
        .unwrap_or_else(|| {
            panic!(
                "the enum block names the slug-cased `{expected_address}`; findings:\n{:#?}",
                report.findings
            )
        });
    assert_eq!(
        enum_block.severity,
        engine::finding::Severity::Blocking,
        "the enum conformance failure is blocking",
    );

    // No commit / retire / adopt on the blocked branch.
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the enum block leaves HEAD unchanged"
    );
    assert_eq!(
        count_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the enum block creates no commit"
    );
    assert!(
        repo.path().join("HISTORY.md").exists(),
        "the enum block retires nothing (foreign original on disk)"
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "the enum block adopts nothing (no canonical doc on disk)"
    );
}

/// RED 2: `finalize` without `--approve` is byte-safe AND state-safe — the foreign
/// original is byte-intact and the persisted edge index is unchanged (the human-reject
/// path mutates nothing durable).
#[test]
fn flow25_red2_block_without_approve_is_byte_and_index_safe() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    drive_conformant_migration(repo.path(), home.path(), &pack);

    let foreign_before = fs::read(repo.path().join("HISTORY.md")).expect("read foreign before");
    let edges_before = committed_edges(repo.path());

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(4),
        "finalize without --approve blocks at the review gate (exit 4); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Byte-safe: the foreign original is untouched.
    assert_eq!(
        foreign_before,
        fs::read(repo.path().join("HISTORY.md")).expect("read foreign after"),
        "a blocked review leaves the foreign original byte-intact"
    );
    // State-safe: no migration edge leaked into the committed index (the staged doc is
    // not registered — the committed edge set is unchanged).
    let edges_after = committed_edges(repo.path());
    assert_eq!(
        edges_before, edges_after,
        "a blocked review registers no edge into the committed index"
    );
    assert!(
        edges_after
            .iter()
            .all(|e| e["from"] != "changelog:changelog" && e["to"] != "changelog:changelog"),
        "a blocked review registers no edge for the staged changelog; edges:\n{edges_after:#?}"
    );
}
