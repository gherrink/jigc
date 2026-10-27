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
//! and the files whose bytes must not change — outside the repository, and inside it where
//! the command has no business touching them ([`Case::untouched`]). It holds no closure,
//! so nothing a case does can go around the runner.
//!
//! **A command that removes is held to what is gone and to what still stands**
//! ([`Check::Absent`], [`Check::Lacks`], [`Check::Contains`]). `jigc setup` refuses over a
//! file it is to write, and what must hold once its route is followed is that file;
//! `jigc uninstall` refuses over something it is to take away, and what must hold is that
//! it is gone — from the tree, or from the file it was merged into — and that somebody's
//! own lines in that file are still there.
//!
//! **What the runner does, in order.** A sandbox — `repo/`, `home/`, `outside/` under one
//! scratch root — holds a repository with one commit and no `jigc setup`, built under the
//! faithful mask ([dev-workflow.md](../../../../implementation/dev-workflow.md) → Gate:
//! every system source of git's configuration dropped, no global file, no invented
//! identity) with `$HOME` in the sandbox, the sandbox's own identity and no inherited
//! `JIGC_PACK_DIR`; every child of the case runs under the same mask. The case's [`Step`]s plant its state; a script is
//! placed by a child process, as that section's Linux rule asks of a file that may be
//! executed. The refusing argv runs on the binary **handed in** — the frame names no
//! binary of its own, so the same case runs on another release's. Then the acts, then the
//! checks, then the witnesses, and one of ten [`Classification`]s.
//!
//! **Where a route names an act and not its object, the runner owns the object.** *Move
//! the directory out of the way* says where from and never where to; *point it at a regular
//! file inside this repository* names no file. A case may not supply either — that would be
//! a path of the case's own beside a printed one — so the runner does: a moved directory
//! goes to the sandbox's `moved/` under the path it had ([`Place::Moved`]) — as a file does
//! that a route says to move out of its path ([`Act::MoveOut`]) — and a link is
//! re-pointed at one of two files of the runner's, the one the sandbox's repository holds
//! committed ([`COMMITTED`]) or one written new, in no commit ([`NEW_FILE`]). **The route
//! is held to either** — the human's ruling of 2026-10-12 (`DECISIONS.md` → *The route is
//! held to any file in the checkout*): *a regular file inside this repository* is any file
//! in the checkout, one in no commit included — and a case says which of the two it
//! pointed the link at ([`Act::PointAtCommitted`], [`Act::PointAtNew`]). Where the route asks for *a script*,
//! the file is [`SCRIPT`] ([`Act::PointAtScript`]): the one the case's state planted there,
//! committed or not, or one the runner writes new.
//!
//! **A case stands where it says, in the environment it says** ([`Case::from`],
//! [`Case::env`]). The refusing command and every command its route has run are started in
//! the case's directory — the repository's root for most, a directory that is no repository
//! for a refusal about that — with the case's variables taken out of the environment or set
//! to a place of the sandbox ([`Var`]); a path the route prints is read from where the case
//! stands. The runner's own children are not the case's: the sandbox's setup, a state's
//! step, an act's commit and a check run in the repository under the mask as it stands.
//!
//! **Where a route says *run it again* and prints no command line, the command is the one
//! the case already stated** ([`Act::Rerun`], [`Act::RerunInTheRepository`]). *Run jigc from
//! inside the target git repository* and *reinstall jigc* name an act and no line to run;
//! what follows them is the refusing command once more. A re-run takes nothing from the
//! case — no span, no path, no word: it is the argv the runner provoked the refusal with,
//! on the same binary, in the case's environment. It is the marked form's sibling and is
//! held the same way: the case comes out no better than
//! [`Classification::HoldsOperandImplied`], and [`follow`] **refuses it wherever the
//! printed route holds a command line of the product** ([`command_spans`]) — there the
//! command is the route's to give, by [`Act::Run`], and a case that ran its own in its
//! place would pass over a route that prints a wrong one. **So the rule is over the lines
//! the case passes over** (the human's ruling of 2026-10-12 on a product command printed
//! without its id): a line the case itself runs as printed is passed over by nobody, and
//! a span that names a command — *re-run `jigc task finalize`*, with no id — is no line at
//! all: there the re-run is the reader's own command, run again as they ran it.
//!
//! **Every repository of a sandbox has the sandbox's identity, and no other.** The mask
//! leaves git no identity to read and none to invent, so the runner supplies one — in the
//! environment of every child, as configuration git reads from there (`GIT_CONFIG_COUNT`),
//! never written into one repository's own. A repository a route's `git init` makes
//! therefore has it too, as a reader's would have theirs; and a case takes it away with
//! the variable that carries it ([`Var::Unset`]), after which there is none.
//!
//! **An act never leaves the sandbox.** The runner does what a route says with a path the
//! product printed; a route that names a path outside the sandbox is not followed there —
//! the act is recorded as failed and says why.
//!
//! **A child is waited for no longer than a limit** ([`LIMIT`]). A product that holds —
//! on a read of a FIFO, which does not return — is ended and the case says so, with a
//! classification of its own: *the command did not end*, whether it was the refusing
//! command or one a route prints. A case that is **listed** as one that does not end is
//! waited for no longer than [`EXPECTED_HOLD`] ([`limit_of`]), so a known hold costs a run
//! seconds and not the standing limit. **A
//! limit a caller hands in ([`follow_within`]) is the limit of the commands under test** —
//! the refusing command and the commands a route prints — and of nothing else: the
//! sandbox's own `git` children, a state's and an act's included, are the runner's and are
//! waited for under the standing limit, so a short limit never ends a setup that a loaded
//! machine made slow.
//!
//! **A printed line that no shell can split is a red case, and stops nothing.** A route may
//! print, between backticks, a line that is no command line as it stands —
//! `git -C <repo> rm --cached -- <path>`, where the angle-bracket word is a redirection with
//! nothing behind it. The runner hands every printed line to a real shell to split
//! ([`super::shell_words_or`]); a line the shell refuses is not run, the case says what the
//! shell said, and it comes out with a classification of its own
//! ([`Classification::UnsplittableLine`]) — a route that cannot be run as printed is what
//! the catalog is there to show, and never a fault that ends the arm.
//!
//! **A printed line with a word in angle brackets is a shape the reader fills, and the
//! runner fills it with what the refusal lists** ([`Act::RunFilled`], [`listed_in`]). The
//! surface contract says so (`design/surface-contract.md` → the span fence: *a
//! `<placeholder>` the reader fills is legal bare or as a quoted span*), so such a line is
//! not a command to run as it stands. The value that fills a placeholder is **a value the
//! refusal's own output lists, by index** — bytes the product printed, as every other
//! operand is, and never a word of the case's: the listing is the indented lines of the
//! refusal's finding that are not its route, a backticked span one value and every other
//! word one, the colon or comma that sets a word off dropped. A placeholder for which the
//! refusal lists no value cannot be filled; a case that runs such a line as printed comes
//! out *a printed line no shell can split*, as before. A value that is a path out of the
//! sandbox is not handed to any command.
//!
//! **A printed line that carries a placeholder for a value of the reader's own is followed
//! with the placeholder filled** ([`Act::RunChosen`], [`Own`]) — the human's ruling of
//! 2026-10-12 (`DECISIONS.md` → *A printed line that carries a placeholder for a value of
//! the reader's own is followed with the placeholder filled*). `--title <title>`, `--slug
//! <other-slug>`, `--from-file <payload>`: no refusal could list such a value, so the case
//! names it — which placeholder is the reader's own, the value a reader would choose in
//! the case's state, and why that one — and it is the one word of a case's that an act
//! hands to a command. It goes where the route prints that placeholder and nowhere else:
//! a placeholder the case does not name is not filled, and the line stays *a printed line
//! no shell can split*. **The value is one word to the command, whatever it holds**: a
//! mark stands in its place while a real shell splits the line, and the value takes the
//! mark's place in the word the shell made — bare in the printed line or quoted, it never
//! meets a shell. The filled line is then held to exit 0 like any other line a route
//! prints, and **what was run is in the record**: the words of the command, each as a
//! shell would take it, with every value and why. **A value the line's own grammar refuses
//! is the case's mistake**: where a filled line is refused and its output quotes the value
//! back — the case's own refusal again apart, which is read first — the runner hands back
//! a fault of the case, which no classification covers and no list can hold. A value that
//! is a path out of the sandbox is handed to no command.
//!
//! **A span that names a command without its operand is no line to run, and is followed
//! with the operand the route speaks of** ([`Act::RunNamed`], [`Named`], [`Word`]) — the
//! human's rulings of 2026-10-12 (`DECISIONS.md` → *A span that names a command without its operand is followed with the operand the route speaks of*).
//! *With `git mv` if git has it staged*, *`git add` your changes*, *`jigc unmanage` it*:
//! the span names a command, as the surface contract already reads a bare `git add`.
//! **What a name is, is the command's own word and never a case's**: run bare, its command
//! refuses the line for want of an operand — the product's argument parser at exit 2,
//! naming required arguments that were not provided; git's usage at exit 129, or `git
//! add`'s *Nothing specified, nothing added*. The runner asks the command; it holds no
//! copy of an argument grammar. A case hands a name its operand, a word each: a span the
//! route prints, a value the refusal lists, or a word of the case's own for the operand the
//! route speaks of and prints nowhere — with, in words, which operand the route means. The
//! command is started where the route says it belongs, is held to exit 0 like any line,
//! and the command that was run is in the record. **A name is run by nobody as printed**:
//! an act that does is a fault of the case, and so is one that names operands for a line
//! that is complete. Nothing loosens for a complete line, or for one that fails any other
//! way.
//!
//! **And a filled line the route says something of is held to those words**
//! ([`Chosen::promised`]) — *`jigc start "<intent>"` mints an ordinary task*: the intent is
//! the reader's own, and the filled line's own output must say what the route said the
//! line does, as a cited line's must; a line that does not is red under the same word.
//!
//! **A line a route cites for what it prints is held to its words, and not to its exit**
//! ([`Act::RunCited`], [`Cited`], [`Promise`]) — the human's ruling of 2026-10-12
//! (`DECISIONS.md` → *A command a route cites for what it prints is held to its words*).
//! *Run `git …` to see what git reports*, *`jigc task finalize <id>` to review the fidelity
//! diff*, *`jigc milestone discard <id>`, which answers for the same path with its own
//! route*: such a line is there for its output, and may well be refused itself. A case
//! marks the line cited and states what the route promised of it — read off the route's
//! own words for the case's state, as pieces of text and places of the sandbox. The runner
//! runs the line as printed and does not hold its exit code; **its own output must keep
//! every promise**, or the case comes out with a classification of its own
//! ([`Classification::CitedLineUnsaid`]) — never *another failure*, never *the same
//! refusal again*. The promise is a witness over output where [`Case::witnesses`] are over
//! files, and it is strict: a cited line with no promise, or a promise with no piece, is a
//! fault of the case. **Nothing loosens for any other line**: a line not marked cited is
//! held to exit 0, the lines a route prints behind a cited one included, and a cited line
//! no shell can split is *a printed line no shell can split* as before.
//!
//! **Where a route sends its reader to the refusal's own listing, the runner runs the
//! command that stands there** ([`Act::RunListed`], [`listed_spans_in`]). *Run the command
//! on that line* names no span of the route: the line is one of the refusal's listing, and
//! the command a backticked span of it — bytes the product printed, named by its index
//! among the listing's spans and never spelled by a case. It is run as a printed line of
//! the route is: split by a real shell, a `jigc` line on the binary under test, a `git`
//! line on git. A listed line that is neither — a recipe of several commands joined by a
//! shell's operators — is run by nobody: the runner hands a line no shell to interpret.
//!
//! **And where a route says a checkout must let a path go, the runner drops its index
//! entry** ([`Act::Unstage`]): *have the contending sub-tasks touch distinct files* names
//! an act and prints neither a line nor a path, so the object is the case's, in the marked
//! form, and the git is the runner's — run in the directory that holds the path, since a
//! linked worktree's index is its own.
//!
//! **And where a route says what must stand at a home in a link's place, the runner does
//! it with the link's own target** ([`Act::MoveLinkOut`], [`Act::PutTheFileItself`]). The
//! store's refusal over a home that is no regular file routes two ways: *move the link out
//! of the doc's home* — the link itself goes to `moved/`, as a file or a directory does,
//! and what it led to is not touched — and *put the regular file itself there — for a
//! link, a copy of the file it points at, in the link's place*: the link is removed and a
//! regular file is written at its path with the bytes of the file it led to. Those bytes
//! are the product's state and never a case's; a link that leads to no regular file has
//! none, and the act is not done.
//!
//! **No more chains run at once in one process than the machine is wide** ([`Gate`]).
//! A case is one child process at a time; [`pooled`] bounds how many cases one caller
//! follows at once, and every case followed in this process — by whichever arm, through a
//! pool or by itself — passes one gate of [`width`] before its sandbox is made. Under a
//! harness that runs a suite's tests as threads of one process, two commands' arms then
//! share one machine's width; under one that gives each test a process of its own, each
//! process has its own gate, and nothing here reaches across them. The gate also says how
//! many jobs it has turned back and not yet let in ([`Gate::waiting`]) — what the arm that
//! holds it counts through, since nothing else tells a job the gate refused from one that
//! has not asked.
//!
//! **A state step may run the binary under test** ([`Step::Jigc`]). Where a case's state is
//! one an earlier run of the product left — an install that stands — the product makes it,
//! on the binary handed in: the state is that release's own, and no case copies the
//! product's lines into its state to stand in for one. **And in another place of the
//! sandbox where the product makes the state only from there** ([`Step::JigcAt`]): a
//! milestone's sub-task is re-entered from its own worktree.
//!
//! **A sandbox is removed whole, whatever its state shut.** A state may take the owner's
//! right to write or search a directory away, and a route need not give it back; the
//! runner gives every directory of the sandbox back to its owner before the scratch root
//! is removed, so no case leaves its root behind ([`Followed::sandbox`] is where it stood).
//!
//! **Which of the declared bounds a case's state stands on is read off the state**
//! ([`standing`], [`bound_of`]; the ruling is `DECISIONS.md` → 2026-10-11, *Which states a
//! route must work in*). Three kinds of planted state are bounds — a permission bit changed
//! by hand, a FIFO or a device node planted, a state only `JIGC_ADAPTERS_DIR` reaches — and
//! a red case whose refusal is over one is recorded and does not block. The function reads
//! a case's steps and its environment, never its acts; where a state stands on a bound and
//! on a state that is none, it says so and the case carries one word for which of the two
//! its refusal is over ([`Case::over`]), with what that was read from. **A bound makes no
//! case green**: the runner reads none, and no classification moves by one.
//!
//! **A second frame runs its children through this runner, and no suite does**
//! ([`Sandbox::over`]; `support::byte_cell`, the byte table's frame). What it is handed is
//! visible to the modules under `support/` and to nothing else (`pub(super)`): the sandbox
//! and where its places stand, a child under the mask, the one function that runs a child
//! and waits no longer than a limit, a state's steps, and the gate every chain of this
//! process passes ([`admitted`]). A suite still starts a child of a case in one way only —
//! by handing a case to [`follow`], or a cell to that frame's runner. **A sandbox made
//! over a prepared state is not initialised here**: the repository and the home are
//! brought in as they were built, nothing is committed, and it has a `tmp/` that every
//! child of it has as `TMPDIR` — the product writes under the system's temporary
//! directory, and that frame reads what it left there. A sandbox of a case
//! ([`Sandbox::new`]) has none and is what it was.
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
use std::sync::{Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
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
/// from the repository root — what [`Act::PointAtCommitted`] points a link at. Its body is
/// [`BODY`].
pub const COMMITTED: &str = "README.md";

/// The file [`Act::PointAtNew`] writes and points a link at, from the repository root: a
/// regular file inside the repository that is in no commit.
pub const NEW_FILE: &str = "route-case-new-file";

/// The body of [`COMMITTED`] and of [`NEW_FILE`] — one that is valid wherever a route can
/// ask for *a regular file inside this repository*: an empty JSON object, which is also a
/// line of text.
const BODY: &str = "{}\n";

/// The script [`Act::PointAtScript`] points a link at, from the repository root. A case's
/// state may plant it — and commit it; where none stands, the runner writes one that exits
/// 0.
pub const SCRIPT: &str = "route-case-script";

/// One file act of a case's **state** — what stands before the refusing command runs. A
/// state names its paths itself; only an [`Act`] may not.
#[derive(Clone, Copy, Debug)]
pub enum Step {
    /// An executable script with this body.
    Script(Place, &'static str),
    /// A symbolic link at the first place whose target is the second place's absolute
    /// path. The target need not exist.
    Link(Place, Place),
    /// A symbolic link at the place whose target is these bytes, to the letter — a
    /// relative target, or a path that is no place of the sandbox.
    LinkTo(Place, &'static str),
    /// A regular file with this body, not executable.
    File(Place, &'static str),
    /// A directory, with those on the way to it.
    Dir(Place),
    /// A special file — a FIFO, made by a child `mkfifo`.
    Fifo(Place),
    /// A `git` command line in the repository, under the case's mask: to commit what the
    /// steps before it planted, or to take the sandbox's one commit away again.
    Git(&'static [&'static str]),
    /// **The binary under test, run in the repository with these arguments** — a state an
    /// earlier run of the product left: an install that stands, made by the product and
    /// never by bytes a case copies out of it. It runs under the mask as it stands, like
    /// every step, and must exit 0.
    Jigc(&'static [&'static str]),
    /// **[`Step::Jigc`], run in this place of the sandbox and not at the repository's root**
    /// — a state the product makes only where it is asked from: a milestone's sub-task is
    /// re-entered from its own worktree, and nowhere else. The place is there by the time
    /// the step runs; like every step it runs under the mask and must exit 0.
    JigcAt(Place, &'static [&'static str]),
}

/// One variable of the environment a case's **commands under test** run in — the refusing
/// command and every command the route has run. The runner's own children — the sandbox's
/// setup, a state's step, an act's commit, a check — run under the mask as it stands.
#[derive(Clone, Copy, Debug)]
pub enum Var {
    /// The variable is taken out of the environment.
    Unset(&'static str),
    /// The variable is set to the absolute path of this place of the sandbox.
    Path(&'static str, Place),
}

/// What an [`Act`] acts on.
#[derive(Clone, Copy, Debug)]
pub enum Operand {
    /// The printed route's backticked span of this index, counted from 0 — its bytes as
    /// printed, read as a path from where the case stands ([`Case::from`]).
    Span(usize),
    /// **The marked form**: an object the route does not print, named by the case as a
    /// path from where it stands. Refused wherever the printed route holds a path
    /// span; where it stands, the case can come out no better than
    /// [`Classification::HoldsOperandImplied`].
    Unprinted(&'static str),
}

/// A printed line and what fills it — indices only: the line is a span of the printed
/// route, and each value one the refusal's own output lists ([`listed_in`]).
#[derive(Clone, Copy, Debug)]
pub struct Filled {
    pub line: usize,
    pub with: &'static [usize],
}

/// **One value of the reader's own, for one placeholder of a printed line** — the human's
/// ruling of 2026-10-12 (`DECISIONS.md` → *A printed line that carries a placeholder for a
/// value of the reader's own is followed with the placeholder filled*). Which placeholder
/// is the reader's own is the case's to say, and so is the value: a title, a slug, a
/// payload file — nothing a refusal could list.
#[derive(Clone, Copy, Debug)]
pub struct Own {
    /// The placeholder as the route prints it, angle brackets included: `<slug>`.
    pub placeholder: &'static str,
    /// The value a reader would choose in the case's state — inside whatever the route's
    /// words bound it by: *another* slug, a *distinct* title, a payload that is one.
    pub value: &'static str,
    /// Why a reader would choose that value there. It is printed with the line that was
    /// run; a value with no reason is a fault of the case.
    pub why: &'static str,
}

/// A printed line and the values of the reader's own that fill it: the line a span of the
/// printed route, by index, and a value for each placeholder the case says is the
/// reader's. A placeholder of the line that no [`Own`] names is not filled.
#[derive(Clone, Copy, Debug)]
pub struct Chosen {
    pub line: usize,
    pub own: &'static [Own],
    /// **What the route says the filled line does, where it says so** — *`jigc start
    /// "<intent>"` mints an ordinary task*: the filled line is then held to those words as
    /// a cited line is ([`Promise`]), its own output asked for each. Empty for a line the
    /// route says nothing of but to run it, which is held to exit 0.
    pub promised: &'static [Promise],
}

/// **One word a case hands a command its route names without the operand** ([`Named`]).
#[derive(Clone, Copy, Debug)]
pub enum Word {
    /// The printed route's backticked span of this index: an operand the route prints, a
    /// clause away from the command it belongs to.
    Span(usize),
    /// A value the refusal's own output lists, by index ([`listed_in`]) — where the
    /// refusal lists the path the route speaks of, it is taken from there.
    Listed(usize),
    /// A word of the case's own: the operand the route speaks of and prints nowhere, or
    /// one it leaves to its reader — *the other*, *a free identity*.
    Named(&'static str),
}

/// **A span that names a command without its operand, followed with the operand the route
/// speaks of** — the human's rulings of 2026-10-12 (`DECISIONS.md` → *A span that names a
/// command without its operand is followed with the operand the route speaks of*). The
/// name is a span of the printed route, by index; each operand is one word; and the case
/// says in words which operand the route means, and why a word of its own is the one a
/// reader would choose.
#[derive(Clone, Copy, Debug)]
pub struct Named {
    pub name: usize,
    /// The operands, in the command's order, each put in as one word.
    pub on: &'static [Word],
    /// Where the route says the command belongs — *inside their worktrees* — where it
    /// says so: the directory the command is started in, read from where the case stands.
    pub within: Option<Word>,
    /// Which operand the route means, in words, and why each word of the case's own.
    pub means: &'static str,
}

/// One piece of what a cited line's output must hold ([`Promise`]).
#[derive(Clone, Copy, Debug)]
pub enum Says {
    /// These bytes — the route's promise, spelled out for the case's state.
    Text(&'static str),
    /// The absolute path of this place of the sandbox, as the product prints one: its
    /// components, with nothing behind the last — `Place::Repo("")` is the repository's
    /// root.
    Path(Place),
}

/// **One thing a route promised of a line it cites for what it prints** — a witness over
/// that line's output, as [`Case::witnesses`] are over files. Every piece is held within
/// one line of the output; a promise with no piece is a fault of the case.
#[derive(Clone, Copy, Debug)]
pub enum Promise {
    /// One line of the output holds every one of these.
    Line(&'static [Says]),
    /// One finding of the output carries a `route:` line of its own, and its lines — its
    /// first, the indented ones under it, the route's included — hold every one of these:
    /// what *answers with its own route* and *names that route* ask for.
    Route(&'static [Says]),
}

/// A printed line a route cites for what it prints, and what the route promised of it: the
/// line a span of the printed route, by index, and the promises its output is held to —
/// every one of them.
#[derive(Clone, Copy, Debug)]
pub struct Cited {
    pub line: usize,
    pub promised: &'static [Promise],
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
    /// **Run a printed line whose `<placeholder>` words are filled with values the refusal
    /// lists** ([`Filled`]) — what a reader does with a line the route prints as a shape.
    /// Every placeholder of the line is filled, in the line's order, each as one word of
    /// the command line; then the line is run as [`Act::Run`] runs one.
    RunFilled(Filled),
    /// **Run a printed line whose placeholders for a value of the reader's own are filled
    /// with the values the case names** ([`Chosen`]) — the human's ruling of 2026-10-12:
    /// such a line *is followed with the placeholder filled*, and the filled line is held
    /// to exit 0 like any other line a route prints. Each value is put in where the route
    /// prints its placeholder, bare or quoted, as one word whatever it holds; a
    /// placeholder the case does not name stays as printed, and the line with it one no
    /// shell can split. A filled line that is refused **for the value** — its output
    /// quotes the value back — is the case's mistake and a fault of the case: it says
    /// nothing of the route.
    RunChosen(Chosen),
    /// **Run the command a span names, on the operand the route speaks of** ([`Named`]).
    /// The span must be the name of a command — the bare line is refused by its own
    /// command for want of an operand; a line that is complete is run as printed, and an
    /// act that names operands for one is a fault of the case. The command is held to
    /// exit 0 as any line a route prints.
    RunNamed(Named),
    /// **Run a printed line the route cites for what it prints, and hold its output to
    /// what the route promised** ([`Cited`]) — the human's ruling of 2026-10-12: such a
    /// line *is held to its words*. It is run as [`Act::Run`] runs one; its exit code is
    /// not held; its output — stdout, then stderr — must keep every promise, or the case
    /// comes out [`Classification::CitedLineUnsaid`]. The promise is the case's to state,
    /// read off the route's own words, and is matched against this line's output and no
    /// other command's. A cited line with no promise is a fault of the case: a witness any
    /// output satisfies is none.
    RunCited(Cited),
    /// Move the directory the operand names out of the way: out of the repository, to the
    /// sandbox's `moved/` under the path it had ([`Place::Moved`]), with everything in it.
    /// A link to a directory is not one.
    MoveAway(Operand),
    /// Move the file the operand names out of its path — what a route asks for with *move
    /// the file out of …*: out of the repository, to the sandbox's `moved/` under the path
    /// it had ([`Place::Moved`]). A regular file, and the entry itself: a link is not one,
    /// nor a directory.
    MoveOut(Operand),
    /// Move the link the operand names out of its path — what a route asks for with *move
    /// the link out of the doc's home*: the link itself, never what it leads to, out of the
    /// repository to the sandbox's `moved/` under the path it had ([`Place::Moved`]). An
    /// entry that is no link is not moved by it: a file has [`Act::MoveOut`], a directory
    /// [`Act::MoveAway`].
    MoveLinkOut(Operand),
    /// Put the regular file itself where the link the operand names stands — what a route
    /// asks for with *for a link, a copy of the file it points at, in the link's place*:
    /// the link is removed and a regular file with the bytes of the file it led to is
    /// written at its path. The bytes are the link's target's, never the case's; a link
    /// that leads to no regular file has none to copy, and the act is not done.
    PutTheFileItself(Operand),
    /// Replace the link the operand names with a real directory: the link is removed —
    /// never what it leads to — and an empty directory is made at its path.
    ReplaceWithDirectory(Operand),
    /// Point the link the operand names at a regular file inside the repository that the
    /// repository holds committed — [`COMMITTED`].
    PointAtCommitted(Operand),
    /// Point the link the operand names at a regular file inside the repository that is
    /// new — [`NEW_FILE`], which the runner writes and no commit holds.
    PointAtNew(Operand),
    /// Point the link the operand names at a script inside the repository — [`SCRIPT`]:
    /// the one the case's state planted there, or one the runner writes where none stands.
    PointAtScript(Operand),
    /// Commit the change at the path the operand names — what a route asks for with
    /// *committing the removal where git tracks the entry*.
    Commit(Operand),
    /// Mend the regular file the operand names — what a route asks for with *fix it so
    /// jigc can merge into it*, which says what must come of the file and not its bytes.
    /// The bytes are the runner's: [`BODY`], the empty JSON object. A link is not followed
    /// and nothing is made where no file stands.
    Mend(Operand),
    /// Revert the edit at the path the operand names — what *revert the edit* asks for
    /// where the edit is in no commit: the file's bytes are put back as git's index holds
    /// them (`git checkout -- <path>`), by the runner's own git.
    Revert(Operand),
    /// Make the entry the operand names readable to its owner — what *ensure it is
    /// readable* asks for. The entry itself, never what a link leads to.
    MakeReadable(Operand),
    /// Make the entry the operand names writable to its owner — what *ensure it is
    /// writable* asks for. The entry itself: a directory's own bit and nothing in it, and
    /// never what a link leads to.
    MakeWritable(Operand),
    /// **Run the refusing command again**, where the case stands — what *re-run* and *run
    /// jigc* ask for where the route prints no command line. It takes nothing from the
    /// case; it is refused wherever the printed route holds a command line of the product
    /// ([`command_spans`]) that the case does not itself run as printed — a span that names
    /// a command is no such line — and where it stands the case can come out no better than
    /// [`Classification::HoldsOperandImplied`].
    Rerun,
    /// [`Act::Rerun`], inside the sandbox's repository — what *run jigc from inside the
    /// target git repository* asks of a reader who stands outside one. The repository is
    /// the runner's, as every object is that a route names and does not print.
    RerunInTheRepository,
    /// **Go where the route says** — the span of this index is `cd <directory>`, as
    /// printed: every command of the route that follows is started there, and a path the
    /// route prints is read from there. The directory is the route's, never the case's; a
    /// span that is no `cd` line is a fault of the case, and a directory outside the
    /// sandbox is gone to by nobody — the act is recorded as not done.
    Cd(usize),
    /// **[`Act::Rerun`] with the flag the route prints** — what *re-run … with
    /// `--carry-staged`* asks for: the refusing command once more, where the case stands,
    /// with the printed span of this index appended as its last word. The span is the
    /// flag, one word that opens with `--`; any other span is a fault of the case. Held as
    /// a re-run is: refused wherever the printed route holds a command line of the
    /// product, and no better than [`Classification::HoldsOperandImplied`].
    RerunWith(usize),
    /// **Run a command line the refusal lists** — what *run the command on that line* asks
    /// for, where a route sends its reader to a line of the refusal's own listing: the
    /// backticked span of this index among the spans the listing holds
    /// ([`listed_spans_in`]), run as [`Act::Run`] runs a printed line of the route. Bytes
    /// the product printed, by index, and never a line of the case's; a listed line that is
    /// no `jigc` or `git` command line is a fault of the case.
    RunListed(usize),
    /// **Take the path the operand names out of the index of the checkout it stands in** —
    /// what *have the contending sub-tasks touch distinct files* asks of one of them: the
    /// entry is dropped by the runner's own git (`git rm --cached`), in the directory that
    /// holds the path, and the file stays where it is. A path git holds no entry for is not
    /// unstaged, and the case is red for it.
    Unstage(Operand),
}

impl Act {
    /// What the act acts on, where it takes an operand.
    fn operand(self) -> Option<Operand> {
        match self {
            Act::Remove(operand)
            | Act::MoveAway(operand)
            | Act::MoveOut(operand)
            | Act::MoveLinkOut(operand)
            | Act::PutTheFileItself(operand)
            | Act::ReplaceWithDirectory(operand)
            | Act::PointAtCommitted(operand)
            | Act::PointAtNew(operand)
            | Act::PointAtScript(operand)
            | Act::Commit(operand)
            | Act::Mend(operand)
            | Act::Revert(operand)
            | Act::MakeReadable(operand)
            | Act::MakeWritable(operand)
            | Act::Unstage(operand) => Some(operand),
            Act::Run(_)
            | Act::RunFilled(_)
            | Act::RunChosen(_)
            | Act::RunNamed(_)
            | Act::RunCited(_)
            | Act::RunListed(_)
            | Act::Cd(_)
            | Act::Rerun
            | Act::RerunInTheRepository
            | Act::RerunWith(_) => None,
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
    /// git ignores this path, from the repository root — asked of git itself
    /// (`git check-ignore`), which is what says whether an ignore file is in force.
    GitIgnores(&'static str),
    /// Nothing stands at this place — no file, no directory, no link: what a command that
    /// removes is refused over, once its route is followed.
    Absent(Place),
    /// The regular file at this place holds these bytes somewhere in it — somebody's own
    /// lines in a file the command edits, which must still be there afterwards.
    Contains(Place, &'static str),
    /// No regular file at this place holds these bytes: what the command exists to take
    /// out of a file is out of it, or the file is gone.
    Lacks(Place, &'static str),
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
    /// **Where the case stands**: the directory the refusing command is started in, and
    /// every command the route has run — and from which a path the route prints is read.
    /// `Place::Repo("")` is the repository's root.
    pub from: Place,
    /// What the environment of those commands differs by from the mask's.
    pub env: &'static [Var],
    /// The acts that follow the printed route, in the route's order.
    pub acts: &'static [Act],
    /// What must hold once they are done — beside every [`Act::Run`] exiting 0 and every
    /// cited line ([`Act::RunCited`]) saying what the route promised.
    pub holds: &'static [Check],
    /// Files under `outside/` whose bytes must be afterwards what they were before the
    /// refusing command ran. One that was absent must still be absent.
    pub witnesses: &'static [&'static str],
    /// **Files inside the repository that the command has no business touching**, from the
    /// repository root — somebody's own file that the product did not write: witnesses like
    /// the ones outside, held the same way. A file the command is there to edit is no
    /// witness; what must survive in one is a [`Check::Contains`].
    pub untouched: &'static [&'static str],
    /// **A doubtful reading of the route's prose**, where the case rests on one: the two
    /// readings, in words. The case follows the printed words most literally and decides
    /// nothing; a marked case is listed for the human with its route.
    pub doubt: Option<&'static str>,
    /// **Which of the two the refusal is over**, where the state stands on a declared
    /// bound and, beside it, on a state the ruling says is none ([`Standing::Both`]) — one
    /// word, with what it was read from. `None` wherever the state says it by itself:
    /// a word is a reading of the cause, and the catalog holds that no case carries one
    /// that [`standing`] could have answered.
    pub over: Option<(Over, &'static str)>,
}

/// **One of the three states the human ruled a declared bound of the route catalog**
/// (`DECISIONS.md` → 2026-10-11, *Which states a route must work in*). A red case whose
/// refusal is over such a state is recorded and stays listed red; it does not block. A
/// bound changes no classification and takes no case off the expected-red list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bound {
    /// A permission bit changed by hand — an entry made read-only, unreadable, unwritable
    /// or unsearchable, wherever it lies.
    PermissionBit,
    /// A FIFO or a device node planted at a path — the node itself. A link that leads to
    /// one, and a configuration value that names one, are a link and a configuration.
    SpecialFile,
    /// A state only the developer's override `JIGC_ADAPTERS_DIR` reaches.
    AdaptersOverride,
}

impl Bound {
    /// Every bound, in the ruling's order.
    pub const ALL: [Bound; 3] = [
        Bound::PermissionBit,
        Bound::SpecialFile,
        Bound::AdaptersOverride,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Bound::PermissionBit => "a permission bit changed by hand",
            Bound::SpecialFile => "a FIFO or a device node planted",
            Bound::AdaptersOverride => "a state only `JIGC_ADAPTERS_DIR` reaches",
        }
    }
}

/// The variable of the third bound: the developer's override of the adapter profiles'
/// directory.
const ADAPTERS_OVERRIDE: &str = "JIGC_ADAPTERS_DIR";

/// Which of the two a refusal is over, where a state stands on both ([`Case::over`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Over {
    /// The bound: without it the command refuses nothing over the rest of the state.
    TheBound,
    /// The state beside it, which is no bound: the case can block.
    TheOther,
}

/// What a case's state stands on, as far as its steps and its environment say.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    /// On no bound.
    Unbounded,
    /// On a bound, and on no other state [`standing`] reads as one.
    On(Bound),
    /// On a bound **and** on a state the ruling says is none — the bound, and the other
    /// in words. Which of the two the refusal is over, the state does not say.
    Both(Bound, &'static str),
}

/// **What `case`'s state stands on — read off its steps and its environment, and never off
/// its acts**, which follow a route and may well change a permission themselves.
///
/// *A bound, by what plants it.* A state has no step for a permission: it is a shell line
/// a `git` alias runs, so the words of every `git` step are read — `chmod` is a permission
/// bit changed by hand, `mkfifo` and `mknod` a special file, as [`Step::Fifo`] is. The
/// override is the case's variable of that name, set. Two bounds at once are the first in
/// the ruling's order; no case of the catalog stands so.
///
/// *A state beside it that is none, where a step spells one by its kind or by a word*: a
/// symbolic link ([`Step::Link`], [`Step::LinkTo`], an `ln` in a shell line), an unborn
/// `HEAD` (`git update-ref -d HEAD`), a `core.hooksPath` value, and a variable other than
/// the override. **What it cannot read as a second state**: a file's bytes, a directory,
/// and what a `git add` or a commit leaves — they are in nearly every state, as the
/// ground a case stands on.
pub fn standing(case: &Case) -> Standing {
    let mut bound: Option<Bound> = None;
    let mut other: Option<&'static str> = None;
    let mut on = |seen: Bound| bound = Some(bound.map_or(seen, |before| before.min(seen)));
    let mut beside = |what: &'static str| other = other.or(Some(what));
    for step in case.state {
        match step {
            Step::Fifo(_) => on(Bound::SpecialFile),
            Step::Link(..) | Step::LinkTo(..) => beside("a symbolic link"),
            Step::Git(args) => {
                if matches!(args, ["update-ref", "-d", "HEAD"]) {
                    beside("an unborn `HEAD`");
                }
                // A shell line is one argument: its words are read, the alias's own
                // punctuation (`alias.x=!line`) and the shell's (`&&`, `;`) apart.
                let words = args.iter().flat_map(|arg| {
                    arg.split(|c: char| c.is_whitespace() || "!=&;|()".contains(c))
                });
                for word in words {
                    match word {
                        "chmod" => on(Bound::PermissionBit),
                        "mkfifo" | "mknod" => on(Bound::SpecialFile),
                        "ln" => beside("a symbolic link"),
                        "core.hooksPath" => beside("a `core.hooksPath` value"),
                        _ => {}
                    }
                }
            }
            Step::Script(..) | Step::File(..) | Step::Dir(_) | Step::Jigc(_) | Step::JigcAt(..) => {
            }
        }
    }
    for var in case.env {
        match var {
            Var::Path(ADAPTERS_OVERRIDE, _) => on(Bound::AdaptersOverride),
            Var::Unset(ADAPTERS_OVERRIDE) => {}
            Var::Path(..) | Var::Unset(_) => {
                beside("a variable of the environment that is not the override");
            }
        }
    }
    match (bound, other) {
        (None, _) => Standing::Unbounded,
        (Some(bound), None) => Standing::On(bound),
        (Some(bound), Some(other)) => Standing::Both(bound, other),
    }
}

/// **The bound `case`'s refusal is over, or none** — [`standing`], and the case's own word
/// where its state stands on both. `Err` is a fault of the case as written: a state that
/// stands on both and says nothing of which, or a word where the state says it by itself.
pub fn bound_of(case: &Case) -> Result<Option<Bound>, String> {
    match (standing(case), case.over) {
        (Standing::Unbounded, None) => Ok(None),
        (Standing::On(bound), None) => Ok(Some(bound)),
        (Standing::Both(bound, _), Some((Over::TheBound, _))) => Ok(Some(bound)),
        (Standing::Both(_, _), Some((Over::TheOther, _))) => Ok(None),
        (Standing::Both(bound, other), None) => Err(format!(
            "the state stands on a declared bound — {} — and on {other}, which is none, \
             and the case does not say which of the two its refusal is over (`Case::over`)",
            bound.as_str()
        )),
        (Standing::Unbounded | Standing::On(_), Some((_, why))) => Err(format!(
            "the case says which of two its refusal is over — {why} — and its state does \
             not stand on both: a word is only where the state cannot say it"
        )),
    }
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
    DidNotEnd,
    UnsplittableLine,
    CitedLineUnsaid,
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
            Classification::DidNotEnd => "the command did not end",
            Classification::UnsplittableLine => "followed, a printed line no shell can split",
            Classification::CitedLineUnsaid => {
                "followed, a cited line does not say what the route promised"
            }
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
    /// A command under test — the refusing command, or one the route prints — had not
    /// ended at its limit and was ended by the runner.
    pub did_not_end: bool,
    /// A line the route prints for an act to run is one no shell splits into words: it
    /// was not run.
    pub unsplit: bool,
    /// A line the route cites for what it prints was run, and its output does not keep
    /// what the route promised of it.
    pub unsaid: bool,
}

/// The one classification of what was seen. **The order is the rule**: a witness that
/// changed outranks everything a followed route can otherwise come to — a lost file is
/// the worst of them, and the refusal that usually follows it would hide it — and a
/// repeated refusal outranks any other failure. **A command that did not end is said
/// where it stopped the case**: the refusing command held says nothing of a refusal, so
/// it is read before *not provoked*; a command of the route held is read behind a changed
/// witness, which it must not hide, and ahead of everything else a followed route comes to.
/// **A printed line no shell can split is read behind those two and ahead of a repeated
/// refusal**: the step it stood for was not done, so the refusal that follows it is what
/// the line cost, and would otherwise be all the case said. **And a cited line that does
/// not say what the route promised is read next, ahead of a repeated refusal and of any
/// other failure**: the reader was sent to it for its words and did not get them, and
/// what a later line of the route then came to would otherwise be all the case said.
pub fn classify(seen: &Seen) -> Classification {
    if seen.did_not_end && !seen.provoked {
        Classification::DidNotEnd
    } else if !seen.provoked {
        Classification::NotProvoked
    } else if !seen.route_printed {
        Classification::NoRoutePrinted
    } else if seen.witness_changed {
        Classification::WitnessChanged
    } else if seen.did_not_end {
        Classification::DidNotEnd
    } else if seen.unsplit {
        Classification::UnsplittableLine
    } else if seen.unsaid {
        Classification::CitedLineUnsaid
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

/// How long a case **listed** as one whose command does not end is waited for. A known
/// hold is then seconds of a run and not the standing limit. Its price, declared: on a
/// machine so loaded that a command which *would* end takes longer than this, a fixed hold
/// still reads as held — a fix is masked there, and no green case is ever turned red by it,
/// since an unlisted case has the standing limit.
pub const EXPECTED_HOLD: Duration = Duration::from_secs(5);

/// The limit `case`'s commands under test are waited for: [`EXPECTED_HOLD`] where
/// `expected` lists it as [`Classification::DidNotEnd`], and [`LIMIT`] otherwise.
pub fn limit_of(expected: &[ExpectedRed], case: &Case) -> Duration {
    let held = |entry: &ExpectedRed| {
        entry.case == case.name && entry.classification == Classification::DidNotEnd
    };
    if expected.iter().any(held) {
        EXPECTED_HOLD
    } else {
        LIMIT
    }
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

/// **The findings a command printed**, each as its lines: the first line of a finding —
/// one [`head_code`] reads a code off — and the indented lines under it, to the first line
/// that is not indented. What a [`Promise::Route`] is asked of: a route is its own
/// finding's, as [`refusal_in`] has it.
fn findings_in(said: &str) -> Vec<Vec<&str>> {
    let mut findings: Vec<Vec<&str>> = Vec::new();
    let mut open = false;
    for line in said.lines() {
        if head_code(line).is_some() {
            findings.push(vec![line]);
            open = true;
        } else if open && line.starts_with(char::is_whitespace) {
            findings.last_mut().expect("a finding is open").push(line);
        } else {
            open = false;
        }
    }
    findings
}

/// **What the refusal of `code` lists**, in order: the values of the indented lines of its
/// finding that are not its `route:` line — between the finding's first line and the next
/// finding's, as [`refusal_in`] reads a route. A backticked span is one value, its bytes as
/// printed; every other word is one, without the colon or the comma that sets it off. A
/// line that is not indented is the message's prose and lists nothing.
pub fn listed_in(said: &str, code: &str) -> Vec<String> {
    let mut lines = said
        .lines()
        .skip_while(|line| head_code(line) != Some(code));
    if lines.next().is_none() {
        return Vec::new();
    }
    lines
        .take_while(|line| head_code(line).is_none())
        .filter(|line| line.starts_with(char::is_whitespace))
        .filter(|line| !line.trim_start().starts_with("route: "))
        .flat_map(|line| {
            line.split('`').enumerate().flat_map(|(at, piece)| {
                if at % 2 == 1 {
                    vec![piece]
                } else {
                    piece
                        .split_whitespace()
                        .map(|word| word.trim_end_matches([':', ',']))
                        .filter(|word| !word.is_empty())
                        .collect()
                }
            })
        })
        .map(str::to_string)
        .collect()
}

/// **The command lines the refusal of `code` lists**, in order: the backticked spans of the
/// indented lines of its finding that are not its `route:` line — the lines [`listed_in`]
/// reads, and of them the spans alone. What [`Act::RunListed`] names a line by.
pub fn listed_spans_in(said: &str, code: &str) -> Vec<String> {
    let mut lines = said
        .lines()
        .skip_while(|line| head_code(line) != Some(code));
    if lines.next().is_none() {
        return Vec::new();
    }
    lines
        .take_while(|line| head_code(line).is_none())
        .filter(|line| line.starts_with(char::is_whitespace))
        .filter(|line| !line.trim_start().starts_with("route: "))
        .flat_map(spans)
        .map(str::to_string)
        .collect()
}

/// The `<placeholder>` words of a printed line, as byte ranges, in order: an angle
/// bracket, bytes that are neither, and the bracket that closes it.
fn placeholders(line: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(open) = line[from..].find('<').map(|at| from + at) {
        match line[open + 1..].find(['<', '>']) {
            Some(len) if len > 0 && line[open + 1 + len..].starts_with('>') => {
                found.push((open, open + len + 2));
                from = open + len + 2;
            }
            _ => from = open + 1,
        }
    }
    found
}

/// `value` as one word of a command line: as it is where every byte of it is inert to a
/// shell, and single-quoted otherwise — a reader types a listed path as one word.
fn one_word(value: &str) -> String {
    let inert = |c: char| c.is_ascii_alphanumeric() || "._-/:@%+=,".contains(c);
    if !value.is_empty() && value.chars().all(inert) {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

/// **Whether a command, run bare, refused the line for want of an operand** — the command's
/// own word that the line is a name and not a line to run. Of the product: the argument
/// parser's refusal, exit 2, naming *required arguments* that *were not provided*. Of git:
/// its usage refusal, exit 129 — or, for `git add`, which takes a bare line and does
/// nothing, its own *Nothing specified, nothing added*. Three signals, each the command's
/// own; a line refused any other way — an unknown flag, a value that is none, a refusal of
/// the product's — is a line that was run and failed.
fn wants_an_operand(program: &str, ran: &Ran) -> bool {
    if ran.held.is_some() {
        return false;
    }
    match program {
        "jigc" => ran.code == Some(2) && ran.said.contains("required arguments were not provided"),
        "git" => {
            (ran.code == Some(129) && ran.said.trim_start().starts_with("usage: git"))
                || ran.said.contains("Nothing specified, nothing added")
        }
        _ => false,
    }
}

/// `Err` where a promise of the line span `line` names holds no piece, or a piece of no
/// bytes — a fault of the case as written: a witness any output satisfies is no witness.
fn stated(line: usize, promised: &[Promise]) -> Result<(), String> {
    for promise in promised {
        let (Promise::Line(pieces) | Promise::Route(pieces)) = promise;
        let empty = |piece: &Says| matches!(piece, Says::Text(text) if text.trim().is_empty());
        if pieces.is_empty() || pieces.iter().any(empty) {
            return Err(format!(
                "an act cites span {line} for what it prints, and a promise of it holds no \
                 piece, or a piece of no bytes: a witness any output satisfies is no witness"
            ));
        }
    }
    Ok(())
}

/// What every mark of the runner's opens with ([`mark`]).
const MARK: &str = "route-case-own-value-";

/// **The word that stands in a printed line where the `at`-th value of the reader's own
/// goes**, until a shell has split the line: letters, digits and hyphens, which a shell
/// hands on unchanged, bare and inside either kind of quotes.
fn mark(at: usize) -> String {
    format!("{MARK}{at}-here")
}

/// `text` with each mark replaced by what `put` makes of its value — the value itself, in
/// a word the shell has split off; the value as a reader would type it, in a line that is
/// shown and not run.
fn unmarked(text: &str, own: &[Own], put: impl Fn(&str) -> String) -> String {
    own.iter()
        .enumerate()
        .fold(text.to_string(), |text, (at, own)| {
            text.replace(&mark(at), &put(own.value))
        })
}

/// **The value of the case's own that a refused line's output quotes back**, where `act`
/// filled a line with one: the value, whole, between a pair of backticks or of either kind
/// of quotation mark — `--slug "Not A Slug"` is not a valid slug; could not read
/// `` `no-such.yaml` ``. It is how a command says that it refuses *this value*, and it is
/// read off the output and off nothing else: a refusal for the value that does not quote
/// it is read as any refusal is, and one that quotes it and refuses for another reason is
/// read as the case's mistake — the fault prints the output, so its reader sees which.
fn quoted_back(act: Act, said: &str) -> Option<&'static Own> {
    let Act::RunChosen(chosen) = act else {
        return None;
    };
    chosen.own.iter().find(|own| {
        ['`', '"', '\'']
            .iter()
            .any(|quote| said.contains(&format!("{quote}{}{quote}", own.value)))
    })
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

/// The indices of the spans that are **command lines of the product**: a span whose first
/// word is `jigc`. Where a printed route holds one, the command to run is the route's to
/// give ([`Act::Run`]) and a case may not run its own in its place ([`Act::Rerun`]).
pub fn command_spans(spans: &[&str]) -> Vec<usize> {
    spans
        .iter()
        .enumerate()
        .filter(|(_, span)| span.split_whitespace().next() == Some("jigc"))
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
    /// Where the case's sandbox stood. It is removed by the time the case is handed back,
    /// whatever its state shut — the arm that holds that asks this path.
    pub sandbox: PathBuf,
}

/// What one child of a case came to.
pub(super) struct Ran {
    /// Its exit code — `None` where a signal ended it, the runner's own included.
    pub(super) code: Option<i32>,
    /// The limit it was ended at, where it did not end by itself.
    pub(super) held: Option<Duration>,
    /// Its stdout, then its stderr, as text.
    pub(super) said: String,
    /// Its stdout, as the bytes it wrote.
    pub(super) out: Vec<u8>,
}

impl Ran {
    pub(super) fn success(&self) -> bool {
        self.held.is_none() && self.code == Some(0)
    }

    /// How it ended, in words.
    pub(super) fn exit(&self) -> String {
        match (self.held, self.code) {
            (Some(limit), _) => format!("did not end within {limit:?} and was ended"),
            (None, Some(code)) => format!("exit {code}"),
            (None, None) => "killed by a signal".to_string(),
        }
    }
}

/// What a refusal printed for an act to take its object from: the backticked spans of its
/// route, every value its listing holds ([`listed_in`]), and of those the backticked spans
/// alone ([`listed_spans_in`]).
#[derive(Clone, Copy)]
struct Printed<'a> {
    spans: &'a [&'a str],
    listed: &'a [String],
    commands: &'a [String],
}

/// A case's sandbox: one scratch root, removed when dropped.
pub(super) struct Sandbox {
    /// The canonical root, so a path the product prints canonical compares equal.
    root: PathBuf,
    /// How long a command under test — the refusing command, a command a route prints —
    /// may run before it is ended. The runner's own children have [`LIMIT`].
    under_test: Duration,
    /// How many children have run — each writes to files of its own.
    children: AtomicUsize,
    /// Where the route has sent its reader ([`Act::Cd`]), once it has: the directory the
    /// route's later commands are started in. Until then the case stands where it says.
    went: Mutex<Option<PathBuf>>,
    /// Whether the sandbox has a `tmp/` of its own, which every child of it then has as
    /// `TMPDIR` — one made over a prepared state has ([`Sandbox::over`]).
    own_tmp: bool,
    _scratch: ScratchDir,
}

impl Sandbox {
    /// A repository with one commit and no `jigc setup`, a `$HOME` and an `outside/`.
    fn new(label: &str, under_test: Duration) -> Sandbox {
        let scratch = ScratchDir::new(&format!("route-case-{label}"));
        let root = fs::canonicalize(scratch.path()).expect("the scratch root is there");
        let sandbox = Sandbox {
            root,
            under_test,
            children: AtomicUsize::new(0),
            went: Mutex::new(None),
            own_tmp: false,
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
        sandbox.git(&["config", "user.useConfigOnly", "true"]);
        fs::write(sandbox.repo().join(COMMITTED), BODY).expect("write a file");
        sandbox.git(&["add", COMMITTED]);
        sandbox.git(&["commit", "-q", "-m", "initial"]);
        sandbox
    }

    /// **A sandbox over a repository and a home that already exist** — a prepared state,
    /// for the byte table's frame. `bring` is handed where the sandbox's `repo/` and
    /// `home/` are to stand and puts both there; nothing is initialised and nothing is
    /// committed here, so the repository is the one that was built. Beside them stand
    /// `outside/`, `staging/` and a `tmp/`, which every child of this sandbox has as
    /// `TMPDIR`.
    pub(super) fn over(
        label: &str,
        under_test: Duration,
        bring: impl FnOnce(&Path, &Path),
    ) -> Sandbox {
        let scratch = ScratchDir::new(&format!("byte-cell-{label}"));
        let root = fs::canonicalize(scratch.path()).expect("the scratch root is there");
        let sandbox = Sandbox {
            root,
            under_test,
            children: AtomicUsize::new(0),
            went: Mutex::new(None),
            own_tmp: true,
            _scratch: scratch,
        };
        for dir in [sandbox.outside(), sandbox.staging(), sandbox.tmp()] {
            fs::create_dir_all(&dir).expect("create a sandbox directory");
        }
        bring(&sandbox.repo(), &sandbox.home());
        for dir in [sandbox.repo(), sandbox.home()] {
            assert!(
                dir.is_dir(),
                "a prepared state brings a `{}` in",
                sandbox.shown(&dir.display().to_string())
            );
        }
        sandbox
    }

    /// The sandbox's root, canonical.
    pub(super) fn root(&self) -> &Path {
        &self.root
    }

    pub(super) fn repo(&self) -> PathBuf {
        self.root.join("repo")
    }

    pub(super) fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    pub(super) fn outside(&self) -> PathBuf {
        self.root.join("outside")
    }

    pub(super) fn staging(&self) -> PathBuf {
        self.root.join("staging")
    }

    /// The temporary directory of a sandbox made over a prepared state.
    pub(super) fn tmp(&self) -> PathBuf {
        self.root.join("tmp")
    }

    fn moved(&self) -> PathBuf {
        self.root.join("moved")
    }

    pub(super) fn at(&self, place: Place) -> PathBuf {
        match place {
            Place::Repo(path) => self.repo().join(path),
            Place::Outside(path) => self.outside().join(path),
            Place::Moved(path) => self.moved().join(path),
        }
    }

    /// `text` with this sandbox's root written `<sandbox>` — a failure reads the same on
    /// every machine.
    pub(super) fn shown(&self, text: &str) -> String {
        text.replace(&self.root.display().to_string(), "<sandbox>")
    }

    /// A child in the repository, under the faithful mask: no system source of git's
    /// configuration, no global file, `$HOME` in the sandbox, and the embedded packs —
    /// **and the sandbox's identity**, as configuration git reads from the environment, so
    /// that it holds in every repository under the sandbox and none is invented in any.
    /// In a sandbox that has a `tmp/` of its own, that is the child's `TMPDIR`.
    pub(super) fn child(&self, program: &Path) -> Command {
        let mut command = Command::new(program);
        command
            .current_dir(self.repo())
            .env("HOME", self.home())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_CONFIG_COUNT", "3")
            .env("GIT_CONFIG_KEY_0", "user.name")
            .env("GIT_CONFIG_VALUE_0", "Route Case")
            .env("GIT_CONFIG_KEY_1", "user.email")
            .env("GIT_CONFIG_VALUE_1", "route-case@example.com")
            .env("GIT_CONFIG_KEY_2", "user.useConfigOnly")
            .env("GIT_CONFIG_VALUE_2", "true")
            .env_remove("JIGC_PACK_DIR")
            .stdin(Stdio::null());
        if self.own_tmp {
            command.env("TMPDIR", self.tmp());
        }
        command
    }

    /// **A command under test** — the refusing command, or one the route has run: a child
    /// under the mask, started in `at`, with the case's variables taken out of its
    /// environment or set to a place of the sandbox.
    fn tested(&self, case: &Case, program: &Path, at: &Path) -> Command {
        let mut command = self.child(program);
        command.current_dir(at);
        for var in case.env {
            match *var {
                Var::Unset(name) => command.env_remove(name),
                Var::Path(name, place) => command.env(name, self.at(place)),
            };
        }
        command
    }

    /// **Run one child and wait for it — no longer than `limit`.** Its output goes to
    /// two files of its own under `staging/`, so a child that prints more than a pipe holds
    /// is not kept waiting for a reader; one that has not ended at the limit is killed —
    /// this child, by its handle — and says so ([`Ran::exit`]). A product that holds would
    /// otherwise hold the suite: a read of a FIFO does not return.
    pub(super) fn run(&self, command: &mut Command, limit: Duration) -> Ran {
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
            if started.elapsed() >= limit {
                // Raced with its own end, the kill fails and the wait still answers.
                let _ = child.kill();
                break (child.wait().expect("wait for an ended child"), Some(limit));
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        let bytes = |path: &Path| fs::read(path).unwrap_or_default();
        let (out, err) = (bytes(&out), bytes(&err));
        Ran {
            code: status.code(),
            held,
            said: format!(
                "{}{}",
                String::from_utf8_lossy(&out),
                String::from_utf8_lossy(&err)
            ),
            out,
        }
    }

    /// A `git` child of the runner's own — the sandbox's setup, a state's step — under
    /// the standing limit, whatever limit the commands under test have.
    fn git(&self, args: &[&str]) {
        let ran = self.run(self.child(Path::new("git")).args(args), LIMIT);
        assert!(ran.success(), "git {args:?}: {}\n{}", ran.exit(), ran.said);
    }

    /// Plant a case's state, step by step. `binary` is the binary under test, for the step
    /// that runs it ([`Step::Jigc`]).
    pub(super) fn plant(&self, binary: &Path, state: &[Step]) {
        for (at, step) in state.iter().enumerate() {
            match *step {
                Step::Script(place, body) => {
                    self.place_script(&self.at(place), body, &format!("step-{at}"));
                }
                Step::Link(link, target) => {
                    let path = self.at(link);
                    fs::create_dir_all(path.parent().expect("a link has a parent"))
                        .expect("create the link's directory");
                    symlink(self.at(target), &path).expect("plant the link");
                }
                Step::LinkTo(link, target) => {
                    let path = self.at(link);
                    fs::create_dir_all(path.parent().expect("a link has a parent"))
                        .expect("create the link's directory");
                    symlink(target, &path).expect("plant the link");
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
                Step::JigcAt(place, args) => {
                    let ran = self.run(
                        self.child(binary).current_dir(self.at(place)).args(args),
                        LIMIT,
                    );
                    assert!(
                        ran.success(),
                        "the binary under test, as a state's step in `{}`, {args:?}: {}\n{}",
                        self.shown(&self.at(place).display().to_string()),
                        ran.exit(),
                        ran.said
                    );
                }
                Step::Jigc(args) => {
                    let ran = self.run(self.child(binary).args(args), LIMIT);
                    assert!(
                        ran.success(),
                        "the binary under test, as a state's step, {args:?}: {}\n{}",
                        ran.exit(),
                        ran.said
                    );
                }
            }
        }
    }

    /// Place an executable script at `path`, as the Linux rule asks of a file that may be
    /// executed: written by a child process, so no thread of this one ever holds its write
    /// handle and no sibling's fork can inherit one (`tooling-tests/placed_executable.rs`).
    /// `tag` names the staged copy.
    fn place_script(&self, path: &Path, body: &str, tag: &str) {
        fs::create_dir_all(path.parent().expect("a script has a parent"))
            .expect("create the script's directory");
        let staged = self.staging().join(tag);
        fs::write(&staged, body).expect("stage the script");
        let placed = Command::new("cp")
            .arg("--")
            .arg(&staged)
            .arg(path)
            .stdin(Stdio::null())
            .status()
            .expect("run cp");
        assert!(placed.success(), "cp to place {}", path.display());
        fs::set_permissions(path, fs::Permissions::from_mode(0o755))
            .expect("make the script executable");
    }

    /// What stands at each witness: the files under `outside/`, then the files inside the
    /// repository a case names as ones the command has no business touching.
    fn witnesses(&self, case: &Case) -> Vec<Option<Vec<u8>>> {
        let outside = case.witnesses.iter().map(|name| self.outside().join(name));
        let inside = case.untouched.iter().map(|name| self.repo().join(name));
        outside
            .chain(inside)
            .map(|path| fs::read(path).ok())
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
        case: &Case,
        act: Act,
        printed: &Printed<'_>,
        seen: &mut Seen,
    ) -> Result<String, String> {
        let Printed {
            spans,
            listed,
            commands,
        } = *printed;
        let code = case.code();
        let stand = self
            .went
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
            .unwrap_or_else(|| self.at(case.from));
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
            let path = stand.join(named);
            let what = match act {
                Act::Remove(_) => format!("remove `{named}` ({how})"),
                Act::MoveAway(_) => format!("move `{named}` out of the way ({how})"),
                Act::MoveOut(_) => format!("move the file `{named}` out of its path ({how})"),
                Act::MoveLinkOut(_) => format!("move the link `{named}` out of its path ({how})"),
                Act::PutTheFileItself(_) => {
                    format!("put the file itself in the place of the link `{named}` ({how})")
                }
                Act::ReplaceWithDirectory(_) => {
                    format!("replace the link `{named}` with a real directory ({how})")
                }
                Act::PointAtCommitted(_) => {
                    format!("point the link `{named}` at `{COMMITTED}` ({how})")
                }
                Act::PointAtNew(_) => format!("point the link `{named}` at `{NEW_FILE}` ({how})"),
                Act::PointAtScript(_) => format!("point the link `{named}` at `{SCRIPT}` ({how})"),
                Act::Commit(_) => format!("commit the change at `{named}` ({how})"),
                Act::Mend(_) => format!("mend the file `{named}` ({how})"),
                Act::Revert(_) => format!("revert the edit at `{named}` ({how})"),
                Act::MakeReadable(_) => format!("make `{named}` readable ({how})"),
                Act::MakeWritable(_) => format!("make `{named}` writable ({how})"),
                Act::Unstage(_) => {
                    format!("take `{named}` out of its checkout's index ({how})")
                }
                Act::Run(_)
                | Act::RunFilled(_)
                | Act::RunChosen(_)
                | Act::RunNamed(_)
                | Act::RunCited(_)
                | Act::RunListed(_)
                | Act::Cd(_)
                | Act::Rerun
                | Act::RerunInTheRepository
                | Act::RerunWith(_) => {
                    unreachable!("a run takes no operand")
                }
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
                Act::MoveOut(_) => self.move_out(&path),
                Act::MoveLinkOut(_) => self.a_link(&path).and_then(|()| self.to_moved(&path)),
                Act::PutTheFileItself(_) => self.put_the_file_itself(&path),
                Act::ReplaceWithDirectory(_) => self
                    .a_link(&path)
                    .and_then(|()| fs::remove_file(&path).map_err(|e| e.to_string()))
                    .and_then(|()| fs::create_dir(&path).map_err(|e| e.to_string()))
                    .map(|()| "the link is gone and a directory stands there".to_string()),
                Act::PointAtCommitted(_) => self.point(&path, COMMITTED),
                Act::PointAtNew(_) => self
                    .a_link(&path)
                    .and_then(|()| {
                        fs::write(self.repo().join(NEW_FILE), BODY).map_err(|e| e.to_string())
                    })
                    .and_then(|()| self.point(&path, NEW_FILE)),
                Act::PointAtScript(_) => self.a_link(&path).and_then(|()| {
                    let script = self.repo().join(SCRIPT);
                    if fs::symlink_metadata(&script).is_err() {
                        self.place_script(&script, "#!/bin/sh\nexit 0\n", "the-runners-script");
                    }
                    self.point(&path, SCRIPT)
                }),
                Act::Commit(_) => self.commit(named),
                Act::Mend(_) => {
                    if fs::symlink_metadata(&path).is_ok_and(|entry| entry.is_file()) {
                        fs::write(&path, BODY)
                            .map(|()| "its bytes are the runner's now".to_string())
                            .map_err(|e| e.to_string())
                    } else {
                        Err("not done — no regular file stands there".to_string())
                    }
                }
                Act::Revert(_) => {
                    let ran = self.run(
                        self.child(Path::new("git")).args(["checkout", "--", named]),
                        LIMIT,
                    );
                    if ran.success() {
                        Ok("its bytes are the index's again".to_string())
                    } else {
                        Err(format!(
                            "not done — git has no entry to put back there: {}\n{}",
                            ran.exit(),
                            ran.said.trim_end()
                        ))
                    }
                }
                Act::MakeReadable(_) => self.permit(&path, 0o400, "its owner may read it"),
                Act::MakeWritable(_) => self.permit(&path, 0o200, "its owner may write it"),
                Act::Unstage(_) => self.unstage(&path),
                Act::Run(_)
                | Act::RunFilled(_)
                | Act::RunChosen(_)
                | Act::RunNamed(_)
                | Act::RunCited(_)
                | Act::RunListed(_)
                | Act::Cd(_)
                | Act::Rerun
                | Act::RerunInTheRepository
                | Act::RerunWith(_) => {
                    unreachable!("a run takes no operand")
                }
            };
            return Ok(match done {
                Ok(done) => format!("{what}: {done}"),
                Err(why) => {
                    seen.failed = true;
                    format!("{what}: {why}")
                }
            });
        }
        let (what, ran) = match act {
            Act::Run(_)
            | Act::RunFilled(_)
            | Act::RunChosen(_)
            | Act::RunCited(_)
            | Act::RunListed(_) => {
                let (at, line, how) = match act {
                    Act::RunFilled(filled) => {
                        let printed = span(filled.line)?;
                        let holes = placeholders(printed);
                        if holes.is_empty() {
                            return Err(format!(
                                "an act fills span {}, `{printed}`, which holds no \
                                 placeholder: a line that is a command as it stands is run \
                                 as printed",
                                filled.line
                            ));
                        }
                        if holes.len() != filled.with.len() {
                            return Err(format!(
                                "an act fills span {}, `{printed}`, which holds {} \
                                 placeholders, with {} values: every placeholder of a line \
                                 is filled, in the line's order",
                                filled.line,
                                holes.len(),
                                filled.with.len()
                            ));
                        }
                        let (mut line, mut how, mut end) = (String::new(), Vec::new(), 0);
                        for ((from, to), at) in holes.iter().zip(filled.with) {
                            let value = listed.get(*at).ok_or_else(|| {
                                format!(
                                    "an act fills a placeholder with listed value {at}, and \
                                     the refusal lists {}",
                                    listed.len()
                                )
                            })?;
                            if !self.stays(&stand, value) {
                                seen.failed = true;
                                return Ok(format!(
                                    "run `{printed}` (span {}) with `{}` filled by `{value}` \
                                     (listed value {at}): not done — it names nothing inside \
                                     the case's sandbox",
                                    filled.line,
                                    &printed[*from..*to]
                                ));
                            }
                            line.push_str(&printed[end..*from]);
                            line.push_str(&one_word(value));
                            how.push(format!(
                                "`{}` filled by listed value {at}",
                                &printed[*from..*to]
                            ));
                            end = *to;
                        }
                        line.push_str(&printed[end..]);
                        (filled.line, line, format!(", {}", how.join(", ")))
                    }
                    Act::Run(at) => (at, span(at)?.to_string(), String::new()),
                    Act::RunChosen(chosen) => {
                        let printed = span(chosen.line)?;
                        stated(chosen.line, chosen.promised)?;
                        let (line, how) = self.marked(chosen, printed)?;
                        if let Some(own) =
                            chosen.own.iter().find(|own| !self.stays(&stand, own.value))
                        {
                            seen.failed = true;
                            return Ok(format!(
                                "run `{printed}` (span {}) with `{}` filled by the case's \
                                 `{}`: not done — it names nothing inside the case's sandbox",
                                chosen.line, own.placeholder, own.value
                            ));
                        }
                        (chosen.line, line, how)
                    }
                    Act::RunCited(cited) => {
                        if cited.promised.is_empty() {
                            return Err(format!(
                                "an act cites span {} for what it prints and states no \
                                 promise: a witness any output satisfies is no witness",
                                cited.line
                            ));
                        }
                        stated(cited.line, cited.promised)?;
                        (cited.line, span(cited.line)?.to_string(), String::new())
                    }
                    Act::RunListed(at) => {
                        let listed = commands.get(at).ok_or_else(|| {
                            format!(
                                "an act runs listed span {at}, and the refusal lists {} \
                                 backticked span(s)",
                                commands.len()
                            )
                        })?;
                        (at, listed.clone(), String::new())
                    }
                    _ => unreachable!("the five acts that run a printed line"),
                };
                // A span of the route, or — for a command the refusal lists — of its listing.
                let of = if matches!(act, Act::RunListed(_)) {
                    "listed span"
                } else {
                    "span"
                };
                // A value of the reader's own stands in the line as a mark until the shell
                // has split it ([`Self::marked`]).
                let own: &[Own] = match act {
                    Act::RunChosen(chosen) => chosen.own,
                    _ => &[],
                };
                let words = match super::shell_words_or(&line, &stand, &self.home()) {
                    Ok(words) => words,
                    Err(said) => {
                        seen.unsplit = true;
                        // Not run: the line as its reader would have typed it.
                        let line = unmarked(&line, own, one_word);
                        return Ok(format!(
                            "run `{line}` ({of} {at}{how}): not run — no shell splits it \
                             into words\n{}",
                            said.trim_end()
                        ));
                    }
                };
                // Each value takes its mark's place in the word the shell made of it: it
                // never met the shell, so it is one word whatever it holds. And the line
                // of the record is the words that are run, each as a shell would take it.
                let (words, line) = if own.is_empty() {
                    (words, line)
                } else {
                    let words: Vec<String> = words
                        .iter()
                        .map(|word| unmarked(word, own, str::to_string))
                        .collect();
                    let line = words
                        .iter()
                        .map(|word| one_word(word))
                        .collect::<Vec<_>>()
                        .join(" ");
                    (words, line)
                };
                let line = line.as_str();
                let program = match words.first().map(String::as_str) {
                    Some("jigc") => binary,
                    Some("git") => Path::new("git"),
                    _ => {
                        return Err(format!(
                            "an act runs {of} {at}, `{line}`, which is no `jigc` or `git` \
                             command line"
                        ));
                    }
                };
                let ran = self.run(
                    self.tested(case, program, &stand).args(&words[1..]),
                    self.under_test,
                );
                if let Act::RunCited(cited) = act {
                    return Ok(self.held_to_its_words(
                        format!("run the cited line `{line}` (span {})", cited.line),
                        cited.promised,
                        &ran,
                        seen,
                    ));
                }
                // A span that names a command is no line to run as printed.
                if matches!(act, Act::Run(_)) && wants_an_operand(&words[0], &ran) {
                    return Err(format!(
                        "an act runs span {at}, `{line}`, as printed — and the span is the \
                         name of a command: run bare, its command refuses it for want of an \
                         operand. A name is followed with the operand the route speaks of \
                         (`Act::RunNamed`), or is the refusing command's, to be run again \
                         (`Act::Rerun`); as printed it is run by nobody\n{}",
                        self.shown(&ran.said)
                    ));
                }
                // A filled line the route says something of is held to those words.
                if let Act::RunChosen(chosen) = act
                    && !chosen.promised.is_empty()
                {
                    return Ok(self.held_to_its_words(
                        format!("run `{line}` ({of} {at}{how}), held to what the route says of it"),
                        chosen.promised,
                        &ran,
                        seen,
                    ));
                }
                (format!("run `{line}` ({of} {at}{how})"), ran)
            }
            Act::RunNamed(named) => {
                let name = span(named.name)?;
                let fault = |what: &str| {
                    Err(format!(
                        "an act runs the command span {} names, `{name}`, and {what}",
                        named.name
                    ))
                };
                if named.on.is_empty() {
                    return fault(
                        "names no operand for it: a name the case names no operand for is \
                         no act",
                    );
                }
                if named.means.trim().is_empty() {
                    return fault("does not say which operand the route means");
                }
                let word = |word: &Word| match *word {
                    Word::Span(at) => {
                        span(at).map(|bytes| (bytes.to_string(), format!("span {at}")))
                    }
                    Word::Listed(at) => listed
                        .get(at)
                        .map(|value| (value.clone(), format!("listed value {at}")))
                        .ok_or_else(|| {
                            format!(
                                "an act hands a named command listed value {at}, and the \
                                 refusal lists {}",
                                listed.len()
                            )
                        }),
                    Word::Named("") => {
                        Err("an act hands a named command a word of no bytes".to_string())
                    }
                    Word::Named(bytes) => Ok((bytes.to_string(), "the case's".to_string())),
                };
                let on: Vec<(String, String)> =
                    named.on.iter().map(&word).collect::<Result<_, _>>()?;
                let within = named.within.as_ref().map(word).transpose()?;
                let how = on
                    .iter()
                    .map(|(bytes, from)| format!("{from} `{bytes}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let what = |line: &str| {
                    format!(
                        "run the named command `{line}` (span {}, `{name}`, on {how}{} — {})",
                        named.name,
                        within
                            .as_ref()
                            .map(|(bytes, _)| format!(", inside `{bytes}`"))
                            .unwrap_or_default(),
                        named.means
                    )
                };
                let shape = format!("{name} …");
                // Where the route says the command belongs: a directory of the sandbox.
                let there = match &within {
                    None => stand.clone(),
                    Some((bytes, _)) => {
                        let there = stand.join(bytes);
                        let inside = fs::canonicalize(&there)
                            .is_ok_and(|there| there.is_dir() && there.starts_with(&self.root));
                        if !inside {
                            seen.failed = true;
                            return Ok(format!(
                                "{}: not done — `{bytes}` names no directory inside the \
                                 case's sandbox",
                                what(&shape)
                            ));
                        }
                        there
                    }
                };
                if let Some((bytes, _)) = on.iter().find(|(bytes, _)| !self.stays(&there, bytes)) {
                    seen.failed = true;
                    return Ok(format!(
                        "{}: not done — `{bytes}` names nothing inside the case's sandbox",
                        what(&shape)
                    ));
                }
                let mut words = match super::shell_words_or(name, &there, &self.home()) {
                    Ok(words) => words,
                    Err(said) => {
                        seen.unsplit = true;
                        return Ok(format!(
                            "{}: not run — no shell splits the name into words\n{}",
                            what(&shape),
                            said.trim_end()
                        ));
                    }
                };
                // A name by its shape: its own command refuses the bare line for want of
                // an operand. A line that is complete is the route's to run as printed.
                if !self.names_a_command(binary, case, &words, &there) {
                    return fault(
                        "names operands for it — but the span is a command line as it \
                         stands: run bare, its command does not refuse it for want of an \
                         operand. A line that is complete is run as printed",
                    );
                }
                let program = if words[0] == "jigc" {
                    binary
                } else {
                    Path::new("git")
                };
                words.extend(on.iter().map(|(bytes, _)| bytes.clone()));
                let line = words
                    .iter()
                    .map(|word| one_word(word))
                    .collect::<Vec<_>>()
                    .join(" ");
                (
                    what(&line),
                    self.run(
                        self.tested(case, program, &there).args(&words[1..]),
                        self.under_test,
                    ),
                )
            }
            Act::Cd(at) => {
                let line = span(at)?;
                // Split from the sandbox's root, which stands whatever the route's earlier
                // commands removed: a command that lands a milestone takes the worktree its
                // reader stood in with it, and `cd` is how that reader leaves.
                let words =
                    super::shell_words_or(line, &self.root, &self.home()).unwrap_or_default();
                let [cd, to] = words.as_slice() else {
                    return Err(format!(
                        "an act goes where span {at} says, and `{line}` is no `cd` line: \
                         `cd` and one directory"
                    ));
                };
                if cd != "cd" {
                    return Err(format!(
                        "an act goes where span {at} says, and `{line}` is no `cd` line: \
                         `cd` and one directory"
                    ));
                }
                let there = stand.join(to);
                // The directory itself is asked, resolved: a `cd` follows a link.
                let inside = fs::canonicalize(&there)
                    .is_ok_and(|there| there.is_dir() && there.starts_with(&self.root));
                if !inside {
                    seen.failed = true;
                    return Ok(format!(
                        "go where `{line}` says (span {at}): not done — it names no \
                         directory inside the case's sandbox"
                    ));
                }
                *self.went.lock().unwrap_or_else(PoisonError::into_inner) = Some(there);
                return Ok(format!("go where `{line}` says (span {at}): there"));
            }
            Act::RerunWith(at) => {
                let flag = span(at)?;
                if !flag.starts_with("--") || flag.split_whitespace().count() != 1 {
                    return Err(format!(
                        "an act runs the refusing command again with span {at}, `{flag}`, \
                         which is no flag: one word that opens with `--`"
                    ));
                }
                (
                    format!(
                        "run the refusing command again with the printed `{flag}` (span \
                         {at}), `jigc {} {flag}`, where the case stands (no command line is \
                         printed)",
                        case.refusing.join(" ")
                    ),
                    self.run(
                        self.tested(case, binary, &stand)
                            .args(case.refusing)
                            .arg(flag),
                        self.under_test,
                    ),
                )
            }
            Act::Rerun | Act::RerunInTheRepository => {
                let (at, wording) = if matches!(act, Act::Rerun) {
                    (stand, "where the case stands")
                } else {
                    (self.repo(), "inside the sandbox's repository")
                };
                (
                    format!(
                        "run the refusing command again, `jigc {}`, {wording} (no command \
                         line is printed)",
                        case.refusing.join(" ")
                    ),
                    self.run(
                        self.tested(case, binary, &at).args(case.refusing),
                        self.under_test,
                    ),
                )
            }
            _ => unreachable!("every other act takes an operand"),
        };
        if ran.success() {
            return Ok(format!("{what}: exit 0"));
        }
        Ok(if ran.held.is_some() {
            seen.did_not_end = true;
            format!("{what}: {}\n{}", ran.exit(), ran.said)
        } else if refusal_in(&ran.said, code).is_some() {
            seen.same_refusal = true;
            format!("{what}: {}, `{code}` again", ran.exit())
        } else if let Some(own) = quoted_back(act, &ran.said) {
            return Err(self.shown(&format!(
                "{what}: {} — refused for the value the case names for `{}`: the line's \
                 output quotes `{}` back. A value the line's own grammar refuses is the \
                 case's mistake, and what the line came to says nothing of the route\n{}",
                ran.exit(),
                own.placeholder,
                own.value,
                ran.said
            )));
        } else {
            seen.failed = true;
            format!("{what}: {}\n{}", ran.exit(), ran.said)
        })
    }

    /// **Whether `words` is the name of a command and no line to run**: run bare, where the
    /// act stands, its own command refuses it for want of an operand
    /// ([`wants_an_operand`]). It is asked of the command and of no table: the frame holds
    /// no copy of an argument grammar. A line that is complete is *run* by the asking — in
    /// a sandbox, and only where a case treats it as a name, which is then a fault of the
    /// case whatever the line did.
    fn names_a_command(&self, binary: &Path, case: &Case, words: &[String], at: &Path) -> bool {
        let program = match words.first().map(String::as_str) {
            Some("jigc") => binary,
            Some("git") => Path::new("git"),
            _ => return false,
        };
        let ran = self.run(
            self.tested(case, program, at).args(&words[1..]),
            self.under_test,
        );
        wants_an_operand(&words[0], &ran)
    }

    /// [`Self::names_a_command`], for a printed span: one no shell can split is no name.
    fn names(&self, binary: &Path, case: &Case, span: &str, at: &Path) -> bool {
        super::shell_words_or(span, at, &self.home())
            .is_ok_and(|words| self.names_a_command(binary, case, &words, at))
    }

    /// **A printed line with a mark where each placeholder the case names stands**, and
    /// what the record says of the values — `Err` is a fault of the case as written. A
    /// mark is a word no shell changes; the value takes its place once the shell has split
    /// the line ([`unmarked`]), so a value is one word of the command whether its
    /// placeholder is printed bare or quoted, and whatever the value holds. A placeholder
    /// no [`Own`] names is left as printed.
    fn marked(&self, chosen: Chosen, printed: &str) -> Result<(String, String), String> {
        let fault = |what: String| {
            Err(format!(
                "an act fills span {}, `{printed}`, with values of the reader's own, and \
                 {what}",
                chosen.line
            ))
        };
        if chosen.own.is_empty() {
            return fault(
                "names no value: a line that is a command as it stands is run as printed"
                    .to_string(),
            );
        }
        if printed.contains(MARK) {
            return fault(format!(
                "the printed line holds the runner's own mark, `{MARK}`"
            ));
        }
        let holes = placeholders(printed);
        for (at, own) in chosen.own.iter().enumerate() {
            if chosen.own[..at]
                .iter()
                .any(|before| before.placeholder == own.placeholder)
            {
                return fault(format!("names `{}` twice", own.placeholder));
            }
            if !holes
                .iter()
                .any(|(from, to)| &printed[*from..*to] == own.placeholder)
            {
                return fault(format!(
                    "names `{}`, which span {} does not hold",
                    own.placeholder, chosen.line
                ));
            }
            if own.value.is_empty() || own.value.contains(MARK) {
                return fault(format!(
                    "names for `{}` a value of no bytes, or one that holds the runner's own \
                     mark: a reader chooses a value",
                    own.placeholder
                ));
            }
            if own.why.trim().is_empty() {
                return fault(format!(
                    "does not say why a reader would choose `{}` for `{}`",
                    own.value, own.placeholder
                ));
            }
        }
        let (mut line, mut end) = (String::new(), 0);
        for (from, to) in holes {
            let Some(at) = chosen
                .own
                .iter()
                .position(|own| own.placeholder == &printed[from..to])
            else {
                continue;
            };
            line.push_str(&printed[end..from]);
            line.push_str(&mark(at));
            end = to;
        }
        line.push_str(&printed[end..]);
        let how: Vec<String> = chosen
            .own
            .iter()
            .map(|own| {
                format!(
                    "`{}` filled by the case's `{}` — {}",
                    own.placeholder, own.value, own.why
                )
            })
            .collect();
        Ok((line, format!(", {}", how.join(", "))))
    }

    /// **What a cited line came to**: it is held to its words and not to its exit. `ran`
    /// is the cited line's own child — its output, stdout then stderr, is what every
    /// promise is asked of, and no other command's. A line that did not end said nothing
    /// that can be read as its answer, and is a command that did not end. What the line
    /// printed is kept in the record either way: it is what the case is about.
    fn held_to_its_words(
        &self,
        what: String,
        promised: &[Promise],
        ran: &Ran,
        seen: &mut Seen,
    ) -> String {
        if ran.held.is_some() {
            seen.did_not_end = true;
            return format!("{what}: {}\n{}", ran.exit(), ran.said);
        }
        let unkept: Vec<String> = promised
            .iter()
            .filter_map(|promise| self.kept(&ran.said, promise).err())
            .collect();
        if unkept.is_empty() {
            format!(
                "{what}: {} — not held to exit 0; its output says what the route promised\n{}",
                ran.exit(),
                ran.said
            )
        } else {
            seen.unsaid = true;
            format!(
                "{what}: {} — its output does not say what the route promised: {}\n{}",
                ran.exit(),
                unkept.join("; "),
                ran.said
            )
        }
    }

    /// Whether `said` keeps `promise` — `Err` is what it lacks, in words. A piece is bytes
    /// of the case's or the absolute path of a place of this sandbox, and is held within
    /// one line.
    fn kept(&self, said: &str, promise: &Promise) -> Result<(), String> {
        let piece = |says: &Says| match *says {
            Says::Text(text) => text.to_string(),
            // As a command prints a path: its components, and nothing behind the last —
            // the repository's root is `Place::Repo("")`, which joins to a path that ends
            // in a separator.
            Says::Path(place) => self
                .at(place)
                .components()
                .collect::<PathBuf>()
                .display()
                .to_string(),
        };
        let named = |pieces: &[String]| {
            pieces
                .iter()
                .map(|piece| format!("`{piece}`"))
                .collect::<Vec<_>>()
                .join(" and ")
        };
        match promise {
            Promise::Line(pieces) => {
                let pieces: Vec<String> = pieces.iter().map(piece).collect();
                if said
                    .lines()
                    .any(|line| pieces.iter().all(|piece| line.contains(piece.as_str())))
                {
                    Ok(())
                } else {
                    Err(format!("no line of it holds {}", named(&pieces)))
                }
            }
            Promise::Route(pieces) => {
                let pieces: Vec<String> = pieces.iter().map(piece).collect();
                if findings_in(said).iter().any(|finding| {
                    finding
                        .iter()
                        .any(|line| line.trim_start().starts_with("route: "))
                        && pieces
                            .iter()
                            .all(|piece| finding.iter().any(|line| line.contains(piece.as_str())))
                }) {
                    Ok(())
                } else {
                    Err(format!(
                        "no finding of it carries a route of its own and holds {}",
                        named(&pieces)
                    ))
                }
            }
        }
    }

    /// Whether `value`, filled into a printed line, is no path out of this sandbox — read
    /// from where the case stands. Where the directory that would hold it is there, it is
    /// asked as every act's path is ([`Self::contains`]); where it is not — an id, an
    /// address, a path whose directory is gone — the value may not be absolute and may not
    /// climb.
    fn stays(&self, stand: &Path, value: &str) -> bool {
        let path = stand.join(value);
        match path.parent().map(fs::canonicalize) {
            Some(Ok(parent)) => parent.starts_with(&self.root),
            _ => !Path::new(value).is_absolute() && value.split('/').all(|part| part != ".."),
        }
    }

    /// Take the path out of the index of the checkout it stands in: `git rm --cached`, run
    /// in the directory that holds the path — so a linked worktree's entry is dropped from
    /// that worktree's index — and the file itself is not touched.
    fn unstage(&self, path: &Path) -> Result<String, String> {
        let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else {
            return Err("not done — the path has no name to unstage".to_string());
        };
        let ran = self.run(
            self.child(Path::new("git"))
                .current_dir(dir)
                .args(["rm", "-q", "--cached", "--"])
                .arg(name),
            LIMIT,
        );
        if ran.success() {
            Ok("its index entry is gone, and the file stands".to_string())
        } else {
            Err(format!(
                "not done — git holds no entry there: {}\n{}",
                ran.exit(),
                ran.said.trim_end()
            ))
        }
    }

    /// Give the owner of the entry at `path` the permission `bit` — of the entry itself: a
    /// link is refused, since a permission set through one is set on what it leads to.
    fn permit(&self, path: &Path, bit: u32, done: &str) -> Result<String, String> {
        match fs::symlink_metadata(path) {
            Ok(entry) if entry.file_type().is_symlink() => {
                Err("not done — a link stands there".to_string())
            }
            Ok(entry) => {
                let mut permissions = entry.permissions();
                permissions.set_mode(permissions.mode() | bit);
                fs::set_permissions(path, permissions)
                    .map(|()| done.to_string())
                    .map_err(|e| e.to_string())
            }
            Err(e) => Err(e.to_string()),
        }
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
        self.to_moved(path)
    }

    /// Move the regular file at `path` to `moved/`, where [`Self::move_away`] moves a
    /// directory: asked of the entry itself, so a link is no file to move.
    fn move_out(&self, path: &Path) -> Result<String, String> {
        if !fs::symlink_metadata(path).is_ok_and(|entry| entry.is_file()) {
            return Err("not done — no regular file stands there".to_string());
        }
        self.to_moved(path)
    }

    /// Put the regular file itself where the link at `path` stands: the bytes of the file
    /// the link leads to, written to a regular file at the link's own path once the link
    /// is gone. What the link led to is read and never written.
    fn put_the_file_itself(&self, path: &Path) -> Result<String, String> {
        self.a_link(path)?;
        if !fs::metadata(path).is_ok_and(|target| target.is_file()) {
            return Err("not done — the link leads to no regular file to copy".to_string());
        }
        let bytes = fs::read(path).map_err(|e| e.to_string())?;
        fs::remove_file(path).map_err(|e| e.to_string())?;
        fs::write(path, bytes).map_err(|e| e.to_string())?;
        Ok("the link is gone and a copy of the file it led to stands there".to_string())
    }

    /// Move the entry at `path` to `moved/`, under the path it had from the repository
    /// root — or under its own name, where it stood elsewhere in the sandbox.
    fn to_moved(&self, path: &Path) -> Result<String, String> {
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

    /// Commit the change at the path `named`: the index's entries there are dropped, what
    /// stands there now is added, and the index is committed.
    ///
    /// **Three plain steps, and no pathspec on the commit.** `git add --all -- <path>` and
    /// `git commit -- <path>` both read the path afresh against the entry the index had, and
    /// where a committed link was replaced by a directory git takes the directory for a
    /// repository of its own — *does not have a commit checked out* — or does not, by the
    /// timestamps of the moment (driven: the same steps failed and passed). Dropping the
    /// entry first leaves nothing to compare with. The price: the commit carries whatever
    /// else the index held, which in a case's sandbox is nothing but the sandbox's own first
    /// file where a state made `HEAD` unborn.
    fn commit(&self, named: &str) -> Result<String, String> {
        let mut steps: Vec<Vec<&str>> = vec![vec![
            "rm",
            "-q",
            "-r",
            "-f",
            "--cached",
            "--ignore-unmatch",
            "--",
            named,
        ]];
        if fs::symlink_metadata(self.repo().join(named)).is_ok() {
            steps.push(vec!["add", "--", named]);
        }
        steps.push(vec!["commit", "-q", "-m", "follow the route"]);
        for args in &steps {
            let ran = self.run(self.child(Path::new("git")).args(args), LIMIT);
            if !ran.success() {
                return Err(format!("git {args:?}: {}\n{}", ran.exit(), ran.said));
            }
        }
        Ok("committed".to_string())
    }

    /// The bytes of the regular file at `path` — `None` where no regular file stands there,
    /// asked without following a link.
    fn regular(&self, path: &Path) -> Option<Result<Vec<u8>, String>> {
        fs::symlink_metadata(path)
            .is_ok_and(|entry| entry.is_file())
            .then(|| fs::read(path).map_err(|e| e.to_string()))
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
            Check::GitIgnores(path) => {
                let ran = self.run(
                    self.child(Path::new("git"))
                        .args(["check-ignore", "-q", "--", path]),
                    LIMIT,
                );
                if ran.success() {
                    Ok(())
                } else {
                    Err(format!("git does not ignore `{path}` ({})", ran.exit()))
                }
            }
            Check::Absent(place) => {
                let path = self.at(place);
                match fs::symlink_metadata(&path) {
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
                    Err(e) => Err(format!("`{}` cannot be asked about: {e}", path.display())),
                    Ok(_) => Err(format!("`{}` still stands", path.display())),
                }
            }
            Check::Contains(place, bytes) => {
                let path = self.at(place);
                match self.regular(&path) {
                    Some(Ok(held)) if contains(&held, bytes.as_bytes()) => Ok(()),
                    Some(Ok(_)) => Err(format!(
                        "`{}` does not hold `{}`",
                        path.display(),
                        bytes.escape_debug()
                    )),
                    Some(Err(why)) => Err(format!("`{}` cannot be read: {why}", path.display())),
                    None => Err(format!("`{}` is no regular file", path.display())),
                }
            }
            Check::Lacks(place, bytes) => {
                let path = self.at(place);
                match self.regular(&path) {
                    Some(Ok(held)) if contains(&held, bytes.as_bytes()) => Err(format!(
                        "`{}` still holds `{}`",
                        path.display(),
                        bytes.escape_debug()
                    )),
                    Some(Ok(_)) | None => Ok(()),
                    Some(Err(why)) => Err(format!("`{}` cannot be read: {why}", path.display())),
                }
            }
        }
    }
}

/// Whether `held` holds `bytes` somewhere in it. Nothing is held by every file.
fn contains(held: &[u8], bytes: &[u8]) -> bool {
    bytes.is_empty() || held.windows(bytes.len()).any(|window| window == bytes)
}

impl Drop for Sandbox {
    /// **Give every directory of the sandbox back to its owner before the scratch root is
    /// removed.** A state may shut a directory — nothing may be made in it, or taken from
    /// it — and a route need not open it again; the root's removal cannot empty such a
    /// directory, and the root would be left behind on every run. This runs before the
    /// root's own removal, which is a field's and comes after.
    fn drop(&mut self) {
        reopen(&self.root);
    }
}

/// Make `dir`, and every directory under it, its owner's to read, write and search — the
/// entry itself, asked without following a link, so nothing a link leads to is touched.
fn reopen(dir: &Path) {
    let Ok(entry) = fs::symlink_metadata(dir) else {
        return;
    };
    if !entry.is_dir() {
        return;
    }
    let mut permissions = entry.permissions();
    permissions.set_mode(permissions.mode() | 0o700);
    let _ = fs::set_permissions(dir, permissions);
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            reopen(&entry.path());
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

/// **How many cases are followed at once** ([`pooled`]): as many as this process may run
/// threads at once — the machine's cores, or the share of them a container was given — and
/// two where the machine does not say.
///
/// *Why a bound at all.* A case is a chain of child processes, one at a time: a sandbox's
/// `git` setup, the refusing command, the route's commands. Started all at once, the cases
/// take a share of the machine in proportion to their number — 138 chains on a ten-core
/// host that another gate was loading finished sooner than any bounded pool did, **because**
/// they crowded everything else out; and in a container of eight CPUs the same 138 made
/// every `jigc setup` slow toward the two-minute [`LIMIT`], at which a green case turns red
/// for no reason of the product's, while an arm beside them that starts no process ran past
/// a minute. The number of cases only grows.
///
/// *Why the cores.* A chain computes about as much as one thread does, so this many chains
/// are one machine's worth of work and no more, whatever the catalog grows to — the bound a
/// test harness puts on its own tests, which already run beside this arm. Narrower is not
/// kinder, only slower: the gate ends a test that runs eight minutes, and under a loaded
/// machine a pool of two or four comes within reach of that once the catalog is two or
/// three times what it is (`DECISIONS.md` → 2026-10-11, *the cases are followed through a
/// bounded pool*, has the seconds).
pub fn width() -> usize {
    std::thread::available_parallelism().map_or(2, usize::from)
}

/// **Run `job` over every item, no more than `width` of them at once**, and hand back what
/// each came to, in the items' order — whichever ended first. `width` threads take the
/// items in order, each the next one that nobody has taken, so what runs at once is never
/// more than the threads there are.
pub fn pooled<T: Sync, R: Send>(width: usize, items: &[T], job: impl Fn(&T) -> R + Sync) -> Vec<R> {
    assert!(width > 0, "a pool of no width runs nothing");
    let next = AtomicUsize::new(0);
    let mut came_to: Vec<(usize, R)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..width.min(items.len()))
            .map(|_| {
                scope.spawn(|| {
                    let mut mine = Vec::new();
                    loop {
                        let at = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(at) else {
                            break mine;
                        };
                        mine.push((at, job(item)));
                    }
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("a case's runner panicked"))
            .collect()
    });
    came_to.sort_by_key(|(at, _)| *at);
    came_to.into_iter().map(|(_, what)| what).collect()
}

/// **A gate no more than `width` jobs are inside at once**, whoever sends them through and
/// from whichever thread. [`pooled`] bounds one caller's jobs by its threads; a gate bounds
/// every caller's together.
///
/// **A gate says how many it has turned back** ([`Gate::waiting`]). Seen from outside, a
/// job that asked and was not let in is the same as one that has not asked yet; only the
/// gate knows which, so it counts them, under the lock it decides under. The arm that
/// holds the gate counts through that, and the runner never reads it.
pub struct Gate {
    width: usize,
    held: Mutex<Held>,
    left: Condvar,
}

/// What a gate counts, under its one lock.
struct Held {
    /// The jobs inside.
    inside: usize,
    /// The jobs that asked, found the gate full and have not been let in.
    waiting: usize,
}

impl Gate {
    pub const fn new(width: usize) -> Gate {
        Gate {
            width,
            held: Mutex::new(Held {
                inside: 0,
                waiting: 0,
            }),
            left: Condvar::new(),
        }
    }

    /// Run `job` once fewer than the gate's width are inside, and hand back what it came
    /// to. The place is given back however the job ends — a job that panics included —
    /// and a job that waits is let in when one is.
    pub fn admit<R>(&self, job: impl FnOnce() -> R) -> R {
        /// One place inside the gate, given back when dropped.
        struct Place<'a>(&'a Gate);
        impl Drop for Place<'_> {
            fn drop(&mut self) {
                self.0.held().inside -= 1;
                self.0.left.notify_one();
            }
        }
        assert!(self.width > 0, "a gate of no width admits nothing");
        let mut held = self.held();
        if held.inside >= self.width {
            held.waiting += 1;
            held = self
                .left
                .wait_while(held, |held| held.inside >= self.width)
                .unwrap_or_else(PoisonError::into_inner);
            held.waiting -= 1;
        }
        held.inside += 1;
        drop(held);
        let _place = Place(self);
        job()
    }

    /// **How many jobs stand at the gate**: each asked, found it full, and has not been
    /// let in. Counted in the hold of the lock that found the gate full and taken back in
    /// the hold that lets the job in, so a job is never both waiting and inside, and one
    /// that waits stays counted until a place was given back.
    pub fn waiting(&self) -> usize {
        self.held().waiting
    }

    fn held(&self) -> MutexGuard<'_, Held> {
        self.held.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// **This process's one gate for the cases it follows**, [`width`] wide: every
/// [`follow_within`] passes it, so the chains of child processes this process runs at once
/// are one machine's worth whatever the number of arms that follow cases in it.
fn chains() -> &'static Gate {
    static CHAINS: OnceLock<Gate> = OnceLock::new();
    CHAINS.get_or_init(|| Gate::new(width()))
}

/// **Run `job` inside this process's one gate** — for the second frame that runs chains of
/// child processes here (`support::byte_cell`): its cells and the cases followed in this
/// process are one machine's worth together.
pub(super) fn admitted<R>(job: impl FnOnce() -> R) -> R {
    chains().admit(job)
}

/// **The tests the running test binary holds, as its harness names them** — asked of the
/// binary itself (`--list`), so that an arm can hold what the suite's wiring declares to
/// what the harness will in fact run.
pub fn harness_tests() -> Vec<String> {
    let binary = std::env::current_exe().expect("the running test binary has a path");
    let listed = Command::new(&binary)
        .args(["--list", "--format", "terse"])
        .stdin(Stdio::null())
        .output()
        .expect("ask the test binary for its tests");
    assert!(
        listed.status.success(),
        "`{} --list --format terse`: {}",
        binary.display(),
        String::from_utf8_lossy(&listed.stderr)
    );
    String::from_utf8_lossy(&listed.stdout)
        .lines()
        .filter_map(|line| line.strip_suffix(": test"))
        .map(str::to_string)
        .collect()
}

/// [`follow`], with the limit a **command under test** is ended at — the refusing command
/// and the commands a route prints — for the arm that holds the limit itself, which cannot
/// wait two minutes for it. The runner's own children keep [`LIMIT`]. Every case passes
/// this process's gate ([`chains`]) before its sandbox is made.
pub fn follow_within(binary: &Path, case: &Case, under_test: Duration) -> Result<Followed, String> {
    chains().admit(|| followed(binary, case, under_test))
}

/// One case, followed — inside the gate.
fn followed(binary: &Path, case: &Case, under_test: Duration) -> Result<Followed, String> {
    let code = case.code();
    let sandbox = Sandbox::new(case.name, under_test);
    sandbox.plant(binary, case.state);
    let before = sandbox.witnesses(case);

    let stand = sandbox.at(case.from);
    let refused = sandbox.run(
        sandbox.tested(case, binary, &stand).args(case.refusing),
        sandbox.under_test,
    );
    let found = refusal_in(&refused.said, code);
    let mut seen = Seen {
        provoked: found.is_some() && refused.held.is_none() && !refused.success(),
        did_not_end: refused.held.is_some(),
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
        detail.push(refused.said.clone());
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
                if let Some(at) = path_spans(&spans, &stand).first() {
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
            if matches!(
                act,
                Act::Rerun | Act::RerunInTheRepository | Act::RerunWith(_)
            ) {
                seen.implied = true;
                // The command is the route's to give wherever the route prints a line to
                // run that the case does not run: a line the case runs as printed is
                // passed over by nobody, and a span that names a command is no line.
                let run_as_printed = |at: usize| {
                    case.acts.iter().any(|act| {
                        matches!(
                            act,
                            Act::Run(line)
                                | Act::RunFilled(Filled { line, .. })
                                | Act::RunChosen(Chosen { line, .. })
                                | Act::RunCited(Cited { line, .. })
                                if *line == at
                        )
                    })
                };
                if let Some(at) = command_spans(&spans).into_iter().find(|at| {
                    !run_as_printed(*at) && !sandbox.names(binary, case, spans[*at], &stand)
                }) {
                    return Err(sandbox.shown(&format!(
                        "an act runs the refusing command again, and the printed route \
                         holds a command line — span {at}, `{}` — that no act of the case \
                         runs. Where a route prints the command, the command is the route's \
                         to give: nothing was done",
                        spans[at]
                    )));
                }
            }
        }
        let listed = listed_in(&refused.said, code);
        let commands = listed_spans_in(&refused.said, code);
        let printed = Printed {
            spans: &spans,
            listed: &listed,
            commands: &commands,
        };
        for act in case.acts {
            let done = sandbox.perform(binary, case, *act, &printed, &mut seen)?;
            detail.push(done);
        }
        if !seen.same_refusal && !seen.failed && !seen.did_not_end && !seen.unsplit && !seen.unsaid
        {
            for check in case.holds {
                if let Err(why) = sandbox.check(*check) {
                    seen.failed = true;
                    detail.push(format!("does not hold: {why}"));
                }
            }
        }
    }

    let after = sandbox.witnesses(case);
    let named = case
        .witnesses
        .iter()
        .map(|name| format!("`outside/{name}`"))
        .chain(
            case.untouched
                .iter()
                .map(|name| format!("`{name}`, inside the repository,")),
        );
    for ((name, before), after) in named.zip(&before).zip(&after) {
        if before != after {
            seen.witness_changed = true;
            detail.push(format!(
                "the witness {name} changed: {}",
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
        sandbox: sandbox.root.clone(),
    })
}
