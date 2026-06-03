//! End-to-end integration test for `jigc config fill` (Increment 4, T5).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts the
//! `slot-fill` authoring verb lands a runnable fill end-to-end (`design/overrides.md`
//! → Authoring deltas, the `config fill` row; `design/worked-examples.md` → 3c). The
//! acceptance walk is flow 3c:
//!
//! `config fill step:implement#extra-guidance --from-file -` (content via stdin)
//! writes `.jigc/config/fills/extra-guidance.md` (id = fill-id) + the `slot-fill`
//! delta, and a subsequent bare `jigc start "<intent>"` composes the fill content
//! into the `implement` step body — proven on emitted bytes through the binary.
//!
//! The pack `implement` step gets its `{{fill: extra-guidance}}` point in T6, so this
//! test seeds a **project step** `implement.yaml` carrying the point (phase-2 shadow):
//! it exercises the verb's resolved-step-body read (the point-exists write-time check
//! resolves through the cascade, project shadow included) without coupling to T6.
//!
//! Two write-time rejections (non-zero, routed, nothing written): a fill aimed at a
//! `{{fill:}}` point no resolved body declares, and fill content that itself contains
//! a `{{fill:}}` (the no-nested rule — phase 5 does not re-run).

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
            "jigc-config-fill-{tag}-{}-{:?}",
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

/// The `{{fill: extra-guidance}}` point the project `implement` step declares — the
/// resolved-body point the verb checks the target against (and the compose fills).
const PROJECT_IMPLEMENT: &str =
    "Implement the change directly in the working tree.\n\n{{fill: extra-guidance}}\n";

/// Initialize a real git repo with one commit (composition mints, which reads HEAD),
/// a `.jigc/config/` project layer whose manifest flips `default-workflow` to
/// `single-task`, plus a project `steps/implement.yaml` that shadows the pack step and
/// carries the `{{fill: extra-guidance}}` extension point.
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
    fs::create_dir_all(config.join("steps")).expect("create project layer + steps");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  default-workflow: single-task\n",
    )
    .expect("write project manifest");
    // A project `implement` step shadows the pack one, carrying the fill point.
    fs::write(
        config.join("steps").join("implement.yaml"),
        PROJECT_IMPLEMENT,
    )
    .expect("seed project implement step");
}

/// Run `jigc config fill <target> --from-file -` with `content` piped on stdin.
fn run_fill(repo: &Path, home: &Path, target: &str, content: &str) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["config", "fill", target, "--from-file", "-"])
        .current_dir(repo)
        .env("HOME", home)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(content.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("wait for the jigc binary")
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
fn fill_writes_native_file_and_delta_then_composes_into_implement() {
    // Flow 3c verbatim: `config fill step:implement#extra-guidance --from-file -`
    // with the house rule on stdin writes `fills/extra-guidance.md` + the slot-fill
    // delta, and a bare `jigc start "<intent>"` composes the house rule into the
    // `implement` body — proven on emitted bytes through the binary.
    let repo = TempDir::new("fill");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let house_rule =
        "Confirm a changelog entry exists for any user-facing change before finalizing.";
    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        &format!("{house_rule}\n"),
    );
    assert!(
        out.status.success(),
        "`jigc config fill ...` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The native fill file landed at `.jigc/config/fills/extra-guidance.md` (id = fill-id).
    let config = repo.path().join(".jigc").join("config");
    let native = config.join("fills").join("extra-guidance.md");
    assert_eq!(
        fs::read_to_string(&native).expect("the native fill file must be written"),
        format!("{house_rule}\n"),
        "the fill file must carry the stdin content verbatim",
    );

    // The slot-fill delta landed in the manifest (preserving the `scalar:` flip).
    let manifest = fs::read_to_string(config.join("manifest.yaml")).expect("manifest written");
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
        Some("slot-fill"),
        "the recorded delta must be a slot-fill; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("target").and_then(serde_yaml_ng::Value::as_str),
        Some("step:implement#extra-guidance"),
        "the slot-fill target must carry `step:<id>#<fill-id>`; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("content").and_then(serde_yaml_ng::Value::as_str),
        Some("fills/extra-guidance.md"),
        "`content:` must reference the native fill file; got:\n{manifest}",
    );

    // The runnable fill lands end-to-end: a bare `jigc start "<intent>"` composes the
    // house rule into the `implement` body, in place of the `{{fill:}}` point — proven
    // on emitted bytes.
    let compose = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "the compose over the filled step must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    assert!(
        stdout.contains(house_rule),
        "the filled house rule must compose into the implement body; got:\n{stdout}",
    );
    // The `{{fill:}}` point itself never survives into the composed output.
    assert!(
        !stdout.contains("{{fill:"),
        "no `{{{{fill:}}}}` point may survive composition; got:\n{stdout}",
    );
}

#[test]
fn orphan_target_is_rejected_and_writes_nothing() {
    // A fill aimed at a `{{fill:}}` point no resolved step body declares is rejected
    // non-zero with its route, writing neither the native fill file nor a delta.
    let repo = TempDir::new("orphan");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    let manifest_path = config.join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#nonesuch",
        "some content\n",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "an orphan fill target must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the rejection must name the absent point and carry a route; got:\n{stderr}",
    );

    assert!(
        !config.join("fills").exists(),
        "a rejected fill must write no native fill file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected fill must leave the manifest byte-unchanged",
    );
}

#[test]
fn nested_fill_in_content_is_rejected_and_writes_nothing() {
    // Fill content that itself contains a `{{fill:}}` is rejected non-zero with its
    // route (the no-nested rule — phase 5 does not re-run), writing nothing.
    let repo = TempDir::new("nested");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let config = repo.path().join(".jigc").join("config");
    let manifest_path = config.join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    let out = run_fill(
        repo.path(),
        home.path(),
        "step:implement#extra-guidance",
        "house rule\n{{fill: another}}\n",
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "fill content containing `{{{{fill:}}}}` must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("route:"),
        "the rejection must carry a route; got:\n{stderr}",
    );

    assert!(
        !config.join("fills").exists(),
        "a rejected fill must write no native fill file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected fill must leave the manifest byte-unchanged",
    );
}
