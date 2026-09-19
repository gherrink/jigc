//! Flow 33 acceptance (M31 Increment 4, T3) — **the combine keystone: a `squash: true`
//! fan-out finalize folds the N worktree-staged code-sets into ONE commit, drops none,
//! blocks a same-file collision, and never mutates the live checkout until a clean commit
//! lands** (`design/worked-examples.md` → flow 33; `design/finalize.md` → `fan-out`
//! finalize / Dirty-tree policy; `DECISIONS.md` → 2026-06-20 M31 planning — the combine +
//! off-substrate gate-review S1/S2 + the determinism bar A1 = tree+message).
//!
//! This is the marquee for the data-loss fix the increment exists to retire: before the
//! combine channel, the `squash: true` boundary `git add --all`ed the **main checkout**,
//! where the worktree-isolated sub-agent code never lives — so it committed **zero**
//! sub-agent code. The whole flow drives the cargo-built `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) against throwaway git repos with the **embedded** dev pack (no
//! `JIGC_PACK_DIR`), the flow9/flow10 real-binary idiom, and proves end-to-end:
//!
//!   - **(a) disjoint code + docs, one commit (drops none).** A `squash: true` fan-out
//!     with two sub-agents on DISJOINT files commits **both** sub-agents' staged code +
//!     the merged docs in **one** commit.
//!   - **(c) the main checkout is clean post-commit** — no ` D` drift after the
//!     combine + worktree teardown.
//!   - **(d) the commit is order-invariant.** The combined commit's **tree hash** and
//!     **message** are byte-identical across divergent sub-agent feed/completion orders
//!     (tree+message, not the timestamped commit SHA — the `milestone.rs:982` bar,
//!     Validation hardening #7 extended to the code substrate).
//!   - **(b) a same-file collision blocks** naming the colliding path — **including** a
//!     sub-agent that *renames* it — and nothing is applied or promoted.
//!   - **(f) unrelated main-checkout WIP survives a blocked combine** — the off-line
//!     temp-index build never touches the live checkout (review S2).
//!   - **(e) an advanced main blocks finalize** — the `base == main-HEAD` preflight
//!     (review S1) holds; HEAD is unchanged and nothing is applied.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow33-{tag}-{}-{:?}",
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

/// Run `git <args>` in `dir`, asserting success and returning trimmed stdout. Used for
/// both the main checkout and the sub-agent worktrees (a worktree `git mv` / `git add` is
/// just `git` in that dir).
fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {dir:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit (the milestone mint pins its shared base to
/// HEAD via `git rev-parse`). `extra` files are committed into the base too (so a sub-agent
/// can rename a base-tracked file).
fn init_repo(root: &Path, extra: &[(&str, &str)]) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write README.md");
    for (rel, body) in extra {
        let p = root.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir extra base file parent");
        }
        fs::write(p, body).expect("write extra base file");
    }
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
}

/// Run `jigc milestone <args>` with `cwd = repo`, `$HOME = home`, and the embedded pack.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("milestone").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation succeeded, surfacing both streams on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A plain ADR body with no `supersedes` ref — a clean, disjoint persisted doc each
/// sub-area edits (distinct slug per area, so no same-doc clash).
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a doc `body` into a milestone sub-task's `docs/` area (`.jigc/tasks/<sub>/docs/`)
/// with `provenance`, exactly as the staging primitives would — the two inputs the
/// by-task-id join reads (mirrors `milestone.rs`'s `stage_doc`).
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

/// Write + `git add` a code file IN a provisioned fan-out worktree
/// (`.jigc/worktrees/<sub>/<rel>`) — the staged code a fanned-out sub-agent produces in
/// its isolated worktree (mirrors `milestone.rs`'s `stage_worktree_code`). Stages in the
/// worktree's OWN index, never the main checkout.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    git(&wt, &["add", rel]);
}

/// Overwrite the milestone's recorded `tasks.json` with `order` (the audit-trail feed
/// order) — the deliberate divergence the determinism headline drives. The combine folds
/// id-sorted regardless, so re-recording the same id SET in divergent orders must produce
/// a byte-identical committed tree + message (hardening #7, the code substrate).
fn rewrite_task_order(repo: &Path, milestone_id: &str, order: &[&str]) {
    let path = repo
        .join(".jigc")
        .join("milestones")
        .join(milestone_id)
        .join("tasks.json");
    let value = serde_json::json!({ "tasks": order });
    let mut bytes = serde_json::to_string_pretty(&value).expect("serialize task list");
    bytes.push('\n');
    fs::write(&path, bytes).expect("rewrite tasks.json");
}

/// HEAD's committed tree hash (`git rev-parse HEAD^{tree}`) — two commits with the same
/// tree hash carry byte-identical committed content, independent of author/date/SHA.
fn head_tree(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD^{tree}"])
}

/// HEAD's full commit message (`git log -1 --format=%B`).
fn head_message(repo: &Path) -> String {
    git(repo, &["log", "-1", "--format=%B"])
}

/// The committed paths of HEAD (`git show --name-only --format= HEAD`).
fn head_paths(repo: &Path) -> Vec<String> {
    git(repo, &["show", "--name-only", "--format=", "HEAD"])
        .lines()
        .map(str::to_string)
        .collect()
}

/// Build the **identical** disjoint-combine fixture in `repo` and finalize it under the
/// recorded `feed_order`, staging the two sub-agents' worktree code in `stage_order` (the
/// completion-order divergence). Returns the landed commit's `(message, tree-hash)`.
///
/// Two sub-tasks (`area-low`, `area-zed`) each stage a DISJOINT persisted ADR + a DISJOINT
/// code file in their isolated worktrees. The recorded `tasks.json` feed order and the
/// code-staging order are the divergences the determinism headline varies; the combined
/// commit must be a pure function of the id SET, never either order.
fn build_and_finalize_disjoint(
    repo: &Path,
    home: &Path,
    feed_order: &[&str],
    stage_order: &[(&str, &str, &str)],
) -> (String, String) {
    init_repo(repo, &[]);
    expect_ok(
        &run_milestone(repo, home, &["create", "Cache rework"]),
        "milestone create",
    );
    // Added in NON-id order (zed before low) so id order is not an accident of insertion.
    for intent in ["Area zed", "Area low"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", "cache-rework", intent]),
            "milestone add-task",
        );
    }
    stage_doc(
        repo,
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
        "edited-from-base",
    );
    stage_doc(
        repo,
        "area-zed",
        "adr:zed-policy",
        &adr_plain("Zed policy"),
        "edited-from-base",
    );

    expect_ok(
        &run_milestone(repo, home, &["provision", "cache-rework"]),
        "milestone provision",
    );

    // Each sub-agent stages DISJOINT code in its own isolated worktree, in the divergent
    // completion order the headline varies.
    for (sub, rel, body) in stage_order {
        stage_worktree_code(repo, sub, rel, body);
    }

    // The deliberate feed-order divergence — the id set is fixed, only the recorded order
    // differs. Rewritten right before the finalize (the flow 9 idiom).
    rewrite_task_order(repo, "cache-rework", feed_order);

    expect_ok(
        &run_milestone(repo, home, &["finalize", "cache-rework"]),
        "milestone finalize (squash:true combine)",
    );

    (head_message(repo), head_tree(repo))
}

/// **(a)+(c)+(d) — disjoint combine commits both sub-agents' code + the merged docs in one
/// commit, the main checkout is clean afterward, and the commit's tree + message are
/// byte-identical across divergent feed/completion orders.**
#[test]
fn flow33_disjoint_combine_commits_both_and_is_order_invariant() {
    let low = ("area-low", "src/low.rs", "pub fn low() {}\n");
    let zed = ("area-zed", "src/zed.rs", "pub fn zed() {}\n");

    let home = TempDir::new("home");

    // Repo A — canonical id-order feed, code staged low→zed.
    let repo_id = TempDir::new("order-id");
    let (msg_id, tree_id) = build_and_finalize_disjoint(
        repo_id.path(),
        home.path(),
        &["area-low", "area-zed"],
        &[low, zed],
    );

    // Repo B — REVERSE feed order AND reverse code-staging order (the divergence a
    // completion-ordered combine would let leak into the committed bytes).
    let repo_rev = TempDir::new("order-rev");
    let (msg_rev, tree_rev) = build_and_finalize_disjoint(
        repo_rev.path(),
        home.path(),
        &["area-zed", "area-low"],
        &[zed, low],
    );

    // ── (d) the determinism bar: tree + message byte-identical across the two orders. ──
    assert_eq!(
        tree_id, tree_rev,
        "the combined commit's TREE must be byte-identical across feed/completion orders \
         ({tree_id} vs {tree_rev})",
    );
    assert_eq!(
        msg_id, msg_rev,
        "the combined commit's MESSAGE must be byte-identical across feed/completion \
         orders;\nid:\n{msg_id}\nreverse:\n{msg_rev}",
    );
    assert!(
        msg_id.contains("Finalize milestone cache-rework (2 sub-tasks)"),
        "the committed message must be the CLI-synthesized aggregate projection; got:\n{msg_id}",
    );

    // ── (a) one commit carries BOTH sub-agents' code AND the merged docs (drops none). ──
    let landed = head_paths(repo_id.path());
    for path in [
        "src/low.rs",
        "src/zed.rs",
        "docs/decisions/low-policy.md",
        "docs/decisions/zed-policy.md",
    ] {
        assert!(
            landed.iter().any(|p| p == path),
            "the single combine commit must carry `{path}` (dropped none); got:\n{landed:?}",
        );
    }
    // The committed code bytes are the sub-agents' staged bytes, read off the landed tree.
    assert_eq!(
        git(repo_id.path(), &["show", "HEAD:src/low.rs"]),
        "pub fn low() {}",
        "area-low's staged code lands verbatim",
    );
    assert_eq!(
        git(repo_id.path(), &["show", "HEAD:src/zed.rs"]),
        "pub fn zed() {}",
        "area-zed's staged code lands verbatim",
    );

    // ── (c) the main checkout is clean after the combine + worktree teardown. ──
    for repo in [repo_id.path(), repo_rev.path()] {
        let status = git(repo, &["status", "--porcelain"]);
        assert!(
            status.trim().is_empty(),
            "the main checkout must be clean after the combine (no ` D` drift); got:\n{status}",
        );
    }
}

/// **(b)+(f) — two sub-agents touching the SAME file block (incl. one that RENAMES it),
/// nothing is applied or promoted, and unrelated main-checkout WIP survives the blocked
/// combine untouched.**
#[test]
fn flow33_same_file_collision_blocks_and_wip_survives() {
    let repo = TempDir::new("collision");
    let home = TempDir::new("home");
    // `shared.txt` lives in the base tree so a sub-agent can rename it.
    init_repo(repo.path(), &[("shared.txt", "shared\n")]);

    expect_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Area zed", "Area low"] {
        expect_ok(
            &run_milestone(
                repo.path(),
                home.path(),
                &["add-task", "cache-rework", intent],
            ),
            "milestone add-task",
        );
    }
    // Disjoint persisted docs (so the block is the code collision, not the empty guard).
    stage_doc(
        repo.path(),
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "area-zed",
        "adr:zed-policy",
        &adr_plain("Zed policy"),
        "edited-from-base",
    );
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]),
        "milestone provision",
    );

    // area-low EDITS shared.txt; area-zed RENAMES it (its rename old-path collides) — the
    // rename-aware block-set must catch the old path.
    let wt_low = repo.path().join(".jigc").join("worktrees").join("area-low");
    fs::write(wt_low.join("shared.txt"), "shared, edited by low\n").expect("edit shared in low");
    git(&wt_low, &["add", "shared.txt"]);
    let wt_zed = repo.path().join(".jigc").join("worktrees").join("area-zed");
    git(&wt_zed, &["mv", "shared.txt", "moved.txt"]);

    // Seed UNRELATED WIP in the MAIN checkout — an untracked file + an unstaged edit to a
    // tracked file. The off-line build must never touch either (review S2).
    fs::write(repo.path().join("wip-untracked.txt"), "scratch\n").expect("write untracked WIP");
    fs::write(repo.path().join("README.md"), "hello\nlocal WIP\n").expect("modify tracked WIP");
    let status_before = git(repo.path(), &["status", "--porcelain"]);
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&finalized.stdout),
        String::from_utf8_lossy(&finalized.stderr),
    );
    assert!(
        !finalized.status.success(),
        "a same-file cross-worktree collision must block finalize; got success\n{rendered}",
    );
    assert!(
        rendered.contains("code collision")
            && rendered.contains("shared.txt")
            && rendered.contains("route:"),
        "the block must name the colliding path (incl. the rename old-path) + a route; \
         got:\n{rendered}",
    );

    // Nothing applied, nothing promoted, HEAD unchanged.
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "a blocked combine must commit nothing",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("decisions")
            .join("low-policy.md")
            .exists()
            && !repo
                .path()
                .join("docs")
                .join("decisions")
                .join("zed-policy.md")
                .exists(),
        "a blocked combine must promote nothing (the rolled-back promotions leave no doc)",
    );

    // ── (f) unrelated main-checkout WIP survives the blocked combine byte-identically. ──
    assert_eq!(
        fs::read_to_string(repo.path().join("wip-untracked.txt")).unwrap(),
        "scratch\n",
        "the untracked WIP file survives a blocked combine untouched",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("README.md")).unwrap(),
        "hello\nlocal WIP\n",
        "the unstaged tracked WIP edit survives a blocked combine untouched",
    );
    assert_eq!(
        git(repo.path(), &["status", "--porcelain"]),
        status_before,
        "a blocked combine leaves the live index/worktree byte-identical (review S2)",
    );
}

/// **(e) — an advanced main blocks finalize (the `base == main-HEAD` preflight, review
/// S1): HEAD is unchanged and nothing is applied.** The sub-agent base-pin guard reads the
/// worktree HEAD and cannot enforce "main does not advance"; the milestone preflight does.
#[test]
fn flow33_advanced_main_blocks_finalize() {
    let repo = TempDir::new("advanced-main");
    let home = TempDir::new("home");
    init_repo(repo.path(), &[]);

    expect_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Area zed", "Area low"] {
        expect_ok(
            &run_milestone(
                repo.path(),
                home.path(),
                &["add-task", "cache-rework", intent],
            ),
            "milestone add-task",
        );
    }
    stage_doc(
        repo.path(),
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
        "edited-from-base",
    );
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]),
        "milestone provision",
    );
    stage_worktree_code(repo.path(), "area-low", "src/low.rs", "pub fn low() {}\n");

    // Main advances past the milestone's pinned base with an unrelated commit.
    fs::write(repo.path().join("unrelated.txt"), "advance main\n").expect("write unrelated");
    git(repo.path(), &["add", "unrelated.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "advance main"]);
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&finalized.stdout),
        String::from_utf8_lossy(&finalized.stderr),
    );
    assert!(
        !finalized.status.success(),
        "an advanced main must block the milestone finalize; got success\n{rendered}",
    );
    assert!(
        rendered.contains("HEAD is now"),
        "the block must be the base-mismatch preflight naming the advanced HEAD; got:\n{rendered}",
    );

    // HEAD unchanged (the finalize laid down no commit) and nothing was applied/promoted.
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        head_before,
        "a base-mismatch block must leave HEAD unchanged (nothing applied)",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("decisions")
            .join("low-policy.md")
            .exists(),
        "a base-mismatch block must promote nothing",
    );
    assert!(
        !repo.path().join("src").join("low.rs").exists(),
        "a base-mismatch block must apply no worktree code to the main checkout",
    );
}

/// **(f-twin) — unrelated main-checkout WIP survives a SUCCESSFUL `squash: true` combine.**
/// The blocked-combine WIP guarantee (test above) only covers the path that returns before
/// the ref advance; this is its success-path twin (Validation hardening #5): the landing must
/// fast-forward main onto the hook-running commit **non-destructively** — carrying unrelated
/// unstaged WIP, never `git reset --hard`ing it away (`design/finalize.md` → `fan-out`
/// finalize: "The main checkout is never mutated until that clean fast-forward";
/// `design/worked-examples.md` flow 33 — "needs no destructive reset … the M30 `git reset
/// --hard` rollback hazard … is rejected").
#[test]
fn flow33_unrelated_wip_survives_successful_combine() {
    let repo = TempDir::new("success-wip");
    let home = TempDir::new("home");
    init_repo(repo.path(), &[]);

    expect_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache rework"]),
        "milestone create",
    );
    for intent in ["Area zed", "Area low"] {
        expect_ok(
            &run_milestone(
                repo.path(),
                home.path(),
                &["add-task", "cache-rework", intent],
            ),
            "milestone add-task",
        );
    }
    stage_doc(
        repo.path(),
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "area-zed",
        "adr:zed-policy",
        &adr_plain("Zed policy"),
        "edited-from-base",
    );
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]),
        "milestone provision",
    );
    // DISJOINT sub-agent code, isolated in each worktree — none touches README.md.
    stage_worktree_code(repo.path(), "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo.path(), "area-zed", "src/zed.rs", "pub fn zed() {}\n");

    // Seed unrelated WIP in the MAIN checkout right before the finalize: an unstaged edit to a
    // tracked file the combine never touches, plus an untracked scratch file. Neither is the
    // milestone's work; both must survive the landing byte-identically.
    fs::write(
        repo.path().join("README.md"),
        "hello\nUNRELATED WIP THE USER IS EDITING\n",
    )
    .expect("seed unstaged tracked WIP");
    fs::write(repo.path().join("wip-untracked.txt"), "scratch\n").expect("seed untracked WIP");

    expect_ok(
        &run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]),
        "milestone finalize (squash:true combine) must succeed",
    );

    // The combine landed: both sub-agents' code rode the one commit.
    assert!(
        repo.path().join("src").join("low.rs").exists()
            && repo.path().join("src").join("zed.rs").exists(),
        "a successful combine brings both worktrees' code into the main checkout",
    );

    // The unrelated WIP survives byte-identically (the success-path twin of the blocked-path
    // assertion above) — the landing must NOT `git reset --hard` it away.
    assert_eq!(
        fs::read_to_string(repo.path().join("README.md")).unwrap(),
        "hello\nUNRELATED WIP THE USER IS EDITING\n",
        "the unstaged tracked WIP edit survives a SUCCESSFUL combine untouched",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("wip-untracked.txt")).unwrap(),
        "scratch\n",
        "the untracked WIP file survives a SUCCESSFUL combine untouched",
    );
    // The WIP stayed OUT of the commit — left behind, never swept in.
    let committed = head_paths(repo.path());
    assert!(
        !committed.contains(&"README.md".to_string())
            && !committed.contains(&"wip-untracked.txt".to_string()),
        "unrelated WIP is left behind, never committed; committed paths: {committed:?}",
    );
}
