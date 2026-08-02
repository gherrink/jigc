//! M27 Increment 4 acceptance (T1) — flow 29, the marquee: doc↔code validation turned on
//! for a **non-Rust** target. M10/M13 proved the `doc-code` differentiator over Rust by
//! pointing the anchors at jigc's own codebase (the Rust-only grammar limit); M27
//! generalizes the probe Rust→six languages, so a citation into a **TypeScript** or a
//! **Python** file genuinely resolves against reality instead of silently passing. This
//! flow mirrors flow 16 (`flow16_acceptance.rs`) — an `arch-doc` with **two** components,
//! each carrying an independently-resolving `implemented-by` anchor, the
//! per-item-disambiguation A-deleted/B-valid proof — but on a **polyglot** repo:
//! component A's anchor at a real **TypeScript** symbol, component B's at a real **Python**
//! symbol.
//!
//! `design/worked-examples.md` → flow 29; `design/validation.md` → Multi-language
//! resolution; `DECISIONS.md` → 2026-06-19 M27 planning, fork F7; `implementation/roadmap.md`
//! → M27 Increment 4 grouped-scope bullet 1.
//!
//! Three arms over the SAME authored fixture (the per-anchor contrast that IS the masking
//! guard — a resolving `doc-code` check emits no finding, so the only honest witness that
//! the probe ran is the pass↔block flip, per language):
//!
//! - **BLOCK A (TypeScript)** — delete component A's TS symbol while B's Python symbol
//!   stays valid → `finalize` blocks on `doc-code.symbol-exists` naming **A's** item
//!   address (`arch-doc:gateway#components/edge-router/implemented-by`) and **never** B's;
//!   HEAD unchanged, nothing promoted.
//! - **BLOCK B (Python)** — symmetrically delete component B's Python symbol while A's TS
//!   symbol stays valid → block names **B's** item address and **never** A's.
//! - **PASS** — both symbols present → `finalize` validates clean, lands exactly **one**
//!   `docs(arch-doc):` commit, promotes the doc to `architecture/gateway.md`, the working
//!   area is cleaned, and the report carries **no** `doc-code` block.
//!
//! The probe is resolved through the **production sibling path** — found beside the running
//! `jigc` (`<bin-dir>/doc-code`, **no** `JIGC_DOC_CODE_PROBE` override), the path a real
//! install hits, exercising the six-grammar set a normal `cargo build` links in (the
//! `flow13_acceptance.rs` `env_remove` + cargo-build-sibling idiom). The temp repo is a
//! real `git init`; self-cleaning `TempDir`s keep the developer's repo clean.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow29-{tag}-{}-{:?}",
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

/// Component A's source file + symbol — a real **TypeScript** symbol (a `class_declaration`,
/// a citable node kind in the TS allowlist).
const COMPONENT_A_FILE: &str = "src/api.ts";
const COMPONENT_A_SYMBOL: &str = "RateRouter";
/// Component B's source file + symbol — a real **Python** symbol (a `class_definition`).
const COMPONENT_B_FILE: &str = "services/limiter.py";
const COMPONENT_B_SYMBOL: &str = "TokenLimiter";

/// Component A's TypeScript source carrying its present symbol (resolves clean).
fn component_a_present() -> &'static str {
    "export class RateRouter {\n  route(): number {\n    return 0;\n  }\n}\n"
}

/// Component A's TypeScript source with the symbol **renamed away** — the file stays, so
/// this is a symbol-absent block, not a file-absent one. B's source is untouched.
fn component_a_deleted() -> &'static str {
    "export class RouterRenamed {\n  route(): number {\n    return 0;\n  }\n}\n"
}

/// Component B's Python source carrying its present symbol (resolves clean).
fn component_b_present() -> &'static str {
    "class TokenLimiter:\n    def allow(self) -> bool:\n        return True\n"
}

/// Component B's Python source with the symbol **renamed away** — the file stays. A's
/// source is untouched.
fn component_b_deleted() -> &'static str {
    "class LimiterRenamed:\n    def allow(self) -> bool:\n        return True\n"
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

/// HEAD commit count — the no-commit witness the blocking walks assert is unchanged.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"]).parse().unwrap()
}

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer, plus the
/// **polyglot** component source files — a TypeScript file carrying A's symbol and a Python
/// file carrying B's symbol.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::create_dir_all(repo.join("src")).expect("mk src");
    fs::create_dir_all(repo.join("services")).expect("mk services");
    fs::write(repo.join(COMPONENT_A_FILE), component_a_present()).expect("write component A (TS)");
    fs::write(repo.join(COMPONENT_B_FILE), component_b_present()).expect("write component B (Py)");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and **no `JIGC_DOC_CODE_PROBE`
/// override** — so the probe resolves through the **production default** path
/// (`<jigc-bin-dir>/doc-code`, a sibling of the running binary). `env_remove` guards against
/// an env var leaking in from the test runner.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE")
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output (no probe override).
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_DOC_CODE_PROBE");
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
/// finalize renders a clean git message and the only pass/block levers are the anchors.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), "arch-doc");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"document the gateway\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Living architecture doc for the gateway.\n",
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
/// address (the emitted bytes are the contract), filling its description and anchoring its
/// `implemented-by` at `anchor`.
fn add_component(
    repo: &Path,
    home: &Path,
    title: &str,
    expect_addr: &str,
    description: &[u8],
    anchor: &str,
) {
    let added = jigc_doc(
        repo,
        home,
        &["add-item", "arch-doc:gateway#components", "--title", title],
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
}

/// Author the arch-doc through the binary: start the workflow, create the doc, author the
/// overview, cite the committed adr, and add **two** components — A anchored at A's
/// TypeScript symbol, B at B's Python symbol. Returns the task id. Every step drives the
/// real binary.
fn author_two_component_arch_doc(repo: &Path, home: &Path) -> &'static str {
    let task = "document-the-gateway";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "architecture-documentation",
            "document the gateway",
        ],
    );
    assert_ok(&out, "`jigc start --workflow architecture-documentation`");

    let create = jigc_doc(
        repo,
        home,
        &["create", "arch-doc", "--title", "Gateway"],
        None,
    );
    assert_ok(&create, "`jigc doc create arch-doc`");
    let arch = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(arch, "arch-doc:gateway", "minted arch-doc address");

    set_slot(
        repo,
        home,
        "arch-doc:gateway#overview",
        b"The gateway routes requests and limits per-client volume.\n",
    );
    // The doc-level n→n `cites` header ref → the committed adr.
    set_field(repo, home, "arch-doc:gateway#cites", "adr:use-a-cache");

    // Component A — anchored at A's present TypeScript symbol.
    add_component(
        repo,
        home,
        "Edge router",
        "arch-doc:gateway#components/edge-router",
        b"Routes requests at the edge.\n",
        &format!("{COMPONENT_A_FILE}#{COMPONENT_A_SYMBOL}"),
    );
    // Component B — anchored at B's present Python symbol.
    add_component(
        repo,
        home,
        "Token limiter",
        "arch-doc:gateway#components/token-limiter",
        b"Limits per-client token volume.\n",
        &format!("{COMPONENT_B_FILE}#{COMPONENT_B_SYMBOL}"),
    );

    fill_commit(repo, home, task);
    task
}

/// Component A's item-anchor address — the TypeScript-anchored component.
const A_ANCHOR_ADDR: &str = "arch-doc:gateway#components/edge-router/implemented-by";
/// Component B's item-anchor address — the Python-anchored component.
const B_ANCHOR_ADDR: &str = "arch-doc:gateway#components/token-limiter/implemented-by";

/// Setup — create + finalize `adr:use-a-cache`, the committed decision the arch-doc cites.
/// Authored through the binary (no probe override). Asserts the ADR lands at its canonical
/// `docs/decisions/` path.
fn commit_cited_adr(repo: &Path, home: &Path) {
    let task = "decide-the-cache-strategy";
    let out = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "single-task",
            "decide the cache strategy",
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

/// Run the JSON-format `task finalize` and return the combined stdout+stderr (the
/// report-with-location surface — `location.address` carries the item address the per-item
/// disambiguation proof asserts on).
fn finalize_json(repo: &Path, home: &Path, task: &str) -> (std::process::Output, String) {
    let out = jigc(repo, home, &["--format", "json", "task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    (out, rendered)
}

/// BLOCK A (TypeScript): delete component A's TS symbol while B's Python symbol stays valid →
/// `finalize` blocks on `doc-code.symbol-exists` naming **A's** item address (and NEVER B's —
/// per-item disambiguation across two grammars); HEAD unchanged, nothing promoted.
#[test]
fn flow29_finalize_blocks_naming_only_the_typescript_component() {
    let repo = TempDir::new("block-ts-repo");
    let home = TempDir::new("block-ts-home");
    init_repo(repo.path());
    commit_cited_adr(repo.path(), home.path());

    let task = author_two_component_arch_doc(repo.path(), home.path());

    // Delete component A's TypeScript symbol (rename it away); B's Python file is untouched.
    fs::write(repo.path().join(COMPONENT_A_FILE), component_a_deleted())
        .expect("delete component A's TS symbol");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees A's symbol gone.
    git(repo.path(), &["add", COMPONENT_A_FILE]);

    let before = head_count(repo.path());
    let (out, rendered) = finalize_json(repo.path(), home.path(), task);

    assert!(
        !out.status.success(),
        "a deleted TypeScript symbol must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces doc-code.symbol-exists; got:\n{rendered}",
    );
    assert!(
        rendered.contains(A_ANCHOR_ADDR),
        "the symbol-exists block names component A's (TS) item address; got:\n{rendered}",
    );
    assert!(
        rendered.contains(&format!("{COMPONENT_A_FILE}#{COMPONENT_A_SYMBOL}")),
        "the block names the dangling TypeScript anchor target; got:\n{rendered}",
    );
    // Per-item disambiguation across grammars: B (Python) stays valid → its address never
    // appears (B's anchor resolved through the Python grammar, not a clobbered shared value).
    assert!(
        !rendered.contains(B_ANCHOR_ADDR),
        "component B (Python) stays valid — its item address must NOT appear; got:\n{rendered}",
    );

    let after = head_count(repo.path());
    assert_eq!(before, after, "a blocked finalize creates no commit");
    assert!(
        !repo.path().join("architecture").join("gateway.md").exists(),
        "a blocked finalize promotes nothing",
    );
}

/// BLOCK B (Python): symmetrically delete component B's Python symbol while A's TypeScript
/// symbol stays valid → `finalize` blocks on `doc-code.symbol-exists` naming **B's** item
/// address (and NEVER A's). The symmetric proof that the probe dispatched each anchor to its
/// own language.
#[test]
fn flow29_finalize_blocks_naming_only_the_python_component() {
    let repo = TempDir::new("block-py-repo");
    let home = TempDir::new("block-py-home");
    init_repo(repo.path());
    commit_cited_adr(repo.path(), home.path());

    let task = author_two_component_arch_doc(repo.path(), home.path());

    // Delete component B's Python symbol (rename it away); A's TypeScript file is untouched.
    fs::write(repo.path().join(COMPONENT_B_FILE), component_b_deleted())
        .expect("delete component B's Python symbol");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees B's symbol gone.
    git(repo.path(), &["add", COMPONENT_B_FILE]);

    let before = head_count(repo.path());
    let (out, rendered) = finalize_json(repo.path(), home.path(), task);

    assert!(
        !out.status.success(),
        "a deleted Python symbol must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces doc-code.symbol-exists; got:\n{rendered}",
    );
    assert!(
        rendered.contains(B_ANCHOR_ADDR),
        "the symbol-exists block names component B's (Python) item address; got:\n{rendered}",
    );
    assert!(
        rendered.contains(&format!("{COMPONENT_B_FILE}#{COMPONENT_B_SYMBOL}")),
        "the block names the dangling Python anchor target; got:\n{rendered}",
    );
    // Per-item disambiguation across grammars: A (TypeScript) stays valid → its address
    // never appears.
    assert!(
        !rendered.contains(A_ANCHOR_ADDR),
        "component A (TypeScript) stays valid — its item address must NOT appear; got:\n{rendered}",
    );

    let after = head_count(repo.path());
    assert_eq!(before, after, "a blocked finalize creates no commit");
}

/// PASS: both the TypeScript and the Python symbol present → `finalize` validates clean,
/// lands **exactly one** `docs(arch-doc):` commit, promotes the doc to
/// `architecture/gateway.md`, and carries **no** `doc-code` block. The masking guard is the
/// pass↔block contrast over this same fixture (the two BLOCK tests above): the probe really
/// ran the TS grammar over `src/api.ts` and the Python grammar over `services/limiter.py`
/// and both symbols resolved — a silently-skipped enumeration would land the commit in
/// every arm.
#[test]
fn flow29_finalize_passes_when_both_polyglot_anchors_resolve() {
    let repo = TempDir::new("pass-repo");
    let home = TempDir::new("pass-home");
    init_repo(repo.path());
    commit_cited_adr(repo.path(), home.path());

    let task = author_two_component_arch_doc(repo.path(), home.path());

    let before = head_count(repo.path());
    // Production sibling-probe resolution (no override). The probe really runs the six-grammar
    // set a normal `cargo build` links in.
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // A missing sibling probe would surface as a floor-locked crash meta-finding — assert it
    // resolved (the production path a real install hits).
    assert!(
        !rendered.contains("pack-probe-integrity"),
        "production resolution must find a runnable `doc-code` sibling; got:\n{rendered}",
    );
    assert_ok(
        &out,
        &format!("finalize must pass when both polyglot anchors resolve; got:\n{rendered}"),
    );
    // Both anchors resolved → no doc-code block (the resolved arm of the masking-guard
    // contrast; the BLOCK tests are the block arms, one per language).
    assert!(
        !rendered.contains("doc-code"),
        "both polyglot anchors resolve, so the report must carry no doc-code block; got:\n{rendered}",
    );

    let after = head_count(repo.path());
    assert_eq!(
        after,
        before + 1,
        "the passing walk lands exactly ONE commit"
    );

    let subject = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert!(
        subject.starts_with("docs(arch-doc):"),
        "the commit subject is `docs(arch-doc):`; got:\n{subject}",
    );

    // The arch-doc promoted to architecture/<slug>.md, carrying both per-component polyglot
    // anchors.
    let committed = git_show(repo.path(), "HEAD:docs/architecture/gateway.md");
    assert!(
        committed.status.success(),
        "the arch-doc must promote + commit at docs/architecture/gateway.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let body = String::from_utf8(committed.stdout).expect("utf-8");
    assert!(
        body.contains(&format!("{COMPONENT_A_FILE}#{COMPONENT_A_SYMBOL}"))
            && body.contains(&format!("{COMPONENT_B_FILE}#{COMPONENT_B_SYMBOL}")),
        "the promoted arch-doc carries both per-component polyglot anchors; got:\n{body}",
    );
    // The working-area staging is gone (promoted, not left behind).
    assert!(
        !repo.path().join(".jigc").join("tasks").join(task).exists(),
        "a finalized task's working area is cleaned up",
    );
}
