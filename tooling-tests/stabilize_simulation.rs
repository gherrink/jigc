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
//! `X1`; the re-review's `R-H2`) is driven per cell of the harness's `NEEDS` that voids
//! ([`ABSENT_ARMS`], which the fence holds to that table): the field left out — and, for a
//! hash, returned wrong — voids the item, leaves the finding unverified or the fork off the
//! record, and the stage reaches its record. **One pin is left by that task, and named:**
//! a confirmed verdict whose hash of the previous release's binary is not the one handed
//! over still halts the stage (`L11`, below) — the cell the plan's `X1b` turns, once the
//! record script takes a confirmed verdict with its regression fact unknown.
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
use crate::support::scratch::ScratchDir;

use super::dev_stabilize_record::DENY_TERM;
use super::dev_stabilize_step::{
    GATE_KIND, Held, LOOP, RUN, RUN_DIR, StepRig, as_recorded, failing_commit, harness_pure,
    node_or_skip, sha256,
};
use super::placed_executable;
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

/// A stand-in for a long command a stage holds — `dev/gate`, `dev/regression-set` — placed
/// where the step tool finds the real one. It writes down that it ran, with its pid and
/// its arguments; runs until the suite releases it — a file, never a number of seconds —
/// OR UNTIL THE RIG IS GONE, so that a command a failed test never released does not run
/// on after it; prints what the runtime's stand-in handed the ONE call that started it; and
/// exits as it is told. **The step tool that starts it, waits for it and judges it is real.**
pub(crate) const LONG_COMMAND: &str = r#"#!/bin/sh
printf '%s %s %s\n' "$$" "$(basename "$0")" "$*" >>"$HOLD_RAN"
while [ ! -e "$HOLD_RELEASE" ] && [ -d "${HOLD_RELEASE%/*}" ]; do sleep 0.1; done
cat "$HOLD_PRINTS"
exit "${HOLD_EXIT:-0}"
"#;

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
        self.rig.write(
            &format!("{RUN_DIR}/opening.md"),
            "# the opening record\n\nThe closing condition, the candidate and the test set of a simulated run.\n",
        );
        let previous = self.rig.rev("main");
        self.record(
            &["run-set", "--run", RUN],
            &json!({"stop": "every-round", "previous": PREVIOUS,
                    "previous-commit": previous, "scope": "delta",
                    "clauses": opening.clauses})
            .to_string(),
        );
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
    // The previous release's build is named by its commit — the rig's — which the golden
    // holds as a placeholder: what the file pins is the steps, never a sha of a test rig.
    golden(
        TRACE,
        &format!("{}\n", ran.trace.join("\n"))
            .replace(&previous_build(&sim), "build-previous-<its commit>"),
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
                        "dev/stabilize-step hash --scratch {scratch} --file bin/c1.a1/jigc"
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
            format!("{}/bin/c1.a1/jigc", sim.rig.scratch.display())
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
    agents.extend(held_steps(&format!("build-c1-a{attempt}")));
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
    agents.extend(held_steps(&format!("build-c1-a{attempt}")));
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
    for name in ["build-c1-a1", previous_build(&sim).as_str(), "gate-c1-a1"] {
        launched.extend(held_steps(name));
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
                && ran.commands.iter().any(|command| command
                    .contains(&format!(" --kind build --commit {candidate} --to bin/c1.a"))),
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
    // too: another hash for it is a verifier that drove something else.
    let sim = Sim::opened("wrong-previous", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(),
               "agents": {"area-a:review": review, "triage:p1": graded(std::slice::from_ref(&one), &["area-a-review"]),
                          format!("verify:{key}"): verdict(key, "confirmed")},
               "wrongPrevious": [format!("verify:{key}")]}),
    );
    assert_eq!(ran.result["status"], "halted", "L11: {}", ran.result);
    assert_eq!(
        ran.result["halted"]["phase"], "binary",
        "L11: {}",
        ran.result
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
/// harness's `NEEDS` whose word is not `halts`. `stabilize_harness_fence`, arm (t), holds
/// this list to that table in both directions — so a field a stage comes to need of a
/// role has its arm here, or is named there as owed to a task.
pub(crate) const ABSENT_ARMS: &[(&str, &str)] = &[
    ("review", "asserted_sha256"),
    ("drive", "asserted_sha256"),
    ("verify", "verdict"),
    ("verify", "asserted_sha256"),
    ("advocate", "asserted_sha256"),
    ("proposal", "asserted_sha256"),
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
///
/// The arms of `roles`, and the control they stand beside: three tests share this, so
/// that no one test is a dozen stages long.
fn a_needed_field_left_out_voids_what_its_reporter_was_launched_for(roles: &[&str], fork: bool) {
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
    ];
    assert_eq!(
        arms.iter().map(|arm| (arm.0, arm.1)).collect::<Vec<_>>(),
        ABSENT_ARMS,
        "one arm per pair the fence holds this suite to"
    );
    assert!(
        roles
            .iter()
            .all(|role| ABSENT_ARMS.iter().any(|(of, _)| of == role)),
        "{roles:?} are roles the fence holds an arm for"
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
                let seen = json!([
                    ran.result["status"],
                    ran.result["unverified"],
                    ran.result["forks"].as_array().map(Vec::len),
                    state["ledger"][0]["triage"]["verdict"],
                    state["ledger"][0]["triage"]["fork"]
                ]);
                if seen
                    != json!(["triaged", [], 1, "confirmed", {"kind": "contested", "case": "robust-now", "drive": "holds"}])
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
        arms.iter().filter(|arm| roles.contains(&arm.0))
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
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(&["review", "drive"], false);
}

#[test]
fn a_verifier_that_returns_no_verdict_or_no_hash_leaves_the_finding_unverified_and_the_stage_records()
 {
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(&["verify"], true);
}

#[test]
fn an_advocate_or_an_independent_drive_that_returns_no_hash_is_no_fork_and_the_stage_records() {
    a_needed_field_left_out_voids_what_its_reporter_was_launched_for(
        &["advocate", "proposal"],
        true,
    );
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
        "build-c1-a1",
        previous.as_str(),
        "gate-c1-a1",
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
    assert!(at("preflight") < at("git:hold-start:build-c1-a1"));
    assert!(
        at("git:hold-wait:gate-c1-a1") < at("area-a:review"),
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
        format!("{}/bin/c1.a1/jigc", sim.rig.scratch.display())
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
        at("git:hold-wait:gate-c1-a1") < at("git:hold-start:regression-set-c1-a1"),
        "the regression set runs after the gate, never beside it"
    );
    assert!(at("git:hold-wait:regression-set-c1-a1") < at("area-a:review"));
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
/// supervisor was killed is DEAD: it has no result, the stage goes on and records, and the
/// state asks for it again. A command still running when nobody is left to ask after it
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
    let over = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"endings": {
            "git:hold-wait:gate-c1-a1": "acts-and-dies-once",
            "git:hold-start:build-c1-a1": "acts-and-dies-once",
        }})),
    );
    assert_eq!(over.result["status"], "triaged", "{}", over.result);
    let asked = |label: &str| over.agents().iter().filter(|a| **a == label).count();
    assert_eq!(
        [
            asked("git:hold-start:gate-c1-a1"),
            asked("git:hold-wait:gate-c1-a1")
        ],
        [1, 2],
        "the wait is asked again, and the start is not"
    );
    // A START WHOSE AGENT DIED is asked again too, and answers with the job that is there.
    assert_eq!(asked("git:hold-start:build-c1-a1"), 2);
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
    let kill = format!(
        "python3 -c \"import json,os,signal; os.kill(json.load(open('{scratch_dead}/hold/regression-set-c1-a1/job.json'))['pid'], signal.SIGKILL)\""
    );
    let dead = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "unreleased": ["regression-set-c1-a1"],
            "meanwhile": {"git:hold-start:regression-set-c1-a1": kill},
        })),
    );
    sim.release("regression-set-c1-a1");
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
    let state = sim.state();
    assert_eq!(
        standing(&state, "regression-set"),
        json!(["void", "not-run"]),
        "a dead check has NO result on record: {state}"
    );
    assert_eq!(standing(&state, "gate"), json!(["green", null]));
    assert!(
        !sim.rig
            .root
            .join(format!("{RUN_DIR}/r1/checks/regression-set.a1.json"))
            .exists(),
        "and no verdict's file"
    );

    // LEFT RUNNING: nobody is left to ask after the candidate's gate.
    let sim = Sim::opened("held-running", &one_area());
    let scratch_running = sim.rig.scratch.display().to_string();
    let opened = sim.rig.rev("HEAD");
    let left = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "unreleased": ["gate-c1-a1"],
            "endings": {"git:hold-wait:gate-c1-a1": "dies"},
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
            "gate-c1-a1",
            "gate",
            format!("{scratch_running}/hold/gate-c1-a1/output")
        ]),
        "the halt names the job: {halted}"
    );
    let call =
        format!("`dev/stabilize-step hold-wait --scratch {scratch_running} --name gate-c1-a1`");
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
        "gate-c1-a1",
        "--slice",
        "1",
    ];
    assert_eq!(
        sim.step(&asking)["status"],
        "running",
        "the command runs on"
    );
    sim.release("gate-c1-a1");
    let ended = sim.step(&[
        "hold-wait",
        "--scratch",
        &scratch_running,
        "--name",
        "gate-c1-a1",
    ]);
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
            "dev/stabilize-step hold-start --scratch {scratch} --name regression-set-c1-a2 --kind regression --previous {previous} --candidate {candidate} --list completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv --digest {scratch}"
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
    let here = ScratchDir::new("stabilize-sim-no-node");
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
