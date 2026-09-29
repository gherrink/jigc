//! M15 Increment 2 / T1 — the methodology pack's `increment` workflow composes
//! through the real binary, emitting its three checkpoint halts as structural
//! `Checkpoint:` directives in body order. This drives the **shipped methodology
//! pack** (NOT a fixture), proving the pack encode of `implementation/increment-
//! workflow.md`'s outer loop (plan → execute → validate → fix) composes end-to-end
//! over the already-shipped checkpoint step kind with **zero engine change**.
//!
//! The done-criterion (`design/worked-examples.md` → flow 18 setup + walk):
//!
//!   - `JIGC_PACK_DIR=<packs/methodology> jigc start --workflow increment "<intent>"`
//!     exits 0;
//!   - the composed stdout carries `Checkpoint: new-fork-at-plan`,
//!     `Checkpoint: blocked-task`, `Checkpoint: fix-rounds-exhausted`, each at a
//!     line's left margin, each at the HEAD of its gate step before that gate's body
//!     prose, and in the order plan → plan-gate → execute → execute-gate → validate
//!     → fix-gate (asserted on the EMITTED bytes via strictly-increasing offsets,
//!     never a reconstruction);
//!   - no `{{ … }}` placeholder survives the compose (every include/data-value/
//!     command-ref resolved);
//!   - `increment` is `creates-task: false` (the outer loop mints no task — the
//!     intent threads by agent substitution, like the router).
//!
//! No external test crates: the binary path is `CARGO_BIN_EXE_jigc`, the methodology
//! pack is `CARGO_MANIFEST_DIR/../../packs/methodology`, and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-increment-compose-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (composition reads HEAD via
/// `git rev-parse`). `jigc setup` itself creates the `.jigc/config/` project layer.
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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, and the methodology
/// pack selected via `JIGC_PACK_DIR` (the sole-pack seam flow 18 drives).
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Set up the methodology pack as the sole pack over a fresh repo, then compose
/// `--workflow increment "<intent>"`, returning the emitted stdout. Asserts the
/// compose exits 0 (the no-engine-change assumption — the shipped checkpoint surface
/// composes the pack's three gates without touching `crates/*/src`).
fn compose_increment(intent: &str) -> String {
    let repo = TempDir::new("repo");
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

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "increment", intent],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow increment` over the methodology pack must exit 0 — the \
         already-shipped checkpoint surface composes the three gates with no engine change; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn increment_workflow_emits_the_three_checkpoint_halts_in_body_order() {
    // The headline: the methodology `increment` workflow composes its three halt
    // points as structural `Checkpoint:` directives, in body order, through the real
    // binary — asserted on EMITTED bytes via strictly-increasing offsets, never a
    // reconstruction.
    let stdout = compose_increment("build increment 2 of M15");

    let plan_gate = "Checkpoint: new-fork-at-plan";
    let execute_gate = "Checkpoint: blocked-task";
    let fix_gate = "Checkpoint: fix-rounds-exhausted";

    // Each directive sits at a line's left margin (the emit rule: line-start, bare
    // slug). `lines().any(|l| l == d)` proves it is its OWN line, not a substring of
    // body prose.
    for d in [plan_gate, execute_gate, fix_gate] {
        assert!(
            stdout.lines().any(|l| l == d),
            "`{d}` must compose as its own line at the left margin; got:\n{stdout}",
        );
        // Bare slug, no backticks — it is a label, not a command.
        let slug = d.strip_prefix("Checkpoint: ").unwrap();
        assert!(
            !stdout.contains(&format!("Checkpoint: `{slug}`")),
            "the reason slug `{slug}` must be bare (no backticks); got:\n{stdout}",
        );
    }

    let plan_at = stdout.find(plan_gate).expect("plan-gate directive");
    let execute_at = stdout.find(execute_gate).expect("execute-gate directive");
    let fix_at = stdout.find(fix_gate).expect("fix-gate directive");

    // Strictly-increasing offsets prove body order: plan-gate → execute-gate →
    // fix-gate, the order the workflow's include list fixes.
    assert!(
        plan_at < execute_at && execute_at < fix_at,
        "the three checkpoint directives must compose in body order \
         (plan-gate < execute-gate < fix-gate); got offsets {plan_at}/{execute_at}/{fix_at}\n{stdout}",
    );
}

#[test]
fn each_gate_directive_heads_its_step_before_its_own_body_prose() {
    // The prepend contract (`workflow-dialect.md` → the directive is emitted at the
    // HEAD of the step's text, before the body prose). Each gate's `Checkpoint:`
    // directive must PRECEDE that gate's distinctive body prose, and the three Reason
    // steps' prose must interleave in the plan → plan-gate → execute → execute-gate →
    // validate → fix-gate spine — all on emitted bytes.
    let stdout = compose_increment("build increment 2 of M15");

    // Distinctive body fragments authored into each step (the prose, not the
    // directive). These pin the spine order AND the head-of-step prepend.
    let plan_prose = "single-concern tasks";
    let plan_gate_prose = "genuinely new fork";
    let execute_prose = "strictly serially";
    let execute_gate_prose = "blocked task";
    let validate_prose = "independent";
    let fix_gate_prose = "fix-round cap";

    let find = |needle: &str| -> usize {
        stdout
            .find(needle)
            .unwrap_or_else(|| panic!("the composed view must carry `{needle}`; got:\n{stdout}"))
    };

    let plan = find(plan_prose);
    let plan_gate_dir = stdout.find("Checkpoint: new-fork-at-plan").unwrap();
    let plan_gate_body = find(plan_gate_prose);
    let execute = find(execute_prose);
    let execute_gate_dir = stdout.find("Checkpoint: blocked-task").unwrap();
    let execute_gate_body = find(execute_gate_prose);
    let validate = find(validate_prose);
    let fix_gate_dir = stdout.find("Checkpoint: fix-rounds-exhausted").unwrap();
    let fix_gate_body = find(fix_gate_prose);

    // Head-of-step prepend: each directive precedes its OWN gate's body prose.
    assert!(
        plan_gate_dir < plan_gate_body,
        "plan-gate directive must precede its body prose; got:\n{stdout}",
    );
    assert!(
        execute_gate_dir < execute_gate_body,
        "execute-gate directive must precede its body prose; got:\n{stdout}",
    );
    assert!(
        fix_gate_dir < fix_gate_body,
        "fix-gate directive must precede its body prose; got:\n{stdout}",
    );

    // The full spine order: plan → plan-gate → execute → execute-gate → validate →
    // fix-gate. The Reason steps interleave between the gates exactly as the include
    // list fixes them.
    let offsets = [
        ("plan", plan),
        ("plan-gate", plan_gate_dir),
        ("execute", execute),
        ("execute-gate", execute_gate_dir),
        ("validate", validate),
        ("fix-gate", fix_gate_dir),
    ];
    for w in offsets.windows(2) {
        assert!(
            w[0].1 < w[1].1,
            "spine order broken: `{}` (@{}) must precede `{}` (@{}); got:\n{stdout}",
            w[0].0,
            w[0].1,
            w[1].0,
            w[1].1,
        );
    }
}

#[test]
fn no_placeholder_survives_the_increment_compose() {
    // Every `{{ … }}` (the six step includes; any data-value / command-ref a step
    // body carries) must resolve — a surviving placeholder is an unresolved include
    // or ref the compose should have filled or rejected. The task-ref-free outer loop
    // carries no `{{task.*}}`, so none should leak.
    let stdout = compose_increment("build increment 2 of M15");
    assert!(
        !stdout.contains("{{"),
        "no `{{{{ … }}}}` placeholder may survive the compose; got:\n{stdout}",
    );
}
