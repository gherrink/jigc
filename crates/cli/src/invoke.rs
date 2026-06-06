//! The subprocess **probe invoker** — the only place a probe (a non-Rust program, by
//! design) runs. The engine stays **shell-free** ([finalize.md](../../../design/finalize.md)
//! — the engine never shells out); the CLI owns the invoker
//! ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary).
//!
//! The invoker's contract is narrow and **classification-free**: it writes the engine's
//! request to the child's stdin, enforces a **bounded wall-clock budget**
//! ([validation.md](../../../design/validation.md) → The wire contract, line 129 — the
//! budget the CLI invoker enforces), and returns a **raw outcome** — the child's stdout
//! bytes plus an exit / timed-out status. It parses nothing and judges nothing: the
//! engine (T3) turns a timeout / a non-zero exit / unparseable output into the three
//! `pack-probe-integrity.*` meta-findings. Keeping the invoker dumb keeps the
//! determinism contract's enforcement points in one engine-owned place.
//!
//! **Read-only invocation** ([validation.md](../../../design/validation.md) → The wire
//! contract, line 101): the invoker hands the probe only its stdin request (a path-ref
//! to a read-only snapshot the engine materialized) and never grants it a callback —
//! the probe reads, it never writes back through the invoker.
//!
//! **The wait-timeout mechanism is std-only** (no crate): the child's stdout is drained
//! on a dedicated reader thread (so a chatty probe never deadlocks on a full pipe),
//! while the main thread keeps the [`Child`] handle and polls [`Child::try_wait`]
//! against a real elapsed-time budget. On expiry the child is **killed** through that
//! retained handle (never leaked) and the outcome is [`ProbeStatus::TimedOut`]. This
//! favours the minimal-deps reflex — `std::process::Command` is already used across the
//! CLI and the poll-against-`try_wait` wait needs no third-party `wait-timeout` crate
//! ([DECISIONS.md](../../../DECISIONS.md) 2026-06-06, M10 inc-3 / T2 elaboration pin).

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// How long the main thread sleeps between [`Child::try_wait`] polls — small enough that
/// the enforced budget is a tight bound, large enough not to spin.
const POLL_INTERVAL: Duration = Duration::from_millis(10);

/// The wall-clock budget the invoker enforces over the `doc-code` probe
/// ([validation.md](../../../design/validation.md) → The wire contract, line 129 — the
/// budget the CLI invoker enforces). A static parse of a task's anchors is sub-second;
/// this is the generous ceiling a runaway / hung probe trips into the `timeout`
/// meta-finding.
pub const DOC_CODE_BUDGET: Duration = Duration::from_secs(30);

/// The env var that overrides the `doc-code` probe program path — the production
/// override knob (the `JIGC_PACK_DIR` precedent; `DECISIONS.md` 2026-06-06, M10 inc-5 /
/// T2 elaboration pin). Set it to an absolute path to a `doc-code` executable.
const DOC_CODE_ENV: &str = "JIGC_DOC_CODE_PROBE";

/// The documented production default for the `doc-code` probe program, resolved relative
/// to the running `jigc` binary's directory (`<bin-dir>/doc-code`) — a probe shipped
/// alongside the CLI in the install tree. `JIGC_DOC_CODE_PROBE` overrides it (a dev /
/// test build, or a relocated install); when neither the env nor a sibling binary is
/// usable the invocation surfaces a `crash` meta-finding (a missing program is an
/// unresolvable invocation, never a silent pass).
fn doc_code_default() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join("doc-code")))
        .unwrap_or_else(|| PathBuf::from("doc-code"))
}

/// Resolve the `doc-code` probe program: the `JIGC_DOC_CODE_PROBE` override if set,
/// else the documented [`doc_code_default`] (a `doc-code` sibling of the `jigc` binary).
pub fn doc_code_program() -> PathBuf {
    match std::env::var_os(DOC_CODE_ENV) {
        Some(path) if !path.is_empty() => PathBuf::from(path),
        _ => doc_code_default(),
    }
}

/// How a probe subprocess ended, observed from the invoker boundary — **raw**, never
/// classified. The engine (T3) maps these onto the `pack-probe-integrity.*`
/// meta-findings; the invoker only reports what the process did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProbeStatus {
    /// The child exited on its own within the budget, carrying its exit code (`None`
    /// when the platform reports termination by signal with no code). A zero code is a
    /// clean run; a non-zero code is what the engine reads as a crash candidate.
    Exited {
        /// The process exit code, or `None` if terminated without one (e.g. a signal).
        code: Option<i32>,
    },
    /// The child exceeded the wall-clock budget and was **killed** by the invoker (not
    /// leaked) — the engine reads this as the `timeout` meta-finding.
    TimedOut,
}

/// The raw outcome of one probe invocation: the child's stdout bytes (verbatim,
/// unparsed) and how it ended. The invoker classifies nothing — `stdout` may be valid
/// JSON, unparseable garbage, or empty, and the engine decides what that means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeOutcome {
    /// The bytes the probe wrote to stdout, captured verbatim (empty if it wrote none).
    pub stdout: Vec<u8>,
    /// How the process ended.
    pub status: ProbeStatus,
}

/// Run the probe `program`, writing `request` to its stdin and enforcing `budget` as a
/// real elapsed-time bound. Returns the [`ProbeOutcome`] — stdout bytes + exit /
/// timed-out status — or an [`io::Error`] if the child could not be spawned.
///
/// On a budget overrun the child is **killed and reaped** (no leaked process, no hang),
/// and the outcome is [`ProbeStatus::TimedOut`] with empty stdout (the probe's partial
/// output is discarded — a timed-out probe is untrusted).
pub fn invoke_probe(program: &Path, request: &[u8], budget: Duration) -> io::Result<ProbeOutcome> {
    let mut child = Command::new(program)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Hand the probe its request, then close stdin so a probe that reads-to-EOF
    // proceeds. The snapshot rides by path-ref, not inline (validation.md:101), so the
    // request is small and a probe that exits before draining it (the crasher) only
    // yields a broken-pipe write error, which is expected — not an invoker failure.
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(request);
        drop(stdin);
    }

    // Drain stdout on a dedicated thread so a probe that writes more than a pipe buffer
    // can never deadlock against our wait. The `Child` handle stays here so we keep the
    // ability to `kill()` on a budget overrun.
    let stdout_pipe = child.stdout.take();
    let reader = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut pipe) = stdout_pipe {
            let _ = pipe.read_to_end(&mut buf);
        }
        buf
    });

    let deadline = Instant::now() + budget;
    loop {
        match child.try_wait()? {
            Some(status) => {
                let stdout = reader.join().unwrap_or_default();
                return Ok(ProbeOutcome {
                    stdout,
                    status: ProbeStatus::Exited {
                        code: status.code(),
                    },
                });
            }
            None => {
                if Instant::now() >= deadline {
                    return Ok(timed_out(child, reader));
                }
                thread::sleep(POLL_INTERVAL);
            }
        }
    }
}

/// Kill and reap a child that blew the budget, joining its stdout reader so no thread or
/// process is leaked, and report [`ProbeStatus::TimedOut`].
fn timed_out(mut child: Child, reader: thread::JoinHandle<Vec<u8>>) -> ProbeOutcome {
    // Kill, then `wait` to reap the zombie (a killed-but-unwaited child leaks). Both may
    // race a just-now-exited child; either way the process is gone after this.
    let _ = child.kill();
    let _ = child.wait();
    // The reader thread unblocks once the killed child's stdout pipe closes.
    let _ = reader.join();
    ProbeOutcome {
        stdout: Vec::new(),
        status: ProbeStatus::TimedOut,
    }
}
