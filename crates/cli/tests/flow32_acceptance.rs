//! M30 Increment 4 acceptance (T3) — flow 32, the marquee: per-task `finalize` commits
//! exactly the **declared change-set** (the agent's git index + jigc's promoted docs)
//! and **surfaces the rest**. The narrowing logic shipped across Inc 1–3
//! (`StagePolicy::IndexHonoring` + the left-out manifest + the index-validated `doc-code`
//! gate); flow 32 proves the closed loop end-to-end on the **production path** a real
//! install hits — the cargo-built `jigc` binary with the **embedded** dev pack (no
//! `JIGC_PACK_DIR`) and the `doc-code` probe resolved as a **sibling of `jigc`** (no
//! `JIGC_DOC_CODE_PROBE` override), the flow30/31 real-binary idiom.
//!
//! `design/worked-examples.md` → flow 32; `design/finalize.md` → Dirty-tree policy
//! (revised M30); `DECISIONS.md` → 2026-06-20 M30 Increment 4 planning (G1/G2/G3/G4/G6);
//! `implementation/roadmap.md` → M30 Increment 4, Acceptance.
//!
//! Two cohesive proofs over the cargo-built binary:
//!
//! - **The change-set is scoped, the rest is surfaced** — a finalize with a *staged* task
//!   edit alongside an unrelated *untracked* file and an unrelated *unstaged-modified
//!   tracked* file commits **only** the staged edit (+ jigc's own promoted/config files);
//!   both unrelated files **remain** uncommitted in the working tree post-commit, and the
//!   finalize output **names the left-out set** (the untracked file + the unstaged-tracked
//!   file). The assertions inspect the **landed git commit** (`git show --name-only HEAD`),
//!   the post-commit `git status --porcelain`, and the **emitted** finalize bytes — never a
//!   reconstruction.
//!
//! - **A citation the agent wrote but did NOT stage blocks** — an `arch-doc` whose
//!   component anchors a Rust symbol the agent appended to a tracked file but never
//!   `git add`ed: the finalize-scope `doc-code` probe validates the **materialized git
//!   index** (M30 Inc 3, G4), so the symbol is absent → `finalize` **blocks** on
//!   `doc-code.symbol-exists` naming the dangling anchor; HEAD is unchanged, nothing
//!   promoted. Validated reality == committed reality.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow32-{tag}-{}-{:?}",
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

/// HEAD commit count — the no-commit witness the blocking arm asserts is unchanged.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README.md");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the **embedded** pack (no
/// `JIGC_PACK_DIR`), and **no `JIGC_DOC_CODE_PROBE` override** — so the probe resolves
/// through the **production default** path (`jigc` spawning itself), the path a real
/// install hits. `env_remove` guards against an env var
/// leaking in from the test runner.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output (production path —
/// embedded pack, no probe override).
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE");
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Set a doc field, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert_ok(&out, &format!("set-field {addr}"));
}

/// Set a doc slot from piped stdin, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc_doc(
        repo,
        home,
        &["set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert_ok(&out, &format!("set-slot {addr}"));
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str, ty: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), ty);
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"scope the declared change set\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"An M30 change.\n",
    );
}

/// The marquee: a per-task `finalize` commits exactly the agent's **staged** edit (+ jigc's
/// own files), leaves both unrelated dirty paths uncommitted, and **names the left-out set**
/// in its emitted output (the untracked file + the unstaged-tracked file). Proven on the
/// production path (embedded pack, self-spawned probe).
#[test]
fn flow32_finalize_commits_only_the_staged_set_and_surfaces_the_rest() {
    let repo = TempDir::new("scope-repo");
    let home = TempDir::new("scope-home");
    init_repo(repo.path());

    let task = "scope-the-set";
    let started = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "scope the set"],
    );
    assert_ok(&started, "`jigc start --workflow single-task`");

    // The agent's own task edit — written AND staged (the G5 contract: `git add` first).
    fs::write(repo.path().join("feature.rs"), "pub fn feature() {}\n").expect("write feature.rs");
    git(repo.path(), &["add", "feature.rs"]);

    // An unrelated untracked file — must NOT ride the commit, must be surfaced as left out.
    fs::write(repo.path().join("scratch.txt"), "private WIP\n").expect("write scratch.txt");
    // An unrelated unstaged-modified TRACKED file — same.
    fs::write(repo.path().join("README.md"), "hello\nlocal edit\n").expect("modify README.md");

    fill_commit(repo.path(), home.path(), task, "feat", "cache");

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize`");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // (1) The landed commit carries the staged edit and NEITHER unrelated file.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    let landed: Vec<&str> = committed.lines().collect();
    assert!(
        landed.contains(&"feature.rs"),
        "the staged task edit lands in the commit; files:\n{committed}",
    );
    assert!(
        !landed.contains(&"scratch.txt"),
        "the unrelated untracked file must NOT ride the commit; files:\n{committed}",
    );
    assert!(
        !landed.contains(&"README.md"),
        "the unrelated unstaged-modified tracked file must NOT ride the commit; files:\n{committed}",
    );

    // Both unrelated changes REMAIN uncommitted in the working tree post-commit.
    let status = git(repo.path(), &["status", "--porcelain"]);
    assert!(
        status.lines().any(|l| l == "?? scratch.txt"),
        "the untracked file stays untracked after the commit; status:\n{status}",
    );
    assert!(
        status
            .lines()
            .any(|l| l.trim_start().starts_with('M') && l.ends_with("README.md")),
        "the unstaged-modified tracked file stays modified after the commit; status:\n{status}",
    );
    // HEAD's README.md is still the unedited baseline (the local edit never landed).
    assert_eq!(
        git(repo.path(), &["show", "HEAD:README.md"]),
        "hello",
        "the unrelated local edit must not be in the committed README.md",
    );

    // (2) The emitted finalize output NAMES the left-out set (untracked + unstaged-tracked).
    assert!(
        stdout.contains("left-out"),
        "finalize surfaces a left-out section; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("scratch.txt"),
        "finalize names the left-out untracked file; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("README.md"),
        "finalize names the left-out unstaged-tracked file; stdout:\n{stdout}",
    );
}

/// The blocking arm: a doc citing a code symbol the agent **wrote but did NOT stage** →
/// `finalize` blocks, because the finalize-scope `doc-code` probe validates the materialized
/// git **index** (not the working tree, M30 Inc 3 G4). The agent appends `render_widget` to a
/// tracked `widget.rs` but never `git add`s it; an unrelated file IS staged so the narrowed
/// set is non-empty (the block is the doc-code gate, never the empty-commit guard). Proven on
/// the production path (embedded pack, self-spawned probe).
#[test]
fn flow32_finalize_blocks_when_a_cited_symbol_is_unstaged() {
    let repo = TempDir::new("block-repo");
    let home = TempDir::new("block-home");
    init_repo(repo.path());

    // A tracked `widget.rs` committed WITHOUT the cited symbol — present in the index, but
    // the symbol the anchor names is absent until staged.
    fs::write(repo.path().join("widget.rs"), "pub fn placeholder() {}\n").expect("write widget.rs");
    git(repo.path(), &["add", "widget.rs"]);
    git(repo.path(), &["commit", "-q", "-m", "track widget"]);

    let task = "document-the-gateway";
    let started = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "document the gateway",
        ],
    );
    assert_ok(
        &started,
        "`jigc start --workflow architecture-documentation`",
    );

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "arch-doc", "--title", "Gateway"],
        None,
    );
    assert_ok(&create, "`jigc doc create arch-doc`");
    let arch = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(arch, "arch-doc:gateway", "minted arch-doc address");

    set_slot(
        repo.path(),
        home.path(),
        "arch-doc:gateway#overview",
        b"The gateway renders widgets.\n",
    );

    // One component anchored at a Rust symbol — its emitted item address is the contract.
    let added = jigc_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            "arch-doc:gateway#components",
            "--title",
            "Widget",
        ],
        None,
    );
    assert_ok(&added, "`jigc doc add-item …#components`");
    let item = String::from_utf8(added.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_owned();
    assert_eq!(
        item, "arch-doc:gateway#components/widget",
        "emitted item address"
    );
    set_slot(
        repo.path(),
        home.path(),
        &format!("{item}/description"),
        b"Renders a widget.\n",
    );
    set_field(
        repo.path(),
        home.path(),
        &format!("{item}/implemented-by"),
        "widget.rs#render_widget",
    );

    // The agent writes the cited symbol into the tracked file but does NOT stage it.
    fs::write(
        repo.path().join("widget.rs"),
        "pub fn placeholder() {}\npub fn render_widget() {}\n",
    )
    .expect("append render_widget unstaged");
    // An unrelated staged file so the narrowed finalize set is non-empty — the block is the
    // doc-code gate, never the empty-commit guard.
    fs::write(repo.path().join("other.rs"), "pub fn other() {}\n").expect("write other.rs");
    git(repo.path(), &["add", "other.rs"]);

    fill_commit(repo.path(), home.path(), task, "docs", "arch-doc");

    let before = head_count(repo.path());
    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", task],
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    assert!(
        !out.status.success(),
        "an unstaged cited symbol must make finalize block (index, not working tree); got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces the doc-code.symbol-exists check; got:\n{rendered}",
    );
    assert!(
        rendered.contains(&format!("{item}/implemented-by")),
        "the block names the citing item's anchor address; got:\n{rendered}",
    );
    assert!(
        rendered.contains("widget.rs#render_widget"),
        "the block names the dangling anchor target; got:\n{rendered}",
    );
    // A probe that could not run would surface as a floor-locked crash meta-finding — assert
    // the production path ran the probe (not a probe-integrity failure standing in).
    assert!(
        !rendered.contains("pack-probe-integrity"),
        "the production path must run the `doc-code` probe; got:\n{rendered}",
    );

    assert_eq!(
        before,
        head_count(repo.path()),
        "a blocked finalize creates no commit",
    );
    assert!(
        !repo.path().join("architecture").join("gateway.md").exists()
            && !repo
                .path()
                .join("docs")
                .join("architecture")
                .join("gateway.md")
                .exists(),
        "a blocked finalize promotes nothing",
    );
}
