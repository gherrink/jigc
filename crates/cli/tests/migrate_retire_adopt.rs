//! M23 Increment 3, T2 — the retire transaction (the first byte-destructive write) +
//! adopt-after-write.
//!
//! On a **migration task** ([auto-migration.md](../../../design/auto-migration.md) →
//! Retire-the-foreign-original / Adopt), `jigc task finalize <id> --approve` executes
//! the transaction all-or-nothing: write `CHANGELOG.md` (byte-stable),
//! **retire the foreign original** (here a committed root `HISTORY.md`, so the
//! deletion stages into the same commit), `git` commit, and adopt. The proof:
//!   - the canonical doc lands byte-stable (`render(parse(x)) == x`) + is committed;
//!   - the foreign original is **gone** from disk and the commit carries its deletion;
//!   - exactly **one** new commit carries both the added managed doc and the deletion;
//!   - a follow-up `jigc ingest` reports the managed changelog **adopted** (not
//!     `unmanaged` / `needs-reconcile`).
//!
//! Cold/empty spike: a SINGLE-release foreign file `--approve`s — commits + retires
//! clean. Drives the built `jigc` binary against a throwaway `git init` temp repo over
//! the shipped dev pack.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-retire-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation exits 0, returning its trimmed stdout.
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

/// The shipped changelog schema, loaded for the round-trip assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped changelog schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// Fill every author-required field/slot of the provisioned commit doc for `task`.
fn make_commit_conformant(repo: &Path, home: &Path, pack: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &["doc", "set-field", addr, "--value", value, "--task", task],
                None,
            ),
            "set-field commit",
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
                Some(prose),
            ),
            "set-slot commit",
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "changelog");
    set_slot(
        &format!("commit:{task}#summary"),
        b"adopt the migrated changelog\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Migrate the foreign HISTORY.md into managed shape.\n",
    );
}

/// Write the foreign file, **commit it** (so its retirement lands as a tracked
/// deletion), then drive the migrate + author spine to a conformant staged
/// `changelog:changelog` over `foreign` plus a conformant commit doc.
fn committed_staged_migration(repo: &Path, home: &Path, pack: &Path, foreign: &str) {
    // Track the foreign original FIRST: the retire-then-`git add --all` must surface a
    // real deletion in the finalize commit (the realistic flow-25 scenario — a
    // pre-existing committed `HISTORY.md`).
    fs::write(repo.join("HISTORY.md"), foreign).expect("write foreign HISTORY.md");
    git(repo, &["add", "HISTORY.md"]);
    git(repo, &["commit", "-q", "-m", "track foreign changelog"]);

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate",
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
    let release = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "0.1.0",
                "--task",
                TASK,
            ],
            None,
        ),
        "add-item release",
    );
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("{release}/date"),
                "--value",
                "2021-03-09",
                "--task",
                TASK,
            ],
            None,
        ),
        "set-field date",
    );
    let group = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
                "--task",
                TASK,
            ],
            None,
        ),
        "add-item change-group",
    );
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
                "--task",
                TASK,
            ],
            Some(b"First public release.\n"),
        ),
        "set-slot notes",
    );
    make_commit_conformant(repo, home, pack, TASK);
}

/// The single-release foreign file (the cold/empty spike input).
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

#[test]
fn migration_finalize_approve_writes_retires_and_adopts() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    committed_staged_migration(repo.path(), home.path(), &pack, FOREIGN);

    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert!(
        repo.path().join("HISTORY.md").exists(),
        "the foreign original is on disk before --approve"
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "finalize --approve on a conformant migration must land (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
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

    // The canonical managed doc is committed + on disk, and round-trips BYTE-STABLE
    // (`render(parse(x)) == x`) — the contract the adopt path leans on.
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
    assert!(
        git(repo.path(), &["cat-file", "-t", "HEAD:CHANGELOG.md"]).contains("blob"),
        "the canonical changelog is committed in HEAD"
    );

    // The foreign original is GONE from disk (the first byte-destructive write)...
    assert!(
        !repo.path().join("HISTORY.md").exists(),
        "the approved migration retires the foreign original from disk"
    );
    // ...and the SAME commit carries its deletion (it was a tracked file).
    let name_status = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.contains("D\tHISTORY.md"),
        "the finalize commit carries the foreign deletion:\n{name_status}"
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md"),
        "the SAME commit carries the added managed doc:\n{name_status}"
    );

    // A follow-up `jigc ingest` reports the managed changelog ADOPTED — never
    // `unmanaged` / `needs-reconcile`.
    let ingest = ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["ingest"], None),
        "jigc ingest",
    );
    let row = ingest
        .lines()
        .find(|l| l.contains("CHANGELOG.md"))
        .unwrap_or_else(|| panic!("ingest must report the managed changelog:\n{ingest}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the managed changelog ingests as adopted:\n{row}"
    );
    assert!(
        !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed changelog is neither unmanaged nor needs-reconcile:\n{row}"
    );
}
