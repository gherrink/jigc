//! M13 Increment 5 acceptance (T1) — the `## Components` schema-ordered
//! materialization named check, in isolation (before T2's headline rides on it).
//!
//! `design/architecture-documentation.md` → The acceptance flow (the `## Components`
//! schema-ordered materialization named check) + Honest caveats (provisioning an
//! empty repeatable home); `implementation/roadmap.md` → M13 Increment 5, grouped
//! scope bullet 1. The create→first-`add-item` handoff: `jigc doc create arch-doc`
//! yields a doc with **no** `## Components` home (the repeatable section is empty),
//! and the FIRST `jigc doc add-item arch-doc:<slug>#components` materializes that
//! home at its **schema-ordered** position — after `## Overview`, never before it
//! and never appended at EOF ahead of it. The minted `### <title>  {#<id>}` sits
//! under the new `## Components`, and the **emitted** item address (captured from
//! `add-item` stdout, run verbatim — increment-workflow.md hardening #4) is
//! `arch-doc:<slug>#components/<id>`. A follow-up `set-slot …/description` +
//! `set-field …/implemented-by` over that emitted address round-trips byte-stable.
//!
//! No external test crates: the binary path comes from `CARGO_BIN_EXE_jigc`, the
//! temp repo is a real `git init` (`jigc start` reads HEAD), and self-cleaning
//! `TempDir`s keep the test off the developer's repo.

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
            "jigc-arch-mat-{tag}-{}-{:?}",
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

/// Initialize a real git repo with one commit + the `.jigc/config/` project layer,
/// then `jigc start --workflow architecture-documentation "<intent>"` to mint the
/// task (the workflow `allows-create: [{type: arch-doc, as: arch-doc}]`, the
/// create-gate the doc authoring needs). Returns the repo + a `$HOME` temp dir.
fn arch_started_repo(intent: &str) -> (TempDir, TempDir) {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .args(args)
            .current_dir(repo.path())
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
    fs::write(repo.path().join("README.md"), "hello\n").expect("write file");
    git(&["add", "."]);
    git(&["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.path().join(".jigc").join("config")).expect("create project layer");

    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(["start", "--workflow", "architecture-documentation", intent])
        .current_dir(repo.path())
        .env("HOME", home.path())
        .output()
        .expect("run jigc start");
    assert!(
        out.status.success(),
        "`jigc start --workflow architecture-documentation` must provision the task; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    (repo, home)
}

/// Run `jigc doc <args>` with `cwd = repo`, optionally piping `stdin`.
fn run_doc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command.arg("doc");
    command.args(args);
    command.current_dir(repo).env("HOME", home);
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
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

/// Assert a `jigc doc` invocation succeeded, surfacing both streams on failure.
fn assert_ok(out: &std::process::Output, what: &str) {
    assert!(
        out.status.success(),
        "{what} must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// The staged `arch-doc:<slug>` instance in the task working area.
fn staged_arch_doc(repo: &Path, task: &str, slug: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("arch-doc:{slug}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The shipped `arch-doc` schema, loaded from the embedded pack source with the
/// dev-pack `code-anchor` field-type (adjudicator `doc-code`, check `symbol-exists`)
/// so the byte-stable round-trip asserts against exactly the bytes that ship
/// (mirrors `item_authoring_acceptance::spec_schema`).
fn arch_doc_schema() -> engine::schema::Schema {
    const ARCH_YAML: &[u8] = include_bytes!("../pack/schemas/arch-doc.yaml");
    let types = vec![engine::schema::PackTypeDecl {
        name: "code-anchor".to_owned(),
        adjudicator: "doc-code".to_owned(),
        check: "symbol-exists".to_owned(),
        hint: None,
    }];
    let mut schema =
        engine::schema::load_schema_with_types(ARCH_YAML, &types).expect("arch-doc.yaml loads");
    engine::schema::inject_schema_version_stamp(&mut schema);
    schema
}

/// The byte offset of a `## <Heading>` line in `body`, or `None` if absent. Matches
/// the heading at column 0 so a `### <component>` item heading never aliases the
/// `## Components` section heading.
fn section_heading_offset(body: &str, heading: &str) -> Option<usize> {
    let needle = format!("## {heading}\n");
    body.match_indices(&needle)
        .find(|(idx, _)| *idx == 0 || body.as_bytes()[idx - 1] == b'\n')
        .map(|(idx, _)| idx)
}

/// The full T1 arc: author the `overview`, then prove the FIRST `add-item`
/// materializes `## Components` after `## Overview` in schema order, the minted item
/// heading sits under it, the emitted address is the canonical item address, and a
/// follow-up slot+field over that emitted address round-trips byte-stable.
#[test]
fn first_add_item_materializes_components_after_overview_in_schema_order() {
    let (repo, home) = arch_started_repo("document the storage layer");
    let task = "document-the-storage-layer";
    let slug = "storage-layer";

    // Provision the `arch-doc` container via the create-gate (the workflow
    // `allows-create: [{type: arch-doc}]`).
    let created = run_doc(
        repo.path(),
        home.path(),
        &["create", "arch-doc", "--title", "Storage layer"],
        None,
    );
    assert_ok(&created, "`jigc doc create arch-doc`");
    let arch = String::from_utf8(created.stdout)
        .expect("utf-8")
        .trim()
        .to_owned();
    assert_eq!(arch, format!("arch-doc:{slug}"), "minted arch-doc address");

    // Author the overview prose (the Simple section preceding the repeatable home).
    let overview = run_doc(
        repo.path(),
        home.path(),
        &["set-slot", &format!("{arch}#overview"), "--from-file", "-"],
        Some(b"The storage layer owns the on-disk task working areas.\n".as_slice()),
    );
    assert_ok(&overview, "`set-slot …#overview`");

    // Before the first add-item, `create` has rendered the full schema skeleton — an
    // EMPTY `## Components` home is provisioned (the empty-repeatable-home this test
    // retires), already in schema order after `## Overview`, but carrying NO item yet.
    let pre = staged_arch_doc(repo.path(), task, slug);
    let pre_overview = section_heading_offset(&pre, "Overview")
        .unwrap_or_else(|| panic!("`## Overview` present before add-item; staged:\n{pre}"));
    let pre_components = section_heading_offset(&pre, "Components")
        .unwrap_or_else(|| panic!("the empty `## Components` home is provisioned; staged:\n{pre}"));
    assert!(
        pre_overview < pre_components,
        "the empty `## Components` home is already in schema order after `## Overview` \
         (overview@{pre_overview} < components@{pre_components}); staged:\n{pre}"
    );
    assert!(
        !pre.contains("### "),
        "no `###` item heading exists before the first add-item; staged:\n{pre}"
    );

    // The FIRST add-item: emits the minted item address on stdout (the addr the agent
    // runs next — captured here, run verbatim below).
    let added = run_doc(
        repo.path(),
        home.path(),
        &[
            "add-item",
            &format!("{arch}#components"),
            "--title",
            "Working area store",
        ],
        None,
    );
    assert_ok(&added, "`jigc doc add-item …#components`");
    let addr = String::from_utf8(added.stdout)
        .expect("utf-8")
        .trim_end_matches('\n')
        .to_owned();

    // The emitted address is the canonical item address `arch-doc:<slug>#components/<id>`.
    let item_id = "working-area-store";
    assert_eq!(
        addr,
        format!("{arch}#components/{item_id}"),
        "the emitted item address is the canonical `#components/<id>` form (run verbatim)"
    );

    // The named acceptance check: the staged body now carries `## Overview` FOLLOWED
    // BY `## Components` — Components materialized at its schema-ordered position
    // (after Overview), neither preceding Overview nor landing at EOF ahead of it.
    let staged = staged_arch_doc(repo.path(), task, slug);
    let overview_at = section_heading_offset(&staged, "Overview")
        .unwrap_or_else(|| panic!("`## Overview` present after add-item; staged:\n{staged}"));
    let components_at = section_heading_offset(&staged, "Components").unwrap_or_else(|| {
        panic!("`## Components` materialized after add-item; staged:\n{staged}")
    });
    assert!(
        overview_at < components_at,
        "`## Components` must follow `## Overview` in schema order \
         (overview@{overview_at} < components@{components_at}); staged:\n{staged}"
    );

    // The minted `### <title>  {#<id>}` heading sits UNDER `## Components` (after the
    // components heading, before any following section / EOF — here EOF).
    let item_heading = format!("### Working area store  {{#{item_id}}}\n");
    let item_at = staged
        .find(&item_heading)
        .unwrap_or_else(|| panic!("the minted item heading is present; staged:\n{staged}"));
    assert!(
        item_at > components_at,
        "the minted item heading sits under `## Components`; staged:\n{staged}"
    );

    // Fill the item's slot + field over the EMITTED address verbatim (hardening #4 —
    // the emitted bytes are the contract, run as emitted, never reconstructed).
    let desc = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-slot",
            &format!("{addr}/description"),
            "--from-file",
            "-",
        ],
        Some(b"Reads and writes the staged docs under .jigc/tasks/<id>/.\n".as_slice()),
    );
    assert_ok(&desc, "`set-slot …#components/<id>/description`");
    let anchor = run_doc(
        repo.path(),
        home.path(),
        &[
            "set-field",
            &format!("{addr}/implemented-by"),
            "--value",
            "crates/engine/src/write.rs#add_item",
        ],
        None,
    );
    assert_ok(&anchor, "`set-field …#components/<id>/implemented-by`");

    // The full authoring chain — `set-slot …#overview` (a leading Simple slot) plus the
    // item-leaf `description` + `implemented-by` writes — round-trips byte-stable over
    // the **whole** staged doc: `render(parse(staged)) == staged`. (Previously scoped to
    // the `## Components` subtree to route around the M13-audit-HIGH leading-Simple-slot
    // `set_slot` defect — `set-slot …#overview` left `## Overview\n<prose>\n\n\n\n##`
    // `Components`, non-canonical; that defect is now fixed (`set_slot` re-renders the
    // section canonically, mirroring the item-slot fix in 3afc98a), so the assertion is
    // broadened to whole-doc to catch a regression.)
    let staged = staged_arch_doc(repo.path(), task, slug);
    let schema = arch_doc_schema();
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged arch-doc re-parses");

    // The authored leaves landed on THIS item (per-item disambiguation through the binary).
    let components = parsed
        .sections
        .iter()
        .find(|s| s.id == "components")
        .expect("components section present");
    assert_eq!(
        components.items.len(),
        1,
        "exactly the one minted component is present; staged:\n{staged}"
    );
    let item = &components.items[0];
    assert_eq!(item.id, item_id, "the item carries its minted anchor");
    assert_eq!(
        item.slot.as_deref().unwrap_or("").trim(),
        "Reads and writes the staged docs under .jigc/tasks/<id>/.",
        "the item's `description` slot is its authored prose"
    );
    let implemented = item
        .fields
        .iter()
        .find(|f| f.key == "implemented-by")
        .expect("the item carries implemented-by");
    match &implemented.value {
        engine::field_block::Value::Scalar(v) => assert_eq!(
            v, "crates/engine/src/write.rs#add_item",
            "the item's `implemented-by` is its authored anchor"
        ),
        other => panic!("implemented-by is a scalar anchor, got {other:?}"),
    }

    let rerendered = engine::write::render(&schema, &parsed);
    assert_eq!(
        rerendered, staged,
        "the full authoring chain (overview slot + item-leaf description + \
         implemented-by) is byte-stable across parse → render over the WHOLE doc",
    );
}
