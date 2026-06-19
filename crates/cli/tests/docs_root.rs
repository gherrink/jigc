//! Consistency acceptance — the `docs-root` knob nests every persisted doctype's
//! managed docs under a single parent (default `docs/`), and the THREE schema-load
//! surfaces agree on that parent (`DECISIONS.md` 2026-06-18; `design/storage.md` →
//! Repository layout / Config layout).
//!
//! Drives the built `jigc` binary against a throwaway git repo through the whole
//! create → author → finalize → ingest spine and asserts:
//!
//! - **Surface C (finalize-promote write path)** lands the promoted ADR at
//!   `docs/decisions/<slug>.md`, not the old flat `decisions/<slug>.md`.
//! - **Addressing is unchanged** — the created doc is still `adr:<slug>` (only the
//!   on-disk path gained the prefix).
//! - **Surface A/B agree on the parent** — a follow-up `jigc ingest` classifies the
//!   promoted doc at `docs/decisions/<slug>.md` as *adoptable* (and adopts it). Were
//!   the ingest read-surface still pointing at the flat `decisions/`, the doc would
//!   be a wrong-location `needs-reconcile` — so an *adopted* verdict proves the read
//!   and write surfaces resolve the same parent (the three-surface consistency guard).
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp
//! repo is a real `git init`, and a self-cleaning `TempDir` keeps the test off the
//! developer's repo.

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
            "jigc-docs-root-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, capturing output.
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .output()
        .expect("run the jigc binary")
}

/// Run `jigc doc <args>`, optionally piping `stdin`, capturing output.
fn jigc_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc").args(args);
    command.current_dir(repo).env("HOME", home);
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

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = jigc_doc(repo, home, &["set-field", addr, "--value", value], None);
        assert_ok(&out, &format!("set-field {addr}"));
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(
        &format!("commit:{task}#summary"),
        b"nest docs under a parent\n",
    );
    set_slot(&format!("commit:{task}#body"), b"A docs-root change.\n");
}

/// Fill an ADR's author-required prose slots.
fn fill_adr_slots(repo: &Path, home: &Path, slug: &str) {
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc_doc(
            repo,
            home,
            &["set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert_ok(&out, &format!("set-slot {addr}"));
    };
    set_slot(
        &format!("adr:{slug}#context"),
        b"The managed-doc dirs polluted the repo root.\n",
    );
    set_slot(
        &format!("adr:{slug}#decision"),
        b"Nest every persisted doctype under docs/.\n",
    );
    set_slot(
        &format!("adr:{slug}#consequences"),
        b"The repo root stays clean.\n",
    );
}

#[test]
fn docs_root_nests_promoted_adr_under_docs_and_the_three_surfaces_agree() {
    let repo = TempDir::new("nest");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // ── on-ramp ──────────────────────────────────────────────────────────────────
    let out = jigc(repo.path(), home.path(), &["setup"]);
    assert_ok(&out, "`jigc setup`");

    // ── mint a task, create + author an ADR, finalize ────────────────────────────
    let out = jigc(
        repo.path(),
        home.path(),
        &["start", "--workflow", "single-task", "nest managed docs"],
    );
    assert_ok(&out, "`jigc start`");
    let task = "nest-managed-docs";

    let create = jigc_doc(
        repo.path(),
        home.path(),
        &["create", "adr", "--title", "Nest managed docs under docs"],
        None,
    );
    assert_ok(&create, "`jigc doc create adr`");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    // Addressing is unchanged — only the on-disk path gains the prefix.
    assert_eq!(
        adr, "adr:nest-managed-docs-under-docs",
        "the created doc keeps its `adr:<slug>` identity"
    );
    let slug = "nest-managed-docs-under-docs";

    fill_adr_slots(repo.path(), home.path(), slug);
    fill_commit(repo.path(), home.path(), task);

    let out = jigc(repo.path(), home.path(), &["task", "finalize", task]);
    assert_ok(&out, "`jigc task finalize`");

    // ── Surface C: the promoted ADR lands under docs/, not the flat root ─────────
    assert!(
        repo.path()
            .join("docs/decisions")
            .join(format!("{slug}.md"))
            .exists(),
        "the promoted ADR must land at docs/decisions/{slug}.md (the docs-root prefix)",
    );
    assert!(
        !repo
            .path()
            .join("decisions")
            .join(format!("{slug}.md"))
            .exists(),
        "the old flat docs/decisions/{slug}.md must NOT exist (the prefix is applied)",
    );
    // Committed at the nested canonical path.
    let committed = Command::new("git")
        .args(["show", &format!("HEAD:docs/decisions/{slug}.md")])
        .current_dir(repo.path())
        .output()
        .expect("git show");
    assert!(
        committed.status.success(),
        "finalize must commit docs/decisions/{slug}.md; stderr:\n{}",
        String::from_utf8_lossy(&committed.stderr),
    );

    // ── Surface A/B: ingest recognizes the nested doc as adoptable ───────────────
    // A wrong read-surface (still pointing at flat `docs/decisions/`) would classify the
    // doc at docs/decisions/ as a wrong-location `needs-reconcile`; an *adopted*
    // verdict proves read + write resolve the same parent (the consistency guard).
    let out = jigc(repo.path(), home.path(), &["ingest"]);
    assert_ok(&out, "`jigc ingest`");
    let report = String::from_utf8(out.stdout).expect("utf-8");
    let row = report
        .lines()
        .find(|l| {
            l.contains(&format!("docs/decisions/{slug}.md"))
                && !l.trim_start().starts_with("route:")
        })
        .unwrap_or_else(|| panic!("the promoted ADR must appear in `jigc ingest`:\n{report}"));
    assert!(
        row.contains("adoptable") || row.contains("adopted"),
        "ingest must classify docs/decisions/{slug}.md as adoptable (read+write agree \
         on the docs-root parent); row:\n{row}\nfull report:\n{report}",
    );
}
