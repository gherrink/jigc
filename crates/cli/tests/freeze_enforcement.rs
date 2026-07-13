//! Real-binary acceptance for the M33 **pack-load freeze gate** (T3): a
//! `JIGC_PACK_DIR` on-disk dev-pack copy whose schema *shape* drifted from the
//! shipped `config/schema-manifest.yaml` without a version bump makes a `jigc`
//! command exit **non-zero**, naming the schema-hash mismatch; an unmutated copy
//! composes clean; a manifest-less copy is unaffected (the same drift passes once
//! the freeze manifest is dropped).
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract, not a reconstructed equivalent. It is the runtime
//! sibling of the build-time freeze gate
//! (`pack::shipped_schema_manifest_matches_the_frozen_doctype_set`); the design is
//! `design/corpus-migration.md` → The enforcement gate fires at pack-load
//! (review Finding 3) + `design/worked-examples.md` → the freeze-enforcement flow.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-freeze-{tag}-{}-{:?}",
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

/// The embedded dev pack tree — the faithful source the on-disk copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the faithful
/// source the methodology-arm copies mirror.
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

/// Copy the dev pack into a fresh temp dir and return the copy's root.
fn dev_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `adr` schema's **shape** (its `location:`) — a hash-affecting
/// change that bumps no `schema-version`, the un-migrated schema change the freeze
/// forbids. The manifest is deliberately left unbumped.
fn drift_adr_schema(pack: &Path) {
    let schema = pack.join("schemas").join("adr.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied adr.yaml");
    let drifted = body.replacen("location: decisions/", "location: adr-records/", 1);
    assert_ne!(
        body, drifted,
        "adr.yaml must declare `location: decisions/`"
    );
    fs::write(&schema, drifted).expect("write the drifted adr.yaml");
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

/// Run `jigc start --workflow single-task "<intent>"` with `cwd = repo`,
/// `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_start(repo: &Path, home: &Path, pack: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", "freeze-gate probe"])
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("spawn the jigc binary")
}

/// A schema-shape change with no manifest bump is **blocked at pack-load**: the
/// composing `jigc start` exits non-zero, and stderr names the schema-hash
/// mismatch on the drifted doctype.
#[test]
fn schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("drift");
    init_repo(repo.path());
    drift_adr_schema(pack.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("adr"),
        "stderr must name the drifted `adr` doctype; got:\n{stderr}",
    );
}

/// The control: an **unmutated** dev-pack copy matches its shipped manifest, so the
/// freeze gate is inert and `jigc start` composes clean.
#[test]
fn unmutated_pack_copy_composes_clean() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("clean");
    init_repo(repo.path());

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "an unmutated dev-pack copy must compose clean; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Copy the methodology pack into a fresh temp dir and return the copy's root.
fn methodology_pack_copy(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&methodology_pack_tree(), dir.path());
    dir
}

/// Mutate the copied `research` schema's **shape** (a slot hint) — a hash-affecting
/// change that bumps no `schema-version`. The methodology manifest is deliberately
/// left unbumped.
fn drift_research_schema(pack: &Path) {
    let schema = pack.join("schemas").join("research.yaml");
    let body = fs::read_to_string(&schema).expect("read the copied research.yaml");
    let drifted = body.replacen(
        "The question this research set out to answer.",
        "A drifted question hint.",
        1,
    );
    assert_ne!(body, drifted, "research.yaml must carry the question hint");
    fs::write(&schema, drifted).expect("write the drifted research.yaml");
}

/// Compose `[dev ▸ methodology-copy]` via the listed-pack mechanism: `packs.yaml`
/// names the on-disk copy over the embedded dev base. `JIGC_PACK_DIR` must stay
/// unset so the embedded base (not an env pack) anchors the composition.
fn list_pack(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the methodology copy");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and NO `JIGC_PACK_DIR`
/// (the listed-pack composition path — the pack rides in `packs.yaml`).
fn run_listed(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("spawn the jigc binary")
}

/// The M40 A1 headline: a **methodology** schema-shape change with no manifest bump
/// is **blocked at pack-load** — the methodology pack now ships its own
/// `config/schema-manifest.yaml`, so it is freeze-enforced exactly like the dev
/// pack (`design/corpus-migration.md` → M40 revises the dichotomy). The composing
/// `jigc start` exits non-zero and stderr names the schema-hash mismatch on the
/// drifted `research` doctype.
#[test]
fn methodology_schema_shape_drift_without_manifest_bump_is_blocked() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = methodology_pack_copy("m-drift");
    init_repo(repo.path());
    list_pack(repo.path(), pack.path());
    drift_research_schema(pack.path());

    let out = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "freeze-gate probe"],
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a drifted methodology schema must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("schema-hash mismatch"),
        "stderr must name the schema-hash mismatch; got:\n{stderr}",
    );
    assert!(
        stderr.contains("research"),
        "stderr must name the drifted `research` doctype; got:\n{stderr}",
    );
}

/// The stamp goes live with the manifest (one atomic unit): a **fresh methodology
/// mint** through the real `do-research` workflow carries `schema-version: 1` in
/// its committed front matter — the value the validate side's version-aware
/// routing demands (`design/corpus-migration.md` → The schema-version stamp).
#[test]
fn fresh_methodology_mint_carries_schema_version_1() {
    let repo = TempDir::new("mint-repo");
    let home = TempDir::new("mint-home");
    init_repo(repo.path());
    list_pack(repo.path(), &methodology_pack_tree());

    // Mint a research doc through the real workflow: start → create → author →
    // fill the commit doc → finalize.
    let start = run_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "do-research", "study the cache"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow do-research` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );
    let run = |args: &[&str]| {
        let out = run_listed(repo.path(), home.path(), args);
        assert!(
            out.status.success(),
            "jigc {args:?} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        use std::io::Write;
        let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(["doc", "set-slot", addr, "--from-file", "-"])
            .current_dir(repo.path())
            .env("HOME", home.path())
            .env_remove("JIGC_PACK_DIR")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("spawn jigc set-slot");
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(prose)
            .expect("write stdin");
        let out = child.wait_with_output().expect("wait for jigc");
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    };
    run(&["doc", "create", "research", "--title", "Cache Study"]);
    for slot in ["question", "findings", "sources"] {
        set_slot(&format!("research:cache-study#{slot}"), b"Some prose.\n");
    }
    let task = "study-the-cache";
    run(&[
        "doc",
        "set-field",
        &format!("commit:{task}#type"),
        "--value",
        "docs",
    ]);
    set_slot(&format!("commit:{task}#summary"), b"record the study\n");
    run(&["task", "finalize", task]);

    let committed = fs::read_to_string(repo.path().join("research").join("cache-study.md"))
        .expect("the finalized research doc is committed at research/cache-study.md");
    assert!(
        committed.contains("schema-version: 1"),
        "a fresh methodology mint must carry `schema-version: 1` in its front matter; got:\n{committed}",
    );
}

/// Compose `[drifted-dev-copy ▸ methodology ▸ embedded-dev]` via the listed-pack
/// mechanism — the production shape a real project uses (`packs.yaml` names pack
/// **directories**, the trigger that discharged the M33 deferral). The methodology
/// pack rides along so the `milestone-record` doctype resolves and `jigc milestone
/// create` reaches its committing write path.
fn list_packs(repo: &Path, packs: &[&Path]) {
    let mut body = String::from("packs:\n");
    for pack in packs {
        body.push_str(&format!("  - {}\n", pack.display()));
    }
    fs::write(repo.join(".jigc").join("config").join("packs.yaml"), body)
        .expect("write packs.yaml naming the listed packs");
}

/// A repo + home + listed **drifted** dev-pack copy (with the methodology pack
/// composed for the `milestone-record` doctype). Every door run against it must
/// block.
struct DriftedProject {
    repo: TempDir,
    home: TempDir,
    _pack: TempDir,
}

impl DriftedProject {
    fn new(tag: &str) -> Self {
        let repo = TempDir::new(&format!("{tag}-repo"));
        let home = TempDir::new(&format!("{tag}-home"));
        let pack = dev_pack_copy(&format!("{tag}-pack"));
        init_repo(repo.path());
        list_packs(repo.path(), &[pack.path(), &methodology_pack_tree()]);
        drift_adr_schema(pack.path());
        DriftedProject {
            repo,
            home,
            _pack: pack,
        }
    }

    /// Run one door and assert it exits non-zero naming the drifted doctype's
    /// schema-hash mismatch — the emitted exit code + stderr are the contract.
    fn door_blocks(&self, args: &[&str]) -> std::process::Output {
        let out = run_listed(self.repo.path(), self.home.path(), args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            !out.status.success(),
            "`jigc {}` must exit non-zero on a drifted frozen schema; stdout:\n{}\nstderr:\n{stderr}",
            args.join(" "),
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains("schema-hash mismatch"),
            "`jigc {}` stderr must name the schema-hash mismatch; got:\n{stderr}",
            args.join(" "),
        );
        assert!(
            stderr.contains("adr"),
            "`jigc {}` stderr must name the drifted `adr` doctype; got:\n{stderr}",
            args.join(" "),
        );
        out
    }
}

/// The M42 Inc 6 headline: the freeze assert lives in the **pack-source factory**, so
/// a drifted frozen schema blocks **every** door — not just the compose front door it
/// used to guard. The read/report verbs sailed past it before this task
/// (`implementation/decisions-pending.md` → the discharged M33 deferral: the trigger
/// "packs are ever loaded from the filesystem in production" fired at M14).
#[test]
fn every_read_door_blocks_on_a_drifted_frozen_schema() {
    let project = DriftedProject::new("doors");

    project.door_blocks(&["validate"]);
    project.door_blocks(&["describe"]);
    project.door_blocks(&["doc", "schema", "adr"]);
    project.door_blocks(&["migrate-corpus"]);
    // The orient front door (bare `jigc start` — no workflow, no intent): a sixth
    // door, distinct from the composing `jigc start --workflow …` the assert already
    // guarded.
    project.door_blocks(&["start"]);
}

/// The **committing** door: `jigc milestone create` wrote and committed a brand-new
/// milestone record at exit 0 over a drifted frozen schema. It must block — and land
/// **no** record file and **no** record commit (the write is what makes this door the
/// dangerous one; Inc 7 builds a second write verb on it).
#[test]
fn the_committing_milestone_door_blocks_and_writes_nothing() {
    let project = DriftedProject::new("mcreate");

    project.door_blocks(&["milestone", "create", "drift probe"]);

    let records = project.repo.path().join("docs").join("milestone-records");
    let landed: Vec<PathBuf> = fs::read_dir(&records)
        .map(|dir| {
            dir.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
                .collect()
        })
        .unwrap_or_default();
    assert!(
        landed.is_empty(),
        "a blocked `milestone create` must land no milestone-record file; found: {landed:?}",
    );

    let log = Command::new("git")
        .args(["log", "--oneline"])
        .current_dir(project.repo.path())
        .output()
        .expect("run git log");
    let log = String::from_utf8_lossy(&log.stdout);
    assert!(
        !log.contains("chore(milestone): open record"),
        "a blocked `milestone create` must land no record commit; git log:\n{log}",
    );
}

/// The omitting context: a **manifest-less** pack is unchecked. The *same* schema
/// drift that the manifest-bearing copy blocks composes clean once
/// `config/schema-manifest.yaml` is dropped — proving the manifest is the gate, and
/// that a seeded / composed pack with no manifest stays inert (never errors).
#[test]
fn manifest_less_pack_is_unaffected() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack_copy("nomanifest");
    init_repo(repo.path());
    drift_adr_schema(pack.path());
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_start(repo.path(), home.path(), pack.path());
    assert!(
        out.status.success(),
        "a manifest-less pack must be unaffected by the freeze gate; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
