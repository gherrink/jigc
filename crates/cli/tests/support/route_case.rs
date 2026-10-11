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
//! identity) with `$HOME` in the sandbox, the sandbox's own identity and no inherited
//! `JIGC_PACK_DIR`; every child of the case runs under the same mask. The case's [`Step`]s plant its state; a script is
//! placed by a child process, as that section's Linux rule asks of a file that may be
//! executed. The refusing argv runs on the binary **handed in** — the frame names no
//! binary of its own, so the same case runs on another release's. Then the acts, then the
//! checks, then the witnesses, and one of eight [`Classification`]s.
//!
//! **Where a route names an act and not its object, the runner owns the object.** *Move
//! the directory out of the way* says where from and never where to; *point it at a regular
//! file inside this repository* names no file. A case may not supply either — that would be
//! a path of the case's own beside a printed one — so the runner does: a moved directory
//! goes to the sandbox's `moved/` under the path it had ([`Place::Moved`]) — as a file does
//! that a route says to move out of its path ([`Act::MoveOut`]) — and a link is
//! re-pointed at one of two files of the runner's, the one the sandbox's repository holds
//! committed ([`COMMITTED`]) or one written new, in no commit ([`NEW_FILE`]). Which of the
//! two a route means is a reading, and a case says which it followed
//! ([`Act::PointAtCommitted`], [`Act::PointAtNew`]). Where the route asks for *a script*,
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
//! place would pass over a route that prints a wrong one.
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
//! **A state step may run the binary under test** ([`Step::Jigc`]). Where a case's state is
//! one an earlier run of the product left — an install that stands — the product makes it,
//! on the binary handed in: the state is that release's own, and no case copies the
//! product's lines into its state to stand in for one.
//!
//! **A sandbox is removed whole, whatever its state shut.** A state may take the owner's
//! right to write or search a directory away, and a route need not give it back; the
//! runner gives every directory of the sandbox back to its owner before the scratch root
//! is removed, so no case leaves its root behind ([`Followed::sandbox`] is where it stood).
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
    /// Move the file the operand names out of its path — what a route asks for with *move
    /// the file out of …*: out of the repository, to the sandbox's `moved/` under the path
    /// it had ([`Place::Moved`]). A regular file, and the entry itself: a link is not one,
    /// nor a directory.
    MoveOut(Operand),
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
    /// ([`command_spans`]), and where it stands the case can come out no better than
    /// [`Classification::HoldsOperandImplied`].
    Rerun,
    /// [`Act::Rerun`], inside the sandbox's repository — what *run jigc from inside the
    /// target git repository* asks of a reader who stands outside one. The repository is
    /// the runner's, as every object is that a route names and does not print.
    RerunInTheRepository,
}

impl Act {
    /// What the act acts on, where it takes an operand.
    fn operand(self) -> Option<Operand> {
        match self {
            Act::Remove(operand)
            | Act::MoveAway(operand)
            | Act::MoveOut(operand)
            | Act::ReplaceWithDirectory(operand)
            | Act::PointAtCommitted(operand)
            | Act::PointAtNew(operand)
            | Act::PointAtScript(operand)
            | Act::Commit(operand)
            | Act::Mend(operand)
            | Act::MakeReadable(operand)
            | Act::MakeWritable(operand) => Some(operand),
            Act::Run(_) | Act::Rerun | Act::RerunInTheRepository => None,
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
    DidNotEnd,
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
}

/// The one classification of what was seen. **The order is the rule**: a witness that
/// changed outranks everything a followed route can otherwise come to — a lost file is
/// the worst of them, and the refusal that usually follows it would hide it — and a
/// repeated refusal outranks any other failure. **A command that did not end is said
/// where it stopped the case**: the refusing command held says nothing of a refusal, so
/// it is read before *not provoked*; a command of the route held is read behind a changed
/// witness, which it must not hide, and ahead of everything else a followed route comes to.
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
    /// How long a command under test — the refusing command, a command a route prints —
    /// may run before it is ended. The runner's own children have [`LIMIT`].
    under_test: Duration,
    /// How many children have run — each writes to files of its own.
    children: AtomicUsize,
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
    /// configuration, no global file, `$HOME` in the sandbox, and the embedded packs —
    /// **and the sandbox's identity**, as configuration git reads from the environment, so
    /// that it holds in every repository under the sandbox and none is invented in any.
    fn child(&self, program: &Path) -> Command {
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
    fn run(&self, command: &mut Command, limit: Duration) -> Ran {
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
        let text =
            |path: &Path| String::from_utf8_lossy(&fs::read(path).unwrap_or_default()).into_owned();
        Ran {
            code: status.code(),
            held,
            said: format!("{}{}", text(&out), text(&err)),
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
    fn plant(&self, binary: &Path, state: &[Step]) {
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
        case: &Case,
        act: Act,
        spans: &[&str],
        seen: &mut Seen,
    ) -> Result<String, String> {
        let code = case.code();
        let stand = self.at(case.from);
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
                Act::MakeReadable(_) => format!("make `{named}` readable ({how})"),
                Act::MakeWritable(_) => format!("make `{named}` writable ({how})"),
                Act::Run(_) | Act::Rerun | Act::RerunInTheRepository => {
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
                Act::MakeReadable(_) => self.permit(&path, 0o400, "its owner may read it"),
                Act::MakeWritable(_) => self.permit(&path, 0o200, "its owner may write it"),
                Act::Run(_) | Act::Rerun | Act::RerunInTheRepository => {
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
            Act::Run(at) => {
                let line = span(at)?;
                let words = super::shell_words(line, &stand, &self.home());
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
                (
                    format!("run `{line}` (span {at})"),
                    self.run(
                        self.tested(case, program, &stand).args(&words[1..]),
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
        } else {
            seen.failed = true;
            format!("{what}: {}\n{}", ran.exit(), ran.said)
        })
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
        }
    }
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

/// [`follow`], with the limit a **command under test** is ended at — the refusing command
/// and the commands a route prints — for the arm that holds the limit itself, which cannot
/// wait two minutes for it. The runner's own children keep [`LIMIT`].
pub fn follow_within(binary: &Path, case: &Case, under_test: Duration) -> Result<Followed, String> {
    let code = case.code();
    let sandbox = Sandbox::new(case.name, under_test);
    sandbox.plant(binary, case.state);
    let before = sandbox.witnesses(case.witnesses);

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
            if matches!(act, Act::Rerun | Act::RerunInTheRepository) {
                seen.implied = true;
                if let Some(at) = command_spans(&spans).first() {
                    return Err(sandbox.shown(&format!(
                        "an act runs the refusing command again, and the printed route \
                         holds a command line — span {at}, `{}`. Where a route prints the \
                         command, the command is the route's to give: nothing was done",
                        spans[*at]
                    )));
                }
            }
        }
        for act in case.acts {
            let done = sandbox.perform(binary, case, *act, &spans, &mut seen)?;
            detail.push(done);
        }
        if !seen.same_refusal && !seen.failed && !seen.did_not_end {
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
        sandbox: sandbox.root.clone(),
    })
}
