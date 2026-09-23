//! Acceptance — **one foreign file, one code, one route at both doors** (M48 Increment 4,
//! T1) and **the managed cell routes on its stamp** (T2); `design/validation.md` → The
//! managed-vs-foreign discriminator.
//!
//! M42 shipped the managed-vs-foreign discriminator and swept it through the **store**
//! family: a committed file squatting at a managed home that jigc never wrote draws the
//! adoption advisory `schema-conformance.unadopted-instance`, routed at `jigc ingest` /
//! `jigc migrate <path> --as <doctype>`. That sweep **never reached the task-scope
//! reconciler**, which adjudicates the very same committed file through
//! `engine::file_state::reconcile_committed`'s `UNKNOWN` + non-conformant arm — and graded
//! **every** such file `reconciliation.conformance-block`, foreign and managed alike.
//!
//! So one file answered **two codes** depending on which door asked, and the task-scope
//! door's advisory carried a route (*"ingest, migrate, or move it out of the managed
//! location"*) that is prose about a class, not about this file — while the store door
//! already named the two concrete verbs with the path substituted in.
//!
//! This suite drives the **real binary** over a repo carrying **both** cells at once, so the
//! split is proven per *file*, not per *report*:
//!
//! - **the foreign cell** — a freeform `docs/decisions/notes.md`, committed in git outside
//!   jigc — draws the **same `(code, target)` pair** and a **byte-identical route** at all
//!   three doors (`jigc validate` store scope, `jigc task validate`, the `task finalize`
//!   preflight), draws **zero** `reconciliation.conformance-block`, is **not** baseline-
//!   adopted into `.jigc/state/file-state.json`, **re-fires** on a second sweep (the
//!   routed-but-not-recorded recurrence), and leaves the task doors at **exit 0**;
//! - **the managed cell** — a v2-stamped ADR with `## Consequences` removed — keeps its
//!   `reconciliation.conformance-block` and draws **no** `unadopted-instance`: the
//!   discriminator suppresses nothing it cannot prove foreign.
//!
//! The route is **lifted from the store report and compared verbatim** — the emitted bytes
//! are the contract, never a route rebuilt in test code.
//!
//! The **stamp axis** (T2) is the second test below: T1 split the *code* and left the
//! *route*, which was written when this arm served foreign ∪ managed. With the foreign half
//! gone, *"ingest, migrate, or move it out of the managed location"* is adoption-or-removal
//! advice about a doc jigc wrote and owns — wrong for 100% of what is left. The route now
//! reads the doc's **schema-version stamp**, and the four stamp states are iterated end to
//! end: at-version ⇒ the blocking twin's hand-repair sanction (lifted from a `DRIFTED`
//! finding in the same run), below-version and stamp-absent ⇒ the corpus-migration route,
//! above-current ⇒ the `ahead` route that names no verb which would block it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-foreign-both-doors-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — the
/// real subprocess the `jigc validate` pre-flight resolves.
fn doc_code_probe() -> &'static Path {
    static PROBE: OnceLock<PathBuf> = OnceLock::new();
    PROBE.get_or_init(|| {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("probes")
            .join("doc-code")
            .join("Cargo.toml");
        let out = Command::new(env!("CARGO"))
            .args(["build", "--quiet", "--manifest-path"])
            .arg(&manifest)
            .output()
            .expect("invoke cargo build for doc-code");
        assert!(
            out.status.success(),
            "building the doc-code probe failed:\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
        let bin = manifest
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("doc-code");
        assert!(bin.is_file(), "doc-code binary missing at {bin:?}");
        bin
    })
}

/// Run a `git` command in `cwd`, asserting success.
fn git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, piping `stdin`.
fn jigc_doc_stdin(repo: &Path, home: &Path, args: &[&str], stdin: &[u8]) -> std::process::Output {
    use std::io::Write;
    use std::process::Stdio;
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .arg("doc")
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn jigc");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(stdin)
        .expect("write stdin");
    child.wait_with_output().expect("wait for jigc")
}

fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Make `root` a real git repo, then run `jigc setup` over it.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    assert_ok(&jigc(repo, home, &["setup"]), "`jigc setup`");
}

/// The **foreign cell**: freeform scratch prose committed into the `adr` location dir
/// outside jigc. It carries no schema-version stamp and parses against no shipped `adr`
/// schema version — never adopted, by its own committed bytes.
const NOTES: &str = "docs/decisions/notes.md";
const NOTES_BODY: &str = "# scratch notes\n\nrandom thoughts, not an ADR\n";

/// The **managed cell**: a v2-**stamped** ADR (so the discriminator's first arm reads it
/// managed outright) whose `## Consequences` is missing, so it fails to parse under the
/// current shape. Its break is a real break on jigc's own doc — never an adoption case.
const MANAGED_ADR: &str = "docs/decisions/cache-sessions-in-memory.md";
const MANAGED_ADR_BODY: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Cache sessions in memory

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.
";

// The **stamp axis**, cell by cell — one un-baselined, non-conformant **managed** ADR per
// stamp state, so the advisory's route is proven over the whole axis and not over one repro
// (M45's complete-fix contract; M48 Inc 4 / T2).
//
// Each body is genuinely non-conformant under the current (v2) `adr` shape, so every cell
// lands in `reconcile_committed`'s `UNKNOWN` + non-conformant arm — the *only* thing that
// varies across the four is the schema-version stamp the route must read.

/// **At-version** — v2-stamped, `## Consequences` removed. The doc is at the schema this
/// binary knows: nothing to migrate, so the route is the blocking twin's hand-repair sanction.
/// (`MANAGED_ADR` above is this cell; the axis reuses it rather than minting a second copy.)
const AT_VERSION: &str = MANAGED_ADR;

/// **Below-version** — the shipped **prior (v1)** shape, stamped `schema-version: 1` while the
/// `adr` manifest is at 2: the commonest stale doc in a real corpus. It does not parse under
/// v2 (the optional `## Options` slot makes `## Decision` read as a renamed section), and
/// hand-repairing it would be repairing what `jigc migrate-corpus` must rewrite.
const BELOW_VERSION: &str = "docs/decisions/queue-writes-behind-a-buffer.md";
const BELOW_VERSION_BODY: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# Queue writes behind a buffer

## Context

Write bursts overwhelm the primary.

## Decision

Buffer writes and drain them on a timer.

## Consequences

A crash loses the un-drained tail.
";

/// **Stamp-absent** — the v0-era corpus state: the prior (v1) shape with **no** stamp at all.
/// It parses against the shipped `adr.v1` snapshot, so the discriminator adjudicates it
/// **managed** (never foreign), and its route is the stamp-absent corpus-migration one.
const STAMP_ABSENT: &str = "docs/decisions/retry-with-backoff.md";
const STAMP_ABSENT_BODY: &str = "\
---
status: accepted
date: 2026-06-25
---

# Retry with backoff

## Context

A flapping upstream turns one failure into a thundering herd.

## Decision

Retry with exponential backoff and jitter.

## Consequences

A slow upstream is held open longer.
";

/// **Above-current** — stamped `schema-version: 3` while the manifest is at 2, with
/// `## Consequences` removed. No verb in this binary can fix a future stamp, so the route must
/// name none that would block it.
const AHEAD: &str = "docs/decisions/shard-the-index.md";
const AHEAD_BODY: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 3
---

# Shard the index

## Context

One index node cannot hold the corpus.

## Options

A bigger node was weighed and rejected on cost.

## Decision

Shard the index by tenant.
";

/// The **baselined** ADR the axis lifts its sanction from: conformant v2 when jigc baselines
/// it, then broken out of band so the reconciler's `DRIFTED + UNTOUCHED` arm mints the
/// **blocking** twin whose route the at-version cell must match byte for byte.
const BASELINED: &str = "docs/decisions/rate-limit-the-api.md";
const BASELINED_CONFORMANT: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Rate limit the API

## Context

A single client can saturate the edge.

## Options

Per-IP throttling was weighed and rejected as too coarse.

## Decision

Rate limit per API key.

## Consequences

A burst-heavy client sees 429s.
";
const BASELINED_DRIFTED: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Rate limit the API

## Context

A single client can saturate the edge.

## Options

Per-IP throttling was weighed and rejected as too coarse.

## Decision

Rate limit per API key.
";

/// The findings array of a `--format json` envelope.
fn envelope_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    let stdout = String::from_utf8(out.stdout.clone()).expect("utf-8 stdout");
    let value: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("{what} envelope must parse ({e}); got:\n{stdout}"));
    value["findings"]
        .as_array()
        .unwrap_or_else(|| {
            panic!("{what}: the envelope carries a `findings` array; got:\n{stdout}")
        })
        .clone()
}

/// Every finding whose stable key is exactly `(code, target)` — the discriminating key the
/// contract pins (`design/command-output-contract.md`), read from the emitted `key` object
/// rather than re-derived from the message.
fn by_key<'a>(
    findings: &'a [serde_json::Value],
    code: &str,
    target: &str,
) -> Vec<&'a serde_json::Value> {
    findings
        .iter()
        .filter(|f| {
            f["key"]["code"].as_str() == Some(code) && f["key"]["target"].as_str() == Some(target)
        })
        .collect()
}

/// The one finding at `(code, target)`, or a panic naming the whole report.
fn one_at<'a>(
    findings: &'a [serde_json::Value],
    code: &str,
    target: &str,
    door: &str,
) -> &'a serde_json::Value {
    let hits = by_key(findings, code, target);
    assert_eq!(
        hits.len(),
        1,
        "{door}: exactly one `{code}` at `{target}`; got:\n{findings:#?}",
    );
    hits[0]
}

/// The route string a finding carries, verbatim.
fn route_of(finding: &serde_json::Value, door: &str) -> String {
    finding["route"]
        .as_str()
        .unwrap_or_else(|| {
            panic!("{door}: the adoption advisory carries a route; got:\n{finding:#?}")
        })
        .to_string()
}

/// Start a commit-only `single-task` for `intent` (its task id is the slugified intent),
/// stage one code file and fill the commit doc, leaving the caller to drive the doors.
fn stage_commit_only(repo: &Path, home: &Path, task: &str, intent: &str) {
    assert_ok(
        &jigc(repo, home, &["start", "--workflow", "single-task", intent]),
        &format!("`jigc start` ({task})"),
    );
    fs::write(repo.join(format!("{task}.txt")), "the code change\n").expect("write code change");
    git(repo, &["add", &format!("{task}.txt")]);
    let set_field = |addr: &str, value: &str| {
        assert_ok(
            &jigc_doc_stdin(repo, home, &["set-field", addr, "--value", value], b""),
            &format!("set-field {addr}"),
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        assert_ok(
            &jigc_doc_stdin(repo, home, &["set-slot", addr, "--from-file", "-"], prose),
            &format!("set-slot {addr}"),
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"change the cache\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");
}

const UNADOPTED: &str = "schema-conformance.unadopted-instance";
const CONFORMANCE_BLOCK: &str = "reconciliation.conformance-block";

/// One foreign file answers **one code and one route** at the store door, the `task
/// validate` door and the `task finalize` preflight — while the managed cell in the very
/// same report keeps its conformance block.
#[test]
fn a_foreign_file_answers_one_code_and_one_route_at_every_door() {
    let repo = TempDir::new("converge");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    // Both cells, committed in git outside jigc — the human-in-git channel.
    fs::create_dir_all(repo.path().join("docs").join("decisions")).expect("mk docs/decisions/");
    fs::write(repo.path().join(NOTES), NOTES_BODY).expect("write the foreign notes");
    fs::write(repo.path().join(MANAGED_ADR), MANAGED_ADR_BODY).expect("write the managed adr");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "seed both cells"]);

    // ── the store door: lift the emitted route verbatim ──────────────────────────────
    let store = jigc(repo.path(), home.path(), &["validate", "--format", "json"]);
    let store_findings = envelope_findings(&store, "jigc validate");
    let store_route = route_of(one_at(&store_findings, UNADOPTED, NOTES, "store"), "store");
    assert!(
        store_route.contains("jigc ingest"),
        "the adoption route names the always-applicable front door; got:\n{store_route}",
    );
    // The path substituted in is the ABSOLUTE one (M53 post-review-fix review, HIGH 2):
    // `jigc migrate <PATH>` roots its argument at the caller's cwd.
    let expected_migrate = format!(
        "jigc migrate {} --as adr",
        repo.path()
            .canonicalize()
            .unwrap_or_else(|_| repo.path().to_path_buf())
            .join(NOTES)
            .display(),
    );
    assert!(
        store_route.contains(&expected_migrate),
        "and — since `migrate-adr` ships — the doctype-directed verb with the path \
         substituted in (the M40 two-verb tier); expected `{expected_migrate}`; \
         got:\n{store_route}",
    );

    // ── the `task validate` door: the same pair, the same bytes ──────────────────────
    let task_a = "first-pass";
    stage_commit_only(repo.path(), home.path(), task_a, task_a);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "validate", task_a, "--format", "json"],
    );
    assert!(
        out.status.success(),
        "an advisory-only sweep does not block `task validate`; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let task_findings = envelope_findings(&out, "task validate");
    assert_eq!(
        route_of(
            one_at(&task_findings, UNADOPTED, NOTES, "task validate"),
            "task validate"
        ),
        store_route,
        "the task door serves the store door's route byte for byte; got:\n{task_findings:#?}",
    );
    assert!(
        by_key(&task_findings, CONFORMANCE_BLOCK, NOTES).is_empty(),
        "and the foreign file draws no conformance block at all — one file, one code; \
         got:\n{task_findings:#?}",
    );

    // The managed cell in the SAME report keeps its block and is never called unadopted.
    let managed = one_at(
        &task_findings,
        CONFORMANCE_BLOCK,
        MANAGED_ADR,
        "task validate",
    );
    assert_eq!(
        managed["severity"].as_str(),
        Some("advisory"),
        "the un-baselined managed doc stays advisory (routed, not recorded); got:\n{managed:#?}",
    );
    assert!(
        by_key(&task_findings, UNADOPTED, MANAGED_ADR).is_empty(),
        "a STAMPED managed doc is never adjudicated foreign; got:\n{task_findings:#?}",
    );

    // ── the `task finalize` preflight: the same pair, the same bytes, and it lands ────
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task_a, "--format", "json"],
    );
    assert!(
        out.status.success(),
        "an advisory-only sweep lands at the finalize boundary; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let finalize_findings = envelope_findings(&out, "task finalize");
    assert_eq!(
        route_of(
            one_at(&finalize_findings, UNADOPTED, NOTES, "task finalize"),
            "task finalize"
        ),
        store_route,
        "the finalize preflight serves the same route; got:\n{finalize_findings:#?}",
    );
    assert!(
        by_key(&finalize_findings, CONFORMANCE_BLOCK, NOTES).is_empty(),
        "no conformance block at the committing door either; got:\n{finalize_findings:#?}",
    );

    // ── routed, NOT recorded ─────────────────────────────────────────────────────────
    let record = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("state")
            .join("file-state.json"),
    )
    .expect("the landed finalize persists the file-state record");
    let value: serde_json::Value = serde_json::from_str(&record).expect("file-state.json parses");
    let hashes = value["hashes"]
        .as_object()
        .expect("the record carries a `hashes` map");
    assert!(
        !hashes.contains_key(NOTES),
        "the foreign file must not be baseline-adopted into the record; got:\n{record}",
    );

    // ── and it re-fires on the next sweep (routed-but-not-recorded recurrence) ────────
    let task_b = "second-pass";
    stage_commit_only(repo.path(), home.path(), task_b, task_b);
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task_b, "--format", "json"],
    );
    assert_ok(&out, "`jigc task finalize` (second pass)");
    let again = envelope_findings(&out, "task finalize (second pass)");
    assert_eq!(
        route_of(
            one_at(&again, UNADOPTED, NOTES, "second sweep"),
            "second sweep"
        ),
        store_route,
        "the advisory re-fires unchanged, never silently absorbed; got:\n{again:#?}",
    );
}

/// **The stamp axis** (M48 Increment 4, T2) — the managed cell stops telling a stamped doc to
/// migrate itself.
///
/// T1 split the *code*: a foreign squatter converges on `schema-conformance.unadopted-instance`,
/// a managed non-conformant doc keeps `reconciliation.conformance-block`. It left the **route**
/// — *"ingest, migrate, or move `<path>` out of the managed location"* — which was written when
/// the arm served foreign ∪ managed and the advice was right for the foreign majority. After
/// the split the arm serves **managed only**, so every clause of it is adoption-or-removal
/// advice about a doc jigc wrote and owns: wrong for 100% of its remaining population.
///
/// The route now reads the doc's **schema-version stamp**, reusing two already-shipped strings
/// and minting no new check id — and the four stamp states **are** the class axis, iterated
/// end to end through the real binary over one un-baselined, non-conformant managed ADR each:
///
/// | stamp | route |
/// |---|---|
/// | **at-version** (2) | the blocking twin's **hand-repair sanction**, byte for byte |
/// | **below-version** (1) | `route_schema_conformance`'s below-version corpus-migration route |
/// | **stamp-absent** (v0-era, parses against `adr.v1`) | its stamp-absent corpus-migration route |
/// | **above-current** (3) | the `ahead` route — no verb named that would block it |
///
/// The at-version route is **lifted from a `DRIFTED`-arm finding produced in this same run**,
/// never re-typed: the sanction is one shared source, and a test that re-typed it would keep
/// passing after the two producers drifted apart. Every cell keeps code
/// `reconciliation.conformance-block` at **advisory** severity — the split is about what the
/// finding *says*, not about what it is — and T1's foreign cell is re-asserted unchanged in the
/// same report, so the two adjudications cannot blur into each other.
#[test]
fn the_managed_advisory_routes_on_the_stamp_never_at_adoption() {
    let repo = TempDir::new("stamp-axis");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    let decisions = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&decisions).expect("mk docs/decisions/");

    // ── seed the sanction's source: a conformant ADR jigc really baselines ───────────
    fs::write(repo.path().join(BASELINED), BASELINED_CONFORMANT).expect("write the conformant adr");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "seed the baseline"]);
    let seed = "seed-the-baseline";
    stage_commit_only(repo.path(), home.path(), seed, "seed the baseline");
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", seed]),
        "`jigc task finalize` (baselining the conformant adr)",
    );

    // ── the axis, plus the drift the sanction is lifted from, plus T1's foreign cell ──
    fs::write(repo.path().join(BASELINED), BASELINED_DRIFTED).expect("break the baselined adr");
    fs::write(repo.path().join(AT_VERSION), MANAGED_ADR_BODY).expect("write the at-version cell");
    fs::write(repo.path().join(BELOW_VERSION), BELOW_VERSION_BODY).expect("write the below cell");
    fs::write(repo.path().join(STAMP_ABSENT), STAMP_ABSENT_BODY).expect("write the absent cell");
    fs::write(repo.path().join(AHEAD), AHEAD_BODY).expect("write the ahead cell");
    fs::write(repo.path().join(NOTES), NOTES_BODY).expect("write the foreign notes");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "the stamp axis"]);

    // One report, every cell — so the split is proven per *file*, not per run.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "walk the stamp axis"],
        ),
        "`jigc start`",
    );
    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "task",
            "validate",
            "walk-the-stamp-axis",
            "--format",
            "json",
        ],
    );
    let findings = envelope_findings(&out, "task validate");
    let door = "task validate";

    // ── the sanction, lifted from the DRIFTED arm in this very report ────────────────
    let drifted = one_at(&findings, CONFORMANCE_BLOCK, BASELINED, door);
    assert_eq!(
        drifted["severity"].as_str(),
        Some("blocking"),
        "the baselined-and-drifted doc is the BLOCKING twin — the arm the sanction belongs to; \
         got:\n{drifted:#?}",
    );
    let sanction = route_of(drifted, door);
    assert!(
        sanction.contains("yours to hand-edit"),
        "and its route is the hand-repair sanction; got:\n{sanction}",
    );

    // ── (a) at-version ⇒ the sanction, byte for byte ─────────────────────────────────
    let at_version = one_at(&findings, CONFORMANCE_BLOCK, AT_VERSION, door);
    assert_eq!(
        at_version["severity"].as_str(),
        Some("advisory"),
        "the un-baselined managed doc stays advisory (routed, not recorded); got:\n{at_version:#?}",
    );
    let at_version_route = route_of(at_version, door);
    assert_eq!(
        at_version_route, sanction,
        "a doc at the CURRENT schema-version has nothing to migrate, so the advisory serves its \
         blocking twin's sanction byte for byte — one shared source, lifted here, never re-typed; \
         got:\n{at_version_route}",
    );
    assert!(
        !at_version_route.contains("migrate"),
        "and it names no migration verb over a doc that is already current; got:\n\
         {at_version_route}",
    );

    // ── (b) below-version ⇒ the corpus-migration route, naming both versions ─────────
    let below = one_at(&findings, CONFORMANCE_BLOCK, BELOW_VERSION, door);
    assert_eq!(
        below["severity"].as_str(),
        Some("advisory"),
        "still advisory — only the route moves; got:\n{below:#?}",
    );
    let below_route = route_of(below, door);
    assert!(
        below_route.starts_with("migrate — "),
        "a STALE managed doc routes at the corpus migration, not at hand-repair — hand-fixing it \
         would repair what `jigc migrate-corpus` must rewrite; got:\n{below_route}",
    );
    assert!(
        below_route.contains("stamped schema-version 1") && below_route.contains("current 2"),
        "and it names both the doc's stamp and the current schema-version; got:\n{below_route}",
    );
    assert!(
        below_route.contains("run the corpus migration"),
        "naming the repair; got:\n{below_route}",
    );
    assert!(
        !below_route.contains("yours to hand-edit"),
        "and never the at-version sanction; got:\n{below_route}",
    );

    // ── (c) stamp-absent (the v0-era corpus) ⇒ the stamp-absent migration route ──────
    let absent = one_at(&findings, CONFORMANCE_BLOCK, STAMP_ABSENT, door);
    assert_eq!(
        absent["severity"].as_str(),
        Some("advisory"),
        "still advisory; got:\n{absent:#?}",
    );
    let absent_route = route_of(absent, door);
    assert!(
        absent_route.starts_with("migrate — ")
            && absent_route.contains("carries no schema-version stamp"),
        "a v0-era doc — unstamped, but parsing against the shipped `adr.v1` snapshot, so MANAGED — \
         routes at the corpus migration that stamps it; got:\n{absent_route}",
    );
    assert!(
        absent_route.contains("schema-version 2"),
        "naming the version it is upgraded to; got:\n{absent_route}",
    );
    assert!(
        by_key(&findings, UNADOPTED, STAMP_ABSENT).is_empty(),
        "and the unstamped v0-era doc is never adjudicated foreign — the parse-against-a-prior \
         arm is what keeps a managed doc off the adoption path; got:\n{findings:#?}",
    );

    // ── (d) above-current ⇒ the ahead route, naming no verb that would block it ──────
    let ahead = one_at(&findings, CONFORMANCE_BLOCK, AHEAD, door);
    assert_eq!(
        ahead["severity"].as_str(),
        Some("advisory"),
        "still advisory; got:\n{ahead:#?}",
    );
    let ahead_route = route_of(ahead, door);
    assert!(
        ahead_route.starts_with("ahead — ") && ahead_route.contains("stamped schema-version 3"),
        "a FUTURE stamp is a third fact: the doc was written to a schema this build does not \
         know; got:\n{ahead_route}",
    );
    assert!(
        ahead_route.contains("upgrade jigc")
            && ahead_route.contains("`jigc migrate-corpus` cannot fix a future stamp"),
        "so the route names the human repairs and says outright that the corpus migration is not \
         one of them; got:\n{ahead_route}",
    );
    assert!(
        !ahead_route.contains("run `jigc migrate-corpus`")
            && !ahead_route.contains("run the corpus migration"),
        "it must never COMMAND a verb that would block it; got:\n{ahead_route}",
    );

    // ── T1's foreign cell, re-asserted unchanged in the same report ──────────────────
    let foreign = one_at(&findings, UNADOPTED, NOTES, door);
    assert_eq!(
        foreign["severity"].as_str(),
        Some("advisory"),
        "the adoption advisory is unchanged by the stamp split; got:\n{foreign:#?}",
    );
    let foreign_route = route_of(foreign, door);
    assert!(
        foreign_route.starts_with("adopt — ") && foreign_route.contains("jigc ingest"),
        "a never-adopted foreign file still routes at ADOPTION — the stamp axis governs the \
         managed cell only; got:\n{foreign_route}",
    );
    assert!(
        by_key(&findings, CONFORMANCE_BLOCK, NOTES).is_empty(),
        "and it draws no conformance block at all — one file, one code; got:\n{findings:#?}",
    );

    // ── no managed cell is ever told to adopt itself ─────────────────────────────────
    for cell in [AT_VERSION, BELOW_VERSION, AHEAD, BASELINED] {
        assert!(
            by_key(&findings, UNADOPTED, cell).is_empty(),
            "`{cell}` is jigc's own doc and is never routed at adoption; got:\n{findings:#?}",
        );
    }
}
