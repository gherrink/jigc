//! M44 Increment 8 / T1 — the **M44 rc.8-wave done-picture acceptance suite**,
//! driven end-to-end through the **real `jigc` binary**. Increments 1–7 proved each
//! feature per-feature (`migrate_adr.rs`, `write_not_present_route.rs`,
//! `workflow_preview.rs`, the `adapter.rs` bootstrap-body tests,
//! `record_decision_acceptance.rs`, `stated_at_fence.rs`, the `render.rs`
//! scan-guard unit); this suite ties them into one arm per claim over the real
//! binary (`design/worked-examples.md` → flow 45; roadmap → M44 Inc 8).
//!
//! **The claim the wave proves: the capabilities behave like the gates — every
//! capability an agent had to *reach for* is now preloaded into the context it
//! can't avoid, or surfaced at the moment of relevance.** M43 made the printed
//! surfaces truthful; M44 makes the *pull* as reliable as the push.
//!
//! The seven arms, each a `#[test]` over the real binary:
//!
//!   (1) **Two long-slug migrations in one directory mint distinct, attributable
//!       ids** — the `blake3(path)` disambiguator separates two source paths whose
//!       first-`MAX_WORDS` slug window collides; the same file re-derives the same
//!       id (the resume/serial-collision contract) (Inc 1).
//!
//!   (2) **A `write.not-present` routes to a followable containing-section read** —
//!       `jigc doc show <type>:<slug>#<section> --task <id>`, emitted verbatim,
//!       resolves and reveals the section's real item ids (Inc 2).
//!
//!   (3) **`jigc workflow <id> --preview` composes step text without minting** — exit
//!       0, no `.jigc/tasks/*` directory, `--format json` carries `"task": null`
//!       (Inc 3).
//!
//!   (4) **A generated `AGENT.md` carries the machine-output paragraph + the
//!       read rule** — every verb speaks `--format json`, the composed producers
//!       return the id at `.task`, and jigc's own behaviour is learned from the
//!       installed binary, never a checked-out source tree (Inc 4).
//!
//!   (5) **A from-knowledge adr is authored with a fresh on-create date via plain
//!       finalize** — `record-decision` mints, the three slots author, and plain
//!       `jigc task finalize` (no `--approve`, no review-hold, no foreign source)
//!       commits one adr carrying today's stamp (Inc 5).
//!
//!   (6) **A `{{schema:<singleton>}}`-bearing step lacking the copy-in statement
//!       reddens at pack-load** — a mutated-filesystem-pack arm strips the
//!       `author-migration` step's `states-constraints:` declaration and the
//!       composing `jigc start` blocks naming `create.singleton-copy-in` (Inc 6).
//!
//!   (7) **`project-alpha-2.0` is not flagged by the fidelity scan** — a migration whose
//!       foreign source mentions `project-alpha-2.0` and whose rewrite omits it renders
//!       the review-gate fidelity summary as `(none)`, never a false "dropped 2.0"
//!       (Inc 7).
//!
//! **The declared proof split** (`design/surface-contract.md` → How each fence is
//! proven): the flow arms above prove the wave's *behaviour changes* through the
//! shipped binary; arm 6 is the **release-real pack-load fence**, proven via a
//! mutated-filesystem-pack copy (the `stated_at_fence.rs` mold). The
//! **debug-posture fences** (route-construction parse, key discrimination) prove
//! via the seam suites and are not re-proven here.
//!
//! Everything is asserted on the EMITTED bytes / exit codes / committed files of the
//! real binary (`CARGO_BIN_EXE_jigc`). No external test crates beyond `serde_json`.

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
            "jigc-flow45-{tag}-{}-{:?}",
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

/// Stdout of an invocation, trailing newlines trimmed.
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
        b"Driven by the flow-45 acceptance suite.\n",
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

/// Today's date the way the binary stamps it (system local date), so the
/// fresh-on-create assertion is exact.
fn today() -> String {
    // The CLI's `set: on-create` date stamp derives from `SystemTime` days-since-epoch
    // in **UTC** (`doc::today_iso`, `secs / 86_400`). Read the same clock here — a local
    // `date +%Y-%m-%d` disagrees with the CLI's UTC stamp across the UTC/local midnight
    // boundary (a machine east of UTC flips a day early), which is a spurious failure.
    let out = Command::new("date")
        .args(["-u", "+%Y-%m-%d"])
        .output()
        .expect("run date");
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_owned()
}

/// A foreign Nygard-shaped ADR body (off-canonical, in-enum status) — the shape the
/// per-increment migrate suites exercise, reused so the arms exercise the proven path.
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

/// Commit a foreign ADR carrying `body` at `rel` so a migration's retire surfaces a
/// tracked deletion.
fn commit_foreign_adr(repo: &Path, rel: &str, body: &str) {
    let path = repo.join(rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create the foreign adr's dir");
    fs::write(&path, body).expect("write the foreign adr");
    git(repo, &["add", rel]);
    git(repo, &["commit", "-q", "-m", "track foreign adr"]);
}

/// Extract the leading backticked command from a route string (`` `<cmd>`<tail> ``).
fn backticked(route: &str) -> &str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("route carries a backticked command; got: {route}"))
}

// ───────── Arm 1 — two long-slug migrations in one dir mint distinct, attributable ids ─────────

/// **Arm 1.** The migration task id folds `blake3(repo-relative-path)` in
/// unconditionally, so two source paths whose first-`MAX_WORDS` slug window **collides**
/// (two long filenames in one directory sharing their leading words) mint **distinct,
/// attributable** ids — the `…/planning-migration-adr.md`-class collision closed at the
/// root — while the same file re-derives the **same** id (the resume/serial-collision
/// contract). Red at rc.7: the legible slug alone drove the id, so the mint's re-slugify
/// cap collapsed two long same-window paths onto one id (a serial collision, a lost
/// migration).
#[test]
fn two_long_slug_migrations_in_one_dir_mint_distinct_attributable_ids() {
    let (repo, home) = setup_repo("longslug");
    let repo = repo.path();
    let home = home.path();

    // Two foreign ADRs in ONE directory whose stems share their leading words — the
    // folded path (`docs-adr-adopt-the-new-…`) caps to the same slug window today.
    let reads = "docs/adr/adopt-the-new-caching-layer-for-reads.md";
    let writes = "docs/adr/adopt-the-new-caching-layer-for-writes.md";
    commit_foreign_adr(repo, reads, FOREIGN_POSTGRES);
    commit_foreign_adr(repo, writes, FOREIGN_POSTGRES);

    let id_reads = minted_task(&stdout_of(&jigc(
        repo,
        home,
        &["migrate", reads, "--as", "adr"],
        None,
    )));
    let id_writes = minted_task(&stdout_of(&jigc(
        repo,
        home,
        &["migrate", writes, "--as", "adr"],
        None,
    )));

    // Distinct ids — the two migrations do not serial-collide.
    assert_ne!(
        id_reads, id_writes,
        "two distinct source paths mint distinct migration task ids",
    );
    // But the same capped slug window (would collide without the hash): both ids share
    // everything up to the trailing 12-hex disambiguator.
    let (stem_reads, hash_reads) = id_reads
        .rsplit_once('-')
        .expect("the id carries a `-<hash>` disambiguator");
    let (stem_writes, hash_writes) = id_writes
        .rsplit_once('-')
        .expect("the id carries a `-<hash>` disambiguator");
    assert_eq!(
        stem_reads, stem_writes,
        "both ids share the same capped slug window — they WOULD collide without the hash",
    );
    assert!(
        stem_reads == "migrate-adr-docs-adr-adopt-the-new",
        "the shared window is the first-MAX_WORDS fold of the directory + stem; got: {stem_reads}",
    );
    assert_ne!(
        hash_reads, hash_writes,
        "the blake3(path) disambiguator separates the two ids",
    );

    // Determinism / the resume contract: re-migrating the SAME file re-derives the SAME
    // id and collides into the serial-collision route (never a double-mint).
    let again = jigc(repo, home, &["migrate", reads, "--as", "adr"], None);
    assert!(
        !again.status.success(),
        "re-migrating the same path must not double-mint; stdout:\n{}",
        stdout_of(&again),
    );
    let stderr = stderr_of(&again);
    assert!(
        stderr.contains(&format!("task `{id_reads}` is already active")),
        "the same path re-derives the same id and serial-collides; got:\n{stderr}",
    );
    assert!(
        stderr.contains(&format!("jigc start --task {id_reads}")),
        "the collision routes to the resume of that same id; got:\n{stderr}",
    );
}

// ───────── Arm 2 — a not-present write routes to a followable containing-section read ─────────

/// **Arm 2.** An agent that addresses a **not-yet-minted item id** on a repeatable
/// doctype gets a **followable** route — `jigc doc show <type>:<slug>#<section> --task
/// <id>`, emitted verbatim, resolves and reveals the section's **real** item ids — never
/// the generic shape-question `doc schema` dead end. Red at rc.7: `write.not-present`
/// shared the shape-question arm, so the route named the *shape* and never the corpus's
/// real ids.
#[test]
fn a_not_present_write_routes_to_a_followable_doc_show() {
    let (repo, home) = setup_repo("not-present");
    let repo = repo.path();
    let home = home.path();

    // `record-change` (off-router, started by name) create-gates the shipped `changelog`
    // singleton — a two-level repeatable of releases.
    let task = "cut-the-release";
    assert_ok(
        &jigc(
            repo,
            home,
            &["start", "--workflow", "record-change", "cut the release"],
            None,
        ),
        "`jigc start --workflow record-change`",
    );
    assert_eq!(
        stdout_of(&jigc(
            repo,
            home,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        )),
        "changelog:changelog",
        "the singleton mints at its fixed slug",
    );
    // A real release, so the followed route reveals a real item id.
    let release = stdout_of(&jigc(
        repo,
        home,
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.3.0",
        ],
        None,
    ));
    let release_id = release
        .strip_prefix("changelog:changelog#releases/")
        .expect("the release address is under #releases/")
        .to_owned();

    // remove-item at a NOT-YET-MINTED release id → `write.not-present`, routed at the
    // followable containing section.
    let blocked = jigc(
        repo,
        home,
        &[
            "doc",
            "remove-item",
            "changelog:changelog#releases/9-9-9",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        !blocked.status.success(),
        "removing a non-existent item must block; stderr:\n{}",
        stderr_of(&blocked),
    );
    let envelope: serde_json::Value = serde_json::from_str(stderr_of(&blocked).trim())
        .expect("the blocking findings envelope parses (stderr, --format json)");
    assert_eq!(
        envelope["findings"][0]["code"], "write.not-present",
        "the block is a not-present write; got:\n{envelope:#}",
    );
    let route = envelope["findings"][0]["route"]
        .as_str()
        .expect("the not-present finding carries a route");
    assert_eq!(
        backticked(route),
        format!("jigc doc show changelog:changelog#releases --task {task}"),
        "the route names the followable containing section; route:\n{route}",
    );

    // Run the emitted `doc show` VERBATIM — it resolves and reveals the section's real
    // item ids, closing the dead end with a followable recovery.
    let cmd = backticked(route);
    let mut parts = cmd.split_whitespace();
    assert_eq!(parts.next(), Some("jigc"), "the route leads with `jigc`");
    let args: Vec<&str> = parts.collect();
    let shown = jigc(repo, home, &args, None);
    assert_ok(&shown, "the emitted `doc show` route");
    assert!(
        stdout_of(&shown).contains(&release_id),
        "the emitted route reveals the section's real item id ({release_id}); got:\n{}",
        stdout_of(&shown),
    );
}

// ───────── Arm 3 — the workflow preview composes without minting ─────────

/// **Arm 3.** `jigc workflow <id> --preview` composes a `creates-task: true` workflow's
/// step text **without minting a task**: exit 0, a mint-first banner (law 3), the
/// `your-task-id` identity in the body (law 1, never a fictional real id), no
/// `.jigc/tasks/*` directory, and a `--format json` serve pinned to `{task: null, text}`.
/// Red at rc.7: a mutation-cautious agent could not read what a work-minting workflow
/// would ask of it without consenting to mint.
#[test]
fn the_workflow_preview_composes_without_minting() {
    let (repo, home) = setup_repo("preview");
    let repo = repo.path();
    let home = home.path();

    let preview = jigc(repo, home, &["workflow", "single-task", "--preview"], None);
    assert_ok(&preview, "`jigc workflow single-task --preview`");
    let out = stdout_of(&preview);
    assert!(
        out.contains("no task minted")
            && out.contains("jigc start --workflow single-task")
            && out.contains("your-task-id")
            && out.contains("jigc doc create adr"),
        "the preview leads with the mint-first banner, renders the `your-task-id` identity, \
         and carries the composed single-task body; got:\n{out}",
    );
    assert!(
        !out.contains("task minted:"),
        "a preview mints nothing — no `task minted:` line; got:\n{out}",
    );

    // No task store touched: a preview provisions no `.jigc/tasks/*` directory.
    let tasks_dir = repo.join(".jigc").join("tasks");
    if tasks_dir.exists() {
        let count = fs::read_dir(&tasks_dir).expect("read tasks dir").count();
        assert_eq!(
            count, 0,
            "a preview must create no `.jigc/tasks/*` directory"
        );
    }

    // The pinned `{task, text}` contract with `task: null` — nothing minted.
    let json = jigc(
        repo,
        home,
        &["--format", "json", "workflow", "single-task", "--preview"],
        None,
    );
    assert_ok(&json, "`--format json workflow single-task --preview`");
    let value: serde_json::Value =
        serde_json::from_str(&stdout_of(&json)).expect("the preview json parses");
    let obj = value.as_object().expect("json object");
    assert_eq!(
        obj.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["task", "text"],
        "the composed-output contract is exactly {{task, text}}",
    );
    assert!(
        value["task"].is_null(),
        "a preview mints no task — `task` is null; got: {}",
        value["task"],
    );
    assert!(
        value["text"]
            .as_str()
            .expect("`text` is a string")
            .contains("your-task-id"),
        "the composed body renders the `your-task-id` identity",
    );
}

// ───────── Arm 4 — a generated AGENT.md carries the machine-output paragraph + read rule ─────────

/// **Arm 4.** The one surface a coding agent reads before it plans — the managed
/// `.jigc/AGENT.md` `jigc setup` writes — carries the **machine-output paragraph** (every
/// verb speaks `--format json` on a successful/validation outcome; the composed producers
/// `start`/`workflow`/`migrate` return the id at `.task`, never scraped from the human
/// line) and the **read-rule amendment** (jigc's own behaviour is learned from the
/// installed binary, never a checked-out jigc/pack source tree). Red at rc.7: the
/// machine-output facts + the stale-source guardrail lived nowhere preloaded, so the
/// model reached for them and missed.
#[test]
fn a_generated_agent_md_carries_the_machine_output_paragraph_and_read_rule() {
    let (repo, _home) = setup_repo("agent-md");
    let agent_md = fs::read_to_string(repo.path().join(".jigc").join("AGENT.md"))
        .expect("`jigc setup` writes the managed .jigc/AGENT.md");

    // The machine-output paragraph (M44 Inc 4, change 1).
    assert!(
        agent_md.contains("Every verb speaks `--format json`")
            && agent_md.contains("`.task`")
            && agent_md.contains("`jigc start`")
            && agent_md.contains("`jigc workflow`")
            && agent_md.contains("`jigc migrate`"),
        "AGENT.md carries the machine-output paragraph — every verb speaks --format json, \
         the composed producers return the id at .task; got:\n{agent_md}",
    );
    assert!(
        agent_md.contains("do not scrape the human-readable lines"),
        "the machine-output paragraph forbids scraping the human line; got:\n{agent_md}",
    );
    // The read-rule amendment (M44 Inc 4, change 2).
    assert!(
        agent_md.contains("ask the installed binary")
            && agent_md.contains("never a checked-out jigc or pack source tree"),
        "AGENT.md carries the read-rule amendment — jigc's own behaviour is learned from \
         the installed binary, never a checked-out source tree; got:\n{agent_md}",
    );
}

// ───────── Arm 5 — a from-knowledge adr, fresh on-create date, plain finalize ─────────

/// **Arm 5.** `record-decision` is the honest front door for "I made a decision, record
/// it, no code to write": it mints (`creates-task: true`), the composed step names **no
/// foreign source**, authoring the three slots then **plain** `jigc task finalize` (no
/// `--approve`, no review-hold) commits **one** adr at its canonical
/// `docs/decisions/<slug>.md` carrying a **fresh on-create date** (today's stamp, not a
/// transcribed foreign date). Red at rc.7: an agent could author an adr only inside a
/// code task or a foreign migration — the flagship doctype's sole-channel hole.
#[test]
fn a_from_knowledge_adr_lands_with_a_fresh_on_create_date_via_plain_finalize() {
    let (repo, home) = setup_repo("from-knowledge");
    let repo = repo.path();
    let home = home.path();

    let task = "adopt-blake3-for-content-hashing";
    let composed = jigc(
        repo,
        home,
        &[
            "start",
            "--workflow",
            "record-decision",
            "adopt blake3 for content hashing",
        ],
        None,
    );
    assert_ok(&composed, "`jigc start --workflow record-decision`");
    let body = stdout_of(&composed);
    // The from-knowledge door names no foreign-migration prose.
    let lower = body.to_lowercase();
    for banned in ["foreign", "transcribe", "supersedes"] {
        assert!(
            !lower.contains(banned),
            "the from-knowledge step must not carry the migration word {banned:?}; got:\n{body}",
        );
    }
    assert!(
        body.contains("jigc doc create adr"),
        "the composed step offers the adr create-gate; got:\n{body}",
    );

    // Author the adr from knowledge — no source consulted.
    assert_eq!(
        stdout_of(&jigc(
            repo,
            home,
            &["doc", "create", "adr", "--title", "Adopt blake3"],
            None,
        )),
        "adr:adopt-blake3",
    );
    set_slot(
        repo,
        home,
        "adr:adopt-blake3#context",
        task,
        b"We need a fast, collision-resistant content hash.\n",
    );
    set_slot(
        repo,
        home,
        "adr:adopt-blake3#decision",
        task,
        b"Adopt blake3 for all content hashing.\n",
    );
    set_slot(
        repo,
        home,
        "adr:adopt-blake3#consequences",
        task,
        b"A vendored dependency; hashes are not sha-compatible.\n",
    );
    fill_commit(repo, home, task);

    // PLAIN finalize — no `--approve`, no review-hold.
    let before = commit_count(repo);
    assert_ok(
        &jigc(repo, home, &["task", "finalize", task], None),
        "plain `jigc task finalize`",
    );
    assert_eq!(
        commit_count(repo),
        before + 1,
        "plain finalize lands exactly ONE commit",
    );

    // The committed adr carries a FRESH on-create date (today's stamp).
    let committed = git(repo, &["show", "HEAD:docs/decisions/adopt-blake3.md"]);
    let today = today();
    assert!(
        committed.contains(&format!("date: {today}")),
        "the committed adr carries a fresh on-create date ({today}); got:\n{committed}",
    );
    assert!(
        !committed.contains("2019-02-12"),
        "no transcribed foreign date — the from-knowledge door has no source; got:\n{committed}",
    );
}

// ───────── Arm 6 — a singleton-authoring step lacking the copy-in statement reddens pack-load ─────────

/// The embedded dev pack tree — the faithful source the on-disk copies mirror.
fn embedded_pack_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
}

/// Recursively copy `src` into `dst`.
fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read pack dir") {
        let entry = entry.expect("dir entry");
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy pack file");
        }
    }
}

/// Strip the named step's `states-constraints:` declaration from a copied pack (the
/// flow-style line becomes the empty list) — the mutation the fence must catch.
fn strip_states_constraints(pack: &Path, step: &str) {
    let path = pack.join("steps").join(format!("{step}.yaml"));
    let body = fs::read_to_string(&path).expect("read the copied step");
    let mut out = String::new();
    let mut hit = false;
    for line in body.lines() {
        if line.starts_with("states-constraints:") {
            out.push_str("states-constraints: []\n");
            hit = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    assert!(
        hit,
        "the shipped `{step}` step must carry a `states-constraints:` declaration to strip",
    );
    fs::write(&path, out).expect("write the stripped step");
}

/// **Arm 6** (the release-real pack-load fence). A dev-pack copy whose singleton-authoring
/// migrate step (`author-migration`, soliciting the `changelog` singleton via
/// `{{schema:changelog}}`) withdrew its `states-constraints: [create.singleton-copy-in]`
/// declaration is **blocked at pack-load** — the composing `jigc start` exits non-zero and
/// names the offending step + the undeclared `create.singleton-copy-in` code. The path-local
/// copy-in guidance is caught by construction (the enumerable `{{schema:<singleton>}}`
/// signal the step already renders), not by author diligence.
#[test]
fn a_singleton_authoring_step_without_the_copy_in_statement_reddens_pack_load() {
    let repo = TempDir::new("copy-in-repo");
    let home = TempDir::new("copy-in-home");
    git_init(repo.path());
    // The project layer the cascade expects (composition reads it).
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("project layer");

    let pack = TempDir::new("copy-in-pack");
    copy_tree(&embedded_pack_tree(), pack.path());
    strip_states_constraints(pack.path(), "author-migration");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "single-task", "copy-in fence probe"])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .env("JIGC_PACK_DIR", pack.path())
        .output()
        .expect("spawn jigc with the mutated pack");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "a singleton-authoring step missing the copy-in declarer must make `jigc start` \
         exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        stderr.contains("create.singleton-copy-in"),
        "stderr names the undeclared `create.singleton-copy-in` code; got:\n{stderr}",
    );
    assert!(
        stderr.contains("author-migration"),
        "stderr names the offending `author-migration` step; got:\n{stderr}",
    );
    assert!(
        stderr.contains("states-constraints"),
        "stderr names the missing `states-constraints:` declaration; got:\n{stderr}",
    );
}

// ───────── Arm 7 — project-alpha-2.0 is not flagged by the fidelity scan ─────────

/// A foreign ADR whose Context prose mentions `project-alpha-2.0` (the only version-like
/// string) — no dotted-numeric version at a word boundary anywhere else.
const FOREIGN_DASHBOARD: &str = "\
# Adopt the dashboard

Date: 2020-01-01

## Status

Accepted

## Context

We are migrating the project-alpha-2.0 project onto the platform.

## Decision

We will adopt the dashboard.

## Consequences

Operators learn the new surface.
";

/// The canonical rewrite — the Context slot OMITS `project-alpha-2.0` (the migration drops the
/// slug mention). The `2.0` glued to `dashboard-` is a slug fragment, not a version, so the
/// scan never treats it as a droppable token.
const PAYLOAD_DASHBOARD: &str = r#"title: "Adopt the dashboard"
sections:
  - id: status
    set:
      status: accepted
  - id: context
    set:
      context: "<<We are migrating the legacy project onto the platform.>>"
  - id: decision
    set:
      decision: "<<We will adopt the dashboard.>>"
  - id: consequences
    set:
      consequences: "<<Operators learn the new surface.>>"
"#;

/// **Arm 7.** The fidelity boundary-guard: a dotted-numeric run glued to a preceding
/// ASCII-alphanumeric or `-` byte is a slug/identifier fragment (`project-alpha-2.0`), not a
/// version token — so a migration whose foreign source mentions `project-alpha-2.0` and whose
/// rewrite omits it renders the review-gate fidelity summary's bare-token line as
/// **`(none)`**, never a false "dropped 2.0". Red at rc.7: the scan cried wolf on `2.0`,
/// the operator's most-checked advisory surface false-alarming on a slug.
#[test]
fn project_alpha_2_0_is_not_flagged_by_the_fidelity_scan() {
    let (repo, home) = setup_repo("fidelity");
    let repo = repo.path();
    let home = home.path();

    // A foreign ADR at the canonical destination (the same-path review renders the
    // fidelity diff on plain finalize).
    let rel = "docs/decisions/adopt-the-dashboard.md";
    commit_foreign_adr(repo, rel, FOREIGN_DASHBOARD);

    let composed = jigc(repo, home, &["migrate", rel, "--as", "adr"], None);
    assert_ok(&composed, "`jigc migrate <same-path adr> --as adr`");
    let task = minted_task(&stdout_of(&composed));

    assert_ok(
        &jigc(
            repo,
            home,
            &["doc", "author", "adr", "--from-file", "-", "--task", &task],
            Some(PAYLOAD_DASHBOARD.as_bytes()),
        ),
        "`jigc doc author adr`",
    );

    // Plain finalize holds at the review gate rendering the fidelity summary.
    let hold = jigc(repo, home, &["task", "finalize", &task], None);
    assert_eq!(
        hold.status.code(),
        Some(4),
        "a plain same-path migration holds at the review gate; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&hold.stdout),
        String::from_utf8_lossy(&hold.stderr),
    );
    let review = stdout_of(&hold);
    assert!(
        review.contains("version-like token absent from the rewrite: (none)"),
        "the bare-token fidelity line is (none) — project-alpha-2.0's `2.0` is a slug fragment, \
         not a droppable version token; got:\n{review}",
    );
    assert!(
        !review.contains("absent from the rewrite: 2.0")
            && !review.contains("token absent from the rewrite: 2.0"),
        "the scan never flags `2.0` as dropped (no crying wolf on the slug); got:\n{review}",
    );
}
