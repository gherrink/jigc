//! Flow 9 **seam verification** — real `--task`-scoped writes satisfy the M7
//! by-task-id join's preconditions with correct provenance, closed through the
//! binary (M8 Increment 3, T6 — `implementation/roadmap.md` → M8 increment 3
//! bullet 3, the integration-seam discipline; `design/storage.md` → The by-task-id
//! join (M7), Same-doc clash).
//!
//! This is the headline of the increment: it **replaces flow-9's hand-staging**.
//! `flow9_milestone_join.rs` writes `tasks/<sub>/docs/<addr>.md` + `provenance.json`
//! directly (its `stage_doc` helper) "because no front-door verb yet stages into a
//! milestone sub-area". T1–T5 built that front door — `jigc doc … --task <id>` with
//! copy-on-first-touch + write-once provenance. T6 proves the join consumes those
//! **real-written** inputs:
//!
//!   - **Disjoint case merges.** Two sub-areas populated through the binary — one
//!     `created` (`jigc doc create adr`), one `edited-from-base` (a first
//!     `set-slot --task` against a base-committed ADR copies the committed body in
//!     and records `edited-from-base`) — write *distinct* slugs. `jigc milestone
//!     join` consumes the real `docs/<addr>.md` + `provenance.json` of each area and
//!     reports a clean merge (exit 0).
//!   - **Mixed same-slug case BLOCKS.** One sub-area `created`s `adr:eviction-policy`;
//!     a sibling `edited-from-base` the *same* committed-at-base slug. The real
//!     `created` × `edited-from-base` overlap on one address is a partition violation
//!     the join blocks with `join.same-doc-clash`, committing nothing.
//!
//! The discipline that makes this the seam proof and not a re-run of
//! `flow9_milestone_join.rs`: **no `stage_doc` hand-staging.** Every staged input the
//! join reads here is written through `jigc workflow … --task` + `jigc doc … --task`.
//! A regression that broke copy-on-first-touch, the `--task` selector, or the
//! provenance record would change the bytes/manifest the join reads and fail these
//! assertions — whereas a hand-staged fixture would mask it.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow9-seam-{tag}-{}-{:?}",
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

/// Run `git` in `repo`, asserting success.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn the jigc binary");
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(bytes)
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing stderr on failure.
fn expect_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A canonical committed ADR (the `write::render` form of an ADR), so a first-touch
/// copy-in is byte-stable. Committed at `docs/decisions/<slug>.md`.
fn committed_adr(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nThe ORIGINAL committed decision prose.\n\n## Consequences\n\nNone.\n"
    )
}

/// Initialize a git repo with one commit + the `.jigc/config/` project layer + a
/// committed ADR at `docs/decisions/eviction-policy.md` (the base doc copy-on-first-touch
/// pulls in).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(root.join("docs").join("decisions")).expect("create docs/decisions/");
    fs::write(
        root.join("docs")
            .join("decisions")
            .join("eviction-policy.md"),
        committed_adr("Eviction policy"),
    )
    .expect("write committed adr");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// A sub-task's `docs/` working area: `.jigc/tasks/<sub>/docs/`.
fn docs_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub).join("docs")
}

/// Stand up a milestone with two write-ready sub-task areas (each minted `--workflow
/// single-task`, then re-entered once so its commit doc is provisioned — the same
/// shape T1–T5 use). Returns `(repo, home, subA, subB)`.
fn milestone_with_two_subtasks(tag: &str) -> (TempDir, TempDir, String, String) {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());

    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "milestone create",
    );

    let sub_a = "move-cache-to-redis".to_string();
    let sub_b = "evict-stale-keys".to_string();
    for intent in ["Move cache to redis", "Evict stale keys"] {
        expect_ok(
            &run(
                repo.path(),
                home.path(),
                &[
                    "milestone",
                    "add-task",
                    "cache-rework",
                    intent,
                    "--workflow",
                    "single-task",
                ],
            ),
            "milestone add-task",
        );
    }
    for sub in [&sub_a, &sub_b] {
        expect_ok(
            &run(
                repo.path(),
                home.path(),
                &["workflow", "single-task", "--task", sub],
            ),
            "first re-entry provisions the commit doc",
        );
    }

    (repo, home, sub_a, sub_b)
}

/// **The seam — real `--task` writes feed the join; a disjoint set merges clean.**
/// Two sub-areas are populated *only* through the binary: subA `created`s a fresh ADR
/// (`jigc doc create adr`), subB `edited-from-base` a committed ADR (a first
/// `set-slot --task` copies the committed body in, T4's copy-on-first-touch). Their
/// slugs are distinct, so `jigc milestone join` consumes the real-written
/// `docs/<addr>.md` + `provenance.json` of each area and reports a clean merge.
#[test]
fn real_task_writes_feed_a_clean_disjoint_join() {
    let (repo, home, sub_a, sub_b) = milestone_with_two_subtasks("disjoint");

    // subA: a `created` ADR through the front door (gate-allowed `{type: adr}`),
    // then a real slot write — records `created` provenance.
    expect_ok(
        &run_doc(
            repo.path(),
            home.path(),
            &[
                "create",
                "adr",
                "--title",
                "Cache strategy",
                "--task",
                &sub_a,
            ],
            None,
        ),
        "doc create adr in subA",
    );
    expect_ok(
        &run_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                "adr:cache-strategy#decision",
                "--from-file",
                "-",
                "--task",
                &sub_a,
            ],
            Some(b"Use a write-through cache.\n"),
        ),
        "set-slot on the created ADR in subA",
    );

    // subB: a first `set-slot --task` against the base-committed `adr:eviction-policy`
    // — copy-on-first-touch pulls the committed body in, splices, records
    // `edited-from-base`.
    expect_ok(
        &run_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                "adr:eviction-policy#decision",
                "--from-file",
                "-",
                "--task",
                &sub_b,
            ],
            Some(b"Evict on a TTL sweep.\n"),
        ),
        "edit the base-committed ADR in subB (copy-on-first-touch)",
    );

    // Sanity — these are REAL writes, not hand-staged: the bodies + manifests the join
    // reads were produced by the binary. subA's manifest says `created`; subB's says
    // `edited-from-base`; subB's body carries the copied-in committed slice.
    let prov_a = fs::read_to_string(docs_area(repo.path(), &sub_a).join("provenance.json"))
        .expect("subA provenance.json");
    assert!(
        prov_a.contains("\"adr:cache-strategy\": \"created\""),
        "subA's real-written manifest must record `created`; got:\n{prov_a}",
    );
    let prov_b = fs::read_to_string(docs_area(repo.path(), &sub_b).join("provenance.json"))
        .expect("subB provenance.json");
    assert!(
        prov_b.contains("\"adr:eviction-policy\": \"edited-from-base\""),
        "subB's real-written manifest must record `edited-from-base`; got:\n{prov_b}",
    );
    let body_b = fs::read_to_string(docs_area(repo.path(), &sub_b).join("adr:eviction-policy.md"))
        .expect("subB copied-in body");
    assert!(
        body_b.contains("Forces.") && body_b.contains("Evict on a TTL sweep."),
        "subB's body must be the copied-in committed body with the spliced decision; got:\n{body_b}",
    );

    // The join consumes those real-written inputs and reports a clean merge: the
    // disjoint slugs are both in the overlay, no blocking finding, exit 0.
    let joined = run(
        repo.path(),
        home.path(),
        &["milestone", "join", "cache-rework"],
    );
    expect_ok(
        &joined,
        "the join over real-written disjoint areas must merge clean",
    );
    let stdout = String::from_utf8_lossy(&joined.stdout);
    assert!(
        stdout.contains("adr:cache-strategy") && stdout.contains("adr:eviction-policy"),
        "the clean merge must report both real-written addresses; got:\n{stdout}",
    );
}

/// **The seam — a real `edited-from-base` × `created` overlap on one slug BLOCKS
/// `join.same-doc-clash`.** subA `created`s `adr:eviction-policy` through the binary;
/// subB `edited-from-base` the *same* committed-at-base slug (copy-on-first-touch).
/// The mixed provenance on one address is a partition violation the join blocks,
/// committing nothing — driven entirely through real `--task` writes (no hand-staging).
#[test]
fn a_real_edited_from_base_times_created_overlap_blocks_same_doc_clash() {
    let (repo, home, sub_a, sub_b) = milestone_with_two_subtasks("clash");

    // subA: `created` `adr:eviction-policy` (its title slugs to `eviction-policy`,
    // colliding with the committed-at-base slug subB will edit).
    expect_ok(
        &run_doc(
            repo.path(),
            home.path(),
            &[
                "create",
                "adr",
                "--title",
                "Eviction policy",
                "--task",
                &sub_a,
            ],
            None,
        ),
        "doc create adr:eviction-policy in subA",
    );
    expect_ok(
        &run_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                "adr:eviction-policy#decision",
                "--from-file",
                "-",
                "--task",
                &sub_a,
            ],
            Some(b"A freshly created eviction decision.\n"),
        ),
        "set-slot on the created ADR in subA",
    );

    // subB: a first `set-slot --task` against the SAME base-committed
    // `adr:eviction-policy` — copy-on-first-touch records `edited-from-base`.
    expect_ok(
        &run_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                "adr:eviction-policy#decision",
                "--from-file",
                "-",
                "--task",
                &sub_b,
            ],
            Some(b"An edited eviction decision.\n"),
        ),
        "edit the base-committed ADR in subB (copy-on-first-touch)",
    );

    // The two real-written manifests carry the mixed provenance on the same address:
    // subA `created`, subB `edited-from-base`.
    let prov_a = fs::read_to_string(docs_area(repo.path(), &sub_a).join("provenance.json"))
        .expect("subA provenance.json");
    assert!(
        prov_a.contains("\"adr:eviction-policy\": \"created\""),
        "subA's manifest must record `created`; got:\n{prov_a}",
    );
    let prov_b = fs::read_to_string(docs_area(repo.path(), &sub_b).join("provenance.json"))
        .expect("subB provenance.json");
    assert!(
        prov_b.contains("\"adr:eviction-policy\": \"edited-from-base\""),
        "subB's manifest must record `edited-from-base`; got:\n{prov_b}",
    );

    // The join over those real-written areas blocks: a `join.same-doc-clash` naming
    // both contenders, non-zero exit, nothing committed.
    let before_head = git(repo.path(), &["rev-parse", "HEAD"]);
    let joined = run(
        repo.path(),
        home.path(),
        &["milestone", "join", "cache-rework"],
    );
    assert!(
        !joined.status.success(),
        "a real mixed `edited-from-base` × `created` overlap must block (non-zero exit); \
         got {:?}\nstdout:\n{}",
        joined.status,
        String::from_utf8_lossy(&joined.stdout),
    );
    let stderr = String::from_utf8_lossy(&joined.stderr);
    assert!(
        stderr.contains("same-doc clash")
            && stderr.contains("adr:eviction-policy")
            && stderr.contains(sub_a.as_str())
            && stderr.contains(sub_b.as_str())
            && stderr.contains("route:"),
        "the block must be `join.same-doc-clash` naming the address + both real sub-tasks \
         + a route; got:\n{stderr}",
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        before_head,
        "the clash block must commit nothing",
    );
}
