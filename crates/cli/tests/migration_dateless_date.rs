//! M24 Increment 4, T2 — migration-mode `set: on-create` date suppression
//! ([auto-migration.md](../../../design/auto-migration.md) → Hardening #6; the
//! `jigc migrate` verb date posture; [changelog.md](../../../design/changelog.md) →
//! Releases date posture).
//!
//! A dateless foreign changelog must not have the migration day fabricated as false
//! history: in **migration mode** (a task minted by `jigc migrate`, recognized by the
//! recorded `source-path`) the `set: on-create` today-stamp is **suppressed**, so a
//! release `add-item`'d with no `set-field date` renders with **no `date:` line** and
//! finalizes **clean** (a `set:` field is not author-required). The two omitting/inverse
//! contexts that must stay unaffected are asserted alongside:
//!   - **authoring** (`record-change` via `jigc start`, NOT a migration) still stamps
//!     today on `add-item` — the suppression is scoped to migration mode only;
//!   - an **explicit** `set-field <release>/date <ISO>` in migration mode still writes.
//!
//! Every assertion runs over the EMITTED bytes / exit code of the real `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) against the shipped dev pack (`JIGC_PACK_DIR`).

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
            "jigc-migration-dateless-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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

/// The staged `changelog:changelog` body in `task`'s working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The migration task id — `jigc migrate` mints a per-file `migrate-<doctype>-<slug(path)>`.
const MIGRATE_TASK: &str = "migrate-changelog-changelog";

/// A dateless foreign Keep-a-Changelog file — the common case (no `- YYYY-MM-DD`).
const DATELESS_FOREIGN: &str = "\
# Changelog

## [1.0.0]
### Added
- First public release.
";

/// Run `jigc migrate CHANGELOG.md --as changelog` over a dateless foreign file, minting
/// the off-router `migrate-changelog-changelog` task (recording the `source-path` = migration mode).
fn start_migration(repo: &Path, home: &Path, pack: &Path) {
    fs::write(repo.join("CHANGELOG.md"), DATELESS_FOREIGN).expect("write foreign CHANGELOG.md");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", "CHANGELOG.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate CHANGELOG.md --as changelog",
    );
}

/// `doc create changelog` against `task`.
fn create_changelog(repo: &Path, home: &Path, pack: &Path, task: &str) {
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
                task,
            ],
            None,
        ),
        "doc create changelog",
    );
}

/// `add-item` a release named `version` against `task`; returns the emitted release
/// address (driven verbatim — never re-spelled).
fn add_release(repo: &Path, home: &Path, pack: &Path, task: &str, version: &str) -> String {
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                version,
                "--task",
                task,
            ],
            None,
        ),
        "add-item release",
    )
}

/// The `date:` field value spliced onto the staged release `version`, or `None` if the
/// release carries no `date:` line at all. Reads the EMITTED staged bytes, scoping to
/// the release block (the version heading down to the next blank-then-heading).
fn release_date(repo: &Path, task: &str, version: &str) -> Option<String> {
    let staged = staged_changelog(repo, task);
    // The release item heading carries the version title; the `date:` field is the first
    // `date:`-keyed line after it (the only date leaf in a release block).
    let after_heading = staged
        .split_once(version)
        .unwrap_or_else(|| panic!("the staged release `{version}` is present:\n{staged}"))
        .1;
    after_heading
        .lines()
        // Stop at the next release heading so a later release's date is not mis-read.
        .take_while(|l| !l.starts_with("## ") || l.contains(version))
        .find_map(|l| l.trim_start().strip_prefix("- date:"))
        .or_else(|| {
            after_heading
                .lines()
                .take_while(|l| !l.starts_with("## ") || l.contains(version))
                .find_map(|l| l.trim_start().strip_prefix("date:"))
        })
        .map(|v| v.trim().to_string())
}

/// A dateless release `add-item`'d in MIGRATION MODE renders NO `date:` line and the
/// migration finalizes CLEAN (an absent `set: on-create` date is not author-required).
#[test]
fn migration_dateless_release_omits_date_and_finalizes_clean() {
    let repo = TempDir::new("omit");
    let home = TempDir::new("omit-home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    start_migration(repo.path(), home.path(), &pack);
    create_changelog(repo.path(), home.path(), &pack, MIGRATE_TASK);

    // The dateless release — NO `set-field date` follows the mint.
    let release = add_release(repo.path(), home.path(), &pack, MIGRATE_TASK, "1.0.0");

    // RED: today's date is NOT stamped in migration mode — the release carries no date.
    assert_eq!(
        release_date(repo.path(), MIGRATE_TASK, "1.0.0"),
        None,
        "a dateless migration release must render NO `date:` line (the today-stamp is \
         suppressed); staged:\n{}",
        staged_changelog(repo.path(), MIGRATE_TASK),
    );

    // Author the rest of the release so it is well-formed, then prove it finalizes clean.
    let group = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
                "--task",
                MIGRATE_TASK,
            ],
            None,
        ),
        "add-item change-group",
    );
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
                "--task",
                MIGRATE_TASK,
            ],
            Some(b"First public release.\n"),
        ),
        "set-slot notes",
    );
    make_commit_conformant(repo.path(), home.path(), &pack, MIGRATE_TASK);

    // `jigc task finalize --approve` lands clean — the absent `set: on-create` date
    // raises NO required-field block (the engine treats a `set:` field as not
    // author-required), so the dateless release commits.
    let finalize = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", MIGRATE_TASK, "--approve"],
        None,
    );
    assert!(
        finalize.status.success(),
        "a dateless migration release must finalize clean (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr),
    );

    // The COMMITTED canonical changelog carries no `date:` line for the release.
    let committed = fs::read_to_string(repo.path().join("changelog").join("changelog.md"))
        .expect("the canonical changelog is on disk after finalize");
    assert!(
        !committed.contains("- date:") && !committed.contains("\ndate:"),
        "the committed dateless migration release carries no `date:` line:\n{committed}",
    );
}

/// Fill every author-required field/slot of the provisioned commit doc for `task` (the
/// migration finalize gates on a conformant commit doc — mirrors `migrate_retire_adopt`).
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

/// AUTHORING (`record-change` via `jigc start`, NOT a migration) is unaffected: the
/// release `add-item` still stamps today's date. This is the omitting context — the
/// suppression must be scoped to migration mode and inert everywhere else.
#[test]
fn authoring_release_still_stamps_today() {
    let repo = TempDir::new("authoring");
    let home = TempDir::new("authoring-home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // `record-change` is off-router and admits `changelog` — NOT a migration (no
    // `source-path` recorded), so the on-create stamp must still fire.
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "record-change", "cut the release"],
            None,
        ),
        "jigc start --workflow record-change",
    );
    let task = "cut-the-release";
    create_changelog(repo.path(), home.path(), &pack, task);
    add_release(repo.path(), home.path(), &pack, task, "2.0.0");

    let date = release_date(repo.path(), task, "2.0.0").unwrap_or_else(|| {
        panic!(
            "an authoring release must carry a stamped `date:` field; staged:\n{}",
            staged_changelog(repo.path(), task),
        )
    });
    let ymd: Vec<&str> = date.split('-').collect();
    assert_eq!(
        ymd.len(),
        3,
        "the on-create authoring date is an ISO `YYYY-MM-DD`; got {date:?}",
    );
    assert!(
        ymd[0].len() == 4 && ymd[0].chars().all(|c| c.is_ascii_digit()),
        "the on-create authoring date's year is four digits; got {date:?}",
    );
}

/// In MIGRATION MODE, an EXPLICIT `set-field <release>/date <ISO>` still writes the date
/// (the suppression only drops the implicit on-create stamp, never an explicit write).
#[test]
fn migration_explicit_date_still_writes() {
    let repo = TempDir::new("explicit");
    let home = TempDir::new("explicit-home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    start_migration(repo.path(), home.path(), &pack);
    create_changelog(repo.path(), home.path(), &pack, MIGRATE_TASK);

    let release = add_release(repo.path(), home.path(), &pack, MIGRATE_TASK, "1.0.0");
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-field",
                &format!("{release}/date"),
                "--value",
                "2020-05-05",
                "--task",
                MIGRATE_TASK,
            ],
            None,
        ),
        "set-field release date (explicit, historical)",
    );

    assert_eq!(
        release_date(repo.path(), MIGRATE_TASK, "1.0.0").as_deref(),
        Some("2020-05-05"),
        "an explicit set-field date writes even in migration mode; staged:\n{}",
        staged_changelog(repo.path(), MIGRATE_TASK),
    );
}
