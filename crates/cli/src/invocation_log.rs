//! CLI-side invocation instrumentation (M36 — `design/measurement.md` → The in-repo
//! invocation log).
//!
//! Two pieces, both **CLI-side** so the engine stays clock-free and ships empty by
//! invariant:
//!
//! - [`Outcome`] — the readable `{code, finding_codes, error_code}` a dispatch handler
//!   returns instead of the opaque, write-only [`std::process::ExitCode`]. `main()` converts
//!   it to an `ExitCode` at the process edge; before that, the log wrapper reads the numeric
//!   exit code, the finding codes, and the error identity off it.
//! - [`log_invocation`] — the opt-in JSONL append. A `bool` cascade knob `invocation-log`
//!   (default **OFF**) gates it; when ON, one record `{timestamp, argv, exit_code,
//!   duration_ms, finding_codes, output_bytes, binary_version, error_code}` is appended to
//!   `.jigc/logs/invocations.jsonl` per run — by every verb but the teardown, which never
//!   **starts** the log and appends only to one that is already there ([`LogWrite`]).
//!   The knob is resolved from the project cascade **independent of argv** (the wrapper must
//!   log clap-rejected usage errors too), and the whole path **no-ops outside a jigc project
//!   layer**. Best-effort throughout — a logging failure never perturbs the invocation.
//!
//! This is a **friction/failure log**, not a drive-around detector: it records what jigc
//! *did* (args, exit, findings, duration), never what was done around it
//! (`design/measurement.md` → Honest capability boundary).

use engine::finding::Finding;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

/// The invocation-log knob key (a `bool` on the closed `knobs.yaml` surface, default OFF).
const KNOB_KEY: &str = "invocation-log";

/// The log's directory under a `.jigc/` root — a member of `crate::gitignore::ENTRIES`, so
/// the log is gitignored and no index ever has a copy of it.
const LOGS_DIR: &str = "logs";

/// The log's file name inside [`LOGS_DIR`].
const LOG_FILE: &str = "invocations.jsonl";

/// The readable result of a dispatch handler: the numeric process exit code plus the finding
/// codes the run surfaced. Threaded up to `main()` in place of [`ExitCode`] (which is
/// write-only — it cannot be read back at the log wrapper).
#[derive(Debug, Clone)]
pub struct Outcome {
    /// The process exit code (the 0/1/2/3/4 vocabulary — `design/measurement.md` → exit-code
    /// hygiene).
    pub code: u8,
    /// The machine-actionable codes of the findings this invocation surfaced (empty when the
    /// run raised none). Populated on the report-bearing verbs (validate / finalize).
    pub finding_codes: Vec<String>,
    /// The **route-exempt error identity** of an operational failure that carries no `Finding`
    /// — `None` on every other run (M42, `design/finalize.md` → "A failed finalize must be
    /// legible in the invocation log"). It exists so the log can name *why* a failed run
    /// failed: a hook-rejected `finalize` and a `finalize <absent-id>` are both exit-1 with no
    /// findings, and were byte-identical in the log. It is deliberately **not** a `Finding` —
    /// that would force a mandatory route under the M41 advisory-route floor and would wrap
    /// git's hook stderr, which `design/finalize.md`:78 pins as verbatim and unwrapped. Like
    /// `binary_version` (M40 A3) it is log-only: an additive record field, no envelope change.
    pub error_code: Option<&'static str>,
}

/// The error identity of the **`jigc task finalize`** door's commit-phase rejection — the
/// user's `pre-commit` / `commit-msg` hook (or git itself) refused the commit, so no commit
/// was made and the task survives intact. The axis's shipped identity since M42; kept at its
/// original spelling (a log identity a reader already knows) while its eight siblings below
/// name their own doors.
pub const ERROR_COMMIT_REJECTED: &str = "finalize.commit-rejected";

/// The **amend arm** of `jigc task finalize` (F-10) — its `git commit --amend` was rejected,
/// so the commit at `HEAD` still carries the message it had.
///
/// Its own identity rather than [`ERROR_COMMIT_REJECTED`]'s, on the `milestone finalize`
/// two-arm precedent: the two commit models are two commit constructions with two different
/// state-truth clauses (one leaves a task's work uncommitted, the other leaves an existing
/// commit unrewritten), and a log that spelled them the same could not tell a rejected
/// *addition* from a rejected *rewrite*.
pub const ERROR_AMEND_REJECTED: &str = "finalize.amend-rejected";

/// The **`jigc milestone finalize`** boundary's rejection under `finalize.fan-out.squash: true`
/// (the single combine commit).
pub const ERROR_MILESTONE_FINALIZE_REJECTED: &str = "milestone-finalize.commit-rejected";

/// The **`jigc milestone finalize`** boundary's rejection under `finalize.fan-out.squash: false`
/// (the per-sub-task commit chain + its aggregate, all built in a dedicated worktree).
pub const ERROR_MILESTONE_CHAIN_REJECTED: &str = "milestone-finalize.chain-commit-rejected";

/// The **`jigc rename`** door's rejection — the atomic identity-refactor self-commit.
pub const ERROR_RENAME_REJECTED: &str = "rename.commit-rejected";

/// The **`jigc migrate-corpus`** door's rejection — the pathspec-limited self-commit.
pub const ERROR_MIGRATE_CORPUS_REJECTED: &str = "migrate-corpus.commit-rejected";

/// The **`jigc milestone create`** door's rejection — the record-only commit opening the record.
pub const ERROR_MILESTONE_CREATE_REJECTED: &str = "milestone-create.commit-rejected";

/// The **`jigc milestone add-task`** door's rejection — the record-only append commit.
pub const ERROR_MILESTONE_ADD_TASK_REJECTED: &str = "milestone-add-task.commit-rejected";

/// The **`jigc milestone add-from-spec`** door's rejection — the record-only seed commit.
pub const ERROR_MILESTONE_ADD_FROM_SPEC_REJECTED: &str = "milestone-add-from-spec.commit-rejected";

/// The **`jigc milestone discard`** door's rejection — the record-only settle commit.
pub const ERROR_MILESTONE_DISCARD_REJECTED: &str = "milestone-discard.commit-rejected";

/// The **`jigc task discard`** door's rejection — the record-only settle commit a milestone
/// sub-task's discard lands before it removes the working area (M49 Inc 2 T3).
pub const ERROR_TASK_DISCARD_REJECTED: &str = "task-discard.commit-rejected";

/// The error identity of the **exit-4 migration review hold** (M43) — a migration finalize
/// without `--approve` rendered the fidelity diff and stopped, committing nothing. A coded
/// stop that named no *why* in the log before this: exit 4 with `error_code: null`.
pub const ERROR_REVIEW_PENDING: &str = "migrate.review-pending";

/// One member of the **committing-door axis** — a production door that runs a hook-capable
/// `git commit` on the user's behalf, paired with the route-exempt error identity it carries
/// into the invocation log when that commit is rejected.
pub struct CommittingDoor {
    /// The door as an operator names it — its `jigc …` verb with arguments elided, and the
    /// commit-model arm appended for the one verb that has two. The axis label the acceptance
    /// suite matches on.
    pub verb: &'static str,
    /// The [`ERROR_CODE_REGISTRY`] member this door logs on a commit-phase rejection.
    pub error_code: &'static str,
    /// The clause this door's own `--help` states its commit in (M52 Increment 10 / T2,
    /// D-2) — the third consumer of the axis, fenced through the real binary by
    /// `tests/help_truth.rs`.
    ///
    /// Four of the ten doors shipped a help that never said they commit, one of them
    /// (`jigc milestone create`) volunteering *"opening its **gitignored** area"*, which
    /// an adopter reads as *nothing is committed* while the door moves `HEAD`
    /// (`completions/artifacts/M52/baseline-surfaces.md` §2.8).
    ///
    /// **The field has two halves, and the split is deliberate.** For those four, the
    /// value is a `pub const` here and the door's long help *renders it* — one home, so a
    /// reword cannot drop the claim. For the five that already stated their commit in
    /// their own words, the row **quotes** that clause instead: rewriting five working
    /// help texts to render a const buys exactly the property this fence already gives,
    /// and would move surfaces nothing asked to move.
    pub commits: &'static str,
}

/// [`jigc task finalize`](COMMITTING_DOORS)'s **amend-arm** commit clause (F-10) — rendered
/// into the door's one long help by `crate::task::finalize_long_about`, beside the ordinary
/// arm's. One leaf, one `--help`, two clauses: the door runs whichever commit model the
/// task's marker selects, and a reader cannot see the marker.
pub const TASK_FINALIZE_AMEND_COMMITS: &str =
    "rewrites the commit at `HEAD` with `git commit --amend`, leaving its tree untouched";

/// [`jigc milestone create`](COMMITTING_DOORS)'s commit clause — rendered into its long
/// help by `crate::milestone::create_long_about`.
pub const MILESTONE_CREATE_COMMITS: &str =
    "lands the milestone's committed record in a record-only commit";

/// [`jigc milestone add-task`](COMMITTING_DOORS)'s commit clause — rendered into its long
/// help by `crate::milestone::add_task_long_about`.
pub const MILESTONE_ADD_TASK_COMMITS: &str = "appends the sub-task to the milestone's committed record and commits that record on \
     its own";

/// [`jigc milestone add-from-spec`](COMMITTING_DOORS)'s commit clause — rendered into its
/// long help by `crate::milestone::add_from_spec_long_about`. One commit per seeded
/// sub-task, which is what the plural says.
pub const MILESTONE_ADD_FROM_SPEC_COMMITS: &str =
    "commits the milestone's record once per seeded sub-task";

/// [`jigc task discard`](COMMITTING_DOORS)'s commit clause — rendered into its long help by
/// `crate::task::discard_long_about`. The commit is the **sub-task** cell only; an ordinary
/// task's discard is workbench-local, which the help states beside it.
pub const TASK_DISCARD_COMMITS: &str =
    "settles its milestone's committed record to `discarded` and commits it";

/// The **committing-door axis** — ONE code-side list with three consumers (M47 Inc 3 T7;
/// `DECISIONS.md` → 2026-07-26 M47 the Settle, Decision 6 + the cross-model review's
/// condition that the producer set derive from the same axis):
///
/// - [`registry_mirrors_the_declared_members`](self::tests::registry_mirrors_the_declared_members)
///   derives [`ERROR_CODE_REGISTRY`] from it, so a door added without a code is a red test;
/// - `tests/commit_rejected_axis.rs` iterates it through the real binary, driving each door
///   under a rejecting `pre-commit` hook and reading the logged `error_code` back — which is
///   what closes the **release hole**: [`Outcome::error`]'s membership check is a
///   `debug_assert!`, compiled out of the release build a trial actually runs;
/// - `tests/help_truth.rs` iterates it through each door's `--help`, asserting the emitted
///   bytes carry that row's [`commits`](CommittingDoor::commits) clause (M52 Inc 10 T2) — so
///   a door that moves `HEAD` cannot ship a help that never says so.
///
/// It is the **rejecting sibling** of `tests/hook_output_axis.rs`' non-blocking enumeration
/// (the same door set, the same one exclusion: `jigc setup`'s install commit passes
/// `--no-verify` by recorded design, so no hook runs and there is nothing to reject). Both
/// read off the one hook-capable commit seam, `task::git_commit_capture`.
///
/// **Why `milestone finalize` is two members and not one.** Its two commit models are two
/// distinct commit constructions with two distinct `Err` arms — the `squash: true` single
/// combine, and the `squash: false` per-sub-task chain — and the log's job is to say which
/// one refused. Both arms re-run through the identical argv, so the *route* is shared while
/// the *identity* is not. Declared bound: inside the chain arm a rejected per-sub-task commit
/// and a rejected chain aggregate reach one untyped `Err`, so the code names the **arm**, not
/// the individual commit.
pub const COMMITTING_DOORS: &[CommittingDoor] = &[
    CommittingDoor {
        verb: "jigc task finalize",
        error_code: ERROR_COMMIT_REJECTED,
        commits: "validate, render, stage, `git commit`, post-commit",
    },
    // The amend arm (F-10). Same leaf, same `--help`, its own commit construction and its own
    // identity — the `milestone finalize` two-arm precedent, applied to the second door that
    // grew a second commit model.
    CommittingDoor {
        verb: "jigc task finalize (amend)",
        error_code: ERROR_AMEND_REJECTED,
        commits: TASK_FINALIZE_AMEND_COMMITS,
    },
    CommittingDoor {
        verb: "jigc milestone finalize (squash: true)",
        error_code: ERROR_MILESTONE_FINALIZE_REJECTED,
        commits: "The milestone commit boundary",
    },
    CommittingDoor {
        verb: "jigc milestone finalize (squash: false)",
        error_code: ERROR_MILESTONE_CHAIN_REJECTED,
        commits: "The milestone commit boundary",
    },
    CommittingDoor {
        verb: "jigc rename",
        error_code: ERROR_RENAME_REJECTED,
        commits: "commits as one atomic transaction",
    },
    CommittingDoor {
        verb: "jigc migrate-corpus",
        error_code: ERROR_MIGRATE_CORPUS_REJECTED,
        commits: "lands its own migration in a pathspec-limited commit",
    },
    CommittingDoor {
        verb: "jigc milestone create",
        error_code: ERROR_MILESTONE_CREATE_REJECTED,
        commits: MILESTONE_CREATE_COMMITS,
    },
    CommittingDoor {
        verb: "jigc milestone add-task",
        error_code: ERROR_MILESTONE_ADD_TASK_REJECTED,
        commits: MILESTONE_ADD_TASK_COMMITS,
    },
    CommittingDoor {
        verb: "jigc milestone add-from-spec",
        error_code: ERROR_MILESTONE_ADD_FROM_SPEC_REJECTED,
        commits: MILESTONE_ADD_FROM_SPEC_COMMITS,
    },
    CommittingDoor {
        verb: "jigc milestone discard",
        error_code: ERROR_MILESTONE_DISCARD_REJECTED,
        commits: "in one record-only commit, then tear the workbench down",
    },
    CommittingDoor {
        verb: "jigc task discard",
        error_code: ERROR_TASK_DISCARD_REJECTED,
        commits: TASK_DISCARD_COMMITS,
    },
];

/// The **error-code registry** — the closed vocabulary of route-exempt error identities an
/// [`Outcome`] may carry into the log (`design/surface-contract.md` → The error-code
/// namespace, which mirrors this const member-for-member). It exists so the anti-collision
/// rule is a test, not prose: no member may collide with the finding-code inventory (the
/// unit test below), or a log reader could mistake an operational identity for a `Finding`
/// code. The naming constructors ([`Outcome::error`] / [`Outcome::coded_error`]) debug-assert
/// membership, so a new identity must join the registry — and thereby the collision test —
/// to ship.
///
/// **M47 revises the recorded "deliberately not a per-verb code mint" rationale rather than
/// silently overriding it** (`DECISIONS.md` → 2026-07-26 M47 the Settle, Decision 6). That
/// rationale — *errored verbs already write records; close the registry at the identities the
/// log genuinely could not distinguish without* — was sound while exactly one door framed its
/// rejection. Once the frame sweeps every committing door, reusing `finalize.commit-rejected`
/// on the non-finalize ones would put a **lying** code in the log, on the very surface
/// M42 built to stop the log lying (law 1). So the vocabulary is per-**door**, not per-verb:
/// it closes at [`COMMITTING_DOORS`] plus the one non-commit identity, and every member is
/// still an identity the log could not otherwise distinguish.
pub const ERROR_CODE_REGISTRY: &[&str] = &[
    ERROR_COMMIT_REJECTED,
    ERROR_AMEND_REJECTED,
    ERROR_MILESTONE_FINALIZE_REJECTED,
    ERROR_MILESTONE_CHAIN_REJECTED,
    ERROR_RENAME_REJECTED,
    ERROR_MIGRATE_CORPUS_REJECTED,
    ERROR_MILESTONE_CREATE_REJECTED,
    ERROR_MILESTONE_ADD_TASK_REJECTED,
    ERROR_MILESTONE_ADD_FROM_SPEC_REJECTED,
    ERROR_MILESTONE_DISCARD_REJECTED,
    ERROR_TASK_DISCARD_REJECTED,
    ERROR_REVIEW_PENDING,
];

impl Outcome {
    /// A clean run — exit 0, no findings.
    pub fn success() -> Self {
        Self {
            code: crate::task::EXIT_SUCCESS,
            finding_codes: Vec::new(),
            error_code: None,
        }
    }

    /// An operational failure — exit 1, no findings, **no error identity** (the unstructured
    /// `anyhow` bail: "no such task", a locator error, …).
    pub fn failure() -> Self {
        Self {
            code: crate::task::EXIT_ERROR,
            finding_codes: Vec::new(),
            error_code: None,
        }
    }

    /// An operational failure that **names itself** — exit 1, no findings, carrying the
    /// route-exempt [`error_code`](Outcome::error_code) the log records (e.g.
    /// [`ERROR_COMMIT_REJECTED`]).
    pub fn error(error_code: &'static str) -> Self {
        debug_assert!(
            ERROR_CODE_REGISTRY.contains(&error_code),
            "an Outcome error identity must be a registry member (join `ERROR_CODE_REGISTRY` \
             so the collision test covers it)",
        );
        Self {
            code: crate::task::EXIT_ERROR,
            finding_codes: Vec::new(),
            error_code: Some(error_code),
        }
    }

    /// An outcome with an explicit exit code and no findings (e.g. a clap usage error 2),
    /// carrying **no error identity**.
    pub fn code(code: u8) -> Self {
        Self {
            code,
            finding_codes: Vec::new(),
            error_code: None,
        }
    }

    /// An explicitly-coded stop that **names itself** — an explicit exit code, no findings,
    /// carrying a route-exempt [`error_code`](Outcome::error_code) (e.g. the exit-4 migration
    /// review hold, [`ERROR_REVIEW_PENDING`]). Distinct from [`Outcome::error`], which is
    /// exit-1-shaped.
    pub fn coded_error(code: u8, error_code: &'static str) -> Self {
        debug_assert!(
            ERROR_CODE_REGISTRY.contains(&error_code),
            "an Outcome error identity must be a registry member (join `ERROR_CODE_REGISTRY` \
             so the collision test covers it)",
        );
        Self {
            code,
            finding_codes: Vec::new(),
            error_code: Some(error_code),
        }
    }

    /// An outcome carrying the `code`s of the given findings — the report-bearing verbs
    /// (validate / finalize) record what they surfaced.
    pub fn with_findings(code: u8, findings: &[Finding]) -> Self {
        Self {
            code,
            finding_codes: findings.iter().map(|f| f.code.clone()).collect(),
            error_code: None,
        }
    }

    /// Convert to the process [`ExitCode`] at the `main()` edge.
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(self.code)
    }
}

/// Render an operational failure to **stderr** and return the [`Outcome`] it earns — the one
/// pairing of the render with the exit record, and the only production caller of
/// [`crate::render::operational_error`].
///
/// **Why the two halves may not be separated.** A refusal raised through
/// [`crate::render::finding_error`] travels as a `render::BlockedFinding` carrying the whole
/// [`Finding`], and M49 Increment 11 / T4 minted that carrier for one stated reason — *"so a
/// dispatch handler can log the identity it prints"*. The read-back was then installed at
/// **two hand-listed handlers** (`cli::run_rename`, `task discard`), so
/// `jigc rename 'research:../../x'` recorded `store.malformed-slug` while
/// `jigc doc show 'adr:../../x'` recorded `finding_codes: []` for the same code at the same
/// corpus: one code, logged at one door of its family and dropped at the ~29 others (M50
/// completion audit, Finding 2). *A guard that has to ask to be called is not a guard*
/// ([dev-workflow.md](../../../implementation/dev-workflow.md) → *a grep is not a fence*), so
/// the read-back moves to where membership is decided — the act of rendering an operational
/// error — and `crates/cli/tests/blocked_finding_log_axis.rs` fences the funnel against a
/// direct caller.
///
/// **The printed bytes do not move**, at any door: this emits exactly
/// `eprintln!("{}", render::operational_error(format, err))`, which for a carried finding is
/// already the house findings line (`render::BlockedFinding`'s `Display`). What changes is the
/// record: [`Outcome::with_findings`] instead of [`Outcome::failure`] — same exit code
/// ([`crate::task::EXIT_ERROR`] either way), a name instead of an anonymous exit 1.
///
/// **The envelope arm** (M51 Increment 6 / T1). Flattening puts the block in the *message*,
/// which costs a driver the envelope **key** — a price `design/command-output-contract.md` →
/// *the complement* declares and accepts for the families whose codes the contract lists under
/// no target form. It is **not** available to a code the contract lists under one: that listing
/// is a promise that `(code, target)` resolves, and a code inside a message is not a key. A
/// refusal raised through [`crate::render::envelope_finding_error`] therefore renders as the
/// `Reject::Findings` arm of [`ENVELOPE_ARMS`](crate::render::ENVELOPE_ARMS) — `findings` +
/// `schema_version` on **stderr**, stdout empty, exit [`crate::task::EXIT_ERROR`] — through the
/// same [`crate::render::validation`] render every other findings envelope takes, so the two
/// cannot drift. The flattened arm below is untouched, and stays the default.
///
/// Both arms record the same thing: the finding's `code` in `finding_codes`. Which arm a
/// refusal takes is settled at the **carrier** — the door's own declaration, or, since the
/// M52 completion audit, the standing obligation of a code the contract lists under a
/// declared target form (`crate::render::ENVELOPE_OWED_CODES`) — and is read here off the
/// carrier's `envelope` flag, never guessed here from the code — and a door that carried findings **beside** its refusal
/// ([`crate::render::envelope_finding_error_beside`]) records every one of them, since the
/// envelope render reads the whole set rather than the head alone.
pub fn operational_failure(format: crate::cli::Format, err: &anyhow::Error) -> Outcome {
    // Every finding the refusal's **one** document owes — the refusal itself, plus whatever
    // the door carried beside it (M52 Increment 5 / T8): a failed transaction's rollback
    // conflicts belong *in* the document, because the arm that carries it owns stderr and a
    // line printed beside it is a line a driver has to drop.
    if let Some(findings) = crate::render::envelope_projecting_findings(err) {
        // A no-delta cascade: this is a refusal, not an inventory sweep, so the M6 severity
        // post-pass is a no-op over it. A cascade that cannot be resolved at all is not worth
        // a second failure surface — the flattened arm below still carries the code, the locus
        // and the route in its message — so the fall-through is the degradation, not a panic.
        if let Ok(resolved) = crate::cascade_util::no_delta_resolved() {
            let report = engine::result::ValidationReport::new(findings, &resolved);
            eprint!("{}", crate::render::validation(format, &report));
            if format != crate::cli::Format::Json {
                eprintln!();
            }
            return Outcome::with_findings(crate::task::EXIT_ERROR, &report.findings);
        }
    }
    eprintln!("{}", crate::render::operational_error(format, err));
    match crate::render::blocked_finding(err) {
        Some(finding) => {
            Outcome::with_findings(crate::task::EXIT_ERROR, std::slice::from_ref(finding))
        }
        None => Outcome::failure(),
    }
}

/// What one invocation may do to the log file: **start** it, or only add to it.
///
/// The log is sole-copy data inside the workbench, so the door that removes the workbench —
/// `jigc uninstall` — **refuses** while the log is there (`design/measurement.md` → The
/// in-repo invocation log, item 7). A door that refuses over a file must not be the one
/// that puts it there: an operator who moved the log out as the refusal's route says, and
/// was then refused for another reason, used to find the log back — one record long, the
/// refusal's own — and blocking the next run (the rc.24 fix pass, left open by `(R9, F5)`).
/// So that one verb's record is [`AppendOnly`](LogWrite::AppendOnly), however the run ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogWrite {
    /// Create `logs/` and the log on demand — every verb but the teardown.
    Mint,
    /// Append to a log that is **already there**, and create nothing — neither the file nor
    /// its directory. No log, no record.
    AppendOnly,
}

impl LogWrite {
    /// The write an invocation that reached the leaf verb `leaf` owes — `["uninstall"]`,
    /// `["doc", "show"]`; `[]` for an argv that reached no verb.
    ///
    /// [`AppendOnly`](LogWrite::AppendOnly) for exactly one leaf: the door that removes the
    /// workbench the log lives in, read off the destroying-door axis's own row
    /// ([`crate::milestone::UNINSTALL_DOOR`]) rather than off a name retyped here — the
    /// same row that carries the code the log blocks under.
    ///
    /// **Per leaf, never per outcome.** The rule holds for a teardown that lands, one that
    /// refuses under any of its codes, and one clap answers before the door runs at all
    /// (`--help`, a usage error): whichever way it ends, reading or running the teardown
    /// between two attempts must not put back the file the next attempt refuses over.
    pub fn for_leaf<S: AsRef<str>>(leaf: &[S]) -> Self {
        let mut door = crate::milestone::UNINSTALL_DOOR.verb.split(' ');
        let is_teardown =
            door.next() == Some("jigc") && door.eq(leaf.iter().map(|word| word.as_ref()));
        if is_teardown {
            LogWrite::AppendOnly
        } else {
            LogWrite::Mint
        }
    }
}

/// Append one JSONL record for this invocation to `logs_dir` (the caller resolved it via
/// [`enabled_logs_dir`], so the knob is already ON). `output_bytes` is the true stdout+stderr
/// total the run emitted, measured by `main()`'s fd-level tee; `write` says whether this
/// invocation may start the log or only add to it ([`LogWrite::for_leaf`]). Best-effort —
/// any filesystem failure is swallowed so instrumentation never breaks a run.
pub fn log_invocation(
    logs_dir: &Path,
    duration: Duration,
    outcome: &Outcome,
    output_bytes: u128,
    write: LogWrite,
) {
    // argv without the (machine-specific, absolute) program path — the record captures the
    // verb + flags + intent text (`design/measurement.md` → Honesty note on `argv` content).
    //
    // `args_os` read lossily, never `args`: this wrapper runs on EVERY invocation, and
    // `std::env::args()` PANICS on an argument that is not valid UTF-8 — here after the
    // verb has already emitted, so the run's own output reached the reader and the
    // process then died at exit 101 with a Rust panic on top of it. The class is *every*
    // production read of the process argv (`main`'s takeover render is the other member,
    // fixed one increment earlier); it is fenced at the source by
    // `crates/cli/tests/clap_error_kind_axis.rs` →
    // `no_production_code_reads_the_process_argv_as_strings`, so a third reader cannot
    // arrive in the panicking form.
    let argv: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    let _ = append_record(
        logs_dir,
        write,
        &now_timestamp(),
        &argv,
        outcome.code,
        duration.as_millis(),
        &outcome.finding_codes,
        output_bytes,
        outcome.error_code,
    );
}

/// The log's path under a `.jigc/` root (`<jigc_home>/.jigc`) — the **one** spelling the
/// writer ([`log_invocation`]) and the teardown's subject (`crate::setup`'s workbench guard)
/// both read, so the door that removes `.jigc/` asks about the file this module writes
/// rather than about a name retyped beside it.
pub(crate) fn log_path(jigc_root: &Path) -> PathBuf {
    jigc_root.join(LOGS_DIR).join(LOG_FILE)
}

/// Resolve the log directory `<jigc_home>/.jigc/logs` **iff** the knob is ON — else `None`.
/// Returns `None` (no log) outside a git repo, outside a jigc project layer, or on any
/// resolution error: instrumentation is opt-in and best-effort, never a failure surface.
/// `main()` reads this once, independent of argv, to gate both the fd-tee install and the log.
pub fn enabled_logs_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let ctx = crate::locate::locate(&cwd).ok()?;
    // Outside a jigc project layer (no `.jigc/config/`) there is no log — the wrapper no-ops.
    let project_config = ctx.project_config?;
    // The one **deliberately non-blocking** pack-source consumer: `make_pack` fires the
    // pack-load freeze gate, and a drifted frozen schema must not turn instrumentation
    // into a failure surface — it silently disables the log here, while the verb itself
    // blocks loudly on its own `make_pack()?` a moment later.
    //
    // **`.ok()?` means silently, and until M52 it did not.** This call runs **before**
    // `Cli::try_parse()` in `main`, so it precedes the format's existence and precedes
    // dispatch entirely — and `make_pack` used to `eprintln!` a warning for a malformed
    // `packs.yaml` rather than returning it, which put two unparseable lines on stderr for
    // every run in such a repo, `jigc --version` included. The factory now propagates that
    // fault (M52 Increment 1 / T3), so this call is what its own sentence above says it is:
    // the consumer that disables the log and says nothing, leaving the surfacing to the
    // verb's `?`, where the format and the repo root are both in hand.
    let pack = crate::pack::make_pack().ok()?;
    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config).ok()?;
    if resolved.scalar(KNOB_KEY) != Some("true") {
        return None;
    }
    Some(ctx.jigc_home.join(".jigc").join(LOGS_DIR))
}

/// Whether the knob is ON for the install at `jigc_home` — [`enabled_logs_dir`]'s answer,
/// asked by a door that already holds its own resolved home (the teardown's log refusal,
/// whose route says *switch it off first* only while that is still to do). `false` wherever
/// [`enabled_logs_dir`] answers `None`, and for a home other than the one this process
/// stands in.
pub(crate) fn enabled_at(jigc_home: &Path) -> bool {
    enabled_logs_dir().is_some_and(|dir| dir == jigc_home.join(".jigc").join(LOGS_DIR))
}

/// The one JSONL record shape (`design/measurement.md` → record shape). `duration_ms` and
/// the ISO-8601 UTC `timestamp` are the CLI-side clock's; `finding_codes` is empty on a run
/// that surfaced none. The surface is **declared unversioned and additive-only** (M51): it
/// carries no format version integer, a key may join this struct but none may be renamed or
/// dropped, and `binary_version` is the per-record discriminator —
/// on an append log fed by successive binaries it maps each record to the format that wrote
/// it, which a file-level integer could not. The key set is fenced against the emitted bytes
/// by `tests::the_emitted_key_set_is_closed_by_the_destructured_fields`, never described. No
/// format is shared with the dogfood hook's JSONL schema; the two are governed independently
/// of each other.
#[derive(serde::Serialize)]
struct Record<'a> {
    timestamp: &'a str,
    argv: &'a [String],
    exit_code: u8,
    duration_ms: u128,
    finding_codes: &'a [String],
    /// The true total stdout+stderr bytes this invocation emitted (incl. clap `--help`/
    /// `--version`), counted by `main()`'s fd-level tee — the adoption trial's A7 mass-output
    /// signal (`design/measurement.md`:70).
    output_bytes: u128,
    /// The running binary's own `CARGO_PKG_VERSION` (M40 A3) — a log spanning an upgrade
    /// attributes each record to the binary that wrote it.
    binary_version: &'static str,
    /// The route-exempt error identity of a `Finding`-less operational failure — `null` on
    /// every other record (M42 T7). Without it a hook-rejected `finalize` and a
    /// `finalize <absent-id>` are indistinguishable in the log (both exit 1, both
    /// `finding_codes: []`).
    error_code: Option<&'static str>,
}

/// Append one JSONL line to `<logs_dir>/invocations.jsonl`, creating `logs_dir` on demand.
///
/// **`logs_dir` only — never the `.jigc/` above it** (the rc.24 fix pass, `(R9, F3)`). The
/// log lives *inside* the workbench and does not mint it: this used `create_dir_all`, so
/// the one run that removes `.jigc/` — a `jigc uninstall` that succeeded — then re-created
/// `.jigc/logs/` to record itself. The door printed *removed .jigc/* over a tree that was
/// back before the process exited, un-ignored (the `.gitignore` that covered it went with
/// the install), and its second run was not the no-op its help promises; since that fix
/// pass the log also **blocks** the teardown, so the residue would have had the second run
/// refuse over a file the first one wrote. A workbench that is gone therefore gets no
/// record: `create_dir` fails `NotFound`, and the caller swallows it like every other
/// logging failure.
///
/// **And under [`LogWrite::AppendOnly`], nothing at all is created** — not `logs/`, not the
/// file: the open carries no `create`, so the *is it there?* question and the append are one
/// system call rather than a check followed by a write, and a dangling link at the log's
/// path is not written through either. An absent log fails `NotFound` and is swallowed the
/// same way. That subsumes the case above for the teardown itself — the log went with the
/// tree — while `create_dir` keeps answering it for every other verb.
#[allow(clippy::too_many_arguments)]
fn append_record(
    logs_dir: &std::path::Path,
    write: LogWrite,
    timestamp: &str,
    argv: &[String],
    exit_code: u8,
    duration_ms: u128,
    finding_codes: &[String],
    output_bytes: u128,
    error_code: Option<&'static str>,
) -> std::io::Result<()> {
    use std::io::Write;
    let mint = match write {
        LogWrite::Mint => true,
        LogWrite::AppendOnly => false,
    };
    if mint {
        match std::fs::create_dir(logs_dir) {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(err) => return Err(err),
        }
    }
    let record = Record {
        timestamp,
        argv,
        exit_code,
        duration_ms,
        finding_codes,
        output_bytes,
        binary_version: env!("CARGO_PKG_VERSION"),
        error_code,
    };
    let mut line = serde_json::to_string(&record).map_err(std::io::Error::other)?;
    line.push('\n');
    let mut file = std::fs::OpenOptions::new()
        .create(mint)
        .append(true)
        .open(logs_dir.join(LOG_FILE))?;
    file.write_all(line.as_bytes())
}

/// The current UTC instant as an ISO-8601 `YYYY-MM-DDTHH:MM:SSZ` string — the CLI-side
/// full-timestamp clock (the engine stays clock-free). Derived from [`std::time::SystemTime`]
/// via Howard Hinnant's `civil_from_days` (no date-crate dependency), the same `std::time`
/// source the CLI reads for the `set: on-create` date; this variant carries the time-of-day
/// too, so it is a distinct helper rather than a reuse of the date-only one.
fn now_timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86_400) as i64;
    let tod = secs % 86_400;
    let (h, m, s) = (tod / 3_600, (tod % 3_600) / 60, tod % 60);
    let (y, mo, d) = civil_from_days(days);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Convert a count of days since the Unix epoch (1970-01-01) to a `(year, month, day)`
/// proleptic-Gregorian civil date — Howard Hinnant's branch-free `civil_from_days`, correct
/// for all civil dates.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// The anti-collision half of the error-code namespace (`design/surface-contract.md` →
    /// The error-code namespace): no [`ERROR_CODE_REGISTRY`] member collides with the
    /// finding-code inventory — the two vocabularies share one dotted shape and one log
    /// field's namespace neighbourhood, so a collision would let a log reader mistake an
    /// operational identity for a `Finding` code. A test, not a prose rule (the doc's own
    /// medicine — cheap at 2 members). The AMBUSH_CLASS_CODES membership-assert is the
    /// shape mold (`pack.rs`).
    #[test]
    fn no_registry_member_collides_with_the_finding_inventory() {
        let inventory: Vec<String> = engine::result::check_inventory_codes().collect();
        assert!(
            !inventory.is_empty(),
            "the projection must yield the real inventory (an empty set passes vacuously)",
        );
        for member in ERROR_CODE_REGISTRY {
            assert!(
                !inventory.iter().any(|code| code == member),
                "the error-code registry member `{member}` collides with the finding-code \
                 inventory — pick an identity no probe owns",
            );
        }
    }

    /// **The log never mints the workbench** (`(R9, F3)`): appending under a `.jigc/` that
    /// is not there writes nothing and creates nothing, so the run that removed the tree —
    /// a successful `jigc uninstall` — cannot bring it back by recording itself. The driven
    /// cell is `tests/uninstall_workbench_subject.rs`; this is the writer's own half.
    #[test]
    fn a_record_is_never_appended_into_a_workbench_that_is_gone() {
        let base = std::env::temp_dir().join(format!(
            "jigc-log-no-mint-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        std::fs::create_dir_all(&base).expect("create the stand-in repository root");
        let jigc_root = base.join(".jigc");
        let argv = vec!["uninstall".to_string()];
        let none: Vec<String> = Vec::new();

        let gone = append_record(
            &jigc_root.join(LOGS_DIR),
            LogWrite::Mint,
            "2026-10-04T09:00:00Z",
            &argv,
            0,
            7,
            &none,
            42,
            None,
        );
        assert!(gone.is_err(), "no workbench, so no record");
        assert!(
            !jigc_root.exists(),
            "appending must not re-create `.jigc/` to hold the log",
        );

        // The control: with the workbench there, `logs/` is created on demand as before.
        std::fs::create_dir(&jigc_root).expect("create the workbench");
        append_record(
            &jigc_root.join(LOGS_DIR),
            LogWrite::Mint,
            "2026-10-04T09:00:01Z",
            &argv,
            0,
            7,
            &none,
            42,
            None,
        )
        .expect("a standing workbench takes the record");
        assert!(
            log_path(&jigc_root).is_file(),
            "and it lands at the log's own path"
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    /// **An append-only write creates nothing, at any shape of *the log is not there*** —
    /// the writer's own half of the teardown's rule ([`LogWrite::AppendOnly`]); the driven
    /// cells are `tests/uninstall_workbench_subject.rs`. Three shapes, because *absent* is
    /// more than a missing file: the directory gone, the directory standing empty, and a
    /// dangling link at the log's own path (which a creating open would write through).
    /// The control is the fourth: a log that is there takes the record.
    #[test]
    fn an_append_only_write_creates_nothing_and_still_appends_to_a_log_that_is_there() {
        let base = std::env::temp_dir().join(format!(
            "jigc-log-append-only-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        let jigc_root = base.join(".jigc");
        std::fs::create_dir_all(&jigc_root).expect("create the stand-in workbench");
        let logs_dir = jigc_root.join(LOGS_DIR);
        let argv = vec!["uninstall".to_string()];
        let none: Vec<String> = Vec::new();
        let append = |write: LogWrite| {
            append_record(
                &logs_dir,
                write,
                "2026-10-04T09:00:00Z",
                &argv,
                1,
                7,
                &none,
                42,
                None,
            )
        };

        // The directory is gone.
        assert!(append(LogWrite::AppendOnly).is_err(), "no log, no record");
        assert!(
            !logs_dir.exists(),
            "an append-only write must not create `logs/`",
        );

        // The directory stands empty.
        std::fs::create_dir(&logs_dir).expect("create `logs/`");
        assert!(append(LogWrite::AppendOnly).is_err(), "no log, no record");
        assert!(
            std::fs::symlink_metadata(log_path(&jigc_root)).is_err(),
            "an append-only write must not create the log",
        );

        // A dangling link sits at the log's path.
        #[cfg(unix)]
        {
            let target = base.join("elsewhere.jsonl");
            std::os::unix::fs::symlink(&target, log_path(&jigc_root))
                .expect("plant a dangling link at the log's path");
            assert!(append(LogWrite::AppendOnly).is_err(), "no log, no record");
            assert!(
                !target.exists(),
                "an append-only write must not create a file through a dangling link",
            );
            std::fs::remove_file(log_path(&jigc_root)).expect("clear the link");
        }

        // The control: a minted log takes the append-only record after it.
        append(LogWrite::Mint).expect("a minting write starts the log");
        append(LogWrite::AppendOnly).expect("a log that is there takes the record");
        let body = std::fs::read_to_string(log_path(&jigc_root)).expect("read the log");
        assert_eq!(
            body.lines().count(),
            2,
            "both records are in the one log; got:\n{body}",
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    /// **Exactly one leaf never starts the log, and it is the teardown** — asked of every
    /// leaf verb in the tree (`BEHALF_DOORS` is total over it, fenced ⇔ against the clap
    /// tree by `cli_parse::every_leaf_verb_says_what_it_acts_on`), and of the leaf clap
    /// actually parses `jigc uninstall` into. A renamed verb, a second door that removes
    /// the workbench, or a [`crate::milestone::UNINSTALL_DOOR`] row respelled away from its
    /// leaf all redden here rather than silently turning the rule off.
    #[test]
    fn the_teardown_is_the_one_leaf_that_never_starts_the_log() {
        use clap::Parser;

        let parsed = crate::cli::Cli::try_parse_from(["jigc", "uninstall", "--force"])
            .expect("`jigc uninstall --force` parses");
        assert_eq!(
            LogWrite::for_leaf(parsed.command.leaf()),
            LogWrite::AppendOnly,
            "the leaf clap parses the teardown into must be the append-only one",
        );

        let append_only: Vec<&[&str]> = crate::cli::BEHALF_DOORS
            .iter()
            .map(|row| row.door)
            .filter(|door| LogWrite::for_leaf(door) == LogWrite::AppendOnly)
            .collect();
        assert_eq!(
            append_only,
            vec![parsed.command.leaf()],
            "every other verb mints the log as before",
        );

        // An argv that reached no verb (`jigc`, `jigc --help`, an unknown subcommand).
        let none: [&str; 0] = [];
        assert_eq!(LogWrite::for_leaf(&none), LogWrite::Mint);
    }

    /// The registry closes at exactly the **declared** members, and the declaration is
    /// [`COMMITTING_DOORS`] — the same axis `tests/commit_rejected_axis.rs` drives, never a
    /// hand-maintained second list (the cross-model review's condition on Decision 6). One
    /// list, two consumers: a door added without a code, or a code minted for no door, is a
    /// red test here rather than a silent log entry naming the wrong verb.
    ///
    /// The one non-door member is [`ERROR_REVIEW_PENDING`] — the exit-4 migration review
    /// hold, which is a *coded stop*, not a rejected commit.
    #[test]
    fn registry_mirrors_the_declared_members() {
        let mut declared: Vec<&str> = COMMITTING_DOORS.iter().map(|d| d.error_code).collect();
        declared.push(ERROR_REVIEW_PENDING);
        assert_eq!(
            ERROR_CODE_REGISTRY, declared,
            "the registry, the committing-door axis, and design/surface-contract.md mirror \
             each other member-for-member",
        );
        assert_eq!(
            declared.len(),
            12,
            "the declared vocabulary is the 11 committing doors + the review hold; a change \
             here revises design/surface-contract.md's mirror in the same commit",
        );

        let mut unique = declared.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(
            unique.len(),
            declared.len(),
            "each door names ITSELF — a shared code would put a lying verb in the log",
        );
    }

    /// **Every field of [`Record`], paired with the wire key it is emitted under and the value
    /// it carries** — one row per field, each built from the field itself, so a row cannot
    /// speak for the neighbour it was copy-pasted from.
    ///
    /// The fence is the **exhaustive destructure**: a field added to `Record` does not compile
    /// until it is bound here, and a row deleted leaves its binding unused — *denied*, not
    /// warned (the compiler's own `field: _` suggestion silences it, which is exactly the
    /// deliberate *"this field is not emitted"* someone would then have to write down).
    /// [`the_emitted_key_set_is_closed_by_the_destructured_fields`] closes the loop against the
    /// bytes [`append_record`] actually writes, which is the half a destructure alone cannot
    /// see: a `#[serde(rename)]`, a `#[serde(skip)]`, or a key emitted from anywhere but this
    /// struct all redden there.
    ///
    /// It lives in-crate because `Record` is private — the JSONL line is the surface, the type
    /// is not (M51 Increment 7 / T5; settle **D9**).
    fn record_key_dispositions(record: Record<'_>) -> Vec<(&'static str, serde_json::Value)> {
        fn key<T: serde::Serialize>(
            key: &'static str,
            field: T,
        ) -> (&'static str, serde_json::Value) {
            (
                key,
                serde_json::to_value(field).expect("a record field serializes"),
            )
        }
        let Record {
            timestamp,
            argv,
            exit_code,
            duration_ms,
            finding_codes,
            output_bytes,
            binary_version,
            error_code,
        } = record;
        vec![
            key("timestamp", timestamp),
            key("argv", argv),
            key("exit_code", exit_code),
            key("duration_ms", duration_ms),
            // Present on **every** record, `[]` on a run that raised none — not a
            // failure-only key, which is what `design/measurement.md` said until M51.
            key("finding_codes", finding_codes),
            key("output_bytes", output_bytes),
            // The per-record discriminator the unversioned-and-additive-only declaration
            // rests on: a log fed by successive binaries maps each record to the format that
            // wrote it, which one file-level version integer could not do.
            key("binary_version", binary_version),
            key("error_code", error_code),
        ]
    }

    /// The record as [`append_record`] builds it, so the declared table and the driven bytes
    /// are the same value twice rather than two hand-kept spellings.
    fn record_for<'a>(
        timestamp: &'a str,
        argv: &'a [String],
        exit_code: u8,
        duration_ms: u128,
        finding_codes: &'a [String],
        output_bytes: u128,
        error_code: Option<&'static str>,
    ) -> Record<'a> {
        Record {
            timestamp,
            argv,
            exit_code,
            duration_ms,
            finding_codes,
            output_bytes,
            binary_version: env!("CARGO_PKG_VERSION"),
            error_code,
        }
    }

    /// **The emitted key set equals the destructured fields — on a success record and on a
    /// failure record alike** (M51 Increment 7 / T5).
    ///
    /// The suite that drives this surface asserted presence and type on four keys and key-set
    /// equality nowhere, so a renamed, dropped or added key reached a log analysis rather than
    /// a test. The assertion runs over the **written JSONL line**, not a re-serialization in
    /// test code: the line is what an analysis reads back.
    ///
    /// Two records, because *present always* is the claim: the success record carries
    /// `finding_codes: []` and `error_code: null` and still emits all eight keys.
    #[test]
    fn the_emitted_key_set_is_closed_by_the_destructured_fields() {
        let dir = std::env::temp_dir().join(format!(
            "jigc-record-keys-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        let argv = vec!["jigc".to_string(), "describe".to_string()];
        let none: Vec<String> = Vec::new();
        let raised = vec!["schema-conformance.schema-version-current".to_string()];

        append_record(
            &dir,
            LogWrite::Mint,
            "2026-09-15T09:00:00Z",
            &argv,
            0,
            7,
            &none,
            42,
            None,
        )
        .expect("the success record is appended");
        append_record(
            &dir,
            LogWrite::Mint,
            "2026-09-15T09:00:01Z",
            &argv,
            1,
            9,
            &raised,
            0,
            Some(ERROR_COMMIT_REJECTED),
        )
        .expect("the failure record is appended");
        let body = std::fs::read_to_string(dir.join("invocations.jsonl"))
            .expect("the appended log is readable");
        let _ = std::fs::remove_dir_all(&dir);

        let lines: Vec<&str> = body.lines().filter(|l| !l.trim().is_empty()).collect();
        assert_eq!(lines.len(), 2, "one line per appended record; got {body}");

        let declared = [
            record_key_dispositions(record_for(
                "2026-09-15T09:00:00Z",
                &argv,
                0,
                7,
                &none,
                42,
                None,
            )),
            record_key_dispositions(record_for(
                "2026-09-15T09:00:01Z",
                &argv,
                1,
                9,
                &raised,
                0,
                Some(ERROR_COMMIT_REJECTED),
            )),
        ];

        for (line, declared) in lines.iter().zip(declared.iter()) {
            let emitted: serde_json::Value =
                serde_json::from_str(line).expect("each log line is valid JSON");
            let emitted = emitted
                .as_object()
                .expect("each record is emitted as a JSON object");
            assert_eq!(
                emitted.keys().map(String::as_str).collect::<BTreeSet<_>>(),
                declared
                    .iter()
                    .map(|(key, _)| *key)
                    .collect::<BTreeSet<_>>(),
                "the emitted key set and `Record`'s destructured fields disagree — the log is \
                 declared additive-only, so a key may join this set but none may be renamed \
                 or dropped (`design/measurement.md` → The in-repo invocation log)",
            );
            for (key, value) in declared {
                assert_eq!(
                    emitted.get(*key),
                    Some(value),
                    "`{key}` is emitted carrying another field's value",
                );
            }
        }
    }

    /// The prose homes that **enumerate** the record's key set, each with the marker opening
    /// its brace list — paths relative to this crate's manifest directory.
    const KEY_LIST_HOMES: &[(&str, &str)] = &[
        ("../../design/measurement.md", "Record shape: `{"),
        ("src/invocation_log.rs", "one record `{"),
    ];

    /// The comma-separated tokens of the brace list `marker` opens, with any `//!` doc-comment
    /// continuation stripped so a list wrapped across source lines reads as one.
    fn brace_list_keys(text: &str, marker: &str) -> Vec<String> {
        let rest = text
            .split_once(marker)
            .unwrap_or_else(|| panic!("the home no longer carries the marker `{marker}`"))
            .1;
        let list = rest
            .split_once('}')
            .expect("the brace list closes on the same page")
            .0;
        list.split(',')
            .map(|token| token.replace("//!", " ").trim().to_string())
            .filter(|token| !token.is_empty())
            .collect()
    }

    /// **Every prose home that enumerates the record shape states exactly the emitted keys**
    /// (M51 Increment 7 / T5) — the `doctype_map_versions` mold: read the code-side set, assert
    /// the doc's rows.
    ///
    /// `design/measurement.md` was wrong in two fields for five milestones — it named
    /// `duration` for `duration_ms` and qualified `finding_codes` as *"(on failure)"* when the
    /// key is present on every record — which is the prose-not-fence mechanism this wave is
    /// closing. A qualifier inside the list reddens here too: the token no longer equals a
    /// field name, so a fact about a key belongs beside the list, not inside it.
    #[test]
    fn every_prose_home_states_exactly_the_emitted_keys() {
        let fields: BTreeSet<String> =
            record_key_dispositions(record_for("2026-09-15T09:00:00Z", &[], 0, 0, &[], 0, None))
                .iter()
                .map(|(key, _)| (*key).to_string())
                .collect();

        for (home, marker) in KEY_LIST_HOMES {
            let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(home);
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("{home} is readable: {err}"));
            let stated: BTreeSet<String> = brace_list_keys(&text, marker).into_iter().collect();
            assert_eq!(
                stated, fields,
                "{home} enumerates a record shape the binary does not emit",
            );
        }
    }

    /// **The declaration itself stands, in one sentence** (M51 Increment 7 / T5; settle
    /// **D9** · §14): the log is *unversioned and additive-only*, with `binary_version` named
    /// as the per-record discriminator that carries what a file-level version integer would
    /// have. Before M51 that sentence called the surface *independently versioned* while no
    /// version field existed anywhere — a reader could only resolve it by driving the binary.
    #[test]
    fn the_design_doc_declares_the_log_unversioned_and_additive_only() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../design/measurement.md");
        let text = std::fs::read_to_string(&path).expect("design/measurement.md is readable");
        let sentence = text
            .lines()
            .find(|line| line.contains("unversioned"))
            .expect("the doc declares the invocation log unversioned");
        for token in ["additive-only", "binary_version"] {
            assert!(
                sentence.contains(token),
                "the declaration sentence must name `{token}` — the declaration rests on it",
            );
        }
    }
}
