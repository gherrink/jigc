//! A destroying door whose leftover probe **cannot run** refuses with its own code, its own
//! `at:` and a route — never a bare `anyhow` chain (M49 completion audit).
//!
//! `cli::milestone::probe_leftover`'s recorded contract already said it: *"an unreadable
//! directory or an unreadable `git status` is an `Err` — the door refuses on it, because
//! removing on an unverified probe is the defect this guard closes."* The doors did refuse.
//! They refused as a bare error, which the route floor is blind to **by construction** —
//! `engine::finding::is_route_exempt` keys on a finding *code*, and a bare error has none:
//!
//! ```text
//! $ rm -rf .jigc/worktrees/dev-one && echo junk > .jigc/worktrees/dev-one
//! $ jigc milestone provision second-wave
//!   -> rc 1
//!      could not read the leftover directory "…/.jigc/worktrees/dev-one": Not a directory
//! ```
//!
//! No `blocking · <code>`, no `at:`, no `route:` — while the adjacent cell (a *directory*
//! holding a file) answered correctly with `milestone.leftover-holds-work` and a `--force`
//! route. M48's destroying-door work gave `jigc uninstall` this fail-closed half
//! (`setup::unverified_worktrees_finding`); its two milestone siblings shipped without it.
//!
//! **The axis is the doors, read code-side.** `cli::milestone::DESTROYING_DOORS` enumerates
//! the four verbs that remove a worktree-shaped path, and `DestroyingDoor::code` is the
//! table's own refuse-vs-narrate discriminator — so this suite iterates the members that
//! refuse and drives each at the same planted state. A fifth refusing door cannot be added
//! without landing here.
//!
//! **The worse face the reported repro did not reach**: `jigc uninstall` did not merely
//! answer badly, it *destroyed the bytes at exit 0*. Its subject was
//! `fanout_worktree_paths`, which filtered on `path.is_dir()` — a claim about shape where the
//! door's question is about bytes — so a plain file under `.jigc/worktrees/` was invisible to
//! the guard **and** to the loss narration, while `remove_dir_all(.jigc/)` took it exactly as
//! hard as a worktree. The last two arms pin both halves: the refusal, and — under the
//! operator's `--force` consent — the narration that names the file before it goes.
//!
//! Drives the REAL binary; the planted bytes surviving byte-intact is the load-bearing half.

use cli::milestone::{DESTROYING_DOORS, DestroyingDoor};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The bytes planted as the leftover — a refusal must leave them exactly this.
const PRECIOUS: &str = "precious, uncommitted, in no object DB\n";

/// The milestone the fixture mints, and the sub-task whose worktree path is squatted.
const MILESTONE: &str = "cache-rework";
const SUB_TASK: &str = "area-zed";

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-leftover-probe-{tag}-{}-{:?}",
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

/// Run `git <args>` in `cwd`, asserting success.
fn git_ok(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A repo carrying `milestone:cache-rework` with one sub-task, and a **non-directory**
/// leftover squatting that sub-task's worktree path — the shape no `LeftoverVerdict` can
/// classify, because `classify_leftover` cannot even `cd` into it.
struct Fixture {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
    leftover: PathBuf,
}

impl Fixture {
    fn plant() -> Self {
        let root = TempDir::new("root");
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo");
        git_ok(&repo, &["init", "-q"]);
        git_ok(&repo, &["config", "user.email", "test@example.com"]);
        git_ok(&repo, &["config", "user.name", "Test"]);
        git_ok(&repo, &["config", "commit.gpgsign", "false"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        git_ok(&repo, &["add", "."]);
        git_ok(&repo, &["commit", "-q", "-m", "initial"]);

        let home = TempDir::new("home");
        let fx = Fixture {
            _root: root,
            home,
            repo,
            leftover: PathBuf::new(),
        };
        fx.jigc_ok(&["milestone", "create", "Cache rework"]);
        fx.jigc_ok(&["milestone", "add-task", MILESTONE, "Area zed"]);

        let worktrees = fx.repo.join(".jigc").join("worktrees");
        fs::create_dir_all(&worktrees).expect("mk .jigc/worktrees/");
        let leftover = worktrees.join(SUB_TASK);
        fs::write(&leftover, PRECIOUS).expect("plant the non-directory leftover");
        Fixture { leftover, ..fx }
    }

    fn run(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(&self.repo)
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    fn jigc_ok(&self, args: &[&str]) {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "`jigc {}` must exit 0; stderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr),
        );
    }

    /// The planted bytes, exactly as the door left them.
    fn planted_bytes(&self) -> String {
        fs::read_to_string(&self.leftover).unwrap_or_else(|e| {
            panic!(
                "the leftover at {:?} must survive the refusal: {e}",
                self.leftover,
            )
        })
    }
}

/// The argv that stands at `door`, **derived from the door's own `verb`** rather than
/// hand-listed: strip the binary name, and give a `milestone` verb the milestone id it takes.
fn argv_at(door: &DestroyingDoor) -> Vec<String> {
    let mut argv: Vec<String> = door
        .verb
        .split_whitespace()
        .skip(1)
        .map(str::to_owned)
        .collect();
    if argv.first().map(String::as_str) == Some("milestone") {
        argv.push(MILESTONE.to_owned());
    }
    argv
}

#[test]
fn every_refusing_door_answers_an_unprobeable_leftover_with_a_code_and_a_route() {
    let refusing: Vec<&DestroyingDoor> = DESTROYING_DOORS
        .into_iter()
        .filter(|door| door.code.is_some())
        .collect();
    assert!(
        !refusing.is_empty(),
        "the door table must carry at least one refusing member",
    );

    for door in refusing {
        let code = door.code.expect("filtered to the refusing members");
        // A fresh fixture per door: a door that refuses must be the only thing that touched
        // this state, so no earlier door's run can be credited with the survival below.
        let fx = Fixture::plant();
        let argv = argv_at(door);
        let args: Vec<&str> = argv.iter().map(String::as_str).collect();
        let out = fx.run(&args);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

        assert!(
            !out.status.success(),
            "[{}] a door must refuse a leftover its probe cannot read; got {:?}\nstdout:\n{}\nstderr:\n{stderr}",
            door.verb,
            out.status,
            String::from_utf8_lossy(&out.stdout),
        );
        assert!(
            stderr.contains(&format!("blocking · {code}")),
            "[{}] the refusal carries the door's own code `{code}`, so a driver can tell \
             which door refused; got:\n{stderr}",
            door.verb,
        );
        assert!(
            stderr.contains("route:"),
            "[{}] the refusal carries a route — the route floor's whole point; got:\n{stderr}",
            door.verb,
        );
        // The target obligation, in the form the contract declares for this code: a finding
        // outside the declared-singleton exemption must name what it is about.
        if !engine::finding::is_declared_singleton(code) {
            assert!(
                stderr.contains("at:") && stderr.contains(SUB_TASK),
                "[{}] a non-singleton refusal names the path it is about; got:\n{stderr}",
                door.verb,
            );
        }
        assert_eq!(
            fx.planted_bytes(),
            PRECIOUS,
            "[{}] a refusal must leave the planted bytes byte-intact",
            door.verb,
        );
    }
}

/// The face the reported repro did not reach: `jigc uninstall` **destroyed** the file at exit
/// 0, because its subject filtered on `path.is_dir()`. The refusal above proves the guard now
/// sees it; this proves the bytes were genuinely at risk, by driving the same door with the
/// consent that skips the guard and asserting the narration names the file **before** it goes.
#[test]
fn the_forced_teardown_names_the_leftover_file_before_taking_it() {
    let fx = Fixture::plant();
    let out = fx.run(&["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "`jigc uninstall --force` is the operator's consent and must land; stderr:\n{stderr}",
    );
    assert!(
        !fx.leftover.exists(),
        "the forced teardown does take the leftover — which is exactly why it must name it",
    );
    assert!(
        stderr.contains("warning:") && stderr.contains(SUB_TASK),
        "law 1: a door that destroys the file must have named it first; got:\n{stderr}",
    );
}
