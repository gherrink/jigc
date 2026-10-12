//! **A stage of the stabilization workflow, run** — the committed harness
//! (`.claude/workflows/stabilize.js`), unmodified, through a whole `test` stage, against a
//! stand-in for the Workflow runtime in which every agent is a function
//! ([DECISIONS.md](../DECISIONS.md) → *2026-10-06 — The `test` stage of the stabilization
//! workflow, simulated*; the repair plan's task 2,
//! [repair-plan.md](../completions/artifacts/M55/stabilization-build/repair-plan.md)).
//!
//! The review of the harness found that *no committed test executes either stage*
//! ([harness-agents-gate.md](../completions/artifacts/M55/stabilization-build/harness-agents-gate.md),
//! `M10`): the fence reads the script, and its self-test launches nothing. Two builders
//! and a reviewer each wrote a simulation to check their work, and threw it away. This is
//! that simulation, kept — so that a repair starts by making its finding fail here.
//!
//! **What is real.** The harness script, as committed. `dev/stabilize-record` and
//! `dev/stabilize-step`, as committed: every report, every row and every git act of a
//! stage goes through them. Git: a throwaway repository with a bare remote
//! ([`StepRig`]), a fixed identity and fixed dates, hooks off, no network. And the
//! commands themselves: an agent here runs what its prompt spells, taken out of the prompt
//! — never a command this suite rebuilt.
//!
//! **What is stubbed, and why** — only what cannot run inside a test:
//!
//! - *The long commands a stage holds — the full gate, the candidate's and a record step's;
//!   the regression set; cargo.* The gate runs this suite, and the other two take the
//!   better part of an hour. Each is a stand-in ([`LONG_COMMAND`], [`CARGO`]) placed where
//!   the step tool finds the real one, and prints what a scenario scripts: a gate green,
//!   red with its steps and tests, or with no verdict; the regression tool's one line; a
//!   build that fails. **What starts, holds and judges it is real** (the second repair
//!   plan's `K11`): `dev/stabilize-step hold-start` and `hold-wait`, the supervisor's fork
//!   and its lock, the verdict read by `dev/stabilize-record gate-check`, `git archive`,
//!   the binary placed read-only — and so is what reads the gate's file afterwards:
//!   `gate-set`, and the commit step that holds one gate to the other.
//! - *gitleaks, of the two hygiene scanners.* A test runner has none; the stand-in is the
//!   record suite's, which keeps gitleaks' exit contract
//!   ([`gitleaks_stub`](super::dev_stabilize_record::gitleaks_stub)), which every
//!   [`StepRig`] carries since a step vets what it would commit or push. The denylist half
//!   is the real `dev/hygiene-scan`, over the rig's denylist of one term.
//! - *The build of a trial image, and what a binary is.* A "binary" is the file the
//!   stand-in for cargo left and the step tool placed. Its sha256 is real — the tool's,
//!   read off its verdict by the harness, and asked of the tool again by every stand-in
//!   that "drives" it, through the call its prompt spells — so the comparison the harness
//!   makes of the two is a comparison of two measurements.
//! - *The instruments' own work*: reviewing, driving, grading, verifying, arguing. What an
//!   agent returns is the scenario's script; that it reports through the record script
//!   under the name it was handed is not.
//! - *The opening's gate, and the human.* The run is opened by the record script's own
//!   calls and one commit, as the workflow's doc has it; no gate precedes that commit here.
//!
//! **The stand-in for the runtime** is `tooling-tests/fixtures/stabilize-runtime.mjs`;
//! its header says what each kind of agent does. **Where it is not the runtime**, and
//! what therefore stays the canary's to show:
//!
//! - `parallel` runs its thunks one after another, in order. Nothing is concurrent, so
//!   nothing here shows what two agents do to one tree at once — the preflight beside the
//!   scope step, whose scope file the preflight's own assert may or may not admit, is
//!   exactly that.
//! - A return is held to the call's schema by a small validator; the runtime's validation
//!   and a model's retries are its own. No agent dies or is retried unless scripted, and
//!   there is no resume.
//! - The script runs as the body of an async function in a context of its own, with the
//!   language's built-ins and no clock; that the runtime accepts the script is not shown.
//! - **Scripted agents are not agents.** That a model runs one command, writes a payload
//!   byte for byte, or reads its definition as this stand-in does, is shown by nothing here.
//!   **One thing a real relay does is modelled, because it was observed** (the relay probe,
//!   2026-10-07): an agent's structured return decodes the `\uXXXX` escapes of the line it
//!   relays. The stand-in's git steps do that to every line of a stage's step — so a line
//!   that holds one comes back with other bytes, as it did in the runtime, and the harness
//!   as it was before the second repair plan's `K3` read no state here. A step's digest
//!   holds none. And a scenario can have a step hand back the line the tool prints unasked,
//!   a digest with one character changed, or one with a character outside ASCII and its
//!   hash made to fit (`relays`): each is caught, and nothing of it is passed on. A probe's
//!   steps are relayed byte for byte, as before: their lines are the probe tool's.
//!
//! **Every command an agent runs is taken out of its prompt, as it is spelled there**
//! (`K11`) — a step's one command, a report's hand-over, the scope's, the batch's, a hash's
//! — so a prompt that names an act the tool does not have, or a flag it does not take,
//! fails the run at that command; a file an agent writes is written as a file tool writes
//! one; and every prompt an agent is handed is held by the prompt fence
//! ([`prompt_faults`](super::stabilize_harness_fence::prompt_faults)): each command in it
//! is one plain invocation of one of three tools, and it tells no agent to wait, to
//! redirect or to pipe by its own means.
//!
//! **What every agent reads by key is read** (`K3`): a prompt names the read — an item's
//! row with its brief, the doors of a round an item covers, the untriaged rows, a pending
//! batch — and the stand-in runs each as the prompt spells it, fails the run where one is
//! refused, and reports what it printed; and every prompt an agent was handed is reported,
//! so that a test can hold that no text of the record is in one.
//!
//! **The trace is a golden** (`tooling-tests/fixtures/stabilize-test-stage.trace`): every
//! phase, every agent launched — its label, its definition, its model, its phase — and
//! every `parallel`, in order. It is re-captured with `UPDATE_GOLDENS=1 cargo test -p jigc
//! stabilize_simulation::`, which CI refuses. **A change to it is a change to what a stage
//! does** — a step added, dropped, moved or handed to another agent — so a diff of that
//! file is adjudicated against the ruling that asked for it, and never re-pinned to make a
//! run green.
//!
//! **Defects the reviews recorded are pinned where a stage meets them, and repaired by
//! nothing here.** Each such assertion says what the stage does *today* and names the
//! finding by its id; the task that repairs it turns the assertion. **Of the first
//! review's, no pin of the `test` stage is left** (the one the second repair left is named
//! below, at `X1`). *Turned by the repair's last task of that stage's harness, and asserted
//! as repaired:* `M3` — a contested fix is a row of the round's triage record, and the
//! human's until ruled; `M6` — the invocation that finishes a round's triage reaches its
//! record, on the candidate the round tested; `M5` and `M9` — an agent that dies, halts,
//! writes and dies, or returns with no report voids what it was launched for and never the
//! stage, a cross-model pass that fails is that pass not run, and an attempt is on record
//! from its first step. With them the scenarios this suite could not drive before: every
//! record is written through the harness, and no test calls into the record script but to
//! open a run and to read it. **Turned by the repair's task 6:** `F3` of the state machine's
//! review — the laps of a triage that is not finished are counted on record, and after
//! one more the next step is the human's, who rules or grants one more; and a stage that
//! halts before its record twice in a row is the human's too. **Turned by the repair's task 3,
//! and asserted as repaired:** `M2` (a record step that returns no evidence), `M4` (a record
//! step under a red gate) and the first of the build record's *Found while the workflow's
//! doc was written* (the scope file of a stage that stopped). **Turned by its task 4:** the
//! lead *a red check without a ledger row* — the call that records a red check files it as
//! a finding. And that task's own path is driven beside them: a check that could not run,
//! the re-run the state then asks for — an attempt inside the round, with no scope step and
//! no fact of the round — and the invocations the state asks no re-run of.
//!
//! **A return that lacks a field the stage needs of its role** (the second repair plan's
//! `X1`; the re-review's `R-H2`) is driven per cell of the harness's `NEEDS`
//! ([`ABSENT_ARMS`], which the fence holds to that table): the field left out — and, for a
//! hash, returned wrong — voids the item, leaves the finding unverified or the fork off the
//! record, and the stage reaches its record. **The two cells that task left halting are
//! turned (the plan's `X1b`):** a confirmed verdict that does not say whether the finding
//! is a regression, or whose hash of the previous release's binary is absent or another
//! (`L11`, below), is recorded as CONFIRMED WITH ITS REGRESSION FACT UNKNOWN — never as
//! unverified and never as *not a regression* — and the state names the finding.
//!
//! **A run with a queue takes another path from its triage on** (the build plan of the
//! reshaped `test` stage, §4, Phase B; its task `T5a`): the scenarios on an opening WITH AN
//! ORDER ([`Sim::queued`]) — every other scenario opens without one and runs the path it ran,
//! unmodified. There the stage has ONE triage, and its stand-in does what the prompt spells:
//! it holds the prompt to numbering every finding the call names, writes the scripted entries
//! to ONE file, and hands it over by the prompt's own `dev/stabilize-record triage` call — so
//! the partition, the rendered report and every refusal are the real script's. **Then the
//! stage's FIRST RECORD, composed by the tool** (the plan's `T5b`; the harness half of `K3`):
//! no executor is launched and no batch is typed — one step of the tool is handed PLAIN WORDS
//! and the sha256 of their canonical form, which the harness computed and the record script
//! computes again over what the step typed — then the record's gate, its one commit and its
//! push, as on the other path. **What of that path is built ends there**
//! ([`ends_at_its_first_record`]): the stage returns `hunted`, nothing is verified, and an
//! invocation that would CONTINUE — the state's position says `verify`, or `triage` — is
//! refused on that path as not built, with nothing run beyond the two reads.
//!
//! **The `fix` stage refuses to start**, and that is all that is asserted of it; the rig,
//! the scenario and the stand-in are shaped so that its half's tasks add its agents here.
//!
//! **The runtime probes are invoked here too** (the second repair plan's task `K0`;
//! [DECISIONS.md](../DECISIONS.md) → *2026-10-07 — The runtime probes of the stabilization
//! harness*): each of the four, through the committed harness and the real
//! `dev/stabilize-probe`, to `probed` with one judged result per case — and held to
//! touching no run, no branch, no remote and no file of the repository. What a probe exists
//! to find out is exactly what a stand-in cannot show: here the scenario SAYS what the
//! runtime does with a return that lacks a required field (`violation`), every line is
//! relayed whole unless a scenario garbles it, and a hold is run in the foreground. The
//! script side of each verdict is [`dev_stabilize_probe`](super::dev_stabilize_probe)'s.
//!
//! **Without `node`** the suite cannot run a stage: it fails under CI and passes anywhere
//! else, and the gate's summary says which tests passed without running
//! ([`node_or_skip`]).

use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::{Value, json};

use crate::support::child_stdin;
use crate::support::goldens::update_mode;

use super::dev_stabilize_record::DENY_TERM;
use super::dev_stabilize_step::{
    GATE_KIND, Held, LONG_COMMAND, LOOP, RUN, RUN_DIR, StepRig, as_recorded, failing_commit,
    harness_pure, node_or_skip, sha256,
};
use super::placed_executable;
use super::rig_processes::RigDir;
use super::stabilize_harness_fence::prompt_faults;

const HARNESS: &str = ".claude/workflows/stabilize.js";
const RUNTIME: &str = "tooling-tests/fixtures/stabilize-runtime.mjs";
const TRACE: &str = "tooling-tests/fixtures/stabilize-test-stage.trace";
const RECORD: &str = "dev/stabilize-record";

/// The previous release every simulated run measures against.
const PREVIOUS: &str = "1.0.0-rc.24";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

/// Whether this machine can run a stage at all; where it cannot, the test passes and says so.
fn can_run() -> bool {
    node_or_skip(&format!("no stage of {HARNESS} was run by the simulation"))
}

// ---------------------------------------------------------------------------
// The rig: a repository a run is opened in, and a stage invoked on
// ---------------------------------------------------------------------------

/// What an opening settles, as the rows the record script is handed.
struct Opening {
    clauses: Vec<&'static str>,
    items: Value,
    bounds: Vec<&'static str>,
    rows: Value,
}

/// An item of the test set that runs on every candidate.
fn item(id: &str, kind: &str, clause: &str) -> Value {
    json!({"item": id, "kind": kind, "clause": clause, "runs": "every-candidate",
           "brief": format!("the brief of {id}")})
}

/// An item that runs only in a round whose doors reach it.
fn in_scope(id: &str, kind: &str, clause: &str, doors: &[&str], registries: &[&str]) -> Value {
    json!({"item": id, "kind": kind, "clause": clause, "runs": "in-scope",
           "brief": format!("the brief of {id}"), "doors": doors, "registries": registries})
}

fn door(name: &str, registry: &str) -> Value {
    json!({"door": name, "registry": registry, "derivation": format!("scripted: {name}")})
}

/// The smallest opening a stage can run on: one audit area, on every candidate.
fn one_area() -> Opening {
    Opening {
        clauses: vec!["audit-clean"],
        items: json!([item("area-a", "audit-area", "audit-clean")]),
        bounds: vec![],
        rows: json!([]),
    }
}

/// The scope the small scenarios resolve: one door inside, none outside.
fn one_door() -> Value {
    json!({"included": [door("jigc doc set", "verbs")], "excluded": [],
           "uncovered": [], "reached_but_excluded": [], "left_open": []})
}

/// A reporter that found nothing.
fn nothing() -> Value {
    json!({"status": "reported", "findings": [], "summary": "Nothing found."})
}

fn finding(id: &str, title: &str, door: &str, clause: &str) -> Value {
    json!({"id": id, "title": title, "door": door, "clause": clause,
           "repro": format!("the block {id}")})
}

/// A reporter that found these.
fn found(findings: &[Value]) -> Value {
    json!({"status": "reported", "findings": findings, "summary": "Found."})
}

/// A triage entry for a finding this stage's reporters returned: a key minted here.
fn entry(key: &str, source: &str, door: &str, clause: &str, grade: &str) -> Value {
    json!({"key": key, "new": true, "doctype": "jigc-feedback", "source": source,
           "door": door, "clause": clause, "repro": format!("the report of {source}"),
           "grade": grade})
}

/// Triage's return over `entries`, each from one finding of one of `reporters`.
fn graded(entries: &[Value], reporters: &[&str]) -> Value {
    json!({
        "status": "graded",
        "entries": entries,
        "to_verify": [],
        "counts": {
            "findings_in": reporters.iter().map(|r| json!({"reporter": r, "count": 1})).collect::<Vec<_>>(),
            "entries": entries.len(),
            "merged": 0,
        },
    })
}

/// One invocation, as the runtime's stand-in reports it.
struct Ran {
    /// What the script returned to the orchestrator.
    result: Value,
    /// Every phase, every agent launched and every `parallel`, in order.
    trace: Vec<String>,
    /// What the script logged.
    logs: Vec<String>,
    /// The reporters launched, by the name each one's prompt handed it.
    reporters: Vec<String>,
    /// Every command the stand-in ran on an agent's behalf, by its first line.
    commands: Vec<String>,
    /// Every prompt an agent was handed: `(its label, the prompt)`, in order.
    prompts: Vec<(String, String)>,
    /// Every read of the record an agent's prompt named, and what it printed:
    /// `(the agent's label, the command, its one line)`.
    reads: Vec<(String, String, String)>,
}

impl Ran {
    fn agents(&self) -> Vec<&str> {
        self.trace
            .iter()
            .filter_map(|line| line.trim_start().strip_prefix("agent "))
            .map(|line| line.split(" · ").next().expect("a label"))
            .collect()
    }
}

struct Sim {
    rig: StepRig,
    invocations: Cell<u32>,
}

/// A stand-in for cargo, first on the rig's `PATH`: the build of a held `build`, in the
/// commit the step tool unpacked. The binary it "builds" prints the version the runtime's
/// stand-in names — the one the step asked for, where it asked for one — or it fails.
pub(crate) const CARGO: &str = r#"#!/bin/sh
dir=
while [ $# -gt 0 ]; do
  if [ "$1" = --target-dir ]; then dir=$2; fi
  shift
done
printf '%s cargo %s\n' "$$" "$(pwd -P)" >>"$HOLD_RAN"
echo "   Compiling jigc (a stand-in) in $(pwd -P)"
if [ -n "$SIM_BUILD_FAILS" ]; then echo "error: could not compile" >&2; exit 101; fi
mkdir -p "$dir/release"
printf '#!/bin/sh\necho "jigc %s"\n' "$SIM_BUILD_VERSION" >"$dir/release/jigc"
chmod 755 "$dir/release/jigc"
"#;

/// The composer of the prompt an agent of this label is handed, for the exceptions the
/// prompt fence names by composer.
fn composer_of(label: &str) -> &'static str {
    if label.starts_with("proposal:") {
        "proposalPrompt"
    } else if label.ends_with(":crossmodel") {
        "crossModelPrompt"
    } else if label.starts_with("preflight") {
        "preflightPrompt"
    } else {
        "a prompt"
    }
}

impl Sim {
    /// A repository with a remote, the scanners a write of the record script needs, and
    /// the loop branch of a run nobody has opened yet checked out.
    fn new(label: &str) -> Self {
        let rig = StepRig::unopened(&format!("sim-{label}"));
        fs::create_dir_all(rig.dir().join("tmp")).expect("create the rig's temp directory");
        // THE LONG COMMANDS A STAGE HOLDS, as stand-ins: committed on `main`, where the
        // step tool finds `dev/gate` and `dev/regression-set` beside itself, and cargo
        // first on the `PATH`. What each prints is handed to it by the call that starts it.
        for tool in ["dev/gate", "dev/regression-set"] {
            placed_executable::write(&rig.root.join(tool), LONG_COMMAND);
        }
        rig.commit("chore: the long commands a stage holds, as stand-ins");
        rig.git(&["push", "-q", "origin", "main"]);
        rig.on_path("cargo", CARGO);
        let held = rig.dir().join("held");
        fs::create_dir_all(&held).expect("create the directory of what a held command prints");
        fs::write(held.join("released"), "").expect("release the held commands");
        fs::write(held.join("ran"), "").expect("nothing ran yet");
        rig.git(&["switch", "-q", "-c", LOOP]);
        Sim {
            rig,
            invocations: Cell::new(0),
        }
    }

    /// A rig whose run is opened as the workflow's doc opens one.
    fn opened(label: &str, opening: &Opening) -> Self {
        let sim = Sim::new(label);
        sim.open(opening);
        sim
    }

    /// A child of the rig that may write a record: the rig's environment, the denylist and
    /// a temp directory of the rig's own, run from the repository's root.
    fn command(&self, program: &Path) -> Command {
        let mut command = self.rig.hermetic(Command::new(program));
        command
            .env(
                "TMPDIR",
                format!("{}/", self.rig.dir().join("tmp").display()),
            )
            .env("SIM_HELD", self.rig.dir().join("held"))
            .current_dir(&self.rig.root);
        command
    }

    /// The long commands that ran, as their stand-ins wrote them down: `<program> <arguments>`
    /// each, without the pid — `gate --keep-going`, `regression-set run …`, `cargo <tree>`.
    fn long_commands(&self) -> Vec<String> {
        fs::read_to_string(self.rig.dir().join("held/ran"))
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

    /// Release the held command `name`, which the scenario left running.
    fn release(&self, name: &str) {
        fs::write(self.rig.dir().join(format!("held/release-{name}")), "")
            .expect("release the held command");
    }

    /// One act of the real step tool, asked directly — never through a stage: its line.
    fn step(&self, args: &[&str]) -> Value {
        let out = self
            .command(&self.rig.root.join("dev/stabilize-step"))
            .args(args)
            .stdin(Stdio::null())
            .output()
            .expect("run the step tool");
        serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
            panic!(
                "the step tool prints one JSON line ({e}): {}",
                String::from_utf8_lossy(&out.stderr)
            )
        })
    }

    /// A call of the real record script that must succeed: the one JSON line it prints.
    fn record(&self, args: &[&str], stdin: &str) -> Value {
        let mut child = self
            .command(&self.rig.root.join(RECORD))
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn the record script");
        child_stdin::feed(&mut child, stdin);
        let out = child.wait_with_output().expect("the record script exits");
        assert!(
            out.status.success(),
            "`{RECORD} {}`: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
        serde_json::from_slice(&out.stdout).expect("the record script prints one JSON line")
    }

    /// The run's state, as the record script reads it — asked directly, not through a stage.
    fn state(&self) -> Value {
        self.record(&["state", "--run", RUN], "")
    }

    /// The opening (implementation/stabilization-workflow.md → *Opening a run*): the
    /// opening record, the run's facts — the clauses of its closing condition among them
    /// — the test set, the declared
    /// bounds and the ledger's opening rows — each by the record script's own call — then
    /// ONE commit, pushed. Ready when the state says so.
    fn open(&self, opening: &Opening) {
        self.open_with(opening, &json!({}));
    }

    /// The opening, with `more` facts of the run beside those every opening writes: the
    /// order, the budget and the hand-on number of A RUN WITH A QUEUE (`Sim::queued`).
    fn open_with(&self, opening: &Opening, more: &Value) {
        self.rig.write(
            &format!("{RUN_DIR}/opening.md"),
            "# the opening record\n\nThe closing condition, the candidate and the test set of a simulated run.\n",
        );
        let previous = self.rig.rev("main");
        let mut facts = json!({"stop": "every-round", "previous": PREVIOUS,
                               "previous-commit": previous, "scope": "delta",
                               "clauses": opening.clauses});
        for (name, value) in more.as_object().expect("more facts are an object") {
            facts[name] = value.clone();
        }
        self.record(&["run-set", "--run", RUN], &facts.to_string());
        self.record(&["item-set", "--run", RUN], &opening.items.to_string());
        for bound in &opening.bounds {
            self.record(
                &[
                    "bound-set",
                    "--run",
                    RUN,
                    "--bound",
                    bound,
                    "--reach",
                    "everything the bound names",
                    "--ruling",
                    "the opening record",
                    "--pin",
                    "unpinned",
                ],
                "",
            );
        }
        if opening.rows.as_array().is_some_and(|rows| !rows.is_empty()) {
            self.record(&["ledger-add", "--run", RUN], &opening.rows.to_string());
        }
        self.rig.commit("docs(record): the run is opened");
        self.rig.git(&["push", "-q", "origin", LOOP]);
        let state = self.state();
        assert_eq!(
            state["not_ready"],
            json!([]),
            "the opening owes nothing: {state}"
        );
        assert_eq!(state["next"], "test", "an opened run's first step: {state}");
    }

    /// The arguments of an invocation of `stage` on the rig's run, with `more`.
    fn args(&self, stage: &str, more: Value) -> Value {
        let mut args = json!({"stage": stage, "run": RUN,
                              "scratch": self.rig.scratch.display().to_string()});
        for (name, value) in more.as_object().expect("more arguments are an object") {
            args[name] = value.clone();
        }
        args
    }

    /// One invocation of the committed harness under the runtime's stand-in: `script` is
    /// what the scripted agents say — `{checks, scope, agents, record}`.
    fn invoke(&self, args: Value, script: Value) -> Ran {
        let n = self.invocations.get() + 1;
        self.invocations.set(n);
        let mut scenario = script;
        scenario["args"] = args;
        let file = self.rig.dir().join(format!("scenario-{n}.json"));
        fs::write(&file, scenario.to_string()).expect("write the scenario");
        let out = self
            .command(Path::new("node"))
            .arg(repo_root().join(RUNTIME))
            .arg(repo_root().join(HARNESS))
            .arg(&file)
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
            "invocation {n} is not one the simulation stands behind: {}\nthe trace so far:\n{}",
            String::from_utf8_lossy(&out.stderr),
            trace.join("\n")
        );
        assert_eq!(
            list("swallowed"),
            Vec::<String>::new(),
            "invocation {n}: something thrown inside a `parallel` became a null"
        );
        assert_eq!(
            list("unused"),
            Vec::<String>::new(),
            "invocation {n}: an agent the scenario scripts was never launched"
        );
        // EVERY PROMPT AN AGENT WAS HANDED IS HELD by the fence that holds every prompt the
        // script can compose (the second repair plan's `K11`): each command it names is one
        // plain invocation of a tool, and it tells no agent to wait, to redirect or to pipe
        // by its own means.
        for prompt in said["prompts"].as_array().expect("the prompts") {
            let (label, handed) = (text(&prompt["label"]), text(&prompt["prompt"]));
            let (faults, _) = prompt_faults(
                composer_of(&label),
                &handed,
                &mut std::collections::BTreeSet::new(),
            );
            assert!(
                faults.is_empty(),
                "invocation {n}: the prompt of `{label}` names what is no plain invocation of a tool:\n{}",
                faults.join("\n")
            );
        }
        Ran {
            result: said["result"].clone(),
            trace,
            logs: list("logs"),
            reporters: list("reporters"),
            commands: list("ran"),
            prompts: said["prompts"]
                .as_array()
                .expect("the stand-in reports `prompts`")
                .iter()
                .map(|p| (text(&p["label"]), text(&p["prompt"])))
                .collect(),
            reads: said["reads"]
                .as_array()
                .expect("the stand-in reports `reads`")
                .iter()
                .map(|r| (text(&r["label"]), text(&r["command"]), text(&r["printed"])))
                .collect(),
        }
    }

    /// The reports of a round's `test` stage on disk, by file name.
    fn reports(&self, round: u32) -> Vec<String> {
        let dir = self
            .rig
            .root
            .join(format!("{RUN_DIR}/r{round}/reports/test"));
        let mut names: Vec<String> = fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
            .map(|entry| {
                entry
                    .expect("a report")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
    }
}

fn text(value: &Value) -> String {
    value.as_str().expect("a text").to_owned()
}

/// A file a return names, read as whoever is handed the return reads it — by its path, and
/// held to the sha256 the return names it by: the one line of JSON it holds. **The harness
/// passes on no document and no prose of the record; it names the file** (the second repair
/// plan's `K3`).
fn named_file(named: &Value) -> Value {
    let path = named["file"]
        .as_str()
        .unwrap_or_else(|| panic!("the return names a file by its path: {named}"));
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    assert_eq!(
        named["sha256"],
        sha256(&text).as_str(),
        "the file at {path} is the one the return names by its hash"
    );
    serde_json::from_str(text.trim_end())
        .unwrap_or_else(|e| panic!("{path} holds one line of JSON ({e})"))
}

/// The state document a return names (`named.document`): what the harness read a digest of.
fn document(result: &Value) -> Value {
    named_file(&result["named"]["document"])
}

/// The line a refused step printed unasked — its halt report in it — which the halt names
/// (`halted.step`), and the harness never read.
fn refused_line(result: &Value) -> Value {
    named_file(&result["halted"]["step"])
}

fn lines(list: &[&str]) -> Vec<String> {
    list.iter().map(|line| (*line).to_owned()).collect()
}

fn sorted(mut list: Vec<String>) -> Vec<String> {
    list.sort();
    list
}

/// The two reads every stage opens with.
const TWO_READS: [&str; 3] = [
    "phase State",
    "agent git:state · build-git · sonnet · State",
    "agent git:state:test-1 · build-git · sonnet · State",
];

/// Hold `text` to the committed golden at `rel` — or write it, where a re-capture is asked
/// for by name and the environment is not CI's.
fn golden(rel: &str, text: &str) {
    let path = repo_root().join(rel);
    let update = update_mode(
        std::env::var("UPDATE_GOLDENS").ok().as_deref(),
        std::env::var("CI").ok().as_deref(),
    );
    if update {
        fs::write(&path, text).unwrap_or_else(|e| panic!("write `{rel}`: {e}"));
        return;
    }
    let committed = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read `{rel}`: {e}"));
    assert_eq!(
        text, committed,
        "what the stage launched, and in which order, is not what `{rel}` holds. A diff of \
         that file is a change to what a stage DOES: adjudicate it against the ruling that \
         asked for it, then re-capture with `UPDATE_GOLDENS=1 cargo test -p jigc \
         stabilize_simulation::` and commit the diff — never re-pin it to make a run green"
    );
}

// ---------------------------------------------------------------------------
// A whole `test` stage
// ---------------------------------------------------------------------------

/// One item of every kind the harness has a chain for, a deterministic check, and one
/// item the round's doors do not reach.
fn every_kind() -> Opening {
    Opening {
        clauses: vec!["gate-green", "no-lost-files", "adoptable", "audit-clean"],
        items: json!([
            item("gate", "check", "gate-green"),
            in_scope(
                "row-a",
                "review-row",
                "no-lost-files",
                &["jigc doc set"],
                &[]
            ),
            in_scope(
                "row-b",
                "review-row",
                "no-lost-files",
                &["jigc rename"],
                &[]
            ),
            in_scope("arm-a", "trial-arm", "adoptable", &[], &["verbs"]),
            in_scope(
                "area-a",
                "audit-area",
                "audit-clean",
                &["jigc doc set"],
                &[]
            ),
            item("cross", "audit-cross-cutting", "audit-clean"),
            in_scope("drive-a", "audit-drive", "audit-clean", &[], &["verbs"]),
        ]),
        bounds: vec!["windows"],
        rows: json!([{
            "key": "setup-on-windows", "doctype": "jigc-feedback", "round": 0,
            "source": "known at the opening", "door": "jigc setup", "clause": "adoptable",
            "repro": "the opening record",
        }]),
    }
}

/// The keys of the four findings the stage grades.
const INSIDE: &str = "doc-set-drops-a-slot";
const OUTSIDE: &str = "rename-loses-a-referrer";
const REFUTED: &str = "doc-set-exit-code";
const SEEDED: &str = "setup-on-windows";

/// What the scripted agents of that stage say: a finding inside the round's test set (and
/// contested, so that it is a fork), one outside it, one the verifier refutes, a row seeded
/// at the opening that triage grades out of scope under a declared bound — and nothing
/// from every other reporter.
fn every_kind_script() -> Value {
    let verified = |key: &str, verdict: &str, basis: &str| {
        let mut v = json!({"status": "verified", "key": key, "verdict": verdict,
                           "basis": basis, "repro": format!("the block of {key}"),
                           "pinnable": true, "left_open": []});
        if verdict == "confirmed" {
            v["regression"] = json!(false);
        }
        v
    };
    let mut contested = verified(INSIDE, "confirmed", "reproduces as written");
    contested["contested"] = json!(true);
    let mut seeded = entry(SEEDED, "ledger", "jigc setup", "adoptable", "out-of-scope");
    seeded["new"] = json!(false);
    seeded["bound"] = json!("windows");
    json!({
        "checks": {"gate": "green"},
        "scope": {
            "included": [door("jigc doc set", "verbs"), door("jigc task finalize", "verbs")],
            "excluded": [door("jigc rename", "verbs"), door("jigc setup", "verbs")],
            "uncovered": [], "reached_but_excluded": ["jigc rename"], "left_open": [],
        },
        "agents": {
            "row-a:source": found(&[finding("s-1", "a slot is dropped", "jigc doc set", "no-lost-files")]),
            "row-a:driver": nothing(),
            "row-a:reconciler": nothing(),
            "arm-a:rehearse": nothing(),
            "arm-a:run": nothing(),
            "arm-a:score": found(&[finding("t-1", "a referrer is lost", "jigc rename", "no-lost-files")]),
            "area-a:review": nothing(),
            "cross:review": nothing(),
            "drive-a:drive": found(&[finding("d-1", "a wrong exit code", "jigc doc set", "no-lost-files")]),
            "triage:p1": graded(
                &[
                    entry(INSIDE, "row-a-source s-1", "jigc doc set", "no-lost-files", "breaks"),
                    entry(OUTSIDE, "arm-a-score t-1", "jigc rename", "no-lost-files", "breaks"),
                    entry(REFUTED, "drive-a-drive d-1", "jigc doc set", "no-lost-files", "unclear"),
                    seeded,
                ],
                &["row-a-source", "arm-a-score", "drive-a-drive", "ledger"],
            ),
            format!("verify:{INSIDE}"): contested,
            format!("verify:{OUTSIDE}"): verified(OUTSIDE, "confirmed", "reproduces as written"),
            format!("verify:{REFUTED}"): verified(REFUTED, "refuted", "does-not-reproduce: exit 0 on a fresh rig"),
            format!("advocate:{INSIDE}"): {
                "status": "argued", "verdict": "robust-now", "case": "The robust case.",
                "proposal": "A change somebody else can apply.",
                "driven": [{"step": "the next step", "command": "jigc doc set", "result": "exit 0"}],
                "undriven": [], "left_open": [],
            },
            format!("proposal:{INSIDE}"): {
                "status": "driven", "holds": true,
                "steps": [{"step": "the next step", "command": "jigc doc set", "result": "exit 0", "agrees": true}],
                "undriven": [], "left_open": [],
            },
        },
    })
}

#[test]
fn a_test_stage_runs_from_the_git_state_to_the_pushed_record() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("whole", &every_kind());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(sim.args("test", json!({})), every_kind_script());
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");

    // (1) WHAT WAS LAUNCHED, AND IN WHICH ORDER: the golden.
    // The previous release's build is named by its commit — the rig's — and the candidate's
    // build and its gate by the candidate's: the golden holds each as a placeholder. What the
    // file pins is the steps, never a sha of a test rig.
    golden(
        TRACE,
        &format!("{}\n", ran.trace.join("\n"))
            .replace(&previous_build(&sim), "build-previous-<its commit>")
            .replace(&sim.opened_at()[..12], "<the candidate>"),
    );

    // (2) EXACTLY ONE REPORT PER LAUNCHED REPORTER, on disk and committed. The launched
    // are those whose prompt handed them a report to write; the stand-in wrote each
    // through the record script, under the name the prompt gave.
    let launched = sorted(ran.reporters.clone());
    assert_eq!(
        launched.len(),
        19,
        "the reporters this stage launched, and the marker its attempt began with: {launched:?}"
    );
    assert!(launched.contains(&"attempt".to_owned()), "{launched:?}");
    assert_eq!(
        sim.reports(1),
        sorted(
            launched
                .iter()
                .map(|name| format!("{name}.a1.md"))
                .collect()
        ),
        "one report per launched reporter, and no other"
    );
    assert_eq!(result["counts"]["reporters"], 19, "{result}");
    assert_eq!(
        sim.rig.status(),
        "",
        "every report is committed, and the tree is clean"
    );
    let committed = sim.rig.git(&["show", "--name-only", "--format=%s", "HEAD"]);
    let mut committed = committed.lines();
    assert_eq!(
        committed.next(),
        Some("docs(record): rc24 r1 - the record of the test stage"),
        "the record is ONE commit, with the subject the step names"
    );
    let paths: Vec<&str> = committed.filter(|line| !line.is_empty()).collect();
    assert!(
        paths
            .iter()
            .all(|path| path.starts_with(&format!("{RUN_DIR}/"))),
        "of the run directory's paths and nothing else: {paths:?}"
    );
    for name in &launched {
        let report = format!("{RUN_DIR}/r1/reports/test/{name}.a1.md");
        assert!(
            paths.contains(&report.as_str()),
            "`{report}` is in the record commit"
        );
    }

    // (3) THE LEDGER AND THE TRIAGE RECORD, as the record script reads them back.
    let state = sim.state();
    let row = |key: &str| -> &Value {
        state["ledger"]
            .as_array()
            .expect("the ledger")
            .iter()
            .find(|row| row["key"] == key)
            .unwrap_or_else(|| panic!("the ledger holds `{key}`: {state}"))
    };
    let read = |key: &str| -> Value {
        let row = row(key);
        json!([
            row["round"],
            row["grade"],
            row["graded_by"],
            row["disposition"],
            row["triage"]["door"],
            row["triage"]["verdict"],
            row["route"],
            row["why"]
        ])
    };
    assert_eq!(state["ledger"].as_array().map(Vec::len), Some(4), "{state}");
    assert_eq!(
        read(INSIDE),
        json!([
            1,
            "confirmed",
            "verify-real",
            "open",
            "included",
            "confirmed",
            "human",
            "fork"
        ]),
        "M3: a contested finding is the human's, though it is inside the round's test set"
    );
    assert_eq!(
        read(OUTSIDE),
        json!([
            1,
            "confirmed",
            "verify-real",
            "open",
            "excluded",
            "confirmed",
            "human",
            "outside"
        ])
    );
    assert_eq!(
        read(REFUTED),
        json!([
            1,
            "refuted",
            "verify-real",
            "open",
            "included",
            "refuted",
            "recorded",
            "not-a-break"
        ])
    );
    assert_eq!(
        read(SEEDED),
        json!([
            0,
            "out-of-scope",
            "triage",
            "open",
            "excluded",
            null,
            "recorded",
            "out-of-scope"
        ])
    );
    let round = &state["rounds"][0];
    assert_eq!(round["facts"]["candidate"], candidate.as_str(), "{round}");
    assert_eq!(
        round["facts"]["binary"], result["candidate"]["sha256"],
        "{round}"
    );
    assert_eq!(
        round["facts"]["base"],
        sim.rig.rev("main").as_str(),
        "{round}"
    );
    assert_eq!(
        json!([
            round["scope"],
            round["triage"],
            round["record"],
            round["test_reports"]
        ]),
        json!([true, true, true, 19])
    );
    for clause in state["clauses"].as_array().expect("the clause table") {
        assert_eq!(
            json!([clause["status"], clause["commit"], clause["round"]]),
            json!(["green", candidate, 1]),
            "every clause an item of this round judged ran to its end on the candidate: {clause}"
        );
    }

    // (4) THE REMOTE BRANCH STANDS AT THE RECORDED HEAD.
    let head = sim.rig.rev("HEAD");
    assert_eq!(result["record"], head.as_str(), "{result}");
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(head.clone()),
        "the loop branch is pushed"
    );
    assert_eq!(
        sim.rig.rev("HEAD~1"),
        candidate,
        "the record is the one commit on the candidate"
    );
    // THE CANDIDATE'S GATE IS ON RECORD, in that commit: what the preflight's run showed —
    // green here — as a table of the round, which every record commit of it is held to.
    assert!(
        paths.contains(&format!("{RUN_DIR}/r1/gate.md").as_str()),
        "the candidate's gate is a file of the record commit: {paths:?}"
    );
    let gate = sim.rig.read(&format!("{RUN_DIR}/r1/gate.md"));
    assert!(
        gate.contains(&format!("| commit | `{candidate}` |"))
            && gate.contains("| verdict | pass |"),
        "it names the candidate and what its gate said: {gate}"
    );
    assert_eq!(
        sim.record(&["pending", "--run", RUN], "")["batch"],
        Value::Null,
        "and the batch the record step applied is settled"
    );

    // EVERY STEP ASKED FOR ITS DIGEST, under the invocation's scratch root (K3) — and the
    // record's push was a record's: held to publishing the run's records and nothing else.
    let scratch = sim.rig.scratch.display().to_string();
    for step in ran
        .commands
        .iter()
        .filter(|ran| ran.starts_with("dev/stabilize-step "))
    {
        // The one command of the step tool that is no step: the hash the independent drive
        // of a proposal asks of the tool, and reads itself.
        assert!(
            step.contains(&format!(" --digest {scratch}"))
                || step
                    == &format!(
                        "dev/stabilize-step hash --scratch {scratch} --file {}",
                        candidate_bin(&candidate)
                    ),
            "a step of the stage asked for the line the tool prints unasked: {step}"
        );
    }
    assert!(
        ran.commands.contains(&format!(
            "dev/stabilize-step push --branch {LOOP} --run-dir {RUN_DIR} --digest {scratch}"
        )),
        "{:?}",
        ran.commands
    );

    // (5) THE RETURN THE ORCHESTRATOR READS — `next` as the record script computes it.
    assert_eq!(state["next"], "rule", "{state}");
    assert_eq!(
        result["next"], state["next"],
        "`next` is relayed, never computed: {result}"
    );
    // THE HUMAN'S LIST IS A COUNT IN THE RETURN, AND A LIST IN THE FILE THE RETURN NAMES
    // (K3): no row of the record passes through the harness.
    assert_eq!(result["rule"]["findings"], 2, "{result}");
    assert_eq!(
        document(result)["human_list"],
        json!([{"key": INSIDE, "why": "fork"}, {"key": OUTSIDE, "why": "outside"}])
    );
    assert_eq!(
        document(result),
        state,
        "the file is the state, as the record script reads it"
    );
    assert!(
        result.get("human_list").is_none() && result.get("blockers").is_none(),
        "{result}"
    );
    assert_eq!(result["unverified"], json!([]));
    assert_eq!(result["round"], 1);
    assert_eq!(
        json!([
            result["candidate"]["label"],
            result["candidate"]["sha"],
            result["candidate"]["binary"]
        ]),
        json!([
            "c1",
            candidate,
            format!(
                "{}/{}",
                sim.rig.scratch.display(),
                candidate_bin(&candidate)
            )
        ])
    );
    assert_eq!(
        result["counts"],
        json!({"items": 7, "items_run": 6, "reporters": 19, "findings_in": 4, "entries": 4,
               "by_grade": {"confirmed": 2, "refuted": 1, "out-of-scope": 1},
               "blockers": 0, "for_the_human": 2, "voided": []})
    );
    assert_eq!(
        result["scope"],
        json!({"status": "written", "uncovered": [], "reached_but_excluded": ["jigc rename"]})
    );
    assert!(
        ran.logs
            .iter()
            .any(|line| line.contains("5 of 6 hunting item(s) run this round, and 1 check(s)")),
        "the round ran what its doors select: {:?}",
        ran.logs
    );

    // REPAIRED — M3 (harness-agents-gate.md): A FORK IS ON RECORD. The contested finding
    // comes back in `forks`, with the advocate's case and the independent drive — and the
    // round's triage record holds it with the verdict it rides on, so the state a later
    // invocation starts from says that there is one, of which kind, what the advocate's
    // verdict is and whether the independent drive holds. It is on the human's list, and
    // no fixer's: the next step is `rule`.
    let forks = result["forks"].as_array().expect("the forks");
    assert_eq!(forks.len(), 1, "{result}");
    assert_eq!(
        json!([
            forks[0]["key"],
            forks[0]["kind"],
            forks[0]["advocate"]["verdict"],
            forks[0]["independent_drive"]["holds"],
            forks[0]["driven_by"]
        ]),
        json!([INSIDE, "contested", "robust-now", true, "general-purpose"])
    );
    assert_eq!(
        state["blockers"],
        json!([]),
        "M3: an unruled fork is no fixer's"
    );
    assert_eq!(
        row(INSIDE)["triage"]["fork"],
        json!({"kind": "contested", "case": "robust-now", "drive": "holds"}),
        "M3: the fork is a row the next invocation reads"
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/triage.md"))
            .contains("| contested: robust-now, holds |"),
        "M3: in the round's committed triage record"
    );
    for name in [
        format!("advocate-p1-{INSIDE}"),
        format!("proposal-p1-{INSIDE}"),
    ] {
        assert!(
            launched.contains(&name),
            "the case and the independent drive are reports of the stage: {launched:?}"
        );
    }

    // WHAT THE RECORD SCRIPT SAYS NEXT, and what a second invocation does with it.
    assert_eq!(
        state["position"]["test"],
        json!({"refused": "round-open", "round": 1})
    );
    assert_eq!(
        state["position"]["fix"],
        json!({"round": 1, "cycle": 1, "attempt": 1, "land": false})
    );
    assert_eq!(
        state["candidate"],
        json!({"round": 1, "commit": candidate, "current": true})
    );
    assert_eq!(
        state["rounds"][0]["facts"]["recorded"],
        json!(["test a1"]),
        "the attempt that reached its record is named in the round's record"
    );
    let again = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(again.result["status"], "refused", "{}", again.result);
    assert_eq!(again.result["refused"], state["position"]["test"]);
    assert_eq!(again.result["next"], "rule");
    assert_eq!(
        again.trace,
        lines(&TWO_READS),
        "nothing ran beyond the two reads"
    );
    assert_eq!(sim.rig.rev("HEAD"), head, "and nothing was committed");

    // THE HUMAN'S RULINGS ON THE LIST, WHILE THE `fix` STAGE REFUSES TO START: an
    // invocation of `test` that carries only rulings records them on the loop branch, by
    // the one step that may write them, and starts nothing.
    let rulings = json!([
        {"key": OUTSIDE, "ruling": "later", "note": "for the release after this one"},
        {"key": REFUTED, "ruling": "bound", "bound": "exit-codes", "reach": "the exit code of a refused write",
         "where": "the stop after round 1, item 2", "pin": "unpinned"},
        {"bound": "second-writer", "reach": "a writer that is not jigc", "where": "the stop after round 1", "pin": "unpinned"},
    ]);
    let stranger = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"key": "no-such-finding", "ruling": "later"}]}),
        ),
        json!({}),
    );
    // WHETHER A RULING NAMES A ROW IS ITS OWN BATCH'S TO HOLD (K3): the ledger is no part
    // of what the harness reads, so the batch is composed, the record script refuses it
    // whole — the key is no row — and nothing is written, committed or left pending.
    assert_eq!(stranger.result["status"], "halted", "{}", stranger.result);
    assert_eq!(
        stranger.agents(),
        ["git:state", "git:state:test-1", "record:rulings:r1"],
        "the batch is refused where it is applied: no commit step, no push"
    );
    assert_eq!(
        json!([
            stranger.result["halted"]["phase"],
            sim.record(&["pending", "--run", RUN], "")["batch"]
        ]),
        json!(["rulings", null]),
        "a ruling on no row records nothing, and leaves no batch: {}",
        stranger.result
    );
    assert_eq!(sim.rig.rev("HEAD"), head);
    let ruled = sim.invoke(sim.args("test", json!({"rulings": rulings})), json!({}));
    assert_eq!(ruled.result["status"], "ruled", "{}", ruled.result);
    assert_eq!(
        ruled.agents(),
        [
            "git:state",
            "git:state:test-1",
            "record:rulings:r1",
            "git:hold-start:record-rulings-r1-1",
            "git:hold-wait:record-rulings-r1-1",
            "git:record:rulings:r1",
            "git:push:rulings",
            "git:state:test-2"
        ],
        "two reads, the one step that records rulings, its gate — held — its commit, the push, the state: nothing is started"
    );
    // M3: THE FORK IS STILL UNRULED, so the next step is still the human's — never `fix`,
    // though nothing else is left to rule on.
    assert_eq!(
        json!([ruled.result["next"], ruled.result["rule"]["findings"]]),
        json!(["rule", 1]),
        "M3: {}",
        ruled.result
    );
    assert_eq!(
        document(&ruled.result)["human_list"],
        json!([{"key": INSIDE, "why": "fork"}]),
        "M3"
    );
    // THE HUMAN'S RULING ON THE FORK is a ruling like the others, written by the same one
    // step: admitted, with the note that reaches the fixer.
    let fork_ruled = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"key": INSIDE, "ruling": "admitted", "note": "take the robust path"}]}),
        ),
        json!({}),
    );
    assert_eq!(
        fork_ruled.result["status"], "ruled",
        "{}",
        fork_ruled.result
    );
    assert_eq!(
        fork_ruled
            .agents()
            .into_iter()
            .filter(|label| label.contains("rulings"))
            .collect::<Vec<_>>(),
        [
            "record:rulings:r1",
            "git:hold-start:record-rulings-r1-1",
            "git:hold-start:record-rulings-r1-2",
            "git:hold-wait:record-rulings-r1-2",
            "git:record:rulings:r1",
            "git:push:rulings"
        ],
        "M3: by the one step that records rulings — whose gate is held under the NEXT name: the first is the gate of the rulings recorded before these, on the tree as it stood then"
    );
    let ruled = fork_ruled;
    let after = sim.state();
    let disposition = |key: &str| -> Value {
        let row = after["ledger"]
            .as_array()
            .expect("the ledger")
            .iter()
            .find(|row| row["key"] == key)
            .expect("the row");
        json!([row["disposition"], row["detail"], row["route"]])
    };
    assert_eq!(
        disposition(INSIDE),
        json!(["admitted", "take the robust path", "fix"]),
        "M3: the ruled fork is a fixer's, with the human's note"
    );
    assert_eq!(after["blockers"], json!([INSIDE]));
    assert_eq!(
        disposition(OUTSIDE),
        json!(["later", "for the release after this one", "recorded"])
    );
    assert_eq!(
        disposition(REFUTED),
        json!(["bound", "the exit code of a refused write", "recorded"])
    );
    let bounds: Vec<&str> = after["bounds"]
        .as_array()
        .expect("the bounds")
        .iter()
        .map(|bound| bound["bound"].as_str().expect("a bound"))
        .collect();
    assert_eq!(bounds, ["windows", "exit-codes", "second-writer"]);
    assert_eq!(
        ruled.result["next"], "fix",
        "nothing is left to rule on, and the blocker is a fixer's: {}",
        ruled.result
    );
    let recorded = sim.rig.rev("HEAD");
    assert_eq!(
        sim.rig.rev("HEAD~2"),
        head,
        "one commit per invocation that ruled"
    );
    assert_eq!(ruled.result["record"], recorded.as_str());
    assert_eq!(sim.rig.remote(LOOP), Some(recorded.clone()), "pushed");
    assert_eq!(sim.rig.status(), "");
    // … and the stage those rulings wait for still refuses, whatever it carries.
    let fix = sim.invoke(sim.args("fix", json!({"rulings": rulings})), json!({}));
    assert_eq!(fix.result["status"], "refused", "{}", fix.result);
    assert_eq!(fix.result["not_fit"]["stage"], "fix");
    assert_eq!(fix.trace, Vec::<String>::new());
    assert_eq!(sim.rig.rev("HEAD"), recorded);
}

// ---------------------------------------------------------------------------
// Where a stage meets a defect the reviews recorded
// ---------------------------------------------------------------------------

/// The return of a verifier that could not drive its finding: a halt, with its report.
fn undriven(key: &str) -> Value {
    json!({
        "status": "halted", "key": key,
        "halt": {"root_cause": "the repro can be neither driven nor reconstructed",
                 "evidence": "scripted", "tree_state": "clean", "recommendation": "none"},
    })
}

/// A triage entry for a row the ledger already holds: found again, under its own key.
fn again(key: &str, door: &str, clause: &str, grade: &str) -> Value {
    let mut entry = entry(key, "ledger", door, clause, grade);
    entry["new"] = json!(false);
    entry
}

/// The agents of a re-run inside round 1, in order: the candidate's binary built again —
/// from its commit, by a held command of that attempt's name — the previous release's
/// answered by the build that is there, NO GATE OF THE CANDIDATE'S, the `instruments` that
/// are due, and the record with its own gate, held.
fn rerun_agents(sim: &Sim, attempt: u32, instruments: &[&str]) -> Vec<String> {
    let mut agents = lines(&["git:state", "git:state:test-1", "git:begin", "preflight"]);
    agents.push(format!(
        "git:hold-start:{}",
        candidate_build(&sim.opened_at())
    ));
    agents.push(format!("git:hold-start:{}", previous_build(sim)));
    agents.push("git:state:test-2".to_owned());
    agents.extend(lines(instruments));
    agents.extend(lines(&["git:check-reports", "record:test:r1"]));
    agents.extend(held_steps(&format!("record-test-r1-a{attempt}-1")));
    agents.extend(lines(&[
        "git:record:test:r1",
        "git:push",
        "git:state:test-3",
    ]));
    agents
}

/// The agents of an invocation that only finishes a round's triage, in order — `attempt`
/// is the stage's: the candidate's binary is built again, by a held command of that
/// attempt's name, and the previous release's, built already under this scratch root, is
/// the one its start answers with; then the record's own gate, held.
fn finishing_lap(sim: &Sim, attempt: u32, verified: &[&str], last_state: &str) -> Vec<String> {
    let mut agents = lines(&["git:state", "git:state:test-1", "git:begin", "preflight"]);
    agents.push(format!(
        "git:hold-start:{}",
        candidate_build(&sim.opened_at())
    ));
    agents.push(format!("git:hold-start:{}", previous_build(sim)));
    agents.push("triage:p1".to_owned());
    agents.extend(verified.iter().map(|key| format!("verify:{key}")));
    agents.extend(lines(&["git:check-reports:p1", "record:triage:r1"]));
    agents.extend(held_steps(&format!("record-triage-r1-a{attempt}-1")));
    agents.extend(lines(&[
        "git:record:triage:r1",
        "git:push:triage",
        last_state,
    ]));
    agents
}

#[test]
fn an_unverified_finding_is_finished_by_the_next_invocation_and_after_one_more_it_is_the_humans() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("unverified", &one_area());
    let key = "doc-set-drops-a-slot";
    let script = json!({
        "scope": one_door(),
        "agents": {
            "area-a:review": found(&[finding("a-1", "a slot is dropped", "jigc doc set", "audit-clean")]),
            "triage:p1": graded(
                &[entry(key, "area-a-review a-1", "jigc doc set", "audit-clean", "breaks")],
                &["area-a-review"],
            ),
            // The verifier could not drive it: a halt, with its report, and no verdict.
            format!("verify:{key}"): undriven(key),
        },
    });
    let ran = sim.invoke(sim.args("test", json!({})), script);
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(result["counts"]["by_grade"], json!({"breaks": 1}));
    assert_eq!(
        result["unverified"],
        json!([{"key": key, "why": "its verifier halted (the repro can be neither driven nor reconstructed)"}]),
        "the stage says which finding it left without a verdict, and why"
    );

    // A VALUE OF `next` THE SCRIPT HAS NO SENTENCE FOR goes back with the state attached:
    // never a guess, never a loop.
    assert_eq!(result["next"], "triage", "{result}");
    assert_eq!(result["returned_to_orchestrator"], true);
    // … AS WHAT THE DIGEST HOLDS OF IT, AND THE FILE THE DOCUMENT LIES IN (K3; the plan's X2):
    // the rows are a count in the return, and a list in the file it names.
    assert_eq!(
        result["state"]["untriaged"],
        json!({"count": 1, "why": [{"why": "unverified", "count": 1}]})
    );
    assert_eq!(
        document(result)["untriaged"],
        json!([{"key": key, "why": "unverified"}])
    );
    assert_eq!(
        result["state"]["position"]["test"],
        json!({"round": 1, "attempt": 2, "triage": true}),
        "the stage that left the triage unfinished finishes it"
    );
    let candidate = sim.rig.rev("HEAD~1");
    let recorded = sim.rig.rev("HEAD");

    // REPAIRED — M6 (harness-agents-gate.md): THE FINISHING LAP REACHES ITS RECORD. It asks
    // the preflight for the commit the round tested, which the record commit has moved
    // `HEAD` off — and the preflight's contract takes that: a candidate that is a commit
    // `HEAD` holds is built from the commit, and nothing of that call reads the tree. The
    // lap grades the ledger's row again, sends it to a verifier once more, and records.
    // No instrument runs, and no scope step.
    let lap = json!({"agents": {
        "triage:p1": graded(&[again(key, "jigc doc set", "audit-clean", "breaks")], &["ledger"]),
        format!("verify:{key}"): undriven(key),
    }});
    let finishing = sim.invoke(sim.args("test", json!({})), lap.clone());
    let finished = &finishing.result;
    assert_eq!(finished["status"], "triaged", "M6: {finished}");
    assert_eq!(finished["triage_only"], true, "M6: {finished}");
    assert_eq!(
        finishing.agents(),
        finishing_lap(&sim, 2, &[key], "git:state:test-2"),
        "M6: the attempt begun, the preflight, triage, the verifier, the reports checked, the record, its commit, the push, the state"
    );
    assert_eq!(
        json!([finished["candidate"]["sha"], finished["candidate"]["label"]]),
        json!([candidate, "c1"]),
        "M6: the verifier drove the commit the round tested, not the record commit ({recorded})"
    );
    let (subject, paths) = head_commit(&sim);
    assert_eq!(
        subject,
        format!("docs(record): {RUN} r1 - the triage of the round, finished")
    );
    assert_eq!(sim.rig.rev("HEAD~1"), recorded, "one record commit");
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");
    for name in [
        "attempt",
        "preflight",
        "triage-p1",
        &format!("verify-p1-{key}"),
    ] {
        assert!(
            paths.contains(&format!("{RUN_DIR}/r1/reports/test/{name}.a2.md")),
            "the lap's report `{name}` is in its record commit: {paths:?}"
        );
    }
    assert_eq!(sim.rig.status(), "");

    // REPAIRED — F3 (state-machine.md), now THROUGH THE HARNESS: the lap's own record
    // counted the pass. The finding, left unverified a second time, is the human's — with
    // the reason — and no call of this suite into the record script wrote that.
    let state = sim.state();
    assert_eq!(finished["next"], "rule", "F3: {finished}");
    assert_eq!(
        json!([
            state["next"],
            state["human_list"],
            finished["rule"]["findings"],
            finished["rule"]["reverify"]
        ]),
        json!(["rule", [{"key": key, "why": "unverified-after-retry"}], 1, [key]]),
        "F3: how many the human's list holds, and the key that may be granted one more triage: {state}"
    );
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["recorded"],
            state["rounds"][0]["test_unrecorded"],
            state["rounds"][0]["facts"]["awaiting"]
        ]),
        json!([["test a2"], 0, [format!("{key} unverified 2")]]),
        "F3: the lap reached its record, and the round's triage has left the finding without a verdict twice: {state}"
    );
    let refused = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(refused.result["status"], "refused", "{}", refused.result);
    assert_eq!(
        refused.result["refused"],
        json!({"refused": "round-open", "round": 1}),
        "F3: no invocation finishes that triage again unasked"
    );
    assert_eq!(refused.trace, lines(&TWO_READS));
    // What the human may do is rule it — or grant one more triage, a fact the one step
    // that records rulings writes, and that the record script takes only here.
    let early = sim.invoke(
        sim.args("test", json!({"rulings": [{"again": "test"}]})),
        json!({}),
    );
    assert_eq!(early.result["status"], "refused", "{}", early.result);
    assert_eq!(
        early.trace,
        lines(&TWO_READS),
        "one more attempt of a stage nobody is asked about records nothing"
    );
    let granted = sim.invoke(
        sim.args("test", json!({"rulings": [{"reverify": key}]})),
        json!({}),
    );
    assert_eq!(granted.result["status"], "ruled", "{}", granted.result);
    assert_eq!(granted.result["next"], "triage", "{}", granted.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["reverify"],
            state["position"]["test"]
        ]),
        json!([[format!("{key} p3")], {"round": 1, "attempt": 3, "triage": true}]),
        "F3: one grant, one more triage: {state}"
    );

    // A STAGE THAT HALTS BEFORE ITS RECORD, TWICE IN A ROW, IS THE HUMAN'S TOO. The granted
    // lap's triage halts — a report of the round is cut off, say — and so does the lap
    // after it: each left its marker, its preflight's report and triage's, and no record.
    let stuck = json!({"agents": {"triage:p1": {
        "status": "halted", "entries": [], "counts": {"findings_in": [], "entries": 0, "merged": 0},
        "halt": {"root_cause": "a report the prompt lists is cut off", "evidence": "scripted",
                 "tree_state": "clean", "recommendation": "none"},
    }}});
    let halted = sim.invoke(sim.args("test", json!({})), stuck.clone());
    assert_eq!(halted.result["status"], "halted", "{}", halted.result);
    assert_eq!(halted.result["halted"]["phase"], "triage");
    let state = sim.state();
    assert_eq!(
        json!([
            state["next"],
            state["position"]["test"],
            state["rounds"][0]["test_unrecorded"]
        ]),
        json!(["triage", {"round": 1, "attempt": 4, "triage": true}, 1]),
        "one attempt without a record is tried once more without the human: {state}"
    );
    let halted_again = sim.invoke(sim.args("test", json!({})), stuck);
    assert_eq!(halted_again.result["halted"]["phase"], "triage");
    let state = sim.state();
    assert_eq!(state["next"], "rule", "F3: {state}");
    assert_eq!(
        state["human_stages"],
        json!([{"stage": "test", "round": 1, "cycle": null, "at": "test", "attempt": 5,
                "attempts": 2, "why": "not-recorded-after-retry"}]),
        "F3: the stage, how often it left reports and no record, and why it is the human's"
    );
    let spent = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(spent.result["status"], "refused", "{}", spent.result);
    assert_eq!(
        spent.result["refused"],
        json!({"refused": "attempts-spent", "round": 1})
    );
    assert!(
        spent.result["message"]
            .as_str()
            .is_some_and(|said| said.contains("args.rulings, `again`")),
        "the refusal names the ruling that lifts it: {}",
        spent.result
    );
    assert_eq!(spent.trace, lines(&TWO_READS), "and it launches nothing");
    let once_more = sim.invoke(
        sim.args("test", json!({"rulings": [{"again": "test"}]})),
        json!({}),
    );
    assert_eq!(once_more.result["status"], "ruled", "{}", once_more.result);
    assert_eq!(once_more.result["next"], "triage", "{}", once_more.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["again"],
            state["position"]["test"],
            state["human_stages"]
        ]),
        json!([["test a5"], {"round": 1, "attempt": 5, "triage": true}, []]),
        "F3: one grant, one more attempt: {state}"
    );

    // THE GRANTED LAP VERIFIES IT, and the triage is finished: a verdict, through the
    // harness, on the candidate the round tested.
    let mut verdict = json!({"status": "verified", "key": key, "verdict": "refuted",
                             "basis": "does-not-reproduce: exit 0 on a fresh rig",
                             "repro": "the block", "pinnable": true, "left_open": []});
    let settled = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            "triage:p1": graded(&[again(key, "jigc doc set", "audit-clean", "breaks")], &["ledger"]),
            format!("verify:{key}"): verdict.take(),
        }}),
    );
    assert_eq!(settled.result["status"], "triaged", "{}", settled.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["next"],
            state["stop"]["then"],
            state["untriaged"],
            state["human_list"],
            state["rounds"][0]["facts"]["awaiting"]
        ]),
        json!(["stop", "close", [], [], []]),
        "the triage is finished by the verdict, and the round is over: {state}"
    );
    assert_eq!(sim.rig.status(), "");
}

/// A scenario's red gate: the suite failed, on these tests. A stabilization run's gates
/// keep going (`dev/gate --keep-going`), so the suite is one step, `test`.
fn red(tests: &[&str]) -> Value {
    json!({"steps": ["test"], "tests": tests})
}

const FLAKY: &str = "jigc::g_tooling a_suite::a_test_that_fails_now_and_then";
const KNOWN: &str = "jigc::g_finalize a_suite::red_on_the_candidate";

/// The small stage every scenario below runs: one audit area that found nothing.
fn small_stage(more: Value) -> Value {
    let mut script = json!({"scope": one_door(), "agents": {"area-a:review": nothing()}});
    for (name, value) in more.as_object().expect("more of a scenario is an object") {
        script[name] = value.clone();
    }
    script
}

/// The subject and the paths of the commit at `HEAD`.
fn head_commit(sim: &Sim) -> (String, Vec<String>) {
    let shown = sim.rig.git(&["show", "--name-only", "--format=%s", "HEAD"]);
    let mut shown = shown.lines();
    let subject = shown.next().expect("a subject").to_owned();
    (
        subject,
        shown
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect(),
    )
}

/// REPAIRED — M4 (harness-agents-gate.md): a record step under a red gate. The step applies
/// its tables as ONE batch, which the record script holds as pending; the gate is red with
/// something the candidate's gate did not show; the commit step refuses, naming it — and the
/// tree is no dirty tree: the next invocation finds the batch, gates it again and commits it.
#[test]
fn a_red_gate_in_a_record_step_leaves_a_pending_batch_that_the_next_invocation_commits() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("red-gate", &one_area());
    let opened = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"record": {"gate": red(&[FLAKY])}})),
    );
    let halted = &ran.result;
    assert_eq!(halted["status"], "halted", "{halted}");
    assert_eq!(halted["halted"]["phase"], "record", "{halted}");
    let reason = halted["halted"]["reason"].as_str().expect("a reason");
    // THE HALT SAYS THE WORD, WHOSE IT IS AND HOW MANY — AND NAMES THE FILE THAT NAMES WHAT
    // IS RED (K3): the tool's refusal is its word in the digest, the sentence is the
    // harness's own, and the names are in the step's own line, which the halt names by its
    // path and its hash and which no agent relayed.
    assert!(
        reason.starts_with("the record's commit step refused `gate-red` — the orchestrator's: ")
            && !reason.contains(FLAKY),
        "the halt says the word and names no test: {reason}"
    );
    assert_eq!(
        json!([
            halted["halted"]["refused"],
            halted["halted"]["whose"],
            halted["halted"]["gate"]["new"],
            halted["halted"]["gate"]["known"]
        ]),
        json!(["gate-red", "the orchestrator's", 2, 0]),
        "{halted}"
    );
    assert_eq!(
        refused_line(halted)["gate"]["new"],
        json!(["step test", format!("test {FLAKY}")]),
        "what is red only with the records is named in the file the halt names"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(opened.clone()),
        "and nothing pushed"
    );

    // WHAT IS LEFT is the applied batch, exactly: the tables it wrote, the journal that
    // holds them, and what the stage's reporters wrote before it.
    let status = sim.rig.status();
    for line in [
        format!(" M {RUN_DIR}/clauses.md"),
        format!("?? {RUN_DIR}/r1/round.md"),
        format!("?? {RUN_DIR}/r1/gate.md"),
        format!("?? {RUN_DIR}/.pending.json"),
        format!("?? {RUN_DIR}/r1/scope.md"),
    ] {
        assert!(status.contains(&line), "`{line}` in:\n{status}");
    }
    let pending = sim.record(&["pending", "--run", RUN], "");
    assert_eq!(
        json!([
            pending["batch"]["subject"],
            pending["batch"]["round"],
            pending["batch"]["altered"]
        ]),
        json!([
            "docs(record): rc24 r1 - the record of the test stage",
            1,
            []
        ])
    );

    // THE HALT SAYS WHAT TO DO, AND IT IS TRUE: invoke the stage again — or take the batch
    // back with the one command it names.
    let message = halted["message"].as_str().expect("a message");
    assert!(
        message.contains("THE BATCH IS STILL APPLIED AND NOT COMMITTED")
            && message.contains("Invoke the stage again: that invocation runs the full gate on the batch once more, commits and pushes it, and does nothing else")
            && message.contains(&format!("`dev/stabilize-step discard --branch {LOOP} --run-dir {RUN_DIR}`"))
            && !message.contains("stabilize-record discard")
            && !message.contains("with the same args"),
        "{message}"
    );
    // The gate is red again, in the same way: the batch stays, and the halt is the same.
    let still = sim.invoke(
        sim.args("test", json!({})),
        json!({"record": {"gate": red(&[FLAKY])}}),
    );
    assert_eq!(
        still.result["halted"]["phase"], "record",
        "{}",
        still.result
    );
    assert_eq!(
        sim.rig.status(),
        status,
        "a second red gate changes nothing"
    );
    // And then it is green: the invocation commits the batch, pushes it, and does nothing
    // else of a stage — no preflight, no instrument, no triage.
    let again = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(again.result["status"], "recorded", "{}", again.result);
    assert_eq!(
        again.agents(),
        [
            "git:state",
            "git:hold-start:record-pending-1",
            "git:hold-start:record-pending-2",
            "git:hold-wait:record-pending-2",
            "git:record:pending",
            "git:push:pending",
            "git:state:test-1"
        ],
        "no executor runs for a batch that is applied; and the gate the invocation before this one held under the first name is ITS gate — red — so the next name is asked"
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(sim.rig.rev("HEAD~1"), opened, "one commit");
    assert_eq!(again.result["record"], head.as_str());
    let (subject, paths) = head_commit(&sim);
    assert_eq!(
        subject,
        "docs(record): rc24 r1 - the record of the test stage"
    );
    assert!(
        paths
            .iter()
            .all(|path| path.starts_with(&format!("{RUN_DIR}/")))
            && !paths.iter().any(|path| path.ends_with(".pending.json"))
            && paths.contains(&format!("{RUN_DIR}/r1/reports/test/area-a-review.a1.md")),
        "the run directory's pending paths, and never the journal: {paths:?}"
    );
    assert_eq!(sim.rig.remote(LOOP), Some(head), "pushed");
    assert_eq!(sim.rig.status(), "", "and the tree is clean");
    let state = sim.state();
    assert_eq!(again.result["next"], state["next"]);
    assert_eq!(state["rounds"][0]["facts"]["candidate"], opened.as_str());
}

/// THE HUMAN'S RULING OF 2026-10-06 (DECISIONS.md → *A stabilization run's record commit
/// under a red candidate*): a record commit is accepted if its gate turns nothing red that
/// the candidate's own gate did not already show red. The candidate's failing tests are on
/// record, in the round; a test that is red only with the records is a halt that names it.
#[test]
fn a_red_candidate_is_recorded_and_a_test_red_only_with_the_records_halts_by_name() {
    if !can_run() {
        return;
    }
    // (1) The candidate's gate is red, and the gate on the tree with the records shows the
    // same red: the round is recorded — the one thing it could not be.
    let sim = Sim::opened("red-candidate", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"gate": red(&[KNOWN]), "record": {"gate": red(&[KNOWN])}})),
    );
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);
    assert_eq!(sim.rig.rev("HEAD~1"), candidate);
    assert_eq!(sim.rig.status(), "");
    let gate = sim.rig.read(&format!("{RUN_DIR}/r1/gate.md"));
    for row in [
        format!("| commit | `{candidate}` |"),
        "| verdict | fail |".to_owned(),
        "| step | `test` |".to_owned(),
        format!("| test | `{KNOWN}` |"),
    ] {
        assert!(gate.contains(&row), "`{row}` is data of the round: {gate}");
    }
    // A later record commit of the round — the human's go — is held to the same list.
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({"record": {"gate": red(&[KNOWN])}}),
    );
    assert_eq!(go.result["status"], "ruled", "{}", go.result);

    // (2) The same candidate, and a test that is red only with the records.
    let sim = Sim::opened("red-with-the-records", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"gate": red(&[KNOWN]), "record": {"gate": red(&[KNOWN, FLAKY])}})),
    );
    let halted = &ran.result;
    assert_eq!(halted["halted"]["phase"], "record", "{halted}");
    assert_eq!(
        json!([
            halted["halted"]["gate"]["new"],
            halted["halted"]["gate"]["known"]
        ]),
        json!([1, 2]),
        "how many are new, and how many the candidate's gate showed: {halted}"
    );
    let line = refused_line(halted);
    assert_eq!(
        line["gate"]["new"],
        json!([format!("test {FLAKY}")]),
        "what the candidate's gate showed red is not named as new: {line}"
    );
    assert_eq!(
        line["gate"]["known"],
        json!(["step test", format!("test {KNOWN}")])
    );
    assert_eq!(sim.rig.rev("HEAD"), candidate);

    // THE OTHER WAY OUT: the batch is taken back — every table as it was, the reports and
    // the scope kept — and the stage is run again, as the next attempt. BY THE COMMAND THE
    // HALT NAMES, taken out of its message and run as it is spelled: the step tool's act,
    // which asks git first.
    let message = halted["message"].as_str().expect("a message");
    let named = message
        .split('`')
        .find(|span| span.starts_with("dev/stabilize-step discard "))
        .unwrap_or_else(|| panic!("the halt names the act that takes the batch back: {message}"));
    sim.rig.shell(named).done("discard", "discarded");
    let status = sim.rig.status();
    assert!(
        !status.contains(" M ") && !status.contains(".pending.json") && !status.contains("gate.md"),
        "no table of the batch is left:\n{status}"
    );
    let rerun = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"gate": red(&[KNOWN]), "record": {"gate": red(&[KNOWN])}})),
    );
    assert_eq!(rerun.result["status"], "triaged", "{}", rerun.result);
    assert_eq!(rerun.result["scope"]["status"], "stands");
    assert_eq!(sim.rig.rev("HEAD~1"), candidate);
    assert_eq!(sim.rig.status(), "");
}

/// REPAIRED — M2 (harness-agents-gate.md): what is accepted as recorded is the commit step's
/// own line, held to its hash, with one result per check the harness composed — never what
/// a record step says of itself. A record step that applied nothing has nothing to show.
#[test]
fn a_record_step_that_returns_no_evidence_of_its_checks_is_not_taken_as_recorded() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("no-evidence", &one_area());
    let opened = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"record": {"apply": "skipped"}})),
    );
    let halted = &ran.result;
    assert_eq!(halted["status"], "halted", "M2: {halted}");
    assert_eq!(halted["halted"]["phase"], "record", "M2: {halted}");
    assert!(
        halted["halted"]["reason"].as_str().is_some_and(
            |reason| reason.starts_with("the record's commit step refused `no-batch` — ")
        ),
        "M2: {halted}"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert_eq!(
        ran.agents()[ran.agents().len() - 2..],
        ["git:hold-wait:record-test-r1-a1-1", "git:record:test:r1"],
        "the record's gate was held, the commit step ran, and is what refused"
    );
    // Nothing is pending, so the stage invoked again works the next attempt.
    let again = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert_eq!(
        again.result["counts"]["reporters"], 4,
        "the attempt's marker, the preflight, the scope step and the one reviewer"
    );
}

/// REPAIRED — the re-review's `R1` (records-state.md; the second repair plan's `K1`): a
/// reporter whose text the record script refused has a shell, and writes the file itself.
/// Until the repair the file was admitted by its name, committed and pushed unscanned. Now
/// the stage's first report check vets what is pending: the file is no report and leaves
/// the tree, its reviewer has left none — so its item is void, by the row a stage has for
/// that — and THE STAGE REACHES ITS RECORD. Neither the commit nor the remote holds the
/// text; the file is kept, outside the repository.
#[test]
fn a_report_overwritten_by_hand_voids_its_item_and_reaches_neither_the_commit_nor_the_remote() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("vet-overwritten", &one_area());
    let text = format!(
        "# a review\n\nIt ran in {}/probe and met {DENY_TERM}.\n\n<!-- end of report -->\n",
        sim.rig.dir().join("home").display()
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"overwrites": {"area-a:review": text}})),
    );
    let result = &ran.result;
    assert_eq!(
        result["status"], "triaged",
        "the stage reached its record: {result}"
    );
    assert_eq!(result["counts"]["voided"], json!(["area-a"]), "{result}");
    let state = sim.state();
    assert_eq!(
        results(&state, 1),
        [json!([
            "area-a",
            1,
            "void",
            "its `review` step left a file at its report's path that is no report (`host-path`): it was set aside, and the step left none"
        ])],
        "the item is void, and says why — the file was set aside (K3): {state}"
    );
    // WHAT WAS COMMITTED AND PUSHED: the reports the script wrote, and not that file.
    assert_eq!(
        sim.reports(1),
        lines(&["attempt.a1.md", "preflight.a1.md", "scope.a1.md"])
    );
    assert_eq!(sim.rig.status(), "");
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(sim.rig.rev("HEAD")),
        "the record is pushed"
    );
    assert_eq!(
        sim.rig
            .git(&["log", "--all", "--format=%H", &format!("-S{DENY_TERM}")]),
        "",
        "and no commit of the repository — so none of the remote — holds the text"
    );
    // THE FILE IS KEPT, OUTSIDE THE REPOSITORY, UNDER THE INVOCATION'S SCRATCH ROOT (K3: the
    // report check is handed it) — and no directory was minted under the temp directory.
    fn files_below(dir: &Path, found: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                files_below(&path, found);
            } else {
                found.push(path);
            }
        }
    }
    let mut aside = Vec::new();
    files_below(&sim.rig.scratch.join("refused"), &mut aside);
    assert_eq!(aside.len(), 1, "one file was set aside, once: {aside:?}");
    assert_eq!(
        aside[0].file_name().and_then(|name| name.to_str()),
        Some("r1__reports__test__area-a-review.a1.md"),
        "{aside:?}"
    );
    assert_eq!(
        fs::read_to_string(&aside[0]).expect("the file that was set aside"),
        text
    );
    assert!(
        !fs::read_dir(sim.rig.dir().join("tmp"))
            .expect("the rig's temp directory")
            .any(|entry| entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .starts_with("jigc-stabilize-refused-")),
        "the place is the one the harness named, and none the script minted"
    );
}

/// The hash comparison of ruling 11, driven: a driver that asserts another binary than the
/// one the stage built drove something else, and nothing it drove is evidence about the
/// candidate. (The whole-stage test could not see this comparison removed: every driver
/// there measures the file it was handed.) TURNED BY THE SECOND REPAIR PLAN'S `X1`: until
/// then the stage halted here, with nothing recorded and every instrument to run again —
/// where a driver that NOTICED the mismatch and halted, as its definition tells it to,
/// voided its item and nothing else. The comparison the harness makes is that halt, made
/// for the agent: the item is void, with the reporter and both hashes in the reason, and
/// the stage reaches its record.
#[test]
fn a_driver_that_drove_another_binary_voids_its_item_and_the_stage_records() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened(
        "wrong-binary",
        &Opening {
            clauses: vec!["audit-clean"],
            items: json!([item("drive-a", "audit-drive", "audit-clean")]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    let opened = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"drive-a:drive": nothing()},
               "wrongBinary": ["drive-a:drive"]}),
    );
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(
        json!([result["counts"]["voided"], result["unasserted"]]),
        json!([["drive-a"], [{"reporter": "drive-a-drive", "asserted": "0".repeat(64)}]]),
        "{result}"
    );
    let state = sim.state();
    let rows = results(&state, 1);
    assert_eq!(json!([rows[0][0], rows[0][2]]), json!(["drive-a", "void"]));
    assert!(
        rows[0][3].as_str().is_some_and(|reason| reason.starts_with(&format!(
            "its `drive` step (drive-a-drive) returned the `asserted_sha256` {}, which is not the candidate's binary (",
            "0".repeat(64)
        ))),
        "the void's reason names the reporter, the field and both hashes: {rows:?}"
    );
    assert_ne!(
        sim.rig.rev("HEAD"),
        opened,
        "the stage's record is committed"
    );
    assert_eq!(sim.rig.status(), "");
    assert_eq!(state["retest"], json!(["audit-clean"]), "{state}");
}

/// The commit step of a record, driven as an act (`dev/stabilize-step record`): it commits
/// exactly the applied batch and what the stage's reporters left, under the subject the
/// batch carries, when the gate holds — and refuses, committing and undoing nothing, a
/// batch that is not the one the step names, a file of it changed since, a stray file, a
/// gate that is newly red, and a gate's output that is none.
#[test]
fn the_records_commit_step_commits_exactly_the_applied_batch_or_nothing() {
    let sim = Sim::opened("commit-step", &one_area());
    let opened = sim.rig.rev("HEAD");
    let scratch = sim.rig.scratch.display().to_string();
    let gate = |name: &str, text: &str| -> String {
        let file = format!("{scratch}/{name}.txt");
        fs::write(&file, text).expect("write a gate's output");
        file
    };
    let green = gate(
        "green",
        "tests   passed=9 failed=0  (over 2 test binaries)\n\nGATE: PASS\n",
    );
    let red = gate(
        "red",
        "gate: mode   full, keep-going (no red step stops the run but a red build; the suite is ONE step, `test` -- what is named red is all that is red)\ntests   passed=8 failed=1  (over 2 test binaries)\n\nGATE: FAIL (step: test)\n\nfailing tests:\n  jigc::g_a s::one\n",
    );
    // The same red, of a gate that stopped at its first red stage: what it names red is
    // not all that is red, so it is held to nothing.
    let stopped = gate(
        "stopped",
        "gate: mode   full\ntests   passed=8 failed=1  (over 2 test binaries)\n\nGATE: FAIL (step: tier1)\n\nfailing tests:\n  jigc::g_a s::one\n",
    );
    let pre_check = gate(
        "pre-check",
        "PRE-CHECK: PASS -- fmt, clippy, build and the fast test tier\n",
    );
    let record = |file: &str, calls: &str, branch: &str| {
        sim.rig.step(&[
            "record",
            "--branch",
            branch,
            "--run-dir",
            RUN_DIR,
            "--gate",
            file,
            "--calls",
            calls,
            "--checks",
            "1",
        ])
    };
    let facts = format!("{RUN_DIR}/run.md");

    // No batch applied: there is nothing this step records.
    record(&green, "2", LOOP).refused("no-batch");
    let batch = json!([
        {"argv": ["run-set", "--run", RUN], "stdin": json!({"rounds": 3}).to_string()},
        {"argv": ["check-ledger", "--run", RUN, "--"]},
    ]);
    sim.record(
        &["apply", "--run", RUN, "--subject", "docs(record): a record"],
        &batch.to_string(),
    );
    let applied = sim.rig.read(&facts);
    let pending = sim.rig.status();

    // The git state a stage starts from says what is pending, and is no refusal.
    let product = ["Cargo.toml".to_owned()];
    let state = sim.rig.step(&[
        "git-state",
        "--stage",
        "test",
        "--loop",
        LOOP,
        "--rounds",
        "fix/rc24-r",
        "--run-dir",
        RUN_DIR,
        "--product",
        &product[0],
    ]);
    assert_eq!(
        state.done("git-state", "ready")["pending"],
        json!({"round": null, "subject": "docs(record): a record", "calls": 2,
               "checks": [{"check": "ledger", "ok": true, "missing": [], "duplicated": []}],
               "files": [facts]})
    );

    for (what, seen, word) in [
        (
            "another branch than the one checked out",
            record(&green, "2", "fix/rc24-r1"),
            "wrong-branch",
        ),
        (
            "a batch of another number of calls",
            record(&green, "3", LOOP),
            "no-batch",
        ),
        (
            "a gate that is red, with no candidate's gate on record",
            record(&red, "2", LOOP),
            "gate-red",
        ),
        (
            "a pre-check's output",
            record(&pre_check, "2", LOOP),
            "record",
        ),
        (
            "a red gate that did not keep going",
            record(&stopped, "2", LOOP),
            "record",
        ),
        (
            "a gate's output that is not there",
            record(&format!("{scratch}/none.txt"), "2", LOOP),
            "record",
        ),
    ] {
        seen.refused(word);
        assert_eq!(sim.rig.rev("HEAD"), opened, "{what}: nothing is committed");
        assert_eq!(sim.rig.status(), pending, "{what}: and nothing undone");
    }
    assert_eq!(
        record(&red, "2", LOOP).refused("gate-red")["gate"]["new"],
        json!(["step test", "test jigc::g_a s::one"]),
        "the refusal's line carries what is newly red"
    );
    // A file of the batch changed after the batch wrote it, and a file nobody wrote.
    sim.rig.write(&facts, &applied.replace("`3`", "`4`"));
    record(&green, "2", LOOP).refused("dirty");
    sim.rig.write(&facts, &applied);
    sim.rig.write(&format!("{RUN_DIR}/notes.md"), "a note\n");
    record(&green, "2", LOOP).refused("dirty");
    fs::remove_file(sim.rig.root.join(format!("{RUN_DIR}/notes.md"))).expect("remove the note");
    assert_eq!(sim.rig.rev("HEAD"), opened);

    // THE COMMIT: the pending paths by name, one commit, the batch's subject, a clean tree.
    let done = record(&green, "2", LOOP);
    let line = done.done("record", "recorded");
    let head = sim.rig.rev("HEAD");
    assert_eq!(sim.rig.rev("HEAD~1"), opened);
    assert_eq!(
        json!([
            line["commit"],
            line["branch"],
            line["paths"],
            line["applied"]["calls"],
            line["gate"]["ok"]
        ]),
        json!([head, LOOP, [facts], 2, true])
    );
    assert_eq!(
        head_commit(&sim),
        ("docs(record): a record".to_owned(), vec![facts.clone()])
    );
    assert_eq!(sim.rig.status(), "");
    assert_eq!(
        done.trace,
        lines(&[
            "branch --show-current",
            "rev-parse --git-path index.lock",
            "rev-parse HEAD",
            "status --porcelain --untracked-files=all",
            &format!("add -- {facts}"),
            &format!("diff --name-only -- {facts}"),
            "commit -q -m docs(record): a record",
            "rev-parse HEAD",
            "status --porcelain --untracked-files=all",
        ]),
        "the commands of the act, in order: the branch and git's lock before anything else, what is staged held to the tree once it is vetted, nothing pushed, and no path added by a directory"
    );
    // Asked again as the harness asks today — with no word of the commit the stage began
    // on — nothing says that the last commit is this record's: there is nothing to record.
    record(&green, "2", LOOP).refused("no-batch");

    // A write of the record that was killed between its files is FINISHED by the read a
    // stage starts from, which says so (the second repair plan's `P1`): no state is read
    // across it, and none is left for a hand.
    fs::write(
        sim.rig.root.join(format!("{RUN_DIR}/.pending.json")),
        "{\"placing\": [{\"path\": \"ledger.md\", \"temporary\": \".stabilize-record.x.tmp\", \"replace\": true}], \"batch\": null}\n",
    )
    .expect("a journal");
    let killed = sim.rig.step(&[
        "git-state",
        "--stage",
        "test",
        "--loop",
        LOOP,
        "--rounds",
        "fix/rc24-r",
        "--run-dir",
        RUN_DIR,
        "--product",
        &product[0],
    ]);
    assert_eq!(
        killed.done("git-state", "ready")["finished"],
        json!(["interrupted-write", "unpushed"]),
        "the killed write is finished, and the record no push took is pushed: {}",
        killed.raw
    );
    assert_eq!(
        sim.record(&["pending", "--run", RUN], "")["interrupted"],
        false
    );
}

/// **An attempt begins on record, and only where the state hands it out** — the act
/// `dev/stabilize-step begin`, driven. It is the first write of every attempt of a stage:
/// it holds the round and the attempt to the state's own position once more, and writes the
/// attempt's marker. So NO STAGE BEGINS ON A RUN WHOSE OPENING IS NOT DONE — by the tool,
/// whatever the harness read — nor as an attempt the state does not hand out.
#[test]
fn an_attempt_is_begun_on_record_by_one_act_and_only_where_the_state_hands_it_out() {
    let begin = |sim: &Sim, stage: &str, round: &str, attempt: &str, scans: bool| {
        // The environment a write of the record needs — the denylist, a temp directory —
        // as every reporter of a stage has it; without it the scan cannot run.
        let environment = if scans {
            format!(
                "JIGC_DENYLIST_FILE='{}' TMPDIR='{}/' ",
                sim.rig.dir().join("denylist").display(),
                sim.rig.dir().join("tmp").display()
            )
        } else {
            format!(
                "JIGC_DENYLIST_FILE='{}' ",
                sim.rig.dir().join("no-denylist").display()
            )
        };
        sim.rig.shell(&format!(
            "{environment}dev/stabilize-step begin --run {RUN} --round {round} --stage {stage} --attempt {attempt} --reporter attempt --commit {} --scratch {}",
            sim.rig.rev("HEAD"),
            sim.rig.scratch.display()
        ))
    };

    // THE OPENING IS NOT DONE: its record is committed, and no fact of the run is written.
    let early = Sim::new("begin-early");
    early
        .rig
        .write(&format!("{RUN_DIR}/opening.md"), "# the opening record\n");
    early.rig.commit("docs(record): the run is opened");
    let refused = begin(&early, "test", "1", "1", true);
    assert!(
        refused.refused("position")["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains("the opening still owes no-clause-row")),
        "the refusal says what the opening owes: {}",
        refused.raw
    );
    assert_eq!(early.rig.status(), "", "and nothing was written");

    // AN OPENED RUN: the state hands the `test` stage round 1, attempt 1 — and nothing else.
    let sim = Sim::opened("begin", &one_area());
    for (stage, round, attempt, what) in [
        ("test", "1", "2", "an attempt the state does not hand out"),
        ("test", "2", "1", "a round the state does not hand out"),
        ("fix", "1", "1", "a stage the state refuses"),
    ] {
        begin(&sim, stage, round, attempt, true).refused("position");
        assert_eq!(sim.rig.status(), "", "{what}: nothing was written");
    }
    // A scanner that cannot run is the record script's refusal, and the attempt is not begun.
    begin(&sim, "test", "1", "1", false).refused("record");
    assert_eq!(sim.rig.status(), "");

    let begun = begin(&sim, "test", "1", "1", true);
    let line = begun.done("begin", "begun");
    let marker = format!("{RUN_DIR}/r1/reports/test/attempt.a1.md");
    assert_eq!(
        json!([line["round"], line["attempt"], line["report"]]),
        json!(["1", "1", marker])
    );
    assert!(
        begun.trace.is_empty(),
        "the act runs no git: {:?}",
        begun.trace
    );
    let text = sim.rig.read(&marker);
    assert!(
        text.contains(&format!("on the commit {}", sim.rig.rev("HEAD")))
            && text.trim_end().ends_with("<!-- end of report -->"),
        "the marker says which commit the attempt began on: {text}"
    );
    // THE ATTEMPT IS ON RECORD: the state counts it, and hands out the next one — so the
    // same attempt cannot be begun twice, by a second invocation or by a retried step.
    let state = sim.state();
    assert_eq!(
        json!([
            state["rounds"][0]["test_unrecorded"],
            state["position"]["test"]
        ]),
        json!([1, {"round": 1, "attempt": 2}])
    );
    begin(&sim, "test", "1", "1", true).refused("position");
    begin(&sim, "test", "1", "2", true).done("begin", "begun");
}

/// REPAIRED — the build record's *Found while the workflow's doc was written*, item 1: a
/// stage stopped after its scope step. The scope it wrote is a pending write — a file the
/// record script wrote and no record step committed — and no dirty tree, exactly as its
/// reports are: the invocation that follows runs the stage as the next attempt, the scope
/// stands, and the record commit holds both attempts.
#[test]
fn a_stage_stopped_after_its_scope_step_is_run_again_by_the_invocation_that_follows() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("stopped", &one_area());
    let opened = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({"stopAfter": "preflight"})),
        json!({"scope": one_door()}),
    );
    assert_eq!(ran.result["status"], "stopped", "{}", ran.result);
    assert_eq!(ran.result["after"], "preflight");
    assert_eq!(
        sorted(ran.reporters.clone()),
        lines(&["attempt", "preflight", "scope"])
    );
    assert_eq!(
        sim.reports(1),
        lines(&["attempt.a1.md", "preflight.a1.md", "scope.a1.md"]),
        "what a stopped stage leaves: its attempt's marker, the reports written so far — and the round's scope, below"
    );
    let mut launched = lines(&["git:state", "git:state:test-1", "git:begin", "preflight"]);
    for name in [
        candidate_build(&opened),
        previous_build(&sim),
        candidates_gate(&opened),
    ] {
        launched.extend(held_steps(&name));
    }
    assert_eq!(
        ran.agents(),
        launched
            .iter()
            .map(String::as_str)
            .chain(["scope", "git:state:test-2"])
            .collect::<Vec<_>>(),
        "the preflight, then what the tool holds — the two builds and the candidate's gate — beside the scope step"
    );
    // A stop after `state` is before the attempt begins: it leaves nothing at all.
    let nothing_left = Sim::opened("stopped-at-state", &one_area());
    let at_state = nothing_left.invoke(
        nothing_left.args("test", json!({"stopAfter": "state"})),
        json!({}),
    );
    assert_eq!(at_state.result["status"], "stopped", "{}", at_state.result);
    assert_eq!(at_state.trace, lines(&TWO_READS));
    assert_eq!(nothing_left.rig.status(), "");
    assert!(
        sim.rig
            .status()
            .contains(&format!("?? {RUN_DIR}/r1/scope.md")),
        "the round's scope is written and in no commit"
    );

    let again = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert_eq!(again.result["round"], 1);
    assert_eq!(
        again.result["scope"]["status"], "stands",
        "a round's scope is written once"
    );
    assert_eq!(
        sim.reports(1),
        lines(&[
            "area-a-review.a2.md",
            "attempt.a1.md",
            "attempt.a2.md",
            "preflight.a1.md",
            "preflight.a2.md",
            "scope.a1.md",
            "scope.a2.md"
        ]),
        "the second attempt's reports beside the first's"
    );
    assert_eq!(sim.rig.rev("HEAD~1"), opened, "one record commit");
    let (_, paths) = head_commit(&sim);
    assert_eq!(
        paths.len(),
        7 + 5,
        "seven reports, the scope, the gate, the items' results, the round's facts and the clause table: {paths:?}"
    );
    assert!(paths.contains(&format!("{RUN_DIR}/r1/scope.md")));
    assert_eq!(sim.rig.status(), "");
}

#[test]
fn a_red_check_is_filed_as_a_finding_and_a_go_is_recorded_without_starting_anything() {
    if !can_run() {
        return;
    }
    let opening = Opening {
        clauses: vec!["gate-green", "audit-clean"],
        items: json!([
            item("gate", "check", "gate-green"),
            item("area-a", "audit-area", "audit-clean"),
        ]),
        bounds: vec![],
        rows: json!([]),
    };
    let sim = Sim::opened("red-check", &opening);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"checks": {"gate": "red"}, "scope": one_door(),
               "agents": {"area-a:review": nothing()}}),
    );
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");

    // TURNED — the lead *a red check without a ledger row* (repair-plan.md §2;
    // stabilization-workflow.md → *A red deterministic check*). A RED CHECK IS A FINDING:
    // the call that records the red files it — one ledger row, under a key of the item's
    // own, ungraded and open — so the round's triage is not finished, and the next triage
    // that runs grades it like any other; then it is a fixer's, or the human's. It is no
    // blocker yet, and nobody's re-run: the round is not over, so the run does not stop.
    assert_eq!(
        result["state"]["ledger"], 1,
        "the ledger is a count in the return: {result}"
    );
    let read = document(result);
    let ledger = read["ledger"].as_array().expect("the ledger");
    assert_eq!(ledger.len(), 1, "{result}");
    assert_eq!(
        json!([
            ledger[0]["key"],
            ledger[0]["round"],
            ledger[0]["clause"],
            ledger[0]["grade"],
            ledger[0]["disposition"],
            ledger[0]["route"]
        ]),
        json!(["red-gate", 1, "gate-green", "ungraded", "open", "triage"]),
        "{result}"
    );
    assert_eq!(result["counts"]["blockers"], 0);
    assert_eq!(
        result["forbids_close"],
        json!({"clauses": [{"clause": "gate-green", "status": "red"}], "no_clauses": false,
               "findings": 1, "candidate": false}),
        "what forbids closing: the clause by name, and how many findings"
    );
    assert_eq!(
        read["forbids_close"],
        json!([{"clause": "gate-green", "status": "red"},
               {"finding": "red-gate", "route": "triage"}])
    );
    assert_eq!(
        read["untriaged"],
        json!([{"key": "red-gate", "why": "ungraded"}])
    );
    assert_eq!(result["state"]["retest"], json!([]));
    assert_eq!(result["next"], "triage", "{result}");
    assert_eq!(result["returned_to_orchestrator"], true);
    assert_eq!(
        result["state"]["position"]["test"],
        json!({"round": 1, "attempt": 2, "triage": true}),
        "the stage that left the triage unfinished is the one that finishes it"
    );
    assert_eq!(result["state"]["stop"], Value::Null);

    // … AND IT IS GRADED THROUGH THE HARNESS — the dependency the derived clause left
    // (M6): the invocation that finishes the round's triage is handed the ledger's row,
    // grades it under its own key, has it verified on the candidate the round tested, and
    // records. A red check that is no regression stands at no door of the round's scope, so
    // it is the human's — as a finding found outside the test set.
    let mut confirmed = json!({"status": "verified", "key": "red-gate", "verdict": "confirmed",
                               "regression": false, "basis": "the gate is red on a fresh clone",
                               "repro": "the block", "pinnable": true, "left_open": []});
    let lap = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            "triage:p1": graded(&[again("red-gate", "the check `gate`", "gate-green", "breaks")], &["ledger"]),
            "verify:red-gate": confirmed.take(),
        }}),
    );
    assert_eq!(lap.result["status"], "triaged", "{}", lap.result);
    assert_eq!(
        lap.agents(),
        finishing_lap(&sim, 2, &["red-gate"], "git:state:test-2")
    );
    let state = sim.state();
    let graded_row = &state["ledger"][0];
    assert_eq!(
        json!([
            graded_row["key"],
            graded_row["grade"],
            graded_row["graded_by"],
            graded_row["triage"]["door"],
            graded_row["route"],
            graded_row["why"],
            state["next"],
            state["untriaged"]
        ]),
        json!([
            "red-gate",
            "confirmed",
            "verify-real",
            "unlisted",
            "human",
            "outside",
            "rule",
            []
        ]),
        "{state}"
    );
    assert_eq!(lap.result["next"], "rule");

    // THE RUN STOPS AFTER EVERY ROUND, and `stop` is a value the script only relays: a
    // round whose check is green leaves nothing open, and is over.
    let sim = Sim::opened("green-check", &opening);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"checks": {"gate": "green"}, "scope": one_door(),
               "agents": {"area-a:review": nothing()}}),
    );
    let result = &ran.result;
    assert_eq!(result["next"], "stop", "{result}");
    assert_eq!(result["returned_to_orchestrator"], true);
    assert_eq!(
        result["state"]["stop"],
        json!({"why": "every-round", "round": 1, "then": "close"})
    );
    let tested = sim.rig.rev("HEAD");

    // THE HUMAN'S GO is its own invocation: recorded on the loop branch, pushed, and
    // nothing started — `next` then names the step the stop was holding.
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(go.result["status"], "ruled", "{}", go.result);
    assert_eq!(go.result["next"], "close");
    assert_eq!(
        go.agents(),
        [
            "git:state",
            "git:state:test-1",
            "record:rulings:r1",
            "git:hold-start:record-rulings-r1-1",
            "git:hold-wait:record-rulings-r1-1",
            "git:record:rulings:r1",
            "git:push:rulings",
            "git:state:test-2"
        ]
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(sim.rig.rev("HEAD~1"), tested, "one commit");
    assert_eq!(go.result["record"], head.as_str());
    assert_eq!(sim.rig.remote(LOOP), Some(head), "pushed");
    assert_eq!(sim.state()["rounds"][0]["facts"]["go"], true);
}

/// **A re-run is an attempt inside the round that selected the item — never a round of its
/// own.** A check that could not run leaves its clause void; with nothing open the state asks
/// for that check again, and the `test` stage's position IS that re-run. The invocation that
/// answers it names the clause, launches no scope step, records one result and no fact of
/// the round — and an invocation the state did not ask for a re-run of, or that answers
/// `retest` without one, is refused before any agent runs.
#[test]
fn a_rerun_is_an_attempt_inside_its_round_begins_no_round_and_is_refused_where_the_state_asks_for_none()
 {
    if !can_run() {
        return;
    }
    let sim = Sim::opened(
        "rerun",
        &Opening {
            clauses: vec!["gate-green", "audit-clean"],
            items: json!([
                item("gate", "check", "gate-green"),
                item("area-a", "audit-area", "audit-clean"),
            ]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    // A `clause` nobody asked for: before round 1 the state asks for no re-run.
    let early = sim.invoke(sim.args("test", json!({"clause": "gate-green"})), json!({}));
    assert_eq!(early.result["status"], "refused", "{}", early.result);
    assert_eq!(early.agents(), ["git:state", "git:state:test-1"]);

    // Round 1: the check could not run. Nothing is open, so the round is over — the run
    // stops after it — and what follows the go is the check, again.
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"checks": {"gate": "void"}, "scope": one_door(),
               "agents": {"area-a:review": nothing()}}),
    );
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);
    assert_eq!(
        ran.result["state"]["stop"],
        json!({"why": "every-round", "round": 1, "then": "retest"})
    );
    assert_eq!(ran.result["counts"]["voided"], json!(["gate"]));
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(go.result["next"], "retest", "{}", go.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["retest"],
            state["position"]["test"]["round"],
            state["position"]["test"]["attempt"],
            state["position"]["test"]["rerun"]["clause"],
            state["position"]["test"]["rerun"]["items"]
        ]),
        json!([["gate-green"], 1, 2, "gate-green", [{"item": "gate", "attempt": 2}]]),
        "{state}"
    );
    let asked = sim.rig.rev("HEAD");

    // `retest` answered without the clause, and with another clause: refused, and nothing
    // ran beyond the two reads — no round is begun on a guess.
    for (wrong, why) in [
        (json!({}), "an invocation that would begin a round"),
        (
            json!({"clause": "audit-clean"}),
            "a clause that is not the one that is due",
        ),
    ] {
        let refused = sim.invoke(sim.args("test", wrong), json!({}));
        assert_eq!(
            refused.result["status"], "refused",
            "{why}: {}",
            refused.result
        );
        assert_eq!(refused.agents(), ["git:state", "git:state:test-1"], "{why}");
    }
    assert_eq!(
        sim.rig.rev("HEAD"),
        asked,
        "a refused invocation commits nothing"
    );
    assert_eq!(sim.rig.status(), "");

    // The re-run: the check, and nothing else — no scope step, no item beside it. One record
    // commit, pushed. A ROUND TESTS ONE CANDIDATE (the orchestrator's ruling of 2026-10-07 on
    // the core review's F3): the round's record and the go lie on the candidate by now, so
    // the tip is another commit — and the re-run builds THE CANDIDATE, from its commit, and
    // names it in its result. A check reads the working tree, which is no longer the
    // candidate: it is not asked for, and is void with that reason — never run on the tip
    // and recorded green for a commit it did not run on.
    let candidate = ran.result["candidate"]["sha"]
        .as_str()
        .expect("the candidate")
        .to_owned();
    assert_ne!(asked, candidate, "two record commits moved the tip");
    let again = sim.invoke(sim.args("test", json!({"clause": "gate-green"})), json!({}));
    let result = &again.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(
        json!([
            result["round"],
            result["rerun"]["clause"],
            result["rerun"]["round"],
            result["counts"]["items_run"]
        ]),
        json!([1, "gate-green", 1, 1]),
        "{result}"
    );
    assert_eq!(
        again.agents(),
        rerun_agents(&sim, 2, &[]),
        "a re-run launches no scope step, no gate of the candidate's and no item it was not asked for"
    );
    assert_eq!(sim.rig.rev("HEAD~1"), asked, "one record commit");
    let (subject, paths) = head_commit(&sim);
    assert_eq!(
        subject,
        format!("docs(record): {RUN} r1 - a re-run of gate-green")
    );
    assert_eq!(
        paths,
        lines(&[
            &format!("{RUN_DIR}/clauses.md"),
            &format!("{RUN_DIR}/r1/reports/test/attempt.a2.md"),
            &format!("{RUN_DIR}/r1/reports/test/preflight.a2.md"),
            &format!("{RUN_DIR}/r1/results.md"),
            &format!("{RUN_DIR}/r1/round.md"),
        ]),
        "the re-run's marker and its one report, its result, the clause table the result rendered — and the round's record, in which the record script counts the item's runs and names the attempt of the stage that reached its record"
    );
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");

    // It began no round, wrote no fact of round 1, and is on record as attempt 2 of it — OF
    // THE ROUND'S CANDIDATE, as every result of the round is. What the re-run built, began
    // on and recorded is that commit, in the words each agent was handed.
    let preflight = &again
        .prompts
        .iter()
        .find(|(label, _)| label == "preflight")
        .expect("the re-run's preflight")
        .1;
    assert!(
        preflight.contains(&format!(
            "Candidate: label c1, commit {candidate} — the commit round 1 tested, an ancestor of"
        )) && preflight.contains("THE CANDIDATE IS NOT `HEAD` HERE")
            && !preflight.contains("the deterministic checks")
            && !preflight.contains("dev/gate"),
        "the preflight builds the candidate from its commit, and is asked for nothing that reads the tree: {preflight}"
    );
    assert!(
        again
            .commands
            .iter()
            .any(|ran| ran.starts_with("dev/stabilize-step begin ")
                && ran.contains(&format!(" --commit {candidate} ")))
            && !again
                .commands
                .iter()
                .any(|ran| ran.contains(&asked) && !ran.starts_with("dev/stabilize-step record ")),
        "the attempt begins on the candidate, and the tip is named to nothing but the commit step, which is held to it: {:?}",
        again.commands
    );
    let record = &again
        .prompts
        .iter()
        .find(|(label, _)| label == "record:test:r1")
        .expect("the re-run's record")
        .1;
    assert!(
        record.contains(&format!(
            "\"result-set\",\"--run\",\"{RUN}\",\"--round\",\"1\",\"--commit\",\"{candidate}\""
        )) && !record.contains(&asked),
        "the result is written for the candidate: {record}"
    );
    let state = sim.state();
    assert_eq!(state["round"], 1, "{state}");
    assert_eq!(
        state["rounds"][0]["facts"]["candidate"], ran.result["candidate"]["sha"],
        "{state}"
    );
    let results = state["rounds"][0]["results"]
        .as_array()
        .expect("the results");
    let gate: Vec<Value> = results
        .iter()
        .filter(|row| row["item"] == "gate")
        .map(|row| json!([row["attempt"], row["outcome"], row["commit"]]))
        .collect();
    assert_eq!(
        gate,
        [json!([1, "void", candidate]), json!([2, "void", candidate]),],
        "every result of the round names the round's candidate: {state}"
    );
    assert!(
        results.iter().any(|row| row["attempt"] == 2
            && row["reason"]
                .as_str()
                .is_some_and(|why| why.contains("is no longer the tree checked out")
                    && why.contains(&candidate))),
        "{state}"
    );
    // The check is still not green after its one re-run: the clause is the human's.
    assert_eq!(
        json!([result["next"], result["rule"]["clauses"]]),
        json!(["rule", [{"clause": "gate-green", "why": "not-green-after-its-rerun"}]]),
        "{result}"
    );
}

// ---------------------------------------------------------------------------
// How an agent can end, and what the stage does then (the harness review's M5 and M9)
// ---------------------------------------------------------------------------

/// A verifier's verdict.
fn verdict(key: &str, said: &str) -> Value {
    let mut v = json!({"status": "verified", "key": key, "verdict": said,
                       "basis": "scripted", "repro": format!("the block of {key}"),
                       "pinnable": true, "left_open": []});
    if said == "confirmed" {
        v["regression"] = json!(false);
    }
    v
}

/// The results a round's record holds, as `[item, attempt, outcome, reason]`.
fn results(state: &Value, round: usize) -> Vec<Value> {
    state["rounds"][round - 1]["results"]
        .as_array()
        .expect("the round's results")
        .iter()
        .map(|row| json!([row["item"], row["attempt"], row["outcome"], row["reason"]]))
        .collect()
}

/// **An instrument's reporter that dies, halts or returns with no report voids its item —
/// with the step and the reason — and never the stage.** Every way a step of a chain can
/// end short of "a result and its report", in one stage: every other item runs, the stage
/// reaches its record, and what each item did is one result. A finding a halted reporter
/// did return, with its report, is triaged; one returned with no report behind it is not.
/// Then the state asks for the void items again, and the re-run — A HUNT'S, over the doors
/// of the round that selected it — is driven through the harness.
#[test]
fn an_instruments_reporter_that_does_not_report_voids_its_item_and_the_hunt_is_run_again() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened(
        "void-hunts",
        &Opening {
            clauses: vec!["audit-clean", "no-lost-files"],
            items: json!([
                item("area-a", "audit-area", "audit-clean"),
                item("cross", "audit-cross-cutting", "audit-clean"),
                item("drive-a", "audit-drive", "audit-clean"),
                item("row-a", "review-row", "no-lost-files"),
                item("area-b", "audit-area", "no-lost-files"),
            ]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    let halt = |cause: &str| {
        json!({"status": "halted", "findings": [],
               "halt": {"root_cause": cause, "evidence": "scripted", "tree_state": "clean",
                        "recommendation": "none"}})
    };
    let mut halted_with_a_finding = halt("the rig could not be built");
    halted_with_a_finding["findings"] = json!([finding(
        "d-1",
        "a wrong exit code",
        "jigc doc set",
        "audit-clean"
    )]);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({
            "scope": one_door(),
            "agents": {
                // dies: nothing returned, on every try, and no report.
                "area-a:review": nothing(),
                // returns findings, and wrote no report.
                "cross:review": found(&[finding("c-1", "a claim with nothing behind it", "jigc doc set", "audit-clean")]),
                // halts, WITH its report — and with what it found before it stopped.
                "drive-a:drive": halted_with_a_finding,
                // one of two parallel steps halts with no report: the chain stops there.
                "row-a:source": nothing(),
                "row-a:driver": halt("the binary's hash did not match"),
                "area-b:review": nothing(),
                "triage:p1": graded(
                    &[entry("doc-set-exit-code", "drive-a-drive d-1", "jigc doc set", "audit-clean", "no-break")],
                    &["drive-a-drive"],
                ),
            },
            "endings": {"area-a:review": "dies", "cross:review": "unreported",
                        "row-a:driver": "halts-unreported"},
        }),
    );
    let result = &ran.result;
    assert_eq!(
        result["status"], "triaged",
        "M9: the stage reached its record: {result}"
    );
    assert_eq!(
        result["counts"]["voided"],
        json!(["area-a", "cross", "drive-a", "row-a"]),
        "{result}"
    );
    assert_eq!(
        ran.agents()
            .iter()
            .filter(|label| **label == "area-a:review")
            .count(),
        3,
        "an agent that returns nothing is tried three times, and then it is dead"
    );
    assert!(
        !ran.agents().contains(&"row-a:reconciler"),
        "the rest of a chain whose step did not report is not launched"
    );
    // WHAT IS ON RECORD: one result per item, each void with the step and how it ended.
    let state = sim.state();
    assert_eq!(
        results(&state, 1),
        [
            json!(["area-a", 1, "void", "its `review` step returned nothing"]),
            json!([
                "cross",
                1,
                "void",
                "its `review` step returned and left no report"
            ]),
            json!([
                "drive-a",
                1,
                "void",
                "its `drive` step halted (the rig could not be built)"
            ]),
            json!([
                "row-a",
                1,
                "void",
                "its `driver` step halted (the binary's hash did not match)"
            ]),
            json!(["area-b", 1, "green", null]),
        ],
        "{state}"
    );
    // Exactly the reports that exist are the stage's: the halted drive's among them, the
    // dead reviewer's and the two that were never written not.
    assert_eq!(
        sim.reports(1),
        lines(&[
            "area-b-review.a1.md",
            "attempt.a1.md",
            "drive-a-drive.a1.md",
            "preflight.a1.md",
            "row-a-source.a1.md",
            "scope.a1.md",
            "triage-p1.a1.md",
        ])
    );
    assert_eq!(sim.rig.status(), "", "and every one of them is committed");
    // THE FINDING A HALTED REPORTER RETURNED, WITH ITS REPORT, IS A ROW; the one that came
    // with no report is none — its item is void, and its re-run finds it again or does not.
    let keys: Vec<&str> = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .map(|row| row["key"].as_str().expect("a key"))
        .collect();
    assert_eq!(keys, ["doc-set-exit-code"]);
    assert_eq!(result["counts"]["findings_in"], 1, "{result}");

    // NOTHING IS OPEN, SO THE STATE ASKS FOR THE VOID ITEMS AGAIN — after the stop.
    assert_eq!(
        result["state"]["stop"],
        json!({"why": "every-round", "round": 1, "then": "retest"})
    );
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(go.result["next"], "retest", "{}", go.result);
    let state = sim.state();
    assert_eq!(state["retest"], json!(["audit-clean", "no-lost-files"]));
    assert_eq!(
        json!([
            state["position"]["test"]["rerun"]["clause"],
            state["position"]["test"]["rerun"]["items"],
            state["position"]["test"]["rerun"]["doors"][0]["door"]
        ]),
        json!(["audit-clean",
               [{"item": "area-a", "attempt": 2}, {"item": "cross", "attempt": 2}, {"item": "drive-a", "attempt": 2}],
               "jigc doc set"]),
        "{state}"
    );
    // THE RE-RUN OF THE HUNTS: the three items of the clause, each as its second attempt in
    // round 1, over that round's doors — no scope step, and no item it was not asked for.
    let again = sim.invoke(
        sim.args("test", json!({"clause": "audit-clean"})),
        json!({"agents": {"area-a:review": nothing(), "cross:review": nothing(), "drive-a:drive": nothing()}}),
    );
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert_eq!(
        again.agents(),
        rerun_agents(&sim, 2, &["area-a:review", "cross:review", "drive-a:drive"]),
        "the hunts, run again, and nothing else"
    );
    let state = sim.state();
    assert_eq!(
        results(&state, 1)[5..],
        [
            json!(["area-a", 2, "green", null]),
            json!(["cross", 2, "green", null]),
            json!(["drive-a", 2, "green", null]),
        ],
        "{state}"
    );
    assert_eq!(
        json!([state["round"], state["next"], state["retest"]]),
        json!([1, "retest", ["no-lost-files"]]),
        "no round was begun, and the other clause's item is due next: {state}"
    );
    let last = sim.invoke(
        sim.args("test", json!({"clause": "no-lost-files"})),
        json!({"agents": {"row-a:source": nothing(), "row-a:driver": nothing(), "row-a:reconciler": nothing()}}),
    );
    assert_eq!(last.result["next"], "close", "{}", last.result);
    assert_eq!(
        last.result["forbids_close"],
        json!({"clauses": [], "no_clauses": false, "findings": 0, "candidate": false})
    );

    // A ROUND TESTS ONE CANDIDATE, AND A RE-RUN IS AN ATTEMPT INSIDE IT (the orchestrator's
    // ruling of 2026-10-07 on the core review's F3). By the second re-run four record
    // commits lie on the candidate — the round's record, the go, the first re-run's — so
    // the tip is another commit each time: and each re-run built the candidate from its
    // commit, drove that binary, and named the candidate in every result. `close` is
    // computed over greens that are all of the commit the round tested.
    let state = sim.state();
    let candidate = state["rounds"][0]["facts"]["candidate"]
        .as_str()
        .expect("the round's candidate")
        .to_owned();
    assert_ne!(
        sim.rig.rev("HEAD"),
        candidate,
        "record commits moved the tip"
    );
    let rows = state["rounds"][0]["results"]
        .as_array()
        .expect("the results");
    assert!(
        rows.len() == 9 && rows.iter().all(|row| row["commit"] == candidate.as_str()),
        "every result of the round, first run and re-run alike, names the round's candidate: {state}"
    );
    for (which, ran) in [("the first re-run", &again), ("the second", &last)] {
        let preflight = &ran
            .prompts
            .iter()
            .find(|(label, _)| label == "preflight")
            .expect("a re-run's preflight")
            .1;
        assert!(
            preflight.contains(&format!("commit {candidate} — the commit round 1 tested"))
                && ran.commands.iter().any(|command| command.contains(&format!(
                    " --kind build --commit {candidate} --to bin/candidate-"
                ))),
            "{which} builds the round's candidate, from its commit, by a held command: {:?}",
            ran.commands
        );
        assert_eq!(
            ran.result["candidate"]["sha"],
            candidate.as_str(),
            "{which}"
        );
        assert!(
            ran.prompts
                .iter()
                .filter(|(label, _)| label.contains(':')
                    && !label.starts_with("git:")
                    && !label.starts_with("record:"))
                .all(|(_, prompt)| prompt.contains(&format!("(commit {candidate}, label c1)"))),
            "{which}: every driving agent is handed the candidate's binary"
        );
    }
}

/// **A reporter the stage stands on.** The first preflight, the scope step and triage: with
/// none of them the stage cannot go on, so it halts — and THE ATTEMPT IS ON RECORD ALL THE
/// SAME, by the marker its first step wrote, though no agent of it wrote a report. Two such
/// attempts in a row, and the next one is the human's to grant. The second preflight is not
/// one of them: what it did not provide is void, and the stage goes on.
#[test]
fn an_attempt_whose_agents_all_died_is_counted_and_a_dead_second_preflight_voids_what_it_owed() {
    if !can_run() {
        return;
    }
    // (1) The preflight and the scope step both die: no agent of the attempt wrote anything.
    let sim = Sim::opened("dead-preflight", &one_area());
    let opened = sim.rig.rev("HEAD");
    let dead = json!({"endings": {"preflight": "dies", "scope": "dies"}});
    let ran = sim.invoke(sim.args("test", json!({})), dead.clone());
    assert_eq!(ran.result["status"], "halted", "{}", ran.result);
    assert_eq!(
        json!([
            ran.result["halted"]["phase"],
            ran.result["halted"]["transient"]
        ]),
        json!(["preflight", true])
    );
    assert_eq!(
        sim.reports(1),
        lines(&["attempt.a1.md"]),
        "the attempt's marker is the one file it left"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["rounds"][0]["test_attempt"],
            state["rounds"][0]["test_unrecorded"],
            state["position"]["test"],
            state["next"]
        ]),
        json!([1, 1, {"round": 1, "attempt": 2}, "test"]),
        "an attempt that left no agent's report is an attempt on record: {state}"
    );
    let second = sim.invoke(sim.args("test", json!({})), dead);
    assert_eq!(second.result["halted"]["phase"], "preflight");
    let state = sim.state();
    assert_eq!(
        json!([
            state["next"],
            state["human_stages"][0]["attempts"],
            state["position"]["test"]
        ]),
        json!(["rule", 2, {"refused": "attempts-spent", "round": 1}]),
        "and after two of them the stage is the human's: {state}"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");

    // (2) The scope step alone, and triage alone: a halt that names the step.
    let sim = Sim::opened("dead-scope", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"endings": {"scope": "dies"}}),
    );
    assert_eq!(ran.result["halted"]["phase"], "scope", "{}", ran.result);
    let sim = Sim::opened("dead-triage", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(),
               "agents": {"area-a:review": found(&[finding("a-1", "x", "jigc doc set", "audit-clean")]),
                          "triage:p1": nothing()},
               "endings": {"triage:p1": "dies"}}),
    );
    assert_eq!(
        json!([
            ran.result["halted"]["phase"],
            ran.result["halted"]["transient"]
        ]),
        json!(["triage", true]),
        "a finding nobody graded has no row: {}",
        ran.result
    );
    assert_eq!(sim.state()["rounds"][0]["test_unrecorded"], 1);
    // … and so does one of them that returned and left no report: what the preflight
    // established, and what triage graded, has nothing on record behind it.
    let sim = Sim::opened("unreported-preflight", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "endings": {"preflight": "unreported"}}),
    );
    assert_eq!(ran.result["halted"]["phase"], "reports", "{}", ran.result);
    assert!(
        ran.result["halted"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("returned and left no report: preflight")),
        "{}",
        ran.result
    );
    let sim = Sim::opened("unreported-triage", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(),
               "agents": {"area-a:review": found(&[finding("a-1", "x", "jigc doc set", "audit-clean")]),
                          "triage:p1": graded(&[entry("a-finding", "area-a-review a-1", "jigc doc set", "audit-clean", "no-break")], &["area-a-review"])},
               "endings": {"triage:p1": "unreported"}}),
    );
    assert_eq!(ran.result["halted"]["phase"], "triage", "{}", ran.result);
    assert!(
        ran.result["halted"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("returned its grades and left no report")),
        "{}",
        ran.result
    );
    // The lap that finishes a round's triage stands on its preflight in the same way.
    let sim = Sim::opened("unreported-lap-preflight", &one_area());
    let key = "doc-set-drops-a-slot";
    sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(),
               "agents": {"area-a:review": found(&[finding("a-1", "x", "jigc doc set", "audit-clean")]),
                          "triage:p1": graded(&[entry(key, "area-a-review a-1", "jigc doc set", "audit-clean", "breaks")], &["area-a-review"]),
                          format!("verify:{key}"): undriven(key)}}),
    );
    let lap = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {"triage:p1": graded(&[again(key, "jigc doc set", "audit-clean", "breaks")], &["ledger"]),
                          format!("verify:{key}"): undriven(key)},
               "endings": {"preflight": "unreported"}}),
    );
    assert_eq!(lap.result["halted"]["phase"], "reports", "{}", lap.result);
    // A file nobody launched is nobody's report: it stops the stage, as it did.
    let sim = Sim::opened("stray-report", &one_area());
    sim.rig.write(
        &format!("{RUN_DIR}/r1/reports/test/notes.txt"),
        "a note somebody left\n",
    );
    let stray = sim.rig.step(&[
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
        "attempt",
    ]);
    assert_eq!(
        stray.done("check-reports", "checked")["check"]["extra"],
        json!(["notes.txt"])
    );

    // (3) The second preflight — the trial image, and a check the scope selected — dies: the
    // trial arm is not launched and is void with the reason, so is the check, and every
    // other item runs. The stage reaches its record.
    let sim = Sim::opened(
        "dead-second-preflight",
        &Opening {
            clauses: vec!["adoptable", "audit-clean"],
            items: json!([
                in_scope("arm-a", "trial-arm", "adoptable", &[], &["verbs"]),
                {"item": "tarball", "kind": "check", "clause": "adoptable", "runs": "in-scope",
                 "brief": "the packaged-tarball install", "doors": ["jigc doc set"], "registries": []},
                item("area-a", "audit-area", "audit-clean"),
            ]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "endings": {"preflight-second": "dies"}}),
    );
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);
    let why = "the second preflight did not provide it — the preflight returned no result";
    assert_eq!(
        results(&sim.state(), 1),
        [
            json!(["area-a", 1, "green", null]),
            json!(["arm-a", 1, "void", format!("no trial image: {why}")]),
            json!(["tarball", 1, "void", why]),
        ]
    );
    assert!(
        !ran.agents().iter().any(|label| label.starts_with("arm-a:")),
        "no trial arm runs without its image: {:?}",
        ran.agents()
    );
    assert_eq!(sim.rig.status(), "");
}

/// **A verifier, an advocate or the independent drive that does not finish leaves its
/// finding unverified** — never a verdict nobody can read a report for, and never half a
/// fork. The record counts the pass; the invocation that finishes the round's triage
/// verifies the finding again and drives the fork again; and left so a second time the
/// finding is the human's.
#[test]
fn a_verifier_an_advocate_or_a_proposal_driver_that_does_not_finish_leaves_the_finding_unverified()
{
    if !can_run() {
        return;
    }
    let key = "doc-set-drops-a-slot";
    let stage = |more: Value| {
        let mut script = json!({
            "scope": one_door(),
            "agents": {
                "area-a:review": found(&[finding("a-1", "a slot is dropped", "jigc doc set", "audit-clean")]),
                "triage:p1": graded(
                    &[entry(key, "area-a-review a-1", "jigc doc set", "audit-clean", "breaks")],
                    &["area-a-review"],
                ),
            },
        });
        for (label, value) in more["agents"].as_object().expect("agents") {
            script["agents"][label] = value.clone();
        }
        script["endings"] = more["endings"].clone();
        script
    };
    let mut contested = verdict(key, "confirmed");
    contested["contested"] = json!(true);
    let advocate = json!({"status": "argued", "verdict": "robust-now", "case": "The robust case.",
                          "proposal": "A change.", "driven": [], "undriven": [], "left_open": []});
    let drive =
        json!({"status": "driven", "holds": false, "steps": [], "undriven": [], "left_open": []});

    // (1) THE ADVOCATE DIES. The verdict was returned, with its report — and it is not
    // recorded: a contested finding with no case is no fork, and without the fork it would
    // be a fixer's. The state holds no fork and no verdict.
    let sim = Sim::opened("dead-advocate", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        stage(json!({
            "agents": {format!("verify:{key}"): contested, format!("advocate:{key}"): advocate},
            "endings": {format!("advocate:{key}"): "dies"},
        })),
    );
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "M9: {result}");
    assert_eq!(
        json!([result["forks"], result["unverified"], result["next"]]),
        json!([[], [{"key": key, "why": "it is contested, and its advocate returned nothing"}], "triage"]),
        "{result}"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["ledger"][0]["grade"],
            state["ledger"][0]["triage"]["verdict"],
            state["ledger"][0]["triage"]["fork"],
            state["blockers"]
        ]),
        json!(["breaks", null, null, []]),
        "{state}"
    );
    assert!(
        sim.reports(1).contains(&format!("verify-p1-{key}.a1.md"))
            && !sim
                .reports(1)
                .iter()
                .any(|name| name.starts_with("advocate-")),
        "the verifier's report is on record, and the dead advocate is held to none: {:?}",
        sim.reports(1)
    );
    // THE FINISHING LAP DRIVES THE FORK AGAIN — and this time the independent drive writes
    // its report and dies: still no fork, still unverified, and now the human's.
    let lap = sim.invoke(
        sim.args("test", json!({})),
        json!({
            "agents": {
                "triage:p1": graded(&[again(key, "jigc doc set", "audit-clean", "breaks")], &["ledger"]),
                format!("verify:{key}"): contested,
                format!("advocate:{key}"): advocate,
                format!("proposal:{key}"): drive,
            },
            "endings": {format!("proposal:{key}"): "dies-after-report"},
        }),
    );
    assert_eq!(lap.result["status"], "triaged", "M9: {}", lap.result);
    assert_eq!(
        lap.result["unverified"],
        json!([{"key": key, "why": "it is contested, and the independent drive of the advocate's proposal returned nothing"}])
    );
    assert!(
        sim.reports(1).contains(&format!("proposal-p1-{key}.a2.md")),
        "a report written before its agent died is the stage's, and is committed: {:?}",
        sim.reports(1)
    );
    assert_eq!(sim.rig.status(), "");
    let state = sim.state();
    assert_eq!(
        json!([state["next"], state["human_list"]]),
        json!(["rule", [{"key": key, "why": "unverified-after-retry"}]]),
        "counted, bounded, and the human's: {state}"
    );
    // The third time the fork is whole: the advocate's case and a drive that DIFFERS.
    let granted = sim.invoke(
        sim.args("test", json!({"rulings": [{"reverify": key}]})),
        json!({}),
    );
    assert_eq!(granted.result["next"], "triage", "{}", granted.result);
    let whole = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            "triage:p1": graded(&[again(key, "jigc doc set", "audit-clean", "breaks")], &["ledger"]),
            format!("verify:{key}"): contested,
            format!("advocate:{key}"): advocate,
            format!("proposal:{key}"): drive,
        }}),
    );
    assert_eq!(whole.result["forks"].as_array().map(Vec::len), Some(1));
    let state = sim.state();
    assert_eq!(
        json!([state["ledger"][0]["triage"]["fork"], state["human_list"]]),
        json!([{"kind": "contested", "case": "robust-now", "drive": "differs"}, [{"key": key, "why": "fork"}]]),
        "{state}"
    );

    // … AND A CASE OR A DRIVE THAT WAS RETURNED WITH NO REPORT is none: what the human would
    // be brought has nothing on record behind it.
    for (label, who, why) in [
        (
            "unreported-case",
            "advocate",
            "it is contested, and its advocate left no report",
        ),
        (
            "unreported-drive",
            "proposal",
            "it is contested, and the independent drive of the advocate's proposal left no report",
        ),
    ] {
        let sim = Sim::opened(label, &one_area());
        let ran = sim.invoke(
            sim.args("test", json!({})),
            stage(json!({
                "agents": {format!("verify:{key}"): contested, format!("advocate:{key}"): advocate,
                           format!("proposal:{key}"): drive},
                "endings": {format!("{who}:{key}"): "unreported"},
            })),
        );
        assert_eq!(ran.result["status"], "triaged", "{label}: {}", ran.result);
        assert_eq!(
            json!([ran.result["forks"], ran.result["unverified"]]),
            json!([[], [{"key": key, "why": why}]]),
            "{label}"
        );
        assert_eq!(
            sim.state()["ledger"][0]["triage"]["fork"],
            Value::Null,
            "{label}"
        );
        assert_eq!(sim.rig.status(), "", "{label}");
    }

    // (2) THE VERIFIER: dead; a verdict with no report; a verdict for another finding. None
    // is a verdict on record.
    for (label, said, ending, why) in [
        (
            "dead-verifier",
            verdict(key, "confirmed"),
            Some("dies"),
            "its verifier returned nothing",
        ),
        (
            "unreported-verdict",
            verdict(key, "confirmed"),
            Some("unreported"),
            "its verifier returned a verdict and left no report",
        ),
        (
            "another-finding",
            verdict("some-other-finding", "refuted"),
            None,
            "its verifier returned a verdict for `some-other-finding`, another finding",
        ),
    ] {
        let sim = Sim::opened(label, &one_area());
        let ran = sim.invoke(
            sim.args("test", json!({})),
            stage(json!({
                "agents": {format!("verify:{key}"): said},
                "endings": ending.map_or(json!({}), |ending| json!({format!("verify:{key}"): ending})),
            })),
        );
        assert_eq!(ran.result["status"], "triaged", "{label}: {}", ran.result);
        assert_eq!(
            ran.result["unverified"],
            json!([{"key": key, "why": why}]),
            "{label}"
        );
        let state = sim.state();
        assert_eq!(
            json!([state["untriaged"], state["rounds"][0]["facts"]["awaiting"]]),
            json!([[{"key": key, "why": "unverified"}], [format!("{key} unverified 1")]]),
            "{label}: {state}"
        );
        assert_eq!(sim.rig.status(), "", "{label}");
    }
}

/// **A cross-model pass is limited and rare, chosen per item — and never fails the pass
/// that runs regardless** (the human's rulings of 2026-10-06; the harness review's `M5`).
/// Named for one item, it runs beside that item's own source pass. Whichever way it fails —
/// the tool does not answer, the agent dies, dies after its report, halts, or returns leads
/// with no report behind them — it is recorded as THAT PASS NOT HAVING RUN, and the item's
/// own passes run to their end: the item is green, and the stage reaches its record.
#[test]
fn a_cross_model_pass_that_fails_is_recorded_as_not_run_and_the_items_own_passes_go_on() {
    if !can_run() {
        return;
    }
    let opening = || Opening {
        clauses: vec!["no-lost-files"],
        items: json!([item("row-a", "review-row", "no-lost-files")]),
        bounds: vec![],
        rows: json!([]),
    };
    let lead = || {
        let mut lead = finding(
            "x-1",
            "a claim of another model",
            "jigc doc set",
            "no-lost-files",
        );
        lead["lead"] = json!(true);
        found(&[lead])
    };
    let halted = json!({"status": "halted", "findings": [],
                        "halt": {"root_cause": "the pass returned nothing usable", "evidence": "scripted",
                                 "tree_state": "clean", "recommendation": "none"}});
    let named = json!({"crossModel": ["row-a"]});
    // (tool, the pass's return, its ending, whether it ran, whether its report is there)
    for (label, tool, said, ending, runs, reports) in [
        ("cross-ran", "green", Some(nothing()), None, true, true),
        ("cross-no-tool", "void", None, None, false, false),
        (
            "cross-dies",
            "green",
            Some(nothing()),
            Some("dies"),
            false,
            false,
        ),
        (
            "cross-wrote-and-died",
            "green",
            Some(nothing()),
            Some("dies-after-report"),
            false,
            true,
        ),
        ("cross-halted", "green", Some(halted), None, false, true),
        (
            "cross-unreported",
            "green",
            Some(lead()),
            Some("unreported"),
            false,
            false,
        ),
    ] {
        let sim = Sim::opened(label, &opening());
        let mut script = json!({
            "checks": {"cross-model-tool": tool},
            "scope": one_door(),
            "agents": {"row-a:source": nothing(), "row-a:driver": nothing(), "row-a:reconciler": nothing()},
            "endings": {},
        });
        if let Some(said) = said {
            script["agents"]["row-a:crossmodel"] = said;
        }
        if let Some(ending) = ending {
            script["endings"]["row-a:crossmodel"] = json!(ending);
        }
        let ran = sim.invoke(sim.args("test", named.clone()), script);
        let result = &ran.result;
        assert_eq!(result["status"], "triaged", "M5, {label}: {result}");
        assert_eq!(
            json!([
                result["cross_model"]["named"],
                result["cross_model"]["ran"],
                result["cross_model"]["void"].as_array().map(Vec::len),
                result["counts"]["voided"]
            ]),
            json!([
                ["row-a"],
                if runs { json!(["row-a"]) } else { json!([]) },
                usize::from(!runs),
                []
            ]),
            "M5, {label}: the pass is recorded as it went, and the item is never void for it: {result}"
        );
        let state = sim.state();
        assert_eq!(
            json!([
                state["rounds"][0]["facts"]["cross-model"],
                state["rounds"][0]["facts"]["cross-model-void"]
            ]),
            if runs {
                json!([["row-a"], []])
            } else {
                json!([[], ["row-a"]])
            },
            "M5, {label}: {state}"
        );
        assert_eq!(
            results(&state, 1),
            [json!(["row-a", 1, "green", null])],
            "M5, {label}: the item's own passes ran to their end"
        );
        assert_eq!(
            sim.reports(1)
                .contains(&"row-a-crossmodel.a1.md".to_owned()),
            reports,
            "M5, {label}: {:?}",
            sim.reports(1)
        );
        assert_eq!(
            ran.agents().contains(&"row-a:crossmodel"),
            tool == "green",
            "{label}: the pass is launched only where its tool answers"
        );
        assert!(ran.agents().contains(&"row-a:reconciler"), "{label}");
        assert_eq!(
            state["ledger"],
            json!([]),
            "{label}: a lead with no report is no finding"
        );
        assert_eq!(sim.rig.status(), "", "{label}");
    }
    // AND WITH NO ITEM NAMED nothing asks about the tool, and nothing is launched for it.
    let sim = Sim::opened("cross-unnamed", &opening());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(),
               "agents": {"row-a:source": nothing(), "row-a:driver": nothing(), "row-a:reconciler": nothing()}}),
    );
    assert_eq!(
        ran.result["cross_model"],
        json!({"named": [], "ran": [], "void": []})
    );
    assert_eq!(sim.state()["rounds"][0]["facts"]["cross-model"], json!([]));
}

/// What a return is held to before anything is taken from it: one entry per key from
/// triage (`L9`), the previous release's binary wherever a verifier drove it (`L11`), the
/// binary a reviewer was handed (`M7`), and the commit a check ran on (`L1`).
#[test]
fn a_return_is_held_to_its_keys_its_binaries_and_its_commit_before_it_is_taken() {
    if !can_run() {
        return;
    }
    let key = "doc-set-drops-a-slot";
    let review = found(&[finding(
        "a-1",
        "a slot is dropped",
        "jigc doc set",
        "audit-clean",
    )]);
    let one = entry(
        key,
        "area-a-review a-1",
        "jigc doc set",
        "audit-clean",
        "breaks",
    );

    // L9 — two entries under one key: a halt that names the key, never a thrown error.
    let sim = Sim::opened("two-entries", &one_area());
    let mut twice = graded(&[one.clone(), one.clone()], &["area-a-review"]);
    twice["counts"]["merged"] = json!(-1);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": review, "triage:p1": twice}}),
    );
    assert_eq!(ran.result["status"], "halted", "L9: {}", ran.result);
    assert_eq!(ran.result["halted"]["phase"], "triage");
    assert!(
        ran.result["halted"]["reason"].as_str().is_some_and(
            |reason| reason.contains(&format!("more than one entry under the key {key}"))
        ),
        "L9: {}",
        ran.result
    );

    // L11 — a confirmed verdict that is NO regression drove the previous release's binary
    // too: another hash for it is a verifier that drove something else THERE. TURNED by the
    // second repair plan's `X1b` — it halted the stage: the verdict about the candidate
    // stands, on the binary the verifier asserted, and what it said of the previous release
    // is a fact nobody established — confirmed, its regression fact UNKNOWN, never `no`.
    let sim = Sim::opened("wrong-previous", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(),
               "agents": {"area-a:review": review, "triage:p1": graded(std::slice::from_ref(&one), &["area-a-review"]),
                          format!("verify:{key}"): verdict(key, "confirmed")},
               "wrongPrevious": [format!("verify:{key}")]}),
    );
    assert_eq!(ran.result["status"], "triaged", "L11: {}", ran.result);
    let state = sim.state();
    assert_eq!(
        json!([
            ran.result["unverified"],
            state["ledger"][0]["triage"]["verdict"],
            state["ledger"][0]["triage"]["regression"],
            state["regression_unknown"],
            ran.result["named"]["regression_unknown"]
        ]),
        json!([[], "confirmed", null, [key], 1]),
        "L11: the verifier said `regression: false` on another binary of the previous release — the record holds no `no`: {state}"
    );

    // M7 — a reviewer is handed the candidate's binary and returns the hash it asserted:
    // another hash voids its item, as a driver's does (turned by the second repair plan's
    // `X1`: it halted the stage).
    let sim = Sim::opened("reviewer-binary", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "wrongBinary": ["area-a:review"]}),
    );
    assert_eq!(
        json!([
            ran.result["status"],
            ran.result["counts"]["voided"],
            ran.result["unasserted"]
        ]),
        json!(["triaged", ["area-a"], [{"reporter": "area-a-review", "asserted": "0".repeat(64)}]]),
        "M7: the reviewer's hash is compared like every driver's: {}",
        ran.result
    );

    // L1 — the posture every driving agent relies on, that a bare `jigc` IS the binary, is
    // no longer a preflight's reading: it is a fact of the build, which the step tool
    // establishes and refuses a build without (`dev_stabilize_step`), and the harness holds
    // the tool's verdict to what it asked for. A commit that does not build halts the stage
    // (below: a held command that is red).
    // L1 — the commit a check ran on is read: green on another commit is void here.
    let sim = Sim::opened(
        "check-commit",
        &Opening {
            clauses: vec!["gate-green", "audit-clean"],
            items: json!([
                item("gate", "check", "gate-green"),
                item("area-a", "audit-area", "audit-clean")
            ]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    let other = "f".repeat(40);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"checks": {"gate": "green"}, "checkCommits": {"gate": other}, "scope": one_door(),
               "agents": {"area-a:review": nothing()}}),
    );
    assert_eq!(
        ran.result["counts"]["voided"],
        json!(["gate"]),
        "L1: {}",
        ran.result
    );
    assert_eq!(
        results(&sim.state(), 1)[1],
        json!([
            "gate",
            1,
            "void",
            format!("the check ran on the commit {other}, not on the candidate")
        ])
    );
}

// ---------------------------------------------------------------------------
// A return that lacks a field the stage needs of it (the second repair plan's `X1`)
// ---------------------------------------------------------------------------

/// The `(role, field)` pairs this suite drives with the field ABSENT: every cell of the
/// harness's `NEEDS` whose word is not `halts` — and none is. `stabilize_harness_fence`, arm (t), holds
/// this list to that table in both directions — so a field a stage comes to need of a
/// role has its arm here, or is named there as owed to a task.
pub(crate) const ABSENT_ARMS: &[(&str, &str)] = &[
    ("review", "asserted_sha256"),
    ("drive", "asserted_sha256"),
    ("verify", "verdict"),
    ("verify", "asserted_sha256"),
    ("advocate", "asserted_sha256"),
    ("proposal", "asserted_sha256"),
    // The two cells `X1` left halting, turned by `X1b`: the verdict stands, the fact is unknown.
    ("verify", "regression"),
    ("verify", "ran_on.previous"),
];

/// **A return that lacks a field the stage needs of that role is that item's fact, and
/// never the stage's** (the re-review's `R-H2`; the second repair plan's `X1`). No schema of
/// a stage requires such a field — a return that omits a required one is a call that
/// failed, three whole runs of its agent (the `required` probe) — so the harness is handed
/// the return without it. Per pair of [`ABSENT_ARMS`]: the field left out voids what that
/// reporter was launched for — an instrument's item, a finding's verification, a fork —
/// WITH A REASON THAT NAMES THE FIELD AND THE REPORTER; every other item is untouched; what
/// the reporter found is triaged all the same; and the stage reaches its record, with
/// `next` as the state computes it. Beside each, the control with the field there. AND A
/// HASH THAT CAME BACK WRONG IS THE SAME CELL as one that did not come back: what that one
/// agent drove is no evidence about the candidate, and nothing of the stage waits on it.
/// AND A CONFIRMED VERDICT WITHOUT ITS REGRESSION FACT — no `regression`, or no hash of the
/// previous release's binary behind it — IS NOT A VOID AT ALL (the plan's `X1b`): the
/// verdict stands with its fork, the record holds it as confirmed with the fact UNKNOWN,
/// never as *not a regression*, the finding is not counted unverified, and the state names
/// it — which the stage's return counts beside what no round selected.
///
/// The arms of `pairs`, and the control they stand beside: four tests share this, so
/// that no one test is a dozen stages long.
fn a_needed_field_left_out_voids_what_its_reporter_was_launched_for(
    pairs: &[(&str, &str)],
    fork: bool,
) {
    if !can_run() {
        return;
    }
    let key = "doc-set-drops-a-slot";
    let kept = "doc-set-exit-code";
    let two = || Opening {
        clauses: vec!["audit-clean"],
        items: json!([
            item("area-a", "audit-area", "audit-clean"),
            item("drive-a", "audit-drive", "audit-clean")
        ]),
        bounds: vec![],
        rows: json!([]),
    };
    let mut contested = verdict(key, "confirmed");
    contested["contested"] = json!(true);
    // What the scripted agents say, as far down the fork's road as `upto` goes: 0 — the
    // two instruments, and a finding graded no break; 1 — the finding graded a break, and
    // its verifier; 2 — the advocate of the contested verdict; 3 — the independent drive.
    let stage = |upto: u8| {
        let (grade, name) = if upto == 0 {
            ("no-break", kept)
        } else {
            ("breaks", key)
        };
        let mut script = json!({
            "scope": one_door(),
            "agents": {
                "area-a:review": found(&[finding("a-1", "a slot is dropped", "jigc doc set", "audit-clean")]),
                "drive-a:drive": nothing(),
                "triage:p1": graded(
                    &[entry(name, "area-a-review a-1", "jigc doc set", "audit-clean", grade)],
                    &["area-a-review"],
                ),
            },
        });
        if upto >= 1 {
            script["agents"][format!("verify:{key}")] = contested.clone();
        }
        if upto >= 2 {
            script["agents"][format!("advocate:{key}")] = json!({"status": "argued", "verdict": "robust-now",
                "case": "The robust case.", "proposal": "A change.", "driven": [], "undriven": [], "left_open": []});
        }
        if upto >= 3 {
            script["agents"][format!("proposal:{key}")] = json!({"status": "driven", "holds": true, "steps": [], "undriven": [], "left_open": []});
        }
        script
    };
    let zeros = "0".repeat(64);
    // (the pair's role, its field, the agent's label, the reporter's name, how far the road
    //  is scripted, what the reason opens with)
    let arms = [
        (
            "review",
            "asserted_sha256",
            "area-a:review".to_owned(),
            "area-a-review".to_owned(),
            0,
            "its `review` step",
        ),
        (
            "drive",
            "asserted_sha256",
            "drive-a:drive".to_owned(),
            "drive-a-drive".to_owned(),
            0,
            "its `drive` step",
        ),
        (
            "verify",
            "verdict",
            format!("verify:{key}"),
            format!("verify-p1-{key}"),
            1,
            "its verifier",
        ),
        (
            "verify",
            "asserted_sha256",
            format!("verify:{key}"),
            format!("verify-p1-{key}"),
            1,
            "its verifier",
        ),
        (
            "advocate",
            "asserted_sha256",
            format!("advocate:{key}"),
            format!("advocate-p1-{key}"),
            2,
            "it is contested, and its advocate",
        ),
        (
            "proposal",
            "asserted_sha256",
            format!("proposal:{key}"),
            format!("proposal-p1-{key}"),
            3,
            "it is contested, and the independent drive of the advocate's proposal",
        ),
        // The whole road of the fork is scripted: the verdict is TAKEN, so its fork is driven.
        (
            "verify",
            "regression",
            format!("verify:{key}"),
            format!("verify-p1-{key}"),
            3,
            "",
        ),
        (
            "verify",
            "ran_on.previous",
            format!("verify:{key}"),
            format!("verify-p1-{key}"),
            3,
            "",
        ),
    ];
    assert_eq!(
        arms.iter().map(|arm| (arm.0, arm.1)).collect::<Vec<_>>(),
        ABSENT_ARMS,
        "one arm per pair the fence holds this suite to"
    );
    assert!(
        !pairs.is_empty() && pairs.iter().all(|pair| ABSENT_ARMS.contains(pair)),
        "{pairs:?} are pairs the fence holds an arm for"
    );

    // EVERY ARM IS RUN AND EVERY ARM'S FAULT IS SAID — never the first one alone: an arm that
    // is red says what the stage did instead, by its pair.
    let mut faults: Vec<String> = Vec::new();
    let run = |label: &str, script: Value| -> Option<(Sim, Ran)> {
        let sim = Sim::opened(label, &two());
        let args = sim.args("test", json!({}));
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sim.invoke(args, script)))
            .ok()
            .map(|ran| (sim, ran))
    };
    let unsound =
        "the invocation is not one the simulation stands behind (its own message is above)";

    // THE CONTROL: every field there. Both items green, the finding a row, nothing owed
    // again — or, for the roles of a fork's road, a fork that is whole, on record with
    // the verdict it rides on.
    if fork {
        match run("needs-control-fork", stage(3)) {
            None => faults.push(format!("the control of the fork: {unsound}")),
            Some((sim, ran)) => {
                let state = sim.state();
                // … and its regression fact as the verifier returned it, backed by the
                // previous release's binary: `false` on record, and nothing named unknown.
                let seen = json!([
                    ran.result["status"],
                    ran.result["unverified"],
                    ran.result["forks"].as_array().map(Vec::len),
                    state["ledger"][0]["triage"]["verdict"],
                    state["ledger"][0]["triage"]["fork"],
                    state["ledger"][0]["triage"]["regression"],
                    state["regression_unknown"],
                    ran.result["named"]["regression_unknown"]
                ]);
                if seen
                    != json!(["triaged", [], 1, "confirmed", {"kind": "contested", "case": "robust-now", "drive": "holds"}, false, [], 0])
                {
                    faults.push(format!("the control of the fork: {seen}"));
                }
            }
        }
    } else {
        match run("needs-control-hunt", stage(0)) {
            None => faults.push(format!("the control of the instruments: {unsound}")),
            Some((sim, ran)) => {
                let state = sim.state();
                let seen = json!([
                    ran.result["status"],
                    ran.result["counts"]["voided"],
                    ran.result["unasserted"],
                    results(&state, 1),
                    state["retest"]
                ]);
                if seen
                    != json!([
                        "triaged",
                        [],
                        [],
                        [["area-a", 1, "green", null], ["drive-a", 1, "green", null]],
                        []
                    ])
                {
                    faults.push(format!("the control of the instruments: {seen}"));
                }
            }
        }
    }

    // THE REST OF A CHAIN WHOSE STEP DID NOT ASSERT THE BINARY IS NOT LAUNCHED — as after a
    // step that did not report: a reconciler handed the reports of a source pass that is
    // evidence about nothing would reconcile nothing. (The stand-in fails an invocation
    // that launches an agent nobody scripted: the reconciler is not scripted here.)
    if !fork {
        let sim = Sim::opened(
            "needs-chain",
            &Opening {
                clauses: vec!["no-lost-files"],
                items: json!([item("row-a", "review-row", "no-lost-files")]),
                bounds: vec![],
                rows: json!([]),
            },
        );
        let args = sim.args("test", json!({}));
        let script = json!({"scope": one_door(),
                            "agents": {"row-a:source": nothing(), "row-a:driver": nothing()},
                            "omits": {"row-a:source": ["asserted_sha256"]}});
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| sim.invoke(args, script))) {
            Err(_) => faults.push(format!("the rest of a chain: {unsound}")),
            Ok(ran) => {
                let rows = if ran.result["status"] == "triaged" {
                    results(&sim.state(), 1)
                } else {
                    vec![]
                };
                if !(rows.len() == 1
                    && rows[0][2] == "void"
                    && rows[0][3].as_str().is_some_and(|reason| {
                        reason.starts_with(
                            "its `source` step (row-a-source) returned no `asserted_sha256` — ",
                        )
                    })
                    && ran.agents().contains(&"row-a:driver")
                    && !ran.agents().contains(&"row-a:reconciler"))
                {
                    faults.push(format!(
                        "the rest of a chain: the item is not void by its `source` step with the reconciler not launched — {} · {rows:?} · {:?}",
                        ran.result["status"],
                        ran.agents()
                    ));
                }
            }
        }
    }

    for (role, field, label, reporter, upto, opens) in
        arms.iter().filter(|arm| pairs.contains(&(arm.0, arm.1)))
    {
        // The field ABSENT — and, where it is the hash, RETURNED AND WRONG: the same cell.
        // (A driver's and a reviewer's wrong hash are the two tests that halted on one.)
        let mut modes = vec!["absent"];
        if *field == "asserted_sha256" && *upto > 0 {
            modes.push("wrong");
        }
        for mode in modes {
            let at = format!("({role}, {field}) {mode}");
            let mut script = stage(*upto);
            if mode == "absent" {
                script["omits"] = json!({label.as_str(): [field]});
            } else {
                script["wrongBinary"] = json!([label]);
            }
            let Some((sim, ran)) = run(&format!("needs-{role}-{}-{mode}", &field[..4]), script)
            else {
                faults.push(format!("{at}: {unsound}"));
                continue;
            };
            let result = &ran.result;
            // THE STAGE REACHES ITS RECORD, committed and pushed, and returns what the state
            // computes — never a halt.
            if result["status"] != "triaged" {
                faults.push(format!(
                    "{at}: the stage did not reach its record — {} at `{}`: {}",
                    text(&result["status"]),
                    text(&result["halted"]["phase"]),
                    text(&result["halted"]["reason"])
                ));
                continue;
            }
            let state = sim.state();
            let mut hold = |holds: bool, what: String| {
                if !holds {
                    faults.push(format!("{at}: {what}"));
                }
            };
            hold(
                sim.rig.status().is_empty(),
                "the record is not committed whole".to_owned(),
            );
            hold(
                result["next"] == state["next"] && state["next"] != "close",
                format!(
                    "`next` is {} where the state says {}",
                    result["next"], state["next"]
                ),
            );
            if ["regression", "ran_on.previous"].contains(field) {
                // A CONFIRMED VERDICT WHOSE REGRESSION FACT NOBODY KNOWS: THE VERDICT STANDS.
                // The finding is confirmed and NOT unverified, its fork is on record with
                // it, the record's cell for the fact is empty — never `no` — the state
                // names the finding, and the return counts it.
                let seen = json!([
                    result["unverified"],
                    result["counts"]["voided"],
                    result["forks"].as_array().map(Vec::len),
                    state["ledger"][0]["triage"]["verdict"],
                    state["ledger"][0]["triage"]["regression"],
                    state["ledger"][0]["triage"]["fork"]["drive"],
                    state["untriaged"],
                    state["regression_unknown"],
                    result["named"]["regression_unknown"]
                ]);
                hold(
                    seen == json!([[], [], 1, "confirmed", null, "holds", [], [key], 1]),
                    format!(
                        "the finding is not on record as confirmed with its regression fact unknown, named by the state and counted by the return: {seen}"
                    ),
                );
                continue;
            }
            let said = if mode == "absent" {
                format!("{opens} ({reporter}) returned no `{field}` — ")
            } else {
                format!(
                    "{opens} ({reporter}) returned the `asserted_sha256` {zeros}, which is not the candidate's binary ("
                )
            };
            if *upto == 0 {
                // AN INSTRUMENT'S STEP: its item is void, with the reason; the other item
                // is green; and what the unasserted reporter found is a row all the same.
                let (void, other) = if *role == "review" {
                    ("area-a", "drive-a")
                } else {
                    ("drive-a", "area-a")
                };
                let rows = results(&state, 1);
                let of = |item: &str| {
                    rows.iter()
                        .find(|row| row[0] == item)
                        .cloned()
                        .unwrap_or(Value::Null)
                };
                hold(
                    result["counts"]["voided"] == json!([void])
                        && of(void)[2] == "void"
                        && of(void)[3]
                            .as_str()
                            .is_some_and(|reason| reason.starts_with(&said)),
                    format!(
                        "`{void}` is not void with a reason that names the field and the reporter: {rows:?}"
                    ),
                );
                hold(
                    json!([of(other)[2], of(other)[3]]) == json!(["green", null]),
                    format!("the other item is touched: {rows:?}"),
                );
                hold(
                    json!([
                        state["ledger"][0]["key"],
                        state["ledger"][0]["grade"],
                        result["counts"]["entries"]
                    ]) == json!([kept, "no-break", 1]),
                    format!(
                        "what the reporter found is not triaged all the same: {}",
                        state["ledger"]
                    ),
                );
                hold(
                    result["unasserted"] == json!([{"reporter": reporter, "asserted": null}]),
                    format!(
                        "the return does not name who did not assert the binary: {}",
                        result["unasserted"]
                    ),
                );
                // The void forbids closing: the state asks for the item again.
                hold(
                    json!([state["retest"], result["state"]["stop"]["then"]])
                        == json!([["audit-clean"], "retest"]),
                    format!(
                        "the state does not ask for the void item again: {}",
                        state["retest"]
                    ),
                );
            } else {
                // A VERIFIER, AN ADVOCATE, THE INDEPENDENT DRIVE: the finding stays
                // unverified — no verdict and no fork on record — both items are green, and
                // the round's triage is not finished.
                let left = result["unverified"].as_array().cloned().unwrap_or_default();
                hold(
                    left.len() == 1
                        && left[0]["key"] == key
                        && left[0]["why"]
                            .as_str()
                            .is_some_and(|why| why.starts_with(&said)),
                    format!(
                        "the finding is not unverified with a reason that names the field and the reporter: {}",
                        result["unverified"]
                    ),
                );
                hold(
                    json!([result["counts"]["voided"], result["forks"]]) == json!([[], []]),
                    format!(
                        "an item is void, or a fork is returned: {} · {}",
                        result["counts"]["voided"], result["forks"]
                    ),
                );
                let seen = json!([
                    state["next"],
                    state["untriaged"],
                    state["ledger"][0]["triage"]["verdict"],
                    state["ledger"][0]["triage"]["fork"],
                    state["blockers"]
                ]);
                hold(
                    seen == json!(["triage", [{"key": key, "why": "unverified"}], null, null, []]),
                    format!(
                        "the record holds a verdict or a fork, or the triage reads as finished: {seen}"
                    ),
                );
            }
        }
    }
    assert!(
        faults.is_empty(),
        "what a stage did with a return that lacks a field it needs — one line per arm that is red:\n{}",
        faults.join("\n")
    );
}

#[test]
fn a_reviewer_or_a_driver_that_returns_no_hash_voids_its_item_and_the_stage_records() {
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(&ABSENT_ARMS[..2], false);
}

#[test]
fn a_verifier_that_returns_no_verdict_or_no_hash_leaves_the_finding_unverified_and_the_stage_records()
 {
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(&ABSENT_ARMS[2..4], true);
}

#[test]
fn an_advocate_or_an_independent_drive_that_returns_no_hash_is_no_fork_and_the_stage_records() {
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(&ABSENT_ARMS[4..6], true);
}

#[test]
fn a_confirmed_verdict_without_its_regression_fact_is_recorded_as_confirmed_with_the_fact_unknown()
{
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(&ABSENT_ARMS[6..], true);
}

// ---------------------------------------------------------------------------
// What a stage refuses, and what it halts on, before it launches an instrument
// ---------------------------------------------------------------------------

#[test]
fn a_stage_refuses_or_halts_before_anything_is_tested() {
    if !can_run() {
        return;
    }
    // The opening record is committed, and nothing else of the opening is.
    let sim = Sim::new("refusals");
    sim.rig
        .write(&format!("{RUN_DIR}/opening.md"), "# the opening record\n");
    sim.rig.commit("docs(record): the run is opened");
    sim.rig.git(&["push", "-q", "origin", LOOP]);
    let head = sim.rig.rev("HEAD");

    // THE `fix` STAGE IS NOT FIT FOR USE: refused before any agent runs — no git step, no
    // read. That is all this suite asserts of it.
    let fix = sim.invoke(sim.args("fix", json!({})), json!({}));
    assert_eq!(fix.result["status"], "refused", "{}", fix.result);
    assert_eq!(fix.result["not_fit"]["stage"], "fix");
    assert_eq!(fix.trace, Vec::<String>::new(), "no agent, and no phase");

    // AN OPENING THAT IS NOT DONE: refused by the position, in one word, after two reads.
    let early = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(early.result["status"], "refused", "{}", early.result);
    assert_eq!(
        early.result["refused"],
        json!({"refused": "not-ready", "round": null})
    );
    assert_eq!(
        early.result["not_ready"],
        json!([
            "no-clause-row",
            "no-stop-mode",
            "no-previous-release",
            "no-default-scope"
        ])
    );
    assert_eq!(early.trace, lines(&TWO_READS));

    // A TREE THAT IS NOT CLEAN: the step tool's refusal is the stage's halt, and the
    // record is not read.
    sim.rig.write("notes.txt", "x\n");
    let dirty = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(dirty.result["status"], "halted", "{}", dirty.result);
    assert_eq!(dirty.result["halted"]["phase"], "git");
    // THE REFUSAL IS ITS WORD (K3): the halt says whose the state is in the harness's own
    // sentence, and names the file the tool's halt report is in — which names the stray file.
    assert_eq!(
        json!([
            dirty.result["halted"]["refused"],
            dirty.result["halted"]["whose"],
            dirty.result["halted"]["halt"]
        ]),
        json!(["dirty", "the human's", null]),
        "{}",
        dirty.result
    );
    assert!(
        refused_line(&dirty.result)["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("dirty: ") && cause.contains("notes.txt")),
        "{}",
        dirty.result
    );
    assert!(
        !dirty.result.to_string().contains("notes.txt"),
        "no prose of the tool's refusal passes through the harness: {}",
        dirty.result
    );
    assert_eq!(dirty.trace, lines(&TWO_READS[..2]));
    fs::remove_file(sim.rig.root.join("notes.txt")).expect("remove the stray file");

    // ANOTHER BRANCH CHECKED OUT: the same.
    sim.rig.git(&["switch", "-q", "main"]);
    let elsewhere = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(
        elsewhere.result["halted"]["phase"], "git",
        "{}",
        elsewhere.result
    );
    assert_eq!(
        json!([
            elsewhere.result["halted"]["refused"],
            elsewhere.result["halted"]["whose"]
        ]),
        json!(["wrong-branch", "the orchestrator's"]),
        "{}",
        elsewhere.result
    );
    assert!(
        refused_line(&elsewhere.result)["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("wrong-branch: ")),
        "{}",
        elsewhere.result
    );
    sim.rig.git(&["switch", "-q", LOOP]);

    assert_eq!(sim.rig.rev("HEAD"), head, "no refusal committed anything");
    assert_eq!(sim.rig.status(), "", "or left anything behind");
}

#[test]
fn a_run_that_was_never_opened_halts_the_stage_at_its_first_read() {
    if !can_run() {
        return;
    }
    let sim = Sim::new("never-opened");
    sim.rig.git(&["push", "-q", "origin", LOOP]);
    let ran = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(ran.result["status"], "halted", "{}", ran.result);
    assert_eq!(ran.result["halted"]["phase"], "state");
    assert!(
        ran.result["halted"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("has no opening record")),
        "{}",
        ran.result
    );
    assert_eq!(ran.trace, lines(&TWO_READS));
}

// ---------------------------------------------------------------------------
// What the harness reads: a digest, and nothing else (the second repair plan's K3)
// ---------------------------------------------------------------------------

/// A text as a real record is written — a dash, a typographic quote, an arrow — under a
/// name a test can look for: what no relayed line carries.
fn marked(what: &str) -> String {
    format!("MARKER-{what} \u{2014} it\u{2019}s \u{2192} here")
}

/// The labels a state read is asked for under: the read, and the read again, twice.
fn state_reads(tag: &str) -> [String; 3] {
    [
        format!("git:state:{tag}"),
        format!("git:state:{tag}:again1"),
        format!("git:state:{tag}:again2"),
    ]
}

/// **A whole stage records over a record no relay would carry — and no text of that record
/// is in any prompt or in the return** (the second repair plan's `K3`; the re-review's
/// `R-H1` as the relay probe observed it on 2026-10-07: a line relayed through an agent
/// comes back altered once it holds one character outside ASCII, and the state document
/// holds them wherever a human wrote). THE STAND-IN RELAYS AS THE RUNTIME DOES — it decodes
/// a line's escapes — so the harness as it was read no state here. The run is opened with a
/// brief, sixty ledger rows and — by its scope step — a door's derivation, each written
/// with a dash, a quote and an arrow and each under a marker: the stage reaches its pushed
/// record; every agent that needs one of them READS IT BY KEY, with the command its prompt
/// spells, and gets it whole; and the markers are in no prompt and in no return. The return
/// names the file the state document lies in, and is a fraction of it.
#[test]
fn a_whole_stage_records_over_a_record_no_relay_carries_and_none_of_its_text_is_in_a_prompt_or_a_return()
 {
    if !can_run() {
        return;
    }
    let seeded: Vec<Value> = (1..=60)
        .map(|n| {
            json!({"key": format!("seeded-{n:02}"), "doctype": "jigc-feedback", "round": 0,
                   "source": "known at the opening", "door": "jigc doc set", "clause": "audit-clean",
                   "repro": format!("{} ({n})", marked("REPRO"))})
        })
        .collect();
    let opening = Opening {
        clauses: vec!["audit-clean"],
        items: json!([
            {"item": "area-a", "kind": "audit-area", "clause": "audit-clean", "runs": "every-candidate",
             "brief": marked("BRIEF")},
            // A door mistyped by one letter: the item is in scope, and no round selects it.
            in_scope("row-typo", "review-row", "audit-clean", &["jigc doc sett"], &[]),
        ]),
        bounds: vec![],
        rows: json!(seeded),
    };
    let sim = Sim::opened("no-text", &opening);
    let entries: Vec<Value> = (1..=60)
        .map(|n| {
            again(
                &format!("seeded-{n:02}"),
                "jigc doc set",
                "audit-clean",
                "no-break",
            )
        })
        .collect();
    let script = json!({
        "scope": {"included": [{"door": "jigc doc set", "registry": "verbs", "derivation": marked("DERIVATION")}],
                  "excluded": [], "uncovered": [], "reached_but_excluded": [], "left_open": []},
        "agents": {
            "area-a:review": nothing(),
            "triage:p1": {"status": "graded", "entries": entries, "to_verify": [],
                          "counts": {"findings_in": [{"reporter": "ledger", "count": 60}], "entries": 60, "merged": 0}},
        },
    });
    let ran = sim.invoke(sim.args("test", json!({})), script);
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");
    assert_eq!(sim.rig.status(), "");

    // NO TEXT OF THE RECORD PASSED THROUGH THE HARNESS.
    let returned = result.to_string();
    for what in ["BRIEF", "DERIVATION", "REPRO"] {
        let marker = format!("MARKER-{what}");
        for (label, prompt) in &ran.prompts {
            assert!(
                !prompt.contains(&marker),
                "the prompt of `{label}` holds text of the record ({marker}): {prompt}"
            );
        }
        assert!(
            !returned.contains(&marker),
            "the return holds text of the record ({marker}): {returned}"
        );
    }
    // … AND EVERY AGENT THAT NEEDS IT READ IT BY KEY, WHOLE, with the command its prompt
    // spells — the dash, the quote and the arrow included.
    let read = |label: &str, command: &str| -> Value {
        let printed = &ran
            .reads
            .iter()
            .find(|(by, asked, _)| by == label && asked == command)
            .unwrap_or_else(|| panic!("`{label}` read `{command}`: {:?}", ran.reads))
            .2;
        serde_json::from_str(printed).expect("a read prints one line of JSON")
    };
    assert_eq!(
        read(
            "area-a:review",
            &format!("{RECORD} item --run {RUN} --item area-a")
        )["item"]["brief"],
        marked("BRIEF").as_str()
    );
    assert_eq!(
        read(
            "area-a:review",
            &format!("{RECORD} item-doors --run {RUN} --round 1 --item area-a")
        )["doors"],
        json!([{"door": "jigc doc set", "registry": "verbs", "derivation": marked("DERIVATION")}])
    );
    let rows = read("triage:p1", &format!("{RECORD} untriaged --run {RUN}"));
    assert_eq!(rows["count"], 60, "{rows}");
    assert_eq!(
        rows["untriaged"][59]["repro"],
        format!("{} (60)", marked("REPRO")).as_str()
    );
    let triage = &ran
        .prompts
        .iter()
        .find(|(label, _)| label == "triage:p1")
        .expect("triage")
        .1;
    assert!(
        triage.contains("Grade every finding below — 60 in all, from 1 reporter(s).")
            && triage.contains(
                "— 60 finding(s): the rows `dev/stabilize-record untriaged --run rc24` prints"
            ),
        "triage is handed the read and the count it is held to: {triage}"
    );

    // THE RETURN NAMES THE FILE AND HOLDS NO DOCUMENT (the plan's X2): a fraction of it.
    let read_back = document(result);
    assert_eq!(read_back["ledger"].as_array().map(Vec::len), Some(60));
    let file = result["named"]["document"]["file"]
        .as_str()
        .expect("the file");
    let whole = fs::metadata(file).expect("the state document").len() as usize;
    assert!(
        returned.len() < 6000 && whole > 5 * returned.len(),
        "the return is {} bytes, over a document of {whole}: {returned}",
        returned.len()
    );
    assert_eq!(
        json!([
            result["next"],
            result["returned_to_orchestrator"],
            result["state"]["ledger"]
        ]),
        json!(["stop", true, 60]),
        "a `next` the script has no sentence for goes back with what the digest holds, and the file: {result}"
    );

    // WHAT NOTHING CAN HOLD IS NAMED IN THE RETURN (K4, through K3): the in-scope item no
    // tested round selected — its door is mistyped — and how many doors of the round no
    // item names; the file names the door.
    assert_eq!(
        result["named"]["never_selected"],
        json!(["row-typo"]),
        "{result}"
    );
    assert_eq!(
        result["named"]["never_selected"],
        read_back["never_selected"]
    );
    assert_eq!(
        result["named"]["uncovered"],
        json!({"round": 1, "doors": read_back["uncovered"].as_array().map(Vec::len)}),
        "{result}"
    );
    // … at every later return too: an invocation that is refused, and one that only looks.
    let refused = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(
        json!([
            refused.result["status"],
            refused.result["named"]["never_selected"]
        ]),
        json!(["refused", ["row-typo"]]),
        "{}",
        refused.result
    );
}

/// **A step that hands back anything but the digest its command printed is caught as
/// altered** — so that what the relay probe found cannot come back silently. Three things
/// a step's agent can hand back in its place, each for every try the harness makes of the
/// state read: THE LINE THE TOOL PRINTS UNASKED — the state document in it, which is what a
/// stage read until `K3` and which holds to its own hash; the digest with ONE CHARACTER
/// CHANGED; and the digest with a character OUTSIDE ASCII, its hash made to fit, so that
/// only the harness's own look at the line can tell. Each halts the stage at the read, says
/// why in a sentence of its own, and passes on nothing of what came back.
#[test]
fn a_step_that_hands_back_anything_but_its_digest_is_caught_as_altered() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("altered", &one_area());
    let head = sim.rig.rev("HEAD");
    for (how, says) in [
        ("document", "is no digest of `dev/stabilize-step state`"),
        ("byte", "does not end with the sha256 of itself"),
        (
            "non-ascii",
            "a backslash or a character outside printable ASCII",
        ),
    ] {
        let relays: serde_json::Map<String, Value> = state_reads("test-1")
            .into_iter()
            .map(|label| (label, json!(how)))
            .collect();
        let ran = sim.invoke(sim.args("test", json!({})), json!({"relays": relays}));
        let result = &ran.result;
        assert_eq!(
            json!([result["status"], result["halted"]["phase"]]),
            json!(["halted", "state"]),
            "{how}: {result}"
        );
        assert!(
            result["halted"]["reason"]
                .as_str()
                .is_some_and(|why| why.contains(says)),
            "{how}: {result}"
        );
        assert_eq!(
            ran.agents(),
            [
                "git:state",
                "git:state:test-1",
                "git:state:test-1:again1",
                "git:state:test-1:again2"
            ],
            "{how}: asked for again twice, and then there is no state"
        );
        assert!(
            !result.to_string().contains("the brief of area-a")
                && !result.to_string().contains("\u{2014}\"")
                && result.to_string().len() < 4000,
            "{how}: nothing of what came back is passed on: {result}"
        );
    }
    // Handed back once, and then the digest: the read is asked for again, and the stage reads.
    let once = sim.invoke(
        sim.args("test", json!({"stopAfter": "state"})),
        json!({"relays": {"git:state:test-1": "document"}}),
    );
    assert_eq!(once.result["status"], "stopped", "{}", once.result);
    assert_eq!(
        once.agents(),
        ["git:state", "git:state:test-1", "git:state:test-1:again1"]
    );
    // And the first read of all — the git state — is held the same way.
    let first = sim.invoke(
        sim.args("test", json!({})),
        json!({"relays": {"git:state": "non-ascii"}}),
    );
    assert_eq!(
        json!([first.result["status"], first.result["halted"]["phase"]]),
        json!(["halted", "git"]),
        "{}",
        first.result
    );
    assert!(
        first.result["halted"]["reason"]
            .as_str()
            .is_some_and(|why| why.contains("outside printable ASCII")),
        "{}",
        first.result
    );
    assert_eq!(first.agents(), ["git:state"]);
    assert_eq!(sim.rig.rev("HEAD"), head);
    assert_eq!(sim.rig.status(), "");
}

/// **An invocation that only looks finishes nothing and says what is owed; the one after it
/// finishes it** (the second repair plan's `K2` through `K3`; the re-review's `R-L1` and
/// `R-M1`). `stopAfter: 'state'` over a batch a record step applied and did not commit,
/// and over a record that was committed and not pushed: neither head moves, no agent but
/// the first read is launched, and the return names what an invocation without the stop
/// would finish. And A PUSH THAT FAILED AFTER THE RECORD IS MADE BY THE NEXT INVOCATION'S
/// FIRST READ — the halt says so, and it is true.
#[test]
fn an_invocation_that_only_looks_finishes_nothing_and_a_push_that_failed_is_made_by_the_next() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("owed", &one_area());
    let opened = sim.rig.rev("HEAD");
    // (1) A pending batch: the record's gate is red only with the records.
    let halted = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"record": {"gate": red(&[FLAKY])}})),
    );
    assert_eq!(
        halted.result["halted"]["refused"], "gate-red",
        "{}",
        halted.result
    );
    let status = sim.rig.status();
    let looked = sim.invoke(sim.args("test", json!({"stopAfter": "state"})), json!({}));
    let result = &looked.result;
    assert_eq!(
        json!([result["status"], result["after"], result["owed"]]),
        json!(["stopped", "state", ["applied"]]),
        "{result}"
    );
    assert_eq!(
        looked.agents(),
        ["git:state"],
        "one read, and it only looked"
    );
    assert!(
        looked.commands.len() == 1 && looked.commands[0].contains(" --look "),
        "{:?}",
        looked.commands
    );
    assert_eq!(
        json!([
            result["pending"]["round"],
            result["pending"]["calls"],
            result["pending"].get("subject")
        ]),
        json!([1, 4, null]),
        "the batch by its counts, and no text of it: {result}"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert_eq!(sim.rig.status(), status, "and nothing changed in the tree");
    assert!(sim.record(&["pending", "--run", RUN], "")["batch"].is_object());

    // (2) The batch is committed, and its push is rejected: the push is owed.
    sim.rig.remote_hook("pre-receive", "exit 1");
    let unpushed = sim.invoke(sim.args("test", json!({})), json!({}));
    let result = &unpushed.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["refused"]
        ]),
        json!(["halted", "push", "push-rejected"]),
        "{result}"
    );
    let message = result["message"].as_str().expect("a message");
    assert!(
        message.contains("THE PUSH IS OWED: invoke the stage again with the same args — its first read finds the record the remote lacks, vets it and pushes it"),
        "{message}"
    );
    let recorded = sim.rig.rev("HEAD");
    assert_eq!(sim.rig.rev("HEAD~1"), opened, "the record is committed");
    assert_eq!(sim.rig.remote(LOOP), Some(opened.clone()), "and not pushed");
    // … which an invocation that only looks says, and does not make.
    fs::remove_file(sim.rig.origin.join("hooks/pre-receive")).expect("the remote answers again");
    let looked = sim.invoke(sim.args("test", json!({"stopAfter": "state"})), json!({}));
    assert_eq!(
        json!([
            looked.result["status"],
            looked.result["owed"],
            looked.result["pending"]
        ]),
        json!(["stopped", ["unpushed"], null]),
        "{}",
        looked.result
    );
    assert_eq!(looked.agents(), ["git:state"]);
    assert_eq!(sim.rig.remote(LOOP), Some(opened), "a look pushes nothing");
    assert_eq!(sim.rig.rev("HEAD"), recorded);

    // (3) THE SAME ARGS AGAIN: the first read makes the push that is owed, and the stage
    // starts from the state as it then stands — here a round that is over.
    let next = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(recorded.clone()),
        "the owed push is made"
    );
    assert_eq!(
        json!([
            next.result["status"],
            next.result["refused"]["refused"],
            next.result["arrived"]["found"],
            next.result["arrived"]["finished"]
        ]),
        json!(["refused", "stopped", ["unpushed"], ["unpushed"]]),
        "the return says what the first read met and finished: {}",
        next.result
    );
    assert_eq!(next.agents(), ["git:state", "git:state:test-1"]);
    assert_eq!(sim.rig.rev("HEAD"), recorded);
    assert_eq!(sim.rig.status(), "");
}

/// **A commit that failed, and a commit step whose agent died, are finished by asking
/// again** (the re-review's `R3` and `R-L5`, through `K3`). A `git commit` that fails
/// leaves the record's paths staged: the stage halts, and the next invocation finds the
/// batch, gates it and commits it. And a commit step whose agent ran its command and then
/// returned nothing is asked for again as THE ONE COMMAND, RUN AGAIN — the stand-in fails
/// the run on a retry that tells a step's agent to look around the tree — and the step,
/// handed the commit the stage began on, answers what it answered: one commit, recorded.
#[test]
fn a_commit_that_failed_and_a_commit_step_whose_agent_died_are_finished_by_asking_again() {
    if !can_run() {
        return;
    }
    // (1) The commit fails.
    let sim = Sim::opened("failed-commit", &one_area());
    let opened = sim.rig.rev("HEAD");
    let marker = failing_commit(&sim.rig);
    let failed = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(
        json!([
            failed.result["status"],
            failed.result["halted"]["phase"],
            failed.result["halted"]["refused"]
        ]),
        json!(["halted", "record", "git"]),
        "{}",
        failed.result
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "the commit failed");
    fs::remove_file(&marker).expect("commits work again");
    let again = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(again.result["status"], "recorded", "{}", again.result);
    assert_eq!(
        again.agents(),
        [
            "git:state",
            "git:hold-start:record-pending-1",
            "git:hold-wait:record-pending-1",
            "git:record:pending",
            "git:push:pending",
            "git:state:test-1"
        ],
        "no executor: the batch is applied, and what is left of the step is its gate and its commit"
    );
    assert_eq!(
        again.result["arrived"]["owed"],
        json!(["staged"]),
        "the first read named what the failed commit left: {}",
        again.result
    );
    assert_eq!(sim.rig.rev("HEAD~1"), opened, "one record commit");
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");
    assert_eq!(sim.rig.status(), "");

    // (2) The commit step's agent dies after its command ran.
    let sim = Sim::opened("dead-commit-step", &one_area());
    let opened = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"endings": {"git:record:test:r1": "acts-and-dies-once"}})),
    );
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);
    let asked: Vec<&String> = ran
        .commands
        .iter()
        .filter(|ran| ran.starts_with("dev/stabilize-step record "))
        .collect();
    assert_eq!(asked.len(), 2, "the one command, run again: {asked:?}");
    assert_eq!(asked[0], asked[1]);
    assert!(
        asked[0].contains(&format!(" --head {opened} ")),
        "held to the commit the stage began on: {}",
        asked[0]
    );
    let retry = &ran
        .prompts
        .iter()
        .filter(|(label, _)| label == "git:record:test:r1")
        .nth(1)
        .expect("the commit step was asked for twice")
        .1;
    assert!(
        retry.contains("RETRY: an earlier try of this same step returned nothing. Run the ONE command above again")
            && !retry.split("\n\nRETRY").nth(1).is_some_and(|told| told.contains("git status") || told.contains("git log")),
        "{retry}"
    );
    assert_eq!(
        sim.rig.rev("HEAD~1"),
        opened,
        "ONE record commit, though the step ran twice"
    );
    assert_eq!(ran.result["record"], sim.rig.rev("HEAD").as_str());
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");
}

/// **The same ruling, sent twice, is refused and leaves no batch** (the re-review's `R2`,
/// the way in without a kill; `no-change`). The second invocation composes the same batch,
/// the record script refuses it as one that changes nothing, the executor returns that
/// word — and the invocation answers `refused`: nothing written, nothing pending, no
/// discard asked for.
#[test]
fn the_same_ruling_sent_twice_is_refused_and_leaves_no_batch() {
    if !can_run() {
        return;
    }
    let opening = Opening {
        clauses: vec!["audit-clean"],
        items: json!([item("area-a", "audit-area", "audit-clean")]),
        bounds: vec![],
        rows: json!([{"key": "known-one", "doctype": "jigc-feedback", "round": 0,
                      "source": "known at the opening", "door": "jigc setup", "clause": "audit-clean",
                      "repro": "the opening record"}]),
    };
    let sim = Sim::opened("twice", &opening);
    let rulings = json!({"rulings": [{"key": "known-one", "ruling": "later", "note": "for the next release"}]});
    let first = sim.invoke(sim.args("test", rulings.clone()), json!({}));
    assert_eq!(first.result["status"], "ruled", "{}", first.result);
    let ruled = sim.rig.rev("HEAD");
    let second = sim.invoke(sim.args("test", rulings), json!({}));
    let result = &second.result;
    assert_eq!(
        json!([result["status"], result["refused"]["refused"]]),
        json!(["refused", "no-change"]),
        "{result}"
    );
    assert_eq!(
        second.agents(),
        ["git:state", "git:state:test-1", "record:rulings:r0"],
        "the batch is refused where it is applied: no commit step, no push"
    );
    let message = result["message"].as_str().expect("a message");
    assert!(
        message.contains("NO BATCH IS LEFT")
            && message.contains("No discard is asked for")
            && !message.contains("discard --branch"),
        "{message}"
    );
    assert_eq!(
        sim.record(&["pending", "--run", RUN], "")["batch"],
        Value::Null
    );
    assert_eq!(sim.rig.rev("HEAD"), ruled);
    assert_eq!(sim.rig.status(), "");
}

/// **What would not have been written has its exit at the commit and at the push** (the
/// second repair plan's `K1`, through `K3`; at the report check it is
/// [`a_report_overwritten_by_hand_voids_its_item_and_reaches_neither_the_commit_nor_the_remote`]).
/// AT THE COMMIT: a report changed in the minutes between its check and the commit is
/// refused there, the batch IS TAKEN BACK ALREADY and the file left the tree — so the halt
/// asks for no discard, and the stage invoked again works the next attempt. AT A PUSH: a
/// commit somebody made by hand under the run's directory, its subject one the scanner
/// refuses, is never pushed — not by the first read that finds it owed either — and it is
/// the human's.
#[test]
fn what_would_not_have_been_written_has_its_exit_at_the_commit_and_at_the_push() {
    if !can_run() {
        return;
    }
    // (1) At the commit.
    let sim = Sim::opened("unvetted-commit", &one_area());
    let opened = sim.rig.rev("HEAD");
    let report = format!("{RUN_DIR}/r1/reports/test/area-a-review.a1.md");
    let spoiled = format!(
        "printf '%s\\n' '# a review' '' 'It ran in {}/probe.' '' '<!-- end of report -->' > {report}",
        sim.rig.dir().join("home").display()
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"meanwhile": {"record:test:r1": spoiled}})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["refused"]
        ]),
        json!(["halted", "record", "unvetted"]),
        "{result}"
    );
    let message = result["message"].as_str().expect("a message");
    assert!(
        message.contains("THE BATCH IS TAKEN BACK ALREADY")
            && message.contains("it reads the state as it stands and works the next attempt")
            && !message.contains("discard --branch")
            && !message.contains("APPLIED AND NOT COMMITTED"),
        "{message}"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert_eq!(
        sim.record(&["pending", "--run", RUN], "")["batch"],
        Value::Null
    );
    let status = sim.rig.status();
    assert!(
        !status.contains(" M ")
            && !status.contains("area-a-review")
            && !status.lines().any(|line| !line.starts_with("?? ")),
        "the tables are as they were, nothing is staged, and the file left the tree:\n{status}"
    );
    let again = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert_eq!(
        again.result["scope"]["status"], "stands",
        "the next attempt"
    );
    assert_eq!(sim.rig.rev("HEAD~1"), opened);
    assert_eq!(sim.rig.status(), "");

    // (2) At a push: a commit by hand, under the run's directory, that the scanner refuses.
    let sim = Sim::opened("unvetted-push", &one_area());
    let opened = sim.rig.rev("HEAD");
    sim.rig.change(
        &format!("{RUN_DIR}/notes.md"),
        "a note\n",
        &format!("docs(record): a note that met {DENY_TERM}"),
    );
    let by_hand = sim.rig.rev("HEAD");
    for _ in 0..2 {
        let ran = sim.invoke(sim.args("test", json!({})), json!({}));
        let result = &ran.result;
        assert_eq!(
            json!([
                result["status"],
                result["halted"]["phase"],
                result["halted"]["refused"],
                result["halted"]["whose"]
            ]),
            json!([
                "halted",
                "git",
                "unvetted",
                "the human's at a push; at a record's commit, the next attempt's"
            ]),
            "{result}"
        );
        assert_eq!(ran.agents(), ["git:state"]);
        assert!(
            !result.to_string().contains(DENY_TERM),
            "the halt names the file that names the place, never the text: {result}"
        );
        assert_eq!(
            sim.rig.remote(LOOP),
            Some(opened.clone()),
            "it is pushed by no invocation, however often the stage is invoked"
        );
    }
    assert_eq!(sim.rig.rev("HEAD"), by_hand);
}

/// **A lock, a commit that is no record's and a head that moved are each named as
/// somebody's, with what leaves the state** (`locked`, `foreign-commit`, `head-moved`: the
/// second repair plan's `K2`, through `K3`). git's own lock is the orchestrator's, and is
/// never removed by a step. Somebody commits a tuning change while the stage runs: the
/// record's commit step is held to the commit the stage began on and refuses — the batch
/// stays applied. The next invocation's first read meets that commit: it is ITS AUTHOR'S to
/// push, by the command the halt spells — taken out of the message here, and run — and
/// neither the human's nor the tool's. Then the stage, invoked once more, gates the batch
/// on the tree as it now stands and records it.
#[test]
fn a_lock_a_commit_that_is_no_records_and_a_head_that_moved_are_each_named_as_somebodys() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("somebodys", &one_area());
    let opened = sim.rig.rev("HEAD");
    // (1) git's lock.
    let lock = sim.rig.root.join(".git/index.lock");
    fs::write(&lock, "").expect("plant git's lock");
    let locked = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(
        json!([
            locked.result["status"],
            locked.result["halted"]["phase"],
            locked.result["halted"]["refused"],
            locked.result["halted"]["whose"]
        ]),
        json!(["halted", "git", "locked", "the orchestrator's"]),
        "{}",
        locked.result
    );
    assert!(lock.exists(), "the lock is never removed by a step");
    assert!(
        refused_line(&locked.result)["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.contains(".git/index.lock")),
        "the file the halt names names the lock"
    );
    fs::remove_file(&lock).expect("the orchestrator removes it");

    // (2) Somebody commits while the stage runs.
    let tuning = "printf 'x\\n' > tuning.txt && git add tuning.txt && git commit -q -m 'chore: a tuning commit'";
    let moved = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"meanwhile": {"record:test:r1": tuning}})),
    );
    let result = &moved.result;
    assert_eq!(
        json!([
            result["halted"]["phase"],
            result["halted"]["refused"],
            result["halted"]["whose"]
        ]),
        json!(["record", "head-moved", "the orchestrator's"]),
        "{result}"
    );
    assert!(
        result["message"].as_str().is_some_and(|message| message
            .contains("Nothing was committed or undone: THE BATCH IS STILL APPLIED")
            && message.contains("its first read meets the commit somebody made")),
        "{result}"
    );
    let theirs = sim.rig.rev("HEAD");
    assert_eq!(
        sim.rig.rev("HEAD~1"),
        opened,
        "the tuning commit, and no record"
    );
    assert!(sim.record(&["pending", "--run", RUN], "")["batch"].is_object());

    // (3) The next first read meets that commit: its author's to push.
    let foreign = sim.invoke(sim.args("test", json!({})), json!({}));
    let result = &foreign.result;
    assert_eq!(
        json!([
            result["halted"]["phase"],
            result["halted"]["refused"],
            result["halted"]["whose"]
        ]),
        json!(["git", "foreign-commit", "its author's"]),
        "{result}"
    );
    assert_eq!(foreign.agents(), ["git:state"]);
    assert_eq!(sim.rig.remote(LOOP), Some(opened), "nothing was pushed");
    let message = result["message"].as_str().expect("a message");
    let named = message
        .split('`')
        .find(|span| span.starts_with("git push origin "))
        .unwrap_or_else(|| panic!("the halt names the push, as a command: {message}"));
    assert_eq!(named, format!("git push origin {LOOP}"));
    sim.rig.git(&named.split(' ').skip(1).collect::<Vec<_>>());
    assert_eq!(sim.rig.remote(LOOP), Some(theirs.clone()));

    // (4) And the stage, invoked once more, finishes the record it left — on the tree as
    // it now stands.
    let finished = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(finished.result["status"], "recorded", "{}", finished.result);
    assert_eq!(
        sim.rig.rev("HEAD~1"),
        theirs,
        "the record, on the tuning commit"
    );
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");
    assert_eq!(sim.rig.status(), "");
}

/// **A round's scope that was set aside halts the stage at the report check** (the second
/// repair plan's `K1`, through `K3`): the file at the scope's path is no longer what the
/// record script would have written, the check moves it out of the tree and names it
/// (`aside`) — and the round has no scope any more, which until `K3` was first found at the
/// record. The next attempt's scope step writes it again.
#[test]
fn a_rounds_scope_that_was_set_aside_halts_the_stage_at_the_report_check() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("scope-aside", &one_area());
    let scope = format!("{RUN_DIR}/r1/scope.md");
    let spoil = format!(
        "sed -i.bak 's#scripted: jigc doc set#seen in {}/probe#' {scope} && rm {scope}.bak",
        sim.rig.dir().join("home").display()
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"meanwhile": {"area-a:review": spoil}})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["aside"]
        ]),
        json!(["halted", "scope", [{"path": scope, "why": "host-path"}]]),
        "{result}"
    );
    assert!(
        result["halted"]["reason"]
            .as_str()
            .is_some_and(|why| why.contains("was set aside by the report check")
                && why.contains("round 1 has no scope any more")),
        "{result}"
    );
    assert_eq!(
        *ran.agents().last().expect("a last agent"),
        "git:check-reports"
    );
    assert!(
        !sim.rig.root.join(&scope).exists(),
        "the file left the tree"
    );
    let again = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert_eq!(again.result["scope"]["status"], "written", "written again");
}

/// **What a held command's step says is read as the tool's words, and its verdict as the
/// tool's file** (the second repair plan's `K10`, read by `K3`: DECISIONS.md → *A long
/// command is started, waited for and judged by the tool*, item 12). The real
/// `hold-start` and `hold-wait`, asked for their digests, over the step suite's stand-in for
/// a long command; each line read by the harness's own `readDigest` and `heldOf`: a command
/// that still runs (`running`), one that ran to its end — green, red, and void where its
/// output holds no verdict — and one whose supervisor is gone (`dead`), which has no verdict
/// whatever its output holds. A held check's result row is the verdict's file and no word.
/// WHAT STARTS AND WAITS FOR ONE IN A STAGE IS `K11`'s: no stage does yet.
#[test]
fn what_a_held_commands_step_says_is_read_as_the_tools_words_and_its_verdict_as_the_tools_file() {
    if !can_run() {
        return;
    }
    let held = Held::new("sim-held");
    let scratch = held.rig.scratch.display().to_string();
    let ends = |line: String, act: &str| -> Value {
        let step = harness_pure(&json!([["readDigest", [{"status": "ran", "line": line}, act]]]))
            .remove(0);
        assert!(step.get("relay").is_none(), "a digest of the tool: {step}");
        harness_pure(&json!([["heldOf", [step, scratch]]])).remove(0)
    };
    let green = as_recorded("gate-green.txt");

    // RUNNING: started, and still running when a slice is over.
    held.released(false);
    let started = ends(
        held.start_digest("gate-c1-a1", &GATE_KIND, &green, 0),
        "hold-start",
    );
    assert_eq!(
        json!([
            started["ends"],
            started["name"],
            started["kind"],
            started["output"]
        ]),
        json!([
            "running",
            "gate-c1-a1",
            "gate",
            format!("{scratch}/hold/gate-c1-a1/output")
        ]),
        "{started}"
    );
    held.runs_when(1);
    assert_eq!(
        ends(held.wait_digest("gate-c1-a1", 1), "hold-wait")["ends"],
        "running"
    );

    // DONE, GREEN: the tool's verdict, in the file the line names.
    held.released(true);
    let done = ends(held.wait_digest("gate-c1-a1", 60), "hold-wait");
    assert_eq!(
        json!([done["ends"], done["why"]]),
        json!(["green", null]),
        "{done}"
    );
    let file = done["verdict"].as_str().expect("the verdict's file");
    let kept = fs::read_to_string(file).expect("the verdict the tool kept");
    assert_eq!(done["verdict_sha256"], sha256(&kept).as_str(), "{done}");
    assert_eq!(file, format!("{scratch}/hold/gate-c1-a1/verdict.json"));

    // DONE, RED and DONE, VOID: the same file, the tool's word.
    let red = as_recorded("gate-red.txt");
    held.start_digest("gate-c1-a2", &GATE_KIND, &red, 1);
    let was_red = ends(held.wait_digest("gate-c1-a2", 60), "hold-wait");
    assert_eq!(was_red["ends"], "red", "{was_red}");
    let quick = as_recorded("gate-quick.txt");
    held.start_digest("gate-c1-a3", &GATE_KIND, &quick, 0);
    let was_void = ends(held.wait_digest("gate-c1-a3", 60), "hold-wait");
    assert_eq!(
        json!([was_void["ends"], was_void["why"]]),
        json!(["void", "no-verdict"]),
        "a pre-check's output is no verdict: {was_void}"
    );

    // DEAD: the supervisor is gone, and a whole green gate in the output changes nothing.
    held.released(false);
    let line = held.start_digest("gate-c1-a4", &GATE_KIND, &green, 0);
    let pid = serde_json::from_str::<Value>(&line).expect("a digest")["pid"]
        .as_u64()
        .expect("the supervisor's pid");
    held.runs_when(4);
    fs::write(held.rig.scratch.join("hold/gate-c1-a4/output"), &green).expect("write the output");
    let killed = Command::new("kill")
        .args(["-9", &pid.to_string()])
        .output()
        .expect("run kill");
    assert!(killed.status.success(), "{killed:?}");
    let dead = ends(held.wait_digest("gate-c1-a4", 60), "hold-wait");
    assert_eq!(
        json!([dead["ends"], dead["why"], dead.get("verdict")]),
        json!(["dead", "no-exit", null]),
        "{dead}"
    );
    held.released(true);

    // A HELD CHECK'S RESULT IS THE FILE, AND NO WORD.
    let rows = harness_pure(&json!([["resultRows", [[
        {"item": "regression-set", "status": "green", "verdict": file},
        {"item": "gate", "status": "green"},
    ]]]]))
    .remove(0);
    assert_eq!(
        rows,
        json!([{"item": "regression-set", "verdict": file}, {"item": "gate", "outcome": "green"}])
    );
}

// ---------------------------------------------------------------------------
// What a stage tells an agent to run (the second repair plan's `K11`)
// ---------------------------------------------------------------------------

/// The names of a stage's held commands, as the harness mints them.
fn previous_build(sim: &Sim) -> String {
    format!("build-previous-{}", &sim.rig.rev("main")[..12])
}

/// THE NAMES OF WHAT A STAGE HOLDS ARE THE COMMIT'S (the reshape's plan, `T5b`; its review's
/// `S12`): the candidate's build and its gate by the candidate's commit, a held check by that
/// commit and THE ITEM'S OWN RUN IN THE ROUND — never by the stage's attempt, so that a stage
/// started again asks for the names the stage before it asked for. A job that stands under a
/// name and ended red, void or dead is passed over for the next: `<name>-2`, `<name>-3`, ….
fn candidate_build(sha: &str) -> String {
    format!("build-candidate-{}", &sha[..12])
}

fn candidates_gate(sha: &str) -> String {
    format!("gate-{}", &sha[..12])
}

fn held_check(item: &str, sha: &str, run: u32) -> String {
    format!("{item}-{}-a{run}", &sha[..12])
}

/// Where the candidate's binary lies, under the scratch root.
fn candidate_bin(sha: &str) -> String {
    format!("bin/candidate-{}/jigc", &sha[..12])
}

impl Sim {
    /// The commit the run was opened by: the candidate of its first round, in every scenario
    /// that commits nothing between the opening and its first stage.
    fn opened_at(&self) -> String {
        self.rig
            .git(&[
                "rev-list",
                "--max-count=1",
                "--grep=^docs(record): the run is opened$",
                LOOP,
            ])
            .trim()
            .to_owned()
    }
}

/// The kinds of step a held command is: its start, and the step that asks after it.
fn held_steps(name: &str) -> [String; 2] {
    [
        format!("git:hold-start:{name}"),
        format!("git:hold-wait:{name}"),
    ]
}

/// An opening whose test set holds held checks beside one audit area: the regression set,
/// `held-regression`, once per item of `regressions`, and the gate, `held-gate`.
fn with_held_checks(regressions: &[&str]) -> Opening {
    let mut items = vec![
        item("area-a", "audit-area", "audit-clean"),
        item("gate", "held-gate", "working-product"),
    ];
    items.extend(
        regressions
            .iter()
            .map(|id| item(id, "held-regression", "working-product")),
    );
    Opening {
        clauses: vec!["audit-clean", "working-product"],
        items: json!(items),
        bounds: vec![],
        rows: json!([]),
    }
}

/// How an item of the test set stands, as the state document has it.
fn standing(state: &Value, id: &str) -> Value {
    state["clauses"]
        .as_array()
        .expect("the clauses")
        .iter()
        .flat_map(|clause| clause["items"].as_array().expect("its items").clone())
        .find(|entry| entry["item"] == id)
        .map(|entry| json!([entry["standing"], entry["why"]]))
        .unwrap_or_else(|| panic!("the state has the item `{id}`: {state}"))
}

/// THE HUMAN'S RULING OF 2026-10-07 — *"there should be tooling for this"*: an agent that
/// was told to start a command "in the background" and wait "in slices" composed a shell
/// no permission rule can name, and stopped on a prompt. So NO AGENT OF A STAGE COMPOSES A
/// SHELL: the two binaries, the candidate's gate and the record's gate are each started by
/// ONE plain call of the step tool and asked after by ONE plain call, the batch and every
/// report reach their writer from a file, and every command that ran on an agent's behalf
/// is a command of `dev/stabilize-step` or of `dev/stabilize-record`, as its prompt spells it.
#[test]
fn the_builds_and_the_gates_are_held_by_acts_and_every_command_an_agent_runs_is_a_plain_call_of_a_tool()
 {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("held-acts", &one_area());
    let previous = previous_build(&sim);
    let opened = sim.opened_at();
    let (build, gate) = (candidate_build(&opened), candidates_gate(&opened));
    let ran = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);

    // THE STEPS: each long command started by one step and asked after by another — the
    // candidate's binary, the previous release's, the candidate's gate, the record's gate.
    let agents: Vec<String> = ran.agents().iter().map(|a| (*a).to_owned()).collect();
    let steps: Vec<String> = agents
        .iter()
        .filter(|label| label.starts_with("git:hold-"))
        .cloned()
        .collect();
    let expected: Vec<String> = [
        build.as_str(),
        previous.as_str(),
        gate.as_str(),
        "record-test-r1-a1-1",
    ]
    .iter()
    .flat_map(|name| held_steps(name))
    .collect();
    assert_eq!(steps, expected, "the held commands of a stage, in order");
    // And the preflight runs before anything is built: an assert that fails stops the
    // stage there.
    let at = |label: &str| {
        agents
            .iter()
            .position(|a| a == label)
            .unwrap_or_else(|| panic!("`{label}` was launched: {agents:?}"))
    };
    assert!(at("preflight") < at(&format!("git:hold-start:{build}")));
    assert!(
        at(&format!("git:hold-wait:{gate}")) < at("area-a:review"),
        "the instruments follow the candidate's gate"
    );
    assert!(
        at("record:test:r1") < at("git:hold-start:record-test-r1-a1-1")
            && at("git:hold-wait:record-test-r1-a1-1") < at("git:record:test:r1"),
        "the record's gate stands between its batch and its commit"
    );

    // EVERY COMMAND THAT RAN is one plain call of a tool — and of no other program.
    for command in &ran.commands {
        assert!(
            command.starts_with("dev/stabilize-step ")
                || command.starts_with("dev/stabilize-record "),
            "an agent ran `{command}`"
        );
        for shape in ["|", ">", "<", "&", ";", "$", "\n"] {
            assert!(!command.contains(shape), "`{command}` holds `{shape}`");
        }
    }
    // THE LONG COMMANDS RAN ONCE EACH, started by the tool and by nobody else: the gate
    // twice — the candidate's and the record's — and one build per binary.
    let long = sim.long_commands();
    assert_eq!(
        long.iter().filter(|c| *c == "gate --keep-going").count(),
        2,
        "{long:?}"
    );
    assert_eq!(
        long.iter().filter(|c| c.starts_with("cargo ")).count(),
        2,
        "{long:?}"
    );

    // NO PROMPT TELLS AN AGENT TO HOLD ANYTHING BY ITS OWN MEANS — the preflight's and the
    // executor's least of all: neither names a gate or a build.
    for (label, prompt) in &ran.prompts {
        for word in [
            "in the background",
            "in slices",
            "here-document",
            "shasum",
            "git archive",
            "dev/gate",
            "2>&1",
        ] {
            assert!(
                !prompt.contains(word),
                "the prompt of `{label}` holds `{word}`"
            );
        }
    }
    let executor = &ran
        .prompts
        .iter()
        .find(|(label, _)| label == "record:test:r1")
        .expect("the record's executor")
        .1;
    assert!(
        executor.contains(" --from ")
            && executor.contains(" --sha256 ")
            && executor.contains("RUN NO GATE"),
        "the executor writes a file, applies it by its hash, and runs no gate: {executor}"
    );

    // THE BINARY'S HASH IS THE TOOL'S: what the stage returns, and what the round's record
    // holds, is the hash of the file every driving agent was handed.
    let candidate = &ran.result["candidate"];
    let binary = candidate["binary"].as_str().expect("the binary's path");
    assert_eq!(
        binary,
        format!("{}/{}", sim.rig.scratch.display(), candidate_bin(&opened))
    );
    let bytes = fs::read(binary).expect("the binary the tool built");
    assert_eq!(candidate["sha256"], sha256_of(&bytes).as_str());
    let state = sim.state();
    assert_eq!(state["rounds"][0]["facts"]["binary"], candidate["sha256"]);
    // AND THE ROUND'S GATE IS ON RECORD from the file the tool kept the gate's output in.
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/gate.md"))
            .contains("pass"),
        "the candidate's gate is a fact of the round"
    );
}

fn sha256_of(bytes: &[u8]) -> String {
    sha256(std::str::from_utf8(bytes).expect("a stand-in binary is text"))
}

/// A SCRIPTED CHECK'S VERDICT IS THE TOOL'S (the re-review's `R-M8`). The stage starts the
/// regression set — after the candidate's gate, never beside it — and the result it records
/// is THE FILE THE TOOL KEPT ITS VERDICT IN: red where that tool's one line says red,
/// whatever any agent of the stage says, and void with the tool's own reason for each of
/// the seven that tool has. The gate as an item of the test set is answered from the
/// candidate's own gate, which is not run twice.
#[test]
fn a_held_check_is_started_by_the_stage_and_recorded_from_the_tools_verdict_file() {
    if !can_run() {
        return;
    }
    let reasons = [
        "did-not-run",
        "build-previous",
        "build-candidate",
        "baseline",
        "swap",
        "incomplete",
        "evidence",
    ];
    let voids: Vec<String> = reasons.iter().map(|r| format!("reg-{r}")).collect();
    let mut ids: Vec<&str> = vec!["regression-set", "reg-green"];
    ids.extend(voids.iter().map(String::as_str));
    let sim = Sim::opened("held-checks", &with_held_checks(&ids));
    let opened = sim.opened_at();
    let (gate, regression) = (
        candidates_gate(&opened),
        held_check("regression-set", &opened, 1),
    );
    let mut held = json!({"regression-set": {"status": "red"}, "reg-green": {"status": "green"}});
    for reason in reasons {
        held[format!("reg-{reason}")] = json!({"status": "void", "reason": reason});
    }
    // The preflight's stand-in would call every check green: nobody asks it.
    let said_green: serde_json::Map<String, Value> = ids
        .iter()
        .map(|id| ((*id).to_owned(), json!("green")))
        .collect();
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"held": held, "checks": said_green})),
    );
    assert_eq!(ran.result["status"], "triaged", "{}", ran.result);
    let agents = ran.agents();
    let at = |label: &str| {
        agents
            .iter()
            .position(|a| *a == label)
            .unwrap_or_else(|| panic!("`{label}` was launched: {agents:?}"))
    };
    assert!(
        at(&format!("git:hold-wait:{gate}")) < at(&format!("git:hold-start:{regression}")),
        "the regression set runs after the gate, never beside it"
    );
    assert!(at(&format!("git:hold-wait:{regression}")) < at("area-a:review"));
    assert!(
        !ran.prompts
            .iter()
            .any(|(label, prompt)| label.starts_with("preflight") && prompt.contains("regression")),
        "the preflight is asked nothing about a held check"
    );

    // WHAT IS ON RECORD: red, green, and void with the tool's reason — each from its file.
    let state = sim.state();
    assert_eq!(
        standing(&state, "regression-set"),
        json!(["red", null]),
        "{state}"
    );
    assert_eq!(standing(&state, "reg-green"), json!(["green", null]));
    assert_eq!(standing(&state, "gate"), json!(["green", null]));
    for id in &voids {
        assert_eq!(standing(&state, id), json!(["void", "void"]), "{id}");
    }
    let results = sim.rig.read(&format!("{RUN_DIR}/r1/results.md"));
    for reason in reasons {
        assert!(
            results.contains(reason),
            "the void's reason `{reason}` is on record: {results}"
        );
    }
    assert!(
        state["ledger"]
            .as_array()
            .expect("the ledger")
            .iter()
            .any(|row| row["key"] == "red-regression-set"),
        "a red check is filed as a finding by the record script: {state}"
    );
    // The verdicts' files are among the record's paths, written once.
    let (_, paths) = head_commit(&sim);
    for id in ids.iter().chain(["gate"].iter()) {
        let file = format!("{RUN_DIR}/r1/checks/{id}.a1.json");
        assert!(
            paths.contains(&file),
            "`{file}` is in the record's commit: {paths:?}"
        );
    }
    let kept: Value = serde_json::from_str(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/checks/regression-set.a1.json"))
            .trim_end(),
    )
    .expect("the verdict on record is the tool's line");
    assert_eq!(
        json!([
            kept["item"],
            kept["kind"],
            kept["verdict"],
            kept["commit"],
            kept["facts"]["candidate"]
        ]),
        json!([
            "regression-set",
            "regression",
            "red",
            ran.result["candidate"]["sha"],
            ran.result["candidate"]["sha"]
        ]),
        "{kept}"
    );
    // The gate's verdict is the candidate's gate, which ran once beside the record's.
    assert_eq!(
        sim.long_commands()
            .iter()
            .filter(|c| *c == "gate --keep-going")
            .count(),
        2
    );
    assert_eq!(
        ran.result["held"]
            .as_array()
            .expect("what became of each held check")
            .iter()
            .map(|h| json!([h["item"], h["ends"]]))
            .collect::<Vec<_>>()[..3],
        [
            json!(["gate", "green"]),
            json!(["regression-set", "red"]),
            json!(["reg-green", "green"])
        ]
    );
}

/// HOW A HELD COMMAND CAN END, each with its cell. A step that asks after one and dies is
/// followed by another THAT TAKES THE WAIT OVER — the command ran once. A held check whose
/// supervisor was killed is DEAD: it left no verdict, so it is VOID BY A WORD — the stage
/// goes on and records that run, with the job's name. A command still running when nobody is left to ask after it
/// halts the stage and NAMES THE JOB, with the one call that answers once it is over. A
/// candidate's gate whose output holds no verdict, and a commit that does not build, halt
/// the stage before any instrument runs.
#[test]
fn a_held_command_that_is_taken_over_dead_left_running_void_or_red_has_its_cell() {
    if !can_run() {
        return;
    }
    // TAKEN OVER: the first agent that asks after the candidate's gate dies; the step is
    // asked for again — the ONE command, which is the wait and never the start.
    let sim = Sim::opened("held-over", &with_held_checks(&["regression-set"]));
    let scratch = sim.rig.scratch.display().to_string();
    let opened = sim.opened_at();
    let (build, gate) = (candidate_build(&opened), candidates_gate(&opened));
    let over = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"endings": {
            format!("git:hold-wait:{gate}"): "acts-and-dies-once",
            format!("git:hold-start:{build}"): "acts-and-dies-once",
        }})),
    );
    assert_eq!(over.result["status"], "triaged", "{}", over.result);
    let asked = |label: &str| over.agents().iter().filter(|a| **a == label).count();
    assert_eq!(
        [
            asked(&format!("git:hold-start:{gate}")),
            asked(&format!("git:hold-wait:{gate}"))
        ],
        [1, 2],
        "the wait is asked again, and the start is not"
    );
    // A START WHOSE AGENT DIED is asked again too, and answers with the job that is there.
    assert_eq!(asked(&format!("git:hold-start:{build}")), 2);
    let long = sim.long_commands();
    assert_eq!(
        [
            long.iter().filter(|c| *c == "gate --keep-going").count(),
            long.iter().filter(|c| c.starts_with("cargo ")).count()
        ],
        [2, 2],
        "the candidate's gate and its build each ran ONCE: {long:?}"
    );

    // DEAD: the regression set's supervisor is killed while the command runs.
    let sim = Sim::opened("held-dead", &with_held_checks(&["regression-set"]));
    let scratch_dead = sim.rig.scratch.display().to_string();
    let regression = held_check("regression-set", &sim.opened_at(), 1);
    let kill = format!(
        "python3 -c \"import json,os,signal; os.kill(json.load(open('{scratch_dead}/hold/{regression}/job.json'))['pid'], signal.SIGKILL)\""
    );
    let dead = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "unreleased": [regression],
            "meanwhile": {format!("git:hold-start:{regression}"): kill},
        })),
    );
    sim.release(&regression);
    assert_eq!(dead.result["status"], "triaged", "{}", dead.result);
    let of = dead.result["held"]
        .as_array()
        .expect("the held checks")
        .iter()
        .find(|h| h["item"] == "regression-set")
        .expect("the regression set");
    assert_eq!(
        json!([of["ends"], of["why"]]),
        json!(["dead", "no-exit"]),
        "{of}"
    );
    // TURNED — it had no result on record, and nothing counted its run: A HELD CHECK THAT
    // LEFT NO VERDICT IS VOID BY A WORD (the record script's `result-set`, for a held item:
    // `void` and nothing else), the word the tool answered with and the job's name.
    let state = sim.state();
    assert_eq!(
        standing(&state, "regression-set"),
        json!(["void", "void"]),
        "a dead check is a void RUN on record: {state}"
    );
    assert_eq!(
        results(&state, 1)
            .into_iter()
            .find(|row| row[0] == "regression-set"),
        Some(json!([
            "regression-set",
            1,
            "void",
            format!("without a verdict: dead:{regression}")
        ])),
        "by the word and the job's name, in the shape the record script takes: {state}"
    );
    assert!(
        dead.result["counts"]["voided"]
            .as_array()
            .is_some_and(|voided| voided.contains(&json!("regression-set"))),
        "the return names it void: {}",
        dead.result
    );
    assert_eq!(standing(&state, "gate"), json!(["green", null]));
    assert!(
        !sim.rig
            .root
            .join(format!("{RUN_DIR}/r1/checks/regression-set.a1.json"))
            .exists(),
        "and no verdict's file"
    );

    // ITS START REFUSED: a command of another kind stands under the held check's name, so
    // the tool refuses the start (`taken`) and starts nothing. The check is void BY THE
    // TOOL'S OWN WORD, with the name — and the stage records.
    let sim = Sim::opened("held-taken", &with_held_checks(&["regression-set"]));
    let scratch_taken = sim.rig.scratch.display().to_string();
    let regression = held_check("regression-set", &sim.opened_at(), 1);
    let other = sim.step(&[
        "hold-start",
        "--scratch",
        &scratch_taken,
        "--name",
        &regression,
        "--kind",
        "probe",
        "--seconds",
        "1",
    ]);
    assert_eq!(other["status"], "started", "{other}");
    let taken = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(taken.result["status"], "triaged", "{}", taken.result);
    assert_eq!(
        results(&sim.state(), 1)
            .into_iter()
            .find(|row| row[0] == "regression-set"),
        Some(json!([
            "regression-set",
            1,
            "void",
            format!("without a verdict: taken:{regression}")
        ])),
        "a refused start is a void run, by the tool's word: {}",
        taken.result
    );
    assert!(
        !taken
            .agents()
            .contains(&format!("git:hold-wait:{regression}").as_str()),
        "nobody asks after a command that was not started: {:?}",
        taken.agents()
    );

    // LEFT RUNNING: nobody is left to ask after the candidate's gate.
    let sim = Sim::opened("held-running", &one_area());
    let scratch_running = sim.rig.scratch.display().to_string();
    let opened = sim.rig.rev("HEAD");
    let gate = candidates_gate(&opened);
    let left = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "unreleased": [gate],
            "endings": {format!("git:hold-wait:{gate}"): "dies"},
            "agents": {},
        })),
    );
    let halted = &left.result;
    assert_eq!(
        json!([
            halted["status"],
            halted["halted"]["phase"],
            halted["halted"]["transient"]
        ]),
        json!(["halted", "gate", true]),
        "{halted}"
    );
    assert_eq!(
        json!([
            halted["halted"]["held"]["name"],
            halted["halted"]["held"]["kind"],
            halted["halted"]["held"]["output"]
        ]),
        json!([
            gate,
            "gate",
            format!("{scratch_running}/hold/{gate}/output")
        ]),
        "the halt names the job: {halted}"
    );
    let call = format!("`dev/stabilize-step hold-wait --scratch {scratch_running} --name {gate}`");
    assert!(
        text(&halted["message"]).contains(&call)
            && text(&halted["message"]).contains("MAY STILL RUN"),
        "and the one call that answers once it is over: {halted}"
    );
    assert!(
        !left.agents().contains(&"area-a:review"),
        "no instrument ran"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was recorded");
    let asking = [
        "hold-wait",
        "--scratch",
        &scratch_running,
        "--name",
        &gate,
        "--slice",
        "1",
    ];
    assert_eq!(
        sim.step(&asking)["status"],
        "running",
        "the command runs on"
    );
    sim.release(&gate);
    let ended = sim.step(&["hold-wait", "--scratch", &scratch_running, "--name", &gate]);
    assert_eq!(
        json!([ended["status"], ended["verdict"]]),
        json!(["done", "green"])
    );

    // VOID: the candidate's gate printed no verdict. Nothing could be recorded against it.
    let sim = Sim::opened("held-void", &one_area());
    let void = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"gate": {"void": true}, "agents": {}})),
    );
    let halted = &void.result;
    assert_eq!(
        json!([
            halted["status"],
            halted["halted"]["phase"],
            halted["halted"]["held"]["ends"],
            halted["halted"]["held"]["why"]
        ]),
        json!(["halted", "gate", "void", "no-verdict"]),
        "{halted}"
    );
    assert!(!void.agents().contains(&"area-a:review"));

    // RED: the candidate's commit does not build. No instrument runs without the binary.
    let sim = Sim::opened("held-red", &one_area());
    let unbuilt = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"builds": {"fails": ["candidate"]}, "agents": {}})),
    );
    let halted = &unbuilt.result;
    assert_eq!(
        json!([
            halted["status"],
            halted["halted"]["phase"],
            halted["halted"]["held"]["ends"],
            halted["halted"]["held"]["why"]
        ]),
        json!(["halted", "build", "red", "build"]),
        "{halted}"
    );
    assert!(
        !unbuilt
            .agents()
            .iter()
            .any(|a| a.starts_with("git:hold-start:gate"))
    );
    let _ = scratch;
}

/// A HELD CHECK THAT NAMES ITS COMMITS IS RUN AGAIN INSIDE ITS ROUND — on the round's
/// candidate, never the tip (the orchestrator's ruling of 2026-10-07 on the core review's
/// `F3`; the harness's header: *where that ends*). The regression set is started with the
/// two commits it compares, so a re-run can hold it again though the round's record
/// commits have moved the tree on; THE CANDIDATE'S GATE IS NOT HELD AGAIN — it reads the
/// working tree — and the round keeps the gate it has on record.
#[test]
fn a_held_regression_that_was_void_is_run_again_in_a_rerun_on_the_rounds_candidate_and_no_gate_is()
{
    if !can_run() {
        return;
    }
    let sim = Sim::opened("held-rerun", &with_held_checks(&["regression-set"]));
    let candidate = sim.rig.rev("HEAD");
    let first = sim.invoke(
        sim.args("test", json!({})),
        small_stage(
            json!({"held": {"regression-set": {"status": "void", "reason": "did-not-run"}}}),
        ),
    );
    assert_eq!(first.result["status"], "triaged", "{}", first.result);
    assert_eq!(
        standing(&sim.state(), "regression-set"),
        json!(["void", "void"])
    );
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(
        json!([go.result["status"], go.result["next"]]),
        json!(["ruled", "retest"]),
        "the state asks for the void check again: {}",
        go.result
    );
    assert_ne!(
        sim.rig.rev("HEAD"),
        candidate,
        "record commits moved the tip"
    );

    let again = sim.invoke(
        sim.args("test", json!({"clause": "working-product"})),
        json!({}),
    );
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    let scratch = sim.rig.scratch.display().to_string();
    let previous = sim.rig.rev("main");
    assert!(
        again.commands.contains(&format!(
            "dev/stabilize-step hold-start --scratch {scratch} --name {} --kind regression --previous {previous} --candidate {candidate} --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv --baseline baseline/previous-{} --digest {scratch}",
            held_check("regression-set", &candidate, 2),
            &previous[..12]
        )),
        "the regression set is held again, over the round's candidate: {:?}",
        again.commands
    );
    assert!(
        !again
            .agents()
            .iter()
            .any(|a| a.starts_with("git:hold-start:gate-")),
        "no gate of the candidate's is held in a re-run: {:?}",
        again.agents()
    );
    let state = sim.state();
    assert_eq!(
        standing(&state, "regression-set"),
        json!(["green", null]),
        "{state}"
    );
    let kept: Value = serde_json::from_str(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/checks/regression-set.a2.json"))
            .trim_end(),
    )
    .expect("the re-run's verdict on record");
    assert_eq!(
        json!([kept["verdict"], kept["commit"]]),
        json!(["green", candidate]),
        "{kept}"
    );
    assert_eq!(
        sim.long_commands()
            .iter()
            .filter(|c| *c == "gate --keep-going")
            .count(),
        4,
        "the candidate's gate once, and one gate per record — the stage's, the ruling's, the re-run's: {:?}",
        sim.long_commands()
    );
}

/// A HELD GATE THAT IS DUE AGAIN IS VOID BY A WORD, AND `next` LEAVES `retest` (the harness
/// task `K11`'s item 8, finished: the record script takes `void` as the one word of a held
/// check, and the harness composes it). A gate reads the working tree, which in a re-run is
/// no longer the round's candidate — so it is not held again inside its round. Until this
/// nothing could be recorded for it: a re-run in which it alone was due was refused before
/// any agent ran, and `next` stayed `retest` for a clause no invocation could move. THE
/// ROAD THERE is the one that will come: the candidate's gate is red, its red is a finding,
/// the finding is confirmed, the human rules it *later*, and gives the go.
#[test]
fn a_held_gate_that_is_due_again_is_recorded_void_by_a_word_and_next_leaves_retest() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("held-gate-rerun", &with_held_checks(&[]));
    let candidate = sim.rig.rev("HEAD");
    let first = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"gate": red(&[KNOWN])})),
    );
    assert_eq!(
        json!([first.result["status"], first.result["next"]]),
        json!(["triaged", "triage"]),
        "the candidate's gate is red, and its red is a finding nobody graded yet: {}",
        first.result
    );
    assert_eq!(standing(&sim.state(), "gate"), json!(["red", null]));
    let lap = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            "triage:p1": graded(&[again("red-gate", "the held check `gate`", "working-product", "breaks")], &["ledger"]),
            "verify:red-gate": verdict("red-gate", "confirmed"),
        }}),
    );
    assert_eq!(lap.result["next"], "rule", "{}", lap.result);
    let ruled = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"key": "red-gate", "ruling": "later", "note": "for the next release"}]}),
        ),
        json!({}),
    );
    assert_eq!(ruled.result["next"], "stop", "{}", ruled.result);
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(
        json!([go.result["next"], sim.state()["retest"]]),
        json!(["retest", ["working-product"]]),
        "the state asks for the red gate again: {}",
        go.result
    );
    assert_ne!(
        sim.rig.rev("HEAD"),
        candidate,
        "record commits moved the tip"
    );
    let gates = sim
        .long_commands()
        .iter()
        .filter(|c| *c == "gate --keep-going")
        .count();

    // THE RE-RUN: the gate is NOT HELD AGAIN — the tree is not the round's candidate — and
    // it is RECORDED, void by the one word the record script takes of a held check: a run
    // that counts, on the round's candidate.
    let again = sim.invoke(
        sim.args("test", json!({"clause": "working-product"})),
        json!({}),
    );
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert!(
        !again
            .agents()
            .iter()
            .any(|a| a.starts_with("git:hold-start:gate-")),
        "no gate of the candidate's is held in a re-run: {:?}",
        again.agents()
    );
    assert_eq!(
        sim.long_commands()
            .iter()
            .filter(|c| *c == "gate --keep-going")
            .count(),
        gates + 1,
        "one gate ran: the re-run's own record's"
    );
    let state = sim.state();
    assert_eq!(
        results(&state, 1)
            .into_iter()
            .filter(|row| row[0] == "gate")
            .collect::<Vec<_>>(),
        [
            json!(["gate", 1, "red", null]),
            json!(["gate", 2, "void", "without a verdict: off-tree"])
        ],
        "the gate's re-run is a run on record, void by a word: {state}"
    );
    assert!(
        state["rounds"][0]["results"]
            .as_array()
            .is_some_and(|rows| rows.iter().all(|row| row["commit"] == candidate.as_str())),
        "and names the round's candidate: {state}"
    );
    assert_eq!(
        json!([
            again.result["counts"]["voided"],
            again.result["held"][0]["item"],
            again.result["held"][0]["ends"]
        ]),
        json!([["gate"], "gate", "not-run"]),
        "the return names it: {}",
        again.result
    );
    // `next` LEAVES `retest`: the clause was run twice inside its round and is not green,
    // so it is the human's — settled by the next round, whose candidate is the tip.
    assert_eq!(
        json!([
            again.result["next"],
            state["next"],
            state["retest"],
            state["human_clauses"]
        ]),
        json!([
            "rule",
            "rule",
            [],
            [{"clause": "working-product", "why": "not-green-after-its-rerun"}]
        ]),
        "{state}"
    );
}

/// A RECORD'S GATE THAT LEFT NO VERDICT, and the name of the next one. The batch stays
/// applied; the next invocation gates it again — UNDER THE NEXT NAME, because a held
/// command is never started again under its own, and a job an earlier invocation left is
/// never read as this one's — and commits it. No executor runs for a batch that is applied.
#[test]
fn a_records_gate_with_no_verdict_leaves_the_batch_applied_and_the_next_invocation_names_the_next_gate()
 {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("held-record", &one_area());
    let opened = sim.rig.rev("HEAD");
    let first = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"record": {"gate": {"void": true}}})),
    );
    let halted = &first.result;
    assert_eq!(
        json!([
            halted["status"],
            halted["halted"]["phase"],
            halted["halted"]["held"]["name"],
            halted["halted"]["held"]["ends"]
        ]),
        json!(["halted", "record", "record-test-r1-a1-1", "void"]),
        "{halted}"
    );
    assert!(text(&halted["message"]).contains("THE BATCH"), "{halted}");
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert!(
        !first.agents().contains(&"git:record:test:r1"),
        "no commit step ran"
    );

    // The next invocation: the gate again, twice void — each under the next name.
    let second = sim.invoke(
        sim.args("test", json!({})),
        json!({"record": {"gate": {"void": true}}}),
    );
    assert_eq!(
        second.result["halted"]["held"]["name"], "record-pending-1",
        "{}",
        second.result
    );
    assert!(
        !second.agents().iter().any(|a| a.starts_with("record:")),
        "a batch that is applied needs no executor: {:?}",
        second.agents()
    );
    let third = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(third.result["status"], "recorded", "{}", third.result);
    let steps: Vec<&str> = third
        .agents()
        .into_iter()
        .filter(|a| a.starts_with("git:hold-"))
        .collect();
    assert_eq!(
        steps,
        [
            "git:hold-start:record-pending-1",
            "git:hold-start:record-pending-2",
            "git:hold-wait:record-pending-2"
        ],
        "the job of the earlier invocation is not this one's: the next name is asked"
    );
    assert_ne!(sim.rig.rev("HEAD"), opened, "the record is committed");
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(sim.rig.rev("HEAD")),
        "and pushed"
    );

    // AN ALTERED BATCH is refused by the writer, which holds the file to its hash.
    let sim = Sim::opened("held-altered", &one_area());
    let altered = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"record": {"payload": "altered"}})),
    );
    assert_eq!(
        json!([altered.result["status"], altered.result["halted"]["phase"]]),
        json!(["halted", "record"]),
        "{}",
        altered.result
    );
    assert!(
        text(&altered.result["halted"]["reason"]).contains("bad-value"),
        "{}",
        altered.result
    );
    assert_eq!(
        sim.state()["position"]["test"]["attempt"],
        2,
        "nothing was applied"
    );
}

// ---------------------------------------------------------------------------
// A run with a queue: the new path, from its triage on (the reshape's plan, Phase B)
// ---------------------------------------------------------------------------

/// The order a simulated run WITH A QUEUE is opened with: the classes its queue is ranked by.
/// What the order ranks is no subject of these scenarios — that a run holds one is: it is what
/// the state's digest says of the run (`queue`), and what the harness chooses its path by.
const ORDER: [&str; 2] = ["route hunt", "behaviour hunt"];

/// Where the record script renders the entries of a stage's one triage: the path a `test`
/// report of the reporter `triage` and that attempt has.
fn rendering(round: u32, attempt: u32) -> String {
    format!("{RUN_DIR}/r{round}/reports/test/triage.a{attempt}.md")
}

/// The line a rendered report opens with — what says that the record script rendered the file
/// from the entries a triage handed over, and that no agent wrote it.
const RENDERED: &str = "<!-- the entries a triage handed over, rendered by dev/stabilize-record and never written by hand -->";

/// A NEW finding as a triage hands it over in its one file: what its ledger row needs, the
/// grade, why, its kind — and the handed findings it folds, each by its source and its number.
/// The kind's evidence is the caller's to add: without it the entry is a lead.
fn keyed(key: &str, kind: &str, grade: &str, folds: Value) -> Value {
    json!({"key": key, "found": "new", "doctype": "jigc-feedback", "door": "jigc doc set",
           "clause": "audit-clean", "repro": format!("the block of {key}"), "grade": grade,
           "kind": kind, "why": format!("why {key} is graded {grade}"), "folds": folds})
}

/// A finding FOUND AGAIN: the row it is, by its key, the grade, why and what it folds — and
/// none of a row's cells.
fn found_again(key: &str, grade: &str, folds: Value) -> Value {
    json!({"key": key, "found": "again", "grade": grade,
           "why": format!("{key} is a row the run holds, and this is it again"), "folds": folds})
}

impl Sim {
    /// A rig whose run is opened WITH AN ORDER — a run with a queue, as its state says.
    fn queued(label: &str, opening: &Opening) -> Self {
        let sim = Sim::new(label);
        sim.open_with(
            opening,
            &json!({"order": ORDER, "budget": 30, "hand-on": 2}),
        );
        let state = sim.state();
        assert!(
            state["queue"].is_object(),
            "a run that holds an order has a queue: {state}"
        );
        sim
    }

    /// The entries a rendered report holds as data: its one fenced block.
    fn rendered(&self, round: u32, attempt: u32) -> Value {
        let text = self.rig.read(&rendering(round, attempt));
        assert!(
            text.starts_with(&format!("{RENDERED}\n")),
            "the triage's report is the record script's rendering of its entries:\n{text}"
        );
        let (_, rest) = text
            .split_once("\n```json\n")
            .unwrap_or_else(|| panic!("a rendered report holds its entries as data:\n{text}"));
        let (block, _) = rest
            .rsplit_once("\n```\n")
            .unwrap_or_else(|| panic!("the block closes:\n{text}"));
        serde_json::from_str(block).unwrap_or_else(|e| panic!("the block is JSON ({e}):\n{block}"))
    }
}

/// The prompt the stage's one triage was handed.
fn one_triage(ran: &Ran) -> &str {
    &ran.prompts
        .iter()
        .find(|(label, _)| label == "triage")
        .unwrap_or_else(|| panic!("the stage's one triage was launched: {:?}", ran.agents()))
        .1
}

/// The ONE line the call the triage's prompt spells printed: the record script's digest.
fn filed_line(ran: &Ran) -> Value {
    let (_, _, printed) = ran
        .reads
        .iter()
        .find(|(label, command, _)| {
            label == "triage" && command.starts_with("dev/stabilize-record triage ")
        })
        .unwrap_or_else(|| panic!("the triage handed its entries over: {:?}", ran.reads));
    serde_json::from_str(printed).expect("the call prints one line of JSON")
}

/// The line of a prompt that hands the source `name` over, with the numbered lines under it.
fn handed_under(prompt: &str, name: &str) -> Vec<String> {
    let lines: Vec<&str> = prompt.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.starts_with(&format!("- {name} — ")))
        .unwrap_or_else(|| panic!("the prompt hands the source `{name}` over:\n{prompt}"));
    let mut block = vec![lines[at].to_owned()];
    block.extend(
        lines[at + 1..]
            .iter()
            .take_while(|line| line.starts_with("   "))
            .map(|line| (*line).to_owned()),
    );
    block
}

/// The subject of the commit a `test` stage's first record is made by, in round 1.
fn first_record_subject() -> String {
    format!("docs(record): {RUN} r1 - the findings of the test stage")
}

/// THE CANONICAL FORM of a composed record's plain words (`dev/stabilize-record compose`: THE
/// CANONICAL FORM), written here a second time, by hand: the run, the round, the attempt and
/// the subject, a line each, then one line a word in the order handed — each line ended by one
/// line feed, the last too.
fn canonical(round: u32, attempt: u32, subject: &str, words: &[String]) -> String {
    let mut text = format!("run={RUN}\nround={round}\nattempt={attempt}\nsubject={subject}\n");
    for word in words {
        text.push_str(word);
        text.push('\n');
    }
    text
}

/// The commands of the composing step that ran, in order: each as `(its flags, its words)`.
fn composing(ran: &Ran) -> Vec<(String, Vec<String>)> {
    ran.commands
        .iter()
        .filter(|command| command.starts_with("dev/stabilize-step compose "))
        .map(|command| {
            let (flags, words) = command
                .split_once(" -- ")
                .unwrap_or_else(|| panic!("the composing step is handed words: {command}"));
            (
                flags.to_owned(),
                words.split(' ').map(str::to_owned).collect(),
            )
        })
        .collect()
}

/// The value of a flag of a command, as it was typed.
fn flag_of<'a>(flags: &'a str, name: &str) -> &'a str {
    let rest = flags
        .split_once(&format!(" {name} "))
        .unwrap_or_else(|| panic!("the command names `{name}`: {flags}"))
        .1;
    rest.split(' ').next().expect("a value")
}

/// WHERE THE NEW PATH ENDS AT THIS COMMIT: AT ITS FIRST RECORD (the reshape's plan, `T5b`; the
/// harness half of `K3`). The stage returns `hunted` with the state's `next`; the record is
/// ONE commit on the candidate, pushed, and the tree is clean; **the record step is tool steps
/// only** — the composing step, the record's gate held, the one commit, the push — and NO
/// EXECUTOR IS LAUNCHED and no batch typed; nothing is verified, and no agent of the other
/// path runs: no pass of a triage, no verifier, no advocate, no independent drive. AND THE
/// WORDS THE STEP WAS HANDED HASH, IN THEIR CANONICAL FORM, TO THE SHA256 THE HARNESS COMPUTED
/// IN JAVASCRIPT: the form is written here a second time, by hand, and the record script — which
/// computes it a third time — took the call. Returns the words.
fn ends_at_its_first_record(
    sim: &Sim,
    ran: &Ran,
    candidate: &str,
    attempt: u32,
    next: &str,
) -> Vec<String> {
    let result = &ran.result;
    assert_eq!(
        json!([result["status"], result["next"], result["halted"]]),
        json!(["hunted", next, null]),
        "{result}"
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(result["record"], head.as_str(), "{result}");
    assert_eq!(
        sim.rig.rev("HEAD~1"),
        candidate,
        "the record is ONE commit, on the candidate"
    );
    assert_eq!(
        sim.rig.remote(LOOP).as_deref(),
        Some(head.as_str()),
        "the record is pushed"
    );
    assert_eq!(sim.rig.status(), "", "nothing is left pending");
    assert_eq!(head_commit(sim).0, first_record_subject());
    let agents = ran.agents();
    for other in ["triage:p", "verify:", "advocate:", "proposal:", "record:"] {
        assert!(
            !agents.iter().any(|label| label.starts_with(other)),
            "`{other}…` is of the other path, an executor, or of a step this path has not built: {agents:?}"
        );
    }
    let gate = format!("record-test-r1-a{attempt}-1");
    assert_eq!(
        agents[agents.len() - 6..],
        [
            "git:compose:test:r1",
            format!("git:hold-start:{gate}").as_str(),
            format!("git:hold-wait:{gate}").as_str(),
            "git:record:test:r1",
            "git:push",
            "git:state:test-3",
        ],
        "the record is tool steps only: composed, gated, committed, pushed, read back: {agents:?}"
    );
    assert!(
        !ran.commands
            .iter()
            .any(|command| command.starts_with("dev/stabilize-record apply ")),
        "no batch is typed: {:?}",
        ran.commands
    );
    let composed = composing(ran);
    assert_eq!(
        composed.len(),
        1,
        "one step composes the record: {composed:?}"
    );
    let (flags, words) = &composed[0];
    assert!(
        flags.contains(&format!(
            " --branch {LOOP} --run-dir {RUN_DIR} --round 1 --attempt {attempt} --subject '{}' --sha256 ",
            first_record_subject()
        )),
        "{flags}"
    );
    assert_eq!(
        flag_of(flags, "--sha256"),
        sha256(&canonical(1, attempt, &first_record_subject(), words)),
        "the hash the harness computed is the sha256 of the words' canonical form: {words:?}"
    );
    words.clone()
}

/// The opening of the fullest scenario of the new path: an audit area that finds things, the
/// gate and the regression set as held checks, one row the opening seeded and nobody graded,
/// and a declared bound that row is graded under.
fn held_and_seeded() -> Opening {
    Opening {
        clauses: vec!["audit-clean", "working-product", "adoptable"],
        items: json!([
            item("area-a", "audit-area", "audit-clean"),
            item("gate", "held-gate", "working-product"),
            item("regression-set", "held-regression", "working-product"),
        ]),
        bounds: vec!["windows"],
        rows: json!([{
            "key": SEEDED, "doctype": "jigc-feedback", "round": 0,
            "source": "known at the opening", "door": "jigc setup", "clause": "adoptable",
            "repro": "the opening record", "kind": "behaviour", "origin": "seed",
            "commands": "jigc setup, on a machine nobody here has",
        }]),
    }
}

const ROUTE: &str = "doc-set-prints-a-dead-route";
const HUNCH: &str = "doc-set-may-drop-a-slot";

/// **A red held check is graded by the stage's one triage, under its own key** (the reshape's
/// plan, `T5a`; its review's `B1`: *a row that appears after the one triage has no grader* —
/// the record call files a red result as `red-<item>`, ungraded, in the batch that follows
/// triage). IN A RUN WITH A QUEUE the one triage is handed EVERY SOURCE KIND, each finding
/// under a number: what the reporters returned, what the scope step left open, THE RED RESULT
/// AS A SOURCE UNDER THE KEY ITS ROW IS FILED WITH — graded from the tool's verdict, by its
/// file — and the rows nobody graded, by the read that prints them. It hands its entries over
/// as ONE FILE, by the call the prompt spells whole, and its return holds a path, a hash and
/// the counts: no entry passes through the harness.
#[test]
fn a_red_held_check_is_graded_by_the_stages_one_triage_under_its_own_key() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-red-held", &held_and_seeded());
    let candidate = sim.rig.rev("HEAD");
    let scratch = sim.rig.scratch.display().to_string();
    let mut route = keyed(ROUTE, "route", "breaks", json!({"area-a-review": [1]}));
    route["route"] = json!("Run: jigc doc set --repair");
    route["code"] = json!("doc.set-slot");
    // A MERGE: the reviewer's second finding and what the scope step left open are one entry.
    let hunch = keyed(
        HUNCH,
        "lead",
        "unclear",
        json!({"area-a-review": [2], "scope": [1]}),
    );
    let mut seeded = found_again(SEEDED, "out-of-scope", json!({"ledger": [1]}));
    seeded["bound"] = json!("windows");
    let mut scope = one_door();
    scope["left_open"] = json!(["a slot may be dropped where nobody listed a door"]);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({
            "held": {"regression-set": {"status": "red"}},
            "scope": scope,
            "agents": {
                "area-a:review": found(&[
                    finding("s-1", "a refusal prints a route that leads nowhere", "jigc doc set", "audit-clean"),
                    finding("s-2", "a slot may be dropped", "jigc doc set", "audit-clean"),
                ]),
                "triage": {"entries": [
                    route,
                    hunch,
                    found_again("red-regression-set", "breaks", json!({"red-regression-set": [1]})),
                    seeded,
                ]},
            },
        }),
    );

    // (1) THE PROMPT, WHOLE IN WHAT IT HANDS OVER: every source under its name, every finding
    // under its number, and the call with one word a source.
    let prompt = one_triage(&ran);
    let entries_file = format!("{scratch}/triage/r1.a1.json");
    let call = format!(
        "`dev/stabilize-record triage --run {RUN} --round 1 --reporter triage --attempt 1 --scratch {scratch} --from {entries_file} -- area-a-review=2 scope=1 red-regression-set=1 ledger=1`"
    );
    assert!(
        prompt.contains(&call),
        "the call, spelled whole ({call}):\n{prompt}"
    );
    assert!(
        prompt.contains("Grade every finding below — 5 in all, from 4 source(s)."),
        "{prompt}"
    );
    let reviewer = handed_under(prompt, "area-a-review");
    assert_eq!(reviewer.len(), 3, "{reviewer:?}");
    assert!(
        reviewer[0].contains(&format!("`{RUN_DIR}/r1/reports/test/area-a-review.a1.md`"))
            && reviewer[1].starts_with("   1. s-1 — a refusal prints a route that leads nowhere")
            && reviewer[2].starts_with("   2. s-2 — a slot may be dropped"),
        "a reporter's findings, numbered from 1 under its name: {reviewer:?}"
    );
    let left_open = handed_under(prompt, "scope");
    assert!(
        left_open.len() == 2
            && left_open[1].starts_with("   1. left open 1 — a slot may be dropped"),
        "what the scope step left open, numbered: {left_open:?}"
    );
    // THE RED RESULT: a source under the key its row will be filed with, and no reporter —
    // its evidence the file the tool kept its verdict in, which is there.
    let red = handed_under(prompt, "red-regression-set");
    let verdict = format!(
        "{scratch}/hold/{}/verdict.json",
        held_check("regression-set", &candidate, 1)
    );
    assert_eq!(red.len(), 2, "{red:?}");
    assert!(
        red[0].contains("A RED RESULT")
            && red[0].contains("under the key `red-regression-set`")
            && red[0].contains(&format!("`{verdict}`"))
            && red[1].starts_with("   1. the check `regression-set` is red")
            && red[1].contains("clause: working-product"),
        "{red:?}"
    );
    let kept: Value = serde_json::from_str(
        fs::read_to_string(&verdict)
            .unwrap_or_else(|e| panic!("read {verdict}: {e}"))
            .trim_end(),
    )
    .expect("the tool's verdict is one line of JSON");
    assert_eq!(
        kept["verdict"], "red",
        "the evidence the prompt names: {kept}"
    );
    // THE ROWS NOBODY GRADED: the read, and none of them — no cell of the seeded row is in
    // the prompt, and the read the prompt names printed it.
    let ledger = handed_under(prompt, "ledger");
    assert!(
        ledger.len() == 1
            && ledger[0].contains(&format!("`dev/stabilize-record untriaged --run {RUN}`"))
            && ledger[0].contains("NUMBERED 1 TO 1 IN THE ORDER THAT READ PRINTS THEM"),
        "{ledger:?}"
    );
    for cell in [
        "jigc setup",
        "known at the opening",
        "a machine nobody here has",
    ] {
        assert!(
            !prompt.contains(cell),
            "`{cell}` is text of a ledger row: {prompt}"
        );
    }
    let (_, _, printed) = ran
        .reads
        .iter()
        .find(|(label, command, _)| label == "triage" && command.contains(" untriaged "))
        .expect("the triage ran the read its prompt names");
    let printed: Value = serde_json::from_str(printed).expect("one line of JSON");
    assert_eq!(
        json!([printed["count"], printed["untriaged"][0]["key"]]),
        json!([1, SEEDED])
    );
    // WHAT THE RETURN IS HELD TO is said, and the report line is a halt's road and no other.
    assert!(
        prompt.contains("WHAT YOUR RETURN IS HELD TO")
            && prompt.contains(&format!("`{}`", rendering(1, 1)))
            && prompt
                .lines()
                .filter(|line| line.starts_with("REPORT: "))
                .count()
                == 1
            && prompt.contains("ONLY WHERE YOU HALT"),
        "{prompt}"
    );

    // (2) WHAT WAS HANDED OVER: one file, and the report rendered from it — the red result
    // under its own key, FOUND AGAIN, with the class its row will be filed with.
    let written: Value = serde_json::from_str(
        &fs::read_to_string(&entries_file).unwrap_or_else(|e| panic!("read {entries_file}: {e}")),
    )
    .expect("the entries are one JSON array");
    assert_eq!(written.as_array().map(Vec::len), Some(4));
    let data = sim.rendered(1, 1);
    assert_eq!(
        data["handed"],
        json!({"area-a-review": 2, "scope": 1, "red-regression-set": 1, "ledger": 1})
    );
    assert_eq!(
        json!([
            data["entries"][2]["key"],
            data["entries"][2]["found"],
            data["entries"][2]["grade"],
            data["entries"][2]["folds"],
            data["entries"][1]["folds"]
        ]),
        json!([
            "red-regression-set",
            "again",
            "breaks",
            ["red-regression-set 1"],
            ["area-a-review 2", "scope 1"]
        ]),
        "{data}"
    );
    let line = filed_line(&ran);
    assert_eq!(
        json!([
            line["findings"],
            line["counts"]["entries"],
            line["entries"][2]
        ]),
        json!([5, 4, {"key": "red-regression-set", "found": "again", "grade": "breaks",
                      "class": "behaviour hunt"}]),
        "{line}"
    );

    // (3) THE STAGE ENDS WHERE THE PATH IS BUILT TO: at its first record — the red result's
    // row filed and graded by one batch, under its own key, beside the rows the triage keyed.
    ends_at_its_first_record(&sim, &ran, &candidate, 1, "verify");
    assert_eq!(ran.result["counts"]["findings_in"], 5, "{}", ran.result);
    let state = sim.state();
    assert_eq!(
        sorted(
            state["ledger"]
                .as_array()
                .expect("the ledger")
                .iter()
                .map(|row| format!("{} {}", text(&row["key"]), text(&row["grade"])))
                .collect()
        ),
        sorted(vec![
            format!("{HUNCH} unclear"),
            format!("{ROUTE} breaks"),
            "red-regression-set breaks".to_owned(),
            format!("{SEEDED} out-of-scope"),
        ]),
        "{state}"
    );
    assert!(
        sim.reports(1).contains(&"triage.a1.md".to_owned())
            && sorted(ran.reporters.clone())
                == sorted(
                    sim.reports(1)
                        .iter()
                        .map(|name| name.trim_end_matches(".a1.md").to_owned())
                        .collect()
                ),
        "one report per launched reporter, the triage's rendered one among them: {:?}",
        sim.reports(1)
    );
}

/// **A red check that is not held is graded too, and its evidence is the preflight's report**
/// (`T5a`; `B1`'s residual: *a check the preflight reads has no verdict file*). Each red check
/// is a source under its own key, graded from THE REPORT OF THE PREFLIGHT THAT READ IT — the
/// first preflight's for a check of every candidate, the second's for one the scope selected —
/// named by its path: no word of what the preflight returned is in the prompt. AND THE STAGE
/// STOPPED AFTER ITS TRIAGE returns the rendering by its path and its hash, and no entry.
#[test]
fn a_red_check_that_is_not_held_is_graded_too_and_its_evidence_is_the_preflights_report() {
    if !can_run() {
        return;
    }
    let opening = Opening {
        clauses: vec!["gate-green", "audit-clean"],
        items: json!([
            item("gate", "check", "gate-green"),
            in_scope("ci", "check", "gate-green", &["jigc doc set"], &[]),
            item("area-a", "audit-area", "audit-clean"),
        ]),
        bounds: vec![],
        rows: json!([]),
    };
    let sim = Sim::queued("queue-red-check", &opening);
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({"stopAfter": "triage"})),
        json!({
            "checks": {"gate": "red", "ci": "red"},
            "scope": one_door(),
            "agents": {
                "area-a:review": nothing(),
                "triage": {"entries": [
                    found_again("red-gate", "unclear", json!({"red-gate": [1]})),
                    found_again("red-ci", "breaks", json!({"red-ci": [1]})),
                ]},
            },
        }),
    );
    let prompt = one_triage(&ran);
    assert!(
        prompt.contains(" -- red-gate=1 red-ci=1`")
            && prompt.contains("Grade every finding below — 2 in all, from 2 source(s)."),
        "a reporter that found nothing is no source, and each red check is one:\n{prompt}"
    );
    for (key, check, preflight) in [
        ("red-gate", "gate", "preflight"),
        ("red-ci", "ci", "preflight-second"),
    ] {
        let red = handed_under(prompt, key);
        let report = format!("{RUN_DIR}/r1/reports/test/{preflight}.a1.md");
        assert!(
            red.len() == 2
                && red[0].contains("A RED RESULT")
                && red[0].contains(&format!("under the key `{key}`"))
                && red[0].contains(&format!(
                    "the report of the preflight that read it red, `{report}`"
                ))
                && red[1].starts_with(&format!("   1. the check `{check}` is red"))
                && red[1].contains("clause: gate-green"),
            "{red:?}"
        );
        assert!(
            sim.rig.root.join(&report).is_file(),
            "`{report}`, the evidence the prompt names, is there"
        );
    }
    assert!(
        !prompt.contains("scripted by the simulation") && !prompt.contains("verdict.json"),
        "no word of what a preflight returned is in the prompt, and no verdict file is named for a check no tool held:\n{prompt}"
    );
    let data = sim.rendered(1, 1);
    assert_eq!(
        data["entries"]
            .as_array()
            .expect("the entries")
            .iter()
            .map(|entry| json!([entry["key"], entry["found"], entry["grade"], entry["folds"]]))
            .collect::<Vec<_>>(),
        [
            json!(["red-gate", "again", "unclear", ["red-gate 1"]]),
            json!(["red-ci", "again", "breaks", ["red-ci 1"]]),
        ],
        "{data}"
    );

    // THE STOP AFTER THE TRIAGE — the tuning stop, on this path: the rendering by its path
    // and its hash, how many findings were handed, and no entry.
    let result = &ran.result;
    let on_disk = sim.rig.read(&rendering(1, 1));
    assert_eq!(
        json!([
            result["status"],
            result["after"],
            result["triage"],
            result["findings_in"],
            result["entries"]
        ]),
        json!([
            "stopped",
            "triage",
            {"reporter": "triage", "report": rendering(1, 1), "sha256": sha256(&on_disk)},
            2,
            null
        ]),
        "{result}"
    );
    assert_eq!(
        sim.rig.rev("HEAD"),
        candidate,
        "a stopped stage records nothing"
    );
}

/// **Triage that leaves a handed finding in no entry, or in two, halts by name** (`T5a`; the
/// arithmetic half of `wf-triage-reads-harness`). THE PARTITION IS THE RECORD SCRIPT'S, held
/// to the counts of the call — so three things are shown. A file that leaves a finding out
/// and folds another twice is REFUSED, NAMING EACH BY ITS SOURCE AND ITS NUMBER, nothing is
/// written, and a triage that does not correct it halts the stage with that line. A triage
/// that corrects the file hands the same attempt over again, and the stage goes on. And A
/// CALL TYPED WITH OTHER COUNTS THAN THE PROMPT SPELLS holds another partition: the harness
/// holds what the call printed of each source to what it handed, and halts by the source's
/// name.
#[test]
fn triage_that_leaves_a_handed_finding_in_no_entry_or_in_two_halts_by_name() {
    if !can_run() {
        return;
    }
    let two = || {
        found(&[
            finding("s-1", "one thing", "jigc doc set", "audit-clean"),
            finding("s-2", "another", "jigc doc set", "audit-clean"),
        ])
    };
    let once = |key: &str, numbers: Value| {
        let mut entry = keyed(
            key,
            "behaviour",
            "breaks",
            json!({"area-a-review": numbers}),
        );
        entry["commands"] = json!("jigc doc set a b");
        entry
    };
    let script =
        |triage: Value| small_stage(json!({"agents": {"area-a:review": two(), "triage": triage}}));

    // (1) IN NO ENTRY, AND IN TWO: refused by the script, by name — and the triage halts.
    let sim = Sim::queued("queue-partition", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        script(json!({"entries": [once("first", json!([1])), once("second", json!([1]))]})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["not_built"]
        ]),
        json!(["halted", "triage", null]),
        "{result}"
    );
    let evidence = text(&result["halted"]["halt"]["evidence"]);
    assert!(
        evidence.contains("refused bad-value")
            && evidence.contains("in no entry: area-a-review 2")
            && evidence.contains("area-a-review 1 (entries 1 and 2)"),
        "the refusal names each finding by its source and its number: {evidence}"
    );
    assert!(
        text(&result["halted"]["reason"]).starts_with("triage halted: "),
        "{result}"
    );
    // NOTHING WAS WRITTEN by the refused call: what lies at the report's path is the halt's
    // report, written by the call of the `REPORT:` line — and no rendering.
    let left = sim.rig.read(&rendering(1, 1));
    assert!(
        !left.starts_with(RENDERED) && left.contains("Halted: the entries were refused"),
        "{left}"
    );
    assert_eq!(sim.rig.rev("HEAD"), candidate);

    // (2) THE FILE CORRECTED, THE SAME CALL MADE AGAIN: the same attempt is handed over, and
    // the stage goes on to its first record.
    let sim = Sim::queued("queue-partition-corrected", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        script(json!({
            "entries": [once("first", json!([1]))],
            "then": [once("first", json!([1])), once("second", json!([2]))],
        })),
    );
    ends_at_its_first_record(&sim, &ran, &candidate, 1, "verify");
    let calls: Vec<&String> = ran
        .commands
        .iter()
        .filter(|command| command.starts_with("dev/stabilize-record triage "))
        .collect();
    assert!(
        calls.len() == 2 && calls[0] == calls[1] && calls[0].ends_with(" -- area-a-review=2"),
        "one call, made twice as it stands: {calls:?}"
    );
    assert_eq!(
        sim.rendered(1, 1)["entries"].as_array().map(Vec::len),
        Some(2),
        "the rendering is of the corrected file"
    );

    // (3) A CALL TYPED WITH ANOTHER COUNT: the script's partition holds — for the one finding
    // the call names — and a finding is in no entry. The harness halts by the source's name.
    let sim = Sim::queued("queue-partition-counts", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        script(json!({"entries": [once("first", json!([1]))], "counts": {"area-a-review": 1}})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["not_built"]
        ]),
        json!(["halted", "triage", null]),
        "{result}"
    );
    let reason = text(&result["halted"]["reason"]);
    assert!(
        reason.contains("`area-a-review`: handed 2, and the call was typed with 1"),
        "the source, by name, with both counts: {reason}"
    );
    assert!(
        sim.rig.read(&rendering(1, 1)).starts_with(RENDERED),
        "the script took the call it was typed: its rendering lies there, uncommitted, and the attempt is spent"
    );

    // (4) AND WHETHER THE RENDERED ENTRIES ARE THERE IS THE REPORT CHECK'S WORD, never the
    // return's: a rendering a hand wrote in is not the text its entries render — the check
    // sets it aside, the triage has left no report, and the stage halts there.
    let sim = Sim::queued("queue-partition-overwritten", &one_area());
    let mut overwritten = script(json!({"entries": [once("first", json!([1, 2]))]}));
    overwritten["overwrites"] = json!({"triage": format!(
        "{RENDERED}\n# Round 1 — the triage, as a hand left it\n\n<!-- end of report -->\n"
    )});
    let ran = sim.invoke(sim.args("test", json!({})), overwritten);
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["not_built"]
        ]),
        json!(["halted", "triage", null]),
        "{result}"
    );
    let reason = text(&result["halted"]["reason"]);
    assert!(
        reason.contains("the report check finds no report of it there")
            && reason.contains("was set aside"),
        "{reason}"
    );
    assert!(
        !sim.reports(1).contains(&"triage.a1.md".to_owned()),
        "what was no rendering has left the tree: {:?}",
        sim.reports(1)
    );
}

/// **A stage with nothing to grade launches no triage** — of the builder's naming (`T5a`),
/// turned by `T5b`. Where no source hands a finding — the instruments found nothing, nothing
/// came back red, and the ledger holds no row nobody graded — there is no call to spell and no
/// entry to hand over: the one triage is not launched, AND THE STAGE STILL RECORDS — what each
/// item did, the candidate's gate and the round's facts — from the words alone.
#[test]
fn a_stage_with_nothing_to_grade_launches_no_triage_on_the_new_path() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-nothing", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    let words = ends_at_its_first_record(&sim, &ran, &candidate, 1, "stop");
    let result = &ran.result;
    assert_eq!(
        json!([result["triage"], result["counts"]["findings_in"]]),
        json!([null, 0]),
        "{result}"
    );
    let agents = ran.agents();
    assert!(
        !agents.iter().any(|label| label.starts_with("triage"))
            && agents[agents.len() - 7] == "git:check-reports",
        "no triage of either path is launched: the record follows the instruments' report check: {agents:?}"
    );
    assert!(
        !words.contains(&"reporter=triage".to_owned())
            && words.contains(&"item=area-a:green".to_owned()),
        "{words:?}"
    );
    let state = sim.state();
    assert_eq!(
        json!([state["ledger"], standing(&state, "area-a")]),
        json!([[], ["green", null]]),
        "{state}"
    );
}

/// **An entry without its kind's evidence is recorded as a lead** (`T5a`; `T1`: *a kind
/// without its evidence is a lead*; the definition: *the file is taken once: the evidence is
/// in the file before you hand it over*). The filing takes it, and the report it renders
/// holds the entry AS THE ROW IT WILL BE — a lead, with what it was given as — while an entry
/// that brings its kind's evidence keeps its kind. The stage does not halt on it and does not
/// read it: what the harness holds of the triage is the path, the hash and the counts.
#[test]
fn an_entry_without_its_kinds_evidence_is_recorded_as_a_lead() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-lead", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let mut evident = keyed(
        "a-route-with-its-line",
        "route",
        "breaks",
        json!({"area-a-review": [1]}),
    );
    evident["route"] = json!("Run: jigc doc set --force");
    // Given as a route, and no route line: the reporter described the route and quoted none.
    let bare = keyed(
        "a-route-nobody-quoted",
        "route",
        "breaks",
        json!({"area-a-review": [2]}),
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": found(&[
                finding("s-1", "a route that is quoted", "jigc doc set", "audit-clean"),
                finding("s-2", "a route that is described", "jigc doc set", "audit-clean"),
            ]),
            "triage": {"entries": [evident, bare]},
        }})),
    );
    ends_at_its_first_record(&sim, &ran, &candidate, 1, "verify");
    let data = sim.rendered(1, 1);
    assert_eq!(
        data["entries"]
            .as_array()
            .expect("the entries")
            .iter()
            .map(|entry| json!([
                entry["key"],
                entry["kind"],
                entry["given"],
                entry["evidence"]
            ]))
            .collect::<Vec<_>>(),
        [
            json!([
                "a-route-with-its-line",
                "route",
                null,
                "Run: jigc doc set --force"
            ]),
            json!(["a-route-nobody-quoted", "lead", "route", null]),
        ],
        "{data}"
    );
    let line = filed_line(&ran);
    assert_eq!(
        json!([line["leads"], line["counts"]["classes"]]),
        json!([["a-route-nobody-quoted"], {"route hunt": 1, "lead hunt": 1}]),
        "the call says which key it entered as a lead: {line}"
    );
    // NO ENTRY PASSED THROUGH THE HARNESS: its return names no key, no grade and no kind.
    let returned = ran.result.to_string();
    for of_an_entry in [
        "a-route-nobody-quoted",
        "a-route-with-its-line",
        "\"lead\"",
        "Run: jigc doc set",
    ] {
        assert!(
            !returned.contains(of_an_entry),
            "`{of_an_entry}` is of an entry, and the harness's return holds it: {returned}"
        );
    }
}

/// A reviewer's one finding, and the entry a triage keys it under: a behaviour, graded a
/// break, with its commands — so that the round's queue holds a row and `next` is `verify`.
fn one_finding() -> Value {
    found(&[finding(
        "s-1",
        "a slot is dropped",
        "jigc doc set",
        "audit-clean",
    )])
}

fn one_entry() -> Value {
    let mut entry = keyed(INSIDE, "behaviour", "breaks", json!({"area-a-review": [1]}));
    entry["commands"] = json!("jigc doc set a b");
    json!({"entries": [entry]})
}

/// **A stage records its findings before anything is verified** (the reshape's plan, `T5b`;
/// ruling 1 of 2026-10-11: *a stage records twice*; the plan's §2.1 — until this task nothing
/// a `test` stage found was on record before its one record at the end, so a stage that died
/// while it verified lost every row). IN A RUN WITH A QUEUE the stage's FIRST RECORD follows
/// its one triage: the reports checked, the candidate's gate, a result an item, every new
/// entry as its row with its class, every entry's grade, the round's facts — composed BY THE
/// TOOL from what was filed and from PLAIN WORDS, in the order the stage means them. A held
/// check's word is its job; a reporter's word is a reporter that left a report. The stage
/// returns `hunted`, `next` is `verify`, and NOTHING IS VERIFIED: no verifier was launched,
/// no row holds a verdict, and the budget is unspent.
#[test]
fn a_stage_records_its_findings_before_anything_is_verified() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-first-record", &held_and_seeded());
    let candidate = sim.rig.rev("HEAD");
    let previous = sim.rig.rev("main");
    let (gate, regression) = (
        candidates_gate(&candidate),
        held_check("regression-set", &candidate, 1),
    );
    let mut route = keyed(ROUTE, "route", "breaks", json!({"area-a-review": [1]}));
    route["route"] = json!("Run: jigc doc set --repair");
    route["code"] = json!("doc.set-slot");
    let hunch = keyed(HUNCH, "lead", "unclear", json!({"area-a-review": [2]}));
    let mut seeded = found_again(SEEDED, "out-of-scope", json!({"ledger": [1]}));
    seeded["bound"] = json!("windows");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({
            "held": {"regression-set": {"status": "red"}},
            "scope": one_door(),
            "agents": {
                "area-a:review": found(&[
                    finding("s-1", "a refusal prints a route that leads nowhere", "jigc doc set", "audit-clean"),
                    finding("s-2", "a slot may be dropped", "jigc doc set", "audit-clean"),
                ]),
                "triage": {"entries": [
                    route,
                    hunch,
                    found_again("red-regression-set", "breaks", json!({"red-regression-set": [1]})),
                    seeded,
                ]},
            },
        }),
    );
    let words = ends_at_its_first_record(&sim, &ran, &candidate, 1, "verify");

    // (1) THE WORDS, WHOLE AND IN ORDER: the reporters that left a report, as they were
    // launched; the commit; the gate's job; what each item did — a held check by its job,
    // whatever its verdict; the round's facts. No row, no entry, no grade is a word.
    let binary = text(&ran.result["candidate"]["sha256"]);
    assert_eq!(
        words,
        [
            "reporter=attempt".to_owned(),
            "reporter=scope".to_owned(),
            "reporter=preflight".to_owned(),
            "reporter=area-a-review".to_owned(),
            "reporter=triage".to_owned(),
            format!("commit={candidate}"),
            format!("gate={gate}"),
            "item=area-a:green".to_owned(),
            format!("item=gate:held:{gate}"),
            format!("item=regression-set:held:{regression}"),
            format!("fact=candidate:{candidate}"),
            format!("fact=binary:{binary}"),
            format!("fact=base:{previous}"),
        ]
    );

    // (2) WHAT IS ON RECORD, in the one commit: every finding as a row with its grade, the
    // red result's among them under its own key; a result an item; the round's facts; the
    // candidate's gate; the reports; what a stop shows.
    let state = sim.state();
    assert_eq!(
        sorted(
            state["ledger"]
                .as_array()
                .expect("the ledger")
                .iter()
                .map(|row| format!("{} {}", text(&row["key"]), text(&row["grade"])))
                .collect()
        ),
        sorted(vec![
            format!("{HUNCH} unclear"),
            format!("{ROUTE} breaks"),
            "red-regression-set breaks".to_owned(),
            format!("{SEEDED} out-of-scope"),
        ]),
        "{state}"
    );
    assert_eq!(
        json!([
            standing(&state, "area-a"),
            standing(&state, "gate"),
            standing(&state, "regression-set")
        ]),
        json!([["green", null], ["green", null], ["red", null]]),
        "{state}"
    );
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["candidate"],
            state["rounds"][0]["facts"]["binary"],
            state["rounds"][0]["facts"]["base"]
        ]),
        json!([candidate, binary, previous]),
        "{state}"
    );
    let (_, paths) = head_commit(&sim);
    for rel in [
        "ledger.md".to_owned(),
        "r1/gate.md".to_owned(),
        "r1/results.md".to_owned(),
        "r1/triage.md".to_owned(),
        "r1/view.md".to_owned(),
        "r1/reports/test/triage.a1.md".to_owned(),
        "r1/checks/regression-set.a1.json".to_owned(),
    ] {
        assert!(
            paths.contains(&format!("{RUN_DIR}/{rel}")),
            "`{rel}` is in the first record's commit: {paths:?}"
        );
    }

    // (3) NOTHING IS VERIFIED, and the state says what follows: the queue holds the rows the
    // triage sent on, the budget is unspent, and the position is a continuation's.
    assert_eq!(
        json!([
            state["next"],
            state["position"]["test"]["verify"],
            state["budget"]["spent"],
            state["queue"]["ranked"].as_array().map(Vec::len)
        ]),
        json!(["verify", true, 0, 2]),
        "{state}"
    );
    assert!(
        state["ledger"]
            .as_array()
            .expect("the ledger")
            .iter()
            .all(|row| row["verdict"].is_null()),
        "no row holds a verdict: {state}"
    );
    assert!(
        !ran.reporters.iter().any(|name| name.starts_with("verify-")),
        "{:?}",
        ran.reporters
    );
}

/// **A stage killed before its first record holds its green gate and its regression set
/// once** (`T5b`; the plan's `F9` and its review's `S12`: the candidate's build, its gate and
/// every held check were named by label AND ATTEMPT, so a stage that ended before its record
/// ran all of them again — 45 minutes of gate and regression set on the canary's numbers).
/// THE NAMES ARE THE COMMIT'S: the stage that is started again — a later attempt, the same
/// candidate — asks for the names the first one asked for, and is answered by the jobs that
/// stand, green: one start step each, NO WAIT, and no long command runs a second time. Its
/// record names those jobs. *Here the first stage is stopped after its instruments, which is
/// how a test ends a stage before its record: its attempt counts, and nothing is recorded.*
#[test]
fn a_stage_killed_before_its_first_record_holds_its_green_gate_and_its_regression_set_once() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-killed", &with_held_checks(&["regression-set"]));
    let candidate = sim.rig.rev("HEAD");
    let (build, gate, regression) = (
        candidate_build(&candidate),
        candidates_gate(&candidate),
        held_check("regression-set", &candidate, 1),
    );
    let script =
        || small_stage(json!({"agents": {"area-a:review": one_finding(), "triage": one_entry()}}));
    let mut first = script();
    first["agents"]
        .as_object_mut()
        .expect("the agents")
        .remove("triage");
    let stopped = sim.invoke(sim.args("test", json!({"stopAfter": "instruments"})), first);
    assert_eq!(stopped.result["status"], "stopped", "{}", stopped.result);
    assert_eq!(sim.rig.rev("HEAD"), candidate, "nothing was recorded");
    let held_by = |ran: &Ran| -> Vec<String> {
        ran.agents()
            .iter()
            .filter(|label| label.starts_with("git:hold-"))
            .map(|label| (*label).to_owned())
            .collect()
    };
    let previous = previous_build(&sim);
    assert_eq!(
        held_by(&stopped),
        [
            build.as_str(),
            previous.as_str(),
            gate.as_str(),
            regression.as_str()
        ]
        .iter()
        .flat_map(|name| held_steps(name))
        .collect::<Vec<_>>(),
        "the first stage starts each command and waits for it"
    );

    // THE STAGE, STARTED AGAIN: attempt 2 of the round, on the same candidate.
    let again = sim.invoke(sim.args("test", json!({})), script());
    let words = ends_at_its_first_record(&sim, &again, &candidate, 2, "verify");
    let mut expected: Vec<String> = [&build, &previous, &gate, &regression]
        .iter()
        .map(|name| format!("git:hold-start:{name}"))
        .collect();
    expected.extend(held_steps("record-test-r1-a2-1"));
    assert_eq!(
        held_by(&again),
        expected,
        "every job that stands green answers its start, and nobody waits for one"
    );
    let long = sim.long_commands();
    assert_eq!(
        [
            long.iter().filter(|c| *c == "gate --keep-going").count(),
            long.iter()
                .filter(|c| c.starts_with("regression-set "))
                .count(),
            long.iter().filter(|c| c.starts_with("cargo ")).count(),
        ],
        [2, 1, 2],
        "the candidate's gate ran once — the other is the record's — the regression set once, and one build a binary: {long:?}"
    );
    for word in [
        format!("gate={gate}"),
        format!("item=gate:held:{gate}"),
        format!("item=regression-set:held:{regression}"),
    ] {
        assert!(words.contains(&word), "`{word}`: {words:?}");
    }
    let state = sim.state();
    assert_eq!(
        json!([standing(&state, "gate"), standing(&state, "regression-set")]),
        json!([["green", null], ["green", null]]),
        "the record of the second attempt holds the verdicts the first one's jobs left: {state}"
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/gate.md"))
            .contains("pass"),
        "the candidate's gate is a fact of the round"
    );
}

/// **A held check that left no verdict is started under its next name in its re-run**
/// (`T5b`; the plan's second review: *held checks named by commit defeat a re-run* — a start is
/// answered by the job that is there, so an item's automatic re-run, on the round's own
/// candidate, would meet the same dead job for ever). A HELD CHECK IS NAMED BY ITS COMMIT AND
/// THE ITEM'S OWN RUN IN THE ROUND: the regression set whose supervisor was killed is recorded
/// void BY A WORD that names its job, the state asks for its re-run, and the re-run — the
/// item's second run — asks for the next name, starts the command, and records its verdict.
#[test]
fn a_held_check_that_left_no_verdict_is_started_under_its_next_name_in_its_rerun() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-held-rerun", &with_held_checks(&["regression-set"]));
    let candidate = sim.rig.rev("HEAD");
    let scratch = sim.rig.scratch.display().to_string();
    let (first, second) = (
        held_check("regression-set", &candidate, 1),
        held_check("regression-set", &candidate, 2),
    );
    let kill = format!(
        "python3 -c \"import json,os,signal; os.kill(json.load(open('{scratch}/hold/{first}/job.json'))['pid'], signal.SIGKILL)\""
    );
    let dead = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "unreleased": [first],
            "meanwhile": {format!("git:hold-start:{first}"): kill},
        })),
    );
    sim.release(&first);
    let words = ends_at_its_first_record(&sim, &dead, &candidate, 1, "stop");
    assert!(
        words.contains(&format!("item=regression-set:void:dead:{first}")),
        "a held check that left no verdict is void by a word, which names its job: {words:?}"
    );
    let state = sim.state();
    assert_eq!(
        results(&state, 1)
            .into_iter()
            .find(|row| row[0] == "regression-set"),
        Some(json!([
            "regression-set",
            1,
            "void",
            format!("without a verdict: dead:{first}")
        ])),
        "{state}"
    );
    // The run stops after every round: the human's go, and the state asks for the re-run.
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(
        json!([go.result["status"], go.result["next"]]),
        json!(["ruled", "retest"]),
        "{}",
        go.result
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["retest"],
            state["position"]["test"]["rerun"]["items"][0]["item"],
            state["position"]["test"]["rerun"]["items"][0]["attempt"]
        ]),
        json!([["working-product"], "regression-set", 2]),
        "the state asks for the item's second run: {state}"
    );

    // THE RE-RUN: the item's second run in the round, on the round's candidate — under the
    // next name. The dead job is not asked for again.
    let again = sim.invoke(
        sim.args("test", json!({"clause": "working-product"})),
        json!({}),
    );
    assert_eq!(
        json!([again.result["status"], again.result["halted"]]),
        json!(["hunted", null]),
        "{}",
        again.result
    );
    let agents = again.agents();
    assert!(
        held_steps(&second)
            .iter()
            .all(|step| agents.contains(&step.as_str()))
            && !agents.iter().any(|label| label.ends_with(first.as_str())),
        "the second run is started and waited for under its own name: {agents:?}"
    );
    let composed = composing(&again);
    assert_eq!(composed.len(), 1, "{composed:?}");
    assert_eq!(
        composed[0].1,
        [
            "reporter=attempt".to_owned(),
            "reporter=preflight".to_owned(),
            format!("commit={candidate}"),
            format!("item=regression-set:held:{second}"),
        ],
        "a re-run's record: its reporters, the round's candidate and the item's result — no gate of the candidate's, and no fact of the round"
    );
    let state = sim.state();
    assert_eq!(
        json!([standing(&state, "regression-set"), state["retest"]]),
        json!([["green", null], []]),
        "{state}"
    );
    assert_eq!(
        sim.long_commands()
            .iter()
            .filter(|c| c.starts_with("regression-set "))
            .count(),
        2,
        "the regression set ran once a run of its item"
    );
}

/// **A second stage against one previous release names the same baseline** (the reshape's
/// plan, §5, `K8b`; `dev/regression-set`: *A KEPT BASELINE* — what the previous release's
/// commit alone decides is built once in a directory the caller names, and used again while
/// that tool's own key holds). THE STAGE NAMES THE DIRECTORY, and names it by the previous
/// release's commit and by nothing else: `baseline/previous-<12>` below the scratch root it
/// was given — no candidate, no round, no attempt and no job in it. So the regression set of
/// a stage and the one of the stage that follows — here the item's re-run, a held command
/// under ITS OWN NAME, as the names of what a stage holds have it — are handed ONE directory,
/// which lies in neither one's own. What is on record of each is the path below the root as
/// it was asked and the regression tool's own word of what became of the baseline — its
/// directory under the public placeholder, never under this machine's path.
#[test]
fn a_second_stage_against_one_previous_release_names_the_same_baseline() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-baseline", &with_held_checks(&["regression-set"]));
    let candidate = sim.rig.rev("HEAD");
    let previous = sim.rig.rev("main");
    let scratch = sim.rig.scratch.display().to_string();
    let kept_in = format!("baseline/previous-{}", &previous[..12]);
    let key = "7".repeat(64);
    let (first, second) = (
        held_check("regression-set", &candidate, 1),
        held_check("regression-set", &candidate, 2),
    );
    // THE FIRST STAGE: the previous release's baseline is built and kept, and the candidate
    // does not build — a void run of the item, which is due again.
    let voided = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"held": {"regression-set": {
            "status": "void", "reason": "build-candidate", "baseline": {"key": key}}}})),
    );
    let words = ends_at_its_first_record(&sim, &voided, &candidate, 1, "stop");
    assert!(
        words.contains(&format!("item=regression-set:held:{first}")),
        "{words:?}"
    );
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(
        json!([go.result["status"], go.result["next"]]),
        json!(["ruled", "retest"]),
        "{}",
        go.result
    );
    // THE STAGE THAT FOLLOWS: the item's second run, on the round's candidate — a held
    // command of its own, which uses what the first one kept.
    let again = sim.invoke(
        sim.args("test", json!({"clause": "working-product"})),
        json!({"held": {"regression-set": {
            "status": "green", "baseline": {"used": true, "key": key}}}}),
    );
    assert_eq!(
        json!([again.result["status"], again.result["halted"]]),
        json!(["hunted", null]),
        "{}",
        again.result
    );

    // (1) WHAT EACH STAGE TOLD ITS STEP TO RUN: the command of the prompt, as it was typed —
    // the same path below the root, under the name of each run of the item.
    for (ran, name) in [(&voided, &first), (&again, &second)] {
        assert!(
            ran.commands.contains(&format!(
                "dev/stabilize-step hold-start --scratch {scratch} --name {name} --kind regression --previous {previous} --candidate {candidate} --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv --baseline {kept_in} --digest {scratch}"
            )),
            "the stage names the kept baseline of the previous release: {:?}",
            ran.commands
        );
    }
    // (2) WHAT THE REGRESSION SET WAS HANDED, both times: ONE directory, under the scratch
    // root — and a work directory of each job's own, in which the baseline does not lie.
    let runs: Vec<String> = sim
        .long_commands()
        .into_iter()
        .filter(|c| c.starts_with("regression-set run "))
        .collect();
    assert_eq!(
        runs,
        [&first, &second].map(|name| format!(
            "regression-set run --previous {previous} --candidate {candidate} --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv --scratch {scratch}/hold/{name}/work --baseline {scratch}/{kept_in}"
        )),
        "two runs of the regression set, one kept baseline"
    );
    assert!(
        !kept_in.contains(&candidate[..12]) && !kept_in.starts_with("hold/"),
        "the baseline is the previous release's, and lies in no job's directory: {kept_in}"
    );

    // (3) WHAT IS ON RECORD of each run: the path as it was asked, and the regression tool's
    // own word of what became of the baseline — and no path of this machine.
    let real = fs::canonicalize(&sim.rig.scratch)
        .expect("the scratch root")
        .display()
        .to_string();
    for (run, used) in [(1, false), (2, true)] {
        let text = sim
            .rig
            .read(&format!("{RUN_DIR}/r1/checks/regression-set.a{run}.json"));
        let kept: Value = serde_json::from_str(text.trim_end()).expect("the verdict on record");
        assert_eq!(
            json!([
                kept["asked"]["baseline"],
                kept["evidence"]["line"]["baseline"]
            ]),
            json!([kept_in, {"dir": format!("<scratch>/{kept_in}"), "used": used,
                             "refused": null, "detail": null, "key": key}]),
            "run {run}: {kept}"
        );
        assert!(
            !text.contains(&scratch) && !text.contains(&real),
            "run {run}: the record names the scratch root by its placeholder only:\n{text}"
        );
    }
    let state = sim.state();
    assert_eq!(
        json!([standing(&state, "regression-set"), state["retest"]]),
        json!([["green", null], []]),
        "{state}"
    );
}

/// **A gate that stands red is not the answer to a stage started again** (`T5b`: *a job that
/// stands is taken only while it runs or where its verdict is green; one that ended red, void
/// or dead is passed over for the next name, as a record's gate already is — a starved red
/// gate is measured again, never answered for ever*). The candidate's gate of a stage that
/// ended before its record was red. The stage started again meets that job under the commit's
/// name, PASSES IT OVER, and holds the gate under the next name: that one is the candidate's
/// gate of the round, by name, in the record.
#[test]
fn a_gate_that_stands_red_is_not_the_answer_to_a_stage_started_again() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-red-gate", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let gate = candidates_gate(&candidate);
    let stopped = sim.invoke(
        sim.args("test", json!({"stopAfter": "preflight"})),
        json!({"gate": red(&[KNOWN]), "scope": one_door(), "agents": {}}),
    );
    assert_eq!(
        json!([
            stopped.result["status"],
            stopped.result["gate"]["name"],
            stopped.result["gate"]["ends"]
        ]),
        json!(["stopped", gate, "red"]),
        "{}",
        stopped.result
    );

    let again = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {"area-a:review": one_finding(), "triage": one_entry()}})),
    );
    let words = ends_at_its_first_record(&sim, &again, &candidate, 2, "verify");
    let next = format!("{gate}-2");
    let gates: Vec<&str> = again
        .agents()
        .into_iter()
        .filter(|label| label.contains(":gate-"))
        .collect();
    assert_eq!(
        gates,
        [
            format!("git:hold-start:{gate}").as_str(),
            format!("git:hold-start:{next}").as_str(),
            format!("git:hold-wait:{next}").as_str(),
        ],
        "the red job answers its start and is passed over; the next name is started and waited for"
    );
    assert!(
        again
            .logs
            .iter()
            .any(|line| line.contains(&format!("`{gate}`")) && line.contains("passed over")),
        "{:?}",
        again.logs
    );
    assert!(
        words.contains(&format!("gate={next}")) && !words.contains(&format!("gate={gate}")),
        "{words:?}"
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/gate.md"))
            .contains("pass"),
        "the round's gate is the one that was measured again"
    );
    assert_eq!(
        sim.long_commands()
            .iter()
            .filter(|c| *c == "gate --keep-going")
            .count(),
        3,
        "the red gate, the gate that was measured again, and the record's"
    );

    // AND A RED GATE THIS INVOCATION STARTED ITSELF IS ITS OWN ANSWER — also where the agent of
    // its start died and the step, asked again, was answered by the job that was there by
    // then: red is a fact about the candidate, and is passed over only where an EARLIER
    // invocation left it.
    let sim = Sim::queued("queue-red-gate-own", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let gate = candidates_gate(&candidate);
    let own = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "gate": red(&[KNOWN]),
            "record": {"gate": red(&[KNOWN])},
            "endings": {format!("git:hold-start:{gate}"): "acts-and-dies-once"},
        })),
    );
    let words = ends_at_its_first_record(&sim, &own, &candidate, 1, "stop");
    assert!(
        words.contains(&format!("gate={gate}")),
        "the gate this invocation started is the candidate's, red as it is: {words:?}"
    );
    assert!(
        !own.agents()
            .iter()
            .any(|label| label.ends_with(&format!("{gate}-2"))),
        "{:?}",
        own.agents()
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/gate.md"))
            .contains(KNOWN),
        "what the candidate's gate shows red is on record"
    );
}

/// **A build that stands failed is passed over, and its binary is written at a path of its
/// own** — of the builder's naming, beside the plan's (`T5b`: *the two builds are named by
/// commit*; *a job that stands is taken only while it runs or where its verdict is green*). A
/// build is named by its commit, so a build that failed would be the answer to every later
/// stage on that candidate under that scratch root. It is passed over: the stage started
/// again builds the commit under the next name, AT THE PATH OF THAT NAME — a binary is written
/// once — and every driving agent is handed that file.
#[test]
fn a_build_that_stands_failed_is_passed_over_and_its_binary_is_written_at_a_path_of_its_own() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-failed-build", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let build = candidate_build(&candidate);
    let failed = sim.invoke(
        sim.args("test", json!({})),
        json!({"builds": {"fails": ["candidate"]}, "scope": one_door(), "agents": {}}),
    );
    assert_eq!(
        json!([
            failed.result["status"],
            failed.result["halted"]["phase"],
            failed.result["halted"]["held"]["name"],
            failed.result["halted"]["held"]["ends"]
        ]),
        json!(["halted", "build", build, "red"]),
        "{}",
        failed.result
    );

    let again = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    ends_at_its_first_record(&sim, &again, &candidate, 2, "stop");
    let next = format!("{build}-2");
    let builds: Vec<&str> = again
        .agents()
        .into_iter()
        .filter(|label| label.contains(":build-candidate-"))
        .collect();
    assert_eq!(
        builds,
        [
            format!("git:hold-start:{build}").as_str(),
            format!("git:hold-start:{next}").as_str(),
            format!("git:hold-wait:{next}").as_str(),
        ],
        "the failed build answers its start and is passed over; the next name is started and waited for"
    );
    let binary = text(&again.result["candidate"]["binary"]);
    assert_eq!(
        binary,
        format!(
            "{}/bin/candidate-{}-2/jigc",
            sim.rig.scratch.display(),
            &candidate[..12]
        ),
        "the second build's binary lies at a path of its name's own"
    );
    assert_eq!(
        again.result["candidate"]["sha256"],
        sha256_of(&fs::read(&binary).expect("the binary the tool built")).as_str()
    );
    let reviewer = &again
        .prompts
        .iter()
        .find(|(label, _)| label == "area-a:review")
        .expect("the reviewer")
        .1;
    assert!(
        reviewer.contains(&format!("BINARY: candidate `{binary}` sha256 ")),
        "every driving agent is handed that file: {reviewer}"
    );
}

/// **Every void says why in one word** — of the builder's naming, beside the plan's (`T5b`;
/// what `K3` left to it: *every void is a word — the sentences the harness composes today need
/// a word each*; *the reporter words are the reporters that left a report: one the report
/// check found missing is dropped, and its item is `void:<word>:<reporter>`*). ONE STAGE, EIGHT
/// ITEMS, EACH VOID ANOTHER WAY: a step that returned and left no report, a report a hand
/// wrote over, a step that halted, a driver that drove another binary, a trial arm with no
/// image, a check the preflight could not run, a check that ran on another commit, a check
/// the second preflight did not provide. The record takes each as ONE WORD AND A KEY — the
/// reporter it is about — and a reporter that left no report is no word of the record.
#[test]
fn every_void_says_why_in_one_word_and_a_reporter_that_left_no_report_is_no_word() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued(
        "queue-voids",
        &Opening {
            clauses: vec!["adoptable", "gate-green", "audit-clean"],
            items: json!([
                in_scope("arm-a", "trial-arm", "adoptable", &[], &["verbs"]),
                {"item": "tarball", "kind": "check", "clause": "adoptable", "runs": "in-scope",
                 "brief": "the packaged-tarball install", "doors": ["jigc doc set"], "registries": []},
                item("lint", "check", "gate-green"),
                item("gate", "check", "gate-green"),
                item("area-a", "audit-area", "audit-clean"),
                item("area-b", "audit-area", "audit-clean"),
                item("cross", "audit-cross-cutting", "audit-clean"),
                item("drive-a", "audit-drive", "audit-clean"),
            ]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    let candidate = sim.rig.rev("HEAD");
    let previous = sim.rig.rev("main");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({
            "checks": {"lint": "void", "gate": "green"},
            "checkCommits": {"gate": "f".repeat(40)},
            "scope": one_door(),
            "agents": {
                "area-a:review": nothing(),
                "area-b:review": nothing(),
                "cross:review": {"status": "halted", "findings": [],
                    "halt": {"root_cause": "the rig could not be built", "evidence": "scripted",
                             "tree_state": "clean", "recommendation": "none"}},
                "drive-a:drive": nothing(),
            },
            "endings": {"preflight-second": "dies", "area-a:review": "unreported"},
            "overwrites": {"area-b:review": format!(
                "# a review\n\nIt met {DENY_TERM}.\n\n<!-- end of report -->\n"
            )},
            "wrongBinary": ["drive-a:drive"],
        }),
    );
    let words = ends_at_its_first_record(&sim, &ran, &candidate, 1, "stop");
    let binary = text(&ran.result["candidate"]["sha256"]);
    assert_eq!(
        words,
        [
            "reporter=attempt".to_owned(),
            "reporter=scope".to_owned(),
            "reporter=preflight".to_owned(),
            "reporter=cross-review".to_owned(),
            "reporter=drive-a-drive".to_owned(),
            format!("commit={candidate}"),
            format!("gate={}", candidates_gate(&candidate)),
            "item=area-a:void:unreported:area-a-review".to_owned(),
            "item=area-b:void:set-aside:area-b-review".to_owned(),
            "item=cross:void:halted:cross-review".to_owned(),
            "item=drive-a:void:unasserted:drive-a-drive".to_owned(),
            "item=arm-a:void:no-image:preflight-second".to_owned(),
            "item=lint:void:could-not-run:preflight".to_owned(),
            "item=gate:void:other-commit:preflight".to_owned(),
            "item=tarball:void:not-provided:preflight-second".to_owned(),
            format!("fact=candidate:{candidate}"),
            format!("fact=binary:{binary}"),
            format!("fact=base:{previous}"),
        ],
        "each void by its word and the reporter it is about; the three reporters that left no report are no word"
    );
    // ON RECORD, each item is void with that word — no sentence of the harness's, and no
    // word an agent returned, is in the round's results.
    let state = sim.state();
    assert_eq!(
        sorted(
            results(&state, 1)
                .iter()
                .map(|row| format!("{} {} {}", text(&row[0]), text(&row[2]), text(&row[3])))
                .collect()
        ),
        sorted(
            words
                .iter()
                .filter_map(|word| word.strip_prefix("item="))
                .map(|did| {
                    let (item, why) = did.split_once(":void:").expect("every item is void");
                    format!("{item} void {why}")
                })
                .collect()
        ),
        "{state}"
    );
    let recorded = sim.rig.read(&format!("{RUN_DIR}/r1/results.md"));
    for sentence in [
        "scripted by the simulation",
        "returned and left no report",
        "no trial image",
    ] {
        assert!(
            !recorded.contains(sentence),
            "`{sentence}` is a sentence, and the results hold it: {recorded}"
        );
    }
    assert_eq!(
        sorted(
            ran.result["counts"]["voided"]
                .as_array()
                .expect("the void items")
                .iter()
                .map(text)
                .collect()
        ),
        sorted(lines(&[
            "area-a", "area-b", "arm-a", "cross", "drive-a", "gate", "lint", "tarball"
        ])),
        "{}",
        ran.result
    );
}

/// **A continuation is refused on the new path until it is built** (`T5b`; the plan's §4,
/// Phase B: *a step that is not built yet is a refusal of that path, which the task's own
/// scenario asserts — never a refusal of the stage*). After its first record a run with a
/// queue stands at a position this harness has no path for yet: `verify` — a row of the
/// queue is unverified, and budget is left — or `triage` — a row nobody graded. Until both
/// are built (`T6a`, `T6b`) an invocation that meets either IS REFUSED, BY NAME, AFTER ITS TWO
/// READS: no attempt is begun, no agent is launched, and nothing of the other path's lap —
/// which would grade every row again and launch an advocate — runs for such a run.
#[test]
fn a_continuation_is_refused_on_the_new_path_until_it_is_built() {
    if !can_run() {
        return;
    }
    let refused = |sim: &Sim, word: &str| {
        let before = sim.rig.rev("HEAD");
        let position = sim.state()["position"]["test"].clone();
        assert_eq!(
            position[word], true,
            "the position says `{word}`: {position}"
        );
        let ran = sim.invoke(sim.args("test", json!({})), json!({}));
        let result = &ran.result;
        assert_eq!(
            json!([
                result["status"],
                result["not_built"],
                result["next"],
                result["position"]
            ]),
            json!(["refused", "continuation", word, position]),
            "{result}"
        );
        assert_eq!(
            ran.agents(),
            ["git:state", "git:state:test-1"],
            "nothing runs beyond the two reads"
        );
        assert!(
            text(&result["message"]).contains("NOT BUILT YET")
                && text(&result["message"]).contains("Nothing was run beyond the two reads"),
            "{result}"
        );
        assert_eq!(sim.rig.rev("HEAD"), before);
        assert_eq!(
            sim.state()["position"]["test"],
            position,
            "no attempt was begun: the position is what it was"
        );
    };

    // (1) `verify`: the first record left a row in the queue.
    let sim = Sim::queued("queue-continue-verify", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {"area-a:review": one_finding(), "triage": one_entry()}})),
    );
    ends_at_its_first_record(&sim, &ran, &candidate, 1, "verify");
    refused(&sim, "verify");

    // (2) `triage`: a red check the triage folded under a key of its own minting — the record
    // files the red result's row under ITS key, and nobody graded that row.
    let sim = Sim::queued(
        "queue-continue-triage",
        &Opening {
            clauses: vec!["gate-green", "audit-clean"],
            items: json!([
                item("gate", "check", "gate-green"),
                in_scope("ci", "check", "gate-green", &["jigc doc set"], &[]),
                item("area-a", "audit-area", "audit-clean"),
            ]),
            bounds: vec![],
            rows: json!([]),
        },
    );
    let candidate = sim.rig.rev("HEAD");
    let mut minted = keyed(
        "the-gate-is-red",
        "behaviour",
        "no-break",
        json!({"red-gate": [1]}),
    );
    minted["commands"] = json!("dev/gate");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "checks": {"gate": "red", "ci": "red"},
            "agents": {"area-a:review": nothing(), "triage": {"entries": [
                minted,
                found_again("red-ci", "breaks", json!({"red-ci": [1]})),
            ]}},
        })),
    );
    let words = ends_at_its_first_record(&sim, &ran, &candidate, 1, "triage");
    assert!(
        words.contains(&"item=gate:red:preflight".to_owned())
            && words.contains(&"item=ci:red:preflight-second".to_owned()),
        "a check an agent read red names the preflight whose report says so — the second's for a check the scope selected: {words:?}"
    );
    let state = sim.state();
    let filed = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .find(|row| row["key"] == "red-gate")
        .unwrap_or_else(|| panic!("the red result's row is filed: {state}"));
    assert_eq!(
        json!([filed["grade"], filed["repro"]]),
        json!([
            "ungraded",
            format!("{RUN_DIR}/r1/reports/test/preflight.a1.md")
        ]),
        "its evidence is the preflight's report, by path, and nobody graded it: {state}"
    );
    let late = state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .find(|row| row["key"] == "red-ci")
        .unwrap_or_else(|| panic!("the late red check's row is filed: {state}"));
    assert_eq!(
        json!([late["grade"], late["repro"]]),
        json!([
            "breaks",
            format!("{RUN_DIR}/r1/reports/test/preflight-second.a1.md")
        ]),
        "a check the scope selected is filed from the second preflight's report, and graded by the batch that files it: {state}"
    );
    refused(&sim, "triage");
}

/// **A step that types one status wrong is refused by the hash** (the plan's `K3`, its harness
/// half; `T5b`). The composing step is an agent that types forty words: THE HARNESS HASHES THE
/// WORDS' CANONICAL FORM IN JAVASCRIPT, and the record script hashes what the step typed. A
/// step that typed `green` for a void item is REFUSED (`words`) with nothing written — a
/// green that nobody measured never reaches a record — and THE STEP IS ASKED FOR AGAIN,
/// exactly: typed as it stands, the same words compose the record. A step refused so a second
/// time halts the stage at its record, by the record script's own word, with nothing written.
#[test]
fn a_step_that_types_one_status_wrong_is_refused_by_the_hash() {
    if !can_run() {
        return;
    }
    // The reviewer dies: its item is void, by a word that names the reporter that left no
    // report — and that reporter is no word of the record.
    const VOID: &str = "item=area-a:void:silent:area-a-review";
    const GREEN: &str = "item=area-a:green";
    let script = |mistypes: Value| {
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "endings": {"area-a:review": "dies"}, "mistypes": mistypes})
    };

    // (1) ONCE: refused, asked again, typed as it stands, recorded.
    let sim = Sim::queued("queue-words-once", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        script(json!({"git:compose:test:r1": [VOID, GREEN]})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([result["status"], result["halted"]]),
        json!(["hunted", null]),
        "{result}"
    );
    let composed = composing(&ran);
    assert_eq!(composed.len(), 2, "the step ran twice: {composed:?}");
    assert_eq!(
        composed[0].0, composed[1].0,
        "the same call both times: the round, the attempt, the subject and the hash"
    );
    assert!(
        composed[0].1.contains(&GREEN.to_owned())
            && !composed[0].1.contains(&VOID.to_owned())
            && composed[1].1.contains(&VOID.to_owned())
            && !composed[1].1.contains(&"reporter=area-a-review".to_owned()),
        "typed wrong, then as it stands — and a reporter that left no report is no word: {composed:?}"
    );
    assert_eq!(
        flag_of(&composed[1].0, "--sha256"),
        sha256(&canonical(1, 1, &first_record_subject(), &composed[1].1)),
        "the hash is of the words as the harness composed them"
    );
    let agents = ran.agents();
    let at = |label: &str| {
        agents
            .iter()
            .position(|a| *a == label)
            .unwrap_or_else(|| panic!("`{label}` was launched: {agents:?}"))
    };
    assert!(
        at("git:compose:test:r1") + 1 == at("git:compose:test:r1:again1")
            && at("git:compose:test:r1:again1") + 1 == at("git:hold-start:record-test-r1-a1-1"),
        "the step is asked again at once, and the record's gate follows the words that hold: {agents:?}"
    );
    let again = &ran
        .prompts
        .iter()
        .find(|(label, _)| label == "git:compose:test:r1:again1")
        .expect("the step, asked again")
        .1;
    assert!(
        again.contains("typed as it stands") && again.contains("AGAIN (1)"),
        "{again}"
    );
    assert_eq!(
        sim.rig.rev("HEAD~1"),
        candidate,
        "one record, on the candidate"
    );
    let state = sim.state();
    assert_eq!(
        results(&state, 1),
        [json!(["area-a", 1, "void", "silent:area-a-review"])],
        "what is on record is what the harness composed — the item void, by its word: {state}"
    );

    // (2) TWICE: the stage halts at its record, by the record script's own word, and nothing
    // was written — the attempt reached no record.
    let sim = Sim::queued("queue-words-twice", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        script(json!({
            "git:compose:test:r1": [VOID, GREEN],
            "git:compose:test:r1:again1": [VOID, GREEN],
        })),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["refused"],
            result["halted"]["refusal"]
        ]),
        json!(["halted", "record", "record", "words"]),
        "{result}"
    );
    let line = refused_line(result);
    assert_eq!(
        json!([line["act"], line["refused"], line["refusal"]]),
        json!(["compose", "record", "words"]),
        "the halt names the file the step's own line is in: {line}"
    );
    assert_eq!(sim.rig.rev("HEAD"), candidate, "nothing was committed");
    assert!(
        sim.rig.status().lines().all(|line| line.starts_with("?? ")),
        "nothing is applied: {}",
        sim.rig.status()
    );
    assert!(
        !ran.agents()
            .iter()
            .any(|label| label.starts_with("git:hold-start:record-")),
        "no gate is held on a record that was not composed: {:?}",
        ran.agents()
    );
    let state = sim.state();
    assert_eq!(
        json!([state["ledger"], state["position"]["test"]["attempt"]]),
        json!([[], 2]),
        "the next invocation works the next attempt: {state}"
    );
}

// ---------------------------------------------------------------------------
// The runtime probes
// ---------------------------------------------------------------------------

const PROBE: &str = "dev/stabilize-probe";

impl Sim {
    /// A rig that holds the probe tool beside what it copies, committed and pushed. No run
    /// is opened in it — and a probe opens none.
    fn with_probe(label: &str) -> Self {
        let sim = Sim::new(label);
        placed_executable::copy(&repo_root().join(PROBE), &sim.rig.root.join(PROBE));
        sim.rig.commit("chore: the probe tool");
        sim.rig.git(&["push", "-q", "origin", LOOP]);
        sim
    }

    /// A fresh scratch root for one probe: outside the repository, of plain segments, made.
    fn probe_scratch(&self, name: &str) -> String {
        let scratch = self.rig.dir().join(format!("probe-{name}"));
        fs::create_dir_all(&scratch).expect("create a probe's scratch root");
        scratch.display().to_string()
    }

    /// Everything of the repository a probe must leave as it was: the head, the branch,
    /// every ref, what `git status` shows, the remote — and that no run's directory exists.
    fn repository(&self) -> Vec<String> {
        vec![
            self.rig.rev("HEAD"),
            self.rig.branch(),
            self.rig
                .git(&["for-each-ref", "--format=%(refname) %(objectname)"]),
            self.rig.status(),
            format!("{:?} {:?}", self.rig.remote("main"), self.rig.remote(LOOP)),
            format!(
                "a run's directory: {}",
                self.rig.root.join("completions").exists()
            ),
        ]
    }
}

/// What a probe returned as judged: each case and its verdict, in order.
fn judged(ran: &Ran) -> Vec<(String, String)> {
    assert_eq!(ran.result["status"], "probed", "the probe: {}", ran.result);
    let cases = ran.result["cases"]
        .as_array()
        .unwrap_or_else(|| panic!("a probe returns its cases: {}", ran.result));
    let written: Value = serde_json::from_str(
        &fs::read_to_string(ran.result["result"].as_str().expect("the result's file"))
            .expect("read the result the tool wrote"),
    )
    .expect("the result is JSON");
    assert!(
        same_numbers(&written["cases"], &ran.result["cases"]),
        "what the harness returns is what the tool wrote under the scratch root:\n{}\n{}",
        written["cases"],
        ran.result["cases"]
    );
    assert_eq!(
        ran.result["observed"].as_array().map(Vec::len),
        Some(cases.len()),
        "one observation of the harness's own per case: {}",
        ran.result
    );
    cases
        .iter()
        .map(|c| {
            (
                c["case"].as_str().expect("a case").to_owned(),
                c["verdict"].as_str().expect("a verdict").to_owned(),
            )
        })
        .collect()
}

/// Whether two results are the same once A NUMBER IS READ AS ITS VALUE. The tool writes a
/// hold of exactly one second as `1.0`; the runtime the harness returns it through hands
/// on `1` — one number, and two JSON values to a reader that tells an integer from a
/// float.
fn same_numbers(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.as_f64() == right.as_f64(),
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| same_numbers(left, right))
        }
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(name, left)| {
                    right
                        .get(name)
                        .is_some_and(|right| same_numbers(left, right))
                })
        }
        _ => left == right,
    }
}

/// The comparison [`judged`] makes must not depend on how long a hold happened to take: a
/// result with a whole number of seconds in it is the result the runtime hands back.
#[test]
fn a_whole_number_of_seconds_is_the_same_result_however_it_is_written() {
    let written: Value = serde_json::from_str(
        r#"[{"case": "a", "held_for": 1.0, "seconds": 1, "ended": ["returned"]}]"#,
    )
    .expect("what the tool wrote");
    let returned: Value = serde_json::from_str(
        r#"[{"case": "a", "held_for": 1, "seconds": 1, "ended": ["returned"]}]"#,
    )
    .expect("what the runtime handed back");
    assert_ne!(written, returned, "the control: to serde the two differ");
    assert!(
        same_numbers(&written, &returned),
        "`1.0` and `1` are one number"
    );
    for other in [
        r#"[{"case": "a", "held_for": 1.004, "seconds": 1, "ended": ["returned"]}]"#,
        r#"[{"case": "a", "held_for": "1", "seconds": 1, "ended": ["returned"]}]"#,
        r#"[{"case": "a", "held_for": 1, "seconds": 1, "ended": []}]"#,
        r#"[{"case": "a", "held_for": 1, "seconds": 1}]"#,
        r#"[]"#,
    ] {
        let other: Value = serde_json::from_str(other).expect("another result");
        assert!(
            !same_numbers(&written, &other) && !same_numbers(&other, &written),
            "a result that differs still differs: {other}"
        );
    }
}

fn all(cases: &[&str], verdict: &str) -> Vec<(String, String)> {
    cases
        .iter()
        .map(|c| ((*c).to_owned(), verdict.to_owned()))
        .collect()
}

/// A probe's commands are the probe tool's, under its scratch root: none names a run, a
/// round or a branch, and none is a command of the record script or git's. `held` are the
/// commands of the step tool a probe may run beside them — the two plain calls that hold
/// the hold probe's case (a), as a stage holds its gate — and no other probe has any.
fn only_the_probe_tool(ran: &Ran, scratch: &str, held: &[&str]) {
    assert!(!ran.commands.is_empty(), "the probe ran commands");
    let mut by_the_step_tool = Vec::new();
    for command in &ran.commands {
        let shown = command.replace(scratch, "<scratch>");
        if shown.starts_with("dev/stabilize-step ") {
            by_the_step_tool.push(shown);
            continue;
        }
        assert!(
            shown.starts_with("dev/stabilize-probe "),
            "a probe ran a command that is not the probe tool's: {shown}"
        );
        for foreign in [
            "fix/",
            "--run ",
            "--round ",
            "--branch",
            "--loop",
            "stabilize-step",
            "stabilize-record",
            "git ",
        ] {
            assert!(
                !shown.contains(foreign),
                "a probe's command names `{foreign}`: {shown}"
            );
        }
    }
    assert_eq!(
        by_the_step_tool, held,
        "the commands of the step tool a probe ran"
    );
}

#[test]
fn each_runtime_probe_returns_probed_with_one_judged_result_per_case_and_touches_nothing_of_the_repository()
 {
    if !can_run() {
        return;
    }
    let sim = Sim::with_probe("probes");
    let before = sim.repository();
    let git = |label: &str| format!("agent git:probe:{label} · build-git · sonnet · Probe");

    // required — (a) leaves the hash out and the runtime hands the return over as it is;
    // (b) returns the hash of the file it was handed.
    let scratch = sim.probe_scratch("required");
    let ran = sim.invoke(
        json!({"probe": "required", "scratch": scratch}),
        json!({
            "agents": {"probe:required:a": nothing(), "probe:required:b": nothing()},
            "omits": {"probe:required:a": ["asserted_sha256"]},
            "violation": "handed",
        }),
    );
    assert_eq!(
        judged(&ran),
        [("a", "absent"), ("b", "returned")].map(|(c, v)| (c.to_owned(), v.to_owned()))
    );
    assert_eq!(
        ran.trace,
        [
            "phase Probe".to_owned(),
            git("begin"),
            "agent probe:required:a · milestone-code-reviewer · opus · Probe".to_owned(),
            "agent probe:required:b · milestone-code-reviewer · opus · Probe".to_owned(),
            git("verdict"),
        ],
        "the `required` probe: its root, two reviewers on the stage's model, the judgement"
    );
    assert_eq!(ran.result["probe"], "required");
    assert_eq!(ran.result["scratch"], scratch.as_str());
    assert_eq!(
        ran.result["result"],
        format!("{scratch}/probe/required/result.json").as_str()
    );
    only_the_probe_tool(&ran, &scratch, &[]);

    // relay, and relay-document — twelve git steps each, each handed the ONE line of a
    // throwaway run: its digest, or the line with the document in it; one of them relays
    // it with a character changed.
    for probe in ["relay", "relay-document"] {
        let scratch = sim.probe_scratch(probe);
        let ran = sim.invoke(
            json!({"probe": probe, "scratch": scratch}),
            json!({"garbles": ["git:probe:state:r60-2"]}),
        );
        let tags = [
            "r1-1", "r1-2", "r1-3", "r20-1", "r20-2", "r20-3", "r60-1", "r60-2", "r60-3", "r120-1",
            "r120-2", "r120-3",
        ];
        let mut expected = all(&tags, "whole");
        expected[7].1 = "altered".to_owned();
        assert_eq!(judged(&ran), expected);
        let mut steps = vec!["phase Probe".to_owned(), git("begin")];
        steps.extend(tags.iter().map(|tag| git(&format!("state:{tag}"))));
        steps.push(git("verdict"));
        assert_eq!(
            ran.trace, steps,
            "the `{probe}` probe: every step a git step, on its model"
        );
        let altered = &ran.result["cases"][7];
        assert!(
            altered["bytes_back"]
                .as_u64()
                .is_some_and(|n| if probe == "relay" {
                    (2_000..8_000).contains(&n)
                } else {
                    n >= 74_000
                })
                && altered["bytes_back"] == altered["bytes_sent"],
            "the digest of a run of the real run's size came back, or the document itself, one character off: {altered}"
        );
        assert!(
            ran.commands
                .iter()
                .filter(|command| command.contains(" state "))
                .all(|command| command.contains(&format!(" --probe {probe} "))),
            "each relay step names its probe: {:?}",
            ran.commands
        );
        assert!(
            ran.result["observed"][7]["fault"]
                .as_str()
                .is_some_and(|why| why.contains("does not end with the sha256 of itself")),
            "what the harness saw of it: {}",
            ran.result["observed"][7]
        );
        assert!(
            ran.result.to_string().len() < 20_000,
            "a probe returns verdicts and never a relayed line: {} bytes",
            ran.result.to_string().len()
        );
        only_the_probe_tool(&ran, &scratch, &[]);
    }

    // payload — four batches, each written by the executor from its prompt and hashed.
    let scratch = sim.probe_scratch("payload");
    let ran = sim.invoke(json!({"probe": "payload", "scratch": scratch}), json!({}));
    let sizes = ["e20", "e60", "e120", "e300"];
    assert_eq!(judged(&ran), all(&sizes, "whole"));
    let mut steps = vec!["phase Probe".to_owned(), git("begin")];
    steps.extend(
        sizes
            .iter()
            .map(|size| format!("agent probe:payload:{size} · build-executor · opus · Probe")),
    );
    steps.push(git("verdict"));
    assert_eq!(
        ran.trace, steps,
        "the `payload` probe: the role that writes a record"
    );
    for (case, entries) in ran.result["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .zip([20, 60, 120, 300])
    {
        assert_eq!(case["entries"], entries, "{case}");
        assert_eq!(case["relayed"], "agrees", "{case}");
        assert!(
            case["bytes"].as_u64().is_some_and(|n| n > 300 * entries),
            "a batch grows with its entries: {case}"
        );
    }
    only_the_probe_tool(&ran, &scratch, &[]);

    // hold — (a) HELD AS A STAGE HOLDS ITS GATE: one step starts the command through the
    // step tool, and one agent asks after it until it has ended — two plain calls, and
    // the tool's verdict; (b) one step starts it, and a later one reads that it ended.
    let scratch = sim.probe_scratch("hold");
    let ran = sim.invoke(
        json!({"probe": "hold", "scratch": scratch, "seconds": 1}),
        json!({}),
    );
    assert_eq!(
        judged(&ran),
        [("a", "held"), ("b", "outlived")].map(|(c, v)| (c.to_owned(), v.to_owned()))
    );
    assert_eq!(
        ran.trace,
        [
            "phase Probe".to_owned(),
            git("begin"),
            "agent git:hold-start:a · build-git · sonnet · Probe".to_owned(),
            "agent git:hold-wait:a · build-git · sonnet · Probe".to_owned(),
            git("hold:b"),
            git("held:b:1"),
            git("verdict"),
        ],
        "the `hold` probe: a start and a wait through the step tool, then a start and a read through the probe tool"
    );
    assert_eq!(ran.result["cases"][1]["reads"], 1);
    assert_eq!(
        json!([
            ran.result["observed"][0]["line"],
            ran.result["observed"][0]["ends"]
        ]),
        json!(["held", "green"]),
        "case a is the tool's verdict of the held command: {}",
        ran.result
    );
    only_the_probe_tool(
        &ran,
        &scratch,
        &[
            "dev/stabilize-step hold-start --scratch <scratch> --name a --kind probe --seconds 1 --digest <scratch>",
            "dev/stabilize-step hold-wait --scratch <scratch> --name a --digest <scratch>",
        ],
    );

    assert_eq!(
        sim.repository(),
        before,
        "five probes ran, and the repository is not what it was"
    );
}

#[test]
fn a_probe_is_refused_beside_a_stage_or_a_run_and_halts_at_its_first_step_on_a_scratch_root_it_cannot_use()
 {
    if !can_run() {
        return;
    }
    let sim = Sim::with_probe("probe-refusals");
    let before = sim.repository();
    let scratch = sim.probe_scratch("unused");

    // Refused before any agent: an unknown probe, no scratch root, and anything of a stage.
    for args in [
        json!({"probe": "everything", "scratch": scratch}),
        json!({"probe": "relay"}),
        json!({"probe": "relay", "scratch": "relative/dir"}),
        json!({"probe": "relay", "scratch": scratch, "stage": "test"}),
        json!({"probe": "relay", "scratch": scratch, "stage": "fix"}),
        json!({"probe": "relay", "scratch": scratch, "run": RUN}),
        json!({"probe": "payload", "scratch": scratch, "stage": "test", "run": RUN}),
        json!({"probe": "relay", "scratch": scratch, "seconds": 5}),
        json!({"probe": "hold", "scratch": scratch, "seconds": 0}),
        json!({"probe": "hold", "scratch": scratch, "stopAfter": "state"}),
    ] {
        let ran = sim.invoke(args.clone(), json!({}));
        assert_eq!(
            ran.result["status"], "refused",
            "args {args}: {}",
            ran.result
        );
        assert!(
            ran.result["message"]
                .as_str()
                .is_some_and(|message| message.contains("Nothing was run")),
            "args {args}: {}",
            ran.result
        );
        assert_eq!(
            ran.trace,
            Vec::<String>::new(),
            "args {args}: no agent, no phase"
        );
        assert_eq!(
            ran.commands,
            Vec::<String>::new(),
            "args {args}: no command"
        );
    }
    assert_eq!(
        fs::read_dir(&scratch)
            .expect("read the scratch root")
            .count(),
        0,
        "a refused probe made nothing"
    );

    // A scratch root the harness cannot see is no good: the tool says so at the probe's
    // first step, and nothing else is launched. Inside the repository — by a link of plain
    // segments, judged on its real path; not there; no directory.
    let inside = sim.rig.dir().join("a-link-inside");
    std::os::unix::fs::symlink(sim.rig.root.join("crates"), &inside)
        .expect("link into the repository");
    let absent = sim.rig.dir().join("absent");
    let a_file = sim.rig.dir().join("a-file");
    fs::write(&a_file, "no directory\n").expect("write a file");
    for (unusable, word) in [
        (&inside, "inside"),
        (&absent, "no-scratch"),
        (&a_file, "no-scratch"),
    ] {
        for probe in ["required", "relay", "relay-document", "payload", "hold"] {
            let ran = sim.invoke(
                json!({"probe": probe, "scratch": unusable.display().to_string()}),
                json!({}),
            );
            assert_eq!(ran.result["status"], "halted", "{probe}: {}", ran.result);
            assert_eq!(ran.result["probe"], probe);
            assert_eq!(ran.result["halted"]["phase"], "begin", "{}", ran.result);
            assert_eq!(ran.result["halted"]["refused"], word, "{}", ran.result);
            assert_eq!(
                ran.agents(),
                ["git:probe:begin"],
                "{probe} under a scratch root that is {word}: one step, and nothing after it"
            );
        }
    }
    assert!(!absent.exists(), "a refused probe makes no scratch root");
    assert_eq!(
        fs::read_dir(sim.rig.root.join("crates"))
            .expect("read the linked directory")
            .count(),
        1,
        "a probe wrote into the repository through a link"
    );
    assert_eq!(
        sim.repository(),
        before,
        "a refused probe changed the repository"
    );
}

#[test]
fn what_the_runtime_does_with_a_return_that_lacks_a_required_field_is_the_probes_answer() {
    if !can_run() {
        return;
    }
    let sim = Sim::with_probe("probe-required");
    let reviewers = json!({"probe:required:a": nothing(), "probe:required:b": nothing()});
    let both =
        json!({"probe:required:a": ["asserted_sha256"], "probe:required:b": ["asserted_sha256"]});
    let reviewer = |id: &str| format!("probe:required:{id}");

    // NOTHING is handed over, on every try — by both reviewers: each is tried three times,
    // two calls exhaust their retries, and the judgement is still asked for.
    let scratch = sim.probe_scratch("nothing");
    let ran = sim.invoke(
        json!({"probe": "required", "scratch": scratch}),
        json!({"agents": reviewers, "omits": both, "violation": "nothing"}),
    );
    assert_eq!(
        judged(&ran),
        [("a", "nothing"), ("b", "nothing")].map(|(c, v)| (c.to_owned(), v.to_owned()))
    );
    let (a, b) = (reviewer("a"), reviewer("b"));
    assert_eq!(
        ran.agents(),
        [
            "git:probe:begin",
            a.as_str(),
            a.as_str(),
            a.as_str(),
            b.as_str(),
            b.as_str(),
            b.as_str(),
            "git:probe:verdict",
        ],
        "three tries each, and the breaker two exhausted calls trip does not stop the judgement"
    );
    assert_eq!(ran.result["cases"][0]["tries"], 3, "{}", ran.result);
    assert_eq!(
        ran.result["observed"][1]["tries"],
        json!([{"ended": "nothing"}, {"ended": "nothing"}, {"ended": "nothing"}]),
        "how each try ended: {}",
        ran.result
    );
    assert!(
        ran.result["observed"][0]["returned"].is_null(),
        "{}",
        ran.result
    );

    // The call THROWS, on every try — for (a) alone.
    let scratch = sim.probe_scratch("throws");
    let ran = sim.invoke(
        json!({"probe": "required", "scratch": scratch}),
        json!({"agents": reviewers, "omits": {"probe:required:a": ["asserted_sha256"]}, "violation": "throws"}),
    );
    assert_eq!(
        judged(&ran),
        [("a", "threw"), ("b", "returned")].map(|(c, v)| (c.to_owned(), v.to_owned()))
    );
    assert!(
        ran.result["observed"][0]["tries"][2]["message"]
            .as_str()
            .is_some_and(|message| message.contains("lacks `asserted_sha256`")),
        "what a try threw is kept: {}",
        ran.result
    );

    // The field comes back, and is not the file's hash; and a reviewer that halts.
    let scratch = sim.probe_scratch("other");
    let ran = sim.invoke(
        json!({"probe": "required", "scratch": scratch}),
        json!({
            "agents": {
                "probe:required:a": nothing(),
                "probe:required:b": {"status": "halted", "findings": [],
                    "halt": {"root_cause": "the file is no binary"}},
            },
            "wrongBinary": ["probe:required:a"],
        }),
    );
    assert_eq!(
        judged(&ran),
        [("a", "other"), ("b", "halted")].map(|(c, v)| (c.to_owned(), v.to_owned()))
    );
}

// ---------------------------------------------------------------------------
// A machine without node
// ---------------------------------------------------------------------------

/// The suite on a machine without `node`, driven: this binary is run again for one test of
/// this suite, with a `PATH` on which nothing answers.
#[test]
fn without_node_the_suite_fails_under_ci_and_anywhere_else_passes_and_tells_the_gate() {
    let here = RigDir::new("stabilize-sim-no-node");
    let skips = here.path().join("skips");
    let run = |ci: Option<&str>| {
        let mut command = Command::new(std::env::current_exe().expect("this test binary"));
        command
            .args([
                "--exact",
                "stabilize_simulation::a_run_that_was_never_opened_halts_the_stage_at_its_first_read",
                "--nocapture",
            ])
            .env("PATH", here.path().join("nothing-here"))
            .env("JIGC_GATE_SKIPS", &skips)
            .env_remove("CI");
        if let Some(ci) = ci {
            command.env("CI", ci);
        }
        command.output().expect("run this test binary")
    };

    let local = run(None);
    let said = String::from_utf8_lossy(&local.stderr).into_owned();
    assert!(local.status.success(), "without CI the test passes: {said}");
    let skipped = format!(
        "SKIPPED: `node` is not on PATH, so no stage of {HARNESS} was run by the simulation"
    );
    assert!(said.contains(&skipped), "and says so on stderr: {said}");
    assert_eq!(
        fs::read_to_string(&skips).expect("the gate's skip file"),
        format!("{skipped}\n"),
        "and in the file the gate names, which is where somebody sees it"
    );

    let ci = run(Some("true"));
    let said =
        String::from_utf8_lossy(&ci.stdout).into_owned() + &String::from_utf8_lossy(&ci.stderr);
    assert!(!ci.status.success(), "under CI the test fails: {said}");
    assert!(
        said.contains("`node` is not on PATH, and the environment says this is CI"),
        "and says why: {said}"
    );
}
