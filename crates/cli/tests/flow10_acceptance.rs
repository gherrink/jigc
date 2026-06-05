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
        "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# {title}\n\n## Context\n\nForces.\n\n## Decision\n\nThe ORIGINAL committed decision prose.\n\n## Consequences\n\nNone.\n"
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
    fs::create_dir_all(root.join("decisions")).expect("create decisions/");
    fs::write(
        root.join("decisions").join("eviction-policy.md"),
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
