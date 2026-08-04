//! M44 Increment 2, T1 — the `write.not-present` route enrichment through the real
//! binary (`design/validation.md` → the `write.*` route split; `design/surface-contract.md`
//! law 2: nothing hides — the route names the followable recovery).
//!
//! An agent that addresses a **not-yet-minted item id** on a repeatable doctype
//! previously got the generic shape-question route (`jigc doc schema <doctype>`), a dead
//! end: the schema names the *shape*, never the corpus's *real item ids*. This suite
//! drives `CARGO_BIN_EXE_jigc` over a real `git init` temp repo against a shipped-shape
//! `changelog` doctype and proves the enriched route:
//!
//!   (a) `set-slot <type>:<slug>#<section>/<nonexistent-item>/<leaf>` blocks with a
//!       `write.not-present` whose `route` (asserted via `--format json`) is
//!       `jigc doc show <type>:<slug>#<section> --task <resolved-id>`, and running that
//!       emitted `doc show` **verbatim** resolves and reveals the section's real item ids;
//!   (b) `remove-item <type>:<slug>#<section>/<nonexistent-item>` blocks with the same
//!       enriched, followable route;
//!   (c) a **nested / field-leaf** not-present strips to the **top showable section**
//!       (`#releases`) — the route target resolves and is never an unshowable field-leaf
//!       (the N2 pin);
//!   (d) `write.wrong-shape` (an `add-item` into a **non-repeatable** section) still
//!       routes the generic `jigc doc schema` read — the arm split left the genuine
//!       shape-questions untouched. (Since M47 Inc 6 T3 the route names the **real**
//!       doctype: `<doctype>` is derivable from the finding's own `key.target`, so P6
//!       route-followability substitutes it. Followability of that route is swept in
//!       `route_followability.rs`.)
//!
//! **M47 Increment 6, T1 — arm (d)'s subject moved, on a changed basis.** Arm (d)
//! originally pinned a `set-field` at a **non-existent item** as a shape question. That
//! was the wrong half of M44's split: a missing item **id** is not a shape question at
//! all — the schema names the declared shape, never the corpus's live item ids — so the
//! whole write-verb × miss-shape axis now emits `write.not-present` there
//! (`write_miss_shape_axis.rs`; `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 8). This
//! is recorded as a **basis-has-changed rebuttal, not an override**: M44's arm split was
//! right that a *genuine* shape question routes the schema, and wrong that a missing item
//! id is one. A genuine shape question — an `add-item` into a section the schema declares
//! as **non-repeatable** — takes arm (d)'s place, so the "the split left the genuine
//! shape-questions untouched" claim keeps a live witness rather than losing one.
//!
//! **M45 Increment 2, T5** joins the suite with the *dead-end* half of the same law:
//!
//!   (e) a write blocked because the **staged source** no longer parses
//!       (`write.non-reparseable`) names the escape hatch `jigc task discard <task-id>`,
//!       not only "re-run the same write" — re-running cannot recover a broken source,
//!       and the emitted command is followed here (the fence's own declared `<task-id>`
//!       substitution) and proven to actually retire the task.
//!
//! Mirrors `doc_remove_item.rs`: the embedded dev pack copied to a temp dir plus a
//! two-level-repeatable `changelog` schema (here carrying a top-level `summary` slot on
//! the release block, so a three-hop `#releases/<item>/summary` set-slot exists) and a
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
            "jigc-not-present-route-{tag}-{}-{:?}",
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

/// The fixture `changelog` schema bytes — a two-level repeatable carrying a top-level
/// `summary` slot on the release block (so a `#releases/<item>/summary` set-slot exists).
const CHANGELOG_SCHEMA: &str = "\
type: changelog
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: date, type: string }
        - { id: summary, slot: { hint: \"One-line release summary.\" } }
        - id: changes
          repeatable:
            id-from: category
            block:
              - { id: category, type: string }
              - { id: notes, slot: { hint: \"One bullet per change.\" } }
              - { id: ticket, type: string }
";

/// Build a throwaway pack: the embedded dev pack tree copied to a temp dir, plus the
/// fixture `changelog` schema and a `log-change` workflow whose create-gate admits it.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(env!("CARGO_MANIFEST_DIR")).join("pack");
    copy_tree(&dev_pack, pack.path());
    // The fixture ships a deliberately divergent `changelog` shape, so it is NOT the frozen
    // dev pack — drop the copied freeze manifest (the pack-load gate would otherwise block
    // the un-bumped shape change). A manifest-less pack is unchecked.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");

    fs::write(
        pack.path().join("schemas").join("changelog.yaml"),
        CHANGELOG_SCHEMA,
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

/// The held provisioning of a changelog with releases: the temp guards must stay alive
/// (dropping them removes the repo), so they are returned to the caller.
struct Fixture {
    repo: TempDir,
    home: TempDir,
    pack: TempDir,
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
}

/// Provision a `changelog` task with the given releases (each carrying a nested `Added`
/// change-group with one notes bullet), returning the live fixture.
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
        ok_stdout(
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
    }

    Fixture {
        repo,
        home,
        pack,
        slug,
    }
}

/// The `route` string of the first finding in a blocking `--format json` stderr envelope.
fn json_route(out: &std::process::Output, what: &str) -> String {
    assert!(
        !out.status.success(),
        "`{what}` must block (non-zero exit); stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let report: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("`{what}` stderr is JSON: {e}; got:\n{stderr}"));
    assert_eq!(
        report["findings"][0]["code"], "write.not-present",
        "`{what}` blocks a not-present write; got:\n{stderr}",
    );
    report["findings"][0]["route"]
        .as_str()
        .unwrap_or_else(|| panic!("`{what}` route is a string; got:\n{stderr}"))
        .to_owned()
}

/// Extract the leading backticked command from a route string (`` `<cmd>`<tail> ``).
fn backticked(route: &str) -> &str {
    route
        .strip_prefix('`')
        .and_then(|rest| rest.split('`').next())
        .unwrap_or_else(|| panic!("route carries a backticked command; got: {route}"))
}

impl Fixture {
    /// Run the backticked command of a route verbatim (splitting on whitespace, dropping
    /// the leading `jigc`), returning its trimmed stdout — the "run the emitted route"
    /// followability proof.
    fn run_route(&self, route: &str, what: &str) -> String {
        let cmd = backticked(route);
        let mut parts = cmd.split_whitespace();
        assert_eq!(parts.next(), Some("jigc"), "the route leads with `jigc`");
        let args: Vec<&str> = parts.collect();
        ok_stdout(self.run(&args, None), what)
    }
}

#[test]
fn set_slot_at_a_nonexistent_item_routes_to_the_containing_section() {
    let fx = provision(&["1-3-0", "1-2-0"]);
    let slug = &fx.slug;

    // (a) A top-level set-slot at a not-yet-minted release id (`9-9-9`) — the leaf
    // `summary` is a real slot, so the only defect is the absent item.
    let out = fx.run(
        &[
            "doc",
            "set-slot",
            &format!("changelog:{slug}#releases/9-9-9/summary"),
            "--from-file",
            "-",
            "--format",
            "json",
        ],
        Some(b"A summary.\n"),
    );
    let route = json_route(&out, "set-slot at a nonexistent item");
    let expected = format!("jigc doc show changelog:{slug}#releases --task log-the-release");
    assert_eq!(
        backticked(&route),
        expected,
        "the not-present route names the followable containing section; route:\n{route}",
    );

    // Running that emitted `doc show` verbatim resolves and reveals the section's real
    // item ids — the dead end is closed with a *followable* recovery.
    let shown = fx.run_route(&route, "the emitted doc show");
    assert!(
        shown.contains("1-3-0") && shown.contains("1-2-0"),
        "the emitted route reveals the section's real item ids; got:\n{shown}",
    );
}

#[test]
fn remove_item_at_a_nonexistent_item_routes_to_the_containing_section() {
    let fx = provision(&["1-3-0"]);
    let slug = &fx.slug;

    // (b) remove-item at a non-existent top-level release id.
    let out = fx.run(
        &[
            "doc",
            "remove-item",
            &format!("changelog:{slug}#releases/9-9-9"),
            "--format",
            "json",
        ],
        None,
    );
    let route = json_route(&out, "remove-item at a nonexistent item");
    let expected = format!("jigc doc show changelog:{slug}#releases --task log-the-release");
    assert_eq!(
        backticked(&route),
        expected,
        "the not-present route names the followable containing section; route:\n{route}",
    );

    let shown = fx.run_route(&route, "the emitted doc show");
    assert!(
        shown.contains("1-3-0"),
        "the emitted route reveals the section's real item ids; got:\n{shown}",
    );
}

#[test]
fn a_nested_not_present_strips_to_the_top_showable_section() {
    let fx = provision(&["1-3-0"]);
    let slug = &fx.slug;

    // (c) A nested set-slot at a non-existent change-group under a real release — the
    // deep address `#releases/1-3-0/changes/nonexistent/notes` strips to the **top**
    // showable section (`#releases`), never the unshowable field-leaf.
    let out = fx.run(
        &[
            "doc",
            "set-slot",
            &format!("changelog:{slug}#releases/1-3-0/changes/nonexistent/notes"),
            "--from-file",
            "-",
            "--format",
            "json",
        ],
        Some(b"A note.\n"),
    );
    let route = json_route(&out, "nested set-slot at a nonexistent group");
    let expected = format!("jigc doc show changelog:{slug}#releases --task log-the-release");
    assert_eq!(
        backticked(&route),
        expected,
        "the nested not-present strips to the top showable section; route:\n{route}",
    );

    // The route target resolves through `doc show` (never a field-leaf that could not be
    // shown) — the followability proof at nested depth.
    let shown = fx.run_route(&route, "the emitted doc show");
    assert!(
        shown.contains("1-3-0"),
        "the stripped route resolves and reveals the release ids; got:\n{shown}",
    );
}

#[test]
fn a_genuine_shape_question_still_routes_to_doc_schema() {
    let fx = provision(&["1-3-0"]);

    // (d) An `add-item` into a section the schema declares **non-repeatable** is a genuine
    // shape question (`write.wrong-shape`) — the schema *can* answer it — so it still
    // routes the generic `jigc doc schema`. (The task's own transient commit doc carries
    // the non-repeatable `summary` section; the same cell is swept as data in
    // `write_miss_shape_axis.rs`.)
    let out = fx.run(
        &[
            "doc",
            "add-item",
            "commit:log-the-release#summary",
            "--title",
            "Nope",
            "--format",
            "json",
        ],
        None,
    );
    assert!(
        !out.status.success(),
        "an add-item into a non-repeatable section must block; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let report: serde_json::Value =
        serde_json::from_str(stderr.trim()).unwrap_or_else(|e| panic!("stderr is JSON: {e}"));
    assert_eq!(
        report["findings"][0]["code"], "write.wrong-shape",
        "an add-item into a non-repeatable section is a shape question; got:\n{stderr}",
    );
    let route = report["findings"][0]["route"]
        .as_str()
        .expect("wrong-shape carries a route");
    // M47 Inc 6 T3 — the pin moves with the fix that repairs it: the shape question still
    // routes `jigc doc schema`, but the `<doctype>` placeholder no longer reaches the agent.
    // It is derivable from the finding's own `key.target` (`commit:…#summary` → `commit`),
    // so P6 route-followability requires it substituted (`design/surface-contract.md` → The
    // route fence). The claim this arm carries is unchanged; only the emitted bytes are.
    assert!(
        route.contains("jigc doc schema commit"),
        "a genuine shape question still routes `jigc doc schema`, naming the real doctype; \
         route:\n{route}",
    );
}

/// (e) M45 Inc 2, T5 — **the dead end names its exit.** A write whose *staged source* no
/// longer parses blocks with `write.non-reparseable`, and its route previously offered
/// only "revise the payload … then re-run the same write" — false when the source, not
/// the payload, is the broken thing (the rc.8 trial's #1-ranked cost: the reporter had to
/// discover `jigc task discard` themselves). The route now names that escape hatch.
///
/// The staged copy is broken **out of band** (the real shape: a doc jigc's own write path
/// cannot reach a valid state from), then the emitted command is *followed* — the route's
/// `jigc task discard <task-id>` span with the fence's own declared substitution applied
/// (`route_fence.rs` → `DUMMY_SUBSTITUTIONS`: a mechanical route's placeholder is filled
/// by the agent, kept verbatim in the text) — and proven to actually retire the task.
#[test]
fn a_broken_staged_source_names_the_discard_escape_hatch() {
    let fx = provision(&["1-3-0"]);
    let slug = &fx.slug;

    // Break the staged changelog out of band: the section heading no longer matches the
    // schema's declared `releases` id, so the source parses against nothing.
    let staged = fx
        .repo
        .path()
        .join(".jigc")
        .join("tasks")
        .join("log-the-release")
        .join("docs")
        .join(format!("changelog:{slug}.md"));
    let source = fs::read_to_string(&staged).expect("read the staged changelog");
    assert!(
        source.contains("## Releases"),
        "the staged changelog carries the canonical section heading; got:\n{source}",
    );
    fs::write(&staged, source.replace("## Releases", "## Releasez")).expect("break the source");

    let out = fx.run(
        &[
            "doc",
            "set-slot",
            &format!("changelog:{slug}#releases/1-3-0/summary"),
            "--from-file",
            "-",
            "--format",
            "json",
        ],
        Some(b"A summary.\n"),
    );
    assert!(
        !out.status.success(),
        "a write over a non-parsing staged source must block; stdout:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    let report: serde_json::Value =
        serde_json::from_str(stderr.trim()).unwrap_or_else(|e| panic!("stderr is JSON: {e}"));
    assert_eq!(
        report["findings"][0]["code"], "write.non-reparseable",
        "the broken staged source blocks the write; got:\n{stderr}",
    );
    let route = report["findings"][0]["route"]
        .as_str()
        .expect("the blocking finding carries a route")
        .to_owned();

    // The escape hatch is named, beyond the re-run direction that cannot work here.
    let discard = route
        .split('`')
        .find(|span| span.starts_with("jigc task discard"))
        .unwrap_or_else(|| panic!("the route names a `jigc task discard` command; route:\n{route}"))
        .to_owned();
    assert_eq!(
        discard, "jigc task discard <task-id>",
        "the route's escape hatch is the shipped discard verb with the declared \
         placeholder; route:\n{route}",
    );

    // Follow it: the emitted argv, `<task-id>` filled with this task's id, retires the task.
    let args: Vec<String> = discard
        .split_whitespace()
        .skip(1)
        .map(|arg| arg.replace("<task-id>", "log-the-release"))
        .collect();
    let followed = fx.run(
        &args.iter().map(String::as_str).collect::<Vec<&str>>(),
        None,
    );
    ok_stdout(followed, "the emitted `jigc task discard`");
    assert!(
        !fx.repo
            .path()
            .join(".jigc")
            .join("tasks")
            .join("log-the-release")
            .exists(),
        "the followed route actually retired the task's working area",
    );
}
