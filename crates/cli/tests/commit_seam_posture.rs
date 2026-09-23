//! **The commit and move seams take a typed subject and re-probe before the act**
//! (M51 Increment 2 / T4; `completions/artifacts/M51/settle-record.md` → Review
//! amendments §3: *the posture subject is typed and re-probed at the seam*).
//!
//! Increment 2's door guard adjudicates the repository posture **once**, at
//! `Cli::dispatch`, before anything is resolved. That is where the user's own state is
//! answered, and it is not where the act happens: a per-task finalize reaches
//! `cli::task::git_commit_capture` after a validate, a promote, a retire and a stage, and
//! the milestone boundary reaches its `git merge --ff-only` after a commit that ran the
//! user's hooks in a worktree sharing the same `.git`. A hook, a concurrent process or an
//! earlier phase of the same run can move HEAD in between, so the seam re-probes.
//!
//! **Why the subject is a type and not a `&Path`.** The seam cannot sniff which checkout
//! it is in: a fan-out worktree answers `git symbolic-ref -q HEAD` with exit 1 —
//! byte-identical to a user's detached HEAD (driven in `tests/repo_posture.rs`) — and the
//! throwaway worktree the squash:true combine commits in is not a registered sub-task, so
//! `repo::posture_subject`'s three legs would classify it *live* and refuse jigc's own
//! commit site. So a caller must **say** which it is, and the dedicated variant has
//! exactly one constructor, which takes a live `DedicatedWorktree` handle — neither a path
//! nor a boolean, the forgeable shapes M50's completion audit condemned in
//! `fanout_worktree_paths`' `is_dir()`.
//!
//! The arms, and what each drives:
//!
//!   * (a) a **live** subject whose HEAD then moves — a detach, and separately a branch
//!     switch — makes the seam refuse and commit nothing. Driven in-process against the
//!     real `cli::task::git_commit`, because the interleaving is one no race can be made
//!     to stage through the binary: the subject must be built, then HEAD moved, then the
//!     seam entered, deterministically, in that order.
//!   * (b) a **dedicated** subject commits clean on its detached HEAD — the full fan-out
//!     `milestone finalize` lands unchanged through the real binary at **both**
//!     `squash: true` and `squash: false`, and names no `repo.head-detached`.
//!   * (c) `move_doc` under a live `MERGE_HEAD` refuses, leaving the file unmoved and the
//!     index unchanged — the mover's class, *operation in progress* only.
//!   * (d) the live-checkout `merge --ff-only` refuses when the checkout moved **after**
//!     the boundary's own commit, driven through the real binary by a `pre-commit` hook
//!     that fires only inside the combine worktree and moves the main checkout's HEAD.
//!   * (e) a stated arm: no public constructor of the dedicated variant takes a path or a
//!     boolean.
//!   * (f) **the boundary asks every worktree it commits FROM, not only the checkout the
//!     command was run in** — the whole `GitState` axis built *inside* a provisioned
//!     sub-task worktree, at both commit models, plus the preview surface that forecasts
//!     the door (M53 post-review fix, 2026-09-22; the M53 per-axis review, axis 2
//!     `DEFECT 1`).
//!   * (g) **every path those surfaces print is spelled against the workbench's own root**,
//!     driven from *inside* a second linked worktree — the checkout the caller stands in is
//!     not the one `.jigc/` hangs off, and rendering against the former leaks a host path
//!     into the message, the pinned `(code, target)` key and the route (the independent
//!     review of `986d5e0a`, MEDIUM 1).
//!   * (h) a stated arm: `cli/src/milestone.rs` renders **no** path against the cwd's repo
//!     root — the class (g) drives two members of, read off the source.
//!   * (i) **one code, one declared envelope arm** at the door and at the preview the door
//!     owns, plus the contract statement that names its target form (the independent review
//!     of `986d5e0a`, MEDIUM 2).
//!
//! (f)'s refusing cells run through `git_state::overlay_worktree`, which **attaches** a
//! branch first, so each probes a subject with one breach; `jigc milestone provision`
//! detaches every fan-out worktree, so production presents **two** and probe order decides.
//! The `Overlay::Detached` cell is the one that runs the production topology (the review's
//! LOW 2).
//!
//! # Why (f) lives here and not in `posture_door_axis.rs`
//!
//! That suite crosses `BEHALF_DOORS` × `GitState::ALL` over the checkout each door is
//! *run in* — 204 cells sharing one cheap fixture per state. (f)'s subject is a different
//! checkout: the fan-out worktree the milestone boundary commits **from**, which needs a
//! provisioned milestone per cell and exists at exactly one door. Folding it in would make
//! every one of those 204 cells pay for a fixture 203 of them cannot use.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop (the project's no-tempfile pattern).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-commit-seam-{tag}-{}-{:?}",
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

fn git(dir: &Path, args: &[&str]) -> std::process::Output {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {dir:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    out
}

fn git_stdout(dir: &Path, args: &[&str]) -> String {
    String::from_utf8(git(dir, args).stdout)
        .expect("utf-8 git stdout")
        .trim()
        .to_string()
}

/// A real git repo with one commit, an identity, and signing off.
fn init_repo(root: &Path) -> String {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    git(root, &["config", "commit.gpgsign", "false"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    git_stdout(root, &["rev-parse", "HEAD"])
}

fn commit_count(repo: &Path) -> u32 {
    git_stdout(repo, &["rev-list", "--count", "HEAD"])
        .parse()
        .expect("count parses")
}

/// Stage one new file and write a commit message file; returns the message path.
fn stage_a_change(repo: &Path, name: &str) -> PathBuf {
    fs::write(repo.join(name), "body\n").expect("write file");
    git(repo, &["add", name]);
    let msg = repo.join("commit-message.txt");
    fs::write(&msg, "test: a commit\n").expect("write msg");
    msg
}

// ---------------------------------------------------------------------------
// (a) A live subject whose checkout moved under it.
// ---------------------------------------------------------------------------

/// The control, so the two refusals below are about the *move* and not about the seam
/// having stopped committing: a live subject whose checkout did **not** move commits.
#[test]
fn a_live_subject_on_an_unmoved_checkout_still_commits() {
    let repo = TempDir::new("live-control");
    init_repo(repo.path());
    let msg = stage_a_change(repo.path(), "a.txt");

    let subject = cli::repo::SeamSubject::live(repo.path());
    let before = commit_count(repo.path());
    cli::task::git_commit(&subject, &msg).expect("an unmoved live checkout still commits");
    assert_eq!(
        commit_count(repo.path()),
        before + 1,
        "the control must land exactly one commit",
    );
}

/// The interleaving a race cannot be made to stage: the subject records the ref this act
/// was decided for, HEAD then **detaches**, and the seam refuses — because a commit made
/// there would belong to no branch and the next checkout would leave it unreachable.
#[test]
fn a_live_subject_refuses_the_seam_when_head_detached_after_the_act_was_decided() {
    let repo = TempDir::new("live-detach");
    let head = init_repo(repo.path());
    let msg = stage_a_change(repo.path(), "a.txt");

    let subject = cli::repo::SeamSubject::live(repo.path());
    // ...and now HEAD moves, after the act was decided and before the seam runs.
    git(repo.path(), &["checkout", "-q", "--detach", &head]);

    let before = commit_count(repo.path());
    let err = cli::task::git_commit(&subject, &msg)
        .expect_err("a live subject whose HEAD detached under it must refuse");
    let rendered = format!("{err:#}");
    assert!(
        rendered.contains("repo.head-detached"),
        "the refusal must carry the posture family's own identity; got:\n{rendered}",
    );
    assert!(
        rendered.contains("git switch"),
        "the refusal must name the git command that resolves the state; got:\n{rendered}",
    );
    assert_eq!(
        commit_count(repo.path()),
        before,
        "a refused seam must commit nothing",
    );
}

/// The member the family cannot express, and the reason the subject records the ref
/// **value** rather than *is HEAD attached*: HEAD is attached the whole time, to a
/// different branch, and a commit landing there lands on a branch nobody asked for.
#[test]
fn a_live_subject_refuses_the_seam_when_the_branch_switched_after_the_act_was_decided() {
    let repo = TempDir::new("live-switch");
    init_repo(repo.path());
    let msg = stage_a_change(repo.path(), "a.txt");
    let started_on = git_stdout(repo.path(), &["symbolic-ref", "HEAD"]);

    let subject = cli::repo::SeamSubject::live(repo.path());
    git(repo.path(), &["switch", "-q", "-c", "other"]);

    let before = commit_count(repo.path());
    let err = cli::task::git_commit(&subject, &msg)
        .expect_err("a live subject whose branch switched under it must refuse");
    let rendered = format!("{err:#}");
    assert!(
        rendered.contains(&started_on) && rendered.contains("refs/heads/other"),
        "the refusal must name the ref the act was decided for AND the one that is there \
         now; got:\n{rendered}",
    );
    assert!(
        rendered.contains("git switch"),
        "the refusal must name the git command that puts it back; got:\n{rendered}",
    );
    assert_eq!(
        commit_count(repo.path()),
        before,
        "a refused seam must commit nothing",
    );
    assert!(
        git_stdout(repo.path(), &["status", "--porcelain"]).contains("a.txt"),
        "the staged change must survive the refusal untouched",
    );
}

// ---------------------------------------------------------------------------
// (c) The move seam.
// ---------------------------------------------------------------------------

/// Seed a committed managed doc and the workbench root `move_doc` re-keys against.
fn seed_movable_doc(repo: &Path) -> PathBuf {
    let docs = repo.join("docs").join("decisions");
    fs::create_dir_all(&docs).expect("mk docs dir");
    fs::write(docs.join("old.md"), "# Old\n").expect("write doc");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "seed the managed doc"]);
    let jigc_root = repo.join(".jigc");
    fs::create_dir_all(jigc_root.join("state")).expect("mk jigc state");
    jigc_root
}

/// The control: with no operation in progress the one move primitive still moves.
#[test]
fn the_move_seam_still_moves_when_no_operation_is_in_progress() {
    let repo = TempDir::new("move-control");
    init_repo(repo.path());
    let jigc_root = seed_movable_doc(repo.path());

    cli::relocate::move_doc(
        repo.path(),
        &jigc_root,
        "docs/decisions/old.md",
        "docs/decisions/new.md",
        "deadbeef",
    )
    .expect("a clean checkout still moves");
    assert!(
        repo.path().join("docs/decisions/new.md").exists()
            && !repo.path().join("docs/decisions/old.md").exists(),
        "the control must actually move the file",
    );
}

/// A mover refuses **operation in progress** and nothing else: the `git mv` batch is
/// re-probed immediately before it runs, so a merge the user started and has not concluded
/// keeps jigc from moving a tracked file out from under it — and nothing moves.
#[test]
fn the_move_seam_refuses_under_a_live_merge_head_and_moves_nothing() {
    let repo = TempDir::new("move-merge");
    init_repo(repo.path());
    let jigc_root = seed_movable_doc(repo.path());

    // A real, un-concluded merge: two branches touching the same line.
    let base = git_stdout(repo.path(), &["rev-parse", "HEAD"]);
    fs::write(repo.path().join("conflict.txt"), "ours\n").expect("write ours");
    git(repo.path(), &["add", "conflict.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "ours"]);
    git(repo.path(), &["checkout", "-q", "-b", "theirs", &base]);
    fs::write(repo.path().join("conflict.txt"), "theirs\n").expect("write theirs");
    git(repo.path(), &["add", "conflict.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "theirs"]);
    git(repo.path(), &["checkout", "-q", "-"]);
    // A conflicting merge leaves MERGE_HEAD in place and exits non-zero — expected.
    let _ = Command::new("git")
        .args(["merge", "theirs"])
        .current_dir(repo.path())
        .output()
        .expect("run git merge");
    assert!(
        repo.path().join(".git").join("MERGE_HEAD").exists(),
        "the fixture must leave a live MERGE_HEAD for the seam to find",
    );

    let index_before = git_stdout(repo.path(), &["ls-files", "--stage"]);
    let err = cli::relocate::move_doc(
        repo.path(),
        &jigc_root,
        "docs/decisions/old.md",
        "docs/decisions/new.md",
        "deadbeef",
    )
    .expect_err("a mover must refuse an operation in progress");
    let rendered = format!("{err:#}");
    assert!(
        rendered.contains("repo.operation-in-progress"),
        "the mover's refusal must carry the family's identity; got:\n{rendered}",
    );
    assert!(
        rendered.contains("git merge --abort"),
        "the refusal must name the command that concludes THIS operation; got:\n{rendered}",
    );
    assert!(
        repo.path().join("docs/decisions/old.md").exists()
            && !repo.path().join("docs/decisions/new.md").exists(),
        "a refused move must leave the file where it was",
    );
    assert_eq!(
        git_stdout(repo.path(), &["ls-files", "--stage"]),
        index_before,
        "a refused move must leave the index byte-identical",
    );
}

// ---------------------------------------------------------------------------
// (b) + (d) The fan-out boundary, through the real binary.
// ---------------------------------------------------------------------------

fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n\
         ## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n\
         ## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a doc body + its provenance bit in a sub-task's working area.
fn stage_doc(repo: &Path, sub: &str, address: &str, body: &str, provenance: &str) {
    let docs = repo.join(".jigc").join("tasks").join(sub).join("docs");
    fs::create_dir_all(&docs).expect("mk docs/");
    fs::write(docs.join(format!("{address}.md")), body).expect("write staged body");
    let manifest = docs.join("provenance.json");
    let mut record: serde_json::Value = match fs::read_to_string(&manifest) {
        Ok(s) => serde_json::from_str(&s).expect("provenance manifest parses"),
        Err(_) => serde_json::json!({ "docs": {} }),
    };
    record["docs"][address] = serde_json::Value::String(provenance.to_string());
    fs::write(
        &manifest,
        serde_json::to_string_pretty(&record).expect("serialize manifest"),
    )
    .expect("write provenance manifest");
}

fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body, "created");
}

fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    fs::create_dir_all(p.parent().expect("code parent")).expect("mkdir worktree code parent");
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// Mint a two-sub-task milestone, stage a disjoint ADR + commit doc per sub-task, and
/// provision the N base-pin worktrees with disjoint staged code in each — the fan-out
/// shape both commit models commit from.
fn setup_fan_out(repo: &Path, home: &Path, squash_false: bool) {
    // The project cascade layer, unconditionally — `jigc milestone`'s door-top
    // precondition (M52 Inc 8 / T1). The `squash:false` arm used to create it as a side
    // effect of writing its knob file; its `squash:true` twin had none at all.
    crate::support::mint_project_layer(repo);
    let config = repo.join(".jigc").join("config");
    if squash_false {
        fs::write(
            config.join("manifest.yaml"),
            "scalar:\n  finalize.fan-out.squash: false\n",
        )
        .expect("write manifest");
    }
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low"] {
        assert!(
            run_milestone(repo, home, &["add-task", "cache-rework", intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
    for (sub, slug, title) in [
        ("area-low", "low-policy", "Low policy"),
        ("area-zed", "zed-policy", "Zed policy"),
    ] {
        stage_doc(
            repo,
            sub,
            &format!("adr:{slug}"),
            &adr_plain(title),
            "edited-from-base",
        );
        stage_subtask_commit(repo, sub, &format!("rework the {sub} cache path"));
    }
    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
}

/// (b) The typed exemption, driven end-to-end: every commit the boundary makes runs in a
/// worktree jigc detached itself, so a subject the seam had to *sniff* would answer
/// `repo.head-detached` and refuse jigc's own commit site. Both commit models land.
fn fan_out_lands_unchanged(tag: &str, squash_false: bool) {
    let repo = TempDir::new(tag);
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_fan_out(repo.path(), home.path(), squash_false);

    let before = commit_count(repo.path());
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8_lossy(&finalized.stderr).to_string();
    assert!(
        finalized.status.success(),
        "the fan-out boundary must land through a dedicated-worktree subject; got {:?}\n\
         stderr:\n{stderr}",
        finalized.status,
    );
    assert!(
        !stderr.contains("repo.head-detached"),
        "a dedicated worktree is exempt from the detached member — the seam must not \
         refuse jigc's own commit site; stderr:\n{stderr}",
    );
    assert!(
        commit_count(repo.path()) > before,
        "the boundary must land its commits",
    );
    let tree = git_stdout(repo.path(), &["ls-tree", "-r", "--name-only", "HEAD"]);
    assert!(
        tree.contains("src/low.rs") && tree.contains("src/zed.rs"),
        "both worktrees' code must ride the landed boundary; got:\n{tree}",
    );
}

#[test]
fn a_dedicated_worktree_subject_commits_clean_at_squash_true() {
    fan_out_lands_unchanged("fanout-squash-true", false);
}

#[test]
fn a_dedicated_worktree_subject_commits_clean_at_squash_false() {
    fan_out_lands_unchanged("fanout-squash-false", true);
}

/// (d) The live-checkout `merge --ff-only`. The boundary's own commit runs the user's
/// hooks in a worktree sharing this `.git`, so the ref the fast-forward was decided for
/// can move **while that commit is being made**. A `pre-commit` hook that fires only
/// inside the combine worktree moves the main checkout's HEAD; the fast-forward must
/// refuse rather than advance a branch nobody asked for.
#[test]
fn the_live_fast_forward_refuses_when_the_checkout_moved_after_the_boundary_commit() {
    let repo = TempDir::new("ff-drift");
    init_repo(repo.path());
    let home = TempDir::new("home");
    // A second branch for the hook to move HEAD onto, at the same commit — so a seam that
    // did NOT re-probe would fast-forward it cleanly and the run would exit 0.
    git(repo.path(), &["branch", "other"]);
    let started_on = git_stdout(repo.path(), &["symbolic-ref", "HEAD"]);

    let hooks = repo.path().join(".git").join("hooks");
    fs::create_dir_all(&hooks).expect("mk hooks");
    let hook = hooks.join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\ntop=$(git rev-parse --show-toplevel)\ncase \"$top\" in\n  \
             *\"/.jigc/worktrees/.combine-\"*)\n    unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE\n    \
             git -C {repo} symbolic-ref HEAD refs/heads/other\n    ;;\nesac\nexit 0\n",
            repo = repo.path().display(),
        ),
    )
    .expect("write hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }

    setup_fan_out(repo.path(), home.path(), false);
    let before_started_on = git_stdout(repo.path(), &["rev-parse", &started_on]);
    let before_other = git_stdout(repo.path(), &["rev-parse", "refs/heads/other"]);

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8_lossy(&finalized.stderr).to_string();
    assert!(
        !finalized.status.success(),
        "a checkout that moved after the boundary's own commit must refuse the \
         fast-forward; got {:?}\nstderr:\n{stderr}",
        finalized.status,
    );
    assert!(
        stderr.contains(&started_on) && stderr.contains("refs/heads/other"),
        "the refusal must name the ref the fast-forward was decided for and the one that \
         is there now; stderr:\n{stderr}",
    );
    assert_eq!(
        git_stdout(repo.path(), &["rev-parse", &started_on]),
        before_started_on,
        "no ref may advance when the fast-forward refuses",
    );
    assert_eq!(
        git_stdout(repo.path(), &["rev-parse", "refs/heads/other"]),
        before_other,
        "the ref the checkout drifted onto must not receive the boundary's commit either",
    );
}

// ---------------------------------------------------------------------------
// (e) The stated arm: the dedicated variant is not constructible from a shape.
// ---------------------------------------------------------------------------

/// **No public constructor of the dedicated variant takes a path or a boolean.**
///
/// This is a claim about what the *source* offers, and it cannot be written as a call —
/// the whole point is that the call does not exist. So it is read off `repo.rs`: every
/// function that constructs the dedicated discriminant must take a
/// `&DedicatedWorktree` — a handle whose own constructor is private to `task.rs` and
/// whose fields are private — and must take neither a `&Path` nor a `bool`. A path is a
/// claim about *shape* where the question is about *provenance*, and a boolean is a claim
/// a caller simply asserts; both are the class M50's completion audit condemned in
/// `fanout_worktree_paths`' `is_dir()`.
#[test]
fn no_public_constructor_of_the_dedicated_variant_takes_a_path_or_a_boolean() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("repo.rs"),
    )
    .expect("read repo.rs");
    let lines: Vec<&str> = source.lines().collect();

    // The discriminant itself is private, so the variant cannot be named from outside.
    assert!(
        lines.iter().any(|l| l.trim_start() == "enum Checkout {"),
        "the checkout discriminant must be a PRIVATE enum — a `pub enum` would let any \
         caller name the dedicated variant directly",
    );

    // Every site that constructs the dedicated discriminant, and the `fn` that encloses it.
    let mut constructors = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        // The CONSTRUCTION, not a mention: `PostureSubject(Checkout::Dedicated)` is the
        // only way the discriminant is built, so matching a bare `Checkout::Dedicated`
        // would also catch the exemption rule's `matches!` arm, which constructs nothing.
        if !line.contains("PostureSubject(Checkout::Dedicated)")
            || line.trim_start().starts_with("//")
        {
            continue;
        }
        let signature = lines[..i]
            .iter()
            .rev()
            .find(|l| l.trim_start().contains("fn "))
            .unwrap_or_else(|| panic!("no enclosing `fn` for the construction at line {}", i + 1));
        constructors.push(signature.trim().to_string());
    }
    assert!(
        !constructors.is_empty(),
        "the dedicated discriminant must be constructed somewhere — this arm is vacuous \
         otherwise",
    );
    for signature in &constructors {
        // `posture_subject` is the door-side classifier: it takes a path by design and
        // answers from three legs INCLUDING the milestone registry, never from the path's
        // shape alone. The seam's constructor is the one this arm is about.
        if signature.contains("fn posture_subject") {
            continue;
        }
        assert!(
            signature.contains("&crate::task::DedicatedWorktree")
                || signature.contains("&DedicatedWorktree"),
            "a seam constructor of the dedicated variant must take a live worktree \
             handle; got `{signature}`",
        );
        assert!(
            !signature.contains("&Path") && !signature.contains("bool"),
            "a seam constructor of the dedicated variant must take neither a path nor a \
             boolean — both are forgeable inside this crate; got `{signature}`",
        );
    }
}

// ---------------------------------------------------------------------------
// (f) The worktrees the boundary commits FROM.
// ---------------------------------------------------------------------------

use crate::support::git_state::{self, GitState};

/// A one-sub-task fan-out, provisioned and (optionally) driven into a git state **before**
/// the sub-task's own code is staged.
///
/// Leaner than [`setup_fan_out`] on purpose: this arm builds a fixture **per cell** over
/// the whole `GitState` axis at both commit models, and the two-sub-task shape would pay
/// for a second worktree no cell reads. The staged doc + commit doc are kept, because the
/// `squash: false` model renders each code-carrying sub-task's authored `commit:<sub>` doc
/// and refuses the boundary without one.
///
/// The state is entered **after** `provision` and **before** the code is staged:
/// `git_state::overlay_worktree` refuses a dirty tree (a rebase cannot begin over one), and
/// the sub-task's own staged work is what the boundary would otherwise swallow the
/// operation alongside.
/// **How a cell enters its git state, and what it leaves HEAD as.**
///
/// The third variant is the one the independent review of `986d5e0a` (LOW 2) asked for:
/// `git_state::overlay_worktree` attaches a branch before driving, and every refusing cell
/// of the axis therefore probes a subject with **one** breach, while a production fan-out
/// worktree is `--detach`ed and answers with **two** — `OperationInProgress` and
/// `HeadDetached` — where probe order plus the `Dedicated` exemption decide which the door
/// refuses with.
enum Overlay {
    /// No operation at all — the zero-false-fire control.
    Nothing,
    /// A branch attached first, which is what `drive`'s branch-named constructions need.
    Attached(GitState),
    /// HEAD left exactly as `jigc milestone provision` created it.
    Detached(GitState),
}

fn setup_one_sub_task(repo: &Path, home: &Path, squash_false: bool, overlay: Overlay) {
    crate::support::mint_project_layer(repo);
    if squash_false {
        fs::write(
            repo.join(".jigc").join("config").join("manifest.yaml"),
            "scalar:\n  finalize.fan-out.squash: false\n",
        )
        .expect("write manifest");
    }
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    assert!(
        run_milestone(repo, home, &["add-task", "cache-rework", "Area low"])
            .status
            .success(),
        "add-task must exit 0",
    );
    stage_doc(
        repo,
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
        "edited-from-base",
    );
    stage_subtask_commit(repo, "area-low", "rework the area-low cache path");
    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    let worktree = repo.join(".jigc").join("worktrees").join("area-low");
    match overlay {
        Overlay::Nothing => {}
        Overlay::Attached(state) => {
            git_state::overlay_worktree(&worktree, home, state)
                .expect("the axis skips the states a worktree cannot hold");
        }
        Overlay::Detached(state) => {
            git_state::overlay_worktree_detached(&worktree, home, state)
                .expect("the detached cell drives only a state buildable with no branch");
        }
    }
    stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
}

/// Every marker the git-state builder knows about that is present in `git_dir`, paired
/// with its bytes where it is a file — the *whole* of what an un-concluded operation left
/// behind, read back so a cell can compare it across the door.
///
/// The bytes matter and their presence does not: the M52 baseline's squash-merge loss was
/// an authored message **destroyed**, and a cell that only checked `SQUASH_MSG` existed
/// would pass over a truncated one.
fn operation_residue(git_dir: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    git_state::MARKER_UNIVERSE
        .iter()
        .filter(|marker| git_dir.join(marker).exists())
        .map(|marker| ((*marker).to_string(), fs::read(git_dir.join(marker)).ok()))
        .collect()
}

/// Every commit this repository holds, over **all** refs — so a boundary that landed on a
/// branch nobody looked at still moves this number.
fn all_ref_commit_count(repo: &Path) -> String {
    git_stdout(repo, &["rev-list", "--all", "--count"])
}

fn run_task(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("task")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// One cell of (f): `state`, built inside the provisioned sub-task worktree, at one commit
/// model.
///
/// A **refusing** cell — every state whose `GitState::in_progress` names an operation —
/// claims five things: a non-zero exit, `repo.operation-in-progress` naming *that*
/// operation's own noun, a route aimed at the worktree with `git -C`, the operation's
/// residue **byte-identical** across the door, and nothing landed (HEAD, the all-ref commit
/// count and the worktree itself unmoved).
///
/// A **proceeding** cell is `GitState::Detached`, and it is the control that keeps the fix
/// from being *refuse whenever a worktree looks unusual*: jigc provisions every fan-out
/// worktree `--detach`, so a boundary that refused there would refuse its own provisioning.
/// It must still land.
fn fan_out_cell(state: GitState, squash_false: bool) {
    let tag = format!(
        "fanout-{}-{}",
        state.name(),
        if squash_false { "chain" } else { "squash" },
    );
    let repo = TempDir::new(&tag);
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_one_sub_task(
        repo.path(),
        home.path(),
        squash_false,
        Overlay::Attached(state),
    );

    let worktree = repo.path().join(".jigc").join("worktrees").join("area-low");
    let git_dir = PathBuf::from(git_stdout(&worktree, &["rev-parse", "--absolute-git-dir"]));
    let residue = operation_residue(&git_dir);
    let head = git_stdout(repo.path(), &["rev-parse", "HEAD"]);
    let commits = all_ref_commit_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8_lossy(&finalized.stderr).to_string();

    let Some(operation) = state.in_progress() else {
        assert!(
            finalized.status.success(),
            "`{}` is the posture a fan-out worktree is provisioned IN — the boundary must \
             still land over it, or the fix refuses jigc's own provisioning; got {:?}\n\
             stderr:\n{stderr}",
            state.name(),
            finalized.status,
        );
        return;
    };

    assert!(
        !finalized.status.success(),
        "the boundary commits `{}`'s index, so an un-concluded `{}` there must refuse — it \
         landed at {:?} instead\nstderr:\n{stderr}",
        worktree.display(),
        state.name(),
        finalized.status,
    );
    assert!(
        stderr.contains("blocking · repo.operation-in-progress"),
        "the refusal must carry the family's shipped identity; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(operation.noun()),
        "the refusal must name THIS operation (`{}`); stderr:\n{stderr}",
        operation.noun(),
    );
    assert!(
        stderr.contains(".jigc/worktrees/area-low"),
        "the refusal must name WHICH worktree — the caller is not standing in it; \
         stderr:\n{stderr}",
    );
    // The site clause **leads** (the independent review of `986d5e0a`, LOW 1). Appended, it
    // landed after a predicate that may already end in a prepositional phrase of its own —
    // `UnmergedIndex`'s *"left unmerged paths in the index"*, `Sequencer`'s *"left a queue of
    // commits in `sequencer/`"* — and read as a place inside a place. Asserted here, over the
    // whole axis, rather than at the two members that read badly: the rule is one shape for
    // every member, and a per-member rendering choice is what the next member forgets.
    let identity_at = stderr
        .find("blocking · repo.operation-in-progress")
        .expect("the refusal carries the family's identity");
    let message = &stderr[identity_at..];
    let site_at = message
        .find("in the fan-out worktree `.jigc/worktrees/area-low`, ")
        .unwrap_or_else(|| panic!("the message leads with the site clause; stderr:\n{stderr}"));
    let noun_at = message
        .find(operation.noun())
        .expect("the message names the operation");
    assert!(
        site_at < noun_at,
        "the site clause must come BEFORE the breach it is about, not after a predicate that \
         may already end in a preposition; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!(
            "git -C {} {}",
            // The `-C` operand is the ABSOLUTE worktree since M53 (the cwd census, C1-06):
            // `-C` takes a directory, and driven, the repo-relative spelling exited 128 from
            // every cwd but the repository root — including from a sibling fan-out worktree,
            // which is where the boundary's own spawn line puts an agent.
            repo.path()
                .canonicalize()
                .unwrap_or_else(|_| repo.path().to_path_buf())
                .join(".jigc/worktrees/area-low")
                .display(),
            operation
                .abandon()
                .strip_prefix("git ")
                .expect("every routed command is a git invocation"),
        )),
        "the route must be runnable from where the caller IS — `git -C <worktree> …`; \
         stderr:\n{stderr}",
    );
    assert_eq!(
        operation_residue(&git_dir),
        residue,
        "a refused boundary must leave the operation exactly as git left it — every marker \
         present and every byte of its authored message intact",
    );
    assert_eq!(
        git_stdout(repo.path(), &["rev-parse", "HEAD"]),
        head,
        "a refused boundary must not move HEAD",
    );
    assert_eq!(
        all_ref_commit_count(repo.path()),
        commits,
        "a refused boundary must land no commit on any ref",
    );
    assert!(
        worktree.join(".git").is_file(),
        "a refused boundary must leave the fan-out worktree standing — the teardown is what \
         destroys the operation's markers",
    );
}

/// (f) **The whole git-state axis, inside the worktree the boundary commits from, at the
/// default commit model.**
///
/// The axis is `GitState::ALL` — iterated, never listed — minus the members a worktree
/// cannot hold, each excluded by its own `GitState::worktree_refusal` rather than by a
/// skip written here. Driven on `1.0.0-rc.17` before the fix, **every refusing cell of this
/// axis** landed at exit 0, committed the operation's staged payload under jigc's synthesized
/// subject and destroyed its authored message with the worktree teardown.
#[test]
fn the_boundary_refuses_every_operation_left_in_a_worktree_it_commits_from() {
    let mut refused = 0;
    let mut excluded = 0;
    for state in GitState::ALL {
        if state.worktree_refusal().is_some() {
            excluded += 1;
            continue;
        }
        fan_out_cell(*state, false);
        if state.in_progress().is_some() {
            refused += 1;
        }
    }
    assert_eq!(
        refused + excluded + 1,
        GitState::ALL.len(),
        "every member of the axis is a refusing cell, an excluded one with a stated \
         reason, or the one proceeding posture (`detached`) — {refused} refused, \
         {excluded} excluded, out of {}",
        GitState::ALL.len(),
    );
}

/// (f) **…and at the honest-rework commit model**, where the same bytes land inside the
/// sub-task's **own** commit under the sub-agent's **authored** message — the user's
/// un-concluded operation dressed as work someone signed for.
///
/// Driven on `1.0.0-rc.17`: `git cherry-pick -n` in the worktree, `finalize.fan-out.squash:
/// false`, and the picked payload landed in `feat(cache): rework the cache path` at exit 0.
/// The M53 per-axis review could not reach this model — it stopped twice at
/// `finalize.render-io` for want of the sub-task's authored commit doc, which
/// `setup_one_sub_task` stages.
#[test]
fn the_chain_commit_model_refuses_the_same_operations() {
    for state in GitState::ALL {
        if state.worktree_refusal().is_some() {
            continue;
        }
        fan_out_cell(*state, true);
    }
}

/// (f) **The preview says what the door does** — `jigc task validate <sub>` run from the
/// **main** checkout, where the cwd's own posture is spotless.
///
/// This is the surface an orchestrator reads before calling the boundary, and `jigc
/// milestone finalize` has no `--dry-run` at all, so it is the only forecast there is.
/// Before the fix it answered *"no findings — the task validates clean"* over a state the
/// boundary swallowed; leaving it that way after the fix would have made it answer *clean*
/// over a state the boundary now refuses, which is the law-1 lie the fix itself would have
/// created (`dev-workflow.md` → *widen a guard's trigger, re-derive its response*).
///
/// The sibling half is the one-answer rule: run **inside** the worktree, the cwd guard has
/// already answered, and the preview must not add a second finding about the same state.
#[test]
fn the_sub_task_preview_forecasts_the_boundarys_worktree_refusal() {
    let repo = TempDir::new("fanout-preview");
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_one_sub_task(
        repo.path(),
        home.path(),
        false,
        Overlay::Attached(GitState::UncommittedPick),
    );
    let worktree = repo.path().join(".jigc").join("worktrees").join("area-low");

    // The main checkout's own posture is spotless — so anything the preview says about
    // posture is said about the worktree. It is the MARKER set that is asserted, not `git
    // status`: the fixture's workbench is untracked by construction, and untracked files
    // are not a posture.
    assert!(
        operation_residue(&repo.path().join(".git")).is_empty(),
        "the main checkout must hold no operation of its own, or this arm proves nothing \
         about the worktree",
    );

    let outside = run_task(repo.path(), home.path(), &["validate", "area-low"]);
    let text = String::from_utf8_lossy(&outside.stderr).to_string();
    assert!(
        !outside.status.success(),
        "the preview must forecast the refusal it previews; got {:?}\nstderr:\n{text}",
        outside.status,
    );
    assert!(
        text.contains("blocking · repo.operation-in-progress")
            && text.contains(".jigc/worktrees/area-low")
            && text.contains(&format!(
                "git -C {} reset",
                repo.path()
                    .canonicalize()
                    .unwrap_or_else(|_| repo.path().to_path_buf())
                    .join(".jigc/worktrees/area-low")
                    .display(),
            )),
        "the preview must render the boundary's own finding, route included; \
         stderr:\n{text}",
    );

    // Inside the worktree the door guard answers first, at its own site — one answer, and
    // it is the one the sub-agent's own `task finalize` would print.
    let inside = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["task", "validate", "area-low"])
        .current_dir(&worktree)
        .env("HOME", home.path())
        .output()
        .expect("run the jigc binary");
    let inside_text = String::from_utf8_lossy(&inside.stderr).to_string();
    assert!(
        !inside.status.success() && inside_text.contains("blocking · repo.operation-in-progress"),
        "inside the worktree the cwd guard must still refuse; stderr:\n{inside_text}",
    );
    assert_eq!(
        inside_text.matches("repo.operation-in-progress").count(),
        1,
        "one state, one answer — the preview must not add a second finding about the \
         posture the cwd guard already reported; stderr:\n{inside_text}",
    );
    assert!(
        !inside_text.contains("in the fan-out worktree"),
        "asked from inside the worktree, the answer is about the checkout the caller IS \
         in — the `BreachSite::Here` bytes; stderr:\n{inside_text}",
    );
}

/// (f) **The zero-false-fire control**: no operation anywhere, and both commit models land.
///
/// `fan_out_cell`'s `detached` cell already carries the *exempt-member* half; this is the
/// plain one — a fan-out with nothing un-concluded in it must be untouched by the new
/// preflight, and the preview must say nothing about posture.
#[test]
fn a_fan_out_with_no_operation_anywhere_is_untouched() {
    for squash_false in [false, true] {
        let repo = TempDir::new(if squash_false {
            "fanout-control-chain"
        } else {
            "fanout-control-squash"
        });
        init_repo(repo.path());
        let home = TempDir::new("home");
        setup_one_sub_task(repo.path(), home.path(), squash_false, Overlay::Nothing);

        let preview = run_task(repo.path(), home.path(), &["validate", "area-low"]);
        let preview_text = String::from_utf8_lossy(&preview.stderr).to_string();
        assert!(
            !preview_text.contains("repo.operation-in-progress")
                && !preview_text.contains("repo.head-detached"),
            "a clean fan-out must draw no posture finding from the preview; \
             stderr:\n{preview_text}",
        );

        let before = commit_count(repo.path());
        let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
        let stderr = String::from_utf8_lossy(&finalized.stderr).to_string();
        assert!(
            finalized.status.success() && commit_count(repo.path()) > before,
            "the control boundary must land; got {:?}\nstderr:\n{stderr}",
            finalized.status,
        );
        assert!(
            git_stdout(repo.path(), &["ls-tree", "-r", "--name-only", "HEAD"])
                .contains("src/low.rs"),
            "the sub-task's own code must ride the landed boundary",
        );
    }
}

// ---------------------------------------------------------------------------
// (g) + (h) The workbench's own root is what a workbench path is spelled against.
// ---------------------------------------------------------------------------

/// Run `jigc` with an explicit cwd — the half `run_milestone` / `run_task` cannot express,
/// because both of them stand in the main checkout by construction.
fn run_from(cwd: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Both spellings of `repo` a leak could carry: the path as the fixture built it, and its
/// canonical form — the milestone doors join their worktree paths onto
/// `jigc_home.canonicalize()`, so on macOS the leaked string is the `/private/var/…` one
/// while the fixture holds `/var/…`.
fn host_spellings(repo: &Path) -> Vec<String> {
    let mut spellings = vec![repo.display().to_string()];
    if let Ok(real) = repo.canonicalize() {
        let real = real.display().to_string();
        if !spellings.contains(&real) {
            spellings.push(real);
        }
    }
    spellings
}

fn assert_no_host_path(repo: &Path, surface: &str, text: &str) {
    // The one declared exception, and it is a region rather than a weakening: a backticked
    // `git -C <absolute> …` span is bytes the reader RUNS, and git resolves a pathspec against
    // the caller's cwd (M53 — the cwd census, the route class;
    // `crates/cli/tests/support/route_spans.rs` carries the whole reason). Everything else on
    // the surface — the message the span sits in, the `at:` locus, the key, every other span —
    // is still scanned, and an absolute in a git span that is NOT aimed still fails here.
    let text = &crate::support::route_spans::redact_aimed_git_spans(text);
    for host in host_spellings(repo) {
        assert!(
            !text.contains(&host),
            "law 1: `{surface}` printed the host-absolute path `{host}` — the subject is a \
             workbench path, and the workbench hangs off jigc_home, not off the checkout the \
             caller happens to stand in; output:\n{text}",
        );
    }
}

/// A two-sub-task milestone with both worktrees provisioned and nothing staged in either —
/// the shape that gives a caller a *second* linked worktree to stand in.
fn setup_two_sub_tasks(repo: &Path, home: &Path) {
    crate::support::mint_project_layer(repo);
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low"] {
        assert!(
            run_milestone(repo, home, &["add-task", "cache-rework", intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
    for (sub, slug, title) in [
        ("area-low", "low-policy", "Low policy"),
        ("area-zed", "zed-policy", "Zed policy"),
    ] {
        stage_doc(
            repo,
            sub,
            &format!("adr:{slug}"),
            &adr_plain(title),
            "edited-from-base",
        );
        stage_subtask_commit(repo, sub, &format!("rework the {sub} cache path"));
    }
    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
}

/// (g) **The boundary's refusal and its preview, typed from inside another sub-task's
/// worktree.**
///
/// `repo_root` is `discover_repo_root(cwd)` — from inside a linked worktree that is the
/// *worktree*, while every path in the worktree set is rooted at `jigc_home`, so the prefix
/// strip fails and `render::repo_relative` falls back to the host-absolute spelling. Driven
/// at `986d5e0a`, all three of the message, the `at:` locus (the pinned `(code, target)` key
/// the `--format json` `Blocked` arm carries) and the route carried
/// `/private/var/folders/…/repo/.jigc/worktrees/area-zed`.
#[test]
fn the_fan_out_refusal_is_workbench_relative_from_inside_another_worktree() {
    let repo = TempDir::new("workbench-relative-finalize");
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_two_sub_tasks(repo.path(), home.path());

    let zed = repo.path().join(".jigc").join("worktrees").join("area-zed");
    let low = repo.path().join(".jigc").join("worktrees").join("area-low");
    git_state::overlay_worktree(&zed, home.path(), GitState::Bisect)
        .expect("a worktree can hold a bisect");

    for (surface, args) in [
        (
            "milestone finalize",
            vec!["milestone", "finalize", "cache-rework"],
        ),
        ("task validate <sub>", vec!["task", "validate", "area-zed"]),
    ] {
        let out = run_from(&low, home.path(), &args);
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        assert!(
            !out.status.success() && text.contains("repo.operation-in-progress"),
            "`{surface}` must still refuse the bisecting worktree from here; got {:?}\n{text}",
            out.status,
        );
        assert!(
            text.contains(".jigc/worktrees/area-zed"),
            "`{surface}` must name the breaching worktree the way every other surface spells \
             it; output:\n{text}",
        );
        assert_no_host_path(repo.path(), surface, &text);
    }
}

/// (g) **The pre-existing sibling the same seam feeds** — `milestone.dirty-worktree`, driven
/// from inside a second worktree at `986d5e0a` and leaking identically (it additionally
/// rendered the cwd's own worktree as `at: .`).
#[test]
fn the_dirty_worktree_refusal_is_workbench_relative_from_inside_another_worktree() {
    let repo = TempDir::new("workbench-relative-discard");
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_two_sub_tasks(repo.path(), home.path());

    let zed = repo.path().join(".jigc").join("worktrees").join("area-zed");
    let low = repo.path().join(".jigc").join("worktrees").join("area-low");
    fs::write(zed.join("scratch.txt"), "work in progress\n").expect("write worktree dirt");

    let out = run_from(&low, home.path(), &["milestone", "discard", "cache-rework"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success() && text.contains("milestone.dirty-worktree"),
        "the abandon must still refuse over the dirty sibling worktree; got {:?}\n{text}",
        out.status,
    );
    assert!(
        text.contains(".jigc/worktrees/area-zed"),
        "the refusal must name the held path the way every other surface spells it; \
         output:\n{text}",
    );
    assert_no_host_path(repo.path(), "milestone discard", &text);
}

/// (h) **`cli/src/milestone.rs` spells no path against the cwd's repo root.**
///
/// Every subject that file renders is a *workbench* path — a fan-out worktree, a task or
/// milestone area, the staging tree, `.jigc/` itself — or a scratch tree outside the
/// repository entirely, for which `repo_relative`'s absolute fallback is the honest answer
/// either way. The workbench hangs off `jigc_home`, so `jigc_home` is the root all of them
/// are spelled against; `repo_root` is the *cwd's* checkout and coincides with it only while
/// the caller stands outside a linked worktree.
///
/// The file already spelled six of its sites that way (and `task.rs` passes
/// `task.jigc_home` at every one of its own), so this is one rule that had two spellings in
/// one file rather than a rule nobody had written. Read off the source, because the claim is
/// about the *arguments* a call is written with and (g) can only reach two members of it.
///
/// ~~Out of scope, stated: `setup.rs` binds no `jigc_home` at all — its `repo_root` is the
/// root it joins every `.jigc/` path off, so the two cannot diverge inside that file, and
/// whether `jigc uninstall` should bind `jigc_home` instead is a different question about a
/// different door.~~ **Struck 2026-09-23** (the cwd fixes' review, LOW 5 / LOW 10): binding
/// only the standing checkout is not an exemption from the question, it is the answer
/// *wrong*. Falsifying datum, driven — `jigc uninstall` from a fan-out worktree removed the
/// repository-wide `pre-commit` hook at exit 0 while printing *"repo-local install removed"*
/// over a main-checkout install it left entirely standing. Both doors bind
/// `repo::jigc_home` since; `cwd_verb_subject.rs`'s three `setup`/`uninstall` arms drive the
/// cwd axis, and `setup.rs`'s module header carries the rule.
#[test]
fn milestone_renders_no_path_against_the_cwd_repo_root() {
    let src = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/milestone.rs"))
        .expect("read cli/src/milestone.rs");
    let offenders: Vec<(usize, String)> = src
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            line.contains("repo_relative(repo_root") || line.contains("repo_relative(&repo_root")
        })
        .map(|(i, line)| (i + 1, line.trim().to_string()))
        .collect();
    assert!(
        offenders.is_empty(),
        "these sites spell a workbench path against the cwd's checkout, which is a \
         host-absolute leak from inside a linked worktree — render against `jigc_home`:\n{}",
        offenders
            .iter()
            .map(|(n, line)| format!("  crates/cli/src/milestone.rs:{n}: {line}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
}

// ---------------------------------------------------------------------------
// (i) One code, one declared arm — the door and its own preview.
// ---------------------------------------------------------------------------

/// The `findings` array of a pinned envelope on `stdout`, or a panic naming what was there
/// instead — a refusal that flattened into `{"error": …}` on stderr fails *here*, which is
/// the divergence this arm is about.
fn envelope_findings(surface: &str, out: &std::process::Output) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!(
            "`{surface}` must answer on the findings envelope (stdout), not the flattened \
             `{{\"error\": …}}` arm: {err}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        )
    });
    doc["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("`{surface}`'s envelope carries a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// (i) **`repo.operation-in-progress` answers on one arm at the door and at the preview the
/// door owns** (the independent review of `986d5e0a`, MEDIUM 2).
///
/// Driven at `986d5e0a`, one state answered two ways: `jigc --format json milestone finalize`
/// printed the pinned findings envelope on stdout at exit 3, while `jigc --format json task
/// validate <sub>` — the *forecast of that very door*, sharing its producer — printed
/// `{"error": "blocking · repo.operation-in-progress — …"}` on stderr at exit 1. A code inside
/// a message is not a key, so the two surfaces disagreed about whether this finding has one.
#[test]
fn the_fan_out_posture_finding_answers_on_one_declared_arm_at_both_surfaces() {
    let repo = TempDir::new("fanout-envelope-arm");
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_one_sub_task(
        repo.path(),
        home.path(),
        false,
        Overlay::Attached(GitState::Bisect),
    );

    for (surface, args) in [
        (
            "milestone finalize",
            vec!["--format", "json", "milestone", "finalize", "cache-rework"],
        ),
        (
            "task validate <sub>",
            vec!["--format", "json", "task", "validate", "area-low"],
        ),
    ] {
        let out = run_from(repo.path(), home.path(), &args);
        let findings = envelope_findings(surface, &out);
        assert_eq!(
            out.status.code(),
            Some(3),
            "`{surface}` must refuse with the blocked exit the door uses; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let keys: Vec<&serde_json::Value> = findings.iter().map(|f| &f["key"]).collect();
        assert!(
            keys.iter().any(|key| {
                key["code"] == "repo.operation-in-progress"
                    && key["target"] == ".jigc/worktrees/area-low"
            }),
            "`{surface}` must project the declared filesystem-path key for the breaching \
             worktree; got:\n{keys:#?}",
        );
    }
}

/// (i) **The contract names the new member of the filesystem-path form.**
///
/// `design/command-output-contract.md`'s closure claim — *every finding that projects a
/// `key` lands in exactly one of those six forms, or in one of the named exceptions* — was
/// false at `986d5e0a`: the finding above projects a filesystem-path key and appeared in no
/// form's Members list, and nothing reddened, because `debug_assert_targets_declared` checks
/// presence and uniqueness and never form membership.
///
/// **This is not the closure fence, and does not claim to be.** Deriving *every code that
/// can serialize with a location* from the source is not cheap — a finding's address is
/// stamped by its producer, not declared in a registry — so what is fenced is the one
/// statement this commit makes: the doc names the code the arm above drives. A general
/// closure fence stays unbuilt and is named as such here rather than implied by a
/// narrower one.
#[test]
fn the_contract_lists_the_fan_out_posture_code_under_the_filesystem_path_form() {
    let doc = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../design/command-output-contract.md"
    ))
    .expect("read design/command-output-contract.md");
    let row = doc
        .lines()
        .find(|line| line.starts_with("| a **file** with no committed URI identity"))
        .expect("the filesystem-path form's row");
    assert!(
        row.contains("repo.operation-in-progress"),
        "the filesystem-path form's Members list must name `repo.operation-in-progress` — it \
         projects a path key at `BreachSite::FanOutWorktree`; row:\n{row}",
    );
}

/// (f) **The refusing subject a production fan-out actually presents: two breaches, not
/// one** (the independent review of `986d5e0a`, LOW 2).
///
/// Every other refusing cell of the axis is driven through `git_state::overlay_worktree`,
/// which attaches a branch first because `drive`'s constructions move between branches by
/// name. That adaptation is inert for what the probe *detects* and not for what it
/// *returns*: `jigc milestone provision` creates every fan-out worktree `--detach`ed, so the
/// real subject answers `[OperationInProgress, HeadDetached]` and which breach the boundary
/// refuses with is decided by `InProgress`' probe order plus the `Dedicated` exemption —
/// neither of which an attached, single-breach fixture exercises.
///
/// One cell, over the one state buildable with no branch to move between: a bisect is one
/// commit and `git bisect start`. It claims exactly the discrimination: the boundary refuses
/// with the **operation**, names it, and never mentions `repo.head-detached` — which, being
/// the member a dedicated worktree is exempt from, would mean jigc refusing its own
/// provisioning.
#[test]
fn a_detached_worktree_carrying_an_operation_is_refused_as_the_operation() {
    let repo = TempDir::new("fanout-detached-bisect");
    init_repo(repo.path());
    let home = TempDir::new("home");
    setup_one_sub_task(
        repo.path(),
        home.path(),
        false,
        Overlay::Detached(GitState::Bisect),
    );

    let before = all_ref_commit_count(repo.path());
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8_lossy(&finalized.stderr).to_string();
    assert!(
        !finalized.status.success(),
        "a detached worktree mid-bisect must still refuse; got {:?}\nstderr:\n{stderr}",
        finalized.status,
    );
    assert!(
        stderr.contains("blocking · repo.operation-in-progress") && stderr.contains("a bisect"),
        "the refusal must be the OPERATION's, named — probe order decides this, and only a \
         two-breach subject exercises it; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("repo.head-detached"),
        "a fan-out worktree is exempt from the detached member — refusing on it would refuse \
         jigc's own provisioning; stderr:\n{stderr}",
    );
    assert_eq!(
        all_ref_commit_count(repo.path()),
        before,
        "a refused boundary must land no commit on any ref",
    );
}
