//! **What a test starts, its rig ends** — [`RigDir`], the directory of a rig that can start
//! a process which outlives the call that started it, and the arms that hold it
//! (DECISIONS.md → 2026-10-11, *A tooling test ends what it started*).
//!
//! **The failure it exists for.** `dev/stabilize-step hold-start` puts a command in a
//! session of its own, so that it outlives its starter: that is what a hold is for. A test
//! that drives one therefore starts a process nothing ends when the test does — and the
//! suites' stand-in for a held command waited for a file in its rig's directory, which the
//! rig removed on its way out. A stand-in still running at that moment waited for a file
//! that could no longer appear: fifty-four of them were found on one machine, the oldest
//! three and a half days old, each waking ten times a second.
//!
//! **What holds it.** A rig's directory has a name no other directory has — its label, the
//! test process's id and a counter — and every process a rig starts is started by a path
//! under it, so the directory's name is in its command line. When a [`RigDir`] is dropped —
//! the test returned, an assertion of it failed, or it panicked — every process whose
//! command line names the directory is ended, **each by its pid**, until none is left; only
//! then is the directory removed. No process is ended by a pattern that another rig's could
//! match: a second run of the same test, in this process or in another checkout's gate,
//! has another directory.
//!
//! - [`a_rig_directory_ends_the_processes_that_name_it_and_no_other`] drives the guard
//!   itself: a process in a group of its own under each of two directories; the first
//!   directory dropped — on a return, and on a panic — and its process is gone while the
//!   other's runs on.
//! - [`a_suite_that_can_start_a_detached_process_holds_no_directory_but_a_rig_directory`]
//!   reads the source: the tools of `dev/` that put a process in a session of its own are
//!   the two this file names, and a suite that names an act of theirs keeps no bare
//!   [`ScratchDir`].
//! - `dev_stabilize_step` drives it through the tool: what a test holds is ended however
//!   the test ends, and the stand-in for a held command ends by itself.
//!
//! **And a group is ended by one spelling** (DECISIONS.md → 2026-10-12, *The tooling leg
//! runs in three parts*). A test that ends a whole process group — a starter's, to show
//! that a hold outlives it — does it through [`end_group`], because the bare spelling is
//! the group on macOS and another group, or every process, on Linux:
//! [`a_group_is_ended_whole_and_no_other_with_it`] drives it, and
//! [`no_suite_names_a_group_to_kill_without_the_double_dash`] reads the suites for any
//! other.
//!
//! **Its bound.** A process is the rig's by what its command line names. One that names the
//! directory without the rig having started it — a hand's `tail` in a rig that is still
//! there — is ended with it. And a process the rig started by a path that does not name the
//! directory is not seen: the suites start none, and the tools resolve their own paths.

use std::fs;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

use crate::support::rust_source;
use crate::support::scratch::ScratchDir;

/// A process, as `ps` lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Process {
    pub(crate) pid: u32,
    /// Its process group: a process in a session of its own leads one.
    pub(crate) group: u32,
    pub(crate) command: String,
}

/// Every process there is, or none where `ps` could not be asked.
fn listed() -> Option<Vec<Process>> {
    let out = Command::new("ps")
        .args(["-A", "-ww", "-o", "pid=", "-o", "pgid=", "-o", "args="])
        .output()
        .ok()
        .filter(|out| out.status.success())?;
    let number = |text: &str| -> Option<(u32, String)> {
        let (number, rest) = text.trim_start().split_once(char::is_whitespace)?;
        Some((number.parse().ok()?, rest.to_owned()))
    };
    Some(
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter_map(|line| {
                let (pid, rest) = number(line)?;
                let (group, command) = number(&rest)?;
                Some(Process {
                    pid,
                    group,
                    command: command.trim_start().to_owned(),
                })
            })
            .collect(),
    )
}

/// Whether a command line names the directory called `name`: the name, and no digit after
/// it — a name ends in a counter, and a longer counter is another directory's.
fn names(command: &str, name: &str) -> bool {
    command
        .match_indices(name)
        .any(|(at, _)| !command[at + name.len()..].starts_with(|next: char| next.is_ascii_digit()))
}

fn naming_among(all: Vec<Process>, dir: &Path) -> Vec<Process> {
    let name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| panic!("a rig's directory has a name: {}", dir.display()));
    let mut naming: Vec<Process> = all
        .into_iter()
        .filter(|process| names(&process.command, name))
        .collect();
    naming.sort_by_key(|process| process.pid);
    naming
}

/// The processes alive whose command line names `dir`.
pub(crate) fn naming(dir: &Path) -> Vec<Process> {
    naming_among(listed().expect("`ps` lists the processes"), dir)
}

/// The process group this test process is in.
pub(crate) fn own_group() -> u32 {
    listed()
        .expect("`ps` lists the processes")
        .into_iter()
        .find(|process| process.pid == std::process::id())
        .expect("this process is listed")
        .group
}

/// End every process whose command line names `dir`, each by its pid, until none is left —
/// a process ended while it starts another leaves that one for the next look. It answers
/// with what is still there after ten seconds: nothing, since no process refuses the signal.
fn end(dir: &Path) -> Vec<Process> {
    let patience = Instant::now() + Duration::from_secs(10);
    loop {
        let alive = listed().map_or_else(Vec::new, |all| naming_among(all, dir));
        if alive.is_empty() || Instant::now() > patience {
            return alive;
        }
        let _ = Command::new("kill")
            .arg("-9")
            .args(alive.iter().map(|process| process.pid.to_string()))
            .output();
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// End a whole process group, named by its leader's id: `kill -9 -- -<group>`.
///
/// **The `--` is the act, and a suite ends a group through this function and by no
/// spelling of its own.** The `kill` of procps — Ubuntu's, so a hosted runner's — takes
/// `-<group>` behind a signal for a run of options. Measured 2026-10-12 on procps 4.0.4,
/// in the runner-shaped container: `kill -9 -238` left group 238 running and exited 0,
/// and `kill -9 -128` ended every process its caller could signal, an unrelated group and
/// the calling shell among them. The group it signals is that of the number's FIRST
/// DIGIT, and `-1` is everyone. macOS' `kill` reads both spellings as the group, so the
/// bare one was green on every gate here and red, or worse, on Linux. With `--` both
/// platforms signal the group named and no other
/// ([`a_group_is_ended_whole_and_no_other_with_it`]).
pub(crate) fn end_group(group: u32) -> Output {
    Command::new("kill")
        .args(["-9", "--", &format!("-{group}")])
        .output()
        .expect("run kill")
}

/// A rig's directory: fresh, of a name no other has, and — when it is dropped — rid of
/// every process whose command line names it before it is removed.
pub(crate) struct RigDir(ScratchDir);

impl RigDir {
    pub(crate) fn new(label: &str) -> Self {
        RigDir(ScratchDir::new(label))
    }

    pub(crate) fn path(&self) -> &Path {
        self.0.path()
    }
}

impl Drop for RigDir {
    fn drop(&mut self) {
        let left = end(self.0.path());
        // A test that is failing already fails for its own reason, and says so.
        if !std::thread::panicking() {
            assert!(
                left.is_empty(),
                "a rig ends what it started, and these run on after it: {left:?}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The guard itself
// ---------------------------------------------------------------------------

/// A script that runs for a minute and ends: what stands for a process nobody ends, and
/// is none itself.
const A_MINUTE: &str = "n=0\nwhile [ \"$n\" -lt 600 ]; do sleep 0.1; n=$((n + 1)); done\n";

/// A process in a group of its own, started by a path under `dir` — as a held
/// command's supervisor is in a session of its own.
fn detached(dir: &Path) -> Child {
    let script = dir.join("a-minute");
    fs::write(&script, A_MINUTE).expect("write the script");
    let child = Command::new("sh")
        .arg(&script)
        .process_group(0)
        .stdin(Stdio::null())
        .spawn()
        .expect("spawn a shell");
    let asked = Instant::now();
    while !naming(dir).iter().any(|process| process.pid == child.id()) {
        assert!(
            asked.elapsed() < Duration::from_secs(30),
            "the control: a process started under {} is listed as naming it",
            dir.display()
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    child
}

/// **When a rig's directory is dropped, the processes that name it are gone — on a
/// return and on a panic — and no other rig's is touched.** The control is in
/// [`detached`]: each process is listed as naming its directory before anything is
/// dropped, so a listing that sees nothing is no pass.
#[test]
fn a_rig_directory_ends_the_processes_that_name_it_and_no_other() {
    let other = RigDir::new("rig-processes-other");
    let mut runs_on = detached(other.path());
    assert_ne!(
        naming(other.path())[0].group,
        own_group(),
        "the process is in a group of its own: no signal to this test's group reaches it"
    );

    // ON A RETURN.
    let returned = RigDir::new("rig-processes-returned");
    let (dir, mut child) = (returned.path().to_path_buf(), detached(returned.path()));
    drop(returned);
    assert_eq!(
        naming(&dir),
        vec![],
        "the rig is gone, and so is its process"
    );
    assert_eq!(
        child.wait().expect("the process is gone").signal(),
        Some(9),
        "it was ended, and did not end by itself"
    );

    // ON A PANIC — a failed assertion is one.
    let mut started: Option<(PathBuf, Child)> = None;
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let rig = RigDir::new("rig-processes-panicked");
        started = Some((rig.path().to_path_buf(), detached(rig.path())));
        assert_eq!(1 + 1, 3, "a test that fails with a process running");
    }));
    assert!(panicked.is_err(), "the test failed");
    let (dir, mut child) = started.expect("the process was started");
    assert_eq!(
        naming(&dir),
        vec![],
        "the rig is gone, and so is its process"
    );
    assert_eq!(child.wait().expect("the process is gone").signal(), Some(9));

    // AND NO OTHER: the first rig's process ran on through both.
    assert_eq!(
        runs_on.try_wait().expect("ask after the other process"),
        None,
        "another rig's process is not this rig's to end"
    );
    assert_eq!(naming(other.path()).len(), 1);
    drop(other);
    assert_eq!(runs_on.wait().expect("it is gone").signal(), Some(9));
}

/// **A group is ended whole, by its number, and no other group with it** — [`end_group`].
/// Two processes, each leading a group of its own; the first group is ended: its leader
/// died of the signal, within ten seconds, and the other's runs on.
///
/// **Its bound.** It cannot be red on macOS whichever way the group is spelled, so what it
/// holds is read on Linux — in CI, and in `dev/runner-faithful`. There the bare spelling
/// leaves the group running (red here, after the ten seconds) or, where the group's number
/// opens with 1, ends this test run with everything else (no arm reports that).
#[test]
fn a_group_is_ended_whole_and_no_other_with_it() {
    let other = RigDir::new("rig-processes-group-other");
    let mut runs_on = detached(other.path());
    let rig = RigDir::new("rig-processes-group");
    let mut leader = detached(rig.path());
    assert_eq!(
        (naming(rig.path())[0].group, naming(other.path())[0].group),
        (leader.id(), runs_on.id()),
        "the control: each leads a group of its own"
    );

    let ended = end_group(leader.id());
    assert!(ended.status.success(), "{ended:?}");
    let asked = Instant::now();
    let died_of = loop {
        if let Some(status) = leader.try_wait().expect("ask after the leader") {
            break status.signal();
        }
        assert!(
            asked.elapsed() < Duration::from_secs(10),
            "ten seconds after its group was ended, the leader still runs: the group named \
             was not the group signalled"
        );
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(died_of, Some(9), "the leader was ended, by the signal");
    assert_eq!(
        runs_on.try_wait().expect("ask after the other process"),
        None,
        "another group is not the group named"
    );
}

/// **No suite hands `kill` a group by a spelling of its own.** A statement that runs
/// `kill` and formats a negative number into its arguments also hands it `--`, as
/// [`end_group`] does — the one such statement there is. **Its bound:** it reads source,
/// so it holds a `kill` run as `Command::new("kill")` whose group is formatted in the same
/// statement; a group spelled inside a shell script's text is not seen.
#[test]
fn no_suite_names_a_group_to_kill_without_the_double_dash() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli");
    // Built, never written out: this arm's own text holds no statement that runs `kill`.
    let runs_kill = format!("Command::new({:?})", "kill");
    // A literal that opens with a dash and a formatted value; the literal `--`.
    let negative = ['"', '-', '{'].iter().collect::<String>();
    let dashes = format!("{:?}", "--");
    let mut statements = 0;
    let mut groups = 0;
    let mut bare = Vec::new();
    for path in rust_source::rust_files(&root.join("tooling-tests")) {
        let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
        let text = rust_source::code_and_strings(&text);
        for (at, _) in text.match_indices(&runs_kill) {
            let statement = &text[at..];
            let statement = &statement[..statement.find(';').unwrap_or(statement.len())];
            statements += 1;
            if !statement.contains(&negative) {
                continue;
            }
            groups += 1;
            if !statement.contains(&dashes) {
                let line = text[..at].lines().count();
                let file = path.strip_prefix(root).unwrap_or(&path);
                bare.push(format!("{}:{line}", file.display()));
            }
        }
    }
    assert!(
        statements > groups && groups >= 1,
        "the control: the suites run `kill` ({statements} statements), and one that names a \
         group is read as one ({groups})"
    );
    assert_eq!(
        bare,
        Vec::<String>::new(),
        "a group handed to `kill` without `--` is, on Linux, another group or every process"
    );
}

/// A directory whose name another's begins with is not that other: a rig's name ends in
/// a counter, and the name is read up to its last digit.
#[test]
fn a_name_is_read_whole() {
    let name = "jigc-trial-stabilize-step-hold-dead-53311-179";
    for (command, named) in [
        (
            "/bin/sh <tmp>/jigc-trial-stabilize-step-hold-dead-53311-179/dev/gate",
            true,
        ),
        (
            "python3 x --scratch <tmp>/jigc-trial-stabilize-step-hold-dead-53311-179",
            true,
        ),
        (
            "/bin/sh <tmp>/jigc-trial-stabilize-step-hold-dead-53311-1790/dev/gate",
            false,
        ),
        (
            "/bin/sh <tmp>/jigc-trial-stabilize-step-hold-53311-179/dev/gate",
            false,
        ),
        ("sleep 0.1", false),
    ] {
        assert_eq!(names(command, name), named, "{command}");
    }
}

/// The spellings by which a script of `dev/` puts a process in a session of its own.
const DETACHES: &[&str] = &["os.setsid(", "start_new_session=True", "setsid ", "nohup "];

/// The tools of `dev/` that do, each with the act of it that does: a suite that drives it
/// names the tool by its path and the act by its word.
const DETACHING: &[(&str, &str)] = &[
    ("stabilize-probe", "--detach"),
    ("stabilize-step", "hold-start"),
];

/// **A suite that can start a detached process keeps no directory but a [`RigDir`].**
/// Which tools detach is read off `dev/` — a tool that begins to joins [`DETACHING`]
/// with its act, or this arm is red — and a suite that names such a tool and its act
/// names no [`ScratchDir`]: every directory it makes ends what was started under it. **Its
/// bound:** it reads source, so it holds the spellings [`DETACHES`] lists, and a suite
/// that names the tool and the act as [`DETACHING`] spells them.
#[test]
fn a_suite_that_can_start_a_detached_process_holds_no_directory_but_a_rig_directory() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli");
    let files = |dir: &str| -> Vec<(String, String)> {
        let mut files: Vec<(String, String)> = fs::read_dir(root.join(dir))
            .unwrap_or_else(|e| panic!("read {dir}/: {e}"))
            .flatten()
            .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                Some((name, fs::read_to_string(entry.path()).ok()?))
            })
            .collect();
        files.sort();
        files
    };
    // A script's comment may speak of a session; its code starts one.
    let code = |text: &str| -> String {
        text.lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let detaching: Vec<String> = files("dev")
        .into_iter()
        .filter(|(_, text)| DETACHES.iter().any(|spelt| code(text).contains(spelt)))
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        detaching,
        DETACHING.iter().map(|(tool, _)| *tool).collect::<Vec<_>>(),
        "the tools of dev/ that put a process in a session of its own"
    );
    let bare: Vec<String> = files("tooling-tests")
        .into_iter()
        .filter(|(name, _)| name.ends_with(".rs") && name != "rig_processes.rs")
        .filter(|(_, text)| {
            let text = rust_source::code_and_strings(text);
            let drives = |(tool, act): &(&str, &str)| {
                text.contains(&format!("dev/{tool}")) && text.contains(act)
            };
            DETACHING.iter().any(drives) && text.contains("ScratchDir")
        })
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        bare,
        Vec::<String>::new(),
        "a suite that names an act which starts a detached process keeps its directories as `RigDir`s"
    );
}
