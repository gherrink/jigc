//! Acceptance — **`completion-record` 1 → 2: the audit vocabulary and prose evidence,
//! in one bump** (M49 Increment 9, T1; `completions/artifacts/M49/settle-record.md` →
//! D10 · Tier 3; `design/corpus-migration.md` → the classifier's kinds
//! (`EnumWidened`, `AddedItemSlot`) · Prior-schema sourcing;
//! `implementation/doctype-authoring.md` → Freeze / versioning).
//!
//! The doctype the methodology's own completion audit writes shipped a `severity`
//! enum of exactly `[blocking, advisory]` — and **every real audit this repo has ever
//! run grades HIGH / MEDIUM / LOW**. So the shipped record could not record the words
//! its own audits use: `jigc doc set-field …/severity --value HIGH` exited **1** at
//! `schema-conformance.field-value-conformant`. The second half is the same shape one
//! leaf over: a finding's `evidence` is a one-line **string** (a `file:line`, a repro
//! command), and there was nowhere to put the paragraph that says what the finding
//! actually was — the item block declared **zero** slots.
//!
//! The bump is one schema-version, carrying two changes, each a kind the classifier
//! already names:
//!
//!   * `severity.of` becomes the **superset** `[blocking, advisory, HIGH, MEDIUM, LOW]`
//!     — `EnumWidened` (`new.of ⊇ old.of`), a **byte no-op**: every committed value is
//!     still a declared member and still means what it meant. A member *replacement*
//!     would be `ValueRemapped`, needing an authored old→new map, and `blocking → ?`
//!     has no defensible answer — which is exactly why the shipped pair is kept.
//!   * an **optional `detail` slot** joins the `findings` item block — `AddedItemSlot`
//!     at the **0 → 1** arity (M49 Increment 4), where a lone declared slot renders
//!     *bare*, under no sub-heading, so the fold writes **zero** bytes. Converting
//!     `evidence` instead would have been `RemovedField` + `AddedItemSlot`, and
//!     `RemovedField` is refused by design (`corpus-migration.md`:188).
//!
//! Every arm drives the **shipped binary** against a throwaway `[dev ▸ methodology]`
//! repo — the real embedded packs, the real freeze manifest, the real snapshot store —
//! and asserts the **bytes on disk** or the **emitted JSON**, never a reconstruction.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-completion-record-v2-{tag}-{}-{:?}",
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

/// A real git repo with one commit and the `[dev ▸ methodology]` compose marker — the
/// exact project-layer key that composes the embedded methodology pack over the dev
/// pack, so the `completion-record` under test is the **shipped** one.
fn init_repo(repo: &Path) {
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
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

// ---------------------------------------------------------------------------------------------
// (1) The corpus migration — the stamp is the only byte the fold moves.
// ---------------------------------------------------------------------------------------------

/// A conformant, **v1-stamped** committed `completion-record`: one slot-less finding
/// item graded on the v1 vocabulary. This is the exact shape every record committed
/// before this bump is in.
const RECORD_V1: &str = "\
---
verdict: green
owner-artifact: completions/artifacts/M1/VERDICT.md
schema-version: 1
---

# M1

## Findings

### A stray finding  {#a-stray-finding}

<!-- fields -->
- severity: advisory
- disposition: fixed
- evidence: audit.log:12
";

/// **The migration.** A committed `schema-version: 1` record folds to 2 under
/// `jigc migrate-corpus` — reported as migrated, and **byte-identical except the
/// stamp**: `EnumWidened` is a no-op by construction, and `AddedItemSlot` at the 0→1
/// arity writes nothing because a lone slot renders bare.
///
/// RED before the bump: the shipped doctype was itself at v1, so the doc was reported
/// `already-current` and `migrated[]` was empty.
#[test]
fn a_committed_v1_record_migrates_with_the_stamp_as_its_only_byte_delta() {
    let repo = TempDir::new("migrate");
    let home = TempDir::new("migrate-home");
    init_repo(repo.path());

    // The file name is the **slug** the mint derives from the title (`slugify("M1") == "m1"`),
    // never the title itself — a hand-written `M1.md` is an identity no jigc operation
    // produces, and since M50 Inc 2 / T1 the store sweep says so.
    let record = repo.path().join("docs").join("completions").join("m1.md");
    fs::create_dir_all(record.parent().expect("a parent")).expect("mk completions/");
    fs::write(&record, RECORD_V1).expect("write the v1 record");
    let artifact = repo
        .path()
        .join("completions")
        .join("artifacts")
        .join("M1")
        .join("VERDICT.md");
    fs::create_dir_all(artifact.parent().expect("a parent")).expect("mk artifact home");
    fs::write(&artifact, "the verdict\n").expect("write the owner-artifact");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed the v1 record"]);

    let out = jigc(
        repo.path(),
        home.path(),
        &["migrate-corpus", "--format", "json"],
    );
    let stdout = assert_ok(&out, "jigc migrate-corpus over a v1 completion-record");
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the report is JSON ({e}); stdout:\n{stdout}"));
    let migrated: Vec<&str> = report["migrated"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries `migrated[]`; got:\n{report:#}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    assert_eq!(
        migrated,
        ["docs/completions/m1.md"],
        "exactly the v1 record migrates; report:\n{report:#}"
    );

    let after = fs::read_to_string(&record).expect("read the migrated record");
    assert_eq!(
        after,
        RECORD_V1.replace("schema-version: 1", "schema-version: 2"),
        "the stamp is the migration's only byte delta: a widened enum admits every \
         committed value unchanged, and a lone added slot renders bare",
    );
    assert_ok(
        &jigc(repo.path(), home.path(), &["validate"]),
        "jigc validate over the migrated corpus",
    );
}

// ---------------------------------------------------------------------------------------------
// (2) The audit vocabulary — settable through the real write path.
// ---------------------------------------------------------------------------------------------

/// The record slug the completion workflow's create mints from the title below.
const SLUG: &str = "m2";

/// The `findings` item address the arm below writes into.
const ITEM: &str = "completion-record:m2#findings/stray-finding";

/// Mint a completion task and stage a `completion-record` carrying one finding item.
fn staged_record(repo: &Path, home: &Path) -> String {
    let stdout = assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "completion",
                "--format",
                "json",
                "M2",
            ],
        ),
        "jigc start --workflow completion",
    );
    let composed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the composed envelope is JSON ({e}); stdout:\n{stdout}"));
    let task = composed["task"]
        .as_str()
        .unwrap_or_else(|| panic!("the composed envelope carries a task id; got:\n{composed:#}"))
        .to_string();

    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "create",
                "completion-record",
                "--title",
                "M2",
                "--task",
                &task,
            ],
        ),
        "jigc doc create completion-record",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                &format!("completion-record:{SLUG}#findings"),
                "--title",
                "Stray finding",
                "--task",
                &task,
            ],
        ),
        "jigc doc add-item findings",
    );
    task
}

/// **The audit vocabulary is settable, and the shipped pair still is.** Each of the
/// five declared members lands through the real `set-field` write path at exit 0, and
/// a non-member is still refused at `schema-conformance.field-value-conformant` — a
/// widening that stopped rejecting anything would be an open string, not an enum.
///
/// RED before the bump: `HIGH` / `MEDIUM` / `LOW` each exited 1 at that same code.
#[test]
fn every_declared_severity_lands_and_a_non_member_is_still_refused() {
    let repo = TempDir::new("severity");
    let home = TempDir::new("severity-home");
    init_repo(repo.path());
    let task = staged_record(repo.path(), home.path());

    for value in ["blocking", "advisory", "HIGH", "MEDIUM", "LOW"] {
        let out = jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-field",
                &format!("{ITEM}/severity"),
                "--value",
                value,
                "--task",
                &task,
            ],
        );
        assert_ok(&out, &format!("set-field severity --value {value}"));
        let shown = assert_ok(
            &jigc(
                repo.path(),
                home.path(),
                &[
                    "doc",
                    "show",
                    &format!("{ITEM}/severity"),
                    "--task",
                    &task,
                    "--format",
                    "json",
                ],
            ),
            "jigc doc show of the staged severity",
        );
        assert!(
            shown.contains(value),
            "the staged read must carry the value just written (`{value}`); got:\n{shown}",
        );
    }

    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{ITEM}/severity"),
            "--value",
            "CRITICAL",
            "--task",
            &task,
        ],
    );
    assert!(
        !refused.status.success(),
        "an undeclared member is still refused; stdout:\n{}",
        String::from_utf8_lossy(&refused.stdout),
    );
    let streams = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr),
    );
    assert!(
        streams.contains("schema-conformance.field-value-conformant"),
        "the refusal keeps its shipped code; got:\n{streams}",
    );
}

/// **The prose leaf.** The added `detail` slot is a real, optional, author-owned item
/// leaf: `set-slot` writes it, the staged read serves it back, and — being optional —
/// it never becomes a required-slot gate on a record that leaves it empty.
#[test]
fn the_detail_slot_takes_prose_and_never_gates_when_left_empty() {
    let repo = TempDir::new("detail");
    let home = TempDir::new("detail-home");
    init_repo(repo.path());
    let task = staged_record(repo.path(), home.path());

    let prose = repo.path().join("detail.txt");
    fs::write(&prose, "The probe compared the wrong pair of paths.\n").expect("write prose");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "set-slot",
                &format!("{ITEM}/detail"),
                "--from-file",
                prose.to_str().expect("utf-8 path"),
                "--task",
                &task,
            ],
        ),
        "jigc doc set-slot detail",
    );
    let shown = assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "show", &format!("{ITEM}/detail"), "--task", &task],
        ),
        "jigc doc show of the staged detail slot",
    );
    assert!(
        shown.contains("The probe compared the wrong pair of paths."),
        "the staged read serves the prose just written; got:\n{shown}",
    );
}

// ---------------------------------------------------------------------------------------------
// (3) The read contract — the item block stops projecting an empty slot set.
// ---------------------------------------------------------------------------------------------

/// **`jigc doc schema` projects `detail`.** The pinned `--format json` contract is the
/// surface an agent reads to learn what it may write; before the bump the `findings`
/// item block projected `"slots": []`, so the prose leaf would have been invisible
/// even once it shipped.
#[test]
fn the_pinned_schema_projection_carries_the_optional_detail_slot() {
    let repo = TempDir::new("schema");
    let home = TempDir::new("schema-home");
    init_repo(repo.path());

    let stdout = assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "schema", "completion-record", "--format", "json"],
        ),
        "jigc doc schema completion-record --format json",
    );
    let projected: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("the projection is JSON ({e}); stdout:\n{stdout}"));

    assert_eq!(
        projected["schema-version"], 2,
        "the projection states the shipped schema-version; got:\n{projected:#}",
    );
    let findings = projected["sections"]
        .as_array()
        .and_then(|s| s.iter().find(|s| s["id"] == "findings"))
        .unwrap_or_else(|| panic!("the projection carries `findings`; got:\n{projected:#}"))
        .clone();
    let slots = findings["item"]["slots"]
        .as_array()
        .unwrap_or_else(|| panic!("the item block projects `slots[]`; got:\n{findings:#}"));
    assert_eq!(
        slots.len(),
        1,
        "the item block declares exactly the one added slot; got:\n{findings:#}",
    );
    assert_eq!(slots[0]["id"], "detail");
    assert_eq!(
        slots[0]["optional"], true,
        "`detail` is optional — a record that leaves it empty must still finalize",
    );
    assert_eq!(
        slots[0]["set-slot"], "completion-record:<slug>#findings/<id>/detail",
        "the projection advertises the write address the write path accepts",
    );

    let severity = findings["item"]["fields"]
        .as_array()
        .and_then(|f| f.iter().find(|f| f["id"] == "severity"))
        .unwrap_or_else(|| panic!("the item block carries `severity`; got:\n{findings:#}"))
        .clone();
    assert_eq!(
        severity["of"],
        serde_json::json!(["blocking", "advisory", "HIGH", "MEDIUM", "LOW"]),
        "the projection enumerates the widened vocabulary — the shipped pair KEPT, so \
         every committed value stays a member and no old→new map is owed",
    );
}
