//! M15 Increment 1 / T4 — real-binary acceptance for the `checkpoint` step kind
//! over a **fixture test pack** (NOT the methodology pack; the pack encode rides in
//! Increment 2). Drives the built `jigc` binary against throwaway git repos with a
//! filesystem pack carrying a checkpoint step, and discharges scope bullet 4 — the
//! forward-review Red obligations (`implementation/increment-workflow.md` →
//! Validation hardening #5 face; the M14 A==A-oracle mask discipline;
//! `design/worked-examples.md` → flow 18, structural deliverable only):
//!
//!   (a) the composed stdout emits `Checkpoint: <slug>` at the **head** of the
//!       checkpoint step, **before** its body prose (the prepend, not an append);
//!   (b) it **differs** with vs. without the marker on the **live fill-aware path**:
//!       the same pack with the checkpoint step's front-matter stripped composes the
//!       identical body with **no** directive line (the M8 face-#5 parsed-but-ignored
//!       guard — a marker that parses but does nothing would compose identically);
//!   (c) **resume-vs-FRESH** byte-identical: `jigc start --task <id>` re-compose of a
//!       minted checkpoint workflow equals a *fresh* `--workflow` compose byte-for-byte
//!       with the same (no-)halt verdict — the oracle is a FRESH compose, never a
//!       second resume (the M14 A==A-oracle mask: comparing resume to resume proves
//!       only idempotence, not that the halt carries no progress state);
//!   (d) a checkpoint-shadowing prose line (`Checkpoint: ` at a step body's left
//!       margin) is **blocked** by `workflow-refs.checkpoint-marker-not-shadowed`
//!       through the binary — non-zero, with the route.
//!
//! Every assertion runs on the EMITTED bytes the agent would actually see
//! (hardening #4). The fixture pack is seeded fresh per test under a self-cleaning
//! `TempDir`, listed in the in-repo `.jigc/config/packs.yaml` the cascade selector
//! CWD-discovers (the `start_compose.rs` listed-pack mechanism), so the new
//! `creates-task: true` checkpoint workflow composes via `--workflow <X>` through the
//! union with the embedded dev base (its body step `{{ include: step:locate }}` pulls
//! an embedded step, so the fixture stays minimal).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init` (composition mints, which reads HEAD), and a
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
            "jigc-checkpoint-accept-{tag}-{}-{:?}",
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

/// The checkpoint step's `reason` slug — the bare label the `Checkpoint:` directive
/// carries (no backticks; it is a label, not a command).
const REASON: &str = "new-fork-at-plan";

/// The distinctive first line of the checkpoint step's body prose — the prose that
/// must follow the prepended directive, never precede it.
const BODY_HEAD: &str = "Pause here: a new design fork has surfaced.";

/// Initialize a real git repo with one commit (composition mints, which reads HEAD
/// via `git rev-parse`) and create the `.jigc/config/` project layer so the cascade
/// resolves.
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

/// Seed a filesystem test pack OUTSIDE the repo carrying a `creates-task: true`
/// workflow whose body is `[orient (a plain pack step), plan-gate (the checkpoint
/// step)]`. The plain step composes a stable body BEFORE the checkpoint, so the
/// directive's **head-of-its-own-step prepend** is observable against the surrounding
/// prose (it must sit between `orient`'s body and `plan-gate`'s body).
///
/// `checkpoint_kind = true` writes `plan-gate` WITH the `checkpoint:` front-matter
/// marker; `false` writes the identical body as a plain step (no front-matter) — the
/// with-vs-without pair the M8 face-#5 guard contrasts.
fn seed_checkpoint_pack(pack: &Path, checkpoint_kind: bool) {
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    fs::create_dir_all(&workflows).expect("mk workflows/");
    fs::create_dir_all(&steps).expect("mk steps/");
    write_empty_catalog(pack);

    fs::write(
        workflows.join("checkpoint-flow.yaml"),
        "---\n\
         when: a fixture workflow exercising the checkpoint step kind\n\
         description: A fixture-pack-only workflow carrying a checkpoint halt directive.\n\
         usage: proving the checkpoint step kind composes through the real binary.\n\
         creates-task: true\n\
         ---\n\
         {{ include: step:orient }}\n\
         {{ include: step:plan-gate }}\n",
    )
    .expect("seed checkpoint-flow workflow");

    // A plain pack step composing a stable body before the checkpoint — the prose the
    // directive must NOT precede (it heads its OWN step, not the whole workflow).
    fs::write(
        steps.join("orient.yaml"),
        "Orient before the gate. Read the surrounding context first.\n",
    )
    .expect("seed orient step");

    // The checkpoint step — the marker lives in front-matter (`workflow-dialect.md` →
    // The checkpoint step kind). With the marker, the composer prepends
    // `Checkpoint: <reason>` before this body; without it, the same body composes as
    // inert Reason prose.
    let plan_gate = if checkpoint_kind {
        format!(
            "---\n\
             checkpoint:\n\
             \x20 reason: {REASON}\n\
             ---\n\
             {BODY_HEAD}\n\
             Surface it to the human, then re-run once the decision is made.\n"
        )
    } else {
        format!(
            "{BODY_HEAD}\n\
             Surface it to the human, then re-run once the decision is made.\n"
        )
    };
    fs::write(steps.join("plan-gate.yaml"), plan_gate).expect("seed plan-gate step");
}

/// Write an EMPTY command catalog into the fixture pack's `config/`. A composing
/// workflow's `{{cli.X}}` command-refs resolve against its OWN pack's catalog (the
/// workflow's origin pack — `start.rs` `compose_core`; `multi-pack.md` → Pack-local
/// body-reference resolution → Command-refs), so the fixture pack — which DEFINES the
/// composed workflow — must ship a `config/commands` even though its steps reference
/// no command (the bodies are plain prose + the checkpoint directive). An empty
/// catalog satisfies the read without declaring any command.
fn write_empty_catalog(pack: &Path) {
    let config = pack.join("config");
    fs::create_dir_all(&config).expect("mk config/");
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty command catalog");
}

/// Record the listed test pack in the in-repo project layer's `packs.yaml` — the
/// pre-cascade selector `make_pack()` CWD-discovers.
fn write_packs_yaml(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the listed pack");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Compose `--workflow checkpoint-flow "<intent>"` over a fresh repo whose listed
/// pack is seeded with (or without) the checkpoint marker, returning the emitted
/// stdout. Each call uses its own repo + home so the only varying input is the pack
/// the test asks for.
fn compose(intent: &str, checkpoint_kind: bool) -> String {
    let repo = TempDir::new("compose");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = TempDir::new("pack");
    seed_checkpoint_pack(pack.path(), checkpoint_kind);
    write_packs_yaml(repo.path(), pack.path());

    let out = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "checkpoint-flow", intent],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow checkpoint-flow` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn checkpoint_directive_emits_at_the_head_of_its_step_before_the_body() {
    // (a) The live fill-aware compose path (`jigc start --workflow`) emits the
    // `Checkpoint: <slug>` directive at the HEAD of the checkpoint step, before its
    // body prose. Asserted on the emitted bytes: the directive line exists, carries
    // the bare reason slug (no backticks), and precedes the step's body head — which
    // in turn sits AFTER the preceding plain step's body (the directive heads its own
    // step, not the whole workflow).
    let stdout = compose("add rate limiter", true);

    let directive = format!("Checkpoint: {REASON}");
    let directive_at = stdout
        .find(&directive)
        .unwrap_or_else(|| panic!("the composed view must carry `{directive}`; got:\n{stdout}"));
    // Bare slug, no backticks — it is a label, not a command.
    assert!(
        !stdout.contains(&format!("Checkpoint: `{REASON}`")),
        "the reason slug must be bare (no backticks); got:\n{stdout}",
    );
    // The directive sits at a line's left margin (line-start), per the emit rule.
    assert!(
        stdout.lines().any(|l| l == directive),
        "`Checkpoint: <slug>` must be its own line at the left margin; got:\n{stdout}",
    );

    let body_at = stdout
        .find(BODY_HEAD)
        .expect("the checkpoint step body must compose");
    assert!(
        directive_at < body_at,
        "the `Checkpoint:` directive must PRECEDE the step body (head-of-step prepend); got:\n{stdout}",
    );

    // The directive heads its OWN step, not the whole workflow: the preceding plain
    // `orient` step's body composes BEFORE the directive.
    let orient_at = stdout
        .find("Orient before the gate.")
        .expect("the preceding orient step must compose");
    assert!(
        orient_at < directive_at,
        "the directive must head only its own step — the preceding `orient` body composes first; got:\n{stdout}",
    );
}

#[test]
fn composed_output_differs_with_vs_without_the_marker_on_the_live_path() {
    // (b) The M8 face-#5 parsed-but-ignored guard on the LIVE fill-aware path: the
    // same fixture pack composed WITH the checkpoint marker differs from the same pack
    // composed WITHOUT it (front-matter stripped, the body identical). A marker that
    // parsed but did nothing would compose identically — so the two must differ, and
    // the difference must be exactly the prepended directive line.
    let with = compose("add rate limiter", true);
    let without = compose("add rate limiter", false);

    assert_ne!(
        with, without,
        "the checkpoint marker must change the composed output (the parsed-but-ignored guard); \
         WITH and WITHOUT composed identically",
    );
    // The WITH variant carries the directive; the WITHOUT variant does not.
    assert!(
        with.contains(&format!("Checkpoint: {REASON}")),
        "the WITH-marker compose must carry the `Checkpoint:` directive; got:\n{with}",
    );
    assert!(
        !without.contains("Checkpoint:"),
        "the WITHOUT-marker compose must carry NO `Checkpoint:` line (inert Reason prose); got:\n{without}",
    );
    // The body prose is identical across both — only the directive line is net-new.
    assert!(
        with.contains(BODY_HEAD) && without.contains(BODY_HEAD),
        "both variants must compose the identical body prose; with:\n{with}\nwithout:\n{without}",
    );
}

#[test]
fn resume_re_compose_is_byte_identical_to_a_fresh_compose() {
    // (c) Resume-vs-FRESH byte-identity (the M14 A==A-oracle mask discipline). A
    // checkpoint workflow minted + resumed via `jigc start --task <id>` must compose
    // byte-for-byte identical to a FRESH `--workflow` compose of the same intent — the
    // oracle is a FRESH compose, NEVER a second resume. The checkpoint adds halting,
    // not progress state: the stateless restart-from-scratch re-derives the SAME view
    // (`workflow-dialect.md` → Resumption is the existing stateless restart). Comparing
    // resume to resume would prove only idempotence; comparing resume to FRESH proves
    // the halt carries no progress cursor that mutates the view.
    let intent = "tighten the gate";

    // FRESH: a brand-new repo composes the workflow from scratch (the oracle).
    let fresh = compose(intent, true);

    // Minted-then-resumed: a SEPARATE repo mints via `--workflow`, then re-composes via
    // `--task <id>`. The resume re-derives the view with no progress state.
    let repo = TempDir::new("resume");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = TempDir::new("pack");
    seed_checkpoint_pack(pack.path(), true);
    write_packs_yaml(repo.path(), pack.path());

    let mint = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "checkpoint-flow", intent],
    );
    assert!(
        mint.status.success(),
        "the checkpoint-flow mint must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&mint.stderr),
    );
    // `checkpoint-flow` is `creates-task: true` — it minted a working area.
    let slug = "tighten-the-gate";
    assert!(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(slug)
            .join("base.json")
            .is_file(),
        "checkpoint-flow is creates-task: true, so it must mint .jigc/tasks/{slug}/",
    );

    let resume = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        resume.status.success(),
        "`jigc start --task <id>` resume must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );
    let resume_out = String::from_utf8(resume.stdout).expect("utf-8 stdout");

    assert_eq!(
        resume_out, fresh,
        "resume of a checkpoint workflow must be byte-identical to a FRESH compose — the \
         checkpoint adds halting, not progress state (the stateless restart re-derives the \
         same view; the FRESH compose is the oracle, never a second resume)",
    );
    // Both carry the same halt verdict: the directive composes in BOTH.
    assert!(
        resume_out.contains(&format!("Checkpoint: {REASON}")),
        "the resume must carry the same `Checkpoint:` directive as the fresh compose; got:\n{resume_out}",
    );
}

#[test]
fn checkpoint_shadowing_prose_is_blocked_through_the_binary() {
    // (d) A step body that shadows the composer-reserved `Checkpoint: ` marker (a line
    // at the left margin starting `Checkpoint: ` in authored prose, NOT a real
    // checkpoint step's own emitted directive) is blocked by
    // `workflow-refs.checkpoint-marker-not-shadowed` — non-zero, with the route —
    // through the real binary on the live fill-aware compose path.
    let repo = TempDir::new("shadow");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = TempDir::new("pack");

    let workflows = pack.path().join("workflows");
    let steps = pack.path().join("steps");
    fs::create_dir_all(&workflows).expect("mk workflows/");
    fs::create_dir_all(&steps).expect("mk steps/");
    write_empty_catalog(pack.path());
    fs::write(
        workflows.join("shadow-flow.yaml"),
        "---\n\
         when: a fixture workflow whose step shadows the Checkpoint marker\n\
         description: A fixture-pack-only workflow that shadows the reserved directive.\n\
         usage: proving checkpoint-marker-not-shadowed fires through the binary.\n\
         creates-task: true\n\
         ---\n\
         {{ include: step:shadows-checkpoint }}\n",
    )
    .expect("seed shadow-flow workflow");
    // A PLAIN step (no `checkpoint:` front-matter) whose authored body starts a line
    // with `Checkpoint: ` — the shadow the conformance check rejects. It is NOT a
    // checkpoint step, so the directive is not the composer's own.
    fs::write(
        steps.join("shadows-checkpoint.yaml"),
        "Do the work, then:\n\
         Checkpoint: this-prose-shadows-the-marker\n\
         and stop.\n",
    )
    .expect("seed the shadowing step");
    write_packs_yaml(repo.path(), pack.path());

    let out = run(
        repo.path(),
        home.path(),
        &["start", "--workflow", "shadow-flow", "do a thing"],
    );
    assert!(
        !out.status.success(),
        "a step shadowing the `Checkpoint: ` marker must block compose non-zero; got {:?}\nstdout:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    // The block is the `checkpoint-marker-not-shadowed` conformance finding — a located
    // shadow error (step file + line), so its envelope names the shadowed `Checkpoint: `
    // marker and the offending prose line, not a repair route (the spawn-shadow sibling
    // carries none either). The load-bearing proof is the non-zero block + the marker
    // named, surfaced through the binary on the live fill-aware compose path.
    assert!(
        stderr.contains("shadows the composer-reserved `Checkpoint: ` marker")
            && stderr.contains("this-prose-shadows-the-marker"),
        "the block must name the shadowed `Checkpoint: ` marker and the offending prose; got:\n{stderr}",
    );
}
