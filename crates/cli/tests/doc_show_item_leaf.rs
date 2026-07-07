//! M39 completion fix — `jigc doc show <ref>#section/<item>/<leaf>` resolves a
//! repeatable item's **field** and **`id-from`** leaves, closing the pinned
//! `doc-read-surface.md` contract's over-claim (the contract table advertises
//! `#section/<item>/<leaf>` as "a field's value", but the milestone-record's field
//! leaves — `task-id` / `intent` / `status` — used to block `store.no-such-leaf`).
//!
//! The **milestone-record is the doc-show contract's own witness doctype** (its per-item
//! leaves are all machine-maintained fields + the `id-from` heading), so it is the
//! faithful reproduction. Driven through the REAL `jigc` binary under the
//! `[dev ▸ methodology]` composition, on the EMITTED bytes (plain + `--format json`).
//!
//! Pre-fix, every leaf slice returned the routed `store.no-such-leaf` block; post-fix the
//! field/id-from leaves resolve to their values, while a genuinely-absent leaf name STILL
//! blocks (the block stays reachable).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-doc-show-item-leaf-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit and the `[dev ▸ methodology]` compose
/// marker (so the composed cascade resolves the `milestone-record` schema).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it absent).
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert an invocation exited 0, surfacing stderr on failure, returning trimmed stdout.
fn ok_stdout(out: &std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim_end()
        .to_string()
}

#[test]
fn item_leaf_slices_resolve_field_and_id_from_leaves() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Build a committed milestone-record with one sub-task item (fields: task-id/intent/status).
    ok_stdout(
        &run_jigc(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "`jigc milestone create`",
    );
    ok_stdout(
        &run_jigc(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Warm the read cache",
            ],
        ),
        "`jigc milestone add-task`",
    );

    let base = "milestone-record:cache-rework#tasks/warm-the-read-cache";

    // (1) The `id-from` leaf → the item's heading; each field leaf → its rendered value.
    //     Plain path (the byte-exact/canonical value).
    for (leaf, want) in [
        ("task-id", "warm-the-read-cache"),
        ("intent", "Warm the read cache"),
        ("status", "active"),
    ] {
        let plain = ok_stdout(
            &run_jigc(
                repo.path(),
                home.path(),
                &["doc", "show", &format!("{base}/{leaf}")],
            ),
            &format!("plain `{base}/{leaf}`"),
        );
        assert_eq!(plain, want, "plain leaf slice of `{leaf}` is its value");

        // (2) `--format json` → the same value as a json string (the pinned leaf shape).
        let json = ok_stdout(
            &run_jigc(
                repo.path(),
                home.path(),
                &["doc", "show", &format!("{base}/{leaf}"), "--format", "json"],
            ),
            &format!("json `{base}/{leaf}`"),
        );
        assert_eq!(
            json,
            format!("\"{want}\""),
            "json leaf slice of `{leaf}` is the value as a json string",
        );
    }

    // (3) GUARD: a genuinely-absent leaf name STILL blocks (the block stays reachable) —
    //     the fix must not make `store.no-such-leaf` unreachable.
    let bad = run_jigc(
        repo.path(),
        home.path(),
        &["doc", "show", &format!("{base}/not-a-leaf")],
    );
    assert!(
        !bad.status.success(),
        "an absent leaf must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&bad.stdout),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("store.no-such-leaf") && stderr.contains("not-a-leaf"),
        "the block names the absent leaf + its route; got:\n{stderr}",
    );
}
