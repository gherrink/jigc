//! M24 Increment 1, T2 — `jigc doc author <doctype> --from-file <payload>`: the
//! declarative whole-doc batch verb, cold-authored through the real `jigc` binary.
//!
//! The headline B1 claim is that the batch path (create + N leaves over ONE
//! in-memory buffer, persisted once) produces the **same bytes** as the per-leaf
//! verb chain M23 proved byte-stable — so it inherits byte-stability, the
//! create-gate, and the squatter seam unchanged. This test drives the **emitted**
//! staged file both ways over the SHIPPED dev pack (`JIGC_PACK_DIR` = the embedded
//! `pack/` tree) and asserts, on a multi-release DATED changelog:
//!   (a) the batch-authored doc round-trips byte-stable (`render(parse(x)) == x`);
//!   (b) it is byte-identical to the equivalent per-leaf `create`/`add-item`/
//!       `set-field`/`set-slot` chain.
//!
//! Atomicity, the create-gate-through-the-batch, and the cold/empty spike are T3/T4.

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
            "jigc-doc-author-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit plus the `.jigc/config/` project layer.
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

/// The task's staged-docs directory in the working area.
fn docs_dir(repo: &Path, task: &str) -> PathBuf {
    repo.join(".jigc").join("tasks").join(task).join("docs")
}

/// The staged `changelog:changelog` instance in the task working area.
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = docs_dir(repo, task).join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The `.md` files currently staged in the task's docs directory (absent dir ⇒ none).
fn staged_docs(repo: &Path, task: &str) -> Vec<String> {
    let dir = docs_dir(repo, task);
    let mut names: Vec<String> = match fs::read_dir(&dir) {
        Ok(entries) => entries
            .filter_map(std::result::Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.ends_with(".md"))
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names
}

/// The shipped changelog schema, for the byte-stable round-trip assertion.
fn shipped_changelog_schema(pack: &Path) -> engine::schema::Schema {
    let yaml = fs::read(pack.join("schemas").join("changelog.yaml")).expect("read shipped schema");
    let mut schema = engine::schema::load_schema(&yaml).expect("shipped changelog schema loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The multi-release DATED changelog payload. Document order: release `1.2.0` (with
/// the optional `link` field + two nested change-groups), then release `1.1.0` (no
/// link, one nested group). The `<<…>>`-wrapped values are slot prose; the bare
/// scalar is the inline `link` field.
const PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        set:
          link: https://example.com/compare/1.1.0...1.2.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- OAuth device-code flow.>>"
              - title: Fixed
                set:
                  notes: "<<- Session fixation on logout.>>"
      - title: 1.1.0
        sections:
          - id: changes
            items:
              - title: Changed
                set:
                  notes: "<<- Default timeout raised to 30s.>>"
"#;

/// Bring a repo to a record-change task ready for changelog authoring (the gate
/// admits `changelog`). Returns the task id.
fn ready_repo(repo: &Path, home: &Path, pack: &Path, intent: &str) -> String {
    init_repo(repo);
    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["start", "--workflow", "record-change", intent],
            None,
        ),
        "jigc start --workflow record-change",
    );
    intent.replace(' ', "-")
}

/// Author the changelog in ONE `doc author` call from the declarative payload (stdin).
fn author_via_batch(repo: &Path, home: &Path, pack: &Path) -> String {
    let task = ready_repo(repo, home, pack, "author batch");
    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "author", "changelog", "--from-file", "-"],
            Some(PAYLOAD.as_bytes()),
        ),
        "jigc doc author changelog",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the batch verb prints the minted singleton address",
    );
    staged_changelog(repo, &task)
}

/// Author the SAME changelog through the per-leaf verb chain, in the exact document
/// order the payload parser lowers to — driving the EMITTED add-item addresses
/// verbatim downstream (never a reconstructed form).
fn author_via_leaf_chain(repo: &Path, home: &Path, pack: &Path) -> String {
    let task = ready_repo(repo, home, pack, "author chain");
    let run = |args: &[&str], stdin: Option<&[u8]>, what: &str| {
        ok_stdout(run_jigc(repo, home, pack, args, stdin), what)
    };

    assert_eq!(
        run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
            "create"
        ),
        "changelog:changelog",
    );

    // Release 1.2.0 + its optional link.
    let r120 = run(
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.2.0",
        ],
        None,
        "add 1.2.0",
    );
    run(
        &[
            "doc",
            "set-field",
            &format!("{r120}/link"),
            "--value",
            "https://example.com/compare/1.1.0...1.2.0",
        ],
        None,
        "set link",
    );
    // 1.2.0 nested change-groups, in order.
    let added = run(
        &[
            "doc",
            "add-item",
            &format!("{r120}/changes"),
            "--title",
            "Added",
        ],
        None,
        "add Added",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{added}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- OAuth device-code flow."),
        "notes Added",
    );
    let fixed = run(
        &[
            "doc",
            "add-item",
            &format!("{r120}/changes"),
            "--title",
            "Fixed",
        ],
        None,
        "add Fixed",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{fixed}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- Session fixation on logout."),
        "notes Fixed",
    );

    // Release 1.1.0 (no link) + its single change-group.
    let r110 = run(
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.1.0",
        ],
        None,
        "add 1.1.0",
    );
    let changed = run(
        &[
            "doc",
            "add-item",
            &format!("{r110}/changes"),
            "--title",
            "Changed",
        ],
        None,
        "add Changed",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{changed}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- Default timeout raised to 30s."),
        "notes Changed",
    );

    staged_changelog(repo, &task)
}

#[test]
fn batch_author_round_trips_and_matches_the_per_leaf_chain() {
    let home = TempDir::new("home");
    let pack = dev_pack();

    let batch_repo = TempDir::new("batch");
    let batch = author_via_batch(batch_repo.path(), home.path(), &pack);

    let chain_repo = TempDir::new("chain");
    let chain = author_via_leaf_chain(chain_repo.path(), home.path(), &pack);

    // (b) The batch-authored bytes are byte-identical to the per-leaf chain — the
    // load-bearing B1 claim (chain-the-primitives-minus-intermediate-persists).
    assert_eq!(
        batch, chain,
        "the batch-authored changelog is byte-identical to the per-leaf verb chain\n\
         batch:\n{batch}\n---\nchain:\n{chain}",
    );

    // The authored content actually landed (not two identically-empty docs).
    assert!(
        batch.contains("1.2.0")
            && batch.contains("1.1.0")
            && batch.contains("OAuth device-code flow."),
        "the multi-release dated content is authored; staged:\n{batch}",
    );

    // (a) Byte-stable: render(parse(batch)) == batch.
    let schema = shipped_changelog_schema(&pack);
    let parsed =
        engine::write::instance_from_source(&schema, &batch).expect("batch changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        batch,
        "the batch-authored changelog is byte-stable across parse → render",
    );
}

#[test]
fn doc_author_emits_the_json_envelope() {
    // The command-output contract (`design/command-output-contract.md` §2): `author` joins
    // the ack envelope — `op: author` + the decomposed `target` (a **whole-doc** write, so
    // the head only — `doctype`+`slug`, no section/item/leaf) + `findings: []`. Before this
    // task `author` printed a **bare address string** (invalid JSON), outside the envelope.
    let home = TempDir::new("home");
    let pack = dev_pack();
    let repo = TempDir::new("author-json");
    ready_repo(repo.path(), home.path(), &pack, "author json");

    let stdout = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "author",
                "changelog",
                "--from-file",
                "-",
                "--format",
                "json",
            ],
            Some(PAYLOAD.as_bytes()),
        ),
        "jigc doc author changelog --format json",
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("author json ack parses");
    assert_eq!(ack["op"], "author");
    assert_eq!(ack["target"]["doctype"], "changelog");
    assert_eq!(ack["target"]["slug"], "changelog");
    assert!(
        ack["target"]["section"].is_null(),
        "a whole-doc author reaches no section; got:\n{stdout}"
    );
    assert!(
        ack["target"]["item"].is_null(),
        "a whole-doc author reaches no item; got:\n{stdout}"
    );
    assert!(
        ack["target"]["leaf"].is_null(),
        "a whole-doc author reaches no leaf; got:\n{stdout}"
    );
    assert_eq!(ack["findings"], serde_json::json!([]));
}

/// A payload whose leaves all parse cleanly but whose **last** leaf is a structural
/// reject: a first release authors fully (create + add-item + nested add-item + slot,
/// all over the in-memory buffer), then an `add-item` into an undeclared section
/// `nonsuch` fails at the engine splice (`write.unknown-section`). The create already
/// persisted the empty singleton before the chain, so atomicity demands the staged
/// doc be **gone** after the mid-chain failure — the "rejected whole, nothing
/// persisted" contract (`design/auto-migration.md` → Hardening #1).
const MID_CHAIN_BAD_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- OAuth device-code flow.>>"
  - id: nonsuch
    items:
      - title: Boom
"#;

#[test]
fn mid_chain_leaf_failure_stages_nothing() {
    let home = TempDir::new("home");
    let pack = dev_pack();
    let repo = TempDir::new("midchain");
    let task = ready_repo(repo.path(), home.path(), &pack, "author midchain");
    // The workflow itself provisions a `commit` doc at start; the batch must add
    // nothing beyond that baseline once it fails.
    let baseline = staged_docs(repo.path(), &task);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "author", "changelog", "--from-file", "-"],
        Some(MID_CHAIN_BAD_PAYLOAD.as_bytes()),
    );

    // Non-zero exit carrying the offending leaf's BLOCK finding (the engine's
    // unknown-section reject, surfaced through the batch verb's `add-item` block).
    assert!(
        !out.status.success(),
        "a mid-chain bad leaf must fail the whole batch; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("write.unknown-section"),
        "the failure surfaces the failing leaf's block finding; stderr:\n{stderr}",
    );

    // Atomicity: the empty changelog `create_gated` staged before the chain must be
    // rolled back, so the staged set is byte-for-byte the pre-author baseline — the
    // batch persisted nothing.
    assert_eq!(
        staged_docs(repo.path(), &task),
        baseline,
        "a mid-chain failure leaves the staged set unchanged from the pre-author baseline",
    );
}

/// A payload whose leaves all parse cleanly but whose **last** nested change-group
/// carries a non-member `category`: a first release authors fully (create + add-item +
/// nested add-item + slot), then a `changes` group titled `Improvements` re-slugs to
/// `improvements` — outside the `category` enum (`added`/`changed`/…/`security`). The
/// `id-from: category` enum reject (M24 inc-2 T1) must fire **mid-chain** through the
/// batch's shared `add-item` path, not be deferred to finalize, so a long authoring run
/// fails fast at the point of the mistake (`design/auto-migration.md` → Hardening #3;
/// `design/write-commands.md` → Two check times). The create already persisted the empty
/// singleton before the chain, so atomicity demands nothing stays staged.
const BAD_CATEGORY_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- OAuth device-code flow.>>"
              - title: Improvements
"#;

#[test]
fn batch_with_non_member_category_stages_nothing() {
    let home = TempDir::new("home");
    let pack = dev_pack();
    let repo = TempDir::new("bad-category");
    let task = ready_repo(repo.path(), home.path(), &pack, "author bad category");
    let baseline = staged_docs(repo.path(), &task);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "author", "changelog", "--from-file", "-"],
        Some(BAD_CATEGORY_PAYLOAD.as_bytes()),
    );

    // Non-zero exit carrying the **shared** id-from-enum block code — identical to
    // finalize's (`schema-conformance.field-value-conformant`); the batch inherits the
    // write-verb reject by chaining the same `add-item` primitive (reuse is verified, not
    // assumed — red if the batch's add-item path bypasses the check).
    assert!(
        !out.status.success(),
        "a batch leaf with a non-member category must fail the whole batch; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("schema-conformance.field-value-conformant"),
        "the failure surfaces the shared id-from-enum block code; stderr:\n{stderr}",
    );
    // The block addresses the slug-cased id-from leaf, not the raw `--title`.
    assert!(
        stderr.contains("improvements"),
        "the block names the slug-cased offending category; stderr:\n{stderr}",
    );

    // Atomicity: the empty changelog `create_gated` staged before the chain must roll
    // back, so the staged set is byte-for-byte the pre-author baseline — nothing persisted.
    assert_eq!(
        staged_docs(repo.path(), &task),
        baseline,
        "a non-member-category batch leaves the staged set unchanged from the pre-author baseline",
    );
}

/// A payload that parses structurally but whose `notes` **slot** carries a **bare**
/// (un-`<<…>>`-wrapped) value — the silent-misroute trap. Before the leaf-kind
/// cross-check, the bare value classified as an inline *field* and was silently
/// written into a trailing `<!-- fields -->` block, leaving the slot empty with NO
/// error. The cross-check now rejects the whole payload at parse — before any persist
/// — naming the offending slot address (`design/auto-migration.md` → Hardening; the
/// `<<…>>` convention itself is unchanged, only the deviation is made loud).
const BARE_SLOT_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.2.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "- OAuth device-code flow."
"#;

#[test]
fn bare_slot_value_is_rejected_and_stages_nothing() {
    let home = TempDir::new("home");
    let pack = dev_pack();
    let repo = TempDir::new("bare-slot");
    let task = ready_repo(repo.path(), home.path(), &pack, "author bare slot");
    let baseline = staged_docs(repo.path(), &task);

    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "author", "changelog", "--from-file", "-"],
        Some(BARE_SLOT_PAYLOAD.as_bytes()),
    );

    // Non-zero exit: the slot/field cross-check rejects the whole payload at parse.
    assert!(
        !out.status.success(),
        "a bare value for a slot leaf must reject the whole batch; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("slot") && stderr.contains("notes") && stderr.contains("<<"),
        "the reject names the slot, its address, and the expected `<<…>>` form; stderr:\n{stderr}",
    );

    // Atomicity: the reject is at parse, before `create_gated`, so the staged set is
    // byte-for-byte the pre-author baseline — nothing persisted, no misrouted fields
    // block, no silently-emptied slot.
    assert_eq!(
        staged_docs(repo.path(), &task),
        baseline,
        "a bare-slot reject leaves the staged set unchanged from the pre-author baseline",
    );
}

/// A SINGLE-release dated changelog — the cold spike's minimal multi-leaf case (one
/// release with its optional `link`, one nested change-group carrying slot prose). It
/// exercises the same create + add-item + set-field + set-slot chain as the marquee
/// multi-release payload, at the smallest non-empty scale.
const SINGLE_RELEASE_PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 1.0.0
        set:
          link: https://example.com/releases/1.0.0
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- Initial public release.>>"
"#;

/// The create-only payload — no `sections`, so the parser lowers it to ZERO leaves.
/// It drives the cold path: `create_gated` then persist once, no chained leaves.
const EMPTY_PAYLOAD: &str = "title: Changelog\n";

/// Author a changelog in ONE `doc author` call from an arbitrary declarative payload,
/// returning the staged bytes. (The marquee test's `author_via_batch` is hardcoded to
/// the multi-release payload; the spike needs the same call over varying input.)
fn author_payload_via_batch(
    repo: &Path,
    home: &Path,
    pack: &Path,
    intent: &str,
    payload: &str,
) -> String {
    let task = ready_repo(repo, home, pack, intent);
    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "author", "changelog", "--from-file", "-"],
            Some(payload.as_bytes()),
        ),
        "jigc doc author changelog",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the batch verb prints the minted singleton address",
    );
    staged_changelog(repo, &task)
}

/// Author the single-release changelog through the per-leaf verb chain, in document
/// order — driving the EMITTED add-item addresses verbatim downstream.
fn author_single_release_via_leaf(repo: &Path, home: &Path, pack: &Path) -> String {
    let task = ready_repo(repo, home, pack, "single chain");
    let run = |args: &[&str], stdin: Option<&[u8]>, what: &str| {
        ok_stdout(run_jigc(repo, home, pack, args, stdin), what)
    };

    assert_eq!(
        run(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
            "create"
        ),
        "changelog:changelog",
    );
    let r100 = run(
        &[
            "doc",
            "add-item",
            "changelog:changelog#releases",
            "--title",
            "1.0.0",
        ],
        None,
        "add 1.0.0",
    );
    run(
        &[
            "doc",
            "set-field",
            &format!("{r100}/link"),
            "--value",
            "https://example.com/releases/1.0.0",
        ],
        None,
        "set link",
    );
    let added = run(
        &[
            "doc",
            "add-item",
            &format!("{r100}/changes"),
            "--title",
            "Added",
        ],
        None,
        "add Added",
    );
    run(
        &[
            "doc",
            "set-slot",
            &format!("{added}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"- Initial public release."),
        "notes Added",
    );

    staged_changelog(repo, &task)
}

/// Author a bare changelog through the per-leaf `create` verb alone — the create-only
/// equivalent of the empty batch payload.
fn author_create_only_via_leaf(repo: &Path, home: &Path, pack: &Path) -> String {
    let task = ready_repo(repo, home, pack, "create only chain");
    assert_eq!(
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &["doc", "create", "changelog", "--title", "Changelog"],
                None,
            ),
            "jigc doc create changelog",
        ),
        "changelog:changelog",
    );
    staged_changelog(repo, &task)
}

/// Assert a staged changelog round-trips byte-stable over the shipped schema:
/// `render(parse(bytes)) == bytes`.
fn assert_byte_stable(pack: &Path, bytes: &str) {
    let schema = shipped_changelog_schema(pack);
    let parsed =
        engine::write::instance_from_source(&schema, bytes).expect("staged changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        bytes,
        "the batch-authored changelog is byte-stable across parse → render",
    );
}

#[test]
fn single_release_batch_round_trips_and_matches_the_per_leaf_chain() {
    let home = TempDir::new("home");
    let pack = dev_pack();

    let batch_repo = TempDir::new("single-batch");
    let batch = author_payload_via_batch(
        batch_repo.path(),
        home.path(),
        &pack,
        "single batch",
        SINGLE_RELEASE_PAYLOAD,
    );

    let chain_repo = TempDir::new("single-chain");
    let chain = author_single_release_via_leaf(chain_repo.path(), home.path(), &pack);

    // The single-release batch bytes are byte-identical to the per-leaf chain.
    assert_eq!(
        batch, chain,
        "the single-release batch changelog is byte-identical to the per-leaf chain\n\
         batch:\n{batch}\n---\nchain:\n{chain}",
    );

    // The authored content actually landed.
    assert!(
        batch.contains("1.0.0") && batch.contains("Initial public release."),
        "the single-release content is authored; staged:\n{batch}",
    );

    assert_byte_stable(&pack, &batch);
}

#[test]
fn empty_payload_authors_a_byte_stable_create_only_instance() {
    let home = TempDir::new("home");
    let pack = dev_pack();

    let batch_repo = TempDir::new("empty-batch");
    let batch = author_payload_via_batch(
        batch_repo.path(),
        home.path(),
        &pack,
        "empty batch",
        EMPTY_PAYLOAD,
    );

    // A create-only batch (zero leaves) is byte-identical to a bare `doc create`: the
    // cold path is just `create_gated` + persist once.
    let chain_repo = TempDir::new("empty-chain");
    let chain = author_create_only_via_leaf(chain_repo.path(), home.path(), &pack);
    assert_eq!(
        batch, chain,
        "the empty (create-only) batch is byte-identical to a bare `doc create`\n\
         batch:\n{batch}\n---\nchain:\n{chain}",
    );

    assert_byte_stable(&pack, &batch);
}

#[test]
fn disallowed_doctype_is_gate_blocked_through_the_batch() {
    let home = TempDir::new("home");
    let pack = dev_pack();
    let repo = TempDir::new("gate");
    let task = ready_repo(repo.path(), home.path(), &pack, "author gate");
    let baseline = staged_docs(repo.path(), &task);

    // `commit` is a shipped doctype the `record-change` gate does NOT admit — the
    // create-gate is inherited because the batch creates through `create_gated`.
    let out = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["doc", "author", "commit", "--from-file", "-"],
        Some(b"title: x\n"),
    );

    assert!(
        !out.status.success(),
        "an un-allowed doctype must be gate-blocked through the batch; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("create.gate-blocked"),
        "the gate-block finding fires through the batch; stderr:\n{stderr}",
    );

    // The gate rejects before any provision, so the staged set is unchanged from the
    // pre-author baseline — the batch stages nothing.
    assert_eq!(
        staged_docs(repo.path(), &task),
        baseline,
        "a gate-blocked author leaves the staged set unchanged from the pre-author baseline",
    );
}
