//! M40 Increment 8 / T4 — the **M40 done-picture acceptance suite**: the rc.4
//! adoption-findings wave, driven end-to-end through the **real `jigc` binary**.
//! Inc 1–7 proved each feature per-feature (`migrate_rollback.rs`, `ingest.rs`,
//! `retitle_item.rs`, `doc_schema.rs`, `migrate_methodology.rs`,
//! `methodology_corpus_stamp.rs`); this suite ties them into the six integrated
//! done-picture arms (`design/worked-examples.md` → flow 41, authored in T5;
//! roadmap → M40 Inc 8).
//!
//! The six arms, each a `#[test]` over the real binary:
//!
//!   (1) **pre-staged `git rm` finalize (F7).** A user who pre-staged the foreign
//!       original's retirement (`git rm` before finalize) still lands the approved
//!       migration clean — exactly ONE commit carrying the promoted doc AND the
//!       staged deletion, no stage-phase fatal.
//!
//!   (2) **gitignored-tree ingest + hollow-adopt annotation (F8 + F4).** A
//!       gitignored `.md` (the adoption trial's `node_modules` funnel poison) is
//!       absent from the triage report, while a structurally hollow roadmap still
//!       adopts — row-annotated with the pinned `repeatable-populated` triage shape.
//!
//!   (3) **retitle-item round-trip (F5/F6).** A committed arch-doc component
//!       retitles through the real verb: the `{#id}` anchor is byte-frozen (only the
//!       heading-title bytes change), a follow-up write at the SAME item address
//!       lands, and finalize re-commits the doc with its inbound addresses intact.
//!
//!   (4) **`doc schema` (F1).** The read surface returns the separately-pinned
//!       contract json shape for a frozen doctype (contract-version 4 since M45 —
//!       the settability states) — versioned, fields with the injected stamp, the
//!       optional `options` slot flagged.
//!
//!   (5) **a methodology migration (F2).** `jigc migrate old-vision.md --as vision`
//!       lands the managed singleton at the repo-root literal `VISION.md` (the
//!       placement home, `# Vision` display-H1), retiring the foreign original in
//!       the same single commit.
//!
//!   (6) **a stamped-corpus validate (A1).** An unstamped (v0) committed methodology
//!       doc is detected + routed `migrate` by `jigc validate`; `jigc migrate-corpus`
//!       stamps it; the re-validate runs CLEAN — no `schema-conformance` finding, no
//!       `migrate` route left.
//!
//! Everything is asserted on the EMITTED bytes / exit codes / committed files of the
//! real binary (`CARGO_BIN_EXE_jigc`). No external test crates.

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
            "jigc-flow41-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/packs/methodology`).
fn methodology_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("packs")
        .join("methodology")
}

/// Build the pack's `doc-code` probe once (process-wide) and return its binary path —
/// the arch-doc finalize (arm 3) and the `validate` pre-flight (arm 6) resolve it via
/// `JIGC_DOC_CODE_PROBE` (mirrors `retitle_item::doc_code_probe`).
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
        .expect("utf-8 git stdout")
        .trim()
        .to_string()
}

/// Initialize a real git repo with one commit and the `.jigc/config/` project layer.
fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("create project layer");
}

/// `[dev ▸ methodology]` via the **listed-pack** mechanism: `packs.yaml` names the
/// on-disk methodology pack OVER the embedded dev base (the `migrate_methodology`
/// harness shape — arm 5's `vision` migration).
fn init_listed_pack(repo: &Path) {
    git_init(repo);
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        format!("packs:\n  - {}\n", methodology_pack_tree().display()),
    )
    .expect("write packs.yaml naming the methodology pack");
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, the doc-code probe selected,
/// optionally piping `stdin`. Never inherits a harness `JIGC_PACK_DIR` — the embedded
/// shipped pack (plus any project-listed pack / setup compose marker) is the base, so
/// the arms prove exactly the doctypes that ship.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_DOC_CODE_PROBE", doc_code_probe())
        .env_remove("JIGC_PACK_DIR")
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

/// Assert a `jigc` invocation succeeded, surfacing its streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must succeed; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Trimmed stdout of an invocation.
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim()
        .to_string()
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the task's commit doc so a finalize renders a clean git message.
fn fill_commit(repo: &Path, home: &Path, task: &str, scope: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), "docs");
    set_field(repo, home, &format!("commit:{task}#scope"), scope);
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        b"Driven by the flow-41 acceptance suite.\n",
    );
}

/// The `HEAD` commit count.
fn head_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .unwrap()
}

// ───────────────── Arm 1 — pre-staged `git rm` migration finalize ─────────────────

/// The foreign Keep-a-Changelog file (committed, so its retirement is a tracked
/// deletion the user can pre-stage with `git rm`).
const FOREIGN_CHANGELOG: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// The declarative whole-doc payload that re-authors the canonical changelog in ONE
/// `doc author --from-file -` batch.
const PAYLOAD_CHANGELOG: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 0.1.0
        set:
          date: 2021-03-09
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;

/// **Arm 1 (F7).** A user who pre-staged the retirement themselves (`git rm
/// HISTORY.md` before finalize) still lands the approved migration clean: the
/// retirement pathspec is discriminated on the INDEX (not HEAD), so the stage-phase
/// `git add` never fatals — exactly ONE whole-index commit carries the promoted
/// `CHANGELOG.md` AND the user's staged deletion (`design/finalize.md` → M40
/// refinement item 1).
#[test]
fn pre_staged_git_rm_migration_finalize_lands_one_clean_commit() {
    let repo = TempDir::new("prestaged");
    let home = TempDir::new("home");
    git_init(repo.path());

    // Track the foreign original so `git rm` has a tracked path to pre-stage.
    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write HISTORY.md");
    git(repo.path(), &["add", "HISTORY.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign changelog"],
    );

    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["migrate", "HISTORY.md", "--as", "changelog"],
            None,
        ),
        "`jigc migrate HISTORY.md --as changelog`",
    );

    // Author the canonical changelog in one declarative batch (the commit doc is
    // auto-provisioned filled on a migration task).
    let task = "migrate-changelog-history-3268e06b69e1";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "-",
                "--task",
                task,
            ],
            Some(PAYLOAD_CHANGELOG.as_bytes()),
        ),
        "`jigc doc author changelog --from-file -`",
    );

    // The user pre-stages the retirement: worktree file gone, deletion in the index.
    git(repo.path(), &["rm", "-q", "HISTORY.md"]);

    let before = head_count(repo.path());
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--approve"],
            None,
        ),
        "`jigc task finalize --approve` (pre-staged git rm)",
    );

    // Exactly ONE commit landed, carrying BOTH the promoted doc and the pre-staged
    // deletion — the whole-index transaction, clean.
    assert_eq!(
        head_count(repo.path()),
        before + 1,
        "the pre-staged migration finalize lands exactly ONE commit",
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains("D\tHISTORY.md"),
        "the landed commit carries the user's pre-staged foreign deletion; \
         name-status:\n{name_status}",
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md"),
        "the SAME commit carries the promoted canonical doc; name-status:\n{name_status}",
    );
    assert!(
        repo.path().join("CHANGELOG.md").exists() && !repo.path().join("HISTORY.md").exists(),
        "the promoted doc is on disk at the root placement home and the foreign is retired",
    );
}

// ─────────── Arm 2 — gitignored-tree ingest + hollow-adopt annotation ───────────

/// **Arm 2 (F8 + F4).** Over a tree carrying a gitignored `.md` (the adoption
/// trial's `node_modules` funnel poison) and a structurally hollow roadmap at its
/// literal placement home: the gitignored candidate is ABSENT from the triage report
/// (the candidate set comes from `git ls-files --exclude-standard`), while the hollow
/// roadmap still adopts — its row carrying the pinned `repeatable-populated` triage
/// annotation instead of a silent verdict.
#[test]
fn gitignored_tree_ingest_excludes_ignored_and_annotates_the_hollow_adopt() {
    let repo = TempDir::new("ingest");
    let home = TempDir::new("home");
    git_init(repo.path());

    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );

    // Gitignore `node_modules/` (appending — setup may have seeded a root
    // `.gitignore`), then seed the gitignored candidate.
    let gitignore = repo.path().join(".gitignore");
    let mut ignore_body = fs::read_to_string(&gitignore).unwrap_or_default();
    ignore_body.push_str("node_modules/\n");
    fs::write(&gitignore, ignore_body).expect("write .gitignore");
    fs::create_dir_all(repo.path().join("node_modules").join("pkg")).expect("mk node_modules");
    fs::write(
        repo.path().join("node_modules/pkg/README.md"),
        "# pkg\n\nA dependency's readme — never a triage candidate.\n",
    )
    .expect("write gitignored candidate");

    // A structurally hollow roadmap at its literal placement home — exactly what the
    // adoption trial adopted silent pre-M40.
    fs::create_dir_all(repo.path().join("docs")).expect("mk docs/");
    fs::write(
        repo.path().join("docs").join("roadmap.md"),
        "# roadmap\n\n## Milestones\n",
    )
    .expect("write hollow roadmap");

    let out = jigc(repo.path(), home.path(), &["ingest"], None);
    assert_ok(
        &out,
        "`jigc ingest` (annotations are fixed-advisory, exit 0)",
    );
    let report = String::from_utf8(out.stdout).expect("utf-8 ingest report");

    // The gitignored candidate never enters the funnel.
    assert!(
        !report.contains("node_modules/pkg/README.md"),
        "a gitignored candidate must be absent from the triage report:\n{report}",
    );

    // The hollow roadmap still adopts — annotated, never silent.
    let row = report
        .lines()
        .find(|l| l.contains("docs/roadmap.md"))
        .unwrap_or_else(|| panic!("the roadmap row must appear in the report:\n{report}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the hollow roadmap still adopts (the annotation never flips a verdict):\n{report}",
    );
    assert!(
        report.contains("adopted — structurally empty: 0 milestones"),
        "the hollow adopt carries the pinned repeatable-populated triage annotation:\n{report}",
    );
}

// ──────────────────── Arm 3 — the retitle-item round-trip ────────────────────

/// Author + finalize `arch-doc:cache-layer` with ONE component (`Session store`,
/// anchored at a present symbol) — the committed doc the round-trip retitles.
fn commit_arch_doc(repo: &Path, home: &Path) {
    let task = "document-the-cache-layer";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "architecture-documentation",
                "document the cache layer",
            ],
            None,
        ),
        "`jigc start --workflow architecture-documentation`",
    );
    let created = jigc(
        repo,
        home,
        &["doc", "create", "arch-doc", "--title", "Cache layer"],
        None,
    );
    assert_ok(&created, "`jigc doc create arch-doc`");
    assert_eq!(stdout_of(&created), "arch-doc:cache-layer");

    set_slot(
        repo,
        home,
        "arch-doc:cache-layer#overview",
        b"The cache layer owns ephemeral session state.\n",
    );
    let added = jigc(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "arch-doc:cache-layer#components",
            "--title",
            "Session store",
        ],
        None,
    );
    assert_ok(&added, "`jigc doc add-item …#components`");
    let item = stdout_of(&added);
    assert_eq!(item, "arch-doc:cache-layer#components/session-store");
    set_slot(
        repo,
        home,
        &format!("{item}/description"),
        b"Holds session blobs keyed by token.\n",
    );
    set_field(
        repo,
        home,
        &format!("{item}/implemented-by"),
        "src/lib.rs#present_symbol",
    );

    fill_commit(repo, home, task, "arch-doc");
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize` (arch-doc setup)",
    );
}

/// **Arm 3 (F5/F6).** A committed arch-doc component retitles through the real verb:
/// the staged copy differs from the committed bytes in EXACTLY the heading-title
/// bytes (the `{#id}` anchor byte-frozen), a follow-up `set-slot` at the SAME item
/// address lands (every inbound address survives), and finalize re-commits the
/// retitled doc clean.
#[test]
fn retitle_item_round_trips_with_the_anchor_and_inbound_addresses_intact() {
    let repo = TempDir::new("retitle");
    let home = TempDir::new("home");
    git_init(repo.path());
    // The known symbol every `implemented-by` anchor resolves against.
    fs::create_dir_all(repo.path().join("src")).expect("mk src/");
    fs::write(
        repo.path().join("src").join("lib.rs"),
        "pub fn present_symbol() -> u32 {\n    42\n}\n",
    )
    .expect("write lib.rs");
    git(repo.path(), &["add", "src/lib.rs"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "seed the anchor symbol"],
    );
    commit_arch_doc(repo.path(), home.path());

    let committed_path = repo
        .path()
        .join("docs")
        .join("architecture")
        .join("cache-layer.md");
    let committed = fs::read_to_string(&committed_path).expect("read the committed arch-doc");
    let old_heading = "### Session store  {#session-store}";
    assert!(
        committed.contains(old_heading),
        "the committed doc carries the canonical component heading; got:\n{committed}",
    );

    // A second task retitles the committed component through the real verb.
    let task = "retitle-the-store";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["start", "--workflow", "single-task", "retitle the store"],
            None,
        ),
        "`jigc start` (retitle task)",
    );
    let addr = "arch-doc:cache-layer#components/session-store";
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["doc", "retitle-item", addr, "--title", "Session vault"],
            None,
        ),
        "`jigc doc retitle-item`",
    );

    // The staged copy differs from the committed bytes in EXACTLY the heading-title
    // bytes: the `{#session-store}` anchor (and every other byte) is frozen.
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(task)
            .join("docs")
            .join("arch-doc:cache-layer.md"),
    )
    .expect("read the staged retitled doc");
    assert_eq!(
        staged,
        committed.replace(old_heading, "### Session vault  {#session-store}"),
        "only the heading-title bytes change; the {{#id}} anchor is byte-identical",
    );

    // A follow-up write at the SAME item address lands — the id (hence every inbound
    // address) survived the retitle.
    set_slot(
        repo.path(),
        home.path(),
        &format!("{addr}/description"),
        b"Holds session blobs and refresh tokens keyed by token.\n",
    );

    // Finalize is green: the retitled doc re-commits with the frozen anchor + the
    // follow-up prose.
    fill_commit(repo.path(), home.path(), task, "arch-doc");
    assert_ok(
        &jigc(repo.path(), home.path(), &["task", "finalize", task], None),
        "`jigc task finalize` (retitle task)",
    );
    let recommitted = fs::read_to_string(&committed_path).expect("read the re-committed arch-doc");
    assert!(
        recommitted.contains("### Session vault  {#session-store}"),
        "the re-committed doc carries the new title over the FROZEN anchor; got:\n{recommitted}",
    );
    assert!(
        recommitted.contains("refresh tokens"),
        "the follow-up set-slot at the old item address landed; got:\n{recommitted}",
    );
}

// ─────────────────── Arm 4 — the pinned `doc schema` contract ───────────────────

/// **Arm 4 (F1).** `jigc doc schema adr --format json` returns the separately-pinned
/// contract shape (contract-version 4 since M45 — the three settability states; 3 was
/// the M43 rc.7 write-verb addresses, 2 the M41 rc.5 `of`/`section` join): the
/// contract version stamped,
/// the doctype's frozen `schema-version` carried, the loader-injected
/// `schema-version` stamp field rendered non-author-required, and the optional
/// `options` slot flagged. (The byte-verbatim goldens live in `doc_schema.rs`; this
/// arm asserts the contract markers behaviorally through the real binary.)
#[test]
fn doc_schema_returns_the_pinned_contract_shape() {
    let repo = TempDir::new("schema");
    let home = TempDir::new("home");
    git_init(repo.path());

    let out = jigc(
        repo.path(),
        home.path(),
        &["doc", "schema", "adr", "--format", "json"],
        None,
    );
    assert_ok(&out, "`jigc doc schema adr --format json`");
    let value: serde_json::Value =
        serde_json::from_str(&stdout_of(&out)).expect("the emitted contract parses as json");

    assert_eq!(
        value["contract-version"], 4,
        "the projection carries the pinned contract version",
    );
    assert_eq!(value["type"], "adr", "the projection names the doctype");
    assert_eq!(
        value["schema-version"], 2,
        "the frozen adr reports schema-version 2 (the M36 `options` migration bump)",
    );

    // The loader-injected stamp field renders CLI-derived (never author-required).
    let fields = value["fields"].as_array().expect("`fields` is an array");
    let stamp = fields
        .iter()
        .find(|f| f["id"] == "schema-version")
        .unwrap_or_else(|| panic!("the injected stamp field renders; got:\n{value}"));
    assert_eq!(
        stamp["author-required"],
        serde_json::Value::Bool(false),
        "the stamp is CLI-derived, never author-required; got:\n{stamp}",
    );

    // The four adr sections, the optional `options` slot flagged optional.
    let sections = value["sections"]
        .as_array()
        .expect("`sections` is an array");
    let ids: Vec<&str> = sections.iter().filter_map(|s| s["id"].as_str()).collect();
    assert_eq!(
        ids,
        ["context", "options", "decision", "consequences"],
        "the adr sections render in schema order",
    );
    let options = &sections[1];
    assert_eq!(
        options["optional"],
        serde_json::Value::Bool(true),
        "the `options` slot is flagged optional; got:\n{options}",
    );
}

// ──────────────── Arm 5 — a methodology migration lands VISION.md ────────────────

/// A plausible foreign vision/charter document at an off-canonical root home.
const FOREIGN_VISION: &str = "\
# Where dashbard is going

Dashbard turns messy spreadsheets into live dashboards nobody has to babysit.

## Principles

- The importer never mutates the source spreadsheet.

## Someday

- A plugin marketplace, once the widget API stabilizes.
";

/// The canonical rewrite payload — the fixed singleton `title: Vision`, the three
/// prose slots, `grounded-in` OMITTED (a foreign vision has no managed `research`
/// target, so the ref would dangle at finalize).
const PAYLOAD_VISION: &str = r#"title: Vision
sections:
  - id: thesis
    set:
      thesis: "<<Dashbard turns messy spreadsheets into live dashboards nobody has to babysit.>>"
  - id: invariants
    set:
      invariants: "<<- The importer never mutates the source spreadsheet.>>"
  - id: open-questions
    set:
      open-questions: "<<- A plugin marketplace, once the widget API stabilizes.>>"
"#;

/// **Arm 5 (F2).** A foreign vision migrates through `jigc migrate old-vision.md
/// --as vision`: the approved finalize lands exactly ONE commit that writes the
/// managed singleton at the repo-root literal `VISION.md` (the placement home, the
/// `# Vision` display-H1) and retires the foreign original.
#[test]
fn methodology_migration_lands_the_root_vision() {
    let repo = TempDir::new("vision");
    let home = TempDir::new("home");
    init_listed_pack(repo.path());

    fs::write(repo.path().join("old-vision.md"), FOREIGN_VISION).expect("write foreign vision");
    git(repo.path(), &["add", "old-vision.md"]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign vision"]);

    let composed = jigc(
        repo.path(),
        home.path(),
        &["migrate", "old-vision.md", "--as", "vision"],
        None,
    );
    assert_ok(&composed, "`jigc migrate old-vision.md --as vision`");
    assert!(
        stdout_of(&composed).contains("doc author vision --from-file"),
        "the composed migrate guidance carries the batch author verb; got:\n{}",
        stdout_of(&composed),
    );

    let task = "migrate-vision-old-vision-b7ea1b0697bf";
    let authored = jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "author",
            "vision",
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(PAYLOAD_VISION.as_bytes()),
    );
    assert_ok(&authored, "`jigc doc author vision`");
    assert_eq!(
        stdout_of(&authored),
        "vision:vision",
        "the singleton mints at the fixed slug = the type id",
    );

    let before = head_count(repo.path());
    assert_ok(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--approve"],
            None,
        ),
        "`jigc task finalize --approve` (vision migration)",
    );
    assert_eq!(
        head_count(repo.path()),
        before + 1,
        "the approved vision migration lands exactly ONE commit",
    );

    // The managed vision is committed at the repo-root literal `VISION.md` — the
    // placement home, never a `docs/`-buried folder — with the display-H1.
    let on_disk = fs::read_to_string(repo.path().join("VISION.md"))
        .expect("the managed VISION.md is on disk at the repo root");
    assert!(
        on_disk.lines().any(|l| l.trim() == "# Vision"),
        "the managed vision's H1 reads `# Vision`; got:\n{on_disk}",
    );
    assert!(
        on_disk.contains("live dashboards nobody has to babysit"),
        "the committed vision carries the authored thesis prose:\n{on_disk}",
    );

    // The SAME commit retires the foreign original (`--no-renames` keeps the similar
    // prose from collapsing the delete+add into one `R`).
    assert!(
        !repo.path().join("old-vision.md").exists(),
        "the approved migration retires the foreign original from disk",
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains("D\told-vision.md") && name_status.contains("A\tVISION.md"),
        "one commit carries the foreign deletion + the added root vision:\n{name_status}",
    );
}

// ─────────── Arm 6 — a stamped-corpus validate over migrate-corpus output ───────────

/// The **v0 (unstamped) research doc** — the byte form a real pre-M40 keeper corpus
/// has: header-bearing, authored prose in all three slots, no `schema-version` line.
const RESEARCH_V0: &str = "\
---
date: 2026-07-10
---

# Cache Strategy

## Question

What cache strategy fits the session store.

## Findings

An in-memory single node wins on latency.

## Sources

Prior art in the issue tracker.
";

/// **Arm 6 (A1).** A committed unstamped (v0) methodology doc — frozen by the M40 A1
/// methodology manifest — is detected + routed `migrate` by `jigc validate`;
/// `jigc migrate-corpus` stamps it; the re-validate over the stamped corpus runs
/// CLEAN: exit 0, no `schema-conformance` finding, no `migrate` route left.
#[test]
fn stamped_corpus_validate_runs_clean_over_migrate_corpus_output() {
    let repo = TempDir::new("stamp");
    let home = TempDir::new("home");
    git_init(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );

    // The v0 corpus: a committed methodology research doc with NO schema-version
    // stamp, otherwise conformant (`research` under the dev `docs-root`).
    let research_rel = "docs/research/cache-strategy.md";
    fs::create_dir_all(repo.path().join("docs").join("research")).expect("mk docs/research/");
    fs::write(repo.path().join(research_rel), RESEARCH_V0).expect("write v0 research");
    git(repo.path(), &["add", research_rel]);
    git(repo.path(), &["commit", "-q", "-m", "seed v0 research"]);

    // DETECT — `jigc validate` reports the stranded v0 doc, routed `migrate`, and (M42) exits
    // **non-zero**: an unmigrated corpus is the third exit-flipping exception
    // (`design/validation.md` → Exit semantics). The re-validate below pins the other half —
    // once migrated, the same sweep is back to exit 0.
    let detect = jigc(repo.path(), home.path(), &["validate"], None);
    assert!(
        !detect.status.success(),
        "`jigc validate` over the v0 corpus exits non-zero; stdout:\n{}",
        stdout_of(&detect),
    );
    let detect_out = stdout_of(&detect);
    assert!(
        detect_out.contains("route: migrate"),
        "the unstamped v0 research doc is detected and routed `migrate`; got:\n{detect_out}",
    );

    // MIGRATE — `jigc migrate-corpus` stamps the doc, the bytes otherwise unchanged.
    let migrate = jigc(repo.path(), home.path(), &["migrate-corpus"], None);
    assert_ok(&migrate, "`jigc migrate-corpus`");
    let after = fs::read_to_string(repo.path().join(research_rel)).expect("read stamped research");
    assert_eq!(
        after,
        RESEARCH_V0.replacen(
            "date: 2026-07-10\n",
            "date: 2026-07-10\nschema-version: 1\n",
            1
        ),
        "the migrated doc is the v0 bytes plus the appended stamp line, nothing else",
    );

    // RE-VALIDATE — the stamped corpus is clean: exit 0, no schema-conformance
    // finding, no migrate route left.
    let revalidate = jigc(repo.path(), home.path(), &["validate"], None);
    assert_ok(&revalidate, "`jigc validate` over the stamped corpus");
    let revalidate_out = stdout_of(&revalidate);
    assert!(
        !revalidate_out.contains("schema-conformance"),
        "the stamped corpus surfaces NO schema-conformance finding; got:\n{revalidate_out}",
    );
    assert!(
        !revalidate_out.contains("route: migrate"),
        "the stamped corpus carries no migrate route; got:\n{revalidate_out}",
    );
}
