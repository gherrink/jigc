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

/// Assert a `jigc` invocation exits 0, returning its trimmed stdout (the minted address
/// an `add-item` echoes for the next splice).
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
const TASK: &str = "migrate-changelog-changelog";

/// The per-file migration task id for a source AT the canonical managed path
/// (`changelog/changelog.md`) — the path is folded into the slug, so this in-location
/// squatter mints a distinct id from the root-`CHANGELOG.md` `TASK` above.
const SQUATTER_TASK: &str = "migrate-changelog-changelog-changelog";

/// A single-release foreign changelog body (the migration input) — the raw Keep-a-Changelog
/// shape, NON-conformant to the managed `changelog` schema (no `{#…}` anchors, no
/// `<!-- fields -->` block), so an in-location squatter at the canonical path is a genuine
/// non-conformant body the copy-in would otherwise build a Frankenstein onto.
const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// The seeded BLANK changelog template — the empty managed skeleton `doc create` mints
/// when it skips the copy-in. The contrast against `FOREIGN` is the whole point: the
/// in-location squatter seeds THIS, never the foreign body.
const EMPTY: &str = "\
# changelog

## Unreleased Changes


## Releases
";

/// Author a conformant `commit:<task>` doc (the transient sink) in the migration `task`.
fn author_commit_doc(repo: &Path, home: &Path, pack: &Path, task: &str) {
    ok(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "feat",
                "--task",
                task,
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
                &format!("commit:{task}#scope"),
                "--value",
                "changelog",
                "--task",
                task,
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
                &format!("commit:{task}#summary"),
                "--from-file",
                "-",
                "--task",
                task,
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
                &format!("commit:{task}#body"),
                "--from-file",
                "-",
                "--task",
                task,
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
    author_commit_doc(repo.path(), home.path(), &pack, TASK);

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
fn recorded_source_path(repo: &Path, task: &str) -> String {
    fs::read_to_string(
        repo.join(".jigc")
            .join("tasks")
            .join(task)
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
            recorded_source_path(repo.path(), SQUATTER_TASK),
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
            recorded_source_path(repo.path(), SQUATTER_TASK),
            "changelog/changelog.md",
            "an absolute spelling must be normalized to the canonical repo-relative path",
        );
    }
}

/// The shipped changelog schema, loaded for the round-trip byte-stability assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    engine::schema::load_schema(&yaml).expect("shipped changelog schema loads")
}

/// M24 inc-5 — the in-location squatter, end-to-end (the M23 e2e FAIL now passes;
/// [auto-migration.md](../../../design/auto-migration.md) → Path-collision guard /
/// Hardening #8, worked-examples flow 26 #8). A NON-conformant changelog committed
/// already AT the canonical managed path (`changelog/changelog.md`) migrates end-to-end:
///   - `doc create` over the occupied canonical path seeds the working area **BLANK** —
///     the empty template, NOT the foreign squatter body. This targets the precise M23
///     failure point: the idempotent-create copy-in (the M16 clobber-fix) read the
///     foreign bytes in as the edit base, so the author sequence built onto
///     non-conformant bytes and finalize blocked on a Frankenstein doc;
///   - the author sequence builds a conformant release onto the clean skeleton;
///   - `finalize --approve` rewrites the canonical doc **in place** (the retire is
///     SKIPPED — the `source-path == promote-destination` guard), byte-stable, and the
///     committed doc is the authored doc, not a foreign/authored merge.
#[test]
fn in_location_squatter_seeds_blank_authors_and_finalizes_byte_stable() {
    let repo = TempDir::new("squatter-e2e");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed NON-conformant changelog squatter AT the canonical managed path.
    fs::create_dir_all(repo.path().join("changelog")).expect("mk changelog dir");
    fs::write(repo.path().join("changelog").join("changelog.md"), FOREIGN)
        .expect("write in-location squatter");
    git(repo.path(), &["add", "changelog/changelog.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track in-location squatter"],
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
            &["migrate", "changelog/changelog.md", "--as", "changelog"],
        ),
        "migrate changelog/changelog.md",
    );

    // `doc create` over the occupied canonical path seeds BLANK — the empty template, NOT
    // the foreign squatter bytes (the precise M23 failure point).
    ok(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                SQUATTER_TASK,
            ],
        ),
        "doc create changelog",
    );
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(SQUATTER_TASK)
        .join("docs")
        .join("changelog:changelog.md");
    let seeded = fs::read_to_string(&staged).expect("read staged changelog");
    assert_eq!(
        seeded, EMPTY,
        "the working area must seed the BLANK empty template over the occupied canonical path",
    );
    assert_ne!(
        seeded, FOREIGN,
        "the foreign squatter bytes must NOT be copied in as the edit base (the M23 failure point)",
    );

    // Author a conformant single release over the clean skeleton.
    let release = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "0.1.0",
                "--task",
                SQUATTER_TASK,
            ],
        ),
        "add-item release",
    );
    ok(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-field",
                &format!("{release}/date"),
                "--value",
                "2021-03-09",
                "--task",
                SQUATTER_TASK,
            ],
        ),
        "set-field date",
    );
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
                SQUATTER_TASK,
            ],
        ),
        "add-item change-group",
    );
    ok(
        run_jigc_stdin(
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
                SQUATTER_TASK,
            ],
            b"- First public release.\n",
        ),
        "set-slot notes",
    );
    author_commit_doc(repo.path(), home.path(), &pack, SQUATTER_TASK);

    // The authored staged buffer — the canonical doc must equal exactly THIS after
    // finalize (no foreign/authored merge).
    let authored = fs::read_to_string(&staged).expect("read authored staged changelog");
    assert_ne!(
        authored, EMPTY,
        "the author sequence must have written a release over the skeleton",
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", SQUATTER_TASK, "--approve"],
    );
    assert!(
        out.status.success(),
        "finalize --approve on the in-location squatter must land clean (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The canonical doc holds the AUTHORED doc (no Frankenstein) and round-trips
    // byte-stable.
    let canonical = repo.path().join("changelog").join("changelog.md");
    let committed = fs::read_to_string(&canonical).expect("the canonical changelog is on disk");
    assert_eq!(
        committed, authored,
        "the committed canonical doc is exactly the authored doc — not a foreign/authored merge",
    );
    let schema = shipped_changelog_schema(&pack);
    let parsed = engine::write::instance_from_source(&schema, &committed)
        .expect("the committed changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        committed,
        "the committed changelog is byte-stable across parse -> render:\n{committed}",
    );

    // The retire is SKIPPED — the file is rewritten in place (Modified), never deleted.
    let name_status = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        name_status.contains("M\tchangelog/changelog.md"),
        "the squatter is rewritten in place (Modified), not retired:\n{name_status}",
    );
    assert!(
        !name_status.contains("D\tchangelog/changelog.md"),
        "the in-location squatter retire must be SKIPPED — no deletion of the just-written doc:\n{name_status}",
    );
}
