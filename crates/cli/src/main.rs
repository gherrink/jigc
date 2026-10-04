//! `jigc` — the binary entry point.
//!
//! The command tree, dispatch, renderers and adapter generation all live in the
//! `cli` **library** (`lib.rs`) so the test suites can enumerate them; this file
//! keeps only `fn main` and the fd-level output tee the invocation log needs. See
//! `implementation/module-layout.md` → The I/O boundary.

mod doc_code_probe;

use clap::Parser;
use cli::invocation_log::{self, LogWrite, Outcome};
use cli::invoke::{self, ProbeArgv};
use cli::{cli as cli_tree, route_fence};
use std::os::unix::io::RawFd;
use std::process::ExitCode;
use std::thread::JoinHandle;
use std::time::Instant;

/// Parse and dispatch, then log the invocation (opt-in) and convert to a process exit code.
///
/// The log wrapper **straddles `Cli::try_parse()`**: a clap-rejected usage error exits the
/// dispatch path before it ever runs, so the parse must be fallible and the wrapper must
/// capture that exit-2 outcome too (`design/measurement.md` → The in-repo invocation log,
/// mechanism note). Timing spans the whole run.
///
/// The `invocation-log` knob is resolved **once, up front, independent of argv** (via
/// [`invocation_log::enabled_logs_dir`]); when it is ON an [`OutputTee`] is installed over
/// fd 1/2 for the whole run so the record's `output_bytes` is the *true* stdout+stderr total
/// this invocation emitted — including the clap `--help`/`--version` arm below, which a
/// per-emitter tally would miss (`design/measurement.md`:70; DECISIONS 2026-07-06 fork F).
/// Install is gated on the knob because teeing repoints fd 1/2 at a pipe (`isatty` → false),
/// which suppresses clap's terminal colour; keeping that cost on the opted-in operator alone.
fn main() -> ExitCode {
    // The self-exec probe intercept (M54 S4) — the FIRST statement, ahead of the route
    // fence, the invocation-log gate and the output tee, so `jigc`'s own probe child is
    // never logged as an agent's call. It matches the literal argv, never through clap.
    if let Some(code) = probe_intercept() {
        return code;
    }
    // The M43 route fence: install the mechanical-route argv validator before anything can
    // construct a route, so every debug-build run (incl. the flow suites driving this binary)
    // carries the parse assert live. A release build stores it and never consults it.
    route_fence::install();
    let started = Instant::now();
    let logs_dir = invocation_log::enabled_logs_dir();
    let tee = logs_dir.as_ref().and_then(|_| OutputTee::install());

    // Beside the outcome, what this invocation may do to the log file: every verb starts
    // it on demand but the teardown, which only ever adds to one that is already there
    // (`invocation_log::LogWrite`). Read off the parsed command where there is one, and off
    // the node the argv reached where clap answered instead — the rule is per verb, however
    // the run ends.
    let (outcome, log_write) = match cli_tree::Cli::try_parse() {
        Ok(cli) => {
            let log_write = LogWrite::for_leaf(cli.command.leaf());
            (cli.dispatch(), log_write)
        }
        Err(err) => {
            // Reproduce clap's own exit convention: `--help`/`--version` print to stdout, exit 0
            // (`use_stderr()` is false); a genuine usage error prints to stderr and exits 2.
            // The codes read from the taxonomy table, not hand literals (T1).
            let code = if err.use_stderr() {
                cli::task::EXIT_USAGE
            } else {
                cli::task::EXIT_SUCCESS
            };
            // The unknown-subcommand block is jigc's own render (M48 Inc 6 T2): the
            // curated sibling tip lands *in* the tip slot instead of below clap's
            // contradicting did-you-mean, which used to steer a `task discard-write`
            // guesser at the verb that destroys the whole task — the correction printed
            // under the lie it exists to remove (`cli_tree::unknown_subcommand_block`;
            // DECISIONS.md 2026-08-13 the Settle, F8).
            //
            // The same takeover, for the same reason, covers an **unexpected argument**
            // typed at a leaf whose foreclosed form has a recorded answer: `jigc describe
            // adr` was a bare exit 2 while the command's own definition named what answers
            // it (M46 Inc 8 T5), and `jigc rename … --task <id>` drew clap's misdirecting
            // `-- --task` while naming `jigc doc rename` nowhere (M49 T5, S-4) —
            // `cli_tree::unexpected_argument_block`; law 2 — the designated recovery is
            // named by the surface that produces the state.
            //
            // What happens to every OTHER clap error kind is no longer stated here in
            // prose: `crates/cli/tests/clap_error_kind_axis.rs` disposes each kind in a
            // declared table and DRIVES the argv that reaches it, so the claim is checked
            // rather than asserted (M49 T5). The stream, the plain-text form and the exit
            // code are unchanged by either takeover.
            //
            // `args_os` rather than `args`: this arm runs exactly when clap rejected the
            // argv, which includes `ErrorKind::InvalidUtf8`, whose whole cause is an
            // argument that is not valid UTF-8 — and `std::env::args()` PANICS on one,
            // taking the render down with it (exit 101, a Rust panic, no usage). Read
            // lossily the render survives, and a lossy token matches no subcommand name,
            // which is the right answer for a token that is not one.
            let argv: Vec<String> = std::env::args_os()
                .map(|arg| arg.to_string_lossy().into_owned())
                .collect();
            let block = cli_tree::unknown_subcommand_block(&err, &argv)
                .or_else(|| cli_tree::unexpected_argument_block(&err, &argv));
            match block {
                Some(block) => eprint!("{block}"),
                None => {
                    let _ = err.print();
                }
            }
            (
                Outcome::code(code),
                LogWrite::for_leaf(&cli_tree::rejected_argv_node(&argv)),
            )
        }
    };

    // Tear the tee down AFTER all emission (incl. the clap arm's own) but BEFORE the
    // log write, so `output_bytes` is the true total (`finish` flushes the std streams first).
    let output_bytes = tee.map(OutputTee::finish).unwrap_or(0);

    if let Some(logs_dir) = logs_dir {
        invocation_log::log_invocation(
            &logs_dir,
            started.elapsed(),
            &outcome,
            output_bytes,
            log_write,
        );
    }
    outcome.exit_code()
}

/// Run the bundled `doc-code` probe when the argv is exactly this build's probe argv
/// ([`invoke::doc_code_probe_args`]); refuse any other `__probe` argv — a skewed
/// `--build` or a malformed one — with its reason on stderr and a non-zero exit, which
/// the invoker carries into the `pack-probe-integrity.probe-failure` finding. `None` for
/// every other argv: `main` goes on as `jigc`.
fn probe_intercept() -> Option<ExitCode> {
    let args: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    match invoke::probe_argv(&args) {
        ProbeArgv::NotProbe => None,
        ProbeArgv::Run => Some(doc_code_probe::run()),
        ProbeArgv::Refuse(reason) => {
            eprintln!("{reason}");
            Some(ExitCode::from(cli::task::EXIT_ERROR))
        }
    }
}

/// An fd-level counting tee over stdout (fd 1) and stderr (fd 2): each fd's real sink is saved
/// (`dup`), the fd is repointed (`dup2`) at a fresh pipe, and a drain thread forwards every
/// byte to the saved sink while counting it. [`Self::finish`] restores the fds, joins the
/// threads, and returns the total stdout+stderr bytes that flowed through — structurally
/// un-bypassable (every write to fd 1/2, from any emitter or child process, is counted).
struct OutputTee {
    streams: Vec<TeeStream>,
}

struct TeeStream {
    /// The teed fd (1 or 2), restored to `saved_fd` on teardown.
    target_fd: RawFd,
    /// A `dup` of the original sink — the tee forwards to it and restores it onto `target_fd`.
    saved_fd: RawFd,
    /// The drain thread; joins to the count of bytes forwarded for this stream.
    reader: JoinHandle<u64>,
}

impl OutputTee {
    /// Install a counting tee over fd 1 and fd 2. Best-effort: on any libc failure, whatever
    /// was already installed is torn down and `None` is returned, leaving the fds untouched.
    fn install() -> Option<Self> {
        let mut streams = Vec::with_capacity(2);
        for target_fd in [libc::STDOUT_FILENO, libc::STDERR_FILENO] {
            match TeeStream::install(target_fd) {
                Some(stream) => streams.push(stream),
                None => {
                    for stream in streams {
                        stream.finish();
                    }
                    return None;
                }
            }
        }
        Some(Self { streams })
    }

    /// Restore the fds, join the drain threads, and return the total bytes (stdout + stderr)
    /// that flowed through the tee.
    fn finish(self) -> u128 {
        use std::io::Write;
        // Flush the buffered std streams into the pipes before restoring, so a trailing partial
        // line (no `\n`) is counted here rather than escaping to the post-`main` exit flush,
        // which would land on the already-restored sink and go uncounted.
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
        self.streams
            .into_iter()
            .map(|stream| stream.finish() as u128)
            .sum()
    }
}

impl TeeStream {
    fn install(target_fd: RawFd) -> Option<Self> {
        // SAFETY: raw fd syscalls on jigc's Unix target; every return value is checked and any
        // fd opened on a failure path is closed before returning `None`.
        unsafe {
            let saved_fd = libc::dup(target_fd);
            if saved_fd < 0 {
                return None;
            }
            let mut fds = [0 as RawFd; 2];
            if libc::pipe(fds.as_mut_ptr()) != 0 {
                libc::close(saved_fd);
                return None;
            }
            let [read_fd, write_fd] = fds;
            if libc::dup2(write_fd, target_fd) < 0 {
                libc::close(saved_fd);
                libc::close(read_fd);
                libc::close(write_fd);
                return None;
            }
            // `target_fd` now holds the only reference to the pipe's write end; drop the
            // standalone one so restoring `target_fd` is the sole event that yields EOF.
            libc::close(write_fd);
            let reader = std::thread::spawn(move || drain(read_fd, saved_fd));
            Some(Self {
                target_fd,
                saved_fd,
                reader,
            })
        }
    }

    /// Restore the saved sink onto the target fd (dropping its pipe-write reference → the drain
    /// thread reads EOF), join the thread for the count, then close the saved fd. Returns the
    /// bytes forwarded for this stream.
    fn finish(self) -> u64 {
        // SAFETY: `saved_fd`/`target_fd` are live fds this stream owns until this call.
        unsafe {
            libc::dup2(self.saved_fd, self.target_fd);
        }
        let count = self.reader.join().unwrap_or(0);
        // SAFETY: the drain thread has exited, so nothing else references `saved_fd`.
        unsafe {
            libc::close(self.saved_fd);
        }
        count
    }
}

/// Forward every byte read from `read_fd` to `sink_fd`, returning the total forwarded. Owns
/// `read_fd` (closed on return); `sink_fd` is owned by the caller. Best-effort: a read/write
/// error stops the loop with the count honest to what was forwarded.
fn drain(read_fd: RawFd, sink_fd: RawFd) -> u64 {
    let mut buf = [0u8; 8192];
    let mut total: u64 = 0;
    loop {
        // SAFETY: reading into a valid local buffer from an fd this thread owns.
        let n = unsafe { libc::read(read_fd, buf.as_mut_ptr().cast(), buf.len()) };
        if n < 0 {
            // EINTR: retry; any other error: stop draining (best-effort).
            if errno_is_eintr() {
                continue;
            }
            break;
        }
        if n == 0 {
            break; // EOF — the last write end closed.
        }
        let n = n as usize;
        total += n as u64;
        let mut off = 0;
        while off < n {
            // SAFETY: writing `n - off` bytes from within the buffer to a live sink fd.
            let w = unsafe { libc::write(sink_fd, buf.as_ptr().add(off).cast(), n - off) };
            if w < 0 {
                if errno_is_eintr() {
                    continue;
                }
                break; // sink error — stop forwarding, keep the count of what we read.
            }
            off += w as usize;
        }
    }
    // SAFETY: this thread owns `read_fd` and is done with it.
    unsafe {
        libc::close(read_fd);
    }
    total
}

/// Whether the last OS error was `EINTR` (an interrupted syscall, retryable) — read
/// portably across Unix via std rather than a platform-specific errno location.
fn errno_is_eintr() -> bool {
    std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR)
}
