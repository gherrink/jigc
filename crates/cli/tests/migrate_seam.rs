//! M23 Increment 1, T2 — the `jigc migrate` verb: dispatch + off-router mint +
//! foreign-bytes staging + source-seam feed, over a real `git init` temp repo
//! against the shipped dev pack.
//!
//! Done-criterion (T2): `jigc migrate <foreign CHANGELOG.md> --as changelog`
//!   - mints the **off-router** migration task — its working area + base pin exist,
//!     and the recorded minting workflow is the off-router `migrate-changelog`
//!     (absent from the selectable catalog: it is `selectable: false`, never offered
//!     by a bare `jigc start`);
//!   - **stages** the foreign bytes into the task working area;
//!   - **feeds the source seam** — the staged foreign bytes reach `ComposeContext`'s
//!     `source` field, proven by the composed view emitting the foreign content
//!     verbatim through the `{{source}}` placeholder.
//!
//! The `migrate-changelog` workflow + its author step are T3's deliverable; this
//! test installs a **minimal project-layer `migrate-changelog.yaml` shadow** (a
//! one-step workflow whose body carries `{{ source }}`) so the binary composes the
//! seam end-to-end without depending on the not-yet-shipped pack workflow. The seam
//! feed is the contract under test, not T3's authored prose.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-migrate-seam-{tag}-{}-{:?}",
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

/// Install a minimal project-layer `migrate-changelog` workflow shadow: a one-step
/// off-router workflow whose body surfaces the source seam. This stands in for T3's
/// pack workflow so the binary composes the seam end-to-end; the project shadow is
/// discovered by the cascade (a `workflows/<id>.yaml` basename shadows the pack).
fn install_migrate_workflow_shadow(root: &Path) {
    let workflows = root.join(".jigc").join("config").join("workflows");
    fs::create_dir_all(&workflows).expect("create project workflows dir");
    fs::write(
        workflows.join("migrate-changelog.yaml"),
        "---\nwhen: migrate a foreign changelog into the managed changelog\ncreates-task: true\nselectable: false\nallows-create: [{type: changelog, as: changelog}]\n---\n{{ include: step:rewrite-foreign }}\n",
    )
    .expect("write migrate-changelog shadow");
    let steps = root.join(".jigc").join("config").join("steps");
    fs::create_dir_all(&steps).expect("create project steps dir");
    fs::write(
        steps.join("rewrite-foreign.yaml"),
        "Here is the foreign changelog to rewrite into managed shape:\n{{ source }}\nRewrite it now through the write verbs.\n",
    )
    .expect("write rewrite-foreign step");
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
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
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
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// A realistic multi-release foreign Keep-a-Changelog file.
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

#[test]
fn migrate_mints_off_router_task_and_feeds_the_source_seam() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");
    install_migrate_workflow_shadow(repo.path());

    // Write the foreign CHANGELOG.md at repo root (the Unmanaged target).
    let foreign_path = repo.path().join("CHANGELOG.md");
    fs::write(&foreign_path, FOREIGN).expect("write foreign CHANGELOG.md");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    let stdout = ok_stdout(out, "jigc migrate CHANGELOG.md --as changelog");

    // The off-router migration task is minted under the `migrate-<doctype>` id — NOT
    // the bare doctype name, which collides with `--task changelog` / the `changelog`
    // doctype namespace (`auto-migration.md` → Hardening #9). Its working area + base
    // pin exist, and the bare `changelog` namespace stays free.
    let task = "migrate-changelog";
    let task_dir = repo.path().join(".jigc").join("tasks").join(task);
    assert!(
        task_dir.join("base.json").is_file(),
        "the migration task's base pin must exist at {task_dir:?}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("changelog")
            .exists(),
        "the migration must not squat the bare `changelog` task namespace",
    );

    // The bare `changelog` namespace stays free: an independent task minted at
    // `changelog` does not serial-collide with the migration.
    let independent = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "single-task", "changelog"],
    );
    ok_stdout(independent, "jigc start --workflow single-task changelog");
    assert!(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("changelog")
            .join("base.json")
            .is_file(),
        "an independent `changelog` task mints without colliding with the migration",
    );

    // The recorded minting workflow is the off-router migrate workflow — NOT a
    // selectable catalog entry (a bare `jigc start` orient never lists it).
    let recorded = fs::read_to_string(task_dir.join("workflow")).expect("read recorded workflow");
    assert_eq!(
        recorded, "migrate-changelog",
        "the migration task records the off-router migrate workflow",
    );
    let orient = ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["start"]),
        "jigc start",
    );
    assert!(
        !orient.contains("migrate-changelog"),
        "the off-router migrate workflow must be absent from the selectable catalog; orient:\n{orient}",
    );

    // The foreign bytes are STAGED into the task working area, verbatim.
    let staged_source = fs::read_to_string(task_dir.join("source")).expect("read staged source");
    assert_eq!(
        staged_source, FOREIGN,
        "the foreign bytes are staged verbatim into the task",
    );

    // The source seam REACHED ComposeContext.source: the composed view emits the
    // foreign content verbatim through the `{{source}}` placeholder.
    assert!(
        stdout.contains(FOREIGN.trim_end()),
        "the composed view must surface the foreign content through the source seam; stdout:\n{stdout}",
    );
}
