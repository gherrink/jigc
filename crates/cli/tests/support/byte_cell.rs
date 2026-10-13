//! **A byte cell** — the frame of the byte table (`crates/cli/tests/byte_table.rs`; the
//! plan is `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md`, and
//! this file is its task `B1`).
//!
//! The exit rule's first clause is about lost files: *no jigc command at exit 0 destroys
//! bytes no git object holds or commits content the user did not ask for*, and *where jigc
//! cannot tell it refuses before writing* (`DECISIONS.md` → *2026-10-04 — The exit rule,
//! revised*). The byte table runs one writing command between two pictures of a sandbox
//! and says what the difference is. The first half of this file starts no process:
//! **the picture as data** ([`Manifest`]), **what a command may change, as its design doc
//! declares it** ([`Row`], [`Line`]), and **the comparison** ([`compare`]) — a function
//! from two manifests, how the command ended and a row to one of eight
//! [`Classification`]s.
//!
//! **The second half runs a cell** ([`run`], the plan's task `B2b`): a sandbox over a copy
//! of a prepared state ([`Built`], [`Prepared`]); a manifest; the cell's leads ([`Lead`])
//! — its lead-ins on the binary under test, and what is put into the state by hand
//! ([`Plant`]); a manifest; the command; a manifest; the comparison. **The binary is
//! handed in** — a state is built on it, and neither this frame nor a cell names one —
//! and every child is the route catalog's frame's: its sandbox, its mask, its one function
//! that runs a child and waits no longer than a limit, its gate
//! ([`route_case`](super::route_case) → *A second frame runs its children through this
//! runner*). A suite can start a child of a cell in one way only, which is to hand the
//! cell to [`run`]. **What takes a manifest off a sandbox is one reader** ([`Reader`]):
//! the file layer off the disk, git's layer asked of git.
//!
//! **A row is written from the docs, never from what the command did.** A line of a row is
//! a place, what may happen there, when, and whether it holds when the command refuses —
//! and **the sentence of the design doc it rests on, quoted** ([`Quote`]). The quote is a
//! run of the doc's own bytes under a named heading, and [`unquoted`] holds it to the
//! doc's text: a doc that moves reddens the row. The vocabulary — [`Where`], [`May`],
//! [`Holding`], [`When`] — is what the four rows of the pilot's acceptance needed and no
//! more (`…/reshape/byte-table-acceptance-rows.md` → §7, which says for each word which
//! lines needed it) — and two words one ruling added, for what a refusal may do to a cache
//! ([`May::Rebuilt`], [`When::Refused`]; the record's §4a).
//!
//! **Whose bytes they are is settled by construction, never by a list copied out of the
//! product.** A planted entry is the user's ([`Planted`]): where its bytes stand nowhere
//! after exit 0 the cell is *lost* **whatever the row declares** — only the consent
//! spelling is exempt, and only where the command's output names the planted path. What
//! the binary under test wrote in a cell's lead-ins is the product's own, read off a
//! manifest taken before them ([`Seen::base`]): a difference there is judged against the
//! row's lines and is never *lost*. And prose handed to jigc — a doc staged under a work
//! unit's `docs/` — must, once its working area is gone, stand byte for byte in a commit
//! the command made, as a regular file. The unit's commit doc is the exception its
//! design doc names: it is rendered into the commit's message and promoted nowhere
//! ([`staged_docs`]).
//!
//! **A classification is what one binary did on one day, never a verdict.** The order they
//! are read in is [`compare`]'s; everything but [`Classification::Holds`] and
//! [`Classification::RefusesNothingChanged`] is red.
//!
//! **What *held* means is an open question of reading** (the plan's §7, question 1), and a
//! cell follows the narrowest reading: bytes are held where a ref, a checkout's `HEAD`, an
//! index or the newest stash entry reaches an object that has them ([`Manifest::held`]).
//! Where the bytes are in the object store and nothing reaches them, the verdict says that
//! it hangs on the question ([`Verdict::hangs_on_reach`]); the reading in between —
//! reachable through a reflog — is not carried apart from the store.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::route_case::{self, Sandbox, Step};
use super::trial_corpus::{State, TrialCorpus};

/// Where the repository stands under a sandbox's root: a manifest's paths are from the
/// root, so the repository's `CLAUDE.md` is `repo/CLAUDE.md`, beside `home/…`,
/// `outside/…` and `tmp/…`.
pub const REPO: &str = "repo";

/// One entry of the file layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    /// A directory, with its permission bits.
    Dir(u32),
    /// A regular file: its permission bits, **the id git gives its bytes through the
    /// path's filters** — so a file rewritten to the same content under a line-ending
    /// conversion is the same file — and the bytes themselves, which the comparison reads
    /// only for a line that merges into a file or takes lines out of one.
    File {
        mode: u32,
        blob: String,
        bytes: Vec<u8>,
    },
    /// A symbolic link, with its target as written.
    Link(String),
    /// A FIFO or a device node.
    Special,
}

impl Entry {
    /// A regular file with these bytes, not executable, under an id that stands for them.
    pub fn file(bytes: &str) -> Entry {
        Entry::File {
            mode: 0o644,
            blob: blob_of(bytes),
            bytes: bytes.as_bytes().to_vec(),
        }
    }

    /// Whether the two are the same entry: kind, permission bits, and the id or the target.
    fn same(&self, other: &Entry) -> bool {
        match (self, other) {
            (
                Entry::File { mode, blob, .. },
                Entry::File {
                    mode: m, blob: b, ..
                },
            ) => mode == m && blob == b,
            _ => self == other,
        }
    }

    fn blob(&self) -> Option<&str> {
        match self {
            Entry::File { blob, .. } => Some(blob),
            _ => None,
        }
    }

    fn lines(&self) -> Option<Vec<&[u8]>> {
        match self {
            Entry::File { bytes, .. } => Some(bytes.split_inclusive(|b| *b == b'\n').collect()),
            _ => None,
        }
    }
}

/// The id a hand-built manifest gives a file's bytes: one that stands for them and for no
/// others. A manifest taken off a sandbox has git's.
pub fn blob_of(bytes: &str) -> String {
    format!("blob:{bytes}")
}

/// One index entry: its mode, the id of its blob, and its flags as git prints them.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Staged {
    pub mode: u32,
    pub blob: String,
    pub flags: String,
}

/// One checkout: what its `HEAD` is — empty where it is unborn — the branch it is on, and
/// its index.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Checkout {
    pub head: String,
    pub branch: Option<String>,
    pub index: BTreeMap<String, Staged>,
}

/// One commit within reach: its first parent, and what it changed against it — a path to
/// the mode and blob it holds there now, or to nothing where it removed the path.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Commit {
    pub parent: Option<String>,
    pub changed: BTreeMap<String, Option<(u32, String)>>,
}

/// Where a plant's marker stands: the files of the sandbox that hold it, whether an object
/// within reach does ([`Manifest::held`]'s reach), and whether any object of the store does.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stands {
    pub files: BTreeSet<String>,
    pub held: bool,
    pub stored: bool,
}

/// **A picture of a sandbox, as data** — what the comparison is of (the plan's §2, *The
/// manifest*).
///
/// *The file layer:* every entry under the sandbox's root, by its path from the root —
/// the repository's working tree, tracked, untracked and ignored, `.jigc/` included; every
/// other checkout that stands in the sandbox; `home/`, `outside/` and `tmp/`. **Not** the
/// inside of a `.git` directory, but for the files of git's that nobody else reads: the
/// hooks, `info/exclude`, and the admin files of a registration whose directory is gone.
/// *Git's layer, asked of git:* the rest. Modification times are in no layer.
#[derive(Clone, Debug, Default)]
pub struct Manifest {
    pub files: BTreeMap<String, Entry>,
    /// The hooks directory git resolves for the repository, from the sandbox's root.
    pub hooks: String,
    /// Every checkout whose directory is there, by that directory from the root: the main
    /// one is [`REPO`].
    pub checkouts: BTreeMap<String, Checkout>,
    pub refs: BTreeMap<String, String>,
    /// The stash's entries, newest first, by commit id.
    pub stash: Vec<String>,
    /// The worktree registrations, by name, each with its path from the sandbox's root.
    pub registrations: BTreeMap<String, String>,
    /// The repository's local configuration, as git lists it.
    pub config: String,
    /// Every object id of the store.
    pub objects: BTreeSet<String>,
    /// **The ids held under the narrowest reading**: what a ref, a checkout's `HEAD`, an
    /// index or the newest stash entry reaches.
    pub held: BTreeSet<String>,
    /// The commits within that reach, by id.
    pub commits: BTreeMap<String, Commit>,
    /// Where each plant's marker stands.
    pub markers: BTreeMap<String, Stands>,
}

/// **One thing put into a prepared state by hand before the command runs**, as the
/// comparison needs it: where it stands, from the sandbox's root, and the bytes found
/// nowhere else that it carries. A plant with no bytes is a fault of the cell — an empty
/// file's blob is the one every empty file has, and would read as held.
#[derive(Clone, Debug)]
pub struct Planted {
    pub at: String,
    pub marker: String,
}

/// The sentence a line rests on: the doc, from the repository's root; the heading it
/// stands under, without its `#`s; and the words — a run of the doc's own bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quote {
    pub doc: &'static str,
    pub heading: &'static str,
    pub words: &'static str,
}

/// The place a line of the file layer speaks of. A path is from the repository's root,
/// and a `*` in it stands for one component — a work unit's id.
#[derive(Clone, Copy, Debug)]
pub enum Where {
    /// One path.
    Path(&'static str),
    /// A directory and everything under it.
    Under(&'static str),
    /// The `pre-commit` hook in the hooks directory git resolves — no fixed path.
    Hook,
    /// Wherever a staged doc of the unit lands: the path is the product's to compute, so
    /// the place is known by the bytes that arrive there ([`May::CopyOfAStagedDoc`]).
    AStagedDocsHome,
}

/// What may happen at a place.
#[derive(Clone, Copy, Debug)]
pub enum May {
    /// Made, or replaced by any bytes, as a regular file; under a directory, anything.
    WriteWhole,
    /// Made, or changed so that every line that stood there stays, in order.
    MergeInto,
    /// Changed so that every line that stands there afterwards stood there before, in
    /// order.
    TakeOutOf,
    Removed,
    /// Made, or written whole, as a regular file — or a directory made. **Nothing is
    /// gone**: what a refusal may do to a cache, which is rebuilt and never taken away.
    Rebuilt,
    /// Made or replaced by a regular file that is, byte for byte, a doc staged in a work
    /// unit.
    CopyOfAStagedDoc,
    /// Gone from its place and standing, the same entry, under this directory of the
    /// repository.
    MovedAsideTo(&'static str),
}

/// What a commit the command makes may hold, beside what the row's lines of the file
/// layer wrote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holding {
    /// Nothing else: not the index as it stood.
    OnlyTheLines,
    /// The main checkout's index, whole, as it stood.
    TheIndex,
    /// What every other checkout of the sandbox had staged.
    TheWorktrees,
}

/// What a line allows.
#[derive(Clone, Copy, Debug)]
pub enum Allows {
    Files(Where, May),
    /// A commit on the checked-out branch.
    Commit(Holding),
    /// The main checkout's index staged at the places the row's lines of the file layer
    /// wrote.
    Staged,
    /// A worktree's registration removed, with its checkout, at a path under this
    /// directory of the repository.
    Unregistered(&'static str),
}

/// A condition a line carries: read off the manifest taken before the command, or off how
/// the command ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum When {
    Always,
    /// The repository has no commit: its `HEAD` is unborn.
    NoCommitYet,
    /// The command refused. A line under it holds on a refusal and **only** then: what
    /// the command may do where it exits 0 is another line's to say.
    Refused,
}

/// **One line of an allowance.** `id` is the line's number in the committed record; a
/// line with no quote is none.
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub id: &'static str,
    pub allows: Allows,
    pub when: When,
    /// Whether the line still holds when the command refuses — from its sentence.
    pub on_refusal: bool,
    pub quotes: &'static [Quote],
}

/// **A leaf's row**: the command as `cli::cli::VERB_KINDS` holds it, the flag its owner
/// doc names as the consent to a destruction where it has one, and the allowance.
#[derive(Clone, Copy, Debug)]
pub struct Row {
    pub leaf: &'static [&'static str],
    pub consent: Option<&'static str>,
    pub lines: &'static [Line],
}

/// What comparing one cell came to. The words are the plan's and are what a table prints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classification {
    DidNotEnd,
    Lost,
    GitMoved,
    WrittenOutside,
    RefusedAndChanged,
    NotTheListedOutcome,
    Holds,
    RefusesNothingChanged,
}

impl Classification {
    pub fn as_str(self) -> &'static str {
        match self {
            Classification::DidNotEnd => "the command did not end",
            Classification::Lost => "lost",
            Classification::GitMoved => "git's state moved outside its allowance",
            Classification::WrittenOutside => "written outside its allowance",
            Classification::RefusedAndChanged => "refused, and changed what a refusal may not",
            Classification::NotTheListedOutcome => "not the listed outcome",
            Classification::Holds => "holds",
            Classification::RefusesNothingChanged => "refuses, nothing changed",
        }
    }

    /// Every classification is red but the two that hold.
    pub fn is_red(self) -> bool {
        !matches!(
            self,
            Classification::Holds | Classification::RefusesNothingChanged
        )
    }
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// What a cell is listed as: a command that acts, or one that refuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Listed {
    Acts,
    Refuses,
}

/// How the command under test ended.
#[derive(Clone, Copy, Debug)]
pub struct Ended<'a> {
    /// `false` where it had not ended at the frame's limit.
    pub ended: bool,
    pub code: Option<i32>,
    /// Its stdout, then its stderr.
    pub said: &'a str,
}

/// **What the frame saw of one cell** — everything [`compare`] reads.
#[derive(Clone, Copy, Debug)]
pub struct Seen<'a> {
    /// The sandbox before the cell's lead-ins: what stands in `before` and not here, the
    /// binary under test wrote.
    pub base: &'a Manifest,
    /// The sandbox before the command, the plants in.
    pub before: &'a Manifest,
    pub after: &'a Manifest,
    pub planted: &'a [Planted],
    pub ended: Ended<'a>,
    /// Whether the cell runs the row's consent spelling.
    pub consent: bool,
    pub listed: Listed,
}

/// What one cell came to: the classification, why, **how many differences each line of
/// the row excused** — every line, so that one that excuses much is seen and one that
/// excuses nothing is too — and whether the classification would be another under a wider
/// reading of *held*.
#[derive(Clone, Debug)]
pub struct Verdict {
    pub classification: Classification,
    pub why: Vec<String>,
    pub excused: Vec<(&'static str, usize)>,
    pub hangs_on_reach: bool,
}

/// Whether `path` is `pattern`, component for component, a `*` standing for any one.
fn is_at(pattern: &str, path: &str) -> bool {
    let (want, got): (Vec<&str>, Vec<&str>) =
        (pattern.split('/').collect(), path.split('/').collect());
    want.len() == got.len() && want.iter().zip(&got).all(|(w, g)| *w == "*" || w == g)
}

/// Whether `path` is the directory `pattern` names or lies under it.
fn under(pattern: &str, path: &str) -> bool {
    let depth = pattern.split('/').count();
    let head: Vec<&str> = path.split('/').take(depth).collect();
    head.len() == depth && is_at(pattern, &head.join("/"))
}

/// `path` from the repository's root, where it lies in the repository.
fn in_repo(path: &str) -> Option<&str> {
    path.strip_prefix(REPO)?.strip_prefix('/')
}

/// Whether `small`'s lines all stand in `large`, in order.
fn stand_in(small: &[&[u8]], large: &[&[u8]]) -> bool {
    let mut rest = large.iter();
    small.iter().all(|line| rest.any(|other| other == line))
}

/// One difference of the file layer.
struct Diff<'a> {
    path: &'a str,
    before: Option<&'a Entry>,
    after: Option<&'a Entry>,
}

/// What a comparison keeps as it goes.
struct Tally<'a> {
    row: &'a Row,
    acted: bool,
    seen: &'a Seen<'a>,
    /// The ids of the docs staged in a work unit before the command.
    staged: BTreeSet<&'a str>,
    excused: Vec<usize>,
    lost: Vec<String>,
    git: Vec<String>,
    outside: Vec<String>,
    hangs: bool,
}

impl<'a> Tally<'a> {
    /// The lines in force: their condition holds, and the command acted or the line holds
    /// on a refusal.
    fn in_force(&self) -> impl Iterator<Item = (usize, &'a Line)> + '_ {
        let unborn = self
            .seen
            .before
            .checkouts
            .get(REPO)
            .is_none_or(|main| main.head.is_empty());
        self.row.lines.iter().enumerate().filter(move |(_, line)| {
            (self.acted || line.on_refusal)
                && match line.when {
                    When::Always => true,
                    When::NoCommitYet => unborn,
                    When::Refused => !self.acted,
                }
        })
    }

    /// Whether `place` is where `path` — from the sandbox's root — stands.
    fn placed(&self, place: Where, path: &str) -> bool {
        match (place, in_repo(path)) {
            (Where::Hook, _) => path == format!("{}/pre-commit", self.seen.before.hooks),
            (Where::Path(pattern), Some(rel)) => is_at(pattern, rel),
            (Where::Under(pattern), Some(rel)) => under(pattern, rel),
            (Where::AStagedDocsHome, Some(rel)) => !under(".git", rel),
            (_, None) => false,
        }
    }

    /// The line in force that lets the difference `diff` happen, by its place in the row.
    fn covering(&self, diff: &Diff<'_>) -> Option<usize> {
        self.in_force()
            .find(|(_, line)| {
                let Allows::Files(place, may) = line.allows else {
                    return false;
                };
                let aside =
                    |dir: &str, path: &str| in_repo(path).is_some_and(|rel| under(dir, rel));
                if let May::MovedAsideTo(dir) = may
                    && aside(dir, diff.path)
                {
                    // The arrival: a directory, or an entry that left a place of the line.
                    return diff.before.is_none()
                        && diff.after.is_some_and(|arrived| {
                            matches!(arrived, Entry::Dir(_))
                                || self.seen.before.files.iter().any(|(from, left)| {
                                    left.same(arrived)
                                        && self.placed(place, from)
                                        && !self.seen.after.files.contains_key(from)
                                })
                        });
                }
                if !self.placed(place, diff.path) {
                    return false;
                }
                match (may, diff.before, diff.after) {
                    (May::WriteWhole, _, Some(Entry::File { .. })) => true,
                    (May::WriteWhole, _, _) => matches!(place, Where::Under(_)),
                    (May::MergeInto, None, Some(Entry::File { .. })) => true,
                    (May::MergeInto, Some(was), Some(is)) => match (was.lines(), is.lines()) {
                        (Some(was), Some(is)) => stand_in(&was, &is),
                        _ => false,
                    },
                    (May::TakeOutOf, Some(was), Some(is)) => match (was.lines(), is.lines()) {
                        (Some(was), Some(is)) => stand_in(&is, &was),
                        _ => false,
                    },
                    (May::Removed, Some(_), None) => true,
                    (May::Rebuilt, _, Some(Entry::File { .. })) => true,
                    (May::Rebuilt, None, Some(Entry::Dir(_))) => true,
                    (May::CopyOfAStagedDoc, _, Some(is)) => {
                        is.blob().is_some_and(|blob| self.staged.contains(blob))
                    }
                    (May::MovedAsideTo(dir), Some(left), None) => {
                        self.seen.after.files.iter().any(|(to, arrived)| {
                            aside(dir, to)
                                && arrived.same(left)
                                && !self.seen.before.files.contains_key(to)
                        })
                    }
                    _ => false,
                }
            })
            .map(|(at, _)| at)
    }

    /// The line in force that lets the commit hold `entry` at `path`, from the
    /// repository's root, because a line of the file layer wrote it there.
    fn written_by_a_line(&self, path: &str, entry: Option<&(u32, String)>) -> Option<usize> {
        let key = format!("{REPO}/{path}");
        let after = self.seen.after.files.get(&key);
        self.in_force()
            .find(|(_, line)| {
                let Allows::Files(place, may) = line.allows else {
                    return false;
                };
                if !self.placed(place, &key) {
                    return false;
                }
                match (may, entry) {
                    (May::Removed | May::MovedAsideTo(_), None) => true,
                    (May::Removed | May::MovedAsideTo(_), Some(_)) => false,
                    (_, None) => false,
                    // A regular file, and no link: mode `100644` or `100755`.
                    (May::CopyOfAStagedDoc, Some((mode, blob))) => {
                        mode & 0o170_000 == 0o100_000 && self.staged.contains(blob.as_str())
                    }
                    (_, Some((mode, blob))) => {
                        mode & 0o170_000 == 0o100_000
                            && after.and_then(Entry::blob) == Some(blob.as_str())
                    }
                }
            })
            .map(|(at, _)| at)
    }

    /// The line in force that allows `wanted`, by its place in the row.
    fn line_for(&self, wanted: impl Fn(&Allows) -> bool) -> Option<usize> {
        self.in_force()
            .find(|(_, line)| wanted(&line.allows))
            .map(|(at, _)| at)
    }

    /// Whether the bytes under `blob` still stand after the command: at a path of the
    /// sandbox, or in an object within reach. Where only the store has them, the verdict
    /// hangs on the question of reach.
    fn stands(&mut self, blob: &str) -> bool {
        let after = self.seen.after;
        if after.files.values().any(|entry| entry.blob() == Some(blob)) || after.held.contains(blob)
        {
            return true;
        }
        self.hangs |= after.objects.contains(blob);
        false
    }
}

/// **The one classification of what was seen**, with why. `Err` is a fault of the cell as
/// written, which no classification covers: a plant with no bytes, two plants with one
/// marker, a plant the manifest taken before the command does not hold where the cell
/// says it stands.
///
/// **The order is the rule.** A command that did not end is said first: nothing it left
/// can be read as its answer. Then, for a command that exited 0: *lost* — the user's bytes
/// stand nowhere — outranks everything, since what follows a loss would hide it; then
/// *git's state moved outside its allowance*; then *written outside its allowance*. A
/// command that refused is held to a second rule, and to that one alone: it may change
/// only what a line that holds on a refusal covers. Then the listed outcome, and the two
/// that are not red.
pub fn compare(row: &Row, seen: &Seen<'_>) -> Result<Verdict, String> {
    for (at, plant) in seen.planted.iter().enumerate() {
        if plant.marker.is_empty() {
            return Err(format!(
                "the plant at `{}` has no bytes: a plant carries bytes found nowhere else",
                plant.at
            ));
        }
        if seen.planted[..at]
            .iter()
            .any(|other| other.marker == plant.marker)
        {
            return Err(format!(
                "two plants carry the marker of the one at `{}`: a plant's bytes are its own",
                plant.at
            ));
        }
        let stood = seen.before.markers.get(&plant.marker);
        if !stood.is_some_and(|stood| stood.files.contains(&plant.at)) {
            return Err(format!(
                "the plant at `{}` does not stand there in the manifest taken before the \
                 command",
                plant.at
            ));
        }
    }
    let acted = seen.ended.code == Some(0);
    let verdict = |classification, why: Vec<String>, tally: Option<&Tally<'_>>| Verdict {
        classification,
        why,
        excused: row
            .lines
            .iter()
            .enumerate()
            .map(|(at, line)| (line.id, tally.map_or(0, |tally| tally.excused[at])))
            .collect(),
        hangs_on_reach: tally.is_some_and(|tally| tally.hangs),
    };
    if !seen.ended.ended {
        return Ok(verdict(
            Classification::DidNotEnd,
            vec!["the command had not ended at the frame's limit".to_string()],
            None,
        ));
    }

    let (before, after) = (seen.before, seen.after);
    let mut tally = Tally {
        row,
        acted,
        seen,
        staged: staged_docs(before)
            .into_iter()
            .map(|(_, blob)| blob)
            .collect(),
        excused: vec![0; row.lines.len()],
        lost: Vec::new(),
        git: Vec::new(),
        outside: Vec::new(),
        hangs: false,
    };

    // --- The plants: the user's, whatever the row declares. ---
    let nowhere = Stands::default();
    let mut judged: BTreeSet<&str> = BTreeSet::new();
    for plant in seen.planted {
        let stands = after.markers.get(&plant.marker).unwrap_or(&nowhere);
        if stands.files.contains(&plant.at) {
            continue;
        }
        let named = in_repo(&plant.at).unwrap_or(&plant.at);
        if seen.consent && row.consent.is_some() && seen.ended.said.contains(named) {
            // The consent, and the path named: the difference is the lines' to judge.
            continue;
        }
        judged.insert(&plant.at);
        let aside = tally.in_force().find_map(|(at, line)| match line.allows {
            Allows::Files(place, May::MovedAsideTo(dir)) if tally.placed(place, &plant.at) => {
                let to = stands
                    .files
                    .iter()
                    .find(|to| in_repo(to).is_some_and(|rel| under(dir, rel)))?;
                Some((at, to))
            }
            _ => None,
        });
        if let Some((at, to)) = aside {
            tally.excused[at] += 1;
            judged.insert(to);
        } else if !stands.files.is_empty() {
            tally.outside.push(format!(
                "the planted `{}` is gone from its place, and its bytes stand at {:?}",
                plant.at, stands.files
            ));
        } else if stands.held {
            tally.git.push(format!(
                "the planted `{}` is gone from every path of the sandbox, and an object \
                 within reach still has its bytes: the command put something else in its place",
                plant.at
            ));
        } else {
            tally.hangs |= stands.stored;
            tally.lost.push(format!(
                "the planted `{}` is gone, and its bytes stand at no path of the sandbox and in \
                 no object within reach",
                plant.at
            ));
        }
    }

    let commits = tally.line_for(|allows| matches!(allows, Allows::Commit(_)));
    let holding = commits.map(|at| match row.lines[at].allows {
        Allows::Commit(holding) => holding,
        _ => unreachable!("the line found is a commit's"),
    });
    let main = (before.checkouts.get(REPO), after.checkouts.get(REPO));
    // The commits the command made on the checked-out branch: from its new `HEAD` back to
    // the old one, each a commit the manifest before did not hold.
    let mut made: Vec<&Commit> = Vec::new();
    let mut landed = false;
    if let (Some(was), Some(is)) = main
        && was.head != is.head
    {
        let mut at = Some(is.head.as_str()).filter(|head| !head.is_empty());
        while let Some(id) = at
            && id != was.head
            && !before.commits.contains_key(id)
            && let Some(commit) = after.commits.get(id)
        {
            made.push(commit);
            at = commit.parent.as_deref();
        }
        landed = at.unwrap_or_default() == was.head && was.branch == is.branch;
        if !landed || commits.is_none() {
            made.clear();
            tally.git.push(format!(
                "the main checkout's `HEAD` moved from `{}` to `{}`, and no line lets the \
                 command commit that",
                was.head, is.head
            ));
        }
    }
    // What the main checkout then holds because its `HEAD` moved there: the last of those
    // commits to change a path says what stands at it.
    let mut followed: BTreeMap<&str, Option<&(u32, String)>> = BTreeMap::new();
    for commit in made.iter().rev() {
        for (path, entry) in &commit.changed {
            followed.insert(path, entry.as_ref());
        }
    }

    // --- The file layer, a difference at a time. ---
    let paths: BTreeSet<&String> = before.files.keys().chain(after.files.keys()).collect();
    let diffs: Vec<Diff<'_>> = paths
        .into_iter()
        .map(|path| Diff {
            path,
            before: before.files.get(path),
            after: after.files.get(path),
        })
        .filter(|diff| match (diff.before, diff.after) {
            (Some(was), Some(is)) => !was.same(is),
            (None, None) => false,
            _ => true,
        })
        .filter(|diff| !judged.contains(diff.path))
        .collect();
    let mut by: BTreeMap<&str, usize> = BTreeMap::new();
    let mut open: Vec<&Diff<'_>> = Vec::new();
    for diff in &diffs {
        // A line's, or the checkout following the commit the command made there.
        let follows = || {
            let entry = followed.get(in_repo(diff.path)?)?;
            let same = match (entry, diff.after) {
                (Some((_, blob)), Some(is)) => is.blob() == Some(blob.as_str()),
                (None, None) => true,
                _ => false,
            };
            commits.filter(|_| same)
        };
        match tally.covering(diff).or_else(follows) {
            Some(at) => {
                by.insert(diff.path, at);
            }
            None => open.push(diff),
        }
    }
    // A directory on the way to a covered write is covered with it, and so is one that
    // emptied under a covered removal: the docs name files.
    open.retain(|diff| {
        let inside = format!("{}/", diff.path);
        let through = match (diff.before, diff.after) {
            (None, Some(Entry::Dir(_))) | (Some(Entry::Dir(_)), None) => by
                .iter()
                .find(|(path, _)| path.starts_with(&inside))
                .map(|(_, at)| *at),
            _ => None,
        };
        let emptied =
            diff.after.is_some() || !after.files.keys().any(|path| path.starts_with(&inside));
        match through {
            Some(at) if emptied => {
                by.insert(diff.path, at);
                false
            }
            _ => true,
        }
    });
    for at in by.values() {
        tally.excused[*at] += 1;
    }
    for diff in open {
        let what = match (diff.before, diff.after) {
            (None, _) => "was made",
            (_, None) => "is gone",
            _ => "is not what it was",
        };
        if in_repo(diff.path).is_some_and(|rel| rel.split('/').any(|part| part == ".git")) {
            tally
                .git
                .push(format!("`{}`, a file of git's, {what}", diff.path));
            continue;
        }
        // Bytes no line covers that stand nowhere are lost — unless a lead-in of the
        // cell wrote them: those are the product's own, and never a loss.
        let own = diff.before.is_some_and(|was| {
            !seen
                .base
                .files
                .get(diff.path)
                .is_some_and(|base| base.same(was))
        });
        match diff.before.and_then(Entry::blob) {
            Some(blob) if !own && !tally.stands(blob) => tally.lost.push(format!(
                "`{}` {what}, no line covers it, and the bytes it had stand at no path of \
                 the sandbox and in no object within reach",
                diff.path
            )),
            _ => tally
                .outside
                .push(format!("`{}` {what}, and no line covers it", diff.path)),
        }
    }

    // --- Git's layer. ---
    if let Some(gone) = before.objects.difference(&after.objects).next() {
        tally
            .git
            .push(format!("the object `{gone}` was in the store and is gone"));
    }
    if before.stash != after.stash {
        tally.git.push("the stash is not what it was".to_string());
    }
    if before.config != after.config {
        tally
            .git
            .push("the repository's configuration is not what it was".to_string());
    }
    let branch = main
        .0
        .and_then(|was| was.branch.as_ref())
        .map(|branch| format!("refs/heads/{branch}"));
    let names: BTreeSet<&String> = before.refs.keys().chain(after.refs.keys()).collect();
    for name in names {
        if before.refs.get(name) == after.refs.get(name) {
            continue;
        }
        match commits {
            Some(at) if landed && branch.as_ref() == Some(name) => tally.excused[at] += 1,
            _ => tally
                .git
                .push(format!("the ref `{name}` is not what it was")),
        }
    }
    let mut committed: BTreeSet<&str> = BTreeSet::new();
    for commit in &made {
        for (path, entry) in &commit.changed {
            let entry = entry.as_ref();
            if let Some((_, blob)) = entry {
                committed.insert(blob);
            }
            let staged = |checkout: &Checkout| match (checkout.index.get(path), entry) {
                (Some(staged), Some((mode, blob))) => staged.mode == *mode && staged.blob == *blob,
                (None, None) => true,
                _ => false,
            };
            let as_it_stood = match holding {
                Some(Holding::TheIndex) => main.0.is_some_and(staged),
                Some(Holding::TheWorktrees) => before
                    .checkouts
                    .iter()
                    .any(|(at, checkout)| at != REPO && entry.is_some() && staged(checkout)),
                _ => false,
            };
            if let Some(at) = tally.written_by_a_line(path, entry) {
                tally.excused[at] += 1;
            } else if as_it_stood {
                tally.excused[commits.expect("a commit was let in by a line")] += 1;
            } else {
                tally.git.push(match entry {
                    Some((mode, _)) if mode & 0o170_000 == 0o120_000 => format!(
                        "a commit the command made holds a link at `{path}`, which no line \
                         declares"
                    ),
                    Some(_) => format!(
                        "a commit the command made holds at `{path}` what the row does not \
                         let it hold"
                    ),
                    None => format!(
                        "a commit the command made removes `{path}`, which no line declares"
                    ),
                });
            }
        }
    }
    // Prose handed to jigc: a staged doc whose working area is gone stands in a commit the
    // command made, or it is somewhere else, or it is lost.
    if acted {
        for (path, blob) in staged_docs(before) {
            if after.files.contains_key(path) || committed.contains(blob) {
                continue;
            }
            if tally.stands(blob) {
                tally.git.push(format!(
                    "the staged doc `{path}` is gone from its working area and stands in no \
                     commit the command made; its bytes stand elsewhere"
                ));
            } else {
                tally.lost.push(format!(
                    "the staged doc `{path}` is gone from its working area, and its bytes \
                     stand in no commit, at no path of the sandbox and in no object within reach"
                ));
            }
        }
    }
    // The registrations, and the other checkouts with them.
    let unregistered = |tally: &Tally<'_>, path: &str| {
        tally.line_for(|allows| {
            matches!(allows, Allows::Unregistered(dir)
                if in_repo(path).is_some_and(|rel| under(dir, rel)))
        })
    };
    let names: BTreeSet<&String> = before
        .registrations
        .keys()
        .chain(after.registrations.keys())
        .collect();
    for name in names {
        match (
            before.registrations.get(name),
            after.registrations.get(name),
        ) {
            (was, is) if was == is => {}
            (Some(path), None) => match unregistered(&tally, path) {
                Some(at) => tally.excused[at] += 1,
                None => tally.git.push(format!(
                    "the registration `{name}`, of the worktree at `{path}`, is gone"
                )),
            },
            _ => tally
                .git
                .push(format!("the registration `{name}` is not what it was")),
        }
    }
    let staging = tally.line_for(|allows| matches!(allows, Allows::Staged | Allows::Commit(_)));
    let places: BTreeSet<&String> = before
        .checkouts
        .keys()
        .chain(after.checkouts.keys())
        .collect();
    for place in places {
        match (before.checkouts.get(place), after.checkouts.get(place)) {
            (Some(_), None) if place != REPO => match unregistered(&tally, place) {
                Some(at) => tally.excused[at] += 1,
                None => tally.git.push(format!("the checkout at `{place}` is gone")),
            },
            (Some(was), Some(is)) => {
                if place != REPO && was.head != is.head {
                    tally
                        .git
                        .push(format!("the `HEAD` of the checkout at `{place}` moved"));
                }
                let paths: BTreeSet<&String> = was.index.keys().chain(is.index.keys()).collect();
                for path in paths {
                    let (was, is) = (was.index.get(path), is.index.get(path));
                    if was == is {
                        continue;
                    }
                    let entry = is.map(|staged| (staged.mode, staged.blob.clone()));
                    let follows = followed
                        .get(path.as_str())
                        .is_some_and(|held| *held == entry.as_ref());
                    let line = Some(place).filter(|place| *place == REPO).and_then(|_| {
                        tally
                            .written_by_a_line(path, entry.as_ref())
                            .or(commits.filter(|_| follows))
                    });
                    match (staging, line) {
                        (Some(_), Some(at)) => tally.excused[at] += 1,
                        _ => tally.git.push(format!(
                            "the index entry at `{path}`, in the checkout at `{place}`, is \
                             not what it was, and no line stages that path"
                        )),
                    }
                }
            }
            (was, is) if was == is => {}
            _ => tally
                .git
                .push(format!("the checkout at `{place}` is not what it was")),
        }
    }

    let (classification, why) = if !acted {
        let changed: Vec<String> = [&tally.lost, &tally.git, &tally.outside]
            .into_iter()
            .flatten()
            .cloned()
            .collect();
        if !changed.is_empty() {
            (Classification::RefusedAndChanged, changed)
        } else if seen.listed == Listed::Acts {
            (
                Classification::NotTheListedOutcome,
                vec!["the cell is listed as acting, and the command refused".to_string()],
            )
        } else {
            (Classification::RefusesNothingChanged, Vec::new())
        }
    } else if !tally.lost.is_empty() {
        (Classification::Lost, tally.lost.clone())
    } else if !tally.git.is_empty() {
        (Classification::GitMoved, tally.git.clone())
    } else if !tally.outside.is_empty() {
        (Classification::WrittenOutside, tally.outside.clone())
    } else if seen.listed == Listed::Refuses {
        (
            Classification::NotTheListedOutcome,
            vec!["the cell is listed as refusing, and the command exited 0".to_string()],
        )
    } else {
        (Classification::Holds, Vec::new())
    };
    Ok(verdict(classification, why, Some(&tally)))
}

/// **The docs staged in a work unit**, each with its id: the regular `.md` files under a
/// task's `docs/` — `.jigc/tasks/<id>/docs/`, which is where the design docs say staged
/// prose stands (`design/project-setup.md` → *Flow 2 hardening*: *an open task under
/// `.jigc/tasks/` holding a staged `docs/*.md`*) — or, where a milestone's join has built
/// its merged set, the files of that set (`.jigc/milestones/<id>/merged/docs/`), which is
/// what the boundary lands. **A stated bound:** at a milestone's boundary a sub-task's own
/// staged copy is held through the merged set and not by itself.
///
/// **The unit's commit doc is none of them.** It is staged beside the others, as
/// `commit:<id>.md` (`design/write-commands.md`: *`jigc start` itself stages
/// `commit:<id>.md`*), and it is promoted nowhere: *"The commit doc (`commit:<task-id>`)
/// is **not** promoted — its sink is the git message rendered in phase 3, not a repo
/// file"* (`design/finalize.md` → *4. Promote managed docs*). Its file goes with the
/// working area. **Not held:** that the commit's message is that doc's rendering — a
/// manifest holds no commit's message, and the rendering is the product's.
fn staged_docs(manifest: &Manifest) -> Vec<(&str, &str)> {
    let docs = |pattern: &str| -> Vec<(&str, &str)> {
        manifest
            .files
            .iter()
            .filter(|(path, _)| {
                path.ends_with(".md") && in_repo(path).is_some_and(|rel| under(pattern, rel))
            })
            .filter(|(path, _)| {
                !path
                    .rsplit('/')
                    .next()
                    .is_some_and(|name| name.starts_with("commit:"))
            })
            .filter_map(|(path, entry)| Some((path.as_str(), entry.blob()?)))
            .collect()
    };
    let merged = docs(".jigc/milestones/*/merged/docs");
    if merged.is_empty() {
        docs(".jigc/tasks/*/docs")
    } else {
        merged
    }
}

/// **What is wrong with `quote`, where its words do not stand in its doc under its
/// heading** — `None` where they do. `text_of` hands back a doc's text by its path from
/// the repository's root. The section is the heading's own: from its line to the next
/// heading of its level or above, and a `#` line inside a fenced block is no heading.
pub fn unquoted(quote: &Quote, text_of: impl Fn(&str) -> Option<&'static str>) -> Option<String> {
    let Quote {
        doc,
        heading,
        words,
    } = *quote;
    if words.trim().is_empty() {
        return Some(format!("a quote from `{doc}` holds no words"));
    }
    let Some(text) = text_of(doc) else {
        return Some(format!("`{doc}` is no doc the fence reads"));
    };
    fn level(line: &str) -> Option<(usize, &str)> {
        let depth = line.bytes().take_while(|b| *b == b'#').count();
        (depth > 0 && line[depth..].starts_with(' ')).then(|| (depth, line[depth + 1..].trim_end()))
    }
    let mut lines = text.lines();
    let Some(depth) = lines.by_ref().find_map(|line| match level(line) {
        Some((depth, title)) if title == heading => Some(depth),
        _ => None,
    }) else {
        return Some(format!("`{doc}` has no heading `{heading}`"));
    };
    let mut fenced = false;
    let section: Vec<&str> = lines
        .take_while(|line| {
            if line.trim_start().starts_with("```") {
                fenced = !fenced;
            }
            fenced || level(line).is_none_or(|(other, _)| other > depth)
        })
        .collect();
    (!section.join("\n").contains(words))
        .then(|| format!("`{doc}` → `{heading}` does not hold the words quoted from it: `{words}`"))
}

// ---------------------------------------------------------------------------
// The half that starts processes — a prepared state, the reader, the runner
// ---------------------------------------------------------------------------

/// **A prepared state**, by the name the suites know it under (the plan's §3): the
/// never-adopted repository, or one of the builder's named states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prepared {
    /// A repository with one commit that jigc was never set up in.
    Bare,
    Of(State),
}

impl Prepared {
    pub fn name(self) -> &'static str {
        match self {
            Prepared::Bare => "bare",
            Prepared::Of(state) => state.name(),
        }
    }
}

/// **A prepared state, built** — once, by the arm that runs its cells, **on the binary
/// handed in**, which is then the binary every cell of it is run on: a state is the
/// binary's own, and neither this frame nor a cell names a binary. A cell runs on a copy.
pub struct Built {
    binary: PathBuf,
    state: Prepared,
    corpus: TrialCorpus,
}

impl Built {
    pub fn on(binary: &Path, state: Prepared) -> Built {
        let corpus = match state {
            Prepared::Bare => TrialCorpus::build_never_adopted(),
            Prepared::Of(state) => TrialCorpus::build_on(binary, state),
        };
        Built {
            binary: binary.to_path_buf(),
            state,
            corpus,
        }
    }
}

/// **One thing put into a prepared state by hand** before the command runs — the user's,
/// by construction: it carries bytes found nowhere else, which the frame makes
/// ([`marker`]), and the comparison asks where those bytes stand afterwards.
///
/// The letters are the plan's (§3), which gives each kind the rule that places it: a
/// cell puts its plant among its leads where that rule says.
#[derive(Clone, Copy, Debug)]
pub enum Plant {
    /// **U**, and **D** — a file in no commit, at this path of the repository. **D** is
    /// the same act in a directory the command removes.
    File(&'static str),
    /// **T** — a file the user committed at this path of the repository: written, staged
    /// and committed in a commit of its own, by `git` as the user would run it.
    Committed(&'static str),
    /// **L**, to nothing — a symbolic link at this path of the repository whose target is
    /// not there. The link's target is where it carries its marker.
    LinkToNothing(&'static str),
    /// **E** — a line added by hand to the tracked file at this path of the repository, in
    /// no commit.
    Edit(&'static str),
    /// **W** — the user's own linked worktree, at `outside/own`: detached, holding a
    /// commit only its `HEAD` reaches and a staged file, each with a marker of its own —
    /// and, where `moved_away`, its directory moved to the sandbox's `moved/` afterwards,
    /// so that git's registration names a directory that is not there.
    Worktree { moved_away: bool },
}

/// **A condition the owner doc names for a command's guard**, which the acceptance runs
/// on both of its sides (the plan's §3): the side that is not the prepared state's own is
/// made by one of these.
#[derive(Clone, Copy, Debug)]
pub enum Condition {
    /// `HEAD` unborn: the checked-out branch's commits are taken away, and the index
    /// keeps what it held (`git update-ref -d HEAD`).
    HeadUnborn,
    /// No file-state baseline — the clone: everything under `.jigc/state/` is gone, as it
    /// is in what `git clone` hands over, the directory being ignored.
    NoBaseline,
}

/// **One thing a cell does before its command**, in order — data, like the cell.
#[derive(Clone, Copy, Debug)]
pub enum Lead {
    /// A step of the route catalog's frame, as it stands there: a lead-in on the binary
    /// under test ([`Step::Jigc`], [`Step::JigcAt`]), which must exit 0, or an act on
    /// files or through git that builds what the cell stands on.
    Step(Step),
    Plant(Plant),
    Under(Condition),
    /// What stands at this path, from the sandbox's root, is moved to the sandbox's
    /// `moved/` under the name it had.
    MovedAway(&'static str),
}

/// **A cell** — a write leaf with one argv, a prepared state with what is put into it, and
/// what the command is listed as (the plan's §2). **It names no path it expects to
/// change**: what the command may change is its leaf's row.
#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub name: &'static str,
    /// The leaf, as `cli::cli::VERB_KINDS` holds it: whose row the cell is compared under.
    pub leaf: &'static [&'static str],
    pub state: Prepared,
    pub leads: &'static [Lead],
    /// The command: the words after `jigc`. It runs the row's consent spelling where it
    /// holds the row's consent flag.
    pub argv: &'static [&'static str],
    pub listed: Listed,
    /// **The suite and the test that hold the same state**, as `suite::test` — a cell is
    /// one plant at one position, and the orderings around it stay in the suite of the
    /// command, which sweeps them (the plan's §1.5, *the rule against duplication*).
    /// `None` for a cell no suite stands beside.
    pub beside: Option<&'static str>,
}

/// What running one cell came to.
pub struct Ran {
    pub verdict: Verdict,
    /// How the command ended, in words.
    pub exit: String,
    /// The sandbox before the command, the plants in, and after it.
    pub before: Manifest,
    pub after: Manifest,
    /// What the command said and why the verdict is what it is, with the sandbox's root
    /// written `<sandbox>`.
    pub detail: String,
    /// The cell's seconds in the frame, from the copy of its state to its verdict: the
    /// sandbox, the leads, the three manifests, the command.
    pub seconds: f64,
}

/// The bytes the plant at place `at` of a cell's leads carries — `word` tells apart the
/// pieces of a plant that has more than one: found nowhere else in the sandbox, not empty,
/// and no other marker begins with them.
fn marker(at: usize, word: &str) -> String {
    format!("byte-table-plant-{at}-{word}-of-its-cell")
}

/// Where the user's own linked worktree stands, from the sandbox's root ([`Plant::Worktree`]).
const OWN_WORKTREE: &str = "outside/own";

/// One `git` child of the frame's own, as the user would run it in `at`: under the
/// sandbox's mask and the standing limit. It must exit 0.
fn git(sandbox: &Sandbox, at: &Path, args: &[&str]) -> Result<(), String> {
    let mut command = sandbox.child(Path::new("git"));
    command.current_dir(at).args(args);
    let ran = sandbox.run(&mut command, route_case::LIMIT);
    if ran.success() {
        Ok(())
    } else {
        Err(format!(
            "`git {}`: {}\n{}",
            args.join(" "),
            ran.exit(),
            ran.said
        ))
    }
}

/// The limit of a cell's command: the route catalog's frame's.
pub const LIMIT: Duration = route_case::LIMIT;

/// **Run one cell on the binary its state was built on.** `Err` is a fault of the cell as
/// written, which no classification covers — a lead-in that was refused, a plant that
/// cannot be put where the cell says, a state that is not the one handed in.
pub fn run(built: &Built, row: &Row, cell: &Cell) -> Result<Ran, String> {
    run_within(built, row, cell, LIMIT)
}

/// [`run`], with the limit the **command under test** is ended at — for the arm that holds
/// the limit itself. The lead-ins and the reader's own children keep [`LIMIT`]. Every cell
/// passes the one gate of this process that the route catalog's cases pass
/// ([`route_case::admitted`]) before its sandbox is made.
pub fn run_within(
    built: &Built,
    row: &Row,
    cell: &Cell,
    under_test: Duration,
) -> Result<Ran, String> {
    route_case::admitted(|| ran(built, row, cell, under_test))
}

/// One cell, run — inside the gate. In order: a sandbox over a copy of the prepared state;
/// a manifest; the leads; a manifest; the command; a manifest; the comparison.
fn ran(built: &Built, row: &Row, cell: &Cell, under_test: Duration) -> Result<Ran, String> {
    if cell.state != built.state {
        return Err(format!(
            "the cell stands in `{}`, and the state handed in is `{}`",
            cell.state.name(),
            built.state.name()
        ));
    }
    if cell.leaf != row.leaf {
        return Err(format!(
            "the cell is of `jigc {}`, and the row handed in is of `jigc {}`",
            cell.leaf.join(" "),
            row.leaf.join(" ")
        ));
    }
    let started = std::time::Instant::now();
    let copy = built.corpus.copy_state();
    let sandbox = Sandbox::over(cell.name, under_test, |repo, home| {
        fs::rename(copy.repo(), repo).expect("bring the prepared repository in");
        fs::rename(copy.home(), home).expect("bring the prepared home in");
    });
    let mut planted = Vec::new();
    for (at, lead) in cell.leads.iter().enumerate() {
        if let Lead::Plant(plant) = lead {
            planted.extend(plant.planted(at));
        }
    }
    let mut reader = Reader {
        sandbox: &sandbox,
        markers: planted.iter().map(|plant| plant.marker.clone()).collect(),
        commits: BTreeMap::new(),
        holding: BTreeMap::new(),
        lists: 0,
    };

    let base = reader.read()?;
    for (at, lead) in cell.leads.iter().enumerate() {
        lead.done(&sandbox, &built.binary, at)
            .map_err(|why| sandbox.shown(&why))?;
    }
    let before = if cell.leads.is_empty() {
        base.clone()
    } else {
        reader.read()?
    };
    let mut command = sandbox.child(&built.binary);
    command.args(cell.argv);
    let ended = sandbox.run(&mut command, under_test);
    let after = reader.read()?;

    let verdict = compare(
        row,
        &Seen {
            base: &base,
            before: &before,
            after: &after,
            planted: &planted,
            ended: Ended {
                ended: ended.held.is_none(),
                code: ended.code,
                said: &ended.said,
            },
            consent: row.consent.is_some_and(|flag| cell.argv.contains(&flag)),
            listed: cell.listed,
        },
    )?;
    let detail = sandbox.shown(&format!(
        "`jigc {}`: {}\n{}\n{}",
        cell.argv.join(" "),
        ended.exit(),
        ended.said.trim_end(),
        verdict.why.join("\n")
    ));
    Ok(Ran {
        verdict,
        exit: ended.exit(),
        before,
        after,
        detail,
        seconds: started.elapsed().as_secs_f64(),
    })
}

impl Plant {
    /// The plant as the comparison needs it: where each piece of it stands, from the
    /// sandbox's root, with its marker.
    fn planted(self, at: usize) -> Vec<Planted> {
        let piece = |path: String, word: &str| Planted {
            at: path,
            marker: marker(at, word),
        };
        match self {
            Plant::File(path)
            | Plant::Committed(path)
            | Plant::LinkToNothing(path)
            | Plant::Edit(path) => vec![piece(format!("{REPO}/{path}"), "only")],
            Plant::Worktree { moved_away } => {
                let stands = if moved_away {
                    "moved/own"
                } else {
                    OWN_WORKTREE
                };
                vec![
                    piece(format!("{stands}/committed-by-its-user.txt"), "committed"),
                    piece(format!("{stands}/staged-by-its-user.txt"), "staged"),
                ]
            }
        }
    }

    /// Put the plant in. Nothing stands where a file or a link is planted: one written
    /// over a file of the state would be an edit, which is another plant.
    fn put(self, sandbox: &Sandbox, at: usize) -> Result<(), String> {
        let repo = sandbox.repo();
        let free = |path: &str| -> Result<PathBuf, String> {
            let file = repo.join(path);
            if fs::symlink_metadata(&file).is_ok() {
                return Err(format!(
                    "a plant goes to `{path}`, and something stands there already"
                ));
            }
            fs::create_dir_all(file.parent().expect("a plant has a parent"))
                .map_err(|why| format!("the directory of the plant at `{path}`: {why}"))?;
            Ok(file)
        };
        let written =
            |path: &'static str| move |why: std::io::Error| format!("the plant at `{path}`: {why}");
        match self {
            Plant::File(path) => {
                fs::write(free(path)?, format!("{}\n", marker(at, "only"))).map_err(written(path))
            }
            Plant::Committed(path) => {
                fs::write(free(path)?, format!("{}\n", marker(at, "only")))
                    .map_err(written(path))?;
                git(sandbox, &repo, &["add", "--", path])?;
                git(
                    sandbox,
                    &repo,
                    &["commit", "-q", "-m", "a file of the user's own", "--", path],
                )
            }
            Plant::LinkToNothing(path) => {
                std::os::unix::fs::symlink(format!("{}.md", marker(at, "only")), free(path)?)
                    .map_err(written(path))
            }
            Plant::Edit(path) => {
                let file = repo.join(path);
                if !fs::symlink_metadata(&file).is_ok_and(|entry| entry.is_file()) {
                    return Err(format!(
                        "an edit goes into `{path}`, and no regular file stands there"
                    ));
                }
                let mut bytes = fs::read(&file).map_err(written(path))?;
                bytes.extend(format!("\n{}\n", marker(at, "only")).bytes());
                fs::write(&file, bytes).map_err(written(path))
            }
            Plant::Worktree { moved_away } => {
                let own = sandbox.root().join(OWN_WORKTREE);
                git(
                    sandbox,
                    &repo,
                    &[
                        "worktree",
                        "add",
                        "-q",
                        "--detach",
                        "../outside/own",
                        "HEAD",
                    ],
                )?;
                for (name, word, commits) in [
                    ("committed-by-its-user.txt", "committed", true),
                    ("staged-by-its-user.txt", "staged", false),
                ] {
                    fs::write(own.join(name), format!("{}\n", marker(at, word)))
                        .map_err(written(name))?;
                    git(sandbox, &own, &["add", "--", name])?;
                    if commits {
                        git(
                            sandbox,
                            &own,
                            &["commit", "-q", "-m", "the user's own commit"],
                        )?;
                    }
                }
                if moved_away {
                    Lead::MovedAway(OWN_WORKTREE).done(sandbox, Path::new(""), at)?;
                }
                Ok(())
            }
        }
    }
}

impl Condition {
    /// Make the condition hold. One that held already is a fault of the cell: its two
    /// sides would be one cell.
    fn made(self, sandbox: &Sandbox) -> Result<(), String> {
        let repo = sandbox.repo();
        match self {
            Condition::HeadUnborn => git(sandbox, &repo, &["update-ref", "-d", "HEAD"]),
            Condition::NoBaseline => {
                let state = repo.join(".jigc/state");
                let entries: Vec<PathBuf> = fs::read_dir(&state)
                    .map(|entries| entries.flatten().map(|entry| entry.path()).collect())
                    .unwrap_or_default();
                if entries.is_empty() {
                    return Err("the state holds no file-state baseline to take away".to_string());
                }
                for entry in entries {
                    let gone = if entry.is_dir() {
                        fs::remove_dir_all(&entry)
                    } else {
                        fs::remove_file(&entry)
                    };
                    gone.map_err(|why| format!("the file-state baseline: {why}"))?;
                }
                Ok(())
            }
        }
    }
}

impl Lead {
    /// Do the lead, the `at`-th of its cell. A lead-in that is refused is a fault of the
    /// cell: the route catalog's runner says so by a panic, which is caught here and
    /// handed back as the fault it is.
    fn done(self, sandbox: &Sandbox, binary: &Path, at: usize) -> Result<(), String> {
        match self {
            Lead::Step(step) => std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                sandbox.plant(binary, &[step]);
            }))
            .map_err(|panic| {
                let said = panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("a step of the cell failed");
                format!("the lead {at} of the cell was not done: {said}")
            }),
            Lead::Plant(plant) => plant.put(sandbox, at),
            Lead::Under(condition) => condition.made(sandbox),
            Lead::MovedAway(path) => {
                let from = sandbox.root().join(path);
                let name = from
                    .file_name()
                    .ok_or_else(|| format!("`{path}` names nothing to move away"))?;
                let moved = sandbox.root().join("moved");
                fs::create_dir_all(&moved)
                    .and_then(|()| fs::rename(&from, moved.join(name)))
                    .map_err(|why| format!("`{path}` could not be moved away: {why}"))
            }
        }
    }
}

/// **What takes a manifest off a sandbox** (the plan's §2, *The manifest*). The file layer
/// is read off the disk, every entry under the sandbox's root but the frame's own
/// `staging/` and the inside of a `.git` directory; git's layer is **asked of git**, by
/// children under the sandbox's mask that take no lock they can do without, so that
/// reading refreshes nothing; and of a `.git` directory the reader takes the files nobody
/// else reads — the hooks that are no samples, `info/exclude`, and the admin files of a
/// registration whose directory is gone. A file's id is the one git gives its bytes
/// **through the filters of the checkout it stands in** (`git hash-object`, by its path).
/// Modification times are read by nothing here.
struct Reader<'a> {
    sandbox: &'a Sandbox,
    /// The markers of the cell's plants.
    markers: Vec<String>,
    /// What each commit changed against its first parent. A commit does not change, so git
    /// is asked once a cell.
    commits: BTreeMap<String, Commit>,
    /// Which of the markers each blob of the store holds. A blob does not change either.
    holding: BTreeMap<String, Vec<bool>>,
    /// How many lists the reader has handed a child as its stdin.
    lists: usize,
}

/// A regular file the reader found, still to be given its id.
struct Found {
    path: PathBuf,
    mode: u32,
    bytes: Vec<u8>,
}

/// Whether `held` holds `bytes` somewhere in it.
fn holds(held: &[u8], bytes: &[u8]) -> bool {
    !bytes.is_empty() && held.windows(bytes.len()).any(|window| window == bytes)
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// A mode as git prints it — octal — or `0` where it is none.
fn mode_of(printed: &str) -> u32 {
    u32::from_str_radix(printed, 8).unwrap_or(0)
}

impl Reader<'_> {
    /// `path` from the sandbox's root, as a manifest names it.
    fn named(&self, path: &Path) -> String {
        let path = match path
            .parent()
            .and_then(|parent| fs::canonicalize(parent).ok())
        {
            Some(parent) => parent.join(path.file_name().unwrap_or_default()),
            None => path.to_path_buf(),
        };
        path.strip_prefix(self.sandbox.root())
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned()
    }

    /// One `git` child of the reader's, started in `at`: its exit code and its stdout.
    /// `fed` is its stdin; `index` is the index file it reads in place of the checkout's.
    fn git(
        &mut self,
        at: &Path,
        args: &[&str],
        fed: Option<&[u8]>,
        index: Option<&Path>,
    ) -> Result<(Option<i32>, Vec<u8>), String> {
        let mut command = self.sandbox.child(Path::new("git"));
        command
            .current_dir(at)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .args(args);
        if let Some(index) = index {
            command.env("GIT_INDEX_FILE", index);
        }
        if let Some(bytes) = fed {
            let list = self
                .sandbox
                .staging()
                .join(format!("reader-{}.in", self.lists));
            self.lists += 1;
            let opened = fs::write(&list, bytes).and_then(|()| fs::File::open(&list));
            command.stdin(opened.map_err(|why| format!("the reader's list: {why}"))?);
        }
        let ran = self.sandbox.run(&mut command, route_case::LIMIT);
        if ran.held.is_some() {
            return Err(format!(
                "the reader's `git {}` {}",
                args.join(" "),
                ran.exit()
            ));
        }
        Ok((ran.code, ran.out))
    }

    /// [`Reader::git`], for a question git must answer: its stdout, or the fault.
    fn asked(
        &mut self,
        at: &Path,
        args: &[&str],
        fed: Option<&[u8]>,
        index: Option<&Path>,
    ) -> Result<Vec<u8>, String> {
        match self.git(at, args, fed, index)? {
            (Some(0), out) => Ok(out),
            (code, out) => Err(self.sandbox.shown(&format!(
                "the reader's `git {}` in `{}` came to {code:?}: {}",
                args.join(" "),
                at.display(),
                text(&out)
            ))),
        }
    }

    /// The file layer under `dir`: every entry by its path from the sandbox's root, a
    /// regular file set aside to be given its id. A `.git` directory is an entry and is
    /// not walked; `staging/`, under the root, is the frame's own and is no entry.
    fn walk(&self, dir: &Path, manifest: &mut Manifest, found: &mut Vec<Found>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths {
            if dir == self.sandbox.root() && path == self.sandbox.staging() {
                continue;
            }
            let Ok(entry) = fs::symlink_metadata(&path) else {
                continue;
            };
            let name = self.named(&path);
            let mode = entry.permissions().mode() & 0o7777;
            if entry.file_type().is_symlink() {
                let target = fs::read_link(&path).unwrap_or_default();
                manifest
                    .files
                    .insert(name, Entry::Link(target.to_string_lossy().into_owned()));
            } else if entry.is_dir() {
                manifest.files.insert(name, Entry::Dir(mode));
                if path.file_name().is_none_or(|name| name != ".git") {
                    self.walk(&path, manifest, found);
                }
            } else if entry.is_file() {
                found.push(Found {
                    bytes: fs::read(&path).unwrap_or_default(),
                    path,
                    mode,
                });
            } else {
                manifest.files.insert(name, Entry::Special);
            }
        }
    }

    /// A regular file at `path`, set aside to be given its id — where one stands there.
    fn file(path: &Path, found: &mut Vec<Found>) {
        if let Ok(entry) = fs::symlink_metadata(path)
            && entry.is_file()
        {
            found.push(Found {
                bytes: fs::read(path).unwrap_or_default(),
                path: path.to_path_buf(),
                mode: entry.permissions().mode() & 0o7777,
            });
        }
    }

    /// The regular files under `dir`, however deep, set aside.
    fn files_under(dir: &Path, found: &mut Vec<Found>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths {
            if fs::symlink_metadata(&path).is_ok_and(|entry| entry.is_dir()) {
                Self::files_under(&path, found);
            } else {
                Self::file(&path, found);
            }
        }
    }

    /// One checkout, asked of git in its directory: its `HEAD`, its branch, its index.
    fn checkout(&mut self, dir: &Path) -> Result<Checkout, String> {
        let head = match self.git(
            dir,
            &["rev-parse", "-q", "--verify", "HEAD^{commit}"],
            None,
            None,
        )? {
            (Some(0), out) => text(&out).trim().to_string(),
            _ => String::new(),
        };
        let branch = match self.git(dir, &["symbolic-ref", "-q", "--short", "HEAD"], None, None)? {
            (Some(0), out) => Some(text(&out).trim().to_string()),
            _ => None,
        };
        let listed = self.asked(dir, &["ls-files", "-s", "-v", "-z"], None, None)?;
        Ok(Checkout {
            head,
            branch,
            index: index_of(&listed, true),
        })
    }

    /// **Take the sandbox's picture.**
    fn read(&mut self) -> Result<Manifest, String> {
        let (root, repo) = (self.sandbox.root().to_path_buf(), self.sandbox.repo());
        let mut manifest = Manifest::default();
        let mut found = Vec::new();
        self.walk(&root, &mut manifest, &mut found);

        let git_dir = repo.join(".git");
        // The checkouts whose directory is there, the main one first.
        let mut checkouts: Vec<PathBuf> = Vec::new();
        if git_dir.is_dir() {
            checkouts.push(repo.clone());
            // The files of git's that nobody else reads.
            let hooks =
                text(&self.asked(&repo, &["rev-parse", "--git-path", "hooks"], None, None)?);
            let hooks = repo.join(hooks.trim());
            manifest.hooks = self.named(&hooks);
            if let Ok(entries) = fs::read_dir(&hooks) {
                let mut paths: Vec<PathBuf> = entries.flatten().map(|entry| entry.path()).collect();
                paths.sort();
                for path in paths {
                    if path.extension().is_none_or(|kind| kind != "sample") {
                        Self::file(&path, &mut found);
                    }
                }
            }
            Self::file(&git_dir.join("info/exclude"), &mut found);

            // The registrations, as the admin directory names them — which is what git
            // lists from. A registration whose directory is gone is read from its admin
            // files: no child runs in a directory that is not there.
            let mut roots: Vec<String> = Vec::new();
            let mut held: BTreeSet<String> = BTreeSet::new();
            let mut names: Vec<PathBuf> = fs::read_dir(git_dir.join("worktrees"))
                .map(|entries| entries.flatten().map(|entry| entry.path()).collect())
                .unwrap_or_default();
            names.sort();
            for admin in names {
                let Ok(points_at) = fs::read_to_string(admin.join("gitdir")) else {
                    continue;
                };
                let dir = Path::new(points_at.trim())
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_default();
                let name = admin
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                manifest.registrations.insert(name, self.named(&dir));
                if dir.is_dir() {
                    checkouts.push(dir);
                    continue;
                }
                Self::file(&admin.join("HEAD"), &mut found);
                Self::file(&admin.join("index"), &mut found);
                Self::files_under(&admin.join("logs"), &mut found);
                let head = fs::read_to_string(admin.join("HEAD")).unwrap_or_default();
                if !head.starts_with("ref:") && !head.trim().is_empty() {
                    roots.push(head.trim().to_string());
                }
                if admin.join("index").is_file() {
                    let listed = self.asked(
                        &repo,
                        &["ls-files", "-s", "-z"],
                        None,
                        Some(&admin.join("index")),
                    )?;
                    held.extend(
                        index_of(&listed, false)
                            .into_values()
                            .map(|staged| staged.blob),
                    );
                }
            }
            for dir in &checkouts {
                let checkout = self.checkout(dir)?;
                if !checkout.head.is_empty() {
                    roots.push(checkout.head.clone());
                }
                held.extend(checkout.index.values().map(|staged| staged.blob.clone()));
                manifest.checkouts.insert(self.named(dir), checkout);
            }

            // The rest of git's layer.
            for line in text(&self.asked(
                &repo,
                &["for-each-ref", "--format=%(refname) %(objectname)"],
                None,
                None,
            )?)
            .lines()
            {
                if let Some((name, id)) = line.split_once(' ') {
                    manifest.refs.insert(name.to_string(), id.to_string());
                }
            }
            manifest.stash =
                text(&self.asked(&repo, &["stash", "list", "--format=%H"], None, None)?)
                    .lines()
                    .map(str::to_string)
                    .collect();
            manifest.config =
                text(&self.asked(&repo, &["config", "--local", "--list"], None, None)?);
            let mut blobs: Vec<String> = Vec::new();
            for line in text(&self.asked(
                &repo,
                &[
                    "cat-file",
                    "--batch-all-objects",
                    "--batch-check=%(objectname) %(objecttype)",
                ],
                None,
                None,
            )?)
            .lines()
            {
                if let Some((id, kind)) = line.split_once(' ') {
                    manifest.objects.insert(id.to_string());
                    if kind == "blob" {
                        blobs.push(id.to_string());
                    }
                }
            }

            // What is held, under the narrowest reading: what a ref, a registered
            // checkout's `HEAD` or its index, or the newest stash entry reaches.
            let mut reach = vec!["rev-list", "--objects", "--all"];
            reach.extend(roots.iter().map(String::as_str));
            for line in text(&self.asked(&repo, &reach, None, None)?).lines() {
                held.insert(line.split(' ').next().unwrap_or_default().to_string());
            }
            manifest.held = held;
            let mut within = vec!["rev-list", "--parents", "--all"];
            within.extend(roots.iter().map(String::as_str));
            for line in text(&self.asked(&repo, &within, None, None)?).lines() {
                let mut ids = line.split(' ');
                let (Some(id), parent) = (ids.next(), ids.next()) else {
                    continue;
                };
                if !self.commits.contains_key(id) {
                    let changed = match parent {
                        Some(parent) => self.asked(
                            &repo,
                            &["diff-tree", "-r", "--raw", "--no-renames", "-z", parent, id],
                            None,
                            None,
                        )?,
                        None => self.asked(
                            &repo,
                            &[
                                "diff-tree",
                                "-r",
                                "--raw",
                                "--no-renames",
                                "-z",
                                "--root",
                                "--no-commit-id",
                                id,
                            ],
                            None,
                            None,
                        )?,
                    };
                    self.commits.insert(
                        id.to_string(),
                        Commit {
                            parent: parent.map(str::to_string),
                            changed: changed_of(&changed),
                        },
                    );
                }
                manifest
                    .commits
                    .insert(id.to_string(), self.commits[id].clone());
            }

            // Which blobs of the store hold a marker.
            let unread: Vec<String> = blobs
                .iter()
                .filter(|id| !self.markers.is_empty() && !self.holding.contains_key(*id))
                .cloned()
                .collect();
            if !unread.is_empty() {
                let list = unread.join("\n") + "\n";
                let out =
                    self.asked(&repo, &["cat-file", "--batch"], Some(list.as_bytes()), None)?;
                let mut rest = &out[..];
                for id in &unread {
                    let read = (|| {
                        let end = rest.iter().position(|byte| *byte == b'\n')?;
                        let size: usize = text(&rest[..end]).rsplit(' ').next()?.parse().ok()?;
                        let body = rest.get(end + 1..end + 1 + size)?;
                        rest = rest.get(end + 2 + size..).unwrap_or_default();
                        Some(body)
                    })();
                    let Some(body) = read else {
                        return Err(format!("git handed the reader no blob `{id}`"));
                    };
                    let has = self
                        .markers
                        .iter()
                        .map(|marker| holds(body, marker.as_bytes()))
                        .collect();
                    self.holding.insert(id.clone(), has);
                }
            }
        }

        // Every file its id, through the filters of the checkout it stands in; a file that
        // stands in none, or inside a `.git` directory, as its bytes are.
        let mut groups: BTreeMap<Option<PathBuf>, Vec<usize>> = BTreeMap::new();
        for (at, file) in found.iter().enumerate() {
            let of_gits = file
                .path
                .strip_prefix(&root)
                .is_ok_and(|rel| rel.components().any(|part| part.as_os_str() == ".git"));
            let checkout = checkouts
                .iter()
                .filter(|dir| !of_gits && file.path.starts_with(dir))
                .max_by_key(|dir| dir.components().count());
            groups.entry(checkout.cloned()).or_default().push(at);
        }
        let mut ids: Vec<String> = vec![String::new(); found.len()];
        for (checkout, files) in groups {
            let mut list = String::new();
            for at in &files {
                let path = match &checkout {
                    Some(dir) => found[*at]
                        .path
                        .strip_prefix(dir)
                        .unwrap_or(&found[*at].path),
                    None => &found[*at].path,
                };
                let path = path.to_string_lossy();
                if path.contains('\n') {
                    return Err(format!(
                        "`{path}` has a line break in its name, and git is handed names by the line"
                    ));
                }
                list.push_str(&path);
                list.push('\n');
            }
            let out = match &checkout {
                Some(dir) => self.asked(
                    dir,
                    &["hash-object", "--stdin-paths"],
                    Some(list.as_bytes()),
                    None,
                )?,
                None => self.asked(
                    &root,
                    &["hash-object", "--no-filters", "--stdin-paths"],
                    Some(list.as_bytes()),
                    None,
                )?,
            };
            let out = text(&out);
            let mut given = out.lines();
            for at in files {
                ids[at] = given
                    .next()
                    .ok_or_else(|| "git gave the reader fewer ids than files".to_string())?
                    .to_string();
            }
        }
        for (file, blob) in found.into_iter().zip(ids) {
            manifest.files.insert(
                self.named(&file.path),
                Entry::File {
                    mode: file.mode,
                    blob,
                    bytes: file.bytes,
                },
            );
        }

        // Where each plant's marker stands.
        for (at, marker) in self.markers.iter().enumerate() {
            let files = manifest
                .files
                .iter()
                .filter(|(_, entry)| match entry {
                    Entry::File { bytes, .. } => holds(bytes, marker.as_bytes()),
                    Entry::Link(target) => holds(target.as_bytes(), marker.as_bytes()),
                    _ => false,
                })
                .map(|(path, _)| path.clone())
                .collect();
            let has = |id: &String| self.holding.get(id).is_some_and(|has| has[at]);
            manifest.markers.insert(
                marker.clone(),
                Stands {
                    files,
                    held: manifest.held.iter().any(has),
                    stored: manifest.objects.iter().any(has),
                },
            );
        }
        Ok(manifest)
    }
}

/// An index as `git ls-files -s -z` lists it, by path — with `-v`, each entry opens with
/// the tag git prints for its flags, which is then kept with its stage.
fn index_of(listed: &[u8], tagged: bool) -> BTreeMap<String, Staged> {
    let mut index = BTreeMap::new();
    for entry in listed
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
    {
        let entry = text(entry);
        let Some((meta, path)) = entry.split_once('\t') else {
            continue;
        };
        let mut meta = meta.split(' ');
        let tag = if tagged {
            meta.next().unwrap_or_default()
        } else {
            ""
        };
        let (mode, blob, stage) = (
            meta.next().unwrap_or_default(),
            meta.next().unwrap_or_default(),
            meta.next().unwrap_or_default(),
        );
        index.insert(
            path.to_string(),
            Staged {
                mode: mode_of(mode),
                blob: blob.to_string(),
                flags: format!("{tag} {stage}").trim().to_string(),
            },
        );
    }
    index
}

/// What a commit changed, as `git diff-tree -r --raw -z` prints it: a path to the mode and
/// blob it holds there now, or to nothing where the path was removed.
fn changed_of(raw: &[u8]) -> BTreeMap<String, Option<(u32, String)>> {
    let mut changed = BTreeMap::new();
    let mut pieces = raw
        .split(|byte| *byte == 0)
        .filter(|piece| !piece.is_empty());
    while let (Some(meta), Some(path)) = (pieces.next(), pieces.next()) {
        let meta = text(meta);
        let fields: Vec<&str> = meta.trim_start_matches(':').split(' ').collect();
        let [_, mode, _, blob, status] = fields[..] else {
            continue;
        };
        changed.insert(
            text(path),
            (status != "D").then(|| (mode_of(mode), blob.to_string())),
        );
    }
    changed
}
