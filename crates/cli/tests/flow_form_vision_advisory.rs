//! M39 Increment 7 / T3 — the **`form-vision` empty-research advisory**, proven
//! end-to-end over the real `jigc` binary under the **`[dev ▸ methodology]`**
//! composition (`ideas/form-vision-research-routing.md`, settled 2026-07-06;
//! `DECISIONS.md` 2026-07-06 M39 planning → form-vision advisory (G7)).
//!
//! A `vision` grounds in the committed research it cites (`grounded-in` is `0..*` —
//! zero research is a LEGAL state). When `form-vision` is composed against an EMPTY
//! research store, its opening (`author-vision`) step carries an advisory route line
//! nudging `do-research` first; when at least one `research` is committed, the
//! advisory is SILENT. The routing is **advisory, never blocking** — a fresh project
//! composes `form-vision` clean either way (some visions ground in experience, and
//! grounding may take several research rounds).
//!
//! Both facts are asserted on the EMITTED bytes of the real binary
//! (`CARGO_BIN_EXE_jigc`) over the `[dev ▸ methodology]` composition — the committed
//! research is authored through the REAL `do-research` workflow, so the query reads a
//! genuinely-committed store. No external test crates.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The distinctive advisory phrase author-vision.yaml carries at its opening step and
/// the CLI keeps only when no research is committed.
const ADVISORY_PHRASE: &str = "consider running `do-research`";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-form-vision-advisory-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Run a `git` command in `repo`, asserting success.
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer
/// naming the methodology pack over the embedded dev base (`[dev ▸ methodology]`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn jigc");
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

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Commit one grounding `research` doc through the REAL `do-research` workflow, so the
/// committed research store is genuinely non-empty. `intent` derives the task id;
/// `title` mints `research:<slug>`.
fn commit_research(repo: &Path, home: &Path, intent: &str, title: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "do-research", intent],
            None,
        ),
        "`jigc start --workflow do-research`",
    );
    let create = jigc(
        repo,
        home,
        &["doc", "create", "research", "--title", title],
        None,
    );
    assert_ok(&create, "`jigc doc create research`");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    set_slot(repo, home, &format!("{addr}#question"), b"A question.\n");
    set_slot(repo, home, &format!("{addr}#findings"), b"Findings.\n");
    set_slot(repo, home, &format!("{addr}#sources"), b"Some sources.\n");
    let task = intent.replace(' ', "-");
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), "research");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record research\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"A grounding record.\n",
    );
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task], None),
        "`jigc task finalize` (do-research) — the committed grounding target",
    );
}

/// Compose `form-vision` and return the emitted composed workflow (stdout).
fn compose_form_vision(repo: &Path, home: &Path) -> String {
    let start = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "form-vision",
            "form the project vision",
        ],
        None,
    );
    assert_ok(&start, "`jigc start --workflow form-vision` must compose");
    String::from_utf8(start.stdout).expect("utf-8 composed stdout")
}

/// EMPTY research store → the advisory route line appears at the author-vision opening
/// step (nudge `do-research` first; advisory, never blocking).
#[test]
fn form_vision_advises_do_research_when_no_research_is_committed() {
    let repo = TempDir::new("empty");
    let home = TempDir::new("empty-home");
    init_repo(repo.path());

    let composed = compose_form_vision(repo.path(), home.path());
    assert!(
        composed.contains(ADVISORY_PHRASE),
        "an EMPTY research store must surface the `do-research` advisory at the \
         author-vision opening step; got:\n{composed}",
    );
    // Advisory, never blocking: the composed workflow still carries the create-vision
    // command-ref (composition succeeded, the vision can still be authored).
    assert!(
        composed.contains("jigc doc create vision"),
        "the advisory is non-blocking — form-vision still composes fully; got:\n{composed}",
    );
    // The delimiting markers never leak into the emitted bytes.
    assert!(
        !composed.contains("research-advisory"),
        "the advisory-block markers must be stripped from the emitted output; got:\n{composed}",
    );
}

/// NON-EMPTY research store (one committed via the real `do-research`) → the advisory
/// is SILENT; form-vision composes clean.
#[test]
fn form_vision_is_silent_when_research_is_committed() {
    let repo = TempDir::new("nonempty");
    let home = TempDir::new("nonempty-home");
    init_repo(repo.path());

    commit_research(
        repo.path(),
        home.path(),
        "benchmark the cache",
        "Cache Benchmarks",
    );

    let composed = compose_form_vision(repo.path(), home.path());
    assert!(
        !composed.contains(ADVISORY_PHRASE),
        "with committed research the `do-research` advisory must be SILENT; got:\n{composed}",
    );
    assert!(
        !composed.contains("research-advisory"),
        "the advisory-block markers must be stripped from the emitted output; got:\n{composed}",
    );
    assert!(
        composed.contains("jigc doc create vision"),
        "form-vision still composes fully; got:\n{composed}",
    );
}
