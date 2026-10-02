//! Flow 15 — Half A: the deterministic dogfood CLI walk on a `/tmp` copy of the real
//! foreign project `gherrink-galey` (M12 Inc 2 / T1; `design/worked-examples.md` →
//! flow 15 Setup + The walk; `design/self-hosting.md` → The dogfood acceptance flow +
//! Verified friction; `roadmap.md` → M12 Inc 2 bullet 1).
//!
//! This is the **automatable, re-runnable half** of the flow-15 integration proof — the
//! part an independent validator can re-run. It drives the real binary, with the
//! **methodology pack** as the sole `JIGC_PACK_DIR`, over a **clean-baseline `/tmp`
//! copy** of the real `gherrink-galey` (a TypeScript pnpm monorepo, genuinely unlike
//! jigc), through the whole walk with **fixed inputs** — a stand-in TS edit standing in
//! for the agent's judgment work, NOT genuine agent judgment:
//!
//!   setup → commit setup → `start "<intent>"` mint+compose → assert `start --task <id>`
//!   recompose is BYTE-IDENTICAL to the mint capture → a fixed stand-in TS edit → the
//!   four fill verbs (`set-field type/scope --value`, `set-slot summary/body
//!   --from-file -`) → `task finalize <id>` lands EXACTLY ONE new git commit that is
//!   **code-only** (diff carries the stand-in edit, NOT `.jigc/tasks/`), **git-only**
//!   (no `cargo`/`rustc` — the finalize path is stack-free on a non-Rust project), and
//!   the **original `gherrink-galey` HEAD + working tree are unchanged**.
//!
//! THE GENUINE LIVE-AGENT RUN IS T2 — this fixed-input walk is the deterministic
//! substrate, never a substitute (the hollow-dogfood trap). A masking variant that only
//! asserts "the pack composes on the copy" is explicitly rejected: this asserts a task
//! ran through to one landed code-only commit on the foreign repo.
//!
//! ## Clean-baseline + copy strategy (the recorded doc-elaboration pin)
//!
//! The real `gherrink-galey` working tree carries pre-existing uncommitted edits and a
//! ~1.4 GB `node_modules` (the portability friction the toy-repo spike did not surface —
//! `DECISIONS.md` 2026-06-07 M12 Inc 2). So the walk:
//!   - **copies the source tree excluding `node_modules` and `.git`** — `finalize`
//!     itself never runs `pnpm`, so `node_modules` is unneeded for the deterministic
//!     walk (it would only be needed by the *agent's* `pnpm test`/`biome` gate, which is
//!     Half B); excluding it keeps the copy small (~5 MB);
//!   - **establishes a clean baseline** by `git init` over the copy and committing the
//!     pre-existing dirt as one `baseline` commit BEFORE setup;
//!   - **sequences the `setup` commit ahead of the work commit**, so the finalize work
//!     commit is code-only.
//!
//! ## Locating the source repo
//!
//! The source `gherrink-galey` is located via `$JIGC_DOGFOOD_GALEY` (override) or the
//! conventional sibling `$HOME/Projects/gherrink-galey`. When the real repo is genuinely
//! absent (a machine without the dogfood checkout), the walk **skips** with a visible
//! `eprintln` rather than failing — but on the dev/validator machine the foreign repo is
//! present and the full walk runs as the regression gate. The original is NEVER touched
//! (the run is on the `/tmp` copy); the test asserts the original's HEAD + tree are
//! unchanged by the run.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-flow15-dogfood-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`) — `CARGO_MANIFEST_DIR`
/// is `<root>/crates/cli`, so the pack tree is its `crates/cli/packs/methodology`.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
}

/// Locate the source `gherrink-galey` checkout: `$JIGC_DOGFOOD_GALEY` (override) or the
/// conventional sibling `$HOME/Projects/gherrink-galey`. Returns `None` if absent — the
/// dogfood checkout is not on this machine, so the walk skips visibly.
fn source_galey() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("JIGC_DOGFOOD_GALEY") {
        let p = PathBuf::from(p);
        return p.is_dir().then_some(p);
    }
    let home = std::env::var("HOME").ok()?;
    let p = PathBuf::from(home).join("Projects").join("gherrink-galey");
    p.is_dir().then_some(p)
}

/// Recursively copy `src` into `dst`, skipping any directory named in `skip` (matched on
/// the immediate entry name). `finalize` never runs `pnpm`, so `node_modules` is skipped
/// (heavy, unneeded for the deterministic walk); `.git` is skipped so the copy can be
/// re-baselined as a fresh repo without inheriting the original's history.
fn copy_tree_skipping(src: &Path, dst: &Path, skip: &[&str]) {
    fs::create_dir_all(dst).expect("create copy target dir");
    for entry in fs::read_dir(src).expect("read source tree").flatten() {
        let name = entry.file_name();
        if skip.iter().any(|s| name == std::ffi::OsStr::new(s)) {
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        let ty = entry.file_type().expect("entry file type");
        if ty.is_dir() {
            copy_tree_skipping(&from, &to, skip);
        } else if ty.is_symlink() {
            // Preserve a symlink as a symlink (don't chase it into node_modules etc.).
            let target = fs::read_link(&from).expect("read symlink");
            #[cfg(unix)]
            std::os::unix::fs::symlink(&target, &to).expect("recreate symlink");
        } else {
            fs::copy(&from, &to).expect("copy file");
        }
    }
}

/// Run `git` in `repo`, asserting success, returning trimmed stdout.
fn git(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {repo:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("utf-8")
        .trim()
        .to_string()
}

/// Run a `jigc` subcommand with `cwd = repo`, `$HOME = home`, `JIGC_PACK_DIR = pack`,
/// optionally piping `stdin`. Returns the full `Output`.
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
        crate::support::child_stdin::feed(&mut child, bytes);
    }
    child.wait_with_output().expect("wait for jigc")
}

/// Run a `jigc` subcommand and return its stdout, asserting exit 0 (stderr on failure).
fn jigc_ok(repo: &Path, home: &Path, pack: &Path, args: &[&str], stdin: Option<&[u8]>) -> String {
    let out = run_jigc(repo, home, pack, args, stdin);
    assert!(
        out.status.success(),
        "`jigc {}` must exit 0; stdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8(out.stdout).expect("utf-8 stdout")
}

#[test]
fn flow15_deterministic_dogfood_walk_lands_one_code_only_commit_on_a_galey_copy() {
    let Some(source) = source_galey() else {
        eprintln!(
            "SKIP flow15 dogfood walk: no `gherrink-galey` checkout found \
             (set $JIGC_DOGFOOD_GALEY or place it at $HOME/Projects/gherrink-galey). \
             This is the foreign-repo regression gate; it runs where the dogfood repo is present."
        );
        return;
    };

    // Record the ORIGINAL's HEAD + working-tree status up front — the run must leave both
    // untouched (the original is NEVER modified; the run is on the /tmp copy). We snapshot
    // HEAD and the porcelain status string and re-assert them after the whole walk.
    let original_head = git(&source, &["rev-parse", "HEAD"]);
    let original_status = git(&source, &["status", "--porcelain"]);

    let work = TempDir::new("work");
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();
    let copy = work.path().join("galey");

    // --- copy strategy: exclude `node_modules` (~1.4 GB, unneeded by finalize) + `.git`
    // (re-baselined fresh below). Keeps the copy ~5 MB. ---
    copy_tree_skipping(&source, &copy, &[".git", "node_modules"]);
    assert!(
        copy.join("package.json").is_file() && copy.join("pnpm-workspace.yaml").is_file(),
        "the /tmp copy must carry the real galey TS-monorepo files (package.json + \
         pnpm-workspace.yaml); the source at {source:?} is not a recognizable galey checkout",
    );
    assert!(
        !copy.join("node_modules").exists(),
        "the copy must EXCLUDE node_modules (the finalize walk never runs pnpm)",
    );

    // --- establish a clean baseline: a fresh repo committing the pre-existing dirt as one
    // `baseline` commit BEFORE setup (the copy's working tree carries the original's
    // uncommitted edits — the portability friction the toy-repo spike did not surface). ---
    git(&copy, &["init", "-q"]);
    git(&copy, &["config", "user.email", "dogfood@example.com"]);
    git(&copy, &["config", "user.name", "Dogfood"]);
    git(&copy, &["add", "-A"]);
    git(&copy, &["commit", "-q", "-m", "baseline"]);
    assert!(
        git(&copy, &["status", "--porcelain"]).is_empty(),
        "the clean baseline must leave a clean working tree before setup",
    );

    // --- step 0: `jigc setup` (start hard-fails without a `.jigc/config/` layer). Setup now
    // lands its OWN install commit AHEAD of the work commit (M26), so the install artifacts
    // are off the finalize work commit without a manual setup commit, and the working tree is
    // left clean. ---
    let _ = jigc_ok(&copy, home.path(), &pack, &["setup"], None);
    assert!(
        git(&copy, &["status", "--porcelain"]).is_empty(),
        "`jigc setup` must commit its own install and leave a clean working tree (M26)",
    );
    let commits_before_work = git(&copy, &["rev-list", "--count", "HEAD"]);

    // --- step 1: `jigc start "<intent>"` mints + composes the flat dev-task spine. ---
    let intent = "add a greeting helper to the utils package";
    let mint = jigc_ok(&copy, home.path(), &pack, &["start", intent], None);
    // The intent → slug derivation gives this task id; the composed spine must carry the
    // finalize Run line addressed to it (guards against an orient/error page masquerading).
    let task = "add-a-greeting-helper";
    let finalize_run = format!("jigc task finalize {task}");
    assert!(
        mint.contains("done-criterion") && mint.contains(&finalize_run),
        "the mint capture must be the composed dev-task spine addressed to {task}; got:\n{mint}",
    );

    // --- determinism: `start --task <id>` recompose is BYTE-IDENTICAL to the mint
    // capture (the original successful mint's stdout — NOT a re-mint, which fails
    // `already active`). Re-execute the recompose twice; all captures byte-identical. ---
    let recompose_a = jigc_ok(&copy, home.path(), &pack, &["start", "--task", task], None);
    let recompose_b = jigc_ok(&copy, home.path(), &pack, &["start", "--task", task], None);
    // The mint capture opens with the frontend's `task minted: <id>` header (M42) — an
    // invocation fact (this run minted the id), never view state, so the recompose (which
    // minted nothing) carries none (`workflow-dialect.md` → The `task minted:` header).
    // The determinism claim is about the *view*: peel the header off the mint capture and
    // everything below it must match byte-for-byte.
    let minted_view = mint
        .strip_prefix(&format!("task minted: {task}\n\n"))
        .unwrap_or_else(|| panic!("the mint announces the id it minted; got:\n{mint}"));
    assert!(
        !recompose_a.contains("task minted:"),
        "the recompose mints nothing — it announces no mint; got:\n{recompose_a}",
    );
    assert_eq!(
        minted_view, recompose_a,
        "the `--task` recompose must be BYTE-IDENTICAL to the original successful mint \
         capture (below the mint header)",
    );
    assert_eq!(
        recompose_a, recompose_b,
        "the recompose must be stable across repeated re-execution (not one lucky run)",
    );

    // --- step 2: a FIXED stand-in TS edit (standing in for the agent's judgment work —
    // the determinism half, not the judgment half). A new file in the core package. ---
    let edit_rel = "packages/core/src/greet.ts";
    let edit_path = copy.join(edit_rel);
    fs::write(
        &edit_path,
        "export function greet(name: string): string {\n  return `Hello, ${name}!`;\n}\n",
    )
    .expect("write the stand-in TS edit");
    // The agent stages its own edit (M30 G5) so the per-task IndexHonoring finalize
    // commits it (the narrowing no longer sweeps the unstaged tree).
    git(&copy, &["add", edit_rel]);

    // --- step 3: the four fill verbs — `set-field type/scope --value`, `set-slot
    // summary/body --from-file -` (the commit schema requires all four non-empty;
    // finalize RENDERS, it does not fill). ---
    let commit_addr = |leaf: &str| format!("commit:{task}#{leaf}");
    let _ = jigc_ok(
        &copy,
        home.path(),
        &pack,
        &["doc", "set-field", &commit_addr("type"), "--value", "feat"],
        None,
    );
    let _ = jigc_ok(
        &copy,
        home.path(),
        &pack,
        &["doc", "set-field", &commit_addr("scope"), "--value", "core"],
        None,
    );
    let _ = jigc_ok(
        &copy,
        home.path(),
        &pack,
        &[
            "doc",
            "set-slot",
            &commit_addr("summary"),
            "--from-file",
            "-",
        ],
        Some(b"add greet() helper to core\n"),
    );
    let _ = jigc_ok(
        &copy,
        home.path(),
        &pack,
        &["doc", "set-slot", &commit_addr("body"), "--from-file", "-"],
        Some(b"A small greeting helper for the core package.\n"),
    );

    // --- step 4: `jigc task finalize <id>` — validates, renders the now-filled commit
    // doc to the git message, lands the work commit. ---
    let log_before: u32 = git(&copy, &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    let finalize = run_jigc(&copy, home.path(), &pack, &["task", "finalize", task], None);
    assert!(
        finalize.status.success(),
        "`jigc task finalize {task}` must exit 0; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr),
    );

    // EXACTLY ONE new commit.
    let log_after: u32 = git(&copy, &["rev-list", "--count", "HEAD"])
        .parse()
        .unwrap();
    assert_eq!(
        log_after,
        log_before + 1,
        "finalize must land EXACTLY ONE new git commit (before={log_before}, after={log_after})",
    );
    // ... and exactly one commit was added since the setup commit (no stray work commits).
    assert_eq!(
        commits_before_work.parse::<u32>().unwrap() + 1,
        log_after,
        "exactly one work commit must sit atop the sequenced setup commit",
    );

    // The commit message equals the rendered commit doc (the commit sink is the git
    // message — git-only, no repo-file payload for the commit doctype itself).
    let message = git(&copy, &["log", "-1", "--format=%B"]);
    assert_eq!(
        message.trim_end(),
        "feat(core): add greet() helper to core\n\nA small greeting helper for the core package.",
        "the work commit message must equal the rendered commit doc; got:\n{message}",
    );

    // CODE-ONLY: the commit diff carries the stand-in TS edit and does NOT carry any
    // `.jigc/tasks/` working-area file (the task scratch area is removed, never committed).
    let committed_files = git(&copy, &["show", "--name-only", "--format=", "HEAD"]);
    let committed: Vec<&str> = committed_files.lines().collect();
    assert!(
        committed.contains(&edit_rel),
        "the work commit must carry the stand-in TS edit {edit_rel}; committed files:\n{committed_files}",
    );
    assert!(
        !committed.iter().any(|f| f.starts_with(".jigc/tasks/")),
        "the work commit must NOT carry any `.jigc/tasks/` working-area file (code-only); \
         committed files:\n{committed_files}",
    );
    // The task working area is gone (cleaned at finalize), never left to leak into a later
    // commit.
    assert!(
        !copy.join(".jigc").join("tasks").join(task).exists(),
        "finalize must remove the `.jigc/tasks/{task}/` working area",
    );

    // GIT-ONLY: the finalize path invoked no Rust toolchain. The methodology pack ships no
    // gate command and the finalize render is stack-free, so neither `cargo` nor `rustc`
    // may appear anywhere in finalize's emitted output (a foreign non-Rust project).
    let finalize_out = format!(
        "{}{}",
        String::from_utf8_lossy(&finalize.stdout),
        String::from_utf8_lossy(&finalize.stderr),
    );
    assert!(
        !finalize_out.contains("cargo") && !finalize_out.contains("rustc"),
        "the finalize path must be git-only on a non-Rust project — no `cargo`/`rustc`; \
         got:\n{finalize_out}",
    );

    // --- the ORIGINAL `gherrink-galey` is unchanged by the whole run (never modified). ---
    assert_eq!(
        git(&source, &["rev-parse", "HEAD"]),
        original_head,
        "the original gherrink-galey HEAD must be unchanged by the dogfood run",
    );
    assert_eq!(
        git(&source, &["status", "--porcelain"]),
        original_status,
        "the original gherrink-galey working tree must be unchanged by the dogfood run",
    );
}
