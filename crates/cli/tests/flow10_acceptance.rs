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
//!     are added in NON-id order emits exactly N `` Spawn: `cd .jigc/worktrees/<id>
//!     && jigc workflow sub-task --task <id>` `` directives in **id-sorted** order. A
//!     sibling milestone with the same sub-tasks added in the **reverse** order emits
//!     the **byte-identical** `execute` stdout — the resolver sorts on resolve, not on
//!     add order. One green run over a single order would mask a completion-ordered
//!     emit; ≥2 divergent add orders with byte-identical output is the real red→green.
//!   - **The L1 guard (#4 face).** The **shipped** `adapters/claude-code.yaml` spawn
//!     template, rendered for a real `(sub-task, <id>)` and its single backticked
//!     `cd <worktree> && jigc workflow … --task` span executed as a subprocess into a
//!     **provisioned** worktree, resolves to the real re-entry verb (composes, exit 0)
//!     — never a clap unknown-subcommand error (a template naming a nonexistent verb).
//!     The contract is the rendered bytes, never a reconstruction.

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
            "jigc-flow10-{tag}-{}-{:?}",
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
    let spawn = |id: &str| {
        format!("Spawn: `cd .jigc/worktrees/{id} && jigc workflow sub-task --task {id}`")
    };

    // Add order #1: zebra, alpha, middle (scrambled, not id-sorted).
    let scrambled = execute_stdout_for_add_order(
        "scrambled",
        &["zebra rework", "alpha rework", "middle rework"],
    );

    // Exactly N=3 Spawn directives, one per sub-task, naming the real re-entry verb.
    assert_eq!(
        scrambled.matches("Spawn: `cd .jigc/worktrees/").count(),
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

/// The launch render: the lexical three-token substitution the adapter's `render_spawn`
/// performs (`{{worktree}}`/`{{workflow}}`/`{{task_id}}` → the concrete values, every
/// other byte verbatim; `{{worktree}}` → `.jigc/worktrees/<task_id>`). The cli crate is
/// a binary by invariant, so the integration test reproduces this one operation rather
/// than linking the private fn — but it renders the *shipped* template bytes, so the
/// command is composed, not authored.
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
/// token in the launch span resolves to the binary under test. The span carries a
/// `cd <worktree> && jigc workflow …` chain (M31 WF4), so it must run through a shell,
/// not as a single argv — the executed-bytes contract for the worktree binding.
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

/// **Half-A step 1, the L1 guard (#4 face).** The shipped spawn template, rendered for
/// a real `(sub-task, <id>)` and its single backticked `cd <worktree> && jigc workflow
/// … --task` span executed as a subprocess into a **provisioned** worktree, resolves to
/// the real re-entry verb (composes, exit 0) — never a clap unknown-subcommand / usage
/// error. The executed bytes are the contract: the command flows from the shipped
/// template through the render, never reconstructed in test code.
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
    // Provision the worktrees (T1) — the `cd <worktree>` half of the span needs the
    // detached checkout to exist before the launch span runs.
    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &["milestone", "provision", "cache-hardening"],
        ),
        "milestone provision",
    );
    let workflow = "sub-task";
    let sub = "add-an-lru-eviction-adr";

    // Render the SHIPPED template for the REAL (workflow, sub-task-id), then lift the
    // single backticked span verbatim — the executed-bytes contract.
    let template = shipped_spawn_template();
    let rendered = render_spawn(&template, workflow, sub);
    let command = backticked_span(&rendered);

    // The rendered command directs the sub-agent into its worktree, then names the real
    // re-entry verb — derived from the rendered bytes, not a hand-built string.
    assert_eq!(
        command,
        format!("cd .jigc/worktrees/{sub} && jigc workflow {workflow} --task {sub}"),
        "the rendered launch span must `cd` into the worktree then invoke the verb",
    );

    // Execute the rendered span VERBATIM via a shell (cwd = repo, the built `jigc` on
    // PATH): `cd .jigc/worktrees/<sub>` enters the provisioned worktree, then
    // `jigc workflow …` re-enters from inside it (the WF3 jigc_home resolver finds the
    // main checkout's `.jigc/`).
    let executed = run_shell(repo.path(), home.path(), &command);

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

/// **T2 hardening #4 — the emitted provision `Run:` line reaches the real verb.** The
/// `jigc milestone execute` view's worktree-provisioning `Run:` line, lifted verbatim
/// from the composed bytes the agent runs, executes against the built binary and reaches
/// the T1 `milestone provision` verb (exit 0) — never a clap unknown-subcommand error. The
/// agent fills the one `<MILESTONE_ID>` marker (its documented run-time substitution point,
/// exactly as the spawn line's `task_id`); every other byte runs verbatim, so a regression
/// in the catalog entry or the verb wiring fails here rather than being masked.
#[test]
fn the_emitted_provision_run_line_reaches_the_real_provision_verb() {
    let repo = TempDir::new("provision-run");
    let home = TempDir::new("provision-run-home");
    init_repo(repo.path());

    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache hardening"],
        ),
        "milestone create",
    );
    for intent in ["add an LRU eviction ADR", "tune the cache size"] {
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
    let view = String::from_utf8(executed.stdout).expect("utf-8 execute stdout");

    // Lift the provision `Run:` line's backticked command VERBATIM from the composed view —
    // the bytes the agent runs, not a reconstruction.
    let command = view
        .lines()
        .filter_map(|l| l.strip_prefix("Run: `").and_then(|r| r.strip_suffix('`')))
        .find(|c| c.starts_with("jigc milestone provision"))
        .unwrap_or_else(|| {
            panic!("the composed view must carry the provision Run line; got:\n{view}")
        })
        .to_owned();

    // The line carries exactly the one `<MILESTONE_ID>` agent fill marker (the run-time
    // substitution point) and the real `milestone provision` verb tokens.
    let tokens: Vec<&str> = command.split_whitespace().collect();
    assert_eq!(
        &tokens[..3],
        &["jigc", "milestone", "provision"],
        "the provision Run line must invoke the `jigc milestone provision` verb; got {command:?}",
    );
    assert!(
        command.contains("<MILESTONE_ID>"),
        "the provision Run line must leave the milestone id as the agent fill marker; got {command:?}",
    );

    // Fill the one agent marker (what the agent does at run-time), then execute the line
    // VERBATIM against the built binary (drop the leading `jigc` token — the built-binary
    // path replaces the program name).
    let filled: Vec<String> = tokens
        .iter()
        .map(|t| {
            if *t == "<MILESTONE_ID>" {
                "cache-hardening".to_owned()
            } else {
                (*t).to_owned()
            }
        })
        .collect();
    let arg_refs: Vec<&str> = filled[1..].iter().map(String::as_str).collect();
    let provisioned = run(repo.path(), home.path(), &arg_refs);

    // It must REACH the real `milestone provision` verb and exit 0 — never a clap
    // unknown-subcommand / usage error (a Run line naming a nonexistent verb).
    let stderr = String::from_utf8_lossy(&provisioned.stderr);
    assert!(
        provisioned.status.success(),
        "the emitted provision Run line must reach the real provision verb and exit 0, not a \
         clap unknown-subcommand error; stderr:\n{stderr}",
    );
    assert!(
        !stderr.contains("Usage:") && !stderr.to_lowercase().contains("unrecognized"),
        "the emitted provision Run line must not trip a clap parse error; stderr:\n{stderr}",
    );
    let stdout = String::from_utf8(provisioned.stdout).expect("utf-8 provision stdout");
    assert!(
        stdout.contains("provisioned"),
        "the executed Run line must reach the provision verb (its summary), proving it is not a \
         clap parse failure; got:\n{stdout}",
    );
}

// ---------------------------------------------------------------------------
// Half-A step 2 — the real write→join seam (`implementation/roadmap.md` → inc-6
// bullet 1; `design/worked-examples.md` → flow 10 Half-A step 2;
// `increment-workflow.md` → hardening #4).
//
// Where flow-9's `flow9_milestone_join.rs` hand-staged `tasks/<sub>/docs/<addr>.md`
// + `provenance.json` directly, flow 10 closes the **whole control plane**: each
// sub-area is provisioned and populated by invoking `jigc workflow sub-task --task
// <id>` as a **separate binary process** — *exactly* what a real fanned sub-agent
// runs (`worked-examples.md` → flow 10: "the 'sub-agents' are the test invoking the
// CLI N times as separate processes"). Then real `jigc doc create adr` + `set-slot`
// writes (incl. ≥1 `edited-from-base` via copy-on-first-touch) produce the
// `docs/<addr>.md` + `provenance.json` the M7 join consumes, and `jigc milestone
// join` merges those real-written inputs clean (exit 0, both addresses in the
// overlay). No hand-staging — the bytes/manifest the join reads were produced by the
// binary, so a regression in copy-on-first-touch, the `--task` selector, or the
// provenance record fails these assertions rather than being masked by a fixture.

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

/// Run `jigc doc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
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

/// A canonical committed ADR (the `write::render` form), so a first-touch copy-in is
/// byte-stable. Committed at `decisions/<slug>.md` — the base copy-on-first-touch
/// pulls in for the `edited-from-base` path.
fn committed_adr(title: &str) -> String {
    format!(
        "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nThe ORIGINAL committed decision prose.\n\n## Consequences\n\nNone.\n"
    )
}

/// Initialize a git repo with one commit + the `.jigc/config/` project layer + a
/// committed ADR at `decisions/eviction-policy.md` — the base the `edited-from-base`
/// sub-area edits via copy-on-first-touch.
fn init_repo_with_base_adr(root: &Path) {
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
    fs::create_dir_all(root.join("docs").join("decisions")).expect("create docs/decisions/");
    fs::write(
        root.join("docs")
            .join("decisions")
            .join("eviction-policy.md"),
        committed_adr("Eviction policy"),
    )
    .expect("write committed adr");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// A sub-task's `docs/` working area: `.jigc/tasks/<sub>/docs/`.
fn docs_area(repo: &Path, sub: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(sub).join("docs")
}

/// **Half-A step 2 — the real write→join seam.** A `cache-hardening` milestone with
/// two `sub-task` sub-areas, each provisioned by invoking `jigc workflow sub-task
/// --task <id>` as a **separate binary process** (what a real fanned sub-agent runs),
/// then populated through the real `jigc doc create adr` + `set-slot --task` verbs:
/// subA `created`s a fresh ADR; subB `edited-from-base` the committed
/// `adr:eviction-policy` (copy-on-first-touch). Their slugs are distinct, so `jigc
/// milestone join` consumes the real-written `docs/<addr>.md` + `provenance.json` of
/// each area and reports a clean merge (exit 0, both addresses in the overlay) — the
/// flow-9 hand-staging replaced by the genuine N-process write path.
#[test]
fn real_n_process_writes_feed_a_clean_milestone_join() {
    let repo = TempDir::new("seam");
    let home = TempDir::new("seam-home");
    init_repo_with_base_adr(repo.path());

    expect_ok(
        &run(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache hardening"],
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
                    "cache-hardening",
                    intent,
                    "--workflow",
                    "sub-task",
                ],
            ),
            "milestone add-task",
        );
    }

    // Each `jigc workflow sub-task --task <sub>` is a SEPARATE binary process — exactly
    // the re-entry verb a real fanned sub-agent runs — and provisions that sub-area's
    // write-ready commit doc on first entry. No hand-staging.
    for sub in [&sub_a, &sub_b] {
        expect_ok(
            &run(
                repo.path(),
                home.path(),
                &["workflow", "sub-task", "--task", sub],
            ),
            "fanned sub-agent re-entry provisions the write-ready sub-area",
        );
    }

    // subA: a `created` ADR through the front door (the `sub-task` create-gate's
    // `{type: adr}`), then a real slot write — records `created` provenance.
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
    // `edited-from-base` (the ≥1 edited-from-base the done-criterion requires).
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

    // Sanity — these are REAL N-process writes, not hand-staged: the bodies + manifests
    // the join reads were produced by the binary. subA's manifest says `created`;
    // subB's says `edited-from-base`; subB's body carries the copied-in committed slice.
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
    // disjoint slugs are both in the overlay, no blocking finding, exit 0 — and it
    // commits nothing (the join only reports; finalize materializes).
    let before_head = git(repo.path(), &["rev-parse", "HEAD"]);
    let joined = run(
        repo.path(),
        home.path(),
        &["milestone", "join", "cache-hardening"],
    );
    expect_ok(
        &joined,
        "the join over real N-process-written disjoint areas must merge clean",
    );
    let stdout = String::from_utf8_lossy(&joined.stdout);
    assert!(
        stdout.contains("adr:cache-strategy") && stdout.contains("adr:eviction-policy"),
        "the clean merge must report both real-written addresses in the overlay; got:\n{stdout}",
    );
    assert_eq!(
        git(repo.path(), &["rev-parse", "HEAD"]),
        before_head,
        "the join only reports the overlay; it commits nothing",
    );
}

// ---------------------------------------------------------------------------
// Half-A step 3 — byte-identical finalize across ≥2 divergent feed orders, the
// HEADLINE (`implementation/roadmap.md` → inc-6 bullet 1; `design/worked-examples.md`
// → flow 10 Half-A step 3; `increment-workflow.md` → A second principle (M7) /
// hardening #7).
//
// Where flow-9's `flow9_milestone_join.rs` proved the determinism headline over a
// HAND-STAGED fixture area set, flow 10 step 3 proves the **same byte-identity over
// real-binary-produced inputs**: the IDENTICAL populated milestone — seeded through
// BOTH origination paths (`add-from-spec` AND `add-task`) and populated by the real
// `jigc workflow sub-task --task <id>` provision + `jigc doc create adr` / `set-slot`
// write verbs of step 2 — is driven through the full `create`/`add-from-spec` →
// `jigc milestone execute` → `jigc milestone finalize` arc in independent throwaway
// repos under deliberately divergent recorded feed orders (id order AND its reverse,
// reverse MANDATORY, plus a third shuffled), and the committed commit MESSAGE and
// committed TREE hash are asserted byte-identical across ALL orders.
//
// Reverse order is mandatory: an id-ordered feed where completion order trivially
// equals id order would pass even a completion-ordered (broken) merge. The fixture
// forces genuine overlap — two sub-tasks each `create` the SAME slug
// `adr:cache-strategy` (distinguishable only by their `#decision` slot prose, "eager"
// vs "lazy", since `id-from: title` forces a shared H1), so which body lands at the
// bare slug vs the `-2` suffix is OBSERVABLE in the committed tree. A suffix keyed on
// feed order instead of task id would swap which body lands where and diverge the tree
// hash under the reverse / shuffled orders.
//
// THIS PRODUCES THE GOLDEN TREE-HASH the Half-B genuine-spawn audit (T4) must match,
// so every populating operation is one a real fanned sub-agent performs through the
// CLI (provision + `create adr` + `set-slot`) — no hand-staging, no hand-injected
// field a genuine spawn could not reproduce.

/// A committed `spec` with one repeatable `criterion` — the `add-from-spec` seed
/// substrate so the arc exercises BOTH origination paths feeding the same finalize.
/// Its single criterion text "Document the cache strategy" slugs to the sub-task id
/// `document-the-cache-strategy` (the higher-id colliding `created` instance).
const SEED_SPEC: &str = "\
# Cache hardening plan

## Goal

Harden the cache layer.

## Context

The cache strategy needs documenting alongside the eviction work.

## Criteria

### Document the cache strategy  {#document-strategy}

The cache strategy is recorded as an ADR.
";

/// Commit `SEED_SPEC` at `specs/<slug>.md` so `add-from-spec` reads genuinely
/// committed state, then re-establish the `.jigc/config/` project layer the commit
/// does not track.
fn commit_spec(repo: &Path, slug: &str) {
    let specs = repo.join("docs").join("specs");
    fs::create_dir_all(&specs).expect("mk docs/specs/");
    fs::write(specs.join(format!("{slug}.md")), SEED_SPEC).expect("write spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "add spec"]);
}

/// Overwrite the milestone's recorded `tasks.json` with `order` (the audit-trail feed
/// order) — the deliberate divergence the headline drives. The join must enumerate
/// id-sorted regardless of this recorded order, so re-recording it in id / reverse /
/// shuffled orders must produce byte-identical committed state. The id SET is fixed;
/// only the recorded order differs (the `flow9_milestone_join.rs` idiom, now over
/// real-write inputs).
fn rewrite_task_order(repo: &Path, milestone_id: &str, order: &[&str]) {
    let path = repo
        .join(".jigc")
        .join("milestones")
        .join(milestone_id)
        .join("tasks.json");
    assert!(path.is_file(), "milestone tasks.json must exist to reorder");
    let value = serde_json::json!({ "tasks": order });
    let mut bytes = serde_json::to_string_pretty(&value).expect("serialize task list");
    bytes.push('\n');
    fs::write(&path, bytes).expect("rewrite tasks.json");
}

/// HEAD's full commit message (`git log -1 --format=%B`) — the committed MESSAGE half
/// of the byte-identity assertion, read verbatim off the landed commit.
fn head_message(repo: &Path) -> String {
    git(repo, &["log", "-1", "--format=%B"])
}

/// HEAD's committed tree hash (`git rev-parse HEAD^{tree}`) — the committed TREE half
/// of the byte-identity assertion (and the golden Half-B must match). Two commits with
/// the same tree hash have byte-identical committed content (every promoted doc body +
/// path), independent of author/date.
fn head_tree(repo: &Path) -> String {
    git(repo, &["rev-parse", "HEAD^{tree}"]).trim().to_string()
}

/// Provision a milestone sub-area through the real re-entry verb, then `create` an ADR
/// at `adr:<slug>` (from `title`) and fill its three prose slots — every operation a
/// genuine fanned sub-agent performs through the CLI. `decision` is the distinguishing
/// slot the suffix-by-task-id proof reads back off the committed tree.
fn populate_created_adr(
    repo: &Path,
    home: &Path,
    sub: &str,
    title: &str,
    slug: &str,
    decision: &[u8],
) {
    expect_ok(
        &run(repo, home, &["workflow", "sub-task", "--task", sub]),
        "fanned sub-agent re-entry provisions the write-ready sub-area",
    );
    expect_ok(
        &run_doc(
            repo,
            home,
            &["create", "adr", "--title", title, "--task", sub],
            None,
        ),
        "doc create adr in the sub-area",
    );
    let set_slot = |section: &str, prose: &[u8]| {
        expect_ok(
            &run_doc(
                repo,
                home,
                &[
                    "set-slot",
                    &format!("adr:{slug}#{section}"),
                    "--from-file",
                    "-",
                    "--task",
                    sub,
                ],
                Some(prose),
            ),
            "set-slot on the created ADR",
        );
    };
    set_slot("context", b"Forces around caching.\n");
    set_slot("decision", decision);
    set_slot("consequences", b"Tradeoffs.\n");
}

/// Build the **identical** populated flow-10 fixture in `repo` via the REAL write path
/// (no hand-staging), record the milestone's `tasks.json` in `feed_order`, then run the
/// full `create`/`add-from-spec` → `execute` → `finalize` arc, returning the landed
/// commit's `(message, tree-hash)`.
///
/// The fixture seeds via BOTH origination paths: an `add-from-spec` seed whose single
/// criterion mints `document-the-cache-strategy`, and two `add-task`s
/// (`add-a-cache-strategy-adr`, `add-an-eviction-adr`). Both `add-a-cache-strategy-adr`
/// and `document-the-cache-strategy` `create` the SAME slug `adr:cache-strategy` —
/// distinguishable only by their `#decision` slot ("eager" vs "lazy"). id-sorted,
/// `add-a-cache-strategy-adr` < `document-the-cache-strategy`, so the former keeps the
/// bare slug (the "eager" body) and the latter takes `-2` (the "lazy" body); a disjoint
/// `add-an-eviction-adr` `create`s `adr:eviction-policy`.
fn build_and_finalize(repo: &Path, home: &Path, feed_order: &[&str]) -> (String, String) {
    init_repo(repo);
    commit_spec(repo, "cache-hardening-plan");

    expect_ok(
        &run(repo, home, &["milestone", "create", "Cache hardening"]),
        "milestone create",
    );
    // Origination path 1 — `add-from-spec` seeds `document-the-cache-strategy`.
    expect_ok(
        &run(
            repo,
            home,
            &[
                "milestone",
                "add-from-spec",
                "cache-hardening",
                "spec:cache-hardening-plan",
            ],
        ),
        "milestone add-from-spec",
    );
    // Origination path 2 — `add-task` for the colliding ADR + a disjoint sub-task,
    // added in NON-id order so the recorded insertion order already diverges from id
    // order before the explicit `tasks.json` rewrite below.
    for intent in ["Add a cache-strategy ADR", "Add an eviction ADR"] {
        expect_ok(
            &run(
                repo,
                home,
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

    // Populate the sub-areas through the REAL write path. The two colliding
    // `adr:cache-strategy` carry distinguishable `#decision` prose ("eager" vs "lazy"),
    // so which lands at the bare slug vs `-2` is observable in the committed tree.
    populate_created_adr(
        repo,
        home,
        "add-a-cache-strategy-adr",
        "Cache strategy",
        "cache-strategy",
        b"Use eager caching.\n",
    );
    populate_created_adr(
        repo,
        home,
        "document-the-cache-strategy",
        "Cache strategy",
        "cache-strategy",
        b"Use lazy caching.\n",
    );
    populate_created_adr(
        repo,
        home,
        "add-an-eviction-adr",
        "Eviction policy",
        "eviction-policy",
        b"Evict the least-recently-used entry.\n",
    );

    // The deliberate feed-order divergence (the audit-trail order). The id set is fixed;
    // only the recorded order differs across repos.
    rewrite_task_order(repo, "cache-hardening", feed_order);

    // The full arc passes through `execute` (the fan-out emit) before `finalize` (the
    // join + commit boundary).
    expect_ok(
        &run(repo, home, &["milestone", "execute", "cache-hardening"]),
        "milestone execute",
    );
    expect_ok(
        &run(repo, home, &["milestone", "finalize", "cache-hardening"]),
        "milestone finalize",
    );

    (head_message(repo), head_tree(repo))
}

/// **Flow 10 Half-A step 3 — the HEADLINE.** The IDENTICAL real-binary-produced fixture
/// is driven through the full `create`/`add-from-spec` → `execute` → `finalize` arc in
/// three independent throwaway repos under three divergent recorded feed orders (id,
/// reverse, seed-shuffled); the committed MESSAGE and TREE hash are byte-identical
/// across all three. A completion-ordered / broken merge — one that let the recorded
/// feed order reach the suffix assignment or the body bytes — would diverge under the
/// reverse / shuffled orders and fail this assertion. THIS TREE HASH IS THE GOLDEN the
/// Half-B genuine-spawn audit must match.
#[test]
fn flow10_finalize_is_byte_identical_across_divergent_feed_orders() {
    // The fixed id set, in canonical id order. The three feed orders below are all
    // permutations of exactly this set.
    let id_order = [
        "add-a-cache-strategy-adr",
        "add-an-eviction-adr",
        "document-the-cache-strategy",
    ];
    let reverse_order = [
        "document-the-cache-strategy",
        "add-an-eviction-adr",
        "add-a-cache-strategy-adr",
    ];
    // A seed-shuffled order distinct from both id and reverse (a third permutation).
    let shuffled_order = [
        "add-an-eviction-adr",
        "document-the-cache-strategy",
        "add-a-cache-strategy-adr",
    ];

    // Sanity: the three feed orders are genuinely divergent (a re-execution under a
    // single order would not exercise hardening #7).
    assert_ne!(id_order, reverse_order, "id vs reverse must diverge");
    assert_ne!(id_order, shuffled_order, "id vs shuffled must diverge");
    assert_ne!(
        reverse_order, shuffled_order,
        "reverse vs shuffled must diverge"
    );

    let home = TempDir::new("step3-home");

    let repo_id = TempDir::new("step3-id");
    let (msg_id, tree_id) = build_and_finalize(repo_id.path(), home.path(), &id_order);
    let repo_rev = TempDir::new("step3-rev");
    let (msg_rev, tree_rev) = build_and_finalize(repo_rev.path(), home.path(), &reverse_order);
    let repo_shuf = TempDir::new("step3-shuf");
    let (msg_shuf, tree_shuf) = build_and_finalize(repo_shuf.path(), home.path(), &shuffled_order);

    // ---- The headline: committed MESSAGE is byte-identical across feed orders. ----
    assert_eq!(
        msg_id, msg_rev,
        "the committed message must be byte-identical under id vs reverse feed order;\n\
         id:\n{msg_id}\nreverse:\n{msg_rev}",
    );
    assert_eq!(
        msg_id, msg_shuf,
        "the committed message must be byte-identical under id vs shuffled feed order;\n\
         id:\n{msg_id}\nshuffled:\n{msg_shuf}",
    );
    // The message is the CLI-synthesized structural projection — subject + id-ordered body.
    assert!(
        msg_id.contains("Finalize milestone cache-hardening (3 sub-tasks)"),
        "the committed message must be the synthesized projection; got:\n{msg_id}",
    );

    // ---- The headline: committed TREE hash is byte-identical across feed orders. ----
    // Equal tree hashes ⇒ every promoted doc body + path is byte-identical, so the
    // suffix assignment and the merge are a pure function of the area SET, never the
    // recorded feed order. THIS IS THE GOLDEN the Half-B audit asserts its commit against.
    assert_eq!(
        tree_id, tree_rev,
        "the committed tree must be byte-identical under id vs reverse feed order \
         ({tree_id} vs {tree_rev})",
    );
    assert_eq!(
        tree_id, tree_shuf,
        "the committed tree must be byte-identical under id vs shuffled feed order \
         ({tree_id} vs {tree_shuf})",
    );

    // ---- Supporting: suffix-by-task-id, read off the committed tree. ----
    // The tree is identical across all three repos, so reading any one reads the shared
    // committed bytes.
    let decisions = repo_id.path().join("docs").join("decisions");
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            decisions.join(format!("{slug}.md")).is_file(),
            "the promoted `{slug}` doc must land at docs/decisions/{slug}.md",
        );
    }
    // The lower task id (`add-a-cache-strategy-adr`, the "eager" body) kept the BARE
    // slug. Asserting the distinguishing decision prose proves suffix-BY-TASK-ID (not
    // merely "a suffix happened"): a completion-ordered suffix would land "lazy" here.
    let bare = fs::read_to_string(decisions.join("cache-strategy.md")).expect("read bare");
    assert!(
        bare.contains("Use eager caching."),
        "the lower task id's body must keep the bare slug; got:\n{bare}",
    );
    // The higher task id (`document-the-cache-strategy`, the "lazy" body) took `-2`.
    let suffixed =
        fs::read_to_string(decisions.join("cache-strategy-2.md")).expect("read suffixed");
    assert!(
        suffixed.contains("Use lazy caching."),
        "the higher task id's body must take the `-2` suffix; got:\n{suffixed}",
    );

    // The promoted docs are genuinely committed (tracked in HEAD's tree).
    let tracked = git(repo_id.path(), &["ls-files", "docs/decisions/"]);
    for slug in ["cache-strategy", "cache-strategy-2", "eviction-policy"] {
        assert!(
            tracked.contains(&format!("docs/decisions/{slug}.md")),
            "docs/decisions/{slug}.md must be committed (tracked); got:\n{tracked}",
        );
    }
}
