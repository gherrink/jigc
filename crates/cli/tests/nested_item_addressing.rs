//! M22 Increment 1, T5 — nested repeatable-item addressing through the real binary.
//!
//! The done-criterion (c): drive `CARGO_BIN_EXE_jigc` over a real `git init` temp
//! repo against a two-level-repeatable doctype, **driving the emitted nested address
//! verbatim** (increment-workflow hardening #4 — the agent runs the CLI's emitted
//! bytes, never a reconstructed equivalent):
//!
//!   1. `doc add-item` a release (top-level), capturing the emitted address;
//!   2. `doc add-item` a nested change-group into that release (`#section/release/
//!      changes`), capturing the emitted nested address;
//!   3. `doc set-slot` + `doc set-field` on the emitted nested address each land on
//!      the nested item (re-read asserts the nested item's own bytes);
//!   4. a nested `set-slot` against an address whose **parent is mis-named** does NOT
//!      misfire onto a same-anchor sibling under a different parent — it fails.
//!
//! The doctype is a test-fixture `changelog` (a release → nested change-groups) shipped
//! in a throwaway pack: the embedded dev pack tree copied to a temp dir, plus the
//! changelog schema + a create-gated workflow that admits it. No external test crates —
//! the binary path comes from `CARGO_BIN_EXE_jigc`, the repo is a real `git init`, and a
//! self-cleaning `TempDir` keeps the test off the developer's tree.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-nested-addr-{tag}-{}-{:?}",
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

/// Recursively copy `from` into `to` (both directories).
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create dest dir");
    for entry in fs::read_dir(from).expect("read src dir") {
        let entry = entry.expect("dir entry");
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            copy_tree(&src, &dst);
        } else {
            fs::copy(&src, &dst).expect("copy file");
        }
    }
}

/// Build a throwaway pack: the embedded dev pack tree copied to a temp dir, plus a
/// two-level-repeatable `changelog` schema and a `log-change` workflow whose create
/// -gate admits it. Returns the pack dir.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
    copy_tree(&dev_pack, pack.path());
    // This fixture ships a deliberately divergent `changelog` shape, so it is NOT the
    // frozen dev pack — drop the copied freeze manifest (M33 pack-load gate would
    // otherwise block the un-bumped shape change). A manifest-less pack is unchecked.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");

    // The two-level-repeatable doctype: a release item carrying a `date` field then a
    // **nested** `changes` repeatable (change-groups, each a bare-prose `notes` slot
    // plus an optional `ticket` field). The `release → change-group` shape.
    fs::write(
        pack.path().join("schemas").join("changelog.yaml"),
        "\
type: changelog
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
              - { id: ticket, type: string }
",
    )
    .expect("write changelog schema");

    // A create-gated workflow that mints a task and admits the changelog (bound to
    // `task.changelog`), then finalizes — mirroring the shipped `plan` workflow shape.
    fs::write(
        pack.path().join("workflows").join("log-change.yaml"),
        "\
---
when: record a release's changes in the changelog
description: Author the changelog for a release.
usage: a release's changes need recording in the changelog.
creates-task: true
allows-create: [{type: changelog, as: changelog}]
---
{{ include: step:finalize }}
",
    )
    .expect("write log-change workflow");

    pack
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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`.
fn run_jigc(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: Option<&[u8]>,
) -> std::process::Output {
    use std::io::Write;
    use std::process::Stdio;
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying stderr.
fn ok_stdout(out: std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "`{what}` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8 stdout")
        .trim_end_matches('\n')
        .to_owned()
}

/// The staged `changelog:<slug>` instance in the task working area.
fn staged_changelog(repo: &Path, task: &str, slug: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("changelog:{slug}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

#[test]
fn nested_item_addressing_lands_on_the_addressed_nested_item_through_the_binary() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), pack.path(), &["setup"], None);
    ok_stdout(setup, "jigc setup");

    let start = run_jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["start", "--workflow", "log-change", "log the release"],
        None,
    );
    ok_stdout(start, "jigc start --workflow log-change");
    let task = "log-the-release";

    // Provision the changelog container via the create-gate.
    let created = run_jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &["doc", "create", "changelog", "--title", "Changelog"],
        None,
    );
    let created_addr = ok_stdout(created, "jigc doc create changelog");
    let slug = created_addr
        .strip_prefix("changelog:")
        .expect("created address is changelog:<slug>")
        .to_owned();

    // (1) add-item TWO releases (so the nested groups have two distinct parents that
    // will each carry a same-anchor `#added` change-group — the disambiguation case).
    let rel_a = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &[
                "doc",
                "add-item",
                &format!("changelog:{slug}#releases"),
                "--title",
                "1-3-0",
            ],
            None,
        ),
        "add-item release 1-3-0",
    );
    let rel_b = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &[
                "doc",
                "add-item",
                &format!("changelog:{slug}#releases"),
                "--title",
                "1-2-0",
            ],
            None,
        ),
        "add-item release 1-2-0",
    );
    assert_eq!(rel_a, format!("changelog:{slug}#releases/1-3-0"));
    assert_eq!(rel_b, format!("changelog:{slug}#releases/1-2-0"));

    // (2) add-item a nested change-group `#added` into EACH release. The add-item
    // *target* names the nested repeatable via `<release-addr>/changes`; the emitted
    // minted address is the canonical SECTION-QUALIFIED chain
    // `#releases/<release>/changes/added` (review finding S1 — carries the `changes`
    // nested-section segment, the exact form set-slot/set-field accept).
    let nested_a = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &[
                "doc",
                "add-item",
                &format!("{rel_a}/changes"),
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item nested #added into 1.3.0",
    );
    let nested_b = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &[
                "doc",
                "add-item",
                &format!("{rel_b}/changes"),
                "--title",
                "Added",
            ],
            None,
        ),
        "add-item nested #added into 1.2.0",
    );
    // Both nested groups carry the SAME `#added` anchor but under DIFFERENT parents —
    // the emitted addresses differ only by the parent release hop, and both are
    // section-qualified (carry the `changes` segment).
    assert_eq!(
        nested_a,
        format!("changelog:{slug}#releases/1-3-0/changes/added")
    );
    assert_eq!(
        nested_b,
        format!("changelog:{slug}#releases/1-2-0/changes/added")
    );
    assert_ne!(nested_a, nested_b);

    // (3) set-slot + set-field on 1.2.0's nested `#added`, addressing the EMITTED
    // nested address verbatim — these must land on 1.2.0's group, not 1.3.0's.
    let slot = run_jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &[
            "doc",
            "set-slot",
            &format!("{nested_b}/notes"),
            "--from-file",
            "-",
        ],
        Some(b"OAuth device-code flow.\n"),
    );
    ok_stdout(slot, "set-slot 1.2.0/added/notes");
    let field = run_jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &[
            "doc",
            "set-field",
            &format!("{nested_b}/ticket"),
            "--value",
            "JIRA-42",
        ],
        None,
    );
    ok_stdout(field, "set-field 1.2.0/added/ticket");

    // (3b) Regression (review finding): set the TOP-LEVEL `date` field on release 1.2.0
    // AFTER its nested `#added` (which now carries a `ticket` field block) — the broken
    // path the increment-1 e2e never exercised. With the bug the release's `date` was
    // appended INTO the nested group's field block, corrupting the doc; the fix lands it
    // in the release's own region.
    let rel_date = run_jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &[
            "doc",
            "set-field",
            &format!("{rel_b}/date"),
            "--value",
            "2026-06-14",
        ],
        None,
    );
    ok_stdout(rel_date, "set-field 1.2.0/date (top-level, after nesting)");

    // Re-read the staged doc and assert the nested write landed on 1.2.0's `#added`
    // (the addressed nested item), leaving 1.3.0's same-anchor `#added` untouched.
    let staged = staged_changelog(repo.path(), task, &slug);
    let schema = changelog_schema();
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged changelog re-parses");
    let releases = parsed
        .sections
        .iter()
        .find(|s| s.id == "releases")
        .expect("releases section present");
    let r120 = releases
        .items
        .iter()
        .find(|i| i.id == "1-2-0")
        .expect("release 1-2-0 present");
    let r130 = releases
        .items
        .iter()
        .find(|i| i.id == "1-3-0")
        .expect("release 1-3-0 present");
    let added_120 = r120
        .items
        .iter()
        .find(|i| i.id == "added")
        .expect("1.2.0's #added present");
    let added_130 = r130
        .items
        .iter()
        .find(|i| i.id == "added")
        .expect("1.3.0's #added present");

    // The top-level `date` landed on release 1.2.0 itself, NOT on its nested #added.
    assert!(
        r120.fields.iter().any(|f| f.key == "date"
            && matches!(&f.value, engine::field_block::Value::Scalar(v) if v == "2026-06-14")),
        "the top-level date landed on release 1.2.0 itself; staged:\n{staged}",
    );
    assert!(
        !added_120.fields.iter().any(|f| f.key == "date"),
        "the nested #added must NOT have absorbed the release's date; staged:\n{staged}",
    );
    // Byte placement: the release's `date` bullet precedes its nested `#### Added`.
    let date_at = staged
        .find("- date: 2026-06-14")
        .expect("date bullet present");
    let added_hdr_at = staged
        .match_indices("#### Added")
        .last()
        .map(|(i, _)| i)
        .expect("a nested #### Added heading present");
    assert!(
        date_at < added_hdr_at,
        "the release's date must sit in its own region, before the nested #### Added; \
         staged:\n{staged}",
    );

    // 1.2.0's #added carries the authored notes + the ticket.
    assert_eq!(
        added_120.slot.as_deref().unwrap_or("").trim(),
        "OAuth device-code flow.",
        "the nested set-slot landed on 1.2.0's #added; staged:\n{staged}",
    );
    assert!(
        added_120.fields.iter().any(|f| f.key == "ticket"
            && matches!(&f.value, engine::field_block::Value::Scalar(v) if v == "JIRA-42")),
        "the nested set-field landed on 1.2.0's #added; staged:\n{staged}",
    );
    // 1.3.0's same-anchor #added is byte-untouched: empty notes, no ticket.
    assert!(
        added_130.slot.as_deref().unwrap_or("").trim().is_empty(),
        "1.3.0's same-anchor #added must stay empty (not the wrong-item write); staged:\n{staged}",
    );
    assert!(
        !added_130.fields.iter().any(|f| f.key == "ticket"),
        "1.3.0's same-anchor #added must carry no ticket; staged:\n{staged}",
    );

    // The whole staged doc is byte-stable: render(parse(staged)) == staged.
    assert_eq!(
        engine::write::render(&schema, &parsed),
        staged,
        "the nested-authored changelog is byte-stable across parse → render",
    );

    // (4) a nested set against an address whose PARENT is mis-named must NOT misfire
    // onto the same-anchor #added under a different parent — it fails (no wrong-item
    // write). `#releases/9-9-9/changes/added/notes` (section-qualified) names no release
    // `9-9-9`.
    let mis = run_jigc(
        repo.path(),
        home.path(),
        pack.path(),
        &[
            "doc",
            "set-slot",
            &format!("changelog:{slug}#releases/9-9-9/changes/added/notes"),
            "--from-file",
            "-",
        ],
        Some(b"should not land anywhere\n"),
    );
    assert!(
        !mis.status.success(),
        "a set against a mis-named parent must fail, not misfire onto a sibling",
    );
    let staged_after = staged_changelog(repo.path(), task, &slug);
    assert_eq!(
        staged_after, staged,
        "the failed mis-named-parent write left the doc byte-identical (no wrong-item write)",
    );
    assert!(
        !staged_after.contains("should not land anywhere"),
        "the mis-named-parent prose must not have landed anywhere",
    );
}

/// The fixture changelog schema, loaded for the re-read assertions (mirrors the bytes
/// the fixture pack ships).
fn changelog_schema() -> engine::schema::Schema {
    let yaml = b"\
type: changelog
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: string }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
              - { id: ticket, type: string }
";
    engine::schema::load_schema(yaml).expect("changelog schema loads")
}
