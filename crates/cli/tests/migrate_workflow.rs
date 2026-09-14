//! M23 Increment 1, T3 — the `migrate-changelog` workflow + its `author-migration`
//! step, shipped in the dev pack (no project shadow).
//!
//! Done-criterion (T3): `jigc migrate <foreign CHANGELOG.md> --as changelog` composes
//! the **shipped** `migrate-changelog` workflow (NOT a test shadow) and the emitted
//! view carries both
//!   - the resolved foreign content (the source seam — `{{ source }}` surfaces the
//!     staged foreign bytes verbatim), and
//!   - the author-spine command guidance (the declarative `jigc doc author changelog
//!     --from-file` batch directive + the category map+merge / infer-from-prefix / dateless
//!     clauses + finalize directions),
//!
//! and the workflow + step load and pass the compose-time workflow-refs gate (a clean
//! exit 0 — a dangling command-ref or step-include would block composition).
//!
//! The seam feed itself is T2's contract; here the contract is that the **shipped pack
//! workflow** composes (so the verb works without the T2 shadow) and surfaces the
//! author spine — including the historical-`date` overwrite guidance (Grouped scope).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-migrate-wf-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Stage a foreign migration source. **M51 Increment 1 / T2**: `jigc migrate` refuses a source
/// git holds no copy of — in neither the index nor `HEAD` — and routes at exactly this
/// `git add`, so every fixture that hands the door a freshly written file stages it first.
fn stage(root: &Path, path: &str) {
    let out = Command::new("git")
        .args(["add", "--", path])
        .current_dir(root)
        .output()
        .expect("run git add");
    assert!(
        out.status.success(),
        "git add -- {path} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Initialize a real git repo with one commit (composition reads HEAD) plus the
/// `.jigc/config/` project layer the cascade expects.
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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
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
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

/// Run a `jigc` subcommand over the EMBEDDED pack pair: `JIGC_PACK_DIR` is cleared
/// (an empty value is not an explicit base selection), so the setup-written
/// `compose-embedded-methodology` marker fires and the composition is the production
/// `[dev ▸ methodology]` every set-up project runs — the full 12-member migratable set.
fn run_jigc_embedded(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", "")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the jigc binary")
        .wait_with_output()
        .expect("wait for jigc")
}

/// The full migratable set — every doctype with a shipped `migrate-<doctype>` workflow
/// across the composed `[dev ▸ methodology]` pair (dev 5 + methodology 7 = 12),
/// address-sorted as the rejection message renders it.
const MIGRATABLE_SET: &str = "adr, arch-doc, changelog, completion-record, decisions-log, \
                              deferral-ledger, idea, prd, research, roadmap, spec, vision";

/// A realistic multi-release foreign Keep-a-Changelog file carrying HISTORICAL dates.
const FOREIGN: &str = "\
# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2023-01-15
### Added
- Device-code OAuth flow.
### Fixed
- Session fixation on logout.

## [1.1.0] - 2022-08-01
### Changed
- Bumped the default timeout to 30s.
";

#[test]
fn migrate_composes_the_shipped_workflow_with_seam_and_author_spine() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");

    // No project shadow: the SHIPPED pack `migrate-changelog` workflow + its
    // `author-migration` step must compose on their own.
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    stage(repo.path(), "CHANGELOG.md");

    // A clean exit 0 IS the workflow-refs gate passing — a dangling command-ref or a
    // missing step-include would block composition and exit non-zero.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    let stdout = ok_stdout(out, "jigc migrate CHANGELOG.md --as changelog");

    // (1) The source seam resolved: the foreign content is surfaced verbatim.
    assert!(
        stdout.contains(FOREIGN.trim_end()),
        "the composed shipped workflow must surface the foreign content (the source seam); stdout:\n{stdout}",
    );

    // (2) The author spine is the declarative BATCH directive: the whole canonical doc
    // is authored in ONE payload via `jigc doc author changelog --from-file` (the inc-1 verb,
    // create + every leaf over one staged buffer), NOT the per-leaf create/add-item walk.
    assert!(
        stdout.contains("jigc doc author changelog --from-file"),
        "the declarative batch directive must be present; stdout:\n{stdout}",
    );

    // (3) The category map+merge instruction: map each foreign category onto exactly one
    // of the six enum members AND merge many-to-one (the enum is the id-from, so two
    // foreign categories collapsing onto one member share its single group). Grouped scope.
    for member in [
        "added",
        "changed",
        "deprecated",
        "removed",
        "fixed",
        "security",
    ] {
        assert!(
            stdout.contains(member),
            "the category enum member `{member}` must be surfaced; stdout:\n{stdout}",
        );
    }
    assert!(
        stdout.contains("merge"),
        "the many-to-one MERGE instruction must be present; stdout:\n{stdout}",
    );

    // (4) The infer-from-prefix instruction: when a foreign file carries NO category
    // headings, infer the member from each change's `feat:` / `fix:` commit prefix.
    assert!(
        stdout.contains("feat:") && stdout.contains("fix:"),
        "the infer-from-prefix-when-headingless instruction must be present; stdout:\n{stdout}",
    );

    // (5) The historical-date clause survives the batch rewrite: provide the FOREIGN
    // historical `date` when present, OMIT it when the source is dateless (no fabricated
    // history). Grouped scope.
    assert!(
        stdout.contains("date") && stdout.contains("omit"),
        "the historical-date / dateless-omit guidance must be present; stdout:\n{stdout}",
    );

    // (6) The finalize step composed in (the spine ends at finalize).
    assert!(
        stdout.contains("jigc task finalize") || stdout.contains("finalize"),
        "the finalize step must compose in; stdout:\n{stdout}",
    );
}

/// A realistic foreign Nygard/MADR-shaped ADR — the four canonical headings plus an
/// in-enum status. The body references another decision (no edge home this increment:
/// the guidance must route it to prose, not author a `supersedes` field).
const FOREIGN_ADR: &str = "\
# Use PostgreSQL for primary storage

## Status
Accepted

## Context
We need a relational store with strong consistency guarantees and mature tooling,
superseding the earlier key-value sketch in ADR 0001.

## Options
Alternatives were weighed and rejected.

## Decision
We will use PostgreSQL 15 as the primary data store for the service.

## Consequences
Operational familiarity is high; we accept the cost of running a managed instance.
";

#[test]
fn migrate_adr_composes_the_shipped_workflow_with_seam_and_adr_author_spine() {
    // T2 (M25 inc-1): `jigc migrate <adr>.md --as adr` composes the SHIPPED `migrate-adr`
    // workflow (NOT a test shadow) — the source seam surfaces the foreign ADR verbatim
    // and the composed view carries the adr-specific author guidance. A clean exit 0 IS
    // the workflow-refs gate passing (a dangling step-include or command-ref blocks it).
    let repo = TempDir::new("adr-repo");
    let home = TempDir::new("adr-home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");

    // No project shadow: the SHIPPED `migrate-adr` workflow + its `author-migration-adr`
    // step must compose on their own. Cold spike — no `docs/decisions/` exists yet.
    fs::write(repo.path().join("decision.md"), FOREIGN_ADR).expect("write foreign ADR");
    stage(repo.path(), "decision.md");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "decision.md", "--as", "adr"],
    );
    let stdout = ok_stdout(out, "jigc migrate decision.md --as adr");

    // (1) The source seam resolved: the foreign ADR is surfaced verbatim.
    assert!(
        stdout.contains(FOREIGN_ADR.trim_end()),
        "the composed migrate-adr workflow must surface the foreign ADR (the source seam); stdout:\n{stdout}",
    );

    // (2) The adr author spine is the declarative batch directive against the `adr`
    // doctype (NOT the changelog one) — create + every leaf over one staged buffer.
    assert!(
        stdout.contains("jigc doc author adr --from-file"),
        "the adr-specific batch directive must be present; stdout:\n{stdout}",
    );

    // (3) The Status/Context/Decision/Consequences heading map (the adr schema's four
    // canonical parts).
    for heading in ["Status", "Context", "Decision", "Consequences"] {
        assert!(
            stdout.contains(heading),
            "the adr heading `{heading}` must be surfaced in the map; stdout:\n{stdout}",
        );
    }

    // (3a) M43 Inc-3 T4 — the generated `{{schema:adr}}` projection replaces the
    // hand-enumerated skeleton (law 1: a template cannot understate the schema). The
    // composed view names ALL FIVE adr sections — including `options`, the slot the
    // hand-written template famously omitted — at the RESOLVED home (docs-root
    // applied), and the "fixed four-part" lie is gone.
    for section in [
        "- `status` (front-matter fields):",
        "- `context`: prose slot",
        "- `options`: prose slot (optional)",
        "- `decision`: prose slot",
        "- `consequences`: prose slot",
    ] {
        assert!(
            stdout.contains(section),
            "the generated projection must name {section:?}; stdout:\n{stdout}",
        );
    }
    assert!(
        stdout.contains("`docs/decisions/<slug>.md`"),
        "the adr home renders resolved through docs-root, never the schema-raw \
         `decisions/`; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("fixed four-part"),
        "the hand-enumerated `fixed four-part schema` lie must be gone; stdout:\n{stdout}",
    );

    // (4) The foreign-status → 3-enum mapping: proposed / accepted / superseded.
    for member in ["proposed", "accepted", "superseded"] {
        assert!(
            stdout.contains(member),
            "the status enum member `{member}` must be surfaced; stdout:\n{stdout}",
        );
    }

    // (5) M25 Increment 2 (T3): the EDGE-FREE prose block is REPLACED with the
    // `supersedes` edge guidance. The view no longer carries the edge-free disclaimer.
    assert!(
        !stdout.contains("EDGE-FREE"),
        "the edge-free disclaimer must be gone — Increment 2 wires the supersedes edge; stdout:\n{stdout}",
    );
    // (5a) The in-set supersession is authored as a typed bracket-list under the status section.
    assert!(
        stdout.contains("supersedes: \"[adr:"),
        "the bracket-list `supersedes: \"[adr:<slug>, …]\"` form must be shown; stdout:\n{stdout}",
    );
    // (5b) The dependency-ordering contract: finalize the target before the ADR that supersedes it.
    assert!(
        stdout.contains("before the ADR that supersedes it"),
        "the dependency-ordering contract must be present; stdout:\n{stdout}",
    );
    // (5c) The out-of-set drop: a target that will not be migrated folds into prose, never a
    // dangling ref.
    assert!(
        stdout.contains("will not be") && stdout.contains("prose"),
        "the out-of-set drop-to-prose escape must be present; stdout:\n{stdout}",
    );

    // (6) The finalize step composed in (the spine ends at finalize).
    assert!(
        stdout.contains("finalize"),
        "the finalize step must compose in; stdout:\n{stdout}",
    );

    // (7) Masking guard (inc-1/T3 class): the heredoc payload skeleton the agent actually
    // fills must still parse through the SAME parser `doc author` runs — a stray doc-level
    // key or a slot/field mismatch leaked by the edge guidance would silently break every
    // migration while this green compose test masks it.
    let skeleton = extract_author_skeleton(&stdout);
    let schema = shipped_adr_schema(&pack);
    let parsed = cli::author::parse_author_payload(Some(&schema), &skeleton);
    assert!(
        parsed.is_ok(),
        "the migrate-adr guidance payload skeleton must parse as a valid adr author payload; \
         skeleton:\n{skeleton}\nerror: {:?}",
        parsed.err(),
    );
}

/// The directory names under `.jigc/tasks/` (the single task-namespace enumeration) —
/// empty when the dir is absent. Lets a test assert that a rejected `migrate` minted no
/// orphan task.
fn task_dirs(repo: &Path) -> Vec<String> {
    let tasks = repo.join(".jigc").join("tasks");
    let mut ids: Vec<String> = match fs::read_dir(&tasks) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .filter_map(|e| e.file_name().into_string().ok())
            .collect(),
        Err(_) => Vec::new(),
    };
    ids.sort();
    ids
}

#[test]
fn migrate_with_an_unknown_as_rejects_before_minting_any_task() {
    // The state-pollution bug: `jigc migrate <path> --as <X>` where no `migrate-<X>`
    // workflow exists must reject with a clear message and mint NO task — the workflow
    // existence is validated BEFORE the mint, so a typo strands no orphan in
    // `jigc task list` (which can never compose or finalize).
    let repo = TempDir::new("reject-repo");
    let home = TempDir::new("reject-home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");

    fs::write(repo.path().join("STATE.md"), "# State\n").expect("write foreign file");

    let before = task_dirs(repo.path());

    // (a) An unknown doctype (a typo) — rejected over the production embedded
    // `[dev ▸ methodology]` composition, no task minted, the FULL 12-member
    // migratable set named.
    let out = run_jigc_embedded(
        repo.path(),
        home.path(),
        &["migrate", "STATE.md", "--as", "nonsense"],
    );
    assert!(
        !out.status.success(),
        "`jigc migrate --as nonsense` must exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unknown doctype `nonsense`"),
        "the message must name the unknown doctype; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("migratable doctypes: {MIGRATABLE_SET}")),
        "the message must name the full 12-member migratable set (dev 5 + \
         methodology 7); stderr:\n{stderr}",
    );
    assert_eq!(
        task_dirs(repo.path()),
        before,
        "a rejected migrate must mint NO task — `.jigc/tasks/` must be unchanged",
    );

    // (b) A known-but-not-migratable doctype (`commit` ships a schema but no
    // `migrate-commit` workflow) — distinguished message, still no task minted.
    let out = run_jigc_embedded(
        repo.path(),
        home.path(),
        &["migrate", "STATE.md", "--as", "commit"],
    );
    assert!(
        !out.status.success(),
        "`jigc migrate --as commit` must exit non-zero; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("commit") && stderr.contains("not migratable"),
        "a known-but-not-migratable doctype must say so; stderr:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("migratable doctypes: {MIGRATABLE_SET}")),
        "the not-migratable message must name the full 12-member migratable set; \
         stderr:\n{stderr}",
    );
    assert_eq!(
        task_dirs(repo.path()),
        before,
        "a rejected migrate of a known-but-not-migratable doctype must mint NO task",
    );

    // (c) The happy path still mints + stages: a VALID `--as changelog` composes and
    // leaves exactly one task behind (the validate-before-mint guard didn't break it).
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    stage(repo.path(), "CHANGELOG.md");
    let out = run_jigc_embedded(
        repo.path(),
        home.path(),
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    ok_stdout(out, "jigc migrate CHANGELOG.md --as changelog");
    assert_eq!(
        task_dirs(repo.path()).len(),
        before.len() + 1,
        "a valid migrate must mint exactly one task",
    );
}

/// The on-disk path of a task's workflow-provisioned `commit:<id>` doc:
/// `.jigc/tasks/<id>/docs/commit:<id>.md`.
fn commit_doc_path(repo: &Path, id: &str) -> PathBuf {
    repo.join(".jigc")
        .join("tasks")
        .join(id)
        .join("docs")
        .join(format!("commit:{id}.md"))
}

#[test]
fn reentered_filled_migrate_commit_survives_byte_untouched() {
    // M32 Inc-2 (T1) regression guard. Inc-1 widened `provision_on_first_entry` to fire
    // for every `creates-task: true` workflow (dropping the `|| !selectable` early-out),
    // which now reaches the `migrate-changelog` re-entry. The `migrate` path auto-provisions
    // the `commit:<id>` doc *FILLED* at mint (Hardening #4 — the templated migration summary
    // + body name the foreign source, so the no-author finalize composes a real message). The
    // property under guard: the `path.exists()` write-once guard (start.rs) makes the now-
    // ungated `provision_commit_doc` a NO-OP on re-entry — the empty fillable form NEVER
    // clobbers the auto-authored migration prose (`FALLBACK_TYPE == schema.ty`, so the
    // guard checks the exact path the migrate provisioner wrote). This is a no-clobber
    // regression (passes before AND after the fix), not RED->GREEN.
    let repo = TempDir::new("reentry-repo");
    let home = TempDir::new("reentry-home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");

    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    stage(repo.path(), "CHANGELOG.md");

    // `jigc migrate` mints the `migrate-changelog-<slug>` task + auto-provisions a FILLED
    // `commit:<id>` doc.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    ok_stdout(out, "jigc migrate CHANGELOG.md --as changelog");

    // Exactly one task minted — the migrate task; its id is the directory name.
    let ids = task_dirs(repo.path());
    assert_eq!(
        ids.len(),
        1,
        "migrate must mint exactly one task; got {ids:?}"
    );
    let id = &ids[0];

    // Capture the FILLED commit doc bytes. Assert it is genuinely FILLED (the migration
    // prose, not the empty fillable) so the byte-identity check below has teeth — a guard
    // over an already-empty form would prove nothing about no-clobber.
    let doc_path = commit_doc_path(repo.path(), id);
    let before = fs::read(&doc_path).expect("read the auto-provisioned filled commit doc");
    let before_text = String::from_utf8(before.clone()).expect("utf-8 commit doc");
    assert!(
        before_text.contains("adopt CHANGELOG.md as a managed changelog")
            && before_text.contains("Migrate the foreign CHANGELOG.md"),
        "the migrate commit doc must be auto-authored FILLED (Hardening #4); doc:\n{before_text}",
    );

    // Re-enter the migration workflow: `jigc workflow migrate-changelog --task <id>` must
    // compose cleanly (exit 0). The widened `provision_on_first_entry` runs, but the
    // `path.exists()` guard short-circuits `provision_commit_doc` to a no-op.
    let reentry = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["workflow", "migrate-changelog", "--task", id],
    );
    ok_stdout(
        reentry,
        "jigc workflow migrate-changelog --task <id> (re-entry)",
    );

    // The contract: the filled commit doc is BYTE-IDENTICAL after re-entry — the empty
    // fillable never overwrote the auto-authored migration prose.
    let after = fs::read(&doc_path).expect("read the commit doc after re-entry");
    assert_eq!(
        before, after,
        "a re-entered filled migrate commit doc must survive byte-untouched; \
         the empty fillable clobbered the migration prose",
    );
}

// === Provisioning property census (M32 Inc-2 / T2) ==========================
//
// The commit-doc provisioning property fans out across FOUR siblings, all governed
// by the one shared `should_provision_commit_doc` predicate (`creates_task`) and its
// `path.exists()` write-once safety (`start.rs`). Inc-1 widened the property by
// dropping the `selectable` gate, so the regression surface is the full census, not
// any single arm:
//
//   1. mint arm            — `compose_core` on a fresh `jigc start`: provisions the
//                            EMPTY fillable form (`provision_commit_doc`) at mint.
//   2. re-entry first-touch — `provision_on_first_entry`, `jigc workflow <W> --task`:
//                            provisions the empty fillable on the FIRST entry that
//                            finds the doc missing, then no-ops forever after via
//                            `path.exists()` (the T1 guard:
//                            `reentered_filled_migrate_commit_survives_byte_untouched`).
//   3. migrate dispatch    — `provision_migration_commit_doc`, `jigc migrate`:
//                            provisions a FILLED commit doc at mint (Hardening #4 — the
//                            templated migration prose, so the no-author finalize
//                            composes a real message).
//   4. RESUME              — `resume_in_repo` -> `compose_task_workflow(provision=false)`,
//                            `jigc start --task <id>`: provisions NOTHING. The omitting
//                            context — `provision == false` short-circuits before the
//                            predicate is ever consulted, so a resume never re-touches
//                            an already-provisioned doc. Guarded below
//                            (`selectable_false_migrate_resume_short_circuits_byte_untouched`).
//
// Siblings 1 + 3 write; sibling 2 writes once then is inert; sibling 4 is inert by
// construction. The two regression guards here lock the no-clobber property over the
// two siblings that re-enter a task whose commit doc already exists (2 + 4) — both
// must leave the FILLED migrate prose byte-untouched. These are no-clobber guards
// (green before AND after the Inc-1 fix), not RED->GREEN.

#[test]
fn selectable_false_migrate_resume_short_circuits_byte_untouched() {
    // M32 Inc-2 (T2) regression guard — the RESUME sibling of the provisioning census.
    // `jigc migrate` mints a `selectable: false` `migrate-changelog-<slug>` task whose
    // `commit:<id>` doc is auto-provisioned FILLED at mint. A `jigc start --task <id>`
    // resume (`resume_in_repo` -> `compose_task_workflow` with `provision = false`)
    // composes the task's recorded workflow but provisions NOTHING — the resume path
    // never calls `provision_on_first_entry`, so the empty fillable can never reach the
    // already-filled migration doc. The contract: resume exits 0 and the commit doc is
    // byte-identical. No-clobber guard (green before AND after the fix), not RED->GREEN.
    let repo = TempDir::new("resume-repo");
    let home = TempDir::new("resume-home");
    let pack = dev_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    ok_stdout(setup, "jigc setup");

    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign CHANGELOG.md");
    stage(repo.path(), "CHANGELOG.md");

    // Mint the `selectable: false` migrate task + its auto-provisioned FILLED commit doc.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
    );
    ok_stdout(out, "jigc migrate CHANGELOG.md --as changelog");

    let ids = task_dirs(repo.path());
    assert_eq!(
        ids.len(),
        1,
        "migrate must mint exactly one task; got {ids:?}"
    );
    let id = &ids[0];

    // Capture the FILLED commit doc bytes — assert it is genuinely FILLED so the
    // byte-identity check below has teeth (a guard over an empty form proves nothing).
    let doc_path = commit_doc_path(repo.path(), id);
    let before = fs::read(&doc_path).expect("read the auto-provisioned filled commit doc");
    let before_text = String::from_utf8(before.clone()).expect("utf-8 commit doc");
    assert!(
        before_text.contains("adopt CHANGELOG.md as a managed changelog")
            && before_text.contains("Migrate the foreign CHANGELOG.md"),
        "the migrate commit doc must be auto-authored FILLED (Hardening #4); doc:\n{before_text}",
    );

    // Resume the task: `jigc start --task <id>` recomposes the recorded `migrate-changelog`
    // workflow with `provision = false`. A clean exit 0 — a `selectable: false` task
    // resumes like any other; resume short-circuits the provisioner entirely.
    let resume = run_jigc(repo.path(), home.path(), &pack, &["start", "--task", id]);
    ok_stdout(resume, "jigc start --task <id> (resume)");

    // The contract: the filled commit doc is BYTE-IDENTICAL after resume — resume
    // provisioned nothing, so the migration prose is untouched.
    let after = fs::read(&doc_path).expect("read the commit doc after resume");
    assert_eq!(
        before, after,
        "a `selectable: false` migrate commit doc must survive a resume byte-untouched; \
         resume must provision nothing (`provision == false`)",
    );
}

/// Extract the `doc author adr --from-file -` heredoc payload skeleton from the composed
/// migrate guidance (between the `<<'EOF'` opener and the standalone `EOF` terminator) —
/// the agent-facing artifact the LLM fills + pipes.
fn extract_author_skeleton(composed: &str) -> String {
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains("doc author adr --from-file") && line.contains("<<'EOF'") {
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
    panic!("no `doc author adr` heredoc skeleton in the composed migrate guidance:\n{composed}");
}

/// Load the SHIPPED `adr` schema with the `code-anchor → doc-code` pack type threaded in
/// (the schema only loads with that decl; mirrors the shipped pack).
fn shipped_adr_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("adr.yaml")).expect("read shipped adr schema");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
        hint: None,
    }];
    engine::schema::load_schema_with_types(&yaml, &types).expect("shipped adr schema loads")
}
