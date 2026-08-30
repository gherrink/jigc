//! M49 Increment 10 / T3 — **the fan-out spawn line runs each sub-task's recorded workflow.**
//!
//! The `fan-out` step's front-matter `run: workflow:sub-task` was the *only* thing the emitted
//! `` Spawn: `cd .jigc/worktrees/<sub> && jigc workflow <W> --task <sub>` `` line could name, so a
//! sub-task minted `jigc milestone add-task … --workflow decided-task` was handed a launch line
//! naming `sub-task` — and the re-entry W-equality guard refused it:
//!
//! ```text
//! names workflow `sub-task`, but sub-task `<id>` was minted with `decided-task`
//!   — re-entry must compose the recorded workflow
//! ```
//!
//! The pack told the agent to run a command the binary refuses. Since M49 Increment 9 the
//! minting workflow is recorded per sub-task **and fresh-clone durable**, so the CLI feeds
//! `(id, recorded workflow)` pairs into `{{milestone.tasks}}` and the emit names each item's own
//! workflow; the step's `run:` survives as the fall-back for an item carrying no recorded value.
//!
//! **Every assertion below drives the REAL binary and reads the EMITTED bytes** — each
//! `` Spawn: `…` `` span is extracted from the composed view and run **verbatim** through a
//! `jigc` shim on `PATH`, never rebuilt in test code: the emitted bytes are the contract.
//!
//! Two arms, over the axis the emit now discriminates:
//!
//!   * **the overridden × un-overridden pair** — one milestone carrying both, each span naming
//!     its own recorded workflow and each composing at exit 0; and
//!   * **the empty milestone** — the `fan-out` over an empty collection still emits zero
//!     `Spawn:` lines at exit 0 (the empty-vs-unresolvable stance, unchanged by the pairing).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-spawn-workflow-{tag}-{}-{:?}",
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

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
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

/// Assert an invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Initialize a `[dev ▸ methodology]` repo whose **initial** commit already carries the compose
/// marker — so the milestone's base pin (HEAD at `create`) is that commit, every later commit is
/// record-only, and each provisioned worktree (detached at the base) carries the marker too.
/// The methodology pack is what makes `decided-task` a real workflow to override with.
fn init_methodology_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    assert_ok(
        &run_jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "`jigc milestone create`",
    );
}

/// `jigc milestone add-task cache-rework "<intent>" [--workflow <w>]`, returning the minted
/// sub-task id **read out of the door's own ack** — never re-slugged in test code.
fn add_task(repo: &Path, home: &Path, intent: &str, workflow: Option<&str>) -> String {
    let mut args = vec!["milestone", "add-task", "cache-rework", intent];
    if let Some(w) = workflow {
        args.push("--workflow");
        args.push(w);
    }
    let out = run_jigc(repo, home, &args);
    assert_ok(&out, "`jigc milestone add-task`");
    let ack = String::from_utf8_lossy(&out.stdout).to_string();
    ack.split_once("task:")
        .and_then(|(_, rest)| rest.split_whitespace().next())
        .unwrap_or_else(|| panic!("the add-task ack names the minted sub-task; got:\n{ack}"))
        .to_string()
}

/// Every `` Spawn: `…` `` span of a composed milestone-execution view, **verbatim** — each is a
/// shell line (`cd <worktree> && jigc workflow <W> --task <id>`), run as emitted below.
fn spawn_spans(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| line.strip_prefix("Spawn: `"))
        .filter_map(|rest| rest.strip_suffix('`'))
        .map(str::to_owned)
        .collect()
}

/// Install a `jigc` shim on a throwaway `PATH` entry, so an emitted `Spawn:` span — which names
/// the bare command `jigc`, as an agent would run it — resolves to the binary under test.
#[cfg(unix)]
fn install_jigc_shim(dir: &Path) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let bin = dir.join("shim-bin");
    fs::create_dir_all(&bin).expect("mk the shim bin dir");
    let shim = bin.join("jigc");
    fs::write(
        &shim,
        format!("#!/bin/sh\nexec {:?} \"$@\"\n", env!("CARGO_BIN_EXE_jigc")),
    )
    .expect("write the jigc shim");
    fs::set_permissions(&shim, fs::Permissions::from_mode(0o755)).expect("chmod the jigc shim");
    bin
}

/// Run an emitted shell span verbatim through `sh -c`, with the `jigc` shim first on `PATH`.
#[cfg(unix)]
fn run_span(cwd: &Path, home: &Path, shim_bin: &Path, span: &str) -> std::process::Output {
    let path = match std::env::var("PATH") {
        Ok(rest) => format!("{}:{rest}", shim_bin.display()),
        Err(_) => shim_bin.display().to_string(),
    };
    Command::new("sh")
        .arg("-c")
        .arg(span)
        .current_dir(cwd)
        .env("HOME", home)
        .env("PATH", path)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the emitted span")
}

/// **The driven repro, green.** A milestone carrying one `--workflow decided-task` sub-task and
/// one un-overridden sibling: each emitted `Spawn:` span names *that* sub-task's recorded
/// workflow, and each composes at exit 0 when run **verbatim** from its own worktree. Before the
/// fix the overridden span named the step's `run:` workflow `sub-task` and exited 1 against the
/// re-entry W-equality guard.
#[cfg(unix)]
#[test]
fn each_spawn_span_names_and_runs_the_sub_tasks_recorded_workflow() {
    let repo = TempDir::new("pair");
    let home = TempDir::new("home");
    init_methodology_repo(repo.path(), home.path());

    let overridden = add_task(
        repo.path(),
        home.path(),
        "Tune the eviction clock",
        Some("decided-task"),
    );
    let plain = add_task(repo.path(), home.path(), "Warm the read cache", None);

    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &["milestone", "provision", "cache-rework"],
        ),
        "`jigc milestone provision`",
    );
    let executed = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert_ok(&executed, "`jigc milestone execute`");
    let view = String::from_utf8(executed.stdout).expect("utf-8 composed view");
    let spans = spawn_spans(&view);

    // The emitted bytes themselves — one span per sub-task, each naming its OWN recorded
    // workflow, in the canonical id-sorted order the collection resolves in.
    let mut expected = vec![
        format!(
            "cd .jigc/worktrees/{overridden} && jigc workflow decided-task --task {overridden}"
        ),
        format!("cd .jigc/worktrees/{plain} && jigc workflow sub-task --task {plain}"),
    ];
    expected.sort();
    assert_eq!(
        spans, expected,
        "each `Spawn:` span must name the workflow its own sub-task was minted with; got:\n{view}",
    );

    // And every span RUNS — the whole point: the pack may not emit a command the binary refuses.
    let shim_bin = install_jigc_shim(repo.path());
    for span in &spans {
        let composed = run_span(repo.path(), home.path(), &shim_bin, span);
        assert!(
            composed.status.success(),
            "the emitted span `{span}` must compose at exit 0; got {:?}\nstdout:\n{}\nstderr:\n{}",
            composed.status,
            String::from_utf8_lossy(&composed.stdout),
            String::from_utf8_lossy(&composed.stderr),
        );
    }
}

/// The **empty** milestone is unchanged: the `fan-out` over an empty collection emits zero
/// `Spawn:` lines and still exits 0 (empty text, never a finding) — the pairing changed what a
/// spawn line *says*, not when one exists.
#[test]
fn an_empty_milestone_emits_no_spawn_line() {
    let repo = TempDir::new("empty");
    let home = TempDir::new("home");
    init_methodology_repo(repo.path(), home.path());

    let executed = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-rework"],
    );
    assert_ok(
        &executed,
        "`jigc milestone execute` over an empty milestone",
    );
    let view = String::from_utf8(executed.stdout).expect("utf-8 composed view");
    assert!(
        spawn_spans(&view).is_empty(),
        "an empty milestone emits no `Spawn:` line; got:\n{view}",
    );
}
