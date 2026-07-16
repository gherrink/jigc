//! Real-binary acceptance for the M43 **stated-at fence** (law 3, structural
//! tier — `design/surface-contract.md` → The stated-at fence; DECISIONS.md →
//! M43 Settle #6): every member of the code-side **ambush-class const**
//! (`finalize.promote-clobber` · the staging pair `finalize.left-out` +
//! `finalize.nothing-staged` · `finalize.carried-staged`) must have at least
//! one declarer among a manifest-shipping pack's steps'
//! `states-constraints:` front-matter — the soliciting step carries the
//! contract's statement, so the constraint is stated where it binds instead
//! of first appearing in its block message (an ambush).
//!
//! A shipped-tree `JIGC_PACK_DIR` / listed-pack copy with one declarer
//! stripped exits **non-zero at pack-load naming the undeclared code**; the
//! unmutated copies, the embedded base, and the composed
//! `[dev ▸ methodology]` pair all load clean (each shipped pack loaded
//! **alone** carries every declarer — the methodology-alone dogfood path); a
//! **manifest-less** pack is unchecked (the `assert_schema_freeze` opt-in
//! precedent). Honest bound: the fence proves presence-of-obligation, never
//! prose quality.
//!
//! This drives the **real `jigc` binary** end-to-end — the emitted exit code +
//! stderr are the contract (the `freeze_enforcement.rs` /
//! `suppression_fence.rs` pattern).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-stated-at-{tag}-{}-{:?}",
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

/// Strip the named step's `states-constraints:` declaration from a copied pack
/// (the flow-style line becomes the empty list) — the mutation the fence must
/// catch: the contract's declarer withdrawn while the ambush-class code still
/// exists in the binary.
fn strip_states_constraints(pack: &Path, step: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied step");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("states-constraints:") {
            out.push_str("states-constraints: []\n");
            hit = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{step}` step must carry a `states-constraints:` declaration to strip"
    );
    fs::write(&path, out).expect("write the stripped step");
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
    "stated-at fence probe",
];

/// The fence: a dev-pack copy whose `finalize` step withdrew its
/// `states-constraints:` declaration (the staging pair + the carried-staged
/// contract) is **blocked at pack-load** — the composing `jigc start` exits
/// non-zero, and stderr names an undeclared ambush-class code.
#[test]
fn a_stripped_staging_declarer_is_blocked_at_pack_load() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = pack_copy("strip", &embedded_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "finalize");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "an ambush-class code with no declarer must make `jigc start` exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("finalize.carried-staged"),
        "stderr must name the undeclared `finalize.carried-staged` code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("states-constraints"),
        "stderr must name the missing `states-constraints:` declaration; got:\n{stderr}",
    );
}

/// Per-constituent isolation: a *listed* methodology-pack copy whose
/// `migration-finalize` step withdrew the `finalize.promote-clobber` declarer
/// blocks at pack-load too — even though the dev pack's own declarer is intact
/// in the same composition, each manifest-shipping pack must carry every
/// declarer itself (the methodology-alone dogfood path).
#[test]
fn a_stripped_clobber_declarer_blocks_through_the_listed_pack_path() {
    let repo = TempDir::new("m-repo");
    let home = TempDir::new("m-home");
    let pack = pack_copy("m-strip", &methodology_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "migration-finalize");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.path().display()),
    )
    .expect("write packs.yaml naming the methodology copy");

    let out = run_embedded(repo.path(), home.path(), START);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a listed pack missing the `finalize.promote-clobber` declarer must block; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("finalize.promote-clobber"),
        "stderr must name the undeclared `finalize.promote-clobber` code; got:\n{stderr}",
    );
}

/// The controls: an **unmutated** dev-pack copy, the bare **embedded** base,
/// and the composed **`[dev ▸ methodology]`** pair all load clean — both
/// shipped packs carry a declarer for all four ambush-class members, so the
/// fence is inert on every production composition.
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
/// fences (the freeze-gate opt-in precedent) — the *same* stripped declarer
/// that the manifest-bearing copy blocks on loads clean once
/// `config/schema-manifest.yaml` is dropped. Never an error: a seeded /
/// project-local pack that ships no manifest stays on skip-on-absent.
#[test]
fn a_manifest_less_pack_is_unchecked() {
    let repo = TempDir::new("nm-repo");
    let home = TempDir::new("nm-home");
    let pack = pack_copy("nomanifest", &embedded_pack_tree());
    init_repo(repo.path());
    strip_states_constraints(pack.path(), "finalize");
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the freeze manifest");

    let out = run_with_pack(repo.path(), home.path(), pack.path(), START);
    assert!(
        out.status.success(),
        "a manifest-less pack must be outside the stated-at fence; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}
