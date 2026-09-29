//! T2 — the methodology pack's `dev-task` workflow composes the flat prose spine,
//! proven through the real binary on a throwaway repo (M12 Inc 1; `design/
//! self-hosting.md` → The dev-workflow sort, Verified friction; `worked-examples.md`
//! → flow 15 setup block).
//!
//! T1 shipped the pack *substrate* (cascade config + vendored commit schema) but no
//! `dev-task` workflow. T2 ships `workflows/dev-task.yaml` + the four prose steps
//! (`scope`/`implement`/`gate`/`finalize`) and proves, end-to-end through the binary:
//!
//!   (a) `JIGC_PACK_DIR=<methodology> jigc setup` then `jigc start "<intent>"` exits
//!       0 and composes the flat spine — the four step bodies in order
//!       scope → implement → gate → finalize.
//!   (b) `{{task.intent}}` resolves to the literal intent *on its own line* (the
//!       lone-line discipline — assert the resolved string appears, NOT the literal
//!       `{{task.intent}}`).
//!   (c) The emitted-bytes contract (hardening #4): the four commit-fill `Run:` lines
//!       (`set-field type`, `set-field scope`, `set-slot summary`, `set-slot body`)
//!       and the `finalize-task` `Run:` line render **verbatim with addresses
//!       resolved** to the minted task's slug — asserted as the emitted lines, never
//!       reconstructed.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! methodology pack from `CARGO_MANIFEST_DIR/packs/methodology`, and a
//! self-cleaning `TempDir` keeps the test off the developer's real repo / files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-methodology-compose-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
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
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack_dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .output()
        .expect("run the jigc binary")
}

/// Like [`run_jigc`] but piping `stdin` (the `set-slot --from-file -` path).
fn run_jigc_stdin(
    repo: &Path,
    home: &Path,
    pack_dir: &Path,
    args: &[&str],
    stdin: &str,
) -> std::process::Output {
    use std::io::Write;
    let mut child = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin.as_bytes())
        .expect("write stdin");
    child.wait_with_output().expect("wait for the jigc binary")
}

/// Extract the backtick-quoted body of the *unique* `Run:` line containing `needle`,
/// so the assertion runs over the **emitted bytes** the agent would copy — never a
/// reconstruction. Panics with the full stdout if absent or ambiguous.
/// Split an emitted command into argv the way a shell would — honouring the single
/// quotes `engine::compose::shell_quote` puts around a multi-word argument, so a `Run:`
/// line is executed as the **emitted bytes** rather than a whitespace-split
/// approximation of them. (Kept local: this suite's group root carries no `support`
/// module.)
fn shell_split(cmd: &str) -> Vec<String> {
    let mut argv = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut started = false;
    for ch in cmd.chars() {
        match ch {
            '\'' => {
                in_quotes = !in_quotes;
                started = true;
            }
            c if c.is_whitespace() && !in_quotes => {
                if started {
                    argv.push(std::mem::take(&mut current));
                    started = false;
                }
            }
            c => {
                current.push(c);
                started = true;
            }
        }
    }
    if started {
        argv.push(current);
    }
    argv
}

fn emitted_run_line<'a>(stdout: &'a str, needle: &str) -> &'a str {
    let mut matches = stdout.lines().filter(|l| {
        let t = l.trim_start();
        t.starts_with("Run:") && t.contains(needle)
    });
    let line = matches
        .next()
        .unwrap_or_else(|| panic!("no `Run:` line containing {needle:?}; got:\n{stdout}"));
    assert!(
        matches.next().is_none(),
        "more than one `Run:` line contains {needle:?}; got:\n{stdout}",
    );
    // The address resolves inside the backtick-quoted command.
    line.split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("`Run:` line not backtick-quoted: {line:?}"))
}

#[test]
fn dev_task_composes_the_flat_spine_resolved_intent_and_verbatim_fill_lines() {
    let repo = TempDir::new("compose");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // The intent → slug derivation is `add rate limiter` → `add-rate-limiter`, so the
    // minted commit doc is `commit:add-rate-limiter` and the task id is
    // `add-rate-limiter` (the same slug the embedded single-task uses in start_compose).
    let intent = "add rate limiter";
    let out = run_jigc(repo.path(), home.path(), &pack, &["start", intent]);
    assert!(
        out.status.success(),
        "`jigc start \"{intent}\"` over the methodology pack must compose dev-task and exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // (b) `{{task.intent}}` resolves to the literal intent on its own line — the
    // resolved string appears, the literal placeholder never does.
    assert!(
        stdout.lines().any(|l| l.trim() == intent),
        "the composed dev-task must resolve `{{{{task.intent}}}}` to the intent on its own \
         line; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("{{task.intent}}") && !stdout.contains("{{ task.intent }}"),
        "the literal `{{{{task.intent}}}}` placeholder must NOT survive into the composed \
         output; got:\n{stdout}",
    );

    // (a) The flat spine composes the four step bodies in order
    // scope → implement → gate → finalize. Each step contributes a distinctive
    // prose marker; their stdout offsets must be strictly increasing.
    let pos = |needle: &str| {
        stdout
            .find(needle)
            .unwrap_or_else(|| panic!("composed spine missing {needle:?}; got:\n{stdout}"))
    };
    let scope_at = pos("done-criterion");
    let implement_at = pos("failing test");
    let gate_at = pos("test, lint, and build gate");
    let finalize_at = pos("one logical commit");
    assert!(
        scope_at < implement_at && implement_at < gate_at && gate_at < finalize_at,
        "the spine must order scope({scope_at}) < implement({implement_at}) < \
         gate({gate_at}) < finalize({finalize_at}); got:\n{stdout}",
    );

    // (c) Emitted-bytes contract: the four commit-fill Run lines + the finalize-task
    // line render verbatim with addresses resolved to the `add-rate-limiter` slug.
    // Asserted on the EMITTED bytes (extracted from stdout), never reconstructed.
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:add-rate-limiter#type"),
        "jigc doc set-field commit:add-rate-limiter#type --value <TYPE> --task add-rate-limiter",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:add-rate-limiter#scope"),
        "jigc doc set-field commit:add-rate-limiter#scope --value <SCOPE> --task add-rate-limiter",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:add-rate-limiter#summary"),
        "jigc doc set-slot commit:add-rate-limiter#summary --from-file - --task add-rate-limiter",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:add-rate-limiter#body"),
        "jigc doc set-slot commit:add-rate-limiter#body --from-file - --task add-rate-limiter",
    );
    assert_eq!(
        emitted_run_line(&stdout, "task finalize"),
        "jigc task finalize add-rate-limiter",
    );

    // The fill lines must precede the finalize line (the agent fills, then renders).
    assert!(
        pos("set-field commit:add-rate-limiter#type") < pos("task finalize add-rate-limiter"),
        "the four commit-fill Run lines must precede `jigc task finalize`; got:\n{stdout}",
    );
}

/// M16 Increment 4 / T2 — the methodology `planning` workflow composes the
/// planning spine through the real binary: `jigc start --workflow planning
/// "<milestone>"` mints a task **and** emits the **Settle checkpoint**
/// (`Checkpoint: settle`) plus the create / add-item / set-slot / set-field
/// authoring guidance for the three running doctypes (roadmap / deferral-ledger /
/// decisions-log).
///
/// This is the compose-time contract (T4 *runs* the emitted commands end-to-end;
/// this proves they are **emitted**, with the singleton create command-refs
/// rendered verbatim on the EMITTED bytes — never reconstructed).
#[test]
fn planning_composes_the_settle_checkpoint_and_the_three_doctype_authoring_lines() {
    let repo = TempDir::new("planning");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // `planning` is `creates-task: true` (the sub-task precedent) — invoked directly
    // off the router by `--workflow planning`, it mints a task and composes the spine.
    // Intent "M99" → slug "m99" (slugify lowercases).
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow planning \"M99\"` over the methodology pack must compose the \
         planning spine and exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `creates-task: true` → a task dir is minted at `.jigc/tasks/m99/`.
    let task_dir = repo.path().join(".jigc").join("tasks").join("m99");
    assert!(
        task_dir.is_dir(),
        "`--workflow planning` must mint the task dir at {task_dir:?}; got stdout:\n{stdout}",
    );

    // No `{{ … }}` placeholder survives a clean compose (every include / data-value /
    // command-ref resolved).
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "no `{{{{ … }}}}` placeholder may survive the planning compose; got:\n{stdout}",
    );

    // The Settle checkpoint emits `Checkpoint: settle` at its own line, the bare slug
    // (no backticks) — the M15 checkpoint primitive, the human gate of the loop.
    assert!(
        stdout.lines().any(|l| l == "Checkpoint: settle"),
        "the Settle step must compose `Checkpoint: settle` at the left margin; got:\n{stdout}",
    );
    assert!(
        !stdout.contains("Checkpoint: `settle`"),
        "the `settle` reason slug must be bare (no backticks); got:\n{stdout}",
    );

    let pos = |needle: &str| {
        stdout
            .find(needle)
            .unwrap_or_else(|| panic!("composed planning spine missing {needle:?}; got:\n{stdout}"))
    };

    // The five phases compose in loop order: scope → detect-gaps → settle → review →
    // decompose. Each contributes a distinctive prose marker; offsets strictly increase.
    let scope_at = pos("Scope the milestone");
    let gaps_at = pos("Detect the gaps");
    let settle_at = pos("Checkpoint: settle");
    let review_at = pos("Review what Settle produced");
    let decompose_at = pos("Decompose the milestone into ordered");
    assert!(
        scope_at < gaps_at
            && gaps_at < settle_at
            && settle_at < review_at
            && review_at < decompose_at,
        "the planning spine must order scope({scope_at}) < detect-gaps({gaps_at}) < \
         settle({settle_at}) < review({review_at}) < decompose({decompose_at}); got:\n{stdout}",
    );

    // The three singleton `create` command-refs render verbatim on the EMITTED bytes —
    // singletons fix the slug to the type id, so `--title` is a fixed literal (ignored).
    assert_eq!(
        emitted_run_line(&stdout, "doc create roadmap"),
        "jigc doc create roadmap --title Roadmap --task m99",
    );
    assert_eq!(
        emitted_run_line(&stdout, "doc create deferral-ledger"),
        "jigc doc create deferral-ledger --title 'Deferral Ledger' --task m99",
    );
    assert_eq!(
        emitted_run_line(&stdout, "doc create decisions-log"),
        "jigc doc create decisions-log --title 'Decisions Log' --task m99",
    );

    // The per-entry authoring lines (add-item + set-slot/set-field on the singleton's
    // item-leaf addresses) appear for each doctype — literal guidance, the item id is
    // runtime-minted so the `<id>` stays an authoring placeholder (the author-arch-doc
    // precedent). Assert one address from each doctype's repeatable section.
    for needle in [
        "jigc doc add-item roadmap:roadmap#milestones",
        "jigc doc set-slot roadmap:roadmap#milestones/<id>/proves",
        "jigc doc set-slot roadmap:roadmap#milestones/<id>/decomposition",
        "jigc doc add-item deferral-ledger:deferral-ledger#entries",
        "jigc doc set-field deferral-ledger:deferral-ledger#entries/<id>/kind",
        "jigc doc set-field deferral-ledger:deferral-ledger#entries/<id>/trigger",
        "jigc doc set-slot deferral-ledger:deferral-ledger#entries/<id>/body",
        "jigc doc add-item decisions-log:decisions-log#entries",
        "jigc doc set-slot decisions-log:decisions-log#entries/<id>/why",
    ] {
        assert!(
            stdout.contains(needle),
            "the planning spine must emit the authoring line `{needle}`; got:\n{stdout}",
        );
    }

    // The doc authoring follows Settle (the running docs record Settle's outputs).
    assert!(
        settle_at < pos("jigc doc create roadmap"),
        "the doctype authoring must follow the Settle checkpoint; got:\n{stdout}",
    );
}

/// M17 Increment 4 / T1 — the composed `planning` workflow carries its finalize
/// tail (the stranded-docs gap): the same `{{ include: step:finalize }}` that
/// completion.yaml/dev-task.yaml carry, which brings the commit-doc authoring
/// guidance with it. Through the real binary, the composed planning spine must
/// emit the four `set-commit-*` `Run:` lines (type/scope/summary/body, addresses
/// resolved to the minted task's commit doc) **and** the `jigc task finalize
/// <minted-id>` line, all AFTER the decompose/author steps — asserted on the
/// EMITTED bytes, never reconstructed.
#[test]
fn planning_composes_the_commit_fill_and_finalize_tail() {
    let repo = TempDir::new("planning-finalize");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // Intent "M99" → slug "m99" (slugify lowercases): the minted task id is `m99`
    // and the minted commit doc is `commit:m99`.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow planning \"M99\"` must compose the planning spine and exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The four commit-fill Run lines + the finalize-task line render verbatim with
    // addresses resolved to the minted `m99` slug (the emitted-bytes contract).
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:m99#type"),
        "jigc doc set-field commit:m99#type --value <TYPE> --task m99",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:m99#scope"),
        "jigc doc set-field commit:m99#scope --value <SCOPE> --task m99",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:m99#summary"),
        "jigc doc set-slot commit:m99#summary --from-file - --task m99",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:m99#body"),
        "jigc doc set-slot commit:m99#body --from-file - --task m99",
    );
    assert_eq!(
        emitted_run_line(&stdout, "task finalize"),
        "jigc task finalize m99",
    );

    let pos = |needle: &str| {
        stdout
            .find(needle)
            .unwrap_or_else(|| panic!("composed planning spine missing {needle:?}; got:\n{stdout}"))
    };

    // The finalize tail follows the decompose phase AND the last author step
    // (author-decisions, whose create-log line is `doc create decisions-log`) —
    // the agent decomposes, authors the running docs, fills the commit, then
    // finalizes.
    let decompose_at = pos("Decompose the milestone into ordered");
    let author_decisions_at = pos("jigc doc create decisions-log");
    let commit_fill_at = pos("jigc doc set-field commit:m99#type");
    let finalize_at = pos("jigc task finalize m99");
    assert!(
        decompose_at < author_decisions_at
            && author_decisions_at < commit_fill_at
            && commit_fill_at < finalize_at,
        "the finalize tail must follow decompose({decompose_at}) and the author steps \
         ({author_decisions_at}): commit-fill({commit_fill_at}) < finalize({finalize_at}); \
         got:\n{stdout}",
    );
}

/// The byte-identical batch-authoring caveat span shared by author-roadmap /
/// author-ledger / author-decisions (added 9b664ae; `ideas/batch-authoring-
/// ergonomics.md`). Planning composes all three, so before the compose-altitude
/// dedupe it appeared verbatim three times.
const BATCH_CAVEAT_SPAN: &str =
    "Or make it one call — the batch alternative to the whole sequence above (run it
instead of the create + per-entry verbs, or after them — the create it implies
over a doc this task already staged acks `already existed — copied in for
update` and changes nothing): the batch verb reads one declarative payload from
stdin (grammar: `jigc doc author --help`) and places every entry in a single";

/// M45 Inc 10 T13 — the compose-altitude render-once dedupe of the batch-authoring
/// caveat (`RC-alpha3/findings-verification.md` §5; `surface-contract.md` /
/// `workflow-dialect.md` → Render-once blocks). The three running-doc author steps
/// each wrap the identical caveat in a `<!-- once:batch-author-note -->` block, so
/// the planning walk composes it **once** — the noise the trial flagged — while the
/// per-doctype `jigc doc author <type>` command lines (outside the block) all
/// survive, and no marker leaks into the agent-facing bytes. Driven on the EMITTED
/// composed output of the real planning workflow through the binary.
#[test]
fn planning_dedupes_the_batch_authoring_caveat() {
    let repo = TempDir::new("planning-dedupe");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow planning \"M99\"` must compose the planning spine and exit 0; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // The caveat span composes exactly once across the three-step planning walk.
    assert_eq!(
        stdout.matches(BATCH_CAVEAT_SPAN).count(),
        1,
        "the batch-authoring caveat must compose exactly once in the planning walk; got:\n{stdout}",
    );

    // No once-marker leaks into the agent-facing composed output.
    assert!(
        !stdout.contains("<!-- once") && !stdout.contains("<!-- /once"),
        "no once-marker may leak into the composed planning output; got:\n{stdout}",
    );

    // The per-doctype batch command lines sit OUTSIDE the block, so the collapse
    // leaves every one intact — the dedupe removes the repeated prose, not the
    // affordance each step solicits.
    for needle in [
        "jigc doc author roadmap --from-file - --task m99",
        "jigc doc author deferral-ledger --from-file - --task m99",
        "jigc doc author decisions-log --from-file - --task m99",
    ] {
        assert!(
            stdout.contains(needle),
            "the batch command line `{needle}` must survive the caveat dedupe; got:\n{stdout}",
        );
    }
}

/// M17 self-hosting dogfood — the methodology `decided-task` workflow composes the
/// dev-workflow spine PLUS a decision-recording step through the real binary:
/// `jigc start --workflow decided-task "<intent>"` mints a task and composes
/// scope → implement → gate → author-decision → finalize, emitting the
/// decisions-log create + per-entry authoring lines. This is the lightweight
/// sub-milestone decision-recording path (the grain gap decisions-pending.md:77),
/// distinct from milestone `planning`: it carries `allows-create: [{decisions-log}]`
/// so the `{{cli.create-log}}` ref resolves (the create-gate admits the type) WITHOUT
/// authoring the roadmap / deferral-ledger that planning forces.
#[test]
fn decided_task_composes_the_dev_spine_plus_the_decision_recording_step() {
    let repo = TempDir::new("decided-task");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // `decided-task` is `creates-task: true` — a bare `--workflow decided-task` mints
    // a task and composes the spine. Intent "add cache" → slug "add-cache".
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "decided-task", "add cache"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow decided-task \"add cache\"` over the methodology pack must \
         compose the spine and exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `creates-task: true` → a task dir is minted at `.jigc/tasks/add-cache/`.
    assert!(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("add-cache")
            .is_dir(),
        "`--workflow decided-task` must mint the task dir; got stdout:\n{stdout}",
    );

    // No `{{ … }}` placeholder survives a clean compose.
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "no `{{{{ … }}}}` placeholder may survive the decided-task compose; got:\n{stdout}",
    );

    let pos = |needle: &str| {
        stdout.find(needle).unwrap_or_else(|| {
            panic!("composed decided-task spine missing {needle:?}; got:\n{stdout}")
        })
    };

    // The spine composes scope → implement → gate → author-decision → finalize. Each
    // step contributes a distinctive prose marker; offsets strictly increase.
    let scope_at = pos("done-criterion");
    let implement_at = pos("failing test");
    let gate_at = pos("test, lint, and build gate");
    let decision_at = pos("design decision worth keeping");
    let finalize_at = pos("one logical commit");
    assert!(
        scope_at < implement_at
            && implement_at < gate_at
            && gate_at < decision_at
            && decision_at < finalize_at,
        "the spine must order scope({scope_at}) < implement({implement_at}) < gate({gate_at}) \
         < author-decision({decision_at}) < finalize({finalize_at}); got:\n{stdout}",
    );

    // The decisions-log authoring composes — the create line resolves ONLY if the
    // workflow's `allows-create` admits `decisions-log` (the create-gate). The singleton
    // create-ref renders verbatim on the EMITTED bytes (the planning precedent).
    assert_eq!(
        emitted_run_line(&stdout, "doc create decisions-log"),
        "jigc doc create decisions-log --title 'Decisions Log' --task add-cache",
    );
    for needle in [
        "jigc doc add-item decisions-log:decisions-log#entries",
        "jigc doc set-slot decisions-log:decisions-log#entries/<id>/why",
    ] {
        assert!(
            stdout.contains(needle),
            "the decided-task spine must emit the decisions-log authoring line `{needle}`; \
             got:\n{stdout}",
        );
    }

    // The decision-recording authoring follows the gate AND precedes the finalize tail
    // (record the decision, then fill the commit, then finalize).
    assert!(
        gate_at < pos("jigc doc create decisions-log")
            && pos("jigc doc create decisions-log") < pos("jigc task finalize add-cache"),
        "the decisions-log authoring must sit between the gate and finalize; got:\n{stdout}",
    );

    // decided-task does NOT author the roadmap / deferral-ledger that milestone planning
    // forces — it is the lightweight path, not milestone ceremony.
    assert!(
        !stdout.contains("jigc doc create roadmap")
            && !stdout.contains("jigc doc create deferral-ledger"),
        "decided-task must NOT author the roadmap / deferral-ledger (it is the lightweight \
         sub-milestone path, not milestone planning); got:\n{stdout}",
    );
}

/// M16 Increment 5 / T2 — the methodology `completion` workflow composes the
/// completion spine through the real binary: `jigc start --workflow completion
/// "<milestone>"` mints a task (off-router) **and** emits the completion-half's
/// human-gated halts as `Checkpoint:` directives plus the create-fresh
/// completion-record authoring (meta verdict/owner-artifact + repeatable findings)
/// and the reused decisions-log append (the both-halves driver).
///
/// This is the compose-time contract (T4 *runs* the emitted commands end-to-end
/// and fires the #5 owner-artifact presence gate; this proves the directives are
/// **emitted**, with the `{{cli.X}}` authoring command-refs rendered verbatim on
/// the EMITTED bytes — never reconstructed). The completion-record is
/// `id-from: title` create-fresh, so its slug is runtime-minted from the milestone
/// title; the per-entry authoring addresses carry the literal `<slug>` authoring
/// placeholder (the author-arch-doc precedent).
#[test]
fn completion_composes_the_checkpoints_and_the_completion_record_authoring_lines() {
    let repo = TempDir::new("completion");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // `completion` is `creates-task: true, selectable: false` (the sub-task precedent)
    // — invoked directly off the router by `--workflow completion`, it mints a task and
    // composes the spine. Intent "M99" → slug "m99" (slugify lowercases).
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "completion", "M99"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow completion \"M99\"` over the methodology pack must compose the \
         completion spine and exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `creates-task: true` → a task dir is minted at `.jigc/tasks/m99/`.
    let task_dir = repo.path().join(".jigc").join("tasks").join("m99");
    assert!(
        task_dir.is_dir(),
        "`--workflow completion` must mint the task dir at {task_dir:?}; got stdout:\n{stdout}",
    );

    // No `{{ … }}` placeholder survives a clean compose (every include / data-value /
    // command-ref resolved) — the spike that the `{{cli.X}}` refs combine against the
    // built grammar, not assumed.
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "no `{{{{ … }}}}` placeholder may survive the completion compose; got:\n{stdout}",
    );

    let pos = |needle: &str| {
        stdout.find(needle).unwrap_or_else(|| {
            panic!("composed completion spine missing {needle:?}; got:\n{stdout}")
        })
    };

    // The completion spine composes the audit → triage → fix → re-verify phase walk,
    // with the human-gated halts as `Checkpoint:` directives. Each phase contributes a
    // distinctive prose marker; offsets strictly increase.
    let audit_at = pos("Audit the assembled milestone");
    let triage_at = pos("Checkpoint: triage-gate");
    let fix_at = pos("Checkpoint: fix-rounds-exhausted");
    let reverify_at = pos("Re-verify");
    assert!(
        audit_at < triage_at && triage_at < fix_at && fix_at < reverify_at,
        "the completion spine must order audit({audit_at}) < triage({triage_at}) < \
         fix({fix_at}) < re-verify({reverify_at}); got:\n{stdout}",
    );

    // Both human-gated halts emit `Checkpoint: <slug>` at their own line, the bare slug
    // (no backticks) — the M15 checkpoint primitive. triage-gate is the completion-half's
    // own gate (too-big / contested); fix-rounds-exhausted is the reused fix-gate.
    for slug in ["triage-gate", "fix-rounds-exhausted"] {
        assert!(
            stdout.lines().any(|l| l == format!("Checkpoint: {slug}")),
            "the completion spine must compose `Checkpoint: {slug}` at the left margin; \
             got:\n{stdout}",
        );
        assert!(
            !stdout.contains(&format!("Checkpoint: `{slug}`")),
            "the `{slug}` reason slug must be bare (no backticks); got:\n{stdout}",
        );
    }

    // The completion-record `create` command-ref renders verbatim on the EMITTED bytes.
    // It is `id-from: title` create-fresh (the adr/arch-doc mint), so `--title` carries
    // the agent's milestone-named title placeholder (NOT a singleton fixed literal).
    assert_eq!(
        emitted_run_line(&stdout, "doc create completion-record"),
        "jigc doc create completion-record --title <TITLE> --task m99",
    );

    // The create-gate admits `completion-record` (the workflow's `allows-create`): the
    // create line composed above only resolves if the gate admits the type.
    // The meta-header authoring: verdict enum + the owner-artifact owned-location path
    // (the #5-gate target). The `<slug>` is the runtime-minted create-fresh slug, an
    // authoring placeholder (the author-arch-doc precedent). The per-finding repeatable
    // authoring: add-item + set-field severity/disposition + set-field evidence (a string
    // FIELD — `set-field`, not `set-slot`; the binary rejects `set-slot` on a field).
    for needle in [
        "jigc doc set-field completion-record:<slug>#verdict",
        "jigc doc set-field completion-record:<slug>#owner-artifact",
        "jigc doc add-item completion-record:<slug>#findings",
        "jigc doc set-field completion-record:<slug>#findings/<id>/severity",
        "jigc doc set-field completion-record:<slug>#findings/<id>/disposition",
        "jigc doc set-field completion-record:<slug>#findings/<id>/evidence",
    ] {
        assert!(
            stdout.contains(needle),
            "the completion spine must emit the authoring line `{needle}`; got:\n{stdout}",
        );
    }

    // The owner-artifact is written under the owned artifact home the #5 gate asserts on
    // (`completions/artifacts/<milestone>/…`) — the guidance names that home so the
    // recorded path is durable, not an arbitrary pre-existing file.
    assert!(
        stdout.contains("completions/artifacts/"),
        "the completion-record authoring must name the owned artifact home \
         `completions/artifacts/<milestone>/`; got:\n{stdout}",
    );

    // The decisions-log append is REUSED (the both-halves driver): the same idempotent
    // create-log + add-item/set-slot the planning half drives, here recording triage
    // decisions. Assert the reused command-ref + per-entry authoring compose.
    assert_eq!(
        emitted_run_line(&stdout, "doc create decisions-log"),
        "jigc doc create decisions-log --title 'Decisions Log' --task m99",
    );
    for needle in [
        "jigc doc add-item decisions-log:decisions-log#entries",
        "jigc doc set-slot decisions-log:decisions-log#entries/<id>/why",
    ] {
        assert!(
            stdout.contains(needle),
            "the completion spine must emit the reused decisions-log authoring line \
             `{needle}`; got:\n{stdout}",
        );
    }

    // The authoring follows the re-verify phase (the record is authored after the loop
    // produces a verdict + closed findings).
    assert!(
        reverify_at < pos("jigc doc create completion-record"),
        "the completion-record authoring must follow the re-verify phase; got:\n{stdout}",
    );

    // The finalize step composes last (validate + commit boundary — the #5 gate fires
    // here in T4). Assert the finalize Run line is emitted, after the authoring.
    assert!(
        pos("jigc doc create completion-record") < pos("jigc task finalize"),
        "the finalize step must follow the completion-record authoring; got:\n{stdout}",
    );
}

/// M17 Increment 5 / T2 — the methodology `record-dogfood` workflow composes the
/// recording spine through the real binary: `jigc start --workflow record-dogfood
/// "<run>"` mints a task (off-router, `creates-task: true, selectable: false` —
/// the completion authoring-spine pattern) **and** emits the create-fresh
/// dogfood-record authoring: the run identity (case + binary-sha), the eight
/// ORGANIC fact-int transcription lines, the two SEEDED instrument-check lines,
/// the verdict + owner-artifact, the judgment slot, and the finalize tail.
///
/// This is the compose-time contract (T5 *runs* the emitted commands end-to-end;
/// this proves they are **emitted**, with the `{{cli.create-dogfood-record}}` ref
/// rendered verbatim on the EMITTED bytes — never reconstructed). The
/// dogfood-record is `id-from: title` create-fresh, so its slug is runtime-minted
/// from the run title; the authoring addresses carry the literal `<slug>`
/// authoring placeholder (the author-completion-record precedent), and EVERY
/// authoring line carries `--task <minted-id>` resolved at compose time.
#[test]
fn record_dogfood_composes_the_record_authoring_lines_and_finalize_tail() {
    let repo = TempDir::new("record-dogfood");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // Off-router by construction: `--workflow record-dogfood` mints a task and
    // composes the spine. Intent "pilot run" → slug "pilot-run".
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "record-dogfood", "pilot run"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow record-dogfood \"pilot run\"` over the methodology pack must \
         compose the recording spine and exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");

    // `creates-task: true` → a task dir is minted at `.jigc/tasks/pilot-run/`.
    let task_dir = repo.path().join(".jigc").join("tasks").join("pilot-run");
    assert!(
        task_dir.is_dir(),
        "`--workflow record-dogfood` must mint the task dir at {task_dir:?}; got stdout:\n{stdout}",
    );

    // No `{{ … }}` placeholder survives a clean compose (every include / data-value /
    // command-ref resolved — incl. the literal lines' inline `{{task.id}}`).
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "no `{{{{ … }}}}` placeholder may survive the record-dogfood compose; got:\n{stdout}",
    );

    // The dogfood-record `create` command-ref renders verbatim on the EMITTED bytes.
    // It is `id-from: title` create-fresh (the completion-record mint), so `--title`
    // carries the agent's run-named title placeholder (NOT a singleton fixed literal).
    assert_eq!(
        emitted_run_line(&stdout, "doc create dogfood-record"),
        "jigc doc create dogfood-record --title <TITLE> --task pilot-run",
    );

    // The meta-header authoring: the run identity, the eight ORGANIC fact ints, the
    // two SEEDED instrument checks, the verdict enum + the owner-artifact
    // owned-location (the #5-gate target). The `<slug>` is the runtime-minted
    // create-fresh slug, an authoring placeholder. Every line carries the minted
    // task id (the >1-active-task disambiguation), resolved — never `{{task.id}}`.
    for field in [
        "case",
        "binary-sha",
        "adapter-writes",
        "oob-edits",
        "drift-caught",
        "validate-blocks",
        "halts-expected",
        "halts-unplanned",
        "fix-rounds",
        "audit-findings",
        "seeded-oob",
        "seeded-blocks",
        "verdict",
        "owner-artifact",
    ] {
        let needle = format!("jigc doc set-field dogfood-record:<slug>#meta/{field}");
        let line = stdout
            .lines()
            .find(|l| l.contains(&needle))
            .unwrap_or_else(|| {
                panic!(
                    "the recording spine must emit the authoring line `{needle}`; got:\n{stdout}"
                )
            });
        assert!(
            line.contains("--task pilot-run"),
            "the `{field}` authoring line must carry `--task pilot-run` resolved; \
             offending line:\n{line}\nfull stdout:\n{stdout}",
        );
    }

    // The judgment slot authoring (the ONE prose slot — the verdict is authored
    // judgment over the counts, never computed).
    let judgment_line = stdout
        .lines()
        .find(|l| l.contains("jigc doc set-slot dogfood-record:<slug>#judgment"))
        .unwrap_or_else(|| {
            panic!("the recording spine must emit the judgment set-slot line; got:\n{stdout}")
        });
    assert!(
        judgment_line.contains("--task pilot-run"),
        "the judgment authoring line must carry `--task pilot-run`; offending line:\
         \n{judgment_line}\nfull stdout:\n{stdout}",
    );

    // The owner-artifact is written under the engine-native owned artifact home the
    // #5 gate asserts on (`completions/artifacts/<run>/…`) — the guidance names that
    // home so the recorded path is durable, not an arbitrary pre-existing file.
    assert!(
        stdout.contains("completions/artifacts/"),
        "the dogfood-record authoring must name the owned artifact home \
         `completions/artifacts/<run>/`; got:\n{stdout}",
    );

    let pos = |needle: &str| {
        stdout.find(needle).unwrap_or_else(|| {
            panic!("composed recording spine missing {needle:?}; got:\n{stdout}")
        })
    };

    // The finalize tail composes last: the four commit-fill Run lines + the
    // finalize-task line render verbatim with addresses resolved to the minted
    // `pilot-run` slug (the emitted-bytes contract), AFTER the record authoring.
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:pilot-run#type"),
        "jigc doc set-field commit:pilot-run#type --value <TYPE> --task pilot-run",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:pilot-run#summary"),
        "jigc doc set-slot commit:pilot-run#summary --from-file - --task pilot-run",
    );
    assert_eq!(
        emitted_run_line(&stdout, "task finalize"),
        "jigc task finalize pilot-run",
    );
    assert!(
        pos("jigc doc create dogfood-record")
            < pos("jigc doc set-slot dogfood-record:<slug>#judgment")
            && pos("jigc doc set-slot dogfood-record:<slug>#judgment")
                < pos("jigc doc set-field commit:pilot-run#type")
            && pos("jigc doc set-field commit:pilot-run#type")
                < pos("jigc task finalize pilot-run"),
        "the spine must order create < judgment-authoring < commit-fill < finalize; \
         got:\n{stdout}",
    );
}

/// M17 Increment 4 / T3 — every composed doc-verb command carries the minted
/// task id, proven against the **>1-active-task** case the workflows themselves
/// create (the M17 settle pre-fix #3 wall: a `--task`-less `jigc doc …` rejects
/// with `more than one active task` as soon as a second task is active).
///
/// With one dev-task already active, `jigc start --workflow planning "M99"`
/// mints a second task (`m99`); the composed planning spine must emit **every**
/// `jigc doc …` line — the `Run:`-class catalog refs *and* the literal authoring
/// guidance — carrying `--task m99` (the catalog refs via `{ from: "task.id" }`,
/// the literal lines via the T2 inline `{{task.id}}` compose-time substitution;
/// the no-`{{`-survives assertion proves the substitution resolved, never leaked).
/// Then one composed authoring line and one composed commit-fill line are
/// executed **verbatim** (the emitted bytes, split into argv — never
/// reconstructed) and must succeed, landing the write in the PLANNING task's
/// working area.
/// Whether the `jigc doc <verb>` leaf declares a `--task` argument, read from the
/// built clap tree. A verb that does not (`schema`, the cascade-resolved shape read)
/// is task-independent by construction, so a composed line for it carries no task id
/// and must not be held to the >1-active-task disambiguation rule.
fn verb_declares_task(verb: &str) -> bool {
    use clap::CommandFactory;
    let mut root = <cli::cli::Cli as CommandFactory>::command();
    root.build();
    root.get_subcommands()
        .find(|sub| sub.get_name() == "doc")
        .and_then(|doc| doc.get_subcommands().find(|sub| sub.get_name() == verb))
        .is_some_and(|leaf| leaf.get_arguments().any(|arg| arg.get_id() == "task"))
}

#[test]
fn composed_authoring_commands_carry_the_minted_task_id_with_two_active_tasks() {
    let repo = TempDir::new("two-active");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; got {:?}\nstderr:\n{}",
        setup.status,
        String::from_utf8_lossy(&setup.stderr),
    );

    // First active task: a bare start composes `dev-task` and mints
    // `add-rate-limiter` — the concurrent task the wall needs.
    let first = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "add rate limiter"],
    );
    assert!(
        first.status.success(),
        "`jigc start \"add rate limiter\"` must mint the first task; got {:?}\nstderr:\n{}",
        first.status,
        String::from_utf8_lossy(&first.stderr),
    );

    // Second active task: the planning workflow mints `m99` ("M99" slugified).
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow planning \"M99\"` must mint a second task and compose; \
         got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    for id in ["add-rate-limiter", "m99"] {
        assert!(
            repo.path().join(".jigc").join("tasks").join(id).is_dir(),
            "both tasks must be active (task dir `{id}` present); got stdout:\n{stdout}",
        );
    }

    // No `{{ … }}` survives — proves the literal lines' ` --task {{task.id}}`
    // substitution resolved at compose time (never leaked to the agent).
    assert!(
        !stdout.contains("{{") && !stdout.contains("}}"),
        "no `{{{{ … }}}}` placeholder may survive the planning compose; got:\n{stdout}",
    );

    // EVERY emitted `jigc doc …` COMMAND LINE — Run-class and literal — carries the
    // minted planning id. Asserted over the emitted bytes, line by line.
    //
    // A command line is one an agent copies and runs: after the `Run: \`` decoration
    // is stripped, the line *begins* with the invocation. A line that merely *mentions*
    // a verb inside a sentence is prose, not a write — the `{{schema:<T>}}` projection's
    // own instruction sentence (*"Author the whole document in ONE `jigc doc author`
    // batch payload"*, M43's generation seam, reaching this workflow at M49 Inc-9 / T6)
    // names the batch verb three lines above the runnable line that carries `--task`.
    // Three exemptions, all non-writes: that prose class; a `--help` mention (the
    // batch alternative's grammar pointer, M43 inc-7 T6), which reads the CLI's own
    // help; and a verb that declares **no** `--task` at all — `jigc doc schema <T>`,
    // the task-independent shape read (M49 Inc 11 / T7), for which `--task m99` would
    // be a usage error rather than a disambiguation. That third exemption is read off
    // the clap tree ([`verb_declares_task`]) rather than spelled as a verb name, so a
    // future `doc` verb joins the rule or its exemption by its own declaration.
    let doc_lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.contains("jigc doc ") && !l.contains("--help"))
        .filter(|l| {
            let bare = l.trim().trim_start_matches("Run: `");
            bare.starts_with("jigc doc ")
        })
        .filter(|l| {
            let bare = l.trim().trim_start_matches("Run: `");
            bare.split_whitespace()
                .nth(2)
                .is_some_and(verb_declares_task)
        })
        .collect();
    assert!(
        !doc_lines.is_empty(),
        "the planning spine must emit `jigc doc …` lines; got:\n{stdout}",
    );
    for line in &doc_lines {
        assert!(
            line.contains("--task m99"),
            "every composed `jigc doc …` line must carry `--task m99` (the >1-active-task \
             disambiguation); offending line:\n{line}\nfull stdout:\n{stdout}",
        );
    }

    // Execute ONE composed authoring line VERBATIM: the create-roadmap Run line.
    // Red today: `more than one active task — name one with --task <id>`.
    let create = emitted_run_line(&stdout, "doc create roadmap");
    let argv: Vec<&str> = create.split_whitespace().collect();
    assert_eq!(
        argv[0], "jigc",
        "the emitted command invokes jigc: {create:?}"
    );
    let executed = run_jigc(repo.path(), home.path(), &pack, &argv[1..]);
    assert!(
        executed.status.success(),
        "the composed authoring line `{create}` must run verbatim with two active tasks; \
         got {:?}\nstderr:\n{}",
        executed.status,
        String::from_utf8_lossy(&executed.stderr),
    );
    // The write routed to the PLANNING task's working area, not the first task's.
    let staged = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("m99")
        .join("docs")
        .join("roadmap:roadmap.md");
    assert!(
        staged.is_file(),
        "the verbatim create must land the roadmap in the planning task's working area \
         ({staged:?}); stdout of the create:\n{}",
        String::from_utf8_lossy(&executed.stdout),
    );

    // Execute the composed **deferral-ledger** create line VERBATIM too (M48). Its
    // `--title` is the schema's `display-title:` — two words — because a divergent title
    // on a singleton is now refused rather than silently dropped
    // (`design/write-commands.md` → The four-way write), so the emitted line carries a
    // shell-quoted argument for the first time. Running the emitted bytes is the only
    // way that quoting is proven: a whitespace-split reconstruction would pass while the
    // line an agent actually pastes broke.
    let ledger = emitted_run_line(&stdout, "doc create deferral-ledger");
    assert!(
        ledger.contains("--title 'Deferral Ledger'"),
        "the composed create carries the schema's own title, shell-quoted: {ledger:?}",
    );
    let ledger_argv = shell_split(ledger);
    assert_eq!(
        ledger_argv.first().map(String::as_str),
        Some("jigc"),
        "the emitted command invokes jigc: {ledger:?}"
    );
    let ledger_args: Vec<&str> = ledger_argv.iter().skip(1).map(String::as_str).collect();
    let ledger_out = run_jigc(repo.path(), home.path(), &pack, &ledger_args);
    assert!(
        ledger_out.status.success(),
        "the composed ledger line `{ledger}` must run verbatim; got {:?}\nstderr:\n{}",
        ledger_out.status,
        String::from_utf8_lossy(&ledger_out.stderr),
    );
    assert!(
        repo.path()
            .join(".jigc/tasks/m99/docs/deferral-ledger:deferral-ledger.md")
            .is_file(),
        "the verbatim ledger create lands in the planning task's working area",
    );

    // Execute ONE composed commit-fill line VERBATIM: the summary set-slot
    // (stdin-fed `--from-file -`). Red today: the same >1-active-task rejection.
    let fill = emitted_run_line(&stdout, "set-slot commit:m99#summary");
    let argv: Vec<&str> = fill.split_whitespace().collect();
    assert_eq!(
        argv[0], "jigc",
        "the emitted command invokes jigc: {fill:?}"
    );
    let filled = run_jigc_stdin(
        repo.path(),
        home.path(),
        &pack,
        &argv[1..],
        "plan M99 into increments\n",
    );
    assert!(
        filled.status.success(),
        "the composed commit-fill line `{fill}` must run verbatim with two active tasks; \
         got {:?}\nstderr:\n{}",
        filled.status,
        String::from_utf8_lossy(&filled.stderr),
    );
}

/// The composed body with runs of whitespace collapsed to single spaces — step prose
/// is hard-wrapped at ~80 columns, so a sentence an agent must read straddles a line
/// break. Still the EMITTED bytes; it just refuses to make the assertion hostage to a
/// reflow (`methodology_staging_contract.rs`'s precedent).
fn flowed(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// M48 Increment 9 / T6 — planning's finalize names the milestone verbs and states
/// the seeding path's bound (`RC-pre-1.0/findings-verification.md` → F15; the Settle
/// → *F15's bound belongs on the surface that creates the adjacency*).
///
/// The trial's planner followed `planning` to its finalize, committed a roadmap
/// entry, and believed it had opened a milestone: the roadmap entry and the
/// `milestone:<id>` work-unit are two objects with no reference between them, and
/// **no pack prose named a single milestone verb** (verified by grep over both packs
/// before this task). So the composed spine must name `jigc milestone create` — and
/// `jigc milestone add-task` with it, because an agent that reaches for the
/// obvious-looking `add-from-spec` is stopped by a gap the naming itself surfaces:
/// that verb seeds one sub-task per committed spec `criterion`, which planning's
/// prose `decomposition` slot cannot feed.
///
/// Asserted on the EMITTED bytes of **both** composing doors — `jigc start --workflow
/// planning` (mints) and `jigc workflow planning --preview` (mints nothing) — since
/// the preview is the shape an operator reads before committing to the walk.
#[test]
fn planning_finalize_names_the_milestone_verbs_and_the_seeding_bound() {
    let repo = TempDir::new("planning-milestone-verbs");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );

    let preview = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["workflow", "planning", "--preview"],
    );
    assert!(
        preview.status.success(),
        "`jigc workflow planning --preview` must compose + exit 0; stderr:\n{}",
        String::from_utf8_lossy(&preview.stderr),
    );
    let previewed = String::from_utf8(preview.stdout).expect("utf-8 stdout");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "M99"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow planning \"M99\"` must compose + exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let started = String::from_utf8(out.stdout).expect("utf-8 stdout");

    for (door, body) in [("start", &started), ("workflow --preview", &previewed)] {
        let flat = flowed(body);
        // The two manual verbs render as runnable lines, verbatim.
        assert!(
            body.lines()
                .any(|line| line.trim() == "jigc milestone create \"<the title you just used>\""),
            "`{door}` must name the work-unit mint as a runnable line; got:\n{body}",
        );
        assert!(
            body.lines()
                .any(|line| line.trim() == "jigc milestone add-task <milestone-id> \"<intent>\""),
            "`{door}` must name `add-task` as the manual seeding path; got:\n{body}",
        );
        // The bound, so naming the seeding path does not dead-end at `add-from-spec`.
        assert!(
            flat.contains("jigc milestone add-from-spec"),
            "`{door}` must name the `add-from-spec` bound rather than leave it to be \
             discovered; got:\n{body}",
        );
        assert!(
            flat.contains("criterion") && flat.contains("`decomposition` slot"),
            "`{door}` must state WHY `add-from-spec` cannot seed this milestone — it reads \
             a committed spec's criteria, and the decomposition is prose in a slot; \
             got:\n{body}",
        );
        // The shared finalize tail still composes through the wrapper.
        assert!(
            flat.contains("only what you have staged"),
            "`{door}` must still compose the shared finalize tail; got:\n{body}",
        );
        assert!(
            !body.contains("{{") && !body.contains("}}"),
            "`{door}` must compose with no surviving `{{{{ … }}}}` placeholder; got:\n{body}",
        );
    }
}

/// The omitting context (dev-workflow hardening #5): `dev-task` composes the SAME
/// shared `step:finalize`, through no wrapper — the milestone prose must be **inert
/// there**, never leaked and never an error. A wrapper step that landed its prose in
/// the shared step instead would pass the arm above and fail here.
#[test]
fn a_workflow_that_composes_the_bare_finalize_carries_no_milestone_prose() {
    let repo = TempDir::new("planning-milestone-inert");
    init_repo(repo.path());
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "dev-task", "add a cache"],
    );
    assert!(
        out.status.success(),
        "`jigc start --workflow dev-task` must compose + exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let body = String::from_utf8(out.stdout).expect("utf-8 stdout");

    assert!(
        flowed(&body).contains("only what you have staged"),
        "`dev-task` must compose the shared finalize tail (the omitting context is real); \
         got:\n{body}",
    );
    assert!(
        !body.contains("jigc milestone"),
        "`dev-task` composes the bare `step:finalize` — no milestone verb may leak into it; \
         got:\n{body}",
    );
}
