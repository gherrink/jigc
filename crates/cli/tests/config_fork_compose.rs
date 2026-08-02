//! End-to-end compose proof for `jigc config fork` (Increment 5, T4).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and proves the
//! `tracked-fork` apply path is *just a phase-2 file shadow, no new resolution
//! logic*, and that the recorded basis is honestly pinned — all on the emitted
//! bytes through the binary (`design/roadmap.md` → Increment 5 Proves;
//! `design/overrides.md` → `tracked-fork` hash basis, `replace` vs `tracked-fork`).
//!
//! The acceptance walk:
//!
//!   1. Baseline: a bare `jigc start "<intent>"` composes `single-task`; capture the
//!      composed `implement` region (the bytes between the `implement` body's leading
//!      marker and the next step's marker).
//!   2. `config fork workflow:single-task#implement` copies the resolved `implement`
//!      body into `.jigc/config/steps/implement.yaml`. Re-composing yields a *faithful
//!      copy*: the forked (now project-owned) `implement` region is **byte-identical**
//!      to the pre-fork baseline region — the shadow resolves through the same
//!      phase-2 path with the same bytes.
//!   3. Editing the forked `steps/implement.yaml` (appending a unique house rule) and
//!      re-composing surfaces the project edit — the shadow diverges.
//!   4. After that edit, the manifest's recorded `base-hash` still equals
//!      `hash_bytes` of the ORIGINAL pack `implement` bytes — the basis is pinned to
//!      the ancestor, unmoved by the later divergence (the M5 precondition: a fork
//!      that knows what it forked).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-fork-compose-{tag}-{}-{:?}",
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
/// the work-workflow whose `implement` step the fork shadows.
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

/// Run `jigc start <args>` with `cwd = repo` and `$HOME = home`, returning stdout.
fn compose(repo: &Path, home: &Path, intent: &str) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start").arg(intent);
    let out = command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc start {intent:?}` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// The pack `implement` body's leading marker — present iff `step:implement`
/// composes (from the pack default or its project shadow).
const IMPLEMENT_MARKER: &str = "Implement the change directly in the working tree.";
/// The pack `superseded-context` body's leading marker — the step that composes
/// immediately after `implement`, so it bounds the `implement` region.
const SUPERSEDED_MARKER: &str = "If your decision supersedes an earlier one";

/// Extract the composed `implement` region: the bytes from the `implement` body's
/// leading marker up to (exclusive) the next step's marker, with the per-intent
/// task slug normalized out. The region embeds `{{ task.commit#summary }}`, which
/// resolves to the live task id (derived from the intent) — a data value, not a
/// fork artifact — so normalizing the slug isolates the *step body* bytes, which
/// are what the faithful-copy contract is about.
fn implement_region(stdout: &str, slug: &str) -> String {
    let start = stdout
        .find(IMPLEMENT_MARKER)
        .unwrap_or_else(|| panic!("the `implement` body must compose; got:\n{stdout}"));
    let end = stdout[start..]
        .find(SUPERSEDED_MARKER)
        .map(|rel| start + rel)
        .unwrap_or_else(|| {
            panic!("superseded-context must compose after implement; got:\n{stdout}")
        });
    stdout[start..end].replace(slug, "<task>")
}

#[test]
fn forked_step_composes_as_a_faithful_copy_then_diverges_on_edit_with_pinned_basis() {
    let repo = TempDir::new("walk");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // (1) Baseline: capture the composed `implement` region before any fork. The
    //     intent slug is normalized out so only the *step body* bytes are compared.
    let baseline_out = compose(repo.path(), home.path(), "baseline check");
    let baseline_region = implement_region(&baseline_out, "baseline-check");

    // (2) Fork `single-task#implement` into the project layer.
    let forked = run_config(
        repo.path(),
        home.path(),
        &["fork", "workflow:single-task#implement"],
    );
    assert!(
        forked.status.success(),
        "`jigc config fork ...` must exit 0; got {:?}\nstderr:\n{}",
        forked.status,
        String::from_utf8_lossy(&forked.stderr),
    );

    // The pinned basis: the blake3 of the ORIGINAL pack `implement` bytes, captured
    // now (the fork just copied them, byte-for-byte, into the native file).
    let native = repo
        .path()
        .join(".jigc")
        .join("config")
        .join("steps")
        .join("implement.yaml");
    let original_pack_bytes = fs::read(&native).expect("the forked native step file");
    let pinned_basis = engine::file_state::hash_bytes(&original_pack_bytes);

    // (2 cont.) Re-compose: the forked (now project-owned) `implement` region is
    // byte-identical to the pre-fork baseline — a faithful copy through the same
    // phase-2 shadow path.
    let after_fork_out = compose(repo.path(), home.path(), "after fork check");
    assert_eq!(
        implement_region(&after_fork_out, "after-fork-check"),
        baseline_region,
        "the forked `implement` body must compose byte-identical to the pre-fork \
         baseline (a faithful copy); got:\n{after_fork_out}",
    );

    // (3) Edit the forked native step: append a unique house rule. Re-composing
    // surfaces the project edit — the shadow diverges from the pack ancestor.
    let house_rule = "House rule: run the project lint probe before you finalize.";
    let mut edited = original_pack_bytes.clone();
    edited.extend_from_slice(format!("\n{house_rule}\n").as_bytes());
    fs::write(&native, &edited).expect("edit the forked native step");

    let after_edit_out = compose(repo.path(), home.path(), "after edit check");
    assert!(
        after_edit_out.contains(house_rule),
        "the edited forked `implement` body must surface in composed output \
         (the shadow diverges); got:\n{after_edit_out}",
    );
    // The forked region still anchors on the pack `implement` body it copied.
    assert!(
        implement_region(&after_edit_out, "after-edit-check").contains(IMPLEMENT_MARKER),
        "the edited shadow must retain its copied `implement` body; got:\n{after_edit_out}",
    );

    // (4) The recorded `base-hash` is unmoved by the edit: it still equals
    // `hash_bytes` of the ORIGINAL pack `implement` bytes — the basis is pinned to
    // the ancestor (the M5 precondition: a fork that knows what it forked).
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
        .and_then(|s| {
            s.iter().find(|d| {
                d.get("target").and_then(serde_yaml_ng::Value::as_str)
                    == Some("workflow:single-task#implement")
            })
        })
        .expect("the implement fork delta is recorded");
    assert_eq!(
        delta.get("kind").and_then(serde_yaml_ng::Value::as_str),
        Some("tracked-fork"),
        "the recorded delta must be a tracked-fork; got:\n{manifest}",
    );
    assert_eq!(
        delta
            .get("base-hash")
            .and_then(serde_yaml_ng::Value::as_str),
        Some(pinned_basis.as_str()),
        "the recorded `base-hash` must still equal the blake3 of the ORIGINAL pack \
         `implement` bytes, unchanged by the later edit; got:\n{manifest}",
    );
}
