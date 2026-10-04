//! **A door answers for what git's registration of a sub-task worktree still holds** — the
//! leftover probe's third leg (the rc.24 fix pass, the own-set sibling of L-22;
//! `design/team-ready-state.md` → *Abandon refuses on a dirty worktree*, the registration's
//! own content).
//!
//! A fan-out worktree is two things: a checkout under `.jigc/worktrees/<sub-task-id>` and
//! git's registration of it under `.git/worktrees/<name>/`, which carries that checkout's
//! `HEAD`, its index and its reflog. The doors that remove the first also drop the second —
//! and until this fix they asked only about the first. Driven on `jigc 1.0.0-rc.24`:
//!
//!   * the **directory gone** (deleted by hand, by an agent's `rm -r`, by a crashed run) with
//!     the sub-agent's `git add`-ed file still in the registration's index:
//!     `jigc milestone provision` printed `provisioned 2 worktree(s)` at exit 0 over the
//!     destroyed deliverable, `jigc milestone discard` and `jigc uninstall` took it at exit 0,
//!     and a landed `jigc milestone finalize` reported that sub-task as
//!     `nothing staged, no worktree provisioned` — every word of which was false;
//!   * a **live, clean** worktree whose sub-agent *committed* instead of staging:
//!     `discard`, `uninstall` and `finalize` each exited 0 and left the commit reachable from
//!     nothing.
//!
//! And the record alone restores the work: with the directory gone, three commands bring the
//! checkout back from git's registration, staged paths still staged, through to a landed
//! boundary. A door that drops that record in silence destroys something that was
//! recoverable a moment before.
//!
//! # The class, and what iterates it
//!
//! *A jigc door drops one of its own registrations whose `HEAD` or index anchors work no ref
//! reaches.* Three axes cross, and every arm below is derived from them rather than listed:
//!
//!   * **the door axis** — `cli::milestone::WORKTREE_DOORS`, read code-side, split by
//!     `DestroyingDoor::consent`: a refusing member is driven without its consent and with
//!     it, the landed boundary (no consent to offer) once per commit arm;
//!   * **the holding axis** — [`HOLDINGS`]: a commit only the worktree's `HEAD` reaches, a
//!     path staged in its index, both, and the **empty** control (HEAD at the base pin,
//!     nothing staged), which must keep clearing at exit 0 — the idempotent re-provision;
//!   * **the standing axis** — [`STANDINGS`]: the checkout **live**, or its directory
//!     **gone** while the registration stands (git's `prunable`).
//!
//! One rule decides every cell ([`reaches`]): *does this door drop this registration?*
//! `provision` reuses a live registered worktree untouched, so it drops nothing there and
//! must neither refuse nor narrate; every other (door, standing) pair drops it. A refusing
//! door **refuses** exactly where it reaches work; under consent, and at the landed boundary,
//! the door **names what it took, and nothing it did not** — asserted against what git still
//! holds afterwards, never against jigc's words alone.
//!
//! Every command a refusal prints is run **as printed**, through a real shell, from outside
//! the repository, and followed to where the sub-agent's work is: restored, kept under a
//! ref, or landed by the boundary.

use cli::milestone::{
    DestroyingDoor, FINALIZE_DOOR, PROVISION_DOOR, UNINSTALL_DOOR, WORKTREE_DOORS,
};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The milestone every fixture mints.
const MILESTONE: &str = "cache-rework";
/// The sub-task whose worktree every cell plants in.
const SUB: &str = "area-low";
/// Its sibling — a live worktree with staged code, so a boundary has something to land.
const SIBLING: &str = "area-zed";

/// The file a planted **commit** carries, and its bytes.
const COMMITTED: (&str, &str) = ("committed-by-the-sub-agent.rs", "fn committed() {}\n");
/// The file a planting **stages**, and its bytes.
const STAGED: (&str, &str) = ("staged-by-the-sub-agent.rs", "fn staged() {}\n");

/// What the sub-task's registration holds beyond the base pin.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Holding {
    /// A commit made inside the worktree — reachable from its detached `HEAD` and nothing else.
    Commit,
    /// A path `git add`-ed and never committed — the fan-out's ordinary shape.
    Staged,
    /// Both of the above.
    Both,
    /// Neither: `HEAD` at the base pin, index equal to it. The control.
    Empty,
}

const HOLDINGS: [Holding; 4] = [
    Holding::Commit,
    Holding::Staged,
    Holding::Both,
    Holding::Empty,
];

impl Holding {
    fn commit(self) -> bool {
        matches!(self, Holding::Commit | Holding::Both)
    }

    fn staged(self) -> bool {
        matches!(self, Holding::Staged | Holding::Both)
    }

    fn any(self) -> bool {
        self != Holding::Empty
    }
}

/// Whether the checkout the registration belongs to is on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Standing {
    /// The directory is gone and the registration stands — `prunable` to git.
    Stale,
    /// The checkout is where git recorded it.
    Live,
}

const STANDINGS: [Standing; 2] = [Standing::Stale, Standing::Live];

/// A throwaway directory that removes itself on drop (the project's no-tempfile pattern).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-wt-anchor-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        // Canonical, because every path a refusal prints is: git records realpaths at
        // `worktree add`, and on macOS the temp root is a symlink.
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

/// What a planting left behind, read back from git before any door ran.
struct Planted {
    holding: Holding,
    /// The milestone's base pin — where the worktree was provisioned.
    base: String,
    /// The registration's `HEAD` as planted: the commit when one was made, the base otherwise.
    head: String,
    /// Every file of the registration, with its bytes — the before-control a refusal is held
    /// to.
    admin: BTreeMap<String, Vec<u8>>,
}

/// A repo carrying `milestone:cache-rework` with two provisioned sub-tasks.
struct Fixture {
    _root: TempDir,
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
            _root: root,
            home: TempDir::new("home"),
            repo,
        };
        fx.jigc_ok(&["milestone", "create", "Cache rework"]);
        for intent in ["Area low", "Area zed"] {
            fx.jigc_ok(&["milestone", "add-task", MILESTONE, intent]);
        }
        fx.jigc_ok(&["milestone", "provision", MILESTONE]);
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

    /// The sub-task's path as every surface prints it — repo-relative (law 1).
    fn printed(&self) -> String {
        format!(".jigc/worktrees/{SUB}")
    }

    /// git's admin directory for the sub-task worktree `sub`.
    fn admin(&self, sub: &str) -> PathBuf {
        self.repo.join(".git").join("worktrees").join(sub)
    }

    fn run(&self, args: &[&str]) -> Output {
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

    /// Give `sub` something a boundary lands: one staged file in its worktree, and the
    /// authored `commit:<sub>` doc the chain arm renders its per-sub-task commit from.
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

    /// Plant `holding` in [`SUB`]'s worktree, then make it `standing`.
    ///
    /// The stale state is made by **deleting** the directory — the precondition the finding
    /// states (a hand `rm -r`, a crashed run), and the one where nothing but git's
    /// registration is left to hold the work.
    fn plant(&self, holding: Holding, standing: Standing) -> Planted {
        let worktree = self.worktree(SUB);
        let base = git_ok(&worktree, &["rev-parse", "HEAD"]);
        if holding.commit() {
            fs::write(worktree.join(COMMITTED.0), COMMITTED.1).expect("write");
            git_ok(&worktree, &["add", COMMITTED.0]);
            git_ok(
                &worktree,
                &["commit", "-q", "-m", "the sub-agent committed"],
            );
        }
        if holding.staged() {
            fs::write(worktree.join(STAGED.0), STAGED.1).expect("write");
            git_ok(&worktree, &["add", STAGED.0]);
        }
        let head = git_ok(&worktree, &["rev-parse", "HEAD"]);
        if standing == Standing::Stale {
            fs::remove_dir_all(&worktree).expect("delete the worktree directory");
        }

        // The before-controls: the fixture is in the state the cell claims to drive.
        let listed = git_ok(&self.repo, &["worktree", "list", "--porcelain"]);
        let record = listed
            .split("\n\n")
            .find(|record| {
                record.lines().next() == Some(&format!("worktree {}", worktree.display()))
            })
            .unwrap_or_else(|| panic!("fixture: git must list `{SUB}`; got:\n{listed}"));
        assert_eq!(
            record.contains("prunable"),
            standing == Standing::Stale,
            "fixture: `{SUB}` must be {standing:?} to git before the door runs; got:\n{record}",
        );
        assert_eq!(
            head != base,
            holding.commit(),
            "fixture: the registration's HEAD is off the base pin iff a commit was planted",
        );
        if holding.commit() {
            assert_eq!(
                git_ok(&self.repo, &["for-each-ref", "--contains", &head]),
                "",
                "fixture: no ref may reach the planted commit — its only anchor is the \
                 worktree's HEAD",
            );
        }
        let planted = Planted {
            holding,
            base,
            head,
            admin: bytes_under(&self.admin(SUB)),
        };
        assert!(
            self.still_held(&planted),
            "fixture: the registration must hold the planted {holding:?} before the door runs",
        );
        planted
    }

    /// Take every subject `door` answers for **other than the worktree one** out of the
    /// fixture, so a cell's verdict is about the registration and nothing else.
    ///
    /// `jigc uninstall` also refuses over a workbench file no index has a copy of, and
    /// `jigc milestone provision` has just amended `.jigc/.gitignore` — a file this bare
    /// fixture never committed (a real install commits it). Staging it is the route that
    /// refusal itself prints.
    fn isolate_the_worktree_subject(&self, door: &DestroyingDoor) {
        if door.verb == UNINSTALL_DOOR.verb {
            git_ok(&self.repo, &["add", "--", ".jigc/.gitignore"]);
        }
    }

    /// Whether git's registration of [`SUB`] **still holds what was planted** — the witness
    /// every *named ⇔ taken* assertion reads. Asked of the registration itself (its `HEAD`
    /// and its index, through the admin directory), so it answers the same way whether or
    /// not a checkout stands at the path, and is not fooled by a door that dropped the
    /// record and registered a fresh one under the same name.
    fn still_held(&self, planted: &Planted) -> bool {
        let admin = self.admin(SUB);
        if !admin.is_dir() {
            return false;
        }
        let git_dir = format!("--git-dir={}", admin.display());
        let head = git(&self.repo, &[&git_dir, "rev-parse", "--verify", "HEAD"]);
        if String::from_utf8_lossy(&head.stdout).trim() != planted.head {
            return false;
        }
        if planted.holding.staged() {
            let staged = git(&self.repo, &[&git_dir, "diff", "--cached", "--name-only"]);
            if !String::from_utf8_lossy(&staged.stdout)
                .lines()
                .any(|path| path == STAGED.0)
            {
                return false;
            }
        }
        true
    }
}

/// Every file under `dir`, relative, with its bytes — a registration read back **whole**.
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

/// The code `door` refuses with over a **worktree-shaped** subject — selected by the subject
/// it answers for, never by its position in the set (the `leftover_operation_in_progress`
/// convention).
fn worktree_code(door: &DestroyingDoor) -> &'static str {
    door.codes
        .iter()
        .copied()
        .find(|code| code.ends_with(".leftover-holds-work") || code.ends_with(".dirty-worktree"))
        .unwrap_or_else(|| {
            panic!(
                "`{}` removes a worktree-shaped path, so it owes a refusal code over that \
                 subject; got {:?}",
                door.verb, door.codes,
            )
        })
}

/// The argv that stands at `door`, **derived from the door's own `verb`** rather than
/// hand-listed.
fn argv_at(door: &DestroyingDoor, force: bool, json: bool) -> Vec<String> {
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
        argv.push(
            door.consent()
                .expect("only a refusing door is driven with its consent")
                .to_owned(),
        );
    }
    if json {
        argv.extend(["--format".to_owned(), "json".to_owned()]);
    }
    argv
}

/// **The one rule every cell is derived from**: does `door` drop the registration of a
/// sub-task worktree that is `standing`?
///
/// `jigc milestone provision` reuses a registered, live worktree untouched — the idempotent
/// re-provision every fan-out depends on — so it drops nothing there. It drops a stale one
/// (to `git worktree add` a fresh checkout at the path). The teardown behind `discard` and
/// the landed `finalize`, and `uninstall`, drop the registration either way.
fn reaches(door: &DestroyingDoor, standing: Standing) -> bool {
    !(door.verb == PROVISION_DOOR.verb && standing == Standing::Live)
}

/// The refusing members of the door axis, read code-side.
fn refusing_doors() -> Vec<&'static DestroyingDoor> {
    let doors: Vec<&'static DestroyingDoor> = WORKTREE_DOORS
        .into_iter()
        .filter(|door| door.consent().is_some())
        .collect();
    assert!(
        doors.len() >= 3,
        "the worktree door table must carry its refusing members; got {}",
        doors.len(),
    );
    doors
}

fn run_door(
    fx: &Fixture,
    door: &DestroyingDoor,
    force: bool,
    json: bool,
) -> (Output, String, String) {
    let argv = argv_at(door, force, json);
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();
    let out = fx.run(&args);
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    (out, stdout, stderr)
}

/// The abbreviated commit a surface prints — the first seven hex digits are a prefix of
/// whatever abbreviation git or jigc chose.
fn short(sha: &str) -> &str {
    &sha[..7]
}

/// Assert `text` names exactly the parts of the planted holding that `expect` says it must:
/// the commit by its sha, the staged path by its name.
fn assert_names(text: &str, planted: &Planted, expect: bool, cell: &str, what: &str) {
    if planted.holding.commit() {
        assert_eq!(
            text.contains(short(&planted.head)),
            expect,
            "{cell}: {what} — the commit `{}` only this registration's HEAD reaches must{} be \
             named; got:\n{text}",
            short(&planted.head),
            if expect { "" } else { " NOT" },
        );
    }
    if planted.holding.staged() {
        assert_eq!(
            text.contains(STAGED.0),
            expect,
            "{cell}: {what} — the staged path `{}` must{} be named; got:\n{text}",
            STAGED.0,
            if expect { "" } else { " NOT" },
        );
    }
}

/// The backticked spans on the refusal line that names [`SUB`]'s path, in the order printed.
fn spans_on_the_held_line(stderr: &str, printed: &str) -> Vec<String> {
    let line = stderr
        .lines()
        .find(|line| line.trim_start().starts_with(&format!("{printed}:")))
        .unwrap_or_else(|| panic!("no refusal line names `{printed}`; stderr:\n{stderr}"));
    line.split('`')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, span)| span.to_owned())
        .collect()
}

/// Run `span` **as printed**, through a real shell, from outside the repository.
fn run_as_printed(fx: &Fixture, span: &str, cell: &str) {
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

// ---------------------------------------------------------------------------
// The refusing doors, without their consent.
// ---------------------------------------------------------------------------

/// **door × holding × standing, un-forced**: a door refuses exactly where it would drop a
/// registration that holds work, names what it holds, and leaves git's record byte-identical;
/// everywhere else it proceeds — the empty stale registration and the live reuse included.
#[test]
fn a_refusing_door_refuses_exactly_where_it_would_drop_a_registration_that_holds_work() {
    let mut refused = 0;
    let mut proceeded = 0;
    for door in refusing_doors() {
        for holding in HOLDINGS {
            for standing in STANDINGS {
                let cell = format!("{} × {holding:?} × {standing:?} × un-forced", door.verb);
                let fx = Fixture::mint("refuse");
                fx.isolate_the_worktree_subject(door);
                let planted = fx.plant(holding, standing);
                let (out, _, stderr) = run_door(&fx, door, false, false);

                let must_refuse = holding.any() && reaches(door, standing);
                assert_eq!(
                    !out.status.success(),
                    must_refuse,
                    "{cell}: {} — got {:?}\nstderr:\n{stderr}",
                    if must_refuse {
                        "the door would drop a registration that is the only thing holding \
                         this work, so it must refuse without `--force`"
                    } else {
                        "nothing this door drops holds work, so it must proceed (the \
                         idempotent re-provision, and the clean teardown)"
                    },
                    out.status,
                );
                if !must_refuse {
                    proceeded += 1;
                    // A door that proceeded un-forced took nothing that held work — so it
                    // has nothing to narrate, and what it did not reach is still there.
                    assert_names(
                        &stderr,
                        &planted,
                        false,
                        &cell,
                        "a clean run narrates no loss",
                    );
                    if !reaches(door, standing) {
                        assert!(
                            fx.still_held(&planted),
                            "{cell}: a reused worktree's registration must be exactly as \
                             the sub-agent left it",
                        );
                    }
                    continue;
                }
                refused += 1;

                let code = worktree_code(door);
                assert!(
                    stderr.contains(&format!("blocking · {code}")),
                    "{cell}: the refusal must carry the door's EXISTING code `{code}`; \
                     stderr:\n{stderr}",
                );
                assert!(
                    stderr.contains(&fx.printed()),
                    "{cell}: the refusal must name WHICH path; stderr:\n{stderr}",
                );
                assert_names(
                    &stderr,
                    &planted,
                    true,
                    &cell,
                    "the refusal says what it holds",
                );
                assert!(
                    stderr.contains("--force"),
                    "{cell}: `--force` is the one consent, and the refusal names it; \
                     stderr:\n{stderr}",
                );
                assert_eq!(
                    bytes_under(&fx.admin(SUB)),
                    planted.admin,
                    "{cell}: a refusal leaves git's registration byte-identical — its HEAD, \
                     its index and its reflog",
                );
                if standing == Standing::Live {
                    assert!(
                        fx.worktree(SUB).join(".git").exists(),
                        "{cell}: a refusal leaves the live checkout standing",
                    );
                }
            }
        }
    }
    // Neither half of the axis is empty: without a refusing cell this arm proves nothing,
    // and without a proceeding one it cannot tell a guard from a wall.
    assert!(
        refused > 0 && proceeded > 0,
        "{refused} refused, {proceeded} proceeded"
    );
}

// ---------------------------------------------------------------------------
// The refusing doors, with their consent.
// ---------------------------------------------------------------------------

/// **door × holding × standing, `--force`**: the consent buys the removal, never the
/// silence — *named ⇔ taken*, asked of what git still holds afterwards.
#[test]
fn consent_takes_the_registration_and_names_exactly_what_it_took() {
    let mut taken_cells = 0;
    let mut kept_cells = 0;
    for door in refusing_doors() {
        for holding in HOLDINGS {
            for standing in STANDINGS {
                let cell = format!("{} × {holding:?} × {standing:?} × --force", door.verb);
                let fx = Fixture::mint("force");
                fx.isolate_the_worktree_subject(door);
                let planted = fx.plant(holding, standing);
                let (out, _, stderr) = run_door(&fx, door, true, false);
                assert!(
                    out.status.success(),
                    "{cell}: `--force` is the one consent past this guard, so it must \
                     proceed; got {:?}\nstderr:\n{stderr}",
                    out.status,
                );
                if !holding.any() {
                    // The control: nothing was held, so there is nothing to have taken and
                    // nothing to name. (An empty registration re-added fresh at the base
                    // pin is indistinguishable from the one it replaced — which is exactly
                    // why dropping it is harmless.)
                    assert!(
                        !stderr.contains("git's registration"),
                        "{cell}: an empty registration goes without a word; stderr:\n{stderr}",
                    );
                    continue;
                }
                let taken = !fx.still_held(&planted);
                assert_eq!(
                    taken,
                    reaches(door, standing),
                    "{cell}: the door drops this registration iff it reaches it",
                );
                if taken {
                    taken_cells += 1;
                } else {
                    kept_cells += 1;
                }
                assert_names(
                    &stderr,
                    &planted,
                    taken,
                    &cell,
                    "named ⇔ taken: the door says what it destroyed, and claims no \
                     destruction it did not perform",
                );
            }
        }
    }
    assert!(
        taken_cells > 0 && kept_cells > 0,
        "{taken_cells} taken, {kept_cells} kept"
    );
}

// ---------------------------------------------------------------------------
// The landed boundary: it cannot refuse after landing, so it names.
// ---------------------------------------------------------------------------

/// The keys of one `committed.sub_tasks[]` entry — the pinned `--format json` shape this fix
/// must not move.
const SUB_TASK_KEYS: [&str; 7] = [
    "code_files",
    "discarded",
    "docs",
    "hash",
    "id",
    "provisioned",
    "worktree_unreadable",
];

/// **`jigc milestone finalize` × holding × standing × format**, on one commit arm: the
/// boundary lands what the live worktrees staged, drops every registration with the
/// teardown, and names what went with one — the path, the commit, the staged paths. Its
/// contribution line says nothing false about a sub-task whose worktree directory is gone.
///
/// One arm per test, so the two halves of the commit-arm axis run side by side.
fn landed_boundary_cells(squash: bool) {
    assert!(
        FINALIZE_DOOR.consent().is_none(),
        "the landed boundary offers no consent — which is why it narrates rather than refuses",
    );
    let mut named_cells = 0;
    {
        for json in [false, true] {
            for holding in HOLDINGS {
                for standing in STANDINGS {
                    let cell = format!(
                        "{} × {holding:?} × {standing:?} × squash: {squash} × {}",
                        FINALIZE_DOOR.verb,
                        if json { "json" } else { "text" },
                    );
                    let fx = Fixture::mint("landed");
                    if !squash {
                        fx.set_squash_false();
                    }
                    fx.stage_code(SIBLING);
                    let planted = fx.plant(holding, standing);
                    if standing == Standing::Live && holding.staged() {
                        // A live sub-task's staged code lands — give the chain arm the
                        // commit doc it renders that sub-task's commit from.
                        let docs = fx.repo.join(".jigc").join("tasks").join(SUB).join("docs");
                        fs::create_dir_all(&docs).expect("mk docs/");
                        fs::write(
                            docs.join(format!("commit:{SUB}.md")),
                            format!(
                                "---\ntype: feat\n---\n\n# {SUB}\n\n## Summary\n\nadd the \
                                 staged file\n\n## Body\n\n\n\n## Trailers\n"
                            ),
                        )
                        .expect("write the staged commit doc");
                        fs::write(
                            docs.join("provenance.json"),
                            format!(
                                "{{\n  \"docs\": {{\n    \"commit:{SUB}\": \"created\"\n  }}\n}}\n"
                            ),
                        )
                        .expect("write the provenance manifest");
                    }

                    let (out, stdout, stderr) = run_door(&fx, &FINALIZE_DOOR, false, json);
                    assert!(
                        out.status.success(),
                        "{cell}: the sibling staged code, so the boundary lands; got {:?}\n\
                         stderr:\n{stderr}",
                        out.status,
                    );
                    assert!(
                        !fx.admin(SUB).exists(),
                        "{cell}: the landed teardown drops the sub-task's registration",
                    );

                    // What landed: the live worktree's staged path, and nothing a stale
                    // registration held — the boundary reads live checkouts only.
                    let tree = git_ok(&fx.repo, &["ls-tree", "-r", "--name-only", "HEAD"]);
                    let landed = |file: &str| tree.lines().any(|path| path == file);
                    assert!(
                        landed(&format!("{SIBLING}.txt")),
                        "{cell}: the sibling lands"
                    );
                    let staged_landed = landed(STAGED.0);
                    assert_eq!(
                        staged_landed,
                        holding.staged() && standing == Standing::Live,
                        "{cell}: a staged path lands iff its checkout was live",
                    );
                    assert!(
                        !landed(COMMITTED.0),
                        "{cell}: a commit made inside a worktree is not what the boundary \
                         lands — it reads the staged set",
                    );

                    // Named ⇔ dropped without landing. On stderr, in both formats.
                    if holding.commit() {
                        named_cells += 1;
                        assert!(
                            stderr.contains(short(&planted.head)) && stderr.contains(&fx.printed()),
                            "{cell}: the commit only that registration's HEAD reached went \
                             with it, so the boundary must name it and its path; \
                             stderr:\n{stderr}",
                        );
                    }
                    if holding.staged() {
                        assert_eq!(
                            stderr.contains(STAGED.0),
                            !staged_landed,
                            "{cell}: a staged path is named as lost iff it did not land; \
                             stderr:\n{stderr}",
                        );
                    }
                    if !holding.any() {
                        assert!(
                            !stderr.contains(&fx.printed()),
                            "{cell}: an empty registration goes without a word — there is \
                             nothing to name; stderr:\n{stderr}",
                        );
                    }

                    if json {
                        // The pinned envelope does not move: same keys, and `provisioned`
                        // keeps meaning *a live worktree stood there*.
                        let doc: serde_json::Value =
                            serde_json::from_str(&stdout).unwrap_or_else(|err| {
                                panic!("{cell}: stdout is JSON: {err}\n{stdout}")
                            });
                        let entry = doc["committed"]["sub_tasks"]
                            .as_array()
                            .and_then(|subs| subs.iter().find(|sub| sub["id"] == SUB))
                            .unwrap_or_else(|| panic!("{cell}: `{SUB}` is in sub_tasks\n{stdout}"));
                        let mut keys: Vec<&str> = entry
                            .as_object()
                            .expect("an object")
                            .keys()
                            .map(String::as_str)
                            .collect();
                        keys.sort_unstable();
                        assert_eq!(
                            keys, SUB_TASK_KEYS,
                            "{cell}: the pinned `committed.sub_tasks[]` key set must not move",
                        );
                        assert_eq!(
                            entry["provisioned"],
                            serde_json::json!(standing == Standing::Live),
                            "{cell}: `provisioned` is *a live worktree stood at the path*",
                        );
                    } else {
                        // The contribution line: true of a sub-task whose directory is gone.
                        let line = stdout
                            .lines()
                            .find(|line| line.trim_start().starts_with("sub-tasks:"))
                            .unwrap_or_else(|| panic!("{cell}: no sub-tasks line\n{stdout}"));
                        let entry = line
                            .split(" · ")
                            .find(|part| part.contains(&format!("{SUB}:")))
                            .unwrap_or_else(|| panic!("{cell}: `{SUB}` is on the line\n{line}"));
                        if standing == Standing::Stale {
                            assert!(
                                !entry.contains("no worktree provisioned"),
                                "{cell}: a worktree WAS provisioned and its registration \
                                 stood — `no worktree provisioned` is false here; got: {entry}",
                            );
                            assert!(
                                entry.contains("directory is gone"),
                                "{cell}: the line must say what is true — the worktree's \
                                 directory is gone; got: {entry}",
                            );
                            assert!(
                                !entry.contains("nothing staged"),
                                "{cell}: `nothing staged` is a measurement of a checkout, \
                                 and there was none to measure; got: {entry}",
                            );
                            assert_names(
                                entry,
                                &planted,
                                true,
                                &cell,
                                "the contribution line says what the registration held",
                            );
                        } else {
                            assert!(
                                !entry.contains("directory is gone"),
                                "{cell}: a live worktree's directory is not gone; got: {entry}",
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(
        named_cells > 0,
        "no cell drove a named commit — this arm proved nothing"
    );
}

/// The single aggregate commit (`finalize.fan-out.squash: true`, the default).
#[test]
fn the_landed_boundary_names_what_goes_with_a_registration_it_drops_squash() {
    landed_boundary_cells(true);
}

/// One commit per sub-task, then the aggregate (`finalize.fan-out.squash: false`).
#[test]
fn the_landed_boundary_names_what_goes_with_a_registration_it_drops_chain() {
    landed_boundary_cells(false);
}

// ---------------------------------------------------------------------------
// The other party's next step: every printed command, run as printed.
// ---------------------------------------------------------------------------

/// **The restore recipe works** — at every refusing door, over every holding a stale
/// registration can have: run as printed it brings the checkout back exactly as git recorded
/// it (the commit at `HEAD`, the staged path staged, its bytes intact), `jigc milestone
/// provision` then reuses it, and the boundary lands the staged code.
#[test]
fn the_restore_recipe_brings_a_missing_checkout_back_through_to_a_landed_boundary() {
    for door in refusing_doors() {
        for holding in HOLDINGS.into_iter().filter(|holding| holding.any()) {
            let cell = format!("{} × {holding:?} × Stale × the restore recipe", door.verb);
            let fx = Fixture::mint("restore");
            fx.isolate_the_worktree_subject(door);
            fx.stage_code(SIBLING);
            let planted = fx.plant(holding, Standing::Stale);
            let (out, _, stderr) = run_door(&fx, door, false, false);
            assert!(
                !out.status.success(),
                "{cell}: fixture — the door refuses\n{stderr}"
            );

            let spans = spans_on_the_held_line(&stderr, &fx.printed());
            let recipe = spans
                .iter()
                .find(|span| span.starts_with("mkdir -p "))
                .unwrap_or_else(|| {
                    panic!(
                        "{cell}: a refusal over a registration with no directory must print \
                         the recipe that brings the checkout back; spans: {spans:?}\n{stderr}"
                    )
                });
            run_as_printed(&fx, recipe, &cell);

            let worktree = fx.worktree(SUB);
            assert_eq!(
                git_ok(&worktree, &["rev-parse", "HEAD"]),
                planted.head,
                "{cell}: the checkout is back at the HEAD git recorded",
            );
            assert_eq!(
                git_ok(&worktree, &["rev-parse", "--show-toplevel"]),
                worktree.display().to_string(),
                "{cell}: …as a worktree of its own",
            );
            if holding.commit() {
                assert_eq!(
                    fs::read_to_string(worktree.join(COMMITTED.0))
                        .ok()
                        .as_deref(),
                    Some(COMMITTED.1),
                    "{cell}: the committed file is back on disk",
                );
            }
            if holding.staged() {
                assert!(
                    git_ok(&worktree, &["status", "--porcelain"])
                        .contains(&format!("A  {}", STAGED.0)),
                    "{cell}: the staged path is back, still staged",
                );
                assert_eq!(
                    fs::read_to_string(worktree.join(STAGED.0)).ok().as_deref(),
                    Some(STAGED.1),
                    "{cell}: …with the bytes the sub-agent staged",
                );
            }

            // The fan-out goes on from there: `provision` reuses the restored worktree…
            fx.jigc_ok(&["milestone", "provision", MILESTONE]);
            assert!(
                fx.still_held(&planted),
                "{cell}: provision reuses the restored worktree untouched",
            );
            // …and the boundary lands what it staged.
            if holding.staged() {
                fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
                assert_eq!(
                    git_ok(&fx.repo, &["show", &format!("HEAD:{}", STAGED.0)]),
                    STAGED.1.trim_end(),
                    "{cell}: the sub-agent's staged code is recovered AND landed",
                );
            }
        }
    }
}

/// **The keep-the-commit command works** — wherever a refusal names a commit no ref reaches,
/// the command beside it, run as printed, gives that commit a ref; the path then clears and
/// the door proceeds with the commit still reachable.
#[test]
fn the_keep_command_gives_the_commit_a_ref_and_the_door_then_proceeds() {
    let mut driven = 0;
    for door in refusing_doors() {
        for standing in STANDINGS {
            if !reaches(door, standing) {
                continue;
            }
            let cell = format!("{} × Commit × {standing:?} × the keep command", door.verb);
            let fx = Fixture::mint("keep");
            fx.isolate_the_worktree_subject(door);
            let planted = fx.plant(Holding::Commit, standing);
            let (out, _, stderr) = run_door(&fx, door, false, false);
            assert!(
                !out.status.success(),
                "{cell}: fixture — the door refuses\n{stderr}"
            );

            let spans = spans_on_the_held_line(&stderr, &fx.printed());
            let keep = spans
                .iter()
                .find(|span| span.contains(" branch ") && span.contains(&planted.head))
                .unwrap_or_else(|| {
                    panic!(
                        "{cell}: a refusal over a commit no ref reaches must print the \
                         command that keeps it; spans: {spans:?}\n{stderr}"
                    )
                });
            run_as_printed(&fx, keep, &cell);
            assert_ne!(
                git_ok(&fx.repo, &["for-each-ref", "--contains", &planted.head]),
                "",
                "{cell}: the commit now has a ref",
            );

            // The path holds nothing no ref reaches any more, so the same door proceeds
            // without consent — and the commit outlives the registration.
            let (out, _, stderr) = run_door(&fx, door, false, false);
            assert!(
                out.status.success(),
                "{cell}: with the commit kept, the door must proceed un-forced; got {:?}\n\
                 stderr:\n{stderr}",
                out.status,
            );
            assert!(
                !fx.still_held(&planted),
                "{cell}: …dropping the registration it no longer needs to answer for",
            );
            assert!(
                git(
                    &fx.repo,
                    &["cat-file", "-e", &format!("{}^{{commit}}", planted.head)]
                )
                .status
                .success()
                    && !git_ok(&fx.repo, &["for-each-ref", "--contains", &planted.head]).is_empty(),
                "{cell}: …and the sub-agent's commit is still reachable",
            );
            assert_ne!(
                planted.head, planted.base,
                "{cell}: fixture — a real commit"
            );
            driven += 1;
        }
    }
    assert!(driven > 0, "no cell drove the keep command");
}

/// **The landed boundary's own keep command works after the fact**: the registration is gone
/// and the commit is unreachable, but it is still in the object database — the command the
/// narration prints gives it a ref.
#[test]
fn the_landed_boundary_prints_a_keep_command_that_still_works_after_the_drop() {
    for standing in STANDINGS {
        let cell = format!(
            "{} × Commit × {standing:?} × the keep command",
            FINALIZE_DOOR.verb
        );
        let fx = Fixture::mint("landed-keep");
        fx.stage_code(SIBLING);
        let planted = fx.plant(Holding::Commit, standing);
        let (out, _, stderr) = run_door(&fx, &FINALIZE_DOOR, false, false);
        assert!(out.status.success(), "{cell}: the boundary lands\n{stderr}");
        assert_eq!(
            git_ok(&fx.repo, &["for-each-ref", "--contains", &planted.head]),
            "",
            "{cell}: fixture — nothing reaches the commit once the registration is gone",
        );
        let keep = stderr
            .split('`')
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, span)| span)
            .find(|span| span.contains(" branch ") && span.contains(&planted.head))
            .unwrap_or_else(|| {
                panic!("{cell}: the narration must print the command that keeps it\n{stderr}")
            });
        run_as_printed(&fx, keep, &cell);
        assert_ne!(
            git_ok(&fx.repo, &["for-each-ref", "--contains", &planted.head]),
            "",
            "{cell}: the commit the boundary dropped is kept",
        );
    }
}

// ---------------------------------------------------------------------------
// The edge of the leg: what it must NOT hold.
// ---------------------------------------------------------------------------

/// **The leg holds a sub-agent's unreached work and nothing else** — the controls on the
/// other side of the refusal, each a state a door proceeded over before this fix and must
/// still proceed over, un-forced. A guard that refused here would be a wall, and would train
/// `--force` into reflex on states that lose nothing.
#[test]
fn the_registration_leg_does_not_hold_what_a_ref_reaches_or_what_jigc_put_there() {
    let discard = refusing_doors()
        .into_iter()
        .find(|door| door.verb == cli::milestone::DISCARD_DOOR.verb)
        .expect("`jigc milestone discard` is a refusing worktree door");
    let uninstall = refusing_doors()
        .into_iter()
        .find(|door| door.verb == UNINSTALL_DOOR.verb)
        .expect("`jigc uninstall` is a refusing worktree door");
    let proceeds = |fx: &Fixture, door: &DestroyingDoor, cell: &str| {
        let (out, _, stderr) = run_door(fx, door, false, false);
        assert!(
            out.status.success(),
            "{cell}: nothing here is a sub-agent's unreached work, so the door must proceed \
             un-forced; got {:?}\nstderr:\n{stderr}",
            out.status,
        );
        stderr
    };

    // (1) The sub-agent committed ON A BRANCH it made inside the worktree: a ref reaches
    // the commit, and dropping the registration loses nothing.
    {
        let fx = Fixture::mint("ctl-branch");
        let worktree = fx.worktree(SUB);
        git_ok(&worktree, &["switch", "-q", "-c", "sub-agent-work"]);
        fs::write(worktree.join(COMMITTED.0), COMMITTED.1).expect("write");
        git_ok(&worktree, &["add", COMMITTED.0]);
        git_ok(&worktree, &["commit", "-q", "-m", "on a branch"]);
        let head = git_ok(&worktree, &["rev-parse", "HEAD"]);
        proceeds(&fx, discard, "a commit on a branch × discard");
        assert_eq!(
            git_ok(&fx.repo, &["rev-parse", "sub-agent-work"]),
            head,
            "the branch outlives the registration, and the commit with it",
        );
    }

    // (2) The base pin itself is orphaned — history rewritten after the fan-out was
    // provisioned — and the worktree still sits exactly where jigc detached it. No ref
    // reaches that commit, and it is not a sub-agent's work. Live, and with the directory
    // gone.
    for standing in STANDINGS {
        let fx = Fixture::mint("ctl-pin");
        let base = git_ok(&fx.worktree(SUB), &["rev-parse", "HEAD"]);
        let rewritten = git_ok(&fx.repo, &["commit-tree", "HEAD^{tree}", "-m", "rewritten"]);
        git_ok(&fx.repo, &["update-ref", "refs/heads/main", &rewritten]);
        git_ok(&fx.repo, &["reflog", "expire", "--expire=now", "--all"]);
        assert_eq!(
            git_ok(&fx.repo, &["for-each-ref", "--contains", &base]),
            "",
            "fixture: no ref reaches the base pin any more",
        );
        if standing == Standing::Stale {
            fs::remove_dir_all(fx.worktree(SUB)).expect("delete the worktree directory");
        }
        proceeds(
            &fx,
            discard,
            &format!("a worktree at an orphaned base pin × {standing:?} × discard"),
        );
    }

    // (3) A commit the sub-agent made and then moved OFF by its own `git reset`: only the
    // registration's reflog reaches it. The declared bound — the subject is HEAD and the
    // index.
    {
        let fx = Fixture::mint("ctl-reflog");
        let worktree = fx.worktree(SUB);
        fs::write(worktree.join(COMMITTED.0), COMMITTED.1).expect("write");
        git_ok(&worktree, &["add", COMMITTED.0]);
        git_ok(&worktree, &["commit", "-q", "-m", "made, then abandoned"]);
        git_ok(&worktree, &["reset", "-q", "--hard", "HEAD~1"]);
        proceeds(&fx, discard, "a commit only the reflog reaches × discard");
    }

    // (4) A boundary's own dedicated worktree, left behind by a crashed run with the
    // aggregate commit it had built: jigc's synthesis of the sub-task worktrees, not a
    // sub-agent's work.
    {
        let fx = Fixture::mint("ctl-combine");
        fx.isolate_the_worktree_subject(uninstall);
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
        fs::write(combine.join("aggregate.rs"), "fn aggregate() {}\n").expect("write");
        git_ok(&combine, &["add", "aggregate.rs"]);
        git_ok(
            &combine,
            &[
                "commit",
                "-q",
                "-m",
                "the aggregate a crashed boundary built",
            ],
        );
        proceeds(
            &fx,
            uninstall,
            "a crashed boundary's dedicated worktree × uninstall",
        );
    }

    // (5) `.jigc/` is already gone: `uninstall` drops registrations as part of removing
    // that tree and touches none without it — so there is nothing for it to answer for,
    // and the registration it leaves is exactly as it found it.
    {
        let fx = Fixture::mint("ctl-no-jigc");
        let planted = fx.plant(Holding::Both, Standing::Stale);
        fs::remove_dir_all(fx.repo.join(".jigc")).expect("remove `.jigc/` by hand");
        proceeds(
            &fx,
            uninstall,
            "a holding registration with no `.jigc/` × uninstall",
        );
        assert_eq!(
            bytes_under(&fx.admin(SUB)),
            planted.admin,
            "a run that drops no registration leaves it byte-identical",
        );
    }
}
