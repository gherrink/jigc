//! M39 Increment 3 / T1 — the freeze-exempt `milestone-record` methodology doctype
//! loads and composes under the **`[dev ▸ methodology]`** composition, proven through
//! the real `jigc describe` binary (`design/team-ready-state.md` → The milestone-record
//! doctype; `implementation/roadmap.md` → M39 Increment 3).
//!
//! The methodology pack ships no `schema-manifest.yaml`, so a new schema dropped into
//! `crates/cli/packs/methodology/schemas/` auto-registers by directory (freeze-exempt). The
//! observable proof that the novel all-machine-set + empty-repeatable schema is **real
//! and composable** is that `describe`:
//!   - exits 0 (a schema that failed to parse would `bail` out of `load_schemas`), AND
//!   - narrates `milestone-record` with its authored `description:` / `usage:` woven
//!     facts-not-advice ("`<type>` is `<description>`. Reach for it when `<usage>`.").
//!
//! Asserted on the EMITTED bytes of the real binary (hardening #4), so a pack file that
//! drops or garbles the doctype's prose fails here. Scaffolding mirrors
//! `design_altitude_schemas.rs`: the binary path from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/packs/methodology`, a real `git init`
//! temp repo, and a self-cleaning `TempDir`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-milestone-record-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`) — the literal
/// directory a `.jigc/config/packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// (the setup gate the cascade + `describe` require).
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

/// Record the methodology pack in the project layer's `packs.yaml` — listed highest,
/// over the implicit embedded dev base: the `[dev ▸ methodology]` composition.
fn write_packs_yaml(repo: &Path, listed: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", listed.display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

#[test]
fn describe_narrates_milestone_record_under_dev_methodology() {
    let repo = TempDir::new("describe");
    init_repo(repo.path());
    let home = TempDir::new("home");
    write_packs_yaml(repo.path(), &methodology_pack_tree());

    let out = run(repo.path(), home.path(), &["describe"]);
    assert!(
        out.status.success(),
        "`jigc describe` over the `[dev ▸ methodology]` composition must exit 0 — a schema \
         that failed to parse would bail; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The authored `description:` is woven verbatim ("`<type>` is `<description>`."),
    // so its presence proves the schema loaded, composed under the union, and carries
    // the authored prose — the freeze-exempt auto-register worked.
    assert!(
        stdout.contains(
            "milestone-record is The per-milestone team-ready state record — the shared base-SHA pin, the ordered sub-task list, and each sub-task's intent and status, resumable from a fresh clone."
        ),
        "describe must narrate the `milestone-record` doctype's authored description; got:\n{stdout}",
    );
    // The authored `usage:` is woven as the "Reach for it when …" clause.
    assert!(
        stdout.contains(
            "Reach for it when a milestone is executing and its in-flight state (base, sub-tasks, statuses) must be committed and legible so a teammate or fresh session can continue it, not stranded in the gitignored workbench."
        ),
        "describe must narrate the `milestone-record` doctype's authored usage; got:\n{stdout}",
    );
}
