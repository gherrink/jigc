//! M47 Increment 10 / T3 — **the five edit verbs ack the copy-in `create` already
//! states** (RC-alpha4 P4-6; `design/command-output-contract.md` §2 → the
//! first-touch copy-in note).
//!
//! Every `jigc doc` edit verb copy-on-writes a committed doc into the task on first
//! touch (`Task::read_or_copy_in`, the sibling suite [`doc_copy_in`] proves the
//! staging itself), and until now said nothing about it: only `doc create` stated
//! the effect (`existed: true`, M43's law-1 fix). The trial's worker learned their
//! task had promoted a committed ADR from the **finalize** output — the M43 statement
//! was an incomplete fix over the *verb* axis.
//!
//! So this suite iterates that axis rather than pinning one verb: over the SAME
//! committed `spec`, each of `set-field` (both arms — `--value` and `--unset`),
//! `set-slot`, `add-item`, `remove-item` and `retitle-item` gets its own fresh task,
//! and each must
//!
//! - state the copy-in on its **first** touch of the committed doc, and
//! - **not** repeat it on a **second** edit in the same task (the steady-state path,
//!   `read_or_copy_in` case 1 — the doc is already staged, nothing is copied in).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-copy-in-ack-{tag}-{}-{:?}",
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

/// A committed 2-criteria `spec` carrying an optional `derived-from` ref — the
/// substrate every arm copies in. Two items so the item verbs get a *second* target
/// for their steady-state edit; the ref so both `set-field` arms have a real field.
const COMMITTED_SPEC: &str = "\
---
derived-from: prd:payments-platform
---

# Rate limit

## Goal

Bound per-client request volume.

## Context

Downstream services enforced limits ad hoc.

## Criteria

### Rejects the 101st request  {#rejects-burst}

The gateway rejects the 101st request in a rolling 60s window.

### Admits within the window  {#admits-within}

Requests under the cap are admitted unchanged.
";

/// Initialize a real git repo whose only managed doc is the committed spec above.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    fs::create_dir_all(repo.join("docs").join("specs")).expect("create docs/specs/");
    fs::write(
        repo.join("docs").join("specs").join("rate-limit.md"),
        COMMITTED_SPEC,
    )
    .expect("write the committed spec");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc <args>`, asserting exit 0, returning stdout.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = jigc(repo, home, args);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Mint a fresh `single-task` for `intent`, returning the id the composed output
/// carries (never a re-derived guess — the slug rule is the CLI's, not the test's).
fn mint_task(repo: &Path, home: &Path, intent: &str) -> String {
    let stdout = ok(
        repo,
        home,
        &[
            "--format",
            "json",
            "start",
            "--workflow",
            "single-task",
            intent,
        ],
        "jigc start --workflow single-task",
    );
    let composed: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("the composed json parses");
    composed["task"]
        .as_str()
        .expect("a work-minting start carries the task id")
        .to_string()
}

/// The phrase the first-touch note is built around — the copy-in's *effect*, in
/// `DocAck::Created`'s mold ("already existed — copied in for update").
const COPY_IN_PHRASE: &str = "copied in for update";

/// The whole five-verb axis, in one test: every edit verb states the copy-in on its
/// first touch of a committed doc and states it exactly once per task.
#[test]
fn every_edit_verb_states_the_copy_in_on_first_touch_and_never_twice() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    let prose = repo.path().join("prose.txt");
    fs::write(&prose, "Replacement prose.\n").expect("write the slot prose");
    let prose = prose.to_string_lossy().to_string();

    // (verb-arm label, the arm's own `jigc start` intent, the FIRST touch of the
    // committed spec, a SECOND edit of the same doc in the same task). Every arm addresses the same committed
    // `spec:rate-limit`, so the only thing that varies is the verb under test.
    let arms: Vec<(&str, &str, Vec<String>, Vec<String>)> = vec![
        (
            "set-field --value",
            "edit by field value",
            argv(&[
                "set-field",
                "spec:rate-limit#meta/derived-from",
                "--value",
                "prd:gateway",
            ]),
            argv(&[
                "set-field",
                "spec:rate-limit#meta/derived-from",
                "--value",
                "prd:edge",
            ]),
        ),
        (
            "set-field --unset",
            "edit by field unset",
            argv(&["set-field", "spec:rate-limit#meta/derived-from", "--unset"]),
            argv(&[
                "set-field",
                "spec:rate-limit#meta/derived-from",
                "--value",
                "prd:edge",
            ]),
        ),
        (
            "set-slot",
            "edit by slot prose",
            argv(&["set-slot", "spec:rate-limit#goal", "--from-file", &prose]),
            argv(&["set-slot", "spec:rate-limit#context", "--from-file", &prose]),
        ),
        (
            "add-item",
            "edit by item add",
            argv(&[
                "add-item",
                "spec:rate-limit#criteria",
                "--title",
                "Warms on boot",
            ]),
            argv(&[
                "add-item",
                "spec:rate-limit#criteria",
                "--title",
                "Evicts under pressure",
            ]),
        ),
        (
            "remove-item",
            "edit by item remove",
            argv(&["remove-item", "spec:rate-limit#criteria/rejects-burst"]),
            argv(&["remove-item", "spec:rate-limit#criteria/admits-within"]),
        ),
        (
            "retitle-item",
            "edit by item retitle",
            argv(&[
                "retitle-item",
                "spec:rate-limit#criteria/rejects-burst",
                "--title",
                "Rejects the burst",
            ]),
            argv(&[
                "retitle-item",
                "spec:rate-limit#criteria/admits-within",
                "--title",
                "Admits inside the window",
            ]),
        ),
    ];

    // Collected, never asserted arm-by-arm: a bail on the first arm would leave the
    // rest of the axis unmeasured, which is the reporting shape this fix exists to
    // stop (a green over one verb hid the silence on the other four).
    let mut silent: Vec<String> = Vec::new();
    let mut repeated: Vec<String> = Vec::new();

    for (label, intent, first, second) in arms {
        // A fresh task per arm, so every arm's write really is a FIRST touch.
        let task = mint_task(repo.path(), home.path(), intent);
        let staged = repo
            .path()
            .join(".jigc/tasks")
            .join(&task)
            .join("docs")
            .join("spec:rate-limit.md");
        assert!(
            !staged.exists(),
            "[{label}] the committed spec must not be pre-staged in `{task}`",
        );

        let first = ok(
            repo.path(),
            home.path(),
            &with_task(&first, &task),
            &format!("jigc doc {label} (first touch)"),
        );
        assert!(
            staged.is_file(),
            "[{label}] the first touch must stage the committed spec",
        );
        if !first.contains(COPY_IN_PHRASE) {
            silent.push(format!("{label} → {}", first.trim()));
        }

        let second = ok(
            repo.path(),
            home.path(),
            &with_task(&second, &task),
            &format!("jigc doc {label} (second edit)"),
        );
        if second.contains("copied in") {
            repeated.push(format!("{label} → {}", second.trim()));
        }
    }

    assert!(
        silent.is_empty(),
        "every edit verb's first touch of a committed doc must state the copy-in \
         (`{COPY_IN_PHRASE}`); silent arms:\n{}",
        silent.join("\n"),
    );
    assert!(
        repeated.is_empty(),
        "a second edit in the same task copies nothing in, so no arm may repeat the \
         note; repeating arms:\n{}",
        repeated.join("\n"),
    );
}

/// `["doc", …]` — the verb argv an arm carries, owned so the arm table is one type.
fn argv(args: &[&str]) -> Vec<String> {
    std::iter::once("doc".to_string())
        .chain(args.iter().map(|a| a.to_string()))
        .collect()
}

/// `<argv> --task <task>` as the `&str` slice the runner takes.
fn with_task<'a>(args: &'a [String], task: &'a str) -> Vec<&'a str> {
    args.iter()
        .map(String::as_str)
        .chain(["--task", task])
        .collect()
}
