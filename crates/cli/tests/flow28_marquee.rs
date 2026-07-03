//! M26 Increment 3, T1 — the **flow-28 marquee**: the migration arc's last doctype
//! (`arch-doc`) proven end to end against the built `jigc` binary over `git init` temp
//! repos against the **shipped** dev pack and the test-built `doc-code` probe
//! (worked-examples.md → flow 28; auto-migration.md → Acceptance corpus (HYBRID, C1),
//! doc↔code on a migrated arch-doc, committed-first ordering; DECISIONS.md 2026-06-18
//! forks C1/C2/C3).
//!
//! Increment 2's `migrate_arch_doc.rs` proves the spine on a single synthetic component;
//! flow 28 is the **HYBRID-corpus consolidation** none of the per-increment tests walks —
//! the two arms settled at M26 planning (C1), the C2 per-item-disambiguation mandate, and
//! the committed-first-ordering cites contract:
//!
//!   - **REAL-FOREIGN arm** — a foreign arc42/C4/README-architecture doc, anchors accepted
//!     **file-only** (a non-Rust target → `symbol-exists` degrades to file-exists, the
//!     in-the-wild fidelity bound): migrate → author → finalize `--approve` → retired +
//!     adopted, byte-stable.
//!   - **SYNTHETIC-over-jigc-Rust arm** (labeled synthetic) carrying the **C2 mandate** —
//!     first provision a **committed `adr`** (the flow-16 precondition; jigc ships none and
//!     `arch-doc.allows-create` cannot mint one in-task), then migrate an arch-doc with
//!     **≥2 components carrying real, independently-resolving `implemented-by` anchors** over
//!     fixture Rust + a `cites → adr` over the committed adr — ONE batch author call, all
//!     anchors + the cites resolve, finalize lands clean, byte-stable, adopted.
//!   - **The blocking C2 red** — delete component A's anchored **fixture** symbol (a throwaway
//!     clone, never a live jigc dep) → finalize blocks at `doc-code.symbol-exists` naming
//!     **A's own item address** `arch-doc:<slug>#components/<a-id>/implemented-by` while B's
//!     address is absent (each item resolves against its own authored value).
//!   - **The committed-first-ordering dangling-cites red** — an out-of-store `[adr:<absent>]`
//!     blocks at `schema-conformance.ref-resolves` naming the dangling target, the resolving
//!     anchors held constant so cites is the sole block lever.
//!
//! Every assertion drives the EMITTED bytes / exit code of the real `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) over the shipped dev pack (`JIGC_PACK_DIR`) and the real
//! `doc-code` probe (`JIGC_DOC_CODE_PROBE`). No pack/engine/CLI/schema change.

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
        let unique = format!(
            "jigc-flow28marquee-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path — a
/// real subprocess the engine/CLI seam drives over each migrated `implemented-by` anchor
/// (never a mock). Mirrors `migrate_arch_doc.rs`.
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

// ── The fixture code the synthetic arm's anchors resolve against — throwaway CLONES in
//    the temp repo, never a live jigc dependency, so the C2 red can delete component A's
//    symbol without breaking jigc's own build. ──────────────────────────────────────────

/// Component A (Lexer) fixture, carrying the present symbol `scan_blocks`.
const LEXER_FILE: &str = "src/lexer.rs";
fn lexer_present() -> &'static str {
    "pub fn scan_blocks() -> u32 {\n    0\n}\n"
}
/// Component A with its symbol **renamed away** — the file stays, so the C2 red is a
/// symbol-absent block, not a file-absent one.
fn lexer_symbol_deleted() -> &'static str {
    "pub fn scan_renamed() -> u32 {\n    0\n}\n"
}

/// Component B (Writer) fixture, carrying the present symbol `render_item_at`.
const WRITER_FILE: &str = "src/writer.rs";
fn writer_present() -> &'static str {
    "pub fn render_item_at() -> u32 {\n    0\n}\n"
}

/// The real-foreign arm's anchor target — a **non-Rust** file present in the repo, so the
/// authored `#symbol` is never grammar-checked (`symbol-exists` degrades to file-exists).
const GATEWAY_FILE: &str = "src/gateway.py";
fn gateway_present() -> &'static str {
    "class GatewayService:\n    def start(self):\n        return 0\n"
}

/// Initialize a real git repo with one commit, the `.jigc/config/` project layer, and the
/// fixture sources the migrated anchors resolve against (two Rust clones for the synthetic
/// arm, one non-Rust file for the real arm).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::create_dir_all(root.join("src")).expect("mk src");
    fs::write(root.join(LEXER_FILE), lexer_present()).expect("write lexer fixture");
    fs::write(root.join(WRITER_FILE), writer_present()).expect("write writer fixture");
    fs::write(root.join(GATEWAY_FILE), gateway_present()).expect("write gateway fixture");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`, and
/// the `doc-code` probe selected via `JIGC_DOC_CODE_PROBE`, optionally piping `stdin`.
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
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
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

/// The combined stdout+stderr of an invocation, for substring assertions.
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
        "arch-doc" => "docs/architecture",
        other => panic!("flow 28 migrates only adr/arch-doc, not {other:?}"),
    }
}

/// The shipped schema for a doctype, loaded for the byte-stable round-trip. Both `adr`
/// (`cites-code`) and `arch-doc` (`implemented-by`) carry a pack-declared `code-anchor`, so
/// the schema only loads with that type threaded in (mirrors the shipped pack's
/// `code-anchor → doc-code` decl).
fn shipped_schema(pack: &Path, doctype: &str) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join(format!("{doctype}.yaml")))
        .unwrap_or_else(|e| panic!("read shipped {doctype} schema: {e}"));
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
    }];
    let mut schema = engine::schema::load_schema_with_types(&yaml, &types)
        .unwrap_or_else(|e| panic!("shipped {doctype} schema loads: {e:?}"));
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The per-file migration task id for a repo-relative source `rel` — the production
/// derivation: strip `.md`, fold path separators to `-`, slugify, prefix
/// `migrate-<doctype>-`.
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

/// `jigc --format json task finalize <task> --approve` — the report-with-location surface
/// whose `location.address` carries the per-item anchor address the C2 red asserts on (the
/// human format prints only the bare anchor string, never the item address).
fn finalize_approve_json(
    repo: &Path,
    home: &Path,
    pack: &Path,
    task: &str,
) -> std::process::Output {
    run_jigc(
        repo,
        home,
        pack,
        &["--format", "json", "task", "finalize", task, "--approve"],
        None,
    )
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

/// Assert a follow-up `jigc ingest` reports `<dir>/<slug>.md` adopted — never `unmanaged` /
/// `needs-reconcile`.
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

/// Provision a **committed** managed `adr` at `docs/decisions/<slug>.md` by migrating a foreign
/// dated ADR end to end — the flow-16 cited-target precondition the synthetic arm leans on
/// (jigc's store ships no managed adr; `arch-doc.allows-create` cannot mint one in-task).
/// Returns the committed adr's per-title slug.
fn provision_committed_adr(repo: &Path, home: &Path, pack: &Path) -> String {
    const FOREIGN_ADR: &str = "\
# 1. Pipeline architecture

## Status

Accepted

## Context

The build needed a deterministic compile pipeline.

## Options

Alternatives were weighed and rejected.

## Decision

We will compose the pipeline from discrete stages.

## Consequences

Each stage is independently testable.
";
    const PAYLOAD_ADR: &str = r#"title: "Pipeline architecture"
sections:
  - id: status
    set:
      status: accepted
  - id: context
    set:
      context: "<<The build needed a deterministic compile pipeline.>>"
  - id: decision
    set:
      decision: "<<We compose the pipeline from discrete stages.>>"
  - id: consequences
    set:
      consequences: "<<Each stage is independently testable.>>"
"#;
    let adr_rel = "docs/adr/0001-pipeline.md";
    commit_foreign(repo, adr_rel, FOREIGN_ADR);
    migrate(repo, home, pack, adr_rel, "adr");
    let adr_task = migration_task("adr", adr_rel);
    let authored = ok_stdout(
        author(repo, home, pack, "adr", &adr_task, PAYLOAD_ADR),
        "doc author adr (cited precondition)",
    );
    assert_eq!(
        authored, "adr:pipeline-architecture",
        "the precondition ADR mints its per-title slug",
    );
    assert!(
        finalize(repo, home, pack, &adr_task, true).status.success(),
        "the cited precondition ADR finalizes clean",
    );
    "pipeline-architecture".to_owned()
}

// ── The hybrid corpus ────────────────────────────────────────────────────────────────

/// The REAL-FOREIGN arm — an arc42/README-flavoured architecture doc whose one component
/// points at a **non-Rust** file (the in-the-wild fidelity case). Off-canonical at `docs/`.
const FOREIGN_REAL_ARCH_DOC: &str = "\
# System Architecture

## Introduction

The gateway service terminates inbound traffic and routes it to internal services.

## Modules

### Request router

Routes inbound requests to the right internal handler. See src/gateway.py.
";

/// The real arm's canonical rewrite — overview + one component anchored **file-only** at a
/// non-Rust path (`src/gateway.py#GatewayService`): the file exists, so the anchor resolves;
/// `#GatewayService` is NEVER grammar-checked (the Rust-only probe degrades symbol-exists to
/// file-exists). No `cites` (the real arm carries no in-store decision).
const PAYLOAD_REAL_ARCH_DOC: &str = r#"title: "Gateway service"
sections:
  - id: overview
    set:
      overview: "<<The gateway service terminates inbound traffic and routes to internal services.>>"
  - id: components
    items:
      - title: "Request router"
        set:
          description: "<<Routes inbound requests to the right internal handler.>>"
          implemented-by: "src/gateway.py#GatewayService"
"#;

/// The SYNTHETIC-over-jigc-Rust arm (labeled synthetic) — ≥2 components, each anchored at a
/// real, **independently-resolving** fixture symbol, plus a `cites → adr` over the committed
/// precondition adr. The C2 mandate corpus.
const PAYLOAD_SYNTHETIC_ARCH_DOC: &str = r#"title: "Parser subsystem"
sections:
  - id: meta
    set:
      cites: "[adr:pipeline-architecture]"
  - id: overview
    set:
      overview: "<<The parser subsystem turns source bytes into a structured document.>>"
  - id: components
    items:
      - title: "Lexer"
        set:
          description: "<<Scans source bytes into block tokens.>>"
          implemented-by: "src/lexer.rs#scan_blocks"
      - title: "Writer"
        set:
          description: "<<Renders a parsed document back to canonical bytes.>>"
          implemented-by: "src/writer.rs#render_item_at"
"#;

/// REAL-FOREIGN arm: migrate → author → finalize `--approve` → ingest. A non-Rust anchor
/// degrades symbol-exists to file-exists; the canonical doc is byte-stable, retired, adopted.
#[test]
fn flow28_real_foreign_arm_migrates_file_only_anchor() {
    let repo = TempDir::new("real");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let rel = "docs/ARCHITECTURE.md";
    commit_foreign(repo.path(), rel, FOREIGN_REAL_ARCH_DOC);
    let composed = migrate(repo.path(), home.path(), &pack, rel, "arch-doc");
    assert!(
        composed.contains("Routes inbound requests"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );
    let task = migration_task("arch-doc", rel);
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "arch-doc",
            &task,
            PAYLOAD_REAL_ARCH_DOC,
        ),
        "doc author arch-doc (real arm)",
    );
    assert_eq!(
        authored, "arch-doc:gateway-service",
        "the real arm mints its per-title slug",
    );

    // The non-Rust anchor's `#symbol` is never checked — finalize lands clean (file-exists
    // is the floor; symbol-exists degraded away, recorded as a bound, not data loss).
    let approve = finalize(repo.path(), home.path(), &pack, &task, true);
    assert!(
        approve.status.success(),
        "the real arm finalizes clean — the non-Rust anchor degrades symbol-exists to \
         file-exists; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    let body = assert_committed_byte_stable(repo.path(), &pack, "arch-doc", "gateway-service");
    assert!(
        body.contains("src/gateway.py#GatewayService"),
        "the committed real-arm arch-doc carries the file-only anchor:\n{body}",
    );
    assert!(
        !repo.path().join(rel).exists(),
        "the real arm retires the foreign original",
    );
    assert_ingest_adopted(
        repo.path(),
        home.path(),
        &pack,
        "arch-doc",
        "gateway-service",
    );
}

/// SYNTHETIC-over-jigc-Rust arm: provision a committed adr first, then migrate → author
/// (ONE batch call) → finalize `--approve` → ingest. ≥2 independently-resolving
/// `implemented-by` anchors + `cites [adr:<committed>]` all resolve; byte-stable, adopted.
#[test]
fn flow28_synthetic_rust_arm_two_anchors_and_cites_over_committed_adr() {
    let repo = TempDir::new("synthetic");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // Precondition: a committed adr in the store for the arch-doc to cite (committed-first
    // ordering — the cited adr lands BEFORE the citing arch-doc).
    let adr_slug = provision_committed_adr(repo.path(), home.path(), &pack);
    assert!(
        repo.path()
            .join("docs")
            .join("decisions")
            .join(format!("{adr_slug}.md"))
            .exists(),
        "the cited adr is committed before the arch-doc migrates",
    );

    let rel = "fixtures/parser-subsystem.md";
    commit_foreign(
        repo.path(),
        rel,
        "# Parser subsystem\n\n## Overview\n\nTurns source bytes into a structured document.\n",
    );
    migrate(repo.path(), home.path(), &pack, rel, "arch-doc");
    let task = migration_task("arch-doc", rel);

    // ONE declarative batch author call carries both anchors + the cites (Framing A).
    let authored = ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "arch-doc",
            &task,
            PAYLOAD_SYNTHETIC_ARCH_DOC,
        ),
        "doc author arch-doc (synthetic arm, one batch call)",
    );
    assert_eq!(
        authored, "arch-doc:parser-subsystem",
        "the synthetic arm mints its per-title slug",
    );

    let approve = finalize(repo.path(), home.path(), &pack, &task, true);
    assert!(
        approve.status.success(),
        "the synthetic arm finalizes clean — both implemented-by anchors resolve over the \
         fixture Rust AND cites resolves over the committed adr; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    let body = assert_committed_byte_stable(repo.path(), &pack, "arch-doc", "parser-subsystem");
    assert!(
        body.contains("src/lexer.rs#scan_blocks") && body.contains("src/writer.rs#render_item_at"),
        "the committed arch-doc carries both independently-resolving anchors:\n{body}",
    );
    assert!(
        body.contains("cites:") && body.contains("adr:pipeline-architecture"),
        "the committed arch-doc carries its resolving cites edge over the committed adr:\n{body}",
    );

    // Both managed records adopt — the cited adr and the citing arch-doc.
    assert_ingest_adopted(repo.path(), home.path(), &pack, "adr", &adr_slug);
    assert_ingest_adopted(
        repo.path(),
        home.path(),
        &pack,
        "arch-doc",
        "parser-subsystem",
    );
}

/// The C2 BLOCKING red — per-item disambiguation. With both anchors authored resolving,
/// delete component **A's** fixture symbol (file stays → symbol-absent, not file-absent):
/// finalize blocks at `doc-code.symbol-exists` naming **A's own item address** while B's is
/// absent — each item resolves against its own authored value. Authored WITHOUT cites so the
/// symbol is the sole block lever.
#[test]
fn flow28_c2_red_delete_component_a_symbol_blocks_naming_a_item_address() {
    let repo = TempDir::new("c2red");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let rel = "fixtures/parser-subsystem.md";
    commit_foreign(
        repo.path(),
        rel,
        "# Parser subsystem\n\n## Overview\n\nTurns source bytes into a structured document.\n",
    );
    migrate(repo.path(), home.path(), &pack, rel, "arch-doc");
    let task = migration_task("arch-doc", rel);

    // Two resolving anchors, NO cites (the symbol is the sole block lever).
    const PAYLOAD_NO_CITES: &str = r#"title: "Parser subsystem"
sections:
  - id: overview
    set:
      overview: "<<The parser subsystem turns source bytes into a structured document.>>"
  - id: components
    items:
      - title: "Lexer"
        set:
          description: "<<Scans source bytes into block tokens.>>"
          implemented-by: "src/lexer.rs#scan_blocks"
      - title: "Writer"
        set:
          description: "<<Renders a parsed document back to canonical bytes.>>"
          implemented-by: "src/writer.rs#render_item_at"
"#;
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "arch-doc",
            &task,
            PAYLOAD_NO_CITES,
        ),
        "doc author arch-doc (two anchors, no cites)",
    );

    // Delete component A's (Lexer) symbol; component B (Writer) stays intact.
    fs::write(repo.path().join(LEXER_FILE), lexer_symbol_deleted())
        .expect("delete component A's anchored symbol");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees A's symbol gone.
    git(repo.path(), &["add", LEXER_FILE]);

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let out = finalize_approve_json(repo.path(), home.path(), &pack, &task);
    assert!(
        !out.status.success(),
        "a vanished component-A symbol must block finalize (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    assert!(
        rendered.contains("doc-code.symbol-exists")
            && rendered.contains("arch-doc:parser-subsystem#components/lexer/implemented-by"),
        "the block is doc-code.symbol-exists naming A's OWN item address:\n{rendered}",
    );
    assert!(
        !rendered.contains("arch-doc:parser-subsystem#components/writer/implemented-by"),
        "B's anchor stays valid — only A's item address is named in the block:\n{rendered}",
    );
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the doc-code block commits nothing",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("architecture")
            .join("parser-subsystem.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );
}

/// The committed-first-ordering dangling-cites red — an out-of-store `[adr:<absent>]` blocks
/// finalize at `schema-conformance.ref-resolves` naming the dangling target. The two anchors
/// resolve, so cites is the sole block lever; nothing committed, nothing promoted.
#[test]
fn flow28_dangling_cites_out_of_store_blocks_at_ref_resolves() {
    let repo = TempDir::new("citesred");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let rel = "fixtures/parser-subsystem.md";
    commit_foreign(
        repo.path(),
        rel,
        "# Parser subsystem\n\n## Overview\n\nTurns source bytes into a structured document.\n",
    );
    migrate(repo.path(), home.path(), &pack, rel, "arch-doc");
    let task = migration_task("arch-doc", rel);

    // cites an adr that lives in NEITHER the committed store NOR the working area (no
    // committed-first provision) — the resolving anchors are held constant.
    const PAYLOAD_DANGLING_CITES: &str = r#"title: "Parser subsystem"
sections:
  - id: meta
    set:
      cites: "[adr:no-such-decision]"
  - id: overview
    set:
      overview: "<<The parser subsystem turns source bytes into a structured document.>>"
  - id: components
    items:
      - title: "Lexer"
        set:
          description: "<<Scans source bytes into block tokens.>>"
          implemented-by: "src/lexer.rs#scan_blocks"
      - title: "Writer"
        set:
          description: "<<Renders a parsed document back to canonical bytes.>>"
          implemented-by: "src/writer.rs#render_item_at"
"#;
    ok_stdout(
        author(
            repo.path(),
            home.path(),
            &pack,
            "arch-doc",
            &task,
            PAYLOAD_DANGLING_CITES,
        ),
        "doc author arch-doc (dangling cites)",
    );

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let out = finalize(repo.path(), home.path(), &pack, &task, true);
    assert!(
        !out.status.success(),
        "a dangling cites must block finalize (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = streams(&out);
    assert!(
        rendered.contains("schema-conformance.ref-resolves")
            && rendered.contains("adr:no-such-decision"),
        "the block is ref-resolves naming the dangling cites target (committed-first \
         ordering not honoured):\n{rendered}",
    );
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the dangling-cites block commits nothing",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("architecture")
            .join("parser-subsystem.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );
}
