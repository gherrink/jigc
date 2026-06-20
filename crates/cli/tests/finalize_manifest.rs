//! B1 dirty-tree sweep — `jigc task finalize` surfaces a **pre-commit manifest** (the
//! file-set the commit carries, untracked sweeps flagged distinctly) and adds a
//! `--dry-run` flag that prints the manifest and stops, committing nothing and touching
//! nothing (`DECISIONS.md` 2026-06-19; `design/finalize.md` → Dirty-tree policy).
//!
//! The accepted criterion: a stray `scratch.txt` is *surfaced* — B1 does not prevent its
//! inclusion on a normal run, it makes the set visible (flagged untracked). The
//! assertions inspect the **landed git commit** (its name-only delta) and the **emitted
//! manifest bytes** the binary printed, never a reconstruction.
//!
//! Drives the built `jigc` binary against throwaway git repos: the binary path comes from
//! `CARGO_BIN_EXE_jigc`, the pack from `JIGC_PACK_DIR`, and a self-cleaning `TempDir` with
//! an isolated `$HOME` keeps the test off the developer's machine.

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
            "jigc-finalize-manifest-{tag}-{}-{:?}",
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

/// The embedded dev pack tree on disk — selected via `JIGC_PACK_DIR`.
fn dev_pack() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("pack")
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
        .trim_end_matches('\n')
        .to_string()
}

/// Initialize a real git repo with one commit.
fn init_repo(repo: &Path) {
    git(repo, &["init", "-q"]);
    git(repo, &["config", "user.email", "test@example.com"]);
    git(repo, &["config", "user.name", "Test"]);
    fs::write(repo.join("README.md"), "hello\n").expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "initial"]);
}

/// Run `jigc <args>` with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`, optionally
/// piping `stdin`, capturing output.
fn run_jigc(repo: &Path, home: &Path, args: &[&str], stdin: Option<&[u8]>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(repo)
        .env("HOME", home)
        .env("JIGC_PACK_DIR", dev_pack())
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

/// Run `jigc <args>` (no stdin), asserting exit 0, returning trimmed stdout.
fn ok_stdout(repo: &Path, home: &Path, args: &[&str], what: &str) -> String {
    let out = run_jigc(repo, home, args, None);
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

/// Fill every author-required field/slot of the provisioned commit doc.
fn fill_commit(repo: &Path, home: &Path, task: &str) {
    let set_field = |addr: &str, value: &str| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-field", addr, "--value", value],
            None,
        );
        assert!(out.status.success(), "set-field {addr} must succeed");
    };
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_field(&format!("commit:{task}#type"), "feat");
    set_field(&format!("commit:{task}#scope"), "cache");
    set_slot(&format!("commit:{task}#summary"), b"surface the manifest\n");
    set_slot(&format!("commit:{task}#body"), b"A B1 change.\n");
}

/// Fill an ADR's author-required prose slots.
fn fill_adr_slots(repo: &Path, home: &Path, slug: &str) {
    let set_slot = |addr: &str, prose: &[u8]| {
        let out = run_jigc(
            repo,
            home,
            &["doc", "set-slot", addr, "--from-file", "-"],
            Some(prose),
        );
        assert!(out.status.success(), "set-slot {addr} must succeed");
    };
    set_slot(
        &format!("adr:{slug}#context"),
        b"A decision warranted recording.\n",
    );
    set_slot(&format!("adr:{slug}#decision"), b"Surface the file-set.\n");
    set_slot(
        &format!("adr:{slug}#consequences"),
        b"Stray files are visible.\n",
    );
}

/// Mint a `single-task`, create + author an ADR, fill the commit doc, and leave a stray
/// untracked `scratch.txt` in the tree. Returns `(task, slug)`.
fn seed_task_with_stray(repo: &Path, home: &Path, intent: &str) -> (String, String) {
    ok_stdout(repo, home, &["setup"], "jigc setup");
    ok_stdout(
        repo,
        home,
        &["start", "--workflow", "single-task", intent],
        "jigc start",
    );
    let task = intent.replace(' ', "-");
    let create = run_jigc(
        repo,
        home,
        &["doc", "create", "adr", "--title", "A recorded decision"],
        None,
    );
    assert!(create.status.success(), "jigc doc create adr must succeed");
    let adr = String::from_utf8(create.stdout)
        .expect("utf-8")
        .trim()
        .to_string();
    let slug = adr.strip_prefix("adr:").expect("adr:<slug>").to_string();
    fill_adr_slots(repo, home, &slug);
    fill_commit(repo, home, &task);
    // The unrelated untracked stray the sweep would carry in.
    fs::write(repo.join("scratch.txt"), "private WIP\n").expect("write scratch.txt");
    (task, slug)
}

/// (i) M30 — a normal finalize with an unstaged stray `scratch.txt` does NOT commit it
/// (the per-task `IndexHonoring` narrowing no longer sweeps the ambient dirty tree); the
/// stray stays untracked in the working tree, the landed **included** manifest carries only
/// the promoted ADR (never the stray), and the landed **left-out** residual names the stray
/// (G3 — post-commit `git status --porcelain` symmetry with the dry-run forecast).
#[test]
fn normal_finalize_leaves_unstaged_stray_uncommitted() {
    let repo = TempDir::new("normal");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let (task, slug) = seed_task_with_stray(repo.path(), home.path(), "surface stray");

    let stdout = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task],
        "jigc task finalize",
    );

    // M30 — the unstaged stray is NOT swept into the commit.
    let committed = git(repo.path(), &["show", "--name-only", "--format=", "HEAD"]);
    assert!(
        !committed.lines().any(|l| l == "scratch.txt"),
        "the unstaged stray `scratch.txt` must NOT ride the commit (M30 narrowing); files:\n{committed}",
    );
    // It remains untracked in the working tree post-commit.
    let status = git(repo.path(), &["status", "--porcelain"]);
    assert!(
        status.lines().any(|l| l == "?? scratch.txt"),
        "the stray stays untracked after the commit; status:\n{status}",
    );

    // The emitted manifest lists the promoted ADR as included...
    assert!(
        stdout.contains(&format!("promoted docs/decisions/{slug}.md")),
        "the manifest lists the promoted ADR; stdout:\n{stdout}",
    );
    // ...and names the uncommitted stray in the left-out residual (G3 — landed symmetry with
    // the dry-run forecast), never as a swept inclusion.
    assert!(
        stdout.contains("left-out") && stdout.contains("scratch.txt"),
        "the landed manifest names the uncommitted stray as left-out; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("swept (was untracked) scratch.txt"),
        "the stray is left-out, never a swept inclusion; stdout:\n{stdout}",
    );
}

/// (ii) `--dry-run` prints the manifest and stops: HEAD unchanged, nothing staged, no
/// promoted doc written, the stray untouched.
#[test]
fn dry_run_prints_manifest_and_commits_nothing() {
    let repo = TempDir::new("dry");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let (task, slug) = seed_task_with_stray(repo.path(), home.path(), "dry run");

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    let stdout = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", &task, "--dry-run"],
        "jigc task finalize --dry-run",
    );

    // The manifest is printed and names the unstaged stray as left out of the commit.
    assert!(
        stdout.contains("dry-run") && stdout.contains("left-out") && stdout.contains("scratch.txt"),
        "the dry-run prints a manifest naming the stray as left-out; stdout:\n{stdout}",
    );

    // No commit landed.
    let head_after = git(repo.path(), &["rev-parse", "HEAD"]);
    assert_eq!(
        head_before, head_after,
        "a dry-run must not move HEAD (it commits nothing)",
    );
    // Nothing was staged.
    let staged = git(repo.path(), &["diff", "--cached", "--name-only"]);
    assert!(
        staged.is_empty(),
        "a dry-run stages nothing; `git diff --cached`:\n{staged}",
    );
    // The promoted doc was not written to its destination.
    assert!(
        !repo
            .path()
            .join(format!("docs/decisions/{slug}.md"))
            .exists(),
        "a dry-run must not promote the ADR to its canonical location",
    );
    // The stray is still untracked and untouched.
    let status = git(repo.path(), &["status", "--porcelain"]);
    assert!(
        status.lines().any(|l| l == "?? scratch.txt"),
        "the stray is still untracked after a dry-run; status:\n{status}",
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("scratch.txt")).expect("read scratch.txt"),
        "private WIP\n",
        "the stray's bytes are untouched by a dry-run",
    );
}

/// (iii) `--format json` — the dry-run JSON carries `dry_run: true` + a `manifest[]`
/// (included) that does NOT carry the unstaged stray and a `left_out[]` that names it as
/// untracked (M30 G3 — the included/left-out split); the landed-run JSON carries
/// `committed.manifest` with the promoted ADR but NOT the stray, and `committed.left_out`
/// naming the stray as untracked (T2 — the post-commit residual, symmetric with the dry-run).
#[test]
fn json_manifest_on_dry_run_and_landed_run() {
    let repo = TempDir::new("json");
    let home = TempDir::new("home");
    init_repo(repo.path());
    let (task, slug) = seed_task_with_stray(repo.path(), home.path(), "json manifest");

    // Dry-run JSON first (no side effect).
    let dry = ok_stdout(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", &task, "--dry-run"],
        "jigc task finalize --dry-run --format json",
    );
    let dry: serde_json::Value = serde_json::from_str(&dry).expect("dry-run stdout parses as JSON");
    assert_eq!(
        dry["dry_run"],
        serde_json::Value::Bool(true),
        "dry-run JSON carries `dry_run: true`; value:\n{dry}",
    );
    let dry_manifest = dry["manifest"].as_array().expect("manifest is an array");
    assert!(
        !dry_manifest.iter().any(|e| e["path"] == "scratch.txt"),
        "dry-run included manifest must NOT carry the unstaged stray; manifest:\n{dry_manifest:?}",
    );
    let dry_left_out = dry["left_out"].as_array().expect("left_out is an array");
    assert!(
        dry_left_out
            .iter()
            .any(|e| e["path"] == "scratch.txt" && e["kind"] == "untracked"),
        "dry-run left_out names the stray as untracked; left_out:\n{dry_left_out:?}",
    );

    // Landed-run JSON.
    let landed = ok_stdout(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", &task],
        "jigc task finalize --format json",
    );
    let landed: serde_json::Value =
        serde_json::from_str(&landed).expect("landed stdout parses as JSON");
    let manifest = landed["committed"]["manifest"]
        .as_array()
        .expect("committed.manifest is an array");
    // M30 — the unstaged stray is no longer committed, so it is absent from the landed
    // manifest (which is derived from the commit's own delta).
    assert!(
        !manifest.iter().any(|e| e["path"] == "scratch.txt"),
        "committed.manifest must NOT carry the uncommitted stray; manifest:\n{manifest:?}",
    );
    assert!(
        manifest.iter().any(|e| {
            e["path"] == format!("docs/decisions/{slug}.md") && e["kind"] == "promoted"
        }),
        "committed.manifest carries the promoted ADR; manifest:\n{manifest:?}",
    );
    // `committed.files` stays present and equals the manifest length.
    assert_eq!(
        landed["committed"]["files"]
            .as_u64()
            .expect("files is a number"),
        manifest.len() as u64,
        "committed.files equals the manifest length",
    );
    // T2 — `committed.left_out` (the post-commit worktree residual) names the uncommitted
    // stray as untracked, symmetric with the dry-run forecast's `left_out`.
    let left_out = landed["committed"]["left_out"]
        .as_array()
        .expect("committed.left_out is an array");
    assert!(
        left_out
            .iter()
            .any(|e| e["path"] == "scratch.txt" && e["kind"] == "untracked"),
        "committed.left_out names the uncommitted stray as untracked; left_out:\n{left_out:?}",
    );
}

/// (iv) A migration `--dry-run` predicts exactly the narrowed set — the promoted canonical
/// doc, the retired foreign deletion, and the jigc-tracked config layer — and NOT an
/// unrelated `scratch.txt` (the narrowed `git add` never sweeps user WIP, B2).
#[test]
fn migration_dry_run_predicts_the_narrowed_set_not_user_wip() {
    let repo = TempDir::new("migrate");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // A committed foreign Keep-a-Changelog file (so its retirement is a tracked deletion).
    const FOREIGN: &str = "\
# Changelog

## [0.1.0] - 2021-03-09
### Added
- First public release.
";
    fs::write(repo.path().join("CHANGELOG.md"), FOREIGN).expect("write foreign");
    git(repo.path(), &["add", "CHANGELOG.md"]);
    git(
        repo.path(),
        &["commit", "-q", "-m", "track foreign changelog"],
    );

    ok_stdout(repo.path(), home.path(), &["setup"], "jigc setup");
    ok_stdout(
        repo.path(),
        home.path(),
        &["migrate", "CHANGELOG.md", "--as", "changelog"],
        "jigc migrate",
    );
    let task = "migrate-changelog-changelog";

    // An unrelated untracked stray — it must NOT enter the narrowed migration prediction.
    fs::write(repo.path().join("scratch.txt"), "private WIP\n").expect("write scratch.txt");

    const PAYLOAD: &str = r#"title: Changelog
sections:
  - id: releases
    items:
      - title: 0.1.0
        set:
          date: 2021-03-09
        sections:
          - id: changes
            items:
              - title: Added
                set:
                  notes: "<<- First public release.>>"
"#;
    let author = run_jigc(
        repo.path(),
        home.path(),
        &[
            "doc",
            "author",
            "changelog",
            "--from-file",
            "-",
            "--task",
            task,
        ],
        Some(PAYLOAD.as_bytes()),
    );
    assert!(author.status.success(), "jigc doc author must succeed");

    let head_before = git(repo.path(), &["rev-parse", "HEAD"]);

    // A migration dry-run needs no `--approve` — it commits nothing.
    let stdout = ok_stdout(
        repo.path(),
        home.path(),
        &["task", "finalize", task, "--dry-run"],
        "jigc task finalize --dry-run (migration)",
    );

    assert!(
        stdout.contains("promoted docs/changelog/changelog.md"),
        "the migration manifest lists the promoted canonical doc; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains("deleted CHANGELOG.md"),
        "the migration manifest lists the retired foreign deletion; stdout:\n{stdout}",
    );
    assert!(
        stdout.contains(".jigc/config"),
        "the migration manifest lists the jigc-tracked config layer; stdout:\n{stdout}",
    );
    assert!(
        !stdout.contains("scratch.txt"),
        "the narrowed migration prediction never sweeps user WIP; stdout:\n{stdout}",
    );

    // Still a dry-run: HEAD unchanged.
    assert_eq!(
        head_before,
        git(repo.path(), &["rev-parse", "HEAD"]),
        "a migration dry-run must not move HEAD",
    );
}

/// (v) T1 — the dry-run forecast splits the porcelain XY into **included** (X column, the
/// index the commit lands) vs **left_out** (Y column, unstaged/untracked WIP). Over a
/// staged task edit, an unrelated untracked file, an unrelated unstaged-modified tracked
/// file, and a staged-then-further-modified (`MM`) path, the included `manifest` carries the
/// staged edit and the X side of `MM` (plus the promoted ADR) while `left_out` names the
/// untracked file, the unstaged-tracked edit, and the Y side of `MM` — the `MM` path in
/// **both** sets. Drives the emitted `--format json` bytes the binary printed, not a rebuild.
#[test]
fn dry_run_splits_included_from_left_out() {
    let repo = TempDir::new("split");
    let home = TempDir::new("home");
    init_repo(repo.path());

    // Tracked files for the included / left-out / MM cases (committed so they are tracked).
    fs::write(repo.path().join("staged.rs"), "fn a() {}\n").expect("write staged.rs");
    fs::write(repo.path().join("unstaged.rs"), "fn b() {}\n").expect("write unstaged.rs");
    fs::write(repo.path().join("mm.rs"), "fn c() {}\n").expect("write mm.rs");
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-q", "-m", "seed tracked"]);

    let (task, slug) = seed_task_with_stray(repo.path(), home.path(), "split forecast");

    // A staged task edit — X column → included.
    fs::write(repo.path().join("staged.rs"), "fn a() { /* edit */ }\n").expect("edit staged.rs");
    git(repo.path(), &["add", "staged.rs"]);
    // An unrelated unstaged-modified tracked file — Y column → left-out.
    fs::write(repo.path().join("unstaged.rs"), "fn b() { /* wip */ }\n").expect("edit unstaged.rs");
    // A staged-then-further-modified (`MM`) path — X → included, Y → left-out.
    fs::write(repo.path().join("mm.rs"), "fn c() { /* staged */ }\n").expect("stage mm.rs");
    git(repo.path(), &["add", "mm.rs"]);
    fs::write(
        repo.path().join("mm.rs"),
        "fn c() { /* staged then more */ }\n",
    )
    .expect("re-edit mm.rs");
    // `scratch.txt` is the unrelated untracked stray the seed left (left-out).

    let dry = ok_stdout(
        repo.path(),
        home.path(),
        &["--format", "json", "task", "finalize", &task, "--dry-run"],
        "jigc task finalize --dry-run --format json",
    );
    let dry: serde_json::Value = serde_json::from_str(&dry).expect("dry-run stdout parses as JSON");
    let manifest = dry["manifest"].as_array().expect("manifest is an array");
    let left_out = dry["left_out"].as_array().expect("left_out is an array");
    let has = |arr: &[serde_json::Value], path: &str, kind: &str| {
        arr.iter().any(|e| e["path"] == path && e["kind"] == kind)
    };
    let names = |arr: &[serde_json::Value], path: &str| arr.iter().any(|e| e["path"] == path);

    // Included: the staged edit + the X side of `MM` + the promoted ADR.
    assert!(
        has(manifest, "staged.rs", "modified"),
        "included manifest carries the staged edit; manifest:\n{manifest:?}",
    );
    assert!(
        has(manifest, "mm.rs", "modified"),
        "included manifest carries the X side of MM; manifest:\n{manifest:?}",
    );
    assert!(
        has(manifest, &format!("docs/decisions/{slug}.md"), "promoted"),
        "included manifest carries the promoted ADR; manifest:\n{manifest:?}",
    );
    // Included must NOT carry the left-out-only paths.
    assert!(
        !names(manifest, "unstaged.rs"),
        "included manifest must NOT carry the unstaged-only edit; manifest:\n{manifest:?}",
    );
    assert!(
        !names(manifest, "scratch.txt"),
        "included manifest must NOT carry the untracked stray; manifest:\n{manifest:?}",
    );

    // Left-out: the untracked stray + the unstaged-tracked edit + the Y side of `MM`.
    assert!(
        has(left_out, "scratch.txt", "untracked"),
        "left_out names the untracked stray; left_out:\n{left_out:?}",
    );
    assert!(
        has(left_out, "unstaged.rs", "modified"),
        "left_out names the unstaged-tracked edit; left_out:\n{left_out:?}",
    );
    assert!(
        has(left_out, "mm.rs", "modified"),
        "left_out names the Y side of MM; left_out:\n{left_out:?}",
    );
    // The staged-only edit is not left out.
    assert!(
        !names(left_out, "staged.rs"),
        "left_out must NOT carry the staged-only edit; left_out:\n{left_out:?}",
    );

    // The `MM` path appears in BOTH sets — a kind cannot express "partially included".
    assert!(
        names(manifest, "mm.rs") && names(left_out, "mm.rs"),
        "the MM path is reported in both included and left_out",
    );
}
