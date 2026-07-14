//! T3 — the methodology pack composes the `dev-task` spine **deterministically**
//! (byte-identical `--task` recompose) and **orientation surfaces the methodology
//! workflow**, both proven through the real binary on a throwaway repo (M12 Inc 1;
//! `design/self-hosting.md` → Determinism via `--task` recompose + The dogfood
//! acceptance flow; `worked-examples.md` → flow 15 bar #1).
//!
//! T1 shipped the pack substrate, T2 proved the spine composes + the emitted-bytes
//! fill contract. T3 proves the two facts the *determinism* bar (flow 15 bar #1)
//! turns on:
//!
//!   (1) **Byte-identical recompose (determinism by re-execution, hardening #7).**
//!       Capture the stdout of the *original successful* `jigc start "<intent>"`
//!       mint, then recompose via `jigc start --task <id>` — the recompose must be
//!       **byte-identical** to the mint capture. A single recompose is not a valid
//!       re-execution proof, so this drives the recompose **twice** and asserts all
//!       three captures (mint + two recomposes) are byte-identical: the deterministic
//!       composer yields the same bytes no matter how often the read-path runs.
//!       CRITICAL (per the planner note + Verified friction): we diff against the
//!       MINT's successful stdout — re-*minting* the same intent correctly fails
//!       `already active`, so it is never a substitute for `--task` recompose.
//!
//!   (2) **Orientation surfaces the methodology workflow.** Bare `jigc start` (no
//!       intent) ORIENTS rather than mints (the pack's `default-workflow: dev-task`
//!       is a `creates-task: true` work-workflow, so the front door routes among the
//!       workflows). Its output must carry `dev-task`, its `when` hint, and the
//!       `Pack: methodology/<version>` provenance label.
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
            "jigc-methodology-recompose-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc` and return its stdout, asserting it exited 0 (with stderr on failure).
fn stdout_ok(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> String {
    let out = run_jigc(repo, home, pack, args);
    assert!(
        out.status.success(),
        "`jigc {}` must exit 0; got {:?}\nstderr:\n{}",
        args.join(" "),
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn recompose_via_task_is_byte_identical_to_the_mint_capture() {
    let repo = TempDir::new("recompose");
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

    // Capture the stdout of the ORIGINAL SUCCESSFUL mint. The `add rate limiter`
    // intent derives the `add-rate-limiter` slug + task id.
    let intent = "add rate limiter";
    let mint = stdout_ok(repo.path(), home.path(), &pack, &["start", intent]);

    // Sanity: the mint actually composed the dev-task spine (not an error/orient
    // page) — guards the byte-identical check from passing over a degenerate
    // "two identical error pages" capture.
    assert!(
        mint.contains("done-criterion") && mint.contains("jigc task finalize add-rate-limiter"),
        "the mint capture must be the composed dev-task spine; got:\n{mint}",
    );

    // Determinism by RE-EXECUTION (hardening #7): recompose via `--task` twice and
    // assert every capture is byte-identical to the mint. A deterministic composer
    // yields the same bytes however often the read-path runs.
    let recompose_a = stdout_ok(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--task", "add-rate-limiter"],
    );
    let recompose_b = stdout_ok(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--task", "add-rate-limiter"],
    );

    // The mint capture opens with the frontend's `task minted: <id>` header (M42) — an
    // invocation fact (this run minted the id), not view state, so a recompose (which
    // mints nothing) carries none (`workflow-dialect.md` → The `task minted:` header).
    // The determinism claim is about the composed *view*: below the header, byte-identical.
    let minted_view = mint
        .strip_prefix("task minted: add-rate-limiter\n\n")
        .unwrap_or_else(|| panic!("the mint announces the id it minted; got:\n{mint}"));
    assert!(
        !recompose_a.contains("task minted:"),
        "the recompose mints nothing — it announces no mint; got:\n{recompose_a}",
    );
    assert_eq!(
        minted_view, recompose_a,
        "the `--task` recompose must be BYTE-IDENTICAL to the original successful mint \
         (below the mint header)",
    );
    assert_eq!(
        recompose_a, recompose_b,
        "the recompose must itself be stable across repeated re-execution (not one lucky run)",
    );

    // Guard against the planner's trap: re-MINTING the same intent must FAIL
    // `already active` — so the byte-identical diff above is genuinely against the
    // mint's success output, never a re-mint's error page masquerading as a recompose.
    let remint = run_jigc(repo.path(), home.path(), &pack, &["start", intent]);
    assert!(
        !remint.status.success(),
        "re-minting the same intent must FAIL (a task is already active) — the recompose \
         must come from `--task`, never a re-mint; got success",
    );
    let remint_err = format!(
        "{}{}",
        String::from_utf8_lossy(&remint.stdout),
        String::from_utf8_lossy(&remint.stderr),
    );
    assert!(
        remint_err.contains("already active"),
        "the re-mint must fail naming the active task; got:\n{remint_err}",
    );
    assert_ne!(
        mint, remint_err,
        "the re-mint's error output must NOT equal the mint capture (proving we never diff \
         a recompose against a re-mint)",
    );
}

#[test]
fn bare_start_orients_with_the_methodology_workflow_its_when_hint_and_pack_label() {
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

    // Bare `jigc start` (no intent) ORIENTS — the pack's `default-workflow: dev-task`
    // is `creates-task: true`, so the front door routes among the workflows rather
    // than minting. The orientation must surface the methodology workflow.
    let stdout = stdout_ok(repo.path(), home.path(), &pack, &["start"]);

    assert!(
        stdout.contains("dev-task"),
        "orientation must list the methodology workflow `dev-task`; got:\n{stdout}",
    );
    assert!(
        stdout.contains("implement one scoped change test-first, end-to-end"),
        "orientation must carry the `dev-task` `when` hint; got:\n{stdout}",
    );
    assert!(
        stdout.contains("Pack: methodology/"),
        "orientation must render the `Pack: methodology/<version>` provenance label; \
         got:\n{stdout}",
    );

    // Bare start ORIENTS, it does not mint — the composed dev-task spine (the
    // finalize Run line) must be absent (guards against the pack accidentally
    // shipping a `creates-task: false` default that auto-mints / front-doors a spine).
    assert!(
        !stdout.contains("jigc task finalize"),
        "bare `jigc start` over a `creates-task: true` default must ORIENT, not compose the \
         spine; got:\n{stdout}",
    );
}
