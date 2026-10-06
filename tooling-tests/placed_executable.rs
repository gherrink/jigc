//! **A file a tooling suite will execute is placed by a child process** — [`write`] and
//! [`copy`], the one way a suite of this directory puts an executable on disk, and the
//! fence that holds every suite to it ([dev-workflow.md](../implementation/dev-workflow.md)
//! → Gate, *A green gate on macOS never ran a file that was still open for writing*).
//!
//! **The failure it exists for.** Linux refuses to execute a file that anything has open
//! for writing: `execve` fails with `ETXTBSY`, "Text file busy". A write handle this
//! process opens is not only this process's — a child forked by **any** thread while it is
//! open inherits a copy, and keeps it until its own `exec`, which is when close-on-exec
//! closes it. The suites fan their arms out over threads, and `cargo test` runs a whole
//! group as the threads of one process. So a rig that copied a script with `fs::copy` and
//! ran it a moment later raced every other thread's `Command::spawn`: the handle was
//! closed here, a sibling's child that had not reached its `exec` still held it, and the
//! script was busy. macOS makes no such check, so every local gate was green while the
//! hosted runner failed two or three **different** tests of `dev_stabilize_record` on
//! every push.
//!
//! **Why a child.** No ordering inside this process closes that window — the fork is
//! another thread's — and a retry around the `exec` only waits for it. So the write
//! happens in a process of its own: `cp` opens the destination, writes it, closes it and
//! exits, and this process waits for it. No thread here ever has the handle, so no fork
//! can inherit one. Content that is not a file yet is staged in one first; a leaked handle
//! on the staging file harms nothing, because nothing executes it.
//!
//! **What holds it.**
//!
//! - [`a_write_handle_a_forked_sibling_holds_makes_the_file_busy_where_linux_checks`] is
//!   the control: the mechanism itself, made deterministic — a child held between its
//!   fork and its exec while a handle is open — and the refusal read on Linux. It is what
//!   says the next arm looks at the right thing.
//! - [`what_is_placed_runs_at_once_while_siblings_are_held_before_their_exec`] drives both
//!   helpers with such children forked **during** the placement and still held when the
//!   file is run. The placement is black-box, so *during* is a matter of timing rather
//!   than of proof: the payload is megabytes, so that an in-process write is open for
//!   milliseconds while the forks land in microseconds. Against helpers that write in
//!   this process the arm is red on Linux; it cannot be red on macOS, whatever the
//!   helpers do, and `dev/runner-faithful` is where it is read.
//! - [`no_tooling_suite_makes_a_file_executable_or_copies_one_but_through_the_helper`]
//!   reads every suite of this directory: no executable bit is granted and no `fs::copy`
//!   (which carries the source's mode) is made outside this file, but for the functions
//!   [`DATA_COPIES`] names with their reason. **Its bound:** it reads source, so it holds
//!   the two spellings a suite here has used; a `chmod` run as a command, or a write into
//!   a file that is executable already, it does not see.

use std::fs;
use std::io::{self, ErrorKind, PipeReader, PipeWriter, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::thread;

use crate::support::rust_source;
use crate::support::scratch::ScratchDir;
use crate::support::test_homes;

/// Write `content` at `path` as an executable — mode `0o755`, whatever was there before.
pub(crate) fn write(path: &Path, content: impl AsRef<[u8]>) {
    let staging = ScratchDir::new("staged-executable");
    let staged = staging.path().join("content");
    fs::write(&staged, content)
        .unwrap_or_else(|e| panic!("stage the content of {}: {e}", path.display()));
    by_a_child(&staged, path);
    mode(path, fs::Permissions::from_mode(0o755));
}

/// Copy `from` to `to` with `from`'s permission bits, as `fs::copy` does — for a file
/// that is executed where it lands.
pub(crate) fn copy(from: &Path, to: &Path) {
    let permissions = fs::metadata(from)
        .unwrap_or_else(|e| panic!("read the mode of {}: {e}", from.display()))
        .permissions();
    by_a_child(from, to);
    mode(to, permissions);
}

/// `chmod`, by path: it opens nothing.
fn mode(path: &Path, permissions: fs::Permissions) {
    fs::set_permissions(path, permissions)
        .unwrap_or_else(|e| panic!("set the mode of {}: {e}", path.display()));
}

/// The one write: `cp`, a process of its own, waited for.
///
/// It is given no pipe. A pipe is read to its end, and its end is every writer's: a
/// sibling's child forked while the pipe is open holds one until its own `exec`, so a
/// pipe here would only move the wait this module exists to remove.
fn by_a_child(from: &Path, to: &Path) {
    let status = Command::new("cp")
        .arg("--")
        .arg(from)
        .arg(to)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .unwrap_or_else(|e| panic!("run cp to place {}: {e}", to.display()));
    assert!(
        status.success(),
        "cp to place {} from {}: {status} — what it said is on this test's stderr",
        to.display(),
        from.display()
    );
}

/// What a placed script prints.
const PLACED: &str = "placed\n";

/// A script that prints [`PLACED`] and exits, followed by `padding` bytes of comment the
/// shell never reaches.
fn script(padding: usize) -> Vec<u8> {
    const LINE: &[u8] = b"# padding, so that writing this file takes a while\n";
    let mut text =
        format!("#!/bin/sh\nprintf '{}'\nexit 0\n", PLACED.escape_default()).into_bytes();
    let end = text.len() + padding;
    while text.len() < end {
        text.extend_from_slice(LINE);
    }
    text
}

/// Children of this process, each **held between its fork and its exec** — where a child
/// has a copy of every descriptor that was open when it was forked, close-on-exec or not.
///
/// Each is forked by a thread of its own, since a spawn returns only at the child's
/// `exec`. A child writes one byte when it exists and then waits for one; dropping this
/// gives every child its byte and joins the threads, on every path out of a test — a
/// child left waiting would hold its thread, and the test, for ever.
struct Held {
    forked: PipeReader,
    release: PipeWriter,
    threads: Vec<thread::JoinHandle<()>>,
}

impl Held {
    /// Start forking `count` children and return **at once**: the forks land while the
    /// caller goes on.
    fn start(count: usize) -> Held {
        let (forked, is_forked) = io::pipe().expect("a pipe for the children to report on");
        let (go_on, release) = io::pipe().expect("a pipe to release the children by");
        let threads = (0..count)
            .map(|_| {
                let is_forked = is_forked.try_clone().expect("clone the report pipe");
                let not_forked = is_forked.try_clone().expect("clone the report pipe");
                let go_on = go_on.try_clone().expect("clone the release pipe");
                thread::spawn(move || {
                    let mut child = Command::new("true");
                    child
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                    // SAFETY: the closure runs in the forked child of a threaded process,
                    // where only async-signal-safe calls are sound. It makes two, `write`
                    // and `read`, on descriptors it owns, and allocates nothing.
                    unsafe {
                        child.pre_exec(move || {
                            (&is_forked).write_all(&[1])?;
                            (&go_on).read_exact(&mut [0])
                        });
                    }
                    // A child that was never forked reports nothing, and the wait for
                    // its report would have no end: the thread reports in its place.
                    let status = child.status().unwrap_or_else(|e| {
                        let _ = (&not_forked).write_all(&[0]);
                        panic!("fork a held child: {e}")
                    });
                    assert!(status.success(), "the held child ran `true`: {status}");
                })
            })
            .collect();
        Held {
            forked,
            release,
            threads,
        }
    }

    /// Wait until every child exists: from here each holds whatever was open at its fork.
    fn all_forked(&mut self) {
        let mut reports = vec![0; self.threads.len()];
        self.forked
            .read_exact(&mut reports)
            .expect("every held child reports that it was forked");
        assert!(
            reports.iter().all(|report| *report == 1),
            "every held child was forked: {reports:?}"
        );
    }
}

impl Drop for Held {
    fn drop(&mut self) {
        let _ = self.release.write_all(&vec![1; self.threads.len()]);
        for thread in self.threads.drain(..) {
            let joined = thread.join();
            if !thread::panicking() {
                joined.expect("a thread that forked a held child");
            }
        }
    }
}

/// Run a placed file as a suite does: the kernel executes it, by its path.
fn run(path: &Path) -> io::Result<Output> {
    Command::new(path).stdin(Stdio::null()).output()
}

fn ran_as_placed(ran: &io::Result<Output>) -> bool {
    matches!(ran, Ok(out) if out.status.success() && out.stdout == PLACED.as_bytes())
}

/// **The control: the mechanism, deterministic.** A file written in this process — the
/// shape the helpers replace — whose handle was open when a sibling's child was forked is
/// busy for as long as that child has not reached its `exec`, though the handle was closed
/// here before the file was run. Once the child goes on, the same file runs.
///
/// The refusal is Linux's, so it is asserted there; elsewhere the arm still shows that the
/// file runs. A kernel that stops refusing turns this red, and that is the day these
/// helpers have nothing left to do.
#[test]
fn a_write_handle_a_forked_sibling_holds_makes_the_file_busy_where_linux_checks() {
    let dir = ScratchDir::new("placed-executable-control");
    let path = dir.path().join("written-in-this-process");

    let mut handle = fs::File::create(&path).expect("create the file in this process");
    handle.write_all(&script(0)).expect("write it");
    let mut held = Held::start(1);
    held.all_forked();
    drop(handle);
    mode(&path, fs::Permissions::from_mode(0o755));

    let while_held = run(&path);
    drop(held);
    let after = run(&path);

    assert!(
        ran_as_placed(&after),
        "the file runs once no child holds a handle on it: {after:?}"
    );
    if cfg!(target_os = "linux") {
        assert!(
            matches!(&while_held, Err(e) if e.kind() == ErrorKind::ExecutableFileBusy),
            "Linux refuses to run a file a forked child still holds open for writing — \
             the mechanism `placed_executable` exists for: {while_held:?}"
        );
    }
}

/// How many children are forked during each placement, and how many placements of each
/// kind are driven.
const SIBLINGS: usize = 4;
const ROUNDS: usize = 6;

/// Megabytes, so that a write made in this process would be open for milliseconds — long
/// enough for every sibling's fork to land inside it.
const PADDING: usize = 4 << 20;

#[test]
fn what_is_placed_runs_at_once_while_siblings_are_held_before_their_exec() {
    let dir = ScratchDir::new("placed-executable-siblings");
    let content = script(PADDING);
    let source = dir.path().join("source");
    write(&source, &content);

    let mut refused = Vec::new();
    for round in 0..ROUNDS {
        for how in ["write", "copy"] {
            // A name with a space, both quotes and a `#`, as the rigs' roots have.
            let to = dir.path().join(format!("it's \"{how}\" #{round}"));
            let mut held = Held::start(SIBLINGS);
            match how {
                "write" => write(&to, &content),
                _ => copy(&source, &to),
            }
            held.all_forked();
            let ran = run(&to);
            drop(held);
            if !ran_as_placed(&ran) {
                refused.push(format!("round {round}, `{how}`: {ran:?}"));
            }
        }
    }
    assert!(
        refused.is_empty(),
        "a placed file is run at once, whatever this process forked meanwhile — {} of {} \
         were not:\n  {}",
        refused.len(),
        2 * ROUNDS,
        refused.join("\n  ")
    );
}

#[test]
fn a_placed_file_has_the_bytes_and_the_mode_it_was_given() {
    let dir = ScratchDir::new("placed-executable-bytes");
    let mode_of = |path: &Path| {
        fs::metadata(path)
            .expect("the placed file's metadata")
            .permissions()
            .mode()
            & 0o7777
    };

    // Over a file that was there, and was not executable.
    let written = dir.path().join("written");
    fs::write(
        &written,
        "something else, and longer than what replaces it\n",
    )
    .expect("a file");
    mode(&written, fs::Permissions::from_mode(0o600));
    write(&written, script(0));
    assert_eq!(fs::read(&written).expect("read it"), script(0));
    assert_eq!(mode_of(&written), 0o755);

    // A copy has its source's mode, whatever that is.
    for bits in [0o755, 0o700, 0o644] {
        mode(&written, fs::Permissions::from_mode(bits));
        let copied = dir.path().join(format!("copied-{bits:o}"));
        copy(&written, &copied);
        assert_eq!(fs::read(&copied).expect("read the copy"), script(0));
        assert_eq!(mode_of(&copied), bits, "the copy of a 0o{bits:o} file");
    }
}

/// Functions of a tooling suite that `fs::copy` what is **never executed**, each with why.
const DATA_COPIES: &[(&str, &str, &str)] = &[(
    "dev_stabilize_record.rs",
    "copy_tree",
    "a run's record directory, set aside and put back: records, which nothing executes",
)];

/// This file's own name: where the helpers and their control are, and the one suite the
/// fence does not read.
const THIS_FILE: &str = "placed_executable.rs";

fn line_of(source: &str, at: usize) -> usize {
    source[..at].matches('\n').count() + 1
}

/// Whether the line holding `at` is a comment from its first character.
///
/// The scan reads the source **as written**, and only this is taken out of it — a
/// mention in a string, or behind code on its line, is an offence until it is reworded.
/// It errs that way on purpose: the shared reading that blanks strings
/// (`support::rust_source::code_only`) takes a `'"'` for the start of one and reads the
/// rest of the file inside out, which hid `dev_gate_report`'s copy of `dev/hygiene-scan`
/// from this fence's first draft.
fn in_a_comment_line(source: &str, at: usize) -> bool {
    let start = source[..at].rfind('\n').map_or(0, |newline| newline + 1);
    source[start..at].trim_start().starts_with("//")
}

/// The function `at` lies in: the name after the nearest `fn ` before it.
fn enclosing_fn(source: &str, at: usize) -> Option<&str> {
    let start = source[..at].rfind("fn ")? + 3;
    source[start..]
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
        .filter(|name| !name.is_empty())
}

/// The mode a call is given, when it is written as a number.
fn literal_mode(argument: &str) -> Option<u32> {
    let digits = argument.trim().replace('_', "");
    if let Some(octal) = digits.strip_prefix("0o") {
        u32::from_str_radix(octal, 8).ok()
    } else if let Some(hex) = digits.strip_prefix("0x") {
        u32::from_str_radix(hex, 16).ok()
    } else {
        digits.parse().ok()
    }
}

/// Every place `source` — the suite file named `file` — grants an executable bit or
/// copies a file with its mode, outside [`DATA_COPIES`].
fn offences(file: &str, source: &str) -> Vec<String> {
    const RULE: &str = "a file that will be executed is placed through \
                        `placed_executable::write` or `placed_executable::copy`, which never \
                        open it in this process";
    let mut found = Vec::new();

    for call in ["from_mode(", "set_mode(", ".mode("] {
        for (at, _) in source.match_indices(call) {
            let named_so = call.starts_with('.')
                || !source[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_');
            if !named_so || in_a_comment_line(source, at) {
                continue;
            }
            let rest = &source[at + call.len()..];
            let argument = rest[..rest.find(')').unwrap_or(rest.len())].trim();
            if argument.is_empty() {
                // A mode that is read, not given.
                continue;
            }
            let line = line_of(source, at);
            match literal_mode(argument) {
                Some(bits) if bits & 0o111 == 0 => {}
                Some(bits) => found.push(format!(
                    "{file}:{line}: `{call}0o{bits:o})` grants an executable bit — {RULE}"
                )),
                None => found.push(format!(
                    "{file}:{line}: `{call}{argument})` is a mode this fence cannot read — \
                     write it as a number; and if it grants an executable bit, {RULE}"
                )),
            }
        }
    }

    for (at, _) in source.match_indices("fs::copy(") {
        if in_a_comment_line(source, at) {
            continue;
        }
        let within = enclosing_fn(source, at);
        let data = DATA_COPIES
            .iter()
            .any(|(suite, function, _)| *suite == file && Some(*function) == within);
        if !data {
            found.push(format!(
                "{file}:{}: `fs::copy` carries its source's mode, so it is how an executable \
                 lands with no `chmod` in sight — {RULE}; what is never executed is read and \
                 written, or its function is named in `DATA_COPIES` with the reason",
                line_of(source, at)
            ));
        }
    }
    found
}

/// Every suite file of the tooling's homes but this one, as `(file name, source)`.
fn suites() -> Vec<(String, String)> {
    let mut suites = Vec::new();
    for home in test_homes::homes_outside_the_crates() {
        for path in rust_source::rust_files(&home) {
            let name = path
                .file_name()
                .expect("a suite file has a name")
                .to_string_lossy()
                .into_owned();
            if name != THIS_FILE {
                let source = fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
                suites.push((name, source));
            }
        }
    }
    suites
}

#[test]
fn no_tooling_suite_makes_a_file_executable_or_copies_one_but_through_the_helper() {
    let suites = suites();
    assert!(
        suites
            .iter()
            .any(|(name, _)| name == "dev_stabilize_record.rs"),
        "the scan reads the tooling suites: {:?}",
        suites.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    let found: Vec<String> = suites
        .iter()
        .flat_map(|(name, source)| offences(name, source))
        .collect();
    assert!(
        found.is_empty(),
        "{} place(s) in the tooling suites write a file in this process that is then \
         executed — on Linux a forked sibling can still hold that handle when it is run \
         (`Text file busy`), and no gate on macOS shows it:\n  {}",
        found.len(),
        found.join("\n  ")
    );

    // An entry that excuses nothing is one nobody reads again.
    for (suite, function, _) in DATA_COPIES {
        let copies = suites
            .iter()
            .filter(|(name, _)| name == suite)
            .any(|(_, source)| {
                source
                    .match_indices("fs::copy(")
                    .any(|(at, _)| enclosing_fn(source, at) == Some(*function))
            });
        assert!(
            copies,
            "`DATA_COPIES` names `{function}` of `{suite}`, which makes no `fs::copy` — \
             drop the entry"
        );
    }
}

#[test]
fn a_planted_writer_reddens_the_scan() {
    let plants = [
        "fn stub() { fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap(); }",
        "fn stub() { perms.set_mode(0o700); }",
        "fn stub() { perms.set_mode(0o0_644 | 1); }",
        "fn stub() { perms.set_mode(mode); }",
        "fn stub() { fs::OpenOptions::new().mode(0o711).write(true).open(p).unwrap(); }",
        "fn stub() { fs::set_permissions(&p, Permissions::from_mode(493)).unwrap(); }",
        "fn rig() { std::fs::copy(repo.join(SCRIPT), root.join(SCRIPT)).unwrap(); }",
        // The excused function, in a suite it is not excused in.
        "fn copy_tree() { fs::copy(a, b).unwrap(); }",
        // Behind a `'"'`, which the shared blanking reader takes for a string's start.
        "fn f() { s.strip_suffix('\"'); fs::copy(a, b).unwrap(); }",
        // A mention is an offence unless its whole line is a comment.
        "fn f() { let said = \"perms.set_mode(0o755)\"; }",
    ];
    for plant in plants {
        let found = offences("some_suite.rs", plant);
        assert_eq!(found.len(), 1, "one offence in `{plant}`: {found:?}");
        assert!(
            found[0].starts_with("some_suite.rs:1: "),
            "it is named by file and line: {}",
            found[0]
        );
    }

    let clean = [
        (
            "some_suite.rs",
            "fn f() { fs::set_permissions(&p, fs::Permissions::from_mode(0o644)).unwrap(); }",
        ),
        ("some_suite.rs", "fn f() { perms.set_mode(0o600); }"),
        (
            "some_suite.rs",
            "fn f() -> u32 { meta.permissions().mode() & 0o777 }",
        ),
        (
            "some_suite.rs",
            "    // fs::copy(a, b), then from_mode(0o755)\nfn f() {}",
        ),
        ("some_suite.rs", "/// `perms.set_mode(0o755)`\nfn f() {}"),
        (
            "some_suite.rs",
            "fn f() { if update_mode() { placed_executable::copy(a, b); } }",
        ),
        (
            "dev_stabilize_record.rs",
            "fn copy_tree() { fs::copy(a, b).unwrap(); }",
        ),
    ];
    for (file, source) in clean {
        let found = offences(file, source);
        assert!(found.is_empty(), "no offence in `{source}`: {found:?}");
    }
}
