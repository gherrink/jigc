//! M24 Increment 3, T2 — the `jigc doc remove-item <addr>` CLI verb through the
//! real binary.
//!
//! A mis-authored item — a top-level release **or** a nested change-group — can be
//! removed without discarding the task (a general recovery verb over the engine's
//! `remove_item` / `remove_nested_item`). Driving `CARGO_BIN_EXE_jigc` over a real
//! `git init` temp repo against the shipped-shape `changelog` doctype, this proves:
//!
//!   1. removing a **nested** change-group (`#releases/<v>/changes/<cat>`) round-trips
//!      byte-stable (`render(parse(staged)) == staged`) and removes exactly that group;
//!   2. removing a **top-level** release (`#releases/<v>`) round-trips byte-stable —
//!      **including the last/only release** (the case the engine last-block EOF fix
//!      unblocked);
//!   3. a **mis-named** target — both a nested wrong-parent and a top-level
//!      non-existent id — **blocks** with a routed finding and leaves the staged file
//!      **byte-unchanged** (no wrong-item write).
//!
//! Mirrors `nested_item_addressing.rs` / `changelog_cold_create.rs`: the embedded dev
//! pack tree copied to a temp dir plus a two-level-repeatable `changelog` schema and a
//! create-gated `log-change` workflow that admits it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-remove-item-{tag}-{}-{:?}",
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
/// two-level-repeatable `changelog` schema and a `log-change` workflow whose
/// create-gate admits it. Returns the pack dir.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
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

/// Assert `staged` is byte-stable across parse → render against the fixture schema.
fn assert_byte_stable(staged: &str) {
    let schema = changelog_schema();
    let parsed =
        engine::write::instance_from_source(&schema, staged).expect("staged changelog re-parses");
    assert_eq!(
        engine::write::render(&schema, &parsed),
        staged,
        "the staged changelog is byte-stable across parse → render",
    );
}

/// The held provisioning of a changelog with releases: the temp guards must stay
/// alive (dropping them removes the repo), so they are returned to the caller.
struct Fixture {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
    task: String,
    slug: String,
}

impl Fixture {
    fn run(&self, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
        run_jigc(
            self.repo.path(),
            self.home.path(),
            self.pack.path(),
            args,
            stdin,
        )
    }

    fn staged(&self) -> String {
        staged_changelog(self.repo.path(), &self.task, &self.slug)
    }
}

/// Provision a `changelog` task with the given releases (each carrying a nested
/// `Added` change-group with one notes bullet), returning the live fixture.
fn provision(releases: &[&str]) -> Fixture {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = fixture_pack();
    init_repo(repo.path());

    ok_stdout(
        run_jigc(repo.path(), home.path(), pack.path(), &["setup"], None),
        "jigc setup",
    );
    ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "jigc start --workflow log-change",
    );
    let task = "log-the-release".to_owned();

    let created = ok_stdout(
        run_jigc(
            repo.path(),
            home.path(),
            pack.path(),
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    let slug = created
        .strip_prefix("changelog:")
        .expect("created address is changelog:<slug>")
        .to_owned();

    for version in releases {
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
                    version,
                ],
                None,
            ),
            "add-item release",
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
                None,
            ),
            "add-item nested change-group",
        );
        ok_stdout(
            run_jigc(
                repo.path(),
                home.path(),
                pack.path(),
                &[
                    "doc",
                    "set-slot",
                    &format!("{nested}/notes"),
                    "--from-file",
                    "-",
                ],
                Some(format!("Change in {version}.\n").as_bytes()),
            ),
            "set-slot nested notes",
        );
    }

    Fixture {
        repo,
        home,
        pack,
        task,
        slug,
    }
}

/// Parse the staged changelog and return the release ids present, in order.
fn release_ids(staged: &str) -> Vec<String> {
    let schema = changelog_schema();
    let parsed =
        engine::write::instance_from_source(&schema, staged).expect("staged changelog re-parses");
    parsed
        .sections
        .iter()
        .find(|s| s.id == "releases")
        .map(|s| s.items.iter().map(|i| i.id.clone()).collect())
        .unwrap_or_default()
}

/// The nested change-group ids of a given release in the staged changelog.
fn change_group_ids(staged: &str, release: &str) -> Vec<String> {
    let schema = changelog_schema();
    let parsed =
        engine::write::instance_from_source(&schema, staged).expect("staged changelog re-parses");
    parsed
        .sections
        .iter()
        .find(|s| s.id == "releases")
        .and_then(|s| s.items.iter().find(|i| i.id == release))
        .map(|r| r.items.iter().map(|i| i.id.clone()).collect())
        .unwrap_or_default()
}

#[test]
fn remove_nested_change_group_round_trips_byte_stable() {
    let fx = provision(&["1-3-0"]);
    let slug = &fx.slug;

    // Add a second nested change-group so the release keeps one after the removal.
    let rel = format!("changelog:{slug}#releases/1-3-0");
    let nested_fixed = ok_stdout(
        fx.run(
            &[
                "doc",
                "add-item",
                &format!("{rel}/changes"),
                "--title",
                "Fixed",
            ],
            None,
        ),
        "add-item nested #fixed",
    );
    ok_stdout(
        fx.run(
            &[
                "doc",
                "set-slot",
                &format!("{nested_fixed}/notes"),
                "--from-file",
                "-",
            ],
            Some(b"A fix.\n"),
        ),
        "set-slot #fixed notes",
    );

    // Remove the `Added` nested change-group via its section-qualified address.
    let removed = fx.run(
        &[
            "doc",
            "remove-item",
            &format!("changelog:{slug}#releases/1-3-0/changes/added"),
        ],
        None,
    );
    ok_stdout(removed, "remove-item nested #added");

    let staged = fx.staged();
    // The `added` group is gone; the `fixed` group survives under the same release.
    assert_eq!(
        change_group_ids(&staged, "1-3-0"),
        vec!["fixed".to_string()],
        "only the addressed nested group is removed; staged:\n{staged}",
    );
    assert_eq!(
        release_ids(&staged),
        vec!["1-3-0".to_string()],
        "the parent release survives the nested removal; staged:\n{staged}",
    );
    assert!(
        !staged.contains("#### Added"),
        "the removed group's heading is gone; staged:\n{staged}",
    );
    assert_byte_stable(&staged);
}

#[test]
fn remove_top_level_release_round_trips_byte_stable_including_the_last() {
    let fx = provision(&["1-3-0", "1-2-0"]);
    let slug = &fx.slug;

    // Remove one of two releases — the survivor and its nested group stay intact.
    let removed = fx.run(
        &[
            "doc",
            "remove-item",
            &format!("changelog:{slug}#releases/1-2-0"),
        ],
        None,
    );
    ok_stdout(removed, "remove-item release 1-2-0");

    let staged = fx.staged();
    assert_eq!(
        release_ids(&staged),
        vec!["1-3-0".to_string()],
        "only the addressed release is removed; staged:\n{staged}",
    );
    assert_eq!(
        change_group_ids(&staged, "1-3-0"),
        vec!["added".to_string()],
        "the survivor's nested group is intact; staged:\n{staged}",
    );
    assert_byte_stable(&staged);

    // Remove the LAST / only remaining release — the engine last-block EOF edge.
    let removed_last = fx.run(
        &[
            "doc",
            "remove-item",
            &format!("changelog:{slug}#releases/1-3-0"),
        ],
        None,
    );
    ok_stdout(removed_last, "remove-item last release 1-3-0");

    let staged_empty = fx.staged();
    assert!(
        release_ids(&staged_empty).is_empty(),
        "removing the last release leaves no releases; staged:\n{staged_empty}",
    );
    assert_byte_stable(&staged_empty);
}

#[test]
fn remove_item_prints_a_success_confirmation() {
    let fx = provision(&["1-3-0"]);
    let slug = &fx.slug;
    let addr = format!("changelog:{slug}#releases/1-3-0");

    // Agent-text: a terse confirmation naming the removed item address, so the
    // outcome is visible without re-reading the working-area file.
    let stdout = ok_stdout(
        fx.run(&["doc", "remove-item", &addr], None),
        "remove-item confirmation",
    );
    assert!(
        stdout.contains(&addr),
        "remove-item confirms the removed address on stdout; got:\n{stdout}"
    );

    // JSON: a structured ack on stdout.
    let fx = provision(&["2-0-0"]);
    let slug = &fx.slug;
    let addr = format!("changelog:{slug}#releases/2-0-0");
    let stdout = ok_stdout(
        fx.run(&["doc", "remove-item", &addr, "--format", "json"], None),
        "remove-item json ack",
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("remove-item json ack parses");
    // The command-output contract (`design/command-output-contract.md` §2): `op` + the
    // address decomposed into `target{doctype, slug, section, item}` (a top-level item
    // reaches section+item, no leaf) + the `removed` effect key + `findings: []`.
    assert_eq!(ack["op"], "remove-item");
    assert_eq!(ack["target"]["doctype"], "changelog");
    assert_eq!(ack["target"]["slug"], slug.as_str());
    assert_eq!(ack["target"]["section"], "releases");
    assert_eq!(ack["target"]["item"], "2-0-0");
    assert!(
        ack["target"]["leaf"].is_null(),
        "a remove-item target reaches no leaf; got:\n{stdout}"
    );
    assert_eq!(ack["removed"], true);
    assert_eq!(ack["findings"], serde_json::json!([]));

    // A **nested** remove keys `item` on the leaf-most item id — the flat 5-key target
    // shape holds at nested depth (`releases/<v>/changes/<cat>` → `item: <cat>`), the
    // top section under `section` and the leaf-most change-group under `item`.
    let fx = provision(&["3-1-0"]);
    let slug = &fx.slug;
    let nested = format!("changelog:{slug}#releases/3-1-0/changes/added");
    let stdout = ok_stdout(
        fx.run(&["doc", "remove-item", &nested, "--format", "json"], None),
        "nested remove-item json ack",
    );
    let ack: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("nested remove-item json ack parses");
    assert_eq!(ack["op"], "remove-item");
    assert_eq!(ack["target"]["section"], "releases");
    assert_eq!(
        ack["target"]["item"], "added",
        "the nested target keys `item` on the leaf-most change-group id; got:\n{stdout}"
    );
    assert_eq!(ack["findings"], serde_json::json!([]));
}

#[test]
fn remove_mis_named_target_blocks_and_leaves_the_doc_byte_unchanged() {
    let fx = provision(&["1-3-0"]);
    let slug = &fx.slug;
    let before = fx.staged();

    // (a) nested wrong-parent: no release `9-9-9` exists, so its nested `added` names
    // nothing — must block, not misfire onto the real `1-3-0/changes/added`.
    let nested_mis = fx.run(
        &[
            "doc",
            "remove-item",
            &format!("changelog:{slug}#releases/9-9-9/changes/added"),
        ],
        None,
    );
    assert!(
        !nested_mis.status.success(),
        "a remove against a mis-named nested parent must block",
    );
    assert_eq!(
        fx.staged(),
        before,
        "the blocked nested remove left the doc byte-identical",
    );

    // (b) top-level non-existent: no release `9-9-9` to remove — must block.
    let top_mis = fx.run(
        &[
            "doc",
            "remove-item",
            &format!("changelog:{slug}#releases/9-9-9"),
        ],
        None,
    );
    assert!(
        !top_mis.status.success(),
        "a remove against a non-existent top-level id must block",
    );
    assert_eq!(
        fx.staged(),
        before,
        "the blocked top-level remove left the doc byte-identical",
    );
}
