//! **A jigc door removes a git worktree registration only at a path jigc created** — the
//! reach of every door over `.git/worktrees/` (the rc.24 blind trial, L-22;
//! `design/team-ready-state.md` → *Abandon refuses on a dirty worktree*, the registration
//! reach).
//!
//! Since M31 four doors ran a bare, repository-wide `git worktree prune`: `jigc milestone
//! provision`, the fan-out teardown behind `jigc milestone finalize` and `jigc milestone
//! discard`, the squash combine's dedicated worktree (twice — on `add` and on drop, so a
//! **refusing** finalize ran it too) and, since the 2026-09-23 confirmation pass, `jigc
//! uninstall`. A prune drops the registration of **every** worktree whose directory is not
//! where git recorded it — and that is not only *a directory somebody deleted*. Driven on
//! `jigc 1.0.0-rc.24`: a worktree on a volume that is not mounted, one moved with plain `mv`
//! and still working at its new path, and a live host worktree seen from a container that
//! mounts the repository alone were each `not a git repository` after one milestone door at
//! exit 0, with nothing printed; `git worktree repair` could not mend them, and a commit only
//! that worktree's detached `HEAD` reached went to git's own gc.
//!
//! The rule the product already stated for its own paths (`design/storage.md` → Repository
//! layout: *no door removes such a path silently*) is the one restored here for git's side
//! of them: a registration is removed **by path**, only under `<jigc_home>/.jigc/worktrees/`,
//! and a run that refuses removes none.
//!
//! # The class, and what iterates it
//!
//! * **the door axis** — `cli::milestone::WORKTREE_DOORS`, read code-side, so a fifth
//!   worktree door lands in the cell matrix rather than being remembered. Each door is driven
//!   over a **bare** workbench (nothing provisioned — which is where `finalize` *refuses*, so
//!   the landing/refusing split is inside the axis, not beside it) and over a **provisioned**
//!   one, on both output formats; `finalize` additionally on both commit arms.
//! * **the foreign-state axis** — [`FOREIGN`]: a detached worktree holding a commit only its
//!   `HEAD` reaches plus a staged-only file (the cell where bytes die), one on a branch, and
//!   a **locked** one. All three have their directories moved away, which is git's `prunable`
//!   state for the first two.
//! * **the mechanism itself** — a source fence over every production `git worktree <verb>`
//!   argv ([`WORKTREE_ADMIN_SITES`]), because a cell matrix proves the doors that exist and
//!   only a fence stops the sixth site.
//!
//! Every cell asserts on **what git holds** — the admin directory's bytes, and the worktree
//! answering `git status` once its directory is back — never on jigc's words alone.

use cli::milestone::{DestroyingDoor, FINALIZE_DOOR, UNINSTALL_DOOR, WORKTREE_DOORS};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::support::rust_source;

/// The milestone every fixture mints.
const MILESTONE: &str = "cache-rework";
/// Its two sub-tasks — and therefore the two worktree paths jigc owns here.
const SUBS: [&str; 2] = ["area-low", "area-zed"];

/// The foreign-state axis: the basename of each foreign worktree, which is also the name of
/// its admin directory under `.git/worktrees/`.
const FOREIGN: [&str; 3] = ["foreign-detached", "foreign-branch", "foreign-locked"];

/// A throwaway directory that removes itself on drop (the project's no-tempfile pattern).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-wt-reach-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        // Canonical from the start: git records realpaths at `worktree add`, and on macOS
        // the temp root is a symlink, so every comparison below would otherwise be between
        // two spellings of one directory.
        TempDir(path.canonicalize().expect("canonicalize temp dir"))
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

fn git(cwd: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git")
}

fn git_ok(cwd: &Path, args: &[&str]) -> String {
    let out = git(cwd, args);
    assert!(
        out.status.success(),
        "git {args:?} in {cwd:?} failed: {}",
        String::from_utf8_lossy(&out.stderr),
    );
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// A repo carrying `milestone:cache-rework` with two sub-tasks, plus a sibling directory the
/// foreign worktrees live in — **outside** the repository, as a human's linked worktree is.
struct Fixture {
    root: TempDir,
    home: TempDir,
    repo: PathBuf,
}

impl Fixture {
    fn mint(tag: &str) -> Self {
        let root = TempDir::new(tag);
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo");
        git_ok(&repo, &["init", "-q", "-b", "main"]);
        git_ok(&repo, &["config", "user.email", "test@example.com"]);
        git_ok(&repo, &["config", "user.name", "Test"]);
        git_ok(&repo, &["config", "commit.gpgsign", "false"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        git_ok(&repo, &["add", "."]);
        git_ok(&repo, &["commit", "-q", "-m", "initial"]);
        // The project cascade layer — `jigc milestone`'s door-top precondition.
        crate::support::mint_project_layer(&repo);

        let fx = Fixture {
            root,
            home: TempDir::new("home"),
            repo,
        };
        fx.jigc_ok(&["milestone", "create", "Cache rework"]);
        for intent in ["Area low", "Area zed"] {
            fx.jigc_ok(&["milestone", "add-task", MILESTONE, intent]);
        }
        fx
    }

    /// Opt the project into per-sub-task commits — the chain arm of the boundary.
    fn set_squash_false(&self) {
        fs::write(
            self.repo.join(".jigc").join("config").join("manifest.yaml"),
            "scalar:\n  finalize.fan-out.squash: false\n",
        )
        .expect("write manifest");
    }

    fn worktree(&self, sub: &str) -> PathBuf {
        self.repo.join(".jigc").join("worktrees").join(sub)
    }

    /// git's admin directory for the worktree whose basename is `name`.
    fn admin(&self, name: &str) -> PathBuf {
        self.repo.join(".git").join("worktrees").join(name)
    }

    /// Where the foreign worktree `name` lives while it is reachable.
    fn foreign(&self, name: &str) -> PathBuf {
        self.root.path().join(name)
    }

    fn run_in(&self, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    fn run(&self, args: &[&str]) -> Output {
        self.run_in(&self.repo, args)
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

    /// Provision the fan-out, then give every sub-task something for a boundary to land:
    /// one staged file in its worktree and the authored `commit:<sub>` doc the chain arm
    /// renders its per-sub-task commit from.
    fn provision_with_staged_code(&self) {
        self.jigc_ok(&["milestone", "provision", MILESTONE]);
        for sub in SUBS {
            self.stage_code(sub);
        }
    }

    fn stage_code(&self, sub: &str) {
        let worktree = self.worktree(sub);
        fs::write(worktree.join(format!("{sub}.txt")), "code\n").expect("write code");
        git_ok(&worktree, &["add", &format!("{sub}.txt")]);

        let docs = self.repo.join(".jigc").join("tasks").join(sub).join("docs");
        fs::create_dir_all(&docs).expect("mk docs/");
        fs::write(
            docs.join(format!("commit:{sub}.md")),
            format!(
                "---\ntype: feat\n---\n\n# {sub}\n\n## Summary\n\nadd the {sub} file\n\n\
                 ## Body\n\n\n\n## Trailers\n"
            ),
        )
        .expect("write the staged commit doc");
        fs::write(
            docs.join("provenance.json"),
            format!("{{\n  \"docs\": {{\n    \"commit:{sub}\": \"created\"\n  }}\n}}\n"),
        )
        .expect("write the provenance manifest");
    }

    /// The names under `.git/worktrees/`, sorted — git's whole registry, by admin directory.
    fn admin_names(&self) -> Vec<String> {
        let mut names: Vec<String> = match fs::read_dir(self.repo.join(".git").join("worktrees")) {
            Ok(entries) => entries
                .map(|entry| {
                    entry
                        .expect("read .git/worktrees")
                        .file_name()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect(),
            // No linked worktree was ever registered: git has not created the directory.
            Err(_) => Vec::new(),
        };
        names.sort();
        names
    }

    /// The `git worktree list --porcelain` record for `path`, or `None` when git no longer
    /// names it.
    fn registration(&self, path: &Path) -> Option<String> {
        let listed = git_ok(&self.repo, &["worktree", "list", "--porcelain"]);
        listed
            .split("\n\n")
            .find(|record| record.lines().next() == Some(&format!("worktree {}", path.display())))
            .map(str::to_owned)
    }
}

/// Every file under `dir`, relative, with its bytes — a registration read back **whole**.
///
/// The bytes, not the presence: the loss this class is about is `HEAD`, the index and the
/// reflog of a checkout going away, and a cell that checked only that the directory still
/// existed would pass over a record emptied in place.
fn bytes_under(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries {
            let path = entry.expect("read admin dir").path();
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                let rel = path
                    .strip_prefix(base)
                    .expect("under the base")
                    .to_string_lossy()
                    .into_owned();
                out.insert(rel, fs::read(&path).expect("read admin file"));
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(dir, dir, &mut out);
    out
}

/// The three foreign worktrees, planted and then made unreachable, with what only they hold.
struct Foreign {
    /// The commit only `foreign-detached`'s `HEAD` reaches.
    detached_commit: String,
    /// Each registration's bytes as planted — the before-control every cell compares to.
    planted: BTreeMap<&'static str, BTreeMap<String, Vec<u8>>>,
}

/// The suffix a foreign worktree's directory carries while it is "not mounted".
const AWAY: &str = ".away";

fn away(path: &Path) -> PathBuf {
    let mut name = path.file_name().expect("a named path").to_os_string();
    name.push(AWAY);
    path.with_file_name(name)
}

/// Plant the foreign-state axis and move every directory away — the unmounted volume, the
/// `mv`, the path a container does not see: one state to git, *the directory is not where
/// the record says*.
fn plant_foreign(fx: &Fixture) -> Foreign {
    // (i) detached, holding a commit only its HEAD reaches, plus a staged-only file.
    let detached = fx.foreign("foreign-detached");
    git_ok(
        &fx.repo,
        &[
            "worktree",
            "add",
            "-q",
            "--detach",
            detached.to_str().unwrap(),
            "HEAD",
        ],
    );
    fs::write(detached.join("foreign-committed.txt"), "c\n").expect("write");
    git_ok(&detached, &["add", "foreign-committed.txt"]);
    git_ok(
        &detached,
        &["commit", "-q", "-m", "foreign detached commit"],
    );
    let detached_commit = git_ok(&detached, &["rev-parse", "HEAD"]);
    fs::write(detached.join("foreign-staged.txt"), "staged-only bytes\n").expect("write");
    git_ok(&detached, &["add", "foreign-staged.txt"]);

    // (ii) on a branch — the common shape of a linked worktree.
    let branch = fx.foreign("foreign-branch");
    git_ok(
        &fx.repo,
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "side",
            branch.to_str().unwrap(),
            "HEAD",
        ],
    );
    fs::write(branch.join("branch-staged.txt"), "z staged\n").expect("write");
    git_ok(&branch, &["add", "branch-staged.txt"]);

    // (iii) locked — git's own documented guard for a worktree on a volume that comes and
    // goes, and the one state the old prune already left alone.
    let locked = fx.foreign("foreign-locked");
    git_ok(
        &fx.repo,
        &[
            "worktree",
            "add",
            "-q",
            "--detach",
            locked.to_str().unwrap(),
            "HEAD",
        ],
    );
    git_ok(&fx.repo, &["worktree", "lock", locked.to_str().unwrap()]);

    for name in FOREIGN {
        let at = fx.foreign(name);
        fs::rename(&at, away(&at)).expect("move the foreign worktree away");
    }

    // The before-control: git itself calls the first two `prunable` and the third `locked`,
    // so a door that runs a repository-wide prune has something to take — a fixture whose
    // foreign worktrees were live would pass under the defect (which is exactly how every
    // earlier foreign planting in these suites missed it).
    for (name, state) in FOREIGN.iter().zip(["prunable", "prunable", "locked"]) {
        let record = fx
            .registration(&fx.foreign(name))
            .unwrap_or_else(|| panic!("fixture: git must list `{name}` before the door runs"));
        assert!(
            record.lines().any(|line| line.starts_with(state)),
            "fixture: `{name}` must be `{state}` to git before the door runs; got:\n{record}",
        );
    }
    let planted: BTreeMap<&'static str, BTreeMap<String, Vec<u8>>> = FOREIGN
        .iter()
        .map(|name| (*name, bytes_under(&fx.admin(name))))
        .collect();
    for (name, bytes) in &planted {
        assert!(
            bytes.contains_key("HEAD") && bytes.contains_key("gitdir"),
            "fixture: `{name}`'s registration must hold its HEAD and gitdir; got {:?}",
            bytes.keys().collect::<Vec<_>>(),
        );
    }
    Foreign {
        detached_commit,
        planted,
    }
}

/// **The whole claim, asserted on what git holds**: every foreign registration is
/// byte-identical to how it was planted, and — once its directory is back — the worktree
/// still answers git, with the staged file still staged and the detached commit still at
/// `HEAD`.
fn assert_foreign_untouched(fx: &Fixture, foreign: &Foreign, cell: &str) {
    for name in FOREIGN {
        assert_eq!(
            bytes_under(&fx.admin(name)).keys().collect::<Vec<_>>(),
            foreign.planted[name].keys().collect::<Vec<_>>(),
            "{cell}: the registration of `{name}` — a worktree jigc did not create — must \
             still be under `.git/worktrees/` with every file it had",
        );
        assert_eq!(
            bytes_under(&fx.admin(name)),
            foreign.planted[name],
            "{cell}: …and byte-identical: its HEAD, its index and its reflog are not jigc's \
             to touch",
        );
        assert!(
            fx.registration(&fx.foreign(name)).is_some(),
            "{cell}: `git worktree list` must still name `{name}`",
        );
    }

    // The directories come back — the volume is mounted again.
    for name in FOREIGN {
        let at = fx.foreign(name);
        fs::rename(away(&at), &at).expect("bring the foreign worktree back");
    }
    let detached = fx.foreign("foreign-detached");
    let status = git(&detached, &["status", "--porcelain"]);
    assert!(
        status.status.success(),
        "{cell}: the returned worktree must still be a git repository; stderr:\n{}",
        String::from_utf8_lossy(&status.stderr),
    );
    assert!(
        String::from_utf8_lossy(&status.stdout).contains("A  foreign-staged.txt"),
        "{cell}: …with its staged-only file still staged; got:\n{}",
        String::from_utf8_lossy(&status.stdout),
    );
    assert_eq!(
        git_ok(&detached, &["rev-parse", "HEAD"]),
        foreign.detached_commit,
        "{cell}: …and the commit only its detached HEAD reaches still at HEAD",
    );
    assert!(
        git(&fx.foreign("foreign-branch"), &["status", "--porcelain"])
            .status
            .success(),
        "{cell}: the branch worktree must answer git too",
    );
}

/// The argv that stands at `door`, **derived from the door's own `verb`** rather than
/// hand-listed (the `leftover_probe_fail_closed` convention), carrying the door's consent
/// when it has one: these cells are about what a *landing* run does to git's registry, and
/// `--force` is the one flag every refusing member lands under.
fn argv_at(door: &DestroyingDoor, json: bool) -> Vec<String> {
    let mut argv: Vec<String> = door
        .verb
        .split_whitespace()
        .skip(1)
        .map(str::to_owned)
        .collect();
    if argv.first().map(String::as_str) == Some("milestone") {
        argv.push(MILESTONE.to_owned());
    }
    if let Some(consent) = door.consent() {
        argv.push(consent.to_owned());
    }
    if json {
        argv.extend(["--format".to_owned(), "json".to_owned()]);
    }
    argv
}

// ---------------------------------------------------------------------------
// The door axis × the foreign-state axis.
// ---------------------------------------------------------------------------

/// One cell: `door`, over a bare or a provisioned workbench, in one output format, on one
/// commit arm.
fn door_cell(door: &DestroyingDoor, provisioned: bool, json: bool, squash: bool) {
    let cell = format!(
        "{} × {} × {} × squash: {squash}",
        door.verb,
        if provisioned { "provisioned" } else { "bare" },
        if json { "json" } else { "text" },
    );
    let fx = Fixture::mint("door");
    if !squash {
        fx.set_squash_false();
    }
    if provisioned {
        fx.provision_with_staged_code();
    }
    let foreign = plant_foreign(&fx);

    let argv = argv_at(door, json);
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let out = fx.run(&args);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    // The landing/refusing split, inside the axis: a boundary over a milestone nothing was
    // provisioned for has nothing to land and refuses (`milestone.zero-contribution`) — the
    // refusing run that used to prune on its way out. Every other cell lands.
    let refuses = door.verb == FINALIZE_DOOR.verb && !provisioned;
    assert_eq!(
        out.status.success(),
        !refuses,
        "{cell}: unexpected exit {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status,
    );

    assert_foreign_untouched(&fx, &foreign, &cell);

    // …and what the door owns, it still takes: the three teardown doors leave none of
    // jigc's own registrations behind, and the boundary leaves no combine worktree.
    // (git names the admin directory after the worktree's basename with the leading dot
    // rewritten — `.combine-1-2` registers as `-combine-1-2` — so the match is on the stem.)
    let names = fx.admin_names();
    assert!(
        !names.iter().any(|name| name.contains("combine-")),
        "{cell}: the boundary's dedicated worktree must not stay registered; got {names:?}",
    );
    let tears_down = provisioned && door.verb != cli::milestone::PROVISION_DOOR.verb;
    if tears_down {
        for sub in SUBS {
            assert!(
                !names.iter().any(|name| name == sub),
                "{cell}: the door removed sub-task `{sub}`'s worktree, so its registration \
                 must be gone too; `.git/worktrees/` holds {names:?}\nstderr:\n{stderr}",
            );
        }
    }
    // `uninstall` is the door that narrates the registry: it says it dropped registrations
    // **iff** it dropped one of its own — over a workbench that held no fan-out worktree the
    // sentence was printed for a foreign record (rc.24), which is the misattribution.
    if door.verb == UNINSTALL_DOOR.verb && !json {
        assert_eq!(
            stdout.contains("dropped git's registrations of the fan-out worktrees"),
            provisioned,
            "{cell}: the ack names dropped registrations iff `.jigc/` held a registered \
             worktree; got:\n{stdout}",
        );
    }
}

/// **The door axis** — every member of `WORKTREE_DOORS`, read code-side, over a bare and a
/// provisioned workbench, on both output formats; the boundary on both commit arms.
#[test]
fn every_worktree_door_leaves_a_foreign_registration_exactly_as_it_found_it() {
    assert!(
        WORKTREE_DOORS.len() >= 4,
        "the worktree door table must carry the four doors this class was found at",
    );
    for door in WORKTREE_DOORS {
        for provisioned in [false, true] {
            for json in [false, true] {
                door_cell(door, provisioned, json, true);
            }
        }
        if door.verb == FINALIZE_DOOR.verb {
            // The per-sub-task chain arm — the other producer of a dedicated worktree.
            door_cell(door, true, false, false);
        }
    }
}

// ---------------------------------------------------------------------------
// A refusing run removes nothing.
// ---------------------------------------------------------------------------

/// **A run that refuses leaves `.git/worktrees/` exactly as it found it** — jigc's own
/// registrations included.
///
/// The provision cell is the sharp one: a sub-task path registered here whose checkout lost
/// its `.git` link is `prunable` to git and holds bytes nothing can vouch for, so the door
/// refuses (`milestone.leftover-holds-work`, *nothing was removed*) — and until this fix
/// had already dropped that registration on its way to the refusal.
#[test]
fn a_refusing_run_leaves_git_worktree_admin_as_it_found_it() {
    type Arrange = fn(&Fixture);
    let cells: [(&str, &[&str], &str, Arrange); 5] = [
        (
            "finalize over a milestone nothing was provisioned for",
            &["milestone", "finalize", MILESTONE],
            "finalize.empty-commit",
            |_| {},
        ),
        (
            "finalize whose combine commit the user's own `pre-commit` hook rejects",
            &["milestone", "finalize", MILESTONE],
            "was rejected (no commit was made)",
            |fx| {
                fx.provision_with_staged_code();
                let hook = fx.repo.join(".git").join("hooks").join("pre-commit");
                fs::create_dir_all(hook.parent().expect("hooks dir")).expect("mk hooks/");
                // Rejects the boundary's own commit and no other — the foreign worktrees
                // planted next share this hooks directory and must still be able to commit.
                fs::write(
                    &hook,
                    "#!/bin/sh\ncase \"$(pwd)\" in\n  */.jigc/worktrees/.combine-*) \
                     echo 'rejected by the project hook' >&2; exit 1 ;;\nesac\nexit 0\n",
                )
                .expect("write the hook");
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&hook, fs::Permissions::from_mode(0o755))
                        .expect("chmod the hook");
                }
            },
        ),
        (
            "provision over a registered sub-task path whose checkout lost its `.git` link",
            &["milestone", "provision", MILESTONE],
            "milestone.leftover-holds-work",
            |fx| {
                fx.jigc_ok(&["milestone", "provision", MILESTONE]);
                fs::remove_file(fx.worktree(SUBS[0]).join(".git")).expect("unlink the checkout");
            },
        ),
        (
            "discard over a sub-task worktree holding uncommitted work",
            &["milestone", "discard", MILESTONE],
            "milestone.dirty-worktree",
            |fx| {
                fx.jigc_ok(&["milestone", "provision", MILESTONE]);
                fs::write(fx.worktree(SUBS[0]).join("wip.txt"), "wip\n").expect("write wip");
            },
        ),
        (
            "uninstall over a sub-task worktree holding uncommitted work",
            &["uninstall"],
            "uninstall.dirty-worktree",
            |fx| {
                fx.jigc_ok(&["milestone", "provision", MILESTONE]);
                fs::write(fx.worktree(SUBS[0]).join("wip.txt"), "wip\n").expect("write wip");
            },
        ),
    ];
    // The third field is what the refusal must carry to be THIS refusal — a finding code,
    // or for the hook-rejected commit (an abort, not a finding) its own opening sentence.
    for (cell, args, code, arrange) in cells {
        let fx = Fixture::mint("refuse");
        arrange(&fx);
        let foreign = plant_foreign(&fx);
        let names = fx.admin_names();

        let out = fx.run(args);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            !out.status.success() && stderr.contains(code),
            "{cell}: the cell must refuse, saying `{code}`; got {:?}\nstderr:\n{stderr}",
            out.status,
        );
        assert_eq!(
            fx.admin_names(),
            names,
            "{cell}: a refusing run must leave git's worktree registry as it found it — \
             jigc's own registrations included",
        );
        assert_foreign_untouched(&fx, &foreign, cell);
    }
}

// ---------------------------------------------------------------------------
// What jigc owns, it still clears — by path.
// ---------------------------------------------------------------------------

/// **The crash-recovery self-heal the prune was added for survives its removal**: a sub-task
/// worktree whose directory is gone is re-provisioned at the base pin, its stale registration
/// dropped by path — with a prunable foreign sibling standing right beside it.
#[test]
fn provision_still_heals_its_own_stale_registration_and_only_its_own() {
    let fx = Fixture::mint("heal");
    fx.jigc_ok(&["milestone", "provision", MILESTONE]);
    let base = git_ok(&fx.repo, &["rev-parse", "HEAD"]);

    // (1) The directory deleted out from under git — the crashed-run shape.
    fs::remove_dir_all(fx.worktree(SUBS[0])).expect("delete the worktree directory");
    // (2) The sibling reduced to an EMPTY directory with no `.git` link — nothing to keep.
    fs::remove_dir_all(fx.worktree(SUBS[1])).expect("delete the sibling");
    fs::create_dir_all(fx.worktree(SUBS[1])).expect("leave an empty directory");
    let foreign = plant_foreign(&fx);
    for sub in SUBS {
        assert!(
            fx.registration(&fx.worktree(sub))
                .is_some_and(|record| record.contains("prunable")),
            "fixture: `{sub}`'s own registration must be stale before the re-run",
        );
    }

    fx.jigc_ok(&["milestone", "provision", MILESTONE]);
    for sub in SUBS {
        let worktree = fx.worktree(sub);
        assert_eq!(
            git_ok(&worktree, &["rev-parse", "HEAD"]),
            base,
            "`{sub}`: the re-run must land a live worktree at the base pin",
        );
        let record = fx
            .registration(&worktree)
            .unwrap_or_else(|| panic!("`{sub}` must be registered after the re-run"));
        assert!(
            !record.contains("prunable"),
            "`{sub}`: …and its registration must be the live one; got:\n{record}",
        );
    }
    assert_eq!(
        fx.admin_names()
            .iter()
            .filter(|name| name.starts_with("area-"))
            .count(),
        SUBS.len(),
        "the stale records must have been replaced, not duplicated under a suffixed name; \
         `.git/worktrees/` holds {:?}",
        fx.admin_names(),
    );
    assert_foreign_untouched(&fx, &foreign, "provision re-run over its own stale records");
}

/// A `git` that answers the way one **older than 2.31** does: every call goes to the real
/// git unchanged, except that `git worktree list --porcelain` comes back without its
/// `prunable` and `locked` lines — the one difference in this listing between those
/// versions (git's 2.31 release notes; driven against a real git 2.30.9 when this cell was
/// written, where the listing carries neither line and the unfixed door behaved exactly as
/// it does under this stand-in).
///
/// It is a stand-in, said plainly: the suite runs on whatever git the machine has. The
/// directory returned goes first on `PATH`.
fn git_without_verdict_lines(root: &Path) -> PathBuf {
    let real = String::from_utf8(
        Command::new("sh")
            .args(["-c", "command -v git"])
            .output()
            .expect("locate git")
            .stdout,
    )
    .expect("utf-8 git path")
    .trim()
    .to_owned();
    assert!(Path::new(&real).is_absolute(), "git resolves to `{real}`");
    let dir = root.join("old-git-bin");
    fs::create_dir_all(&dir).expect("mk the wrapper dir");
    let script = format!(
        "#!/bin/sh\n\
         real='{real}'\n\
         worktree=0 list=0 porcelain=0\n\
         for arg in \"$@\"; do\n\
         \x20 case \"$arg\" in\n\
         \x20   worktree) worktree=1 ;;\n\
         \x20   list) list=1 ;;\n\
         \x20   --porcelain) porcelain=1 ;;\n\
         \x20 esac\n\
         done\n\
         if [ \"$worktree$list$porcelain\" = 111 ]; then\n\
         \x20 out=$(\"$real\" \"$@\") || exit $?\n\
         \x20 printf '%s\\n' \"$out\" | grep -v -e '^prunable' -e '^locked'\n\
         \x20 exit 0\n\
         fi\n\
         exec \"$real\" \"$@\"\n"
    );
    let wrapper = dir.join("git");
    fs::write(&wrapper, script).expect("write the wrapper");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755)).expect("chmod");
    }
    dir
}

/// **`provision` heals its own stale registration on a git that prints no `prunable` line**
/// (the rc.24 fix pass's completion audit, F2).
///
/// The reach fix replaced *prune, then list* with *list, and read `prunable`* — a line git
/// has printed only since 2.31. On an older git a sub-task worktree whose directory was gone
/// read as live: `jigc milestone provision` reused it, printed `provisioned`, and left no
/// directory, at exit 0, where `1.0.0-rc.24` re-created it; `jigc milestone execute` then
/// routed back at the provision that had done nothing.
///
/// Three cells under [`git_without_verdict_lines`], each against what the real git does:
/// a healthy fan-out is **reused** untouched (the must-not-drop control), a registration
/// whose directory is gone is **re-created** at the base pin, and a **locked** one is left
/// exactly as it is — git never calls a locked registration stale, with the line or
/// without it.
#[test]
fn provision_heals_its_own_stale_registration_where_git_prints_no_prunable_line() {
    let fx = Fixture::mint("old-git");
    let old_git = git_without_verdict_lines(fx.root.path());
    let path = format!(
        "{}:{}",
        old_git.display(),
        std::env::var("PATH").expect("PATH is set"),
    );
    let run_old = |args: &[&str]| -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(&fx.repo)
            .env("HOME", fx.home.path())
            .env("PATH", &path)
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    };
    let listing_old = || -> String {
        let out = Command::new("git")
            .args(["worktree", "list", "--porcelain"])
            .current_dir(&fx.repo)
            .env("PATH", &path)
            .output()
            .expect("run the wrapped git");
        String::from_utf8_lossy(&out.stdout).into_owned()
    };
    let provision = ["milestone", "provision", MILESTONE];

    fx.jigc_ok(&provision);
    let base = git_ok(&fx.repo, &["rev-parse", "HEAD"]);

    // (1) Healthy: nothing is stale, so the re-run reuses both worktrees untouched — a file
    // only that checkout holds is still there.
    let marker = fx.worktree(SUBS[1]).join("only-copy.txt");
    fs::write(&marker, "kept\n").expect("write a marker");
    let out = run_old(&provision);
    assert!(
        out.status.success() && marker.is_file(),
        "a healthy fan-out is reused untouched on a git with no verdict lines; got {:?}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );

    // (2) The directory gone, the registration empty — the crashed-run shape.
    fs::remove_dir_all(fx.worktree(SUBS[0])).expect("delete the worktree directory");
    let foreign = plant_foreign(&fx);
    assert!(
        fx.registration(&fx.worktree(SUBS[0]))
            .is_some_and(|record| record.contains("prunable")),
        "fixture: the real git reads the registration as stale",
    );
    assert!(
        !listing_old()
            .lines()
            .any(|line| line.starts_with("prunable") || line.starts_with("locked")),
        "fixture: the stand-in prints neither verdict line; got:\n{}",
        listing_old(),
    );
    let out = run_old(&provision);
    assert!(
        out.status.success(),
        "the re-run must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let worktree = fx.worktree(SUBS[0]);
    assert!(
        worktree.is_dir(),
        "the sub-task worktree whose directory was gone must be there again — the door \
         printed:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert_eq!(
        git_ok(&worktree, &["rev-parse", "HEAD"]),
        base,
        "…as a live worktree at the base pin",
    );
    assert_eq!(
        fx.admin_names()
            .iter()
            .filter(|name| name.starts_with("area-"))
            .count(),
        SUBS.len(),
        "the stale record must have been replaced, not duplicated; `.git/worktrees/` holds \
         {:?}",
        fx.admin_names(),
    );
    assert!(
        marker.is_file(),
        "…and the healthy sibling is still untouched"
    );
    assert_foreign_untouched(&fx, &foreign, "provision on a git with no verdict lines");
    let execute = run_old(&["milestone", "execute", MILESTONE]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&execute.stdout),
        String::from_utf8_lossy(&execute.stderr),
    );
    assert!(
        !said.contains("milestone.worktrees-partial"),
        "`milestone execute` must have nothing partial left to report; got:\n{said}",
    );

    // (3) Locked and gone. git calls a locked registration stale on no version, so the
    // door leaves the record exactly as it is — and answers as it does on the real git.
    git_ok(
        &fx.repo,
        &["worktree", "lock", fx.worktree(SUBS[1]).to_str().unwrap()],
    );
    fs::remove_dir_all(fx.worktree(SUBS[1])).expect("delete the locked worktree's directory");
    let before = bytes_under(&fx.admin(SUBS[1]));
    let old = run_old(&provision);
    assert_eq!(
        bytes_under(&fx.admin(SUBS[1])),
        before,
        "a locked registration is never dropped; stderr:\n{}",
        String::from_utf8_lossy(&old.stderr),
    );
    let real = fx.run(&provision);
    assert_eq!(
        old.status.code(),
        real.status.code(),
        "the door answers a locked registration the same way with the verdict lines and \
         without them",
    );
    assert_eq!(bytes_under(&fx.admin(SUBS[1])), before);
}

/// **The mirror**: jigc's own registrations, seen from a path that is not the one they were
/// made at — a repository moved with `mv`, or the same repository at another mount.
///
/// The door refuses there today and it refused before (the checkouts hold staged code nothing
/// can vouch for). What changed is that the refusal no longer destroys the records on its way
/// out — so git's own `worktree repair` still has something to mend, and the staged work
/// comes back staged.
#[test]
fn a_moved_repository_keeps_the_registrations_repair_needs() {
    let mut fx = Fixture::mint("moved");
    fx.provision_with_staged_code();
    let names = fx.admin_names();

    let moved = fx.root.path().join("repo-moved");
    fs::rename(&fx.repo, &moved).expect("mv the repository");
    fx.repo = moved;

    let refused = fx.run(&["milestone", "provision", MILESTONE]);
    let stderr = String::from_utf8_lossy(&refused.stderr).into_owned();
    assert!(
        !refused.status.success() && stderr.contains("milestone.leftover-holds-work"),
        "provision in the moved repository refuses over checkouts it cannot vouch for; got \
         {:?}\nstderr:\n{stderr}",
        refused.status,
    );
    assert_eq!(
        fx.admin_names(),
        names,
        "the refusal said nothing was removed — so the registrations naming the old path \
         must still be there",
    );

    // git's own repair, which needs exactly those records.
    let paths: Vec<PathBuf> = SUBS.iter().map(|sub| fx.worktree(sub)).collect();
    let mut repair = vec!["worktree", "repair"];
    repair.extend(paths.iter().map(|path| path.to_str().unwrap()));
    git_ok(&fx.repo, &repair);
    for sub in SUBS {
        assert!(
            git_ok(&fx.worktree(sub), &["status", "--porcelain"])
                .contains(&format!("A  {sub}.txt")),
            "`{sub}`: the repaired worktree must still hold its staged file, staged",
        );
    }

    // …and the milestone goes on from there: reuse, then land.
    fx.jigc_ok(&["milestone", "provision", MILESTONE]);
    fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
    let tree = git_ok(&fx.repo, &["ls-tree", "-r", "--name-only", "HEAD"]);
    for sub in SUBS {
        assert!(
            tree.lines().any(|path| path == format!("{sub}.txt")),
            "`{sub}`: the staged code must land at the boundary; HEAD holds:\n{tree}",
        );
    }
}

/// **`uninstall` takes every registration under its own worktrees root, by path** — a
/// sub-task's, and a dedicated combine worktree a crashed boundary left behind.
#[test]
fn uninstall_drops_the_registrations_under_its_own_worktrees_root() {
    let fx = Fixture::mint("uninstall-own");
    fx.jigc_ok(&["milestone", "provision", MILESTONE]);
    // A crashed boundary's dedicated worktree, its directory since deleted by hand.
    let combine = fx.repo.join(".jigc").join("worktrees").join(".combine-1-2");
    git_ok(
        &fx.repo,
        &[
            "worktree",
            "add",
            "-q",
            "--detach",
            combine.to_str().unwrap(),
            "HEAD",
        ],
    );
    fs::remove_dir_all(&combine).expect("delete the combine directory");
    let foreign = plant_foreign(&fx);

    let out = fx.run(&["uninstall", "--force"]);
    assert!(
        out.status.success(),
        "`jigc uninstall --force` must exit 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    let mut expected: Vec<String> = FOREIGN.iter().map(|name| (*name).to_owned()).collect();
    expected.sort();
    assert_eq!(
        fx.admin_names(),
        expected,
        "every registration under `.jigc/worktrees/` must be gone, and nothing else",
    );
    assert_foreign_untouched(&fx, &foreign, "uninstall over its own registrations");
}

// ---------------------------------------------------------------------------
// The remedy a leaked worktree prints.
// ---------------------------------------------------------------------------

/// The backticked spans of the leaked-worktree warning's `remedy:` line for `sub`, in the
/// order printed.
fn remedy_spans(stderr: &str, sub: &str) -> Vec<String> {
    let mut lines = stderr.lines().skip_while(|line| {
        !(line.starts_with("warning: could not remove the fan-out worktree") && line.contains(sub))
    });
    lines
        .next()
        .unwrap_or_else(|| panic!("no leaked-worktree warning for `{sub}`; stderr:\n{stderr}"));
    let remedy = lines
        .find(|line| line.trim_start().starts_with("remedy:"))
        .unwrap_or_else(|| panic!("the warning for `{sub}` carries no remedy; stderr:\n{stderr}"));
    remedy
        .split('`')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, span)| span.to_owned())
        .collect()
}

/// **The leaked-worktree remedy names the one registration, and works as printed.**
///
/// It used to read *run `git worktree prune`, then `git worktree remove --force <path>`* —
/// which sent the reader to the repository-wide prune by their own hand (and after it the
/// second command failed: the record it names was already gone). Two causes a teardown can
/// leak a worktree for, each at both teardown doors: a **locked** worktree, where the printed
/// spans are the whole repair, and a checkout that lost its `.git` link, where the reader
/// first deals with the directory git will not delete.
#[test]
fn the_leaked_worktree_remedy_names_one_registration_and_works_as_printed() {
    for cause in ["locked", "unlinked"] {
        for door in ["finalize", "discard"] {
            let cell = format!("milestone {door} × {cause}");
            let fx = Fixture::mint("remedy");
            fx.provision_with_staged_code();
            let leaked = fx.worktree(SUBS[0]);
            match cause {
                "locked" => {
                    git_ok(&fx.repo, &["worktree", "lock", leaked.to_str().unwrap()]);
                }
                _ => {
                    if door == "finalize" {
                        // The boundary refuses, **before it lands**, over a registration
                        // whose index holds a path it would not carry — and with the link
                        // gone this checkout's staged file is exactly that
                        // (`worktree_registration_anchor.rs`, the unlinked cell). So the
                        // leak a landed teardown can still meet is over a registration
                        // that holds nothing: the sub-agent's file is on disk, unstaged.
                        git_ok(&leaked, &["reset", "-q"]);
                    }
                    fs::remove_file(leaked.join(".git")).expect("unlink the checkout");
                }
            }
            let foreign = plant_foreign(&fx);

            let args: Vec<&str> = match door {
                "finalize" => vec!["milestone", "finalize", MILESTONE],
                _ => vec!["milestone", "discard", MILESTONE, "--force"],
            };
            let out = fx.run(&args);
            let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
            assert!(
                out.status.success(),
                "{cell}: a leaked worktree never changes the door's exit; got {:?}\n\
                 stderr:\n{stderr}",
                out.status,
            );
            assert!(
                !stderr.contains("worktree prune"),
                "{cell}: no remedy may send the reader to a repository-wide prune; \
                 stderr:\n{stderr}",
            );
            assert!(
                fx.admin(SUBS[0]).is_dir(),
                "{cell}: fixture — the leaked worktree must still be registered",
            );
            assert!(
                leaked.join("README.md").is_file(),
                "{cell}: …and a door that could not remove it must have left its bytes",
            );

            let spans = remedy_spans(&stderr, SUBS[0]);
            assert!(
                spans
                    .last()
                    .is_some_and(|span| span.contains("worktree remove --force")
                        && span.contains(leaked.to_str().unwrap())),
                "{cell}: the remedy must end at the one registration, by path; got {spans:?}",
            );
            if cause == "unlinked" {
                // The step the remedy asks of the reader: git will not delete a directory it
                // no longer reads as a worktree, so they move it out of the way themselves.
                fs::rename(&leaked, away(&leaked)).expect("move the directory aside");
            }
            // Run the printed bytes verbatim, through a real shell, from OUTSIDE the
            // repository — the spans are aimed, so that must not matter.
            for span in &spans {
                let ran = Command::new("sh")
                    .arg("-c")
                    .arg(span)
                    .current_dir(fx.home.path())
                    .output()
                    .expect("run the printed span");
                assert!(
                    ran.status.success(),
                    "{cell}: the printed span `{span}` must run as printed; stderr:\n{}",
                    String::from_utf8_lossy(&ran.stderr),
                );
            }
            assert!(
                !fx.admin(SUBS[0]).exists(),
                "{cell}: after the remedy the leaked registration must be gone",
            );
            assert_foreign_untouched(&fx, &foreign, &cell);
        }
    }
}

// ---------------------------------------------------------------------------
// The mechanism, fenced where membership is decided.
// ---------------------------------------------------------------------------

/// Every production `git worktree <verb>` argv, by the function that holds it.
///
/// The class is *a jigc door mutating git's worktree registry*, and the doors above are the
/// members that exist today. This table is what stops the next one: a `git worktree prune`
/// added anywhere, or a `git worktree remove` outside the one home that checks the path is
/// jigc's own, is a site with no row here and reddens the fence below.
const WORKTREE_ADMIN_SITES: &[(&str, &str, &str, &str)] = &[
    (
        "crates/cli/src/milestone.rs",
        "worktree_registrations",
        "list",
        "the one read of git's registry — every door's registered set, `prunable` and `locked` \
         included, comes from here",
    ),
    (
        "crates/cli/src/milestone.rs",
        "provision_worktrees",
        "add",
        "a sub-task's worktree, at `<jigc_home>/.jigc/worktrees/<sub-task-id>` — a path the \
         door builds itself",
    ),
    (
        "crates/cli/src/milestone.rs",
        "remove_owned_registration",
        "remove",
        "THE one home of a registration removal: by path, and only for a direct child of \
         `<root>/.jigc/worktrees/` — it refuses any other path before git is asked",
    ),
    (
        "crates/engine/src/finding.rs",
        "migrate_operand",
        "stash",
        "NOT an argv, and listed so the scan's one false positive is stated rather than \
         filtered: `GIT_COMMAND_GROUPS`, the aim fence's word list of git command groups, \
         where `worktree` and `stash` are neighbours (the const follows `migrate_operand`, \
         which is why the scan names that function)",
    ),
    (
        "crates/cli/src/task.rs",
        "add",
        "add",
        "`DedicatedWorktree::add` — the boundary's throwaway checkout at \
         `.jigc/worktrees/.combine-<pid>-<nanos>`, a name no earlier run can have left a \
         record at",
    ),
];

/// The verbs of `git worktree` that **remove or rewrite** a registration. `list` and `add`
/// are not here: neither can touch a record that is not the caller's own path.
const REGISTRY_MUTATORS: [&str; 5] = ["prune", "remove", "move", "repair", "unlock"];

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the workspace root is two levels above crates/cli")
        .to_path_buf()
}

/// Every production `"worktree"` argv element in the two crates, with the argv element that
/// follows it: `(file, enclosing fn, verb)`.
fn worktree_argv_sites() -> Vec<(String, String, String)> {
    let mut sites = Vec::new();
    for dir in ["crates/cli/src", "crates/engine/src"] {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            let body = fs::read_to_string(&path).expect("read source");
            let code = rust_source::code_only(&body);
            let regions = rust_source::cfg_test_regions(&code);
            let literals = rust_source::string_literals(&body);
            for (i, literal) in literals.iter().enumerate() {
                if literal.value != "worktree"
                    || rust_source::is_test_domain(&path, &regions, literal.offset)
                {
                    continue;
                }
                // The next literal is the verb only when it sits in the same argv — on the
                // same or the following line, with nothing but punctuation between.
                let Some(next) = literals.get(i + 1) else {
                    continue;
                };
                let between = &code[literal.offset..next.offset];
                if between.chars().any(|c| c.is_alphanumeric() || c == ';') {
                    continue;
                }
                let rel = path
                    .strip_prefix(workspace_root())
                    .expect("under the workspace")
                    .to_string_lossy()
                    .into_owned();
                let owner = rust_source::enclosing_fn(&code, literal.offset)
                    .unwrap_or("<top level>")
                    .to_owned();
                sites.push((rel, owner, next.value.clone()));
            }
        }
    }
    sites
}

/// **No production source runs a repository-wide `git worktree prune`, and a registration is
/// removed in exactly one place.**
#[test]
fn every_production_git_worktree_argv_is_a_row_and_none_is_a_prune() {
    let sites = worktree_argv_sites();
    assert!(
        !sites.is_empty(),
        "the scan found no `git worktree` argv at all — it is reading the wrong tree",
    );

    let mut unlisted: Vec<String> = Vec::new();
    for (file, owner, verb) in &sites {
        assert_ne!(
            verb, "prune",
            "{file}::{owner} runs `git worktree prune` — a repository-wide act that drops \
             the registration of every worktree whose directory is not where git recorded \
             it, foreign ones included. Remove one registration by path through \
             `cli::milestone::remove_owned_registration` instead",
        );
        let listed = WORKTREE_ADMIN_SITES
            .iter()
            .any(|(f, o, v, _)| f == file && o == owner && v == verb);
        if !listed {
            unlisted.push(format!("  {file}::{owner}: `git worktree {verb}`"));
        }
    }
    assert!(
        unlisted.is_empty(),
        "these production sites run a `git worktree` verb and are in no row of \
         `WORKTREE_ADMIN_SITES` — a door that touches git's registry states which paths it \
         reaches:\n{}",
        unlisted.join("\n"),
    );

    // The table cannot go stale either: a row nothing in the source matches is a sentence.
    for (file, owner, verb, reason) in WORKTREE_ADMIN_SITES {
        assert!(
            !reason.trim().is_empty(),
            "{file}::{owner} carries no reason"
        );
        assert!(
            sites
                .iter()
                .any(|(f, o, v)| f == file && o == owner && v == verb),
            "`WORKTREE_ADMIN_SITES` names {file}::{owner} running `git worktree {verb}`, \
             and no such argv is there",
        );
    }
    // Exactly one mutator, in exactly one function.
    let mutators: Vec<&(String, String, String)> = sites
        .iter()
        .filter(|(_, _, verb)| REGISTRY_MUTATORS.contains(&verb.as_str()))
        .collect();
    assert_eq!(
        mutators.len(),
        1,
        "git's worktree registry is mutated from one production home; got {mutators:?}",
    );
}

/// **No shipped text recommends the repository-wide prune either** — not a route, not a
/// warning's remedy, not a guide, a pack step or an adapter file. A remedy that sends the
/// reader there walks them into this class's harm by their own hand.
#[test]
fn no_shipped_text_recommends_a_repository_wide_worktree_prune() {
    let mut offenders: Vec<String> = Vec::new();
    for dir in ["crates/cli/src", "crates/engine/src"] {
        for path in rust_source::rust_files(&workspace_root().join(dir)) {
            let body = fs::read_to_string(&path).expect("read source");
            let code = rust_source::code_only(&body);
            let regions = rust_source::cfg_test_regions(&code);
            for literal in rust_source::string_literals(&body) {
                if literal.value.contains("worktree prune")
                    && !rust_source::is_test_domain(&path, &regions, literal.offset)
                {
                    offenders.push(format!(
                        "  {}::{}: {:?}",
                        path.display(),
                        rust_source::enclosing_fn(&code, literal.offset).unwrap_or("<top level>"),
                        literal.value,
                    ));
                }
            }
        }
    }
    for dir in [
        "crates/cli/packs",
        "crates/cli/adapters",
        "crates/cli/guides",
    ] {
        for path in crate::support::root_walk::files(&workspace_root().join(dir), |_| true) {
            let Ok(body) = fs::read_to_string(&path) else {
                continue;
            };
            if body.contains("worktree prune") {
                offenders.push(format!("  {}", path.display()));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "these shipped texts name `git worktree prune` — name the one registration instead \
         (`git -C <checkout> worktree remove --force <path>`):\n{}",
        offenders.join("\n"),
    );
}
