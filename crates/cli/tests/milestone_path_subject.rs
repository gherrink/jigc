//! M46 Increment 2 / T1 — the milestone boundary's worktree subject becomes **the on-disk
//! path**, gated on the shipped [`cli::milestone::LeftoverVerdict::OwnWorktree`] verdict
//! (`implementation/roadmap.md` → Milestone 46, Increment 2, deliverable (a);
//! `design/finalize.md` → `fan-out` finalize / the landing manifest).
//!
//! `provisioned_worktrees` used to intersect the milestone's task list with the repo's
//! **registered** worktree set — the same wrong subject M48 retired at the three destroying
//! doors, which survived here. The ordinary trigger is a `cp -R` or `mv` of the whole repo
//! (how every RC trial corpus is made): the copy's worktree admin records name the
//! **source's** paths, so nothing under the copy's own `.jigc/worktrees/` is registered
//! there. The boundary then saw *no worktrees at all* exactly where the sub-agents' live
//! work sat, and landed a docs-only commit at exit 0 while the landing manifest printed
//! `no worktree provisioned` over directories holding the work.
//!
//! The subject is now the path, classified once per sub-task:
//!
//!   * **live** — [`cli::milestone::LeftoverVerdict::OwnWorktree`] on an *existing* path:
//!     git reads its index, so the boundary reads, credits and commits it, registered or
//!     not;
//!   * **unreadable** — a path that exists but git cannot read as a worktree of its own
//!     (`Unverifiable` after a `mv`, `NoOwnLinkage` for a plain directory): **named** in
//!     the manifest, never read for code and never committed;
//!   * **absent** — nothing on disk: the genuinely never-provisioned sub-task, whose
//!     manifest words are unchanged.
//!
//! Membership is the *verdict*, never mere existence, and that is the hazard arm (b)
//! exists for: `git -C <plain-dir-under-.jigc/worktrees/> diff --cached --name-only`
//! prints the **enclosing** repo's staged set at exit 0, so a mere-existence subject
//! would attribute the main checkout's staged files to a sub-task and commit them.
//!
//! Four arms, all driving the REAL binary — the landed commit and the emitted manifest
//! bytes are the contract:
//!
//!   (a) a live-but-**unregistered** worktree (the `cp -R` shape) lands its staged code and
//!       is credited in the manifest;
//!   (b) the hazard cell — a plain directory at a sub-task's worktree path while the main
//!       checkout holds a staged file — is neither attributed nor committed;
//!   (c) the `Unverifiable` (`mv`) cell — the manifest **names** the unreadable worktree
//!       rather than claiming the sub-task contributed nothing, on the agent text and on
//!       the `--format json` envelope;
//!   (d) the negative control: a genuinely **absent** worktree path still reads
//!       `no worktree provisioned` (the `path.exists()` pre-check `probe_leftover` keeps —
//!       `classify_leftover` on a missing directory answers `Unverifiable`, which would
//!       flip the shipped never-provisioned fixtures). The two shipped fixtures that pin
//!       those exact bytes live in `crates/cli/tests/milestone.rs` and
//!       `crates/cli/tests/milestone_zero_contribution.rs`, unchanged by this wave.
//!
//! **The loss narration is deliberately NOT widened here.** `discarded_work` names what the
//! teardown destroys, and `remove_worktrees` removes **registered** worktrees only — so a
//! live-but-unregistered worktree the teardown leaves standing must not be narrated as
//! lost. That would be a law-1 lie in the other direction (`design/surface-contract.md`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-path-subject-{tag}-{}-{:?}",
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
        // Linked worktrees inside the tree are ordinary directories to `remove_dir_all`;
        // a moved-away source is simply already gone.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Run `git <args>` in `cwd`, asserting success, returning stdout.
fn git(cwd: &Path, args: &[&str]) -> String {
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

/// Initialize a real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    git(repo, &["config", "commit.gpgsign", "false"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(repo);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc milestone <args>`.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut argv = vec!["milestone"];
    argv.extend_from_slice(args);
    run_jigc(repo, home, &argv)
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// `cp -R <from> <to>` — the ordinary way a corpus copy is made, reproduced verbatim
/// rather than simulated (the `provision_leftover_guard` fixture idiom).
fn copy_repo(from: &Path, to: &Path) {
    let out = Command::new("cp")
        .arg("-R")
        .arg(from)
        .arg(to)
        .output()
        .expect("run cp");
    assert!(
        out.status.success(),
        "cp -R {from:?} {to:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Mint `milestone:cache-rework` with one sub-task per intent, in `repo`.
fn mint_milestone(repo: &Path, home: &Path, intents: &[&str]) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`jigc milestone create`",
    );
    for intent in intents {
        assert_ok(
            &run_milestone(repo, home, &["add-task", "cache-rework", intent]),
            "`jigc milestone add-task`",
        );
    }
}

/// A plain, ref-free ADR body.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area — the
/// managed docs a fanned-out sub-agent authors (the `milestone_zero_contribution` idiom).
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

/// Write + `git add` a code file **in** a sub-task's fan-out worktree.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// The paths the landed boundary commit actually carries — `git diff --name-only HEAD~1 HEAD`,
/// the read an agent does when it wants to know what landed.
fn landed_paths(repo: &Path) -> Vec<String> {
    git(repo, &["diff", "--name-only", "HEAD~1", "HEAD"])
        .lines()
        .map(str::to_owned)
        .collect()
}

/// A source repo carrying a provisioned milestone, plus the `cp -R` copy it is run from —
/// the shape whose worktrees are **live but unregistered** in the copy.
struct Copied {
    /// Kept alive for the `OwnWorktree` shape (the copy's `.git` files point into it).
    _source: TempDir,
    _copies: TempDir,
    home: TempDir,
    source: PathBuf,
    copy: PathBuf,
}

/// Mint + provision `milestone:cache-rework` with `intents` in a fresh source repo, then
/// `cp -R` the whole repo. In the copy, `git rev-parse --show-toplevel` inside each
/// `.jigc/worktrees/<id>` prints **that** path (`OwnWorktree`) while `git worktree list`
/// in the copy names only the **source's** paths — live, and unregistered here.
fn provisioned_copy(tag: &str, intents: &[&str]) -> Copied {
    let source_root = TempDir::new(tag);
    let source = source_root.path().join("repo");
    fs::create_dir_all(&source).expect("mk source repo dir");
    init_repo(&source);
    let home = TempDir::new("home");
    mint_milestone(&source, home.path(), intents);
    assert_ok(
        &run_milestone(&source, home.path(), &["provision", "cache-rework"]),
        "`jigc milestone provision` in the source",
    );
    let copies = TempDir::new("copies");
    let copy = copies.path().join("copy");
    copy_repo(&source, &copy);
    Copied {
        _source: source_root,
        _copies: copies,
        home,
        source,
        copy,
    }
}

// ---------------------------------------------------------------------------------------
// (a) A live-but-unregistered worktree lands its code and is credited.
// ---------------------------------------------------------------------------------------

#[test]
fn a_live_but_unregistered_worktree_lands_its_staged_code_and_is_credited() {
    let f = provisioned_copy("live", &["Area low"]);
    stage_worktree_code(&f.copy, "area-low", "src/low.rs", "pub fn low() {}\n");

    // The copy registers nothing under its OWN worktrees root — the whole premise.
    let listed = git(&f.copy, &["worktree", "list", "--porcelain"]);
    let own_root = f
        .copy
        .canonicalize()
        .expect("canonicalize the copy")
        .join(".jigc")
        .join("worktrees");
    assert!(
        !listed.contains(&own_root.display().to_string()),
        "the fixture's premise is that the copy registers NO worktree of its own; got:\n{listed}",
    );

    let landed = run_milestone(&f.copy, f.home.path(), &["finalize", "cache-rework"]);
    assert_ok(
        &landed,
        "`jigc milestone finalize` over a live-but-unregistered worktree",
    );

    let paths = landed_paths(&f.copy);
    assert!(
        paths.iter().any(|p| p == "src/low.rs"),
        "the sub-agent's staged code must be IN the landed commit; got: {paths:?}",
    );

    let stdout = String::from_utf8_lossy(&landed.stdout).into_owned();
    assert!(
        stdout.contains("sub-tasks: area-low: 1 code file"),
        "the manifest must credit the sub-task's code; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("no worktree provisioned"),
        "a live worktree must NOT be reported as never provisioned; got:\n{stdout}",
    );
}

// ---------------------------------------------------------------------------------------
// (b) The hazard cell — a plain directory at a worktree path is neither attributed nor
//     committed, even while the main checkout holds a staged file.
// ---------------------------------------------------------------------------------------

#[test]
fn a_plain_directory_at_a_worktree_path_is_neither_attributed_nor_committed() {
    let f = provisioned_copy("hazard", &["Area low", "Area zed"]);
    stage_worktree_code(&f.copy, "area-low", "src/low.rs", "pub fn low() {}\n");

    // `area-zed`'s worktree becomes a PLAIN directory: `git rev-parse --show-toplevel`
    // there walks up and answers for the enclosing repo (`NoOwnLinkage`).
    let zed = f.copy.join(".jigc").join("worktrees").join("area-zed");
    fs::remove_dir_all(&zed).expect("clear the area-zed worktree");
    fs::create_dir_all(&zed).expect("mk the plain directory");
    fs::write(zed.join("leftover.txt"), "not a worktree\n").expect("plant the leftover");

    // The main checkout holds a staged file — what `git -C <plain-dir> diff --cached`
    // would hand a mere-existence subject, attributed to `area-zed` and committed.
    fs::write(f.copy.join("main-wip.txt"), "main-checkout WIP\n").expect("write main WIP");
    git(&f.copy, &["add", "main-wip.txt"]);

    let landed = run_milestone(
        &f.copy,
        f.home.path(),
        &["finalize", "cache-rework", "--carry-staged"],
    );
    assert_ok(&landed, "`jigc milestone finalize --carry-staged`");

    let paths = landed_paths(&f.copy);
    assert!(
        paths.iter().any(|p| p == "src/low.rs"),
        "the live sub-task's code must still land; got: {paths:?}",
    );
    assert!(
        !paths.iter().any(|p| p == "main-wip.txt"),
        "the MAIN checkout's staged file must never be committed as sub-task code — that is \
         what a mere-existence subject would do; got: {paths:?}",
    );
    assert!(
        !paths.iter().any(|p| p.contains("leftover.txt")),
        "the plain directory's contents must never be committed; got: {paths:?}",
    );

    let stdout = String::from_utf8_lossy(&landed.stdout).into_owned();
    assert!(
        !stdout.contains("area-zed: 1 code file") && !stdout.contains("area-zed: 1 doc, 1 code"),
        "the plain directory must not be credited with code; got:\n{stdout}",
    );
    assert!(
        stdout.contains("unreadable worktree at .jigc/worktrees/area-zed"),
        "the manifest must name the path it could not read; got:\n{stdout}",
    );
}

// ---------------------------------------------------------------------------------------
// (c) The `mv` cell — the manifest NAMES the unreadable worktree.
// ---------------------------------------------------------------------------------------

/// The `Unverifiable` shape: the provisioned copy with its **source moved away**, so each
/// `.jigc/worktrees/<id>/.git` points at an admin directory that no longer exists and
/// `git rev-parse` exits 128. One sub-task, with a staged doc so the boundary lands (and
/// therefore prints a manifest) without any readable worktree.
fn moved_away_copy(tag: &str) -> (Copied, TempDir) {
    let f = provisioned_copy(tag, &["Area low"]);
    stage_doc(
        &f.copy,
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
    );
    let attic = TempDir::new("attic");
    fs::rename(&f.source, attic.path().join("moved-source")).expect("move the source repo away");
    (f, attic)
}

#[test]
fn the_manifest_names_an_unreadable_worktree_instead_of_claiming_none_was_provisioned() {
    // --- the agent-text surface ---------------------------------------------------------
    let (f, _attic) = moved_away_copy("unverifiable-agent");
    let landed = run_milestone(&f.copy, f.home.path(), &["finalize", "cache-rework"]);
    assert_ok(
        &landed,
        "`jigc milestone finalize` over an unreadable worktree",
    );
    let stdout = String::from_utf8_lossy(&landed.stdout).into_owned();
    assert!(
        stdout.contains(
            "sub-tasks: area-low: 1 doc, unreadable worktree at .jigc/worktrees/area-low, \
             no code counted"
        ),
        "the manifest must NAME the unreadable worktree; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("no worktree provisioned"),
        "a directory git cannot read is not the same fact as no worktree at all; got:\n{stdout}",
    );

    // --- the same fact on the machine surface -------------------------------------------
    let (f, _attic) = moved_away_copy("unverifiable-json");
    let landed = run_jigc(
        &f.copy,
        f.home.path(),
        &["--format", "json", "milestone", "finalize", "cache-rework"],
    );
    assert_ok(
        &landed,
        "the unreadable-worktree finalize (`--format json`)",
    );
    let value: serde_json::Value =
        serde_json::from_slice(&landed.stdout).expect("the landed envelope is valid JSON");
    let sub = value["committed"]["sub_tasks"]
        .as_array()
        .expect("the committed envelope carries `sub_tasks`")
        .iter()
        .find(|s| s["id"] == "area-low")
        .expect("area-low is named")
        .clone();
    assert_eq!(
        sub["worktree_unreadable"], true,
        "the unreadable worktree is data, not only prose; got:\n{sub:#}",
    );
    assert_eq!(
        sub["provisioned"], false,
        "an unreadable path is no usable worktree; got:\n{sub:#}",
    );
    assert_eq!(
        sub["code_files"], 0,
        "nothing may be counted out of a path git cannot read; got:\n{sub:#}",
    );
}

// ---------------------------------------------------------------------------------------
// (d) The negative control — an ABSENT path still reads `no worktree provisioned`.
// ---------------------------------------------------------------------------------------

#[test]
fn a_genuinely_absent_worktree_path_still_reads_no_worktree_provisioned() {
    let root = TempDir::new("absent");
    let repo = root.path().join("repo");
    fs::create_dir_all(&repo).expect("mk repo dir");
    init_repo(&repo);
    let home = TempDir::new("home");
    mint_milestone(&repo, home.path(), &["Area low"]);
    stage_doc(
        &repo,
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
    );

    let landed = run_milestone(&repo, home.path(), &["finalize", "cache-rework"]);
    assert_ok(&landed, "the never-provisioned `jigc milestone finalize`");
    let stdout = String::from_utf8_lossy(&landed.stdout).into_owned();
    assert!(
        stdout.contains("sub-tasks: area-low: 1 doc, no worktree provisioned"),
        "an absent path keeps the shipped never-provisioned words; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("unreadable worktree"),
        "`classify_leftover` on a MISSING directory answers `Unverifiable` — the \
         `path.exists()` pre-check must keep that off this cell; got:\n{stdout}",
    );
}
