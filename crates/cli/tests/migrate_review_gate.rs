//! M23 Increment 3, T1 — the `--approve` review gate (block-without-approve).
//!
//! On a **migration task** ([auto-migration.md](../../../design/auto-migration.md) →
//! The review gate), `jigc task finalize <id>` *without* `--approve` renders the
//! **fidelity diff** (the staged source-seam bytes vs the staged canonical doc) and
//! **exits non-zero, committing nothing** — the foreign original is byte-intact and
//! nothing is adopted. `--approve` is the human's only check on rewrite fidelity (the
//! strict parse guarantees *structure*, never *content-faithfulness*).
//!
//! The omitting-context arm: a **non-migration** finalize ignores `--approve` (the gate
//! is inert — the existing transactional path runs unchanged).
//!
//! Drives the built `jigc` binary against a throwaway `git init` temp repo over the
//! shipped dev pack. Retire + commit + adopt are later tasks of this increment — not
//! asserted here.

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
            "jigc-reviewgate-{tag}-{}-{:?}",
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

/// Assert a `jigc` invocation exits 0, returning its trimmed stdout.
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

/// The off-router migration task id — `migrate` mints a per-file `migrate-<doctype>-<slug(path)>` (the empty
/// intent slugs the `migrate-` id-source fallback), keeping the bare `changelog`
/// namespace free (`auto-migration.md` -> Hardening #9).
const TASK: &str = "migrate-changelog-history-3268e06b69e1";

/// Fill every author-required field/slot of the provisioned commit doc for `task` so a
/// `finalize` over it validates clean.
fn make_commit_conformant(repo: &Path, home: &Path, pack: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &["doc", "set-field", addr, "--value", value, "--task", task],
                None,
            ),
            "set-field commit",
        );
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
                Some(prose),
            ),
            "set-slot commit",
        );
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "changelog");
    set_slot(
        &format!("commit:{task}#summary"),
        b"adopt the migrated changelog\n",
    );
    set_slot(
        &format!("commit:{task}#body"),
        b"Migrate the foreign HISTORY.md into managed shape.\n",
    );
}

/// Drive the migrate + author spine to a conformant staged `changelog:changelog` whose
/// releases are exactly `release_titles` (in order), over `foreign`, plus a conformant
/// commit doc — the state finalize gates. Authoring fewer releases than `foreign` carries
/// is the dropped-release case the fidelity summary surfaces.
fn staged_migration(
    repo: &Path,
    home: &Path,
    pack: &Path,
    foreign_name: &str,
    foreign: &str,
    release_titles: &[&str],
) -> String {
    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    fs::write(repo.join(foreign_name), foreign).expect("write the foreign original");
    // M51 Inc 1 / T2 — the migrate door takes only a source git holds a copy of.
    git(repo, &["add", "--", foreign_name]);
    let composed = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", foreign_name, "--as", "changelog"],
            None,
        ),
        "jigc migrate",
    );
    // The minted id is read off the compose rather than assumed: the migration task id
    // carries a hash of the source path (M44), so it differs per foreign original.
    let task = composed
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .expect("the compose announces the minted task id")
        .trim()
        .to_owned();
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "create",
                "changelog",
                "--title",
                "Changelog",
                "--task",
                &task,
            ],
            None,
        ),
        "doc create changelog",
    );
    for title in release_titles {
        let release = ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "add-item",
                    "changelog:changelog#releases",
                    "--title",
                    title,
                    "--task",
                    &task,
                ],
                None,
            ),
            "add-item release",
        );
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "set-field",
                    &format!("{release}/date"),
                    "--value",
                    "2021-03-09",
                    "--task",
                    &task,
                ],
                None,
            ),
            "set-field date",
        );
        let group = ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "add-item",
                    &format!("{release}/changes"),
                    "--title",
                    "Added",
                    "--task",
                    &task,
                ],
                None,
            ),
            "add-item change-group",
        );
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "set-slot",
                    &format!("{group}/notes"),
                    "--from-file",
                    "-",
                    "--task",
                    &task,
                ],
                Some(b"First public release.\n"),
            ),
            "set-slot notes",
        );
    }
    make_commit_conformant(repo, home, pack, &task);
    task
}

const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

#[test]
fn migration_finalize_without_approve_blocks_and_commits_nothing() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    staged_migration(
        repo.path(),
        home.path(),
        &pack,
        "HISTORY.md",
        FOREIGN,
        &["0.1.0"],
    );

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let log_before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let foreign_before = fs::read(repo.path().join("HISTORY.md")).expect("read foreign before");

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );

    // The review gate exits with a DISTINCT review-pending code (not 0, not the
    // validation-blocked 3, not the operational 1) — the build-pin EXIT_REVIEW_PENDING.
    assert_eq!(
        out.status.code(),
        Some(4),
        "finalize without --approve on a migration task must exit 4 (review-pending); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The fidelity diff shows BOTH inputs: the foreign source (its KaC bracket heading,
    // present only in the foreign file) and the canonical rewrite (labeled by its
    // destination path).
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        rendered.contains("## [0.1.0] - 2021-03-09"),
        "the fidelity diff must surface the foreign source bytes; got:\n{rendered}"
    );
    assert!(
        rendered.contains("CHANGELOG.md"),
        "the fidelity diff must surface the canonical rewrite (its destination); got:\n{rendered}"
    );
    assert!(
        rendered.contains("--approve"),
        "the block must tell the human how to approve; got:\n{rendered}"
    );

    // Nothing committed: HEAD unchanged, no new commit, no `CHANGELOG.md` in HEAD.
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "a blocked review must leave HEAD unchanged"
    );
    assert_eq!(
        log_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        "a blocked review must create no commit"
    );
    let show = Command::new("git")
        .args(["show", "HEAD:CHANGELOG.md"])
        .current_dir(repo.path())
        .output()
        .expect("run git show");
    assert!(
        !show.status.success(),
        "a blocked review must commit no canonical changelog"
    );

    // The foreign original is byte-intact (rejection is byte-safe).
    assert_eq!(
        foreign_before,
        fs::read(repo.path().join("HISTORY.md")).expect("read foreign after"),
        "a blocked review must leave the foreign original untouched"
    );

    // Nothing adopted: the adopt path's file-state hash for the canonical committed
    // path is never recorded (the swept record is dropped on the gated branch).
    let record = repo
        .path()
        .join(".jigc")
        .join("state")
        .join("file-state.json");
    if let Ok(json) = fs::read_to_string(&record) {
        assert!(
            !json.contains("CHANGELOG.md"),
            "a blocked review must not adopt (no file-state baseline for the canonical doc); \
             got:\n{json}"
        );
    }

    // The working area survives for the approve re-run.
    assert!(
        repo.path().join(".jigc").join("tasks").join(TASK).exists(),
        "a blocked review must keep the working area for the --approve re-run"
    );
}

/// A foreign changelog carrying TWO releases — the dropped-release case: the agent's
/// canonical rewrite authors only `1.0.0`, silently dropping `0.9.0`.
const FOREIGN_MULTI: &str = "\
# Changelog

## [1.0.0] - 2022-01-01
### Added
- Stable release.

## [0.9.0] - 2021-06-01
### Added
- Beta release.
";

#[test]
fn review_gate_names_dropped_release_as_fuzzy_advisory_and_feeds_nothing_structural() {
    // M24 Inc 6 / T3 — the structural fidelity summary (auto-migration.md → Hardening #5;
    // DECISIONS C4). The foreign source carries 1.0.0 + 0.9.0; the authored canonical
    // rewrite carries only 1.0.0, so 0.9.0 is dropped. The review gate must surface the
    // release-delta as a FUZZY, ADVISORY aid — and feed NOTHING structural (the gate still
    // exits 4 regardless, and a follow-up `--approve` still proceeds to commit).
    let repo = TempDir::new("repo-delta");
    let home = TempDir::new("home-delta");
    let pack = dev_pack();
    init_repo(repo.path());
    staged_migration(
        repo.path(),
        home.path(),
        &pack,
        "HISTORY.md",
        FOREIGN_MULTI,
        &["1.0.0"],
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );
    // The delta feeds nothing structural: the gate still exits 4 (review-pending),
    // exactly as it would with no dropped release.
    assert_eq!(
        out.status.code(),
        Some(4),
        "the fidelity summary must not change the gate's exit code; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The advisory line names the dropped release (a bare version token) and the summary
    // is labeled fuzzy/heuristic (the fuzzy framing rides the fidelity header line).
    let summary_line = rendered
        .lines()
        .find(|l| l.contains("version-like token absent from the rewrite"))
        .unwrap_or_else(|| {
            panic!("the gate must render the release-delta summary; got:\n{rendered}")
        });
    assert!(
        summary_line.contains("0.9.0"),
        "the summary must name the dropped 0.9.0; got:\n{summary_line}"
    );
    let lower = rendered.to_lowercase();
    assert!(
        lower.contains("fuzzy") || lower.contains("heuristic"),
        "the summary must carry a fuzzy/heuristic label (Framing A — never a second \
         structural authority); got:\n{rendered}"
    );
    // The kept release is NOT listed as absent (the canonical side is precise).
    assert!(
        !summary_line.contains("1.0.0"),
        "the kept release must not be reported as dropped; got:\n{summary_line}"
    );

    // The summary feeds nothing structural: a follow-up `--approve` still proceeds to
    // commit, identical to a migration with no dropped release.
    let log_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let approved = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK, "--approve"],
        None,
    );
    assert!(
        approved.status.success(),
        "`--approve` must proceed to commit despite the dropped release; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approved.stdout),
        String::from_utf8_lossy(&approved.stderr),
    );
    let log_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "`--approve` must land exactly one commit (the delta gates nothing)"
    );
}

#[test]
fn review_gate_renders_none_when_nothing_dropped() {
    // M24 Inc 7 — the affirmative structural signal (auto-migration.md → Hardening #5;
    // worked-examples.md flow 26). On a fully-migrated source (the agent's rewrite keeps
    // every release the foreign source carries), the release-delta line must STILL render,
    // in the affirmative `(none)` form — an absent line is ambiguous (did the scan run? was
    // nothing dropped? is the feature active?). FOREIGN carries only 0.1.0 and the rewrite
    // authors 0.1.0, so nothing is dropped.
    let repo = TempDir::new("repo-none");
    let home = TempDir::new("home-none");
    let pack = dev_pack();
    init_repo(repo.path());
    staged_migration(
        repo.path(),
        home.path(),
        &pack,
        "HISTORY.md",
        FOREIGN,
        &["0.1.0"],
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", TASK],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(4),
        "finalize without --approve on a migration task must exit 4 (review-pending); \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // The affirmative form renders verbatim (worked-examples.md flow 26): the bare
    // version-token category reports `(none)` when nothing is dropped.
    let summary_line = rendered
        .lines()
        .find(|l| l.contains("version-like token absent from the rewrite"))
        .unwrap_or_else(|| {
            panic!(
                "the gate must render the release-delta summary on the happy path; got:\n{rendered}"
            )
        });
    assert!(
        summary_line.contains("version-like token absent from the rewrite: (none)"),
        "the nothing-dropped path must render the affirmative `(none)` form; got:\n{summary_line}"
    );
    // Still labeled fuzzy/heuristic — the empty case stays inside the advisory framing
    // (the fuzzy framing rides the fidelity header line).
    let lower = rendered.to_lowercase();
    assert!(
        lower.contains("fuzzy") || lower.contains("heuristic"),
        "the `(none)` line must still carry the fuzzy/heuristic label; got:\n{rendered}"
    );
}

#[test]
fn non_migration_finalize_ignores_approve() {
    // The omitting-context arm: a plain single-task finalize WITH --approve runs the
    // existing transactional path unchanged (the gate is inert with no source seam).
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "setup",
    );

    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "single-task", "add rate limiter"],
            None,
        ),
        "jigc start",
    );
    let task = "add-rate-limiter";
    fs::write(repo.path().join("limiter.rs"), "// limiter\n").expect("write code change");
    git(repo.path(), &["add", "limiter.rs"]); // M30 G5 — the agent stages its own edit.
    make_commit_conformant(repo.path(), home.path(), &pack, task);

    let log_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", task, "--approve"],
        None,
    );
    assert!(
        out.status.success(),
        "a non-migration finalize must ignore --approve and land (exit 0); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let log_after: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "a non-migration finalize --approve must land exactly ONE commit (the existing path)"
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        !rendered.contains("review required") && !rendered.contains("fidelity"),
        "the review gate must be inert on a non-migration task; got:\n{rendered}"
    );
}

/// Parse the one JSON document a `--format json` invocation wrote to stdout.
fn json_doc(out: &std::process::Output, what: &str) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|err| {
        panic!(
            "`{what}` must write exactly one JSON document to stdout ({err}); stdout:\n{stdout}\nstderr:\n{}",
            String::from_utf8_lossy(&out.stderr),
        )
    })
}

/// The review hold's headline — the single line that carries the `--approve` re-run
/// instruction, i.e. the sentence that promises what approving will do.
fn headline(rendered: &str) -> &str {
    rendered
        .lines()
        .find(|l| l.contains("--approve"))
        .unwrap_or_else(|| {
            panic!("the review hold must tell the human how to approve; got:\n{rendered}")
        })
}

#[test]
fn the_review_hold_names_the_file_approve_will_delete() {
    // M51 Increment 1 / T7 (`settle-record.md` → D1 part 4) — the exit-4 hold is a
    // HUMAN gate over a destructive act, and at HEAD it said "retire the foreign
    // original" without naming it: the one fact a reviewer needs in order to consent to
    // a deletion was on no surface. The hold now names the file `--approve` will delete,
    // in its text AND as the additive `retires` key of the pinned envelope, read off the
    // same `ValidatedRetirement` the sink unlinks from — so the gate and the sink cannot
    // name different files.
    let repo = TempDir::new("repo-retires");
    let home = TempDir::new("home-retires");
    let pack = dev_pack();
    init_repo(repo.path());
    let task = staged_migration(
        repo.path(),
        home.path(),
        &pack,
        "HISTORY.md",
        FOREIGN,
        &["0.1.0"],
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(4),
        "the plain finalize over a migration task holds at the review gate; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let line = headline(&rendered);
    assert!(
        line.contains("HISTORY.md"),
        "the sentence that promises the deletion must name the file it deletes; got:\n{line}",
    );

    // The same fact on the machine surface, as a declared key carrying the adjudicated
    // repo-relative path — a driver must not have to scrape the prose for it.
    let json = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    assert_eq!(
        json.status.code(),
        Some(4),
        "the `--format json` hold exits 4 too; stderr:\n{}",
        String::from_utf8_lossy(&json.stderr),
    );
    let doc = json_doc(&json, "jigc task finalize --format json (review hold)");
    assert_eq!(
        doc.get("retires"),
        Some(&serde_json::json!(["HISTORY.md"])),
        "the hold's envelope carries the retire set as the additive `retires` key; got:\n{doc:#}",
    );

    // The hold is still a hold: it deletes nothing it names.
    assert!(
        repo.path().join("HISTORY.md").exists(),
        "naming the file must not delete it — the hold commits and destroys nothing",
    );
}

#[test]
fn the_same_path_review_hold_claims_no_deletion() {
    // The omitting context (the in-place, same-path migration: the foreign original IS
    // the canonical destination, so `plan_retirements` skips it). The hold must carry
    // the HONEST EMPTY form on both surfaces — an empty `retires` key and a sentence
    // that promises no deletion — never the retire sentence with nothing behind it.
    let repo = TempDir::new("repo-inplace");
    let home = TempDir::new("home-inplace");
    let pack = dev_pack();
    init_repo(repo.path());
    let task = staged_migration(
        repo.path(),
        home.path(),
        &pack,
        "CHANGELOG.md",
        FOREIGN,
        &["0.1.0"],
    );

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(4),
        "the same-path migration holds at the review gate too; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let rendered = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let line = headline(&rendered);
    assert!(
        !line.contains("DELETE"),
        "an in-place rewrite retires nothing — the hold must not promise a deletion; got:\n{line}",
    );
    assert!(
        line.contains("deletes nothing"),
        "the empty form must be affirmative (an absent clause is ambiguous); got:\n{line}",
    );

    let json = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task, "--format", "json"],
        None,
    );
    let doc = json_doc(&json, "jigc task finalize --format json (same-path hold)");
    assert_eq!(
        doc.get("retires"),
        Some(&serde_json::json!([])),
        "the key is present and EMPTY on the in-place arm — absent would be an unanswered \
         question, not an honest empty; got:\n{doc:#}",
    );
}

#[test]
fn the_hold_refuses_a_retire_target_it_cannot_name_honestly() {
    // The third arm of the same rule: the hold states what `--approve` WILL do, and when
    // the recorded `source-path` has been rewritten to something the repository cannot
    // retire, what `--approve` will do is refuse. Naming it as "the file I will delete"
    // would be a promise the next run cannot keep, so the hold raises the sink's own
    // identity instead — one code, one repair — with the state-truth clause of THIS door
    // (M47's per-door rule): the hold has promoted nothing, so it must not narrate a
    // rollback it never performed.
    let repo = TempDir::new("repo-refuse");
    let home = TempDir::new("home-refuse");
    let outside = TempDir::new("outside-refuse");
    let pack = dev_pack();
    init_repo(repo.path());
    let task = staged_migration(
        repo.path(),
        home.path(),
        &pack,
        "HISTORY.md",
        FOREIGN,
        &["0.1.0"],
    );

    // A file in another tree entirely — the thing a naive `repo_root.join(<absolute>)`
    // resolves to, and the thing that must still be here afterwards.
    let canary = outside.path().join("keepme.md");
    fs::write(&canary, "keep me\n").expect("plant the canary outside the repository");
    fs::write(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(&task)
            .join("source-path"),
        canary.to_string_lossy().as_bytes(),
    )
    .expect("rewrite the recorded source path");

    // `--carry-staged` because rewriting the recorded source path also removes this
    // fixture's foreign original from the carryover gate's retire-exempt set, and that
    // gate sits AHEAD of the review hold by design. Declaring the carry is what lets the
    // run reach the door this arm is about; it changes nothing the arm asserts.
    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task, "--carry-staged"],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(1),
        "the hold refuses at the destroying-door mold's exit 1; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        stderr.contains("finalize.retire-untrackable"),
        "the hold raises the sink's own identity, not a new one; got:\n{stderr}",
    );
    assert!(
        !stderr.contains("rolled back"),
        "the hold promoted nothing, so its route must not narrate a rollback; got:\n{stderr}",
    );
    assert!(
        !String::from_utf8_lossy(&out.stdout).contains("migration review required"),
        "a hold that cannot state the consequence must not print the consent prompt anyway",
    );

    assert_eq!(
        fs::read_to_string(&canary).expect("read the canary"),
        "keep me\n",
        "the out-of-tree canary must be byte-identical — the hold reads, it never deletes",
    );
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "the refusing hold leaves HEAD unmoved",
    );
}
