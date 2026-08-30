//! M40 Increment 7, T2+T3 — the methodology pack's seven migrate workflows
//! (`migrate-vision` / `migrate-idea` / `migrate-research` — the design-altitude trio,
//! T2 — plus `migrate-roadmap` / `migrate-decisions-log` / `migrate-deferral-ledger` /
//! `migrate-completion-record` — the GSD work-doc quartet, T3), end-to-end through the
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
//!   - **The verbatim-source rung + the floored gate (the increment's second Proves
//!     arm, T3)** — a GSD completion record with NO evidence file anywhere migrates via
//!     the artifact ladder's rung 3 (the verbatim foreign source IS the artifact,
//!     copied under `completions/artifacts/<milestone>/` BEFORE finalize retires the
//!     original), and M17's floored `owner-artifact` presence gate still BLOCKS an
//!     approved finalize when the rung is skipped — no migration carve-out.
//!   - **Historic dates transcribe-or-omit, per entry (T3)** — a dated foreign
//!     decisions-log entry carries its date verbatim into the committed record; a
//!     dateless one renders dateless (the M24 migration-mode suppression — never a
//!     fabricated today-stamp).
//!   - **The two-slot roadmap item payload (T3)** — one `milestones` item with both
//!     `proves` + `decomposition` in one `set:` map renders both `#### Proves` /
//!     `#### Decomposition` sub-headings.
//!   - **Guidance skeletons** — each of the seven shipped author-guidance steps emits
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
            engine::tempname::unique_nanos(),
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
const TASK: &str = "migrate-vision-old-vision-b7ea1b0697bf";

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

// ---------------------------------------------------------------------------
// T3 — the GSD work-doc quartet (roadmap / decisions-log / deferral-ledger /
// completion-record).
// ---------------------------------------------------------------------------

/// A GSD-style foreign completion record with NO evidence file anywhere in the repo —
/// the verbatim-source rung's motivating shape (the record itself is the milestone's
/// only surviving evidence).
const FOREIGN_COMPLETION: &str = "\
# M3 Completion — importer hardening

Status: DONE. All acceptance criteria met; audit closed clean.

## Findings

- Importer retries flaked in CI once; re-ran green (fixed).
";

/// The canonical completion-record rewrite: the milestone title mints the per-milestone
/// slug, the `meta` header carries the derived `verdict` + the rung-3 `owner-artifact`
/// path (the copied verbatim source), and the one foreign finding maps onto a
/// `findings` item — every `set:` value here is an inline FIELD (no `<<…>>`).
const PAYLOAD_COMPLETION: &str = r#"title: M3
sections:
  - id: meta
    set:
      verdict: green
      owner-artifact: completions/artifacts/M3/source.md
  - id: findings
    items:
      - title: "Importer retries flaked in CI"
        set:
          severity: advisory
          disposition: fixed
          evidence: "CI run 118 - re-ran green"
"#;

/// The per-file migration task id `jigc migrate M3-DONE.md --as completion-record`
/// mints (the production derivation: strip `.md`, fold `/` to `-`, slugify, prefix
/// `migrate-completion-record-`).
const TASK_COMPLETION: &str = "migrate-completion-record-m3-done-dc422cd75edf";

/// The increment's second Proves arm: a GSD completion record with NO evidence file
/// migrates via the artifact ladder's **verbatim-source rung** (the foreign source is
/// copied under `completions/artifacts/<milestone>/` and staged BEFORE finalize retires
/// the original) — and M17's floored `owner-artifact` presence gate still BLOCKS an
/// approved finalize while the rung is skipped (no migration carve-out).
#[test]
fn migrate_completion_record_verbatim_source_rung_and_the_floored_gate() {
    let repo = TempDir::new("completion");
    let home = TempDir::new("home");
    init_repo(repo.path());

    fs::write(repo.path().join("M3-DONE.md"), FOREIGN_COMPLETION)
        .expect("write foreign completion record");
    git(repo.path(), &["add", "M3-DONE.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign completion record"],
    );

    let composed = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["migrate", "M3-DONE.md", "--as", "completion-record"],
            None,
        ),
        "jigc migrate M3-DONE.md --as completion-record",
    );
    assert!(
        composed.contains("audit closed clean"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );
    assert!(
        composed.contains("doc author completion-record --from-file"),
        "the composed guidance carries the batch author verb; stdout:\n{composed}",
    );
    assert!(
        composed.contains("completions/artifacts/"),
        "the composed guidance names the owned artifact home the ladder resolves \
         `owner-artifact` into; stdout:\n{composed}",
    );

    // M43 Inc-3 T5 — the generated `{{schema:completion-record}}` projection: the
    // located per-milestone mint renders its resolved home (methodology locations
    // stay flat — the recorded docs-root divergence), and the tree spells out the
    // enum members + the findings repeatable the hand template restated.
    for line in [
        "The `completion-record` schema — each instance a managed file at \
         `completions/<slug>.md`, its `<slug>` minted from `title`.",
        "`verdict`: enum, one of: green | red — author-required",
        "`owner-artifact`: owned-location — author-required",
        "- `findings`: repeatable items, one per `title`:",
        "`severity`: enum, one of: blocking | advisory | HIGH | MEDIUM | LOW — author-required",
        "`disposition`: enum, one of: fixed | deferred | contested — author-required",
    ] {
        assert!(
            composed.contains(line),
            "the generated projection must render {line:?}; composed:\n{composed}",
        );
    }

    let authored = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "author",
                "completion-record",
                "--from-file",
                "-",
                "--task",
                TASK_COMPLETION,
            ],
            Some(PAYLOAD_COMPLETION.as_bytes()),
        ),
        "jigc doc author completion-record",
    );
    assert_eq!(
        authored, "completion-record:m3",
        "the per-milestone record mints its slug from the milestone title",
    );

    // The rung SKIPPED: `owner-artifact` names the ladder path but nothing was copied.
    // M17's floored presence gate blocks even an APPROVED finalize — exit 3
    // (validation-blocked), `owner-artifact.present`, nothing committed, the foreign
    // original untouched.
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let blocked = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK_COMPLETION, "--approve"],
        None,
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "an absent owner-artifact file must block the approved finalize with exit 3 \
         (the floored gate carries no migration carve-out); got:\n{rendered}",
    );
    assert!(
        rendered.contains("owner-artifact.present"),
        "the block surfaces the #5 presence gate's finding; got:\n{rendered}",
    );
    assert_eq!(
        count_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the gate-blocked finalize commits nothing",
    );
    assert!(
        repo.path().join("M3-DONE.md").exists(),
        "the gate-blocked finalize leaves the foreign original untouched",
    );

    // The verbatim-source rung: NO evidence file exists anywhere, so the foreign source
    // ITSELF is the artifact — copied under the owned home + staged BEFORE finalize
    // retires the original.
    fs::create_dir_all(repo.path().join("completions/artifacts/M3"))
        .expect("mk owned artifact home");
    fs::copy(
        repo.path().join("M3-DONE.md"),
        repo.path().join("completions/artifacts/M3/source.md"),
    )
    .expect("copy the verbatim foreign source under the owned home");
    git(repo.path(), &["add", "completions/artifacts/M3/source.md"]);

    let approve = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK_COMPLETION, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "with the verbatim source staged as the artifact, finalize --approve must land; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        count_before + 1,
        "the approved migration lands exactly ONE commit",
    );

    // The managed record is committed, byte-stable, and the artifact is the foreign
    // source VERBATIM — while the original is retired in the SAME commit.
    let on_disk = fs::read_to_string(repo.path().join("completions/m3.md"))
        .expect("the managed completion-record is on disk");
    assert!(
        on_disk.contains("Importer retries flaked in CI"),
        "the committed record carries the migrated finding:\n{on_disk}",
    );
    let schema = methodology_schema("completion-record");
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed completion-record re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated completion-record is byte-stable across parse -> render",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("completions/artifacts/M3/source.md"))
            .expect("the owner-artifact is on disk"),
        FOREIGN_COMPLETION,
        "the rung-3 artifact is the foreign source VERBATIM",
    );
    assert!(
        !repo.path().join("M3-DONE.md").exists(),
        "the approved migration retires the foreign original from disk",
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    for expected in [
        "D\tM3-DONE.md",
        "A\tcompletions/m3.md",
        "A\tcompletions/artifacts/M3/source.md",
    ] {
        assert!(
            name_status.contains(expected),
            "the ONE finalize commit must carry `{expected}`:\n{name_status}",
        );
    }
}

/// A foreign decisions log with one DATED entry and one DATELESS entry — the
/// historic-date TRANSCRIBE-or-OMIT rule's two arms in one file.
const FOREIGN_DECISIONS: &str = "\
# Decisions

## 2024-03-05 — Use Postgres

Because the ORM already speaks it.

## Drop the mobile app

Not enough users to justify it. No date was ever recorded.
";

/// The canonical rewrite: the dated entry TRANSCRIBES its historic date verbatim in a
/// per-entry `date` key; the dateless entry OMITS the key entirely (in migration the
/// CLI does not stamp today, so it renders dateless — no false history).
const PAYLOAD_DECISIONS: &str = r#"title: Decisions Log
sections:
  - id: entries
    items:
      - title: "Use Postgres"
        set:
          date: "2024-03-05"
          why: "<<Because the ORM already speaks it.>>"
      - title: "Drop the mobile app"
        set:
          why: "<<Not enough users to justify it.>>"
"#;

/// The task id `jigc migrate OLD-DECISIONS.md --as decisions-log` mints.
const TASK_DECISIONS: &str = "migrate-decisions-log-old-decisions-b3e221f3098b";

/// Historic dates transcribe-or-omit, PER ENTRY: the dated foreign entry carries
/// `2024-03-05` verbatim into the committed singleton; the dateless one renders with
/// NO date field at all — never a fabricated migration-day stamp (the M24 rule).
#[test]
fn migrate_decisions_log_transcribes_dates_verbatim_and_keeps_dateless_dateless() {
    let repo = TempDir::new("decisions");
    let home = TempDir::new("home");
    init_repo(repo.path());

    fs::write(repo.path().join("OLD-DECISIONS.md"), FOREIGN_DECISIONS)
        .expect("write foreign decisions log");
    git(repo.path(), &["add", "OLD-DECISIONS.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign decisions log"],
    );

    let composed = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["migrate", "OLD-DECISIONS.md", "--as", "decisions-log"],
            None,
        ),
        "jigc migrate OLD-DECISIONS.md --as decisions-log",
    );
    assert!(
        composed.contains("doc author decisions-log --from-file"),
        "the composed guidance carries the batch author verb; stdout:\n{composed}",
    );

    let authored = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "author",
                "decisions-log",
                "--from-file",
                "-",
                "--task",
                TASK_DECISIONS,
            ],
            Some(PAYLOAD_DECISIONS.as_bytes()),
        ),
        "jigc doc author decisions-log",
    );
    assert_eq!(
        authored, "decisions-log:decisions-log",
        "the singleton mints at the fixed slug = the type id",
    );

    let approve = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK_DECISIONS, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve on a conformant decisions-log migration must land; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );

    let on_disk = fs::read_to_string(repo.path().join("docs/decisions-log.md"))
        .expect("the managed decisions-log is on disk at its placement home");
    assert!(
        on_disk.contains("date: 2024-03-05"),
        "the dated entry transcribes its historic date verbatim:\n{on_disk}",
    );
    let schema = methodology_schema("decisions-log");
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed decisions-log re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated decisions-log is byte-stable across parse -> render",
    );

    let entries = parsed
        .sections
        .iter()
        .find(|s| s.id == "entries")
        .expect("entries section present");
    let dated = entries
        .items
        .iter()
        .find(|i| i.id == "use-postgres")
        .expect("the dated entry is present");
    assert!(
        dated.fields.iter().any(|f| f.key == "date"
            && matches!(&f.value, engine::field_block::Value::Scalar(v) if v == "2024-03-05")),
        "the dated entry's `date` field is the historic date verbatim; parsed: {dated:?}",
    );
    let dateless = entries
        .items
        .iter()
        .find(|i| i.id == "drop-the-mobile-app")
        .expect("the dateless entry is present");
    assert!(
        !dateless.fields.iter().any(|f| f.key == "date"),
        "the dateless foreign entry renders with NO date field — the migration-mode \
         on-create suppression, never a fabricated today-stamp; parsed: {dateless:?}",
    );
}

/// A foreign roadmap with one milestone — a goal statement and a step list, the
/// two-slot item payload's source shape.
const FOREIGN_ROADMAP: &str = "\
# Roadmap

## M1 — The importer

Goal: spreadsheets import without babysitting.

Steps: build the retry loop, then the resume path.
";

/// The canonical rewrite: ONE `milestones` item carrying BOTH slot keys (`proves` +
/// `decomposition`) in one `set:` map — the two-slot item payload spiked at planning.
const PAYLOAD_ROADMAP: &str = r#"title: Roadmap
sections:
  - id: milestones
    items:
      - title: "M1 - The importer"
        set:
          proves: "<<Spreadsheets import without babysitting.>>"
          decomposition: "<<One increment: the retry loop, then the resume path.>>"
"#;

/// The task id `jigc migrate OLD-ROADMAP.md --as roadmap` mints.
const TASK_ROADMAP: &str = "migrate-roadmap-old-roadmap-22f49ca94e73";

/// The two-slot item payload: one authored `milestones` item renders BOTH `#### Proves`
/// and `#### Decomposition` sub-headings in the committed placement singleton.
#[test]
fn migrate_roadmap_two_slot_item_renders_both_sub_headings() {
    let repo = TempDir::new("roadmap");
    let home = TempDir::new("home");
    init_repo(repo.path());

    fs::write(repo.path().join("OLD-ROADMAP.md"), FOREIGN_ROADMAP).expect("write foreign roadmap");
    git(repo.path(), &["add", "OLD-ROADMAP.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign roadmap"],
    );

    let composed = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &["migrate", "OLD-ROADMAP.md", "--as", "roadmap"],
            None,
        ),
        "jigc migrate OLD-ROADMAP.md --as roadmap",
    );
    assert!(
        composed.contains("doc author roadmap --from-file"),
        "the composed guidance carries the batch author verb; stdout:\n{composed}",
    );

    // M43 Inc-3 T5 — the generated `{{schema:roadmap}}` projection replaces the
    // hand-enumerated skeleton (law 1: a template cannot understate the schema).
    // The placement singleton's home renders at its LITERAL file (docs-root never
    // applies — the T1 placement branch), and the items tree names the repeatable
    // + both its slots.
    for line in [
        "The `roadmap` schema — the managed singleton at `docs/roadmap.md`.",
        "- `milestones`: repeatable items, one per `title`:",
        "- `proves`: prose slot",
        "- `decomposition`: prose slot",
    ] {
        assert!(
            composed.contains(line),
            "the generated projection must render {line:?}; composed:\n{composed}",
        );
    }

    let authored = ok_stdout(
        jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "author",
                "roadmap",
                "--from-file",
                "-",
                "--task",
                TASK_ROADMAP,
            ],
            Some(PAYLOAD_ROADMAP.as_bytes()),
        ),
        "jigc doc author roadmap",
    );
    assert_eq!(
        authored, "roadmap:roadmap",
        "the singleton mints at the fixed slug = the type id",
    );

    let approve = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", TASK_ROADMAP, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve on a conformant roadmap migration must land; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );

    let on_disk = fs::read_to_string(repo.path().join("docs/roadmap.md"))
        .expect("the managed roadmap is on disk at its placement home");
    assert!(
        on_disk.contains("#### Proves") && on_disk.contains("#### Decomposition"),
        "the two-slot item payload renders BOTH sub-headings:\n{on_disk}",
    );
    assert!(
        on_disk.contains("Spreadsheets import without babysitting")
            && on_disk.contains("the retry loop, then the resume path"),
        "the committed roadmap carries both authored slot proses:\n{on_disk}",
    );
    let schema = methodology_schema("roadmap");
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed roadmap re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated roadmap is byte-stable across parse -> render",
    );
    assert!(
        !repo.path().join("OLD-ROADMAP.md").exists(),
        "the approved migration retires the foreign original from disk",
    );
}

/// Each of the seven shipped guidance steps emits a payload skeleton that parses
/// against the SAME parser `doc author` runs — an undeclared key or a mis-marked
/// slot/field value (rejected by the parse-time cross-check) would silently break
/// every per-guidance migration with a green spine test masking it.
#[test]
fn shipped_guidance_payload_skeletons_parse_for_all_seven_doctypes() {
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
        ("roadmap", "OLD-ROADMAP.md", FOREIGN_ROADMAP),
        ("decisions-log", "OLD-DECISIONS.md", FOREIGN_DECISIONS),
        (
            "deferral-ledger",
            "OLD-LEDGER.md",
            "# Deferred\n\n- Decide the auth provider once SSO lands.\n",
        ),
        ("completion-record", "M3-DONE.md", FOREIGN_COMPLETION),
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

/// M43 Inc-3 T5 — the store sweep over the composed `[dev ▸ methodology]` pair stays
/// clean: `workflow-refs.schema-ref-resolves` (blocking) scans every shipped workflow
/// of BOTH origin packs against the composed cascade's doctype set, so a mistyped
/// `{{schema:<doctype>}}` ref in any rewritten migrate template would fail this
/// through the real binary.
#[test]
fn store_sweep_over_the_composed_pair_stays_clean() {
    let repo = TempDir::new("sweep");
    let home = TempDir::new("home");
    init_repo(repo.path());

    let out = jigc(repo.path(), home.path(), &["validate"], None);
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        out.status.success(),
        "`jigc validate` over the composed pair must exit 0 (no blocking finding); \
         got:\n{rendered}",
    );
    assert!(
        !rendered.contains("schema-ref-resolves"),
        "no shipped workflow may carry a dangling `{{{{schema:<doctype>}}}}` ref; \
         got:\n{rendered}",
    );
}
