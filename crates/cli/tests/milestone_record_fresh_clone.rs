//! M39 Increment 4 / T5 — fresh-clone resume: reseed the demoted `.jigc` cache from the
//! committed record on the milestone-op read path (`design/team-ready-state.md` → Engine
//! capability 2 (read-back): "on a fresh clone (no `.jigc/` working state) the first
//! milestone op parses the record back into `BasePin` + `TaskList` and re-seeds the cache";
//! "Continue" means resume, not WIP recovery). Under a `[dev ▸ methodology]` project whose
//! composed cascade resolves the `milestone-record` schema.
//!
//! One flow, driving the REAL binary against a throwaway git repo:
//!
//!   create → add-task ×2 → **simulate a fresh clone** (`rm -rf .jigc`, then restore the
//!   *tracked* `.jigc` bits — committed `config/` + `.gitignore` — the way a clone would,
//!   leaving the gitignored WIP `.jigc/milestones/` gone). Then:
//!     (a) `jigc doc show milestone-record:<id> --format json` returns the pinned shape
//!         (Inc 1's committed-record read — works without the cache); and
//!     (b) a subsequent milestone op (`list-tasks`) **re-derives the cache from the record**
//!         and continues (resume-from-scratch), rebuilding `.jigc/milestones/<id>/`.
//!
//! Pre-fix, (b) fails: `list-tasks` bails "milestone does not exist" because the WIP cache
//! is gone and nothing reseeds it from the committed record.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-milestone-record-fresh-clone-{tag}-{}-{:?}",
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
    String::from_utf8(out.stdout).expect("utf-8 git stdout")
}

/// Initialize a real git repo with one commit (mint reads HEAD via `git rev-parse`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Write the `[dev ▸ methodology]` compose marker — the exact key `make_pack` reads to
/// assemble the composition dev-highest (so the dev `docs-root` knob applies → `docs/`).
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT, else the env pack
/// supersedes the marker).
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert an invocation exited 0, surfacing stderr on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// The demoted `.jigc` cache dir for milestone `cache-rework`.
fn cache_dir(repo: &Path) -> PathBuf {
    repo.join(".jigc").join("milestones").join("cache-rework")
}

#[test]
fn fresh_clone_reseeds_cache_from_record_and_resumes() {
    let repo = TempDir::new("resume");
    let home = TempDir::new("home");
    init_repo(repo.path());
    write_compose_marker(repo.path());

    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "`[dev ▸ methodology]` `jigc milestone create`",
    );
    // Commit the *tracked* `.jigc` bits (the compose marker + the generated `.gitignore`)
    // exactly as `jigc setup` would — these survive a clone; the WIP under `.jigc/` does not.
    git(
        repo.path(),
        &["add", ".jigc/config/packs.yaml", ".jigc/.gitignore"],
    );
    git(repo.path(), &["commit", "-q", "-m", "jigc config"]);

    assert_ok(
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
        "add-task #1",
    );
    assert_ok(
        &run_jigc(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Evict cold entries",
            ],
        ),
        "add-task #2",
    );

    let record_before = fs::read_to_string(record_path(repo.path())).expect("record committed");

    // --- Simulate a fresh clone: drop ALL of `.jigc/`, then restore only the *tracked*
    //     bits (committed `config/` + `.gitignore`) the clone would carry. The gitignored
    //     WIP (`.jigc/milestones/…`) is gone — nothing on disk re-derives the milestone.
    fs::remove_dir_all(repo.path().join(".jigc")).expect("rm -rf .jigc");
    git(repo.path(), &["checkout", "--", ".jigc"]);
    assert!(
        !cache_dir(repo.path()).exists(),
        "the WIP cache must be absent after the fresh-clone simulation",
    );

    // (a) `doc show` reads the committed record directly (Inc 1) — works without the cache.
    let show = run_jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "show",
            "milestone-record:cache-rework",
            "--format",
            "json",
        ],
    );
    assert_ok(&show, "fresh-clone `doc show`");
    let json = String::from_utf8(show.stdout).expect("utf-8 json");
    // The pinned `--format json` shape (§ The read surface): type, slug, fields{base,status},
    // sections{tasks:[{task-id,intent,status}…]}.
    assert!(
        json.contains("\"type\": \"milestone-record\"")
            && json.contains("\"slug\": \"cache-rework\"")
            && json.contains("\"status\": \"active\"")
            && json.contains("\"task-id\": \"warm-the-read-cache\"")
            && json.contains("\"intent\": \"Warm the read cache\"")
            && json.contains("\"task-id\": \"evict-cold-entries\""),
        "fresh-clone `doc show --format json` returns the pinned record shape; got:\n{json}",
    );

    // (b) A subsequent milestone op re-derives the cache from the record and continues.
    let list = run_jigc(
        repo.path(),
        home.path(),
        &["milestone", "list-tasks", "cache-rework"],
    );
    assert_ok(&list, "fresh-clone `list-tasks` (resume-from-scratch)");
    let out = String::from_utf8(list.stdout).expect("utf-8 list-tasks stdout");
    assert!(
        out.contains("evict-cold-entries") && out.contains("warm-the-read-cache"),
        "fresh-clone `list-tasks` emits the re-derived sub-task ids; got:\n{out}",
    );

    // The op re-seeded the demoted cache from the committed record (resume, not WIP recovery).
    assert!(
        cache_dir(repo.path()).join("tasks.json").is_file()
            && cache_dir(repo.path()).join("base.json").is_file(),
        "the milestone op re-seeded `.jigc/milestones/cache-rework/{{base,tasks}}.json` from the record",
    );

    // The committed record was not disturbed by the read-path reseed (it is the source of truth).
    assert_eq!(
        fs::read_to_string(record_path(repo.path())).expect("record still readable"),
        record_before,
        "the read-path reseed must not rewrite the committed record",
    );
}
