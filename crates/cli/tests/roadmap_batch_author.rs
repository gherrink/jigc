//! M43 Increment 7 / T6 — B15: the fresh-authoring steps model `doc author` for
//! many-leaf docs (`design/surface-contract.md` → law 2; RC-lacon B15;
//! `ideas/batch-authoring-ergonomics.md` — the trigger fired: the third session
//! defaulted to N per-leaf calls *because that's what the workflow text models*).
//!
//! Two claims, both driven through the real binary over the on-disk methodology
//! pack:
//!
//! 1. **The composed planning workflow names `doc author`** — the batch
//!    alternative line for each of the three running singletons composes with the
//!    task id resolved (the emitted bytes, never a reconstructed form).
//! 2. **The modeled story is true, not prose** — an items-only `doc author`
//!    payload over a **committed** roadmap copies the committed doc in and
//!    appends the new milestone without disturbing existing entries. The batch
//!    invocation is the composed `Run` line extracted from the planning output
//!    and executed verbatim.

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
            "jigc-roadmap-batch-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
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

/// Initialize a real git repo with one commit (composition mints, which reads HEAD).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` over the methodology pack, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", methodology_pack_tree())
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

/// The trimmed stdout of a successful invocation, panicking with both streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The committed bytes of `path` at HEAD.
fn committed(repo: &Path, path: &str) -> String {
    let out = Command::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        out.status.success(),
        "{path} must be committed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed bytes")
}

/// The staged bytes of a managed doc in the task working area.
fn staged(repo: &Path, task: &str, doc_file: &str) -> String {
    fs::read_to_string(
        repo.join(".jigc")
            .join("tasks")
            .join(task)
            .join("docs")
            .join(doc_file),
    )
    .unwrap_or_else(|e| panic!("read staged {doc_file} for task {task}: {e}"))
}

/// Seed the repo with a COMMITTED one-milestone roadmap through a first planning
/// task: create the singleton, per-leaf author the `M-Alpha` entry, fill the
/// commit doc, finalize. Returns nothing — the committed `docs/roadmap.md` is the
/// fixture the batch task edits from.
fn commit_one_milestone_roadmap(repo: &Path, home: &Path) {
    ok_stdout(
        jigc(
            repo,
            home,
            &["start", "--workflow", "planning", "M-Alpha"],
            None,
        ),
        "`jigc start --workflow planning M-Alpha`",
    );
    ok_stdout(
        jigc(
            repo,
            home,
            &["doc", "create", "roadmap", "--title", "Roadmap"],
            None,
        ),
        "`doc create roadmap`",
    );
    let item = ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "add-item",
                "roadmap:roadmap#milestones",
                "--title",
                "M-Alpha",
            ],
            None,
        ),
        "`doc add-item roadmap#milestones`",
    );
    ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("{item}/proves"),
                "--from-file",
                "-",
            ],
            Some(b"M-Alpha proves the per-leaf seed.\n"),
        ),
        "`set-slot <alpha>/proves`",
    );
    ok_stdout(
        jigc(
            repo,
            home,
            &[
                "doc",
                "set-slot",
                &format!("{item}/decomposition"),
                "--from-file",
                "-",
            ],
            Some(b"Inc 1: the alpha increment, as prose.\n"),
        ),
        "`set-slot <alpha>/decomposition`",
    );
    // Fill the commit doc so finalize validates clean, then finalize.
    for (leaf, value) in [("type", "docs"), ("scope", "planning")] {
        ok_stdout(
            jigc(
                repo,
                home,
                &[
                    "doc",
                    "set-field",
                    &format!("commit:m-alpha#{leaf}"),
                    "--value",
                    value,
                ],
                None,
            ),
            "`set-field commit`",
        );
    }
    for (leaf, prose) in [
        ("summary", &b"plan the alpha milestone\n"[..]),
        ("body", &b"The alpha planning pass.\n"[..]),
    ] {
        ok_stdout(
            jigc(
                repo,
                home,
                &[
                    "doc",
                    "set-slot",
                    &format!("commit:m-alpha#{leaf}"),
                    "--from-file",
                    "-",
                ],
                Some(prose),
            ),
            "`set-slot commit`",
        );
    }
    ok_stdout(
        jigc(repo, home, &["task", "finalize", "m-alpha"], None),
        "`task finalize m-alpha`",
    );
}

/// The items-only batch payload: ONE new milestone entry, both slots, no
/// section-level `set` — the trial's exact append shape.
const ITEMS_ONLY_PAYLOAD: &str = r#"title: Roadmap
sections:
  - id: milestones
    items:
      - title: M-Beta
        set:
          proves: "<<M-Beta proves the items-only batch append.>>"
          decomposition: "<<Inc 1: the beta increment, authored in one call.>>"
"#;

/// The shipped methodology roadmap schema (stamped), for the byte-stability check.
fn stamped_roadmap_schema() -> engine::schema::Schema {
    let yaml = fs::read(methodology_pack_tree().join("schemas").join("roadmap.yaml"))
        .expect("read shipped roadmap schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped roadmap schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

#[test]
fn composed_batch_line_appends_a_milestone_to_the_committed_roadmap() {
    let repo = TempDir::new("append");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok_stdout(
        jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );

    // Seed: a committed one-milestone roadmap at docs/roadmap.md.
    commit_one_milestone_roadmap(repo.path(), home.path());
    let before = committed(repo.path(), "docs/roadmap.md");
    assert!(
        before.contains("M-Alpha proves the per-leaf seed.") && !before.contains("M-Beta"),
        "the seed roadmap carries exactly the alpha milestone; got:\n{before}",
    );

    // A second planning task: the composed output must NAME the batch alternative
    // for each of the three running singletons (law 2 — the affordance is named by
    // the surface that solicits the many-leaf authoring), with the task id resolved.
    let composed = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "planning", "M-Beta"],
            None,
        ),
        "`jigc start --workflow planning M-Beta`",
    );
    for needle in [
        "jigc doc author roadmap --from-file - --task m-beta",
        "jigc doc author deferral-ledger --from-file - --task m-beta",
        "jigc doc author decisions-log --from-file - --task m-beta",
    ] {
        assert!(
            composed.contains(needle),
            "the composed planning workflow must name the batch alternative `{needle}`; got:\n{composed}",
        );
    }

    // Extract the roadmap batch line from the COMPOSED bytes and run it verbatim
    // (never a reconstructed equivalent) with the items-only payload on stdin.
    let line = composed
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("jigc doc author roadmap"))
        .expect("the composed planning output carries the roadmap batch line");
    let args: Vec<&str> = line.split_whitespace().skip(1).collect(); // drop the leading `jigc`
    let ack = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &args,
            Some(ITEMS_ONLY_PAYLOAD.as_bytes()),
        ),
        "the composed `doc author roadmap` line, run verbatim",
    );
    assert_eq!(
        ack, "roadmap:roadmap",
        "the batch verb acks the singleton address",
    );

    // The modeled story, proven on the staged bytes: the committed roadmap was
    // copied in and the new milestone APPENDED — every committed byte survives as
    // a prefix (existing entries undisturbed), and both authored slots landed.
    let after = staged(repo.path(), "m-beta", "roadmap:roadmap.md");
    assert!(
        after.starts_with(&before),
        "the items-only batch append must leave existing entries byte-undisturbed \
         (the committed roadmap is a prefix of the staged result)\n\
         committed:\n{before}\n---\nstaged:\n{after}",
    );
    assert!(
        after.len() > before.len()
            && after.contains("M-Beta proves the items-only batch append.")
            && after.contains("Inc 1: the beta increment, authored in one call."),
        "the new milestone entry and both its slots are appended; staged:\n{after}",
    );

    // Byte-stable across parse → render over the shipped stamped schema.
    let schema = stamped_roadmap_schema();
    let parsed =
        engine::write::instance_from_source(&schema, &after).expect("staged roadmap re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        after,
        "the batch-appended roadmap is byte-stable across parse → render",
    );
}
