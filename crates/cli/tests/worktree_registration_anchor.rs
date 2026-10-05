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
//!   * **the door axis** — `cli::milestone::WORKTREE_DOORS`, read code-side. All four refuse.
//!     The three that stand where nothing lands take `--force` as their consent
//!     (`DestroyingDoor::consent`) and are driven without it and with it; the milestone
//!     boundary has no consent to offer, so it is driven once per commit arm and format;
//!   * **the holding axis** — [`HOLDINGS`]: a commit only the worktree's `HEAD` reaches, a
//!     path staged in its index, both, and the **empty** control (HEAD at the base pin,
//!     nothing staged), which must keep clearing at exit 0 — the idempotent re-provision;
//!   * **the standing axis** — [`STANDINGS`]: the checkout **live**, or its directory
//!     **gone** while the registration stands (git's `prunable`).
//!
//! One rule decides every cell at the three consenting doors ([`reaches`]): *does this door
//! drop this registration?* `provision` reuses a live registered worktree untouched, so it
//! drops nothing there and must neither refuse nor narrate; every other (door, standing)
//! pair drops it. A consenting door **refuses** exactly where it reaches work; under consent
//! it **names what it took, and nothing it did not** — asserted against what git still holds
//! afterwards, never against jigc's words alone.
//!
//! # The milestone boundary refuses before it lands
//!
//! `jigc milestone finalize` used to be this suite's narrating member: it landed, dropped
//! the registration with its teardown, and named what went. That left the sub-agent's
//! deliverable out of a milestone whose record had already flipped to the terminal
//! `joined`, and destroyed it, at exit 0. Its rule is its own ([`boundary_refuses`]): *would
//! the boundary land without work this registration holds?* — a commit, stale or live; a
//! staged path wherever the boundary does not carry it (no live checkout, or a sub-task
//! settled by `jigc task discard`). A live sub-task's staged path is the boundary's ordinary
//! input and lands. A refusal commits nothing, leaves the record `active` and the
//! registration byte-identical, and offers no `--force` — the door has none.
//!
//! Every command a refusal prints is run **as printed**, through a real shell, from outside
//! the repository, and followed to where the sub-agent's work is: restored, re-linked, kept
//! under a ref or a stash, or landed by the boundary.

use cli::milestone::{
    DISCARD_DOOR, DestroyingDoor, FINALIZE_DOOR, PROVISION_DOOR, UNINSTALL_DOOR, WORKTREE_DOORS,
};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The milestone every fixture mints.
const MILESTONE: &str = "cache-rework";
/// Its committed record, in the `[dev ▸ methodology]` fixture ([`Fixture::mint_recorded`]).
const RECORD: &str = "docs/milestone-records/cache-rework.md";
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
    /// The dev-only fixture: no methodology pack, so no committed milestone record.
    fn mint(tag: &str) -> Self {
        Self::mint_with(tag, false)
    }

    /// The `[dev ▸ methodology]` fixture: the milestone has a **committed record**
    /// ([`RECORD`]), which is what a boundary flips to the terminal `joined` and what `jigc
    /// task discard <sub-task>` settles one item of. The cells that assert *the record stays
    /// `active`* or drive a settled sub-task need it; dev-only resolves no such doctype.
    fn mint_recorded(tag: &str) -> Self {
        Self::mint_with(tag, true)
    }

    fn mint_with(tag: &str, recorded: bool) -> Self {
        let root = TempDir::new(tag);
        let repo = root.path().join("repo");
        fs::create_dir_all(&repo).expect("mk repo");
        git_ok(&repo, &["init", "-q", "-b", "main"]);
        git_ok(&repo, &["config", "user.email", "test@example.com"]);
        git_ok(&repo, &["config", "user.name", "Test"]);
        git_ok(&repo, &["config", "commit.gpgsign", "false"]);
        fs::write(repo.join("README.md"), "hello\n").expect("write README");
        if recorded {
            // The compose marker, committed with the base — the project layer a real
            // `jigc setup` leaves behind.
            fs::create_dir_all(repo.join(".jigc").join("config")).expect("mk project config");
            fs::write(
                repo.join(".jigc").join("config").join("packs.yaml"),
                "compose-embedded-methodology: true\n",
            )
            .expect("write compose marker");
        }
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
        self.run_in(&self.repo, args)
    }

    /// Run jigc from `cwd` — the repository itself, or a `cp -R` copy of it.
    fn run_in(&self, cwd: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_jigc"))
            .args(args)
            .current_dir(cwd)
            .env("HOME", self.home.path())
            .env_remove("JIGC_PACK_DIR")
            .output()
            .expect("run the jigc binary")
    }

    /// The committed record's `status:` leaf — `None` in the dev-only fixture, which has
    /// no record.
    fn record_status(&self) -> Option<String> {
        let source = fs::read_to_string(self.repo.join(RECORD)).ok()?;
        source
            .lines()
            .find_map(|line| line.strip_prefix("status: "))
            .map(str::to_owned)
    }

    /// Everything the main checkout would show a reader who asked what a door changed:
    /// its `HEAD`, and `git status` over the whole tree.
    fn main_checkout(&self) -> (String, String) {
        (
            git_ok(&self.repo, &["rev-parse", "HEAD"]),
            git_ok(&self.repo, &["status", "--porcelain"]),
        )
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
        self.author_commit_doc(sub);
    }

    /// The authored `commit:<sub>` doc alone — what the chain arm needs for a sub-task whose
    /// code this suite staged some other way.
    fn author_commit_doc(&self, sub: &str) {
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
        .find(|code| {
            code.ends_with(".leftover-holds-work")
                || code.ends_with(".dirty-worktree")
                || code.ends_with(".unlanded-work")
        })
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

/// **The milestone boundary's rule**: would `jigc milestone finalize` land *without* work
/// the registration holds? A commit is never what the boundary lands, stale or live; a
/// staged path is, exactly when a live checkout of a sub-task the boundary lands holds it.
fn boundary_refuses(holding: Holding, standing: Standing) -> bool {
    holding.commit() || (holding.staged() && standing == Standing::Stale)
}

/// Whether `door` is the milestone boundary — the one member with no consent to offer.
fn is_boundary(door: &DestroyingDoor) -> bool {
    door.verb == FINALIZE_DOOR.verb
}

/// Set a fixture up for `door`: isolate the worktree subject, and — for the boundary, which
/// refuses a milestone that would land no work at all — give the sibling staged code, so
/// that once [`SUB`]'s path clears the boundary has something to land.
fn prepare(fx: &Fixture, door: &DestroyingDoor) {
    fx.isolate_the_worktree_subject(door);
    if is_boundary(door) {
        fx.stage_code(SIBLING);
    }
}

/// Drive `door` un-forced and require a refusal; return its stderr.
fn refusal_of(fx: &Fixture, door: &DestroyingDoor, cell: &str) -> String {
    let (out, _, stderr) = run_door(fx, door, false, false);
    assert!(
        !out.status.success(),
        "{cell}: fixture — the door refuses; got {:?}\nstderr:\n{stderr}",
        out.status,
    );
    let code = worktree_code(door);
    assert!(
        stderr.contains(&format!("blocking · {code}")),
        "{cell}: the refusal carries the door's own code `{code}`; stderr:\n{stderr}",
    );
    assert_the_route_promises_only_what_the_line_prints(fx, &stderr, cell);
    stderr
}

/// Whether `span` is something a reader runs — one of the commands a refusal line prints
/// (the keep, `reset --soft` and stash commands, the restore recipe, the re-link) — rather
/// than a backticked name such as `` `.git` ``.
fn is_runnable(span: &str) -> bool {
    ["git ", "mkdir ", "printf "]
        .iter()
        .any(|head| span.starts_with(head))
}

/// **A route that says *run the command on that line* is a promise about that line** (the
/// rc.24 fix pass's completion audit, F5). Asked of every refusal this suite drives, so
/// each cell of the door × holding × standing grid holds it rather than the one cell the
/// finding was reported at.
fn assert_the_route_promises_only_what_the_line_prints(fx: &Fixture, stderr: &str, cell: &str) {
    if !stderr.contains("run the command on that line") {
        return;
    }
    let printed = fx.printed();
    let runnable = stderr
        .lines()
        .filter(|line| line.trim_start().starts_with(&format!("{printed}:")))
        .flat_map(|line| line.split('`').skip(1).step_by(2))
        .any(is_runnable);
    assert!(
        runnable,
        "{cell}: the route tells the reader to run the command on `{printed}`'s line, and \
         that line prints none; stderr:\n{stderr}",
    );
}

/// The one span on [`SUB`]'s refusal line that `is` picks — or a panic naming what was
/// printed instead.
fn span_where(
    stderr: &str,
    printed: &str,
    what: &str,
    cell: &str,
    is: impl Fn(&str) -> bool,
) -> String {
    let spans = spans_on_the_held_line(stderr, printed);
    spans
        .iter()
        .find(|span| is(span))
        .cloned()
        .unwrap_or_else(|| {
            panic!("{cell}: the refusal must print {what}; spans: {spans:?}\n{stderr}")
        })
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
// The milestone boundary: it refuses before it lands.
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

/// **`jigc milestone finalize` × holding × standing × format**, on one commit arm.
///
/// Where the boundary would land without work the registration holds
/// ([`boundary_refuses`]) it refuses **before it lands**: exit 3 under its own code, keyed
/// at the path, naming what is held, with no `--force` anywhere — and nothing has moved:
/// `HEAD`, the index, the committed record (`active`), git's registration of the path (byte
/// for byte) and the sibling's worktree. Everywhere else it lands exactly as it did, and the
/// pinned landed envelope keeps its keys.
///
/// One arm per test, so the two halves of the commit-arm axis run side by side.
fn boundary_cells(squash: bool) {
    assert!(
        FINALIZE_DOOR.consent().is_none(),
        "the boundary offers no consent — its refusal has no `--force` to name",
    );
    let code = worktree_code(&FINALIZE_DOOR);
    let mut refused = 0;
    let mut landed_cells = 0;
    for json in [false, true] {
        for holding in HOLDINGS {
            for standing in STANDINGS {
                let cell = format!(
                    "{} × {holding:?} × {standing:?} × squash: {squash} × {}",
                    FINALIZE_DOOR.verb,
                    if json { "json" } else { "text" },
                );
                let fx = Fixture::mint_recorded("boundary");
                if !squash {
                    fx.set_squash_false();
                }
                fx.stage_code(SIBLING);
                let planted = fx.plant(holding, standing);
                if standing == Standing::Live && holding.staged() {
                    // A live sub-task's staged code lands — give the chain arm the
                    // commit doc it renders that sub-task's commit from.
                    fx.author_commit_doc(SUB);
                }
                let before = fx.main_checkout();
                assert_eq!(
                    fx.record_status().as_deref(),
                    Some("active"),
                    "{cell}: fixture — the milestone is in flight",
                );

                let (out, stdout, stderr) = run_door(&fx, &FINALIZE_DOOR, false, json);

                if boundary_refuses(holding, standing) {
                    refused += 1;
                    assert_eq!(
                        out.status.code(),
                        Some(3),
                        "{cell}: the boundary would land without work this registration \
                         holds, so it must refuse before it lands (exit 3); got {:?}\n\
                         stdout:\n{stdout}\nstderr:\n{stderr}",
                        out.status,
                    );
                    // What the refusal said, on the channel the format puts it on.
                    let said = if json {
                        let doc: serde_json::Value =
                            serde_json::from_str(&stdout).unwrap_or_else(|err| {
                                panic!("{cell}: stdout is the findings envelope: {err}\n{stdout}")
                            });
                        let finding = doc["findings"]
                            .as_array()
                            .and_then(|findings| {
                                findings
                                    .iter()
                                    .find(|finding| finding["key"]["code"] == code)
                            })
                            .unwrap_or_else(|| {
                                panic!("{cell}: a `{code}` finding is on the envelope\n{stdout}")
                            });
                        assert_eq!(
                            finding["key"]["target"],
                            serde_json::json!(fx.printed()),
                            "{cell}: the finding is keyed at the held path",
                        );
                        assert_eq!(
                            finding["severity"],
                            serde_json::json!("blocking"),
                            "{cell}: …and it blocks",
                        );
                        format!(
                            "{}\n{}",
                            finding["message"].as_str().unwrap_or_default(),
                            finding["route"].as_str().unwrap_or_default(),
                        )
                    } else {
                        assert!(
                            stderr.contains(&format!("blocking · {code}")),
                            "{cell}: the refusal carries `{code}`; stderr:\n{stderr}",
                        );
                        stderr.clone()
                    };
                    assert!(
                        said.contains(&fx.printed()),
                        "{cell}: the refusal names WHICH path; got:\n{said}",
                    );
                    // Named ⇔ held. A live sub-task's staged path is the boundary's input:
                    // the re-run lands it, so beside a held commit it is not a hold and a
                    // refusal that listed it would be claiming a loss that is not one.
                    if holding.commit() {
                        assert!(
                            said.contains(short(&planted.head)),
                            "{cell}: the refusal names the commit only this registration's \
                             HEAD reaches; got:\n{said}",
                        );
                    }
                    if holding.staged() {
                        assert_eq!(
                            said.contains(STAGED.0),
                            standing == Standing::Stale,
                            "{cell}: a staged path is named iff the boundary would not \
                             carry it — its checkout is gone; got:\n{said}",
                        );
                    }
                    assert!(
                        !said.contains("--force"),
                        "{cell}: this door has no consent flag, so its refusal names none; \
                         got:\n{said}",
                    );
                    assert!(
                        said.contains(&format!("jigc milestone discard {MILESTONE}")),
                        "{cell}: the route names the abandon as the other honest exit; \
                         got:\n{said}",
                    );

                    // Nothing landed, nothing was flipped, nothing was dropped.
                    assert_eq!(
                        fx.main_checkout(),
                        before,
                        "{cell}: a refusal commits nothing and stages nothing — HEAD and \
                         `git status` are as they were",
                    );
                    assert_eq!(
                        fx.record_status().as_deref(),
                        Some("active"),
                        "{cell}: the committed record stays `active` — the milestone is \
                         still finalizable",
                    );
                    assert_eq!(
                        bytes_under(&fx.admin(SUB)),
                        planted.admin,
                        "{cell}: a refusal leaves git's registration byte-identical",
                    );
                    if standing == Standing::Live {
                        assert!(
                            fx.worktree(SUB).join(".git").exists(),
                            "{cell}: a refusal leaves the live checkout standing",
                        );
                    }
                    assert!(
                        fx.worktree(SIBLING)
                            .join(format!("{SIBLING}.txt"))
                            .is_file()
                            && fx.admin(SIBLING).is_dir(),
                        "{cell}: …and the sibling's worktree, with the code a re-run lands",
                    );
                    continue;
                }

                landed_cells += 1;
                assert!(
                    out.status.success(),
                    "{cell}: nothing this registration holds would be left behind, so the \
                     boundary lands; got {:?}\nstderr:\n{stderr}",
                    out.status,
                );
                assert_eq!(
                    fx.record_status().as_deref(),
                    Some("joined"),
                    "{cell}: a landed boundary settles the record",
                );
                assert!(
                    !fx.admin(SUB).exists(),
                    "{cell}: the landed teardown drops the sub-task's registration",
                );
                let tree = git_ok(&fx.repo, &["ls-tree", "-r", "--name-only", "HEAD"]);
                let landed = |file: &str| tree.lines().any(|path| path == file);
                assert!(
                    landed(&format!("{SIBLING}.txt")),
                    "{cell}: the sibling lands"
                );
                assert_eq!(
                    landed(STAGED.0),
                    holding.staged(),
                    "{cell}: a live sub-task's staged path is the boundary's input, and lands",
                );
                // A boundary that landed dropped nothing that held work — so it has no
                // loss to name.
                assert!(
                    !stderr.contains(&fx.printed()),
                    "{cell}: a landed boundary names no loss at this path; stderr:\n{stderr}",
                );

                if json {
                    // The pinned envelope does not move: same keys, and `provisioned`
                    // keeps meaning *a live worktree stood there*.
                    let doc: serde_json::Value = serde_json::from_str(&stdout)
                        .unwrap_or_else(|err| panic!("{cell}: stdout is JSON: {err}\n{stdout}"));
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
    // Neither half is empty: a guard that never fires proves nothing, and one that always
    // fires is a wall across the ordinary fan-out.
    assert!(
        refused > 0 && landed_cells > 0,
        "{refused} refused, {landed_cells} landed"
    );
}

/// The single aggregate commit (`finalize.fan-out.squash: true`, the default).
#[test]
fn the_boundary_refuses_before_landing_without_work_a_registration_holds_squash() {
    boundary_cells(true);
}

/// One commit per sub-task, then the aggregate (`finalize.fan-out.squash: false`).
#[test]
fn the_boundary_refuses_before_landing_without_work_a_registration_holds_chain() {
    boundary_cells(false);
}

// ---------------------------------------------------------------------------
// The other party's next step: every printed command, run as printed.
// ---------------------------------------------------------------------------

/// **The restore recipe works** — at every worktree door, over every holding a stale
/// registration can have: run as printed it brings the checkout back exactly as git recorded
/// it (the commit at `HEAD`, the staged path staged, its bytes intact), `jigc milestone
/// provision` then reuses it, and the boundary lands the sub-agent's work — the staged path
/// directly, and a commit through the fold the boundary's own refusal prints for the
/// checkout that is now live.
#[test]
fn the_restore_recipe_brings_a_missing_checkout_back_through_to_a_landed_boundary() {
    for door in WORKTREE_DOORS {
        for holding in HOLDINGS.into_iter().filter(|holding| holding.any()) {
            let cell = format!("{} × {holding:?} × Stale × the restore recipe", door.verb);
            let fx = Fixture::mint("restore");
            fx.isolate_the_worktree_subject(door);
            fx.stage_code(SIBLING);
            let planted = fx.plant(holding, Standing::Stale);
            let stderr = refusal_of(&fx, door, &cell);

            let recipe = span_where(
                &stderr,
                &fx.printed(),
                "the recipe that brings the checkout back",
                &cell,
                |span| span.starts_with("mkdir -p "),
            );
            run_as_printed(&fx, &recipe, &cell);

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
            // …and the boundary lands the work. A commit is not what it lands, so over the
            // restored checkout it refuses once more — live now — and prints the fold.
            if holding.commit() {
                let stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
                let fold = span_where(
                    &stderr,
                    &fx.printed(),
                    "the command that folds the commit into the staged set",
                    &cell,
                    |span| span.contains(" reset --soft "),
                );
                run_as_printed(&fx, &fold, &cell);
            }
            fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
            if holding.commit() {
                assert_eq!(
                    git_ok(&fx.repo, &["show", &format!("HEAD:{}", COMMITTED.0)]),
                    COMMITTED.1.trim_end(),
                    "{cell}: the sub-agent's committed code is recovered AND landed",
                );
            }
            if holding.staged() {
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
/// the door proceeds with the commit still reachable. At the boundary *proceeds* is *lands
/// without it*: the commit is kept aside, and the milestone's tree does not carry its file.
#[test]
fn the_keep_command_gives_the_commit_a_ref_and_the_door_then_proceeds() {
    let mut driven = 0;
    for door in WORKTREE_DOORS {
        for standing in STANDINGS {
            if !reaches(door, standing) {
                continue;
            }
            let cell = format!("{} × Commit × {standing:?} × the keep command", door.verb);
            let fx = Fixture::mint("keep");
            prepare(&fx, door);
            let planted = fx.plant(Holding::Commit, standing);
            let stderr = refusal_of(&fx, door, &cell);

            let keep = span_where(
                &stderr,
                &fx.printed(),
                "the command that keeps the commit",
                &cell,
                |span| span.contains(" branch ") && span.contains(&planted.head),
            );
            run_as_printed(&fx, &keep, &cell);
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
            if is_boundary(door) {
                let tree = git_ok(&fx.repo, &["ls-tree", "-r", "--name-only", "HEAD"]);
                assert!(
                    tree.lines().any(|path| path == format!("{SIBLING}.txt"))
                        && !tree.lines().any(|path| path == COMMITTED.0),
                    "{cell}: the boundary landed the sibling and — the reader's stated \
                     choice — not the kept commit; tree:\n{tree}",
                );
            }
            driven += 1;
        }
    }
    assert!(driven > 0, "no cell drove the keep command");
}

/// **The boundary's own exit lands a commit the sub-agent made instead of staging**: the
/// `git reset --soft <base pin>` its refusal prints, run as printed, moves the worktree's
/// `HEAD` back to where jigc detached it and leaves everything the commit changed *staged* —
/// the one thing the boundary reads — so the same `finalize` then lands it, beside whatever
/// was already staged, on both commit arms.
#[test]
fn the_boundary_prints_the_fold_that_lands_a_commit_made_inside_a_live_worktree() {
    for squash in [true, false] {
        for holding in [Holding::Commit, Holding::Both] {
            let cell = format!(
                "{} × {holding:?} × Live × squash: {squash} × the fold",
                FINALIZE_DOOR.verb
            );
            let fx = Fixture::mint_recorded("fold");
            if !squash {
                fx.set_squash_false();
            }
            fx.stage_code(SIBLING);
            fx.author_commit_doc(SUB);
            let planted = fx.plant(holding, Standing::Live);
            let stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);

            let fold = span_where(
                &stderr,
                &fx.printed(),
                "the command that folds the commit into the staged set",
                &cell,
                |span| span.contains(" reset --soft "),
            );
            assert!(
                fold.ends_with(&planted.base),
                "{cell}: the fold goes back to the milestone's base pin; got `{fold}`",
            );
            run_as_printed(&fx, &fold, &cell);
            assert_eq!(
                git_ok(&fx.worktree(SUB), &["rev-parse", "HEAD"]),
                planted.base,
                "{cell}: the worktree is back where jigc detached it",
            );

            fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
            assert_eq!(
                fx.record_status().as_deref(),
                Some("joined"),
                "{cell}: the boundary landed",
            );
            assert_eq!(
                git_ok(&fx.repo, &["show", &format!("HEAD:{}", COMMITTED.0)]),
                COMMITTED.1.trim_end(),
                "{cell}: what the sub-agent committed is in the milestone",
            );
            if holding.staged() {
                assert_eq!(
                    git_ok(&fx.repo, &["show", &format!("HEAD:{}", STAGED.0)]),
                    STAGED.1.trim_end(),
                    "{cell}: …beside what it staged",
                );
            }
        }
    }
}

/// How a branch already in the repository gets in the keep command's way.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Occupied {
    /// `kept/<sub-task-id>` exists — an earlier keep from the same sub-task.
    TheName,
    /// A branch named exactly `kept` exists, which makes every `kept/…` name uncreatable.
    TheNamespace,
    /// `KEPT/<sub-task-id>` exists: the same file as the name on a case-insensitive
    /// filesystem, where git stores a loose ref.
    TheNameInAnotherCase,
    /// A branch named `Kept` exists: on such a filesystem it is the file `kept`, and blocks
    /// the namespace exactly as the lower-case one does.
    TheNamespaceInAnotherCase,
    /// `KEPT/<sub-task-id>/older` exists, which makes `kept/<sub-task-id>` a directory
    /// there.
    BelowTheNameInAnotherCase,
}

impl Occupied {
    /// The branch that is in the way.
    fn branch(self) -> String {
        match self {
            Occupied::TheName => format!("kept/{SUB}"),
            Occupied::TheNamespace => "kept".to_owned(),
            Occupied::TheNameInAnotherCase => format!("KEPT/{SUB}"),
            Occupied::TheNamespaceInAnotherCase => "Kept".to_owned(),
            Occupied::BelowTheNameInAnotherCase => format!("KEPT/{SUB}/older"),
        }
    }

    /// Whether the member differs from the name only in letter case — the cells whose
    /// harm needs a case-insensitive filesystem to show. The door axis is already walked by
    /// the exact-case members over the same one predicate, so these drive the two doors
    /// that print the command on different lines (a consenting door and the boundary).
    fn is_a_case_variant(self) -> bool {
        !matches!(self, Occupied::TheName | Occupied::TheNamespace)
    }
}

const OCCUPIED: [Occupied; 5] = [
    Occupied::TheName,
    Occupied::TheNamespace,
    Occupied::TheNameInAnotherCase,
    Occupied::TheNamespaceInAnotherCase,
    Occupied::BelowTheNameInAnotherCase,
];

/// **The keep command never dead-ends on a branch that is already there.** Driven on the
/// tree before this fix, the printed `git branch kept/<sub-task-id> <sha>` exited 128 in
/// both states — *a branch named … already exists*, and *cannot lock ref … 'refs/heads/kept'
/// exists* — leaving the reader at a refusal whose own route had just failed. The name is
/// chosen against the refs that exist, at every door that prints it and in the narration
/// that prints it after a drop.
///
/// **And against the refs that exist in another letter case** (the pass's completion audit,
/// F6): on a case-insensitive filesystem `Kept` and `KEPT/<sub-task-id>` are the same files
/// as their lower-case spellings, and the printed command exited 128 over each. The name is
/// compared case-folded on every filesystem, so the cell asserts the same thing wherever
/// the suite runs — the command runs as printed — and on a case-sensitive one it is the
/// control that folding costs nothing.
#[test]
fn the_keep_command_picks_a_branch_name_git_can_create() {
    let occupy = |fx: &Fixture, occupied: Occupied| -> String {
        let name = occupied.branch();
        git_ok(&fx.repo, &["branch", &name, "main"]);
        name
    };
    for occupied in OCCUPIED {
        for door in WORKTREE_DOORS {
            for standing in STANDINGS {
                if !reaches(door, standing) {
                    continue;
                }
                if occupied.is_a_case_variant()
                    && !(standing == Standing::Live
                        && (door.verb == DISCARD_DOOR.verb || is_boundary(door)))
                {
                    continue;
                }
                let cell = format!(
                    "{} × Commit × {standing:?} × {occupied:?} is taken",
                    door.verb
                );
                let fx = Fixture::mint("keep-name");
                prepare(&fx, door);
                let planted = fx.plant(Holding::Commit, standing);
                let taken = occupy(&fx, occupied);
                let stderr = refusal_of(&fx, door, &cell);
                let keep = span_where(
                    &stderr,
                    &fx.printed(),
                    "the command that keeps the commit",
                    &cell,
                    |span| span.contains(" branch ") && span.contains(&planted.head),
                );
                assert!(
                    !keep.contains(&format!(" branch kept/{SUB} ")),
                    "{cell}: `kept/{SUB}` cannot be created beside `{taken}`, so the \
                     command must name another branch; got `{keep}`",
                );
                run_as_printed(&fx, &keep, &cell);
                assert_ne!(
                    git_ok(
                        &fx.repo,
                        &["for-each-ref", "--contains", &planted.head, "refs/heads/"]
                    ),
                    "",
                    "{cell}: the commit now has a branch",
                );
                assert_eq!(
                    git_ok(&fx.repo, &["rev-parse", &taken]),
                    git_ok(&fx.repo, &["rev-parse", "main"]),
                    "{cell}: …and the branch that was in the way is where it was",
                );
                let (out, _, stderr) = run_door(&fx, door, false, false);
                assert!(
                    out.status.success(),
                    "{cell}: with the commit kept, the door proceeds; got {:?}\n{stderr}",
                    out.status,
                );
            }
        }

        if occupied.is_a_case_variant() {
            continue;
        }
        // The narration after a consented drop prints the same command, aimed at the main
        // checkout — and it must be as runnable.
        let cell = format!(
            "{} --force × Commit × Live × {occupied:?} is taken",
            DISCARD_DOOR.verb
        );
        let fx = Fixture::mint("keep-name-narrated");
        let planted = fx.plant(Holding::Commit, Standing::Live);
        occupy(&fx, occupied);
        let (out, _, stderr) = run_door(&fx, &DISCARD_DOOR, true, false);
        assert!(out.status.success(), "{cell}: consent proceeds\n{stderr}");
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
            "{cell}: the commit the consent dropped is kept",
        );
    }
}

/// **An unlinked checkout is given back to its registration by one printed line.** The
/// worktree's directory stands with every file in it and only its `.git` link is gone, so
/// git reads the registration as `prunable` and the path as no worktree at all — while the
/// registration's index still holds the sub-agent's staged path. Every worktree door refuses
/// over it; until this fix none printed a command that mends it (the restore recipe is for a
/// path with nothing standing at it: its `git restore .` would write over files). The
/// re-link is the recipe's middle step alone, and it is driven through to a landed boundary.
#[test]
fn an_unlinked_checkout_is_re_linked_by_the_command_the_refusal_prints() {
    for door in WORKTREE_DOORS {
        let cell = format!("{} × Staged × the `.git` link gone", door.verb);
        let fx = Fixture::mint("relink");
        fx.isolate_the_worktree_subject(door);
        fx.stage_code(SIBLING);
        fx.plant(Holding::Staged, Standing::Live);
        let worktree = fx.worktree(SUB);
        fs::remove_file(worktree.join(".git")).expect("remove the `.git` link");
        assert!(
            git_ok(&fx.repo, &["worktree", "list", "--porcelain"]).contains("prunable"),
            "{cell}: fixture — git reads the registration as stale",
        );

        let stderr = refusal_of(&fx, door, &cell);
        assert!(
            stderr.contains(STAGED.0),
            "{cell}: the refusal names the path the registration's index holds; \
             stderr:\n{stderr}",
        );
        assert!(
            !spans_on_the_held_line(&stderr, &fx.printed())
                .iter()
                .any(|span| span.starts_with("mkdir -p ")),
            "{cell}: the restore recipe is for a path nothing stands at — its `git restore .` \
             would write over these files; stderr:\n{stderr}",
        );
        let relink = span_where(
            &stderr,
            &fx.printed(),
            "the command that re-links the checkout",
            &cell,
            |span| span.starts_with("printf 'gitdir: "),
        );
        assert!(
            !stderr.contains("worktree repair"),
            "{cell}: never `git worktree repair` — it re-points every other worktree this \
             repository has a registration for; stderr:\n{stderr}",
        );
        run_as_printed(&fx, &relink, &cell);

        assert_eq!(
            git_ok(&worktree, &["rev-parse", "--show-toplevel"]),
            worktree.display().to_string(),
            "{cell}: the path is a worktree of its own again",
        );
        assert!(
            git_ok(&worktree, &["status", "--porcelain"]).contains(&format!("A  {}", STAGED.0)),
            "{cell}: …with the sub-agent's path still staged",
        );
        assert_eq!(
            fs::read_to_string(worktree.join(STAGED.0)).ok().as_deref(),
            Some(STAGED.1),
            "{cell}: …and its file untouched",
        );
        assert!(
            !git_ok(&fx.repo, &["worktree", "list", "--porcelain"]).contains("prunable"),
            "{cell}: git no longer reads the registration as stale",
        );

        // The fan-out goes on: `provision` reuses it, the boundary lands what it staged.
        fx.jigc_ok(&["milestone", "provision", MILESTONE]);
        fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
        assert_eq!(
            git_ok(&fx.repo, &["show", &format!("HEAD:{}", STAGED.0)]),
            STAGED.1.trim_end(),
            "{cell}: the sub-agent's staged code is landed",
        );
    }

    // The control: a `.git` entry that IS there and leads nowhere git can read is another
    // repository's linkage, not a missing one. The door still refuses — the registration
    // still holds the staged path — and prints no line that would overwrite it. **And the
    // line says what the reader can do** (the completion audit, F5): until then the
    // boundary named what the registration held under a route that said *run the command on
    // that line*, over a line with no command on it, at the one door with no consent to
    // fall back on. The step is the reader's — move the entry aside — and it is driven
    // here through to a landed boundary.
    let entries: [(&str, &[u8]); 3] = [
        (
            "a link naming nothing",
            b"gitdir: /nonexistent/elsewhere/.git/worktrees/x\n",
        ),
        ("bytes that are no link at all", b"garbage\n"),
        ("an empty directory", b""),
    ];
    for (what, bytes) in entries {
        for door in [&DISCARD_DOOR, &FINALIZE_DOOR] {
            if bytes.is_empty() && !is_boundary(door) {
                continue;
            }
            let cell = format!("{} × Staged × a `.git` entry: {what}", door.verb);
            let fx = Fixture::mint("relink-ctl");
            prepare(&fx, door);
            fx.plant(Holding::Staged, Standing::Live);
            let worktree = fx.worktree(SUB);
            let link = worktree.join(".git");
            fs::remove_file(&link).expect("take the real link out");
            if bytes.is_empty() {
                fs::create_dir(&link).expect("leave a directory named `.git`");
            } else {
                fs::write(&link, bytes).expect("write the entry");
            }
            let stderr = refusal_of(&fx, door, &cell);
            assert!(
                !stderr.contains("printf 'gitdir: "),
                "{cell}: an entry that is there is never overwritten by a printed command; \
                 stderr:\n{stderr}",
            );
            if !bytes.is_empty() {
                assert_eq!(
                    fs::read(&link).ok().as_deref(),
                    Some(bytes),
                    "{cell}: …and the refusal left it alone",
                );
            }
            assert!(
                stderr.contains("move that entry aside"),
                "{cell}: the line names the step that is the reader's; stderr:\n{stderr}",
            );
            if !is_boundary(door) {
                continue;
            }
            assert!(
                !stderr.contains("run the command on that line"),
                "{cell}: no command is on that line, so the route must not send the reader \
                 to one; stderr:\n{stderr}",
            );

            // The step, by hand — out of the worktree, so nothing new is left inside it.
            fs::rename(&link, fx.home.path().join("the-entry-moved-aside"))
                .expect("move the entry aside");
            let stderr = refusal_of(&fx, door, &cell);
            let relink = span_where(
                &stderr,
                &fx.printed(),
                "the command that re-links the checkout",
                &cell,
                |span| span.starts_with("printf 'gitdir: "),
            );
            run_as_printed(&fx, &relink, &cell);
            fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
            assert_eq!(
                git_ok(&fx.repo, &["show", &format!("HEAD:{}", STAGED.0)]),
                STAGED.1.trim_end(),
                "{cell}: the sub-agent's staged code is landed",
            );
        }
    }
}

// ---------------------------------------------------------------------------
// A repository moved after its worktrees were provisioned.
// ---------------------------------------------------------------------------

/// How a sub-task worktree stands in a repository that was **moved** after provisioning —
/// git still lists its registration at the old path in every one of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AfterTheMove {
    /// The checkout came along, its `.git` link still naming the admin directory at the
    /// repository's old location. git reads nothing through it.
    LinkNamesTheOldPlace,
    /// The checkout came along and has no `.git` entry at all.
    NoLink,
    /// The directory is gone.
    DirectoryGone,
}

const AFTER_THE_MOVE: [AfterTheMove; 3] = [
    AfterTheMove::LinkNamesTheOldPlace,
    AfterTheMove::NoLink,
    AfterTheMove::DirectoryGone,
];

/// Move `fx`'s repository to a sibling path, and mend the **sibling** sub-task's worktree
/// the way git offers (`git worktree repair <path>`), so the boundary has something to land
/// and [`SUB`]'s is the one registration still naming the old place.
fn move_the_repository(fx: &mut Fixture) {
    let moved = fx._root.path().join("repo-moved");
    fs::rename(&fx.repo, &moved).expect("mv the repository");
    fx.repo = moved;
    git_ok(
        &fx.repo,
        &["worktree", "repair", fx.worktree(SIBLING).to_str().unwrap()],
    );
    let listed = git_ok(&fx.repo, &["worktree", "list", "--porcelain"]);
    assert!(
        !listed.contains(&format!("worktree {}", fx.worktree(SUB).display())),
        "fixture: git must not list `{SUB}` at the repository's new path; got:\n{listed}",
    );
}

/// **In a moved repository the boundary still refuses over what one of its own sub-task
/// registrations holds** (the rc.24 fix pass's completion audit, F4).
///
/// The guard matched registrations by path, and after an `mv` of the repository git's
/// records name the old one. Driven before this: one sub-task worktree mended with `git
/// worktree repair`, the other still holding a `git add`-ed file — `jigc milestone finalize`
/// exited 0, reported `unreadable worktree …, no code counted`, and the milestone was
/// settled without the file.
///
/// Every way the unmended worktree can stand is refused **before** anything lands, and
/// each is then walked out through what its refusal prints, as printed, to a boundary
/// that lands the file. The must-not-refuse cells follow: a moved registration that holds
/// nothing, and the consenting door, which reaches no registration git lists elsewhere.
#[test]
fn a_moved_repositorys_boundary_refuses_over_what_its_own_registration_holds() {
    for standing in AFTER_THE_MOVE {
        let cell = format!("moved repository × Staged × {standing:?}");
        let mut fx = Fixture::mint("moved");
        fx.stage_code(SIBLING);
        let planted = fx.plant(Holding::Staged, Standing::Live);
        move_the_repository(&mut fx);
        let worktree = fx.worktree(SUB);
        match standing {
            AfterTheMove::LinkNamesTheOldPlace => {}
            AfterTheMove::NoLink => {
                fs::remove_file(worktree.join(".git")).expect("remove the link");
            }
            AfterTheMove::DirectoryGone => {
                fs::remove_dir_all(&worktree).expect("delete the worktree directory");
            }
        }
        let head = git_ok(&fx.repo, &["rev-parse", "HEAD"]);
        let admin = bytes_under(&fx.admin(SUB));

        let stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
        assert!(
            stderr.contains(STAGED.0),
            "{cell}: the refusal names the path the registration's index holds; \
             stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("before it was moved"),
            "{cell}: …and says where git lists that registration; stderr:\n{stderr}",
        );
        assert!(
            !stderr.contains("worktree repair"),
            "{cell}: never `git worktree repair` — it re-points every other worktree this \
             repository has a registration for; stderr:\n{stderr}",
        );
        assert!(
            fx.still_held(&planted),
            "{cell}: the registration still holds the staged path after the refusal",
        );
        assert_eq!(
            git_ok(&fx.repo, &["rev-parse", "HEAD"]),
            head,
            "{cell}: nothing was committed",
        );
        assert_eq!(
            bytes_under(&fx.admin(SUB)),
            admin,
            "{cell}: the registration is byte-identical after the refusal",
        );

        // Out through what the refusal prints.
        let stderr = if standing == AfterTheMove::LinkNamesTheOldPlace {
            assert!(
                stderr.contains("move that entry aside")
                    && !stderr.contains("run the command on that line"),
                "{cell}: a link that is there is never overwritten by a printed command, \
                 so the line names the reader's step; stderr:\n{stderr}",
            );
            fs::rename(
                worktree.join(".git"),
                fx.home.path().join("the-old-link-moved-aside"),
            )
            .expect("move the old link aside");
            refusal_of(&fx, &FINALIZE_DOOR, &cell)
        } else {
            stderr
        };
        let span = span_where(
            &stderr,
            &fx.printed(),
            "the command that brings the checkout back to its registration",
            &cell,
            |span| match standing {
                AfterTheMove::DirectoryGone => span.starts_with("mkdir -p "),
                _ => span.starts_with("printf 'gitdir: "),
            },
        );
        run_as_printed(&fx, &span, &cell);
        assert!(
            git_ok(&worktree, &["status", "--porcelain"]).contains(&format!("A  {}", STAGED.0)),
            "{cell}: the path is a worktree again, the sub-agent's path still staged",
        );
        fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
        assert_eq!(
            git_ok(&fx.repo, &["show", &format!("HEAD:{}", STAGED.0)]),
            STAGED.1.trim_end(),
            "{cell}: the sub-agent's staged code is landed",
        );
    }

    // MUST NOT REFUSE (1): the moved registration holds nothing — its `HEAD` at the base
    // pin, nothing staged, its directory gone. The boundary lands the sibling's work.
    let cell = "moved repository × an empty registration";
    let mut fx = Fixture::mint("moved-empty");
    fx.stage_code(SIBLING);
    move_the_repository(&mut fx);
    fs::remove_dir_all(fx.worktree(SUB)).expect("delete the worktree directory");
    let (out, _, stderr) = run_door(&fx, &FINALIZE_DOOR, false, false);
    assert!(
        out.status.success(),
        "{cell}: nothing is held, so the boundary lands; got {:?}\n{stderr}",
        out.status,
    );

    // MUST NOT REFUSE (2): the consenting door. It drops a registration at the path git
    // lists it at and reaches none listed elsewhere, so it has nothing to refuse over —
    // exactly as on `1.0.0-rc.24` — and the record it did not reach is byte-identical.
    let cell = "moved repository × `milestone discard`, un-forced";
    let mut fx = Fixture::mint("moved-discard");
    let planted = fx.plant(Holding::Staged, Standing::Live);
    move_the_repository(&mut fx);
    fs::remove_dir_all(fx.worktree(SUB)).expect("delete the worktree directory");
    let admin = bytes_under(&fx.admin(SUB));
    let (out, _, stderr) = run_door(&fx, &DISCARD_DOOR, false, false);
    assert!(
        out.status.success(),
        "{cell}: the door drops nothing git lists elsewhere, so it does not refuse over \
         it; got {:?}\n{stderr}",
        out.status,
    );
    assert_eq!(
        bytes_under(&fx.admin(SUB)),
        admin,
        "{cell}: …and the registration it did not reach is byte-identical",
    );
    assert!(
        fx.still_held(&planted),
        "{cell}: …still holding the staged path"
    );

    // MUST NOT REFUSE (3): a copied repository whose source still stands. Its records name
    // the SOURCE's worktrees — checkouts that are there and answer for the source — so a
    // sub-task worktree deleted in the copy is not a registration of the copy's own that
    // moved, and the copy's boundary lands as it always did.
    let cell = "a `cp -R` copy whose source still stands";
    let fx = Fixture::mint("moved-copy");
    fx.plant(Holding::Staged, Standing::Live);
    let copy = fx.repo.parent().expect("the fixture root").join("copy");
    let copied = Command::new("cp")
        .arg("-R")
        .arg(&fx.repo)
        .arg(&copy)
        .status()
        .expect("run cp -R");
    assert!(copied.success(), "{cell}: fixture — the copy is made");
    let sibling = copy.join(".jigc").join("worktrees").join(SIBLING);
    fs::write(sibling.join("copied.txt"), "code\n").expect("write");
    git_ok(&sibling, &["add", "copied.txt"]);
    fs::remove_dir_all(copy.join(".jigc").join("worktrees").join(SUB))
        .expect("delete the copy's worktree directory");
    let out = fx.run_in(&copy, &["milestone", "finalize", MILESTONE]);
    assert!(
        out.status.success(),
        "{cell}: the source's registrations are not the copy's moved ones, so nothing is \
         held; got {:?}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
}

// ---------------------------------------------------------------------------
// The boundary over a sub-task settled by `jigc task discard`.
// ---------------------------------------------------------------------------

/// **A settled sub-task's worktree is still the teardown's subject**, and nothing lands from
/// it — so everything its registration holds is work the boundary would drop: a commit, and
/// a staged path **live or stale**. Driven on the tree before this fix, a `jigc task discard
/// <sub-task>` followed by `jigc milestone finalize` landed at exit 0 and then printed
/// `<path> (staged, in no commit)` … `they are not recoverable`.
///
/// The verdict is iterated over the whole holding × standing grid; the empty registration
/// still lands, the settled sub-task named in no manifest.
#[test]
fn the_boundary_refuses_over_what_a_settled_sub_tasks_registration_holds() {
    let mut refused = 0;
    for holding in HOLDINGS {
        for standing in STANDINGS {
            let cell = format!(
                "{} × a settled sub-task × {holding:?} × {standing:?}",
                FINALIZE_DOOR.verb
            );
            let fx = Fixture::mint_recorded("settled");
            fx.stage_code(SIBLING);
            let planted = fx.plant(holding, standing);
            fx.jigc_ok(&["task", "discard", SUB]);
            let before = fx.main_checkout();

            let (out, _, stderr) = run_door(&fx, &FINALIZE_DOOR, false, false);
            if !holding.any() {
                assert!(
                    out.status.success(),
                    "{cell}: an empty registration holds nothing, so the boundary lands; \
                     got {:?}\n{stderr}",
                    out.status,
                );
                assert!(
                    !fx.admin(SUB).exists(),
                    "{cell}: …and the teardown still drops the settled sub-task's worktree",
                );
                continue;
            }
            refused += 1;
            assert_eq!(
                out.status.code(),
                Some(3),
                "{cell}: nothing lands from a settled sub-task, so what its registration \
                 holds would be dropped — the boundary must refuse; got {:?}\n{stderr}",
                out.status,
            );
            assert!(
                stderr.contains(&format!("blocking · {}", worktree_code(&FINALIZE_DOOR))),
                "{cell}: under the boundary's own code; stderr:\n{stderr}",
            );
            assert_names(
                &stderr,
                &planted,
                true,
                &cell,
                "the refusal says what is held",
            );
            assert!(
                stderr.contains("settled"),
                "{cell}: the refusal says WHY the boundary would not carry it — the \
                 sub-task is settled; stderr:\n{stderr}",
            );
            assert!(
                !stderr.contains(" reset --soft "),
                "{cell}: nothing lands from a settled sub-task, so no landing exit is \
                 offered; stderr:\n{stderr}",
            );
            assert_eq!(
                fx.main_checkout(),
                before,
                "{cell}: a refusal commits nothing"
            );
            assert_eq!(
                bytes_under(&fx.admin(SUB)),
                planted.admin,
                "{cell}: …and leaves git's registration byte-identical",
            );
        }
    }
    assert!(refused > 0, "no settled cell refused");
}

/// **The settled sub-task's exits work as printed.** A staged path in its live worktree is
/// kept with the `git stash` the refusal prints — a stash is a ref of the repository, so it
/// outlives the worktree — and where the directory is gone the restore recipe brings the
/// checkout back first. Either way the boundary then lands the rest of the milestone.
#[test]
fn a_settled_sub_tasks_staged_path_is_kept_by_the_stash_the_refusal_prints() {
    for standing in STANDINGS {
        let cell = format!(
            "{} × a settled sub-task × Staged × {standing:?} × the stash",
            FINALIZE_DOOR.verb
        );
        let fx = Fixture::mint_recorded("settled-stash");
        fx.stage_code(SIBLING);
        fx.plant(Holding::Staged, standing);
        fx.jigc_ok(&["task", "discard", SUB]);

        let mut stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
        if standing == Standing::Stale {
            let recipe = span_where(
                &stderr,
                &fx.printed(),
                "the recipe that brings the checkout back",
                &cell,
                |span| span.starts_with("mkdir -p "),
            );
            run_as_printed(&fx, &recipe, &cell);
            // Live again — and still a settled sub-task's, so still not carried.
            stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
        }
        let stash = span_where(
            &stderr,
            &fx.printed(),
            "the command that keeps the staged path",
            &cell,
            |span| span.ends_with(" stash"),
        );
        run_as_printed(&fx, &stash, &cell);

        fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
        assert!(
            !fx.admin(SUB).exists(),
            "{cell}: the boundary landed and tore the worktree down",
        );
        assert_eq!(
            git_ok(&fx.repo, &["show", &format!("stash@{{0}}:{}", STAGED.0)]),
            STAGED.1.trim_end(),
            "{cell}: the staged bytes outlive the worktree, in the repository's stash",
        );
        let tree = git_ok(&fx.repo, &["ls-tree", "-r", "--name-only", "HEAD"]);
        assert!(
            tree.lines().any(|path| path == format!("{SIBLING}.txt"))
                && !tree.lines().any(|path| path == STAGED.0),
            "{cell}: the milestone landed the sibling and nothing from the settled \
             sub-task; tree:\n{tree}",
        );
    }
}

// ---------------------------------------------------------------------------
// The boundary in a copied repository.
// ---------------------------------------------------------------------------

/// **A sub-task the boundary lands is asked at its path, registered here or not.** A `cp -R`
/// of the repository leaves the copy's worktrees live and registered at the *source's*
/// paths: the copy's boundary reads their staged code all the same, and its teardown leaves
/// them standing. A commit in one is still work the milestone would be settled without — so
/// the copy's boundary refuses, says the teardown would leave the worktree where it is, and
/// its keep command (aimed at the checkout, whose refs are the source's) clears the path.
#[test]
fn a_copied_repositorys_boundary_refuses_over_a_commit_in_a_worktree_it_never_registered() {
    let cell = format!("{} × Commit × Live × a `cp -R` copy", FINALIZE_DOOR.verb);
    let fx = Fixture::mint("copy");
    let copy = fx.repo.parent().expect("the fixture root").join("copy");
    let copied = Command::new("cp")
        .arg("-R")
        .arg(&fx.repo)
        .arg(&copy)
        .status()
        .expect("run cp -R");
    assert!(copied.success(), "{cell}: fixture — the copy is made");
    let worktree = copy.join(".jigc").join("worktrees").join(SUB);
    let sibling = copy.join(".jigc").join("worktrees").join(SIBLING);
    fs::write(sibling.join("copied.txt"), "code\n").expect("write");
    git_ok(&sibling, &["add", "copied.txt"]);
    fs::write(worktree.join(COMMITTED.0), COMMITTED.1).expect("write");
    git_ok(&worktree, &["add", COMMITTED.0]);
    git_ok(
        &worktree,
        &["commit", "-q", "-m", "the sub-agent committed"],
    );
    let head = git_ok(&worktree, &["rev-parse", "HEAD"]);
    assert!(
        !git_ok(&copy, &["worktree", "list", "--porcelain"])
            .contains(&format!("worktree {}", worktree.display())),
        "{cell}: fixture — the copy has no registration at its own worktree path",
    );

    let out = fx.run_in(&copy, &["milestone", "finalize", MILESTONE]);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(3),
        "{cell}: the milestone would be settled without the commit; got {:?}\n{stderr}",
        out.status,
    );
    assert!(
        stderr.contains(&head[..7]) && stderr.contains("has not registered"),
        "{cell}: the refusal names the commit and says what its teardown would do — \
         nothing; stderr:\n{stderr}",
    );
    let keep = span_where(
        &stderr,
        &fx.printed(),
        "the command that keeps the commit",
        &cell,
        |span| span.contains(" branch ") && span.contains(&head),
    );
    run_as_printed(&fx, &keep, &cell);
    let out = fx.run_in(&copy, &["milestone", "finalize", MILESTONE]);
    assert!(
        out.status.success(),
        "{cell}: with the commit kept, the copy's boundary lands; got {:?}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr),
    );
    assert_eq!(
        git_ok(&copy, &["show", "HEAD:copied.txt"]),
        "code",
        "{cell}: …the sibling's staged code",
    );
    assert!(
        worktree.join(".git").exists(),
        "{cell}: …and its teardown left the worktree it never registered standing",
    );
}

/// **The boundary fails closed, and never prints a command it cannot back.** Two states
/// where the registration's work is real and the ordinary commands do not apply:
///
///   * the registration **cannot be read** (its `HEAD` is not one git accepts) — *it holds
///     nothing* is the one thing the probe did not establish, and this door settles the
///     milestone for good, so it refuses, quotes git's own message, and says there is no
///     command to run rather than pointing at one;
///   * a **file** stands at the path of a registration whose directory is gone — the restore
///     recipe would `mkdir` over it, so the refusal says to move it aside, and once it is
///     gone the same door prints the recipe, which then lands the staged path.
#[test]
fn the_boundary_fails_closed_where_it_cannot_read_or_reach_the_registration() {
    // (1) Unreadable.
    {
        let cell = format!("{} × an unreadable registration", FINALIZE_DOOR.verb);
        let fx = Fixture::mint_recorded("unreadable");
        fx.stage_code(SIBLING);
        fx.plant(Holding::Staged, Standing::Stale);
        fs::write(fx.admin(SUB).join("HEAD"), "not a head\n").expect("corrupt the HEAD");
        let before = fx.main_checkout();
        let stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
        assert!(
            stderr.contains(&fx.printed()) && stderr.contains("could not be read"),
            "{cell}: the refusal names the path and says the registration could not be \
             read; stderr:\n{stderr}",
        );
        assert!(
            stderr.contains("there is no command to run"),
            "{cell}: …and does not send the reader to a command that is not on the line; \
             stderr:\n{stderr}",
        );
        assert_eq!(fx.main_checkout(), before, "{cell}: nothing was committed");
        assert_eq!(
            fx.record_status().as_deref(),
            Some("active"),
            "{cell}: the record stays `active`",
        );
    }

    // (2) A file in the way.
    {
        let cell = format!("{} × Staged × a file at the path", FINALIZE_DOOR.verb);
        let fx = Fixture::mint("file-in-the-way");
        fx.stage_code(SIBLING);
        fx.plant(Holding::Staged, Standing::Stale);
        let path = fx.worktree(SUB);
        fs::write(&path, "left behind\n").expect("write a file at the worktree path");
        let stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
        assert!(
            stderr.contains(STAGED.0) && stderr.contains("move it aside"),
            "{cell}: the refusal names the staged path and the move that unblocks it; \
             stderr:\n{stderr}",
        );
        assert!(
            !stderr.contains("mkdir -p "),
            "{cell}: the recipe cannot run over a file, so it is not printed; \
             stderr:\n{stderr}",
        );
        fs::rename(&path, fx.home.path().join("moved-aside")).expect("move the file aside");
        let stderr = refusal_of(&fx, &FINALIZE_DOOR, &cell);
        let recipe = span_where(
            &stderr,
            &fx.printed(),
            "the recipe that brings the checkout back",
            &cell,
            |span| span.starts_with("mkdir -p "),
        );
        run_as_printed(&fx, &recipe, &cell);
        fx.jigc_ok(&["milestone", "finalize", MILESTONE]);
        assert_eq!(
            git_ok(&fx.repo, &["show", &format!("HEAD:{}", STAGED.0)]),
            STAGED.1.trim_end(),
            "{cell}: the sub-agent's staged code is recovered AND landed",
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
