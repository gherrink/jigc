//! Acceptance — the store-scope `schema-conformance.mention-resolves` check
//! (M33 Increment 3, T1): an in-prose **managed mention** `#<type>:<slug>` that
//! names no committed doc is reported, advisory, at store scope.
//!
//! A **managed mention** is a `#<type>:<slug>` token in slot prose whose `<type>` is a
//! known managed doctype — the `:`-plus-known-doctype discriminator, so a bare external
//! `#issue-42` is *never* a managed mention and is never flagged
//! (`design/document-type-schema.md` → In-prose mentions, settled M33;
//! `design/validation.md` → the mention-resolves check). It resolves by the same
//! `committed_reachable` rule the `ref-resolves` families use, and is the **lighter,
//! prose-embedded sibling** of `ref-resolves`: **default severity advisory**,
//! **store-scope only** (the cross-doc backstop — a mention dangles when *another* doc is
//! renamed/deleted), **never a per-task finalize gate**. Doc-level only in v1
//! (section-anchor mentions deferred).
//!
//! This drives the built `jigc` binary through the real store-scope `jigc validate` and
//! the per-task `jigc task finalize` (the M10 invocation-path-masking lesson: drive the
//! bytes an operator would actually run, never a reconstructed equivalent):
//!
//! - **(dangling)** — a committed ADR whose `## Decision` slot prose carries
//!   `#adr:does-not-exist` (no such committed doc) surfaces **exactly one** advisory
//!   `schema-conformance.mention-resolves` finding at store scope, naming the dangling
//!   mention; `jigc validate` still exits 0 (advisory, report-only).
//! - **(external + resolving both pass)** — a committed ADR whose prose carries an
//!   external `#issue-42` (not a managed mention) **and** a resolving `#adr:<slug>`
//!   (a committed ADR by that slug) surfaces **none**.
//! - **(never a per-task gate)** — a `jigc task finalize` over a task whose created ADR
//!   carries a dangling managed mention emits **no** mention-resolves finding and
//!   finalizes clean — the obligation is never the per-task gate.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init` + `jigc setup`, the probe is the real built `doc-code`
//! (so the `jigc validate` pre-flight resolves), and a self-cleaning `TempDir` keeps
//! the test off the developer's repo.

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
            "jigc-mention-{tag}-{}-{:?}",
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

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the **real** tree-sitter subprocess the engine/CLI seam drives, so the `jigc
/// validate` probe pre-flight resolves (the `prd_inverse_cardinality.rs` idiom).
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

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, and the real `doc-code` probe
/// selected via `JIGC_DOC_CODE_PROBE` so the validate pre-flight resolves.
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

/// Make `root` a real git repo with identity, then run `jigc setup` over it (the
/// project layer + probe extraction). Returns once the store is a clean, set-up repo.
fn setup_repo(repo: &Path, home: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);

    let out = jigc(repo, home, &["setup"]);
    assert_ok(&out, "`jigc setup`");
}

/// A conformant committed ADR body: `accepted` status header, then the three prose
/// slots filled with the given `context` / `decision` prose (consequences fixed).
fn adr_body(title: &str, context: &str, decision: &str) -> String {
    format!(
        "---\n\
         status: accepted\n\
         date: 2026-05-23\n\
         ---\n\
         \n\
         # {title}\n\
         \n\
         ## Context\n\
         \n\
         {context}\n\
         \n\
         ## Options\n\
         Alternatives were weighed and rejected.\n\
         \n\
         ## Decision\n\
         \n\
         {decision}\n\
         \n\
         ## Consequences\n\
         \n\
         None.\n",
    )
}

/// Commit a conformant ADR at its canonical `docs/decisions/<slug>.md`.
fn commit_adr(repo: &Path, slug: &str, body: &str) {
    let dir = repo.join("docs").join("decisions");
    fs::create_dir_all(&dir).expect("mk docs/decisions/");
    fs::write(dir.join(format!("{slug}.md")), body).expect("write adr");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", &format!("seed {slug}")]);
}

/// Count non-overlapping occurrences of `needle` in `haystack`.
fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// (dangling) A committed ADR whose `## Decision` slot prose carries a managed mention
/// `#adr:does-not-exist` naming no committed doc surfaces **exactly one** advisory
/// `schema-conformance.mention-resolves` finding at store scope; `jigc validate` still
/// exits 0 (report-only).
#[test]
fn store_sweep_surfaces_dangling_managed_mention() {
    let repo = TempDir::new("dangling");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    commit_adr(
        repo.path(),
        "single-node-cache",
        &adr_body(
            "Single-node cache",
            "Forces at play.",
            "Keep sessions on a single node; see #adr:does-not-exist for the prior call.",
        ),
    );

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    // Advisory, report-only: a content-only sweep exits 0.
    assert!(
        out.status.success(),
        "an advisory mention finding must not gate `jigc validate` (exit 0); \
         stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert_eq!(
        count(&stdout, "schema-conformance.mention-resolves"),
        1,
        "a slot-prose managed mention naming no committed doc must surface EXACTLY ONE \
         advisory mention-resolves finding at store scope; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("adr:does-not-exist"),
        "the finding must name the dangling mention `adr:does-not-exist`; stdout:\n{stdout}",
    );
}

/// (external + resolving both pass) A committed ADR whose prose carries an external
/// `#issue-42` (not a managed mention) **and** a resolving `#adr:<slug>` (a committed
/// ADR by that slug) surfaces **no** mention-resolves finding.
#[test]
fn store_sweep_passes_external_and_resolving_mentions() {
    let repo = TempDir::new("pass");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());
    // The resolve target — a committed ADR by the slug the referrer mentions.
    commit_adr(
        repo.path(),
        "single-node-cache",
        &adr_body("Single-node cache", "Forces.", "Keep sessions on one node."),
    );
    // The referrer: an external `#issue-42` (no `<type>:` — never a managed mention) in
    // context, and a resolving `#adr:single-node-cache` in decision.
    commit_adr(
        repo.path(),
        "session-eviction",
        &adr_body(
            "Session eviction",
            "Tracking #issue-42 in the external issue tracker.",
            "Evict sessions per #adr:single-node-cache.",
        ),
    );

    let out = jigc(repo.path(), home.path(), &["validate"]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "a content-only sweep must exit 0; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
    assert!(
        !stdout.contains("schema-conformance.mention-resolves"),
        "an external `#issue-42` is not a managed mention and a resolving `#adr:<slug>` \
         names a committed doc, so NO mention-resolves finding may surface; stdout:\n{stdout}",
    );
}

/// (never a per-task gate) A `jigc task finalize` over a task whose created ADR carries
/// a dangling managed mention in its prose emits **no** mention-resolves finding and
/// finalizes clean — the obligation is **store-scope only**, never the per-task gate
/// (`design/validation.md` → never a per-task finalize gate).
#[test]
fn task_finalize_never_emits_mention_resolves() {
    let repo = TempDir::new("per-task");
    let home = TempDir::new("home");
    setup_repo(repo.path(), home.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &[
            "start",
            "--workflow",
            "single-task",
            "cache sessions in a single in-memory node",
        ],
    );
    assert_ok(&out, "`jigc start`");
    let task = "cache-sessions-in-a-single";

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Single-node cache"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");

    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(
            repo.path(),
            home.path(),
            &["set-field", addr, "--value", value],
            None,
        );
        assert_ok(&out, &format!("set-field {addr}"));
    };
    set_slot(
        "adr:single-node-cache#context",
        b"Session lookups must stay sub-millisecond.\n",
    );
    // A dangling managed mention in the created ADR's decision prose — store-scope only,
    // so `validate_task` (the finalize gate) must NOT surface it.
    set_slot(
        "adr:single-node-cache#decision",
        b"Keep sessions in a single in-memory node; see #adr:does-not-exist.\n",
    );
    set_slot(
        "adr:single-node-cache#consequences",
        b"A cold node loses its sessions.\n",
    );
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"cache sessions\n");
    set_slot(&format!("commit:{task}#body"), b"A cache change.\n");

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_ok(
        &out,
        "`jigc task finalize` over a task whose ADR carries a dangling managed mention",
    );
    assert!(
        !stdout.contains("schema-conformance.mention-resolves")
            && !stderr.contains("schema-conformance.mention-resolves"),
        "the per-task finalize gate (`validate_task`) must NEVER emit a mention-resolves \
         finding — mentions are a store-scope concern; stdout:\n{stdout}\nstderr:\n{stderr}",
    );
}
