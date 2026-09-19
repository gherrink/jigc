//! M52 Increment 8 / T1 — **every `milestone` door refuses a repository with no
//! `jigc setup`.**
//!
//! ## The class this closes
//!
//! `design/team-ready-state.md` → *Engine capability 2 (read-back)* settles that the
//! committed `.md` milestone record is the **source of truth** and
//! `.jigc/milestones/<id>/{base,tasks}.json` a rebuildable cache. A repository with no
//! `jigc setup` has no `.jigc/config/` layer, no `docs-root`, and nothing tracked — so a
//! milestone minted there exists **only** in the gitignored workbench, which is the
//! precise inverse of that settlement: no clone sees it and no fresh clone can continue
//! it.
//!
//! Driven at `HEAD~` over `dev/jigc-rig bare` (the baseline's LD-3,
//! `completions/artifacts/M52/baseline-contracts.md` §4): `create`, `add-task`,
//! `list-tasks`, `provision`, `join` and `discard` ran at **exit 0** —
//! `provision` registering a real detached git worktree in `.git/worktrees/` —
//! `finalize` reached a genuine `milestone.zero-contribution` adjudication at exit 3,
//! `add-from-spec` refused with `store.not-found` (it had resolved a docs-root path in an
//! unconfigured repo rather than asking), and only `execute` gave the shipped answer.
//! `git status --porcelain` afterwards read `?? .jigc/` and `git log` held one commit.
//!
//! **One of nine doors knew the rule.** That is M45's complete-fix shape verbatim — *a
//! fence applied where its wave pointed is not applied at all*
//! ([pinning.md](../../../implementation/pinning.md)) — so the subject here is the
//! **door class**, not the eight cells an instrument happened to name.
//!
//! ## What the answer is, and why it is the `{error}` arm
//!
//! [`cli::locate::not_set_up`] — the shipped constructor every other door requiring the
//! project layer already reaches for in this state (21 of the 47 leaves, measured at
//! this wave's baseline, before this suite), a bare `anyhow` carrying a mechanically
//! rendered `jigc setup` route. The milestone doors give **the same answer**, on
//! `Reject::Error`, as a
//! deliberate consistency with the shipped surface rather than an omission
//! (`completions/artifacts/M52/settle-record.md` §13). Re-coding not-set-up as a
//! registered finding at all 47 leaves is scope this wave does not carry, and eight doors
//! answering one way while 21 siblings answer another would be the law-1 divergence the
//! wave exists to remove.
//!
//! ## The subject is derived, never remembered
//!
//! The doors come from [`cli::cli::VERB_KINDS`] — already fenced ⇔ against the real clap
//! tree (`cli_parse::every_leaf_verb_is_classified`) — filtered to the `milestone`
//! family, and each door's argv tail from the shared
//! [`crate::support::leaf_argv::MINIMAL_ARGV`] table, which the same fence keeps total.
//! A tenth milestone leaf therefore joins this sweep the day it is added, and cannot ship
//! without an author deciding what it says in a repository jigc was never installed into.
//!
//! Per cell, through the **real binary**, from a git repository with one commit and no
//! `jigc setup` anywhere:
//!
//! 1. the run exits **1** — never clap's 2 (the argv parses), never a debug-fence panic's
//!    101, and never the 0 or 3 six of them shipped;
//! 2. the `--format json` `{error}` arm carries the shared constructor's text
//!    **byte-for-byte**, compared against [`cli::locate::not_set_up`] rather than a copy
//!    typed here;
//! 3. it carries the `jigc setup` route;
//! 4. and `git status --porcelain` is **empty** afterwards — no `.jigc/` minted, no
//!    worktree registered, nothing written on the way to the refusal.
//!
//! Arm 4 is the load-bearing half: an answer printed *after* the workbench was written
//! would leave exactly the un-clonable state this task exists to prevent.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

use cli::cli::VERB_KINDS;

use crate::support::leaf_argv;
use crate::support::trial_corpus::unique_root;

/// A git repository with one commit and **no `jigc setup`** — `dev/jigc-rig bare`'s
/// state, built here in-process because `support::trial_corpus::State::ALL` deliberately
/// does not carry it (`dev_rig_parity.rs`'s `RIG_ONLY_STATES`: every `State` consumer
/// presupposes a set-up corpus).
///
/// One per cell rather than one for the suite: arm 4 asserts the worktree is clean
/// afterwards, and a shared fixture would let the first door that writes mask every
/// later door's arm-4 evidence.
struct BareRepo {
    root: PathBuf,
}

impl BareRepo {
    fn build(label: &str) -> Self {
        let root = unique_root(&format!("bare-{label}"));
        fs::create_dir_all(root.join("repo")).expect("create the bare repo dir");
        fs::create_dir_all(root.join("home")).expect("create the bare home dir");
        let bare = BareRepo { root };
        bare.git(&["init", "-q"]);
        bare.git(&["config", "user.email", "bare@example.com"]);
        bare.git(&["config", "user.name", "Bare Repo"]);
        bare.git(&["config", "commit.gpgsign", "false"]);
        fs::write(bare.repo().join("README.md"), "bare repo\n").expect("write README.md");
        bare.git(&["add", "."]);
        bare.git(&["commit", "-q", "-m", "initial"]);
        assert!(
            !bare.repo().join(".jigc").exists(),
            "the `bare` fixture must have no `.jigc/` — this cell would otherwise be \
             driven against a set-up corpus",
        );
        bare
    }

    fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(self.repo())
            .args(args)
            .env("HOME", self.home())
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr),
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// `git status --porcelain` — empty when nothing was written on the way to a refusal.
    fn porcelain(&self) -> String {
        self.git(&["status", "--porcelain"])
    }

    fn drive(&self, argv: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .arg("--format")
            .arg("json")
            .args(argv)
            .current_dir(self.repo())
            .env("HOME", self.home())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("spawn jigc")
    }
}

impl Drop for BareRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Every `milestone` leaf the clap tree carries, with the argv tail that reaches its
/// preconditions — derived from [`VERB_KINDS`] and the shared argv table, so the sweep's
/// subject grows with the surface.
fn milestone_cells() -> Vec<(&'static [&'static str], &'static [&'static str])> {
    leaf_argv::assert_covers_every_leaf_verb();
    let cells: Vec<_> = VERB_KINDS
        .iter()
        .filter(|(path, _)| path.first() == Some(&"milestone"))
        .map(|(path, _)| {
            let tail = leaf_argv::MINIMAL_ARGV
                .iter()
                .find(|(known, _)| known == path)
                .unwrap_or_else(|| panic!("`jigc {}` has no argv arm", path.join(" ")))
                .1;
            (*path, tail)
        })
        .collect();
    assert!(
        !cells.is_empty(),
        "the `milestone` family is empty — this sweep would pass vacuously",
    );
    cells
}

/// The `{error}` string a `--format json` refusal carries, or a panic naming what came
/// back instead.
fn error_arm(out: &Output, label: &str) -> String {
    let stderr = String::from_utf8_lossy(&out.stderr);
    let value: serde_json::Value = serde_json::from_str(stderr.trim()).unwrap_or_else(|err| {
        panic!(
            "`jigc {label}` must answer on the `--format json` envelope; \
             parsing stderr failed ({err}).\nstderr: {stderr}\nstdout: {}",
            String::from_utf8_lossy(&out.stdout),
        )
    });
    value
        .get("error")
        .and_then(|e| e.as_str())
        .unwrap_or_else(|| panic!("`jigc {label}` must answer on the `{{error}}` arm; got {value}"))
        .to_string()
}

#[test]
fn every_milestone_door_refuses_a_repository_with_no_jigc_setup() {
    let route = engine::finding::Route::mechanical(["jigc", "setup"], "").to_string();

    let cells = milestone_cells();
    let mut driven = 0usize;
    for (path, tail) in &cells {
        let label = path.join(" ");
        let bare = BareRepo::build(&label.replace(' ', "-"));
        // The constructor is asked about **this cell's own** absent layer: since M52
        // Inc 8 / T2 it tells *absent* from *unreadable* by `stat`-ing the path, so the
        // expected bytes are derived from the fixture rather than from a path that
        // happens to be missing (`tests/unreadable_project_layer.rs`).
        let expected = format!(
            "{:#}",
            cli::locate::not_set_up(&bare.repo().join(".jigc").join("config")),
        );
        let mut argv: Vec<&str> = path.to_vec();
        argv.extend_from_slice(tail);
        let out = bare.drive(&argv);

        assert_eq!(
            out.status.code(),
            Some(1),
            "`jigc {label}` over a repository with no `jigc setup` exits 1.\nstderr: {}\nstdout: {}",
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout),
        );
        assert_eq!(
            error_arm(&out, &label),
            expected,
            "`jigc {label}` gives the shared not-set-up answer byte-for-byte",
        );
        assert!(
            expected.contains(&route),
            "the shared constructor carries the `jigc setup` route; it reads: {expected}",
        );
        assert_eq!(
            bare.porcelain(),
            "",
            "`jigc {label}` wrote nothing on the way to the refusal — no `.jigc/` minted, \
             no worktree registered",
        );
        assert!(
            !bare.repo().join(".git").join("worktrees").exists(),
            "`jigc {label}` registered a git worktree in a repository it had just called \
             not set up",
        );
        driven += 1;
    }
    assert_eq!(
        driven,
        cells.len(),
        "every registered `milestone` leaf is driven — a row with no cell is a hard \
         failure, never a skip",
    );
}
