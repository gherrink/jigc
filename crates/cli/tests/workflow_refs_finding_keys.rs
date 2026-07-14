//! M42 Increment 9 / T7 — the `workflow-refs.*` family keys at the **pack resource**, through
//! the real binary (`design/command-output-contract.md` → `workflow-refs.*` — the pack-resource
//! form, keyed at the resource + the declared non-unique exceptions).
//!
//! The contract **already pinned** these 28 codes under the pack-resource form; the code emitted
//! `Location::at(…)` at every one of their 37 sites, so every `workflow-refs.*` finding the
//! store sweep reports projected the degenerate key `(code, null)`. Two unresolvable
//! `{{cli.…}}` refs in one step rode one `jigc validate --format json` array with **byte-
//! identical keys**, and a driver deserializing it could not tell them apart — nor tell them
//! from a *different* workflow's broken include.
//!
//! What "keyed" means here is the doc's own granularity, and it is not "one key per finding":
//! the family is **keyed at the resource and declared non-unique below it**. Two bad refs in one
//! step *correctly* collapse to one key — a workflow-def break means **the pack is broken**, a
//! defect a pack author fixes once, not a corpus finding a driver tracks per-occurrence. So the
//! green is a **non-null** target on both, and **distinct resources → distinct keys**.
//!
//! The arm drives the emitted envelope, never a reconstruction: `jigc validate --format json` is
//! run over a store whose project layer shadows one step (two catalog-absent command-refs) and
//! one workflow (a dangling include), and the assertions read the `key` the emitted JSON carries.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-workflow-refs-keys-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — the real
/// subprocess the `jigc validate` pre-flight resolves (the `validate_command.rs` idiom).
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Run `jigc <args>` against the embedded dev pack, with the real `doc-code` probe selected.
fn jigc(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// A committed repo with the project layer present (the locate-preamble's gate) — no managed
/// docs, so the sweep's content families are silent and the `workflow-refs.*` family is the
/// only producer.
fn seed_store(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// A project **step** shadow (`.jigc/config/steps/implement-quick.yaml`) whose body carries
/// **two** lone `{{cli.<id>}}` refs naming no catalog entry — the collision the family was named
/// for: both are `workflow-refs.command-ref-resolves`, in one step, in one sweep.
fn seed_two_dangling_command_refs(repo: &Path) {
    let steps = repo.join(".jigc").join("config").join("steps");
    fs::create_dir_all(&steps).expect("create project steps dir");
    fs::write(
        steps.join("implement-quick.yaml"),
        "Apply the fix, then record it:\n\
         \n\
         {{ cli.no-such-alpha }}\n\
         {{ cli.no-such-beta }}\n",
    )
    .expect("write the project step shadow");
}

/// A project **workflow** shadow (`.jigc/config/workflows/single-task.yaml`) carrying a dangling
/// `{{ include: step:not-a-step }}` — a *different* pack resource, so a *different* key.
fn seed_dangling_include(repo: &Path) {
    let workflows = repo.join(".jigc").join("config").join("workflows");
    fs::create_dir_all(&workflows).expect("create project workflows dir");
    fs::write(
        workflows.join("single-task.yaml"),
        "---\n\
         when: implement one scoped change end-to-end\n\
         creates-task: true\n\
         ---\n\
         {{ include: step:not-a-step }}\n",
    )
    .expect("write the project workflow shadow");
}

/// The findings array of `jigc validate --format json`.
fn findings(out: &std::process::Output) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`jigc validate --format json` emits the pinned envelope ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the report carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `key.target` of every finding carrying `code`.
fn targets(findings: &[serde_json::Value], code: &str) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == code)
        .map(|f| f["key"]["target"].clone())
        .collect()
}

/// Two catalog-absent command-refs in one step key at **that step** (a non-null, per-resource
/// target — not `null`, and not two invented sub-keys), while a second workflow's dangling
/// include keys at **that workflow**: distinct pack resources → distinct keys.
#[test]
fn workflow_refs_findings_key_at_their_pack_resource() {
    let repo = TempDir::new("resource");
    seed_store(repo.path());
    seed_two_dangling_command_refs(repo.path());
    seed_dangling_include(repo.path());

    let out = jigc(repo.path(), &["validate", "--format", "json"]);
    let findings = findings(&out);

    // The step resource — both refs, one key, and it is not `null`.
    let refs = targets(&findings, "workflow-refs.command-ref-resolves");
    assert_eq!(
        refs.len(),
        2,
        "both catalog-absent refs surface in one sweep; findings:\n{findings:#?}"
    );
    for target in &refs {
        assert_eq!(
            target, "step:implement-quick",
            "a command-ref break keys at the step it lives in — the pack-resource form; \
             findings:\n{findings:#?}"
        );
    }

    // The workflow resource — a different subject, so a different key.
    let includes = targets(&findings, "workflow-refs.include-resolves");
    assert_eq!(
        includes.len(),
        1,
        "the shadowed workflow's dangling include surfaces once; findings:\n{findings:#?}"
    );
    assert_eq!(
        includes[0], "workflow:single-task",
        "a dangling include keys at the workflow whose body names it — the pack-resource form; \
         findings:\n{findings:#?}"
    );

    assert_ne!(
        refs[0], includes[0],
        "distinct pack resources project distinct keys — the whole point of the target",
    );
}
