//! **A route case** — the frame of the route catalog (`crates/cli/tests/route_catalog.rs`;
//! the plan's step `P1`, `DECISIONS.md` → 2026-10-10, ruling 3: *the routes are
//! pre-constructed once, as tests*).
//!
//! A refusal prints a route, and the exit rule's sentence about it is *every refusal's route
//! works as printed*. The repository runs printed routes in many suites, each with its own
//! lifter — and the one suite that planted the state of the `setup.install-hook` regression
//! was green over it, because it took the route's step as a path it wrote by hand
//! (`replacing_writers_never_follow.rs`: `fs::remove_file(repo.join(".git/hooks/pre-commit"))`)
//! where the route had printed another one. So this frame has one rule above the rest:
//!
//! **An act's operand is a span of the printed route, by index.** [`follow`] provokes the
//! refusal, lifts the `route:` line of the finding that carries the case's code, splits it
//! into its backticked [`spans`], and hands an [`Act`] the bytes of the span it names —
//! never a path the case spells. An act on an object the route does not print is a second,
//! **marked** form ([`Operand::Unprinted`]), and [`follow`] refuses it wherever the printed
//! route holds a path span ([`path_spans`]), so it cannot stand in for the printed operand.
//!
//! **A case is data** ([`Case`]): the site as the committed list of refusal sites names it,
//! the cause in words, the state, the refusing argv, the acts, what must hold afterwards,
//! and the files outside the repository whose bytes must not change. It holds no closure,
//! so nothing a case does can go around the runner.
//!
//! **What the runner does, in order.** A sandbox — `repo/`, `home/`, `outside/` under one
//! scratch root — holds a repository with one commit and no `jigc setup`, built under the
//! faithful mask ([dev-workflow.md](../../../../implementation/dev-workflow.md) → Gate:
//! every system source of git's configuration dropped, no global file, no invented
//! identity) with `$HOME` in the sandbox and no inherited `JIGC_PACK_DIR`; every child of
//! the case runs under the same mask. The case's [`Step`]s plant its state; a script is
//! placed by a child process, as that section's Linux rule asks of a file that may be
//! executed. The refusing argv runs on the binary **handed in** — the frame names no
//! binary of its own, so the same case runs on another release's. Then the acts, then the
//! checks, then the witnesses, and one of seven [`Classification`]s.
//!
//! **Where a route names an act and not its object, the runner owns the object.** *Move
//! the directory out of the way* says where from and never where to; *point it at a regular
//! file inside this repository* names no file. A case may not supply either — that would be
//! a path of the case's own beside a printed one — so the runner does: a moved directory
//! goes to the sandbox's `moved/` under the path it had ([`Place::Moved`]), and a link is
//! re-pointed at one of two files of the runner's, the one the sandbox's repository holds
//! committed ([`COMMITTED`]) or one written new, in no commit ([`NEW_FILE`]). Which of the
//! two a route means is a reading, and a case says which it followed
//! ([`Act::PointAtCommitted`], [`Act::PointAtNew`]).
//!
//! **An act never leaves the sandbox.** The runner does what a route says with a path the
//! product printed; a route that names a path outside the sandbox is not followed there —
//! the act is recorded as failed and says why.
//!
//! **A child is waited for no longer than a limit** ([`LIMIT`]). A product that holds —
//! on a read of a FIFO, which does not return — is ended and the case says so: the refusing
//! command held is *not provoked*, a command of the route held is *another failure*.
//!
//! **What it does not decide.** Whether a refusal *should* have been printed, what a route's
//! prose means beyond the acts a case lists, and whether a red case is a regression — a
//! classification is what happened on the binary handed in, and no more.

use std::fmt;
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use super::scratch::ScratchDir;

/// Where a case's state names a file: under the repository, or under the sandbox's
/// `outside/` — a directory that is no part of the repository.
#[derive(Clone, Copy, Debug)]
pub enum Place {
    Repo(&'static str),
    Outside(&'static str),
    /// Under the sandbox's `moved/` — where [`Act::MoveAway`] puts what a route says to
    /// move out of the way: what stood at the repository path `p` is afterwards at
    /// `Moved(p)`. A check's place, never a state's.
    Moved(&'static str),
}

/// The one file the sandbox's repository holds committed before a case's state is planted,
/// from the repository root — what [`Act::PointAtCommitted`] points a link at.
pub const COMMITTED: &str = "README.md";

/// The file [`Act::PointAtNew`] writes and points a link at, from the repository root: a
/// regular file inside the repository that is in no commit.
pub const NEW_FILE: &str = "route-case-new-file";

/// The body of [`NEW_FILE`] — one that is valid wherever a route can ask for *a regular
/// file inside this repository*: an empty JSON object, which is also a line of text.
const NEW_BODY: &str = "{}\n";

/// One file act of a case's **state** — what stands before the refusing command runs. A
/// state names its paths itself; only an [`Act`] may not.
#[derive(Clone, Copy, Debug)]
pub enum Step {
    /// An executable script with this body.
    Script(Place, &'static str),
    /// A symbolic link at the first place whose target is the second place's absolute
    /// path. The target need not exist.
    Link(Place, Place),
    /// A regular file with this body, not executable.
    File(Place, &'static str),
    /// A directory, with those on the way to it.
    Dir(Place),
    /// A special file — a FIFO, made by a child `mkfifo`.
    Fifo(Place),
    /// A `git` command line in the repository, under the case's mask: to commit what the
    /// steps before it planted, or to take the sandbox's one commit away again.
    Git(&'static [&'static str]),
}

/// What an [`Act`] acts on.
#[derive(Clone, Copy, Debug)]
pub enum Operand {
    /// The printed route's backticked span of this index, counted from 0 — its bytes as
    /// printed, read as a path from the repository root.
    Span(usize),
    /// **The marked form**: an object the route does not print, named by the case as a
    /// path from the repository root. Refused wherever the printed route holds a path
    /// span; where it stands, the case can come out no better than
    /// [`Classification::HoldsOperandImplied`].
    Unprinted(&'static str),
}

/// One thing a reader does to follow a route.
#[derive(Clone, Copy, Debug)]
pub enum Act {
    /// Remove the file or the link the operand names — the entry itself, never what a
    /// link leads to, and never a directory.
    Remove(Operand),
    /// Run the span of this index as the command line it is, split by a real shell: a
    /// `jigc` line on the binary under test, a `git` line on git.
    Run(usize),
    /// Move the directory the operand names out of the way: out of the repository, to the
    /// sandbox's `moved/` under the path it had ([`Place::Moved`]), with everything in it.
    /// A link to a directory is not one.
    MoveAway(Operand),
    /// Replace the link the operand names with a real directory: the link is removed —
    /// never what it leads to — and an empty directory is made at its path.
    ReplaceWithDirectory(Operand),
    /// Point the link the operand names at a regular file inside the repository that the
    /// repository holds committed — [`COMMITTED`].
    PointAtCommitted(Operand),
    /// Point the link the operand names at a regular file inside the repository that is
    /// new — [`NEW_FILE`], which the runner writes and no commit holds.
    PointAtNew(Operand),
    /// Commit the change at the path the operand names, and at no other path — what a
    /// route asks for with *committing the removal where git tracks the entry*.
    Commit(Operand),
}

impl Act {
    /// What the act acts on, where it takes an operand.
    fn operand(self) -> Option<Operand> {
        match self {
            Act::Remove(operand)
            | Act::MoveAway(operand)
            | Act::ReplaceWithDirectory(operand)
            | Act::PointAtCommitted(operand)
            | Act::PointAtNew(operand)
            | Act::Commit(operand) => Some(operand),
            Act::Run(_) => None,
        }
    }
}

/// One thing that must hold once the route is followed.
#[derive(Clone, Copy, Debug)]
pub enum Check {
    /// The entry at this place is a regular file — not a link, not absent.
    RegularFile(Place),
    /// The entry at this place is a directory — not a link to one.
    Directory(Place),
}

/// One case of the catalog: one refusal site in one cause.
#[derive(Clone, Copy, Debug)]
pub struct Case {
    /// The case's own name — what an expected-red entry and a failure name it by.
    pub name: &'static str,
    /// The refusal site, as `crates/cli/tests/route_catalog/sites.tsv` holds it:
    /// `file::function::code#ordinal`.
    pub site: &'static str,
    /// The cause, in words: the state that reaches the site.
    pub cause: &'static str,
    /// The state, planted over a repository with one commit and no `jigc setup`.
    pub state: &'static [Step],
    /// The refusing command, without the binary.
    pub refusing: &'static [&'static str],
    /// The acts that follow the printed route, in the route's order.
    pub acts: &'static [Act],
    /// What must hold once they are done — beside every [`Act::Run`] exiting 0.
    pub holds: &'static [Check],
    /// Files under `outside/` whose bytes must be afterwards what they were before the
    /// refusing command ran. One that was absent must still be absent.
    pub witnesses: &'static [&'static str],
    /// **A doubtful reading of the route's prose**, where the case rests on one: the two
    /// readings, in words. The case follows the printed words most literally and decides
    /// nothing; a marked case is listed for the human with its route.
    pub doubt: Option<&'static str>,
}

impl Case {
    /// The finding code of the case's site — the `code` of `file::function::code#ordinal`.
    pub fn code(&self) -> &'static str {
        self.site
            .rsplit_once("::")
            .and_then(|(_, tail)| tail.split_once('#'))
            .map(|(code, _)| code)
            .unwrap_or_else(|| {
                panic!(
                    "case `{}`: `{}` is no site id of the form file::function::code#ordinal",
                    self.name, self.site
                )
            })
    }
}

/// What following one case came to. The words are the plan's and are what a table prints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    NotProvoked,
    NoRoutePrinted,
    Holds,
    HoldsOperandImplied,
    SameRefusalAgain,
    WitnessChanged,
    AnotherFailure,
}

impl Classification {
    pub fn as_str(self) -> &'static str {
        match self {
            Classification::NotProvoked => "not provoked",
            Classification::NoRoutePrinted => "provoked, no route printed",
            Classification::Holds => "followed, holds",
            Classification::HoldsOperandImplied => "followed, holds, operand implied",
            Classification::SameRefusalAgain => "followed, the same refusal again",
            Classification::WitnessChanged => "followed, a witness changed",
            Classification::AnotherFailure => "followed, another failure",
        }
    }

    /// Every classification is red but the two that hold. *The same refusal again* is red
    /// whatever the route printed; a case that no longer provokes its refusal is red too —
    /// it says nothing about the route any more.
    pub fn is_red(self) -> bool {
        !matches!(
            self,
            Classification::Holds | Classification::HoldsOperandImplied
        )
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// What the runner saw of one case — the facts a classification is read from.
#[derive(Clone, Copy, Debug, Default)]
pub struct Seen {
    /// The refusing command exited non-zero and printed a finding of the case's code.
    pub provoked: bool,
    /// That finding carries a `route:` line.
    pub route_printed: bool,
    /// A witness's bytes are not what they were.
    pub witness_changed: bool,
    /// A command of the route was refused under the case's own code again.
    pub same_refusal: bool,
    /// An act could not be done, a command of the route failed any other way, or a
    /// check did not hold.
    pub failed: bool,
    /// An act took the marked form.
    pub implied: bool,
}

/// The one classification of what was seen. **The order is the rule**: a witness that
/// changed outranks everything a followed route can otherwise come to — a lost file is
/// the worst of them, and the refusal that usually follows it would hide it — and a
/// repeated refusal outranks any other failure.
pub fn classify(seen: &Seen) -> Classification {
    if !seen.provoked {
        Classification::NotProvoked
    } else if !seen.route_printed {
        Classification::NoRoutePrinted
    } else if seen.witness_changed {
        Classification::WitnessChanged
    } else if seen.same_refusal {
        Classification::SameRefusalAgain
    } else if seen.failed {
        Classification::AnotherFailure
    } else if seen.implied {
        Classification::HoldsOperandImplied
    } else {
        Classification::Holds
    }
}

/// **A case that is red, and known to be**: its name, the key of the finding it
/// reproduces, and the classification it is red with.
#[derive(Clone, Copy, Debug)]
pub struct ExpectedRed {
    pub case: &'static str,
    pub key: &'static str,
    pub classification: Classification,
}

/// Every way `expected` and what the cases came to disagree — empty when the list is
/// exact. **Strict three ways**: a listed case that is not red, a red case that is not
/// listed, and a listed case whose classification moved are each a difference; so is an
/// entry for a case that is not among the results, a case listed twice, and an entry that
/// expects a classification that is not red.
pub fn differences(expected: &[ExpectedRed], results: &[(&str, Classification)]) -> Vec<String> {
    let mut out = Vec::new();
    for (at, entry) in expected.iter().enumerate() {
        if !entry.classification.is_red() {
            out.push(format!(
                "the entry for `{}` (`{}`) expects `{}`, which is not red — an entry lists \
                 a red case",
                entry.case, entry.key, entry.classification
            ));
            continue;
        }
        if expected[..at].iter().any(|other| other.case == entry.case) {
            out.push(format!(
                "`{}` is listed twice — one entry a case, under the one key of the finding \
                 it reproduces",
                entry.case
            ));
            continue;
        }
        match results.iter().find(|(name, _)| *name == entry.case) {
            None => out.push(format!(
                "the entry for `{}` (`{}`) names a case that was not followed — the catalog \
                 holds none of that name, or it could not be followed as written",
                entry.case, entry.key
            )),
            Some((_, got)) if !got.is_red() => out.push(format!(
                "`{}` is listed as red under `{}` and is not red: it came out `{got}` — if \
                 the defect is fixed, the entry goes in the commit that fixes it",
                entry.case, entry.key
            )),
            Some((_, got)) if *got != entry.classification => out.push(format!(
                "`{}` is listed under `{}` as `{}` and came out `{got}` — its classification \
                 moved, and an entry does not cover a case that changed",
                entry.case, entry.key, entry.classification
            )),
            Some(_) => {}
        }
    }
    for (name, got) in results {
        if got.is_red() && !expected.iter().any(|entry| entry.case == *name) {
            out.push(format!(
                "`{name}` is red — `{got}` — and no entry lists it: a red case is a finding, \
                 listed with its key and this classification or fixed"
            ));
        }
    }
    out
}

/// The refusal a command printed under one code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The text of its `route:` line, where it has one.
    pub route: Option<String>,
}

/// The code of a printed finding's first line, if `line` is one: `<severity> · <code> — …`.
fn head_code(line: &str) -> Option<&str> {
    let line = line.trim_start();
    let rest = [
        "blocking (gates at finalize)",
        "blocking",
        "warning",
        "advisory",
    ]
    .iter()
    .find_map(|severity| line.strip_prefix(severity)?.strip_prefix(" · "))?;
    rest.split_once(" — ").map(|(code, _)| code)
}

/// The first finding of `code` in what a command printed (stdout and stderr, as text),
/// with **its own** route: the `route:` line between its first line and the next
/// finding's. A finding that prints none has none — the route of the finding after it is
/// not its route.
pub fn refusal_in(said: &str, code: &str) -> Option<Refusal> {
    let mut lines = said
        .lines()
        .skip_while(|line| head_code(line) != Some(code));
    lines.next()?;
    let route = lines
        .take_while(|line| head_code(line).is_none())
        .filter(|line| line.starts_with(char::is_whitespace))
        .find_map(|line| line.trim_start().strip_prefix("route: "))
        .map(str::to_string);
    Some(Refusal { route })
}

/// The backticked spans of a route, in order — the odd pieces of a split on `` ` ``. An
/// unpaired last backtick opens no span.
pub fn spans(route: &str) -> Vec<&str> {
    let pieces: Vec<&str> = route.split('`').collect();
    pieces
        .iter()
        .enumerate()
        .filter(|(at, _)| at % 2 == 1 && at + 1 < pieces.len())
        .map(|(_, span)| *span)
        .collect()
}

/// The indices of the spans that are **path spans**: a span that, read from `repo`, names
/// an entry that is there (asked without following a link), or that is written as a path
/// — one word holding a `/`. A span that opens with `-` is a flag and never a path; a
/// command line is several words and names no entry.
pub fn path_spans(spans: &[&str], repo: &Path) -> Vec<usize> {
    spans
        .iter()
        .enumerate()
        .filter(|(_, span)| {
            !span.is_empty()
                && !span.starts_with('-')
                && (fs::symlink_metadata(repo.join(span)).is_ok()
                    || (span.contains('/') && !span.contains(char::is_whitespace)))
        })
        .map(|(at, _)| at)
        .collect()
}

/// One case, followed.
#[derive(Clone, Debug)]
pub struct Followed {
    pub classification: Classification,
    /// The route as printed — the bytes every [`Operand::Span`] was taken from.
    pub route: Option<String>,
    /// What happened, step by step, with the sandbox's root written `<sandbox>`.
    pub detail: String,
}

/// What one child of a case came to.
struct Ran {
    /// Its exit code — `None` where a signal ended it, the runner's own included.
    code: Option<i32>,
    /// The limit it was ended at, where it did not end by itself.
    held: Option<Duration>,
    /// Its stdout, then its stderr, as text.
    said: String,
}

impl Ran {
    fn success(&self) -> bool {
        self.held.is_none() && self.code == Some(0)
    }

    /// How it ended, in words.
    fn exit(&self) -> String {
        match (self.held, self.code) {
            (Some(limit), _) => format!("did not end within {limit:?} and was ended"),
            (None, Some(code)) => format!("exit {code}"),
            (None, None) => "killed by a signal".to_string(),
        }
    }
}

/// A case's sandbox: one scratch root, removed when dropped.
struct Sandbox {
    /// The canonical root, so a path the product prints canonical compares equal.
    root: PathBuf,
    /// How long a child may run before it is ended.
    limit: Duration,
    /// How many children have run — each writes to files of its own.
    children: AtomicUsize,
    _scratch: ScratchDir,
}

impl Sandbox {
    /// A repository with one commit and no `jigc setup`, a `$HOME` and an `outside/`.
    fn new(label: &str, limit: Duration) -> Sandbox {
        let scratch = ScratchDir::new(&format!("route-case-{label}"));
        let root = fs::canonicalize(scratch.path()).expect("the scratch root is there");
        let sandbox = Sandbox {
            root,
            limit,
            children: AtomicUsize::new(0),
            _scratch: scratch,
        };
        for dir in [
            sandbox.repo(),
            sandbox.home(),
            sandbox.outside(),
            sandbox.staging(),
        ] {
            fs::create_dir_all(&dir).expect("create a sandbox directory");
        }
        sandbox.git(&["init", "-q", "-b", "main"]);
        sandbox.git(&["config", "user.name", "Route Case"]);
        sandbox.git(&["config", "user.email", "route-case@example.com"]);
        sandbox.git(&["config", "user.useConfigOnly", "true"]);
        fs::write(sandbox.repo().join(COMMITTED), "a repository\n").expect("write a file");
        sandbox.git(&["add", COMMITTED]);
        sandbox.git(&["commit", "-q", "-m", "initial"]);
        sandbox
    }

    fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    fn outside(&self) -> PathBuf {
        self.root.join("outside")
    }

    fn staging(&self) -> PathBuf {
        self.root.join("staging")
    }

    fn moved(&self) -> PathBuf {
        self.root.join("moved")
    }

    fn at(&self, place: Place) -> PathBuf {
        match place {
            Place::Repo(path) => self.repo().join(path),
            Place::Outside(path) => self.outside().join(path),
            Place::Moved(path) => self.moved().join(path),
        }
    }

    /// `text` with this sandbox's root written `<sandbox>` — a failure reads the same on
    /// every machine.
    fn shown(&self, text: &str) -> String {
        text.replace(&self.root.display().to_string(), "<sandbox>")
    }

    /// A child in the repository, under the faithful mask: no system source of git's
    /// configuration, no global file, `$HOME` in the sandbox, and the embedded packs.
    fn child(&self, program: &Path) -> Command {
        let mut command = Command::new(program);
        command
            .current_dir(self.repo())
            .env("HOME", self.home())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env_remove("JIGC_PACK_DIR")
            .stdin(Stdio::null());
        command
    }

    /// **Run one child and wait for it — no longer than the limit.** Its output goes to
    /// two files of its own under `staging/`, so a child that prints more than a pipe holds
    /// is not kept waiting for a reader; one that has not ended at the limit is killed —
    /// this child, by its handle — and says so ([`Ran::exit`]). A product that holds would
    /// otherwise hold the suite: a read of a FIFO does not return.
    fn run(&self, command: &mut Command) -> Ran {
        let at = self.children.fetch_add(1, Ordering::Relaxed);
        let (out, err) = (
            self.staging().join(format!("child-{at}.out")),
            self.staging().join(format!("child-{at}.err")),
        );
        let file = |path: &Path| fs::File::create(path).expect("create a child's output file");
        let mut child = command
            .stdout(file(&out))
            .stderr(file(&err))
            .spawn()
            .expect("start a child of the case");
        let started = Instant::now();
        let (status, held) = loop {
            if let Some(status) = child.try_wait().expect("ask after a child") {
                break (status, None);
            }
            if started.elapsed() >= self.limit {
                // Raced with its own end, the kill fails and the wait still answers.
                let _ = child.kill();
                break (
                    child.wait().expect("wait for an ended child"),
                    Some(self.limit),
                );
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        let text =
            |path: &Path| String::from_utf8_lossy(&fs::read(path).unwrap_or_default()).into_owned();
        Ran {
            code: status.code(),
            held,
            said: format!("{}{}", text(&out), text(&err)),
        }
    }

    fn git(&self, args: &[&str]) {
        let ran = self.run(self.child(Path::new("git")).args(args));
        assert!(ran.success(), "git {args:?}: {}\n{}", ran.exit(), ran.said);
    }

    fn plant(&self, state: &[Step]) {
        for (at, step) in state.iter().enumerate() {
            match *step {
                Step::Script(place, body) => {
                    let path = self.at(place);
                    fs::create_dir_all(path.parent().expect("a script has a parent"))
                        .expect("create the script's directory");
                    // The Linux rule: a file that may be executed is written by a child
                    // process, so no thread of this one ever holds its write handle and
                    // no sibling's fork can inherit one (`tooling-tests/placed_executable.rs`).
                    let staged = self.staging().join(format!("step-{at}"));
                    fs::write(&staged, body).expect("stage the script");
                    let placed = Command::new("cp")
                        .arg("--")
                        .arg(&staged)
                        .arg(&path)
                        .stdin(Stdio::null())
                        .status()
                        .expect("run cp");
                    assert!(placed.success(), "cp to place {}", path.display());
                    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
                        .expect("make the script executable");
                }
                Step::Link(link, target) => {
                    let path = self.at(link);
                    fs::create_dir_all(path.parent().expect("a link has a parent"))
                        .expect("create the link's directory");
                    symlink(self.at(target), &path).expect("plant the link");
                }
                Step::File(place, body) => {
                    let path = self.at(place);
                    fs::create_dir_all(path.parent().expect("a file has a parent"))
                        .expect("create the file's directory");
                    fs::write(&path, body).expect("plant the file");
                }
                Step::Dir(place) => {
                    fs::create_dir_all(self.at(place)).expect("plant the directory");
                }
                Step::Fifo(place) => {
                    let path = self.at(place);
                    fs::create_dir_all(path.parent().expect("a FIFO has a parent"))
                        .expect("create the FIFO's directory");
                    let made = Command::new("mkfifo")
                        .arg(&path)
                        .stdin(Stdio::null())
                        .status()
                        .expect("run mkfifo");
                    assert!(made.success(), "mkfifo {}", path.display());
                }
                Step::Git(args) => self.git(args),
            }
        }
    }

    fn witnesses(&self, names: &[&str]) -> Vec<Option<Vec<u8>>> {
        names
            .iter()
            .map(|name| fs::read(self.outside().join(name)).ok())
            .collect()
    }

    /// Whether `path` lies in this sandbox — asked of the directory that holds it,
    /// resolved, so that neither a link on the way nor a `..` leads an act out.
    fn contains(&self, path: &Path) -> bool {
        path.parent()
            .and_then(|parent| fs::canonicalize(parent).ok())
            .is_some_and(|parent| parent.starts_with(&self.root))
    }

    /// Do one act. `Ok` is what happened, for the detail; `Err` is a fault of the case as
    /// written, which no classification covers.
    fn perform(
        &self,
        binary: &Path,
        act: Act,
        spans: &[&str],
        code: &str,
        seen: &mut Seen,
    ) -> Result<String, String> {
        let span = |at: usize| {
            spans.get(at).copied().ok_or_else(|| {
                format!(
                    "an act names span {at}, and the printed route has {} spans",
                    spans.len()
                )
            })
        };
        if let Some(operand) = act.operand() {
            let (named, how) = match operand {
                Operand::Span(at) => (span(at)?, format!("span {at}")),
                Operand::Unprinted(path) => (path, "named by the case".to_string()),
            };
            let path = self.repo().join(named);
            let what = match act {
                Act::Remove(_) => format!("remove `{named}` ({how})"),
                Act::MoveAway(_) => format!("move `{named}` out of the way ({how})"),
                Act::ReplaceWithDirectory(_) => {
                    format!("replace the link `{named}` with a real directory ({how})")
                }
                Act::PointAtCommitted(_) => {
                    format!("point the link `{named}` at `{COMMITTED}` ({how})")
                }
                Act::PointAtNew(_) => format!("point the link `{named}` at `{NEW_FILE}` ({how})"),
                Act::Commit(_) => format!("commit the change at `{named}` ({how})"),
                Act::Run(_) => unreachable!("a run takes no operand"),
            };
            if !self.contains(&path) {
                seen.failed = true;
                return Ok(format!(
                    "{what}: not done — it names nothing inside the case's sandbox"
                ));
            }
            let done = match act {
                Act::Remove(_) => fs::remove_file(&path)
                    .map(|()| "removed".to_string())
                    .map_err(|e| e.to_string()),
                Act::MoveAway(_) => self.move_away(&path),
                Act::ReplaceWithDirectory(_) => self
                    .a_link(&path)
                    .and_then(|()| fs::remove_file(&path).map_err(|e| e.to_string()))
                    .and_then(|()| fs::create_dir(&path).map_err(|e| e.to_string()))
                    .map(|()| "the link is gone and a directory stands there".to_string()),
                Act::PointAtCommitted(_) => self.point(&path, COMMITTED),
                Act::PointAtNew(_) => self
                    .a_link(&path)
                    .and_then(|()| {
                        fs::write(self.repo().join(NEW_FILE), NEW_BODY).map_err(|e| e.to_string())
                    })
                    .and_then(|()| self.point(&path, NEW_FILE)),
                Act::Commit(_) => self.commit(named),
                Act::Run(_) => unreachable!("a run takes no operand"),
            };
            return Ok(match done {
                Ok(done) => format!("{what}: {done}"),
                Err(why) => {
                    seen.failed = true;
                    format!("{what}: {why}")
                }
            });
        }
        let Act::Run(at) = act else {
            unreachable!("every act but a run takes an operand");
        };
        let line = span(at)?;
        let words = super::shell_words(line, &self.repo(), &self.home());
        let program = match words.first().map(String::as_str) {
            Some("jigc") => binary,
            Some("git") => Path::new("git"),
            _ => {
                return Err(format!(
                    "an act runs span {at}, `{line}`, which is no `jigc` or `git` \
                     command line"
                ));
            }
        };
        let ran = self.run(self.child(program).args(&words[1..]));
        if ran.success() {
            return Ok(format!("run `{line}` (span {at}): exit 0"));
        }
        Ok(
            if ran.held.is_none() && refusal_in(&ran.said, code).is_some() {
                seen.same_refusal = true;
                format!("run `{line}` (span {at}): {}, `{code}` again", ran.exit())
            } else {
                seen.failed = true;
                format!("run `{line}` (span {at}): {}\n{}", ran.exit(), ran.said)
            },
        )
    }

    /// `Ok` where the entry at `path` is a link, asked without following it.
    fn a_link(&self, path: &Path) -> Result<(), String> {
        if fs::symlink_metadata(path).is_ok_and(|entry| entry.file_type().is_symlink()) {
            Ok(())
        } else {
            Err("not done — no link stands there".to_string())
        }
    }

    /// Move the directory at `path` to `moved/`, under the path it had from the repository
    /// root — or under its own name, where it stood elsewhere in the sandbox.
    fn move_away(&self, path: &Path) -> Result<String, String> {
        if !fs::symlink_metadata(path).is_ok_and(|entry| entry.is_dir()) {
            return Err("not done — no directory stands there".to_string());
        }
        let under = path
            .strip_prefix(self.repo())
            .ok()
            .map(Path::to_path_buf)
            .or_else(|| path.file_name().map(PathBuf::from))
            .ok_or("not done — the path has no name to move it under")?;
        let to = self.moved().join(under);
        let parent = to.parent().expect("a place under `moved/` has a parent");
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        fs::rename(path, &to).map_err(|e| e.to_string())?;
        Ok("moved out of the repository".to_string())
    }

    /// Point the link at `path` at the repository's file `target`: the link is removed and
    /// planted again, and what it led to is not touched.
    fn point(&self, path: &Path, target: &str) -> Result<String, String> {
        self.a_link(path)?;
        fs::remove_file(path).map_err(|e| e.to_string())?;
        symlink(self.repo().join(target), path).map_err(|e| e.to_string())?;
        Ok("the link leads there now".to_string())
    }

    /// Commit the change at the path `named`, and at no other path.
    fn commit(&self, named: &str) -> Result<String, String> {
        for args in [
            &["add", "--all", "--", named][..],
            &["commit", "-q", "-m", "follow the route", "--", named][..],
        ] {
            let ran = self.run(self.child(Path::new("git")).args(args));
            if !ran.success() {
                return Err(format!("git {args:?}: {}\n{}", ran.exit(), ran.said));
            }
        }
        Ok("committed".to_string())
    }

    fn check(&self, check: Check) -> Result<(), String> {
        match check {
            Check::RegularFile(place) => {
                let path = self.at(place);
                if fs::symlink_metadata(&path).is_ok_and(|entry| entry.is_file()) {
                    Ok(())
                } else {
                    Err(format!("`{}` is no regular file", path.display()))
                }
            }
            Check::Directory(place) => {
                let path = self.at(place);
                if fs::symlink_metadata(&path).is_ok_and(|entry| entry.is_dir()) {
                    Ok(())
                } else {
                    Err(format!("`{}` is no directory", path.display()))
                }
            }
        }
    }
}

/// **Follow one case on `binary`.** `Err` is a fault of the case as written — the marked
/// form where the printed route holds a path span, a span the route does not have, no act
/// under a printed route — and is no classification: the catalog names it and fails.
pub fn follow(binary: &Path, case: &Case) -> Result<Followed, String> {
    follow_within(binary, case, LIMIT)
}

/// How long a child of a case may run before the runner ends it. A `jigc setup` over a
/// sandbox takes a fraction of a second; the limit is two minutes so that a loaded machine
/// never turns a slow command into a red case, and a command that holds is one red case
/// after two minutes rather than a suite that does not come back.
pub const LIMIT: Duration = Duration::from_secs(120);

/// [`follow`], with the limit a child is ended at — for the arm that holds the limit
/// itself, which cannot wait two minutes for it.
pub fn follow_within(binary: &Path, case: &Case, limit: Duration) -> Result<Followed, String> {
    let code = case.code();
    let sandbox = Sandbox::new(case.name, limit);
    sandbox.plant(case.state);
    let before = sandbox.witnesses(case.witnesses);

    let refused = sandbox.run(sandbox.child(binary).args(case.refusing));
    let found = refusal_in(&refused.said, code);
    let mut seen = Seen {
        provoked: found.is_some() && refused.held.is_none() && !refused.success(),
        ..Seen::default()
    };
    let mut detail = vec![format!(
        "`jigc {}`: {}, `{code}` {}",
        case.refusing.join(" "),
        refused.exit(),
        if found.is_some() {
            "printed"
        } else {
            "not printed"
        }
    )];
    if !seen.provoked {
        detail.push(refused.said);
    }

    let route = found
        .filter(|_| seen.provoked)
        .and_then(|refusal| refusal.route);
    if let Some(route) = &route {
        seen.route_printed = true;
        detail.push(format!("route: {route}"));
        let spans = spans(route);
        if case.acts.is_empty() {
            return Err("a route is printed and the case follows it with no act".to_string());
        }
        for act in case.acts {
            if let Some(Operand::Unprinted(named)) = act.operand() {
                seen.implied = true;
                if let Some(at) = path_spans(&spans, &sandbox.repo()).first() {
                    return Err(sandbox.shown(&format!(
                        "an act names `{named}` by the marked form, and the printed route \
                         holds a path — span {at}, `{}`. Where a route prints a path, the \
                         operand is the route's to give: nothing was done",
                        spans[*at]
                    )));
                }
            }
        }
        for act in case.acts {
            let done = sandbox.perform(binary, *act, &spans, code, &mut seen)?;
            detail.push(done);
        }
        if !seen.same_refusal && !seen.failed {
            for check in case.holds {
                if let Err(why) = sandbox.check(*check) {
                    seen.failed = true;
                    detail.push(format!("does not hold: {why}"));
                }
            }
        }
    }

    let after = sandbox.witnesses(case.witnesses);
    for ((name, before), after) in case.witnesses.iter().zip(&before).zip(&after) {
        if before != after {
            seen.witness_changed = true;
            detail.push(format!(
                "the witness `outside/{name}` changed: {}",
                match (before, after) {
                    (Some(_), None) => "it is gone",
                    (None, Some(_)) => "it was created",
                    _ => "its bytes differ",
                }
            ));
        }
    }

    Ok(Followed {
        classification: classify(&seen),
        detail: sandbox.shown(&detail.join("\n")),
        route,
    })
}
