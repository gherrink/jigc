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
//! **N8's axis is the removal's *cause of failure*, not the leftover's shape** (M50 Increment
//! 12 completion audit). The first pass swept it over `Shape::ALL` alone, and `remove_dir_all`
//! aimed at a plain file is one cause among many: driven at `ee8c3e2`, one `chmod 555` on a
//! leftover directory holding a file put the identical defect back at **three** call sites at
//! once — `jigc milestone provision --force` (exit 1), `jigc uninstall --force` (exit 1) and
//! `jigc milestone discard --force` over a *registered* worktree (**exit 0**) — each printing
//! *"they are not recoverable"* over bytes still on disk afterwards. So [`Shape`] carries
//! `Unremovable`, the cell that fails for a reason the narrator cannot see, and the narration
//! moved behind the removal it describes (`cli::milestone::PendingLoss`): the honest claim is
//! about the outcome, and keying it there is complete over every cause by construction.
//!
//! **The axis is (refusing door) × (leftover shape) × (consent), each side read code-side or
//! enumerated here.** `cli::milestone::DESTROYING_DOORS` enumerates the four verbs that
//! remove a worktree-shaped path and `DestroyingDoor::code` is the table's own
//! refuse-vs-narrate discriminator, so a fifth refusing door cannot be added without landing
//! here. [`Shape`] carries the four plantable shapes — `Both` is the cell that was red,
//! because masking is only visible where there is a sibling to mask, and `Unremovable` the one
//! the audit added. The consent axis is the one N8 lived in and no cell of this suite drove
//! before.
//!
//! One pairing the door table cannot reach lives in its own test
//! ([`a_registered_worktree_the_teardown_cannot_remove_is_not_narrated_as_gone`]): the fixture
//! plants leftovers at *unregistered* paths, so `cli::milestone::remove_worktrees` — the
//! narrate-then-remove pair `discard` and `finalize` share — is never entered from here.
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
    /// A directory holding one untracked file that **cannot be removed** — the read-only
    /// parent bits make the unlink fail with `EACCES`. Shape-wise it is `Directory`; what it
    /// varies is the *cause* of the removal's failure, which is the axis N8 actually lives on
    /// and the one the first sweep did not iterate.
    ///
    /// **Declared bound:** run as `root`, permission bits do not bind and the removal
    /// succeeds — the `set_dir_readonly` idiom borrowed from `tests/setup.rs` and
    /// `tests/milestone_provision_handoff.rs` carries the same exposure. The cell then
    /// degenerates into `Directory` and still asserts truthfully.
    Unremovable,
}

impl Shape {
    const ALL: [Shape; 4] = [
        Shape::Directory,
        Shape::File,
        Shape::Both,
        Shape::Unremovable,
    ];

    /// The sub-task ids this shape plants a leftover at.
    fn planted(self) -> &'static [&'static str] {
        match self {
            Shape::Directory | Shape::Unremovable => &[SUB_DIR],
            Shape::File => &[SUB_FILE],
            Shape::Both => &[SUB_DIR, SUB_FILE],
        }
    }
}

/// Set `dir`'s permission bits — the `tests/setup.rs` idiom, which is how every cell that
/// needs a removal to genuinely fail manufactures one.
fn set_dir_mode(dir: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let Ok(meta) = fs::metadata(dir) else {
        return;
    };
    let mut perms = meta.permissions();
    perms.set_mode(mode);
    let _ = fs::set_permissions(dir, perms);
}

/// Give every directory under `root` (and `root` itself) its write bit back, so a fixture
/// that planted an [`Shape::Unremovable`] cell can be thrown away on drop. Top-down: a
/// read-only parent has to be opened before its children can be reached.
fn restore_writable(root: &Path) {
    set_dir_mode(root, 0o755);
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            restore_writable(&path);
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
        let fx = Fixture::mint();
        let worktrees = fx.repo.join(".jigc").join("worktrees");
        fs::create_dir_all(&worktrees).expect("mk .jigc/worktrees/");
        for sub in shape.planted() {
            let witness = fx.witness(sub);
            if let Some(parent) = witness.parent() {
                fs::create_dir_all(parent).expect("mk the leftover's parent");
            }
            fs::write(&witness, PRECIOUS).expect("plant the leftover");
        }
        if matches!(shape, Shape::Unremovable) {
            // The unlink needs write permission on the *containing* directory, so this is
            // what makes the removal genuinely impossible rather than merely slow.
            for sub in shape.planted() {
                set_dir_mode(&worktrees.join(sub), 0o555);
            }
        }
        fx
    }

    /// The repo + milestone + two sub-tasks, with nothing squatting either worktree path —
    /// [`Fixture::plant`]'s base, and the state the registered-worktree pairing provisions
    /// from.
    fn mint() -> Self {
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

impl Drop for Fixture {
    /// Hand the write bits back before the throwaway roots drop: a [`Shape::Unremovable`]
    /// cell is a directory `remove_dir_all` genuinely cannot clear, and that binds this
    /// suite's own cleanup exactly as hard as it binds the door under test.
    fn drop(&mut self) {
        restore_writable(&self.repo.join(".jigc"));
    }
}

/// Whether `stderr` narrated a removal **of** `printed` — the law-1 line every destroying
/// door prints through `cli::milestone::narrate_removal` for what it actually took.
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

                // **A non-zero exit under `--force` is a failed removal, not a refusal.**
                // `--force` is consent past every guard below, so what a door answers with
                // here is its failure identity (`milestone.provision-failed`,
                // `uninstall.remove-jigc`) rather than its refusal code — a different claim,
                // owed the route floor but not the fail-closed one, since bytes may
                // legitimately have moved before it stopped. Reachable only since the
                // `Unremovable` cell: with a removal that can happen, every consenting cell
                // exits 0. What it must NOT have done is narrate a destruction that did not
                // happen, and (a) has already checked that on the planted bytes.
                if force {
                    assert!(
                        stderr.contains("blocking · ") && stderr.contains("route:"),
                        "[{cell}] a consenting run that failed still answers with a code and \
                         a route; got:\n{stderr}",
                    );
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

/// **The pairing the door table cannot reach: a *registered* worktree the teardown could not
/// remove.** `cli::milestone::remove_worktrees` is `discard`'s and `finalize`'s own
/// narrate-then-remove pair, and it only ever looks at paths this repository has registered as
/// worktrees — which the fixture above deliberately never has, so every cell of the door table
/// walks straight past it.
///
/// Driven at `ee8c3e2` it carried N8 in its worst form: `jigc milestone discard --force` over a
/// read-only registered worktree printed *"the fan-out worktree is the only copy of these bytes
/// — they are not recoverable"*, git's removal then failed with `Permission denied`, and the
/// door **exited 0** — the one cell of the class where the operator is told their work is gone
/// by a command that reports success.
///
/// The removable sibling is in the same run on purpose: it is the control that keeps this test
/// from passing by the door simply going quiet. One worktree is narrated **and** gone, the
/// other is neither.
#[test]
fn a_registered_worktree_the_teardown_cannot_remove_is_not_narrated_as_gone() {
    let fx = Fixture::mint();
    fx.jigc_ok(&["milestone", "provision", MILESTONE]);

    // One untracked file per worktree — the bytes `git worktree remove --force` takes and
    // the narration names.
    let worktrees = fx.repo.join(".jigc").join("worktrees");
    for sub in [SUB_DIR, SUB_FILE] {
        fs::write(worktrees.join(sub).join("scratch.rs"), PRECIOUS).expect("plant sub-agent WIP");
    }
    // `SUB_DIR`'s removal cannot happen; `SUB_FILE`'s can.
    set_dir_mode(&worktrees.join(SUB_DIR), 0o555);

    let out = fx.run(&["milestone", "discard", MILESTONE, "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the abandon is best-effort about its teardown and still settles; stderr:\n{stderr}",
    );

    let held = worktrees.join(SUB_DIR).join("scratch.rs");
    assert_eq!(
        fs::read_to_string(&held).ok().as_deref(),
        Some(PRECIOUS),
        "the un-removable worktree's bytes must survive — the premise of the assertion \
         below; stderr:\n{stderr}",
    );
    assert!(
        !narrated(&stderr, &fx.printed(SUB_DIR)),
        "a removal that failed must not be narrated as a destruction — the door exits 0 \
         here, so this line is the operator's only account of the bytes; stderr:\n{stderr}",
    );

    let taken = worktrees.join(SUB_FILE).join("scratch.rs");
    assert!(
        !taken.exists(),
        "the control worktree must actually have been removed, or this test proves nothing \
         about the narration; stderr:\n{stderr}",
    );
    assert!(
        narrated(&stderr, &fx.printed(SUB_FILE)),
        "a removal that happened must be narrated — the other half of the biconditional, \
         and what keeps the fix above from being `narrate nothing`; stderr:\n{stderr}",
    );
}

/// **The same rule at `jigc uninstall`'s other two subjects.** The teardown narrates three
/// things — the fan-out worktrees, the open tasks' staged prose, and the rest of the workbench
/// split on recoverability — and all three were printed *before* `remove_dir_all(.jigc)` ran.
/// Driven at `ee8c3e2` over a `.jigc/` whose bits make the removal impossible, the door
/// therefore reported the whole workbench as destroyed and exited 1 with every byte in place.
///
/// The removable control run is the other half: a teardown that succeeds must still name what
/// it took, or the fix is indistinguishable from deleting the narration.
#[test]
fn a_teardown_that_removed_nothing_narrates_nothing() {
    let claims_a_removal = |stderr: &str| {
        stderr
            .lines()
            .any(|line| line.starts_with("warning: removing"))
    };

    let held = Fixture::mint();
    let notes = held.repo.join(".jigc").join("notes.md");
    fs::write(&notes, PRECIOUS).expect("plant a workbench file no index has a copy of");
    set_dir_mode(&held.repo.join(".jigc"), 0o555);

    let out = held.run(&["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        !out.status.success(),
        "a teardown that could not remove `.jigc/` must not exit 0; stderr:\n{stderr}",
    );
    assert_eq!(
        fs::read_to_string(&notes).ok().as_deref(),
        Some(PRECIOUS),
        "the premise: nothing was taken; stderr:\n{stderr}",
    );
    assert!(
        !claims_a_removal(&stderr),
        "a teardown that took nothing must claim no removal at all — of the workbench, of a \
         worktree, or of staged prose; stderr:\n{stderr}",
    );

    let cleared = Fixture::mint();
    let taken = cleared.repo.join(".jigc").join("notes.md");
    fs::write(&taken, PRECIOUS).expect("plant a workbench file no index has a copy of");
    let out = cleared.run(&["uninstall", "--force"]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the control teardown must succeed; stderr:\n{stderr}",
    );
    assert!(
        !taken.exists() && claims_a_removal(&stderr) && stderr.contains("notes.md"),
        "a teardown that took the workbench must name it — the control that keeps the \
         assertion above from being satisfied by silence; stderr:\n{stderr}",
    );
}
