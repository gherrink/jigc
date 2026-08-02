//! The #4 face (M8-inc-4 hardening): the spawn launch line is an **executed-bytes
//! contract**. The adapter owns a validated launch template; this test proves the
//! rendered launch line actually *resolves to the real re-entry verb* when run —
//! guarding the L1 landmine, a template naming a command that does not exist
//! (`design/assistant-adapter.md` → Bind the spawn mechanism; roadmap M8
//! Increment 4 bullet 3; `increment-workflow.md` → M8 faces, #4 face).
//!
//! The contract is the **rendered bytes**, never a reconstruction. So the command
//! is never hand-written in test code: it flows from the **shipped** spawn template
//! bytes (the embedded `adapters/claude-code.yaml` asset — what actually ships)
//! through the launch render (the lexical `{{worktree}}`/`{{workflow}}`/`{{task_id}}`
//! substitution `render_spawn` performs) for a **real** provisioned milestone
//! sub-task, the single backticked `cd <worktree> && jigc workflow …` span the
//! install-time validation rule guarantees is extracted verbatim, and **that** span
//! is executed as a subprocess (through a shell, into the provisioned worktree)
//! against the built `jigc` binary.
//!
//! The L1 guard: a template naming a nonexistent verb would surface as a clap
//! unknown-subcommand / usage error (non-zero, a clap diagnostic). The shipped
//! template names `jigc workflow … --task …` — the M8-inc-2 `Command::Workflow`
//! re-entry verb (cli.rs → `run_reenter` → `start::reenter_in_repo`). So the
//! executed line must reach **that verb's compose path**: for a real sub-task it
//! composes the single-task view (exit 0, carrying the create-gate ADR affordance),
//! never a clap parse failure.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-spawn-exec-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project
/// layer so the cascade resolves (milestone mint reads HEAD).
fn init_repo(root: &Path) {
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
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn run(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// The **shipped** spawn launch template — read verbatim from the embedded
/// `adapters/claude-code.yaml` asset (the bytes that ship in the binary), never
/// hand-written here. The template line is `  template: "<...>"`; we lift the
/// double-quoted value so the test renders exactly what the adapter renders.
fn shipped_spawn_template() -> String {
    let yaml = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("adapters")
            .join("claude-code.yaml"),
    )
    .expect("the embedded claude-code profile asset exists");
    let line = yaml
        .lines()
        .map(str::trim_start)
        .find(|l| l.starts_with("template:"))
        .expect("the shipped profile declares a spawn template");
    let open = line.find('"').expect("the template value is double-quoted");
    let rest = &line[open + 1..];
    let close = rest
        .rfind('"')
        .expect("the template value closes its quote");
    rest[..close].to_owned()
}

/// The launch render: the lexical three-token substitution the adapter's
/// `render_spawn` performs (`{{worktree}}`/`{{workflow}}`/`{{task_id}}` → the
/// concrete values, every other byte verbatim; `{{worktree}}` →
/// `.jigc/worktrees/<task_id>`). The cli crate is a binary by invariant
/// (`module-layout.md` → the I/O boundary), so the integration test reproduces
/// this one operation rather than linking the private fn — but it renders the
/// *shipped* template bytes, so the command is composed, not authored.
fn render_spawn(template: &str, workflow: &str, task_id: &str) -> String {
    template
        .replace("{{worktree}}", &format!(".jigc/worktrees/{task_id}"))
        .replace("{{workflow}}", workflow)
        .replace("{{task_id}}", task_id)
}

/// Extract the single backticked span the install-time validation rule guarantees
/// (`validate_spawn_template`: exactly one backticked span). This is the rendered
/// launch command, lifted verbatim from the rendered launch line.
fn backticked_span(rendered: &str) -> String {
    let mut parts = rendered.split('`');
    let _before = parts.next();
    let span = parts
        .next()
        .expect("the rendered launch line carries a backticked span");
    assert!(
        parts.next().is_some() && parts.next().is_none(),
        "the rendered launch line must carry exactly one backticked span; got {rendered:?}",
    );
    span.to_owned()
}

/// Execute `command` VERBATIM as a shell line with `cwd = repo`, `$HOME = home`, and
/// the built `jigc` binary's directory **prepended to `$PATH`** so the bare `jigc`
/// token in the launch span resolves to the binary under test. The shipped span now
/// carries a `cd <worktree> && jigc workflow …` chain (M31 WF4), so it must run
/// through a shell, not as a single argv — the executed-bytes contract.
fn run_shell(repo: &Path, home: &Path, command: &str) -> std::process::Output {
    let bin_dir = Path::new(env!("CARGO_BIN_EXE_jigc"))
        .parent()
        .expect("the built jigc binary has a parent dir");
    let path = match std::env::var_os("PATH") {
        Some(existing) => {
            let mut dirs = vec![bin_dir.to_path_buf()];
            dirs.extend(std::env::split_paths(&existing));
            std::env::join_paths(dirs).expect("join PATH")
        }
        None => bin_dir.as_os_str().to_owned(),
    };
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(repo)
        .env("HOME", home)
        .env("PATH", path)
        .output()
        .expect("run the rendered launch span via a shell")
}

#[test]
fn rendered_spawn_line_resolves_to_the_real_reentry_verb() {
    let repo = TempDir::new("exec");
    init_repo(repo.path());
    let home = TempDir::new("home");

    // Provision a REAL milestone sub-task whose recorded mint workflow is
    // single-task, so the re-entry verb composes (the W-equality guard passes).
    let created = run(
        repo.path(),
        home.path(),
        &["milestone", "create", "Cache rework"],
    );
    assert!(
        created.status.success(),
        "`jigc milestone create` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr),
    );
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
        "`add-task --workflow single-task` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&added.stderr),
    );
    // Provision the worktrees (T1) — the `cd <worktree>` half of the shipped span
    // needs the detached checkout to exist before the launch span runs.
    let provisioned = run(
        repo.path(),
        home.path(),
        &["milestone", "provision", "cache-rework"],
    );
    assert!(
        provisioned.status.success(),
        "`jigc milestone provision` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&provisioned.stderr),
    );
    let workflow = "single-task";
    let sub = "move-cache-to-redis";

    // Render the SHIPPED template for the REAL (workflow, sub-task-id), then lift
    // the single backticked span verbatim — the executed-bytes contract.
    let template = shipped_spawn_template();
    let rendered = render_spawn(&template, workflow, sub);
    let command = backticked_span(&rendered);

    // The rendered command directs the sub-agent into its worktree, then names the
    // real re-entry verb — derived from the rendered bytes, not a hand-built string.
    assert_eq!(
        command,
        format!("cd .jigc/worktrees/{sub} && jigc workflow {workflow} --task {sub}"),
        "the rendered launch span must `cd` into the worktree then invoke the verb; got {command:?}",
    );

    // Execute the rendered span VERBATIM via a shell (cwd = repo, the built `jigc` on
    // PATH): `cd .jigc/worktrees/<sub>` enters the provisioned worktree, then
    // `jigc workflow …` re-enters from inside it (the WF3 jigc_home resolver finds the
    // main checkout's `.jigc/`).
    let executed = run_shell(repo.path(), home.path(), &command);

    // It must RESOLVE to the real `Command::Workflow` re-entry verb's compose path:
    // for this real sub-task it composes the single-task view (exit 0), carrying the
    // create-gate ADR affordance — never a clap unknown-subcommand / usage error.
    let stderr = String::from_utf8_lossy(&executed.stderr);
    assert!(
        executed.status.success(),
        "the rendered launch line must reach the real re-entry verb and compose \
         (exit 0), not a clap unknown-subcommand error; stderr:\n{stderr}",
    );
    let stdout = String::from_utf8(executed.stdout).expect("utf-8 stdout");
    assert!(
        stdout.contains("jigc doc create adr"),
        "the executed launch line must compose the single-task view (its create-gate \
         ADR affordance), proving it reached `start::reenter_in_repo`, not a clap \
         parse failure; got:\n{stdout}",
    );
    // A clap unknown-subcommand / usage diagnostic would name a usage line; the real
    // verb's compose carries none.
    assert!(
        !stderr.contains("Usage:") && !stderr.to_lowercase().contains("unrecognized"),
        "the rendered command must not trip a clap parse error; stderr:\n{stderr}",
    );
}
