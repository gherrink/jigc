//! M47 Inc 3 T2 — a **zero-contribution** milestone finalize is refused, and the landing
//! manifest **names the sub-tasks that contributed nothing**
//! (`DECISIONS.md` → 2026-07-26 M47 Increment 3 halt resolution, calls (b)(i) + (b)(ii);
//! `design/finalize.md` → Empty commit — the degrade statement).
//!
//! The settled *"refuse when a milestone **recorded as provisioned** has no live worktrees"*
//! was **withdrawn as unbuildable**: no such record exists (`provision_worktrees` persists
//! nothing beyond git's own registration, and a `.jigc`-local marker fails open **by
//! construction** on the fresh-clone case it exists to catch). Its only fresh-clone-durable
//! home is the committed `milestone-record` — a frozen, manifest-governed doctype, i.e. a
//! one-way door out of charter (→ M46). Two no-new-state replacements land instead, and this
//! suite is their acceptance:
//!
//! **(b)(i) the refusal.** A boundary that would land **no work** — nothing in the plan's
//! promote set, nothing staged in any sub-task worktree — commits *only jigc's own
//! bookkeeping*. The motivating shape is the fresh clone, reproduced live by the halt
//! resolution: clone → `finalize` → **exit 0**, zero work landed, and the record flipped to the
//! terminal `joined` so the milestone can never be re-finalized. That now blocks (exit 3), the
//! flip restored by the `RecordFlipGuard`, with a route naming both honest exits (provision +
//! execute + re-run, or `jigc milestone discard`).
//!
//! **(b)(ii) the visible degrade.** A sub-task with **no provisioned worktree** can contribute
//! no code *by construction* — the worktree is the sole copy of a sub-agent's code since M31
//! Inc 4/5 — so the landing manifest names it, on the agent text **and** the `--format json`
//! envelope, instead of crediting silence.
//!
//! Four arms, all driving the REAL binary:
//!
//!   (a) a **fresh clone** carrying an active milestone record → `finalize` exits 3, nothing
//!       is committed, the record is `active` on disk **and** at HEAD, and the finding's
//!       mechanical route span parses against the real CLI;
//!   (a2) the **contribution-channel axis**: a milestone whose sub-task staged only its
//!       **transient** `commit:<sub-id>` doc — materialized, never promoted — lands nothing
//!       either, and is refused the same way, iterated over both pack contexts (with a
//!       committed record and without) so the message's record-specific consequence clause is
//!       proven present where it holds and absent where it does not;
//!   (b) a genuine **docs-only** milestone (merged docs, no code) still lands at exit 0 — the
//!       no-false-fire arm, because a docs-only milestone is a legitimate shape;
//!   (c) a landed fan-out with one **never-provisioned** sub-task names that sub-task in the
//!       manifest on both `agent` and `--format json`.
//!
//! (a2) is why the refusal keys on the **plan's promote set** rather than on `has_diff`'s
//! materialized-address term: a transient doc satisfies the latter while landing nothing, so
//! the settled reading would have left the identical zero-work commit reachable one channel
//! over (verified live before the widening — exit 0, record burned to `joined`).
//!
//! **Why no `squash`-knob arm.** Both fixes are **knob-independent by construction**, not by
//! coverage: the refusal sits between the planner and the `squash` branch, and the manifest's
//! contribution facts are computed once, above it, and handed to both commit channels — so
//! there is one seam per fix, not one per mode (the fix-shape hierarchy: unify the seam >
//! iterate the members). The pack axis is not skipped, though — it is arm (a2)'s loop, because
//! the refusal's *message* is context-fed on whether a record exists; an all-empty dev-only
//! milestone still reports the planner's own empty-commit block, which keeps precedence.

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-zero-contribution-{tag}-{}-{:?}",
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

/// The `[dev ▸ methodology]` compose marker — written **before** the initial commit so a
/// clone carries it (the marker is the project's committed jigc config, and the milestone's
/// base pin must not trail HEAD by a config commit).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Initialize a real git repo with one commit (the milestone mint reads HEAD).
fn init_repo(repo: &Path, methodology: bool) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    if methodology {
        write_compose_marker(repo);
    }
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack supersedes).
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

fn head_sha(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD"]).trim().to_string()
}

fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("commit count parses")
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_rel() -> &'static str {
    "docs/milestone-records/cache-rework.md"
}

/// Stage a doc body + its provenance bit into a sub-task's `tasks/<sub>/docs/` area — the
/// managed docs a fanned-out sub-agent authors (the [`milestone_abort_survives`] fixture idiom).
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

/// Stage a sub-task's authored `commit:<sub>` doc — a **transient** doc (the `commit` doctype
/// has neither `location` nor `placement`, so it is materialized by the join and never promoted
/// to the store). The whole point of arm (a2): it makes `has_diff`'s doc term true while
/// landing nothing.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body);
}

/// Write + `git add` a code file **in** a provisioned fan-out worktree.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// The first `` `…` `` span of `text`, split into an argv — the *emitted* route bytes an
/// agent would copy, never a reconstruction.
fn first_command_span(text: &str) -> Vec<String> {
    let (_, rest) = text.split_once('`').expect("the route carries a `…` span");
    let (span, _) = rest.split_once('`').expect("the `…` span closes");
    span.split_whitespace().map(str::to_owned).collect()
}

// ---------------------------------------------------------------------------------------
// (a) The fresh clone — a zero-contribution finalize is refused, not silently landed.
// ---------------------------------------------------------------------------------------

/// A `[dev ▸ methodology]` repo whose milestone record is committed and **active**, with no
/// merged docs and no provisioned worktrees: `create → add-task ×2`. Returns nothing — the
/// caller clones it.
fn setup_recorded_milestone(repo: &Path, home: &Path) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`jigc milestone create`",
    );
    for intent in ["Warm the read cache", "Evict cold entries"] {
        assert_ok(
            &run_milestone(repo, home, &["add-task", "cache-rework", intent]),
            "`jigc milestone add-task`",
        );
    }
}

#[test]
fn a_fresh_clone_cannot_silently_land_a_zero_contribution_finalize() {
    let origin = TempDir::new("origin");
    let home = TempDir::new("home");
    init_repo(origin.path(), true);
    setup_recorded_milestone(origin.path(), home.path());

    // A genuinely fresh clone: the committed record + config only — the gitignored `.jigc/`
    // workbench (milestone area, sub-task areas, worktrees) does not exist, exactly as it
    // does not for a teammate who clones the repo.
    let workdir = TempDir::new("clone");
    let clone = workdir.path().join("clone");
    git(
        workdir.path(),
        &[
            "clone",
            "-q",
            &origin.path().display().to_string(),
            &clone.display().to_string(),
        ],
    );
    git(&clone, &["config", "user.email", "test@example.com"]);
    git(&clone, &["config", "user.name", "Test"]);
    assert!(
        !clone.join(".jigc").join("milestones").exists(),
        "the fresh clone must not carry the gitignored milestone workbench",
    );

    let before_head = head_sha(&clone);
    let before_count = commit_count(&clone);
    let record_abs = clone.join(record_rel());
    let before_record = fs::read_to_string(&record_abs).expect("the clone carries the record");

    // --- the refusal, read as machine output --------------------------------------------
    let blocked = run_jigc(
        &clone,
        home.path(),
        &["--format", "json", "milestone", "finalize", "cache-rework"],
    );
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "a zero-contribution finalize must block (exit 3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let report: serde_json::Value =
        serde_json::from_slice(&blocked.stdout).expect("the blocked envelope is valid JSON");
    let finding = report["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .find(|f| f["code"] == "milestone.zero-contribution")
        .unwrap_or_else(|| panic!("the refusal names itself; got:\n{report:#}"));
    assert_eq!(
        finding["severity"], "blocking",
        "the zero-contribution refusal is blocking; got:\n{finding:#}",
    );
    assert_eq!(
        finding["key"]["target"], "milestone:cache-rework",
        "the finding keys on the milestone; got:\n{finding:#}",
    );

    // The route's mechanical span is copy-runnable: parse the EMITTED bytes against the real
    // CLI (never a hand-built equivalent).
    let route = finding["route"]
        .as_str()
        .expect("the refusal carries a route");
    let argv = first_command_span(route);
    cli::cli::Cli::try_parse_from(&argv)
        .unwrap_or_else(|err| panic!("the route span `{}` must parse: {err}", argv.join(" ")));
    assert_eq!(
        argv,
        vec!["jigc", "milestone", "provision", "cache-rework"],
        "the route leads with the provision step; got: {route}",
    );
    assert!(
        route.contains("jigc milestone discard cache-rework"),
        "the route must name the other honest exit (settle the record); got: {route}",
    );

    // --- nothing landed, and the record is still `active` on both sides -------------------
    assert_eq!(
        head_sha(&clone),
        before_head,
        "a refused finalize must leave HEAD untouched",
    );
    assert_eq!(
        commit_count(&clone),
        before_count,
        "a refused finalize must commit nothing",
    );
    let after_record = fs::read_to_string(&record_abs).expect("the record survives the refusal");
    assert_eq!(
        after_record, before_record,
        "the `RecordFlipGuard` must restore the pre-flip bytes on the refusal",
    );
    assert!(
        !after_record.contains("status: joined"),
        "the on-disk record must stay `active` — a refused milestone is re-finalizable; got:\n{after_record}",
    );
    let committed_record = git(&clone, &["show", &format!("HEAD:{}", record_rel())]);
    assert!(
        !committed_record.contains("status: joined"),
        "the record at HEAD must stay `active`; got:\n{committed_record}",
    );

    // The agent-text surface says the same thing, with its route on stderr.
    let plain = run_milestone(&clone, home.path(), &["finalize", "cache-rework"]);
    assert_eq!(
        plain.status.code(),
        Some(3),
        "the refusal is stable across re-runs (nothing landed to change it)",
    );
    let stderr = String::from_utf8_lossy(&plain.stderr).to_string();
    assert!(
        stderr.contains("route: `jigc milestone provision cache-rework`"),
        "the agent-text refusal must carry its route; got:\n{stderr}",
    );
}

// ---------------------------------------------------------------------------------------
// (a2) The contribution-channel axis — a transient-only boundary lands nothing either.
// ---------------------------------------------------------------------------------------

#[test]
fn a_boundary_whose_only_doc_is_transient_is_refused_in_both_pack_contexts() {
    // The axis: with a committed milestone record (`[dev ▸ methodology]`) and without (dev-only,
    // where no record exists to flip). The refusal must fire in both — it keys on what would
    // LAND, not on the record — and its consequence clause must tell the truth in each.
    for methodology in [true, false] {
        let label = if methodology {
            "[dev ▸ methodology]"
        } else {
            "dev-only"
        };
        let repo = TempDir::new(if methodology {
            "transient-m"
        } else {
            "transient-d"
        });
        let home = TempDir::new("home");
        init_repo(repo.path(), methodology);

        assert_ok(
            &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
            "`jigc milestone create`",
        );
        assert_ok(
            &run_milestone(
                repo.path(),
                home.path(),
                &["add-task", "cache-rework", "Area low"],
            ),
            "`jigc milestone add-task`",
        );
        // The sub-agent authored its commit message and staged NOTHING else — no persisted doc,
        // no worktree, no code. The commit doc is materialized by the join and never promoted.
        stage_subtask_commit(repo.path(), "area-low", "rework the low cache path");

        let before_count = commit_count(repo.path());
        let blocked = run_jigc(
            repo.path(),
            home.path(),
            &["--format", "json", "milestone", "finalize", "cache-rework"],
        );
        assert_eq!(
            blocked.status.code(),
            Some(3),
            "[{label}] a transient-only boundary lands nothing and must block; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&blocked.stdout),
            String::from_utf8_lossy(&blocked.stderr),
        );
        assert_eq!(
            commit_count(repo.path()),
            before_count,
            "[{label}] the refused boundary must commit nothing",
        );

        let report: serde_json::Value =
            serde_json::from_slice(&blocked.stdout).expect("the blocked envelope is valid JSON");
        let finding = report["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .find(|f| f["code"] == "milestone.zero-contribution")
            .unwrap_or_else(|| panic!("[{label}] the refusal names itself; got:\n{report:#}"));
        let message = finding["message"].as_str().expect("the finding's message");
        // The record-specific consequence is stated exactly where it holds — a
        // context-independent clause would render the same lie on every dev-only boundary.
        assert_eq!(
            message.contains("terminal `joined`"),
            methodology,
            "[{label}] the consequence clause must match the pack context; got: {message}",
        );
    }
}

// ---------------------------------------------------------------------------------------
// (b) The no-false-fire arm — a genuine docs-only milestone still lands.
// ---------------------------------------------------------------------------------------

#[test]
fn a_docs_only_milestone_still_lands_at_exit_zero() {
    let repo = TempDir::new("docs-only");
    let home = TempDir::new("home");
    init_repo(repo.path(), true);
    setup_recorded_milestone(repo.path(), home.path());

    // Real merged docs, no code and no worktrees — the legitimate docs-only shape the
    // refusal must NOT touch (its `has_diff` does not rest on the record flip alone).
    stage_doc(
        repo.path(),
        "warm-the-read-cache",
        "adr:warm-policy",
        &adr_plain("Warm policy"),
    );

    let before_count = commit_count(repo.path());
    let landed = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert_ok(&landed, "a docs-only `jigc milestone finalize`");
    assert_eq!(
        commit_count(repo.path()),
        before_count + 1,
        "the docs-only boundary lands exactly one commit",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.contains("docs/decisions/warm-policy.md"),
        "the merged doc must land; got:\n{committed}",
    );
    let record = git(repo.path(), &["show", &format!("HEAD:{}", record_rel())]);
    assert!(
        record.contains("status: joined") && !record.contains("status: active"),
        "the landed boundary still folds the join flip; got:\n{record}",
    );
}

// ---------------------------------------------------------------------------------------
// (c) The visible degrade — a never-provisioned sub-task is named, both surfaces.
// ---------------------------------------------------------------------------------------

/// A dev-only fan-out where `area-low` is provisioned (and stages code) and `area-zed` is
/// added **after** the provision, so it never gets a worktree — the degrade that used to be
/// silent. Both sub-tasks author a doc, so `area-zed`'s line proves the marker rides a
/// *contributing* sub-task too, not only the empty one.
fn setup_partial_fanout(repo: &Path, home: &Path) {
    assert_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "`jigc milestone create`",
    );
    assert_ok(
        &run_milestone(repo, home, &["add-task", "cache-rework", "Area low"]),
        "add-task `Area low`",
    );
    assert_ok(
        &run_milestone(repo, home, &["provision", "cache-rework"]),
        "`jigc milestone provision`",
    );
    assert_ok(
        &run_milestone(repo, home, &["add-task", "cache-rework", "Area zed"]),
        "add-task `Area zed` (after the provision — never gets a worktree)",
    );
    stage_doc(repo, "area-low", "adr:low-policy", &adr_plain("Low policy"));
    stage_doc(repo, "area-zed", "adr:zed-policy", &adr_plain("Zed policy"));
    stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
}

#[test]
fn the_landing_manifest_names_the_never_provisioned_sub_task() {
    // --- the agent-text surface ---------------------------------------------------------
    let repo = TempDir::new("degrade-agent");
    let home = TempDir::new("home");
    init_repo(repo.path(), false);
    setup_partial_fanout(repo.path(), home.path());

    let landed = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert_ok(&landed, "the partial fan-out `jigc milestone finalize`");
    let stdout = String::from_utf8_lossy(&landed.stdout).to_string();
    assert!(
        stdout.contains(
            "sub-tasks: area-low: 1 doc, 1 code file · area-zed: 1 doc, no worktree provisioned"
        ),
        "the manifest must name the never-provisioned sub-task's degrade; got:\n{stdout}",
    );

    // --- the same fact on the machine surface -------------------------------------------
    let repo = TempDir::new("degrade-json");
    let home = TempDir::new("home");
    init_repo(repo.path(), false);
    setup_partial_fanout(repo.path(), home.path());

    let landed = run_jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "milestone", "finalize", "cache-rework"],
    );
    assert_ok(&landed, "the partial fan-out finalize (`--format json`)");
    let value: serde_json::Value =
        serde_json::from_slice(&landed.stdout).expect("the landed envelope is valid JSON");
    let sub_tasks = value["committed"]["sub_tasks"]
        .as_array()
        .expect("the committed envelope carries `sub_tasks`");
    let low = sub_tasks
        .iter()
        .find(|s| s["id"] == "area-low")
        .expect("area-low is named");
    let zed = sub_tasks
        .iter()
        .find(|s| s["id"] == "area-zed")
        .expect("area-zed is named");
    assert_eq!(
        low["provisioned"], true,
        "the provisioned sub-task reads `provisioned: true`; got:\n{low:#}",
    );
    assert_eq!(
        low["code_files"], 1,
        "the provisioned sub-task's staged code is counted; got:\n{low:#}",
    );
    assert_eq!(
        zed["provisioned"], false,
        "the never-provisioned sub-task reads `provisioned: false` — the degrade is data, not \
         only prose; got:\n{zed:#}",
    );
    assert_eq!(
        zed["code_files"], 0,
        "a sub-task with no worktree contributes no code BY CONSTRUCTION; got:\n{zed:#}",
    );
}
