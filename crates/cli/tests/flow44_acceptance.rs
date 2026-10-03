//! M43 Increment 9 / T1 — the **M43 rc.7-wave done-picture acceptance suite**, driven
//! end-to-end through the **real `jigc` binary**. Increments 1–8 proved each feature
//! per-feature (`schema_projection.rs`, `catalog_shape_fence.rs`, `carryover_gate.rs`,
//! `migrate_adr.rs`, `doc_show_staged.rs`, `start_compose.rs`, `create_or_update.rs`,
//! `doc_list.rs`); this suite ties them into one arm per claim over the real binary
//! (`design/worked-examples.md` → flow 44; roadmap → M43 Inc 9).
//!
//! **The claim the wave proves: everything jigc prints is a contracted surface —
//! nothing lies, nothing hides, nothing ambushes.** Every arm is a printed surface
//! that lied, hid a capability, or ambushed at rc.6 and now does not.
//!
//! The nine arms, each a `#[test]` over the real binary:
//!
//!   (1) **The generated migrate template names all five adr sections** — the
//!       `{{schema:adr}}` projection renders status/context/options/decision/
//!       consequences from the resolved schema; the retired hand-enumerated
//!       "fixed four-part schema" lie (which omitted `options`) is gone.
//!
//!   (2) **`decided-task` routes from the catalog** — listed with its decision-axis
//!       one-liner (vs `dev-task`'s "recording no decision"), and it composes via
//!       `--workflow` over the `[dev ▸ methodology]` pack-set a setup repo ships.
//!
//!   (3) **A pre-mint staged file refuses at finalize** — one blocking routed
//!       `finalize.carried-staged` per carried path — and `--carry-staged` lands it
//!       labeled `carried-over` in the manifest.
//!
//!   (4) **A same-path migration renders the fidelity diff on plain finalize** and
//!       `--approve` lands ONE commit with `M <path>` — never `D`+`A`.
//!
//!   (5) **An in-task write round-trips through `jigc doc show <addr> --task <id>`**
//!       at slice depth, the whole-doc json arm carries the `staged` marker key, and
//!       the committed serve stays unmarked (byte-shape-identical to the pin).
//!
//!   (6) **`jigc start --task <migration-id>` re-shows the foreign source**, and
//!       re-invoking the identical `jigc migrate` routes to that resume.
//!
//!   (7) **The enum-members line renders generated from `Field.of`**, adjacent to the
//!       composed `Run:` line and outside its backticked span.
//!
//!   (8) **`doc create` fresh acks created (`existed: false`)**; over a committed
//!       non-singleton it copies the committed body in and acks `existed: true`.
//!
//!   (9) **An empty-store `jigc doc list` prints the empty-set line** at exit 0 —
//!       never zero bytes — while the pinned json wrapper stays prose-free.
//!
//! **The declared proof split** (`design/surface-contract.md` → How each fence is
//! proven): the flow arms above prove the wave's *behaviour changes*; the
//! **debug-posture fences** (route-construction parse, floor presence, key
//! discrimination) prove via the seam suites (`crates/engine/src/finding.rs` tests,
//! `anyhow_route_spans.rs`), and the **release-real pack-load fences** prove via the
//! mutated-filesystem-pack suites (`suppression_fence.rs`, `catalog_shape_fence.rs`,
//! `stated_at_fence.rs`) — this suite does not re-prove them.
//!
//! Everything is asserted on the EMITTED bytes / exit codes / committed files of the
//! real binary (`CARGO_BIN_EXE_jigc`). No external test crates beyond `serde_json`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-flow44-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit.
fn git_init(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, optionally piping `stdin`.
/// Never inherits a harness `JIGC_PACK_DIR` — the embedded shipped packs are the
/// base, so the arms prove exactly what ships.
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
    let mut child = command.spawn().expect("spawn jigc");
    if let Some(bytes) = stdin {
        crate::support::child_stdin::feed(&mut child, bytes);
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

/// Stdout of an invocation, trailing newlines trimmed (leading bytes kept, so
/// byte-strong compose comparisons stay honest).
fn stdout_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_string()
}

/// Stderr of an invocation as UTF-8.
fn stderr_of(out: &std::process::Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("utf-8 stderr")
}

/// A repo under a **real `jigc setup`** — the `[dev ▸ methodology]` pack-set a
/// dogfooding project runs, `docs-root` at its shipped default `docs/`.
fn setup_repo(tag: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new(tag);
    let home = TempDir::new("home");
    git_init(repo.path());
    assert_ok(
        &jigc(repo.path(), home.path(), &["setup"], None),
        "`jigc setup`",
    );
    (repo, home)
}

/// Set one prose slot through the binary (stdin `--from-file -`), asserting success.
fn set_slot(repo: &Path, home: &Path, addr: &str, task: &str, prose: &[u8]) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-", "--task", task],
            Some(prose),
        ),
        &format!("set-slot {addr}"),
    );
}

/// Set one header field through the binary, asserting success.
fn set_field(repo: &Path, home: &Path, addr: &str, task: &str, value: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value, "--task", task],
            None,
        ),
        &format!("set-field {addr}"),
    );
}

/// Fill the task's commit doc so a finalize renders a clean git message.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    set_field(repo, home, &format!("commit:{task}#type"), task, "docs");
    set_slot(
        repo,
        home,
        &format!("commit:{task}#summary"),
        task,
        b"record it\n",
    );
    set_slot(
        repo,
        home,
        &format!("commit:{task}#body"),
        task,
        b"Driven by the flow-44 acceptance suite.\n",
    );
}

/// Create + fully fill an `adr:cache-strategy` in `task` (single-task's create gate
/// admits the adr), so the staged instance parses and a finalize passes the
/// required-slot gate.
fn stage_cache_strategy_adr(repo: &Path, home: &Path, task: &str) {
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "doc",
                "create",
                "adr",
                "--title",
                "Cache strategy",
                "--task",
                task,
            ],
            None,
        ),
        "`jigc doc create adr`",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#context",
        task,
        b"Lookups must stay fast.\n",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#decision",
        task,
        b"Cache locally.\n",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#consequences",
        task,
        b"A cold node re-warms.\n",
    );
}

/// The minted task id, parsed from a task-minting compose's announcement line.
fn minted_task(composed: &str) -> String {
    composed
        .lines()
        .find_map(|l| l.strip_prefix("task minted: "))
        .expect("the compose announces the minted task id")
        .trim()
        .to_string()
}

/// The number of commits reachable from HEAD.
fn commit_count(repo: &Path) -> u32 {
    git(repo, &["rev-list", "--count", "HEAD"])
        .parse()
        .expect("commit count parses")
}

/// The headline foreign ADR (Nygard-shaped, off-canonical, in-enum status) — the
/// `migrate_adr.rs` fixture, reused so the arms exercise the proven shape.
const FOREIGN_POSTGRES: &str = "\
# 1. Use PostgreSQL

Date: 2019-02-12

## Status

Accepted

## Context

We need a relational database with strong consistency.

## Options

Alternatives were weighed and rejected.

## Decision

We will use PostgreSQL as the primary datastore.

## Consequences

Operators must run and back up a Postgres instance.
";

/// The canonical rewrite payload — the section-based shape the shipped guidance
/// directs: the `status` field under the `status` section, prose slots `<<…>>`-wrapped.
const PAYLOAD_POSTGRES: &str = r#"title: "Use PostgreSQL"
sections:
  - id: status
    set:
      status: accepted
  - id: context
    set:
      context: "<<We need a relational database with strong consistency.>>"
  - id: decision
    set:
      decision: "<<We will use PostgreSQL as the primary datastore.>>"
  - id: consequences
    set:
      consequences: "<<Operators must run and back up a Postgres instance.>>"
"#;

/// Commit a foreign ADR at `rel` so a migration's retire surfaces a tracked deletion.
fn commit_foreign_adr(repo: &Path, rel: &str) {
    let path = repo.join(rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create the foreign adr's dir");
    fs::write(&path, FOREIGN_POSTGRES).expect("write the foreign adr");
    git(repo, &["add", rel]);
    git(repo, &["commit", "-q", "-m", "track foreign adr"]);
}

// ───────── Arm 1 — the generated migrate template names all five adr sections ─────────

/// **Arm 1.** The migrate author-template is **generated from the resolved schema**
/// (`{{schema:adr}}` — the law-1 generation seam), so `jigc migrate <file> --as adr`
/// composes a template naming ALL FIVE adr sections — status/context/options/decision/
/// consequences — with the home resolved through docs-root. Red at rc.6: the template
/// hand-enumerated a "fixed four-part schema" (no `options` — an adopter's weighed
/// alternatives had no mapped home) at the schema-raw `decisions/` path; a schema bump
/// could not reach prose a human once typed.
#[test]
fn the_generated_migrate_template_names_all_five_adr_sections() {
    let (repo, home) = setup_repo("template");
    let repo = repo.path();
    let home = home.path();
    commit_foreign_adr(repo, "docs/adr/0001-use-postgresql.md");

    let out = jigc(
        repo,
        home,
        &["migrate", "docs/adr/0001-use-postgresql.md", "--as", "adr"],
        None,
    );
    assert_ok(&out, "`jigc migrate <adr> --as adr`");
    let composed = stdout_of(&out);

    // The source seam still feeds the foreign bytes.
    assert!(
        composed.contains("We will use PostgreSQL as the primary datastore."),
        "the migrate compose surfaces the foreign source; got:\n{composed}",
    );
    // The generated projection names all five sections — including `options`.
    for section in [
        "- `status` (front-matter fields):",
        "- `context`: prose slot",
        "- `options`: prose slot (optional)",
        "- `decision`: prose slot",
        "- `consequences`: prose slot",
    ] {
        assert!(
            composed.contains(section),
            "the generated template names {section:?}; got:\n{composed}",
        );
    }
    // The home renders RESOLVED through docs-root — never the schema-raw `decisions/`.
    assert!(
        composed.contains("`docs/decisions/<slug>.md`"),
        "the projected home is the resolved path; got:\n{composed}",
    );
    // The retired hand-enumerated lie is gone.
    assert!(
        !composed.contains("four-part"),
        "the retired \"fixed four-part schema\" enumeration must not survive; got:\n{composed}",
    );
}

// ───────── Arm 2 — decided-task routes from the catalog and composes ─────────

/// **Arm 2.** `decided-task` is **routable**: the router catalog lists it with its
/// decision-axis one-liner — discriminated from `dev-task` on the axis agents actually
/// decide across (records-a-decision vs records-none) — and `--workflow decided-task`
/// composes over the `[dev ▸ methodology]` pack-set a setup repo ships. Red at rc.6:
/// `selectable: false` hid the pack's lightweight decision path with no reason, so the
/// capability existed and no surface named it.
#[test]
fn decided_task_routes_from_the_catalog_and_composes() {
    let (repo, home) = setup_repo("catalog");
    let repo = repo.path();
    let home = home.path();

    // The catalog: bare `jigc start "<intent>"` presents; the agent picks.
    let router = jigc(repo, home, &["start", "sort out a small change"], None);
    assert_ok(&router, "`jigc start` (the router catalog)");
    let catalog = stdout_of(&router);
    assert!(
        catalog.contains(
            "- decided-task — implement one scoped change test-first, \
             recording its design decision on the running decisions log"
        ),
        "the catalog lists `decided-task` with its decision-axis one-liner; got:\n{catalog}",
    );
    assert!(
        catalog.contains(
            "- dev-task — implement one scoped change test-first, recording no decision, \
             touching no documented code (code a managed doc names)"
        ),
        "the sibling axis discriminates — `dev-task` records none; got:\n{catalog}",
    );

    // The route composes: `--workflow decided-task` mints and emits the decision spine.
    let composed = jigc(
        repo,
        home,
        &["start", "--workflow", "decided-task", "cache the index"],
        None,
    );
    assert_ok(&composed, "`jigc start --workflow decided-task`");
    let body = stdout_of(&composed);
    assert_eq!(minted_task(&body), "cache-the-index");
    assert!(
        body.contains("design decision worth keeping"),
        "the composed spine carries the decision-recording step; got:\n{body}",
    );
    assert!(
        !body.contains("{{"),
        "no placeholder survives the compose; got:\n{body}",
    );
}

// ───────── Arm 3 — the carryover refuse + the --carry-staged land ─────────

/// The `finalize.carried-staged` findings in a findings envelope.
fn carried_staged(findings: &[serde_json::Value]) -> Vec<serde_json::Value> {
    findings
        .iter()
        .filter(|f| f["code"] == "finalize.carried-staged")
        .cloned()
        .collect()
}

/// **Arm 3.** A foreign change staged **before the task existed** cannot silently ride
/// the task's whole-index commit — the trial's #1-ranked v1 gate. Finalize refuses with
/// **one blocking routed `finalize.carried-staged` per carried path** (both exits named:
/// unstage, or declare with `--carry-staged`), and the declared `--carry-staged` run
/// lands the carryover **labeled `carried-over`** in the manifest — the task's own
/// post-mint staging keeps its kind. Red at rc.6: the pre-staged foreign files rode the
/// commit without a word.
#[test]
fn a_pre_mint_staged_file_refuses_and_carry_staged_lands_labeled() {
    let (repo, home) = setup_repo("carryover");
    let repo = repo.path();
    let home = home.path();

    // Two foreign files, staged BEFORE the task exists.
    fs::write(repo.join("foreign-a.txt"), "not this task's work\n").expect("write a");
    fs::write(repo.join("foreign-b.txt"), "also not\n").expect("write b");
    git(repo, &["add", "foreign-a.txt", "foreign-b.txt"]);

    // Mint, do the task's own work (post-mint staging), fill the commit doc.
    let task = "gate-the-carryover";
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "single-task", "gate the carryover"],
            None,
        ),
        "`jigc start --workflow single-task`",
    );
    fs::write(repo.join("feature.rs"), "pub fn work() {}\n").expect("write task edit");
    git(repo, &["add", "feature.rs"]);
    fill_commit(repo, home, task);

    // The refuse: exit 3, one blocking routed finding PER carried path.
    let before = commit_count(repo);
    let blocked = jigc(
        repo,
        home,
        &["task", "finalize", task, "--format", "json"],
        None,
    );
    assert_eq!(
        blocked.status.code(),
        Some(3),
        "the carryover blocks with the validation exit; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let envelope: serde_json::Value =
        serde_json::from_str(&stdout_of(&blocked)).expect("the findings envelope parses");
    let findings = envelope["findings"]
        .as_array()
        .expect("the envelope carries findings")
        .clone();
    let carried = carried_staged(&findings);
    assert_eq!(
        carried.len(),
        2,
        "exactly ONE finding per carried path; got:\n{findings:#?}",
    );
    for (finding, path) in carried.iter().zip(["foreign-a.txt", "foreign-b.txt"]) {
        assert_eq!(finding["severity"], "blocking");
        assert_eq!(
            finding["key"]["target"], path,
            "the finding keys at the carried file path; got: {finding}",
        );
        let route = finding["route"].as_str().expect("the refusal routes");
        assert!(
            route.contains("--carry-staged")
                && route.contains(" restore --staged -- ")
                && route.contains("git -C /"),
            "the route names both exits, the unstage aimed at the index it is about \
             (M53 — the cwd census, C1-01); got: {route}",
        );
    }
    assert!(
        !carried.iter().any(|f| f["key"]["target"] == "feature.rs"),
        "the task's own post-mint staging never trips the gate; got:\n{findings:#?}",
    );
    assert_eq!(commit_count(repo), before, "the refusal commits nothing");

    // The declared land: the carried entries ride, LABELED, in one commit.
    let land = jigc(
        repo,
        home,
        &["task", "finalize", task, "--carry-staged"],
        None,
    );
    assert_ok(&land, "`jigc task finalize --carry-staged`");
    let land_text = stdout_of(&land);
    for line in [
        "carried-over foreign-a.txt",
        "carried-over foreign-b.txt",
        "added feature.rs",
    ] {
        assert!(
            land_text.contains(line),
            "the manifest labels the carried entries and keeps the task's own kind \
             (wanted {line:?}); got:\n{land_text}",
        );
    }
    let committed = git(repo, &["show", "--name-only", "--format=", "HEAD"]);
    for path in ["foreign-a.txt", "foreign-b.txt", "feature.rs"] {
        assert!(
            committed.lines().any(|l| l == path),
            "the declared carryover and the task's own edit land together; files:\n{committed}",
        );
    }
}

// ───────── Arm 4 — the same-path migration: review on plain finalize, `M` on --approve ─────────

/// **Arm 4.** A foreign non-conformant ADR committed **at the canonical destination**
/// (the most common brownfield case — the user's layout already matches jigc's)
/// migrates end-to-end: the plain finalize **holds at the review gate rendering the
/// fidelity diff** (foreign source vs canonical rewrite, exit 4, nothing committed),
/// and `--approve` lands **one commit with `M <path>`** — an in-place rewrite, never
/// `D`+`A`, nothing retired. Red at rc.6: the singleton-only carve-out sent every
/// non-singleton same-path migration into the clobber refusal, and the route taught
/// raw git.
#[test]
fn a_same_path_migration_reviews_then_approve_lands_m() {
    let (repo, home) = setup_repo("same-path");
    let repo = repo.path();
    let home = home.path();

    let rel = "docs/decisions/use-postgresql.md";
    commit_foreign_adr(repo, rel);
    let before = commit_count(repo);

    let composed = jigc(repo, home, &["migrate", rel, "--as", "adr"], None);
    assert_ok(&composed, "`jigc migrate <same-path adr> --as adr`");
    let task = minted_task(&stdout_of(&composed));

    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "author", "adr", "--from-file", "-", "--task", &task],
            Some(PAYLOAD_POSTGRES.as_bytes()),
        ),
        "`jigc doc author adr` (same-path)",
    );

    // Plain finalize: the review hold renders the fidelity diff, commits nothing.
    let hold = jigc(repo, home, &["task", "finalize", &task], None);
    assert_eq!(
        hold.status.code(),
        Some(4),
        "a plain same-path finalize holds at the review gate; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&hold.stdout),
        String::from_utf8_lossy(&hold.stderr),
    );
    let review = stdout_of(&hold);
    assert!(
        review.contains("migration review required")
            && review.contains("Alternatives were weighed and rejected.")
            && review.contains("We will use PostgreSQL as the primary datastore."),
        "the review hold renders the fidelity diff (foreign + canonical); got:\n{review}",
    );
    assert_eq!(
        commit_count(repo),
        before,
        "the review hold commits nothing"
    );

    // --approve rewrites IN PLACE: one commit, `M <path>`, never `D`+`A`.
    assert_ok(
        &jigc(repo, home, &["task", "finalize", &task, "--approve"], None),
        "`jigc task finalize --approve` (same-path)",
    );
    assert_eq!(
        commit_count(repo),
        before + 1,
        "the approved same-path migration lands exactly ONE commit",
    );
    let name_status = git(
        repo,
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.lines().any(|l| l == format!("M\t{rel}")),
        "the same-path migration modifies the canonical doc in place:\n{name_status}",
    );
    assert!(
        !name_status.lines().any(|l| l == format!("D\t{rel}"))
            && !name_status.lines().any(|l| l == format!("A\t{rel}")),
        "the in-place rewrite is never a delete+re-add:\n{name_status}",
    );
    let body = fs::read_to_string(repo.join(rel)).expect("read the rewritten adr");
    assert!(
        body.contains("We will use PostgreSQL")
            && !body.contains("Alternatives were weighed and rejected."),
        "the committed adr is the canonical rewrite, none of the foreign body:\n{body}",
    );
}

// ───────── Arm 5 — the staged read round-trips a slot; the marker key is staged-only ─────────

/// **Arm 5.** An in-flight agent can read back its own staged write at any slice
/// depth: `jigc doc show <addr> --task <id>` serves the task's staged working copy
/// through the identical parse/slice/render path; the whole-doc `--format json` serve
/// carries the ONE additive marker key `staged: <task-id>`; and after finalize the
/// committed serve is **unmarked** — byte-shape-identical to the pinned four keys.
/// Red at rc.6: an agent that staged a write could not read it back through any
/// sanctioned surface until finalize.
#[test]
fn an_in_task_write_round_trips_through_the_staged_read() {
    let (repo, home) = setup_repo("staged-read");
    let repo = repo.path();
    let home = home.path();

    let task = "add-rate-limiter";
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "single-task", "add rate limiter"],
            None,
        ),
        "`jigc start --workflow single-task`",
    );
    stage_cache_strategy_adr(repo, home, task);

    // The slice read serves the staged prose — the write round-trips in-task.
    let slice = jigc(
        repo,
        home,
        &["doc", "show", "adr:cache-strategy#decision", "--task", task],
        None,
    );
    assert_ok(&slice, "`jigc doc show <slice> --task <id>`");
    assert_eq!(
        stdout_of(&slice).trim(),
        "Cache locally.",
        "the staged slice read hands back the very prose the task staged",
    );

    // The whole-doc json arm: the pinned shape + the ONE marker key.
    let json = jigc(
        repo,
        home,
        &[
            "doc",
            "show",
            "adr:cache-strategy",
            "--task",
            task,
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&json, "`jigc doc show ... --task <id> --format json`");
    let value: serde_json::Value = serde_json::from_str(&stdout_of(&json)).expect("valid json");
    assert_eq!(
        value["staged"], task,
        "the staged whole-doc json carries the marker key with the task id",
    );
    assert_eq!(value["sections"]["decision"], "Cache locally.");
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("whole-doc object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "fields",
            "item-count",
            "schema-version",
            "sections",
            "slug",
            "staged",
            "title",
            "type"
        ],
        "exactly the pinned keys + item-count + schema-version + title + the one marker key",
    );

    // After finalize, the committed serve is UNMARKED — the pin holds byte-shape.
    fill_commit(repo, home, task);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "`jigc task finalize`",
    );
    let committed = jigc(
        repo,
        home,
        &["doc", "show", "adr:cache-strategy", "--format", "json"],
        None,
    );
    assert_ok(&committed, "`jigc doc show` (committed)");
    let value: serde_json::Value =
        serde_json::from_str(&stdout_of(&committed)).expect("valid json");
    let mut keys: Vec<&str> = value
        .as_object()
        .expect("whole-doc object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "fields",
            "item-count",
            "schema-version",
            "sections",
            "slug",
            "title",
            "type"
        ],
        "a committed serve differs from the staged serve by exactly the `staged` key",
    );
}

// ───────── Arm 6 — the resume re-shows the foreign source; re-invoking migrate routes to it ─────────

/// **Arm 6.** The migrate resume is **whole**: `jigc start --task <migration-id>`
/// re-feeds the persisted foreign source — the resumed view is byte-identical to the
/// minting compose minus its `task minted:` header — and re-invoking the identical
/// `jigc migrate` routes to that now-working resume instead of double-minting. Red at
/// rc.6: the resume compose hardcoded no source, so the serial-collision route landed
/// on a view whose `{{source}}` rendered empty — a route to a broken surface.
#[test]
fn the_resume_re_shows_the_foreign_source_and_migrate_routes_to_it() {
    let (repo, home) = setup_repo("resume");
    let repo = repo.path();
    let home = home.path();

    let rel = "docs/adr/0007-resume-source.md";
    commit_foreign_adr(repo, rel);
    let composed = jigc(repo, home, &["migrate", rel, "--as", "adr"], None);
    assert_ok(&composed, "`jigc migrate <adr> --as adr`");
    let composed = stdout_of(&composed);
    let task = minted_task(&composed);

    // The resume — the exact invocation the serial-collision route names.
    let resumed = jigc(repo, home, &["start", "--task", &task], None);
    assert_ok(&resumed, "`jigc start --task <migration-id>`");
    let resumed = stdout_of(&resumed);
    assert!(
        resumed.contains("We will use PostgreSQL as the primary datastore."),
        "the resumed migration re-feeds the persisted source; got:\n{resumed}",
    );
    assert_eq!(
        composed,
        format!("task minted: {task}\n\n{resumed}"),
        "the resumed view is byte-identical to the minting compose minus its header",
    );

    // Re-invoking the identical migrate: no double-mint — routed at the resume.
    let again = jigc(repo, home, &["migrate", rel, "--as", "adr"], None);
    assert!(
        !again.status.success(),
        "re-invoking the identical `jigc migrate` must not double-mint; stdout:\n{}",
        stdout_of(&again),
    );
    let stderr = stderr_of(&again);
    assert!(
        stderr.contains(&format!("task `{task}` is already active")),
        "the serial-collision block names the live migration task; got:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("jigc start --task {task}")),
        "the route names the resume that now carries the source; got:\n{stderr}",
    );
}

// ───────── Arm 7 — the enum-members line, generated, adjacent to the Run: line ─────────

/// **Arm 7.** The composed authoring surface states an enum field's members **generated
/// from `Field.of`** — one line immediately below the `Run:` line, outside its
/// backticked span (the copy-runnable command stays copy-runnable). Red at rc.6: the
/// compose said "set the required type" and nothing on any surface named the members —
/// the agent guessed, and a wrong guess bounced at the write.
#[test]
fn the_enum_members_line_renders_adjacent_to_the_run_line() {
    let (repo, home) = setup_repo("enum-line");
    let repo = repo.path();
    let home = home.path();

    let out = jigc(
        repo,
        home,
        &["start", "--workflow", "single-task", "add rate limiter"],
        None,
    );
    assert_ok(&out, "`jigc start --workflow single-task`");
    let composed = stdout_of(&out);
    let lines: Vec<&str> = composed.lines().collect();

    let run_line = "Run: `jigc doc set-field commit:add-rate-limiter#type \
                    --value <COMMIT_TYPE> --task add-rate-limiter`";
    let at = lines
        .iter()
        .position(|l| *l == run_line)
        .unwrap_or_else(|| panic!("the compose emits the set-field Run: line; got:\n{composed}"));
    assert_eq!(
        lines[at + 1],
        "The `type` value is one of: feat | fix | docs | style | refactor | perf | \
         test | build | ci | chore | revert",
        "the generated members line sits immediately below the Run: line, outside \
         the backticked span; got:\n{composed}",
    );
}

// ───────── Arm 8 — doc create: fresh acks created, over a committed doc copies in + acks existed ─────────

/// **Arm 8.** The create-ack discriminates: a fresh `doc create` acks
/// **`existed: false`** (the key always present), and a second task's create over the
/// now-committed same slug **copies the committed body in** and acks
/// **`existed: true`** — the M16 create-or-update intent restored, dissolving the
/// promote-clobber ambush. Red at rc.6: the second create seeded blank, and the
/// finalize ambushed with `finalize.promote-clobber` on a route that taught raw git.
#[test]
fn doc_create_acks_created_then_existed_with_copy_in() {
    let (repo, home) = setup_repo("create-ack");
    let repo = repo.path();
    let home = home.path();

    // Task 1: fresh create → `existed: false`; author + land the adr.
    let first = "record-the-cache-decision";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "record the cache decision",
            ],
            None,
        ),
        "`jigc start` (first)",
    );
    let fresh = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache Strategy",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&fresh, "`jigc doc create adr` (fresh)");
    let ack: serde_json::Value =
        serde_json::from_str(&stdout_of(&fresh)).expect("the fresh create ack parses");
    assert_eq!(ack["op"], "create");
    assert_eq!(
        ack["existed"],
        serde_json::json!(false),
        "a fresh create acks `existed: false` — the key is always present; got:\n{ack:#}",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#context",
        first,
        b"The cache is cold on every boot.\n",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#decision",
        first,
        b"Warm it eagerly at boot.\n",
    );
    set_slot(
        repo,
        home,
        "adr:cache-strategy#consequences",
        first,
        b"Boot takes longer.\n",
    );
    fill_commit(repo, home, first);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", first], None),
        "`jigc task finalize` (first)",
    );

    // Task 2: create over the committed slug → copy-in + `existed: true`.
    let second = "revise-the-cache-decision";
    assert_ok(
        &jigc(
            repo,
            home,
            &[
                "start",
                "--workflow",
                "single-task",
                "revise the cache decision",
            ],
            None,
        ),
        "`jigc start` (second)",
    );
    let over = jigc(
        repo,
        home,
        &[
            "doc",
            "create",
            "adr",
            "--title",
            "Cache Strategy",
            "--format",
            "json",
        ],
        None,
    );
    assert_ok(&over, "`jigc doc create adr` (over committed)");
    let ack: serde_json::Value =
        serde_json::from_str(&stdout_of(&over)).expect("the copy-in create ack parses");
    assert_eq!(ack["op"], "create");
    assert_eq!(ack["target"]["slug"], "cache-strategy");
    assert_eq!(
        ack["existed"],
        serde_json::json!(true),
        "a create over a committed same-slug doc acks `existed: true`; got:\n{ack:#}",
    );

    // The working copy carries the committed body — copied in, never seeded blank.
    let staged = fs::read_to_string(
        repo.join(".jigc/tasks")
            .join(second)
            .join("docs")
            .join("adr:cache-strategy.md"),
    )
    .expect("read the second task's staged working copy");
    assert!(
        staged.contains("The cache is cold on every boot."),
        "the working copy carries the committed body (copy-in); got:\n{staged}",
    );
}

// ───────── Arm 9 — the empty-store doc list prints the empty-set line ─────────

/// **Arm 9.** An empty listing prints an **empty-set line, never zero bytes** — a
/// fresh setup repo's `jigc doc list` says so at exit 0, while the pinned
/// `--format json` wrapper stays prose-free. Red at rc.6: zero bytes, exit 0 —
/// indistinguishable from a broken pipe, and the agent re-ran it three times.
#[test]
fn an_empty_store_doc_list_prints_the_empty_set_line() {
    let (repo, home) = setup_repo("empty-list");
    let repo = repo.path();
    let home = home.path();

    let plain = jigc(repo, home, &["doc", "list"], None);
    assert_ok(&plain, "`jigc doc list` over an empty store");
    assert_eq!(
        stdout_of(&plain),
        "jigc doc list — no committed docs",
        "the empty store prints the empty-set line, never zero bytes",
    );

    let json = jigc(repo, home, &["doc", "list", "--format", "json"], None);
    assert_ok(&json, "`jigc doc list --format json` over an empty store");
    assert_eq!(
        stdout_of(&json),
        "{\n  \"docs\": []\n}",
        "the empty listing keeps the pinned object wrapper — the prose line never rides it",
    );
}
