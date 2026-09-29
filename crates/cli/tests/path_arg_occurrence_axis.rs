//! M51 Increment 1 / T6 — **every occurrence of every path-bearing argument, over the
//! whole escape-shape axis.**
//!
//! ## The class this closes
//!
//! A caller-supplied token that becomes a path component reaches the filesystem — at the
//! door, or one commit closure later at the destructive sink. Until this increment `jigc
//! migrate` took its `<path>` as an opaque token: driven at `5688e2c`, an **absolute path
//! outside the repository** was joined onto the repo root, read, and recorded as the value
//! `jigc task finalize --approve` **deletes**, at exit 0 (`DECISIONS.md` → the 2026-09-12
//! M51 Increment 1 entries; `completions/artifacts/M51/baseline-tokens.md`).
//!
//! T1–T5 shipped the rules. This suite is the claim that the rules cover the **class**: not
//! the door an audit happened to walk, but every occurrence of every argument whose value
//! becomes a path component, each driven over every escape shape.
//!
//! ## The axis is the shipped registry, not a list
//!
//! The subject is [`PATH_ARG_OCCURRENCES`], **derived** from the clap tree — `ARG_TOKENS`
//! classifies every argument of every leaf verb, and an argument answered
//! `PlainValue::PathBearing` must carry a row at **every leaf it occurs at**
//! (`cli_parse::every_path_arg_occurrence_is_registered`, proven red by an applied mutant:
//! a row removed, and a leaf's argument renamed). So a fifteenth occurrence cannot ship
//! without joining the registry, and joining it cannot ship without being driven here.
//!
//! **Keyed by `(leaf, argument id, conditional arm)`, never by deduplicated id**
//! (`completions/artifacts/M51/settle-record.md` → §2): `path` is `jigc migrate`'s source
//! to be read and retired *and* `jigc unmanage`'s lookup key; `file` occurs twice; `value`
//! is path-like only when `key ∈ ROOT_KNOBS`; and `from_file` carries the `-` stdin
//! sentinel, which is not a path at all. Each of those is its own row with its own answer.
//!
//! ## The ten cells
//!
//! `{absolute, ../ escape, symlink escape, .git/ component, workbench root, untracked
//! in-repo, leading-colon pathspec magic, a wildmatch glob, a shell-unsafe name, the -
//! stdin sentinel}` — the escape shapes the wave was chartered on, plus the sentinel that is
//! **not** an escape and is driven for exactly that reason (it is the arm split the
//! registry's key exists to express), plus a tenth cell that is not an escape either.
//!
//! **The tenth cell asks the other half of every door's obligation: not what it decided, but
//! whether the sentence it decided it in can be run.** Every cell above is shell-safe by
//! accident of spelling, so no cell could witness what a door *prints*. Driven at
//! `c2faae6b`, `jigc migrate 'my notes.md' --as adr` refused correctly with
//! `migrate.source-untracked` and routed to `` stage it with `git add -- my notes.md` `` —
//! which exits **128** (pathspec `my`), before a re-run that exits 2. So the cell plants a
//! name a shell re-lexes into two words, and the route-runnability assertion beside it
//! ([`assert_command_spans_run`]) is applied after **every** cell, because the glob and
//! colon cells carry shell metachars of their own.
//!
//! **The ninth cell is the eighth's own hole, and it is why the axis has a *magic* dimension
//! rather than a `:` one.** Git has two pathspec magics: the `:` prefix, and wildmatch (`*`,
//! `?`, `[`, `\`), which needs no prefix at all. The colon cell's token was
//! `:(top)README.md`, which names **no readable file**, so every door that reads before it
//! adjudicates answered `NotFound`, and the expectation table recorded that as a disposition
//! rather than as a blind spot: the whole class was left to the sink on the stated ground
//! that the door refuses such a token anyway. `*.md` is the spelling that ground is false
//! for. Driven at `b9d9262`, on a corpus holding six tracked `.md` files, with the literal
//! file `*.md` planted untracked: `jigc migrate '*.md' --as changelog` read it, asked
//! `git ls-files -- '*.md'` whether git held a copy, was told **yes** about six other
//! people's files, and minted at exit 0 — after which `--approve` unlinked the source
//! (recoverable from no git object) and `git add -- '*.md'` swept an unrelated unstaged edit
//! into a commit naming the changelog. Each magic cell therefore plants a file that
//! **exists**; a cell naming nothing can only ever prove that nothing is named.
//!
//! **That rule then applied to the cell it was written about.** `*.md` is not the only
//! spelling the old ground is false for — `:colon.md` is a perfectly readable name, and
//! driven on the pre-fix binary a **staged** one made `jigc migrate` refuse with
//! `migrate.source-untracked`, saying the path was *"in neither this repository's index nor
//! its HEAD"* while `git ls-files --stage -- ./:colon.md` printed its blob, and route the
//! operator to `git add -- :colon.md`, which exits **128**; committed, the same source was
//! **admitted**, and the refusal arrived one whole authoring later at the retire sink. So the
//! colon cell now carries [`COLON_SOURCE`] — a planted, readable, colon-named file — and the
//! five occurrences that had answered it with a read miss answer it with their own
//! behaviour instead.
//!
//! **A row with no cell is a hard panic, never a skip.** The cell expectation is an
//! exhaustive `match` over [`Cell`] per row — the compiler is the fence on that dimension
//! — the outer match panics for a row this suite does not answer, and the **driven** key
//! set is compared back to the set the registry declares, so an occurrence cannot join the
//! registry and quietly run nothing.
//!
//! **Every disposition is asserted, including the no-rule ones.** A `NoRule` row claims the
//! token becomes no path component; that claim is driven as the ACCEPTING behaviour it
//! predicts — the token read as content, resolved against a closed vocabulary, or answered
//! as an idempotent no-op — never as a silence.
//!
//! ## The composite cost, and the one cell that was a defect
//!
//! After **every** cell the canary planted outside the repository is re-read byte-for-byte,
//! and after every refusing cell `HEAD` and `git status --porcelain` are re-asserted
//! unmoved: a correct sentence over a tree that has already changed is what this wave
//! exists to catch.
//!
//! One cell was red when this suite was first written, and it is the reason the suite is
//! not a pin over shipped behaviour: `jigc config set docs-root ':!docs'` landed at **exit
//! 0**, and a task that then authored an `adr` finalized at exit 0 reporting `1 file
//! committed` while the promoted doc — written to `:!docs/decisions/probe.md` on disk —
//! matched the *exclude* pathspec and reached no commit. T6 closed it by joining
//! `config::unusable_root_reason`'s existing lexical legs (`config.unusable-root`, no new
//! code), which is the same narrowing the sink already takes on a recorded retirement path.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Output;

use cli::cli::{
    ArmToken, PATH_ARG_OCCURRENCES, PATH_ARG_SLOT, PathArgArm, PathArgBase, PathArgDisposition,
    PathArgOccurrence, PathArgSubject,
};

use crate::support;
use support::trial_corpus::{FixturePack, State, TrialCorpus};

/// Every blocking code this wave's path rules refuse with — the set a `NoRule` row must
/// **not** produce, which is how a no-rule claim is asserted rather than assumed.
const PATH_RULE_CODES: &[&str] = &[
    "migrate.source-untrackable",
    "migrate.source-untracked",
    "config.step-source-untrackable",
    "config.untrackable-root",
    "config.workbench-root",
    "config.unusable-root",
    "finalize.retire-untrackable",
];

/// The bytes planted **outside the repository** — the canary re-read after every cell, and
/// simultaneously a valid `jigc doc author` payload, so the cells that reach it as *content*
/// answer about the content rather than about a parse failure.
const CANARY: &str = "\
title: Probe
sections:
  - id: context
    set: {context: <<Axis prose.>>}
";

/// A step file the `target`-carrying `replace-step` row names, so that row's door answers
/// about its **target** and not about a missing source.
const REPLACEMENT_STEP: &str = "id: replacement-step\nbody: Replacement.\n";

/// The repo-relative in-repo untracked source — the sixth cell's spelling, carrying
/// [`CANARY`]'s bytes so it reads as a payload wherever a payload is what the door wants.
const UNTRACKED_SOURCE: &str = "untracked-source.yaml";

/// The glob cell's spelling: a file whose **name is a pattern**, planted so the token names
/// a file that genuinely exists *and* matches tracked files git holds. It carries [`CANARY`]
/// too, so the doors that read a source answer about the payload exactly as the untracked
/// cell's does — the only difference between the two cells is the name.
const GLOB_SOURCE: &str = "*.md";

/// The colon cell's spelling: a file whose **name begins with git's magic prefix**, planted
/// so the token names a file that genuinely exists. It carries [`CANARY`] too, so the doors
/// that read a source answer about the payload exactly as the untracked cell's does — the
/// only difference between the two cells is the name.
///
/// **Why it must exist**, which is this cell's own history: the cell shipped as
/// `:(top)README.md`, a token naming no readable file, so every door that reads before it
/// adjudicates answered `NotFound` and five occurrences were answered by a read miss rather
/// than by their own behaviour — the same blind spot the module header names for the glob
/// cell, left standing in the cell it was named about. A colon-named file that is readable
/// is the spelling the blind spot hides: driven on the pre-fix binary, a **staged**
/// `:colon.md` made `jigc migrate` report `migrate.source-untracked` — *"in neither this
/// repository's index nor its HEAD"* — about a path `git ls-files --stage` printed, and emit
/// `git add -- :colon.md`, which exits **128**.
const COLON_SOURCE: &str = ":colon.md";

/// The **shell-unsafe** cell's spelling: an in-repo, untracked file whose name a shell does
/// not re-lex as itself — a space (which splits it into two words) and a single quote (which
/// breaks naive quoting). It carries [`CANARY`] too, so the doors that read a source answer
/// about the payload exactly as the untracked cell's does; the only difference between the
/// two cells is the name.
///
/// **Why the axis needed it** (M51 completion audit): every other cell's token is shell-safe
/// by accident of spelling, so no cell could ever witness what a door *prints*. Driven at
/// `c2faae6b`, `jigc migrate 'my notes.md' --as adr` refused with a route reading `` stage it
/// with `git add -- my notes.md` ``; followed verbatim that exits **128** (pathspec `my`) and
/// the re-run it names exits **2**. The finding code was right, the sentence was right, and
/// the bytes were unrunnable — which is exactly the state a nine-cell axis with no
/// shell-unsafe member cannot see.
const UNSAFE_SOURCE: &str = "it's an odd name.md";

/// The in-repo symlink the third cell reaches through — **relative**, so a copied corpus's
/// link points at the copy's own outside directory and never back at the source fixture.
const SYMLINK_DIR: &str = "linkdir";

/// One escape shape.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Cell {
    /// An absolute path outside the repository — EC-1's own Tier-0 cell.
    Absolute,
    /// A `../` hop above the repository root.
    ParentEscape,
    /// A path *through* a symlink that leaves the repository.
    Symlink,
    /// A path with a `.git` component — git's own directory.
    GitComponent,
    /// A path inside jigc's own `.jigc/` workbench.
    WorkbenchRoot,
    /// An in-repo path git holds no copy of (neither the index nor HEAD).
    UntrackedInRepo,
    /// A leading `:` — what git reads as pathspec magic (`:(top)`, `:!`), never as a name,
    /// carried by a file that **exists** ([`COLON_SOURCE`]), so the doors that read before
    /// they adjudicate answer about this token instead of about a missing one.
    PathspecMagic,
    /// Git's OTHER magic: a wildmatch pattern (`*`) that names a file which **exists** while
    /// matching files git holds. The cell above could not reach this, and that is why it is
    /// its own cell rather than a second spelling of one (see the module header).
    PathspecGlob,
    /// A name a shell does not re-lex as itself (a space and a `'`), carried by a file that
    /// **exists** ([`UNSAFE_SOURCE`]). Not a path escape at all: the door's *verdict* is the
    /// untracked-in-repo one. What this cell asks is the other half of every door's
    /// obligation — whether the command line it prints can be run.
    ShellUnsafeName,
    /// `-`, the declared stdin sentinel. Not an escape: the arm split itself.
    StdinSentinel,
}

impl Cell {
    const ALL: [Cell; 10] = [
        Cell::Absolute,
        Cell::ParentEscape,
        Cell::Symlink,
        Cell::GitComponent,
        Cell::WorkbenchRoot,
        Cell::UntrackedInRepo,
        Cell::PathspecMagic,
        Cell::PathspecGlob,
        Cell::ShellUnsafeName,
        Cell::StdinSentinel,
    ];
}

/// What a door is expected to do with one cell's token.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    /// A **path rule** refuses it, by this code — the `Adjudicated` rows' claim.
    PathRuleBlocks(&'static str),
    /// The door refuses, by a code that is **not** a path rule — the token was resolved
    /// against a closed vocabulary (a step id, a fill point, a knob's declared type) and
    /// answered there. A `NoRule` row's claim, in its strongest form.
    AnswersElsewhere(&'static str),
    /// The bytes were **read as content** and the door answered about the content. Also a
    /// `NoRule` claim: the token was a source to open, never a path to adjudicate.
    ReadAsContent,
    /// The token names no file, so the door's read misses. Carries no finding code — a
    /// declared, pre-existing route-floor gap this increment neither closes nor widens
    /// (`crate::doc::read_handoff`'s own statement of what its disposition does not claim).
    NotFound,
    /// The door does the thing it exists to do.
    Accepted,
}

/// The token one cell takes at one subject — the spelling that asks *that* occurrence its
/// own question, which is why [`PathArgSubject`] is part of the registry rather than of
/// this suite.
fn token(cell: Cell, subject: PathArgSubject, repo: &Path) -> String {
    let outside = repo
        .parent()
        .expect("the corpus root is the repo's parent")
        .join("outside");
    let file = |cell: Cell| -> String {
        match cell {
            Cell::Absolute => outside.join("payload.yaml").to_string_lossy().into_owned(),
            Cell::ParentEscape => "../outside/payload.yaml".to_owned(),
            Cell::Symlink => format!("{SYMLINK_DIR}/payload.yaml"),
            Cell::GitComponent => ".git/config".to_owned(),
            Cell::WorkbenchRoot => ".jigc/state/file-state.json".to_owned(),
            Cell::UntrackedInRepo => UNTRACKED_SOURCE.to_owned(),
            Cell::PathspecMagic => COLON_SOURCE.to_owned(),
            Cell::PathspecGlob => GLOB_SOURCE.to_owned(),
            Cell::ShellUnsafeName => UNSAFE_SOURCE.to_owned(),
            Cell::StdinSentinel => "-".to_owned(),
        }
    };
    match subject {
        PathArgSubject::SourceFile | PathArgSubject::FieldValue => file(cell),
        PathArgSubject::Home => match cell {
            Cell::Absolute => outside.to_string_lossy().into_owned(),
            Cell::ParentEscape => "../outside".to_owned(),
            Cell::Symlink => SYMLINK_DIR.to_owned(),
            Cell::GitComponent => ".git/docs".to_owned(),
            Cell::WorkbenchRoot => ".jigc/docs".to_owned(),
            Cell::UntrackedInRepo => "untracked-home".to_owned(),
            Cell::PathspecMagic => ":(top)docs".to_owned(),
            Cell::PathspecGlob => "do*s".to_owned(),
            Cell::ShellUnsafeName => "an odd docs home".to_owned(),
            Cell::StdinSentinel => "-".to_owned(),
        },
        // The head is held valid so the door answers about the TAIL — the component that
        // would name `.jigc/config/steps/<id>.yaml` or `.jigc/config/fills/<id>.md`.
        PathArgSubject::AddressTail(head) => format!("{head}{}", file(cell)),
    }
}

/// What the door owes for one `(occurrence, arm, cell)`.
///
/// The cell dimension is an exhaustive `match`, so the **compiler** refuses a row that
/// leaves a cell unanswered; the outer match panics for a row this suite does not answer at
/// all. Neither is a skip.
fn expectation(door: &str, arg: &str, arm: &PathArgArm, cell: Cell) -> Outcome {
    use Cell::*;
    use Outcome::*;
    match (door, arg, arm.token, arm.subject) {
        // `jigc migrate <path>` — the source read now, retired on `--approve` later.
        ("migrate", "path", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot => {
                PathRuleBlocks("migrate.source-untrackable")
            }
            UntrackedInRepo => PathRuleBlocks("migrate.source-untracked"),
            // The door refuses git's magic as a CLASS, at the same code as the other
            // location legs — and BOTH magic cells moved here, which is the shape of the
            // defect rather than a bonus. The row used to read *"pathspec magic is a SINK
            // property by decision: at the door the same token is already refused, for the
            // unrelated reason that it names no readable file"*. That ground is false for
            // every readable spelling of the class, colon and glob alike — `*.md` and
            // `:colon.md` both name files that **exist**, so the door read them, asked its
            // trackedness leg a `git ls-files` that answered about a set nobody named, and
            // (glob) minted at exit 0 a deletion target no git object holds / (colon) said
            // a staged source was in neither the index nor HEAD and routed to a `git add`
            // that exits 128. One spelling of a rule is not the rule, so the class is asked
            // in the home the door and the sink share — and both magic cells name files that
            // exist, so neither is answered by a read miss that names the wrong problem.
            PathspecMagic | PathspecGlob => PathRuleBlocks("migrate.source-untrackable"),
            // Not a location escape — the verdict is the untracked one. The cell's question
            // is the route, and it is asked by the composite assertion below.
            ShellUnsafeName => PathRuleBlocks("migrate.source-untracked"),
            StdinSentinel => NotFound,
        },
        // `jigc unmanage <path>` — a lookup key over the recorded store, never a path op.
        ("unmanage", "path", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName | StdinSentinel => Accepted,
        },
        // `jigc relocate --from <prior-home>` — a prefix over committed spellings, with the
        // one class the prefix reaches that is jigc's own: the roots the install writes into
        // (M52 Increment 8 / T7). `.jigc/docs` is the workbench cell, and the adapter's own
        // artifact roots are driven as their own class by
        // [`relocate_refuses_every_root_jigcs_own_install_writes_into`] — they are read from
        // the installed profile, which no cell of a fixed escape axis can spell.
        ("relocate", "from", _, _) => match cell {
            WorkbenchRoot => PathRuleBlocks("config.workbench-root"),
            Absolute | ParentEscape | Symlink | GitComponent | UntrackedInRepo | PathspecMagic
            | PathspecGlob | ShellUnsafeName | StdinSentinel => Accepted,
        },
        // The two `<file>` occurrences — a SOURCE rule: an out-of-repo source is admitted
        // and copied in; git's own directory and jigc's transient workbench are not.
        ("config insert-step" | "config replace-step", "file", _, _) => match cell {
            GitComponent | WorkbenchRoot => PathRuleBlocks("config.step-source-untrackable"),
            // Both magic cells join the accepting set rather than the refusing one, and that
            // is the class boundary stated as an outcome: these doors `std::fs::read` the
            // token and never hand it to git, so a name git would read as a pattern is just
            // a name. The colon cell says that here only because its file exists — while it
            // named nothing, this row's no-rule claim was asserted by a read miss.
            Absolute | ParentEscape | Symlink | UntrackedInRepo | PathspecMagic | PathspecGlob
            | ShellUnsafeName => Accepted,
            // `-` is read as a file name at these two doors, not as stdin — which is the
            // discriminator that earns them a rule while `--from-file` takes none.
            StdinSentinel => NotFound,
        },
        // `--from-file` at the three handoff doors: the sentinel arm, then the path arm.
        ("config fill" | "doc set-slot", "from_file", ArmToken::Literal(_), _) => match cell {
            StdinSentinel => Accepted,
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName => {
                unreachable!("the sentinel arm answers for `-` and no other token")
            }
        },
        ("config fill" | "doc set-slot", "from_file", ArmToken::Caller, _) => match cell {
            // Any bytes are prose / fill content — the token was a source to open, whatever
            // git would have read the name as.
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName => Accepted,
            StdinSentinel => unreachable!("the sentinel arm claims this cell"),
        },
        ("doc author", "from_file", ArmToken::Literal(_), _) => match cell {
            StdinSentinel => Accepted,
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName => {
                unreachable!("the sentinel arm answers for `-` and no other token")
            }
        },
        ("doc author", "from_file", ArmToken::Caller, _) => match cell {
            Absolute | ParentEscape | Symlink | UntrackedInRepo | PathspecMagic | PathspecGlob
            | ShellUnsafeName => Accepted,
            // Read, then answered about the PAYLOAD — the no-rule claim in its plainest
            // form: git's config and jigc's own state file are opened like any other
            // source and rejected for what they say, not for where they are.
            GitComponent | WorkbenchRoot => ReadAsContent,
            StdinSentinel => unreachable!("the sentinel arm claims this cell"),
        },
        // The `target` occurrences — resolved against a closed vocabulary before any write.
        ("config replace-step" | "config remove-step" | "config fork", "target", _, _) => {
            match cell {
                Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot
                | UntrackedInRepo | PathspecMagic | PathspecGlob | ShellUnsafeName
                | StdinSentinel => AnswersElsewhere("config.anchor-absent"),
            }
        }
        ("config fill", "target", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName | StdinSentinel => {
                AnswersElsewhere("config.fill-point-absent")
            }
        },
        // `jigc config set <key> <value>` — the root-knob arm is the one that resolves a
        // HOME, and it is the arm that refuses.
        ("config set", "value", _, PathArgSubject::Home) => match cell {
            ParentEscape | Symlink | GitComponent => PathRuleBlocks("config.untrackable-root"),
            WorkbenchRoot => PathRuleBlocks("config.workbench-root"),
            // The absolute cell, and — as of T6 — the pathspec-magic cell, which landed at
            // exit 0 until this task. The glob cell joins them for the same reason one leg
            // out: a root knob is the first component of every pathspec built from it, and
            // wildmatch needs no leading `:` to make one a pattern.
            Absolute | PathspecMagic | PathspecGlob => PathRuleBlocks("config.unusable-root"),
            // A new relative directory is the ordinary case; `-` reads back as itself and
            // is an ordinary (if odd) directory name, so refusing it would refuse a home —
            // and so does a name holding a space, which is legal on every filesystem jigc
            // runs on. What it owes is a home NAMED runnably wherever a surface prints it.
            UntrackedInRepo | ShellUnsafeName | StdinSentinel => Accepted,
        },
        ("config set", "value", _, PathArgSubject::FieldValue) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName | StdinSentinel => {
                AnswersElsewhere("config.value-rejected")
            }
        },
        // `jigc doc set-field --value` — the value is document content.
        ("doc set-field", "value", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | ShellUnsafeName | StdinSentinel => Accepted,
        },
        _ => panic!(
            "`jigc {door}`'s `{arg}` [{}] joined PATH_ARG_OCCURRENCES with no cell \
             expectations — every registered arm is driven over the whole escape-shape \
             axis, or this suite is claiming a completeness it does not have",
            arm.when,
        ),
    }
}

/// Which cells reach one arm — the partition the registry's `ArmToken` states.
///
/// A `Literal` arm claims exactly the cell whose token **is** that literal (and a literal
/// no cell carries is a hard panic, not a silently empty arm); a `Caller` arm takes
/// everything its siblings did not claim.
fn domain(occurrence: &PathArgOccurrence, arm: &PathArgArm) -> Vec<Cell> {
    let claimed: Vec<Cell> = occurrence
        .arms
        .iter()
        .filter_map(|sibling| match sibling.token {
            ArmToken::Literal(literal) => {
                let cell = Cell::ALL
                    .into_iter()
                    .find(|cell| literal_cell_token(*cell) == Some(literal))
                    .unwrap_or_else(|| {
                        panic!(
                            "`jigc {}`'s `{}` declares a literal arm for `{literal}`, which no \
                             cell of this axis carries — a literal arm no cell reaches is an \
                             arm nothing drives",
                            occurrence.door.join(" "),
                            occurrence.arg,
                        )
                    });
                Some(cell)
            }
            ArmToken::Caller => None,
        })
        .collect();
    match arm.token {
        ArmToken::Literal(literal) => Cell::ALL
            .into_iter()
            .filter(|cell| literal_cell_token(*cell) == Some(literal))
            .collect(),
        ArmToken::Caller => Cell::ALL
            .into_iter()
            .filter(|cell| !claimed.contains(cell))
            .collect(),
    }
}

/// The literal token a cell **is**, for the cells that are one exact string at every
/// subject — today only the stdin sentinel.
fn literal_cell_token(cell: Cell) -> Option<&'static str> {
    matches!(cell, Cell::StdinSentinel).then_some("-")
}

/// Plant one copy's fixture: the canary outside the repository, the relative symlink
/// through it, the untracked in-repo source, and the step file the `target` rows name.
fn plant(corpus: &TrialCorpus) {
    let repo = corpus.repo();
    let outside = repo
        .parent()
        .expect("the corpus root is the repo's parent")
        .join("outside");
    fs::create_dir_all(&outside).expect("create the outside dir");
    fs::write(outside.join("payload.yaml"), CANARY).expect("plant the canary");
    let link = repo.join(SYMLINK_DIR);
    if !link.exists() {
        std::os::unix::fs::symlink("../outside", &link).expect("plant the symlink");
    }
    fs::write(repo.join(UNTRACKED_SOURCE), CANARY).expect("plant the untracked source");
    fs::write(repo.join(GLOB_SOURCE), CANARY).expect("plant the glob-named source");
    fs::write(repo.join(COLON_SOURCE), CANARY).expect("plant the colon-named source");
    fs::write(repo.join(UNSAFE_SOURCE), CANARY).expect("plant the shell-unsafe-named source");
    fs::write(repo.join("replacement-step.yaml"), REPLACEMENT_STEP).expect("plant the step");
}

/// The canary's bytes, read back — the composite cost, asserted after every single cell.
fn canary(corpus: &TrialCorpus) -> String {
    let path = corpus
        .repo()
        .parent()
        .expect("the corpus root is the repo's parent")
        .join("outside")
        .join("payload.yaml");
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the canary planted outside the repository must survive every cell: {} — {e}",
            path.display()
        )
    })
}

/// `HEAD` plus the porcelain status — the tree a refusing cell must leave untouched.
fn tree(corpus: &TrialCorpus) -> (String, String) {
    (
        corpus.git(&["rev-parse", "HEAD"]),
        corpus.git(&["status", "--porcelain"]),
    )
}

/// Drive one arm's argv with one cell's token, always handing the child a payload on
/// stdin: the sentinel arms read it, and a door that does not read it is unaffected — while
/// a child left attached to the harness's stdin would hang.
fn drive(corpus: &TrialCorpus, arm: &PathArgArm, token: &str) -> Output {
    let argv: Vec<String> = arm
        .argv
        .iter()
        .map(|part| {
            if *part == PATH_ARG_SLOT {
                token.to_owned()
            } else {
                (*part).to_owned()
            }
        })
        .collect();
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    corpus.jigc_stdin(&argv, CANARY)
}

/// **The axis** — every registered occurrence-arm, over every cell of the escape space.
///
/// Three bases, because three preconditions are genuinely different and each is built once
/// and **copied** per cell (the mutating-arm rule, `pinning.md` §4): a committed corpus for
/// the store-level doors, that corpus plus one live task and a staged `adr` for the
/// in-task write doors, and a corpus over a **manifest-less fixture pack** for `jigc
/// relocate`, whose door refuses a frozen doctype before it ever reads `--from`.
#[test]
fn every_path_arg_occurrence_answers_the_whole_escape_axis() {
    let base = TrialCorpus::build(State::CommittedSingletons);

    // The in-task base: one live task, one staged `adr`, so the `doc` rows' argv are
    // runnable and the token is the only thing their doors can fault on.
    let in_task = base.copy_state();
    in_task.jigc_ok(&["start", "--workflow", "single-task", "probe"]);
    in_task.jigc_ok(&[
        "doc", "create", "adr", "--title", "Probe", "--task", "probe",
    ]);

    // The freeze-exempt base: `jigc relocate` refuses a FROZEN doctype before it parses
    // `--from` at all, and every doctype both shipped packs carry is frozen — so the only
    // way to ask this occurrence its own question is a pack that ships no freeze manifest.
    let pack = FixturePack::from_dev_pack("path-arg-axis");
    let exempt = TrialCorpus::build_with_pack(State::Fresh, &pack);
    fs::create_dir_all(exempt.repo().join("oldhome")).expect("create the prior home");
    fs::write(
        exempt.repo().join("oldhome").join("keeper.md"),
        support::trial_corpus::read(&base.repo(), "docs/decisions-log.md"),
    )
    .expect("plant a committed doc at a prior home");
    exempt.git(&["add", "oldhome"]);
    exempt.git(&["commit", "-q", "-m", "a doc at a prior home"]);

    let mut driven: BTreeSet<(String, String, String, Cell)> = BTreeSet::new();
    let mut declared: BTreeSet<(String, String, String, Cell)> = BTreeSet::new();

    for occurrence in PATH_ARG_OCCURRENCES {
        let door = occurrence.door.join(" ");
        for arm in occurrence.arms {
            let cells = domain(occurrence, arm);
            assert!(
                !cells.is_empty(),
                "`jigc {door}`'s `{}` [{}] reaches no cell of the axis",
                occurrence.arg,
                arm.when,
            );
            for cell in cells {
                declared.insert((
                    door.clone(),
                    occurrence.arg.to_owned(),
                    arm.when.to_owned(),
                    cell,
                ));

                let source = match (door.as_str(), occurrence.arg) {
                    ("relocate", "from") => &exempt,
                    ("doc set-slot" | "doc author" | "doc set-field", _) => &in_task,
                    _ => &base,
                };
                let corpus = source.copy_state();
                plant(&corpus);
                let before = tree(&corpus);
                let planted = canary(&corpus);

                let token = token(cell, arm.subject, &corpus.repo());
                let shown = format!("jigc {door} [{}] · {cell:?} `{token}`", arm.when);
                let out = drive(&corpus, arm, &token);
                let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
                let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
                let expected = expectation(&door, occurrence.arg, arm, cell);

                match expected {
                    Outcome::PathRuleBlocks(code) => {
                        assert!(
                            !out.status.success(),
                            "{shown}: a path rule must block non-zero\n{stderr}",
                        );
                        assert!(
                            stderr.contains(&format!("· {code} — ")),
                            "{shown}: must carry `{code}`\n{stderr}",
                        );
                    }
                    Outcome::AnswersElsewhere(code) => {
                        assert!(
                            !out.status.success(),
                            "{shown}: must refuse non-zero\n{stderr}{stdout}",
                        );
                        assert!(
                            stderr.contains(&format!("· {code} — ")),
                            "{shown}: must answer with `{code}` — the closed vocabulary the \
                             row's stated no-rule rests on\n{stderr}",
                        );
                        assert_no_path_rule(&shown, &stderr);
                    }
                    Outcome::ReadAsContent => {
                        assert!(
                            !out.status.success(),
                            "{shown}: a payload the door cannot read must refuse\n{stdout}",
                        );
                        assert!(
                            stderr.contains("payload"),
                            "{shown}: must answer about the CONTENT it read, not about the \
                             path it read it from\n{stderr}",
                        );
                        assert_no_path_rule(&shown, &stderr);
                    }
                    Outcome::NotFound => {
                        assert!(
                            !out.status.success(),
                            "{shown}: a token naming no file must refuse\n{stdout}",
                        );
                        assert!(
                            stderr.contains(&token),
                            "{shown}: the read miss must name the token the caller typed\
                             \n{stderr}",
                        );
                        assert_no_path_rule(&shown, &stderr);
                    }
                    Outcome::Accepted => assert!(
                        out.status.success(),
                        "{shown}: the row's stated no-rule claims this token becomes no path \
                         component — so the door does its job\n{stderr}",
                    ),
                }

                // **The second composite cost, after every cell: what the door PRINTED can
                // be run.** The cell above says what the door decided; this says the
                // sentence it decided it in is not a dead end. It is asserted over every
                // cell rather than over the shell-unsafe one alone, because the escape
                // shapes already on this axis carry shell metachars of their own (`*.md`
                // globs, `:colon.md`), and a door that names one of them raw prints an
                // exit that runs against somebody else's files.
                assert_command_spans_run(&shown, &token, &stderr);
                assert_command_spans_run(&shown, &token, &stdout);

                // The composite cost, after every cell: the canary outside the repository
                // is untouched, and a refusal moved nothing at all.
                assert_eq!(canary(&corpus), planted, "{shown}: the canary was written");
                if expected != Outcome::Accepted {
                    assert_eq!(
                        tree(&corpus),
                        before,
                        "{shown}: a refusal left the tree changed",
                    );
                }

                driven.insert((
                    door.clone(),
                    occurrence.arg.to_owned(),
                    arm.when.to_owned(),
                    cell,
                ));
            }
        }
    }

    assert_eq!(
        driven, declared,
        "every declared (occurrence, arm, cell) is driven, and nothing else is",
    );
    assert_eq!(
        driven.len(),
        PATH_ARG_OCCURRENCES
            .iter()
            .flat_map(|occurrence| occurrence
                .arms
                .iter()
                .map(|arm| domain(occurrence, arm).len()))
            .sum::<usize>(),
        "the driven count is the registry's own",
    );
}

/// **Every backticked `git …` / `jigc …` span a door printed is copy-runnable** — the
/// route-followability half of each cell, asserted from the emitted bytes rather than from
/// a reconstruction.
///
/// Two claims, because one alone is blind. The token check
/// ([`engine::finding::command_spans_are_shell_safe`]) catches a span carrying a metachar a
/// shell would act on. It **cannot** catch the commonest spelling: an unquoted path with a
/// space is not one bad token, it is two perfectly inert ones, and `` `git add -- my
/// notes.md` `` passes every token check while exiting 128. The missing information is the
/// word boundary — and this suite has it, because it typed the token. So the second claim is
/// that a span naming the caller's own token names it as [`engine::finding::shell_token`]
/// would have written it.
fn assert_command_spans_run(shown: &str, token: &str, emitted: &str) {
    assert!(
        engine::finding::command_spans_are_shell_safe(emitted),
        "{shown}: printed a command span a shell does not re-lex as itself\n{emitted}",
    );
    let quoted = engine::finding::shell_token(token);
    if quoted == token {
        return; // the token needs no quoting; there is nothing to get wrong.
    }
    for span in emitted.split('`').skip(1).step_by(2) {
        let head = span.split_whitespace().next().unwrap_or_default();
        if (head != "git" && head != "jigc") || !span.contains(token) {
            continue;
        }
        assert!(
            span.contains(quoted.as_str()),
            "{shown}: the command span `{span}` names the caller's token unquoted — run \
             verbatim it is a different command (`{quoted}` is the spelling that is not)",
        );
    }
}

/// No refusal that a `NoRule` row predicts may carry a path-rule code — the assertion that
/// makes *"this token becomes no path component"* checkable rather than asserted.
fn assert_no_path_rule(shown: &str, stderr: &str) {
    for code in PATH_RULE_CODES {
        assert!(
            !stderr.contains(code),
            "{shown}: answered with the path rule `{code}`, but this occurrence's registered \
             disposition is a stated NO-rule — one of the two is wrong\n{stderr}",
        );
    }
}

/// **Every no-rule disposition is a reason, and every adjudicated one names a code this
/// suite drove** — the registry's prose and the suite's drives are one claim.
///
/// The `cli_parse` fence proves each row's argv reaches its door; this proves the other
/// half: a row that says *adjudicated* names codes that actually appear on the axis above,
/// so a disposition cannot claim a rule the binary does not have.
#[test]
fn every_adjudicated_disposition_names_a_code_the_axis_drives() {
    let mut expected: BTreeSet<&str> = BTreeSet::new();
    for occurrence in PATH_ARG_OCCURRENCES {
        let door = occurrence.door.join(" ");
        for arm in occurrence.arms {
            let PathArgDisposition::Adjudicated { codes, .. } = arm.disposition else {
                continue;
            };
            for code in codes {
                assert!(
                    PATH_RULE_CODES.contains(code),
                    "`jigc {door}`'s `{}` claims `{code}`, which this suite does not know as \
                     a path-rule code",
                    occurrence.arg,
                );
                expected.insert(code);
            }
        }
    }

    let mut driven: BTreeSet<&str> = BTreeSet::new();
    for occurrence in PATH_ARG_OCCURRENCES {
        let door = occurrence.door.join(" ");
        for arm in occurrence.arms {
            for cell in domain(occurrence, arm) {
                if let Outcome::PathRuleBlocks(code) = expectation(&door, occurrence.arg, arm, cell)
                {
                    driven.insert(code);
                }
            }
        }
    }

    assert_eq!(
        expected, driven,
        "every code a disposition claims is a code the axis drives, and the reverse",
    );
}

/// **`jigc relocate --from` refuses every root jigc's own install writes into** — the class
/// the escape axis above cannot carry, because its members are not a spelling anybody chose
/// (M52 Increment 8 / T7).
///
/// `--from` names a **prior home**, and the door sweeps every committed `.md` under it into
/// the doctype's home. Driven at `4c0c513e` on a freeze-exempt pack, twice. With one
/// ordinarily-named doc planted under each root, `jigc relocate adr --from .claude` and
/// `--from .jigc` each reported `1 moved` at exit **0** and staged
/// `R .claude/notes.md -> docs/decisions/notes.md` / `R .jigc/notes.md -> …`. On the bare
/// install footprint, with nothing planted, all three roots reported `0 moved, 1 blocked` at
/// exit **0** — jigc's own artifacts escaped only by T6's destination-identity gate, which
/// then printed, as the repair, `` git mv .jigc/AGENT.md docs/decisions/agent.md `` followed
/// by `jigc ingest`: a route that, followed, adopts the install as an ADR.
///
/// **The set is read, not written.** `.jigc` is spelled here because production spells it —
/// every door computes the workbench as `repo_root.join(".jigc")` — while the adapter's roots
/// come from [`cli::config::installed_artifact_roots`], which derives them from the installed
/// profile's declared artifacts. A profile that declares an artifact somewhere new joins this
/// class with no edit here, which is the half a hand-written `.claude` could never have.
#[test]
fn relocate_refuses_every_root_jigcs_own_install_writes_into() {
    let pack = FixturePack::from_dev_pack("relocate-installed-roots");
    let base = TrialCorpus::build_with_pack(State::Fresh, &pack);

    let mut roots: Vec<(String, &str)> = vec![(".jigc".to_owned(), "config.workbench-root")];
    roots.extend(
        cli::config::installed_artifact_roots()
            .into_iter()
            .map(|root| (root, "config.unusable-root")),
    );
    assert!(
        roots.len() > 1,
        "the installed profile declares artifacts, so this class has adapter members too — \
         an empty derivation would make this suite assert nothing",
    );

    for (root, code) in roots {
        let corpus = base.copy_state();
        // A doc whose basename IS a doc id, so the destination-identity gate (T6) cannot be
        // what answers: at HEAD this file was the one that moved. A root that is a *file*
        // (the always-loaded bootstrap file) has nothing to plant under and is swept as
        // itself.
        if corpus.repo().join(&root).is_dir() {
            let planted = format!("{root}/notes.md");
            fs::write(corpus.repo().join(&planted), "# Notes\n\nSome prose.\n")
                .expect("plant a doc under an installed root");
            corpus.git(&["add", "-f", &planted]);
            corpus.git(&["commit", "-q", "-m", "a doc under an installed root"]);
        }

        let before = tree(&corpus);
        let out = corpus.jigc(&["relocate", "adr", "--from", &root]);
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let shown = format!("jigc relocate adr --from {root}");

        assert!(
            !out.status.success(),
            "{shown}: a prior home naming jigc's own install must refuse\n{stdout}{stderr}",
        );
        assert!(
            stderr.contains(&format!("· {code} — ")),
            "{shown}: must refuse with the sibling door's shipped `{code}`\n{stderr}",
        );
        assert!(
            stderr.contains("route:"),
            "{shown}: a blocking refusal carries a route\n{stderr}",
        );
        assert_command_spans_run(&shown, &root, &stderr);
        assert_eq!(
            tree(&corpus),
            before,
            "{shown}: the refusal moved something",
        );
    }
}

// ---------------------------------------------------------------------------------------
// The cwd axis (M53 — the cwd census, rows C2-03 / C2-04).
// ---------------------------------------------------------------------------------------

/// **Every occurrence's stated base, driven from a subdirectory.**
///
/// The escape axis above asks *what a door decides about a token*. This asks the question
/// one step earlier and never asked before: **what the token is resolved against**. The
/// census drove all fourteen occurrences from `docs/deep` and found two opposite bases on
/// one binary with no surface stating either — `--from-file ./pay.txt` worked while
/// `jigc migrate note.md` did not, and `jigc migrate ../../rootnote.md` was refused as
/// *"resolves outside the repository"* about a file inside it.
///
/// The set iterated is [`PATH_ARG_OCCURRENCES`] — the same registry, the same ⇔ fence — so a
/// fifteenth occurrence cannot ship without stating a base, and a stated base cannot ship
/// without being driven. Each arm is run **from a subdirectory** with the spelling its own
/// base claims will work:
///
/// * [`PathArgBase::Cwd`] — a bare basename naming a file that exists **only** beside the
///   caller. A root-based door cannot find it.
/// * [`PathArgBase::RepoRoot`] — a repo-relative spelling of a file that exists only at the
///   root. A cwd-based door cannot find it.
/// * [`PathArgBase::NotAPath`] — skipped, and the skip is the claim: the arm has no base,
///   which the type already says and no drive could add to.
///
/// The predicate is deliberately narrow — **the door must not complain about resolving the
/// token** — because the base is the only thing under test. Every door here has its own
/// downstream verdicts (a doctype it will not migrate, a step id it will not replace), and
/// asserting those would be asserting the fixture.
#[test]
fn every_path_arg_occurrence_resolves_its_token_against_the_base_it_states() {
    let base = TrialCorpus::build(State::CommittedSingletons);
    let in_task = base.copy_state();
    in_task.jigc_ok(&["start", "--workflow", "single-task", "probe"]);
    in_task.jigc_ok(&[
        "doc", "create", "adr", "--title", "Probe", "--task", "probe",
    ]);
    let pack = FixturePack::from_dev_pack("path-arg-cwd");
    let exempt = TrialCorpus::build_with_pack(State::Fresh, &pack);

    /// What a door says when it could not resolve the token — the only failure this arm is
    /// about. Anything else is the door's own downstream verdict.
    const UNRESOLVED: &[&str] = &[
        "could not read",
        "No such file",
        "no such file",
        "resolves outside the repository",
    ];

    let mut driven = 0usize;
    let mut skipped = 0usize;
    for occurrence in PATH_ARG_OCCURRENCES {
        let door = occurrence.door.join(" ");
        for arm in occurrence.arms {
            let (spelling, why) = match arm.base {
                PathArgBase::NotAPath { .. } => {
                    skipped += 1;
                    continue;
                }
                // Beside the caller, and nowhere else.
                PathArgBase::Cwd => ("cwd-probe.yaml".to_owned(), "the caller's cwd"),
                // At the root, and nowhere else.
                PathArgBase::RepoRoot { .. } => {
                    ("root-probe.yaml".to_owned(), "the repository root")
                }
            };
            let source = match (door.as_str(), occurrence.arg) {
                ("relocate", "from") => &exempt,
                ("doc set-slot" | "doc author" | "doc set-field", _) => &in_task,
                _ => &base,
            };
            let corpus = source.copy_state();
            let deep = corpus.repo().join("docs").join("deep");
            fs::create_dir_all(&deep).expect("create the subdirectory the caller stands in");
            // One file beside the caller and one at the root, each named so that only a door
            // resolving against the matching base can reach it.
            fs::write(deep.join("cwd-probe.yaml"), "probe: beside the caller\n")
                .expect("plant the cwd probe");
            fs::write(
                corpus.repo().join("root-probe.yaml"),
                "probe: at the root\n",
            )
            .expect("plant the root probe");
            // Tracked, so a trackedness leg is not the thing that answers.
            corpus.git(&["add", "docs/deep/cwd-probe.yaml", "root-probe.yaml"]);
            corpus.git(&["commit", "-q", "-m", "plant the base probes"]);

            // The `AddressTail` and `Home` subjects take the same spelling in their own
            // shape; the `Home` arms name a directory, so the probe is its parent.
            let token = match arm.subject {
                PathArgSubject::AddressTail(head) => format!("{head}{spelling}"),
                PathArgSubject::Home => match arm.base {
                    PathArgBase::Cwd => "docs/deep".to_owned(),
                    _ => "docs".to_owned(),
                },
                _ => spelling.clone(),
            };
            let argv: Vec<String> = arm
                .argv
                .iter()
                .map(|part| {
                    if *part == PATH_ARG_SLOT {
                        token.clone()
                    } else {
                        (*part).to_owned()
                    }
                })
                .collect();
            let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
            let out = corpus.jigc_stdin_from(&deep, &argv, CANARY);
            let seen = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );
            for complaint in UNRESOLVED {
                assert!(
                    !seen.contains(complaint),
                    "`jigc {door}`'s `{}` [{}] states its base is {why}, so `{token}` typed \
                     from `docs/deep` must resolve — the door answered `{complaint}`:\n{seen}",
                    occurrence.arg,
                    arm.when,
                );
            }
            driven += 1;
        }
    }

    assert!(
        driven >= 10 && skipped >= 5,
        "the cwd axis must reach most of the registry and skip only the based-on-nothing \
         arms; drove {driven}, skipped {skipped}",
    );
}
