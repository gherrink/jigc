//! M24 Increment 7, T3 — the flow-26 marquee: the **full changelog migration through
//! the declarative batch path**, proven end to end against the built `jigc` binary over
//! `git init` temp repos against the **shipped** dev pack ([worked-examples.md](../../../design/worked-examples.md)
//! → flow 26; [auto-migration.md](../../../design/auto-migration.md) → Hardening #1–#9).
//!
//! Flow 25 proved the migrate spine with **per-leaf** authoring; flow 26 is the M24
//! done-bar: the whole canonical changelog is authored from **ONE** `jigc doc author
//! --from-file` payload (the batch), over a **dateless** foreign source carrying a
//! category-**merge** case (two foreign categories → one enum member) and a
//! **headingless** `feat:`/`fix:` case (the LLM infers the member). It exercises:
//!   - **HAPPY** — setup → migrate → ONE batch author → review-gate block (no
//!     `--approve`) → `--approve`: the whole changelog round-trips byte-stable, dateless
//!     releases render NO date line (no fabricated history, #6), the commit is
//!     auto-provisioned (the formulaic `docs(changelog): …` subject, NO manual commit
//!     authoring, #4), and the landed commit holds only the migration set (promote +
//!     retire + `.jigc/config/` + `.jigc/.gitignore`) with no unrelated user WIP swept
//!     in (#9a); `jigc ingest` reports it adopted;
//!   - **RED — write-time enum block (#3)** — a non-member category is rejected at the
//!     `add-item` write verb (`schema-conformance.field-value-conformant` naming the
//!     slug-cased `…/category` address), before anything is staged or committed;
//!   - **RED — `remove-item` (#1)** — a mis-authored NESTED change-group is retracted
//!     byte-stable, the sibling group + parent release surviving;
//!   - **RED — an off-canonical foreign source (#8)** — `jigc migrate docs/changelog/changelog.md`
//!     (the changelog's OLD folder home, now off-canonical post-M38) seeds the working area
//!     BLANK then authors end-to-end through the batch, finalize promoting root `CHANGELOG.md`
//!     (Added) and retiring the off-canonical original (Deleted);
//!   - **RED — the structural release-delta summary (#5)** — a batch that drops a source
//!     release surfaces the dropped version at the review gate, labeled fuzzy/advisory.
//!
//! Every assertion drives the EMITTED bytes / exit code of the real `jigc` binary
//! (`CARGO_BIN_EXE_jigc`) over the shipped dev pack (`JIGC_PACK_DIR`). Distinct from T4:
//! this is the cargo-built TDD proof, not the PATH-pinned measured run over the two
//! named repos.

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
            "jigc-flow26marquee-{tag}-{}-{:?}",
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
        .trim_end_matches('\n')
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both streams.
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

/// The off-router migration task id — `jigc migrate` mints a per-file `migrate-<doctype>-<slug(path)>` (the empty
/// intent slugs the `migrate-` id-source fallback), keeping the bare `changelog`
/// namespace free (`auto-migration.md` → Hardening #9).
const TASK: &str = "migrate-changelog-history";

/// The shipped changelog schema, loaded for the byte-stable round-trip assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped changelog schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// A representative multi-release **dateless** foreign Keep-a-Changelog file. Release
/// `1.2.0` carries TWO foreign categories (`Improvements` + `Changes`) that MERGE onto
/// the single enum member `changed` (#7), plus a `Fixed`. Release `1.1.0` is
/// **headingless** — bare `feat:`/`fix:` bullets the LLM maps onto `added`/`fixed` (#7).
/// Release `1.0.0` is the plain KaC case. No `- YYYY-MM-DD` anywhere (#6).
const FOREIGN: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0]
### Improvements
- Faster cold start.
### Changes
- Default timeout raised to 30s.
### Fixed
- Session fixation on logout.

## [1.1.0]
- feat: device-code OAuth flow.
- fix: race in the cache warmer.

## [1.0.0]
### Added
- First public release.
";

/// The ONE declarative batch payload the LLM authors from [`FOREIGN`]: 1.2.0 merges the
/// two foreign categories onto `Changed` + keeps `Fixed`; 1.1.0 infers `Added`/`Fixed`
/// from the `feat:`/`fix:` prefixes; 1.0.0 maps directly. NO `date:` keys — the dateless
/// source fabricates no history. The whole doc is one payload, one CLI batch.
const HAPPY_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        sections:
          - id: changes
            items:
              - title: Changed
                set:
                  notes: "<<- Faster cold start.\n- Default timeout raised to 30s.>>"
              - title: Fixed
                set:
                  notes: "<<- Session fixation on logout.>>"
      - title: 1.1.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- Device-code OAuth flow.>>"
              - title: Fixed
                set:
                  notes: "<<- Race in the cache warmer.>>"
      - title: 1.0.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;

/// The staged `changelog:changelog` instance in the migration `task`'s working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// Assert `staged` round-trips byte-stable over the shipped schema: `render(parse(x)) == x`.
fn assert_byte_stable(pack: &Path, staged: &str) {
    let schema = shipped_changelog_schema(pack);
    let parsed =
        engine::write::instance_from_source(&schema, staged).expect("staged changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        staged,
        "the changelog is byte-stable across parse → render:\n{staged}",
    );
}

/// Write + **commit** the foreign original at `source_path` (so its retirement lands as a
/// tracked deletion), then `setup` and `jigc migrate <source_path> --as changelog`,
/// minting the off-router `migrate-changelog-history` task. Returns the composed migrate stdout.
fn setup_and_migrate(
    repo: &Path,
    home: &Path,
    pack: &Path,
    source_path: &str,
    foreign: &str,
) -> String {
    let abs = repo.join(source_path);
    if let Some(parent) = abs.parent() {
        fs::create_dir_all(parent).expect("create source parent dir");
    }
    fs::write(&abs, foreign).expect("write foreign source");
    git(repo, &["add", source_path]);
    git(repo, &["commit", "-q", "-m", "track foreign changelog"]);

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", source_path, "--as", "changelog"],
            None,
        ),
        "jigc migrate --as changelog",
    )
}

/// Author the whole canonical changelog in ONE `doc author --from-file-file` batch against the
/// migration `task`, asserting it stages the singleton address.
fn author_via_batch(repo: &Path, home: &Path, pack: &Path, task: &str, payload: &str) {
    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "-",
                "--task",
                task,
            ],
            Some(payload.as_bytes()),
        ),
        "jigc doc author changelog --from-file -",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the batch verb stages the minted singleton address",
    );
}

/// HAPPY — the full batch migration, end to end: setup → migrate → ONE batch author →
/// review-gate block → `--approve`. Asserts byte-stable round-trip, dateless rendering,
/// the auto-provisioned formulaic commit (no manual commit authoring), the narrowed
/// commit set (no unrelated WIP), and a follow-up `ingest` adopting the doc.
#[test]
fn flow26_full_batch_migration_walk() {
    let repo = TempDir::new("walk");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    setup_and_migrate(repo.path(), home.path(), &pack, "HISTORY.md", FOREIGN);
    author_via_batch(repo.path(), home.path(), &pack, TASK, HAPPY_PAYLOAD);

    // The batch-authored staged doc round-trips byte-stable, carries all three versions
    // and the merged/inferred groups, and renders NO date line (dateless source, #6).
    let staged = staged_changelog(repo.path(), TASK);
    assert_byte_stable(&pack, &staged);
    assert!(
        staged.contains("### 1.2.0")
            && staged.contains("### 1.1.0")
            && staged.contains("### 1.0.0"),
        "all three releases are authored from the one batch:\n{staged}",
    );
    assert!(
        !staged.contains("date:"),
        "the dateless migration fabricates no date line:\n{staged}",
    );

    // An unrelated untracked file the user left in the tree — it must NOT be swept in (#9a).
    fs::write(repo.path().join("scratch.txt"), "private WIP\n").expect("write scratch.txt");

    // ── Review gate: finalize WITHOUT --approve blocks (exit 4), commits nothing ──
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let gated = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );
    assert_eq!(
        gated.status.code(),
        Some(4),
        "finalize without --approve on a migration task exits 4 (review-pending); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&gated.stdout),
        String::from_utf8_lossy(&gated.stderr),
    );
    let diff = format!(
        "{}{}",
        String::from_utf8_lossy(&gated.stdout),
        String::from_utf8_lossy(&gated.stderr),
    );
    assert!(
        diff.contains("## [1.2.0]"),
        "the fidelity diff surfaces the foreign source bytes; got:\n{diff}"
    );
    assert!(
        diff.contains("CHANGELOG.md"),
        "the fidelity diff names the canonical rewrite destination; got:\n{diff}"
    );
    assert!(
        diff.contains("--approve"),
        "the block tells the human how to approve; got:\n{diff}"
    );
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "a blocked review leaves HEAD unchanged"
    );

    // ── Approve: write byte-stable, retire foreign, ONE commit, ingest adopted ──
    let approved = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        approved.status.success(),
        "finalize --approve on the conformant batch migration lands (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approved.stdout),
        String::from_utf8_lossy(&approved.stderr),
    );
    let count_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        count_after,
        count_before + 1,
        "the approved migration lands exactly ONE commit"
    );

    // The committed canonical doc is byte-stable and matches the staged bytes (the batch
    // body landed verbatim) — and still carries no date line.
    let committed = fs::read_to_string(repo.path().join("CHANGELOG.md"))
        .expect("the canonical changelog is on disk after finalize");
    assert_eq!(
        committed, staged,
        "the committed canonical doc equals the batch-authored staged bytes",
    );
    assert_byte_stable(&pack, &committed);
    assert!(
        !committed.contains("date:"),
        "the committed dateless migration carries no date line:\n{committed}",
    );

    // The commit is AUTO-PROVISIONED — the formulaic subject lands with NO manual commit
    // authoring (this test never set-field/set-slot the commit doc). Asserted on the
    // EMITTED bytes (`git log`), never a reconstructed equivalent.
    let subject = git(repo.path(), &["log", "-1", "--format=%s"]);
    assert_eq!(
        subject, "docs(changelog): adopt HISTORY.md as a managed changelog",
        "the auto-provisioned migration commit lands the formulaic subject",
    );
    assert!(
        !git(repo.path(), &["log", "-1", "--format=%b"])
            .trim()
            .is_empty(),
        "the auto-provisioned commit carries a non-empty body",
    );

    // The landed commit holds ONLY the migration set: A CHANGELOG.md, D
    // HISTORY.md — NOT the jigc config layer (setup committed that on its own, M26) and
    // NOT the unrelated WIP.
    let delta = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        delta.lines().any(|l| l == "A\tCHANGELOG.md"),
        "the commit promotes the canonical doc:\n{delta}",
    );
    assert!(
        delta.lines().any(|l| l == "D\tHISTORY.md"),
        "the SAME commit retires the foreign original:\n{delta}",
    );
    // The jigc-tracked config layer + `.gitignore` are committed by `setup` itself (M26),
    // so they are present at HEAD but carry NO entry in the migration commit's delta.
    let tree = git(repo.path(), &["ls-tree", "-r", "--name-only", "HEAD"]);
    assert!(
        tree.lines().any(|l| l.starts_with(".jigc/config/")),
        "the jigc-tracked config layer is committed (by setup) and present at HEAD:\n{tree}",
    );
    assert!(
        tree.lines().any(|l| l == ".jigc/.gitignore"),
        "`.jigc/.gitignore` is committed (by setup) and present at HEAD:\n{tree}",
    );
    assert!(
        !delta.lines().any(|l| l.contains(".jigc/")),
        "the migration commit's delta carries NO `.jigc/` paths (setup committed them):\n{delta}",
    );
    assert!(
        !delta.lines().any(|l| l.ends_with("scratch.txt")),
        "the unrelated user WIP is NOT swept into the migration commit:\n{delta}",
    );
    assert!(
        git(repo.path(), &["status", "--porcelain"])
            .lines()
            .any(|l| l == "?? scratch.txt"),
        "scratch.txt is still untracked after the migration commit",
    );

    // A follow-up `jigc ingest` reports the managed changelog ADOPTED.
    let ingest = ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["ingest"], None),
        "jigc ingest",
    );
    let row = ingest
        .lines()
        .find(|l| l.contains("CHANGELOG.md"))
        .unwrap_or_else(|| panic!("ingest reports the managed changelog:\n{ingest}"));
    assert!(
        row.contains("adopted") && !row.contains("needs-reconcile") && !row.contains("unmanaged"),
        "the managed changelog ingests as adopted:\n{row}",
    );
}

/// RED — write-time enum block (#3). On a staged migration, an `add-item` whose category
/// title slugs outside the `category` enum is rejected AT THE WRITE VERB — non-zero exit,
/// the finding `schema-conformance.field-value-conformant` naming the slug-cased
/// `…/category` address, with no commit. Drives the EMITTED release address verbatim.
#[test]
fn flow26_red_write_time_enum_block_at_add_item() {
    let repo = TempDir::new("enum");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    setup_and_migrate(repo.path(), home.path(), &pack, "HISTORY.md", FOREIGN);
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                TASK,
            ],
            None,
        ),
        "doc create changelog",
    );
    // Add a real release via add-item, then drive its EMITTED address downstream.
    let rel = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.2.0",
                "--task",
                TASK,
            ],
            None,
        ),
        "add-item release",
    );
    // (M42 inc-9 T2) The finding address is the minted release address in **URI normal
    // form**, driven verbatim (`design/command-output-contract.md` → the `write.*` row: a
    // bare fragment is not a stable key).
    let expected_address = format!("{rel}/changes/performance/category");

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    // `Performance` ∉ the enum {added, changed, deprecated, removed, fixed, security}.
    // `--format json` so the emitted finding envelope is parseable.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "--format",
            "json",
            "doc",
            "add-item",
            &format!("{rel}/changes"),
            "--title",
            "Performance",
            "--task",
            TASK,
        ],
        None,
    );
    assert!(
        !out.status.success(),
        "a non-member category blocks at the add-item write verb (non-zero exit); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let report: engine::result::ValidationReport = serde_json::from_slice(&out.stderr)
        .unwrap_or_else(|e| {
            panic!(
                "the block renders a JSON validation report on stderr ({e}); stderr:\n{}",
                String::from_utf8_lossy(&out.stderr)
            )
        });
    let block = report
        .findings
        .iter()
        .find(|f| {
            f.code == "schema-conformance.field-value-conformant"
                && f.location.as_ref().and_then(|l| l.address.as_deref()) == Some(&expected_address)
        })
        .unwrap_or_else(|| {
            panic!(
                "the enum block names the slug-cased `{expected_address}`; findings:\n{:#?}",
                report.findings
            )
        });
    assert_eq!(
        block.severity,
        engine::finding::Severity::Blocking,
        "the enum conformance failure is blocking",
    );

    // No commit on the blocked branch.
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the enum block leaves HEAD unchanged"
    );
    assert!(
        !repo.path().join("CHANGELOG.md").exists(),
        "the enum block adopts nothing (no canonical doc on disk)"
    );
}

/// RED — `remove-item` (#1). After authoring the whole changelog via the batch, removing
/// a mis-authored NESTED change-group retracts exactly that group, round-trips
/// byte-stable, and leaves the sibling group + parent release intact.
#[test]
fn flow26_red_remove_item_retracts_nested_change_group_byte_stable() {
    let repo = TempDir::new("remove");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    setup_and_migrate(repo.path(), home.path(), &pack, "HISTORY.md", FOREIGN);
    author_via_batch(repo.path(), home.path(), &pack, TASK, HAPPY_PAYLOAD);

    // 1.2.0 carries `changed` + `fixed`; retract the `changed` group.
    let removed = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "remove-item",
            "changelog:changelog#releases/120/changes/changed",
            "--task",
            TASK,
        ],
        None,
    );
    ok_stdout(removed, "doc remove-item nested change-group");

    let staged = staged_changelog(repo.path(), TASK);
    assert!(
        !staged.contains("#### Changed"),
        "the removed change-group heading is gone:\n{staged}",
    );
    // The sibling `fixed` group under 1.2.0 and the other releases survive.
    assert!(
        staged.contains("### 1.2.0")
            && staged.contains("#### Fixed")
            && staged.contains("### 1.1.0")
            && staged.contains("### 1.0.0"),
        "only the addressed nested group is removed; the rest survive:\n{staged}",
    );
    assert_byte_stable(&pack, &staged);
}

/// RED — an OFF-canonical foreign source (#8, post-M38). `jigc migrate
/// docs/changelog/changelog.md` — the changelog's OLD folder home, now off-canonical
/// since the doctype relocated to root `CHANGELOG.md` — seeds the working area BLANK
/// (nothing squats the root canonical home), authors end-to-end through the batch, and
/// finalize PROMOTES root `CHANGELOG.md` (Added) + RETIRES the off-canonical original
/// (Deleted), byte-stable. (The AT-canonical adopt-in-place case — a foreign file already
/// sitting at root `CHANGELOG.md` — is T2's own proof.)
#[test]
fn flow26_off_canonical_docs_changelog_source_promotes_root_and_retires() {
    let repo = TempDir::new("squatter");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A NON-conformant changelog at the OLD folder home (`docs/changelog/changelog.md`),
    // now off-canonical. The per-file migration id folds the (extension-stripped,
    // separator-folded) source path into the slug, so this source mints a distinct task id
    // from the root-`HISTORY.md` tests above.
    const SQUATTER_TASK: &str = "migrate-changelog-docs-changelog-changelog";
    const SQUATTER: &str = "# Whatever\n\nnon-conformant prior content at the old folder home\n";
    setup_and_migrate(
        repo.path(),
        home.path(),
        &pack,
        "docs/changelog/changelog.md",
        SQUATTER,
    );

    // The batch authors the canonical root home: the working area seeds BLANK (nothing
    // squats root `CHANGELOG.md`), never the off-canonical source bytes.
    const SQUATTER_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.0.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;
    author_via_batch(
        repo.path(),
        home.path(),
        &pack,
        SQUATTER_TASK,
        SQUATTER_PAYLOAD,
    );

    let staged = staged_changelog(repo.path(), SQUATTER_TASK);
    assert!(
        !staged.contains("non-conformant prior content"),
        "the off-canonical source body must NOT be copied in as the edit base (no Frankenstein doc):\n{staged}",
    );
    assert!(
        staged.contains("### 1.0.0") && staged.contains("First public release."),
        "the batch authors onto the clean blank skeleton:\n{staged}",
    );
    assert_byte_stable(&pack, &staged);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", SQUATTER_TASK, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "finalize --approve on the off-canonical migration lands clean (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Off-canonical source: the canonical root `CHANGELOG.md` is PROMOTED (Added) and the
    // old folder-home original is RETIRED (Deleted) — source-path != promote-destination,
    // so the in-place exclusion does NOT fire.
    let delta = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        delta.lines().any(|l| l == "A\tCHANGELOG.md"),
        "the off-canonical migration promotes the canonical root doc (Added):\n{delta}",
    );
    assert!(
        delta.lines().any(|l| l == "D\tdocs/changelog/changelog.md"),
        "the off-canonical original is retired (Deleted):\n{delta}",
    );

    // The committed doc is the authored doc (byte-stable, no Frankenstein).
    let committed = fs::read_to_string(repo.path().join("CHANGELOG.md"))
        .expect("the canonical changelog is on disk after finalize");
    assert_eq!(
        committed, staged,
        "the committed doc equals the authored doc"
    );
    assert!(
        !committed.contains("non-conformant prior content"),
        "the committed doc carries none of the off-canonical source body:\n{committed}",
    );
}

/// RED — the AT-canonical adopt-in-place case (#8 / M38 T2). A foreign root `CHANGELOG.md`
/// already sits at the changelog's canonical **placement** home (post-M38 the doctype homes
/// at root `CHANGELOG.md`, no folder). `jigc migrate CHANGELOG.md` records a `source-path`
/// == the promote destination, so it IS the in-location squatter: create seeds the working
/// area BLANK (never the foreign body), the batch authors end-to-end, and finalize ADOPTS
/// IN PLACE — the committed doc round-trips byte-stable and git name-status shows
/// `M CHANGELOG.md` (Modified), NEVER `D` (the retire is skipped — source == destination —
/// so the migration never self-deletes the doc it just wrote). RED before T2's create-side
/// placement fix: the copy-in branch read the foreign body in as the edit base (a
/// Frankenstein doc), because the create-side squatter discriminator derived its destination
/// from the (now-absent) `location` and missed the `placement.file` home.
#[test]
fn flow26_at_canonical_root_changelog_adopts_in_place() {
    let repo = TempDir::new("in-place");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // The foreign root `CHANGELOG.md` IS the canonical placement destination, so the
    // per-file migration id folds the source stem into `migrate-changelog-changelog`.
    const AT_CANONICAL_TASK: &str = "migrate-changelog-changelog";
    const SQUATTER: &str =
        "# Whatever\n\nnon-conformant foreign changelog sitting at the canonical root\n";
    setup_and_migrate(repo.path(), home.path(), &pack, "CHANGELOG.md", SQUATTER);

    const PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.0.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;
    author_via_batch(repo.path(), home.path(), &pack, AT_CANONICAL_TASK, PAYLOAD);

    // Create seeded BLANK: the foreign body was never copied in as the edit base.
    let staged = staged_changelog(repo.path(), AT_CANONICAL_TASK);
    assert!(
        !staged.contains("non-conformant foreign changelog"),
        "the AT-canonical foreign body must NOT be copied in as the edit base (no Frankenstein doc):\n{staged}",
    );
    assert!(
        staged.contains("### 1.0.0") && staged.contains("First public release."),
        "the batch authors onto the clean blank skeleton:\n{staged}",
    );
    assert_byte_stable(&pack, &staged);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", AT_CANONICAL_TASK, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "finalize --approve on the AT-canonical migration lands clean (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Adopt IN PLACE: the canonical root `CHANGELOG.md` is MODIFIED (rewritten from the
    // foreign body to the managed doc), NEVER Deleted — the retire is skipped because
    // source-path == promote-destination (the in-location squatter exclusion), so the
    // migration never self-deletes the doc it just wrote.
    let delta = git(repo.path(), &["show", "--name-status", "--format=", "HEAD"]);
    assert!(
        delta.lines().any(|l| l == "M\tCHANGELOG.md"),
        "the AT-canonical migration modifies the canonical root doc in place (Modified):\n{delta}",
    );
    assert!(
        !delta.lines().any(|l| l == "D\tCHANGELOG.md"),
        "the in-location squatter is NEVER self-deleted (no `D CHANGELOG.md`):\n{delta}",
    );

    // The committed doc is the authored doc, byte-stable, carrying none of the foreign body.
    let committed = fs::read_to_string(repo.path().join("CHANGELOG.md"))
        .expect("the canonical changelog is on disk after finalize");
    assert_eq!(
        committed, staged,
        "the committed doc equals the authored doc"
    );
    assert!(
        !committed.contains("non-conformant foreign changelog"),
        "the committed doc carries none of the foreign source body:\n{committed}",
    );
}

/// RED — the structural release-delta summary (#5). A batch that authors fewer releases
/// than the source carries surfaces the dropped version at the review gate, labeled
/// fuzzy/advisory — feeding nothing structural (the gate still exits 4).
#[test]
fn flow26_red_review_gate_shows_structural_release_delta_summary() {
    let repo = TempDir::new("delta");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    setup_and_migrate(repo.path(), home.path(), &pack, "HISTORY.md", FOREIGN);

    // Author only 1.2.0 + 1.1.0 from the one batch — drop the source's 1.0.0.
    const DROPPED_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        sections:
          - id: changes
            items:
              - title: Changed
                set:
                  notes: "<<- Faster cold start.>>"
      - title: 1.1.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- Device-code OAuth flow.>>"
"#;
    author_via_batch(repo.path(), home.path(), &pack, TASK, DROPPED_PAYLOAD);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );
    // The summary feeds nothing structural: the gate still exits 4 (review-pending).
    assert_eq!(
        out.status.code(),
        Some(4),
        "the release-delta summary must not change the gate's exit code; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let summary = rendered
        .lines()
        .find(|l| l.contains("source releases absent from the rewrite"))
        .unwrap_or_else(|| {
            panic!("the gate must render the release-delta summary; got:\n{rendered}")
        });
    assert!(
        summary.contains("1.0.0"),
        "the summary names the dropped 1.0.0; got:\n{summary}",
    );
    let lower = summary.to_lowercase();
    assert!(
        lower.contains("fuzzy") || lower.contains("heuristic"),
        "the summary carries a fuzzy/heuristic label (Framing A — never a second \
         structural authority); got:\n{summary}",
    );
    assert!(
        !summary.contains("1.2.0") && !summary.contains("1.1.0"),
        "the kept releases must not be reported as dropped; got:\n{summary}",
    );
}
