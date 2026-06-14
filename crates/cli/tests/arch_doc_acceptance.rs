//! M13 Increment 4 acceptance (T4) — the `arch-doc` cites→adr edge pass+block and
//! promotion to the fourth `location:` (`architecture/`), end-to-end through the
//! built `jigc` binary against the real test-built `doc-code` probe.
//!
//! `design/architecture-documentation.md` → The acceptance flow (flow 16);
//! `design/validation.md` → Forward-ref resolution; `design/finalize.md` → Promote;
//! `implementation/roadmap.md` → M13 Increment 4. A single
//! `architecture-documentation`-started task documents one part of jigc's own
//! architecture: create the `arch-doc`, author the `overview`, add one `component`
//! whose `implemented-by` anchors a **present** Rust symbol the working tree carries
//! (so the `check: symbol-exists` doc-code probe passes and `cites` is the SOLE
//! pass/block variable), and cite an adr. Two halves:
//!
//! - **PASS** — `cites` names a **committed** adr → `finalize` validates clean,
//!   lands exactly one `docs(arch-doc):` commit, and **promotes** the doc to
//!   `architecture/<slug>.md` (the fourth `location:`, via the generic
//!   `plan_promotions` loop — zero new finalize code).
//! - **BLOCK** — `cites` names a **dangling** adr → `finalize` blocks on
//!   `schema-conformance.ref-resolves` naming the dangling target + the routing
//!   options, exits non-zero, lands NO commit (`git rev-list --count HEAD`
//!   unchanged) and promotes nothing. The mandated half — a happy-path-only
//!   acceptance would be a masking test (increment-workflow.md hardening #4).
//!
//! INC-4↔INC-5 seam: `implemented-by` is held at a PRESENT symbol in BOTH halves so
//! the symbol-exists doc-code check is never the lever — `cites` alone flips
//! pass→block. The symbol-exists-BLOCK proof on a repeatable-item anchor (delete a
//! component's symbol, assert finalize blocks naming the item address) is Increment
//! 5's owned landing, not this test's concern.
//!
//! The probe binary is built once (a `cargo build` of the pack's `doc-code` crate)
//! and selected via `JIGC_DOC_CODE_PROBE`. No external test crates: the `jigc` path
//! comes from `CARGO_BIN_EXE_jigc`, the temp repo is a real `git init`, and
//! self-cleaning `TempDir`s keep the developer's repo clean.

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
            "jigc-arch-doc-{tag}-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
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
/// A real subprocess the engine/CLI seam drives — never a mock (mirrors
/// `doc_code_gate::doc_code_probe`).
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
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer, and
/// a `src/lib.rs` carrying a known Rust symbol every `implemented-by` anchor resolves
/// against (a present jigc-style symbol — the doc-code probe is Rust-grammar-only).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::write(
        repo.join("src").join("lib.rs"),
        "pub fn present_symbol() -> u32 {\n    42\n}\n",
    )
    .expect("write lib.rs");
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
/// finalize renders a clean git message and the only pass/block lever is `cites`.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), "arch-doc");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"document the cache layer\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Living architecture doc for the cache layer.\n",
    );
}

/// Setup — create + finalize `adr:use-a-cache`, the committed decision the arch-doc's
/// PASS half cites. Asserts the ADR lands at its canonical `decisions/` path.
fn commit_cited_adr(repo: &Path, home: &Path) {
    let task = "decide-the-caching-strategy";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "decide the caching strategy",
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
        b"Cache the sessions.\n",
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
    let committed = jigc_show(repo, "HEAD:decisions/use-a-cache.md");
    assert!(
        committed.status.success(),
        "the cited ADR must commit at decisions/use-a-cache.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
}

/// `git show <rev>` in `repo`, capturing output (existence probe on committed paths).
fn jigc_show(repo: &Path, rev: &str) -> std::process::Output {
    Command::new("git")
        .args(["show", rev])
        .current_dir(repo)
        .output()
        .expect("git show")
}

/// Author the arch-doc through the binary: start the `architecture-documentation`
/// workflow, create the doc, author the overview, cite `cites_target`, and add ONE
/// component anchored at the PRESENT symbol. Returns the task id. Every step drives the
/// real binary; the `add-item` address is captured from stdout and run verbatim
/// (hardening #4 — the emitted bytes are the contract).
fn author_arch_doc(repo: &Path, home: &Path, cites_target: &str) -> &'static str {
    let task = "document-the-cache-layer";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "document the cache layer",
        ],
    );
    assert_ok(&out, "`jigc start --workflow architecture-documentation`");

    let create = jigc_doc(
        repo,
        home,
        &["create", "arch-doc", "--title", "Cache layer"],
        None,
    );
    assert_ok(&create, "`jigc doc create arch-doc`");
    let arch = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(arch, "arch-doc:cache-layer", "minted arch-doc address");

    set_slot(
        repo,
        home,
        "arch-doc:cache-layer#overview",
        b"The cache layer owns ephemeral session state.\n",
    );
    // The doc-level n→n `cites` header ref — the SOLE pass/block lever in this test.
    set_field(repo, home, "arch-doc:cache-layer#cites", cites_target);

    // One component; `add-item` *emits* the item address an agent runs next.
    let added = jigc_doc(
        repo,
        home,
        &[
            "add-item",
            "arch-doc:cache-layer#components",
            "--title",
            "Session store",
        ],
        None,
    );
    assert_ok(&added, "`jigc doc add-item …#components`");
    let addr = String::from_utf8(added.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_owned();
    assert_eq!(
        addr, "arch-doc:cache-layer#components/session-store",
        "the emitted item address (run verbatim downstream)"
    );

    set_slot(
        repo,
        home,
        &format!("{addr}/description"),
        b"Holds session blobs keyed by token.\n",
    );
    // implemented-by held at a PRESENT symbol in BOTH halves (INC-4↔INC-5 seam).
    set_field(
        repo,
        home,
        &format!("{addr}/implemented-by"),
        "src/lib.rs#present_symbol",
    );

    fill_commit(repo, home, task);
    task
}

/// PASS — `cites` names a committed adr → finalize validates clean, lands exactly one
/// `docs(arch-doc):` commit, and promotes the doc to `architecture/<slug>.md`.
#[test]
fn arch_doc_finalize_passes_promotes_to_architecture_when_cites_resolves() {
    let repo = TempDir::new("pass-repo");
    let home = TempDir::new("pass-home");
    init_repo(repo.path());

    // The cited adr is committed → cites resolves.
    commit_cited_adr(repo.path(), home.path());
    let task = author_arch_doc(repo.path(), home.path(), "adr:use-a-cache");

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_ok(
        &out,
        &format!("finalize with a resolving cites; got:\n{rendered}"),
    );

    // Exactly one commit landed.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(after, before + 1, "finalize must land exactly ONE commit");

    // It is a `docs(arch-doc):` commit (rendered from the filled commit doc).
    let subject = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        subject.starts_with("docs(arch-doc):"),
        "the commit subject is `docs(arch-doc):`; got:\n{subject}",
    );

    // The arch-doc promoted to the fourth `location:` — architecture/<slug>.md.
    let committed = jigc_show(repo.path(), "HEAD:architecture/cache-layer.md");
    assert!(
        committed.status.success(),
        "the arch-doc must promote + commit at architecture/cache-layer.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let body = String::from_utf8(committed.stdout).expect("utf-8");
    assert!(
        body.contains("cites: adr:use-a-cache"),
        "the promoted arch-doc carries its cites edge; got:\n{body}",
    );
    // The working-area staging is gone (promoted, not left behind).
    assert!(
        !repo.path().join(".jigc").join("tasks").join(task).exists(),
        "a finalized task's working area is cleaned up",
    );
}

/// BLOCK (the mandated half) — `cites` names a dangling adr → finalize blocks on
/// `schema-conformance.ref-resolves` naming the dangling target + route, exits
/// non-zero, lands NO commit, and promotes nothing. `implemented-by` stays at the
/// present symbol so `cites` is provably the sole lever.
#[test]
fn arch_doc_finalize_blocks_on_ref_resolves_when_cites_dangles() {
    let repo = TempDir::new("block-repo");
    let home = TempDir::new("block-home");
    init_repo(repo.path());

    // No adr is committed: the dangling target resolves in neither surface.
    let task = author_arch_doc(repo.path(), home.path(), "adr:no-such-decision");

    let before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !out.status.success(),
        "a dangling cites must make finalize exit non-zero; got:\n{rendered}",
    );
    // The block is the forward-ref integrity walk over the cites→adr edge.
    assert!(
        rendered.contains("schema-conformance.ref-resolves"),
        "the block surfaces schema-conformance.ref-resolves; got:\n{rendered}",
    );
    // It names the specific dangling target.
    assert!(
        rendered.contains("adr:no-such-decision"),
        "the block names the dangling cites target; got:\n{rendered}",
    );
    // The cites edge is the one that dangles (not a symbol-exists block from the anchor).
    assert!(
        rendered.contains("arch-doc:cache-layer#cites"),
        "the block locates the dangling edge at the cites field; got:\n{rendered}",
    );
    assert!(
        !rendered.contains("doc-code.symbol-exists"),
        "the present anchor must not block — cites is the sole lever; got:\n{rendered}",
    );
    // The three routing options.
    assert!(
        rendered.contains("fix the reference")
            && rendered.contains("create the target in this task")
            && rendered.contains("drop"),
        "the block surfaces the routing options; got:\n{rendered}",
    );

    // No commit landed; nothing promoted to architecture/.
    let after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(before, after, "a blocked finalize creates no commit");
    assert!(
        !repo
            .path()
            .join("architecture")
            .join("cache-layer.md")
            .exists(),
        "a blocked finalize promotes nothing",
    );
}
