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
//! **And what the guards let through, the teardown names** (M46 Inc 2 T3). `uninstall` is a
//! member of `cli::milestone::DESTROYING_DOORS`, and every member owes the loss narration
//! law 1 requires: it removes `.jigc/` with `remove_dir_all`, so whatever the two guards do
//! not refuse on is destroyed exactly as hard as what they do. That is observable precisely
//! where no refusal fires — a worktree holding nothing but **gitignored** bytes, which the
//! refusal probe deliberately does not look at — so the narration is not gated on `--force`
//! and the last cell below drives the ordinary clean teardown.
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

/// How a refusal spells a path it names, since **M50 Increment 12 / T1**: repo-relative and
/// `/`-separated, because a surface prints no host filesystem
/// (`design/surface-contract.md` → The printed-path fence; the axis suite is
/// `crates/cli/tests/repo_relative_paths.rs`). Asserting the absolute spelling here was
/// asserting the defect — and it is why this file used to normalize macOS's `/private/`
/// prefix by hand: a repo-relative path has no such prefix to normalize.
fn as_printed(repo: &Path, path: &Path) -> String {
    let (root, real) = match (repo.canonicalize(), path.canonicalize()) {
        (Ok(root), Ok(real)) => (root, real),
        _ => (repo.to_path_buf(), path.to_path_buf()),
    };
    real.strip_prefix(&root)
        .unwrap_or(&real)
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
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

/// Commit what a real `jigc setup` would already have committed: the non-transient
/// `.jigc/` files this suite's fixtures hand-write or let a jigc verb create — the
/// `.gitignore` `crate::gitignore::ensure` seeds and the `config/` layer
/// [`set_squash_false`] writes.
///
/// Since M50 Increment 4 the teardown carries a **third** sole-copy guard beside this
/// suite's own: a file under `.jigc/` outside the transient `ENTRIES` prefixes that no
/// index has a copy of blocks with `uninstall.untracked-workbench-file`. These fixtures
/// skip `jigc setup` — which is what tracks those paths in a real install — so without
/// this the arms below would be answered by a fixture artifact instead of by the
/// **worktree** axis they are declared on. Idempotent, and a no-op where nothing is
/// staged.
fn commit_workbench(repo: &Path) {
    let mut args: Vec<&str> = vec!["add", "--force", "--"];
    for rel in [".jigc/.gitignore", ".jigc/config"] {
        if repo.join(rel).exists() {
            args.push(rel);
        }
    }
    if args.len() == 3 {
        return;
    }
    git_ok(repo, &args);
    let clean = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .current_dir(repo)
        .status()
        .expect("run git diff --cached")
        .success();
    if !clean {
        git_ok(repo, &["commit", "-q", "-m", "workbench"]);
    }
}

/// Run `jigc uninstall` with `cwd = repo` and `$HOME = home`, over a workbench whose
/// non-transient files a commit has a copy of ([`commit_workbench`]).
fn run_uninstall(repo: &Path, home: &Path) -> std::process::Output {
    commit_workbench(repo);
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
            let shown = as_printed(repo.path(), &worktree_dir(repo.path(), sub));
            assert!(
                stderr.contains(&shown),
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

/// The **dominant cell of the same guard**, and the one the suite above never reached: a
/// task that was `start`ed and nothing more. `start` auto-creates the workflow's transient
/// `commit:<task>` doc, so `.jigc/tasks/<id>/docs/` holds a machine-written *skeleton* with
/// zero authored bytes — and the probe, by its own design note, "can no more distinguish a
/// pristine skeleton from authored prose" than it can read minds. It therefore refuses here
/// too (correct — refusing is the honest answer when the binary cannot prove the bytes are
/// disposable), but the refusal must not **claim** what it cannot back: saying `.jigc/`
/// "holds authored doc prose … removing it would destroy that prose" over a skeleton nobody
/// typed into is a law-1 lie ([surface-contract.md](../../design/surface-contract.md) → law
/// 1), the exact repair this door's *sibling* message already took ("content", not
/// "uncommitted work").
///
/// The route compounds it: over this cell `jigc task finalize <id>` **cannot succeed** —
/// the empty skeleton fails `schema-conformance` — so the reachable exit (`jigc task
/// discard`) has to be named first. This test pins the unreachability rather than assuming
/// it, so the ordering claim rests on a measured fact.
#[test]
fn uninstall_refuses_a_pristine_skeleton_without_claiming_prose_it_cannot_see() {
    let repo = TempDir::new("skeleton");
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
        &["start", "--workflow", "quick-fix", "add a widget"],
    );
    assert!(
        minted.status.success(),
        "the quick-fix mint must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&minted.stderr),
    );
    let task = "add-a-widget";

    // The fixture IS the pristine cell: the auto-created doc exists and carries no authored
    // bytes at all — every slot is empty. (Asserted, so the test cannot silently drift into
    // the authored case the suite above already covers.)
    let staged = repo
        .path()
        .join(".jigc/tasks")
        .join(task)
        .join("docs")
        .join(format!("commit:{task}.md"));
    let skeleton = fs::read_to_string(&staged).expect("`start` auto-creates the commit doc");
    assert!(
        skeleton.contains("## Summary"),
        "the fixture must be the auto-created skeleton; got:\n{skeleton}",
    );
    for line in skeleton.lines() {
        let line = line.trim();
        assert!(
            line.is_empty()
                || line.starts_with('#')
                || line.starts_with("---")
                || line.ends_with(':'),
            "the fixture must hold ZERO authored bytes — `{line}` is content; got:\n{skeleton}",
        );
    }

    // (1) The teardown still refuses — the guard is right to fire; it cannot prove these
    //     bytes are disposable.
    let refused = run_uninstall(repo.path(), home.path());
    let stderr = String::from_utf8(refused.stderr).expect("utf-8 stderr");
    let stdout = String::from_utf8(refused.stdout).expect("utf-8 stdout");
    assert!(
        !refused.status.success(),
        "`jigc uninstall` must still refuse over an open task's staged doc; \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("uninstall.staged-prose") && stderr.contains(&format!("commit:{task}")),
        "the refusal must carry its code and name the doc; stderr:\n{stderr}",
    );

    // (2) …and it claims only what the probe can back: a staged doc no commit has a copy
    //     of. Not authored prose — nobody authored anything here.
    for banned in ["authored", "doc prose", "prose for", "that prose"] {
        assert!(
            !stderr.contains(banned),
            "the refusal must not claim prose the probe cannot see — found {banned:?}; \
             stderr:\n{stderr}",
        );
    }

    // (3) The route names the reachable exit first. `finalize` is not reachable from this
    //     state — measured here, not assumed.
    let finalized = run_jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !finalized.status.success(),
        "the premise: `jigc task finalize` cannot land an untouched skeleton; stderr:\n{}",
        String::from_utf8_lossy(&finalized.stderr),
    );
    let discard_at = stderr
        .find("jigc task discard")
        .unwrap_or_else(|| panic!("the route must name `jigc task discard`; stderr:\n{stderr}"));
    let finalize_at = stderr
        .find("jigc task finalize")
        .unwrap_or_else(|| panic!("the route must name `jigc task finalize`; stderr:\n{stderr}"));
    assert!(
        discard_at < finalize_at,
        "the route must name the reachable exit (`jigc task discard`) before the one this \
         state cannot reach; stderr:\n{stderr}",
    );

    // (4) Nothing was removed, and `--force` is still the consent that completes it.
    assert!(
        repo.path().join(".jigc").is_dir(),
        "`.jigc/` must survive the refusal",
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
    let shown = as_printed(repo.path(), &ghost);
    assert!(
        stderr.contains(&shown),
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
/// M50 Increment 4 falsified the count a third time: the workbench file no index has a
/// copy of is a **third** refused state, so "Two states" joins the banned list below —
/// a count in a help text is a claim, and this one has now been wrong twice.
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
        "uninstall.untracked-workbench-file",
        // … the escape hatches the findings' own routes name, the abandon one carrying the
        // condition that makes it reachable (a milestone teardown removes the worktrees this
        // repo registered and skips every other path, so an unqualified offer is a law-1 lie
        // over the `cp -R` shape) …
        "has registered as a worktree",
        "jigc milestone discard",
        "jigc task discard",
        "jigc task finalize",
        "git add",
        // … and the one flag that turns the refusal into a deletion.
        "--force",
    ] {
        assert!(
            help.contains(needle),
            "`uninstall --help` must state all three refusals and the consent flag — \
             missing {needle:?}; got:\n{help}",
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
    // A third falsified claim, same law: the help described the staged-doc refusal as
    // firing on "authored doc prose", which the probe cannot see — `start` alone trips it
    // (see `uninstall_refuses_a_pristine_skeleton_without_claiming_prose_it_cannot_see`).
    for banned in [
        "Idempotent and non-destructive: a second run",
        "non-destructive on every state it accepts",
        "One state it refuses",
        "Two states it refuses",
        "holds authored doc prose",
        "authored doc prose blocks",
    ] {
        assert!(
            !help.contains(banned),
            "`uninstall --help` must not restate the falsified promise {banned:?}; got:\n{help}",
        );
    }
}

/// Recursively copy `src` into `dst` — the `cp -R` shape every trial corpus is made with.
/// A copied repo's worktree admin records still name the **source's** paths, so nothing
/// under the copy's own `.jigc/worktrees/` is registered there (the fixtures hold no
/// symlinks, so a plain file/dir walk is the whole job).
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("mk the copy root");
    for entry in fs::read_dir(src).expect("read the source dir") {
        let entry = entry.expect("source dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type().expect("file type").is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy a file");
        }
    }
}

/// Move a blocking worktree path out of `.jigc/` — the route's **first** arm ("get the work
/// out of those worktrees first … then re-run"), followed mechanically.
fn take_the_work_out(repo: &Path, stash: &Path, sub: &str) {
    let from = worktree_dir(repo, sub);
    let to = stash.join(sub);
    fs::create_dir_all(stash).expect("mk the stash root");
    fs::rename(&from, &to).expect("move the worktree path out of `.jigc/`");
}

/// The **route's own claim**, followed: the refusal names exits, and an exit that provably
/// does not clear the block is a law-1 lie ([surface-contract.md](../../design/surface-contract.md)).
///
/// The cell this test owns is the one the M48 widening added: a worktree path holding
/// content that **this repository has registered nowhere** — the ordinary `cp -R` shape,
/// and the whole reason the guard's subject moved off the registered set. The teardown the
/// route used to name (`jigc milestone discard <id> --force`) removes **registered**
/// worktrees and skips everything else, so on this cell following it burns an irreversible,
/// committed abandon and leaves `uninstall` blocked by the identical finding. That premise
/// is **measured here, not assumed**: the discard is run and its non-effect asserted.
#[test]
fn uninstall_route_offers_no_abandon_arm_where_no_abandon_can_clear_the_block() {
    let src = TempDir::new("copysrc");
    init_repo(src.path());
    let home = TempDir::new("home");
    // Dirty worktrees, no staged prose — so the worktree door is the only one in play.
    setup_fanout(src.path(), home.path(), true, false);

    let copy = TempDir::new("copydst");
    let copied = copy.path().join("repo");
    copy_tree(src.path(), &copied);
    let planted = worktree_dir(&copied, "area-low").join("wip.txt");
    fs::write(&planted, "live sub-agent bytes\n").expect("plant the live bytes");
    let before = fs::read(&planted).expect("read the planted bytes");

    // (1) The copy's teardown refuses — the guard sees the path, registered or not.
    let refused = run_uninstall(&copied, home.path());
    let stderr = String::from_utf8(refused.stderr).expect("utf-8 stderr");
    let stdout = String::from_utf8(refused.stdout).expect("utf-8 stdout");
    assert!(
        !refused.status.success(),
        "the copy's teardown must refuse; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        stderr.contains("uninstall.dirty-worktree"),
        "the refusal must carry the `uninstall.dirty-worktree` code; stderr:\n{stderr}",
    );

    // (2) …and it names NO abandon command, because no abandon reaches these paths.
    assert!(
        !stderr.contains("jigc milestone discard"),
        "the route must not name an abandon that cannot clear the block; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("jigc uninstall --force"),
        "the route must still name the consent flag; stderr:\n{stderr}",
    );

    // (3) The premise, measured: the abandon the route used to name settles the record
    //     irreversibly at exit 0 and clears nothing here.
    let discarded = run_milestone(
        &copied,
        home.path(),
        &["discard", "cache-rework", "--force"],
    );
    assert!(
        discarded.status.success(),
        "the premise: `milestone discard --force` succeeds on the copy; stderr:\n{}",
        String::from_utf8_lossy(&discarded.stderr),
    );
    assert_eq!(
        fs::read(&planted).expect("the planted bytes survive the abandon"),
        before,
        "the premise: the abandon leaves an unregistered path's content on disk",
    );
    let still = run_uninstall(&copied, home.path());
    let still_err = String::from_utf8(still.stderr).expect("utf-8 stderr");
    assert!(
        !still.status.success() && still_err.contains("uninstall.dirty-worktree"),
        "the premise: after the abandon the teardown is blocked by the identical finding; \
         stderr:\n{still_err}",
    );

    // (4) The arm the route DOES name clears it: take the work out, re-run.
    let stash = copy.path().join("rescued");
    for sub in ["area-low", "area-zed"] {
        take_the_work_out(&copied, &stash, sub);
    }
    let cleared = run_uninstall(&copied, home.path());
    assert!(
        cleared.status.success(),
        "following the emitted route must clear the block; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&cleared.stdout),
        String::from_utf8_lossy(&cleared.stderr),
    );
    assert!(
        !copied.join(".jigc").exists(),
        "the followed route must complete the teardown",
    );
    assert_eq!(
        fs::read(stash.join("area-low").join("wip.txt")).expect("the rescued bytes"),
        before,
        "the rescued work must be byte-intact",
    );
}

/// The other two cells of the same axis — the ones where this repository **did** register
/// the path — pinned by following the emitted arm rather than trusting it:
///
/// - **all registered**: the abandon arm is named, and running it clears the block;
/// - **mixed**: the abandon arm is named *and* says what it leaves behind, and running it
///   leaves exactly that — the unregistered path, still blocking, named on its own.
#[test]
fn uninstall_route_keeps_the_abandon_arm_where_this_repo_registered_the_path() {
    for ghost in [false, true] {
        let label = if ghost { "mixed" } else { "all-registered" };
        let repo = TempDir::new(if ghost { "mixed" } else { "registered" });
        init_repo(repo.path());
        let home = TempDir::new("home");
        setup_fanout(repo.path(), home.path(), true, false);
        let ghost_dir = worktree_dir(repo.path(), "area-ghost");
        if ghost {
            fs::create_dir_all(&ghost_dir).expect("mk the unregistered path");
            fs::write(ghost_dir.join("precious.txt"), "in no object DB\n")
                .expect("plant the content");
        }

        let refused = run_uninstall(repo.path(), home.path());
        let stderr = String::from_utf8(refused.stderr).expect("utf-8 stderr");
        assert!(
            !refused.status.success() && stderr.contains("uninstall.dirty-worktree"),
            "[{label}] the teardown must refuse; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("jigc milestone discard") && stderr.contains("--force"),
            "[{label}] the route must name the abandon arm that does reach these paths; \
             stderr:\n{stderr}",
        );
        if ghost {
            assert!(
                stderr.contains("still block"),
                "[{label}] the route must say the unregistered path is not cleared by the \
                 abandon; stderr:\n{stderr}",
            );
        }

        // Follow it.
        let discarded = run_milestone(
            repo.path(),
            home.path(),
            &["discard", "cache-rework", "--force"],
        );
        assert!(
            discarded.status.success(),
            "[{label}] the named abandon must run; stderr:\n{}",
            String::from_utf8_lossy(&discarded.stderr),
        );
        let after = run_uninstall(repo.path(), home.path());
        let after_err = String::from_utf8(after.stderr).expect("utf-8 stderr");
        if ghost {
            // The route said the unregistered path survives and still blocks — it does, and
            // it is now the only path named.
            assert!(
                !after.status.success() && after_err.contains("uninstall.dirty-worktree"),
                "[{label}] the unregistered path must still block; stderr:\n{after_err}",
            );
            assert!(
                after_err.contains("precious.txt") && !after_err.contains("area-low"),
                "[{label}] the refusal must now name only the path the abandon left; \
                 stderr:\n{after_err}",
            );
            take_the_work_out(repo.path(), &repo.path().join("rescued"), "area-ghost");
            let cleared = run_uninstall(repo.path(), home.path());
            assert!(
                cleared.status.success(),
                "[{label}] taking the leftover out must clear the block; stderr:\n{}",
                String::from_utf8_lossy(&cleared.stderr),
            );
        } else {
            assert!(
                after.status.success(),
                "[{label}] following the abandon arm must clear the block; stderr:\n{after_err}",
            );
        }
        assert!(
            !repo.path().join(".jigc").exists(),
            "[{label}] the teardown must complete once the route is followed",
        );
    }
}

/// **The teardown names what its guards let through** (M46 Inc 2 T3 — the narration law at
/// this door; `design/surface-contract.md` → law 1).
///
/// The refusal probe deliberately stays `git status --porcelain` **without** `--ignored`, on
/// measured evidence: a provisioned worktree arrives tracked-only while the sub-task walk
/// tells the agent to build and test, so refusing on the ignored axis would fire on the
/// ordinary fan-out **success** path and train `--force` into reflex. A worktree holding
/// nothing but gitignored build output therefore **clears both guards** — and
/// `remove_dir_all(<repo>/.jigc)` then destroys it just as hard as anything else.
///
/// So the declared bound is *visible, not prevented*, and this is the cell where it is
/// observable: the teardown exits 0, removes the tree, and **names the ignored bytes first**.
/// It is driven **without** `--force`, which is what pins that the narration is not the
/// consent path's decoration — `--force` skips the guards, never the naming.
#[test]
fn the_clean_teardown_still_names_the_ignored_bytes_it_destroys() {
    let repo = TempDir::new("ignored");
    init_repo(repo.path());
    // The ignore rule has to be IN THE TREE: a fan-out worktree checks out the milestone's
    // base commit, so a rule sitting in the main checkout's untracked scratch never reaches
    // it. Committed before the mint, which pins that base.
    fs::write(repo.path().join(".gitignore"), "secrets.env\n").expect("write .gitignore");
    git_ok(repo.path(), &["add", ".gitignore"]);
    git_ok(repo.path(), &["commit", "-q", "-m", "ignore the secret"]);
    let home = TempDir::new("home");

    // Clean worktrees, no staged prose — neither guard has a subject.
    setup_fanout(repo.path(), home.path(), false, false);
    let ignored = worktree_dir(repo.path(), "area-low").join("secrets.env");
    fs::write(&ignored, "TOKEN=hunter2\n").expect("plant the ignored file");

    let out = run_uninstall(repo.path(), home.path());
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "an ignored-only worktree must not block the teardown — the refusal probe stays \
         `--porcelain`; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("secrets.env") && stderr.contains("ignored by git"),
        "the teardown must NAME the gitignored bytes it destroys, and not mislabel a file \
         that was never staged; stderr:\n{stderr}",
    );
    assert!(
        !repo.path().join(".jigc").exists(),
        "the teardown still removes `.jigc/` — visible, not prevented",
    );
    assert!(
        !ignored.exists(),
        "the narrated bytes really are destroyed, or this cell proves a warning over nothing",
    );
}
