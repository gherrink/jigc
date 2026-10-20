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
//! **What *held* means is the human's ruling of 2026-10-14** (the plan's §7, question 1,
//! reading (b)): bytes are held where a ref, a registered checkout's `HEAD`, an index, the
//! newest stash entry or any reflog reaches an object that has them ([`Manifest::held`]).
//! The bare object store is not held: bytes that only it has are lost.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::leaf_argv::MINIMAL_ARGV;
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

/// Where a plant's marker stands: the files of the sandbox that hold it, and whether an
/// object within reach does ([`Manifest::held`]'s reach).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stands {
    pub files: BTreeSet<String>,
    pub held: bool,
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
    /// **The ids held**: what a ref, a registered checkout's `HEAD`, an index, the newest
    /// stash entry or any reflog reaches — the human's ruling of 2026-10-14 on what
    /// *held* means. The bare object store is not held.
    pub held: BTreeSet<String>,
    /// The commits within that reach, by id.
    pub commits: BTreeMap<String, Commit>,
    /// **The bytes of every regular file a commit within reach changed a path to**, by the
    /// id of its blob — what a line that merges into a file asks of a commit the command
    /// made before its last ([`May::MergeInto`]): `add-from-spec`'s *"commits the record
    /// once per sub-task"*. Added 2026-10-19, after the review of the task `B7`'s second
    /// part (its tenth finding).
    pub blobs: BTreeMap<String, Vec<u8>>,
    /// Where each plant's marker stands.
    pub markers: BTreeMap<String, Stands>,
}

/// **One thing put into a prepared state by hand before the command runs**, as the
/// comparison needs it: where it stands, from the sandbox's root, and the bytes found
/// nowhere else that it carries. A plant with no bytes is a fault of the cell — an empty
/// file's blob is the one every empty file has, and would read as held.
///
/// **A staged piece** stands in the main checkout's index at its path and in no file: it
/// is the user's as any plant is, and it stands for as long as an object within reach has
/// its bytes — the index entry it was planted as, or a commit that took the entry in.
#[derive(Clone, Debug)]
pub struct Planted {
    pub at: String,
    pub marker: String,
    /// Whether the piece stands in the index only.
    pub staged: bool,
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
/// and a `*` in it stands for one component — any work unit's id. **Three words of a
/// component stand for a unit the command acts on, and no other** ([`Unit`]), each alone or
/// with a fixed suffix (`<unit>.md`): [`OWN`], the unit the command names — a task's area is
/// `.jigc/tasks/<unit>`, never a sibling's; [`ITS_SUB_TASKS`], any sub-task the committed
/// record of that unit names, which is a milestone's — so a line of a milestone's row
/// reaches its own sub-tasks' areas and no other task's; and [`ITS_MILESTONE`], the
/// milestone whose committed record names the unit as one of its tasks. **A place may open
/// with [`DOCS_ROOT`]**, the directory the cascade resolves `docs-root` to. And a fourth
/// stands for a unit the command mints and does not name by its id ([`MINTED`], or
/// [`EACH_MINTED`] where it mints several). Two more
/// stand for a sub-task of the unit where a thing of it did not stand: [`RE_SEEDED`], a
/// live one whose area did not, and [`UNPROVISIONED`], one whose worktree no checkout was.
#[derive(Clone, Copy, Debug)]
pub enum Where {
    /// One path.
    Path(&'static str),
    /// A directory and everything under it.
    Under(&'static str),
    /// The `pre-commit` hook in the hooks directory git resolves — no fixed path.
    Hook,
    /// Wherever a staged doc of the unit lands: the path is the product's to compute, so
    /// the place is known by the bytes that arrive there ([`May::CopyOfAStagedDoc`]). Under
    /// [`May::CopiedFrom`] it is the same home seen before the task holds the doc: where
    /// the committed doc a write copies in stands.
    AStagedDocsHome,
}

/// **The component of a place that stands for the work unit the command acts on** — the id
/// its row's [`Unit`] reads off the command. Where the command names no unit, a place that
/// holds it is nowhere.
pub const OWN: &str = "<unit>";

/// **The component of a place that stands for any sub-task of the unit the command acts
/// on**: an id the unit's committed record names as one of its tasks — a `### <id>  {#<id>}`
/// item of `<docs-root>/milestone-records/<unit>.md`, the record `design/storage.md` →
/// *Source of truth* makes the milestone's source of truth (*"its ordered sub-task list"*),
/// read as `HEAD` holds it: *"The committed `.md` record is the **source of truth**"*
/// (`design/team-ready-state.md` → *Engine capability 2 (read-back)*). A unit no record
/// names has no sub-task, and neither has a record whose bytes in the working tree are not
/// the ones `HEAD` holds — the stricter direction, since the product reads the file and
/// refuses one edited out of band.
pub const ITS_SUB_TASKS: &str = "<sub-task>";

/// **The component of a place that stands for the milestone of the unit the command acts
/// on**: the one milestone whose committed record names the unit as one of its tasks — *"The
/// owning milestone is resolved from **what the committed record names**, never from the
/// gitignored `.jigc/` cache"* (`design/team-ready-state.md` → *`jigc task discard
/// <sub-id>` — the per-sub-task terminal (M49)*). A unit no record names, or two records
/// name, has none.
pub const ITS_MILESTONE: &str = "<its-milestone>";

/// **The component of a place that stands for the work unit the command mints**: the one id
/// the command minted — the committed record names it after the command and did not before
/// — at which nothing stood before the command, the path up to and with this component
/// holding no entry in the manifest taken before it, and nothing at or under it in what git
/// held there, the tree the main checkout's `HEAD` held and its index. A mint makes what was
/// not there and touches nothing that was: *"both mints still refuse a pre-existing id
/// before creating anything, which is exactly what makes removing what this call minted
/// safe"* (`design/team-ready-state.md` → *The commit model — path-scoped commit at each
/// milestone op*), and *"a home with nothing on disk is a free id only when git holds
/// nothing there either"* (→ *The terminal is terminal — a settled record has no workbench
/// (M42 completion audit)*, the rc.24 fix pass's CPL-5).
///
/// It is the word for a unit the command's argv does not name by its id — the milestone
/// `jigc milestone create "<title>"` mints, whose id is the title's slug, and the sub-task
/// `add-task` mints from an intent. **The id is read off the record, as `HEAD` holds it
/// after the command**, beside the record before it: for a command that acts on a unit,
/// an item its record names and did not ([`ITS_SUB_TASKS`]'s reading of it) — *"Appends
/// one `tasks` item"*; for one that acts on none, a record at the home `HEAD` holds and did
/// not — *"**`create` / `add-task`** — a **separate** record-only path-scoped commit each"*.
/// **And it is one**: where the command minted two, the place is nowhere — `add-task`
/// *"mints a sub-task"*, `create` one milestone. A door that mints one per something says
/// so with [`EACH_MINTED`].
///
/// Narrowed 2026-10-19, after the review of the task `B7`'s second part, its eighth finding:
/// the word asked the working tree alone, so a record deleted from disk that `HEAD` still
/// held counted as nothing stood, and it could not tell one mint from two — an area at an
/// id no record gained, a second area beside the one it did, and an area made for a
/// sub-task the record already named and had settled, each held.
pub const MINTED: &str = "<minted>";

/// **The component of a place that stands for any of the work units the command mints**:
/// [`MINTED`]'s reading, of each id the command minted however many it minted — the
/// sub-tasks `add-from-spec` mints *"one sub-task per criterion"* of a committed spec
/// (`design/write-commands.md` → *Minting a milestone + its task list (M7)*). **What it
/// cannot tell apart**: how many the door should have minted — the frame does not count a
/// spec's criteria.
pub const EACH_MINTED: &str = "<each-minted>";

/// **The component of a place that stands for the task the command mints** — a task, by
/// its area: the one id at which an area under `.jigc/tasks/` carries its base pin after
/// the command, where nothing stood before it in the working tree or in what git held, the
/// path up to and with this component holding no entry — *"A task is an area that carries
/// its base pin"* (`design/storage.md` → *The per-task working area (staging)*), and *"the
/// CLI mints the *id* and opens the area"* (`design/write-commands.md` → *Task
/// origination*). **And it is one**: where two areas came to carry a pin, the place is
/// nowhere — each door mints one task. [`MINTED`]'s reading of a mint is the committed
/// record's, and a task has no record: `jigc start`, `jigc migrate` and `jigc task amend`
/// mint a task and commit nothing. Added 2026-10-20, with the mint family's rows (the
/// task `B9`).
pub const MINTED_TASK: &str = "<minted-task>";

/// **The component of a place that stands for a sub-task whose area the re-seed rebuilds**:
/// a sub-task the committed record of the unit the command acts on names
/// ([`ITS_SUB_TASKS`]), whose item that record has not settled, and at which nothing stood
/// before the command — the path up to and with this component held no entry in the
/// manifest taken before it, as under [`MINTED`]. *"one standard working area per sub-task
/// the record names"*, and *"A sub-task whose area already exists is skipped untouched, so a
/// live session is never clobbered — **and so is one the record has settled**"*
/// (`design/team-ready-state.md` → *Engine capability 2 (read-back)*): the re-seed makes
/// an area where none stood and touches none that did. An item is settled where its
/// `status` leaf is `joined` or `discarded`. Added 2026-10-19, after the review of the task
/// `B7`'s second part, its second finding: the re-seed's lines read [`ITS_SUB_TASKS`], so a
/// rewrite of a standing sub-task's staged prose or base pin held.
pub const RE_SEEDED: &str = "<re-seeded>";

/// **The component of a place that stands for a sub-task whose worktree `provision` adds**:
/// a sub-task the committed record of the unit the command acts on names
/// ([`ITS_SUB_TASKS`]), at whose path no linked checkout stood before the command — *"a
/// **registered** worktree is reused untouched and never probed at all"*
/// (`design/team-ready-state.md` → *`jigc milestone discard <id>` — the record-settling
/// op*). A directory that stood there and is no checkout is a leftover the door clears
/// before it adds the worktree — *"`provision` (it clears a leftover before `git worktree
/// add`)"*, the same section — so the place stands for it: only a checkout is reused, and
/// only a checkout is left untouched. The same review, the same finding: `MP1` read
/// [`ITS_SUB_TASKS`], so a rewrite inside a live sub-task's worktree held. **And whose item
/// the record has not settled** — read so 2026-10-19 under the record's §6, entry 20, then
/// open, and ruled the same day by the human: `milestone provision` skips sub-tasks the
/// record has settled, as the re-seed does, and the milestone design is to say so (the
/// sentence that names the case narrates it as the defect the abandon's teardown was built
/// against, *"`milestone provision` will happily re-provision a worktree for a sub-task that
/// was discarded"*).
pub const UNPROVISIONED: &str = "<unprovisioned>";

/// **The leading component of a place that stands for the directory the cascade resolves
/// `docs-root` to** — *"The resolved `docs-root` is prepended to every persisted doctype's
/// `location:` at schema load"* (`design/storage.md` → *Config layout*): the project
/// layer's value (`.jigc/config/manifest.yaml`), else the team layer's
/// (`~/.config/jigc/manifest.yaml`), else the dev pack's default; `.` is the repository's
/// root. Where a layer that stands does not read, the place is nowhere.
pub const DOCS_ROOT: &str = "<docs-root>";

/// **A whole place that is a doc of the store, at its doctype's current home** — `<slug>.md`
/// directly in a doctype's home directory, or a placement doctype's one file — as the
/// cascade resolves the homes before the command or after it ([`Homes::over`]; a root
/// knob's re-point moves the homes): *"The resolved
/// `docs-root` is prepended to every persisted doctype's `location:` at schema load"*
/// (`design/storage.md` → *Config layout*). A path two doctypes' homes would hold is no
/// doc's. Added 2026-10-19, for the movers family's rows (the task `B8`): every door of the
/// family writes the committed store where it stands.
pub const A_DOC: &str = "<a-doc>";

/// **A whole place that is a doc at a home its doctype had at an earlier schema version**,
/// and no doctype's current home — what a snapshot of the schema declares
/// (`design/corpus-migration.md` → *Prior-schema sourcing — the versioned snapshot store*).
pub const A_PRIOR_DOC: &str = "<a-prior-doc>";

/// **The doc the command's address names, at its home** — the unit, `<type>:<slug>`, the
/// first word after the leaf: `<slug>.md` at the doctype's home directory, or a placement
/// doctype's file.
pub const THE_DOC: &str = "<the-doc>";

/// **Where the doc the command's address names stands once its identity has moved** — the
/// unit's doctype's home, under the slug the command gives: `--slug`'s, where it names one,
/// and else the one `--to`'s title gives, *"**Derives the new slug** = `slugify("<New
/// Title>")` (the same `engine::slug::slugify` the `create` path uses via `id-from: title`,
/// so a renamed doc is byte-identical to a freshly-created one of that title), unless
/// `--slug` overrides"* (`design/write-commands.md` → *`jigc rename`*) — `<slug>.md` at a
/// home directory, or a placement doctype's file, whose identity does not move. Where that
/// slug is the doc's own, the place is [`THE_DOC`]'s. Added 2026-10-20, after the review of
/// the task `B8` (H7, H11): the arrival was read inside the word, at no place, so nothing
/// that squats a rename's destination was a need, and any free path of the home directory
/// was taken for it.
pub const THE_NEW_DOC: &str = "<the-new-doc>";

/// **The prior home the command's `--from` names** — that file, or any path under that
/// directory: *"the door sweeps every committed `.md` under it into the doctype's current
/// home"* (`design/corpus-migration.md` → *The freeze-exempt sibling*).
pub const PRIOR_HOME: &str = "<prior-home>";

/// **The path the command's first argument names** — the source `jigc migrate <path> --as
/// <doctype>` reads, the first word after the leaf, from the repository's root (a leading
/// `./` read away): *"Reads the foreign file's bytes"* (`design/auto-migration.md` → *The
/// `jigc migrate` verb (the trigger)*). It names where the plants a line of it is aimed at
/// stand; its one line lets nothing happen there ([`May::CopiedFrom`]), so the comparison
/// reads no path for it, and a difference at the source is outside every line. Added
/// 2026-10-20, with the mint family's rows (the task `B9`).
pub const THE_SOURCE: &str = "<the-source>";

/// **The foreign original the unit's migration recorded** — the path the unit's area holds
/// in `.jigc/tasks/<unit>/source-path` before the command, from the repository's root: the
/// file a migration task's `jigc task finalize <id> --approve` retires — *"`jigc task
/// finalize <id> --approve` proceeds through the retire + commit + adopt"*
/// (`design/auto-migration.md` → *The review gate (fidelity)*). Nothing where the unit's area
/// records no source. Added 2026-10-20, after the review of the task `B9` (H6): the task
/// family's row had no line for the retire, so no plant could be aimed where it deletes.
pub const THE_RETIRED: &str = "<the-retired>";

/// **Where a move into a doctype's home lands a doc** — a mover's destination: for a doc that
/// stood before the command where a move into its home ([`May::IntoItsHome`]) takes it — a
/// `<name>.md` under the prior home `--from` names, of the doctype the command names, or a
/// doc at a doctype's home whose home the command moves — the path its name stands at in
/// that doctype's home as the cascade resolves it **after** the command, where that is
/// another path. *"the door sweeps every committed `.md` under it into the doctype's current
/// home"* (`design/corpus-migration.md` → *The freeze-exempt sibling*), *"`config set
/// docs-root` now detects + routes + **auto-moves** each stranded instance"*
/// (`design/storage.md` → *Placement*). What stands there before the command blocks the
/// move — the human's ruling of 2026-10-20 on the rows' record's §6, entry 23: **any file
/// at a mover's destination refuses**, and nothing is moved aside from one. Added
/// 2026-10-20, the task `B9`'s first commit: a destination was no place, and its occupant
/// was what the parking lines `RL3` and `CS3` were for.
pub const A_NEW_HOME: &str = "<a-new-home>";

/// Where a milestone's committed record stands under the docs root: `milestone-records/`,
/// the `location:` the `milestone-record` doctype declares.
const RECORD_HOME: &str = "milestone-records";

/// The dev pack's knob declarations, whose `docs-root` default stands where no layer sets
/// the knob.
const DEV_KNOBS: &str = include_str!("../../packs/dev/config/knobs.yaml");

/// **How a row's command names the work unit it acts on** — read off its argv, as the
/// command's own grammar has it, so that a line written with [`OWN`] or [`ITS_SUB_TASKS`]
/// reaches that unit's area and no sibling's: *"a sub-agent physically cannot stage into a
/// sibling's area"* (`design/write-commands.md` → *The write-time `--task`-scoped barrier
/// (M8)*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    /// The command acts on no work unit.
    Nothing,
    /// **`--task <id>`, and with no flag the single task that stands** — the same section:
    /// *"explicit `--task <id>` wins; else the single active task"*. A task stands where
    /// its area carries its base pin before the command — *"A task is an area that carries
    /// its base pin"*, `.jigc/tasks/<id>/base.json` a file (`design/storage.md` → *The
    /// per-task working area (staging)*); a leftover directory is no task.
    TheTaskFlag,
    /// **The first word after the leaf** — `jigc task finalize <id>`, `jigc milestone
    /// finalize <id>`.
    TheWordAfterTheLeaf,
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
    /// gone**: what a refusal may do to a cache, which is rebuilt and never taken away —
    /// and what a command that acts may do in a directory it writes into, where no
    /// sentence of its doc lets a file of that directory go.
    Rebuilt,
    /// **Rebuilt, or the stamped file — this path, from the repository's root — taken
    /// away**: a cache whose stamp a command invalidates and whose **next read** rebuilds
    /// it, so its file gone is an invalidation and no loss. The edge index's halves of
    /// `T5` and `M10`, `.jigc/index/edges.json`: *"Invalidate the edge-index stamp … The
    /// next read rebuilds from the new HEAD"* (`design/finalize.md` → *7. Post-commit
    /// (best-effort)*), the index *"stamp-rebuildable by construction"* (`design/storage.md`
    /// → *Concurrent writers*). Only that one regular file may go; every other file of the
    /// place is as under [`May::Rebuilt`], and the file-state hashes, which have *"no stamp
    /// and no rebuild path"* (→ *Derived caches*), never take this word. The review of the
    /// task `B7`'s first part, its first finding.
    Invalidated(&'static str),
    /// **A directory made, and nothing in it** — what a command lays out for a later writer
    /// and writes no file into: the workbench's cache directories `setup` lays out, by the
    /// human's ruling of 2026-10-15 on the record's §6, entry 1.
    LaidOut,
    /// Made or replaced by a regular file that is, byte for byte, a doc staged in a work
    /// unit.
    CopyOfAStagedDoc,
    /// **Provisioned** — made where nothing stood: a file or a directory at a path that held
    /// no entry before the command; nothing that stood is rewritten, and nothing is gone. A
    /// regular file or a directory and no other entry — a link or a FIFO made in the area is
    /// none (after the review of the task `B9`, H4).
    /// *"The first `jigc workflow <W> --task <id>` re-entry **provisions** the
    /// sub-workflow's deterministic instances (the commit doc, any workflow-provisioned
    /// docs) into the sub-area"* (`design/write-commands.md` → *Sub-agent re-entry*): a
    /// second re-entry finds them and writes nothing, so a staged doc a sub-agent authored
    /// is no instance to provision. `jigc workflow`'s `WF1`; added 2026-10-20, with the mint
    /// family's rows (the task `B9`).
    Provisioned,
    /// **Read, and nothing written** — the home of a committed doc a write copies into a
    /// task's working area: the place the copy is read from, which the write leaves as it
    /// stands. It lets no difference happen there, at exit 0 or on a refusal, and lets no
    /// commit or index hold anything there; what it is for is the plants put where the
    /// copy is read from, which are aimed at it ([`aimed`]). The `doc` family's `W3`
    /// (`…/reshape/byte-table-acceptance-rows.md` → §3b; the task `B6`).
    CopiedFrom,
    /// **Carried as it stands** — a place whose bytes a commit the command makes may hold,
    /// **the bytes the working tree holds there**, and which the command writes nothing at:
    /// the project config layer a finalize lands on its first commit, *"on a first commit,
    /// the project config layer"* (`design/finalize.md` → *5. Stage*) and the milestone
    /// boundary's *"overlays the promoted docs + first-commit config"* (→ *`fan-out`
    /// finalize*). It lets no difference happen at the file layer, and lets a commit or the
    /// index hold only what stands there in the working tree. Its lines stand under
    /// [`When::FirstCommitOfItsPlace`]: the word says what a commit may hold, and the
    /// condition which commit.
    Carried,
    /// **A checkout populated from the pinned base's tree** — what `git worktree add
    /// --detach` leaves at a place no checkout stood: a directory made; the checkout's own
    /// `.git` file at its root, which names its admin directory; and a regular file whose
    /// path under the checkout and whose bytes are the ones the tree of the base the unit's
    /// committed record pins holds there, its executable bit with them — *"`git worktree add
    /// --detach` populates from the pinned base's *tree*"* (`design/team-ready-state.md` →
    /// *What a fan-out worktree isolates — and what it does not (M45)*). The checkout's root
    /// is the place's directory: a line of this word names it with [`Where::Under`]. A file
    /// the base does not hold there, other bytes, a link and anything changed or gone are
    /// none of it. `MP1`; added 2026-10-19, after the review of the task `B7`'s second part
    /// (its ninth finding, the sixth fix's left-open): the line read *rebuilt*, any bytes.
    FromTheBasesTree,
    /// Gone from its place and standing, the same entry, under this directory of the
    /// repository — whose components may stand for the acting unit and its sub-tasks as a
    /// place's do ([`OWN`], [`ITS_SUB_TASKS`]): a landed sub-task worktree's leftovers go to
    /// `.jigc/displaced/<sub-task>`, by the human's ruling of 2026-10-19 on the rows'
    /// record's §6, entry 18, and to no other unit's place there. **A component the place
    /// and the directory both name is one unit** ([`Tally::of_its_unit`]): what left
    /// `s1`'s worktree arrives under `s1`'s place and no sibling's — *"to
    /// `.jigc/displaced/<task-id>/` with its relative paths intact"* (`design/finalize.md`
    /// → *7. Post-commit*), *"`.jigc/displaced/<unit-id>/<relative>`"* (`design/storage.md`
    /// → *The per-task working area*). A directory the move makes on its way there — the
    /// directory's fixed head, `.jigc/displaced` — is its too. Keyed so since the review of
    /// the task `B8` (its H10): a sub-task's leftover held under a sibling's place, and
    /// the three lines that move an area aside named `displaced/` at large.
    ///
    /// **Only from a place under a directory** — a place the command removes: at one file's
    /// place it moves nothing aside, since jigc moves a file aside only from a place it is
    /// about to remove and never from a destination — the human's ruling of 2026-10-20 on the
    /// rows' record's §6, entry 23, where `RL3` and `CS3` parked a mover's destination's
    /// foreign occupant ([`Tally::parks`]; `byte_table::a_line_moves_aside_only_from_a_place_it_removes`
    /// names a line that tries).
    MovedAsideTo(&'static str),
    /// **Gone from its name, and standing under another in the same unit** — a staged doc
    /// re-slugged: its unit's base pin stands, and the unit's `docs/` gained a doc it did
    /// not hold whose lines are the gone doc's, its `# ` title line rewritten
    /// ([`Tally::re_slugged`]). The doc family's `W4`, `doc rename`'s *"`--to` rewrites the
    /// `# H1` **and** moves the identity: the staged body"* — and the one word under which
    /// a staged doc gone from a unit that stands is not *lost*. A commit holds nothing
    /// under it.
    ReSlugged,
    /// **Retitled** — the doc the address names ([`THE_DOC`]) rewritten so that its lines
    /// are the ones it held before the command but its `# ` title line, which is the title
    /// `--to` gives; where its slug changes, standing so at its new home
    /// ([`THE_NEW_DOC`], the slug the command gives), where nothing stood before in the
    /// working tree or in what git held, and gone from its own: *"**Rewrites the moved
    /// doc's H1** to `<New Title>` (the unified retitle), then **`git mv`** old→new
    /// path"*, and the retitle-only arm, *"rewrite the H1 and commit, with **no `git mv`
    /// and no referrer repoint**"* (`design/write-commands.md` → *`jigc rename`*). Read at
    /// its place, each end of the move: the old home gone or rewritten in place (`RN1`), the
    /// new one arrived at (`RN1, at its new home`). `jigc rename`'s; added 2026-10-19 (the
    /// task `B8`); its arrival read at its own place and only at the slug the command
    /// gives since 2026-10-20 (the review of `B8`, H7 and H11), where any free path of the
    /// home directory held.
    Retitled,
    /// **Repointed** — a doc of the store that referred to the doc [`May::Retitled`] moved,
    /// rewritten so that each line that changed is a **ref-field** of the doc's front
    /// matter — a field its doctype's schema declares `type: ref` to the moved doc's
    /// doctype — one of whose values was the old identity, whole, and that now holds the
    /// field's values with that one the new identity, in the canonical form the field
    /// grammar writes (`key: value`, a list `key: [a, b]`); every other line as it stood,
    /// none added or gone: *"rewrites each referrer's structured ref-field old→new"*,
    /// *"swaps the one element and re-emits the canonical whole-list"*, and *"the CLI
    /// rewrites **structured ref-fields in the managed store** and nothing else"*. Nothing
    /// where the doc did not move. `jigc rename`'s `RN2`. Narrowed 2026-10-20, after the
    /// review of the task `B8` (H4): a line that held the old identity anywhere, as a
    /// substring, could take the new one — a ref to another doc whose id the old one
    /// prefixes, and a mention in the prose.
    Repointed,
    /// **Into its home** — gone from its place, and standing, the same entry, at its
    /// doctype's home as the cascade resolves it **after** the command, under its own name
    /// (or as a placement doctype's file). The doctype is the one the command names, for a
    /// place that is a prior home ([`PRIOR_HOME`], `jigc relocate`'s `RL1`), and else the
    /// one whose home held the path before the command (`jigc config set`'s `CS2`, a root
    /// knob's re-point). Added 2026-10-19 (the task `B8`). **Read at both ends** since the
    /// human's ruling of 2026-10-20 on the rows' record's §6, entry 23 (the task `B9`): the
    /// departure at the line's place, the arrival at a mover's destination ([`A_NEW_HOME`]),
    /// **where nothing stood before, in the working tree or in what git held** — what stands
    /// there blocks the move, where an occupant parked under `.jigc/displaced/` had let it
    /// land. **What moves is a doc git held, a regular file** — in the working tree, at a
    /// path `HEAD` or the index held as one: *"every **committed** `.md`"*, *"A relocation
    /// **lands a regular file** or refuses"* (after the review of `B9`, H2, where a `.md`
    /// git never held and a link each moved into a home and held).
    IntoItsHome,
    /// **Migrated** — a doc of the store below its doctype's current schema version (its
    /// stamp, `schema-version: N`, below the version the doctype's pack manifest declares,
    /// or absent) rewritten at its home — or, from a home its doctype had at an earlier
    /// version ([`A_PRIOR_DOC`]), moved to the current one — so that its stamp is the current
    /// version and **every byte the migration's sentences do not let it change stands as it
    /// stood, in order**: its front matter's fields and values, its `- key:` bullets, its
    /// headings, its prose. What may change is the stamp, the `# ` title where the
    /// doctype's `display-title:` moved, and the `## ` headings of sections its schema
    /// added, each read off the two schemas ([`carried`]). *"The prose is **unchanged**"*
    /// (`design/corpus-migration.md` → *The determinism boundary holds*). `jigc
    /// migrate-corpus`'s `MG1`; added 2026-10-19 (the task `B8`); every byte, not every
    /// prose line, since the review of `B8` (its H3).
    Migrated,
    /// **Its knob set** — the project manifest, made or rewritten so that every line of it
    /// that is no line of the knob the command names (the unit) stands as it stood, in
    /// order, and one line names that knob, under the `scalar:` map that holds it — made
    /// where none stood — **set to the value the command names**: *"`jigc config set <key>
    /// <value>` | a `scalar-set`"*, in the one file a hand edit is first-class — *"**by
    /// hand** (edit `manifest.yaml`"* (`design/overrides.md` → *Authoring deltas*). `jigc
    /// config set`'s `CS1`; added 2026-10-19 (the task `B8`); the value read, and the
    /// `scalar:` line held, since the review of `B8` (its H9).
    ItsKeySet,
    /// **Retired** — the foreign original a migration task recorded ([`THE_RETIRED`]) gone
    /// from the working tree and from the index, **where what goes is what `HEAD` holds
    /// there**: the working tree's regular file there the bytes `HEAD`'s tree holds at the
    /// path, and the index there `HEAD`'s entry or nothing. *"the retire deletes only a file
    /// git holds a copy of"*, whose warrant is *"a guarded migrate destroys only a reviewed,
    /// in-repo, **git-recoverable** file"* (`design/auto-migration.md` →
    /// *Retire-the-foreign-original (the first destructive write)*): the doc's test asks
    /// whether git holds a copy **of the path**, and the warrant is about the bytes, so the
    /// word reads the warrant — an edit in no commit, or a change staged there, is bytes no
    /// commit holds, and a retire that takes them is a loss the word does not excuse; nor a
    /// link or anything that is no regular file. Nothing is written. `jigc task finalize`'s
    /// `T10`, under its spelling `--approve`; added 2026-10-20, after the review of the task
    /// `B9` (H6), where the row had no line for the retire. Aimed at by **U** — a file of the
    /// user's in the source's place, the index's entry gone with the one taken away — by
    /// **E** at both its positions, before the migration copies the source in and after, and
    /// by **S** there ([`aimed`]): the user's bytes that no commit holds, where the retire
    /// deletes.
    Retired,
}

/// **What no line that lets a cache be rebuilt covers**, by its path from the repository's
/// root: the install-footprint record of a failed `jigc setup`, at the path a run showed it
/// under (`…/reshape/byte-table-acceptance-rows.md` → §6, entry 3). The human's ruling of
/// 2026-10-15 on that entry: the record **is not rebuildable**, so it is no cache — it is
/// to move out of `.jigc/state/`, a refusal leaves none, and **the table checks its path**.
/// A file here is covered by no [`May::Rebuilt`] line, at exit 0 or on a refusal; where
/// the record moves to, no line of a cache's covers it either.
pub const NO_CACHE: &[&str] = &[".jigc/state/setup-install-footprint"];

/// **The rendered message's temporary file, by its name — which no line covers where it
/// stands after the command, made or rewritten**: the record-only commit's
/// `record-commit-msg.txt` and the finalize's `finalize-message.tmp`, the two names
/// `engine` declares (`RECORD_COMMIT_MSG_FILE`, `state::FINALIZE_MESSAGE_FILE`), under
/// `.jigc/`. *"`git commit -F <rendered-message-file>` (a temp file, deleted after)"*
/// (`design/finalize.md` → *6. Commit*), and the human's ruling of 2026-10-14 on the
/// record's §6, entry 6: **the file is deleted whether or not the commit landed**. So a
/// line whose place holds the file — a directory a line lets be rebuilt — lets nothing of
/// it stand, at exit 0 or on a refusal; the file gone is as the place's line has it.
/// Added 2026-10-19, after the review of the task `B7`'s second part (its tenth finding):
/// the milestone's area is the record doors' lines whole, so the file a landed record
/// commit left stood unseen at exit 0.
pub const DELETED_AFTER: &[&str] = &["record-commit-msg.txt", "finalize-message.tmp"];

/// Whether `rel`, from the repository's root, is a file [`DELETED_AFTER`] names.
fn deleted_after(rel: &str) -> bool {
    rel.starts_with(".jigc/")
        && rel
            .rsplit('/')
            .next()
            .is_some_and(|name| DELETED_AFTER.contains(&name))
}

/// What a commit the command makes may hold, beside what the row's lines of the file
/// layer wrote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Holding {
    /// Nothing else: not the index as it stood.
    OnlyTheLines,
    /// The main checkout's index, whole, as it stood.
    TheIndex,
    /// **What the checkouts at this place had staged** — a path from the repository's root
    /// whose components [`OWN`] and [`ITS_SUB_TASKS`] may stand for the acting unit and its
    /// sub-tasks: a fan-out's *"sub-agent's **code** lives in its own git worktree
    /// (`.jigc/worktrees/<sub-id>`, `git add`ed there)"* (`design/finalize.md` →
    /// *`fan-out` finalize*). Another checkout's staged bytes — a task's its unit's record
    /// does not name, the user's own linked worktree — are none of it.
    TheWorktrees(&'static str),
    /// **The same checkouts, a commit each, then the aggregate** — `squash: false`'s chain:
    /// *"one commit per code-carrying sub-task in task-id order, then the parent's aggregate
    /// commit"*, *"Each per-sub-task commit applies **that one worktree's** staged code"*
    /// (`design/finalize.md` → *`fan-out` finalize*). A checkout at this place carries code
    /// where its index differs from the tree its own `HEAD` holds; the commits the command
    /// made are, oldest first, one for each such checkout in the order of the sub-task ids
    /// they stand at — each holding what that one checkout staged and nothing else — and
    /// then one more, holding only what the row's lines wrote. *"A sub-task that staged no
    /// code contributes no commit."* A single commit that folds them all, a chain out of
    /// order, a sub-task's commit carrying another's code and a chain with no aggregate are
    /// each git's state moved outside the allowance.
    TheWorktreesOneByOne(&'static str),
    /// **Only what the row's lines changed, and every change of theirs to a tracked path**
    /// — a path the main checkout's index held before the command: such a change lands in
    /// a commit the command made, and one the working tree keeps with no commit carrying
    /// it is outside the allowance. **Nor may the commit carry bytes of the user's that no
    /// object within reach held before** — a plant's edit swept in with the command's own
    /// change at the same path. The human's ruling of 2026-10-14 on `uninstall`
    /// (the record's §6, entry 5).
    AllTheLinesChanged,
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
    /// **A worktree's registration added, with its checkout, at a path under this place of
    /// the repository** — whose components [`OWN`] and [`ITS_SUB_TASKS`] may stand for the
    /// acting unit and its sub-tasks: `jigc milestone provision`'s *"`git worktree add
    /// --detach` populates from the pinned base's *tree*"* (`design/team-ready-state.md` →
    /// *What a fan-out worktree isolates — and what it does not (M45)*), at
    /// `.jigc/worktrees/<sub-task>`. A registration made anywhere else, or for a task the
    /// unit's record does not name, is git's state moved outside the allowance. **And the
    /// checkout stands as that add leaves one**: its `HEAD` detached (*"`--detach`"*) at the
    /// shared base the unit's committed record pins (*"pins one shared base (= HEAD) that
    /// every sub-task inherits"*, `design/write-commands.md` → *Minting a milestone + its
    /// task list (M7)*), and its index that commit's tree, nothing staged against it. One
    /// at another commit, on a branch, or with something staged is git's state moved too.
    /// Added 2026-10-19, after the review of the task `B7`'s second part, its ninth
    /// finding: the line excused a new checkout by its path alone. *What it does not ask*:
    /// the checkout's files, which are the file layer's lines'.
    Registered(&'static str),
}

/// A condition a line carries: read off the manifest taken before the command, or off how
/// the command ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum When {
    Always,
    /// The repository has no commit: its `HEAD` is unborn.
    NoCommitYet,
    /// **The place's first commit**: the tree the main checkout's `HEAD` holds before the
    /// command has nothing at the line's place — an unborn `HEAD` included — so the commit
    /// the command makes is the first to land what stands there. *"on a first commit, the
    /// project config layer (`.jigc/config`, `.jigc/.gitignore`)"* (`design/finalize.md` →
    /// *5. Stage*): the layer `setup` wrote and, until M48, never committed, which the
    /// first finalize landed. A line under it of any other place than the file layer's is
    /// in force nowhere. Added 2026-10-19, after the review of the task `B7`'s second
    /// part, its first finding: [`May::Carried`] had held under any `HEAD`, so a pending
    /// delta, and a file of the user's nobody staged under the place, rode a commit that
    /// was not the layer's first and held.
    FirstCommitOfItsPlace,
    /// The command refused. A line under it holds on a refusal and **only** then: what
    /// the command may do where it exits 0 is another line's to say.
    Refused,
    /// **It committed** — the command exited 0, or it refused having landed a commit of its
    /// own on the checked-out branch: a line under it that holds on a refusal holds on such
    /// a refusal alone. `jigc migrate-corpus` refuses one doc while *"the run's other docs
    /// still migrate and commit, and the run exits non-zero"* (`design/finalize.md` → *4.
    /// Promote managed docs*), and a refusal that committed nothing is held to change
    /// nothing — the human's ruling of 2026-10-19 on the rows' record's §6, entry 22. **On
    /// such a refusal, what git tracks changes only as the commit landed it**
    /// ([`Tally::as_it_landed`]): a path the main checkout's `HEAD` held before the command
    /// or after it, or its index did, stands in the working tree and in the index as the
    /// commit left it — so the doc the refusal reported *"joins `blocked`"* and is not
    /// written; what git tracks nothing of, the gitignored caches, the line's word reads as
    /// it does at exit 0. Added 2026-10-19, with the movers family's rows (the task `B8`);
    /// held to the commit since its review (H8): the refused doc migrated in the working
    /// tree, or staged, held.
    ItCommitted,
    /// **A doc of the sweep left, and named** — the command exited 0, or it refused having
    /// left a doc its sweep was asked to move where it stood, and said so: an entry with a
    /// doc's name (`<name>.md`) stood under the prior home `--from` names before the command
    /// and stands there after as it stood, and the command's output names its path, or the
    /// path its move would have landed it at ([`A_NEW_HOME`]). The human's two rulings of
    /// 2026-10-20: *an acting sweep does all it can, then exits non-zero if any act it was
    /// asked for did not happen* (R1), and `jigc relocate` *blocks that doc, moves the
    /// others, names the path to clear and exits non-zero* (the rows' record's §6, entry
    /// 23). A refusal that names no doc it left — an argument's, a doctype's, taken before
    /// the sweep — is held to change nothing. `jigc relocate`'s lines that write.
    ADocLeftAndNamed,
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
    /// **The spelling of the leaf the line is in force under** ([`Spelling`]): `None` for a
    /// line of every spelling but one that replaces it — the leaf as typed plainly among
    /// them — and a spelling's name for a line in force under that spelling alone.
    pub spelling: Option<&'static str>,
}

/// **A second spelling of a leaf** — the same command, which its doc has commit, or write,
/// otherwise in a state no repository's bytes show: a task whose workflow includes
/// `step:finalize-doc-only`, a cascade knob. A cell names the spelling it runs
/// ([`Cell::spelling`]), and is compared under its row's lines as that spelling has them:
/// every line of no spelling but those it `replaces`, and the lines that name it. The
/// human's ruling of 2026-10-18 on the two spellings (`DECISIONS.md`).
#[derive(Clone, Copy, Debug)]
pub struct Spelling {
    /// **What the spelling is, by its name** ([`Spelling::typed_in`]): a name that begins
    /// with `--` is the flag a command types; `bare` is the leaf typed with no word after
    /// it; any other name is a state the cell builds, which no command's words show.
    pub name: &'static str,
    /// The `id`s of the row's lines this spelling takes out of force.
    pub replaces: &'static [&'static str],
}

impl Spelling {
    /// **Whether the command `argv` of `leaf` types this spelling** — `None` for a spelling
    /// no command's words show ([`Spelling::name`]). A cell whose command types a spelling
    /// is compared under it, and names it: the review of the task `B9`, its third hole —
    /// the catalog's bare `jigc start` and `jigc workflow … --preview` were compared under
    /// the plain spelling's lines.
    pub fn typed_in(&self, leaf: &[&str], argv: &[&str]) -> Option<bool> {
        if self.name == "bare" {
            Some(argv == leaf)
        } else if self.name.starts_with("--") {
            Some(argv.contains(&self.name))
        } else {
            None
        }
    }
}

/// **A leaf's row**: the command as `cli::cli::VERB_KINDS` holds it, the flag its owner
/// doc names as the consent to a destruction where it has one, its home state, and the
/// allowance.
#[derive(Clone, Copy, Debug)]
pub struct Row {
    pub leaf: &'static [&'static str],
    /// How the command names the work unit it acts on — whose area [`OWN`] and
    /// [`ITS_SUB_TASKS`] are.
    pub unit: Unit,
    pub consent: Option<&'static str>,
    /// **The leaf's home state** — the smallest prepared state in which its write happens
    /// (the plan's §3): where its planted cells and its base cell stand, and run in the
    /// gate. A base cell in any other state runs on demand ([`Layer`]).
    pub home: Prepared,
    pub lines: &'static [Line],
    /// The leaf's second spellings, each with the lines it replaces; its own lines are the
    /// row's lines that name it.
    pub spellings: &'static [Spelling],
}

/// **A plant's kind, at one of its positions** — the plan's letters (§3, *The plants —
/// each with the rule that places it*). A kind with more than one position, or more than
/// one shape, is one variant each: a cell is one plant at one position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// **U** — an untracked file.
    Untracked,
    /// **T** — a committed file.
    Committed,
    /// **L** — a link to nothing.
    LinkToNothing,
    /// **L** — a link to a file in the repository.
    LinkInside,
    /// **L** — a link to a file under `outside/`.
    LinkOutside,
    /// **K** — a directory in a file's place.
    DirForAFile,
    /// **K** — a file in a directory's place: at the parent of a file the command writes.
    FileForADir,
    /// **E** — an edit in no commit, before the work unit's first write to the file: a line
    /// added ([`Plant::Edit`]), or the file deleted, the index left as it stands
    /// ([`Plant::Deleted`]).
    EditBefore,
    /// **E** — the same after that first write; or, where no unit stands between, last
    /// before the command.
    EditAfter,
    /// **M** — a user's own line in a file jigc merges into, the file in no commit.
    UsersLine,
    /// **S** — a staged change at a place the command writes.
    StagedThere,
    /// **S** — a staged change at a place it does not.
    StagedElsewhere,
    /// **N** — a file whose name git quotes.
    QuotedName,
    /// **D** — a file the product did not write, in a directory the command removes.
    Foreign,
    /// **H** — a `pre-commit` hook of the user's that rejects.
    RejectingHook,
    /// **W** — the user's own linked worktree, its directory there.
    WorktreeThere,
    /// **W** — the same, its directory moved away.
    WorktreeMovedAway,
}

impl Kind {
    pub const ALL: &'static [Kind] = &[
        Kind::Untracked,
        Kind::Committed,
        Kind::LinkToNothing,
        Kind::LinkInside,
        Kind::LinkOutside,
        Kind::DirForAFile,
        Kind::FileForADir,
        Kind::EditBefore,
        Kind::EditAfter,
        Kind::UsersLine,
        Kind::StagedThere,
        Kind::StagedElsewhere,
        Kind::QuotedName,
        Kind::Foreign,
        Kind::RejectingHook,
        Kind::WorktreeThere,
        Kind::WorktreeMovedAway,
    ];

    /// The plan's letter, with the position where the kind has more than one.
    pub fn tag(self) -> &'static str {
        match self {
            Kind::Untracked => "U",
            Kind::Committed => "T",
            Kind::LinkToNothing => "L, to nothing",
            Kind::LinkInside => "L, to a file in the repository",
            Kind::LinkOutside => "L, to a file outside it",
            Kind::DirForAFile => "K, a directory in a file's place",
            Kind::FileForADir => "K, a file in a directory's place",
            Kind::EditBefore => "E, before the first write",
            Kind::EditAfter => "E, after the first write",
            Kind::UsersLine => "M",
            Kind::StagedThere => "S, where the command writes",
            Kind::StagedElsewhere => "S, where it does not",
            Kind::QuotedName => "N",
            Kind::Foreign => "D",
            Kind::RejectingHook => "H",
            Kind::WorktreeThere => "W, its directory there",
            Kind::WorktreeMovedAway => "W, its directory moved away",
        }
    }
}

/// **Whether a plant of `kind` targets `line`** — the plan's position rules (§3), read
/// against the allowance's vocabulary. `Err` is **the reason the frame derives** for a
/// line a kind does not target: nobody types it.
///
/// The rules, a kind each: **U** goes *at each place a line lets the command create or
/// replace*; **T** *at a place the command creates*, which a commit can hold; **L** and
/// **K** *at each place the command creates, replaces or merges into* — one file's
/// place; **E** *in a tracked file the command rewrites*, and before a unit's first
/// write only where a unit's staged doc is what lands; **M** *in a file jigc merges
/// into*; **S** *at a place the command writes, and at one it does not* — the second is
/// the commit's to answer; **N** *where the command lists or stages paths*; **D** *in a
/// directory the command removes*; **H** *at every leaf whose row has a commit*; **W**
/// where a line removes a registration.
///
/// **And the home a write copies a committed doc in from** ([`May::CopiedFrom`]) — read,
/// and nothing written: what stands in the doc's place there is what the copy-in meets,
/// so **U**, **L** and **K**, *a directory in a file's place*, go there as they go to a
/// place the command writes, and **E** at both of its positions, the command's own write
/// being the task's first or a later one. **T** does not: a committed file there is the
/// committed doc the line reads. Nor does **K**, *a file in a directory's place* — it
/// takes every doc of the home's directory, not the one the line reads — nor **S** or
/// **M**, which are held to a write.
///
/// **And a committed doc the command rewrites or moves where it stands** — the movers
/// family's words, [`May::Retitled`], [`May::Repointed`], [`May::IntoItsHome`],
/// [`May::Migrated`] and [`May::ItsKeySet`] (the task `B8`): **U**, **L** and **K** go there
/// as to a place the command replaces, **E** after the first write and **S** where the
/// command writes, as to a tracked file it rewrites; **M** to the knob's manifest, which
/// the command merges a line into. **T** does not: a committed file at the place is the
/// doc itself.
///
/// **And a destination a doc is moved to** — the doc an address names, at its new home
/// ([`THE_NEW_DOC`], `RN1, at its new home`), and a mover's ([`A_NEW_HOME`], `RL1, at its
/// new home`, `CS2, at its new home`): nothing stands there before the move, so what does is
/// a squatter, and **U**, **T**, **L** and **K**, *a directory in a file's place*, go there
/// — a file of the user's, one committed, a link, a directory — and no other, since a
/// tracked file there to edit or stage over is the committed one **T** puts, and the home's
/// directory is the doc's own home's, which the doc's own place takes (added 2026-10-20,
/// after the review of the task `B8`, H7; a mover's since the human's ruling of that day on
/// the rows' record's §6, entry 23, where `RL3` and `CS3`, which parked what stood there,
/// had been the place).
///
/// **And what a re-entry provisions** ([`May::Provisioned`], `jigc workflow`'s `WF1`): its
/// place is the sub-task's area, a directory, but what it makes there is named files —
/// *"the commit doc, any workflow-provisioned docs"* — each one file's place, so **L** and
/// **K** go there as to one file's place: a link or a directory where the commit doc goes,
/// a file where its directory does. **T**, **E** and **S** do not: the area is under the
/// workbench, where git holds no file to commit, edit or stage (added 2026-10-20, after the
/// review of the task `B9`, H4's lead).
pub fn aimed(kind: Kind, line: &Line) -> Result<(), &'static str> {
    if line.when == When::Refused {
        return Err(
            "the line holds on a refusal only, and a plant targets what a command \
                    may do where it acts",
        );
    }
    let one_file = |place: Where| !matches!(place, Where::Under(_));
    let lands = |may: May| {
        matches!(
            may,
            May::WriteWhole
                | May::Rebuilt
                | May::Invalidated(_)
                | May::CopyOfAStagedDoc
                | May::FromTheBasesTree
                | May::Provisioned
        )
    };
    let merges = |may: May| matches!(may, May::MergeInto | May::TakeOutOf);
    // What a migration task's retire deletes: a file the migration copied in before — so a
    // file of the user's in its place, an edit there before the copy or after it, and a
    // change staged there are each the user's own bytes in no commit where the retire takes
    // the file away.
    let retires = |may: May| matches!(may, May::Retired);
    let read = |may: May| matches!(may, May::CopiedFrom);
    // What a re-entry provisions: named files in an area, each one file's place.
    let provisions = |may: May| matches!(may, May::Provisioned);
    // A doc's destination: where the doc the address names stands once its identity moved,
    // and where a mover lands a doc.
    let destination = |place: Where| matches!(place, Where::Path(path) if path == THE_NEW_DOC || path == A_NEW_HOME);
    // A committed doc the command rewrites or moves where it stands — the movers family's
    // words. **T** does not go there: a committed file at the place is the doc itself.
    let rewrites = |place: Where, may: May| {
        !destination(place)
            && matches!(
                may,
                May::Retitled | May::Repointed | May::IntoItsHome | May::Migrated | May::ItsKeySet
            )
    };
    let aimed = match (kind, line.allows) {
        (Kind::Untracked, Allows::Files(place, may)) => {
            lands(may) || read(may) || rewrites(place, may) || destination(place) || retires(may)
        }
        (Kind::Committed, Allows::Files(place, may)) => {
            (lands(may) && one_file(place) && !matches!(place, Where::Hook)) || destination(place)
        }
        (
            Kind::LinkToNothing | Kind::LinkInside | Kind::LinkOutside | Kind::DirForAFile,
            Allows::Files(place, may),
        ) => {
            provisions(may)
                || (one_file(place)
                    && (lands(may)
                        || read(may)
                        || rewrites(place, may)
                        || destination(place)
                        || matches!(may, May::MergeInto)))
        }
        (Kind::FileForADir, Allows::Files(place, may)) => {
            provisions(may)
                || (one_file(place)
                    && (lands(may) || rewrites(place, may) || matches!(may, May::MergeInto)))
        }
        (Kind::EditBefore, Allows::Files(_, may)) => {
            matches!(may, May::CopyOfAStagedDoc) || read(may) || retires(may)
        }
        (Kind::EditAfter, Allows::Files(place, may)) => {
            one_file(place)
                && !matches!(place, Where::Hook)
                && (lands(may) || merges(may) || read(may) || rewrites(place, may) || retires(may))
        }
        (Kind::StagedThere, Allows::Files(place, may)) => {
            one_file(place)
                && !matches!(place, Where::Hook)
                && (lands(may) || merges(may) || rewrites(place, may) || retires(may))
        }
        (Kind::UsersLine, Allows::Files(_, may)) => merges(may) || matches!(may, May::ItsKeySet),
        (Kind::Foreign, Allows::Files(place, may)) => {
            matches!(place, Where::Under(_)) && matches!(may, May::Removed)
        }
        (Kind::StagedElsewhere | Kind::QuotedName | Kind::RejectingHook, Allows::Commit(_)) => true,
        (Kind::WorktreeThere | Kind::WorktreeMovedAway, Allows::Unregistered(_)) => true,
        _ => false,
    };
    if aimed {
        return Ok(());
    }
    Err(match kind {
        Kind::Untracked => "the line lets the command create or replace nothing",
        Kind::Committed => "the line lets the command create nothing a commit can hold",
        Kind::LinkToNothing | Kind::LinkInside | Kind::LinkOutside => {
            "the line names no one file's place the command creates, replaces or merges into"
        }
        Kind::DirForAFile | Kind::FileForADir => {
            "the line names no one file's place the command writes"
        }
        Kind::EditBefore => {
            "no staged doc of a work unit lands at the line's place, and no committed doc is \
             copied in from it"
        }
        Kind::EditAfter => "the line lets the command rewrite or copy in no tracked file",
        Kind::UsersLine => "the line merges into no file and takes lines out of none",
        Kind::StagedThere => "the line names no tracked file's place the command writes",
        Kind::StagedElsewhere | Kind::QuotedName | Kind::RejectingHook => "the line is no commit's",
        Kind::Foreign => "the line removes no directory",
        Kind::WorktreeThere | Kind::WorktreeMovedAway => "the line removes no registration",
    })
}

/// **What a cell's plant is aimed at**: a line of its leaf's row, by its `id`, and the
/// kind of plant at the position it stands in. It names a line and never a path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Aim {
    pub line: &'static str,
    pub kind: Kind,
}

/// **The layer a cell stands in** (the plan's §3, *How the cross is kept from
/// exploding*) — derived from the cell and its row, typed by nobody.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layer {
    /// The leaf with the argv that resolves nothing (`support::leaf_argv`), in `fresh`,
    /// with no lead.
    Refusing,
    /// The leaf with something to act on and no plant, in its home state.
    Base,
    /// The same in a state that is not the leaf's home: **it runs on demand**, in no gate.
    BaseElsewhere,
    /// A plant, or a condition a doc names for the command's guard.
    Planted,
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

/// **The refusal a cell expects**: the exit code, and words the command's output holds —
/// the finding's code where the refusal is a finding, else a run of the sentence that
/// says why. A cell of the table names its refusal: one that refuses for another reason
/// than the one its plant was put in for is no answer to its plant.
///
/// **And the docs it moved before refusing**, where its door is an acting sweep: each pair a
/// doc's place before the command and its home, the same entry standing at the home after it
/// and nothing at the place — the human's rulings of 2026-10-20: *an acting sweep **does all
/// it can**, then exits non-zero* (R1), and `jigc relocate` *blocks that doc, **moves the
/// others***, names the path to clear and exits non-zero (the rows' record's §6, entry 23).
/// A refusal of the whole sweep moves nothing, and is not the listed outcome. Added
/// 2026-10-20, after the review of the task `B9` (H1): a refusal's words were all a listing
/// held, and the whole refusal came out *refuses, nothing changed*.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub exit: i32,
    pub says: &'static str,
    pub moved: &'static [(&'static str, &'static str)],
}

impl Refusal {
    /// **What the refusal did not move that its listing names**: each pair whose doc does
    /// not stand at its home after the command, the same entry it was at its place, with
    /// nothing left at the place.
    fn not_moved(&self, before: &Manifest, after: &Manifest) -> Vec<String> {
        self.moved
            .iter()
            .filter(|(from, to)| {
                let was = before.files.get(&format!("{REPO}/{from}"));
                let is = after.files.get(&format!("{REPO}/{to}"));
                !(matches!(was, Some(Entry::File { .. }))
                    && is.is_some_and(|is| was.is_some_and(|was| was.same(is)))
                    && !after.files.contains_key(&format!("{REPO}/{from}")))
            })
            .map(|(from, to)| format!("`{from}` → `{to}`"))
            .collect()
    }
}

/// What a cell is listed as: a command that acts, or one that refuses — and, for a cell
/// of the table, with which refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Listed {
    Acts,
    /// It refuses: any non-zero exit. The frame's own arms use it; a cell of the table
    /// names its refusal.
    Refuses,
    /// It refuses with this exit code, and its output holds these words.
    RefusesWith(Refusal),
}

impl Listed {
    /// Whether the cell is listed as one that refuses.
    pub fn refuses(self) -> bool {
        self != Listed::Acts
    }
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
    /// The command: the words after `jigc` — what the row's [`Unit`] reads the unit off.
    pub argv: &'a [&'a str],
    /// The spelling of the leaf the command ran under ([`Spelling`]).
    pub spelling: Option<&'a str>,
    pub ended: Ended<'a>,
    /// Whether the cell runs the row's consent spelling.
    pub consent: bool,
    pub listed: Listed,
}

/// What one cell came to: the classification, why, and **how many differences each line
/// of the row excused** — every line, so that one that excuses much is seen and one that
/// excuses nothing is too.
///
/// **A red cell shows every class it is red in**: the classification is the one that
/// outranks the others, and `also` holds each lower class the comparison found, with its
/// reasons — a file written outside the allowance beside a loss, a refusal that is not
/// the one the cell names beside a refusal that changed something.
#[derive(Clone, Debug)]
pub struct Verdict {
    pub classification: Classification,
    pub why: Vec<String>,
    pub also: Vec<(Classification, Vec<String>)>,
    pub excused: Vec<(&'static str, usize)>,
}

impl Verdict {
    /// Every class the cell shows, the classification first, each with its reasons.
    pub fn shown(&self) -> String {
        let mut out = self.why.join("\n");
        for (class, why) in &self.also {
            out.push_str(&format!("\nalso *{class}*:\n{}", why.join("\n")));
        }
        out
    }
}

/// **The work unit a command acts on** — the id its row's [`Unit`] reads off the command,
/// the sub-tasks the unit's committed record names, the milestone whose committed record
/// names the unit, and the directory the cascade resolves `docs-root` to: what [`OWN`],
/// [`ITS_SUB_TASKS`], [`ITS_MILESTONE`] and [`DOCS_ROOT`] stand for in a place. None of the
/// first three where the command names no unit.
#[derive(Clone, Debug, Default)]
struct Acting {
    unit: Option<String>,
    sub_tasks: BTreeSet<String>,
    /// The sub-tasks of `sub_tasks` whose item the record has settled — what
    /// [`RE_SEEDED`] leaves out.
    settled: BTreeSet<String>,
    milestone: Option<String>,
    /// From the repository's root, `""` for the root itself; `None` where a layer that
    /// stands does not read.
    docs_root: Option<String>,
    /// Every path of the repository that held an entry before the command, from its root —
    /// in the working tree, or in what git held: the tree the main checkout's `HEAD` held
    /// and its index, each path with the directories above it. What [`MINTED`],
    /// [`EACH_MINTED`] and [`RE_SEEDED`] ask a place against; `None` where no manifest was
    /// read, and then a place that holds the word is nowhere.
    stood: Option<BTreeSet<String>>,
    /// The ids the command minted, read off the committed records before and after it —
    /// what [`MINTED`] and [`EACH_MINTED`] stand for.
    minted: BTreeSet<String>,
    /// The tasks the command minted: the ids at which an area under `.jigc/tasks/` carries
    /// its base pin after the command, where nothing stood before it — what [`MINTED_TASK`]
    /// stands for.
    minted_tasks: BTreeSet<String>,
    /// The shared base the unit's committed record pins — the `base:` of its front matter,
    /// the full id — what [`Allows::Registered`] asks a new checkout's `HEAD` against.
    base: Option<String>,
    /// Every linked checkout whose directory stood before the command, by that directory
    /// from the repository's root — what [`UNPROVISIONED`] asks a place against.
    checked_out: BTreeSet<String>,
    /// The doctypes' homes as the cascade resolves them before the command and after it —
    /// what [`A_DOC`], [`A_PRIOR_DOC`], [`THE_DOC`] and the words that move a doc read.
    homes: Option<Homes>,
    homes_after: Option<Homes>,
    /// The doc the unit names, where the unit is an address: its identity, and its home.
    the_doc: Option<(String, String)>,
    /// What the command's `--from` names, from the repository's root — [`PRIOR_HOME`].
    from: Option<String>,
    /// The title the command's `--to` gives — what [`May::Retitled`] asks the new `# `
    /// line to be.
    to: Option<String>,
    /// Where the doc the unit names stands once its identity has moved — [`THE_NEW_DOC`].
    new_doc: Option<String>,
    /// **The word after the unit**, where the unit is the word after the leaf — the value
    /// `jigc config set <key> <value>` names, what [`May::ItsKeySet`] asks the knob to be.
    value: Option<String>,
    /// **Where the command's moves into a doctype's home land** — [`A_NEW_HOME`], each path
    /// from the repository's root: the destination ([`Acting::destination`]) of every doc that
    /// stood before the command.
    new_homes: BTreeSet<String>,
    /// **What the unit's migration recorded as its source** — [`THE_RETIRED`]: the bytes of
    /// `.jigc/tasks/<unit>/source-path` before the command, a path from the repository's
    /// root; `None` where the unit's area holds no such file.
    retired: Option<String>,
}

impl Acting {
    /// What the row's [`Unit`] reads off `argv`, against the manifest taken before the
    /// command — where the task areas stand, the cascade's layers, and the committed
    /// records — and what it minted, against the committed records after it.
    fn of(row: &Row, argv: &[&str], before: &Manifest, after: &Manifest) -> Acting {
        let flag = argv.iter().enumerate().find_map(|(at, word)| match *word {
            "--task" => argv.get(at + 1).map(|id| id.to_string()),
            word => word.strip_prefix("--task=").map(str::to_string),
        });
        let unit = match row.unit {
            Unit::Nothing => None,
            Unit::TheTaskFlag => flag.or_else(|| {
                // A task is an area that carries its base pin.
                let standing: Vec<&str> = before
                    .files
                    .iter()
                    .filter(|(_, entry)| matches!(entry, Entry::Dir(_)))
                    .filter_map(|(path, _)| {
                        let id = in_repo(path)?.strip_prefix(".jigc/tasks/")?;
                        (!id.contains('/')
                            && matches!(
                                before.files.get(&format!("{path}/base.json")),
                                Some(Entry::File { .. })
                            ))
                        .then_some(id)
                    })
                    .collect();
                match standing.as_slice() {
                    [one] => Some(one.to_string()),
                    _ => None,
                }
            }),
            Unit::TheWordAfterTheLeaf => argv
                .get(row.leaf.len())
                .filter(|word| !word.starts_with('-'))
                .map(|word| word.to_string()),
        };
        let value = matches!(row.unit, Unit::TheWordAfterTheLeaf)
            .then(|| argv.get(row.leaf.len() + 1))
            .flatten()
            .filter(|word| !word.starts_with('-'))
            .map(|word| word.to_string());
        let docs_root = docs_root(before);
        let records = docs_root
            .as_deref()
            .map(|root| committed_records(before, root))
            .unwrap_or_default();
        let own = unit.as_ref().and_then(|unit| records.get(unit));
        let (sub_tasks, settled, base) = own
            .map(|record| {
                (
                    record.items.clone(),
                    record.settled.clone(),
                    record.base.clone(),
                )
            })
            .unwrap_or_default();
        let milestone = unit.as_ref().and_then(|unit| {
            let naming: Vec<&String> = records
                .iter()
                .filter(|(_, record)| record.items.contains(unit))
                .map(|(id, _)| id)
                .collect();
            match naming.as_slice() {
                [one] => Some(one.to_string()),
                _ => None,
            }
        });
        // What stood: the working tree, and what git held — `HEAD`'s tree and the index,
        // each path with the directories above it.
        let mut stood: BTreeSet<String> = before
            .files
            .keys()
            .filter_map(|path| in_repo(path).map(str::to_string))
            .collect();
        if let Some(main) = before.checkouts.get(REPO) {
            for path in tree_of(before, &main.head).keys().chain(main.index.keys()) {
                let mut at = String::new();
                for part in path.split('/') {
                    if !at.is_empty() {
                        at.push('/');
                    }
                    at.push_str(part);
                    stood.insert(at.clone());
                }
            }
        }
        // What the command minted, off the records `HEAD` holds after it: for a unit, the
        // items its record names that it did not; for none, the records at the home that
        // stood nowhere before.
        let minted: BTreeSet<String> = docs_root
            .as_deref()
            .map(|root| {
                let now = committed_records(after, root);
                match unit.as_ref() {
                    Some(unit) => now
                        .get(unit)
                        .map(|record| record.items.difference(&sub_tasks).cloned().collect())
                        .unwrap_or_default(),
                    None => now
                        .keys()
                        .filter(|id| !stood.contains(&record_path(root, id)))
                        .cloned()
                        .collect(),
                }
            })
            .unwrap_or_default();
        // The tasks it minted: an area that carries its pin after the command, at an id where
        // nothing stood.
        let minted_tasks: BTreeSet<String> = after
            .files
            .iter()
            .filter(|(_, entry)| matches!(entry, Entry::File { .. }))
            .filter_map(|(path, _)| {
                let id = in_repo(path)?
                    .strip_prefix(".jigc/tasks/")?
                    .strip_suffix("/base.json")?;
                (!id.contains('/') && !stood.contains(&format!(".jigc/tasks/{id}")))
                    .then(|| id.to_string())
            })
            .collect();
        let homes = Homes::over(before);
        let the_doc = unit.as_ref().and_then(|unit| {
            let (ty, slug) = unit.split_once(':')?;
            let home = homes.as_ref()?.current.get(ty)?;
            Some((unit.clone(), home.for_name(&format!("{slug}.md"))))
        });
        let word_after = |flag: &str| {
            argv.iter()
                .position(|word| *word == flag)
                .and_then(|at| argv.get(at + 1))
                .map(|word| word.to_string())
        };
        let from = word_after("--from").map(|from| {
            from.trim_start_matches("./")
                .trim_end_matches('/')
                .to_string()
        });
        let to = word_after("--to");
        // The slug the command gives: `--slug`'s, and else the one `--to`'s title gives.
        let new_doc = unit.as_ref().and_then(|unit| {
            let (ty, _) = unit.split_once(':')?;
            let home = homes.as_ref()?.current.get(ty)?;
            let slug = word_after("--slug")
                .or_else(|| to.as_deref().map(engine::slug::slugify))
                .filter(|slug| !slug.is_empty())?;
            Some(home.for_name(&format!("{slug}.md")))
        });
        let retired = unit.as_ref().and_then(|unit| {
            match before
                .files
                .get(&format!("{REPO}/.jigc/tasks/{unit}/source-path"))
            {
                Some(Entry::File { bytes, .. }) => Some(
                    String::from_utf8_lossy(bytes)
                        .trim()
                        .trim_start_matches("./")
                        .to_string(),
                )
                .filter(|path| !path.is_empty()),
                _ => None,
            }
        });
        let mut acting = Acting {
            retired,
            homes,
            homes_after: Homes::over(after),
            the_doc,
            from,
            to,
            new_doc,
            value,
            new_homes: BTreeSet::new(),
            unit,
            sub_tasks,
            settled,
            milestone,
            docs_root,
            stood: Some(stood),
            minted,
            minted_tasks,
            base,
            checked_out: before
                .checkouts
                .keys()
                .filter_map(|path| in_repo(path).map(str::to_string))
                .collect(),
        };
        acting.new_homes = before
            .files
            .keys()
            .filter_map(|path| acting.destination(in_repo(path)?))
            .collect();
        acting
    }

    /// **Where a move into its doctype's home takes the doc at `rel`** — from the
    /// repository's root, before the command: a `<name>.md` under the prior home `--from`
    /// names is the doctype the command names, and any other path the doctype whose home
    /// held it before the command; it lands at its name in that doctype's home as the
    /// cascade resolves it after the command, where that is another path ([`A_NEW_HOME`],
    /// [`May::IntoItsHome`]).
    fn destination(&self, rel: &str) -> Option<String> {
        let name = rel.rsplit('/').next()?;
        if !name.ends_with(".md") {
            return None;
        }
        let ty: &str = if is_at_for(PRIOR_HOME, rel, self) {
            self.unit.as_deref()?
        } else {
            self.homes.as_ref()?.doctype_at(rel)?
        };
        let to = self.homes_after.as_ref()?.current.get(ty)?.for_name(name);
        (to != rel).then_some(to)
    }

    /// Whether the component `want` of a place stands for `got`: `*` for any; [`OWN`] for
    /// the unit, [`ITS_SUB_TASKS`] for a sub-task of it and [`ITS_MILESTONE`] for its
    /// milestone, each alone or with the fixed suffix that follows it; and else the word
    /// itself.
    fn stands_for(&self, want: &str, got: &str) -> bool {
        if want == "*" {
            return true;
        }
        for word in [OWN, ITS_SUB_TASKS, ITS_MILESTONE] {
            if let Some(suffix) = want.strip_prefix(word) {
                let Some(id) = got.strip_suffix(suffix) else {
                    return false;
                };
                return match word {
                    OWN => self.unit.as_deref() == Some(id),
                    ITS_SUB_TASKS => self.sub_tasks.contains(id),
                    _ => self.milestone.as_deref() == Some(id),
                };
            }
        }
        want == got
    }

    /// Whether nothing stood at `path`, from the repository's root, before the command — in
    /// the working tree or in what git held ([`MINTED`], [`RE_SEEDED`]).
    fn nothing_stood(&self, path: &str) -> bool {
        self.stood
            .as_ref()
            .is_some_and(|stood| !stood.contains(path))
    }

    /// `pattern` with a leading [`DOCS_ROOT`] read as the directory it stands for — `None`
    /// where the docs root does not resolve.
    fn rooted<'p>(&self, pattern: &'p str) -> Option<std::borrow::Cow<'p, str>> {
        let Some(rest) = pattern.strip_prefix(DOCS_ROOT) else {
            return Some(pattern.into());
        };
        let rest = rest.strip_prefix('/')?;
        match self.docs_root.as_deref()? {
            "" => Some(rest.into()),
            root => Some(format!("{root}/{rest}").into()),
        }
    }
}

/// **The directory the cascade resolves `docs-root` to**, from the repository's root — the
/// project layer's value, else the team layer's, else the dev pack's default; `.` and `""`
/// the root itself. `None` where a layer that stands is no YAML map, or the knob there is
/// no string. *Not read*: a listed pack's own `knobs.yaml`, which the shipped topology has
/// none of.
fn docs_root(before: &Manifest) -> Option<String> {
    let root = knob(before, "docs-root")?;
    let root = root.trim_matches('/');
    Some(if root == "." {
        String::new()
    } else {
        root.to_string()
    })
}

/// **What the cascade resolves a scalar knob to** over `manifest` — the project layer's
/// value (`.jigc/config/manifest.yaml`, its `scalar:` map), else the team layer's
/// (`~/.config/jigc/manifest.yaml`), else the dev pack's own default (`knobs.yaml`). `None`
/// where a layer that stands is no YAML map, or the knob there is no string.
fn knob(manifest: &Manifest, key: &str) -> Option<String> {
    fn says(bytes: &[u8], at: &[&str]) -> Result<Option<String>, ()> {
        let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_slice(bytes).map_err(|_| ())?;
        if !yaml.is_mapping() {
            return Err(());
        }
        match at.iter().try_fold(&yaml, |value, key| value.get(key)) {
            None => Ok(None),
            Some(value) => value
                .as_str()
                .map(|value| Some(value.to_string()))
                .ok_or(()),
        }
    }
    for layer in [
        format!("{REPO}/.jigc/config/manifest.yaml"),
        "home/.config/jigc/manifest.yaml".to_string(),
    ] {
        match manifest.files.get(&layer) {
            None => continue,
            Some(Entry::File { bytes, .. }) => match says(bytes, &["scalar", key]) {
                Ok(None) => continue,
                Ok(Some(set)) => return Some(set),
                Err(()) => return None,
            },
            Some(_) => return None,
        }
    }
    says(DEV_KNOBS.as_bytes(), &[key, "default"]).ok()?
}

/// **The shipped packs' own files** — their schemas and their schema snapshots, read as a
/// doctype's home is declared there (`location:` or `placement:`), the way the docs root's
/// default is read off the dev pack's `knobs.yaml` and never spelled in the frame.
static SHIPPED_PACKS: include_dir::Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/packs");

/// **Where a doctype's docs stand**, from the repository's root: a directory each of its
/// docs stands in as `<slug>.md` (`""` the root itself), or the one file a placement
/// doctype's doc is.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Home {
    Dir(String),
    File(String),
}

impl Home {
    /// Whether `rel`, from the repository's root, is a doc at this home: the file itself,
    /// or `<slug>.md` directly in the directory.
    fn holds(&self, rel: &str) -> bool {
        match self {
            Home::File(file) => rel == file,
            Home::Dir(dir) => {
                let name = match dir.as_str() {
                    "" => Some(rel),
                    dir => rel
                        .strip_prefix(dir)
                        .and_then(|rest| rest.strip_prefix('/')),
                };
                name.is_some_and(|name| !name.contains('/') && name.ends_with(".md"))
            }
        }
    }

    /// Where a doc whose file is named `name` stands at this home.
    fn for_name(&self, name: &str) -> String {
        match self {
            Home::File(file) => file.clone(),
            Home::Dir(dir) => joined(dir, name),
        }
    }
}

/// `rest` under `dir`, both from the repository's root (`""` the root itself).
fn joined(dir: &str, rest: &str) -> String {
    match dir {
        "" => rest.to_string(),
        dir => format!("{dir}/{rest}"),
    }
}

/// **Every doctype's home, as the cascade resolves it over one manifest** — its current
/// schema's, and every home a snapshot of an earlier schema version declares
/// (`design/corpus-migration.md` → *Prior-schema sourcing — the versioned snapshot store*).
#[derive(Clone, Debug, Default)]
struct Homes {
    current: BTreeMap<String, Home>,
    prior: Vec<(String, Home)>,
    /// Each doctype's current schema version, as its pack's `schema-manifest.yaml` declares
    /// it — the version a migrated doc is stamped with (`design/corpus-migration.md` → *The
    /// schema-version stamp*). A doctype no manifest governs has none.
    versions: BTreeMap<String, u64>,
    /// **Each doctype's shape at each version a schema declares** — its current schema's at
    /// [`Homes::versions`], and a snapshot's (`schema-snapshots/<type>.v<N>.yaml`) at `N`:
    /// what a migration between two versions may change of a doc's own lines
    /// ([`May::Migrated`]).
    shapes: BTreeMap<String, BTreeMap<u64, Shape>>,
    /// **Each doctype's ref-fields**, by its current schema: a field of a header section —
    /// a line of the doc's front matter — that the schema declares `type: ref`, with the
    /// doctype its `to:` names. What a rename may rewrite in a doc that refers to the one it
    /// moves ([`May::Repointed`]): *"rewrites each referrer's structured ref-field
    /// old→new"* (`design/write-commands.md` → *`jigc rename`*).
    refs: BTreeMap<String, BTreeMap<String, String>>,
}

/// **What a schema declares of a doc's own lines that a migration may change**
/// (`design/corpus-migration.md` → *The deterministic transform*, *Relocation*): the
/// `## ` heading of each body section, which an *added-optional-section* splices — *"splice
/// an empty `## <Heading>` slot-section block at the section's schema-ordered offset"* — and
/// the `# ` title a `display-title:` fixes, which a *display-title-changed* rewrites —
/// *"rewrites the doc's H1 line (`# changelog` → `# Changelog`)"*.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Shape {
    /// `## <Heading>` of each section that is no header, as the writer renders its id:
    /// *"the writer title-cases `out-of-scope` → "Out Of Scope""*
    /// (`design/project-setup.md`).
    headings: Vec<String>,
    /// `# <display-title>`, where the schema declares one.
    title: Option<String>,
}

/// The doctype a schema file declares, and its [`Shape`].
fn shaped(bytes: &[u8]) -> Option<(String, Shape)> {
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_slice(bytes).ok()?;
    let ty = yaml.get("type")?.as_str()?.to_string();
    let headings = yaml
        .get("sections")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .into_iter()
        .flatten()
        .filter(|section| {
            !section
                .get("header")
                .and_then(serde_yaml_ng::Value::as_bool)
                .unwrap_or(false)
        })
        .filter_map(|section| section.get("id")?.as_str())
        .map(|id| {
            let words: Vec<String> = id
                .split('-')
                .map(|word| {
                    let mut chars = word.chars();
                    chars.next().map_or_else(String::new, |first| {
                        first.to_ascii_uppercase().to_string() + chars.as_str()
                    })
                })
                .collect();
            format!("## {}", words.join(" "))
        })
        .collect();
    let title = yaml
        .get("display-title")
        .and_then(serde_yaml_ng::Value::as_str)
        .map(|title| format!("# {title}"));
    Some((ty, Shape { headings, title }))
}

/// The doctype a schema file declares, and its ref-fields: each field of a header section
/// declared `type: ref`, with the doctype its `to:` names ([`Homes::refs`]).
fn ref_fields(bytes: &[u8]) -> Option<(String, BTreeMap<String, String>)> {
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_slice(bytes).ok()?;
    let ty = yaml.get("type")?.as_str()?.to_string();
    let refs = yaml
        .get("sections")
        .and_then(serde_yaml_ng::Value::as_sequence)
        .into_iter()
        .flatten()
        .filter(|section| {
            section
                .get("header")
                .and_then(serde_yaml_ng::Value::as_bool)
                .unwrap_or(false)
        })
        .filter_map(|section| section.get("fields")?.as_sequence())
        .flatten()
        .filter(|field| field.get("type").and_then(serde_yaml_ng::Value::as_str) == Some("ref"))
        .filter_map(|field| {
            Some((
                field.get("id")?.as_str()?.to_string(),
                field.get("to")?.as_str()?.to_string(),
            ))
        })
        .collect();
    Some((ty, refs))
}

/// What a schema file declares of its doctype's home.
enum Declared {
    Location(String),
    Placement(String),
}

/// The doctype a schema file declares, and its home as declared — `None` for a schema with
/// neither, a transient doctype.
fn declared(bytes: &[u8]) -> Option<(String, Declared)> {
    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_slice(bytes).ok()?;
    let ty = yaml.get("type")?.as_str()?.to_string();
    if let Some(location) = yaml.get("location").and_then(serde_yaml_ng::Value::as_str) {
        return Some((ty, Declared::Location(location.to_string())));
    }
    let file = yaml.get("placement")?.get("file")?.as_str()?;
    Some((ty, Declared::Placement(file.to_string())))
}

impl Homes {
    /// **The homes over `manifest`**: the dev pack's schemas; the methodology pack's where
    /// the project's `.jigc/config/packs.yaml` composes it (`compose-embedded-methodology:
    /// true`, `design/multi-pack.md`); a listed pack's schemas, read off the manifest under
    /// the directory that file's `packs:` names, from the repository's root; and a project
    /// layer's shadow, `.jigc/config/schemas/<type>.yaml` — each outranking the one before
    /// it. A `location:` stands under the docs root — *"The resolved `docs-root` is
    /// prepended to every persisted doctype's `location:` at schema load"*
    /// (`design/storage.md` → *Config layout*) — and a `placement:` file carrying a leading
    /// directory under `placement-root` where that knob is set: *"a declared home
    /// **carrying a leading directory component** re-roots to
    /// `<placement-root>/<remainder>`"* (→ *Placement — direct-file and root-located homes
    /// (M38)*). `None` where the docs root does not resolve. *Not read*: a pack named by an
    /// absolute path, and a listed pack's own knobs.
    fn over(manifest: &Manifest) -> Option<Homes> {
        let docs_root = docs_root(manifest)?;
        let placement_root = knob(manifest, "placement-root").unwrap_or_default();
        let packs: Option<serde_yaml_ng::Value> = match manifest
            .files
            .get(&format!("{REPO}/.jigc/config/packs.yaml"))
        {
            Some(Entry::File { bytes, .. }) => serde_yaml_ng::from_slice(bytes).ok(),
            _ => None,
        };
        let composes = packs
            .as_ref()
            .and_then(|yaml| yaml.get("compose-embedded-methodology"))
            .and_then(serde_yaml_ng::Value::as_bool)
            .unwrap_or(false);
        let listed: Vec<String> = packs
            .as_ref()
            .and_then(|yaml| yaml.get("packs"))
            .and_then(serde_yaml_ng::Value::as_sequence)
            .map(|packs| {
                packs
                    .iter()
                    .filter_map(serde_yaml_ng::Value::as_str)
                    .filter(|pack| !pack.starts_with('/'))
                    .map(|pack| {
                        pack.trim_start_matches("./")
                            .trim_end_matches('/')
                            .to_string()
                    })
                    .collect()
            })
            .unwrap_or_default();
        // Each file of `kind` in the packs composed, by its name.
        let shipped_named = |kind: &str| -> Vec<(String, Vec<u8>)> {
            let mut packs = vec!["dev"];
            if composes {
                packs.push("methodology");
            }
            packs
                .into_iter()
                .filter_map(|pack| SHIPPED_PACKS.get_dir(format!("{pack}/{kind}")))
                .flat_map(|dir| {
                    dir.files().map(|file| {
                        let name = file
                            .path()
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        (name, file.contents().to_vec())
                    })
                })
                .collect()
        };
        let shipped = |kind: &str| -> Vec<Vec<u8>> {
            shipped_named(kind)
                .into_iter()
                .map(|(_, bytes)| bytes)
                .collect()
        };
        let from_the_manifest = |dir: &str| -> Vec<Vec<u8>> {
            let prefix = format!("{REPO}/{dir}/");
            manifest
                .files
                .iter()
                .filter(|(path, _)| {
                    path.strip_prefix(&prefix)
                        .is_some_and(|name| !name.contains('/') && name.ends_with(".yaml"))
                })
                .filter_map(|(_, entry)| match entry {
                    Entry::File { bytes, .. } => Some(bytes.clone()),
                    _ => None,
                })
                .collect()
        };
        let mut schemas = shipped("schemas");
        // A listed pack outranks the shipped ones, the first of the list the highest: read
        // lowest first, so a later one wins.
        for pack in listed.iter().rev() {
            schemas.extend(from_the_manifest(&format!("{pack}/schemas")));
        }
        schemas.extend(from_the_manifest(".jigc/config/schemas"));
        let resolve = |declared: Declared| match declared {
            Declared::Location(dir) => Home::Dir(joined(&docs_root, dir.trim_matches('/'))),
            Declared::Placement(file) => Home::File(re_rooted(&placement_root, &file)),
        };
        let mut homes = Homes::default();
        for manifest in shipped("config") {
            let Ok(yaml) = serde_yaml_ng::from_slice::<serde_yaml_ng::Value>(&manifest) else {
                continue;
            };
            for doctype in yaml
                .get("doctypes")
                .and_then(serde_yaml_ng::Value::as_sequence)
                .into_iter()
                .flatten()
            {
                if let (Some(ty), Some(version)) = (
                    doctype.get("type").and_then(serde_yaml_ng::Value::as_str),
                    doctype
                        .get("schema-version")
                        .and_then(serde_yaml_ng::Value::as_u64),
                ) {
                    homes.versions.insert(ty.to_string(), version);
                }
            }
        }
        for schema in schemas {
            if let Some((ty, home)) = declared(&schema) {
                homes.current.insert(ty, resolve(home));
            }
            if let Some((ty, refs)) = ref_fields(&schema) {
                homes.refs.insert(ty, refs);
            }
            if let Some((ty, shape)) = shaped(&schema)
                && let Some(version) = homes.versions.get(&ty)
            {
                homes.shapes.entry(ty).or_default().insert(*version, shape);
            }
        }
        for (name, snapshot) in shipped_named("schema-snapshots") {
            let version = name
                .strip_suffix(".yaml")
                .and_then(|stem| stem.rsplit_once(".v"))
                .and_then(|(_, version)| version.parse::<u64>().ok());
            if let (Some(version), Some((ty, shape))) = (version, shaped(&snapshot)) {
                homes
                    .shapes
                    .entry(ty)
                    .or_default()
                    .entry(version)
                    .or_insert(shape);
            }
            if let Some((ty, home)) = declared(&snapshot) {
                let home = resolve(home);
                if homes.current.contains_key(&ty)
                    && homes.current.get(&ty) != Some(&home)
                    && !homes.prior.contains(&(ty.clone(), home.clone()))
                {
                    homes.prior.push((ty, home));
                }
            }
        }
        Some(homes)
    }

    /// The doctype whose current home holds `rel` — `None` where none does, or two do.
    fn doctype_at(&self, rel: &str) -> Option<&str> {
        let at: Vec<&str> = self
            .current
            .iter()
            .filter(|(_, home)| home.holds(rel))
            .map(|(ty, _)| ty.as_str())
            .collect();
        match at.as_slice() {
            [one] => Some(one),
            _ => None,
        }
    }

    /// The doctype one of whose earlier homes holds `rel`, where no current home does.
    fn prior_doctype_at(&self, rel: &str) -> Option<&str> {
        if self.current.values().any(|home| home.holds(rel)) {
            return None;
        }
        let at: BTreeSet<&str> = self
            .prior
            .iter()
            .filter(|(_, home)| home.holds(rel))
            .map(|(ty, _)| ty.as_str())
            .collect();
        match at.len() {
            1 => at.into_iter().next(),
            _ => None,
        }
    }
}

/// A declared placement file under `placement-root` — `""` unset, every declared home
/// standing; `.` the repository's root; else that directory — where it carries a leading
/// directory component; a home declared at the root is never re-rooted.
fn re_rooted(root: &str, file: &str) -> String {
    let root = root.trim_matches('/');
    match (root, file.split_once('/')) {
        ("", _) | (_, None) => file.to_string(),
        (".", Some((_, rest))) => rest.to_string(),
        (root, Some((_, rest))) => format!("{root}/{rest}"),
    }
}

/// **A milestone's committed record, as the frame reads it**: the sub-task ids its items
/// name, `### <id>  {#<id>}`, the sub-task's id its anchor; of those, the items it has
/// settled — a `- status: ` leaf of the item, `joined` or `discarded`, *"active / joined /
/// discarded"*, the two that are over (`design/team-ready-state.md` → *The lifecycle*); and
/// the shared base its front matter pins, `base: <sha> <short>`, the full id.
#[derive(Clone, Debug, Default)]
struct Record {
    items: BTreeSet<String>,
    settled: BTreeSet<String>,
    base: Option<String>,
}

/// Where a milestone's record stands, from the repository's root, under the docs root
/// `root` (`""` the root itself).
fn record_path(root: &str, id: &str) -> String {
    match root {
        "" => format!("{RECORD_HOME}/{id}.md"),
        root => format!("{root}/{RECORD_HOME}/{id}.md"),
    }
}

/// Every record at the home under the docs root `root` whose bytes in the working tree are
/// the ones the main checkout's `HEAD` holds ([`committed`]), by its milestone.
fn committed_records(manifest: &Manifest, root: &str) -> BTreeMap<String, Record> {
    let home = match root {
        "" => RECORD_HOME.to_string(),
        root => format!("{root}/{RECORD_HOME}"),
    };
    manifest
        .files
        .keys()
        .filter_map(|path| {
            let rel = in_repo(path)?;
            let id = rel
                .strip_prefix(home.as_str())?
                .strip_prefix('/')?
                .strip_suffix(".md")?;
            (!id.contains('/')).then_some((id.to_string(), committed(manifest, rel)?))
        })
        .map(|(id, entry)| {
            let mut record = Record::default();
            let mut item: Option<String> = None;
            // The front matter: between a first line `---` and the next.
            let mut front = None;
            for (at, line) in entry.lines().unwrap_or_default().into_iter().enumerate() {
                let Ok(line) = std::str::from_utf8(line) else {
                    continue;
                };
                let line = line.trim_end();
                match (at, front, line) {
                    (0, None, "---") => front = Some(true),
                    (_, Some(true), "---") => front = Some(false),
                    (_, Some(true), line) => {
                        if let Some(value) = line.strip_prefix("base: ") {
                            record.base = value.split_whitespace().next().map(str::to_string);
                        }
                    }
                    _ => {}
                }
                if front == Some(true) {
                    continue;
                }
                if let Some(heading) = line.strip_prefix("### ") {
                    item = heading
                        .rsplit_once("{#")
                        .and_then(|(_, anchor)| anchor.strip_suffix('}'))
                        .map(str::to_string);
                    record.items.extend(item.clone());
                } else if let (Some(at), Some(status)) =
                    (item.as_ref(), line.strip_prefix("- status: "))
                    && matches!(status.trim(), "joined" | "discarded")
                {
                    record.settled.insert(at.clone());
                }
            }
            (id, record)
        })
        .collect()
}

/// The entry at `rel` — from the repository's root — where its bytes are the ones the main
/// checkout's `HEAD` holds there: the blob of the commit that last changed the path, down
/// its first parents.
fn committed<'m>(manifest: &'m Manifest, rel: &str) -> Option<&'m Entry> {
    let entry = manifest.files.get(&format!("{REPO}/{rel}"))?;
    let mut at = manifest.checkouts.get(REPO)?.head.as_str();
    while !at.is_empty() {
        let commit = manifest.commits.get(at)?;
        if let Some(held) = commit.changed.get(rel) {
            let (_, blob) = held.as_ref()?;
            return (entry.blob() == Some(blob.as_str())).then_some(entry);
        }
        at = commit.parent.as_deref().unwrap_or_default();
    }
    None
}

/// Whether `path` is `pattern`, component for component, a `*` standing for any one — and,
/// for a command that acts on a unit, [`OWN`], [`ITS_SUB_TASKS`] and [`ITS_MILESTONE`] for
/// what [`Acting`] reads, and a leading [`DOCS_ROOT`] for the docs root.
fn is_at_for(pattern: &str, path: &str, acting: &Acting) -> bool {
    match pattern {
        THE_DOC => return acting.the_doc.as_ref().is_some_and(|(_, at)| at == path),
        THE_NEW_DOC => return acting.new_doc.as_deref() == Some(path),
        THE_RETIRED => return acting.retired.as_deref() == Some(path),
        A_NEW_HOME => return acting.new_homes.contains(path),
        A_DOC => {
            return [&acting.homes, &acting.homes_after]
                .into_iter()
                .flatten()
                .any(|homes| homes.doctype_at(path).is_some());
        }
        A_PRIOR_DOC => {
            return acting
                .homes
                .as_ref()
                .is_some_and(|homes| homes.prior_doctype_at(path).is_some());
        }
        PRIOR_HOME => {
            return acting.from.as_ref().is_some_and(|from| {
                !from.is_empty()
                    && (path == from
                        || path
                            .strip_prefix(from.as_str())
                            .is_some_and(|rest| rest.starts_with('/')))
            });
        }
        _ => {}
    }
    let Some(pattern) = acting.rooted(pattern) else {
        return false;
    };
    let (want, got): (Vec<&str>, Vec<&str>) =
        (pattern.split('/').collect(), path.split('/').collect());
    want.len() == got.len()
        && want.iter().zip(&got).enumerate().all(|(at, (want, part))| {
            let id_with = |word: &str| want.strip_prefix(word).and_then(|s| part.strip_suffix(s));
            if want.starts_with(MINTED_TASK) {
                id_with(MINTED_TASK).is_some_and(|id| {
                    acting.minted_tasks.len() == 1 && acting.minted_tasks.contains(id)
                }) && acting.nothing_stood(&got_so_far(&got, at))
            } else if want.starts_with(EACH_MINTED) {
                id_with(EACH_MINTED).is_some_and(|id| acting.minted.contains(id))
                    && acting.nothing_stood(&got_so_far(&got, at))
            } else if want.starts_with(MINTED) {
                id_with(MINTED)
                    .is_some_and(|id| acting.minted.len() == 1 && acting.minted.contains(id))
                    && acting.nothing_stood(&got_so_far(&got, at))
            } else if want.starts_with(RE_SEEDED) {
                id_with(RE_SEEDED)
                    .is_some_and(|id| acting.sub_tasks.contains(id) && !acting.settled.contains(id))
                    && acting.nothing_stood(&got_so_far(&got, at))
            } else if want.starts_with(UNPROVISIONED) {
                id_with(UNPROVISIONED)
                    .is_some_and(|id| acting.sub_tasks.contains(id) && !acting.settled.contains(id))
                    && !acting.checked_out.contains(&got_so_far(&got, at))
            } else {
                acting.stands_for(want, part)
            }
        })
}

/// The first `at + 1` components of a path, joined.
fn got_so_far(got: &[&str], at: usize) -> String {
    got[..=at].join("/")
}

/// Whether `path` is the directory `pattern` names or lies under it.
fn under_for(pattern: &str, path: &str, acting: &Acting) -> bool {
    let Some(pattern) = acting.rooted(pattern) else {
        return false;
    };
    let depth = pattern.split('/').count();
    let head: Vec<&str> = path.split('/').take(depth).collect();
    head.len() == depth && is_at_for(&pattern, &head.join("/"), acting)
}

/// **Whether `path` is a directory on the way to the one `pattern` names** — a path of its
/// fixed head, the components before the first that stands for something: what a move
/// into `.jigc/displaced/<sub-task>` makes before it reaches a unit's place.
fn on_the_way_to(pattern: &str, path: &str, acting: &Acting) -> bool {
    let Some(pattern) = acting.rooted(pattern) else {
        return false;
    };
    let head: Vec<&str> = pattern
        .split('/')
        .take_while(|word| !word.starts_with('<') && *word != "*")
        .collect();
    let got: Vec<&str> = path.split('/').collect();
    got.len() <= head.len() && got == head[..got.len()]
}

/// [`under_for`] with no unit: a place that names one is nowhere.
fn under(pattern: &str, path: &str) -> bool {
    under_for(pattern, path, &Acting::default())
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
    /// The work unit the command acts on, which [`OWN`] and [`ITS_SUB_TASKS`] stand for.
    acting: Acting,
    acted: bool,
    seen: &'a Seen<'a>,
    /// The ids of the docs staged in the unit the command acts on, before the command
    /// ([`staged_in_the_unit`]).
    staged: BTreeSet<&'a str>,
    excused: Vec<usize>,
    lost: Vec<String>,
    git: Vec<String>,
    outside: Vec<String>,
}

impl<'a> Tally<'a> {
    /// The lines in force: their condition holds, and the command acted or the line holds
    /// on a refusal.
    fn in_force(&self) -> impl Iterator<Item = (usize, &'a Line)> + '_ {
        let head = self
            .seen
            .before
            .checkouts
            .get(REPO)
            .map(|main| main.head.as_str())
            .unwrap_or_default();
        let unborn = head.is_empty();
        // What `HEAD` held before the command, read once and only for a line that asks.
        let committed = std::cell::OnceCell::new();
        let first_commit_of = move |allows: Allows| {
            let Allows::Files(place, _) = allows else {
                return false;
            };
            !committed
                .get_or_init(|| tree_of(self.seen.before, head))
                .keys()
                .any(|path| self.placed(place, &format!("{REPO}/{path}")))
        };
        // Under the spelling the cell names: its own lines, and every line of no spelling
        // but those it replaces.
        let spelled = |line: &Line| match line.spelling {
            Some(name) => self.seen.spelling == Some(name),
            None => !self.row.spellings.iter().any(|spelling| {
                self.seen.spelling == Some(spelling.name) && spelling.replaces.contains(&line.id)
            }),
        };
        self.row.lines.iter().enumerate().filter(move |(_, line)| {
            spelled(line)
                && (self.acted || line.on_refusal)
                && match line.when {
                    When::Always => true,
                    When::NoCommitYet => unborn,
                    When::FirstCommitOfItsPlace => first_commit_of(line.allows),
                    When::Refused => !self.acted,
                    When::ItCommitted => self.acted || self.made_a_commit(),
                    When::ADocLeftAndNamed => self.acted || self.left_a_doc_and_named_it(),
                }
        })
    }

    /// **Whether the command left a doc of its sweep where it stood, and named it**
    /// ([`When::ADocLeftAndNamed`]): an entry with a doc's name stood under the prior home
    /// `--from` names before the command and stands there after as it stood, and what the
    /// command said names its path, or the path its move would have landed it at.
    fn left_a_doc_and_named_it(&self) -> bool {
        self.seen.before.files.iter().any(|(path, was)| {
            let Some(rel) = in_repo(path) else {
                return false;
            };
            rel.ends_with(".md")
                && is_at_for(PRIOR_HOME, rel, &self.acting)
                && self
                    .seen
                    .after
                    .files
                    .get(path)
                    .is_some_and(|now| now.same(was))
                && (self.seen.ended.said.contains(rel)
                    || self
                        .acting
                        .destination(rel)
                        .is_some_and(|to| self.seen.ended.said.contains(to.as_str())))
        })
    }

    /// **Whether the command left a commit of its own on the checked-out branch** — the
    /// main checkout's `HEAD` moved, on the same branch, to a commit whose first parents
    /// reach the one it stood at.
    fn made_a_commit(&self) -> bool {
        let (Some(was), Some(is)) = (
            self.seen.before.checkouts.get(REPO),
            self.seen.after.checkouts.get(REPO),
        ) else {
            return false;
        };
        if was.head == is.head || was.branch != is.branch || is.head.is_empty() {
            return false;
        }
        let mut at = Some(is.head.as_str());
        let mut steps = 0;
        while let Some(id) = at
            && steps <= self.seen.after.commits.len()
        {
            if id == was.head {
                return true;
            }
            at = self
                .seen
                .after
                .commits
                .get(id)
                .and_then(|commit| commit.parent.as_deref());
            steps += 1;
        }
        was.head.is_empty() && at.is_none()
    }

    /// **Whether `line` is in force by the command's commit alone** — a line under
    /// [`When::ItCommitted`] on a refusal: what it covers is what the commit landed
    /// ([`Tally::as_it_landed`]).
    fn by_its_commit_alone(&self, line: &Line) -> bool {
        line.when == When::ItCommitted && !self.acted
    }

    /// **Whether `now` — the id of a regular file's bytes, or nothing — at `rel`, from the
    /// repository's root, is what the command's commit left git tracking there**: the tree
    /// the main checkout's `HEAD` holds after the command holds that file there, or nothing
    /// where `now` is nothing. A path git tracks nothing of — no `HEAD` before or after the
    /// command holds it, nor the index — is none of it, and holds.
    fn as_it_landed(&self, rel: &str, now: Option<&str>) -> bool {
        let head = |manifest: &Manifest| {
            manifest
                .checkouts
                .get(REPO)
                .map(|main| main.head.clone())
                .unwrap_or_default()
        };
        let (was, is) = (self.seen.before, self.seen.after);
        let landed = tree_of(is, &head(is));
        let tracked = landed.contains_key(rel)
            || tree_of(was, &head(was)).contains_key(rel)
            || [was, is].into_iter().any(|manifest| {
                manifest
                    .checkouts
                    .get(REPO)
                    .is_some_and(|main| main.index.contains_key(rel))
            });
        if !tracked {
            return true;
        }
        match landed.get(rel) {
            None => now.is_none(),
            Some((mode, blob)) => mode & 0o170_000 == 0o100_000 && now == Some(blob.as_str()),
        }
    }

    /// Whether `place` is where `path` — from the sandbox's root — stands.
    fn placed(&self, place: Where, path: &str) -> bool {
        match (place, in_repo(path)) {
            (Where::Hook, _) => path == format!("{}/pre-commit", self.seen.before.hooks),
            (Where::Path(pattern), Some(rel)) => is_at_for(pattern, rel, &self.acting),
            (Where::Under(pattern), Some(rel)) => under_for(pattern, rel, &self.acting),
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
                if self.by_its_commit_alone(line)
                    && in_repo(diff.path).is_some_and(|rel| {
                        !self.as_it_landed(rel, diff.after.and_then(Entry::blob))
                    })
                {
                    return false;
                }
                let aside = |dir: &str, path: &str| {
                    in_repo(path).is_some_and(|rel| under_for(dir, rel, &self.acting))
                };
                let on_the_way = |dir: &str, path: &str| {
                    matches!(diff.after, Some(Entry::Dir(_)))
                        && in_repo(path).is_some_and(|rel| on_the_way_to(dir, rel, &self.acting))
                };
                if let May::MovedAsideTo(dir) = may
                    && (aside(dir, diff.path) || on_the_way(dir, diff.path))
                {
                    // The arrival: a directory, or an entry that left a place the line parks
                    // from — gone from it, or something else standing there now — under its
                    // own unit's place; at one file's place, only a destination's foreign
                    // occupant.
                    let unit = |from: &str| {
                        in_repo(from)
                            .zip(in_repo(diff.path))
                            .is_some_and(|(from, to)| self.of_its_unit(place, from, dir, to))
                    };
                    return diff.before.is_none()
                        && diff.after.is_some_and(|arrived| {
                            matches!(arrived, Entry::Dir(_))
                                || self.seen.before.files.iter().any(|(from, left)| {
                                    left.same(arrived)
                                        && self.parks(place, from)
                                        && unit(from)
                                        && self
                                            .seen
                                            .after
                                            .files
                                            .get(from)
                                            .is_none_or(|now| !now.same(left))
                                })
                        });
                }
                if !self.placed(place, diff.path) {
                    return false;
                }
                let rel = in_repo(diff.path).unwrap_or_default();
                if matches!(may, May::Rebuilt | May::Invalidated(_))
                    && in_repo(diff.path).is_some_and(|rel| NO_CACHE.contains(&rel))
                {
                    return false;
                }
                if matches!(diff.after, Some(Entry::File { .. }))
                    && in_repo(diff.path).is_some_and(deleted_after)
                {
                    return false;
                }
                match (may, diff.before, diff.after) {
                    (May::WriteWhole, _, Some(Entry::File { .. })) => true,
                    (May::WriteWhole, _, _) => matches!(place, Where::Under(_)),
                    (
                        May::MergeInto
                        | May::TakeOutOf
                        | May::Repointed
                        | May::Migrated
                        | May::ItsKeySet,
                        was,
                        Some(is @ Entry::File { .. }),
                    ) => self.fits(may, rel, was, is, false),
                    (May::Retitled, Some(Entry::File { .. }), Some(Entry::File { .. })) => self
                        .retitle()
                        .is_some_and(|(old, new)| old == rel && new == old),
                    (May::Retitled, Some(Entry::File { .. }), None) => self
                        .retitle()
                        .is_some_and(|(old, new)| old == rel && new != old),
                    (May::Retitled, None, Some(Entry::File { .. })) => self
                        .retitle()
                        .is_some_and(|(old, new)| new == rel && new != old),
                    // A doc moved into its home: gone from its place, and arrived at a
                    // mover's destination, where nothing stood.
                    (May::IntoItsHome, Some(_), None) => self.moved_home(rel).is_some(),
                    (May::IntoItsHome, None, Some(is)) => {
                        matches!(place, Where::Path(A_NEW_HOME)) && self.arrived_home(rel, is)
                    }
                    (May::Migrated, Some(was @ Entry::File { .. }), None) => {
                        self.migrated_away(rel, was)
                    }
                    (May::Removed, Some(_), None) => true,
                    (May::Retired, Some(Entry::File { blob, .. }), None) => {
                        self.as_head_held(rel, Some(blob))
                    }
                    (May::ReSlugged, Some(_), None) => self.re_slugged(diff.path),
                    (May::Rebuilt | May::Invalidated(_), _, Some(Entry::File { .. })) => true,
                    (
                        May::Rebuilt | May::Invalidated(_) | May::LaidOut,
                        None,
                        Some(Entry::Dir(_)),
                    ) => true,
                    (May::Invalidated(stamped), Some(Entry::File { .. }), None) => {
                        in_repo(diff.path) == Some(stamped)
                    }
                    (May::Provisioned, None, Some(Entry::File { .. } | Entry::Dir(_))) => true,
                    (May::FromTheBasesTree, None, Some(Entry::Dir(_))) => true,
                    (May::FromTheBasesTree, None, Some(Entry::File { mode, blob, .. })) => {
                        self.of_the_bases_tree(place, diff.path, *mode, blob)
                    }
                    (May::CopyOfAStagedDoc, _, Some(is)) => {
                        is.blob().is_some_and(|blob| self.staged.contains(blob))
                    }
                    (May::MovedAsideTo(dir), Some(left), None) => {
                        self.parks(place, diff.path)
                            && self.seen.after.files.iter().any(|(to, arrived)| {
                                aside(dir, to)
                                    && arrived.same(left)
                                    && !self.seen.before.files.contains_key(to)
                                    && in_repo(to)
                                        .is_some_and(|to| self.of_its_unit(place, rel, dir, to))
                            })
                    }
                    _ => false,
                }
            })
            .map(|(at, _)| at)
    }

    /// **Whether what a retire takes away at `rel` is what `HEAD` holds there** — from the
    /// repository's root, before the command ([`May::Retired`]). With a `blob`, the working
    /// tree's regular file there: `HEAD`'s tree holds a regular file at the path with those
    /// bytes. Without one, the index: it held at the path `HEAD`'s entry, or nothing.
    fn as_head_held(&self, rel: &str, blob: Option<&str>) -> bool {
        let before = self.seen.before;
        let Some(main) = before.checkouts.get(REPO) else {
            return false;
        };
        let tree = tree_of(before, &main.head);
        let head = tree.get(rel);
        match blob {
            Some(blob) => {
                head.is_some_and(|(mode, held)| mode & 0o170_000 == 0o100_000 && held == blob)
            }
            None => main.index.get(rel).is_none_or(|staged| {
                head.is_some_and(|(mode, held)| *mode == staged.mode && *held == staged.blob)
            }),
        }
    }

    /// **What `HEAD` held at `rel`**, from the repository's root, before the command — a
    /// regular file with its bytes, or nothing: what a word's content rule reads a commit's
    /// file against. `Err` where `HEAD` holds an entry there whose
    /// bytes the manifest does not carry: what it held cannot be told.
    fn head_held(&self, rel: &str) -> Result<Option<Entry>, ()> {
        let before = self.seen.before;
        let head = before
            .checkouts
            .get(REPO)
            .map(|main| main.head.as_str())
            .unwrap_or_default();
        match tree_of(before, head).get(rel) {
            None => Ok(None),
            Some((mode, blob)) => before
                .blobs
                .get(blob)
                .map(|bytes| {
                    Some(Entry::File {
                        mode: mode & 0o7777,
                        blob: blob.clone(),
                        bytes: bytes.clone(),
                    })
                })
                .ok_or(()),
        }
    }

    /// **Whether `is` may stand at `rel`** — from the repository's root — **over `was`**,
    /// by the content rule of a word that has one: *merged into*, *taken out of*,
    /// *repointed*, *migrated*, *its knob set*. Read over the bytes the working tree held
    /// there before the command where the working tree changed, and over the bytes `HEAD`
    /// held where a commit holds the path — `committed`, which a migration from a prior home
    /// reads its source by too.
    fn fits(&self, may: May, rel: &str, was: Option<&Entry>, is: &Entry, committed: bool) -> bool {
        let Some(now) = is.lines() else {
            return false;
        };
        match may {
            May::MergeInto => was
                .map(Entry::lines)
                .is_none_or(|lines| lines.is_some_and(|was| stand_in(&was, &now))),
            May::TakeOutOf => was
                .and_then(Entry::lines)
                .is_some_and(|was| stand_in(&now, &was)),
            May::Repointed => was.is_some_and(|was| self.repointed(rel, was, is)),
            May::Migrated => self.migrated_into(rel, was, is, committed),
            May::ItsKeySet => self.its_knob_set(was, is),
            _ => false,
        }
    }

    /// **The doc the address names, retitled** ([`May::Retitled`]): its home before the
    /// command, and where it stands after — its new home ([`THE_NEW_DOC`]), the slug the
    /// command gives: that same path, rewritten, or another of its home directory at which
    /// nothing stood, its own gone. The doc there holds the lines it held but its `# ` title
    /// line, and that line is the title `--to` gives; an unchanged file is no retitle in
    /// place. `None` where it does not stand so.
    fn retitle(&self) -> Option<(String, String)> {
        let (_, old) = self.acting.the_doc.as_ref()?;
        let new = self.acting.new_doc.as_ref()?;
        let was = self.seen.before.files.get(&format!("{REPO}/{old}"))?;
        let is = self.seen.after.files.get(&format!("{REPO}/{new}"))?;
        let kept = but_its_title(was)?;
        let titled = match &self.acting.to {
            None => true,
            Some(to) => is
                .lines()
                .and_then(|lines| lines.into_iter().find(|line| line.starts_with(b"# ")))
                .is_some_and(|line| {
                    line.strip_suffix(b"\n").unwrap_or(line) == format!("# {to}").as_bytes()
                }),
        };
        let moved = if new == old {
            !is.same(was)
        } else {
            self.acting.nothing_stood(new)
                && !self.seen.after.files.contains_key(&format!("{REPO}/{old}"))
        };
        (matches!(is, Entry::File { .. })
            && moved
            && but_its_title(is).as_ref() == Some(&kept)
            && titled)
            .then(|| (old.clone(), new.clone()))
    }

    /// **Whether `is` is `was` repointed** ([`May::Repointed`]) — the doc of the store at
    /// `rel`, from the repository's root: the doc the address names moved, and each line that
    /// changed is a ref-field of the front matter of `was` that held the old identity among
    /// its values, whole, and holds those values in the field grammar's canonical form with
    /// that one the new identity — the slug its new home names — every other line as it
    /// stood, none added or gone, and one line at least changed. The ref-fields are the ones
    /// the schema of the doctype whose home holds `rel` declares `type: ref` to the moved
    /// doc's doctype ([`Homes::refs`]).
    fn repointed(&self, rel: &str, was: &Entry, is: &Entry) -> bool {
        let (Some((old, new)), Some((identity, _))) =
            (self.retitle(), self.acting.the_doc.as_ref())
        else {
            return false;
        };
        let (Some(ty), Some(slug)) = (
            identity.split(':').next(),
            new.rsplit('/')
                .next()
                .and_then(|name| name.strip_suffix(".md")),
        ) else {
            return false;
        };
        if old == new {
            return false;
        }
        let fresh = format!("{ty}:{slug}");
        let Some(fields) = self.acting.homes.as_ref().and_then(|homes| {
            let of = homes.doctype_at(rel)?;
            Some(
                homes
                    .refs
                    .get(of)?
                    .iter()
                    .filter(|(_, to)| to.as_str() == ty)
                    .map(|(field, _)| field.as_str())
                    .collect::<BTreeSet<&str>>(),
            )
        }) else {
            return false;
        };
        let (Some(was), Some(is)) = (was.lines(), is.lines()) else {
            return false;
        };
        if was.len() != is.len() {
            return false;
        }
        let front = front_matter(&was);
        let mut changed = false;
        for (at, (before, after)) in was.iter().zip(&is).enumerate() {
            if before == after {
                continue;
            }
            if !front.contains(&at) {
                return false;
            }
            let Some(line) = std::str::from_utf8(before)
                .ok()
                .map(|line| line.strip_suffix('\n').unwrap_or(line))
            else {
                return false;
            };
            let Some((key, value)) = line.split_once(':') else {
                return false;
            };
            let key = key.trim();
            if !fields.contains(key) {
                return false;
            }
            let value = value.trim();
            let rendered = match value
                .strip_prefix('[')
                .and_then(|inner| inner.strip_suffix(']'))
            {
                Some(inner) => {
                    let elements: Vec<&str> = inner.split(',').map(str::trim).collect();
                    if !elements.contains(&identity.as_str()) {
                        return false;
                    }
                    let swapped: Vec<&str> = elements
                        .into_iter()
                        .map(|element| {
                            if element == identity {
                                fresh.as_str()
                            } else {
                                element
                            }
                        })
                        .collect();
                    format!("[{}]", swapped.join(", "))
                }
                None if value == identity => fresh.clone(),
                None => return false,
            };
            let ends = if before.ends_with(b"\n") { "\n" } else { "" };
            if *after != format!("{key}: {rendered}{ends}").as_bytes() {
                return false;
            }
            changed = true;
        }
        changed
    }

    /// **Whether what stood at `rel` before the command is a doc a move into its home takes**
    /// ([`May::IntoItsHome`]), from the repository's root: a regular file in the working
    /// tree, at a path where git held a regular file — `HEAD`'s tree or the index, what `git
    /// ls-files` answers the movers' sweeps with. *"every **committed** `.md` under it"*
    /// (`design/corpus-migration.md` → *The freeze-exempt sibling*), *"A relocation **lands
    /// a regular file** or refuses"* (`design/finalize.md`). Added 2026-10-20, after the
    /// review of the task `B9` (H2): a `.md` of the user's git never held, and a link, each
    /// moved into a home, held.
    fn a_doc_git_holds(&self, rel: &str) -> bool {
        let before = self.seen.before;
        let regular = |mode: u32| mode & 0o170_000 == 0o100_000;
        matches!(
            before.files.get(&format!("{REPO}/{rel}")),
            Some(Entry::File { .. })
        ) && before.checkouts.get(REPO).is_some_and(|main| {
            main.index
                .get(rel)
                .is_some_and(|staged| regular(staged.mode))
                || tree_of(before, &main.head)
                    .get(rel)
                    .is_some_and(|(mode, _)| regular(*mode))
        })
    }

    /// **Where a doc moved into its doctype's home stands** ([`May::IntoItsHome`]): the path
    /// `rel` held before the command — a regular file git held ([`Self::a_doc_git_holds`]) —
    /// the same entry standing after at its destination
    /// ([`Acting::destination`]) — its name in the home of the doctype the command names, for
    /// a doc under the prior home `--from` names, and else of the one whose home held it,
    /// as the cascade resolves it after the command.
    fn moved_home(&self, rel: &str) -> Option<String> {
        let was = self.seen.before.files.get(&format!("{REPO}/{rel}"))?;
        if !self.a_doc_git_holds(rel) {
            return None;
        }
        let to = self.acting.destination(rel)?;
        let is = self.seen.after.files.get(&format!("{REPO}/{to}"))?;
        is.same(was).then_some(to)
    }

    /// **Whether `is`, at `rel` after the command, arrived there by a move into its home**
    /// ([`May::IntoItsHome`]): a path that held the same entry before the command — a regular
    /// file git held ([`Self::a_doc_git_holds`]) — and is gone after, has `rel` for its
    /// destination ([`Acting::destination`]) — **and nothing
    /// stood at `rel` before, in the working tree or in what git held**: what stands at a
    /// mover's destination blocks the move, and nothing is moved aside from one (the human's
    /// ruling of 2026-10-20 on the rows' record's §6, entry 23, where a foreign occupant
    /// parked under `.jigc/displaced/` let the move land).
    fn arrived_home(&self, rel: &str, is: &Entry) -> bool {
        self.acting.nothing_stood(rel)
            && self.seen.before.files.iter().any(|(from, left)| {
                left.same(is)
                    && !self.seen.after.files.contains_key(from)
                    && in_repo(from).is_some_and(|from| {
                        self.acting.destination(from).as_deref() == Some(rel)
                            && self.a_doc_git_holds(from)
                    })
            })
    }

    /// **Whether `to` — an arrival under the directory `dir` a line moves aside to — is under
    /// the place of the unit `from` left** ([`May::MovedAsideTo`]), each from the
    /// repository's root: every component `dir` names by a unit's word ([`OWN`],
    /// [`ITS_SUB_TASKS`]) that the line's `place` names too stands in `to` for what it
    /// stands for in `from`. A word only `dir` names reads as a place's does; a place that
    /// names none binds nothing. **And with its relative path intact** ([`Tally::intact`]).
    fn of_its_unit(&self, place: Where, from: &str, dir: &str, to: &str) -> bool {
        let (Where::Under(pattern) | Where::Path(pattern)) = place else {
            return true;
        };
        let (Some(pattern), Some(dir)) = (self.acting.rooted(pattern), self.acting.rooted(dir))
        else {
            return false;
        };
        let source: Vec<&str> = pattern.split('/').collect();
        let (from_at, to_at): (Vec<&str>, Vec<&str>) =
            (from.split('/').collect(), to.split('/').collect());
        dir.split('/').enumerate().all(|(at, word)| {
            if !word.starts_with('<') {
                return true;
            }
            match source.iter().position(|named| *named == word) {
                None => true,
                Some(left) => from_at
                    .get(left)
                    .is_some_and(|unit| to_at.get(at) == Some(unit)),
            }
        }) && self.intact(place, &pattern, from, &dir, to)
    }

    /// **Whether `to` keeps the relative path `from` had in the place it left**, each from the
    /// repository's root — *"to `.jigc/displaced/<task-id>/` with its relative paths
    /// intact"* (`design/finalize.md` → *7. Post-commit*), *"`.jigc/displaced/<unit-id>/
    /// <relative>`"* (`design/storage.md` → *The per-task working area*): the path under
    /// `dir` is the path `from` had under its place — under a directory, the components
    /// after the last one of the place that `dir` names too, else after the place's fixed
    /// head; at one file's place, the parked entry's name and what lies in it
    /// ([`Tally::parks`], a directory parked whole). **One component may carry `.<n>`**,
    /// `n` from 2, where the path through the same component without it stands after the
    /// command — the no-clobber rule of both producers: *"under the first free `<name>.<n>`
    /// where that name is already taken, so nothing parked is ever replaced"*
    /// (`design/storage.md` → *Repository layout & state location*, the `displaced/` line;
    /// the same in `design/reconciliation.md` → *Relocation collisions*), the product's
    /// `free_displacement_path`. Added 2026-10-20, by the eighth of the fixes after the
    /// review of the task `B8`, the seventh fix's left-open: a unit's file under its own
    /// place at another name held.
    fn intact(&self, place: Where, pattern: &str, from: &str, dir: &str, to: &str) -> bool {
        let from: Vec<&str> = from.split('/').collect();
        let to: Vec<&str> = to.split('/').collect();
        let depth = dir.split('/').count();
        let Some(moved) = to.get(depth..) else {
            return false;
        };
        // Under a directory, the path below the last component of the place that `dir` names
        // too — a unit's word, which the arrival carries in `dir` — and else below the
        // place's fixed head: a component the line's directory does not name travels with
        // the bytes (`.jigc/tasks/*` aside to `.jigc/displaced` keeps the task's id).
        let source: Vec<&str> = pattern.split('/').collect();
        let named: Vec<&str> = dir
            .split('/')
            .filter(|word| word.starts_with('<'))
            .collect();
        let below = source
            .iter()
            .rposition(|word| named.contains(word))
            .map(|at| at + 1)
            .unwrap_or_else(|| {
                source
                    .iter()
                    .take_while(|word| !word.starts_with('<') && **word != "*")
                    .count()
            });
        // One file's place parks nothing ([`Tally::parks`]).
        let Where::Under(_) = place else {
            return false;
        };
        let relative: &[&str] = from.get(below..).unwrap_or_default();
        if moved.is_empty() || moved.len() != relative.len() {
            return false;
        }
        let mut renamed = moved
            .iter()
            .zip(relative)
            .enumerate()
            .filter(|(_, (now, was))| now != was);
        match (renamed.next(), renamed.next()) {
            (None, _) => true,
            (Some((at, (now, was))), None) => {
                let numbered = now
                    .strip_prefix(*was)
                    .and_then(|rest| rest.strip_prefix('.'))
                    .is_some_and(|n| !n.starts_with('0') && n.parse::<u32>().is_ok_and(|n| n >= 2));
                let mut taken: Vec<&str> = to[..depth + at].to_vec();
                taken.push(was);
                numbered
                    && self
                        .seen
                        .after
                        .files
                        .contains_key(&format!("{REPO}/{}", taken.join("/")))
            }
            _ => false,
        }
    }

    /// **Whether `path`, from the sandbox's root, is where a line that moves aside parks
    /// from** ([`May::MovedAsideTo`]): any path of its place under a directory — a place the
    /// command removes. At one file's place, nothing: jigc moves a file aside only from a
    /// place it is about to remove, never from a destination (the human's ruling of
    /// 2026-10-20 on the rows' record's §6, entry 23).
    fn parks(&self, place: Where, path: &str) -> bool {
        matches!(place, Where::Under(_)) && self.placed(place, path)
    }

    /// **The doc a migration rewrote at `rel`, and whether it is `is` migrated**
    /// ([`May::Migrated`]): the doc that stood there before (`was`), or — at a current home
    /// where none stood — one at a home its doctype had at an earlier version, gone after,
    /// whose name lands there; below the doctype's current schema version before, at it
    /// after, its prose in `is`, in order. Where `committed`, the doc at the prior home is
    /// what `HEAD` held there: a commit is read against history.
    fn migrated_into(&self, rel: &str, was: Option<&Entry>, is: &Entry, committed: bool) -> bool {
        let (Some(homes), Some(after)) =
            (self.acting.homes.as_ref(), self.acting.homes_after.as_ref())
        else {
            return false;
        };
        let source: Option<(&str, Entry)> = match was {
            Some(was) => homes.doctype_at(rel).map(|ty| (ty, was.clone())),
            None => self.seen.before.files.iter().find_map(|(from, entry)| {
                let from = in_repo(from)?;
                let ty = homes.prior_doctype_at(from)?;
                let name = from.rsplit('/').next()?;
                if after.current.get(ty)?.for_name(name) != rel
                    || self
                        .seen
                        .after
                        .files
                        .contains_key(&format!("{REPO}/{from}"))
                {
                    return None;
                }
                if committed {
                    self.head_held(from).ok().flatten().map(|held| (ty, held))
                } else {
                    Some((ty, entry.clone()))
                }
            }),
        };
        let Some((ty, source)) = source else {
            return false;
        };
        let source = &source;
        let (Some(current), Some(shapes)) = (homes.versions.get(ty), homes.shapes.get(ty)) else {
            return false;
        };
        let Some(now) = shapes.get(current) else {
            return false;
        };
        // The shape the doc was written to: its stamp's version's — a doc with none, *"the
        // v0 corpus state"*, has the first version's shape, whose stamp alone it lacks —
        // and where no snapshot declares that version, the current one, which lets the
        // migration change nothing but the stamp.
        let was = stamp(source);
        let then = shapes.get(&was.max(1)).unwrap_or(now);
        was < *current && stamp(is) == *current && carried(source, is, *current, then, now)
    }

    /// **Where a doc at `rel`, a home its doctype had at an earlier version, is migrated
    /// to**: the doctype's current home after the command, under the doc's name.
    fn migrated_to(&self, rel: &str) -> Option<String> {
        let ty = self.acting.homes.as_ref()?.prior_doctype_at(rel)?;
        let name = rel.rsplit('/').next()?;
        let home = self.acting.homes_after.as_ref()?.current.get(ty)?;
        Some(home.for_name(name))
    }

    /// **Whether a doc gone from `rel`, a home its doctype had at an earlier version, was
    /// migrated into its current home** ([`May::Migrated`]).
    fn migrated_away(&self, rel: &str, was: &Entry) -> bool {
        let Some(to) = self.migrated_to(rel) else {
            return false;
        };
        let landed = self.seen.after.files.get(&format!("{REPO}/{to}"));
        let stood = self.seen.before.files.get(&format!("{REPO}/{to}"));
        stood.is_none()
            && landed.is_some_and(|is| self.migrated_into(&to, None, is, false))
            && was.lines().is_some()
    }

    /// **Whether `is` is the project manifest with its knob set** ([`May::ItsKeySet`]) over
    /// `was`: the lines that do not name the knob the unit is the same in both, in order —
    /// but one `scalar:` line, the map that holds the knob, where `was` held none;
    /// one line of `is` names the knob, and the manifest read as YAML holds, under
    /// `scalar:`, the knob set to the value the command names, a string — as the cascade
    /// reads a knob ([`knob`]). Since the review of the task `B8` (its H9): a line naming
    /// the knob held, whatever its value, and the `scalar:` line was free.
    fn its_knob_set(&self, was: Option<&Entry>, is: &Entry) -> bool {
        let (Some(knob), Some(value)) = (self.acting.unit.as_deref(), self.acting.value.as_deref())
        else {
            return false;
        };
        let names = |line: &&[u8]| {
            String::from_utf8_lossy(line)
                .trim_start()
                .starts_with(&format!("{knob}:"))
        };
        let the_map = |line: &&[u8]| line.trim_ascii_end() == b"scalar:";
        let (Some(lines), Entry::File { bytes, .. }) = (is.lines(), is) else {
            return false;
        };
        let held = match was {
            None => Vec::new(),
            Some(was) => match was.lines() {
                Some(lines) => lines,
                None => return false,
            },
        };
        // The map that holds the knob may be made, once, where none stood.
        let mut made = held.iter().any(the_map);
        let rest_was: Vec<&[u8]> = held.iter().filter(|line| !names(line)).copied().collect();
        let rest_is: Vec<&[u8]> = lines
            .iter()
            .filter(|line| {
                if names(line) {
                    return false;
                }
                if !made && the_map(line) {
                    made = true;
                    return false;
                }
                true
            })
            .copied()
            .collect();
        let set = serde_yaml_ng::from_slice::<serde_yaml_ng::Value>(bytes)
            .ok()
            .is_some_and(|yaml| {
                yaml.get("scalar")
                    .and_then(|map| map.get(knob))
                    .and_then(serde_yaml_ng::Value::as_str)
                    == Some(value)
            });
        lines.iter().filter(|line| names(line)).count() == 1 && rest_was == rest_is && set
    }

    /// **Whether the regular file at `path`, from the sandbox's root, is one the pinned
    /// base's tree holds** at its path under the checkout `place` names — the directory a
    /// [`Where::Under`] place stands for — with these bytes and this executable bit; or the
    /// checkout's own `.git` file at its root ([`May::FromTheBasesTree`]).
    fn of_the_bases_tree(&self, place: Where, path: &str, mode: u32, blob: &str) -> bool {
        let (Where::Under(pattern), Some(rel)) = (place, in_repo(path)) else {
            return false;
        };
        let Some(pattern) = self.acting.rooted(pattern) else {
            return false;
        };
        let depth = pattern.split('/').count();
        let inner: Vec<&str> = rel.split('/').skip(depth).collect();
        if inner.is_empty() {
            return false;
        }
        let inner = inner.join("/");
        if inner == ".git" {
            return true;
        }
        let Some(base) = self.acting.base.as_deref().filter(|base| !base.is_empty()) else {
            return false;
        };
        tree_of(self.seen.before, base)
            .get(&inner)
            .is_some_and(|(held, id)| {
                held & 0o170_000 == 0o100_000
                    && id == blob
                    && (held & 0o111 != 0) == (mode & 0o111 != 0)
            })
    }

    /// The line in force that lets the commit hold `entry` at `path`, from the
    /// repository's root, because a line of the file layer wrote it there.
    fn written_by_a_line(&self, path: &str, entry: Option<&(u32, String)>) -> Option<usize> {
        let key = format!("{REPO}/{path}");
        let after = self.seen.after.files.get(&key);
        // The regular file the commit holds at the path is the one the working tree holds
        // there after the command.
        let as_after = |(mode, blob): &(u32, String)| {
            mode & 0o170_000 == 0o100_000 && after.and_then(Entry::blob) == Some(blob.as_str())
        };
        self.in_force()
            .find(|(_, line)| {
                let Allows::Files(place, may) = line.allows else {
                    return false;
                };
                if self.by_its_commit_alone(line)
                    && !self.as_it_landed(
                        path,
                        entry
                            .filter(|(mode, _)| mode & 0o170_000 == 0o100_000)
                            .map(|(_, blob)| blob.as_str()),
                    )
                {
                    return false;
                }
                if !self.placed(place, &key) {
                    return false;
                }
                match (may, entry) {
                    (May::Removed, None) => true,
                    (May::Retired, None) => self.as_head_held(path, None),
                    (May::MovedAsideTo(_), None) => self.parks(place, &key),
                    (May::Removed | May::Retired | May::MovedAsideTo(_), Some(_)) => false,
                    (May::Retitled, None) => self
                        .retitle()
                        .is_some_and(|(old, new)| old == path && new != old),
                    (May::IntoItsHome, None) => self.moved_home(path).is_some(),
                    (May::Migrated, None) => self
                        .seen
                        .before
                        .files
                        .get(&key)
                        .is_some_and(|was| self.migrated_away(path, was)),
                    // The doc at its new home, or rewritten in place where that is its own.
                    (May::Retitled, Some(entry)) => {
                        as_after(entry) && self.retitle().is_some_and(|(_, new)| new == path)
                    }
                    // The arrival, at a mover's destination: the file the working tree holds
                    // there after the command — or the index entry the move carried with it,
                    // what the main checkout's index held at the path it left.
                    (May::IntoItsHome, Some(entry @ (mode, blob))) => {
                        matches!(place, Where::Path(A_NEW_HOME))
                            && after.is_some_and(|is| self.arrived_home(path, is))
                            && (as_after(entry)
                                || self.seen.before.checkouts.get(REPO).is_some_and(|main| {
                                    main.index.iter().any(|(from, staged)| {
                                        staged.mode == *mode
                                            && staged.blob == *blob
                                            && self.moved_home(from).as_deref() == Some(path)
                                    })
                                }))
                    }
                    // A word that says what a file may become: the commit holds the file the
                    // working tree holds after the command, and that file is what the word
                    // lets stand over what `HEAD` held there before it — so a commit is no
                    // way around the word, nor one that carries bytes of the user's no commit
                    // held where the word does not let them in.
                    (
                        May::MergeInto
                        | May::TakeOutOf
                        | May::Repointed
                        | May::Migrated
                        | May::ItsKeySet,
                        Some(entry),
                    ) => {
                        as_after(entry)
                            && after.is_some_and(|is| match self.head_held(path) {
                                Ok(was) => self.fits(may, path, was.as_ref(), is, true),
                                Err(()) => false,
                            })
                    }
                    // Laid out holds no file; read from holds nothing the command wrote.
                    (_, None) | (May::LaidOut | May::CopiedFrom | May::ReSlugged, _) => false,
                    // Carried: what stood there in the working tree before the command.
                    (May::Carried, Some((mode, blob))) => {
                        mode & 0o170_000 == 0o100_000
                            && self.seen.before.files.get(&key).and_then(Entry::blob)
                                == Some(blob.as_str())
                    }
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

    /// **The line in force that lets a commit the command made before its last hold
    /// `blob` at `path`** — from the repository's root — because the line merges into the
    /// file there and that commit is a step of the merge: every line the working file held
    /// before the command stands in the blob, in order, and every line of the blob stands
    /// in the working file after it. `add-from-spec` *"commits the record once per
    /// sub-task"*, so each commit but the last holds the record partway. The commit's
    /// bytes are read off [`Manifest::blobs`]; a blob the manifest holds no bytes of is
    /// none of it.
    fn merged_on_the_way(&self, path: &str, entry: Option<&(u32, String)>) -> Option<usize> {
        let (mode, blob) = entry?;
        if mode & 0o170_000 != 0o100_000 {
            return None;
        }
        let key = format!("{REPO}/{path}");
        let lines =
            |bytes: &'a [u8]| -> Vec<&'a [u8]> { bytes.split_inclusive(|b| *b == b'\n').collect() };
        let midway = lines(self.seen.after.blobs.get(blob)?);
        let after = self.seen.after.files.get(&key)?.lines()?;
        let before = match self.seen.before.files.get(&key) {
            Some(entry) => entry.lines()?,
            None => Vec::new(),
        };
        if !(stand_in(&before, &midway) && stand_in(&midway, &after)) {
            return None;
        }
        self.in_force()
            .find(|(_, line)| {
                matches!(line.allows, Allows::Files(place, May::MergeInto)
                    if self.placed(place, &key))
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
    /// sandbox, or in an object within reach. The store alone is no reach.
    fn stands(&self, blob: &str) -> bool {
        let after = self.seen.after;
        after.files.values().any(|entry| entry.blob() == Some(blob)) || after.held.contains(blob)
    }

    /// **Whether the staged doc at `path`, gone, was re-slugged** ([`May::ReSlugged`]):
    /// its unit — the directory its `docs/` stands in — still carries its base pin, and
    /// that `docs/` gained at least as many docs as it lost with the gone doc's lines but
    /// its title, so no two docs gone are answered by one gained.
    fn re_slugged(&self, path: &str) -> bool {
        let (before, after) = (self.seen.before, self.seen.after);
        let Some(Some(was)) = before.files.get(path).map(but_its_title) else {
            return false;
        };
        let Some((docs, _)) = path.rsplit_once('/') else {
            return false;
        };
        let Some(unit) = docs.strip_suffix("/docs") else {
            return false;
        };
        if !after.files.contains_key(&format!("{unit}/base.json")) {
            return false;
        }
        let alike = |manifest: &Manifest, other: &Manifest| {
            staged_docs(manifest)
                .into_iter()
                .filter(|(at, _)| {
                    at.rsplit_once('/').is_some_and(|(dir, _)| dir == docs)
                        && !other.files.contains_key(*at)
                })
                .filter(|(at, _)| {
                    manifest.files.get(*at).and_then(but_its_title).as_ref() == Some(&was)
                })
                .count()
        };
        alike(after, before) >= alike(before, after)
    }
}

/// **The positions of a doc's front-matter lines** — those between the `---` its first line
/// is and the next `---`, neither fence among them; none where the doc opens with no fence
/// or its front matter has no end.
fn front_matter(lines: &[&[u8]]) -> BTreeSet<usize> {
    let fence = |line: &[u8]| line.trim_ascii_end() == b"---";
    if !lines.first().is_some_and(|line| fence(line)) {
        return BTreeSet::new();
    }
    match lines.iter().skip(1).position(|line| fence(line)) {
        Some(end) => (1..=end).collect(),
        None => BTreeSet::new(),
    }
}

/// A doc's lines with its title — the first line that opens `# ` — left out: what a
/// re-slug keeps of a staged doc. `None` for what is no regular file.
fn but_its_title(entry: &Entry) -> Option<Vec<&[u8]>> {
    let mut lines = entry.lines()?;
    if let Some(title) = lines.iter().position(|line| line.starts_with(b"# ")) {
        lines.remove(title);
    }
    Some(lines)
}

/// **Whether `is` is `source` migrated from the shape `then` to the shape `now`, stamped
/// `version`, with every byte the migration's sentences do not let it change standing**
/// ([`May::Migrated`]): every line of `source` stands in `is`, in order and byte for byte —
/// its front matter's fields and their values, its `- key:` bullets, its headings of every
/// level and its blank lines among them — but two the sentences rewrite in place: the
/// stamp, *"value-bumps the stamp"*, which stands at `version`, and the `# ` title where
/// `now`'s `display-title:` is not `then`'s, which stands as `now` declares it. Each other
/// line of `is` is blank, or one a sentence adds: the `## ` heading of a section `now`
/// declares and `then` does not, once — *"splice an empty `## <Heading>` slot-section
/// block"*; the stamp where `source` had none, once; and the two `---` around it where it had
/// no front matter — *"splices a `---` front-matter block into a header-less doctype"*
/// (`design/corpus-migration.md` → *The deterministic transform*, *Relocation*).
///
/// *Not read* — each comes out red where a migration takes it: a value an authored map
/// remaps (`ValueRemapped`), a field or bullet an *added-optional-field* or *added item
/// field* writes, a slot promoted into an item block, a section's offset among its
/// siblings. No cell's doctype migrates by one (`adr` 1→2 adds `## Options`, `changelog`
/// 1→2 moves and re-titles).
fn carried(source: &Entry, is: &Entry, version: u64, then: &Shape, now: &Shape) -> bool {
    let (Some(source), Some(is)) = (source.lines(), is.lines()) else {
        return false;
    };
    let ended = |line: &[u8], text: &str| -> Vec<u8> {
        let mut out = text.as_bytes().to_vec();
        if line.ends_with(b"\n") {
            out.push(b'\n');
        }
        out
    };
    let fenced = source
        .first()
        .is_some_and(|line| line.trim_ascii_end() == b"---");
    let front_ends = if fenced {
        source
            .iter()
            .skip(1)
            .position(|line| line.trim_ascii_end() == b"---")
            .map(|at| at + 1)
    } else {
        None
    };
    let stamp_line = format!("schema-version: {version}");
    let retitled = now
        .title
        .as_ref()
        .filter(|title| then.title.as_ref() != Some(*title));
    let mut stamped = false;
    let mut titled = false;
    let mut expected: Vec<Vec<u8>> = Vec::new();
    for (at, line) in source.iter().enumerate() {
        let in_front = front_ends.is_some_and(|end| at > 0 && at < end);
        if in_front && !stamped && line.starts_with(b"schema-version:") {
            stamped = true;
            expected.push(ended(line, &stamp_line));
        } else if !in_front
            && front_ends.is_none_or(|end| at > end)
            && !titled
            && line.starts_with(b"# ")
        {
            titled = true;
            match retitled {
                Some(title) => expected.push(ended(line, title)),
                None => expected.push(line.to_vec()),
            }
        } else {
            expected.push(line.to_vec());
        }
    }
    // What a sentence adds, each as often as it may stand.
    let mut adds: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    for heading in now.headings.iter().filter(|h| !then.headings.contains(h)) {
        *adds
            .entry(heading.trim_end().as_bytes().to_vec())
            .or_default() += 1;
    }
    if !stamped {
        *adds.entry(stamp_line.into_bytes()).or_default() += 1;
    }
    if front_ends.is_none() {
        *adds.entry(b"---".to_vec()).or_default() += 2;
    }
    // Every line of `source` matched in order, the first `is` line that equals it taken —
    // which loses no match, since a line `is` adds may equally stand where it was matched.
    let mut wanted = expected.iter().peekable();
    for line in is {
        if wanted.peek().is_some_and(|want| want.as_slice() == line) {
            wanted.next();
            continue;
        }
        if line.trim_ascii().is_empty() {
            continue;
        }
        match adds.get_mut(line.trim_ascii_end()) {
            Some(left) if *left > 0 => *left -= 1,
            _ => return false,
        }
    }
    wanted.peek().is_none()
}

/// **A doc's schema-version stamp**, `schema-version: N` in its front matter — `0` where it
/// carries none (*"stamp **absent** (the v0 corpus state)"*, `design/corpus-migration.md`).
fn stamp(entry: &Entry) -> u64 {
    let Some(lines) = entry.lines() else {
        return 0;
    };
    if lines
        .first()
        .is_none_or(|line| line.trim_ascii_end() != b"---")
    {
        return 0;
    }
    lines
        .iter()
        .skip(1)
        .take_while(|line| line.trim_ascii_end() != b"---")
        .find_map(|line| {
            String::from_utf8_lossy(line)
                .trim_end()
                .strip_prefix("schema-version: ")
                .and_then(|value| value.trim().parse().ok())
        })
        .unwrap_or(0)
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
        let stands = match (plant.staged, stood) {
            (false, Some(stood)) => stood.files.contains(&plant.at),
            (true, Some(stood)) => stood.held && !stood.files.contains(&plant.at),
            (_, None) => false,
        };
        if !stands {
            return Err(format!(
                "the plant at `{}` does not stand there in the manifest taken before the \
                 command",
                plant.at
            ));
        }
    }
    let acted = seen.ended.code == Some(0);
    let verdict = |classification,
                   why: Vec<String>,
                   also: Vec<(Classification, Vec<String>)>,
                   tally: Option<&Tally<'_>>| Verdict {
        classification,
        why,
        also,
        excused: row
            .lines
            .iter()
            .enumerate()
            .map(|(at, line)| (line.id, tally.map_or(0, |tally| tally.excused[at])))
            .collect(),
    };
    if !seen.ended.ended {
        return Ok(verdict(
            Classification::DidNotEnd,
            vec!["the command had not ended at the frame's limit".to_string()],
            Vec::new(),
            None,
        ));
    }

    let (before, after) = (seen.before, seen.after);
    let acting = Acting::of(row, seen.argv, before, after);
    let mut tally = Tally {
        row,
        staged: staged_in_the_unit(before, &acting)
            .into_iter()
            .map(|(_, blob)| blob)
            .collect(),
        acting,
        acted,
        seen,
        excused: vec![0; row.lines.len()],
        lost: Vec::new(),
        git: Vec::new(),
        outside: Vec::new(),
    };

    // --- The plants: the user's, whatever the row declares. ---
    let nowhere = Stands::default();
    let mut judged: BTreeSet<&str> = BTreeSet::new();
    // Where a plant was carried to under a line — a place the directories on the way to it
    // are made for, as they are for a covered write.
    let mut carried: BTreeMap<&str, usize> = BTreeMap::new();
    for plant in seen.planted {
        let stands = after.markers.get(&plant.marker).unwrap_or(&nowhere);
        if stands.files.contains(&plant.at) {
            continue;
        }
        if plant.staged {
            // Staged, and in no file: it stands while an object within reach has it —
            // the index still, or a commit that took the entry in. What became of the
            // entry is the index's and the commit's to answer, below.
            if !stands.held {
                tally.lost.push(format!(
                    "what was staged at `{}` is in the index no longer, and its bytes stand \
                     at no path of the sandbox and in no object within reach",
                    plant.at
                ));
            }
            continue;
        }
        let named = in_repo(&plant.at).unwrap_or(&plant.at);
        if seen.consent && row.consent.is_some() && seen.ended.said.contains(named) {
            // The consent, and the path named: the difference is the lines' to judge.
            continue;
        }
        judged.insert(&plant.at);
        // A plant at a place a line moves aside.
        let aside = tally.in_force().find_map(|(at, line)| match line.allows {
            Allows::Files(place, May::MovedAsideTo(dir)) if tally.parks(place, &plant.at) => {
                let to = stands.files.iter().find(|to| {
                    in_repo(to).is_some_and(|rel| {
                        under_for(dir, rel, &tally.acting)
                            && tally.of_its_unit(place, named, dir, rel)
                    })
                })?;
                Some((at, to))
            }
            // A doc moved into its home carries what was planted in it.
            Allows::Files(place, May::IntoItsHome) if tally.placed(place, &plant.at) => {
                let to = format!("{REPO}/{}", tally.moved_home(in_repo(&plant.at)?)?);
                let to = stands.files.iter().find(|at| **at == to)?;
                Some((at, to))
            }
            // So does a doc migrated from a home its doctype had at an earlier version.
            Allows::Files(place, May::Migrated) if tally.placed(place, &plant.at) => {
                let to = format!("{REPO}/{}", tally.migrated_to(in_repo(&plant.at)?)?);
                let to = stands.files.iter().find(|at| **at == to)?;
                Some((at, to))
            }
            _ => None,
        });
        if let Some((at, to)) = aside {
            tally.excused[at] += 1;
            judged.insert(to);
            carried.insert(to, at);
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
    // Whether the change the line at `at` let the command make at `path` is one the row's
    // commit must carry: the row's commit holds all the lines changed, the line is of the
    // file layer, and the main checkout's index held the path.
    let must_land = |at: usize, path: &str| {
        holding == Some(Holding::AllTheLinesChanged)
            && matches!(row.lines[at].allows, Allows::Files(..))
            && in_repo(path)
                .is_some_and(|rel| main.0.is_some_and(|was| was.index.contains_key(rel)))
    };
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
        match tally.covering(diff) {
            // A change a line lets the command make at a tracked path, which the row's
            // commit must carry, and no commit the command made does.
            Some(at) if must_land(at, diff.path) && follows().is_none() => {
                tally.outside.push(format!(
                    "`{}` {} under the line `{}`, and no commit the command made carries it: \
                     its row lands that change in a commit",
                    diff.path,
                    match diff.after {
                        Some(_) => "is not what it was",
                        None => "is gone",
                    },
                    row.lines[at].id
                ));
            }
            Some(at) => {
                by.insert(diff.path, at);
            }
            None => match follows() {
                Some(at) => {
                    by.insert(diff.path, at);
                }
                None => open.push(diff),
            },
        }
    }
    // A directory on the way to a covered write is covered with it, and so is one that
    // emptied under a covered removal: the docs name files.
    open.retain(|diff| {
        let inside = format!("{}/", diff.path);
        let through = match (diff.before, diff.after) {
            (None, Some(Entry::Dir(_))) | (Some(Entry::Dir(_)), None) => by
                .iter()
                .chain(carried.iter())
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
    // Under a chain, the commits the command made are held to it whole — how many, in which
    // order, each holding what — and each path of one to the checkout whose commit it is.
    let chain = match (holding, commits) {
        (Some(Holding::TheWorktreesOneByOne(place)), Some(at)) if acted => {
            let carriers = carriers(before, place, &tally.acting);
            let id = row.lines[at].id;
            let moved = matches!(main, (Some(was), Some(is)) if was.head != is.head);
            if !made.is_empty() {
                tally.git.extend(chained(&made, &carriers, place, id));
            } else if !moved {
                tally.git.push(format!(
                    "the command made no commit on the checked-out branch, where its line \
                     `{id}` has {}: {}",
                    carriers.len() + 1,
                    the_chain(&carriers, place)
                ));
            }
            // Read link by link where the chain is as long as the line's: else a path of it
            // is any of the checkouts', and the length is what is wrong.
            let in_line = made.len() == carriers.len() + 1;
            Some((carriers, in_line))
        }
        _ => None,
    };
    let mut committed: BTreeSet<&str> = BTreeSet::new();
    for (newest, commit) in made.iter().enumerate() {
        // Its place in the chain, oldest first.
        let link = made.len() - 1 - newest;
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
                Some(Holding::TheWorktrees(place)) => {
                    before.checkouts.iter().any(|(at, checkout)| {
                        at != REPO
                            && in_repo(at).is_some_and(|rel| is_at_for(place, rel, &tally.acting))
                            && entry.is_some()
                            && staged(checkout)
                    })
                }
                Some(Holding::TheWorktreesOneByOne(_)) => {
                    let staged_by = |(_, staged): &(String, Staging)| {
                        staged.get(path.as_str()).map(Option::as_ref) == Some(entry)
                    };
                    match &chain {
                        Some((carriers, true)) => carriers.get(link).is_some_and(staged_by),
                        Some((carriers, false)) => carriers.iter().any(staged_by),
                        None => false,
                    }
                }
                _ => false,
            };
            // A commit a newer one the command made changes this path in again is a step
            // on the way to what the newest holds there.
            let on_the_way = made[..newest]
                .iter()
                .any(|newer| newer.changed.contains_key(path));
            if let Some(at) = tally.written_by_a_line(path, entry).or_else(|| {
                on_the_way
                    .then(|| tally.merged_on_the_way(path, entry))
                    .flatten()
            }) {
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
    // Under a commit that holds all the lines changed, no bytes of the user's ride it: a
    // plant no object within reach held before, which one holds now, was swept into git by
    // the command.
    if holding == Some(Holding::AllTheLinesChanged) {
        for plant in seen.planted.iter().filter(|plant| !plant.staged) {
            let held = |manifest: &Manifest| {
                manifest
                    .markers
                    .get(&plant.marker)
                    .is_some_and(|stands| stands.held)
            };
            if !held(before) && held(after) {
                tally.git.push(format!(
                    "a commit or the index the command left carries the bytes planted at `{}`, \
                     which no object within reach held before",
                    plant.at
                ));
            }
        }
    }
    // Prose handed to jigc: a staged doc gone from its working area stands in a commit the
    // command made, or it is somewhere else, or it is lost — **whatever the unit's base pin
    // does**. One move is the lines' to judge: a re-slug, under a line that lets it
    // ([`May::ReSlugged`]) — the doc under a new name in the same unit, which stands.
    if acted {
        for (path, blob) in staged_docs(before) {
            if after.files.contains_key(path) || committed.contains(blob) {
                continue;
            }
            let re_slugs = tally.in_force().any(|(_, line)| {
                matches!(line.allows, Allows::Files(place, May::ReSlugged)
                    if tally.placed(place, path))
            });
            if re_slugs && tally.re_slugged(path) {
                continue;
            }
            // Under the consent, a staged doc the command's output names is one the
            // caller was told goes: by its identity, the file's name without `.md`.
            let identity = path
                .rsplit('/')
                .next()
                .and_then(|name| name.strip_suffix(".md"))
                .unwrap_or(path);
            if seen.consent && row.consent.is_some() && seen.ended.said.contains(identity) {
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
    // A registration at its line's place, its checkout as `git worktree add --detach` leaves
    // one at the pinned base; or, where only the place is right, why the checkout is not.
    let off_the_base = |tally: &Tally<'_>, path: &str| -> Option<String> {
        let checkout = after.checkouts.get(path)?;
        let base = tally.acting.base.as_deref().unwrap_or_default();
        let mut why = Vec::new();
        if base.is_empty() || checkout.head != base {
            why.push("its `HEAD` is not the pinned base");
        }
        if checkout.branch.is_some() {
            why.push("it is on a branch");
        }
        if !staged_in(after, checkout).is_empty() {
            why.push("its index is not its `HEAD`'s tree");
        }
        (!why.is_empty()).then(|| why.join(", "))
    };
    let at_the_place = |tally: &Tally<'_>, path: &str| {
        tally.line_for(|allows| {
            matches!(allows, Allows::Registered(place)
                if in_repo(path).is_some_and(|rel| under_for(place, rel, &tally.acting)))
        })
    };
    let registered = |tally: &Tally<'_>, path: &str| {
        at_the_place(tally, path)
            .filter(|_| after.checkouts.contains_key(path) && off_the_base(tally, path).is_none())
    };
    let not_from_the_base = |tally: &Tally<'_>, path: &str| -> String {
        at_the_place(tally, path)
            .and_then(|_| off_the_base(tally, path))
            .map(|why| format!(" — not as the add leaves one at the pinned base: {why}"))
            .unwrap_or_default()
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
            (None, Some(path)) => match registered(&tally, path) {
                Some(at) => tally.excused[at] += 1,
                None => {
                    let why = not_from_the_base(&tally, path);
                    tally.git.push(format!(
                        "the registration `{name}`, of a worktree at `{path}`, was made{why}"
                    ))
                }
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
            (None, Some(_)) if place != REPO => match registered(&tally, place) {
                Some(at) => tally.excused[at] += 1,
                None => {
                    let why = not_from_the_base(&tally, place);
                    tally
                        .git
                        .push(format!("a checkout at `{place}` was made{why}"))
                }
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

    // Every class the cell is red in, in the order they are read: the first is its
    // classification, the others stand beside it.
    let mut classes: Vec<(Classification, Vec<String>)> = Vec::new();
    if acted {
        classes.push((Classification::Lost, tally.lost.clone()));
        classes.push((Classification::GitMoved, tally.git.clone()));
        classes.push((Classification::WrittenOutside, tally.outside.clone()));
        if seen.listed.refuses() {
            classes.push((
                Classification::NotTheListedOutcome,
                vec!["the cell is listed as refusing, and the command exited 0".to_string()],
            ));
        }
    } else {
        classes.push((
            Classification::RefusedAndChanged,
            [&tally.lost, &tally.git, &tally.outside]
                .into_iter()
                .flatten()
                .cloned()
                .collect(),
        ));
        let mut other: Vec<String> = Vec::new();
        match seen.listed {
            Listed::Acts => {
                other.push("the cell is listed as acting, and the command refused".to_string())
            }
            Listed::RefusesWith(refusal) => {
                if seen.ended.code != Some(refusal.exit) || !seen.ended.said.contains(refusal.says)
                {
                    other.push(format!(
                        "the cell is listed as refusing with exit {} and `{}`, and the command \
                         ended with {} and {}",
                        refusal.exit,
                        refusal.says,
                        seen.ended
                            .code
                            .map_or("no exit code".to_string(), |code| format!("exit {code}")),
                        if seen.ended.said.contains(refusal.says) {
                            "said so"
                        } else {
                            "did not say so"
                        },
                    ));
                }
                let not_moved = refusal.not_moved(before, after);
                if !not_moved.is_empty() {
                    other.push(format!(
                        "the cell is listed as refusing having moved what it could, and the \
                         command did not move {}: an acting sweep does all it can",
                        not_moved.join(", ")
                    ));
                }
            }
            Listed::Refuses => {}
        }
        classes.push((Classification::NotTheListedOutcome, other));
    }
    let mut red = classes.into_iter().filter(|(_, why)| !why.is_empty());
    let (classification, why) = red.next().unwrap_or((
        if acted {
            Classification::Holds
        } else {
            Classification::RefusesNothingChanged
        },
        Vec::new(),
    ));
    Ok(verdict(classification, why, red.collect(), Some(&tally)))
}

/// What one checkout staged, by path: the mode and blob its index holds there, or nothing
/// where it staged the path's removal.
type Staging = BTreeMap<String, Option<(u32, String)>>;

/// **The tree a commit holds**, by path: its first parents' changes laid down from the root
/// — what a manifest knows of a commit is what it changed against its first parent. Empty
/// for an unborn `HEAD`.
fn tree_of(manifest: &Manifest, head: &str) -> BTreeMap<String, (u32, String)> {
    let mut line: Vec<&Commit> = Vec::new();
    let mut at = Some(head).filter(|head| !head.is_empty());
    while let Some(id) = at
        && let Some(commit) = manifest.commits.get(id)
        && line.len() <= manifest.commits.len()
    {
        line.push(commit);
        at = commit.parent.as_deref();
    }
    let mut tree = BTreeMap::new();
    for commit in line.iter().rev() {
        for (path, entry) in &commit.changed {
            match entry {
                Some(entry) => tree.insert(path.clone(), entry.clone()),
                None => tree.remove(path),
            };
        }
    }
    tree
}

/// **What a checkout staged**: its index against the tree its own `HEAD` holds — an entry
/// that differs, and a path the tree holds and the index does not.
fn staged_in(manifest: &Manifest, checkout: &Checkout) -> Staging {
    let tree = tree_of(manifest, &checkout.head);
    let mut staged: Staging = checkout
        .index
        .iter()
        .filter(|(path, entry)| tree.get(path.as_str()) != Some(&(entry.mode, entry.blob.clone())))
        .map(|(path, entry)| (path.clone(), Some((entry.mode, entry.blob.clone()))))
        .collect();
    staged.extend(
        tree.keys()
            .filter(|path| !checkout.index.contains_key(*path))
            .map(|path| (path.clone(), None)),
    );
    staged
}

/// **The checkouts at `place` that staged code**, each with the id it stands at and what it
/// staged, in the order of those ids — the component of the place that is
/// [`ITS_SUB_TASKS`], or the checkout's path where the place has none. A checkout that
/// staged nothing carries nothing.
fn carriers(before: &Manifest, place: &str, acting: &Acting) -> Vec<(String, Staging)> {
    let position = place.split('/').position(|part| part == ITS_SUB_TASKS);
    let mut out: Vec<(String, Staging)> = before
        .checkouts
        .iter()
        .filter(|(at, _)| at.as_str() != REPO)
        .filter_map(|(at, checkout)| {
            let rel = in_repo(at)?;
            if !is_at_for(place, rel, acting) {
                return None;
            }
            let id = position
                .and_then(|position| rel.split('/').nth(position))
                .unwrap_or(rel);
            let staged = staged_in(before, checkout);
            (!staged.is_empty()).then(|| (id.to_string(), staged))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// The chain a line of [`Holding::TheWorktreesOneByOne`] has, in words.
fn the_chain(carriers: &[(String, Staging)], place: &str) -> String {
    let ids: Vec<String> = carriers.iter().map(|(id, _)| format!("`{id}`")).collect();
    format!(
        "one for each checkout at `{place}` that staged code, in the order of its sub-task's \
         id — {} — then the aggregate",
        if ids.is_empty() {
            "none".to_string()
        } else {
            ids.join(", ")
        }
    )
}

/// **Every way the commits the command made — newest first, as `made` holds them — are not
/// the chain its line `id` has**: as many as the checkouts that staged code, and one more;
/// and, link by link, each holding what its checkout staged and nothing else. A path a
/// checkout staged that its commit does not change is in line where an earlier link landed
/// the same entry there.
fn chained(made: &[&Commit], carriers: &[(String, Staging)], place: &str, id: &str) -> Vec<String> {
    if made.len() != carriers.len() + 1 {
        return vec![format!(
            "the command made {} commit{} on the checked-out branch, where its line `{id}` has \
             {}: {}",
            made.len(),
            if made.len() == 1 { "" } else { "s" },
            carriers.len() + 1,
            the_chain(carriers, place)
        )];
    }
    let mut out = Vec::new();
    for (link, ((sub_task, staged), commit)) in carriers.iter().zip(made.iter().rev()).enumerate() {
        let besides: Vec<&String> = commit
            .changed
            .iter()
            .filter(|(path, entry)| staged.get(path.as_str()) != Some(entry))
            .map(|(path, _)| path)
            .collect();
        let lacks: Vec<&String> = staged
            .iter()
            .filter(|(path, entry)| {
                commit.changed.get(path.as_str()) != Some(entry)
                    && !carriers[..link]
                        .iter()
                        .any(|(_, earlier)| earlier.get(path.as_str()) == Some(entry))
            })
            .map(|(path, _)| path)
            .collect();
        if !besides.is_empty() || !lacks.is_empty() {
            out.push(format!(
                "commit {} of the chain under the line `{id}` is the one of `{sub_task}`, whose \
                 checkout staged {:?}: it holds besides {besides:?}, and lacks {lacks:?}",
                link + 1,
                staged.keys().collect::<Vec<_>>()
            ));
        }
    }
    out
}

/// **The docs staged in every work unit**, each with its id: the regular `.md` files under
/// a task's `docs/` — `.jigc/tasks/<id>/docs/`, which is where the design docs say staged
/// prose stands (`design/project-setup.md` → *Flow 2 hardening*: *an open task under
/// `.jigc/tasks/` holding a staged `docs/*.md`*) — **and** the files of every merged set a
/// milestone's join has built (`.jigc/milestones/<id>/merged/docs/`), which is what a
/// boundary lands. Both, always: a joined milestone in the repository is no reason to
/// leave a task's own staged docs unread, and a sub-task's staged copy is held by itself
/// as well as through the merged set. What the loss loop holds to a commit; which of them
/// a line lets land is the acting unit's to say ([`Tally::staged`]).
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
    let mut all = docs(STAGED_IN_A_TASK);
    all.extend(docs(MERGED_IN_A_MILESTONE));
    all
}

/// Where a task's staged docs stand, a component for its id.
const STAGED_IN_A_TASK: &str = ".jigc/tasks/*/docs";

/// Where a milestone's join builds its merged set, a component for its id.
const MERGED_IN_A_MILESTONE: &str = ".jigc/milestones/*/merged/docs";

/// **The docs staged in the unit the command acts on** ([`staged_docs`], read through
/// [`Acting`]): its own task area's — `.jigc/tasks/<unit>/docs/` — its sub-tasks' areas'
/// — `.jigc/tasks/<sub-task>/docs/` — and its own merged set —
/// `.jigc/milestones/<unit>/merged/docs/`. Whose copy a line of [`May::CopyOfAStagedDoc`]
/// lets land: *"**Copy** (not move) the file from the working area to its canonical
/// path"* (`design/finalize.md` → *4. Promote managed docs*) — the unit's own working area,
/// so a sibling's staged doc, or another milestone's merged set, is none. Where the command
/// names no unit, nothing.
fn staged_in_the_unit<'a>(manifest: &'a Manifest, acting: &Acting) -> Vec<(&'a str, &'a str)> {
    let homes = [
        STAGED_IN_A_TASK.replacen('*', OWN, 1),
        STAGED_IN_A_TASK.replacen('*', ITS_SUB_TASKS, 1),
        MERGED_IN_A_MILESTONE.replacen('*', OWN, 1),
    ];
    staged_docs(manifest)
        .into_iter()
        .filter(|(path, _)| {
            in_repo(path).is_some_and(|rel| homes.iter().any(|home| under_for(home, rel, acting)))
        })
        .collect()
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
    /// **E**, the file deleted — **a file simply deleted by hand**: the regular file at this
    /// path of the repository is removed from the working tree, outside jigc, and nothing is
    /// put in its place. The human's ruling of 2026-10-18 on the review of the task `B6`
    /// (`DECISIONS.md`). **The index is left as it stands** — the plain `rm` a user does:
    /// at a path git tracks, git then lists the path *deleted* among the changes not staged
    /// for commit, beside an edit, and that is what **E** is — *an edit in no commit, in a
    /// tracked file*. A deletion staged with it (`git rm`) is a staged change, which is
    /// **S**, and is no plant of this kind. **It carries no bytes**, so the comparison has no
    /// marker of it to look for and nothing of the user's can be lost by it: what the
    /// command does at the path is held to its row as any write is — a file made there
    /// where no line covers it is *written outside its allowance* — and what a cell of it
    /// is for is its listed outcome. A place that holds no regular file is a fault of the
    /// cell, and so is a deletion in the place of a file ([`Plant::InPlaceOf`]): it makes
    /// no entry.
    Deleted(&'static str),
    /// **W** — the user's own linked worktree, at `outside/own`: detached, holding a
    /// commit only its `HEAD` reaches and a staged file, each with a marker of its own —
    /// and, where `moved_away`, its directory moved to the sandbox's `moved/` afterwards,
    /// so that git's registration names a directory that is not there.
    Worktree { moved_away: bool },
    /// **L**, to a file — a symbolic link at this path of the repository whose target is
    /// a file of the user's that is there: in the repository, at its root, or under
    /// `outside/`. The link's target carries one marker, by its name, and the file it
    /// points at another, in its bytes: a command that writes through the link leaves the
    /// first standing and takes the second.
    LinkToAFile { at: &'static str, outside: bool },
    /// **K**, a directory in a file's place — a directory at this path of the repository,
    /// holding one file of the user's.
    Dir(&'static str),
    /// **S**, and **N** — a change of the user's staged at this path of the repository: a
    /// file with the marker, or the marker added as a line to the file that stands there,
    /// then `git add`. Where not `kept`, the working file is put back as it was — taken
    /// away, where nothing stood — so that the staged bytes stand in the index alone.
    Staged { at: &'static str, kept: bool },
    /// **H** — the user's own lines in the `pre-commit` hook git resolves, which reject
    /// every commit: put in after the hook's first line where a hook stands — whatever
    /// else it holds stays — and as a hook of their own where none does.
    RejectingHook,
    /// **M** — a line of the user's own in the file at this path of the repository, which
    /// jigc merges into or takes lines out of: added as a line of its own to what stands
    /// there, or the file made with it where nothing does.
    UsersLine(&'static str),
    /// **M**, in the hook — the user's own line in the `pre-commit` hook git resolves,
    /// which lets every commit through: put in as [`Plant::RejectingHook`]'s lines are.
    UsersHook,
    /// **A plant in the place of a file that stands** — the human's ruling of 2026-10-18
    /// (`DECISIONS.md`, the entry of the task `B6`'s first commit, ruling 7): the regular
    /// file at the plant's path is taken away, and the plant put where it stood — a file of
    /// the user's, one the user committed, a link, a directory. The file taken away is the
    /// prepared state's or a lead-in's, never a plant, and is gone before the picture the
    /// command is held to. **Each is what its kind says, at a path git tracks too:** a file
    /// of the user's is **U**, *an untracked file*, so the index's entry at the path goes
    /// with the file taken away, as `git rm --cached` takes it — the index has the path
    /// deleted, and the tree a file git does not track, where a file written over the
    /// tracked one would be an edit of it, which is **E**; a file the user committed is
    /// committed over it; a link or a directory leaves the index's entry standing, as
    /// `ln -sf` or a `rm` and a `mkdir` leave it — **L** and **K** name what stands in
    /// the tree, not whether git tracks it. **A plant that is put into what stands** — an
    /// edit, a staged change, a line, a hook — **or a place that holds no regular file** is
    /// a fault of the cell; and a file in the place of a *directory* that stands is no
    /// plant of this kind: a cell moves the directory away first ([`Lead::MovedAway`]) and
    /// plants a [`Plant::File`] there.
    InPlaceOf(&'static Plant),
}

/// The name of the file a [`Plant::Dir`] holds.
const INSIDE_THE_DIRECTORY: &str = "a-file-of-its-users.txt";

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
    /// **The binary under test, run in the repository with these arguments, which must
    /// refuse** — any exit that is not 0. It is how a cell runs its own command a second
    /// time over what a first refusal left: the ruling on a refusal's caches is bounded
    /// by *a refused command run again refuses again, with the plant standing*. And how a
    /// lead runs a command that refuses and leaves what the cell needs: `task validate`
    /// over a task with no write refuses, and builds the edge index first.
    Refused(&'static [&'static str]),
    /// **A `doc` write to the part an uncommitted hand edit stands in** — the binary under
    /// test as `step` runs it ([`Step::Jigc`], or [`Step::JigcAt`] from a sub-task's
    /// worktree), writing the part the edit planted at the cell's lead `plant` changed.
    /// The human's ruling of 2026-10-19, which retired their bound 2, has such a write
    /// **refuse, naming the part and showing the hand-edited text**; this tree and the
    /// previous release let it act. **Either end lets the cell go on** — it acts, or it
    /// refuses with the plant's bytes in what it said — and the cell's command meets what
    /// the write left: so a cell whose lead-in is that write is held to its command's
    /// answer on a product before the ruling's fix and on one after it, and never stops on
    /// its lead. Any other end — a refusal that does not show the edit, a step that is not
    /// the binary's, no exit within the limit — is a fault of the cell, as a lead-in's is.
    OverAHandEdit {
        step: Step,
        plant: usize,
    },
}

/// **An open question a cell turns on, or a declared bound it stands on** — a mark. A
/// marked cell is built like any other, comes out under the reading by which more is red,
/// and says which question that reading is one of: **a ruling changes a mark and a list,
/// and never a cell's construction**. A mark makes no cell green, and takes none off the
/// expected-red list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mark {
    /// A place where a design doc is silent or says two things: an entry of the committed
    /// record's list, by its number (`…/reshape/byte-table-acceptance-rows.md` → §6).
    Silence(u8),
    /// One of the bounds the human declared on 2026-10-06, by its number in the plan's list
    /// (its §1.1) — the first, the third or the fourth since the human retired the second
    /// on 2026-10-19.
    Bound(u8),
}

impl Mark {
    /// The mark, as a table prints it.
    pub fn named(self) -> String {
        match self {
            Mark::Silence(entry) => format!("silence {entry}"),
            Mark::Bound(number) => format!("bound {number}"),
        }
    }
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
    /// What the cell's plant is aimed at; `None` for a cell that plants nothing.
    pub aim: Option<Aim>,
    /// The open questions the cell turns on and the bounds it stands on.
    pub marks: &'static [Mark],
    /// **The spelling of its leaf the cell runs** ([`Spelling`]) — `None` for the leaf as
    /// typed plainly. The cell builds the state the spelling stands for; the name says
    /// which lines it is compared under.
    pub spelling: Option<&'static str>,
    /// **Where the command runs** — `None` for the repository's root, else a place of the
    /// sandbox: a milestone sub-task is re-entered at its worktree, where its area stands
    /// under the main checkout (`design/storage.md` → *CLI and git*). Added 2026-10-20,
    /// with the mint family's cells (the task `B9`): `jigc workflow`'s re-entry provisions
    /// a sub-task's area only there.
    pub at: Option<route_case::Place>,
}

impl Cell {
    /// Whether the cell puts something into its state by hand, or makes a condition hold.
    pub fn plants(&self) -> bool {
        self.leads
            .iter()
            .any(|lead| matches!(lead, Lead::Plant(_) | Lead::Under(_)))
    }

    /// The layer the cell stands in, under its leaf's row.
    pub fn layer(&self, row: &Row) -> Layer {
        let resolves_nothing = MINIMAL_ARGV
            .iter()
            .find(|(leaf, _)| *leaf == self.leaf)
            .is_some_and(|(leaf, tail)| self.argv.iter().eq(leaf.iter().chain(tail.iter())));
        if self.plants() {
            Layer::Planted
        } else if resolves_nothing
            && self.leads.is_empty()
            && self.state == Prepared::Of(State::Fresh)
        {
            Layer::Refusing
        } else if self.state == row.home {
            Layer::Base
        } else {
            Layer::BaseElsewhere
        }
    }
}

impl Plant {
    /// Whether a plant of this shape can be one of `kind`.
    pub fn is_of(self, kind: Kind) -> bool {
        match self {
            Plant::File(_) => matches!(kind, Kind::Untracked | Kind::Foreign | Kind::FileForADir),
            Plant::Committed(_) => kind == Kind::Committed,
            Plant::LinkToNothing(_) => kind == Kind::LinkToNothing,
            Plant::Edit(_) => matches!(kind, Kind::EditBefore | Kind::EditAfter),
            Plant::Deleted(_) => matches!(kind, Kind::EditBefore | Kind::EditAfter),
            Plant::Worktree { moved_away } => {
                kind == if moved_away {
                    Kind::WorktreeMovedAway
                } else {
                    Kind::WorktreeThere
                }
            }
            Plant::LinkToAFile { outside, .. } => {
                kind == if outside {
                    Kind::LinkOutside
                } else {
                    Kind::LinkInside
                }
            }
            Plant::Dir(_) => kind == Kind::DirForAFile,
            Plant::Staged { kept, .. } => {
                kind == Kind::StagedThere
                    || (kept && matches!(kind, Kind::StagedElsewhere | Kind::QuotedName))
            }
            Plant::RejectingHook => kind == Kind::RejectingHook,
            Plant::UsersLine(_) | Plant::UsersHook => kind == Kind::UsersLine,
            // In a file's place: the inner plant's kind, but for the two that stand where
            // no file does — in a directory removed, or in a directory's place.
            Plant::InPlaceOf(inner) => {
                !matches!(kind, Kind::Foreign | Kind::FileForADir) && inner.is_of(kind)
            }
        }
    }

    /// Where a plant that makes an entry of its own stands, from the repository's root —
    /// `None` for one put into what stands there.
    fn place(self) -> Option<&'static str> {
        match self {
            Plant::File(path)
            | Plant::Committed(path)
            | Plant::LinkToNothing(path)
            | Plant::Dir(path)
            | Plant::LinkToAFile { at: path, .. } => Some(path),
            _ => None,
        }
    }
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
    if let Some(name) = cell.spelling
        && !row.spellings.iter().any(|spelling| spelling.name == name)
    {
        return Err(format!(
            "the cell names the spelling `{name}`, which the row of `jigc {}` does not have",
            row.leaf.join(" ")
        ));
    }
    let started = std::time::Instant::now();
    let copy = built.corpus.copy_state();
    let sandbox = Sandbox::over(cell.name, under_test, |repo, home| {
        fs::rename(copy.repo(), repo).expect("bring the prepared repository in");
        fs::rename(copy.home(), home).expect("bring the prepared home in");
    });
    // Every marker a plant of the cell could carry — a plant has up to two pieces, under
    // one of these words each: the reader looks for all of them.
    let markers = cell
        .leads
        .iter()
        .enumerate()
        .filter(|(_, lead)| matches!(lead, Lead::Plant(_)))
        .flat_map(|(at, _)| ["only", "target", "committed", "staged"].map(|word| marker(at, word)))
        .collect();
    let mut reader = Reader {
        sandbox: &sandbox,
        markers,
        commits: BTreeMap::new(),
        holding: BTreeMap::new(),
        blobs: BTreeMap::new(),
        lists: 0,
    };

    let base = reader.read()?;
    // The hooks directory git resolves, which a plant in the hook is put into.
    let hooks = base.hooks.clone();
    let mut planted = Vec::new();
    for (at, lead) in cell.leads.iter().enumerate() {
        if let Lead::Plant(plant) = lead {
            planted.extend(plant.planted(at, &hooks));
        }
        lead.done(&sandbox, &built.binary, at, &hooks)
            .map_err(|why| sandbox.shown(&why))?;
    }
    let before = if cell.leads.is_empty() {
        base.clone()
    } else {
        reader.read()?
    };
    let mut command = sandbox.child(&built.binary);
    if let Some(place) = cell.at {
        command.current_dir(sandbox.at(place));
    }
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
            argv: cell.argv,
            spelling: cell.spelling,
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
        verdict.shown()
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
    fn planted(self, at: usize, hooks: &str) -> Vec<Planted> {
        let piece = |path: String, word: &str| Planted {
            at: path,
            marker: marker(at, word),
            staged: false,
        };
        match self {
            Plant::InPlaceOf(inner) => inner.planted(at, hooks),
            // A deletion carries no bytes: there is no piece of it to find afterwards.
            Plant::Deleted(_) => Vec::new(),
            Plant::LinkToAFile { at: path, outside } => vec![
                piece(format!("{REPO}/{path}"), "only"),
                piece(
                    format!(
                        "{}/{}.md",
                        if outside { "outside" } else { REPO },
                        marker(at, "only")
                    ),
                    "target",
                ),
            ],
            Plant::Dir(path) => vec![piece(
                format!("{REPO}/{path}/{INSIDE_THE_DIRECTORY}"),
                "only",
            )],
            Plant::Staged { at: path, kept } => vec![Planted {
                staged: !kept,
                ..piece(format!("{REPO}/{path}"), "only")
            }],
            Plant::RejectingHook | Plant::UsersHook => {
                vec![piece(format!("{hooks}/pre-commit"), "only")]
            }
            Plant::File(path)
            | Plant::Committed(path)
            | Plant::LinkToNothing(path)
            | Plant::Edit(path)
            | Plant::UsersLine(path) => vec![piece(format!("{REPO}/{path}"), "only")],
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
    fn put(self, sandbox: &Sandbox, at: usize, hooks: &str) -> Result<(), String> {
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
            Plant::InPlaceOf(inner) => {
                let Some(path) = inner.place() else {
                    return Err(
                        "a plant in the place of a file is one that makes an entry of \
                                its own — a file, a committed file, a link or a directory"
                            .to_string(),
                    );
                };
                let file = repo.join(path);
                if !fs::symlink_metadata(&file).is_ok_and(|entry| entry.is_file()) {
                    return Err(format!(
                        "a plant goes in the place of the file at `{path}`, and no regular \
                         file stands there"
                    ));
                }
                // **U** is an untracked file: where a file of the user's goes, the entry
                // the index holds at the path goes too, as `git rm --cached` takes it —
                // else git would see an edit of the tracked file, which is **E**.
                if matches!(inner, Plant::File(_)) {
                    git(
                        sandbox,
                        &repo,
                        &["rm", "-q", "--cached", "--ignore-unmatch", "--", path],
                    )?;
                }
                fs::remove_file(&file).map_err(written(path))?;
                inner.put(sandbox, at, hooks)
            }
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
            // The plain `rm`: the file goes from the tree, and the index is not touched.
            Plant::Deleted(path) => {
                let file = repo.join(path);
                if !fs::symlink_metadata(&file).is_ok_and(|entry| entry.is_file()) {
                    return Err(format!(
                        "a deletion takes the file at `{path}`, and no regular file stands there"
                    ));
                }
                fs::remove_file(&file).map_err(written(path))
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
                    Lead::MovedAway(OWN_WORKTREE).done(sandbox, Path::new(""), at, hooks)?;
                }
                Ok(())
            }
            Plant::LinkToAFile { at: path, outside } => {
                let name = format!("{}.md", marker(at, "only"));
                let target = if outside {
                    sandbox.outside().join(&name)
                } else {
                    repo.join(&name)
                };
                if fs::symlink_metadata(&target).is_ok() {
                    return Err(format!("the link's target `{name}` stands already"));
                }
                fs::write(&target, format!("{}\n", marker(at, "target"))).map_err(written(path))?;
                std::os::unix::fs::symlink(&target, free(path)?).map_err(written(path))
            }
            Plant::Dir(path) => {
                let dir = free(path)?;
                fs::create_dir(&dir)
                    .and_then(|()| {
                        fs::write(
                            dir.join(INSIDE_THE_DIRECTORY),
                            format!("{}\n", marker(at, "only")),
                        )
                    })
                    .map_err(written(path))
            }
            Plant::Staged { at: path, kept } => {
                let file = repo.join(path);
                let was = match fs::symlink_metadata(&file) {
                    Ok(entry) if entry.is_file() => Some(fs::read(&file).map_err(written(path))?),
                    Ok(_) => {
                        return Err(format!(
                            "a staged change goes to `{path}`, and what stands there is no \
                             regular file"
                        ));
                    }
                    Err(_) => None,
                };
                let mut bytes = was.clone().unwrap_or_default();
                if was.is_some() {
                    bytes.push(b'\n');
                }
                bytes.extend(format!("{}\n", marker(at, "only")).bytes());
                fs::create_dir_all(file.parent().expect("a plant has a parent"))
                    .and_then(|()| fs::write(&file, bytes))
                    .map_err(written(path))?;
                git(sandbox, &repo, &["add", "--", path])?;
                match (kept, was) {
                    (true, _) => Ok(()),
                    (false, Some(was)) => fs::write(&file, was).map_err(written(path)),
                    (false, None) => fs::remove_file(&file).map_err(written(path)),
                }
            }
            Plant::UsersLine(path) => {
                let file = repo.join(path);
                let mut bytes = match fs::symlink_metadata(&file) {
                    Ok(entry) if entry.is_file() => fs::read(&file).map_err(written(path))?,
                    Ok(_) => {
                        return Err(format!(
                            "a line of the user's goes into `{path}`, and what stands there \
                             is no regular file"
                        ));
                    }
                    Err(_) => Vec::new(),
                };
                if !bytes.is_empty() && !bytes.ends_with(b"\n") {
                    bytes.push(b'\n');
                }
                bytes.extend(format!("{}\n", marker(at, "only")).bytes());
                fs::create_dir_all(file.parent().expect("a plant has a parent"))
                    .and_then(|()| fs::write(&file, bytes))
                    .map_err(written(path))
            }
            Plant::RejectingHook | Plant::UsersHook => {
                let hook = sandbox.root().join(hooks).join("pre-commit");
                let stands = fs::read_to_string(&hook).unwrap_or_default();
                let (first, rest) = match stands.split_once('\n') {
                    Some((first, rest)) if first.starts_with("#!") => (first, rest),
                    _ => ("#!/bin/sh", stands.as_str()),
                };
                let lines = match self {
                    Plant::RejectingHook => format!("echo '{}' >&2\nexit 1", marker(at, "only")),
                    _ => format!(": '{}'", marker(at, "only")),
                };
                let body = format!("{first}\n{lines}\n{rest}");
                // Placed by a child process, as the Linux rule asks of a file that may be
                // executed: no thread of this process ever holds its write handle.
                let staged = sandbox.staging().join(format!("plant-{at}-hook"));
                fs::write(&staged, body).map_err(written("the hook"))?;
                fs::create_dir_all(hook.parent().expect("a hook has a directory"))
                    .map_err(written("the hook"))?;
                let mut command = sandbox.child(Path::new("cp"));
                command.arg("--").arg(&staged).arg(&hook);
                let placed = sandbox.run(&mut command, route_case::LIMIT);
                if !placed.success() {
                    return Err(format!("the hook could not be placed: {}", placed.exit()));
                }
                fs::set_permissions(&hook, fs::Permissions::from_mode(0o755))
                    .map_err(written("the hook"))
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
    fn done(self, sandbox: &Sandbox, binary: &Path, at: usize, hooks: &str) -> Result<(), String> {
        match self {
            Lead::Refused(args) => {
                let mut command = sandbox.child(binary);
                command.args(args);
                let ran = sandbox.run(&mut command, route_case::LIMIT);
                if ran.held.is_none() && ran.code != Some(0) {
                    Ok(())
                } else {
                    Err(format!(
                        "the lead {at} of the cell must refuse, and `jigc {}` came to {}",
                        args.join(" "),
                        ran.exit()
                    ))
                }
            }
            Lead::OverAHandEdit { step, plant } => {
                let (place, args) = match step {
                    Step::Jigc(args) => (sandbox.repo(), args),
                    Step::JigcAt(place, args) => (sandbox.at(place), args),
                    _ => {
                        return Err(format!(
                            "the lead {at} of the cell is a write over a hand edit, and its \
                             step is not the binary under test"
                        ));
                    }
                };
                let mut command = sandbox.child(binary);
                command.current_dir(place).args(args);
                let ran = sandbox.run(&mut command, route_case::LIMIT);
                let refused = ran.held.is_none() && ran.code.is_some_and(|code| code != 0);
                if ran.success() || (refused && ran.said.contains(&marker(plant, "only"))) {
                    Ok(())
                } else {
                    Err(format!(
                        "the lead {at} of the cell, a write over the hand edit of its lead \
                         {plant}, acts or refuses showing that edit, and `jigc {}` came to {}: {}",
                        args.join(" "),
                        ran.exit(),
                        ran.said.trim_end()
                    ))
                }
            }
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
            Lead::Plant(plant) => plant.put(sandbox, at, hooks),
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
    /// The bytes of each blob a commit changed a regular file to, read once a cell.
    blobs: BTreeMap<String, Vec<u8>>,
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

            // What is held, as the human ruled it on 2026-10-14: what a ref, a registered
            // checkout's `HEAD` or its index, the newest stash entry, or any reflog
            // reaches.
            let mut reach = vec!["rev-list", "--objects", "--all", "--reflog"];
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

            // The bytes of every regular file a commit within reach changed a path to.
            let mut unread: Vec<String> = manifest
                .commits
                .values()
                .flat_map(|commit| commit.changed.values().flatten())
                .filter(|(mode, _)| mode & 0o170_000 == 0o100_000)
                .map(|(_, blob)| blob.clone())
                .filter(|blob| !self.blobs.contains_key(blob))
                .collect();
            unread.sort();
            unread.dedup();
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
                        Some(body.to_vec())
                    })();
                    let Some(body) = read else {
                        return Err(format!("git handed the reader no blob `{id}`"));
                    };
                    self.blobs.insert(id.clone(), body);
                }
            }
            for commit in manifest.commits.values() {
                for (_, blob) in commit.changed.values().flatten() {
                    if let Some(bytes) = self.blobs.get(blob) {
                        manifest.blobs.insert(blob.clone(), bytes.clone());
                    }
                }
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
