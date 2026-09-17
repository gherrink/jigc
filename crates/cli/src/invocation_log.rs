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
//!   `.jigc/logs/invocations.jsonl` per run.
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
}

/// The **committing-door axis** — ONE code-side list with two consumers (M47 Inc 3 T7;
/// `DECISIONS.md` → 2026-07-26 M47 the Settle, Decision 6 + the cross-model review's
/// condition that the producer set derive from the same axis):
///
/// - [`registry_mirrors_the_declared_members`](self::tests::registry_mirrors_the_declared_members)
///   derives [`ERROR_CODE_REGISTRY`] from it, so a door added without a code is a red test;
/// - `tests/commit_rejected_axis.rs` iterates it through the real binary, driving each door
///   under a rejecting `pre-commit` hook and reading the logged `error_code` back — which is
///   what closes the **release hole**: [`Outcome::error`]'s membership check is a
///   `debug_assert!`, compiled out of the release build a trial actually runs.
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
    },
    CommittingDoor {
        verb: "jigc milestone finalize (squash: true)",
        error_code: ERROR_MILESTONE_FINALIZE_REJECTED,
    },
    CommittingDoor {
        verb: "jigc milestone finalize (squash: false)",
        error_code: ERROR_MILESTONE_CHAIN_REJECTED,
    },
    CommittingDoor {
        verb: "jigc rename",
        error_code: ERROR_RENAME_REJECTED,
    },
    CommittingDoor {
        verb: "jigc migrate-corpus",
        error_code: ERROR_MIGRATE_CORPUS_REJECTED,
    },
    CommittingDoor {
        verb: "jigc milestone create",
        error_code: ERROR_MILESTONE_CREATE_REJECTED,
    },
    CommittingDoor {
        verb: "jigc milestone add-task",
        error_code: ERROR_MILESTONE_ADD_TASK_REJECTED,
    },
    CommittingDoor {
        verb: "jigc milestone add-from-spec",
        error_code: ERROR_MILESTONE_ADD_FROM_SPEC_REJECTED,
    },
    CommittingDoor {
        verb: "jigc milestone discard",
        error_code: ERROR_MILESTONE_DISCARD_REJECTED,
    },
    CommittingDoor {
        verb: "jigc task discard",
        error_code: ERROR_TASK_DISCARD_REJECTED,
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
/// refusal takes is declared at the door that raises it (the carrier's `envelope` flag), never
/// guessed here from the code.
pub fn operational_failure(format: crate::cli::Format, err: &anyhow::Error) -> Outcome {
    if let Some(finding) = crate::render::envelope_projecting_finding(err) {
        // A no-delta cascade: this is a refusal, not an inventory sweep, so the M6 severity
        // post-pass is a no-op over it. A cascade that cannot be resolved at all is not worth
        // a second failure surface — the flattened arm below still carries the code, the locus
        // and the route in its message — so the fall-through is the degradation, not a panic.
        if let Ok(resolved) = crate::cascade_util::no_delta_resolved() {
            let report = engine::result::ValidationReport::new(vec![finding.clone()], &resolved);
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

/// Append one JSONL record for this invocation to `logs_dir` (the caller resolved it via
/// [`enabled_logs_dir`], so the knob is already ON). `output_bytes` is the true stdout+stderr
/// total the run emitted, measured by `main()`'s fd-level tee. Best-effort — any filesystem
/// failure is swallowed so instrumentation never breaks a run.
pub fn log_invocation(logs_dir: &Path, duration: Duration, outcome: &Outcome, output_bytes: u128) {
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
        &now_timestamp(),
        &argv,
        outcome.code,
        duration.as_millis(),
        &outcome.finding_codes,
        output_bytes,
        outcome.error_code,
    );
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
    Some(ctx.jigc_home.join(".jigc").join("logs"))
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
#[allow(clippy::too_many_arguments)]
fn append_record(
    logs_dir: &std::path::Path,
    timestamp: &str,
    argv: &[String],
    exit_code: u8,
    duration_ms: u128,
    finding_codes: &[String],
    output_bytes: u128,
    error_code: Option<&'static str>,
) -> std::io::Result<()> {
    use std::io::Write;
    std::fs::create_dir_all(logs_dir)?;
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
        .create(true)
        .append(true)
        .open(logs_dir.join("invocations.jsonl"))?;
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
            11,
            "the declared vocabulary is the 10 committing doors + the review hold; a change \
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

        append_record(&dir, "2026-09-15T09:00:00Z", &argv, 0, 7, &none, 42, None)
            .expect("the success record is appended");
        append_record(
            &dir,
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
