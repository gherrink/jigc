//! M47 Inc 1 T1 — the **free-text item mint** at slug-rule generation 3, through
//! the real binary.
//!
//! `jigc doc add-item --title "<free text>"` is the surface where the word-aware
//! edge-stopword drop is *reachable* by an agent: the anchor it mints is frozen at
//! creation, `add-item` has no `--slug`, and `retitle-item` explicitly freezes the
//! anchor — so under generation 2 an item titled `On-call handoff artifact` was
//! addressable **only** as `call-handoff-artifact`, with no shipped command in any
//! sequence able to reach the right identity (`DECISIONS.md` → 2026-07-26 M47
//! planning: the Settle → Decision 2).
//!
//! The engine's own axis is swept in `engine::slug` (leading · trailing ·
//! char-cap-exposed × whole-source-word ⇒ dropped, hyphen-glued ⇒ kept). What is
//! proven **here** is that the emitted artifact carries it: the assertions run over
//! the address `add-item` prints on stdout and the `{#id}` anchor in the staged
//! bytes on disk — never a hand-built expectation of what the mint "should" do.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let unique = format!(
            "jigc-item-slug-{tag}-{}-{:?}",
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

/// The on-disk methodology pack home (`<root>/crates/cli/packs/methodology`) — it carries
/// `deferral-ledger`, whose `entries` repeatable is keyed by a **free-text** title.
fn methodology_pack_tree() -> PathBuf {
    Path::new(cli::pack_path!(methodology)).to_path_buf()
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

/// The staged singleton body for `<type>:<type>.md` in the task working area.
fn staged_singleton(repo: &Path, task: &str, ty: &str) -> String {
    let path = repo
        .join(".jigc")
        .join("tasks")
        .join(task)
        .join("docs")
        .join(format!("{ty}:{ty}.md"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read staged {path:?}: {e}"))
}

/// The item mint iterates the generation-3 axis over the **emitted** artifact: the
/// address `add-item` prints and the `{#id}` anchor it writes into the staged doc.
///
/// Three titles, one per axis cell — a hyphen-glued stopword at the **leading**
/// edge, one at the **trailing** edge, and a whole separator-delimited one at the
/// leading edge (which still drops). Under generation 2 the first two minted
/// `call-handoff-artifact` and `telemetry-consent-opt`, and every one of them is
/// frozen at creation.
#[test]
fn free_text_item_titles_mint_word_aware_anchors() {
    let repo = TempDir::new("repo");
    let home = TempDir::new("home");
    let pack = methodology_pack_tree();
    init_repo(repo.path());

    let setup = run_jigc(repo.path(), home.path(), &pack, &["setup"]);
    assert!(
        setup.status.success(),
        "`JIGC_PACK_DIR=<methodology> jigc setup` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&setup.stderr),
    );

    // The `planning` workflow's create-gate admits `deferral-ledger`.
    let start = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &["start", "--workflow", "planning", "plan the next milestone"],
    );
    assert!(
        start.status.success(),
        "`jigc start --workflow planning` must mint the task; stderr:\n{}",
        String::from_utf8_lossy(&start.stderr),
    );
    let task = "plan-the-next-milestone";

    let create = run_jigc(
        repo.path(),
        home.path(),
        &pack,
        &[
            "doc",
            "create",
            "deferral-ledger",
            "--title",
            "Deferral Ledger",
        ],
    );
    assert!(
        create.status.success(),
        "`jigc doc create deferral-ledger` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&create.stderr),
    );

    let cases = [
        // (axis cell, --title, minted id)
        (
            "leading edge, hyphen-glued component",
            "On-call handoff artifact",
            "on-call-handoff-artifact",
        ),
        (
            "trailing edge, hyphen-glued component",
            "Telemetry consent opt-in",
            "telemetry-consent-opt-in",
        ),
        (
            "leading edge, whole separator-delimited word",
            "The dashboard rollout",
            "dashboard-rollout",
        ),
    ];

    for (cell, title, minted) in cases {
        let add = run_jigc(
            repo.path(),
            home.path(),
            &pack,
            &[
                "doc",
                "add-item",
                "deferral-ledger:deferral-ledger#entries",
                "--title",
                title,
            ],
        );
        assert!(
            add.status.success(),
            "[{cell}] `jigc doc add-item --title {title:?}` must exit 0; stderr:\n{}",
            String::from_utf8_lossy(&add.stderr),
        );
        // The emitted address IS the contract a driver reads back and addresses the
        // item by — assert on the bytes, not on a re-slug in test code.
        assert_eq!(
            String::from_utf8_lossy(&add.stdout).trim_end_matches('\n'),
            format!("deferral-ledger:deferral-ledger#entries/{minted}"),
            "[{cell}] the emitted item address carries the minted id",
        );
    }

    // …and the frozen `{#id}` anchor is in the staged bytes on disk, under the
    // title the agent authored.
    let staged = staged_singleton(repo.path(), task, "deferral-ledger");
    for (cell, title, minted) in cases {
        let anchored = staged
            .lines()
            .find(|line| line.contains(&format!("{{#{minted}}}")))
            .unwrap_or_else(|| {
                panic!("[{cell}] no staged heading carries `{{#{minted}}}`; staged:\n{staged}")
            });
        assert!(
            anchored.contains(title),
            "[{cell}] the `{{#{minted}}}` anchor must sit on the `{title}` heading; \
             got {anchored:?}",
        );
    }
}
