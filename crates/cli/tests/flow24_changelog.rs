//! M22 Increment 5, T3 — Flow 24 end-to-end acceptance: the `changelog` doctype's
//! cold-create → warm-append byte-stable proof, the two reds, and the `single-task`
//! fold-in bar, all driven through the **rebuilt** `jigc` binary over a `git init`
//! temp repo against the **shipped** dev pack (selected via `JIGC_PACK_DIR` = the
//! embedded `pack/` tree, so it is the bytes that ship, not a fixture).
//!
//! The multi-level authoring path is net-new engine, so the verb sequence was spiked
//! against the rebuilt binary at build (M16 exercise-don't-infer). The spike confirmed
//! a load-bearing fact the design's illustrative notation glossed: the version-title
//! slugger DROPS dots, so `add-item --title "1.0.0"` mints id `100` (not `1-0-0`).
//! Every downstream address here is therefore driven from the EMITTED `add-item`
//! address verbatim — never a reconstructed `1-0-0` form — so the test asserts the
//! bytes an agent would actually run (the masking-test guard).
//!
//! `worked-examples.md` flow 24 acceptance bar:
//!   1. a new project authors a managed `changelog` through jigc, promoted byte-stable;
//!   2. the nested `Leaf::Repeatable` (release → change-group) round-trips;
//!   3. maintained over two runs — warm-re-create (copy-in preserves the prior release)
//!      then append a second release, re-promoted byte-stable with BOTH present;
//!   4. the conformance gate fires on a real empty NESTED leaf (red 1), and the
//!      absent OPTIONAL `link` finalizes clean while a required slot still blocks (red 2);
//!   5. the `single-task` fold-in appends an unreleased entry through the create-gate;
//!   6. the honest Flow-A-only bound — a foreign `changelog/CHANGELOG.md` routes
//!      `needs-reconcile` (auto-migration G1 is the separate later milestone).
//!
//! Byte-stability is asserted over the WHOLE committed document against the staged
//! promote source (`git show HEAD:changelog/changelog.md == <task working area copy>`),
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
    let task1 = "cut-100";

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

    // The version-title slugger drops dots → id `100`; drive the EMITTED address verbatim.
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
    author_group(
        repo,
        home,
        pack,
        &rel1,
        "added",
        b"- OAuth device-code flow\n",
    );
    author_group(
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

    let committed1 = git_show(repo, "HEAD:changelog/changelog.md");
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
    let task2 = "cut-110";

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
        warm_staged.contains("### 1.0.0  {#100}")
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
    author_group(repo, home, pack, &rel2, "changed", b"- new config knob\n");

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
    // The gate names the genuinely-empty NESTED leaf address — the parent-scoped path
    // (`releases/<v>/changes/<group>/notes` projected to `releases/<v>/<group>`), which
    // depends on the parent-scoped path locator confirmed at the spike (B1/S1).
    let nested_leaf = removed
        .strip_prefix("changelog:changelog#")
        .expect("nested group address is a changelog fragment");
    assert!(
        blocked_err.contains(nested_leaf) && blocked_err.contains("notes"),
        "red 1 must name the NESTED leaf {nested_leaf}; stderr:\n{blocked_err}",
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

    let committed2 = git_show(repo, "HEAD:changelog/changelog.md");
    assert_eq!(
        committed2, staged2,
        "run 2: re-promoted bytes == staged promote source (whole-doc byte-stable)",
    );
    // Bar 3: BOTH releases + their nested groups are present after the warm append.
    assert!(
        committed2.contains("### 1.0.0  {#100}") && committed2.contains("### 1.1.0  {#110}"),
        "both releases are present after warm append; committed:\n{committed2}",
    );
    assert!(
        committed2.contains("- OAuth device-code flow")
            && committed2.contains("- new config knob")
            && committed2.contains("- deprecated endpoint dropped"),
        "the nested change-groups of both releases survive; committed:\n{committed2}",
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

    let committed = git_show(repo, "HEAD:changelog/changelog.md");
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
}

/// Bar 6 — the honest Flow-A-only bound: a foreign, non-conformant `changelog/CHANGELOG.md`
/// at the managed location still routes `needs-reconcile` (auto-migration G1 is the
/// separate later milestone — `changelog.md` → honest bounds). The doctype helps a
/// project authoring its changelog through jigc from the start, NOT an imported foreign one.
#[test]
fn flow24_foreign_changelog_routes_needs_reconcile() {
    let repo = TempDir::new("foreign");
    let home = TempDir::new("home");
    let pack = dev_pack();
    init_repo(repo.path());
    let (repo, home, pack) = (repo.path(), home.path(), pack.as_path());

    ok_stdout(run_jigc(repo, home, pack, &["setup"], None), "jigc setup");

    // A real foreign Keep-a-Changelog file at the managed location — the strict canonical
    // parser rejects it (its release headings are not the managed section ids).
    fs::create_dir_all(repo.join("changelog")).expect("create changelog dir");
    fs::write(
        repo.join("changelog").join("CHANGELOG.md"),
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
    git(&["add", "changelog/CHANGELOG.md"]);
    git(&["commit", "-q", "-m", "foreign changelog"]);

    let ingest = run_jigc(repo, home, pack, &["ingest"], None);
    let report = format!(
        "{}{}",
        String::from_utf8_lossy(&ingest.stdout),
        String::from_utf8_lossy(&ingest.stderr),
    );
    assert!(
        report.contains("needs-reconcile changelog/CHANGELOG.md"),
        "a foreign changelog at the managed location routes needs-reconcile; report:\n{report}",
    );
}
