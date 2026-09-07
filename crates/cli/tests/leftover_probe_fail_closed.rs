//! A destroying door answers **every** leftover shape under `.jigc/worktrees/` honestly: it
//! refuses with its own code, its own `at:`, a route that fits what it found and names the
//! consent — or it performs the removal it narrated. Never a narrated destruction that did
//! not happen, and never one sibling's answer swallowing another's (M50 Increment 12 / T2;
//! RC-m50 W-2, N9, N8).
//!
//! `cli::milestone::probe_leftover`'s recorded contract already said the fail-closed half:
//! *"an unreadable directory or an unreadable `git status` is an `Err` — the door refuses on
//! it, because removing on an unverified probe is the defect this guard closes."* The doors
//! did refuse. **How** they refused is what three defects shared, and all three sit on one
//! subject — *a leftover that is not a directory* — because they share the probe, the
//! narrator and the removal:
//!
//! - **N9, the masking.** The probe's failure propagated with `?`, so the FIRST unreadable
//!   path ended the walk: `jigc uninstall` over a directory holding work *and* a file
//!   answered only for the file and never named the directory at all. A door that names one
//!   of the two things it would destroy is the law-1 half-truth
//!   (`design/surface-contract.md`), and it is the same `?` at all three refusing doors.
//! - **W-2, the route that fits nothing.** That refusal routed *"make sure `git` is on PATH
//!   and the repository is readable"* over a plain file — nothing about the state it
//!   described — and never named `--force`, the consent its own sibling refusal names one
//!   screen away.
//! - **N8, the narrated destruction that did not happen.** `jigc milestone provision --force`
//!   printed `removing the leftover file … they are not recoverable`, then `remove_dir_all`
//!   failed with `Not a directory (os error 20)`, the bytes stayed, and the block routed at
//!   `jigc milestone provision <id> --force` — **the argv that had just failed**. 3/3
//!   identical runs. The narrator knew the shape since M49; the removal did not.
//!
//! **The axis is (refusing door) × (leftover shape) × (consent), each side read code-side or
//! enumerated here.** `cli::milestone::DESTROYING_DOORS` enumerates the four verbs that
//! remove a worktree-shaped path and `DestroyingDoor::code` is the table's own
//! refuse-vs-narrate discriminator, so a fifth refusing door cannot be added without landing
//! here. [`Shape`] carries the three plantable shapes — and `Both` is the cell that was red,
//! because masking is only visible where there is a sibling to mask. The consent axis is the
//! one N8 lived in and no cell of this suite drove before.
//!
//! Every cell asserts, on the **planted bytes**:
//!
//! 1. narrated ⇔ removed, per planted path — and a refusal leaves every one byte-intact;
//! 2. every planted leftover appears in the refusal;
//! 3. the refusal names `--force`;
//! 4. the route does not loop: every concrete consent command it names, run **verbatim**,
//!    does not reproduce the same `(code, target)`.
//!
//! Drives the REAL binary; the emitted refusal and the surviving bytes are the contract, not
//! a reconstructed equivalent.

use cli::milestone::{DESTROYING_DOORS, DestroyingDoor};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The bytes planted as every leftover — a refusal must leave them exactly this.
const PRECIOUS: &str = "precious, uncommitted, in no object DB\n";

/// The milestone the fixture mints.
const MILESTONE: &str = "cache-rework";
/// The sub-task whose worktree path carries the **directory**-shaped leftover.
const SUB_DIR: &str = "area-yak";
/// The sub-task whose worktree path carries the **file**-shaped leftover — the shape no
/// `LeftoverVerdict` can classify, because git cannot be run inside a file.
const SUB_FILE: &str = "area-zed";

/// The shape axis: what sits at a sub-task's worktree path when the door arrives.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// A directory holding one untracked file — the cell that always answered correctly.
    Directory,
    /// A plain file at the worktree path — the reported cell.
    File,
    /// Both at once, at two different sub-tasks' paths — the cell that makes masking
    /// visible, and the one nothing drove before.
    Both,
}

impl Shape {
    const ALL: [Shape; 3] = [Shape::Directory, Shape::File, Shape::Both];

    /// The sub-task ids this shape plants a leftover at.
    fn planted(self) -> &'static [&'static str] {
        match self {
            Shape::Directory => &[SUB_DIR],
            Shape::File => &[SUB_FILE],
            Shape::Both => &[SUB_DIR, SUB_FILE],
        }
    }
}

/// The consent axis — `--force` is the one word past every one of these guards.
const CONSENT: [bool; 2] = [false, true];

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

/// A repo carrying `milestone:cache-rework` with two sub-tasks, and the leftovers one
/// [`Shape`] plants squatting their worktree paths.
struct Fixture {
    _root: TempDir,
    home: TempDir,
    repo: PathBuf,
}

impl Fixture {
    fn plant(shape: Shape) -> Self {
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

        let fx = Fixture {
            _root: root,
            home: TempDir::new("home"),
            repo,
        };
        fx.jigc_ok(&["milestone", "create", "Cache rework"]);
        fx.jigc_ok(&["milestone", "add-task", MILESTONE, "Area zed"]);
        fx.jigc_ok(&["milestone", "add-task", MILESTONE, "Area yak"]);

        let worktrees = fx.repo.join(".jigc").join("worktrees");
        fs::create_dir_all(&worktrees).expect("mk .jigc/worktrees/");
        for sub in shape.planted() {
            let witness = fx.witness(sub);
            if let Some(parent) = witness.parent() {
                fs::create_dir_all(parent).expect("mk the leftover's parent");
            }
            fs::write(&witness, PRECIOUS).expect("plant the leftover");
        }
        fx
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

    /// The path holding the planted bytes for `sub` — the leftover file itself, or the one
    /// untracked file inside the leftover directory. **Not the worktree path**: a `--force`
    /// run clears the path and `git worktree add`s a fresh checkout at it, so the path
    /// existing again says nothing about whether the bytes survived.
    fn witness(&self, sub: &str) -> PathBuf {
        let at = self.repo.join(".jigc").join("worktrees").join(sub);
        if sub == SUB_DIR {
            at.join("keep.txt")
        } else {
            at
        }
    }

    /// Whether the planted bytes are still there, byte-intact.
    fn bytes_survive(&self, sub: &str) -> bool {
        fs::read_to_string(self.witness(sub)).ok().as_deref() == Some(PRECIOUS)
    }

    /// The worktree path as every surface prints it — repo-relative.
    fn printed(&self, sub: &str) -> String {
        format!(".jigc/worktrees/{sub}")
    }
}

/// Whether `stderr` narrated a removal **of** `printed` — the law-1 line every destroying
/// door prints through `cli::milestone::narrate_removal` before it takes anything.
fn narrated(stderr: &str, printed: &str) -> bool {
    stderr
        .lines()
        .any(|line| line.starts_with("warning: removing the ") && line.contains(printed))
}

/// The argv that stands at `door`, **derived from the door's own `verb`** rather than
/// hand-listed: strip the binary name, give a `milestone` verb the milestone id it takes,
/// and append the consent when the cell carries it.
fn argv_at(door: &DestroyingDoor, force: bool) -> Vec<String> {
    let mut argv: Vec<String> = door
        .verb
        .split_whitespace()
        .skip(1)
        .map(str::to_owned)
        .collect();
    if argv.first().map(String::as_str) == Some("milestone") {
        argv.push(MILESTONE.to_owned());
    }
    if force {
        argv.push("--force".to_owned());
    }
    argv
}

/// The `at:` locus the refusal named, if it named one (a declared-singleton finding does
/// not).
fn at_target(stderr: &str) -> Option<String> {
    stderr
        .lines()
        .find_map(|line| line.trim_start().strip_prefix("at: "))
        .map(|at| at.trim().to_owned())
}

/// Every **concrete consent command** the route names — the commands a reader can run
/// unchanged, which is what "the route, run verbatim" means:
///
/// - the mechanical route's own argv (a mechanical route renders as a leading backticked
///   span by construction, `engine::finding::Route::mechanical`), and
/// - every backticked `jigc … --force` span, whatever the route kind: `--force` is
///   unconditional consent, so a route that names it is promising it works.
///
/// A *"then re-run `jigc <door>`"* span is deliberately **not** one of these — it is
/// conditional on the human action the route asked for first, and running it without taking
/// that action is expected to refuse again.
fn consent_commands(stderr: &str) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in stderr.lines() {
        let Some((_, after)) = line.split_once("route:") else {
            continue;
        };
        let after = after.trim_start();
        let mechanical = after.starts_with('`');
        for (i, span) in after.split('`').enumerate() {
            // Odd indices are the insides of the backtick pairs.
            if i % 2 == 0 {
                continue;
            }
            let concrete = span.starts_with("jigc ") && !span.contains('<');
            if concrete && (span.contains("--force") || (mechanical && i == 1)) {
                let argv: Vec<String> =
                    span.split_whitespace().skip(1).map(str::to_owned).collect();
                if !out.contains(&argv) {
                    out.push(argv);
                }
            }
        }
    }
    out
}

#[test]
fn every_refusing_door_answers_every_leftover_shape_and_never_narrates_a_removal_it_did_not_make() {
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
        for shape in Shape::ALL {
            for force in CONSENT {
                // A fresh fixture per cell: the door under test must be the only thing that
                // touched this state, so no earlier run can be credited with the survival —
                // or the loss — below.
                let fx = Fixture::plant(shape);
                let cell = format!("{} × {shape:?} × force={force}", door.verb);
                let argv = argv_at(door, force);
                let args: Vec<&str> = argv.iter().map(String::as_str).collect();
                let out = fx.run(&args);
                let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                let stdout = String::from_utf8_lossy(&out.stdout).into_owned();

                // (a) Narrated ⇔ removed, proven on the planted bytes. A door that printed
                // `they are not recoverable` and then left the bytes is lying in one
                // direction; one that took them without a word is lying in the other.
                for sub in shape.planted() {
                    let printed = fx.printed(sub);
                    assert_eq!(
                        narrated(&stderr, &printed),
                        !fx.bytes_survive(sub),
                        "[{cell}] `{printed}`: a narrated removal must have happened, and a \
                         removal that happened must have been narrated\nstdout:\n{stdout}\n\
                         stderr:\n{stderr}",
                    );
                }

                if out.status.success() {
                    continue;
                }

                // A refusal moves nothing — the fail-closed claim, on the bytes.
                for sub in shape.planted() {
                    assert!(
                        fx.bytes_survive(sub),
                        "[{cell}] a refusal must leave `{}` byte-intact\nstderr:\n{stderr}",
                        fx.printed(sub),
                    );
                }
                assert!(
                    stderr.contains(&format!("blocking · {code}")),
                    "[{cell}] the refusal carries the door's own code `{code}`, so a driver \
                     can tell which door refused; got:\n{stderr}",
                );
                assert!(
                    stderr.contains("route:"),
                    "[{cell}] the refusal carries a route — the route floor's whole point; \
                     got:\n{stderr}",
                );
                // (b) Every planted leftover is named. One sibling's answer may not swallow
                // another's: this is the cell N9 was red in.
                for sub in shape.planted() {
                    assert!(
                        stderr.contains(&fx.printed(sub)),
                        "[{cell}] the refusal names every leftover it would destroy, \
                         `{}` among them; got:\n{stderr}",
                        fx.printed(sub),
                    );
                }
                // The target obligation, in the form the contract declares for this code: a
                // finding outside the declared-singleton exemption must name what it is about.
                if !engine::finding::is_declared_singleton(code) {
                    assert!(
                        at_target(&stderr).is_some(),
                        "[{cell}] a non-singleton refusal carries an `at:` locus; got:\n{stderr}",
                    );
                }
                // (c) The consent is named — the one word past this guard.
                assert!(
                    stderr.contains("--force"),
                    "[{cell}] the refusal names `--force`, the consent that gets past it; \
                     got:\n{stderr}",
                );

                // (d) The route does not loop: every concrete consent command it names,
                // run verbatim, must not reproduce this same refusal.
                let commands = consent_commands(&stderr);
                assert!(
                    !commands.is_empty(),
                    "[{cell}] the refusal's route must name at least one command a reader \
                     can run unchanged; got:\n{stderr}",
                );
                for command in commands {
                    let rerun_args: Vec<&str> = command.iter().map(String::as_str).collect();
                    let rerun = fx.run(&rerun_args);
                    let rerun_err = String::from_utf8_lossy(&rerun.stderr).into_owned();
                    assert!(
                        !rerun_err.contains(&format!("blocking · {code}")),
                        "[{cell}] the route `jigc {}` must not answer with the very refusal \
                         that printed it — a route at the argv that just failed is a loop\n\
                         first:\n{stderr}\nrerun:\n{rerun_err}",
                        command.join(" "),
                    );
                }
            }
        }
    }
}
