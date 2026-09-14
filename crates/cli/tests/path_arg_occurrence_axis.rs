//! M51 Increment 1 / T6 — **every occurrence of every path-bearing argument, over the
//! whole escape-shape axis.**
//!
//! ## The class this closes
//!
//! A caller-supplied token that becomes a path component reaches the filesystem — at the
//! door, or one commit closure later at the destructive sink. Until this increment `jigc
//! migrate` took its `<path>` as an opaque token: driven at `abd81df`, an **absolute path
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
//! ## The nine cells
//!
//! `{absolute, ../ escape, symlink escape, .git/ component, workbench root, untracked
//! in-repo, leading-colon pathspec magic, a wildmatch glob, the - stdin sentinel}` — the
//! escape shapes the wave was chartered on, plus the sentinel that is **not** an escape and
//! is driven for exactly that reason: it is the arm split the registry's key exists to
//! express.
//!
//! **The ninth cell is the eighth's own hole, and it is why the axis has a *magic* dimension
//! rather than a `:` one.** Git has two pathspec magics: the `:` prefix, and wildmatch (`*`,
//! `?`, `[`, `\`), which needs no prefix at all. The colon cell's token — `:(top)README.md` —
//! names **no readable file**, so every door that reads before it adjudicates answered
//! `NotFound`, and the expectation table recorded that as a disposition rather than as a
//! blind spot: the whole class was left to the sink on the stated ground that the door
//! refuses such a token anyway. `*.md` is the spelling that ground is false for. Driven at
//! `8bc6f4e`, on a corpus holding six tracked `.md` files, with the literal file `*.md`
//! planted untracked: `jigc migrate '*.md' --as changelog` read it, asked
//! `git ls-files -- '*.md'` whether git held a copy, was told **yes** about six other
//! people's files, and minted at exit 0 — after which `--approve` unlinked the source
//! (recoverable from no git object) and `git add -- '*.md'` swept an unrelated unstaged edit
//! into a commit naming the changelog. The cell therefore plants a file that **exists** and
//! whose name **matches files git holds**; a cell naming nothing can only ever prove that
//! nothing is named.
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
    ArmToken, PATH_ARG_OCCURRENCES, PATH_ARG_SLOT, PathArgArm, PathArgDisposition,
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
    /// A leading `:` — what git reads as pathspec magic (`:(top)`, `:!`), never as a name.
    PathspecMagic,
    /// Git's OTHER magic: a wildmatch pattern (`*`) that names a file which **exists** while
    /// matching files git holds. The cell above could not reach this, and that is why it is
    /// its own cell rather than a second spelling of one (see the module header).
    PathspecGlob,
    /// `-`, the declared stdin sentinel. Not an escape: the arm split itself.
    StdinSentinel,
}

impl Cell {
    const ALL: [Cell; 9] = [
        Cell::Absolute,
        Cell::ParentEscape,
        Cell::Symlink,
        Cell::GitComponent,
        Cell::WorkbenchRoot,
        Cell::UntrackedInRepo,
        Cell::PathspecMagic,
        Cell::PathspecGlob,
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
            Cell::PathspecMagic => ":(top)README.md".to_owned(),
            Cell::PathspecGlob => GLOB_SOURCE.to_owned(),
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
            // unrelated reason that it names no readable file"*. That is true of
            // `:(top)README.md` and false of `*.md`: a glob names a file that **exists**, so
            // the door read it, asked its trackedness leg a `git ls-files` that answered
            // about six OTHER tracked files, and minted at exit 0 a deletion target no git
            // object holds. One spelling of a rule is not the rule, so the class is asked in
            // the home the door and the sink share — and the colon cell stops being answered
            // by a read miss that names the wrong problem.
            PathspecMagic | PathspecGlob => PathRuleBlocks("migrate.source-untrackable"),
            StdinSentinel => NotFound,
        },
        // `jigc unmanage <path>` — a lookup key over the recorded store, never a path op.
        ("unmanage", "path", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | StdinSentinel => Accepted,
        },
        // `jigc relocate --from <prior-home>` — a prefix over committed spellings.
        ("relocate", "from", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | StdinSentinel => Accepted,
        },
        // The two `<file>` occurrences — a SOURCE rule: an out-of-repo source is admitted
        // and copied in; git's own directory and jigc's transient workbench are not.
        ("config insert-step" | "config replace-step", "file", _, _) => match cell {
            GitComponent | WorkbenchRoot => PathRuleBlocks("config.step-source-untrackable"),
            // The glob cell joins the accepting set rather than the refusing one, and that is
            // the class boundary stated as an outcome: these doors `std::fs::read` the token
            // and never hand it to git, so a name that is a pattern is just a name.
            Absolute | ParentEscape | Symlink | UntrackedInRepo | PathspecGlob => Accepted,
            // `-` is read as a file name at these two doors, not as stdin — which is the
            // discriminator that earns them a rule while `--from-file` takes none.
            PathspecMagic | StdinSentinel => NotFound,
        },
        // `--from-file` at the three handoff doors: the sentinel arm, then the path arm.
        ("config fill" | "doc set-slot", "from_file", ArmToken::Literal(_), _) => match cell {
            StdinSentinel => Accepted,
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob => {
                unreachable!("the sentinel arm answers for `-` and no other token")
            }
        },
        ("config fill" | "doc set-slot", "from_file", ArmToken::Caller, _) => match cell {
            // Any bytes are prose / fill content — the token was a source to open.
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecGlob => Accepted,
            PathspecMagic => NotFound,
            StdinSentinel => unreachable!("the sentinel arm claims this cell"),
        },
        ("doc author", "from_file", ArmToken::Literal(_), _) => match cell {
            StdinSentinel => Accepted,
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob => {
                unreachable!("the sentinel arm answers for `-` and no other token")
            }
        },
        ("doc author", "from_file", ArmToken::Caller, _) => match cell {
            Absolute | ParentEscape | Symlink | UntrackedInRepo | PathspecGlob => Accepted,
            // Read, then answered about the PAYLOAD — the no-rule claim in its plainest
            // form: git's config and jigc's own state file are opened like any other
            // source and rejected for what they say, not for where they are.
            GitComponent | WorkbenchRoot => ReadAsContent,
            PathspecMagic => NotFound,
            StdinSentinel => unreachable!("the sentinel arm claims this cell"),
        },
        // The `target` occurrences — resolved against a closed vocabulary before any write.
        ("config replace-step" | "config remove-step" | "config fork", "target", _, _) => {
            match cell {
                Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot
                | UntrackedInRepo | PathspecMagic | PathspecGlob | StdinSentinel => {
                    AnswersElsewhere("config.anchor-absent")
                }
            }
        }
        ("config fill", "target", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | StdinSentinel => {
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
            // is an ordinary (if odd) directory name, so refusing it would refuse a home.
            UntrackedInRepo | StdinSentinel => Accepted,
        },
        ("config set", "value", _, PathArgSubject::FieldValue) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | StdinSentinel => {
                AnswersElsewhere("config.value-rejected")
            }
        },
        // `jigc doc set-field --value` — the value is document content.
        ("doc set-field", "value", _, _) => match cell {
            Absolute | ParentEscape | Symlink | GitComponent | WorkbenchRoot | UntrackedInRepo
            | PathspecMagic | PathspecGlob | StdinSentinel => Accepted,
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
