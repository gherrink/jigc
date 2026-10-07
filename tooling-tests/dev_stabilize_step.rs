//! **`dev/stabilize-step` — every git act of a stabilization run's harness, as ONE command**
//! ([DECISIONS.md](../DECISIONS.md) → *2026-10-06 — Every git act of the stabilization
//! harness is one command*; the repair plan's change C0,
//! [repair-plan.md](../completions/artifacts/M55/stabilization-build/repair-plan.md)).
//!
//! The harness has no shell, so a git act of a run is a step it hands to a `build-git`
//! agent. Until the tool, a step was a numbered list of shell commands the agent followed,
//! and nothing could execute one in a test. Now the agent runs one command and relays the
//! one line it prints — and this suite is where an act is executed: **against real git**,
//! in a throwaway clone with a bare remote, a fixed identity and fixed dates, hooks off.
//!
//! **Four things are held here.**
//!
//! - *Each act does what its list did.* Every act is driven once to its end, and each
//!   refusal its list had is driven too — a tree that is not clean, a pushed branch that is
//!   ahead, a branch that is not the one the step names, a merge whose tree differs, and
//!   the rest ([`REFUSALS`]). For the acts that change something the git commands the tool
//!   ran are **traced** and held to the list, command for command: the rig's `git` is a
//!   shim that writes down each call the tool itself makes and then runs the real one. The
//!   tool was built to change nothing a step checks; the trace is how that is read.
//! - *The tool can do nothing an agent may not.* [`offences`] reads every git call out of
//!   the tool's source — the call sites are derived, never listed — and holds each to the
//!   flags its subcommand may carry, each name a changing command takes to an argument
//!   whose grammar requires a branch prefix, and `main` to the one act that fetches it.
//!   The spellings of a forced push are read from the definition of the agent that runs
//!   the steps. [`a_planted_offender_reddens_the_scan`] plants each forbidden act in a copy
//!   of the source and requires the scan to name it.
//! - *The harness and the tool are one contract.* Where `node` is installed, the harness's
//!   own prompt functions compose each step, the one command is taken out of the prompt and
//!   run **verbatim**, and the harness's own `readStep` reads the line back
//!   ([`every_command_the_harness_composes_is_taken_by_the_tool_and_read_back`]). The close's
//!   sync act is held to the build harness's sync step by the commands it ran
//!   ([`the_sync_act_runs_the_build_harnesss_sync_step`]) — the half of
//!   [`merge_logs_fence`](super::merge_logs_fence)'s arm (k) that can no longer be read off
//!   two prompts.
//!
//! - *Nothing is committed or pushed by a step that the record script would not have
//!   written* (the second repair plan's `K1`; [DECISIONS.md](../DECISIONS.md) → *2026-10-07 —
//!   What a step publishes is vetted where it becomes permanent*). A reporter has a shell,
//!   and a commit is whoever made it; so what a writer checks at its write is checked again
//!   at the report check, at the record's commit and at every push — and **every refusal
//!   has its exit, driven here and not described**: the file that is no report leaves the
//!   tree, the batch is taken back and the next attempt begins, and the read a refused
//!   push names is run as the refusal spells it ([`Stage`] stands a stage at its record
//!   through the two scripts; the tests are the section *What is vetted, and where*).
//!   **The scanners there**: the denylist half is the real `dev/hygiene-scan`, over a
//!   denylist of one term the rig writes; gitleaks is the record suite's stand-in, which
//!   keeps gitleaks' exit contract, reads the tree and never a range, and names no file —
//!   so [`the_machines_own_gitleaks_reads_the_range_a_push_would_publish`] runs whatever
//!   the machine has over a real range.
//!
//! - *A record act is repeatable, and every invocation reconciles first* (the second repair
//!   plan's `K2`; [DECISIONS.md](../DECISIONS.md) → *2026-10-07 — A record act is
//!   repeatable, and every invocation reconciles first*). A record is three commands, and
//!   what a kill, a failed commit or a rejected push leaves between two of them has a name
//!   and an answer in ONE table the tool holds as data and prints (`table`), keyed by act
//!   and phase. **Kills are driven for real**: each command runs under [`KILLER`], which
//!   sends the unmodified script SIGKILL between two of its system calls, in a copy of a
//!   stage stood up once ([`StepRig::copy`]) — and every end is asserted through the
//!   record script's `state`, the tree, the journal and the remote ([`End`]), never
//!   described ([`killed_before_each_point`]). Every arrival state of the table is driven by a test this suite names
//!   ([`DRIVEN`]), so a cell added to the table without its test is red; the tests are
//!   the section *A record act is repeatable, and every invocation reconciles first*.
//!
//! - *A line holds what the harness acts on, and nothing else* (the second repair plan's
//!   `K9`; [DECISIONS.md](../DECISIONS.md) → *2026-10-07 — A line holds what the harness
//!   acts on*). Asked for its **digest** (`--digest`), an act prints ids, words, counts
//!   and hashes of declared shapes — printable ASCII with no escape in it — and names the
//!   file that holds the line it prints unasked. Every act and sixteen refusals are
//!   driven through the flag ([`every_line_is_a_digest`]); **and every line this rig reads
//!   unasked is put to the tool's own projection and held as a digest with nothing unfit**
//!   (`ran_in`, [`StepRig::projected`]) — so each refusal [`REFUSALS`] names is held in the
//!   test that drives it, here and in the simulation. The tests are the section *The
//!   digest*; what no suite shows — that a real agent relays one whole — is the relay
//!   probe's.
//!
//! - *A long command is started, waited for and judged by the tool* (the second repair
//!   plan's `K10`; [DECISIONS.md](../DECISIONS.md) → *2026-10-07 — A long command is
//!   started, waited for and judged by the tool*; the human's ruling of 2026-10-07 that no
//!   step of a stage may need his permission). `hold-start` starts one in a session of its
//!   own and answers at once; `hold-wait` answers within a slice the tool owns; and the
//!   verdict is read by the tool, out of the command's own output, per kind. **Driven for
//!   real, with commands that are stand-ins** ([`LONG_COMMAND`]: it runs until the test
//!   releases it — a file, never a number of seconds — and prints what a file holds; and
//!   [`CARGO`], which "builds" what the tree it runs in says): the starter exits and
//!   another process waits, two wait at once, a supervisor and a command are killed with
//!   SIGKILL. **The verdict readers are fed recorded real output** ([`as_recorded`],
//!   [`regression_line`]), and the probe's kind runs the real probe tool. The tool's
//!   kinds are [`HELD_KINDS`], each by a test of this file; the tests are the section *A
//!   held command*. What no arm shows: a command that outlives the SHELL of its starter
//!   being ended — the hold probe's case (b) observed that — and anything with an agent.
//!
//! **Every arm runs under a shell-hostile root** — a space, a `'`, a `"` and a `#` in the
//! repository's path — because the rig has no other kind.
//!
//! **The rig is the bed the simulation of a stage is built on**
//! ([`stabilize_simulation`](super::stabilize_simulation)). [`StepRig`] is built in two
//! steps — [`StepRig::unopened`], then the opening — so that a suite can open the run
//! itself, and lends its environment; [`node_or_skip`] is what every test that runs the
//! harness under `node` asks first. That suite's agents are functions of its runtime's
//! stand-in, so [`harness_pure`] (the harness's pure functions, called under `node`) and
//! [`git_agent`] (a `build-git` agent as a function: the one command of a step's prompt,
//! run, and its line relayed) serve this suite's own contract arm.
//!
//! **Preserved on purpose, and marked where it is driven.** The tool repairs nothing. A
//! landing that halted after its merge is still answered `landed-before` by the next call,
//! unchecked and unpushed (the harness review's `H5`); the tree a stage starts from still
//! admits no untracked file but a report, the round's scope file included; and the
//! path-class assert is the list and the subject rule it was (`M8`). Each belongs to a
//! later task of the repair, and each is an assertion here that task will turn.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::support::child_stdin;
use crate::support::install_line::INSTALL_COMMAND;
use crate::support::scratch::ScratchDir;

use super::dev_stabilize_record::{DENY_TERM, STUB_SECRET, gitleaks_stub};
use super::placed_executable;

const TOOL: &str = "dev/stabilize-step";
const RECORD: &str = "dev/stabilize-record";
const MERGE_LOGS: &str = "dev/merge-logs";
/// The denylist half of the public-hygiene scan, and gitleaks' configuration: what the
/// record script runs wherever it vets what a step would commit or push. The denylist's
/// one term and what the stand-in for gitleaks reports are the record suite's.
const SCANNER: &str = "dev/hygiene-scan";
const GITLEAKS_CONFIG: &str = ".gitleaks.toml";
const HARNESS: &str = ".claude/workflows/stabilize.js";
const BUILD_HARNESS: &str = ".claude/workflows/milestone-build.js";
const GIT_DEFINITION: &str = ".claude/agents/build-git.md";

/// The run every rig holds, and the names the harness would mint for it.
pub(crate) const RUN: &str = "rc24";
pub(crate) const LOOP: &str = "fix/rc24";
pub(crate) const ROUNDS: &str = "fix/rc24-r";
pub(crate) const RUN_DIR: &str = "completions/artifacts/rc24";

/// A space, both quotes and a `#`: a path that breaks any command that is not quoted.
const HOSTILE_ROOT: &str = "it's a \"repo\" #1";

/// The word a refusal opens with, and its exit status.
const REFUSALS: &[(&str, i32)] = &[
    ("usage", 2),
    ("wrong-branch", 3),
    ("dirty", 4),
    ("remote-ahead", 5),
    ("product-path", 6),
    ("foreign-merge", 7),
    ("merge-in-round", 8),
    ("conflict", 9),
    ("merge-logs", 10),
    ("unresolved", 11),
    ("merge-differs", 12),
    ("pick-stopped", 13),
    ("carry-differs", 14),
    ("push-rejected", 15),
    ("remote-differs", 16),
    ("record", 17),
    ("git", 18),
    ("gate-red", 19),
    ("no-batch", 20),
    ("position", 21),
    ("unvetted", 22),
    ("did-not-run", 23),
    ("locked", 24),
    ("head-moved", 25),
    ("foreign-commit", 26),
    ("committed", 27),
    ("taken", 28),
    ("missing", 29),
    ("build", 30),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("read `{rel}`: {e}"))
}

/// The quoted entries of the harness's one-line list `const NAME = [ … ]`.
fn harness_list(name: &str) -> Vec<String> {
    let source = read(HARNESS);
    let declaration = format!("const {name} = [");
    let line = source
        .lines()
        .find(|line| line.starts_with(&declaration))
        .unwrap_or_else(|| panic!("the harness declares `{name}`"));
    line[declaration.len()..]
        .split('\'')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

/// The product paths and the append-only logs, as the harness hands them to the tool: its
/// own lists, never a copy of this suite's.
fn product() -> Vec<String> {
    harness_list("PRODUCT_PATHS")
}
fn logs() -> Vec<String> {
    harness_list("SYNC_LOGS")
}

// ---------------------------------------------------------------------------
// The rig
// ---------------------------------------------------------------------------

/// The rig's `git`: it writes down a call the tool itself made — never one of a script
/// the tool ran, nor one of the fixture's — and runs the real git.
///
/// It tells the tool's own calls by its parent's command line, which it asks `ps` for —
/// so this suite, and the simulation that shares its rig, need `ps` beside `git` and
/// `python3`. Both places they must run have it (established 2026-10-06): the image
/// `dev/runner-faithful` builds is `FROM ubuntu:24.04`, whose base carries `procps`
/// (`/usr/bin/ps`; priority *important*) though the image's own install list does not
/// name it, and on a GitHub-hosted `ubuntu-latest` runner every test of this suite passed
/// (CI run 37513437924) — which none that reads a trace can without it. A machine without
/// `ps` would leave every trace EMPTY: the arms that expect calls fail, by name.
const GIT_SHIM: &str = r#"#!/bin/sh
if [ -n "${STEP_TRACE:-}" ]; then
case "$(ps -o command= -p "$PPID")" in
*dev/stabilize-step*) printf '%s\n' "$*" >>"$STEP_TRACE" ;;
esac
fi
exec "@GIT@" "$@"
"#;

/// The real `git` on the ambient `PATH`.
fn real_git() -> PathBuf {
    let path = std::env::var("PATH").expect("the suite runs with a UTF-8 PATH");
    path.split(':')
        .map(|dir| Path::new(dir).join("git"))
        .find(|candidate| candidate.is_file())
        .expect("git is on PATH")
}

/// A throwaway repository under a shell-hostile path, with a bare remote beside it: the
/// tool's own bytes and the two scripts it runs, committed; `main` pushed; and the loop
/// branch of one opened run checked out and pushed, its tip the commit that added the
/// opening record.
pub(crate) struct StepRig {
    dir: ScratchDir,
    pub(crate) root: PathBuf,
    pub(crate) origin: PathBuf,
    /// The scratch root a `state` act is handed: outside the repository, of plain segments.
    pub(crate) scratch: PathBuf,
    home: PathBuf,
    trace: PathBuf,
    path: String,
}

/// What a call of the tool left: its exit status, the one line it printed — held to the
/// hash it ends with before it is read — its stderr, and the git commands it ran.
pub(crate) struct Stepped {
    pub(crate) code: i32,
    pub(crate) raw: String,
    pub(crate) line: Value,
    pub(crate) stderr: String,
    pub(crate) trace: Vec<String>,
}

impl Stepped {
    /// The act ran to its end and says `status`.
    pub(crate) fn done(&self, act: &str, status: &str) -> &Value {
        assert_eq!(
            self.code, 0,
            "`{act}` exits 0\nline: {}\nstderr: {}",
            self.raw, self.stderr
        );
        assert_eq!(
            self.line["act"], act,
            "the line names its act: {}",
            self.raw
        );
        assert_eq!(self.line["status"], status, "the line: {}", self.raw);
        assert!(
            self.stderr.is_empty(),
            "an act that is done prints nothing on stderr: {}",
            self.stderr
        );
        &self.line
    }

    /// The act refused with `word`: its status, one line on stderr that opens with the
    /// word, and a line on stdout that carries the same word and a halt report.
    pub(crate) fn refused(&self, word: &str) -> &Value {
        let status = REFUSALS
            .iter()
            .find(|(known, _)| *known == word)
            .map(|(_, status)| *status)
            .unwrap_or_else(|| panic!("`{word}` is a refusal of the tool"));
        assert_eq!(
            self.code, status,
            "refused `{word}` exits {status}\nline: {}\nstderr: {}",
            self.raw, self.stderr
        );
        let opening = format!("stabilize-step: refused {word}: ");
        assert!(
            self.stderr.starts_with(&opening) && self.stderr.trim_end().lines().count() == 1,
            "a refusal is one line on stderr that opens `{opening}`: {}",
            self.stderr
        );
        assert_eq!(self.line["status"], "halted", "the line: {}", self.raw);
        assert_eq!(self.line["refused"], word, "the line: {}", self.raw);
        let halt = &self.line["halt"];
        assert!(
            halt["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.starts_with(&format!("{word}: "))),
            "the halt report's root cause opens with the word: {}",
            self.raw
        );
        for field in ["evidence", "tree_state", "recommendation"] {
            assert!(halt[field].is_string(), "the halt report has `{field}`");
        }
        &self.line
    }
}

fn sha256(text: &str) -> String {
    let mut child = Command::new("python3")
        .args([
            "-c",
            "import hashlib, sys; sys.stdout.write(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn python3");
    child_stdin::feed(&mut child, text);
    let out = child.wait_with_output().expect("python3 exits");
    assert!(out.status.success(), "python3 hashes the text");
    String::from_utf8(out.stdout).expect("a hex digest")
}

/// The one line of a call's stdout, as the object it is — after the line is held to the
/// sha256 it ends with, which is the sha256 of the line without that field.
fn line_of(stdout: &str) -> Value {
    assert!(
        stdout.ends_with('\n') && stdout.trim_end().lines().count() == 1,
        "the tool prints ONE line on stdout: {stdout:?}"
    );
    let line = stdout.trim_end_matches('\n');
    let marker = ", \"sha256\": \"";
    let at = line
        .rfind(marker)
        .unwrap_or_else(|| panic!("the line ends with its sha256: {line}"));
    let hash = &line[at + marker.len()..];
    let hash = hash
        .strip_suffix("\"}")
        .unwrap_or_else(|| panic!("the sha256 is the line's last field: {line}"));
    assert_eq!(
        sha256(&format!("{}}}", &line[..at])),
        hash,
        "the line's sha256 is that of the line without it: {line}"
    );
    serde_json::from_str(line).unwrap_or_else(|e| panic!("the line is JSON ({e}): {line}"))
}

impl StepRig {
    /// The rig with its one run opened: the loop branch checked out and pushed, its tip the
    /// commit that added the opening record.
    pub(crate) fn new(label: &str) -> Self {
        let rig = Self::unopened(label);
        rig.git(&["switch", "-q", "-c", LOOP]);
        rig.write(&format!("{RUN_DIR}/opening.md"), "# the opening record\n");
        rig.commit("docs(record): the run is opened");
        rig.git(&["push", "-q", "origin", LOOP]);
        rig
    }

    /// The rig before any run is opened: `main` checked out and pushed, and nothing else —
    /// for a suite that opens the run itself.
    pub(crate) fn unopened(label: &str) -> Self {
        let dir = ScratchDir::new(&format!("stabilize-step-{label}"));
        let root = dir.path().join(HOSTILE_ROOT);
        let origin = dir.path().join("origin.git");
        let scratch = dir.path().join("scratch");
        let home = dir.path().join("home");
        let bin = dir.path().join("bin");
        for made in [&root.join("dev"), &scratch, &home, &bin] {
            fs::create_dir_all(made).expect("create a rig directory");
        }
        let rig = StepRig {
            trace: dir.path().join("trace"),
            path: format!(
                "{}:{}",
                bin.display(),
                std::env::var("PATH").expect("the suite runs with a UTF-8 PATH")
            ),
            dir,
            root,
            origin,
            scratch,
            home,
        };
        rig.on_path(
            "git",
            &GIT_SHIM.replace("@GIT@", &real_git().display().to_string()),
        );

        // The base: the tool and what it runs, a product path, the two logs, a file that
        // is neither.
        for script in [TOOL, RECORD, MERGE_LOGS, SCANNER] {
            placed_executable::copy(&repo_root().join(script), &rig.root.join(script));
        }
        // What a vet needs: gitleaks' configuration, as committed; the stand-in for
        // gitleaks the record suite keeps, first on the `PATH`; and a denylist of one term.
        let config = fs::read(repo_root().join(GITLEAKS_CONFIG)).expect("read the gitleaks config");
        fs::write(rig.root.join(GITLEAKS_CONFIG), config).expect("write the rig's gitleaks config");
        rig.on_path("gitleaks", &gitleaks_stub());
        fs::write(rig.denylist(), format!("# a private term\n\n{DENY_TERM}\n"))
            .expect("write the denylist");
        rig.write("Cargo.toml", "[workspace]\n");
        rig.write("crates/a.txt", "a\n");
        rig.write("README.md", "a readme\n");
        rig.write(
            "DECISIONS.md",
            "# Decisions\n\nThe log.\n\n## 2026-01-01 — the base\n\nThe base entry.\n",
        );
        rig.write(
            "implementation/project-history.md",
            "# History\n\nThe record.\n",
        );
        rig.git(&["init", "-q", "-b", "main"]);
        rig.git(&["config", "core.hooksPath", "/dev/null"]);
        rig.commit("chore: the base");
        let bare = rig
            .hermetic(Command::new("git"))
            .args(["init", "-q", "--bare"])
            .arg(&rig.origin)
            .output()
            .expect("run git init --bare");
        assert!(bare.status.success(), "init the bare remote: {bare:?}");
        rig.git(&["remote", "add", "origin", &rig.origin.display().to_string()]);
        rig.git(&["push", "-q", "origin", "main"]);
        rig
    }

    /// The directory the rig lies in — the repository, its remote and the scratch root are
    /// its children — for what a suite keeps beside them.
    pub(crate) fn dir(&self) -> &Path {
        self.dir.path()
    }

    /// The rig's denylist: every child of the rig is pointed at it.
    pub(crate) fn denylist(&self) -> PathBuf {
        self.dir.path().join("denylist")
    }

    /// An executable placed first on the `PATH` of every child of the rig.
    pub(crate) fn on_path(&self, name: &str, script: &str) {
        placed_executable::write(&self.dir.path().join("bin").join(name), script);
    }

    /// The environment every child of the rig runs in: its own home, no git configuration
    /// of the machine's, a fixed identity and fixed dates.
    pub(crate) fn hermetic(&self, mut command: Command) -> Command {
        command
            .env("PATH", &self.path)
            .env("HOME", &self.home)
            .env("JIGC_DENYLIST_FILE", self.denylist())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_AUTHOR_NAME", "The Rig")
            .env("GIT_AUTHOR_EMAIL", "rig@example.invalid")
            .env("GIT_AUTHOR_DATE", "2026-01-01T00:00:00Z")
            .env("GIT_COMMITTER_NAME", "The Rig")
            .env("GIT_COMMITTER_EMAIL", "rig@example.invalid")
            .env("GIT_COMMITTER_DATE", "2026-01-01T00:00:00Z")
            .env("PYTHONPYCACHEPREFIX", self.dir.path().join("pycache"))
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("STEP_TRACE");
        command
    }

    fn git_at(&self, at: &Path, args: &[&str]) -> std::process::Output {
        self.hermetic(Command::new("git"))
            .args(args)
            .current_dir(at)
            .output()
            .expect("run git")
    }

    /// A git command of the fixture's, in the work clone: it must succeed. Its stdout,
    /// without the last line break.
    pub(crate) fn git(&self, args: &[&str]) -> String {
        let out = self.git_at(&self.root, args);
        assert!(
            out.status.success(),
            "`git {}` in the rig: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout)
            .trim_end_matches('\n')
            .to_owned()
    }

    /// Whether a git command of the fixture's succeeds.
    pub(crate) fn git_ok(&self, args: &[&str]) -> bool {
        self.git_at(&self.root, args).status.success()
    }

    pub(crate) fn write(&self, rel: &str, text: &str) {
        let path = self.root.join(rel);
        fs::create_dir_all(path.parent().expect("a file has a directory"))
            .expect("create the file's directory");
        fs::write(&path, text).unwrap_or_else(|e| panic!("write `{rel}` in the rig: {e}"));
    }

    pub(crate) fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root.join(rel))
            .unwrap_or_else(|e| panic!("read `{rel}` in the rig: {e}"))
    }

    /// Everything in the tree committed; the commit's sha.
    pub(crate) fn commit(&self, subject: &str) -> String {
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "-m", subject]);
        self.rev("HEAD")
    }

    /// One file written and committed.
    pub(crate) fn change(&self, rel: &str, text: &str, subject: &str) -> String {
        self.write(rel, text);
        self.commit(subject)
    }

    pub(crate) fn rev(&self, name: &str) -> String {
        self.git(&["rev-parse", name])
    }

    pub(crate) fn branch(&self) -> String {
        self.git(&["branch", "--show-current"])
    }

    /// What `git status --porcelain --untracked-files=all` prints.
    pub(crate) fn status(&self) -> String {
        self.git(&["status", "--porcelain", "--untracked-files=all"])
    }

    /// The sha the remote holds for a branch, or none.
    pub(crate) fn remote(&self, branch: &str) -> Option<String> {
        let out = self.git_at(
            &self.origin,
            &[
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/heads/{branch}"),
            ],
        );
        out.status
            .success()
            .then(|| String::from_utf8_lossy(&out.stdout).trim().to_owned())
    }

    /// The commits the remote's heads stand at, sorted and each once: what the tool is told
    /// when it ASKS THE REMOTE what it holds (`git ls-remote --heads origin`).
    pub(crate) fn remote_heads(&self) -> Vec<String> {
        let out = self.git_at(
            &self.origin,
            &["for-each-ref", "--format=%(objectname)", "refs/heads"],
        );
        assert!(out.status.success(), "list the remote's heads: {out:?}");
        let heads: BTreeSet<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(str::to_owned)
            .collect();
        heads.into_iter().collect()
    }

    /// A commit somebody else pushed: made in a second clone, so that the remote's branch
    /// has a commit the work clone lacks.
    pub(crate) fn pushed_by_another(&self, branch: &str, rel: &str, text: &str) -> String {
        let other = self
            .dir
            .path()
            .join(format!("other-{}", branch.replace('/', "-")));
        if !other.exists() {
            let cloned = self
                .hermetic(Command::new("git"))
                .args(["clone", "-q", "--branch", branch])
                .arg(&self.origin)
                .arg(&other)
                .output()
                .expect("run git clone");
            assert!(cloned.status.success(), "clone the remote: {cloned:?}");
        }
        fs::create_dir_all(other.join(rel).parent().expect("a directory"))
            .expect("create the file's directory");
        fs::write(other.join(rel), text).expect("write the other clone's file");
        for args in [
            vec!["add", "-A"],
            vec!["commit", "-q", "-m", "docs: a commit somebody else pushed"],
            vec!["push", "-q", "origin", branch],
        ] {
            let out = self.git_at(&other, &args);
            assert!(
                out.status.success(),
                "`git {}` elsewhere: {out:?}",
                args.join(" ")
            );
        }
        String::from_utf8_lossy(&self.git_at(&other, &["rev-parse", "HEAD"]).stdout)
            .trim()
            .to_owned()
    }

    /// A hook of the bare remote — the clone's own hooks are off.
    fn remote_hook(&self, name: &str, body: &str) {
        let hook = self.origin.join("hooks").join(name);
        fs::create_dir_all(hook.parent().expect("the hooks directory")).expect("create hooks/");
        placed_executable::write(&hook, format!("#!/bin/sh\n{body}\n"));
    }

    fn ran(&self, command: Command) -> Stepped {
        self.ran_in(&[], command)
    }

    /// As [`StepRig::ran`], with `env` over the rig's own environment.
    fn ran_in(&self, env: &[(&str, &str)], command: Command) -> Stepped {
        fs::write(&self.trace, "").expect("empty the trace");
        // A line of the step tool that was not asked for as a digest is held to having one
        // (below): the tool's by its path, or the one command of a step's prompt.
        let spelled: Vec<String> = std::iter::once(command.get_program())
            .chain(command.get_args())
            .map(|word| word.to_string_lossy().into_owned())
            .collect();
        let whole = spelled[0] == self.root.join(TOOL).display().to_string()
            || spelled
                .iter()
                .any(|word| word.starts_with(&format!("{TOOL} ")));
        let whole = whole && !spelled.iter().any(|word| word.contains("--digest"));
        let out = self
            .hermetic(command)
            .envs(env.iter().copied())
            .env("STEP_TRACE", &self.trace)
            .stdin(Stdio::null())
            .output()
            .expect("run the tool");
        let raw = String::from_utf8_lossy(&out.stdout).into_owned();
        let stepped = Stepped {
            code: out.status.code().expect("the tool exits"),
            line: line_of(&raw),
            raw,
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
            trace: fs::read_to_string(&self.trace)
                .expect("read the trace")
                .lines()
                .map(str::to_owned)
                .collect(),
        };
        if whole {
            // EVERY LINE THIS RIG SEES HAS A DIGEST, and every value of it fits the shape
            // the tool declares for its field — whatever the act, done or refused, in this
            // suite and in the simulation that shares the rig.
            let digest = self.projected(&stepped.raw);
            assert_eq!(
                digest["unfit"],
                json!([]),
                "a value of this line does not fit the shape its digest declares: {}",
                stepped.raw
            );
        }
        stepped
    }

    /// The tool, called by its path from a directory that is not the repository: it finds
    /// its repository from where it lies.
    pub(crate) fn step(&self, args: &[&str]) -> Stepped {
        let mut command = Command::new(self.root.join(TOOL));
        command.args(args).current_dir(self.dir.path());
        self.ran(command)
    }

    /// The tool, with `env` over the rig's own environment: a scanner that is not there.
    fn step_in(&self, env: &[(&str, &str)], args: &[&str]) -> Stepped {
        let mut command = Command::new(self.root.join(TOOL));
        command.args(args).current_dir(self.dir.path());
        self.ran_in(env, command)
    }

    /// A command line of the record script's — a call of it, or a line a refusal of the
    /// tool names — run by a shell from the repository's root, fed `stdin`: its exit
    /// status, what it printed, and its stderr.
    fn record_script(&self, command_line: &str, stdin: &str) -> (i32, String, String) {
        let mut child = self
            .hermetic(Command::new("sh"))
            .args(["-c", command_line])
            .current_dir(&self.root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn a shell");
        child_stdin::feed(&mut child, stdin);
        let out = child.wait_with_output().expect("the shell exits");
        (
            out.status.code().expect("the record script exits"),
            String::from_utf8_lossy(&out.stdout).into_owned(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        )
    }

    /// A call of the record script that must succeed: what it printed.
    fn wrote(&self, call: &str, stdin: &str) -> String {
        let (code, out, err) = self.record_script(&format!("{RECORD} {call}"), stdin);
        assert_eq!(code, 0, "`{RECORD} {call}`: {err}");
        out
    }

    /// A command line exactly as a step's prompt gives it, run as an agent runs it: by a
    /// shell, from the repository's root.
    pub(crate) fn shell(&self, command_line: &str) -> Stepped {
        let mut command = Command::new("sh");
        command.args(["-c", command_line]).current_dir(&self.root);
        self.ran(command)
    }

    /// The round's branch opened from the loop branch and checked out, by plain git.
    pub(crate) fn round(&self, number: u32) -> String {
        let branch = format!("{ROUNDS}{number}");
        self.git(&["switch", "-q", LOOP]);
        self.git(&["switch", "-q", "--no-track", "-c", &branch, LOOP]);
        branch
    }
}

fn with<'a>(head: &[&'a str], lists: &[(&'a str, &'a [String])]) -> Vec<&'a str> {
    let mut args = head.to_vec();
    for (flag, values) in lists {
        for value in *values {
            args.push(flag);
            args.push(value.as_str());
        }
    }
    args
}

fn lines(list: &[&str]) -> Vec<String> {
    list.iter().map(|line| (*line).to_owned()).collect()
}

/// **The git calls of the tool's one push**, in its order (the tool's header: *What a push
/// publishes*): the remote is ASKED what it holds, and which of those commits this clone
/// holds too; the branch's commit is read once; the range is that commit less what the
/// remote holds, by sha; after the vet both are asked again; then the push, and the remote
/// read back. `held` is what the remote's heads stood at before the call
/// ([`StepRig::remote_heads`]) and `tip` the commit the branch stood at.
fn push_calls(branch: &str, tip: &str, held: &[String]) -> Vec<String> {
    let held = held.join(" ");
    vec![
        "ls-remote --heads origin".to_owned(),
        format!("rev-list --no-walk --ignore-missing {held}"),
        format!("rev-parse {branch}"),
        format!("rev-list --reverse {tip} --not {held}"),
        "ls-remote --heads origin".to_owned(),
        format!("rev-parse {branch}"),
        format!("push origin {branch}"),
        format!("ls-remote --exit-code --heads origin {branch}"),
    ]
}

/// A report as `dev/stabilize-record report` leaves one: text, and its last line.
const REPORT: &str = "# a report\n\nNothing found.\n\n<!-- end of report -->\n";

// ---------------------------------------------------------------------------
// git-state
// ---------------------------------------------------------------------------

fn git_state<'a>(stage: &'a str, product: &'a [String]) -> Vec<&'a str> {
    with(
        &[
            "git-state",
            "--stage",
            stage,
            "--loop",
            LOOP,
            "--rounds",
            ROUNDS,
            "--run-dir",
            RUN_DIR,
        ],
        &[("--product", product)],
    )
}

#[test]
fn git_state_reads_the_state_a_stage_starts_from() {
    let rig = StepRig::new("git-state");
    let product = product();
    let opening = rig.rev("HEAD");
    // What a stage wrote through the record script and no record step has committed is
    // what a tree may hold: a report, and the round's scope — each at the one path the
    // record script names for it (`pending`). A stage that stopped after its scope step
    // left exactly these.
    let report = format!("{RUN_DIR}/r1/reports/test/scope.a1.md");
    let scope = format!("{RUN_DIR}/r1/scope.md");
    rig.write(&report, "a report\n");
    rig.write(&scope, "the scope\n");

    let seen = rig.step(&git_state("test", &product));
    let line = seen.done("git-state", "ready");
    assert_eq!(
        *line,
        json!({
            "act": "git-state", "status": "ready", "found": [], "finished": [], "owed": [],
            "branch": LOOP, "head": opening, "loop_head": opening, "remote_head": opening,
            "opening": opening, "untracked": [report, scope],
            "pending": null, "sha256": line["sha256"],
        })
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            "rev-parse --git-path index.lock",
            "status --porcelain --untracked-files=all",
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("fetch origin {LOOP}"),
            &format!("merge-base --is-ancestor origin/{LOOP} {LOOP}"),
            &format!("log --diff-filter=A --format=%H -n 1 {LOOP} -- {RUN_DIR}/opening.md"),
            &format!(
                "log --first-parent --no-merges --format=%H {opening}..{LOOP} -- {}",
                product.join(" ")
            ),
            &format!("log --first-parent --merges --format=%s {opening}..{LOOP}"),
            &format!("rev-parse {LOOP}"),
            "rev-parse HEAD",
            &format!("rev-parse {LOOP}"),
        ]),
        "the commands of the read, in its order: the branch, git's lock, the tree, the remote, the path-class assert, and what the remote lacks — nothing, here"
    );
}

#[test]
fn git_state_holds_the_branch_to_the_stage() {
    let rig = StepRig::new("git-state-branch");
    let product = product();
    let branch = rig.round(1);
    // The `test` stage works on the loop branch; the `fix` stage on it or on a round's.
    rig.step(&git_state("test", &product))
        .refused("wrong-branch");
    let line = rig.step(&git_state("fix", &product));
    let line = line.done("git-state", "ready");
    assert_eq!(line["branch"], branch.as_str());
    assert_eq!(line["loop_head"], rig.rev(LOOP).as_str());
    // A branch that is neither is refused for both.
    rig.git(&["switch", "-q", "main"]);
    for stage in ["test", "fix"] {
        let seen = rig.step(&git_state(stage, &product));
        seen.refused("wrong-branch");
        assert_eq!(seen.trace[0], "branch --show-current");
        assert!(
            seen.line["halt"]["tree_state"]
                .as_str()
                .is_some_and(|tree| tree.contains("on `main`")),
            "a halt says what is checked out: {}",
            seen.raw
        );
    }
}

/// What is done to a clean tree before a step is asked for.
type Soil<'a> = &'a dyn Fn(&StepRig);

#[test]
fn git_state_refuses_a_tree_that_holds_more_than_reports() {
    let product = product();
    let dirty: [(&str, Soil); 6] = [
        ("a tracked file modified", &|rig| {
            rig.write("README.md", "changed\n")
        }),
        ("an untracked file outside the run", &|rig| {
            rig.write("notes.txt", "x\n")
        }),
        // A file under the run's directory is pending only at a path the record script
        // names: a note beside the tables, and a file beside the reports that is no
        // report's name, are a tree no stage starts from.
        ("an untracked file of the run that is no record's", &|rig| {
            rig.write(&format!("{RUN_DIR}/notes.md"), "a note\n");
        }),
        (
            "an untracked file beside the reports that is no report",
            &|rig| {
                rig.write(&format!("{RUN_DIR}/r1/reports/test/notes.txt"), "x\n");
            },
        ),
        // And a table nobody applied as a batch is a table changed by hand.
        (
            "a tracked file of the run modified, with no batch applied",
            &|rig| {
                rig.write(&format!("{RUN_DIR}/opening.md"), "# another opening\n");
            },
        ),
        ("a staged file", &|rig| {
            rig.write(&format!("{RUN_DIR}/r1/reports/test/x.a1.md"), "x\n");
            rig.git(&["add", "-A"]);
        }),
    ];
    for (what, soil) in dirty {
        let rig = StepRig::new("git-state-dirty");
        soil(&rig);
        let before = rig.status();
        let seen = rig.step(&git_state("test", &product));
        seen.refused("dirty");
        assert_eq!(
            rig.status(),
            before,
            "{what}: a refusal leaves the tree as it was"
        );
        assert_eq!(
            seen.trace.len(),
            5,
            "{what}: nothing runs past the check but the halt's two reads: {:?}",
            seen.trace
        );
    }
}

#[test]
fn git_state_refuses_a_pushed_loop_branch_that_is_ahead() {
    let rig = StepRig::new("git-state-ahead");
    let head = rig.rev("HEAD");
    rig.pushed_by_another(LOOP, &format!("{RUN_DIR}/note.md"), "a record\n");
    rig.step(&git_state("test", &product()))
        .refused("remote-ahead");
    assert_eq!(
        rig.rev("HEAD"),
        head,
        "the local branch is never moved to the remote's"
    );
}

#[test]
fn git_state_asserts_that_product_paths_move_only_through_a_rounds_merge() {
    let product = product();

    // A round's merge may carry a product change.
    let rig = StepRig::new("git-state-merge");
    let branch = rig.round(1);
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    rig.git(&["switch", "-q", LOOP]);
    rig.git(&["merge", "-q", "--no-ff", "--no-edit", &branch]);
    rig.git(&["push", "-q", "origin", LOOP]);
    rig.step(&git_state("test", &product))
        .done("git-state", "ready");

    // A commit made directly on the loop branch may not.
    let direct = rig.change("crates/a.txt", "tuned\n", "chore: a direct change");
    let seen = rig.step(&git_state("test", &product));
    let line = seen.refused("product-path");
    assert!(
        line["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(&direct))
            && line["halt"]["evidence"]
                .as_str()
                .is_some_and(|shown| shown.contains("crates/a.txt")),
        "the commit and the path it changed are named: {}",
        seen.raw
    );

    // A direct commit that touches no product path is no concern of the assert's — once
    // its author pushed it: unpushed, it is a commit no stage starts over
    // ([`a_commit_that_is_no_records_is_not_pushed_by_the_tool`]). And a merge that is not
    // a round's is refused whatever it carries.
    let rig = StepRig::new("git-state-foreign");
    rig.change("README.md", "tuned\n", "docs: a direct change");
    rig.step(&git_state("test", &product))
        .refused("foreign-commit");
    rig.git(&["push", "-q", "origin", LOOP]);
    rig.step(&git_state("test", &product))
        .done("git-state", "ready");
    rig.git(&["switch", "-q", "-c", "work/side"]);
    rig.change("notes.md", "x\n", "docs: a side note");
    rig.git(&["switch", "-q", LOOP]);
    rig.git(&["merge", "-q", "--no-ff", "--no-edit", "work/side"]);
    rig.step(&git_state("test", &product))
        .refused("foreign-merge");

    // Before the opening record is committed the assert does not run.
    let rig = StepRig::new("git-state-unopened");
    rig.git(&["switch", "-q", "-c", "fix/later", "main"]);
    rig.change("crates/a.txt", "direct\n", "chore: before any opening");
    let seen = rig.step(&with(
        &[
            "git-state",
            "--stage",
            "test",
            "--loop",
            "fix/later",
            "--rounds",
            "fix/later-r",
            "--run-dir",
            "completions/artifacts/later",
        ],
        &[("--product", &product)],
    ));
    assert_eq!(seen.done("git-state", "ready")["opening"], "");
    assert!(
        !seen
            .trace
            .iter()
            .any(|call| call.contains("--first-parent")),
        "without an opening the path-class assert does not run: {:?}",
        seen.trace
    );
}

// ---------------------------------------------------------------------------
// state · check-reports
// ---------------------------------------------------------------------------

#[test]
fn state_relays_the_record_scripts_document_whole() {
    let rig = StepRig::new("state");
    let scratch = rig.scratch.display().to_string();
    let seen = rig.step(&[
        "state",
        "--run",
        RUN,
        "--scratch",
        &scratch,
        "--tag",
        "test-1",
    ]);
    let line = seen.done("state", "read");
    assert_eq!(line["branch"], LOOP);
    assert_eq!(line["state"]["run"], RUN);
    assert_eq!(line["state"]["opened"], true);
    assert_eq!(
        line["state"]["next"], "not-ready",
        "the document is the record script's"
    );
    // The document is kept where the step says, as the record script printed it.
    let kept = rig.scratch.join("state/test-1.json");
    assert_eq!(line["file"], kept.display().to_string().as_str());
    let kept: Value = serde_json::from_str(&fs::read_to_string(kept).expect("read the kept state"))
        .expect("the kept state is JSON");
    assert_eq!(kept, line["state"]);
    assert_eq!(seen.trace, lines(&["branch --show-current"]));

    // A run that was never opened is an answer, as it is the record script's.
    let seen = rig.step(&[
        "state",
        "--run",
        "never",
        "--scratch",
        &scratch,
        "--tag",
        "test-2",
    ]);
    assert_eq!(
        seen.done("state", "read")["state"],
        json!({"run": "never", "opened": false})
    );

    // A record the script refuses to read is the step's refusal, with that script's line.
    rig.write(&format!("{RUN_DIR}/ledger.md"), "not a table\n");
    let seen = rig.step(&[
        "state",
        "--run",
        RUN,
        "--scratch",
        &scratch,
        "--tag",
        "test-3",
    ]);
    let line = seen.refused("record");
    assert!(
        line["halt"]["evidence"]
            .as_str()
            .is_some_and(|shown| shown.contains("stabilize-record: refused corrupt")),
        "the record script's refusal is the evidence: {}",
        seen.raw
    );
    assert!(!rig.scratch.join("state/test-3.json").exists());
}

#[test]
fn check_reports_relays_the_check_and_a_failed_check_is_its_answer() {
    let rig = StepRig::new("check-reports");
    let check = [
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "1",
        "--",
        "scope",
    ];
    let seen = rig.step(&check);
    let line = seen.done("check-reports", "checked");
    assert_eq!(line["check"]["ok"], false);
    assert_eq!(line["check"]["missing"], json!(["scope"]));
    assert_eq!(line["aside"], json!([]));
    assert_eq!(
        seen.trace,
        lines(&["status --porcelain --untracked-files=all"]),
        "the one thing the check asks git: which files of the run no commit holds"
    );

    // A file at a report's path that is what the record script would have written — text,
    // no host path, its last line — is a report, whoever wrote it.
    rig.write(&format!("{RUN_DIR}/r1/reports/test/scope.a1.md"), REPORT);
    let line = rig.step(&check);
    let line = line.done("check-reports", "checked");
    assert_eq!(
        json!([line["check"]["ok"], line["aside"]]),
        json!([true, []])
    );

    // The fix stage's flags reach the record script too.
    let seen = rig.step(&[
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "fix",
        "--cycle",
        "2",
        "--attempt",
        "1",
        "--",
        "audit-review",
    ]);
    assert_eq!(
        seen.done("check-reports", "checked")["check"]["dir"],
        format!("{RUN_DIR}/r1/reports/fix/c2").as_str()
    );

    // What the record script refuses is the step's refusal.
    let seen = rig.step(&[
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "1",
        "--",
        "Not A Slug",
    ]);
    seen.refused("record");
}

// ---------------------------------------------------------------------------
// find-round · open-round · push
// ---------------------------------------------------------------------------

#[test]
fn find_round_lists_the_rounds_branches_here_and_on_origin() {
    let rig = StepRig::new("find-round");
    let prefix = format!("{ROUNDS}1");
    let find = ["find-round", "--prefix", prefix.as_str()];
    let line = rig.step(&find);
    let line = line.done("find-round", "ready");
    assert_eq!(line["local"], json!([]));
    assert_eq!(line["remote"], json!([]));

    for name in [
        "fix/rc24-r1",
        "fix/rc24-r1-part1",
        "fix/rc24-r10",
        "fix/rc24-r2",
    ] {
        rig.git(&["branch", name]);
    }
    rig.git(&["push", "-q", "origin", "fix/rc24-r1", "fix/rc24-r2"]);
    let seen = rig.step(&find);
    let line = seen.done("find-round", "ready");
    assert_eq!(line["branch"], LOOP);
    assert_eq!(line["head"], rig.rev("HEAD").as_str());
    assert_eq!(
        line["local"],
        json!(["fix/rc24-r1", "fix/rc24-r1-part1", "fix/rc24-r10"]),
        "every local branch the prefix opens, and no other round's"
    );
    assert_eq!(line["remote"], json!(["fix/rc24-r1"]));
    assert_eq!(
        seen.trace,
        lines(&[
            &format!("branch --list {prefix}* --format=%(refname:short)"),
            &format!("ls-remote --heads origin {prefix}*"),
            "branch --show-current",
            "rev-parse HEAD",
        ])
    );
}

#[test]
fn open_round_opens_the_branch_from_the_loop_branch_or_switches_to_it() {
    let rig = StepRig::new("open-round");
    let branch = format!("{ROUNDS}1");
    let open = [
        "open-round",
        "--loop",
        LOOP,
        "--branch",
        branch.as_str(),
        "--run-dir",
        RUN_DIR,
    ];
    rig.write(
        &format!("{RUN_DIR}/r1/reports/test/scope.a1.md"),
        "a report\n",
    );

    let seen = rig.step(&open);
    let line = seen.done("open-round", "ready");
    assert_eq!(line["created"], true);
    assert_eq!(line["branch"], branch.as_str());
    assert_eq!(line["head"], rig.rev(LOOP).as_str());
    assert_eq!(rig.branch(), branch);
    assert!(
        !rig.git_ok(&[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{branch}@{{upstream}}")
        ]),
        "a round's branch is opened without an upstream"
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("rev-parse --verify --quiet refs/heads/{branch}"),
            &format!("switch {LOOP}"),
            &format!("switch --no-track -c {branch} {LOOP}"),
            "branch --show-current",
            "rev-parse HEAD",
        ])
    );

    // A branch that exists is switched to, wherever the tree stands.
    let tip = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    rig.git(&["switch", "-q", LOOP]);
    let seen = rig.step(&open);
    let line = seen.done("open-round", "ready");
    assert_eq!(line["created"], false);
    assert_eq!(line["head"], tip.as_str());
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("rev-parse --verify --quiet refs/heads/{branch}"),
            &format!("switch {branch}"),
            "branch --show-current",
            "rev-parse HEAD",
        ])
    );

    // A tree that holds more than reports opens nothing.
    let rig = StepRig::new("open-round-dirty");
    rig.write("README.md", "changed\n");
    rig.step(&open).refused("dirty");
    assert!(!rig.git_ok(&[
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("refs/heads/{branch}")
    ]));
    assert_eq!(rig.branch(), LOOP);
}

#[test]
fn push_pushes_the_branch_by_name_and_reads_the_remote_back() {
    let rig = StepRig::new("push");
    let branch = rig.round(1);
    let push = ["push", "--branch", branch.as_str()];
    let first = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let held = rig.remote_heads();

    let seen = rig.step(&push);
    let line = seen.done("push", "ready");
    assert_eq!(
        *line,
        json!({
            "act": "push", "status": "ready", "vetted": [first], "remote_head": first,
            "branch": branch, "head": first, "sha256": line["sha256"],
        })
    );
    assert_eq!(rig.remote(&branch), Some(first.clone()));
    let mut expected = vec![
        "branch --show-current".to_owned(),
        format!("ls-remote --exit-code --heads origin {branch}"),
    ];
    expected.extend(push_calls(&branch, &first, &held));
    expected.push("rev-parse HEAD".to_owned());
    assert_eq!(
        seen.trace, expected,
        "a branch that is not pushed yet has nothing to compare; what the remote lacks of it is asked of the remote, and the vet is of the commit by its sha"
    );

    // A branch that is pushed is first held to its pushed tip.
    let second = rig.change("crates/a.txt", "fixed again\n", "fix: the same finding");
    let seen = rig.step(&push);
    let line = seen.done("push", "ready");
    assert_eq!(line["remote_head"], second.as_str());
    assert_eq!(
        line["vetted"],
        json!([second]),
        "what is vetted is what the remote does not hold yet, and no more"
    );
    assert_eq!(
        seen.trace[1..4],
        lines(&[
            &format!("ls-remote --exit-code --heads origin {branch}"),
            &format!("fetch origin {branch}"),
            &format!("merge-base --is-ancestor origin/{branch} {branch}"),
        ])
    );
}

#[test]
fn push_refuses_where_its_list_halted() {
    // A branch that is not the one the step names.
    let rig = StepRig::new("push-branch");
    let branch = rig.round(1);
    rig.git(&["switch", "-q", LOOP]);
    let seen = rig.step(&["push", "--branch", &branch]);
    seen.refused("wrong-branch");
    assert_eq!(rig.remote(&branch), None, "nothing was pushed");

    // A pushed branch with commits the local one lacks: never pulled, merged or rebased.
    let rig = StepRig::new("push-ahead");
    let head = rig.rev("HEAD");
    let theirs = rig.pushed_by_another(LOOP, &format!("{RUN_DIR}/note.md"), "a record\n");
    let seen = rig.step(&["push", "--branch", LOOP]);
    seen.refused("remote-ahead");
    assert_eq!(rig.rev("HEAD"), head);
    assert_eq!(rig.remote(LOOP), Some(theirs));
    assert!(
        !seen.trace.iter().any(|call| call.starts_with("push ")),
        "no push is tried: {:?}",
        seen.trace
    );

    // A push the remote does not take: never forced.
    let rig = StepRig::new("push-rejected");
    let pushed = rig.rev("HEAD");
    rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record\n",
        "docs(record): a note",
    );
    rig.remote_hook("pre-receive", "exit 1");
    let seen = rig.step(&["push", "--branch", LOOP]);
    seen.refused("push-rejected");
    assert_eq!(rig.remote(LOOP), Some(pushed.clone()));
    assert_eq!(
        seen.trace
            .iter()
            .filter(|call| call.starts_with("push "))
            .count(),
        1,
        "one push, by name, and no second try: {:?}",
        seen.trace
    );

    // A remote that does not stand at the local head after the push.
    let rig = StepRig::new("push-differs");
    let pushed = rig.rev("HEAD");
    rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record\n",
        "docs(record): a note",
    );
    rig.remote_hook(
        "post-receive",
        &format!("git update-ref refs/heads/{LOOP} {pushed}"),
    );
    rig.step(&["push", "--branch", LOOP])
        .refused("remote-differs");
}

// ---------------------------------------------------------------------------
// land
// ---------------------------------------------------------------------------

fn land<'a>(branch: &'a str, product: &'a [String], logs: &'a [String]) -> Vec<&'a str> {
    with(
        &[
            "land",
            "--loop",
            LOOP,
            "--branch",
            branch,
            "--run-dir",
            RUN_DIR,
        ],
        &[("--product", product), ("--log", logs)],
    )
}

/// A round with one fix on a product path and one record commit, checked out.
fn a_round(rig: &StepRig) -> String {
    let branch = rig.round(1);
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "the round's record\n",
        "docs(record): the fix cycle's record",
    );
    branch
}

#[test]
fn land_merges_the_round_with_a_merge_commit_and_pushes_the_loop_branch() {
    let rig = StepRig::new("land");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    let (pre, round_head) = (rig.rev(LOOP), rig.rev(&branch));
    let outside = format!(":(exclude){RUN_DIR}");
    let held = rig.remote_heads();

    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "merged");
    let merge = rig.rev(LOOP);
    assert_eq!(
        *line,
        json!({
            "act": "land", "status": "merged", "tip_moved": false, "moved_outside": [],
            "resolved_logs": [], "merge_commit": merge, "vetted": line["vetted"],
            "remote_head": merge, "head": merge, "sha256": line["sha256"],
        })
    );
    assert_eq!(
        line["vetted"].as_array().and_then(|vetted| vetted.last()),
        Some(&json!(merge)),
        "the landing's push vets what it publishes — the merge commit, last: {line}"
    );
    assert_eq!(rig.branch(), LOOP);
    assert_eq!(rig.remote(LOOP), Some(merge.clone()));
    assert_eq!(
        rig.git(&["rev-list", "--parents", "-n", "1", "HEAD"]),
        format!("{merge} {pre} {round_head}"),
        "a merge commit: the loop branch's old tip, then the round's"
    );
    assert_eq!(
        rig.git(&["log", "-1", "--format=%s"]),
        format!("Merge branch '{branch}' into {LOOP}"),
        "the subject the path-class assert reads"
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("rev-parse {branch}"),
            &format!("switch {LOOP}"),
            "rev-parse HEAD",
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("fetch origin {LOOP}"),
            &format!("merge-base --is-ancestor origin/{LOOP} {LOOP}"),
            &format!("merge-base --is-ancestor {branch} {LOOP}"),
            &format!("merge-base --is-ancestor {LOOP} {branch}"),
            &format!("merge-base {LOOP} {branch}"),
            &format!("diff --name-only {pre} {LOOP} -- . {outside}"),
            &format!("merge --no-ff --no-edit {branch}"),
            "rev-parse HEAD",
            "rev-list --parents -n 1 HEAD",
            "status --porcelain",
            &format!("diff --quiet HEAD {branch} -- {}", product.join(" ")),
            &format!("diff --quiet HEAD {branch} -- . {outside}"),
        ])
        .into_iter()
        .chain(push_calls(LOOP, &merge, &held))
        .collect::<Vec<_>>(),
        "the commands of the land step's list, in its order"
    );

    // A round that landed before is said to have, and nothing else runs.
    rig.git(&["switch", "-q", &branch]);
    let seen = rig.step(&land(&branch, &product, &logs));
    assert_eq!(seen.done("land", "landed-before")["head"], merge.as_str());
    assert_eq!(
        seen.trace.last().map(String::as_str),
        Some(format!("merge-base --is-ancestor {branch} {LOOP}").as_str())
    );
}

#[test]
fn land_takes_a_loop_branch_that_moved_in_the_runs_directory_only() {
    let rig = StepRig::new("land-moved");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    rig.git(&["switch", "-q", LOOP]);
    rig.change(
        &format!("{RUN_DIR}/r1/rulings.md"),
        "a ruling\n",
        "docs(record): the human's rulings",
    );
    rig.git(&["push", "-q", "origin", LOOP]);

    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "merged");
    assert_eq!(line["tip_moved"], true);
    assert_eq!(line["moved_outside"], json!([]));
    assert_eq!(rig.read("crates/a.txt"), "fixed\n");
    assert_eq!(rig.read(&format!("{RUN_DIR}/r1/rulings.md")), "a ruling\n");
}

/// Both sides add an entry to the decisions log at the same place.
fn both_add_a_decision(rig: &StepRig, branch: &str, theirs: &str) {
    let entry = |date: &str, who: &str| {
        format!(
            "# Decisions\n\nThe log.\n\n## {date} — {who}\n\nAn entry of {who}.\n\n## 2026-01-01 — the base\n\nThe base entry.\n"
        )
    };
    rig.git(&["switch", "-q", branch]);
    rig.change(
        "DECISIONS.md",
        &entry("2026-02-02", "ours"),
        "docs: a decision of ours",
    );
    rig.git(&["switch", "-q", theirs]);
    rig.change(
        "DECISIONS.md",
        &entry("2026-02-01", "theirs"),
        "docs: a decision of theirs",
    );
}

#[test]
fn land_resolves_a_conflict_in_the_append_only_logs_and_no_other() {
    let (product, logs) = (product(), logs());

    // Both branches appended to the decisions log: both entries stay, by dev/merge-logs.
    let rig = StepRig::new("land-logs");
    let branch = a_round(&rig);
    both_add_a_decision(&rig, &branch, LOOP);
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "merged");
    assert_eq!(line["resolved_logs"], json!(["DECISIONS.md"]));
    assert_eq!(line["moved_outside"], json!(["DECISIONS.md"]));
    assert_eq!(line["tip_moved"], true);
    let log = rig.read("DECISIONS.md");
    assert!(
        log.contains("An entry of ours.") && log.contains("An entry of theirs."),
        "both sides' entries are kept: {log}"
    );
    let merging: Vec<&str> = seen
        .trace
        .iter()
        .map(String::as_str)
        .skip_while(|call| !call.starts_with("merge --no-ff"))
        .take(5)
        .collect();
    assert_eq!(
        merging,
        [
            format!("merge --no-ff --no-edit {branch}").as_str(),
            "diff --name-only --diff-filter=U",
            "diff --name-only --diff-filter=U",
            "commit --no-edit",
            "rev-parse HEAD",
        ],
        "the conflict is listed, resolved by dev/merge-logs, listed again, and concluded"
    );
    assert!(
        !seen
            .trace
            .iter()
            .any(|call| call.contains(&format!(":(exclude){RUN_DIR}"))
                && call.starts_with("diff --quiet")),
        "where a log was resolved the merged tree is not held to the round's outside the run: {:?}",
        seen.trace
    );

    // A conflict anywhere else is the human's: the merge is aborted.
    let rig = StepRig::new("land-conflict");
    let branch = a_round(&rig);
    rig.change(
        "README.md",
        "the round's readme\n",
        "docs: the round's readme",
    );
    rig.git(&["switch", "-q", LOOP]);
    let pre = rig.change(
        "README.md",
        "the loop's readme\n",
        "docs: the loop's readme",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.refused("conflict");
    assert!(
        line.get("merge_commit").is_none(),
        "no merge commit exists: {}",
        seen.raw
    );
    assert_eq!(rig.rev("HEAD"), pre, "the merge is aborted");
    assert_eq!(rig.status(), "");
    assert!(!rig.git_ok(&["rev-parse", "--verify", "--quiet", "MERGE_HEAD"]));
    assert!(seen.trace.contains(&"merge --abort".to_owned()));

    // A conflict in a log that is not a pure append is refused by dev/merge-logs, and
    // the merge is aborted.
    let rig = StepRig::new("land-merge-logs");
    let branch = a_round(&rig);
    let reworded = |how: &str| {
        format!("# Decisions\n\nThe log.\n\n## 2026-01-01 — the base\n\nThe base entry, {how}.\n")
    };
    rig.change(
        "DECISIONS.md",
        &reworded("reworded by the round"),
        "docs: a rewording",
    );
    rig.git(&["switch", "-q", LOOP]);
    let pre = rig.change(
        "DECISIONS.md",
        &reworded("reworded on the loop"),
        "docs: another",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&land(&branch, &product, &logs));
    seen.refused("merge-logs");
    assert_eq!(rig.rev("HEAD"), pre);
    assert_eq!(rig.status(), "");

    // A dev/merge-logs that says it resolved and left the path conflicted: the list said
    // "must print nothing" and named no abort, so the merge is left in progress.
    let rig = StepRig::new("land-unresolved");
    rig.change(
        MERGE_LOGS,
        "#!/bin/sh\nexit 0\n",
        "chore: a merge-logs that resolves nothing",
    );
    let branch = a_round(&rig);
    both_add_a_decision(&rig, &branch, LOOP);
    rig.git(&["push", "-q", "origin", LOOP]);
    rig.step(&land(&branch, &product, &logs))
        .refused("unresolved");
    assert!(
        rig.git_ok(&["rev-parse", "--verify", "--quiet", "MERGE_HEAD"]),
        "the merge is still in progress, as the step left it"
    );
}

#[test]
fn land_refuses_a_merge_whose_tree_is_not_the_rounds_and_undoes_nothing() {
    let rig = StepRig::new("land-differs");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    // The loop branch took a product change while the round was out: the merge is clean,
    // and what it holds on the product paths is not what the round's audit read.
    rig.git(&["switch", "-q", LOOP]);
    let pre = rig.change("crates/b.txt", "b\n", "chore: a direct change");
    rig.git(&["push", "-q", "origin", LOOP]);
    let round_head = rig.rev(&branch);

    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.refused("merge-differs");
    let merge = rig.rev("HEAD");
    assert_eq!(
        line["merge_commit"],
        merge.as_str(),
        "the merge commit is reported"
    );
    assert_eq!(line["moved_outside"], json!(["crates/b.txt"]));
    assert_eq!(
        rig.git(&["rev-list", "--parents", "-n", "1", "HEAD"]),
        format!("{merge} {pre} {round_head}"),
        "nothing is undone: the merge is on the local branch"
    );
    assert_eq!(rig.remote(LOOP), Some(pre), "and nothing is pushed");
    assert!(!seen.trace.iter().any(|call| call.starts_with("push ")));
}

#[test]
fn land_refuses_where_its_list_halted_before_the_merge() {
    let (product, logs) = (product(), logs());

    // The tree: clean, untracked files included — a report is not admitted here.
    let rig = StepRig::new("land-dirty");
    let branch = a_round(&rig);
    rig.write(
        &format!("{RUN_DIR}/r1/reports/fix/c1/x.a1.md"),
        "a report\n",
    );
    let seen = rig.step(&land(&branch, &product, &logs));
    seen.refused("dirty");
    assert_eq!(rig.branch(), branch, "nothing was switched");
    assert_eq!(seen.trace[0], "status --porcelain --untracked-files=all");

    // The pushed loop branch is ahead.
    let rig = StepRig::new("land-ahead");
    let branch = a_round(&rig);
    let pre = rig.rev(LOOP);
    rig.pushed_by_another(LOOP, &format!("{RUN_DIR}/note.md"), "a record\n");
    let seen = rig.step(&land(&branch, &product, &logs));
    seen.refused("remote-ahead");
    assert_eq!(rig.rev(LOOP), pre, "nothing was merged");
    assert_eq!(
        rig.branch(),
        LOOP,
        "the step had switched, and leaves everything as it is"
    );

    // A command that fails where the list expects none to: a round's branch that is not
    // there.
    let rig = StepRig::new("land-no-branch");
    let seen = rig.step(&land("fix/rc24-r9", &product, &logs));
    let line = seen.refused("git");
    assert!(
        line["halt"]["evidence"]
            .as_str()
            .is_some_and(|shown| shown.contains("`git rev-parse fix/rc24-r9` exited")),
        "the command and what it printed are the evidence: {}",
        seen.raw
    );
}

#[test]
fn a_landing_that_halted_after_its_merge_is_answered_landed_before_as_the_list_did() {
    // PRESERVED ON PURPOSE — the harness review's H5. The land step's list stopped at
    // "the round landed before" without checking the merge and without pushing; so does
    // the act. A later task of the repair turns the last three assertions.
    let rig = StepRig::new("land-halted");
    let (product, logs) = (product(), logs());
    let branch = a_round(&rig);
    let pre = rig.rev(LOOP);
    rig.remote_hook("pre-receive", "exit 1");
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.refused("push-rejected");
    let merge = rig.rev(LOOP);
    assert_ne!(merge, pre);
    assert_eq!(
        line["merge_commit"],
        merge.as_str(),
        "a land that halts after its merge still reports the merge commit"
    );
    assert_eq!(rig.remote(LOOP), Some(pre.clone()));

    fs::remove_file(rig.origin.join("hooks/pre-receive")).expect("lift the remote's refusal");
    let seen = rig.step(&land(&branch, &product, &logs));
    let line = seen.done("land", "landed-before");
    assert_eq!(line["head"], merge.as_str());
    assert!(line.get("remote_head").is_none(), "no remote is read");
    assert_eq!(
        rig.remote(LOOP),
        Some(pre),
        "and the merge is still not pushed"
    );
}

// ---------------------------------------------------------------------------
// round-commits · carry
// ---------------------------------------------------------------------------

#[test]
fn round_commits_lists_the_rounds_commits_and_which_touch_the_runs_directory() {
    let rig = StepRig::new("round-commits");
    let branch = rig.round(1);
    let fix = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let record = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    rig.write("crates/a.txt", "fixed twice\n");
    let mixed = rig.change(
        &format!("{RUN_DIR}/r1/note.md"),
        "n\n",
        "chore: both at once",
    );
    let commits = [
        "round-commits",
        "--loop",
        LOOP,
        "--branch",
        branch.as_str(),
        "--run-dir",
        RUN_DIR,
    ];

    let seen = rig.step(&commits);
    let line = seen.done("round-commits", "listed");
    assert_eq!(line["all"], json!([fix, record, mixed]));
    assert_eq!(line["inside"], json!([record, mixed]));
    assert_eq!(line["outside"], json!([fix, mixed]));
    let between = format!("{LOOP}..{branch}");
    assert_eq!(
        seen.trace,
        lines(&[
            &format!("rev-list --merges --count {between}"),
            &format!("log --reverse --format=%H {between}"),
            &format!("log --reverse --format=%H {between} -- {RUN_DIR}"),
            &format!("log --reverse --format=%H {between} -- . :(exclude){RUN_DIR}"),
        ])
    );

    // A merge among a round's commits is refused: no carry-over can take it.
    rig.git(&["switch", "-q", "-c", "work/side", LOOP]);
    rig.change("notes.md", "x\n", "docs: a side note");
    rig.git(&["switch", "-q", &branch]);
    rig.git(&["merge", "-q", "--no-ff", "--no-edit", "work/side"]);
    rig.step(&commits).refused("merge-in-round");
}

fn carry<'a>(product: &'a [String], shas: &[&'a str]) -> Vec<&'a str> {
    let mut args = with(
        &["carry", "--loop", LOOP, "--run-dir", RUN_DIR],
        &[("--product", product)],
    );
    args.push("--");
    args.extend_from_slice(shas);
    args
}

#[test]
fn carry_takes_a_dropped_rounds_record_commits_over_to_the_loop_branch() {
    let rig = StepRig::new("carry");
    let product = product();
    let branch = rig.round(1);
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let first = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    let second = rig.change(
        &format!("{RUN_DIR}/r1/drop.md"),
        "d\n",
        "docs(record): dropped",
    );
    let pre = rig.rev(LOOP);
    let held = rig.remote_heads();

    let seen = rig.step(&carry(&product, &[first.as_str(), second.as_str()]));
    let line = seen.done("carry", "carried");
    let made: Vec<String> = rig
        .git(&["log", "--reverse", "--format=%H", &format!("{pre}..{LOOP}")])
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(made.len(), 2);
    assert_eq!(
        line["picked"],
        json!([{"from": first, "to": made[0]}, {"from": second, "to": made[1]}])
    );
    assert_eq!(line["branch"], LOOP);
    assert_eq!(line["head"], made[1].as_str());
    assert_eq!(line["remote_head"], made[1].as_str());
    assert_eq!(rig.remote(LOOP), Some(made[1].clone()));
    assert_eq!(
        rig.read("crates/a.txt"),
        "a\n",
        "the round's fix is not carried"
    );
    assert!(
        rig.git(&["log", "-1", "--format=%B"])
            .contains(&format!("(cherry picked from commit {second})")),
        "a carried commit names the commit it was picked from"
    );
    assert_eq!(
        rig.rev(&branch),
        second,
        "the round's branch is left as it is"
    );
    assert_eq!(
        seen.trace,
        lines(&[
            "status --porcelain --untracked-files=all",
            &format!("switch {LOOP}"),
            "rev-parse HEAD",
            &format!("ls-remote --exit-code --heads origin {LOOP}"),
            &format!("fetch origin {LOOP}"),
            &format!("merge-base --is-ancestor origin/{LOOP} {LOOP}"),
            &format!("cherry-pick -x {first}"),
            &format!("cherry-pick -x {second}"),
            &format!("rev-list --count {pre}..HEAD"),
            &format!("diff --name-only {pre} HEAD"),
            &format!("diff --quiet {pre} HEAD -- {}", product.join(" ")),
        ])
        .into_iter()
        .chain(push_calls(LOOP, &made[1], &held))
        .chain(lines(&[
            "branch --show-current",
            "rev-parse HEAD",
            &format!("log --reverse --format=%H {pre}..HEAD"),
        ]))
        .collect::<Vec<_>>(),
        "the commands of the carry step's list, in its order"
    );
}

#[test]
fn carry_refuses_where_its_list_halted() {
    let product = product();

    // A commit that changed a path outside the run's directory is carried, found, and
    // left: nothing is undone, and nothing is pushed.
    let rig = StepRig::new("carry-differs");
    rig.round(1);
    let fix = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let pre = rig.rev(LOOP);
    let seen = rig.step(&carry(&product, &[fix.as_str()]));
    seen.refused("carry-differs");
    assert_eq!(
        rig.git(&["rev-list", "--count", &format!("{pre}..{LOOP}")]),
        "1"
    );
    assert_eq!(rig.remote(LOOP), Some(pre));

    // The list's third check — no product path changed — is its own: a product path that
    // lies inside the run's directory passes the check before it and is caught here.
    let rig = StepRig::new("carry-product");
    rig.round(1);
    let record = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    let inside = vec![format!("{RUN_DIR}/r1")];
    let seen = rig.step(&carry(&inside, &[record.as_str()]));
    assert!(
        seen.refused("carry-differs")["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains("a product path")),
        "the product paths are held on their own: {}",
        seen.raw
    );

    // A pick that stops is aborted; the commits carried before it stay.
    let rig = StepRig::new("carry-stopped");
    rig.round(1);
    let first = rig.change(
        &format!("{RUN_DIR}/r1/round.md"),
        "r\n",
        "docs(record): a cycle",
    );
    let second = rig.change(
        &format!("{RUN_DIR}/r1/note.md"),
        "theirs\n",
        "docs(record): a note",
    );
    rig.git(&["switch", "-q", LOOP]);
    rig.change(
        &format!("{RUN_DIR}/r1/note.md"),
        "ours\n",
        "docs(record): another note",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    let pre = rig.rev(LOOP);
    let seen = rig.step(&carry(&product, &[first.as_str(), second.as_str()]));
    seen.refused("pick-stopped");
    assert_eq!(
        rig.git(&["rev-list", "--count", &format!("{pre}..{LOOP}")]),
        "1"
    );
    assert_eq!(rig.status(), "", "the stopped pick is aborted");
    assert!(!rig.git_ok(&["rev-parse", "--verify", "--quiet", "CHERRY_PICK_HEAD"]));
    assert_eq!(rig.remote(LOOP), Some(pre));

    // A tree that is not clean carries nothing; and a commit is named by its full sha.
    let rig = StepRig::new("carry-dirty");
    let head = rig.rev("HEAD");
    rig.write("notes.txt", "x\n");
    rig.step(&carry(&product, &[head.as_str()]))
        .refused("dirty");
    let seen = rig.step(&carry(&product, &[&head[..12]]));
    seen.refused("usage");
    assert!(seen.trace.is_empty());
}

// ---------------------------------------------------------------------------
// sync-main
// ---------------------------------------------------------------------------

fn sync(logs: &[String]) -> Vec<&str> {
    with(&["sync-main", "--loop", LOOP], &[("--log", logs)])
}

/// The four ways the sync act ends, each with the git commands it ran.
fn sync_runs() -> Vec<(&'static str, Vec<String>)> {
    let logs = logs();
    let mut runs = Vec::new();

    // The branch holds main already.
    let rig = StepRig::new("sync-up-to-date");
    let head = rig.rev("HEAD");
    let seen = rig.step(&sync(&logs));
    let line = seen.done("sync-main", "up-to-date");
    assert_eq!(line["head"], head.as_str());
    assert_eq!(line["origin_main"], rig.rev("origin/main").as_str());
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            "status --porcelain",
            "rev-parse HEAD",
            "fetch origin main",
            "rev-parse origin/main",
            &format!("merge-base --is-ancestor origin/main {LOOP}"),
            "rev-parse HEAD",
        ])
    );
    runs.push(("up to date", seen.trace));

    // main moved: a merge commit, and nothing pushed.
    let rig = StepRig::new("sync-merged");
    let pre = rig.rev("HEAD");
    let theirs = rig.pushed_by_another("main", "notes.md", "from main\n");
    let seen = rig.step(&sync(&logs));
    let line = seen.done("sync-main", "merged");
    let merge = rig.rev("HEAD");
    assert_eq!(line["head"], merge.as_str());
    assert_eq!(line["origin_main"], theirs.as_str());
    assert_eq!(line["resolved_logs"], json!([]));
    assert_eq!(
        rig.git(&["rev-list", "--parents", "-n", "1", "HEAD"]),
        format!("{merge} {pre} {theirs}")
    );
    assert_eq!(rig.remote(LOOP), Some(pre), "the sync pushes nothing");
    assert_eq!(rig.remote("main"), Some(theirs));
    assert_eq!(
        seen.trace,
        lines(&[
            "branch --show-current",
            "status --porcelain",
            "rev-parse HEAD",
            "fetch origin main",
            "rev-parse origin/main",
            &format!("merge-base --is-ancestor origin/main {LOOP}"),
            "merge --no-ff --no-edit origin/main",
            "rev-list --parents -n 1 HEAD",
            "rev-parse origin/main",
            "status --porcelain",
            "rev-parse HEAD",
        ]),
        "the commands of the sync step's list, in its order"
    );
    runs.push(("merged", seen.trace));

    // Both appended to the decisions log.
    let rig = StepRig::new("sync-logs");
    let entry = |date: &str, who: &str| {
        format!(
            "# Decisions\n\nThe log.\n\n## {date} — {who}\n\nAn entry of {who}.\n\n## 2026-01-01 — the base\n\nThe base entry.\n"
        )
    };
    rig.change(
        "DECISIONS.md",
        &entry("2026-02-02", "the run"),
        "docs: a decision of the run's",
    );
    rig.pushed_by_another("main", "DECISIONS.md", &entry("2026-02-01", "main"));
    let seen = rig.step(&sync(&logs));
    assert_eq!(
        seen.done("sync-main", "merged")["resolved_logs"],
        json!(["DECISIONS.md"])
    );
    let log = rig.read("DECISIONS.md");
    assert!(log.contains("An entry of the run.") && log.contains("An entry of main."));
    runs.push(("a log resolved", seen.trace));

    // A conflict outside the logs: aborted, and the human's.
    let rig = StepRig::new("sync-conflict");
    let pre = rig.change("README.md", "the run's readme\n", "docs: the run's readme");
    rig.pushed_by_another("main", "README.md", "main's readme\n");
    let seen = rig.step(&sync(&logs));
    seen.refused("conflict");
    assert_eq!(rig.rev("HEAD"), pre);
    assert_eq!(rig.status(), "");
    runs.push(("a conflict", seen.trace));

    runs
}

#[test]
fn sync_main_merges_origin_main_into_the_loop_branch_and_pushes_nothing() {
    sync_runs();

    // The branch and the tree the step starts from.
    let logs = logs();
    let rig = StepRig::new("sync-branch");
    rig.round(1);
    rig.step(&sync(&logs)).refused("wrong-branch");
    rig.git(&["switch", "-q", LOOP]);
    rig.write("notes.txt", "x\n");
    rig.step(&sync(&logs)).refused("dirty");
}

/// The git commands a harness's step lists: every `` `git …` `` between backticks in the
/// numbered lines of its prompt function, the branch's variable filled in.
fn listed_git_commands(
    harness: &str,
    step: &str,
    variable: &str,
    branch: &str,
) -> BTreeSet<String> {
    let at = harness
        .find(&format!("\nfunction {step}("))
        .unwrap_or_else(|| panic!("the harness has the step `{step}`"));
    let body = &harness[at..];
    let body = &body[..body.find("\n}\n").expect("the function closes")];
    let mut listed = BTreeSet::new();
    for line in body.lines().filter(|line| {
        let line = line.trim_start();
        line.starts_with('\'') && line[1..].starts_with(|c: char| c.is_ascii_digit())
    }) {
        for command in line.split('`').skip(1).step_by(2) {
            if let Some(command) = command.strip_prefix("git ") {
                listed.insert(command.replace(&format!("' + {variable} + '"), branch));
            }
        }
    }
    listed
}

#[test]
fn the_sync_act_runs_the_build_harnesss_sync_step() {
    // The close's sync is one step, whichever harness returns it. The build harness lists
    // it as commands; the stabilization harness hands it to the tool. So the two are held
    // together by what the act RAN, over the four ways it ends: every git command it ran
    // is one the build harness's step lists, and every command that step lists was run.
    let listed = listed_git_commands(&read(BUILD_HARNESS), "syncMainPrompt", "m", LOOP);
    assert!(
        listed.len() >= 10 && listed.contains("merge --no-ff --no-edit origin/main"),
        "the scan found the build harness's sync step: {listed:#?}"
    );
    let ran: BTreeSet<String> = sync_runs()
        .into_iter()
        .flat_map(|(_, trace)| trace)
        .collect();
    assert_eq!(
        ran, listed,
        "the git commands the sync act ran (left) are the commands {BUILD_HARNESS}'s sync step lists (right)"
    );
}

// ---------------------------------------------------------------------------
// What is vetted, and where: the report check, the commit, every push
// ---------------------------------------------------------------------------

/// The reporters of the stage a [`Stage`] stands at its record: the attempt's marker, the
/// scope step, and one reviewer.
const LAUNCHED: [&str; 3] = ["attempt", "scope", "review-setup"];
const SUBJECT: &str = "docs(record): rc24 r1 — the test stage's record";

/// A `test` stage of the rig's run, stood where its record step begins — through the two
/// scripts, as a stage stands there: the opening's facts committed and pushed, attempt 1 of
/// round 1 begun, the round's scope and two reports written, the candidate's gate in a
/// file, and the batch of the record composed and not applied.
struct Stage {
    rig: StepRig,
    gate: String,
    batch: Value,
}

impl Stage {
    fn new(label: &str) -> Self {
        let rig = StepRig::new(label);
        let scratch = rig.scratch.display().to_string();
        rig.wrote(
            &format!("run-set --run {RUN}"),
            &json!({"stop": "at-the-bound", "rounds": 3, "previous": "1.0.0-rc.24",
                    "previous-commit": "9".repeat(40), "scope": "delta",
                    "clauses": ["clause-a"]})
            .to_string(),
        );
        rig.wrote(
            &format!("item-set --run {RUN}"),
            &json!([
                {"item": "gate", "kind": "check", "clause": "clause-a",
                 "runs": "every-candidate", "brief": "the candidate's gate"},
                {"item": "review-setup", "kind": "review-row", "clause": "clause-a",
                 "runs": "in-scope", "doors": ["jigc setup"], "brief": "the setup door"},
            ])
            .to_string(),
        );
        rig.commit("docs(record): the opening's facts");
        rig.git(&["push", "-q", "origin", LOOP]);
        let candidate = rig.rev("HEAD");
        rig.step(&[
            "begin",
            "--run",
            RUN,
            "--round",
            "1",
            "--stage",
            "test",
            "--attempt",
            "1",
            "--reporter",
            "attempt",
            "--commit",
            &candidate,
            "--scratch",
            &scratch,
        ])
        .done("begin", "begun");
        rig.wrote(
            &format!("scope-set --run {RUN} --round 1"),
            &json!({"included": [{"door": "jigc setup", "registry": "verbs",
                                  "derivation": "the change reaches it"}],
                    "excluded": []})
            .to_string(),
        );
        for reporter in &LAUNCHED[1..] {
            rig.wrote(
                &format!(
                    "report --run {RUN} --round 1 --stage test --reporter {reporter} --attempt 1"
                ),
                REPORT,
            );
        }
        let gate = format!("{scratch}/gate.txt");
        fs::write(
            &gate,
            "gate: mode   full, keep-going\ntests   passed=10 failed=0  (over 3 test binaries)\nGATE: PASS\n",
        )
        .expect("write the gate's output");
        let call = |argv: &[&str], stdin: Value| json!({"argv": argv, "stdin": stdin.to_string()});
        let mut check = vec![
            "check-reports",
            "--run",
            RUN,
            "--round",
            "1",
            "--stage",
            "test",
            "--attempt",
            "1",
            "--",
        ];
        check.extend(LAUNCHED);
        let batch = json!([
            {"argv": check},
            {"argv": ["gate-set", "--run", RUN, "--round", "1", "--commit", candidate,
                      "--summary", gate]},
            call(
                &["result-set", "--run", RUN, "--round", "1", "--commit", &candidate],
                json!([{"item": "gate", "outcome": "green"},
                       {"item": "review-setup", "outcome": "green"}]),
            ),
            call(
                &["ledger-add", "--run", RUN],
                json!([{"key": "f-1", "doctype": "jigc-feedback", "round": 1,
                        "source": "review-setup", "door": "jigc setup",
                        "clause": "clause-a", "repro": "the report"}]),
            ),
            call(
                &["triage-set", "--run", RUN, "--round", "1"],
                json!([{"key": "f-1", "grade": "breaks", "verdict": "refuted"}]),
            ),
            call(
                &["round-set", "--run", RUN, "--round", "1"],
                json!({"candidate": candidate, "binary": "b".repeat(64)}),
            ),
            {"argv": ["check-ledger", "--run", RUN, "--", "f-1"]},
        ]);
        Stage { rig, gate, batch }
    }

    /// The report of one of the stage's reporters, repository-relative.
    fn report(reporter: &str) -> String {
        format!("{RUN_DIR}/r1/reports/test/{reporter}.a1.md")
    }

    /// The record's batch, applied under `subject`: the tables written, and no commit.
    fn apply(&self, subject: &str) {
        let kept = self.rig.scratch.join("subject");
        fs::write(&kept, subject).expect("keep the subject");
        self.rig.wrote(
            &format!(
                "apply --run {RUN} --round 1 --subject \"$(cat '{}')\"",
                kept.display()
            ),
            &self.batch.to_string(),
        );
    }

    /// The record's commit step, as the harness asks for it — and, where it moves a file,
    /// under the rig's scratch root.
    fn record(&self) -> Stepped {
        self.record_in(&[])
    }

    fn record_in(&self, env: &[(&str, &str)]) -> Stepped {
        let scratch = self.rig.scratch.display().to_string();
        self.rig.step_in(
            env,
            &[
                "record",
                "--branch",
                LOOP,
                "--run-dir",
                RUN_DIR,
                "--gate",
                &self.gate,
                "--calls",
                "7",
                "--checks",
                "2",
                "--scratch",
                &scratch,
            ],
        )
    }

    fn check_in(&self, env: &[(&str, &str)], scratch: bool) -> Stepped {
        let root = self.rig.scratch.display().to_string();
        let mut args = vec![
            "check-reports",
            "--run",
            RUN,
            "--round",
            "1",
            "--stage",
            "test",
            "--attempt",
            "1",
        ];
        if scratch {
            args.extend(["--scratch", root.as_str()]);
        }
        args.push("--");
        args.extend(LAUNCHED);
        self.rig.step_in(env, &args)
    }

    /// The report check, as the harness asks for it today: with no scratch root.
    fn check(&self) -> Stepped {
        self.check_in(&[], false)
    }

    /// What a stage would start from: the git state's line.
    fn git_state(&self) -> Stepped {
        self.rig.step(&git_state("test", &product()))
    }

    /// What the run's state hands the `test` stage.
    fn position(&self) -> Value {
        let state: Value = serde_json::from_str(&self.rig.wrote(&format!("state --run {RUN}"), ""))
            .expect("the state document");
        state["position"]["test"].clone()
    }

    /// The tree is one a stage starts from, with no batch pending, and the state hands the
    /// stage `attempt`.
    fn starts_again_as(&self, attempt: u32, what: &str) {
        let state = self.git_state();
        assert_eq!(
            state.done("git-state", "ready")["pending"],
            Value::Null,
            "{what}: a tree a stage starts from, and no batch: {}",
            state.raw
        );
        assert_eq!(
            self.position(),
            json!({"round": 1, "attempt": attempt}),
            "{what}: the state hands out the next attempt"
        );
    }
}

/// What a hand can leave in a file of the run that no writer of the record script would
/// have: the word the writer refuses it with, a line that carries it, and the needle.
fn unwritable(rig: &StepRig) -> Vec<(&'static str, String, String)> {
    let home = format!("{}/probe", rig.home.display());
    vec![
        (
            "hygiene",
            format!("It met {DENY_TERM} there."),
            DENY_TERM.to_owned(),
        ),
        (
            "hygiene",
            format!("The environment held {STUB_SECRET}."),
            STUB_SECRET.to_owned(),
        ),
        ("host-path", format!("It ran in {home}."), home),
        // The fence's refusal names the command it reads; what it does not name is the rest.
        (
            "fence",
            format!("{INSTALL_COMMAND} --locked"),
            "--locked".to_owned(),
        ),
    ]
}

/// A report whose body is `line`.
fn report_saying(line: &str) -> String {
    format!("# a report\n\n{line}\n\n<!-- end of report -->\n")
}

/// **A file at a report's path that the record script would not have written is no
/// report** (the re-review's `R1`, at the first boundary; the plan review's `B3`): the
/// report check vets every pending written-once file first, the file leaves the tree, the
/// check names it, and its reporter has left none — which a stage has a row for, so the
/// stage goes on. It is an answer, never a refusal.
#[test]
fn a_report_the_script_would_not_have_written_is_no_report() {
    let stage = Stage::new("vet-check");
    let rig = &stage.rig;
    let path = Stage::report("review-setup");

    // MUST NOT REFUSE: an ordinary stage's reports; and a report a hand wrote that is what
    // the script would have written — the public placeholders themselves among its text.
    let line = stage.check();
    let line = line.done("check-reports", "checked");
    assert_eq!(
        json!([line["check"]["ok"], line["aside"]]),
        json!([true, []])
    );
    rig.write(
        &path,
        &report_saying("It ran in <scratch>/probe, in <tmp>/x and in ~/y, from `.`."),
    );
    let line = stage.check();
    let line = line.done("check-reports", "checked");
    assert_eq!(
        json!([line["check"]["ok"], line["aside"]]),
        json!([true, []])
    );

    let mut cases: Vec<(&str, String, String)> = unwritable(rig)
        .into_iter()
        .map(|(word, line, needle)| (word, report_saying(&line), needle))
        .collect();
    cases.push(("truncated", "a report\n".to_owned(), String::new()));
    for (n, (word, text, needle)) in cases.into_iter().enumerate() {
        rig.write(&path, &text);
        // The harness names no scratch root yet (the second repair's `K3`): the record
        // script mints a directory. With one named, the file lies under it.
        let named = n % 2 == 0;
        let seen = stage.check_in(&[], named);
        let line = seen.done("check-reports", "checked");
        let what = format!("`{word}`");
        assert_eq!(
            json!([
                line["check"]["ok"],
                line["check"]["missing"],
                line["check"]["extra"]
            ]),
            json!([false, ["review-setup"], []]),
            "{what}: its reporter has left no report: {}",
            seen.raw
        );
        assert_eq!(
            json!([
                line["aside"][0]["path"],
                line["aside"][0]["why"],
                line["aside"].as_array().map(Vec::len)
            ]),
            json!([path, word, 1]),
            "{what}: the check names the file, and no other: {}",
            seen.raw
        );
        let to = PathBuf::from(line["aside"][0]["to"].as_str().expect("where it went"));
        assert_eq!(
            fs::read_to_string(&to).expect("the file, where it was moved"),
            text,
            "{what}: the file is kept, outside the tree"
        );
        assert!(
            !to.starts_with(&rig.root)
                && (!named || to.starts_with(rig.scratch.join("refused")))
                && !rig.root.join(&path).exists(),
            "{what}: the tree does not hold it: {}",
            to.display()
        );
        assert!(
            needle.is_empty() || !seen.raw.contains(&needle),
            "{what}: named by where, never by what: {}",
            seen.raw
        );
        stage.git_state().done("git-state", "ready");
    }

    // A file of AN EARLIER ATTEMPT that no commit holds is vetted too — it would ride the
    // next record's commit — and so is the round's scope.
    rig.write(&path, REPORT);
    let earlier = format!("{RUN_DIR}/r1/reports/test/review-setup.a7.md");
    let scope = format!("{RUN_DIR}/r1/scope.md");
    rig.write(&earlier, &report_saying(&format!("It met {DENY_TERM}.")));
    rig.write(
        &scope,
        &format!("{}\nIt met {DENY_TERM}.\n", rig.read(&scope)),
    );
    let seen = stage.check();
    let line = seen.done("check-reports", "checked");
    let mut moved: Vec<&str> = line["aside"]
        .as_array()
        .expect("what was moved")
        .iter()
        .map(|found| found["path"].as_str().expect("a path"))
        .collect();
    moved.sort_unstable();
    assert_eq!(
        (line["check"]["ok"].clone(), moved),
        (json!(true), vec![earlier.as_str(), scope.as_str()]),
        "{}",
        seen.raw
    );
    assert!(!rig.root.join(&earlier).exists() && !rig.root.join(&scope).exists());
}

/// What the commit step is handed something no writer would have written in.
#[derive(Clone, Copy, PartialEq)]
enum Unwritten {
    Report,
    FixReport,
    Scope,
    Subject,
}

/// **Nothing is committed that the record script would not have written** (`R1` and `R13`,
/// at the second boundary): each refusal a writer has × a `test` report, a fix cycle's
/// report, a round's scope, and the subject. The commit step stages the pending paths,
/// vets what it staged, and on a hit commits nothing.
#[test]
fn nothing_is_committed_that_the_record_script_would_not_have_written() {
    std::thread::scope(|threads| {
        for (n, target) in [
            Unwritten::Report,
            Unwritten::FixReport,
            Unwritten::Scope,
            Unwritten::Subject,
        ]
        .into_iter()
        .enumerate()
        {
            threads.spawn(move || {
                let stage = Stage::new(&format!("vet-commit-{n}"));
                let rig = &stage.rig;
                let path = match target {
                    Unwritten::Report => Stage::report("review-setup"),
                    Unwritten::FixReport => {
                        format!("{RUN_DIR}/r1/reports/fix/c1/audit-review.a1.md")
                    }
                    Unwritten::Scope => format!("{RUN_DIR}/r1/scope.md"),
                    Unwritten::Subject => String::new(),
                };
                let scope = rig.read(&format!("{RUN_DIR}/r1/scope.md"));
                let head = rig.rev("HEAD");
                // What is written: a line of the subject, or a file's whole text — and a
                // report may also lack its last line.
                let mut cases: Vec<(&str, String, String)> = unwritable(rig)
                    .into_iter()
                    .map(|(word, line, needle)| {
                        let text = if target == Unwritten::Subject {
                            line
                        } else {
                            report_saying(&line)
                        };
                        (word, text, needle)
                    })
                    .collect();
                if matches!(target, Unwritten::Report | Unwritten::FixReport) {
                    cases.push((
                        "truncated",
                        "# a report\n\nIt was cut off here\n".to_owned(),
                        "cut off here".to_owned(),
                    ));
                }
                for (word, text, needle) in cases {
                    let what = format!(
                        "`{word}` in {}",
                        if path.is_empty() {
                            "the subject"
                        } else {
                            path.as_str()
                        }
                    );
                    if target == Unwritten::Subject {
                        if word == "fence" {
                            continue;
                        }
                        stage.apply(&format!("docs(record): {text}"));
                    } else {
                        stage.apply(SUBJECT);
                        // Written by a hand, AFTER the batch's own check counted the file.
                        rig.write(&path, &text);
                    }
                    let seen = stage.record();
                    let said = seen.refused("unvetted");
                    assert_eq!(rig.rev("HEAD"), head, "{what}: nothing was committed");
                    assert!(
                        !rig.status().lines().any(|line| !line.starts_with("?? ")),
                        "{what}: nothing is left staged, and no table is left changed: {}",
                        rig.status()
                    );
                    assert!(
                        !seen.raw.contains(&needle),
                        "{what}: named by where, never by what: {}",
                        seen.raw
                    );
                    if target == Unwritten::Subject {
                        assert_eq!(
                            json!([said["vet"]["subject"]["why"], said["vet"]["refused"]]),
                            json!([word, []]),
                            "{what}: {}",
                            seen.raw
                        );
                    } else {
                        assert_eq!(
                            json!([
                                said["vet"]["refused"][0]["path"],
                                said["vet"]["refused"][0]["why"],
                                said["aside"][0]["path"]
                            ]),
                            json!([path, word, path]),
                            "{what}: {}",
                            seen.raw
                        );
                        assert!(
                            !rig.root.join(&path).exists(),
                            "{what}: the file left the tree"
                        );
                    }
                    assert_eq!(said["discarded"], true, "{what}: the batch is taken back");
                    stage.git_state().done("git-state", "ready");
                    // What the next case stands on again.
                    match target {
                        Unwritten::Report => rig.write(&path, REPORT),
                        Unwritten::Scope => rig.write(&path, &scope),
                        Unwritten::FixReport | Unwritten::Subject => {}
                    }
                }
                // MUST NOT REFUSE: the same stage, with nothing a writer would refuse — the
                // record is one commit of exactly the pending paths.
                stage.apply(SUBJECT);
                let line = stage.record();
                let line = line.done("record", "recorded");
                assert_eq!(rig.git(&["log", "-1", "--format=%s"]), SUBJECT);
                assert_eq!(line["commit"], rig.rev("HEAD").as_str());
                assert_eq!(rig.status(), "");
            });
        }
    });
}

/// **A vet hit at the commit leaves a tree the next attempt starts from** (the plan
/// review's `B3`): a report is written once, so a refusal that left the file and the batch
/// where they were would be met again by every later invocation. The step takes back what
/// it staged, the batch is discarded, the file leaves the tree — and the refusal's own
/// text names the read that answers for the tree. Driven: that read answers `ready`, the
/// state hands out the next attempt, and that attempt begins.
#[test]
fn a_vet_hit_at_the_commit_leaves_a_tree_the_next_attempt_starts_from() {
    let stage = Stage::new("vet-exit");
    let rig = &stage.rig;
    let path = Stage::report("review-setup");
    let tables = rig.read(&format!("{RUN_DIR}/run.md"));
    stage.apply(SUBJECT);
    // The re-review's own block: the text the script refused, written by a redirect.
    let text = report_saying(&format!(
        "ran in {}/probe and met {DENY_TERM}",
        rig.home.display()
    ));
    let (code, _, err) = rig.record_script(
        &format!(
            "{RECORD} report --run {RUN} --round 1 --stage test --reporter review-rename --attempt 1"
        ),
        &text,
    );
    assert!(
        code != 0 && err.starts_with("stabilize-record: refused "),
        "{err}"
    );
    rig.write(&path, &text);

    let seen = stage.record();
    let said = seen.refused("unvetted");
    let then = said["halt"]["recommendation"]
        .as_str()
        .expect("what is done about it");
    assert!(
        then.contains("`dev/stabilize-step git-state`") && then.contains("the next attempt"),
        "the refusal names the read that answers for the tree, and what follows: {then}"
    );
    let to = said["aside"][0]["to"]
        .as_str()
        .expect("where the file went");
    assert!(
        then.contains(to) && Path::new(to).starts_with(rig.scratch.join("refused")),
        "and where the file is: {then}"
    );
    assert_eq!(fs::read_to_string(to).expect("the moved file"), text);
    assert_eq!(
        rig.read(&format!("{RUN_DIR}/run.md")),
        tables,
        "the tables are as before the batch"
    );
    assert_eq!(
        rig.status().lines().collect::<Vec<_>>(),
        [
            format!("?? {}", Stage::report("attempt")),
            format!("?? {}", Stage::report("scope")),
            format!("?? {RUN_DIR}/r1/scope.md"),
        ],
        "what the attempt wrote and the script would have written is still pending"
    );
    stage.starts_again_as(2, "after the refusal");
    let scratch = rig.scratch.display().to_string();
    let head = rig.rev("HEAD");
    rig.step(&[
        "begin",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "2",
        "--reporter",
        "attempt",
        "--commit",
        &head,
        "--scratch",
        &scratch,
    ])
    .done("begin", "begun");
    // And nothing of it is on the way to the remote.
    let pushed = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(pushed.done("push", "ready")["vetted"], json!([]));
}

/// **A batch whose report is gone is not committed** (`B3`): the batch keeps the result of
/// its report check, and the commit step is held to that result. So the check is held to
/// the disk again where the commit is made — a report it counted that is gone, or that
/// holds other bytes, is no record of that attempt.
#[test]
fn a_batch_whose_report_is_gone_is_not_committed() {
    for (n, (what, change)) in [
        ("gone", None),
        ("changed", Some(report_saying("Rewritten after the check."))),
    ]
    .into_iter()
    .enumerate()
    {
        let stage = Stage::new(&format!("vet-stale-{n}"));
        let rig = &stage.rig;
        let path = Stage::report("review-setup");
        let head = rig.rev("HEAD");
        stage.apply(SUBJECT);
        match &change {
            None => fs::remove_file(rig.root.join(&path)).expect("delete a report"),
            Some(text) => rig.write(&path, text),
        }
        let seen = stage.record();
        let said = seen.refused("unvetted");
        assert_eq!(
            json!([
                said["vet"]["stale"][0][what],
                said["vet"]["stale"][0]["attempt"],
                said["discarded"]
            ]),
            json!([["review-setup.a1.md"], 1, true]),
            "a report {what} since the batch's check: {}",
            seen.raw
        );
        assert_eq!(rig.rev("HEAD"), head, "{what}: nothing was committed");
        // A report that changed into what a writer would have left is a report still: it
        // stays, and the next attempt's record holds it.
        assert_eq!(rig.root.join(&path).exists(), change.is_some());
        stage.starts_again_as(2, what);
    }
}

/// **What is committed is what was vetted**: the commit step stages the pending paths
/// before it vets them, and holds the tree to the index once more when the vet is back. A
/// path that changed under the step — here by a `git` that rewrites the round's scope the
/// moment it is staged, into another text a writer could have left, so that nothing but
/// this look can tell — is not committed, whatever it changed into.
#[test]
fn a_path_that_changes_under_the_commit_step_is_not_committed() {
    let stage = Stage::new("vet-moved");
    let rig = &stage.rig;
    let path = format!("{RUN_DIR}/r1/scope.md");
    let head = rig.rev("HEAD");
    stage.apply(SUBJECT);
    let later = rig.dir().join("later.md");
    let rewritten = rig.read(&path).replace(
        "the change reaches it",
        "the change reaches it, as said later",
    );
    assert_ne!(rewritten, rig.read(&path));
    fs::write(&later, &rewritten).expect("the later text");
    let quoted = |path: &Path| path.display().to_string().replace('\'', "'\\''");
    rig.on_path(
        "git",
        &format!(
            "#!/bin/sh\n'{git}' \"$@\"\ncode=$?\nif [ \"$1\" = add ]; then cat '{later}' >'{report}'; fi\nexit $code\n",
            git = quoted(&real_git()),
            later = quoted(&later),
            report = quoted(&rig.root.join(&path)),
        ),
    );
    let seen = stage.record();
    let said = seen.refused("unvetted");
    assert!(
        said["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(&format!("changed while the step ran: {path}"))),
        "the path that changed is named: {}",
        seen.raw
    );
    assert_eq!(rig.rev("HEAD"), head, "nothing was committed");
    assert!(
        !rig.status().lines().any(|line| !line.starts_with("?? ")),
        "and nothing is left staged: {}",
        rig.status()
    );
    assert_eq!(
        json!([said["vet"]["ok"], said["aside"]]),
        json!([true, []]),
        "the vet found nothing: only the second look did: {}",
        seen.raw
    );
    stage.starts_again_as(2, "a path that changed under the step");
}

/// **A scanner that cannot run is a refusal at every boundary, never a pass** — and it has
/// its exit: the step left everything as it found it, and the same step, asked again once
/// the scanner runs, does what it was asked.
#[test]
fn a_scanner_that_cannot_run_is_a_refusal_at_every_boundary() {
    let stage = Stage::new("vet-did-not-run");
    let rig = &stage.rig;
    let nowhere = rig.dir().join("no-denylist").display().to_string();
    let breaks: [(&str, [(&str, &str); 1]); 2] = [
        ("no denylist", [("JIGC_DENYLIST_FILE", nowhere.as_str())]),
        (
            "gitleaks fails on an error of its own",
            [("STUB_GITLEAKS", "crash")],
        ),
    ];

    // The report check: what is pending is what a writer left, and nothing says so.
    let pending = rig.status();
    for (why, env) in &breaks {
        let seen = stage.check_in(env, true);
        let said = seen.refused("did-not-run");
        assert!(
            rig.status() == pending && !rig.scratch.join("refused").exists(),
            "{why}: no report was vetted, and none was moved: {}",
            seen.raw
        );
        assert!(
            said["halt"]["recommendation"]
                .as_str()
                .is_some_and(|then| then.contains("the same step is asked for again")),
            "{why}: the refusal says what leaves it: {}",
            seen.raw
        );
    }
    assert_eq!(
        stage.check_in(&[], true).done("check-reports", "checked")["check"]["ok"],
        true,
        "with the scanner back, the same step vets and checks"
    );

    // The commit.
    stage.apply(SUBJECT);
    let head = rig.rev("HEAD");
    for (why, env) in &breaks {
        stage.record_in(env).refused("did-not-run");
        assert_eq!(rig.rev("HEAD"), head, "{why}: nothing was committed");
        let state = stage.git_state();
        assert_eq!(
            state.done("git-state", "ready")["pending"]["subject"],
            SUBJECT,
            "{why}: the batch stays applied, and nothing is left staged: {}",
            state.raw
        );
    }
    stage.record().done("record", "recorded");

    // The push.
    let recorded = rig.rev("HEAD");
    for (why, env) in &breaks {
        rig.step_in(env, &["push", "--branch", LOOP])
            .refused("did-not-run");
        assert_eq!(
            rig.remote(LOOP),
            Some(head.clone()),
            "{why}: nothing was pushed"
        );
    }
    assert_eq!(
        rig.step(&["push", "--branch", LOOP]).done("push", "ready")["vetted"],
        json!([recorded])
    );
    // MUST NOT REFUSE: a push that publishes nothing needs no scanner.
    rig.step_in(&breaks[0].1, &["push", "--branch", LOOP])
        .done("push", "ready");
}

/// A place a vet of a range refuses: the path — none for the commit's message — and the
/// word a writer would have refused it with.
type Place<'a> = (Option<&'a str>, &'a str);

/// The command a refusal's text names, taken out of that text: the code span that opens
/// with `opens`.
fn named_command<'a>(text: &'a str, opens: &str) -> &'a str {
    let at = text
        .find(&format!("`{opens}"))
        .unwrap_or_else(|| panic!("the text names `{opens} …`: {text}"));
    let command = &text[at + 1..];
    &command[..command.find('`').expect("the code span closes")]
}

/// **Nothing is published that was not vetted** (the plan review's `B2`): the step tool
/// makes every push of a run, and a commit is whoever made it. So every push — of `push`,
/// of a landing, of a carried record — is preceded by the vet of every commit the remote
/// does not hold yet: the two scanners CI runs over a push, and, for a commit that changes
/// a file of the run, what a writer of the record script holds its text to. A hit is not
/// pushed, and it is the human's; the refusal names the read that names every commit and
/// place again, which is driven here as the refusal spells it.
#[test]
fn nothing_is_published_that_was_not_vetted() {
    let stage = Stage::new("vet-push");
    let rig = &stage.rig;
    let home = format!("{}/probe", rig.home.display());

    // MUST NOT REFUSE: a tuning commit outside the run's directory that the remote lacks —
    // source code names `/tmp`, and so may its message: the placeholder rule is the
    // record's, and what a file outside the run is held to is the two scanners.
    rig.write(
        "README.md",
        &format!("a scratch file under /tmp/x, and {home}\n"),
    );
    rig.git(&["add", "--", "README.md"]);
    rig.git(&[
        "commit",
        "-q",
        "-m",
        &format!("build(dev): a tuning commit\n\nIt reads /tmp/x and {home}."),
    ]);
    let tuning = rig.rev("HEAD");
    let line = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(line.done("push", "ready")["vetted"], json!([tuning]));
    assert_eq!(rig.remote(LOOP), Some(tuning.clone()));
    // MUST NOT REFUSE: a push of what is already there.
    let again = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(again.done("push", "ready")["vetted"], json!([]));
    // MUST NOT REFUSE: an ordinary round's record.
    stage.apply(SUBJECT);
    stage.record().done("record", "recorded");
    let recorded = rig.rev("HEAD");
    let line = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(line.done("push", "ready")["vetted"], json!([recorded]));

    // THE PLAN REVIEW'S BLOCK: a report with the denylisted term and a host path,
    // committed under the run's directory by plain git — and each alone, in each place a
    // commit publishes: a file of the run, the subject, the body.
    let by_hand = format!("{RUN_DIR}/r1/reports/test/handmade.a1.md");
    let clean = report_saying("Nothing found.");
    let file = Some(by_hand.as_str());
    let cases: Vec<(&str, String, String, Vec<Place>)> = vec![
        (
            "the block: the term and a host path, in the report and in the subject",
            report_saying(&format!("ran in {home} and met {DENY_TERM}")),
            format!("docs(record): {DENY_TERM} by hand"),
            vec![(None, "hygiene"), (file, "hygiene"), (file, "host-path")],
        ),
        (
            "a host path alone, in a file of the run",
            report_saying(&format!("ran in {home}")),
            "docs(record): by hand".to_owned(),
            vec![(file, "host-path")],
        ),
        (
            "a report that does not end as one",
            "# a report\n\ncut off\n".to_owned(),
            "docs(record): by hand".to_owned(),
            vec![(file, "truncated")],
        ),
        (
            "the term in the body of the message alone",
            clean.clone(),
            format!("docs(record): by hand\n\nIt met {DENY_TERM}."),
            vec![(None, "hygiene")],
        ),
        (
            "a host path in the message of a commit that changes the run's record",
            clean.clone(),
            format!("docs(record): by hand\n\nWritten in {home}."),
            vec![(None, "host-path")],
        ),
        // WHAT A COMMIT SAYS IS HELD TO WHAT ITS FILES ARE HELD TO (the core review's `F6`):
        // gitleaks reads added lines and never a message, so a secret-shaped string in a
        // subject or a body passed every push.
        (
            "a secret-shaped string in the body of the message alone",
            clean.clone(),
            format!("docs(record): by hand\n\nThe environment held {STUB_SECRET}."),
            vec![(None, "hygiene")],
        ),
        (
            "a secret-shaped string in the subject alone",
            clean.clone(),
            format!("docs(record): by hand, with {STUB_SECRET}"),
            vec![(None, "hygiene")],
        ),
    ];
    for (what, text, message, expected) in cases {
        rig.write(&by_hand, &text);
        rig.git(&["add", "--", &by_hand]);
        rig.git(&["commit", "-q", "-m", &message]);
        let commit = rig.rev("HEAD");
        // The read a stage starts from meets it as the push does — the push that is owed
        // is the read's to make, and it makes none that is not vetted.
        stage.git_state().refused("unvetted");
        assert_eq!(rig.remote(LOOP), Some(recorded.clone()));

        let seen = rig.step(&["push", "--branch", LOOP]);
        let said = seen.refused("unvetted");
        assert_eq!(
            rig.remote(LOOP),
            Some(recorded.clone()),
            "{what}: NOTHING REACHED THE REMOTE"
        );
        assert!(
            !rig.git_at(&rig.origin, &["cat-file", "-e", &commit])
                .status
                .success(),
            "{what}: the bare remote does not hold the commit"
        );
        assert!(
            !seen.trace.iter().any(|call| call.starts_with("push ")),
            "{what}: no push was tried: {:?}",
            seen.trace
        );
        let named: Vec<Place> = said["vet"]["refused"]
            .as_array()
            .expect("what the vet refused")
            .iter()
            .map(|found| {
                assert_eq!(
                    found["commit"],
                    commit.as_str(),
                    "{what}: the commit is named"
                );
                (
                    found["path"].as_str(),
                    found["why"].as_str().expect("a word"),
                )
            })
            .collect();
        assert_eq!(
            named, expected,
            "{what}: each place, by its word: {}",
            seen.raw
        );
        assert!(
            !seen.raw.contains(DENY_TERM)
                && !seen.raw.contains(&home)
                && !seen.raw.contains(STUB_SECRET),
            "{what}: named by where, never by what: {}",
            seen.raw
        );

        // THE EXIT. The state is one a read names: the refusal's text spells it, and it is
        // run as spelled. It names the commit again, pushes nothing — and answers ok once
        // the human has taken the commit back, after which the push is made.
        let then = said["halt"]["recommendation"]
            .as_str()
            .expect("what is done about it");
        assert!(then.contains("the human's"), "{what}: {then}");
        // The read is of THE BRANCH, less what the remote held when the step asked it —
        // each commit by its sha, and no remote-tracking ref.
        let read = named_command(then, "dev/stabilize-record vet --range ");
        assert_eq!(
            read,
            format!(
                "dev/stabilize-record vet --range -- {LOOP} --not {}",
                rig.remote_heads().join(" ")
            )
        );
        let (code, out, _) = rig.record_script(read, "");
        let answer: Value = serde_json::from_str(&out).expect("the read prints its result");
        assert_eq!(
            json!([code, answer["ok"], answer["commits"], answer["refused"]]),
            json!([24, false, [commit], said["vet"]["refused"]]),
            "{what}: the read names what the refusal named"
        );
        assert!(
            answer["then"]
                .as_str()
                .is_some_and(|then| then.contains("the human's"))
        );
        assert_eq!(
            rig.remote(LOOP),
            Some(recorded.clone()),
            "{what}: a read pushes nothing"
        );
        // The human's act — plain git, never the tool's: the commit is taken back.
        rig.git(&["reset", "-q", "--hard", &recorded]);
        let (code, out, _) = rig.record_script(read, "");
        let answer: Value = serde_json::from_str(&out).expect("the read prints its result");
        assert_eq!(
            json!([code, answer["ok"], answer["commits"]]),
            json!([0, true, []]),
            "{what}"
        );
        rig.step(&["push", "--branch", LOOP]).done("push", "ready");
    }

    // MUST NOT REFUSE: what the remote holds already. A file of the run that a hand pushed
    // with plain git — outside the tool — is not held against the merge that later carries
    // it: a merge is vetted for what it changes itself, and no commit can be rewritten to
    // un-publish what is public.
    let product = product();
    let logs = logs();
    let earlier = rig.round(9);
    rig.change("crates/a.txt", "fixed early\n", "fix: an earlier finding");
    rig.step(&["push", "--branch", &earlier])
        .done("push", "ready");
    rig.git(&["switch", "-q", LOOP]);
    rig.change(
        &format!("{RUN_DIR}/r1/notes.md"),
        &format!("It ran in {home}.\n"),
        "docs(record): a note, pushed by hand",
    );
    rig.git(&["push", "-q", "origin", LOOP]);
    rig.git(&["switch", "-q", &earlier]);
    let landed = rig.step(&land(&earlier, &product, &logs));
    let recorded = landed.done("land", "merged")["merge_commit"]
        .as_str()
        .expect("the merge")
        .to_owned();
    assert_eq!(landed.line["vetted"], json!([recorded]));
    assert_eq!(rig.remote(LOOP), Some(recorded.clone()));

    // EVERY PUSH THE TOOL MAKES: a landing's, and a carried record's.
    let round = rig.round(1);
    rig.change(
        "crates/a.txt",
        "fixed\n",
        &format!("fix: a finding of {DENY_TERM}"),
    );
    let seen = rig.step(&land(&round, &product, &logs));
    let said = seen.refused("unvetted");
    assert!(
        said["merge_commit"].is_string(),
        "the merge is made, and says so: {}",
        seen.raw
    );
    assert_eq!(
        rig.remote(LOOP),
        Some(recorded.clone()),
        "a landing's push is vetted"
    );
    rig.git(&["reset", "-q", "--hard", &recorded]);
    let dropped = rig.round(2);
    let record = rig.change(
        &format!("{RUN_DIR}/r1/dropped.md"),
        &format!("It ran in {home}.\n"),
        "docs(record): the round is dropped",
    );
    let seen = rig.step(&carry(&product, &[record.as_str()]));
    seen.refused("unvetted");
    assert_eq!(
        rig.remote(LOOP),
        Some(recorded),
        "a carried record's push is vetted"
    );
    assert_eq!(rig.remote(&dropped), None);
}

/// A path inside single quotes of a shell text.
fn sh_quoted(path: &Path) -> String {
    path.display().to_string().replace('\'', "'\\''")
}

/// **Something that happens WHILE THE RANGE IS SCANNED**: the rig's `gitleaks` becomes one
/// that — the first time it is asked to scan a range, which is the push's vet — runs
/// `during`, a shell text, and then scans as the stand-in does. It stands in for a second
/// committer, or a second pusher, in the seconds the two scanners take at every push.
fn while_the_range_is_scanned(rig: &StepRig, during: &str) {
    rig.on_path("gitleaks-stub", &gitleaks_stub());
    let flag = sh_quoted(&rig.dir().join("happened"));
    fs::remove_file(rig.dir().join("bin/gitleaks")).expect("remove the stand-in");
    rig.on_path(
        "gitleaks",
        &format!(
            "#!/bin/sh\ncase \" $* \" in *\" --log-opts \"*) if [ ! -e '{flag}' ]; then : >'{flag}'\n( {during} ) >/dev/null 2>&1\nfi ;; esac\nexec gitleaks-stub \"$@\"\n"
        ),
    );
}

/// **What the remote lacks is asked of the remote, and never read off a remote-tracking
/// ref** (the core review's `F2`). Both reads of a push — which commits are no record's,
/// and which commits are vetted — were `<branch> --not --remotes=origin`: with ONE ref under
/// `refs/remotes/origin/` standing at a local commit, nothing was foreign, nothing was
/// vetted, and `git push` published everything. A tracking ref says what a fetch once saw,
/// or what a hand wrote there; the tool now asks the remote (`git ls-remote`), and a ref
/// the remote does not bear out switches off nothing — whichever ref it is.
#[test]
fn what_the_remote_lacks_is_asked_of_the_remote_and_never_read_off_a_tracking_ref() {
    let rig = StepRig::new("vet-asked");
    let product = product();
    let record_push = ["push", "--branch", LOOP, "--run-dir", RUN_DIR];

    // MUST NOT REFUSE: a head of the remote that this clone does not hold — `main`, moved
    // on by somebody else — leaves nothing out of a range, and stops no record's push.
    rig.pushed_by_another("main", "elsewhere.md", "elsewhere\n");
    let note = rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a note\n",
        "docs(record): a note",
    );
    let seen = rig.step(&git_state("test", &product));
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([line["vetted"], line["remote_head"]]),
        json!([[note], note]),
        "{}",
        seen.raw
    );
    assert_eq!(rig.remote(LOOP), Some(note.clone()));

    // Some task's commit, outside the run's directory, with a denylisted line — and the
    // control: it is refused, by the read and by both pushes.
    let leak = rig.change(
        "docs-x/leak.md",
        &format!("{DENY_TERM} outside\n"),
        "docs: a task's commit",
    );
    rig.step(&git_state("test", &product))
        .refused("foreign-commit");

    // A ref the remote does not bear out: of a branch it does not hold, of `main`, of the
    // loop branch itself.
    for stale in ["fix/gone-branch", "main", LOOP] {
        rig.git(&[
            "update-ref",
            &format!("refs/remotes/origin/{stale}"),
            "HEAD",
        ]);
        for (what, seen, word) in [
            (
                "git-state",
                rig.step(&git_state("test", &product)),
                "foreign-commit",
            ),
            ("a record's push", rig.step(&record_push), "foreign-commit"),
            ("a push", rig.step(&["push", "--branch", LOOP]), "unvetted"),
        ] {
            let what = format!("`{what}`, with `origin/{stale}` at the local head");
            let said = seen.refused(word);
            if word == "unvetted" {
                assert_eq!(
                    json!([
                        said["vet"]["commits"],
                        said["vet"]["refused"][0]["commit"],
                        said["vet"]["refused"][0]["path"]
                    ]),
                    json!([[leak], leak, "docs-x/leak.md"]),
                    "{what}: the commit is vetted, and named: {}",
                    seen.raw
                );
            }
            assert_eq!(
                rig.remote(LOOP),
                Some(note.clone()),
                "{what}: NOTHING REACHED THE REMOTE"
            );
            assert!(
                !rig.git_at(&rig.origin, &["cat-file", "-e", &leak])
                    .status
                    .success(),
                "{what}: the bare remote does not hold the commit"
            );
            assert!(
                !seen.trace.iter().any(|call| call.starts_with("push ")),
                "{what}: no push was tried: {:?}",
                seen.trace
            );
        }
    }
}

/// **A commit that lands while the range is vetted is not published** (the core review's
/// `F1`). The push listed what the remote lacked, vetted that list, and pushed the branch
/// by name — so whatever the branch stood at when `git push` ran was published, and the
/// line came back with `vetted: [A]` and `remote_head: B`. The vet is now of ONE commit, by
/// its sha, and after it the branch and the remote are both asked again: where either
/// moved, nothing is pushed, and the step asked again meets what landed as its first read
/// meets any commit.
#[test]
fn a_commit_that_lands_while_the_range_is_vetted_is_not_published() {
    // THE BRANCH MOVED: a second committer, in the seconds the scanners take.
    let rig = StepRig::new("vet-raced");
    let product = product();
    let pushed = rig.rev("HEAD");
    let note = rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a note\n",
        "docs(record): a note",
    );
    while_the_range_is_scanned(
        &rig,
        &format!(
            "cd '{}' && mkdir -p docs-x && printf '%s raced\\n' {DENY_TERM} >docs-x/raced.md && git add docs-x/raced.md && git commit -q -m 'docs: landed while the vet ran'",
            sh_quoted(&rig.root)
        ),
    );
    let seen = rig.step(&git_state("test", &product));
    let said = seen.refused("remote-differs");
    let raced = rig.rev("HEAD");
    assert_ne!(raced, note, "a commit landed while the range was scanned");
    assert!(
        said["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(&note) && cause.contains(&raced)),
        "the commit that was vetted and the one the branch stands at are named: {}",
        seen.raw
    );
    assert_eq!(
        rig.remote(LOOP),
        Some(pushed.clone()),
        "NOTHING REACHED THE REMOTE"
    );
    for commit in [&note, &raced] {
        assert!(
            !rig.git_at(&rig.origin, &["cat-file", "-e", commit])
                .status
                .success(),
            "the bare remote does not hold {commit}"
        );
    }
    assert!(
        !seen.trace.iter().any(|call| call.starts_with("push ")),
        "no push was tried: {:?}",
        seen.trace
    );
    // THE EXIT: asked again, the step meets what landed as it meets any commit — it is no
    // record's, and it is vetted like every other.
    rig.step(&git_state("test", &product))
        .refused("foreign-commit");
    let seen = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(
        seen.refused("unvetted")["vet"]["commits"],
        json!([note, raced]),
        "{}",
        seen.raw
    );
    assert_eq!(rig.remote(LOOP), Some(pushed));

    // THE REMOTE MOVED: it holds another head than when the range was read.
    let rig = StepRig::new("vet-moved");
    let pushed = rig.rev("HEAD");
    let note = rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a note\n",
        "docs(record): a note",
    );
    while_the_range_is_scanned(
        &rig,
        &format!(
            "git --git-dir='{}' update-ref refs/heads/fix/elsewhere {pushed}",
            sh_quoted(&rig.origin)
        ),
    );
    let seen = rig.step(&["push", "--branch", LOOP]);
    seen.refused("remote-differs");
    assert_eq!(rig.remote(LOOP), Some(pushed), "nothing was pushed");
    assert!(
        !seen.trace.iter().any(|call| call.starts_with("push ")),
        "no push was tried: {:?}",
        seen.trace
    );
    // MUST NOT REFUSE: the step asked again, over a remote that stands still.
    let seen = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(seen.done("push", "ready")["vetted"], json!([note]));
    assert_eq!(rig.remote(LOOP), Some(note));
}

/// **What a commit says is held to what its files are held to** (the core review's `F6`) —
/// here for a commit that changes nothing of a run: a fixer's, a tuning commit. gitleaks
/// reads added lines and never a message, so the one check of a writer that a push did not
/// repeat was the secret scan of what a commit SAYS.
#[test]
fn what_a_commit_says_is_held_to_what_its_files_are_held_to() {
    let rig = StepRig::new("vet-message");
    let pushed = rig.rev("HEAD");
    for message in [
        format!("fix: a finding\n\nThe environment held {STUB_SECRET}."),
        format!("fix: a finding of {STUB_SECRET}"),
    ] {
        let commit = rig.change("crates/a.txt", "fixed\n", &message);
        let seen = rig.step(&["push", "--branch", LOOP]);
        let said = seen.refused("unvetted");
        let refused = said["vet"]["refused"].as_array().expect("what was refused");
        assert_eq!(
            refused
                .iter()
                .map(|found| json!([found["commit"], found["path"], found["why"]]))
                .collect::<Vec<_>>(),
            vec![json!([commit, null, "hygiene"])],
            "the commit, and its message: {}",
            seen.raw
        );
        assert!(
            !seen.raw.contains(STUB_SECRET),
            "by where, never by what: {}",
            seen.raw
        );
        assert_eq!(rig.remote(LOOP), Some(pushed.clone()), "nothing was pushed");
        rig.git(&["reset", "-q", "--hard", &pushed]);
    }
    // MUST NOT REFUSE: the same change, saying nothing of the kind.
    let clean = rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let seen = rig.step(&["push", "--branch", LOOP]);
    assert_eq!(seen.done("push", "ready")["vetted"], json!([clean]));
}

/// **The secret scan of a range reads a merge's own lines** (the core review's sixth lead,
/// driven: a credential typed while a conflict was resolved was pushed by the tool).
/// gitleaks reads what `git log -p` prints, and that prints no diff for a merge unless it
/// is asked; the denylist scan asks (`--remerge-diff`), and the push's secret scan now asks
/// the same. Held here on every machine by what the scanner is ASKED — the stand-in reads
/// no range — and by the machine's own gitleaks over a real merge in the arm below.
#[test]
fn the_secret_scan_of_a_range_is_asked_for_a_merges_own_lines() {
    let rig = StepRig::new("vet-merge-asked");
    let note = rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a note\n",
        "docs(record): a note",
    );
    let held = rig.remote_heads().join(" ");
    let asked = rig.dir().join("asked");
    while_the_range_is_scanned(
        &rig,
        &format!("printf '%s\\n' \"$*\" >'{}'", sh_quoted(&asked)),
    );
    rig.step(&["push", "--branch", LOOP]).done("push", "ready");
    let asked = fs::read_to_string(&asked).expect("what the range's secret scan was asked");
    assert!(
        asked.contains(&format!("--log-opts --remerge-diff {note} --not {held} ")),
        "the range, and a merge's own lines with it: {asked}"
    );
}

/// The stub stands in for gitleaks' exit contract, reads the tree and not the range, and
/// says nothing about gitleaks' rules. This arm runs whatever the machine has over the
/// range a push would publish: with gitleaks installed, a credential-shaped string in an
/// unpushed commit is a hit that names the commit and the file, and a clean commit is
/// pushed; without it, the scan did not run. On no machine is the credential pushed.
#[test]
fn the_machines_own_gitleaks_reads_the_range_a_push_would_publish() {
    let rig = StepRig::new("vet-real-gitleaks");
    fs::remove_file(rig.dir().join("bin/gitleaks")).expect("remove the stub");
    let installed = std::env::var_os("PATH")
        .is_some_and(|path| std::env::split_paths(&path).any(|dir| dir.join("gitleaks").is_file()));
    let pushed = rig.rev("HEAD");

    let clean = rig.change("crates/a.txt", "tuned\n", "build(dev): a tuning commit");
    let seen = rig.step(&["push", "--branch", LOOP]);
    if !installed {
        seen.refused("did-not-run");
        assert_eq!(rig.remote(LOOP), Some(pushed));
        return;
    }
    assert_eq!(seen.done("push", "ready")["vetted"], json!([clean]));

    // Built here, never spelled: a literal of this shape in a tracked file is a finding of
    // the very scan this arm drives.
    let mut state = 0x2545_f491_4f6c_dd1d_u64;
    let tail: String = (0..36)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            alphabet[(state >> 33) as usize % alphabet.len()] as char
        })
        .collect();
    let token = format!("{}{}_{tail}", "gh", 'p');
    let leaked = rig.change(
        "crates/a.txt",
        &format!("tuned\nthe environment held {token}\n"),
        "build(dev): another tuning commit",
    );
    let seen = rig.step(&["push", "--branch", LOOP]);
    let said = seen.refused("unvetted");
    assert_eq!(
        json!([
            said["vet"]["refused"][0]["commit"],
            said["vet"]["refused"][0]["path"],
            said["vet"]["refused"][0]["why"]
        ]),
        json!([leaked, "crates/a.txt", "hygiene"]),
        "gitleaks names the commit and the file: {}",
        seen.raw
    );
    assert!(
        !seen.raw.contains(&token),
        "and never the string: {}",
        seen.raw
    );
    assert_eq!(
        rig.remote(LOOP),
        Some(clean.clone()),
        "the credential was not pushed"
    );

    // AND WHAT A COMMIT SAYS (the core review's `F6`): the same string in a message, over a
    // file that holds nothing — which gitleaks, reading a range, never reads.
    rig.git(&["reset", "-q", "--hard", &clean]);
    let said_it = rig.change(
        "crates/a.txt",
        "tuned again\n",
        &format!("build(dev): a third tuning commit\n\nThe environment held {token}."),
    );
    let seen = rig.step(&["push", "--branch", LOOP]);
    let said = seen.refused("unvetted");
    assert_eq!(
        json!([
            said["vet"]["refused"][0]["commit"],
            said["vet"]["refused"][0]["path"],
            said["vet"]["refused"][0]["why"]
        ]),
        json!([said_it, null, "hygiene"]),
        "gitleaks' own rules read the message: {}",
        seen.raw
    );
    assert!(
        !seen.raw.contains(&token),
        "and never the string: {}",
        seen.raw
    );
    assert_eq!(
        rig.remote(LOOP),
        Some(clean.clone()),
        "the credential in the message was not pushed"
    );

    // AND A MERGE'S OWN LINES (the core review's sixth lead): neither parent holds the
    // string — only the hand that resolved the conflict typed it.
    rig.git(&["reset", "-q", "--hard", &clean]);
    rig.git(&["switch", "-q", "-c", "fix/side"]);
    rig.change("crates/a.txt", "theirs\n", "fix: theirs");
    rig.git(&["switch", "-q", LOOP]);
    rig.change("crates/a.txt", "ours\n", "fix: ours");
    assert!(
        !rig.git_ok(&["merge", "-q", "fix/side"]),
        "the fixture needs a conflict, so that the resolution is the merge's own"
    );
    rig.write(
        "crates/a.txt",
        &format!("resolved, and the environment held {token}\n"),
    );
    let merge = rig.commit("Merge branch 'fix/side'");
    let seen = rig.step(&["push", "--branch", LOOP]);
    let said = seen.refused("unvetted");
    assert!(
        said["vet"]["refused"]
            .as_array()
            .expect("what was refused")
            .iter()
            .any(|found| found["commit"] == merge.as_str()
                && found["path"] == "crates/a.txt"
                && found["why"] == "hygiene"),
        "gitleaks names the merge and the file: {}",
        seen.raw
    );
    assert!(
        !seen.raw.contains(&token),
        "and never the string: {}",
        seen.raw
    );
    assert_eq!(
        rig.remote(LOOP),
        Some(clean),
        "the credential in the merge was not pushed"
    );
}

// ---------------------------------------------------------------------------
// A record act is repeatable, and every invocation reconciles first
// ---------------------------------------------------------------------------

/// Runs one of the two scripts unmodified, in a process of its own, and writes down each
/// point it passes — for the step tool every child command and the one file it writes of a
/// record, the commit step's kept answer (`KILL_WRAPS=children`), for the record script
/// every operation that makes, places or removes a file below the run's directory
/// (`files`: a temporary made, a rename, a link, an unlink). `KILL_AT=N` sends the
/// process SIGKILL just before its Nth point, `KILL_BEFORE=WORDS` before the first point
/// whose text holds the words: a kill between two system calls of the committed code,
/// never inside a copy of it. (The re-review's helpers `killat.py` and `killstep.py`, as one
/// file.)
const KILLER: &str = r#"import builtins
import os
import runpy
import signal
import subprocess
import sys
import tempfile

at = int(os.environ.get("KILL_AT", "0"))
words = os.environ.get("KILL_BEFORE", "")
log = os.environ["KILL_LOG"]
under = os.path.realpath(os.environ["KILL_UNDER"])
seen = [0]


def point(what):
    seen[0] += 1
    with open(log, "a", encoding="utf-8") as out:
        out.write(what + "\n")
    if seen[0] == at or (words and words in what):
        os.kill(os.getpid(), signal.SIGKILL)


def before(module, name, says):
    real = getattr(module, name)

    def wrapped(*args, **kwargs):
        what = says(*args, **kwargs)
        if what is not None:
            point(what)
        return real(*args, **kwargs)

    setattr(module, name, wrapped)


def below(path):
    return (os.path.realpath(str(path)) + "/").startswith(under + "/")


def named(what, path):
    return "%s %s" % (what, os.path.basename(str(path))) if below(os.path.dirname(str(path))) else None


if os.environ["KILL_WRAPS"] == "children":
    before(subprocess, "run", lambda argv, *a, **k: " ".join(os.path.basename(arg) if os.path.isabs(arg) else arg for arg in argv[:3]))
    before(builtins, "open", lambda path, mode="r", *a, **k: "write " + os.path.basename(str(path)) if mode == "w" and str(path).endswith(".recorded.json") else None)
else:
    before(tempfile, "mkstemp", lambda *a, **k: "mkstemp" if "dir" in k and below(k["dir"]) else None)
    before(os, "replace", lambda source, target, *a, **k: named("replace", target))
    before(os, "link", lambda source, target, *a, **k: named("link", target))
    before(os, "unlink", lambda path, *a, **k: named("unlink", path))
script = sys.argv[1]
sys.argv = sys.argv[1:]
runpy.run_path(script, run_name="__main__")
"#;

/// What a counted or a killed run of a script left: its exit status — none where a signal
/// ended it — the points it passed, and its stderr.
struct Counted {
    code: Option<i32>,
    points: Vec<String>,
    stderr: String,
}

impl StepRig {
    /// A copy of the rig as it stands, in a directory of its own: the repository, its
    /// remote, the scratch root and what lies beside them — made by a child (`cp -R`: no
    /// file that will be executed is written by this process, [`placed_executable`]) — and
    /// the clone pointed at the copy of its remote. What a kill is driven on: a stage is
    /// stood up once, and every kill point gets a copy.
    fn copy(&self, label: &str) -> Self {
        let dir = ScratchDir::new(&format!("stabilize-step-{label}"));
        let copied = Command::new("cp")
            .arg("-R")
            .arg(self.dir.path().join("."))
            .arg(dir.path())
            .output()
            .expect("run cp");
        assert!(copied.status.success(), "copy the rig: {copied:?}");
        let rig = StepRig {
            root: dir.path().join(HOSTILE_ROOT),
            origin: dir.path().join("origin.git"),
            scratch: dir.path().join("scratch"),
            home: dir.path().join("home"),
            trace: dir.path().join("trace"),
            path: format!(
                "{}:{}",
                dir.path().join("bin").display(),
                std::env::var("PATH").expect("the suite runs with a UTF-8 PATH")
            ),
            dir,
        };
        rig.git(&[
            "remote",
            "set-url",
            "origin",
            &rig.origin.display().to_string(),
        ]);
        rig
    }

    /// One of the two scripts under [`KILLER`], fed `stdin`: `wraps` says which points are
    /// counted, `kill` which of them it dies before (`("KILL_AT", "0")`: none).
    fn wrapped(
        &self,
        wraps: &str,
        script: &str,
        args: &[&str],
        stdin: &str,
        kill: (&str, &str),
    ) -> Counted {
        let killer = self.dir.path().join("kill.py");
        fs::write(&killer, KILLER).expect("write the killer");
        let log = self.dir.path().join("points");
        fs::write(&log, "").expect("empty the points");
        let mut command = Command::new("python3");
        command
            .arg(&killer)
            .arg(self.root.join(script))
            .args(args)
            .current_dir(self.dir.path());
        let mut child = self
            .hermetic(command)
            .env("KILL_WRAPS", wraps)
            .env("KILL_LOG", &log)
            .env("KILL_UNDER", self.root.join(RUN_DIR))
            .env(kill.0, kill.1)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn python3");
        child_stdin::feed(&mut child, stdin);
        let out = child.wait_with_output().expect("the script ends");
        Counted {
            code: out.status.code(),
            points: fs::read_to_string(&log)
                .expect("read the points")
                .lines()
                .map(str::to_owned)
                .collect(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }

    /// The run's journal, as the bytes on disk — none where there is no file.
    fn journal(&self) -> Option<String> {
        fs::read_to_string(self.root.join(RUN_DIR).join(".pending.json")).ok()
    }

    /// Every ref of the clone and what it stands at: a fetch or a push moves one.
    fn refs(&self) -> String {
        self.git(&["for-each-ref", "--format=%(refname) %(objectname)"])
    }
}

/// Where a run stands, as everything a later act could read of it: the record script's
/// state document, the local head and the remote's, the tree, and every file below the
/// run's directory — the journal and a temporary among them.
#[derive(Debug, Clone, PartialEq)]
struct End {
    state: Value,
    head: String,
    remote: Option<String>,
    status: String,
    files: BTreeMap<String, String>,
}

fn files_below(dir: &Path, prefix: &str, found: &mut BTreeMap<String, String>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("list {}: {e}", dir.display()))
        .map(|entry| entry.expect("a directory entry"))
        .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        if entry.path().is_dir() {
            files_below(&entry.path(), &format!("{name}/"), found);
        } else {
            let text = fs::read(entry.path()).expect("read a file of the run");
            found.insert(name, String::from_utf8_lossy(&text).into_owned());
        }
    }
}

/// The arguments of the `record` act's three commands, as the kill test and the plants
/// below run them.
const PUSH: [&str; 3] = ["push", "--branch", LOOP];

impl Stage {
    /// A copy of the stage as it stands ([`StepRig::copy`]): its gate's file and the batch
    /// that names it are the copy's own.
    fn copy(&self, label: &str) -> Self {
        let rig = self.rig.copy(label);
        let (from, to) = (
            self.rig.dir().display().to_string(),
            rig.dir().display().to_string(),
        );
        Stage {
            gate: self.gate.replace(&from, &to),
            batch: serde_json::from_str(&self.batch.to_string().replace(&from, &to))
                .expect("the batch, with the copy's paths"),
            rig,
        }
    }

    fn end(&self) -> End {
        let mut files = BTreeMap::new();
        files_below(&self.rig.root.join(RUN_DIR), "", &mut files);
        End {
            state: serde_json::from_str(&self.rig.wrote(&format!("state --run {RUN}"), ""))
                .expect("the state document"),
            head: self.rig.rev("HEAD"),
            remote: self.rig.remote(LOOP),
            status: self.rig.status(),
            files,
        }
    }

    /// The commit step, with more flags than the harness passes today.
    fn record_with(&self, more: &[&str]) -> Stepped {
        let scratch = self.rig.scratch.display().to_string();
        let mut args = vec![
            "record",
            "--branch",
            LOOP,
            "--run-dir",
            RUN_DIR,
            "--gate",
            self.gate.as_str(),
            "--calls",
            "7",
            "--checks",
            "2",
            "--scratch",
            scratch.as_str(),
        ];
        args.extend(more);
        self.rig.step(&args)
    }

    /// The `apply` of the stage's batch, under the killer.
    fn apply_under(&self, kill: (&str, &str)) -> Counted {
        self.rig.wrapped(
            "files",
            RECORD,
            &["apply", "--run", RUN, "--round", "1", "--subject", SUBJECT],
            &self.batch.to_string(),
            kill,
        )
    }

    /// The commit step, as the harness asks for it, under the killer.
    fn record_under(&self, kill: (&str, &str)) -> Counted {
        let scratch = self.rig.scratch.display().to_string();
        self.rig.wrapped(
            "children",
            TOOL,
            &[
                "record",
                "--branch",
                LOOP,
                "--run-dir",
                RUN_DIR,
                "--gate",
                &self.gate,
                "--calls",
                "7",
                "--checks",
                "2",
                "--scratch",
                &scratch,
            ],
            "",
            kill,
        )
    }

    /// A stage whose record is COMMITTED AND NEITHER SETTLED NOR PUSHED: the commit step
    /// was killed between its commit and the call that forgets the batch.
    fn unsettled(label: &str) -> Self {
        let stage = Stage::new(label);
        stage.apply(SUBJECT);
        let killed = stage.record_under(("KILL_BEFORE", "stabilize-record settle"));
        assert_eq!(
            killed.code, None,
            "the commit step was killed: {}",
            killed.stderr
        );
        assert_eq!(stage.rig.git(&["log", "-1", "--format=%s"]), SUBJECT);
        assert!(
            stage.rig.journal().is_some(),
            "and its batch is not settled"
        );
        stage
    }

    /// `git-state` with one more flag.
    fn git_state_with(&self, flag: &str) -> Stepped {
        let product = product();
        let mut args = git_state("test", &product);
        args.push(flag);
        self.rig.step(&args)
    }

    /// ONE READ, as an invocation of a stage makes it: the git state — which finishes what
    /// the tool finishes — and then the one step that state leaves to the stage: a batch
    /// that is applied and not committed is committed by the commit step, on the gate's
    /// file, and pushed. What the git state said; or why the read ended at a refusal.
    fn read(&self) -> Result<Value, String> {
        let seen = self.git_state();
        if seen.code != 0 {
            return Err(format!("`git-state` refused: {}", seen.stderr.trim()));
        }
        if !seen.line["pending"].is_null() {
            let recorded = self.record();
            if recorded.code != 0 {
                return Err(format!(
                    "the commit step refused: {}",
                    recorded.stderr.trim()
                ));
            }
            let pushed = self.rig.step(&PUSH);
            if pushed.code != 0 {
                return Err(format!("the push refused: {}", pushed.stderr.trim()));
            }
        }
        Ok(seen.line)
    }

    /// TWO READS, and where they end: the second must find nothing at all.
    fn within_two_reads(&self) -> Result<End, String> {
        self.read()?;
        let second = self.read()?;
        if second["found"] != json!([]) || !second["pending"].is_null() {
            return Err(format!("the second read still found something: {second}"));
        }
        Ok(self.end())
    }
}

/// The three commands of the `record` act, as the tool's table names them: what a kill is
/// driven between. [`the_table_names_every_act_and_every_arrival_and_each_is_driven`] holds
/// the table's own list to this one, so an act that gains phases there gains its kills here.
const KILLED: [&str; 3] = [
    "dev/stabilize-record apply",
    "dev/stabilize-step record",
    "dev/stabilize-step push",
];

/// ONE COMMAND OF A RECORD, KILLED BEFORE EACH OF ITS POINTS — what the four tests below
/// share. The command (`KILLED[which]`) runs once under the killer to count its points:
/// every file operation of `apply` below the run's directory; every child command of
/// `record` and of `push`, and the write of the commit step's kept answer. It is then
/// killed before each of them, in a copy of the stage of its own, and within two reads the
/// run stands — **as `state`, the tree, the journal and the remote say it** — at one of two
/// states:
///
/// - where the journal was not yet written, at **the state before the batch**: the tables
///   as they were, the reports and the scope still pending, the attempt counted, nothing to
///   push;
/// - else at **the state of the uninterrupted control**, with the remote at the local head
///   and a clean tree.
///
/// Never at a third, and never at a refusal: nothing here is the human's. With `retried`,
/// the commit step is first asked for again as the harness would ask — the same command,
/// with the commit the stage began on — and must answer `recorded` with the control's
/// commit; what it found at each point is returned, in the points' order.
fn killed_before_each_point(which: usize, retried: bool) -> Vec<(String, Value)> {
    let command = KILLED[which];
    let label = format!("kill-{which}{}", if retried { "-again" } else { "" });
    let base = Stage::new(&label);
    let before = base.end();
    let control = base.copy(&format!("{label}-control"));
    control.apply(SUBJECT);
    control.record().done("record", "recorded");
    control.rig.step(&PUSH).done("push", "ready");
    let after = control.end();
    assert_eq!(
        (after.remote.as_deref(), after.status.as_str()),
        (Some(after.head.as_str()), ""),
        "the control ends pushed and clean"
    );
    assert!(
        !after.files.contains_key(".pending.json") && before.head != after.head,
        "and with one commit more, and no journal"
    );

    // The command, from where it starts.
    let from = base.copy(&format!("{label}-from"));
    if which >= 1 {
        from.apply(SUBJECT);
    }
    if which == 2 {
        from.record().done("record", "recorded");
    }
    let run = |stage: &Stage, kill: (&str, &str)| match which {
        0 => stage.apply_under(kill),
        1 => stage.record_under(kill),
        _ => stage.rig.wrapped("children", TOOL, &PUSH, "", kill),
    };
    let counted = from.copy(&format!("{label}-count"));
    let whole = run(&counted, ("KILL_AT", "0"));
    assert_eq!(
        whole.code,
        Some(0),
        "`{command}`, counted: {}",
        whole.stderr
    );
    let points = whole.points;
    assert!(points.len() >= 8, "`{command}` passes points: {points:?}");
    if which == 1 {
        let at = |point: &str| {
            points
                .iter()
                .position(|what| what == point)
                .unwrap_or_else(|| panic!("`{point}` is a point of the commit step: {points:?}"))
        };
        assert!(
            at("git commit -q") < at("write gate.txt.recorded.json")
                && at("write gate.txt.recorded.json") < at("stabilize-record settle --run"),
            "the commit step's answer is kept after its commit and before the batch is settled: {points:?}"
        );
    }

    let next = std::sync::atomic::AtomicUsize::new(0);
    let ended = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|threads| {
        for _ in 0..6 {
            threads.spawn(|| {
                loop {
                    let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                    let Some(what) = points.get(k - 1) else {
                        break;
                    };
                    let stage = from.copy(&format!("{label}-{k}"));
                    let killed = run(&stage, ("KILL_AT", &k.to_string()));
                    let journaled = stage.rig.journal().is_some();
                    let mut found = Value::Null;
                    let end = if killed.code.is_some() {
                        Err(format!(
                            "it was not killed (exit {:?}): {}",
                            killed.code, killed.stderr
                        ))
                    } else if retried {
                        let again = stage.record_with(&["--head", &before.head]);
                        found = again.line["found"].clone();
                        if again.code != 0
                            || again.line["status"] != "recorded"
                            || again.line["commit"] != after.head.as_str()
                            || again.line["applied"]["calls"] != 7
                            || again.line["gate"]["ok"] != true
                        {
                            Err(format!(
                                "asked for again, the commit step did not answer `recorded` with the control's commit: {}",
                                again.raw.trim()
                            ))
                        } else if stage.rig.step(&PUSH).code != 0 {
                            Err("the push after it refused".to_owned())
                        } else {
                            stage.within_two_reads()
                        }
                    } else {
                        stage.within_two_reads()
                    };
                    ended.lock().expect("the ends").push((
                        k,
                        what.clone(),
                        which == 0 && !journaled,
                        end,
                        found,
                    ));
                }
            });
        }
    });
    let mut ended = ended.into_inner().expect("the ends");
    ended.sort_by_key(|(k, ..)| *k);
    assert_eq!(ended.len(), points.len(), "every kill point was driven");
    let mut wrong = Vec::new();
    let (mut undone, mut finished) = (0, 0);
    for (k, what, unjournaled, end, _) in &ended {
        let (wanted, name) = if *unjournaled {
            undone += 1;
            (&before, "the state before the batch")
        } else {
            finished += 1;
            (&after, "the state of the uninterrupted control")
        };
        match end {
            Err(why) => wrong.push(format!("before point {k} (`{what}`): {why}")),
            Ok(end) if end != wanted => wrong.push(format!(
                "before point {k} (`{what}`): a third state, and not {name} — head {} remote {:?} tree [{}] next {} journal {}",
                end.head,
                end.remote,
                end.status.replace('\n', " | "),
                end.state["next"],
                end.files.contains_key(".pending.json"),
            )),
            Ok(_) => {}
        }
    }
    assert!(
        wrong.is_empty(),
        "`{command}`: {} of {} kills did not end in one of the two states:\n{}",
        wrong.len(),
        ended.len(),
        wrong.join("\n")
    );
    if which == 0 {
        assert!(
            undone >= 3 && finished >= 8,
            "both ends are met where the batch is applied: {undone} before the batch, {finished} at the control"
        );
    } else {
        assert_eq!(
            undone, 0,
            "past its journal a record is finished, never undone"
        );
    }
    ended
        .into_iter()
        .map(|(_, what, _, _, found)| (what, found))
        .collect()
}

/// **A record step killed between its commands ends in one of two states, and no third**
/// (the second repair plan's `P1`; the re-review's `R2`, `R3`, `R4` and `R7`; the plan
/// review's `B8`) — here, **while its batch is applied**: before each file operation of
/// `dev/stabilize-record apply` below the run's directory. A kill before the journal is
/// written ends at the state before the batch — what it left are temporaries, which the
/// read removes; a kill after it ends at the control's state — the batch is put in place
/// whole, found applied, gated, committed and pushed ([`killed_before_each_point`]). One
/// test per command, so that none of them runs for minutes: this name selects all three.
#[test]
fn a_record_step_killed_between_its_commands_ends_in_one_of_two_states_while_its_batch_is_applied()
{
    killed_before_each_point(0, false);
}

/// The same, **in its commit step**: before each child command of `dev/stabilize-step
/// record` and before the write of its kept answer. Every such kill ends at the control's
/// state: a batch that is applied or staged is committed by the step asked again, one
/// whose commit is made is settled by the read, and the record is pushed.
#[test]
fn a_record_step_killed_between_its_commands_ends_in_one_of_two_states_in_its_commit_step() {
    killed_before_each_point(1, false);
}

/// The same, **in its push**: before each child command of `dev/stabilize-step push`. The
/// read makes the push that is owed, or finds it made.
#[test]
fn a_record_step_killed_between_its_commands_ends_in_one_of_two_states_in_its_push() {
    killed_before_each_point(2, false);
}

/// **A commit step that was killed, asked for again, answers `recorded`** — at every one of
/// its points (the orchestrator's *done when* for a repeatable act: *a second identical
/// call*). Handed the commit the stage began on, the same command commits where no commit
/// was made; takes its batch as recorded where the commit was made and the batch not
/// settled (`committed`); and answers again from the answer it kept beside the gate's file
/// once the batch is settled (`recorded`) — the file the tool's table lists under `kept`.
/// **A kill between the commit and that file's write** is one of the points, and ends at
/// `committed`: the journal still holds the batch, and HEAD holds its files. Each time the
/// answer names the control's commit, and the push then ends at the control's state.
#[test]
fn a_commit_step_that_was_killed_answers_recorded_when_it_is_asked_for_again() {
    let answered = killed_before_each_point(1, true);
    let found_at = |point: &str| -> &Value {
        let mut at = answered.iter().filter(|(what, _)| what.starts_with(point));
        let (_, found) = at
            .next()
            .unwrap_or_else(|| panic!("the commit step was killed before `{point}`"));
        assert!(at.next().is_none(), "`{point}` names one point");
        found
    };
    assert_eq!(
        json!([
            found_at("git add --"),
            found_at("stabilize-record vet"),
            found_at("git commit -q"),
            found_at("write gate.txt.recorded.json"),
            found_at("stabilize-record settle"),
            answered.last().expect("the last point").1,
        ]),
        json!([
            ["applied"],
            ["staged"],
            ["staged"],
            ["committed"],
            ["committed"],
            ["recorded"],
        ]),
        "what a commit step that was killed finds when it is asked for again: its batch to commit, its own staged paths, the batch in HEAD — from the commit to the settling, the write of its kept answer between them — and after the settling the answer it kept"
    );
}

/// **The wrong branch is refused before any write, fetch or push** (the plan review's
/// `B8`): a stage launched from the wrong directory, or on the wrong branch, must leave
/// nothing behind — the canary's safety rests on that order, and the acts now finish steps
/// themselves. Everything an act could finish is planted — a temporary of a killed write, a
/// batch whose commit is made and could be settled, a commit the remote lacks — and each
/// act of the record is asked on another branch: the journal, the tree, the remote and
/// every ref are as they were, and the git commands the tool ran are the branch's name and
/// the halt report's two reads.
#[test]
fn the_wrong_branch_is_refused_before_any_write_fetch_or_push() {
    let stage = Stage::unsettled("wrong-branch");
    let rig = &stage.rig;
    let temporary = format!("{RUN_DIR}/.stabilize-record.killed.tmp");
    rig.write(&temporary, "half a table\n");
    let branch = rig.round(1);
    let (journal, status, refs, remote) =
        (rig.journal(), rig.status(), rig.refs(), rig.remote(LOOP));
    assert_ne!(remote, Some(rig.rev(LOOP)), "a push is owed");

    let product = product();
    let state = git_state("test", &product);
    let mut look = state.clone();
    look.push("--look");
    let scratch = rig.scratch.display().to_string();
    let record = [
        "record",
        "--branch",
        LOOP,
        "--run-dir",
        RUN_DIR,
        "--gate",
        stage.gate.as_str(),
        "--calls",
        "7",
        "--checks",
        "2",
        "--scratch",
        scratch.as_str(),
    ];
    let discard = ["discard", "--branch", LOOP, "--run-dir", RUN_DIR];
    let record_push = ["push", "--branch", LOOP, "--run-dir", RUN_DIR];
    let acts: [(&str, &[&str]); 6] = [
        ("git-state", &state),
        ("git-state --look", &look),
        ("record", &record),
        ("push", &PUSH),
        ("push --run-dir", &record_push),
        ("discard", &discard),
    ];
    for (what, args) in acts {
        let seen = rig.step(args);
        seen.refused("wrong-branch");
        assert_eq!(
            seen.trace,
            lines(&[
                "branch --show-current",
                "branch --show-current",
                "status --porcelain"
            ]),
            "`{what}`: the branch's name, and the halt report's two reads — no fetch, no push, no write"
        );
        assert_eq!(
            (rig.journal(), rig.status(), rig.refs(), rig.remote(LOOP)),
            (
                journal.clone(),
                status.clone(),
                refs.clone(),
                remote.clone()
            ),
            "`{what}`: the journal, the tree, every ref and the remote are as they were"
        );
        assert!(
            rig.root.join(&temporary).exists() && rig.branch() == branch,
            "`{what}`: the temporary is where it was"
        );
    }
}

/// **A lock git left is named and never removed** (the plan review's advisory on `P1`): a
/// kill inside a git child leaves `index.lock`, and every later `git add` fails on it. The
/// acts that touch the index refuse before they read past the branch — naming the file,
/// leaving it, and saying whose it is: the orchestrator's, once no git process runs. With
/// the lock gone the same act does what it was asked.
#[test]
fn a_lock_git_left_is_named_and_not_removed() {
    let stage = Stage::new("lock");
    let rig = &stage.rig;
    stage.apply(SUBJECT);
    let lock = rig.root.join(".git/index.lock");
    fs::write(&lock, "").expect("leave a lock");
    let (journal, status, head) = (rig.journal(), rig.status(), rig.rev("HEAD"));
    let discard = ["discard", "--branch", LOOP, "--run-dir", RUN_DIR];
    for (what, seen) in [
        ("git-state", stage.git_state()),
        ("record", stage.record()),
        ("discard", rig.step(&discard)),
    ] {
        let said = seen.refused("locked");
        let halt = &said["halt"];
        assert!(
            halt["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.contains(".git/index.lock"))
                && halt["recommendation"].as_str().is_some_and(|then| then
                    .contains("no git process")
                    && then.contains("asked for again")),
            "`{what}`: the lock is named, and what leaves it: {}",
            seen.raw
        );
        assert!(lock.exists(), "`{what}`: the lock is not removed");
        assert_eq!(
            (rig.journal(), rig.status(), rig.rev("HEAD")),
            (journal.clone(), status.clone(), head.clone()),
            "`{what}`: nothing was written"
        );
        assert!(
            !seen
                .trace
                .iter()
                .any(|call| ["add", "commit", "restore", "push", "fetch"]
                    .contains(&call.split(' ').next().unwrap_or_default())),
            "`{what}`: nothing that changes anything ran: {:?}",
            seen.trace
        );
    }
    // MUST NOT REFUSE: the same acts once the lock is gone.
    fs::remove_file(&lock).expect("the orchestrator removes the lock");
    assert_eq!(
        stage.git_state().done("git-state", "ready")["pending"]["subject"],
        SUBJECT
    );
    stage.record().done("record", "recorded");
}

/// **Looking finishes nothing, and says what is owed** (the plan review's advisory; the
/// re-review's `R-L1`): an invocation that only looks (`stopAfter: 'state'`) asks with
/// `--look`. Every state the tool would finish is named in `found` and `owed`, in the
/// table's order, and nothing is finished — the journal, the temporary, every ref and the
/// remote are as they were. The same read without the flag finishes all of it.
#[test]
fn looking_finishes_nothing_and_says_what_is_owed() {
    let stage = Stage::unsettled("look");
    let rig = &stage.rig;
    let temporary = format!("{RUN_DIR}/r1/.stabilize-record.killed.tmp");
    rig.write(&temporary, "half a table\n");
    let (journal, status, head, remote) = (
        rig.journal(),
        rig.status(),
        rig.rev("HEAD"),
        rig.remote(LOOP),
    );
    let all = json!(["stray-temporaries", "committed", "unpushed"]);

    let seen = stage.git_state_with("--look");
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([
            line["found"],
            line["owed"],
            line["finished"],
            line["pending"]
        ]),
        json!([all, all, [], null]),
        "everything the tool would finish is owed, and named: {}",
        seen.raw
    );
    assert_eq!(
        json!([line["head"], line["remote_head"]]),
        json!([head, remote]),
        "the remote's head is said beside the local one: {}",
        seen.raw
    );
    assert_eq!(
        (
            rig.journal(),
            rig.status(),
            rig.rev("HEAD"),
            rig.remote(LOOP)
        ),
        (journal, status, head.clone(), remote),
        "and nothing is finished"
    );
    assert!(rig.root.join(&temporary).exists());
    assert!(
        !seen.trace.iter().any(|call| call.starts_with("push ")),
        "no push: {:?}",
        seen.trace
    );

    // The same read, without the flag: all of it finished, by name.
    let seen = stage.git_state();
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([
            line["found"],
            line["owed"],
            line["finished"],
            line["remote_head"]
        ]),
        json!([all, [], all, head]),
        "{}",
        seen.raw
    );
    assert_eq!(
        (rig.journal(), rig.status(), rig.remote(LOOP)),
        (None, String::new(), Some(head)),
        "the batch is settled, the temporary gone, the record pushed"
    );

    // A batch that is applied and not committed is the stage's to finish, in both.
    let stage = Stage::new("look-applied");
    stage.apply(SUBJECT);
    for seen in [stage.git_state_with("--look"), stage.git_state()] {
        let line = seen.done("git-state", "ready");
        assert_eq!(
            json!([
                line["found"],
                line["owed"],
                line["finished"],
                line["pending"]["subject"]
            ]),
            json!([["applied"], ["applied"], [], SUBJECT]),
            "{}",
            seen.raw
        );
    }
}

/// **A commit that is no record's is not pushed by the tool** (the plan review's advisory;
/// the lead *a commit made on the loop branch while a stage runs*): the loop branch ahead
/// of its remote by a commit that touches anything outside the run's directory is some
/// task's commit, behind that task's own gate. A stage does not start over it, and the
/// refusal is NOT the human's: it names the commit and the one command its author runs.
/// Nor does a record's own push (`push --run-dir`) publish it — and neither publishes the
/// record commits beside it. Once its author pushed it, the stage starts.
#[test]
fn a_commit_that_is_no_records_is_not_pushed_by_the_tool() {
    let rig = StepRig::new("foreign");
    let product = product();
    let pushed = rig.rev("HEAD");
    rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record\n",
        "docs(record): a note",
    );
    let tuning = rig.change("README.md", "tuned\n", "build(dev): a tuning commit");
    let record_push = ["push", "--branch", LOOP, "--run-dir", RUN_DIR];
    for (what, seen) in [
        ("git-state", rig.step(&git_state("test", &product))),
        ("a record's push", rig.step(&record_push)),
    ] {
        let said = seen.refused("foreign-commit");
        let halt = &said["halt"];
        assert!(
            halt["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.contains(&tuning)),
            "`{what}`: the commit is named: {}",
            seen.raw
        );
        let then = halt["recommendation"].as_str().expect("what leaves it");
        assert!(
            then.contains(&format!("`git push origin {LOOP}`"))
                && then.contains("its author")
                && !then.contains("the human reconciles"),
            "`{what}`: whose it is, and the command: {then}"
        );
        assert_eq!(
            rig.remote(LOOP),
            Some(pushed.clone()),
            "`{what}`: nothing was pushed — not the record's commit beside it either"
        );
        assert!(
            !seen.trace.iter().any(|call| call.starts_with("push ")),
            "`{what}`: no push is tried: {:?}",
            seen.trace
        );
    }
    // Its author pushes it, behind its own gate — by name, with plain git.
    rig.git(&["push", "-q", "origin", LOOP]);
    let seen = rig.step(&git_state("test", &product));
    assert_eq!(seen.done("git-state", "ready")["found"], json!([]));

    // MUST NOT REFUSE: a commit that touches only the run's directory is a record's, and
    // the push that is owed for it is made — by the read, and by a record's own push.
    let record = rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record, again\n",
        "docs(record): the note, again",
    );
    assert_eq!(
        rig.step(&record_push).done("push", "ready")["vetted"],
        json!([record])
    );
    let record = rig.change(
        &format!("{RUN_DIR}/note.md"),
        "a record, once more\n",
        "docs(record): the note, once more",
    );
    let seen = rig.step(&git_state("test", &product));
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([line["found"], line["finished"], line["remote_head"]]),
        json!([["unpushed"], ["unpushed"], record]),
        "{}",
        seen.raw
    );
    assert_eq!(rig.remote(LOOP), Some(record));
}

/// **A record that is committed and not pushed is said, and pushed first** (the re-review's
/// `R7`; the harness review's `R-M1` from the script's side): a push the remote did not
/// take left a record one commit ahead, and the read a stage starts from said `ready` —
/// the next round began on a tip the remote did not have. The read now reports the
/// remote's head, makes the push that is owed before anything else, and where the remote
/// still does not take it, is refused as the push is.
#[test]
fn a_record_the_remote_lacks_is_pushed_before_a_stage_starts() {
    let stage = Stage::new("owed-push");
    let rig = &stage.rig;
    let pushed = rig.rev("HEAD");
    stage.apply(SUBJECT);
    stage.record().done("record", "recorded");
    let recorded = rig.rev("HEAD");
    rig.remote_hook("pre-receive", "exit 1");
    rig.step(&PUSH).refused("push-rejected");

    // The remote still does not answer: the stage does not start, and the refusal is the
    // push's own.
    stage.git_state().refused("push-rejected");
    assert_eq!(rig.remote(LOOP), Some(pushed));

    // The cause is dealt with: the same read makes the push, and says so.
    fs::remove_file(rig.origin.join("hooks/pre-receive")).expect("the remote answers again");
    let seen = stage.git_state();
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([
            line["found"],
            line["finished"],
            line["head"],
            line["remote_head"],
            line["vetted"]
        ]),
        json!([["unpushed"], ["unpushed"], recorded, recorded, [recorded]]),
        "the push that was owed is made first, vetted: {}",
        seen.raw
    );
    assert_eq!(rig.remote(LOOP), Some(recorded.clone()));

    // A commit of the range that the record script would not have written is NOT pushed by
    // the read either: it is the human's, as at every push (the plan review's `B2`).
    let path = format!("{RUN_DIR}/r1/reports/test/by-hand.a2.md");
    rig.write(&path, &report_saying(&format!("It met {DENY_TERM} there.")));
    rig.git(&["add", "--", &path]);
    rig.git(&["commit", "-q", "-m", "docs(record): a report, by hand"]);
    let seen = stage.git_state();
    let said = seen.refused("unvetted");
    assert!(
        said["halt"]["recommendation"]
            .as_str()
            .is_some_and(|then| then.contains("it is the human's")),
        "{}",
        seen.raw
    );
    assert_eq!(rig.remote(LOOP), Some(recorded), "nothing was pushed");
}

/// A `git` that fails every `commit` while the file `fail-commit` lies in the rig — a
/// signing agent that does not answer, a hook that refuses — and is the real git otherwise.
fn failing_commit(rig: &StepRig) -> PathBuf {
    let quoted = |path: &Path| path.display().to_string().replace('\'', "'\\''");
    let marker = rig.dir().join("fail-commit");
    rig.on_path(
        "git",
        &format!(
            "#!/bin/sh\nif [ \"$1\" = commit ] && [ -e '{marker}' ]; then echo 'error: gpg failed to sign the data' >&2; exit 128; fi\nexec '{git}' \"$@\"\n",
            marker = quoted(&marker),
            git = quoted(&real_git()),
        ),
    );
    fs::write(&marker, "").expect("commits fail from here");
    marker
}

/// **A commit that failed is asked for again, and commits** (the re-review's `R3`): the
/// commit step is `git add` and then `git commit`, under whatever the machine's git
/// configuration says — and a commit that failed, or a kill between the two, left a staged
/// tree every act refused as dirty, which no command of either script left. The staged
/// lines of the act's own paths are the act's: the read names the state, and the same step
/// commits.
#[test]
fn a_commit_that_failed_is_asked_for_again_and_commits() {
    let stage = Stage::new("failed-commit");
    let rig = &stage.rig;
    let head = rig.rev("HEAD");
    stage.apply(SUBJECT);
    let marker = failing_commit(rig);
    let seen = stage.record();
    seen.refused("git");
    assert_eq!(rig.rev("HEAD"), head, "the commit failed");
    assert!(
        rig.status().lines().any(|line| line.starts_with("A  ")),
        "and its paths are staged: {}",
        rig.status()
    );

    // The cause still stands: the same refusal, and nothing worse.
    stage.record().refused("git");
    // What a stage would start from says what it found, and is no refusal.
    let state = stage.git_state();
    let line = state.done("git-state", "ready");
    assert_eq!(
        json!([line["found"], line["owed"], line["pending"]["subject"]]),
        json!([["staged"], ["staged"], SUBJECT]),
        "{}",
        state.raw
    );

    // The cause is dealt with: the step, asked for again, commits exactly the record.
    fs::remove_file(marker).expect("commits work again");
    let seen = stage.record();
    let line = seen.done("record", "recorded");
    assert_eq!(line["found"], json!(["staged"]), "{}", seen.raw);
    assert_eq!(rig.rev("HEAD~1"), head, "one commit");
    assert_eq!(rig.git(&["log", "-1", "--format=%s"]), SUBJECT);
    assert_eq!(rig.status(), "");
}

/// The human's ruling on the stage's one finding, as a batch of its own.
fn ruling() -> Value {
    json!([
        {"argv": ["ledger-set", "--run", RUN],
         "stdin": json!([{"key": "f-1", "disposition": "later", "detail": "next release"}]).to_string()},
        {"argv": ["check-ledger", "--run", RUN, "--", "f-1"]},
    ])
}

/// **A batch that changes nothing is refused at `apply`, before a journal exists** (the
/// re-review's `R2`, the way in without a kill): the same ruling sent twice was applied
/// twice, its commit step found nothing to commit, and the journal kept the batch — after
/// which every later record of the run was refused. A batch every file of which is what
/// the run holds already is no batch.
#[test]
fn a_batch_that_changes_nothing_is_refused_at_apply() {
    let stage = Stage::new("no-change");
    let rig = &stage.rig;
    stage.apply(SUBJECT);
    stage.record().done("record", "recorded");
    let apply = format!(
        "{RECORD} apply --run {RUN} --round 1 --subject 'docs(record): rc24 r1 — the rulings'"
    );
    let rule = |what: &str| {
        let (code, _, err) = rig.record_script(&apply, &ruling().to_string());
        assert_eq!(code, 0, "{what}: {err}");
        let scratch = rig.scratch.display().to_string();
        rig.step(&[
            "record",
            "--branch",
            LOOP,
            "--run-dir",
            RUN_DIR,
            "--gate",
            &stage.gate,
            "--calls",
            "2",
            "--checks",
            "1",
            "--scratch",
            &scratch,
        ])
        .done("record", "recorded");
    };
    rule("the ruling, recorded");
    let head = rig.rev("HEAD");

    // The same ruling, sent again.
    let (code, out, err) = rig.record_script(&apply, &ruling().to_string());
    assert_eq!(
        (code, out.as_str()),
        (25, ""),
        "the same batch again is refused: {err}"
    );
    assert!(
        err.starts_with("stabilize-record: refused no-change: ")
            && err.contains("nothing was written"),
        "{err}"
    );
    assert_eq!(
        (rig.journal(), rig.status(), rig.rev("HEAD")),
        (None, String::new(), head),
        "no journal, and nothing to commit"
    );
    // MUST NOT REFUSE: any later record of the run.
    let other = json!([
        {"argv": ["ledger-set", "--run", RUN],
         "stdin": json!([{"key": "f-1", "disposition": "later", "detail": "the release after"}]).to_string()},
        {"argv": ["check-ledger", "--run", RUN, "--", "f-1"]},
    ]);
    let (code, _, err) = rig.record_script(&apply, &other.to_string());
    assert_eq!(code, 0, "a later record is applied: {err}");
    // (Nothing of this stage was pushed: the read says that too, and leaves the push to
    // the record that is open.)
    assert_eq!(
        stage.git_state().done("git-state", "ready")["owed"],
        json!(["applied", "unpushed"])
    );
}

/// **`discard` asks git, and refuses a batch that is in a commit** (the re-review's `R2`):
/// taking a batch back writes every file as it was before the batch — committed or not. As
/// an act of the step tool it reads the batch's phase first: one that is applied is taken
/// back, with what the commit step staged of it unstaged; one whose commit is made is not,
/// and the refusal names what settles it. Asked twice, it says that nothing is left.
#[test]
fn discard_refuses_a_batch_that_is_in_a_commit() {
    let discard = ["discard", "--branch", LOOP, "--run-dir", RUN_DIR];

    // Committed and not settled: nothing is taken back.
    let stage = Stage::unsettled("discard-committed");
    let rig = &stage.rig;
    let (journal, status, tables) = (
        rig.journal(),
        rig.status(),
        rig.read(&format!("{RUN_DIR}/ledger.md")),
    );
    let seen = rig.step(&discard);
    let said = seen.refused("committed");
    assert!(
        said["halt"]["recommendation"]
            .as_str()
            .is_some_and(|then| then.contains("`dev/stabilize-step git-state`")),
        "the refusal names the read that settles it: {}",
        seen.raw
    );
    assert_eq!(
        (
            rig.journal(),
            rig.status(),
            rig.read(&format!("{RUN_DIR}/ledger.md"))
        ),
        (journal, status, tables),
        "the committed tables are as they were"
    );

    // Applied, and its paths staged by a commit that failed: taken back, and unstaged.
    let stage = Stage::new("discard-staged");
    let rig = &stage.rig;
    let before = stage.end();
    stage.apply(SUBJECT);
    let marker = failing_commit(rig);
    stage.record().refused("git");
    fs::remove_file(marker).expect("commits work again");
    let seen = rig.step(&discard);
    let line = seen.done("discard", "discarded");
    assert_eq!(line["found"], json!(["staged"]), "{}", seen.raw);
    assert!(
        line["discarded"]
            .as_array()
            .is_some_and(|files| files.contains(&json!(format!("{RUN_DIR}/ledger.md")))),
        "{}",
        seen.raw
    );
    assert_eq!(
        stage.end(),
        before,
        "the state before the batch: the tables, the pending reports, nothing staged"
    );
    stage.starts_again_as(2, "after the batch is taken back");

    // MUST NOT REFUSE: a second identical call. It reports that nothing is left.
    let seen = rig.step(&discard);
    let line = seen.done("discard", "discarded");
    assert_eq!(
        json!([line["found"], line["discarded"]]),
        json!([[], []]),
        "{}",
        seen.raw
    );
}

/// **A batch whose commit is made is not taken back because a hand wrote in it since** (the
/// core review's `F4`). `discard` refused a committed batch only where no file of it was
/// altered — and a file of an applied batch that a hand changed is refused with `discard`
/// spelled as the command that leaves it. So a batch that was in HEAD, and on the remote,
/// and one of whose tables a hand then touched, was taken back on the tool's own advice:
/// the round's tables left the tree, the journal was gone, and `state` answered `test` for
/// a round whose record the remote held. Whether a batch's commit is made is now ASKED OF
/// HEAD — every file of it is there as the batch wrote it — and of nothing the tree holds.
/// Such a batch is settled, never taken back; and the file is then what it is: a tracked
/// file changed by hand, the human's, with no word of `discard`.
#[test]
fn a_batch_whose_commit_is_made_is_not_taken_back_because_a_hand_wrote_in_it_since() {
    let discard = ["discard", "--branch", LOOP, "--run-dir", RUN_DIR];
    let table = format!("{RUN_DIR}/ledger.md");
    for first in ["discard", "git-state"] {
        // Committed, not settled — and PUSHED, by the push that reads no journal.
        let stage = Stage::unsettled(&format!("discard-altered-{first}"));
        let rig = &stage.rig;
        rig.step(&PUSH).done("push", "ready");
        let recorded = rig.rev("HEAD");
        assert_eq!(rig.remote(LOOP), Some(recorded.clone()));
        assert!(rig.journal().is_some(), "a push settles nothing");
        // A hand in a table of the record.
        let by_hand = format!("{}\n<!-- a hand was here -->\n", rig.read(&table));
        rig.write(&table, &by_hand);

        if first == "discard" {
            // THE ACT THE REFUSAL USED TO NAME, asked first: nothing is taken back.
            let (journal, status) = (rig.journal(), rig.status());
            let seen = rig.step(&discard);
            let said = seen.refused("committed");
            assert_eq!(said["found"], json!(["committed"]), "{}", seen.raw);
            assert!(
                said["halt"]["root_cause"]
                    .as_str()
                    .is_some_and(|cause| cause.contains(&table)),
                "the file a hand changed is named: {}",
                seen.raw
            );
            assert_eq!(
                (rig.journal(), rig.status(), rig.read(&table)),
                (journal, status, by_hand.clone()),
                "nothing was written: the journal, the tree and the table are as they were"
            );
        }

        // THE READ A STAGE STARTS FROM settles the batch — its commit is made — and names
        // the file as what it then is: a tracked file changed by hand.
        let seen = stage.git_state();
        let said = seen.refused("dirty");
        assert_eq!(
            json!([said["found"], said["finished"]]),
            json!([["altered", "committed", "dirty"], ["committed"]]),
            "{}",
            seen.raw
        );
        let then = said["halt"]["recommendation"]
            .as_str()
            .expect("what leaves it");
        assert!(
            then.contains("the human") && !then.contains("discard"),
            "it is the human's, and no act that takes a batch back is named: {then}"
        );
        assert!(rig.journal().is_none(), "the batch is settled");
        assert_eq!(rig.read(&table), by_hand, "the tree is as the hand left it");
        assert_eq!(
            (rig.rev("HEAD"), rig.remote(LOOP)),
            (recorded.clone(), Some(recorded.clone())),
            "and the commit, here and on the remote, is the record's"
        );

        // `discard` then has no batch, and takes nothing back.
        let seen = rig.step(&discard);
        let line = seen.done("discard", "discarded");
        assert_eq!(
            json!([line["found"], line["discarded"]]),
            json!([[], []]),
            "{}",
            seen.raw
        );
        assert_eq!(rig.read(&table), by_hand);

        // THE HUMAN'S ACT, plain git: what the commit holds is put back — and the run
        // stands at its record, a tested round with its candidate.
        rig.git(&["restore", "--", &table]);
        let seen = stage.git_state();
        assert_eq!(seen.done("git-state", "ready")["found"], json!([]));
        let end = stage.end();
        assert_eq!((end.head, end.status.as_str()), (recorded, ""));
        assert!(
            end.state["candidate"]["commit"].is_string() && end.state["next"] != "test",
            "the state reads the round's record: {}",
            end.state
        );
    }
}

/// **A temporary is no report** (the re-review's `R4`): a writer of the record script that
/// is killed leaves its temporary beside its target. The report check counted one as a
/// file nobody launched, and the read a stage starts from refused the tree as dirty, naming
/// no command. A temporary is the script's own: its check does not count one, and every
/// act of the step tool that reads the run's pending writes has the script remove them
/// first.
#[test]
fn a_temporary_is_no_report() {
    let stage = Stage::new("temporary");
    let rig = &stage.rig;
    let beside_reports = format!("{RUN_DIR}/r1/reports/test/.stabilize-record.killed.tmp");
    let beside_tables = format!("{RUN_DIR}/.stabilize-record.killed.tmp");

    // The record script's own check: a temporary is neither a report nor an extra file.
    rig.write(&beside_reports, "half a report\n");
    let mut check = format!("check-reports --run {RUN} --round 1 --stage test --attempt 1 --");
    for reporter in LAUNCHED {
        check.push_str(&format!(" {reporter}"));
    }
    let said: Value = serde_json::from_str(&rig.wrote(&check, "")).expect("the check's result");
    assert_eq!(
        json!([said["ok"], said["extra"]]),
        json!([true, []]),
        "{said}"
    );

    // The step's report check removes it, and says so.
    let seen = stage.check();
    let line = seen.done("check-reports", "checked");
    assert_eq!(
        json!([line["check"]["ok"], line["finished"]]),
        json!([true, ["stray-temporaries"]]),
        "{}",
        seen.raw
    );
    assert!(!rig.root.join(&beside_reports).exists());

    // And so does the read a stage starts from, for one beside the tables.
    rig.write(&beside_tables, "half a table\n");
    rig.write(&beside_reports, "half a report\n");
    let seen = stage.git_state();
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([line["found"], line["finished"]]),
        json!([["stray-temporaries"], ["stray-temporaries"]]),
        "{}",
        seen.raw
    );
    assert!(!rig.root.join(&beside_tables).exists() && !rig.root.join(&beside_reports).exists());
    stage.starts_again_as(2, "after the temporaries are gone");
}

/// **A branch that moved under the stage is refused** (the lead *a commit made on the loop
/// branch while a stage runs*): the commit step is handed the commit the stage began on
/// (`--head`), and a record is committed on that commit or not at all — else a commit
/// nobody gated beside this record would be published by the record's push. Nothing is
/// committed or undone; the stage invoked again meets the commit as one that is not
/// pushed, and then gates the batch on the tree as it stands.
#[test]
fn a_branch_that_moved_under_the_stage_is_refused() {
    let stage = Stage::new("head-moved");
    let rig = &stage.rig;
    let began = rig.rev("HEAD");
    stage.apply(SUBJECT);
    let pending = rig.status();
    // Somebody's commit, made while the stage's instruments ran: one file, by name.
    rig.write("README.md", "tuned\n");
    rig.git(&["add", "--", "README.md"]);
    rig.git(&["commit", "-q", "-m", "build(dev): a tuning commit"]);
    let tuning = rig.rev("HEAD");
    assert_eq!(rig.status(), pending, "the batch is as it was applied");

    let seen = stage.record_with(&["--head", &began]);
    let said = seen.refused("head-moved");
    assert!(
        said["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(&began) && cause.contains(&tuning)),
        "both commits are named: {}",
        seen.raw
    );
    assert_eq!(
        (rig.rev("HEAD"), rig.status()),
        (tuning.clone(), pending),
        "nothing was committed or undone"
    );
    // MUST NOT REFUSE: the head the stage began on.
    let seen = stage.record_with(&["--head", &tuning]);
    seen.done("record", "recorded");
    assert_eq!(rig.rev("HEAD~1"), tuning);
}

/// **An applied batch a hand changed is the human's — and the refusal names the command**
/// (the plan's cell *a file of the batch changed since*): the one arrival state of the
/// record act the tool neither finishes nor undoes. Driven as the refusal spells it: the
/// act that takes the batch back, and then a tree a stage starts from.
#[test]
fn an_applied_batch_a_hand_changed_is_the_humans_and_the_refusal_names_what_leaves_it() {
    let stage = Stage::new("altered");
    let rig = &stage.rig;
    let before = stage.end();
    stage.apply(SUBJECT);
    let ledger = format!("{RUN_DIR}/ledger.md");
    rig.write(&ledger, &rig.read(&ledger).replace("f-1", "f-2"));
    for seen in [stage.git_state(), stage.record()] {
        let said = seen.refused("dirty");
        let then = said["halt"]["recommendation"]
            .as_str()
            .expect("what leaves it");
        assert!(
            said["halt"]["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.contains(&ledger))
                && then.contains("the human's"),
            "the file is named, and whose it is: {}",
            seen.raw
        );
        assert_eq!(
            named_command(then, "dev/stabilize-step discard"),
            format!("dev/stabilize-step discard --branch {LOOP} --run-dir {RUN_DIR}"),
            "and the command that takes the batch back: {then}"
        );
    }
    let then = stage.git_state();
    let then = then.line["halt"]["recommendation"]
        .as_str()
        .expect("what leaves it")
        .to_owned();
    rig.shell(named_command(&then, "dev/stabilize-step discard"))
        .done("discard", "discarded");
    assert_eq!(stage.end(), before, "the state before the batch");
    stage.starts_again_as(2, "after the batch is taken back");
}

/// **A second identical call finds what is done, and says so** — the must-not-refuse
/// controls of a repeatable act. An ordinary record, and then each command of it once
/// more with the same arguments: the commit step answers `recorded` again with the commit
/// it made, the batch it committed and the gate it was held to (`found: recorded`) where
/// it is handed the commit the stage began on — which is what says that the one commit
/// since is this record's; the push publishes nothing and says where the remote stands;
/// the read finds nothing. A commit step whose batch is committed and not settled answers
/// `recorded` too (`found: committed`).
#[test]
fn a_second_identical_call_finds_what_is_done() {
    let stage = Stage::new("again");
    let rig = &stage.rig;
    let began = rig.rev("HEAD");
    stage.apply(SUBJECT);
    let first = stage.record_with(&["--head", &began]);
    let first = first.done("record", "recorded").clone();
    assert_eq!(first["found"], json!(["applied"]), "an ordinary record");
    let commit = rig.rev("HEAD");

    let again = stage.record_with(&["--head", &began]);
    let line = again.done("record", "recorded");
    assert_eq!(
        json!([
            line["found"],
            line["commit"],
            line["applied"],
            line["gate"],
            line["paths"],
            line["round"]
        ]),
        json!([
            ["recorded"],
            commit,
            first["applied"],
            first["gate"],
            first["paths"],
            first["round"]
        ]),
        "the same answer, and that it was found done: {}",
        again.raw
    );
    assert_eq!(rig.rev("HEAD"), commit, "no second commit");
    // Without the commit the stage began on, nothing says the last commit is this record's.
    stage.record().refused("no-batch");
    // Nor does a record of another stage's answer for this one: another head.
    stage.record_with(&["--head", &commit]).refused("no-batch");

    let pushed = rig.step(&PUSH);
    assert_eq!(pushed.done("push", "ready")["vetted"], json!([commit]));
    let pushed = rig.step(&PUSH);
    let line = pushed.done("push", "ready");
    assert_eq!(
        json!([line["vetted"], line["remote_head"]]),
        json!([[], commit]),
        "{}",
        pushed.raw
    );
    for _ in 0..2 {
        let seen = stage.git_state();
        let line = seen.done("git-state", "ready");
        assert_eq!(
            json!([
                line["found"],
                line["finished"],
                line["owed"],
                line["pending"],
                line["remote_head"]
            ]),
            json!([[], [], [], null, commit]),
            "{}",
            seen.raw
        );
    }

    // Committed and not settled: the commit step settles it and answers as recorded.
    let stage = Stage::unsettled("again-committed");
    let commit = stage.rig.rev("HEAD");
    let seen = stage.record();
    let line = seen.done("record", "recorded");
    assert_eq!(
        json!([
            line["found"],
            line["commit"],
            line["applied"]["calls"],
            line["gate"]["ok"]
        ]),
        json!([["committed"], commit, 7, true]),
        "{}",
        seen.raw
    );
    assert_eq!(
        (
            stage.rig.journal(),
            stage.rig.rev("HEAD"),
            stage.rig.status()
        ),
        (None, commit, String::new()),
        "settled, and no second commit"
    );
}

/// **A kept answer is believed only for the commit and the batch it names** (the core
/// review's `F8`). A commit step asked again answers `recorded` from the answer it kept
/// beside its gate's file where four things hold: it is handed the commit the stage began
/// on, the calls and checks are the ones asked, the branch and THE COMMIT are the ones the
/// file names, and HEAD's one parent is that commit. The code was right; but with the
/// commit's equality dropped, and with the calls and checks' dropped, the whole suite stayed
/// green — two of four conditions were held by no test. Each is driven here: every other
/// condition holds, and the step still answers `no-batch`.
#[test]
fn a_kept_answer_is_believed_only_for_the_commit_and_the_batch_it_names() {
    let stage = Stage::new("again-kept");
    let rig = &stage.rig;
    let began = rig.rev("HEAD");
    stage.apply(SUBJECT);
    stage
        .record_with(&["--head", &began])
        .done("record", "recorded");
    let commit = rig.rev("HEAD");
    // The control: the same step, asked again, is believed.
    let again = stage.record_with(&["--head", &began]);
    assert_eq!(
        again.done("record", "recorded")["found"],
        json!(["recorded"])
    );

    // THE CALLS AND THE CHECKS: the answer of a batch of seven calls and two checks is
    // none for a step that names another batch — fewer, more, or other checks.
    let scratch = rig.scratch.display().to_string();
    for (calls, checks) in [("6", "2"), ("8", "2"), ("7", "1"), ("7", "3")] {
        let seen = rig.step(&[
            "record",
            "--branch",
            LOOP,
            "--run-dir",
            RUN_DIR,
            "--gate",
            stage.gate.as_str(),
            "--calls",
            calls,
            "--checks",
            checks,
            "--scratch",
            scratch.as_str(),
            "--head",
            &began,
        ]);
        seen.refused("no-batch");
        assert_eq!(rig.rev("HEAD"), commit, "nothing was committed");
    }

    // THE COMMIT: another commit on the commit the stage began on — one parent, that one,
    // the tree clean, the same branch — is not the commit the kept answer names.
    rig.git(&["reset", "-q", "--hard", &began]);
    let note = format!("{RUN_DIR}/another.md");
    rig.write(&note, "another record\n");
    rig.git(&["add", "--", &note]);
    rig.git(&[
        "commit",
        "-q",
        "-m",
        "docs(record): another commit on the same parent",
    ]);
    let other = rig.rev("HEAD");
    assert_ne!(other, commit);
    assert_eq!(rig.rev("HEAD^"), began);
    let seen = stage.record_with(&["--head", &began]);
    let said = seen.refused("no-batch");
    assert!(
        said["commit"].is_null() && said["found"] == json!([]),
        "nothing of the kept answer is said for another commit: {}",
        seen.raw
    );
    assert_eq!(rig.rev("HEAD"), other, "nothing was committed");
}

/// **The push a record is owed is left while a batch is applied** (the core review's eighth
/// lead: the guard removed, four tests stayed green). An earlier record that is committed
/// and not pushed is `unpushed`; with a batch applied on top the push is that record's own
/// — its commit step's, after the gate — and the read a stage starts from leaves it owed.
#[test]
fn the_push_a_record_is_owed_is_left_while_a_batch_is_applied() {
    let stage = Stage::new("owed-applied");
    let rig = &stage.rig;
    let pushed = rig.remote(LOOP);
    let note = format!("{RUN_DIR}/earlier.md");
    rig.write(&note, "an earlier record\n");
    rig.git(&["add", "--", &note]);
    rig.git(&["commit", "-q", "-m", "docs(record): an earlier record"]);
    stage.apply(SUBJECT);
    let seen = stage.git_state();
    let line = seen.done("git-state", "ready");
    assert_eq!(
        json!([line["found"], line["owed"], line["finished"]]),
        json!([["applied", "unpushed"], ["applied", "unpushed"], []]),
        "{}",
        seen.raw
    );
    assert_eq!(rig.remote(LOOP), pushed, "nothing was pushed");
    assert!(
        !seen.trace.iter().any(|call| call.starts_with("push ")),
        "no push was tried: {:?}",
        seen.trace
    );
}

/// Every arrival state of the tool's table, and the test of this suite that drives it.
const DRIVEN: &[(&str, &str)] = &[
    (
        "wrong-branch",
        "the_wrong_branch_is_refused_before_any_write_fetch_or_push",
    ),
    ("git-lock", "a_lock_git_left_is_named_and_not_removed"),
    (
        "interrupted-write",
        "a_record_step_killed_between_its_commands_ends_in_one_of_two_states_while_its_batch_is_applied",
    ),
    ("stray-temporaries", "a_temporary_is_no_report"),
    ("applied", "looking_finishes_nothing_and_says_what_is_owed"),
    (
        "staged",
        "a_commit_that_failed_is_asked_for_again_and_commits",
    ),
    (
        "altered",
        "an_applied_batch_a_hand_changed_is_the_humans_and_the_refusal_names_what_leaves_it",
    ),
    (
        "head-moved",
        "a_branch_that_moved_under_the_stage_is_refused",
    ),
    ("committed", "discard_refuses_a_batch_that_is_in_a_commit"),
    ("recorded", "a_second_identical_call_finds_what_is_done"),
    (
        "unpushed",
        "a_record_the_remote_lacks_is_pushed_before_a_stage_starts",
    ),
    (
        "unvetted",
        "a_record_the_remote_lacks_is_pushed_before_a_stage_starts",
    ),
    (
        "foreign-commit",
        "a_commit_that_is_no_records_is_not_pushed_by_the_tool",
    ),
    (
        "remote-ahead",
        "git_state_refuses_a_pushed_loop_branch_that_is_ahead",
    ),
    (
        "dirty",
        "git_state_refuses_a_tree_that_holds_more_than_reports",
    ),
];

/// **The table of acts and arrivals is the tool's own, printed by a read, and every cell of
/// it is driven** (the plan's `P1`: *one table, keyed by act and phase*). `dev/stabilize-step
/// table` prints it as data: every act of the tool with the half that owns it and — for an
/// act the tool reconciles — its phases in order, each with what says it is done and the
/// step that resumes it; and every state an act can be found in on arrival, keyed by the
/// act and the phase it stands before, with the tool's answer, the word of a refusal, whose
/// it is and what leaves it. Held here: every act the parser takes has its row, and an act
/// of the `fix` half is marked as that half's and given no phase by this one; every word is
/// a refusal of the tool; and the arrivals are exactly [`DRIVEN`], each by a test of this
/// file — so a cell added to the table without its test is red.
#[test]
fn the_table_names_every_act_and_every_arrival_and_each_is_driven() {
    let rig = StepRig::new("table");
    let seen = rig.step(&["table"]);
    let table = seen.done("table", "listed");
    assert!(seen.trace.is_empty(), "the read asks git nothing");

    // The acts: the parser's own, each with its half; the phases of `record`, in order.
    let rows: BTreeMap<String, &Value> = table["acts"]
        .as_array()
        .expect("the acts")
        .iter()
        .map(|act| (act["act"].as_str().expect("an act's name").to_owned(), act))
        .collect();
    assert_eq!(
        rows.keys().cloned().collect::<BTreeSet<_>>(),
        acts().into_keys().collect::<BTreeSet<_>>(),
        "the table's acts (left) are the acts the tool takes (right)"
    );
    // The acts the table gives phases are the acts a kill is driven between ([`KILLED`]).
    let phased: Vec<&str> = table["acts"]
        .as_array()
        .expect("the acts")
        .iter()
        .filter(|act| act["phases"].is_array())
        .flat_map(|act| act["commands"].as_array().expect("an act's commands"))
        .map(|command| command.as_str().expect("a command"))
        .collect();
    assert_eq!(
        phased, KILLED,
        "the commands of the acts the table gives phases (left) are the commands this suite kills (right): an act that gains phases gains its kills"
    );
    let phases: Vec<&str> = rows["record"]["phases"]
        .as_array()
        .expect("the record act has phases")
        .iter()
        .map(|phase| {
            for field in ["done", "resumes", "by"] {
                assert!(phase[field].is_string(), "a phase says `{field}`: {phase}");
            }
            phase["phase"].as_str().expect("a phase's name")
        })
        .collect();
    assert_eq!(phases, ["applied", "committed", "settled", "pushed"]);
    for act in ["land", "carry", "open-round"] {
        assert_eq!(
            json!([rows[act]["half"], rows[act]["phases"]]),
            json!(["fix", null]),
            "`{act}` is the `fix` half's, and this half gives it no phase"
        );
    }

    // The arrivals: keyed by act and phase, answered, and each driven by name.
    let source = read("tooling-tests/dev_stabilize_step.rs");
    let arrivals = table["arrivals"].as_array().expect("the arrivals");
    let mut named = BTreeSet::new();
    for arrival in arrivals {
        let state = arrival["state"].as_str().expect("a state's name");
        assert!(named.insert(state.to_owned()), "`{state}` stands once");
        for field in ["act", "found", "whose", "leaves"] {
            assert!(
                arrival[field].is_string(),
                "`{state}` says `{field}`: {arrival}"
            );
        }
        let act = arrival["act"].as_str().expect("the act");
        assert!(
            act == "*" || rows.contains_key(act),
            "`{state}` is keyed by an act of the table: {act}"
        );
        if act != "*" {
            let known: Vec<&str> = rows[act]["phases"]
                .as_array()
                .expect("an act with arrivals has phases")
                .iter()
                .map(|phase| phase["phase"].as_str().expect("a phase"))
                .collect();
            assert!(
                known.contains(&arrival["phase"].as_str().expect("a phase")),
                "`{state}` is keyed by a phase of `{act}`: {arrival}"
            );
        }
        let answer = arrival["answer"].as_str().expect("an answer");
        assert!(
            ["refused", "finished", "undone", "owed", "answered"].contains(&answer),
            "`{state}`: {answer}"
        );
        assert_eq!(
            arrival["word"].is_string(),
            answer == "refused",
            "`{state}`: a refusal has its word, and nothing else has one"
        );
        if let Some(word) = arrival["word"].as_str() {
            assert!(
                REFUSALS.iter().any(|(known, _)| *known == word),
                "`{state}` is refused with a word of the tool: {word}"
            );
        }
    }
    // What the tool writes, both outside the repository: one row per act that writes.
    let kept: Vec<(&str, &str)> = table["kept"]
        .as_array()
        .expect("the kept files")
        .iter()
        .map(|kept| {
            for field in ["written", "read", "believed"] {
                assert!(
                    kept[field].is_string(),
                    "a kept file says `{field}`: {kept}"
                );
            }
            (
                kept["act"].as_str().expect("the act that writes it"),
                kept["file"].as_str().expect("the file"),
            )
        })
        .collect();
    assert_eq!(
        kept,
        [
            ("state", "<scratch>/state/<tag>.json"),
            ("record", "<gate file>.recorded.json"),
            ("*", "<digest root>/lines/<act>.<16 hex>.json"),
            ("hold-start", "<scratch>/hold/<name>/job.json"),
            ("hold-start", "<scratch>/hold/<name>/lock"),
            ("hold-start", "<scratch>/hold/<name>/output"),
            ("hold-start", "<scratch>/hold/<name>/exit.json"),
            ("hold-wait", "<scratch>/hold/<name>/verdict.json"),
            ("build", "<scratch>/<work>/"),
            ("build", "<scratch>/<to>"),
        ],
        "the files the tool writes: the state document, a commit step's kept answer, for an act asked for its digest the line the digest leaves out — and what a held command leaves under the scratch root: its record, its lock, its output, how it ended, the tool's verdict, and a build's work and its binary"
    );
    let driven: BTreeSet<String> = DRIVEN
        .iter()
        .map(|(state, _)| (*state).to_owned())
        .collect();
    assert_eq!(
        named, driven,
        "the table's arrival states (left) are the ones this suite drives (right)"
    );
    for (state, test) in DRIVEN {
        assert!(
            source.contains(&format!("\nfn {test}()")),
            "`{state}` is driven by `{test}`, which is a test of this file"
        );
    }
}

// ---------------------------------------------------------------------------
// The names an act takes
// ---------------------------------------------------------------------------

/// The acts of the tool and the flags each requires, read from its own parser.
fn acts() -> BTreeMap<String, Vec<String>> {
    let source = read(TOOL);
    let mut acts = BTreeMap::new();
    for line in source
        .lines()
        .filter_map(|line| line.strip_prefix("    act(\""))
    {
        let name = &line[..line.find('"').expect("the act's name closes")];
        let flags = &line[line.find('[').expect("an act lists its flags") + 1..];
        let flags = &flags[..flags.find(']').expect("the list closes")];
        acts.insert(
            name.to_owned(),
            flags
                .split('"')
                .skip(1)
                .step_by(2)
                .map(str::to_owned)
                .collect(),
        );
    }
    assert!(acts.len() >= 10, "the scan found the tool's acts: {acts:?}");
    acts
}

/// A value each required flag takes.
fn valid(flag: &str, scratch: &str) -> String {
    match flag {
        "loop" => LOOP.to_owned(),
        "branch" | "prefix" => format!("{ROUNDS}1"),
        "rounds" => ROUNDS.to_owned(),
        "run-dir" => RUN_DIR.to_owned(),
        "run" => RUN.to_owned(),
        "tag" => "test-1".to_owned(),
        "scratch" => scratch.to_owned(),
        "stage" => "test".to_owned(),
        "round" | "attempt" | "calls" | "checks" => "1".to_owned(),
        "reporter" => "attempt".to_owned(),
        "commit" => "c".repeat(40),
        "gate" => format!("{scratch}/gate.txt"),
        other => panic!("the suite has no value for the flag `--{other}`: give it one"),
    }
}

#[test]
fn no_act_takes_main_or_any_branch_without_a_prefix() {
    let rig = StepRig::new("names");
    let scratch = rig.scratch.display().to_string();
    let (main, head) = (rig.remote("main"), rig.rev("HEAD"));
    let mut driven = 0;
    for (act, flags) in acts() {
        for naming in flags
            .iter()
            .filter(|flag| ["loop", "branch", "rounds", "prefix"].contains(&flag.as_str()))
        {
            for hostile in ["main", "HEAD", "+fix/rc24", "fix/rc24:main", "-f", "fix/"] {
                let mut args = vec![act.clone()];
                for flag in &flags {
                    args.push(format!("--{flag}"));
                    args.push(if flag == naming {
                        hostile.to_owned()
                    } else {
                        valid(flag, &scratch)
                    });
                }
                if act == "carry" {
                    args.push(head.clone());
                }
                let args: Vec<&str> = args.iter().map(String::as_str).collect();
                let seen = rig.step(&args);
                seen.refused("usage");
                assert!(
                    seen.trace.is_empty(),
                    "`{act} --{naming} {hostile}` ran git: {:?}",
                    seen.trace
                );
                driven += 1;
            }
        }
    }
    assert!(
        driven >= 70,
        "every naming flag of every act was driven: {driven}"
    );
    assert_eq!(rig.remote("main"), main, "main is where it was");
    assert_eq!(rig.rev("HEAD"), head);
    assert_eq!(rig.branch(), LOOP);
}

#[test]
fn the_tools_header_its_statuses_and_this_suites_table_agree() {
    let source = read(TOOL);
    // The header's table: `#   NN  word   …`.
    let header: BTreeMap<String, i32> = source
        .lines()
        .take_while(|line| line.starts_with('#'))
        .filter_map(|line| {
            let mut words = line.trim_start_matches('#').split_whitespace();
            let status: i32 = words.next()?.parse().ok()?;
            let word = words.next()?;
            (status >= 2).then(|| (word.to_owned(), status))
        })
        .collect();
    // The code's: `    "word": NN,`.
    let at = source.find("\nSTATUS = {\n").expect("the tool's statuses");
    let table = &source[at..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    let code: BTreeMap<String, i32> = table
        .lines()
        .filter_map(|line| line.trim().strip_prefix('"'))
        .map(|line| {
            let (word, status) = line.split_once("\": ").expect("a word and its status");
            (
                word.to_owned(),
                status.trim_end_matches(',').parse().expect("a status"),
            )
        })
        .collect();
    let suite: BTreeMap<String, i32> = REFUSALS
        .iter()
        .map(|(word, status)| ((*word).to_owned(), *status))
        .collect();
    assert_eq!(
        header, code,
        "the header's refusals (left) and the code's (right)"
    );
    assert_eq!(
        code, suite,
        "the code's refusals (left) and this suite's (right)"
    );
    let statuses: BTreeSet<i32> = code.values().copied().collect();
    assert_eq!(
        statuses.len(),
        code.len(),
        "a status per refusal, and no two alike"
    );

    // `--help` prints that header, and an act that is none is refused in one line.
    let rig = StepRig::new("usage");
    let help = rig
        .hermetic(Command::new(rig.root.join(TOOL)))
        .arg("--help")
        .output()
        .expect("run the tool");
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).starts_with("\ndev/stabilize-step — "));
    rig.step(&["rebase"]).refused("usage");
    rig.step(&[]).refused("usage");
}

// ---------------------------------------------------------------------------
// The tool can do nothing an agent may not: the source scan
// ---------------------------------------------------------------------------

/// The flags a git subcommand of the tool may carry. A subcommand that is not here is one
/// the tool does not run — `rebase`, `reset`, `pull`, `stash`, `tag`, `checkout` — and a
/// flag that is not here is one it does not pass: `--force`, `-D`, `--delete`, `--hard`.
const ALLOWED: &[(&str, &[&str])] = &[
    (
        "branch",
        &["--show-current", "--list", "--format=%(refname:short)"],
    ),
    ("status", &["--porcelain", "--untracked-files=all"]),
    ("ls-remote", &["--exit-code", "--heads"]),
    ("fetch", &[]),
    ("merge-base", &["--is-ancestor"]),
    (
        "log",
        &[
            "--diff-filter=A",
            "--format=%H",
            "--format=%s",
            "-n",
            "--first-parent",
            "--no-merges",
            "--merges",
            "--reverse",
            "--",
        ],
    ),
    // `--name-only --format=`: the paths a record's commit holds, where the commit step
    // finds its batch in HEAD already.
    (
        "show",
        &["--stat", "--format=%s", "--name-only", "--format=", "--"],
    ),
    // `--git-path`: where git's own lock would lie, which the tool names and never removes.
    ("rev-parse", &["--verify", "--quiet", "--git-path"]),
    (
        "rev-list",
        &[
            "--merges",
            "--count",
            "--parents",
            "-n",
            "--reverse",
            // Which of the commits the remote's heads stand at this clone holds too: what
            // a range leaves out is what the remote said, and no remote-tracking ref.
            "--no-walk",
            "--ignore-missing",
        ],
    ),
    ("diff", &["--name-only", "--diff-filter=U", "--quiet", "--"]),
    ("switch", &["--no-track", "-c"]),
    ("push", &[]),
    ("merge", &["--no-ff", "--no-edit", "--abort"]),
    ("commit", &["--no-edit", "-q", "-m"]),
    ("add", &["--"]),
    ("restore", &["--staged", "--"]),
    ("cherry-pick", &["-x", "--abort"]),
    // A commit, unpacked for a build: the archive of it, as a file under the scratch root.
    ("archive", &["--format=tar", "-o"]),
];

/// Every place the tool opens a file, as the line spells it, and how often: its own source
/// for `--help`; the state it keeps under the scratch root and the line a digest leaves
/// out (both `path`, written); a commit step's kept answer, read and written; the file a
/// digest names, read back for its hash; and A HELD COMMAND'S FILES — read whole by one
/// helper, written once by another, the supervisor's two streams, the two locks, a
/// build's archive, cargo's output and the binary, which is created and never replaced.
const OPENS: &[(&str, usize)] = &[
    ("for line in open(__file__, encoding=\"utf-8\").read()", 1),
    ("with open(path, \"w\", encoding=\"utf-8\") as file:", 2),
    (
        "json.loads(open(answered_before(args), encoding=\"utf-8\").read())",
        1,
    ),
    (
        "with open(answered_before(args), \"w\", encoding=\"utf-8\") as file:",
        1,
    ),
    ("with open(path, \"rb\") as file:", 1),
    ("with open(found, \"rb\") as file:", 1),
    ("with open(at, \"x\", encoding=\"utf-8\") as file:", 1),
    (
        "out = os.open(os.path.join(home, \"output\"), os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)",
        1,
    ),
    ("null = os.open(os.devnull, os.O_RDWR)", 1),
    ("mutex = os.open(holds, os.O_RDONLY)", 2),
    (
        "lock = os.open(os.path.join(home, \"lock\"), os.O_RDONLY)",
        1,
    ),
    (
        "lock = os.open(os.path.join(home, \"lock\"), os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)",
        1,
    ),
    ("with tarfile.open(tarball) as archive:", 1),
    (
        "with open(os.path.join(work, \"cargo.log\"), \"x\", encoding=\"utf-8\") as log:",
        1,
    ),
    (
        "with os.fdopen(os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o555), \"wb\") as file:",
        1,
    ),
];

/// **Every file the tool opens THROUGH ITS OWN TWO HELPERS** (the core review's `F7`):
/// `whole` reads the path it is handed and `kept_once` writes the one it is handed, so
/// [`OPENS`] — which lists each helper's own line, once — says nothing about WHICH files
/// they open. This does: every call of either, by the function it stands in and the path
/// as written. A caller that is not here is a file the tool opens somewhere else, and a
/// row whose call is gone is a file it no longer reads or writes; both are red.
const THROUGH: &[(&str, &str, &str)] = &[
    // What a held command leaves, written once each: the job, how it ended, its verdict.
    (
        "kept_once",
        "hold_start",
        "os.path.join(home, \"job.json\")",
    ),
    (
        "kept_once",
        "supervise",
        "os.path.join(home, \"exit.json\")",
    ),
    ("kept_once", "stands", "kept"),
    // And read back: the job, how it ended, the verdict; the command's output, by the
    // reader of its kind; the binary a build made, and the one a verdict hashes again;
    // a file whose hash is asked of the tool.
    ("whole", "read_job", "os.path.join(home, \"job.json\")"),
    ("whole", "stands", "os.path.join(home, \"exit.json\")"),
    ("whole", "stands", "kept"),
    // The command's output once more, hashed into the verdict that is kept of it.
    ("whole", "stands", "output"),
    ("whole", "judged_regression", "output"),
    ("whole", "judged_build", "output"),
    ("whole", "judged_build", "path"),
    ("whole", "judged_probe", "output"),
    ("whole", "build", "made"),
    ("whole", "hash_file", "path"),
];

/// The commands a held kind may run, as `held_command` returns them: each opens with one of
/// the tool's own constants — the gate, the regression set, this tool, the probe tool.
const HELD_PROGRAMS: [&str; 4] = [
    "return [GATE, \"--keep-going\"]",
    "return [REGRESSION, \"run\", \"--previous\", ",
    "return [BUILD, \"build\", \"--scratch\", ",
    "return [PROBE, \"hold\", \"--scratch\", ",
];

/// The subcommands that change a branch, a commit or a remote, and every shape the tool
/// may give one: a literal stands for itself, `<…>` for one of the names below.
const CHANGING: &[(&str, &[&str])] = &[
    ("fetch", &["fetch origin <branch>", "fetch origin main"]),
    (
        "switch",
        &["switch <branch>", "switch --no-track -c <branch> <branch>"],
    ),
    ("push", &["push origin <branch>"]),
    (
        "merge",
        &["merge --no-ff --no-edit <target>", "merge --abort"],
    ),
    ("commit", &["commit --no-edit", "commit -q -m <subject>"]),
    ("add", &["add -- <paths>"]),
    // What the record's commit step staged and does not commit, taken out of the index
    // again: the index, never the tree, and those paths only.
    ("restore", &["restore --staged -- <paths>"]),
    (
        "cherry-pick",
        &["cherry-pick -x <sha>", "cherry-pick --abort"],
    ),
];

/// The names a changing command may take: an argument of the branch grammar, or the
/// parameter of a helper whose every caller passes one.
const BRANCH_NAMES: [&str; 3] = ["args.loop", "args.branch", "branch"];

/// The pieces of text the tool may join a name to, inside a git call.
const JOINED: [&str; 6] = [
    "origin/",
    "refs/heads/",
    "*",
    "/opening.md",
    ":(exclude)",
    // A file of the applied batch as HEAD holds it: whether the batch's commit is made.
    "HEAD:",
];

#[derive(Debug, Clone, PartialEq)]
enum Arg {
    /// One string literal, and nothing else.
    Literal(String),
    /// Anything else, as written.
    Expression(String),
}

#[derive(Debug)]
struct Call {
    /// The top-level function the call stands in.
    within: String,
    name: String,
    args: Vec<Arg>,
}

/// The tool's code: no comment line.
fn python_code(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| format!("{line}\n"))
        .collect()
}

/// The string literals of a piece of python: double-quoted, as the tool writes them.
fn literals(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '"' {
            let mut literal = String::new();
            while let Some(c) = chars.next() {
                match c {
                    '\\' => literal.extend(chars.next()),
                    '"' => break,
                    c => literal.push(c),
                }
            }
            found.push(literal);
        }
    }
    found
}

/// Every call of a function named in `names`, with its arguments split at the top level.
fn calls(code: &str, names: &[&str]) -> Vec<Call> {
    let mut found = Vec::new();
    let mut within = String::new();
    let mut offset = 0;
    for line in code.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("def ") {
            within = rest[..rest.find('(').expect("a def has parameters")].to_owned();
        }
        for name in names {
            let needle = format!("{name}(");
            let mut from = 0;
            while let Some(at) = line[from..].find(&needle) {
                let start = from + at;
                from = start + needle.len();
                let before = line[..start].chars().next_back();
                if before.is_some_and(|c| c.is_alphanumeric() || c == '_' || c == '.')
                    || line[..start].ends_with("def ")
                {
                    continue;
                }
                // The arguments: up to the parenthesis that closes the call, which may
                // lie on a later line.
                let rest = &code[offset + from..];
                let mut args = Vec::new();
                let mut current = String::new();
                let (mut depth, mut quoted, mut escaped) = (1, false, false);
                for c in rest.chars() {
                    if quoted {
                        current.push(c);
                        if escaped {
                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == '"' {
                            quoted = false;
                        }
                        continue;
                    }
                    match c {
                        '"' => quoted = true,
                        '(' | '[' | '{' => depth += 1,
                        ')' | ']' | '}' => depth -= 1,
                        _ => {}
                    }
                    if depth == 0 {
                        break;
                    }
                    if c == ',' && depth == 1 {
                        args.push(std::mem::take(&mut current));
                    } else {
                        current.push(c);
                    }
                }
                args.push(current);
                let args = args
                    .iter()
                    .map(|arg| arg.trim())
                    .filter(|arg| !arg.is_empty())
                    .map(|arg| {
                        let inner = literals(arg);
                        let whole = arg.starts_with('"')
                            && arg.ends_with('"')
                            && inner.len() == 1
                            && arg.len() == inner[0].len() + 2;
                        if whole {
                            Arg::Literal(inner[0].clone())
                        } else {
                            Arg::Expression(arg.to_owned())
                        }
                    })
                    .collect();
                found.push(Call {
                    within: within.clone(),
                    name: (*name).to_owned(),
                    args,
                });
            }
        }
        offset += line.len();
    }
    found
}

/// The spellings of a forced push, as the definition of the agent that runs the steps
/// names them: the code spans of its *never forced (…)*.
fn forced_spellings() -> Vec<String> {
    let definition = read(GIT_DEFINITION);
    let at = definition
        .find("never forced (")
        .expect("build-git.md says what a forced push is spelled like");
    let list = &definition[at..];
    let list = &list[..list.find(')').expect("the list closes")];
    let spellings: Vec<String> = list
        .split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect();
    assert!(
        spellings.contains(&"--force".to_owned()) && spellings.contains(&"+".to_owned()),
        "the scan found the spellings: {spellings:?}"
    );
    spellings
}

/// Everything in the tool's source that an agent may not do — or nothing.
fn offences(source: &str) -> Vec<String> {
    let code = python_code(source);
    let mut found = Vec::new();
    let allowed: BTreeMap<&str, &[&str]> = ALLOWED.iter().copied().collect();
    let changing: BTreeMap<&str, &[&str]> = CHANGING.iter().copied().collect();

    // A forced push, in any spelling, in any string of the tool.
    let forced = forced_spellings();
    for literal in literals(&code) {
        for spelling in &forced {
            let hit = if spelling.len() == 1 {
                literal.starts_with(spelling.as_str())
            } else if spelling.starts_with("--") {
                literal.contains(spelling.as_str())
            } else {
                literal == *spelling
            };
            if hit {
                found.push(format!(
                    "the string \"{literal}\" spells a forced push (`{spelling}`)"
                ));
            }
        }
    }

    // Every git call: its subcommand, its flags, what it joins a name to, and — for a
    // command that changes something — its whole shape.
    for call in calls(&code, &["git", "said"]) {
        let shown = format!(
            "`{}({})` in `{}`",
            call.name,
            shape(&call.args),
            call.within
        );
        // `said` is `git` with a check on its exit status: it hands its own arguments on.
        if call.within == "said" && call.args == [Arg::Expression("*argv".to_owned())] {
            continue;
        }
        let Some(Arg::Literal(subcommand)) = call.args.first() else {
            found.push(format!(
                "{shown}: a git call whose subcommand is not a literal"
            ));
            continue;
        };
        let Some(flags) = allowed.get(subcommand.as_str()) else {
            found.push(format!("{shown}: the tool runs `git {subcommand}`"));
            continue;
        };
        for arg in &call.args[1..] {
            match arg {
                Arg::Literal(text) if text.starts_with('-') => {
                    if !flags.contains(&text.as_str()) {
                        found.push(format!("{shown}: `git {subcommand}` is passed `{text}`"));
                    }
                }
                Arg::Literal(text) if text.contains("main") => {
                    let fetched = subcommand == "fetch" && text == "main";
                    let read = text == "origin/main"
                        && ["rev-parse", "merge-base"].contains(&subcommand.as_str());
                    if call.within != "sync_main" || !(fetched || read) {
                        found.push(format!("{shown}: `main` outside the sync act's fetch and its reads of `origin/main`"));
                    }
                }
                Arg::Literal(_) => {}
                Arg::Expression(text) => {
                    for piece in literals(text) {
                        if !JOINED.contains(&piece.as_str()) {
                            found.push(format!("{shown}: a name is joined to \"{piece}\""));
                        }
                    }
                }
            }
        }
        if let Some(shapes) = changing.get(subcommand.as_str()) {
            let fits = shapes.iter().any(|allowed| {
                let allowed: Vec<&str> = allowed.split(' ').collect();
                allowed.len() == call.args.len()
                    && allowed
                        .iter()
                        .zip(&call.args)
                        .all(|(want, got)| match (*want, got) {
                            ("<branch>", Arg::Expression(name)) => {
                                BRANCH_NAMES.contains(&name.as_str())
                            }
                            ("<target>", Arg::Expression(name)) => name == "target",
                            ("<sha>", Arg::Expression(name)) => name == "sha",
                            // The record's commit: the subject its batch carries, and
                            // the run's pending paths, each by name.
                            ("<subject>", Arg::Expression(name)) => name == "subject",
                            ("<paths>", Arg::Expression(name)) => name == "*paths",
                            (literal, Arg::Literal(text)) => text == literal,
                            _ => false,
                        })
            });
            if !fits {
                found.push(format!(
                    "{shown}: `git {subcommand}` in a shape the tool does not have"
                ));
            }
        }
    }

    // A helper that takes a branch, or a merge's target, is handed a checked name by every
    // caller — the helpers derived from their `def` lines.
    let helpers: Vec<(String, bool)> = code
        .lines()
        .filter_map(|line| line.strip_prefix("def "))
        .filter_map(|rest| {
            let (name, parameters) = rest.split_once('(')?;
            let first = parameters.split([',', ')']).next()?.trim();
            ["branch", "target"]
                .contains(&first)
                .then(|| (name.to_owned(), first == "target"))
        })
        .collect();
    if helpers.len() < 3 {
        found.push(format!(
            "the scan found the helpers that take a branch: {helpers:?}"
        ));
    }
    for (helper, merges) in &helpers {
        for call in calls(&code, &[helper.as_str()]) {
            let handed = match call.args.first() {
                Some(Arg::Expression(text)) => {
                    ["args.loop", "args.branch"].contains(&text.as_str())
                }
                Some(Arg::Literal(text)) => {
                    *merges && text == "origin/main" && call.within == "sync_main"
                }
                None => false,
            };
            if !handed {
                found.push(format!(
                    "`{helper}({})` in `{}`: a name nobody checked",
                    shape(&call.args),
                    call.within
                ));
            }
        }
    }
    // A sha that is picked is one of the act's own arguments.
    if code.matches("for sha in ").count() != 2
        || !code.contains("    for sha in args.shas:\n")
        || !code.contains(" for sha in direct]")
    {
        found.push("`sha` is bound somewhere else than over the carry's arguments and the path-class assert's listing".to_owned());
    }
    // And the grammar those arguments are checked against requires a prefix.
    for required in [
        "BRANCH = re.compile(r\"[a-z0-9]+(?:-[a-z0-9]+)*(?:/[a-z0-9]+(?:-[a-z0-9]+)*)+\")\n",
        "branch_name = grammar(BRANCH, ",
        "    \"loop\": branch_name,\n",
        "    \"branch\": branch_name,\n",
        "full_sha = grammar(FULL_SHA, ",
        "names=(\"shas\", argparse.ONE_OR_MORE, full_sha)",
        "            sub.add_argument(\"--\" + flag, required=True, type=FLAGS[flag])\n",
    ] {
        if !code.contains(required) {
            found.push(format!(
                "the grammar of a name: the tool no longer holds `{}`",
                required.trim()
            ));
        }
    }

    // One road to a process, and three programs on it: git, and the two scripts.
    for (n, line) in code.lines().enumerate() {
        for forbidden in [
            "os.system",
            "os.popen",
            "os.exec",
            "os.spawn",
            "shell=True",
            "pty.",
            "shutil",
            "os.remove",
            "os.unlink",
            "os.rmdir",
            "os.rename",
            "os.replace",
            "os.chmod",
            // It ends nothing: a held command runs to its end, or is somebody's to kill.
            "os.kill",
            "import signal",
            "eval(",
            "exec(",
            "__import__",
            "importlib",
        ] {
            if line.contains(forbidden) {
                found.push(format!("line {}: `{forbidden}`: {}", n + 1, line.trim()));
            }
        }
    }
    // TWO roads to a process: `run`, whose output the tool reads, from the repository's
    // root — and `streamed`, a long command whose output is kept and not read. And ONE
    // fork: the supervisor of a held command, in a session of its own, which never returns.
    let spawning: Vec<&str> = code
        .lines()
        .filter(|line| line.contains("subprocess") && !line.starts_with("import "))
        .map(str::trim_start)
        .collect();
    if spawning.len() != 2
        || !spawning[0].starts_with("done = subprocess.run(argv, cwd=ROOT, ")
        || spawning[1]
            != "return subprocess.run(argv, cwd=cwd, stdin=subprocess.DEVNULL, stdout=out, stderr=subprocess.STDOUT if out else None).returncode"
    {
        found.push(format!(
            "a process is started somewhere else than in `run` and `streamed`: {spawning:#?}"
        ));
    }
    for (once, within) in [
        ("os.fork(", "hold_start"),
        ("os.setsid(", "supervise"),
        ("os._exit(", "supervise"),
    ] {
        if code.matches(once).count() != 1
            || !code
                .lines()
                .skip_while(|line| !line.starts_with(&format!("def {within}(")))
                .skip(1)
                .take_while(|line| !line.starts_with("def "))
                .any(|line| line.contains(once))
        {
            found.push(format!(
                "`{once}…)` stands somewhere else than once, in `{within}`: a second fork, or a session of somebody else's"
            ));
        }
    }
    // What a held command runs is the tool's own: `streamed` is called by the supervisor,
    // with the command `held_command` built, and by the build, with cargo — and
    // `held_command` returns nothing but the four programs of the table.
    for call in calls(&code, &["streamed"]) {
        let program = match call.args.first() {
            Some(Arg::Expression(text)) => text.clone(),
            other => format!("{other:?}"),
        };
        let known = (call.within == "supervise" && program == "argv" && call.args.len() == 2)
            || (call.within == "build"
                && program
                    == "[\"cargo\", \"build\", \"--release\", \"--locked\", \"--bin\", \"jigc\", \"--target-dir\", os.path.join(work, \"target\")]");
        if !known {
            found.push(format!(
                "`streamed({program})` in `{}`: a long command the tool does not hold",
                call.within
            ));
        }
    }
    let supervised: Vec<String> = calls(&code, &["supervise"])
        .iter()
        .map(|call| format!("{} in {}", shape(&call.args), call.within))
        .collect();
    let returned: Vec<&str> = code
        .lines()
        .skip_while(|line| !line.starts_with("def held_command("))
        .skip(1)
        .take_while(|line| !line.starts_with("def "))
        .map(str::trim)
        .filter(|line| line.starts_with("return "))
        .collect();
    if supervised != ["home argv in hold_start"]
        || !code
            .contains("        argv = held_command(args.kind, args.name, asked, args.scratch)\n")
        || returned.len() != HELD_PROGRAMS.len()
        || !returned
            .iter()
            .zip(HELD_PROGRAMS)
            .all(|(line, opens)| line.starts_with(opens))
    {
        found.push(format!(
            "a held command is something else than what `held_command` builds from the table's four programs: supervised {supervised:?}, returned {returned:#?}"
        ));
    }
    for call in calls(&code, &["run"]) {
        let program = match call.args.first() {
            Some(Arg::Expression(text)) => text.clone(),
            other => format!("{other:?}"),
        };
        let known = (call.within == "git" && program == "[\"git\"] + list(argv)")
            || program == "[MERGE_LOGS]"
            || program.starts_with("[RECORD, \"state\", ")
            || program.starts_with("[RECORD, \"check-reports\"] + ")
            || program.starts_with("[RECORD, \"pending\", ")
            || program.starts_with("[RECORD, \"gate-check\", ")
            || program.starts_with("[RECORD, \"settle\", ")
            // The vet of what a step would commit or push, and its two exits: a file that
            // is no report set aside, and a batch that is not committed taken back.
            || program.starts_with("[RECORD, \"vet\", ")
            || program.starts_with("[RECORD, \"set-aside\", ")
            || (call.within == "taken_back" && program.starts_with("[RECORD, \"discard\", "))
            // A write of the record script that was killed, finished before the tree is
            // read: by the one helper every act that reads the pending writes asks.
            || (call.within == "arrived" && program.starts_with("[RECORD, \"recover\", "))
            // The binary a build made, asked for its version before it is placed.
            || (call.within == "build" && program == "[made, \"--version\"]")
            // The one write the tool makes through the record script: an attempt's marker,
            // by the act that begins the attempt, under the reporter it was handed.
            || (call.within == "begin"
                && program.starts_with(
                    "[RECORD, \"report\"] + stage_flags(args) + [\"--reporter\", args.reporter, ",
                ));
        if !known {
            found.push(format!(
                "`run({program})` in `{}`: a program the tool does not run",
                call.within
            ));
        }
    }
    // The files the tool opens: its own source for `--help`; the state it keeps under the
    // scratch root; the answer of a commit step that is done, kept beside the file of
    // the gate it was held to and read by that step asked again; and, for an act asked for
    // its digest, the line the digest leaves out — written under the digest's root, and
    // the file the digest names read back for its hash.
    // — and what a held command leaves under the scratch root. Each site is spelled in
    // [`OPENS`], and stands as often as it says.
    let opened: Vec<&str> = code
        .lines()
        .filter(|line| line.contains("open("))
        .map(str::trim)
        .collect();
    let strange: Vec<&&str> = opened
        .iter()
        .filter(|line| !OPENS.iter().any(|(site, _)| line.contains(site)))
        .collect();
    let miscounted: Vec<String> = OPENS
        .iter()
        .filter(|(site, often)| opened.iter().filter(|line| line.contains(site)).count() != *often)
        .map(|(site, often)| format!("{often} × `{site}`"))
        .collect();
    if !strange.is_empty() || !miscounted.is_empty() {
        found.push(format!(
            "the tool opens a file somewhere else than its help, the kept state, a commit step's kept answer, the file a digest names and a held command's own files: {strange:#?}; not as often as listed: {miscounted:#?}"
        ));
    }
    // AND THROUGH ITS TWO HELPERS that open a path handed to them: every call of either
    // is a row of [`THROUGH`], and every row a call — held as two sorted lists, so a new
    // caller and a site that is gone are each named.
    let mut through: Vec<(String, String, String)> = calls(&code, &["whole", "kept_once"])
        .iter()
        .map(|call| {
            let path = match call.args.first() {
                Some(Arg::Literal(text) | Arg::Expression(text)) => text.clone(),
                None => String::new(),
            };
            (call.name.clone(), call.within.clone(), path)
        })
        .collect();
    through.sort();
    let mut listed: Vec<(String, String, String)> = THROUGH
        .iter()
        .map(|(helper, within, path)| {
            (
                (*helper).to_owned(),
                (*within).to_owned(),
                (*path).to_owned(),
            )
        })
        .collect();
    listed.sort();
    if through != listed {
        let unlisted: Vec<_> = through
            .iter()
            .filter(|call| !listed.contains(call))
            .collect();
        let gone: Vec<_> = listed.iter().filter(|row| !through.contains(row)).collect();
        found.push(format!(
            "the tool opens a file through `whole` or `kept_once` somewhere else than listed — {unlisted:?} — or a listed site is gone — {gone:?} — or one stands more often than it is listed"
        ));
    }
    found
}

/// A call's arguments as one line: a literal as itself, anything else as written.
fn shape(args: &[Arg]) -> String {
    args.iter()
        .map(|arg| match arg {
            Arg::Literal(text) | Arg::Expression(text) => text.as_str(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn the_tool_runs_no_git_command_an_agent_may_not() {
    let source = read(TOOL);
    assert_eq!(
        offences(&source),
        Vec::<String>::new(),
        "{TOOL} does something an agent may not — or the scan no longer reads it"
    );
    // The scan read the tool: every subcommand it may run is one it does run, and the
    // calls are found in the acts.
    let found = calls(&python_code(&source), &["git", "said"]);
    assert!(
        found.len() >= 60,
        "the scan found the tool's git calls: {}",
        found.len()
    );
    let run: BTreeSet<String> = found
        .iter()
        .filter_map(|call| match call.args.first() {
            Some(Arg::Literal(subcommand)) => Some(subcommand.clone()),
            _ => None,
        })
        .collect();
    let allowed: BTreeSet<String> = ALLOWED.iter().map(|(name, _)| (*name).to_owned()).collect();
    assert_eq!(
        run, allowed,
        "the subcommands the tool runs (left) are the ones it may (right)"
    );
    let pushes: Vec<String> = found
        .iter()
        .filter(|call| call.args.first() == Some(&Arg::Literal("push".to_owned())))
        .map(|call| format!("{} in {}", shape(&call.args), call.within))
        .collect();
    assert_eq!(
        pushes,
        ["push origin branch in push"],
        "one push, by name, in one helper"
    );
}

#[test]
fn a_planted_offender_reddens_the_scan() {
    let source = read(TOOL);
    let anchor = "def push_branch(args, facts):\n";
    assert_eq!(
        source.matches(anchor).count(),
        1,
        "the act the offenders are planted in"
    );
    // Each line is planted alone, as the first statement of an act.
    let planted: &[(&str, &str)] = &[
        (
            "git(\"push\", \"--force\", \"origin\", args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"-f\", \"origin\", args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"--force-with-lease\", \"origin\", args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"origin\", \"+\" + args.branch)",
            "forced push",
        ),
        (
            "git(\"push\", \"origin\", \"--delete\", args.branch)",
            "`--delete`",
        ),
        (
            "git(\"push\", \"origin\", \":\" + args.branch)",
            "joined to \":\"",
        ),
        (
            "git(\"push\", \"origin\", args.branch + \":main\")",
            "joined to \":main\"",
        ),
        (
            "git(\"push\", \"origin\", \"main\")",
            "`main` outside the sync act",
        ),
        (
            "git(\"push\", \"origin\", args.rounds)",
            "a shape the tool does not have",
        ),
        ("git(\"push\", \"--all\", \"origin\")", "`--all`"),
        // What the remote lacks is asked of the remote: a range read off the remote-tracking
        // refs, in either read of a push, is one the tool no longer makes.
        (
            "said(\"rev-list\", \"--reverse\", args.branch, \"--not\", \"--remotes=origin\")",
            "`--remotes=origin`",
        ),
        (
            "said(\"log\", \"--format=%H\", args.branch, \"--not\", \"--remotes=origin\")",
            "`--remotes=origin`",
        ),
        (
            "git(\"push\", \"origin\", \"HEAD\")",
            "a shape the tool does not have",
        ),
        ("git(\"push\", \"--tags\", \"origin\")", "`--tags`"),
        ("git(\"rebase\", args.branch)", "`git rebase`"),
        ("git(\"reset\", \"--hard\", \"HEAD\")", "`git reset`"),
        ("git(\"pull\", \"origin\", args.branch)", "`git pull`"),
        ("git(\"pull\")", "`git pull`"),
        ("git(\"stash\")", "`git stash`"),
        ("git(\"branch\", \"-D\", args.branch)", "`-D`"),
        ("git(\"branch\", \"-d\", args.branch)", "`-d`"),
        ("git(\"branch\", \"--delete\", args.branch)", "`--delete`"),
        ("git(\"tag\", \"-d\", \"v1\")", "`git tag`"),
        ("git(\"checkout\", \"main\")", "`git checkout`"),
        ("said(\"switch\", \"main\")", "`main` outside the sync act"),
        ("said(\"switch\", \"-C\", args.branch, args.loop)", "`-C`"),
        (
            "git(\"merge\", \"--no-ff\", \"--no-edit\", \"main\")",
            "`main` outside the sync act",
        ),
        (
            "git(\"merge\", \"--no-ff\", \"--no-edit\", args.branch)",
            "a shape the tool does not have",
        ),
        (
            "git(\"merge\", \"--ff-only\", \"origin/main\")",
            "`--ff-only`",
        ),
        (
            "said(\"fetch\", \"origin\", \"main:main\")",
            "`main` outside the sync act",
        ),
        (
            "said(\"fetch\", \"origin\", \"main\")",
            "`main` outside the sync act",
        ),
        ("git(\"commit\", \"--amend\", \"--no-edit\")", "`--amend`"),
        ("git(\"commit\", \"--no-edit\", \"-a\")", "`-a`"),
        (
            "git(\"cherry-pick\", \"-x\", args.branch)",
            "a shape the tool does not have",
        ),
        ("git(args.branch)", "subcommand is not a literal"),
        ("push(\"main\", facts)", "a name nobody checked"),
        ("push(args.rounds, facts)", "a name nobody checked"),
        ("merge(\"main\", [], facts)", "a name nobody checked"),
        ("merge(\"origin/main\", [], facts)", "a name nobody checked"),
        (
            "subprocess.run([\"git\", \"push\", \"--force\"])",
            "a process is started somewhere else",
        ),
        ("os.system(\"git reset --hard\")", "`os.system`"),
        (
            "run([\"git\", \"rebase\", \"main\"])",
            "a program the tool does not run",
        ),
        (
            "run([\"sh\", \"-c\", \"git pull\"])",
            "a program the tool does not run",
        ),
        ("os.remove(args.branch)", "`os.remove`"),
        // A held command: nothing a caller names is run, nothing is forked or detached a
        // second time, nothing is killed, and no file is opened outside the listed sites.
        (
            "streamed([\"sh\", \"-c\", args.branch], ROOT)",
            "a long command the tool does not hold",
        ),
        (
            "streamed(args.branch.split(), ROOT)",
            "a long command the tool does not hold",
        ),
        (
            "supervise(args.branch, [args.branch])",
            "a held command is something else",
        ),
        ("os.fork()", "`os.fork(…)` stands somewhere else"),
        ("os.setsid()", "`os.setsid(…)` stands somewhere else"),
        (
            "subprocess.Popen([args.branch], start_new_session=True)",
            "a process is started somewhere else",
        ),
        ("os.kill(1, 9)", "`os.kill`"),
        (
            "open(args.branch, \"w\").write(\"\")",
            "the tool opens a file somewhere else",
        ),
        (
            "git(\"archive\", \"--remote=origin\", args.branch)",
            "`--remote=origin`",
        ),
        // A FILE OPENED IN THE TOOL'S OWN WAY (the core review's `F7`): the two helpers
        // open whatever path they are handed, so a new caller of either is a file the
        // tool opens — one written inside the repository, one read from anywhere.
        (
            "kept_once(os.path.join(ROOT, \"planted.txt\"), \"x\")",
            "through `whole` or `kept_once`",
        ),
        ("whole(args.branch)", "through `whole` or `kept_once`"),
    ];
    for (line, names) in planted {
        let mutant = source.replacen(anchor, &format!("{anchor}    {line}\n"), 1);
        let found = offences(&mutant);
        assert!(
            found.iter().any(|offence| offence.contains(names)),
            "the scan must name `{line}` ({names}); it found: {found:#?}"
        );
    }
    // AND THE OTHER DIRECTION: a listed site of the two helpers that is gone — the
    // supervisor no longer writing how the command ended, a build no longer hashed.
    for (site, instead) in [
        (
            "        kept_once(os.path.join(home, \"exit.json\"), ",
            "        print(os.path.join(home, \"exit.json\"), ",
        ),
        ("    data = whole(made)\n", "    data = bytes(made)\n"),
    ] {
        assert_eq!(source.matches(site).count(), 1, "`{site}` stands once");
        let found = offences(&source.replacen(site, instead, 1));
        assert!(
            found
                .iter()
                .any(|offence| offence.contains("through `whole` or `kept_once`")),
            "the scan must miss the site `{site}`; it found: {found:#?}"
        );
    }
    // And the grammar a name is checked against: a branch without a prefix, or a name
    // that is no longer checked at all.
    for (from, to) in [
        (
            "(?:/[a-z0-9]+(?:-[a-z0-9]+)*)+\")",
            "(?:/[a-z0-9]+(?:-[a-z0-9]+)*)*\")",
        ),
        ("    \"loop\": branch_name,\n", "    \"loop\": str,\n"),
        ("required=True, type=FLAGS[flag])", "required=True)"),
        (
            "names=(\"shas\", argparse.ONE_OR_MORE, full_sha)",
            "names=(\"shas\", argparse.ONE_OR_MORE, str)",
        ),
    ] {
        assert_eq!(source.matches(from).count(), 1, "`{from}` stands once");
        let found = offences(&source.replacen(from, to, 1));
        assert!(
            found
                .iter()
                .any(|offence| offence.contains("the grammar of a name")),
            "the scan must name the grammar without `{from}`; it found: {found:#?}"
        );
    }
}

// ---------------------------------------------------------------------------
// The harness and the tool: one contract (where node is installed)
// ---------------------------------------------------------------------------

/// Calls the harness's pure functions — everything above its first statement that runs,
/// which the harness's own fence holds to launching nothing — and prints what they return.
const PURE: &str = r#"
import { readFileSync } from 'node:fs'
const [script, callsJson] = process.argv.slice(2)
const source = readFileSync(script, 'utf8').replace(/^export const meta = /m, 'const meta = ')
const pure = source.slice(0, source.indexOf('\n// ---- args: parsed, and refused before any agent ----\n'))
const calls = JSON.parse(callsJson)
const names = [...new Set(calls.map((call) => call[0]))]
const made = new Function(pure + '\nreturn { ' + names.join(', ') + ' }')()
console.log(JSON.stringify(calls.map(([name, args]) => made[name](...args))))
"#;

/// The file the gate names for the tests that pass without running what they test
/// (`dev/gate`'s header: *PASSED WITHOUT RUNNING*): one line per test, which the gate
/// counts and prints above its verdict.
const GATE_SKIPS: &str = "JIGC_GATE_SKIPS";

/// Whether `node` is there to run the harness under — and, where it is not, what a test
/// that needs it does: `unrun` says what was not run on this machine.
///
/// **Under CI it fails.** GitHub's hosted runners ship `node`, so a runner without it is a
/// job that changed, and a suite that passed there by running nothing would be the gap the
/// suite exists to close. **Anywhere else it returns false, and the caller passes** — the
/// gate gains no dependency — **but not silently**: a test runner shows a passing test's
/// stderr to nobody, so beside the line on stderr the skip is written to the file the gate
/// names, and the gate prints it in its own summary.
pub(crate) fn node_or_skip(unrun: &str) -> bool {
    let answers = Command::new("node")
        .arg("--version")
        .output()
        .is_ok_and(|out| out.status.success());
    if answers {
        return true;
    }
    assert!(
        std::env::var_os("CI").is_none_or(|ci| ci.is_empty()),
        "`node` is not on PATH, and the environment says this is CI: {unrun}. A hosted \
         runner ships `node`; a job that lacks it installs it (`actions/setup-node`) — \
         the suite is never left to pass there without running"
    );
    let skipped = format!("SKIPPED: `node` is not on PATH, so {unrun}");
    eprintln!("{skipped}");
    if let Some(skips) = std::env::var_os(GATE_SKIPS) {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&skips)
            .unwrap_or_else(|e| panic!("open the gate's skip file: {e}"));
        writeln!(file, "{skipped}").expect("write the skip to the gate's file");
    }
    false
}

/// The harness's own pure functions, called under `node`: one result per `[name, args]`.
pub(crate) fn harness_pure(calls: &Value) -> Vec<Value> {
    let scratch = ScratchDir::new("stabilize-step-pure");
    let driver = scratch.path().join("pure.mjs");
    fs::write(&driver, PURE).expect("write the driver");
    let out = Command::new("node")
        .arg(&driver)
        .arg(repo_root().join(HARNESS))
        .arg(calls.to_string())
        .output()
        .expect("run node");
    assert!(
        out.status.success(),
        "the harness's pure functions under {calls}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("the driver prints one JSON list")
}

/// The one command of a git step's prompt: the code span of its first numbered line.
pub(crate) fn command_of(prompt: &str) -> &str {
    let line = prompt
        .lines()
        .find(|line| line.starts_with("1. `"))
        .unwrap_or_else(|| panic!("a git step's prompt lists its command first: {prompt}"));
    let command = &line["1. `".len()..];
    &command[..command.rfind('`').expect("the code span closes")]
}

/// A `build-git` agent, as a function: the one command of the step's prompt, run as
/// written from the repository's root, and its one line relayed — what the agent returns.
pub(crate) fn git_agent(rig: &StepRig, prompt: &str) -> Value {
    json!({"status": "ran", "line": rig.shell(command_of(prompt)).raw})
}

#[test]
fn every_command_the_harness_composes_is_taken_by_the_tool_and_read_back() {
    if !node_or_skip(&format!(
        "the commands {HARNESS} composes were not run against {TOOL}"
    )) {
        return;
    }
    let rig = StepRig::new("harness");
    let scratch = rig.scratch.display().to_string();
    let v = |stage: &str| json!({"run": RUN, "stage": stage, "scratch": scratch});
    let branch = format!("{ROUNDS}1");
    let dropped = format!("{ROUNDS}2");
    let ctx = json!({"run": RUN, "round": 1, "stage": "test", "attempt": 1, "scratch": scratch});

    // One step: the harness composes it, the agent runs its one command, the harness
    // reads the line — each by its own code.
    let step = |act: &str, function: &str, args: Value| -> Value {
        let prompt = harness_pure(&json!([[function, args]]))
            .remove(0)
            .as_str()
            .unwrap_or_else(|| panic!("`{function}` returns a prompt"))
            .to_owned();
        assert!(
            command_of(&prompt).starts_with(&format!("{TOOL} {act} ")),
            "`{function}` composes ONE command of the tool: {prompt}"
        );
        let relayed = git_agent(&rig, &prompt);
        let read = harness_pure(&json!([["readStep", [relayed, act]]])).remove(0);
        assert!(
            read.get("relay").is_none() && read["act"] == act,
            "`readStep` reads the line `{function}`'s command printed: {read}"
        );
        read
    };

    let state = step("git-state", "gitStatePrompt", json!([v("test")]));
    assert_eq!(state["status"], "ready", "{state}");
    assert_eq!(state["branch"], LOOP);
    let read = step("state", "statePrompt", json!([v("test"), "test-1"]));
    assert_eq!(read["status"], "read", "{read}");
    assert_eq!(read["state"]["opened"], true);
    // The step that begins an attempt: the tool takes the command as composed, and its
    // answer is read back — here its refusal, because this rig's run has an opening record
    // and no fact, and no stage begins before the opening is done. (The act writes through
    // the record script, whose scanners this rig lacks: it is driven to its end in
    // [`stabilize_simulation`](super::stabilize_simulation).)
    let begun = step("begin", "beginPrompt", json!([ctx, rig.rev("HEAD")]));
    assert_eq!(
        json!([begun["status"], begun["refused"]]),
        json!(["halted", "position"]),
        "{begun}"
    );
    let check = step(
        "check-reports",
        "checkReportsPrompt",
        json!([ctx, ["scope", "preflight"]]),
    );
    assert_eq!(check["status"], "checked", "{check}");
    assert_eq!(check["check"]["missing"], json!(["scope", "preflight"]));
    let found = step("find-round", "findRoundPrompt", json!([v("fix"), 1]));
    assert_eq!(found["local"], json!([]), "{found}");
    let opened = step(
        "open-round",
        "openRoundPrompt",
        json!([v("fix"), 1, branch]),
    );
    assert_eq!(opened["created"], true, "{opened}");
    rig.change("crates/a.txt", "fixed\n", "fix: a finding");
    let pushed = step("push", "pushPrompt", json!([v("fix"), branch]));
    assert_eq!(pushed["remote_head"], pushed["head"], "{pushed}");
    let listed = step(
        "round-commits",
        "roundCommitsPrompt",
        json!([v("fix"), 1, branch]),
    );
    assert_eq!(listed["all"], json!([rig.rev(&branch)]), "{listed}");
    let landed = step("land", "landPrompt", json!([v("fix"), 1, branch]));
    assert_eq!(landed["status"], "merged", "{landed}");
    assert_eq!(landed["remote_head"], landed["merge_commit"]);
    let again = step("land", "landPrompt", json!([v("fix"), 1, branch]));
    assert_eq!(again["status"], "landed-before", "{again}");
    let after = step("git-state", "gitStatePrompt", json!([v("fix")]));
    assert_eq!(
        after["status"], "ready",
        "a landed round passes the path-class assert: {after}"
    );

    // A second round, dropped: its record commit is carried over.
    step(
        "open-round",
        "openRoundPrompt",
        json!([v("fix"), 2, dropped]),
    );
    let record = rig.change(
        &format!("{RUN_DIR}/r2/round.md"),
        "r\n",
        "docs(record): dropped",
    );
    let carried = step("carry", "carryPrompt", json!([v("fix"), 2, [record], null]));
    assert_eq!(carried["status"], "carried", "{carried}");
    assert_eq!(carried["picked"][0]["from"], record.as_str());
    let synced = step("sync-main", "syncMainPrompt", json!([RUN]));
    assert_eq!(synced["status"], "up-to-date", "{synced}");

    // A refusal is read back as a halted step that says why — and a line that was
    // retyped on its way is not the tool's.
    rig.write("notes.txt", "x\n");
    let refused = step("git-state", "gitStatePrompt", json!([v("test")]));
    assert_eq!(refused["status"], "halted", "{refused}");
    assert_eq!(refused["refused"], "dirty");
    assert!(
        refused["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("dirty: "))
    );
    let line = rig.shell(&format!("{TOOL} push --branch {LOOP}")).raw;
    let head = rig.rev("HEAD");
    let retyped = line.replacen(&head, &head.chars().rev().collect::<String>(), 1);
    assert_ne!(retyped, line);
    for (what, relayed, act) in [
        (
            "a sha retyped",
            json!({"status": "ran", "line": retyped}),
            "push",
        ),
        (
            "another act's line",
            json!({"status": "ran", "line": line}),
            "land",
        ),
        ("no line", json!({"status": "ran"}), "push"),
        (
            "no JSON",
            json!({"status": "ran", "line": "pushed"}),
            "push",
        ),
    ] {
        let read = harness_pure(&json!([["readStep", [relayed, act]]])).remove(0);
        assert!(
            read["status"] == "halted" && read["relay"].is_string(),
            "{what}: `readStep` says the line is not the command's: {read}"
        );
    }

    // UNCONVERTED ON PURPOSE: the re-cut of a part is still a list of commands the agent
    // follows — the exit is ruled to be rebuilt as a revert (DECISIONS.md → 2026-10-06,
    // *a part lands by revert*), and a mechanism about to be deleted is not ported.
    let part = harness_pure(&json!([[
        "carryPrompt",
        [v("fix"), 2, [record], format!("{dropped}-part1")]
    ]]))
    .remove(0);
    let part = part.as_str().expect("a prompt");
    assert!(
        part.starts_with("GIT STEP — re-cut a part of round 2")
            && part.contains("`git cherry-pick -x <sha>`")
            && !part.contains(TOOL),
        "the part's re-cut is the list it was: {part}"
    );
}

// ---------------------------------------------------------------------------
// The digest: a line holds what the harness acts on, and nothing else
// ---------------------------------------------------------------------------

/// The most a digest measures, in bytes: the bound every digest this suite sees is held
/// under, the state's at the first real run's shape among them
/// ([`the_state_digest_at_the_real_runs_shape_is_a_few_kilobytes`] measures that one).
const DIGEST_BOUND: usize = 8 * 1024;

/// The tool's own projection of a line it printed onto that line's digest, with no file
/// written: the tool is loaded as a module, and its `digest_line` is called.
const PROJECT: &str = r#"import importlib.machinery, importlib.util, json, sys
loader = importlib.machinery.SourceFileLoader("stabilize_step", sys.argv[1])
tool = importlib.util.module_from_spec(importlib.util.spec_from_loader("stabilize_step", loader))
loader.exec_module(tool)
sys.stdout.write(tool.digest_line(json.loads(sys.stdin.read()), None) + "\n")
"#;

/// What no declared shape of a digest lets through, checked without the tool's own table:
/// a string is of letters, digits and `. _ / + -` — so no space, no quote and no prose — a
/// number is a whole one, and a key is a plain name.
fn plain(value: &Value, at: &str, raw: &str) {
    let of = |text: &str, more: &str| {
        text.len() <= 300
            && text
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || more.contains(c))
    };
    match value {
        Value::String(text) => assert!(
            of(text, "._/+-"),
            "`{at}` holds what is no id, word, count, hash or path — {text:?}: {raw}"
        ),
        Value::Number(number) => assert!(
            number.is_u64(),
            "`{at}` holds a number that is no count — {number}: {raw}"
        ),
        Value::Array(list) => list.iter().for_each(|entry| plain(entry, at, raw)),
        Value::Object(fields) => {
            for (name, inner) in fields {
                assert!(
                    of(name, "_-"),
                    "`{at}` has a field that is no plain name — {name:?}: {raw}"
                );
                plain(inner, &format!("{at}.{name}"), raw);
            }
        }
        Value::Null | Value::Bool(_) => {}
    }
}

/// **A digest, held to what makes it one**: ONE line that ends with the sha256 of itself,
/// as every line of the tool does; printable ASCII, and no backslash — so there is no
/// escape in it for a relay to decode; under [`DIGEST_BOUND`]; and every value [`plain`].
/// It says which of its fields did not fit their shape (`unfit`), and names the file that
/// holds what it leaves out.
fn held_as_a_digest(raw: &str) -> Value {
    let digest = line_of(raw);
    let line = raw.trim_end_matches('\n');
    assert!(
        line.bytes().all(|byte| (0x20..0x7f).contains(&byte)),
        "a digest is printable ASCII: {line:?}"
    );
    assert!(!line.contains('\\'), "a digest holds no backslash: {line}");
    assert!(
        line.len() <= DIGEST_BOUND,
        "a digest is at most {DIGEST_BOUND} bytes, and this one is {}: {line}",
        line.len()
    );
    plain(&digest, "the digest", raw);
    assert!(digest["unfit"].is_array(), "a digest says what did not fit");
    for field in ["file", "file_sha256"] {
        assert!(
            digest.get(field).is_some(),
            "a digest has `{field}`: {line}"
        );
    }
    assert_eq!(
        digest["file"].is_null(),
        digest["file_sha256"].is_null(),
        "a file is named with its hash, or neither is: {line}"
    );
    digest
}

/// What an act that was asked for its digest left: its exit status, the digest — held as
/// one — its stderr, and the text of the file the digest names, held to the hash it names
/// it with.
struct Digested {
    code: i32,
    raw: String,
    line: Value,
    stderr: String,
    kept: Option<String>,
}

impl StepRig {
    /// The digest the tool makes of a line it printed, by its own function.
    fn projected(&self, raw: &str) -> Value {
        let mut child = self
            .hermetic(Command::new("python3"))
            .args(["-c", PROJECT])
            .arg(self.root.join(TOOL))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn python3");
        child_stdin::feed(&mut child, raw);
        let out = child.wait_with_output().expect("python3 exits");
        assert!(
            out.status.success(),
            "the tool makes a digest of a line it printed: {}\nthe line: {raw}",
            String::from_utf8_lossy(&out.stderr)
        );
        held_as_a_digest(&String::from_utf8_lossy(&out.stdout))
    }

    /// An act asked for its digest (`--digest`), the file it names lying under the rig's
    /// scratch root.
    fn digest(&self, args: &[&str]) -> Digested {
        self.digest_in(&[], args)
    }

    fn digest_in(&self, env: &[(&str, &str)], args: &[&str]) -> Digested {
        let scratch = self.scratch.display().to_string();
        let mut asked = vec![args[0], "--digest", scratch.as_str()];
        asked.extend(&args[1..]);
        let seen = self.step_in(env, &asked);
        let line = held_as_a_digest(&seen.raw);
        let kept = line["file"].as_str().map(|rel| {
            let text = fs::read_to_string(self.scratch.join(rel))
                .unwrap_or_else(|e| panic!("the file a digest names is there — `{rel}`: {e}"));
            assert_eq!(
                line["file_sha256"],
                sha256(&text).as_str(),
                "the file a digest names is held to the hash it names it with: {}",
                seen.raw
            );
            text
        });
        Digested {
            code: seen.code,
            raw: seen.raw,
            line,
            stderr: seen.stderr,
            kept,
        }
    }
}

impl Digested {
    /// The file the digest names holds the line the act prints unasked — whole, ending in
    /// its own hash — and the digest is the tool's projection of exactly that line.
    fn of_its_line(&self, rig: &StepRig) -> Value {
        let kept = self
            .kept
            .as_ref()
            .unwrap_or_else(|| panic!("the digest names a file: {}", self.raw));
        let whole = line_of(kept);
        let mut projected = rig.projected(kept);
        let mut said = self.line.clone();
        for digest in [&mut projected, &mut said] {
            for field in ["file", "file_sha256", "sha256"] {
                digest
                    .as_object_mut()
                    .expect("a digest is an object")
                    .remove(field);
            }
        }
        assert_eq!(
            said, projected,
            "the digest an act prints (left) is the digest of the line its file holds (right)"
        );
        whole
    }

    /// The act is done and says `status`; nothing of it did not fit.
    fn done(&self, rig: &StepRig, act: &str, status: &str) -> &Value {
        assert_eq!(
            self.code, 0,
            "`{act}` exits 0: {} {}",
            self.raw, self.stderr
        );
        assert_eq!(
            json!([self.line["act"], self.line["status"], self.line["unfit"]]),
            json!([act, status, []]),
            "{}",
            self.raw
        );
        assert!(self.stderr.is_empty(), "{}", self.stderr);
        if act != "state" {
            let whole = self.of_its_line(rig);
            assert_eq!(json!([whole["act"], whole["status"]]), json!([act, status]));
        }
        &self.line
    }

    /// The act refused with `word`: the word is in the digest, and the prose — the halt
    /// report — only in the file it names.
    fn refused(&self, rig: &StepRig, word: &str) -> &Value {
        let status = REFUSALS
            .iter()
            .find(|(known, _)| *known == word)
            .map(|(_, status)| *status)
            .unwrap_or_else(|| panic!("`{word}` is a refusal of the tool"));
        assert_eq!(self.code, status, "{} {}", self.raw, self.stderr);
        assert_eq!(
            json!([
                self.line["status"],
                self.line["refused"],
                self.line["unfit"]
            ]),
            json!(["halted", word, []]),
            "{}",
            self.raw
        );
        assert!(
            self.stderr
                .starts_with(&format!("stabilize-step: refused {word}: ")),
            "{}",
            self.stderr
        );
        assert!(
            self.line.get("halt").is_none(),
            "a refusal's prose is in no digest: {}",
            self.raw
        );
        let whole = self.of_its_line(rig);
        assert!(
            whole["halt"]["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.starts_with(&format!("{word}: ")))
                && whole["halt"]["recommendation"].is_string(),
            "the file holds the halt report: {whole}"
        );
        &self.line
    }
}

/// A gate that is red on one test, whose name is what a real one can be: prose.
const RED_GATE: &str = "gate: mode   full, keep-going\ntests   passed=9 failed=1  (over 3 test binaries)\n\nGATE: FAIL (step: test)\n\nfailing tests:\n  jigc::g_flows flow01::a “name” \u{2014} with a dash \u{2192} and an arrow\n\nfull output: x\n";

/// **Every line of the tool is a digest when it is asked for one** (the second repair
/// plan's `K9`; [DECISIONS.md](../DECISIONS.md) → *2026-10-07 — A line holds what the
/// harness acts on*). Observed on 2026-10-07: a line relayed through an agent never
/// arrives byte for byte once it holds one character outside ASCII — the tool prints it as
/// an escape, the agent's return decodes it, the hash fails — and a commit's subject, a
/// refusal's prose and a ledger row are full of them. So with `--digest` an act prints
/// what the harness ACTS ON and nothing else — ids, words of a fixed list, counts, hashes,
/// each of a shape its field declares — and names the file that holds the line it would
/// have printed.
///
/// Driven here FOR REAL, through the flag: every act the parser takes, to its end; and
/// sixteen refusals. Each digest is [`held_as_a_digest`]; the file it names is held to the
/// hash it names it with and holds the act's whole line; and the digest is the tool's own
/// projection of that line ([`Digested::of_its_line`]). **Every other line this suite and
/// the simulation see** — each refusal [`REFUSALS`] names, in the test that drives it — is
/// held to the same by the rig itself, which projects every line it reads
/// ([`StepRig::projected`], in `ran_in`): the same function, without the file.
#[test]
fn every_line_is_a_digest() {
    let stage = Stage::new("digest");
    let rig = &stage.rig;
    let scratch = rig.scratch.display().to_string();
    let (product, logs) = (product(), logs());
    let candidate = rig.rev("HEAD");
    let mut done = BTreeSet::new();
    let mut refused = BTreeSet::new();
    let mut ended = |seen: &Digested, rig: &StepRig, act: &str, status: &str| -> Value {
        done.insert(act.to_owned());
        seen.done(rig, act, status).clone()
    };
    let mut refuse = |seen: &Digested, rig: &StepRig, word: &str| -> Value {
        refused.insert(word.to_owned());
        seen.refused(rig, word).clone()
    };
    let record = [
        "record",
        "--branch",
        LOOP,
        "--run-dir",
        RUN_DIR,
        "--gate",
        stage.gate.as_str(),
        "--calls",
        "7",
        "--checks",
        "2",
        "--scratch",
        scratch.as_str(),
    ];
    let state = git_state("test", &product);

    // A stage at its record step: what it is refused, and what it does.
    refuse(&rig.digest(&["push"]), rig, "usage");
    refuse(
        &rig.digest(&["push", "--branch", "fix/another"]),
        rig,
        "wrong-branch",
    );
    refuse(
        &rig.digest(&[
            "begin",
            "--run",
            RUN,
            "--round",
            "1",
            "--stage",
            "test",
            "--attempt",
            "5",
            "--reporter",
            "attempt",
            "--commit",
            &candidate,
            "--scratch",
            &scratch,
        ]),
        rig,
        "position",
    );
    refuse(&rig.digest(&record), rig, "no-batch");
    let mut check = vec![
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "1",
        "--scratch",
        scratch.as_str(),
        "--",
    ];
    check.extend(LAUNCHED);
    let nowhere = rig.dir().join("no-denylist").display().to_string();
    refuse(
        &rig.digest_in(&[("JIGC_DENYLIST_FILE", nowhere.as_str())], &check),
        rig,
        "did-not-run",
    );
    let checked = ended(&rig.digest(&check), rig, "check-reports", "checked");
    assert_eq!(
        checked["check"],
        json!({"ok": true, "attempt": 1, "missing": [], "extra": 0, "other_attempts": 0, "reports": 3}),
        "the check, as counts and the reporters that left none: {checked}"
    );
    assert_eq!(checked["aside"], json!([]));

    stage.apply(SUBJECT);
    let owed = ended(&rig.digest(&state), rig, "git-state", "ready");
    assert_eq!(
        json!([
            owed["branch"],
            owed["head"],
            owed["loop_head"],
            owed["remote_head"]
        ]),
        json!([LOOP, candidate, candidate, candidate])
    );
    assert_eq!(
        json!([
            owed["pending"]["round"],
            owed["pending"]["calls"],
            owed["pending"]["checks"]
        ]),
        json!([1, 7, 2]),
        "the pending batch, without its subject and its paths: {owed}"
    );
    assert!(
        owed["pending"]["files"].as_u64().is_some_and(|n| n >= 4)
            && owed["untracked"].as_u64().is_some_and(|n| n >= 3)
            && owed["opening"].as_str().is_some_and(|sha| sha.len() == 40),
        "{owed}"
    );
    assert_eq!(
        json!([owed["found"], owed["finished"], owed["owed"]]),
        json!([["applied"], [], ["applied"]]),
        "the arrival states are words: {owed}"
    );
    let mut moved = record.to_vec();
    let elsewhere = "0".repeat(40);
    moved.extend(["--head", elsewhere.as_str()]);
    refuse(&rig.digest(&moved), rig, "head-moved");
    let green = fs::read_to_string(&stage.gate).expect("the gate's output");
    fs::write(&stage.gate, RED_GATE).expect("a red gate");
    let red = refuse(&rig.digest(&record), rig, "gate-red");
    assert_eq!(
        red["gate"],
        json!({"ok": false, "verdict": "fail", "red": 2, "known": 0, "new": 2, "candidate": candidate}),
        "what is red is counted, and named in the file alone: {red}"
    );
    fs::write(&stage.gate, green).expect("the green gate again");
    let lock = rig.root.join(".git/index.lock");
    fs::write(&lock, "").expect("a lock git left");
    refuse(&rig.digest(&state), rig, "locked");
    fs::remove_file(&lock).expect("remove the lock");
    rig.write("notes.txt", "a note \u{2014} by hand\n");
    refuse(&rig.digest(&state), rig, "dirty");
    fs::remove_file(rig.root.join("notes.txt")).expect("remove the note");

    let read = ended(
        &rig.digest(&[
            "state",
            "--run",
            RUN,
            "--scratch",
            &scratch,
            "--tag",
            "test-1",
        ]),
        rig,
        "state",
        "read",
    );
    assert_eq!(read["file"], "state/test-1.json");
    assert_eq!(
        read["state"]["position"]["test"],
        json!({"round": 2, "attempt": 1}),
        "the state as the applied batch leaves it: {read}"
    );

    let recorded = ended(&rig.digest(&record), rig, "record", "recorded");
    let commit = rig.rev("HEAD");
    assert_eq!(
        json!([
            recorded["commit"],
            recorded["head"],
            recorded["round"],
            recorded["applied"]
        ]),
        json!([commit, commit, 1, {"calls": 7, "checks": 2, "failed": 0}]),
        "{recorded}"
    );
    assert_eq!(
        json!([
            recorded["gate"]["ok"],
            recorded["found"],
            recorded["paths"].as_u64().map(|n| n >= 8)
        ]),
        json!([true, ["applied"], true]),
        "{recorded}"
    );
    let pushed = ended(&rig.digest(&state), rig, "git-state", "ready");
    assert_eq!(
        json!([
            pushed["finished"],
            pushed["remote_head"],
            pushed["vetted"],
            pushed["pending"]
        ]),
        json!([["unpushed"], commit, 1, null]),
        "the push that was owed is made, and said in words and a count: {pushed}"
    );
    let again = ended(
        &rig.digest(&["push", "--branch", LOOP, "--run-dir", RUN_DIR]),
        rig,
        "push",
        "ready",
    );
    assert_eq!(
        json!([again["vetted"], again["remote_head"]]),
        json!([0, commit])
    );
    let nothing = ended(
        &rig.digest(&["discard", "--branch", LOOP, "--run-dir", RUN_DIR]),
        rig,
        "discard",
        "discarded",
    );
    assert_eq!(
        json!([nothing["unstaged"], nothing["discarded"]]),
        json!([0, 0])
    );
    let begun = ended(
        &rig.digest(&[
            "begin",
            "--run",
            RUN,
            "--round",
            "2",
            "--stage",
            "test",
            "--attempt",
            "1",
            "--reporter",
            "attempt",
            "--commit",
            &commit,
            "--scratch",
            &scratch,
        ]),
        rig,
        "begin",
        "begun",
    );
    assert_eq!(
        json!([begun["round"], begun["attempt"], begun["report"]]),
        json!([2, 1, format!("{RUN_DIR}/r2/reports/test/attempt.a1.md")]),
        "a number is a number, and the marker a path: {begun}"
    );
    rig.change("README.md", "tuned\n", "docs: a tuning commit");
    refuse(&rig.digest(&state), rig, "foreign-commit");

    // The acts of the `fix` half and of the close, on a rig of their own.
    let rig = StepRig::new("digest-rounds");
    let branch = format!("{ROUNDS}1");
    let listed = ended(&rig.digest(&["table"]), &rig, "table", "listed");
    assert!(
        listed["acts"]
            .as_array()
            .is_some_and(|acts| acts.len() >= 14)
            && listed["arrivals"]
                .as_array()
                .is_some_and(|rows| rows.contains(&json!("unpushed"))),
        "the table's digest is its names; its sentences are in the file: {listed}"
    );
    let found = ended(
        &rig.digest(&["find-round", "--prefix", &branch]),
        &rig,
        "find-round",
        "ready",
    );
    assert_eq!(json!([found["local"], found["remote"]]), json!([[], []]));
    let open = [
        "open-round",
        "--loop",
        LOOP,
        "--branch",
        branch.as_str(),
        "--run-dir",
        RUN_DIR,
    ];
    let opened = ended(&rig.digest(&open), &rig, "open-round", "ready");
    assert_eq!(
        json!([opened["branch"], opened["created"]]),
        json!([branch, true])
    );
    let fix = rig.change(
        "crates/a.txt",
        "fixed\n",
        "fix: a finding \u{2014} “quoted”",
    );
    let pushed = ended(
        &rig.digest(&["push", "--branch", &branch]),
        &rig,
        "push",
        "ready",
    );
    assert_eq!(json!([pushed["head"], pushed["vetted"]]), json!([fix, 1]));
    let commits = [
        "round-commits",
        "--loop",
        LOOP,
        "--branch",
        branch.as_str(),
        "--run-dir",
        RUN_DIR,
    ];
    let between = ended(&rig.digest(&commits), &rig, "round-commits", "listed");
    assert_eq!(
        json!([between["all"], between["inside"], between["outside"]]),
        json!([[fix], [], [fix]])
    );
    let land = land(&branch, &product, &logs);
    let landed = ended(&rig.digest(&land), &rig, "land", "merged");
    assert_eq!(
        json!([
            landed["merge_commit"],
            landed["tip_moved"],
            landed["moved_outside"],
            landed["resolved_logs"]
        ]),
        json!([rig.rev(LOOP), false, 0, []]),
        "{landed}"
    );
    assert_eq!(landed["remote_head"], landed["head"]);
    ended(&rig.digest(&land), &rig, "land", "landed-before");
    let dropped = format!("{ROUNDS}2");
    rig.round(2);
    let kept = rig.change(
        &format!("{RUN_DIR}/r2/round.md"),
        "r\n",
        "docs(record): dropped",
    );
    let carried = ended(
        &rig.digest(&carry(&product, &[&kept])),
        &rig,
        "carry",
        "carried",
    );
    assert_eq!(
        carried["picked"],
        json!([{"from": kept, "to": rig.rev(LOOP)}])
    );
    assert_eq!(
        rig.branch(),
        LOOP,
        "the carry stands on the loop branch; {dropped} is left"
    );
    let synced = ended(&rig.digest(&sync(&logs)), &rig, "sync-main", "up-to-date");
    assert_eq!(
        synced["origin_main"],
        rig.remote("main").expect("main").as_str()
    );

    // Three more refusals: a merge among a round's commits is told in a word; so is a
    // remote that takes no push, and one that is ahead; and a record the script cannot
    // read.
    rig.remote_hook("pre-receive", "exit 1");
    let third = format!("{ROUNDS}3");
    rig.round(3);
    rig.change("crates/a.txt", "again\n", "fix: another");
    refuse(
        &rig.digest(&["push", "--branch", &third]),
        &rig,
        "push-rejected",
    );
    rig.remote_hook("pre-receive", "exit 0");
    rig.git(&["switch", "-q", LOOP]);
    rig.pushed_by_another(LOOP, "notes.md", "theirs\n");
    refuse(
        &rig.digest(&git_state("fix", &product)),
        &rig,
        "remote-ahead",
    );
    rig.write(&format!("{RUN_DIR}/ledger.md"), "not a table\n");
    let unread = refuse(
        &rig.digest(&[
            "state",
            "--run",
            RUN,
            "--scratch",
            &scratch_of(&rig),
            "--tag",
            "test-9",
        ]),
        &rig,
        "record",
    );
    assert!(unread.get("state").is_none(), "{unread}");

    // The acts that hold a long command, on a rig whose commands are stand-ins: a gate
    // that is red, started, waited for and judged; a file hashed; a binary built.
    let held = Held::new("digest-held");
    let rig = &held.rig;
    rig.on_path("cargo", CARGO);
    rig.write("version.txt", "1.0.0-rc.24\n");
    let built_from = rig.commit("chore: a version");
    held.released(true);
    let prints = rig.dir().join("prints");
    fs::write(&prints, as_recorded("gate-red.txt")).expect("what the gate prints");
    let told = [
        prints.display().to_string(),
        held.ran.display().to_string(),
        held.release.display().to_string(),
    ];
    let env = [
        ("HOLD_PRINTS", told[0].as_str()),
        ("HOLD_RAN", told[1].as_str()),
        ("HOLD_RELEASE", told[2].as_str()),
        ("HOLD_EXIT", "1"),
    ];
    let start = |run: &'static str| {
        rig.digest_in(
            &env,
            &[
                "hold-start",
                "--scratch",
                &held.scratch,
                "--name",
                "gate-c1-a1",
                "--kind",
                "gate",
                "--run",
                run,
            ],
        )
    };
    let started = ended(&start(RUN), rig, "hold-start", "started");
    assert_eq!(
        json!([started["name"], started["kind"], started["output"]]),
        json!(["gate-c1-a1", "gate", "hold/gate-c1-a1/output"]),
        "{started}"
    );
    let wait = |name: &str| {
        rig.digest(&[
            "hold-wait",
            "--scratch",
            &held.scratch,
            "--name",
            name,
            "--slice",
            "60",
        ])
    };
    let waited = ended(&wait("gate-c1-a1"), rig, "hold-wait", "done");
    assert_eq!(
        json!([
            waited["verdict"],
            waited["why"],
            waited["exit"],
            waited["facts"],
            waited["verdict_file"]
        ]),
        json!(["red", null, 1,
               {"passed": 1, "failed": 1, "binaries": 3, "steps": ["test"], "tests": 1},
               "hold/gate-c1-a1/verdict.json"]),
        "what is red is counted, and named in the verdict's file alone: {waited}"
    );
    refuse(&start("another"), rig, "taken");
    refuse(&wait("nobody"), rig, "missing");
    let hashed = ended(
        &rig.digest(&[
            "hash",
            "--scratch",
            &held.scratch,
            "--file",
            "hold/gate-c1-a1/output",
        ]),
        rig,
        "hash",
        "hashed",
    );
    assert_eq!(
        json!([hashed["path"], hashed["content_sha256"]]),
        json!([
            "hold/gate-c1-a1/output",
            sha256(&as_recorded("gate-red.txt"))
        ]),
        "{hashed}"
    );
    let build = |commit: &str, work: &str, to: &str| {
        rig.digest(&[
            "build",
            "--scratch",
            &held.scratch,
            "--commit",
            commit,
            "--work",
            work,
            "--to",
            to,
        ])
    };
    let built = ended(
        &build(&built_from, "work-1", "bin/c1/jigc"),
        rig,
        "build",
        "built",
    );
    assert_eq!(
        json!([
            built["commit"],
            built["binary"],
            built["version"],
            built["resolves"]
        ]),
        json!([built_from, "bin/c1/jigc", "1.0.0-rc.24", true]),
        "{built}"
    );
    rig.write("fails-to-build", "\n");
    let broken = rig.commit("chore: a commit that does not build");
    refuse(&build(&broken, "work-2", "bin/c2/jigc"), rig, "build");

    assert_eq!(
        done,
        acts().into_keys().collect::<BTreeSet<_>>(),
        "every act the parser takes was driven to its end as a digest"
    );
    assert_eq!(
        refused.into_iter().collect::<Vec<_>>(),
        [
            "build",
            "did-not-run",
            "dirty",
            "foreign-commit",
            "gate-red",
            "head-moved",
            "locked",
            "missing",
            "no-batch",
            "position",
            "push-rejected",
            "record",
            "remote-ahead",
            "taken",
            "usage",
            "wrong-branch",
        ],
        "the refusals driven through the flag"
    );
}

fn scratch_of(rig: &StepRig) -> String {
    rig.scratch.display().to_string()
}

/// What a real record is written in, and a line of JSON carries as escapes: a dash, an
/// arrow, typographic quotes, a backtick, a straight quote, a backslash and a pipe.
const HOSTILE: &str =
    "a \u{201c}quoted\u{201d} cell \u{2014} `jigc doc set` \u{2192} it\u{2019}s \"ok\" \\ a | b";

/// A run of the rig at a shape — `rows` ledger rows, `doors` doors inside round 1's scope
/// and as many left out, `items` items of the test set over four clauses, and the first
/// `human` rows on the human's list — written through the record script, with `said` in
/// every cell that takes a sentence: a brief, a row's source and repro, a door's
/// derivation.
fn shaped(
    label: &str,
    rows: usize,
    doors: usize,
    items: usize,
    human: usize,
    said: &str,
) -> StepRig {
    let rig = StepRig::new(label);
    let clauses = ["audit-clean", "no-lost-files", "ports-clean", "trial-clean"];
    let door = |n: usize| format!("jigc door {n:03}");
    rig.wrote(
        &format!("run-set --run {RUN}"),
        &json!({"stop": "every-round", "previous": "1.0.0-rc.24", "previous-commit": "9".repeat(40),
                "scope": "delta", "clauses": clauses})
        .to_string(),
    );
    let set: Vec<Value> = (0..items)
        .map(|n| {
            if n == 0 {
                json!({"item": "gate", "kind": "check", "clause": clauses[0],
                       "runs": "every-candidate", "brief": said})
            } else {
                json!({"item": format!("review-{n:03}"), "kind": "review-row",
                       "clause": clauses[n % clauses.len()], "runs": "in-scope",
                       "doors": [door(n)], "brief": said})
            }
        })
        .collect();
    rig.wrote(&format!("item-set --run {RUN}"), &json!(set).to_string());
    let ledger: Vec<Value> = (1..=rows)
        .map(|n| {
            json!({"key": format!("row-{n:03}"), "doctype": "jigc-feedback", "round": 0,
                   "source": said, "door": door(n), "clause": clauses[n % clauses.len()],
                   "repro": said})
        })
        .collect();
    rig.wrote(
        &format!("ledger-add --run {RUN}"),
        &json!(ledger).to_string(),
    );
    if human > 0 {
        let graded: Vec<Value> = (1..=human)
            .map(|n| json!({"key": format!("row-{n:03}"), "grade": "needs-bound", "graded_by": "triage"}))
            .collect();
        rig.wrote(
            &format!("ledger-set --run {RUN}"),
            &json!(graded).to_string(),
        );
    }
    rig.commit("docs(record): the opening's facts");
    rig.git(&["push", "-q", "origin", LOOP]);
    let listed = |from: usize| -> Vec<Value> {
        (from + 1..=from + doors)
            .map(|n| json!({"door": door(n), "registry": "verbs", "derivation": said}))
            .collect()
    };
    rig.wrote(
        &format!("scope-set --run {RUN} --round 1"),
        &json!({"included": listed(0), "excluded": listed(doors)}).to_string(),
    );
    rig
}

impl StepRig {
    /// The state's digest, and the document the file it names holds.
    fn state_digest(&self, tag: &str) -> (Digested, Value) {
        let scratch = scratch_of(self);
        let seen = self.digest(&["state", "--run", RUN, "--scratch", &scratch, "--tag", tag]);
        seen.done(self, "state", "read");
        let document = serde_json::from_str(seen.kept.as_ref().expect("the state's file"))
            .expect("the state document");
        (seen, document)
    }
}

/// A digest up to the hash of the file it names: everything but its two hashes.
fn up_to_its_file(raw: &str) -> &str {
    &raw[..raw
        .find(",\"file_sha256\"")
        .expect("a digest names its file's hash")]
}

/// A digest without its two hashes — its own, and the one of the file it names — and with
/// every number struck out: what is left is its shape and its words.
fn struck(raw: &str) -> String {
    let mut out = String::new();
    let mut chars = up_to_its_file(raw).chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            while chars.peek().is_some_and(char::is_ascii_digit) {
                chars.next();
            }
            out.push('#');
        } else {
            out.push(c);
        }
    }
    out
}

/// **The state's digest does not grow with the ledger or the doors** — which is what the
/// document did: 3, 25, 75 and 143 KB at these four sizes (the re-review's `R-H1`), relayed
/// whole three times a stage. At 1 · 20 · 60 · 120 ledger rows over 2 · 40 · 150 · 300
/// doors on each side of the round's scope the four digests differ in digits and in
/// nothing else; and a run whose every sentence holds a dash, an arrow and typographic
/// quotes has the digest, byte for byte up to the hash of the file it names, of the same
/// run written in plain words.
#[test]
fn the_state_digest_does_not_grow_with_the_ledger_or_the_doors() {
    let sizes = [(1, 2), (20, 40), (60, 150), (120, 300)];
    let digests: Vec<(usize, String, Value)> = std::thread::scope(|threads| {
        let built: Vec<_> = sizes
            .iter()
            .map(|(rows, doors)| {
                threads.spawn(move || {
                    let rig = shaped(&format!("digest-size-{rows}"), *rows, *doors, 2, 0, HOSTILE);
                    let (seen, document) = rig.state_digest("test-1");
                    (*rows, seen.raw, document)
                })
            })
            .collect();
        built
            .into_iter()
            .map(|thread| thread.join().expect("a rig is built"))
            .collect()
    });
    for ((rows, doors), (_, raw, document)) in sizes.iter().zip(&digests) {
        let digest = line_of(raw);
        // The control: the run really has that many rows and doors, and the document says so.
        assert_eq!(document["ledger"].as_array().map(Vec::len), Some(*rows));
        assert_eq!(
            document["doors"]["included"].as_array().map(Vec::len),
            Some(*doors)
        );
        assert_eq!(
            json!([
                digest["state"]["ledger"],
                digest["state"]["doors"],
                digest["state"]["untriaged"]
            ]),
            json!([rows, {"included": doors, "excluded": doors},
                   {"count": rows, "why": [{"why": "ungraded", "count": rows}]}]),
            "the digest counts what the document lists: {raw}"
        );
    }
    let (_, smallest, _) = &digests[0];
    for (rows, raw, document) in &digests {
        assert_eq!(
            struck(raw),
            struck(smallest),
            "at {rows} rows the digest differs from the one at 1 row in more than digits"
        );
        assert!(
            raw.len() < smallest.len() + 40,
            "at {rows} rows the digest is {} bytes, and at 1 row {}",
            raw.len(),
            smallest.len()
        );
        assert!(
            document.to_string().len() > 2_000 * rows.min(&60) / 20,
            "the document grows, as measured: {} bytes at {rows} rows",
            document.to_string().len()
        );
    }

    // The same run in plain words: the digest is the same line, up to the hash of the
    // file — which holds other bytes, because the document holds the sentences.
    let spoken = shaped("digest-plain", 20, 40, 2, 0, "a plain cell")
        .state_digest("test-1")
        .0;
    let (_, hostile, document) = &digests[1];
    assert_eq!(
        up_to_its_file(hostile),
        up_to_its_file(&spoken.raw),
        "a dash, an arrow and a typographic quote in every sentence of the record change no byte of the digest"
    );
    assert_ne!(line_of(hostile)["file_sha256"], spoken.line["file_sha256"]);
    assert!(
        [
            &document["ledger"][0]["repro"],
            &document["items"][0]["brief"]
        ]
        .iter()
        .all(
            |cell| cell.as_str().is_some_and(|text| text.contains('\u{2014}')
                && text.contains('\u{2192}')
                && text.contains('\u{201c}'))
        ),
        "the control: the sentences are in the document, as written: {}",
        document["ledger"][0]
    );
}

/// **The digest at the first real run's shape is a few kilobytes** — 60 ledger rows, 150
/// doors on each side of the scope, 30 items of the test set over four clauses, and a
/// human's list of 20: the shape the re-review measured the document at 75 KB for. What it
/// grows with is the test set — an item is one entry of it, with where its own run stands
/// — and with nothing else.
#[test]
fn the_state_digest_at_the_real_runs_shape_is_a_few_kilobytes() {
    let rig = shaped("digest-real", 60, 150, 30, 20, HOSTILE);
    let (seen, document) = rig.state_digest("test-1");
    let state = &seen.line["state"];
    assert_eq!(
        json!([
            state["ledger"],
            state["human_list"],
            state["untriaged"]["count"],
            state["doors"]
        ]),
        json!([60, 20, 40, {"included": 150, "excluded": 150}]),
        "{}",
        seen.raw
    );
    assert_eq!(state["items"].as_array().map(Vec::len), Some(30));
    assert_eq!(state["clauses"].as_array().map(Vec::len), Some(4));
    let whole = document.to_string().len();
    eprintln!(
        "the state at the real run's shape: a digest of {} bytes, of a document of {whole}",
        seen.raw.trim_end().len()
    );
    assert!(
        whole > 70_000,
        "the control: the document at this shape is what was measured, {whole} bytes"
    );
    assert!(
        seen.raw.len() <= 6_500,
        "the digest at the real run's shape is {} bytes: more than 6,500",
        seen.raw.len()
    );

    // An item is one entry: what a stage needs to launch it, and where its run stands.
    assert_eq!(
        state["items"][1],
        json!({"item": "review-001", "kind": "review-row", "clause": "no-lost-files",
               "runs": "in-scope", "selected": true, "doors": 1, "standing": "not-selected"}),
        "{}",
        seen.raw
    );
    assert_eq!(
        json!([state["items"][0]["doors"], state["items"][0]["selected"]]),
        json!([150, true]),
        "an item that runs on every candidate and names no door covers the round's"
    );
    // What nothing can hold is named in it (K4): the doors of the round no item reaches,
    // as a count per round — 150 inside, 29 of them an item's.
    assert_eq!(state["uncovered"], json!([{"round": 1, "doors": 121}]));
    // No round is tested yet, so no tested round has selected an in-scope item: all 29.
    assert_eq!(state["never_selected"].as_array().map(Vec::len), Some(29));
    // And every field the harness reads of the document is there, of the document's value.
    for name in [
        "run",
        "opened",
        "next",
        "not_ready",
        "stop",
        "round",
        "fix_rounds",
        "candidate",
        "retest",
        "human_clauses",
        "human_stages",
        "unsettled",
    ] {
        assert_eq!(state[name], document[name], "`{name}` is the document's");
    }
    assert_eq!(state["position"], document["position"]);
    assert_eq!(state["facts"]["previous"], "1.0.0-rc.24");
    assert_eq!(state["facts"]["previous-commit"], "9".repeat(40).as_str());
    assert_eq!(state["blockers"], 0);
    assert_eq!(state["reverify"], json!([]));
    // What forbids closing, by kind: the clauses that are not green by name, the findings
    // that are not recorded as a count, and whether the candidate is untested.
    assert_eq!(
        state["forbids_close"],
        json!({"clauses": [
                   {"clause": "audit-clean", "status": "void"}, {"clause": "no-lost-files", "status": "void"},
                   {"clause": "ports-clean", "status": "void"}, {"clause": "trial-clean", "status": "void"}],
               "no_clauses": false, "findings": 60, "candidate": true}),
        "{}",
        seen.raw
    );

    // Two states this rig is not in, as the record script prints them, put to the tool's
    // own projection. A RE-RUN: its items, each with how many of THE RE-RUN'S doors it
    // covers — which are not the latest round's — and none of the doors themselves. And
    // THE HUMAN'S LIST after a retry: the keys a further triage may be granted to.
    let mut document = document;
    document["position"]["test"] = json!({"round": 1, "attempt": 2, "rerun": {
        "clause": "audit-clean", "round": 1,
        "items": [{"item": "review-001", "attempt": 2}, {"item": "gate", "attempt": 1}],
        "doors": [document["doors"]["included"][0], document["doors"]["included"][1], document["doors"]["included"][2]],
    }});
    document["human_list"][3]["why"] = json!("ungraded-after-retry");
    let forged = json!({"act": "state", "status": "read", "branch": LOOP, "state": document});
    let digest = rig.projected(&forged.to_string());
    assert_eq!(digest["unfit"], json!([]), "{digest}");
    assert_eq!(
        digest["state"]["position"]["test"],
        json!({"round": 1, "attempt": 2, "rerun": {"clause": "audit-clean", "round": 1, "items": [
            {"item": "review-001", "attempt": 2, "doors": 1}, {"item": "gate", "attempt": 1, "doors": 3}]}}),
        "{digest}"
    );
    assert_eq!(digest["state"]["reverify"], json!(["row-004"]));
    assert_eq!(digest["state"]["human_list"], 20);
}

/// **What a line leaves out is in a file it names** — by its path under the root the flag
/// names, and by its hash: the document for the state, and for every other act the line
/// the act prints unasked, a refusal's halt report in it. And **without the flag every
/// line is what it was**: for the acts that only read, the line printed unasked is, byte
/// for byte, the file that the same act's digest names.
#[test]
fn what_a_line_leaves_out_is_in_a_file_it_names() {
    let stage = Stage::new("digest-file");
    let rig = &stage.rig;
    let scratch = scratch_of(rig);
    stage.apply(SUBJECT);

    // The state: the file is the record script's document, where the act always kept it.
    let asked = [
        "state",
        "--run",
        RUN,
        "--scratch",
        scratch.as_str(),
        "--tag",
        "test-1",
    ];
    let (seen, document) = rig.state_digest("test-1");
    let unasked = rig.step(&asked);
    assert_eq!(unasked.done("state", "read")["state"], document);
    assert_eq!(
        unasked.line["file"],
        rig.scratch
            .join("state/test-1.json")
            .display()
            .to_string()
            .as_str()
    );
    assert_eq!(
        seen.line["file"], "state/test-1.json",
        "by its path under the root"
    );
    assert_eq!(
        rig.wrote(&format!("state --run {RUN}"), ""),
        *seen.kept.as_ref().expect("the file"),
        "the file is what the record script printed"
    );

    // Every other act: the line it prints unasked.
    let product = product();
    let look: Vec<&str> = git_state("test", &product)
        .into_iter()
        .chain(["--look"])
        .collect();
    for (act, args) in [
        ("git-state", look),
        ("table", vec!["table"]),
        ("find-round", vec!["find-round", "--prefix", ROUNDS]),
    ] {
        let unasked = rig.step(&args);
        let seen = rig.digest(&args);
        assert_eq!(
            seen.kept.as_deref(),
            Some(unasked.raw.as_str()),
            "`{act}`: the file its digest names is the line it prints unasked, byte for byte"
        );
        let file = seen.line["file"].as_str().expect("a file");
        assert!(
            file.starts_with(&format!("lines/{act}.")) && file.ends_with(".json"),
            "`{act}`: the file is named after the act and its content: {file}"
        );
    }
    // What a digest leaves out of a pending batch — its subject, its paths, its checks —
    // is in the file, and is in no digest.
    let seen = rig.digest(&git_state("test", &product));
    let whole = seen.of_its_line(rig);
    assert_eq!(whole["pending"]["subject"], SUBJECT);
    assert!(whole["pending"]["files"].is_array() && whole["pending"]["checks"].is_array());
    assert!(
        !seen.raw.contains("record"),
        "no word of the subject is in the digest: {}",
        seen.raw
    );

    // A root the files cannot lie under is refused before the act runs, and the digest
    // that says so names no file: a root that is no absolute directory of plain segments,
    // and a link planted beneath the root, which would carry the write elsewhere.
    let head = rig.rev("HEAD");
    for root in ["relative/dir", "/tmp/a b"] {
        let seen = rig.step(&["push", "--digest", root, "--branch", LOOP]);
        let digest = held_as_a_digest(&seen.raw);
        assert_eq!(seen.code, 2, "{}", seen.raw);
        assert_eq!(
            json!([digest["status"], digest["refused"], digest["file"]]),
            json!(["halted", "usage", null]),
            "{}",
            seen.raw
        );
    }
    let planted = rig.dir().join("planted");
    let elsewhere = rig.dir().join("elsewhere");
    fs::create_dir_all(&planted).expect("a root");
    fs::create_dir_all(&elsewhere).expect("another place");
    std::os::unix::fs::symlink(&elsewhere, planted.join("lines")).expect("plant a link");
    let seen = rig.step(&[
        "push",
        "--digest",
        &planted.display().to_string(),
        "--branch",
        LOOP,
    ]);
    let digest = held_as_a_digest(&seen.raw);
    assert_eq!(
        json!([seen.code, digest["refused"].clone(), digest["file"].clone()]),
        json!([2, "usage", null]),
        "{}",
        seen.raw
    );
    assert!(
        seen.trace.is_empty(),
        "the act did not run: {:?}",
        seen.trace
    );
    assert_eq!(
        fs::read_dir(&elsewhere).expect("read it").count(),
        0,
        "nothing was written through the link"
    );
    // The state's file lies under the same root as every other: two roots are refused.
    let another = rig.dir().join("another-root").display().to_string();
    let seen = rig.step(&[
        "state",
        "--run",
        RUN,
        "--scratch",
        &scratch,
        "--tag",
        "test-2",
        "--digest",
        &another,
    ]);
    assert_eq!(
        held_as_a_digest(&seen.raw)["refused"],
        "usage",
        "{}",
        seen.raw
    );
    assert_eq!(rig.rev("HEAD"), head);
}

/// **Hostile content reaches no digest.** Every writer that takes a sentence is fed what a
/// real record is written in — dashes, arrows, typographic quotes, a backslash, a title of
/// 200 characters — and what no writer takes arrives another way: a commit's subject, the
/// name of a test a gate shows red, a file somebody left beside the reports whose name has
/// a line break in it. Each digest is still printable ASCII of plain values under its
/// bound, its hash holds, nothing of it is unfit — and the content is in the file the
/// digest names, which is what shows it was really there.
#[test]
fn hostile_content_reaches_no_digest() {
    let title = format!("{HOSTILE} {}", "a title that goes on \u{2014} ".repeat(9));
    let title = title.trim_end().to_owned();
    assert!(title.chars().count() >= 200);
    let rig = shaped("digest-hostile", 3, 2, 3, 1, &title);
    let product = product();
    let needle = "a title that goes on \u{2014} ";
    let holds = |seen: &Digested, what: &str| {
        assert_eq!(seen.line["unfit"], json!([]), "{what}: {}", seen.raw);
        assert!(
            seen.kept.as_ref().is_some_and(|kept| {
                kept.contains(needle) || kept.contains(&needle.replace('\u{2014}', "\\u2014"))
            }),
            "{what}: the control — the content is in the file the digest names"
        );
    };

    // The state, over a ledger, a test set and a scope written in it.
    let (seen, document) = rig.state_digest("test-1");
    holds(&seen, "the state");
    assert_eq!(document["items"][1]["brief"], title.as_str());

    // A pending batch whose subject is the title; and a gate red on a test of that name.
    let stage = Stage::new("digest-hostile-stage");
    let rig = &stage.rig;
    stage.apply(&title);
    let seen = rig.digest(&git_state("test", &product));
    seen.done(rig, "git-state", "ready");
    holds(&seen, "a pending batch's subject");
    assert_eq!(seen.line["pending"]["calls"], 7, "{}", seen.raw);
    fs::write(
        &stage.gate,
        RED_GATE.replace("jigc::g_flows flow01::a", &title),
    )
    .expect("a red gate");
    let scratch = scratch_of(rig);
    let record = [
        "record",
        "--branch",
        LOOP,
        "--run-dir",
        RUN_DIR,
        "--gate",
        stage.gate.as_str(),
        "--calls",
        "7",
        "--checks",
        "2",
        "--scratch",
        scratch.as_str(),
    ];
    let seen = rig.digest(&record);
    seen.refused(rig, "gate-red");
    holds(&seen, "a red test's name");

    // A file beside the reports that no reporter left, named with a line break, a quote
    // and a dash: the report check counts it, and names it in the file alone.
    let stray = format!("{RUN_DIR}/r1/reports/test/it\u{2019}s \u{2014} a\nstray \"file\".md");
    rig.write(&stray, "x\n");
    let mut check = vec![
        "check-reports",
        "--run",
        RUN,
        "--round",
        "1",
        "--stage",
        "test",
        "--attempt",
        "1",
        "--",
    ];
    check.extend(LAUNCHED);
    let seen = rig.digest(&check);
    seen.done(rig, "check-reports", "checked");
    assert_eq!(
        json!([
            seen.line["check"]["ok"],
            seen.line["check"]["extra"],
            seen.line["check"]["missing"]
        ]),
        json!([false, 1, []]),
        "{}",
        seen.raw
    );
    assert!(
        seen.kept
            .as_ref()
            .is_some_and(|kept| kept.contains("stray")),
        "the name is in the file"
    );
    // And the tree that file makes dirty: its name is the refusal's prose.
    let seen = rig.digest(&git_state("test", &product));
    seen.refused(rig, "dirty");
    assert!(
        seen.kept
            .as_ref()
            .is_some_and(|kept| kept.contains("stray"))
    );

    // WHAT DOES NOT FIT IS NOT PRINTED: a value that is no id where an id is declared — a
    // line the tool did not make — leaves the digest, and the digest says which field.
    let forged = json!({"act": "push", "status": "ready", "branch": title, "head": "not a sha",
                        "remote_head": null, "vetted": ["a", "b"]});
    let digest = rig.projected(&forged.to_string());
    assert_eq!(
        json!([
            digest["branch"],
            digest["head"],
            digest["vetted"],
            digest["unfit"]
        ]),
        json!([null, null, 2, ["branch", "head"]]),
        "{digest}"
    );
    let forged = json!({"act": "it\u{2019}s \u{2014} no act", "status": "read", "state": {"run": RUN, "opened": true, "next": "a sentence, not a word"}});
    let digest = rig.projected(&forged.to_string());
    assert_eq!(
        json!([digest["act"], digest["unfit"]]),
        json!([null, ["act"]]),
        "{digest}"
    );
}

/// **A digest survives the relay that no document did.** Observed on 2026-10-07 (the relay
/// probe, twelve of twelve): an agent's structured return decodes the `\uXXXX` escapes of
/// the line it relays, and the harness's hash of the bytes fails. The relay is modelled
/// here as that decoding — every escape of the line replaced by its character — applied to
/// both forms of the same state: the line with the document in it comes back altered, and
/// the digest comes back as it was sent, because it holds no escape. Where `node` is
/// installed the harness's own `readStep` reads the digest as it reads any line of the
/// tool, and refuses the decoded document.
#[test]
fn a_digest_survives_a_relay_that_decodes_its_escapes() {
    let rig = shaped("digest-relay", 3, 2, 3, 0, HOSTILE);
    let scratch = scratch_of(&rig);
    let asked = [
        "state",
        "--run",
        RUN,
        "--scratch",
        scratch.as_str(),
        "--tag",
        "test-1",
    ];
    let document = rig.step(&asked).raw;
    let digest = rig.digest(&asked).raw;
    // The relay: what a JSON reader makes of the line's escapes, written back as text.
    let relayed = |line: &str| -> String {
        let mut out = String::new();
        let mut rest = line;
        while let Some(at) = rest.find("\\u") {
            let code = u32::from_str_radix(&rest[at + 2..at + 6], 16).expect("an escape");
            out.push_str(&rest[..at]);
            out.push(char::from_u32(code).expect("a character"));
            rest = &rest[at + 6..];
        }
        out + rest
    };
    assert_ne!(
        relayed(&document),
        document,
        "the control: the document's line holds escapes"
    );
    assert_eq!(
        relayed(&digest),
        digest,
        "a digest holds none: it is relayed as it was sent"
    );
    assert!(document.len() > 2 * digest.len());

    if !node_or_skip(&format!(
        "{HARNESS}'s `readStep` was not run on a digest of {TOOL}"
    )) {
        return;
    }
    let read = harness_pure(&json!([
        ["readStep", [{"status": "ran", "line": relayed(&digest)}, "state"]],
        ["readStep", [{"status": "ran", "line": relayed(&document)}, "state"]],
    ]));
    assert!(
        read[0].get("relay").is_none()
            && read[0]["status"] == "read"
            && read[0]["state"]["next"] == "test"
            && read[0]["file"] == "state/test-1.json",
        "the harness reads a relayed digest: {}",
        read[0]
    );
    assert!(
        read[1]["relay"]
            .as_str()
            .is_some_and(|why| why.contains("does not end with the sha256 of itself")),
        "and the relayed document is what it was on 2026-10-07 — altered: {}",
        read[1]["relay"]
    );
}

// ---------------------------------------------------------------------------
// A held command: started, waited for and judged by the tool
// ---------------------------------------------------------------------------

/// How long a shell call of an agent runs before its tool gives up on it, in seconds, where
/// the agent names no other limit. A wait ends inside it, or the wait itself is what stops
/// the agent.
const AGENT_SHELL_SECONDS: u64 = 120;

/// The kinds of held command the tool has, each with the test of this suite that drives its
/// verdict — the tool's own table (`table`: `holds`) is held to this list.
const HELD_KINDS: &[(&str, &str)] = &[
    ("gate", "the_verdict_of_a_held_command_is_read_by_the_tool"),
    (
        "regression",
        "the_verdict_of_a_held_command_is_read_by_the_tool",
    ),
    (
        "build",
        "a_binary_is_built_from_a_commit_and_judged_by_its_hash_its_version_and_its_path",
    ),
    ("probe", "the_verdict_of_a_held_command_is_read_by_the_tool"),
];

/// A stand-in for the command of a held kind — `dev/gate`, `dev/regression-set`. It writes
/// down that it ran, with its pid and its arguments; waits until the test releases it — a
/// file, never a number of seconds, so that no arm races a clock; prints what a file holds;
/// and exits as it is told.
const LONG_COMMAND: &str = r#"#!/bin/sh
printf '%s %s\n' "$$" "$*" >>"$HOLD_RAN"
while [ ! -e "$HOLD_RELEASE" ]; do sleep 0.1; done
cat "$HOLD_PRINTS"
exit "${HOLD_EXIT:-0}"
"#;

/// A stand-in for cargo: `build … --target-dir DIR`, run in an unpacked commit. The binary
/// it "builds" prints the version `Cargo.toml` of that tree names — so what it built is
/// read off the tree it built from — or it fails, where the tree says so.
const CARGO: &str = r#"#!/bin/sh
dir=
while [ $# -gt 0 ]; do
  if [ "$1" = --target-dir ]; then dir=$2; fi
  shift
done
echo "   Compiling jigc (stand-in) in $(pwd -P)"
if [ -e fails-to-build ]; then echo "error: could not compile" >&2; exit 101; fi
mkdir -p "$dir/release"
printf '#!/bin/sh\necho "jigc %s"\n' "$(cat version.txt)" >"$dir/release/jigc"
chmod 755 "$dir/release/jigc"
"#;

/// What a real command of a held kind printed, recorded: `tooling-tests/fixtures/held/`.
/// The gate's four are `dev/gate` itself, run on a throwaway crate of two tests on
/// 2026-10-07 — `--keep-going` green and red, `--fast`, `--quick`; the regression tool's are
/// its own lines. Host paths are replaced by the public placeholders, and nothing else.
fn as_recorded(name: &str) -> String {
    read(&format!("tooling-tests/fixtures/held/{name}"))
}

/// A line of the regression tool as it prints one: the committed record of a real run,
/// which is that line laid out over many, as ONE line again.
fn regression_line(record: &str) -> (Value, String) {
    let line: Value = serde_json::from_str(&read(&format!(
        "completions/artifacts/M55/stabilization-build/regression-set/{record}"
    )))
    .expect("a committed line of the regression tool");
    let text = format!("{line}\n");
    (line, text)
}

const REGRESSION_LIST: &str =
    "completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv";

/// A rig whose held commands are stand-ins, and what each call tells them.
struct Held {
    rig: StepRig,
    scratch: String,
    ran: PathBuf,
    release: PathBuf,
}

impl Held {
    fn new(label: &str) -> Self {
        let rig = StepRig::new(label);
        for tool in ["dev/gate", "dev/regression-set"] {
            placed_executable::write(&rig.root.join(tool), LONG_COMMAND);
        }
        let scratch = rig.scratch.display().to_string();
        let (ran, release) = (rig.dir().join("ran"), rig.dir().join("release"));
        fs::write(&ran, "").expect("nothing ran yet");
        Held {
            rig,
            scratch,
            ran,
            release,
        }
    }

    /// The stand-ins run to their end at once, or wait to be released.
    fn released(&self, yes: bool) {
        if yes {
            fs::write(&self.release, "").expect("release the command");
        } else if self.release.exists() {
            fs::remove_file(&self.release).expect("hold the command");
        }
    }

    /// `hold-start` of `name` — `flags` are the kind and what it takes — with a command that
    /// prints `prints` and exits `exit`.
    fn start(&self, name: &str, flags: &[&str], prints: &str, exit: i32) -> Stepped {
        let file = self.rig.dir().join(format!("prints-{name}"));
        fs::write(&file, prints).expect("what the command prints");
        let (file, exit) = (file.display().to_string(), exit.to_string());
        let (ran, release) = (
            self.ran.display().to_string(),
            self.release.display().to_string(),
        );
        let mut args = vec!["hold-start", "--scratch", &self.scratch, "--name", name];
        args.extend(flags);
        self.rig.step_in(
            &[
                ("HOLD_RAN", ran.as_str()),
                ("HOLD_RELEASE", release.as_str()),
                ("HOLD_PRINTS", file.as_str()),
                ("HOLD_EXIT", exit.as_str()),
            ],
            &args,
        )
    }

    /// `hold-wait` of `name`, within `slice` seconds — or within the tool's own.
    fn wait(&self, name: &str, slice: Option<u32>) -> Stepped {
        let slice = slice.map(|seconds| seconds.to_string());
        let mut args = vec!["hold-wait", "--scratch", &self.scratch, "--name", name];
        if let Some(slice) = &slice {
            args.extend(["--slice", slice.as_str()]);
        }
        self.rig.step(&args)
    }

    /// Started with a command that runs to its end at once, and waited for.
    fn judged(&self, name: &str, flags: &[&str], prints: &str, exit: i32) -> Value {
        self.released(true);
        self.start(name, flags, prints, exit)
            .done("hold-start", "started");
        let seen = self.wait(name, Some(60));
        assert_eq!(seen.code, 0, "{} {}", seen.raw, seen.stderr);
        seen.line
    }

    /// The commands that ran, once `count` of them have: a command is started by a process
    /// of its own, a moment after the start has answered.
    fn runs_when(&self, count: usize) -> Vec<String> {
        until("the command has started", || self.runs().len() >= count);
        self.runs()
    }

    /// The commands that ran: `<pid> <arguments>` each.
    fn runs(&self) -> Vec<String> {
        fs::read_to_string(&self.ran)
            .expect("what ran")
            .lines()
            .map(str::to_owned)
            .collect()
    }

    /// The file a job keeps under the scratch root.
    fn kept(&self, name: &str, file: &str) -> PathBuf {
        self.rig.scratch.join("hold").join(name).join(file)
    }

    /// The verdict file a line names: there, ONE line that ends with its own sha256, and
    /// the file the line names it by.
    fn verdict_of(&self, line: &Value) -> Value {
        let rel = line["verdict_file"]
            .as_str()
            .unwrap_or_else(|| panic!("a job that is done names its verdict's file: {line}"));
        let text = fs::read_to_string(self.rig.scratch.join(rel)).expect("the verdict file");
        assert_eq!(
            line["verdict_sha256"],
            sha256(&text).as_str(),
            "the line names the file by its hash: {line}"
        );
        let verdict = line_of(&text);
        assert_eq!(
            json!([
                verdict["tool"],
                verdict["name"],
                verdict["kind"],
                verdict["verdict"],
                verdict["why"]
            ]),
            json!([
                "stabilize-step",
                line["name"],
                line["kind"],
                line["verdict"],
                line["why"]
            ]),
            "the file says what the line says: {verdict}"
        );
        verdict
    }
}

/// Wait — a minute at most — until `holds`.
fn until(what: &str, holds: impl Fn() -> bool) {
    let asked = std::time::Instant::now();
    while !holds() {
        assert!(
            asked.elapsed() < std::time::Duration::from_secs(60),
            "after a minute, still not: {what}"
        );
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

const GATE_KIND: [&str; 4] = ["--kind", "gate", "--run", RUN];

fn regression_kind<'a>(previous: &'a str, candidate: &'a str) -> Vec<&'a str> {
    vec![
        "--kind",
        "regression",
        "--previous",
        previous,
        "--candidate",
        candidate,
        "--list",
        REGRESSION_LIST,
    ]
}

/// **A long command is started ONCE, by name, and it outlives the process that started it**
/// (the second repair plan's `K10`; the human's ruling of 2026-10-07 on the permission
/// prompt; the hold probe's case (b)). `hold-start` answers at once — while the command
/// still runs — and names the file the command's output is kept in, which is the tool's to
/// name and no redirect of anybody's. Asked again under the same name it answers with the
/// job that is there, running or done, and nothing is started a second time; the same
/// name with other arguments is refused (`taken`). And ANOTHER process waits for it, after
/// the starter has exited, and reads its verdict.
#[test]
fn a_held_command_is_started_once_and_outlives_its_starter() {
    let held = Held::new("hold-once");
    let green = as_recorded("gate-green.txt");
    held.released(false);
    let first = held.start("gate-c1-a1", &GATE_KIND, &green, 0);
    let started = first.done("hold-start", "started").clone();
    assert_eq!(
        json!([started["name"], started["kind"], started["output"]]),
        json!(["gate-c1-a1", "gate", "hold/gate-c1-a1/output"]),
        "the job, and the file its output is kept in — under the scratch root: {started}"
    );
    let pid = started["pid"].as_u64().expect("the supervisor's pid");
    assert!(
        first.trace.is_empty(),
        "a start asks git nothing: {:?}",
        first.trace
    );

    // The starter has exited — its call returned — and the command runs.
    let at_once = held.wait("gate-c1-a1", Some(0));
    assert_eq!(
        json!([
            at_once.done("hold-wait", "running")["pid"],
            at_once.line["verdict"]
        ]),
        json!([pid, null]),
        "{}",
        at_once.raw
    );
    assert_eq!(
        held.runs_when(1).len(),
        1,
        "the command was started: {:?}",
        held.runs()
    );
    assert!(
        held.runs()[0].ends_with(" --keep-going"),
        "the gate's command is the tool's own, `dev/gate --keep-going`: {:?}",
        held.runs()
    );

    // ASKED TWICE, IT ANSWERS WITH THE JOB THAT IS RUNNING — never a second one.
    let again = held.start("gate-c1-a1", &GATE_KIND, &green, 0);
    assert_eq!(again.done("hold-start", "running")["pid"], pid);
    assert_eq!(
        held.runs().len(),
        1,
        "and nothing was started: {:?}",
        held.runs()
    );
    // The same name, another command: refused, and nothing is started.
    let other = regression_kind(&"a".repeat(40), &"b".repeat(40))
        .iter()
        .map(|flag| (*flag).to_owned())
        .collect::<Vec<_>>();
    let other: Vec<&str> = other.iter().map(String::as_str).collect();
    held.start("gate-c1-a1", &other, &green, 0).refused("taken");
    held.start(
        "gate-c1-a1",
        &["--kind", "gate", "--run", "another"],
        &green,
        0,
    )
    .refused("taken");
    assert_eq!(held.runs().len(), 1);

    // ANOTHER PROCESS WAITS, the starter long gone, and reads the verdict.
    held.released(true);
    let ended = held.wait("gate-c1-a1", Some(60));
    let done = ended.done("hold-wait", "done");
    assert_eq!(
        json!([done["verdict"], done["why"], done["exit"], done["pid"]]),
        json!(["green", null, 0, pid]),
        "{done}"
    );
    assert_eq!(
        fs::read_to_string(held.kept("gate-c1-a1", "output")).expect("the output"),
        green,
        "the command's output, whole, in the file the tool named"
    );
    held.verdict_of(done);

    // Asked for once more when it is done: the job that is there, with its verdict — and
    // it is not run again.
    let after = held.start("gate-c1-a1", &GATE_KIND, &green, 0);
    assert_eq!(
        json!([
            after.done("hold-start", "done")["verdict"],
            after.line["verdict_sha256"]
        ]),
        json!(["green", done["verdict_sha256"]])
    );
    assert_eq!(held.runs().len(), 1, "it ran once: {:?}", held.runs());
    // A name nobody started is no job.
    held.wait("gate-c9-a9", Some(0)).refused("missing");
}

/// **A wait answers within its slice — a number the tool owns, shorter than an agent's
/// shell lets a call run.** While the command runs, `hold-wait` returns `running` when the
/// slice is over, having waited that long and no longer; the slice an agent may name only
/// shortens it. Two waiters at once are both answered, with one verdict — so a wait can be
/// taken over.
#[test]
fn a_wait_answers_within_its_slice() {
    let held = Held::new("hold-slice");
    let green = as_recorded("gate-green.txt");
    held.released(false);
    held.start("gate", &GATE_KIND, &green, 0)
        .done("hold-start", "started");

    // The slice is over, and the command still runs: `running`, after that long.
    let asked = std::time::Instant::now();
    let waiting = held.wait("gate", Some(2));
    let took = asked.elapsed().as_secs_f64();
    assert_eq!(
        json!([
            waiting.done("hold-wait", "running")["waited"],
            waiting.line["slice"]
        ]),
        json!([2, 2]),
        "{}",
        waiting.raw
    );
    assert!(
        (2.0..60.0).contains(&took),
        "the wait took its slice, and no more than a process costs beside it: {took} s"
    );

    // THE TOOL'S OWN SLICE: what a wait is bounded by where none is named — shorter than
    // an agent's shell lets a call run — and a longer one is no argument.
    let table = held.rig.step(&["table"]);
    let own = table.done("table", "listed")["slice"]
        .as_u64()
        .expect("the tool's slice");
    assert!(
        (30..AGENT_SHELL_SECONDS - 10).contains(&own),
        "the tool's slice ends inside an agent's default shell limit of {AGENT_SHELL_SECONDS} s, with room for the call itself: {own}"
    );
    let longer = (own + 1).to_string();
    held.rig
        .step(&[
            "hold-wait",
            "--scratch",
            &held.scratch,
            "--name",
            "gate",
            "--slice",
            &longer,
        ])
        .refused("usage");

    // TWO WAITERS AT ONCE, and the command ends under them: both are answered, alike.
    let (first, second) = std::thread::scope(|threads| {
        let one = threads.spawn(|| held.wait("gate", None));
        let two = threads.spawn(|| held.wait("gate", Some(60)));
        std::thread::sleep(std::time::Duration::from_millis(600));
        held.released(true);
        (one.join().expect("a waiter"), two.join().expect("a waiter"))
    });
    assert_eq!(
        first.done("hold-wait", "done")["slice"],
        own,
        "a wait that names no slice is bounded by the tool's: {}",
        first.raw
    );
    for seen in [&first, &second] {
        assert_eq!(seen.done("hold-wait", "done")["verdict"], "green");
    }
    assert_eq!(
        first.line["verdict_sha256"], second.line["verdict_sha256"],
        "one verdict, whoever read it first"
    );
    assert!(
        first.line["waited"]
            .as_u64()
            .is_some_and(|waited| waited < own),
        "a wait ends when the command does: {}",
        first.raw
    );
    assert_eq!(held.runs().len(), 1);
}

/// **A job whose process is gone without its end on record is `dead` — named, and never
/// `done`.** The supervisor killed: no record of how the command ended. The command killed:
/// a record that says by which signal. Neither is read for a verdict, whatever the output
/// holds — here a whole green gate, written before the kill — and neither is started again
/// under its name.
#[test]
fn a_held_command_whose_process_is_gone_is_dead_and_never_done() {
    let held = Held::new("hold-dead");
    let green = as_recorded("gate-green.txt");
    let kill = |pid: u64| {
        let out = Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output()
            .expect("run kill");
        assert!(out.status.success(), "kill {pid}: {out:?}");
    };
    held.released(false);

    // The supervisor is killed.
    let started = held.start("gate-a1", &GATE_KIND, &green, 0);
    let pid = started.done("hold-start", "started")["pid"]
        .as_u64()
        .expect("the supervisor's pid");
    // The command runs — and the output holds a whole green gate already: it is not what
    // makes a job done.
    held.runs_when(1);
    fs::write(held.kept("gate-a1", "output"), &green).expect("write the output");
    kill(pid);
    let gone = held.wait("gate-a1", Some(60));
    let dead = gone.done("hold-wait", "dead");
    assert_eq!(
        json!([
            dead["why"],
            dead["verdict"],
            dead["verdict_file"],
            dead["exit"]
        ]),
        json!(["no-exit", null, null, null]),
        "{dead}"
    );
    assert!(!held.kept("gate-a1", "verdict.json").exists());
    assert_eq!(
        held.start("gate-a1", &GATE_KIND, &green, 0)
            .done("hold-start", "dead")["why"],
        "no-exit",
        "asked for again, a dead job is named — and nothing is started under its name"
    );
    assert_eq!(held.runs().len(), 1, "{:?}", held.runs());

    // The command is killed, its supervisor alive: the record says by what.
    held.start("gate-a2", &GATE_KIND, &green, 0)
        .done("hold-start", "started");
    let command: u64 = held.runs_when(2)[1]
        .split(' ')
        .next()
        .and_then(|pid| pid.parse().ok())
        .expect("the command's pid");
    kill(command);
    let gone = held.wait("gate-a2", Some(60));
    let dead = gone.done("hold-wait", "dead");
    assert_eq!(
        json!([dead["why"], dead["signal"], dead["verdict"], dead["exit"]]),
        json!(["killed", 9, null, null]),
        "{dead}"
    );
    held.released(true);
}

/// **The verdict of a held command is read by the tool, out of the command's own output —
/// per kind, and from recorded real output.** A gate: the record script's own reader of a
/// full gate's two lines, so a pre-check's output (`--fast`, and `--quick`, which prints a
/// verdict line and no totals) is no verdict. The regression set: its ONE line, held to
/// the exit status its tool gives that line and to the two commits and the list that were
/// asked. A verdict that is missing, cut off, another kind's, or that the exit status does
/// not bear out is `void` and says why — `done`, and never green.
///
/// Recorded, and what is not: the gate's four outputs and the regression tool's green,
/// red, `evidence`, `did-not-run` and refused lines are real ([`as_recorded`],
/// [`regression_line`]). The five other void reasons are the recorded `did-not-run` line
/// with the reason's word, at the exit status the tool's header gives that word: a void
/// line is read by three fields, and no run of thirty-five minutes was made to print each.
#[test]
fn the_verdict_of_a_held_command_is_read_by_the_tool() {
    let held = Held::new("hold-verdict");
    let (green, red) = (as_recorded("gate-green.txt"), as_recorded("gate-red.txt"));
    let (ran_green, green_line) = regression_line("run-3-result.json");
    let (ran_red, red_line) = regression_line("run-2-result.json");
    let commits = |line: &Value| {
        (
            line["previous"]["commit"]
                .as_str()
                .expect("a sha")
                .to_owned(),
            line["candidate"]["commit"]
                .as_str()
                .expect("a sha")
                .to_owned(),
        )
    };
    let (previous, candidate) = commits(&ran_green);
    let (red_previous, red_candidate) = commits(&ran_red);

    // --- a gate -------------------------------------------------------------------
    let seen = held.judged("gate-green", &GATE_KIND, &green, 0);
    assert_eq!(
        json!([
            seen["status"],
            seen["verdict"],
            seen["why"],
            seen["exit"],
            seen["facts"]
        ]),
        json!(["done", "green", null, 0,
               {"passed": 2, "failed": 0, "binaries": 3, "steps": [], "tests": 0}]),
        "a green gate, from its two lines: {seen}"
    );
    let on_file = held.verdict_of(&seen);
    assert_eq!(
        on_file["evidence"]["totals"], "tests   passed=2 failed=0  (over 3 test binaries)",
        "the totals line is in the file: {on_file}"
    );
    let seen = held.judged("gate-red", &GATE_KIND, &red, 1);
    assert_eq!(
        json!([
            seen["status"],
            seen["verdict"],
            seen["why"],
            seen["exit"],
            seen["facts"]
        ]),
        json!(["done", "red", null, 1,
               {"passed": 1, "failed": 1, "binaries": 3, "steps": ["test"], "tests": 1}]),
        "a red gate: its red steps, and how many tests: {seen}"
    );
    assert_eq!(
        held.verdict_of(&seen)["evidence"]["red"],
        json!(["step test", "test tiny::numbers two_is_three"]),
        "what is red is named in the file"
    );
    let cut = &green[..green.find("\nGATE: PASS").expect("the verdict line")];
    for (name, prints, exit, why) in [
        ("gate-fast", as_recorded("gate-fast.txt"), 0, "no-verdict"),
        ("gate-quick", as_recorded("gate-quick.txt"), 0, "no-verdict"),
        ("gate-cut", cut.to_owned(), 0, "no-verdict"),
        ("gate-nothing", String::new(), 0, "no-verdict"),
        ("gate-other-kind", green_line.clone(), 0, "no-verdict"),
        ("gate-green-exit-1", green.clone(), 1, "exit-differs"),
        ("gate-red-exit-0", red.clone(), 0, "exit-differs"),
    ] {
        let seen = held.judged(name, &GATE_KIND, &prints, exit);
        assert_eq!(
            json!([seen["status"], seen["verdict"], seen["why"]]),
            json!(["done", "void", why]),
            "`{name}`: {seen}"
        );
        held.verdict_of(&seen);
    }

    // --- the regression set ---------------------------------------------------------
    let asked = regression_kind(&previous, &candidate);
    let seen = held.judged("regression-green", &asked, &green_line, 0);
    assert_eq!(
        json!([seen["status"], seen["verdict"], seen["why"], seen["facts"]]),
        json!(["done", "green", null,
               {"status": "green", "reason": null, "previous": previous, "candidate": candidate,
                "list_sha256": ran_green["list"]["sha256"], "differences": 22,
                "not_on_list": 0, "excluded": 11, "excluded_by_no_row": 0}]),
        "{seen}"
    );
    assert_eq!(
        held.verdict_of(&seen)["evidence"]["line"],
        ran_green,
        "the tool's own line is in the file, whole"
    );
    assert!(
        held.runs().last().is_some_and(|ran| ran.contains(&format!(
            " run --previous {previous} --candidate {candidate} --list {REGRESSION_LIST} --scratch "
        ))),
        "the command is the tool's own, from the commits and the list asked: {:?}",
        held.runs().last()
    );
    let red_asked = regression_kind(&red_previous, &red_candidate);
    let seen = held.judged("regression-red", &red_asked, &red_line, 1);
    assert_eq!(
        json!([seen["verdict"], seen["why"], seen["facts"]["not_on_list"]]),
        json!(["red", null, 1]),
        "{seen}"
    );
    // Void, and why: each reason the tool has, at the exit status it gives it.
    let did_not_run = as_recorded("regression-void-did-not-run.txt");
    let ran_void: Value = serde_json::from_str(&did_not_run).expect("a void line");
    let (void_previous, void_candidate) = commits(&ran_void);
    let void_asked = regression_kind(&void_previous, &void_candidate);
    let seen = held.judged("regression-did-not-run", &void_asked, &did_not_run, 10);
    assert_eq!(
        json!([seen["verdict"], seen["why"], seen["facts"]["reason"]]),
        json!(["void", "did-not-run", "did-not-run"]),
        "{seen}"
    );
    let seen = held.judged(
        "regression-evidence",
        &void_asked,
        &as_recorded("regression-void-evidence.txt"),
        16,
    );
    assert_eq!(
        json!([seen["verdict"], seen["why"]]),
        json!(["void", "evidence"])
    );
    for (reason, exit) in [
        ("build-previous", 11),
        ("build-candidate", 12),
        ("baseline", 13),
        ("swap", 14),
        ("incomplete", 15),
    ] {
        let line = did_not_run.replace(
            "\"reason\": \"did-not-run\"",
            &format!("\"reason\": \"{reason}\""),
        );
        assert_ne!(line, did_not_run);
        let seen = held.judged(&format!("regression-{reason}"), &void_asked, &line, exit);
        assert_eq!(
            json!([seen["status"], seen["verdict"], seen["why"]]),
            json!(["done", "void", reason]),
            "{seen}"
        );
    }
    // A refusal is no run: void. So is a line the exit status does not bear out, a line of
    // other commits than the ones asked, a line that was cut off, and another kind's output.
    let half = &green_line[..green_line.len() / 2];
    for (name, flags, prints, exit, why) in [
        (
            "regression-refused",
            &asked,
            as_recorded("regression-refused.txt"),
            4,
            "refused",
        ),
        (
            "regression-green-exit-1",
            &asked,
            green_line.clone(),
            1,
            "exit-differs",
        ),
        (
            "regression-red-exit-0",
            &red_asked,
            red_line.clone(),
            0,
            "exit-differs",
        ),
        (
            "regression-void-exit-0",
            &void_asked,
            did_not_run.clone(),
            0,
            "exit-differs",
        ),
        (
            "regression-other-commits",
            &red_asked,
            green_line.clone(),
            0,
            "not-asked",
        ),
        ("regression-cut", &asked, half.to_owned(), 0, "no-verdict"),
        (
            "regression-twice",
            &asked,
            format!("{green_line}{green_line}"),
            0,
            "no-verdict",
        ),
        (
            "regression-other-kind",
            &asked,
            green.clone(),
            0,
            "no-verdict",
        ),
    ] {
        let seen = held.judged(name, flags, &prints, exit);
        assert_eq!(
            json!([seen["status"], seen["verdict"], seen["why"]]),
            json!(["done", "void", why]),
            "`{name}`: {seen}"
        );
    }

    // --- the probe's hold: the real tool, a second long ---------------------------------
    placed_executable::copy(
        &repo_root().join("dev/stabilize-probe"),
        &held.rig.root.join("dev/stabilize-probe"),
    );
    let begun = held
        .rig
        .hermetic(Command::new(held.rig.root.join("dev/stabilize-probe")))
        .args(["begin", "--scratch", &held.scratch, "--probe", "hold"])
        .output()
        .expect("run the probe tool");
    assert!(begun.status.success(), "{begun:?}");
    let probe = ["--kind", "probe", "--seconds", "1"];
    let seen = held.judged("probe-a", &probe, "", 0);
    assert_eq!(
        json!([seen["status"], seen["verdict"], seen["why"], seen["facts"]]),
        json!(["done", "green", null, {"seconds": 1}]),
        "the probe's own last line, of the name and the seconds asked: {seen}"
    );
    // A name the probe tool has held already is its refusal: no last line, void.
    let seen = held.judged("probe-b", &probe, "", 0);
    assert_eq!(seen["verdict"], "green", "{seen}");
    fs::remove_dir_all(held.rig.scratch.join("hold/probe-b")).expect("forget the job");
    let seen = held.judged("probe-b", &probe, "", 0);
    assert_eq!(
        json!([seen["status"], seen["verdict"], seen["why"]]),
        json!(["done", "void", "no-verdict"]),
        "{seen}"
    );
    // THE PROBE'S OWN LINE, RECORDED — a hold of one second under the names `probe-c` and
    // `probe-d` — and then a stand-in in the probe tool's place that prints it: a line of
    // other seconds than were asked is not the hold that was asked for, and a line that
    // does not hold to its own hash is no line of that tool at all.
    let real = |name: &str| {
        let out = held
            .rig
            .hermetic(Command::new(held.rig.root.join("dev/stabilize-probe")))
            .args([
                "hold",
                "--scratch",
                &held.scratch,
                "--name",
                name,
                "--seconds",
                "1",
            ])
            .output()
            .expect("run the probe tool");
        assert!(out.status.success(), "{out:?}");
        String::from_utf8(out.stdout).expect("the probe's line")
    };
    let (line_c, line_d) = (real("probe-c"), real("probe-d"));
    placed_executable::write(&held.rig.root.join("dev/stabilize-probe"), LONG_COMMAND);
    let two = ["--kind", "probe", "--seconds", "2"];
    let seen = held.judged("probe-c", &two, &line_c, 0);
    assert_eq!(
        json!([seen["status"], seen["verdict"], seen["why"]]),
        json!(["done", "void", "not-asked"]),
        "{seen}"
    );
    let retyped = line_d.replace("\"seconds\": 1,", "\"seconds\": 2,");
    assert_ne!(retyped, line_d, "the line says its seconds: {line_d}");
    let seen = held.judged("probe-d", &two, &retyped, 0);
    assert_eq!(
        json!([seen["status"], seen["verdict"], seen["why"]]),
        json!(["done", "void", "no-verdict"]),
        "a line retyped to say what was asked does not hold to its hash: {seen}"
    );
}

/// **The exit statuses a regression verdict is held to are the regression tool's own**: the
/// tool's table (`table`: `holds`) names, per status of that tool's line, the exit status
/// that goes with it, and this arm reads the same table out of the header of
/// `dev/regression-set`.
#[test]
fn the_exits_a_regression_verdict_is_held_to_are_the_regression_tools_own() {
    let header = read("dev/regression-set");
    let at = header
        .find("# Exit status, and the word a refusal opens with")
        .expect("the regression tool's table of exit statuses");
    let theirs: BTreeMap<String, u64> = header[at..]
        .lines()
        .skip(1)
        .take_while(|line| line.starts_with('#'))
        .filter_map(|line| {
            let mut words = line.trim_start_matches('#').split_whitespace();
            let status: u64 = words.next()?.parse().ok()?;
            Some((words.next()?.to_owned(), status))
        })
        .collect();
    assert!(theirs.len() >= 12, "the scan read the table: {theirs:?}");
    let rig = StepRig::new("hold-exits");
    let table = rig.step(&["table"]);
    let holds = table.done("table", "listed")["holds"]
        .as_array()
        .expect("the kinds")
        .clone();
    let regression = holds
        .iter()
        .find(|kind| kind["kind"] == "regression")
        .expect("the regression set's row");
    let ours: BTreeMap<String, u64> = regression["exits"]
        .as_object()
        .expect("the exits")
        .iter()
        .map(|(word, status)| (word.clone(), status.as_u64().expect("a status")))
        .collect();
    assert_eq!(
        ours, theirs,
        "the exits the step tool holds a regression line to (left) are dev/regression-set's (right)"
    );
    // And the kinds are the ones this suite drives, each by a test of this file.
    let source = read("tooling-tests/dev_stabilize_step.rs");
    assert_eq!(
        holds
            .iter()
            .map(|kind| kind["kind"].as_str().expect("a kind"))
            .collect::<Vec<_>>(),
        HELD_KINDS.iter().map(|(kind, _)| *kind).collect::<Vec<_>>(),
        "the kinds of held command (left) are the ones this suite drives (right)"
    );
    for (kind, test) in HELD_KINDS {
        assert!(
            source.contains(&format!("\nfn {test}()")),
            "`{kind}` is driven by `{test}`"
        );
    }
    for kind in &holds {
        for field in ["command", "judged", "takes"] {
            assert!(
                kind[field].is_string() || kind[field].is_array(),
                "a kind says `{field}`: {kind}"
            );
        }
    }
}

/// A build of the rig's `HEAD` under `name`, to `to` under the scratch root.
fn build_kind<'a>(commit: &'a str, to: &'a str, version: Option<&'a str>) -> Vec<&'a str> {
    let mut flags = vec!["--kind", "build", "--commit", commit, "--to", to];
    if let Some(version) = version {
        flags.extend(["--version", version]);
    }
    flags
}

/// **A binary is built from a COMMIT, as a held command, and judged by the tool: by its
/// hash, the version it prints and the path a bare `jigc` resolves to** — under the suite's
/// stand-in for cargo, which builds what the tree it runs in says. So what was built is
/// the commit's, never the working tree's; the binary lands where it was asked for, under
/// the scratch root, read-only; a build that fails, or whose binary prints another version
/// than the one asked, is `red`; and a binary that is not the one the build left is `void`.
#[test]
fn a_binary_is_built_from_a_commit_and_judged_by_its_hash_its_version_and_its_path() {
    let held = Held::new("hold-build");
    let rig = &held.rig;
    rig.on_path("cargo", CARGO);
    rig.write("version.txt", "1.0.0-rc.24\n");
    let commit = rig.commit("chore: a version");
    // The working tree moves on, uncommitted: no build may see it.
    rig.write("version.txt", "9.9.9\n");
    held.released(true);

    held.start(
        "build-c1",
        &build_kind(&commit, "bin/c1/jigc", Some("1.0.0-rc.24")),
        "",
        0,
    )
    .done("hold-start", "started");
    let ended = held.wait("build-c1", Some(60));
    let done = ended.done("hold-wait", "done").clone();
    let binary = rig.scratch.join("bin/c1/jigc");
    let bytes = fs::read(&binary).expect("the binary, where it was asked for");
    assert_eq!(
        json!([done["verdict"], done["why"], done["facts"]]),
        json!(["green", null,
               {"commit": commit, "binary": "bin/c1/jigc", "bytes": bytes.len(),
                "content_sha256": sha256(&String::from_utf8_lossy(&bytes)),
                "version": "1.0.0-rc.24", "resolves": true}]),
        "the hash, the version and the path: {done}"
    );
    use std::os::unix::fs::PermissionsExt as _;
    assert_eq!(
        fs::metadata(&binary)
            .expect("the binary")
            .permissions()
            .mode()
            & 0o222,
        0,
        "the binary is read-only"
    );
    let unpacked =
        fs::canonicalize(held.kept("build-c1", "work/source")).expect("the unpacked commit");
    let cargo =
        fs::read_to_string(held.kept("build-c1", "work/cargo.log")).expect("cargo's output");
    assert_eq!(
        cargo,
        format!("   Compiling jigc (stand-in) in {}\n", unpacked.display()),
        "cargo ran in the unpacked commit, never in the repository — and its output is kept beside it"
    );
    assert_eq!(
        fs::read_to_string(held.kept("build-c1", "output"))
            .expect("the build's output")
            .lines()
            .count(),
        1,
        "the held build's output is the build's ONE line"
    );
    held.verdict_of(&done);

    // A binary that is no longer the one the build left: void.
    held.start("build-c2", &build_kind(&commit, "bin/c2/jigc", None), "", 0)
        .done("hold-start", "started");
    let swapped = rig.scratch.join("bin/c2/jigc");
    until("the build has ended", || {
        swapped.exists() && held.kept("build-c2", "exit.json").exists()
    });
    fs::remove_file(&swapped).expect("remove the binary");
    fs::write(&swapped, "#!/bin/sh\necho 'jigc 1.0.0-rc.24'\n").expect("another binary");
    let seen = held.wait("build-c2", Some(60));
    assert_eq!(
        json!([seen.done("hold-wait", "done")["verdict"], seen.line["why"]]),
        json!(["void", "hash-differs"]),
        "{}",
        seen.raw
    );

    // Another version than the one asked: red. A build that fails: red.
    held.start(
        "build-c3",
        &build_kind(&commit, "bin/c3/jigc", Some("1.0.0")),
        "",
        0,
    )
    .done("hold-start", "started");
    let seen = held.wait("build-c3", Some(60));
    assert_eq!(
        json!([seen.done("hold-wait", "done")["verdict"], seen.line["why"]]),
        json!(["red", "build"]),
        "{}",
        seen.raw
    );
    rig.git(&["checkout", "-q", "--", "version.txt"]);
    rig.write("fails-to-build", "\n");
    let broken = rig.commit("chore: a commit that does not build");
    held.start("build-c4", &build_kind(&broken, "bin/c4/jigc", None), "", 0)
        .done("hold-start", "started");
    let seen = held.wait("build-c4", Some(60));
    assert_eq!(
        json!([seen.done("hold-wait", "done")["verdict"], seen.line["why"]]),
        json!(["red", "build"]),
        "{}",
        seen.raw
    );
    for built in ["bin/c3/jigc", "bin/c4/jigc"] {
        assert!(
            !rig.scratch.join(built).exists(),
            "a build that is red leaves no binary: {built}"
        );
    }

    // The build itself, asked for directly, refuses a target that is there and a commit
    // that is none — and leaves what is there as it is.
    let direct = |work: &str, commit: &str, to: &str| {
        rig.step(&[
            "build",
            "--scratch",
            &held.scratch,
            "--commit",
            commit,
            "--work",
            work,
            "--to",
            to,
        ])
    };
    direct("work-1", &commit, "bin/c1/jigc").refused("taken");
    assert_eq!(fs::read(&binary).expect("the binary"), bytes);
    direct("work-2", &"0".repeat(40), "bin/c5/jigc").refused("git");
    direct("work-3", &broken, "bin/c5/jigc").refused("build");
}

/// **A held command writes only under the scratch root it resolved** (`SP-2`'s class: only
/// the root is resolved, so a link planted beneath it carries a write elsewhere). Every
/// path the tool writes — a job's directory, the file a binary lands at, a build's work —
/// is held to lying under the real root, and a link on the way is refused before anything
/// is started or written; so is a root inside the repository, and a file to hash that a
/// link leads out to.
#[test]
fn a_held_command_writes_only_under_the_scratch_root_it_resolved() {
    let held = Held::new("hold-links");
    let rig = &held.rig;
    rig.on_path("cargo", CARGO);
    rig.write("version.txt", "1.0.0-rc.24\n");
    let commit = rig.commit("chore: a version");
    held.released(true);
    let elsewhere = rig.dir().join("elsewhere");
    fs::create_dir(&elsewhere).expect("a directory that is not the scratch root");
    let link = |at: &Path| std::os::unix::fs::symlink(&elsewhere, at).expect("plant a link");
    let untouched = || {
        assert_eq!(
            fs::read_dir(&elsewhere).expect("elsewhere").count(),
            0,
            "nothing was written where the link leads"
        );
    };
    let green = as_recorded("gate-green.txt");

    // `<scratch>/hold` is a link.
    link(&rig.scratch.join("hold"));
    held.start("gate", &GATE_KIND, &green, 0).refused("usage");
    held.wait("gate", Some(0)).refused("usage");
    untouched();
    fs::remove_file(rig.scratch.join("hold")).expect("remove the link");
    // A job's directory is a link.
    fs::create_dir(rig.scratch.join("hold")).expect("the holds");
    link(&rig.scratch.join("hold/gate"));
    held.start("gate", &GATE_KIND, &green, 0).refused("usage");
    held.wait("gate", Some(0)).refused("usage");
    untouched();
    assert!(
        held.runs().is_empty(),
        "nothing was started: {:?}",
        held.runs()
    );
    fs::remove_file(rig.scratch.join("hold/gate")).expect("remove the link");
    // Where a binary is to land is behind a link — as a held build, and asked directly.
    link(&rig.scratch.join("bin"));
    held.start("build", &build_kind(&commit, "bin/c1/jigc", None), "", 0)
        .refused("usage");
    rig.step(&[
        "build",
        "--scratch",
        &held.scratch,
        "--commit",
        &commit,
        "--work",
        "work",
        "--to",
        "bin/c1/jigc",
    ])
    .refused("usage");
    link(&rig.scratch.join("work-linked"));
    rig.step(&[
        "build",
        "--scratch",
        &held.scratch,
        "--commit",
        &commit,
        "--work",
        "work-linked",
        "--to",
        "built/jigc",
    ])
    .refused("usage");
    untouched();
    // A file to hash that a link leads out to; one that is not there; a path that climbs.
    fs::write(elsewhere.join("secret"), "s\n").expect("a file elsewhere");
    let hash = |file: &str| rig.step(&["hash", "--scratch", &held.scratch, "--file", file]);
    hash("bin/secret").refused("usage");
    hash("no-such-file").refused("missing");
    hash("../elsewhere/secret").refused("usage");
    fs::remove_file(elsewhere.join("secret")).expect("remove it");

    // A scratch root that resolves INTO THE REPOSITORY — a link, since the rig's own root
    // is no plain path: refused, and the repository is as it was.
    let into = rig.dir().join("into-the-repository");
    std::os::unix::fs::symlink(rig.root.join("crates"), &into).expect("plant a link");
    let into = into.display().to_string();
    let before = rig.status();
    for args in [
        vec![
            "hold-start",
            "--scratch",
            &into,
            "--name",
            "gate",
            "--kind",
            "gate",
            "--run",
            RUN,
        ],
        vec!["hold-wait", "--scratch", &into, "--name", "gate"],
        vec!["hash", "--scratch", &into, "--file", "a.txt"],
        vec![
            "build",
            "--scratch",
            &into,
            "--commit",
            &commit,
            "--work",
            "w",
            "--to",
            "b/jigc",
        ],
    ] {
        rig.step(&args).refused("usage");
    }
    // — and so is the root a digest's files are kept under, before one of them is made.
    let asked = rig.step(&[
        "hash",
        "--digest",
        &into,
        "--scratch",
        &into,
        "--file",
        "a.txt",
    ]);
    assert_eq!(
        json!([asked.code, asked.line["refused"], asked.line["file"]]),
        json!([2, "usage", null]),
        "a digest's root inside the repository is refused, and no file is kept there: {}",
        asked.raw
    );
    assert_eq!(
        rig.status(),
        before,
        "nothing was written in the repository"
    );
    assert!(
        held.runs().is_empty(),
        "and nothing was started: {:?}",
        held.runs()
    );
}

/// **A file's hash is asked of the tool**, and no agent runs `shasum`: the file lies under
/// the scratch root, and the answer is its sha256 and its length.
#[test]
fn a_file_is_hashed_by_the_tool() {
    let rig = StepRig::new("hold-hash");
    let scratch = rig.scratch.display().to_string();
    let text = "{\"a\": \"\u{2014}\"}\n";
    fs::create_dir_all(rig.scratch.join("record/x")).expect("a directory of a record step");
    fs::write(rig.scratch.join("record/x/batch.json"), text).expect("write a batch");
    let seen = rig.step(&[
        "hash",
        "--scratch",
        &scratch,
        "--file",
        "record/x/batch.json",
    ]);
    assert_eq!(
        json!([
            seen.done("hash", "hashed")["path"],
            seen.line["bytes"],
            seen.line["content_sha256"]
        ]),
        json!(["record/x/batch.json", text.len(), sha256(text)]),
        "{}",
        seen.raw
    );
    assert!(seen.trace.is_empty(), "it asks git nothing");
    rig.step(&["hash", "--scratch", &scratch, "--file", "record/x"])
        .refused("missing");
}

/// **The tool's verdict file is what the record script records a scripted check from** —
/// the two scripts, end to end: a regression set held to its end, and its verdict file
/// handed to `result-set` for an item of kind `held-regression`. The result's word is the
/// file's; the file on record holds the two commits, the list's hash and what was excluded.
#[test]
fn the_tools_verdict_file_is_what_the_record_script_records() {
    let held = Held::new("hold-record");
    let rig = &held.rig;
    let (ran, line) = regression_line("run-3-result.json");
    let (previous, candidate) = (
        ran["previous"]["commit"].as_str().expect("a sha"),
        ran["candidate"]["commit"].as_str().expect("a sha"),
    );
    rig.wrote(
        &format!("run-set --run {RUN}"),
        &json!({"stop": "at-the-bound", "rounds": 3, "previous": "1.0.0-rc.24",
                "previous-commit": previous, "scope": "delta", "clauses": ["working-product"]})
        .to_string(),
    );
    rig.wrote(
        &format!("item-set --run {RUN}"),
        &json!({"item": "regression-set", "kind": "held-regression", "clause": "working-product",
                "runs": "every-candidate",
                "brief": "the regression set, as a held command of dev/stabilize-step"})
        .to_string(),
    );
    rig.wrote(
        &format!("scope-set --run {RUN} --round 1"),
        &json!({"included": [], "excluded": []}).to_string(),
    );
    let done = held.judged(
        "regression-r1-a1",
        &regression_kind(previous, candidate),
        &line,
        0,
    );
    assert_eq!(done["verdict"], "green", "{done}");
    let verdict = rig
        .scratch
        .join(done["verdict_file"].as_str().expect("the verdict's file"));
    let result = |row: Value| {
        rig.record_script(
            &format!(
                "{RECORD} result-set --run {RUN} --round 1 --commit {candidate} --scratch '{}'",
                held.scratch
            ),
            &row.to_string(),
        )
    };
    let (code, out, err) = result(json!({"item": "regression-set", "verdict": verdict}));
    assert_eq!(code, 0, "{err}");
    let recorded: Value = serde_json::from_str(out.trim()).expect("the result");
    assert_eq!(
        recorded["recorded"],
        json!([{"item": "regression-set", "attempt": 1, "outcome": "green"}]),
        "{recorded}"
    );
    let on_record: Value =
        serde_json::from_str(&rig.read(&format!("{RUN_DIR}/r1/checks/regression-set.a1.json")))
            .expect("the verdict on record");
    assert_eq!(
        json!([
            on_record["verdict"],
            on_record["evidence"]["line"]["previous"]["commit"],
            on_record["evidence"]["line"]["candidate"]["commit"],
            on_record["evidence"]["line"]["list"]["sha256"],
            on_record["evidence"]["line"]["excluded"]
                .as_array()
                .map(Vec::len)
        ]),
        json!(["green", previous, candidate, ran["list"]["sha256"], 11]),
        "the two commits, the list's hash and what was excluded are on record: {on_record}"
    );
}
