//! M22 Increment 5, T3 — Flow 24 end-to-end acceptance: the `changelog` doctype's
//! cold-create → warm-append byte-stable proof, the two reds, and the `single-task`
//! fold-in bar, all driven through the **rebuilt** `jigc` binary over a `git init`
//! temp repo against the **shipped** dev pack (selected via `JIGC_PACK_DIR` = the
//! embedded `pack/` tree, so it is the bytes that ship, not a fixture).
//!
//! The multi-level authoring path is net-new engine, so the verb sequence was spiked
//! against the rebuilt binary at build (M16 exercise-don't-infer). The spike confirmed
//! a load-bearing fact the design's illustrative notation glossed: the version-title
//! slugger DROPPED dots, so `add-item --title "1.0.0"` minted id `100` — colliding with
//! a release literally titled `100`. **That is the M42 fork**: since slug-rule-version 2 a
//! dot is a SEPARATOR, so the id is `1-0-0` (`design/storage.md` → Identity → The slug
//! rule is itself a versioned rule; the shipped rule is generation 3, which forked the
//! edge-stopword drop and left the separator map alone). Every downstream address here is still driven from
//! the EMITTED `add-item` address verbatim — never a reconstructed form — so the test
//! asserts the bytes an agent would actually run (the masking-test guard), and the fork
//! showed up here as the emitted address moving, not as a test rewritten to agree.
//!
//! The emitted nested-item address is the canonical **section-qualified** form (review
//! finding S1, `design/changelog.md` → engine work #1): `#releases/<id>/changes/<cat>`,
//! carrying the `changes` nested-section segment. `set-slot`/`set-field` accept that
//! exact form and the validate conformance gate names it, so the test drives the
//! section-qualified address an agent really types — not the segment-less form a prior
//! divergence emitted (the M22-audit un-masking).
//!
//! `worked-examples.md` flow 24 acceptance bar:
//!   1. a new project authors a managed `changelog` through jigc, promoted byte-stable;
//!   2. the nested `Leaf::Repeatable` (release → change-group) round-trips;
//!   3. maintained over two runs — warm-re-create (copy-in preserves the prior release)
//!      then append a second release, re-promoted byte-stable with BOTH present;
//!   4. the conformance gate fires on a real empty NESTED leaf (red 1), and the
//!      absent OPTIONAL `link` finalizes clean while a required slot still blocks (red 2);
//!   5. the `single-task` fold-in appends an unreleased entry through the create-gate;
//!   6. the honest Flow-A-only bound — a foreign root `CHANGELOG.md` routes
//!      `needs-reconcile` (auto-migration G1 is the separate later milestone).
//!
//! M40 A2 extends the acceptance with the nested **round-trip** arms
//! (`design/doc-read-surface.md` → Nested repeatables join the pin): after finalize,
//! every address the suite's `add-item` calls emitted reads back **verbatim** through
//! the task-less `jigc doc show` in BOTH formats (plain and `--format json`) at every
//! depth — section, nested array, nested item, nested leaf — and a wrong nested
//! address (a bad nested-section segment, the segment-less physical shortcut, an
//! absent group id) exits non-zero with the honest finding code, never a wrong node
//! with exit 0.
//!
//! Byte-stability is asserted over the WHOLE committed document against the staged
//! promote source (`git show HEAD:CHANGELOG.md == <task working area copy>`),
//! never scoped to a filled subtree (the M13 cold-start discipline).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow24-{tag}-{}-{:?}",
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
fn run_jigc(repo: &Path, home: &Path, pack: &Path, args: &[&str], stdin: Option<&[u8]>) -> Output {
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

/// The trimmed stdout of a successful `jigc` invocation, or a panic carrying both streams.
fn ok_stdout(out: Output, what: &str) -> String {
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

/// `git show <rev>:<path>` over the repo, returning the committed bytes as a String,
/// or a panic carrying stderr (a missing path fails the show).
fn git_show(repo: &Path, spec: &str) -> String {
    let out = Command::new("git")
        .args(["show", spec])
        .current_dir(repo)
        .output()
        .expect("git show");
    assert!(
        out.status.success(),
        "`git show {spec}` must succeed; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 committed bytes")
}

/// The number of commits reachable from HEAD.
fn head_count(repo: &Path) -> u32 {
    let out = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(repo)
        .output()
        .expect("git rev-list");
    assert!(out.status.success(), "git rev-list --count HEAD failed");
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .parse()
        .expect("commit count parses")
}

/// The staged `changelog:changelog` instance in a task's working area (the promote source).
fn staged_changelog(repo: &Path, task: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// Fill the engine-native `commit` doc for `task` so finalize has a renderable VCS
/// message and the changelog conformance gate is the only structural concern.
fn fill_commit(repo: &Path, home: &Path, pack: &Path, task: &str, scope: &str, summary: &str) {
    let set_field = |key: &str, value: &str| {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "set-field",
                    &format!("commit:{task}#{key}"),
                    "--value",
                    value,
                ],
                None,
            ),
            &format!("set-field commit:{task}#{key}"),
        );
    };
    let set_slot = |key: &str, prose: &[u8]| {
        ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &[
                    "doc",
                    "set-slot",
                    &format!("commit:{task}#{key}"),
                    "--from-file",
                    "-",
                ],
                Some(prose),
            ),
            &format!("set-slot commit:{task}#{key}"),
        );
    };
    set_field("type", "docs");
    set_field("scope", scope);
    set_slot("summary", format!("{summary}\n").as_bytes());
    set_slot("body", b"A changelog change.\n");
}

/// Author one nested change-group (`#### <category>` + its `notes` slot) under a parent
/// item address, driving the EMITTED `add-item` address verbatim downstream.
fn author_group(
    repo: &Path,
    home: &Path,
    pack: &Path,
    parent: &str,
    category: &str,
    notes: &[u8],
) -> String {
    let group = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                &format!("{parent}/changes"),
                "--title",
                category,
            ],
            None,
        ),
        &format!("add-item {parent}/changes ({category})"),
    );
    // The emitted nested-item address is the canonical SECTION-QUALIFIED form (review
    // finding S1, `design/changelog.md` → engine work #1): it carries the `changes`
    // nested-section segment, and the `set-slot` below drives it VERBATIM — proving the
    // address an agent actually types (`#releases/<id>/changes/<cat>/notes`) resolves,
    // not just the previously-emitted segment-less form (the masking-test guard).
    assert_eq!(
        group,
        format!("{parent}/changes/{category}"),
        "the minted nested-group address is section-qualified (includes `changes`)",
    );
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
            ],
            Some(notes),
        ),
        &format!("set-slot {group}/notes"),
    );
    group
}

/// Assert `jigc doc show <addr>` reads back `plain` and — with `--format json` —
/// `json`, verbatim (M40 A2: the addressed node round-trips in BOTH formats).
fn assert_shows(repo: &Path, home: &Path, pack: &Path, addr: &str, plain: &str, json: &str) {
    let got = ok_stdout(
        run_jigc(repo, home, pack, &["doc", "show", addr], None),
        &format!("doc show {addr}"),
    );
    assert_eq!(got, plain, "`doc show {addr}` (plain) reads back verbatim");
    let got = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "show", addr, "--format", "json"],
            None,
        ),
        &format!("doc show {addr} --format json"),
    );
    assert_eq!(got, json, "`doc show {addr}` (json) reads back verbatim");
}

/// Assert `jigc doc show <addr>` BLOCKS honestly in BOTH formats: non-zero exit, the
/// finding `code` + the located `detail` on stderr, and an EMPTY stdout — a wrong
/// nested address never returns a node with exit 0 (the M40 rule).
fn assert_show_blocks(repo: &Path, home: &Path, pack: &Path, addr: &str, code: &str, detail: &str) {
    let plain = ["doc", "show", addr];
    let json = ["doc", "show", addr, "--format", "json"];
    for args in [&plain[..], &json[..]] {
        let out = run_jigc(repo, home, pack, args, None);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success(),
            "`jigc {args:?}` must exit non-zero; stdout:\n{}\nstderr:\n{stderr}",
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains(code) && stderr.contains(detail),
            "`jigc {args:?}` must block with `{code}` naming {detail:?}; stderr:\n{stderr}",
        );
        assert!(
            out.stdout.is_empty(),
            "a blocked show emits NO node on stdout (never a wrong node); stdout:\n{}",
            String::from_utf8_lossy(&out.stdout),
        );
    }
}

// ---- the A2 read-back goldens (spiked against the rebuilt binary; `<DATE1>`/
// ---- `<DATE2>` interpolated — the `date` field is `set: on-create`) ----

/// `doc show <rel1>` plain — the release item slice carries its field group + the
/// nested change-groups (self-rooted at `###` — the T1 fix), each heading in the
/// writer's exact canonical form: title, two spaces, the frozen `{#id}` anchor (M42
/// inc-8 T4 — the id an agent addresses the item back by, at every depth).
const REL1_ITEM_PLAIN: &str = "### 1.0.0  {#1-0-0}

<!-- fields -->
- date: <DATE1>
- link: https://example.com/compare/0.9.0...1.0.0

#### added  {#added}

- OAuth device-code flow

#### fixed  {#fixed}

- session fixation on logout";

/// `doc show <rel2>` plain — no `link` line (the absent optional field), and the
/// run-2 groups.
const REL2_ITEM_PLAIN: &str = "### 1.1.0  {#1-1-0}

<!-- fields -->
- date: <DATE2>

#### changed  {#changed}

- new config knob

#### removed  {#removed}

- deprecated endpoint dropped";

/// `doc show <rel1> --format json` — the pinned recursive item object: the `changes`
/// nested block keys the array of recursive item objects.
const REL1_ITEM_JSON: &str = r#"{
  "changes": [
    {
      "category": "added",
      "id": "added",
      "notes": "- OAuth device-code flow"
    },
    {
      "category": "fixed",
      "id": "fixed",
      "notes": "- session fixation on logout"
    }
  ],
  "date": "<DATE1>",
  "id": "1-0-0",
  "link": "https://example.com/compare/0.9.0...1.0.0",
  "title": "1.0.0"
}"#;

/// `doc show <rel2> --format json` — the absent optional `link` key is simply absent.
const REL2_ITEM_JSON: &str = r#"{
  "changes": [
    {
      "category": "changed",
      "id": "changed",
      "notes": "- new config knob"
    },
    {
      "category": "removed",
      "id": "removed",
      "notes": "- deprecated endpoint dropped"
    }
  ],
  "date": "<DATE2>",
  "id": "1-1-0",
  "title": "1.1.0"
}"#;

/// `doc show changelog:changelog#releases --format json` — the section slice is the
/// array of BOTH recursive release objects, in on-disk order.
const RELEASES_SECTION_JSON: &str = r#"[
  {
    "changes": [
      {
        "category": "added",
        "id": "added",
        "notes": "- OAuth device-code flow"
      },
      {
        "category": "fixed",
        "id": "fixed",
        "notes": "- session fixation on logout"
      }
    ],
    "date": "<DATE1>",
    "id": "1-0-0",
    "link": "https://example.com/compare/0.9.0...1.0.0",
    "title": "1.0.0"
  },
  {
    "changes": [
      {
        "category": "changed",
        "id": "changed",
        "notes": "- new config knob"
      },
      {
        "category": "removed",
        "id": "removed",
        "notes": "- deprecated endpoint dropped"
      }
    ],
    "date": "<DATE2>",
    "id": "1-1-0",
    "title": "1.1.0"
  }
]"#;

/// `doc show <rel1>/changes` plain — the nested ARRAY slices self-rooted (each group
/// at `###`, like a section-level item list).
const REL1_CHANGES_PLAIN: &str = "### added  {#added}

- OAuth device-code flow

### fixed  {#fixed}

- session fixation on logout";

/// `doc show <rel1>/changes --format json` — the canonical write address → the
/// nested array of recursive item objects.
const REL1_CHANGES_JSON: &str = r#"[
  {
    "category": "added",
    "id": "added",
    "notes": "- OAuth device-code flow"
  },
  {
    "category": "fixed",
    "id": "fixed",
    "notes": "- session fixation on logout"
  }
]"#;

/// The full Flow-24 acceptance: cold-create → warm-append byte-stable, both reds, in
/// one continuous two-run e2e over a single repo (the proven flow-19 two-run shape).
#[test]
fn flow24_cold_create_then_warm_append_byte_stable_with_the_reds() {
    let repo = TempDir::new("main");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), pack.as_path());

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");

    // ── RUN 1 (cold): record-change mints off-router, cold-create the singleton ──────
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["start", "--workflow", "record-change", "cut 1.0.0"],
            None,
        ),
        "jigc start --workflow record-change (run 1)",
    );
    let task1 = "cut-1-0-0";

    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog (cold)",
    );
    assert_eq!(
        created, "changelog:changelog",
        "the singleton cold-mints at the fixed slug = the type id",
    );

    // The version-title slugger MAPS dots (since slug-rule-version 2) → id `1-0-0`; drive the
    // EMITTED address verbatim regardless.
    let rel1 = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.0.0",
            ],
            None,
        ),
        "add-item release 1.0.0",
    );
    assert!(
        rel1.starts_with("changelog:changelog#releases/"),
        "the release address is under #releases/; got {rel1}",
    );

    // The OPTIONAL `link` field IS authored on 1.0.0.
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-field",
                &format!("{rel1}/link"),
                "--value",
                "https://example.com/compare/0.9.0...1.0.0",
            ],
            None,
        ),
        "set-field 1.0.0 link",
    );

    // Nested change-groups under the release (the Leaf::Repeatable authoring path).
    let g_added = author_group(
        repo,
        home,
        pack,
        &rel1,
        "added",
        b"- OAuth device-code flow\n",
    );
    let g_fixed = author_group(
        repo,
        home,
        pack,
        &rel1,
        "fixed",
        b"- session fixation on logout\n",
    );

    fill_commit(repo, home, pack, task1, "changelog", "cut 1.0.0");

    // Capture the staged promote source, then finalize and assert committed == staged.
    let staged1 = staged_changelog(repo, task1);
    let before1 = head_count(repo);
    let fin1 = run_jigc(repo, home, pack, &["task", "finalize", task1], None);
    assert!(
        fin1.status.success(),
        "run 1 finalize must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&fin1.stdout),
        String::from_utf8_lossy(&fin1.stderr),
    );
    assert_eq!(
        head_count(repo),
        before1 + 1,
        "run 1 finalize must land exactly ONE commit",
    );

    let committed1 = git_show(repo, "HEAD:CHANGELOG.md");
    assert_eq!(
        committed1, staged1,
        "run 1: committed bytes == staged promote source (whole-doc byte-stable)",
    );
    // Bar 2: the nested groups + the multi-word section are intact in the committed bytes.
    assert!(
        committed1.contains("## Unreleased Changes"),
        "the multi-word section heading is intact; committed:\n{committed1}",
    );
    assert!(
        committed1.contains("#### added  {#added}") && committed1.contains("#### fixed  {#fixed}"),
        "the nested change-groups are present; committed:\n{committed1}",
    );

    // ── RUN 2 (warm): warm-re-create preserves 1.0.0, append 1.1.0 (no link) ────────
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["start", "--workflow", "record-change", "cut 1.1.0"],
            None,
        ),
        "jigc start --workflow record-change (run 2)",
    );
    let task2 = "cut-1-1-0";

    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog (warm copy-in)",
    );
    // Bar 3: the warm copy-in preserves the prior 1.0.0 release in the working area.
    let warm_staged = staged_changelog(repo, task2);
    assert!(
        warm_staged.contains("### 1.0.0  {#1-0-0}")
            && warm_staged.contains("- OAuth device-code flow"),
        "warm create copies the committed 1.0.0 release in; staged:\n{warm_staged}",
    );

    let rel2 = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.1.0",
            ],
            None,
        ),
        "add-item release 1.1.0",
    );
    // 1.1.0 authors a `changed` group but leaves the OPTIONAL `link` ABSENT this run.
    let g_changed = author_group(repo, home, pack, &rel2, "changed", b"- new config knob\n");

    // ── RED 1: a half-authored NESTED entry (empty required `notes`) BLOCKS ──────────
    let removed = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                &format!("{rel2}/changes"),
                "--title",
                "removed",
            ],
            None,
        ),
        "add-item 1.1.0/changes (removed, left empty)",
    );
    fill_commit(repo, home, pack, task2, "changelog", "cut 1.1.0");

    let head_pre_red = head_count(repo);
    let blocked = run_jigc(repo, home, pack, &["task", "finalize", task2], None);
    assert!(
        !blocked.status.success(),
        "a half-authored nested entry must make finalize exit non-zero; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&blocked.stdout),
        String::from_utf8_lossy(&blocked.stderr),
    );
    let blocked_err = String::from_utf8_lossy(&blocked.stderr);
    assert!(
        blocked_err.contains("schema-conformance.required-slot-present"),
        "red 1 must fire `schema-conformance.required-slot-present`; stderr:\n{blocked_err}",
    );
    // The gate names the genuinely-empty NESTED leaf at its canonical SECTION-QUALIFIED
    // address — `releases/<v>/changes/<group>/notes`, the same form `add-item` emits and
    // `set-slot` accepts (review finding S1; the parent-scoped path locator confirmed at
    // the spike, B1/S1). We assert the full qualified group fragment (which carries the
    // `changes` segment) appears verbatim, so the gate names the address an agent types.
    let nested_group = removed
        .strip_prefix("changelog:changelog#")
        .expect("nested group address is a changelog fragment");
    assert!(
        nested_group.contains("/changes/"),
        "the emitted nested-group address is section-qualified; got {nested_group}",
    );
    assert!(
        blocked_err.contains(nested_group) && blocked_err.contains("notes"),
        "red 1 must name the section-qualified NESTED group {nested_group} and its `notes` \
         leaf; stderr:\n{blocked_err}",
    );
    assert_eq!(
        head_count(repo),
        head_pre_red,
        "a blocked finalize commits nothing",
    );

    // ── RED 2: fill the nested leaf → finalize CLEAN even though 1.1.0 has NO link ───
    // (an absent OPTIONAL field finalizes clean while the required slot, once empty,
    //  blocked above — the two halves of the optional/required contrast).
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("{removed}/notes"),
                "--from-file",
                "-",
            ],
            Some(b"- deprecated endpoint dropped\n"),
        ),
        "set-slot 1.1.0/removed/notes",
    );

    let staged2 = staged_changelog(repo, task2);
    // The absent optional `link` renders with NO stray fields-block line for 1.1.0.
    let rel2_block = staged2
        .split("### 1.1.0")
        .nth(1)
        .expect("1.1.0 block present in staged doc");
    assert!(
        !rel2_block.contains("- link:"),
        "the absent optional link emits no stray fields-block line; 1.1.0 block:\n{rel2_block}",
    );

    let before2 = head_count(repo);
    let fin2 = run_jigc(repo, home, pack, &["task", "finalize", task2], None);
    assert!(
        fin2.status.success(),
        "run 2 finalize (absent optional link) must exit 0 CLEAN; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&fin2.stdout),
        String::from_utf8_lossy(&fin2.stderr),
    );
    assert_eq!(
        head_count(repo),
        before2 + 1,
        "run 2 finalize must land exactly ONE commit",
    );

    let committed2 = git_show(repo, "HEAD:CHANGELOG.md");
    assert_eq!(
        committed2, staged2,
        "run 2: re-promoted bytes == staged promote source (whole-doc byte-stable)",
    );
    // Bar 3: BOTH releases + their nested groups are present after the warm append.
    assert!(
        committed2.contains("### 1.0.0  {#1-0-0}") && committed2.contains("### 1.1.0  {#1-1-0}"),
        "both releases are present after warm append; committed:\n{committed2}",
    );
    assert!(
        committed2.contains("- OAuth device-code flow")
            && committed2.contains("- new config knob")
            && committed2.contains("- deprecated endpoint dropped"),
        "the nested change-groups of both releases survive; committed:\n{committed2}",
    );

    // ── M40 A2: the nested corpus ROUND-TRIPS — every emitted `add-item` address ─────
    // reads back verbatim through the task-less `doc show`, plain AND json, at every
    // depth. The `date` field is `set: on-create` (non-deterministic), so each
    // release's stamp is read back through the already-pinned leaf slice and
    // interpolated into the goldens; every other byte is matched verbatim.
    let date_of = |rel: &str| {
        let date = ok_stdout(
            run_jigc(
                repo,
                home,
                pack,
                &["doc", "show", &format!("{rel}/date")],
                None,
            ),
            &format!("doc show {rel}/date (plain leaf)"),
        );
        assert!(
            !date.is_empty() && date.chars().all(|c| c.is_ascii_digit() || c == '-'),
            "the date leaf reads back as the bare date value; got {date:?}",
        );
        date
    };
    let (date1, date2) = (date_of(&rel1), date_of(&rel2));
    let dated = |golden: &str| golden.replace("<DATE1>", &date1).replace("<DATE2>", &date2);

    // Section depth: `#releases` — plain is the two release items joined (fields +
    // nested groups carried, the T1 fix); json is the array of recursive objects.
    assert_shows(
        repo,
        home,
        pack,
        "changelog:changelog#releases",
        &format!("{}\n\n{}", dated(REL1_ITEM_PLAIN), dated(REL2_ITEM_PLAIN)),
        &dated(RELEASES_SECTION_JSON),
    );
    // Item depth: both emitted release addresses, verbatim.
    assert_shows(
        repo,
        home,
        pack,
        &rel1,
        &dated(REL1_ITEM_PLAIN),
        &dated(REL1_ITEM_JSON),
    );
    assert_shows(
        repo,
        home,
        pack,
        &rel2,
        &dated(REL2_ITEM_PLAIN),
        &dated(REL2_ITEM_JSON),
    );
    // Nested-array depth: the canonical section-qualified write address (this is the
    // address that falsely blocked `store.no-such-item` pre-M40).
    assert_shows(
        repo,
        home,
        pack,
        &format!("{rel1}/changes"),
        REL1_CHANGES_PLAIN,
        REL1_CHANGES_JSON,
    );
    // Nested-item + nested-leaf depth: EVERY emitted nested-group address (run 1's
    // two + run 2's two, `removed` being the empty-then-filled red-1 group) reads
    // back as its own node — the deep leaf previously degraded to the ENCLOSING
    // release item with exit 0 (the wrong-node defect).
    for (group, category, notes) in [
        (&g_added, "added", "- OAuth device-code flow"),
        (&g_fixed, "fixed", "- session fixation on logout"),
        (&g_changed, "changed", "- new config knob"),
        (&removed, "removed", "- deprecated endpoint dropped"),
    ] {
        // The item's minted `id` joins the pinned item object (M42) — and it is read
        // from the EMITTED address's last segment, never rebuilt from the category:
        // the json key is exactly the handle the address grammar takes. The PLAIN slice
        // carries that same id as the heading's frozen `{#id}` anchor (M42 inc-8 T4), so
        // both formats hand back the handle you address the item by.
        let id = group.rsplit('/').next().expect("the emitted item address");
        assert_shows(
            repo,
            home,
            pack,
            group,
            &format!("### {category}  {{#{id}}}\n\n{notes}"),
            &format!(
                "{{\n  \"category\": \"{category}\",\n  \"id\": \"{id}\",\n  \"notes\": \"{notes}\"\n}}"
            ),
        );
        assert_shows(
            repo,
            home,
            pack,
            &format!("{group}/notes"),
            notes,
            &format!("\"{notes}\""),
        );
    }

    // ── M40 A2: a wrong nested address BLOCKS honestly in both formats — never a ─────
    // wrong node with exit 0.
    // A mistyped nested-section segment is rejected, never treated as an item id.
    assert_show_blocks(
        repo,
        home,
        pack,
        &format!("{rel1}/wrong/added"),
        "store.no-such-section",
        "names no nested section `wrong`",
    );
    // The segment-less physical shortcut blocks with a route naming the declared
    // nested section (S1: the one canonical address).
    assert_show_blocks(
        repo,
        home,
        pack,
        &format!("{rel1}/added"),
        "store.no-such-leaf",
        "address it through its declared nested section (changes)",
    );
    // An absent group id under the declared nested section is the plain item miss.
    assert_show_blocks(
        repo,
        home,
        pack,
        &format!("{rel1}/changes/security"),
        "store.no-such-item",
        "names no item `security`",
    );
}

/// Bar 5: the `single-task` fold-in (`allows-create:[{type: changelog, as: change}]`)
/// lets an everyday coding task append an unreleased change-group through the same
/// create-gate it uses for an ADR, and finalize promotes it.
#[test]
fn flow24_single_task_fold_in_appends_an_unreleased_entry_and_promotes() {
    let repo = TempDir::new("foldin");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), pack.as_path());

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "start",
                "--workflow",
                "single-task",
                "add an oauth login button",
            ],
            None,
        ),
        "jigc start --workflow single-task",
    );
    let task = "add-an-oauth-login-button";

    let created = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog (single-task gate)",
    );
    assert_eq!(created, "changelog:changelog");

    let group = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#unreleased-changes",
                "--title",
                "added",
            ],
            None,
        ),
        "add-item unreleased #added",
    );
    assert_eq!(group, "changelog:changelog#unreleased-changes/added");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "set-slot",
                &format!("{group}/notes"),
                "--from-file",
                "-",
            ],
            Some(b"- OAuth login button on the sign-in page\n"),
        ),
        "set-slot unreleased #added notes",
    );

    fill_commit(repo, home, pack, task, "auth", "add oauth login");
    // A real code change so the task's commit carries content beyond the promoted doc.
    fs::write(repo.join("login.txt"), "login\n").expect("write code change");

    let staged = staged_changelog(repo, task);
    let before = head_count(repo);
    let fin = run_jigc(repo, home, pack, &["task", "finalize", task], None);
    assert!(
        fin.status.success(),
        "single-task fold-in finalize must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&fin.stdout),
        String::from_utf8_lossy(&fin.stderr),
    );
    assert_eq!(
        head_count(repo),
        before + 1,
        "finalize lands exactly ONE commit"
    );

    let committed = git_show(repo, "HEAD:CHANGELOG.md");
    assert_eq!(
        committed, staged,
        "the fold-in-authored changelog promotes byte-stable",
    );
    // The unreleased change-groups are a SINGLE-level repeatable, so they render at the
    // level-1 depth (`###`), not the level-2 `####` the nested release groups use.
    assert!(
        committed.contains("## Unreleased Changes")
            && committed.contains("### added  {#added}")
            && committed.contains("- OAuth login button on the sign-in page"),
        "the unreleased entry promoted through the create-gate; committed:\n{committed}",
    );

    // M40 A2: the emitted SINGLE-level group address reads back verbatim in both
    // formats — the flat witness (no fields, no nested children). Its json carries the
    // item's minted `id` (M42 — the key that makes the contract closed under its own
    // address grammar: it is the last segment of the EMITTED address above).
    assert_shows(
        repo,
        home,
        pack,
        &group,
        "### added  {#added}\n\n- OAuth login button on the sign-in page",
        "{\n  \"category\": \"added\",\n  \"id\": \"added\",\n  \"notes\": \"- OAuth login button on the sign-in page\"\n}",
    );
    assert_shows(
        repo,
        home,
        pack,
        &format!("{group}/notes"),
        "- OAuth login button on the sign-in page",
        "\"- OAuth login button on the sign-in page\"",
    );
}

/// Bar 6 — the honest Flow-A-only bound: a foreign, non-conformant `CHANGELOG.md`
/// at the managed placement home (root `CHANGELOG.md`, post-M38) still routes
/// `needs-reconcile` (auto-migration G1 is the separate later milestone —
/// `changelog.md` → honest bounds). The doctype helps a project authoring its changelog
/// through jigc from the start, NOT an imported foreign one.
#[test]
fn flow24_foreign_changelog_routes_needs_reconcile() {
    let repo = TempDir::new("foreign");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), pack.as_path());

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");

    // A real foreign Keep-a-Changelog file at the managed placement home (root
    // `CHANGELOG.md`) — the strict canonical parser rejects it (its release headings are
    // not the managed section ids), so it is a near-miss at home, not adoptable.
    fs::write(
        repo.join("CHANGELOG.md"),
        "# Changelog\n\nAll notable changes.\n\n## [1.0.0] - 2020-01-01\n- did stuff\n",
    )
    .expect("write foreign changelog");
    let git = |args: &[&str]| {
        Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .expect("git");
    };
    git(&["add", "CHANGELOG.md"]);
    git(&["commit", "-q", "-m", "foreign changelog"]);

    let ingest = run_jigc(repo, home, pack, &["ingest"], None);
    let report = format!(
        "{}{}",
        String::from_utf8_lossy(&ingest.stdout),
        String::from_utf8_lossy(&ingest.stderr),
    );
    assert!(
        report.contains("needs-reconcile CHANGELOG.md"),
        "a foreign changelog at the managed placement home routes needs-reconcile; report:\n{report}",
    );
}

/// M40 triage — an UNDECLARED field key on a NESTED item must never reach the committed
/// store. The change-group template declares NO bullet fields (`category` is the
/// heading-derived id-source, `notes` a slot), so a stray `- title: Changed` fields block
/// after the notes prose is undeclared bytes.
///
/// **M47 Increment 6, T4 — the pin moves with the fix that repairs it.** This arm used to
/// assert the write itself lands **exit-0** ("mirroring the top-level undeclared write,
/// whose gate is also validate/finalize, not write time") and then be caught downstream.
/// That premise is gone in both halves: the undeclared address is now rejected at the
/// **write door**, `write.unknown-field`, nothing persisted, at every depth and at both
/// the per-leaf and batch verbs (swept as data in `undeclared_address_writes.rs`;
/// `DECISIONS.md` → 2026-07-26 M47 Settle, Decision 9). The claim this arm carries is
/// unchanged and now proven **twice**, so it asserts the write-time reject first.
///
/// The downstream gate keeps its own live witness on the one cause still reachable — the
/// stray bullet arriving **out of band**, which is exactly the shape the `conformance.*`
/// route exemption was written for (*"no CLI verb repairs a hand-broken byte"*) and which
/// the storage model guarantees stays possible (`design/storage.md`: managed docs are
/// plain, human-editable files). Hand-broken, `task validate` must still fire
/// `conformance.unknown-field` and `task finalize` must still block, committing nothing.
#[test]
fn flow24_nested_undeclared_field_blocks_at_validate_and_finalize() {
    let repo = TempDir::new("nested-undeclared");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), pack.as_path());

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");
    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["start", "--workflow", "record-change", "cut 1.0.0"],
            None,
        ),
        "jigc start --workflow record-change",
    );
    let task = "cut-1-0-0";

    ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &["doc", "create", "changelog", "--title", "Changelog"],
            None,
        ),
        "jigc doc create changelog",
    );
    let rel = ok_stdout(
        run_jigc(
            repo,
            home,
            pack,
            &[
                "doc",
                "add-item",
                "changelog:changelog#releases",
                "--title",
                "1.0.0",
            ],
            None,
        ),
        "add-item release 1.0.0",
    );
    let group = author_group(
        repo,
        home,
        pack,
        &rel,
        "added",
        b"- OAuth device-code flow\n",
    );
    fill_commit(repo, home, pack, task, "changelog", "cut 1.0.0");

    // The undeclared write: `title` is not a declared field of the change-group block.
    // The write door refuses it outright and persists nothing — the class the M40 triage
    // could only catch downstream is now closed at its source.
    let rejected = run_jigc(
        repo,
        home,
        pack,
        &[
            "doc",
            "set-field",
            &format!("{group}/title"),
            "--value",
            "Changed",
        ],
        None,
    );
    assert!(
        !rejected.status.success(),
        "set-field at an undeclared nested field must block; stdout:\n{}",
        String::from_utf8_lossy(&rejected.stdout),
    );
    let reject = String::from_utf8_lossy(&rejected.stderr);
    assert!(
        reject.contains("write.unknown-field"),
        "the reject names the undeclared address; stderr:\n{reject}",
    );
    let clean = staged_changelog(repo, task);
    assert!(
        !clean.contains("- title: Changed"),
        "the rejected write persisted nothing; staged:\n{clean}",
    );

    // The one cause still reachable: the stray bullet arrives **out of band** — a human
    // edit of the staged working copy, the shape the downstream gate exists for. These
    // are the exact bytes the write verb used to splice.
    let staged_path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join("changelog:changelog.md");
    fs::write(
        &staged_path,
        format!("{clean}\n<!-- fields -->\n- title: Changed\n"),
    )
    .expect("hand-break the staged working copy");
    let staged = staged_changelog(repo, task);
    assert!(
        staged.contains("- title: Changed"),
        "the stray fields block is in the staged bytes; staged:\n{staged}",
    );

    // `task validate` fires `conformance.unknown-field` naming the undeclared key.
    let validate = run_jigc(repo, home, pack, &["task", "validate", task], None);
    let report = format!(
        "{}{}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr),
    );
    assert!(
        !validate.status.success(),
        "task validate must exit non-zero on the nested undeclared field; report:\n{report}",
    );
    assert!(
        report.contains("conformance.unknown-field") && report.contains("`title`"),
        "task validate must fire `conformance.unknown-field` naming `title`; report:\n{report}",
    );

    // `task finalize` blocks — the corrupted bytes never reach the committed store.
    let before = head_count(repo);
    let finalize = run_jigc(repo, home, pack, &["task", "finalize", task], None);
    assert!(
        !finalize.status.success(),
        "task finalize must block on the nested undeclared field; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr),
    );
    assert_eq!(
        head_count(repo),
        before,
        "a blocked finalize commits nothing",
    );
}
