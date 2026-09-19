//! Real-binary acceptance for the M43 **suppression fence** (law 2, the
//! `decided-task` lesson made mechanical): the eager workflow-front-matter
//! sweep in the pack-source factory requires every `selectable: false`
//! workflow of a **manifest-shipping** pack to declare a
//! `suppressed: {reason, expires}` block — a hidden capability must carry a
//! machine-visible reason that can expire. A shipped-tree `JIGC_PACK_DIR`
//! copy with one hidden workflow's `suppressed:` stripped exits **non-zero at
//! pack-load naming the workflow**; the unmutated copy, the embedded base,
//! and the composed `[dev ▸ methodology]` pair all load clean; a
//! **manifest-less** pack is unchecked (the `assert_schema_freeze` opt-in
//! precedent — only a pack that ships a freeze manifest is held to the pack-load
//! fences).
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract (the `freeze_enforcement.rs` pattern). Design:
//! `design/surface-contract.md` → The suppression fence + The fences
//! (pack-load posture).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-suppress-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// The embedded dev pack tree — the faithful source the on-disk copies mirror.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Copy a shipped pack tree into a fresh temp dir and return the copy's root.
fn pack_copy(tag: &str, src: &Path) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(src, dir.path());
    dir
}

/// Strip the named workflow's whole `suppressed:` block (the key line + its
/// indented body) from a copied pack — the mutation the fence must catch: a workflow
/// the router catalog leaves out, left with no machine-visible reason. Either cause
/// of absence qualifies (`selectable: false`, or `creates-task: false`), which is the
/// fence's subject since M49 Increment 11 / T6.
fn strip_suppressed_block(pack: &Path, workflow: &str) {
    let path = pack.join("workflows").join(format!("{workflow}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied workflow");
    let mut out = String::new();
    let mut dropping = false;
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("suppressed:") {
            dropping = true;
            hit = true;
            continue;
        }
        if dropping {
            if line.starts_with(' ') {
                continue;
            }
            dropping = false;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{workflow}` workflow must declare a `suppressed:` block to strip"
    );
    assert!(
        out.contains("selectable: false") || out.contains("creates-task: false"),
        "the stripped `{workflow}` workflow must stay off the router catalog — \
         `selectable: false` or `creates-task: false`; got:\n{out}"
    );
    fs::write(&path, out).expect("write the stripped workflow");
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// Run `jigc <args>` with NO `JIGC_PACK_DIR` (the embedded / listed-pack path).
fn run_embedded(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

const START: &[&str] = &[
    "start",
    "--workflow",
    "single-task",
    "suppression-fence probe",
];

/// The fence: a dev-pack copy whose hidden `sub-task` lost its `suppressed:`
/// block is **blocked at pack-load** — the composing `jigc start` exits
/// non-zero, and stderr names the offending workflow and the missing block.
#[test]
fn a_stripped_suppressed_block_is_blocked_at_pack_load() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_copy("strip", &embedded_pack_tree());
    init_repo(repo.path());
    strip_suppressed_block(pack.path(), "sub-task");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a hidden workflow with no `suppressed:` block must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("sub-task"),
        "stderr must name the offending `sub-task` workflow; got:\n{stderr}",
    );
    assert!(
        stderr.contains("suppressed"),
        "stderr must name the missing `suppressed:` declaration; got:\n{stderr}",
    );
}

/// The other cause of absence, and the fence's widened subject (M49 Increment 11 /
/// T6): a workflow off the catalog because it **mints no task** owes the reader a
/// reason exactly as a `selectable: false` one does. `ingest-existing` is
/// `creates-task: false`; stripping its `suppressed:` block blocks at pack-load.
///
/// Without this arm the fence would still be keyed on one of the catalog's two
/// exclusion causes, which is the defect T6 repairs — `step:route-to-workflow`
/// promises a reason for every absence, and a fence over half the population buys
/// half the promise.
#[test]
fn a_stripped_creates_task_false_workflow_is_blocked_at_pack_load() {
    let repo = TempDir::new("ct-repo");
    let home = TempDir::new("ct-home");
    let pack = pack_copy("ct-strip", &embedded_pack_tree());
    init_repo(repo.path());
    strip_suppressed_block(pack.path(), "ingest-existing");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a `creates-task: false` workflow with no `suppressed:` block must make `jigc start` \
         exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("ingest-existing") && stderr.contains("suppressed"),
        "stderr must name the offending `ingest-existing` workflow and the missing \
         `suppressed:` declaration; got:\n{stderr}",
    );
}

/// The sweep covers **every** manifest-shipping constituent, not just the base:
/// a *listed* methodology-pack copy whose hidden `planning` lost its
/// `suppressed:` block blocks at pack-load too.
#[test]
fn a_stripped_methodology_workflow_blocks_through_the_listed_pack_path() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = pack_copy("m-strip", &methodology_pack_tree());
    init_repo(repo.path());
    strip_suppressed_block(pack.path(), "planning");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    let out = run_embedded(repo.path(), home.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a listed pack's hidden workflow with no `suppressed:` block must block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("planning") && stderr.contains("suppressed"),
        "stderr must name the offending `planning` workflow and the missing `suppressed:` declaration; got:\n{stderr}",
    );
}

/// The controls: an **unmutated** dev-pack copy, the bare **embedded** base,
/// and the composed **`[dev ▸ methodology]`** pair (the setup-written compose
/// marker) all load clean — the shipped front-matters conform, so the fence is
/// inert on every production composition.
#[test]
fn the_shipped_compositions_all_load_clean() {
    // The unmutated JIGC_PACK_DIR copy.
    let repo = TempDir::new("clean-repo");
    let home = TempDir::new("clean-home");
    let pack = pack_copy("clean", &embedded_pack_tree());
    init_repo(repo.path());
    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // The embedded base (no JIGC_PACK_DIR, no marker).
    let repo = TempDir::new("emb-repo");
    let home = TempDir::new("emb-home");
    init_repo(repo.path());
    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "the embedded base must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    // The composed [dev ▸ methodology] pair via the setup-written marker.
    let repo = TempDir::new("pair-repo");
    let home = TempDir::new("pair-home");
    init_repo(repo.path());
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
    let out = run_embedded(repo.path(), home.path(), START);
    assert!(
        out.status.success(),
        "the composed [dev ▸ methodology] pair must load clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The omitting context: a **manifest-less** pack is outside the pack-load
/// fences (the freeze-gate opt-in precedent) — the *same* stripped
/// `suppressed:` block that the manifest-bearing copy blocks on loads clean
/// once `config/schema-manifest.yaml` is dropped. Never an error: a seeded /
/// project-local pack that ships no manifest stays on skip-on-absent.
#[test]
fn a_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("nm-repo");
    let home = TempDir::new("nm-home");
    let pack = pack_copy("nomanifest", &embedded_pack_tree());
    init_repo(repo.path());
    strip_suppressed_block(pack.path(), "sub-task");
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the suppression fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// The `door` key (M52 Increment 9 / T1) — `suppressed:` gains an optional
// `door: <argv string>` naming the **real command line** a verb-routed workflow
// is reached through. The key is what D9's compose-door refusal renders as its
// `Mechanical` route, so a declared door that does not parse against the real
// CLI would compose a route an agent cannot run — the defect the fence below
// refuses at pack-load, naming the workflow and
// `workflow-refs.suppressed-malformed`
// (`design/surface-contract.md` → The suppression fence; settle-record §11).
// ---------------------------------------------------------------------------

/// The **declared** verb-routed set: the fourteen workflows the packs reach only
/// through a verb, each with the door it is reached through.
///
/// **Declared bound — this is a declaration, not a derivation.** The eight other
/// suppressed workflows (`router`, `ingest-existing`, `increment`, `planning`,
/// `completion`, `fix-task`, `record-change`, `record-dogfood`) are off the catalog
/// for reasons no machine-readable key discriminates from verb-routing, so **no
/// fence can prove a missing fifteenth member**. What this pins is the other
/// direction and the shape: no workflow gains a `door` without being named here,
/// and every named door is a command line the real CLI accepts.
const VERB_ROUTED_DOORS: &[(&str, &str)] = &[
    ("migrate-adr", "jigc migrate <path> --as adr"),
    ("migrate-arch-doc", "jigc migrate <path> --as arch-doc"),
    ("migrate-changelog", "jigc migrate <path> --as changelog"),
    ("migrate-prd", "jigc migrate <path> --as prd"),
    ("migrate-spec", "jigc migrate <path> --as spec"),
    (
        "migrate-completion-record",
        "jigc migrate <path> --as completion-record",
    ),
    (
        "migrate-decisions-log",
        "jigc migrate <path> --as decisions-log",
    ),
    (
        "migrate-deferral-ledger",
        "jigc migrate <path> --as deferral-ledger",
    ),
    ("migrate-idea", "jigc migrate <path> --as idea"),
    ("migrate-research", "jigc migrate <path> --as research"),
    ("migrate-roadmap", "jigc migrate <path> --as roadmap"),
    ("migrate-vision", "jigc migrate <path> --as vision"),
    ("sub-task", "jigc workflow sub-task --task <task-id>"),
    (
        "milestone-execution",
        "jigc milestone execute <milestone-id>",
    ),
];

/// Every shipped workflow of **both** embedded packs, read from the source trees
/// at runtime and parsed through the production loader — `(workflow id, def)`,
/// sorted by id.
fn shipped_workflow_defs() -> Vec<(String, engine::compose::WorkflowDef)> {
    let mut out = Vec::new();
    for tree in [embedded_pack_tree(), methodology_pack_tree()] {
        let dir = tree.join("workflows");
        for entry in fs::read_dir(&dir).expect("read the pack's workflows dir") {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                continue;
            }
            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .expect("workflow file stem")
                .to_owned();
            let bytes = fs::read(&path).expect("read the workflow");
            let def = engine::compose::load_workflow_def(&bytes)
                .unwrap_or_else(|f| panic!("`{id}` must load: {}", f.message));
            out.push((id, def));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// (a) The set of shipped workflows carrying a `door:` is **exactly** the fourteen
/// declared members, and each declares the door it is reached through.
#[test]
fn exactly_the_declared_verb_routed_workflows_carry_a_door() {
    let mut carrying: Vec<(String, String)> = shipped_workflow_defs()
        .into_iter()
        .filter_map(|(id, def)| {
            def.suppressed
                .and_then(|s| s.door)
                .map(|door| (id.clone(), door))
        })
        .collect();
    carrying.sort();

    let mut declared: Vec<(String, String)> = VERB_ROUTED_DOORS
        .iter()
        .map(|(id, door)| ((*id).to_owned(), (*door).to_owned()))
        .collect();
    declared.sort();

    assert_eq!(
        carrying, declared,
        "the shipped set carrying `suppressed.door` must be exactly the declared \
         verb-routed members, each with its own door",
    );
}

/// (b) Every declared door is a command line the **real CLI** accepts — asked as a
/// question through the shipped argv fence (`route_fence::accepts`), never by
/// constructing a `Route::mechanical`, whose own check is a debug-only panic.
#[test]
fn every_declared_door_is_accepted_by_the_cli_argv_fence() {
    for (id, def) in shipped_workflow_defs() {
        let Some(door) = def.suppressed.as_ref().and_then(|s| s.door.as_deref()) else {
            continue;
        };
        let argv = engine::compose::Suppressed::door_argv(door);
        assert!(
            cli::route_fence::accepts(&argv),
            "the `{id}` workflow's declared door `{door}` must parse against the real CLI",
        );
    }
}

/// (c) The fence: a `JIGC_PACK_DIR` copy whose `migrate-adr` declares a door that
/// does **not** parse (`jigc migrat …` — the typo'd verb) is blocked at pack-load,
/// naming the workflow and the code.
#[test]
fn a_non_parsing_door_is_blocked_at_pack_load() {
    let repo = TempDir::new("door-repo");
    let home = TempDir::new("door-home");
    let pack = pack_copy("door-strip", &embedded_pack_tree());
    init_repo(repo.path());
    retarget_door(pack.path(), "migrate-adr", "jigc migrat <path> --as adr");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a declared door that does not parse must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("migrate-adr") && stderr.contains("workflow-refs.suppressed-malformed"),
        "stderr must name the offending workflow and `workflow-refs.suppressed-malformed`; got:\n{stderr}",
    );
}

/// Rewrite the named workflow's `door:` line in a copied pack to `door`.
fn retarget_door(pack: &Path, workflow: &str, door: &str) {
    let path = pack.join("workflows").join(format!("{workflow}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied workflow");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if line.trim_start().starts_with("door:") {
            let indent = &line[..line.len() - line.trim_start().len()];
            out.push_str(&format!("{indent}door: {door}\n"));
            hit = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{workflow}` workflow must declare a `door:` to retarget"
    );
    fs::write(&path, out).expect("write the retargeted workflow");
}
