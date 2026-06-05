//! Flow 10 **Half-A step 1** — the deterministic `Spawn:` emit + the L1 launch-line
//! guard, driven through the **real `jigc` binary** (M8 Increment 6, T1 —
//! `implementation/roadmap.md` → inc-6 bullet 1; `design/worked-examples.md` →
//! flow 10 Half-A step 1; `increment-workflow.md` → M8 faces #4 + hardening #7).
//!
//! Where the in-crate `milestone::tests` unit proves the *core* compose
//! (`execute_milestone_core` over a fixture pack), this acceptance proves the
//! **front-door binary**: `jigc milestone execute <id>` over a real milestone with
//! N sub-tasks, asserting on the bytes the agent actually receives on stdout.
//!
//!   - **Deterministic id-sorted emit (hardening #7).** A milestone whose sub-tasks
//!     are added in NON-id order emits exactly N `` Spawn: `jigc workflow sub-task
//!     --task <id>` `` directives in **id-sorted** order. A sibling milestone with
//!     the same sub-tasks added in the **reverse** order emits the **byte-identical**
//!     `execute` stdout — the resolver sorts on resolve, not on add order. One green
//!     run over a single order would mask a completion-ordered emit; ≥2 divergent
//!     add orders with byte-identical output is the real red→green.
//!   - **The L1 guard (#4 face).** The **shipped** `adapters/claude-code.yaml` spawn
//!     template, rendered for a real `(sub-task, <id>)` and its single backticked
//!     span executed as a subprocess, resolves to the real `jigc workflow … --task`
//!     re-entry verb (composes, exit 0) — never a clap unknown-subcommand error (a
//!     template naming a nonexistent verb). The contract is the rendered bytes, never
//!     a reconstruction.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow10-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer so
/// the cascade resolves (milestone mint reads HEAD).
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

/// Stand up a fresh milestone `cache-hardening` whose sub-tasks are added in the
/// caller-supplied `intents` order (each minted `--workflow sub-task`), then return
/// the stdout bytes of `jigc milestone execute cache-hardening` — the directives the
/// agent actually receives. Each milestone lives in its own throwaway repo so add
/// order is the only variable.
fn execute_stdout_for_add_order(tag: &str, intents: &[&str]) -> String {
    let repo = TempDir::new(tag);
    let home = TempDir::new(&format!("{tag}-home"));
    init_repo(repo.path());

    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache hardening"],
        ),
        "milestone create",
    );
    for intent in intents {
        expect_ok(
            &run(
                repo.path(),
                home.path(),
                &[
                    "milestone",
                    "add-task",
                    "cache-hardening",
                    intent,
                    "--workflow",
                    "sub-task",
                ],
            ),
            "milestone add-task",
        );
    }
    let executed = run(
        repo.path(),
        home.path(),
        &["milestone", "execute", "cache-hardening"],
    );
    expect_ok(&executed, "milestone execute");
    String::from_utf8(executed.stdout).expect("utf-8 execute stdout")
}

/// **Half-A step 1, the determinism headline (hardening #7).** `jigc milestone
/// execute` over a milestone whose sub-tasks were added in NON-id order emits exactly
/// N `Spawn:` directives in **id-sorted** order — and a sibling milestone with the
/// same sub-tasks added in the **reverse** order emits the **byte-identical** stdout.
/// The assertion is on the emitted bytes, never a reconstruction.
#[test]
fn milestone_execute_emits_n_id_sorted_spawns_byte_identical_across_add_orders() {
    // Three sub-tasks whose intents slug to ids that are NOT in add order. Intents are
    // chosen so the slugged ids sort alpha < middle < zebra, but they are added in a
    // scrambled order — so an id-sorted emit cannot be an accident of add order.
    let id_alpha = "alpha-rework";
    let id_middle = "middle-rework";
    let id_zebra = "zebra-rework";
    let spawn = |id: &str| format!("Spawn: `jigc workflow sub-task --task {id}`");

    // Add order #1: zebra, alpha, middle (scrambled, not id-sorted).
    let scrambled = execute_stdout_for_add_order(
        "scrambled",
        &["zebra rework", "alpha rework", "middle rework"],
    );

    // Exactly N=3 Spawn directives, one per sub-task, naming the real re-entry verb.
    assert_eq!(
        scrambled.matches("Spawn: `jigc workflow").count(),
        3,
        "exactly one Spawn per sub-task (no extras/dupes); got:\n{scrambled}",
    );
    let at_alpha = scrambled
        .find(&spawn(id_alpha))
        .unwrap_or_else(|| panic!("alpha Spawn present; got:\n{scrambled}"));
    let at_middle = scrambled
        .find(&spawn(id_middle))
        .unwrap_or_else(|| panic!("middle Spawn present; got:\n{scrambled}"));
    let at_zebra = scrambled
        .find(&spawn(id_zebra))
        .unwrap_or_else(|| panic!("zebra Spawn present; got:\n{scrambled}"));
    assert!(
        at_alpha < at_middle && at_middle < at_zebra,
        "the Spawn directives must be id-sorted (alpha < middle < zebra), not in add \
         order; got:\n{scrambled}",
    );

    // Add order #2: the REVERSE add order. A completion/add-ordered emit would differ;
    // the id-sorted-on-resolve emit is byte-identical.
    let reversed = execute_stdout_for_add_order(
        "reversed",
        &["middle rework", "alpha rework", "zebra rework"],
    );
    assert_eq!(
        scrambled, reversed,
        "`milestone execute` stdout must be byte-identical across divergent add orders \
         (id-sorted on resolve, hardening #7)",
    );
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

/// The launch render: the lexical two-token substitution the adapter's `render_spawn`
/// performs (`{{workflow}}`/`{{task_id}}` → the concrete ids, every other byte
/// verbatim). The cli crate is a binary by invariant, so the integration test
/// reproduces this one operation rather than linking the private fn — but it renders
/// the *shipped* template bytes, so the command is composed, not authored.
fn render_spawn(template: &str, workflow: &str, task_id: &str) -> String {
    template
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

/// **Half-A step 1, the L1 guard (#4 face).** The shipped spawn template, rendered for
/// a real `(sub-task, <id>)` provisioned through the binary and its single backticked
/// span executed as a subprocess, resolves to the real `jigc workflow … --task`
/// re-entry verb (composes, exit 0) — never a clap unknown-subcommand / usage error.
/// The executed bytes are the contract: the command flows from the shipped template
/// through the render, never reconstructed in test code.
#[test]
fn the_rendered_spawn_line_resolves_to_the_real_reentry_verb() {
    let repo = TempDir::new("l1");
    let home = TempDir::new("l1-home");
    init_repo(repo.path());

    // Provision a REAL milestone sub-task whose recorded mint workflow is `sub-task`
    // (the workflow the fan-out spawn template names), so the re-entry verb composes.
    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache hardening"],
        ),
        "milestone create",
    );
    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-hardening",
                "add an LRU eviction ADR",
                "--workflow",
                "sub-task",
            ],
        ),
        "milestone add-task",
    );
    let workflow = "sub-task";
    let sub = "add-an-lru-eviction-adr";

    // Render the SHIPPED template for the REAL (workflow, sub-task-id), then lift the
    // single backticked span verbatim — the executed-bytes contract.
    let template = shipped_spawn_template();
    let rendered = render_spawn(&template, workflow, sub);
    let command = backticked_span(&rendered);

    // The rendered command names the real re-entry verb — derived from the rendered
    // bytes, not asserted against a hand-built string.
    let tokens: Vec<&str> = command.split_whitespace().collect();
    assert_eq!(
        &tokens[..3],
        &["jigc", "workflow", workflow],
        "the rendered launch command must invoke the `jigc workflow <W>` verb; got {command:?}",
    );

    // Execute the rendered command VERBATIM against the built binary (drop the leading
    // `jigc` token — the built-binary path replaces the program name).
    let executed = run(repo.path(), home.path(), &tokens[1..]);

    // It must RESOLVE to the real `Command::Workflow` re-entry verb's compose path: for
    // this real sub-task it composes the sub-task view (exit 0), carrying the
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
        "the executed launch line must compose the sub-task view (its create-gate ADR \
         affordance), proving it reached the re-entry compose, not a clap parse \
         failure; got:\n{stdout}",
    );
    // A clap unknown-subcommand / usage diagnostic would name a usage line; the real
    // verb's compose carries none.
    assert!(
        !stderr.contains("Usage:") && !stderr.to_lowercase().contains("unrecognized"),
        "the rendered command must not trip a clap parse error; stderr:\n{stderr}",
    );
}
