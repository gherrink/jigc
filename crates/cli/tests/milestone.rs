//! End-to-end integration tests for the `milestone` work-unit front door — the
//! mint/populate spine (`create` · `add-task` · `add-from-spec` · `list-tasks`,
//! Increment 1) and the **by-task-id join verb** (`jigc milestone join`, Increment
//! 3 T4): the join enumerates the sub-areas by sorted task id, merges their staged
//! docs, and reports the merged outcome — suffixing a `created` collision (with the
//! self-ref rewrite) and routing a same-doc clash without committing anything (it is
//! not yet wired to finalize; `design/storage.md` → The by-task-id join;
//! `design/worked-examples.md` → flow 9).
//!
//! Drives the built `jigc` binary against a throwaway temp git repo and asserts
//! the done-criterion: `milestone create "Cache rework"` mints
//! `milestone:cache-rework` (exit 0, named in output); two `add-task`s under it
//! (`"Zebra fix"`, then `"Alpha fix"`) mint sub-tasks pinned to the **milestone's
//! shared base** in isolated `tasks/<sub>/` areas; `tasks.json` records the
//! sub-tasks in **insertion order** `[zebra-fix, alpha-fix]` (the audit trail),
//! while the binary's `list-tasks` enumeration surfaces them **id-sorted**
//! `[alpha-fix, zebra-fix]` regardless of add order (the order the join reads); a
//! duplicate add-task intent **rejects** non-zero with a routed finding; and
//! `.jigc/milestones/` is gitignored.
//!
//! No external test crates: the binary path comes from Cargo's
//! `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init` (mint reads HEAD), and
//! a self-cleaning `TempDir` keeps the test off the developer's real repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-milestone-cli-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (mint reads HEAD via `git
/// rev-parse`). No `.jigc/config/` layer is needed — the milestone front door
/// reads HEAD and writes engine state under `.jigc/`, it does not resolve the
/// cascade.
fn init_repo(root: &Path) -> String {
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
        out
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    let head = git(&["rev-parse", "HEAD"]);
    String::from_utf8(head.stdout)
        .expect("utf-8 head")
        .trim()
        .to_string()
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

#[test]
fn milestone_create_then_add_tasks_through_the_binary() {
    let repo = TempDir::new("create-add");
    let head = init_repo(repo.path());
    let home = TempDir::new("home");

    // `milestone create "Cache rework"` mints `milestone:cache-rework`.
    let created = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    let stdout = String::from_utf8(created.stdout).expect("utf-8 stdout");
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );
    assert!(
        stdout.contains("milestone:cache-rework"),
        "the create summary must name the minted `milestone:cache-rework`; got:\n{stdout}",
    );
    let milestone_dir = repo
        .path()
        .join(".jigc")
        .join("milestones")
        .join("cache-rework");
    assert!(
        milestone_dir.join("base.json").is_file(),
        "create must open the milestone area with a shared base pin",
    );

    // Add two sub-tasks under it, in NON-id order (zebra before alpha), so the
    // id-sorted enumeration below is not an accident of insertion order.
    for intent in ["Zebra fix", "Alpha fix"] {
        let added = run_milestone(
            repo.path(),
            home.path(),
            &["add-task", "cache-rework", intent],
        );
        assert!(
            added.status.success(),
            "`jigc milestone add-task cache-rework \"{intent}\"` must exit 0; got {:?}\nstderr:\n{}",
            added.status,
            String::from_utf8_lossy(&added.stderr),
        );
    }

    // `tasks.json` is the recorded backing state: it keeps the `[zebra-fix,
    // alpha-fix]` insertion order as the audit trail. The deterministic id-sorted
    // `[alpha-fix, zebra-fix]` order the join reads is surfaced through the binary's
    // `list-tasks` (asserted below), never read off this backing file.
    let tasks_json =
        fs::read_to_string(milestone_dir.join("tasks.json")).expect("task list written");
    let list: serde_json::Value = serde_json::from_str(&tasks_json).expect("task list parses");
    let recorded: Vec<&str> = list["tasks"]
        .as_array()
        .expect("tasks array")
        .iter()
        .map(|v| v.as_str().expect("string id"))
        .collect();
    // Recorded backing order is insertion order (the audit trail) ...
    assert_eq!(
        recorded,
        vec!["zebra-fix", "alpha-fix"],
        "the recorded task list keeps insertion order",
    );

    // Each sub-task opened its own isolated area pinned to the MILESTONE'S base.
    let milestone_base: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(milestone_dir.join("base.json")).unwrap())
            .expect("milestone base parses");
    assert_eq!(
        milestone_base["sha"].as_str().unwrap(),
        head,
        "the milestone base pins the repo HEAD",
    );
    for sub in ["alpha-fix", "zebra-fix"] {
        let sub_dir = repo.path().join(".jigc").join("tasks").join(sub);
        assert!(
            sub_dir.is_dir(),
            "sub-task `{sub}` must open its own isolated `tasks/{sub}/` area",
        );
        let sub_base: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(sub_dir.join("base.json")).unwrap())
                .expect("sub base parses");
        assert_eq!(
            sub_base, milestone_base,
            "sub-task `{sub}` must be pinned to the milestone's shared base, not a fresh HEAD",
        );
    }

    // The binary surfaces the id-sorted enumeration the by-task-id join reads —
    // NOT the recorded `[zebra-fix, alpha-fix]` add order. This is the headline
    // deliverable: the deterministic sorted-id order must be reachable through the
    // binary, not just an internal test-only function.
    let listed = run_milestone(repo.path(), home.path(), &["list-tasks", "cache-rework"]);
    let list_stdout = String::from_utf8(listed.stdout).expect("utf-8 stdout");
    assert!(
        listed.status.success(),
        "`jigc milestone list-tasks cache-rework` must exit 0; got {:?}\nstderr:\n{}",
        listed.status,
        String::from_utf8_lossy(&listed.stderr),
    );
    // alpha-fix must precede zebra-fix in the emitted bytes (id-sorted), even
    // though they were added zebra-then-alpha.
    let alpha_at = list_stdout
        .find("alpha-fix")
        .expect("emitted list names alpha-fix");
    let zebra_at = list_stdout
        .find("zebra-fix")
        .expect("emitted list names zebra-fix");
    assert!(
        alpha_at < zebra_at,
        "the binary must emit the task list id-sorted (alpha-fix before zebra-fix), \
         not in add order; got:\n{list_stdout}",
    );

    // A duplicate add-task intent rejects non-zero with a routed finding.
    let dup = run_milestone(
        repo.path(),
        home.path(),
        &["add-task", "cache-rework", "Alpha fix"],
    );
    let dup_stderr = String::from_utf8(dup.stderr).expect("utf-8 stderr");
    assert!(
        !dup.status.success(),
        "a duplicate add-task intent must exit non-zero; got {:?}",
        dup.status,
    );
    assert!(
        dup_stderr.contains("alpha-fix") && dup_stderr.contains("route:"),
        "the duplicate-add block must name the colliding sub-task and carry a route; got:\n{dup_stderr}",
    );
    // The collision appended nothing — the list is unchanged.
    let after = fs::read_to_string(milestone_dir.join("tasks.json")).expect("task list");
    assert_eq!(after, tasks_json, "a collision appends nothing to the list");

    // `.jigc/milestones/` is gitignored — the milestone area is never committed.
    let gitignore = fs::read_to_string(repo.path().join(".jigc").join(".gitignore"))
        .expect(".gitignore written");
    assert!(
        gitignore.lines().any(|l| l.trim() == "milestones/"),
        "`.jigc/.gitignore` must ignore `milestones/`; got:\n{gitignore}",
    );
    // Verified through git itself: the milestone area is ignored.
    let check = Command::new("git")
        .args(["check-ignore", ".jigc/milestones/cache-rework/base.json"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        check.status.success(),
        "git must treat `.jigc/milestones/` as ignored; got {:?}\nstderr:\n{}",
        check.status,
        String::from_utf8_lossy(&check.stderr),
    );
}

/// A committed `spec` with **three** repeatable `criteria` items — the seed
/// substrate `add-from-spec` enumerates. The three titles are deliberately not in
/// id-sorted physical order so the mint (physical) and the milestone's enumeration
/// (id-sorted) are distinct. Sub-task ids (the criterion-title slugs):
/// `rejects-the-101st-request`, `admits-within-the-window`,
/// `recovers-after-the-window` → id-sorted: admits, recovers, rejects.
const THREE_CRITERIA_SPEC: &str = "\
# Gateway rate limiting

## Goal

Bound per-client request volume at the gateway.

## Context

Downstream services were each enforcing limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.

### Recovers after the window  {#recovers}

The next window admits requests again.
";

/// A committed `spec` whose `criteria` section has **zero** items — the
/// "nothing to seed from" block fixture.
const ZERO_CRITERIA_SPEC: &str = "\
# Empty plan

## Goal

A goal with no criteria yet.

## Context

Context without any acceptance criteria.

## Criteria
";

/// Write `body` to the canonical committed spec path (`docs/specs/<slug>.md`) under
/// `repo` and `git add`/`commit` it, so the spec is genuinely committed state
/// (the done-criterion commits the spec before seeding from it).
fn commit_spec(repo: &Path, slug: &str, body: &str) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join(format!("{slug}.md")), body).expect("write spec");
    let git = |args: &[&str]| {
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
    };
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "add spec"]);
}

#[test]
fn add_from_spec_seeds_one_sub_task_per_criterion_through_the_binary() {
    let repo = TempDir::new("from-spec");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // A committed 3-criteria spec is the seed substrate.
    commit_spec(repo.path(), "gateway-rate-limiting", THREE_CRITERIA_SPEC);

    // Mint the milestone, then seed its task list from the committed spec.
    let created = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    let seeded = run_milestone(
        repo.path(),
        home.path(),
        &[
            "add-from-spec",
            "cache-rework",
            "spec:gateway-rate-limiting",
        ],
    );
    let seed_stdout = String::from_utf8(seeded.stdout).expect("utf-8 stdout");
    assert!(
        seeded.status.success(),
        "`jigc milestone add-from-spec` over a 3-criteria spec must exit 0; got {:?}\nstderr:\n{}",
        seeded.status,
        String::from_utf8_lossy(&seeded.stderr),
    );
    // The summary names the 3 seeded sub-tasks.
    for id in [
        "rejects-the-101st-request",
        "admits-within-the-window",
        "recovers-after-the-window",
    ] {
        assert!(
            seed_stdout.contains(id),
            "the seed summary must name sub-task `{id}`; got:\n{seed_stdout}",
        );
    }
    assert!(
        seed_stdout.contains('3'),
        "the seed summary must name the count (3); got:\n{seed_stdout}",
    );

    // `list-tasks` reports the 3 ids id-sorted (the order the join reads).
    let listed = run_milestone(repo.path(), home.path(), &["list-tasks", "cache-rework"]);
    let list_stdout = String::from_utf8(listed.stdout).expect("utf-8 stdout");
    assert!(listed.status.success(), "list-tasks must exit 0");
    let admits = list_stdout
        .find("admits-within-the-window")
        .expect("lists admits-within-the-window");
    let recovers = list_stdout
        .find("recovers-after-the-window")
        .expect("lists recovers-after-the-window");
    let rejects = list_stdout
        .find("rejects-the-101st-request")
        .expect("lists rejects-the-101st-request");
    assert!(
        admits < recovers && recovers < rejects,
        "the 3 sub-tasks must be id-sorted (admits < recovers < rejects); got:\n{list_stdout}",
    );
}

#[test]
fn add_from_spec_over_a_zero_criteria_spec_blocks_and_seeds_nothing() {
    let repo = TempDir::new("from-spec-empty");
    init_repo(repo.path());
    let home = TempDir::new("home");

    commit_spec(repo.path(), "empty-plan", ZERO_CRITERIA_SPEC);

    let created = run_milestone(repo.path(), home.path(), &["create", "Empty work"]);
    assert!(created.status.success(), "create must exit 0");

    let seeded = run_milestone(
        repo.path(),
        home.path(),
        &["add-from-spec", "empty-work", "spec:empty-plan"],
    );
    let stderr = String::from_utf8(seeded.stderr).expect("utf-8 stderr");
    assert!(
        !seeded.status.success(),
        "a zero-criteria spec must exit non-zero; got {:?}",
        seeded.status,
    );
    assert!(
        stderr.contains("no criteria") && stderr.contains("route:"),
        "the zero-criteria block must carry the `milestone.no-criteria` message + a route; got:\n{stderr}",
    );

    // Nothing was seeded — the task list is still empty.
    let listed = run_milestone(repo.path(), home.path(), &["list-tasks", "empty-work"]);
    let list_stdout = String::from_utf8(listed.stdout).expect("utf-8 stdout");
    assert!(listed.status.success(), "list-tasks must exit 0");
    assert!(
        list_stdout.contains("(0)"),
        "the task list must be empty after a blocked seed; got:\n{list_stdout}",
    );
}

/// Stage a doc `body` into a milestone sub-task's `docs/` area with a chosen
/// `provenance`, exactly as the staging primitives would — write
/// `tasks/<sub>/docs/<type>:<slug>.md` verbatim and record (merge into) the area's
/// `docs/provenance.json` manifest. These are the two inputs the by-task-id join
/// reads (the staged body + its provenance bit); the integration test writes them
/// directly because no front-door verb yet stages into a milestone sub-area.
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

/// An ADR body whose `supersedes` ref points at `to` (its OWN slug when `to` is its
/// own address) — the self-reference the collision-suffix rule must rewrite in
/// lockstep with the `-2` slug suffix.
fn adr_superseding(title: &str, to: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\nsupersedes: {to}\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A plain ADR body with no `supersedes` ref — a clean, disjoint doc.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// Current HEAD sha + the porcelain working-tree status of `repo` — the pair the
/// "nothing was committed" assertion compares before/after the join.
fn git_state(repo: &Path) -> (String, String) {
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()
        .expect("git rev-parse");
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .output()
        .expect("git status");
    (
        String::from_utf8(head.stdout).unwrap().trim().to_string(),
        String::from_utf8(status.stdout).unwrap(),
    )
}

#[test]
fn milestone_join_suffixes_a_created_collision_and_reports_the_decision() {
    let repo = TempDir::new("join-ok");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Mint the milestone + two sub-tasks, added in NON-id order (zed before low) so
    // id-order is not an accident of insertion order. id-sorted: [area-low, area-zed].
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

    // Both areas `created` an `adr:cache-strategy` that supersedes its OWN slug — a
    // collision of DISTINCT created instances (a suffix, not a clash). The lower task
    // id (`area-low`) keeps the bare slug; `area-zed` takes the deterministic `-2`
    // suffix, its self-ref rewritten in lockstep. `area-low` also stages a clean,
    // disjoint `edited-from-base` doc.
    stage_doc(
        repo.path(),
        "area-low",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy", "adr:cache-strategy"),
        "created",
    );
    stage_doc(
        repo.path(),
        "area-low",
        "adr:eviction-policy",
        &adr_plain("Eviction policy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "area-zed",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy", "adr:cache-strategy"),
        "created",
    );

    let before = git_state(repo.path());

    let joined = run_milestone(repo.path(), home.path(), &["join", "cache-rework"]);
    let stdout = String::from_utf8(joined.stdout).expect("utf-8 stdout");
    assert!(
        joined.status.success(),
        "a created-collision join must exit 0; got {:?}\nstderr:\n{}",
        joined.status,
        String::from_utf8_lossy(&joined.stderr),
    );

    // The summary names the merged docs, the `-2` suffix decision, and the self-ref
    // rewrite — read straight off the emitted bytes (the agent-facing artifact).
    assert!(
        stdout.contains("adr:cache-strategy-2"),
        "the join summary must name the `-2` suffixed instance; got:\n{stdout}",
    );
    assert!(
        stdout.contains("suffixed -2 on collision"),
        "the join summary must name the suffix decision; got:\n{stdout}",
    );
    assert!(
        stdout.contains("self-ref rewritten"),
        "the join summary must name the self-ref rewrite; got:\n{stdout}",
    );
    // The disjoint doc is merged but carries no suffix annotation.
    assert!(
        stdout.contains("adr:eviction-policy"),
        "the disjoint doc must be merged; got:\n{stdout}",
    );

    // Nothing was committed (the verb is not wired to finalize): no new commit and the
    // working tree is unchanged.
    let after = git_state(repo.path());
    assert_eq!(
        before, after,
        "the join must commit nothing and leave the working tree unchanged",
    );
}

#[test]
fn milestone_join_same_doc_clash_blocks_and_commits_nothing() {
    let repo = TempDir::new("join-clash");
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

    // Both areas write the SAME committed-at-base slug as `edited-from-base` — a
    // same-doc clash (a partition violation), blocking, never blind-merged.
    stage_doc(
        repo.path(),
        "area-low",
        "adr:cache-strategy",
        &adr_plain("Cache strategy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "area-zed",
        "adr:cache-strategy",
        &adr_plain("Cache strategy"),
        "edited-from-base",
    );

    let before = git_state(repo.path());

    let joined = run_milestone(repo.path(), home.path(), &["join", "cache-rework"]);
    let stderr = String::from_utf8(joined.stderr).expect("utf-8 stderr");
    assert!(
        !joined.status.success(),
        "a same-doc clash join must exit non-zero; got {:?}",
        joined.status,
    );
    assert!(
        stderr.contains("same-doc clash") && stderr.contains("route:"),
        "the clash block must carry the `join.same-doc-clash` message + a route; got:\n{stderr}",
    );
    assert!(
        stderr.contains("area-low") && stderr.contains("area-zed"),
        "the clash block must name both contending sub-tasks; got:\n{stderr}",
    );

    // Nothing was committed and the working tree is unchanged — a clash routes, it
    // never mutates the repo.
    let after = git_state(repo.path());
    assert_eq!(
        before, after,
        "a clash must commit nothing and leave the working tree unchanged",
    );
}

/// The number of commits reachable from HEAD (`git rev-list --count HEAD`) — the
/// "EXACTLY ONE new commit" assertion compares this before/after the finalize.
fn rev_list_count(repo: &Path) -> u32 {
    let out = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(repo)
        .output()
        .expect("git rev-list");
    String::from_utf8(out.stdout)
        .unwrap()
        .trim()
        .parse()
        .expect("count parses")
}

/// HEAD's full commit message (`git log -1 --format=%B`) — the synthesized-message
/// assertion reads it verbatim off the landed commit.
fn head_message(repo: &Path) -> String {
    let out = Command::new("git")
        .args(["log", "-1", "--format=%B"])
        .current_dir(repo)
        .output()
        .expect("git log");
    String::from_utf8(out.stdout).unwrap()
}

/// The subject line (first line) of each of the most recent `n` commits, **oldest
/// first** — the committed message *sequence* the `squash: false` determinism
/// assertion compares across feed orders. `%s` is the subject; `--reverse` flips
/// `git log`'s newest-first order so the list reads in commit (id-sorted) order.
fn recent_subjects(repo: &Path, n: u32) -> Vec<String> {
    let out = Command::new("git")
        .args(["log", &format!("-{n}"), "--reverse", "--format=%s"])
        .current_dir(repo)
        .output()
        .expect("git log subjects");
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

/// The `git ls-tree -r --name-only HEAD` listing — every path in HEAD's tree, sorted
/// — the committed *tree* the `squash: false` determinism assertion compares across
/// feed orders (paired with the message sequence).
fn head_tree_paths(repo: &Path) -> Vec<String> {
    let out = Command::new("git")
        .args(["ls-tree", "-r", "--name-only", "HEAD"])
        .current_dir(repo)
        .output()
        .expect("git ls-tree");
    let mut paths: Vec<String> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect();
    paths.sort();
    paths
}

/// Write `.jigc/config/manifest.yaml` setting the `finalize.fan-out.squash` knob to
/// `false` — the project override that opts a milestone finalize into per-sub-task
/// commits (the `scalar:` block shape `load_project_layer` reads).
fn set_squash_false(repo: &Path) {
    let config = repo.join(".jigc").join("config");
    fs::create_dir_all(&config).expect("mk config layer");
    fs::write(
        config.join("manifest.yaml"),
        "scalar:\n  finalize.fan-out.squash: false\n",
    )
    .expect("write manifest");
}

/// Stage a sub-task's **authored** `commit:<sub>` doc body in its working area
/// `.jigc/tasks/<sub>/docs/commit:<sub>.md` — the transient doc a fanned-out sub-agent
/// authors (a `feat` header + a per-sub-task summary), the prose the `squash: false`
/// per-sub-task render reads. The commit doc is workflow-provisioned `created` (the
/// `provision_commit_doc` precedent, `DECISIONS.md` 2026-06-04), so its provenance bit
/// is recorded like any staged doc — it is transient (no `location:`), so the join
/// never promotes it.
fn stage_subtask_commit(repo: &Path, sub: &str, summary: &str) {
    let body = format!(
        "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\n{summary}\n\n## Body\n\n\n\n## Trailers\n"
    );
    stage_doc(repo, sub, &format!("commit:{sub}"), &body, "created");
}

#[test]
fn milestone_finalize_commits_the_materialized_join_in_one_commit() {
    let repo = TempDir::new("finalize-ok");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Mint the milestone + two sub-tasks, added NON-id-order (zed before low) so the
    // suffix-by-task-id is not an accident of insertion order. id-sorted: [area-low,
    // area-zed], so `area-low` keeps the bare slug and `area-zed` takes the `-2` suffix.
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

    // `area-low`: a clean disjoint persisted doc + a `created` collision (self-ref).
    // `area-zed`: the same created slug → suffixed `-2`, self-ref rewritten in lockstep.
    stage_doc(
        repo.path(),
        "area-low",
        "adr:eviction-policy",
        &adr_plain("Eviction policy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "area-low",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy", "adr:cache-strategy"),
        "created",
    );
    stage_doc(
        repo.path(),
        "area-zed",
        "adr:cache-strategy",
        &adr_superseding("Cache strategy", "adr:cache-strategy"),
        "created",
    );

    let before_count = rev_list_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize cache-rework` must exit 0; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );

    // EXACTLY ONE new commit.
    assert_eq!(
        rev_list_count(repo.path()),
        before_count + 1,
        "finalize must land exactly one new commit",
    );

    // The synthesized message: subject names the milestone + its 2 sub-tasks, body lists
    // them id-sorted (`area-low` before `area-zed`).
    let message = head_message(repo.path());
    assert!(
        message.contains("Finalize milestone cache-rework (2 sub-tasks)"),
        "the commit message must be the synthesized projection; got:\n{message}",
    );
    let low_at = message.find("- area-low").expect("body lists area-low");
    let zed_at = message.find("- area-zed").expect("body lists area-zed");
    assert!(
        low_at < zed_at,
        "the synthesized body must list sub-tasks id-sorted; got:\n{message}",
    );

    // The promoted suffix-resolved docs landed at their canonical `docs/decisions/` paths.
    let decisions = repo.path().join("docs").join("decisions");
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            decisions.join(format!("{slug}.md")).is_file(),
            "the promoted `{slug}` doc must land at docs/decisions/{slug}.md",
        );
    }
    // The suffixed instance's self-ref was rewritten in lockstep to its `-2` slug.
    let suffixed =
        fs::read_to_string(decisions.join("cache-strategy-2.md")).expect("read suffixed");
    assert!(
        suffixed.contains("supersedes: adr:cache-strategy-2"),
        "the `-2` instance's self-ref must be rewritten to its suffixed slug; got:\n{suffixed}",
    );
    // The promoted docs are genuinely committed (in HEAD's tree), not just on disk.
    let tracked = Command::new("git")
        .args(["ls-files", "docs/decisions/"])
        .current_dir(repo.path())
        .output()
        .expect("git ls-files");
    let tracked = String::from_utf8(tracked.stdout).unwrap();
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            tracked.contains(&format!("docs/decisions/{slug}.md")),
            "docs/decisions/{slug}.md must be committed (tracked); got:\n{tracked}",
        );
    }

    // The working tree is clean after the commit (everything staged landed) and the
    // milestone area was removed.
    let (_, status) = git_state(repo.path());
    assert!(
        status.trim().is_empty(),
        "the working tree must be clean after the finalize commit; got:\n{status}",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("milestones")
            .join("cache-rework")
            .exists(),
        "the milestone area must be removed after a landed commit",
    );
}

/// Mint `Cache rework` + two sub-tasks under `repo` in the given `add_order`, opt the
/// project into `squash: false`, stage in each sub-area a clean disjoint persisted ADR
/// (so the parent aggregate has a real tree diff) + that sub-task's **authored**
/// `commit:<sub>` doc, then run `jigc milestone finalize`. Returns the finalize output.
/// Two callers feed divergent add orders to prove the committed sequence is
/// order-invariant. id-sorted sub-tasks: [area-low, area-zed].
fn finalize_squash_false(repo: &Path, home: &Path, add_order: &[&str]) -> std::process::Output {
    setup_squash_false(repo, home, add_order);
    run_milestone(repo, home, &["finalize", "cache-rework"])
}

/// The `finalize_squash_false` setup WITHOUT the terminating `finalize` call: opt into
/// `squash: false`, mint the milestone + two sub-tasks in `add_order`, and stage in each
/// sub-area a clean disjoint persisted ADR + that sub-task's authored `commit:<sub>` doc.
/// Split out so a caller can capture the pre-finalize HEAD/tree state between setup and
/// the `finalize` invocation (the transactionality assertion's baseline).
fn setup_squash_false(repo: &Path, home: &Path, add_order: &[&str]) {
    set_squash_false(repo);
    assert!(
        run_milestone(repo, home, &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in add_order {
        assert!(
            run_milestone(repo, home, &["add-task", "cache-rework", intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
    // Each sub-task: a clean disjoint persisted ADR (distinct slugs — no collision) +
    // its own authored commit doc with distinct prose.
    stage_doc(
        repo,
        "area-low",
        "adr:low-policy",
        &adr_plain("Low policy"),
        "edited-from-base",
    );
    stage_subtask_commit(repo, "area-low", "rework the low cache path");
    stage_doc(
        repo,
        "area-zed",
        "adr:zed-policy",
        &adr_plain("Zed policy"),
        "edited-from-base",
    );
    stage_subtask_commit(repo, "area-zed", "rework the zed cache path");
}

#[test]
fn milestone_finalize_squash_false_lands_n_plus_one_commits_in_id_order() {
    let repo = TempDir::new("finalize-squash-false");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let before = rev_list_count(repo.path());
    let finalized = finalize_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize` (squash:false) must exit 0; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );

    // N+1 = 3 new commits: one per sub-task (2) + the parent aggregate.
    assert_eq!(
        rev_list_count(repo.path()),
        before + 3,
        "squash:false must land N+1 commits (2 sub-task + 1 parent)",
    );

    // The committed SEQUENCE (oldest first): the two sub-task authored subjects in
    // id-sorted order (area-low before area-zed, NOT add order), then the parent's
    // synthesized aggregate subject.
    let subjects = recent_subjects(repo.path(), 3);
    assert_eq!(
        subjects,
        vec![
            "feat: rework the low cache path".to_owned(),
            "feat: rework the zed cache path".to_owned(),
            "Finalize milestone cache-rework (2 sub-tasks)".to_owned(),
        ],
        "the commit sequence must be the id-sorted sub-task messages then the parent aggregate",
    );

    // The parent aggregate landed the merged tree: both promoted ADRs are committed.
    let tracked = head_tree_paths(repo.path());
    assert!(
        tracked.contains(&"docs/decisions/low-policy.md".to_owned())
            && tracked.contains(&"docs/decisions/zed-policy.md".to_owned()),
        "the parent aggregate must commit the merged persisted docs; got:\n{tracked:?}",
    );

    // Clean tree + the milestone area removed after the boundary.
    let (_, status) = git_state(repo.path());
    assert!(
        status.trim().is_empty(),
        "the working tree must be clean after the squash:false boundary; got:\n{status}",
    );
}

#[test]
fn milestone_finalize_squash_false_sequence_is_byte_identical_across_feed_orders() {
    // Validation hardening #7: the SAME two sub-tasks (each authoring its own commit doc)
    // finalized under two DIVERGENT add orders (id-order and its reverse) must land a
    // byte-identical committed sequence — the per-sub-task commit messages AND the parent
    // tree, ordered by task id, never by add/feed order.
    let home = TempDir::new("home");

    let forward = TempDir::new("squash-false-fwd");
    init_repo(forward.path());
    let fwd = finalize_squash_false(forward.path(), home.path(), &["Area low", "Area zed"]);
    assert!(fwd.status.success(), "forward finalize must exit 0");

    let reverse = TempDir::new("squash-false-rev");
    init_repo(reverse.path());
    let rev = finalize_squash_false(reverse.path(), home.path(), &["Area zed", "Area low"]);
    assert!(rev.status.success(), "reverse finalize must exit 0");

    // The message sequence (subjects, oldest first) is byte-identical across feed orders.
    let fwd_subjects = recent_subjects(forward.path(), 3);
    let rev_subjects = recent_subjects(reverse.path(), 3);
    assert_eq!(
        fwd_subjects,
        vec![
            "feat: rework the low cache path".to_owned(),
            "feat: rework the zed cache path".to_owned(),
            "Finalize milestone cache-rework (2 sub-tasks)".to_owned(),
        ],
        "the forward feed must land the id-sorted commit sequence",
    );
    assert_eq!(
        fwd_subjects, rev_subjects,
        "the committed message sequence must be byte-identical across divergent feed orders",
    );

    // The committed tree (the parent aggregate's promoted docs) is identical too.
    assert_eq!(
        head_tree_paths(forward.path()),
        head_tree_paths(reverse.path()),
        "the committed tree must be byte-identical across divergent feed orders",
    );
}

/// Install a `pre-commit` hook in `repo` that rejects any commit which stages a path
/// under `docs/decisions/` (the promoted ADRs the parent aggregate stages via `git add
/// --all`). The `squash: false` per-sub-task commits run `git commit --allow-empty`
/// staging nothing, so they pass; only the parent aggregate trips the hook — a clean,
/// controllable failure point AFTER the N sub-task commits have landed.
fn install_aggregate_rejecting_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        "#!/bin/sh\nif git diff --cached --name-only | grep -q '^docs/decisions/'; then\n  echo 'aggregate rejected by test hook' >&2\n  exit 1\nfi\nexit 0\n",
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

#[test]
fn milestone_finalize_squash_false_aggregate_failure_resets_to_pre_finalize_head() {
    // The `squash: false` boundary must be all-or-nothing: the N per-sub-task commits
    // advance HEAD BEFORE the parent aggregate lands. If the aggregate commit fails
    // (here: a `pre-commit` hook rejects the staged ADRs), HEAD must reset to the
    // pre-finalize sha — no orphaned sub-task commits, clean tree, as-if-finalize-was-
    // never-called (`CLAUDE.md`: "Writes are transactional"; `finalize.md` rollback
    // discipline). RED before the fix (HEAD left advanced by N), GREEN after.
    let repo = TempDir::new("finalize-squash-false-reset");
    init_repo(repo.path());
    let home = TempDir::new("home");

    install_aggregate_rejecting_hook(repo.path());

    // Set up the milestone + two sub-tasks + staged docs, then capture the pre-finalize
    // HEAD + status JUST before the `finalize` call — the "as-if-never-called" baseline
    // (the staged `.jigc/` working area is part of this baseline, left intact on failure).
    setup_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);
    let (before_head, before_status) = git_state(repo.path());
    let before_count = rev_list_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);

    // The aggregate commit was rejected, so the boundary fails (no parent aggregate).
    assert!(
        !finalized.status.success(),
        "the aggregate-rejecting hook must make `jigc milestone finalize` fail; got success\nstdout:\n{}",
        String::from_utf8_lossy(&finalized.stdout),
    );

    // HEAD is back at the pre-finalize sha — the N sub-task commits were rolled back,
    // not left orphaned on HEAD.
    let (after_head, after_status) = git_state(repo.path());
    assert_eq!(
        after_head, before_head,
        "on aggregate failure HEAD must reset to the pre-finalize sha (no orphaned sub-task commits)",
    );
    assert_eq!(
        rev_list_count(repo.path()),
        before_count,
        "on aggregate failure the commit count must be unchanged (the N sub-task commits are gone)",
    );

    // The working tree is restored to the pre-finalize state — no dangling staging from
    // the sub-task commits and no leftover promoted ADR from the failed aggregate. The
    // staged `.jigc/` working area is left intact (the executor's "working area intact on
    // failure" guarantee), so the status matches the pre-finalize baseline byte-for-byte.
    assert_eq!(
        after_status, before_status,
        "on aggregate failure the working tree must match the pre-finalize state (as-if-finalize-was-never-called)",
    );
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "on aggregate failure no promoted ADR may be left in `docs/decisions/`",
    );
}

/// The marker a non-blocking `pre-commit` hook writes to stderr before exiting 0 — git
/// redirects a hook's own stdout to stderr, so a warn-only hook (the M19 doc↔code
/// backstop's shape) speaks on the hook stream `git_commit` captures.
const HOOK_WARNING: &str = "NON-BLOCKING-MILESTONE-HOOK-WARNING";

/// Install a **non-blocking** `pre-commit` hook in `repo` that prints `HOOK_WARNING` and
/// exits 0 — git surfaces it on every commit's stderr (the hook stream). It fires on the
/// N per-sub-task `commit_empty_message` commits AND the parent aggregate, so the relay's
/// once-not-(N+1) discipline (`design/finalize.md` → 6. Commit, review B1) is asserted at
/// the binary: only the aggregate's capture is relayed.
fn install_warning_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!("#!/bin/sh\necho {HOOK_WARNING} 1>&2\nexit 0\n"),
    )
    .expect("write the non-blocking pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).expect("hook metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).expect("chmod hook");
    }
}

/// M19 increment 2, T3 — a milestone finalize relays the **aggregate** commit's
/// non-blocking hook output to the agent, exactly ONCE, never once-per-sub-task
/// (`design/finalize.md` → 6. Commit, review B1: fan-out relays only the aggregate). The
/// `pre-commit` hook fires on **every** commit, so a `squash: false` finalize runs it N+1
/// times (N tree-empty `commit_empty_message` per-sub-task commits + 1 aggregate); the N
/// per-sub-task fires are intentionally swallowed (`commit_empty_message` relays nothing)
/// and only the aggregate's capture surfaces. If the per-sub-task commits leaked into the
/// relay the warning would appear N+1 times — the bug bullet B1 guards against. The
/// `squash: true` default relays the single aggregate's output the same way.
#[test]
fn milestone_finalize_relays_only_the_aggregate_hook_output() {
    // ── squash:false: 2 sub-task commits + 1 aggregate; warning relayed ONCE ──────
    let repo = TempDir::new("relay-squash-false");
    init_repo(repo.path());
    let home = TempDir::new("home");
    install_warning_hook(repo.path());

    let before = rev_list_count(repo.path());
    let finalized = finalize_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);
    assert!(
        finalized.status.success(),
        "a non-blocking hook must not block the squash:false finalize; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );
    // N+1 = 3 commits landed (2 per-sub-task + 1 aggregate), so the hook fired 3 times —
    // the precondition that makes once-not-(N+1) a genuine distinction.
    assert_eq!(
        rev_list_count(repo.path()),
        before + 3,
        "squash:false must land N+1 commits, so the hook fires N+1 times",
    );
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout.matches(HOOK_WARNING).count(),
        1,
        "the aggregate hook warning must be relayed EXACTLY once — not N+1 times, one per \
         sub-task commit (review B1: fan-out relays only the aggregate); got:\n{stdout}",
    );
    assert!(
        stdout.contains("--- hook output ---"),
        "the relay must land in the delimited section; got:\n{stdout}",
    );

    // ── squash:true: the single aggregate's output is relayed the same way ─────────
    let repo = TempDir::new("relay-squash-true");
    init_repo(repo.path());
    let home = TempDir::new("home");
    install_warning_hook(repo.path());

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

    let before = rev_list_count(repo.path());
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "a non-blocking hook must not block the squash:true finalize; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );
    // squash:true lands EXACTLY one commit (the synthesized aggregate), so the hook fires
    // once — the relay surfaces it once.
    assert_eq!(
        rev_list_count(repo.path()),
        before + 1,
        "squash:true must land exactly one aggregate commit",
    );
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout.matches(HOOK_WARNING).count(),
        1,
        "the squash:true aggregate's hook warning must be relayed once; got:\n{stdout}",
    );
    assert!(
        stdout.contains("--- hook output ---"),
        "the relay must land in the delimited section; got:\n{stdout}",
    );
}

#[test]
fn milestone_finalize_removes_every_sub_task_working_area_on_a_landed_commit() {
    // A landed milestone finalize must clean up the per-sub-task working areas
    // (`.jigc/tasks/<sub>/`) in addition to the milestone area — they are gitignored
    // runtime state that otherwise accumulates across milestone runs and can trip a
    // later top-level `jigc doc`'s ">1 active task" bail. Cleanup is best-effort and
    // happens ONLY after the commit boundary succeeds (a failed/rolled-back finalize
    // leaves the areas intact for retry — proven by the reset test above). RED before
    // the fix (the sub-task areas persist), GREEN after.
    let repo = TempDir::new("finalize-subtask-cleanup");
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

    // Each sub-task stages a clean, disjoint persisted ADR (distinct slugs — no
    // collision), so the parent aggregate has a real tree diff and the boundary lands.
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

    // Both sub-task working areas exist before the finalize.
    let tasks = repo.path().join(".jigc").join("tasks");
    for sub in ["area-low", "area-zed"] {
        assert!(
            tasks.join(sub).is_dir(),
            "sub-task `{sub}` working area must exist before finalize",
        );
    }

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize cache-rework` must exit 0; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );

    // The milestone area was removed (as before)...
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("milestones")
            .join("cache-rework")
            .exists(),
        "the milestone area must be removed after a landed commit",
    );
    // ...AND every sub-task working area is removed too.
    for sub in ["area-low", "area-zed"] {
        assert!(
            !tasks.join(sub).exists(),
            "sub-task `{sub}` working area must be removed after a landed milestone finalize",
        );
    }
}

#[test]
fn milestone_finalize_same_doc_clash_blocks_and_commits_nothing() {
    let repo = TempDir::new("finalize-clash");
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

    // Both areas edit the SAME committed-at-base slug — a same-doc clash (a blocking
    // finding inside the join's Ok outcome), so finalize must block BEFORE materializing
    // or committing (planner-note (c)).
    stage_doc(
        repo.path(),
        "area-low",
        "adr:cache-strategy",
        &adr_plain("Cache strategy"),
        "edited-from-base",
    );
    stage_doc(
        repo.path(),
        "area-zed",
        "adr:cache-strategy",
        &adr_plain("Cache strategy"),
        "edited-from-base",
    );

    let before = git_state(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");
    assert!(
        !finalized.status.success(),
        "a same-doc clash finalize must exit non-zero; got {:?}",
        finalized.status,
    );
    assert!(
        stderr.contains("same-doc clash") && stderr.contains("route:"),
        "the clash block must carry the `join.same-doc-clash` message + a route; got:\n{stderr}",
    );

    // NO new commit and the working tree is unchanged — a clash routes, never mutates.
    let after = git_state(repo.path());
    assert_eq!(
        before, after,
        "a clash must commit nothing and leave HEAD + the working tree unchanged",
    );
    // No docs/decisions/ doc was promoted.
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "a blocked finalize must promote nothing",
    );
}

#[test]
fn add_task_to_an_unknown_milestone_rejects() {
    let repo = TempDir::new("unknown");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let out = run_milestone(
        repo.path(),
        home.path(),
        &["add-task", "no-such-milestone", "Some fix"],
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(
        !out.status.success(),
        "add-task against an unknown milestone must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("no-such-milestone") && stderr.contains("route:"),
        "the unknown-milestone block must name it and carry a route; got:\n{stderr}",
    );
}

#[test]
fn execute_an_unknown_milestone_routes_a_block_and_composes_nothing() {
    let repo = TempDir::new("execute-unknown");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // `jigc milestone execute <unknown>` must reject before any compose: the
    // unknown-milestone block routes to stderr, exits non-zero, and emits no
    // composed view on stdout (`design/write-commands.md` → Executing the milestone).
    let out = run_milestone(repo.path(), home.path(), &["execute", "no-such-milestone"]);
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        !out.status.success(),
        "execute against an unknown milestone must exit non-zero; got {:?}",
        out.status,
    );
    assert!(
        stderr.contains("no-such-milestone") && stderr.contains("route:"),
        "the unknown-milestone block must name it and carry a route; got:\n{stderr}",
    );
    // Composed nothing — no `Spawn:` directive, no composed view reached stdout.
    assert!(
        stdout.trim().is_empty(),
        "an unknown-milestone execute must compose nothing (empty stdout); got:\n{stdout}",
    );
}

/// Extract every `` Spawn: `<launch-line>` `` directive's backtick-wrapped launch
/// line from a composed view's bytes, **in emit order** — the agent-facing artifact
/// (the bytes the adapter launches), never a reconstructed equivalent.
fn spawn_lines(composed: &str) -> Vec<String> {
    composed
        .lines()
        .filter_map(|l| l.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

#[test]
fn milestone_execute_composes_the_real_fanout_join_finalize_workflow() {
    let repo = TempDir::new("execute-real");
    init_repo(repo.path());
    let home = TempDir::new("home");
    // `execute` resolves the cascade (phase-2/4/5 owners + pack defaults), so the
    // project must be set up — an empty `.jigc/config/` layer is the marker.
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config layer");

    // Mint a milestone + two sub-tasks added in NON-id order (zebra before alpha) so the
    // id-sorted Spawn emit is not an accident of insertion order.
    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Zebra fix", "Alpha fix"] {
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

    let out = run_milestone(repo.path(), home.path(), &["execute", "cache-rework"]);
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(
        out.status.success(),
        "`jigc milestone execute cache-rework` must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // The fan-out emits EXACTLY N (=2) `Spawn:` directives, one per sub-task, read
    // straight off the composed bytes the agent runs — never a hand-built equivalent.
    let spawns = spawn_lines(&stdout);
    assert_eq!(
        spawns,
        vec![
            "jigc workflow sub-task --task alpha-fix".to_owned(),
            "jigc workflow sub-task --task zebra-fix".to_owned(),
        ],
        "the real `milestone-execution` workflow must emit one id-sorted `Spawn:` directive \
         per sub-task (alpha before zebra, not add order); got:\n{stdout}",
    );

    // The join barrier prose sits AFTER the spawns and BEFORE the finalize Run line.
    let join_at = stdout
        .find("merged by task-id order")
        .expect("the join-tasks step's barrier prose must compose into the view");
    let last_spawn_at = stdout
        .rfind("Spawn: `jigc workflow")
        .expect("at least one Spawn directive");
    let run_at = stdout
        .find("Run: `jigc milestone finalize <MILESTONE_ID>`")
        .expect("the milestone-finalize step must resolve a `Run:` line");
    assert!(
        last_spawn_at < join_at && join_at < run_at,
        "the view must be fan-out → join barrier → finalize Run, in that order; got:\n{stdout}",
    );

    // Composition is fan-out-join-paired — a paired workflow raises no
    // `workflow-refs.fan-out-join-paired` block on stdout/stderr.
    assert!(
        !stdout.contains("fan-out-join-paired"),
        "a paired fan-out/join workflow must not raise the pairing block; got:\n{stdout}",
    );
}

/// Mint `milestone` under `repo` with its sub-tasks added in `add_order`, set up the
/// cascade layer, then run `jigc milestone execute` and return the composed stdout.
/// Two callers feed divergent add orders to prove the Spawn emit is order-invariant.
fn execute_with_add_order(repo: &Path, home: &Path, milestone: &str, add_order: &[&str]) -> String {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk config layer");
    assert!(
        run_milestone(repo, home, &["create", milestone])
            .status
            .success(),
        "create must exit 0",
    );
    let slug = milestone.to_lowercase().replace(' ', "-");
    for intent in add_order {
        assert!(
            run_milestone(repo, home, &["add-task", &slug, intent])
                .status
                .success(),
            "add-task `{intent}` must exit 0",
        );
    }
    let out = run_milestone(repo, home, &["execute", &slug]);
    assert!(
        out.status.success(),
        "execute must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn milestone_execute_spawn_emit_is_byte_identical_across_divergent_add_orders() {
    // Validation hardening #7: the same three sub-tasks added in two DIVERGENT orders
    // (id-order and its reverse) must emit a byte-identical id-sorted `Spawn:` block —
    // the enumeration sorts on resolve, never leaking add/read order into the output.
    let home = TempDir::new("home");

    let forward = TempDir::new("exec-order-fwd");
    init_repo(forward.path());
    let fwd_stdout = execute_with_add_order(
        forward.path(),
        home.path(),
        "Cache rework",
        &["Alpha fix", "Mid fix", "Zebra fix"],
    );

    let reverse = TempDir::new("exec-order-rev");
    init_repo(reverse.path());
    let rev_stdout = execute_with_add_order(
        reverse.path(),
        home.path(),
        "Cache rework",
        &["Zebra fix", "Mid fix", "Alpha fix"],
    );

    let fwd_spawns = spawn_lines(&fwd_stdout);
    let rev_spawns = spawn_lines(&rev_stdout);
    assert_eq!(
        fwd_spawns,
        vec![
            "jigc workflow sub-task --task alpha-fix".to_owned(),
            "jigc workflow sub-task --task mid-fix".to_owned(),
            "jigc workflow sub-task --task zebra-fix".to_owned(),
        ],
        "the forward add order must emit id-sorted Spawn directives",
    );
    assert_eq!(
        fwd_spawns, rev_spawns,
        "the `Spawn:` block must be byte-identical across divergent add orders (id-sorted on \
         resolve); forward:\n{fwd_stdout}\nreverse:\n{rev_stdout}",
    );
}

/// Read the persisted minting-workflow id of a sub-task from its isolated working
/// area (`.jigc/tasks/<sub>/workflow`) — the `read_workflow_id` companion of the
/// engine mint, surfaced through the on-disk file the CLI threads `--workflow` into.
fn sub_task_workflow(repo: &Path, sub: &str) -> String {
    let path = repo.join(".jigc").join("tasks").join(sub).join("workflow");
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("sub-task `{sub}` must persist its minting workflow: {e}"))
}

#[test]
fn add_task_records_the_default_and_explicit_minting_workflow() {
    let repo = TempDir::new("workflow-arg");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let created = run_milestone(repo.path(), home.path(), &["create", "Cache rework"]);
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    // No `--workflow` → the new default `sub-task` is recorded (no longer the
    // hardcoded `single-task`).
    let defaulted = run_milestone(
        repo.path(),
        home.path(),
        &["add-task", "cache-rework", "Default fix"],
    );
    assert!(
        defaulted.status.success(),
        "`jigc milestone add-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&defaulted.stderr),
    );
    assert_eq!(
        sub_task_workflow(repo.path(), "default-fix"),
        "sub-task",
        "with no `--workflow`, the sub-task must record the default `sub-task`",
    );

    // Explicit `--workflow single-task` → that value is recorded.
    let explicit = run_milestone(
        repo.path(),
        home.path(),
        &[
            "add-task",
            "cache-rework",
            "Explicit fix",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        explicit.status.success(),
        "`jigc milestone add-task --workflow single-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&explicit.stderr),
    );
    assert_eq!(
        sub_task_workflow(repo.path(), "explicit-fix"),
        "single-task",
        "with `--workflow single-task`, the sub-task must record `single-task`",
    );
}

/// `git rev-parse HEAD` in `dir` — when `dir` is a linked worktree this reads the
/// worktree's own detached HEAD, so the provision test can prove each worktree is
/// detached at the milestone base pin (C0), not at advanced main HEAD (C1).
fn rev_parse_head(dir: &Path) -> String {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .expect("run git rev-parse");
    assert!(out.status.success(), "rev-parse HEAD failed in {dir:?}");
    String::from_utf8(out.stdout)
        .expect("utf-8 head")
        .trim()
        .to_string()
}

/// Count the immediate sub-directories of `dir` (the provisioned worktree dirs) —
/// the "exactly N worktrees / same N on a re-run" assertion.
fn dir_child_count(dir: &Path) -> usize {
    fs::read_dir(dir)
        .expect("read worktrees dir")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .count()
}

#[test]
fn milestone_provision_creates_detached_base_pin_worktrees_idempotently() {
    let repo = TempDir::new("provision");
    let pin = init_repo(repo.path()); // C0 — the milestone base pin
    let home = TempDir::new("home");

    // Mint the milestone + two sub-tasks, added in NON-id order (zed before low) so the
    // id-sorted provisioning is not an accident of insertion order. id-sorted:
    // [area-low, area-zed].
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

    // Advance MAIN past the pin AFTER minting, so jigc_home's HEAD (C1) != the pin (C0).
    // The provisioned worktrees must detach at the recorded base PIN, never at main HEAD.
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
    };
    fs::write(repo.path().join("advance.txt"), "more\n").expect("write advance.txt");
    git(&["add", "advance.txt"]);
    git(&["commit", "-q", "-m", "advance main"]);
    let main_head = rev_parse_head(repo.path());
    assert_ne!(main_head, pin, "main HEAD must have advanced past the pin");

    let wt_root = repo.path().join(".jigc").join("worktrees");

    // Inject a STALE, non-registered leftover dir at one worktree path (a crashed-run
    // remnant): provision must clear it and still succeed (the stale-leftover idempotency).
    let stale = wt_root.join("area-low");
    fs::create_dir_all(&stale).expect("mk stale leftover dir");
    fs::write(stale.join("junk.txt"), "leftover\n").expect("write junk");

    // Provision #1 — over the stale leftover.
    let provisioned = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "`jigc milestone provision cache-rework` must exit 0; got {:?}\nstderr:\n{}",
        provisioned.status,
        String::from_utf8_lossy(&provisioned.stderr),
    );

    // (a) EXACTLY N=2 worktrees, each a *linked* worktree detached at the base pin.
    let subs = ["area-low", "area-zed"];
    for sub in subs {
        let wt = wt_root.join(sub);
        assert!(
            wt.is_dir(),
            "worktree `{sub}` must exist under .jigc/worktrees/",
        );
        assert!(
            wt.join(".git").is_file(),
            "`{sub}` must be a linked worktree (its `.git` is a file pointer)",
        );
        assert_eq!(
            rev_parse_head(&wt),
            pin,
            "`{sub}` must be detached at the base pin (C0), not main HEAD (C1)",
        );
    }
    assert_eq!(
        dir_child_count(&wt_root),
        2,
        "exactly N=2 worktrees must be provisioned, no extras",
    );

    // (c) gitignored — the provisioned worktrees stay out of the main checkout's
    // `git status`, and git agrees via `check-ignore`.
    let (_, status) = git_state(repo.path());
    assert!(
        !status.contains("worktrees"),
        "the provisioned worktrees must not pollute the main checkout `git status`; got:\n{status}",
    );
    let check = Command::new("git")
        .args(["check-ignore", ".jigc/worktrees/area-low"])
        .current_dir(repo.path())
        .output()
        .expect("run git check-ignore");
    assert!(
        check.status.success(),
        "git must treat `.jigc/worktrees/` as ignored; got {:?}\nstderr:\n{}",
        check.status,
        String::from_utf8_lossy(&check.stderr),
    );

    // (b) idempotent — a SECOND run exits 0 and yields the same 2 worktrees at the pin.
    let again = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        again.status.success(),
        "a second provision must exit 0 (idempotent); got {:?}\nstderr:\n{}",
        again.status,
        String::from_utf8_lossy(&again.stderr),
    );
    for sub in subs {
        assert_eq!(
            rev_parse_head(&wt_root.join(sub)),
            pin,
            "`{sub}` must still be detached at the pin after the idempotent re-run",
        );
    }
    assert_eq!(
        dir_child_count(&wt_root),
        2,
        "the idempotent re-run must yield the same 2 worktrees (no duplicates)",
    );
}
