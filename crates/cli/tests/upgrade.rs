//! End-to-end integration test for `jigc upgrade` (M5 increment 4, T3).
//!
//! Drives the built `jigc` binary in a seeded repo and asserts the agent-facing
//! contract **on the emitted bytes** (increment-workflow.md hardening #4): the
//! command runs the `override-default` classifier over the recorded deltas against
//! the current (`JIGC_PACK_DIR`-selected) pack, renders findings + routes through
//! the standard renderer, and gates the exit code on `report.has_blocking()` —
//! **report-and-route only**, mutating nothing (`design/overrides.md` → The `jigc
//! upgrade` command, step 3).
//!
//! Two halves of the done-criterion:
//!   (a) a manifest with a blocking-classifying delta (a `remove-step` whose target
//!       the pack omits → `orphaned`) exits non-zero and the emitted output carries
//!       the finding's `severity · code — message` + an indented `route:` line;
//!   (b) an all-clean / no-delta manifest exits zero with the positive no-findings
//!       line.
//! The genuine v1→v2 two-pack walk + re-pin→clean loop is T4 (`flow 7`); this test
//! pins the dispatch + render + gate contract through the binary.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-upgrade-e2e-{tag}-{}-{:?}",
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

/// Seed a repo: a `.git` marker (repo-root discovery walks to it) and a
/// `.jigc/config/` project layer carrying `manifest_text`.
fn seed_repo(root: &Path, manifest_text: &str) {
    fs::create_dir_all(root.join(".git")).expect("mk .git");
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk .jigc/config");
    fs::write(config.join("manifest.yaml"), manifest_text).expect("write manifest");
}

/// Seed a `JIGC_PACK_DIR` filesystem pack carrying the given `(stem, body)` step
/// files — the current pack the classifier compares the recorded deltas against.
fn seed_pack(dir: &TempDir, steps: &[(&str, &str)]) -> PathBuf {
    let pack_root = dir.path().join("pack");
    let steps_dir = pack_root.join("steps");
    fs::create_dir_all(&steps_dir).expect("mk pack/steps");
    for (stem, body) in steps {
        fs::write(steps_dir.join(format!("{stem}.yaml")), body).expect("seed step");
    }
    pack_root
}

/// Run `jigc upgrade <args>` with `cwd = repo`, `$HOME = home`, and the given
/// `JIGC_PACK_DIR` selecting the current pack.
fn run_upgrade(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("upgrade");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn blocking_delta_exits_non_zero_with_the_finding_line_and_route() {
    // (a) A `remove-step` over `workflow:single-task#gone`; the current pack omits
    // `gone` entirely → the existence question orphans it, blocking. The command
    // exits non-zero and the EMITTED bytes carry the finding's
    // `severity · code — message` line + the indented `route:` line.
    let repo = TempDir::new("block");
    let home = TempDir::new("home");
    seed_repo(
        repo.path(),
        "\
deltas:
  - kind: remove-step
    target: workflow:single-task#gone
",
    );
    // The pack carries some *other* step but not `gone`.
    let pack_dir = seed_pack(&repo, &[("implement", "implement body\n")]);

    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        !out.status.success(),
        "a blocking-classifying delta must exit non-zero; got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    // The finding line: `<severity> · <code> — <message>`. Assert each constituent
    // on the emitted bytes (the agent-facing contract).
    assert!(
        stdout.contains("blocking · override-default.target-exists — "),
        "the emitted output must carry the `severity · code — message` finding line; got:\n{stdout}",
    );
    assert!(
        stdout.contains("workflow:single-task#gone"),
        "the finding message must name the orphaned delta's target; got:\n{stdout}",
    );
    // The block-payload envelope: a blocking finding carries an indented `route:`.
    assert!(
        stdout.contains("\n  route: "),
        "the emitted output must carry the indented `route:` line; got:\n{stdout}",
    );

    // Report-and-route only: the manifest is byte-unchanged on disk.
    let manifest = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    assert!(
        fs::read_to_string(&manifest)
            .expect("manifest still readable")
            .contains("workflow:single-task#gone"),
        "`jigc upgrade` must mutate nothing — the manifest's delta survives",
    );
}

#[test]
fn no_delta_manifest_exits_zero_with_the_positive_no_findings_line() {
    // (b) A no-delta manifest (only a `scalar:` block) classifies clean → exit zero,
    // and the emitted output carries the positive no-findings line so the agent sees
    // a positive signal.
    let repo = TempDir::new("clean");
    let home = TempDir::new("home");
    seed_repo(repo.path(), "scalar:\n  default-workflow: single-task\n");
    let pack_dir = seed_pack(&repo, &[("implement", "implement body\n")]);

    let out = run_upgrade(repo.path(), home.path(), &pack_dir, &[]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "a clean / no-delta manifest must exit zero; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        stdout.contains("no findings"),
        "the emitted output must carry the positive no-findings line; got:\n{stdout}",
    );
}
