//! **A byte cell** — the frame of the byte table (`crates/cli/tests/byte_table.rs`; the
//! plan is `completions/artifacts/M55/stabilization-build/reshape/byte-table-plan.md`, and
//! this file is its task `B1`).
//!
//! The exit rule's first clause is about lost files: *no jigc command at exit 0 destroys
//! bytes no git object holds or commits content the user did not ask for*, and *where jigc
//! cannot tell it refuses before writing* (`DECISIONS.md` → *2026-10-04 — The exit rule,
//! revised*). The byte table runs one writing command between two pictures of a sandbox
//! and says what the difference is. This file is the half of it that starts no process:
//! **the picture as data** ([`Manifest`]), **what a command may change, as its design doc
//! declares it** ([`Row`], [`Line`]), and **the comparison** ([`compare`]) — a function
//! from two manifests, how the command ended and a row to one of eight
//! [`Classification`]s. What takes a manifest off a sandbox, and what runs the command, is
//! the plan's task `B2b`; until then nothing here can run a child, and nothing here names a
//! binary.
//!
//! **A row is written from the docs, never from what the command did.** A line of a row is
//! a place, what may happen there, when, and whether it holds when the command refuses —
//! and **the sentence of the design doc it rests on, quoted** ([`Quote`]). The quote is a
//! run of the doc's own bytes under a named heading, and [`unquoted`] holds it to the
//! doc's text: a doc that moves reddens the row. The vocabulary — [`Where`], [`May`],
//! [`Holding`], [`When`] — is what the four rows of the pilot's acceptance needed and no
//! more (`…/reshape/byte-table-acceptance-rows.md` → §7, which says for each word which
//! lines needed it).
//!
//! **Whose bytes they are is settled by construction, never by a list copied out of the
//! product.** A planted entry is the user's ([`Planted`]): where its bytes stand nowhere
//! after exit 0 the cell is *lost* **whatever the row declares** — only the consent
//! spelling is exempt, and only where the command's output names the planted path. What
//! the binary under test wrote in a cell's lead-ins is the product's own, read off a
//! manifest taken before them ([`Seen::base`]): a difference there is judged against the
//! row's lines and is never *lost*. And prose handed to jigc — a doc staged under a work
//! unit's `docs/` — must, once its working area is gone, stand byte for byte in a commit
//! the command made, as a regular file.
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

/// A condition a line carries, read off the manifest taken before the command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum When {
    Always,
    /// The repository has no commit: its `HEAD` is unborn.
    NoCommitYet,
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
fn staged_docs(manifest: &Manifest) -> Vec<(&str, &str)> {
    let docs = |pattern: &str| -> Vec<(&str, &str)> {
        manifest
            .files
            .iter()
            .filter(|(path, _)| {
                path.ends_with(".md") && in_repo(path).is_some_and(|rel| under(pattern, rel))
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
