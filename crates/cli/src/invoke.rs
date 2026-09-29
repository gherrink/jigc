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
//! **The wait-timeout mechanism is std-only** (no crate): the child's stdout and stderr
//! are each drained on a dedicated reader thread (so a chatty probe never deadlocks on a
//! full pipe; stderr is kept only up to its bound),
//! while the main thread keeps the [`Child`] handle and polls [`Child::try_wait`]
//! against a real elapsed-time budget. On expiry the child is **killed** through that
//! retained handle (never leaked) and the outcome is [`ProbeStatus::TimedOut`]. This
//! favours the minimal-deps reflex — `std::process::Command` is already used across the
//! CLI and the poll-against-`try_wait` wait needs no third-party `wait-timeout` crate
//! ([DECISIONS.md](../../../DECISIONS.md) 2026-06-06, M10 inc-3 / T2 elaboration pin).

use engine::probe::PROBE_STDERR_BOUND;
use std::ffi::{OsStr, OsString};
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
/// T2 elaboration pin). Set it to an absolute path to a **standalone** probe executable,
/// which is spawned with no arguments (the crash and timeout stubs).
const DOC_CODE_ENV: &str = "JIGC_DOC_CODE_PROBE";

/// The argv word that turns `jigc` into its own `doc-code` probe. `main` matches it on
/// the literal argv as its first statement — never through clap, where even a hidden
/// subcommand would trip the surface fences and surface in did-you-mean
/// ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary, the
/// bundled probe; `DECISIONS.md` → M54 Settle S4).
pub const PROBE_ARGV_HEAD: &str = "__probe";

/// The build id a probe child must match: `jigc`'s own version (M54 S4).
const BUILD_ID: &str = env!("CARGO_PKG_VERSION");

/// The **one** spelling of the arguments `jigc` passes itself to run as the `doc-code`
/// probe: `__probe doc-code --build <CARGO_PKG_VERSION>`. The production arm of
/// [`doc_code_command`] uses it, `main`'s intercept ([`probe_argv`]) matches it, and an
/// in-process test pairs it with `env!("CARGO_BIN_EXE_jigc")` — so the command has one
/// owner and no second spelling (a shell-wrapper shim that re-adds the args is forbidden).
pub fn doc_code_probe_args() -> Vec<String> {
    [PROBE_ARGV_HEAD, "doc-code", "--build", BUILD_ID]
        .map(str::to_owned)
        .to_vec()
}

/// The `JIGC_DOC_CODE_PROBE` override, when set to a non-empty path. The pre-flights
/// check only this arm: the production arm is the running `jigc` itself, which always
/// carries its probe.
pub fn doc_code_override() -> Option<PathBuf> {
    match std::env::var_os(DOC_CODE_ENV) {
        Some(path) if !path.is_empty() => Some(PathBuf::from(path)),
        _ => None,
    }
}

/// The probe **pre-flight** both scopes run before a sweep reaches the probe
/// ([validation.md](../../../design/validation.md) → Distribution bound): a set
/// `JIGC_DOC_CODE_PROBE` that names no file is one misconfiguration, reported once as an
/// operational error rather than as a `crash` meta-finding per anchor. Without the
/// override there is nothing to check — the probe is the running `jigc` (M54 S4).
pub fn require_doc_code_override() -> anyhow::Result<()> {
    match doc_code_override() {
        Some(program) if !program.is_file() => anyhow::bail!(
            "`doc-code` probe not found at {program:?} — `{DOC_CODE_ENV}` names no file; point it at a probe executable, or unset it to run the probe built into `jigc`"
        ),
        _ => Ok(()),
    }
}

/// How the `doc-code` probe is run — `(program, args)`. The override arm is
/// `(JIGC_DOC_CODE_PROBE, [])`; the production arm is `jigc` spawning itself,
/// `(<own image>, `[`doc_code_probe_args`]`())`, where the own image is `/proc/self/exe`
/// on Linux — which names the running image's inode even after the file on disk is
/// replaced — and `current_exe()` elsewhere. An `Err` is the running image's path being
/// unreadable, which the invoker reports as the probe's *could not start*.
pub fn doc_code_command() -> io::Result<(PathBuf, Vec<String>)> {
    match doc_code_override() {
        Some(program) => Ok((program, Vec::new())),
        None => Ok((own_image()?, doc_code_probe_args())),
    }
}

/// The running `jigc`'s own image, as the program the probe child is spawned from.
#[cfg(target_os = "linux")]
fn own_image() -> io::Result<PathBuf> {
    Ok(PathBuf::from("/proc/self/exe"))
}

/// The running `jigc`'s own image, as the program the probe child is spawned from.
#[cfg(not(target_os = "linux"))]
fn own_image() -> io::Result<PathBuf> {
    std::env::current_exe()
}

/// What `main`'s first-statement intercept makes of an argv (the arguments after the
/// program name).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProbeArgv {
    /// Not a probe invocation — `main` goes on to the route fence, the log and clap.
    NotProbe,
    /// Exactly [`doc_code_probe_args`] — run the bundled `doc-code` probe.
    Run,
    /// A `__probe` argv that is not this build's — a skewed `--build`, or malformed. The
    /// reason names both versions where there are two; `main` writes it on stderr and
    /// exits non-zero, so the invoker carries it into the finding's message.
    Refuse(String),
}

/// Classify `args` (the argv after the program name) for `main`'s probe intercept.
pub fn probe_argv(args: &[OsString]) -> ProbeArgv {
    if args.first().map(OsString::as_os_str) != Some(OsStr::new(PROBE_ARGV_HEAD)) {
        return ProbeArgv::NotProbe;
    }
    let expected = doc_code_probe_args();
    if args
        .iter()
        .map(OsString::as_os_str)
        .eq(expected.iter().map(OsStr::new))
    {
        return ProbeArgv::Run;
    }
    let got: Vec<String> = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();
    match got.as_slice() {
        [_, probe, flag, build] if probe == "doc-code" && flag == "--build" => {
            ProbeArgv::Refuse(format!(
                "jigc {BUILD_ID}: the `doc-code` probe was asked for build `{build}`, but this \
                 jigc is build `{BUILD_ID}` — the jigc binary changed while the run was in \
                 flight; rerun the command"
            ))
        }
        _ => ProbeArgv::Refuse(format!(
            "jigc {BUILD_ID}: `{PROBE_ARGV_HEAD}` takes exactly `doc-code --build {BUILD_ID}`, \
             got `{}`",
            got.join(" ")
        )),
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
/// unparsed), its bounded stderr, and how it ended. The invoker classifies nothing —
/// `stdout` may be valid JSON, unparseable garbage, or empty, and the engine decides what
/// that means.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeOutcome {
    /// The bytes the probe wrote to stdout, captured verbatim (empty if it wrote none).
    pub stdout: Vec<u8>,
    /// The first [`PROBE_STDERR_BOUND`] bytes the probe wrote to stderr (empty if it wrote
    /// none, or timed out); the rest is drained and dropped.
    pub stderr: Vec<u8>,
    /// How the process ended.
    pub status: ProbeStatus,
}

/// Run the probe `program` with `args`, writing `request` to its stdin and enforcing
/// `budget` as a real elapsed-time bound. Returns the [`ProbeOutcome`] — stdout bytes,
/// the first [`PROBE_STDERR_BOUND`] bytes of stderr, and the exit / timed-out status —
/// or an [`io::Error`] if the child could not be spawned.
///
/// On a budget overrun the child is **killed and reaped** (no leaked process, no hang),
/// and the outcome is [`ProbeStatus::TimedOut`] with empty stdout and stderr (the
/// probe's partial output is discarded — a timed-out probe is untrusted).
pub fn invoke_probe(
    program: &Path,
    args: &[String],
    request: &[u8],
    budget: Duration,
) -> io::Result<ProbeOutcome> {
    let mut child = Command::new(program)
        .args(args)
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

    // Drain stdout and stderr on dedicated threads so a probe that writes more than a
    // pipe buffer to either can never deadlock against our wait. stderr keeps only its
    // first `PROBE_STDERR_BOUND` bytes and drains the rest (M54 Inc 2 T2). The `Child`
    // handle stays here so we keep the ability to `kill()` on a budget overrun.
    let stdout = drain(child.stdout.take(), None);
    let stderr = drain(child.stderr.take(), Some(PROBE_STDERR_BOUND));

    let deadline = Instant::now() + budget;
    loop {
        match child.try_wait()? {
            Some(status) => {
                return Ok(ProbeOutcome {
                    stdout: stdout.join().unwrap_or_default(),
                    stderr: stderr.join().unwrap_or_default(),
                    status: ProbeStatus::Exited {
                        code: status.code(),
                    },
                });
            }
            None => {
                if Instant::now() >= deadline {
                    return Ok(timed_out(child, [stdout, stderr]));
                }
                thread::sleep(POLL_INTERVAL);
            }
        }
    }
}

/// Read `pipe` to its end on a dedicated thread, keeping at most `keep` bytes (all of
/// them when `None`) and discarding the rest — the rest is still **read**, so the child
/// never blocks writing to a full pipe.
fn drain<R: Read + Send + 'static>(
    pipe: Option<R>,
    keep: Option<usize>,
) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut pipe) = pipe {
            match keep {
                None => {
                    let _ = pipe.read_to_end(&mut buf);
                }
                Some(bound) => {
                    let _ = pipe.by_ref().take(bound as u64).read_to_end(&mut buf);
                    let _ = io::copy(&mut pipe, &mut io::sink());
                }
            }
        }
        buf
    })
}

/// Kill and reap a child that blew the budget, joining its stdout and stderr readers so
/// no thread or process is leaked, and report [`ProbeStatus::TimedOut`].
fn timed_out(mut child: Child, readers: [thread::JoinHandle<Vec<u8>>; 2]) -> ProbeOutcome {
    // Kill, then `wait` to reap the zombie (a killed-but-unwaited child leaks). Both may
    // race a just-now-exited child; either way the process is gone after this.
    let _ = child.kill();
    let _ = child.wait();
    // The reader threads unblock once the killed child's pipes close.
    for reader in readers {
        let _ = reader.join();
    }
    ProbeOutcome {
        stdout: Vec::new(),
        stderr: Vec::new(),
        status: ProbeStatus::TimedOut,
    }
}
