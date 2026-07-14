//! M42 Inc 12 / T6 — **the anchor finding names the root it read, and routes to `git add`**
//! (`design/validation.md`:279).
//!
//! The two roots are correct by design (`validation.md`:265): task scope resolves a
//! `code-anchor` against the **materialized git index** (exactly the bytes `finalize`
//! commits), store scope against the **on-disk working tree**. The defect the rc.5 trial
//! surfaced is that the finding said neither — on the dominant disagreement case (a file
//! **present on disk, never `git add`ed**) the task-scope finding claimed the file was
//! *"absent from the working tree"* (factually false — it is sitting right there) and its
//! route commanded *"update the citation … or revert the change"*: **both of the wrong
//! repairs**, since the citation and the code are both correct and the only correct action
//! is **`git add`**. An agent following that route corrupts a correct citation.
//!
//! The proof drives the **real binary** (never a reconstruction) over a real git repo and
//! the real `doc-code` probe subprocess:
//!
//! - **(a) task scope, present-but-unstaged** — an ADR citing `src/feature.rs#feature`,
//!   the file written to disk but **not staged** → `jigc task finalize` **blocks**, the
//!   message names the **staged index** (and never says "absent from the working tree"),
//!   and the route names **`git add`**. A `git add` of that one file, then the *same*
//!   finalize → the commit **lands**, carrying the file. (The repair the route commands is
//!   the repair that works — the route is executed, not merely read.)
//! - **(b) store scope, genuinely absent** — a committed ADR citing a file that exists
//!   nowhere → `jigc validate` names the **working tree** (the root the store sweep really
//!   read) and keeps its **update-the-citation** route. The asymmetry is preserved, not
//!   flattened: a reader can always tell which generation produced the verdict.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-anchor-root-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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

/// Build the `doc-code` probe executable once and return its path (the `doc_code_gate`
/// idiom — the probe crate lives outside the workspace, built into its own target dir).
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE
        .get_or_init(|| {
            let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("probes/doc-code/Cargo.toml");
            let target_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("probes/doc-code/target");
            let out = Command::new(env!("CARGO"))
                .args(["build", "--manifest-path"])
                .arg(&manifest)
                .arg("--target-dir")
                .arg(&target_dir)
                .output()
                .expect("cargo build the doc-code probe");
            assert!(
                out.status.success(),
                "cargo build failed for the doc-code probe:\n{}",
                String::from_utf8_lossy(&out.stderr),
            );
            let bin = target_dir.join("debug/doc-code");
            assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
            bin
        })
        .as_path()
}

/// Run a `git` command in `repo`, asserting success, returning trimmed stdout.
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
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Run `jigc <args>` with the real `doc-code` probe selected, optionally piping stdin.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> std::process::Output {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    out
}

/// The joined `stdout` + `stderr` of an invocation — the bytes an agent actually reads.
fn rendered(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// Initialize a real git repo with one commit.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Fill the created ADR's author-required prose slots.
fn fill_adr_slots(repo: &Path, home: &Path, slug: &str) {
    for (section, prose) in [
        ("context", &b"Forces at play.\n"[..]),
        ("options", &b"Alternatives were weighed.\n"[..]),
        ("decision", &b"We decided.\n"[..]),
        ("consequences", &b"Tradeoffs.\n"[..]),
    ] {
        let addr = format!("adr:{slug}#{section}");
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", &addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(
            out.status.success(),
            "set-slot {addr} must succeed; stderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    for (field, value) in [("type", "feat"), ("scope", "engine")] {
        ok(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#{field}"),
                "--value",
                value,
            ],
            "set-field",
        );
    }
    for (slot, prose) in [
        ("summary", &b"add the feature\n"[..]),
        ("body", &b"A feature.\n"[..]),
    ] {
        let addr = format!("commit:{task}#{slot}");
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", &addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    }
}

/// Inject a `cites-code: <anchor>` line into the staged ADR's front-matter (the optional
/// anchor is absent from the created skeleton — the `doc_code_gate::inject_cites_code`
/// idiom keeps this test's concern the *finding*, not the write path).
fn inject_cites_code(repo: &Path, task: &str, slug: &str, anchor: &str) {
    let staged = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("adr:{slug}.md"));
    let body = fs::read_to_string(&staged).expect("read staged ADR");
    let with = body.replacen("\n---\n", &format!("\ncites-code: {anchor}\n---\n"), 1);
    assert_ne!(body, with, "the staged ADR carries a front-matter block");
    fs::write(&staged, &with).expect("inject cites-code anchor");
}

/// (a) Task scope — a doc citing a **present-but-unstaged** file blocks at `jigc task
/// finalize` with a message naming the **staged index** and a route naming **`git add`**;
/// the `git add` the route commands, plus the *same* finalize, lands the commit.
#[test]
fn unstaged_cited_file_blocks_naming_the_staged_index_and_routes_to_git_add() {
    let repo = TempDir::new("unstaged-repo");
    let home = TempDir::new("unstaged-home");
    init_repo(repo.path());

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    let task = "add-the-feature";
    ok(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "add the feature"],
        "jigc start",
    );

    let slug = "feature-decision";
    ok(
        repo.path(),
        home.path(),
        &["doc", "create", "adr", "--title", "Feature decision"],
        "jigc doc create adr",
    );
    fill_adr_slots(repo.path(), home.path(), slug);
    fill_commit(repo.path(), home.path(), task);
    inject_cites_code(repo.path(), task, slug, "src/feature.rs#feature");

    // The agent wrote the cited code — and never `git add`ed it. It IS in the working
    // tree; it is NOT in the index `finalize` commits.
    fs::create_dir_all(repo.path().join("src")).expect("mk src");
    fs::write(
        repo.path().join("src/feature.rs"),
        "pub fn feature() -> u32 {\n    7\n}\n",
    )
    .expect("write feature.rs");

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    let text = rendered(&out);
    assert!(
        !out.status.success(),
        "the unstaged cited file must block finalize; output:\n{text}",
    );
    assert!(
        text.contains("absent from the staged index"),
        "the finding must name the root it read — the staged index; output:\n{text}",
    );
    assert!(
        !text.contains("absent from the working tree"),
        "the file IS in the working tree — the finding must not claim otherwise; output:\n{text}",
    );
    assert!(
        text.contains("git add"),
        "the route must name `git add` — the only correct repair; output:\n{text}",
    );

    // Execute the repair the route commands, verbatim — then the same finalize lands.
    git(repo.path(), &["add", "src/feature.rs"]);
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    assert!(
        out.status.success(),
        "the routed `git add` must clear the block; output:\n{}",
        rendered(&out),
    );
    let landed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        landed.lines().any(|f| f == "src/feature.rs"),
        "the staged cited file lands in the commit; files:\n{landed}",
    );
}

/// (b) Store scope — a committed doc citing a **genuinely absent** file keeps the working
/// tree as the root it names and the update-the-citation route: the sweep really does read
/// the working tree, so the asymmetry is preserved (never flattened onto the index).
#[test]
fn store_sweep_over_an_absent_file_names_the_working_tree() {
    let repo = TempDir::new("store-repo");
    let home = TempDir::new("store-home");

    init_repo(repo.path());
    fs::create_dir_all(repo.path().join("docs/decisions")).expect("mk decisions");
    fs::write(
        repo.path().join("docs/decisions/cache.md"),
        "---\n\
         status: accepted\n\
         date: 2026-07-13\n\
         cites-code: src/gone.rs#gone\n\
         ---\n\
         \n\
         # The cache decision\n\
         \n\
         ## Context\n\
         Forces.\n\
         \n\
         ## Options\n\
         Alternatives.\n\
         \n\
         ## Decision\n\
         Decided.\n\
         \n\
         ## Consequences\n\
         Effects.\n",
    )
    .expect("write the committed adr");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed the store"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    let out = jigc(repo.path(), home.path(), &["validate"], None);
    let text = rendered(&out);
    assert!(
        text.contains("doc-code.symbol-exists") && text.contains("src/gone.rs#gone"),
        "the absent cited file must surface a doc-code finding naming the anchor; output:\n{text}",
    );
    assert!(
        text.contains("`src/gone.rs` is absent from the working tree"),
        "the store sweep reads the working tree — its finding must name that root; \
         output:\n{text}",
    );
    assert!(
        !text.contains("staged index"),
        "the store sweep is task-less — it must never claim to have read a staged index; \
         output:\n{text}",
    );
    assert!(
        text.contains("update the citation to match the renamed/moved code"),
        "a genuinely absent file keeps the update-the-citation route; output:\n{text}",
    );
}
