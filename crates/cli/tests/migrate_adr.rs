//! M25 Increment 1, T3 — end-to-end ADR migration acceptance (the deliverable's
//! Proves). Drives the **built** `jigc` binary against throwaway `git init` temp repos
//! over the **shipped** dev pack (`JIGC_PACK_DIR` = the embedded `pack/` tree),
//! exercising — not assuming — the doctype-general migrate → author → review-gate →
//! retire → adopt spine on the new NON-SINGLETON `docs/decisions/` doctype
//! ([auto-migration.md](../../../design/auto-migration.md) → Migration model +
//! Per-doctype workflows + Acceptance; [roadmap.md](../../../implementation/roadmap.md)
//! → M25 Increment 1 Proves):
//!
//!   - **Headline** — a real foreign ADR (committed, off-canonical at `docs/adr/`)
//!     migrates: `jigc migrate <path> --as adr` mints the per-file migration task, the
//!     agent authors the canonical record through `doc author adr --from-file -` (the status
//!     field + the context/decision/consequences slots), and `task finalize --approve`
//!     lands exactly ONE commit that (a) writes `docs/decisions/<title-slug>.md` byte-stable
//!     (`render(instance_from_source(x)) == x`), (b) carries `A docs/decisions/<slug>.md` +
//!     `D <foreign>` (the foreign original retired/gone), and (c) a follow-up `jigc
//!     ingest` reports the managed adr **adopted** (not unmanaged / needs-reconcile). The
//!     review gate blocks the bare finalize first (nothing committed). The headline runs
//!     into an **empty `docs/decisions/`** — the cold-spike arm.
//!   - **RED** — an un-mapped raw foreign status (`rejected`, outside the 3-enum) **blocks
//!     at the write-time enum check** (`write.malformed-value`) and stages nothing.
//!   - **2nd migrate + mapped status** — a second `jigc migrate <other>.md --as adr` mints
//!     a DISTINCT task (no serial collision) and migrates independently; its foreign status
//!     (`Deprecated`, outside the enum) is **mapped per guidance** to `superseded` and
//!     authors clean.
//!   - **Guidance skeleton** — the shipped `migrate-adr` guidance's emitted `doc author`
//!     payload skeleton parses as a valid adr author payload (the agent-facing artifact is
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
            "jigc-migrate-adr-{tag}-{}-{:?}",
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

/// The shipped `adr` schema, loaded for the byte-stable round-trip + the guidance
/// skeleton cross-check. The adr's `cites-code` is a pack-declared `code-anchor`, so the
/// schema only loads with that type threaded in (mirrors the shipped pack's `code-anchor
/// → doc-code` decl; `doc_write.rs` does the same for `spec`).
fn shipped_adr_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("adr.yaml")).expect("read shipped adr schema");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
    }];
    let mut schema =
        engine::schema::load_schema_with_types(&yaml, &types).expect("shipped adr schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The per-file migration task id — `migrate-adr-<slug(source-path)>`, path-folded so a
/// corpus migrates sequentially without serial-colliding on the singleton `migrate-adr`
/// (T1; `auto-migration.md` → Migration model). For `docs/adr/<stem>.md` the stem folds
/// path separators to `-`, then the mint re-slugifies (applying the word cap, so a long
/// stem yields a capped id).
fn migration_task(stem: &str) -> String {
    engine::slug::slugify(&format!("migrate-adr-docs-adr-{stem}"))
}

/// Write a foreign ADR at `docs/adr/<stem>.md` and **commit it**, so the retire surfaces
/// a tracked deletion in the finalize commit (the realistic flow — a pre-existing
/// committed ADR).
fn commit_foreign_adr(repo: &Path, stem: &str, body: &str) -> String {
    let rel = format!("docs/adr/{stem}.md");
    let path = repo.join(&rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create docs/adr/");
    fs::write(&path, body).expect("write foreign adr");
    git(repo, &["add", &rel]);
    git(repo, &["commit", "-q", "-m", "track foreign adr"]);
    rel
}

/// `jigc migrate <rel> --as adr` — returns the composed migration workflow's stdout
/// (which surfaces the foreign content through the source seam and the author guidance).
fn migrate(repo: &Path, home: &Path, pack: &Path, rel: &str) -> String {
    ok_stdout(
        run_jigc(repo, home, pack, &["migrate", rel, "--as", "adr"], None),
        "jigc migrate <adr> --as adr",
    )
}

/// `jigc doc author adr --from-file -` over `task`, piping the declarative `payload`.
fn author_adr(
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
        &["doc", "author", "adr", "--from-file", "-", "--task", task],
        Some(payload.as_bytes()),
    )
}

/// Assert the canonical `docs/decisions/<slug>.md` on disk round-trips byte-stable through the
/// shipped adr schema: `render(&schema, &instance_from_source(&schema, x)) == x`.
fn assert_committed_byte_stable(repo: &Path, pack: &Path, slug: &str) -> String {
    let on_disk = fs::read_to_string(
        repo.join("docs")
            .join("decisions")
            .join(format!("{slug}.md")),
    )
    .expect("the canonical adr is on disk");
    let schema = shipped_adr_schema(pack);
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed adr re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated adr is byte-stable across parse -> render:\n{on_disk}",
    );
    on_disk
}

/// Assert a follow-up `jigc ingest` reports `docs/decisions/<slug>.md` adopted — never
/// `unmanaged` / `needs-reconcile`.
fn assert_ingest_adopted(repo: &Path, home: &Path, pack: &Path, slug: &str) {
    let ingest = ok_stdout(run_jigc(repo, home, pack, &["ingest"], None), "jigc ingest");
    let needle = format!("docs/decisions/{slug}.md");
    let row = ingest
        .lines()
        .find(|l| l.contains(&needle))
        .unwrap_or_else(|| panic!("ingest must report the managed adr:\n{ingest}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the managed adr ingests as adopted:\n{row}"
    );
    assert!(
        !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed adr is neither unmanaged nor needs-reconcile:\n{row}"
    );
}

/// The headline foreign ADR (Nygard-shaped, off-canonical, in-enum status).
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

/// The canonical rewrite payload — the section-based shape the shipped guidance directs:
/// the `status` field under the `status` header section, the prose slots `<<…>>`-wrapped.
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

#[test]
fn migrate_adr_finalize_writes_retires_and_adopts() {
    let repo = TempDir::new("headline");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed foreign ADR at the off-canonical `docs/adr/` path; `docs/decisions/` is
    // empty (the cold spike — migrate into a never-before-populated doctype location).
    let rel = commit_foreign_adr(repo.path(), "0001-use-postgresql", FOREIGN_POSTGRES);
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "the cold spike starts with no `docs/decisions/` directory"
    );

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let composed = migrate(repo.path(), home.path(), &pack, &rel);
    assert!(
        composed.contains("We will use PostgreSQL"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );

    let task = migration_task("0001-use-postgresql");
    let authored = ok_stdout(
        author_adr(repo.path(), home.path(), &pack, &task, PAYLOAD_POSTGRES),
        "jigc doc author adr",
    );
    assert_eq!(
        authored, "adr:use-postgresql",
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
        "finalize --approve on a conformant migration must land (exit 0); stdout:\n{}\nstderr:\n{}",
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

    // (a) The canonical doc is committed at its per-title-slug path + byte-stable.
    let body = assert_committed_byte_stable(repo.path(), &pack, "use-postgresql");
    assert!(
        body.contains("status: accepted") && body.contains("We will use PostgreSQL"),
        "the committed adr carries the mapped status + authored prose:\n{body}",
    );

    // (b) The foreign original is GONE, and the SAME commit carries its deletion + the
    // added managed doc. `--no-renames` keeps the similar prose from collapsing the
    // delete+add into a single `R` (the contract is a retire + a fresh managed write).
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
        name_status.contains("A\tdocs/decisions/use-postgresql.md"),
        "the SAME commit carries the added managed adr:\n{name_status}"
    );

    // (c) A follow-up ingest reports it adopted.
    assert_ingest_adopted(repo.path(), home.path(), &pack, "use-postgresql");
}

/// The RED arm: a raw foreign status outside the 3-enum, authored un-mapped, must block
/// at the write-time enum check — not slip through to finalize.
const FOREIGN_REJECTED: &str = "\
# 2. Drop the message queue

## Status

Rejected

## Context

We considered a queue.

## Options

Alternatives were weighed and rejected.

## Decision

We will not add one.

## Consequences

Latency stays synchronous.
";

#[test]
fn unmapped_foreign_status_blocks_at_the_write_time_enum_check() {
    let repo = TempDir::new("red");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    let rel = commit_foreign_adr(repo.path(), "0002-drop-the-queue", FOREIGN_REJECTED);
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    migrate(repo.path(), home.path(), &pack, &rel);
    let task = migration_task("0002-drop-the-queue");

    // The agent authors the foreign status VERBATIM (`rejected`) instead of mapping it —
    // outside the enum {proposed, accepted, superseded}.
    let payload = r#"title: "Drop the message queue"
sections:
  - id: status
    set:
      status: rejected
  - id: context
    set:
      context: "<<We considered a queue.>>"
"#;
    let out = author_adr(repo.path(), home.path(), &pack, &task, payload);
    assert!(
        !out.status.success(),
        "an un-mapped foreign status must block the author; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("write.malformed-value") && stderr.contains("rejected"),
        "the block is the write-time enum reject naming the offending value; stderr:\n{stderr}",
    );

    // The batch stages nothing: no `adr:*` doc in the working area (only the
    // auto-provisioned commit doc remains).
    let docs = repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join(&task)
        .join("docs");
    let staged_adr = fs::read_dir(&docs)
        .expect("docs dir")
        .filter_map(std::result::Result::ok)
        .any(|e| e.file_name().to_string_lossy().starts_with("adr:"));
    assert!(
        !staged_adr,
        "a blocked author stages no adr doc in {docs:?}",
    );
}

/// A foreign ADR whose status is outside the enum (`Deprecated`) — the second-corpus
/// member, mapped per guidance to `superseded`.
const FOREIGN_DEPRECATED: &str = "\
# 3. Use a monolith

## Status

Deprecated

## Context

The team is small.

## Options

Alternatives were weighed and rejected.

## Decision

We will ship a single deployable.

## Consequences

We will revisit at scale.
";

const PAYLOAD_MONOLITH: &str = r#"title: "Use a monolith"
sections:
  - id: status
    set:
      status: superseded
  - id: context
    set:
      context: "<<The team is small; the original status was 'Deprecated'.>>"
  - id: decision
    set:
      decision: "<<We will ship a single deployable.>>"
  - id: consequences
    set:
      consequences: "<<We will revisit at scale.>>"
"#;

/// Migrate one foreign ADR end-to-end through `--approve` over an already-set-up repo,
/// returning the committed canonical slug. Authors the supplied payload.
fn migrate_one(
    repo: &Path,
    home: &Path,
    pack: &Path,
    stem: &str,
    foreign: &str,
    payload: &str,
    slug: &str,
) {
    let rel = commit_foreign_adr(repo, stem, foreign);
    migrate(repo, home, pack, &rel);
    let task = migration_task(stem);
    ok_stdout(
        author_adr(repo, home, pack, &task, payload),
        "jigc doc author adr",
    );
    let approve = run_jigc(
        repo,
        home,
        pack,
        &["task", "finalize", &task, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "finalize --approve ({stem}) must land; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    assert!(
        !repo.join(&rel).exists(),
        "the foreign original {rel} is retired"
    );
    assert_committed_byte_stable(repo, pack, slug);
}

#[test]
fn a_second_migrate_mints_a_distinct_task_and_migrates_independently() {
    let repo = TempDir::new("corpus");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // First migration: in-enum status, authors clean.
    migrate_one(
        repo.path(),
        home.path(),
        &pack,
        "0001-use-postgresql",
        FOREIGN_POSTGRES,
        PAYLOAD_POSTGRES,
        "use-postgresql",
    );

    // The second `jigc migrate <other>.md --as adr` mints a DISTINCT task (per-file id —
    // no serial collision on a singleton `migrate-adr`) and migrates independently; its
    // foreign `Deprecated` status is mapped per guidance to `superseded` and authors clean.
    assert_ne!(
        migration_task("0001-use-postgresql"),
        migration_task("0003-use-a-monolith"),
        "two foreign files mint distinct per-file migration tasks",
    );
    migrate_one(
        repo.path(),
        home.path(),
        &pack,
        "0003-use-a-monolith",
        FOREIGN_DEPRECATED,
        PAYLOAD_MONOLITH,
        "use-a-monolith",
    );

    // Both managed records are committed at their per-title slugs, adopted.
    for slug in ["use-postgresql", "use-a-monolith"] {
        assert!(
            git(
                repo.path(),
                &["cat-file", "-t", &format!("HEAD:docs/decisions/{slug}.md")]
            )
            .contains("blob"),
            "docs/decisions/{slug}.md is committed",
        );
        assert_ingest_adopted(repo.path(), home.path(), &pack, slug);
    }
    // The mapped status landed.
    assert!(
        git(
            repo.path(),
            &["show", "HEAD:docs/decisions/use-a-monolith.md"]
        )
        .contains("status: superseded"),
        "the mapped (Deprecated -> superseded) status is committed",
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

#[test]
fn shipped_guidance_payload_skeleton_parses_as_a_valid_adr_payload() {
    let repo = TempDir::new("guidance");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_adr(repo.path(), "0001-x", FOREIGN_POSTGRES);
    let composed = migrate(repo.path(), home.path(), &pack, &rel);

    // The skeleton the agent actually fills must parse against the SAME payload parser
    // `doc author` runs — a doc-level `set:` (rejected by the parser's deny_unknown_fields)
    // would silently break every per-guidance migration with a green spine test masking it.
    let skeleton = extract_author_skeleton(&composed);
    let schema = shipped_adr_schema(&pack);
    let parsed = cli::author::parse_author_payload(Some(&schema), &skeleton);
    assert!(
        parsed.is_ok(),
        "the shipped migrate-adr guidance payload skeleton must parse as a valid adr \
         author payload; skeleton:\n{skeleton}\nerror: {:?}",
        parsed.err(),
    );
}

/// The shipped `migrate-adr` guidance must instruct the agent to TRANSCRIBE a dated
/// source's historical date into the payload — NOT to drop it. The engine preserves an
/// authored date verbatim (proven by `an_explicit_payload_date_survives_a_migration_verbatim`)
/// and suppresses the today-stamp only for dateless sources, but a guidance-following
/// agent on the dominant real-corpus (dated) ADR shape authors exactly what the guidance
/// says. Guidance that says "do NOT author it; no historical date is carried" loses every
/// dated ADR's date — so we assert on the emitted bytes verbatim (`auto-migration.md`:117:
/// an explicit date survives, so only dateless ADRs are dropped).
#[test]
fn shipped_guidance_instructs_transcribing_a_dated_sources_date() {
    let repo = TempDir::new("date-guidance");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_adr(repo.path(), "0001-dated", FOREIGN_POSTGRES);
    let composed = migrate(repo.path(), home.path(), &pack, &rel);

    // The defect: guidance that instructs dropping the historical date.
    assert!(
        !composed.contains("no historical date is carried"),
        "the migrate-adr guidance must NOT instruct dropping the foreign date:\n{composed}",
    );
    // The fix: guidance that instructs transcribing the foreign date for dated sources.
    assert!(
        composed.contains("TRANSCRIBE") && composed.to_lowercase().contains("date"),
        "the migrate-adr guidance must instruct transcribing a dated source's date into \
         the payload (it survives — proven):\n{composed}",
    );
}

/// A *dateless* foreign ADR — Nygard-shaped but carrying NO `Date:` line (the common
/// real-corpus case the doc-level date-suppression fix targets;
/// `auto-migration.md` → Doc-level date-suppression).
const FOREIGN_DATELESS: &str = "\
# 4. Cache with Redis

## Status

Accepted

## Context

We need a fast cache.

## Options

Alternatives were weighed and rejected.

## Decision

We will use Redis.

## Consequences

Another service to operate.
";

/// The canonical rewrite payload for the dateless ADR — carrying NO `date:` (the agent
/// has no foreign date to transcribe).
const PAYLOAD_DATELESS: &str = r#"title: "Cache with Redis"
sections:
  - id: status
    set:
      status: accepted
  - id: context
    set:
      context: "<<We need a fast cache.>>"
  - id: decision
    set:
      decision: "<<We will use Redis.>>"
  - id: consequences
    set:
      consequences: "<<Another service to operate.>>"
"#;

/// The same canonical rewrite, but the agent transcribes an EXPLICIT foreign date into
/// the `status` section's `date` field — which must survive the migration verbatim (the
/// on-create stamp is overwritable on the write path; suppression only drops the
/// *fabricated* today-stamp, never an authored value).
const PAYLOAD_DATED: &str = r#"title: "Cache with Redis"
sections:
  - id: status
    set:
      status: accepted
      date: "2014-05-09"
  - id: context
    set:
      context: "<<We need a fast cache.>>"
  - id: decision
    set:
      decision: "<<We will use Redis.>>"
  - id: consequences
    set:
      consequences: "<<Another service to operate.>>"
"#;

/// Migrate the dateless foreign ADR end-to-end with the supplied author `payload`,
/// asserting `finalize --approve` lands clean, then returning the committed canonical
/// body (byte-stable across parse -> render).
fn migrate_dateless(repo: &Path, home: &Path, pack: &Path, payload: &str) -> String {
    let rel = commit_foreign_adr(repo, "0004-cache-with-redis", FOREIGN_DATELESS);
    migrate(repo, home, pack, &rel);
    let task = migration_task("0004-cache-with-redis");
    ok_stdout(
        author_adr(repo, home, pack, &task, payload),
        "jigc doc author adr",
    );
    let approve = run_jigc(
        repo,
        home,
        pack,
        &["task", "finalize", &task, "--approve"],
        None,
    );
    assert!(
        approve.status.success(),
        "a dateless adr migration finalizes clean — a `set: on-create` date is not \
         author-required, so its absence does not block (the item-level precedent, now \
         exercised doc-level); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );
    assert_committed_byte_stable(repo, pack, "cache-with-redis")
}

/// (a) The deliverable: a *dateless* foreign ADR migrates with NO date line rendered —
/// the migration day is **not** fabricated as false decision history — and the finalize
/// lands clean (an absent doc-level `set: on-create` date is not author-required). The
/// doc-level twin of M24's item-level date-suppression fix.
#[test]
fn dateless_adr_migration_renders_no_date_and_finalizes_clean() {
    let repo = TempDir::new("dateless");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let body = migrate_dateless(repo.path(), home.path(), &pack, PAYLOAD_DATELESS);

    // No `date:` header line — the today-stamp was suppressed, not fabricated.
    assert!(
        !body.contains("date:"),
        "a dateless adr migration renders no date line (no fabricated today):\n{body}",
    );
    // The rest of the record still landed — scoped suppression, not a blanket drop.
    assert!(
        body.contains("status: accepted") && body.contains("We will use Redis"),
        "the dateless migration still lands the mapped status + authored prose:\n{body}",
    );
}

/// (b) An EXPLICIT date in the author payload survives the migration verbatim into the
/// promoted doc — suppression drops only the fabricated today-stamp, never an authored
/// value (the on-create stamp stays overwritable on the write path).
#[test]
fn an_explicit_payload_date_survives_a_migration_verbatim() {
    let repo = TempDir::new("dated");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let body = migrate_dateless(repo.path(), home.path(), &pack, PAYLOAD_DATED);
    assert!(
        body.contains("date: 2014-05-09"),
        "an explicit payload date survives the migration verbatim:\n{body}",
    );
}

/// (c) Regression: a NON-migration `jigc doc create adr` still stamps a `date:` header
/// line (today's on-create stamp) — the suppression is scoped to migration tasks and
/// must not bleed into ordinary authoring. The exact `today_iso() == today` equality is
/// unit-pinned in `doc.rs`; here the integration witness is that the flag-false create
/// path stamps a well-formed on-create date at all (the only thing suppression could
/// have wrongly dropped).
#[test]
fn a_non_migration_doc_create_adr_still_stamps_today() {
    let repo = TempDir::new("regression");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // Mint an ordinary (NON-migration) task via the single-task work-workflow, then
    // create an adr through its create-gate — the `migration = false` write path.
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "single-task", "add a thing"],
            None,
        ),
        "jigc start --workflow single-task",
    );
    let created = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "create",
                "adr",
                "--title",
                "Pick a Datastore",
                "--task",
                "add-a-thing",
            ],
            None,
        ),
        "jigc doc create adr",
    );
    assert_eq!(
        created, "adr:pick-a-datastore",
        "the non-migration create mints the per-title slug",
    );

    // The staged working doc carries an on-create `date:` line with a well-formed ISO
    // date — the non-migration path is unaffected by the migration date-suppression.
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join("add-a-thing")
            .join("docs")
            .join("adr:pick-a-datastore.md"),
    )
    .expect("the staged adr is on disk");
    let date_line = staged
        .lines()
        .find_map(|l| l.strip_prefix("date: "))
        .unwrap_or_else(|| panic!("a non-migration create stamps a date line:\n{staged}"));
    let parts: Vec<&str> = date_line.split('-').collect();
    assert!(
        parts.len() == 3
            && parts[0].len() == 4
            && parts[0].chars().all(|c| c.is_ascii_digit())
            && parts[1].parse::<u32>().is_ok_and(|m| (1..=12).contains(&m))
            && parts[2].parse::<u32>().is_ok_and(|d| (1..=31).contains(&d)),
        "the non-migration create stamps a well-formed on-create (today) ISO date; got: \
         {date_line:?}\n{staged}",
    );
}

// ---------------------------------------------------------------------------
// M25 Increment 4, T2 — finalize-promote clobber guard acceptance (the deliverable's
// Proves; `auto-migration.md` → Honest bounds → the data-loss clobber guard;
// `worked-examples.md` → flow 27), revised at M43 (fork 5, the same-path carve-out). A
// migration whose promote destination holds a DISTINCT committed managed doc **blocks at
// finalize-promote** instead of silently clobbering it — while the same-path migration
// (recorded source == destination) rewrites IN PLACE, review-before-destroy — driven
// end-to-end through the built binary on the NON-SINGLETON `adr` shape.
// ---------------------------------------------------------------------------

/// The **per-file** migration task id for an arbitrary repo-relative source `rel` — the
/// production derivation ([`crate::start::mint_migration_in_repo`]): strip `.md`, fold path
/// separators to `-`, slugify. The hard-coded [`migration_task`] above is the `docs/adr/`
/// specialization; the squatter arm migrates a source under `docs/decisions/`, so it needs the
/// general form.
fn migration_task_for(rel: &str) -> String {
    let stem = rel.strip_suffix(".md").unwrap_or(rel);
    let folded: String = stem
        .chars()
        .map(|c| if c == '/' { '-' } else { c })
        .collect();
    let source = format!("migrate-adr-{}", engine::slug::slugify(&folded));
    engine::slug::slugify(&source)
}

/// `task finalize <task> --approve` raw output (the clobber arm asserts the BLOCK, so it
/// cannot use the success-asserting [`migrate_one`] tail).
fn finalize_approve(repo: &Path, home: &Path, pack: &Path, task: &str) -> std::process::Output {
    run_jigc(
        repo,
        home,
        pack,
        &["task", "finalize", task, "--approve"],
        None,
    )
}

/// (a) **Title-slug collision across the doctype dir** — with `docs/decisions/use-postgresql.md`
/// already committed (a first migration), a SECOND foreign ADR at a distinct off-canonical
/// path whose authored title slugs to the SAME `use-postgresql` must **block at
/// finalize-promote** (`finalize.promote-clobber`): no clobber, nothing committed, the
/// committed ADR byte-intact, and the 2nd foreign original NOT retired (the all-or-nothing
/// transaction never ran). The data-loss case is conformant-over-conformant.
#[test]
fn a_title_slug_collision_blocks_at_finalize_promote() {
    let repo = TempDir::new("collision");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // First migration lands a committed managed ADR at `docs/decisions/use-postgresql.md`.
    migrate_one(
        repo.path(),
        home.path(),
        &pack,
        "0001-use-postgresql",
        FOREIGN_POSTGRES,
        PAYLOAD_POSTGRES,
        "use-postgresql",
    );
    let committed_before = fs::read(
        repo.path()
            .join("docs")
            .join("decisions")
            .join("use-postgresql.md"),
    )
    .expect("committed adr");
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    // A SECOND foreign ADR at a DISTINCT off-canonical path; the agent authors it with the
    // SAME title (`Use PostgreSQL`), which slugs to the already-occupied `use-postgresql`.
    let rel = commit_foreign_adr(repo.path(), "0009-postgres-again", FOREIGN_POSTGRES);
    let count_with_foreign: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    migrate(repo.path(), home.path(), &pack, &rel);
    let task = migration_task_for("docs/adr/0009-postgres-again.md");
    let authored = ok_stdout(
        author_adr(repo.path(), home.path(), &pack, &task, PAYLOAD_POSTGRES),
        "jigc doc author adr (colliding title)",
    );
    assert_eq!(
        authored, "adr:use-postgresql",
        "the 2nd author succeeds in the working area — the collision is a finalize-promote \
         concern, not a write-time one",
    );

    // finalize --approve must BLOCK with `finalize.promote-clobber` naming the destination.
    let out = finalize_approve(repo.path(), home.path(), &pack, &task);
    assert!(
        !out.status.success(),
        "a colliding title must block at finalize-promote (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let streams = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        streams.contains("finalize.promote-clobber")
            && streams.contains("docs/decisions/use-postgresql.md"),
        "the block is the clobber guard naming the destination it refused to overwrite:\n{streams}",
    );

    // Commits nothing (the all-or-nothing transaction never ran).
    assert_eq!(
        count_with_foreign,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the clobber block commits nothing past the foreign-tracking commit",
    );
    assert_eq!(
        count_with_foreign,
        count_before + 1,
        "only the foreign-tracking commit landed — the migration did not",
    );
    // The committed ADR is byte-intact — never clobbered.
    assert_eq!(
        fs::read(
            repo.path()
                .join("docs")
                .join("decisions")
                .join("use-postgresql.md")
        )
        .expect("committed adr"),
        committed_before,
        "the already-committed ADR is byte-intact after the refused clobber",
    );
    // The 2nd foreign original is NOT retired (no transaction ran).
    assert!(
        repo.path().join(&rel).exists(),
        "the blocked migration does not retire the 2nd foreign original",
    );
}

/// (b) **The same-path migration rewrites in place (M43 fork 5)** — `jigc migrate
/// docs/decisions/<slug>.md --as adr` where the foreign file already sits at the canonical
/// destination: the working area seeds BLANK (the copy-in is singleton-gated — no
/// Frankenstein doc), a plain finalize holds at the review gate (exit 4) rendering the
/// fidelity diff, and `--approve` — the sole destructive gate — rewrites the committed
/// file IN PLACE: one commit whose name-status shows `M <path>` (never `D`+`A`), the
/// committed file now the byte-stable rewrite, nothing retired.
#[test]
fn a_same_path_migration_rewrites_in_place() {
    let repo = TempDir::new("same-path");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // A foreign non-conformant ADR committed AT the canonical destination — the user's
    // layout already matches jigc's (the most common brownfield case).
    let rel = "docs/decisions/use-postgresql.md";
    let path = repo.path().join(rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create docs/decisions/");
    fs::write(&path, FOREIGN_POSTGRES).expect("write the same-path foreign adr");
    git(repo.path(), &["add", rel]);
    git(repo.path(), &["commit", "-q", "-m", "track foreign adr"]);
    let count_before: u32 = git(repo.path(), &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();

    migrate(repo.path(), home.path(), &pack, rel);
    let task = migration_task_for(rel);
    let authored = ok_stdout(
        author_adr(repo.path(), home.path(), &pack, &task, PAYLOAD_POSTGRES),
        "jigc doc author adr (same-path)",
    );
    assert_eq!(
        authored, "adr:use-postgresql",
        "the same-path author succeeds in the working area (adr is non-singleton — no \
         committed-store copy-in)",
    );
    // The working area seeded BLANK: the authored doc carries none of the foreign body.
    let staged = fs::read_to_string(
        repo.path()
            .join(".jigc")
            .join("tasks")
            .join(&task)
            .join("docs")
            .join("adr:use-postgresql.md"),
    )
    .expect("the staged adr in the working area");
    assert!(
        !staged.contains("Alternatives were weighed and rejected."),
        "the same-path foreign body must NOT seed the edit base (no Frankenstein doc):\n{staged}",
    );

    // Plain finalize: the review hold renders the fidelity diff and commits NOTHING.
    let bare = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["task", "finalize", &task],
        None,
    );
    assert_eq!(
        bare.status.code(),
        Some(4),
        "a plain same-path finalize holds at the review gate (exit 4); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&bare.stdout),
        String::from_utf8_lossy(&bare.stderr),
    );
    let review = String::from_utf8_lossy(&bare.stdout).to_string();
    assert!(
        review.contains("migration review required")
            && review.contains("Alternatives were weighed and rejected.")
            && review.contains("We will use PostgreSQL as the primary datastore."),
        "the review hold renders the fidelity diff (foreign source + canonical rewrite):\n{review}",
    );
    assert_eq!(
        count_before,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the review hold commits nothing",
    );

    // --approve rewrites IN PLACE: ONE commit, `M <path>` — never `D`+`A`, nothing retired.
    let out = finalize_approve(repo.path(), home.path(), &pack, &task);
    assert!(
        out.status.success(),
        "finalize --approve on the same-path migration must land; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        count_before + 1,
        git(repo.path(), &["rev-list", "--count", "HEAD"])
            .parse::<u32>()
            .unwrap(),
        "the approved same-path migration lands exactly ONE commit",
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.lines().any(|l| l == format!("M\t{rel}")),
        "the same-path migration modifies the canonical doc in place:\n{name_status}",
    );
    assert!(
        !name_status.lines().any(|l| l == format!("D\t{rel}"))
            && !name_status.lines().any(|l| l == format!("A\t{rel}")),
        "the in-place rewrite is never a delete+re-add (`D`+`A`):\n{name_status}",
    );

    // The committed file is now the byte-stable rewrite; the foreign body is gone.
    let body = assert_committed_byte_stable(repo.path(), &pack, "use-postgresql");
    assert!(
        body.contains("status: accepted") && body.contains("We will use PostgreSQL"),
        "the committed adr is the canonical rewrite:\n{body}",
    );
    assert!(
        !body.contains("Alternatives were weighed and rejected."),
        "the committed adr carries none of the foreign source body:\n{body}",
    );
    assert_ingest_adopted(repo.path(), home.path(), &pack, "use-postgresql");
}

// ---------------------------------------------------------------------------
// M43 Increment 6, T2 — `--slug` on `jigc migrate`, threaded via task state to
// `doc author`'s `create_gated` override (RC-lacon A2; `DECISIONS.md` 2026-07-16
// M43 Inc 6 decomposition → T2). The override drives the PROMOTED DOC's id
// verbatim — decoupled from the authored title — while the task id stays the
// per-file `migrate-adr-<slug(source-path)>`. A malformed value is rejected
// BEFORE minting (the `ensure_migratable` mint-after-validate discipline), never
// silently re-slugified. The omitting contexts stay byte-identical to today: a
// slug-less migrate is pinned by `migrate_adr_finalize_writes_retires_and_adopts`
// (ack `adr:use-postgresql`, the title-derived slug), and a non-migration
// `doc author` by the `doc_write.rs` suite — no override file, `None` read.
// ---------------------------------------------------------------------------

/// The headline: `jigc migrate <path> --as adr --slug pinned-decision`, then
/// `doc author adr` with a payload whose title slugs DIFFERENTLY
/// (`Use PostgreSQL` → `use-postgresql`), acks `adr:pinned-decision` — the
/// override, not the title, drives the id — and `finalize --approve` promotes to
/// `docs/decisions/pinned-decision.md` (byte-stable), retiring the foreign
/// original. The title-derived path is never created.
#[test]
fn a_migrate_slug_override_drives_the_promoted_id() {
    let repo = TempDir::new("slug-override");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    let rel = commit_foreign_adr(repo.path(), "0005-pin-the-id", FOREIGN_POSTGRES);
    let composed = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["migrate", &rel, "--as", "adr", "--slug", "pinned-decision"],
            None,
        ),
        "jigc migrate <adr> --as adr --slug pinned-decision",
    );
    assert!(
        composed.contains("We will use PostgreSQL"),
        "the slug-carrying migrate still composes the source seam:\n{composed}",
    );

    // The payload title slugs to `use-postgresql` — the ack must carry the OVERRIDE.
    let task = migration_task("0005-pin-the-id");
    let authored = ok_stdout(
        author_adr(repo.path(), home.path(), &pack, &task, PAYLOAD_POSTGRES),
        "jigc doc author adr (slug override)",
    );
    assert_eq!(
        authored, "adr:pinned-decision",
        "the recorded `--slug` override drives the authored doc id, not the title",
    );

    let approve = finalize_approve(repo.path(), home.path(), &pack, &task);
    assert!(
        approve.status.success(),
        "finalize --approve on the slug-overridden migration must land; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&approve.stdout),
        String::from_utf8_lossy(&approve.stderr),
    );

    // Promoted at the OVERRIDE slug, byte-stable; the title-derived path never created.
    let body = assert_committed_byte_stable(repo.path(), &pack, "pinned-decision");
    assert!(
        body.contains("# Use PostgreSQL") && body.contains("We will use PostgreSQL"),
        "the promoted doc keeps the authored title — only the id is overridden:\n{body}",
    );
    assert!(
        !repo
            .path()
            .join("docs")
            .join("decisions")
            .join("use-postgresql.md")
            .exists(),
        "the title-derived slug path is never created when `--slug` overrides",
    );
    // The foreign original is retired in the same commit that adds the override path.
    assert!(
        !repo.path().join(&rel).exists(),
        "the approved migration retires the foreign original"
    );
    let name_status = git(
        repo.path(),
        &["show", "--name-status", "--no-renames", "--format=", "HEAD"],
    );
    assert!(
        name_status.contains("A\tdocs/decisions/pinned-decision.md")
            && name_status.contains(&format!("D\t{rel}")),
        "one commit adds the override-slugged doc + retires the foreign original:\n{name_status}",
    );
}

/// A malformed `--slug` rejects BEFORE any task is minted (the mint-after-validate
/// discipline): non-zero exit, the reject-verbatim message (never a silent
/// re-slugify), `jigc task list` unchanged, and no task dir stranded.
#[test]
fn a_malformed_migrate_slug_rejects_before_minting() {
    let repo = TempDir::new("slug-malformed");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_adr(repo.path(), "0006-bad-slug", FOREIGN_POSTGRES);

    let list_before = ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["task", "list"], None),
        "jigc task list (before)",
    );
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["migrate", &rel, "--as", "adr", "--slug", "Pinned Decision"],
        None,
    );
    assert!(
        !out.status.success(),
        "a malformed `--slug` must reject (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("is not a valid slug"),
        "the reject names the malformed slug verbatim — never silently re-slugified:\n{stderr}",
    );

    // Nothing minted: the task list is unchanged and no per-file task dir exists.
    let list_after = ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["task", "list"], None),
        "jigc task list (after)",
    );
    assert_eq!(
        list_before, list_after,
        "a rejected `--slug` strands no task — `jigc task list` is unchanged",
    );
    assert!(
        !repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join(migration_task("0006-bad-slug"))
            .exists(),
        "no migration task dir is minted for the rejected `--slug`",
    );
}

/// (c) **Regression** — a normal first-time `jigc migrate <foreign>.md --as adr` into a
/// non-colliding, empty `docs/decisions/` is **unaffected** by the guard: it promotes, retires
/// the foreign original, commits, and a follow-up `jigc ingest` reports it **adopted**. The
/// guard's negative case, proven adjacent to the blocks so the acceptance is self-contained.
#[test]
fn a_first_time_migration_into_empty_decisions_is_unaffected() {
    let repo = TempDir::new("regression");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    assert!(
        !repo.path().join("docs").join("decisions").exists(),
        "the regression starts with no `docs/decisions/` directory (no possible collision)",
    );
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // The guard must NOT fire — the canonical destination is unoccupied. `migrate_one`
    // asserts the approve lands + the foreign original is retired + byte-stability.
    migrate_one(
        repo.path(),
        home.path(),
        &pack,
        "0001-use-postgresql",
        FOREIGN_POSTGRES,
        PAYLOAD_POSTGRES,
        "use-postgresql",
    );
    assert_ingest_adopted(repo.path(), home.path(), &pack, "use-postgresql");
}
