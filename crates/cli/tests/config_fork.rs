//! End-to-end integration test for `jigc config fork` (Increment 5, T4-style proof
//! for the T3 verb).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts the
//! `tracked-fork` authoring verb records the last-resort rung honestly
//! (`design/overrides.md` → Authoring deltas, the `config fork` row; `tracked-fork`
//! hash basis). The acceptance walk:
//!
//! `config fork workflow:single-task#implement` copies the resolved `implement` step
//! body **byte-for-byte** into `.jigc/config/steps/implement.yaml` and records a
//! `tracked-fork` delta whose `base-version` is the pack version and whose
//! `base-hash` is the blake3 hash of those written bytes. Re-running the loader
//! round-trips that delta. An unknown step id, and a fork of an already-forked unit,
//! each exit non-zero with a route and leave the tree byte-untouched.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-config-fork-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and a `.jigc/config/` project layer
/// whose manifest flips `default-workflow` to `single-task` (the work-workflow whose
/// `implement` step the fork copies).
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
    let config = root.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("create project layer");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n",
    )
    .expect("write project manifest");
}

/// Run `jigc config <args>` with `cwd = repo` and `$HOME = home`.
fn run_config(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("config");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn config_dir(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("config")
}

#[test]
fn fork_copies_step_bytes_records_pinned_basis_and_round_trips() {
    let repo = TempDir::new("fork");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_config(
        repo.path(),
        home.path(),
        &["fork", "workflow:single-task#implement"],
    );
    assert!(
        out.status.success(),
        "`jigc config fork ...` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // (a) the native step file holds the pack `implement` body byte-for-byte.
    let native = config_dir(repo.path()).join("steps").join("implement.yaml");
    let written = fs::read(&native).expect("the forked native step file must be written");
    let pack_implement = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/pack/steps/implement.yaml"
    ));
    assert_eq!(
        written,
        pack_implement.as_slice(),
        "the fork must copy the resolved `implement` step body byte-for-byte",
    );

    // (b) the manifest carries a `tracked-fork` delta whose basis is pinned: the
    //     `base-version` is the pack (binary) version, and the `base-hash` is the
    //     blake3 of the written bytes.
    let manifest = fs::read_to_string(config_dir(repo.path()).join("manifest.yaml"))
        .expect("manifest written");
    let doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&manifest).expect("manifest is valid YAML");
    // The `scalar:` flip survives the delta append.
    assert_eq!(
        doc.get("scalar")
            .and_then(|s| s.get("default-workflow"))
            .and_then(serde_yaml_ng::Value::as_str),
        Some("single-task"),
        "the `scalar:` flip must survive the delta append; got:\n{manifest}",
    );
    let delta = doc
        .get("deltas")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .and_then(|s| s.first())
        .expect("one delta entry");
    assert_eq!(
        delta.get("kind").and_then(serde_yaml_ng::Value::as_str),
        Some("tracked-fork"),
        "the recorded delta must be a tracked-fork; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("target").and_then(serde_yaml_ng::Value::as_str),
        Some("workflow:single-task#implement"),
        "the fork target must carry the `#<step-id>`; got:\n{manifest}",
    );
    assert_eq!(
        delta
            .get("base-version")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(env!("CARGO_PKG_VERSION")),
        "`base-version` must be the pack (binary) version; got:\n{manifest}",
    );
    assert_eq!(
        delta
            .get("base-hash")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(engine::file_state::hash_bytes(&written).as_str()),
        "`base-hash` must be the blake3 of the written file's bytes; got:\n{manifest}",
    );

    // (c) the loader round-trips the recorded fork delta (the T2 path), on the binary.
    let resume = run_config(
        repo.path(),
        home.path(),
        // A second fork of a *different* step exercises the loader reading the prior
        // fork back without error (round-trip), then records its own.
        &["fork", "workflow:single-task#locate"],
    );
    assert!(
        resume.status.success(),
        "a second fork must succeed (the loader round-trips the prior fork); got {:?}\nstderr:\n{}",
        resume.status,
        String::from_utf8_lossy(&resume.stderr),
    );
    let manifest = fs::read_to_string(config_dir(repo.path()).join("manifest.yaml"))
        .expect("manifest written");
    let doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&manifest).expect("manifest is valid YAML");
    let forks = doc
        .get("deltas")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .map(|s| {
            s.iter()
                .filter(|d| {
                    d.get("kind").and_then(serde_yaml_ng::Value::as_str) == Some("tracked-fork")
                })
                .count()
        })
        .unwrap_or(0);
    assert_eq!(
        forks, 2,
        "both fork deltas must be recorded after the round-trip; got:\n{manifest}",
    );
}

#[test]
fn unknown_step_id_is_rejected_and_writes_nothing() {
    let repo = TempDir::new("unknown");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let manifest_path = config_dir(repo.path()).join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    let out = run_config(
        repo.path(),
        home.path(),
        &["fork", "workflow:single-task#nonesuch"],
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "an unknown step id must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the rejection must name the unknown step and carry a route; got:\n{stderr}",
    );

    assert!(
        !config_dir(repo.path()).join("steps").exists(),
        "a rejected fork must write no native step file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected fork must leave the manifest byte-unchanged",
    );
}

#[test]
fn fork_of_already_forked_unit_is_rejected_and_leaves_tree_untouched() {
    let repo = TempDir::new("twice");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // First fork succeeds and shadows `implement` in the project layer.
    let first = run_config(
        repo.path(),
        home.path(),
        &["fork", "workflow:single-task#implement"],
    );
    assert!(
        first.status.success(),
        "the first fork must succeed; got {:?}\nstderr:\n{}",
        first.status,
        String::from_utf8_lossy(&first.stderr),
    );

    let manifest_path = config_dir(repo.path()).join("manifest.yaml");
    let manifest_before = fs::read_to_string(&manifest_path).expect("manifest exists");
    let native = config_dir(repo.path()).join("steps").join("implement.yaml");
    let native_before = fs::read(&native).expect("native exists");

    // Second fork of the same (already-forked) unit is rejected.
    let second = run_config(
        repo.path(),
        home.path(),
        &["fork", "workflow:single-task#implement"],
    );
    let stderr = String::from_utf8(second.stderr).expect("utf-8 stderr");
    assert!(
        !second.status.success(),
        "a fork of an already-forked unit must exit non-zero; got {:?}",
        second.status,
    );
    assert!(
        stderr.contains("route:"),
        "the rejection must carry a route; got:\n{stderr}",
    );

    // The tree is untouched: the manifest and the native file are byte-unchanged.
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        manifest_before,
        "the rejected re-fork must leave the manifest byte-unchanged",
    );
    assert_eq!(
        fs::read(&native).expect("native still readable"),
        native_before,
        "the rejected re-fork must leave the native step file byte-unchanged",
    );
}
