//! T2 — the CLI subprocess **invoker** driven against **real** adversarial stub
//! probe executables.
//!
//! The invoker is the only place a probe (a non-Rust program, by design) runs; the
//! engine stays **shell-free** ([finalize.md](../../../design/finalize.md) — the
//! engine never shells out; [module-layout.md](../../../implementation/module-layout.md)
//! → Probe boundary: the CLI owns the invoker). Its contract is narrow: write the T1
//! request to the child's stdin, enforce a **bounded wall-clock budget**
//! ([validation.md](../../../design/validation.md) → The wire contract, line 129),
//! and return a **RAW outcome** (stdout bytes + exit / timed-out status). It classifies
//! **nothing** — meta-finding synthesis is the engine's T3 job; the invoker only
//! reports what the process did.
//!
//! These tests build four real stub executables (compiled with `rustc` into a temp
//! dir — actual processes, not mocks) and drive the invoker against each adversarial
//! shape the determinism contract's three enforced meta-findings observe from the
//! boundary:
//!
//! - **well-behaved** — echoes a valid JSON response, exits 0 → the invoker returns
//!   its stdout bytes verbatim + a zero exit; well under budget.
//! - **sleeper** — sleeps past the budget → a **timed-out** outcome, the child
//!   **killed not leaked** (the test must not hang); a real elapsed-time bound.
//! - **crasher** — exits non-zero, emits no JSON → a non-zero-exit outcome.
//! - **garbage** — writes non-JSON to stdout, exits 0 → raw unparseable bytes + a
//!   zero exit (the invoker does NOT parse; malformed-output is T3's call).
//!
//! Since M54 Inc 2 T2 the outcome also carries the child's **stderr**, bounded at
//! [`PROBE_STDERR_BOUND`]: the crasher's stderr rides its outcome, and a **flooder**
//! writing 1 MiB of stderr neither hangs the invoker on a full pipe nor hands back more
//! than the bound.

use cli::invoke::{self, ProbeStatus};
use engine::probe::PROBE_STDERR_BOUND;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// A throwaway directory that removes itself on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "jigc-probe-invoker-{tag}-{}-{:?}",
            std::process::id(),
            engine::tempname::unique_nanos(),
        ));
        fs::create_dir_all(&path).expect("create temp dir");
        TempDir(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Compile `source` (a tiny Rust program) into an executable at `<dir>/<name>` and
/// return its path. A **real** stub probe — a separate process the invoker drives,
/// not an in-test mock.
fn build_stub(dir: &Path, name: &str, source: &str) -> PathBuf {
    let src = dir.join(format!("{name}.rs"));
    fs::write(&src, source).expect("write stub source");
    let bin = dir.join(name);
    let out = std::process::Command::new("rustc")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .arg("--edition")
        .arg("2021")
        .output()
        .expect("invoke rustc");
    assert!(
        out.status.success(),
        "rustc failed to build stub `{name}`:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    bin
}

/// A request payload the invoker writes to the child's stdin — the bytes are opaque to
/// the invoker (it never parses them); here a small JSON object the well-behaved stub
/// echoes back through.
fn sample_request() -> Vec<u8> {
    br#"{"probe_id":"doc-code","target":"adr:cache#status/cites-code","schema_version":2}"#.to_vec()
}

/// **Well-behaved**: the invoker returns the stub's stdout bytes verbatim and reports a
/// zero exit. It runs well under the budget, so no timeout fires.
#[test]
fn well_behaved_stub_returns_stdout_and_zero_exit() {
    let dir = TempDir::new("well-behaved");
    let stub = build_stub(
        dir.path(),
        "wellbehaved",
        r##"
fn main() {
    // Drain stdin (the invoker writes the request there) then emit a valid response.
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok();
    print!(r#"{{"findings":[],"schema_version":2}}"#);
}
"##,
    );

    let outcome = invoke::invoke_probe(&stub, &sample_request(), Duration::from_secs(10))
        .expect("invoker runs the well-behaved stub");

    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(0) },
        "a clean stub reports a zero exit",
    );
    assert_eq!(
        String::from_utf8_lossy(&outcome.stdout),
        r#"{"findings":[],"schema_version":2}"#,
        "the invoker returns the stub's stdout bytes verbatim",
    );
}

/// **Sleeper**: a stub that sleeps far past the budget. The invoker enforces a real
/// elapsed-time bound — it returns a `TimedOut` outcome and the test does **not** hang
/// (elapsed must be near the budget, nowhere near the stub's sleep), proving the child
/// is killed and not leaked.
#[test]
fn sleeper_stub_times_out_without_hanging() {
    let dir = TempDir::new("sleeper");
    let stub = build_stub(
        dir.path(),
        "sleeper",
        r##"
fn main() {
    // Sleep far past any reasonable budget; if the invoker leaked us the test hangs.
    std::thread::sleep(std::time::Duration::from_secs(30));
    print!(r#"{{"findings":[],"schema_version":2}}"#);
}
"##,
    );

    let budget = Duration::from_millis(500);
    let start = Instant::now();
    let outcome =
        invoke::invoke_probe(&stub, &sample_request(), budget).expect("invoker drives the sleeper");
    let elapsed = start.elapsed();

    assert_eq!(
        outcome.status,
        ProbeStatus::TimedOut,
        "a stub past the budget times out",
    );
    assert!(
        elapsed < Duration::from_secs(10),
        "the invoker returns at the budget, not the stub's 30s sleep (elapsed {elapsed:?}) \
         — the child is killed, not leaked",
    );
}

/// **Crasher**: a stub that exits non-zero and emits no JSON. The invoker reports the
/// non-zero exit code raw — it does NOT classify this as a crash meta-finding (T3's job).
#[test]
fn crasher_stub_reports_nonzero_exit() {
    let dir = TempDir::new("crasher");
    let stub = build_stub(
        dir.path(),
        "crasher",
        r#"
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok();
    eprintln!("probe blew up");
    std::process::exit(3);
}
"#,
    );

    let outcome = invoke::invoke_probe(&stub, &sample_request(), Duration::from_secs(10))
        .expect("invoker drives the crasher");

    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(3) },
        "the invoker reports the non-zero exit code raw",
    );
    assert!(
        outcome.stdout.is_empty(),
        "the crasher emitted no stdout: {:?}",
        String::from_utf8_lossy(&outcome.stdout),
    );
    assert_eq!(
        String::from_utf8_lossy(&outcome.stderr),
        "probe blew up\n",
        "the outcome carries what the crasher wrote on stderr (M54 Inc 2 T2)",
    );
}

/// **Flooder** (M54 Inc 2 T2): a stub that writes **1 MiB** to stderr before answering
/// cleanly. The invoker drains stderr on its own thread, so the child never blocks on a
/// full pipe — the run ends on its own, well inside the budget, with its stdout intact —
/// and the outcome keeps no more than [`PROBE_STDERR_BOUND`] bytes of it. Red before: the
/// piped-but-unread stderr filled, the child blocked on its write, and the run timed out.
#[test]
fn stderr_flooder_neither_hangs_nor_exceeds_the_bound() {
    let dir = TempDir::new("flooder");
    let stub = build_stub(
        dir.path(),
        "flooder",
        r##"
fn main() {
    use std::io::{Read, Write};
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok();
    let flood = vec![b'e'; 1024 * 1024];
    std::io::stderr().write_all(&flood).expect("write the flood");
    print!(r#"{{"findings":[],"schema_version":2}}"#);
}
"##,
    );

    let budget = Duration::from_secs(10);
    let start = Instant::now();
    let outcome =
        invoke::invoke_probe(&stub, &sample_request(), budget).expect("invoker drives the flooder");
    let elapsed = start.elapsed();

    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(0) },
        "a child flooding stderr must still run to its own exit, never block on a full pipe \
         (elapsed {elapsed:?})",
    );
    assert!(
        elapsed < budget,
        "the flood is drained, not waited out (elapsed {elapsed:?})",
    );
    assert_eq!(
        String::from_utf8_lossy(&outcome.stdout),
        r#"{"findings":[],"schema_version":2}"#,
        "the flooder's stdout is intact",
    );
    assert_eq!(
        outcome.stderr.len(),
        PROBE_STDERR_BOUND,
        "the outcome keeps exactly the bound of a 1 MiB flood, never more",
    );
}

/// **Garbage**: a stub that writes non-JSON to stdout and exits 0. The invoker returns
/// the raw unparseable bytes + a zero exit — it does NOT parse stdout, so malformed
/// output is invisible to it (the engine's T3 parse turns this into a meta-finding).
#[test]
fn garbage_stub_returns_raw_bytes_and_zero_exit() {
    let dir = TempDir::new("garbage");
    let stub = build_stub(
        dir.path(),
        "garbage",
        r#"
fn main() {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).ok();
    print!("this is not json at all <<>>");
}
"#,
    );

    let outcome = invoke::invoke_probe(&stub, &sample_request(), Duration::from_secs(10))
        .expect("invoker drives the garbage stub");

    assert_eq!(
        outcome.status,
        ProbeStatus::Exited { code: Some(0) },
        "the garbage stub exits 0 — the invoker reports that, not a classification",
    );
    assert_eq!(
        String::from_utf8_lossy(&outcome.stdout),
        "this is not json at all <<>>",
        "the invoker returns the raw unparseable bytes verbatim",
    );
}
