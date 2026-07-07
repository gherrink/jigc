//! CLI-side invocation instrumentation (M36 — `design/measurement.md` → The in-repo
//! invocation log).
//!
//! Two pieces, both **CLI-side** so the engine stays clock-free and ships empty by
//! invariant:
//!
//! - [`Outcome`] — the readable `{code, finding_codes}` a dispatch handler returns instead
//!   of the opaque, write-only [`std::process::ExitCode`]. `main()` converts it to an
//!   `ExitCode` at the process edge; before that, the log wrapper reads the numeric exit
//!   code and the finding codes off it.
//! - [`log_invocation`] — the opt-in JSONL append. A `bool` cascade knob `invocation-log`
//!   (default **OFF**) gates it; when ON, one record `{timestamp, argv, exit_code,
//!   duration_ms, finding_codes}` is appended to `.jigc/logs/invocations.jsonl` per run.
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
}

impl Outcome {
    /// A clean run — exit 0, no findings.
    pub fn success() -> Self {
        Self {
            code: 0,
            finding_codes: Vec::new(),
        }
    }

    /// An operational failure — exit 1, no findings.
    pub fn failure() -> Self {
        Self {
            code: 1,
            finding_codes: Vec::new(),
        }
    }

    /// An outcome with an explicit exit code and no findings (e.g. a clap usage error 2, or
    /// a migration review hold 4).
    pub fn code(code: u8) -> Self {
        Self {
            code,
            finding_codes: Vec::new(),
        }
    }

    /// An outcome carrying the `code`s of the given findings — the report-bearing verbs
    /// (validate / finalize) record what they surfaced.
    pub fn with_findings(code: u8, findings: &[Finding]) -> Self {
        Self {
            code,
            finding_codes: findings.iter().map(|f| f.code.clone()).collect(),
        }
    }

    /// Convert to the process [`ExitCode`] at the `main()` edge.
    pub fn exit_code(&self) -> ExitCode {
        ExitCode::from(self.code)
    }
}

/// Append one JSONL record for this invocation to `logs_dir` (the caller resolved it via
/// [`enabled_logs_dir`], so the knob is already ON). `output_bytes` is the true stdout+stderr
/// total the run emitted, measured by `main()`'s fd-level tee. Best-effort — any filesystem
/// failure is swallowed so instrumentation never breaks a run.
pub fn log_invocation(logs_dir: &Path, duration: Duration, outcome: &Outcome, output_bytes: u128) {
    // argv without the (machine-specific, absolute) program path — the record captures the
    // verb + flags + intent text (`design/measurement.md` → Honesty note on `argv` content).
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let _ = append_record(
        logs_dir,
        &now_timestamp(),
        &argv,
        outcome.code,
        duration.as_millis(),
        &outcome.finding_codes,
        output_bytes,
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
    let pack = crate::pack::make_pack();
    let resolved = crate::start::resolve_severity_cascade(pack.as_ref(), &project_config).ok()?;
    if resolved.scalar(KNOB_KEY) != Some("true") {
        return None;
    }
    Some(ctx.jigc_home.join(".jigc").join("logs"))
}

/// The one JSONL record shape (`design/measurement.md` → record shape). `duration_ms` and
/// the ISO-8601 UTC `timestamp` are the CLI-side clock's; `finding_codes` is empty on a run
/// that surfaced none. An independently-versioned surface — no format is shared with the
/// dogfood hook's JSONL schema.
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
