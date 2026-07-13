//! M42 Increment 7 / T6 — the committed **milestone record** stops being writable
//! through the `jigc doc` **write** verbs (A18; `design/team-ready-state.md` → The
//! record is not writable through the `jigc doc` verbs).
//!
//! Every leaf of the `milestone-record` doctype is `set:`-bearing — there is no
//! author-owned prose slot and no author-required field — so a doctype with **no
//! author-owned leaf has no legitimate `jigc doc` write**. Today the rule is applied
//! piecemeal (`jigc rename` refuses a record reslug; `jigc doc retitle-item` refuses a
//! record item) and the remaining write verbs were simply never enumerated: RED,
//! `jigc doc set-field milestone-record:<id>#status --value joined` with **no `--task`**
//! exits **0** — it silently auto-selects the sole live task (a milestone sub-task),
//! copies the committed record into that task's area, and stages it to promote at that
//! task's finalize, mutating the committed record outside the milestone verbs.
//!
//! Two proofs, driving the REAL binary against a throwaway `[dev ▸ methodology]` repo:
//!
//!   (1) **Every write verb refuses.** `create` · `author` · `set-field` (both
//!       `--value` and `--unset`) · `set-slot` · `add-item` · `remove-item` on a
//!       `milestone-record` target exit **non-zero** with a blocking
//!       `write.machine-maintained` finding whose route names the milestone verbs —
//!       and move **no bytes**: the committed record stays byte-identical and no staged
//!       copy is created in the task area (the guard fires before the copy-in staging,
//!       which is itself part of the defect). Each is invoked with **no `--task`**,
//!       exactly the auto-selecting shape that exits 0 today.
//!
//!   (2) **The READ verbs stay open.** `jigc doc show` and `jigc doc schema` on the same
//!       record/doctype still exit 0 — that uniformity is the whole reason the record is
//!       a doctype rather than a raw-JSON island.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-doc-write-milestone-record-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `[dev ▸ methodology]` compose marker
/// (the exact key `make_pack` reads — the composed cascade resolves the methodology-pack
/// `milestone-record` schema).
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
/// `JIGC_PACK_DIR` (the compose-marker path requires it ABSENT).
fn jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert a `jigc` invocation exited 0, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The committed record path under docs-root for milestone `cache-rework`.
fn record_path(repo: &Path) -> PathBuf {
    repo.join("docs")
        .join("milestone-records")
        .join("cache-rework.md")
}

/// The staged copy the defective write path creates in the sole live task's area.
fn staged_record(repo: &Path) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(SUB_TASK)
        .join("docs")
        .join("milestone-record:cache-rework.md")
}

/// The milestone's sole sub-task — and therefore the SOLE live task, the one a
/// `--task`-less `jigc doc` write silently auto-selects.
const SUB_TASK: &str = "warm-the-read-cache";

/// Mint the committed record with one sub-task, so exactly ONE task is live.
fn setup(repo: &Path, home: &Path) {
    assert_ok(
        &jigc(repo, home, &["milestone", "create", "Cache rework"]),
        "jigc milestone create",
    );
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Warm the read cache",
            ],
        ),
        "jigc milestone add-task",
    );
}

/// (1) Every `jigc doc` WRITE verb refuses a `milestone-record` target — non-zero, a
/// blocking `write.machine-maintained` finding routed at the milestone verbs — and moves
/// no bytes (committed record byte-identical, no staged copy).
#[test]
fn every_doc_write_verb_refuses_a_milestone_record_and_moves_no_bytes() {
    let repo = TempDir::new("write");
    let home = TempDir::new("write-home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    let before = fs::read_to_string(record_path(repo.path())).expect("committed record");

    // A real payload + a real prose file: the guard must refuse on the DOCTYPE, not
    // because an input was missing.
    let payload = repo.path().join("payload.yaml");
    fs::write(
        &payload,
        "title: Ghost rework\nsections:\n  - id: meta\n    set:\n      status: active\n",
    )
    .expect("write author payload");
    let prose = repo.path().join("prose.txt");
    fs::write(&prose, "some prose\n").expect("write prose");
    let payload = payload.to_str().expect("utf-8 path").to_string();
    let prose = prose.to_str().expect("utf-8 path").to_string();

    // The six write verbs, each with NO `--task` — the auto-selecting shape that exits
    // 0 today on `set-field`.
    let invocations: Vec<(&str, Vec<&str>)> = vec![
        (
            "create",
            vec![
                "doc",
                "create",
                "milestone-record",
                "--title",
                "Ghost rework",
            ],
        ),
        (
            "author",
            vec!["doc", "author", "milestone-record", "--from-file", &payload],
        ),
        (
            "set-field",
            vec![
                "doc",
                "set-field",
                "milestone-record:cache-rework#status",
                "--value",
                "joined",
            ],
        ),
        (
            "set-field --unset",
            vec![
                "doc",
                "set-field",
                "milestone-record:cache-rework#status",
                "--unset",
            ],
        ),
        (
            "set-slot",
            vec![
                "doc",
                "set-slot",
                "milestone-record:cache-rework#status",
                "--from-file",
                &prose,
            ],
        ),
        (
            "add-item",
            vec![
                "doc",
                "add-item",
                "milestone-record:cache-rework#tasks",
                "--title",
                "Evict cold entries",
            ],
        ),
        (
            "remove-item",
            vec![
                "doc",
                "remove-item",
                "milestone-record:cache-rework#tasks/warm-the-read-cache",
            ],
        ),
    ];

    for (verb, args) in invocations {
        let refused = jigc(repo.path(), home.path(), &args);
        assert!(
            !refused.status.success(),
            "`jigc doc {verb}` on a milestone-record must exit non-zero; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&refused.stdout),
            String::from_utf8_lossy(&refused.stderr),
        );
        let stderr = String::from_utf8_lossy(&refused.stderr).to_string();
        assert!(
            stderr.contains("blocking · write.machine-maintained"),
            "`{verb}` must refuse with a blocking `write.machine-maintained` finding; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("route: ") && stderr.contains("jigc milestone"),
            "`{verb}`'s route must name the milestone verbs; stderr:\n{stderr}",
        );

        // No bytes moved — the guard fires before the copy-in staging (which is itself
        // part of the defect: it stages the record to promote at the sub-task's finalize).
        assert_eq!(
            fs::read_to_string(record_path(repo.path())).expect("committed record after"),
            before,
            "the refused `{verb}` left the committed record byte-identical",
        );
        assert!(
            !staged_record(repo.path()).exists(),
            "the refused `{verb}` must create NO staged milestone-record copy in the task area",
        );
    }
}

/// (2) The READ verbs stay open: `doc show` and `doc schema` on the same record still
/// exit 0 — the record is a managed doctype, read like any other.
#[test]
fn the_doc_read_verbs_stay_open_on_a_milestone_record() {
    let repo = TempDir::new("read");
    let home = TempDir::new("read-home");
    init_repo(repo.path());
    setup(repo.path(), home.path());

    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "show", "milestone-record:cache-rework"],
        ),
        "jigc doc show on a milestone-record",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "schema", "milestone-record"],
        ),
        "jigc doc schema on a milestone-record",
    );
}
