//! M54 Increment 2 / T8 — **on Linux, a replaced `jigc` still spawns its own image as the
//! `doc-code` probe** ([module-layout.md](../../../implementation/module-layout.md) → Probe
//! boundary, the `/proc/self/exe` clause; [DECISIONS.md](../../../DECISIONS.md) → M54 Settle,
//! S4).
//!
//! `jigc` runs the probe by spawning itself. On Linux the program it spawns is
//! `/proc/self/exe`, which names the **running image's inode** — so a `jigc` whose file on
//! disk is replaced mid-run (an upgrade, a reinstall) still runs its own probe, never the file
//! now sitting at its path. Elsewhere the program is `current_exe()`, a *path*.
//!
//! **The replacement happens before the launch, so there is no race to win.** The suite copies
//! `jigc`, holds the copy open read-only, **unlinks** the copy's path and installs a different
//! executable (the *impostor*) there. It then launches the copy through its open descriptor,
//! `/proc/self/fd/<n>` — the kernel resolves that link to the unlinked inode, so from its first
//! instruction the running `jigc` is an image whose path already holds something else.
//!
//! - **The run** — `jigc validate` over the [`State::Vendored`] corpus with its anchored symbol
//!   renamed away, no override. The report carries the real `doc-code.symbol-exists` on the
//!   drifted anchor and no `pack-probe-integrity` finding: the probe that ran was the image.
//! - **The control** — the impostor spawned by path with the probe argv fails, printing its own
//!   marker. So a probe spawned by *path* would not have produced that report.
//!
//! The production arm spawning `current_exe()` instead reddens the run: `current_exe()` reads
//! `/proc/self/exe` as a link, which for an unlinked image is `<path> (deleted)` — a file that
//! does not exist — so the probe reports *could not start*.
//!
//! Every file the suite writes sits under the corpus's own home, so its drop removes them. The
//! copy and the impostor are written by `install(1)`, never by this process: a write descriptor
//! this multi-threaded test process held could be inherited by a sibling test's fork and make
//! the later `execve` fail with `ETXTBSY`.
#![cfg(target_os = "linux")]

use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus, VENDORED_CODE_FILE, VENDORED_CODE_SYMBOL};
use serde_json::Value;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::Path;
use std::process::{Command, Stdio};

/// The corpus's tracked code file with its anchored symbol renamed away.
const DRIFTED: &str = "export function renamed(s: string): string {\n  return s;\n}\n";

/// What the impostor prints on stderr, so the control can tell it ran.
const IMPOSTOR_MARKER: &str = "impostor: not the jigc that was launched";

/// The impostor's exit code.
const IMPOSTOR_EXIT: i32 = 42;

/// `install -m 0755 <src> <dest>`, in its own process, asserting it succeeded.
fn install_executable(src: &Path, dest: &Path) {
    let out = Command::new("install")
        .arg("-m")
        .arg("0755")
        .arg(src)
        .arg(dest)
        .output()
        .expect("spawn install(1)");
    assert!(
        out.status.success(),
        "install {src:?} -> {dest:?} failed:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
}

#[test]
fn a_replaced_jigc_still_spawns_its_own_image_as_the_probe() {
    let corpus = TrialCorpus::build(State::Vendored);
    fs::write(corpus.repo().join(VENDORED_CODE_FILE), DRIFTED).expect("drift the anchored symbol");
    let dir = corpus.home().join("self-image");
    fs::create_dir_all(&dir).expect("create the image dir");

    // The copy that will run, held open read-only.
    let copy = dir.join("jigc");
    install_executable(Path::new(env!("CARGO_BIN_EXE_jigc")), &copy);
    let image = File::open(&copy).expect("open the copy read-only");

    // Replace it on disk: unlink the path, then install the impostor there.
    let source = dir.join("impostor.sh");
    fs::write(
        &source,
        format!("#!/bin/sh\necho '{IMPOSTOR_MARKER}' >&2\nexit {IMPOSTOR_EXIT}\n"),
    )
    .expect("write the impostor source");
    fs::remove_file(&copy).expect("unlink the copy's path");
    install_executable(&source, &copy);

    // The control: the file at the path is the impostor, and as a probe it fails.
    let control = Command::new(&copy)
        .args(cli::invoke::doc_code_probe_args())
        .stdin(Stdio::null())
        .output()
        .expect("spawn the impostor by path");
    assert_eq!(
        control.status.code(),
        Some(IMPOSTOR_EXIT),
        "the impostor at {copy:?} runs and fails; stderr:\n{}",
        String::from_utf8_lossy(&control.stderr),
    );
    assert!(
        String::from_utf8_lossy(&control.stderr).contains(IMPOSTOR_MARKER)
            && control.stdout.is_empty(),
        "the path holds the impostor, not a probe; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&control.stdout),
        String::from_utf8_lossy(&control.stderr),
    );

    // The run: the unlinked image, launched through its open descriptor, no override.
    let launched = format!("/proc/self/fd/{}", image.as_raw_fd());
    let out = Command::new(&launched)
        .args(["validate", "--format", "json"])
        .current_dir(corpus.repo())
        .env("HOME", corpus.home())
        .env_remove("JIGC_PACK_DIR")
        .env_remove("JIGC_DOC_CODE_PROBE")
        .stdin(Stdio::null())
        .output()
        .expect("launch the unlinked image through its descriptor");
    drop(image);
    let envelope: Value = stdout_json(&out, &[0, 1], "`jigc validate --format json`");
    let findings = envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("the envelope carries a findings array:\n{envelope}"));

    let integrity: Vec<&Value> = findings
        .iter()
        .filter(|f| f["probe"] == "pack-probe-integrity")
        .collect();
    assert!(
        integrity.is_empty(),
        "the probe is the running image, so it runs — no probe-integrity finding; got \
         {integrity:?}",
    );
    let anchor = format!("{VENDORED_CODE_FILE}#{VENDORED_CODE_SYMBOL}");
    assert!(
        findings
            .iter()
            .any(|f| f["code"] == "doc-code.symbol-exists"
                && f["message"].as_str().is_some_and(|m| m.contains(&anchor))),
        "the real probe ran and reports the drifted `{anchor}`; got {findings:?}",
    );
}
