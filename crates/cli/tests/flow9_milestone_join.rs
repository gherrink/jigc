//! Flow 9 acceptance — **the deterministic by-task-id join commits reproducibly,
//! end-to-end, through the real `jigc` binary** (`design/worked-examples.md` → flow 9;
//! `implementation/increment-workflow.md` → A second principle (M7): determinism by
//! re-execution, Validation hardening #7; `CLAUDE.md` → "merges at a join ordered by
//! task ID, not completion order"). This is M7's headline `Proves`: the join is a pure
//! function of the *set* of sub-task working areas — same set in, byte-identical
//! committed state out, regardless of the order the sub-tasks were recorded / read.
//!
//! The whole flow drives the emitted binary against throwaway git repos, exercising
//! every grouped-scope contention path (increment-workflow.md hardening #1 — each
//! exercised through the binary) plus the re-execution headline (hardening #7 — ≥2
//! divergent feed orders, byte-identical output):
//!
//!   - **Permutation determinism (the headline).** The *identical* populated fixture
//!     is built in **three independent repos** whose recorded `tasks.json` feed order
//!     is deliberately divergent — canonical **id order**, **reverse** id order, and a
//!     **seed-shuffled** order (the audit-trail order a completion-ordered / broken
//!     merge would key on). Each repo is finalized through the binary and the
//!     **committed bytes — the commit MESSAGE *and* the committed TREE hash — are
//!     asserted byte-identical across all three.** Reverse order is mandatory: an
//!     id-ordered fixture where completion-order trivially equals id-order would pass
//!     even a completion-ordered (broken) merge. The fixture forces genuine overlap
//!     (two same-slug `created` docs, one self-referential; an `add-from-spec` seeded
//!     sub-task) so the proof is not over a trivially-disjoint set.
//!   - **Suffix-by-task-id + self-ref rewrite (through the committed tree).** The lower
//!     task id keeps the bare slug; the higher takes the `-2` suffix, its **own**
//!     self-reference rewritten in lockstep — read straight off the promoted, committed
//!     `decisions/*.md` files, not a reconstruction.
//!   - **Cross-area ref blocked + the naive-union control.** A sub-task references a
//!     *sibling* sub-area's slug; the join blocks with the intrinsic, already-floored
//!     `schema-conformance.ref-resolves` (no commit). The **control** — the same ref
//!     staged so its target lives in the authoring sub-area's *own* `docs/` (a naive
//!     all-areas union resolves it clean) — joins clean and commits, proving the block
//!     is the per-`from` narrowing, never a genuinely-absent target.
//!   - **Same-doc clash blocked.** Two sub-tasks each `edited-from-base` the same
//!     committed-at-base slug → a partition violation that blocks with
//!     `join.same-doc-clash`, committing nothing.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, each repo
//! is a real `git init`, and self-cleaning `TempDir`s keep the test off the dev's repo
//! (mirroring `milestone.rs` / `pack_source_determinism.rs` / `flow8_*`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow9-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// Run `git <args>` in `repo`, asserting success and returning stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
}

/// Run `jigc milestone <args>` with `cwd = repo` and `$HOME = home`.
fn run_milestone(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("milestone");
    command.args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc milestone` invocation succeeded, surfacing stderr on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A committed `spec` with one repeatable `criterion` — the `add-from-spec` seed
/// substrate so flow 9 exercises BOTH origination paths (`add-task` AND
/// `add-from-spec`) feeding the same join. Its single criterion slugs to the sub-task
/// id `document-the-cache-strategy`.
const SEED_SPEC: &str = "\
# Cache hardening plan

## Goal

Harden the cache layer.

## Context

The cache strategy needs documenting alongside the eviction work.

## Criteria

### Document the cache strategy  {#document-strategy}

The cache strategy is recorded as an ADR.
";

/// Commit `SEED_SPEC` at `specs/<slug>.md` so `add-from-spec` reads genuinely
/// committed state.
fn commit_spec(repo: &Path, slug: &str) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join(format!("{slug}.md")), SEED_SPEC).expect("write spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "add spec"]);
}

/// An ADR body whose `supersedes` ref points at `to` (its OWN slug when `to` is its own
/// address) — the self-reference the collision-suffix rule must rewrite in lockstep
/// with the `-2` slug suffix.
fn adr_superseding(title: &str, to: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\nsupersedes: {to}\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A plain ADR body with no `supersedes` ref — a clean, disjoint doc.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Stage a doc `body` into a milestone sub-task's `docs/` area with a chosen
/// `provenance`, exactly as the staging primitives would: write
/// `tasks/<sub>/docs/<address>.md` verbatim and merge `(address → provenance)` into the
/// area's `docs/provenance.json`. These are the two inputs the by-task-id join reads;
/// the test writes them directly because no front-door verb yet stages into a milestone
/// sub-area (mirroring `milestone.rs`'s `stage_doc`).
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

/// Overwrite the milestone's recorded `tasks.json` with `order` (the audit-trail feed
/// order) — the deliberate divergence the permutation headline drives. The join must
/// enumerate id-sorted regardless of this recorded order, so re-recording it in id /
/// reverse / shuffled orders must produce byte-identical committed state. The set of
/// ids is unchanged; only their recorded order differs.
fn rewrite_task_order(repo: &Path, milestone_id: &str, order: &[&str]) {
    let path = repo
        .path_tasks_json(milestone_id)
        .expect("milestone tasks.json path");
    let value = serde_json::json!({ "tasks": order });
    let mut bytes = serde_json::to_string_pretty(&value).expect("serialize task list");
    bytes.push('\n');
    fs::write(&path, bytes).expect("rewrite tasks.json");
}

/// Path helper trait so `rewrite_task_order` reads naturally.
trait MilestonePaths {
    fn path_tasks_json(&self, milestone_id: &str) -> Option<PathBuf>;
}
impl MilestonePaths for Path {
    fn path_tasks_json(&self, milestone_id: &str) -> Option<PathBuf> {
        let p = self
            .join(".jigc")
            .join("milestones")
            .join(milestone_id)
            .join("tasks.json");
        p.is_file().then_some(p)
    }
}

/// HEAD's full commit message (`git log -1 --format=%B`) — read verbatim off the landed
/// commit (the committed MESSAGE half of the byte-identity assertion).
fn head_message(repo: &Path) -> String {
    git(repo, &["log", "-1", "--format=%B"])
}

/// HEAD's committed tree hash (`git rev-parse HEAD^{tree}`) — the committed TREE half of
/// the byte-identity assertion. Two commits with the same tree hash have byte-identical
/// committed content (every promoted doc body + path), independent of author/date.
fn head_tree(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD^{tree}"]).trim().to_string()
}

/// Build the **identical** populated flow-9 fixture in `repo` and finalize it under the
/// recorded `feed_order`, returning the landed commit's `(message, tree-hash)`.
///
/// The fixture, per flow 9's "overlapping-by-design sub-tasks", seeds via BOTH
/// origination paths: an `add-from-spec` seed whose single criterion mints the sub-task
/// id `document-the-cache-strategy`, and an `add-task "Add a cache-strategy ADR"` →
/// `add-a-cache-strategy-adr`. Both sub-tasks *create* the same slug `adr:cache-strategy`,
/// each superseding its OWN slug — a collision of distinct `created` instances. id-sorted,
/// `add-a-cache-strategy-adr` < `document-the-cache-strategy`, so the former keeps the
/// bare slug and the latter takes the `-2` suffix. A third sub-task stages a disjoint
/// `edited-from-base` doc.
///
/// After population the recorded `tasks.json` is rewritten to `feed_order` — the
/// divergence the headline varies. The same area bytes are on disk in every repo; only
/// the recorded order differs.
fn build_and_finalize(repo: &Path, home: &Path, feed_order: &[&str]) -> (String, String) {
    init_repo(repo);
    commit_spec(repo, "cache-hardening-plan");

    expect_ok(
        &run_milestone(repo, home, &["create", "Cache hardening"]),
        "milestone create",
    );

    // Origination path 1 — `add-from-spec` seeds `document-the-cache-strategy`.
    expect_ok(
        &run_milestone(
            repo,
            home,
            &[
                "add-from-spec",
                "cache-hardening",
                "spec:cache-hardening-plan",
            ],
        ),
        "milestone add-from-spec",
    );
    // Origination path 2 — `add-task` for the colliding ADR + a disjoint sub-task.
    // Added in NON-id order so the recorded insertion order already diverges from id
    // order before the explicit `tasks.json` rewrite below.
    for intent in ["Add a cache-strategy ADR", "Add an eviction ADR"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", "cache-hardening", intent]),
            "milestone add-task",
        );
    }

    // Populate the sub-areas, forcing genuine overlap: two `created` `adr:cache-strategy`
    // (each self-referential) + one disjoint `edited-from-base` doc. The two colliding
    // bodies carry **distinguishable titles** ("eager" vs "lazy"), so which one lands at
    // the bare slug vs the `-2` suffix is *observable in the committed tree* — a
    // completion-ordered suffix bug (one keyed on feed order instead of task id) would
    // swap which title lands where and diverge the tree hash. The lower task id
    // (`add-a-cache-strategy-adr`) keeps the bare slug; the higher
    // (`document-the-cache-strategy`) takes `-2`.
    stage_doc(
        repo,
        "add-a-cache-strategy-adr",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy (eager)", "adr:cache-strategy"),
        "created",
    );
    stage_doc(
        repo,
        "document-the-cache-strategy",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy (lazy)", "adr:cache-strategy"),
        "created",
    );
    stage_doc(
        repo,
        "add-an-eviction-adr",
        "adr:eviction-policy",
        &adr_plain("Eviction policy"),
        "edited-from-base",
    );

    // The deliberate feed-order divergence (the audit-trail order). The id set is fixed;
    // only the recorded order differs across repos.
    rewrite_task_order(repo, "cache-hardening", feed_order);

    expect_ok(
        &run_milestone(repo, home, &["finalize", "cache-hardening"]),
        "milestone finalize",
    );

    (head_message(repo), head_tree(repo))
}

/// **Flow 9 headline — permutation determinism through the binary.** The identical
/// populated fixture is finalized in three independent repos under three divergent
/// recorded feed orders (id, reverse, seed-shuffled); the committed MESSAGE and TREE
/// hash are asserted byte-identical across all three. A completion-ordered / broken
/// merge — one that let the recorded order reach the suffix assignment or the body bytes
/// — would diverge under the reverse / shuffled orders and fail this assertion. Plus the
/// supporting contention assertions read off the committed tree (hardening #1 + #7).
#[test]
fn flow9_join_commits_byte_identically_across_divergent_feed_orders() {
    // The fixed id set, in canonical id order. The three feed orders below are all
    // permutations of exactly this set.
    let id_order = [
        "add-a-cache-strategy-adr",
        "add-an-eviction-adr",
        "document-the-cache-strategy",
    ];
    let reverse_order = [
        "document-the-cache-strategy",
        "add-an-eviction-adr",
        "add-a-cache-strategy-adr",
    ];
    // A seed-shuffled order distinct from both id and reverse (a third permutation).
    let shuffled_order = [
        "add-an-eviction-adr",
        "document-the-cache-strategy",
        "add-a-cache-strategy-adr",
    ];

    // Sanity: the three feed orders are genuinely divergent (a re-execution under a
    // single order would not exercise hardening #7).
    assert_ne!(id_order, reverse_order, "id vs reverse must diverge");
    assert_ne!(id_order, shuffled_order, "id vs shuffled must diverge");
    assert_ne!(
        reverse_order, shuffled_order,
        "reverse vs shuffled must diverge"
    );

    let home = TempDir::new("home");

    let repo_id = TempDir::new("order-id");
    let (msg_id, tree_id) = build_and_finalize(repo_id.path(), home.path(), &id_order);
    let repo_rev = TempDir::new("order-rev");
    let (msg_rev, tree_rev) = build_and_finalize(repo_rev.path(), home.path(), &reverse_order);
    let repo_shuf = TempDir::new("order-shuf");
    let (msg_shuf, tree_shuf) = build_and_finalize(repo_shuf.path(), home.path(), &shuffled_order);

    // ---- The headline: committed MESSAGE is byte-identical across feed orders. ----
    assert_eq!(
        msg_id, msg_rev,
        "the committed message must be byte-identical under id vs reverse feed order;\n\
         id:\n{msg_id}\nreverse:\n{msg_rev}",
    );
    assert_eq!(
        msg_id, msg_shuf,
        "the committed message must be byte-identical under id vs shuffled feed order;\n\
         id:\n{msg_id}\nshuffled:\n{msg_shuf}",
    );
    // The message is the CLI-synthesized structural projection — subject + id-ordered body.
    assert!(
        msg_id.contains("Finalize milestone cache-hardening (3 sub-tasks)"),
        "the committed message must be the synthesized projection; got:\n{msg_id}",
    );

    // ---- The headline: committed TREE hash is byte-identical across feed orders. ----
    // Equal tree hashes ⇒ every promoted doc body + path is byte-identical, so the
    // suffix assignment, the self-ref rewrite, and the disjoint merge are all a pure
    // function of the area SET, never the recorded feed order.
    assert_eq!(
        tree_id, tree_rev,
        "the committed tree must be byte-identical under id vs reverse feed order \
         ({tree_id} vs {tree_rev})",
    );
    assert_eq!(
        tree_id, tree_shuf,
        "the committed tree must be byte-identical under id vs shuffled feed order \
         ({tree_id} vs {tree_shuf})",
    );

    // ---- Supporting: suffix-by-task-id + self-ref rewrite, off the committed tree. ----
    // Read the promoted docs from the id-order repo (the tree is identical across all
    // three, so reading any one is reading the shared committed bytes).
    let decisions = repo_id.path().join("docs").join("decisions");
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            decisions.join(format!("{slug}.md")).is_file(),
            "the promoted `{slug}` doc must land at docs/decisions/{slug}.md",
        );
    }
    // The lower task id (`add-a-cache-strategy-adr`, the "eager" body) kept the BARE
    // slug; its self-ref is unchanged at `adr:cache-strategy`. Asserting the title proves
    // suffix-BY-TASK-ID (not merely "a suffix happened"): a completion-ordered suffix
    // would land the "lazy" body here instead.
    let bare = fs::read_to_string(decisions.join("cache-strategy.md")).expect("read bare");
    assert!(
        bare.contains("# Cache strategy (eager)"),
        "the lower task id's body must keep the bare slug; got:\n{bare}",
    );
    assert!(
        bare.contains("supersedes: adr:cache-strategy"),
        "the bare instance keeps its self-ref unchanged; got:\n{bare}",
    );
    // The higher task id (`document-the-cache-strategy`, the "lazy" body) took the `-2`
    // suffix; its OWN self-ref was rewritten in lockstep to the suffixed slug.
    let suffixed =
        fs::read_to_string(decisions.join("cache-strategy-2.md")).expect("read suffixed");
    assert!(
        suffixed.contains("# Cache strategy (lazy)"),
        "the higher task id's body must take the `-2` suffix; got:\n{suffixed}",
    );
    assert!(
        suffixed.contains("supersedes: adr:cache-strategy-2"),
        "the `-2` instance's self-ref must be rewritten to its suffixed slug; got:\n{suffixed}",
    );

    // The promoted docs are genuinely committed (tracked in HEAD's tree).
    let tracked = git(repo_id.path(), &["ls-files", "docs/decisions/"]);
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            tracked.contains(&format!("docs/decisions/{slug}.md")),
            "docs/decisions/{slug}.md must be committed (tracked); got:\n{tracked}",
        );
    }
}

/// **Supporting — a cross-area ref is blocked (not silently resolved), with the
/// naive-union control proving the per-`from` narrowing is load-bearing.** Sub-task B
/// references sibling A's slug; even though that doc byte-exists in A's area at join
/// time, the ref resolves against `committed ∪ B's own area` only, so the join blocks
/// with the intrinsic `schema-conformance.ref-resolves` and commits nothing. The
/// control finalizes the SAME ref with its target staged into B's own area (a naive
/// all-areas union resolves it clean) — that commits, so the block above can only come
/// from the per-area scoping, never a genuinely-absent target.
#[test]
fn flow9_cross_area_ref_blocks_with_naive_union_control() {
    let home = TempDir::new("home");

    // ---- The block: B references A's slug (cross-area). ----
    let repo = TempDir::new("cross-area");
    init_repo(repo.path());
    expect_ok(
        &run_milestone(repo.path(), home.path(), &["create", "Cache hardening"]),
        "create",
    );
    // Added NON-id order; id-sorted: [add-a-cache-strategy-adr, add-an-lru-eviction-adr].
    for intent in ["Add an LRU eviction ADR", "Add a cache-strategy ADR"] {
        expect_ok(
            &run_milestone(
                repo.path(),
                home.path(),
                &["add-task", "cache-hardening", intent],
            ),
            "add-task",
        );
    }
    // A creates `adr:lru-eviction`; B creates `adr:cache-strategy` whose `supersedes`
    // GUESSES A's slug — a cross-area ref resolvable only inside A's sibling area.
    stage_doc(
        repo.path(),
        "add-an-lru-eviction-adr",
        "adr:lru-eviction",
        &adr_superseding("LRU eviction", "adr:lru-eviction"),
        "created",
    );
    stage_doc(
        repo.path(),
        "add-a-cache-strategy-adr",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy", "adr:lru-eviction"),
        "created",
    );

    let before_head = git(repo.path(), &["rev-parse", "HEAD"]);
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-hardening"]);
    let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");
    assert!(
        !finalized.status.success(),
        "a cross-area ref finalize must exit non-zero; got {:?}\nstdout:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stdout),
    );
    assert!(
        stderr.contains("forward-ref integrity")
            && stderr.contains("adr:lru-eviction")
            && stderr.contains("route:"),
        "the cross-area block must be the intrinsic ref-resolves finding naming the \
         unreachable target + a route; got:\n{stderr}",
    );
    // Nothing committed, nothing promoted.
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        before_head,
        "a cross-area block must commit nothing",
    );
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "a cross-area block must promote nothing",
    );

    // ---- The control: the SAME ref resolves clean when its target is in B's OWN area. ----
    // This is the naive-union surface: stage A's `adr:lru-eviction` body INTO B's own
    // sub-area, so B's `supersedes: adr:lru-eviction` resolves against `committed ∪ B`.
    // The cross-area block above can therefore only come from per-area scoping.
    let ctrl = TempDir::new("cross-area-control");
    init_repo(ctrl.path());
    expect_ok(
        &run_milestone(ctrl.path(), home.path(), &["create", "Cache hardening"]),
        "create",
    );
    expect_ok(
        &run_milestone(
            ctrl.path(),
            home.path(),
            &["add-task", "cache-hardening", "Add a cache-strategy ADR"],
        ),
        "add-task",
    );
    // B (`add-a-cache-strategy-adr`) stages BOTH the referencing doc and its target in
    // its own area — the union view that resolves clean.
    stage_doc(
        ctrl.path(),
        "add-a-cache-strategy-adr",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy", "adr:lru-eviction"),
        "created",
    );
    stage_doc(
        ctrl.path(),
        "add-a-cache-strategy-adr",
        "adr:lru-eviction",
        &adr_plain("LRU eviction"),
        "created",
    );

    let ctrl_finalized = run_milestone(ctrl.path(), home.path(), &["finalize", "cache-hardening"]);
    expect_ok(
        &ctrl_finalized,
        "CONTROL: the same ref resolves clean when its target is in the authoring area",
    );
    // The control genuinely committed both docs — proof the ref is not intrinsically
    // dangling; only the cross-area scoping rejects it.
    let ctrl_tracked = git(ctrl.path(), &["ls-files", "docs/decisions/"]);
    assert!(
        ctrl_tracked.contains("docs/decisions/cache-strategy.md")
            && ctrl_tracked.contains("docs/decisions/lru-eviction.md"),
        "CONTROL: the naive-union fixture commits both docs clean; got:\n{ctrl_tracked}",
    );
}

/// Build the shipped `join.same-doc-clash` fixture in `repo`: milestone
/// `cache-hardening`, two sub-tasks, and BOTH areas staging the SAME
/// committed-at-base slug `edited-from-base` — the partition violation the join
/// refuses. One fixture, so the two arms that drive it (the `finalize` block below
/// and the blocked-join surface arm) cannot drift apart on what "a clash" is.
fn stage_same_doc_clash(repo: &Path, home: &Path) {
    expect_ok(
        &run_milestone(repo, home, &["create", "Cache hardening"]),
        "create",
    );
    for intent in ["Tune eviction thresholds", "Document eviction policy"] {
        expect_ok(
            &run_milestone(repo, home, &["add-task", "cache-hardening", intent]),
            "add-task",
        );
    }
    for sub in ["tune-eviction-thresholds", "document-eviction-policy"] {
        stage_doc(
            repo,
            sub,
            "adr:eviction-policy",
            &adr_plain("Eviction policy"),
            "edited-from-base",
        );
    }
}

/// **Supporting — two sub-tasks `edited-from-base` the same committed-at-base slug → a
/// blocking `join.same-doc-clash`, committing nothing.** A partition violation the join
/// routes to a human rather than blind-merging (no section-merge, no last-writer-win).
#[test]
fn flow9_same_doc_clash_blocks_and_commits_nothing() {
    let home = TempDir::new("home");
    let repo = TempDir::new("clash");
    init_repo(repo.path());
    stage_same_doc_clash(repo.path(), home.path());

    let before_head = git(repo.path(), &["rev-parse", "HEAD"]);
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-hardening"]);
    let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");
    assert!(
        !finalized.status.success(),
        "a same-doc clash finalize must exit non-zero; got {:?}",
        finalized.status,
    );
    assert!(
        stderr.contains("same-doc clash")
            && stderr.contains("tune-eviction-thresholds")
            && stderr.contains("document-eviction-policy")
            && stderr.contains("route:"),
        "the clash block must be `join.same-doc-clash` naming both contenders + a route; \
         got:\n{stderr}",
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        before_head,
        "a same-doc clash must commit nothing",
    );
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "a same-doc clash must promote nothing",
    );
}

/// **Supporting — a blocked `jigc milestone join` states the block, and its envelope
/// names its version** (M50 Increment 10 / T2; `design/surface-contract.md` → law 1;
/// `design/command-output-contract.md` → The third version integer; baseline-ledger
/// **N10**). Driven over the shipped `join.same-doc-clash` fixture, the door used to
/// open `joined milestone:<id> — 0 doc(s) merged`, close with the routing footer, and
/// only THEN print the blocking finding — so stdout narrated a join that merged
/// nothing, and the verdict arrived after the line an agent reads as the end of the
/// output. Its `--format json` arm carried `findings` with no `schema_version`, while
/// the block envelope a refused write returns carries it beside the same key — so of
/// the two envelopes that hand a driver a blocking finding as data, only the join's was
/// unversioned.
///
/// Three assertions, one per leg of the defect:
///
///   a. the clash run's **stdout** carries no completed-join claim;
///   b. the blocking finding is legible **ahead of** the routing footer in the merged
///      read (both streams into one file — the ordering an agent actually sees, which
///      two separately-captured pipes cannot prove);
///   c. `--format json` carries `schema_version` beside `findings`, read from
///      `engine::result::SCHEMA_VERSION` rather than retyped — the constant is global,
///      so a test spelling `3` would keep passing through a bump.
///
/// Stream discipline is untouched: the finding stays on stderr, the ack on stdout.
#[test]
fn flow9_blocked_join_states_the_block_and_versions_its_envelope() {
    let home = TempDir::new("home");
    let repo = TempDir::new("blocked-join");
    init_repo(repo.path());
    stage_same_doc_clash(repo.path(), home.path());

    // (a) The separated-stream run: stdout must not claim a join that did not happen.
    let joined = run_milestone(repo.path(), home.path(), &["join", "cache-hardening"]);
    assert!(
        !joined.status.success(),
        "a same-doc clash join must exit non-zero; got {:?}",
        joined.status,
    );
    let stdout = String::from_utf8(joined.stdout).expect("utf-8 stdout");
    assert!(
        !stdout.contains("joined milestone:"),
        "a blocked join must not narrate a completed join on stdout (law 1 — the join \
         merged nothing and committed nothing); got:\n{stdout}",
    );

    // (c) The JSON envelope: the version integer beside the findings. The blocking
    // code is read back here and reused by (b), so the merged-read assertion keys on
    // the identity the WIRE carries rather than one spelled in this test.
    let json = run_milestone(
        repo.path(),
        home.path(),
        &["join", "cache-hardening", "--format", "json"],
    );
    let envelope: serde_json::Value =
        serde_json::from_slice(&json.stdout).expect("the join renders a JSON outcome on stdout");
    assert_eq!(
        envelope["schema_version"],
        serde_json::json!(engine::result::SCHEMA_VERSION),
        "the join envelope must carry the result-contract version beside its findings, \
         as the block envelope a refused write returns already does; got:\n{envelope:#}",
    );
    let code = envelope["findings"]
        .as_array()
        .expect("a findings array beside `schema_version`")
        .iter()
        .find(|f| f["severity"] == "blocking")
        .and_then(|f| f["code"].as_str())
        .expect("the blocked join's envelope carries the blocking finding")
        .to_owned();

    // (b) The merged read — both streams into ONE file, so the assertion is about the
    // order an agent sees, not about two independently-buffered captures.
    let merged_path = home.path().join("merged.txt");
    let sink = fs::File::create(&merged_path).expect("create the merged sink");
    let dup = sink
        .try_clone()
        .expect("share the sink between both streams");
    let status = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["milestone", "join", "cache-hardening"])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .stdout(std::process::Stdio::from(sink))
        .stderr(std::process::Stdio::from(dup))
        .status()
        .expect("run the jigc binary");
    assert!(
        !status.success(),
        "the merged run blocks too; got {status:?}"
    );
    let merged = fs::read_to_string(&merged_path).expect("utf-8 merged output");
    let head = format!("blocking · {code} — ");
    let at_finding = merged
        .find(&head)
        .unwrap_or_else(|| panic!("the merged read must carry `{head}`; got:\n{merged}"));
    let at_footer = merged
        .find(cli::render::ROUTING_FOOTER)
        .unwrap_or_else(|| panic!("the merged read must carry the routing footer; got:\n{merged}"));
    assert!(
        at_finding < at_footer,
        "the blocking finding must be legible AHEAD of the routing footer — a reader \
         that stops at the footer stopped before the verdict; got:\n{merged}",
    );
}

/// **Supporting — a blocked join's doc-less line names only the sub-task that genuinely
/// staged nothing** (M51 Increment 9 / T6; charter Tier 2 **EC-15**;
/// `design/surface-contract.md` → law 1). `render::doc_less_sub_tasks` used to derive
/// *"who staged nothing"* from the merge outcome's `overlay`, and a `join.same-doc-clash`
/// keeps the contending address group **out** of that overlay — so both contenders, which
/// had each staged the clashing doc, were reported on the `no docs staged from:` line and
/// in the envelope's `no_docs_from` key as having staged nothing. The derivation now reads
/// each sub-area's own `provenance.json`, the join's own input, so the line states a fact
/// about the disk rather than about what survived the block.
///
/// The fixture forces the distinction: two sub-tasks contend on **one** address (the
/// shipped clash fixture), and a **third** sub-task stages nothing at all. Only the third
/// may be named — on **both** surfaces, which are one derivation, so they cannot disagree.
/// `overlay` is asserted unchanged (`{}` under the block): the fix corrects `no_docs_from`
/// without silently moving the sibling key beside it.
#[test]
fn flow9_blocked_join_names_only_the_genuinely_idle_sub_task() {
    let home = TempDir::new("home");
    let repo = TempDir::new("idle-sibling");
    init_repo(repo.path());
    stage_same_doc_clash(repo.path(), home.path());
    // The third sub-task: minted under the same milestone, staging nothing.
    expect_ok(
        &run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-hardening", "Audit telemetry"],
        ),
        "add-task",
    );

    let joined = run_milestone(repo.path(), home.path(), &["join", "cache-hardening"]);
    assert!(
        !joined.status.success(),
        "a same-doc clash join must exit non-zero; got {:?}",
        joined.status,
    );
    let stdout = String::from_utf8(joined.stdout).expect("utf-8 stdout");
    let line = stdout
        .lines()
        .find(|l| l.contains("no docs staged from:"))
        .unwrap_or_else(|| panic!("the join ack must carry the doc-less line; got:\n{stdout}"));
    assert!(
        line.contains("audit-telemetry"),
        "the genuinely idle sub-task must still be named; got:\n{line}",
    );
    assert!(
        !line.contains("tune-eviction-thresholds") && !line.contains("document-eviction-policy"),
        "a contender that staged the clashing doc must NOT be named as having staged \
         nothing; got:\n{line}",
    );

    // The same fact on the wire — one derivation, two surfaces.
    let json = run_milestone(
        repo.path(),
        home.path(),
        &["join", "cache-hardening", "--format", "json"],
    );
    let envelope: serde_json::Value =
        serde_json::from_slice(&json.stdout).expect("the join renders a JSON outcome on stdout");
    assert_eq!(
        envelope["no_docs_from"],
        serde_json::json!(["audit-telemetry"]),
        "the envelope's doc-less set must name the idle sub-task alone; got:\n{envelope:#}",
    );
    assert_eq!(
        envelope["overlay"],
        serde_json::json!({}),
        "the block keeps the contending group out of the overlay — that key does not move \
         with this fix; got:\n{envelope:#}",
    );
    assert!(
        envelope["findings"]
            .as_array()
            .expect("a findings array")
            .iter()
            .any(|f| f["code"] == "join.same-doc-clash"),
        "the arm must be driven over a real clash; got:\n{envelope:#}",
    );
}
