//! M40 Increment 2, T2 — the `jigc doc retitle-item <addr> --title "<new>"` CLI verb
//! through the real binary: the retitle-without-reslug invariant's verb at **item**
//! level (`design/write-commands.md` → `jigc doc retitle-item`; VISION → Stable IDs).
//!
//! Two halves, per the task's done-criterion:
//!
//! 1. **Retitle (string id-from)** — a **committed** arch-doc component retitles
//!    through the real verb: the `{#id}` anchor stays byte-identical (only the
//!    heading-title bytes change against the committed file), a follow-up `set-slot`
//!    at the **same** item address lands (inbound addresses intact), and finalize is
//!    green — the retitled doc re-commits clean.
//! 2. **Refuse (enum id-from)** — `retitle-item` on a changelog change-group
//!    (`id-from: category`, an enum) exits **non-zero** with a blocking finding whose
//!    route names `doc remove-item` on the item + `doc add-item` under the target
//!    category — a member change is an identity change, not a retitle (the Settle-
//!    decided REFUSE + remove/add route), and the refusal is **unconditional**: it
//!    fires even when the new title IS an enum member. Both the top-level
//!    (`#unreleased-changes/<cat>`) and nested (`#releases/<v>/changes/<cat>`) guard
//!    arms are driven, and the staged file stays byte-unchanged.
//!
//! Mirrors `arch_doc_acceptance.rs` (the shipped embedded pack, a real `git init`
//! repo, the test-built `doc-code` probe for the arch-doc finalize) and
//! `doc_remove_item.rs` (the shared item-address forms).
//!
//! T3 adds the **set-field id-from guard** tests (`design/write-commands.md` → The
//! set-field id-from guard): `doc set-field` on a heading-derived (`id-from`) field
//! rejects with a **type-aware route** — string id-from names `doc retitle-item`,
//! enum id-from names `remove-item` + `add-item` — and moves no bytes (killing the
//! or-insert corruption shapes). A non-id-from item field stays writable.

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
            "jigc-retitle-item-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the arch-doc finalize resolves `implemented-by` anchors through it (mirrors
/// `arch_doc_acceptance::doc_code_probe`).
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
/// against.
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the doc-code probe selected,
/// optionally piping `stdin`, capturing output. The embedded shipped pack (no
/// `JIGC_PACK_DIR`) — the verb is proven against exactly the doctypes that ship.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
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

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure, returning
/// trimmed stdout.
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

/// Set a doc slot from piped stdin, asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    let out = jigc(
        repo,
        home,
        &["doc", "set-slot", addr, "--from-file", "-"],
        Some(prose),
    );
    ok_stdout(out, &format!("set-slot {addr}"));
}

/// Set a doc field, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    let out = jigc(
        repo,
        home,
        &["doc", "set-field", addr, "--value", value],
        None,
    );
    ok_stdout(out, &format!("set-field {addr}"));
}

/// Fill the commit doc bound to `task` with a conventional `docs(<scope>):` shape so
/// finalize renders a clean git message.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str, summary: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        format!("{summary}\n").as_bytes(),
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Driven by the retitle-item integration suite.\n",
    );
}

/// The staged copy of `addr` (`<type>:<slug>`) in the named task's working area.
fn staged_doc(repo: &Path, task: &str, addr: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("{addr}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// Author + finalize `arch-doc:cache-layer` with ONE component (`Session store`,
/// anchored at the present symbol) — the committed doc half (1) retitles.
fn commit_arch_doc(repo: &Path, home: &Path) {
    let task = "document-the-cache-layer";
    ok_stdout(
        jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "architecture-documentation",
                "document the cache layer",
            ],
            None,
        ),
        "jigc start --workflow architecture-documentation",
    );

    let created = ok_stdout(
        jigc(
            repo,
            home,
            &["doc", "create", "arch-doc", "--title", "Cache layer"],
            None,
        ),
        "jigc doc create arch-doc",
    );
    assert_eq!(created, "arch-doc:cache-layer", "minted arch-doc address");

    set_slot(
        repo,
        home,
        "arch-doc:cache-layer#overview",
        b"The cache layer owns ephemeral session state.\n",
    );

    // One component; `add-item` emits the item address every later step runs verbatim.
    let added = ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                "arch-doc:cache-layer#components",
                "--title",
                "Session store",
            ],
            None,
        ),
        "jigc doc add-item …#components",
    );
    assert_eq!(
        added, "arch-doc:cache-layer#components/session-store",
        "the emitted item address"
    );
    set_slot(
        repo,
        home,
        &format!("{added}/description"),
        b"Holds session blobs keyed by token.\n",
    );
    set_field(
        repo,
        home,
        &format!("{added}/implemented-by"),
        "src/lib.rs#present_symbol",
    );

    fill_commit(repo, home, task, "arch-doc", "document the cache layer");
    let out = jigc(repo, home, &["task", "finalize", task], None);
    ok_stdout(out, "jigc task finalize (arch-doc setup)");
}

/// Half (1) — a **committed** arch-doc component retitles through the real verb: the
/// `{#id}` anchor is byte-identical (only the heading-title bytes change against the
/// committed file), a follow-up `set-slot` at the SAME item address lands, and
/// finalize is green — the retitled doc re-commits with its inbound addresses intact.
#[test]
fn committed_arch_doc_component_retitles_with_the_anchor_frozen() {
    let repo = TempDir::new("retitle-repo");
    let home = TempDir::new("retitle-home");
    init_repo(repo.path());
    commit_arch_doc(repo.path(), home.path());

    let committed_path = repo
        .path()
        .join("docs")
        .join("architecture")
        .join("cache-layer.md");
    let committed = fs::read_to_string(&committed_path).expect("read the committed arch-doc");
    let old_heading = "### Session store  {#session-store}";
    assert!(
        committed.contains(old_heading),
        "the committed doc carries the canonical component heading; got:\n{committed}",
    );

    // A second task retitles the committed component through the real verb.
    let task = "retitle-the-store";
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "retitle the store"],
            None,
        ),
        "jigc start (retitle task)",
    );

    let addr = "arch-doc:cache-layer#components/session-store";
    let stdout = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["doc", "retitle-item", addr, "--title", "Session vault"],
            None,
        ),
        "jigc doc retitle-item",
    );
    assert!(
        stdout.contains(addr),
        "retitle-item confirms the item address on stdout; got:\n{stdout}",
    );

    // The staged copy differs from the committed bytes in EXACTLY the heading-title
    // bytes: the `{#session-store}` anchor (and every other byte) is frozen.
    let staged = staged_doc(repo.path(), task, "arch-doc:cache-layer");
    let expected = committed.replace(old_heading, "### Session vault  {#session-store}");
    assert_ne!(staged, committed, "the retitle must change bytes");
    assert_eq!(
        staged, expected,
        "only the heading-title bytes change; the {{#id}} anchor is byte-identical",
    );

    // A follow-up write at the SAME item address lands — the id (hence every inbound
    // address) survived the retitle.
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}/description"),
        b"Holds session blobs and refresh tokens keyed by token.\n",
    );

    // Finalize is green: the retitled doc re-commits clean.
    fill_commit(
        repo.path(),
        home.path(),
        task,
        "arch-doc",
        "retitle the session store component",
    );
    let out = jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    ok_stdout(out, "jigc task finalize (retitle task)");

    let recommitted = fs::read_to_string(&committed_path).expect("read the re-committed arch-doc");
    assert!(
        recommitted.contains("### Session vault  {#session-store}"),
        "the re-committed doc carries the new title over the FROZEN anchor; got:\n{recommitted}",
    );
    assert!(
        recommitted.contains("refresh tokens"),
        "the follow-up set-slot at the old item address landed; got:\n{recommitted}",
    );
}

/// The anchor-injection reject (M40 blocking finding): a `--title` carrying the `{#`
/// anchor pattern must exit **non-zero** with a blocking finding and move **no
/// bytes** — spliced, `### Evil {#other-anchor} title  {#session-store}` would be
/// re-read with `{#other-anchor}` as the item's identity, silently reslug-hijacking
/// the frozen anchor with exit 0 (every inbound `#components/session-store` address
/// dead, the hijacked id live, the corruption committed clean at finalize). After the
/// reject, the frozen address must still be LIVE (a clean retitle at the same address
/// lands) — the exact evidence sequence, run through the real binary. `add-item`'s
/// mint path takes the same reject.
#[test]
fn anchor_syntax_title_is_refused_and_the_frozen_address_stays_live() {
    let repo = TempDir::new("inject-repo");
    let home = TempDir::new("inject-home");
    init_repo(repo.path());
    commit_arch_doc(repo.path(), home.path());

    let task = "retitle-the-store";
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "retitle the store"],
            None,
        ),
        "jigc start (inject task)",
    );

    let addr = "arch-doc:cache-layer#components/session-store";
    let staged_path = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("arch-doc:cache-layer.md");

    // (1) The injection attempt refuses: non-zero, blocking, no bytes moved (the
    // reject stages nothing — the committed doc is the only copy).
    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "retitle-item",
            addr,
            "--title",
            "Evil {#other-anchor} title",
        ],
        None,
    );
    assert!(
        !refused.status.success(),
        "retitle-item with an anchor-syntax title must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&refused.stdout),
    );
    let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        stderr.contains("blocking"),
        "the reject is a blocking finding; stderr:\n{stderr}",
    );
    if staged_path.is_file() {
        let committed = fs::read_to_string(
            repo.path()
                .join("docs")
                .join("architecture")
                .join("cache-layer.md"),
        )
        .expect("read the committed arch-doc");
        assert_eq!(
            staged_doc(repo.path(), task, "arch-doc:cache-layer"),
            committed,
            "the refused retitle moved no bytes against the committed doc",
        );
    }

    // (2) The frozen address is still live: a clean retitle at the SAME address lands
    // (in the broken build this exits 1 `item not present` — the anchor was hijacked).
    let stdout = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["doc", "retitle-item", addr, "--title", "Clean title"],
            None,
        ),
        "jigc doc retitle-item (clean, after the reject)",
    );
    assert!(
        stdout.contains(addr),
        "the frozen address still resolves; got:\n{stdout}",
    );
    assert!(
        staged_doc(repo.path(), task, "arch-doc:cache-layer")
            .contains("### Clean title  {#session-store}"),
        "the clean retitle landed over the FROZEN anchor",
    );

    // (3) `add-item` refuses the same input class at mint time.
    let refused_add = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "add-item",
            "arch-doc:cache-layer#components",
            "--title",
            "Evil {#injected} component",
        ],
        None,
    );
    assert!(
        !refused_add.status.success(),
        "add-item with an anchor-syntax title must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&refused_add.stdout),
    );
    assert!(
        !staged_doc(repo.path(), task, "arch-doc:cache-layer").contains("{#injected}"),
        "no injected anchor was minted",
    );
}

/// Half (2) — the **unconditional** enum-id-from refusal: `retitle-item` on a
/// changelog change-group (`id-from: category`, an enum) exits non-zero with a
/// blocking finding routing to `doc remove-item` + `doc add-item` under the target
/// category — even though the new title IS a legal enum member (a member change is an
/// identity change, not a retitle). Both guard arms are driven — the top-level
/// `#unreleased-changes/<cat>` form and the nested `#releases/<v>/changes/<cat>`
/// chain — and the staged file stays byte-unchanged.
#[test]
fn changelog_change_group_retitle_refuses_with_the_remove_add_route() {
    let repo = TempDir::new("refuse-repo");
    let home = TempDir::new("refuse-home");
    init_repo(repo.path());

    let task = "record-the-change";
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "record-change", "record the change"],
            None,
        ),
        "jigc start --workflow record-change",
    );
    let created = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    assert_eq!(created, "changelog:changelog", "the singleton address");

    // A top-level staged change-group + a nested per-release one — both enum-id-from.
    let staged_group = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item staged change-group",
    );
    assert_eq!(staged_group, "changelog:changelog#unreleased-changes/added");
    let release = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1-0-0",
            ],
            None,
        ),
        "add-item release",
    );
    let nested_group = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Fixed",
            ],
            None,
        ),
        "add-item nested change-group",
    );
    assert_eq!(
        nested_group,
        "changelog:changelog#releases/1-0-0/changes/fixed"
    );

    let before = staged_doc(repo.path(), task, "changelog:changelog");

    // (a) top-level: `Added` → `Changed` is member→member and STILL refuses — the
    // refusal is unconditional on an enum id-from, with the remove+add route.
    let refused = jigc(
        repo.path(),
        home.path(),
        &["doc", "retitle-item", &staged_group, "--title", "Changed"],
        None,
    );
    assert!(
        !refused.status.success(),
        "retitle-item on an enum-id-from item must exit non-zero",
    );
    let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        stderr.contains("blocking"),
        "the refusal is a blocking finding; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("jigc doc remove-item {staged_group}")),
        "the route names `doc remove-item` on the item; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(
            "jigc doc add-item changelog:changelog#unreleased-changes --title \"Changed\""
        ),
        "the route names `doc add-item` under the target category; stderr:\n{stderr}",
    );

    // (b) nested chain: the same unconditional refusal through the nested guard arm,
    // routing the add-item at the release's own `changes` repeatable.
    let refused_nested = jigc(
        repo.path(),
        home.path(),
        &["doc", "retitle-item", &nested_group, "--title", "Security"],
        None,
    );
    assert!(
        !refused_nested.status.success(),
        "retitle-item on a nested enum-id-from item must exit non-zero",
    );
    let stderr_nested = String::from_utf8_lossy(&refused_nested.stderr).to_string();
    assert!(
        stderr_nested.contains(&format!("jigc doc remove-item {nested_group}")),
        "the nested route names `doc remove-item` on the item; stderr:\n{stderr_nested}",
    );
    assert!(
        stderr_nested.contains(
            "jigc doc add-item changelog:changelog#releases/1-0-0/changes --title \"Security\""
        ),
        "the nested route names `doc add-item` under the target category; stderr:\n{stderr_nested}",
    );

    // The refusals moved no bytes.
    assert_eq!(
        staged_doc(repo.path(), task, "changelog:changelog"),
        before,
        "the refused retitles left the staged changelog byte-identical",
    );
}

/// T3 (1)+(3) — the set-field id-from guard, **string** arm: `doc set-field` on
/// `…#components/<id>/title` (arch-doc's `id-from: title`, a string field) exits
/// non-zero with a blocking finding whose route names `jigc doc retitle-item
/// <item-addr>` — and the staged buffer is byte-unchanged (no or-inserted `title`
/// bullet, the corruption shape dead). A **non-id-from** item field on the same item
/// still sets fine — the guard is scoped to the heading-derived field, inert
/// everywhere else.
#[test]
fn set_field_on_a_string_id_from_field_rejects_with_the_retitle_route() {
    let repo = TempDir::new("guard-string-repo");
    let home = TempDir::new("guard-string-home");
    init_repo(repo.path());

    let task = "document-the-cache-layer";
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "start",
                "--workflow",
                "architecture-documentation",
                "document the cache layer",
            ],
            None,
        ),
        "jigc start --workflow architecture-documentation",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["doc", "create", "arch-doc", "--title", "Cache layer"],
            None,
        ),
        "jigc doc create arch-doc",
    );
    let item = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "arch-doc:cache-layer#components",
                "--title",
                "Session store",
            ],
            None,
        ),
        "jigc doc add-item …#components",
    );
    assert_eq!(item, "arch-doc:cache-layer#components/session-store");

    let before = staged_doc(repo.path(), task, "arch-doc:cache-layer");

    // (1) the heading-derived `title` field rejects with the retitle-item route.
    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{item}/title"),
            "--value",
            "Session vault",
        ],
        None,
    );
    assert!(
        !refused.status.success(),
        "set-field on a string id-from field must exit non-zero",
    );
    let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        stderr.contains("blocking"),
        "the guard is a blocking finding; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!(
            "jigc doc retitle-item {item} --title \"Session vault\""
        )),
        "the route names `doc retitle-item` at the item address; stderr:\n{stderr}",
    );

    // No or-inserted `title` bullet: the staged buffer is byte-unchanged.
    assert_eq!(
        staged_doc(repo.path(), task, "arch-doc:cache-layer"),
        before,
        "the refused set-field left the staged arch-doc byte-identical",
    );

    // (3) a non-id-from item field on the SAME item still sets fine — the guard is
    // inert off the heading-derived field.
    set_field(
        repo.path(),
        home.path(),
        &format!("{item}/implemented-by"),
        "src/lib.rs#present_symbol",
    );
    assert!(
        staged_doc(repo.path(), task, "arch-doc:cache-layer").contains("src/lib.rs#present_symbol"),
        "the non-id-from item field landed",
    );
}

/// T3 (2) — the set-field id-from guard, **enum** arm: `doc set-field` on a
/// change-group's `category` (`id-from: category`, an enum) exits non-zero with the
/// remove+add route — a category change is an identity change, so the route names
/// `doc remove-item` on the item + `doc add-item` under the target category. Both
/// guard arms are driven — the nested chain (`#releases/<v>/changes/<cat>/category`,
/// the done-criterion form) and the top-level item (`#unreleased-changes/<cat>/
/// category`) — and the staged file stays byte-unchanged (even for a member value:
/// the reject is about identity, not value conformance).
#[test]
fn set_field_on_an_enum_id_from_field_rejects_with_the_remove_add_route() {
    let repo = TempDir::new("guard-enum-repo");
    let home = TempDir::new("guard-enum-home");
    init_repo(repo.path());

    let task = "record-the-change";
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "record-change", "record the change"],
            None,
        ),
        "jigc start --workflow record-change",
    );
    ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    let staged_group = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item staged change-group",
    );
    let release = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1-0-0",
            ],
            None,
        ),
        "add-item release",
    );
    let nested_group = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Fixed",
            ],
            None,
        ),
        "add-item nested change-group",
    );
    assert_eq!(
        nested_group,
        "changelog:changelog#releases/1-0-0/changes/fixed"
    );

    let before = staged_doc(repo.path(), task, "changelog:changelog");

    // (a) the nested chain — the done-criterion form.
    let refused = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{nested_group}/category"),
            "--value",
            "security",
        ],
        None,
    );
    assert!(
        !refused.status.success(),
        "set-field on a nested enum id-from field must exit non-zero",
    );
    let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
    assert!(
        stderr.contains(&format!("jigc doc remove-item {nested_group}")),
        "the route names `doc remove-item` on the item; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(
            "jigc doc add-item changelog:changelog#releases/1-0-0/changes --title \"security\""
        ),
        "the route names `doc add-item` under the target category; stderr:\n{stderr}",
    );

    // (b) the top-level item arm — the same guard through the `#section/<item>/<field>`
    // form.
    let refused_top = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "set-field",
            &format!("{staged_group}/category"),
            "--value",
            "changed",
        ],
        None,
    );
    assert!(
        !refused_top.status.success(),
        "set-field on a top-level enum id-from field must exit non-zero",
    );
    let stderr_top = String::from_utf8_lossy(&refused_top.stderr).to_string();
    assert!(
        stderr_top.contains(&format!("jigc doc remove-item {staged_group}")),
        "the top-level route names `doc remove-item` on the item; stderr:\n{stderr_top}",
    );
    assert!(
        stderr_top.contains(
            "jigc doc add-item changelog:changelog#unreleased-changes --title \"changed\""
        ),
        "the top-level route names `doc add-item` under the target category; stderr:\n{stderr_top}",
    );

    // The refusals moved no bytes — no contradictory `category` bullet or-inserted.
    assert_eq!(
        staged_doc(repo.path(), task, "changelog:changelog"),
        before,
        "the refused set-fields left the staged changelog byte-identical",
    );
}
