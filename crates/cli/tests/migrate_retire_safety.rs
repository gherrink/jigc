//! M23 milestone-completion fixes — the byte-destructive retire path's safety
//! preconditions ([auto-migration.md](../../../design/auto-migration.md) →
//! Retire-the-foreign-original + Path-collision guard):
//!
//!   - **F1** — the retire fires ONLY when the migration produced its canonical
//!     replacement. A migration that filled its `commit:<id>` doc but never authored the
//!     target doctype has an empty promote set; finalizing `--approve` must NOT delete the
//!     foreign original (it blocks, never wipes-with-no-replacement).
//!   - **F2** — the in-location-squatter path-collision guard holds regardless of the
//!     spelling the caller passed (`./`-prefixed, absolute). Migrating an in-location
//!     squatter at the canonical managed path must NOT delete the just-promoted canonical
//!     doc.
//!
//! Drives the built `jigc` binary against throwaway `git init` repos over the dev pack.

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
            "jigc-retire-safety-{tag}-{}-{:?}",
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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jigc")
        .wait_with_output()
        .expect("wait for jigc")
}

/// Run a `jigc` subcommand piping `stdin`.
fn run_jigc_stdin(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: &[u8],
) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation exits 0.
fn ok(out: std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The off-router migration task id — `migrate` mints from the doctype-name fallback.
const TASK: &str = "changelog";

/// A single-release foreign changelog body (the migration input).
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// Author a conformant `commit:<TASK>` doc (the transient sink) in the migration task.
fn author_commit_doc(repo: &Path, home: &Path, pack: &Path) {
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("commit:{TASK}#type"),
                "--value",
                "feat",
                "--task",
                TASK,
            ],
        ),
        "set-field commit type",
    );
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("commit:{TASK}#scope"),
                "--value",
                "changelog",
                "--task",
                TASK,
            ],
        ),
        "set-field commit scope",
    );
    ok(
        run_jigc_stdin(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("commit:{TASK}#summary"),
                "--from-file",
                "-",
                "--task",
                TASK,
            ],
            b"adopt the migrated changelog\n",
        ),
        "set-slot commit summary",
    );
    ok(
        run_jigc_stdin(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("commit:{TASK}#body"),
                "--from-file",
                "-",
                "--task",
                TASK,
            ],
            b"Migrate the foreign changelog into managed shape.\n",
        ),
        "set-slot commit body",
    );
}

/// Review F1: a migration that fills only its transient `commit:<id>` doc — never
/// authoring the target doctype — has no canonical replacement to promote. Finalizing
/// `--approve` must NOT delete the foreign original; it blocks (the no-replacement
/// precondition), exiting non-zero with the foreign file byte-intact on disk.
#[test]
fn approved_migration_without_a_replacement_does_not_delete_the_foreign() {
    let repo = TempDir::new("no-replacement");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed foreign CHANGELOG.md at the repo root.
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign changelog"],
    );

    ok(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "setup",
    );
    ok(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["migrate", "CHANGELOG.md", "--as", "changelog"],
        ),
        "migrate",
    );
    // Fill the commit doc but DELIBERATELY skip `doc create changelog` — no managed doc.
    author_commit_doc(repo.path(), home.path(), &pack);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
    );

    assert!(
        !out.status.success(),
        "an --approve finalize with no managed replacement must block (non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        combined.contains("replace"),
        "the block names the missing replacement (F1 finding); got:\n{combined}",
    );

    // The whole point: the foreign original survives byte-intact (never wiped).
    let foreign = repo.path().join("CHANGELOG.md");
    assert!(
        foreign.exists(),
        "the foreign original must NOT be deleted when nothing replaces it",
    );
    assert_eq!(
        fs::read_to_string(&foreign).expect("read foreign"),
        FOREIGN,
        "the foreign original must be byte-intact",
    );
}

/// Read the `source-path` `jigc migrate` recorded for the minted migration task — the
/// retire target the finalize collision guard reads back.
fn recorded_source_path(repo: &Path) -> String {
    fs::read_to_string(
        repo.join(".jigc")
            .join("tasks")
            .join(TASK)
            .join("source-path"),
    )
    .expect("read recorded source-path")
}

/// Review F2: `jigc migrate` records a clean, canonical repo-relative `source-path`
/// regardless of the spelling the caller passed — so the finalize retire's in-location
/// squatter guard (which compares the recorded path against the canonical promote
/// destination) holds. A `./`-prefixed and an absolute spelling of the in-location
/// squatter at `changelog/changelog.md` must both be recorded as the canonical
/// `changelog/changelog.md`; pre-fix the verbatim spelling was recorded and slipped the
/// guard, so the retire wiped the just-promoted canonical doc. Asserted at the
/// recorded-state level (the mint-side normalization the guard consumes); the guard's
/// skip-on-match itself is covered by the engine planner unit tests.
#[test]
fn migrate_records_a_canonical_source_path_for_redundant_spellings() {
    let pack = dev_pack();

    // `./`-prefixed spelling of the canonical managed path → canonical.
    {
        let repo = TempDir::new("squatter-dotslash");
        let home = TempDir::new("home");
        init_repo(repo.path());
        fs::create_dir_all(repo.path().join("changelog")).expect("mk changelog dir");
        fs::write(repo.path().join("changelog").join("changelog.md"), FOREIGN)
            .expect("write squatter");
        ok(
            run_jigc(repo.path(), home.path(), &pack, &["setup"]),
            "setup",
        );
        ok(
            run_jigc(
                repo.path(),
                home.path(),
                &pack,
                &["migrate", "./changelog/changelog.md", "--as", "changelog"],
            ),
            "migrate ./changelog/changelog.md",
        );
        assert_eq!(
            recorded_source_path(repo.path()),
            "changelog/changelog.md",
            "a `./`-prefixed spelling must be normalized to the canonical repo-relative path",
        );
    }

    // Absolute spelling of the canonical managed path → canonical. The binary resolves its
    // cwd via `getcwd` (symlink-resolved); canonicalize the repo path so the absolute
    // spelling shares that prefix and the repo-root strip succeeds.
    {
        let repo = TempDir::new("squatter-absolute");
        let home = TempDir::new("home");
        init_repo(repo.path());
        fs::create_dir_all(repo.path().join("changelog")).expect("mk changelog dir");
        fs::write(repo.path().join("changelog").join("changelog.md"), FOREIGN)
            .expect("write squatter");
        ok(
            run_jigc(repo.path(), home.path(), &pack, &["setup"]),
            "setup",
        );
        let abs = fs::canonicalize(repo.path())
            .expect("canonicalize repo")
            .join("changelog")
            .join("changelog.md");
        ok(
            run_jigc(
                repo.path(),
                home.path(),
                &pack,
                &["migrate", abs.to_str().unwrap(), "--as", "changelog"],
            ),
            "migrate <absolute path>",
        );
        assert_eq!(
            recorded_source_path(repo.path()),
            "changelog/changelog.md",
            "an absolute spelling must be normalized to the canonical repo-relative path",
        );
    }
}
