//! M47 Increment 7 / T1 — `jigc task diff` honours `--format json`, and the
//! envelope's `id` is provably the read-back address.
//!
//! `TaskCommand::dispatch` fed `run_diff` no `format` — the sole arm of the six
//! that dropped it — so `jigc --format json task diff <id>` printed markdown, and
//! over a task with an empty working area printed **zero bytes on both streams at
//! exit 0**. This suite pins the settled envelope
//! (`design/command-output-contract.md` §2 → `jigc task diff`; DECISIONS
//! 2026-07-26 M47 Settle, Decision 11; DECISIONS 2026-08-05 the one-key halt):
//!
//! ```json
//! { "op": "task-diff", "task": "<id>", "base": {"sha": …, "short": …},
//!   "code_diff": "<the diff vs base, \"\" when there is none>",
//!   "staged_docs": [{"id": "<type>:<slug>"}], "findings": [] }
//! ```
//!
//! Four arms, all driven against the real binary (`CARGO_BIN_EXE_jigc`):
//!
//! - **(a)** a task carrying a staged doc *and* a working-tree code change — stdout
//!   parses as exactly one document with every key at its settled value, and **no**
//!   `address` / `body` key anywhere.
//! - **(b)** the **followability** proof of the one-key decision — each
//!   `staged_docs[].id` fed **verbatim** to `jigc doc show <id> --task <task-id>`
//!   exits 0 and returns that doc, for a **transient** `commit` doc and for a
//!   **placement singleton** (`changelog:changelog`, the one case where
//!   identity/address divergence was plausible).
//! - **(c)** the **empty state** — a task with no staged docs and no code diff still
//!   emits one JSON document with `code_diff: ""` / `staged_docs: []`, stdout > 0
//!   bytes, exit 0.
//! - **(d)** stderr carries no JSON document on any arm (`design/command-output-contract.md`
//!   → Stream discipline: the JSON-bearing stream parses as exactly one document and
//!   the other stream carries no JSON at all).
//!
//! No external test crates: the binary path comes from Cargo's `CARGO_BIN_EXE_jigc`,
//! the temp repo is a real `git init`, and a self-cleaning `TempDir` keeps the test
//! off the developer's repo.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-task-diff-envelope-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
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

/// Run `git <args>` in `repo`, asserting success.
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

/// A real `git init` repo with one commit and the `.jigc/config/` project layer, plus
/// a throwaway `$HOME`.
fn bare_repo() -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    (repo, home)
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Assert an invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// The task's own recorded base pin, read straight off `.jigc/tasks/<id>/base.json` —
/// the value the envelope's `base` must equal, read from the source rather than
/// re-derived, so a wrong pin cannot agree with a wrong expectation.
fn recorded_base(repo: &Path, task: &str) -> serde_json::Value {
    let raw = fs::read_to_string(
        repo.join(".jigc")
            .join("tasks")
            .join(task)
            .join("base.json"),
    )
    .expect("read the task's base pin");
    serde_json::from_str(&raw).expect("the base pin parses")
}

/// Parse a stream as **exactly one** JSON document — the stream-discipline predicate
/// (`design/command-output-contract.md` → Stream discipline).
fn one_document(stream: &[u8], what: &str) -> serde_json::Value {
    let text = String::from_utf8(stream.to_vec()).expect("utf-8 stream");
    let mut de = serde_json::Deserializer::from_str(&text).into_iter::<serde_json::Value>();
    let first = de
        .next()
        .unwrap_or_else(|| panic!("{what} must carry one JSON document; got:\n{text}"))
        .unwrap_or_else(|err| panic!("{what} must parse as JSON ({err}); got:\n{text}"));
    assert!(
        de.next().is_none(),
        "{what} must carry EXACTLY one JSON document; got:\n{text}"
    );
    first
}

/// Assert a stream carries no JSON document at all — the other half of the stream
/// rule. Empty is the common case; plain diagnostics are permitted, JSON is not.
fn no_document(stream: &[u8], what: &str) {
    let text = String::from_utf8_lossy(stream);
    if text.trim().is_empty() {
        return;
    }
    assert!(
        serde_json::from_str::<serde_json::Value>(&text).is_err(),
        "{what} must carry no JSON document; got:\n{text}"
    );
}

/// Recursively assert no object anywhere in `value` carries `key` — the one-key
/// decision, asserted over the whole document rather than its top level (DECISIONS
/// 2026-08-05: `address` is not a second key, and bodies are refused outright).
fn assert_no_key(value: &serde_json::Value, key: &str, what: &str) {
    match value {
        serde_json::Value::Object(map) => {
            assert!(
                !map.contains_key(key),
                "{what} must carry no `{key}` key; found one in:\n{}",
                serde_json::to_string_pretty(value).expect("re-serialize")
            );
            for nested in map.values() {
                assert_no_key(nested, key, what);
            }
        }
        serde_json::Value::Array(elems) => {
            for nested in elems {
                assert_no_key(nested, key, what);
            }
        }
        _ => {}
    }
}

/// Mint a `single-task` and stage into it: the auto-provisioned transient `commit`
/// doc gets a field written, and a **placement singleton** (`changelog:changelog`,
/// homed at the repo-root `CHANGELOG.md`) is created into the same working area —
/// `single-task` `allows-create` a changelog as `change`.
fn task_with_two_staged_docs(repo: &Path, home: &Path) -> String {
    let task = "add-rate-limiter".to_string();
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "single-task", "add rate limiter"],
        ),
        "`jigc start`",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "feat",
            ],
        ),
        "`jigc doc set-field commit:<task>#type`",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                &task,
            ],
        ),
        "`jigc doc create changelog --task <id>`",
    );
    task
}

/// (a) + (d) — the settled envelope over a task carrying a staged doc *and* a
/// working-tree code change: exactly one JSON document on stdout, every key at its
/// settled value, no `address` / `body` anywhere, no JSON on stderr.
#[test]
fn task_diff_json_carries_the_settled_envelope() {
    let (repo, home) = bare_repo();
    let task = task_with_two_staged_docs(repo.path(), home.path());

    // A working-tree code change, so `code_diff` has something to carry.
    fs::write(repo.path().join("README.md"), "hello\nrate limiter\n").expect("edit the code");

    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "diff", &task],
    );
    assert_ok(&out, "`jigc --format json task diff <id>`");
    let doc = one_document(&out.stdout, "`task diff --format json` stdout");
    no_document(&out.stderr, "`task diff --format json` stderr");

    assert_eq!(doc["op"], "task-diff", "the op names the verb");
    assert_eq!(
        doc["task"],
        task.as_str(),
        "the envelope carries the task id"
    );
    assert_eq!(
        doc["base"],
        recorded_base(repo.path(), &task),
        "`base` is the task's own recorded pin, `{{sha, short}}`"
    );
    let code_diff = doc["code_diff"].as_str().expect("`code_diff` is a string");
    assert!(
        code_diff.contains("+rate limiter"),
        "`code_diff` must carry the changed line; got:\n{code_diff}"
    );
    assert_eq!(
        doc["staged_docs"],
        serde_json::json!([{"id": "changelog:changelog"}, {"id": format!("commit:{task}")}]),
        "`staged_docs` carries the sorted `<type>:<slug>` identities, one key each"
    );
    assert_eq!(
        doc["findings"],
        serde_json::json!([]),
        "`findings` is structurally always empty on this verb"
    );

    assert_no_key(&doc, "address", "the `task diff` envelope");
    assert_no_key(&doc, "body", "the `task diff` envelope");
}

/// (b) — the followability proof of the one-key decision: each `staged_docs[].id`
/// fed **verbatim** to `jigc doc show <id> --task <task-id>` exits 0 and returns
/// that doc. Asserted for the transient `commit` doc and for the placement
/// singleton, the one case where identity/address divergence was plausible.
#[test]
fn each_staged_doc_id_is_the_read_back_address() {
    let (repo, home) = bare_repo();
    let task = task_with_two_staged_docs(repo.path(), home.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "diff", &task],
    );
    assert_ok(&out, "`jigc --format json task diff <id>`");
    let doc = one_document(&out.stdout, "`task diff --format json` stdout");

    let ids: Vec<String> = doc["staged_docs"]
        .as_array()
        .expect("`staged_docs` is an array")
        .iter()
        .map(|entry| {
            entry["id"]
                .as_str()
                .expect("each staged doc carries a string `id`")
                .to_string()
        })
        .collect();
    assert_eq!(
        ids,
        vec!["changelog:changelog".to_string(), format!("commit:{task}")],
        "both the placement singleton and the transient doc are enumerated"
    );

    // The transient `commit` doc renders its own H1; the placement singleton renders
    // the `# Changelog` display title — each read back through the emitted identity,
    // fed verbatim.
    for (id, marker) in [
        ("changelog:changelog", "# Changelog"),
        (&format!("commit:{task}"), &format!("# {task}")),
    ] {
        let shown = jigc(
            repo.path(),
            home.path(),
            &["doc", "show", id, "--task", &task],
        );
        assert_ok(&shown, &format!("`jigc doc show {id} --task {task}`"));
        let text = String::from_utf8(shown.stdout).expect("utf-8 stdout");
        assert!(
            text.contains(marker),
            "`doc show {id} --task {task}` must return that doc (looking for `{marker}`); got:\n{text}"
        );
    }
}

/// (c) + (d) — the empty state is **non-empty**: a task with no staged docs and no
/// code diff still emits exactly one JSON document, `code_diff: ""`,
/// `staged_docs: []`, stdout > 0 bytes, exit 0. At HEAD this printed zero bytes on
/// both streams at exit 0 — silence a driver cannot distinguish from a crash.
#[test]
fn the_empty_state_still_emits_one_document() {
    let (repo, home) = bare_repo();
    // A milestone sub-task area is minted without provisioning (the deferred
    // first-entry provisioner has not run), so its working area stages nothing.
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache hardening"],
        ),
        "`jigc milestone create`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["milestone", "add-task", "cache-hardening", "tune the cache"],
        ),
        "`jigc milestone add-task`",
    );
    let task = "tune-the-cache";

    let out = jigc(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "diff", task],
    );
    assert_ok(&out, "`jigc --format json task diff <empty task>`");
    assert!(
        !out.stdout.is_empty(),
        "the empty state must still write a document to stdout"
    );
    let doc = one_document(
        &out.stdout,
        "`task diff --format json` stdout (empty state)",
    );
    no_document(
        &out.stderr,
        "`task diff --format json` stderr (empty state)",
    );

    assert_eq!(doc["op"], "task-diff");
    assert_eq!(doc["task"], task);
    assert_eq!(
        doc["base"],
        recorded_base(repo.path(), task),
        "`base` is present even with nothing to diff"
    );
    assert_eq!(
        doc["code_diff"], "",
        "`code_diff` is present-always, the empty string when there is no diff"
    );
    assert_eq!(
        doc["staged_docs"],
        serde_json::json!([]),
        "`staged_docs` is present-always, `[]` when the area staged nothing"
    );
    assert_eq!(doc["findings"], serde_json::json!([]));
}
