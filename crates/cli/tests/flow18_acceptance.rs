//! M15 Increment 2 / T3 — the flow-18 **seven-part acceptance** through the real
//! binary (`design/worked-examples.md` → flow 18 → What it asserts). This is the
//! consolidated M15 proof: jigc composes its *own* multi-phase, human-gated,
//! fix-looping development workflow — the [increment-workflow](../../../implementation/increment-workflow.md)
//! (plan → execute → validate → fix) — as a methodology-pack definition, the three
//! halts emitted as machine-recognizable `Checkpoint:` directives, the deepest
//! self-hosting proof beyond M12's reduced-linear slice, with the multi-agent
//! orchestration honestly bounded out.
//!
//! Every assertion runs on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`) — never a reconstruction — over a throwaway git repo with
//! a self-cleaning `TempDir`. The shipped methodology pack
//! (`CARGO_MANIFEST_DIR/../../packs/methodology`) is selected via `JIGC_PACK_DIR`
//! for the headline assertions (1, 5, 6); the scope-honesty contrasts that need a
//! marker-removed sibling, a shadowing prose line, or a `{{task.intent}}` carrier
//! (2, 3, 4) ride a minimal **fixture** pack seeded under the same temp root, since
//! the methodology `increment` workflow itself carries none of those (it is the
//! conformant artifact, not the violating one).
//!
//! The seven-part bar (flow 18 → What it asserts):
//!   1. the `Checkpoint:` emit class is real — each gate step composes to a literal
//!      `Checkpoint: <reason>` line + its body (emitted bytes, methodology pack);
//!   2. a marker that parses but does nothing is a blocking scope finding — composed
//!      output WITH the `checkpoint:` marker DIFFERS from the same body WITHOUT it on
//!      the LIVE fill-aware path (`workflow_refs_with_fills`, `start.rs:541`);
//!   3. `checkpoint-marker-not-shadowed` fires — a prose line-starting `Checkpoint: `
//!      blocks with a `@ <file>:<line>` finding through the binary;
//!   4. the outer loop is task-ref-free — a `{{task.intent}}` inside a
//!      `creates-task: false` workflow blocks `workflow-refs.task-ref-in-no-task-workflow`
//!      (exit non-zero);
//!   5. halting, not branching — same six steps, same order, and TWO FRESH
//!      `jigc start --workflow increment` composes are byte-identical (fresh-vs-fresh,
//!      the M14 A==A discipline — NOT a `--task` resume, which does not exist for
//!      `creates-task: false`);
//!   6. the structural deliverable — the three halt directives present as
//!      machine-recognizable lines (the advance over M12);
//!   7. the honest bound — the ≤3-round loop / per-phase-agent independence /
//!      independent validate stay orchestration-level, pinned in a comment, never
//!      claimed as composed.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow18-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — `CARGO_MANIFEST_DIR`
/// is `<root>/crates/cli`, so the pack tree is two parents up plus `packs/methodology`.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// The three checkpoint reason slugs the methodology `increment` workflow's gates
/// carry, in body order (plan → execute → fix). The bare label the `Checkpoint:`
/// directive carries — no backticks; it is a label, not a command.
const PLAN_GATE: &str = "Checkpoint: new-fork-at-plan";
const EXECUTE_GATE: &str = "Checkpoint: blocked-task";
const FIX_GATE: &str = "Checkpoint: fix-rounds-exhausted";

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

/// Create the `.jigc/config/` project cascade layer directly (the fixture-pack
/// tests do not run `jigc setup` — the fixture packs ship no setup adapter — so they
/// mint the layer the cascade selector requires by hand, the `checkpoint_acceptance.rs`
/// mechanism).
fn make_project_layer(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Record the listed fixture pack in the in-repo project layer's `packs.yaml` — the
/// pre-cascade selector `make_pack()` CWD-discovers. Listing (rather than
/// `JIGC_PACK_DIR`) UNIONS the fixture pack with the embedded dev base, so the base
/// supplies `knobs`/`defaults` while the fixture supplies only its own workflow +
/// steps (the `start_compose.rs` / `checkpoint_acceptance.rs` listed-pack mechanism).
fn write_packs_yaml(repo: &Path, pack: &Path) {
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the listed pack");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, and `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, and NO `JIGC_PACK_DIR`
/// — the embedded base pack stays in the union, and the listed fixture pack (named in
/// `packs.yaml`) composes alongside it. The fixture-pack assertions (2, 3, 4) use
/// this so the base supplies `knobs`/`defaults`.
fn run_jigc_listed(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Set up the methodology pack as the sole pack over a fresh repo, then compose
/// `--workflow increment "<intent>"`, returning the emitted stdout. Asserts the
/// compose exits 0.
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
        "`jigc start --workflow increment` over the methodology pack must exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn part1_checkpoint_emit_class_is_real_each_gate_directive_heads_its_body() {
    // (1) The `Checkpoint:` emit class is real, not narrated. Over the SHIPPED
    // methodology pack, each gate step composes to a literal `Checkpoint: <reason>`
    // line at the left margin, before its own body prose — driven from the emitted
    // bytes of the real binary, never a reconstruction. The reason slug is bare (no
    // backticks; it is a label).
    let stdout = compose_increment("build increment 2 of M15");

    // Distinctive body fragments authored into each gate step (the prose that must
    // FOLLOW the directive, never precede it).
    let pairs = [
        (PLAN_GATE, "genuinely new fork"),
        (EXECUTE_GATE, "blocked task"),
        (FIX_GATE, "fix-round cap"),
    ];
    for (directive, body_head) in pairs {
        // Its OWN line at the left margin (the emit rule: line-start, bare slug).
        assert!(
            stdout.lines().any(|l| l == directive),
            "`{directive}` must compose as its own line at the left margin; got:\n{stdout}",
        );
        let slug = directive.strip_prefix("Checkpoint: ").unwrap();
        assert!(
            !stdout.contains(&format!("Checkpoint: `{slug}`")),
            "the reason slug `{slug}` must be bare (no backticks); got:\n{stdout}",
        );
        // Head-of-step prepend: the directive precedes its own gate's body prose.
        let directive_at = stdout.find(directive).unwrap();
        let body_at = stdout
            .find(body_head)
            .unwrap_or_else(|| panic!("gate body `{body_head}` must compose; got:\n{stdout}"));
        assert!(
            directive_at < body_at,
            "the `{directive}` directive must PRECEDE its body prose `{body_head}` \
             (head-of-step prepend); got:\n{stdout}",
        );
    }
}

#[test]
fn part2_marker_changes_output_on_the_live_fill_aware_path() {
    // (2) A checkpoint marker that parses but does nothing is a blocking scope
    // finding — so composed output WITH the `checkpoint:` marker must DIFFER from the
    // same body WITHOUT it (directive present vs. inert Reason prose), against the
    // LIVE fill-aware compose path (`workflow_refs_with_fills`, `start.rs:541`), the
    // M8 face-#5 / M4 context-scoped-check discipline. The methodology `increment`
    // workflow carries the conformant marker; the contrast is a fixture pack whose
    // single gate step is seeded WITH and WITHOUT the marker, the body byte-identical.
    let with = compose_fixture_gate("tighten the gate", true);
    let without = compose_fixture_gate("tighten the gate", false);

    assert_ne!(
        with, without,
        "the checkpoint marker must change the composed output on the live fill-aware path \
         (the parsed-but-ignored guard); WITH and WITHOUT composed identically",
    );
    assert!(
        with.contains("Checkpoint: fixture-halt"),
        "the WITH-marker compose must carry the `Checkpoint:` directive; got:\n{with}",
    );
    assert!(
        !without.contains("Checkpoint:"),
        "the WITHOUT-marker compose must carry NO `Checkpoint:` line (inert Reason prose); \
         got:\n{without}",
    );
    // The body prose is identical across both — only the directive line is net-new.
    assert!(
        with.contains(FIXTURE_GATE_BODY) && without.contains(FIXTURE_GATE_BODY),
        "both variants must compose the identical body prose; with:\n{with}\nwithout:\n{without}",
    );
}

#[test]
fn part3_checkpoint_marker_not_shadowed_fires_through_the_binary() {
    // (3) `checkpoint-marker-not-shadowed` fires — authored step prose must not
    // line-start `Checkpoint: `. A shadowing definition is rejected at compose-time
    // with a step-file + line pointer, the same discipline as the run/spawn shadow
    // siblings — surfaced through the binary on the live fill-aware compose path.
    let repo = TempDir::new("shadow-repo");
    init_repo(repo.path());
    make_project_layer(repo.path());
    let home = TempDir::new("shadow-home");
    let pack = TempDir::new("shadow-pack");
    seed_fixture_pack(pack.path());

    let workflows = pack.path().join("workflows");
    let steps = pack.path().join("steps");
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
    // with `Checkpoint: ` — the shadow the conformance check rejects.
    fs::write(
        steps.join("shadows-checkpoint.yaml"),
        "Do the work, then:\n\
         Checkpoint: this-prose-shadows-the-marker\n\
         and stop.\n",
    )
    .expect("seed the shadowing step");
    write_packs_yaml(repo.path(), pack.path());

    let out = run_jigc_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "shadow-flow", "do a thing"],
    );
    assert!(
        !out.status.success(),
        "a step shadowing the `Checkpoint: ` marker must block compose non-zero; \
         got {:?}\nstdout:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("shadows the composer-reserved `Checkpoint: ` marker")
            && stderr.contains("this-prose-shadows-the-marker"),
        "the block must name the shadowed `Checkpoint: ` marker and the offending prose \
         (the located @ <file>:<line> finding); got:\n{stderr}",
    );
}

#[test]
fn part4_task_ref_in_the_no_task_outer_loop_blocks() {
    // (4) The outer loop is task-ref-free. `increment` is `creates-task: false`; a
    // `{{task.intent}}` placeholder inside it is a blocking
    // `workflow-refs.task-ref-in-no-task-workflow` finding (exit non-zero). The
    // composed spine threads the intent by agent-substitution, never a task context.
    // A fixture `creates-task: false` workflow carrying a `{{task.intent}}` step
    // proves the guard FIRES (the methodology `increment` carries none, by design —
    // this is the violating sibling that must be rejected).
    let repo = TempDir::new("taskref-repo");
    init_repo(repo.path());
    make_project_layer(repo.path());
    let home = TempDir::new("taskref-home");
    let pack = TempDir::new("taskref-pack");
    seed_fixture_pack(pack.path());

    let workflows = pack.path().join("workflows");
    let steps = pack.path().join("steps");
    fs::write(
        workflows.join("taskref-flow.yaml"),
        "---\n\
         when: a fixture creates-task:false workflow that illegally references task.*\n\
         description: A fixture-pack-only no-task workflow carrying a task.* ref.\n\
         usage: proving the task-ref-in-no-task-workflow guard fires through the binary.\n\
         creates-task: false\n\
         ---\n\
         {{ include: step:carries-task-ref }}\n",
    )
    .expect("seed taskref-flow workflow");
    // A step body carrying `{{task.intent}}` — illegal in a creates-task:false loop.
    fs::write(
        steps.join("carries-task-ref.yaml"),
        "Work the increment for the following intent:\n\n{{ task.intent }}\n\nand stop.\n",
    )
    .expect("seed the task-ref step");
    write_packs_yaml(repo.path(), pack.path());

    let out = run_jigc_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "taskref-flow", "build the increment"],
    );
    assert!(
        !out.status.success(),
        "a `{{{{ task.intent }}}}` inside a `creates-task: false` workflow must block compose \
         non-zero; got {:?}\nstdout:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        stderr.contains("cannot be referenced by a `creates-task: false` workflow"),
        "the block must be the `task-ref-in-no-task-workflow` conformance finding; got:\n{stderr}",
    );
}

#[test]
fn part5_two_fresh_composes_are_byte_identical_halting_not_branching() {
    // (5) The invariant holds — halting, not branching. The increment-workflow
    // composes the SAME six steps in the SAME order every time (structure
    // task-independent); the checkpoint adds a halt the agent honors, no step
    // appears/disappears on runtime state. Determinism is FRESH-vs-FRESH: `increment`
    // is `creates-task: false`, so it mints NO task to resume (`start.rs:512` — the
    // no-mint branch), and there is no `--task` resume path. Comparing two FRESH
    // composes is the correct oracle (the M14 A==A discipline: a resume-vs-resume
    // comparison would not even be available here, and a resume-vs-resume oracle
    // proves only idempotence).
    let first = compose_increment("build increment 2 of M15");
    let second = compose_increment("build increment 2 of M15");

    assert_eq!(
        first, second,
        "two FRESH `jigc start --workflow increment` composes of the same intent must be \
         byte-identical — the increment-workflow composes the same six steps in the same \
         order every time (halting, not branching; structure task-independent)",
    );

    // Same six steps, same order: the three gate directives compose in body order in
    // BOTH (plan-gate < execute-gate < fix-gate), and nothing appears/disappears.
    for compose in [&first, &second] {
        let plan_at = compose.find(PLAN_GATE).expect("plan-gate directive");
        let execute_at = compose.find(EXECUTE_GATE).expect("execute-gate directive");
        let fix_at = compose.find(FIX_GATE).expect("fix-gate directive");
        assert!(
            plan_at < execute_at && execute_at < fix_at,
            "the three checkpoint directives must compose in body order every time; \
             got offsets {plan_at}/{execute_at}/{fix_at}\n{compose}",
        );
    }
}

#[test]
fn part6_the_three_halts_are_machine_recognizable_directive_lines() {
    // (6) The structural deliverable is the halts-as-directives, not the prose — the
    // advance over M12 (whose dev-workflow Settle-halt was only buried prose). The
    // three checkpoint halts compose as machine-recognizable `Checkpoint: <slug>`
    // lines an orchestrator can extract by line-prefix, in body order, from the
    // emitted bytes.
    let stdout = compose_increment("build increment 2 of M15");

    // Extract the directive lines the way an orchestrator would: filter by the
    // reserved line-prefix, take the bare slugs, assert the exact set in body order.
    let slugs: Vec<&str> = stdout
        .lines()
        .filter_map(|l| l.strip_prefix("Checkpoint: "))
        .collect();
    assert_eq!(
        slugs,
        vec!["new-fork-at-plan", "blocked-task", "fix-rounds-exhausted"],
        "the three halts must be machine-recognizable `Checkpoint: <slug>` lines in body \
         order (the M15 advance over M12's buried-prose halt); got:\n{stdout}",
    );
}

// (7) The honest bound — observed, not over-claimed. The flow proves jigc *composes*
// the multi-phase human-gated spine. The orchestration-level half stays out of the
// composed artifact, exactly as M12 bounded its reduced-linear encode and M8 carved
// out the genuine spawn (`design/self-hosting.md` → the two-half pattern):
//
//   - the bounded ≤3-round validate→fix iteration runs in the BUILD HARNESS
//     (`.claude/workflows/milestone-build.js`), never composed — the dialect
//     contributes the terminal halt marker (`fix-rounds-exhausted`), NOT the loop;
//   - one agent per phase (plan / execute / validate as distinct agents) is
//     orchestration, not a composed primitive;
//   - the independent READ-ONLY validate (a separate agent that does not edit/commit)
//     is orchestration, not a composed primitive.
//
// These tests assert ONLY the composed spine (the six-step structure + the three
// machine-recognizable halts). None asserts the loop, the per-phase agents, or the
// independent validator as *composed* — presenting the composed spine as the whole
// orchestration loop is the hollow-dogfood trap this comment pins out of scope.

// --- Fixture-pack helpers (assertions 2, 3, 4) ----------------------------------

/// The distinctive body prose of the fixture gate step — identical across the
/// WITH-marker and WITHOUT-marker variants, so the only difference assertion 2 can
/// observe is the prepended directive line.
const FIXTURE_GATE_BODY: &str = "Pause here and surface the fork to the human.";

/// Seed a minimal filesystem fixture pack carrying an empty command catalog. The
/// caller adds the workflow + step(s) the specific assertion needs.
fn seed_fixture_pack(pack: &Path) {
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    fs::create_dir_all(&workflows).expect("mk workflows/");
    fs::create_dir_all(&steps).expect("mk steps/");
    fs::create_dir_all(&config).expect("mk config/");
    // A composing workflow's `{{cli.X}}` command-refs resolve against its OWN pack's
    // catalog; the fixture's bodies reference no command, so an empty catalog
    // satisfies the read without declaring any (`multi-pack.md` → Pack-local
    // body-reference resolution → Command-refs).
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
}

/// Compose a fixture `creates-task: true` workflow whose single gate step is seeded
/// WITH (`checkpoint_kind = true`) or WITHOUT the `checkpoint:` front-matter marker,
/// the body byte-identical across both. Returns the emitted stdout. This is the
/// with-vs-without pair assertion 2's M8 face-#5 guard contrasts on the live path.
fn compose_fixture_gate(intent: &str, checkpoint_kind: bool) -> String {
    let repo = TempDir::new("fixgate-repo");
    init_repo(repo.path());
    make_project_layer(repo.path());
    let home = TempDir::new("fixgate-home");
    let pack = TempDir::new("fixgate-pack");
    seed_fixture_pack(pack.path());

    let workflows = pack.path().join("workflows");
    let steps = pack.path().join("steps");
    fs::write(
        workflows.join("fixture-gate-flow.yaml"),
        "---\n\
         when: a fixture workflow exercising the checkpoint step kind\n\
         description: A fixture-pack-only workflow carrying a single gate step.\n\
         usage: proving the marker changes composed output on the live path.\n\
         creates-task: true\n\
         ---\n\
         {{ include: step:fixture-gate }}\n",
    )
    .expect("seed fixture-gate-flow workflow");

    let gate = if checkpoint_kind {
        format!("---\ncheckpoint:\n  reason: fixture-halt\n---\n{FIXTURE_GATE_BODY}\n")
    } else {
        format!("{FIXTURE_GATE_BODY}\n")
    };
    fs::write(steps.join("fixture-gate.yaml"), gate).expect("seed fixture gate step");
    write_packs_yaml(repo.path(), pack.path());

    let out = run_jigc_listed(
        repo.path(),
        home.path(),
        &["start", "--workflow", "fixture-gate-flow", intent],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow fixture-gate-flow` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}
