//! **N12 — the landing ack names every commit the boundary made, and attributes every
//! landed file to the sha that landed it** (M50 Increment 11 / T1;
//! `design/finalize.md` → `fan-out` finalize — the two squash modes;
//! `design/command-output-contract.md` → the pre-1.0 additive-key window).
//!
//! Driven at `aa98d1b` before the fix: a two-sub-task `squash: false` milestone lands
//! **three** commits, and the `--format json` envelope reported `hash` = the aggregate
//! alone beside `files: 6` and a six-entry `manifest` **of which that sha owns four** —
//! the two per-sub-task shas, *the entire product of `squash: false`*, appeared nowhere,
//! and `sub_tasks[]` carried no `hash`. That envelope is exactly what a fix-round
//! orchestrator reads to find the commit a given fix landed in, so the ack was not
//! merely thin: it **attributed another commit's files to a sha that does not contain
//! them**.
//!
//! The fix is additive by rule — no pinned key changes meaning. `manifest` and `files`
//! keep their boundary-wide membership (`git diff <pre-boundary-HEAD> HEAD`, one entry
//! per path) and gain **attribution** through the new `commits` array; `sub_tasks[]`
//! gains its own `hash`.
//!
//! The suite drives the REAL binary over the **cell set** the ack's shape turns on —
//! `{squash: false × code-carrying sub-tasks, squash: false × code-less sub-tasks,
//! squash: true}` — and asserts, per cell:
//!
//!   (a) **completeness** — every sha `git rev-list <pre-boundary-HEAD>..HEAD` yields
//!       appears in the envelope, and no more;
//!   (b) **attribution** — each `manifest` entry's owning sha (the last `commits[]`
//!       member listing it) genuinely contains that path, cross-read from
//!       `git show --name-only <sha>` rather than from the envelope's own claim;
//!   (c) **the per-sub-task sha** — present and its own commit in the code-carrying
//!       cell, `null` in the two cells that mint no per-sub-task commit;
//!   (d) **the agent-text arm prints the same shas** — the machine and prose channels
//!       name one commit set, not two.
//!
//! Plus the multi-commit-path arm: a path landed by **more than one** chain commit still
//! yields exactly ONE `manifest` entry, and its owning sha is the **last** commit that
//! landed it — the sha whose bytes are at HEAD.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-landed-attribution-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit, opting the project into `squash`.
fn init_repo(repo: &Path, squash: bool) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    let config = repo.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk config layer");
    fs::write(
        config.join("manifest.yaml"),
        format!("scalar:\n  finalize.fan-out.squash: {squash}\n"),
    )
    .expect("write manifest");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
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
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## \
         Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## \
         Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a sub-task's authored transient `commit:<sub>` doc — the prose each per-sub-task
/// commit carries under `squash: false`.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body);
}

/// Write + `git add` a file **in** a provisioned fan-out worktree.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// The repo-relative paths one commit changed, read from git itself — never from the
/// envelope's own claim, which is the thing under test.
fn commit_changed_files(repo: &Path, rev: &str) -> Vec<String> {
    git(repo, &["show", "--name-only", "--format=", rev])
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

/// Every commit the boundary landed, oldest first, as FULL shas — git's own answer.
fn boundary_shas(repo: &Path, pre: &str) -> Vec<String> {
    git(repo, &["rev-list", "--reverse", &format!("{pre}..HEAD")])
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

// ─────────────────────────────── the cell set ───────────────────────────────

/// The three cells the landing ack's commit shape turns on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    /// `squash: false`, every sub-task staged code — N per-sub-task commits + the
    /// aggregate. The cell the ack was wrong on.
    ChainWithCode,
    /// `squash: false`, no sub-task staged code — the chain degrades to the docs-only
    /// aggregate, so no sub-task owns a commit.
    ChainWithoutCode,
    /// `squash: true` — one aggregate commit whose files ARE the manifest.
    Squashed,
}

impl Cell {
    fn tag(self) -> &'static str {
        match self {
            Cell::ChainWithCode => "false-code",
            Cell::ChainWithoutCode => "false-docs",
            Cell::Squashed => "true",
        }
    }

    fn squash(self) -> bool {
        self == Cell::Squashed
    }

    fn carries_code(self) -> bool {
        self != Cell::ChainWithoutCode
    }

    /// How many commits the boundary lands: N code-carrying sub-tasks + the aggregate
    /// under `squash: false`; one aggregate under `squash: true`.
    fn expected_commits(self) -> usize {
        match self {
            Cell::ChainWithCode => 3,
            Cell::ChainWithoutCode | Cell::Squashed => 1,
        }
    }
}

/// Mint `Cache rework` + two sub-tasks, stage each one's merged ADR + authored commit
/// doc, provision the worktrees, and (per cell) stage disjoint code in each. Returns the
/// pre-boundary HEAD.
fn setup(repo: &Path, home: &Path, cell: Cell) -> String {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`jigc milestone create`",
    );
    for intent in ["Area low", "Area zed"] {
        assert_ok(
            &run_milestone(repo, home, &["add-task", "cache-rework", intent]),
            "`jigc milestone add-task`",
        );
    }
    stage_doc(repo, "area-low", "adr:low-policy", &adr_plain("Low policy"));
    stage_doc(repo, "area-zed", "adr:zed-policy", &adr_plain("Zed policy"));
    stage_subtask_commit(repo, "area-low", "rework the low cache path");
    stage_subtask_commit(repo, "area-zed", "rework the zed cache path");
    assert_ok(
        &run_milestone(repo, home, &["provision", "cache-rework"]),
        "`jigc milestone provision`",
    );
    if cell.carries_code() {
        stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
        stage_worktree_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
    }
    git(repo, &["rev-parse", "HEAD"]).trim().to_owned()
}

/// The envelope's `commits[]` member whose `hash` prefixes `full` — the bijection (a)
/// asserts.
fn commit_for<'a>(commits: &'a [serde_json::Value], full: &str) -> Option<&'a serde_json::Value> {
    commits.iter().find(|c| {
        c["hash"]
            .as_str()
            .is_some_and(|h| !h.is_empty() && full.starts_with(h))
    })
}

#[test]
fn the_landing_ack_names_every_boundary_commit_and_attributes_every_landed_file() {
    for cell in [Cell::ChainWithCode, Cell::ChainWithoutCode, Cell::Squashed] {
        let repo = TempDir::new(cell.tag());
        let home = TempDir::new("home");
        init_repo(repo.path(), cell.squash());
        let pre = setup(repo.path(), home.path(), cell);

        let landed = run_jigc(
            repo.path(),
            home.path(),
            &["--format", "json", "milestone", "finalize", "cache-rework"],
        );
        assert_ok(&landed, &format!("[{cell:?}] `jigc milestone finalize`"));
        let value: serde_json::Value =
            serde_json::from_slice(&landed.stdout).expect("the landed envelope is valid JSON");
        let committed = &value["committed"];

        // The fixture must actually BE the cell it claims — otherwise every assertion
        // below is vacuous over a shape the cell never reached.
        let shas = boundary_shas(repo.path(), &pre);
        assert_eq!(
            shas.len(),
            cell.expected_commits(),
            "[{cell:?}] the fixture must land {} commit(s); git says {shas:?}",
            cell.expected_commits(),
        );

        // ---- (a) completeness: every sha git lists appears in the envelope, and no more.
        let commits = committed["commits"].as_array().unwrap_or_else(|| {
            panic!("[{cell:?}] the landed envelope carries `commits`; got:\n{committed:#}")
        });
        assert_eq!(
            commits.len(),
            shas.len(),
            "[{cell:?}] the envelope must name every boundary commit and no more; git: {shas:?}, \
             envelope:\n{commits:#?}",
        );
        for (index, full) in shas.iter().enumerate() {
            let entry = commit_for(commits, full).unwrap_or_else(|| {
                panic!(
                    "[{cell:?}] the boundary commit {full} appears nowhere in the envelope; \
                     got:\n{commits:#?}"
                )
            });
            // Oldest-first, matching the order the chain laid them down.
            assert!(
                full.starts_with(commits[index]["hash"].as_str().expect("a `hash` string")),
                "[{cell:?}] `commits` must be oldest-first (git: {shas:?}); got:\n{commits:#?}",
            );
            let subject = git(repo.path(), &["show", "--no-patch", "--format=%s", full]);
            assert_eq!(
                entry["subject"].as_str().unwrap_or_default(),
                subject.trim(),
                "[{cell:?}] each commit carries its own subject; got:\n{entry:#}",
            );
        }
        // The header hash stays the boundary's HEAD — the aggregate, i.e. the LAST commit.
        assert_eq!(
            committed["hash"],
            commits[commits.len() - 1]["hash"],
            "[{cell:?}] `hash` is the boundary's HEAD, which is the last commit it made",
        );

        // ---- (b) attribution: each manifest entry's owning sha really contains that path.
        let manifest = committed["manifest"]
            .as_array()
            .unwrap_or_else(|| panic!("[{cell:?}] the envelope carries `manifest`"));
        assert_eq!(
            committed["files"].as_u64(),
            Some(manifest.len() as u64),
            "[{cell:?}] `files` stays the boundary-wide membership count",
        );
        assert!(
            manifest.len() >= 3,
            "[{cell:?}] the fixture must land a manifest worth attributing; got:\n{manifest:#?}",
        );
        for entry in manifest {
            let path = entry["path"].as_str().expect("a manifest `path`");
            // The stated resolution rule: the OWNING sha is the LAST commit that landed
            // the path — the one whose bytes are at HEAD.
            let owner = commits
                .iter()
                .rev()
                .find(|c| {
                    c["paths"]
                        .as_array()
                        .is_some_and(|ps| ps.iter().any(|p| p == path))
                })
                .unwrap_or_else(|| {
                    panic!(
                        "[{cell:?}] no commit in the envelope claims the landed path `{path}`; \
                         got:\n{commits:#?}"
                    )
                });
            let owner_hash = owner["hash"].as_str().expect("an owning `hash`");
            // Cross-read from GIT, never from the envelope's own claim.
            let real = commit_changed_files(repo.path(), owner_hash);
            assert!(
                real.iter().any(|p| p == path),
                "[{cell:?}] `{path}` is attributed to {owner_hash}, which does not contain it; \
                 that commit really landed:\n{real:?}",
            );
        }

        // ---- (c) the per-sub-task sha.
        let sub_tasks = committed["sub_tasks"]
            .as_array()
            .unwrap_or_else(|| panic!("[{cell:?}] the envelope carries `sub_tasks`"));
        assert_eq!(sub_tasks.len(), 2, "[{cell:?}] both sub-tasks are named");
        for sub in sub_tasks {
            let id = sub["id"].as_str().expect("a sub-task `id`");
            let hash = sub.get("hash").unwrap_or_else(|| {
                panic!("[{cell:?}] `sub_tasks[]` carries `hash`; got:\n{sub:#}")
            });
            if cell == Cell::ChainWithCode {
                let hash = hash.as_str().unwrap_or_else(|| {
                    panic!(
                        "[{cell:?}] `{id}` owns a commit, so its `hash` is a string; got:\n{sub:#}"
                    )
                });
                let files = commit_changed_files(repo.path(), hash);
                let expected = if id == "area-low" {
                    "src/low.rs"
                } else {
                    "src/zed.rs"
                };
                assert_eq!(
                    files,
                    vec![expected.to_owned()],
                    "[{cell:?}] `{id}`'s `hash` must be ITS OWN commit; {hash} landed:\n{files:?}",
                );
                // …and it is one of the boundary's commits, not some other sha.
                assert!(
                    shas.iter().any(|full| full.starts_with(hash)),
                    "[{cell:?}] `{id}`'s `hash` must be a boundary commit; git: {shas:?}",
                );
            } else {
                assert!(
                    hash.is_null(),
                    "[{cell:?}] `{id}` mints no commit of its own, so `hash` is null; got:\n{sub:#}",
                );
            }
        }

        // ---- (d) the agent-text arm prints the same shas.
        let repo = TempDir::new(&format!("{}-text", cell.tag()));
        let home = TempDir::new("home");
        init_repo(repo.path(), cell.squash());
        let pre = setup(repo.path(), home.path(), cell);
        let landed = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
        assert_ok(&landed, &format!("[{cell:?}] the agent-text finalize"));
        let text = String::from_utf8_lossy(&landed.stdout).to_string();
        for full in boundary_shas(repo.path(), &pre) {
            let short = git(repo.path(), &["rev-parse", "--short", &full])
                .trim()
                .to_owned();
            assert!(
                text.contains(&short),
                "[{cell:?}] the agent text must name the boundary commit {short}; got:\n{text}",
            );
        }
    }
}

#[test]
fn a_path_landed_by_two_chain_commits_is_one_manifest_entry_owned_by_the_last() {
    // A sub-agent hand-wrote a file at a managed doc's destination and `git add`ed it in
    // its worktree: the per-sub-task commit lands that path, and the aggregate lands it
    // again with the promoted canonical bytes. The boundary-wide manifest must still
    // carry ONE entry for it, and the owning sha must be the LAST commit that landed it
    // — the sha whose bytes are at HEAD.
    let repo = TempDir::new("twice");
    let home = TempDir::new("home");
    init_repo(repo.path(), false);
    let pre = setup(repo.path(), home.path(), Cell::ChainWithCode);
    let shared = "docs/decisions/low-policy.md";
    stage_worktree_code(repo.path(), "area-low", shared, "# placeholder\n");

    let landed = run_jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "milestone", "finalize", "cache-rework"],
    );
    assert_ok(&landed, "the overlapping-path `jigc milestone finalize`");
    let value: serde_json::Value =
        serde_json::from_slice(&landed.stdout).expect("the landed envelope is valid JSON");
    let committed = &value["committed"];

    let commits = committed["commits"]
        .as_array()
        .expect("the landed envelope carries `commits`");
    let claiming: Vec<&str> = commits
        .iter()
        .filter(|c| {
            c["paths"]
                .as_array()
                .is_some_and(|ps| ps.iter().any(|p| p == shared))
        })
        .map(|c| c["hash"].as_str().expect("a `hash`"))
        .collect();
    assert_eq!(
        claiming.len(),
        2,
        "the fixture must land `{shared}` in TWO chain commits (else the rule is untested); \
         got:\n{commits:#?}",
    );

    let entries: Vec<&serde_json::Value> = committed["manifest"]
        .as_array()
        .expect("the envelope carries `manifest`")
        .iter()
        .filter(|e| e["path"] == shared)
        .collect();
    assert_eq!(
        entries.len(),
        1,
        "a path landed by two chain commits is ONE manifest entry; got:\n{entries:#?}",
    );

    // The owning sha — the last claimant — is the boundary's HEAD, and git agrees it
    // contains the path.
    let owner = claiming[claiming.len() - 1];
    assert_eq!(
        committed["hash"], owner,
        "the last commit to land `{shared}` is the aggregate at HEAD",
    );
    assert!(
        commit_changed_files(repo.path(), owner)
            .iter()
            .any(|p| p == shared),
        "the owning sha must genuinely contain `{shared}`",
    );
    // And the boundary really did make more than one commit (the cell is not degenerate).
    assert!(
        boundary_shas(repo.path(), &pre).len() > 1,
        "the overlapping-path arm must run over a multi-commit boundary",
    );
}
