//! **`dev/stabilize-canary` — the setup of the small canary of the stabilization workflow's
//! `test` stage** ([DECISIONS.md](../DECISIONS.md) → *2026-10-07 — The small canary's setup*;
//! the second repair plan's task `K7`,
//! [plan.md](../completions/artifacts/M55/stabilization-build/re-review-test-half/plan.md)
//! → §6, *The small canary*).
//!
//! No `test` stage has run under the Workflow runtime with real agents. The small canary is
//! that run, once, where nothing of it can reach this repository or its remote: a clone
//! under a root outside the repository, whose one remote is a bare repository beside it, on
//! a loop branch that exists only there, over a **synthetic opening** written through the
//! record script's own writers. This suite is where the setup is executed, for real, on a
//! small source repository of its own:
//!
//! - what a setup leaves — the clone at the commit named, one remote and it the bare
//!   repository, the loop branch, the opening committed and pushed there by the step tool,
//!   and the state a fresh run has
//!   ([`a_setup_leaves_a_clone_whose_one_remote_is_the_bare_repository_beside_it_and_a_run_that_is_ready`]);
//! - the opening — one item of each kind, and one seeded finding nobody has graded
//!   ([`the_opening_holds_one_item_of_each_kind_and_one_seeded_finding_nobody_has_graded`]);
//! - **the printed invocation, run as printed**: the clone's own harness, by the path the
//!   line names, from the directory the line names, with the `args` object the line holds —
//!   through a whole `test` stage under the runtime's stand-in, to a record the bare remote
//!   holds. The seeded finding reaches triage and a verifier, and the stage launches as many
//!   agents as the line says
//!   ([`the_printed_invocation_runs_the_clones_own_harness_through_a_whole_test_stage`]);
//! - the canary's one plant, reachable: a verifier that dies leaves the seeded finding
//!   unverified, and the line's later arguments finish the triage
//!   ([`a_verifier_that_dies_leaves_the_seeded_finding_unverified_and_the_later_arguments_finish_the_triage`]);
//! - **the held checks of the default opening, started and read** — the gate and the
//!   regression set, each one long command the clone's own step tool holds: the stage
//!   starts each, a step asks after it, and the item is recorded from the tool's verdict
//!   file
//!   ([`a_stage_over_the_default_opening_starts_its_held_checks_and_records_the_tools_verdicts`]);
//! - a smaller set of items, the trial arm's stand-in among them
//!   ([`a_smaller_set_of_items_is_the_callers_to_name_and_runs_as_many_agents_as_the_line_says`]);
//! - what is refused
//!   ([`setup_refuses_a_root_it_may_not_use_and_a_run_or_a_commit_the_repository_cannot_give_it`]),
//!   and that **nothing is written inside the repository the tool is run from** — its whole
//!   directory, `.git` included, is byte for byte what it was
//!   ([`nothing_is_written_inside_the_repository_the_tool_is_run_from`]);
//! - `check`: a clone that could reach anything but its bare remote is refused, and what a
//!   stage left is read
//!   ([`check_refuses_a_clone_that_could_reach_anything_but_its_bare_remote`]);
//! - and every command the tool names is one plain invocation of a committed tool
//!   ([`every_command_the_tool_names_is_one_plain_invocation_of_a_committed_tool`]).
//!
//! **Why a small source repository and not this one.** The tool clones the repository it
//! lies in, so the suite places it in a [`StepRig`] — the two scripts of a run, the hygiene
//! scan and a stand-in for gitleaks, a bare remote — beside a copy of the committed harness.
//! This repository would do (a setup from it measures seconds), but a hosted runner's
//! checkout is shallow, its history is not the suite's to depend on, and a write of the
//! record script needs a denylist the machine may not have.
//!
//! **Proved red on applied mutants**, each applied alone against the arm that names its
//! decision — forty-six, in six groups: which root is taken (inside the repository by any
//! of its working trees, relative, of other than plain segments, holding something, below
//! nothing, refused by the file system); what the repository may hold of the run (in the
//! commit, in the tree, as a branch here or on a remote) and of the commit (no harness, a
//! run that is not fresh); how the clone is made (the previous release's commit, no second
//! remote, the vetted push, the machine's configuration); the opening (the seeded row
//! missing, graded or at another door, the review row's door, the default items, an
//! unknown item, a kind, a character outside ASCII); the line (each `args` object, the
//! harness's path, the session's directory, every term of the count, the estimate, the
//! one command); and `check` (each of its rules, the caller's configuration, a link, a
//! root with no canary). None survived.
//!
//! **What this suite cannot show** is what the canary exists to show: real agents. The
//! stage that runs here is the committed harness under the simulation's stand-in
//! (`tooling-tests/fixtures/stabilize-runtime.mjs`), whose agents are functions.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::support::child_stdin;

use super::dev_stabilize_step::{StepRig, node_or_skip};
use super::placed_executable;
use super::stabilize_simulation::{CARGO, LONG_COMMAND};

const TOOL: &str = "dev/stabilize-canary";
const RECORD: &str = "dev/stabilize-record";
const HARNESS: &str = ".claude/workflows/stabilize.js";
const RUNTIME: &str = "tooling-tests/fixtures/stabilize-runtime.mjs";

/// The canary's run in every test, the release it measures against, the one door of its
/// round, and the key of the finding its opening seeds.
const RUN: &str = "canary-small";
const LOOP: &str = "fix/canary-small";
const RUN_DIR: &str = "completions/artifacts/canary-small";
const PREVIOUS: &str = "1.0.0-rc.24";
const DOOR: &str = "jigc doc list";
const SEEDED: &str = "canary-seeded-claim";
const CLAUSE: &str = "no-lost-files";

/// The opening's items when none is named, by id and kind.
const DEFAULT_ITEMS: [(&str, &str); 5] = [
    ("row-doc-list", "review-row"),
    ("cross-cutting", "audit-cross-cutting"),
    ("arm-control", "trial-arm"),
    ("gate", "held-gate"),
    ("regression-set", "held-regression"),
];

/// The word a refusal opens with, and its exit status.
const REFUSALS: &[(&str, i32)] = &[
    ("usage", 2),
    ("inside", 3),
    ("no-root", 4),
    ("exists", 5),
    ("taken", 6),
    ("no-commit", 7),
    ("unfit", 8),
    ("tool", 9),
    ("not-isolated", 10),
    ("not-fresh", 11),
    ("missing", 12),
    ("link", 13),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn sha256(bytes: &[u8]) -> String {
    let mut child = Command::new("python3")
        .args([
            "-c",
            "import hashlib, sys; sys.stdout.write(hashlib.sha256(sys.stdin.buffer.read()).hexdigest())",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn python3");
    child_stdin::feed(&mut child, bytes);
    let out = child.wait_with_output().expect("python3 exits");
    assert!(out.status.success(), "python3 hashes the bytes");
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
    assert!(
        line.bytes().all(|b| (0x20..0x7f).contains(&b)),
        "the line is printable ASCII: {line}"
    );
    let marker = ", \"sha256\": \"";
    let at = line
        .rfind(marker)
        .unwrap_or_else(|| panic!("the line ends with its sha256: {line}"));
    let hash = line[at + marker.len()..]
        .strip_suffix("\"}")
        .unwrap_or_else(|| panic!("the sha256 is the line's last field: {line}"));
    assert_eq!(
        sha256(format!("{}}}", &line[..at]).as_bytes()),
        hash,
        "the line's sha256 is that of the line without it: {line}"
    );
    serde_json::from_str(line).unwrap_or_else(|e| panic!("the line is JSON ({e}): {line}"))
}

/// What a call of the tool left: its exit status, the ONE line it printed — held to the
/// hash it ends with before it is read — and its stderr.
struct Told {
    code: i32,
    raw: String,
    line: Value,
    stderr: String,
}

impl Told {
    /// The act ran to its end and says `status`.
    fn done(&self, act: &str, status: &str) -> &Value {
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
    fn refused(&self, word: &str) -> &Value {
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
        let opening = format!("stabilize-canary: refused {word}: ");
        assert!(
            self.stderr.starts_with(&opening) && self.stderr.trim_end().lines().count() == 1,
            "a refusal is one line on stderr that opens `{opening}`: {}",
            self.stderr
        );
        assert_eq!(self.line["status"], "halted", "the line: {}", self.raw);
        assert_eq!(self.line["refused"], word, "the line: {}", self.raw);
        for field in ["root_cause", "evidence", "tree_state", "recommendation"] {
            assert!(
                self.line["halt"][field].is_string(),
                "the halt report has `{field}`: {}",
                self.raw
            );
        }
        &self.line
    }
}

/// Every file below `dir`, `.git` included, by its path: its bytes, or — for a link —
/// where it points. Read with the file system alone: a `git status` would write the index.
fn files_below(dir: &Path, prefix: &str, found: &mut BTreeMap<String, Vec<u8>>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("list {}: {e}", dir.display()))
        .map(|entry| entry.expect("a directory entry"))
        .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        let kind = entry.file_type().expect("a file type");
        if kind.is_symlink() {
            let target = fs::read_link(entry.path()).expect("read a link");
            found.insert(
                format!("{name} ->"),
                target.to_string_lossy().into_owned().into_bytes(),
            );
        } else if kind.is_dir() {
            found.insert(format!("{name}/"), Vec::new());
            files_below(&entry.path(), &format!("{name}/"), found);
        } else {
            found.insert(name, fs::read(entry.path()).expect("read a file"));
        }
    }
}

/// The paths at which two snapshots differ.
fn differing(before: &BTreeMap<String, Vec<u8>>, after: &BTreeMap<String, Vec<u8>>) -> Vec<String> {
    let mut names: Vec<String> = before
        .keys()
        .chain(after.keys())
        .filter(|name| before.get(*name) != after.get(*name))
        .cloned()
        .collect();
    names.sort();
    names.dedup();
    names
}

/// A reporter that found nothing.
fn nothing() -> Value {
    json!({"status": "reported", "findings": [], "summary": "Nothing found."})
}

/// One invocation of a harness under the runtime's stand-in, as that stand-in reports it.
struct Ran {
    /// What the script returned to the orchestrator.
    result: Value,
    /// The label of every agent launched, in order.
    agents: Vec<String>,
}

/// The source repository the tool is run from — a [`StepRig`] that holds the tool and the
/// committed harness beside the scripts of a run — and the commits a setup is handed.
struct Canary {
    rig: StepRig,
    /// The commit a setup is told to clone: the source's head, which holds the tool.
    commit: String,
    /// The commit the "previous release" was released from: one on a branch of its own.
    previous: String,
}

impl Canary {
    fn new(label: &str) -> Self {
        let rig = StepRig::unopened(&format!("canary-{label}"));
        // The previous release was released from a branch of its own: its commit is no
        // ancestor of the candidate, so nothing brings it along unasked.
        rig.git(&["switch", "-q", "-c", "work/the-previous-release"]);
        let previous = rig.change("RELEASED.md", "a release\n", "chore: the previous release");
        rig.git(&["switch", "-q", "main"]);
        placed_executable::copy(&repo_root().join(TOOL), &rig.root.join(TOOL));
        // THE LONG COMMANDS A STAGE HOLDS, as the simulation's stand-ins: `dev/gate` and
        // `dev/regression-set` where the clone's step tool finds them, and cargo first on
        // the `PATH`. The step tool that starts, holds and judges each is the clone's own.
        for tool in ["dev/gate", "dev/regression-set"] {
            placed_executable::write(&rig.root.join(tool), LONG_COMMAND);
        }
        rig.on_path("cargo", CARGO);
        let held = rig.dir().join("held-commands");
        fs::create_dir_all(&held).expect("create the directory of what a held command prints");
        fs::write(held.join("released"), "").expect("release the held commands");
        fs::write(held.join("ran"), "").expect("nothing ran yet");
        let harness = fs::read(repo_root().join(HARNESS)).expect("read the committed harness");
        rig.write(
            HARNESS,
            std::str::from_utf8(&harness).expect("the harness is UTF-8"),
        );
        let commit = rig.commit("chore: the canary's setup tool and the harness");
        rig.git(&["push", "-q", "origin", "main"]);
        fs::create_dir_all(rig.dir().join("tmp")).expect("create the rig's temp directory");
        Canary {
            rig,
            commit,
            previous,
        }
    }

    /// The rig's directory as the file system resolves it: the tool prints real paths.
    fn real(&self) -> PathBuf {
        fs::canonicalize(self.rig.dir()).expect("resolve the rig's directory")
    }

    /// A canary root that is not there yet, outside the source repository.
    fn root(&self, name: &str) -> PathBuf {
        self.real().join(name)
    }

    /// A child of the rig: its environment, and a temp directory of the rig's own.
    fn command(&self, program: &Path) -> Command {
        let mut command = self.rig.hermetic(Command::new(program));
        command
            .env(
                "TMPDIR",
                format!("{}/", self.rig.dir().join("tmp").display()),
            )
            .env("SIM_HELD", self.rig.dir().join("held-commands"));
        command
    }

    /// The long commands that ran, as their stand-ins wrote them down, without the pid.
    fn long_commands(&self) -> Vec<String> {
        fs::read_to_string(self.rig.dir().join("held-commands/ran"))
            .expect("what ran")
            .lines()
            .map(|line| {
                line.split_once(' ')
                    .expect("a pid and a command")
                    .1
                    .to_owned()
            })
            .collect()
    }

    fn told(&self, mut command: Command) -> Told {
        let out = command.stdin(Stdio::null()).output().expect("run the tool");
        let raw = String::from_utf8_lossy(&out.stdout).into_owned();
        Told {
            code: out.status.code().expect("the tool exits"),
            line: line_of(&raw),
            raw,
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }

    /// The tool, called by its path from a directory that is not the repository: it finds
    /// its repository from where it lies.
    fn tool(&self, args: &[&str]) -> Told {
        self.tool_in(&[], args)
    }

    /// The tool, with `env` over the rig's own environment.
    fn tool_in(&self, env: &[(&str, &str)], args: &[&str]) -> Told {
        let mut command = self.command(&self.rig.root.join(TOOL));
        command
            .args(args)
            .envs(env.iter().copied())
            .current_dir(self.rig.dir());
        self.told(command)
    }

    /// The arguments of a setup under `root`, for the run `run`, with `more`.
    fn setup_args<'a>(&'a self, root: &'a str, run: &'a str, more: &[&'a str]) -> Vec<&'a str> {
        let mut args = vec![
            "setup",
            "--root",
            root,
            "--commit",
            self.commit.as_str(),
            "--run",
            run,
            "--previous",
            PREVIOUS,
            "--previous-commit",
            self.previous.as_str(),
        ];
        args.extend_from_slice(more);
        args
    }

    /// A setup of the canary's run under `root`, with `more`.
    fn setup(&self, root: &Path, more: &[&str]) -> Told {
        let root = root.display().to_string();
        self.tool(&self.setup_args(&root, RUN, more))
    }

    /// A setup that must succeed: its line.
    fn ready(&self, name: &str, more: &[&str]) -> Value {
        self.setup(&self.root(name), more)
            .done("setup", "ready")
            .clone()
    }

    /// A git command of the suite's in `at`: whether it succeeded, and what it printed.
    fn git_at(&self, at: &Path, args: &[&str]) -> (bool, String) {
        let out = self
            .command(Path::new("git"))
            .args(args)
            .current_dir(at)
            .output()
            .expect("run git");
        (
            out.status.success(),
            String::from_utf8_lossy(&out.stdout)
                .trim_end_matches('\n')
                .to_owned(),
        )
    }

    /// A git command of the suite's in `at` that must succeed: what it printed.
    fn git(&self, at: &Path, args: &[&str]) -> String {
        let (ok, out) = self.git_at(at, args);
        assert!(ok, "`git {}` in {}", args.join(" "), at.display());
        out
    }

    /// The run's state, as the clone's own record script reads it.
    fn state(&self, clone: &Path) -> Value {
        let out = self
            .command(&clone.join(RECORD))
            .args(["state", "--run", RUN])
            .current_dir(self.rig.dir())
            .stdin(Stdio::null())
            .output()
            .expect("run the clone's record script");
        assert!(
            out.status.success(),
            "the clone's `{RECORD} state`: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        serde_json::from_slice(&out.stdout).expect("the state is one JSON line")
    }

    /// Everything below the source repository, `.git` included.
    fn snapshot(&self) -> BTreeMap<String, Vec<u8>> {
        let mut found = BTreeMap::new();
        files_below(&self.rig.root, "", &mut found);
        found
    }

    /// ONE INVOCATION, AS THE LINE NAMES IT: the harness at the path `line` gives, run by
    /// the runtime's stand-in from the directory `line` gives, with `args` — an `args`
    /// object of the line, as printed. `script` is what the scripted agents say.
    fn invoke(&self, line: &Value, args: &Value, mut script: Value, n: u32) -> Ran {
        script["args"] = args.clone();
        let file = self.rig.dir().join(format!("scenario-{n}.json"));
        fs::write(&file, script.to_string()).expect("write the scenario");
        let out = self
            .command(Path::new("node"))
            .arg(repo_root().join(RUNTIME))
            .arg(line["script_path"].as_str().expect("the harness's path"))
            .arg(&file)
            .current_dir(
                line["session_cwd"]
                    .as_str()
                    .expect("the session's directory"),
            )
            .stdin(Stdio::null())
            .output()
            .expect("run node");
        let said: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
            panic!(
                "the runtime's stand-in prints one JSON line ({e}): {}\n{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            )
        });
        let list = |name: &str| -> Vec<String> {
            said[name]
                .as_array()
                .unwrap_or_else(|| panic!("the stand-in reports `{name}`"))
                .iter()
                .map(|line| line.as_str().expect("a line of text").to_owned())
                .collect()
        };
        let trace = list("trace");
        assert!(
            out.status.success() && list("fatal").is_empty(),
            "invocation {n} is not one the stand-in stands behind: {:?}\n{}\nthe trace so far:\n{}",
            list("fatal"),
            String::from_utf8_lossy(&out.stderr),
            trace.join("\n")
        );
        assert_eq!(
            [list("swallowed"), list("unused")],
            [Vec::<String>::new(), Vec::new()],
            "invocation {n}: nothing was swallowed, and every scripted agent was launched"
        );
        Ran {
            result: said["result"].clone(),
            agents: trace
                .iter()
                .filter_map(|line| line.trim_start().strip_prefix("agent "))
                .map(|line| line.split(" · ").next().expect("a label").to_owned())
                .collect(),
        }
    }
}

/// What the scripted agents of a whole stage say over the items of `line`: every check
/// green, the one door inside, every instrument finding nothing, the seeded row graded
/// `unclear` by triage — found again, under its own key — and the verifier as `verifier`
/// has it.
fn stage_script(line: &Value, verifier: Value) -> Value {
    let mut checks = json!({});
    let mut agents = json!({});
    for item in line["items"].as_array().expect("the line's items") {
        let id = item["item"].as_str().expect("an item's id");
        let steps: &[&str] = match item["kind"].as_str().expect("an item's kind") {
            "check" => {
                checks[id] = json!("green");
                &[]
            }
            // A held check has no agent of its own, and no agent's word: the tool starts
            // it, holds it and judges it, and the stand-ins for the long commands print
            // green.
            "held-gate" | "held-regression" => &[],
            "review-row" => &["source", "driver", "reconciler"],
            "trial-arm" => &["rehearse", "run", "score"],
            "audit-cross-cutting" => &["review"],
            "audit-drive" => &["drive"],
            other => panic!("the suite scripts no agent for an item of kind `{other}`"),
        };
        for step in steps {
            agents[format!("{id}:{step}")] = nothing();
        }
    }
    agents["triage:p1"] = json!({
        "status": "graded",
        "entries": [{"key": SEEDED, "new": false, "doctype": "jigc-feedback", "source": "ledger",
                     "door": DOOR, "clause": CLAUSE, "repro": "the opening record's block",
                     "grade": "unclear"}],
        "to_verify": [],
        "counts": {"findings_in": [{"reporter": "ledger", "count": 1}], "entries": 1, "merged": 0},
    });
    agents[format!("verify:{SEEDED}")] = verifier;
    json!({
        "checks": checks,
        "scope": {"included": [{"door": DOOR, "registry": "verbs", "derivation": "named by the invocation"}],
                  "excluded": [], "uncovered": [], "reached_but_excluded": [], "left_open": []},
        "agents": agents,
    })
}

/// A verifier that drove the seeded claim and refutes it.
fn refuted() -> Value {
    json!({"status": "verified", "key": SEEDED, "verdict": "refuted",
           "basis": "does-not-reproduce: the file stands", "repro": "the block, driven",
           "pinnable": true, "left_open": []})
}

#[test]
fn a_setup_leaves_a_clone_whose_one_remote_is_the_bare_repository_beside_it_and_a_run_that_is_ready()
 {
    let canary = Canary::new("leaves");
    let root = canary.root("canary");
    // The rig's `git` writes down every call the STEP TOOL makes, and no other.
    let trace = canary.rig.dir().join("the-step-tools-git-calls");
    let line = canary
        .tool_in(
            &[("STEP_TRACE", &trace.display().to_string())],
            &canary.setup_args(&root.display().to_string(), RUN, &[]),
        )
        .done("setup", "ready")
        .clone();
    let at = |name: &str| root.join(name).display().to_string();

    // (1) THE LINE: where everything is, by real and absolute paths under the root.
    assert_eq!(
        json!([
            line["root"],
            line["clone"],
            line["origin"],
            line["scratch"],
            line["results"]
        ]),
        json!([
            root.display().to_string(),
            at("clone"),
            at("origin.git"),
            at("scratch"),
            at("results")
        ]),
        "{line}"
    );
    assert_eq!(
        json!([
            line["run"],
            line["loop"],
            line["run_dir"],
            line["candidate"],
            line["door"],
            line["seeded"]
        ]),
        json!([RUN, LOOP, RUN_DIR, canary.commit, DOOR, SEEDED]),
        "{line}"
    );
    assert_eq!(
        line["previous"],
        json!({"version": PREVIOUS, "commit": canary.previous}),
        "{line}"
    );
    let clone = root.join("clone");
    let origin = root.join("origin.git");
    for made in ["clone", "origin.git", "scratch", "results"] {
        assert!(
            root.join(made).is_dir(),
            "`{made}` is a directory of the root"
        );
    }

    // (2) THE CLONE: at the commit named, on the loop branch, with ONE commit on top — the
    // opening — and a clean tree.
    let head = canary.git(&clone, &["rev-parse", "HEAD"]);
    assert_eq!(line["head"], json!(head), "{line}");
    assert_eq!(
        canary.git(&clone, &["rev-parse", "HEAD^"]),
        canary.commit,
        "the opening is one commit on the commit named"
    );
    assert_eq!(canary.git(&clone, &["branch", "--show-current"]), LOOP);
    assert_eq!(
        canary.git(
            &clone,
            &["for-each-ref", "--format=%(refname)", "refs/heads"]
        ),
        format!("refs/heads/{LOOP}"),
        "the loop branch is the clone's one branch"
    );
    assert_eq!(
        canary.git(&clone, &["status", "--porcelain", "--untracked-files=all"]),
        "",
        "the clone's tree is clean"
    );
    assert_eq!(
        canary.git(
            &clone,
            &[
                "diff",
                "--name-only",
                "HEAD^",
                "HEAD",
                "--",
                ".",
                &format!(":(exclude){RUN_DIR}")
            ]
        ),
        "",
        "the opening changes nothing outside the run's directory"
    );
    assert!(
        canary
            .git_at(
                &clone,
                &["cat-file", "-e", &format!("{}^{{commit}}", canary.previous)]
            )
            .0
            && !canary
                .git_at(
                    &clone,
                    &["merge-base", "--is-ancestor", &canary.previous, "HEAD"]
                )
                .0,
        "the clone holds the previous release's commit, which no history of the candidate brings"
    );
    assert_eq!(
        fs::read(clone.join(HARNESS)).expect("the clone's harness"),
        fs::read(repo_root().join(HARNESS)).expect("the committed harness"),
        "the clone holds the harness of the commit named"
    );

    // (3) ONE REMOTE, AND IT IS THE BARE REPOSITORY BESIDE THE CLONE — asserted on git's
    // own answers, and on the clone's configuration as bytes: neither the source's remote
    // nor the source itself is named in it.
    assert_eq!(canary.git(&clone, &["remote"]), "origin");
    for push in [
        &["remote", "get-url", "--all", "origin"][..],
        &["remote", "get-url", "--push", "--all", "origin"],
    ] {
        assert_eq!(canary.git(&clone, push), origin.display().to_string());
    }
    assert_eq!(
        canary.git(&origin, &["rev-parse", "--is-bare-repository"]),
        "true"
    );
    let config = fs::read_to_string(clone.join(".git/config")).expect("the clone's config");
    for elsewhere in [
        canary.rig.origin.display().to_string(),
        fs::canonicalize(&canary.rig.origin)
            .expect("the source's remote")
            .display()
            .to_string(),
        canary.rig.root.display().to_string(),
        fs::canonicalize(&canary.rig.root)
            .expect("the source")
            .display()
            .to_string(),
    ] {
        assert!(
            !config.contains(&elsewhere),
            "the clone's configuration names `{elsewhere}`:\n{config}"
        );
    }

    // (4) A PUSH TO `origin` THERE LANDS IN THAT BARE REPOSITORY: the opening is in it, and
    // one more commit pushed by name arrives there and nowhere else.
    assert_eq!(
        canary.git(&origin, &["rev-parse", &format!("refs/heads/{LOOP}")]),
        head,
        "the bare repository holds the opening"
    );
    assert_eq!(line["remote_head"], json!(head), "{line}");
    // AND THE OPENING WAS PUSHED BY THE STEP TOOL — the one push of a run that vets what it
    // publishes — once: the push before it, of the commit named, is no call of that tool.
    let pushes: Vec<String> = fs::read_to_string(&trace)
        .expect("read the step tool's git calls")
        .lines()
        .filter(|call| call.starts_with("push "))
        .map(str::to_owned)
        .collect();
    assert_eq!(
        pushes,
        [format!("push origin {LOOP}")],
        "the step tool's pushes"
    );
    let source_before = canary.snapshot();
    fs::write(clone.join(RUN_DIR).join("a-note.md"), "a note\n").expect("write a note");
    canary.git(&clone, &["add", "-A"]);
    canary.git(
        &clone,
        &[
            "commit",
            "-q",
            "-m",
            "docs(record): a commit pushed by name",
        ],
    );
    canary.git(&clone, &["push", "-q", "origin", LOOP]);
    assert_eq!(
        canary.git(&origin, &["rev-parse", &format!("refs/heads/{LOOP}")]),
        canary.git(&clone, &["rev-parse", "HEAD"]),
        "a push to `origin` lands in the bare repository beside the clone"
    );
    assert_eq!(
        differing(&source_before, &canary.snapshot()),
        Vec::<String>::new(),
        "and nothing of it reaches the source repository, nor its remote"
    );
    assert_eq!(
        canary.rig.remote(LOOP),
        None,
        "the source's remote holds no such branch"
    );
    // Put back, for the reads below: the state is of the opening.
    canary.git(&clone, &["reset", "-q", "--hard", &head]);

    // (5) THE LOOP BRANCH IS ONE THE BRANCH RULE ACCEPTS, and the source holds neither it
    // nor the run.
    let rule = Command::new(repo_root().join("dev/branch-name"))
        .arg(LOOP)
        .output()
        .expect("run dev/branch-name");
    assert!(rule.status.success(), "`dev/branch-name {LOOP}`");
    assert!(!canary.rig.git_ok(&[
        "rev-parse",
        "--verify",
        "--quiet",
        &format!("refs/heads/{LOOP}")
    ]));
    assert!(!canary.rig.root.join(RUN_DIR).exists());

    // (6) THE STATE A FRESH RUN HAS, as the clone's own record script reads it — and as the
    // line says it.
    let state = canary.state(&clone);
    assert_eq!(
        json!([
            state["opened"],
            state["not_ready"],
            state["next"],
            state["position"]["test"]
        ]),
        json!([true, [], "test", {"round": 1, "attempt": 1}]),
        "{state}"
    );
    assert_eq!(
        line["state"],
        json!({"next": "test", "round": 1, "attempt": 1}),
        "{line}"
    );
    assert_eq!(
        json!([
            state["facts"]["stop"],
            state["facts"]["previous"],
            state["facts"]["previous-commit"],
            state["facts"]["scope"]
        ]),
        json!(["every-round", PREVIOUS, canary.previous, "delta"]),
        "{state}"
    );

    // (7) WHAT THE ORCHESTRATOR INVOKES, with nothing to interpret: the directory the
    // session stands in, the harness by its path IN THE CLONE, and the two `args` objects.
    assert_eq!(line["session_cwd"], json!(at("clone")), "{line}");
    assert_eq!(
        line["script_path"],
        json!(clone.join(HARNESS).display().to_string()),
        "{line}"
    );
    assert_eq!(
        line["args"],
        json!({"stage": "test", "run": RUN, "scratch": at("scratch"), "scope": {"doors": [DOOR]}}),
        "{line}"
    );
    assert_eq!(
        line["args_then"],
        json!({"stage": "test", "run": RUN, "scratch": at("scratch")}),
        "{line}"
    );
    assert_eq!(
        line["returns"],
        json!({"first": at("results/first.json"), "then": at("results/then.json")}),
        "{line}"
    );

    // (8) THE SAME ROOT AGAIN IS REFUSED, and the canary is what it was.
    let before = canary.git(&clone, &["rev-parse", "HEAD"]);
    canary.setup(&root, &[]).refused("exists");
    assert_eq!(canary.git(&clone, &["rev-parse", "HEAD"]), before);
    assert_eq!(canary.state(&clone)["next"], "test");
}

#[test]
fn the_opening_holds_one_item_of_each_kind_and_one_seeded_finding_nobody_has_graded() {
    let canary = Canary::new("opening");
    let line = canary.ready("canary", &[]);
    let clone = canary.root("canary").join("clone");
    let state = canary.state(&clone);

    // The items, by id and kind: the line's, and the test set's.
    let listed = |items: &Value| -> Vec<(String, String)> {
        items
            .as_array()
            .expect("items")
            .iter()
            .map(|i| {
                (
                    i["item"].as_str().expect("an id").to_owned(),
                    i["kind"].as_str().expect("a kind").to_owned(),
                )
            })
            .collect()
    };
    let expected: Vec<(String, String)> = DEFAULT_ITEMS
        .iter()
        .map(|(item, kind)| ((*item).to_owned(), (*kind).to_owned()))
        .collect();
    assert_eq!(listed(&line["items"]), expected, "{line}");
    assert_eq!(listed(&state["items"]), expected, "{state}");

    // ONE OF EACH KIND a round's `test` runs: a review row, the cross-cutting pass, a trial
    // arm, and the scripted checks — the gate and the regression set, each a held command.
    let kinds: Vec<&str> = expected.iter().map(|(_, kind)| kind.as_str()).collect();
    for kind in [
        "review-row",
        "audit-cross-cutting",
        "trial-arm",
        "held-gate",
        "held-regression",
    ] {
        assert!(kinds.contains(&kind), "an item of kind `{kind}`: {kinds:?}");
    }
    let item = |id: &str| -> &Value {
        state["items"]
            .as_array()
            .expect("items")
            .iter()
            .find(|i| i["item"] == id)
            .unwrap_or_else(|| panic!("the item `{id}`"))
    };
    // The review row is reached by the round's one door, and by nothing else; every other
    // item runs on every candidate.
    assert_eq!(
        json!([
            item("row-doc-list")["runs"],
            item("row-doc-list")["doors"],
            item("row-doc-list")["registries"]
        ]),
        json!(["in-scope", [DOOR], []]),
        "{state}"
    );
    for (id, _) in &DEFAULT_ITEMS[1..] {
        assert_eq!(item(id)["runs"], "every-candidate", "`{id}`: {state}");
    }
    // THE LONG SCRIPTED CHECK IS A HELD CHECK: its kind names the command the step tool
    // holds, and its brief says so and is no command line for an agent to run.
    assert!(
        item("regression-set")["brief"]
            .as_str()
            .is_some_and(|brief| brief.contains("`hold-start --kind regression`")
                && brief.contains(&canary.previous)
                && !brief.contains("dev/regression-set run")
                && !brief.contains("--scratch")),
        "the held check's brief names the held command and the previous release's commit, and spells no command of the regression tool: {state}"
    );

    // THE CENSUS: four clauses, and every item judges one of them. The fourth has no
    // instrument, as the exit rule's has none.
    assert_eq!(
        state["facts"]["clauses"],
        json!([
            "no-lost-files",
            "working-product",
            "usable-by-agents",
            "migration-works"
        ]),
        "{state}"
    );
    let judged: Vec<&str> = state["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|i| i["clause"].as_str().expect("a clause"))
        .collect();
    assert_eq!(
        judged,
        [
            "no-lost-files",
            "no-lost-files",
            "usable-by-agents",
            "working-product",
            "working-product"
        ],
        "{state}"
    );

    // ONE SEEDED FINDING, AND NOBODY HAS GRADED IT: a row of round 0 at the round's one
    // door, which the first triage that runs is handed.
    let ledger = state["ledger"].as_array().expect("the ledger");
    assert_eq!(ledger.len(), 1, "{state}");
    assert_eq!(
        json!([
            ledger[0]["key"],
            ledger[0]["round"],
            ledger[0]["door"],
            ledger[0]["clause"],
            ledger[0]["grade"],
            ledger[0]["disposition"],
            ledger[0]["route"]
        ]),
        json!([SEEDED, 0, DOOR, CLAUSE, "ungraded", "open", "triage"]),
        "{state}"
    );
    assert_eq!(
        state["untriaged"],
        json!([{"key": SEEDED, "why": "ungraded"}]),
        "{state}"
    );
    assert_eq!(state["bounds"], json!([]), "no bound is declared: {state}");

    // Its repro names the opening record, which holds the block — and the closing
    // condition by reference, which triage reads.
    let opening =
        fs::read_to_string(clone.join(RUN_DIR).join("opening.md")).expect("the opening record");
    assert!(
        ledger[0]["repro"]
            .as_str()
            .is_some_and(|repro| repro.contains(&format!("{RUN_DIR}/opening.md"))),
        "{state}"
    );
    for said in [
        "The exit rule, revised",
        "notes.md",
        "doc list",
        SEEDED,
        "SYNTHETIC",
    ] {
        assert!(
            opening.contains(said),
            "the opening record says `{said}`:\n{opening}"
        );
    }

    // WHAT THE OPENING ADDS TO THE STATE IS PRINTABLE ASCII: a line that carries it is
    // relayed by an agent, and a character outside ASCII does not survive that.
    let texts = [
        opening,
        state["items"].to_string(),
        state["ledger"].to_string(),
        state["facts"].to_string(),
    ];
    for text in &texts {
        assert!(
            text.bytes()
                .all(|b| b == b'\n' || (0x20..0x7f).contains(&b))
                && !text.contains("\\u"),
            "what the opening wrote is printable ASCII: {text}"
        );
    }
}

#[test]
fn the_printed_invocation_runs_the_clones_own_harness_through_a_whole_test_stage() {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let canary = Canary::new("stage");
    let line = canary.ready("canary", &[]);
    let root = canary.root("canary");
    let (clone, origin) = (root.join("clone"), root.join("origin.git"));
    let source_before = canary.snapshot();

    // THE FIRST INVOCATION, AS PRINTED: `args`, the clone's harness, from the clone.
    let ran = canary.invoke(&line, &line["args"], stage_script(&line, refuted()), 1);
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(
        json!([
            result["run"],
            result["round"],
            result["counts"]["items"],
            result["counts"]["items_run"],
            result["counts"]["voided"]
        ]),
        json!([RUN, 1, 5, 5, []]),
        "every item of the DEFAULT opening ran, the two held checks among them, and none is void: {result}"
    );

    // THE SEEDED FINDING REACHED TRIAGE AND A VERIFIER, and nothing masked it: it is the
    // one finding the stage was handed, and its verdict is on record.
    assert!(
        ran.agents.contains(&"triage:p1".to_owned())
            && ran.agents.contains(&format!("verify:{SEEDED}")),
        "{:?}",
        ran.agents
    );
    assert_eq!(
        json!([
            result["counts"]["findings_in"],
            result["counts"]["entries"],
            result["unverified"]
        ]),
        json!([1, 1, []]),
        "{result}"
    );
    let state = canary.state(&clone);
    assert_eq!(
        json!([
            state["ledger"][0]["grade"],
            state["ledger"][0]["triage"]["door"],
            state["ledger"][0]["route"]
        ]),
        json!(["refuted", "included", "recorded"]),
        "{state}"
    );

    // AS MANY AGENTS AS THE LINE SAYS, and of them as many one-command steps of the tool.
    assert_eq!(
        json!([line["expect"]["agents"], line["expect"]["tool_steps"]]),
        json!([
            ran.agents.len(),
            ran.agents
                .iter()
                .filter(|label| label.starts_with("git:"))
                .count()
        ]),
        "the line's count is the stage's: {:?}",
        ran.agents
    );
    assert_eq!(
        json!([line["expect"]["agents"], line["expect"]["tool_steps"]]),
        json!([32, 19]),
        "{line}"
    );

    // THE RECORD IS ONE COMMIT ON THE OPENING, AND THE BARE REMOTE HOLDS IT.
    let head = canary.git(&clone, &["rev-parse", "HEAD"]);
    assert_eq!(result["record"], json!(head), "{result}");
    assert_eq!(
        canary.git(&clone, &["rev-parse", "HEAD^"]),
        line["head"].as_str().expect("a sha")
    );
    assert_eq!(
        canary.git(&origin, &["rev-parse", &format!("refs/heads/{LOOP}")]),
        head,
        "the stage's push landed in the bare repository"
    );
    assert_eq!(
        canary.git(&clone, &["status", "--porcelain", "--untracked-files=all"]),
        "",
        "the stage left a clean tree"
    );

    // THE RUN STOPS AFTER ITS ROUND, and the later arguments — as printed — are answered
    // from that state by an invocation that has nothing to do.
    assert_eq!(result["next"], "stop", "{result}");
    let then = canary.invoke(&line, &line["args_then"], json!({}), 2);
    assert_eq!(
        json!([
            then.result["status"],
            then.result["refused"]["refused"],
            then.result["next"]
        ]),
        json!(["refused", "stopped", "stop"]),
        "{}",
        then.result
    );
    assert_eq!(
        then.agents,
        ["git:state", "git:state:test-1"],
        "two reads, and nothing else"
    );

    // `check` READS WHAT THE STAGE LEFT.
    let checked = canary.tool(&["check", "--root", &root.display().to_string()]);
    let read = checked.done("check", "checked");
    assert_eq!(
        json!([
            read["head"],
            read["remote_head"],
            read["pushed"],
            read["records"],
            read["state"]["next"]
        ]),
        json!([head, head, true, 1, "stop"]),
        "{read}"
    );
    assert_eq!(
        read["ledger"],
        json!([{"key": SEEDED, "grade": "refuted", "disposition": "open", "route": "recorded"}]),
        "{read}"
    );

    // AND NOTHING OF THE STAGE REACHED THE SOURCE REPOSITORY.
    assert_eq!(
        differing(&source_before, &canary.snapshot()),
        Vec::<String>::new(),
        "the source repository is what it was"
    );
}

#[test]
fn a_verifier_that_dies_leaves_the_seeded_finding_unverified_and_the_later_arguments_finish_the_triage()
 {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let canary = Canary::new("plant");
    let line = canary.ready("canary", &[]);
    let clone = canary.root("canary").join("clone");

    // THE CANARY'S ONE PLANT: the seeded row's verifier is stopped from outside, each time
    // it is tried. The stage records all the same, and the finding is left unverified.
    let mut script = stage_script(&line, refuted());
    script["endings"] = json!({format!("verify:{SEEDED}"): "dies"});
    let ran = canary.invoke(&line, &line["args"], script, 1);
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(result["next"], "triage", "{result}");
    assert_eq!(result["unverified"][0]["key"], SEEDED, "{result}");
    let state = canary.state(&clone);
    assert_eq!(
        json!([
            state["ledger"][0]["grade"],
            state["untriaged"],
            state["position"]["test"]["triage"]
        ]),
        json!(["unclear", [{"key": SEEDED, "why": "unverified"}], true]),
        "{state}"
    );

    // THE FIRST ARGUMENTS AGAIN ARE REFUSED: a round's scope is written once, and the
    // invocation that finishes a triage takes none.
    let again = canary.invoke(&line, &line["args"], json!({}), 2);
    assert_eq!(again.result["status"], "refused", "{}", again.result);
    assert_eq!(again.agents, ["git:state", "git:state:test-1"]);

    // THE LATER ARGUMENTS FINISH THE TRIAGE — on the commit the round tested, which the
    // record's commit has moved the branch past — and run no instrument.
    let lap = canary.invoke(
        &line,
        &line["args_then"],
        json!({"agents": {
            "triage:p1": {
                "status": "graded",
                "entries": [{"key": SEEDED, "new": false, "doctype": "jigc-feedback", "source": "ledger",
                             "door": DOOR, "clause": CLAUSE, "repro": "the opening record's block",
                             "grade": "unclear"}],
                "to_verify": [],
                "counts": {"findings_in": [{"reporter": "ledger", "count": 1}], "entries": 1, "merged": 0},
            },
            format!("verify:{SEEDED}"): refuted(),
        }}),
        3,
    );
    assert_eq!(
        json!([
            lap.result["status"],
            lap.result["triage_only"],
            lap.result["next"]
        ]),
        json!(["triaged", true, "stop"]),
        "{}",
        lap.result
    );
    assert!(
        !lap.agents.iter().any(|label| label.contains(':')
            && DEFAULT_ITEMS
                .iter()
                .any(|(item, _)| label.starts_with(&format!("{item}:")))),
        "the lap runs no instrument: {:?}",
        lap.agents
    );
    assert_eq!(canary.state(&clone)["ledger"][0]["grade"], "refuted");
}

/// **A stage over the DEFAULT opening starts its held checks and reads them** (the second
/// repair plan's `K11`; until it this arm pinned the halt of a harness that read a held
/// check's kind and started no command). The opening's `regression-set` is
/// `held-regression` and its `gate` is `held-gate`. The printed invocation holds the
/// candidate's gate once — two plain calls of the clone's own step tool, a start and a
/// step that asks after it — answers `gate` from it, starts the regression set AFTER it,
/// over the previous release's commit — which is on no branch of the clone, and which the
/// tool also builds — and the candidate's, and records both items from the files the tool
/// kept its verdicts in: green here, where the stand-ins for the long commands say so.
/// No preflight is asked anything about either.
#[test]
fn a_stage_over_the_default_opening_starts_its_held_checks_and_records_the_tools_verdicts() {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let canary = Canary::new("held");
    let line = canary.ready("canary", &[]);
    let root = canary.root("canary");
    let clone = root.join("clone");
    assert_eq!(
        json!([
            line["expect"]["agents"],
            line["expect"]["tool_steps"],
            line["expect"]["wall_minutes"]
        ]),
        json!([32, 19, {"from": 120, "to": 150}]),
        "the line's count and the plan's estimate for the whole set — a held command is two steps: {line}"
    );
    let ran = canary.invoke(&line, &line["args"], stage_script(&line, refuted()), 1);
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(
        result["halted"],
        Value::Null,
        "the stage does not halt at a held check: {result}"
    );

    // THE STAGE STARTS THE HELD COMMANDS, in this order: the two builds, the candidate's
    // gate, the regression set; and, at its record, the record's own gate.
    let previous = &canary.previous[..12];
    let held: Vec<&str> = ran
        .agents
        .iter()
        .filter_map(|label| label.strip_prefix("git:hold-start:"))
        .collect();
    assert_eq!(
        held,
        [
            "build-c1-a1",
            format!("build-previous-{previous}").as_str(),
            "gate-c1-a1",
            "regression-set-c1-a1",
            "record-test-r1-a1-1"
        ],
        "{:?}",
        ran.agents
    );
    for name in &held {
        assert!(
            ran.agents.contains(&format!("git:hold-wait:{name}")),
            "`{name}` is asked after by a step of its own: {:?}",
            ran.agents
        );
    }
    // The gate ran twice — the candidate's, and the record's — and the regression set once,
    // over the previous release's commit and the candidate's.
    let long = canary.long_commands();
    assert_eq!(
        long.iter().filter(|c| *c == "gate --keep-going").count(),
        2,
        "{long:?}"
    );
    let regression: Vec<&String> = long
        .iter()
        .filter(|c| c.starts_with("regression-set run "))
        .collect();
    assert_eq!(regression.len(), 1, "{long:?}");
    assert!(
        regression[0].contains(&format!("--previous {} --candidate {} ", canary.previous, line["head"].as_str().expect("the candidate")))
            && regression[0].contains("--list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv"),
        "{regression:?}"
    );

    // WHAT IS ON RECORD is the tool's verdict of each, from its file — and no agent's word.
    assert_eq!(
        result["held"]
            .as_array()
            .expect("what became of each held check")
            .iter()
            .map(|h| json!([h["item"], h["ends"]]))
            .collect::<Vec<_>>(),
        [json!(["gate", "green"]), json!(["regression-set", "green"])],
        "{result}"
    );
    let state = canary.state(&clone);
    for id in ["gate", "regression-set"] {
        let kept: Value = serde_json::from_str(
            fs::read_to_string(clone.join(format!("{RUN_DIR}/r1/checks/{id}.a1.json")))
                .unwrap_or_else(|e| panic!("the verdict of `{id}` on record: {e}"))
                .trim_end(),
        )
        .expect("a verdict on record is one JSON line");
        assert_eq!(
            json!([kept["item"], kept["verdict"]]),
            json!([id, "green"]),
            "{kept}"
        );
    }
    assert_eq!(
        state["clauses"]
            .as_array()
            .expect("the clauses")
            .iter()
            .find(|c| c["clause"] == "working-product")
            .map(|c| c["status"].clone()),
        Some(json!("green")),
        "{state}"
    );
    assert_eq!(result["next"], "stop", "{result}");
}

#[test]
fn a_smaller_set_of_items_is_the_callers_to_name_and_runs_as_many_agents_as_the_line_says() {
    let canary = Canary::new("items");

    // An item the tool has no row for is refused, and so is one named twice, and none.
    for bad in ["row-doc-list,nobody", "gate,gate", ""] {
        let root = canary.root("refused").display().to_string();
        canary
            .tool(&canary.setup_args(&root, RUN, &["--items", bad]))
            .refused("usage");
        assert!(
            !canary.root("refused").exists(),
            "nothing was made for `{bad}`"
        );
    }

    // The review row, the gate, and the trial arm's stand-in: a drive of the one door.
    // WHAT A SETUP MAKES IS MADE WITH NO GIT CONFIGURATION OF THE MACHINE'S: here the
    // caller's own demands a signature of every commit, from a program that gives none.
    let global = canary.rig.dir().join("a-global-config");
    fs::write(
        &global,
        "[commit]\n\tgpgsign = true\n[gpg]\n\tprogram = false\n",
    )
    .expect("write a global configuration");
    let root = canary.root("canary").display().to_string();
    let line = canary
        .tool_in(
            &[("GIT_CONFIG_GLOBAL", &global.display().to_string())],
            &canary.setup_args(&root, RUN, &["--items", "row-doc-list,drive-doc-list,gate"]),
        )
        .done("setup", "ready")
        .clone();
    assert_eq!(
        line["items"],
        json!([{"item": "row-doc-list", "kind": "review-row"},
               {"item": "drive-doc-list", "kind": "audit-drive"},
               {"item": "gate", "kind": "held-gate"}]),
        "{line}"
    );
    let clone = canary.root("canary").join("clone");
    let state = canary.state(&clone);
    assert_eq!(state["items"].as_array().map(Vec::len), Some(3), "{state}");
    assert_eq!(state["next"], "test", "{state}");
    assert_eq!(
        json!([line["expect"]["agents"], line["expect"]["tool_steps"]]),
        json!([26, 17]),
        "{line}"
    );
    assert_eq!(
        line["expect"]["wall_minutes"],
        Value::Null,
        "nobody estimated this set: {line}"
    );

    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let ran = canary.invoke(&line, &line["args"], stage_script(&line, refuted()), 1);
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);
    assert_eq!(
        json!([line["expect"]["agents"], line["expect"]["tool_steps"]]),
        json!([
            ran.agents.len(),
            ran.agents
                .iter()
                .filter(|label| label.starts_with("git:"))
                .count()
        ]),
        "the line's count is the stage's: {:?}",
        ran.agents
    );
    assert!(
        !ran.agents
            .iter()
            .any(|label| label.starts_with("preflight-second")),
        "no trial arm, so no trial image: {:?}",
        ran.agents
    );
}

#[test]
fn setup_refuses_a_root_it_may_not_use_and_a_run_or_a_commit_the_repository_cannot_give_it() {
    let canary = Canary::new("refuses");
    let rig = &canary.rig;
    let setup = |root: &str, run: &str| canary.tool(&canary.setup_args(root, run, &[]));

    // INSIDE THE REPOSITORY — by its real path: the root itself, a directory below it, and
    // a link of plain segments that leads into it.
    let inside = rig.root.join("canary").display().to_string();
    setup(&inside, RUN).refused("inside");
    setup(&rig.root.join("crates/canary").display().to_string(), RUN).refused("inside");
    let link = canary.root("a-link-inside");
    std::os::unix::fs::symlink(rig.root.join("crates"), &link).expect("link into the repository");
    setup(&link.display().to_string(), RUN).refused("inside");
    setup(&link.join("canary").display().to_string(), RUN).refused("inside");
    assert!(!rig.root.join("canary").exists() && !rig.root.join("crates/canary").exists());
    // EVERY WORKING TREE OF THE REPOSITORY IS INSIDE IT: a linked one seen from the main
    // one, and the main one seen from the tool as a linked one holds it.
    let linked = canary.root("a-linked-working-tree");
    rig.git(&[
        "worktree",
        "add",
        "-q",
        "--detach",
        &linked.display().to_string(),
    ]);
    setup(&linked.join("canary").display().to_string(), RUN).refused("inside");
    let mut from_linked = canary.command(&linked.join(TOOL));
    let in_main = rig.root.join("canary").display().to_string();
    from_linked
        .args(canary.setup_args(&in_main, RUN, &[]))
        .current_dir(rig.dir());
    canary.told(from_linked).refused("inside");
    assert!(!rig.root.join("canary").exists() && !linked.join("canary").exists());

    // NOT ABSOLUTE: a relative root would be read from wherever the caller stands.
    setup("canary", RUN).refused("usage");
    setup("./canary", RUN).refused("usage");
    assert!(
        !rig.dir().join("canary").exists(),
        "nothing was made where the tool was called"
    );
    // No plain path: the harness takes no other scratch root.
    setup(&canary.root("a root").display().to_string(), RUN).refused("usage");

    // A ROOT WHOSE PARENT IS NOT THERE is made by nobody.
    setup(&canary.root("absent/canary").display().to_string(), RUN).refused("no-root");
    assert!(!canary.root("absent").exists());
    // A file is no root.
    fs::write(canary.root("a-file"), "no directory\n").expect("write a file");
    setup(&canary.root("a-file").display().to_string(), RUN).refused("no-root");
    // What the file system refuses is a refusal, with its line: a name it has no room for.
    setup(&canary.root(&"n".repeat(300)).display().to_string(), RUN).refused("tool");

    // A ROOT THAT HOLDS ANYTHING — a file, or a link planted where the clone would go: it is
    // refused, and what the link leads to is untouched.
    let held = canary.root("held");
    fs::create_dir(&held).expect("make a root");
    fs::write(held.join("a-file"), "something\n").expect("write a file");
    setup(&held.display().to_string(), RUN).refused("exists");
    let planted = canary.root("planted");
    let target = canary.root("the-links-target");
    fs::create_dir(&planted).expect("make a root");
    fs::create_dir(&target).expect("make the link's target");
    std::os::unix::fs::symlink(&target, planted.join("clone")).expect("plant a link");
    setup(&planted.display().to_string(), RUN).refused("exists");
    assert_eq!(
        fs::read_dir(&target).expect("the target").count(),
        0,
        "nothing was written through the link"
    );
    // An empty directory that is there — one `mktemp -d` minted — is a root.
    let minted = canary.root("minted");
    fs::create_dir(&minted).expect("mint a root");
    setup(&minted.display().to_string(), RUN).done("setup", "ready");

    // A RUN THE REPOSITORY HOLDS: its directory in the commit named or in the tree, or its
    // loop branch — here, or on a remote.
    rig.write(
        "completions/artifacts/held-in-a-commit/opening.md",
        "# an opening\n",
    );
    let later = rig.commit("docs(record): a run of the source's own");
    rig.git(&["rm", "-q", "-r", "completions/artifacts/held-in-a-commit"]);
    rig.commit("docs(record): and it is gone from the tree again");
    rig.write(
        "completions/artifacts/held-in-the-tree/opening.md",
        "# an opening\n",
    );
    let root = |name: &str| canary.root(name).display().to_string();
    canary
        .tool(&[
            "setup",
            "--root",
            &root("t1"),
            "--commit",
            &later,
            "--run",
            "held-in-a-commit",
            "--previous",
            PREVIOUS,
            "--previous-commit",
            &canary.previous,
        ])
        .refused("taken");
    // In the tree, and in no commit.
    setup(&root("t2"), "held-in-the-tree").refused("taken");
    rig.git(&["branch", "fix/held-as-a-branch"]);
    setup(&root("t3"), "held-as-a-branch").refused("taken");
    rig.git(&[
        "update-ref",
        "refs/remotes/origin/fix/held-on-a-remote",
        "HEAD",
    ]);
    setup(&root("t4"), "held-on-a-remote").refused("taken");
    // A slug outside the grammar names no run.
    setup(&root("t5"), "Canary_Small").refused("usage");

    // A COMMIT THE REPOSITORY CANNOT GIVE: one that does not resolve — the candidate's, or
    // the previous release's — and one that holds no harness to invoke.
    let with = |commit: &str, previous: &str, at: &str| {
        canary.tool(&[
            "setup",
            "--root",
            &root(at),
            "--commit",
            commit,
            "--run",
            RUN,
            "--previous",
            PREVIOUS,
            "--previous-commit",
            previous,
        ])
    };
    with("no-such-rev", &canary.previous, "c1").refused("no-commit");
    with(&canary.commit, &"0".repeat(40), "c2").refused("no-commit");
    with(&canary.commit, "HEAD", "c3").refused("usage");
    with(&canary.previous, &canary.previous, "c5").refused("unfit");
    for unmade in ["t1", "t2", "t3", "t4", "t5", "c1", "c2", "c3", "c5"] {
        assert!(
            !canary.root(unmade).exists(),
            "a refused setup makes nothing: `{unmade}` is there"
        );
    }

    // A COMMIT WHOSE RECORD SCRIPT OPENS A RUN THAT IS NOT READY: the setup calls no such
    // run a canary. Here that script is wrapped, and the wrapper drops the stop mode.
    let script = fs::read(rig.root.join(RECORD)).expect("read the record script");
    placed_executable::write(&rig.root.join(format!("{RECORD}.real")), script);
    placed_executable::write(&rig.root.join(RECORD), FORGETFUL_RECORD);
    let forgetful = rig.commit("chore: a record script that forgets the stop mode");
    with(&forgetful, &canary.previous, "n1").refused("not-fresh");
    assert!(
        !canary.root("n1").join("canary.json").exists(),
        "a setup that fails leaves no canary for `check` to take"
    );
    canary
        .tool(&["check", "--root", &root("n1")])
        .refused("missing");
}

/// A record script that hands every call to the real one beside it — but for the run's
/// facts, from which it drops the stop mode: the run it opens is not ready.
const FORGETFUL_RECORD: &str = r#"#!/usr/bin/env python3
import json, os, subprocess, sys
real = os.path.join(os.path.dirname(os.path.realpath(__file__)), "stabilize-record.real")
fed = sys.stdin.buffer.read()
if sys.argv[1:2] == ["run-set"]:
    facts = json.loads(fed)
    facts.pop("stop", None)
    fed = json.dumps(facts).encode()
sys.exit(subprocess.run([sys.executable, real] + sys.argv[1:], input=fed).returncode)
"#;

#[test]
fn nothing_is_written_inside_the_repository_the_tool_is_run_from() {
    let canary = Canary::new("untouched");
    let before = canary.snapshot();
    assert!(
        before.contains_key(".git/HEAD") && before.contains_key(TOOL),
        "the snapshot is of the whole directory, `.git` included"
    );

    // A whole setup, a check of it, a second setup that is refused, a setup that is refused
    // for a root INSIDE the repository — and each of them called from inside the repository.
    let root = canary.root("canary").display().to_string();
    let inside = canary.rig.root.join("canary").display().to_string();
    let calls: Vec<Vec<&str>> = vec![
        canary.setup_args(&root, RUN, &[]),
        vec!["check", "--root", &root],
        canary.setup_args(&root, RUN, &[]),
        canary.setup_args(&inside, RUN, &[]),
        canary.setup_args("canary", RUN, &[]),
        vec!["check", "--root", &inside],
        vec!["--help"],
    ];
    let mut statuses = Vec::new();
    for args in &calls {
        let out = canary
            .command(&canary.rig.root.join(TOOL))
            .args(args)
            .current_dir(&canary.rig.root)
            .stdin(Stdio::null())
            .output()
            .expect("run the tool");
        statuses.push(out.status.code().expect("the tool exits"));
    }
    assert_eq!(statuses, [0, 0, 5, 3, 2, 3, 0], "each call's exit status");
    assert_eq!(
        differing(&before, &canary.snapshot()),
        Vec::<String>::new(),
        "no file of the repository — its tree, its refs, its configuration, its index — is \
         other than it was"
    );
    assert!(
        canary.root("canary").join("clone").is_dir(),
        "and the canary was made"
    );
}

#[test]
fn check_refuses_a_clone_that_could_reach_anything_but_its_bare_remote() {
    let canary = Canary::new("check");
    let line = canary.ready("canary", &[]);
    let root = canary.root("canary");
    let (clone, origin) = (root.join("clone"), root.join("origin.git"));
    let at = root.display().to_string();
    let bare = origin.display().to_string();
    let check = || canary.tool(&["check", "--root", &at]);

    // AS SETUP LEFT IT: checked, and the facts a later read wants.
    let told = check();
    let read = told.done("check", "checked");
    assert_eq!(
        json!([
            read["run"],
            read["loop"],
            read["branch"],
            read["remotes"],
            read["fetch_url"],
            read["push_url"]
        ]),
        json!([RUN, LOOP, LOOP, ["origin"], bare, bare]),
        "{read}"
    );
    assert_eq!(
        json!([
            read["head"],
            read["remote_head"],
            read["pushed"],
            read["records"],
            read["pending"]
        ]),
        json!([line["head"], line["head"], true, 0, 0]),
        "{read}"
    );
    assert_eq!(
        read["state"],
        json!({"next": "test", "round": 1, "attempt": 1}),
        "{read}"
    );
    assert_eq!(
        read["returned"],
        json!({"first": false, "then": false}),
        "no invocation has returned yet: {read}"
    );

    // A SECOND REMOTE.
    canary.git(
        &clone,
        &["remote", "add", "upstream", "/somewhere/else.git"],
    );
    check().refused("not-isolated");
    canary.git(&clone, &["remote", "remove", "upstream"]);
    check().done("check", "checked");

    // A REWRITE IN THE CLONE'S OWN CONFIGURATION — also one that rewrites nothing the clone
    // pushes to today: a clone made by this tool has no business holding one.
    canary.git(
        &clone,
        &[
            "config",
            "url./somewhere/else/.insteadOf",
            "/nowhere/at/all/",
        ],
    );
    check().refused("not-isolated");
    canary.git(
        &clone,
        &["config", "--unset", "url./somewhere/else/.insteadOf"],
    );
    canary.git(
        &clone,
        &[
            "config",
            "url./somewhere/else/.pushInsteadOf",
            "/nowhere/at/all/",
        ],
    );
    check().refused("not-isolated");
    canary.git(
        &clone,
        &["config", "--unset", "url./somewhere/else/.pushInsteadOf"],
    );
    check().done("check", "checked");

    // A REWRITE THE SESSION'S OWN CONFIGURATION HOLDS — outside the clone: what git would
    // push to is asked of git, under the configuration the caller runs with.
    let global = canary.rig.dir().join("a-global-config");
    fs::write(
        &global,
        format!("[url \"/somewhere/else.git\"]\n\tpushInsteadOf = {bare}\n"),
    )
    .expect("write a global configuration");
    canary
        .tool_in(
            &[("GIT_CONFIG_GLOBAL", &global.display().to_string())],
            &["check", "--root", &at],
        )
        .refused("not-isolated");
    check().done("check", "checked");

    // ANOTHER URL — to fetch from, or only to push to.
    canary.git(
        &clone,
        &[
            "remote",
            "set-url",
            "--push",
            "origin",
            "/somewhere/else.git",
        ],
    );
    check().refused("not-isolated");
    canary.git(&clone, &["config", "--unset", "remote.origin.pushurl"]);
    canary.git(
        &clone,
        &["remote", "set-url", "origin", "/somewhere/else.git"],
    );
    check().refused("not-isolated");
    // To fetch from only: what the clone pushes to is still the bare repository.
    canary.git(&clone, &["remote", "set-url", "--push", "origin", &bare]);
    check().refused("not-isolated");
    canary.git(&clone, &["config", "--unset", "remote.origin.pushurl"]);
    canary.git(&clone, &["remote", "set-url", "origin", &bare]);
    check().done("check", "checked");

    // THE SOURCE'S REMOTE, OR THE SOURCE ITSELF, NAMED ANYWHERE IN THE CLONE'S CONFIGURATION
    // — where no remote and no rewrite holds it: a branch's own remote may be a URL, and a
    // bare `git push` would follow it.
    for (key, elsewhere) in [
        (
            format!("branch.{LOOP}.pushRemote"),
            canary.rig.origin.display().to_string(),
        ),
        (
            format!("branch.{LOOP}.remote"),
            canary.rig.root.display().to_string(),
        ),
    ] {
        canary.git(&clone, &["config", &key, &elsewhere]);
        check().refused("not-isolated");
        canary.git(&clone, &["config", "--unset", &key]);
    }
    check().done("check", "checked");

    // WHAT AN INVOCATION RETURNED is where the line said it would be written.
    fs::write(line["returns"]["first"].as_str().expect("a path"), "{}\n").expect("write a return");
    assert_eq!(
        check().done("check", "checked")["returned"],
        json!({"first": true, "then": false})
    );

    // A DIRECTORY OF THE CANARY THAT IS A LINK: nothing is read through it.
    let moved = canary.root("the-clone-moved-away");
    fs::rename(&clone, &moved).expect("move the clone away");
    std::os::unix::fs::symlink(&moved, &clone).expect("plant a link");
    check().refused("link");
    fs::remove_file(&clone).expect("remove the link");
    fs::rename(&moved, &clone).expect("move the clone back");
    check().done("check", "checked");

    // A ROOT THAT HOLDS NO CANARY, and one inside the repository.
    fs::create_dir(canary.root("empty")).expect("make a root");
    canary
        .tool(&[
            "check",
            "--root",
            &canary.root("empty").display().to_string(),
        ])
        .refused("missing");
    canary
        .tool(&[
            "check",
            "--root",
            &canary.root("absent").display().to_string(),
        ])
        .refused("missing");
    canary
        .tool(&[
            "check",
            "--root",
            &canary.rig.root.join("canary").display().to_string(),
        ])
        .refused("inside");
}

/// What makes a command line more than one plain invocation.
const SHELL: [&str; 12] = [
    "|", ">", "<", "&", ";", "$(", "`", "sleep", "nohup", "\n", "*", "~",
];

#[test]
fn every_command_the_tool_names_is_one_plain_invocation_of_a_committed_tool() {
    let canary = Canary::new("plain");
    let line = canary.ready("canary", &[]);
    let root = canary.root("canary").display().to_string();

    // THE LINE NAMES ONE COMMAND, and it is the tool's own `check`: plain words, the first
    // of them a file the commit named holds under `dev/`.
    let command = line["check"].as_str().expect("the check's command");
    assert_eq!(command, format!("{TOOL} check --root {root}"), "{line}");
    for token in SHELL {
        assert!(!command.contains(token), "`{command}` holds `{token}`");
    }
    let words: Vec<&str> = command.split(' ').collect();
    assert!(
        words.iter().all(|word| !word.is_empty()
            && word
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"/._=:-".contains(&b))),
        "plain words: {command}"
    );
    assert!(
        words[0].starts_with("dev/")
            && canary
                .rig
                .git_ok(&["cat-file", "-e", &format!("{}:{}", canary.commit, words[0])]),
        "`{}` is a file the commit holds under dev/",
        words[0]
    );
    // RUN AS PRINTED, from the repository's root, by a shell.
    let mut shell = canary.command(Path::new("sh"));
    shell.args(["-c", command]).current_dir(&canary.rig.root);
    canary.told(shell).done("check", "checked");

    // NO FIELD OF THE LINE IS A COMMAND TO COMPOSE: no other value holds a shell's word.
    for (name, value) in line.as_object().expect("the line is an object") {
        if name == "check" || name == "door" {
            continue;
        }
        let text = value.to_string();
        for token in ["|", "&&", "$(", "`", ";", "sleep"] {
            assert!(!text.contains(token), "`{name}` holds `{token}`: {text}");
        }
    }

    // THE STEPS THE TOOL'S OWN HELP GIVES: every command line of it is one plain invocation
    // too — and it says by which path the harness is invoked, and where the session stands.
    let help = canary
        .command(&canary.rig.root.join(TOOL))
        .arg("--help")
        .output()
        .expect("run --help");
    assert!(help.status.success(), "`--help` exits 0");
    let help = String::from_utf8(help.stdout).expect("the help is UTF-8");
    let commands: Vec<&str> = help
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("dev/") || line.starts_with("mktemp "))
        .collect();
    assert!(
        commands.len() >= 3,
        "the help gives its commands: {commands:?}"
    );
    for command in commands {
        for token in SHELL {
            assert!(
                !command.contains(token),
                "the help's `{command}` holds `{token}`"
            );
        }
    }
    for said in [
        "scriptPath",
        "session_cwd",
        "args_then",
        "WHAT IT SHOWS",
        "WHAT IT DOES NOT SHOW",
        "TO DISPOSE OF IT",
    ] {
        assert!(help.contains(said), "the help says `{said}`");
    }
}
