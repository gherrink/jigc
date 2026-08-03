//! The aborted `squash: false` milestone finalize leaves the sub-agents' code **alive**
//! (M47 Inc 3 T0(a)/T1).
//!
//! `design/finalize.md`:157 promises *"working area intact"* on a rejected commit, and
//! `design/team-ready-state.md` justifies the fan-out's unconditional
//! `git worktree remove --force` by *"the commit lands **first**, so every byte the worktree
//! held is already in git"* — a precondition that is **false** on the abort arm, where the
//! commit did **not** land. Since M31 Inc 4/5 the provisioned worktree is the **sole copy** of
//! a sub-agent's code, so tearing it down on the abort destroys exactly the work a retry needs.
//!
//! **The acceptance iterates the rejection-cause axis, not the reported repro.** The destroyer
//! sat behind an untyped `Err(err) =>` from `try_execute_finalize_plan`, so it fired on every
//! way the boundary can refuse:
//!
//! 1. an **aggregate**-hook rejection (a `pre-commit` hook refusing the merged docs),
//! 2. a **per-sub-task**-hook rejection (the same hook refusing a sub-task's code commit),
//! 3. a **hookless** `git merge --ff-only` refusal — ordinary untracked WIP in the main
//!    checkout colliding with a path the fan-out commits, *with no hook installed at all*.
//!
//! Per cause: the finalize exits non-zero, every provisioned worktree is still **registered**
//! with its staged code intact, HEAD is unchanged — and with the cause removed the **identical**
//! `jigc milestone finalize <id>` re-run lands the sub-task code. Red before T1 on all three
//! (the worktrees are gone and the re-run degrades to a docs-only commit at exit 0).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-abort-survives-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
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

/// Initialize a real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(root: &Path) {
    git_ok(root, &["init", "-q"]);
    git_ok(root, &["config", "user.email", "test@example.com"]);
    git_ok(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("milestone")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// `.jigc/config/manifest.yaml` opting the project into per-sub-task commits.
fn set_squash_false(repo: &Path) {
    let config = repo.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk config layer");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  finalize.fan-out.squash: false\n",
    )
    .expect("write manifest");
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area.
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

/// A plain, ref-free ADR body.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a sub-task's authored `commit:<sub>` doc — the prose the per-sub-task render reads.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body);
}

/// Write + `git add` a code file **in** a provisioned fan-out worktree — the staged code a
/// fanned-out sub-agent produces in its isolated worktree.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = worktree_dir(repo, sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git_ok(&wt, &["add", rel]);
}

fn worktree_dir(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub)
}

/// The `git worktree list --porcelain` listing — the *registration* record, not the dir.
fn worktree_list(repo: &Path) -> String {
    git_ok(repo, &["worktree", "list", "--porcelain"])
}

fn head_sha(repo: &Path) -> String {
    git_ok(repo, &["rev-parse", "HEAD"]).trim().to_string()
}

fn rev_list_count(repo: &Path) -> u32 {
    git_ok(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("count parses")
}

/// Every path the most recent `n` commits changed (`git show --name-only`), unioned.
fn recent_changed_files(repo: &Path, n: u32) -> Vec<String> {
    let out = git_ok(repo, &["log", &format!("-{n}"), "--name-only", "--format="]);
    let mut paths: Vec<String> = out
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_owned)
        .collect();
    paths.sort();
    paths.dedup();
    paths
}

/// Install an executable `pre-commit` hook rejecting any commit whose staged set matches
/// `pattern` (a `grep -q` regex over `git diff --cached --name-only`).
fn install_rejecting_hook(repo: &Path, pattern: &str) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nif git diff --cached --name-only | grep -q '{pattern}'; then\n  echo 'rejected by test hook' >&2\n  exit 1\nfi\nexit 0\n"
        ),
    )
    .expect("write pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
}

fn remove_hook(repo: &Path) {
    let _ = fs::remove_file(repo.join(".git").join("hooks").join("pre-commit"));
}

/// The `squash: false` fan-out fixture: milestone + two sub-tasks, each with a disjoint
/// persisted ADR + its authored commit doc staged in its area, and a provisioned worktree
/// holding that sub-task's **staged** code. id-sorted sub-tasks: `[area-low, area-zed]`.
fn setup_fanout(repo: &Path, home: &Path) {
    set_squash_false(repo);
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
    stage_doc(repo, "area-low", "adr:low-policy", &adr_plain("Low policy"));
    stage_subtask_commit(repo, "area-low", "rework the low cache path");
    stage_doc(repo, "area-zed", "adr:zed-policy", &adr_plain("Zed policy"));
    stage_subtask_commit(repo, "area-zed", "rework the zed cache path");

    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
}

/// One member of the **rejection-cause axis**: how the `squash: false` boundary is made to
/// refuse, and how that cause is then removed for the recovery re-run.
#[derive(Clone, Copy)]
enum RejectionCause {
    /// A `pre-commit` hook refusing the merged docs — the parent aggregate, after every
    /// per-sub-task commit has already landed in the dedicated worktree.
    AggregateHook,
    /// The same hook refusing a **sub-task's** code commit — the chain aborts at its first
    /// commit, before the aggregate is ever built.
    SubtaskHook,
    /// **No hook at all**: ordinary untracked WIP in the main checkout at a path the fan-out
    /// commits, so `git merge --ff-only` refuses to overwrite it (the carry-or-refuse
    /// contract). The cause the "hook-rejected" framing understated.
    FfRefusal,
}

impl RejectionCause {
    fn label(self) -> &'static str {
        match self {
            RejectionCause::AggregateHook => "an aggregate-hook rejection",
            RejectionCause::SubtaskHook => "a per-sub-task-hook rejection",
            RejectionCause::FfRefusal => "a hookless `git merge --ff-only` refusal",
        }
    }

    fn install(self, repo: &Path) {
        match self {
            // The per-sub-task commits stage only `src/*.rs`, so only the aggregate trips.
            RejectionCause::AggregateHook => install_rejecting_hook(repo, "^docs/decisions/"),
            // `area-low` sorts first, so the chain aborts at its very first commit.
            RejectionCause::SubtaskHook => install_rejecting_hook(repo, "^src/low\\.rs$"),
            RejectionCause::FfRefusal => {
                let p = repo.join("src").join("low.rs");
                fs::create_dir_all(p.parent().expect("src parent")).expect("mk src/");
                fs::write(&p, "// untracked human WIP\n").expect("seed colliding untracked WIP");
            }
        }
    }

    fn remove(self, repo: &Path) {
        match self {
            RejectionCause::AggregateHook | RejectionCause::SubtaskHook => remove_hook(repo),
            RejectionCause::FfRefusal => {
                fs::remove_file(repo.join("src").join("low.rs")).expect("clear the colliding WIP");
            }
        }
    }
}

#[test]
fn an_aborted_squash_false_finalize_keeps_every_sub_task_worktree_alive_and_recovers() {
    for cause in [
        RejectionCause::AggregateHook,
        RejectionCause::SubtaskHook,
        RejectionCause::FfRefusal,
    ] {
        let label = cause.label();
        let repo = TempDir::new("abort");
        init_repo(repo.path());
        let home = TempDir::new("home");

        setup_fanout(repo.path(), home.path());
        cause.install(repo.path());

        let before_head = head_sha(repo.path());
        let before_count = rev_list_count(repo.path());

        // (1) The boundary refuses.
        let aborted = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
        assert!(
            !aborted.status.success(),
            "[{label}] the finalize must exit non-zero; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&aborted.stdout),
            String::from_utf8_lossy(&aborted.stderr),
        );

        // (2) HEAD is untouched — nothing landed.
        assert_eq!(
            head_sha(repo.path()),
            before_head,
            "[{label}] an aborted finalize must leave HEAD at the pre-finalize sha",
        );
        assert_eq!(
            rev_list_count(repo.path()),
            before_count,
            "[{label}] an aborted finalize must leave the commit count unchanged",
        );

        // (3) Every provisioned worktree is still REGISTERED with its staged code intact —
        // the worktree is the sole copy of the sub-agent's code, and the commit did not land.
        let registered = worktree_list(repo.path());
        for (sub, rel) in [("area-low", "src/low.rs"), ("area-zed", "src/zed.rs")] {
            assert!(
                registered.contains(&format!("worktrees/{sub}")),
                "[{label}] the `{sub}` fan-out worktree must still be REGISTERED after the \
                 abort; got:\n{registered}",
            );
            let wt = worktree_dir(repo.path(), sub);
            assert!(
                wt.is_dir(),
                "[{label}] the `{sub}` worktree checkout must survive the abort at {wt:?}",
            );
            let staged = git_ok(&wt, &["diff", "--cached", "--name-only"]);
            assert!(
                staged.lines().any(|l| l == rel),
                "[{label}] `{sub}`'s staged code (`{rel}`) must survive the abort; \
                 `git diff --cached --name-only` gave:\n{staged}",
            );
        }

        // (4) Remove the cause and re-run the IDENTICAL argv — the abort is recoverable.
        cause.remove(repo.path());
        let landed = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
        assert!(
            landed.status.success(),
            "[{label}] the re-run of the identical `jigc milestone finalize cache-rework` must \
             exit 0 once the cause is removed; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&landed.stdout),
            String::from_utf8_lossy(&landed.stderr),
        );

        // (5) The landed boundary carries the sub-task CODE — not a docs-only degrade.
        let new_commits = rev_list_count(repo.path()) - before_count;
        assert_eq!(
            new_commits, 3,
            "[{label}] the recovered squash:false boundary must land N+1 = 3 commits",
        );
        let changed = recent_changed_files(repo.path(), new_commits);
        for rel in ["src/low.rs", "src/zed.rs"] {
            assert!(
                changed.contains(&rel.to_owned()),
                "[{label}] the recovered boundary must commit the sub-task code `{rel}`; \
                 the landed commits changed:\n{changed:?}",
            );
        }
    }
}
