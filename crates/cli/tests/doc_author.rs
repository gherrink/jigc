//! M24 Increment 1, T2 — `jigc doc author <doctype> --from <payload>`: the
//! declarative whole-doc batch verb, cold-authored through the real `jigc` binary.
//!
//! The headline B1 claim is that the batch path (create + N leaves over ONE
//! in-memory buffer, persisted once) produces the **same bytes** as the per-leaf
//! verb chain M23 proved byte-stable — so it inherits byte-stability, the
//! create-gate, and the squatter seam unchanged. This test drives the **emitted**
//! staged file both ways over the SHIPPED dev pack (`JIGC_PACK_DIR` = the embedded
//! `pack/` tree) and asserts, on a multi-release DATED changelog:
//!   (a) the batch-authored doc round-trips byte-stable (`render(parse(x)) == x`);
//!   (b) it is byte-identical to the equivalent per-leaf `create`/`add-item`/
//!       `set-field`/`set-slot` chain.
//!
//! Atomicity, the create-gate-through-the-batch, and the cold/empty spike are T3/T4.

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
            "jigc-doc-author-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["config", "user.email", "test@example.com"]);
    git(&["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`.
fn run_jigc(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both
/// streams.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The staged `changelog:changelog` instance in the task working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The shipped changelog schema, for the byte-stable round-trip assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    engine::schema::load_schema(&yaml).expect("shipped changelog schema loads")
}

/// The multi-release DATED changelog payload. Document order: release `1.2.0` (with
/// the optional `link` field + two nested change-groups), then release `1.1.0` (no
/// link, one nested group). The `<<…>>`-wrapped values are slot prose; the bare
/// scalar is the inline `link` field.
const PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        set:
          link: https://example.com/compare/1.1.0...1.2.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- OAuth device-code flow.>>"
              - title: Fixed
                set:
                  notes: "<<- Session fixation on logout.>>"
      - title: 1.1.0
        sections:
          - id: changes
            items:
              - title: Changed
                set:
                  notes: "<<- Default timeout raised to 30s.>>"
"#;

/// Bring a repo to a record-change task ready for changelog authoring (the gate
/// admits `changelog`). Returns the task id.
fn ready_repo(repo: &Path, home: &Path, pack: &Path, intent: &str) -> String {
    init_repo(repo);
    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["start", "--workflow", "record-change", intent],
            None,
        ),
        "jigc start --workflow record-change",
    );
    intent.replace(' ', "-")
}

/// Author the changelog in ONE `doc author` call from the declarative payload (stdin).
fn author_via_batch(repo: &Path, home: &Path, pack: &Path) -> String {
    let task = ready_repo(repo, home, pack, "author batch");
    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "author", "changelog", "--from", "-"],
            Some(PAYLOAD.as_bytes()),
        ),
        "jigc doc author changelog",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the batch verb prints the minted singleton address",
    );
    staged_changelog(repo, &task)
}

/// Author the SAME changelog through the per-leaf verb chain, in the exact document
/// order the payload parser lowers to — driving the EMITTED add-item addresses
/// verbatim downstream (never a reconstructed form).
fn author_via_leaf_chain(repo: &Path, home: &Path, pack: &Path) -> String {
    let task = ready_repo(repo, home, pack, "author chain");
    let run = |args: &[&str], stdin: Option<&[u8]>, what: &str| {
        ok_stdout(run_jigc(repo, home, pack, args, stdin), what)
    };

    assert_eq!(
        run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
            "create"
        ),
        "changelog:changelog",
    );

    // Release 1.2.0 + its optional link.
    let r120 = run(
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.2.0",
        ],
        None,
        "add 1.2.0",
    );
    run(
        &[
            "doc",
            "set-field",
            &format!("{r120}/link"),
            "--value",
            "https://example.com/compare/1.1.0...1.2.0",
        ],
        None,
        "set link",
    );
    // 1.2.0 nested change-groups, in order.
    let added = run(
        &[
            "doc",
            "add-item",
            &format!("{r120}/changes"),
            "--title",
            "Added",
        ],
        None,
        "add Added",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{added}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- OAuth device-code flow."),
        "notes Added",
    );
    let fixed = run(
        &[
            "doc",
            "add-item",
            &format!("{r120}/changes"),
            "--title",
            "Fixed",
        ],
        None,
        "add Fixed",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{fixed}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- Session fixation on logout."),
        "notes Fixed",
    );

    // Release 1.1.0 (no link) + its single change-group.
    let r110 = run(
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.1.0",
        ],
        None,
        "add 1.1.0",
    );
    let changed = run(
        &[
            "doc",
            "add-item",
            &format!("{r110}/changes"),
            "--title",
            "Changed",
        ],
        None,
        "add Changed",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{changed}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- Default timeout raised to 30s."),
        "notes Changed",
    );

    staged_changelog(repo, &task)
}

#[test]
fn batch_author_round_trips_and_matches_the_per_leaf_chain() {
    let home = TempDir::new("home");
    let pack = dev_pack();

    let batch_repo = TempDir::new("batch");
    let batch = author_via_batch(batch_repo.path(), home.path(), &pack);

    let chain_repo = TempDir::new("chain");
    let chain = author_via_leaf_chain(chain_repo.path(), home.path(), &pack);

    // (b) The batch-authored bytes are byte-identical to the per-leaf chain — the
    // load-bearing B1 claim (chain-the-primitives-minus-intermediate-persists).
    assert_eq!(
        batch, chain,
        "the batch-authored changelog is byte-identical to the per-leaf verb chain\n\
         batch:\n{batch}\n---\nchain:\n{chain}",
    );

    // The authored content actually landed (not two identically-empty docs).
    assert!(
        batch.contains("1.2.0")
            && batch.contains("1.1.0")
            && batch.contains("OAuth device-code flow."),
        "the multi-release dated content is authored; staged:\n{batch}",
    );

    // (a) Byte-stable: render(parse(batch)) == batch.
    let schema = shipped_changelog_schema(&pack);
    let parsed =
        engine::write::instance_from_source(&schema, &batch).expect("batch changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        batch,
        "the batch-authored changelog is byte-stable across parse → render",
    );
}
