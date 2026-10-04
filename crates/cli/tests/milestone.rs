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
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-milestone-cli-{tag}-{}-{:?}",
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
    // The project cascade layer — `jigc milestone`'s door-top precondition (M52 Inc 8 / T1).
    crate::support::mint_project_layer(root);
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
    // The real `milestone create` path writes the shared canonical union entry set
    // (M39 3→1 collapse) — byte-exact, so a re-divergence from the one source of truth
    // (e.g. a dropped `worktrees/`) is caught here at the milestone.rs write path.
    assert_eq!(
        gitignore, "tasks/\nindex/\nstate/\nmilestones/\nworktrees/\nlogs/\ndisplaced/\n",
        "`.jigc/.gitignore` must be the canonical union set incl. `worktrees/` + the \
         `displaced/` relocation workbench; got:\n{gitignore}",
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
    // Round-2 D6h: the printed block carries its stable finding KEY (the funnel used
    // to drop it, unlike every envelope-rendered sibling)…
    assert!(
        stderr.contains("blocking · milestone.no-criteria — "),
        "the agent-format block must lead with `blocking · milestone.no-criteria`; got:\n{stderr}",
    );
    // …and the route's first arm names the executable in-task repair path.
    assert!(
        stderr.contains("`jigc doc add-item spec:empty-plan#criteria"),
        "the route names the in-task add-item repair; got:\n{stderr}",
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
        "---\nstatus: accepted\ndate: 2026-06-04\nsupersedes: {to}\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
    )
}

/// A plain ADR body with no `supersedes` ref — a clean, disjoint doc.
fn adr_plain(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-06-04\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nDo the thing.\n\n## Consequences\n\nTradeoffs.\n"
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

    // Mint the milestone + three sub-tasks, added in NON-id order (zed before low) so
    // id-order is not an accident of insertion order. id-sorted: [area-idle, area-low,
    // area-zed]. `area-idle` stages NOTHING — the C3 ack must name it as contributing
    // no docs instead of silently crediting it.
    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low", "Area idle"] {
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
    // C3 (round-2 surface fixes) — the ack states its effect FULLY: the sub-task that
    // staged no docs is named, instead of being silently invisible in a merged list
    // that only credits contributors.
    assert!(
        stdout.contains("no docs staged from: area-idle"),
        "the join ack must name the sub-task that contributed no docs; got:\n{stdout}",
    );
    let no_docs_line = stdout
        .lines()
        .find(|l| l.contains("no docs staged from:"))
        .expect("the no-docs line exists");
    assert!(
        !no_docs_line.contains("area-low") && !no_docs_line.contains("area-zed"),
        "the contributing sub-tasks must NOT be listed as doc-less; got:\n{no_docs_line}",
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
    // `area-idle` is minted alongside the two contenders and stages NOTHING — the
    // control that separates *"staged nothing"* from *"staged the doc that lost to the
    // block"* (M51 Increment 9 / T6; charter Tier 2 **EC-15**).
    for intent in ["Area zed", "Area low", "Area idle"] {
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

    // The doc-less line is a fact about the **disk**, not about what survived the block:
    // each contender staged the clashing doc, so neither may be reported as having staged
    // nothing, while `area-idle` — which staged nothing — must still be named (EC-15).
    let stdout = String::from_utf8(joined.stdout).expect("utf-8 stdout");
    let no_docs_line = stdout
        .lines()
        .find(|l| l.contains("no docs staged from:"))
        .unwrap_or_else(|| {
            panic!("the blocked join ack carries the doc-less line; got:\n{stdout}")
        });
    assert!(
        no_docs_line.contains("area-idle"),
        "the sub-task that staged nothing must be named; got:\n{no_docs_line}",
    );
    assert!(
        !no_docs_line.contains("area-low") && !no_docs_line.contains("area-zed"),
        "a contender that staged the clashing doc must NOT be listed as doc-less; \
         got:\n{no_docs_line}",
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

    // Mint the milestone + three sub-tasks, added NON-id-order (zed before low) so the
    // suffix-by-task-id is not an accident of insertion order. id-sorted: [area-idle,
    // area-low, area-zed], so `area-low` keeps the bare slug and `area-zed` takes the
    // `-2` suffix. `area-idle` stages NOTHING — the C2 landing manifest must make the
    // no-work sub-task visible instead of silently crediting it.
    assert!(
        run_milestone(repo.path(), home.path(), &["create", "Cache rework"])
            .status
            .success(),
        "create must exit 0",
    );
    for intent in ["Area zed", "Area low", "Area idle"] {
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

    // C2 (round-2 surface fixes) — the landing manifest, on the EMITTED bytes: the
    // highest-stakes commit boundary must not succeed with empty stdout (the blind
    // reader fell back to raw `git show` — the exact drive-around the tool exists to
    // prevent). The `jigc task finalize` mold: `finalized <sha> — <subject>` + the
    // manifest of landed files + the count + the per-sub-task contribution line, the
    // no-work sub-task visible instead of silently credited.
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("finalized ")
            && stdout.contains("— Finalize milestone cache-rework (3 sub-tasks)"),
        "the landing must print `finalized <sha> — <subject>`; got:\n{stdout}",
    );
    for line in [
        "  promoted docs/decisions/cache-strategy.md",
        "  promoted docs/decisions/cache-strategy-2.md",
        "  promoted docs/decisions/eviction-policy.md",
    ] {
        assert!(
            stdout.contains(line),
            "the landing manifest must carry `{line}`; got:\n{stdout}",
        );
    }
    // 4 = the 3 promoted docs + the first-commit `.jigc/.gitignore` config layer the
    // boundary also lands (named `added` in the manifest — the set is the commit's
    // real delta, never a curated subset).
    assert!(
        stdout.contains("4 files committed"),
        "the landing must carry the file count; got:\n{stdout}",
    );
    // The docs-only milestone provisions no worktrees at all, so EVERY sub-task carries the
    // `no worktree provisioned` clause (M47 Inc 3 (b)(ii)): with no worktree a sub-task could
    // not have contributed code even in principle, and the manifest says so rather than
    // letting `0 code files` read as "the sub-agent staged nothing."
    assert!(
        stdout.contains(
            "sub-tasks: area-idle: nothing staged, no worktree provisioned · \
             area-low: 2 docs, no worktree provisioned · area-zed: 1 doc, no worktree provisioned"
        ),
        "the landing must name each sub-task's contribution, the no-work one included; got:\n{stdout}",
    );

    // EXACTLY ONE new commit.
    assert_eq!(
        rev_list_count(repo.path()),
        before_count + 1,
        "finalize must land exactly one new commit",
    );

    // The synthesized message: subject names the milestone + its 3 sub-tasks, body lists
    // them id-sorted (`area-low` before `area-zed`).
    let message = head_message(repo.path());
    assert!(
        message.contains("Finalize milestone cache-rework (3 sub-tasks)"),
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
/// `squash: false`, mint the milestone + two sub-tasks in `add_order`, stage in each
/// sub-area a clean disjoint persisted ADR + that sub-task's authored `commit:<sub>` doc,
/// **provision the N base-pin worktrees, and stage disjoint code in each** — the honest
/// rework (M31 Inc 5) commits that worktree code per sub-task. Split out so a caller can
/// capture the pre-finalize HEAD/tree state between setup and the `finalize` invocation
/// (the transactionality assertion's baseline). Main is NOT advanced, so the base pin ==
/// HEAD (the finalize preflight requires it).
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

    // Provision the N base-pin worktrees and stage DISJOINT code in each — the code each
    // per-sub-task commit carries under the honest rework. Idempotent (a caller may
    // re-provision).
    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    stage_worktree_code(repo, "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo, "area-zed", "src/zed.rs", "pub fn zed() {}\n");
}

/// The repo-relative paths a single commit changed (`git show --name-only --format= <rev>`)
/// — proves a per-sub-task commit carries **that** sub-task's files (a real tree), not the
/// retired tree-empty `--allow-empty` form, and that the merged docs ride the aggregate.
fn commit_changed_files(repo: &Path, rev: &str) -> Vec<String> {
    let out = Command::new("git")
        .args(["show", "--name-only", "--format=", rev])
        .current_dir(repo)
        .output()
        .expect("git show --name-only");
    String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect()
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

    // Each per-sub-task commit carries THAT sub-task's worktree code — a real tree, not the
    // retired tree-empty `--allow-empty` form (the honest rework, M31 Inc 5). Sequence
    // (oldest→newest): HEAD~2 = area-low, HEAD~1 = area-zed, HEAD = aggregate.
    let low_files = commit_changed_files(repo.path(), "HEAD~2");
    assert!(
        low_files.contains(&"src/low.rs".to_owned()),
        "the area-low per-sub-task commit must carry its worktree code (src/low.rs), not an \
         empty tree; got:\n{low_files:?}",
    );
    let zed_files = commit_changed_files(repo.path(), "HEAD~1");
    assert!(
        zed_files.contains(&"src/zed.rs".to_owned()),
        "the area-zed per-sub-task commit must carry its worktree code (src/zed.rs), not an \
         empty tree; got:\n{zed_files:?}",
    );
    // The merged docs land in the parent AGGREGATE (HEAD), never partitioned into a
    // per-sub-task commit (review B1/B2 — docs are a milestone-level merge artifact).
    let agg_files = commit_changed_files(repo.path(), "HEAD");
    assert!(
        agg_files.contains(&"docs/decisions/low-policy.md".to_owned())
            && agg_files.contains(&"docs/decisions/zed-policy.md".to_owned()),
        "the merged docs must land in the parent aggregate commit; got:\n{agg_files:?}",
    );
    assert!(
        !low_files.iter().any(|p| p.starts_with("docs/decisions/"))
            && !zed_files.iter().any(|p| p.starts_with("docs/decisions/")),
        "no merged doc may be partitioned into a per-sub-task code commit; \
         low={low_files:?} zed={zed_files:?}",
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
/// under `docs/decisions/` (the promoted ADRs the parent aggregate stages). The
/// `squash: false` per-sub-task commits stage only their worktree CODE (`src/*.rs`, never
/// `docs/decisions/`), so they pass; only the parent aggregate (which stages the merged
/// docs) trips the hook — a clean, controllable failure point AFTER the N sub-task commits
/// have landed.
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
fn milestone_finalize_squash_false_aggregate_failure_is_wip_safe_and_resets_to_pre_finalize_head() {
    // The `squash: false` boundary must be all-or-nothing AND WIP-safe (review S2; the M30/M31
    // data-loss class). The N per-sub-task commits + the parent aggregate are built away from
    // the live main checkout (a dedicated worktree), so on ANY abort — here a `pre-commit` hook
    // rejecting the staged ADRs — main is NEVER touched: HEAD stays at the pre-finalize sha (no
    // orphaned sub-task commits) AND a human's unrelated unstaged tracked edit SURVIVES
    // byte-for-byte. The old path `git reset --hard`ed the live checkout, silently reverting
    // that edit (the data loss this retires). RED before the fix (the WIP edit is destroyed),
    // GREEN after (`CLAUDE.md`: "Writes are transactional"; `finalize.md` rollback discipline;
    // `combine.rs::wip_survives_a_blocked_combine` is the squash:true sibling).
    let repo = TempDir::new("finalize-squash-false-reset");
    init_repo(repo.path());
    let home = TempDir::new("home");

    install_aggregate_rejecting_hook(repo.path());

    // Set up the milestone + two sub-tasks + staged docs.
    setup_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);

    // Seed an UNRELATED unstaged edit to a TRACKED file (`README.md`, committed by
    // `init_repo`) — the human WIP a `git reset --hard` of the live checkout would silently
    // destroy. Captured AFTER setup, so it is part of the pre-finalize baseline.
    const WIP: &str = "LOCAL WIP — do not lose me\n";
    fs::write(repo.path().join("README.md"), WIP).expect("seed unrelated tracked WIP");

    // Capture the pre-finalize HEAD + status JUST before the `finalize` call — the
    // "as-if-never-called" baseline (the staged `.jigc/` working area + the unrelated WIP
    // are part of this baseline, left intact on failure).
    let (before_head, before_status) = git_state(repo.path());
    let before_count = rev_list_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);

    // The aggregate commit was rejected, so the boundary fails (no parent aggregate).
    assert!(
        !finalized.status.success(),
        "the aggregate-rejecting hook must make `jigc milestone finalize` fail; got success\nstdout:\n{}",
        String::from_utf8_lossy(&finalized.stdout),
    );

    // The headline: the unrelated tracked WIP SURVIVED the abort byte-for-byte — the abort
    // never reset --hard the live checkout (the data-loss class is retired).
    assert_eq!(
        fs::read_to_string(repo.path().join("README.md")).expect("read README"),
        WIP,
        "on a squash:false abort, unrelated unstaged tracked WIP in the main checkout must SURVIVE \
         (the abort must never `git reset --hard` the live checkout)",
    );

    // HEAD is back at the pre-finalize sha — the N sub-task commits never landed on main,
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

    // The working tree matches the pre-finalize state — no dangling staging from the
    // sub-task commits and no leftover promoted ADR from the failed aggregate, and the
    // unrelated WIP still shows as a modification. The status matches the pre-finalize
    // baseline byte-for-byte.
    assert_eq!(
        after_status, before_status,
        "on aggregate failure the working tree must match the pre-finalize state (as-if-finalize-was-never-called)",
    );
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "on aggregate failure no promoted ADR may be left in `docs/decisions/`",
    );
}

#[test]
fn milestone_finalize_squash_false_blocks_a_cross_worktree_code_collision() {
    // The WF2 same-file block fires across the fan-out: two sub-tasks staging the SAME code
    // path in their isolated worktrees must block (the honest rework disjoint-applies each
    // worktree's patch in sequence and never text-merges a shared file), naming the
    // colliding path, committing nothing, HEAD unchanged. RED before the up-front
    // `detect_code_collision` block.
    let repo = TempDir::new("squash-false-collision");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // setup stages disjoint code (src/low.rs, src/zed.rs); now ALSO stage the SAME path in
    // BOTH worktrees → a cross-worktree collision on `src/shared.rs`.
    setup_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);
    stage_worktree_code(repo.path(), "area-low", "src/shared.rs", "fn low() {}\n");
    stage_worktree_code(repo.path(), "area-zed", "src/shared.rs", "fn zed() {}\n");

    let (before_head, _) = git_state(repo.path());
    let before_count = rev_list_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        !finalized.status.success(),
        "a cross-worktree code collision must make `jigc milestone finalize` fail; got success\nstdout:\n{}",
        String::from_utf8_lossy(&finalized.stdout),
    );
    let stderr = String::from_utf8_lossy(&finalized.stderr);
    assert!(
        stderr.contains("src/shared.rs"),
        "the collision block must name the contended path; got:\n{stderr}",
    );

    // Nothing committed — the block fires UP FRONT, before any per-sub-task commit.
    let (after_head, _) = git_state(repo.path());
    assert_eq!(
        after_head, before_head,
        "a collision block must commit nothing (HEAD unchanged)",
    );
    assert_eq!(
        rev_list_count(repo.path()),
        before_count,
        "a collision block must leave the commit count unchanged",
    );
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "a collision block must promote no ADR",
    );
}

/// The marker a non-blocking `pre-commit` hook writes to stderr before exiting 0 — git
/// redirects a hook's own stdout to stderr, so a warn-only hook (the M19 doc↔code
/// backstop's shape) speaks on the hook stream `git_commit` captures.
const HOOK_WARNING: &str = "NON-BLOCKING-MILESTONE-HOOK-WARNING";

/// Install a **non-blocking** `pre-commit` hook in `repo` that prints `HOOK_WARNING` and
/// exits 0 — git surfaces it on every commit's stderr (the hook stream). Under the honest
/// rework (M31 Inc 5) it fires on the N per-sub-task CODE commits AND the parent aggregate,
/// and **every** fan-out commit now relays its hook output, so a `squash: false` finalize
/// surfaces the warning N+1 times — one per code commit + the aggregate
/// (`design/finalize.md` → 6. Commit; never bypass hooks).
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

/// M31 Inc 5 — **every** fan-out commit runs the user's hooks and relays their output
/// (`design/finalize.md` → 6. Commit; never bypass hooks; `DECISIONS.md` 2026-06-21 —
/// squash:true hook restoration + squash:false honest rework). The `pre-commit` hook fires
/// on **every** commit, so a `squash: false` finalize runs it N+1 times (N per-sub-task
/// CODE commits + 1 aggregate), and each fan-out commit now relays its non-blocking hook
/// output — so the warning surfaces N+1 times (review B1/B2: each per-sub-task commit
/// carries real code, the merged docs ride the aggregate). The `squash: true` path commits
/// the off-line combine from a dedicated detached worktree via `git commit -F`, so the
/// user's hooks run against the combined tree; its single aggregate fires the non-blocking
/// hook EXACTLY once and relays its output.
#[test]
fn milestone_finalize_relays_every_fan_out_commit_hook_output() {
    // ── squash:false: 2 per-sub-task code commits + 1 aggregate; warning relayed N+1 = 3 ──
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
    // N+1 = 3 commits landed (2 per-sub-task code commits + 1 aggregate), so the hook fired
    // 3 times.
    assert_eq!(
        rev_list_count(repo.path()),
        before + 3,
        "squash:false must land N+1 commits, so the hook fires N+1 times",
    );
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout.matches(HOOK_WARNING).count(),
        3,
        "every fan-out commit (the 2 per-sub-task code commits + the aggregate) must relay \
         its hook output — N+1 = 3 times, not once (honest rework: each code commit runs \
         the user's hooks); got:\n{stdout}",
    );
    assert!(
        stdout.contains("--- hook output ---"),
        "the relay must land in the delimited section; got:\n{stdout}",
    );

    // ── squash:true: the M31 Inc 5 WIP-safe combine commit runs the user's pre-commit/
    // commit-msg hooks against the combined tree (from a dedicated detached worktree, then
    // fast-forwards main) — so the single aggregate FIRES the non-blocking hook EXACTLY once
    // and relays its output in the delimited section (`design/finalize.md` → never bypass
    // hooks; `DECISIONS.md` 2026-06-21 — squash:true hook restoration). RED before (Inc 4's
    // `commit-tree` ran none). ───────────────────────────────────────────────────────────
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
        "the squash:true combine must land its commit; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );
    // squash:true lands EXACTLY one commit (the synthesized aggregate, off-line combined).
    assert_eq!(
        rev_list_count(repo.path()),
        before + 1,
        "squash:true must land exactly one aggregate commit",
    );
    // The Inc 5 dedicated-worktree commit runs the user's pre-commit hook against the
    // combined tree, so the non-blocking warning is captured + relayed EXACTLY once.
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    assert_eq!(
        stdout.matches(HOOK_WARNING).count(),
        1,
        "the squash:true combine must run the user's pre-commit hook against the combined \
         tree and relay its warning exactly once (Inc 5 hook restoration); got:\n{stdout}",
    );
    assert!(
        stdout.contains("--- hook output ---"),
        "the squash:true aggregate hook output must land in the delimited section; got:\n{stdout}",
    );
}

/// The `hook_output` **producer axis**, fan-out member (confidence-audit sibling-hunt
/// item 4; the axis home is `tests/hook_output_axis.rs`): under `--format json` a
/// `squash: false` finalize lands N+1 hook-capable commits (N per-sub-task code
/// commits plus the aggregate), and **every** commit's captured non-blocking hook
/// stream must reach the one landed envelope — `committed.hook_output` folds all N+1
/// streams, and the stderr relay carries the *same* folded string (one capture, two
/// channels; `design/command-output-contract.md` → Stream discipline). RED before the
/// fix: the envelope carried the aggregate's stream only, while the per-sub-task
/// streams were relayed to stderr and reached no envelope — a JSON driver reading the
/// document never saw them.
#[test]
fn milestone_finalize_json_envelope_folds_every_fan_out_commits_hook_output() {
    let repo = TempDir::new("relay-json-squash-false");
    init_repo(repo.path());
    let home = TempDir::new("home");
    install_warning_hook(repo.path());

    setup_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);
    let finalized = run_milestone(
        repo.path(),
        home.path(),
        &["finalize", "cache-rework", "--format", "json"],
    );
    let stdout = String::from_utf8(finalized.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");
    assert!(
        finalized.status.success(),
        "a non-blocking hook must not block the json squash:false finalize; got {:?}\nstderr:\n{stderr}",
        finalized.status,
    );
    // Stream discipline: stdout is exactly one JSON document.
    let value: serde_json::Value = serde_json::from_str(stdout.trim()).unwrap_or_else(|err| {
        panic!("stdout must parse as exactly one JSON document ({err}); got:\n{stdout}")
    });
    let hook_output = value["committed"]["hook_output"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("committed.hook_output must be a present string; got:\n{stdout}")
        });
    // All N+1 = 3 commit streams (2 per-sub-task code commits + the aggregate) fold
    // into the ONE envelope key — not just the aggregate's.
    assert_eq!(
        hook_output.matches(HOOK_WARNING).count(),
        3,
        "committed.hook_output must fold every fan-out commit's hook stream (N+1 = 3), \
         not the aggregate's alone; got:\n{stdout}",
    );
    // One capture, two channels: the stderr relay carries the same folded string.
    assert!(
        stderr.contains(&format!("--- hook output ---\n{hook_output}")),
        "the stderr relay must carry the same folded string as committed.hook_output; \
         stderr:\n{stderr}",
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
    // The `cd` operand is the ABSOLUTE worktree path since M53 (the cwd census, C1-14 /
    // C3-01): the line is the one `cd` jigc emits, an orchestrator pastes it into a shell of
    // unknown cwd, and spelled repo-relative it ran from the repository root and nowhere
    // else. The id-ordering this asserts is untouched by the root it is rooted at.
    let root = std::fs::canonicalize(repo.path())
        .unwrap_or_else(|_| repo.path().to_path_buf())
        .display()
        .to_string();
    let spawns = spawn_lines(&stdout);
    assert_eq!(
        spawns,
        vec![
            format!(
                "cd {root}/.jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix"
            ),
            format!(
                "cd {root}/.jigc/worktrees/zebra-fix && jigc workflow sub-task --task zebra-fix"
            ),
        ],
        "the real `milestone-execution` workflow must emit one id-sorted `Spawn:` directive \
         per sub-task, each `cd`-ing into its own worktree (alpha before zebra, not add order); \
         got:\n{stdout}",
    );

    // C1 (round-2 surface fixes) — the compose teaches the FULL order: provision →
    // spawn → wait-for-completions → join (merge + report, commits nothing) →
    // finalize (the one commit). The join step is an INSTRUCTION with its own `Run:`
    // line, never static text posing as a state report (the corpus's strongest
    // misreading: an agent believed the fan-out had already run and went straight to
    // finalize).
    // Absolute `cd` operand since M53 (the cwd census, C1-14 / C3-01) — the marker is the
    // directive prefix, and what this probe is about is the **order**.
    let last_spawn_at = stdout
        .rfind("Spawn: `cd /")
        .expect("at least one Spawn directive");
    let join_prose_at = stdout
        .find("Once every spawned sub-task reports complete")
        .expect("the join-tasks step must be conditional on the sub-tasks completing");
    // Each `Run:` line carries the **bound** milestone's id, not an agent-fill marker:
    // this door was reached with the work-unit bound, so the token is the CLI's to
    // fill (M49 Increment 10 / T4). The marker's home is the off-verb compose, pinned
    // by `flow10_acceptance.rs`.
    let join_run_at = stdout
        .find("Run: `jigc milestone join cache-rework`")
        .expect("the join-tasks step must resolve a `Run:` line — the join is taught");
    let run_at = stdout
        .find("Run: `jigc milestone finalize cache-rework`")
        .expect("the milestone-finalize step must resolve a `Run:` line");
    assert!(
        last_spawn_at < join_prose_at && join_prose_at < join_run_at && join_run_at < run_at,
        "the view must be fan-out → wait → join Run → finalize Run, in that order; got:\n{stdout}",
    );
    // State-honesty: no compose-time text may assert the fan-out already ran.
    assert!(
        !stdout.contains("All sub-tasks are complete"),
        "the compose must not print a state report as static text; got:\n{stdout}",
    );

    // Composition is fan-out-join-paired — a paired workflow raises no
    // `workflow-refs.fan-out-join-paired` block on stdout/stderr.
    assert!(
        !stdout.contains("fan-out-join-paired"),
        "a paired fan-out/join workflow must not raise the pairing block; got:\n{stdout}",
    );
}

/// T2 (M31 Inc 3): the real `milestone-execution` workflow composes the worktree-
/// provisioning `Run:` step **before** the fan-out's first `Spawn:` directive, and the
/// compose raises no `command-ref-resolves` block — proving the `milestone-provision`
/// catalog entry resolves the provision step's `{{cli.…}}` ref. The assertion reads the
/// agent-facing composed bytes the agent runs, never a hand-built equivalent.
#[test]
fn milestone_execute_emits_the_provision_run_before_the_first_spawn() {
    let repo = TempDir::new("execute-provision");
    init_repo(repo.path());
    let home = TempDir::new("home");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config layer");

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
        "`jigc milestone execute cache-rework` must exit 0 (the `milestone-provision` catalog \
         entry resolves the provision step's command-ref); got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    // `workflow-refs.command-ref-resolves` passes — no blocking finding leaked to the view.
    assert!(
        !stdout.contains("command-ref-resolves"),
        "the provision step's `{{cli.milestone-provision}}` must resolve against the catalog; \
         got:\n{stdout}",
    );

    // The bound milestone's id, resolved — see the sibling above (M49 Inc 10 / T4).
    let provision_at = stdout
        .find("Run: `jigc milestone provision cache-rework`")
        .expect("the provision step must resolve a `Run:` line into the composed view");
    // The `cd` operand is absolute since M53 (the cwd census, C1-14 / C3-01), so the marker
    // is the directive prefix; what this probe is about is the **order**.
    let first_spawn_at = stdout
        .find("Spawn: `cd /")
        .expect("the fan-out emits at least one `Spawn:` directive");
    assert!(
        provision_at < first_spawn_at,
        "the worktree-provisioning `Run:` step must compose BEFORE the fan-out's first \
         `Spawn:` directive; got:\n{stdout}",
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

    // Each arm is its own throwaway repository, and the `cd` operand is absolute since M53
    // (the cwd census, C1-14 / C3-01) — so each arm's own root is normalized to a
    // placeholder before comparison. The claim is the **order** and its invariance across
    // add orders, which no path can change.
    let normalize = |stdout: &str, repo: &Path| -> Vec<String> {
        let root = std::fs::canonicalize(repo)
            .unwrap_or_else(|_| repo.to_path_buf())
            .display()
            .to_string();
        spawn_lines(stdout)
            .into_iter()
            .map(|line| line.replace(&root, "<ROOT>"))
            .collect()
    };
    let fwd_spawns = normalize(&fwd_stdout, forward.path());
    let rev_spawns = normalize(&rev_stdout, reverse.path());
    assert_eq!(
        fwd_spawns,
        vec![
            "cd <ROOT>/.jigc/worktrees/alpha-fix && jigc workflow sub-task --task alpha-fix"
                .to_owned(),
            "cd <ROOT>/.jigc/worktrees/mid-fix && jigc workflow sub-task --task mid-fix".to_owned(),
            "cd <ROOT>/.jigc/worktrees/zebra-fix && jigc workflow sub-task --task zebra-fix"
                .to_owned(),
        ],
        "the forward add order must emit id-sorted Spawn directives",
    );
    assert_eq!(
        fwd_spawns, rev_spawns,
        "the `Spawn:` block must be byte-identical across divergent add orders (id-sorted on \
         resolve); forward:\n{fwd_stdout}\nreverse:\n{rev_stdout}",
    );
}

/// M45 Inc 10 (T4): the milestone-execution compose carries the three orchestrator
/// stated-at notices the project-alpha-3.0 trial found missing at the fan-out surface —
/// (a) the two-channel docs-overlay/code-index fold statement (milestone-finalize),
/// (b) the worktrees-lack-runtime-deps note, and (c) the bookkeeping-commits-land-on-
/// the-working-branch notice (both on provision-worktrees). Asserted on the emitted
/// composed bytes the agent reads, never a hand-built equivalent.
#[test]
fn milestone_execute_states_the_two_channel_runtime_dep_and_bookkeeping_notices() {
    let repo = TempDir::new("execute-notices");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let stdout = execute_with_add_order(
        repo.path(),
        home.path(),
        "Cache rework",
        &["Alpha fix", "Zebra fix"],
    );

    // (a) The finalize fold is two-channel — docs merged as an overlay, code folded
    // from each worktree's staged index (stated only in the sub-task's own commit
    // step before this fix; now on the orchestrator surface too).
    assert!(
        stdout.contains("two channels")
            && stdout.contains("overlay")
            && stdout.contains("staged index"),
        "the milestone-finalize step must state the two-channel docs-overlay/code-index \
         fold; got:\n{stdout}",
    );

    // (b) The detached worktrees carry only tracked files — no installed deps.
    assert!(
        stdout.contains("git-tracked")
            && stdout.contains("node_modules")
            && stdout.contains("worktree first"),
        "the provision step must note the worktrees lack runtime dependencies; got:\n{stdout}",
    );

    // (c) Milestone bookkeeping lands record-only commits on the working branch.
    assert!(
        stdout.contains("record-only commits") && stdout.contains("working branch"),
        "the provision step must note bookkeeping commits land on the working branch; \
         got:\n{stdout}",
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

    // Inject a non-registered leftover dir at one worktree path. Until M48 Inc 1 this
    // asserted the leftover was CLEARED and the run exited 0 — which pinned data loss as
    // expected output: `junk.txt` passes only because the test author knows it is junk, and
    // the binary cannot tell it from `precious.txt` (`DECISIONS.md` 2026-08-13 → the Settle,
    // F3). A leftover holding anything now REFUSES; the guard's own axis (every verdict ×
    // `--force`) lives in `provision_leftover_guard.rs`, so this arm pins only the fact that
    // the shipped idempotency path no longer walks over content.
    let stale = wt_root.join("area-low");
    fs::create_dir_all(&stale).expect("mk stale leftover dir");
    fs::write(stale.join("junk.txt"), "leftover\n").expect("write junk");

    let refused = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        !refused.status.success(),
        "a leftover holding content must REFUSE the provision; got {:?}\nstdout:\n{}",
        refused.status,
        String::from_utf8_lossy(&refused.stdout),
    );
    assert!(
        fs::read_to_string(stale.join("junk.txt")).expect("the leftover survives") == "leftover\n",
        "the refused provision must leave the leftover's bytes untouched",
    );

    // Emptied — an EMPTY leftover is provably safe, so the crashed-run idempotency the rest
    // of this test asserts still holds.
    fs::remove_file(stale.join("junk.txt")).expect("empty the leftover");

    // Provision #1 — over the (now empty) stale leftover.
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

/// The `git worktree list --porcelain` listing of `repo` — the registered worktrees
/// (one `worktree <path>` line each). The teardown assertions read it to prove the
/// fan-out worktrees were unregistered (removed), not merely deleted on disk.
fn worktree_list(repo: &Path) -> String {
    let out = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(repo)
        .output()
        .expect("git worktree list");
    String::from_utf8(out.stdout).unwrap()
}

/// Mint `Cache rework` + two sub-tasks under `repo` (added NON-id-order), stage in each
/// sub-area a clean disjoint persisted ADR so the finalize has a real tree diff and
/// lands, then `provision` the N=2 base-pin worktrees. Returns the `.jigc/worktrees/`
/// root. Main is NOT advanced, so the milestone base pin == HEAD (the finalize preflight
/// requires `base == HEAD`). The teardown tests share this provisioned-and-ready setup.
fn provision_for_finalize(repo: &Path, home: &Path) -> PathBuf {
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

    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "`jigc milestone provision cache-rework` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    let wt_root = repo.join(".jigc").join("worktrees");
    assert_eq!(
        dir_child_count(&wt_root),
        2,
        "the setup must provision exactly N=2 worktrees before the finalize",
    );
    assert!(
        worktree_list(repo).contains("worktrees/area-low")
            && worktree_list(repo).contains("worktrees/area-zed"),
        "both fan-out worktrees must be registered before the finalize",
    );
    wt_root
}

#[test]
fn milestone_finalize_tears_down_the_provisioned_worktrees_on_a_landed_commit() {
    // (a) A landed milestone finalize must tear down the N provisioned fan-out worktrees
    // (`git worktree remove --force`, each by path), not just the milestone/sub-task areas —
    // a leaked worktree is a registered git object, heavier than gitignored scratch. RED
    // before the teardown wiring (the worktrees persist), GREEN after.
    let repo = TempDir::new("teardown-ok");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let wt_root = provision_for_finalize(repo.path(), home.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize cache-rework` must exit 0; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );

    // Every fan-out worktree dir is gone AND unregistered (removed, not orphaned).
    assert_eq!(
        dir_child_count(&wt_root),
        0,
        "a landed finalize must remove every provisioned worktree dir",
    );
    let list = worktree_list(repo.path());
    assert!(
        !list.contains("worktrees/area-low") && !list.contains("worktrees/area-zed"),
        "a landed finalize must UNREGISTER the fan-out worktrees (not just delete dirs); got:\n{list}",
    );

    // The main checkout `git status` is clean after the commit + teardown.
    let (_, status) = git_state(repo.path());
    assert!(
        status.trim().is_empty(),
        "the main checkout must be clean after the finalize + worktree teardown; got:\n{status}",
    );
}

#[test]
fn milestone_finalize_keeps_the_provisioned_worktrees_on_an_aborted_finalize() {
    // (b) An aborted milestone finalize (a `pre-commit` hook rejects the aggregate, driving
    // the `squash: false` rollback) must leave the provisioned worktrees ALIVE — registered,
    // with their staged code intact — alongside the sub-task areas it already left intact for
    // retry. HEAD stays at the pre-finalize sha and nothing is torn down.
    //
    // **This assertion is the INVERSE of the one this test shipped with, and the reversal is a
    // basis-has-changed rebuttal, not an override** (M47 Inc 3 T1). The original demanded the
    // teardown on the strength of `DECISIONS.md` → 2026-06-21 M31 Inc 3 T4, which wired it
    // into the abort path when that path still `reset --hard`ed the LIVE checkout — the
    // worktrees were then redundant scratch. M31 Inc 4/5 then made the provisioned worktree
    // the **sole copy** of a sub-agent's staged code, falsifying that basis: `git worktree
    // remove --force` on the abort arm destroys work no commit ever captured, against
    // `design/finalize.md` → 6. Commit (*"working area intact"*). A landed finalize still tears
    // down (the two success arms), and an abandoned milestone is torn down by `jigc milestone
    // discard`, which refuses on a dirty worktree unless `--force`.
    //
    // The rejection-cause AXIS this instance sits on — the aggregate hook, a per-sub-task
    // hook, and a hookless `git merge --ff-only` refusal, each with its recovery re-run — is
    // `crates/cli/tests/milestone_abort_survives.rs`.
    let repo = TempDir::new("teardown-abort");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // The aggregate-rejecting hook + `squash: false` mode is the controllable abort point
    // (the `milestone_finalize_squash_false_aggregate_failure_resets_to_pre_finalize_head`
    // pattern); set `squash: false` + author each sub-task's commit doc.
    install_aggregate_rejecting_hook(repo.path());
    setup_squash_false(repo.path(), home.path(), &["Area zed", "Area low"]);

    let provisioned = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "`jigc milestone provision cache-rework` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    let wt_root = repo.path().join(".jigc").join("worktrees");
    assert_eq!(
        dir_child_count(&wt_root),
        2,
        "the setup must provision exactly N=2 worktrees before the aborted finalize",
    );

    let (before_head, _) = git_state(repo.path());
    let before_count = rev_list_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        !finalized.status.success(),
        "the aggregate-rejecting hook must make the finalize fail; got success\nstdout:\n{}",
        String::from_utf8_lossy(&finalized.stdout),
    );

    // HEAD rolled back to the pre-finalize sha (the abort reset).
    let (after_head, _) = git_state(repo.path());
    assert_eq!(
        after_head, before_head,
        "an aborted finalize must reset HEAD to the pre-finalize sha",
    );
    assert_eq!(
        rev_list_count(repo.path()),
        before_count,
        "an aborted finalize must leave the commit count unchanged",
    );

    // The worktrees survive the abort — the sub-agents' code is not destroyed.
    assert_eq!(
        dir_child_count(&wt_root),
        2,
        "an aborted finalize must leave every provisioned worktree dir in place",
    );
    let list = worktree_list(repo.path());
    assert!(
        list.contains("worktrees/area-low") && list.contains("worktrees/area-zed"),
        "an aborted finalize must leave the fan-out worktrees REGISTERED; got:\n{list}",
    );
}

/// Write + `git add` a code file IN a provisioned fan-out worktree
/// (`.jigc/worktrees/<sub>/<rel>`) — the staged code a fanned-out sub-agent produces in
/// its isolated worktree, the code-set the squash:true combine must fold into the single
/// commit (M31 Inc 4). Stages in the worktree's OWN index (`git add` with `current_dir`
/// the worktree), never the main checkout.
fn stage_worktree_code(repo: &Path, sub: &str, rel: &str, body: &str) {
    let wt = repo.join(".jigc").join("worktrees").join(sub);
    let p = wt.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).expect("mkdir worktree code parent");
    }
    fs::write(&p, body).expect("write worktree code");
    let out = Command::new("git")
        .args(["add", rel])
        .current_dir(&wt)
        .output()
        .expect("git add in worktree");
    assert!(
        out.status.success(),
        "staging `{rel}` in worktree `{sub}` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

#[test]
fn milestone_finalize_squash_true_combines_disjoint_worktree_code() {
    // The combine keystone (M31 Inc 4): a squash:true fan-out with two sub-agents touching
    // DISJOINT files commits BOTH sub-agents' staged code (drops none) + the merged docs in
    // ONE commit. RED before the wiring — the `Sweep` policy `git add --all`s the main
    // checkout, where the worktree-isolated code never lives, so it commits ZERO worktree
    // code (the data-loss repro). GREEN once the Combine channel folds the worktrees in.
    let repo = TempDir::new("combine-disjoint");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Mint + 2 sub-tasks + 2 base-pin worktrees (provision_for_finalize also stages a
    // disjoint persisted ADR per sub-area — the merged docs the same commit must carry).
    provision_for_finalize(repo.path(), home.path());

    // Each sub-agent stages DISJOINT code in its own isolated worktree.
    stage_worktree_code(repo.path(), "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo.path(), "area-zed", "src/zed.rs", "pub fn zed() {}\n");

    let before = rev_list_count(repo.path());
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize` (squash:true) must combine the worktree code + docs and \
         exit 0; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );

    // C2 (round-2 surface fixes) — the landing manifest names the combined worktree
    // code alongside the promoted docs, and the contribution line counts each
    // sub-task's staged code files.
    let stdout = String::from_utf8_lossy(&finalized.stdout).to_string();
    assert!(
        stdout.contains("  added src/low.rs") && stdout.contains("  added src/zed.rs"),
        "the landing manifest must name both worktrees' combined code; got:\n{stdout}",
    );
    assert!(
        stdout.contains("sub-tasks: area-low: 1 doc, 1 code file · area-zed: 1 doc, 1 code file"),
        "the contribution line must count each sub-task's docs + code files; got:\n{stdout}",
    );

    // EXACTLY ONE new commit.
    assert_eq!(
        rev_list_count(repo.path()),
        before + 1,
        "the combine must land exactly one commit",
    );

    // The single commit's tree carries BOTH worktrees' code AND the merged docs.
    let tree = head_tree_paths(repo.path());
    assert!(
        tree.iter().any(|p| p == "src/low.rs"),
        "worktree `area-low`'s staged code must land in the commit (dropped none); got:\n{tree:?}",
    );
    assert!(
        tree.iter().any(|p| p == "src/zed.rs"),
        "worktree `area-zed`'s staged code must land in the commit (dropped none); got:\n{tree:?}",
    );
    assert!(
        tree.iter().any(|p| p == "docs/decisions/low-policy.md")
            && tree.iter().any(|p| p == "docs/decisions/zed-policy.md"),
        "the merged docs must land in the SAME commit; got:\n{tree:?}",
    );

    // The main checkout is clean after the combine (no ` D` drift; worktrees torn down).
    let (_, status) = git_state(repo.path());
    assert!(
        status.trim().is_empty(),
        "the main checkout must be clean after the combine; got:\n{status}",
    );
}

#[test]
fn milestone_finalize_squash_true_code_only_fan_out_does_not_false_block_as_empty() {
    // A fan-out whose sub-agents touched ONLY code (no persisted docs) must finalize: the
    // narrowed CLI-side has_diff sees the worktree-staged code (Σ `git diff --cached`), so
    // the empty-commit guard does not false-block it. RED before the narrowing — has_diff
    // scanned only the main checkout (empty under worktree isolation) and the milestone had
    // no materialized docs, so the guard aborted it as empty.
    let repo = TempDir::new("combine-code-only");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Mint + 2 sub-tasks, provision worktrees, stage NO docs — code only.
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
    let provisioned = run_milestone(repo.path(), home.path(), &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    stage_worktree_code(repo.path(), "area-low", "src/low.rs", "pub fn low() {}\n");
    stage_worktree_code(repo.path(), "area-zed", "src/zed.rs", "pub fn zed() {}\n");

    let before = rev_list_count(repo.path());
    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "a code-only fan-out must NOT false-block as empty; got {:?}\nstderr:\n{}",
        finalized.status,
        String::from_utf8_lossy(&finalized.stderr),
    );
    assert_eq!(
        rev_list_count(repo.path()),
        before + 1,
        "the code-only combine must land exactly one commit",
    );
    let tree = head_tree_paths(repo.path());
    assert!(
        tree.iter().any(|p| p == "src/low.rs") && tree.iter().any(|p| p == "src/zed.rs"),
        "both worktrees' code must land in the code-only combine; got:\n{tree:?}",
    );
}

#[test]
fn milestone_finalize_warns_on_a_leaked_worktree_but_still_succeeds() {
    // (c) A forced teardown failure (a `git worktree lock` makes single-`--force` removal
    // refuse the leaf) surfaces a NON-BLOCKING warning naming the leaked worktree path +
    // the remedy for that one registration (review A2 — pinned, not silent-log/block), and
    // the landed commit's exit code is UNCHANGED. The unlocked sibling is still torn down.
    let repo = TempDir::new("teardown-leak");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let wt_root = provision_for_finalize(repo.path(), home.path());

    // Lock `area-low` so `git worktree remove --force` refuses it (needs `--force --force`),
    // forcing the teardown of that one worktree to fail.
    let locked = wt_root.join("area-low");
    let lock = Command::new("git")
        .args(["worktree", "lock", locked.to_str().unwrap()])
        .current_dir(repo.path())
        .output()
        .expect("git worktree lock");
    assert!(
        lock.status.success(),
        "locking the worktree must succeed; stderr:\n{}",
        String::from_utf8_lossy(&lock.stderr),
    );

    let before_count = rev_list_count(repo.path());

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    let stderr = String::from_utf8(finalized.stderr).expect("utf-8 stderr");

    // The commit still landed — a teardown failure never blocks a landed commit.
    assert!(
        finalized.status.success(),
        "a leaked-worktree teardown failure must NOT change the finalize exit code; got {:?}\nstderr:\n{stderr}",
        finalized.status,
    );
    assert_eq!(
        rev_list_count(repo.path()),
        before_count + 1,
        "the finalize must still land its one commit despite the teardown warning",
    );

    // The non-blocking warning names the leaked worktree path + its remedy.
    assert!(
        stderr.contains("area-low"),
        "the teardown warning must name the leaked worktree path; got:\n{stderr}",
    );
    // The remedy is keyed to the ONE registration, and to why git refused it (the rc.24
    // blind trial, L-22): for a locked worktree, `unlock` then `remove --force`, both by
    // path. It read *run `git worktree prune`, then …* until then — a repository-wide act
    // that drops every other stale registration in the repository and, for this cause,
    // repairs nothing (a prune skips a locked worktree). Both spans are aimed since M53
    // (the cwd census, C1-08): `remove` resolves its operand against the caller's cwd.
    // Run verbatim in `worktree_registration_reach.rs`.
    assert!(
        stderr.contains(" worktree unlock ")
            && stderr.contains(" worktree remove --force ")
            && stderr.contains("git -C /"),
        "the teardown warning must name the locked worktree's remedy — unlock, then remove \
         that one registration — aimed at the checkout it runs in; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("worktree prune"),
        "no remedy may send the reader to a repository-wide prune; got:\n{stderr}",
    );

    // The unlocked sibling was still torn down (best-effort proceeds past the failure).
    assert!(
        !wt_root.join("area-zed").exists(),
        "the unlocked worktree must still be torn down despite the locked sibling failing",
    );
    let list = worktree_list(repo.path());
    assert!(
        !list.contains("worktrees/area-zed"),
        "the unlocked worktree must be unregistered; got:\n{list}",
    );
}

/// Run `jigc <args>` with an explicit `cwd` (a fan-out worktree) and `$HOME = home`,
/// optionally piping `stdin` — the genuine sub-agent re-entry + author path runs from
/// **inside** the sub-task's worktree, so the cwd is `.jigc/worktrees/<sub>`, not the
/// main checkout. (`jigc_home_or_repo` resolves the shared `.jigc/` against the main
/// checkout regardless, so the writes still land in `jigc_home/.jigc/tasks/<sub>/`.)
fn run_jigc_in(
    cwd: &Path,
    home: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(cwd)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// HEAD~n's full commit message at `rev` (`git log -1 --format=%B <rev>`) — the
/// per-sub-task body assertion reads it off the landed sub-task commit.
fn message_at(repo: &Path, rev: &str) -> String {
    let out = Command::new("git")
        .args(["log", "-1", "--format=%B", rev])
        .current_dir(repo)
        .output()
        .expect("git log rev");
    String::from_utf8(out.stdout).unwrap()
}

/// The `commit:<sub>` doc the genuine re-entry provisions, bound to **jigc_home** (the
/// main checkout): `.jigc/tasks/<sub>/docs/commit:<sub>.md` — the surface the join reads.
fn subtask_commit_doc(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs")
        .join(format!("commit:{sub}.md"))
}

/// The genuine sub-agent re-entry + author spine over the **real `sub-task` workflow** and
/// the M31 worktrees — the acceptance the masked `single-task` / hand-staged
/// `stage_subtask_commit` fixtures never drove. Mints `Cache rework` + two default
/// (`sub-task`) sub-tasks in `add_order`, provisions the N base-pin worktrees, `execute`s
/// the milestone, then **for each sub-task, from inside its worktree**: genuinely re-enters
/// via `jigc workflow sub-task --task <sub>` (which provisions the `commit:<sub>` doc on
/// first entry — the property the gate fix decouples from `selectable`), asserts the doc
/// landed (case d), authors its `type`/`summary`/`body`, then stages a disjoint persisted
/// ADR (the merged-doc tree the parent aggregate lands) + disjoint worktree code (the
/// per-sub-task commit's tree). The caller finalizes + asserts the squash-mode outcome.
fn drive_genuine_reentry(repo: &Path, home: &Path, add_order: &[&str]) {
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
            "add-task `{intent}` (default sub-task) must exit 0",
        );
    }
    let provisioned = run_milestone(repo, home, &["provision", "cache-rework"]);
    assert!(
        provisioned.status.success(),
        "provision must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    let executed = run_milestone(repo, home, &["execute", "cache-rework"]);
    assert!(
        executed.status.success(),
        "execute must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&executed.stderr),
    );

    // (sub, summary, body, code-path, code-body) — id-sorted [area-low, area-zed], each
    // with distinct authored prose + disjoint code so the per-sub-task render + tree are
    // genuinely the sub-agent's own.
    let subs = [
        (
            "area-low",
            "rework the low cache path",
            "Reworks the low cache path body.",
            "src/low.rs",
            "pub fn low() {}\n",
        ),
        (
            "area-zed",
            "rework the zed cache path",
            "Reworks the zed cache path body.",
            "src/zed.rs",
            "pub fn zed() {}\n",
        ),
    ];
    for (sub, summary, body, code_rel, code_body) in subs {
        let wt = repo.join(".jigc").join("worktrees").join(sub);
        assert!(wt.is_dir(), "the worktree for `{sub}` must be provisioned");

        // Genuine re-entry from INSIDE the worktree — provisions `commit:<sub>` on first
        // entry (the gate fix; RED before it: the `sub-task` (`selectable: false`) branch
        // returns early, provisioning nothing).
        let reentry = run_jigc_in(&wt, home, &["workflow", "sub-task", "--task", sub], None);
        assert!(
            reentry.status.success(),
            "genuine re-entry `jigc workflow sub-task --task {sub}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&reentry.stderr),
        );
        // (case d) the provisioning assertion the masked fixtures never made: the commit
        // doc landed in jigc_home over the REAL `sub-task` workflow.
        assert!(
            subtask_commit_doc(repo, sub).is_file(),
            "re-entry over the real `sub-task` workflow must provision \
             jigc_home/.jigc/tasks/{sub}/docs/commit:{sub}.md",
        );

        // Author the commit prose through the CLI — `set-field` here is exactly the call
        // that exits 1 ("no staged instance — provision it first") on current code.
        let set_field = run_jigc_in(
            &wt,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{sub}#type"),
                "--value",
                "feat",
                "--task",
                sub,
            ],
            None,
        );
        assert!(
            set_field.status.success(),
            "set-field commit:{sub}#type must exit 0 after the genuine re-entry provisions \
             it; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&set_field.stdout),
            String::from_utf8_lossy(&set_field.stderr),
        );
        let set_summary = run_jigc_in(
            &wt,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{sub}#summary"),
                "--from-file",
                "-",
                "--task",
                sub,
            ],
            Some(summary.as_bytes()),
        );
        assert!(
            set_summary.status.success(),
            "set-slot commit:{sub}#summary must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&set_summary.stderr),
        );
        let set_body = run_jigc_in(
            &wt,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{sub}#body"),
                "--from-file",
                "-",
                "--task",
                sub,
            ],
            Some(body.as_bytes()),
        );
        assert!(
            set_body.status.success(),
            "set-slot commit:{sub}#body must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&set_body.stderr),
        );

        // A disjoint persisted ADR (the merged-doc tree the parent aggregate lands) +
        // disjoint worktree code (the per-sub-task commit's tree). Staged AFTER the
        // re-entry so the commit doc's recorded provenance survives.
        stage_doc(
            repo,
            sub,
            &format!("adr:{sub}-policy"),
            &adr_plain(&format!("{sub} policy")),
            "edited-from-base",
        );
        stage_worktree_code(repo, sub, code_rel, code_body);
    }
}

#[test]
fn milestone_finalize_squash_false_genuine_reentry_authors_per_subtask_commits() {
    // The genuine acceptance (review-A1, worktree-active): a real fan-out sub-agent
    // provisions AND authors its `commit:<sub>` doc through a genuine
    // `jigc workflow sub-task --task <sub>` re-entry, and that authored commit flows
    // through join + finalize in squash:false — the rendered per-sub-task message carries
    // the AUTHORED summary/body (not merely exit 0). RED on current code: the re-entry
    // provisions nothing (`provision_on_first_entry`'s `!def.selectable` early-returns for
    // `sub-task`), so the provisioning assertion + the subsequent `set-field` both fail.
    let repo = TempDir::new("reentry-squash-false");
    init_repo(repo.path());
    let home = TempDir::new("home");
    set_squash_false(repo.path());

    let before = rev_list_count(repo.path());
    // Non-id add order (zed before low) so the id-sorted commit sequence is not an accident
    // of insertion order.
    drive_genuine_reentry(repo.path(), home.path(), &["Area zed", "Area low"]);

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize` (squash:false) must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&finalized.stderr),
    );

    // N+1 = 3 new commits: one per sub-task (2) + the parent aggregate.
    assert_eq!(
        rev_list_count(repo.path()),
        before + 3,
        "squash:false must land N+1 commits (2 sub-task + 1 parent)",
    );

    // The committed SEQUENCE (oldest first): the two AUTHORED sub-task subjects id-sorted
    // (area-low before area-zed, NOT add order), then the parent's synthesized aggregate.
    let subjects = recent_subjects(repo.path(), 3);
    assert_eq!(
        subjects,
        vec![
            "feat: rework the low cache path".to_owned(),
            "feat: rework the zed cache path".to_owned(),
            "Finalize milestone cache-rework (2 sub-tasks)".to_owned(),
        ],
        "the commit sequence must be the id-sorted AUTHORED sub-task messages then the parent",
    );

    // Each per-sub-task commit carries THAT sub-task's authored BODY — proving the genuine
    // re-entry's authored prose (not merely an exit code) flows through the render. HEAD~2
    // == area-low, HEAD~1 == area-zed (oldest first), HEAD == the aggregate.
    let low_msg = message_at(repo.path(), "HEAD~2");
    assert!(
        low_msg.contains("Reworks the low cache path body."),
        "the area-low commit must carry its authored body; got:\n{low_msg}",
    );
    let zed_msg = message_at(repo.path(), "HEAD~1");
    assert!(
        zed_msg.contains("Reworks the zed cache path body."),
        "the area-zed commit must carry its authored body; got:\n{zed_msg}",
    );

    // The parent aggregate landed the merged persisted ADRs.
    let tree = head_tree_paths(repo.path());
    assert!(
        tree.contains(&"docs/decisions/area-low-policy.md".to_owned())
            && tree.contains(&"docs/decisions/area-zed-policy.md".to_owned()),
        "the parent aggregate must commit the merged persisted docs; got:\n{tree:?}",
    );
}

#[test]
fn milestone_finalize_squash_true_genuine_reentry_materializes_transient_then_lands_aggregate() {
    // The squash:true face of the same genuine re-entry: the authored `commit:<sub>` docs
    // are TRANSIENT — materialized in staging, then skipped at promote (no `location:`) —
    // and the single CLI-synthesized aggregate still lands the merged docs + worktree code.
    // No `.jigc/config/` knob → the pack-default squash:true.
    let repo = TempDir::new("reentry-squash-true");
    init_repo(repo.path());
    let home = TempDir::new("home");
    // The project cascade layer the `execute` / re-entry surfaces require; no squash knob
    // → the pack-default squash:true.
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config layer");

    let before = rev_list_count(repo.path());
    drive_genuine_reentry(repo.path(), home.path(), &["Area zed", "Area low"]);

    let finalized = run_milestone(repo.path(), home.path(), &["finalize", "cache-rework"]);
    assert!(
        finalized.status.success(),
        "`jigc milestone finalize` (squash:true) must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&finalized.stderr),
    );

    // EXACTLY ONE new commit (the synthesized aggregate).
    assert_eq!(
        rev_list_count(repo.path()),
        before + 1,
        "squash:true must land exactly one aggregate commit",
    );
    let message = head_message(repo.path());
    assert!(
        message.contains("Finalize milestone cache-rework (2 sub-tasks)"),
        "the aggregate message must be the synthesized projection; got:\n{message}",
    );

    let tree = head_tree_paths(repo.path());
    // The authored commit docs are transient — never promoted (materialize-then-skip).
    assert!(
        !tree.iter().any(|p| p.contains("commit:")),
        "the transient authored commit docs must NOT be promoted into the tree; got:\n{tree:?}",
    );
    // The aggregate DID land the merged persisted docs + the disjoint worktree code.
    assert!(
        tree.contains(&"docs/decisions/area-low-policy.md".to_owned())
            && tree.contains(&"docs/decisions/area-zed-policy.md".to_owned()),
        "the aggregate must commit the merged persisted docs; got:\n{tree:?}",
    );
    assert!(
        tree.contains(&"src/low.rs".to_owned()) && tree.contains(&"src/zed.rs".to_owned()),
        "the aggregate must combine the disjoint worktree code; got:\n{tree:?}",
    );
}

/// **Round-2 D1 — the sub-task finalize guard** (the batch-C live probe as a test):
/// `jigc task finalize <sub-id>` on a milestone sub-task, run from inside the
/// provisioned fan-out worktree with a filled commit doc and staged work, must
/// REFUSE with the routed blocking `finalize.milestone-sub-task` finding naming
/// `jigc milestone finalize <milestone-id>` — the promote-clobber refusal class
/// (an always-wrong destructive op: the milestone combine folds the sub-task
/// staged indexes directly, so a per-sub-task finalize has zero legitimate use).
///
/// RED before the guard: the finalize landed a commit on the worktree's detached
/// HEAD, emptied the staged index the combine folds, and the later
/// `jigc milestone finalize` silently landed WITHOUT the work while the record
/// claimed the sub-task joined.
#[test]
fn task_finalize_on_a_milestone_sub_task_refuses_with_the_milestone_route() {
    let repo = TempDir::new("subtask-finalize-guard");
    init_repo(repo.path());
    let home = TempDir::new("home");
    // The workflow re-entry needs a set-up project — an empty `.jigc/config/` layer
    // is the marker (the same seed the reachability tests use).
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk config layer");

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

    let wt = repo.path().join(".jigc").join("worktrees").join("area-low");
    assert!(wt.is_dir(), "the area-low worktree must be provisioned");

    // Re-enter from inside the worktree (provisions `commit:area-low`), then author
    // the commit doc — so the pre-guard code path would genuinely LAND a commit (the
    // destructive probe), not merely block on commit-doc validation.
    let reentry = run_jigc_in(
        &wt,
        home.path(),
        &["workflow", "sub-task", "--task", "area-low"],
        None,
    );
    assert!(
        reentry.status.success(),
        "sub-task re-entry must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&reentry.stderr),
    );
    let set_field = run_jigc_in(
        &wt,
        home.path(),
        &[
            "doc",
            "set-field",
            "commit:area-low#type",
            "--value",
            "feat",
            "--task",
            "area-low",
        ],
        None,
    );
    assert!(
        set_field.status.success(),
        "set-field must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&set_field.stderr),
    );
    let set_slot = run_jigc_in(
        &wt,
        home.path(),
        &[
            "doc",
            "set-slot",
            "commit:area-low#summary",
            "--from-file",
            "-",
            "--task",
            "area-low",
        ],
        Some(b"rework the low cache path\n"),
    );
    assert!(
        set_slot.status.success(),
        "set-slot must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&set_slot.stderr),
    );

    // Staged, uncommitted work in the worktree — exactly what the milestone combine
    // folds, and what the pre-guard finalize consumed.
    fs::write(wt.join("lru.py"), "print('lru')\n").expect("write worktree code");
    let add = Command::new("git")
        .args(["add", "lru.py"])
        .current_dir(&wt)
        .output()
        .expect("git add in the worktree");
    assert!(add.status.success(), "git add must succeed");

    let head_before = git_capture_in(&wt, &["rev-parse", "HEAD"]);

    let finalized = run_jigc_in(&wt, home.path(), &["task", "finalize", "area-low"], None);
    let stderr = String::from_utf8_lossy(&finalized.stderr);
    assert!(
        !finalized.status.success(),
        "`jigc task finalize` on a milestone sub-task must refuse; stdout:\n{}",
        String::from_utf8_lossy(&finalized.stdout),
    );
    assert!(
        stderr.contains("finalize.milestone-sub-task"),
        "the refusal must carry the `finalize.milestone-sub-task` key; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains("`jigc milestone finalize cache-rework`"),
        "the route must name the parent milestone's finalize verbatim; stderr:\n{stderr}",
    );

    // No commit landed on the detached worktree HEAD…
    assert_eq!(
        git_capture_in(&wt, &["rev-parse", "HEAD"]),
        head_before,
        "the refusal must land no commit on the worktree's detached HEAD",
    );
    // …and the staged index the milestone combine folds is intact.
    let staged = git_capture_in(&wt, &["diff", "--cached", "--name-only"]);
    assert!(
        staged.lines().any(|l| l == "lru.py"),
        "the staged work the combine folds must survive the refusal; staged:\n{staged}",
    );
}

/// `git <args>` in `dir`, captured stdout trimmed — the worktree-side sibling of the
/// repo-bound helpers above (the guard test reads the detached worktree HEAD/index).
fn git_capture_in(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// M45 Increment 9, T4 (Fork 4) — `milestone-execution` declares
/// `suppressed: {reason, expires}`, so the M43 law-2 narration
/// (`introspect.rs weave_workflow`, fires on `.suppressed.is_some()`) makes its
/// degenerate off-verb walk machine-visible: reached by a router pick it composes
/// zero `Spawn:` lines with an unresolved `<MILESTONE_ID>` and could never land
/// (`design/surface-contract.md` → law 2, the suppressed narration; DECISIONS
/// 2026-07-23 Settle → Fork 4). Driven over the EMITTED bytes of the real `jigc
/// describe` binary (the unfiltered menu narrates `creates-task: false` workflows
/// too), asserted **both** in the `--format json` projection and the prose surface.
#[test]
fn describe_narrates_milestone_execution_hidden_from_the_router_catalog() {
    let repo = TempDir::new("suppressed-narration");
    fs::create_dir_all(repo.path().join(".git")).expect("create .git marker");
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    let home = TempDir::new("suppressed-narration-home");

    // The authored reason — the verbatim degenerate-walk statement the pack ships.
    let reason = "composes a degenerate walk — zero `Spawn:` lines and an unresolved \
                  `<MILESTONE_ID>` off-verb, so a router pick could never land";

    // The JSON projection: milestone-execution's own narration carries the clause +
    // the declared reason.
    let json_out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["describe", "--format", "json"])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run the jigc binary");
    assert!(
        json_out.status.success(),
        "`jigc describe --format json` must exit 0; got {:?}\nstderr:\n{}",
        json_out.status,
        String::from_utf8_lossy(&json_out.stderr),
    );
    let json: serde_json::Value =
        serde_json::from_slice(&json_out.stdout).expect("describe --format json emits valid JSON");
    let prose = json["definitions"]
        .as_array()
        .expect("the projection carries a definitions array")
        .iter()
        .find(|d| d["id"].as_str() == Some("milestone-execution"))
        .expect("milestone-execution is narrated by describe")["prose"]
        .as_str()
        .expect("a definition prose");
    assert!(
        prose.contains("hidden from the router catalog"),
        "milestone-execution's entry must say it is hidden from the router catalog; got: {prose:?}",
    );
    assert!(
        prose.contains(reason),
        "milestone-execution's entry must carry its declared suppression reason; got: {prose:?}",
    );

    // The emitted prose surface (the bytes an agent reads) carries the same reason.
    let text_out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .arg("describe")
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run the jigc binary");
    assert!(
        text_out.status.success(),
        "`jigc describe` must exit 0; got {:?}\nstderr:\n{}",
        text_out.status,
        String::from_utf8_lossy(&text_out.stderr),
    );
    let text = String::from_utf8(text_out.stdout).expect("utf-8 stdout");
    assert!(
        text.contains(reason),
        "the emitted prose must carry milestone-execution's suppression reason; got:\n{text}",
    );
}

// ── N13 · `jigc milestone create` names its commit, its path, and the next step ─────────
//
// M47 Increment 8 / T5 (`completions/artifacts/M47/baseline.md` § 3c → N13;
// `design/surface-contract.md` → law 2, nothing hides). Live-reproduced at rc.9: `create`
// printed only `minted milestone:<id> (shared base <sha>)` while landing a
// `chore(milestone): open record …` commit and writing `docs/milestone-records/<id>.md` —
// commit, path and next step all unnamed, the `jigc setup` mold (which DOES name its install
// commit) unapplied. The ack now names all three — **and stays honest in the dev-only arm**,
// where there is no record and no commit to name.

/// Run the `jigc` binary with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack supersedes
/// the marker).
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads, so the
/// composed cascade resolves the `milestone-record` doctype and `create` lands a record commit.
fn compose_methodology(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write the compose marker");
}

/// The remainder of the line in `stdout` starting with `prefix`, panicking with the whole
/// surface when no such line was emitted (the emitted bytes are the contract).
fn ack_line<'a>(stdout: &'a str, prefix: &str) -> &'a str {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("the create ack must carry a `{prefix}` line; got:\n{stdout}"))
}

/// The backticked command span of an ack line (`next: \`jigc …\`   — why`) — the emitted argv,
/// extracted from the line the binary printed rather than reconstructed here.
fn backticked(line: &str) -> &str {
    let after = line
        .split_once('`')
        .unwrap_or_else(|| panic!("expected a backticked command in: {line}"))
        .1;
    after
        .split_once('`')
        .unwrap_or_else(|| panic!("expected a closing backtick in: {line}"))
        .0
}

/// Run the emitted `next:` argv verbatim, substituting `real` for its `"<intent>"`
/// placeholder — the emitted bytes are what an agent runs, so the test runs them too.
fn run_emitted_next_step(repo: &Path, home: &Path, stdout: &str, real: &str) {
    let argv = backticked(ack_line(stdout, "next: "));
    let mut parts: Vec<&str> = argv.split_whitespace().collect();
    assert_eq!(
        parts.first().copied(),
        Some("jigc"),
        "the next-step argv must lead with `jigc`; got `{argv}`",
    );
    let placeholder = parts
        .iter()
        .position(|p| *p == "\"<intent>\"")
        .unwrap_or_else(|| panic!("the next step must carry an `\"<intent>\"` slot; got `{argv}`"));
    parts[placeholder] = real;
    let out = run_jigc(repo, home, &parts[1..]);
    assert!(
        out.status.success(),
        "the emitted next step `{argv}` must run as printed; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// (a) `[dev ▸ methodology]` — the ack names the landed record commit (a short sha that
/// `git rev-parse` resolves to the `chore(milestone): open record …` commit at HEAD), the
/// record path it wrote (which exists on disk), and a next step that runs as printed.
#[test]
fn milestone_create_names_its_record_commit_its_path_and_the_next_step() {
    let repo = TempDir::new("ack-composed");
    init_repo(repo.path());
    let home = TempDir::new("home");
    compose_methodology(repo.path());

    let created = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "create", "Wave One"],
    );
    let stdout = String::from_utf8(created.stdout).expect("utf-8 stdout");
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );

    // The record path it names exists on disk, at the record home.
    let path = ack_line(&stdout, "record: ")
        .split_whitespace()
        .next()
        .expect("a named record path");
    assert_eq!(
        path, "docs/milestone-records/wave-one.md",
        "the ack names the record's repo-relative home; got:\n{stdout}",
    );
    assert!(
        repo.path().join(path).is_file(),
        "the named record must exist on disk at {path}; got:\n{stdout}",
    );

    // The short sha it names resolves — to the record commit, which is HEAD.
    let sha = ack_line(&stdout, "record commit: ")
        .split_whitespace()
        .next()
        .expect("a named record commit");
    let resolved = Command::new("git")
        .args(["rev-parse", "--verify", &format!("{sha}^{{commit}}")])
        .current_dir(repo.path())
        .output()
        .expect("run git");
    assert!(
        resolved.status.success(),
        "`git rev-parse {sha}` must resolve the named record commit; stderr:\n{}",
        String::from_utf8_lossy(&resolved.stderr),
    );
    let resolved = String::from_utf8(resolved.stdout).expect("utf-8 sha");
    assert_eq!(
        resolved.trim(),
        git_state(repo.path()).0.trim(),
        "the named commit must be the record commit at HEAD",
    );
    assert_eq!(
        head_message(repo.path()).lines().next().unwrap_or_default(),
        "chore(milestone): open record for milestone:wave-one",
        "the named sha must be the record-opening commit",
    );

    // The next step runs verbatim.
    run_emitted_next_step(repo.path(), home.path(), &stdout, "Add caching");
}

/// (b) Dev-only (no methodology pack) — there is no record and no record commit, so the ack
/// claims neither (law 1: naming one would be the lie this task closes), while still naming
/// the next step.
#[test]
fn milestone_create_dev_only_claims_no_record_and_no_commit_but_names_the_next_step() {
    let repo = TempDir::new("ack-dev-only");
    init_repo(repo.path());
    let home = TempDir::new("home");

    let before = rev_list_count(repo.path());
    let created = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "create", "Wave One"],
    );
    let stdout = String::from_utf8(created.stdout).expect("utf-8 stdout");
    assert!(
        created.status.success(),
        "dev-only `jigc milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );
    assert_eq!(
        rev_list_count(repo.path()),
        before,
        "dev-only create lands no commit — the ack below must claim none",
    );
    assert!(
        !stdout.contains("record commit:"),
        "dev-only create must claim NO record commit (it made none); got:\n{stdout}",
    );
    assert!(
        !stdout.contains("milestone-records"),
        "dev-only create must claim NO record path (it wrote none); got:\n{stdout}",
    );
    assert!(
        stdout.contains("minted milestone:wave-one"),
        "the mint itself is still named; got:\n{stdout}",
    );

    run_emitted_next_step(repo.path(), home.path(), &stdout, "Add caching");
}

/// (c) The pinned `--format json` envelope is untouched by the growth: exactly
/// `{text, hook_output}`, with the grown ack inside `text`
/// (`design/command-output-contract.md` → Stream discipline).
#[test]
fn milestone_create_json_envelope_keeps_exactly_text_and_hook_output() {
    let repo = TempDir::new("ack-json");
    init_repo(repo.path());
    let home = TempDir::new("home");
    compose_methodology(repo.path());

    let created = run_jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "milestone", "create", "Wave One"],
    );
    assert!(
        created.status.success(),
        "`jigc --format json milestone create` must exit 0; got {:?}\nstderr:\n{}",
        created.status,
        String::from_utf8_lossy(&created.stderr),
    );
    let value: serde_json::Value =
        serde_json::from_slice(&created.stdout).expect("the create envelope parses as JSON");
    let keys: Vec<&str> = value
        .as_object()
        .expect("a JSON object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        vec!["hook_output", "text"],
        "the create envelope stays exactly `{{text, hook_output}}`; got {value}",
    );
    let text = value["text"].as_str().expect("a text string");
    assert!(
        text.contains("record commit: ") && text.contains("next: "),
        "the growth rides inside `text`; got:\n{text}",
    );
}
