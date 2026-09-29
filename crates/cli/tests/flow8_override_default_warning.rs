//! Flow 8 acceptance — **demote an `override-default` conflict from blocking to
//! warning** so `jigc upgrade` *warns instead of blocks* (exit 0), driven verbatim
//! through the emitted `jigc` binary over the `JIGC_PACK_DIR` v1→v2 seam
//! (`design/worked-examples.md` → flow 8, Headline; `design/validation.md` → The
//! non-task `Probe` seam; `implementation/roadmap.md` → M6 Increment 3, grouped-scope
//! bullet 3).
//!
//! This is M6's headline and the **one genuinely new assertion** of the increment:
//! `upgrade_v1_v2.rs` asserts the same `replace #implement` conflict *blocks*; here it
//! is demoted through the cascade and the binary **warns + exits 0** instead. The
//! supporting trio is delivered by sibling coverage and cross-referenced below so the
//! flow is tracked complete (increment-workflow.md hardening #1 / #6):
//!   - **tunable demote stops blocking** — `severity_tuning.rs` ::
//!     `tunable_severity_demotion_stops_file_state_drift_blocking` (file-state).
//!   - **intrinsic demote floor-rejected + shown in `--explain`** — `severity_tuning.rs`
//!     :: `intrinsic_severity_demotion_is_floor_rejected_and_shown_in_explain`.
//!   - **no-override path byte-identical** — `upgrade.rs` ::
//!     `no_delta_*` byte-identity goldens (the post-pass is inert with no severity
//!     delta recorded).
//!
//! The walk, flow-8 verbatim onto the embedded pack's real `implement` step:
//!
//! Step 1 — under `JIGC_PACK_DIR=<v1>` (a faithful copy of the embedded pack tree)
//! record a `replace #implement` with pinned basis (`config replace-step`), then
//! demote the conflict check through the cascade (`config set
//! validation.override-default.target-unchanged.severity warning`).
//!
//! Step 2 — build `<v2>`: `implement` bytes change (the body otherwise identical), so
//! the recorded basis no longer matches → the classifier fires `content-changed`.
//!
//! Step 3 — `JIGC_PACK_DIR=<v2> jigc upgrade` surfaces the conflict as a **WARNING**
//! (not blocking), reports **0 blocking findings**, and the process **EXITS 0** — the
//! flow-8 headline, asserted on the binary's emitted bytes (hardening #4).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the v1/v2
//! packs are real on-disk copies of the embedded tree, and a self-cleaning `TempDir`
//! keeps the test off the dev's repo (mirroring `upgrade_v1_v2.rs` /
//! `pack_source_determinism.rs`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow8-{tag}-{}-{:?}",
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

/// The embedded pack source tree (`crates/cli/packs/dev/`) — `CARGO_MANIFEST_DIR` is
/// `<root>/crates/cli`, the very tree `include_dir!` embeds. The v1 pack is a faithful
/// copy of it, so the recording verbs read byte-identical pack bytes (mirroring
/// `upgrade_v1_v2.rs`).
fn embedded_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
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

/// Copy the embedded pack tree into `dir` and stamp `config/defaults.yaml`'s
/// `version:` (the `FilesystemPack` reads it through `pack_version`). Returns the pack
/// root the `JIGC_PACK_DIR` selection points at.
fn dir_pack_with_version(dir: &Path, version: &str) -> PathBuf {
    copy_tree(&embedded_pack_tree(), dir);
    let defaults = dir.join("config").join("defaults.yaml");
    let existing = fs::read_to_string(&defaults).expect("read copied defaults.yaml");
    fs::write(&defaults, format!("{existing}version: {version}\n"))
        .expect("write versioned defaults.yaml");
    dir.to_path_buf()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer
/// so the cascade resolves (mirrors `upgrade_v1_v2.rs`).
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

/// Run `jigc config <args>` with `cwd = repo`, `$HOME = home`, the recording pack
/// selected by `JIGC_PACK_DIR=<v1>` — the env-selected obligation the upgrade path
/// honors under v2.
fn run_config(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("config")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc upgrade` with `cwd = repo`, `$HOME = home`, the **current** pack selected
/// by `JIGC_PACK_DIR=<v2>` — the same env-selected pack the recording verbs honored.
fn run_upgrade(repo: &Path, home: &Path, pack_dir: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("upgrade")
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

fn config_dir(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("config")
}

/// Assert a `jigc` invocation succeeded, surfacing stderr on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Build the `<v2>` pack from a fresh copy of the embedded tree, then mutate it:
/// `implement` bytes change (the body otherwise identical to v1), so the recorded
/// `replace #implement` basis no longer matches → the classifier fires
/// `content-changed`. Returns the pack root.
fn build_v2(dir: &Path) -> PathBuf {
    let root = dir_pack_with_version(dir, "0.4.0");
    // The rewritten step still solicits a `doc set-slot` write through the catalog,
    // so it owes the M48 read-back statement + declaration like any shipped
    // soliciting step (`design/surface-contract.md` → The stated-at fence, the
    // write-solicit tier); the fixture is a manifest-shipping pack, hence inside the
    // fence.
    fs::write(
        root.join("steps").join("implement.yaml"),
        "---\nstates-constraints: [read.staged-read-back]\n---\n\
         Implement the change directly in the working tree (v2 rewrite).\n\n\
         {{ cli.set-commit-summary }}\n<<author: {{ task.commit#summary }}>>\n\n\
         Read your write back: jigc doc show commit:{{task.id}} --task {{task.id}}\n",
    )
    .expect("write v2 implement");
    root
}

/// **Flow 8 headline.** Record a `replace #implement` against v1, demote the conflict
/// check (`config set validation.override-default.target-unchanged.severity warning`),
/// then `JIGC_PACK_DIR=<v2> jigc upgrade` against a v2 whose `implement` changed: the
/// conflict is surfaced as a **WARNING**, there are **0 blocking findings**, and the
/// process **EXITS 0** — the demoted conflict warns instead of blocks. Asserted on the
/// binary's emitted bytes (hardening #4); `upgrade_v1_v2.rs` proves the same conflict
/// *blocks* without the demote, so this is the one genuinely new assertion.
#[test]
fn demoted_override_default_conflict_warns_instead_of_blocks_through_the_binary() {
    let repo = TempDir::new("repo");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // The two genuine packs across the `v1 → v2` boundary.
    let v1_dir = TempDir::new("pack-v1");
    let v1 = dir_pack_with_version(v1_dir.path(), "0.3.0");
    let v2_dir = TempDir::new("pack-v2");
    let v2 = build_v2(v2_dir.path());

    // ---- Step 1a: record a `replace #implement` under JIGC_PACK_DIR=<v1> ----
    // The basis is pinned to v1's `implement`; v2 changes it → a conflict.
    let project_impl = repo.path().join("project-impl.yaml");
    fs::write(
        &project_impl,
        "{{ include: step:implement }}\nProject house rule for implementing.\n",
    )
    .expect("write replacement source");
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &[
                "replace-step",
                "workflow:single-task#implement",
                project_impl.to_str().expect("utf-8 path"),
            ],
        ),
        "config replace-step implement",
    );

    // ---- Step 1b: demote the conflict check through the cascade ----
    // `validation.override-default.target-unchanged.severity = warning` — a real,
    // tunable knob (no floor), so the demotion applies at resolution.
    expect_ok(
        &run_config(
            repo.path(),
            home.path(),
            &v1,
            &[
                "set",
                "validation.override-default.target-unchanged.severity",
                "warning",
            ],
        ),
        "config set override-default.target-unchanged.severity warning",
    );

    // The manifest carries every recorded delta before the upgrade (nothing mutated yet).
    let manifest_before =
        fs::read_to_string(config_dir(repo.path()).join("manifest.yaml")).expect("manifest");

    // ---- Step 2+3: JIGC_PACK_DIR=<v2> jigc upgrade — warns instead of blocks ----
    let out = run_upgrade(repo.path(), home.path(), &v2);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // (1) The flow-8 headline: the process EXITS 0 — the demoted conflict does NOT block.
    assert!(
        out.status.success(),
        "the demoted `override-default` conflict must NOT block — `jigc upgrade` exits 0; \
         got {:?}\nstdout:\n{stdout}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // (2) The conflict is surfaced as a WARNING on the emitted bytes — the exact
    // `warning · <code> — <message>` line the renderer composes (not a reconstruction).
    assert!(
        stdout.contains(
            "warning · override-default.content-changed — override target \
             `workflow:single-task#implement` changed in the current pack since it was recorded"
        ),
        "the demoted conflict must render as a `warning · …` line naming the changed target; \
         got:\n{stdout}",
    );
    // …and it carries its indented `route:` line (the block-payload envelope).
    assert!(
        stdout.contains("\n  route: "),
        "the warning finding must carry an indented `route:` line; got:\n{stdout}",
    );

    // (3) 0 blocking findings: the conflict no longer renders as `blocking`. Pre-demote
    // (`upgrade_v1_v2.rs`) this exact conflict emits `blocking · …`; the demote flips it.
    assert!(
        !stdout.contains("blocking · "),
        "no finding may render as `blocking` after the demote — 0 blocking findings; \
         got:\n{stdout}",
    );

    // Report-and-route only: the manifest is byte-unchanged on disk after the upgrade.
    assert_eq!(
        fs::read_to_string(config_dir(repo.path()).join("manifest.yaml"))
            .expect("manifest still readable"),
        manifest_before,
        "`jigc upgrade` mutates nothing — the manifest must be byte-unchanged",
    );
}
