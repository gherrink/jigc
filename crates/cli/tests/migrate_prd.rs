//! M25 Increment 6, T2 — end-to-end prd migration acceptance (the deliverable's
//! Proves). Drives the **built** `jigc` binary against throwaway `git init` temp repos
//! over the **shipped** dev pack (`JIGC_PACK_DIR` = the embedded `pack/` tree),
//! exercising — not assuming — the doctype-general migrate → author → review-gate →
//! retire → adopt spine on the `prd` doctype, whose shape difference from `adr`/`spec` is
//! the **repeatable `requirements`** (per-requirement `title` + `statement` slot, the
//! post-inc-5 schema) with NO code-anchor and NO date field
//! ([auto-migration.md](../../../design/auto-migration.md) → Per-doctype migration
//! workflows + Honest bounds (prd); [worked-examples.md](../../../design/worked-examples.md)
//! → flow 27; [roadmap.md](../../../implementation/roadmap.md) → M25 Increment 6).
//!
//! **The prd arm is honestly weaker** — there is no real foreign-corpus PRD standard, so
//! the fixture below is **synthesized and labeled synthetic** (the measured-on-real-corpus
//! claim is the spec/adr arms'; `auto-migration.md` → Honest bounds (prd)).
//!
//!   - **Headline** — a synthesized-and-labeled-synthetic foreign prd (committed,
//!     off-canonical at `docs/<x>.md`) migrates: `jigc migrate <path> --as prd` mints the
//!     per-file migration task, the agent authors the canonical record through `doc author
//!     prd --from-file -` (the vision/context slots + ≥2 repeatable `requirements` items, each a
//!     `title` + a `statement` slot), and `task finalize --approve` lands exactly ONE
//!     commit that (a) writes `docs/prds/<slug>.md` byte-stable
//!     (`render(instance_from_source(x)) == x`), (b) carries `A docs/prds/<slug>.md` + `D
//!     <foreign>` (the foreign original retired/gone), and (c) a follow-up `jigc ingest`
//!     reports the managed prd **adopted**. The review gate blocks the bare finalize first
//!     (nothing committed). The headline runs into an **empty `docs/prds/`** — the cold-spike
//!     arm.
//!   - **Guidance skeleton** — the shipped `migrate-prd` guidance's emitted `doc author`
//!     payload skeleton parses as a valid prd author payload (the agent-facing artifact is
//!     the contract; a skeleton the parser rejects would silently break every per-guidance
//!     migration).

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
            "jigc-migrate-prd-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::write(root.join("README.md"), "hello\n").expect("write file");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
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

/// The shipped `prd` schema, loaded for the byte-stable round-trip + the guidance skeleton
/// cross-check. `prd` carries NO pack `code-anchor` type (its `requirements` drops the
/// code-anchor that `spec.criteria` carries) and no date field, so it loads bare.
fn shipped_prd_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("prd.yaml")).expect("read shipped prd schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped prd schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The per-file migration task id for a repo-relative source `rel` — the production
/// derivation ([`crate::start::mint_migration_in_repo`]): strip `.md`, fold path
/// separators to `-`, slugify, prefix `migrate-<doctype>-`, then re-slugify at the
/// mint (which applies the word cap, so a long path yields a capped id).
fn migration_task(rel: &str) -> String {
    let stem = rel.strip_suffix(".md").unwrap_or(rel);
    let folded: String = stem
        .chars()
        .map(|c| if c == '/' { '-' } else { c })
        .collect();
    let source = format!("migrate-prd-{}", engine::slug::slugify(&folded));
    engine::slug::slugify(&source)
}

/// Write a foreign prd at `docs/<stem>.md` and **commit it**, so the retire surfaces a
/// tracked deletion in the finalize commit (the realistic flow — a pre-existing committed
/// prd at an off-canonical path).
fn commit_foreign_prd(repo: &Path, stem: &str, body: &str) -> String {
    let rel = format!("docs/{stem}.md");
    let path = repo.join(&rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create docs/");
    fs::write(&path, body).expect("write foreign prd");
    git(repo, &["add", &rel]);
    git(repo, &["commit", "-q", "-m", "track foreign prd"]);
    rel
}

/// `jigc migrate <rel> --as prd` — returns the composed migration workflow's stdout
/// (which surfaces the foreign content through the source seam and the author guidance).
fn migrate(repo: &Path, home: &Path, pack: &Path, rel: &str) -> String {
    ok_stdout(
        run_jigc(repo, home, pack, &["migrate", rel, "--as", "prd"], None),
        "jigc migrate <prd> --as prd",
    )
}

/// `jigc doc author prd --from-file -` over `task`, piping the declarative `payload`.
fn author_prd(
    repo: &Path,
    home: &Path,
    pack: &Path,
    task: &str,
    payload: &str,
) -> std::process::Output {
    run_jigc(
        repo,
        home,
        pack,
        &["doc", "author", "prd", "--from-file", "-", "--task", task],
        Some(payload.as_bytes()),
    )
}

/// Assert the canonical `docs/prds/<slug>.md` on disk round-trips byte-stable through the
/// shipped prd schema: `render(&schema, &instance_from_source(&schema, x)) == x`.
fn assert_committed_byte_stable(repo: &Path, pack: &Path, slug: &str) -> String {
    let on_disk = fs::read_to_string(repo.join("docs").join("prds").join(format!("{slug}.md")))
        .expect("the canonical prd is on disk");
    let schema = shipped_prd_schema(pack);
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed prd re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated prd is byte-stable across parse -> render:\n{on_disk}",
    );
    on_disk
}

/// Assert a follow-up `jigc ingest` reports `docs/prds/<slug>.md` adopted — never `unmanaged` /
/// `needs-reconcile`.
fn assert_ingest_adopted(repo: &Path, home: &Path, pack: &Path, slug: &str) {
    let ingest = ok_stdout(run_jigc(repo, home, pack, &["ingest"], None), "jigc ingest");
    let needle = format!("docs/prds/{slug}.md");
    let row = ingest
        .lines()
        .find(|l| l.contains(&needle))
        .unwrap_or_else(|| panic!("ingest must report the managed prd:\n{ingest}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the managed prd ingests as adopted:\n{row}"
    );
    assert!(
        !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed prd is neither unmanaged nor needs-reconcile:\n{row}"
    );
}

/// A **synthesized, labeled-synthetic** foreign prd — there is no real foreign-corpus PRD
/// standard, so this is hand-built to a plausible idiosyncratic shape (free-form Vision /
/// Requirements / Background headings, a bullet list of requirements). It is NOT a measured
/// real-corpus sample; the real-corpus claim lives on the spec/adr arms
/// (`auto-migration.md` → Honest bounds (prd)).
const FOREIGN_PRD: &str = "\
# Habit Tracker PRD

## Vision

A mobile app that turns good intentions into daily streaks.

## Requirements

- A user can log a habit with a single tap from the home screen.
- The current streak is shown front and center.

## Background

Existing planners are too heavyweight; solo users abandon them within a week.
";

/// The canonical rewrite payload — the vision/context fixed slots + a `requirements`
/// section carrying TWO `items`, each a `title` + a `statement` slot (the post-inc-5
/// repeatable shape). The title slugs to `habit-tracker`, the canonical path
/// `docs/prds/habit-tracker.md`.
const PAYLOAD_PRD: &str = r#"title: "Habit tracker"
sections:
  - id: vision
    set:
      vision: "<<A tracker that turns intentions into daily streaks.>>"
  - id: requirements
    items:
      - title: "Log a habit in one tap"
        set:
          statement: "<<Logging a habit takes a single tap from the home screen.>>"
      - title: "Show the current streak"
        set:
          statement: "<<The current streak is shown front and center.>>"
  - id: context
    set:
      context: "<<Built for solo users who abandon heavyweight planners.>>"
"#;

#[test]
fn migrate_prd_finalize_writes_retires_and_adopts() {
    let repo = TempDir::new("headline");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed foreign prd at the off-canonical `docs/` path; `docs/prds/` is empty (the cold
    // spike — migrate into a never-before-populated doctype location).
    let rel = commit_foreign_prd(repo.path(), "habit-tracker-prd", FOREIGN_PRD);
    assert!(
        !repo.path().join("docs").join("prds").exists(),
        "the cold spike starts with no `docs/prds/` directory"
    );

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let composed = migrate(repo.path(), home.path(), &pack, &rel);
    assert!(
        composed.contains("log a habit with a single tap"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );

    let task = migration_task(&rel);
    let authored = ok_stdout(
        author_prd(repo.path(), home.path(), &pack, &task, PAYLOAD_PRD),
        "jigc doc author prd",
    );
    assert_eq!(
        authored, "prd:habit-tracker",
        "the batch verb prints the minted per-title-slug address",
    );

    // The review gate blocks the bare finalize: nothing committed, nothing written.
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let bare = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task],
        None,
    );
    assert!(
        !bare.status.success(),
        "a migration finalize without --approve must block (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&bare.stdout),
        String::from_utf8_lossy(&bare.stderr),
    );
    assert_eq!(
        count_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the review-gate block commits nothing",
    );
    assert!(
        repo.path().join(&rel).exists(),
        "the review-gate block leaves the foreign original untouched"
    );

    // --approve: write the canonical doc, retire the foreign original, commit, adopt.
    let approve = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve on a conformant prd migration must land (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );

    // Exactly ONE new commit (the all-or-nothing transaction).
    let count_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count_after,
        count_before + 1,
        "the approved migration lands exactly ONE commit"
    );

    // (a) The canonical doc is committed at its per-title-slug path + byte-stable, and the
    // two requirements authored as repeatable items re-parse.
    let body = assert_committed_byte_stable(repo.path(), &pack, "habit-tracker");
    assert!(
        body.contains("A tracker that turns intentions into daily streaks")
            && body.contains("Logging a habit takes a single tap"),
        "the committed prd carries the authored vision + requirement prose:\n{body}",
    );
    let schema = shipped_prd_schema(&pack);
    let doc = engine::parse::parse_sections(&schema, &body).expect("committed prd re-parses");
    let reqs = doc
        .sections
        .iter()
        .find(|s| s.id == "requirements")
        .expect("requirements section present");
    assert_eq!(
        reqs.items.len(),
        2,
        "exactly the two migrated requirements re-parse as repeatable items; got:\n{body}"
    );

    // (b) The foreign original is GONE, and the SAME commit carries its deletion + the
    // added managed doc. `--no-renames` keeps the similar prose from collapsing the
    // delete+add into a single `R`.
    assert!(
        !repo.path().join(&rel).exists(),
        "the approved migration retires the foreign original from disk"
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains(&format!("D\t{rel}")),
        "the finalize commit carries the foreign deletion:\n{name_status}"
    );
    assert!(
        name_status.contains("A\tdocs/prds/habit-tracker.md"),
        "the SAME commit carries the added managed prd:\n{name_status}"
    );

    // (c) A follow-up ingest reports it adopted.
    assert_ingest_adopted(repo.path(), home.path(), &pack, "habit-tracker");
}

/// Extract the `doc author prd --from-file -` heredoc payload skeleton from the composed
/// migrate guidance (between the `<<'EOF'` opener and the standalone `EOF` terminator) —
/// the agent-facing artifact the LLM fills + pipes.
fn extract_author_skeleton(composed: &str) -> String {
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains("doc author prd --from-file") && line.contains("<<'EOF'") {
            break;
        }
    }
    let mut body = String::new();
    for line in lines {
        if line == "EOF" {
            return body;
        }
        body.push_str(line);
        body.push('\n');
    }
    panic!("no `doc author prd` heredoc skeleton in the composed migrate guidance:\n{composed}");
}

#[test]
fn shipped_guidance_payload_skeleton_parses_as_a_valid_prd_payload() {
    let repo = TempDir::new("guidance");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_prd(repo.path(), "habit-tracker-prd", FOREIGN_PRD);
    let composed = migrate(repo.path(), home.path(), &pack, &rel);

    // The skeleton the agent actually fills must parse against the SAME payload parser
    // `doc author` runs — an undeclared field (rejected by deny_unknown_fields) would
    // silently break every per-guidance migration with a green spine test masking it.
    let skeleton = extract_author_skeleton(&composed);
    let schema = shipped_prd_schema(&pack);
    let parsed = cli::author::parse_author_payload(Some(&schema), &skeleton);
    assert!(
        parsed.is_ok(),
        "the shipped migrate-prd guidance payload skeleton must parse as a valid prd \
         author payload; skeleton:\n{skeleton}\nerror: {:?}",
        parsed.err(),
    );
}
