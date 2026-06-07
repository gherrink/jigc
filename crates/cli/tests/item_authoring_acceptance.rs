//! End-to-end byte-stable item-authoring acceptance (M13 Increment 3 / T3).
//!
//! The composed arc on the **shipped `spec` doctype** (no test-only fixture pack):
//! `jigc start --workflow plan` → `jigc doc create spec` → `jigc doc add-item
//! spec:<slug>#criteria --title …` (×2, two distinct criteria) → per item
//! `set-slot …/statement` + `set-field …/maps-to-test`. This retires the
//! long-deferred `spec.criteria` authoring gap — M3 left criteria un-authorable
//! through the binary (`add-from-spec` only *read* them); the repeatable-item
//! surface (T1 `add-item` + T2 item-leaf addressing) now closes it end-to-end.
//!
//! The load-bearing discipline (increment-workflow.md hardening #4 — the agent
//! runs the CLI's *emitted* address, never a reconstructed one): each
//! `add-item`'s minted item-address line is **captured from stdout and run
//! verbatim** as the addr of the following `set-slot`/`set-field`. A test that
//! rebuilt the address in test code could pass while the emitted bytes an agent
//! would actually run were broken — so the emitted bytes are the contract here.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! temp repo is a real `git init` (`jigc start` reads HEAD), and a self-cleaning
//! `TempDir` keeps the test off the developer's repo.

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
            "jigc-item-auth-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer,
/// then `jigc start --workflow plan "<intent>"` to mint a task (the `plan` workflow
/// `allows-create: [{type: spec, as: spec}]`, the gate item-authoring needs).
/// Returns the repo + a `$HOME` temp dir.
fn plan_started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "plan", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start --workflow plan` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
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

/// The staged `spec:<slug>` instance in the task working area.
fn staged_spec(repo: &Path, task: &str, slug: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("spec:{slug}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The shipped `spec` schema, loaded from the embedded pack source with the
/// dev-pack `code-anchor` type so the byte-stable round-trip asserts against
/// exactly the bytes that ship (mirrors `doc_write.rs`).
fn spec_schema() -> engine::schema::Schema {
    const SPEC_YAML: &[u8] = include_bytes!("../pack/schemas/spec.yaml");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
    }];
    engine::schema::load_schema_with_types(SPEC_YAML, &types).expect("spec.yaml loads")
}

/// Mint a `criteria` item via `jigc doc add-item`, returning the **emitted**
/// item-address line (captured from stdout, to be run verbatim downstream).
fn add_criterion(repo: &Path, home: &Path, slug: &str, title: &str) -> String {
    let out = run_doc(
        repo,
        home,
        &[
            "add-item",
            &format!("spec:{slug}#criteria"),
            "--title",
            title,
        ],
        None,
    );
    assert!(
        out.status.success(),
        "`jigc doc add-item … --title {title:?}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let addr = String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned();
    assert!(
        !addr.is_empty(),
        "`add-item` must emit the minted item address on stdout (the addr the agent runs next)"
    );
    addr
}

/// The full item-authoring arc on the shipped `spec` doctype: two criteria,
/// each filled at its own slot + field through the binary, with every
/// `set-*` addressing the **emitted** `add-item` address verbatim.
#[test]
fn item_authoring_arc_on_spec_is_byte_stable_through_the_binary() {
    let (repo, home) = plan_started_repo("plan the auth flow");
    let task = "plan-the-auth-flow";
    let slug = "auth-flow";

    // Provision the `spec` container via the create-gate (plan allows {type: spec}).
    let created = run_doc(
        repo.path(),
        home.path(),
        &["create", "spec", "--title", "Auth flow"],
        None,
    );
    assert!(
        created.status.success(),
        "`jigc doc create spec` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&created.stderr)
    );

    // Two distinct criteria; each `add-item` *emits* the item address an agent
    // runs next — captured here, then fed verbatim to the `set-*` calls below.
    let arc = [
        (
            "Rate limit holds under burst",
            b"Under a 100-rps burst, no client exceeds its quota.\n".as_slice(),
            "`crates/gateway/src/limiter.rs#enforce`",
        ),
        (
            "Tokens expire after TTL",
            b"A token presented after its TTL is rejected as expired.\n".as_slice(),
            "`crates/auth/src/token.rs#is_expired`",
        ),
    ];

    // Mint both items first, capturing each emitted address. Minting both before
    // filling proves the per-item disambiguation: two items with identically-keyed
    // `statement`/`maps-to-test` leaves coexist, and each fill must land on its own.
    let addrs: Vec<String> = arc
        .iter()
        .map(|(title, _, _)| add_criterion(repo.path(), home.path(), slug, title))
        .collect();

    // The emitted addresses are distinct item addresses under the criteria section
    // (per-item identity through the binary — not the same item twice).
    assert_ne!(
        addrs[0], addrs[1],
        "the two emitted item addresses must differ; got {addrs:?}"
    );
    for addr in &addrs {
        assert!(
            addr.starts_with(&format!("spec:{slug}#criteria/")),
            "each emitted item address is under the criteria section; got {addr:?}"
        );
    }

    // Fill each item's slot + field, addressing the EMITTED address verbatim
    // (hardening #4: the emitted bytes are the contract, run as emitted, never
    // a reconstructed equivalent).
    for (addr, (_, statement, anchor)) in addrs.iter().zip(arc.iter()) {
        let slot = run_doc(
            repo.path(),
            home.path(),
            &["set-slot", &format!("{addr}/statement"), "--from-file", "-"],
            Some(statement),
        );
        assert!(
            slot.status.success(),
            "`set-slot {addr}/statement` (emitted addr) must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&slot.stderr)
        );

        let field = run_doc(
            repo.path(),
            home.path(),
            &[
                "set-field",
                &format!("{addr}/maps-to-test"),
                "--value",
                anchor,
            ],
            None,
        );
        assert!(
            field.status.success(),
            "`set-field {addr}/maps-to-test` (emitted addr) must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&field.stderr)
        );
    }

    let staged = staged_spec(repo.path(), task, slug);
    let schema = spec_schema();
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged spec re-parses");
    let crit = parsed
        .sections
        .iter()
        .find(|s| s.id == "criteria")
        .expect("criteria section present");
    assert_eq!(
        crit.items.len(),
        arc.len(),
        "exactly the two minted criteria are present; got:\n{staged}"
    );

    // Each item's leaves resolve to ITS OWN authored bytes (per-item
    // disambiguation through the binary — the headline T3 claim).
    for (addr, (_, statement, anchor)) in addrs.iter().zip(arc.iter()) {
        let item_id = addr
            .rsplit('/')
            .next()
            .expect("item id is the last hop of the emitted address");
        let item = crit
            .items
            .iter()
            .find(|i| i.id == item_id)
            .unwrap_or_else(|| panic!("item {item_id} present; staged:\n{staged}"));
        let want_statement = std::str::from_utf8(statement).unwrap().trim();
        assert_eq!(
            item.slot.as_deref().unwrap_or("").trim(),
            want_statement,
            "item {item_id}'s `statement` slot is exactly its own authored prose"
        );
        let mapped = item
            .fields
            .iter()
            .find(|f| f.key == "maps-to-test")
            .unwrap_or_else(|| panic!("item {item_id} carries maps-to-test; staged:\n{staged}"));
        match &mapped.value {
            engine::field_block::Value::Scalar(v) => assert_eq!(
                v, *anchor,
                "item {item_id}'s `maps-to-test` field is exactly its own authored anchor"
            ),
            other => panic!("maps-to-test is a scalar anchor, got {other:?}"),
        }
    }

    // The final staged `spec` is canonical: render(parse(staged)) == staged
    // (the byte-stability the deliverable names — the retired #1 risk, proven
    // through the composed arc on shipped pack content).
    let rerendered = engine::write::render(&schema, &parsed);
    assert_eq!(
        rerendered, staged,
        "the item-authored spec is byte-stable across parse → render",
    );
}
