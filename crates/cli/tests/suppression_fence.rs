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

use crate::support::root_walk;
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
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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
    // F-10. Its door binds what the step reads the same way every `migrate-*` door does —
    // the commit at HEAD, pinned at the mint — so a router pick would compose over no
    // pinned commit and its finalize would have nothing to rewrite.
    ("amend", "jigc task amend"),
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
        for path in root_walk::files_in(&tree.join("workflows"), root_walk::ext("yaml")) {
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

// ---------------------------------------------------------------------------
// The door-shape **axis** (M52 Increment 9, the validate→fix loop's B2).
//
// The parse half above shipped with its subject set to the manifest-shipping
// origin packs — `origin_packs(Config, schema-manifest)`, the scope every
// sibling pack-load fence uses. That scope is right for a fence over what a
// pack *states* and wrong for one over a *door*: a declared door is the argv
// the compose-door refusal renders as its `Mechanical` route, whose own check
// is a **debug-only panic**. So two layers that serve workflow definitions were
// never parse-checked, and both were driven at `dbb35b8c` before this axis was
// written — `jigc start --workflow <id>` panicked at exit **101** (release:
// emitted a route that cannot run) for a **project-layer** shadow and for a
// **manifest-less listed pack** alike.
//
// The axis below is **manufactured, and says so**: *which layer served the
// definition* is a property of how a composition is assembled, not a set any
// code-side registry holds — `origin_packs` enumerates the constituents of one
// assembly and the project layer is not a pack at all.
// ---------------------------------------------------------------------------

/// The layers a workflow definition can be served from — the axis the door-shape
/// fence's subject must cover, and the reason the fence's scope is wider than
/// every other pack-load fence's.
#[derive(Clone, Copy, Debug)]
enum DoorLayer {
    /// A `JIGC_PACK_DIR` copy of the dev pack: ships `config/schema-manifest.yaml`,
    /// so it is the **one** layer the pre-fix subject reached.
    ManifestShippingPack,
    /// A `packs.yaml`-listed pack shipping **no** manifest — outside every
    /// statement-shaped pack-load fence by the `assert_schema_freeze` opt-in
    /// precedent, and inside this one because its door is still served.
    ManifestLessListedPack,
    /// `.jigc/config/workflows/<id>.yaml` — the whole-file definition shadow that
    /// **outranks every pack** (`design/overrides.md` → Authored metadata on a
    /// definition resolves by whole-file shadow).
    ProjectShadow,
}

impl DoorLayer {
    /// The two layers the pre-fix subject **missed** lead, deliberately: a red here
    /// should name the cell that panicked the binary, not the one that merely failed
    /// to say which layer it was refusing.
    const ALL: &'static [DoorLayer] = &[
        DoorLayer::ProjectShadow,
        DoorLayer::ManifestLessListedPack,
        DoorLayer::ManifestShippingPack,
    ];

    /// The token the refusal must carry to name **which layer** declared the door.
    fn named_in_refusal(self) -> &'static str {
        match self {
            // The dev pack copy's own `config/defaults` pack-id.
            DoorLayer::ManifestShippingPack => "pack `dev`",
            DoorLayer::ManifestLessListedPack => "pack `probe-listed`",
            DoorLayer::ProjectShadow => ".jigc/config/workflows/",
        }
    }

    /// The workflow whose `door:` this layer carries.
    fn workflow(self) -> &'static str {
        match self {
            // The shipped verb-routed member, retargeted in the copy.
            DoorLayer::ManifestShippingPack => "migrate-adr",
            DoorLayer::ManifestLessListedPack | DoorLayer::ProjectShadow => "probe-door",
        }
    }
}

/// Three malformed doors, one per way a door can fail the **parse** half while
/// passing the engine's lexical half (which sees only `jigc`-leading + shell-inert
/// tokens): an unknown verb, a `<placeholder>` outside the declared substitution
/// table, and a bare `jigc` naming no subcommand at all.
const MALFORMED_DOORS: &[(&str, &str)] = &[
    ("unknown-verb", "jigc migraaate <path> --as adr"),
    ("undeclared-placeholder", "jigc migrate <päth> --as adr"),
    ("no-subcommand", "jigc"),
];

/// The well-formed control — a door the real CLI accepts.
const WELL_FORMED_DOOR: &str = "jigc migrate <path> --as adr";

/// A workflow definition whose `suppressed:` block declares `door`.
fn door_workflow_source(door: &str) -> String {
    format!(
        "---\n\
         when: probe one layer's declared door\n\
         description: A manufactured verb-routed workflow for the door-shape axis.\n\
         usage: probe.\n\
         creates-task: true\n\
         selectable: false\n\
         suppressed:\n  \
           reason: verb-routed — reached only through its own verb\n  \
           expires: never\n  \
           door: {door}\n\
         ---\n\
         {{{{ include: step:locate }}}}\n"
    )
}

/// Stand one layer up carrying `door` and run a composing `jigc start` against it.
/// The two `TempDir`s live until the child has exited, which is all the assertions
/// read.
fn drive_door_cell(layer: DoorLayer, door: &str) -> std::process::Output {
    let repo = TempDir::new("axis-repo");
    let home = TempDir::new("axis-home");
    init_repo(repo.path());
    match layer {
        DoorLayer::ManifestShippingPack => {
            let pack = pack_copy("axis-pack", &embedded_pack_tree());
            retarget_door(pack.path(), layer.workflow(), door);
            run_with_pack(repo.path(), home.path(), pack.path(), START)
        }
        DoorLayer::ManifestLessListedPack => {
            let listed = TempDir::new("axis-listed");
            fs::create_dir_all(listed.path().join("workflows")).expect("create listed workflows");
            fs::create_dir_all(listed.path().join("config")).expect("create listed config");
            fs::write(
                listed.path().join("config").join("defaults.yaml"),
                "pack-id: probe-listed\n",
            )
            .expect("write the listed pack id");
            fs::write(
                listed
                    .path()
                    .join("workflows")
                    .join(format!("{}.yaml", layer.workflow())),
                door_workflow_source(door),
            )
            .expect("write the listed workflow");
            fs::write(
                repo.path().join(".jigc").join("config").join("packs.yaml"),
                format!("packs:\n  - {}\n", listed.path().display()),
            )
            .expect("write packs.yaml");
            run_embedded(repo.path(), home.path(), START)
        }
        DoorLayer::ProjectShadow => {
            let dir = repo.path().join(".jigc").join("config").join("workflows");
            fs::create_dir_all(&dir).expect("create the project workflows dir");
            fs::write(
                dir.join(format!("{}.yaml", layer.workflow())),
                door_workflow_source(door),
            )
            .expect("write the project workflow shadow");
            run_embedded(repo.path(), home.path(), START)
        }
    }
}

/// The axis: **every** layer × **every** malformed door is refused at pack-load,
/// naming the workflow, the code and the layer — and refused as a *finding*, never
/// as the `Route::mechanical` construction panic the pre-fix project-layer and
/// listed-pack cells exited 101 with.
#[test]
fn every_layer_that_serves_a_workflow_has_its_declared_door_parse_checked() {
    for &layer in DoorLayer::ALL {
        for (tag, door) in MALFORMED_DOORS {
            let out = drive_door_cell(layer, door);
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert!(
                !out.status.success(),
                "{layer:?} / {tag}: a door that does not parse must refuse; stdout:\n{}\nstderr:\n{stderr}",
                String::from_utf8_lossy(&out.stdout),
            );
            assert!(
                !stderr.contains("panicked") && out.status.code() != Some(101),
                "{layer:?} / {tag}: the refusal must be a finding, never a construction panic; \
                 exit {:?}; stderr:\n{stderr}",
                out.status.code(),
            );
            assert!(
                stderr.contains(layer.workflow())
                    && stderr.contains("workflow-refs.suppressed-malformed")
                    && stderr.contains(layer.named_in_refusal()),
                "{layer:?} / {tag}: the refusal must name the workflow, the code and the layer \
                 (`{}`); stderr:\n{stderr}",
                layer.named_in_refusal(),
            );
        }
    }
}

/// The control, one per layer: a **well-formed** door on the same layer loads clean
/// — the fence refuses unparseable doors, not declared ones.
#[test]
fn a_well_formed_door_loads_clean_on_every_layer() {
    for &layer in DoorLayer::ALL {
        let out = drive_door_cell(layer, WELL_FORMED_DOOR);
        assert!(
            out.status.success(),
            "{layer:?}: a well-formed door must load clean; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// The finding's own repro, kept as its own cell because the axis above never
/// composes the workflow whose door is malformed: a project shadow **of a shipped
/// id** (`single-task`) declaring an unparseable door, named to the compose door
/// that would render it as a `Mechanical` route. At `dbb35b8c` this exited **101**
/// with `a `Route::mechanical` argv must parse against the real CLI` — the fence
/// having never looked at this layer. It must refuse as a finding instead, in the
/// debug posture this suite runs in and in release alike.
#[test]
fn a_project_shadow_of_a_shipped_workflow_refuses_its_unparseable_door_rather_than_panicking() {
    let repo = TempDir::new("repro-repo");
    let home = TempDir::new("repro-home");
    init_repo(repo.path());
    let dir = repo.path().join(".jigc").join("config").join("workflows");
    fs::create_dir_all(&dir).expect("create the project workflows dir");
    fs::write(
        dir.join("single-task.yaml"),
        door_workflow_source("jigc migraaate <path> --as adr"),
    )
    .expect("write the project shadow");

    let out = run_embedded(repo.path(), home.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(1),
        "the composed door must refuse at exit 1, never panic (101); stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        !stderr.contains("panicked"),
        "the refusal must not be a `Route::mechanical` construction panic; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("single-task")
            && stderr.contains("workflow-refs.suppressed-malformed")
            && stderr.contains(".jigc/config/workflows/"),
        "the refusal must name the workflow, the code and the project layer; stderr:\n{stderr}",
    );
}
