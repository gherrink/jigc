//! Integration test for copy-on-first-touch on the production `--task <id>` edit
//! path (M8 Increment 3, T4 — `design/write-commands.md` → copy-on-first-touch:
//! "the first write to a base-committed doc copies the committed body into the
//! sub-area (`copy_in` → `edited-from-base`), then splices").
//!
//! Drives the built `jigc` binary against a throwaway temp git repo holding a
//! milestone sub-task area plus a committed ADR at `decisions/<slug>.md`. The
//! committed ADR is **absent** from the sub-area, so the first `set-slot
//! --task <sub>` against it must copy the committed body in, splice, and record
//! `edited-from-base` provenance the M7 join consumes. The cases:
//!
//! - (i) a first edit of a base-committed-but-unstaged slug copies the committed
//!   body in, splices, and `provenance.json` records `<addr> → edited-from-base`;
//! - (ii) a second edit keeps `edited-from-base` and does **not** re-copy
//!   (the first edit's prose survives — no clobber by a fresh copy-in);
//! - (iii) a created-then-edited doc stays `created` (write-once, sticky-`created`);
//! - (iv) an address neither staged nor committed still rejects.
//!
//! This is the copy-in part of grouped-scope bullet 2 — `state::copy_in` wired into
//! the real verb for the first time, proven through the binary (never the primitive).

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
            "jigc-doc-copy-in-{tag}-{}-{:?}",
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
fn git(repo: &Path, args: &[&str]) {
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

/// The committed ADR body that gets copied in on first touch. Canonical form
/// (`write::render` of an ADR), so a first-touch copy-in is byte-stable.
const COMMITTED_ADR: &str = "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# Single-node cache\n\n## Context\n\nForces.\n\n## Decision\n\nThe ORIGINAL committed decision prose.\n\n## Consequences\n\nNone.\n";

/// Initialize a git repo with one commit + the `.jigc/config/` project layer +
/// a committed ADR at `decisions/single-node-cache.md`.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(root.join("decisions")).expect("create decisions/");
    fs::write(
        root.join("decisions").join("single-node-cache.md"),
        COMMITTED_ADR,
    )
    .expect("write committed adr");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// The sub-task's `docs/` area path of a staged instance `<type>:<slug>.md`.
fn staged(repo: &Path, sub: &str, ty: &str, slug: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs")
        .join(format!("{ty}:{slug}.md"))
}

/// The sub-task's provenance manifest path.
fn provenance(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(sub)
        .join("docs")
        .join("provenance.json")
}

/// Stand up a milestone with ONE write-ready sub-task area (minted `--workflow
/// single-task`, re-entered once so its commit doc is provisioned). One sub-task
/// keeps `--task <sub>` unambiguous so the test concern stays copy-on-first-touch.
fn milestone_with_one_subtask() -> (TempDir, TempDir, &'static str) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "`milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );

    let sub = "move-cache-to-redis";
    let added = run(
        repo.path(),
        home.path(),
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Move cache to redis",
            "--workflow",
            "single-task",
        ],
    );
    assert!(
        added.status.success(),
        "`add-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr),
    );

    let entered = run(
        repo.path(),
        home.path(),
        &["workflow", "single-task", "--task", sub],
    );
    assert!(
        entered.status.success(),
        "first re-entry of `{sub}` must provision its commit doc; stderr:\n{}",
        String::from_utf8_lossy(&entered.stderr),
    );

    (repo, home, sub)
}

#[test]
fn first_touch_of_a_base_committed_adr_copies_in_and_records_edited_from_base() {
    let (repo, home, sub) = milestone_with_one_subtask();

    // (i) The committed `adr:single-node-cache` is absent from the sub-area.
    let adr_staged = staged(repo.path(), sub, "adr", "single-node-cache");
    assert!(
        !adr_staged.exists(),
        "the committed ADR must NOT be pre-staged in the sub-area",
    );

    // First edit of the base-committed-but-unstaged slug: copies the committed
    // body in, then splices the new decision prose.
    let first = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            "adr:single-node-cache#decision",
            "--from-file",
            "-",
            "--task",
            sub,
        ],
        Some(b"The REPLACED decision prose.\n"),
    );
    assert!(
        first.status.success(),
        "first `set-slot` of a base-committed ADR must copy-in + splice (exit 0); stderr:\n{}",
        String::from_utf8_lossy(&first.stderr),
    );

    // The committed body was copied in: the OTHER committed slices survive
    // (proving the body came from `decisions/single-node-cache.md`, not a fresh
    // empty template), and the edited slice carries the new prose.
    let body = fs::read_to_string(&adr_staged).expect("read staged ADR after copy-in");
    assert!(
        body.contains("Forces."),
        "the committed `## Context` prose must survive the copy-in; got:\n{body}",
    );
    assert!(
        body.contains("The REPLACED decision prose."),
        "the spliced `## Decision` prose must be present; got:\n{body}",
    );
    assert!(
        !body.contains("The ORIGINAL committed decision prose."),
        "the spliced decision must REPLACE the committed one; got:\n{body}",
    );

    // The provenance manifest records `<addr> → edited-from-base`.
    let manifest = fs::read_to_string(provenance(repo.path(), sub)).expect("provenance.json");
    assert!(
        manifest.contains("\"adr:single-node-cache\": \"edited-from-base\""),
        "provenance.json must record `adr:single-node-cache -> edited-from-base`; got:\n{manifest}",
    );

    // The committed source is untouched (copy-in writes only the working copy).
    let committed = fs::read_to_string(repo.path().join("decisions").join("single-node-cache.md"))
        .expect("read committed ADR");
    assert_eq!(
        committed, COMMITTED_ADR,
        "the committed source file must be untouched by the copy-in",
    );

    // (ii) A second edit keeps `edited-from-base` and does NOT re-copy — the first
    // edit's prose survives (a re-copy would clobber it back to the committed body).
    let second = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            "adr:single-node-cache#consequences",
            "--from-file",
            "-",
            "--task",
            sub,
        ],
        Some(b"A second edited slice.\n"),
    );
    assert!(
        second.status.success(),
        "second `set-slot` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&second.stderr),
    );
    let body = fs::read_to_string(&adr_staged).expect("read staged ADR after second edit");
    assert!(
        body.contains("The REPLACED decision prose."),
        "the first edit's prose must survive the second edit (no re-copy clobber); got:\n{body}",
    );
    assert!(
        body.contains("A second edited slice."),
        "the second edit's prose must be present; got:\n{body}",
    );
    let manifest = fs::read_to_string(provenance(repo.path(), sub)).expect("provenance.json");
    assert!(
        manifest.contains("\"adr:single-node-cache\": \"edited-from-base\""),
        "the second edit keeps `adr:single-node-cache -> edited-from-base`, never flips; got:\n{manifest}",
    );
}

#[test]
fn a_created_then_edited_doc_stays_created() {
    let (repo, home, sub) = milestone_with_one_subtask();

    // (iii) Create a fresh ADR in the sub-area — records `created`.
    let create = run_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "New decision", "--task", sub],
        None,
    );
    assert!(
        create.status.success(),
        "`doc create adr` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr),
    );
    assert!(
        staged(repo.path(), sub, "adr", "new-decision").is_file(),
        "the created ADR must be staged",
    );

    // Edit the created doc — provenance must stay `created` (write-once, sticky).
    let edit = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            "adr:new-decision#decision",
            "--from-file",
            "-",
            "--task",
            sub,
        ],
        Some(b"A decision on a freshly created ADR.\n"),
    );
    assert!(
        edit.status.success(),
        "editing a created ADR must succeed; stderr:\n{}",
        String::from_utf8_lossy(&edit.stderr),
    );
    let manifest = fs::read_to_string(provenance(repo.path(), sub)).expect("provenance.json");
    assert!(
        manifest.contains("\"adr:new-decision\": \"created\""),
        "a created-then-edited doc must stay `created`; got:\n{manifest}",
    );
    assert!(
        !manifest.contains("edited-from-base"),
        "a created-then-edited doc must NOT flip to `edited-from-base`; got:\n{manifest}",
    );
}

#[test]
fn an_address_neither_staged_nor_committed_still_rejects() {
    let (repo, home, sub) = milestone_with_one_subtask();

    // (iv) `adr:ghost` is neither staged in the sub-area nor committed at base.
    let out = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            "adr:ghost#decision",
            "--from-file",
            "-",
            "--task",
            sub,
        ],
        Some(b"nothing to splice into\n"),
    );
    assert!(
        !out.status.success(),
        "a `set-slot` of a slug neither staged nor committed must reject (non-zero exit)",
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("no staged instance") && stderr.contains("adr:ghost"),
        "the rejection must say `no staged instance` and name the address; got:\n{stderr}",
    );
}
