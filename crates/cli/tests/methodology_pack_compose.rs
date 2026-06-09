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
//! methodology pack from `CARGO_MANIFEST_DIR/../../packs/methodology`, and a
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Extract the backtick-quoted body of the *unique* `Run:` line containing `needle`,
/// so the assertion runs over the **emitted bytes** the agent would copy — never a
/// reconstruction. Panics with the full stdout if absent or ambiguous.
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
        "jigc doc set-field commit:add-rate-limiter#type --value <TYPE>",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-field commit:add-rate-limiter#scope"),
        "jigc doc set-field commit:add-rate-limiter#scope --value <SCOPE>",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:add-rate-limiter#summary"),
        "jigc doc set-slot commit:add-rate-limiter#summary --from-file -",
    );
    assert_eq!(
        emitted_run_line(&stdout, "set-slot commit:add-rate-limiter#body"),
        "jigc doc set-slot commit:add-rate-limiter#body --from-file -",
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
        "jigc doc create roadmap --title Roadmap",
    );
    assert_eq!(
        emitted_run_line(&stdout, "doc create deferral-ledger"),
        "jigc doc create deferral-ledger --title Deferral-Ledger",
    );
    assert_eq!(
        emitted_run_line(&stdout, "doc create decisions-log"),
        "jigc doc create decisions-log --title Decisions-Log",
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
        "jigc doc create completion-record --title <TITLE>",
    );

    // The create-gate admits `completion-record` (the workflow's `allows-create`): the
    // create line composed above only resolves if the gate admits the type.
    // The meta-header authoring: verdict enum + the owner-artifact owned-location path
    // (the #5-gate target). The `<slug>` is the runtime-minted create-fresh slug, an
    // authoring placeholder (the author-arch-doc precedent). The per-finding repeatable
    // authoring: add-item + set-field severity/disposition + set-slot evidence.
    for needle in [
        "jigc doc set-field completion-record:<slug>#verdict",
        "jigc doc set-field completion-record:<slug>#owner-artifact",
        "jigc doc add-item completion-record:<slug>#findings",
        "jigc doc set-field completion-record:<slug>#findings/<id>/severity",
        "jigc doc set-field completion-record:<slug>#findings/<id>/disposition",
        "jigc doc set-slot completion-record:<slug>#findings/<id>/evidence",
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
        "jigc doc create decisions-log --title Decisions-Log",
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
