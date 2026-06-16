//! M23 Increment 3, T1 — the `--approve` review gate (block-without-approve).
//!
//! On a **migration task** ([auto-migration.md](../../../design/auto-migration.md) →
//! The review gate), `jigc task finalize <id>` *without* `--approve` renders the
//! **fidelity diff** (the staged source-seam bytes vs the staged canonical doc) and
//! **exits non-zero, committing nothing** — the foreign original is byte-intact and
//! nothing is adopted. `--approve` is the human's only check on rewrite fidelity (the
//! strict parse guarantees *structure*, never *content-faithfulness*).
//!
//! The omitting-context arm: a **non-migration** finalize ignores `--approve` (the gate
//! is inert — the existing transactional path runs unchanged).
//!
//! Drives the built `jigc` binary against a throwaway `git init` temp repo over the
//! shipped dev pack. Retire + commit + adopt are later tasks of this increment — not
//! asserted here.

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
            "jigc-reviewgate-{tag}-{}-{:?}",
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

/// The off-router migration task id — `migrate` mints `migrate-<doctype>` (the empty
/// intent slugs the `migrate-` id-source fallback), keeping the bare `changelog`
/// namespace free (`auto-migration.md` -> Hardening #9).
const TASK: &str = "migrate-changelog";

/// Fill every author-required field/slot of the provisioned commit doc for `task` so a
/// `finalize` over it validates clean.
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
        b"Migrate the foreign CHANGELOG.md into managed shape.\n",
    );
}

/// Drive the migrate + author spine to a conformant staged `changelog:changelog` over a
/// single-release foreign file, plus a conformant commit doc — the state finalize gates.
fn staged_migration(repo: &Path, home: &Path, pack: &Path, foreign: &str) {
    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    fs::write(repo.join("CHANGELOG.md"), foreign).expect("write foreign CHANGELOG.md");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", "CHANGELOG.md", "--as", "changelog"],
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

const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

#[test]
fn migration_finalize_without_approve_blocks_and_commits_nothing() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    staged_migration(repo.path(), home.path(), &pack, FOREIGN);

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let log_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let foreign_before = fs::read(repo.path().join("CHANGELOG.md")).expect("read foreign before");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );

    // The review gate exits with a DISTINCT review-pending code (not 0, not the
    // validation-blocked 3, not the operational 1) — the build-pin EXIT_REVIEW_PENDING.
    assert_eq!(
        out.status.code(),
        Some(4),
        "finalize without --approve on a migration task must exit 4 (review-pending); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The fidelity diff shows BOTH inputs: the foreign source (its KaC bracket heading,
    // present only in the foreign file) and the canonical rewrite (labeled by its
    // destination path).
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("## [0.1.0] - 2021-03-09"),
        "the fidelity diff must surface the foreign source bytes; got:\n{rendered}"
    );
    assert!(
        rendered.contains("changelog/changelog.md"),
        "the fidelity diff must surface the canonical rewrite (its destination); got:\n{rendered}"
    );
    assert!(
        rendered.contains("--approve"),
        "the block must tell the human how to approve; got:\n{rendered}"
    );

    // Nothing committed: HEAD unchanged, no new commit, no `changelog/changelog.md` in HEAD.
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "a blocked review must leave HEAD unchanged"
    );
    assert_eq!(
        log_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        "a blocked review must create no commit"
    );
    let show = Command::new("git")
        .args(["show", "HEAD:changelog/changelog.md"])
        .current_dir(repo.path())
        .output()
        .expect("run git show");
    assert!(
        !show.status.success(),
        "a blocked review must commit no canonical changelog"
    );

    // The foreign original is byte-intact (rejection is byte-safe).
    assert_eq!(
        foreign_before,
        fs::read(repo.path().join("CHANGELOG.md")).expect("read foreign after"),
        "a blocked review must leave the foreign original untouched"
    );

    // Nothing adopted: the adopt path's file-state hash for the canonical committed
    // path is never recorded (the swept record is dropped on the gated branch).
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    if let Ok(json) = fs::read_to_string(&record) {
        assert!(
            !json.contains("changelog/changelog.md"),
            "a blocked review must not adopt (no file-state baseline for the canonical doc); \
             got:\n{json}"
        );
    }

    // The working area survives for the approve re-run.
    assert!(
        repo.path().join(".jigc").join("tasks").join(TASK).exists(),
        "a blocked review must keep the working area for the --approve re-run"
    );
}

#[test]
fn non_migration_finalize_ignores_approve() {
    // The omitting-context arm: a plain single-task finalize WITH --approve runs the
    // existing transactional path unchanged (the gate is inert with no source seam).
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "setup",
    );

    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "single-task", "add rate limiter"],
            None,
        ),
        "jigc start",
    );
    let task = "add-rate-limiter";
    fs::write(repo.path().join("limiter.rs"), "// limiter\n").expect("write code change");
    make_commit_conformant(repo.path(), home.path(), &pack, task);

    let log_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", task, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "a non-migration finalize must ignore --approve and land (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let log_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "a non-migration finalize --approve must land exactly ONE commit (the existing path)"
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !rendered.contains("review required") && !rendered.contains("fidelity"),
        "the review gate must be inert on a non-migration task; got:\n{rendered}"
    );
}
