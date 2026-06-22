//! End-to-end determinism guard for the `JIGC_PACK_DIR` pack-source seam
//! (M5 Increment 3, T3 — `implementation/roadmap.md` → Increment 3 →
//! Determinism guard; `design/overrides.md` → the `FilesystemPack` seam).
//!
//! Drives the built `jigc` binary and asserts the three seam invariants:
//!   (a) with `JIGC_PACK_DIR` **unset**, a bare `jigc start "<intent>"` compose is
//!       **byte-identical** to the committed golden — the determinism boundary: the
//!       seam is inert when the env is not set, so the embedded pack drives output.
//!   (b) `JIGC_PACK_DIR=<copy-of-embedded-pack>` loads the directory pack via the
//!       factory and composes **byte-identically** to the no-env run — a genuine
//!       alternate `PackSource` drives the binary with zero behavior change.
//!   (c) two pack dirs whose `config/defaults.yaml` declare **distinct** `version:`
//!       report distinct `pack_version()`s through the binary, visible in the
//!       `Pack: <id>/<version>` provenance label that orientation renders.
//!
//! The faithful directory pack is a recursive copy of the embedded pack tree at
//! `crates/cli/pack/` (the very tree `include_dir!` embeds), so the FilesystemPack
//! reads byte-identical resource bytes. No external test crates: the binary path
//! comes from `CARGO_BIN_EXE_jigc`, the pack source from `CARGO_MANIFEST_DIR`, and
//! a self-cleaning `TempDir` keeps the test off the developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-pack-source-{tag}-{}-{:?}",
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

/// The **no-override** bare-`jigc start "<intent>"` agent-text composition, byte
/// for byte — the cascade default (`router`) resolved with no project delta. The
/// router lists the selectable work-workflows and carries no `{{task.intent}}`, so
/// the golden is intent-stable and path-free (compose emits no provenance header).
/// This is the no-env determinism golden the `JIGC_PACK_DIR` seam must not perturb.
const NO_OVERRIDE_ROUTER_GOLDEN: &str = "\
These are the selectable work-workflows, each with the situation it fits:

- architecture-documentation — document the architecture of a part of the system, tying its components to the code that implements them
- implement-from-spec — build from a committed spec whose acceptance criteria already exist
- plan — draft the specification for upcoming work before writing any code
- project-setup — bootstrap a brand-new project by developing the idea into its first product requirements
- quick-fix — apply a small commit-only fix that touches no documented code and records no decision
- single-task — implement one scoped change end-to-end

Pick the workflow whose situation best fits the intent, then re-run with that
choice and the original intent:

jigc start --workflow <chosen> \"<intent>\"
— jigc · run `jigc start` for orientation; all writes through `jigc`.
";

/// The embedded pack source tree (`crates/cli/pack/`) — `CARGO_MANIFEST_DIR` is
/// `<root>/crates/cli`, the very tree `include_dir!` embeds into the binary.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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

/// Copy the embedded pack tree into `dir` and overwrite `config/defaults.yaml`'s
/// `version:` key with `version` (the `FilesystemPack` reads it through
/// `pack_version`). `pack-id` is left untouched, so the provenance label stays
/// `Pack: dev/<version>`.
fn dir_pack_with_version(dir: &Path, version: &str) {
    copy_tree(&embedded_pack_tree(), dir);
    let defaults = dir.join("config").join("defaults.yaml");
    let existing = fs::read_to_string(&defaults).expect("read copied defaults.yaml");
    fs::write(&defaults, format!("{existing}version: {version}\n"))
        .expect("write versioned defaults.yaml");
}

/// Initialize a real git repo with one commit (composition mints, which reads
/// HEAD) plus the `.jigc/config/` project layer so the cascade resolves.
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

/// Run `jigc start <args>` with `cwd = repo`, `$HOME = home`, and an optional
/// `JIGC_PACK_DIR` (the seam under test).
fn run_start(
    repo: &Path,
    home: &Path,
    pack_dir: Option<&Path>,
    args: &[&str],
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("start");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    // Always clear an inherited value first, then set it only when the test
    // selects the directory pack — so the no-env case is genuinely env-unset.
    command.env_remove("JIGC_PACK_DIR");
    if let Some(dir) = pack_dir {
        command.env("JIGC_PACK_DIR", dir);
    }
    command.output().expect("run the jigc binary")
}

#[test]
fn no_env_bare_intent_compose_is_byte_identical_to_the_golden() {
    // (a) The determinism boundary: with `JIGC_PACK_DIR` unset, the seam is inert —
    // the embedded pack drives compose, byte-identical to the committed golden.
    let repo = TempDir::new("no-env");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_start(repo.path(), home.path(), None, &["add rate limiter"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the no-env bare compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_OVERRIDE_ROUTER_GOLDEN,
        "with JIGC_PACK_DIR unset the compose output must stay byte-identical to today's golden",
    );
}

#[test]
fn jigc_pack_dir_loads_the_directory_pack_and_composes_identically() {
    // (b) A genuine alternate `PackSource`: `JIGC_PACK_DIR=<faithful copy of the
    // embedded tree>` is selected by the factory and composes byte-for-byte the same
    // as the no-env run — zero behavior change from routing through FilesystemPack.
    let repo = TempDir::new("env-loaded");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let pack_dir = TempDir::new("pack-copy");
    copy_tree(&embedded_pack_tree(), pack_dir.path());

    let out = run_start(
        repo.path(),
        home.path(),
        Some(pack_dir.path()),
        &["add rate limiter"],
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        out.status.success(),
        "the JIGC_PACK_DIR compose must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        stdout, NO_OVERRIDE_ROUTER_GOLDEN,
        "the directory pack (a faithful copy of the embedded tree) must compose byte-identically \
         to the no-env run — the FilesystemPack reads the same resource bytes",
    );
}

#[test]
fn two_pack_dirs_report_distinct_versions_in_the_provenance_label() {
    // (c) `pack_version()` flows from `config/defaults.yaml`'s `version:` key through
    // the binary into the `Pack: <id>/<version>` orientation provenance label: two
    // dirs with distinct versions surface distinct labels.
    let repo = TempDir::new("versions");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let pack_v1 = TempDir::new("pack-v1");
    dir_pack_with_version(pack_v1.path(), "0.3.0");
    let pack_v2 = TempDir::new("pack-v2");
    dir_pack_with_version(pack_v2.path(), "0.4.0");

    let label = |pack_dir: &Path| -> String {
        // Bare `jigc start` (no intent) renders the orientation provenance header.
        let out = run_start(repo.path(), home.path(), Some(pack_dir), &[]);
        assert!(
            out.status.success(),
            "orientation over the directory pack must exit 0; got {:?}\nstderr:\n{}",
            out.status,
            String::from_utf8_lossy(&out.stderr),
        );
        let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
        stdout
            .lines()
            .find(|l| l.starts_with("Pack: "))
            .unwrap_or_else(|| panic!("orientation must render a `Pack: ` label; got:\n{stdout}"))
            .split(" · ")
            .next()
            .expect("the `Pack: ` segment")
            .to_owned()
    };

    let v1 = label(pack_v1.path());
    let v2 = label(pack_v2.path());

    assert_eq!(
        v1, "Pack: dev/0.3.0",
        "the v1 dir's defaults.yaml `version: 0.3.0` must surface in the provenance label",
    );
    assert_eq!(
        v2, "Pack: dev/0.4.0",
        "the v2 dir's defaults.yaml `version: 0.4.0` must surface in the provenance label",
    );
    assert_ne!(
        v1, v2,
        "two pack dirs with distinct config/defaults.yaml `version:` must report distinct \
         pack_version()s through the binary",
    );
}
