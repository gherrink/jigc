//! M52 Increment 8 / T3 — **the two doors that re-baseline an out-of-band edit say so**
//! (`completions/artifacts/M52/settle-record.md` → D10.1 as registered at §14;
//! [gap-docs.md](../../../completions/artifacts/M52/gap-docs.md) → B8;
//! [gap-findings.md](../../../completions/artifacts/M52/gap-findings.md) → G-20;
//! `design/reconciliation.md` → The absorb surface; `design/validation.md` → The M52
//! registrations — Increment 8).
//!
//! # What was broken
//!
//! `design/reconciliation.md:186` states **"every absorb surfaces"**, and `:44-47` names the
//! surface owed — *external edit absorbed: `<doc>`*. Two doors advance a committed managed
//! doc's file-state baseline **past** an out-of-band edit and said nothing at all:
//!
//!   * **`jigc ingest`** — `engine::ingest::adopt` re-gates parse + conformance and then
//!     `record.record(path, hash)`, so a drifted-but-conformant committed doc is re-baselined
//!     at exit 0 with `"finding": null, "adopted": true`
//!     ([baseline-freeze.md](../../../completions/artifacts/M52/baseline-freeze.md) §2.2, F1
//!     and F3 — the defect is home-kind-independent).
//!   * **`jigc rename`** — the move primitive re-keys the doc's baseline at the *post-drift*
//!     bytes and step 5 re-keys every repointed referrer, so the blocking
//!     `file-state.hash-matches` row is simply gone from the next `jigc validate`
//!     ([baseline-freeze.md] §4, **L-1**, which commits it).
//!
//! **Nothing about the classification moves** (the settle's own words). The absorb arm at
//! `ingest` *is* `adopt`'s parse + conformance re-gate, and the block arm already fires on
//! non-conformant drift (baseline F2, built + proven). What was missing is the *surface*.
//!
//! # The axis this suite iterates
//!
//! The class is **a path whose committed baseline a door advances past an out-of-band edit**,
//! and the axis is the set of such paths **per door** — not the one path each defect report
//! named:
//!
//!   * `jigc ingest` — every adopted candidate ([`ingest_names_the_edit_it_absorbed`]).
//!   * `jigc rename` — the renamed doc at **both** of its arms, the reslug and the
//!     retitle-only ([`rename_names_the_edit_it_absorbed_at_the_docs_own_path`]), **the
//!     idempotent no-op** — which stages nothing, commits nothing and re-keys the baseline
//!     anyway, so it is the arm a fix keyed on the commit would miss
//!     ([`the_idempotent_rename_absorbs_and_says_so`]) — **and every repointed referrer**,
//!     whose bytes are read off disk, repointed and re-baselined at step 5
//!     ([`rename_names_the_edit_it_absorbed_at_a_repointed_referrer`]).
//!
//! The doors outside the class are the baseline's driven answer, not a guess (§2.2's door
//! table): `migrate-corpus` leaves the drift finding standing, `unmanage` drops the baseline
//! and narrates the drop, `relocate` no-ops on an unmoved home, and the task/finalize sweep
//! already emits `reconciliation.absorb` through `engine::file_state::reconcile_committed`.
//!
//! [`an_undrifted_run_names_no_absorb`] is the control, and it is half the test: an absorb
//! line on every ordinary run would be a law-1 lie at exactly the two doors an adopter runs
//! most.

use std::fs;
use std::path::Path;

use crate::support::committing_doors::{TempDir, base_repo, git, jigc, jigc_ok};

/// One committed ADR's bytes — optionally carrying a `supersedes:` edge, which is how a
/// second ADR becomes a **structured referrer** the rename repoints and re-baselines.
fn adr(title: &str, supersedes: Option<&str>) -> String {
    let edge = match supersedes {
        Some(id) => format!("supersedes: {id}\n"),
        None => String::new(),
    };
    format!(
        "---\nstatus: accepted\ndate: 2026-07-26\nschema-version: 2\n{edge}---\n\n# {title}\n\n\
         ## Context\n\nSession lookups must stay sub-millisecond.\n\n## Options\n\nA \
         distributed cache was weighed and rejected on latency.\n\n## Decision\n\nKeep \
         sessions in one node.\n\n## Consequences\n\nA cold node loses its sessions.\n"
    )
}

/// The prose the out-of-band edit rewrites, and what it rewrites it to — a **conformant**
/// edit: one slot's prose changes and the document's shape does not, which is the cell the
/// absorb arm is *for* (the non-conformant cell is baseline F2 and still blocks).
const CLEAN_PROSE: &str = "A cold node loses its sessions.";
const EDITED_PROSE: &str = "A cold node loses its sessions, and the operator edited this by hand.";

/// A repo with two committed ADRs — `use-sqlite` and `drop-redis`, the latter superseding
/// the former — **baselined** by one `jigc ingest`, so every later drift is genuine drift
/// against a recorded hash rather than the `file-state.un-baselined` cell.
fn baselined_repo(tag: &str) -> (TempDir, TempDir) {
    let (repo, home) = base_repo(tag, None);
    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("use-sqlite.md"), adr("Use SQLite", None)).expect("write use-sqlite");
    fs::write(
        dir.join("drop-redis.md"),
        adr("Drop Redis", Some("adr:use-sqlite")),
    )
    .expect("write drop-redis");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed the decisions"]);
    jigc_ok(
        repo.path(),
        home.path(),
        &["ingest"],
        "the baselining ingest",
    );
    (repo, home)
}

/// Edit `rel`'s prose out of band and commit it — the state both doors absorb.
fn drift(repo: &Path, rel: &str) {
    let abs = repo.join(rel);
    let source = fs::read_to_string(&abs).expect("read the doc to drift");
    assert!(
        source.contains(CLEAN_PROSE),
        "the fixture's prose must be there to edit: {rel}"
    );
    fs::write(&abs, source.replace(CLEAN_PROSE, EDITED_PROSE)).expect("write the drifted doc");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "an out-of-band edit"]);
}

/// Whether `jigc validate`'s store sweep still carries a `file-state.hash-matches` for `rel`
/// — the finding the absorb retires, asked before and after each door. The fixture's drift is
/// committed, so since M55 Increment 4 the row is the advisory *baseline lags `HEAD`* grade;
/// the predicate asks for the row, never its severity.
fn drift_reported(repo: &Path, home: &Path, rel: &str) -> bool {
    let out = jigc(repo, home, &["validate"], None);
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.lines()
        .any(|l| l.contains("file-state.hash-matches") && l.contains(rel))
}

/// The count of `file-state.absorbed` finding heads in an agent-text render.
fn absorb_lines(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|l| l.contains("file-state.absorbed"))
        .collect()
}

/// The `findings` array of a door's `--format json` success envelope.
fn envelope_findings(repo: &Path, home: &Path, args: &[&str]) -> Vec<serde_json::Value> {
    let mut argv = vec!["--format", "json"];
    argv.extend_from_slice(args);
    let out = jigc_ok(repo, home, &argv, "the machine arm");
    let value: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("the success envelope parses as one JSON value");
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope declares a `findings` key: {value}"))
        .clone()
}

/// Assert the **agent text** half: exactly one absorb line, naming the absorbed path and the
/// `file-state.hash-matches` it retired.
fn assert_absorbed_text(text: &str, rel: &str) {
    let lines = absorb_lines(text);
    assert_eq!(
        lines.len(),
        1,
        "exactly one `file-state.absorbed` line, naming {rel}; text was:\n{text}"
    );
    assert!(
        lines[0].contains(rel),
        "the absorb line names the absorbed path {rel}: {}",
        lines[0]
    );
    assert!(
        text.contains("file-state.hash-matches"),
        "the absorb names the finding it retired; text was:\n{text}"
    );
}

/// Assert the **machine** half: the same finding rides the success envelope's `findings` as
/// data, keyed at the same path, advisory, its route naming what it retired.
fn assert_absorbed_envelope(findings: &[serde_json::Value], rel: &str) {
    let absorbed: Vec<&serde_json::Value> = findings
        .iter()
        .filter(|f| f["key"]["code"] == "file-state.absorbed")
        .collect();
    assert_eq!(
        absorbed.len(),
        1,
        "exactly one `file-state.absorbed` on the envelope: {findings:?}"
    );
    let finding = absorbed[0];
    assert_eq!(finding["severity"], "advisory", "advisory: {finding}");
    assert_eq!(
        finding["key"]["target"], rel,
        "keyed at the absorbed path: {finding}"
    );
    assert!(
        finding["message"].as_str().unwrap_or("").contains(rel),
        "the message names the path: {finding}"
    );
    assert!(
        finding["route"]
            .as_str()
            .unwrap_or("")
            .contains("file-state.hash-matches"),
        "the route names the finding the absorb retired: {finding}"
    );
}

/// **`jigc ingest`** — the adopted candidate's baseline advances past the edit, and the run
/// says which path and which finding that retired. Exit unchanged (0), and the next
/// `jigc validate` no longer carries the drift.
#[test]
fn ingest_names_the_edit_it_absorbed() {
    let (repo, home) = baselined_repo("absorb-ingest");
    let rel = "docs/decisions/use-sqlite.md";
    drift(repo.path(), rel);
    assert!(
        drift_reported(repo.path(), home.path(), rel),
        "the fixture's drift is a `file-state.hash-matches` before the door runs"
    );

    let out = jigc(repo.path(), home.path(), &["ingest"], None);
    assert!(out.status.success(), "`jigc ingest` exits 0 — unchanged");
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert_absorbed_text(&text, rel);

    // The machine arm is driven on its own fixture: the text run above already absorbed.
    let (repo2, home2) = baselined_repo("absorb-ingest-json");
    drift(repo2.path(), rel);
    assert_absorbed_envelope(
        &envelope_findings(repo2.path(), home2.path(), &["ingest"]),
        rel,
    );
    assert!(
        !drift_reported(repo.path(), home.path(), rel),
        "the absorb retired the drift finding it named"
    );
}

/// **`jigc rename`, the doc's own path** — a retitle-only keeps the doc where it is, so
/// *"the next `jigc validate` no longer carries `file-state.hash-matches` for that path"* is
/// a claim about the same path before and after, not about a path that stopped existing.
#[test]
fn rename_names_the_edit_it_absorbed_at_the_docs_own_path() {
    let rel = "docs/decisions/use-sqlite.md";
    let retitle = &[
        "rename",
        "adr:use-sqlite",
        "--to",
        "Prefer SQLite",
        "--slug",
        "use-sqlite",
    ];

    let (repo, home) = baselined_repo("absorb-rename");
    drift(repo.path(), rel);
    assert!(
        drift_reported(repo.path(), home.path(), rel),
        "the fixture's drift is a `file-state.hash-matches` before the door runs"
    );
    let out = jigc(repo.path(), home.path(), retitle, None);
    assert!(out.status.success(), "`jigc rename` exits 0 — unchanged");
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert_absorbed_text(&text, rel);

    let (repo2, home2) = baselined_repo("absorb-rename-json");
    drift(repo2.path(), rel);
    assert_absorbed_envelope(&envelope_findings(repo2.path(), home2.path(), retitle), rel);
    assert!(
        !drift_reported(repo.path(), home.path(), rel),
        "the absorb retired the drift finding it named"
    );
}

/// **`jigc rename`, a repointed referrer** — step 5 re-baselines every doc the repoint
/// rewrote, whose bytes it read off disk drift and all. The reported defect named only the
/// renamed doc; this is the sibling path in the same transaction.
#[test]
fn rename_names_the_edit_it_absorbed_at_a_repointed_referrer() {
    let referrer = "docs/decisions/drop-redis.md";
    let (repo, home) = baselined_repo("absorb-rename-referrer");
    drift(repo.path(), referrer);
    assert!(
        drift_reported(repo.path(), home.path(), referrer),
        "the referrer's drift is reported before the door runs"
    );

    let reslug = &["rename", "adr:use-sqlite", "--to", "Use Postgres"];
    let out = jigc(repo.path(), home.path(), reslug, None);
    assert!(out.status.success(), "`jigc rename` exits 0 — unchanged");
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        text.contains("repointed adr:drop-redis#supersedes"),
        "the fixture's referrer is genuinely repointed:\n{text}"
    );

    assert_absorbed_text(&text, referrer);

    let (repo2, home2) = baselined_repo("absorb-rename-referrer-json");
    drift(repo2.path(), referrer);
    assert_absorbed_envelope(
        &envelope_findings(repo2.path(), home2.path(), reslug),
        referrer,
    );
    assert!(
        !drift_reported(repo.path(), home.path(), referrer),
        "the absorb retired the drift finding it named"
    );
}

/// **The idempotent no-op arm** — `--to` the title the doc already holds stages nothing and
/// commits nothing, and the move primitive re-keys the baseline anyway. A fix keyed on the
/// landed commit would be silent here, which is why the arm is driven rather than assumed.
#[test]
fn the_idempotent_rename_absorbs_and_says_so() {
    let rel = "docs/decisions/use-sqlite.md";
    let noop = &["rename", "adr:use-sqlite", "--to", "Use SQLite"];
    let (repo, home) = baselined_repo("absorb-rename-noop");
    drift(repo.path(), rel);

    let out = jigc(repo.path(), home.path(), noop, None);
    assert!(out.status.success(), "the no-op exits 0 — unchanged");
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        text.contains("nothing renamed, nothing committed"),
        "this is the idempotent arm:\n{text}"
    );

    assert_absorbed_text(&text, rel);

    let (repo2, home2) = baselined_repo("absorb-rename-noop-json");
    drift(repo2.path(), rel);
    assert_absorbed_envelope(&envelope_findings(repo2.path(), home2.path(), noop), rel);
    assert!(
        !drift_reported(repo.path(), home.path(), rel),
        "the no-op advanced the baseline, so the drift finding is retired — and named"
    );
}

/// **The control** — over an undrifted corpus neither door absorbs anything, so neither says
/// it did. An advisory printed on every ordinary `ingest` / `rename` would be the law-1 lie
/// this surface exists to prevent, told at the two doors an adopter runs most.
#[test]
fn an_undrifted_run_names_no_absorb() {
    let (repo, home) = baselined_repo("absorb-control");
    for door in [
        vec!["ingest"],
        vec!["rename", "adr:use-sqlite", "--to", "Prefer SQLite"],
    ] {
        let out = jigc_ok(repo.path(), home.path(), &door, "the undrifted door");
        let text = String::from_utf8_lossy(&out.stdout).into_owned();
        assert!(
            absorb_lines(&text).is_empty(),
            "no absorb line over an undrifted corpus ({door:?}):\n{text}"
        );
    }
    // …and the machine arm says the same thing as data: the key is declared and empty, never
    // absent (a driver reads `findings: []`, not a missing key it has to special-case).
    let (repo2, home2) = baselined_repo("absorb-control-json");
    assert!(
        envelope_findings(repo2.path(), home2.path(), &["ingest"]).is_empty(),
        "`ingest` declares an empty `findings` over an undrifted corpus"
    );
    assert!(
        envelope_findings(
            repo2.path(),
            home2.path(),
            &["rename", "adr:use-sqlite", "--to", "Prefer SQLite"],
        )
        .is_empty(),
        "`rename` declares an empty `findings` over an undrifted corpus"
    );
}
