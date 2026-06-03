//! End-to-end integration test for `jigc config replace-step` / `remove-step`
//! (Increment 3, T4).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! the two remaining `structural-op` authoring verbs land runnable overrides
//! end-to-end (`design/overrides.md` → Authoring deltas; `design/worked-examples.md`
//! → 3a). The acceptance walk is flow 3a verbatim:
//!
//! `config replace-step workflow:single-task#implement ./project-implement.yaml`
//! — where `project-implement.yaml` re-includes `step:implement` + a house rule —
//! writes the native step `.jigc/config/steps/project-implement.yaml` (id = file
//! basename) + the `replace-step` delta, and a subsequent bare `jigc start
//! "<intent>"` composes `single-task` with the include list `[locate,
//! project-implement, superseded-context, finalize]` — the pack `implement` body
//! (re-included) and the house rule both present. `config remove-step
//! workflow:single-task#superseded-context` drops that step's body from the
//! composed output. An absent target (`#nonesuch`) on each verb exits non-zero
//! with its route and writes nothing.
//!
//! All proven on the emitted bytes through the binary, matching `config_insert_step.rs`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-config-rr-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (composition mints, which reads
/// HEAD), and create the `.jigc/config/` project layer with a manifest that flips
/// `default-workflow` to `single-task` so a bare `jigc start "<intent>"` composes
/// the work-workflow whose include list the structural deltas mutate.
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

/// The pack `implement` body's leading marker — present iff `step:implement`
/// composes (directly or via the re-include in `project-implement`).
const IMPLEMENT_MARKER: &str = "Implement the change directly in the working tree.";
/// The pack `superseded-context` body's leading marker.
const SUPERSEDED_MARKER: &str = "If your decision supersedes an earlier one";

#[test]
fn replace_step_re_includes_pack_step_and_adds_house_rule_through_compose() {
    // Flow 3a verbatim: `replace-step workflow:single-task#implement
    // ./project-implement.yaml` — the native step re-includes `step:implement` and
    // adds a house rule. The composed include list becomes
    // `[locate, project-implement, superseded-context, finalize]`; the pack
    // `implement` body (re-included) *and* the house rule both appear — asserted on
    // the emitted bytes through the binary.
    let repo = TempDir::new("replace");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // The source file the verb reads; its basename (`project-implement`) becomes the
    // native step id. It re-includes the pack step, then adds a unique house rule.
    let house_rule = "Before you finalize, run the project lint probe and fix any findings.";
    fs::write(
        repo.path().join("project-implement.yaml"),
        format!("{{{{ include: step:implement }}}}\n\n{house_rule}\n"),
    )
    .expect("write source step file");

    let out = run_config(
        repo.path(),
        home.path(),
        &[
            "replace-step",
            "workflow:single-task#implement",
            "./project-implement.yaml",
        ],
    );
    assert!(
        out.status.success(),
        "`jigc config replace-step ...` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The native step landed at `.jigc/config/steps/project-implement.yaml`.
    let native = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("steps")
        .join("project-implement.yaml");
    assert!(native.exists(), "the native step file must be written");

    // The replace-step delta landed in the manifest (preserving the `scalar:` flip).
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
        Some("replace-step"),
        "the recorded delta must be a replace-step; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("target").and_then(serde_yaml_ng::Value::as_str),
        Some("workflow:single-task#implement"),
        "the replace target must carry the `#<step-id>`; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("with").and_then(serde_yaml_ng::Value::as_str),
        Some("step:project-implement"),
        "`with:` must reference the native step by basename id; got:\n{manifest}",
    );

    // The runnable override lands end-to-end: a bare `jigc start "<intent>"`
    // composes single-task with the re-included pack body *and* the house rule, in
    // the order `[locate, project-implement (= implement body + house rule),
    // superseded-context, finalize]` — proven on emitted bytes.
    let compose = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "the compose over the replaced step must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    // Both bodies compose: the re-included pack `implement` body *and* the project
    // house rule. The replaced position carries both — the override re-includes
    // rather than forks.
    let implement_at = stdout.find(IMPLEMENT_MARKER).unwrap_or_else(|| {
        panic!("the re-included pack `implement` body must compose; got:\n{stdout}")
    });
    let house_at = stdout
        .find(house_rule)
        .unwrap_or_else(|| panic!("the project house rule must compose; got:\n{stdout}"));
    // The intra-step order is the **in-place** expansion contract
    // (`design/workflow-dialect.md` → Composition: "expanded recursively in place";
    // `worked-examples.md` 3a phase-7): `project-implement`'s body is
    // `{{ include: step:implement }}` then the house rule, so the re-included pack
    // `implement` body composes **followed by** the house rule — never hoisted.
    assert!(
        implement_at < house_at,
        "in-place expansion: the re-included pack `implement` body must precede the house rule; got:\n{stdout}",
    );
    // The include-list order: locate before the replaced position's content, and
    // superseded-context after it — the list became
    // `[locate, project-implement, superseded-context, finalize]`.
    let locate_at = stdout
        .find("Reason about the change.")
        .expect("the locate step composes");
    let superseded_at = stdout
        .find(SUPERSEDED_MARKER)
        .expect("superseded-context still composes after the replace");
    assert!(
        locate_at < implement_at && house_at < superseded_at,
        "the include order must be [locate, project-implement, superseded-context, ...]; got:\n{stdout}",
    );
}

#[test]
fn remove_step_drops_the_step_from_composed_output() {
    // `remove-step workflow:single-task#superseded-context` drops that step's body
    // from the composed output — asserted on the emitted bytes through the binary.
    let repo = TempDir::new("remove");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Baseline: superseded-context composes before the remove. A distinct intent
    // from the post-remove compose so the two mint distinct task ids (no collision).
    let baseline = run_start(repo.path(), home.path(), &["baseline check"]);
    let baseline_out = String::from_utf8(baseline.stdout).expect("utf-8 stdout");
    assert!(
        baseline_out.contains(SUPERSEDED_MARKER),
        "the pack-default compose must include superseded-context; got:\n{baseline_out}",
    );

    let out = run_config(
        repo.path(),
        home.path(),
        &["remove-step", "workflow:single-task#superseded-context"],
    );
    assert!(
        out.status.success(),
        "`jigc config remove-step ...` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // remove-step writes no native file (no `with:`), only the delta.
    let manifest = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("config")
            .join("manifest.yaml"),
    )
    .expect("manifest written");
    let doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&manifest).expect("manifest is valid YAML");
    let delta = doc
        .get("deltas")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .and_then(|s| s.first())
        .expect("one delta entry");
    assert_eq!(
        delta.get("kind").and_then(serde_yaml_ng::Value::as_str),
        Some("remove-step"),
        "the recorded delta must be a remove-step; got:\n{manifest}",
    );
    assert_eq!(
        delta.get("target").and_then(serde_yaml_ng::Value::as_str),
        Some("workflow:single-task#superseded-context"),
        "the remove target must carry the `#<step-id>`; got:\n{manifest}",
    );
    assert!(
        delta.get("with").is_none(),
        "a remove-step records no `with:` (no native file); got:\n{manifest}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("steps")
            .exists(),
        "a remove-step must write no native step file",
    );

    // The composed output drops the superseded-context body.
    let compose = run_start(repo.path(), home.path(), &["add rate limiter"]);
    let stdout = String::from_utf8(compose.stdout).expect("utf-8 stdout");
    assert!(
        compose.status.success(),
        "the compose over the removed step must exit 0; got {:?}\nstderr:\n{}",
        compose.status,
        String::from_utf8_lossy(&compose.stderr),
    );
    assert!(
        !stdout.contains(SUPERSEDED_MARKER),
        "the removed superseded-context body must not compose; got:\n{stdout}",
    );
    // The other steps survive — the remove is surgical.
    assert!(
        stdout.contains(IMPLEMENT_MARKER),
        "the unrelated `implement` step must still compose; got:\n{stdout}",
    );
}

#[test]
fn absent_target_is_rejected_on_each_verb_and_writes_nothing() {
    // An absent `#nonesuch` target on replace-step and on remove-step each exits
    // non-zero with its route, writing neither a native file nor a delta.
    let repo = TempDir::new("absent");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let manifest_path = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("manifest.yaml");
    let before = fs::read_to_string(&manifest_path).expect("seed manifest exists");

    // (a) replace-step against an absent target.
    fs::write(
        repo.path().join("project-implement.yaml"),
        "{{ include: step:implement }}\n",
    )
    .expect("write source step file");
    let replace = run_config(
        repo.path(),
        home.path(),
        &[
            "replace-step",
            "workflow:single-task#nonesuch",
            "./project-implement.yaml",
        ],
    );
    let stderr = String::from_utf8(replace.stderr).expect("utf-8 stderr");
    assert!(
        !replace.status.success(),
        "an absent replace target must exit non-zero; got {:?}",
        replace.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the rejection must name the absent target and carry a route; got:\n{stderr}",
    );

    // (b) remove-step against an absent target.
    let remove = run_config(
        repo.path(),
        home.path(),
        &["remove-step", "workflow:single-task#nonesuch"],
    );
    let stderr = String::from_utf8(remove.stderr).expect("utf-8 stderr");
    assert!(
        !remove.status.success(),
        "an absent remove target must exit non-zero; got {:?}",
        remove.status,
    );
    assert!(
        stderr.contains("nonesuch") && stderr.contains("route:"),
        "the rejection must name the absent target and carry a route; got:\n{stderr}",
    );

    // Neither rejection wrote anything.
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("config")
            .join("steps")
            .exists(),
        "a rejected verb must write no native step file",
    );
    assert_eq!(
        fs::read_to_string(&manifest_path).expect("manifest still readable"),
        before,
        "a rejected verb must leave the manifest byte-unchanged",
    );
}
