//! `jigc uninstall` refuses while a fan-out sub-task worktree holds uncommitted work
//! (the M47 completion audit's `uninstall` finding; `design/team-ready-state.md` →
//! Abandon refuses on a dirty worktree, the sibling guard).
//!
//! `uninstall` removes `<repo>/.jigc/` with `std::fs::remove_dir_all`, and since M31
//! Inc 4/5 the fan-out worktrees live **inside** it (`.jigc/worktrees/<sub-task-id>`).
//! M47 Inc 3 made "a provisioned worktree holding the sole copy of a sub-agent's
//! uncommitted work" a **normal, documented, promised-safe** state: the aborted
//! `squash: false` finalize deliberately stops tearing worktrees down so the retry can
//! recover. An unguarded `uninstall` over exactly that state destroyed the work —
//! LLM-authored doc prose in `.jigc/tasks/<id>/docs/*.md` that was never in the object
//! DB, the worktrees' unstaged/untracked edits, and the staged code reduced to a
//! pathless dangling blob — at **exit 0**, reporting only `- removed .jigc/`.
//!
//! So `uninstall` carries the same guard its sibling teardown already carries
//! (`jigc milestone discard`'s `milestone.dirty-worktree`): a registered worktree under
//! `.jigc/worktrees/` whose `git status --porcelain` is non-empty **blocks** the
//! teardown with `uninstall.dirty-worktree`, naming each path and its porcelain
//! entries, routed at the already-shipped escape hatch.
//!
//! **The acceptance iterates the rejection-cause axis, not one repro** — the promised-safe
//! state is reachable through each of the three ways the fan-out boundary can refuse
//! (`milestone_abort_survives.rs`' axis: an aggregate-hook rejection, a per-sub-task-hook
//! rejection, and a hookless `git merge --ff-only` refusal) — and it pins **both** clean
//! arms, because a guard that refuses a legitimate teardown is its own defect: a repo with
//! no worktrees at all, and a repo whose provisioned worktrees are **clean**, both still
//! uninstall at exit 0.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-uninstall-guard-{tag}-{}-{:?}",
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
        // Registered worktrees inside the tree are ordinary directories to `remove_dir_all`.
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
    git_ok(root, &["config", "commit.gpgsign", "false"]);
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc uninstall` with `cwd = repo` and `$HOME = home`.
fn run_uninstall(repo: &Path, home: &Path) -> std::process::Output {
    run_jigc(repo, home, &["uninstall"])
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

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area —
/// the LLM-authored prose that exists **only** there until the boundary lands.
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

fn worktree_dir(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub)
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

/// The `squash: false` fan-out fixture: milestone + two sub-tasks, (unless `prose` is
/// false) each with a persisted ADR + its authored commit doc staged in its area, and
/// (unless `dirty` is false) a provisioned worktree holding that sub-task's **staged**
/// code.
///
/// **`prose` is a separate knob from `dirty`** because the teardown now has two guards
/// and the clean arm has to isolate one: staged `.jigc/tasks/<id>/docs/*.md` refuses the
/// teardown on its own (`uninstall.staged-prose`), so a fixture asserting *clean
/// worktrees still tear down* must stage none.
fn setup_fanout(repo: &Path, home: &Path, dirty: bool, prose: bool) {
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
    if prose {
        stage_doc(repo, "area-low", "adr:low-policy", &adr_plain("Low policy"));
        stage_subtask_commit(repo, "area-low", "rework the low cache path");
        stage_doc(repo, "area-zed", "adr:zed-policy", &adr_plain("Zed policy"));
        stage_subtask_commit(repo, "area-zed", "rework the zed cache path");
    }

    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    if dirty {
        stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
        stage_worktree_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
    }
}

/// One member of the **rejection-cause axis** — how the `squash: false` boundary is made
/// to refuse, leaving the promised-safe state `uninstall` must not destroy
/// (`milestone_abort_survives.rs` owns the survival property itself).
#[derive(Clone, Copy)]
enum RejectionCause {
    /// A `pre-commit` hook refusing the merged docs — the parent aggregate, after every
    /// per-sub-task commit has already landed in the dedicated worktree.
    AggregateHook,
    /// The same hook refusing a **sub-task's** code commit — the chain aborts at its first
    /// commit, before the aggregate is ever built.
    SubtaskHook,
    /// **No hook at all**: ordinary untracked WIP in the main checkout at a path the fan-out
    /// commits, so `git merge --ff-only` refuses to overwrite it.
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
            RejectionCause::AggregateHook => install_rejecting_hook(repo, "^docs/decisions/"),
            RejectionCause::SubtaskHook => install_rejecting_hook(repo, "^src/low\\.rs$"),
            RejectionCause::FfRefusal => {
                let p = repo.join("src").join("low.rs");
                fs::create_dir_all(p.parent().expect("src parent")).expect("mk src/");
                fs::write(&p, "// untracked human WIP\n").expect("seed colliding untracked WIP");
            }
        }
    }
}

#[test]
fn uninstall_refuses_while_an_aborted_fan_outs_worktrees_hold_uncommitted_work() {
    for cause in [
        RejectionCause::AggregateHook,
        RejectionCause::SubtaskHook,
        RejectionCause::FfRefusal,
    ] {
        let label = cause.label();
        let repo = TempDir::new("dirty");
        init_repo(repo.path());
        let home = TempDir::new("home");

        setup_fanout(repo.path(), home.path(), true, true);
        cause.install(repo.path());

        // The boundary refuses, leaving the worktrees alive with the sole copy of the
        // sub-agents' work (M47 Inc 3 — the promised-safe state).
        let aborted = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
        assert!(
            !aborted.status.success(),
            "[{label}] the finalize must exit non-zero; stderr:\n{}",
            String::from_utf8_lossy(&aborted.stderr),
        );
        assert!(
            worktree_dir(repo.path(), "area-low").is_dir(),
            "[{label}] the fixture must leave the fan-out worktrees alive",
        );

        // (1) `uninstall` refuses — non-zero, with the routed finding on stderr.
        let refused = run_uninstall(repo.path(), home.path());
        let stderr = String::from_utf8(refused.stderr).expect("utf-8 stderr");
        let stdout = String::from_utf8(refused.stdout).expect("utf-8 stdout");
        assert!(
            !refused.status.success(),
            "[{label}] `jigc uninstall` must refuse while a fan-out worktree is dirty; \
             stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        assert!(
            stderr.contains("uninstall.dirty-worktree"),
            "[{label}] the refusal must carry the `uninstall.dirty-worktree` code; \
             stderr:\n{stderr}",
        );

        // (2) It names every dirty worktree and the porcelain entries that make it dirty —
        // the `milestone.dirty-worktree` sibling's shape.
        for (sub, rel) in [("area-low", "src/low.rs"), ("area-zed", "src/zed.rs")] {
            let path = worktree_dir(repo.path(), sub);
            let shown = path
                .canonicalize()
                .unwrap_or(path)
                .display()
                .to_string()
                .replace("/private/", "/");
            let stderr_norm = stderr.replace("/private/", "/");
            assert!(
                stderr_norm.contains(&shown),
                "[{label}] the refusal must name the dirty worktree `{shown}`; stderr:\n{stderr}",
            );
            assert!(
                stderr.contains(rel),
                "[{label}] the refusal must name `{rel}`, the uncommitted entry; \
                 stderr:\n{stderr}",
            );
        }

        // (3) The route names the already-shipped escape hatch: get the work out, or
        // abandon the milestone with `--force`, then re-run the teardown.
        assert!(
            stderr.contains("jigc milestone discard") && stderr.contains("--force"),
            "[{label}] the route must name the abandon escape hatch; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("jigc uninstall"),
            "[{label}] the route must name the re-run; stderr:\n{stderr}",
        );

        // (4) NOTHING was removed — the refusal is a guard, not a partial teardown: the
        // workbench, the sub-agents' staged code, and the LLM-authored prose that lives
        // nowhere else are all still on disk.
        assert!(
            repo.path().join(".jigc").is_dir(),
            "[{label}] `.jigc/` must survive the refusal",
        );
        for (sub, rel) in [("area-low", "src/low.rs"), ("area-zed", "src/zed.rs")] {
            assert!(
                worktree_dir(repo.path(), sub).join(rel).is_file(),
                "[{label}] `{sub}`'s worktree code must survive the refusal",
            );
        }
        assert!(
            repo.path()
                .join(".jigc/tasks/area-low/docs/adr:low-policy.md")
                .is_file(),
            "[{label}] the authored ADR prose (never in the object DB) must survive",
        );
    }
}

#[test]
fn uninstall_still_tears_down_a_repo_with_no_worktrees() {
    let repo = TempDir::new("plain");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A workbench with no fan-out at all — the ordinary teardown the guard must not touch.
    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    assert!(
        repo.path().join(".jigc").is_dir(),
        "the fixture has a `.jigc/`"
    );

    let out = run_uninstall(repo.path(), home.path());
    assert!(
        out.status.success(),
        "a worktree-less repo must still uninstall at exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "the teardown must still remove `.jigc/`",
    );
}

#[test]
fn uninstall_still_tears_down_when_every_provisioned_worktree_is_clean() {
    let repo = TempDir::new("clean");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Provisioned worktrees, none of them holding uncommitted work, and no staged sub-task
    // prose — nothing is lost by removing them, so both guards must stay silent.
    setup_fanout(repo.path(), home.path(), false, false);
    assert!(
        worktree_dir(repo.path(), "area-low").is_dir(),
        "the fixture must provision the fan-out worktrees",
    );

    let out = run_uninstall(repo.path(), home.path());
    assert!(
        out.status.success(),
        "clean worktrees must not block the teardown; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "the teardown must still remove `.jigc/`",
    );
}

/// The **reproduced loss** (the pre-1.0.0 trial, live on `1.0.0-rc.10`): the ordinary
/// single-task loop — `setup` → `start` → `doc set-slot` — leaves LLM-authored prose in
/// `.jigc/tasks/<id>/docs/*.md` that **no object DB has a copy of**, and the teardown
/// removed `.jigc/` wholesale at exit 0, reporting only `- removed .jigc/`.
///
/// The registered-worktree probe structurally cannot see this: no worktree is involved at
/// all. So the teardown carries a **second** guard over the staged-`*.md` set, under its
/// own `uninstall.staged-prose` identity — the same door, a different subject, because
/// `uninstall.dirty-worktree` printed over a path with no worktree in it would be a law-1
/// lie ([surface-contract.md](../../design/surface-contract.md) → law 1).
///
/// **The subject is the staged `*.md` set, never directory-non-emptiness** — `docs/` always
/// also holds `provenance.json` — and it does **not** filter on the transient mark: the doc
/// destroyed in the reproduced loss is `commit:<task>`, a transient doctype whose prose is
/// exactly what the operator typed.
#[test]
fn uninstall_refuses_while_an_open_tasks_authored_prose_lives_only_in_the_workbench() {
    let repo = TempDir::new("prose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let installed = run_jigc(repo.path(), home.path(), &["setup"]);
    assert!(
        installed.status.success(),
        "`jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&installed.stderr),
    );
    let minted = run_jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "quick-fix", "fix the typo"],
    );
    assert!(
        minted.status.success(),
        "the quick-fix mint must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&minted.stderr),
    );
    let task = "fix-the-typo";
    let prose = "Fix the typo in the gateway header.";
    let authored = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args([
            "doc",
            "set-slot",
            &format!("commit:{task}#summary"),
            "--from-file",
            "-",
        ])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .as_mut()
                .expect("stdin pipe")
                .write_all(prose.as_bytes())?;
            child.wait_with_output()
        })
        .expect("run `jigc doc set-slot`");
    assert!(
        authored.status.success(),
        "`jigc doc set-slot` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&authored.stderr),
    );

    let staged = repo
        .path()
        .join(".jigc/tasks")
        .join(task)
        .join("docs")
        .join(format!("commit:{task}.md"));
    let before = fs::read(&staged).expect("the staged commit doc exists after the write");
    assert!(
        String::from_utf8_lossy(&before).contains(prose),
        "the fixture must have landed the authored prose in the staged doc",
    );

    // (1) The teardown refuses — non-zero, routed finding on stderr.
    let refused = run_uninstall(repo.path(), home.path());
    let stderr = String::from_utf8(refused.stderr).expect("utf-8 stderr");
    let stdout = String::from_utf8(refused.stdout).expect("utf-8 stdout");
    assert!(
        !refused.status.success(),
        "`jigc uninstall` must refuse while an open task holds authored prose; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("uninstall.staged-prose"),
        "the refusal must carry its own `uninstall.staged-prose` code; stderr:\n{stderr}",
    );

    // (2) It names the doc identity that would be destroyed — the address `jigc doc show
    //     <addr> --task <id>` reads back, not a bare filename.
    assert!(
        stderr.contains(&format!("commit:{task}")),
        "the refusal must name `commit:{task}`; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(task),
        "the refusal must name the open task; stderr:\n{stderr}",
    );

    // (3) The route names the honest exits and the consent flag.
    assert!(
        stderr.contains("jigc task finalize") || stderr.contains("jigc task discard"),
        "the route must name what to do with the open task; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("--force"),
        "the route must name the consent flag; stderr:\n{stderr}",
    );

    // (4) NOTHING was removed, and the prose is byte-intact.
    assert!(
        repo.path().join(".jigc").is_dir(),
        "`.jigc/` must survive the refusal",
    );
    assert_eq!(
        fs::read(&staged).expect("the staged doc survives"),
        before,
        "the authored prose must survive the refusal byte-for-byte",
    );

    // (5) `--force` is the operator's consent: the teardown completes.
    let forced = run_jigc(repo.path(), home.path(), &["uninstall", "--force"]);
    assert!(
        forced.status.success(),
        "`jigc uninstall --force` must complete the teardown; stderr:\n{}",
        String::from_utf8_lossy(&forced.stderr),
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "`--force` must remove `.jigc/`",
    );
}

/// The cell the registered-worktree probe **structurally cannot see**: a non-empty,
/// worktree-shaped path under `.jigc/worktrees/` that is registered nowhere. It is the
/// ordinary `cp -R` / `mv` shape (a copy's worktree admin record names the *source*, so
/// nothing under the copy's own `.jigc/worktrees/` is registered there) reduced to its
/// essence — and under the old registered-set subject the guard was inert over it and
/// `remove_dir_all` took the lot at exit 0.
///
/// So the subject is now the **path**, classified fail-closed by the shared
/// `probe_leftover`: git can vouch for nothing here, and a directory holding content the
/// binary cannot place refuses.
#[test]
fn uninstall_refuses_a_non_empty_worktree_path_no_registered_probe_can_see() {
    let repo = TempDir::new("unregistered");
    init_repo(repo.path());
    let home = TempDir::new("home");

    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    let ghost = worktree_dir(repo.path(), "area-ghost");
    fs::create_dir_all(&ghost).expect("mk the unregistered worktree-shaped path");
    let precious = ghost.join("precious.txt");
    fs::write(&precious, "work that is in no object DB\n").expect("plant the content");
    let before = fs::read(&precious).expect("read the planted bytes");

    let refused = run_uninstall(repo.path(), home.path());
    let stderr = String::from_utf8(refused.stderr).expect("utf-8 stderr");
    let stdout = String::from_utf8(refused.stdout).expect("utf-8 stdout");
    assert!(
        !refused.status.success(),
        "`jigc uninstall` must refuse over a non-empty path under `.jigc/worktrees/` that is \
         registered nowhere; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("uninstall.dirty-worktree"),
        "the refusal must carry the `uninstall.dirty-worktree` code; stderr:\n{stderr}",
    );
    let shown = ghost
        .canonicalize()
        .unwrap_or(ghost.clone())
        .display()
        .to_string()
        .replace("/private/", "/");
    assert!(
        stderr.replace("/private/", "/").contains(&shown),
        "the refusal must name the path `{shown}`; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("precious.txt"),
        "the refusal must name what would be deleted; stderr:\n{stderr}",
    );
    assert!(
        repo.path().join(".jigc").is_dir() && precious.is_file(),
        "the refusal must remove nothing",
    );
    assert_eq!(
        fs::read(&precious).expect("the planted bytes survive"),
        before,
        "the planted bytes must survive byte-for-byte",
    );

    let forced = run_jigc(repo.path(), home.path(), &["uninstall", "--force"]);
    assert!(
        forced.status.success(),
        "`jigc uninstall --force` must complete the teardown; stderr:\n{}",
        String::from_utf8_lossy(&forced.stderr),
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "`--force` must remove `.jigc/`",
    );
}

/// The law-1 half of the same guard: the refusals this suite pins are part of what the
/// verb *is*, so `jigc uninstall --help` — the surface a reader consults before running a
/// teardown — must state them. Before the M47 fix the long help read "Idempotent and
/// non-destructive: a second run is a clean no-op, …" with no qualification; the fix
/// qualified it with the worktree refusal, and this wave falsified it twice more — the
/// staged-prose refusal is a **second** refused state (the help claimed "One state"), and
/// `--force` makes the verb destructive on demand (the help promised your own file content
/// "preserved byte-for-byte" unconditionally)
/// ([surface-contract.md](../../design/surface-contract.md) → law 1, nothing lies).
///
/// Asserted against the **emitted bytes** (the real `--help` render through the built
/// binary), not the const the doc comment compiles into, so the pin binds what a reader
/// actually sees. `--help` needs no repo or pack — clap prints it before dispatch.
#[test]
fn uninstall_long_help_states_the_refusal_and_names_its_escape_hatch() {
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["uninstall", "--help"])
        .output()
        .expect("run the jigc binary");
    assert!(
        out.status.success(),
        "`jigc uninstall --help` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    // clap wraps the long help to the terminal width, so compare on collapsed whitespace.
    let help = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    for needle in [
        // it names BOTH states that refuse, each by the code its refusal carries …
        "uninstall.dirty-worktree",
        ".jigc/worktrees/",
        "uninstall.staged-prose",
        ".jigc/tasks/",
        // … the escape hatches the findings' own routes name …
        "jigc milestone discard",
        "jigc task finalize",
        // … and the one flag that turns the refusal into a deletion.
        "--force",
    ] {
        assert!(
            help.contains(needle),
            "`uninstall --help` must state both refusals and the consent flag — missing \
             {needle:?}; got:\n{help}",
        );
    }
    // The verb is still idempotent, and still leaves the host files it edits byte-for-byte
    // — that half of the promise survives, scoped to what is actually true of it.
    for needle in ["Idempotent", "byte-for-byte"] {
        assert!(
            help.contains(needle),
            "`uninstall --help` must keep the accepted-state promise — missing {needle:?}; \
             got:\n{help}",
        );
    }
    // The two falsified claims are the defect itself: an unqualified "non-destructive"
    // (`--force` deletes on demand) and a refusal set of exactly "One state" (there are two).
    for banned in [
        "Idempotent and non-destructive: a second run",
        "non-destructive on every state it accepts",
        "One state it refuses",
    ] {
        assert!(
            !help.contains(banned),
            "`uninstall --help` must not restate the falsified promise {banned:?}; got:\n{help}",
        );
    }
}
