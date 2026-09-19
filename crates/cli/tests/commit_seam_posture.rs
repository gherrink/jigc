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
