//! M53 Increment 1 / T1 — **the join's staging area clears what it wrote, and the
//! boundary gate asks the predicate its two siblings ask**
//! (`completions/artifacts/M53/settle-record.md` → D1.3 as amended by §1 and §2;
//! `implementation/roadmap.md` → M53 Increment 1).
//!
//! `engine::milestone::materialize` opened with an unconditional
//! `remove_dir_all(merged/docs)` — *"a clean rebuild, never a stale-body accretion"* — so
//! every byte under `merged/docs/` died on each materialize, whoever wrote it. That is the
//! one remover standing between a foreign byte under `merged/` and the door that is meant
//! to answer for it, and `.jigc/milestones/<id>/merged/` is **not** jigc's alone: a blocked
//! finalize leaves materialized bodies there at exit 3, and a succeeding `pre-commit` hook
//! writes into the area during the commit (`engine/state.rs`, M52's own recorded writer).
//!
//! **The two edits are one task by necessity.** The selective clear is what lets a foreign
//! `<doctype>.md` reach the boundary gate's copy loop at all; and that loop asked
//! `split(':').next()`, which resolves the stem `adr` to the doctype `adr`, copies the file
//! into the gate staging area and blocks the boundary at exit 3 on a file jigc does not own
//! — on every re-run. Its two siblings — `engine::finalize::plan_promotions` and
//! `engine::state::staged_doc_id` — both ask `split_once(':')` and skip a colon-less stem;
//! the loop now asks the same thing. Correcting the loop alone is unobservable through the
//! binary, because the wholesale clear destroys the plant before the loop can see it.
//!
//! **What this suite drives** (the increment's owed spike, per
//! `completions/artifacts/M53/acceptance-design.md` → *Spikes owed*): a foreign file left
//! in `merged/docs/` reaches none of `plan_promotions`, the corrected copy loop, or the
//! merged-state conformance validation — and the selective clear still removes a **stale**
//! jigc body, a doc the current area set no longer produces.
//!
//! The subject is a finalize blocked at exit 3 **on its own cause** (a sub-area doc missing
//! its required `## Decision`, the shipped `milestone_boundary_gate` arm (a) fixture): the
//! block is what leaves the area standing to be read, and it must be the boundary's own
//! cause rather than one of the plants.
//!
//! The plants are read back two ways on purpose. `fs::read_to_string` is byte-exact, and a
//! real `grep -rlE` runs **before and after** as the instrument control — the same
//! instrument the re-review's drivers are told to use, because this harness's own `grep`
//! honours `.gitignore` and `.jigc/` is gitignored, so an instrument that cannot see the
//! plants *before* proves nothing about their absence after.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// The milestone the fixture builds.
const MILESTONE: &str = "cache-rework";

/// The foreign plants left under `merged/docs/`, each with a marker unique enough for the
/// `grep` control to key on. Three distinct cells:
///
/// * `adr.md` — a `.md` whose stem carries **no** `:`, wearing a real doctype's name. The
///   §1 cell: the gate copy loop's `split(':').next()` resolved this to the doctype `adr`.
/// * `deep.txt` — a foreign non-`.md` file, the plain complement member.
/// * `provenance.json` — the §2 cell. A milestone's `merged/docs/` rule is
///   [`engine::state::staged_doc_id`] **alone**; inheriting the task branch's
///   `TASK_DOCS_FILES` would call this jigc's own and destroy it.
const PLANTS: &[(&str, &str)] = &[
    (
        "adr.md",
        "# Notes\n\nFOREIGN-ADR-PLANT — a human's draft, not a staged identity.\n",
    ),
    ("deep.txt", "FOREIGN-DEEP-PLANT\n"),
    ("provenance.json", "{\"FOREIGN-PROVENANCE-PLANT\": true}\n"),
];

/// A **stale jigc body** — a name [`engine::state::staged_doc_id`] recognises, left over
/// from a materialize whose area set has since changed. The selective clear must still take
/// it: the rebuild is clean for what jigc wrote, and only for that.
const STALE_BODY: &str = "adr:stale-policy.md";

/// The stale body's marker, so the grep control can assert its *absence* on an instrument
/// that demonstrably saw it before.
const STALE_MARKER: &str = "STALE-JIGC-BODY-MARKER";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-merged-clear-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success and returning stdout.
fn git_ok(cwd: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// A real git repo with one commit plus the project cascade layer (`jigc milestone`'s
/// door-top precondition, M52 Inc 8 / T1).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README.md");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
    crate::support::mint_project_layer(root);
}

/// Build the pack's real `doc-code` probe once (process-wide) — the merged-state gate's
/// anchor arm shells out to it (the flow13 / `milestone_boundary_gate` idiom).
fn real_doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run `jigc --format json milestone <args>` — the blocked arm's findings envelope carries
/// the finding **codes**, which is what the `unknown-type` assertion reads.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["--format", "json", "milestone"])
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", real_doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation succeeded, surfacing both streams on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area, the
/// two inputs the by-task-id join reads.
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write staged body");

    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String("created".to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize manifest"),
    )
    .expect("write provenance manifest");
}

/// A conformant, ref-free ADR body.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A **non-conformant** ADR body — the required `## Decision` section is missing, so the
/// merged-state gate blocks at exit 3. This is the boundary's **own** cause.
fn adr_missing_decision(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// The milestone's join staging docs dir, `.jigc/milestones/<id>/merged/docs/`.
fn merged_docs(repo: &Path) -> PathBuf {
    repo.join(".jigc")
        .join("milestones")
        .join(MILESTONE)
        .join("merged")
        .join("docs")
}

/// HEAD commit count — the no-commit witness.
fn head_count(repo: &Path) -> u32 {
    git_ok(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("commit count parses")
}

/// **The instrument control** — a real `grep -rlE <pattern> .jigc`, run from `repo`. Not
/// the harness's `grep`, which honours `.gitignore` and would report every one of these
/// plants absent whether it was there or not. Returns the matching repo-relative paths.
fn grep_jigc(repo: &Path, pattern: &str) -> Vec<String> {
    let out = Command::new("grep")
        .args(["-rlE", pattern, ".jigc"])
        .current_dir(repo)
        .output()
        .expect("run grep");
    // `grep` exits 1 when nothing matched — a verdict, not a failure.
    assert!(
        out.status.code() == Some(0) || out.status.code() == Some(1),
        "grep -rlE {pattern:?} failed: {:?}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect()
}

/// **The spike, driven.** A fan-out milestone whose merged staging area holds three foreign
/// plants and one stale jigc body is finalized; the boundary blocks at exit 3 on its own
/// conformance cause, the stale body is gone, and every plant is still on disk.
#[test]
fn a_blocked_boundary_clears_the_stale_body_and_keeps_every_foreign_plant() {
    let repo_dir = TempDir::new("repo");
    let home_dir = TempDir::new("home");
    let repo = repo_dir.path();
    let home = home_dir.path();
    init_repo(repo);

    expect_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Doc area", "Code area"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", MILESTONE, intent]),
            "milestone add-task",
        );
    }

    // One area stages a doc missing its required `## Decision` — the boundary's OWN cause.
    stage_doc(
        repo,
        "doc-area",
        "adr:broken-policy",
        &adr_missing_decision("Broken policy"),
    );
    stage_doc(
        repo,
        "code-area",
        "adr:code-policy",
        &adr_plain("Code policy"),
    );
    expect_ok(
        &run_milestone(repo, home, &["provision", MILESTONE]),
        "milestone provision",
    );

    // Plant into the join's staging area, which `materialize` is about to rebuild.
    let docs = merged_docs(repo);
    fs::create_dir_all(&docs).expect("mk merged/docs/");
    for (name, bytes) in PLANTS {
        fs::write(docs.join(name), bytes).expect("write plant");
    }
    fs::write(
        docs.join(STALE_BODY),
        format!("---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {STALE_MARKER}\n"),
    )
    .expect("write the stale jigc body");

    // ---- The instrument control: grep SEES every plant, and the stale body, before. ----
    for (name, bytes) in PLANTS {
        let marker = bytes
            .split_whitespace()
            .find(|w| w.contains("PLANT"))
            .expect("each plant carries a marker");
        assert!(
            !grep_jigc(repo, marker).is_empty(),
            "the control must SEE `{name}`'s marker {marker} under .jigc/ before the run — \
             an instrument blind to the plant proves nothing about its absence after",
        );
    }
    assert!(
        !grep_jigc(repo, STALE_MARKER).is_empty(),
        "the control sees the stale jigc body before the run",
    );

    let before = head_count(repo);
    let out = run_milestone(repo, home, &["finalize", MILESTONE]);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let both = format!("{stdout}{stderr}");

    // ---- The boundary blocks at exit 3, on its own cause, having committed nothing. ----
    assert_eq!(
        out.status.code(),
        Some(3),
        "the boundary blocks at exit 3 on the non-conformant merged doc; got {:?}\n{both}",
        out.status.code(),
    );
    assert!(
        both.contains("conformance.section"),
        "the block names the boundary's OWN cause — the missing required section; got:\n{both}",
    );
    assert_eq!(
        head_count(repo),
        before,
        "a blocked milestone finalize commits nothing",
    );

    // ---- …and it is not blocked BY a plant. The foreign `adr.md` reaches neither the ----
    // gate's copy loop nor the merged-state conformance validation, so no doctype is
    // resolved off a colon-less stem.
    assert!(
        !both.contains("unknown-type"),
        "a foreign `adr.md` under merged/docs/ is not a doctype: the copy loop asks \
         `split_once(':')`, the predicate `plan_promotions` and `staged_doc_id` already ask, \
         so nothing resolves the stem `adr` to a schema; got:\n{both}",
    );

    // ---- The stale jigc body is gone: the rebuild is still clean for what jigc wrote. ----
    assert!(
        !docs.join(STALE_BODY).exists(),
        "`{STALE_BODY}` is a name `staged_doc_id` recognises, so the selective clear takes it",
    );
    assert!(
        grep_jigc(repo, STALE_MARKER).is_empty(),
        "the same instrument that saw the stale body before finds it nowhere after",
    );

    // ---- Every foreign plant is still on disk, byte-intact, where it was planted. ----
    for (name, bytes) in PLANTS {
        assert_eq!(
            fs::read_to_string(docs.join(name)).unwrap_or_else(|err| panic!(
                "`merged/docs/{name}` must survive the rebuild — jigc did not write it, so \
                 `materialize` does not remove it ({err})"
            )),
            *bytes,
            "`merged/docs/{name}` survives BYTE-intact",
        );
        let marker = bytes
            .split_whitespace()
            .find(|w| w.contains("PLANT"))
            .expect("each plant carries a marker");
        assert!(
            !grep_jigc(repo, marker).is_empty(),
            "the control still finds `{name}`'s marker {marker} after the blocked finalize",
        );
    }

    // ---- The rebuild itself still happened: the area set's own bodies are there. ----
    for body in ["adr:broken-policy.md", "adr:code-policy.md"] {
        assert!(
            docs.join(body).is_file(),
            "`materialize` still writes every body the current area set produces ({body})",
        );
    }
}

/// **The third consumer, driven** — the promote sweep. `engine::finalize::plan_promotions`
/// reads the same `merged/docs/` the gate does, and skips a colon-less stem at its own
/// `split_once(':')`; the spike's claim is that a foreign file left there reaches it no more
/// than it reaches the gate. Read at the line that is a `continue`; driven, it is the
/// committed tree — a **landed** boundary over the identical plants promotes exactly the two
/// bodies the area set produced, and nothing derived from a name jigc never wrote.
///
/// This arm is a control, not the fix's red: the promote sweep was already correct. It is
/// here because the spike's claim is about what a surviving foreign file reaches, and a
/// claim the build rests on becomes a standing test rather than a sentence in a record.
#[test]
fn a_landed_boundary_promotes_only_what_the_area_set_produced() {
    let repo_dir = TempDir::new("landed-repo");
    let home_dir = TempDir::new("landed-home");
    let repo = repo_dir.path();
    let home = home_dir.path();
    init_repo(repo);

    expect_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Doc area", "Code area"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", MILESTONE, intent]),
            "milestone add-task",
        );
    }
    // Both conformant this time — the boundary lands, so the promote sweep actually runs.
    stage_doc(repo, "doc-area", "adr:doc-policy", &adr_plain("Doc policy"));
    stage_doc(
        repo,
        "code-area",
        "adr:code-policy",
        &adr_plain("Code policy"),
    );
    expect_ok(
        &run_milestone(repo, home, &["provision", MILESTONE]),
        "milestone provision",
    );

    let docs = merged_docs(repo);
    fs::create_dir_all(&docs).expect("mk merged/docs/");
    for (name, bytes) in PLANTS {
        fs::write(docs.join(name), bytes).expect("write plant");
    }

    let out = run_milestone(repo, home, &["finalize", MILESTONE]);
    expect_ok(&out, "milestone finalize over a merged area holding plants");

    // The committed decisions tree is exactly the area set's two bodies. A stem resolved off
    // a colon-less name would land here as a slug-less `docs/decisions/.md`, or as a body
    // under whatever the plant's stem named.
    let mut promoted: Vec<String> =
        git_ok(repo, &["ls-tree", "--name-only", "HEAD", "docs/decisions/"])
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_owned)
            .collect();
    promoted.sort();
    assert_eq!(
        promoted,
        vec![
            "docs/decisions/code-policy.md".to_string(),
            "docs/decisions/doc-policy.md".to_string(),
        ],
        "the promote sweep lands the area set's two bodies and nothing else — a foreign \
         `merged/docs/adr.md` is not an address, so it is promoted nowhere",
    );
    // …and no plant's bytes reached the committed tree under any path at all.
    for (name, bytes) in PLANTS {
        let marker = bytes
            .split_whitespace()
            .find(|w| w.contains("PLANT"))
            .expect("each plant carries a marker");
        let tracked = Command::new("git")
            .args(["grep", "-l", marker, "HEAD"])
            .current_dir(repo)
            .output()
            .expect("run git grep");
        assert!(
            tracked.stdout.is_empty(),
            "`{name}`'s bytes must not be in the committed tree; got:\n{}",
            String::from_utf8_lossy(&tracked.stdout),
        );
    }
}
