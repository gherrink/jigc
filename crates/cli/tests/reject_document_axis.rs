//! The **reject document** — M52 Increment 1 / T1 (`completions/artifacts/M52/settle-record.md`
//! → D6 as amended by §3; `design/command-output-contract.md` → Stream discipline, the reject
//! row).
//!
//! The contract has said since M45 that on a **reject** *stderr carries exactly one document*
//! and stdout is empty. The binary did not: a committing door whose commit a hook refused
//! emitted the flattened `{"error": …}` envelope and then printed **beside** it — the pre-commit
//! advisories above it, each rollback conflict below it — so `json.loads(stderr)` raised, and a
//! driver that reached for `raw_decode` silently dropped the only line naming the parked
//! pre-image of its own raced bytes (`advocates/F6.md` spike 1, driven on `1.0.0-rc.15`).
//!
//! The rule this suite fences, stated once in the contract and applied here: **under
//! `--format json` the stream that carries the document carries nothing else**. On a reject
//! the document is the `{findings, schema_version}` envelope, the operational error is a
//! finding in it (the door's own `*.commit-rejected` identity — the one the invocation log
//! already records), and everything that would have printed beside it folds in or is withheld
//! with its reason.
//!
//! The arms, each driving the real binary:
//!
//!   1. [`every_committing_door_rejects_as_one_findings_document`] — the axis, read from
//!      `cli::invocation_log::COMMITTING_DOORS` (never hand-listed), under a rejecting
//!      `pre-commit` hook: stdout is **0 bytes**, stderr parses **whole** (not `raw_decode`),
//!      and the door's own identity is in it with `key.code` and `key.target` both non-null.
//!   2. [`a_raced_config_layer_file_rides_the_same_document`] — the driven defect: a hook that
//!      edits `.jigc/.gitignore` and exits 1 leaves the rollback unable to restore it, and the
//!      conflict **and** the parked pre-image path are inside the one document.
//!   3. [`an_untracked_file_never_breaks_the_reject_document`] — the same ten doors with an
//!      untracked file present, so `emit_left_out_advisory` has something to say. At HEAD this
//!      is the sharpest cell: the advisory prints *above* the envelope, so the parse fails at
//!      **char 0**.
//!   4. [`the_carried_advisory_and_the_hook_output_fold_into_the_document`] — the two remaining
//!      `Format`-routing producers: a `--carry-staged` run (the carried-over print) under a
//!      hook that also speaks on **stdout** (the relay's source bytes).
//!   5. [`config_set_docs_root_emits_one_document_and_no_prose`] — the fifth producer, whose
//!      fact is carried by the `relocated` key the ack already declares.
//!   6. [`the_text_arm_of_a_rejection_still_frames_it`] — the negative cell: agent-text is
//!      untouched by all of the above (the goldens and `commit_rejected_axis` are the
//!      assertion; this is the cheap cross-check that the two arms were split, not swapped).
//!   7. [`setup_and_uninstall_reject_on_the_declared_findings_arm`] — M52 Increment 1 / T2:
//!      the **third root shape**. `setup` and `uninstall` are the only doors in the binary
//!      whose `--format json` reject was neither of the two declared arms — they serialized a
//!      **bare `Finding`** at the root (`render::setup_block`'s `Format::Json` line, its two
//!      call sites `cli.rs`'s `run_setup` / `run_uninstall`), an undeclared shape no
//!      `ENVELOPE_ARMS` row described. Both cells are driven here, and both doors' agent-text
//!      frame — the routing footer included — is cross-checked in the same arm.
//!
//! **The bare-`Finding` root is asserted dead by driving, not by grepping.** [`findings_of`]
//! — which every arm above calls on the document it just parsed — refuses a root that is a
//! serialized `Finding`, so the claim *no leaf verb emits one* is made over each reject this
//! suite actually provokes rather than over a source pattern. The class is closed at its
//! producer: the shape had exactly one (`setup_block`'s json line, now deleted) and exactly
//! two call sites, and arm 7 drives both. A grep would have proved the source; this proves
//! the bytes.
//!
//! **Non-vacuity.** Arm 1's `key.target` clause fails at HEAD for all ten doors (there is no
//! findings arm at all), arm 3 fails at char 0, which is the shape the trial reported, and
//! arm 7 fails at `schema_version` for both doors — driven on `1.0.0-rc.15` and again at this
//! increment's own base, each emitting `{"severity": …, "probe": "setup", …}` at the root.

use std::fs;
use std::path::Path;

use cli::invocation_log::COMMITTING_DOORS;
use serde_json::Value;

use crate::support::committing_doors::{
    DoorCase, HOOK_MARKER, TempDir, base_repo, drive, git, jigc, jigc_ok, seed_task,
};

/// Parse the **whole** stream as one JSON document — never `raw_decode`, because "the
/// document plus some bytes" is exactly the defect.
fn one_document(stream: &str, what: &str) -> Value {
    serde_json::from_str(stream.trim()).unwrap_or_else(|err| {
        panic!("{what} must be exactly one JSON document ({err});\nstream:\n{stream}")
    })
}

/// The findings array of a reject document, with the envelope's own shape asserted — and
/// with the **third root shape** refused (M52 Increment 1 / T2).
///
/// A serialized [`engine::finding::Finding`] at the root is not one of the two declared reject
/// arms: it carries the finding's own `severity` / `probe` / `check` / `code` keys where the
/// envelope carries `findings`, so a driver that parses either declared arm reads nothing it
/// can use. `setup` and `uninstall` were its only producers; checking it here rather than in
/// their arm alone is what makes *no leaf verb emits one* a claim over every reject this suite
/// drives.
fn findings_of(doc: &Value, what: &str) -> Vec<Value> {
    assert!(
        doc.get("code").is_none() && doc.get("severity").is_none(),
        "{what}: a reject document is never a bare `Finding` serialized at the root — that \
         shape is neither declared reject arm and no `ENVELOPE_ARMS` row describes it; \
         got:\n{doc:#}",
    );
    assert!(
        doc.get("schema_version").is_some(),
        "{what}: the reject document is the findings envelope — it carries `schema_version`; \
         got:\n{doc:#}",
    );
    doc["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{what}: the envelope carries a `findings` array; got:\n{doc:#}"))
        .clone()
}

/// The one finding carrying `code`, with the **identity** half of the stable key asserted on
/// it: `key.code` is non-null and is this code.
///
/// The *target* half is [`keyed`]'s, because it does not hold for every keyed finding — a
/// **declared singleton** keys at `target: null` by the pin (`design/command-output-contract.md`
/// → The declared singleton exception), which is what every `setup.*` / `uninstall.*` code is:
/// both doors are fail-fast `Result<_, Finding>`, so two instances of one code cannot coexist
/// in one output and there is nothing for a target to discriminate.
fn keyed_identity(findings: &[Value], code: &str, what: &str) -> Value {
    let hit = findings
        .iter()
        .find(|f| f["code"].as_str() == Some(code))
        .unwrap_or_else(|| {
            panic!("{what}: the document must carry `{code}`; findings:\n{findings:#?}")
        })
        .clone();
    assert_eq!(
        hit["key"]["code"].as_str(),
        Some(code),
        "{what}: `{code}` projects its own `key.code`; got:\n{hit:#}",
    );
    hit
}

/// [`keyed_identity`] plus the **discriminating** half: a code the contract lists under a
/// target form owes a non-null `key.target`.
fn keyed(findings: &[Value], code: &str, what: &str) -> Value {
    let hit = keyed_identity(findings, code, what);
    assert!(
        hit["key"]["target"].as_str().is_some(),
        "{what}: `{code}` must project a non-null `key.target` — a code inside a message is \
         not a key (design/command-output-contract.md → The membership test); got:\n{hit:#}",
    );
    hit
}

/// Drive one door's fixture with `--format json` in front of its own argv.
fn reject_json(case: &DoorCase) -> (Vec<u8>, String) {
    let mut argv = vec!["--format".to_string(), "json".to_string()];
    argv.extend(case.driven.iter().cloned());
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let out = jigc(case.repo.path(), case.home.path(), &args, None);
    assert!(
        !out.status.success(),
        "a rejected commit exits non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    (
        out.stdout,
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// **Arm 1 — the axis.** Every code-side committing door, rejected by a hook, answers
/// `--format json` with exactly one findings document naming itself.
#[test]
fn every_committing_door_rejects_as_one_findings_document() {
    assert_eq!(
        COMMITTING_DOORS.len(),
        11,
        "the axis is the registry, not a hand list — `jigc setup` stays excluded by its \
         recorded `--no-verify` reason",
    );
    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case = drive(verb);
        let (stdout, stderr) = reject_json(&case);

        assert!(
            stdout.is_empty(),
            "[{verb}] a reject leaves stdout empty — the document owns stderr; stdout:\n{}",
            String::from_utf8_lossy(&stdout),
        );
        let doc = one_document(&stderr, &format!("[{verb}] the reject stream"));
        let findings = findings_of(&doc, verb);
        let hit = keyed(&findings, door.error_code, verb);

        // The hook's own bytes survive verbatim — inside the document now, never beside it.
        assert!(
            hit["message"]
                .as_str()
                .is_some_and(|m| m.contains(HOOK_MARKER)),
            "[{verb}] the hook's bytes are the finding's message, verbatim; got:\n{hit:#}",
        );
        // The state-truth clause and the door's own re-run ride the route.
        let route = hit["route"].as_str().unwrap_or_else(|| {
            panic!("[{verb}] a blocking finding names its recovery; got:\n{hit:#}")
        });
        assert!(
            route.contains(&case.survived),
            "[{verb}] the route states what survived (expected `{}`); got:\n{route}",
            case.survived,
        );
        let printed_rerun = case.expected_rerun.join(" ");
        assert!(
            route.contains(case.expected_rerun[1].as_str()) && route.contains("re-run"),
            "[{verb}] the route names this door's own re-run (`{printed_rerun}`); got:\n{route}",
        );
        // The `{error}` arm is gone from this cell — one shape per reject, never two.
        assert!(
            doc.get("error").is_none(),
            "[{verb}] a reject carrying a finding takes the findings arm alone; got:\n{doc:#}",
        );
    }
}

/// Install a `pre-commit` hook that **edits `.jigc/.gitignore`** and rejects — the roadmap's
/// own Proves cell, and the shape `advocates/F6.md` drove: the finalize transaction rewrote
/// that file, so the compare-and-swap rollback refuses to restore it and parks its pre-image.
fn install_ignore_racing_hook(repo: &Path) {
    let hook = repo.join(".git").join("hooks").join("pre-commit");
    fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks dir");
    fs::write(
        &hook,
        format!(
            "#!/bin/sh\nprintf 'raced-by-the-hook/\\n' >> .jigc/.gitignore\n\
             echo '{HOOK_MARKER}' 1>&2\nexit 1\n"
        ),
    )
    .expect("write the racing pre-commit hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }
}

/// Shrink the committed `.jigc/.gitignore` by one canonical entry, so this finalize's
/// `gitignore::ensure` genuinely **rewrites** it — the precondition for the rollback having
/// anything to put back.
fn shrink_the_ignore_set(repo: &Path) {
    let path = repo.join(".jigc").join(".gitignore");
    let body = fs::read_to_string(&path).expect("read .jigc/.gitignore");
    let shrunk: String = body
        .lines()
        .filter(|line| line.trim() != "displaced/")
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(body, shrunk, "the fixture must actually remove an entry");
    fs::write(&path, shrunk).expect("write the shrunken ignore set");
    git(repo, &["add", ".jigc/.gitignore"]);
    git(repo, &["commit", "-q", "-m", "shrink the ignore set"]);
}

/// **Arm 2 — the driven defect.** The rollback conflict and the path to the parked pre-image
/// are inside the one document, not on 459 bytes of prose after it.
#[test]
fn a_raced_config_layer_file_rides_the_same_document() {
    let (repo, home) = base_repo("reject-doc-raced", None);
    shrink_the_ignore_set(repo.path());
    let task = seed_task(repo.path(), home.path(), "survive the race");
    fs::write(repo.path().join("code.txt"), "the task's work\n").expect("write code.txt");
    git(repo.path(), &["add", "code.txt"]);
    install_ignore_racing_hook(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", &task],
        None,
    );
    assert!(!out.status.success(), "the raced finalize exits non-zero");
    assert!(out.stdout.is_empty(), "a reject leaves stdout empty");
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let doc = one_document(&stderr, "the raced reject stream");
    let findings = findings_of(&doc, "the raced finalize");

    keyed(&findings, "finalize.commit-rejected", "the raced finalize");
    let conflict = keyed(
        &findings,
        "finalize.rollback-conflict",
        "the raced finalize",
    );
    assert_eq!(
        conflict["key"]["target"].as_str(),
        Some(".jigc/.gitignore"),
        "the conflict keys on the raced path; got:\n{conflict:#}",
    );
    assert!(
        conflict["route"]
            .as_str()
            .is_some_and(|r| r.contains(".jigc/displaced/")),
        "the parked pre-image path is INSIDE the document — it is the only naming of the \
         user's own bytes; got:\n{conflict:#}",
    );
}

/// **Arm 3 — the pre-commit advisories.** With an untracked file present the left-out advisory
/// has something to say, and at HEAD it says it *above* the envelope, so the parse fails at
/// char 0. The document still parses whole.
#[test]
fn an_untracked_file_never_breaks_the_reject_document() {
    for door in COMMITTING_DOORS {
        let verb = door.verb;
        let case = drive(verb);
        fs::write(case.repo.path().join("private-wip.txt"), "private WIP\n")
            .expect("write the untracked file");
        let (stdout, stderr) = reject_json(&case);
        assert!(
            stdout.is_empty(),
            "[{verb}] a reject leaves stdout empty even with WIP around; stdout:\n{}",
            String::from_utf8_lossy(&stdout),
        );
        let doc = one_document(
            &stderr,
            &format!("[{verb}] the reject stream beside an untracked file"),
        );
        keyed(&findings_of(&doc, verb), door.error_code, verb);
    }
}

/// **Arm 4 — the carried-over print and the hook relay.** A `--carry-staged` finalize under a
/// hook that speaks on **stdout** as well as stderr: the carried print has something to say,
/// the hook's bytes are the relay's own source, and the reject is still one document.
#[test]
fn the_carried_advisory_and_the_hook_output_fold_into_the_document() {
    let (repo, home) = base_repo("reject-doc-carried", None);
    // Staged BEFORE the mint — the carryover gate's subject, declared past with `--carry-staged`.
    fs::write(repo.path().join("carried.txt"), "staged before the task\n")
        .expect("write carried.txt");
    git(repo.path(), &["add", "carried.txt"]);
    let task = seed_task(repo.path(), home.path(), "carry the staged set");
    fs::write(repo.path().join("code.txt"), "the task's work\n").expect("write code.txt");
    git(repo.path(), &["add", "code.txt"]);

    let hook = repo.path().join(".git").join("hooks").join("pre-commit");
    fs::write(
        &hook,
        format!("#!/bin/sh\necho 'hook chatter on stdout'\necho '{HOOK_MARKER}' 1>&2\nexit 1\n"),
    )
    .expect("write the chatty rejecting hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&hook, fs::Permissions::from_mode(0o755)).expect("chmod hook");
    }

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "--format",
            "json",
            "task",
            "finalize",
            &task,
            "--carry-staged",
        ],
        None,
    );
    assert!(!out.status.success(), "the rejected carry exits non-zero");
    assert!(
        out.stdout.is_empty(),
        "a reject leaves stdout empty — the carried-over print does not take it; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let doc = one_document(&stderr, "the carried reject stream");
    let hit = keyed(
        &findings_of(&doc, "the carried finalize"),
        "finalize.commit-rejected",
        "the carried finalize",
    );
    assert!(
        hit["message"]
            .as_str()
            .is_some_and(|m| m.contains("hook chatter on stdout")),
        "the hook's whole captured stream reaches the document; got:\n{hit:#}",
    );
}

/// **Arm 5 — `config set`'s relocation prose.** Its fact is the ack's own `relocated` key
/// (M51), so under `--format json` the prose is not printed beside the document.
#[test]
fn config_set_docs_root_emits_one_document_and_no_prose() {
    let (repo, home) = base_repo("reject-doc-config", None);
    // A committed doc under the current `docs-root`, so the re-point has something to relocate.
    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(
        dir.join("keep-sessions-local.md"),
        "---\nstatus: accepted\ndate: 2026-09-17\nschema-version: 2\n---\n\n\
         # Keep sessions local\n\n## Context\n\nLookups must stay fast.\n\n## Options\n\n\
         A distributed cache was weighed and rejected.\n\n## Decision\n\nKeep sessions in one \
         node.\n\n## Consequences\n\nA cold node loses its sessions.\n",
    )
    .expect("write the adr");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed the adr"]);

    let out = jigc_ok(
        repo.path(),
        home.path(),
        &["--format", "json", "config", "set", "docs-root", "records"],
        "`jigc config set docs-root`",
    );
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let ack = one_document(&stdout, "the `config set` ack");
    assert!(
        ack["relocated"]
            .as_array()
            .is_some_and(|moves| !moves.is_empty()),
        "the moves ride the ack's declared `relocated` key; got:\n{ack:#}",
    );
    assert!(
        stderr.trim().is_empty(),
        "under `--format json` the document's sibling stream carries no prose — the \
         relocation narration folds into `relocated`; stderr:\n{stderr}",
    );
}

/// **Arm 6 — the negative cell.** The agent-text arm is untouched: git's bytes verbatim, the
/// state clause, the re-run — the frame `commit_rejected_axis` pins, byte-for-byte.
#[test]
fn the_text_arm_of_a_rejection_still_frames_it() {
    let case = drive("jigc task finalize");
    let args: Vec<&str> = case.driven.iter().map(String::as_str).collect();
    let out = jigc(case.repo.path(), case.home.path(), &args, None);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        stderr.contains(HOOK_MARKER) && stderr.contains(&case.survived),
        "the text arm keeps the survivable frame verbatim; stderr:\n{stderr}",
    );
    assert!(
        !stderr.trim_start().starts_with('{'),
        "the text arm is prose, never the document; stderr:\n{stderr}",
    );
}

/// A directory with **no `.git` anywhere above it**, and a `$HOME` of its own — the state
/// `jigc setup` refuses with `setup.repo-root`. The system temp dir is not inside a
/// repository (the same ground `not_in_repo_axis` stands on).
fn outside_any_repository() -> (TempDir, TempDir) {
    (
        TempDir::new("reject-doc-no-repo"),
        TempDir::new("reject-doc-no-repo-home"),
    )
}

/// **Arm 7 — the third root shape.** `setup` and `uninstall` were the binary's only doors
/// whose `--format json` reject was neither declared arm: they serialized a bare `Finding`
/// at the root (M52 Increment 1 / T2; `settle-record.md` → D6.2 as amended by §3).
///
/// Both cells are the doors' own guards, not manufactured faults: `setup` outside a git
/// repository, and `uninstall` over a `.jigc/` holding a file no index has a copy of — the
/// M50 destroying-door guard, whose whole point is that the user's bytes are named before
/// anything is removed. That naming is what the bare root cost a driver: the route carrying
/// `git add <path>` was reachable only by a consumer that knew this one undeclared shape.
///
/// Per cell: exit non-zero · stdout **0 bytes** · stderr parses **whole** into the
/// `{findings, schema_version}` envelope · the door's own code with a non-null `key.code`
/// (`key.target` is null by the declared-singleton pin, which is why this arm keys on the
/// identity half) · no `error` key, because one reject takes one arm.
///
/// And the **named regression risk**, cross-checked in the same arm: the agent-text frame is
/// the house finding line **plus the routing footer**. The footer is the half a naive move
/// onto the shared operational funnel would have dropped, so it is asserted on the emitted
/// bytes here as well as at the render seam.
#[test]
fn setup_and_uninstall_reject_on_the_declared_findings_arm() {
    // --- `jigc setup` outside a git repository ---------------------------------------
    let (nowhere, nowhere_home) = outside_any_repository();
    let out = jigc(
        nowhere.path(),
        nowhere_home.path(),
        &["--format", "json", "setup"],
        None,
    );
    assert!(
        !out.status.success(),
        "`jigc setup` outside a repository refuses; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.stdout.is_empty(),
        "[setup] a reject leaves stdout empty — the document owns stderr; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let doc = one_document(&stderr, "[setup] the reject stream");
    let findings = findings_of(&doc, "setup");
    keyed_identity(&findings, "setup.repo-root", "setup");
    assert!(
        doc.get("error").is_none(),
        "[setup] a reject carrying a finding takes the findings arm alone; got:\n{doc:#}",
    );

    // --- `jigc uninstall` over an untracked workbench file ----------------------------
    let (repo, home) = base_repo("reject-doc-uninstall", None);
    fs::write(
        repo.path().join(".jigc").join("stray.txt"),
        "unsaved work\n",
    )
    .expect("write the untracked workbench file");
    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "uninstall"],
        None,
    );
    assert!(
        !out.status.success(),
        "`jigc uninstall` over unsaved workbench bytes refuses; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.stdout.is_empty(),
        "[uninstall] a reject leaves stdout empty; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let doc = one_document(&stderr, "[uninstall] the reject stream");
    let findings = findings_of(&doc, "uninstall");
    let hit = keyed_identity(&findings, "uninstall.untracked-workbench-file", "uninstall");
    assert!(
        hit["message"]
            .as_str()
            .is_some_and(|m| m.contains(".jigc/stray.txt")),
        "[uninstall] the guard names the bytes it would destroy; got:\n{hit:#}",
    );
    assert!(
        hit["route"]
            .as_str()
            // Aimed since M53 (the cwd census, C1-11): the recovery runs in the checkout
            // the workbench belongs to, from whatever directory the reader is standing in.
            .is_some_and(|r| r.contains(" add -- <path>") && r.contains("git -C /")),
        "[uninstall] the recovery move is data on the route, not prose in an error string; \
         got:\n{hit:#}",
    );
    assert!(
        doc.get("error").is_none(),
        "[uninstall] a reject carrying a finding takes the findings arm alone; got:\n{doc:#}",
    );

    // --- the text arm of both doors, frame and footer intact --------------------------
    for (label, repo, home, args, code) in [
        (
            "setup",
            nowhere.path(),
            nowhere_home.path(),
            ["setup"],
            "setup.repo-root",
        ),
        (
            "uninstall",
            repo.path(),
            home.path(),
            ["uninstall"],
            "uninstall.untracked-workbench-file",
        ),
    ] {
        let out = jigc(repo, home, &args, None);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            stderr.starts_with(&format!("blocking · {code} ")),
            "[{label}] the text arm opens with the house finding line; stderr:\n{stderr}",
        );
        assert!(
            stderr.ends_with(&format!("{}\n", cli::render::ROUTING_FOOTER)),
            "[{label}] the text arm ends with its routing footer and exactly one newline — \
             the half a move onto the shared funnel would silently drop; stderr:\n{stderr}",
        );
        assert!(
            !stderr.trim_start().starts_with('{'),
            "[{label}] the text arm is prose, never the document; stderr:\n{stderr}",
        );
    }
}
