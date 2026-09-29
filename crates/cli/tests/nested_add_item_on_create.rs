//! Regression (M22-completion finding 2): a **nested** repeatable-item field
//! declared `set: on-create` must be materialized when `jigc doc add-item` mints the
//! nested entry — symmetric with the top-level `add-item` on-create path
//! (`add_item_on_create.rs`).
//!
//! Before the fix the nested `add-item` branch passed no fields to the engine
//! (`add_nested_item(..., &[])`), so a nested `{ id: …, type: date, set: on-create }`
//! leaf was silently dropped — the schema's "CLI-set on create" promise was inert for
//! nested items, asymmetric with top-level items. This test drives the real binary
//! (`CARGO_BIN_EXE_jigc`) over a throwaway pack whose changelog nests a `changes`
//! repeatable carrying a `set: on-create` date, mints a nested change-group, and reads
//! back the staged bytes to assert the nested entry carries the populated date.
//!
//! Every assertion runs over the EMITTED bytes of the real binary, not a
//! reconstruction (increment-workflow hardening #4).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-nested-on-create-{tag}-{}-{:?}",
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
/// two-level-repeatable `changelog` schema whose **nested** `changes` block carries a
/// `{ id: at, type: date, set: on-create }` leaf (the nested on-create target), and a
/// `log-change` workflow whose create-gate admits it.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(cli::pack_path!(dev)).to_path_buf();
    copy_tree(&dev_pack, pack.path());
    // This fixture ships a deliberately divergent `changelog` shape, so it is NOT the
    // frozen dev pack — drop the copied freeze manifest (M33 pack-load gate would
    // otherwise block the un-bumped shape change). A manifest-less pack is unchecked.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");

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
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: at, type: date, set: on-create }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
",
    )
    .expect("write changelog schema");

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

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`.
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", pack)
        .output()
        .expect("run the jigc binary")
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
fn nested_add_item_materializes_an_on_create_date_on_the_change_group() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), pack.path(), &["setup"]),
        "jigc setup",
    );
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &["start", "--workflow", "log-change", "log the release"],
        ),
        "jigc start --workflow log-change",
    );
    let task = "log-the-release";

    let created = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &["doc", "create", "changelog", "--title", "Changelog"],
        ),
        "jigc doc create changelog",
    );
    let slug = created
        .strip_prefix("changelog:")
        .expect("created address is changelog:<slug>")
        .to_owned();

    // A top-level release, then a NESTED change-group into it. The nested mint is the
    // path under test — its `changes` block declares `{ id: at, set: on-create }`.
    let rel = ok_stdout(
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
        ),
        "add-item release 1-3-0",
    );
    let nested = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &[
                "doc",
                "add-item",
                &format!("{rel}/changes"),
                "--title",
                "Added",
            ],
        ),
        "add-item nested #added",
    );
    assert_eq!(
        nested,
        format!("changelog:{slug}#releases/1-3-0/changes/added")
    );

    // Re-read the staged doc and assert the minted nested change-group carries a
    // populated `at` date — the nested `set: on-create` promise. Before the fix the
    // nested branch passed no fields, so the `at:` bullet was absent (the red). The
    // re-read parses the EMITTED bytes against the schema, so the assertion is over the
    // real binary's output.
    let staged = staged_changelog(repo.path(), task, &slug);
    let schema = changelog_schema();
    let parsed =
        engine::write::instance_from_source(&schema, &staged).expect("staged changelog re-parses");
    let added = parsed
        .sections
        .iter()
        .find(|s| s.id == "releases")
        .and_then(|s| s.items.iter().find(|i| i.id == "1-3-0"))
        .and_then(|r| r.items.iter().find(|i| i.id == "added"))
        .expect("the nested #added change-group is present");
    let at = added
        .fields
        .iter()
        .find(|f| f.key == "at")
        .unwrap_or_else(|| {
            panic!("the minted nested #added must carry an `at` on-create date; staged:\n{staged}")
        });
    match &at.value {
        engine::field_block::Value::Scalar(v) => {
            let ymd: Vec<&str> = v.split('-').collect();
            assert!(
                ymd.len() == 3 && ymd[0].len() == 4 && ymd[0].chars().all(|c| c.is_ascii_digit()),
                "the nested on-create date is an ISO `YYYY-MM-DD`; got {v:?}",
            );
        }
        other => panic!("the nested on-create date is a scalar; got {other:?}"),
    }
}

/// The embedded dev pack tree on disk (selected via `JIGC_PACK_DIR` so the binary
/// composes the bytes it ships) — its changelog nests a `changes` repeatable keyed by
/// an `enum` id-from (`category`), the nested write-time reject target.
fn dev_pack() -> PathBuf {
    Path::new(cli::pack_path!(dev)).to_path_buf()
}

/// M24 inc-2 T1 — a **nested** `add-item` (`{release}/changes`) whose `--title`
/// re-slugs OUTSIDE the nested repeatable's `id-from` enum is rejected **at the write
/// verb** (non-zero exit), not deferred to finalize. Symmetric with the top-level
/// reject: `Improvements` ∉ {added, changed, …} blocks with the SHARED finalize code
/// `schema-conformance.field-value-conformant` naming the section-qualified slug-cased
/// id-from address, while a valid member `Added`→`added` passes. Driven over the real
/// binary against the shipped dev pack, so the block is the emitted contract.
#[test]
fn nested_foreign_category_blocks_at_the_add_item_verb() {
    let repo = TempDir::new("nested-block-repo");
    let home = TempDir::new("nested-block-home");
    let pack = dev_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), &pack, &["setup"]),
        "jigc setup",
    );
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["start", "--workflow", "record-change", "cut the release"],
        ),
        "jigc start --workflow record-change",
    );
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
        ),
        "jigc doc create changelog",
    );

    // A top-level release to nest the change-group under (its id is the slugged version
    // title — drive the EMITTED address verbatim).
    let release = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.2.0",
            ],
        ),
        "add-item release 1.2.0",
    );

    // The foreign nested category — blocks at the write verb.
    let blocked = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "add-item",
            &format!("{release}/changes"),
            "--title",
            "Improvements",
            "--format",
            "json",
        ],
    );
    assert!(
        !blocked.status.success(),
        "a foreign nested id-from enum category must block at the add-item verb; \
         stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let stderr = String::from_utf8_lossy(&blocked.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("stderr is JSON: {e}; got:\n{stderr}"));
    let finding = &report["findings"][0];
    assert_eq!(
        finding["code"], "schema-conformance.field-value-conformant",
        "the nested block carries the SHARED finalize code; got:\n{stderr}",
    );
    let address = finding["location"]["address"]
        .as_str()
        .unwrap_or_else(|| panic!("the finding is addressed; got:\n{stderr}"));
    // (M42 inc-9 T2) The section-qualified nested id-from address in **URI normal form** —
    // the emitted release address prefixes it whole (`design/command-output-contract.md` →
    // the `write.*` row: a bare fragment is not a stable key).
    assert_eq!(
        address,
        format!("{release}/changes/improvements/category"),
        "the finding names the doc-qualified, section-qualified slug-cased nested id-from address",
    );
    assert_eq!(
        finding["key"]["target"], address,
        "the stable key.target IS that address; got:\n{stderr}",
    );

    // A valid member passes — `Added` re-slugs to the `added` enum member (the
    // composing-context guard: the reject is not always-on).
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                &format!("{release}/changes"),
                "--title",
                "Added",
            ],
        ),
        "add-item nested #added (valid member)",
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
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: at, type: date, set: on-create }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
";
    engine::schema::load_schema(yaml).expect("changelog schema loads")
}
