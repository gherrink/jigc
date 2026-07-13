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

/// A canonical committed ADR (the shipped `write::render` shape): its `status` header
/// section carries the `status` enum + `date` field leaves — the archetypal
/// `#<section>/<field>` target `jigc validate` itself emits.
const COMMITTED_ADR: &str = "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# Single-node cache\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nA single in-memory node.\n\n## Consequences\n\nNone.\n";

/// (M42 inc-8 T1) The `#<section>/<leaf>` read resolves on a **non-repeatable** section
/// — through the REAL binary, on the emitted bytes, plain **and** `--format json`.
///
/// This is the round-trip the tool broke: `jigc validate` emits `#<section>/<field>`
/// targets (`schema-conformance.required-field-present` / `field-value-conformant`) and
/// `jigc doc set-field` accepts the same string at exit 0, while `jigc doc show` answered
/// `blocking · store.no-such-item`. The engine branch and the json projection are one
/// atomic unit: the engine alone would turn today's honest block into a json
/// wrong-node-exit-0 (the empty slot string), the exact class this wave closes.
#[test]
fn section_leaf_slice_resolves_a_field_in_a_simple_section() {
    let repo = TempDir::new("adr-leaf");
    let home = TempDir::new("adr-leaf-home");
    init_repo(repo.path());
    let adr = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&adr).expect("mk docs/decisions/");
    fs::write(adr.join("single-node-cache.md"), COMMITTED_ADR).expect("write committed adr");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "adr"]);

    // (1) Plain: the leaf's canonical rendered value (an enum leaf reads lowercase).
    for (leaf, want) in [("status", "accepted"), ("date", "2026-05-23")] {
        let addr = format!("adr:single-node-cache#status/{leaf}");
        let plain = ok_stdout(
            &run_jigc(repo.path(), home.path(), &["doc", "show", &addr]),
            &format!("plain `{addr}`"),
        );
        assert_eq!(plain, want, "plain `{addr}` is the field's value");

        // (2) `--format json`: the same value, shaped exactly as in `fields` (a string).
        let json = ok_stdout(
            &run_jigc(
                repo.path(),
                home.path(),
                &["doc", "show", &addr, "--format", "json"],
            ),
            &format!("json `{addr}`"),
        );
        assert_eq!(
            json,
            format!("\"{want}\""),
            "json `{addr}` is the leaf value, never the empty slot string",
        );
    }

    // (3) An absent leaf name blocks honestly — `store.no-such-leaf` + a route, never a
    //     wrong node at exit 0.
    let bad = run_jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:single-node-cache#status/nope"],
    );
    assert!(
        !bad.status.success(),
        "an absent leaf must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&bad.stdout),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("store.no-such-leaf")
            && stderr.contains("nope")
            && stderr.contains("route:"),
        "the block names the absent leaf + carries a route; got:\n{stderr}",
    );
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
