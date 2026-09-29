//! T1 — the methodology pack's cascade-config substrate, proven through the real
//! binary on a throwaway repo (M12 Inc 1; `design/self-hosting.md` → Settled forks
//! → Subsume; the knob-enum build note).
//!
//! The substrate this task ships is *pure pack data* — `config/{defaults,knobs,
//! commands}.yaml` + the vendored `schemas/commit.yaml` under `packs/methodology/`,
//! with **zero `crates/*/src` changes**. At T1 the `dev-task` workflow does **not**
//! yet exist (it lands in T2), so this file does **not** assert `start "<intent>"`
//! composes. It asserts the two facts the substrate must already carry:
//!
//!   (a) **It loads as the sole pack.** `JIGC_PACK_DIR=<methodology-pack> jigc setup`
//!       honors the env (the adapter install), then a **bare** `jigc start` (no
//!       intent — the orientation read-path, which reads only `config/defaults` +
//!       `config/knobs`, never the catalog's default-workflow) exits 0 and renders
//!       the `Pack: methodology/<version>` provenance label. The pack names itself
//!       through its own cascade config.
//!
//!   (b) **The enum-rewrite is load-bearing — the trap is proven, not assumed.** A
//!       same-shape pack whose `default-workflow` enum + default are left at the
//!       dev-pack's verbatim `router` (the copy-without-edit failure mode) drives a
//!       `start "<intent>"` compose that FAILS with `no workflow` + `router` — the
//!       knob enum constrains the `defaults.yaml` value, so a verbatim copy points
//!       the front door at a workflow no methodology pack ships. The rewritten
//!       `[dev-task]` enum is what makes (a)'s pack correct.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, and a
//! self-cleaning `TempDir` keeps the test off the developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-methodology-substrate-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — `CARGO_MANIFEST_DIR`
/// is `<root>/crates/cli`, so the pack tree is two parents up plus `packs/methodology`.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read source tree").flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Initialize a real git repo with one commit (composition mints, which reads
/// HEAD). `jigc setup` itself creates the `.jigc/config/` project layer.
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
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, and a selected
/// `JIGC_PACK_DIR` (the sole-pack seam under test).
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn methodology_pack_loads_as_sole_pack_and_orients_with_its_provenance_label() {
    // (a) `JIGC_PACK_DIR=<methodology-pack> jigc setup` honors the env, then bare
    // `jigc start` (no intent) renders `Pack: methodology/<version>` — the pack
    // loads as the sole composed pack and names itself through its cascade config.
    let repo = TempDir::new("orient");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    let out = run_jigc(repo.path(), home.path(), &pack, &["start"]);
    assert!(
        out.status.success(),
        "bare `jigc start` over the methodology pack must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let label = stdout
        .lines()
        .find(|l| l.starts_with("Pack: "))
        .unwrap_or_else(|| panic!("orientation must render a `Pack: ` label; got:\n{stdout}"))
        .split(" · ")
        .next()
        .expect("the `Pack: ` segment");
    assert_eq!(
        label, "Pack: methodology/0.1.0",
        "the methodology pack's defaults.yaml (`pack-id: methodology`, `version: 0.1.0`) must \
         surface as the provenance label; got:\n{stdout}",
    );
}

#[test]
fn verbatim_router_enum_fails_no_workflow_router_the_trap_proven() {
    // (b) The trap: a methodology-shaped pack whose `default-workflow` enum +
    // default are left at the dev-pack's verbatim `router` points the front door at
    // a workflow the pack does not ship. A `start "<intent>"` compose must FAIL with
    // `no workflow` + `router` — proving the enum-rewrite to `[dev-task]` is
    // load-bearing, not assumed.
    let repo = TempDir::new("trap");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A faithful copy of the methodology pack with ONLY the default-workflow knob
    // reverted to the dev-pack's verbatim `router` (enum + default) — the
    // copy-without-edit failure mode.
    let trap_pack = TempDir::new("router-enum");
    copy_tree(&methodology_pack_tree(), trap_pack.path());
    let knobs = trap_pack.path().join("config").join("knobs.yaml");
    let knobs_text = fs::read_to_string(&knobs).expect("read copied knobs.yaml");
    let reverted = knobs_text.replace(
        "default-workflow:\n  type: enum\n  of: [dev-task]\n  default: dev-task",
        "default-workflow:\n  type: enum\n  of: [router, single-task, quick-fix, plan, \
         implement-from-spec]\n  default: router",
    );
    assert_ne!(
        reverted, knobs_text,
        "the trap fixture must actually revert the default-workflow enum — the methodology \
         knobs.yaml must declare `of: [dev-task]` / `default: dev-task` for the replace to bite",
    );
    fs::write(&knobs, reverted).expect("write reverted knobs.yaml");
    let defaults = trap_pack.path().join("config").join("defaults.yaml");
    let defaults_text = fs::read_to_string(&defaults).expect("read copied defaults.yaml");
    fs::write(
        &defaults,
        defaults_text.replace("default-workflow: dev-task", "default-workflow: router"),
    )
    .expect("write reverted defaults.yaml");

    let setup = run_jigc(repo.path(), home.path(), trap_pack.path(), &["setup"]);
    assert!(setup.status.success(), "`jigc setup` must exit 0");

    let out = run_jigc(
        repo.path(),
        home.path(),
        trap_pack.path(),
        &["start", "add rate limiter"],
    );
    assert!(
        !out.status.success(),
        "a `start \"<intent>\"` compose over the verbatim-router pack must FAIL; got success",
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        combined.contains("no workflow") && combined.contains("router"),
        "the verbatim-router compose must fail naming `no workflow ... router` (the enum \
         constrains the defaults value, so the front door composes the ungifted `router`); \
         got:\n{combined}",
    );
}
