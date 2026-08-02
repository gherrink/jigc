//! End-to-end integration test for `jigc config insert-step` (Increment 3, T3).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! the structural-override authoring path lands a runnable override end-to-end
//! (`design/overrides.md` → Authoring deltas; `design/worked-examples.md` → 3a):
//! `jigc config insert-step --workflow single-task --after implement ./extra.yaml`
//! writes the native step `.jigc/config/steps/extra.yaml` (id = file basename) +
//! the `insert-step` delta into the project manifest, and a subsequent bare
//! `jigc start "<intent>"` composes `single-task` with `extra` in the include
//! list — proven on the emitted bytes through the binary.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (composition mints,
//! which reads HEAD), and a self-cleaning `TempDir` keeps the test off the
//! developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-config-insert-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (composition mints, which reads
/// HEAD), and create the `.jigc/config/` project layer with a manifest that flips
/// `default-workflow` to `single-task` so a bare `jigc start "<intent>"` composes
/// the work-workflow whose include list the insert-step delta mutates.
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

/// Run `jigc start <args>` with `cwd = repo` and `$HOME = home`.
fn run_start(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn insert_step_writes_native_file_and_delta_then_compose_includes_the_new_step() {
    // The T3 done-criterion: `config insert-step --workflow single-task --after
    // implement ./extra.yaml` writes the native step + the insert-step delta, and a
    // subsequent bare `jigc start "<intent>"` composes single-task with `extra` in
    // the include list — asserted on the emitted bytes through the binary.
    let repo = TempDir::new("insert");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // The source file the verb reads; its basename (`extra`) becomes the native
    // step id. A literal body so the composed bytes carry a unique marker.
    let source = repo.path().join("extra.yaml");
    fs::write(&source, "Run the extra project step before finalizing.\n")
        .expect("write source step file");

    let out = run_config(
        repo.path(),
        home.path(),
        &[
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "implement",
            "./extra.yaml",
        ],
    );
    assert!(
        out.status.success(),
        "`jigc config insert-step ...` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The native step landed at `.jigc/config/steps/extra.yaml` (id = basename),
    // carrying the source bytes verbatim.
    let native = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("steps")
        .join("extra.yaml");
    assert_eq!(
        fs::read_to_string(&native).expect("native step written"),
        "Run the extra project step before finalizing.\n",
        "the native step file must carry the source bytes",
    );

    // The insert-step delta landed in the manifest (preserving the `scalar:` flip).
    let manifest = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
    )
    .expect("manifest written");
    let doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&manifest).expect("manifest is valid YAML");
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
        Some("insert-step"),
        "the recorded delta must be an insert-step; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("target").and_then(serde_yaml_ng::Value::as_str),
        Some("workflow:single-task"),
        "the insert target must be the bare workflow id; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("after").and_then(serde_yaml_ng::Value::as_str),
        Some("implement"),
        "the anchor must ride the `after:` key; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("with").and_then(serde_yaml_ng::Value::as_str),
        Some("step:extra"),
        "`with:` must reference the native step by basename id; got:\n{manifest}",
    );

    // The runnable override lands end-to-end: a bare `jigc start "<intent>"`
    // composes single-task with the new `extra` step body in the include list,
    // immediately after the pack `implement` body — proven on emitted bytes.
    let compose = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "the compose over the inserted step must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    let implement_at = stdout
        .find("Implement the change directly in the working tree.")
        .expect("pack implement body composes");
    let extra_at = stdout
        .find("Run the extra project step before finalizing.")
        .unwrap_or_else(|| panic!("the inserted `extra` step body must compose; got:\n{stdout}"));
    assert!(
        implement_at < extra_at,
        "`extra` must compose immediately after `implement` (--after implement); got:\n{stdout}",
    );
}

#[test]
fn insert_step_rejects_unknown_anchor_and_colliding_basename_and_writes_nothing() {
    // The T3 negative path: an unknown anchor (`--after nonesuch`) and a colliding
    // basename (`./implement.yaml` — `implement` is already a step id) each exit
    // non-zero with their route, and neither writes the native file or the delta.
    let repo = TempDir::new("reject");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let manifest_path = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    // (a) An unknown anchor — `nonesuch` is not in single-task's include list.
    fs::write(repo.path().join("extra.yaml"), "Run the extra step.\n")
        .expect("write source step file");
    let unknown = run_config(
        repo.path(),
        home.path(),
        &[
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "nonesuch",
            "./extra.yaml",
        ],
    );
    let stderr = String::from_utf8(unknown.stderr).expect("utf-8 stderr");
    assert!(
        !unknown.status.success(),
        "an unknown anchor must exit non-zero; got {:?}",
        unknown.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the rejection must name the unknown anchor and carry a route; got:\n{stderr}",
    );

    // (b) A colliding basename — `implement` is already a pack step id, so the
    // native file's basename would shadow it rather than add a new unit.
    fs::write(repo.path().join("implement.yaml"), "shadowing body\n")
        .expect("write colliding source file");
    let collision = run_config(
        repo.path(),
        home.path(),
        &[
            "insert-step",
            "--workflow",
            "single-task",
            "--after",
            "locate",
            "./implement.yaml",
        ],
    );
    let stderr = String::from_utf8(collision.stderr).expect("utf-8 stderr");
    assert!(
        !collision.status.success(),
        "a colliding basename must exit non-zero; got {:?}",
        collision.status,
    );
    assert!(
        stderr.contains("implement") && stderr.contains("route:"),
        "the collision must name the colliding step id and carry a route; got:\n{stderr}",
    );

    // Neither rejection wrote anything: no `steps/` dir, and the manifest is unchanged.
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("steps")
            .exists(),
        "a rejected insert-step must write no native step file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected insert-step must leave the manifest byte-unchanged",
    );
}
