//! M39 completion fix — `jigc doc show <ref>#section/<item>/<leaf>` resolves a
//! repeatable item's **field** and **`id-from`** leaves, closing the pinned
//! `doc-read-surface.md` contract's over-claim (the contract table advertises
//! `#section/<item>/<leaf>` as "a field's value", but the milestone-record's field
//! leaves — `task-id` / `intent` / `status` — used to block `store.no-such-leaf`).
//!
//! The **milestone-record is the doc-show contract's own witness doctype** (its per-item
//! leaves are all machine-maintained fields + the `id-from` heading), so it is the
//! faithful reproduction. Driven through the REAL `jigc` binary under the
//! `[dev ▸ methodology]` composition, on the EMITTED bytes (plain + `--format json`).
//!
//! Pre-fix, every leaf slice returned the routed `store.no-such-leaf` block; post-fix the
//! field/id-from leaves resolve to their values, while a genuinely-absent leaf name STILL
//! blocks (the block stays reachable).
//!
//! **The multi-slot arm (M47 Inc 6, T5).** The write side's undeclared-leaf class
//! (`undeclared_address_writes.rs`) has a read-side face: `ParsedItem::slot_span` falls
//! back to the item's bare prose body whenever the item carries no sub-labelled `slots`,
//! so a leaf-resolving read that consulted it *first* would hand back a neighbouring
//! slot's prose at exit 0 for a leaf the schema never declared. It does not — both
//! duplicated leaf resolvers (`engine::store::resolve_leaf` and its CLI twin `leaf_json`,
//! the plain and `--format json` surfaces) gate the span on the **template's** declaration
//! first. That was never covered on a **multi-slot** item, because no dev-pack repeatable
//! declares two slots (`baseline.md` §3a — N4's multi-slot arm, `UNVERIFIED`); the arm
//! below closes it against a synthetic multi-slot fixture pack rather than re-declaring
//! the bound, in **both** item states — mint-empty and filled.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-doc-show-item-leaf-{tag}-{}-{:?}",
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

/// Run a `git` command in `repo`, asserting success.
fn git(repo: &Path, args: &[&str]) {
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
}

/// Initialize a real git repo with one commit and the `[dev ▸ methodology]` compose
/// marker (so the composed cascade resolves the `milestone-record` schema).
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
    fs::write(
        repo.join(".jigc").join("config").join("packs.yaml"),
        "compose-embedded-methodology: true\n",
    )
    .expect("write compose marker");
}

/// Run `jigc <args>` with `cwd = repo` and `$HOME = home`, never inheriting a harness
/// `JIGC_PACK_DIR` (the compose-marker path requires it absent).
fn run_jigc(repo: &Path, home: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR")
        .output()
        .expect("run the jigc binary")
}

/// Assert an invocation exited 0, surfacing stderr on failure, returning trimmed stdout.
fn ok_stdout(out: &std::process::Output, what: &str) -> String {
    assert!(
        out.status.success(),
        "{what} must exit 0; got {:?}\nstderr:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout.clone())
        .expect("utf-8 stdout")
        .trim_end()
        .to_string()
}

/// Recursively copy `from` into `to` (both directories) — the fixture pack is built by
/// copying the embedded dev pack tree and overwriting one schema.
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

/// The **multi-slot** fixture: a repeatable whose item block declares **two** slots, so
/// `slot_span`'s single-slot (bare-prose) fallback is not what resolves a leaf here and
/// the two slots are distinguishable prose. No dev-pack repeatable declares two, which is
/// why this arm needs a fixture at all.
const MULTI_SLOT_SCHEMA: &str = "\
type: changelog
id-from: title
sections:
  - id: releases
    repeatable:
      id-from: version
      block:
        - { id: version, type: string }
        - { id: summary, slot: { hint: \"One-line release summary.\" } }
        - { id: caveats, slot: { hint: \"Anything to watch out for.\" } }
";

/// Build a throwaway pack: the embedded dev pack tree plus the multi-slot `changelog`
/// fixture schema and a workflow whose create-gate admits it.
fn fixture_pack() -> TempDir {
    let pack = TempDir::new("pack");
    let dev_pack = Path::new(cli::pack_path!(dev)).to_path_buf();
    copy_tree(&dev_pack, pack.path());
    // The fixture ships a deliberately divergent `changelog` shape, so it is NOT the
    // frozen dev pack — drop the copied freeze manifest, which would otherwise block the
    // un-bumped shape change at pack-load.
    fs::remove_file(pack.path().join("config").join("schema-manifest.yaml"))
        .expect("drop the copied freeze manifest");
    fs::write(
        pack.path().join("schemas").join("changelog.yaml"),
        MULTI_SLOT_SCHEMA,
    )
    .expect("write the multi-slot changelog schema");
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
    .expect("write the log-change workflow");
    pack
}

/// Initialize a git repo + the `.jigc/config/` project layer, without the compose marker
/// (the fixture-pack arm resolves its schema through `JIGC_PACK_DIR`).
fn init_plain_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "README.md"]);
    git(repo, &["commit", "-q", "-m", "initial"]);
    fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
}

/// Run `jigc <args>` against the fixture pack, handing `stdin` when given.
fn run_fixture(
    repo: &Path,
    home: &Path,
    pack: &Path,
    args: &[&str],
    stdin: Option<&str>,
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
    if let Some(text) = stdin {
        child
            .stdin
            .take()
            .expect("stdin piped")
            .write_all(text.as_bytes())
            .expect("write stdin");
    }
    child.wait_with_output().expect("wait for jigc")
}

/// A canonical committed ADR (the shipped `write::render` shape): its `status` header
/// section carries the `status` enum + `date` field leaves — the archetypal
/// `#<section>/<field>` target `jigc validate` itself emits.
const COMMITTED_ADR: &str = "---\nstatus: accepted\ndate: 2026-05-23\n---\n\n# Single-node cache\n\n## Context\n\nForces.\n\n## Options\n\nAlternatives were weighed and rejected.\n\n## Decision\n\nA single in-memory node.\n\n## Consequences\n\nNone.\n";

/// (M42 inc-8 T1) The `#<section>/<leaf>` read resolves on a **non-repeatable** section
/// — through the REAL binary, on the emitted bytes, plain **and** `--format json`.
///
/// This is the round-trip the tool broke: `jigc validate` emits `#<section>/<field>`
/// targets (`schema-conformance.required-field-present` / `field-value-conformant`) and
/// `jigc doc set-field` accepts the same string at exit 0, while `jigc doc show` answered
/// `blocking · store.no-such-item`. The engine branch and the json projection are one
/// atomic unit: the engine alone would turn today's honest block into a json
/// wrong-node-exit-0 (the empty slot string), the exact class this wave closes.
#[test]
fn section_leaf_slice_resolves_a_field_in_a_simple_section() {
    let repo = TempDir::new("adr-leaf");
    let home = TempDir::new("adr-leaf-home");
    init_repo(repo.path());
    let adr = repo.path().join("docs").join("decisions");
    fs::create_dir_all(&adr).expect("mk docs/decisions/");
    fs::write(adr.join("single-node-cache.md"), COMMITTED_ADR).expect("write committed adr");
    git(repo.path(), &["add", "docs"]);
    git(repo.path(), &["commit", "-q", "-m", "adr"]);

    // (1) Plain: the leaf's canonical rendered value (an enum leaf reads lowercase).
    for (leaf, want) in [("status", "accepted"), ("date", "2026-05-23")] {
        let addr = format!("adr:single-node-cache#status/{leaf}");
        let plain = ok_stdout(
            &run_jigc(repo.path(), home.path(), &["doc", "show", &addr]),
            &format!("plain `{addr}`"),
        );
        assert_eq!(plain, want, "plain `{addr}` is the field's value");

        // (2) `--format json`: the same value, shaped exactly as in `fields` (a string).
        let json = ok_stdout(
            &run_jigc(
                repo.path(),
                home.path(),
                &["doc", "show", &addr, "--format", "json"],
            ),
            &format!("json `{addr}`"),
        );
        assert_eq!(
            json,
            format!("\"{want}\""),
            "json `{addr}` is the leaf value, never the empty slot string",
        );
    }

    // (3) An absent leaf name blocks honestly — `store.no-such-leaf` + a route, never a
    //     wrong node at exit 0.
    let bad = run_jigc(
        repo.path(),
        home.path(),
        &["doc", "show", "adr:single-node-cache#status/nope"],
    );
    assert!(
        !bad.status.success(),
        "an absent leaf must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&bad.stdout),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("store.no-such-leaf")
            && stderr.contains("nope")
            && stderr.contains("route:"),
        "the block names the absent leaf + carries a route; got:\n{stderr}",
    );
}

#[test]
fn item_leaf_slices_resolve_field_and_id_from_leaves() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Build a committed milestone-record with one sub-task item (fields: task-id/intent/status).
    ok_stdout(
        &run_jigc(
            repo.path(),
            home.path(),
            &["milestone", "create", "Cache rework"],
        ),
        "`jigc milestone create`",
    );
    ok_stdout(
        &run_jigc(
            repo.path(),
            home.path(),
            &[
                "milestone",
                "add-task",
                "cache-rework",
                "Warm the read cache",
            ],
        ),
        "`jigc milestone add-task`",
    );

    let base = "milestone-record:cache-rework#tasks/warm-the-read-cache";

    // (1) The `id-from` leaf → the item's heading; each field leaf → its rendered value.
    //     Plain path (the byte-exact/canonical value).
    for (leaf, want) in [
        ("task-id", "warm-the-read-cache"),
        ("intent", "Warm the read cache"),
        ("status", "active"),
    ] {
        let plain = ok_stdout(
            &run_jigc(
                repo.path(),
                home.path(),
                &["doc", "show", &format!("{base}/{leaf}")],
            ),
            &format!("plain `{base}/{leaf}`"),
        );
        assert_eq!(plain, want, "plain leaf slice of `{leaf}` is its value");

        // (2) `--format json` → the same value as a json string (the pinned leaf shape).
        let json = ok_stdout(
            &run_jigc(
                repo.path(),
                home.path(),
                &["doc", "show", &format!("{base}/{leaf}"), "--format", "json"],
            ),
            &format!("json `{base}/{leaf}`"),
        );
        assert_eq!(
            json,
            format!("\"{want}\""),
            "json leaf slice of `{leaf}` is the value as a json string",
        );
    }

    // (3) GUARD: a genuinely-absent leaf name STILL blocks (the block stays reachable) —
    //     the fix must not make `store.no-such-leaf` unreachable.
    let bad = run_jigc(
        repo.path(),
        home.path(),
        &["doc", "show", &format!("{base}/not-a-leaf")],
    );
    assert!(
        !bad.status.success(),
        "an absent leaf must exit non-zero; stdout:\n{}",
        String::from_utf8_lossy(&bad.stdout),
    );
    let stderr = String::from_utf8_lossy(&bad.stderr);
    assert!(
        stderr.contains("store.no-such-leaf") && stderr.contains("not-a-leaf"),
        "the block names the absent leaf + its route; got:\n{stderr}",
    );
}

/// The multi-slot item's address for both states below.
const MULTI_SLOT_ITEM: &str = "changelog:changelog#releases/1-0-0";
/// The task the fixture arm writes and reads through.
const MULTI_SLOT_TASK: &str = "log-the-release";

/// (M47 Inc 6, T5) On a **multi-slot** repeatable item, an **undeclared** leaf blocks
/// `store.no-such-leaf` on **both** read surfaces — plain and `--format json`, the two
/// duplicated leaf resolvers — never a neighbouring slot's prose at exit 0.
///
/// Driven in **both** item states, because they reach `slot_span` differently: the
/// **mint-empty** item (whose sub-labels exist but carry no prose — the state closest to
/// the single-slot bare-prose fallback the write side had to guard against) and the
/// **filled** item, where each declared leaf must resolve to *its own* prose, so the block
/// on the undeclared leaf cannot be a resolver that simply fails everywhere.
///
/// This arm closes `baseline.md` §3a's `UNVERIFIED` multi-slot bound with a synthetic
/// fixture rather than re-declaring it; it found the read side **already correct** (the
/// `declares_slot` gate both resolvers apply since M40) and pins it so.
#[test]
fn multi_slot_item_undeclared_leaf_blocks_on_both_read_surfaces() {
    let repo = TempDir::new("multi-slot");
    let home = TempDir::new("multi-slot-home");
    let pack = fixture_pack();
    init_plain_repo(repo.path());
    let jigc = |args: &[&str], stdin: Option<&str>| {
        run_fixture(repo.path(), home.path(), pack.path(), args, stdin)
    };

    ok_stdout(&jigc(&["setup"], None), "`jigc setup`");
    ok_stdout(
        &jigc(
            &["start", "--workflow", "log-change", "log the release"],
            None,
        ),
        "`jigc start --workflow log-change`",
    );
    ok_stdout(
        &jigc(
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "`jigc doc create changelog`",
    );
    ok_stdout(
        &jigc(
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1-0-0",
            ],
            None,
        ),
        "`jigc doc add-item`",
    );

    // The undeclared leaf, adjudicated on both surfaces.
    let assert_blocks = |state: &str| {
        let addr = format!("{MULTI_SLOT_ITEM}/bogus");
        for format in [&[][..], &["--format", "json"][..]] {
            let mut args = vec!["doc", "show", &addr, "--task", MULTI_SLOT_TASK];
            args.extend_from_slice(format);
            let out = jigc(&args, None);
            assert!(
                !out.status.success(),
                "`{addr}` ({state}, {format:?}) must exit non-zero — an undeclared leaf is \
                 never a neighbouring slot's prose; stdout:\n{}",
                String::from_utf8_lossy(&out.stdout),
            );
            let stderr = String::from_utf8_lossy(&out.stderr);
            assert!(
                stderr.contains("store.no-such-leaf") && stderr.contains("bogus"),
                "`{addr}` ({state}, {format:?}) blocks `store.no-such-leaf` naming the leaf; \
                 got:\n{stderr}",
            );
        }
    };

    // (1) Mint-empty: the declared leaves resolve (to their empty prose), the undeclared
    //     one blocks. This is the state where a leaf-blind resolver would hand back the
    //     item's whole body.
    for leaf in ["summary", "caveats"] {
        let plain = ok_stdout(
            &jigc(
                &[
                    "doc",
                    "show",
                    &format!("{MULTI_SLOT_ITEM}/{leaf}"),
                    "--task",
                    MULTI_SLOT_TASK,
                ],
                None,
            ),
            &format!("mint-empty `{leaf}`"),
        );
        assert_eq!(plain, "", "a mint-empty declared slot reads back empty");
    }
    assert_blocks("mint-empty");

    // (2) Filled: each declared leaf carries its OWN prose — so the undeclared leaf's
    //     block below is a resolver that discriminates, not one that fails blindly.
    for (leaf, prose) in [
        ("summary", "The summary prose.\n"),
        ("caveats", "The caveats prose.\n"),
    ] {
        ok_stdout(
            &jigc(
                &[
                    "doc",
                    "set-slot",
                    &format!("{MULTI_SLOT_ITEM}/{leaf}"),
                    "--from-file",
                    "-",
                    "--task",
                    MULTI_SLOT_TASK,
                ],
                Some(prose),
            ),
            &format!("`jigc doc set-slot` on `{leaf}`"),
        );
    }
    for (leaf, want) in [
        ("summary", "The summary prose."),
        ("caveats", "The caveats prose."),
    ] {
        let addr = format!("{MULTI_SLOT_ITEM}/{leaf}");
        let plain = ok_stdout(
            &jigc(&["doc", "show", &addr, "--task", MULTI_SLOT_TASK], None),
            &format!("plain `{addr}`"),
        );
        assert_eq!(plain, want, "plain `{addr}` is that slot's own prose");
        let json = ok_stdout(
            &jigc(
                &[
                    "doc",
                    "show",
                    &addr,
                    "--task",
                    MULTI_SLOT_TASK,
                    "--format",
                    "json",
                ],
                None,
            ),
            &format!("json `{addr}`"),
        );
        assert_eq!(
            json,
            format!("\"{want}\""),
            "json `{addr}` is that slot's own prose, never its neighbour's",
        );
    }
    assert_blocks("filled");
}
