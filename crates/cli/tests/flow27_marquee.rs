//! M25 Increment 7, T1 — the **flow-27 marquee**: the generalized migration done-bar,
//! proven end to end against the built `jigc` binary over `git init` temp repos against
//! the **shipped** dev pack ([worked-examples.md](../../../design/worked-examples.md) →
//! flow 27; [auto-migration.md](../../../design/auto-migration.md) → Generalizing to
//! adr/spec/prd).
//!
//! The per-increment migration tests (`migrate_adr.rs`, `migrate_spec.rs`,
//! `migrate_prd.rs`, the clobber-guard arms) prove each behaviour in isolation; flow 27
//! is the **consolidation** none of them walks: an **N-ADR supersession SET migrated
//! sequentially, one file at a time, target-first**, edges wired across the set, *then*
//! a spec and a prd through the same one-file-one-task spine — with all **six reds**
//! firing on the same corpus shape:
//!
//!   - **HAPPY** — a 2-ADR supersession corpus (`use-mysql` first, the supersession
//!     TARGET; then `use-postgres` which `supersedes: [adr:use-mysql]`), each a distinct
//!     per-file task id `migrate-adr-<slug(path)>`, per-title-slug doc at `docs/decisions/`,
//!     retired + adopted, byte-stable; `adr:use-mysql` resolves in the committed store at
//!     the second finalize (edge integrity across the migrated set). The target ADR is
//!     **dateless** → renders NO date line (red 3, folded). Then a spec → `docs/specs/`
//!     finalizing clean with criteria carrying NO `maps-to-test`, and a prd → `docs/prds/`
//!     with ≥2 repeatable requirement items.
//!   - **RED 1 — write-time ref-shape reject** — a bare slug / wrong-type / unbracketed
//!     comma `supersedes` value blocks at the write verb (`write.malformed-value`), not
//!     deferred to a misleading finalize dangle.
//!   - **RED 2 — the ordering-contract dangle** — superseding a NOT-yet-migrated sibling
//!     makes finalize block on `schema-conformance.ref-resolves`, nothing committed.
//!   - **RED 4 — the finalize-promote clobber guard** — a title-slug collision across the
//!     corpus AND an in-location squatter both block at `finalize.promote-clobber`, the
//!     committed doc byte-intact, nothing committed.
//!   - **RED 5 — the out-of-set supersedes drop** — a prior the operator won't migrate is
//!     dropped to PROSE (no `supersedes` ref authored), and that record finalizes clean
//!     with no edge — the escape hatch, distinct from the dangle that blocks.
//!   - **RED 6 — bare finalize without `--approve`** — exits non-zero (review-pending),
//!     the foreign original byte-intact, nothing committed.
//!
//! Every assertion drives the EMITTED bytes / exit code of the real `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) over the shipped dev pack (`JIGC_PACK_DIR`).

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
            "jigc-flow27marquee-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`.
fn run_jigc(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn().expect("spawn the jigc binary");
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The combined stdout+stderr of a finalize block (the rendered findings ride either
/// stream depending on format), for substring assertions.
fn streams(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

/// The canonical destination directory for a migrated doctype.
fn canonical_dir(doctype: &str) -> &'static str {
    match doctype {
        "adr" => "docs/decisions",
        "spec" => "docs/specs",
        "prd" => "docs/prds",
        other => panic!("flow 27 migrates only adr/spec/prd, not {other:?}"),
    }
}

/// The shipped schema for a doctype, loaded for the byte-stable round-trip. `adr`/`spec`
/// carry a pack-declared `code-anchor` (`supersedes`/`maps-to-test` resp. need the
/// `doc-code` type threaded in); `prd` loads bare (mirrors the per-doctype migrate tests).
fn shipped_schema(pack: &Path, doctype: &str) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join(format!("{doctype}.yaml")))
        .unwrap_or_else(|e| panic!("read shipped {doctype} schema: {e}"));
    let mut schema = if doctype == "prd" {
        engine::schema::load_schema(&yaml).expect("shipped prd schema loads")
    } else {
        let types = vec![engine::schema::PackTypeDecl {
            name: "code-anchor".to_owned(),
            adjudicator: "doc-code".to_owned(),
            check: "symbol-exists".to_owned(),
        }];
        engine::schema::load_schema_with_types(&yaml, &types)
            .unwrap_or_else(|e| panic!("shipped {doctype} schema loads: {e:?}"))
    };
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The per-file migration task id for a repo-relative source `rel` — the production
/// derivation ([`crate::start::mint_migration_in_repo`]): strip `.md`, fold path
/// separators to `-`, slugify, prefix `migrate-<doctype>-`.
fn migration_task(doctype: &str, rel: &str) -> String {
    let stem = rel.strip_suffix(".md").unwrap_or(rel);
    let folded: String = stem
        .chars()
        .map(|c| if c == '/' { '-' } else { c })
        .collect();
    format!("migrate-{doctype}-{}", engine::slug::slugify(&folded))
}

/// Write a foreign doc at `rel` and **commit it**, so its retirement lands as a tracked
/// deletion in the finalize commit (the realistic flow — a pre-existing committed doc).
fn commit_foreign(repo: &Path, rel: &str, body: &str) {
    let path = repo.join(rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create source parent dir");
    fs::write(&path, body).expect("write foreign source");
    git(repo, &["add", rel]);
    git(repo, &["commit", "-q", "-m", &format!("track {rel}")]);
}

/// `jigc migrate <rel> --as <doctype>` — returns the composed migration workflow stdout.
fn migrate(repo: &Path, home: &Path, pack: &Path, rel: &str, doctype: &str) -> String {
    ok_stdout(
        run_jigc(repo, home, pack, &["migrate", rel, "--as", doctype], None),
        "jigc migrate --as <doctype>",
    )
}

/// `jigc doc author <doctype> --from-file -` over `task`, piping the declarative `payload`.
fn author(
    repo: &Path,
    home: &Path,
    pack: &Path,
    doctype: &str,
    task: &str,
    payload: &str,
) -> std::process::Output {
    run_jigc(
        repo,
        home,
        pack,
        &["doc", "author", doctype, "--from-file", "-", "--task", task],
        Some(payload.as_bytes()),
    )
}

/// `jigc task finalize <task> [--approve]` raw output.
fn finalize(
    repo: &Path,
    home: &Path,
    pack: &Path,
    task: &str,
    approve: bool,
) -> std::process::Output {
    let mut args = vec!["task", "finalize", task];
    if approve {
        args.push("--approve");
    }
    run_jigc(repo, home, pack, &args, None)
}

/// Assert the canonical `<dir>/<slug>.md` on disk round-trips byte-stable through the
/// shipped schema: `render(&schema, &instance_from_source(&schema, x)) == x`. Returns the
/// on-disk body.
fn assert_committed_byte_stable(repo: &Path, pack: &Path, doctype: &str, slug: &str) -> String {
    let on_disk = fs::read_to_string(repo.join(canonical_dir(doctype)).join(format!("{slug}.md")))
        .unwrap_or_else(|e| panic!("the canonical {doctype} {slug} is on disk: {e}"));
    let schema = shipped_schema(pack, doctype);
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed doc re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated {doctype} is byte-stable across parse -> render:\n{on_disk}",
    );
    on_disk
}

/// Assert a follow-up `jigc ingest` reports `<dir>/<slug>.md` adopted — never
/// `unmanaged` / `needs-reconcile`.
fn assert_ingest_adopted(repo: &Path, home: &Path, pack: &Path, doctype: &str, slug: &str) {
    let ingest = ok_stdout(run_jigc(repo, home, pack, &["ingest"], None), "jigc ingest");
    let needle = format!("{}/{slug}.md", canonical_dir(doctype));
    let row = ingest
        .lines()
        .find(|l| l.contains(&needle))
        .unwrap_or_else(|| panic!("ingest must report the managed {doctype}:\n{ingest}"));
    assert!(
        row.contains("adopted") && !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed {doctype} ingests as adopted:\n{row}",
    );
}

// ── The cross-referencing ADR corpus (Nygard-shaped, off-canonical at docs/adr/) ──────

/// ADR-0001 — the supersession TARGET, migrated FIRST. **Dateless** (no `Date:` line) so
/// the doc-level date-suppression must render no date (red 3, folded into the happy walk).
const FOREIGN_USE_MYSQL: &str = "\
# 1. Use MySQL

## Status

Deprecated

## Context

The first product needed a relational datastore with familiar operations.

## Decision

We will use MySQL as the primary datastore.

## Consequences

Operators run and back up a MySQL instance.
";

/// ADR-0001's canonical rewrite — `Deprecated` mapped onto the 3-enum as `superseded` (it
/// was later superseded by 0007), NO `date:` key (the source is dateless).
const PAYLOAD_USE_MYSQL: &str = r#"title: "Use MySQL"
sections:
  - id: status
    set:
      status: superseded
  - id: context
    set:
      context: "<<A relational datastore with familiar operations was needed.>>"
  - id: decision
    set:
      decision: "<<We chose MySQL as the primary datastore.>>"
  - id: consequences
    set:
      consequences: "<<Operators run and back up a MySQL instance.>>"
"#;

/// ADR-0007 — supersedes 0001. Migrated SECOND, after 0001 is COMMITTED, so the forward
/// `supersedes` ref resolves against the committed store (edge integrity across the set).
const FOREIGN_USE_POSTGRES: &str = "\
# 7. Use Postgres

Date: 2021-08-03

## Status

Accepted

## Context

We outgrew MySQL's operational story and want stronger consistency guarantees.

## Decision

We will migrate the primary datastore to PostgreSQL.

## Consequences

A data migration is required; operators learn Postgres tooling.
";

/// ADR-0007's canonical rewrite — `supersedes: "[adr:use-mysql]"` (the BRACKET-list 0..*
/// form, the comma footgun avoided), the dated source keeps its transcribed date.
const PAYLOAD_USE_POSTGRES: &str = r#"title: "Use Postgres"
sections:
  - id: status
    set:
      status: accepted
      date: "2021-08-03"
      supersedes: "[adr:use-mysql]"
  - id: context
    set:
      context: "<<We outgrew MySQL's operational story and want stronger consistency.>>"
  - id: decision
    set:
      decision: "<<We will migrate the primary datastore to PostgreSQL.>>"
  - id: consequences
    set:
      consequences: "<<A data migration is required; operators learn Postgres tooling.>>"
"#;

/// HAPPY — the cross-referencing ADR corpus + a spec + a prd, one file at a time,
/// target-first. Asserts the edge wires across the set, byte-stability throughout, the
/// dateless render (red 3), the spec's optional anchor, and the prd's repeatable items.
#[test]
fn flow27_corpus_spec_and_prd_migrate_one_file_at_a_time() {
    let repo = TempDir::new("happy");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // ── ADR-0001 (use-mysql) — the supersession target, migrated FIRST ───────────────
    let mysql_rel = "docs/adr/0001-use-mysql.md";
    commit_foreign(repo.path(), mysql_rel, FOREIGN_USE_MYSQL);
    migrate(repo.path(), home.path(), &pack, mysql_rel, "adr");
    let mysql_task = migration_task("adr", mysql_rel);
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &mysql_task,
            PAYLOAD_USE_MYSQL,
        ),
        "doc author adr (use-mysql)",
    );
    assert_eq!(
        authored, "adr:use-mysql",
        "the target ADR mints its per-title slug",
    );
    let approve = finalize(repo.path(), home.path(), &pack, &mysql_task, true);
    assert!(
        approve.status.success(),
        "the target ADR finalizes clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    // Dateless → NO date line (red 3): no fabricated decision history.
    let mysql_body = assert_committed_byte_stable(repo.path(), &pack, "adr", "use-mysql");
    assert!(
        !mysql_body.contains("date:"),
        "the dateless target ADR renders no date line (no fabricated today):\n{mysql_body}",
    );
    assert!(
        mysql_body.contains("status: superseded"),
        "the foreign `Deprecated` is mapped onto the enum as `superseded`:\n{mysql_body}",
    );
    assert!(
        !repo.path().join(mysql_rel).exists(),
        "the target ADR's foreign original is retired",
    );

    // ── ADR-0007 (use-postgres) — supersedes use-mysql, now COMMITTED ────────────────
    let pg_rel = "docs/adr/0007-use-postgres.md";
    commit_foreign(repo.path(), pg_rel, FOREIGN_USE_POSTGRES);
    migrate(repo.path(), home.path(), &pack, pg_rel, "adr");
    let pg_task = migration_task("adr", pg_rel);
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &pg_task,
            PAYLOAD_USE_POSTGRES,
        ),
        "doc author adr (use-postgres)",
    );
    assert_eq!(
        authored, "adr:use-postgres",
        "the superseding ADR mints its slug"
    );
    let approve = finalize(repo.path(), home.path(), &pack, &pg_task, true);
    assert!(
        approve.status.success(),
        "the superseding ADR finalizes clean — `adr:use-mysql` resolves in the committed \
         store (edge integrity across the migrated set); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    let pg_body = assert_committed_byte_stable(repo.path(), &pack, "adr", "use-postgres");
    assert!(
        pg_body.contains("supersedes:") && pg_body.contains("adr:use-mysql"),
        "the committed superseding ADR carries its forward supersedes edge:\n{pg_body}",
    );
    assert!(
        pg_body.contains("date: 2021-08-03"),
        "the dated source's transcribed date survives the migration:\n{pg_body}",
    );

    // Both managed records adopt, both per-title slugs present in the store.
    assert_ingest_adopted(repo.path(), home.path(), &pack, "adr", "use-mysql");
    assert_ingest_adopted(repo.path(), home.path(), &pack, "adr", "use-postgres");

    // ── A spec → docs/specs/, criteria with NO maps-to-test, finalizing clean ─────────────
    const FOREIGN_SPEC: &str = "\
# Authentication Spec

## Overview

Users must be able to sign in with OAuth before the product launches.

## Acceptance Criteria

- A user can sign in with their Google account.
- A session expires 24 hours after sign-in.
";
    const PAYLOAD_SPEC: &str = r#"title: "Auth spec"
sections:
  - id: goal
    set:
      goal: "<<Users can authenticate via OAuth before launch.>>"
  - id: context
    set:
      context: "<<The product has no auth today; required before public launch.>>"
  - id: criteria
    items:
      - title: "Sign in with Google"
        set:
          statement: "<<A user can sign in with their Google account.>>"
      - title: "Sessions expire"
        set:
          statement: "<<A session expires 24 hours after sign-in.>>"
"#;
    let spec_rel = "docs/specs/auth.md";
    commit_foreign(repo.path(), spec_rel, FOREIGN_SPEC);
    migrate(repo.path(), home.path(), &pack, spec_rel, "spec");
    let spec_task = migration_task("spec", spec_rel);
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "spec",
            &spec_task,
            PAYLOAD_SPEC,
        ),
        "doc author spec",
    );
    assert_eq!(
        authored, "spec:auth-spec",
        "the spec mints its per-title slug"
    );
    let approve = finalize(repo.path(), home.path(), &pack, &spec_task, true);
    assert!(
        approve.status.success(),
        "the spec finalizes clean — criteria with NO maps-to-test (optional anchor); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    let spec_body = assert_committed_byte_stable(repo.path(), &pack, "spec", "auth-spec");
    assert!(
        !spec_body.contains("maps-to-test"),
        "a migrated spec with no test anchors carries no `maps-to-test` field:\n{spec_body}",
    );
    let spec_schema = shipped_schema(&pack, "spec");
    let spec_doc =
        engine::parse::parse_sections(&spec_schema, &spec_body).expect("committed spec re-parses");
    assert_eq!(
        spec_doc
            .sections
            .iter()
            .find(|s| s.id == "criteria")
            .expect("criteria section present")
            .items
            .len(),
        2,
        "exactly the two migrated criteria re-parse:\n{spec_body}",
    );

    // ── A prd → docs/prds/, requirements as a repeatable section (≥2 items) ───────────────
    const FOREIGN_PRD: &str = "\
# Habit Tracker PRD

## Vision

A mobile app that turns good intentions into daily streaks.

## Requirements

- A user can log a habit with a single tap from the home screen.
- The current streak is shown front and center.
";
    const PAYLOAD_PRD: &str = r#"title: "Habit tracker"
sections:
  - id: vision
    set:
      vision: "<<A tracker that turns intentions into daily streaks.>>"
  - id: requirements
    items:
      - title: "Log a habit in one tap"
        set:
          statement: "<<Logging a habit takes a single tap from the home screen.>>"
      - title: "Show the current streak"
        set:
          statement: "<<The current streak is shown front and center.>>"
  - id: context
    set:
      context: "<<Built for solo users who abandon heavyweight planners.>>"
"#;
    let prd_rel = "docs/product-brief.md";
    commit_foreign(repo.path(), prd_rel, FOREIGN_PRD);
    migrate(repo.path(), home.path(), &pack, prd_rel, "prd");
    let prd_task = migration_task("prd", prd_rel);
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "prd",
            &prd_task,
            PAYLOAD_PRD,
        ),
        "doc author prd",
    );
    assert_eq!(
        authored, "prd:habit-tracker",
        "the prd mints its per-title slug"
    );
    let approve = finalize(repo.path(), home.path(), &pack, &prd_task, true);
    assert!(
        approve.status.success(),
        "the prd finalizes clean; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    let prd_body = assert_committed_byte_stable(repo.path(), &pack, "prd", "habit-tracker");
    let prd_schema = shipped_schema(&pack, "prd");
    let prd_doc =
        engine::parse::parse_sections(&prd_schema, &prd_body).expect("committed prd re-parses");
    assert_eq!(
        prd_doc
            .sections
            .iter()
            .find(|s| s.id == "requirements")
            .expect("requirements section present")
            .items
            .len(),
        2,
        "exactly the two migrated requirements re-parse as repeatable items:\n{prd_body}",
    );
}

/// RED 1 — the write-time ref-shape check. On a staged migration ADR, a `set-field` of
/// `supersedes` with a bare slug / wrong type / unbracketed comma blocks at the write verb
/// (`write.malformed-value`) — immediate, not deferred to a misleading finalize dangle.
#[test]
fn flow27_red_write_time_ref_shape_reject() {
    let repo = TempDir::new("refshape");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // A staged migration ADR with no supersedes yet (so set-field hits the absent-field
    // generation path, which still runs the value shape check).
    let pg_rel = "docs/adr/0007-use-postgres.md";
    commit_foreign(repo.path(), pg_rel, FOREIGN_USE_POSTGRES);
    migrate(repo.path(), home.path(), &pack, pg_rel, "adr");
    let task = migration_task("adr", pg_rel);
    const PAYLOAD_NO_REF: &str = r#"title: "Use Postgres"
sections:
  - id: status
    set:
      status: accepted
  - id: context
    set:
      context: "<<Context.>>"
"#;
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &task,
            PAYLOAD_NO_REF,
        ),
        "doc author adr (no ref)",
    );

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    // bare slug (no type prefix) · wrong type · unbracketed comma — each a `write.malformed-value`.
    for bad in ["use-mysql", "spec:use-mysql", "adr:a, adr:b"] {
        let out = run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "set-field",
                "adr:use-postgres#status/supersedes",
                "--value",
                bad,
                "--task",
                &task,
            ],
            None,
        );
        assert!(
            !out.status.success(),
            "the malformed ref {bad:?} blocks at the write verb (non-zero exit); stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("write.malformed-value"),
            "the block is the write-time ref-shape reject for {bad:?}; stderr:\n{stderr}",
        );
    }
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the write-time ref-shape rejects commit nothing",
    );
}

/// RED 2 — the dependency-ordering contract, enforced by reality. Superseding a
/// not-yet-migrated sibling makes finalize block on `schema-conformance.ref-resolves`
/// (the forward ref resolves in neither the committed store nor the working area),
/// committing nothing — there is no cross-task transaction machinery.
#[test]
fn flow27_red_ordering_contract_dangle_blocks() {
    let repo = TempDir::new("ordering");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // Migrate the SUPERSEDING ADR first, before its target use-mysql is migrated/committed.
    let pg_rel = "docs/adr/0007-use-postgres.md";
    commit_foreign(repo.path(), pg_rel, FOREIGN_USE_POSTGRES);
    migrate(repo.path(), home.path(), &pack, pg_rel, "adr");
    let task = migration_task("adr", pg_rel);
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &task,
            PAYLOAD_USE_POSTGRES,
        ),
        "doc author adr (supersedes a not-yet-migrated sibling)",
    );

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    // Validation runs BEFORE the review gate, so even the bare finalize surfaces the dangle.
    let out = finalize(repo.path(), home.path(), &pack, &task, false);
    assert!(
        !out.status.success(),
        "a forward ref to a not-yet-migrated sibling must block finalize (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    assert!(
        rendered.contains("schema-conformance.ref-resolves") && rendered.contains("adr:use-mysql"),
        "the block is the ordering-contract dangle naming the unresolved target:\n{rendered}",
    );
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the dangle block commits nothing",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("decisions")
            .join("use-postgres.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );
}

/// RED 4a — the finalize-promote clobber guard, title-slug collision across the corpus.
/// With `docs/decisions/use-mysql.md` committed, a SECOND foreign ADR at a distinct path whose
/// authored title slugs to the SAME `use-mysql` blocks at `finalize.promote-clobber`: no
/// clobber, nothing committed, the committed ADR byte-intact, the 2nd foreign not retired.
#[test]
fn flow27_red_clobber_guard_title_slug_collision() {
    let repo = TempDir::new("collision");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // First migration lands a committed managed ADR at docs/decisions/use-mysql.md.
    let first_rel = "docs/adr/0001-use-mysql.md";
    commit_foreign(repo.path(), first_rel, FOREIGN_USE_MYSQL);
    migrate(repo.path(), home.path(), &pack, first_rel, "adr");
    let first_task = migration_task("adr", first_rel);
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &first_task,
            PAYLOAD_USE_MYSQL,
        ),
        "doc author adr (first)",
    );
    assert!(
        finalize(repo.path(), home.path(), &pack, &first_task, true)
            .status
            .success(),
        "the first migration lands",
    );
    let committed_before = fs::read(
        repo.path()
            .join("docs")
            .join("decisions")
            .join("use-mysql.md"),
    )
    .expect("committed adr");

    // A SECOND foreign ADR at a distinct path, authored with the SAME title (slugs to the
    // already-occupied use-mysql).
    let second_rel = "docs/adr/0042-mysql-again.md";
    commit_foreign(repo.path(), second_rel, FOREIGN_USE_MYSQL);
    let count_with_foreign: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    migrate(repo.path(), home.path(), &pack, second_rel, "adr");
    let second_task = migration_task("adr", second_rel);
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &second_task,
            PAYLOAD_USE_MYSQL,
        ),
        "doc author adr (colliding title)",
    );
    assert_eq!(
        authored, "adr:use-mysql",
        "the colliding author succeeds in the working area — the collision is a \
         finalize-promote concern, not a write-time one",
    );

    let out = finalize(repo.path(), home.path(), &pack, &second_task, true);
    assert!(
        !out.status.success(),
        "a colliding title must block at finalize-promote (non-zero exit); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    assert!(
        rendered.contains("finalize.promote-clobber")
            && rendered.contains("docs/decisions/use-mysql.md"),
        "the block is the clobber guard naming the destination it refused to overwrite:\n{rendered}",
    );
    assert_eq!(
        count_with_foreign,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the clobber block commits nothing past the foreign-tracking commit",
    );
    assert_eq!(
        fs::read(
            repo.path()
                .join("docs")
                .join("decisions")
                .join("use-mysql.md")
        )
        .expect("committed adr"),
        committed_before,
        "the already-committed ADR is byte-intact after the refused clobber",
    );
    assert!(
        repo.path().join(second_rel).exists(),
        "the blocked migration does not retire the 2nd foreign original",
    );
}

/// RED 4b — the finalize-promote clobber guard, in-location squatter. `jigc migrate
/// docs/decisions/<slug>.md --as adr` where the canonical path itself holds a committed managed
/// doc blocks at `finalize.promote-clobber`, the squatter byte-intact, nothing committed.
#[test]
fn flow27_red_clobber_guard_in_location_squatter() {
    let repo = TempDir::new("squatter");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // Land a committed managed ADR at the canonical docs/decisions/use-mysql.md.
    let first_rel = "docs/adr/0001-use-mysql.md";
    commit_foreign(repo.path(), first_rel, FOREIGN_USE_MYSQL);
    migrate(repo.path(), home.path(), &pack, first_rel, "adr");
    let first_task = migration_task("adr", first_rel);
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &first_task,
            PAYLOAD_USE_MYSQL,
        ),
        "doc author adr (first)",
    );
    assert!(
        finalize(repo.path(), home.path(), &pack, &first_task, true)
            .status
            .success(),
        "the first migration lands",
    );

    let squatter_rel = "docs/decisions/use-mysql.md";
    let squatter_before = fs::read(repo.path().join(squatter_rel)).expect("the squatter on disk");
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // Migrate the canonical doc IN PLACE: source-path == the canonical destination.
    migrate(repo.path(), home.path(), &pack, squatter_rel, "adr");
    let task = migration_task("adr", squatter_rel);
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &task,
            PAYLOAD_USE_MYSQL,
        ),
        "doc author adr (in-location squatter)",
    );

    let out = finalize(repo.path(), home.path(), &pack, &task, true);
    assert!(
        !out.status.success(),
        "an in-location squatter must block at finalize-promote (non-zero exit); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    assert!(
        rendered.contains("finalize.promote-clobber") && rendered.contains(squatter_rel),
        "the block is the clobber guard naming the canonical destination:\n{rendered}",
    );
    assert_eq!(
        fs::read(repo.path().join(squatter_rel)).expect("the squatter on disk"),
        squatter_before,
        "the in-location squatter is byte-intact after the refused clobber",
    );
    assert_eq!(
        count_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the squatter block commits nothing",
    );
}

/// RED 5 — the out-of-set supersedes drop. A prior the operator will NOT migrate is
/// dropped to PROSE (captured in the decision slot, NO `supersedes` ref authored) and the
/// record finalizes clean with no edge — the escape hatch, distinct from the dangle (red
/// 2) that blocks. Drives the EMITTED committed bytes: a `supersedes:` ref would have
/// dangled-blocked finalize.
#[test]
fn flow27_red_out_of_set_supersedes_dropped_to_prose() {
    let repo = TempDir::new("outofset");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let pg_rel = "docs/adr/0007-use-postgres.md";
    commit_foreign(repo.path(), pg_rel, FOREIGN_USE_POSTGRES);
    migrate(repo.path(), home.path(), &pack, pg_rel, "adr");
    let task = migration_task("adr", pg_rel);

    // The foreign ADR references a prior (`use-mysql`) the operator will NOT migrate. The
    // guidance folds it into PROSE rather than authoring a dangling `supersedes` ref.
    const PAYLOAD_PROSE_DROP: &str = r#"title: "Use Postgres"
sections:
  - id: status
    set:
      status: accepted
      date: "2021-08-03"
  - id: context
    set:
      context: "<<This supersedes the un-migrated MySQL decision, captured here in prose only.>>"
  - id: decision
    set:
      decision: "<<We will migrate the primary datastore to PostgreSQL.>>"
  - id: consequences
    set:
      consequences: "<<A data migration is required.>>"
"#;
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &task,
            PAYLOAD_PROSE_DROP,
        ),
        "doc author adr (out-of-set drop to prose)",
    );

    let approve = finalize(repo.path(), home.path(), &pack, &task, true);
    assert!(
        approve.status.success(),
        "the out-of-set prior dropped to prose finalizes CLEAN (no dangling ref); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    let body = assert_committed_byte_stable(repo.path(), &pack, "adr", "use-postgres");
    assert!(
        !body.contains("supersedes:"),
        "the out-of-set prior is dropped to prose, NEVER authored as a ref:\n{body}",
    );
    assert!(
        body.contains("un-migrated MySQL decision"),
        "the dropped prior survives as prose in the record:\n{body}",
    );
}

/// RED 6 — the bare finalize without `--approve`. A conformant migration blocks at the
/// review gate (exit non-zero), the foreign original byte-intact, nothing committed — the
/// human is the only content-faithfulness check (Framing A).
#[test]
fn flow27_red_bare_finalize_without_approve_blocks() {
    let repo = TempDir::new("review");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let mysql_rel = "docs/adr/0001-use-mysql.md";
    commit_foreign(repo.path(), mysql_rel, FOREIGN_USE_MYSQL);
    let foreign_before = fs::read(repo.path().join(mysql_rel)).expect("the foreign on disk");
    migrate(repo.path(), home.path(), &pack, mysql_rel, "adr");
    let task = migration_task("adr", mysql_rel);
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "adr",
            &task,
            PAYLOAD_USE_MYSQL,
        ),
        "doc author adr",
    );

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let bare = finalize(repo.path(), home.path(), &pack, &task, false);
    assert!(
        !bare.status.success(),
        "a migration finalize without --approve must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&bare.stdout),
        String::from_utf8_lossy(&bare.stderr),
    );
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the review-gate block commits nothing",
    );
    assert_eq!(
        fs::read(repo.path().join(mysql_rel)).expect("the foreign on disk"),
        foreign_before,
        "the review-gate block leaves the foreign original byte-intact",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("decisions")
            .join("use-mysql.md")
            .exists(),
        "the review-gate block promotes nothing",
    );
}
