//! M42 Increment 12 / T8 — the planning workflow sanctions the checklist reading,
//! and the Settle gate still binds.
//!
//! Provenance: the rc.5 adoption trial's probe 4 (a full brownfield milestone) —
//! verbatim calibration note (`completions/artifacts/RC-adoption/impl-rc5/trial-record.md:243`):
//! *"the planning workflow's output is a long, prescriptive wall … For a brownfield
//! milestone where I'd already done deep verification against the running binary, most
//! of it was orientation I'd effectively completed. I used it as a checklist rather than
//! literal steps, which felt right"*. The settled fix (`implementation/decisions-pending.md`
//! → the rc.6 wave, P6) is **prose only**: the planning workflow's own text sanctions the
//! reading the trial agent already took, *while restating that the Settle gate still binds*.
//! A lighter brownfield **mode** stays unbuilt — conditionals are barred by the
//! workflow-dialect invariant, so there is no branch to compose.
//!
//! What is asserted, on the EMITTED bytes of the real binary over the composed
//! `[dev ▸ methodology]` pack-set (a normal `jigc setup`, no `JIGC_PACK_DIR`):
//!
//!   (i)  `jigc start --workflow planning "<intent>"` composes the sanction — the agent
//!        is told, in the composed context it actually reads, that already-scouted phases
//!        may be worked as a checklist — AND **still** composes the Settle checkpoint
//!        halt (`Checkpoint: settle`) with its human-owned-gate prose intact. Sanctioning
//!        the checklist reading must not dissolve the one phase that is a *gate*; a pass
//!        on the sanction alone would be a green over a loosened gate.
//!   (ii) The sanction is **context-scoped** to `plan-scope`, the step only `planning`
//!        includes. In an omitting context (`--workflow increment` — the sibling planning
//!        workflow that does NOT include `plan-scope`) the feature is **inert, never an
//!        error**: the compose exits 0 and carries no sanction prose. A green over the
//!        composing context alone would hide a leak into every other workflow.
//!
//! Prose is line-wrapped in the step YAML, so the sanction assertions run over a
//! whitespace-normalized view of the emitted stdout (still the emitted bytes — only the
//! wrap is neutralized). The structural `Checkpoint: settle` marker is asserted raw.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-planning-checklist-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting
/// `JIGC_PACK_DIR` (the marker path composes `[dev ▸ methodology]` and requires it absent).
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// A real git repo whose project layer carries the `compose-embedded-methodology` marker
/// via a normal `jigc setup` — the `[dev ▸ methodology]` pack-set `planning` lives in.
fn marker_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
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
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);

    let setup = run(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "`jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );
    (repo, home)
}

/// Collapse every whitespace run to a single space — the wrap-independent view of the
/// emitted prose (the bytes are the composed step body; only the line wrap is neutralized).
fn unwrapped(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Compose a workflow through the real binary and return its stdout (asserting exit 0).
fn compose(repo: &Path, home: &Path, workflow: &str, intent: &str) -> String {
    let out = run(repo, home, &["start", "--workflow", workflow, intent]);
    assert!(
        out.status.success(),
        "`jigc start --workflow {workflow}` must compose + exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn planning_composes_the_checklist_sanction_and_still_halts_at_settle() {
    // (i) The composing context. The planning spine's own prose sanctions the checklist
    // reading — and the Settle checkpoint halt survives it, human-owned gate intact.
    let (repo, home) = marker_repo("planning");
    let stdout = compose(repo.path(), home.path(), "planning", "M99");
    let flat = unwrapped(&stdout);

    // The sanction: already-scouted phases may be worked as a checklist.
    assert!(
        flat.contains("work them as a checklist"),
        "(i) the planning compose must sanction the checklist reading of the phase walk \
         (probe 4's calibration note); got:\n{stdout}",
    );
    // ... conditioned on the scouting being real, not assumed — the sanction must not
    // read as permission to skip the baseline verification the same step demands.
    assert!(
        flat.contains("Covered means verified, never assumed."),
        "(i) the sanction must condition the skip on verified coverage, never assumed \
         coverage (it sits directly under the RAISE-the-audit-intensity demand); got:\n{stdout}",
    );

    // ... and the Settle gate still binds — stated in the sanction itself.
    assert!(
        flat.contains("The Settle gate still binds"),
        "(i) the sanction must restate that the Settle gate still binds; got:\n{stdout}",
    );

    // ... and, structurally, the Settle CHECKPOINT is still composed: the composer-emitted
    // halt marker (the driver's actual stop signal) and the human-owned-gate prose. This is
    // what makes the sanction safe — the checklist reading may not dissolve the gate.
    assert!(
        stdout.contains("Checkpoint: settle"),
        "(i) the planning compose must STILL emit the structural Settle halt marker \
         `Checkpoint: settle`; got:\n{stdout}",
    );
    assert!(
        flat.contains("The human owns this gate."),
        "(i) the Settle step's human-owned-gate prose must still compose; got:\n{stdout}",
    );

    // A clean compose leaves no unresolved placeholder in the bytes the agent reads.
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "(i) no `{{{{ … }}}}` placeholder may survive the planning compose; got:\n{stdout}",
    );
}

#[test]
fn the_sanction_is_absent_from_a_workflow_that_omits_plan_scope() {
    // (ii) The omitting context (hardening #5). `plan-scope` is included by `planning`
    // alone; `increment` is the sibling planning-side workflow that omits it. The sanction
    // must be INERT there — a clean exit-0 compose with no sanction prose — never an error,
    // and never leaked into a workflow whose phases were not the trial's "prescriptive wall".
    let (repo, home) = marker_repo("increment");
    let stdout = compose(repo.path(), home.path(), "increment", "inc 1");
    let flat = unwrapped(&stdout);

    assert!(
        !flat.contains("work them as a checklist") && !flat.contains("The Settle gate still binds"),
        "(ii) a workflow that omits `plan-scope` must NOT carry the sanction prose — the \
         feature is scoped to the planning phase walk; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "(ii) the omitting context still composes cleanly — no surviving placeholder; \
         got:\n{stdout}",
    );
}
