//! M42 Increment 12 / T9 — the **M42 rc.6-wave done-picture acceptance suite**, driven
//! end-to-end through the **real `jigc` binary**. Increments 1–11 proved each feature
//! per-feature (`migrate_corpus_value_remap.rs`, `managed_vs_foreign.rs`,
//! `validate_envelope.rs`, `flow38_adr_options.rs`, `corpus_migration_backstop.rs`,
//! `milestone_discard.rs`, `doc_show.rs` / `doc_list.rs`, `methodology_staging_contract.rs`,
//! `changelog_step_subset.rs`); this suite ties them into one arm per claim over the real
//! binary (`design/worked-examples.md` → flow 43; roadmap → M42 Inc 12).
//!
//! **The claim the wave proves: the tool's own routes tell the truth.** Every arm is a route
//! (or a verdict) that lied at rc.5 and now does not.
//!
//! The nine arms, each a `#[test]` over the real binary:
//!
//!   (1) **HEADLINE — the trial's probe-1 upgrade, re-run against a stale-stamped corpus.**
//!       A committed **v1** `deferral-ledger` under an rc-stamped store: `jigc validate`
//!       **BLOCKS** where rc.5 printed *"the committed store validates clean"*, the
//!       `binary-mismatch` advisory names **`jigc migrate-corpus`** instead of handing out the
//!       false all-clear (*"re-run `jigc setup`"* — which re-stamps and silences), `--dry-run`
//!       shows the plan **without writing**, the run **self-commits** its own migration
//!       (pathspec-limited), and `validate` then goes **green**.
//!
//!   (2) **A stock brownfield `CHANGELOG.md` routes to `ingest`** — not `migrate-corpus` — and
//!       reads consistently across all three surfaces (`validate` · `doc list` · `doc show`):
//!       one adoption advisory, one `unregistered` row, one block carrying the very same
//!       adoption route. The sweep **exits non-zero** over it (M46 Inc 3 / T1) while nothing
//!       *blocks* — the flip is the sweep's verdict on its own run, not a gate.
//!
//!   (3) **An OOB edit to the managed `VISION.md` is DETECTED at store scope** — the placement
//!       class was invisible to `jigc validate` (a false green over a tampered managed doc).
//!
//!   (4) **A doc that already carries the optional section migrates cleanly** — the shipped
//!       `adr` v1→v2 stranding: an adopter who hand-authored `## Options` before the bump was
//!       stranded forever.
//!
//!   (5) **A schema bump with no transform kind BLOCKS** — never a silent stamp-bump over a
//!       corpus the new schema does not fit.
//!
//!   (6) **Abandoning a milestone settles the record** — and the terminal **holds**: the verbs
//!       refuse afterwards, so nothing rebuilds the workbench the teardown removed (the M42
//!       completion-audit HIGH — the arm asserted the settled *snapshot* and never ran a verb
//!       after the discard, while the claim it makes is a *lifecycle invariant*).
//!
//!   (7) **`validate` emits → `doc show` reads.** The target string the sweep emits, fed back
//!       **verbatim**, resolves — the read path honours the address grammar validate emits.
//!
//!   (8) **A methodology `dev-task` composes the staging contract** (`git add` + its
//!       consequence) — and it stays **inert** in a workflow that composes neither step.
//!
//!   (9) **`implement-from-spec` prints no command the binary refuses** — every emitted
//!       `jigc doc create …` line, run **verbatim**, is admitted by the gate it composes under.
//!
//! Everything is asserted on the EMITTED bytes / exit codes / committed files of the real
//! binary (`CARGO_BIN_EXE_jigc`). No external test crates beyond `serde_json`.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow43-{tag}-{}-{:?}",
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
        // The fan-out worktrees (arm 6) are ordinary directories under `.jigc/worktrees/`.
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — the
/// `jigc validate` / finalize probe pre-flight resolves it via `JIGC_DOC_CODE_PROBE`
/// (the `flow42_acceptance` / `validate_envelope` idiom).
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

/// The embedded dev-pack tree on disk — the faithful source arm 5's mutated copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Recursively copy `src` into `dst` (both directories), creating `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
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
    String::from_utf8(out.stdout)
        .expect("utf-8 git stdout")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit.
fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the doc-code probe selected,
/// optionally piping `stdin`. Never inherits a harness `JIGC_PACK_DIR` — the embedded
/// shipped packs are the base, so the arms prove exactly what ships.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
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

/// Run `jigc <args>` against an on-disk pack copy (`JIGC_PACK_DIR`) — arm 5 only.
fn jigc_with_pack(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
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

/// Trimmed stdout of an invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim()
        .to_string()
}

/// Combined stdout+stderr, for asserting on a blocked invocation's message.
fn streams_of(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// A repo under a **real `jigc setup`** — the `[dev ▸ methodology]` pack-set a dogfooding
/// project runs (setup writes the `compose-embedded-methodology` marker), `docs-root` at its
/// shipped default `docs/`.
fn setup_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    git_init(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );
    (repo, home)
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the task's commit doc so a finalize renders a clean git message.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Driven by the flow-43 acceptance suite.\n",
    );
}

/// The `jigc validate --format json` envelope plus its exit code.
fn validate_json(repo: &Path, home: &Path) -> (i32, serde_json::Value) {
    let out = jigc(repo, home, &["validate", "--format", "json"], None);
    let stdout = stdout_of(&out);
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!("`jigc validate --format json` emits JSON ({err}):\n{stdout}")
    });
    (out.status.code().expect("an exit code"), json)
}

/// Every finding of `code` in a validate envelope.
fn by_code<'a>(report: &'a serde_json::Value, code: &str) -> Vec<&'a serde_json::Value> {
    report["findings"]
        .as_array()
        .expect("the report carries a `findings` array")
        .iter()
        .filter(|f| f["code"].as_str() == Some(code))
        .collect()
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .parse()
        .expect("commit count parses")
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

// ───────── Arm 1 — HEADLINE: the trial's probe-1 upgrade over a stale-stamped corpus ─────────

/// Author one `deferral-ledger` entry (the `migrate_corpus_value_remap::author_entry` shape).
fn author_entry(repo: &Path, home: &Path, title: &str, kind: &str, trigger: &str, body: &[u8]) {
    let item = jigc(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "deferral-ledger:deferral-ledger#entries",
            "--title",
            title,
        ],
        None,
    );
    assert_ok(&item, "`doc add-item deferral-ledger#entries`");
    let addr = stdout_of(&item);
    set_field(repo, home, &format!("{addr}/kind"), kind);
    set_field(repo, home, &format!("{addr}/trigger"), trigger);
    set_slot(repo, home, &format!("{addr}/body"), body);
}

/// **Arm 1 (HEADLINE).** The rc.5 adoption trial's **probe 1**, re-run against a stale-stamped
/// corpus — and it must now **block at every step it greened**
/// (`completions/artifacts/RC-adoption/impl-rc5/trial-record.md` → Probe 1: *"the stamp claims
/// rc.5, the corpus is still on the rc.4 schema, and validate is green"*).
///
/// The corpus is a committed **v1** `deferral-ledger` carrying the old `kind: D` / `kind: I`
/// enum members — the exact-inverse downgrade of a **real v2 canonical oracle** rendered through
/// the binary (the stale-corpus fixture pattern), under a store stamped by an older binary.
///
/// Four verdicts, in the order the operator meets them:
///   1. **`jigc validate` BLOCKS** (exit non-zero) with `schema-conformance.schema-version-current`
///      targeted at the ledger and routed at **`jigc migrate-corpus`** — rc.5 printed *"the
///      committed store validates clean"*, exit 0 (the false green a CI pipeline would bind).
///   2. **The `binary-mismatch` advisory names `migrate-corpus`** — rc.5's route named only
///      *"re-run `jigc setup`"*, which re-stamps `.jigc/version` and **self-clears the advisory
///      while the corpus stays stale**: the false all-clear the trial's agent survived only by
///      distrusting it.
///   3. **`--dry-run` shows the plan without writing** — rc.5 had no way to look first ("the only
///      way to answer *does my corpus need migrating* is to migrate it").
///   4. **The run self-commits** its own migration, pathspec-limited (rc.5 left the tree dirty and
///      the agent had to reach for raw `git add` — the adapter-contract breach) — and `validate`
///      then goes **green**, with the mismatch advisory falling back to its plain
///      align-or-re-stamp route (naming `migrate-corpus` there would command a verb with nothing
///      left to do).
#[test]
fn probe_1_upgrade_blocks_dry_runs_self_commits_and_greens() {
    let (repo, home) = setup_repo("upgrade");
    let repo = repo.path();
    let home = home.path();

    // The v2 canonical oracle: a real deferral-ledger authored through the production verbs.
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "planning", "M-Test"],
            None,
        ),
        "`jigc start --workflow planning`",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "create",
                "deferral-ledger",
                "--title",
                "Deferral Ledger",
            ],
            None,
        ),
        "`doc create deferral-ledger`",
    );
    author_entry(
        repo,
        home,
        "Cache the index",
        "Decision",
        "M-Store",
        b"Deferred until the store scope lands.\n",
    );
    author_entry(
        repo,
        home,
        "A plugin surface",
        "Idea",
        "M-External",
        b"Parked until a real external domain earns it.\n",
    );
    let canonical_v2 =
        fs::read_to_string(repo.join(".jigc/tasks/m-test/docs/deferral-ledger:deferral-ledger.md"))
            .expect("read the staged canonical v2 deferral-ledger");
    assert!(
        canonical_v2.contains("schema-version: 2")
            && canonical_v2.contains("- kind: Decision")
            && canonical_v2.contains("- kind: Idea"),
        "the oracle is the v2 canonical form; got:\n{canonical_v2}",
    );

    // The stale corpus: the exact-inverse downgrade of the oracle, committed — plus a store
    // stamped by an older binary (the trial's rc.4→rc.5 upgrade state).
    let committed_v1 = canonical_v2
        .replace("schema-version: 2", "schema-version: 1")
        .replace("- kind: Decision", "- kind: D")
        .replace("- kind: Idea", "- kind: I");
    assert_ne!(committed_v1, canonical_v2, "the downgrade actually differs");
    fs::create_dir_all(repo.join("docs")).expect("mk docs/");
    fs::write(repo.join("docs/deferral-ledger.md"), &committed_v1).expect("write the v1 fixture");
    fs::write(repo.join(".jigc/version"), "jigc-version: 1.0.0-rc.4\n").expect("stale the stamp");
    git(repo, &["add", "docs/deferral-ledger.md", ".jigc/version"]);
    git(repo, &["commit", "-q", "-m", "the rc.4-era corpus"]);

    // (1) `jigc validate` BLOCKS — where rc.5 printed "the committed store validates clean".
    let (code, report) = validate_json(repo, home);
    assert_ne!(
        code, 0,
        "an unmigrated MANAGED corpus flips the exit; got:\n{report:#}",
    );
    let stale = by_code(&report, "schema-conformance.schema-version-current");
    assert_eq!(
        stale.len(),
        1,
        "the stale ledger raises its own version-currency break; got:\n{report:#}",
    );
    assert_eq!(
        stale[0]["severity"].as_str(),
        Some("blocking"),
        "the version break is blocking, not a shrug: {:#}",
        stale[0],
    );
    assert!(
        stale[0]["key"]["target"]
            .as_str()
            .is_some_and(|t| t.starts_with("deferral-ledger:deferral-ledger")),
        "the break is keyed at the stale doc: {:#}",
        stale[0],
    );
    assert!(
        stale[0]["route"]
            .as_str()
            .is_some_and(|r| r.contains("jigc migrate-corpus")),
        "the route names the verb that fixes it: {:#}",
        stale[0],
    );

    // (2) The `binary-mismatch` advisory names `migrate-corpus` — never the false all-clear.
    let mismatch = by_code(&report, "store-version.binary-mismatch");
    assert_eq!(
        mismatch.len(),
        1,
        "the older-binary store raises the provenance advisory; got:\n{report:#}",
    );
    let route = mismatch[0]["route"].as_str().expect("the advisory routes");
    assert!(
        route.contains("jigc migrate-corpus"),
        "over a STALE corpus the mismatch route must name `migrate-corpus` FIRST — re-stamping \
         alone clears the advisory and leaves the corpus stale (the trial's false all-clear); \
         got: {route}",
    );

    // (3) `--dry-run` shows the plan and writes NOTHING — no bytes, no commit.
    let before_commits = commit_count(repo);
    let dry = jigc(repo, home, &["migrate-corpus", "--dry-run"], None);
    assert_ok(&dry, "`jigc migrate-corpus --dry-run`");
    let dry_out = stdout_of(&dry);
    assert!(
        dry_out.contains("1 would migrate") && dry_out.contains("docs/deferral-ledger.md"),
        "the dry run SHOWS the pending work (in the conditional — F2, never past-tense over \
         writes that never happened); stdout:\n{dry_out}",
    );
    assert_eq!(
        fs::read_to_string(repo.join("docs/deferral-ledger.md")).expect("read the ledger"),
        committed_v1,
        "the dry run writes NOTHING — the corpus is byte-identical",
    );
    assert_eq!(
        commit_count(repo),
        before_commits,
        "the dry run commits nothing",
    );

    // (4) The run migrates, self-commits (pathspec-limited), and `validate` goes green.
    let migrate = jigc(repo, home, &["migrate-corpus"], None);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    assert!(
        stdout_of(&migrate).contains("1 migrated"),
        "the run migrates the stale ledger; stdout:\n{}",
        stdout_of(&migrate),
    );
    assert_eq!(
        fs::read_to_string(repo.join("docs/deferral-ledger.md")).expect("read the ledger"),
        canonical_v2,
        "the migrated doc is byte-faithful to the v2 canonical oracle (D→Decision, I→Idea, \
         stamp 1→2, every other byte preserved)",
    );
    assert_eq!(
        commit_count(repo),
        before_commits + 1,
        "the migration LANDS through the tool — no raw `git add` drive-around; stdout:\n{}",
        stdout_of(&migrate),
    );
    assert_eq!(
        git(
            repo,
            &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"]
        ),
        "docs/deferral-ledger.md",
        "the self-commit is pathspec-limited to the migrated paths",
    );

    let (green, after) = validate_json(repo, home);
    assert_eq!(
        green, 0,
        "the migrated corpus validates clean; got:\n{after:#}",
    );
    assert!(
        by_code(&after, "schema-conformance.schema-version-current").is_empty(),
        "no version break survives the migration; got:\n{after:#}",
    );
    let settled = by_code(&after, "store-version.binary-mismatch");
    assert_eq!(settled.len(), 1, "the store is still on the older stamp");
    assert!(
        !settled[0]["route"]
            .as_str()
            .expect("the advisory routes")
            .contains("migrate-corpus"),
        "with the corpus current, the mismatch route drops `migrate-corpus` — commanding a verb \
         with nothing to do is the sibling lie: {:#}",
        settled[0],
    );
}

// ───────── Arm 2 — a stock brownfield CHANGELOG.md routes to `ingest`, on all three surfaces ─────────

/// A **real** Keep-a-Changelog file — the stock brownfield `CHANGELOG.md`, sitting at the
/// `changelog` doctype's placement home. Unstamped, and it parses against no shipped
/// `changelog` shape ⇒ **foreign, never adopted** (a file the user never handed to jigc).
const KEEP_A_CHANGELOG: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2026-05-01

### Added

- A thing.
";

/// **Arm 2.** A **stock brownfield repo** — a real Keep-a-Changelog `CHANGELOG.md` plus `jigc
/// setup`, nothing else — is an **adoption** case, not an unmigrated corpus: `jigc validate`
/// raises exactly one advisory routed at **`jigc ingest`** (never `migrate-corpus`, a verb that
/// would do nothing to it), `jigc doc list` shows the file `unregistered`, and `jigc doc show`
/// blocks carrying **the same adoption route**. The sweep exits **non-zero** over it (M46 Inc 3
/// / T1) with **nothing blocking** — the flip is the sweep's own verdict, not a gate.
///
/// Red under a naive family-5 fix: four *blocking* findings on the default brownfield first
/// run, all routed at a verb with nothing to do — the wave's disease in its purest form (a
/// route that lies), and the reason the discriminator ships before the exit flip.
#[test]
fn a_stock_brownfield_changelog_routes_to_ingest_on_all_three_surfaces() {
    let repo = TempDir::new("brownfield");
    let home = TempDir::new("home");
    git_init(repo.path());
    fs::write(repo.path().join("CHANGELOG.md"), KEEP_A_CHANGELOG).expect("write CHANGELOG.md");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "the brownfield changelog"],
    );
    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );

    // Surface 1 — `validate`: **exit non-zero** (M46 Inc 3 / T1 — the adoption advisory is
    // `STORE_EXIT_FLIPS`' fifth member, so the sweep refuses to report a green over a file jigc
    // was never handed), one adoption advisory, routed at the adoption verbs. Nothing *blocks*
    // — the flip is the sweep's verdict on its own run, not a gate the advisory now carries,
    // which is what the next assertion holds.
    let (code, report) = validate_json(repo.path(), home.path());
    assert_ne!(
        code, 0,
        "the brownfield first run refuses the green over the squatter; got:\n{report:#}"
    );
    let blocking: Vec<_> = report["findings"]
        .as_array()
        .expect("findings")
        .iter()
        .filter(|f| f["severity"].as_str() == Some("blocking"))
        .collect();
    assert!(
        blocking.is_empty(),
        "a file the user never handed to jigc blocks nothing; got: {blocking:#?}",
    );
    let unadopted = by_code(&report, "schema-conformance.unadopted-instance");
    assert_eq!(
        unadopted.len(),
        1,
        "the squatter is exactly one fact — the adoption advisory; got:\n{report:#}",
    );
    let route = unadopted[0]["route"].as_str().expect("the advisory routes");
    assert!(
        route.contains("jigc ingest") && route.contains("jigc migrate CHANGELOG.md --as changelog"),
        "the route names the adoption front door + the doctype-directed verb; got: {route}",
    );
    assert!(
        !route.contains("migrate-corpus"),
        "a foreign file is NOT an unmigrated corpus; got: {route}",
    );

    // Surface 2 — `doc list`: the file is LISTED (omitting the very file jigc tells the agent
    // to adopt would send it straight to `cat`), flagged `unregistered` — under the column
    // header that names what that third column is (M47 Inc 10 / T5).
    let listing = jigc(repo.path(), home.path(), &["doc", "list"], None);
    assert_ok(&listing, "`jigc doc list`");
    assert_eq!(
        stdout_of(&listing),
        "id  path  state\nchangelog:changelog  CHANGELOG.md  unregistered",
        "the store surface names the squatter and its registration state",
    );

    // Surface 3 — `doc show`: blocks (it cannot read a doc jigc never adopted) carrying the
    // SAME adoption route — one file, one story, on every surface.
    let show = jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "changelog:changelog"],
        None,
    );
    assert!(
        !show.status.success(),
        "a foreign file has no managed content to read back; got:\n{}",
        streams_of(&show),
    );
    let shown = streams_of(&show);
    assert!(
        shown.contains("jigc ingest") && !shown.contains("migrate-corpus"),
        "the read surface routes the SAME adoption fix the sweep did; got:\n{shown}",
    );
}

// ───────── Arm 3 — an OOB edit to the managed VISION.md is detected at store scope ─────────

/// **Arm 3.** `VISION.md` is a **placement** doctype: managed directly at the repo-root literal
/// file. The read-only file↔CLI-state twin walked only `location:`-bearing schemas, so the whole
/// placement class was invisible to `jigc validate` — an out-of-band edit to a baselined
/// `VISION.md` reported *"no findings — the committed store validates clean"*, a **false green
/// over a tampered managed doc** voiding CLAUDE.md's *"out-of-band edits are detected and
/// routed"* invariant for that class. The edit is now DETECTED, report-only at store scope, and
/// routed to a human.
#[test]
fn an_oob_edit_to_the_managed_vision_is_detected_at_store_scope() {
    let (repo, home) = setup_repo("vision-oob");
    let repo = repo.path();
    let home = home.path();

    // A managed vision, authored and finalized through the production verbs — the landed
    // finalize baselines `VISION.md` in the file-state record.
    let task = "form-the-project-vision";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "form-vision",
                "form the project vision",
            ],
            None,
        ),
        "`jigc start --workflow form-vision`",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "create", "vision", "--title", "Vision"],
            None,
        ),
        "`jigc doc create vision`",
    );
    set_slot(
        repo,
        home,
        "vision:vision#thesis",
        b"A context compiler for coding agents.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#invariants",
        b"The CLI owns structure; the LLM owns prose.\n",
    );
    set_slot(
        repo,
        home,
        "vision:vision#open-questions",
        b"When does a public pack platform earn its keep?\n",
    );
    fill_commit(repo, home, task, "vision");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (form-vision)",
    );
    let baseline = fs::read_to_string(repo.join(".jigc/state/file-state.json"))
        .expect("the file-state record");
    assert!(
        baseline.contains("VISION.md"),
        "precondition: the landed finalize baselines the placement doc at its literal home; \
         record:\n{baseline}",
    );

    // The out-of-band edit: a human appends a sentence through git, outside the CLI.
    let managed = fs::read_to_string(repo.join("VISION.md")).expect("read VISION.md");
    fs::write(
        repo.join("VISION.md"),
        format!("{managed}\nA human appended this out of band.\n"),
    )
    .expect("apply the OOB edit");
    git(repo, &["add", "VISION.md"]);
    git(
        repo,
        &["commit", "-q", "-m", "human edits VISION.md out of band"],
    );

    let (code, report) = validate_json(repo, home);
    assert_eq!(
        code, 0,
        "an OOB edit is content drift — report-only at store scope; got:\n{report:#}",
    );
    let drift = by_code(&report, "file-state.hash-matches");
    assert_eq!(
        drift.len(),
        1,
        "the OOB edit to the managed placement doc is DETECTED — not `no findings`; got:\n{report:#}",
    );
    assert!(
        drift[0]["message"]
            .as_str()
            .is_some_and(|m| m.contains("VISION.md")),
        "the drift names the literal home it read: {:#}",
        drift[0],
    );
    assert!(
        drift[0]["route"].is_string(),
        "the drift routes to a human review: {:#}",
        drift[0],
    );
}

// ───────── Arm 4 — a doc that already carries the optional section migrates cleanly ─────────

/// A **v1** `adr` that already carries a hand-authored `## Options` at its schema-ordered home
/// — the shape an adopter who wrote the alternatives down *before* the v1→v2 bump has committed.
/// Structurally it is already the v2 form; only the stamp is behind.
const ADR_V1_WITH_OPTIONS: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 1
---

# Beta decision

## Context

Session lookups must stay sub-millisecond.

## Options

A shared Redis cache lost on latency budget.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// **Arm 4.** The **stranding defect that shipped**: `AddedOptionalSection` re-spliced a heading
/// the doc already carried, `write::generate_section` refused it, and the doc collected the
/// prose-needing route — a **dead end** (the section is optional and already authored, so there
/// is no prose to write and re-running changes nothing). Any adopter who hand-added `## Options`
/// to an ADR before the `adr` v1→v2 bump was permanently stranded on rc.5. It now migrates: the
/// heading is minted **exactly once**, and the only byte that moves is the stamp digit.
#[test]
fn an_adr_that_already_carries_the_optional_section_migrates_cleanly() {
    let (repo, home) = setup_repo("carries");
    let repo = repo.path();
    let home = home.path();

    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("beta-decision.md"), ADR_V1_WITH_OPTIONS).expect("write the v1 adr");
    git(repo, &["add", "docs"]);
    git(
        repo,
        &["commit", "-q", "-m", "seed a v1 adr carrying Options"],
    );

    let migrate = jigc(repo, home, &["migrate-corpus"], None);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    let report = stdout_of(&migrate);
    assert!(
        report.contains("migrated   docs/decisions/beta-decision.md"),
        "the ADR that already carries `## Options` MIGRATES (rc.5: blocked, and forever); \
         stdout:\n{report}",
    );
    assert_eq!(
        count(&report, "  blocked    "),
        0,
        "nothing is blocked — an already-authored optional section is not a dead end; \
         stdout:\n{report}",
    );

    let after = fs::read_to_string(dir.join("beta-decision.md")).expect("read the migrated adr");
    assert_eq!(
        count(&after, "## Options"),
        1,
        "exactly one `## Options` survives — the heading is never re-spliced; got:\n{after}",
    );
    assert_eq!(
        after,
        ADR_V1_WITH_OPTIONS.replace("schema-version: 1", "schema-version: 2"),
        "the migration moved only the stamp digit — the hand-authored prose is preserved",
    );
}

// ───────── Arm 5 — a schema bump with no transform kind blocks ─────────

/// A conformant, **v2-stamped** ADR at the default docs-root home — the corpus arm 5 runs over.
const ADR_V2: &str = "\
---
status: accepted
date: 2026-06-25
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// **Arm 5.** The **empty-diff backstop**: a dev-pack copy bumps `adr` **2 → 3** with a change no
/// transform kind classifies (a field's `type:`, which the conformance-relevant structural
/// projection carries). The migration **refuses loudly**, routed at *build the transform kind* —
/// never the silent stamp-bump that strands the corpus at *v3-failing-its-own-gate*. The rule
/// both M41 and M42 paid to learn — *a bump without a kind strands the corpus* — is now
/// mechanically enforced, not prose in a checklist nobody is gated on.
#[test]
fn a_schema_bump_with_no_transform_kind_blocks_the_migration() {
    let repo = TempDir::new("backstop");
    let home = TempDir::new("home");
    let pack = TempDir::new("pack");
    git_init(repo.path());
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("mk project layer");

    // The pack copy: `adr` bumped 2→3, its v2 snapshot declaring `date` as `type: string` (so
    // the diff is a field type change — unclassifiable), the shipped `adr.yaml` untouched (so
    // the pack-load freeze gate stays quiet).
    copy_tree(&embedded_pack_tree(), pack.path());
    let current = fs::read_to_string(pack.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "{ id: date, type: date, set: on-create }",
        "{ id: date, type: string, set: on-create }",
        1,
    );
    assert_ne!(current, prior, "adr.yaml declares `date` as `type: date`");
    fs::write(
        pack.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");
    let manifest_path = pack.path().join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        "- type: adr\n    schema-version: 2",
        "- type: adr\n    schema-version: 3",
        1,
    );
    assert_ne!(manifest, bumped, "the manifest carries `adr` at 2");
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");

    let dir = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("alpha.md"), ADR_V2).expect("write the v2 adr");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "seed a v2 adr"]);

    let out = jigc_with_pack(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let report = stdout_of(&out);
    // **A REFUSED MIGRATION EXITS NON-ZERO** (M42 completion audit, Finding 2). This asserted
    // success on the rationale that *a routed block is an interim state* — but the run migrated
    // **nothing** and still reported success, so the refusal was inaudible to a machine and
    // `validate` (exit 1, *run `jigc migrate-corpus`*) → `migrate-corpus` (exit 0) → `validate`
    // looped forever. The *doc* is an interim state; the *run* failed to do what it was asked.
    assert!(
        !out.status.success(),
        "a refused migration exits NON-ZERO; stdout:\n{report}",
    );
    assert!(
        report.contains("0 migrated") && report.contains("1 blocked"),
        "an unclassifiable structural change is REFUSED, never migrated; stdout:\n{report}",
    );
    // The refusal is a real finding — a `migrate-corpus.*` code, a diagnosis, and a route naming
    // the real repair (build the kind), never a prose-authoring dead end.
    assert!(
        report.contains("migrate-corpus.unclassified-change"),
        "the refusal carries its machine code; stdout:\n{report}",
    );
    assert!(
        report.contains("no transform kind") && report.contains("build the transform kind"),
        "the route names the real repair — build the kind; stdout:\n{report}",
    );
    assert_eq!(
        fs::read_to_string(dir.join("alpha.md")).expect("read the adr"),
        ADR_V2,
        "the refused doc is byte-identical — the stamp never flipped",
    );
}

// ───────── Arm 6 — abandoning a milestone settles the record, a joined sub-task stays joined ─────────

/// The `status` leaf of the record header — read from the emitted committed bytes.
fn header_status(body: &str) -> String {
    let end = body.find("\n# ").expect("the record carries an H1");
    body[..end]
        .lines()
        .find_map(|line| line.strip_prefix("status: "))
        .map(str::to_string)
        .unwrap_or_else(|| panic!("the record header carries a `status` field:\n{body}"))
}

/// The `status` leaf recorded for sub-task `sub` — sliced out of its own item field block.
fn item_status(body: &str, sub: &str) -> String {
    let start = body
        .find(&format!("{{#{sub}}}"))
        .unwrap_or_else(|| panic!("the record names sub-task `{sub}`:\n{body}"));
    let rest = &body[start..];
    let end = rest[1..]
        .find("\n### ")
        .map(|i| i + 1)
        .unwrap_or(rest.len());
    rest[..end]
        .lines()
        .find_map(|line| line.trim().strip_prefix("- status: "))
        .map(str::to_string)
        .unwrap_or_else(|| {
            panic!(
                "sub-task `{sub}` carries a `status` leaf:\n{}",
                &rest[..end]
            )
        })
}

/// **Arm 6.** A milestone abandoned mid-flight: `jigc milestone discard --force` **settles the
/// committed record** to the `discarded` terminal in one record-only commit, tears the workbench
/// down — and the terminal **holds**: the milestone verbs refuse afterwards, so nothing rebuilds
/// the workbench the teardown removed. Red on rc.5: `status: active` forever, on the record *and*
/// every task item, with **no verb that could settle it** — a lying committed record and a
/// skippable completion.
///
/// **The second half is the M42 completion-audit HIGH.** Settling the record and removing the
/// workbench are a *snapshot*; the claim — *abandoning a milestone leaves no lying committed
/// record* — is a **lifecycle invariant**, and this arm originally asserted the snapshot and
/// stopped. It never ran a milestone verb after the discard. It had to: the workbench reseed (the
/// fresh-clone continuation path) rebuilt `.jigc/milestones/<id>/` from the committed record
/// without ever reading its `status`, so `provision` re-provisioned worktrees at the abandoned
/// base and `add-task` appended an **active** sub-task to the **discarded** record — both exit 0.
/// The lying record was back. So the arm now exercises the invariant it claims, on both terminals
/// (`joined` had the identical hole — same predicate, same fix).
#[test]
fn discarding_a_milestone_settles_the_record_and_the_terminal_holds() {
    let (repo, home) = setup_repo("abandon");
    let repo = repo.path();
    let home = home.path();
    let milestone = "cache-rework";

    for args in [
        vec!["milestone", "create", "Cache rework"],
        vec!["milestone", "add-task", milestone, "Warm the read cache"],
        vec!["milestone", "add-task", milestone, "Evict cold entries"],
        vec!["milestone", "provision", milestone],
    ] {
        assert_ok(
            &jigc(repo, home, &args, None),
            &format!("`jigc {}`", args.join(" ")),
        );
    }

    let record_rel = format!("docs/milestone-records/{milestone}.md");
    let area = repo.join(".jigc").join("milestones").join(milestone);
    let before = fs::read_to_string(repo.join(&record_rel)).expect("the committed record");
    for sub in ["warm-the-read-cache", "evict-cold-entries"] {
        assert_eq!(
            item_status(&before, sub),
            "active",
            "precondition: `{sub}` is in flight:\n{before}",
        );
    }
    assert!(area.is_dir(), "precondition: the workbench is live");
    let commits = commit_count(repo);

    let discard = jigc(
        repo,
        home,
        &["milestone", "discard", milestone, "--force"],
        None,
    );
    assert_ok(&discard, "`jigc milestone discard --force`");

    let after = fs::read_to_string(repo.join(&record_rel)).expect("the settled record");
    assert_eq!(
        header_status(&after),
        "discarded",
        "the abandoned milestone's record settles to the terminal:\n{after}",
    );
    for sub in ["warm-the-read-cache", "evict-cold-entries"] {
        assert_eq!(
            item_status(&after, sub),
            "discarded",
            "the never-joined sub-task `{sub}` settles to discarded:\n{after}",
        );
    }
    assert!(
        !after.contains("status: active"),
        "no recorded status survives un-settled:\n{after}",
    );
    assert_eq!(
        commit_count(repo),
        commits + 1,
        "the settlement lands as exactly one commit",
    );
    assert_eq!(
        git(
            repo,
            &["diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"]
        ),
        record_rel,
        "and that commit records ONLY the milestone record",
    );
    assert!(
        !area.exists(),
        "the workbench is torn down — `.jigc/milestones/<id>/` had no reachable remover at all",
    );

    // The invariant, not the snapshot: the terminal REFUSES the verbs that would resurrect the
    // milestone or mutate its settled record. Before the fix, both of these exited 0 — the
    // teardown was not a guard, because the reseed put the workbench straight back.
    for args in [
        vec!["milestone", "provision", milestone],
        vec!["milestone", "add-task", milestone, "Purge stale keys"],
        vec!["milestone", "execute", milestone],
        vec!["milestone", "finalize", milestone],
    ] {
        let out = jigc(repo, home, &args, None);
        assert!(
            !out.status.success(),
            "`jigc {}` on a discarded milestone must REFUSE; got exit 0\nstdout:\n{}",
            args.join(" "),
            stdout_of(&out),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("discarded")
                && stderr.contains(&format!("jigc doc show milestone-record:{milestone}")),
            "`jigc {}`: the refusal names the terminal and routes to the record's read surface; \
             stderr:\n{stderr}",
            args.join(" "),
        );
    }
    assert!(
        !area.exists(),
        "no verb rebuilds the abandoned milestone's workbench",
    );
    assert_eq!(
        fs::read_to_string(repo.join(&record_rel)).expect("the record after the refusals"),
        after,
        "and the settled record is byte-identical — no `active` sub-task was appended to it",
    );
    assert_eq!(
        commit_count(repo),
        commits + 1,
        "a refused verb commits nothing",
    );
}

// ───────── Arm 7 — validate emits → doc show reads ─────────

/// A committed, **current** (v2-stamped) ADR carrying an out-of-enum `status` — ordinary content
/// drift, which the store sweep reports (report-only) with a `#<section>/<field>` target.
const ADR_BAD_ENUM: &str = "\
---
status: acceptedish
date: 2026-06-25
schema-version: 2
---

# Cache sessions

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.

## Consequences

A cold node loses its sessions.
";

/// **Arm 7.** **`validate` emits → `doc show` reads.** The sweep's finding target is an address in
/// the one structural grammar (`<type>:<slug>#<section>/<field>`); the read path could not resolve
/// it — `doc show 'adr:x#status/status'` blocked with `store.no-such-item` while `doc set-field`
/// on the *same string* exited 0 (same string, opposite verdicts: the read contract was not closed
/// under its own address grammar). The emitted target is fed back **verbatim** — never
/// reconstructed — and it resolves to the offending value.
#[test]
fn validate_emits_the_address_doc_show_reads() {
    let (repo, home) = setup_repo("roundtrip");
    let repo = repo.path();
    let home = home.path();

    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join("cache-sessions.md"), ADR_BAD_ENUM).expect("write the adr");
    git(repo, &["add", "docs"]);
    git(repo, &["commit", "-q", "-m", "seed a non-conformant adr"]);

    let (code, report) = validate_json(repo, home);
    assert_eq!(
        code, 0,
        "ordinary content drift stays report-only (the masking-trap guard); got:\n{report:#}",
    );
    let drift = by_code(&report, "schema-conformance.field-value-conformant");
    assert_eq!(drift.len(), 1, "one value break; got:\n{report:#}");
    let emitted = drift[0]["key"]["target"]
        .as_str()
        .expect("the finding is keyed at an address")
        .to_string();
    assert_eq!(
        emitted, "adr:cache-sessions#status/status",
        "the sweep emits the leaf address in the one structural grammar",
    );

    // The EMITTED target, verbatim — the contract is the bytes the tool hands the agent.
    let show = jigc(repo, home, &["doc", "show", &emitted], None);
    assert_ok(
        &show,
        &format!("`jigc doc show {emitted}` — the address validate itself emitted"),
    );
    assert_eq!(
        stdout_of(&show),
        "acceptedish",
        "the read surface hands back the very value the sweep complained about",
    );
}

// ───────── Arm 8 — a methodology dev-task composes the staging contract ─────────

/// The composed body with runs of whitespace collapsed — step prose is hard-wrapped, so a
/// sentence can straddle a line break; the contract is the words, not the wrap column.
fn flowed(body: &str) -> String {
    body.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// **Arm 8.** The verified root cause of the trial's **silent partial commits**: the methodology
/// pack composed **zero** mentions of `git add` while the dev pack stated it twice — finalize
/// commits only what the agent staged, and nothing told it so. A `dev-task` now composes the
/// contract *and its consequence*; and the **omitting context** holds: `increment` composes
/// neither step, so the contract is **inert** there — never leaked, never an error.
#[test]
fn a_methodology_dev_task_composes_the_staging_contract() {
    let (repo, home) = setup_repo("staging");
    let repo = repo.path();
    let home = home.path();

    let task = jigc(
        repo,
        home,
        &["start", "--workflow", "dev-task", "add cache"],
        None,
    );
    assert_ok(&task, "`jigc start --workflow dev-task`");
    let composed = flowed(&stdout_of(&task));
    assert!(
        composed.contains("`git add`"),
        "a committing methodology workflow composes the `git add` staging contract; got:\n{composed}",
    );
    assert!(
        composed.contains("only what you have staged"),
        "and states its CONSEQUENCE — finalize commits only what you staged; got:\n{composed}",
    );

    // The omitting context: `increment` composes neither `step:implement` nor `step:finalize`.
    let increment = jigc(
        repo,
        home,
        &["start", "--workflow", "increment", "M99 increment 1"],
        None,
    );
    assert_ok(&increment, "`jigc start --workflow increment`");
    let body = stdout_of(&increment);
    assert!(
        !flowed(&body).contains("`git add`"),
        "the contract is INERT where neither step composes — never leaked; got:\n{body}",
    );
}

// ───────── Arm 9 — implement-from-spec prints no command the binary refuses ─────────

/// Every emitted `jigc doc create …` **command** — the bare command lines and the ``Run: `…` ``
/// lines, never prose that merely mentions the verb in backticks.
fn emitted_create_commands(composed: &str) -> Vec<String> {
    composed
        .lines()
        .map(str::trim)
        .filter_map(|line| {
            let command = line
                .strip_prefix("Run: `")
                .and_then(|rest| rest.strip_suffix('`'))
                .or_else(|| line.starts_with("jigc ").then_some(line))?;
            command
                .starts_with("jigc doc create")
                .then(|| command.to_string())
        })
        .collect()
}

/// **Arm 9.** A prompt whose literal command the binary **refuses** is the defect this arm kills:
/// `step:implement` instructed `jigc doc create changelog` under every workflow that included it,
/// while only `single-task` grants the gate — so `implement-from-spec` printed a command the
/// binary answered with `create.gate-blocked`. Every `jigc doc create …` line
/// `implement-from-spec` composes is now **run verbatim** and **admitted** by the gate it composes
/// under, and it composes **zero** changelog creates.
#[test]
fn implement_from_spec_prints_no_command_the_binary_refuses() {
    let (repo, home) = setup_repo("no-refusal");
    let repo = repo.path();
    let home = home.path();

    let composed = jigc(
        repo,
        home,
        &["start", "--workflow", "implement-from-spec", "work the arm"],
        None,
    );
    assert_ok(&composed, "`jigc start --workflow implement-from-spec`");
    let body = stdout_of(&composed);

    let creates = emitted_create_commands(&body);
    assert!(
        !creates.is_empty(),
        "the arm is vacuous unless the workflow instructs at least one create; got:\n{body}",
    );
    assert!(
        !creates
            .iter()
            .any(|c| c.starts_with("jigc doc create changelog")),
        "`implement-from-spec` does not grant the changelog gate — it must instruct no changelog \
         create at all; got: {creates:?}",
    );

    for command in creates {
        // The emitted bytes, run verbatim — the one substitution is the `<TITLE>` payload the
        // slot marks as the agent's own prose, passed as the single (shell-quoted) argument the
        // emitted line spells.
        let argv: Vec<&str> = command
            .split_whitespace()
            .map(|token| {
                if token == "<TITLE>" {
                    "Probe Decision"
                } else {
                    token
                }
            })
            .collect();
        assert_eq!(argv[0], "jigc", "the emitted line invokes the binary");
        let out = jigc(repo, home, &argv[1..], None);
        assert!(
            out.status.success(),
            "the composed line `{command}` must be ADMITTED by the gate it is composed under \
             (never `create.gate-blocked`); got:\n{}",
            streams_of(&out),
        );
    }
}
