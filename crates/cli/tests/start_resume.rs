//! End-to-end integration test for `jigc start --task <id>` resume — the inc-5
//! `as:` role-binding round-trip (`DECISIONS.md` 2026-05-31 → inc-5 `as:` role
//! binding at create; `worked-examples.md` → Superseding decision: the agent
//! re-composes to pick up context now that the ADR is bound to `task.decision`).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo: an intent
//! mints a task and composes (the `superseded-context` step's
//! `{{@task.decision.supersedes#decision}}` is **empty** — `task.decision` is
//! declared-but-unbound). `jigc doc create adr --title "…"` binds the created
//! ADR to `task.decision` (records `.jigc/tasks/<id>/roles.json`). On resume,
//! `jigc start --task <id>` reads that binding back, so the same placeholder now
//! resolves to the bound ADR's address — observably no longer absent.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init`, and a self-cleaning
//! `TempDir` keeps the test off the developer's real repo / `~/.config`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-start-resume-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (composition mints, which reads HEAD).
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

#[test]
fn resume_re_composes_with_the_created_adr_bound_to_task_decision() {
    let repo = TempDir::new("bind");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // 1. Mint + compose. `task.decision` is declared-but-unbound, so the
    //    superseded-context placeholder resolves to empty text — the address is
    //    nowhere in the composed view.
    let first = run(
        repo.path(),
        home.path(),
        &["start", "move the cache to redis"],
    );
    assert!(
        first.status.success(),
        "the initial mint+compose must succeed; stderr:\n{}",
        String::from_utf8_lossy(&first.stderr),
    );
    let first_out = String::from_utf8(first.stdout).expect("utf-8 stdout");
    let slug = "move-the-cache-to-redis";
    assert!(
        !first_out.contains("> adr:"),
        "before any ADR is created, task.decision is unbound — no `> adr:` slice; got:\n{first_out}",
    );

    // 2. Create the ADR — the create-gate binds it to `task.decision`
    //    (roles.json records `decision -> adr:<slug>`).
    let create = run(
        repo.path(),
        home.path(),
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Shared Redis session cache",
        ],
    );
    assert!(
        create.status.success(),
        "`jigc doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr),
    );
    let adr_addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr_addr, "adr:shared-redis-session-cache");

    // The bind landed on disk: .jigc/tasks/<id>/roles.json maps decision -> the ADR.
    let roles_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(slug)
        .join("roles.json");
    let roles = fs::read_to_string(&roles_path).expect("roles.json written by the create-gate");
    assert!(
        roles.contains("\"decision\": \"adr:shared-redis-session-cache\""),
        "roles.json must bind decision -> the created ADR; got:\n{roles}",
    );

    // 3. Resume: `jigc start --task <id>` reads roles.json back, so the same
    //    superseded-context placeholder now resolves to the bound ADR — no
    //    longer Absent.
    let resume = run(repo.path(), home.path(), &["start", "--task", slug]);
    assert!(
        resume.status.success(),
        "`jigc start --task <id>` resume must succeed; stderr:\n{}",
        String::from_utf8_lossy(&resume.stderr),
    );
    let resume_out = String::from_utf8(resume.stdout).expect("utf-8 stdout");
    assert!(
        resume_out.contains("> adr:shared-redis-session-cache#decision"),
        "on resume, task.decision is bound — the superseded-context slice resolves \
         to the created ADR's address; got:\n{resume_out}",
    );
}
