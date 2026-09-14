//! M23 Increment 1, T4 — flow-25 staged-only acceptance: the migrate spine, end to
//! end, over a real `git init` temp repo against the **shipped** dev pack.
//!
//! This is the increment's headline proof ([auto-migration.md](../../../design/auto-migration.md)
//! → Acceptance — worked-examples flow 25, the staged-only portion). It drives the
//! whole T1–T3 spine through the real `jigc` binary:
//!   - `jigc migrate <foreign HISTORY.md> --as changelog` mints the off-router task
//!     and composes the shipped `migrate-changelog` workflow with the **foreign content
//!     present** in the emitted view (the source seam — `{{ source }}` — resolves);
//!   - driving the author spine (`doc create` / `add-item` / `set-field` / `set-slot`)
//!     against the minted task produces a staged changelog that
//!     **round-trips byte-stable**: `render(&schema, &instance_from_source(&schema,
//!     staged)) == staged` (the idiom reused from `changelog_cold_create.rs`);
//!   - the release `date` is authored from the foreign file's **historical** date,
//!     OVERWRITING the on-create today-stamp, so historical dates survive migration.
//!
//! Three foreign inputs author + round-trip clean: a multi-release file (the headline),
//! a single-release file, and an empty-`[Unreleased]`-only file (the cold/empty spike).
//!
//! Staged-only: no commit / retire / adopt is asserted here — those are Increment 3.

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
            "jigc-flow25-{tag}-{}-{:?}",
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Stage a foreign migration source. **M51 Increment 1 / T2**: `jigc migrate` refuses a source
/// git holds no copy of — in neither the index nor `HEAD` — and routes at exactly this
/// `git add`, so every fixture that hands the door a freshly written file stages it first.
fn stage(root: &Path, path: &str) {
    let out = Command::new("git")
        .args(["add", "--", path])
        .current_dir(root)
        .output()
        .expect("run git add");
    assert!(
        out.status.success(),
        "git add -- {path} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both
/// streams.
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

/// The staged `changelog:changelog` instance in the migration task's working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The shipped changelog schema, loaded for the re-read + round-trip assertions.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped changelog schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The off-router migration task id — `migrate` mints a per-file `migrate-<doctype>-<slug(path)>` (the empty
/// intent slugs the `migrate-` id-source fallback). The foreign source is an OFF-canonical
/// `HISTORY.md` (post-M38, root `CHANGELOG.md` is the changelog's managed canonical home,
/// so a foreign file there adopts in place — a separate proof; a foreign changelog at any
/// other path migrates + promotes to the canonical root), so the task lands at
/// `migrate-changelog-history` (`auto-migration.md` -> Hardening #9).
const TASK: &str = "migrate-changelog-history-3268e06b69e1";

/// `jigc migrate HISTORY.md --as changelog`: write the foreign file, run the verb,
/// and return the composed view's stdout — asserting it surfaced the foreign content
/// through the source seam (`{{ source }}`, T1's pinned spelling).
fn migrate(repo: &Path, home: &Path, pack: &Path, foreign: &str) -> String {
    fs::write(repo.join("HISTORY.md"), foreign).expect("write foreign HISTORY.md");
    stage(repo, "HISTORY.md");
    let stdout = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "jigc migrate HISTORY.md --as changelog",
    );
    // The seam resolved: the composed view carries the foreign content verbatim (the
    // `{{ source }}` read-only context placeholder, syntactically distinct from a
    // managed-doc `{{@…}}` deref).
    assert!(
        stdout.contains(foreign.trim_end()),
        "the composed migrate workflow must surface the foreign content through the \
         source seam; stdout:\n{stdout}",
    );
    stdout
}

/// `doc create changelog` against the migration task — cold-mints the FIXED-slug
/// singleton (`changelog:changelog`).
fn create_changelog(repo: &Path, home: &Path, pack: &Path) {
    let created = ok_stdout(
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
        "jigc doc create changelog",
    );
    assert_eq!(
        created, "changelog:changelog",
        "a singleton mints at the fixed slug = the type id",
    );
}

/// `add-item` a release, then `set-field` its historical date (OVERWRITING the
/// on-create today-stamp). Returns the emitted release address, driven verbatim.
fn add_release(repo: &Path, home: &Path, pack: &Path, version: &str, date: &str) -> String {
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
                version,
                "--task",
                TASK,
            ],
            None,
        ),
        "add-item release",
    );
    // The historical date OVERWRITES the on-create stamp `add-item` placed (the
    // migration-author path the Grouped scope verified at planning).
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
                date,
                "--task",
                TASK,
            ],
            None,
        ),
        "set-field release date (historical)",
    );
    release
}

/// `add-item` a nested change-group under `parent_addr` + author its notes. Drives the
/// emitted group address verbatim for the `set-slot`.
fn add_group(
    repo: &Path,
    home: &Path,
    pack: &Path,
    parent_addr: &str,
    category: &str,
    notes: &[u8],
) {
    let group = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                parent_addr,
                "--title",
                category,
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
            Some(notes),
        ),
        "set-slot change-group notes",
    );
}

/// Re-read the staged changelog and assert it round-trips byte-stable through the
/// shipped schema: `render(&schema, &instance_from_source(&schema, staged)) == staged`.
fn assert_byte_stable(repo: &Path, pack: &Path) -> engine::write::Instance {
    let staged = staged_changelog(repo, TASK);
    let schema = shipped_changelog_schema(pack);
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        staged,
        "the migrated changelog is byte-stable across parse -> render:\n{staged}",
    );
    parsed
}

/// The headline: a real MULTI-RELEASE foreign Keep-a-Changelog file migrates + the
/// staged doc round-trips byte-stable, with HISTORICAL dates overwriting the on-create
/// stamp.
#[test]
fn flow25_multi_release_foreign_changelog_migrates_byte_stable() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let foreign = "\
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
    migrate(repo.path(), home.path(), &pack, foreign);

    // Drive the author spine: create the singleton, then re-author each release
    // oldest-to-newest with its historical date + nested change-groups.
    create_changelog(repo.path(), home.path(), &pack);

    let rel_110 = add_release(repo.path(), home.path(), &pack, "1.1.0", "2022-08-01");
    add_group(
        repo.path(),
        home.path(),
        &pack,
        &format!("{rel_110}/changes"),
        "Changed",
        b"Bumped the default timeout to 30s.\n",
    );

    let rel_120 = add_release(repo.path(), home.path(), &pack, "1.2.0", "2023-01-15");
    add_group(
        repo.path(),
        home.path(),
        &pack,
        &format!("{rel_120}/changes"),
        "Added",
        b"Device-code OAuth flow.\n",
    );
    add_group(
        repo.path(),
        home.path(),
        &pack,
        &format!("{rel_120}/changes"),
        "Fixed",
        b"Session fixation on logout.\n",
    );

    let parsed = assert_byte_stable(repo.path(), &pack);

    // The HISTORICAL date survived migration — the release carries 2023-01-15, NOT the
    // today-stamp `add-item` placed on create.
    let releases = parsed
        .sections
        .iter()
        .find(|s| s.id == "releases")
        .expect("releases section present");
    let rel_120_id = rel_120
        .strip_prefix("changelog:changelog#releases/")
        .expect("release address under #releases/");
    let rel = releases
        .items
        .iter()
        .find(|i| i.id == rel_120_id)
        .unwrap_or_else(|| panic!("release {rel_120_id} present"));
    assert!(
        rel.fields.iter().any(|f| f.key == "date"
            && matches!(&f.value, engine::field_block::Value::Scalar(v) if v == "2023-01-15")),
        "the historical date overwrote the on-create stamp; parsed release: {rel:?}",
    );
}

/// The cold/empty spike, arm 1: a SINGLE-release foreign file authors + round-trips
/// clean.
#[test]
fn flow25_single_release_foreign_changelog_migrates_byte_stable() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let foreign = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";
    migrate(repo.path(), home.path(), &pack, foreign);

    create_changelog(repo.path(), home.path(), &pack);
    let rel = add_release(repo.path(), home.path(), &pack, "0.1.0", "2021-03-09");
    add_group(
        repo.path(),
        home.path(),
        &pack,
        &format!("{rel}/changes"),
        "Added",
        b"First public release.\n",
    );

    assert_byte_stable(repo.path(), &pack);
}

/// The cold/empty spike, arm 2: an empty-`[Unreleased]`-only foreign file (no cut
/// release) authors a staged change-group + round-trips clean.
#[test]
fn flow25_empty_unreleased_foreign_changelog_migrates_byte_stable() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let foreign = "\
# Changelog

## [Unreleased]
### Added
- Work in progress on the new importer.
";
    migrate(repo.path(), home.path(), &pack, foreign);

    create_changelog(repo.path(), home.path(), &pack);
    // No cut release — author the change-group under the STAGED `#unreleased-changes`
    // section instead.
    add_group(
        repo.path(),
        home.path(),
        &pack,
        "changelog:changelog#unreleased-changes",
        "Added",
        b"Work in progress on the new importer.\n",
    );

    assert_byte_stable(repo.path(), &pack);
}
