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
//! **The detect side — the placement bound is GONE (M42 Inc-2 T1).** Through rc.5 the
//! family-5 store-conformance sweep walked only `location:`-bearing schemas, so a committed
//! v1 `deferral-ledger` (`placement: docs/deferral-ledger.md`, `location: None`) was
//! **verb-migratable but not `validate`-routed** — a bound this test asserted (`dl_findings`
//! empty) rather than a behavior anyone wanted. Family 5 now enumerates through
//! `index::committed_instances` (`design/validation.md` → "every committed instance"), so the
//! stale placement doc **is** detected and routed `migrate` at the doc the verb then fixes.
//! The assertion flips accordingly, and is **presence**-shaped, never set-equality: other
//! store families legitimately raise their own findings against the same doc. The
//! **file↔CLI-state twin** joins it (M42 Inc-2 T2, the same census hole one site over): the
//! read-only sweep now sees the placement doc too, so this never-baselined ledger surfaces
//! its `file-state.un-baselined` advisory at its literal home — pinned here as the second
//! detect-side assertion.

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
            engine::tempname::unique_nanos(),
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`) — the sibling
/// methodology-test seam threaded via `JIGC_PACK_DIR`, so the composed verbs govern the
/// `deferral-ledger` doctype (and its frozen schema-version 2 manifest entry).
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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
        crate::support::child_stdin::feed(&mut child, bytes);
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

/// The paths the report lists under `key`. `migrated` / `already_current` are bare path strings;
/// a `blocked` entry is a **`Finding`** (M42 completion audit, Finding 2 — it was an untyped
/// `[path, route]` tuple), whose path is its stable `key.target`.
fn paths(report: &serde_json::Value, key: &str) -> Vec<String> {
    report[key]
        .as_array()
        .unwrap_or_else(|| panic!("`{key}` is an array; got: {report}"))
        .iter()
        .map(|v| {
            v.as_str().map(str::to_string).unwrap_or_else(|| {
                v["key"]["target"]
                    .as_str()
                    .unwrap_or_else(|| panic!("a blocked finding carries a path target; got: {v}"))
                    .to_string()
            })
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
            &["create", "deferral-ledger", "--title", "Deferral Ledger"],
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

    // --- the detect side: the stale placement doc IS routed at the verb (M42 Inc-2 T1) ---
    // Family 5 enumerates through `index::committed_instances`, so a below-version PLACEMENT
    // doc raises its blocking version-currency break — `schema-conformance.schema-version-current`
    // (M42 Inc-3 T2: its own check id, so the staleness fact is machine-distinguishable from an
    // ordinary value break) — routed at `jigc migrate-corpus`, the verb that fixes it below.
    // Presence, not set-equality: the store sweep's other families may legitimately raise their
    // own findings against the same doc.
    // The exit is **non-zero** here (M42 Inc-4 T2): the corpus is unmigrated, the third
    // exit-flipping exception. The JSON report is still emitted — that is what this reads.
    let validate_before = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    assert!(
        !validate_before.status.success(),
        "`jigc validate` over the unmigrated ledger corpus exits non-zero; stdout:\n{}",
        String::from_utf8_lossy(&validate_before.stdout),
    );
    let before: serde_json::Value =
        serde_json::from_slice(&validate_before.stdout).expect("validate emits valid JSON");
    let routed: Vec<&serde_json::Value> = before["findings"]
        .as_array()
        .expect("findings array")
        .iter()
        .filter(|f| {
            f["severity"] == "blocking"
                && f["code"].as_str() == Some("schema-conformance.schema-version-current")
                && f["key"]["target"]
                    .as_str()
                    .is_some_and(|t| t.starts_with("deferral-ledger:deferral-ledger"))
                && f["route"].as_str().is_some_and(|r| {
                    r.starts_with("migrate")
                        && r.contains("deferral-ledger.md")
                        && r.contains("jigc migrate-corpus")
                })
        })
        .collect();
    assert!(
        !routed.is_empty(),
        "the below-version placement deferral-ledger is DETECTED, surfaces the version-currency \
         break, and is routed at `jigc migrate-corpus`; got: {}",
        before["findings"],
    );
    // The same sweep's file↔CLI-state twin now SEES the placement doc too (M42 Inc-2 T2):
    // this committed-but-never-finalized ledger carries no baseline, so the read-only twin
    // classifies it `un-baselined` — an advisory, at its literal placement home. Pre-fix the
    // twin skipped every `location: None` schema, so the whole placement class was invisible
    // to `jigc validate` (drift included — the invariant break this pins the fix of).
    assert!(
        before["findings"]
            .as_array()
            .expect("findings array")
            .iter()
            .any(|f| {
                f["code"] == "file-state.un-baselined"
                    && f["message"]
                        .as_str()
                        .is_some_and(|m| m.contains("docs/deferral-ledger.md"))
            }),
        "the committed placement deferral-ledger is visible to the file-state twin \
         (un-baselined, at its literal home); got: {}",
        before["findings"],
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
    // The content break is gone. What remains is the twin's `file-state.un-baselined`
    // advisory — this fixture's ledger was committed by hand (git), never through a
    // `finalize`, so it carries no baseline, and the read-only twin's designed
    // not-yet-tracked outcome is informational ("no action needed"), never a break. It is
    // visible *because* of the same placement fix (M42 Inc-2 T2) and is pinned positively
    // above; asserted **exactly** here — every remaining ledger finding must be that
    // advisory — so no real conformance break can hide behind a narrowed filter.
    assert!(
        dl_after
            .iter()
            .all(|f| f["code"] == "file-state.un-baselined" && f["severity"] == "advisory"),
        "the migrated deferral-ledger carries no content break — at most the un-baselined \
         advisory of a hand-committed (never finalized) doc; got: {dl_after:?}"
    );
}
