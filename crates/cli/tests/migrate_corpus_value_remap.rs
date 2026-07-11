//! Acceptance — the **first methodology v1→v2 migration ever driven** through the real
//! binary (M41 Increment 5, F4 / T3; `design/corpus-migration.md` → the value-remap kind,
//! the structural-auto / value-semantic-authored distinction).
//!
//! The `deferral-ledger` `entries.kind` enum members renamed `[D, I]` → `[Decision, Idea]`
//! at schema-version 2. An enum rename is **unrecoverable from the schema pair alone** — the
//! classifier detects only *that* the members moved, never *which* old value maps onto
//! *which* new one — so `jigc migrate-corpus` threads a CLI-**authored** old→new map into the
//! emitted `ValueRemapped` before the engine fold (the `with_stamp_default` precedent). This
//! drives the shipped binary over a committed **v1-stamped** `docs/deferral-ledger.md`
//! carrying `kind: D` + `kind: I` entries and asserts the migrated **bytes on disk**:
//!
//!   - `D` → `Decision`, `I` → `Idea`, remapped **byte-faithful** — every other byte (the
//!     `date`, `trigger`, body prose, headings, item anchors) preserved. The proof is a
//!     round-trip: the v1 fixture is the exact-inverse downgrade of a **real** rendered v2
//!     canonical doc (the oracle), so a byte-faithful remap reproduces the oracle exactly.
//!   - the schema-version stamp **value-bumps** `1` → `2`.
//!   - a **re-run** reports the doc `already-current`, byte-untouched (idempotent).
//!   - the migrated doc **validates clean**.
//!
//! **The detect side — the recorded placement bound.** The done-picture's "routed migrate by
//! the store detector" premise does **not** hold for a *placement* doctype: the family-5
//! store-conformance sweep walks only `location:`-bearing schemas (`validate.rs` → Placement
//! coverage; `methodology_corpus_stamp.rs` records the same bound for the placement roadmap),
//! so a committed v1 `deferral-ledger` (`placement: docs/deferral-ledger.md`, `location:
//! None`) is **verb-migratable but not `validate`-routed** — asserted here as the true
//! behavior, not a false precondition. The verb migrates it regardless.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-value-remap-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the sibling
/// methodology-test seam threaded via `JIGC_PACK_DIR`, so the composed verbs govern the
/// `deferral-ledger` doctype (and its frozen schema-version 2 manifest entry).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Run `jigc <args>` over the methodology pack with `cwd = repo`, `$HOME = home`.
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

/// Make `repo` a real git repo, then `jigc setup` over it (installs the methodology pack).
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    std::fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    assert_ok(&jigc(repo, home, &["setup"]), "`jigc setup`");
}

/// Author one `deferral-ledger` entry (`kind` enum + `trigger` string + `body` slot; `date`
/// is CLI-set on create) in the already-created singleton, returning nothing — the canonical
/// staged bytes are read afterward.
fn author_entry(repo: &Path, home: &Path, title: &str, kind: &str, trigger: &str, body: &[u8]) {
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
    let addr = String::from_utf8(item.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-field", &format!("{addr}/kind"), "--value", kind],
            None,
        ),
        "`set-field <entry>/kind`",
    );
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-field", &format!("{addr}/trigger"), "--value", trigger],
            None,
        ),
        "`set-field <entry>/trigger`",
    );
    assert_ok(
        &jigc_doc(
            repo,
            home,
            &["set-slot", &format!("{addr}/body"), "--from-file", "-"],
            Some(body),
        ),
        "`set-slot <entry>/body`",
    );
}

/// Run `jigc migrate-corpus --format json` and parse the report.
fn migrate_json(repo: &Path, home: &Path) -> serde_json::Value {
    let out = jigc(repo, home, &["migrate-corpus", "--format", "json"]);
    assert_ok(&out, "`jigc migrate-corpus`");
    serde_json::from_slice(&out.stdout).expect("migrate-corpus --format json emits valid JSON")
}

/// The report's string array under `key` (a `blocked` entry is a `(path, route)` pair; the
/// others are bare paths).
fn paths(report: &serde_json::Value, key: &str) -> Vec<String> {
    report[key]
        .as_array()
        .unwrap_or_else(|| panic!("`{key}` is an array; got: {report}"))
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_string)
                .unwrap_or_else(|| v[0].as_str().expect("path").to_string())
        })
        .collect()
}

/// The first methodology v1→v2 migration through the real binary: a committed **v1-stamped**
/// `docs/deferral-ledger.md` (`kind: D` + `kind: I`) is remapped `D`→`Decision` / `I`→`Idea`
/// **byte-faithful** via the CLI-authored map, the stamp value-bumped `1`→`2`, a re-run is
/// idempotent, and the migrated doc validates clean — with the placement doctype's
/// not-`validate`-routed bound asserted as the true detect-side behavior.
#[test]
fn migrate_corpus_remaps_the_deferral_ledger_kind_byte_faithful() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // --- author a REAL v2 canonical deferral-ledger (the byte-faithfulness oracle) ---
    // The staged canonical bytes are the exact form the renderer produces; the committed v1
    // fixture below is their exact-inverse downgrade, so a byte-faithful remap must reproduce
    // these bytes exactly. Authored off-router through the `planning` workflow's create-gate.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "planning", "M-Test"],
        ),
        "`jigc start --workflow planning`",
    );
    assert_ok(
        &jigc_doc(
            repo.path(),
            home.path(),
            &["create", "deferral-ledger", "--title", "Deferral-Ledger"],
            None,
        ),
        "`doc create deferral-ledger`",
    );
    author_entry(
        repo.path(),
        home.path(),
        "Cache the index",
        "Decision",
        "M-Store",
        b"Deferred until the store scope lands.\n",
    );
    author_entry(
        repo.path(),
        home.path(),
        "A plugin surface",
        "Idea",
        "M-External",
        b"Parked until a real external domain earns it.\n",
    );
    let canonical_v2 = std::fs::read_to_string(
        repo.path()
            .join(".jigc/tasks/m-test/docs/deferral-ledger:deferral-ledger.md"),
    )
    .expect("read the staged canonical v2 deferral-ledger");
    // Sanity: the oracle really is v2-shaped (spelled-out members, stamp 2) and its body
    // prose carries no `Decision`/`Idea` token the downgrade could clobber outside the field.
    assert!(
        canonical_v2.contains("schema-version: 2")
            && canonical_v2.contains("- kind: Decision")
            && canonical_v2.contains("- kind: Idea"),
        "the oracle is the v2 canonical form; got:\n{canonical_v2}"
    );

    // --- commit the v1 fixture: the exact-inverse downgrade of the oracle ---
    // Only the two `kind:` field lines and the stamp value change (the v1→v2 delta is exactly
    // the enum rename + the stamp bump); every other byte is identical, so the migration's
    // remap is the exact inverse.
    let committed_v1 = canonical_v2
        .replace("schema-version: 2", "schema-version: 1")
        .replace("- kind: Decision", "- kind: D")
        .replace("- kind: Idea", "- kind: I");
    assert_ne!(committed_v1, canonical_v2, "the downgrade actually differs");
    std::fs::create_dir_all(repo.path().join("docs")).expect("mk docs/");
    std::fs::write(repo.path().join("docs/deferral-ledger.md"), &committed_v1)
        .expect("write the v1 fixture");
    git(repo.path(), &["add", "docs/deferral-ledger.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "seed v1 deferral-ledger"],
    );

    // --- the detect side: the recorded PLACEMENT bound (verb-migratable, not validate-routed) ---
    // Family 5 walks only `location:`-bearing schemas, so a below-version placement doc
    // surfaces NO store finding — the true behavior, asserted rather than a false "routed
    // migrate" precondition (the `methodology_corpus_stamp.rs` placement-roadmap precedent).
    let validate_before = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    assert_ok(&validate_before, "`jigc validate` (before)");
    let before: serde_json::Value =
        serde_json::from_slice(&validate_before.stdout).expect("validate emits valid JSON");
    let dl_findings: Vec<&serde_json::Value> = before["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .filter(|f| {
            serde_json::to_string(f)
                .unwrap()
                .contains("deferral-ledger")
        })
        .collect();
    assert!(
        dl_findings.is_empty(),
        "a below-version placement deferral-ledger is not validate-routed (the recorded \
         placement bound); got: {dl_findings:?}"
    );

    // --- migrate-corpus: the authored remap drives the first methodology v1→v2 migration ---
    let report = migrate_json(repo.path(), home.path());
    assert_eq!(
        paths(&report, "migrated"),
        vec!["docs/deferral-ledger.md".to_string()],
        "the v1 deferral-ledger migrates (was stranded pre-map); got: {report}",
    );
    assert!(
        paths(&report, "blocked").is_empty() && paths(&report, "already_current").is_empty(),
        "nothing blocked or skipped on the migrating run; got: {report}",
    );

    // BYTE-FAITHFUL: the migrated bytes on disk equal the v2 canonical oracle exactly —
    // `D`→`Decision`, `I`→`Idea`, stamp `1`→`2`, every other byte (date/trigger/body/anchors)
    // preserved. Drives the emitted artifact (the file), never a reconstruction.
    let migrated = std::fs::read_to_string(repo.path().join("docs/deferral-ledger.md"))
        .expect("read migrated");
    assert_eq!(
        migrated, canonical_v2,
        "the migrated doc is byte-faithful to the v2 canonical oracle",
    );

    // --- idempotent re-run: already-current, byte-untouched ---
    let rerun = migrate_json(repo.path(), home.path());
    assert_eq!(
        paths(&rerun, "already_current"),
        vec!["docs/deferral-ledger.md".to_string()],
        "the migrated doc is already-current on a re-run; got: {rerun}",
    );
    assert!(
        paths(&rerun, "migrated").is_empty(),
        "nothing migrates twice; got: {rerun}",
    );
    let after_rerun =
        std::fs::read_to_string(repo.path().join("docs/deferral-ledger.md")).expect("read");
    assert_eq!(after_rerun, canonical_v2, "the re-run is byte-inert");

    // --- the migrated doc validates clean (the field-value break is gone: kind now conforms) ---
    let validate_after = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    assert_ok(&validate_after, "`jigc validate` (after)");
    let after: serde_json::Value =
        serde_json::from_slice(&validate_after.stdout).expect("validate emits valid JSON");
    let dl_after: Vec<&serde_json::Value> = after["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .filter(|f| {
            serde_json::to_string(f)
                .unwrap()
                .contains("deferral-ledger")
        })
        .collect();
    assert!(
        dl_after.is_empty(),
        "the migrated deferral-ledger validates clean; got: {dl_after:?}"
    );
}
