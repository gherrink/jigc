//! M42 Increment 11 / T1 — the methodology pack states the **agent-stage contract**
//! ([finalize.md](../../../design/finalize.md) → Dirty-tree policy / *The sibling
//! obligation*: every work-workflow's implement/finalize step states the `git add`
//! contract to the agent, **in every pack**).
//!
//! The mechanism has always been sound — an empty index over a dirty tree already blocks
//! `finalize` with a routed finding. The RC-adoption trial's silent partial commits were a
//! **prompting gap**: the dev pack states the contract twice
//! (`crates/cli/packs/dev/steps/implement.yaml`, `finalize.yaml`) while the methodology pack
//! stated it **nowhere** — `dev-task` / `decided-task` / `planning` / `completion` composed
//! **zero** mentions of `git add` — and `crates/cli/packs/methodology/steps/finalize.yaml`
//! overloaded the word *"Stage"* to mean *set the commit-doc slot* ("Stage the subject line"), an
//! active semantic collision in the one step where the git contract belongs.
//!
//! Everything here is asserted on the **emitted bytes** of the real binary over the
//! marker-composed `[dev ▸ methodology]` pack-set (a normal `jigc setup`, no
//! `JIGC_PACK_DIR`, no listed packs) — the pack-set a dogfooding project actually runs.
//! Note the includes resolve **per-origin-pack**: even though dev wins every top-level
//! collision, methodology's `dev-task` composes *methodology's own* `step:implement` and
//! `step:finalize` — so this contract can only be fixed in `crates/cli/packs/methodology/steps/`.
//!
//! The omitting context is covered too (hardening #5): `increment` is `creates-task: false`
//! and includes **neither** step — it must compose clean and stay **inert** (no contract,
//! no error), never made an error by this change.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-staging-contract-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting
/// `JIGC_PACK_DIR` (the marker path requires it ABSENT).
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

/// A repo whose project layer carries the `compose-embedded-methodology` marker via a
/// *normal* `jigc setup` — the `[dev ▸ methodology]` pack-set a dogfooding project runs.
fn marker_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    init_repo(repo.path());
    let home = TempDir::new("home");
    let setup = run(repo.path(), home.path(), &["setup"]);
    assert!(
        setup.status.success(),
        "a normal `jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );
    (repo, home)
}

/// Compose `jigc start --workflow <workflow> <intent>` and return its emitted stdout.
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

/// The composed body with runs of whitespace collapsed to single spaces. Step prose is
/// hard-wrapped at ~80 columns, so a *sentence* the agent must read can straddle a line
/// break; the contract is the words, not the wrap column. This still asserts over the
/// **emitted** bytes — it just refuses to make the assertion hostage to a reflow.
fn flowed(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The four methodology work-workflows that carry a task to a commit — every one of them
/// composes `step:finalize`, and the two task-shaped ones also compose `step:implement`.
/// (`increment` is `creates-task: false` and includes neither — the omitting context.)
/// Each intent slugs to a *distinct* task id: one repo composes them all in sequence.
const COMMITTING_WORKFLOWS: [(&str, &str); 4] = [
    ("dev-task", "add cache"),
    ("decided-task", "swap the store"),
    ("planning", "M98"),
    ("completion", "M99"),
];

#[test]
fn every_committing_methodology_workflow_composes_the_agent_stage_contract() {
    // The deliverable: each of the four composes the `git add` agent-stage contract AND
    // its consequence (unstaged work is left out) — the sentence an agent must read to
    // avoid the trial's silent partial commits. RED before this task: all four compose
    // ZERO mentions of `git add`.
    let (repo, home) = marker_repo("contract");

    for (workflow, intent) in COMMITTING_WORKFLOWS {
        let body = compose(repo.path(), home.path(), workflow, intent);
        let flat = flowed(&body);
        assert!(
            flat.contains("`git add`"),
            "`{workflow}` must compose the `git add` agent-stage contract \
             (design/finalize.md → the sibling obligation); got:\n{body}",
        );
        assert!(
            flat.contains("only what you have staged"),
            "`{workflow}` must state the contract's CONSEQUENCE — finalize commits only \
             what you have staged — not merely the words `git add`; got:\n{body}",
        );
    }
}

#[test]
fn no_composed_methodology_body_calls_a_slot_write_staging() {
    // The semantic collision dies: "Stage the subject line" / "Stage the body" meant *set
    // the commit-doc slot* in the ONE step where the git contract belongs. No composed
    // methodology body may say it — the word "stage" is git's here.
    let (repo, home) = marker_repo("collision");

    for (workflow, intent) in COMMITTING_WORKFLOWS {
        let body = compose(repo.path(), home.path(), workflow, intent);
        assert!(
            !flowed(&body).contains("Stage the subject"),
            "`{workflow}` must not call a commit-doc slot-write \"Stage\" — it collides with \
             the git staging contract composed in the same step; got:\n{body}",
        );
        assert!(
            !flowed(&body).contains("Stage the body"),
            "`{workflow}` must not call a commit-doc slot-write \"Stage\"; got:\n{body}",
        );
        // No placeholder survives a clean compose (the edited steps still resolve).
        assert!(
            !body.contains("{{") && !body.contains("}}"),
            "`{workflow}` must compose with no surviving `{{{{ … }}}}` placeholder; got:\n{body}",
        );
    }
}

#[test]
fn the_contract_is_inert_in_a_workflow_that_composes_neither_step() {
    // Hardening #5 — the omitting context. `increment` is `creates-task: false` and
    // includes neither `step:implement` nor `step:finalize`: it must compose CLEAN and
    // simply not carry the contract (inert), never error. A step-prose change must not
    // leak into, or break, a workflow that never composes the step.
    let (repo, home) = marker_repo("inert");

    let body = compose(repo.path(), home.path(), "increment", "M99 increment 1");
    assert!(
        !flowed(&body).contains("`git add`"),
        "`increment` composes neither implement nor finalize — the staging contract must be \
         absent there, not leaked; got:\n{body}",
    );
    assert!(
        body.contains("Checkpoint:"),
        "`increment` must still compose its own body (its Checkpoint halts); got:\n{body}",
    );
}

/// The four code-less findings workflows (M55 Increment 8; `design/findings-channel.md`
/// → 2, 3): each composes `step:finalize-doc-only` in place of `step:finalize`, so its
/// commit takes nothing from the index — the path-scoped doc-only commit. They are the
/// **opposite** of [`COMMITTING_WORKFLOWS`]: the agent-stage contract would be a lie
/// here, so it must be absent, and the path-scoped statement present in its place.
/// Each intent slugs to a distinct task id (a slug keeps five words at most).
const DOC_ONLY_WORKFLOWS: [(&str, &str); 4] = [
    ("report-jigc-feedback", "doc show drops a byte"),
    ("report-inconsistency", "cache doc disagrees"),
    ("triage-jigc-feedback", "resolve the dropped byte"),
    ("triage-inconsistency", "settle the cache record"),
];

/// The path-scoped statement, read off `step:finalize-doc-only`'s own text rather than
/// restated: its first paragraph, which says the commit takes only this task's docs and
/// leaves every other staged path staged.
fn doc_only_statement() -> String {
    use engine::packsource::{PackResourceKind, PackSource, ResourceId};
    let pack = cli::pack::EmbeddedPack::methodology();
    let bytes = pack
        .read(
            PackResourceKind::Steps,
            &ResourceId::from("finalize-doc-only"),
        )
        .expect("the methodology pack ships `step:finalize-doc-only`");
    let def = engine::compose::load_step_def("finalize-doc-only", &bytes)
        .expect("`step:finalize-doc-only` loads");
    let paragraph = def
        .body
        .split("\n\n")
        .next()
        .expect("a split yields one part")
        .to_owned();
    assert!(
        paragraph.contains("path-scoped"),
        "the first paragraph of `step:finalize-doc-only` is no longer its path-scoped \
         statement — read the statement from wherever it moved; got:\n{paragraph}",
    );
    flowed(&paragraph)
}

#[test]
fn every_doc_only_methodology_workflow_composes_the_path_scope_not_the_stage_contract() {
    // The opposite of the agent-stage contract: a doc-only commit takes nothing from the
    // index, so "`git add`" and "only what you have staged" would tell the agent to stage
    // work this commit never takes. Each of the four instead composes the path-scoped
    // statement of `step:finalize-doc-only`, read from that step's own text.
    let (repo, home) = marker_repo("doc-only");
    let statement = doc_only_statement();

    for (workflow, intent) in DOC_ONLY_WORKFLOWS {
        let body = compose(repo.path(), home.path(), workflow, intent);
        let flat = flowed(&body);
        assert!(
            !flat.contains("`git add`"),
            "`{workflow}` commits path-scoped, so it must not compose the `git add` \
             agent-stage contract; got:\n{body}",
        );
        assert!(
            !flat.contains("only what you have staged"),
            "`{workflow}` commits path-scoped, so it must not say finalize commits only \
             what you have staged; got:\n{body}",
        );
        assert!(
            flat.contains(&statement),
            "`{workflow}` must compose `step:finalize-doc-only`'s path-scoped statement:\n  \
             {statement}\ngot:\n{body}",
        );
    }
}
