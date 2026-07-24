//! M43 Increment 2 / T3+T4 — the **carryover gate** at `jigc task finalize` and
//! `jigc milestone finalize`
//! (`design/surface-contract.md` → The carryover gate; the trial's #1-ranked v1 gate).
//!
//! A foreign change staged **before the task existed** (a pre-mint `git add` /
//! `git rm`) must not silently ride the task's whole-index commit. Finalize's
//! committing path refuses with **one blocking routed `finalize.carried-staged` per
//! carried path** (file-path target form — mid-carry the path may be foreign, with no
//! managed identity), overridable with `--carry-staged` (the `--approve` mold:
//! undecidable intent converted to a declared one; on a migration finalize it
//! **composes** with `--approve` — two independent declarations). The task's own
//! post-mint staging never trips the gate, a `--dry-run` still renders its forecast
//! (the refuse sits on the committing path only), and a migration's recorded retire
//! pathspec is exempt (that deletion is the task's own work).
//!
//! The **milestone arm** (T4): `jigc milestone create` is the shared checkout's
//! aggregate-index door — its create-time snapshot is what `milestone finalize`
//! refuses against, same code, same override flag. One honest-wording bound (law 1):
//! both aggregate channels build from throwaway indexes / dedicated worktrees over
//! targeted pathspecs and land via `--ff-only`, so a live-index foreign entry
//! structurally CANNOT ride the milestone commit — the finding says the entry **stays
//! staged across the boundary**, never that it would ride it (the refuse is the
//! declare-at-the-boundary rule, not a leak fix), and `--carry-staged` proceeds with
//! the entry left staged, uncommitted.
//!
//! The **labeled manifest** (T5): a carried entry riding the commit under a declared
//! `--carry-staged` is labeled **`carried-over`** at all four render sites — the
//! `--dry-run` forecast (text + JSON), the pre-commit print, and the landed manifest
//! text + JSON — over one pre-commit-computed set, so the forecast/landed
//! identical-set invariant holds by construction (labeling changes no set
//! membership).
//!
//! Every arm drives the real binary and asserts on the emitted findings envelope /
//! the landed git commit — never a reconstruction.

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
            "jigc-carryover-gate-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit (a tracked `README.md`).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write README");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, against the embedded packs.
fn jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
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

/// Run `jigc <args>` (no stdin), asserting exit 0.
fn ok(repo: &Path, home: &Path, args: &[&str], what: &str) {
    let out = jigc(repo, home, args, None);
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#type"),
            "--value",
            "feat",
        ],
        "doc set-field type",
    );
    ok(
        repo,
        home,
        &[
            "doc",
            "set-field",
            &format!("commit:{task}#scope"),
            "--value",
            "gate",
        ],
        "doc set-field scope",
    );
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_slot(&format!("commit:{task}#summary"), b"gate the carryover\n");
    set_slot(&format!("commit:{task}#body"), b"An M43 change.\n");
}

/// Mint a `single-task` task and do its work: write + `git add` a task edit
/// (post-mint staging — the task's own), fill the commit doc.
fn mint_and_work(repo: &Path, home: &Path, intent: &str, task: &str, edit: &str) {
    ok(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    fs::write(repo.join(edit), "pub fn work() {}\n").expect("write task edit");
    git(repo, &["add", edit]);
    fill_commit(repo, home, task);
}

/// The blocked finalize's pinned findings envelope (stdout, `--format json`),
/// asserting the run blocked with the validation exit (3).
fn blocked_findings(out: &std::process::Output, what: &str) -> Vec<serde_json::Value> {
    assert_eq!(
        out.status.code(),
        Some(3),
        "`{what}` must block with the validation exit (3); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
        panic!("`{what}` blocks with the pinned findings envelope on stdout ({e}); got:\n{stdout}")
    });
    value["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a `findings` array; got:\n{stdout}"))
        .clone()
}

/// The `finalize.carried-staged` findings in an envelope.
fn carried_staged(findings: &[serde_json::Value]) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == "finalize.carried-staged")
        .cloned()
        .collect()
}

/// Assert one carried-staged finding is blocking, keyed at `path` (the file-path
/// target form), and routed at both exits (unstage, or declare with `--carry-staged`).
fn assert_carried(finding: &serde_json::Value, path: &str) {
    assert_eq!(
        finding["severity"], "blocking",
        "the carryover refusal is blocking; got: {finding}"
    );
    assert_eq!(
        finding["key"]["target"], path,
        "the finding keys at the carried file path; got: {finding}"
    );
    let route = finding["route"]
        .as_str()
        .unwrap_or_else(|| panic!("the refusal carries a route; got: {finding}"));
    assert!(
        route.contains("--carry-staged") && route.contains("git restore --staged"),
        "the route names both exits (unstage, or `--carry-staged`); got: {route}"
    );
}

/// A **pre-mint staged add** blocks finalize — one routed blocking
/// `finalize.carried-staged` **per carried path** (two staged foreign files → exactly
/// two findings, each keyed at its own path) — while `--dry-run` still renders its
/// forecast (the refuse sits on the committing path only, after the dry-run branch).
#[test]
fn a_pre_mint_staged_add_blocks_with_one_routed_finding_per_path() {
    let repo = TempDir::new("add");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged adds — staged BEFORE the task exists.
    fs::write(repo.path().join("foreign-a.txt"), "not this task's work\n").expect("write a");
    fs::write(repo.path().join("foreign-b.txt"), "also not\n").expect("write b");
    git(repo.path(), &["add", "foreign-a.txt", "foreign-b.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    // The forecast still renders: the refuse must not block the `--dry-run` branch.
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--dry-run"],
        "jigc task finalize --dry-run",
    );

    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged adds)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(
        carried.len(),
        2,
        "exactly ONE finding per carried path (two foreign paths staged); got:\n{findings:#?}"
    );
    assert_carried(&carried[0], "foreign-a.txt");
    assert_carried(&carried[1], "foreign-b.txt");
    assert!(
        !carried_staged(&findings)
            .iter()
            .any(|f| f["key"]["target"] == "feature.rs"),
        "the task's own post-mint staged edit is never carried; got:\n{findings:#?}"
    );

    // Nothing committed: the refusal precedes any side effect.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "the blocked finalize commits nothing"
    );
}

/// A **pre-mint staged modify** of a tracked file blocks the same way.
#[test]
fn a_pre_mint_staged_modify_blocks() {
    let repo = TempDir::new("modify");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged modification of the tracked README.
    fs::write(repo.path().join("README.md"), "hello\nforeign edit\n").expect("modify README");
    git(repo.path(), &["add", "README.md"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged modify)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(carried.len(), 1, "one carried path; got:\n{findings:#?}");
    assert_carried(&carried[0], "README.md");
}

/// A **pre-mint staged deletion** (`git rm` before the mint) blocks too — the
/// staged-deletion half an entry-only snapshot is structurally blind to (the trial's
/// A7 case), and the message says it is a deletion.
#[test]
fn a_pre_mint_staged_deletion_blocks() {
    let repo = TempDir::new("delete");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // A second tracked file whose deletion the user pre-stages.
    fs::write(repo.path().join("doomed.txt"), "to be removed\n").expect("write doomed");
    git(repo.path(), &["add", "doomed.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "track doomed.txt"]);
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged deletion — staged BEFORE the task exists.
    git(repo.path(), &["rm", "-q", "doomed.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        ),
        "jigc task finalize (pre-mint staged deletion)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(carried.len(), 1, "one carried path; got:\n{findings:#?}");
    assert_carried(&carried[0], "doomed.txt");
    assert!(
        carried[0]["message"]
            .as_str()
            .expect("message is a string")
            .contains("deletion"),
        "the message names the deletion half; got: {}",
        carried[0]
    );
}

/// **Post-mint staging never trips the gate**: a task whose index was clean at mint
/// stages its own work and finalizes clean — no `--carry-staged` needed.
#[test]
fn post_mint_staging_never_trips_the_gate() {
    let repo = TempDir::new("post-mint");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task],
        "jigc task finalize (post-mint staging only)",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|l| l == "feature.rs"),
        "the task's own staged edit lands; files:\n{committed}"
    );
}

/// `--carry-staged` **lands the declared carryover**: the same pre-mint staged add
/// that blocks above rides the whole-index commit once declared.
#[test]
fn carry_staged_lands_the_declared_carryover() {
    let repo = TempDir::new("declared");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    fs::write(repo.path().join("foreign-a.txt"), "deliberately carried\n").expect("write a");
    git(repo.path(), &["add", "foreign-a.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--carry-staged"],
        "jigc task finalize --carry-staged",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed.lines().any(|l| l == "foreign-a.txt")
            && committed.lines().any(|l| l == "feature.rs"),
        "the declared carryover AND the task's own edit land in the one commit; files:\n{committed}"
    );
}

// ───────────────────────── the labeled manifest (T5) ─────────────────────────

/// The `kind` the JSON manifest carries for `path`, from a `manifest`-shaped array.
fn kind_of(manifest: &serde_json::Value, path: &str) -> String {
    manifest
        .as_array()
        .expect("manifest is an array")
        .iter()
        .find(|e| e["path"] == path)
        .unwrap_or_else(|| panic!("`{path}` is in the manifest; got:\n{manifest:#?}"))["kind"]
        .as_str()
        .expect("kind is a string")
        .to_string()
}

/// A pre-mint staged **add and deletion** render `carried-over` across the agent
/// surfaces — the `--dry-run` forecast (text **and** JSON) and, on the `--carry-staged`
/// run, both the **pre-commit print** (named before the commit lands) and the **landed
/// manifest text** — while the task's own post-mint staged edit keeps its `added` kind
/// (labeling changes no set membership).
#[test]
fn carried_entries_label_carried_over_across_the_agent_surfaces() {
    let repo = TempDir::new("label-agent");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // A second tracked file whose deletion the user pre-stages.
    fs::write(repo.path().join("doomed.txt"), "to be removed\n").expect("write doomed");
    git(repo.path(), &["add", "doomed.txt"]);
    git(repo.path(), &["commit", "-q", "-m", "track doomed.txt"]);
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    // The foreign pre-staged add AND deletion — staged BEFORE the task exists.
    fs::write(repo.path().join("foreign-a.txt"), "not this task's work\n").expect("write a");
    git(repo.path(), &["add", "foreign-a.txt"]);
    git(repo.path(), &["rm", "-q", "doomed.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    // Site 1a — the dry-run forecast, agent text.
    let dry = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--dry-run"],
        None,
    );
    assert!(dry.status.success(), "the dry-run forecast must render");
    let dry_text = String::from_utf8_lossy(&dry.stdout).to_string();
    for line in [
        "  carried-over foreign-a.txt",
        "  carried-over doomed.txt",
        "  added feature.rs",
    ] {
        assert!(
            dry_text.contains(line),
            "the forecast labels the carried entries and keeps the task's own kind \
             (wanted {line:?}); got:\n{dry_text}"
        );
    }
    assert!(
        !dry_text.contains("added foreign-a.txt") && !dry_text.contains("deleted doomed.txt"),
        "a carried entry is labeled, not double-listed under its raw kind; got:\n{dry_text}"
    );

    // Site 1b — the dry-run forecast, JSON.
    let dry_json = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--dry-run", "--format", "json"],
        None,
    );
    assert!(dry_json.status.success(), "the JSON forecast must render");
    let forecast: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(&dry_json.stdout))
            .expect("the dry-run JSON parses");
    assert_eq!(
        kind_of(&forecast["manifest"], "foreign-a.txt"),
        "carried-over"
    );
    assert_eq!(kind_of(&forecast["manifest"], "doomed.txt"), "carried-over");
    assert_eq!(kind_of(&forecast["manifest"], "feature.rs"), "added");

    // Sites 2 + 3 — the `--carry-staged` run: the pre-commit print names each carried
    // path BEFORE the commit lands, and the landed manifest text labels the same set.
    let land = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--carry-staged"],
        None,
    );
    assert!(
        land.status.success(),
        "`--carry-staged` lands; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&land.stdout),
        String::from_utf8_lossy(&land.stderr),
    );
    let land_text = String::from_utf8_lossy(&land.stdout).to_string();
    let (pre_commit, landed) = land_text
        .split_once("finalized ")
        .expect("the landed summary follows the pre-commit surface");
    assert!(
        pre_commit.contains("carrying over"),
        "the pre-commit print announces the declared carry-over; got:\n{pre_commit}"
    );
    for site in [pre_commit, landed] {
        for line in ["  carried-over foreign-a.txt", "  carried-over doomed.txt"] {
            assert!(
                site.contains(line),
                "both the pre-commit print and the landed text carry {line:?}; got:\n{site}"
            );
        }
    }
    assert!(
        landed.contains("  added feature.rs"),
        "the task's own edit keeps its kind in the landed text; got:\n{landed}"
    );
}

/// The **landed JSON** labels the carried entry `carried-over` — and the forecast/landed
/// **identical-set symmetry holds with the labels on**: the dry-run `manifest` and the
/// landed `committed.manifest` are byte-identical (one pre-commit-computed carried set,
/// threaded to both sites). Under `--format json` the pre-commit print goes to stderr
/// (the envelope owns stdout) and names the carried path there.
#[test]
fn the_landed_json_labels_carried_over_and_forecast_landed_stay_identical() {
    let repo = TempDir::new("label-json");
    let home = TempDir::new("home");
    init_repo(repo.path());
    ok(repo.path(), home.path(), &["setup"], "jigc setup");

    fs::write(repo.path().join("foreign-a.txt"), "deliberately carried\n").expect("write a");
    git(repo.path(), &["add", "foreign-a.txt"]);

    let task = "gate-the-carryover";
    mint_and_work(
        repo.path(),
        home.path(),
        "gate the carryover",
        task,
        "feature.rs",
    );

    // The forecast (commits nothing) ...
    let dry = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--dry-run", "--format", "json"],
        None,
    );
    assert!(dry.status.success(), "the JSON forecast must render");
    let forecast: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&dry.stdout))
        .expect("the dry-run JSON parses");

    // ... then the declared landed run.
    let land = jigc(
        repo.path(),
        home.path(),
        &[
            "task",
            "finalize",
            task,
            "--carry-staged",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        land.status.success(),
        "`--carry-staged --format json` lands; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&land.stdout),
        String::from_utf8_lossy(&land.stderr),
    );
    let landed: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&land.stdout))
        .expect("the landed JSON parses");
    let committed = &landed["committed"];

    // Site 4 — the landed JSON carries the additive kind for the carried path only.
    assert_eq!(
        kind_of(&committed["manifest"], "foreign-a.txt"),
        "carried-over"
    );
    assert_eq!(kind_of(&committed["manifest"], "feature.rs"), "added");

    // The identical-set invariant holds with the labels on (G3 symmetry).
    assert_eq!(
        forecast["manifest"], committed["manifest"],
        "dry-run and landed agree on the labeled included set",
    );
    assert_eq!(
        forecast["left_out"], committed["left_out"],
        "dry-run and landed agree on the left-out set",
    );

    // Site 2 under JSON: the envelope owns stdout, so the pre-commit print names the
    // carried path on stderr.
    let stderr = String::from_utf8_lossy(&land.stderr);
    assert!(
        stderr.contains("carried-over foreign-a.txt"),
        "the pre-commit print reaches stderr under `--format json`; got:\n{stderr}"
    );
}

// ───────────────────────── the migration composition ─────────────────────────

/// The foreign Keep-a-Changelog file the migration arms consume.
const FOREIGN_CHANGELOG: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";

/// The declarative whole-doc payload that re-authors the canonical changelog.
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

/// Mint the changelog migration task over a committed `HISTORY.md` and author the
/// canonical doc; returns the migration task id.
fn mint_migration_and_author(repo: &Path, home: &Path) -> String {
    ok(
        repo,
        home,
        &["migrate", "HISTORY.md", "--as", "changelog"],
        "jigc migrate HISTORY.md --as changelog",
    );
    let task = "migrate-changelog-history-3268e06b69e1";
    let out = jigc(
        repo,
        home,
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
    );
    assert!(
        out.status.success(),
        "`jigc doc author changelog` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    task.to_string()
}

/// A migration finalize over a foreign pre-mint staged change needs **both**
/// declarations, independently: `--approve` alone still refuses on the carryover
/// (exit 3), `--carry-staged` alone still holds at the review gate (exit 4), and
/// the two together land the commit.
#[test]
fn a_migration_finalize_needs_approve_and_carry_staged_together() {
    let repo = TempDir::new("migration");
    let home = TempDir::new("home");
    init_repo(repo.path());
    // The foreign original, committed; plus a tracked file whose foreign
    // modification is pre-staged before the migration mints.
    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write HISTORY.md");
    git(repo.path(), &["add", "HISTORY.md"]);
    git(repo.path(), &["commit", "-q", "-m", "track HISTORY.md"]);
    fs::write(repo.path().join("README.md"), "hello\nforeign edit\n").expect("modify README");
    git(repo.path(), &["add", "README.md"]);

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    let task = mint_migration_and_author(repo.path(), home.path());

    // `--approve` alone: the fidelity declaration does NOT declare the carryover.
    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", &task, "--approve", "--format", "json"],
            None,
        ),
        "jigc task finalize --approve (carryover undeclared)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(carried.len(), 1, "one carried path; got:\n{findings:#?}");
    assert_carried(&carried[0], "README.md");

    // `--carry-staged` alone: the carryover declaration does NOT approve the
    // fidelity diff — the review gate still holds (exit 4, nothing committed).
    let out = jigc(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--carry-staged"],
        None,
    );
    assert_eq!(
        out.status.code(),
        Some(4),
        "`--carry-staged` alone holds at the review gate; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    // Both declarations together land the one whole-index commit.
    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--approve", "--carry-staged"],
        "jigc task finalize --approve --carry-staged",
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains("A\tCHANGELOG.md")
            && name_status.contains("D\tHISTORY.md")
            && name_status.contains("M\tREADME.md"),
        "the landed commit carries the migration AND the declared carryover; got:\n{name_status}"
    );
}

/// The migration's **retire pathspec is exempt**: a user who pre-staged the source's
/// own deletion (`git rm --cached HISTORY.md` before `jigc migrate`) finalizes with
/// `--approve` alone — that deletion is the task's own work, never a carryover.
#[test]
fn the_migration_retire_pathspec_is_exempt() {
    let repo = TempDir::new("exempt");
    let home = TempDir::new("home");
    init_repo(repo.path());
    fs::write(repo.path().join("HISTORY.md"), FOREIGN_CHANGELOG).expect("write HISTORY.md");
    git(repo.path(), &["add", "HISTORY.md"]);
    git(repo.path(), &["commit", "-q", "-m", "track HISTORY.md"]);

    // The source's own deletion, pre-staged BEFORE the migration mints (`--cached`
    // keeps the worktree bytes for `jigc migrate` to read).
    git(repo.path(), &["rm", "-q", "--cached", "HISTORY.md"]);

    ok(repo.path(), home.path(), &["setup"], "jigc setup");
    let task = mint_migration_and_author(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--approve"],
        "jigc task finalize --approve (retire pathspec exempt)",
    );
    assert!(
        repo.path().join("CHANGELOG.md").exists() && !repo.path().join("HISTORY.md").exists(),
        "the migration lands: canonical doc promoted, foreign source retired"
    );
}

// ───────────────────────── the milestone arm (T4) ─────────────────────────

/// Write the `[dev ▸ methodology]` compose marker — the milestone-record flip is what
/// gives a docs-only milestone a non-empty finalize commit (the
/// `milestone_record_finalize` precedent), so the finalize reaches the carryover gate
/// instead of the empty-commit guard.
fn write_compose_marker(repo: &Path) {
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// create → add-task ×2 — the milestone whose create-time snapshot the finalize gate
/// consumes. Each op lands its own path-scoped record commit, so a pre-create staged
/// foreign entry stays staged (and same-blob) all the way to finalize.
fn setup_milestone(repo: &Path, home: &Path) {
    ok(
        repo,
        home,
        &["milestone", "create", "Cache rework"],
        "jigc milestone create",
    );
    ok(
        repo,
        home,
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Warm the read cache",
        ],
        "milestone add-task #1",
    );
    ok(
        repo,
        home,
        &[
            "milestone",
            "add-task",
            "cache-rework",
            "Evict cold entries",
        ],
        "milestone add-task #2",
    );
}

/// A foreign file staged **before `milestone create`** blocks `milestone finalize`
/// with one blocking routed `finalize.carried-staged` **per carried path** — and the
/// wording is law-1 honest: the entry **stays staged across the boundary** (the
/// aggregate is built from the sub-task worktrees and structurally cannot carry it),
/// never "would ride the commit". Nothing is committed by the block.
#[test]
fn a_pre_create_staged_add_blocks_milestone_finalize_per_path() {
    let repo = TempDir::new("ms-add");
    let home = TempDir::new("ms-home");
    init_repo(repo.path());
    write_compose_marker(repo.path());

    // The foreign pre-staged adds — staged BEFORE the milestone exists.
    fs::write(
        repo.path().join("foreign-a.txt"),
        "not this milestone's work\n",
    )
    .expect("write a");
    fs::write(repo.path().join("foreign-b.txt"), "also not\n").expect("write b");
    git(repo.path(), &["add", "foreign-a.txt", "foreign-b.txt"]);

    setup_milestone(repo.path(), home.path());

    let before = git(repo.path(), &["rev-list", "--count", "HEAD"]);
    let findings = blocked_findings(
        &jigc(
            repo.path(),
            home.path(),
            &["milestone", "finalize", "cache-rework", "--format", "json"],
            None,
        ),
        "jigc milestone finalize (pre-create staged adds)",
    );
    let carried = carried_staged(&findings);
    assert_eq!(
        carried.len(),
        2,
        "exactly ONE finding per carried path (two foreign paths staged); got:\n{findings:#?}"
    );
    assert_carried(&carried[0], "foreign-a.txt");
    assert_carried(&carried[1], "foreign-b.txt");

    // The honest-wording bound: the milestone aggregate cannot carry a live-index
    // entry, so the finding must say the entry STAYS STAGED — and must never claim
    // it would ride the milestone commit.
    for finding in &carried {
        let message = finding["message"].as_str().expect("message is a string");
        let route = finding["route"].as_str().expect("route is a string");
        assert!(
            message.contains("milestone"),
            "the message names the milestone boundary; got: {message}"
        );
        assert!(
            message.contains("stays staged"),
            "the message says the entry stays staged across the boundary; got: {message}"
        );
        assert!(
            !message.contains("ride") && !route.contains("ride"),
            "the milestone wording must never claim the entry would ride the aggregate \
             commit (it structurally cannot); message: {message}\nroute: {route}"
        );
    }

    // Nothing committed: the refusal precedes any side effect.
    assert_eq!(
        git(repo.path(), &["rev-list", "--count", "HEAD"]),
        before,
        "the blocked milestone finalize commits nothing"
    );
}

/// `--carry-staged` **proceeds** — and proves the honest wording: the landed
/// aggregate commit does NOT carry the foreign entry (it is built from the sub-task
/// worktrees over targeted pathspecs), and the entry is **still staged after the
/// land**.
#[test]
fn milestone_carry_staged_proceeds_and_the_foreign_entry_stays_staged() {
    let repo = TempDir::new("ms-declared");
    let home = TempDir::new("ms-home2");
    init_repo(repo.path());
    write_compose_marker(repo.path());

    fs::write(
        repo.path().join("foreign-a.txt"),
        "deliberately left staged\n",
    )
    .expect("write a");
    git(repo.path(), &["add", "foreign-a.txt"]);

    setup_milestone(repo.path(), home.path());

    ok(
        repo.path(),
        home.path(),
        &["milestone", "finalize", "cache-rework", "--carry-staged"],
        "jigc milestone finalize --carry-staged",
    );

    // The landed aggregate commit carries the milestone's own record flip — and NOT
    // the foreign entry (structurally cannot; the declared carry is a stay-staged).
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        committed
            .lines()
            .any(|l| l == "docs/milestone-records/cache-rework.md"),
        "the record flip rides the aggregate commit; files:\n{committed}"
    );
    assert!(
        !committed.lines().any(|l| l == "foreign-a.txt"),
        "the foreign entry does NOT ride the milestone aggregate commit; files:\n{committed}"
    );

    // ... and it is STILL staged after the land (law 1: it stays staged).
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        staged.lines().any(|l| l == "foreign-a.txt"),
        "the foreign entry stays staged across the boundary; staged:\n{staged}"
    );
}

/// **Missing snapshot ⇒ fail-open** (the declared bound): a milestone created
/// pre-M43 has no create-time snapshot in its area, so its finalize proceeds as
/// today — no `--carry-staged` needed — and the foreign entry still neither rides
/// the aggregate nor leaves the index.
#[test]
fn milestone_missing_snapshot_fails_open() {
    let repo = TempDir::new("ms-failopen");
    let home = TempDir::new("ms-home3");
    init_repo(repo.path());
    write_compose_marker(repo.path());

    fs::write(repo.path().join("foreign-a.txt"), "pre-create staged\n").expect("write a");
    git(repo.path(), &["add", "foreign-a.txt"]);

    setup_milestone(repo.path(), home.path());

    // Simulate a pre-M43 milestone: no create-time snapshot in the area.
    fs::remove_file(
        repo.path()
            .join(".jigc")
            .join("milestones")
            .join("cache-rework")
            .join("staged-snapshot.json"),
    )
    .expect("remove the create-time snapshot");

    ok(
        repo.path(),
        home.path(),
        &["milestone", "finalize", "cache-rework"],
        "jigc milestone finalize (no snapshot — fail-open)",
    );
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        !committed.lines().any(|l| l == "foreign-a.txt"),
        "the foreign entry still does not ride the aggregate; files:\n{committed}"
    );
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        staged.lines().any(|l| l == "foreign-a.txt"),
        "the foreign entry is still staged after the fail-open land; staged:\n{staged}"
    );
}

// ─────────────── the cross-order repro block (the confidence-audit back-sweep) ───────────────

/// **Repro block (pinning.md §3) — the M43 e2e staging-order witness, made standing**
/// (the confidence-audit back-sweep, triage item c2). The M43 completion audit drove
/// this one-off: `completions/artifacts/M43/VERDICT.md` → the e2e paragraph: "the
/// carryover gate refuses pre-mint staged adds *and deletions* at every minting door,
/// `--carry-staged` lands them labeled, **byte-identical across divergent staging
/// orders**". Standing coverage asserted the findings/labels in a **single** staging
/// order only (`a_pre_mint_staged_add_blocks_with_one_routed_finding_per_path`,
/// `the_landed_json_labels_carried_over_and_forecast_landed_stay_identical`); nothing
/// re-drove the gate under two orders, so a re-plumbing that let the recording order
/// reach the emitted findings or the manifest would ship driver-visible
/// nondeterminism with every existing test green.
///
/// ```yaml
/// claim: "carryover findings + manifest labels are byte-identical across staging orders"
/// verdict: CONFIRMED (M43 e2e audit — one-off; standing as of this test)
/// setup:
///   - two fresh repos; the SAME two foreign files written + `git add`-ed pre-mint,
///     in divergent staging orders (a-then-b vs b-then-a); the same task minted + worked
/// repro:
///   - ["jigc", "task", "finalize", "<task>", "--format", "json"]            # blocks (3)
///   - ["jigc", "task", "finalize", "<task>", "--carry-staged", "--format", "json"]
/// expect:
///   blocked: the findings envelope byte-identical across the two repos
///   landed: the envelope byte-identical modulo `committed.hash` (histories differ);
///           both foreign paths labeled `carried-over`, the task's own edit `added`
/// pinned-by: carryover_gate::carryover_findings_and_manifest_are_byte_identical_across_staging_orders
/// ```
///
/// Verified catchable: locally re-keying the carried fold (`engine::finalize`'s
/// `carried` `BTreeMap`) on worktree mtime — a staging-order proxy, since the fixture
/// diverges the write+add order — reorders the emitted findings and reddens the
/// blocked byte-compare (mutate → catch → restore; never committed).
#[test]
fn carryover_findings_and_manifest_are_byte_identical_across_staging_orders() {
    /// Build the fixture with the foreign files written + staged in `order`, then
    /// return `(blocked stdout bytes, landed envelope with `committed.hash` masked)`.
    fn stage_and_finalize(order: [(&str, &str); 2]) -> (String, serde_json::Value) {
        let repo = TempDir::new("stage-order");
        let home = TempDir::new("home");
        init_repo(repo.path());
        ok(repo.path(), home.path(), &["setup"], "jigc setup");

        // The divergent axis: the SAME two foreign files, written and staged
        // one-at-a-time in the arm's order (staging = the write + `git add` pair).
        for (path, body) in order {
            fs::write(repo.path().join(path), body).expect("write foreign file");
            git(repo.path(), &["add", path]);
        }

        let task = "gate-the-carryover";
        mint_and_work(
            repo.path(),
            home.path(),
            "gate the carryover",
            task,
            "feature.rs",
        );

        let blocked = jigc(
            repo.path(),
            home.path(),
            &["task", "finalize", task, "--format", "json"],
            None,
        );
        let findings = blocked_findings(&blocked, "jigc task finalize (staging-order arm)");
        // Non-vacuity: both foreign paths carried, each with its own keyed finding.
        assert_eq!(
            carried_staged(&findings).len(),
            2,
            "both staged foreign paths are carried; got:\n{findings:#?}",
        );
        let blocked_stdout = String::from_utf8(blocked.stdout).expect("utf-8 envelope");

        let landed = jigc(
            repo.path(),
            home.path(),
            &[
                "task",
                "finalize",
                task,
                "--carry-staged",
                "--format",
                "json",
            ],
            None,
        );
        assert!(
            landed.status.success(),
            "`--carry-staged --format json` lands; stdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&landed.stdout),
            String::from_utf8_lossy(&landed.stderr),
        );
        let mut envelope: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&landed.stdout))
                .expect("the landed JSON parses");
        // Non-vacuity: the labels are the M43 witness's — both foreign paths
        // `carried-over`, the task's own edit `added`.
        assert_eq!(
            kind_of(&envelope["committed"]["manifest"], "foreign-a.txt"),
            "carried-over"
        );
        assert_eq!(
            kind_of(&envelope["committed"]["manifest"], "foreign-b.txt"),
            "carried-over"
        );
        assert_eq!(
            kind_of(&envelope["committed"]["manifest"], "feature.rs"),
            "added"
        );
        // Mask the one legitimately repo-specific byte range: the landed commit's
        // hash (the two repos' index histories differ, so the hashes differ by
        // design — everything else must match to the byte).
        envelope["committed"]["hash"] = serde_json::Value::Null;
        (blocked_stdout, envelope)
    }

    let order_a = [
        ("foreign-a.txt", "not this task's work\n"),
        ("foreign-b.txt", "also not\n"),
    ];
    let order_b = [
        ("foreign-b.txt", "also not\n"),
        ("foreign-a.txt", "not this task's work\n"),
    ];
    assert_ne!(order_a, order_b, "the staging orders must diverge");

    let (blocked_a, landed_a) = stage_and_finalize(order_a);
    let (blocked_b, landed_b) = stage_and_finalize(order_b);

    assert_eq!(
        blocked_a, blocked_b,
        "the blocked findings envelope must be byte-identical across divergent \
         staging orders",
    );
    assert_eq!(
        serde_json::to_string_pretty(&landed_a).expect("serialize"),
        serde_json::to_string_pretty(&landed_b).expect("serialize"),
        "the landed envelope (commit hash masked) must be byte-identical across \
         divergent staging orders",
    );
}
