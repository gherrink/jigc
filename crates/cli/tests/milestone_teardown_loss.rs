//! The **landed** fan-out teardown names the work it discards (M47 Inc 3 T4 — the law-1
//! minimum of `DECISIONS.md` → 2026-07-26 M47 Increment 3 halt resolution, call (c)).
//!
//! Both landed arms commit only the **staged** set (`worktree_staged_patch`) and then
//! `git worktree remove --force` the whole checkout — so a sub-agent's unstaged and
//! untracked bytes were destroyed at **exit 0, unnarrated, on the success path**. The
//! codebase already knew this: `dirty_worktrees` reads `git status --porcelain` precisely
//! because the teardown destroys *"staged, unstaged, and untracked alike"*, and `discard`
//! guards that set behind `--force` while `finalize` guarded nothing.
//!
//! **Honest bound, recorded not glossed:** this makes the loss **visible, not prevented**.
//! The `discard`-style refusal + `--force` on the landed path is new surface and is
//! chartered to M46 (`implementation/decisions-pending.md` → the capability wave).
//!
//! **The acceptance iterates two axes, not the reported repro:**
//!
//! 1. the **porcelain index-column partition** — a *wholly staged* path (`A `), a *never
//!    staged* path (`??`), a *partly staged* path (`MM`, the cell whose staged half
//!    lands and whose unstaged half does not) and — since M46 Inc 2 T2 — an **ignored**
//!    path (`!!`, which the teardown destroys exactly as hard as the others) live in one
//!    worktree at once. Only the latter three may be named: naming the wholly-staged path
//!    would be the over-report the M47 completion audit caught, fixed here at birth rather
//!    than shipped-then-patched;
//!
//!    The ignored cell is reported at the **matching level** — `build/`, the pattern the
//!    ignore rule matched, not `build/out.o` — because a `target/`/`node_modules/`
//!    inventory is the over-report `discarded_work`'s own doc-comment says makes a loss
//!    warning untrustworthy (`implementation/roadmap.md` → M46 Inc 2, deliverable (b);
//!    `completions/artifacts/M46/razor-ledger.md` §2f). **Declared bound:** a precious file
//!    nested inside an ignored directory is covered by its container's name, not listed
//!    individually;
//! 2. the **landed-arm axis** — `finalize.fan-out.squash: true` (the single combine
//!    commit) and `false` (the per-sub-task chain) each call `remove_worktrees` and each
//!    build the landing manifest, so both must narrate.
//!
//! …across both output surfaces (`agent` text and the pinned `--format json` envelope),
//! and on **both** channels the call names: the loss warning on stderr and the landing
//! manifest on stdout. (Since the M50 Increment 12 audit that warning is printed *after* the
//! removal it describes and lists only what the removal took — `cli::milestone::PendingLoss`
//! — so on this landed path, where every worktree is genuinely removed, its content is
//! unchanged.)

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-teardown-loss-{tag}-{}-{:?}",
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
    // Committed so every fan-out worktree checkout inherits the same ignore rules — the
    // `!!` cell of the partition needs a rule that is in the tree, not in the main
    // repo's untracked scratch.
    fs::write(root.join(".gitignore"), "secrets.env\nbuild/\n").expect("write .gitignore");
    git_ok(root, &["add", "."]);
    git_ok(root, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
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

fn worktree_dir(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("worktrees").join(sub)
}

/// Write + `git add` a file **in** a provisioned fan-out worktree — the `A `/`M ` cell:
/// wholly staged, so the boundary's `git diff --cached` patch carries every byte and the
/// teardown discards nothing.
fn stage_wholly(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = worktree_dir(repo, sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree parent");
    }
    fs::write(&p, body).expect("write worktree file");
    git_ok(&wt, &["add", rel]);
}

/// The `MM` cell: a **tracked** file staged and then modified again. The staged half rides
/// the boundary commit; the later edit dies with the worktree.
fn stage_partly(repo: &Path, sub: &str, rel: &str, staged: &str, then: &str) {
    stage_wholly(repo, sub, rel, staged);
    fs::write(worktree_dir(repo, sub).join(rel), then).expect("re-modify after staging");
}

/// The `!!` cell: a path git **ignores** — invisible to `git status --porcelain`, and
/// destroyed by `git worktree remove --force` exactly as hard as an untracked one.
fn leave_ignored(repo: &Path, sub: &str, rel: &str, body: &str) {
    leave_untracked(repo, sub, rel, body);
}

/// The `??` cell: never staged at all.
fn leave_untracked(repo: &Path, sub: &str, rel: &str, body: &str) {
    let p = worktree_dir(repo, sub).join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree parent");
    }
    fs::write(&p, body).expect("write untracked worktree file");
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

/// The fan-out fixture: milestone + two sub-tasks, each with a persisted ADR + its authored
/// commit doc staged, provisioned worktrees, and — in `area-low` — one cell of each of the
/// three porcelain index-column states at once.
fn setup_fanout(repo: &Path, home: &Path, squash: bool) {
    if !squash {
        set_squash_false(repo);
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

    // The three-cell worktree: wholly staged · partly staged · never staged.
    stage_wholly(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_partly(
        repo,
        "area-low",
        "README.md",
        "hello\nstaged half\n",
        "hello\nstaged half\nunstaged half\n",
    );
    leave_untracked(repo, "area-low", "notes/scratch.rs", "// sub-agent WIP\n");
    // The `!!` cells: a directly-matched ignored file, and a file *inside* an ignored
    // directory — the latter must be named by its matching-level container (`build/`),
    // never enumerated file by file.
    leave_ignored(repo, "area-low", "secrets.env", "TOKEN=hunter2\n");
    leave_ignored(repo, "area-low", "build/out.o", "OBJ\n");

    // The clean sibling: everything it holds is staged, so nothing may be reported for it.
    stage_wholly(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
}

/// The loss-warning block for one worktree — the stderr lines from the
/// `warning: removing the fan-out worktree …<sub>…` header through its indented body.
fn warning_block(stderr: &str, sub: &str) -> String {
    let mut lines = stderr.lines().skip_while(|line| {
        !(line.starts_with("warning: removing the fan-out worktree") && line.contains(sub))
    });
    let header = lines
        .next()
        .unwrap_or_else(|| panic!("no teardown loss warning for `{sub}`; stderr:\n{stderr}"));
    let mut block = header.to_owned();
    for line in lines {
        if !line.starts_with(' ') {
            break;
        }
        block.push('\n');
        block.push_str(line);
    }
    block
}

/// The landing manifest's discarded block — the `  discarded with the fan-out worktrees`
/// header through its indented body.
fn discarded_block(stdout: &str) -> String {
    let mut lines = stdout.lines().skip_while(|line| {
        !line
            .trim_start()
            .starts_with("discarded with the fan-out worktrees")
    });
    let header = lines.next().unwrap_or_else(|| {
        panic!("the landing manifest names no discarded work; stdout:\n{stdout}")
    });
    let mut block = header.to_owned();
    for line in lines {
        if !line.starts_with("    ") {
            break;
        }
        block.push('\n');
        block.push_str(line);
    }
    block
}

#[test]
fn a_landed_fan_out_finalize_names_the_work_its_teardown_discards() {
    // The landed-arm axis × the output-surface axis. Each cell is a fresh repo: a landed
    // boundary is terminal.
    for (squash, json) in [(true, false), (true, true), (false, false), (false, true)] {
        let label = format!(
            "squash: {squash}, format: {}",
            if json { "json" } else { "agent" }
        );
        let repo = TempDir::new(if squash { "squash" } else { "chain" });
        init_repo(repo.path());
        let home = TempDir::new("home");

        setup_fanout(repo.path(), home.path(), squash);

        let before_count = rev_list_count(repo.path());
        let mut args = vec!["finalize", "cache-rework"];
        if json {
            args.extend(["--format", "json"]);
        }
        let finalized = run_milestone(repo.path(), home.path(), &args);
        let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
        let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");

        // (1) The boundary landed — the narration is on the SUCCESS path, not a refusal.
        assert!(
            finalized.status.success(),
            "[{label}] the finalize must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
        );
        let landed = rev_list_count(repo.path()) - before_count;
        assert!(landed >= 1, "[{label}] the boundary must land commit(s)");

        // (2) The staged set landed — including the staged HALF of the `MM` path.
        let changed = recent_changed_files(repo.path(), landed);
        for path in ["src/low.rs", "src/zed.rs", "README.md"] {
            assert!(
                changed.iter().any(|p| p == path),
                "[{label}] the boundary must commit the staged path `{path}`; landed: {changed:?}",
            );
        }
        let readme = git_ok(repo.path(), &["show", "HEAD:README.md"]);
        assert!(
            readme.contains("staged half") && !readme.contains("unstaged half"),
            "[{label}] only the STAGED half of the partly-staged path may land; got:\n{readme}",
        );

        // (3) The loss warning names the two discarded cells and NOT the wholly
        // staged one — the over-report the completion audit caught, fixed at birth.
        let warning = warning_block(&stderr, "area-low");
        assert!(
            warning.contains("notes/scratch.rs") && warning.contains("never staged"),
            "[{label}] the warning must name the never-staged path; got:\n{warning}",
        );
        assert!(
            warning.contains("README.md") && warning.contains("staged only in part"),
            "[{label}] the warning must name the partly-staged path; got:\n{warning}",
        );
        assert!(
            !warning.contains("src/low.rs"),
            "[{label}] the warning must NOT name the wholly-staged path (it landed); got:\n{warning}",
        );
        // The `!!` cell — destroyed as hard as the rest, so it is named, at the matching
        // level and with a state that is not a lie (M46 Inc 2 T2).
        assert!(
            warning.contains("secrets.env (ignored by git)"),
            "[{label}] the warning must name the ignored file; got:\n{warning}",
        );
        assert!(
            warning.contains("build/ (ignored by git)"),
            "[{label}] the warning must name the ignored directory at its matching level; \
             got:\n{warning}",
        );
        assert!(
            !warning.contains("out.o"),
            "[{label}] an ignored directory is named, never enumerated file by file; \
             got:\n{warning}",
        );
        // The clean sibling has nothing to narrate, so it prints no warning at all.
        assert!(
            !stderr.contains("worktrees/area-zed discards"),
            "[{label}] a worktree whose whole content is staged must print no loss warning; \
             stderr:\n{stderr}",
        );

        // (4) The landing manifest carries the same set, on stdout, in both surfaces.
        if json {
            let envelope: serde_json::Value =
                serde_json::from_str(&stdout).expect("the landed envelope is valid JSON");
            let subs = envelope["committed"]["sub_tasks"]
                .as_array()
                .expect("sub_tasks is an array");
            let low = subs
                .iter()
                .find(|s| s["id"] == "area-low")
                .expect("area-low rides the manifest");
            assert_eq!(
                low["discarded"],
                serde_json::json!([
                    { "path": "README.md", "state": "partly-staged" },
                    { "path": "build/", "state": "ignored" },
                    { "path": "notes/scratch.rs", "state": "never-staged" },
                    { "path": "secrets.env", "state": "ignored" },
                ]),
                "[{label}] the JSON manifest must carry the discarded set, path-sorted, and \
                 nothing else; got:\n{stdout}",
            );
            let zed = subs
                .iter()
                .find(|s| s["id"] == "area-zed")
                .expect("area-zed rides the manifest");
            assert_eq!(
                zed["discarded"],
                serde_json::json!([]),
                "[{label}] a fully-staged sub-task discards nothing; got:\n{stdout}",
            );
        } else {
            let block = discarded_block(&stdout);
            assert!(
                block.contains("area-low")
                    && block.contains("notes/scratch.rs (never staged)")
                    && block.contains("README.md (staged only in part)")
                    && block.contains("build/ (ignored by git)")
                    && block.contains("secrets.env (ignored by git)"),
                "[{label}] the manifest's discarded block must name every reportable cell; \
                 got:\n{block}",
            );
            assert!(
                !block.contains("out.o"),
                "[{label}] an ignored directory is named, never enumerated file by file; \
                 got:\n{block}",
            );
            assert!(
                !block.contains("src/low.rs"),
                "[{label}] the manifest must NOT report the wholly-staged path as discarded; \
                 got:\n{block}",
            );
            assert!(
                !block.contains("area-zed"),
                "[{label}] a fully-staged sub-task must not appear in the discarded block; \
                 got:\n{block}",
            );
        }

        // (5) The bound is honest: visible, NOT prevented — the worktree is gone.
        assert!(
            !worktree_dir(repo.path(), "area-low").exists(),
            "[{label}] the landed teardown still removes the worktree (visible, not prevented)",
        );
    }
}

/// **Refusal stays refused** — the narration learns the `--ignored` axis, the *refusal*
/// probe deliberately does not (M46 Inc 2, "Refusal stays refused, on measured evidence";
/// `completions/artifacts/M46/razor-ledger.md` §2f). A provisioned worktree arrives
/// tracked-only *while the sub-task walk tells the agent to build the code and run the
/// tests*, so a worktree that did its job holds `target/`-shaped build output: refusing on
/// the ignored axis would fire on the ordinary fan-out **success** path and train `--force`
/// into reflex, which is strictly worse than no guard.
///
/// So `dirty_worktrees` — the refusal probe — stays `--porcelain` **without** `--ignored`,
/// and a worktree holding nothing but a gitignored file still clears `discard`'s
/// `milestone.dirty-worktree` refusal. The declared cost is stated rather than implied:
/// **the loss becomes visible, not prevented** — which is the second half this cell pins,
/// on the same door.
#[test]
fn a_gitignored_only_worktree_still_clears_the_discard_refusal() {
    let repo = TempDir::new("ignored-refusal");
    init_repo(repo.path());
    let home = TempDir::new("home");

    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    assert!(
        run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", "Area low"]
        )
        .status
        .success(),
        "add-task must exit 0",
    );
    let provisioned = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );

    // The whole content of the worktree: one gitignored file, invisible to the refusal probe.
    leave_ignored(repo.path(), "area-low", "secrets.env", "TOKEN=hunter2\n");

    let discarded = run_milestone(repo.path(), home.path(), &["discard", "cache-rework"]);
    let stdout = String::from_utf8(discarded.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(discarded.stderr).expect("utf-8 stderr");

    // (1) The refusal is UNCHANGED — the un-forced abandon still goes through.
    assert!(
        discarded.status.success(),
        "an ignored-only worktree must still clear the abandon refusal; stdout:\n{stdout}\n\
         stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("milestone.dirty-worktree"),
        "the ignored axis must not reach the refusal probe; stderr:\n{stderr}",
    );

    // (2) …and the loss is NAMED before it happens — visible, not prevented.
    let warning = warning_block(&stderr, "area-low");
    assert!(
        warning.contains("secrets.env (ignored by git)"),
        "the teardown must name the ignored bytes it destroys; got:\n{warning}",
    );
    assert!(
        !worktree_dir(repo.path(), "area-low").exists(),
        "the abandon still removes the worktree (visible, not prevented)",
    );
}

/// **The abandon door's second subject: the authored prose in the sub-task areas.**
///
/// `discard`'s teardown does not only remove worktrees — it `remove_dir_all`s each
/// `.jigc/tasks/<sub-id>/`, which is where a sub-agent's authored `docs/*.md` live, in no
/// object DB at all. Until this cell that removal was **silent at exit 0**, while
/// `jigc uninstall` refused over byte-identical state and named the very same docs
/// (`uninstall.staged-prose`) — the law-1 half-truth M46 Inc 2 removes at the one door and
/// left standing at its sibling, against the increment's own claim (*no milestone door
/// silently drops sub-agent work or destroys bytes it does not name*).
///
/// **The axis is the task set the door reaches**, not the reported repro: the narration must
/// name the milestone's sub-tasks that hold staged docs, must **not** name a sub-task holding
/// none (the over-report that makes a loss warning untrustworthy), and must not name — or
/// touch — an unrelated open task, whose area this door never removes.
///
/// **Since the M50 completion audit (finding 4) this arm drives the FORCED abandon**, because
/// the state it plants *is* the loss cell the door now refuses over (`milestone.staged-prose`)
/// — the refusal is asserted first, and every narration assertion below then runs against
/// `--force`, unchanged. That split is the point: M46 pinned that consent buys the teardown
/// and never silence, so a `--force` that suppressed the warning would redden here.
#[test]
fn the_abandon_names_the_authored_task_prose_it_destroys() {
    let repo = TempDir::new("abandon-prose");
    init_repo(repo.path());
    let home = TempDir::new("home");

    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low"] {
        assert!(
            run_milestone(
                repo.path(),
                home.path(),
                &["add-task", "cache-rework", intent]
            )
            .status
            .success(),
            "add-task `{intent}` must exit 0",
        );
    }
    // The cell that must be named: a milestone sub-task whose authored ADR exists only here.
    stage_doc(
        repo.path(),
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
    );
    // `area-zed` stages nothing — the cell that must NOT be named.
    // The unrelated open task: outside the milestone, so this door neither names nor takes it.
    stage_doc(
        repo.path(),
        "solo-task",
        "adr:solo-policy",
        &adr_plain("Solo policy"),
    );

    let provisioned = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );

    // Since the M50 completion audit (finding 4) the staged prose this arm is *about* is
    // exactly what the door now refuses over: `area-low` stages `adr:low-policy`, the
    // worktrees are clean, and that is cell D of the `{staged prose} × {dirty worktree}`
    // 2×2 — the loss cell the worktree probe structurally cannot see. So the un-forced run
    // no longer reaches the teardown, and this arm carries the consent.
    let refused = run_milestone(repo.path(), home.path(), &["discard", "cache-rework"]);
    let refusal = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        !refused.status.success() && refusal.contains("milestone.staged-prose"),
        "the un-forced abandon must now REFUSE over the very prose this arm is about — the \
         narration is what the door says once consent is given, never instead of it; \
         got:\n{refusal}",
    );

    let discarded = run_milestone(
        repo.path(),
        home.path(),
        &["discard", "cache-rework", "--force"],
    );
    let stdout = String::from_utf8(discarded.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(discarded.stderr).expect("utf-8 stderr");

    // (1) The consent runs, and it buys the teardown — never silence: every narration
    // assertion below is asserted against the FORCED run, unchanged from before the guard.
    assert!(
        discarded.status.success(),
        "the consented abandon must reach its teardown; stdout:\n{stdout}\nstderr:\n{stderr}",
    );

    // (2) The door names the authored prose it is about to destroy.
    assert!(
        stderr.contains("area-low: adr:low-policy"),
        "the abandon must name the staged doc it destroys, as `uninstall` does over the same \
         bytes; stderr:\n{stderr}",
    );

    // (3) …and names only that: no sub-task holding nothing, no task outside the milestone.
    assert!(
        !stderr.contains("area-zed:"),
        "a sub-task holding no staged doc must not be reported; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("solo-task"),
        "the abandon removes only its own sub-tasks' areas — naming another task's prose \
         would claim a destruction it never performs; stderr:\n{stderr}",
    );

    // (4) Non-vacuity, both ways: the named bytes really are gone (visible, not prevented),
    // and the unnamed task's prose really did survive.
    let tasks = repo.path().join(".jigc").join("tasks");
    assert!(
        !tasks.join("area-low").exists(),
        "the abandon really removes the sub-task area, or the narration was about nothing",
    );
    assert!(
        tasks.join("solo-task").join("docs").exists(),
        "an unrelated open task's staged prose must survive the abandon",
    );
}
