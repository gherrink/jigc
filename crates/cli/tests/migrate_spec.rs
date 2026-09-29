//! M25 Increment 6, T1 — end-to-end spec migration acceptance (the deliverable's
//! Proves). Drives the **built** `jigc` binary against throwaway `git init` temp repos
//! over the **shipped** dev pack (`JIGC_PACK_DIR` = the embedded `pack/` tree),
//! exercising — not assuming — the doctype-general migrate → author → review-gate →
//! retire → adopt spine on the `spec` doctype, whose shape difference from `adr` is the
//! **repeatable `criteria`** with an **optional `maps-to-test`** code-anchor
//! ([auto-migration.md](../../../design/auto-migration.md) → Per-doctype migration
//! workflows + Honest bounds (spec); [worked-examples.md](../../../design/worked-examples.md)
//! → flow 27; [roadmap.md](../../../implementation/roadmap.md) → M25 Increment 6):
//!
//!   - **Headline** — a real-but-idiosyncratic foreign spec (committed, off-canonical at
//!     `docs/specs/`) migrates: `jigc migrate <path> --as spec` mints the per-file
//!     migration task, the agent authors the canonical record through `doc author spec
//!     --from-file -` (the goal/context slots + ≥2 `criteria` items, each a `title` + a
//!     `statement` slot, with **NO `maps-to-test`**), and `task finalize --approve` lands
//!     exactly ONE commit that (a) writes `docs/specs/<slug>.md` byte-stable
//!     (`render(instance_from_source(x)) == x`), (b) carries `A docs/specs/<slug>.md` + `D
//!     <foreign>` (the foreign original retired/gone), and (c) a follow-up `jigc ingest`
//!     reports the managed spec **adopted**. A spec with no test anchors finalizes clean —
//!     `maps-to-test` is optional. The review gate blocks the bare finalize first (nothing
//!     committed). The headline runs into an **empty `docs/specs/`** — the cold-spike arm.
//!   - **Guidance skeleton** — the shipped `migrate-spec` guidance's emitted `doc author`
//!     payload skeleton parses as a valid spec author payload (the agent-facing artifact is
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
            "jigc-migrate-spec-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR` so the binary
/// composes the exact bytes it ships.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
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

/// The shipped `spec` schema, loaded for the byte-stable round-trip + the guidance
/// skeleton cross-check. The spec's `maps-to-test` is a pack-declared `code-anchor`, so
/// the schema only loads with that type threaded in (mirrors the shipped pack's
/// `code-anchor → doc-code` decl; `doc_write.rs` does the same for `spec`).
fn shipped_spec_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("spec.yaml")).expect("read shipped spec schema");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
        hint: None,
    }];
    let mut schema =
        engine::schema::load_schema_with_types(&yaml, &types).expect("shipped spec schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The per-file migration task id for a repo-relative source `rel` — the production
/// derivation ([`crate::start::mint_migration_in_repo`]): strip `.md`, fold path
/// separators to `-`, slugify, prefix `migrate-<doctype>-`.
fn migration_task(rel: &str) -> String {
    let hash = &engine::file_state::hash_bytes(rel.as_bytes())[..12];
    let stem = rel.strip_suffix(".md").unwrap_or(rel);
    let folded: String = stem
        .chars()
        .map(|c| if c == '/' { '-' } else { c })
        .collect();
    let slug = engine::slug::slugify(&folded);
    if slug.is_empty() {
        format!("migrate-spec-{hash}")
    } else {
        format!("migrate-spec-{slug}-{hash}")
    }
}

/// Write a foreign spec at the off-canonical `specs/<stem>.md` and **commit it**, so
/// the retire surfaces a tracked deletion in the finalize commit (the realistic flow —
/// a pre-existing committed spec). The canonical home is now `docs/specs/` (the
/// `docs-root` nesting), so flat `specs/` is the off-canonical foreign location.
fn commit_foreign_spec(repo: &Path, stem: &str, body: &str) -> String {
    let rel = format!("specs/{stem}.md");
    let path = repo.join(&rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create specs/");
    fs::write(&path, body).expect("write foreign spec");
    git(repo, &["add", &rel]);
    git(repo, &["commit", "-q", "-m", "track foreign spec"]);
    rel
}

/// `jigc migrate <rel> --as spec` — returns the composed migration workflow's stdout
/// (which surfaces the foreign content through the source seam and the author guidance).
fn migrate(repo: &Path, home: &Path, pack: &Path, rel: &str) -> String {
    ok_stdout(
        run_jigc(repo, home, pack, &["migrate", rel, "--as", "spec"], None),
        "jigc migrate <spec> --as spec",
    )
}

/// `jigc doc author spec --from-file -` over `task`, piping the declarative `payload`.
fn author_spec(
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
        &["doc", "author", "spec", "--from-file", "-", "--task", task],
        Some(payload.as_bytes()),
    )
}

/// Assert the canonical `docs/specs/<slug>.md` on disk round-trips byte-stable through the
/// shipped spec schema: `render(&schema, &instance_from_source(&schema, x)) == x`.
fn assert_committed_byte_stable(repo: &Path, pack: &Path, slug: &str) -> String {
    let on_disk = fs::read_to_string(repo.join("docs").join("specs").join(format!("{slug}.md")))
        .expect("the canonical spec is on disk");
    let schema = shipped_spec_schema(pack);
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed spec re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated spec is byte-stable across parse -> render:\n{on_disk}",
    );
    on_disk
}

/// Assert a follow-up `jigc ingest` reports `docs/specs/<slug>.md` adopted — never
/// `unmanaged` / `needs-reconcile`.
fn assert_ingest_adopted(repo: &Path, home: &Path, pack: &Path, slug: &str) {
    let ingest = ok_stdout(run_jigc(repo, home, pack, &["ingest"], None), "jigc ingest");
    let needle = format!("docs/specs/{slug}.md");
    let row = ingest
        .lines()
        .find(|l| l.contains(&needle))
        .unwrap_or_else(|| panic!("ingest must report the managed spec:\n{ingest}"));
    assert!(
        row.contains("adoptable") && row.contains("adopted"),
        "the managed spec ingests as adopted:\n{row}"
    );
    assert!(
        !row.contains("unmanaged") && !row.contains("needs-reconcile"),
        "the managed spec is neither unmanaged nor needs-reconcile:\n{row}"
    );
}

/// A real-but-idiosyncratic foreign spec — free-form headings (Overview / Background /
/// Acceptance Criteria), no canonical structure, no test references at all.
const FOREIGN_SPEC: &str = "\
# Authentication Spec

## Overview

Users must be able to sign in with OAuth before the product launches.

## Background

The product has no authentication today; it is required before the public launch.

## Acceptance Criteria

- A user can sign in with their Google account.
- A session expires 24 hours after sign-in.
";

/// The canonical rewrite payload — the goal/context fixed slots + a `criteria` section
/// carrying TWO `items`, each a `title` + a `statement` slot, with **NO `maps-to-test`**
/// (a migrated spec with no test anchors must finalize clean). The title slugs to
/// `auth-spec`, the canonical path `docs/specs/auth-spec.md`.
const PAYLOAD_SPEC: &str = r#"title: "Auth spec"
sections:
  - id: goal
    set:
      goal: "<<Users can authenticate via OAuth before launch.>>"
  - id: context
    set:
      context: "<<The product has no auth today; it is required before the public launch.>>"
  - id: criteria
    items:
      - title: "Sign in with Google"
        set:
          statement: "<<A user can sign in with their Google account.>>"
      - title: "Sessions expire"
        set:
          statement: "<<A session expires 24 hours after sign-in.>>"
"#;

#[test]
fn migrate_spec_finalize_writes_retires_and_adopts() {
    let repo = TempDir::new("headline");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed foreign spec at the off-canonical `docs/specs/` path; `docs/specs/` is empty
    // (the cold spike — migrate into a never-before-populated doctype location).
    let rel = commit_foreign_spec(repo.path(), "auth-spec", FOREIGN_SPEC);
    assert!(
        !repo.path().join("docs").join("specs").exists(),
        "the cold spike starts with no `docs/specs/` directory"
    );

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let composed = migrate(repo.path(), home.path(), &pack, &rel);
    assert!(
        composed.contains("sign in with their Google account"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );

    let task = migration_task(&rel);
    let authored = ok_stdout(
        author_spec(repo.path(), home.path(), &pack, &task, PAYLOAD_SPEC),
        "jigc doc author spec",
    );
    assert_eq!(
        authored, "spec:auth-spec",
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
        "finalize --approve on a conformant migration (criteria with NO maps-to-test) must \
         land (exit 0); stdout:\n{}\nstderr:\n{}",
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

    // (a) The canonical doc is committed at its per-title-slug path + byte-stable, and the
    // two criteria authored with NO maps-to-test re-parse.
    let body = assert_committed_byte_stable(repo.path(), &pack, "auth-spec");
    assert!(
        body.contains("Users can authenticate via OAuth")
            && body.contains("A user can sign in with their Google account"),
        "the committed spec carries the authored goal + criteria prose:\n{body}",
    );
    assert!(
        !body.contains("maps-to-test"),
        "a migrated spec with no test anchors carries no `maps-to-test` field:\n{body}",
    );
    let schema = shipped_spec_schema(&pack);
    let doc = engine::parse::parse_sections(&schema, &body).expect("committed spec re-parses");
    let criteria = doc
        .sections
        .iter()
        .find(|s| s.id == "criteria")
        .expect("criteria section present");
    assert_eq!(
        criteria.items.len(),
        2,
        "exactly the two migrated criteria re-parse; got:\n{body}"
    );

    // (b) The foreign original is GONE, and the SAME commit carries its deletion + the
    // added managed doc. `--no-renames` keeps the similar prose from collapsing the
    // delete+add into a single `R`.
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
        name_status.contains("A\tdocs/specs/auth-spec.md"),
        "the SAME commit carries the added managed spec:\n{name_status}"
    );

    // (c) A follow-up ingest reports it adopted.
    assert_ingest_adopted(repo.path(), home.path(), &pack, "auth-spec");
}

/// Extract the `doc author spec --from-file -` heredoc payload skeleton from the composed
/// migrate guidance (between the `<<'EOF'` opener and the standalone `EOF` terminator) —
/// the agent-facing artifact the LLM fills + pipes.
fn extract_author_skeleton(composed: &str) -> String {
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains("doc author spec --from-file") && line.contains("<<'EOF'") {
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
    panic!("no `doc author spec` heredoc skeleton in the composed migrate guidance:\n{composed}");
}

#[test]
fn shipped_guidance_payload_skeleton_parses_as_a_valid_spec_payload() {
    let repo = TempDir::new("guidance");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_spec(repo.path(), "auth-spec", FOREIGN_SPEC);
    let composed = migrate(repo.path(), home.path(), &pack, &rel);

    // The skeleton the agent actually fills must parse against the SAME payload parser
    // `doc author` runs — an undeclared field (rejected by deny_unknown_fields) would
    // silently break every per-guidance migration with a green spine test masking it.
    let skeleton = extract_author_skeleton(&composed);
    let schema = shipped_spec_schema(&pack);
    let parsed = cli::author::parse_author_payload(Some(&schema), &skeleton);
    assert!(
        parsed.is_ok(),
        "the shipped migrate-spec guidance payload skeleton must parse as a valid spec \
         author payload; skeleton:\n{skeleton}\nerror: {:?}",
        parsed.err(),
    );

    // M43 Inc-3 T4 — the generated `{{schema:spec}}` projection names `derived-from`
    // (the spec→prd edge the hand-written template omitted; law 1: a template cannot
    // understate the schema) at the RESOLVED docs-root home.
    assert!(
        composed.contains("`derived-from`: ref -> prd"),
        "the generated projection must name the `derived-from` edge; composed:\n{composed}",
    );
    assert!(
        composed.contains("`docs/specs/<slug>.md`"),
        "the spec home renders resolved through docs-root; composed:\n{composed}",
    );
}
