//! The `locate-from-spec` read-back, driven on **both** arms of the write it names
//! (M46 Increment 8, T7 — the razor ledger's **B3b-2**;
//! `design/surface-contract.md` → law 1 + the surface style guide's scope rule;
//! `design/doc-read-surface.md` → R7, the staged read).
//!
//! The defect: the step printed *"Read your write back … **the write you just
//! made**"* over `jigc doc show spec:<slug> --task <id>` **unconditionally**, while
//! its own prose four paragraphs up sanctions leaving the spec write off — *"leave
//! `maps-to-test` off a criterion this task did not cover rather than pointing it at
//! a test you have not written and staged"*. Take that sanctioned path and the task
//! stages nothing of the spec, so the read the step just promised blocks
//! `store.not-staged`. A law-1 lie over a refusal that is itself right: the repair is
//! **scope** — name the write this step always makes, and say what the member outside
//! that scope does instead — never a behaviour change that stages the spec anyway.
//!
//! The claim pinned, on the **emitted bytes**: every `jigc doc show …` argv the
//! composed step prints, run **verbatim**, exits 0 in **both** arms —
//!
//!   * **arm A** sets `maps-to-test` on the bound spec's criterion (the spec is
//!     staged), and
//!   * **arm B** sets none (the path the step's own prose sanctions).
//!
//! Both arms follow the step in the order it states: the writes it solicits are the
//! **emitted** `jigc doc set-field` lines, filled only at their agent-fill `<…>`
//! placeholders and run verbatim, so a broken emission fails here rather than passing
//! against a hand-built equivalent. Arm B additionally pins the state it stands for —
//! the staged spec read really does block there — so a "fix" that staged the spec
//! regardless cannot green this suite.
//!
//! Scoped to the `locate-from-spec` region of the composed workflow: the two sibling
//! steps `implement-from-spec` also includes print read-backs of their own, and this
//! is a claim about one step's prose.

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
            "jigc-spec-read-back-{tag}-{}-{:?}",
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
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
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

/// Stage a slot from `prose` for `addr`, asserting it succeeds.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc_doc(
        repo,
        home,
        &["set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert_ok(&out, &format!("set-slot {addr}"));
}

/// Stage a field `value` for `addr`, asserting it succeeds.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert_ok(&out, &format!("set-field {addr}"));
}

/// The slug `id-from: title` derives for the spec the setup task commits.
const SPEC_SLUG: &str = "gateway-rate-limiting";
/// The criterion's title — its `{#id}` anchor is read back out of the composed slice.
const CRITERION_TITLE: &str = "Rejects the 101st request";
/// The `implement-from-spec` task both arms mint.
const TASK: &str = "enforce-the-rate-limit";

/// The first line of `step:locate-from-spec` — the region this suite's claim is about.
const STEP_OPENING: &str = "Implement from a committed spec.";
/// The first line of `step:implement`, the next step the workflow includes.
const NEXT_STEP_OPENING: &str = "Implement the change directly in the working tree.";

/// Setup — a `plan` task creates, authors and finalizes `spec:gateway-rate-limiting`
/// with one criterion, so the committed store carries a spec to bind and read.
fn commit_spec_with_criterion(repo: &Path, home: &Path) {
    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "plan", "draft the rate limit spec"],
    );
    assert_ok(&out, "`jigc start --workflow plan` (setup task)");
    let task = "draft-the-rate-limit-spec";

    let create = jigc_doc(
        repo,
        home,
        &["create", "spec", "--title", "Gateway rate limiting"],
        None,
    );
    assert_ok(&create, "`jigc doc create spec`");

    set_slot(
        repo,
        home,
        &format!("spec:{SPEC_SLUG}#goal"),
        b"Bound per-client request volume at the gateway.\n",
    );
    set_slot(
        repo,
        home,
        &format!("spec:{SPEC_SLUG}#context"),
        b"Downstream services each enforced limits ad hoc.\n",
    );

    let add = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            &format!("spec:{SPEC_SLUG}#criteria"),
            "--title",
            CRITERION_TITLE,
        ],
        None,
    );
    assert_ok(&add, "`jigc doc add-item spec:…#criteria`");
    let item_addr = String::from_utf8(add.stdout)
        .expect("utf-8")
        .trim()
        .to_owned();
    set_slot(
        repo,
        home,
        &format!("{item_addr}/statement"),
        b"The gateway rejects the 101st request in a rolling 60s window.\n",
    );

    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"draft the rate limit spec\n",
    );

    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (setup task)");
}

/// The `locate-from-spec` slice of the composed workflow — the step whose prose is
/// under test, cut off before the next step's own read-back.
fn locate_from_spec_region(composed: &str) -> String {
    let start = composed
        .find(STEP_OPENING)
        .unwrap_or_else(|| panic!("the compose must open `locate-from-spec`; got:\n{composed}"));
    let end = composed
        .find(NEXT_STEP_OPENING)
        .unwrap_or_else(|| panic!("the compose must include `step:implement`; got:\n{composed}"));
    assert!(
        start < end,
        "`locate-from-spec` must precede `step:implement`; got:\n{composed}",
    );
    composed[start..end].to_owned()
}

/// Every `jigc doc show …` argv the region prints — a standalone command line, and an
/// inline backticked span unwrapped across its hard wrap. Both are argv an agent reads
/// off this surface and runs; a fix that only made the standalone one honest would be
/// pinned by an extractor that saw only standalone lines.
fn printed_doc_show_argvs(region: &str) -> Vec<String> {
    const NEEDLE: &str = "jigc doc show";
    let bytes = region.as_bytes();
    let mut out: Vec<String> = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = region[from..].find(NEEDLE) {
        let start = from + rel;
        from = start + NEEDLE.len();
        let rest = &region[start..];
        let argv = if start > 0 && bytes[start - 1] == b'`' {
            let end = rest
                .find('`')
                .unwrap_or_else(|| panic!("an inline code span must close; got:\n{region}"));
            rest[..end].split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            rest.lines()
                .next()
                .unwrap_or_default()
                .trim_end()
                .to_owned()
        };
        out.push(argv);
    }
    out
}

/// The emitted line matching `needle`, trimmed — the pack's own bytes, never a
/// reconstruction.
fn emitted_line(region: &str, needle: &str) -> String {
    region
        .lines()
        .find(|line| line.trim_start().starts_with("jigc doc ") && line.contains(needle))
        .unwrap_or_else(|| panic!("the step must emit a `{needle}` line; got:\n{region}"))
        .trim()
        .to_owned()
}

/// Run one filled argv line verbatim, returning the invocation's output.
fn run_emitted(repo: &Path, home: &Path, filled: &str) -> std::process::Output {
    assert!(
        !filled.contains('<') && !filled.contains('>'),
        "every agent-fill placeholder must be filled before the line runs; got:\n{filled}",
    );
    let args: Vec<&str> = filled.split_whitespace().collect();
    assert_eq!(args[0], "jigc", "the emitted line must invoke jigc");
    jigc(repo, home, &args[1..])
}

/// Drive one arm: mint + bind + re-compose, take the sanctioned path or not, run the
/// commit write the step always solicits, and run every printed `jigc doc show` argv
/// verbatim.
fn drive_arm(arm: &str, set_maps_to_test: bool) {
    let repo = TempDir::new(arm);
    let home = TempDir::new("home");
    init_repo(repo.path());
    commit_spec_with_criterion(repo.path(), home.path());

    let mint = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "implement-from-spec",
            "enforce the rate limit",
        ],
    );
    assert_ok(&mint, "`jigc start --workflow implement-from-spec`");
    let bind = jigc(
        repo.path(),
        home.path(),
        &["task", "bind", "spec", &format!("spec:{SPEC_SLUG}"), TASK],
    );
    assert_ok(&bind, "`jigc task bind spec`");
    let resume = jigc(repo.path(), home.path(), &["start", "--task", TASK]);
    assert_ok(&resume, "`jigc start --task <id>` re-compose");
    let composed = String::from_utf8(resume.stdout).expect("utf-8");
    let region = locate_from_spec_region(&composed);

    if set_maps_to_test {
        // Arm A takes the wiring path: a real staged test, then the EMITTED
        // `set-field …/maps-to-test` line filled at its `<…>` placeholders only.
        fs::create_dir_all(repo.path().join("src")).expect("create src/");
        fs::write(
            repo.path().join("src").join("limiter.rs"),
            "pub fn burst() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn burst_is_capped() {}\n}\n",
        )
        .expect("write the test");
        git(repo.path(), &["add", "src/limiter.rs"]);

        let anchor_line = region
            .lines()
            .find(|line| line.contains(CRITERION_TITLE) && line.contains("{#"))
            .unwrap_or_else(|| {
                panic!("the criteria slice must anchor the criterion; got:\n{region}")
            });
        let item_id = anchor_line
            .split_once("{#")
            .and_then(|(_, rest)| rest.split_once('}'))
            .map(|(id, _)| id.to_owned())
            .expect("the anchor carries an id");

        let filled = emitted_line(&region, "maps-to-test")
            .replace("<slug>", SPEC_SLUG)
            .replace("<id>", &item_id)
            .replace("<path>#<test-fn>", "src/limiter.rs#burst_is_capped");
        let out = run_emitted(repo.path(), home.path(), &filled);
        assert_ok(
            &out,
            &format!("[{arm}] the EMITTED `set-field …/maps-to-test` line, run verbatim"),
        );
    } else {
        // Arm B takes the path the step's own prose sanctions — `maps-to-test` left
        // off — so this task stages NOTHING of the spec. Pinned here: the repair is a
        // scoping of what the step promises, never a behaviour change that stages the
        // spec anyway.
        let staged_spec_read = jigc(
            repo.path(),
            home.path(),
            &["doc", "show", &format!("spec:{SPEC_SLUG}"), "--task", TASK],
        );
        assert!(
            !staged_spec_read.status.success()
                && String::from_utf8_lossy(&staged_spec_read.stderr).contains("store.not-staged"),
            "[{arm}] with `maps-to-test` left off, the task must stage nothing of the spec \
             (`store.not-staged`) — the sanctioned state this arm stands for; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&staged_spec_read.stdout),
            String::from_utf8_lossy(&staged_spec_read.stderr),
        );
    }

    // The write the step solicits unconditionally, run from its EMITTED line.
    let filled = emitted_line(&region, "#implements").replace("<slug>", SPEC_SLUG);
    let out = run_emitted(repo.path(), home.path(), &filled);
    assert_ok(
        &out,
        &format!("[{arm}] the EMITTED `set-field commit:…#implements` line, run verbatim"),
    );

    // The claim: EVERY printed read argv, run verbatim, answers.
    let argvs = printed_doc_show_argvs(&region);
    assert!(
        !argvs.is_empty(),
        "[{arm}] the step must print at least one `jigc doc show` argv; got:\n{region}",
    );
    assert!(
        argvs.iter().any(|argv| argv.contains("--task")),
        "[{arm}] the step must keep stating a staged read-back (`--task {TASK}`); got:\n{argvs:#?}",
    );
    for argv in &argvs {
        let filled = argv.replace("<slug>", SPEC_SLUG);
        let out = run_emitted(repo.path(), home.path(), &filled);
        assert!(
            out.status.success(),
            "[{arm}] the printed read `{filled}` must exit 0 — the step prints it \
             unconditionally, so it must answer on every path the step sanctions; \
             stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }
}

/// B3b-2 — the read-back the step prints must answer on both arms of the write it
/// names: with `maps-to-test` set (the spec is staged) and with it left off (the path
/// `locate-from-spec`'s own prose sanctions).
#[test]
fn every_printed_read_answers_whether_or_not_the_optional_spec_write_was_made() {
    drive_arm("arm-a-maps-to-test-set", true);
    drive_arm("arm-b-maps-to-test-left-off", false);
}
