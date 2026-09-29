//! Real-binary regression — **the invocation log carries what a `migrate-corpus` run decided**
//! (M46 completion audit, finding F3; `design/measurement.md` → the record shape).
//!
//! [`crate::migrate_corpus::run`] selected **one** finding set to carry into the log, on the
//! rationale recorded in its own comment:
//!
//! > a run that declined to act on three files is not the same event as a run that found
//! > nothing, and the log is where that difference is readable after the fact.
//!
//! That argument is the rule, and the code three lines under it was the violation. It applies
//! verbatim to `unfilled` — a run that left two `set:` leaves absent is not the same event as
//! one that filled everything — yet `unfilled` reached **neither** branch; and on the
//! non-empty-`blocked` branch `unadopted` **dropped out too**, so the same run that logged its
//! adoption declines at exit 0 stopped logging them the moment one managed doc blocked.
//!
//! Both sets already ride the `--format json` envelope and the text headline, so this is a
//! **log-completeness** gap, not a surface lie: the two cells below read the **invocation log**,
//! not stdout, and assert nothing about what the run printed.
//!
//! The exit code is the other half. `Outcome::with_findings` carries the process status as well
//! as the codes, and the exit rule is `report.blocked`'s emptiness and nothing else
//! (`design/corpus-migration.md`; the exit taxonomy `crates/cli/src/render.rs` →
//! `STORE_EXIT_FLIPS`), so each cell pins its own exit alongside the codes: **0** with an
//! unfilled leaf, **1** with a blocked doc — merging the sets moves neither.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-corpuslog-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — the faithful source a mutated copy mirrors.
fn embedded_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
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

/// Bump `ty`'s manifest `schema-version` `from → to` in the copied pack at `pack`.
fn bump_manifest(pack: &Path, ty: &str, from: u32, to: u32) {
    let manifest_path = pack.join("config").join("schema-manifest.yaml");
    let manifest = fs::read_to_string(&manifest_path).expect("read the copied manifest");
    let bumped = manifest.replacen(
        &format!("- type: {ty}\n    schema-version: {from}"),
        &format!("- type: {ty}\n    schema-version: {to}"),
        1,
    );
    assert_ne!(
        manifest, bumped,
        "the manifest must carry `{ty}` at schema-version {from}",
    );
    fs::write(&manifest_path, bumped).expect("write the bumped manifest");
}

/// A dev-pack copy in which `adr` is bumped **2 → 3** by adding back its header `date` field
/// (`type: date, set: on-create`) — the `migrate_corpus_set_fields` fixture, which migrates the
/// doc as a byte no-op and reports one `migrate-corpus.set-field-unfilled` advisory at exit 0.
fn pack_adding_a_set_derived_adr_field(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen("      - { id: date, type: date, set: on-create }\n", "", 1);
    assert_ne!(
        current, prior,
        "adr.yaml must declare a header `date` field with `set: on-create`",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");

    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// A dev-pack copy in which `adr` is bumped **2 → 3** by adding back its required
/// `## Consequences` section — the `migrate_corpus_halt_causes` fixture, which mints an empty
/// required slot into a doc that lacks the heading, so the gate blocks it at exit 1.
fn pack_adding_the_required_consequences_section(tag: &str) -> TempDir {
    let dir = TempDir::new(tag);
    copy_tree(&embedded_pack_tree(), dir.path());

    let current = fs::read_to_string(dir.path().join("schemas").join("adr.yaml"))
        .expect("read the copied adr.yaml");
    let prior = current.replacen(
        "  - id: consequences\n    slot: { hint: \"Tradeoffs and follow-on effects.\" }\n",
        "",
        1,
    );
    assert_ne!(
        current, prior,
        "adr.yaml must declare the required `consequences` section",
    );
    fs::write(
        dir.path().join("schema-snapshots").join("adr.v2.yaml"),
        prior,
    )
    .expect("write the adr.v2 snapshot");

    bump_manifest(dir.path(), "adr", 2, 3);
    dir
}

/// A conformant, **v2-stamped**, `date`-less ADR — under the v3 `set:`-derived shape its absent
/// `date:` still conforms, so the migration is a byte no-op that leaves the leaf unfilled.
const V2_ADR: &str = "\
---
status: accepted
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

/// The same ADR **without** a `## Consequences` heading — conformant under the prior shape, and
/// the doc the required-section fold mints an empty slot into.
const V2_ADR_WITHOUT_CONSEQUENCES: &str = "\
---
status: accepted
schema-version: 2
---

# Alpha decision

## Context

Session lookups must stay sub-millisecond.

## Options

A distributed cache was weighed and rejected on latency.

## Decision

Keep sessions in a single in-memory node.
";

/// An MADR-shaped decision record at the `adr` doctype's `docs/decisions/` home — **foreign**:
/// no stamp, and its sections match no shipped `adr` shape, so the managed-vs-foreign
/// discriminator excludes it from the fold and reports it as the store door's adoption advisory.
const MADR_DECISION: &str = "\
# 2. Use Postgres

Date: 2026-01-04

## Status

Accepted

## Context and problem

We need a relational store, and we need it before the pilot.

## Chosen option

Postgres, because the team already runs it.
";

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    for args in [
        &["init", "-q"][..],
        &["config", "user.email", "test@example.com"][..],
        &["config", "user.name", "Test"][..],
    ] {
        run_git(root, args);
    }
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    run_git(root, &["add", "."]);
    run_git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run one `git` command in `cwd`, asserting it succeeded.
fn run_git(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Write `files` into `repo` and commit them.
fn commit_files(repo: &Path, files: &[(&str, &str)]) {
    for (path, body) in files {
        let full = repo.join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("mk doc dir");
        }
        fs::write(&full, body).expect("write the doc");
    }
    run_git(repo, &["add", "."]);
    run_git(repo, &["commit", "-q", "-m", "seed the corpus"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
}

/// Turn the invocation-log knob ON in the project cascade — the log is opt-in, and this suite
/// reads nothing else.
fn enable_log(repo: &Path, home: &Path, pack: &Path) {
    let out = jigc(
        repo,
        home,
        pack,
        &["config", "set", "invocation-log", "true"],
    );
    assert!(
        out.status.success(),
        "`jigc config set invocation-log true` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The `finding_codes` of the last logged `migrate-corpus` invocation, plus its logged exit code.
fn logged_migrate_corpus(repo: &Path) -> (Vec<String>, u64) {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    let body = fs::read_to_string(&path).expect("the invocation log exists");
    let record = body
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line).expect("each log line is valid JSON")
        })
        .rfind(|rec| {
            rec["argv"]
                .as_array()
                .is_some_and(|argv| argv.iter().any(|v| v.as_str() == Some("migrate-corpus")))
        })
        .unwrap_or_else(|| panic!("a `migrate-corpus` invocation is logged; log:\n{body}"));
    let codes = record["finding_codes"]
        .as_array()
        .unwrap_or_else(|| panic!("the record carries a `finding_codes` array; got:\n{record:#}"))
        .iter()
        .map(|v| v.as_str().expect("a code string").to_string())
        .collect();
    let exit = record["exit_code"]
        .as_u64()
        .unwrap_or_else(|| panic!("the record carries an `exit_code`; got:\n{record:#}"));
    (codes, exit)
}

/// **The exit-0 branch logs the leaves it left unfilled.** The comment's own argument — *a run
/// that declined to act is not the same event as a run that found nothing* — applies verbatim to
/// `unfilled`, which reached neither branch: a corpus whose `set:`-derived leaf stays absent
/// logged an empty `finding_codes`, indistinguishable after the fact from a run that filled
/// everything the schema declares. The exit stays **0** — nothing blocked.
#[test]
fn the_log_carries_the_unfilled_leaves_of_a_clean_run() {
    let repo = TempDir::new("unfilled");
    let home = TempDir::new("home");
    let pack = pack_adding_a_set_derived_adr_field("pack");
    init_repo(repo.path());
    commit_files(repo.path(), &[("docs/decisions/alpha.md", V2_ADR)]);
    enable_log(repo.path(), home.path(), pack.path());

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "nothing blocked — the exit rule is `blocked`'s emptiness and nothing else; \
         stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let (codes, exit) = logged_migrate_corpus(repo.path());
    assert!(
        codes.contains(&"migrate-corpus.set-field-unfilled".to_string()),
        "the run left a `set:`-derived leaf absent, and the log is where that difference is \
         readable after the fact; finding_codes: {codes:?}",
    );
    assert_eq!(
        exit, 0,
        "and the merge moves no exit code; codes: {codes:?}"
    );
}

/// **The blocking branch logs its adoption declines too.** A refusal and an adoption decline are
/// two things the run decided, and the branch that carried `blocked` dropped `unadopted` — so
/// the same foreign file that was logged at exit 0 vanished from the record the moment one
/// managed doc blocked. The exit stays **1** — something blocked.
#[test]
fn the_blocking_branch_logs_the_adoption_declines_it_used_to_drop() {
    let repo = TempDir::new("blocked");
    let home = TempDir::new("home");
    let pack = pack_adding_the_required_consequences_section("pack");
    init_repo(repo.path());
    commit_files(
        repo.path(),
        &[
            ("docs/decisions/alpha.md", V2_ADR_WITHOUT_CONSEQUENCES),
            ("docs/decisions/0002-use-postgres.md", MADR_DECISION),
        ],
    );
    enable_log(repo.path(), home.path(), pack.path());

    let out = jigc(repo.path(), home.path(), pack.path(), &["migrate-corpus"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(1),
        "a blocked managed doc exits non-zero; stdout:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );

    let (codes, exit) = logged_migrate_corpus(repo.path());
    assert!(
        codes.contains(&"migrate-corpus.prose-needed".to_string()),
        "the refusal is logged, as it always was; finding_codes: {codes:?}",
    );
    assert!(
        codes.contains(&"schema-conformance.unadopted-instance".to_string()),
        "and so is the adoption decline the exit-0 branch logs — one branch's record is not the \
         other's; finding_codes: {codes:?}",
    );
    assert_eq!(
        exit, 1,
        "and the merge moves no exit code; codes: {codes:?}"
    );
}
