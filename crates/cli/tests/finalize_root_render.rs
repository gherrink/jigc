//! M37 Increment 2 / T2 — the non-destructive finalize **root-render** through the real
//! `jigc` binary (`design/design-altitude-doctypes.md` → §4 The vision surface → Root
//! render; `ideas/root-changelog-render.md`).
//!
//! Drives the built binary over a throwaway `root-render` doctype (`note`, persisted to
//! `notes/`, declaring `root-render: NOTE.md`) seeded at project scope via a listed fixture
//! pack. Two arms prove the mechanism honors the no-silent-data-loss invariant:
//!
//!   - **greenfield** — with no pre-existing root file, `finalize` writes `NOTE.md`
//!     **byte-identical** to the promoted managed `docs/notes/my-note.md` and commits both
//!     in the SAME commit (the root file is a deterministic regenerated artifact);
//!   - **pre-existing foreign root** — a hand-authored `NOTE.md` with no managed doc yet
//!     makes `finalize` **block** (`finalize.root-render-foreign`, exit 3) and does NOT
//!     overwrite the foreign file nor create a commit; after the foreign file is removed the
//!     render owns the root file and lands.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the temp repo
//! is a real `git init`, and a self-cleaning `TempDir` keeps the test off the developer's
//! repo. The base pack is the binary-embedded dev pack (its `commit` doctype loads); the
//! listed fixture pack unions the `note` doctype + a create-gate host workflow on top.

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
            "jigc-root-render-{tag}-{}-{:?}",
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

/// Seed the fixture pack holding the throwaway `root-render` doctype + a create-gate host
/// workflow, and list it (highest-precedence) in the project layer's `packs.yaml`. The
/// schema declares `location: notes/` + `root-render: NOTE.md`; the workflow's create-gate
/// admits it so `jigc doc create note` mints in-task.
fn seed_fixture_pack(repo: &Path, pack: &Path) {
    let schemas = pack.join("schemas");
    let workflows = pack.join("workflows");
    let steps = pack.join("steps");
    let config = pack.join("config");
    for d in [&schemas, &workflows, &steps, &config] {
        fs::create_dir_all(d).expect("mk fixture pack subdir");
    }
    fs::write(
        schemas.join("note.yaml"),
        "type: note\n\
         location: notes/\n\
         root-render: NOTE.md\n\
         id-from: title\n\
         description: A throwaway note doctype whose managed doc renders to a repo-root NOTE.md.\n\
         usage: proving the root-render mechanism through the binary.\n\
         sections:\n\
        \x20 - id: body\n\
        \x20   slot: { hint: the note body }\n",
    )
    .expect("seed note schema");
    fs::write(
        workflows.join("host-note.yaml"),
        "---\n\
         when: write a note that renders to the repo root\n\
         description: A host workflow that creates a note.\n\
         usage: proving the root-render note doctype through the binary.\n\
         creates-task: true\n\
         allows-create: [{type: note, as: note}]\n\
         ---\n\
         {{ include: step:author-note }}\n",
    )
    .expect("seed host-note workflow");
    fs::write(
        steps.join("author-note.yaml"),
        "Author the note for the following intent:\n\n{{ task.intent }}\n",
    )
    .expect("seed author-note step");
    fs::write(config.join("commands.yaml"), "commands: []\n").expect("seed empty catalog");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", pack.display()),
    )
    .expect("write packs.yaml naming the fixture pack");
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer, seed
/// the fixture pack, and `jigc start --workflow host-note` a task. Returns the repo + $HOME
/// temp dirs + the fixture-pack temp dir (kept alive for the test's duration).
fn started_repo(intent: &str) -> (TempDir, TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = TempDir::new("pack");
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");
    seed_fixture_pack(repo.path(), pack.path());

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "host-note", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start --workflow host-note` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home, pack)
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
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

/// Assert a `jigc` invocation exited 0, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Create + author the note doc, then fill the commit doc's two required levers, so
/// `finalize` over the task validates clean and the root-render guard is the only variable.
fn author_note_and_commit(repo: &Path, home: &Path, task: &str) {
    let create = run_jigc(
        repo,
        home,
        &["doc", "create", "note", "--title", "My Note"],
        None,
    );
    assert_ok(&create, "jigc doc create note");
    let addr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    assert_eq!(addr, "note:my-note", "id-from: title slugs the note");

    assert_ok(
        &run_jigc(
            repo,
            home,
            &["doc", "set-slot", "note:my-note#body", "--from-file", "-"],
            Some(b"A durable note worth rendering to the repo root.\n"),
        ),
        "set-slot note body",
    );

    // The commit doctype's two required levers (dev-pack `commit`): type + summary.
    assert_ok(
        &run_jigc(
            repo,
            home,
            &[
                "doc",
                "set-field",
                &format!("commit:{task}#type"),
                "--value",
                "docs",
            ],
            None,
        ),
        "set-field commit type",
    );
    assert_ok(
        &run_jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("commit:{task}#summary"),
                "--from-file",
                "-",
            ],
            Some(b"record a note\n"),
        ),
        "set-slot commit summary",
    );
}

#[test]
fn finalize_renders_the_root_target_byte_identical_and_commits_it_together() {
    let (repo, home, _pack) = started_repo("write a note");
    let task = "write-a-note";
    author_note_and_commit(repo.path(), home.path(), task);

    let log_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = run_jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    assert_ok(&out, "jigc task finalize (greenfield root-render)");

    // Exactly ONE new commit.
    let log_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "a root-render finalize must produce exactly ONE new commit"
    );

    // The managed doc AND the root render are committed in the SAME commit.
    let files = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        files.lines().any(|l| l == "docs/notes/my-note.md"),
        "the promoted managed doc must land in the commit; files:\n{files}"
    );
    assert!(
        files.lines().any(|l| l == "NOTE.md"),
        "the rendered root file must land in the SAME commit; files:\n{files}"
    );

    // The rendered root file is BYTE-IDENTICAL to the promoted managed doc.
    let managed = Command::new("git")
        .args(["show", "HEAD:docs/notes/my-note.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show managed");
    assert!(managed.status.success(), "managed doc committed");
    let rendered = Command::new("git")
        .args(["show", "HEAD:NOTE.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show render");
    assert!(rendered.status.success(), "root render committed");
    assert_eq!(
        rendered.stdout, managed.stdout,
        "the root render must be byte-identical to the promoted managed doc"
    );
    // And the WORKTREE root file matches too (the write, not just the commit).
    let worktree_note = fs::read(repo.path().join("NOTE.md")).expect("read worktree NOTE.md");
    assert_eq!(
        worktree_note, managed.stdout,
        "the worktree root render must equal the managed doc bytes"
    );

    // The managed doc is recorded in file-state (its reconcile baseline). The root render
    // carries only the universal committed-file baseline every committed file gets — it is
    // never a *managed doc* (never in the engine's `hash_updates` set — proven in the engine
    // unit — and outside any `location:` dir, so the reconcile sweep never treats it as one).
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    let record_json = fs::read_to_string(&record).expect("read the file-state record");
    assert!(
        record_json.contains("docs/notes/my-note.md"),
        "the managed doc's file-state hash is recorded; got:\n{record_json}"
    );
}

#[test]
fn finalize_blocks_a_foreign_root_file_without_overwriting_it() {
    let (repo, home, _pack) = started_repo("write a note over a foreign root");
    let task = "write-a-note-over-a-foreign-root";
    author_note_and_commit(repo.path(), home.path(), task);

    // A hand-authored root NOTE.md that jigc did NOT generate (the existing-project on-ramp).
    const FOREIGN: &[u8] = b"# Notes\n\nhand-authored by a human, not jigc.\n";
    fs::write(repo.path().join("NOTE.md"), FOREIGN).expect("write the foreign root file");

    let log_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);

    let out = run_jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a foreign root file with no managed doc must block finalize (exit 3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("finalize.root-render-foreign") && stderr.contains("NOTE.md"),
        "the block must name the root-render-foreign code + the foreign target; got:\n{stderr}"
    );

    // The foreign file was NOT overwritten (no silent data loss), and no commit was created.
    let after = fs::read(repo.path().join("NOTE.md")).expect("read NOTE.md after the block");
    assert_eq!(
        after, FOREIGN,
        "a blocked finalize must not overwrite the foreign file"
    );
    let log_after = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    assert_eq!(
        log_before, log_after,
        "a root-render-foreign block must create no commit"
    );
    // The managed doc was NOT promoted (the transaction aborted before promote).
    assert!(
        !repo
            .path()
            .join("docs")
            .join("notes")
            .join("my-note.md")
            .exists(),
        "a blocked finalize promotes nothing"
    );

    // Recovery: remove the foreign file, re-run — the render now owns the root file and lands.
    fs::remove_file(repo.path().join("NOTE.md")).expect("remove the foreign root file");
    let out = run_jigc(repo.path(), home.path(), &["task", "finalize", task], None);
    assert_ok(
        &out,
        "jigc task finalize after removing the foreign root file",
    );
    let managed = Command::new("git")
        .args(["show", "HEAD:docs/notes/my-note.md"])
        .current_dir(repo.path())
        .output()
        .expect("git show managed");
    assert!(
        managed.status.success(),
        "the managed doc lands after recovery"
    );
    let rendered = fs::read(repo.path().join("NOTE.md")).expect("read the rendered NOTE.md");
    assert_eq!(
        rendered, managed.stdout,
        "after recovery the render owns the root file, byte-identical to the managed doc"
    );
}
