//! (M42 inc-9 T2) **Every blocking `write.*` finding keys at the doc URI it names.**
//!
//! `design/command-output-contract.md` → the stable finding key / the `write.*` row: the
//! contract *claims* `(code, target)` is unique-per-instance, and the address-bearing
//! writes broke the claim — the engine's write constructors set **no** `Location::address`
//! (they hold a section + a field, never a `type:slug`), so every failed `set-field` in the
//! corpus collided on the single degenerate key `(write.malformed-value, null)`. The CLI is
//! the layer that holds the parsed [`engine::address::Address`], so the CLI stamps it.
//!
//! Driven through the **real binary** over throwaway repos: the emitted `--format json`
//! block envelope is the contract, so the assertions read the emitted bytes — never a
//! reconstructed finding.
//!
//! The bare-singleton arm is the hazard M42 inc-8 introduced: a singleton head is legal at
//! the verb boundary (`vision#thesis`), so a target stamped from the **raw CLI argument**
//! would key `vision#thesis` — not the URI normal form `vision:vision#thesis`. The target
//! is stamped from the **parsed** address.

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
            "jigc-write-keys-{tag}-{}-{:?}",
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

/// The methodology pack's source tree (the `vision` singleton's home).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
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

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A git repo with one commit, the `.jigc/config/` project layer (naming the methodology
/// pack, so the `vision` singleton composes alongside the dev pack), and one started task.
fn started_repo(workflow: &str, intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.path().join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", workflow, intent],
            None,
        ),
        &format!("`jigc start --workflow {workflow}`"),
    );
    (repo, home)
}

/// Run a **blocking** `jigc doc <args> --format json` write and return the one finding its
/// emitted block envelope carries — the bytes a driver deserializes.
fn blocked(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> serde_json::Value {
    let mut full = vec!["doc"];
    full.extend_from_slice(args);
    full.extend_from_slice(&["--format", "json"]);
    let out = jigc(repo, home, &full, stdin);
    assert!(
        !out.status.success(),
        "`jigc {}` must block (non-zero exit); stdout:\n{}",
        full.join(" "),
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    let envelope: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("the block envelope is json ({e}); got:\n{stderr}"));
    let findings = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("findings is an array; got:\n{stderr}"))
        .clone();
    assert_eq!(
        findings.len(),
        1,
        "a blocked write emits exactly its one finding; got:\n{stderr}"
    );
    findings.into_iter().next().expect("the one finding")
}

/// The finding's stable `key.target` — the value a driver dedupes on.
fn target(finding: &serde_json::Value) -> &str {
    finding["key"]["target"]
        .as_str()
        .unwrap_or_else(|| panic!("key.target is a string, not null; got:\n{finding:#}"))
}

/// The **doc-scoped** write verbs — `set-field` (a malformed enum value) and `set-slot` (a
/// schema-reserved heading depth in the prose) — key at the URI of the node they name, at
/// the depth they name it. And two failed writes of the **same code** to **two different
/// docs** no longer share one key (the degenerate `(code, null)` the contract forbids).
#[test]
fn the_doc_scoped_write_verbs_key_at_the_uri_they_name() {
    let (repo, home) = started_repo("single-task", "add rate limiter");
    for title in ["Cache strategy", "Retry policy"] {
        assert_ok(
            &jigc(
                repo.path(),
                home.path(),
                &["doc", "create", "adr", "--title", title],
                None,
            ),
            &format!("`jigc doc create adr --title {title:?}`"),
        );
    }

    // `set-field` — a value outside the enum's members blocks at the field it named.
    let cache_field = blocked(
        repo.path(),
        home.path(),
        &[
            "set-field",
            "adr:cache-strategy#status/status",
            "--value",
            "notastatus",
        ],
        None,
    );
    assert_eq!(
        target(&cache_field),
        "adr:cache-strategy#status/status",
        "a blocked `set-field` keys at the field's URI; got:\n{cache_field:#}"
    );

    // `set-slot` — a schema-reserved `##` depth in the prose blocks at the slot it named.
    let cache_slot = blocked(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            "adr:cache-strategy#decision",
            "--from-file",
            "-",
        ],
        Some(b"## A shadowing heading\n\nprose.\n"),
    );
    assert_eq!(
        target(&cache_slot),
        "adr:cache-strategy#decision",
        "a blocked `set-slot` keys at the slot's URI; got:\n{cache_slot:#}"
    );

    // The collision the key exists to prevent: the SAME code, two docs, two keys.
    let retry_field = blocked(
        repo.path(),
        home.path(),
        &[
            "set-field",
            "adr:retry-policy#status/status",
            "--value",
            "notastatus",
        ],
        None,
    );
    assert_eq!(
        target(&retry_field),
        "adr:retry-policy#status/status",
        "the sibling doc's blocked `set-field` keys at its own URI; got:\n{retry_field:#}"
    );
    assert_eq!(
        cache_field["code"], retry_field["code"],
        "the two blocks are the same KIND of finding — the code cannot discriminate them"
    );
    assert_ne!(
        cache_field["key"], retry_field["key"],
        "two failed writes to two docs must not share one key"
    );
}

/// The **item** verbs — `add-item` (an unslugable title), `remove-item` / `retitle-item` (an
/// item that is not present) — key at the URI they name; and the `set-field` **id-from
/// guard** keys at the FULL URI, not the bare `<section>/<item>/<field>` fragment it emitted
/// (the same defect one step short of normalized).
#[test]
fn the_item_write_verbs_key_at_the_uri_they_name() {
    let (repo, home) = started_repo("plan", "plan the auth flow");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "create", "spec", "--title", "Auth flow"],
            None,
        ),
        "`jigc doc create spec`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "add-item",
                "spec:auth-flow#criteria",
                "--title",
                "It renders fast",
            ],
            None,
        ),
        "`jigc doc add-item spec:auth-flow#criteria`",
    );

    // `add-item` — an unslugable title blocks at the section it was minting into.
    let minted = blocked(
        repo.path(),
        home.path(),
        &["add-item", "spec:auth-flow#criteria", "--title", "!!!"],
        None,
    );
    assert_eq!(
        target(&minted),
        "spec:auth-flow#criteria",
        "a blocked `add-item` keys at the section's URI; got:\n{minted:#}"
    );

    // `remove-item` / `retitle-item` — an absent item blocks at the item's URI.
    let removed = blocked(
        repo.path(),
        home.path(),
        &["remove-item", "spec:auth-flow#criteria/no-such-item"],
        None,
    );
    assert_eq!(
        target(&removed),
        "spec:auth-flow#criteria/no-such-item",
        "a blocked `remove-item` keys at the item's URI; got:\n{removed:#}"
    );
    let retitled = blocked(
        repo.path(),
        home.path(),
        &[
            "retitle-item",
            "spec:auth-flow#criteria/no-such-item",
            "--title",
            "Renamed",
        ],
        None,
    );
    assert_eq!(
        target(&retitled),
        "spec:auth-flow#criteria/no-such-item",
        "a blocked `retitle-item` keys at the item's URI; got:\n{retitled:#}"
    );

    // The id-from guard — the heading-derived `title` field is refused, keyed at the FULL
    // URI (it emitted the bare fragment `criteria/it-renders-fast/title`).
    let id_from = blocked(
        repo.path(),
        home.path(),
        &[
            "set-field",
            "spec:auth-flow#criteria/it-renders-fast/title",
            "--value",
            "It renders faster",
        ],
        None,
    );
    assert_eq!(
        id_from["code"], "write.id-from-field",
        "the heading-derived id-from field is refused; got:\n{id_from:#}"
    );
    assert_eq!(
        target(&id_from),
        "spec:auth-flow#criteria/it-renders-fast/title",
        "the id-from guard keys at the FULL URI, not the bare fragment; got:\n{id_from:#}"
    );
}

/// The **bare-singleton hazard** (M42 inc-8): a placement doctype answers to its bare type
/// id at the verb boundary, so a target stamped from the raw CLI argument would key
/// `vision#thesis`. The target is stamped from the **parsed** address, so the key is the URI
/// normal form `vision:vision#thesis` — the same key the canonical spelling produces.
#[test]
fn a_bare_singleton_write_keys_at_the_uri_normal_form() {
    let (repo, home) = started_repo("form-vision", "revise the vision");
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "create", "vision", "--title", "Vision"],
            None,
        ),
        "`jigc doc create vision`",
    );

    let prose: &[u8] = b"## A shadowing heading\n\nthe thesis.\n";
    let bare = blocked(
        repo.path(),
        home.path(),
        &["set-slot", "vision#thesis", "--from-file", "-"],
        Some(prose),
    );
    assert_eq!(
        target(&bare),
        "vision:vision#thesis",
        "the bare singleton keys at the URI normal form (the PARSED address, never the raw \
         argument); got:\n{bare:#}"
    );

    let canonical = blocked(
        repo.path(),
        home.path(),
        &["set-slot", "vision:vision#thesis", "--from-file", "-"],
        Some(prose),
    );
    assert_eq!(
        bare["key"], canonical["key"],
        "the bare and canonical spellings of one address are ONE finding key"
    );
}
