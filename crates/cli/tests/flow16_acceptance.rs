//! M13 Increment 5 acceptance (T2) — flow 16, the headline: the first **real**
//! `finalize` snapshot-spawn over a **repeatable-item** `symbol-exists` anchor in the
//! BLOCK direction, with per-item disambiguation, end-to-end through the built `jigc`
//! binary against the real test-built `doc-code` probe.
//!
//! `design/architecture-documentation.md` → The acceptance flow (flow 16), `arch-doc↔code`,
//! and Honest caveats (Finalize snapshot-spawn for an item anchor end-to-end);
//! `design/validation.md` → The `doc-code` probe; `implementation/roadmap.md` → M13
//! Increment 5 grouped-scope bullet 1 (the headline).
//!
//! A single `architecture-documentation`-started task documents a part of jigc's own
//! (Rust) architecture: create the `arch-doc`, author the `overview`, cite an `adr`, and
//! add **two** `components` — **component A** anchored at one present jigc-style Rust
//! symbol, **component B** at a *different* present one (two distinct, independently
//! resolving `implemented-by` anchors). Two halves over the same authored fixture:
//!
//! - **BLOCK (the mandated half)** — delete **component A's** symbol while **B stays
//!   valid**, and let `cites` name a **dangling** adr → `finalize` blocks. The rendered
//!   report (JSON — the report-with-location surface) carries a `doc-code.symbol-exists`
//!   finding whose `location.address` is **A's** item address
//!   (`arch-doc:<slug>#components/<a-id>/implemented-by`) and **never** B's (per-item
//!   disambiguation — each item's anchor resolves against *its own* authored value, not a
//!   clobbered shared one; increment-workflow.md hardening #5), **plus** a
//!   `schema-conformance.ref-resolves` block on the dangling `cites`. HEAD is unchanged,
//!   nothing is promoted. The block direction is the mandated half (hardening #4) — the
//!   PASS direction for one present item anchor is already proven
//!   (`arch_doc_acceptance.rs`), so the inc-4↔inc-5 seam pin (DECISIONS 2026-06-07, M13
//!   inc-4) lands its symbol-exists-BLOCK-on-a-repeatable-item proof HERE.
//! - **PASS** — restore A's symbol and cite a **committed** adr → `finalize` validates
//!   clean, lands exactly **one** `docs(arch-doc):` commit, **promotes** the doc to
//!   `docs/architecture/<slug>.md`, and the working area is cleaned.
//!
//! The two-component A-deleted-B-valid shape is what forces genuine disambiguation: were
//! the item-leaf setters clobbering one shared value, B's deletion-free anchor could not
//! stay green while A's named-and-deleted one blocks. The probe binary is built once and
//! selected via `JIGC_DOC_CODE_PROBE`; the temp repo is a real `git init`; self-cleaning
//! `TempDir`s keep the developer's repo clean (mirrors `arch_doc_acceptance.rs`).

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
            "jigc-flow16-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path.
/// A real subprocess the engine/CLI seam drives — never a mock.
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

/// The two components' source files + the present Rust symbol each `implemented-by`
/// anchor resolves against. Two **distinct** real jigc-style symbols (an edge-index
/// builder and a target-surface enumerator) in two distinct files, so deleting one
/// leaves the other genuinely present — the lever per-item disambiguation turns.
const COMPONENT_A_FILE: &str = "src/edge_index.rs";
const COMPONENT_A_SYMBOL: &str = "rebuild_committed";
const COMPONENT_B_FILE: &str = "src/target_surface.rs";
const COMPONENT_B_SYMBOL: &str = "collect_repeatable";

/// Component A's source carrying its present symbol (the version that resolves clean).
fn component_a_present() -> &'static str {
    "pub fn rebuild_committed() -> u32 {\n    0\n}\n"
}

/// Component A's source with the symbol **deleted** (renamed away) — the file stays,
/// so this is a symbol-absent block, not a file-absent one. B's source is untouched.
fn component_a_deleted() -> &'static str {
    "pub fn rebuilt_renamed() -> u32 {\n    0\n}\n"
}

/// Component B's source carrying its present symbol (never mutated across the two halves).
fn component_b_present() -> &'static str {
    "pub fn collect_repeatable() -> u32 {\n    1\n}\n"
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
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer, and
/// the **two** component source files each carrying its distinct present Rust symbol (the
/// doc-code probe is Rust-grammar-only — these stand in for jigc's own architecture).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::write(repo.join(COMPONENT_A_FILE), component_a_present()).expect("write component A");
    fs::write(repo.join(COMPONENT_B_FILE), component_b_present()).expect("write component B");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the doc-code probe selected via
/// `JIGC_DOC_CODE_PROBE`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
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
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe());
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

/// Set a doc slot from piped stdin, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc_doc(
        repo,
        home,
        &["set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    assert_ok(&out, &format!("set-slot {addr}"));
}

/// Set a doc field, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
    assert_ok(&out, &format!("set-field {addr}"));
}

/// Fill the commit doc bound to `task` with the conventional `docs(arch-doc):` shape so
/// finalize renders a clean git message and the only pass/block levers are the anchors +
/// `cites`.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), "arch-doc");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"document the index layer\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Living architecture doc for the index layer.\n",
    );
}

/// `git show <rev>` in `repo`, capturing output (existence probe on committed paths).
fn git_show(repo: &Path, rev: &str) -> std::process::Output {
    Command::new("git")
        .args(["show", rev])
        .current_dir(repo)
        .output()
        .expect("git show")
}

/// Add one component item under `#components`, capturing + verifying the *emitted* item
/// address (hardening #4 — the emitted bytes are the contract), filling its description
/// and anchoring its `implemented-by` at `anchor`. Returns the emitted item address.
fn add_component(
    repo: &Path,
    home: &Path,
    title: &str,
    expect_addr: &str,
    description: &[u8],
    anchor: &str,
) -> String {
    let added = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            "arch-doc:index-layer#components",
            "--title",
            title,
        ],
        None,
    );
    assert_ok(
        &added,
        &format!("`jigc doc add-item …#components` ({title})"),
    );
    let addr = String::from_utf8(added.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_owned();
    assert_eq!(
        addr, expect_addr,
        "the emitted item address (run verbatim downstream)"
    );
    set_slot(repo, home, &format!("{addr}/description"), description);
    set_field(repo, home, &format!("{addr}/implemented-by"), anchor);
    addr
}

/// Author the arch-doc through the binary: start the workflow, create the doc, author the
/// overview, cite `cites_target`, and add **two** components — A anchored at A's symbol, B
/// at B's *different* symbol. Returns the task id. Every step drives the real binary.
fn author_two_component_arch_doc(repo: &Path, home: &Path, cites_target: &str) -> &'static str {
    let task = "document-the-index-layer";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "document the index layer",
        ],
    );
    assert_ok(&out, "`jigc start --workflow architecture-documentation`");

    let create = jigc_doc(
        repo,
        home,
        &["create", "arch-doc", "--title", "Index layer"],
        None,
    );
    assert_ok(&create, "`jigc doc create arch-doc`");
    let arch = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(arch, "arch-doc:index-layer", "minted arch-doc address");

    set_slot(
        repo,
        home,
        "arch-doc:index-layer#overview",
        b"The index layer owns edge derivation and the target surface.\n",
    );
    // The doc-level n→n `cites` header ref.
    set_field(repo, home, "arch-doc:index-layer#cites", cites_target);

    // Component A — anchored at A's present symbol.
    add_component(
        repo,
        home,
        "Edge index",
        "arch-doc:index-layer#components/edge-index",
        b"Builds the committed edge index.\n",
        &format!("{COMPONENT_A_FILE}#{COMPONENT_A_SYMBOL}"),
    );
    // Component B — anchored at B's DIFFERENT present symbol.
    add_component(
        repo,
        home,
        "Target surface",
        "arch-doc:index-layer#components/target-surface",
        b"Enumerates the repeatable target anchors.\n",
        &format!("{COMPONENT_B_FILE}#{COMPONENT_B_SYMBOL}"),
    );

    fill_commit(repo, home, task);
    task
}

/// Component A's item-anchor address — the one the BLOCK half must name.
const A_ANCHOR_ADDR: &str = "arch-doc:index-layer#components/edge-index/implemented-by";
/// Component B's item-anchor address — the one the BLOCK half must NEVER name (B stays valid).
const B_ANCHOR_ADDR: &str = "arch-doc:index-layer#components/target-surface/implemented-by";

/// The headline (mandated) BLOCK: two components, delete A's symbol while B stays valid,
/// and a dangling `cites` → `finalize` blocks on `doc-code.symbol-exists` naming **A's**
/// item address (and NOT B's — per-item disambiguation) **and** on
/// `schema-conformance.ref-resolves` for the dangling cites; HEAD unchanged, nothing
/// promoted. Then fix both → `finalize` PASSES, lands exactly one `docs(arch-doc):`
/// commit, the doc promotes to `docs/architecture/<slug>.md`, the working area is cleaned.
#[test]
fn flow16_finalize_blocks_naming_only_component_a_then_passes_when_fixed() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // The adr the PASS half will cite is committed up front, so the SOLE block lever in
    // the first finalize is the (then-dangling) cites target + A's deleted symbol.
    commit_cited_adr(repo.path(), home.path());

    // Author with a DANGLING cites; both anchors present at author time.
    let task = author_two_component_arch_doc(repo.path(), home.path(), "adr:no-such-decision");

    // Delete component A's symbol (rename it away); B's file is untouched and stays valid.
    fs::write(repo.path().join(COMPONENT_A_FILE), component_a_deleted())
        .expect("delete component A's symbol");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees A's symbol gone.
    git(repo.path(), &["add", COMPONENT_A_FILE]);

    // ---- BLOCK half ----
    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    // JSON is the report-with-location surface — `location.address` carries the item
    // address the per-item disambiguation proof asserts on (the emitted report bytes).
    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", task],
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a deleted item symbol + dangling cites must make finalize exit non-zero; got:\n{rendered}",
    );

    // The repeatable-item symbol-exists block surfaces, naming A's SPECIFIC item address.
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces doc-code.symbol-exists; got:\n{rendered}",
    );
    assert!(
        rendered.contains(A_ANCHOR_ADDR),
        "the symbol-exists block names component A's item address; got:\n{rendered}",
    );
    // Per-item disambiguation: B stays valid → B's item address never appears (B's
    // identically-keyed `implemented-by` resolved against its own authored value, not a
    // clobbered shared one).
    assert!(
        !rendered.contains(B_ANCHOR_ADDR),
        "component B stays valid — its item address must NOT appear in the block; got:\n{rendered}",
    );
    // And the dangling cites blocks the forward-ref walk.
    assert!(
        rendered.contains("schema-conformance.ref-resolves"),
        "the dangling cites surfaces schema-conformance.ref-resolves; got:\n{rendered}",
    );
    assert!(
        rendered.contains("adr:no-such-decision"),
        "the ref-resolves block names the dangling cites target; got:\n{rendered}",
    );

    // No commit landed; nothing promoted.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
    assert!(
        !repo
            .path()
            .join("architecture")
            .join("index-layer.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );

    // ---- FIX both ----
    // Restore A's symbol so the anchor resolves again.
    fs::write(repo.path().join(COMPONENT_A_FILE), component_a_present())
        .expect("restore component A's symbol");
    // Stage the restoration so the index the doc-code probe validates carries A's symbol.
    git(repo.path(), &["add", COMPONENT_A_FILE]);
    // Re-point the dangling cites at the committed adr through the binary. `cites` is a
    // list-cardinality (`0..*`) ref already carrying a value, so the re-point uses the
    // explicit bracket-list form (the blessed whole-list replace) — a bare single value
    // would be rejected by the list-overwrite guard.
    set_field(
        repo.path(),
        home.path(),
        "arch-doc:index-layer#cites",
        "[adr:use-a-cache]",
    );

    // ---- PASS half ----
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("finalize must pass once both blocks are fixed; got:\n{rendered}"),
    );

    // Exactly one commit landed.
    let fixed: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        fixed,
        after + 1,
        "the fixing finalize lands exactly ONE commit"
    );

    // It is a `docs(arch-doc):` commit.
    let subject = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        subject.starts_with("docs(arch-doc):"),
        "the commit subject is `docs(arch-doc):`; got:\n{subject}",
    );

    // The arch-doc promoted to the fourth `location:` — docs/architecture/<slug>.md.
    let committed = git_show(repo.path(), "HEAD:docs/architecture/index-layer.md");
    assert!(
        committed.status.success(),
        "the arch-doc must promote + commit at docs/architecture/index-layer.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let body = String::from_utf8(committed.stdout).expect("utf-8");
    assert!(
        body.contains("cites: [adr:use-a-cache]"),
        "the promoted arch-doc carries its fixed cites edge (the bracket-list re-point form); got:\n{body}",
    );
    // Both component anchors survived into the promoted doc.
    assert!(
        body.contains(&format!("{COMPONENT_A_FILE}#{COMPONENT_A_SYMBOL}"))
            && body.contains(&format!("{COMPONENT_B_FILE}#{COMPONENT_B_SYMBOL}")),
        "the promoted arch-doc carries both per-component anchors; got:\n{body}",
    );
    // The working-area staging is gone (promoted, not left behind).
    assert!(
        !repo.path().join(".jigc").join("tasks").join(task).exists(),
        "a finalized task's working area is cleaned up",
    );
}

/// Setup — create + finalize `adr:use-a-cache`, the committed decision the PASS half
/// cites. Asserts the ADR lands at its canonical `docs/decisions/` path.
fn commit_cited_adr(repo: &Path, home: &Path) {
    let task = "decide-the-index-strategy";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "decide the index strategy",
        ],
    );
    assert_ok(&out, "`jigc start` (adr setup task)");

    let create = jigc_doc(
        repo,
        home,
        &["create", "adr", "--title", "Use a cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(adr, "adr:use-a-cache");

    set_slot(
        repo,
        home,
        "adr:use-a-cache#context",
        b"Lookups must stay fast.\n",
    );
    set_slot(
        repo,
        home,
        "adr:use-a-cache#decision",
        b"Cache the index.\n",
    );
    set_slot(
        repo,
        home,
        "adr:use-a-cache#consequences",
        b"A cold node re-warms.\n",
    );
    fill_commit(repo, home, task);

    let out = jigc(repo, home, &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize` (adr setup)");
    let committed = git_show(repo, "HEAD:docs/decisions/use-a-cache.md");
    assert!(
        committed.status.success(),
        "the cited ADR must commit at docs/decisions/use-a-cache.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
}
