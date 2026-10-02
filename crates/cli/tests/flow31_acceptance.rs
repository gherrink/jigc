//! M29 Increment 2 acceptance (T1) — flow 31, the marquee: doc↔code validation turned on
//! for a **YAML** target. M28 (flow 30) cashed in the CSS addressable-unit keystone — the
//! HD1 model that reads an addressable-unit name per node-kind rather than a single `.name`
//! field. M29 proves that keystone **generalizes cheaply**: YAML is the *first language to
//! ride the seam* CSS forced (not build it). A YAML *mapping key* (a docker-compose service
//! key) is not an AST named item in the `path#symbol` sense — it needed its own extractor
//! (`block_mapping_pair` / `flow_pair` → the `key` field, quotes stripped), reusing the
//! per-grammar dispatch CSS introduced with no engine/CLI/pack-schema change. This flow
//! mirrors flow 30 (`flow30_acceptance.rs`) — an `arch-doc` with **two** components, each
//! carrying an independently-resolving `implemented-by` anchor, the per-item-disambiguation
//! A-deleted/B-valid proof — but anchored at real **compose service keys** in a single
//! `compose.yaml`: component A's anchor at `compose.yaml#web`, component B's at
//! `compose.yaml#db`.
//!
//! `design/worked-examples.md` → flow 31; `design/validation.md` → Multi-language
//! resolution; `DECISIONS.md` → 2026-06-20 M29 planning, forks F4/F5/F7;
//! `implementation/roadmap.md` → M29 Increment 2 grouped-scope bullet 1.
//!
//! Three arms over the SAME authored fixture (the per-anchor contrast that IS the masking
//! guard — a resolving `doc-code` check emits no finding, so the only honest witness that
//! the probe ran is the pass↔block flip, per service key):
//!
//! - **BLOCK A** — delete component A's service key (`web`) from the compose file while B's
//!   key (`db`) stays valid → `finalize` blocks on `doc-code.symbol-exists` naming **A's**
//!   item address (`arch-doc:gateway#components/web/implemented-by`) and **never** B's;
//!   HEAD unchanged, nothing promoted.
//! - **BLOCK B** — symmetrically delete component B's service key (`db`) while A's key
//!   (`web`) stays valid → block names **B's** item address and **never** A's.
//! - **PASS** — both keys present → `finalize` validates clean, lands exactly **one**
//!   `docs(arch-doc):` commit, promotes the doc to `architecture/gateway.md`, the working
//!   area is cleaned, and the report carries **no** `doc-code` block.
//!
//! Both anchors cite the **same file** (`compose.yaml`) at different service keys, so a
//! deletion is a rewrite of the compose file that drops one service block and keeps the
//! other — the file always exists, so each block is a *key-absent* block, never a
//! file-absent one. That single shared file is the sharpest per-item-disambiguation
//! fixture: the only lever that picks A from B is the per-key extractor over the same
//! `compose.yaml`. (Component B's item slug is `database` while its anchor symbol is `db`,
//! so the block is proven keyed on the *item address*, not on a name coincidence with the
//! symbol.)
//!
//! The probe is resolved through the **production path** — `jigc` spawning itself as the
//! probe, **no** `JIGC_DOC_CODE_PROBE` override — the path a real install hits, exercising
//! the eight-grammar set the binary links in (the `flow30_acceptance.rs` `env_remove`
//! idiom). The temp repo is a real
//! `git init`; self-cleaning `TempDir`s keep the developer's repo clean.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow31-{tag}-{}-{:?}",
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

/// The single compose file both components anchor into — a YAML *mapping key* is the
/// addressable unit (not an AST named item), the first language to ride the HD1 seam.
const COMPOSE_FILE: &str = "compose.yaml";
/// Component A's compose service key — a `block_mapping_pair` key `web`.
const COMPONENT_A_SYMBOL: &str = "web";
/// Component B's compose service key — a `block_mapping_pair` key `db`.
const COMPONENT_B_SYMBOL: &str = "db";

/// Component A's service block (`web:` under `services:`) — present → its key resolves.
const WEB_SERVICE: &str = "  web:\n    image: nginx\n    ports:\n      - \"8080:80\"\n";
/// Component B's service block (`db:` under `services:`) — present → its key resolves.
const DB_SERVICE: &str =
    "  db:\n    image: postgres\n    environment:\n      POSTGRES_PASSWORD: secret\n";

/// Compose the docker-compose file with whichever service blocks are present. Dropping a
/// block removes its service key (the file stays, the key vanishes) — a key-absent block,
/// never a file-absent one. The two arms below never drop both, so `services:` is never
/// empty.
fn compose(web: bool, db: bool) -> String {
    let mut doc = String::from("services:\n");
    if web {
        doc.push_str(WEB_SERVICE);
    }
    if db {
        doc.push_str(DB_SERVICE);
    }
    doc
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
/// shared `compose.yaml` carrying **both** component service keys (`web` + `db`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join(COMPOSE_FILE), compose(true, true)).expect("write compose.yaml");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and **no `JIGC_DOC_CODE_PROBE`
/// override** — so the probe resolves through the **production default** path (`jigc`
/// spawning itself). `env_remove` guards against an env var leaking in from the test
/// runner.
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
/// overview, cite the committed adr, and add **two** components — A anchored at A's compose
/// service key (`compose.yaml#web`), B at B's service key (`compose.yaml#db`). Returns the
/// task id. Every step drives the real binary.
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
        b"The gateway runs a web tier backed by a database service.\n",
    );
    // The doc-level n→n `cites` header ref → the committed adr.
    set_field(repo, home, "arch-doc:gateway#cites", "adr:use-a-cache");

    // Component A — anchored at A's present compose service key (`web`).
    add_component(
        repo,
        home,
        "Web",
        "arch-doc:gateway#components/web",
        b"The web tier container.\n",
        &format!("{COMPOSE_FILE}#{COMPONENT_A_SYMBOL}"),
    );
    // Component B — anchored at B's present compose service key (`db`). Its item slug
    // (`database`) deliberately differs from its anchor symbol (`db`), so the block proves
    // it is keyed on the item address, not on a name coincidence with the symbol.
    add_component(
        repo,
        home,
        "Database",
        "arch-doc:gateway#components/database",
        b"The database service.\n",
        &format!("{COMPOSE_FILE}#{COMPONENT_B_SYMBOL}"),
    );

    fill_commit(repo, home, task);
    task
}

/// Component A's item-anchor address — the `web`-anchored component.
const A_ANCHOR_ADDR: &str = "arch-doc:gateway#components/web/implemented-by";
/// Component B's item-anchor address — the `db`-anchored component (item slug `database`).
const B_ANCHOR_ADDR: &str = "arch-doc:gateway#components/database/implemented-by";

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

/// BLOCK A: delete component A's service key (`web`) from the compose file while B's key
/// (`db`) stays valid → `finalize` blocks on `doc-code.symbol-exists` naming **A's** item
/// address (and NEVER B's — per-item disambiguation over the *same* `compose.yaml`); HEAD
/// unchanged, nothing promoted.
#[test]
fn flow31_finalize_blocks_naming_only_the_web_component() {
    let repo = TempDir::new("block-web-repo");
    let home = TempDir::new("block-web-home");
    init_repo(repo.path());
    commit_cited_adr(repo.path(), home.path());

    let task = author_two_component_arch_doc(repo.path(), home.path());

    // Delete component A's service key (drop the `web:` block); B's `db:` block stays.
    fs::write(repo.path().join(COMPOSE_FILE), compose(false, true))
        .expect("delete component A's service key");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees A's service key gone.
    git(repo.path(), &["add", COMPOSE_FILE]);

    let before = head_count(repo.path());
    let (out, rendered) = finalize_json(repo.path(), home.path(), task);

    assert!(
        !out.status.success(),
        "a deleted compose service key must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces doc-code.symbol-exists; got:\n{rendered}",
    );
    assert!(
        rendered.contains(A_ANCHOR_ADDR),
        "the symbol-exists block names component A's (web) item address; got:\n{rendered}",
    );
    assert!(
        rendered.contains(&format!("{COMPOSE_FILE}#{COMPONENT_A_SYMBOL}")),
        "the block names the dangling YAML anchor target; got:\n{rendered}",
    );
    // Per-item disambiguation over the same file: B (`db`) stays valid → its address never
    // appears (B's anchor resolved through the YAML extractor over the same compose.yaml, not
    // a clobbered shared value).
    assert!(
        !rendered.contains(B_ANCHOR_ADDR),
        "component B (db) stays valid — its item address must NOT appear; got:\n{rendered}",
    );

    let after = head_count(repo.path());
    assert_eq!(before, after, "a blocked finalize creates no commit");
    assert!(
        !repo.path().join("architecture").join("gateway.md").exists(),
        "a blocked finalize promotes nothing",
    );
}

/// BLOCK B: symmetrically delete component B's service key (`db`) while A's key (`web`) stays
/// valid → `finalize` blocks on `doc-code.symbol-exists` naming **B's** item address (and
/// NEVER A's). The symmetric proof that the extractor picked each anchor's own key out of the
/// shared compose file.
#[test]
fn flow31_finalize_blocks_naming_only_the_database_component() {
    let repo = TempDir::new("block-db-repo");
    let home = TempDir::new("block-db-home");
    init_repo(repo.path());
    commit_cited_adr(repo.path(), home.path());

    let task = author_two_component_arch_doc(repo.path(), home.path());

    // Delete component B's service key (drop the `db:` block); A's `web:` block stays.
    fs::write(repo.path().join(COMPOSE_FILE), compose(true, false))
        .expect("delete component B's service key");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees B's service key gone.
    git(repo.path(), &["add", COMPOSE_FILE]);

    let before = head_count(repo.path());
    let (out, rendered) = finalize_json(repo.path(), home.path(), task);

    assert!(
        !out.status.success(),
        "a deleted compose service key must make finalize exit non-zero; got:\n{rendered}",
    );
    assert!(
        rendered.contains("doc-code.symbol-exists"),
        "the block surfaces doc-code.symbol-exists; got:\n{rendered}",
    );
    assert!(
        rendered.contains(B_ANCHOR_ADDR),
        "the symbol-exists block names component B's (db) item address; got:\n{rendered}",
    );
    assert!(
        rendered.contains(&format!("{COMPOSE_FILE}#{COMPONENT_B_SYMBOL}")),
        "the block names the dangling YAML anchor target; got:\n{rendered}",
    );
    // Per-item disambiguation over the same file: A (`web`) stays valid → its address never
    // appears.
    assert!(
        !rendered.contains(A_ANCHOR_ADDR),
        "component A (web) stays valid — its item address must NOT appear; got:\n{rendered}",
    );

    let after = head_count(repo.path());
    assert_eq!(before, after, "a blocked finalize creates no commit");
}

/// PASS: both service keys present → `finalize` validates clean, lands **exactly one**
/// `docs(arch-doc):` commit, promotes the doc to `architecture/gateway.md`, and carries
/// **no** `doc-code` block. The masking guard is the pass↔block contrast over this same
/// fixture (the two BLOCK tests above): the probe really ran the YAML grammar over
/// `compose.yaml` and both keys resolved — a silently-skipped enumeration would land the
/// commit in every arm.
#[test]
fn flow31_finalize_passes_when_both_yaml_anchors_resolve() {
    let repo = TempDir::new("pass-repo");
    let home = TempDir::new("pass-home");
    init_repo(repo.path());
    commit_cited_adr(repo.path(), home.path());

    let task = author_two_component_arch_doc(repo.path(), home.path());

    let before = head_count(repo.path());
    // Production sibling-probe resolution (no override). The probe really runs the eight-grammar
    // set a normal `cargo build` links in.
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // A probe that could not run would surface as a floor-locked crash meta-finding — assert
    // it ran (the production path a real install hits).
    assert!(
        !rendered.contains("pack-probe-integrity"),
        "the production path must run the `doc-code` probe; got:\n{rendered}",
    );
    assert_ok(
        &out,
        &format!("finalize must pass when both YAML anchors resolve; got:\n{rendered}"),
    );
    // Both anchors resolved → no doc-code block (the resolved arm of the masking-guard
    // contrast; the BLOCK tests are the block arms, one per service key).
    assert!(
        !rendered.contains("doc-code"),
        "both YAML anchors resolve, so the report must carry no doc-code block; got:\n{rendered}",
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

    // The arch-doc promoted to architecture/<slug>.md, carrying both per-component YAML anchors.
    let committed = git_show(repo.path(), "HEAD:docs/architecture/gateway.md");
    assert!(
        committed.status.success(),
        "the arch-doc must promote + commit at docs/architecture/gateway.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );
    let body = String::from_utf8(committed.stdout).expect("utf-8");
    assert!(
        body.contains(&format!("{COMPOSE_FILE}#{COMPONENT_A_SYMBOL}"))
            && body.contains(&format!("{COMPOSE_FILE}#{COMPONENT_B_SYMBOL}")),
        "the promoted arch-doc carries both per-component YAML anchors; got:\n{body}",
    );
    // The working-area staging is gone (promoted, not left behind).
    assert!(
        !repo.path().join(".jigc").join("tasks").join(task).exists(),
        "a finalized task's working area is cleaned up",
    );
}
