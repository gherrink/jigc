//! M22 Increment 5, T1 — the shipped `changelog` doctype + `record-change` driver,
//! cold-create round-trip through the real `jigc` binary.
//!
//! This is the T1 done-criterion's real-binary arm: over a `git init` temp repo
//! against the **shipped** dev pack (selected via `JIGC_PACK_DIR` = the embedded
//! `pack/` tree, so it is the bytes that ship, not a fixture), prove that
//!   - `jigc start --workflow record-change` composes (author-change step +
//!     finalize, no compose error) — the off-router driver mints a task;
//!   - `jigc doc create changelog` cold-mints the FIXED-slug singleton
//!     (`changelog:changelog`, slug == type id), never a `--title` slug;
//!   - `add-item` a release + nested change-groups, `set-field` the optional `link`,
//!     and `set-slot` the nested notes author it; the item-level `date` stamps
//!     on-create;
//!   - the staged doc round-trips byte-stable: `render(parse(staged)) == staged`,
//!     with the multi-word `## Unreleased Changes` section intact.
//!
//! Warm-append + the conformance/optional reds are T3's job; this file proves
//! cold-create round-trips and that the doctype + driver ship and load.

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
            "jigc-changelog-cold-{tag}-{}-{:?}",
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
/// composes the exact bytes it ships (a FilesystemPack over the same `pack/` the
/// `include_dir!` embeds).
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

/// The staged `changelog:changelog` instance in the task working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The shipped changelog schema, loaded for the re-read + round-trip assertions
/// (`load_pack_schema` resolves it the way the production loader does).
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    engine::schema::load_schema(&yaml).expect("shipped changelog schema loads")
}

#[test]
fn record_change_cold_creates_the_changelog_singleton_byte_stable() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"], None);
    ok_stdout(setup, "jigc setup");

    // `record-change` is off-router (`selectable: false`), so it is started by name.
    // The compose (author-change + finalize) must succeed — a compose error here
    // would mean the workflow/step/command-ref do not resolve against the shipped
    // catalog.
    let start = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "record-change", "cut the release"],
        None,
    );
    ok_stdout(start, "jigc start --workflow record-change");
    let task = "cut-the-release";

    // Cold-mint the singleton through the create-gate. A singleton mints at the
    // FIXED slug (= the type id `changelog`), NOT a `--title` slug.
    let created = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "create", "changelog", "--title", "Changelog"],
        None,
    );
    let created_addr = ok_stdout(created, "jigc doc create changelog");
    assert_eq!(
        created_addr, "changelog:changelog",
        "a singleton mints at the fixed slug = the type id",
    );

    // add-item a release (its `date` stamps on-create); the minted id is the slugged
    // version title.
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
                "1.2.0",
            ],
            None,
        ),
        "add-item release 1.2.0",
    );
    // The minted release id is the slugged version title (dots dropped by the
    // slugger). Drive the EMITTED address verbatim downstream rather than a
    // reconstructed form.
    let release_id = release
        .strip_prefix("changelog:changelog#releases/")
        .expect("release address is under #releases/")
        .to_owned();
    assert!(
        !release_id.is_empty(),
        "the release minted a non-empty id; got {release}",
    );

    // set the OPTIONAL `link` field on the release.
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-field",
                &format!("{release}/link"),
                "--value",
                "https://example.com/compare/1.1.0...1.2.0",
            ],
            None,
        ),
        "set-field release link",
    );

    // add-item a NESTED change-group `#added` under the release, then author its
    // notes — the `Leaf::Repeatable` authoring path.
    let added = ok_stdout(
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
            ],
            None,
        ),
        "add-item nested #added",
    );
    assert_eq!(added, format!("{release}/added"));
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-slot",
                &format!("{added}/notes"),
                "--from-file",
                "-",
            ],
            Some(b"OAuth device-code flow.\n"),
        ),
        "set-slot nested #added notes",
    );

    // Also author a STAGED (unreleased) change-group, exercising the multi-word
    // `## Unreleased Changes` section's authoring + round-trip.
    let staged_group = ok_stdout(
        run_jigc(
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
            None,
        ),
        "add-item unreleased #fixed",
    );
    assert_eq!(staged_group, "changelog:changelog#unreleased-changes/fixed");
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-slot",
                &format!("{staged_group}/notes"),
                "--from-file",
                "-",
            ],
            Some(b"Session fixation on logout.\n"),
        ),
        "set-slot unreleased #fixed notes",
    );

    // Re-read the staged doc and assert the authored content + byte-stable round-trip.
    let staged = staged_changelog(repo.path(), task);
    let schema = shipped_changelog_schema(&pack);
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged changelog re-parses");

    // The multi-word section heading is rendered and survives the round-trip.
    assert!(
        staged.contains("## Unreleased Changes"),
        "the multi-word section heading is intact; staged:\n{staged}",
    );

    // The release carries its on-create date and the optional link.
    let releases = parsed
        .sections
        .iter()
        .find(|s| s.id == "releases")
        .expect("releases section present");
    let rel = releases
        .items
        .iter()
        .find(|i| i.id == release_id)
        .unwrap_or_else(|| panic!("release {release_id} present"));
    assert!(
        rel.fields.iter().any(|f| f.key == "date"
            && matches!(&f.value, engine::field_block::Value::Scalar(v) if !v.is_empty())),
        "the release's on-create date is stamped; staged:\n{staged}",
    );
    assert!(
        rel.fields.iter().any(|f| f.key == "link"
            && matches!(&f.value, engine::field_block::Value::Scalar(v)
                if v == "https://example.com/compare/1.1.0...1.2.0")),
        "the optional link field is authored; staged:\n{staged}",
    );
    let nested = rel
        .items
        .iter()
        .find(|i| i.id == "added")
        .expect("nested #added present");
    assert_eq!(
        nested.slot.as_deref().unwrap_or("").trim(),
        "OAuth device-code flow.",
        "the nested notes are authored; staged:\n{staged}",
    );

    // Byte-stable: render(parse(staged)) == staged.
    assert_eq!(
        engine::write::render(&schema, &parsed),
        staged,
        "the cold-authored changelog is byte-stable across parse → render",
    );
}
