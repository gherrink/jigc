//! M26 Increment 2, T1 — end-to-end arch-doc migration acceptance (the deliverable's
//! Proves). Drives the **built** `jigc` binary against throwaway `git init` temp repos
//! over the **shipped** dev pack (`JIGC_PACK_DIR` = the embedded `packs/dev/` tree) with the
//! real `doc-code` probe (no `JIGC_DOC_CODE_PROBE` override), exercising — not assuming —
//! the doctype-general migrate → author → review-gate → retire → adopt spine on
//! `arch-doc`, the migration arc's last and hardest doctype: its repeatable `components`
//! carry **both** a `description` slot **and** an `implemented-by` code-anchor field, and
//! those anchors resolve against real Rust ([auto-migration.md](../../../design/auto-migration.md)
//! → Generalizing to arch-doc (M26) + The author-migration-arch-doc guidance;
//! [worked-examples.md](../../../design/worked-examples.md) → flow 28; the migrate-adr /
//! migrate-spec workflows as the copy template):
//!
//!   - **Smoke (green)** — a synthetic-over-Rust foreign arch-doc (committed,
//!     off-canonical at `docs/architecture/`) migrates: `jigc migrate <path> --as arch-doc`
//!     mints `migrate-arch-doc-<slug>` and composes (no longer errors `no workflow`),
//!     surfacing the foreign content through the source seam; the agent authors the
//!     canonical record through `doc author arch-doc --from-file -` (the `overview` slot + a
//!     `components` item whose `implemented-by` anchors a real `path#symbol` present in the
//!     repo's Rust); and `task finalize --approve` lands exactly ONE commit that (a) writes
//!     `docs/architecture/<slug>.md` byte-stable (`render(instance_from_source(x)) == x`) with
//!     one resolving `implemented-by`, (b) carries `A docs/architecture/<slug>.md` + `D
//!     <foreign>` (the foreign original retired/gone) in the SAME commit. The review gate
//!     blocks the bare finalize first (nothing committed).
//!   - **Reds fire** — a dangling `cites [adr:<absent>]` blocks at
//!     `schema-conformance.ref-resolves`; an unfilled `description` blocks at
//!     `schema-conformance.required-slot-present`; a vanished anchored symbol (deleted
//!     before finalize) blocks at `doc-code.symbol-exists` naming the offending item
//!     address `arch-doc:<slug>#components/<id>/implemented-by`.
//!   - **Guidance skeleton** — the shipped `migrate-arch-doc` guidance's emitted `doc
//!     author` payload skeleton parses as a valid arch-doc author payload (the agent-facing
//!     artifact is the contract; a skeleton the parser rejects would silently break every
//!     per-guidance migration with a green spine test masking it).

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
            "jigc-migrate-arch-doc-{tag}-{}-{:?}",
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

/// The component source file carrying the present Rust symbol the `implemented-by` anchor
/// resolves against — a throwaway fixture target (never a live jigc dependency), so the
/// red can delete the symbol without breaking jigc's own build. The anchor authored into
/// the payload is `src/edge_index.rs#rebuild_committed`.
const COMPONENT_FILE: &str = "src/edge_index.rs";

/// The component source carrying its present symbol (the version that resolves clean).
fn component_present() -> &'static str {
    "pub fn rebuild_committed() -> u32 {\n    0\n}\n"
}

/// The component source with the symbol **deleted** (renamed away) — the file stays, so
/// this is a symbol-absent block, not a file-absent one.
fn component_deleted() -> &'static str {
    "pub fn rebuilt_renamed() -> u32 {\n    0\n}\n"
}

/// Initialize a real git repo with one commit, the `.jigc/config/` project layer, and a
/// `src/` Rust file carrying the present symbol the migrated anchor resolves against (the
/// doc-code probe is Rust-grammar-only).
fn init_repo(root: &Path) {
    git(root, &["init", "-q"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "Test"]);
    fs::create_dir_all(root.join("src")).expect("mk src");
    fs::write(root.join(COMPONENT_FILE), component_present()).expect("write component source");
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(root.join(".jigc").join("config")).expect("create project layer");
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`, and
/// the `JIGC_DOC_CODE_PROBE` override removed (the real probe resolves through the
/// production path), optionally piping `stdin`.
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
        .env_remove("JIGC_DOC_CODE_PROBE")
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

/// The shipped `arch-doc` schema, loaded for the byte-stable round-trip + the guidance
/// skeleton cross-check. `implemented-by` is a pack-declared `code-anchor`, so the schema
/// only loads with that type threaded in (mirrors the shipped pack's `code-anchor →
/// doc-code` decl).
fn shipped_arch_doc_schema(pack: &Path) -> engine::schema::Schema {
    let yaml =
        fs::read(pack.join("schemas").join("arch-doc.yaml")).expect("read shipped arch-doc schema");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
        hint: None,
    }];
    let mut schema = engine::schema::load_schema_with_types(&yaml, &types)
        .expect("shipped arch-doc schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The per-file migration task id for a repo-relative source `rel` — the production
/// derivation ([`crate::start::migration_task_id`]): `migrate-arch-doc-<slug>-<hash>`,
/// where `<slug>` is the extension-stripped, separator-folded, slugified path and
/// `<hash>` is a 12-hex-char prefix of `blake3(rel)` (the M44 fork-1 disambiguator).
fn migration_task(rel: &str) -> String {
    let hash = &engine::file_state::hash_bytes(rel.as_bytes())[..12];
    let stem = rel.strip_suffix(".md").unwrap_or(rel);
    let folded: String = stem
        .chars()
        .map(|c| if c == '/' { '-' } else { c })
        .collect();
    let slug = engine::slug::slugify(&folded);
    if slug.is_empty() {
        format!("migrate-arch-doc-{hash}")
    } else {
        format!("migrate-arch-doc-{slug}-{hash}")
    }
}

/// Write a foreign arch-doc at `docs/architecture/<stem>.md` and **commit it**, so the
/// retire surfaces a tracked deletion in the finalize commit (the realistic flow — a
/// pre-existing committed architecture document). Returns the repo-relative path.
fn commit_foreign_arch_doc(repo: &Path, stem: &str, body: &str) -> String {
    // The canonical home is now `docs/architecture/` (the `docs-root` nesting), so flat
    // `architecture/` is the off-canonical foreign location.
    let rel = format!("architecture/{stem}.md");
    let path = repo.join(&rel);
    fs::create_dir_all(path.parent().unwrap()).expect("create architecture/");
    fs::write(&path, body).expect("write foreign arch-doc");
    git(repo, &["add", &rel]);
    git(repo, &["commit", "-q", "-m", "track foreign arch-doc"]);
    rel
}

/// `jigc migrate <rel> --as arch-doc` — returns the composed migration workflow's stdout
/// (which surfaces the foreign content through the source seam and the author guidance).
fn migrate(repo: &Path, home: &Path, pack: &Path, rel: &str) -> String {
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["migrate", rel, "--as", "arch-doc"],
            None,
        ),
        "jigc migrate <arch-doc> --as arch-doc",
    )
}

/// `jigc doc author arch-doc --from-file -` over `task`, piping the declarative `payload`.
fn author_arch_doc(
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
        &[
            "doc",
            "author",
            "arch-doc",
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(payload.as_bytes()),
    )
}

/// `task finalize <task> --approve` raw output (the reds assert a BLOCK, so they cannot
/// use a success-asserting tail).
fn finalize_approve(repo: &Path, home: &Path, pack: &Path, task: &str) -> std::process::Output {
    run_jigc(
        repo,
        home,
        pack,
        &["task", "finalize", task, "--approve"],
        None,
    )
}

/// `task finalize <task> --approve` rendered as JSON — the report-with-location surface
/// whose `location.address` carries the per-item anchor address the doc-code red asserts on
/// (the human format prints only the bare anchor string, never the item address).
fn finalize_approve_json(
    repo: &Path,
    home: &Path,
    pack: &Path,
    task: &str,
) -> std::process::Output {
    run_jigc(
        repo,
        home,
        pack,
        &["--format", "json", "task", "finalize", task, "--approve"],
        None,
    )
}

/// Assert the canonical `docs/architecture/<slug>.md` on disk round-trips byte-stable through
/// the shipped arch-doc schema: `render(&schema, &instance_from_source(&schema, x)) == x`.
fn assert_committed_byte_stable(repo: &Path, pack: &Path, slug: &str) -> String {
    let on_disk = fs::read_to_string(
        repo.join("docs")
            .join("architecture")
            .join(format!("{slug}.md")),
    )
    .expect("the canonical arch-doc is on disk");
    let schema = shipped_arch_doc_schema(pack);
    let parsed = engine::write::instance_from_source(&schema, &on_disk)
        .expect("the committed arch-doc re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        on_disk,
        "the migrated arch-doc is byte-stable across parse -> render:\n{on_disk}",
    );
    on_disk
}

/// A synthetic-over-Rust foreign architecture document (labeled synthetic, the M25
/// prd-arm honesty posture) — free-form headings the agent rewrites onto the canonical
/// `overview` / `components` shape.
const FOREIGN_ARCH_DOC: &str = "\
# Index Layer

## Overview

The index layer owns edge derivation and the target surface for the document store.

## Edge index component

Builds the committed edge index from the parsed documents, implemented in
src/edge_index.rs by rebuild_committed.
";

/// The canonical rewrite payload — the `overview` slot + a `components` item carrying a
/// filled `description` slot AND an `implemented-by` anchor at the present Rust symbol
/// (NO `cites` — the smoke proves a clean resolving anchor, the C2 cites-over-committed-adr
/// mandate is Increment 3). The title slugs to `index-layer` → `docs/architecture/index-layer.md`.
const PAYLOAD_ARCH_DOC: &str = r#"title: "Index layer"
sections:
  - id: overview
    set:
      overview: "<<The index layer owns edge derivation and the target surface.>>"
  - id: components
    items:
      - title: "Edge index"
        set:
          description: "<<Builds the committed edge index from the parsed documents.>>"
          implemented-by: "src/edge_index.rs#rebuild_committed"
"#;

#[test]
fn migrate_arch_doc_smoke_finalize_writes_retires_and_adopts() {
    let repo = TempDir::new("smoke");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());

    // A committed foreign arch-doc at the off-canonical `docs/architecture/` path;
    // `docs/architecture/` is empty (the cold spike — migrate into a never-populated location).
    let rel = commit_foreign_arch_doc(repo.path(), "index-layer", FOREIGN_ARCH_DOC);
    assert!(
        !repo.path().join("docs").join("architecture").exists(),
        "the cold spike starts with no `docs/architecture/` directory"
    );

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );

    // `migrate --as arch-doc` no longer errors `no workflow` — it composes, surfacing the
    // foreign content through the source seam.
    let composed = migrate(repo.path(), home.path(), &pack, &rel);
    assert!(
        composed.contains("Builds the committed edge index"),
        "the composed migrate workflow surfaces the foreign content through the source \
         seam; stdout:\n{composed}",
    );

    let task = migration_task(&rel);
    let authored = ok_stdout(
        author_arch_doc(repo.path(), home.path(), &pack, &task, PAYLOAD_ARCH_DOC),
        "jigc doc author arch-doc",
    );
    assert_eq!(
        authored, "arch-doc:index-layer",
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

    // --approve: write the canonical doc, retire the foreign original, commit, adopt.
    let approve = finalize_approve(repo.path(), home.path(), &pack, &task);
    assert!(
        approve.status.success(),
        "finalize --approve on a conformant migration (one resolving implemented-by) must \
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

    // (a) The canonical doc is committed at its per-title-slug path + byte-stable (the
    // slot+field-group component round-trips on the Increment-1 substrate), and carries the
    // resolving anchor.
    let body = assert_committed_byte_stable(repo.path(), &pack, "index-layer");
    assert!(
        body.contains("The index layer owns edge derivation")
            && body.contains("src/edge_index.rs#rebuild_committed"),
        "the committed arch-doc carries the overview prose + the resolving anchor:\n{body}",
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
        name_status.contains("A\tdocs/architecture/index-layer.md"),
        "the SAME commit carries the added managed arch-doc:\n{name_status}"
    );
}

/// RED 1 — a dangling `cites [adr:<absent>]` (no such adr in the store or working area)
/// blocks finalize at `schema-conformance.ref-resolves`, naming the dangling target. The
/// resolving anchor is held constant so the cites is the sole block lever.
#[test]
fn a_dangling_cites_blocks_at_ref_resolves() {
    let repo = TempDir::new("red-cites");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_arch_doc(repo.path(), "index-layer", FOREIGN_ARCH_DOC);
    migrate(repo.path(), home.path(), &pack, &rel);
    let task = migration_task(&rel);

    let payload = r#"title: "Index layer"
sections:
  - id: meta
    set:
      cites: "[adr:no-such-decision]"
  - id: overview
    set:
      overview: "<<The index layer owns edge derivation and the target surface.>>"
  - id: components
    items:
      - title: "Edge index"
        set:
          description: "<<Builds the committed edge index.>>"
          implemented-by: "src/edge_index.rs#rebuild_committed"
"#;
    ok_stdout(
        author_arch_doc(repo.path(), home.path(), &pack, &task, payload),
        "jigc doc author arch-doc (dangling cites)",
    );

    let out = finalize_approve(repo.path(), home.path(), &pack, &task);
    assert!(
        !out.status.success(),
        "a dangling cites must block finalize (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let streams = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        streams.contains("schema-conformance.ref-resolves")
            && streams.contains("adr:no-such-decision"),
        "the block is ref-resolves naming the dangling cites target:\n{streams}",
    );
}

/// RED 2 — an unfilled component `description` (omitted from the item `set`) blocks
/// finalize at `schema-conformance.required-slot-present`. The anchor resolves and there
/// is no cites, so the empty description is the sole block lever.
#[test]
fn an_unfilled_description_blocks_at_required_slot_present() {
    let repo = TempDir::new("red-desc");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_arch_doc(repo.path(), "index-layer", FOREIGN_ARCH_DOC);
    migrate(repo.path(), home.path(), &pack, &rel);
    let task = migration_task(&rel);

    // The component carries its `implemented-by` anchor but NO `description` — the required
    // slot is left empty.
    let payload = r#"title: "Index layer"
sections:
  - id: overview
    set:
      overview: "<<The index layer owns edge derivation and the target surface.>>"
  - id: components
    items:
      - title: "Edge index"
        set:
          implemented-by: "src/edge_index.rs#rebuild_committed"
"#;
    ok_stdout(
        author_arch_doc(repo.path(), home.path(), &pack, &task, payload),
        "jigc doc author arch-doc (no description)",
    );

    let out = finalize_approve(repo.path(), home.path(), &pack, &task);
    assert!(
        !out.status.success(),
        "an unfilled component description must block finalize (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let streams = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        streams.contains("schema-conformance.required-slot-present"),
        "the block is required-slot-present on the empty description:\n{streams}",
    );
}

/// RED 3 — a present `implemented-by` whose anchored symbol VANISHES before finalize
/// (deleted from the working tree) blocks at `doc-code.symbol-exists`, naming the offending
/// item address `arch-doc:<slug>#components/<id>/implemented-by`. Authored clean (the symbol
/// is present at author time), then deleted before the finalize.
#[test]
fn a_vanished_anchored_symbol_blocks_at_doc_code_symbol_exists() {
    let repo = TempDir::new("red-symbol");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_arch_doc(repo.path(), "index-layer", FOREIGN_ARCH_DOC);
    migrate(repo.path(), home.path(), &pack, &rel);
    let task = migration_task(&rel);

    ok_stdout(
        author_arch_doc(repo.path(), home.path(), &pack, &task, PAYLOAD_ARCH_DOC),
        "jigc doc author arch-doc",
    );

    // Delete the anchored symbol (rename it away); the file stays, so this is a symbol-absent
    // block, not a file-absent one.
    fs::write(repo.path().join(COMPONENT_FILE), component_deleted())
        .expect("delete the anchored symbol");
    // Stage the deletion so the finalize-scope `doc-code` probe — which validates the git
    // index, not the working tree (M30 Inc 3, G4) — sees the symbol gone.
    git(repo.path(), &["add", COMPONENT_FILE]);

    let out = finalize_approve_json(repo.path(), home.path(), &pack, &task);
    assert!(
        !out.status.success(),
        "a vanished anchored symbol must block finalize (exit non-zero); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let streams = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    assert!(
        streams.contains("doc-code.symbol-exists")
            && streams.contains("arch-doc:index-layer#components/edge-index/implemented-by"),
        "the block is doc-code.symbol-exists naming the offending item anchor address:\n{streams}",
    );
}

/// Extract the `doc author arch-doc --from-file -` heredoc payload skeleton from the composed
/// migrate guidance (between the `<<'EOF'` opener and the standalone `EOF` terminator) —
/// the agent-facing artifact the LLM fills + pipes.
fn extract_author_skeleton(composed: &str) -> String {
    let mut lines = composed.lines();
    for line in lines.by_ref() {
        if line.contains("doc author arch-doc --from-file") && line.contains("<<'EOF'") {
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
        "no `doc author arch-doc` heredoc skeleton in the composed migrate guidance:\n{composed}"
    );
}

#[test]
fn shipped_guidance_payload_skeleton_parses_as_a_valid_arch_doc_payload() {
    let repo = TempDir::new("guidance");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"], None),
        "jigc setup",
    );
    let rel = commit_foreign_arch_doc(repo.path(), "index-layer", FOREIGN_ARCH_DOC);
    let composed = migrate(repo.path(), home.path(), &pack, &rel);

    // The skeleton the agent actually fills must parse against the SAME payload parser
    // `doc author` runs — an undeclared field or a mis-wrapped slot/field value would
    // silently break every per-guidance migration with a green spine test masking it.
    let skeleton = extract_author_skeleton(&composed);
    let schema = shipped_arch_doc_schema(&pack);
    let parsed = cli::author::parse_author_payload(Some(&schema), &skeleton);
    assert!(
        parsed.is_ok(),
        "the shipped migrate-arch-doc guidance payload skeleton must parse as a valid \
         arch-doc author payload; skeleton:\n{skeleton}\nerror: {:?}",
        parsed.err(),
    );
}
