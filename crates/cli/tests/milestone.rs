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

/// Write `body` to the canonical committed spec path (`specs/<slug>.md`) under
/// `repo` and `git add`/`commit` it, so the spec is genuinely committed state
/// (the done-criterion commits the spec before seeding from it).
fn commit_spec(repo: &Path, slug: &str, body: &str) {
    let specs = repo.join("specs");
    fs::create_dir_all(&specs).expect("mk specs/");
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

    // The promoted suffix-resolved docs landed at their canonical `decisions/` paths.
    let decisions = repo.path().join("decisions");
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            decisions.join(format!("{slug}.md")).is_file(),
            "the promoted `{slug}` doc must land at decisions/{slug}.md",
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
        .args(["ls-files", "decisions/"])
        .current_dir(repo.path())
        .output()
        .expect("git ls-files");
    let tracked = String::from_utf8(tracked.stdout).unwrap();
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            tracked.contains(&format!("decisions/{slug}.md")),
            "decisions/{slug}.md must be committed (tracked); got:\n{tracked}",
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
    // No decisions/ doc was promoted.
    assert!(
        !repo.path().join("decisions").exists(),
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
