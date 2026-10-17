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
//! **Under one switch, `--parts`, the setup writes another opening** — the one the reshaped
//! `test` stage is run on in parts
//! ([staged-canary.md](../completions/artifacts/M55/stabilization-build/reshape/staged-canary.md)
//! → §4, and §10's `SC2`; [DECISIONS.md](../DECISIONS.md) → *2026-10-12 — The canary's
//! setup opens a run with a queue*): a run with a queue, on the door `jigc setup`, whose
//! ledger holds the plan's plants. Its arms:
//!
//! - the facts — a budget, the number a verifier hands on, an order of classes and the entry
//!   that states the closing condition — as given, and as the flags give them
//!   ([`a_fresh_setup_is_ready_with_the_budget_the_order_and_the_hand_on_it_was_given`]);
//! - the plants, word for word, each graded at the opening with its class and its kind's
//!   evidence — so that the queue is empty, nothing is untriaged and `next` is `test`
//!   ([`its_seeded_rows_carry_the_grade_and_the_class_the_canary_plants`]);
//! - the order part 3 rules, written through the clone's own record script, and the queue
//!   it leaves
//!   ([`the_order_part_three_rules_puts_the_route_plants_and_the_reserve_into_the_queue_in_that_rank`]);
//! - the one door ([`its_door_is_the_review_rows_and_the_plants`]);
//! - what the switch refuses
//!   ([`under_the_switch_a_flag_outside_its_grammar_a_log_without_the_one_entry_and_a_plant_nobody_graded_are_refused`]);
//! - and **without the switch a setup is what it was, to the byte** — its line, the file it
//!   keeps, its commit and every file of the opening, held to what the tool made before
//!   the switch existed ([`a_setup_without_the_switch_is_what_it_was`];
//!   `tooling-tests/fixtures/stabilize-canary-setup.golden`).
//!
//! **And under that switch the setup writes what the human invokes, act by act, and `check`
//! reads a part back** (the same plan's §5, §8 and §10, `SC3`; [DECISIONS.md](../DECISIONS.md)
//! → *2026-10-13 — The canary's setup writes the parts' prompts, and `check` reads the
//! parts*). Its arms:
//!
//! - one prompt file for each act, which holds the harness's path, the return file and the
//!   one argument object the line names, and nothing a shell reads; each act's count; the
//!   tool that wrote the opening, by its hash
//!   ([`a_prompt_file_holds_the_arguments_its_line_names_and_nothing_a_shell_reads`]);
//! - **the acts, run in order through the clone's own harness, each as its prompt file says**
//!   — the first stage ever run on that opening: part 1 to its stop, part 2 to its return,
//!   the order ruled, part 3 to its second record, the budget raised, a wave that dies after
//!   two filings, a look and the continuation, each launching as many agents as its line
//!   says ([`the_acts_run_in_order_through_the_clones_own_harness`]);
//! - what `check` says of each act — whether its return is there, and whether the run shows
//!   it — and of the held commands
//!   ([`check_says_of_each_act_what_was_returned_and_what_the_run_shows`]);
//! - a filed verdict with one hash before and after the record that takes it, in the root
//!   the plan opens afresh for its fourth part
//!   ([`a_filed_verdict_is_printed_with_its_hash_before_and_after_the_record_that_takes_it`]);
//! - a write that was killed read as a state, and no refusal of a tool `check` calls turned
//!   into *the root holds no canary*
//!   ([`check_reads_a_killed_write_as_a_state_and_never_says_of_a_canary_that_the_root_holds_none`]);
//! - and the planted rows: no key and no cell says how a claim falls or that it was planted
//!   ([`no_planted_row_says_which_way_its_claim_falls_or_that_it_was_planted`]), and the
//!   answer sheet is `check`'s to print and in no file the setup writes
//!   ([`the_answer_sheet_is_checks_to_print_and_is_in_no_file_the_setup_writes`]);
//! - and the one item part 3 hands on for certain (the human's ruling of 2026-10-16): the
//!   first planted row quotes it with its kind's evidence, filed as a verifier hands it on it
//!   enters the queue ahead of the reserve, `check` says how it fell beside what the sheet
//!   says it should return, and the clone's tree does not hold that answer
//!   ([`the_first_rows_drive_meets_an_item_its_verifier_hands_on_and_check_says_how_it_fell`]).
//!
//! **And what the second review before the first part found in the tool** (the staged plan's
//! `R1`; [DECISIONS.md](../DECISIONS.md) → *2026-10-13 — The canary's setup refuses a commit
//! whose tool is not the one running, and `check` says what is so*): the setup refuses a
//! commit that does not hold the tool as it runs, a budget whose raises the harness would
//! not take, and writes a prompt file for each ruling off the main line
//! ([`a_prompt_file_holds_the_arguments_its_line_names_and_nothing_a_shell_reads`]); `check`
//! names a batch that is applied and in no commit with every field it read across it, runs
//! no git command that takes a lock, and says of a link among the filed verdicts what it is
//! ([`check_says_what_is_applied_and_in_no_commit_and_takes_no_lock_a_stage_could_meet`]; the
//! window before the parts' last act, on the main line and in the root opened afresh, in
//! the two drives above); says of each held command what the step tool would answer, of each
//! return whether it is the act's own, and refuses a kept line it cannot read
//! ([`check_says_of_each_act_what_was_returned_and_what_the_run_shows`]); names the five
//! files that hold an answer
//! ([`the_answer_sheet_is_checks_to_print_and_is_in_no_file_the_setup_writes`]); and names
//! the ruling that is the step, whose prompt file the harness takes
//! ([`the_ruling_off_the_main_line_is_named_where_it_is_the_step_and_the_harness_takes_it`]).
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

use super::dev_stabilize_step::{LONG_COMMAND, StepRig, node_or_skip};
use super::placed_executable;
use super::stabilize_simulation::CARGO;

const TOOL: &str = "dev/stabilize-canary";
const RECORD: &str = "dev/stabilize-record";
const HARNESS: &str = ".claude/workflows/stabilize.js";
const RUNTIME: &str = "tooling-tests/fixtures/stabilize-runtime.mjs";
/// The write guard, its fence and its probe: what a commit a root of the parts is set up
/// from must hold, and the probe's script, which act `part-0` invokes by its path.
const GUARD_NEEDS: [&str; 3] = ["dev/stabilize-guard", "dev/stabilize-fence", GUARD_SCRIPT];
const GUARD_SCRIPT: &str = "dev/stabilize-guard-probe.js";

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
    /// Every prompt an agent was handed: `(its label, the prompt)`, in order.
    prompts: Vec<(String, String)>,
    /// Every command the stand-in ran on an agent's behalf, as its prompt spelled it.
    commands: Vec<String>,
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

    /// A source whose log holds the ONE entry that states the exit rule: what a setup under
    /// the switch names as the run's closing condition.
    fn parts(label: &str) -> Self {
        let mut canary = Canary::new(label);
        canary.guarded();
        canary.log(&[EXIT_RULE]);
        canary
    }

    /// THE WRITE GUARD, ITS FENCE AND ITS PROBE, committed: a root of the parts runs the
    /// probe first, and a commit that cannot is refused (unfit).
    fn guarded(&mut self) {
        for tool in GUARD_NEEDS {
            placed_executable::copy(&repo_root().join(tool), &self.rig.root.join(tool));
        }
        self.commit = self
            .rig
            .commit("chore: the write guard, its fence and its probe");
    }

    /// The source's log with one entry more under each of `headings`, committed: the commit
    /// a setup is handed from then on.
    fn log(&mut self, headings: &[&str]) {
        let mut log = self.rig.read("DECISIONS.md");
        for heading in headings {
            log.push_str(&format!("\n## {heading}\n\nAn entry.\n"));
        }
        self.rig.write("DECISIONS.md", &log);
        self.commit = self.rig.commit("docs: the log's entries");
    }

    /// A write of the clone's own record script, fed `fed`: whether it was taken, and what
    /// it printed on stderr.
    fn record(&self, clone: &Path, act: &str, fed: &Value) -> (bool, String) {
        let mut child = self
            .command(&clone.join(RECORD))
            .args([act, "--run", RUN])
            .current_dir(self.rig.dir())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the clone's record script");
        child_stdin::feed(&mut child, fed.to_string());
        let out = child
            .wait_with_output()
            .expect("the clone's record script exits");
        (
            out.status.success(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        )
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
    fn invoke(&self, line: &Value, args: &Value, script: Value, n: u32) -> Ran {
        self.invoke_by(
            line["script_path"].as_str().expect("the harness's path"),
            line["session_cwd"]
                .as_str()
                .expect("the session's directory"),
            args,
            script,
            n,
        )
    }

    /// One invocation of the harness at the path `harness`, run by the runtime's stand-in
    /// from the directory `cwd`, with `args`. `script` is what the scripted agents say.
    fn invoke_by(&self, harness: &str, cwd: &str, args: &Value, mut script: Value, n: u32) -> Ran {
        script["args"] = args.clone();
        let file = self.rig.dir().join(format!("scenario-{n}.json"));
        fs::write(&file, script.to_string()).expect("write the scenario");
        let out = self
            .command(Path::new("node"))
            .arg(repo_root().join(RUNTIME))
            .arg(harness)
            .arg(&file)
            .current_dir(cwd)
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
            prompts: said["prompts"]
                .as_array()
                .expect("the stand-in reports `prompts`")
                .iter()
                .map(|p| {
                    (
                        p["label"].as_str().expect("a label").to_owned(),
                        p["prompt"].as_str().expect("a prompt").to_owned(),
                    )
                })
                .collect(),
            commands: list("ran"),
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
        [format!("push origin {head}:refs/heads/{LOOP}")],
        "the step tool's pushes: the opening's commit, by its sha, into the loop branch's ref"
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
    let candidate = &result["candidate"]["sha"]
        .as_str()
        .expect("the candidate's commit")[..12];
    let held: Vec<&str> = ran
        .agents
        .iter()
        .filter_map(|label| label.strip_prefix("git:hold-start:"))
        .collect();
    assert_eq!(
        held,
        [
            format!("build-candidate-{candidate}").as_str(),
            format!("build-previous-{previous}").as_str(),
            format!("gate-{candidate}").as_str(),
            format!("regression-set-{candidate}-a1").as_str(),
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

    // UNDER THE SWITCH THE LINE NAMES THE SAME ONE COMMAND, and nothing else of it is a
    // command to compose: not an act's argument object, not the note beside a count, not a
    // path (the plan's `SC3`: this arm, carried). The help's command lines for the parts —
    // the setup of the root, and of each root that is opened afresh after a repair — are
    // among those the loop above held to plain words.
    let parts = Canary::parts("plain-parts");
    let line = parts.ready("canary", &[PARTS]);
    let root = parts.root("canary").display().to_string();
    assert_eq!(
        line["check"],
        json!(format!("{TOOL} check --root {root}")),
        "{line}"
    );
    assert!(line["acts"].is_array(), "{line}");
    for (name, value) in line.as_object().expect("the line is an object") {
        if name == "check" || name == "door" {
            continue;
        }
        let text = value.to_string();
        for token in ["|", "&&", "$(", "`", ";", "sleep"] {
            assert!(!text.contains(token), "`{name}` holds `{token}`: {text}");
        }
    }
    assert!(
        help.lines()
            .map(str::trim_start)
            .filter(|line| line.starts_with("dev/") && line.contains(" --parts"))
            .count()
            >= 4,
        "the help gives the setup of the parts' root, and of each root opened afresh: {help}"
    );
    for said in ["THE ACTS OF A RUN IN PARTS", "prompts", "returned-unshown"] {
        assert!(help.contains(said), "the help says `{said}`");
    }
}

// --- under the switch: the opening of a run in parts ---------------------------------

/// The switch, the one door of the opening it writes, and the facts that opening holds
/// where no flag says otherwise.
const PARTS: &str = "--parts";
const PARTS_DOOR: &str = "jigc setup";
const BUDGET: u64 = 9;
const HAND_ON: u64 = 2;
const ORDER: [&str; 2] = ["route under-confirmed", "behaviour under-confirmed"];
/// The order part 3 rules, as the record script takes it and as the tool's flag spells it.
const ORDER_THREE: [&str; 4] = [
    "route under-confirmed",
    "route seed",
    "behaviour under-confirmed",
    "behaviour seed",
];
const ORDER_THREE_FLAG: &str =
    "route:under-confirmed,route:seed,behaviour:under-confirmed,behaviour:seed";
/// The heading of the entry of the log that states the exit rule, whole: what the run
/// names as where its closing condition is stated.
const EXIT_RULE: &str = "2026-10-04 — The exit rule, revised: four clauses, each with a scope and one instrument — a finding blocks the call only if it breaks a clause inside its scope (the human's)";

/// The opening's items under the switch when none is named, by id and kind.
const PARTS_ITEMS: [(&str, &str); 4] = [
    ("row-setup", "review-row"),
    ("gate", "held-gate"),
    ("regression-set", "held-regression"),
    ("regression-again", "held-regression"),
];

/// The two route lines the plants quote, each as the binary prints it after its two
/// leading spaces — the first whole, to its last word — and the refusal's code.
const ROUTE_LINK: &str = "route: remove the link at `<rig>/outside/shared-hook`, or point it at a script inside this repository, then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link";
const ROUTE_WRITABLE: &str =
    "route: ensure the repo's git hooks directory is writable, then re-run `jigc setup`";
const CODE: &str = "setup.install-hook";

/// **The plants, word for word** — typed here a second time, from the table the plan's
/// step `P0` drove on both binaries, so that a plant reworded in the tool is red: the key,
/// the class, the claim, and the kind's evidence (the route line as printed, or the
/// commands as run; none for the lead).
const PLANTS: [(&str, &str, &str, Option<&str>); 14] = [
    (
        "setup-hook-link-route-deletes-the-script",
        "route seed",
        "where `.git/hooks/pre-commit` is a link to an existing script outside the repository, `jigc setup` refuses at `setup.install-hook` and its route names the script as the link; done as printed, the script is deleted and the re-run exits 1 again; the previous release exits 0",
        Some(ROUTE_LINK),
    ),
    (
        "setup-hooks-mode-555-route-refuses-again",
        "route seed",
        "where the default hooks directory is of mode 555, `jigc setup` refuses at `setup.install-hook`; done as its route prints — the directory made writable — the re-run exits 1 again",
        Some(ROUTE_WRITABLE),
    ),
    (
        "setup-hooks-immutable-route-as-root",
        "route seed",
        "on a Linux host, for a caller that is root, a hooks directory carrying the immutable attribute draws the same refusal; the route as printed changes nothing for root, and the re-run exits 1 again",
        Some(ROUTE_WRITABLE),
    ),
    (
        "setup-several-worktrees",
        "lead seed",
        "`jigc setup` may behave otherwise in a repository with several worktrees; nobody looked",
        None,
    ),
    (
        "setup-exits-1-in-a-fresh-repository",
        "behaviour seed",
        "in `<rig>`, `jigc setup` exits 1",
        Some("`jigc setup`; the exit status"),
    ),
    (
        "setup-makes-no-commit",
        "behaviour seed",
        "in `<rig>`, `jigc setup` exits 0 and makes no commit",
        Some("`git rev-parse HEAD` · `jigc setup` · `git rev-parse HEAD`"),
    ),
    (
        "setup-leaves-the-status-not-empty",
        "behaviour seed",
        "in `<rig>`, `jigc setup` leaves `git status --porcelain` not empty",
        Some("`jigc setup` · `git status --porcelain`"),
    ),
    (
        "setup-makes-two-commits",
        "behaviour seed",
        "in `<rig>`, `jigc setup` makes two commits",
        Some("`git rev-list --count HEAD` · `jigc setup` · `git rev-list --count HEAD`"),
    ),
    (
        "setup-removes-an-untracked-file",
        "behaviour seed",
        "an untracked file `notes.md` at the root of `<rig>` is gone after `jigc setup`",
        Some("`printf 'my notes\\n' > notes.md` · `jigc setup` · `cat notes.md`"),
    ),
    (
        "setup-second-run-exits-1",
        "behaviour seed",
        "a second `jigc setup`, right after the first, exits 1",
        Some("`jigc setup` · `jigc setup`; the second exit status"),
    ),
    (
        "setup-second-run-commits-again",
        "behaviour seed",
        "a second `jigc setup` makes a second commit",
        Some(
            "`jigc setup` · `git rev-list --count HEAD` · `jigc setup` · `git rev-list --count HEAD`",
        ),
    ),
    (
        "setup-leaves-another-branch-checked-out",
        "behaviour seed",
        "`jigc setup` leaves another branch checked out",
        Some("`git branch --show-current` · `jigc setup` · `git branch --show-current`"),
    ),
    (
        "setup-adds-a-remote",
        "behaviour seed",
        "`jigc setup` adds a remote",
        Some("`git remote` · `jigc setup` · `git remote`"),
    ),
    (
        "setup-leaves-a-stash-entry",
        "behaviour seed",
        "`jigc setup` leaves a stash entry",
        Some("`jigc setup` · `git stash list`"),
    ),
];

/// The plants' keys of one class, in the ledger's order.
fn plants_of(class: &str) -> Vec<&'static str> {
    PLANTS
        .iter()
        .filter(|plant| plant.1 == class)
        .map(|plant| plant.0)
        .collect()
}

/// The keys the state's queue ranks, first to last.
fn ranked(state: &Value) -> Vec<String> {
    state["queue"]["ranked"]
        .as_array()
        .unwrap_or_else(|| panic!("a run with a queue says its queue: {state}"))
        .iter()
        .map(|row| row["key"].as_str().expect("a key").to_owned())
        .collect()
}

#[test]
fn a_fresh_setup_is_ready_with_the_budget_the_order_and_the_hand_on_it_was_given() {
    let canary = Canary::parts("parts-ready");
    let line = canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let clone = root.join("clone");

    // (1) THE LINE SAYS THE FACTS OF THE OPENING: the budget, the number a verifier hands
    // on, the order — as the record script holds it — and the entry that states the closing
    // condition, which the setup found in the log of the commit named.
    assert_eq!(
        json!([
            line["budget"],
            line["hand_on"],
            line["order"],
            line["condition"]
        ]),
        json!([BUDGET, HAND_ON, ORDER, EXIT_RULE]),
        "{line}"
    );

    // (2) AND THE RUN HOLDS THEM, as the clone's own record script reads them back: it is a
    // run with a queue, nothing of its budget is spent, and it is ready.
    let state = canary.state(&clone);
    assert_eq!(
        json!([
            state["facts"]["budget"],
            state["facts"]["hand-on"],
            state["facts"]["order"],
            state["facts"]["condition"]
        ]),
        json!([BUDGET, HAND_ON, ORDER, EXIT_RULE]),
        "{state}"
    );
    assert_eq!(
        state["budget"],
        json!({"limit": BUDGET, "spent": 0, "left": BUDGET}),
        "{state}"
    );
    assert_eq!(
        json!([
            state["facts"]["stop"],
            state["facts"]["previous"],
            state["facts"]["previous-commit"],
            state["facts"]["scope"],
            state["facts"]["clauses"]
        ]),
        json!([
            "every-round",
            PREVIOUS,
            canary.previous,
            "delta",
            [
                "no-lost-files",
                "working-product",
                "usable-by-agents",
                "migration-works"
            ]
        ]),
        "{state}"
    );
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

    // (3) THE CANARY IS MADE AS EVERY CANARY IS: one commit on the commit named, pushed to
    // the bare repository beside the clone, a clean tree — and `check` holds it. Under the
    // switch the commit named is first given ONE COMMIT that withholds the answer key in the
    // clone (`the_clones_tree_names_no_planted_row_beside_how_it_falls`): the opening is on
    // that one.
    let head = canary.git(&clone, &["rev-parse", "HEAD"]);
    assert_eq!(
        json!([line["head"], line["remote_head"], line["candidate"]]),
        json!([head, head, canary.commit]),
        "{line}"
    );
    assert_eq!(canary.git(&clone, &["rev-parse", "HEAD^^"]), canary.commit);
    assert_eq!(
        canary.git(&clone, &["diff", "--name-only", "HEAD^^", "HEAD^"]),
        TOOL,
        "the rig's commit names a planted row in the tool alone"
    );
    assert_eq!(
        canary.git(&clone, &["status", "--porcelain", "--untracked-files=all"]),
        ""
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
    let checked = canary.tool(&["check", "--root", &root.display().to_string()]);
    let read = checked.done("check", "checked");
    assert_eq!(
        json!([
            read["pushed"],
            read["records"],
            read["pending"],
            read["state"]
        ]),
        json!([true, 0, 0, {"next": "test", "round": 1, "attempt": 1}]),
        "{read}"
    );

    // (4) THE LINE NAMES WHERE THE SESSION STANDS AND THE HARNESS BY ITS PATH, AND PRINTS NO
    // ARGUMENTS OF ITS OWN: what each act of the parts is invoked with stands with the act
    // (`acts`), and the small canary's two argument objects and its count are of another
    // stage.
    assert_eq!(
        json!([line["session_cwd"], line["script_path"], line["check"]]),
        json!([
            clone.display().to_string(),
            clone.join(HARNESS).display().to_string(),
            format!("{TOOL} check --root {}", root.display())
        ]),
        "{line}"
    );
    for absent in ["args", "args_then", "returns", "expect", "seeded"] {
        assert_eq!(line[absent], Value::Null, "`{absent}`: {line}");
    }

    // (5) AS IT WAS GIVEN: another budget, another order and other items are the caller's
    // flags — here the root the plan opens for its fourth part after a repair: the gate
    // alone, the order part 3 rules, a budget of four. The queue then holds every plant
    // the order names from the opening on, and the run is as fresh.
    let line = canary.ready(
        "part-four",
        &[
            PARTS,
            "--budget",
            "4",
            "--order",
            ORDER_THREE_FLAG,
            "--items",
            "gate",
        ],
    );
    assert_eq!(
        json!([
            line["budget"],
            line["hand_on"],
            line["order"],
            line["items"]
        ]),
        json!([4, HAND_ON, ORDER_THREE, [{"item": "gate", "kind": "held-gate"}]]),
        "{line}"
    );
    let state = canary.state(&canary.root("part-four").join("clone"));
    assert_eq!(
        json!([
            state["facts"]["budget"],
            state["facts"]["hand-on"],
            state["facts"]["order"],
            state["budget"]["left"],
            state["next"]
        ]),
        json!([4, HAND_ON, ORDER_THREE, 4, "test"]),
        "{state}"
    );
    let queue: Vec<&str> = [plants_of("route seed"), plants_of("behaviour seed")].concat();
    assert_eq!(ranked(&state), queue, "{state}");
    assert_eq!(line["queue"], json!(queue), "{line}");
}

#[test]
fn its_seeded_rows_carry_the_grade_and_the_class_the_canary_plants() {
    let canary = Canary::parts("parts-plants");
    let line = canary.ready("canary", &[PARTS]);
    let clone = canary.root("canary").join("clone");
    let state = canary.state(&clone);

    // (1) THE PLANTS, IN THE TABLE'S ORDER — each a row of round 0 at the round's one door,
    // GRADED AT THE OPENING: `unclear`, by the human, as the ledger takes a grade given
    // there — so that no triage decides whether a plant is launched.
    let ledger = state["ledger"].as_array().expect("the ledger");
    assert_eq!(
        ledger.iter().map(|row| &row["key"]).collect::<Vec<_>>(),
        PLANTS.iter().map(|plant| plant.0).collect::<Vec<_>>(),
        "{state}"
    );
    for (row, (key, class, claim, evidence)) in ledger.iter().zip(PLANTS) {
        assert_eq!(
            json!([
                row["round"],
                row["door"],
                row["grade"],
                row["graded_by"],
                row["disposition"]
            ]),
            json!([0, PARTS_DOOR, "unclear", "human", "open"]),
            "`{key}`: {row}"
        );
        // The clause its claim would break: the second, but for the one claim of a file
        // that is gone, which is the first's.
        assert_eq!(
            row["clause"],
            if key == "setup-removes-an-untracked-file" {
                "no-lost-files"
            } else {
                "working-product"
            },
            "`{key}`: {row}"
        );
        // ITS CLASS: its kind and where it comes from — a seed, every one — and ITS KIND'S
        // EVIDENCE, word for word: the route line as printed, or the commands as run. The
        // lead carries none, and was given as nothing but a lead.
        let (kind, origin) = class.split_once(' ').expect("a class is two words");
        assert_eq!(
            json!([
                row["class"]["kind"],
                row["class"]["origin"],
                row["class"]["given"],
                row["class"]["parent"],
                row["class"]["evidence"]
            ]),
            json!([kind, origin, null, null, evidence]),
            "`{key}`: {row}"
        );
        // A route plant names the refusal's code; no other row names one.
        assert_eq!(
            row["class"]["code"],
            if kind == "route" {
                json!(CODE)
            } else {
                Value::Null
            },
            "`{key}`: {row}"
        );
        // THE CLAIM, WORD FOR WORD, opens the row's repro — and nothing of the row says
        // which way the claim falls.
        let repro = row["repro"].as_str().expect("a repro");
        assert!(
            repro.starts_with(&format!("The claim: {claim}.")),
            "`{key}` holds its claim word for word: {repro}"
        );
        // A VERIFIER HAS THE ROW AND NOTHING ELSE: every row with evidence says how its
        // state is built — the rig, and what `<rig>` stands for in that row.
        if evidence.is_some() {
            assert!(
                repro.contains("The state: `<rig>` is ")
                    && repro.contains("`dev/jigc-rig bare`")
                    && repro.contains("`--binary`"),
                "`{key}` says how its state is built: {repro}"
            );
        }
        let told = json!([row["source"], row["repro"], row["class"]["evidence"]]).to_string();
        for answer in ["false", "refuted", "confirmed", "reserve", "ballast"] {
            assert!(
                !told.contains(answer),
                "a text of `{key}` says `{answer}`: {told}"
            );
        }
    }
    // The regression's row is the one whose `<rig>` is the rig's ROOT: its route line names
    // the script beside the repository, and its repro says where both lie and cites the
    // block of the first canary's report that is this drive.
    let regression = ledger[0]["repro"].as_str().expect("a repro");
    for said in [
        "the repository is `<rig>/repo`",
        "`<rig>/outside/shared-hook`",
        "`Repro VR-1`",
        "the previous release's binary",
    ] {
        assert!(regression.contains(said), "`{said}`: {regression}");
    }
    // NO PATH OF A MACHINE in any row, and none in the opening record.
    let opening =
        fs::read_to_string(clone.join(RUN_DIR).join("opening.md")).expect("the opening record");
    for text in [&state["ledger"].to_string(), &opening] {
        for host in ["/Users/", "/home/", "/private/", "/tmp/", "/var/"] {
            assert!(!text.contains(host), "`{host}`: {text}");
        }
    }
    assert!(
        opening
            .bytes()
            .all(|b| b == b'\n' || (0x20..0x7f).contains(&b)),
        "the opening record is printable ASCII: {opening}"
    );
    for said in [
        "SYNTHETIC",
        "The exit rule, revised",
        PARTS_DOOR,
        "setup-hook-link-route-deletes-the-script",
        "setup-leaves-a-stash-entry",
    ] {
        assert!(
            opening.contains(said),
            "the opening record says `{said}`:\n{opening}"
        );
    }

    // (2) AT THE OPENING THE QUEUE IS EMPTY, NOTHING IS UNTRIAGED AND `next` IS `test`: the
    // order names two classes only a verifier's filing gives a row, so every plant is
    // listed — the lead as a lead, every other as off the order — and none is the human's.
    assert_eq!(
        json!([
            state["queue"]["ranked"],
            state["untriaged"],
            state["human_list"],
            state["next"]
        ]),
        json!([[], [], [], "test"]),
        "{state}"
    );
    for row in ledger {
        assert_eq!(
            json!([row["route"], row["why"]]),
            json!([
                "listed",
                if row["class"]["kind"] == "lead" {
                    "lead"
                } else {
                    "class-off-the-order"
                }
            ]),
            "{row}"
        );
    }

    // (3) AND THE LINE SAYS IT: the plants by key and class, the queue, and what is listed
    // by the word that keeps it out.
    assert_eq!(
        line["plants"],
        Value::Array(
            PLANTS
                .iter()
                .map(|plant| json!({"key": plant.0, "class": plant.1}))
                .collect()
        ),
        "{line}"
    );
    let off: Vec<&str> = [plants_of("route seed"), plants_of("behaviour seed")].concat();
    assert_eq!(
        json!([line["queue"], line["listed"]]),
        json!([[], {"class-off-the-order": off, "lead": ["setup-several-worktrees"]}]),
        "{line}"
    );
}

#[test]
fn the_order_part_three_rules_puts_the_route_plants_and_the_reserve_into_the_queue_in_that_rank() {
    let canary = Canary::parts("parts-order");
    canary.ready("canary", &[PARTS]);
    let clone = canary.root("canary").join("clone");
    assert_eq!(ranked(&canary.state(&clone)), Vec::<String>::new());

    // THE ORDER PART 3 RULES, written as a ruling writes it: one `run-set` of the clone's
    // own record script, and nothing else.
    let (taken, said) = canary.record(&clone, "run-set", &json!({"order": ORDER_THREE}));
    assert!(taken, "another order is taken at any step: {said}");
    let state = canary.state(&clone);

    // THIRTEEN ROWS: the three route plants, then the ten reserve plants, each class in the
    // ledger's order — and the lead in no queue.
    let (route, reserve) = (plants_of("route seed"), plants_of("behaviour seed"));
    assert_eq!(
        route,
        [
            "setup-hook-link-route-deletes-the-script",
            "setup-hooks-mode-555-route-refuses-again",
            "setup-hooks-immutable-route-as-root"
        ]
    );
    assert_eq!(reserve.len(), 10);
    assert_eq!(
        ranked(&state),
        [route.clone(), reserve.clone()].concat(),
        "{state}"
    );
    assert_eq!(
        state["queue"]["classes"],
        json!([
            {"kind": "route", "origin": "under-confirmed", "length": 0},
            {"kind": "route", "origin": "seed", "length": 3},
            {"kind": "behaviour", "origin": "under-confirmed", "length": 0},
            {"kind": "behaviour", "origin": "seed", "length": 10}
        ]),
        "{state}"
    );
    let lead = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .find(|row| row["key"] == "setup-several-worktrees")
        .expect("the lead");
    assert_eq!(
        json!([lead["route"], lead["why"]]),
        json!(["listed", "lead"])
    );
    // THE QUEUE IS LONGER THAN THE BUDGET, of which nothing is spent; nothing is untriaged;
    // and the round, which has no candidate yet, is still the `test` stage's to begin.
    assert_eq!(
        json!([
            state["budget"],
            state["untriaged"],
            state["next"],
            state["position"]["test"]
        ]),
        json!([{"limit": BUDGET, "spent": 0, "left": BUDGET}, [], "test", {"round": 1, "attempt": 1}]),
        "{state}"
    );

    // THE CONTROL — THE RANK IS THE ORDER'S, never the ledger's: with the reserve's class
    // named first, the ten reserve plants stand ahead of the three route plants.
    let (taken, said) = canary.record(
        &clone,
        "run-set",
        &json!({"order": ["behaviour seed", "route seed"]}),
    );
    assert!(taken, "{said}");
    assert_eq!(
        ranked(&canary.state(&clone)),
        [reserve, route].concat(),
        "the queue is ranked by the order"
    );
}

#[test]
fn its_door_is_the_review_rows_and_the_plants() {
    let canary = Canary::parts("parts-door");
    let line = canary.ready("canary", &[PARTS]);
    let clone = canary.root("canary").join("clone");
    let state = canary.state(&clone);

    // ONE DOOR, `jigc setup`: the line's, the review row's, and every plant's.
    assert_eq!(line["door"], PARTS_DOOR, "{line}");
    let items = state["items"].as_array().expect("the items");
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
    let expected: Vec<(String, String)> = PARTS_ITEMS
        .iter()
        .map(|(item, kind)| ((*item).to_owned(), (*kind).to_owned()))
        .collect();
    assert_eq!(listed(&line["items"]), expected, "{line}");
    assert_eq!(listed(&state["items"]), expected, "{state}");
    let item = |id: &str| -> &Value {
        items
            .iter()
            .find(|i| i["item"] == id)
            .unwrap_or_else(|| panic!("the item `{id}`"))
    };
    assert_eq!(
        json!([
            item("row-setup")["runs"],
            item("row-setup")["doors"],
            item("row-setup")["registries"],
            item("row-setup")["clause"]
        ]),
        json!(["in-scope", [PARTS_DOOR], [], "no-lost-files"]),
        "{state}"
    );
    assert!(
        item("row-setup")["brief"]
            .as_str()
            .is_some_and(|brief| brief.contains("`jigc setup`") && !brief.contains("doc list")),
        "the review row's brief is of its door: {state}"
    );
    for row in state["ledger"].as_array().expect("the ledger") {
        assert_eq!(row["door"], PARTS_DOOR, "{row}");
    }
    // The three held checks run on every candidate and judge the second clause; THE SECOND
    // REGRESSION ITEM is the first once more — the same held command, over the same two
    // commits — under an id of its own.
    for (id, _) in &PARTS_ITEMS[1..] {
        assert_eq!(
            json!([item(id)["runs"], item(id)["clause"]]),
            json!(["every-candidate", "working-product"]),
            "`{id}`: {state}"
        );
    }
    for id in ["regression-set", "regression-again"] {
        assert!(
            item(id)["brief"]
                .as_str()
                .is_some_and(|brief| brief.contains("`hold-start --kind regression`")
                    && brief.contains(&canary.previous)
                    && !brief.contains("dev/regression-set run")),
            "`{id}` is the held regression set over the previous release's commit: {state}"
        );
    }
    // NO SECOND DOOR, NO ROUTE HUNT, NO TRIAL ARM, NO SCRIPTED ARM: the kinds are the review
    // row's and the held checks', and the small canary's door is nowhere in the run.
    let kinds: Vec<&str> = items
        .iter()
        .map(|i| i["kind"].as_str().expect("a kind"))
        .collect();
    assert_eq!(
        kinds,
        [
            "review-row",
            "held-gate",
            "held-regression",
            "held-regression"
        ],
        "{state}"
    );
    let opening =
        fs::read_to_string(clone.join(RUN_DIR).join("opening.md")).expect("the opening record");
    for text in [
        &state["items"].to_string(),
        &state["ledger"].to_string(),
        &opening,
    ] {
        assert!(!text.contains(DOOR), "the small canary's door: {text}");
    }
    assert!(opening.contains("`jigc setup`"), "{opening}");

    // THE ITEMS ARE THE CALLER'S TO NAME, from the switch's own table: the root the plan
    // opens for its second part after a repair leaves the two regression items out — and an
    // item of the small canary's table is none of this opening's.
    let line = canary.ready("part-two", &[PARTS, "--items", "gate,row-setup"]);
    assert_eq!(
        line["items"],
        json!([{"item": "row-setup", "kind": "review-row"}, {"item": "gate", "kind": "held-gate"}]),
        "{line}"
    );
    for bad in [
        "row-doc-list",
        "gate,arm-control",
        "regression-again,regression-again",
        "",
    ] {
        let root = canary.root("refused").display().to_string();
        canary
            .tool(&canary.setup_args(&root, RUN, &[PARTS, "--items", bad]))
            .refused("usage");
        assert!(
            !canary.root("refused").exists(),
            "nothing was made for `{bad}`"
        );
    }
    // And without the switch the switch's items are none of the small canary's.
    let root = canary.root("refused").display().to_string();
    canary
        .tool(&canary.setup_args(&root, RUN, &["--items", "row-setup"]))
        .refused("usage");
}

/// A record script that hands every call to the real one beside it — but for the ledger's
/// rows, from which it drops the grade: the plants it adds are rows nobody graded.
const UNGRADING_RECORD: &str = r#"#!/usr/bin/env python3
import json, os, subprocess, sys
real = os.path.join(os.path.dirname(os.path.realpath(__file__)), "stabilize-record.real")
fed = sys.stdin.buffer.read()
if sys.argv[1:2] == ["ledger-add"]:
    rows = json.loads(fed)
    for row in rows:
        row.pop("grade", None)
        row.pop("graded_by", None)
    fed = json.dumps(rows).encode()
sys.exit(subprocess.run([sys.executable, real] + sys.argv[1:], input=fed).returncode)
"#;

#[test]
fn under_the_switch_a_flag_outside_its_grammar_a_log_without_the_one_entry_and_a_plant_nobody_graded_are_refused()
 {
    let mut canary = Canary::new("parts-refuses");
    let unmade = |canary: &Canary, name: &str| {
        assert!(
            !canary.root(name).exists(),
            "a refused setup makes nothing: `{name}` is there"
        );
    };

    // THE LOG OF THE COMMIT NAMED HOLDS THE ENTRY THAT STATES THE EXIT RULE EXACTLY ONCE, or
    // the commit is unfit for this opening — asked before anything is made. Here it holds
    // none …
    canary.setup(&canary.root("l0"), &[PARTS]).refused("unfit");
    unmade(&canary, "l0");
    // … and the same commit is fit for the small canary, which names no entry.
    canary
        .setup(&canary.root("small"), &[])
        .done("setup", "ready");
    // … here an entry only speaks of it, which is not the entry.
    canary.log(&["2026-10-05 — What follows from the exit rule, revised"]);
    canary.setup(&canary.root("l1"), &[PARTS]).refused("unfit");
    unmade(&canary, "l1");

    // From here on the log holds the one entry.
    canary.log(&[EXIT_RULE]);
    // … AND A COMMIT WITHOUT THE WRITE GUARD, ITS FENCE AND ITS PROBE IS UNFIT TOO: a root of
    // the parts runs the probe first.
    let told = canary.setup(&canary.root("g0"), &[PARTS]);
    let refused = told.refused("unfit");
    assert!(
        GUARD_NEEDS.iter().all(|tool| refused["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(tool))),
        "it names what the commit lacks: {refused}"
    );
    unmade(&canary, "g0");
    canary.guarded();
    let setup = |at: &str, more: &[&str]| canary.setup(&canary.root(at), more);

    // A BUDGET IS A POSITIVE WHOLE NUMBER, AND AN ORDER A LIST OF CLASSES, each one plain
    // word `<kind>:<origin>` and none twice.
    for budget in ["0", "-1", "nine", "1.5", "", "09"] {
        setup("b", &[PARTS, "--budget", budget]).refused("usage");
    }
    for order in [
        "",
        "route",
        "route seed",
        "route:seed,route:seed",
        "route:seed,",
        "Route:seed",
        "route:seed:hunt",
    ] {
        setup("o", &[PARTS, "--order", order]).refused("usage");
    }
    unmade(&canary, "b");
    unmade(&canary, "o");
    // A CLASS THE RECORD SCRIPT DOES NOT KNOW is that script's to refuse, in its own words:
    // the setup fails at its write, and the root holds no canary.
    let told = setup("o2", &[PARTS, "--order", "route:nowhere"]);
    let refused = told.refused("tool");
    assert!(
        refused["halt"]["evidence"]
            .as_str()
            .is_some_and(|evidence| evidence.contains("stabilize-record: refused bad-value")),
        "{refused}"
    );
    assert!(!canary.root("o2").join("canary.json").exists());

    // WITHOUT THE SWITCH NEITHER FLAG IS TAKEN: the small canary's run has no queue.
    setup("n1", &["--budget", "9"]).refused("usage");
    setup("n2", &["--order", ORDER_THREE_FLAG]).refused("usage");
    unmade(&canary, "n1");
    unmade(&canary, "n2");
    // And the switch takes no value.
    setup("n3", &["--parts=yes"]).refused("usage");
    unmade(&canary, "n3");

    // A PLANT NOBODY GRADED IS NO OPENING OF THE PARTS: a triage would be handed it, and
    // would decide whether it is launched. Here the commit's record script is wrapped, and
    // the wrapper drops every row's grade — the run is ready, its next step is `test`, and
    // the setup refuses it all the same.
    let script = fs::read(canary.rig.root.join(RECORD)).expect("read the record script");
    placed_executable::write(&canary.rig.root.join(format!("{RECORD}.real")), script);
    placed_executable::write(&canary.rig.root.join(RECORD), UNGRADING_RECORD);
    canary.commit = canary
        .rig
        .commit("chore: a record script that grades no plant");
    let told = canary.setup(&canary.root("g1"), &[PARTS]);
    let refused = told.refused("not-fresh");
    assert!(
        refused["halt"]["evidence"]
            .as_str()
            .is_some_and(
                |evidence| evidence.contains("setup-hook-link-route-deletes-the-script")
                    && evidence.contains("ungraded")
            ),
        "the refusal names the rows nobody graded: {refused}"
    );
    assert!(!canary.root("g1").join("canary.json").exists());
    // The same commit opens the small canary, whose one seeded row nobody is to grade.
    canary.setup(&canary.root("g2"), &[]).done("setup", "ready");

    // AND TWO ENTRIES THAT OPEN AS THE ONE DOES ARE NOT ONE: the run would name neither.
    canary.log(&["2026-10-04 — The exit rule, revised once more: a second entry"]);
    canary.setup(&canary.root("l2"), &[PARTS]).refused("unfit");
    unmade(&canary, "l2");
}

/// What a setup WITHOUT the switch made, as the tool made it before the switch existed.
const GOLDEN: &str = "tooling-tests/fixtures/stabilize-canary-setup.golden";

/// Everything a setup under `root` made that is the tool's own to decide, as one text: the
/// line it printed, the file it keeps, the opening's commit — its subject and its paths —
/// and every file of the run's directory, byte for byte. What differs from one rig to the
/// next is named by what it is: the root, the three commits. The line's own hash, which is
/// over those, is left out.
fn made_by(canary: &Canary, root: &Path, raw: &str, line: &Value) -> String {
    let clone = root.join("clone");
    let named = [
        (root.display().to_string(), "<root>"),
        (line["head"].as_str().expect("a sha").to_owned(), "<head>"),
        (canary.commit.clone(), "<candidate>"),
        (canary.previous.clone(), "<previous>"),
    ];
    let plain = |text: &str| {
        named.iter().fold(text.to_owned(), |text, (value, name)| {
            text.replace(value, name)
        })
    };
    let printed = raw
        .rfind(", \"sha256\": \"")
        .map(|at| &raw[..at])
        .expect("the line ends with its sha256");
    let mut made = format!("== the line, without its sha256\n{}\n", plain(printed));
    made.push_str(&format!(
        "== canary.json\n{}",
        plain(&fs::read_to_string(root.join("canary.json")).expect("the kept line"))
    ));
    made.push_str(&format!(
        "== the opening's commit\n{}\n",
        canary.git(&clone, &["log", "-1", "--format=%an <%ae>%n%s%n%b"])
    ));
    let paths = canary.git(&clone, &["diff", "--name-only", "HEAD^", "HEAD"]);
    for path in paths.lines() {
        made.push_str(&format!(
            "== {path}\n{}",
            plain(&fs::read_to_string(clone.join(path)).expect("a file of the opening"))
        ));
    }
    made
}

/// **Without the switch the setup is what it was, to the byte.** The fixture is what the
/// tool made of this suite's source before it had a switch — captured from that tool, at
/// the commit this arm was written on — and a setup made today is held to it whole: the
/// line and its order of fields, the kept file, the commit, and every file the record
/// script wrote for it. A switch that leaks into the small canary's opening — a fact, a
/// row, a field of the line — is red here.
#[test]
fn a_setup_without_the_switch_is_what_it_was() {
    let canary = Canary::new("unswitched");
    let root = canary.root("canary");
    let told = canary.setup(&root, &[]);
    let line = told.done("setup", "ready").clone();
    let made = made_by(&canary, &root, &told.raw, &line);
    let golden = fs::read_to_string(repo_root().join(GOLDEN)).expect("read the golden");
    assert_eq!(
        made, golden,
        "a setup without the switch is not what it was — the left is what the tool makes now"
    );
    // AND WHAT IS HELD IS A WHOLE OPENING, no excerpt of one: the line, the kept file, the
    // commit and the five files a small canary's opening is.
    assert_eq!(golden.matches("\n== ").count() + 1, 8, "{golden}");
    // The same source under the switch is another opening — the control that the fixture
    // can tell the two apart.
    let switched = Canary::parts("switched");
    let root = switched.root("canary");
    let told = switched.setup(&root, &[PARTS]);
    let line = told.done("setup", "ready").clone();
    assert_ne!(made_by(&switched, &root, &told.raw, &line), golden);
}

// --- under the switch: the acts of a run in parts, and what `check` reads of them -----

/// The acts of a run in parts, in the order the human makes them — the plan's §5: each is one
/// invocation of the harness, and has a prompt file and a return file of its own.
const ACTS: [&str; 9] = [
    "part-1",
    "part-2",
    "part-3-act-1",
    "part-3-act-2",
    "part-4-act-1",
    "part-4-act-2",
    "part-4-act-3",
    "part-4-act-4",
    "part-4-act-5",
];

/// **The argument object of each act, typed here a second time from the plan's §5** — so
/// that an act whose arguments moved in the tool is red: the stage with the tuning stop
/// `preflight`; the stage; the order part 3 rules; the stage, which continues; the budget
/// raised by five; the stage, which the human ends; a look; the stage; and the same once more.
fn act_args(act: &str, scratch: &str) -> Value {
    let mut args = json!({"stage": "test", "run": RUN, "scratch": scratch});
    let more = match act {
        "part-1" => json!({"scope": {"doors": [PARTS_DOOR]}, "stopAfter": "preflight"}),
        "part-2" => json!({"scope": {"doors": [PARTS_DOOR]}}),
        "part-3-act-1" => json!({"rulings": [{"order": ORDER_THREE}]}),
        "part-4-act-1" => json!({"rulings": [{"budget": BUDGET + 5}]}),
        "part-4-act-3" => json!({"stopAfter": "state"}),
        "part-3-act-2" | "part-4-act-2" | "part-4-act-4" | "part-4-act-5" => json!({}),
        other => panic!("`{other}` is no act of the parts"),
    };
    for (name, value) in more.as_object().expect("an object") {
        args[name] = value.clone();
    }
    args
}

/// The act `id` of a line.
fn act_of<'a>(line: &'a Value, id: &str) -> &'a Value {
    line["acts"]
        .as_array()
        .unwrap_or_else(|| panic!("the line names the acts: {line}"))
        .iter()
        .find(|act| act["act"] == id)
        .unwrap_or_else(|| panic!("the line names the act `{id}`: {line}"))
}

/// WHAT AN ACT'S PROMPT FILE TELLS THE SESSION, READ OUT OF THE FILE — never out of the
/// line: the harness's path, the ONE argument object, and the file the return is written
/// to. The prompt file is what the human pastes, so it is what the suite runs.
struct Prompted {
    text: String,
    harness: String,
    args: Value,
    returns: PathBuf,
}

fn prompted(act: &Value) -> Prompted {
    let path = act["prompt"].as_str().expect("an act's prompt file");
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let one = |what: &str, found: Vec<&str>| -> String {
        assert_eq!(
            found.len(),
            1,
            "the prompt file {path} holds ONE {what}: {found:?}\n{text}"
        );
        found[0].to_owned()
    };
    let harness = one(
        "path of the harness, on a line of its own",
        text.lines()
            .filter(|line| line.starts_with('/') && line.ends_with(HARNESS))
            .collect(),
    );
    let returns = one(
        "return file, on a line of its own",
        text.lines()
            .filter(|line| line.starts_with('/') && line.ends_with(".json"))
            .collect(),
    );
    let object = one(
        "argument object, on a line of its own",
        text.lines().filter(|line| line.starts_with('{')).collect(),
    );
    Prompted {
        harness,
        args: serde_json::from_str(&object)
            .unwrap_or_else(|e| panic!("the argument object of {path} is JSON ({e}): {object}")),
        returns: PathBuf::from(returns),
        text,
    }
}

impl Canary {
    /// ONE ACT, RUN AS ITS PROMPT FILE SAYS: the harness by the path the file names, from
    /// the directory the line has the session stand in, with the argument object the file
    /// holds. `script` is what the scripted agents say.
    fn act(&self, line: &Value, id: &str, script: Value, n: u32) -> Ran {
        let told = prompted(act_of(line, id));
        self.invoke_by(
            &told.harness,
            line["session_cwd"]
                .as_str()
                .expect("the session's directory"),
            &told.args,
            script,
            n,
        )
    }

    /// `check`, as the human runs it before and after every act: its line.
    fn checked(&self, root: &Path) -> Value {
        self.tool(&["check", "--root", &root.display().to_string()])
            .done("check", "checked")
            .clone()
    }
}

/// What the session does with a return, as the act's prompt file tells it: written, as
/// JSON, to the file that prompt names.
fn written(line: &Value, id: &str, ran: &Ran) {
    fs::write(prompted(act_of(line, id)).returns, ran.result.to_string())
        .expect("write the act's return");
}

/// What `check` says of the act `id`.
fn said_of<'a>(read: &'a Value, id: &str) -> &'a Value {
    read["acts"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` says of each act what the run shows: {read}"))
        .iter()
        .find(|act| act["act"] == id)
        .unwrap_or_else(|| panic!("`check` names the act `{id}`: {read}"))
}

/// How every act stands, by its id, as `check` says it.
fn standing(read: &Value) -> Vec<(String, String)> {
    read["acts"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` says of each act what the run shows: {read}"))
        .iter()
        .map(|act| {
            (
                act["act"].as_str().expect("an act's id").to_owned(),
                act["stands"].as_str().expect("how it stands").to_owned(),
            )
        })
        .collect()
}

/// `(the act, how it stands)` for every act: `ran` are the acts whose return is written
/// and which the run shows; every other act is as an act that never ran — and the two the
/// run cannot show, a look and the record an ended act left, are said to be that.
fn stands_with(known: &[(&str, &str)]) -> Vec<(String, String)> {
    ACTS.iter()
        .map(|act| {
            let word = known
                .iter()
                .find(|(id, _)| id == act)
                .map(|(_, word)| *word)
                .unwrap_or(match *act {
                    "part-4-act-3" | "part-4-act-5" => "cannot-tell",
                    _ => "never-ran",
                });
            ((*act).to_owned(), word.to_owned())
        })
        .collect()
}

/// The scope the round's scope step resolves in every act: the one door, inside.
fn parts_scope() -> Value {
    json!({"included": [{"door": PARTS_DOOR, "registry": "verbs", "derivation": "named by the invocation"}],
           "excluded": [], "uncovered": [], "reached_but_excluded": [], "left_open": []})
}

/// A verifier that files: the members of its filing.
fn files(members: Value) -> Value {
    json!({"files": members})
}

/// A filing that refutes its finding as one that does not reproduce.
fn refutes() -> Value {
    files(json!({"verdict": "refuted", "refutation": "does-not-reproduce"}))
}

/// An item a verifier hands on at the round's one door: a route item, with the line it saw.
fn handed(proposal: &str, line: &str) -> Value {
    json!({"key": proposal, "doctype": "jigc-feedback", "door": PARTS_DOOR,
           "clause": "working-product", "repro": "the verifier's report, its own heading",
           "kind": "route", "route": line})
}

/// The sha256 of a file's bytes.
fn sha256_at(path: &Path) -> String {
    sha256(&fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display())))
}

/// The filed verdicts as `check` prints them: `(the key, the file's sha256, whether it
/// counts, whether a record took it)`, in the state's order.
fn filed(read: &Value) -> Vec<(String, String, bool, bool)> {
    read["filed"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` prints what is filed: {read}"))
        .iter()
        .map(|entry| {
            (
                entry["key"].as_str().expect("a key").to_owned(),
                entry["sha256"].as_str().expect("a hash").to_owned(),
                entry["counts"].as_bool().expect("whether it counts"),
                entry["recorded"]
                    .as_bool()
                    .expect("whether a record took it"),
            )
        })
        .collect()
}

/// The three keys of the plan's route plants and the ten of its reserve plants, in the
/// ledger's order — which is the queue's rank within a class.
fn route_and_reserve() -> (Vec<&'static str>, Vec<&'static str>) {
    (plants_of("route seed"), plants_of("behaviour seed"))
}

/// The six attempts of the write guard's probe, as its agent returns them where the guard
/// held: the file tool's write refused by the hook; the three writes of the shell refused by
/// the fence, each with its banner; the scratch file written, fenced; and the plain call of
/// the record script run with no fence.
fn guard_held() -> Value {
    let fenced = |attempt: &str| json!({"attempt": attempt, "exit": 1, "refused_by": "fence", "banner": true, "said": "stabilize-fence: under the write fence of the clone"});
    json!([
        {"attempt": "write", "exit": -1, "refused_by": "hook", "banner": false, "said": "PreToolUse:Write hook error: stabilize-guard: refused repository: ..."},
        fenced("echo"),
        fenced("commit"),
        fenced("config"),
        {"attempt": "scratch", "exit": 0, "refused_by": "none", "banner": true, "said": "stabilize-fence: under the write fence of the clone"},
        {"attempt": "record", "exit": 0, "refused_by": "none", "banner": false, "said": "{\"run\": \"canary-parts\"}"},
    ])
}

/// **The write guard's probe is made first, and `check` reads the clone, not the agent**
/// (DECISIONS.md → *2026-10-16 — The write guard of a stabilization run*; the plan's §5,
/// part 0). Under the switch the setup names act `part-0` apart from the acts (`guard`): the
/// probe's script by its path in the clone, its argument object, its prompt file, its
/// return file, ONE agent and no tool step, what the human does around it, and the clone's
/// snapshot at the setup. Its script, run under the runtime's stand-in, launches that one
/// agent — of a type the guard fences — and returns what it said. And `check` says what the
/// clone shows: where the guard held, nothing landed, the snapshot is the same and `fenced`
/// is true; WHERE THE HOOK DID NOT FIRE, the probe's writes land — and `check` names each,
/// whatever the agent returned, and `fenced` is false.
#[test]
fn the_write_guards_probe_is_made_first_and_check_reads_the_clone_not_the_agent() {
    let canary = Canary::parts("parts-guard");
    let line = canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let clone = root.join("clone");
    let scratch = root.join("scratch");

    // (1) THE LINE NAMES THE PROBE, apart from the acts.
    let guard = &line["guard"];
    let script = clone.join(GUARD_SCRIPT);
    assert!(script.is_file(), "the clone holds the probe's script");
    let args = json!({"run": RUN, "clone": clone, "scratch": scratch});
    assert_eq!(
        json!([
            guard["act"],
            guard["script"],
            guard["args"],
            guard["prompt"],
            guard["return"],
            guard["expect"]
        ]),
        json!(["part-0", script, args, root.join("prompts/part-0.md"), root.join("results/part-0.json"), {"agents": 1, "tool_steps": 0}]),
        "{guard}"
    );
    assert!(
        guard["human"]
            .as_str()
            .is_some_and(|said| said.starts_with("FIRST, on a fresh root")),
        "{guard}"
    );
    let snapshot: Vec<&str> = guard["snapshot"]
        .as_object()
        .expect("the clone's snapshot")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        snapshot,
        ["config", "head", "hooks", "reflog", "refs", "status"]
    );
    assert!(
        line["acts"]
            .as_array()
            .expect("the acts")
            .iter()
            .all(|act| act["act"] != "part-0"),
        "the probe is no act of the harness"
    );
    // Its prompt file: the script's path, the argument object and the return file, each on
    // a line of its own — as every act's.
    let text = fs::read_to_string(root.join("prompts/part-0.md")).expect("the probe's prompt file");
    let own_line = |wanted: String| text.lines().filter(|line| *line == wanted).count() == 1;
    let objects: Vec<Value> = text
        .lines()
        .filter(|line| line.starts_with('{'))
        .map(|line| serde_json::from_str(line).expect("the argument object is JSON"))
        .collect();
    assert!(
        own_line(script.display().to_string())
            && objects == [args.clone()]
            && own_line(root.join("results/part-0.json").display().to_string()),
        "{text}"
    );

    // (2) RIGHT AFTER THE SETUP: nothing returned, nothing landed, the snapshot the same.
    let read = canary.checked(&root);
    let refused = json!({"write": "refused", "echo": "refused", "commit": "refused", "config": "refused", "scratch": "absent"});
    assert_eq!(
        read["guard"],
        json!({"act": "part-0", "returned": false, "status": null, "own": null, "attempts": null,
               "disk": refused, "landed": [], "snapshot": {"same": true, "moved": []}, "fenced": null}),
        "{}",
        read["guard"]
    );

    // (3) THE PROBE'S SCRIPT, run as its prompt file says, under the runtime's stand-in:
    // ONE agent, of a type the guard fences, and its return passed on.
    let returned = if node_or_skip(&format!("the probe {GUARD_SCRIPT} was not run")) {
        let ran = canary.invoke_by(
            &script.display().to_string(),
            &clone.display().to_string(),
            &args,
            json!({"agents": {"guard-probe": {"attempts": guard_held()}}}),
            1,
        );
        assert_eq!(ran.agents, ["guard-probe"], "one agent: {:?}", ran.agents);
        assert_eq!(
            json!([
                ran.result["status"],
                ran.result["agent_type"],
                ran.result["attempts"]
            ]),
            json!(["probed", "milestone-e2e-tester", guard_held()])
        );
        let (_, prompt) = &ran.prompts[0];
        assert!(
            prompt.contains(&clone.display().to_string())
                && prompt.contains(&format!("`dev/stabilize-record state --run {RUN}`")),
            "the prompt aims at the clone, and spells the plain call: {prompt}"
        );
        // An argument object that is not the three is refused, and launches nobody.
        let bad = canary.invoke_by(
            &script.display().to_string(),
            &clone.display().to_string(),
            &json!({"run": RUN, "clone": "relative/path", "scratch": scratch}),
            json!({}),
            2,
        );
        assert_eq!(
            (bad.result["status"].clone(), bad.agents.len()),
            (json!("refused"), 0)
        );
        ran.result
    } else {
        json!({"status": "probed", "agent_type": "milestone-e2e-tester", "attempts": guard_held()})
    };
    // The session writes the return; the agent's scratch file is where it wrote it.
    fs::write(root.join("results/part-0.json"), returned.to_string()).expect("the probe's return");
    let mine = scratch.join("guard.probe1");
    fs::create_dir_all(&mine).expect("the agent's own directory");
    fs::write(mine.join("guard-probe-scratch.txt"), "probe\n").expect("the scratch file");
    let read = canary.checked(&root);
    let mut written = refused.clone();
    written["scratch"] = json!("written");
    assert_eq!(
        json!([
            read["guard"]["returned"],
            read["guard"]["status"],
            read["guard"]["own"],
            read["guard"]["attempts"],
            read["guard"]["disk"],
            read["guard"]["landed"],
            read["guard"]["snapshot"],
            read["guard"]["fenced"]
        ]),
        json!([true, "probed", true, guard_held(), written, [], {"same": true, "moved": []}, true]),
        "where the guard held: {}",
        read["guard"]
    );

    // (4) A HOOK THAT DID NOT FIRE: the probe's writes land in the clone — and `check` names
    // each from the clone, though the return on file says every one was refused.
    fs::write(clone.join("guard-probe-echo.txt"), "probe\n").expect("a write that landed");
    let git = |argv: &[&str]| {
        let out = Command::new("git")
            .arg("-C")
            .arg(&clone)
            .args(["-c", "user.name=probe", "-c", "user.email=probe"])
            .args(argv)
            .output()
            .expect("git runs");
        assert!(
            out.status.success(),
            "git {argv:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["config", "--local", "guard.probe", "1"]);
    git(&["commit", "--allow-empty", "-q", "-m", "guard-probe"]);
    git(&["reset", "-q", "--soft", "HEAD~1"]);
    let read = canary.checked(&root);
    let guard = &read["guard"];
    assert_eq!(
        json!([guard["disk"], guard["landed"], guard["fenced"]]),
        json!([{"write": "refused", "echo": "landed", "commit": "landed", "config": "landed", "scratch": "written"}, ["echo", "commit", "config"], false]),
        "{guard}"
    );
    assert_eq!(guard["snapshot"]["same"], false, "{guard}");
    for moved in ["config", "reflog", "status"] {
        assert!(
            guard["snapshot"]["moved"]
                .as_array()
                .expect("what moved")
                .contains(&json!(moved)),
            "`{moved}` moved: {guard}"
        );
    }
}

/// **A prompt file holds the arguments its line names, and nothing a shell reads** (the
/// plan's §5 and §10, `SC3`; its §12: *a prompt lifted from the setup's line of JSON — a text
/// with escapes, pasted by hand* — rejected). Under the switch the setup writes ONE PROMPT
/// FILE FOR EACH ACT under `<root>/prompts/` and names it in its line beside the act's
/// argument object and its return file under `<root>/results/`. The file's text says one
/// thing — invoke the Workflow tool once, by the script's path, with these arguments; write
/// what it returned to this file; do nothing else — and it is what the human pastes: the
/// harness's path, the return file and the ONE argument object each stand on a line of their
/// own, the object as the line has it, with no escape in it. NOTHING IN IT IS A COMMAND, a
/// pipe, a redirect or a substitution; none names a planted row; and without the switch no
/// such file is made.
#[test]
fn a_prompt_file_holds_the_arguments_its_line_names_and_nothing_a_shell_reads() {
    let canary = Canary::parts("parts-prompts");
    let line = canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let at = |name: &str| root.join(name).display().to_string();

    // (1) THE ACTS, in the order the human makes them: each with its argument object — the
    // plan's, typed here a second time — its prompt file and its return file.
    let acts = line["acts"]
        .as_array()
        .unwrap_or_else(|| panic!("the line names the acts: {line}"));
    assert_eq!(
        acts.iter().map(|act| &act["act"]).collect::<Vec<_>>(),
        ACTS,
        "{line}"
    );
    assert_eq!(line["prompts"], json!(at("prompts")), "{line}");
    // … AND, APART FROM THE ACTS, THE RULINGS OFF THE MAIN LINE (the review before part 1:
    // the ruling that grants one more attempt had no prompt file, and its argument object
    // was the human's to type): each under a name no act has, with its argument object —
    // the plan's, typed here a second time — its prompt file and its return file, and in
    // words when it is the step. No count: nobody counted an invocation off the line.
    let rulings = line["rulings"]
        .as_array()
        .unwrap_or_else(|| panic!("the line names the rulings off the main line: {line}"));
    assert_eq!(
        rulings
            .iter()
            .map(|ruling| json!([
                ruling["ruling"],
                ruling["args"]["rulings"],
                ruling["expect"]
            ]))
            .collect::<Vec<_>>(),
        [
            json!(["ruling-again", [{"again": "test"}], null]),
            json!(["ruling-raise-again", [{"budget": BUDGET + 10}], null])
        ],
        "{line}"
    );
    for ruling in rulings {
        let id = ruling["ruling"].as_str().expect("a ruling's name");
        assert!(
            id.starts_with("ruling-") && !ACTS.contains(&id) && !id.starts_with("part-"),
            "`{id}` is no act's name"
        );
        assert_eq!(
            ruling["args"],
            json!({"stage": "test", "run": RUN, "scratch": at("scratch"),
                   "rulings": ruling["args"]["rulings"]}),
            "a ruling names the stage, the run, the scratch root and what is ruled: {ruling}"
        );
        let when = ruling["when"].as_str().expect("when it is the step");
        assert!(
            when.contains("OFF THE MAIN LINE")
                && when.contains("only where check names this file under ruling")
                && when.contains("NEVER WHILE A SESSION OF AN ACT IS OPEN"),
            "{when}"
        );
    }
    for act in acts.iter().chain(rulings) {
        let ruling = act["ruling"].as_str();
        let id = ruling.unwrap_or_else(|| act["act"].as_str().expect("an act's id"));
        if ruling.is_none() {
            assert_eq!(act["args"], act_args(id, &at("scratch")), "`{id}`: {act}");
        }
        assert_eq!(
            json!([act["prompt"], act["return"]]),
            json!([
                at(&format!("prompts/{id}.md")),
                at(&format!("results/{id}.json"))
            ]),
            "`{id}`: {act}"
        );

        // (2) THE FILE HOLDS WHAT THE LINE NAMES: the harness by its path in the clone, the
        // return file, and the argument object — one of each, each on a line of its own.
        let told = prompted(act);
        assert_eq!(
            json!([told.harness, told.args, told.returns.display().to_string()]),
            json!([line["script_path"], act["args"], act["return"]]),
            "`{id}`:\n{}",
            told.text
        );
        assert!(
            !told.returns.exists(),
            "no act has returned: `{id}` has no return file yet"
        );
        // IT SAYS THE ONE THING: the Workflow tool, once, by the script's path; the return
        // written to the file; and nothing else.
        for said in [
            "Workflow tool",
            "ONCE",
            "scriptPath",
            "args",
            "nothing else",
        ] {
            assert!(
                told.text.contains(said),
                "the prompt of `{id}` says `{said}`:\n{}",
                told.text
            );
        }
        // (3) AND NOTHING A SHELL READS: printable ASCII in plain lines, with no word a
        // shell composes with, no escape, no command of a tool — judged with the three
        // paths named by what they are, so that a path of this machine decides nothing.
        assert!(
            told.text
                .bytes()
                .all(|b| b == b'\n' || (0x20..0x7f).contains(&b)),
            "the prompt of `{id}` is printable ASCII:\n{}",
            told.text
        );
        let plain = told.text.replace(&root.display().to_string(), "<root>");
        for token in [
            "|", ">", "<", "&", ";", "$", "`", "\\", "*", "~", "sleep", "nohup", "dev/", "git ",
            "#!",
        ] {
            assert!(
                !plain.replace("<root>", "ROOT").contains(token),
                "the prompt of `{id}` holds `{token}`:\n{plain}"
            );
        }
        // No planted row is named in it, and nothing of the answer sheet.
        for plant in PLANTS {
            assert!(
                !plain.contains(plant.0),
                "the prompt of `{id}` names the planted row `{}`:\n{plain}",
                plant.0
            );
        }
        // (`under-confirmed` is a class of the order part 3 rules, and no answer.)
        for answer in ["plant", "refuted", "confirmed", "undrivable", "reserve"] {
            assert!(
                !plain.replace("under-confirmed", "").contains(answer),
                "the prompt of `{id}` says `{answer}`:\n{plain}"
            );
        }
    }

    // (4) THE PROMPTS' DIRECTORY HOLDS THOSE FILES AND NO OTHER, and the kept file names
    // the acts as the line does.
    let mut made: Vec<String> = fs::read_dir(root.join("prompts"))
        .expect("the prompts' directory")
        .map(|entry| {
            entry
                .expect("a prompt file")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    made.sort();
    let mut named: Vec<String> = ACTS
        .iter()
        .chain(&["ruling-again", "ruling-raise-again", "part-0"])
        .map(|act| format!("{act}.md"))
        .collect();
    named.sort();
    assert_eq!(made, named);
    let kept: Value =
        serde_json::from_str(&fs::read_to_string(root.join("canary.json")).expect("the kept line"))
            .expect("the kept line is JSON");
    assert_eq!(kept["acts"], line["acts"]);
    assert_eq!(kept["rulings"], line["rulings"]);
    assert_eq!(kept["guard"], line["guard"]);

    // (5) THE TWO ACTS THAT ONLY RULE NAME WHAT THE HUMAN RULES, as plain text the harness
    // takes: the order of part 3 — the classes as the record script holds them — and the
    // budget raised by five, so that five verdicts are left (four until 2026-10-17: part 3's
    // wave 2 is now the item the first row's verifier hands on, and the reserve row it took
    // moves to part 4).
    assert_eq!(
        json!([
            act_of(&line, "part-3-act-1")["args"]["rulings"],
            act_of(&line, "part-4-act-1")["args"]["rulings"]
        ]),
        json!([[{"order": ORDER_THREE}], [{"budget": 14}]]),
        "{line}"
    );
    // AS IT WAS GIVEN: a root opened with another budget is raised from that one.
    let other = canary.ready("budget-four", &[PARTS, "--budget", "4"]);
    assert_eq!(
        json!([
            act_of(&other, "part-4-act-1")["args"]["rulings"],
            other["rulings"][1]["args"]["rulings"]
        ]),
        json!([[{"budget": 9}], [{"budget": 14}]]),
        "{other}"
    );
    // AND A BUDGET SO LARGE THAT A RAISE WOULD BE A NUMBER THE HARNESS TAKES NONE OF is
    // refused at the setup, before anything is made (the review before part 1, a row): the
    // harness takes no number above 999999 in a ruling, part 4 raises by five and a missed
    // window by five more.
    let large = canary.setup(&canary.root("too-large"), &[PARTS, "--budget", "999990"]);
    assert!(
        large.refused("usage")["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains("--budget 999990 is too large")
                && cause.contains("at most 999989")),
        "{}",
        large.raw
    );
    assert!(!canary.root("too-large").exists());
    let largest = canary.ready("largest", &[PARTS, "--budget", "999989"]);
    assert_eq!(
        largest["rulings"][1]["args"]["rulings"],
        json!([{"budget": 999_999}]),
        "{largest}"
    );

    // HOW MANY AGENTS EACH ACT LAUNCHES WHERE EVERY AGENT ENDS WELL, as the line says it —
    // the numbers the acts' own arm holds to the clone's harness, typed here a second time:
    // `[agents, tool_steps, verifiers, ended]`. Two acts launch a verifier for each verdict
    // the budget has left, beside their steps; one is ended by the human; and NO TIME IS
    // PRINTED, which nobody measured.
    let counted = [
        ("part-1", json!([16, 14, null, null])),
        ("part-2", json!([23, 17, null, null])),
        ("part-3-act-1", json!([9, 8, null, null])),
        ("part-3-act-2", json!([24, 15, null, null])),
        ("part-4-act-1", json!([10, 9, null, null])),
        (
            "part-4-act-2",
            json!([13, 13, "budget.left", {"agents": 10, "tool_steps": 5}]),
        ),
        ("part-4-act-3", json!([2, 2, null, null])),
        ("part-4-act-4", json!([13, 13, "budget.left", null])),
        ("part-4-act-5", json!([6, 6, null, null])),
    ];
    for (id, numbers) in &counted {
        let expect = &act_of(&line, id)["expect"];
        assert_eq!(
            &json!([
                expect["agents"],
                expect["tool_steps"],
                expect["verifiers"],
                expect["ended"]
            ]),
            numbers,
            "`{id}`: {expect}"
        );
    }
    let said = line["acts"].to_string();
    for time in ["minutes", "seconds", "hours", "wall"] {
        assert!(!said.contains(time), "an act names `{time}`: {said}");
    }
    // A ROOT OPENED WITH ANOTHER BUDGET IS COUNTED BY NOBODY: its acts say so — but for the
    // two whose count no opening moves, a look and the record an ended act left.
    for (id, numbers) in &counted {
        let expect = &act_of(&other, id)["expect"];
        if ["part-4-act-3", "part-4-act-5"].contains(id) {
            assert_eq!(
                &json!([expect["agents"], expect["tool_steps"], null, null]),
                numbers
            );
        } else {
            assert_eq!(expect, &Value::Null, "`{id}`: {other}");
        }
    }

    // … AND SO IS A ROOT OPENED WITH FEWER ITEMS, as the plan opens one for part 2 after a
    // repair: its stage is another stage.
    let fewer = canary.ready("fewer-items", &[PARTS, "--items", "row-setup,gate"]);
    assert_eq!(
        json!([
            act_of(&fewer, "part-1")["expect"],
            act_of(&fewer, "part-2")["expect"],
            act_of(&fewer, "part-4-act-3")["expect"]["agents"]
        ]),
        json!([null, null, 2]),
        "{fewer}"
    );

    // (6) WHAT THE HUMAN DOES AROUND AN ACT, IN WORDS, where it is more than pasting a
    // prompt: the act the human ends names the directory a verdict is filed in; and THE ACT
    // THAT FOLLOWS A RECORD ENDED AT ITS GATE HAS THE HUMAN WAIT FOR WHAT `check` SAYS OF
    // THE JOB — never for a verdict's file, which nobody writes while nobody asks.
    assert_eq!(
        act_of(&line, "part-4-act-2")["verdicts"],
        json!(at(&format!("clone/{RUN_DIR}/r1/verdicts"))),
        "{line}"
    );
    let waits = act_of(&line, "part-4-act-5")["human"]
        .as_str()
        .expect("what the human does before act 5");
    // THE WAIT IS KEEPABLE FOR EVERY STATE OF THAT JOB (the review before part 1): `done` ends
    // it; `running` is waited out; and of a job that is `dead` no other word will ever
    // come — the wait is over, and NOTHING THAT NAMES THE ROOT IS ENDED THERE: after act 4 a
    // process that names the root is the supervisor of a record's gate.
    assert!(
        waits.contains("OPTIONAL")
            && waits.contains("check, until held says of that record's job done - or dead")
            && waits.contains("While it says running the gate still runs, and this act waits")
            && waits.contains("its supervisor wrote exit.json, and held prints its exit")
            && waits.contains("No verdict.json appears while nobody asks after a job")
            && waits.contains("none is waited for")
            && waits.contains("gates the batch again under a name of its own")
            && waits.contains("dead: the job's supervisor is gone, no other word will ever come and the wait is over")
            && waits.contains("NOTHING THAT NAMES THE ROOT IS ENDED HERE")
            && !waits.contains("that it has ended"),
        "{waits}"
    );
    for id in ["part-1", "part-2", "part-3-act-1", "part-4-act-4"] {
        assert_eq!(act_of(&line, id)["human"], Value::Null, "`{id}`: {line}");
    }

    // (7) THE TOOL THAT RUNS IS THE TOOL THE COMMIT NAMED HOLDS — OR THE SETUP IS REFUSED (the
    // review before part 1, its first finding: the setup took a commit that lacked the
    // tool as it stood, printed `as_committed: false` in a line of thirteen thousand
    // characters and said `ready`; the clone then ran another harness than the line counted
    // for, and no `check` said so again). The line says which tool wrote the opening, and
    // `check` says of the tool what is so: what wrote the opening, what the clone's commit
    // holds at the tool's path, and what is asking now.
    let tool = canary.rig.root.join(TOOL);
    let wrote = sha256_at(&tool);
    assert_eq!(
        line["tool"],
        json!({"sha256": wrote, "as_committed": true}),
        "{line}"
    );
    assert_eq!(
        canary.checked(&root)["tool"],
        json!({"wrote": wrote, "commit": wrote, "reads": wrote, "same": true})
    );
    // AN EDIT NOBODY COMMITTED: the file that runs is in no commit. Refused, with nothing
    // made — the root is not there — and the refusal says what to do.
    let was = fs::read(&tool).expect("read the tool");
    let mut edited = was.clone();
    edited.extend_from_slice(b"# an edit nobody committed\n");
    fs::write(&tool, &edited).expect("edit the tool");
    let reads = sha256_at(&tool);
    assert_ne!(reads, wrote);
    let unfit = |told: &Told, advice: &str| {
        let refused = told.refused("unfit");
        let cause = refused["halt"]["root_cause"].as_str().expect("a cause");
        assert!(
            cause.contains("the tool that runs is not the tool the commit named")
                && cause.contains(TOOL),
            "{cause}"
        );
        let then = refused["halt"]["recommendation"]
            .as_str()
            .expect("a recommendation");
        assert!(
            then.starts_with("nothing was made: the root holds what it held. ")
                && then.contains(advice),
            "{then}"
        );
    };
    let uncommitted = canary.setup(&canary.root("uncommitted"), &[PARTS]);
    unfit(
        &uncommitted,
        "The file that runs is in no commit this checkout stands at: commit it and name that commit, or run the setup from a checkout of the commit named",
    );
    assert!(!canary.root("uncommitted").exists());
    // … and `check` of the root the first one wrote says that another tool asks now.
    assert_eq!(
        canary.checked(&root)["tool"],
        json!({"wrote": wrote, "commit": wrote, "reads": reads, "same": false})
    );
    // THE THREE ARE READ EACH FROM ITS OWN PLACE — what wrote the opening from the kept line,
    // what the clone's commit holds from the clone, what asks now from the file that runs —
    // so a root that a tool wrote which the commit does not hold (as the tool before this
    // repair made one) is said to be that: here the kept line names the edited tool.
    let kept_line = root.join("canary.json");
    let kept_was = fs::read_to_string(&kept_line).expect("the kept line");
    let mut bent: Value = serde_json::from_str(&kept_was).expect("the kept line is JSON");
    bent["tool"]["sha256"] = json!(reads);
    fs::write(&kept_line, bent.to_string()).expect("bend the kept line");
    assert_eq!(
        canary.checked(&root)["tool"],
        json!({"wrote": reads, "commit": wrote, "reads": reads, "same": false})
    );
    fs::write(&kept_line, kept_was).expect("put the kept line back");
    // THE EDIT COMMITTED, AND THE OLDER COMMIT NAMED — the state the plan's own words led to:
    // the tip holds the tool as it stands, the commit named holds the tool as it was. Refused
    // again, and the refusal names the commit to name: the one checked out. Named, it is
    // taken — and a root that a refused setup was handed is as usable as it was.
    let older = canary.commit.clone();
    let tip = canary.rig.commit("fix: the tool, repaired");
    let stale = canary.setup(&canary.root("stale"), &[PARTS]);
    unfit(
        &stale,
        &format!("Name the commit that is checked out where this tool lies: --commit {tip} "),
    );
    assert!(
        stale.line["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(&older)),
        "{}",
        stale.raw
    );
    assert!(!canary.root("stale").exists());
    let at_the_tip = canary.root("stale").display().to_string();
    let named = canary.tool(
        &canary
            .setup_args(&at_the_tip, RUN, &[PARTS])
            .iter()
            .map(|arg| if *arg == older { tip.as_str() } else { *arg })
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        named.done("setup", "ready")["tool"],
        json!({"sha256": reads, "as_committed": true}),
        "{}",
        named.raw
    );
    // WITHOUT THE SWITCH THE QUESTION IS NOT ASKED: the small canary's setup is what it was,
    // and takes the older commit from the tool as it stands.
    canary.ready("small-of-the-older", &[]);

    // (8) WITHOUT THE SWITCH NO PROMPT IS WRITTEN: the small canary's line names two
    // argument objects, and its root holds no such directory.
    let small = Canary::new("no-prompts");
    let line = small.ready("canary", &[]);
    assert_eq!(
        json!([line["acts"], line["rulings"], line["prompts"], line["tool"]]),
        json!([null, null, null, null]),
        "{line}"
    );
    assert!(!small.root("canary").join("prompts").exists());
}

/// **No planted row says which way its claim falls, nor that it was planted** (what the
/// builder of `SC3` was handed beside the plan: the keys the plan names — `plant-…-regression`,
/// `plant-false-…`, `plant-reserve-…` — tell a verifier, which is handed its key, the answer
/// before it drives anything). THE RULE IS HELD OVER WHAT THE TOOL PLANTED, row by row, as the
/// run's own state reads it: A KEY NAMES THE DOOR AND THE SUBJECT OF ITS CLAIM, in the
/// ledger's key grammar, as a hunter's finding would be keyed — it opens with the door's
/// word — and holds no word that says it is a plant or how it will fall; and no cell a
/// verifier reads of the row — its source, its repro, its kind's evidence — says either.
#[test]
fn no_planted_row_says_which_way_its_claim_falls_or_that_it_was_planted() {
    let canary = Canary::parts("parts-keys");
    canary.ready("canary", &[PARTS]);
    let state = canary.state(&canary.root("canary").join("clone"));
    let ledger = state["ledger"].as_array().expect("the ledger");
    assert_eq!(ledger.len(), 14, "{state}");
    for row in ledger {
        let key = row["key"].as_str().expect("a key");
        assert!(
            key.starts_with("setup-")
                && key.len() <= 60
                && key.split('-').all(|word| !word.is_empty()
                    && word
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())),
            "`{key}` names the door and its claim's subject, as a slug"
        );
        for word in [
            "plant",
            "planted",
            "seed",
            "seeded",
            "reserve",
            "spare",
            "false",
            "true",
            "regression",
            "canary",
            "lead",
            "refuted",
            "confirmed",
            "undrivable",
            "ballast",
            "claim",
        ] {
            assert!(
                key.split('-').all(|part| part != word),
                "the key `{key}` holds the word `{word}`"
            );
        }
        let told = json!([
            row["source"],
            row["repro"],
            row["class"]["evidence"],
            row["class"]["code"]
        ])
        .to_string();
        for answer in [
            "false",
            "refuted",
            "confirmed",
            "undrivable",
            "reserve",
            "ballast",
            "plant",
            "seeded",
            "stabilize-canary",
        ] {
            assert!(
                !told.contains(answer),
                "a cell of `{key}` says `{answer}`: {told}"
            );
        }
    }
    // The keys are fourteen, and no two alike.
    let mut keys: Vec<&str> = ledger
        .iter()
        .map(|row| row["key"].as_str().expect("a key"))
        .collect();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(keys.len(), 14);
}

/// The plan's name for each planted row, in the ledger's order: what the plan's tables call
/// the row the tool plants under a key that says nothing.
const PLAN_NAMES: [&str; 14] = [
    "plant-install-hook-regression",
    "plant-false-route-claim",
    "plant-linux-root-claim",
    "plant-prose-lead",
    "plant-reserve-1",
    "plant-reserve-2",
    "plant-reserve-3",
    "plant-reserve-4",
    "plant-reserve-5",
    "plant-reserve-6",
    "plant-reserve-7",
    "plant-reserve-8",
    "plant-reserve-9",
    "plant-reserve-10",
];

/// The plan's document, this tool and the log: where the answers are, one read away — and
/// this suite, which types them a second time.
const PLAN: &str = "completions/artifacts/M55/stabilization-build/reshape/staged-canary.md";
const SUITE: &str = "tooling-tests/dev_stabilize_canary.rs";

/// Every file below `dir`, as text where it is one: `(its path, its text)`.
fn texts_below(dir: &Path) -> Vec<(String, String)> {
    let mut found = BTreeMap::new();
    files_below(dir, "", &mut found);
    found
        .into_iter()
        .filter(|(name, _)| !name.ends_with('/') && !name.ends_with(" ->"))
        .map(|(name, bytes)| (name, String::from_utf8_lossy(&bytes).into_owned()))
        .collect()
}

/// **The answer sheet is `check`'s to print, and is in no file the setup writes.** Which
/// planted row is which of the plan's, and what a verifier should return for it, is printed
/// by `check` — run by the human, from the repository — and by nothing else: not the setup's
/// line, not the file it keeps, no prompt file, nothing the setup wrote into the clone's
/// run directory, nothing under the scratch root. And `check` names the ONE QUESTION a
/// part's reading has to answer about it — whether a verifier went and read the answers,
/// which lie in the candidate's own commit — as a question, with no detector behind it.
#[test]
fn the_answer_sheet_is_checks_to_print_and_is_in_no_file_the_setup_writes() {
    let canary = Canary::parts("parts-sheet");
    let told = canary.setup(&canary.root("canary"), &[PARTS]);
    let line = told.done("setup", "ready").clone();
    let root = canary.root("canary");
    let read = canary.checked(&root);

    // (1) THE SHEET: fourteen rows, in the ledger's order — the plan's name, the key that
    // is planted, the class, and what a verifier should return.
    let sheet = read["sheet"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` prints the answer sheet: {read}"));
    assert_eq!(
        sheet
            .iter()
            .map(|row| json!([row["plan"], row["key"], row["class"]]))
            .collect::<Vec<_>>(),
        PLAN_NAMES
            .iter()
            .zip(PLANTS)
            .map(|(plan, plant)| json!([plan, plant.0, plant.1]))
            .collect::<Vec<_>>(),
        "{read}"
    );
    let returns = |plan: &str| -> &str {
        sheet
            .iter()
            .find(|row| row["plan"] == plan)
            .and_then(|row| row["returns"].as_str())
            .unwrap_or_else(|| panic!("the sheet says what `{plan}` should return: {read}"))
    };
    assert!(
        returns("plant-install-hook-regression").starts_with("confirmed")
            && returns("plant-install-hook-regression").contains("regression")
            && returns("plant-false-route-claim").starts_with("refuted")
            && returns("plant-false-route-claim").contains("does-not-reproduce")
            && returns("plant-linux-root-claim").starts_with("undrivable")
            && returns("plant-prose-lead").starts_with("no verdict")
            && (1..=10).all(|n| returns(&format!("plant-reserve-{n}")).starts_with("refuted")),
        "{read}"
    );

    // (2) THE QUESTION FOR THE READING, named in the output. The clone holds the answer key
    // WITHHELD — every file of the commit named that names a planted row, here the tool alone
    // (`the_clones_tree_names_no_planted_row_beside_how_it_falls` holds the rest) — and its
    // history holds it whole: the question names each withheld file by its path, and asks
    // whether a verifier read one there. A second question, about the report a row cites, is
    // asked only where the clone holds that report withheld, which this rig's commit does not
    // hold. (The review before part 1: the question named three files; the third root: five,
    // in the tree every verifier works in.)
    let asked: Vec<&str> = read["to_answer"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` names what a part's reading has to answer: {read}"))
        .iter()
        .map(|question| question.as_str().expect("a question"))
        .collect();
    assert_eq!(asked.len(), 1, "{read}");
    assert_eq!(line["withheld"], json!([TOOL]), "{line}");
    assert!(
        asked[0].ends_with('?')
            && asked[0].contains(&format!("withheld - {TOOL} -"))
            && asked[0].contains("its history holds them whole")
            && asked[0].contains("did any verifier read one of them there"),
        "{read}"
    );
    // … THE CITED REPORT IS THE ONE THE ROW HANDS ITS VERIFIER, word for word, and it does
    // hold an answer — or the clone would withhold it for nothing.
    let cited = CITED;
    let state = canary.state(&root.join("clone"));
    let first = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .find(|row| row["key"] == PLANTS[0].0)
        .unwrap_or_else(|| panic!("the first planted row: {state}"));
    assert!(
        first["repro"]
            .as_str()
            .is_some_and(|repro| repro.ends_with(&format!("of {cited}."))),
        "{first}"
    );
    let report = fs::read_to_string(repo_root().join(cited)).expect("read the cited report");
    assert!(
        report.contains("`confirmed`") && report.contains("regression: true"),
        "the cited report states a verdict"
    );

    // (3) AND NOTHING THE SETUP WROTE HOLDS THE SHEET: its line, the file it keeps, the
    // prompt files, the run's directory in the clone, the scratch root — none names a row
    // as the plan does, and none says what a row should return.
    let mut written = vec![
        ("the line".to_owned(), told.raw.clone()),
        (
            "canary.json".to_owned(),
            fs::read_to_string(root.join("canary.json")).expect("the kept line"),
        ),
    ];
    for dir in ["prompts", "scratch", "results"] {
        written.extend(texts_below(&root.join(dir)));
    }
    written.extend(texts_below(&root.join("clone").join(RUN_DIR)));
    assert!(written.len() > 2 + ACTS.len(), "{}", written.len());
    for (name, text) in &written {
        for plan in ["plant-", "reserve", "the sheet", "should return"] {
            assert!(!text.contains(plan), "`{name}` says `{plan}`: {text}");
        }
        for row in sheet {
            let returns = row["returns"].as_str().expect("what it should return");
            assert!(
                !text.contains(returns),
                "`{name}` says what a row should return, `{returns}`: {text}"
            );
        }
    }
    assert_eq!(line["sheet"], Value::Null, "{line}");
}

/// The report the first planted row's repro cites, which states that row's verdict.
const CITED: &str = "completions/artifacts/M55/stabilization-build/canary-one/run/r1/reports/test/verify-p2-r1-setup-install-hook-route-other-causes-unexamined.a1.md";

/// How many lines apart a row's name and a phrase of how it falls may stand and still be
/// read together: a table row holds both on one line, a paragraph within a few.
const NEAR: usize = 6;

/// Every place in `texts` where a NAME of a planted row stands within [`NEAR`] lines of a
/// phrase that says how a row falls: `(the file, the line, the name, the phrase)`. Both are
/// matched without regard to case.
fn beside(
    texts: &[(String, String)],
    names: &[String],
    falls: &[String],
) -> Vec<(String, usize, String, String)> {
    let mut found = Vec::new();
    for (file, text) in texts {
        let lines: Vec<String> = text.lines().map(str::to_lowercase).collect();
        for (at, line) in lines.iter().enumerate() {
            for name in names {
                if !line.contains(&name.to_lowercase()) {
                    continue;
                }
                let window = &lines[at.saturating_sub(NEAR)..(at + NEAR + 1).min(lines.len())];
                if let Some(fall) = falls.iter().find(|fall| {
                    window
                        .iter()
                        .any(|near| near.contains(&fall.to_lowercase()))
                }) {
                    found.push((file.clone(), at + 1, name.clone(), fall.clone()));
                }
            }
        }
    }
    found
}

/// **The clone's tree names no planted row beside how it falls** (the staged canary's third
/// root: two verifiers saw the answer key's lines of the log through a broad grep — the
/// candidate is a commit of this repository, and the files that say which planted row is
/// which and how each must fall were in the tree every verifier works in). THE SETUP
/// WITHHOLDS THEM IN THE CLONE, by a commit of its own on the commit named, before the loop
/// branch is pushed: every file of that commit that names a planted row — by its key or by
/// the plan's name for it — and every file a planted row cites as its evidence. The log
/// keeps every heading, and the entry that states the exit rule whole; only the body of an
/// entry that names a row is withheld. Everything here is DERIVED: a row's names from the
/// sheet `check` prints and its claim from the row the clone's ledger holds; how a row falls
/// from every phrase of what the sheet says it should return; the files a row cites from
/// its repro. And the scan is shown to find what it looks for in the commit the setup was
/// handed, which holds the answer key as this repository does.
#[test]
fn the_clones_tree_names_no_planted_row_beside_how_it_falls() {
    let mut canary = Canary::new("parts-withheld");
    canary.guarded();
    // THE COMMIT NAMED HOLDS THE ANSWER KEY AS THIS REPOSITORY DOES: the log, the plan, this
    // suite and the report the first row cites — beside the tool, which the rig holds.
    for place in ["DECISIONS.md", PLAN, SUITE, CITED] {
        let text = fs::read_to_string(repo_root().join(place))
            .unwrap_or_else(|e| panic!("read `{place}`: {e}"));
        canary.rig.write(place, &text);
    }
    canary.commit = canary
        .rig
        .commit("docs: the answer key, as the repository holds it");
    let root = canary.root("canary");
    let line = canary.setup(&root, &[PARTS]).done("setup", "ready").clone();
    let read = canary.checked(&root);
    let clone = root.join("clone");
    let state = canary.state(&clone);

    // THE TERMS, derived. A row's names: its key and the plan's name, as the sheet prints
    // them, and its claim, as its row opens. How a row falls: every phrase of what the sheet
    // says a row should return, cut at its commas and colons.
    let sheet = read["sheet"].as_array().expect("`check` prints the sheet");
    let mut names: Vec<String> = Vec::new();
    let mut falls: Vec<String> = Vec::new();
    for row in sheet {
        for field in ["key", "plan"] {
            names.push(row[field].as_str().expect("a name").to_owned());
        }
        for phrase in row["returns"]
            .as_str()
            .expect("what a row should return")
            .split([',', ':'])
        {
            let phrase = phrase.trim();
            let phrase = phrase.strip_prefix("a ").unwrap_or(phrase);
            if !phrase.is_empty() && !falls.iter().any(|known| known == phrase) {
                falls.push(phrase.to_owned());
            }
        }
    }
    let rows: Vec<&Value> = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .filter(|row| sheet.iter().any(|planted| planted["key"] == row["key"]))
        .collect();
    assert_eq!(rows.len(), sheet.len(), "{state}");
    let mut cited: Vec<String> = Vec::new();
    for row in &rows {
        let repro = row["repro"].as_str().expect("a row's repro");
        let claim = repro
            .strip_prefix("The claim: ")
            .and_then(|rest| rest.split(". The state:").next())
            .map(|claim| claim.trim_end_matches('.'))
            .unwrap_or_else(|| panic!("a planted row opens with its claim: {repro}"));
        names.push(claim.to_owned());
        // THE FILES A ROW CITES: every path its repro names that the commit named holds.
        for word in repro.split([' ', '`', '(', ')']) {
            let path = word.trim_end_matches(['.', ',', ';']);
            if path.contains('/')
                && canary.rig.root.join(path).is_file()
                && !cited.iter().any(|known| known == path)
            {
                cited.push(path.to_owned());
            }
        }
    }
    assert!(falls.len() >= 4, "{falls:?}");
    assert_eq!(cited, [CITED], "the files a planted row cites");

    // (1) THE CONTROL: the commit the setup was handed names a row beside how it falls — in
    // the tool, the log, the plan and this suite.
    let tracked = |at: &Path, skip: &[&str]| -> Vec<(String, String)> {
        texts_below(at)
            .into_iter()
            .filter(|(file, _)| !skip.iter().any(|prefix| file.starts_with(prefix)))
            .collect()
    };
    let source = beside(&tracked(&canary.rig.root, &[".git/"]), &names, &falls);
    for place in [TOOL, "DECISIONS.md", PLAN, SUITE] {
        assert!(
            source.iter().any(|(file, ..)| file == place),
            "the commit named pairs a row with how it falls in `{place}`"
        );
    }

    // (2) THE CLONE'S TREE, AS THE SETUP LEAVES IT — every file but its git directory and
    // the run's directory, which holds the rows themselves, as a verifier is handed its own
    // — names no row beside how it falls.
    let skip = [".git/".to_owned(), format!("{RUN_DIR}/")];
    let skip: Vec<&str> = skip.iter().map(String::as_str).collect();
    let texts = tracked(&clone, &skip);
    assert!(texts.len() > 10, "{}", texts.len());
    assert_eq!(
        beside(&texts, &names, &falls),
        Vec::<(String, usize, String, String)>::new(),
        "the clone's tree names a planted row beside how it falls"
    );

    // (3) A FILE A ROW CITES SAYS NOTHING OF HOW ANYTHING FALLS in the clone, and the row
    // still cites it.
    for path in &cited {
        let text = fs::read_to_string(clone.join(path)).expect("read the cited file");
        for fall in &falls {
            assert!(
                !text.to_lowercase().contains(&fall.to_lowercase()),
                "the clone's `{path}` says `{fall}`: {text}"
            );
        }
    }

    // (4) WHAT WAS WITHHELD IS ONE COMMIT ON THE COMMIT NAMED, the opening on it: it changes
    // exactly the files the line names under `withheld`, keeps each where it was, and keeps
    // every heading of the log and the entry that states the exit rule whole. `check` asks
    // the reading about each of those files by its path.
    let withheld: Vec<String> = line["withheld"]
        .as_array()
        .unwrap_or_else(|| panic!("the line names what it withheld: {line}"))
        .iter()
        .map(|path| path.as_str().expect("a path").to_owned())
        .collect();
    let mut expected: Vec<String> = [CITED, "DECISIONS.md", PLAN, SUITE, TOOL]
        .iter()
        .map(|path| (*path).to_owned())
        .collect();
    expected.sort();
    assert_eq!(withheld, expected, "{line}");
    assert_eq!(canary.git(&clone, &["rev-parse", "HEAD^^"]), canary.commit);
    assert_eq!(
        canary.git(
            &clone,
            &["diff", "--name-only", "--diff-filter=M", "HEAD^^", "HEAD^"]
        ),
        canary.git(&clone, &["diff", "--name-only", "HEAD^^", "HEAD^"]),
        "every withheld file is still where it was"
    );
    assert_eq!(
        canary.git(&clone, &["diff", "--name-only", "HEAD^^", "HEAD^"]),
        expected.join("\n")
    );
    let headings = |text: &str| -> Vec<String> {
        text.lines()
            .filter(|line| line.starts_with("## "))
            .map(str::to_owned)
            .collect()
    };
    let log = fs::read_to_string(clone.join("DECISIONS.md")).expect("the clone's log");
    let whole = fs::read_to_string(repo_root().join("DECISIONS.md")).expect("the log");
    assert_eq!(
        headings(&log),
        headings(&whole),
        "the log keeps every heading"
    );
    let rule = |text: &str| -> String {
        let at = text
            .find(&format!("## {EXIT_RULE}\n"))
            .expect("the entry that states the exit rule");
        let rest = &text[at..];
        rest[..rest[1..].find("\n## ").map_or(rest.len(), |end| end + 1)].to_owned()
    };
    assert_eq!(rule(&log), rule(&whole), "the exit rule's entry is whole");
    let asked = read["to_answer"]
        .as_array()
        .expect("`check` asks the reading");
    for path in &withheld {
        assert!(
            asked[0]
                .as_str()
                .is_some_and(|question| question.contains(path.as_str())),
            "the first question names `{path}`: {read}"
        );
    }
    // … and, the report a row cites being among them, a second asks about that row's
    // verifier.
    assert!(
        asked.len() == 2
            && asked[1]
                .as_str()
                .is_some_and(|question| question.contains(CITED)
                    && question.contains("go looking for the file in the history")),
        "{read}"
    );

    // (5) A FILE OF A KIND THE SETUP WITHHOLDS NOTHING OF, THAT NAMES A ROW, IS REFUSED
    // before anything is made: the clone would hold that row's answer.
    canary.rig.write(
        "notes/rows.json",
        &format!("{{\"{}\": \"refuted\"}}\n", PLANTS[4].0),
    );
    canary.commit = canary
        .rig
        .commit("docs: a row's answer in a file of another kind");
    let refused = canary.root("unfit");
    let said = canary.setup(&refused, &[PARTS]);
    said.refused("unfit");
    assert!(
        said.stderr.contains("`notes/rows.json`"),
        "the refusal names the file: {}",
        said.stderr
    );
    assert!(!refused.exists(), "nothing was made");
}

/// The identity the canary's clone commits under: the setup's own, which its commits carry.
const CANARY_NAME: &str = "The Canary";
const CANARY_EMAIL: &str = "canary@example.invalid";

/// **A record commit in the clone carries the canary's own identity, and `check` names one
/// that does not** (the staged canary's third root: its record commits carried the machine's
/// global `user.name` and `user.email` — a canary that leans on the machine's ambient git
/// configuration is not the same run on another machine, and its records carry an identity
/// they did not choose). The setup writes the canary's identity into the clone's own
/// configuration, which outranks a global one; `check` prints, of every commit on the
/// opening, its author and its committer and whether both are the canary's, and lists the
/// commits that are not — an identity the environment names outranks every configuration.
#[test]
fn a_record_commit_in_the_clone_carries_the_canarys_identity_and_check_names_one_that_does_not() {
    let canary = Canary::parts("parts-identity");
    canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let clone = root.join("clone");
    let canarys = format!("{CANARY_NAME} <{CANARY_EMAIL}>");

    // (1) THE CLONE'S OWN CONFIGURATION NAMES THE CANARY.
    assert_eq!(
        [
            canary.git(&clone, &["config", "--local", "user.name"]),
            canary.git(&clone, &["config", "--local", "user.email"]),
        ],
        [CANARY_NAME, CANARY_EMAIL]
    );

    // (2) A COMMIT MADE THERE AS A STAGE'S STEP MAKES IT — a machine's global configuration
    // naming somebody, no identity in the environment — carries the canary's identity.
    let global = canary.rig.dir().join("machine.gitconfig");
    fs::write(
        &global,
        "[user]\n\tname = Somebody Else\n\temail = somebody@example.invalid\n",
    )
    .expect("write a machine's own configuration");
    let mut command = canary.command(Path::new("git"));
    for name in [
        "GIT_AUTHOR_NAME",
        "GIT_AUTHOR_EMAIL",
        "GIT_COMMITTER_NAME",
        "GIT_COMMITTER_EMAIL",
    ] {
        command.env_remove(name);
    }
    let out = command
        .env("GIT_CONFIG_GLOBAL", &global)
        .args(["commit", "--allow-empty", "-q", "-m", "a record"])
        .current_dir(&clone)
        .output()
        .expect("run git commit");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let first = canary.git(&clone, &["rev-parse", "HEAD"]);
    assert_eq!(
        canary.git(&clone, &["log", "-1", "--format=%an <%ae>|%cn <%ce>"]),
        format!("{canarys}|{canarys}")
    );

    // (3) ONE WHOSE IDENTITY THE ENVIRONMENT NAMES — the rig's, here — is not the canary's,
    // and `check` says so, by its commit.
    canary.git(
        &clone,
        &["commit", "--allow-empty", "-q", "-m", "another record"],
    );
    let second = canary.git(&clone, &["rev-parse", "HEAD"]);
    // … and one whose author is the canary's and whose committer the environment names.
    let out = canary
        .command(Path::new("git"))
        .env_remove("GIT_AUTHOR_NAME")
        .env_remove("GIT_AUTHOR_EMAIL")
        .args(["commit", "--allow-empty", "-q", "-m", "a third record"])
        .current_dir(&clone)
        .output()
        .expect("run git commit");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let third = canary.git(&clone, &["rev-parse", "HEAD"]);
    let read = canary.checked(&root);
    let trail: Vec<Value> = read["trail"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` prints the trail: {read}"))
        .iter()
        .map(|record| {
            json!([
                record["commit"],
                record["author"],
                record["committer"],
                record["canary"]
            ])
        })
        .collect();
    let rigs = "The Rig <rig@example.invalid>";
    assert_eq!(
        trail,
        [
            json!([first, canarys, canarys, true]),
            json!([second, rigs, rigs, false]),
            json!([third, canarys, rigs, false]),
        ],
        "{read}"
    );
    assert_eq!(
        read["identity"],
        json!({"canary": canarys, "others": [second, third]}),
        "{read}"
    );
}

/// **`check` says of each act what was returned, and what the run shows** (the plan's §8; the
/// third fault of the row `wf-canary-setup`: *its `check` says `returned.then: false` both of
/// an invocation that never ran and of one that returned and was not written down*). For a
/// root of the parts `check` prints, beside what it printed, what a part is read by — the
/// budget, the queue, what is listed and why, the human's list, every filed verdict, the
/// view's standing, the attempts begun, the records made, what is pending by its path — and
/// FOR EACH ACT TWO THINGS: whether its return file is there, with the status it holds; and
/// whether THE RUN ITSELF shows the act. So an act that NEVER RAN, one that RAN AND WHOSE
/// RETURN WAS NOT WRITTEN DOWN, one that returned, and a return file the run does not bear
/// out are four answers. And for a root without the switch `check` prints what it printed,
/// field for field.
#[test]
fn check_says_of_each_act_what_was_returned_and_what_the_run_shows() {
    let canary = Canary::parts("parts-check");
    let line = canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");

    // (1) BEFORE ANY ACT: no act ran — and of the two the run cannot show, a look and the
    // record an ended act left, `check` says that it cannot tell.
    let read = canary.checked(&root);
    assert_eq!(standing(&read), stands_with(&[]), "{read}");
    for act in read["acts"].as_array().expect("the acts") {
        assert_eq!(
            json!([act["returned"], act["status"], act["next"]]),
            json!([false, null, null]),
            "{act}"
        );
    }
    // WHAT A PART IS READ BY, at the opening: the budget unspent, the queue empty, every
    // plant listed under the word that keeps it out, nothing filed, no view yet, no attempt
    // begun, no record made, nothing pending.
    let (route, reserve) = route_and_reserve();
    let off_the_order = [route, reserve].concat();
    assert_eq!(
        json!([
            read["budget"],
            read["queue"],
            read["listed"],
            read["human"],
            read["filed"],
            read["view"],
            read["attempts"],
            read["held"],
            read["trail"],
            read["pending_paths"]
        ]),
        json!([
            {"limit": BUDGET, "spent": 0, "left": BUDGET},
            [],
            {"class-off-the-order": off_the_order, "lead": ["setup-several-worktrees"]},
            [],
            [],
            null,
            {"begun": 0, "unrecorded": 0},
            [],
            [],
            []
        ]),
        "{read}"
    );
    assert_eq!(
        read["returned"],
        Value::Null,
        "the small canary's two return files are no act's of the parts: {read}"
    );

    // (2) A RETURN FILE THE RUN DOES NOT BEAR OUT: somebody wrote one for an act that left
    // nothing — a refusal, a halt, or a file written by hand. Its status is read from it.
    let second = prompted(act_of(&line, "part-2")).returns;
    fs::write(
        &second,
        json!({"status": "refused", "next": "test"}).to_string(),
    )
    .expect("write a return");
    let read = canary.checked(&root);
    assert_eq!(
        said_of(&read, "part-2"),
        &json!({"act": "part-2", "returned": true, "status": "refused", "next": "test",
                "own": false, "shows": false, "stands": "returned-unshown"}),
        "a refusal is no return of the act's own: {read}"
    );
    // … and one that is no JSON object is there, and says nothing.
    fs::write(&second, "not written down as JSON\n").expect("write a return");
    assert_eq!(
        said_of(&canary.checked(&root), "part-2"),
        &json!({"act": "part-2", "returned": true, "status": null, "next": null,
                "own": null, "shows": false, "stands": "returned-unshown"})
    );
    // WHETHER A RETURN IS ONE OF THE ACT'S OWN (the review before part 1, a row: `returned:
    // true` says only that something is at the path — another stage's return, a halt in the
    // wrong repository and the act's own read the same). The act's own status, of this run
    // and this stage, is its own; the same status of another run is not, nor of another
    // stage; and a status the act does not return on the main line is not — a halt, or
    // another act's return written to this file.
    for (said, own) in [
        (json!({"status": "hunted", "next": "stop"}), true),
        (
            json!({"status": "hunted", "stage": "test", "run": RUN}),
            true,
        ),
        (json!({"status": "hunted", "run": "another-run"}), false),
        (json!({"status": "hunted", "stage": "fix"}), false),
        (json!({"status": "halted"}), false),
        (json!({"status": "ruled", "next": "verify"}), false),
    ] {
        fs::write(&second, said.to_string()).expect("write a return");
        assert_eq!(
            said_of(&canary.checked(&root), "part-2")["own"],
            json!(own),
            "{said}"
        );
    }
    fs::remove_file(&second).expect("remove the return");
    // THE STATUSES THAT ARE EACH ACT'S OWN — and the act the human ends has none: every
    // return file written with one status, for each status a stage returns.
    let owns = [
        ("part-1", Some("stopped")),
        ("part-2", Some("hunted")),
        ("part-3-act-1", Some("ruled")),
        ("part-3-act-2", Some("verified")),
        ("part-4-act-1", Some("ruled")),
        ("part-4-act-2", None),
        ("part-4-act-3", Some("stopped")),
        ("part-4-act-4", Some("verified")),
        ("part-4-act-5", Some("recorded")),
    ];
    for said in [
        "stopped", "hunted", "ruled", "verified", "recorded", "halted", "refused",
    ] {
        for (act, _) in owns {
            fs::write(
                prompted(act_of(&line, act)).returns,
                json!({"status": said}).to_string(),
            )
            .expect("write a return");
        }
        let read = canary.checked(&root);
        for (act, status) in owns {
            assert_eq!(
                said_of(&read, act)["own"],
                json!(status == Some(said)),
                "`{said}` as the return of `{act}`"
            );
        }
    }
    for (act, _) in owns {
        fs::remove_file(prompted(act_of(&line, act)).returns).expect("remove the return");
    }

    // (3) WITHOUT THE SWITCH `check` IS WHAT IT WAS: the fields it printed, in their order,
    // and no other.
    let small = Canary::new("check-small");
    small.ready("canary", &[]);
    let told = small.tool(&[
        "check",
        "--root",
        &small.root("canary").display().to_string(),
    ]);
    told.done("check", "checked");
    let expected = [
        "act",
        "status",
        "root",
        "run",
        "loop",
        "branch",
        "remotes",
        "fetch_url",
        "push_url",
        "head",
        "remote_head",
        "pushed",
        "records",
        "pending",
        "state",
        "not_ready",
        "items",
        "ledger",
        "returned",
        "sha256",
    ];
    let object: serde_json::Map<String, Value> =
        serde_json::from_str(&told.raw).expect("the line is an object");
    let mut fields: Vec<&str> = object.keys().map(String::as_str).collect();
    fields.sort_unstable();
    let mut named = expected.to_vec();
    named.sort_unstable();
    assert_eq!(fields, named, "{}", told.raw);
    let places: Vec<usize> = expected
        .iter()
        .map(|name| {
            told.raw
                .find(&format!("\"{name}\": "))
                .unwrap_or_else(|| panic!("the line has `{name}`: {}", told.raw))
        })
        .collect();
    assert!(
        places.windows(2).all(|pair| pair[0] < pair[1]),
        "the fields stand in the order they stood in: {}",
        told.raw
    );

    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }

    // (4) AN ACT THAT RAN, AND WHOSE RETURN WAS NOT WRITTEN DOWN: part 1, run as its prompt
    // file says, to its stop. The run shows it — an attempt was begun, and reached no
    // record — and no return file is there.
    let ran = canary.act(&line, "part-1", json!({"scope": parts_scope()}), 1);
    assert_eq!(
        json!([ran.result["status"], ran.result["after"]]),
        json!(["stopped", "preflight"]),
        "{}",
        ran.result
    );
    let read = canary.checked(&root);
    assert_eq!(
        said_of(&read, "part-1"),
        &json!({"act": "part-1", "returned": false, "status": null, "next": null,
                "own": null, "shows": true, "stands": "not-written"}),
        "{read}"
    );
    assert_eq!(
        standing(&read),
        stands_with(&[("part-1", "not-written")]),
        "{read}"
    );
    assert_eq!(
        json!([
            read["attempts"],
            read["records"],
            read["trail"],
            read["state"]
        ]),
        json!([{"begun": 1, "unrecorded": 1}, 0, [], {"next": "test", "round": 1, "attempt": 2}]),
        "{read}"
    );

    // (5) AND ONE THAT RETURNED: the same act, its return written as its prompt says.
    written(&line, "part-1", &ran);
    let read = canary.checked(&root);
    assert_eq!(
        said_of(&read, "part-1"),
        &json!({"act": "part-1", "returned": true, "status": "stopped", "next": null,
                "own": true, "shows": true, "stands": "returned"}),
        "{read}"
    );
    assert_eq!(
        standing(&read),
        stands_with(&[("part-1", "returned")]),
        "{read}"
    );

    // (6) THE HELD COMMANDS, EACH AS THE STEP TOOL WOULD ANSWER IT — so that nobody looks into
    // the scratch root to see whether a gate still runs — READ, with nothing asked of that
    // tool, whose every answer of a command that has ended judges it and keeps the verdict.
    // (The review before part 1: three words of `held` were not the step tool's — a build
    // whose binary was replaced read `judged`, `green`; a directory without a record was not
    // listed; and a job whose supervisor was gone read `no-exit`, as a command that runs.)
    // THE CONTROL OF EVERY WORD IS THE CLONE'S OWN STEP TOOL, asked after `check`.
    let candidate = &line["head"].as_str().expect("the opening's commit")[..12];
    let previous = &canary.previous[..12];
    let gate = format!("gate-{candidate}");
    let build = format!("build-candidate-{candidate}");
    // `exit`: how the command ended, as its supervisor wrote it — where it is `done`, and
    // only there (the stand-ins for the long commands exit 0).
    let job = |name: &str, kind: &str, state: &str, why: Value, verdict: Value, kept: Value| json!({"name": name, "kind": kind, "state": state, "why": why, "verdict": verdict, "kept": kept, "exit": if state == "done" { json!(0) } else { Value::Null }});
    let green = |name: &str, kind: &str| {
        job(
            name,
            kind,
            "done",
            Value::Null,
            json!("green"),
            json!("green"),
        )
    };
    let held = |build_stands: Value, gate_stands: Value| {
        json!([
            build_stands,
            green(&format!("build-previous-{previous}"), "build"),
            gate_stands,
            green(&format!("regression-set-{candidate}-a1"), "regression"),
            green(&format!("regression-again-{candidate}-a1"), "regression"),
        ])
    };
    // What the step tool answers, as `[status, why, verdict]` — beside what `check` says
    // of the same job, as `[state, why, verdict]`.
    let answered = |name: &str| -> Value {
        let line = canary.step_answers(&root, name);
        json!([line["status"], line["why"], line["verdict"]])
    };
    let said = |read: &Value, name: &str| -> Value {
        let job = read["held"]
            .as_array()
            .expect("the held commands")
            .iter()
            .find(|job| job["name"] == name)
            .unwrap_or_else(|| panic!("`check` lists `{name}`: {read}"));
        json!([job["state"], job["why"], job["verdict"]])
    };
    // THE FIVE PART 1 HELD, in the order they were started, each done, green, and kept so.
    assert_eq!(
        read["held"],
        held(green(&build, "build"), green(&gate, "gate")),
        "{read}"
    );
    for name in [&build, &gate] {
        assert_eq!(said(&read, name), answered(name), "`{name}`");
    }
    // A COMMAND THAT ENDED WHILE NOBODY ASKED AFTER IT has no verdict's file — the step
    // tool's supervisor writes how it ended, and a verdict is written by the step that asks:
    // `check` says `done` with no verdict, kept or answered — which is what the human waits
    // for before act 5 — and HOW IT ENDED, from the supervisor's `exit.json`: ended, exit 0,
    // no verdict yet. (No control here: asked, the step tool would judge it and keep it.)
    let home = root.join("scratch/hold").join(&gate);
    let verdict = fs::read(home.join("verdict.json")).expect("the kept verdict");
    fs::remove_file(home.join("verdict.json")).expect("the verdict nobody had asked for yet");
    let unjudged = job(&gate, "gate", "done", Value::Null, Value::Null, Value::Null);
    assert_eq!(
        canary.checked(&root)["held"],
        held(green(&build, "build"), unjudged)
    );
    // … and of one that ended red, exit 1: the exit is the supervisor's word, read as it is.
    let ended = fs::read(home.join("exit.json")).expect("how the command ended");
    let mut red: Value = serde_json::from_slice(&ended).expect("the exit file is JSON");
    red["exit"] = json!(1);
    fs::write(home.join("exit.json"), red.to_string()).expect("a command that ended red");
    let mut unjudged_red = job(&gate, "gate", "done", Value::Null, Value::Null, Value::Null);
    unjudged_red["exit"] = json!(1);
    assert_eq!(
        canary.checked(&root)["held"],
        held(green(&build, "build"), unjudged_red)
    );
    fs::write(home.join("exit.json"), &ended).expect("put back how the command ended");
    // ONE WHOSE SUPERVISOR LIVES RUNS: the job's lock is held — here by the suite, as the
    // supervisor holds it — and both say `running`. Whatever else the directory holds.
    let lock = lock_held(&home.join("lock"));
    let read = canary.checked(&root);
    let running = job(
        &gate,
        "gate",
        "running",
        Value::Null,
        Value::Null,
        Value::Null,
    );
    assert_eq!(
        read["held"],
        held(green(&build, "build"), running),
        "{read}"
    );
    assert_eq!(said(&read, &gate), answered(&gate));
    drop(lock);
    // ONE WHOSE SUPERVISOR IS GONE AND WROTE NOTHING IS DEAD — the lock is free, and nothing
    // says how the command ended: never the word of a command that runs.
    let exit = fs::read(home.join("exit.json")).expect("how the command ended");
    fs::remove_file(home.join("exit.json")).expect("how the command ended");
    let read = canary.checked(&root);
    let dead = job(
        &gate,
        "gate",
        "dead",
        json!("no-exit"),
        Value::Null,
        Value::Null,
    );
    assert_eq!(read["held"], held(green(&build, "build"), dead), "{read}");
    assert_eq!(said(&read, &gate), answered(&gate));
    // … and one a signal ended is dead by that word.
    fs::write(
        home.join("exit.json"),
        json!({"exit": null, "signal": 9, "began": 1.0, "ended": 2.0}).to_string(),
    )
    .expect("a command a signal ended");
    let read = canary.checked(&root);
    let killed = job(
        &gate,
        "gate",
        "dead",
        json!("killed"),
        Value::Null,
        Value::Null,
    );
    assert_eq!(read["held"], held(green(&build, "build"), killed), "{read}");
    assert_eq!(said(&read, &gate), answered(&gate));
    fs::write(home.join("exit.json"), exit).expect("put back how the command ended");
    fs::write(home.join("verdict.json"), verdict).expect("put the kept verdict back");
    assert_eq!(
        canary.checked(&root)["held"],
        held(green(&build, "build"), green(&gate, "gate"))
    );

    // A BUILD THAT STANDS GREEN IS ANSWERED GREEN ONLY WHILE ITS BINARY IS THE FILE IT
    // HASHED: with another file at the path the answer is `void`, `hash-differs` — and THE
    // KEPT WORD IS PRINTED AS KEPT. (No executable bit: nothing here runs the file.)
    {
        use std::os::unix::fs::PermissionsExt as _;
        let record: Value = serde_json::from_slice(
            &fs::read(root.join("scratch/hold").join(&build).join("job.json"))
                .expect("the build's record"),
        )
        .expect("the record is JSON");
        let binary = root.join("scratch").join(
            record["asked"]["to"]
                .as_str()
                .expect("where the binary lands"),
        );
        let bytes = fs::read(&binary).expect("the binary");
        let place = |content: &[u8]| {
            fs::remove_file(&binary).expect("remove the binary");
            fs::write(&binary, content).expect("write at the binary's path");
            fs::set_permissions(&binary, fs::Permissions::from_mode(0o444)).expect("set its mode");
        };
        // The same bytes, open for writing, are not that binary either.
        fs::remove_file(&binary).expect("remove the binary");
        fs::write(&binary, &bytes).expect("write at the binary's path");
        fs::set_permissions(&binary, fs::Permissions::from_mode(0o644)).expect("set its mode");
        let read = canary.checked(&root);
        assert_eq!(
            said(&read, &build),
            json!(["done", "hash-differs", "void"]),
            "{read}"
        );
        assert_eq!(said(&read, &build), answered(&build));
        place(b"#!/bin/sh\necho another binary\n");
        let read = canary.checked(&root);
        let differs = job(
            &build,
            "build",
            "done",
            json!("hash-differs"),
            json!("void"),
            json!("green"),
        );
        assert_eq!(read["held"], held(differs, green(&gate, "gate")), "{read}");
        assert_eq!(said(&read, &build), answered(&build));
        place(&bytes);
        let read = canary.checked(&root);
        assert_eq!(
            read["held"],
            held(green(&build, "build"), green(&gate, "gate")),
            "{read}"
        );
        assert_eq!(said(&read, &build), answered(&build));
    }

    // A DIRECTORY WITHOUT A RECORD IS LISTED, AND IS DEAD BY A WORD OF ITS OWN — a start that
    // was killed before it wrote one: first, as a job with no start on record. `supervised`
    // says whether a supervisor lives under it all the same.
    let orphan = root.join("scratch/hold/gate-no-record");
    fs::create_dir_all(orphan.join("work")).expect("a directory a start left");
    fs::write(orphan.join("lock"), "").expect("its lock");
    let no_record = |supervised: bool| {
        json!({"name": "gate-no-record", "kind": null, "state": "dead", "why": "no-record",
               "verdict": null, "kept": null, "exit": null, "supervised": supervised})
    };
    let read = canary.checked(&root);
    assert_eq!(read["held"][0], no_record(false), "{read}");
    assert_eq!(read["held"].as_array().expect("the held commands").len(), 6);
    let answer = canary.step_answers(&root, "gate-no-record");
    assert_eq!(
        json!([answer["status"], answer["why"], answer["supervised"]]),
        json!(["dead", "no-record", false]),
        "{answer}"
    );
    let lock = lock_held(&orphan.join("lock"));
    assert_eq!(canary.checked(&root)["held"][0], no_record(true));
    assert_eq!(
        canary.step_answers(&root, "gate-no-record")["supervised"],
        true
    );
    drop(lock);
    // WHAT A HAND LEFT IN A JOB'S DIRECTORY THE STEP TOOL ANSWERS NOTHING OF, and `check`
    // says which: a verdict's file that is not the line the tool kept.
    let was = fs::read(home.join("verdict.json")).expect("the kept verdict");
    fs::write(home.join("verdict.json"), "{}\n").expect("a file a hand wrote");
    assert_eq!(
        canary.checked(&root)["held"][3],
        json!({"name": gate, "kind": "gate", "state": null, "why": "verdict-unread",
               "verdict": null, "kept": null, "exit": null})
    );
    fs::write(home.join("verdict.json"), was).expect("put the kept verdict back");

    // (7) A KEPT LINE WITH A KEY MISSING IS REFUSED, IN ONE LINE — never a traceback (the
    // review before part 1, a row): `check` reads the root by that file, and reads nothing
    // of it on a guess. The run in the clone is untouched, and no setup is asked for.
    let kept = root.join("canary.json");
    let whole = fs::read_to_string(&kept).expect("the kept line");
    for lacking in ["head", "budget", "run_dir", "candidate"] {
        let mut cut: Value = serde_json::from_str(&whole).expect("the kept line is JSON");
        cut.as_object_mut().expect("an object").remove(lacking);
        fs::write(&kept, cut.to_string()).expect("cut the kept line");
        let told = canary.tool(&["check", "--root", &root.display().to_string()]);
        let refused = told.refused("missing");
        assert!(
            refused["halt"]["root_cause"]
                .as_str()
                .is_some_and(
                    |cause| cause.contains("is not the line a setup of this tool kept")
                        && cause.contains(&format!("it lacks {lacking}"))
                ),
            "{refused}"
        );
        assert!(
            refused["halt"]["recommendation"]
                .as_str()
                .is_some_and(|then| then.contains("THE RUN IN THE CLONE IS UNTOUCHED")
                    && then.contains("No setup is asked for")),
            "{refused}"
        );
    }
    let mut cut: Value = serde_json::from_str(&whole).expect("the kept line is JSON");
    cut["acts"][0]
        .as_object_mut()
        .expect("an act")
        .remove("return");
    fs::write(&kept, cut.to_string()).expect("cut the kept line");
    canary
        .tool(&["check", "--root", &root.display().to_string()])
        .refused("missing");
    // … and so is a kept file that is no JSON at all.
    fs::write(&kept, "cut short: {\"act\": \"setup\", \n").expect("cut the kept line");
    let told = canary.tool(&["check", "--root", &root.display().to_string()]);
    assert!(
        told.refused("missing")["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains("it lacks a JSON object")),
        "{}",
        told.raw
    );
    fs::write(&kept, whole).expect("put the kept line back");
    canary.checked(&root);
}

/// A job's lock, HELD — as the supervisor of a held command holds it for as long as it
/// lives: a child that takes the lock and keeps it until it is dropped. Its command names
/// the lock by its path under the rig, so the rig ends it where a test did not.
struct LockHeld(std::process::Child);

fn lock_held(lock: &Path) -> LockHeld {
    use std::io::BufRead as _;
    let mut child = Command::new("python3")
        .args([
            "-c",
            "import fcntl, os, sys\nfd = os.open(sys.argv[1], os.O_RDONLY)\nfcntl.flock(fd, fcntl.LOCK_EX)\nprint('held', flush=True)\nsys.stdin.read()",
        ])
        .arg(lock)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn python3");
    let mut said = String::new();
    std::io::BufReader::new(child.stdout.as_mut().expect("its stdout"))
        .read_line(&mut said)
        .expect("read what it says");
    assert_eq!(said, "held\n", "the lock is held");
    LockHeld(child)
}

impl Drop for LockHeld {
    fn drop(&mut self) {
        drop(self.0.stdin.take());
        let _ = self.0.wait();
    }
}

impl Canary {
    /// WHAT THE CLONE'S OWN STEP TOOL ANSWERS OF THE HELD COMMAND `name`, asked by a wait of
    /// one second: the control of what `check` says of it. Asked AFTER `check` — of a
    /// command that has ended the answer is its verdict, which the tool then keeps.
    fn step_answers(&self, root: &Path, name: &str) -> Value {
        let clone = root.join("clone");
        let out = self
            .command(&clone.join("dev/stabilize-step"))
            .args(["hold-wait", "--scratch"])
            .arg(root.join("scratch"))
            .args(["--name", name, "--slice", "1"])
            .current_dir(&clone)
            .stdin(Stdio::null())
            .output()
            .expect("run the clone's step tool");
        assert!(
            !out.stdout.is_empty(),
            "the clone's step tool answers of `{name}`: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        line_of(&String::from_utf8_lossy(&out.stdout))
    }
}

/// What a reviewer of the review row found, and how the stage's one triage keys it: ONE
/// finding at the round's door, a behaviour item of the hunt — a class on no order of the
/// parts, so that it is listed in every part and no verifier is launched for it.
const HUNTED: &str = "setup-prints-no-next-step";

fn hunted_script() -> Value {
    json!({
        "scope": parts_scope(),
        "agents": {
            "row-setup:source": {"status": "reported", "summary": "Found.", "findings": [
                {"id": "s-1", "title": "the verb prints no next step", "door": PARTS_DOOR,
                 "clause": "no-lost-files", "repro": "the block s-1"}]},
            "row-setup:driver": nothing(),
            "row-setup:reconciler": nothing(),
            "triage": {"entries": [
                {"key": HUNTED, "found": "new", "doctype": "jigc-feedback", "door": PARTS_DOOR,
                 "clause": "no-lost-files", "repro": "the block s-1", "grade": "breaks",
                 "kind": "behaviour", "commands": "jigc setup",
                 "why": "the verb leaves its caller with nothing to do next",
                 "folds": {"row-setup-source": [1]}}]},
        },
    })
}

/// The agents of an act that are one-command steps of the step tool.
fn tool_steps(ran: &Ran) -> usize {
    ran.agents
        .iter()
        .filter(|label| label.starts_with("git:"))
        .count()
}

/// The verifiers an invocation launched, by the key each was launched for, in order.
fn verified(ran: &Ran) -> Vec<&str> {
    ran.agents
        .iter()
        .filter_map(|label| label.strip_prefix("verify:"))
        .collect()
}

/// `[agents, tool_steps]` as an act's line expects them.
fn expected_of(line: &Value, id: &str) -> Value {
    let expect = &act_of(line, id)["expect"];
    json!([expect["agents"], expect["tool_steps"]])
}

/// **The acts run in order through the clone's own harness** (the plan's §5 and §10, `SC3`).
/// NO STAGE HAD RUN ON THE OPENING OF THE PARTS: this is the first, under the runtime's
/// stand-in — and every act is run AS ITS PROMPT FILE SAYS: the harness's path and the
/// argument object are read out of the file the human pastes, never out of the line. Part 1,
/// to its tuning stop, with no record; part 2, to the stage's own return, with ONE record
/// and no verifier; the order ruled, which starts nothing; part 3, which continues: a wave
/// of eight, a wave of one, the budget spent and the second record; the budget raised,
/// which starts nothing; A WAVE THAT DIES AFTER TWO FILINGS; a look; and the continuation,
/// which launches a verifier only for what has no verdict. EACH ACT LAUNCHES AS MANY AGENTS
/// AS ITS LINE SAYS, and after each `check` says how every act stands and what the run holds.
#[test]
fn the_acts_run_in_order_through_the_clones_own_harness() {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let canary = Canary::parts("parts-acts");
    let line = canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let (clone, origin) = (root.join("clone"), root.join("origin.git"));
    let source_before = canary.snapshot();
    let (route, reserve) = route_and_reserve();
    let previous = &canary.previous[..12];
    let candidate = &line["head"].as_str().expect("the opening's commit")[..12];

    // ---- PART 1 — the prelude: the stage, to its tuning stop `preflight` -----------------
    let part_1 = canary.act(&line, "part-1", json!({"scope": parts_scope()}), 1);
    written(&line, "part-1", &part_1);
    let result = &part_1.result;
    assert_eq!(
        json!([result["status"], result["after"], result["scope"]["status"]]),
        json!(["stopped", "preflight", "written"]),
        "{result}"
    );
    // THE TWO BUILDS, THE GATE AND TWO HELD REGRESSION ITEMS — the second after the first,
    // under a name of its own: each started by a step and asked after by another.
    let held = |ran: &Ran, step: &str| -> Vec<String> {
        ran.agents
            .iter()
            .filter_map(|label| label.strip_prefix(&format!("git:hold-{step}:")))
            .map(str::to_owned)
            .collect()
    };
    let jobs = [
        format!("build-candidate-{candidate}"),
        format!("build-previous-{previous}"),
        format!("gate-{candidate}"),
        format!("regression-set-{candidate}-a1"),
        format!("regression-again-{candidate}-a1"),
    ];
    assert_eq!(held(&part_1, "start"), jobs, "{:?}", part_1.agents);
    assert_eq!(held(&part_1, "wait"), jobs, "{:?}", part_1.agents);
    assert_eq!(
        result["held"]
            .as_array()
            .expect("what became of each held check")
            .iter()
            .map(|h| json!([h["item"], h["ends"]]))
            .collect::<Vec<_>>(),
        [
            json!(["gate", "green"]),
            json!(["regression-set", "green"]),
            json!(["regression-again", "green"])
        ],
        "{result}"
    );
    // BOTH REGRESSION SETS NAME ONE KEPT BASELINE, which is the previous release's.
    let long = canary.long_commands();
    let kept = root
        .join(format!("scratch/baseline/previous-{previous}"))
        .display()
        .to_string();
    let baselines: Vec<&str> = long
        .iter()
        .filter(|command| command.starts_with("regression-set run "))
        .map(|command| {
            command
                .split_once(" --baseline ")
                .unwrap_or_else(|| panic!("a held regression set names a kept baseline: {command}"))
                .1
                .split(' ')
                .next()
                .expect("a path")
        })
        .collect();
    assert_eq!(baselines, [kept.as_str(), kept.as_str()], "{long:?}");
    assert_eq!(
        expected_of(&line, "part-1"),
        json!([part_1.agents.len(), tool_steps(&part_1)]),
        "part 1 launches as many agents as its line says: {:?}",
        part_1.agents
    );
    // NOTHING ON RECORD, NOTHING PUSHED BEYOND THE OPENING: four files pending, every one
    // under the round's directory — and THE ROUND'S SCOPE SELECTS THE REVIEW ROW.
    let read = canary.checked(&root);
    assert_eq!(
        json!([
            read["records"],
            read["pushed"],
            read["pending"],
            read["state"],
            read["attempts"]
        ]),
        json!([0, true, 4, {"next": "test", "round": 1, "attempt": 2}, {"begun": 1, "unrecorded": 1}]),
        "{read}"
    );
    let pending: Vec<&str> = read["pending_paths"]
        .as_array()
        .expect("what is pending, by its path")
        .iter()
        .map(|path| path.as_str().expect("a path"))
        .collect();
    assert!(
        pending.len() == 4
            && pending
                .iter()
                .all(|path| path.starts_with(&format!("{RUN_DIR}/r1/"))),
        "{pending:?}"
    );
    assert_eq!(
        read["items"],
        json!([
            {"item": "row-setup", "kind": "review-row", "selected": true},
            {"item": "gate", "kind": "held-gate", "selected": true},
            {"item": "regression-set", "kind": "held-regression", "selected": true},
            {"item": "regression-again", "kind": "held-regression", "selected": true}
        ]),
        "the scope step's door selects the review row: {read}"
    );
    assert_eq!(
        standing(&read),
        stands_with(&[("part-1", "returned")]),
        "{read}"
    );

    // ---- PART 2 — the hunt and the first record: the stage, with no stop -----------------
    let part_2 = canary.act(&line, "part-2", hunted_script(), 2);
    written(&line, "part-2", &part_2);
    let result = &part_2.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["continues"],
            result["verify"]["waves"],
            result["budget"],
            result["queue"]["length"]
        ]),
        json!(["hunted", "stop", [], 0, {"limit": BUDGET, "spent": 0, "left": BUDGET}, 0]),
        "the stage returns by itself, as it hunted — the queue is empty, and nobody is verified: {result}"
    );
    // NOTHING BUILT, GATED OR CHECKED TWICE: each of the five jobs is asked for by its
    // start step and answered by the job that stands — the one wait is the record's gate.
    assert_eq!(held(&part_2, "start")[..5], jobs, "{:?}", part_2.agents);
    assert_eq!(
        held(&part_2, "wait"),
        ["record-test-r1-a2-1"],
        "{:?}",
        part_2.agents
    );
    assert_eq!(
        canary.long_commands()[long.len()..],
        ["gate --keep-going".to_owned()],
        "the one long command of part 2 is its record's gate"
    );
    // THE THREE REPORTERS OF THE REVIEW ROW, ONE TRIAGE, NO EXECUTOR, NO VERIFIER.
    let roles: Vec<&str> = part_2
        .agents
        .iter()
        .map(String::as_str)
        .filter(|label| !label.starts_with("git:"))
        .collect();
    assert_eq!(
        roles,
        [
            "preflight",
            "scope",
            "row-setup:source",
            "row-setup:driver",
            "row-setup:reconciler",
            "triage"
        ],
        "{:?}",
        part_2.agents
    );
    // THE TRIAGE IS HANDED NO PLANT: its sources are the reporters of the review row, it is
    // named no read of rows nobody graded, and no planted row's key stands in its prompt.
    let (_, triage) = part_2
        .prompts
        .iter()
        .find(|(label, _)| label == "triage")
        .expect("the stage's one triage");
    let call = part_2
        .commands
        .iter()
        .find(|command| command.starts_with("dev/stabilize-record triage "))
        .expect("the triage handed its entries over");
    assert_eq!(
        call.split_once(" -- ")
            .expect("the call names its sources")
            .1,
        "row-setup-source=1",
        "{call}"
    );
    assert!(
        !triage.contains("dev/stabilize-record untriaged ")
            && PLANTS.iter().all(|plant| !triage.contains(plant.0)),
        "the triage is handed no plant:\n{triage}"
    );
    assert_eq!(
        expected_of(&line, "part-2"),
        json!([part_2.agents.len(), tool_steps(&part_2)]),
        "part 2 launches as many agents as its line says: {:?}",
        part_2.agents
    );
    // ONE RECORD, PUSHED, NOTHING PENDING — and every plant stands as planted and listed.
    let read = canary.checked(&root);
    assert_eq!(
        json!([
            read["records"],
            read["pushed"],
            read["pending"],
            read["pending_paths"],
            read["state"]["next"],
            read["budget"],
            read["queue"],
            read["filed"],
            read["view"]["on_disk"]
        ]),
        json!([1, true, 0, [], "stop", {"limit": BUDGET, "spent": 0, "left": BUDGET}, [], [], "written"]),
        "{read}"
    );
    let plants_and_the_hunted = [route.clone(), reserve.clone(), vec![HUNTED]].concat();
    assert_eq!(
        read["listed"],
        json!({"class-off-the-order": plants_and_the_hunted,
               "lead": ["setup-several-worktrees"]}),
        "{read}"
    );
    for row in read["ledger"].as_array().expect("the ledger") {
        if row["key"] != HUNTED {
            assert_eq!(
                json!([row["grade"], row["route"]]),
                json!(["unclear", "listed"]),
                "{row}"
            );
        }
    }
    assert_eq!(
        standing(&read),
        stands_with(&[("part-1", "returned"), ("part-2", "returned")]),
        "{read}"
    );

    // ---- PART 3, ACT 1 — the order is ruled: recorded, and nothing started ---------------
    let ruled = canary.act(&line, "part-3-act-1", json!({}), 3);
    written(&line, "part-3-act-1", &ruled);
    let result = &ruled.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["continues"],
            result["queue"]["length"]
        ]),
        json!(["ruled", "verify", ["verify"], 13]),
        "the order re-opens the stage, and no go is owed: {result}"
    );
    assert_eq!(
        ruled
            .agents
            .iter()
            .filter(|label| !label.starts_with("git:"))
            .collect::<Vec<_>>(),
        ["record:rulings:r1"],
        "one executor, and nobody else: {:?}",
        ruled.agents
    );
    assert_eq!(
        expected_of(&line, "part-3-act-1"),
        json!([ruled.agents.len(), tool_steps(&ruled)]),
        "the ruling launches as many agents as its line says: {:?}",
        ruled.agents
    );
    let read = canary.checked(&root);
    let thirteen = [route.clone(), reserve.clone()].concat();
    assert_eq!(
        json!([
            read["records"],
            read["pushed"],
            read["pending"],
            read["state"],
            read["queue"],
            read["listed"]
        ]),
        json!([2, true, 0, {"next": "verify", "round": 1, "attempt": 3},
               thirteen,
               {"class-off-the-order": [HUNTED], "lead": ["setup-several-worktrees"]}]),
        "thirteen rows in the queue, the three route plants first: {read}"
    );
    assert_eq!(
        standing(&read),
        stands_with(&[
            ("part-1", "returned"),
            ("part-2", "returned"),
            ("part-3-act-1", "returned")
        ]),
        "{read}"
    );

    // ---- PART 3, ACT 2 — the stage continues: a wave of eight, a wave of one -------------
    // The first route plant is confirmed, a regression, and its verifier hands on three
    // items: two enter the queue. The second is refuted as one that does not reproduce —
    // what its verifier hands on is never queued. Nobody here can drive the third. The five
    // reserve plants of the wave are refuted.
    let (first, second, beyond, under_refuted) = (
        "setup-hook-link-route-names-no-link",
        "setup-hook-link-rerun-refuses-again",
        "setup-hook-link-route-third-item",
        "setup-hooks-mode-555-another-route",
    );
    let item = |key: &str| handed(&key.replace('-', " "), "route: do as the line says");
    let mut agents = json!({});
    agents[format!("verify:{}", route[0])] = files(json!({
        "verdict": "confirmed", "regression": true,
        "hands_on": [item(first), item(second), item(beyond)],
    }));
    agents[format!("verify:{}", route[1])] = files(json!({
        "verdict": "refuted", "refutation": "does-not-reproduce",
        "hands_on": [item(under_refuted)],
    }));
    agents[format!("verify:{}", route[2])] =
        files(json!({"verdict": "undrivable", "reason": "needs-a-linux-host"}));
    for key in &reserve[..5] {
        agents[format!("verify:{key}")] = refutes();
    }
    agents[format!("verify:{first}")] = refutes();
    let part_3 = canary.act(&line, "part-3-act-2", json!({"agents": agents}), 4);
    written(&line, "part-3-act-2", &part_3);
    let result = &part_3.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["continued"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["budget"]
        ]),
        json!(["verified", "rule", ["verify"], 2, 9, 9, {"limit": BUDGET, "spent": BUDGET, "left": 0}]),
        "{result}"
    );
    // WAVE 1 IS THE EIGHT THE PLAN NAMES — the three route plants and the first five of the
    // reserve, in the ledger's order — and WAVE 2 THE ONE HANDED-ON ITEM, ahead of the
    // sixth reserve plant, an older row.
    assert_eq!(
        verified(&part_3),
        [route.clone(), reserve[..5].to_vec(), vec![first]].concat(),
        "{:?}",
        part_3.agents
    );
    // NO INSTRUMENT, NO TRIAGE, NO EXECUTOR: tool steps and verifiers.
    assert!(
        part_3
            .agents
            .iter()
            .all(|label| label.starts_with("git:") || label.starts_with("verify:")),
        "{:?}",
        part_3.agents
    );
    assert_eq!(
        canary
            .long_commands()
            .iter()
            .filter(
                |command| command.starts_with("regression-set ") || command.starts_with("cargo ")
            )
            .count(),
        4,
        "no build and no regression set ran again"
    );
    assert_eq!(
        expected_of(&line, "part-3-act-2"),
        json!([part_3.agents.len(), tool_steps(&part_3)]),
        "part 3 launches as many agents as its line says: {:?}",
        part_3.agents
    );
    // THE BUDGET IS SPENT, WITH ROWS NOBODY REACHED: the second handed-on item, then the
    // five reserve plants that are left. Nine verdicts are filed, each recorded.
    let read = canary.checked(&root);
    let unreached = [vec![second], reserve[5..].to_vec()].concat();
    assert_eq!(
        json!([
            read["records"],
            read["pushed"],
            read["pending"],
            read["state"]["next"],
            read["budget"],
            read["queue"]
        ]),
        json!([3, true, 0, "rule", {"limit": BUDGET, "spent": BUDGET, "left": 0},
               unreached]),
        "{read}"
    );
    let on_record = filed(&read);
    assert_eq!(on_record.len(), 9, "{read}");
    assert!(
        on_record
            .iter()
            .all(|(_, _, counts, recorded)| *counts && *recorded),
        "{read}"
    );
    assert_eq!(
        read["human"],
        json!([{"key": route[2], "why": "undrivable"}, {"key": second, "why": "unreached"}]),
        "{read}"
    );
    assert_eq!(
        json!([
            read["listed"]["beyond-hand-on"],
            read["listed"]["parent-did-not-reproduce"]
        ]),
        json!([[beyond], [under_refuted]]),
        "{read}"
    );
    let through_part_3 = [
        ("part-1", "returned"),
        ("part-2", "returned"),
        ("part-3-act-1", "returned"),
        ("part-3-act-2", "returned"),
    ];
    assert_eq!(standing(&read), stands_with(&through_part_3), "{read}");

    // ---- PART 4, ACT 1 — the raise, alone: recorded, and nothing started -----------------
    let raised = canary.act(&line, "part-4-act-1", json!({}), 5);
    written(&line, "part-4-act-1", &raised);
    let result = &raised.result;
    assert_eq!(
        json!([result["status"], result["next"], result["budget"]]),
        json!(["ruled", "verify", {"limit": 14, "spent": 9, "left": 5}]),
        "{result}"
    );
    assert_eq!(
        expected_of(&line, "part-4-act-1"),
        json!([raised.agents.len(), tool_steps(&raised)]),
        "the raise launches as many agents as its line says: {:?}",
        raised.agents
    );
    let read = canary.checked(&root);
    let attempt = read["state"]["attempt"]
        .as_u64()
        .expect("the attempt the next invocation begins");
    assert_eq!(
        json!([read["records"], read["state"], read["budget"]]),
        json!([4, {"next": "verify", "round": 1, "attempt": 4}, {"limit": 14, "spent": 9, "left": 5}]),
        "{read}"
    );
    assert_eq!(
        standing(&read),
        stands_with(&[through_part_3.as_slice(), &[("part-4-act-1", "returned")]].concat()),
        "a raise is no attempt: of the act after it the run shows nothing yet: {read}"
    );

    // ---- PART 4, ACT 2 — the wave, and the kill: IT DIES AFTER TWO FILINGS ----------------
    // The wave is the first five of the queue. Two verifiers file; the session is ended
    // while the other three drive: they leave nothing, and the invocation never returns.
    let wave = [second, reserve[5], reserve[6], reserve[7], reserve[8]];
    let mut agents = json!({});
    for key in &wave[..2] {
        agents[format!("verify:{key}")] = refutes();
    }
    let mut endings = json!({"git:check-reports:wave1": "dies"});
    for key in &wave[2..] {
        agents[format!("verify:{key}")] = refutes();
        endings[format!("verify:{key}")] = json!("dies");
    }
    let killed = canary.act(
        &line,
        "part-4-act-2",
        json!({"agents": agents, "endings": endings}),
        6,
    );
    assert_eq!(killed.result["status"], "halted", "{}", killed.result);
    assert_eq!(verified(&killed), wave, "{:?}", killed.agents);
    // UNTIL IT IS ENDED the act launched what its line says is launched by then: the steps
    // an invocation that continues opens with, and the wave's five verifiers.
    let until: Vec<&String> = killed
        .agents
        .iter()
        .take_while(|label| *label != "git:check-reports:wave1")
        .collect();
    let ended = &act_of(&line, "part-4-act-2")["expect"]["ended"];
    assert_eq!(
        json!([ended["agents"], ended["tool_steps"]]),
        json!([
            until.len(),
            until
                .iter()
                .filter(|label| label.starts_with("git:"))
                .count()
        ]),
        "{:?}",
        killed.agents
    );
    // THE SNAPSHOT: `check` prints every filed file with its hash — the two of this
    // attempt among them, which count and which no record took — and says of the act that
    // the run shows it and no return is there.
    let snapshot = canary.checked(&root);
    let before = filed(&snapshot);
    assert_eq!(before.len(), 11, "{snapshot}");
    let waiting: Vec<&(String, String, bool, bool)> =
        before.iter().filter(|entry| !entry.3).collect();
    assert_eq!(
        waiting
            .iter()
            .map(|entry| (entry.0.as_str(), entry.2))
            .collect::<Vec<_>>(),
        [(wave[0], true), (wave[1], true)],
        "{snapshot}"
    );
    for entry in snapshot["filed"].as_array().expect("what is filed") {
        let file = clone.join(entry["file"].as_str().expect("a filed file's path"));
        assert_eq!(entry["sha256"], sha256_at(&file).as_str(), "{entry}");
    }
    assert!(
        waiting.iter().all(|entry| snapshot["filed"]
            .as_array()
            .expect("what is filed")
            .iter()
            .any(|filed| filed["key"] == entry.0.as_str()
                && filed["file"]
                    .as_str()
                    .is_some_and(|file| file.ends_with(&format!(".a{attempt}.json"))))),
        "the two filings are of the attempt `check` said this invocation would begin: {snapshot}"
    );
    assert_eq!(
        json!([
            snapshot["records"],
            snapshot["state"],
            snapshot["budget"],
            said_of(&snapshot, "part-4-act-2")
        ]),
        json!([4, {"next": "verify", "round": 1, "attempt": 5}, {"limit": 14, "spent": 11, "left": 3},
               {"act": "part-4-act-2", "returned": false, "status": null, "next": null,
                "own": null, "shows": true, "stands": "not-written"}]),
        "{snapshot}"
    );
    assert_eq!(
        standing(&snapshot),
        stands_with(
            &[
                through_part_3.as_slice(),
                &[
                    ("part-4-act-1", "returned"),
                    ("part-4-act-2", "not-written")
                ]
            ]
            .concat()
        ),
        "the ended act left an attempt and no record: of the continuation the run shows nothing yet: {snapshot}"
    );

    // ---- PART 4, ACT 3 — a look: it launches nobody -------------------------------------
    let looked = canary.act(&line, "part-4-act-3", json!({}), 7);
    written(&line, "part-4-act-3", &looked);
    assert_eq!(
        json!([
            looked.result["status"],
            looked.result["after"],
            looked.result["continues"]
        ]),
        json!(["stopped", "state", ["verify", "filed"]]),
        "{}",
        looked.result
    );
    assert_eq!(
        expected_of(&line, "part-4-act-3"),
        json!([looked.agents.len(), tool_steps(&looked)]),
        "{:?}",
        looked.agents
    );
    assert_eq!(
        filed(&canary.checked(&root)),
        before,
        "a look changes nothing"
    );

    // ---- PART 4, ACT 4 — the continuation: only what has no verdict is launched ----------
    let left = snapshot["budget"]["left"]
        .as_u64()
        .expect("what the budget has left");
    let mut agents = json!({});
    for key in &wave[2..] {
        agents[format!("verify:{key}")] = refutes();
    }
    // IT IS ENDED AT ITS RECORD'S GATE — the act-5 window of the plan, on the main line: the
    // record's batch is applied, its gate did not pass, and no commit holds it.
    let continued = canary.act(
        &line,
        "part-4-act-4",
        json!({"agents": agents, "record": {"gate": {"void": true}}}),
        8,
    );
    let result = &continued.result;
    assert_eq!(
        json!([result["status"], result["halted"]["phase"]]),
        json!(["halted", "record"]),
        "{result}"
    );
    assert_eq!(verified(&continued), wave[2..], "{:?}", continued.agents);
    assert!(
        continued
            .agents
            .iter()
            .all(|label| label.starts_with("git:") || label.starts_with("verify:")),
        "no instrument after the kill: {:?}",
        continued.agents
    );
    // AS MANY AGENTS AS ITS LINE SAYS, LESS THE THREE STEPS AFTER THE GATE — the record's
    // commit, its push and the state read back, which the invocation after it makes: the
    // steps, and one verifier for each verdict the budget had left when the act started —
    // which `check` printed.
    let expect = &act_of(&line, "part-4-act-4")["expect"];
    assert_eq!(
        json!([expect["tool_steps"], expect["verifiers"]]),
        json!([tool_steps(&continued) + 3, "budget.left"]),
        "{expect}"
    );
    assert_eq!(
        expect["agents"].as_u64().expect("the act's steps") + left,
        continued.agents.len() as u64 + 3,
        "{:?}",
        continued.agents
    );
    assert_eq!(
        continued.agents.last().map(String::as_str),
        Some("git:hold-wait:record-verdicts-r1-a5-1"),
        "{:?}",
        continued.agents
    );
    // THE WINDOW, AS `check` READS IT (the review before part 1, its second finding: in this
    // window act 4 read `never-ran`, every filed verdict `recorded: true`, and nothing named
    // the batch). The human ended the act, so no return is written: THE RUN SHOWS THE ACT —
    // the batch that is applied is the record it will be, with the markers that are pending
    // — and it stands as one that ran and was not written down. The batch is named, with
    // the five filings it takes: the two of the ended act and the three of this one, each
    // printed as one NO RECORD TOOK. What is committed is said as it is: four records.
    let read = canary.checked(&root);
    assert_eq!(
        said_of(&read, "part-4-act-4"),
        &json!({"act": "part-4-act-4", "returned": false, "status": null, "next": null,
                "own": null, "shows": true, "stands": "not-written"}),
        "{read}"
    );
    assert_eq!(
        said_of(&read, "part-4-act-2")["stands"],
        "not-written",
        "{read}"
    );
    let window = filed(&read);
    assert_eq!(window.len(), 14, "{read}");
    for entry in &before {
        assert!(
            window.contains(&(entry.0.clone(), entry.1.clone(), true, entry.3)),
            "`{}` is kept, with its hash, and is recorded only where a commit holds it: {read}",
            entry.0
        );
    }
    let in_the_batch: Vec<&str> = read["filed"]
        .as_array()
        .expect("what is filed")
        .iter()
        .filter(|entry| entry["applied"] == true)
        .map(|entry| {
            assert_eq!(entry["recorded"], false, "{entry}");
            entry["key"].as_str().expect("a key")
        })
        .collect();
    let mut of_both_attempts = wave.to_vec();
    of_both_attempts.sort_unstable();
    let mut taken = in_the_batch.clone();
    taken.sort_unstable();
    assert_eq!(taken, of_both_attempts, "{read}");
    assert_eq!(
        window.iter().filter(|entry| entry.3).count(),
        9,
        "the nine verdicts part 3 recorded are recorded: {read}"
    );
    assert_eq!(
        json!([
            read["applied"]["subject"],
            read["applied"]["in_commit"],
            read["applied"]["filed"].as_array().map(Vec::len),
            read["state"]["across"],
            read["records"],
            read["ruling"]["ruling"]
        ]),
        json!([
            format!("docs(record): {RUN} r1 - the verdicts of the test stage"),
            false,
            5,
            "applied",
            4,
            "ruling-raise-again"
        ]),
        "{read}"
    );
    // (ACROSS THE BATCH the state reads as the state of the missed window does — the budget
    // spent, rows nobody reached — and `check` WITHHOLDS the ruling's file: the next
    // invocation finishes the batch, and no ruling is the step.)
    assert!(
        read["ruling"]["prompt"].is_null()
            && read["ruling"]["withheld"]
                .as_str()
                .is_some_and(|why| why.contains("a batch is applied and in no commit")),
        "{read}"
    );
    // … and the return the act wrote — it halted — is there, and is none of its own.
    written(&line, "part-4-act-4", &continued);
    assert_eq!(
        said_of(&canary.checked(&root), "part-4-act-4"),
        &json!({"act": "part-4-act-4", "returned": true, "status": "halted", "next": null,
                "own": false, "shows": true, "stands": "returned"})
    );

    // ---- PART 4, ACT 5 — the same arguments once more: the batch's record, and nothing else
    let finished = canary.act(&line, "part-4-act-5", json!({}), 9);
    written(&line, "part-4-act-5", &finished);
    assert_eq!(finished.result["status"], "recorded", "{}", finished.result);
    assert_eq!(
        expected_of(&line, "part-4-act-5"),
        json!([finished.agents.len(), tool_steps(&finished)]),
        "act 5 launches as many agents as its line says: {:?}",
        finished.agents
    );
    // ITS LAST THREE STEPS ARE THE THREE ACT 4 DID NOT REACH.
    assert_eq!(
        finished.agents[finished.agents.len() - 3..]
            .iter()
            .map(|label| label.split(':').nth(1).unwrap_or_default())
            .collect::<Vec<_>>(),
        ["record", "push", "state"],
        "{:?}",
        finished.agents
    );
    // THE FILED VERDICTS ARE KEPT: every file of the snapshot is there with the hash it had,
    // and the record took it — and with it the two of the continuation's own attempt.
    let read = canary.checked(&root);
    let after = filed(&read);
    assert_eq!(after.len(), 14, "{read}");
    for entry in &before {
        assert!(
            after.contains(&(entry.0.clone(), entry.1.clone(), true, true)),
            "`{}` is kept, with its hash, and recorded: {read}",
            entry.0
        );
    }
    assert!(after.iter().all(|entry| entry.3), "{read}");
    assert_eq!(
        json!([
            read["records"],
            read["pushed"],
            read["pending"],
            read["budget"],
            read["queue"],
            read["applied"],
            read["state"]["across"]
        ]),
        json!([5, true, 0, {"limit": 14, "spent": 14, "left": 0}, reserve[9..], null, null]),
        "{read}"
    );
    assert_eq!(
        standing(&read),
        stands_with(&[
            ("part-1", "returned"),
            ("part-2", "returned"),
            ("part-3-act-1", "returned"),
            ("part-3-act-2", "returned"),
            ("part-4-act-1", "returned"),
            ("part-4-act-2", "not-written"),
            ("part-4-act-3", "returned"),
            ("part-4-act-4", "returned"),
            ("part-4-act-5", "returned")
        ]),
        "{read}"
    );
    // THE RUN'S TRAIL: five records on the opening, in the order the acts made them.
    assert_eq!(
        read["trail"]
            .as_array()
            .expect("the records made")
            .iter()
            .map(|record| record["subject"]
                .as_str()
                .expect("a subject")
                .rsplit(" - ")
                .next()
                .expect("what the record is of")
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "the findings of the test stage",
            "the rulings of the human",
            "the verdicts of the test stage",
            "the rulings of the human",
            "the verdicts of the test stage"
        ],
        "{read}"
    );

    // ---- OFF THE MAIN LINE — ONE MORE RAISE, by its own prompt file -------------------------
    // The budget part 4 raised is spent, and rows are left that nobody reached: the state
    // the plan's missed window leaves. `check` NAMES THE RULING THAT IS THE STEP, by its
    // prompt file — and the harness takes the argument object that file holds: recorded,
    // and nothing started.
    let ruling = &read["ruling"];
    let raise = line["rulings"]
        .as_array()
        .expect("the rulings off the main line")
        .iter()
        .find(|ruling| ruling["ruling"] == "ruling-raise-again")
        .expect("one more raise");
    assert_eq!(
        json!([ruling["ruling"], ruling["prompt"], ruling["withheld"]]),
        json!(["ruling-raise-again", raise["prompt"], null]),
        "{read}"
    );
    let told = prompted(raise);
    assert_eq!(told.args, raise["args"], "{}", told.text);
    let raised_again = canary.invoke_by(
        &told.harness,
        line["session_cwd"]
            .as_str()
            .expect("the session's directory"),
        &told.args,
        json!({}),
        10,
    );
    let result = &raised_again.result;
    assert_eq!(
        json!([result["status"], result["next"], result["budget"]]),
        json!(["ruled", "verify", {"limit": 19, "spent": 14, "left": 5}]),
        "{result}"
    );
    assert!(
        verified(&raised_again).is_empty(),
        "{:?}",
        raised_again.agents
    );
    let read = canary.checked(&root);
    assert_eq!(
        json!([read["ruling"], read["budget"], read["records"]]),
        json!([null, {"limit": 19, "spent": 14, "left": 5}, 6]),
        "{read}"
    );

    // AND NOTHING OF IT REACHED THE SOURCE REPOSITORY — nor does anything the acts wrote
    // into the run's directory, the results or the scratch root name a row as the plan does.
    // (But for ONE PLACE, which is the candidate's own: a held build unpacks the commit it
    // builds under the scratch root, and that tree holds this tool with its header.)
    assert_eq!(
        canary.git(&origin, &["rev-parse", &format!("refs/heads/{LOOP}")]),
        canary.git(&clone, &["rev-parse", "HEAD"])
    );
    assert_eq!(
        differing(&source_before, &canary.snapshot()),
        Vec::<String>::new(),
        "the source repository is what it was"
    );
    for dir in [
        root.join("scratch"),
        clone.join(RUN_DIR),
        root.join("results"),
    ] {
        for (name, text) in texts_below(&dir) {
            assert!(
                name.contains("/work/") || (!text.contains("plant-") && !text.contains("reserve")),
                "`{name}` names a row as the plan does"
            );
        }
    }
}

/// **A filed verdict is printed with its hash before and after the record that takes it**
/// (the plan's §5, part 4: *the filed verdicts kept — every file of the snapshot still there,
/// with its hash, and recorded*; its §9: the root that is opened afresh for part 4). HERE THE
/// ROOT IS THE ONE THE PLAN OPENS FOR ITS FOURTH PART AFTER A REPAIR — the gate as the one
/// item, the order part 3 rules, a budget of four — and the stage is run as part 2's prompt
/// file says: it goes from its first record INTO A FIRST WAVE in one invocation, the three
/// route plants and the first reserve plant, and dies after they filed. `check` then prints
/// each filed file with the sha256 of its bytes, as one that counts and that NO RECORD TOOK.
/// The continuation is ended at its record's gate — the batch applied, and in no commit —
/// and the invocation after it finishes that record and starts nothing: as many agents as
/// the line says of the act. Each file is printed with THE SAME HASH throughout, and at the
/// end as one a record took.
#[test]
fn a_filed_verdict_is_printed_with_its_hash_before_and_after_the_record_that_takes_it() {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let canary = Canary::parts("parts-filed");
    let line = canary.ready(
        "canary",
        &[
            PARTS,
            "--items",
            "gate",
            "--budget",
            "4",
            "--order",
            ORDER_THREE_FLAG,
        ],
    );
    let root = canary.root("canary");
    let clone = root.join("clone");
    let (route, reserve) = route_and_reserve();
    let wave = [route[0], route[1], route[2], reserve[0]];
    assert_eq!(canary.checked(&root)["filed"], json!([]));

    // THE STAGE, AS PART 2'S PROMPT FILE SAYS — which here goes on into a first wave, and
    // dies after its four verifiers filed.
    let mut agents = json!({});
    agents[format!("verify:{}", wave[0])] =
        files(json!({"verdict": "confirmed", "regression": true}));
    agents[format!("verify:{}", wave[1])] = refutes();
    agents[format!("verify:{}", wave[2])] =
        files(json!({"verdict": "undrivable", "reason": "needs-a-linux-host"}));
    agents[format!("verify:{}", wave[3])] = refutes();
    let died = canary.act(
        &line,
        "part-2",
        json!({"scope": parts_scope(), "agents": agents,
               "endings": {"git:check-reports:wave1": "dies"}}),
        1,
    );
    assert_eq!(died.result["status"], "halted", "{}", died.result);
    assert_eq!(verified(&died), wave, "{:?}", died.agents);

    // BEFORE THE RECORD: four files, each with the hash of its bytes, each counting, and
    // none in a record.
    let read = canary.checked(&root);
    let before = filed(&read);
    let mut waiting: Vec<(&str, bool, bool)> = before
        .iter()
        .map(|entry| (entry.0.as_str(), entry.2, entry.3))
        .collect();
    waiting.sort_unstable();
    let mut of_the_wave: Vec<(&str, bool, bool)> =
        wave.iter().map(|key| (*key, true, false)).collect();
    of_the_wave.sort_unstable();
    assert_eq!(waiting, of_the_wave, "{read}");
    let hashes = |read: &Value| -> Vec<String> {
        read["filed"]
            .as_array()
            .expect("what is filed")
            .iter()
            .map(|entry| {
                let file = clone.join(entry["file"].as_str().expect("a filed file's path"));
                assert_eq!(entry["sha256"], sha256_at(&file).as_str(), "{entry}");
                assert_eq!(
                    json!([entry["verdict"].is_string(), entry["attempt"], entry["why"]]),
                    json!([true, 1, null]),
                    "{entry}"
                );
                sha256_at(&file)
            })
            .collect()
    };
    let hashed = hashes(&read);
    assert_eq!(hashed.len(), 4);
    // (Of the acts `check` says what this root shows: the stage ran as part 2's prompt has
    // it and wrote no return — and no attempt went before its first record, so part 1 did
    // not run here.)
    assert_eq!(
        json!([
            said_of(&read, "part-1")["stands"],
            said_of(&read, "part-2")["stands"]
        ]),
        json!(["never-ran", "not-written"]),
        "{read}"
    );
    assert_eq!(
        json!([read["records"], read["budget"], read["state"]["next"]]),
        json!([1, {"limit": 4, "spent": 4, "left": 0}, "verify"]),
        "{read}"
    );

    // THE CONTINUATION, ENDED AT ITS RECORD'S GATE: the batch is applied, and no commit
    // holds it. The files are what they were.
    let ended = canary.act(
        &line,
        "part-4-act-4",
        json!({"record": {"gate": {"void": true}}}),
        2,
    );
    assert_eq!(
        json!([ended.result["status"], ended.result["halted"]["phase"]]),
        json!(["halted", "record"]),
        "{}",
        ended.result
    );
    assert!(verified(&ended).is_empty(), "{:?}", ended.agents);
    let read = canary.checked(&root);
    assert_eq!(hashes(&read), hashed, "{read}");
    assert_eq!(read["records"], 1, "{read}");
    // `check` SAYS HOW THE RECORD'S GATE STANDS, as the step tool would answer: here a step
    // asked after it, and its verdict is kept — void.
    let gate = read["held"]
        .as_array()
        .expect("the held commands")
        .last()
        .expect("the record's gate");
    assert_eq!(
        json!([
            gate["name"],
            gate["kind"],
            gate["state"],
            gate["verdict"],
            gate["kept"]
        ]),
        json!(["record-verdicts-r1-a2-1", "gate", "done", "void", "void"]),
        "{read}"
    );
    // THE WINDOW BEFORE ACT 5 — A BATCH APPLIED, AND IN NO COMMIT (the review before part 1,
    // its second finding: `check` read the run's state across that batch and named it
    // nowhere — every filed verdict read `recorded: true`, of a record no commit held).
    // `check` NAMES THE BATCH: its subject, the filings it takes, that no commit holds it,
    // and every field of the line that was read across it; `state` says so itself; and EACH
    // FILED VERDICT THE BATCH TAKES IS PRINTED AS ONE NO RECORD TOOK — `recorded` is asked
    // of what is committed — and as `applied`. The call that takes the batch back is
    // spelled whole.
    let applied = &read["applied"];
    let mut taken: Vec<&str> = read["filed"]
        .as_array()
        .expect("what is filed")
        .iter()
        .map(|entry| entry["file"].as_str().expect("a filed file's path"))
        .collect();
    taken.sort_unstable();
    assert_eq!(
        json!([
            applied["subject"],
            applied["in_commit"],
            applied["filed"],
            applied["across"],
            applied["discard"]
        ]),
        json!([
            format!("docs(record): {RUN} r1 - the verdicts of the test stage"),
            false,
            taken,
            [
                "state",
                "not_ready",
                "items",
                "ledger",
                "budget",
                "queue",
                "listed",
                "human",
                "filed",
                "view",
                "attempts"
            ],
            format!("dev/stabilize-step discard --branch {LOOP} --run-dir {RUN_DIR}")
        ]),
        "{read}"
    );
    assert_eq!(read["state"]["across"], "applied", "{read}");
    assert_eq!(
        filed(&read),
        before
            .iter()
            .map(|entry| (entry.0.clone(), entry.1.clone(), true, false))
            .collect::<Vec<_>>(),
        "no commit holds the record: of no filed verdict is it said that a record took it: {read}"
    );
    assert!(
        read["filed"]
            .as_array()
            .expect("what is filed")
            .iter()
            .all(|entry| entry["applied"] == true),
        "{read}"
    );
    assert_eq!(read["ruling"], Value::Null, "{read}");

    // AND THE INVOCATION AFTER IT — the same arguments once more — finishes the record and
    // starts nothing: as many agents as the line says of the act, every one a tool step.
    let finished = canary.act(&line, "part-4-act-5", json!({}), 3);
    written(&line, "part-4-act-5", &finished);
    assert_eq!(finished.result["status"], "recorded", "{}", finished.result);
    assert_eq!(
        expected_of(&line, "part-4-act-5"),
        json!([finished.agents.len(), tool_steps(&finished)]),
        "{:?}",
        finished.agents
    );
    assert_eq!(finished.agents.len(), tool_steps(&finished));

    // AFTER THE RECORD: the same four files, the same four hashes — and a record took each.
    let read = canary.checked(&root);
    assert_eq!(hashes(&read), hashed, "{read}");
    assert_eq!(
        filed(&read),
        before
            .iter()
            .map(|entry| (entry.0.clone(), entry.1.clone(), true, true))
            .collect::<Vec<_>>(),
        "{read}"
    );
    assert_eq!(
        json!([read["records"], read["pushed"], read["pending"]]),
        json!([2, true, 0]),
        "{read}"
    );
    // THE BATCH IS IN ITS COMMIT, AND NOTHING IS APPLIED: no field is read across anything.
    assert_eq!(
        json!([read["applied"], read["state"]["across"]]),
        json!([null, null]),
        "{read}"
    );
    assert!(
        read["filed"]
            .as_array()
            .expect("what is filed")
            .iter()
            .all(|entry| entry["applied"].is_null()),
        "{read}"
    );
}

/// **The ruling that grants one more attempt has a prompt file, `check` names it where it
/// is the step, and the harness takes what it holds** (the review before part 1, its closing
/// list: part 1 spends one attempt without a record; where the stage then halts before its
/// record once more, the third attempt is the human's to grant — and the argument object of
/// that ruling was the human's to type). HERE THE STAGE IS STOPPED TWICE BEFORE ANY RECORD,
/// as part 1's prompt file has it: the state then says that the step is the human's, for
/// the attempts that are spent, and `check` names the ruling's prompt file — but NOT WHILE
/// A HELD COMMAND RUNS, which is as much as it can see of an act that may be running. The
/// file's argument object, run through the clone's own harness, is recorded and starts
/// nothing; the stage has its attempt again, and no ruling is the step.
#[test]
fn the_ruling_off_the_main_line_is_named_where_it_is_the_step_and_the_harness_takes_it() {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let canary = Canary::parts("parts-rulings");
    let line = canary.ready("canary", &[PARTS, "--items", "gate"]);
    let root = canary.root("canary");
    assert_eq!(canary.checked(&root)["ruling"], Value::Null);

    // ONE ATTEMPT WITHOUT A RECORD — part 1 — and no ruling is the step.
    let first = canary.act(&line, "part-1", json!({"scope": parts_scope()}), 1);
    assert_eq!(first.result["status"], "stopped", "{}", first.result);
    let read = canary.checked(&root);
    assert_eq!(
        json!([read["ruling"], read["attempts"]]),
        json!([null, {"begun": 1, "unrecorded": 1}]),
        "{read}"
    );
    // A SECOND IN A ROW: the third is the human's to grant.
    let second = canary.act(&line, "part-1", json!({"scope": parts_scope()}), 2);
    assert_eq!(second.result["status"], "stopped", "{}", second.result);
    let read = canary.checked(&root);
    assert_eq!(
        json!([read["state"], read["attempts"]]),
        json!([{"next": "rule", "round": 1, "attempt": null, "refused": "attempts-spent"},
               {"begun": 2, "unrecorded": 2}]),
        "{read}"
    );
    let again = line["rulings"]
        .as_array()
        .expect("the rulings off the main line")
        .iter()
        .find(|ruling| ruling["ruling"] == "ruling-again")
        .expect("one more attempt");
    let named = &read["ruling"];
    assert_eq!(
        json!([named["ruling"], named["prompt"], named["withheld"]]),
        json!(["ruling-again", again["prompt"], null]),
        "{read}"
    );
    // WHAT `check` CANNOT TELL IS SAID BESIDE IT: it cannot see a session.
    assert!(
        named["only"]
            .as_str()
            .is_some_and(|only| only.contains("NO SESSION OF AN ACT IS OPEN")
                && only.contains("check cannot see a session")),
        "{read}"
    );
    // WHILE A HELD COMMAND RUNS THE FILE IS WITHHELD: an act may be running.
    let candidate = &line["head"].as_str().expect("the opening's commit")[..12];
    let gate = format!("gate-{candidate}");
    let lock = lock_held(&root.join("scratch/hold").join(&gate).join("lock"));
    let read = canary.checked(&root);
    assert_eq!(
        json!([read["ruling"]["ruling"], read["ruling"]["prompt"]]),
        json!(["ruling-again", null]),
        "{read}"
    );
    assert!(
        read["ruling"]["withheld"]
            .as_str()
            .is_some_and(|why| why.contains("a held command still runs") && why.contains(&gate)),
        "{read}"
    );
    drop(lock);

    // THE RULING, AS ITS PROMPT FILE SAYS: the harness takes the object, records it and
    // starts nothing.
    let told = prompted(again);
    assert_eq!(told.args, again["args"], "{}", told.text);
    let ruled = canary.invoke_by(
        &told.harness,
        line["session_cwd"]
            .as_str()
            .expect("the session's directory"),
        &told.args,
        json!({}),
        3,
    );
    assert_eq!(ruled.result["status"], "ruled", "{}", ruled.result);
    assert!(
        ruled
            .agents
            .iter()
            .all(|label| label.starts_with("git:") || label.starts_with("record:")),
        "a ruling starts nothing: {:?}",
        ruled.agents
    );
    fs::write(&told.returns, ruled.result.to_string()).expect("write the ruling's return");
    let read = canary.checked(&root);
    assert_eq!(
        json!([read["ruling"], read["state"]["next"], read["records"]]),
        json!([null, "test", 1]),
        "{read}"
    );
}

/// A stand-in for git that writes down every command it is asked — where the child's
/// environment names a file to write to — and then is git.
const GIT_LOGGED: &str = r#"#!/bin/sh
if [ -n "${GIT_ARGV_LOG:-}" ]; then printf '%s\n' "$*" >>"$GIT_ARGV_LOG"; fi
exec "REAL_GIT" "$@"
"#;

/// **`check` says what is applied and in no commit, reads no field across it unsaid, and
/// takes no lock a stage could meet** (the review before part 1: its second finding, its
/// eighth, and what `check` can say of its fifth). The states are made by the clone's own
/// tools, with no stage: A BATCH APPLIED BY THE RECORD SCRIPT AND COMMITTED BY NOBODY — the
/// order's, as part 3's first act applies it — which `check` names under `applied` with
/// every field that was read across it, while the step tool's own read says `applied` is
/// owed; THE ONE CALL THAT TAKES IT BACK, run as printed; a batch under the subject of a
/// stage's first record, which is the record it will be, so that the act that reached it
/// is not said to have never run; a batch whose commit is made and that nobody settled; a
/// link and a directory among the filed verdicts, each said to be one, with the one call
/// that sets it aside, run as printed. AND EVERY GIT COMMAND `check` RUNS IS A READ THAT
/// TAKES NO LOCK: its `git status` is asked with no optional lock.
#[test]
fn check_says_what_is_applied_and_in_no_commit_and_takes_no_lock_a_stage_could_meet() {
    let canary = Canary::parts("parts-applied");
    // GIT ITSELF, by the path of its own binary — never by a name the stand-in would answer
    // to: on macOS the `git` on the `PATH` is a trampoline that looks its tool up by name.
    let core = canary
        .command(Path::new("git"))
        .arg("--exec-path")
        .output()
        .expect("ask git where its programs lie");
    let real = Path::new(String::from_utf8(core.stdout).expect("a path").trim_end()).join("git");
    assert!(real.is_file(), "git, by its path: {}", real.display());
    canary.rig.on_path(
        "git",
        &GIT_LOGGED.replace("REAL_GIT", &real.display().to_string()),
    );
    canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let clone = root.join("clone");
    let at = root.display().to_string();
    let fresh = canary.checked(&root);
    assert_eq!(
        json!([fresh["applied"], fresh["state"], fresh["ruling"]]),
        json!([null, {"next": "test", "round": 1, "attempt": 1}, null]),
        "{fresh}"
    );

    // A BATCH, APPLIED BY THE CLONE'S OWN RECORD SCRIPT — the order part 3 rules — under
    // `subject`, and committed by nobody.
    let apply = |subject: &str| {
        let batch = json!([{"argv": ["run-set", "--run", RUN],
                            "stdin": json!({"order": ORDER_THREE}).to_string()}]);
        let mut child = canary
            .command(&clone.join(RECORD))
            .args(["apply", "--run", RUN, "--subject", subject, "--scratch"])
            .arg(root.join("scratch"))
            .current_dir(&clone)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the clone's record script");
        child_stdin::feed(&mut child, batch.to_string());
        let out = child
            .wait_with_output()
            .expect("the clone's record script exits");
        assert!(
            out.status.success(),
            "the batch is applied: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    // One plain call, run as printed, from the clone, by a shell.
    let as_printed = |call: &str| {
        for token in SHELL {
            assert!(!call.contains(token), "`{call}` holds `{token}`");
        }
        let out = canary
            .command(Path::new("sh"))
            .args(["-c", call])
            .current_dir(&clone)
            .stdin(Stdio::null())
            .output()
            .expect("run the call");
        assert!(
            out.status.success(),
            "`{call}`: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    let rulings = format!("docs(record): {RUN} r0 - the rulings of the human");

    // (1) THE ORDER'S BATCH, APPLIED AND IN NO COMMIT. The control: the step tool's own read
    // says that `applied` is owed. `check` names the batch — and every field it read across.
    apply(&rulings);
    let looked = canary
        .command(&clone.join("dev/stabilize-step"))
        .args(["git-state", "--stage", "test", "--loop", LOOP, "--rounds"])
        .arg(format!("{LOOP}-r"))
        .args(["--run-dir", RUN_DIR, "--look"])
        .current_dir(&clone)
        .stdin(Stdio::null())
        .output()
        .expect("run the clone's step tool");
    assert_eq!(
        line_of(&String::from_utf8_lossy(&looked.stdout))["owed"],
        json!(["applied"]),
        "the control: the step tool says the batch is applied and in no commit"
    );
    let log = canary.rig.dir().join("git-argv");
    let told = canary.tool_in(
        &[("GIT_ARGV_LOG", &log.display().to_string())],
        &["check", "--root", &at],
    );
    let read = told.done("check", "checked").clone();
    let applied = &read["applied"];
    assert_eq!(
        json!([
            applied["subject"],
            applied["round"],
            applied["calls"],
            applied["files"],
            applied["filed"],
            applied["altered"],
            applied["in_commit"],
            applied["across"],
            applied["discard"]
        ]),
        json!([
            rulings,
            null,
            1,
            [format!("{RUN_DIR}/run.md")],
            [],
            [],
            false,
            [
                "state",
                "not_ready",
                "items",
                "ledger",
                "budget",
                "queue",
                "listed",
                "human",
                "filed",
                "view",
                "attempts"
            ],
            format!("dev/stabilize-step discard --branch {LOOP} --run-dir {RUN_DIR}")
        ]),
        "{read}"
    );
    let then = applied["then"].as_str().expect("what follows");
    assert!(
        then.contains("NO COMMIT HOLDS IT")
            && then.contains(
                "every field named under across says what the tables say with the batch applied"
            )
            && then.contains("records, trail, head and pushed say what is committed")
            && then.contains("whatever it carries")
            && then.contains("the one call under discard, run from the clone"),
        "{then}"
    );
    // EVERY FIELD NAMED THERE IS A FIELD OF THE LINE, and `state` says it of itself.
    for field in applied["across"].as_array().expect("the fields") {
        assert!(
            read.as_object()
                .expect("the line")
                .contains_key(field.as_str().expect("a field's name")),
            "`{field}` is a field of the line: {read}"
        );
    }
    assert_eq!(
        json!([
            read["state"],
            read["records"],
            read["pushed"],
            read["trail"]
        ]),
        json!([{"next": "test", "round": 1, "attempt": 1, "across": "applied"}, 0, true, []]),
        "what is committed is said as it is: {read}"
    );
    // (What the tables say with the batch applied: thirteen rows in the queue.)
    assert_eq!(read["queue"].as_array().expect("the queue").len(), 13);
    assert_eq!(
        json!([
            said_of(&read, "part-3-act-1")["stands"],
            said_of(&read, "part-2")["stands"]
        ]),
        json!(["not-written", "never-ran"]),
        "{read}"
    );
    assert_eq!(read["ruling"], Value::Null, "{read}");

    // (2) EVERY GIT COMMAND OF THAT `check` IS A READ THAT TAKES NO LOCK A STAGE COULD MEET
    // (the review's eighth finding: a plain `git status` may take the index's lock for a
    // moment, and a commit step that stages at that moment is refused): `git status` is
    // asked with no optional lock, and nothing else is run that writes or locks.
    let ran = fs::read_to_string(&log).expect("what git was asked");
    let ran: Vec<&str> = ran.lines().collect();
    assert!(ran.len() > 10, "{ran:?}");
    let statuses: Vec<&&str> = ran
        .iter()
        .filter(|command| command.split(' ').any(|word| word == "status"))
        .collect();
    assert!(
        statuses.len() >= 2
            && statuses
                .iter()
                .all(|command| command.starts_with("--no-optional-locks status --porcelain")),
        "{ran:?}"
    );
    for command in &ran {
        let verb = command
            .split(' ')
            .find(|word| !word.starts_with('-'))
            .expect("a git command");
        assert!(
            [
                "remote",
                "config",
                "rev-parse",
                "branch",
                "rev-list",
                "status",
                "log",
                "diff-tree",
                "cat-file",
                "worktree",
                // The write guard's probe: the clone's local refs, for its snapshot.
                "for-each-ref"
            ]
            .contains(&verb),
            "`git {command}` is no read `check` is known to make"
        );
    }

    // (3) THE ONE CALL THAT TAKES THE BATCH BACK, RUN AS PRINTED: nothing is applied, and
    // the line is the fresh root's again.
    as_printed(applied["discard"].as_str().expect("the call"));
    assert_eq!(canary.checked(&root), fresh);

    // (4) A BATCH THAT IS APPLIED IS THE RECORD IT WILL BE: under the subject of a stage's
    // first record, the act that makes that record ran — to its record's gate — and is not
    // said to have never run. (On the main line the same holds for the record of part 4's
    // fourth act, which the acts' own arm drives.)
    apply(&format!(
        "docs(record): {RUN} r1 - the findings of the test stage"
    ));
    let read = canary.checked(&root);
    assert_eq!(
        json!([
            said_of(&read, "part-2")["shows"],
            said_of(&read, "part-2")["stands"],
            read["records"]
        ]),
        json!([true, "not-written", 0]),
        "{read}"
    );

    // (5) A BATCH WHOSE COMMIT IS MADE, AND THAT NOBODY SETTLED: every file of it is in HEAD
    // as the batch wrote it. `check` says so, and reads nothing across it.
    canary.git(&clone, &["add", "--", &format!("{RUN_DIR}/run.md")]);
    canary.git(&clone, &["commit", "-q", "-m", "docs(record): by hand"]);
    let read = canary.checked(&root);
    assert_eq!(
        json!([
            read["applied"]["in_commit"],
            read["applied"]["across"],
            read["applied"]["discard"],
            read["state"]["across"],
            read["records"]
        ]),
        json!([true, null, null, null, 1]),
        "{read}"
    );
    assert!(
        read["applied"]["then"]
            .as_str()
            .is_some_and(|then| then.contains("its commit is made")
                && then.contains("settles it at its first read")),
        "{read}"
    );
    as_printed(&format!("{RECORD} settle --run {RUN}"));
    assert_eq!(canary.checked(&root)["applied"], Value::Null);

    // (6) A LINK, AND A DIRECTORY, AMONG THE FILED VERDICTS — a hand's doing (the review's
    // fifth finding, which is the record script's and stays as recorded): the record script
    // cannot say what is pending over the link, so NO INVOCATION GETS PAST ITS FIRST READ,
    // and `check` said nothing of it. It says so now — a refusal of that read is a state —
    // and says of each entry what it is, with THE ONE CALL THAT SETS IT ASIDE, which is run
    // here as printed.
    let verdicts = clone.join(RUN_DIR).join("r1/verdicts");
    fs::create_dir_all(verdicts.join("notes")).expect("a directory among the filed verdicts");
    std::os::unix::fs::symlink(root.join("canary.json"), verdicts.join("more.json"))
        .expect("a link among the filed verdicts");
    let read = canary.checked(&root);
    let refused = &read["applied"];
    assert!(
        refused["refused"].as_str().is_some_and(|said| said
            .starts_with("stabilize-record: refused outside: ")
            && said.contains("r1/verdicts/more.json is a link")),
        "{read}"
    );
    assert!(
        refused["then"]
            .as_str()
            .is_some_and(|then| then.contains("none gets past it while this stands")
                && then.contains(&format!("{RECORD} set-aside --run {RUN} -- PATH"))),
        "{read}"
    );
    let entries = read["filed"].as_array().expect("what is filed");
    assert_eq!(
        entries
            .iter()
            .map(|entry| json!([entry["file"], entry["no_file"], entry["set_aside"]]))
            .collect::<Vec<_>>(),
        [("more.json", "link"), ("notes", "directory")]
            .iter()
            .map(|(name, what)| json!([
                format!("{RUN_DIR}/r1/verdicts/{name}"),
                what,
                format!("{RECORD} set-aside --run {RUN} -- {RUN_DIR}/r1/verdicts/{name}")
            ]))
            .collect::<Vec<_>>(),
        "{read}"
    );
    for entry in entries {
        as_printed(entry["set_aside"].as_str().expect("the call"));
    }
    let read = canary.checked(&root);
    assert_eq!(
        json!([read["applied"], read["filed"]]),
        json!([null, []]),
        "{read}"
    );
}

/// **`check` reads a write that was killed as a state, and no refusal of a tool it calls
/// says that the root holds no canary** (a finding of the scoped review before part 1, in
/// the class that strands state: `check` ran the clone's `state` as a command that must
/// succeed, and turned every refusal of it into the SETUP's advice — *the root holds no
/// canary: a setup is asked for again under another root*). The record script does not read
/// a run's state across a write that was killed between its files (`interrupted`) — and
/// that is a state the next invocation finishes by itself, met exactly where part 4 has the
/// human run `check`: after a session was ended. SO `check` EXITS 0 OVER IT: it says
/// `refused: interrupted` for the state, hands on the record script's own line and what
/// follows, still prints what git says of the clone and each act's return, and says nothing
/// the state alone could say. ANY OTHER REFUSAL of the record script, and a git command that
/// fails, is `tool` — with a recommendation that says the canary stands and asks for no
/// setup. ONLY A SETUP THAT FAILED says that its root holds no canary: that one does not.
#[test]
fn check_reads_a_killed_write_as_a_state_and_never_says_of_a_canary_that_the_root_holds_none() {
    let canary = Canary::parts("parts-killed-write");
    let line = canary.ready("canary", &[PARTS]);
    let root = canary.root("canary");
    let clone = root.join("clone");
    let check = || canary.tool(&["check", "--root", &root.display().to_string()]);
    let before = check().done("check", "checked").clone();

    // A WRITE KILLED BETWEEN ITS FILES, as the record script's journal says one: an entry
    // that is still being placed. THE CONTROL: the clone's own record script refuses to
    // read the state across it, by its word `interrupted`.
    let journal = clone.join(RUN_DIR).join(".pending.json");
    let killed = json!({"placing": [{"path": "ledger.md", "temporary": ".stabilize-record.killed.tmp", "replace": true}],
                        "batch": null});
    fs::write(&journal, format!("{killed}\n")).expect("plant the journal of a killed write");
    let state = canary
        .command(&clone.join(RECORD))
        .args(["state", "--run", RUN])
        .current_dir(canary.rig.dir())
        .stdin(Stdio::null())
        .output()
        .expect("run the clone's record script");
    let said = String::from_utf8_lossy(&state.stderr).into_owned();
    assert!(
        state.status.code() == Some(17)
            && said.starts_with("stabilize-record: refused interrupted: "),
        "the record script does not read the state across a killed write: {said}"
    );

    // `check` READS IT AS A STATE: done, exit 0 — the record script's line, what follows,
    // and no word of a root that holds no canary.
    let told = check();
    let read = told.done("check", "checked");
    assert_eq!(
        read["state"],
        json!({"next": null, "round": null, "attempt": null, "refused": "interrupted"}),
        "{read}"
    );
    assert_eq!(
        read["interrupted"]["said"],
        json!(said.split_whitespace().collect::<Vec<_>>().join(" ")),
        "{read}"
    );
    let then = read["interrupted"]["then"]
        .as_str()
        .expect("what follows from a killed write");
    assert!(
        then.contains("the next invocation of the stage finishes that write")
            && then.contains("by itself")
            && then.contains("no setup is asked for"),
        "{then}"
    );
    for wrong in ["holds no canary", "asked for again", "under another root"] {
        assert!(
            !told.raw.contains(wrong),
            "`check` says `{wrong}`: {}",
            told.raw
        );
    }
    // WHAT GIT SAYS OF THE CLONE IS READ AS EVER — the journal is the one pending file —
    // and what only the state could say is empty or null: nothing is guessed across it.
    assert_eq!(
        json!([
            read["head"],
            read["pushed"],
            read["records"],
            read["pending"],
            read["pending_paths"],
            read["trail"]
        ]),
        json!([
            line["head"],
            true,
            0,
            1,
            [format!("{RUN_DIR}/.pending.json")],
            []
        ]),
        "{read}"
    );
    assert_eq!(
        json!([
            read["not_ready"],
            read["items"],
            read["ledger"],
            read["budget"],
            read["queue"],
            read["listed"],
            read["human"],
            read["filed"],
            read["view"],
            read["attempts"]
        ]),
        json!([null, [], [], null, null, null, null, null, null, null]),
        "{read}"
    );
    for act in read["acts"].as_array().expect("the acts") {
        assert_eq!(
            json!([act["returned"], act["shows"], act["stands"]]),
            json!([false, null, "cannot-tell"]),
            "of no act is it said whether the run shows it: {act}"
        );
    }
    assert_eq!(read["sheet"], before["sheet"], "{read}");

    // THE WRITE FINISHED — here the plant removed — AND `check` IS WHAT IT WAS.
    fs::remove_file(&journal).expect("remove the plant");
    assert_eq!(check().done("check", "checked"), &before);

    // ANY OTHER REFUSAL OF THE RECORD SCRIPT IS `tool`, IN ITS OWN WORDS — here a journal
    // that is none of that script's — and what follows is `check`'s to say, not a setup's:
    // the canary stands, and no setup is asked for.
    let stands = |told: &Told, evidence: &str| {
        let refused = told.refused("tool");
        let advice = refused["halt"]["recommendation"]
            .as_str()
            .expect("a recommendation");
        assert!(
            advice.contains("THE CANARY STANDS AS IT STOOD")
                && advice.contains("NO SETUP IS ASKED FOR"),
            "{advice}"
        );
        assert!(
            refused["halt"]["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.starts_with("tool: a command the check runs failed: ")),
            "{refused}"
        );
        assert!(
            refused["halt"]["evidence"]
                .as_str()
                .is_some_and(|said| said.contains(evidence)),
            "the evidence is the command's own words, `{evidence}`: {refused}"
        );
        for wrong in ["holds no canary", "asked for again", "under another root"] {
            assert!(
                !told.raw.contains(wrong),
                "`check` says `{wrong}`: {}",
                told.raw
            );
        }
    };
    fs::write(&journal, "no journal of the record script\n").expect("plant a file");
    stands(&check(), "stabilize-record: refused corrupt: ");
    fs::remove_file(&journal).expect("remove the plant");
    // … AND SO IS A GIT COMMAND THAT FAILS IN THE CLONE: here its `HEAD` names nothing.
    let head = clone.join(".git/HEAD");
    let was = fs::read(&head).expect("the clone's HEAD");
    fs::write(&head, "ref: refs/heads/no-such-branch\n").expect("bend the clone's HEAD");
    stands(&check(), "rev-parse HEAD");
    fs::write(&head, was).expect("put the clone's HEAD back");
    assert_eq!(check().done("check", "checked"), &before);

    // THE SAME FOR A ROOT WITHOUT THE SWITCH: a killed write is a state there too, and the
    // line gains the one field that says so.
    let small = Canary::new("small-killed-write");
    small.ready("canary", &[]);
    let at = small.root("canary");
    fs::write(
        at.join("clone").join(RUN_DIR).join(".pending.json"),
        format!("{killed}\n"),
    )
    .expect("plant the journal of a killed write");
    let told = small.tool(&["check", "--root", &at.display().to_string()]);
    let read = told.done("check", "checked");
    assert_eq!(
        json!([
            read["state"]["refused"],
            read["interrupted"]["then"],
            read["returned"]
        ]),
        json!(["interrupted", then, {"first": false, "then": false}]),
        "{read}"
    );
    assert!(!told.raw.contains("holds no canary"), "{}", told.raw);

    // THE CONTROL — A SETUP THAT FAILED HOLDS NO CANARY, AND SAYS SO: only there.
    let told = canary.setup(&canary.root("failed"), &[PARTS, "--order", "route:nowhere"]);
    assert!(
        told.refused("tool")["halt"]["recommendation"]
            .as_str()
            .is_some_and(|advice| advice.contains("The root holds no canary")),
        "{}",
        told.raw
    );
}

/// The route line the first planted row quotes of the second refusal its drive meets — typed
/// here a second time, from that drive on the candidate (2026-10-17): the hook is then a link
/// that leads to nothing, and the line names the repository's own hook as the link.
const FOLLOW_UP_ROUTE: &str = "route: remove the link at `<rig>/repo/.git/hooks/pre-commit`, or point it at a script inside this repository, then re-run `jigc setup`. `--force` does not change this: it consents to replacing a file, not to following a link";
/// The plan's name for the item that route is, and what its own verifier should return.
const FOLLOW_UP_PLAN: &str = "plant-install-hook-follow-up";
const FOLLOW_UP_RETURNS: &str = "refuted: does-not-reproduce";

/// **Part 3 hands one item on for certain: the first planted row's drive meets it, its verifier
/// may hand it on under the rules it already has, and `check` says how it fell** (the human's
/// ruling of 2026-10-16; DECISIONS.md → *2026-10-17 — Part 3 of the staged canary hands one
/// item on for certain*). No act of the canary had run the live chain of a hand-on. THE ROW
/// QUOTES THE ITEM WITH ITS KIND'S EVIDENCE — the route line of the second refusal its drive
/// meets, and that refusal's code — and says that nobody drove it, and nothing of how it falls.
/// FILED AS A VERIFIER HANDS AN ITEM ON, by the clone's own record script under the clone's own
/// harness, THE ITEM ENTERS THE QUEUE AHEAD OF EVERY RESERVE ROW; and once its own verifier
/// has filed, `check` says how it fell beside what the sheet says it should return. A verifier
/// that hands on another item beside it, within the number, is allowed: `check` names the one
/// the row quotes wherever it stands. THE SHEET CARRIES ITS ANSWER, AND THE CLONE'S TREE DOES
/// NOT: a file of the commit named that names it and no planted row is withheld there.
///
/// HERE THE ROOT IS THE ONE THE PLAN OPENS FOR ITS FOURTH PART AFTER A REPAIR — the gate as the
/// one item, the order part 3 rules, a budget of four — so that the stage goes from its first
/// record into its first wave: the three route rows and the first reserve row.
#[test]
fn the_first_rows_drive_meets_an_item_its_verifier_hands_on_and_check_says_how_it_fell() {
    if !node_or_skip(&format!("no stage of {HARNESS} was run on a canary")) {
        return;
    }
    let mut canary = Canary::parts("parts-follow-up");
    // A FILE OF THE COMMIT NAMED THAT NAMES THE ITEM BESIDE ITS ANSWER, AND NO PLANTED ROW.
    canary.rig.write(
        "notes/follow-up.md",
        &format!("{FOLLOW_UP_PLAN}: {FOLLOW_UP_RETURNS}\n"),
    );
    canary.commit = canary
        .rig
        .commit("docs: an item a row's verifier may hand on, and its answer");
    let line = canary.ready(
        "canary",
        &[
            PARTS,
            "--items",
            "gate",
            "--budget",
            "4",
            "--order",
            ORDER_THREE_FLAG,
        ],
    );
    let root = canary.root("canary");
    let clone = root.join("clone");
    let (route, reserve) = route_and_reserve();

    // (1) THE FIRST ROW QUOTES THE ITEM, WITH ITS KIND'S EVIDENCE: the second refusal at the
    // same code, its route line whole — and that nobody drove it. Its claim still opens the
    // row, and the report it cites still closes it.
    let state = canary.state(&clone);
    let first = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .find(|row| row["key"] == route[0])
        .unwrap_or_else(|| panic!("the first planted row: {state}"));
    let repro = first["repro"].as_str().expect("a repro");
    for said in [
        format!("refuses once more at `{CODE}`"),
        "a link that leads to nothing".to_owned(),
        format!("{FOLLOW_UP_ROUTE}. Nobody drove that route."),
    ] {
        assert!(repro.contains(&said), "`{said}`: {repro}");
    }
    assert!(
        repro.starts_with(&format!("The claim: {}. The state:", PLANTS[0].2))
            && repro.ends_with(&format!("of {CITED}.")),
        "{repro}"
    );
    // The row's own class is its own route, and no other.
    assert_eq!(first["class"]["evidence"], ROUTE_LINK, "{first}");

    // (2) FILED AS A VERIFIER HANDS AN ITEM ON — in the shape the record script takes, built
    // from what the row quotes — beside ANOTHER item, put first: within the number, allowed.
    let quoted = json!({"key": "setup hook link to nothing route", "doctype": "jigc-feedback",
                        "door": PARTS_DOOR, "clause": "working-product",
                        "repro": "the verifier's report, its own heading",
                        "kind": "route", "route": FOLLOW_UP_ROUTE, "code": CODE});
    let other = handed("setup hook link route other", "route: do as the line says");
    let mut agents = json!({});
    agents[format!("verify:{}", route[0])] = files(json!({
        "verdict": "confirmed", "regression": true, "hands_on": [other, quoted],
    }));
    agents[format!("verify:{}", route[1])] = refutes();
    agents[format!("verify:{}", route[2])] =
        files(json!({"verdict": "undrivable", "reason": "needs-a-linux-host"}));
    agents[format!("verify:{}", reserve[0])] = refutes();
    let wave_1 = canary.act(
        &line,
        "part-2",
        json!({"scope": parts_scope(), "agents": agents}),
        1,
    );
    assert_eq!(
        verified(&wave_1),
        [route.clone(), vec![reserve[0]]].concat(),
        "{:?}",
        wave_1.agents
    );
    let (minted, beside_it) = (
        "setup-hook-link-to-nothing-route",
        "setup-hook-link-route-other",
    );

    // THE TWO ITEMS ENTER THE QUEUE AHEAD OF EVERY RESERVE ROW, in the verifier's order — the
    // budget is spent, so each is routed to the human as unreached, as part 3 leaves a row
    // nobody reached — and `check` says what was handed on, and which item the row quotes.
    let read = canary.checked(&root);
    assert_eq!(
        read["queue"],
        json!([vec![beside_it, minted], reserve[1..].to_vec()].concat()),
        "{read}"
    );
    let hand_on = &read["hand_on"];
    assert_eq!(
        json!([hand_on["under"], hand_on["beside"]]),
        json!([route[0], 1]),
        "{read}"
    );
    let items: Vec<Value> = hand_on["items"]
        .as_array()
        .unwrap_or_else(|| panic!("`check` says what the first row's verifier handed on: {read}"))
        .iter()
        .map(|item| {
            json!([
                item["key"],
                item["kind"],
                item["origin"],
                item["beyond"],
                item["route"],
                item["rank"],
                item["follows"],
                item["verdict"]
            ])
        })
        .collect();
    assert_eq!(
        items,
        [
            json!([
                beside_it,
                "route",
                "under-confirmed",
                false,
                "human",
                0,
                false,
                null
            ]),
            json!([
                minted,
                "route",
                "under-confirmed",
                false,
                "human",
                1,
                true,
                null
            ])
        ],
        "{read}"
    );
    assert_eq!(
        json!([
            hand_on["follows"]["key"],
            hand_on["follows"]["returns"],
            hand_on["follows"]["as_planted"]
        ]),
        json!([minted, FOLLOW_UP_RETURNS, null]),
        "{read}"
    );
    // THE SHEET CARRIES ITS ANSWER, beside the first row's — and the key its verifier minted.
    let sheet = read["sheet"].as_array().expect("the sheet");
    assert_eq!(
        sheet[0]["hands_on"],
        json!({"plan": FOLLOW_UP_PLAN, "key": minted, "class": "route under-confirmed",
               "returns": FOLLOW_UP_RETURNS}),
        "{read}"
    );
    assert!(
        sheet[1..].iter().all(|row| row["hands_on"].is_null()),
        "{read}"
    );

    // (3) THE BUDGET RAISED AS PART 4'S PROMPT FILE SAYS, AND THE STAGE CONTINUED: the wave is
    // the first five of the queue — the two items, then the reserve. The item the row quotes
    // is refuted as one that does not reproduce: AS PLANTED.
    let raised = canary.act(&line, "part-4-act-1", json!({}), 2);
    assert_eq!(
        raised.result["budget"],
        json!({"limit": 9, "spent": 4, "left": 5}),
        "{}",
        raised.result
    );
    let mut agents = json!({});
    for key in [beside_it, minted, reserve[1], reserve[2], reserve[3]] {
        agents[format!("verify:{key}")] = refutes();
    }
    let wave_2 = canary.act(&line, "part-4-act-4", json!({"agents": agents}), 3);
    assert_eq!(
        verified(&wave_2),
        [beside_it, minted, reserve[1], reserve[2], reserve[3]],
        "{:?}",
        wave_2.agents
    );
    let read = canary.checked(&root);
    let follows = &read["hand_on"]["follows"];
    assert_eq!(
        json!([
            follows["key"],
            follows["rank"],
            follows["verdict"],
            follows["as_planted"]
        ]),
        json!([minted, null, FOLLOW_UP_RETURNS, true]),
        "{read}"
    );
    assert!(follows["attempt"].is_u64(), "{read}");
    assert_eq!(read["queue"], json!(reserve[4..]), "{read}");

    // (4) THE CLONE'S TREE DOES NOT CARRY ITS ANSWER: the file of the commit named that names
    // the item beside it is withheld there — while the commit itself pairs the two, in that
    // file and in the tool — and `check` asks the reading about it by its path.
    let names = [FOLLOW_UP_PLAN.to_owned(), FOLLOW_UP_ROUTE.to_owned()];
    let falls = ["refuted".to_owned(), "does-not-reproduce".to_owned()];
    let source: Vec<(String, String)> = texts_below(&canary.rig.root)
        .into_iter()
        .filter(|(file, _)| !file.starts_with(".git/"))
        .collect();
    let paired = beside(&source, &names, &falls);
    for place in ["notes/follow-up.md", TOOL] {
        assert!(
            paired.iter().any(|(file, ..)| file == place),
            "the commit named pairs the item with its answer in `{place}`"
        );
    }
    assert!(
        line["withheld"]
            .as_array()
            .is_some_and(|withheld| withheld.contains(&json!("notes/follow-up.md"))),
        "{line}"
    );
    let skip = [".git/".to_owned(), format!("{RUN_DIR}/")];
    let tree: Vec<(String, String)> = texts_below(&clone)
        .into_iter()
        .filter(|(file, _)| !skip.iter().any(|prefix| file.starts_with(prefix.as_str())))
        .collect();
    assert_eq!(
        beside(&tree, &names, &falls),
        Vec::<(String, usize, String, String)>::new(),
        "the clone's tree names the item beside its answer"
    );
    assert!(
        read["to_answer"][0]
            .as_str()
            .is_some_and(|question| question.contains("notes/follow-up.md")),
        "{read}"
    );
}
