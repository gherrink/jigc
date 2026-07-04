//! M16 Increment 4 / T4 — **flow 19: the planning-half encode**, proven end-to-end
//! over the real `jigc` binary across **two milestone runs** (cold-create then
//! warm-append) plus the three reds (`design/methodology-docs.md` → Acceptance flows,
//! Flow 19; `design/worked-examples.md` → flow 19).
//!
//! `jigc start --workflow planning "<milestone>"` mints a task (off-router), the agent
//! authors the running working-docs the planning loop maintains — the `roadmap`
//! milestone entry (BOTH the `proves` and the prose `decomposition` slots, the
//! two-slot-per-repeatable-item case Increment 1b unblocked), the `deferral-ledger`
//! entries, and a `decisions-log` entry — via idempotent-create + `add-item`/`set-slot`/
//! `set-field`, and `finalize` promotes the three running singletons. This file is the
//! flow-19 instantiation of the cold/warm/drift harness `singleton_running_doc.rs`
//! proved on the `runlog` FIXTURE — now driven over the THREE REAL methodology
//! doctypes through the real `planning` workflow.
//!
//! Every assertion runs on the EMITTED bytes of the real binary (`CARGO_BIN_EXE_jigc`)
//! over the on-disk methodology pack (`JIGC_PACK_DIR=<methodology>`, the same seam the
//! sibling methodology tests use): `jigc setup` installs the pack, the promotion is
//! read back via `git show HEAD:<path>`, so the byte-stability claims are over the
//! bytes git actually holds.
//!
//! The five done-criteria:
//!   (1) **run 1 cold-create** — `jigc start --workflow planning` → idempotent-create
//!       the three singletons → author → `finalize` promotes
//!       `docs/roadmap.md`, `docs/deferral-ledger.md`,
//!       `docs/decisions-log.md` byte-stable;
//!   (2) **run 2 warm-append** — a second milestone copies the committed singletons in
//!       (prior milestone preserved, provenance `edited-from-base`), `add-item`
//!       appends, finalize re-promotes byte-stable with BOTH milestones present;
//!   (3) **a half-authored roadmap entry BLOCKS** at validate
//!       (`schema-conformance.required-slot-present`) — a real roadmap entry with a
//!       genuinely-empty `decomposition` slot, proving Increment 1's per-item
//!       conformance fires on THIS schema's leaves (the vacuously-green-managed-doc
//!       guard);
//!   (4) **`planning` is ABSENT from the router catalog** — bare `jigc start` lists no
//!       `planning` line (selectable: false, off-router);
//!   (5) **a baseline-recorded-then-OOB-drifted warm-append CONFLICT-BLOCKS** at
//!       finalize-preflight with `reconciliation.conflict-block` (the baseline is
//!       recorded by the cold finalize FIRST, then the committed file is drifted
//!       out-of-band — an unrecorded-baseline drift would vacuously baseline-adopt).

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
            "jigc-flow19-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` over the methodology pack with `cwd = repo`, `$HOME = home`,
/// `JIGC_PACK_DIR = <methodology>` (the sibling methodology-test seam), capturing
/// output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree());
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

/// Run `jigc setup` (installs the methodology pack) over a fresh repo, asserting OK.
fn setup(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(repo, home, &["setup"]),
        "`JIGC_PACK_DIR=<methodology> jigc setup`",
    );
}

/// Start a `planning` task for `milestone`, returning the slugged task id git/finalize
/// addresses (the methodology `planning` workflow is off-router but invoked directly).
fn start_planning(repo: &Path, home: &Path, milestone: &str) {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", "planning", milestone]),
        &format!("`jigc start --workflow planning {milestone}`"),
    );
}

/// Fill the commit doc's author-required header + prose so finalize validates clean.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "docs");
    set_field(&format!("commit:{task}#scope"), "planning");
    set_slot(&format!("commit:{task}#summary"), b"plan a milestone\n");
    set_slot(&format!("commit:{task}#body"), b"A planning pass.\n");
}

/// The committed bytes of `path` at HEAD.
fn committed(repo: &Path, path: &str) -> String {
    let out = Command::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        out.status.success(),
        "{path} must be committed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed bytes")
}

/// The staged bytes of a managed doc in the task working area (the promote source).
fn staged(repo: &Path, task: &str, doc_file: &str) -> String {
    fs::read_to_string(
        repo.join(".jigc")
            .join("tasks")
            .join(task)
            .join("docs")
            .join(doc_file),
    )
    .unwrap_or_else(|e| panic!("read staged {doc_file} for task {task}: {e}"))
}

/// Idempotent-create the `<ty>` singleton in the active task (`create <ty>`): mint
/// cold / copy-in warm. Called once per doctype per task, mirroring the planning
/// workflow's author steps (the per-entry `add-item` authoring rides `read_or_copy_in`,
/// so a single `create` per task is the workflow's shape).
fn create_singleton(repo: &Path, home: &Path, ty: &str, title: &str) {
    assert_ok(
        &jigc_doc(repo, home, &["create", ty, "--title", title], None),
        &format!("`doc create {ty}`"),
    );
}

/// Author one `roadmap` milestone entry in the already-created singleton: `add-item`
/// the milestone → author BOTH the `proves` and the prose `decomposition` slots (the
/// two-slot item case Increment 1b unblocked).
fn author_roadmap_entry(repo: &Path, home: &Path, milestone: &str, proves: &[u8], decomp: &[u8]) {
    let item = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            "roadmap:roadmap#milestones",
            "--title",
            milestone,
        ],
        None,
    );
    assert_ok(&item, "`doc add-item roadmap:roadmap#milestones`");
    let item_addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &[
                "set-slot",
                &format!("{item_addr}/proves"),
                "--from-file",
                "-",
            ],
            Some(proves),
        ),
        "`set-slot <milestone>/proves`",
    );
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &[
                "set-slot",
                &format!("{item_addr}/decomposition"),
                "--from-file",
                "-",
            ],
            Some(decomp),
        ),
        "`set-slot <milestone>/decomposition`",
    );
}

/// Author one `deferral-ledger` entry (kind enum + trigger string + body slot; the
/// `date` is CLI-set on create) in the already-created singleton.
fn author_ledger_entry(
    repo: &Path,
    home: &Path,
    title: &str,
    kind: &str,
    trigger: &str,
    body: &[u8],
) {
    let item = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            "deferral-ledger:deferral-ledger#entries",
            "--title",
            title,
        ],
        None,
    );
    assert_ok(&item, "`doc add-item deferral-ledger#entries`");
    let item_addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-field", &format!("{item_addr}/kind"), "--value", kind],
            None,
        ),
        "`set-field <entry>/kind`",
    );
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &[
                "set-field",
                &format!("{item_addr}/trigger"),
                "--value",
                trigger,
            ],
            None,
        ),
        "`set-field <entry>/trigger`",
    );
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-slot", &format!("{item_addr}/body"), "--from-file", "-"],
            Some(body),
        ),
        "`set-slot <entry>/body`",
    );
}

/// Author one `decisions-log` entry (the `why` slot; `date` CLI-set on create) in the
/// already-created singleton.
fn author_decisions_entry(repo: &Path, home: &Path, title: &str, why: &[u8]) {
    let item = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            "decisions-log:decisions-log#entries",
            "--title",
            title,
        ],
        None,
    );
    assert_ok(&item, "`doc add-item decisions-log#entries`");
    let item_addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-slot", &format!("{item_addr}/why"), "--from-file", "-"],
            Some(why),
        ),
        "`set-slot <entry>/why`",
    );
}

#[test]
fn flow19_two_run_cold_create_then_warm_append_repromotes_byte_stable() {
    // (1) + (2): the two-milestone walk. Run 1 cold-creates the three singletons; run 2
    // (a second milestone) warm-re-creates each (copy-in preserves the prior milestone),
    // appends, and finalize re-promotes byte-stable with BOTH milestones present.
    let repo = TempDir::new("walk");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // ── Run 1: cold-create the three singletons + finalize (records the baselines) ──
    let cold = "m-alpha";
    start_planning(repo.path(), home.path(), "M-Alpha");
    create_singleton(repo.path(), home.path(), "roadmap", "Roadmap");
    create_singleton(
        repo.path(),
        home.path(),
        "deferral-ledger",
        "Deferral-Ledger",
    );
    create_singleton(repo.path(), home.path(), "decisions-log", "Decisions-Log");
    author_roadmap_entry(
        repo.path(),
        home.path(),
        "M-Alpha",
        b"M-Alpha proves the cold create.\n",
        b"Inc 1: the alpha increment, as prose.\n",
    );
    author_ledger_entry(
        repo.path(),
        home.path(),
        "defer the alpha cleanup",
        "D",
        "M-Beta",
        b"The alpha cleanup is owed to M-Beta.\n",
    );
    author_decisions_entry(
        repo.path(),
        home.path(),
        "chose the alpha shape",
        b"Because the alpha shape is simplest.\n",
    );

    // The cold staged sources — the promote sources for the byte-stability check.
    let roadmap_cold = staged(repo.path(), cold, "roadmap:roadmap.md");
    let ledger_cold = staged(repo.path(), cold, "deferral-ledger:deferral-ledger.md");
    let log_cold = staged(repo.path(), cold, "decisions-log:decisions-log.md");

    fill_commit(repo.path(), home.path(), cold);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", cold]),
        "`task finalize` (run 1 cold)",
    );

    // The three singletons are promoted to their canonical paths, each byte-stable vs
    // its staged source.
    assert_eq!(
        committed(repo.path(), "docs/roadmap.md"),
        roadmap_cold,
        "the cold-promoted roadmap is byte-stable (committed == staged)",
    );
    assert_eq!(
        committed(repo.path(), "docs/deferral-ledger.md"),
        ledger_cold,
        "the cold-promoted deferral-ledger is byte-stable",
    );
    assert_eq!(
        committed(repo.path(), "docs/decisions-log.md"),
        log_cold,
        "the cold-promoted decisions-log is byte-stable",
    );
    // BOTH slots of the roadmap milestone entry survived finalize (the two-slot
    // repeatable-item case Increment 1b unblocked).
    let roadmap_committed = committed(repo.path(), "docs/roadmap.md");
    assert!(
        roadmap_committed.contains("M-Alpha proves the cold create.")
            && roadmap_committed.contains("Inc 1: the alpha increment, as prose."),
        "both the `proves` and `decomposition` slots are committed; got:\n{roadmap_committed}",
    );

    // ── Run 2: a second milestone warm-re-creates the same committed singletons ─────
    let warm = "m-beta";
    start_planning(repo.path(), home.path(), "M-Beta");

    // The warm `create roadmap` copies the committed body in (the prior M-Alpha entry
    // survives), recording `edited-from-base` — not a blank mint.
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "roadmap", "--title", "Roadmap"],
            None,
        ),
        "`doc create roadmap` (warm copy-in)",
    );
    let after_copy_in = staged(repo.path(), warm, "roadmap:roadmap.md");
    assert!(
        after_copy_in.contains("M-Alpha proves the cold create."),
        "the warm create copies the committed roadmap in (prior milestone preserved); got:\n{after_copy_in}",
    );
    let provenance = staged(repo.path(), warm, "provenance.json");
    assert!(
        provenance.contains("\"roadmap:roadmap\": \"edited-from-base\""),
        "a warm singleton re-create records `edited-from-base`; got:\n{provenance}",
    );

    // Append the M-Beta milestone entry (BOTH slots) + the ledger/log entries. The
    // roadmap singleton is already copied-in above; the ledger/log warm-copy-in here.
    author_roadmap_entry(
        repo.path(),
        home.path(),
        "M-Beta",
        b"M-Beta proves the warm append.\n",
        b"Inc 1: the beta increment, as prose.\n",
    );
    create_singleton(
        repo.path(),
        home.path(),
        "deferral-ledger",
        "Deferral-Ledger",
    );
    author_ledger_entry(
        repo.path(),
        home.path(),
        "park the beta idea",
        "I",
        "M-Gamma",
        b"The beta idea is parked for M-Gamma.\n",
    );
    create_singleton(repo.path(), home.path(), "decisions-log", "Decisions-Log");
    author_decisions_entry(
        repo.path(),
        home.path(),
        "chose the beta shape",
        b"Because the beta shape composes.\n",
    );

    // The warm staged sources after the appends — the re-promote sources.
    let roadmap_warm = staged(repo.path(), warm, "roadmap:roadmap.md");
    let ledger_warm = staged(repo.path(), warm, "deferral-ledger:deferral-ledger.md");
    let log_warm = staged(repo.path(), warm, "decisions-log:decisions-log.md");

    fill_commit(repo.path(), home.path(), warm);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", warm]),
        "`task finalize` (run 2 warm)",
    );

    // Re-promoted byte-stable, each singleton carrying BOTH milestone runs' content.
    assert_eq!(
        committed(repo.path(), "docs/roadmap.md"),
        roadmap_warm,
        "the warm-re-promoted roadmap is byte-stable (committed == staged)",
    );
    assert_eq!(
        committed(repo.path(), "docs/deferral-ledger.md"),
        ledger_warm,
        "the warm-re-promoted deferral-ledger is byte-stable",
    );
    assert_eq!(
        committed(repo.path(), "docs/decisions-log.md"),
        log_warm,
        "the warm-re-promoted decisions-log is byte-stable",
    );

    let roadmap_final = committed(repo.path(), "docs/roadmap.md");
    assert!(
        roadmap_final.contains("M-Alpha proves the cold create.")
            && roadmap_final.contains("M-Beta proves the warm append."),
        "the warm-re-promoted roadmap carries BOTH milestone entries; got:\n{roadmap_final}",
    );
    let ledger_final = committed(repo.path(), "docs/deferral-ledger.md");
    assert!(
        ledger_final.contains("The alpha cleanup is owed to M-Beta.")
            && ledger_final.contains("The beta idea is parked for M-Gamma."),
        "the warm-re-promoted ledger carries BOTH entries; got:\n{ledger_final}",
    );
    let log_final = committed(repo.path(), "docs/decisions-log.md");
    assert!(
        log_final.contains("Because the alpha shape is simplest.")
            && log_final.contains("Because the beta shape composes."),
        "the warm-re-promoted decisions-log carries BOTH entries; got:\n{log_final}",
    );
}

#[test]
fn flow19_half_authored_roadmap_entry_blocks_at_validate() {
    // (3) The conformance red. A REAL roadmap milestone entry with a genuinely-empty
    // required `decomposition` slot (the `proves` slot is authored; `decomposition` is
    // left blank) must BLOCK finalize at `schema-conformance.required-slot-present` —
    // proving Increment 1's per-item conformance fires on THIS schema's leaves (the
    // vacuously-green-managed-doc-gate guard: the gate fires on a real empty leaf, not
    // a malformed fixture).
    let repo = TempDir::new("halfauthored");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let task = "m-half";
    start_planning(repo.path(), home.path(), "M-Half");
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "roadmap", "--title", "Roadmap"],
            None,
        ),
        "`doc create roadmap` (half-authored)",
    );
    let item = jigc_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            "roadmap:roadmap#milestones",
            "--title",
            "M-Half",
        ],
        None,
    );
    assert_ok(&item, "`doc add-item milestones` (half-authored)");
    let item_addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    // Author `proves` ONLY; leave `decomposition` genuinely empty.
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                &format!("{item_addr}/proves"),
                "--from-file",
                "-",
            ],
            Some(b"Only proves is authored.\n"),
        ),
        "`set-slot proves` (half-authored)",
    );

    fill_commit(repo.path(), home.path(), task);
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert!(
        !out.status.success(),
        "a half-authored roadmap entry must block finalize non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("schema-conformance.required-slot-present"),
        "the block is the required-slot-present conformance finding; got:\n{rendered}",
    );
    assert!(
        rendered.contains("decomposition"),
        "the finding names the empty `decomposition` slot (the half-authored leaf); got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a conformance block creates no commit");
}

#[test]
fn flow19_planning_is_absent_from_the_router_catalog() {
    // (4) The off-router red (the M8 catalog-leak class, the live front-door path).
    // Bare `jigc start` over the methodology pack orients to the clean-no-task state and
    // lists only the selectable (`creates-task: true && selectable: true`) work-workflows
    // — `planning` is `selectable: false`, so it must NOT appear in the catalog. Asserted
    // on the real binary's emitted bytes (human catalog lines + the JSON `workflows`
    // array), never a reconstruction.
    let repo = TempDir::new("router");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // Human emitted bytes: no catalog line names `planning`.
    let human = jigc(repo.path(), home.path(), &["start"]);
    assert_ok(&human, "bare `jigc start` orientation");
    let human_out = String::from_utf8(human.stdout).expect("utf-8 stdout");
    let catalog_lines: Vec<&str> = human_out
        .lines()
        .filter(|l| l.trim_start().starts_with("- "))
        .collect();
    assert!(
        !catalog_lines.iter().any(|l| l.contains("planning")),
        "the model-free selection catalog must NOT name `planning` (selectable: false, \
         off-router — the M8 catalog-leak class); catalog lines: {catalog_lines:?}",
    );

    // JSON emitted bytes: the `workflows` array carries no `planning` id.
    let json = jigc(repo.path(), home.path(), &["start", "--format", "json"]);
    assert_ok(&json, "bare `jigc start --format json` orientation");
    let json_out = String::from_utf8(json.stdout).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&json_out)
        .unwrap_or_else(|e| panic!("orientation must be valid JSON ({e}); got:\n{json_out}"));
    let ids: Vec<&str> = value["workflows"]
        .as_array()
        .unwrap_or_else(|| panic!("`workflows` must be an array; got:\n{json_out}"))
        .iter()
        .map(|w| w["id"].as_str().unwrap_or_default())
        .collect();
    assert!(
        !ids.contains(&"planning"),
        "the selectable catalog must NOT name `planning` (selectable: false, off-router); \
         got: {ids:?}",
    );
}

#[test]
fn flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize() {
    // (5) The warm-drift red. The cold finalize records the committed baseline FIRST;
    // the committed docs/roadmap.md is then drifted OUT-OF-BAND (a human edit + raw
    // git commit advances HEAD + the on-disk bytes but does NOT touch the recorded
    // file-state baseline, so it diverges → DRIFTED). A second planning task copies-in +
    // edits the roadmap (TOUCHED). finalize-preflight's `reconcile_committed_store` sees
    // DRIFTED+TOUCHED and conflict-blocks — both sides moved, no silent merge. (An
    // unrecorded baseline would baseline-adopt vacuously; recording-first is what makes
    // this a genuine red — the singleton_running_doc.rs lesson.)
    let repo = TempDir::new("drift");
    let home = TempDir::new("home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    // ── Run 1: cold-create the roadmap + finalize → records the committed baseline ──
    let cold = "m-base";
    start_planning(repo.path(), home.path(), "M-Base");
    create_singleton(repo.path(), home.path(), "roadmap", "Roadmap");
    author_roadmap_entry(
        repo.path(),
        home.path(),
        "M-Base",
        b"M-Base proves the baseline.\n",
        b"Inc 1: the baseline increment.\n",
    );
    fill_commit(repo.path(), home.path(), cold);
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", cold]),
        "`task finalize` (drift/run1 cold) — records the baseline FIRST",
    );

    // OOB drift: a human hand-edits the committed roadmap and raw-commits it. Drift
    // BEFORE the warm task starts so the warm task's base == this drifted HEAD (no
    // `finalize.base-mismatch`, which would pre-empt the reconcile gate).
    let committed_path = repo.path().join("docs").join("roadmap.md");
    let on_disk = fs::read_to_string(&committed_path).expect("read committed roadmap on disk");
    fs::write(
        &committed_path,
        on_disk.replace(
            "M-Base proves the baseline.",
            "An out-of-band human edit to the roadmap.",
        ),
    )
    .expect("apply OOB drift");
    git(repo.path(), &["add", "docs/roadmap.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "oob: hand-edit the roadmap"],
    );

    // ── Run 2: a warm planning task touches the roadmap (copy-in via create + edit) ─
    let warm = "m-drift";
    start_planning(repo.path(), home.path(), "M-Drift");
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "roadmap", "--title", "Roadmap"],
            None,
        ),
        "`doc create roadmap` (drift/run2 warm copy-in)",
    );
    let item = jigc_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            "roadmap:roadmap#milestones",
            "--title",
            "M-Drift",
        ],
        None,
    );
    assert_ok(&item, "`doc add-item milestones` (drift/run2)");
    let item_addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                &format!("{item_addr}/proves"),
                "--from-file",
                "-",
            ],
            Some(b"M-Drift proves the warm touch.\n"),
        ),
        "`set-slot proves` (drift/run2 — the task TOUCHED the roadmap)",
    );
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &[
                "set-slot",
                &format!("{item_addr}/decomposition"),
                "--from-file",
                "-",
            ],
            Some(b"Inc 1: the drift increment.\n"),
        ),
        "`set-slot decomposition` (drift/run2)",
    );

    // finalize-preflight: DRIFTED (committed != recorded baseline) + TOUCHED (staged in
    // the warm area) → conflict-block, non-zero exit, NO new commit.
    fill_commit(repo.path(), home.path(), warm);
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", warm]);
    assert!(
        !out.status.success(),
        "a DRIFTED+TOUCHED singleton must conflict-block finalize non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("reconciliation.conflict-block"),
        "the block is the `reconciliation.conflict-block` finding; got:\n{rendered}",
    );
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap();
    assert_eq!(before, after, "a conflict-block creates no commit");
}
