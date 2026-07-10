//! M40 Increment 7, T2 — the design-altitude trio's migrate workflows
//! (`migrate-vision` / `migrate-idea` / `migrate-research`), end-to-end through the
//! **built** `jigc` binary over the `[dev ▸ methodology]` composition (methodology pack
//! listed in `packs.yaml` over the embedded dev base — the `flow_form_vision.rs`
//! harness shape). Pure pack YAML rides the proven verb + source seam + review gate +
//! retire spine ([auto-migration.md](../../../design/auto-migration.md) → Generalizing
//! to the methodology work docs (M40)).
//!
//!   - **Headline (the increment's first Proves arm)** — a committed foreign vision
//!     document at `old-vision.md` migrates: `jigc migrate old-vision.md --as vision`
//!     mints the off-router task and surfaces the foreign bytes through the source
//!     seam, the agent authors the canonical record through `doc author vision
//!     --from-file -` (the fixed-slug singleton `vision:vision`, `grounded-in`
//!     OMITTED per the template — a foreign vision has no managed research target),
//!     the bare finalize blocks at the review gate (exit **4**, nothing committed),
//!     and `task finalize --approve` lands exactly ONE commit that (a) writes the
//!     placement-singleton home — the repo-root literal `VISION.md`, H1 `# Vision`,
//!     byte-stable — (b) retires the foreign original in the SAME commit, and (c) a
//!     follow-up `jigc ingest` reports the managed vision **adopted**.
//!   - **Guidance skeletons** — each of the three shipped author-guidance steps emits
//!     a `doc author <doctype> --from-file -` heredoc payload skeleton that parses
//!     against the SAME payload parser `doc author` runs (the agent-facing emitted
//!     artifact is the contract; a skeleton the parser rejects would silently break
//!     every per-guidance migration — the masking-test lesson).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-migrate-methodology-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`) — the literal
/// directory a `.jigc/config/packs.yaml` entry names.
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
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

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer,
/// then record the methodology pack in `packs.yaml` (listed over the embedded dev
/// base: the `[dev ▸ methodology]` composition).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
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

/// A shipped methodology schema, loaded + version-stamped for the byte-stable
/// round-trip and the skeleton cross-check.
fn methodology_schema(doctype: &str) -> engine::schema::Schema {
    let yaml = fs::read(
        methodology_pack_tree()
            .join("schemas")
            .join(format!("{doctype}.yaml")),
    )
    .expect("read shipped methodology schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped methodology schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// Extract the `doc author <doctype> --from-file -` heredoc payload skeleton from the
/// composed migrate guidance (between the `<<'EOF'` opener and the standalone `EOF`
/// terminator) — the agent-facing artifact the LLM fills + pipes.
fn extract_author_skeleton(composed: &str, doctype: &str) -> String {
    let opener = format!("doc author {doctype} --from-file");
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains(&opener) && line.contains("<<'EOF'") {
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
    panic!(
        "no `doc author {doctype}` heredoc skeleton in the composed migrate guidance:\n{composed}"
    );
}

/// A plausible foreign vision/charter document — free-form headings (thesis prose
/// under the H1, a Principles list, a Someday section, a research pointer with no
/// managed target).
const FOREIGN_VISION: &str = "\
# Where dashbard is going

Dashbard turns messy spreadsheets into live dashboards nobody has to babysit.

## Principles

- The importer never mutates the source spreadsheet.
- Every widget is derived from a named query, not ad-hoc SQL.

## Someday

- A plugin marketplace, once the widget API stabilizes.

## Research behind this

See the notes in research/notes.md (never adopted through any jigc verb).
";

/// The canonical rewrite payload — the fixed singleton `title: Vision`, the three
/// prose slots, and `grounded-in` OMITTED (the template's rule: a foreign vision has
/// no managed `research` target, so the ref would dangle at finalize). The foreign
/// research pointer folds into `open-questions` prose (the surplus rule).
const PAYLOAD_VISION: &str = r#"title: Vision
sections:
  - id: thesis
    set:
      thesis: "<<Dashbard turns messy spreadsheets into live dashboards nobody has to babysit.>>"
  - id: invariants
    set:
      invariants: |-
        <<- The importer never mutates the source spreadsheet.
        - Every widget is derived from a named query, not ad-hoc SQL.>>
  - id: open-questions
    set:
      open-questions: |-
        <<- A plugin marketplace, once the widget API stabilizes.
        - The spreadsheet-pain notes (research/notes.md) are not yet managed research.>>
"#;

/// The per-file migration task id `jigc migrate old-vision.md --as vision` mints
/// (the production derivation: strip `.md`, fold `/` to `-`, slugify, prefix
/// `migrate-vision-`).
const TASK: &str = "migrate-vision-old-vision";

#[test]
fn migrate_vision_finalize_writes_root_vision_retires_and_adopts() {
    let repo = TempDir::new("headline");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // A committed foreign vision at the off-canonical root `old-vision.md`; no managed
    // `VISION.md` exists yet (the placement-singleton cold spike).
    fs::write(repo.path().join("old-vision.md"), FOREIGN_VISION).expect("write foreign vision");
    git(repo.path(), &["add", "old-vision.md"]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign vision"]);
    assert!(
        !repo.path().join("VISION.md").exists(),
        "the cold spike starts with no managed VISION.md"
    );

    let composed = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["migrate", "old-vision.md", "--as", "vision"],
            None,
        ),
        "jigc migrate old-vision.md --as vision",
    );
    assert!(
        composed.contains("live dashboards nobody has to babysit"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );
    assert!(
        composed.contains("doc author vision --from-file"),
        "the composed guidance carries the batch author verb; stdout:\n{composed}",
    );

    let authored = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "author",
                "vision",
                "--from-file",
                "-",
                "--task",
                TASK,
            ],
            Some(PAYLOAD_VISION.as_bytes()),
        ),
        "jigc doc author vision",
    );
    assert_eq!(
        authored, "vision:vision",
        "the singleton mints at the fixed slug = the type id",
    );

    // The review gate blocks the bare finalize: exit 4 (review-pending), nothing
    // committed, nothing written, the foreign original untouched.
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let bare = jigc(repo.path(), home.path(), &["task", "finalize", TASK], None);
    assert_eq!(
        bare.status.code(),
        Some(4),
        "finalize without --approve on a migration task must exit 4 (review-pending); \
         stdout:\n{}\nstderr:\n{}",
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
        !repo.path().join("VISION.md").exists(),
        "the review-gate block writes no VISION.md"
    );
    assert!(
        repo.path().join("old-vision.md").exists(),
        "the review-gate block leaves the foreign original untouched"
    );

    // --approve: write the placement-singleton home, retire the foreign original,
    // commit, adopt.
    let approve = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve on a conformant vision migration must land (exit 0); \
         stdout:\n{}\nstderr:\n{}",
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

    // (a) The managed vision is committed at the repo-root literal `VISION.md` (the
    // placement knob — no `docs/`-buried `<location>/<slug>.md` home), its H1 reads
    // `# Vision` (display-title knob), and it round-trips byte-stable.
    let on_disk = fs::read_to_string(repo.path().join("VISION.md"))
        .expect("the managed VISION.md is on disk at the repo root");
    assert!(
        on_disk.lines().any(|l| l.trim() == "# Vision"),
        "the managed vision's H1 reads `# Vision`; got:\n{on_disk}",
    );
    assert!(
        on_disk.contains("live dashboards nobody has to babysit")
            && on_disk.contains("never mutates the source spreadsheet"),
        "the committed vision carries the authored thesis + invariants prose:\n{on_disk}",
    );
    assert!(
        !on_disk.contains("grounded-in"),
        "the template's OMIT rule holds — no dangling `grounded-in` ref is authored:\n{on_disk}",
    );
    let schema = methodology_schema("vision");
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed vision re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated vision is byte-stable across parse -> render:\n{on_disk}",
    );

    // (b) The foreign original is GONE, and the SAME commit carries its deletion +
    // the added managed vision. `--no-renames` keeps the similar prose from collapsing
    // the delete+add into a single `R`.
    assert!(
        !repo.path().join("old-vision.md").exists(),
        "the approved migration retires the foreign original from disk"
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains("D\told-vision.md"),
        "the finalize commit carries the foreign deletion:\n{name_status}"
    );
    assert!(
        name_status.contains("A\tVISION.md"),
        "the SAME commit carries the added managed vision at the repo root:\n{name_status}"
    );

    // (c) A follow-up ingest reports the managed vision adopted — never `unmanaged` /
    // `needs-reconcile`.
    let ingest = ok_stdout(
        jigc(repo.path(), home.path(), &["ingest"], None),
        "jigc ingest",
    );
    let row = ingest
        .lines()
        .find(|l| l.contains("VISION.md"))
        .unwrap_or_else(|| panic!("ingest must report the managed vision:\n{ingest}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the managed vision ingests as adopted:\n{row}"
    );
    assert!(
        !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed vision is neither unmanaged nor needs-reconcile:\n{row}"
    );
}

/// Each of the three shipped guidance steps emits a payload skeleton that parses
/// against the SAME parser `doc author` runs — an undeclared key or a mis-marked
/// slot/field value (rejected by the parse-time cross-check) would silently break
/// every per-guidance migration with a green spine test masking it.
#[test]
fn shipped_guidance_payload_skeletons_parse_for_all_three_doctypes() {
    let repo = TempDir::new("skeletons");
    let home = TempDir::new("home");
    init_repo(repo.path());

    for (doctype, rel, foreign) in [
        ("vision", "old-vision.md", FOREIGN_VISION),
        (
            "idea",
            "old-idea.md",
            "# Offline mode\n\nWorth doing once sync stabilizes.\n",
        ),
        (
            "research",
            "old-notes.md",
            "# Spreadsheet pain notes\n\nUsers re-import hourly.\n",
        ),
    ] {
        fs::write(repo.path().join(rel), foreign).expect("write foreign file");
        let composed = ok_stdout(
            jigc(
                repo.path(),
                home.path(),
                &["migrate", rel, "--as", doctype],
                None,
            ),
            &format!("jigc migrate {rel} --as {doctype}"),
        );
        let skeleton = extract_author_skeleton(&composed, doctype);
        let schema = methodology_schema(doctype);
        let parsed = cli::author::parse_author_payload(Some(&schema), &skeleton);
        assert!(
            parsed.is_ok(),
            "the shipped migrate-{doctype} guidance payload skeleton must parse as a \
             valid {doctype} author payload; skeleton:\n{skeleton}\nerror: {:?}",
            parsed.err(),
        );
    }
}
