//! M52 Increment 1 / T3 — **one pre-dispatch funnel, and the faults become a registry
//! with a phase.**
//!
//! The class ([M52 baseline-contracts.md](../../../completions/artifacts/M52/baseline-contracts.md)
//! §1.3, §3.5): *anything that writes non-JSON to stderr under `--format json`* — and its
//! sharpest members are the failures that happen **before or beside** a verb's own reject
//! funnel, where nothing has yet decided what a document looks like. Two of the eleven
//! failure points the baseline enumerated bypassed or corrupted both funnels at rc.15:
//!
//!   * **the working directory jigc cannot read** — 23 `cli.rs` match arms each printed
//!     `cannot determine the current directory: …` as **plain prose** and returned
//!     `Outcome::failure()`, so all 47 leaves answered `--format json` with bytes no
//!     driver can parse;
//!   * **a malformed `.jigc/config/packs.yaml`** — `pack::make_pack` swallowed the
//!     located error into an `eprintln!("warning: …")` at **two** sites, and
//!     `invocation_log::enabled_logs_dir` calls `make_pack` before `Cli::try_parse()`, so
//!     the pair fired **twice** per run and **before the format existed**: an ordinary
//!     `jigc --format json doc list` emitted four `warning:` lines carrying a host-absolute
//!     path, and `jigc --version` — which dispatches nothing at all — emitted two.
//!
//! **What the suite asserts, per cell:** a fault that a leaf *reaches* is **answered**,
//! and a run that answers a fault emits **exactly one JSON value, on the declared stream,
//! and no other bytes**. A fault a leaf does **not** reach changes nothing: the cell is
//! driven against a **control** run of the same argv over a sound corpus and must match it
//! shape-for-shape. That second half is not a skip — an omitted cell is an untested cell,
//! and an untested cell is how a fault reaches a leaf nobody looked at.
//!
//! **Why "on the declared stream" and not "on both".** The shipped contract routes every
//! agent-text side channel to stderr, which is correct while **stdout** owns the document
//! ([command-output-contract.md](../../../design/command-output-contract.md) → Stream
//! discipline), and `jigc uninstall`'s removal narration is a deliberate instance. What T1
//! of this increment settled — and what this suite holds one phase earlier — is the other
//! half: on the arm whose document is **stderr's**, nothing else is written there.
//!
//! **The axis is two code-side registries under a stated applicability relation**
//! ([settle-record](../../../completions/artifacts/M52/settle-record.md) §12):
//! [`PRE_DISPATCH_FAULTS`] × [`cli::cli::VERB_KINDS`], each cell carrying an expected
//! `(arm, exit)` and [`expectation`] **panicking** — never skipping — on a cell no row
//! claims. The fault rows are count-fenced against the source on the `UNSWEPT_PRODUCERS`
//! mold (`crates/cli/tests/repo_relative_paths.rs`): *a bound nothing measures is a
//! sentence*.
//!
//! **The one site that was declared out is closed, and the declaration with it.** At rc.15
//! `cli.rs` held **24** `std::env::current_dir()` occurrences and only **23** of them
//! rendered the refusal; the 24th was `refuse_on_posture`'s `current_dir().ok()?`, which
//! skipped the whole M51 posture family **without a word** on the same fault — a door
//! adjudicating on behalf of a repository it could not name, and answering `None` as if it
//! had found nothing to refuse. Increment 1 declared that hole on [`Fault::unanswered`]
//! rather than omitting it, because closing it is a control-flow change on an
//! `Option<Outcome>` and not an expression substitution; Increment 3 / T3 (D2.4) closes it
//! by routing that reader through the same seam, so the guard **refuses** the fault
//! instead of standing down on it. The module now holds **one** reader of the process cwd
//! — the seam — with **24** callers reaching it, and this fault's `unanswered` set is
//! empty. Both halves stay measured, so neither can go stale in silence.
//!
//! **What catches a guard that calls the seam and then stands down anyway** is the driven
//! arm, not the count — and that is worth writing down, because the arm and the exit are
//! deliberately *unchanged* by D2.4, so the obvious reading is that nothing driveable can
//! tell the two apart. It can. [`cwd_or_refusal`] renders and logs as it constructs, so a
//! guard that calls it and discards what it returns falls through to the dispatch arm
//! behind it and that arm calls the seam again: the run emits the document **twice**, and
//! [`every_reachable_cell_answers_its_fault_with_one_document`] fails on `stderr: NotJson`
//! at the first acting leaf. Applied as a mutant (`Err(_) => return None`) before this was
//! written, it reddened exactly there.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use crate::support::leaf_argv;
use crate::support::rust_source;
use crate::support::trial_corpus::{State, TrialCorpus, unique_root};
use cli::cli::VERB_KINDS;

// ---------------------------------------------------------------------------------
// The registry
// ---------------------------------------------------------------------------------

/// How far a run has got when the fault fires — the **applicability relation**: it is
/// what decides which leaves can reach a fault at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Phase {
    /// Before jigc has located anything: the process cwd itself is unreadable, so no
    /// repository, project layer or pack has been looked for. **Every** leaf reaches it.
    BeforeDiscovery,
    /// The repository and its `.jigc/config/` layer are found and the pack-set is being
    /// assembled from them. Only a leaf that loads a pack reaches it.
    AfterDiscovery,
    /// The pack source exists and a **resource read through it** misses. Only a leaf that
    /// asks the loaded pack for something reaches it.
    AfterPackLoad,
}

/// The state a cell is driven in — the fault's **fixture constructor**, as a value rather
/// than a closure so the registry stays a `const`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Fixture {
    /// Run with a working directory that has been removed out from under the process.
    DeletedCwd,
    /// A set-up repository whose `.jigc/config/packs.yaml` is not valid YAML.
    MalformedPacksYaml,
    /// A set-up repository whose `JIGC_PACK_DIR` names a directory that is not a pack.
    EmptyPackDir,
}

/// A counted source scan: `(file, needle, occurrences)` over **production** code —
/// comments and, for a code idiom, string literals blanked, `#[cfg(test)]` bodies
/// excluded. A count of `0` is a live assertion too: it is how "nothing swallows this
/// fault any more" is stated as a measurement instead of a sentence.
type Sites = (&'static str, &'static str, usize, Reading);

/// Which reading of the source a needle wants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reading {
    /// Code with comments **and** string literals blanked — for an idiom
    /// (`std::env::current_dir()`), where prose mentioning it is not an instance of it.
    Code,
    /// Code with comments blanked and literals **kept** — for a needle that exists in
    /// source only as a `"…"` (a finding code, a message stem).
    Literals,
}

/// One pre-dispatch fault.
pub(crate) struct Fault {
    /// The row's id — the key the expectation table and the failure messages use.
    pub(crate) id: &'static str,
    /// How far the run had got when it fired; the applicability relation above.
    pub(crate) phase: Phase,
    /// The state that produces it.
    pub(crate) fixture: Fixture,
    /// The production sites that can **raise** it, and the sites that must (or must not)
    /// exist for it to be answered — each measured against the source.
    sites: &'static [Sites],
    /// Sites that reach this fault and do **not** answer it, each with the reason and the
    /// increment that owns closing it. A declared hole, never a silent one.
    unanswered: &'static [(&'static str, &'static str)],
}

/// The class, one row per fault. Iterated × [`VERB_KINDS`] by the arms below — and, since
/// M52 Increment 11, by `flow53_acceptance.rs`' arm 6, which crosses it with
/// `cli::render::ENVELOPE_ARMS`. That consumer is the reason the row type and its three
/// read fields are `pub(crate)` rather than private: the acceptance arm reads **this**
/// registry instead of re-deriving a second one, so a fault added here joins the
/// done-picture walk with no edit there.
pub(crate) const PRE_DISPATCH_FAULTS: &[Fault] = &[
    Fault {
        id: "cwd-unreadable",
        phase: Phase::BeforeDiscovery,
        fixture: Fixture::DeletedCwd,
        sites: &[
            // Every reader of the process cwd in the dispatch module — **one**: the seam
            // below. At rc.15 there were 24 — one per dispatch arm, which is what made the
            // fault answerable 23 different ways — and through Increment 1 there were two,
            // the seam plus `refuse_on_posture`'s `.ok()?`, which answered it in no way at
            // all. A second reader re-appearing here is the whole class re-opening.
            (
                "crates/cli/src/cli.rs",
                "std::env::current_dir()",
                1,
                Reading::Code,
            ),
            // The 23 dispatch arms **and the posture guard**, each reaching that seam. The
            // 24th is D2.4: a door that commits or moves on the user's behalf refuses the
            // fault rather than standing its whole guard down on it.
            (
                "crates/cli/src/cli.rs",
                "cwd_or_refusal(format)",
                24,
                Reading::Code,
            ),
            // … and none of which prints the refusal as prose any more. The needle is the
            // sentence itself: a 25th site copying its neighbour would reintroduce it.
            (
                "crates/cli/src/cli.rs",
                "cannot determine the current directory",
                1,
                Reading::Literals,
            ),
        ],
        unanswered: &[],
    },
    Fault {
        id: "packs-yaml-malformed",
        phase: Phase::AfterDiscovery,
        fixture: Fixture::MalformedPacksYaml,
        sites: &[
            // The two readers of the one file, each composing the located message.
            (
                "crates/cli/src/pack.rs",
                "is not a valid pack-set list",
                2,
                Reading::Literals,
            ),
            // Nothing swallows it into a warning any more — the measurement that replaces
            // the sentence. `make_pack`'s own doc-comment has said *propagated, not
            // swallowed* since M14; until M52 the body said otherwise.
            ("crates/cli/src/pack.rs", "warning: ", 0, Reading::Literals),
        ],
        unanswered: &[],
    },
    Fault {
        id: "pack-resource-missing",
        phase: Phase::AfterPackLoad,
        fixture: Fixture::EmptyPackDir,
        sites: &[(
            "crates/cli/src/start.rs",
            "pack.resource-missing",
            1,
            Reading::Literals,
        )],
        unanswered: &[],
    },
];

// ---------------------------------------------------------------------------------
// The expectation column
// ---------------------------------------------------------------------------------

/// The root shape a stream's bytes deserialize to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arm {
    /// Nothing at all.
    Empty,
    /// The operational-error reject arm: `{"error": …}` and nothing else.
    Error,
    /// The findings reject arm: `{"findings": …, "schema_version": …}`.
    Findings,
    /// Any other JSON object (a verb's own success envelope).
    Object,
    /// A JSON array (`task list`'s rows).
    Array,
    /// Bytes that are not one JSON value.
    NotJson,
}

/// What a `(fault, leaf)` cell must do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expect {
    /// This leaf **reaches** the fault and answers it: one JSON value of this shape on
    /// stderr at this exit code, stdout empty, and nothing else on either stream.
    Answers(Arm, i32),
    /// This leaf never reaches the fault — the phase relation says so. The cell is still
    /// driven, against a **control** run of the same argv over a sound corpus, and must
    /// match it shape-for-shape.
    Unreached,
}

/// Every `(fault, leaf)` cell, keyed by the fault's id and the leaf's argv path.
///
/// **A missing cell is a panic, never a skip** ([`expectation`]). The two `Unreached`
/// blocks are the phase filter written down: a leaf that loads no pack has no
/// after-discovery cell to answer, and the table says which leaves those are rather than
/// leaving the reader to infer it from an omission.
type Cell = (&'static str, &'static [&'static str], Expect);

const EXPECTATIONS: &[Cell] = &[
    // -------------------------------------------------------------------------
    // `cwd-unreadable` — before discovery, so every one of the 47 reaches it.
    // -------------------------------------------------------------------------
    ("cwd-unreadable", &["start"], Expect::Answers(Arm::Error, 1)),
    (
        "cwd-unreadable",
        &["workflow"],
        Expect::Answers(Arm::Error, 1),
    ),
    ("cwd-unreadable", &["setup"], Expect::Answers(Arm::Error, 1)),
    (
        "cwd-unreadable",
        &["uninstall"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["upgrade"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["ingest"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["migrate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["migrate-corpus"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["unmanage"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["rename"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["relocate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["describe"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["validate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "create"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "add-item"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "remove-item"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "retitle-item"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "rename"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "set-field"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "set-slot"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "author"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "show"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "schema"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["doc", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["task", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["task", "diff"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["task", "validate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["task", "discard"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["task", "finalize"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["task", "bind"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "set"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "insert-step"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "replace-step"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "remove-step"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "fill"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "fork"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "get"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["config", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "create"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "add-task"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "add-from-spec"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "list-tasks"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "provision"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "execute"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "join"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "finalize"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "cwd-unreadable",
        &["milestone", "discard"],
        Expect::Answers(Arm::Error, 1),
    ),
    // -------------------------------------------------------------------------
    // `packs-yaml-malformed` — after discovery. A leaf reaches it iff, with the argv
    // the shared table gives it, it asks for the pack-set before anything else
    // refuses. `setup` and `uninstall` adjudicate their own install paths first; the
    // `doc` writers, the `task` verbs and the `config` step-editors resolve a task or
    // a step first.
    // -------------------------------------------------------------------------
    (
        "packs-yaml-malformed",
        &["start"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["workflow"],
        Expect::Answers(Arm::Error, 1),
    ),
    // `setup` reaches the fault through its **own** reader — it writes the compose marker
    // into the same file — and answers with its door's identity on the findings arm, which
    // is a stronger answer than `{error}`: it carries a `(code, target)` key and a route.
    // (Observed beside it, and left alone as a surface-tier matter outside this task's
    // class: `setup.compose-marker`'s route says *ensure `.jigc/config/` is writable* for
    // a fault that is a YAML parse error, not a permission one. Pre-existing at rc.15 —
    // nothing in this increment touches `setup.rs`.)
    (
        "packs-yaml-malformed",
        &["setup"],
        Expect::Answers(Arm::Findings, 1),
    ),
    ("packs-yaml-malformed", &["uninstall"], Expect::Unreached),
    (
        "packs-yaml-malformed",
        &["upgrade"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["ingest"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["migrate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["migrate-corpus"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["unmanage"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["rename"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["relocate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["describe"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["validate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["doc", "create"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "add-item"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "remove-item"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "retitle-item"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "rename"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "set-field"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "set-slot"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "author"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["doc", "show"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["doc", "schema"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["doc", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    ("packs-yaml-malformed", &["task", "list"], Expect::Unreached),
    ("packs-yaml-malformed", &["task", "diff"], Expect::Unreached),
    (
        "packs-yaml-malformed",
        &["task", "validate"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["task", "discard"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["task", "finalize"],
        Expect::Unreached,
    ),
    ("packs-yaml-malformed", &["task", "bind"], Expect::Unreached),
    (
        "packs-yaml-malformed",
        &["config", "set"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["config", "insert-step"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["config", "replace-step"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["config", "remove-step"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["config", "fill"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["config", "fork"],
        Expect::Unreached,
    ),
    (
        "packs-yaml-malformed",
        &["config", "get"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["config", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "create"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "add-task"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "add-from-spec"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "list-tasks"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "provision"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "execute"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "join"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "finalize"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "packs-yaml-malformed",
        &["milestone", "discard"],
        Expect::Answers(Arm::Error, 1),
    ),
    // -------------------------------------------------------------------------
    // `pack-resource-missing` — after pack load. A leaf reaches it iff it reads a
    // resource **through** the loaded pack. This row is the hold-the-line row: it was
    // already correct at rc.15, and it is here so that the phase vocabulary is
    // populated by a driven fault rather than declared over an empty set.
    // -------------------------------------------------------------------------
    (
        "pack-resource-missing",
        &["start"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["workflow"],
        Expect::Answers(Arm::Error, 1),
    ),
    ("pack-resource-missing", &["setup"], Expect::Unreached),
    ("pack-resource-missing", &["uninstall"], Expect::Unreached),
    (
        "pack-resource-missing",
        &["upgrade"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["ingest"],
        Expect::Answers(Arm::Error, 1),
    ),
    ("pack-resource-missing", &["migrate"], Expect::Unreached),
    (
        "pack-resource-missing",
        &["migrate-corpus"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["unmanage"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["rename"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["relocate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["describe"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["validate"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["doc", "create"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "add-item"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "remove-item"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "retitle-item"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "rename"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "set-field"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "set-slot"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "author"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["doc", "show"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["doc", "schema"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["doc", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["task", "list"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["task", "diff"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["task", "validate"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["task", "discard"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["task", "finalize"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["task", "bind"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["config", "set"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["config", "insert-step"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["config", "replace-step"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["config", "remove-step"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["config", "fill"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["config", "fork"],
        Expect::Unreached,
    ),
    (
        "pack-resource-missing",
        &["config", "get"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["config", "list"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "create"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "add-task"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "add-from-spec"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "list-tasks"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "provision"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "execute"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "join"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "finalize"],
        Expect::Answers(Arm::Error, 1),
    ),
    (
        "pack-resource-missing",
        &["milestone", "discard"],
        Expect::Answers(Arm::Error, 1),
    ),
];

/// The `(fault, leaf)` cell's expectation — **panicking** when no row claims it.
///
/// A skip here would be the vacuous pass this whole class is about: the cell would be
/// driven by nothing, and a fault reaching a leaf nobody decided about would ship green.
fn expectation(fault: &str, leaf: &[&str]) -> Expect {
    EXPECTATIONS
        .iter()
        .find(|(id, path, _)| *id == fault && path == &leaf)
        .map(|(_, _, expect)| *expect)
        .unwrap_or_else(|| {
            panic!(
                "no expectation for the cell (`{fault}`, `jigc {}`). Every \
                 PRE_DISPATCH_FAULTS × VERB_KINDS cell needs one — `Answers(arm, exit)` \
                 when the leaf reaches the fault, `Unreached` when the phase relation \
                 says it cannot. Deciding is the point; omitting is how a fault reaches \
                 a leaf nobody looked at.",
                leaf.join(" "),
            )
        })
}

// ---------------------------------------------------------------------------------
// Driving
// ---------------------------------------------------------------------------------

/// Classify one stream's bytes by their **root shape**.
fn arm_of(bytes: &[u8]) -> Arm {
    let text = String::from_utf8_lossy(bytes);
    if text.trim().is_empty() {
        return Arm::Empty;
    }
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return Arm::NotJson;
    };
    match &value {
        serde_json::Value::Array(_) => Arm::Array,
        serde_json::Value::Object(map) => {
            if map.len() == 1 && map.contains_key("error") {
                Arm::Error
            } else if map.contains_key("findings") && map.contains_key("schema_version") {
                Arm::Findings
            } else {
                Arm::Object
            }
        }
        _ => Arm::Object,
    }
}

/// Run `jigc --format json <argv>` in `cwd` with `$HOME = home`, `JIGC_PACK_DIR` either
/// removed or set — the pack the child reads is chosen here, never inherited.
fn drive(cwd: &Path, home: &Path, pack_dir: Option<&Path>, argv: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .arg("--format")
        .arg("json")
        .args(argv)
        .current_dir(cwd)
        .env("HOME", home);
    match pack_dir {
        Some(dir) => command.env("JIGC_PACK_DIR", dir),
        None => command.env_remove("JIGC_PACK_DIR"),
    };
    command.output().expect("spawn jigc")
}

/// Run `jigc --format json <argv>` from a working directory that **no longer exists**.
///
/// It cannot be done with [`Command::current_dir`]: the child `chdir`s before `exec`, so a
/// removed directory fails the spawn itself with `ENOENT` and jigc never runs. A shell
/// does it the way a human hits this state — `cd` into the directory, remove it, and carry
/// on with a dangling cwd — and `exec` hands that cwd straight to the binary. The argv
/// rides as positional parameters, so nothing here is re-quoted or re-split.
fn drive_from_a_deleted_cwd(home: &Path, argv: &[&str]) -> Output {
    let gone = unique_root("pre-dispatch-gone");
    fs::create_dir_all(&gone).expect("create the doomed cwd");
    let mut command = Command::new("sh");
    command
        .arg("-c")
        .arg(r#"cd "$1" || exit 90; rmdir "$1" || exit 91; shift; exec "$@""#)
        .arg("sh")
        .arg(&gone)
        .arg(env!("CARGO_BIN_EXE_jigc"))
        .arg("--format")
        .arg("json")
        .args(argv)
        .env("HOME", home)
        .env_remove("JIGC_PACK_DIR");
    let out = command.output().expect("spawn jigc through sh");
    assert!(
        !matches!(out.status.code(), Some(90) | Some(91)),
        "the deleted-cwd fixture failed to build the state it tests (exit {:?}); the cell \
         below would have been driven against an ordinary cwd",
        out.status.code(),
    );
    out
}

/// One driven cell, as the arms compare it.
#[derive(Debug, PartialEq, Eq)]
struct Observed {
    exit: Option<i32>,
    stdout: Arm,
    stderr: Arm,
}

fn observe(out: &Output) -> Observed {
    Observed {
        exit: out.status.code(),
        stdout: arm_of(&out.stdout),
        stderr: arm_of(&out.stderr),
    }
}

/// Render a cell's raw streams for a failure message — the bytes, not a summary.
fn detail(out: &Output) -> String {
    format!(
        "--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    )
}

// ---------------------------------------------------------------------------------
// The fences — the axis is the product of two registries
// ---------------------------------------------------------------------------------

/// [`EXPECTATIONS`] ⇔ [`PRE_DISPATCH_FAULTS`] × [`VERB_KINDS`]: every cell of the product
/// is claimed exactly once, and no row names a fault or a leaf that does not exist.
#[test]
fn every_cell_of_the_product_carries_exactly_one_expectation() {
    leaf_argv::assert_covers_every_leaf_verb();

    for fault in PRE_DISPATCH_FAULTS {
        for (path, _) in VERB_KINDS {
            let hits = EXPECTATIONS
                .iter()
                .filter(|(id, leaf, _)| *id == fault.id && leaf == path)
                .count();
            assert_eq!(
                hits,
                1,
                "the cell (`{}`, `jigc {}`) is claimed {hits} times; it needs exactly one",
                fault.id,
                path.join(" "),
            );
        }
    }
    for (id, leaf, _) in EXPECTATIONS {
        assert!(
            PRE_DISPATCH_FAULTS.iter().any(|fault| fault.id == *id),
            "the expectation row for `jigc {}` names the fault `{id}`, which the registry \
             does not carry",
            leaf.join(" "),
        );
        assert!(
            VERB_KINDS.iter().any(|(path, _)| path == leaf),
            "the `{id}` expectation names `jigc {}`, a path the clap tree no longer has",
            leaf.join(" "),
        );
    }
    assert_eq!(
        EXPECTATIONS.len(),
        PRE_DISPATCH_FAULTS.len() * VERB_KINDS.len(),
        "one expectation per (fault, leaf) cell, no duplicates",
    );
    assert!(
        PRE_DISPATCH_FAULTS
            .iter()
            .any(|fault| fault.phase == Phase::BeforeDiscovery)
            && PRE_DISPATCH_FAULTS
                .iter()
                .any(|fault| fault.phase == Phase::AfterDiscovery)
            && PRE_DISPATCH_FAULTS
                .iter()
                .any(|fault| fault.phase == Phase::AfterPackLoad),
        "each phase is populated by a driven fault — a phase declared over an empty set \
         is a vocabulary, not an applicability relation",
    );
}

/// **The remainder is a count, not a sentence** — the `UNSWEPT_PRODUCERS` mold
/// (`crates/cli/tests/repo_relative_paths.rs`). Each [`Fault::sites`] row is measured
/// against the production source it names, so the arithmetic every fault row states —
/// *N sites raise it, M reach the seam, the rest are declared* — is checked rather than
/// written down. A count of `0` is the strongest row of all: it is how "nothing swallows
/// this fault any more" stops being a claim.
///
/// **Proven red by applied mutants**, because a fence nobody has seen fail is a hope:
/// a 24th reader of the process cwd added to `cli.rs` in the shape a new dispatch arm
/// would add one took the `std::env::current_dir()` row from 2 to 3 and reddened this
/// test; and deleting the `pack-resource-missing` row from [`PRE_DISPATCH_FAULTS`]
/// reddened [`every_cell_of_the_product_carries_exactly_one_expectation`] at its first
/// orphaned expectation.
///
/// **Its honest bound, stated rather than discovered:** this measures the rows it is
/// given. Deleting one of a fault's `sites` rows — the `warning: ` count of 0, say — is
/// invisible here, exactly as dropping an `UNSWEPT_PRODUCERS` row is invisible there. What
/// the deletion *cannot* hide is the behaviour: the swallowed fault would resurface as
/// stray bytes and redden [`every_reachable_cell_answers_its_fault_with_one_document`],
/// which drives the binary rather than reading the source.
#[test]
fn the_fault_site_counts_are_measured_against_the_source() {
    let mut offenders: Vec<String> = Vec::new();

    for fault in PRE_DISPATCH_FAULTS {
        assert!(
            !fault.sites.is_empty(),
            "`{}` carries no measured site — a fault nothing counts is a sentence",
            fault.id,
        );
        for (site, why) in fault.unanswered {
            assert!(
                !why.trim().is_empty(),
                "`{}`: the declared-unanswered site `{site}` carries no reason",
                fault.id,
            );
        }
        for (file, needle, count, reading) in fault.sites {
            let found = production_occurrences(file, needle, *reading);
            if found != *count {
                offenders.push(format!(
                    "  `{}` — {file} says {count} production occurrence(s) of `{needle}`, \
                     the source has {found}",
                    fault.id,
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "a bound nothing measures is a sentence — update the fault row (or the code it \
         describes):\n{}",
        offenders.join("\n"),
    );
}

/// Occurrences of `needle` in `file`'s **production** code, under the requested reading,
/// with `#[cfg(test)]` module bodies excluded — the same reader the standing source
/// fences use, so "counted here" and "counted there" mean the same thing.
fn production_occurrences(file: &str, needle: &str, reading: Reading) -> usize {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(file);
    let body = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {file}: {e}"));
    let code = match reading {
        Reading::Code => rust_source::code_only(&body),
        Reading::Literals => rust_source::code_and_strings(&body),
    };
    let regions = rust_source::cfg_test_regions(&rust_source::code_only(&body));
    code.match_indices(needle)
        .filter(|(at, _)| !rust_source::is_test_domain(&path, &regions, *at))
        .count()
}

// ---------------------------------------------------------------------------------
// Arm 1 — the product, driven
// ---------------------------------------------------------------------------------

/// The axis itself: every `(fault, leaf)` cell, through the **real binary**.
///
/// A cell the fault reaches must answer it — one JSON value of the declared shape on
/// stderr, stdout empty, **and nothing else on either stream**. A cell it does not reach
/// must be indistinguishable from the same argv over a sound corpus, which is driven as
/// a control rather than written down.
#[test]
fn every_reachable_cell_answers_its_fault_with_one_document() {
    // One sound corpus, copied per cell — the fault fixtures mutate, and a shared corpus
    // would make the sweep order-dependent.
    let base = TrialCorpus::build(State::Fresh);
    let empty_pack = unique_root("pre-dispatch-empty-pack");
    fs::create_dir_all(&empty_pack).expect("create the not-a-pack dir");

    for fault in PRE_DISPATCH_FAULTS {
        for (path, _) in VERB_KINDS {
            let tail = leaf_argv::MINIMAL_ARGV
                .iter()
                .find(|(known, _)| known == path)
                .map(|(_, tail)| *tail)
                .expect("the shared argv table covers every leaf");
            let argv: Vec<&str> = path.iter().copied().chain(tail.iter().copied()).collect();
            let label = format!("`{}` × `jigc {}`", fault.id, argv.join(" "));

            let out = match fault.fixture {
                Fixture::DeletedCwd => drive_from_a_deleted_cwd(&base.home(), &argv),
                Fixture::MalformedPacksYaml => {
                    let corpus = base.copy_state();
                    write_fixture_files(&corpus.repo());
                    corrupt_packs_yaml(&corpus);
                    drive(&corpus.repo(), &corpus.home(), None, &argv)
                }
                Fixture::EmptyPackDir => {
                    let corpus = base.copy_state();
                    write_fixture_files(&corpus.repo());
                    drive(&corpus.repo(), &corpus.home(), Some(&empty_pack), &argv)
                }
            };
            let seen = observe(&out);

            match expectation(fault.id, path) {
                Expect::Answers(arm, exit) => {
                    assert_eq!(
                        seen,
                        Observed {
                            exit: Some(exit),
                            stdout: Arm::Empty,
                            stderr: arm,
                        },
                        "{label}: a reached fault answers with ONE document on stderr and \
                         nothing else\n{}",
                        detail(&out),
                    );
                }
                Expect::Unreached => {
                    let control = base.copy_state();
                    write_fixture_files(&control.repo());
                    let reference = drive(&control.repo(), &control.home(), None, &argv);
                    assert_eq!(
                        seen,
                        observe(&reference),
                        "{label}: the cell is declared Unreached, so the fault must change \
                         nothing — it does\n--- with the fault ---\n{}\n--- control ---\n{}",
                        detail(&out),
                        detail(&reference),
                    );
                }
            }
        }
    }
}

/// Put a corrupt `packs.yaml` in the corpus **and commit it**, which is the state an
/// operator is actually in: the file is authored by hand, it is tracked, and the mistake
/// rides a commit. Leaving it uncommitted would test something else — `.jigc/config/packs.yaml`
/// is inside `jigc setup`'s own install footprint, so an *uncommitted* edit there makes
/// `setup` refuse with `setup.dirty-install-path` before it ever asks for a pack, and the
/// cell would then be measuring the dirty-install guard while claiming to measure this fault.
fn corrupt_packs_yaml(corpus: &TrialCorpus) {
    fs::write(
        corpus.repo().join(".jigc/config/packs.yaml"),
        "not: [valid\n",
    )
    .expect("corrupt packs.yaml");
    corpus.git(&["add", ".jigc/config/packs.yaml"]);
    corpus.git(&["commit", "-m", "corrupt the pack-set list"]);
}

/// The files the shared argv tails name, written into a corpus so a cell that got past
/// the fault fails on something else, loudly, rather than on a missing file.
fn write_fixture_files(repo: &Path) {
    for (name, body) in leaf_argv::FIXTURE_FILES {
        fs::write(repo.join(name), body).expect("write an argv-table fixture file");
    }
}

// ---------------------------------------------------------------------------------
// Arm 2 — the clap carve-out emits no bytes of jigc's own
// ---------------------------------------------------------------------------------

/// **The declared carve-out stays a carve-out** ([command-output-contract.md](../../../design/command-output-contract.md)
/// → The clap carve-out): `--help`/`--version` print to stdout at exit 0 and a usage error
/// to stderr at exit 2, both plain text, because the parse failed before any format value
/// existed. That is a statement about *clap's* bytes — and at rc.15 it was false in the
/// other direction: over a corrupt `packs.yaml` jigc emitted **two** `warning:` lines of
/// its own beside them, from a pre-parse `make_pack` neither argv ever dispatches.
#[test]
fn the_clap_carve_out_emits_no_bytes_of_jigcs_own() {
    let corpus = TrialCorpus::build(State::Fresh);
    corrupt_packs_yaml(&corpus);

    let version = drive(&corpus.repo(), &corpus.home(), None, &["--version"]);
    assert_eq!(version.status.code(), Some(0), "`--version` exits 0");
    assert_eq!(
        String::from_utf8_lossy(&version.stderr),
        "",
        "`--version` over a corrupt `packs.yaml` writes nothing to stderr — it dispatches \
         no verb, so no pre-dispatch surface may speak for it",
    );
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).lines().count(),
        1,
        "`--version` prints one line and only one:\n{}",
        detail(&version),
    );

    let rejected = drive(&corpus.repo(), &corpus.home(), None, &["bogusverb"]);
    assert_eq!(
        rejected.status.code(),
        Some(2),
        "a clap-rejected argv keeps its declared exit 2",
    );
    let stderr = String::from_utf8_lossy(&rejected.stderr);
    assert!(
        !stderr.contains("warning: "),
        "a clap-rejected argv emits clap's block and nothing of jigc's own; got:\n{stderr}",
    );
    assert!(
        String::from_utf8_lossy(&rejected.stdout).is_empty(),
        "a clap-rejected argv writes nothing to stdout:\n{}",
        detail(&rejected),
    );
}

// ---------------------------------------------------------------------------------
// Arm 3 — the located `packs.yaml` fault names the file the way every surface does
// ---------------------------------------------------------------------------------

/// **Law 1's printed-path rule reaches pack-load** (`design/surface-contract.md` → The
/// printed-path fence). Until M52 the two `packs.yaml` readers rendered
/// `path.display()` — a host-absolute `/private/var/folders/…/repo/.jigc/config/packs.yaml`
/// — and `repo_relative_paths.rs` disposed the module with the reason *"pack-load has no
/// repo-root subject to be relative to"*. Driven, that is false: `discover_project_config`
/// finds the repo root one call earlier and the config dir sits inside it.
#[test]
fn the_packs_yaml_fault_names_the_file_repo_relatively() {
    let corpus = TrialCorpus::build(State::Fresh);
    corrupt_packs_yaml(&corpus);

    let out = drive(&corpus.repo(), &corpus.home(), None, &["doc", "list"]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(".jigc/config/packs.yaml"),
        "the fault names the file it is about:\n{stderr}",
    );
    assert!(
        !stderr.contains(&corpus.repo().display().to_string()),
        "the fault names the file repo-relatively, never by its host path:\n{stderr}",
    );
}
