//! T2 — the pack-config edge: the methodology `increment` workflow stays **off
//! the router** (M15 Inc 2; `design/self-hosting.md` → What M15 built;
//! `implementation/increment-workflow.md` → Validation hardening #5;
//! `design/worked-examples.md` → flow 18 setup — ADD, don't touch).
//!
//! Increment 1 shipped the `increment` workflow (`creates-task: false`, six
//! task-ref-free steps) and proved it composes via `--workflow increment`. That is
//! the **composing** context. This test is the **omitting** context (hardening #5):
//! the falsifiable scope-honesty proof that `increment` never leaks into the
//! model-free selection catalog a bare `jigc start` orients over — the M8
//! catalog-leak class. `documented != delivered`, so it earns its own gated task.
//!
//! M16 Inc 4 T3 EXTENDS this guard to the `planning` workflow (the same M8
//! catalog-leak class; `design/methodology-docs.md` → The authoring spine
//! (`creates-task: true, selectable: false`) + Off-router note). Planning is
//! `creates-task: true, selectable: false` — it MINTS a task (so authoring rides
//! the create-gate + finalize-promote spine) but stays off the router catalog and
//! out of the `default-workflow.of` enum. The base `dev-task`-only assertions
//! already exclude `planning` *by construction*, but `documented != delivered`:
//! T3 adds the EXPLICIT `planning`-named negative assertions so the guard is
//! falsifiable for THIS workflow — a leak (planning added to the enum, or
//! `selectable: true` flipping it into the catalog) goes red here, not silently.
//!
//! M16 Inc 5 T3 EXTENDS the same guard to the `completion` workflow — the second
//! authoring-spine workflow (`design/methodology-docs.md` → The authoring spine).
//! Like `planning`, `completion` is `creates-task: true, selectable: false`: it
//! MINTS a task (riding the create-gate + finalize-promote spine) but stays off the
//! router catalog and out of the `default-workflow.of` enum. The `dev-task`-only
//! base already excludes it *by construction*, but `documented != delivered`, so T3
//! adds the EXPLICIT `completion`-named negatives — a leak (`completion` in the enum,
//! or `selectable: true` flipping it into the catalog) goes red here, not silently.
//!
//! M17 Inc 5 T3 EXTENDS the same guard to the `record-dogfood` workflow — the
//! measured-dogfood recording spine (`design/measurement.md` → The dogfood-record
//! doctype: "`creates-task: true, selectable: false` — the completion-workflow
//! authoring-spine pattern"). Like planning/completion it mints a task (riding the
//! create-gate + finalize-promote spine) but is invoked only off-router via
//! `jigc start --workflow record-dogfood "<run>"`. The `dev-task`-only base already
//! excludes it *by construction*, but `documented != delivered`, so T3 adds the
//! EXPLICIT `record-dogfood`-named negatives — a leak (`record-dogfood` in the
//! enum, or `selectable: true` flipping it into the catalog) goes red here, not
//! silently.
//!
//! M43 Inc 4 T1 (fork 2) FLIPS `decided-task` to `selectable: true` — the 2026-06-13
//! hide's expiry (the router-flip) fired long ago and the authoring-spine
//! categorization was the original miscall, so the decided-task negatives below
//! reddened deliberately and are rewritten POSITIVE: the catalog must list it with
//! its decision-axis `when` hint. The knobs guard keeps its decided-task negative —
//! the `default-workflow.of` enum is a valid-values list, not a selectable mirror.
//!
//! Two leak vectors, two guards:
//!   (a) The **front-door catalog** (the live path). Bare `jigc start` (no intent)
//!       over `JIGC_PACK_DIR=<methodology>` orients to the clean-no-task state and
//!       lists the selectable (`creates-task: true`) work-workflows only — `dev-task`
//!       alone, never `increment`. Asserted on the real binary's emitted bytes
//!       (human catalog line + the JSON `workflows` array), not a reconstruction.
//!   (b) The **config enum** (the forbidden static leak). The methodology
//!       `config/knobs.yaml` `default-workflow.of` enum must not name `increment`
//!       — adding it there is exactly the M8 catalog-leak this task forbids.
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
            "jigc-increment-off-router-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (`setup`/orientation read HEAD).
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

#[test]
fn bare_start_over_methodology_lists_the_selectables_never_increment() {
    // (a) The live front-door catalog. Set up the methodology project, then bare
    // `jigc start` (no intent) — the read-only orientation over the clean-no-task
    // state. The catalog it composes is filtered to the selectable (`creates-task:
    // true`) work-workflows, so `increment` (`creates-task: false`) is excluded by
    // construction. The omitting context (hardening #5): `increment` is inert here,
    // never listed, never an error.
    let repo = TempDir::new("orient");
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

    // Human emitted bytes: the catalog lists the `dev-task` selection line and no
    // `increment` line.
    let human = run_jigc(repo.path(), home.path(), &pack, &["start"]);
    assert!(
        human.status.success(),
        "bare `jigc start` must orient and exit 0; got {:?}\nstderr:\n{}",
        human.status,
        String::from_utf8_lossy(&human.stderr),
    );
    let human_out = String::from_utf8(human.stdout).expect("utf-8 stdout");
    assert!(
        human_out.contains(
            "dev-task — implement one scoped change test-first, recording no decision \
             and touching no documented code"
        ),
        "the orientation catalog must list the `dev-task` selection line (the M43 \
         decision-axis `when` hint); got:\n{human_out}",
    );
    // The selection-catalog lines are the `  - <id> — <when>` entries; assert no
    // catalog line names `increment` (scope to the catalog so the temp-repo path in
    // the provenance header — which carries this test's tag — can't false-positive).
    let catalog_lines: Vec<&str> = human_out
        .lines()
        .filter(|l| l.trim_start().starts_with("- "))
        .collect();
    assert!(
        !catalog_lines.iter().any(|l| l.contains("increment")),
        "the model-free selection catalog must NOT name `increment` (the M8 catalog-leak \
         class — `increment` is `creates-task: false`, not router-selectable); \
         catalog lines: {catalog_lines:?}",
    );
    // T3 (planning): the same M8 catalog-leak guard, named for `planning`. Planning
    // is `creates-task: true, selectable: false` — it mints a task but stays out of
    // the selectable catalog (the `creates_task && selectable` filter). A leak
    // (`selectable: true`) would surface it here; assert no catalog line names it.
    assert!(
        !catalog_lines.iter().any(|l| l.contains("planning")),
        "the model-free selection catalog must NOT name `planning` (the M8 catalog-leak \
         class — `planning` is `selectable: false`, off-router by construction); \
         catalog lines: {catalog_lines:?}",
    );
    // T3 (completion): the same M8 catalog-leak guard, named for `completion`. Like
    // planning it is `creates-task: true, selectable: false` — it mints a task but
    // stays out of the selectable catalog (the `creates_task && selectable` filter).
    // A leak (`selectable: true`) would surface it here; assert no catalog line names it.
    assert!(
        !catalog_lines.iter().any(|l| l.contains("completion")),
        "the model-free selection catalog must NOT name `completion` (the M8 catalog-leak \
         class — `completion` is `selectable: false`, off-router by construction); \
         catalog lines: {catalog_lines:?}",
    );
    // M17 T3 (record-dogfood): the same M8 catalog-leak guard, named for the
    // recording workflow. `record-dogfood` is `creates-task: true, selectable:
    // false` — it mints a task but stays out of the selectable catalog (the
    // `creates_task && selectable` filter); it is invoked only off-router via
    // `--workflow record-dogfood`. A leak (`selectable: true`) would surface it
    // here; assert no catalog line names it.
    assert!(
        !catalog_lines.iter().any(|l| l.contains("record-dogfood")),
        "the model-free selection catalog must NOT name `record-dogfood` (the M8 \
         catalog-leak class — `record-dogfood` is `selectable: false`, off-router by \
         construction); catalog lines: {catalog_lines:?}",
    );
    // M43 fork 2 (the un-hide): `decided-task` flips `selectable: true` — the
    // 2026-06-13 hide's expiry (the router-flip) fired long ago, and the
    // authoring-spine categorization was the original miscall (it is a
    // work-workflow with a decision step). The old negative guard reddened
    // deliberately and is rewritten POSITIVE: the catalog must list decided-task
    // with its decision-axis `when` hint (records-a-decision is the axis that
    // discriminates it from `dev-task`).
    assert!(
        catalog_lines.iter().any(|l| l.contains(
            "decided-task — implement one scoped change test-first, recording its design \
             decision on the running decisions log"
        )),
        "the model-free selection catalog must list `decided-task` with its decision-axis \
         `when` hint (M43 fork 2 — the hide's expiry fired, selectable: true); \
         catalog lines: {catalog_lines:?}",
    );

    // JSON emitted bytes: the `workflows` array carries exactly one entry, `dev-task`
    // — `increment` is absent and nothing else leaks.
    let json = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--format", "json"],
    );
    assert!(
        json.status.success(),
        "bare `jigc start --format json` must exit 0; got {:?}\nstderr:\n{}",
        json.status,
        String::from_utf8_lossy(&json.stderr),
    );
    let json_out = String::from_utf8(json.stdout).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&json_out)
        .unwrap_or_else(|e| panic!("orientation must be valid JSON ({e}); got:\n{json_out}"));
    let workflows = value["workflows"]
        .as_array()
        .unwrap_or_else(|| panic!("`workflows` must be an array; got:\n{json_out}"));
    let ids: Vec<&str> = workflows
        .iter()
        .map(|w| w["id"].as_str().unwrap_or_default())
        .collect();
    assert_eq!(
        ids,
        vec![
            "decided-task",
            "dev-task",
            "do-research",
            "form-vision",
            "park-idea"
        ],
        "the selectable catalog must be exactly [decided-task, dev-task, do-research, \
         form-vision, park-idea] — the five selectable work-workflows (`do-research`, \
         `form-vision`, and `park-idea` are `selectable: true` by design, M37 §3; \
         `decided-task` joins at M43 fork 2 — the hide's expiry fired); `increment` \
         must not leak in (M8 catalog-leak class); got:\n{json_out}",
    );
    // T3 (planning): the explicit planning-named negative over the same JSON bytes —
    // `ids == [dev-task]` already excludes it by construction, but the named guard
    // makes the planning leak falsifiable (documented != delivered).
    assert!(
        !ids.contains(&"planning"),
        "the selectable catalog must NOT name `planning` — it is `selectable: false`, \
         off-router (M8 catalog-leak class); got:\n{json_out}",
    );
    // T3 (completion): the explicit completion-named negative over the same JSON bytes
    // — `ids == [dev-task]` already excludes it by construction, but the named guard
    // makes the completion leak falsifiable (documented != delivered).
    assert!(
        !ids.contains(&"completion"),
        "the selectable catalog must NOT name `completion` — it is `selectable: false`, \
         off-router (M8 catalog-leak class); got:\n{json_out}",
    );
    // M17 T3 (record-dogfood): the explicit record-dogfood-named negative over the
    // same JSON bytes — `ids == [dev-task]` already excludes it by construction, but
    // the named guard makes the recording leak falsifiable (documented != delivered).
    assert!(
        !ids.contains(&"record-dogfood"),
        "the selectable catalog must NOT name `record-dogfood` — it is `selectable: false`, \
         off-router (M8 catalog-leak class); got:\n{json_out}",
    );
    // M43 fork 2: the explicit decided-task-named POSITIVE over the same JSON bytes
    // — the exact-set assert above already includes it, but the named guard makes a
    // re-hide regression falsifiable on its own (documented != delivered).
    assert!(
        ids.contains(&"decided-task"),
        "the selectable catalog must name `decided-task` — `selectable: true` since M43 \
         fork 2 (the 2026-06-13 hide's expiry fired); got:\n{json_out}",
    );
}

#[test]
fn knobs_default_workflow_enum_does_not_name_increment() {
    // (b) The forbidden static leak. Adding `increment` to the `default-workflow.of`
    // enum is precisely the M8 catalog-leak this task forbids — it would make the
    // `creates-task: false` increment a router-selectable default. Read the committed
    // methodology `knobs.yaml` and assert the enum stays free of `increment`.
    let knobs_path = methodology_pack_tree().join("config").join("knobs.yaml");
    let text = fs::read_to_string(&knobs_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", knobs_path.display()));
    let doc: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(&text).expect("methodology knobs.yaml must be valid YAML");
    let of = doc["default-workflow"]["of"]
        .as_sequence()
        .expect("`default-workflow.of` must be a sequence");
    let names: Vec<&str> = of.iter().filter_map(|v| v.as_str()).collect();
    assert!(
        !names.contains(&"increment"),
        "`default-workflow.of` must NOT name `increment` (the forbidden M8 catalog-leak — \
         `increment` is `creates-task: false`); got enum: {names:?}",
    );
    // T3 (planning): the same static-leak guard, named for `planning`. Adding
    // `planning` to the enum would make the `selectable: false` authoring workflow a
    // router-selectable default — exactly the M8 catalog-leak this task forbids.
    assert!(
        !names.contains(&"planning"),
        "`default-workflow.of` must NOT name `planning` (the forbidden M8 catalog-leak — \
         `planning` is `selectable: false`, off-router); got enum: {names:?}",
    );
    // T3 (completion): the same static-leak guard, named for `completion`. Adding it to
    // the enum would make the `selectable: false` authoring workflow a router-selectable
    // default — exactly the M8 catalog-leak this task forbids.
    assert!(
        !names.contains(&"completion"),
        "`default-workflow.of` must NOT name `completion` (the forbidden M8 catalog-leak — \
         `completion` is `selectable: false`, off-router); got enum: {names:?}",
    );
    // M17 T3 (record-dogfood): the same static-leak guard, named for the recording
    // workflow. Adding it to the enum would make the `selectable: false` recording
    // spine a router-selectable default — exactly the M8 catalog-leak this task forbids.
    assert!(
        !names.contains(&"record-dogfood"),
        "`default-workflow.of` must NOT name `record-dogfood` (the forbidden M8 catalog-leak \
         — `record-dogfood` is `selectable: false`, off-router); got enum: {names:?}",
    );
    // M43 fork 2 revised this guard's decided-task rationale, not its assertion: the
    // enum is a VALID-VALUES list for the `default-workflow` knob, not a mirror of the
    // selectable catalog (dev's enum likewise omits selectable workflows). `decided-task`
    // is `selectable: true` since M43, but the pack's default stays `dev-task` — the
    // enum deliberately does not grow with the catalog.
    assert!(
        !names.contains(&"decided-task"),
        "`default-workflow.of` must NOT name `decided-task` — the enum is the knob's \
         valid-values list, not a selectable mirror, and the pack default stays `dev-task` \
         (M43 fork 2); got enum: {names:?}",
    );
    assert_eq!(
        names,
        vec!["dev-task"],
        "`default-workflow.of` must stay the single-entry [dev-task] enum (byte-unchanged \
         from base, forbidding `increment`, `planning`, `completion`, and `record-dogfood`; \
         `decided-task` is selectable since M43 but the enum is a valid-values list, not a \
         catalog mirror); got: {names:?}",
    );
}
