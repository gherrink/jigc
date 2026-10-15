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
//! push, as on the other path. **Then the stage VERIFIES, in waves, in the same invocation**
//! (the plan's `T6a`; the scenarios on [`Sim::budgeted`]): the keys the queue prints, read
//! again after every wave, never more than the budget has left. A verifier's stand-in does
//! what its definition has it do where the prompt spells a `dev/stabilize-record verdict`
//! call — the kit's call first, the read that prints its row, its report, then ITS FILING,
//! written to the file that call names and filed by the call as the prompt spells it — so
//! every refusal, every row a filing hands on, the budget and the queue are the real script's;
//! and a scenario can script a verifier that files nothing and says it did, or that returns
//! what its call did not print. **Then the SECOND RECORD**, composed by the tool over the
//! attempt's reporters ([`ends_at_its_second_record`]) — or none, where no verifier left
//! anything ([`ends_at_its_first_record`], [`nobody_verifies`]). **And an invocation that finds
//! such a stage unfinished CONTINUES it** (the plan's `T6b`; [`continuation_opens`]): the
//! state's position says `verify` — a row of the queue is unverified — or `triage` — a row
//! nobody graded is left — or `filed`: a filed verdict is in no record, which is all a stage
//! leaves that died after its last verifier filed, or after the wave that spent its budget
//! ([`dies_after_its_wave_filed`]). It runs no instrument: its reads, its attempt, the two
//! binaries that stand; a triage only over rows nobody graded; the verdicts an earlier
//! invocation filed taken as the state counts them; a verifier only for what has none; and
//! its record.
//! **A verifier call is never tried twice**, and a wave in which two of them return nothing is
//! the last: where the steps after it return nothing too, the stage halts as what may be the
//! account's limit, naming the filed verdicts. *The scenarios of a lap, of an unverified
//! finding and of an advocate in `test` are moved to an opening with an order or retired,
//! each where it stood, with where its cells live now.*
//! **What an invocation of that path returns, and the rulings about its queue** (the plan's
//! `T7`; the row `wf-return-names`): the status is `hunted`, `verified` or `continued`, for
//! what the invocation itself put on record; beside `next` every return says the budget, the
//! queue's length class by class, the hit rate and the stop's view by its path and its hash —
//! each held here to the state document the return names ([`says_what_the_state_says`]) — and
//! what the stage is continued for; and it holds no list of the harness's own making
//! ([`holds_no_list`]). A budget raised, another order and a row put into the queue by key are
//! recorded by the one step that records rulings, start nothing, and are followed by an
//! invocation that continues the stage. **The golden trace is such a stage's** since that
//! task: the scenario that ran a run without an order through a triage pass, an advocate and
//! an independent drive to the return `triaged` is moved, under its name.
//! **What the scoped review before the run in parts found in the harness is driven here**
//! ([DECISIONS.md](../DECISIONS.md) → *2026-10-13 — Three findings of the scoped review,
//! repaired in the harness*): a job that answers a start as ended is never taken for one the
//! invocation must run itself, whatever its start step did — a record's gate two rulings of a
//! round share, a candidate's gate an earlier invocation left red, a name whose start left no
//! record, a build whose binary was replaced; every record a run with a queue composes or
//! applies is preceded, in the same invocation, by the report check — a continuation that owes
//! only a record, and the batch of the human's rulings, each past a file no tool wrote
//! ([`FILED_CHECK`], [`RULINGS_CHECK`]); the halt that may be the account's limit says what is
//! so at each of the seven steps it can be of; and a pending batch whose gate has no name
//! left halts saying so, and the exit it names works.
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
//! **The stand-in refuses what the runtime refuses to load** ([DECISIONS.md](../DECISIONS.md)
//! → *2026-10-15 — The harness a session invokes is generated, without its comments*): a
//! script over 524,288 bytes, with the runtime's own words, before a line of it runs
//! ([`the_runtimes_stand_in_refuses_a_script_over_the_runtimes_limit`]). Until then the
//! stand-in read the harness with no limit, and every arm here passed a harness a session
//! could not start. What this suite runs is the GENERATED harness, the file a session
//! invokes; its commented source is read by none of these arms.
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

use super::dev_regression_set::a_kept_baselines_line;
use super::dev_stabilize_record::{DENY_TERM, STUB_SECRET, gitleaks_stub};
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
/// item the round's doors do not reach — in A RUN WITH A QUEUE: the row its opening seeds
/// carries its class, as every row of such a run does.
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
            "repro": "the opening record", "kind": "behaviour", "origin": "seed",
            "commands": "jigc setup, on a machine nobody here has",
        }]),
    }
}

/// The keys of the four findings the stage grades.
const INSIDE: &str = "doc-set-drops-a-slot";
const OUTSIDE: &str = "rename-loses-a-referrer";
const REFUTED: &str = "doc-set-exit-code";
const SEEDED: &str = "setup-on-windows";

/// What the verifier of the finding inside the round's test set says a settled decision is
/// against it with: the basis of a contested verdict, which is text of its filing alone.
const BASIS: &str = "the decision of the write boundary says that a refusal never writes";

/// What the scripted agents of that stage say: a finding inside the round's test set, whose
/// verifier confirms and CONTESTS it, so that it is the human's; one outside it — a route
/// item, which the queue ranks first; one the verifier refutes; a row seeded at the opening
/// that the one triage grades out of scope under a declared bound — and nothing from every
/// other reporter.
fn every_kind_script() -> Value {
    let keyed_at = |key: &str, kind: &str, grade: &str, door: &str, folds: Value| {
        let mut entry = keyed(key, kind, grade, folds);
        entry["door"] = json!(door);
        entry["clause"] = json!("no-lost-files");
        if kind == "route" {
            entry["route"] = json!(format!("Run: jigc rename --repair {key}"));
        } else {
            entry["commands"] = json!(format!("jigc doc set {key}"));
        }
        entry
    };
    let mut seeded = found_again(SEEDED, "out-of-scope", json!({"ledger": [1]}));
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
            "triage": {"entries": [
                keyed_at(INSIDE, "behaviour", "breaks", "jigc doc set", json!({"row-a-source": [1]})),
                keyed_at(OUTSIDE, "route", "breaks", "jigc rename", json!({"arm-a-score": [1]})),
                keyed_at(REFUTED, "behaviour", "unclear", "jigc doc set", json!({"drive-a-drive": [1]})),
                seeded,
            ]},
            format!("verify:{INSIDE}"): files(json!({"verdict": "confirmed", "regression": false, "contested": BASIS})),
            format!("verify:{OUTSIDE}"): files(json!({"verdict": "confirmed", "regression": false})),
            format!("verify:{REFUTED}"): refutes("does-not-reproduce"),
        },
    })
}

/// **A whole `test` stage of a run with a queue, from the git state to its second pushed
/// record** — THE GOLDEN TRACE (the reshape's plan, `T7`, which moved it: until then it ran a
/// run without an order through a triage pass, three verifiers, an advocate and an independent
/// drive, to ONE record and the return `triaged`). One item of every kind: the preflight, the
/// held builds and the candidate's gate beside the scope step; the instruments; THE ONE
/// TRIAGE; THE FIRST RECORD, composed by the tool; THE VERIFY PHASE — one wave, the queue's
/// three rows in its rank, the route item first — and THE SECOND RECORD. No advocate and no
/// independent drive: the verdict its verifier contests is the human's by that filing. Then
/// what the orchestrator reads, and the invocations that follow: the stage refused while the
/// round is open, and the human's rulings recorded by the one step.
/// The Workflow tool's limit on a script, in bytes, and its words for a script past it
/// (observed 2026-10-13: `Workflow script file <path> exceeds 524288 bytes`).
const RUNTIME_LIMIT: usize = 524_288;

/// THE STAND-IN LOADS WHAT THE RUNTIME LOADS, AND NOTHING LARGER: a script one byte over the
/// limit is refused with the runtime's words, before any of it runs, and the one at the limit
/// runs. *Not established:* that the runtime takes a script of exactly 524,288 bytes — its
/// words say `exceeds`, and this reads them so.
#[test]
fn the_runtimes_stand_in_refuses_a_script_over_the_runtimes_limit() {
    if !can_run() {
        return;
    }
    let dir = RigDir::new("stabilize-runtime-limit");
    let scenario = dir.path().join("scenario.json");
    fs::write(&scenario, "{}").expect("write the scenario");
    let ran = |size: usize| {
        let script = dir.path().join(format!("script-{size}.js"));
        let code = "log('the script ran')\nreturn 'ran'\n// ";
        let padded = format!("{code}{}\n", "x".repeat(size - code.len() - 1));
        assert_eq!(padded.len(), size);
        fs::write(&script, padded).expect("write the script");
        let out = Command::new("node")
            .arg(repo_root().join(RUNTIME))
            .arg(&script)
            .arg(&scenario)
            .stdin(Stdio::null())
            .output()
            .expect("run node");
        let said: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
            panic!(
                "the stand-in prints one JSON line ({e}): {}",
                String::from_utf8_lossy(&out.stdout)
            )
        });
        (out.status.success(), said, script)
    };
    let (ok, said, _) = ran(RUNTIME_LIMIT);
    assert!(
        ok && said["result"] == json!("ran"),
        "a script at the limit runs: {said}"
    );
    let (ok, said, script) = ran(RUNTIME_LIMIT + 1);
    assert!(!ok, "a script over the limit fails the invocation: {said}");
    assert_eq!(
        said["fatal"],
        json!([format!(
            "Workflow script file {} exceeds {RUNTIME_LIMIT} bytes",
            script.display()
        )]),
        "the stand-in refuses with the runtime's words"
    );
    assert_eq!(
        said["logs"],
        json!([]),
        "and before a line of the script ran"
    );
}

#[test]
fn a_test_stage_runs_from_the_git_state_to_the_pushed_record() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("whole", &every_kind(), 30);
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(sim.args("test", json!({})), every_kind_script());
    let result = &ran.result;
    assert_eq!(result["status"], "verified", "{result}");

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
    assert_eq!(
        verified(&ran),
        [OUTSIDE, INSIDE, REFUTED],
        "one wave, in the queue's rank: the route item first, then the behaviour items as the ledger has them"
    );

    // (2) EXACTLY ONE REPORT PER LAUNCHED REPORTER, on disk and committed — by the stage's two
    // records. The launched are those whose prompt handed them a report to write, and the one
    // triage, whose report the record script rendered from its entries.
    let launched = sorted(ran.reporters.clone());
    assert_eq!(
        launched.len(),
        17,
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
    assert_eq!(result["counts"]["reporters"], 17, "{result}");
    assert_eq!(
        sim.rig.status(),
        "",
        "every report is committed, and the tree is clean"
    );
    let words = ends_at_its_second_record(&sim, &ran, &candidate);
    assert_eq!(
        sorted(words),
        sorted(
            launched
                .iter()
                .map(|name| format!("reporter={name}"))
                .collect()
        ),
        "the second record names every reporter of the attempt"
    );
    let commit_paths = |rev: &str| -> Vec<String> {
        sim.rig
            .git(&["show", "--name-only", "--format=", rev])
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect()
    };
    let (first_paths, second_paths) = (commit_paths("HEAD~1"), commit_paths("HEAD"));
    assert_eq!(
        sim.rig.git(&["show", "-s", "--format=%s", "HEAD~1"]).trim(),
        first_record_subject()
    );
    assert!(
        first_paths
            .iter()
            .chain(&second_paths)
            .all(|path| path.starts_with(&format!("{RUN_DIR}/"))),
        "of the run directory's paths and nothing else: {first_paths:?} {second_paths:?}"
    );
    for name in &launched {
        let report = format!("{RUN_DIR}/r1/reports/test/{name}.a1.md");
        let (holds, says) = if name.starts_with("verify-") {
            (&second_paths, "second")
        } else {
            (&first_paths, "first")
        };
        assert!(
            holds.contains(&report),
            "`{report}` is in the {says} record's commit"
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
            "contested"
        ]),
        "a contested finding is the human's, though it is inside the round's test set"
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
        json!([true, true, true, 17])
    );
    for clause in state["clauses"].as_array().expect("the clause table") {
        assert_eq!(
            json!([clause["status"], clause["commit"], clause["round"]]),
            json!(["green", candidate, 1]),
            "every clause an item of this round judged ran to its end on the candidate: {clause}"
        );
    }

    // (4) THE REMOTE BRANCH STANDS AT THE SECOND RECORD.
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([result["record"], result["verify"]["record"]]),
        json!([sim.rig.rev("HEAD~1"), head]),
        "{result}"
    );
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(head.clone()),
        "the loop branch is pushed"
    );
    // THE CANDIDATE'S GATE IS ON RECORD, in the first record's commit: what the stage's run of
    // it showed — green here — as a table of the round, which every record commit of it is
    // held to.
    assert!(
        first_paths.contains(&format!("{RUN_DIR}/r1/gate.md")),
        "the candidate's gate is a file of the first record's commit: {first_paths:?}"
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
        "and the batch the last record applied is settled"
    );

    // EVERY STEP ASKED FOR ITS DIGEST, under the invocation's scratch root (K3) — and each
    // record's push was a record's: held to publishing the run's records and nothing else.
    let scratch = sim.rig.scratch.display().to_string();
    for step in ran
        .commands
        .iter()
        .filter(|ran| ran.starts_with("dev/stabilize-step "))
    {
        // The one command of the step tool that is no step: the hash a driving agent asks of
        // the tool, and reads itself.
        assert!(
            step.contains(&format!(" --digest {scratch}"))
                || step.starts_with(&format!(
                    "dev/stabilize-step hash --scratch {scratch} --file bin/"
                )),
            "a step of the stage asked for the line the tool prints unasked: {step}"
        );
    }
    assert_eq!(
        ran.commands
            .iter()
            .filter(|command| **command
                == format!(
                    "dev/stabilize-step push --branch {LOOP} --run-dir {RUN_DIR} --digest {scratch}"
                ))
            .count(),
        2,
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
    // (K3): no row of the record passes through the harness — and which finding its verifier
    // contests, with the basis, is that file's to say.
    assert_eq!(result["rule"]["findings"], 2, "{result}");
    assert_eq!(
        document(result)["human_list"],
        json!([{"key": INSIDE, "why": "contested", "basis": BASIS}, {"key": OUTSIDE, "why": "outside"}])
    );
    assert_eq!(
        document(result),
        state,
        "the file is the state, as the record script reads it"
    );
    assert!(
        result.get("human_list").is_none()
            && result.get("blockers").is_none()
            && result.get("forks").is_none()
            && result.get("unverified").is_none(),
        "{result}"
    );
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
        json!({"items": 7, "items_run": 6, "reporters": 17, "findings_in": 4,
               "blockers": 0, "for_the_human": 2, "voided": 0, "unasserted": 0, "held": 0})
    );
    assert_eq!(
        result["scope"],
        json!({"status": "written", "uncovered": 0, "reached_but_excluded": 1}),
        "the doors the scope step left out are HOW MANY in the return, and the round's scope names them"
    );
    assert_eq!(
        json!([result["verify"], result["budget"], result["continues"]]),
        json!([
            {"waves": 1, "launched": 3, "verdicts": 3, "record": head, "unverified": 0,
             "unheld": 0, "contested": 1, "silent": 0, "last": null},
            {"limit": 30, "spent": 3, "left": 27},
            []
        ]),
        "{result}"
    );
    assert_eq!(
        json!([
            result["view"]["verified"],
            result["view"]["confirmed"],
            result["view"]["refuted"],
            result["view"]["inside"],
            result["view"]["outside"],
            result["view"]["unreached"]
        ]),
        json!([3, 2, 1, 0, 1, 0]),
        "the hit rate, and how long the view's lists are — two confirmed, and one of them in the list of what was confirmed outside the round's test set: the other is the finding its verifier contests, which the view lists apart, AND OF THAT LIST THE DIGEST HOLDS NO COUNT: {result}"
    );
    assert_eq!(
        document(result)["view"]["contested"]
            .as_array()
            .map(|rows| rows
                .iter()
                .map(|row| row["key"].clone())
                .collect::<Vec<_>>()),
        Some(vec![json!(INSIDE)]),
        "which finding is contested is the document's to say, and the view's file's"
    );
    says_what_the_state_says(&sim, result);
    holds_no_list(
        result,
        &[
            INSIDE,
            OUTSIDE,
            REFUTED,
            SEEDED,
            "verify-",
            "row-a",
            "arm-a",
            "drive-a",
            "jigc rename",
            "jigc doc set",
            "write boundary",
        ],
    );
    assert!(
        ran.logs
            .iter()
            .any(|line| line.contains("5 of 6 hunting item(s) run this round, and 1 check(s)")),
        "the round ran what its doors select: {:?}",
        ran.logs
    );

    // A CONTESTED VERDICT IS ON RECORD BY ITS VERIFIER'S OWN FILING — and no advocate and no
    // independent drive was launched for it: it is on the human's list, and no fixer's.
    assert!(
        !ran.agents()
            .iter()
            .any(|label| label.starts_with("advocate:")
                || label.starts_with("proposal:")
                || label.starts_with("triage:p")
                || label.starts_with("record:")),
        "{:?}",
        ran.agents()
    );
    assert_eq!(
        state["blockers"],
        json!([]),
        "a contested finding nobody ruled is no fixer's"
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/triage.md"))
            .lines()
            .any(|line| line.starts_with(&format!("| `{INSIDE}` |"))
                && line.ends_with("| contested |")),
        "in the round's committed triage record"
    );

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
        [
            "git:state",
            "git:state:test-1",
            RULINGS_CHECK,
            "record:rulings:r1"
        ],
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
        only_rules(1),
        "two reads, the one step that records rulings, its gate — held — its commit, the push, the state: nothing is started"
    );
    // THE CONTESTED FINDING IS STILL UNRULED, so the next step is still the human's — never
    // `fix`, though nothing else is left to rule on.
    assert_eq!(
        json!([ruled.result["next"], ruled.result["rule"]["findings"]]),
        json!(["rule", 1]),
        "{}",
        ruled.result
    );
    assert_eq!(
        document(&ruled.result)["human_list"],
        json!([{"key": INSIDE, "why": "contested", "basis": BASIS}])
    );
    // THE HUMAN'S RULING ON IT is a ruling like the others, written by the same one step:
    // admitted, with the note that reaches the fixer.
    let contested_ruled = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"key": INSIDE, "ruling": "admitted", "note": "take the robust path"}]}),
        ),
        json!({}),
    );
    assert_eq!(
        contested_ruled.result["status"], "ruled",
        "{}",
        contested_ruled.result
    );
    assert_eq!(
        contested_ruled.agents(),
        only_rules(2),
        "by the one step that records rulings — whose gate is held under the NEXT name: the first is the gate of the rulings recorded before these, on the tree as it stood then"
    );
    let ruled = contested_ruled;
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
        "the ruled finding is a fixer's, with the human's note"
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

/// **An unverified finding is finished by the next invocation, and after one more it is the
/// human's** — MOVED TO AN OPENING WITH AN ORDER (the reshape's plan, `T6b`: *the old scenarios
/// about a lap, an unverified finding, a second pass and an advocate in `test` are moved*).
/// On the other path this was the lap: an invocation that graded every untriaged row again,
/// verified it, and launched an advocate where it was contested. IN A RUN WITH A QUEUE IT IS
/// A CONTINUATION: the finding is a row of the queue, no triage grades it a second time, and
/// the invocation that follows launches its verifier once more — under its own attempt, on
/// the candidate the round tested (`M6`) — and reaches its record. WHAT IS COUNTED IS THE
/// LAUNCHES THAT LEFT NO VERDICT, by the verifier's reports (`F3`, as the record script counts
/// it in such a run): left so twice, the finding is the human's, who rules it or grants one
/// more; and a stage that leaves reports and no record twice in a row is the human's too.
#[test]
fn an_unverified_finding_is_finished_by_the_next_invocation_and_after_one_more_it_is_the_humans() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("unverified", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let key = INSIDE;
    let undriven = || halts_reported("the repro can be neither driven nor reconstructed");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": one_finding(),
            "triage": one_entry(),
            // The verifier could not drive it: a halt, with its report, and no verdict.
            format!("verify:{key}"): undriven(),
        }})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["verify"]["unverified"],
            result["verify"]["verdicts"]
        ]),
        json!(["hunted", "verify", 1, 0]),
        "{result}"
    );
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", key),
        [format!(
            "NOT VERIFIED: `{key}` — its verifier halted (the repro can be neither driven nor reconstructed), and the queue prints the finding again: no verdict of it counts. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "the stage says which finding it left without a verdict, and why — in its log: {:?}",
        ran.logs
    );
    // `verify` IS A VALUE OF `next` THE SCRIPT KNOWS (`T7`): it goes back with what the stage
    // is continued for, and with no state attached — the document lies in the file the return
    // names.
    assert_eq!(
        json!([
            result["returned_to_orchestrator"],
            result["state"],
            result["continues"]
        ]),
        json!([null, null, ["verify"]]),
        "{result}"
    );
    assert_eq!(
        document(result)["position"]["test"],
        json!({"round": 1, "attempt": 2, "verify": true}),
        "the stage that left the finding unverified continues"
    );
    assert_eq!(
        row_of(&document(result), key)["awaits"],
        json!({"why": "unverified", "passes": 1, "retry": "due"}),
        "one launch of its verifier left no verdict"
    );
    let recorded = sim.rig.rev("HEAD");
    assert_eq!(sim.rig.rev("HEAD~1"), candidate);

    // THE NEXT INVOCATION FINISHES IT — AND REACHES ITS RECORD (`M6`): no instrument, no scope
    // step and NO TRIAGE — the row is graded — but the verifier, once more, on the commit the
    // round tested, which the record commit has moved `HEAD` off.
    let lap = json!({"agents": {format!("verify:{key}"): undriven()}});
    let finishing = sim.invoke(sim.args("test", json!({})), lap);
    let finished = &finishing.result;
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(format!("verify:{key}"));
    expected.extend(lines(&["git:check-reports:wave1", "git:state:test-2"]));
    expected.extend(continuation_records("verdicts", 2, 3));
    assert_eq!(
        finishing.agents(),
        expected,
        "the attempt begun, the binaries that stand, the verifier, the reports checked, the queue read, the record, its commit, the push, the state"
    );
    assert_eq!(
        json!([
            finished["continued"],
            finished["candidate"]["sha"],
            finished["candidate"]["label"]
        ]),
        json!([["verify"], candidate, "c1"]),
        "M6: the verifier drove the commit the round tested, not the record commit ({recorded}): {finished}"
    );
    let (subject, paths) = head_commit(&sim);
    assert_eq!(subject, second_record_subject());
    assert_eq!(sim.rig.rev("HEAD~1"), recorded, "one record commit");
    assert_eq!(sim.rig.remote(LOOP), Some(sim.rig.rev("HEAD")), "pushed");
    for name in [
        "attempt.a2".to_owned(),
        format!("verify-{key}.a1"),
        format!("verify-{key}.a2"),
    ] {
        assert!(
            paths.contains(&format!("{RUN_DIR}/r1/reports/test/{name}.md")),
            "the report `{name}` is in the continuation's record commit — the one the first stage's halted verifier left among them: {paths:?}"
        );
    }
    assert_eq!(sim.rig.status(), "");

    // REPAIRED — F3 (state-machine.md), THROUGH THE HARNESS: the finding, left without a
    // verdict by a second launch, is the human's — with the reason — and no call of this
    // suite into the record script wrote that.
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
        "F3: how many the human's list holds, and the key that may be granted one more launch: {state}"
    );
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["recorded"],
            state["rounds"][0]["test_unrecorded"],
            row_of(&state, key)["awaits"]
        ]),
        json!([["test a2"], 0, {"why": "unverified", "passes": 2, "retry": "spent"}]),
        "F3: the continuation reached its record, and two launches have left the finding without a verdict: {state}"
    );
    let refused = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(refused.result["status"], "refused", "{}", refused.result);
    assert_eq!(
        refused.result["refused"],
        json!({"refused": "round-open", "round": 1}),
        "F3: no invocation verifies that finding again unasked"
    );
    assert_eq!(refused.trace, lines(&TWO_READS));
    // What the human may do is rule it — or grant one more, a fact the one step that records
    // rulings writes, and that the record script takes only here.
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
    assert_eq!(granted.result["next"], "verify", "{}", granted.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["reverify"],
            state["position"]["test"]
        ]),
        json!([[format!("{key} p3")], {"round": 1, "attempt": 3, "verify": true}]),
        "F3: one grant, one more launch: {state}"
    );

    // A STAGE THAT LEAVES ITS MARKER AND NO RECORD, TWICE IN A ROW, IS THE HUMAN'S TOO — and
    // that is the one exit of a finding whose verifier leaves NOTHING AT ALL, whose launches
    // nothing counts. The granted continuation's verifier leaves no report and no verdict:
    // THERE IS NOTHING TO RECORD, so the invocation composes no record — a record that held
    // nothing but its own attempt would make every such attempt one that reached its record,
    // and the loop would have no counter. It returns, says so, and the attempt is counted.
    let stuck = || nobody_verifies(json!({}), &[key]);
    let left_nothing = sim.invoke(sim.args("test", json!({})), stuck());
    let nothing = &left_nothing.result;
    assert_eq!(
        json!([
            nothing["status"],
            nothing["halted"],
            nothing["record"],
            nothing["verify"]["record"],
            nothing["verify"]["launched"],
            nothing["verify"]["unverified"],
            nothing["next"]
        ]),
        json!(["continued", null, null, null, 1, 1, "verify"]),
        "an invocation that continued a stage and recorded nothing hunted nothing, and does not say that it did (`T7`): {nothing}"
    );
    assert!(
        left_nothing.agents().last() == Some(&"git:state:test-2")
            && composing(&left_nothing).is_empty()
            && left_nothing
                .logs
                .iter()
                .any(|line| line.starts_with("NOTHING TO RECORD")),
        "no record is composed where the invocation has nothing to record, and it says so: {:?}",
        left_nothing.agents()
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["next"],
            state["position"]["test"],
            state["rounds"][0]["test_unrecorded"]
        ]),
        json!(["verify", {"round": 1, "attempt": 4, "verify": true}, 1]),
        "one attempt without a record is tried once more without the human: {state}"
    );
    // … and the one after it HALTS before its record: its wave's report check never comes back.
    let mut dying = stuck();
    dying["endings"]["git:check-reports:wave1"] = json!("dies");
    let halted_again = sim.invoke(sim.args("test", json!({})), dying);
    assert_eq!(
        json!([
            halted_again.result["status"],
            halted_again.result["halted"]["phase"]
        ]),
        json!(["halted", "reports"]),
        "{}",
        halted_again.result
    );
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
    assert_eq!(once_more.result["next"], "verify", "{}", once_more.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["rounds"][0]["facts"]["again"],
            state["position"]["test"],
            state["human_stages"]
        ]),
        json!([["test a5"], {"round": 1, "attempt": 5, "verify": true}, []]),
        "F3: one grant, one more attempt: {state}"
    );

    // THE GRANTED CONTINUATION VERIFIES IT: a verdict, filed, on the candidate the round
    // tested — and the round is over.
    let settled = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {format!("verify:{key}"): refutes("does-not-reproduce")}}),
    );
    assert_eq!(settled.result["status"], "verified", "{}", settled.result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["next"],
            state["stop"]["then"],
            state["untriaged"],
            state["human_list"],
            queued_keys(&state),
            row_of(&state, key)["grade"]
        ]),
        json!(["stop", "close", [], [], [], "refuted"]),
        "the finding is finished by the verdict, and the round is over: {state}"
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

// RETIRED WITH THE CONTINUATION (the reshape's plan, `T6b`: *the old scenarios about … an
// advocate in `test` are moved to an opening with an order, each rewritten for the new path
// or retired with its cell's new home named*): the scenario that stood here,
// `a_verifier_an_advocate_or_a_proposal_driver_that_does_not_finish_leaves_the_finding_unverified`.
// A run with a queue launches no advocate and no independent drive in `test`, so five of its
// cells have no subject there. Where each one's cell lives now:
//   the advocate dies · returns its case   a contested verdict has no case and no drive in
//   with no report · the independent       `test`: `a_contested_verdict_is_the_humans_and_no_
//   drive dies after its report · returns  advocate_is_launched_on_the_new_path`. A fork — a
//   with no report · a fork that is whole, case and a drive, whole or none — is the `fix`
//   its drive differing                    half's, and returns with that half's repair (the
//                                          row `wf-fork-serial`).
//   the verifier dies                      `one_silent_verifier_call_leaves_its_finding_
//                                          unverified_and_the_phase_goes_on`
//   a verdict returned with no report      that scenario too: a verifier that returns a
//                                          filing and left no report
//   a verdict for another finding          a verdict is filed under the reporter of its own
//                                          key or refused (`dev_stabilize_record::a_filing_
//                                          under_a_reporter_that_is_not_the_keys_is_refused`),
//                                          and a return that names another filing's path is
//                                          not held (`a_verifier_that_returns_a_verdict_and_
//                                          filed_none_left_no_verdict`)
//   the lap verifies again, and after one  `an_unverified_finding_is_finished_by_the_next_
//   more the finding is the human's, who   invocation_and_after_one_more_it_is_the_humans`,
//   grants one more                        moved to an opening with an order

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

/// **A digest that comes back with ONE VALUE SLIPPED is read as the tool printed it** (the
/// staged canary's part 1, 2026-10-16). The state's digest came back from its agent with one
/// row's `"retry":"never-selected"` as the `"no-instrument"` of the rows below it — on three
/// tries of three, the same slip — and the stage halted at its state read, every retry
/// failing alike. The value put back where the line holds it under the same name makes the
/// line hash to the sha256 it ends with, and that line is the tool's: the read is taken on its
/// FIRST try, and the log says which value came back as what. A slip of a value the line
/// holds nowhere else under its name is no line this harness can put back: that read is asked
/// for again twice and the stage halts, as before.
#[test]
fn a_digest_relayed_with_one_value_slipped_is_read_as_the_tool_printed_it() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("slipped", &one_area());
    let head = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({"stopAfter": "state"})),
        json!({"relays": {"git:state:test-1": "slip"}}),
    );
    assert_eq!(ran.result["status"], "stopped", "{}", ran.result);
    assert_eq!(
        ran.agents(),
        ["git:state", "git:state:test-1"],
        "read on its first try: {:?}",
        ran.logs
    );
    let mended: Vec<&String> = ran
        .logs
        .iter()
        .filter(|line| line.contains("came back with ONE value altered"))
        .collect();
    assert_eq!(mended.len(), 1, "{:?}", ran.logs);
    assert!(
        mended[0].starts_with(
            "the relayed line of `git:state:test-1` came back with ONE value altered — `"
        ) && mended[0].contains(" where the tool printed ")
            && !ran
                .logs
                .iter()
                .any(|line| line.contains("state relay rejected")),
        "{:?}",
        ran.logs
    );

    // A SLIP NOTHING IN THE LINE CAN PUT BACK: asked for again twice, and then no state.
    let relays: serde_json::Map<String, Value> = state_reads("test-1")
        .into_iter()
        .map(|label| (label, json!("slip-lone")))
        .collect();
    let lone = sim.invoke(sim.args("test", json!({})), json!({"relays": relays}));
    assert_eq!(
        json!([lone.result["status"], lone.result["halted"]["phase"]]),
        json!(["halted", "state"]),
        "{}",
        lone.result
    );
    assert!(
        lone.result["halted"]["reason"]
            .as_str()
            .is_some_and(|why| why.contains("does not end with the sha256 of itself")),
        "{}",
        lone.result
    );
    assert_eq!(
        lone.agents(),
        [
            "git:state",
            "git:state:test-1",
            "git:state:test-1:again1",
            "git:state:test-1:again2"
        ]
    );
    assert!(
        !lone
            .logs
            .iter()
            .any(|line| line.contains("came back with ONE value altered")),
        "{:?}",
        lone.logs
    );
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

/// **Where no name is left for a pending batch's gate the halt says so, and the exit it names
/// works** (the scoped review before the parts, its fourth, in words: *the tenth finish of a
/// pending batch finds no name, and its halt says that the next invocation holds the gate
/// under the next name — there is none*). The gate of a batch an earlier invocation applied
/// is held under `record-pending-<n>`, whichever batch it is, and a name is one command's for
/// the life of a scratch root: nine of them. HERE EVERY ONE IS TAKEN — a directory under each,
/// which the tool answers as a job that is there. The invocation asks all nine, starts
/// nothing, and HALTS SAYING THAT NO NAME IS LEFT: not that an agent returned nothing, not
/// that the next invocation gates the batch *under the next name* — and it names the exit,
/// the batch taken back. THE EXIT WORKS: the named act takes the batch back, and the stage
/// invoked after it works its next attempt, whose record is gated under that attempt's own
/// name, committed and pushed. (A run without an order, as every scenario of a pending batch
/// here: the function that finishes one is both paths'.)
#[test]
fn a_pending_batch_whose_gate_has_no_name_left_halts_saying_so_and_names_the_exit_that_works() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("held-no-name", &one_area());
    let opened = sim.rig.rev("HEAD");
    let first = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"record": {"gate": {"void": true}}})),
    );
    assert_eq!(
        json!([first.result["status"], first.result["halted"]["phase"]]),
        json!(["halted", "record"]),
        "the batch is applied, and its gate left no verdict: {}",
        first.result
    );
    for n in 1..=9 {
        fs::create_dir_all(
            sim.rig
                .scratch
                .join("hold")
                .join(format!("record-pending-{n}")),
        )
        .expect("a job's directory");
    }
    let discard = format!("`dev/stabilize-step discard --branch {LOOP} --run-dir {RUN_DIR}`");
    for again in 1..=2 {
        let halted = sim.invoke(sim.args("test", json!({})), json!({}));
        let result = &halted.result;
        assert_eq!(
            json!([
                result["status"],
                result["halted"]["phase"],
                result["halted"]["transient"],
                result["halted"]["held"]["name"]
            ]),
            json!(["halted", "record", false, "record-pending-9"]),
            "invocation {again}: {result}"
        );
        let mut expected = lines(&["git:state"]);
        expected.extend((1..=9).map(|n| format!("git:hold-start:record-pending-{n}")));
        assert_eq!(
            halted.agents(),
            expected,
            "invocation {again}: every name is asked, and nothing is started, waited for or committed"
        );
        let message = text(&result["message"]);
        assert!(
            message.contains("the record's gate was not held: every name from `record-pending-1` to `record-pending-9`")
                && message.contains("NO NAME IS LEFT TO HOLD ITS GATE UNDER")
                && message.contains("INVOKING THE STAGE AGAIN CHANGES NOTHING")
                && message.contains(&discard)
                && !message.contains("under the next name")
                && !message.contains("An agent returned no result"),
            "invocation {again}: the halt says that no name is left, and names the batch taken back: {message}"
        );
        assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    }

    // THE EXIT THE HALT NAMES, taken as it is spelled: the batch is taken back …
    let taken_back = sim.step(&["discard", "--branch", LOOP, "--run-dir", RUN_DIR]);
    assert_eq!(taken_back["status"], "discarded", "{taken_back}");
    assert_eq!(
        sim.record(&["pending", "--run", RUN], "")["batch"],
        Value::Null,
        "no batch is pending"
    );
    // … and the stage invoked after it works its next attempt, to its pushed record — gated
    // under that attempt's own name.
    let after = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(after.result["status"], "triaged", "{}", after.result);
    assert!(
        after
            .agents()
            .contains(&"git:hold-wait:record-test-r1-a2-1"),
        "{:?}",
        after.agents()
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            sim.rig.remote(LOOP),
            sim.rig.status()
        ]),
        json!([opened, head, ""]),
        "ONE record on the candidate, pushed, and nothing left pending"
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

/// A STAGE WHOSE FIRST RECORD IS ITS ONLY ONE (the reshape's plan, `T5b`; the harness half of
/// `K3`; its `T6a` for what follows the record). The stage returns `hunted` with the state's
/// `next`; the record is ONE commit on the candidate, pushed, and the tree is clean; **the
/// record step is tool steps only** — the composing step, the record's gate held, the one
/// commit, the push — and NO EXECUTOR IS LAUNCHED and no batch typed; no agent of the other
/// path runs: no pass of a triage, no advocate, no independent drive. AND THE WORDS THE STEP
/// WAS HANDED HASH, IN THEIR CANONICAL FORM, TO THE SHA256 THE HARNESS COMPUTED IN JAVASCRIPT:
/// the form is written here a second time, by hand, and the record script — which computes it
/// a third time — took the call. WHAT FOLLOWS THE RECORD IS THE VERIFY PHASE, AND HERE IT LEFT
/// NOTHING: either the queue printed no key and nobody was launched, or every verifier that
/// was launched left neither a report nor a verdict ([`nobody_verifies`]) — then the second
/// record was composed over the first one's reporters, refused as one that changes nothing,
/// and that was no halt. NO VERIFIER STARTS BEFORE THE FIRST RECORD IS PUSHED AND READ BACK.
/// Returns the first record's words.
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
    for other in ["triage:p", "advocate:", "proposal:", "record:"] {
        assert!(
            !agents.iter().any(|label| label.starts_with(other)),
            "`{other}…` is of the other path, or an executor: {agents:?}"
        );
    }
    let gate = format!("record-test-r1-a{attempt}-1");
    let read_back = agents
        .iter()
        .position(|label| *label == "git:state:test-3")
        .unwrap_or_else(|| panic!("the first record is read back: {agents:?}"));
    assert_eq!(
        agents[read_back - 5..=read_back],
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
    // WHAT FOLLOWS IT: nothing — or one wave of verifiers that left nothing, its report check,
    // the queue read again, and a second record composed and refused as changing nothing.
    let launched = verified(ran);
    assert!(
        !agents[..read_back]
            .iter()
            .any(|label| label.starts_with("verify:")),
        "no verifier starts before the first record is pushed and read back: {agents:?}"
    );
    let mut after: Vec<String> = launched.iter().map(|key| format!("verify:{key}")).collect();
    if !launched.is_empty() {
        after.extend(lines(&[
            "git:check-reports:wave1",
            "git:state:test-4",
            "git:compose:verdicts:r1",
        ]));
    }
    assert_eq!(
        agents[read_back + 1..],
        after.iter().map(String::as_str).collect::<Vec<_>>(),
        "after its first record the stage launched what the queue printed, and nothing of it was left to record: {agents:?}"
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
        if launched.is_empty() { 1 } else { 2 },
        "one step composes the record — and, where a verifier was launched, one more is asked for the second: {composed:?}"
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

/// A stage whose verifiers LEAVE NOTHING: each of `keys` — the rows the queue prints after the
/// first record — gets a verifier that halts and leaves no report, so that no file is written,
/// no launch is counted, nothing is spent and the state stands exactly as the first record
/// left it. What a scenario whose subject is not the verify phase scripts for it.
fn nobody_verifies(mut script: Value, keys: &[&str]) -> Value {
    for key in keys {
        let label = format!("verify:{key}");
        script["agents"][&label] = json!({"status": "halted", "halt": {
            "root_cause": "scripted: it left nothing", "evidence": "none", "tree_state": "clean",
            "recommendation": "none"}});
        script["endings"][&label] = json!("halts-unreported");
    }
    script
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
        nobody_verifies(
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
            &[ROUTE, "red-regression-set"],
        ),
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
        nobody_verifies(
            script(json!({
                "entries": [once("first", json!([1]))],
                "then": [once("first", json!([1])), once("second", json!([2]))],
            })),
            &["first", "second"],
        ),
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
        nobody_verifies(
            small_stage(json!({"agents": {
                "area-a:review": found(&[
                    finding("s-1", "a route that is quoted", "jigc doc set", "audit-clean"),
                    finding("s-2", "a route that is described", "jigc doc set", "audit-clean"),
                ]),
                "triage": {"entries": [evident, bare]},
            }})),
            &["a-route-with-its-line"],
        ),
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
        // (The WORD `lead` stands in the return, since `T7`: as one of the words that keep a
        // row out of the queue, with how many rows it keeps out — the view's, as the digest
        // has it. A kind of an entry does not.)
        "\"kind\":\"lead\"",
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
/// check's word is its job; a reporter's word is a reporter that left a report. AT THAT
/// RECORD NOTHING IS VERIFIED: the state the stage reads back says `verify`, the queue holds
/// the rows the triage sent on, no row holds a verdict and the budget is unspent — and only
/// then are the verifiers launched (the plan's `T6a`), which here leave nothing.
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
        nobody_verifies(
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
            &[ROUTE, "red-regression-set"],
        ),
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

    // (3) AT THE RECORD NOTHING IS VERIFIED, and the state the stage read back says what
    // follows: the queue holds the rows the triage sent on, in its rank, the budget is
    // unspent, and the position is a continuation's. THE VERIFIERS START AFTER THAT READ, for
    // exactly those rows — and here they leave nothing, so the state stands as it stood.
    let read_back = sim.read_at(3);
    for state in [&read_back, &state] {
        assert_eq!(
            json!([
                state["next"],
                state["position"]["test"]["verify"],
                state["budget"]["spent"],
                queued_keys(state)
            ]),
            json!(["verify", true, 0, [ROUTE, "red-regression-set"]]),
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
    }
    assert_eq!(verified(&ran), [ROUTE, "red-regression-set"]);
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
    let again = sim.invoke(
        sim.args("test", json!({})),
        nobody_verifies(script(), &[INSIDE]),
    );
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
    // WHAT THE REGRESSION TOOL'S LINE SAYS OF THE BASELINE HAS SIX FIELDS since that tool's
    // repair (DECISIONS.md → 2026-10-13, *Three findings of the scoped review, repaired*, 3):
    // `unkept` — what a run built there and did not keep, by the word the run was void by.
    // The stand-in prints what it is told, and is told here what that tool now says: A VOID
    // RUN KEEPS NOTHING, and says so; the run after it builds afresh, and keeps that.
    let unkept = json!({"why": "build-candidate",
                        "detail": "scripted by the simulation: what this run built was not kept"});
    // THE FIRST STAGE: the previous release's baseline is built, and the candidate does not
    // build — a void run of the item, which is due again, and which keeps no baseline.
    let voided = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"held": {"regression-set": {
            "status": "void", "reason": "build-candidate", "baseline": {"unkept": unkept}}}})),
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
    // command of its own, handed the same directory: it finds no kept baseline there, builds
    // its own, and keeps it.
    let again = sim.invoke(
        sim.args("test", json!({"clause": "working-product"})),
        json!({"held": {"regression-set": {
            "status": "green", "baseline": {"sha256": key}}}}),
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
    for (run, key, unkept) in [(1, Value::Null, unkept), (2, json!(key), Value::Null)] {
        let text = sim
            .rig
            .read(&format!("{RUN_DIR}/r1/checks/regression-set.a{run}.json"));
        let kept: Value = serde_json::from_str(text.trim_end()).expect("the verdict on record");
        assert_eq!(
            json!([
                kept["asked"]["baseline"],
                kept["evidence"]["line"]["baseline"]
            ]),
            json!([kept_in, {"dir": format!("<scratch>/{kept_in}"), "used": false,
                             "refused": null, "detail": null, "sha256": key, "unkept": unkept}]),
            "run {run}: the line's six fields, as the regression tool prints them: {kept}"
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

/// The machine's own gitleaks, where it has one — and where it has none, what a test that
/// needs it does: `unrun` says what was not run. It returns None and the caller passes, BUT
/// NOT SILENTLY: beside the line on stderr the skip is written to the file the gate names
/// (`JIGC_GATE_SKIPS`), and `dev/gate` prints it in its own summary, as it does a missing
/// `node` ([`node_or_skip`]). CI's tooling legs carry no gitleaks — only its `hygiene` job
/// installs one, and that scans the pushed range, never a record a test composes — so there
/// a test that needs it runs nothing, and what holds the scan instead is the local gate and
/// the record script's own scan at every real record.
fn gitleaks_or_skip(unrun: &str) -> Option<PathBuf> {
    let found = std::env::var_os("PATH").and_then(|path| {
        std::env::split_paths(&path)
            .map(|dir| dir.join("gitleaks"))
            .find(|candidate| candidate.is_file())
    });
    if found.is_none() {
        let skipped = format!("SKIPPED: `gitleaks` is not on PATH, so {unrun}");
        eprintln!("{skipped}");
        if let Some(skips) = std::env::var_os("JIGC_GATE_SKIPS") {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&skips)
                .unwrap_or_else(|e| panic!("open the gate's skip file: {e}"));
            std::io::Write::write_all(&mut file, format!("{skipped}\n").as_bytes())
                .expect("write the skip to the gate's file");
        }
    }
    found
}

/// The machine's own gitleaks over `target`, with this repository's configuration: the
/// rules of its findings, by `(rule, file)`, the file as gitleaks names it.
fn gitleaks_finds(gitleaks: &Path, target: &Path) -> Vec<(String, String)> {
    let report = RigDir::new("sim-gitleaks-report");
    let path = report.path().join("report.json");
    let out = Command::new(gitleaks)
        .args([
            "dir",
            "--no-banner",
            "--no-color",
            "--redact",
            "--exit-code",
            "77",
        ])
        .args(["--report-format", "json", "--report-path"])
        .arg(&path)
        .arg("--config")
        .arg(repo_root().join(".gitleaks.toml"))
        .arg(target)
        .output()
        .expect("run the machine's gitleaks");
    let code = out.status.code();
    assert!(
        code == Some(0) || code == Some(77),
        "gitleaks ran and answered — exit {code:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let found: Value = serde_json::from_str(&fs::read_to_string(&path).expect("gitleaks' report"))
        .expect("gitleaks' report is JSON");
    let found: Vec<(String, String)> = found
        .as_array()
        .expect("a list of findings")
        .iter()
        .map(|leak| (text(&leak["RuleID"]), text(&leak["File"])))
        .collect();
    assert_eq!(
        code == Some(77),
        !found.is_empty(),
        "its exit and its report agree"
    );
    found
}

/// **What the tools write into a record passes the scan the record is held to — the
/// machine's own gitleaks, with this repository's configuration** (the staged canary's part
/// 2, run by the human on 2026-10-15: NO RECORD OF A RUN THAT HELD A REGRESSION ITEM COULD BE
/// COMPOSED — the regression tool named the sha256 of its kept baseline's key `key`, the
/// record carries the tool's line, and gitleaks' `generic-api-key` reads a `key` beside a
/// 64-hex value as a credential; `dev/stabilize-record` refused it `hygiene`, as it must).
/// No suite met it: every record a stage writes here is scanned by a stand-in for gitleaks
/// that flags one planted string, and the stand-in for the regression set prints the field
/// its scenario names, with a hash of one repeated digit no entropy rule flags. Here the
/// line is THE TOOL'S OWN — a run of `dev/regression-set` that kept its baseline, under its
/// suite's stand-ins for cargo — and its baseline is handed to the stand-in for the long
/// command as the tool printed it, but for the directory, which is the stage's; the
/// stand-in for gitleaks is removed; and the stage's first record is composed, scanned by
/// gitleaks and committed. A BEFORE-CONTROL FIRST: the tool's hash under the name the tool
/// wrote until this repair is a hit of the same scanner and configuration, so a scanner that
/// flags nothing cannot pass this arm.
#[test]
fn a_record_composed_from_the_regression_tools_own_line_passes_the_machines_own_gitleaks() {
    if !can_run() {
        return;
    }
    let Some(gitleaks) = gitleaks_or_skip(
        "no record composed from the regression tool's own line was scanned by gitleaks",
    ) else {
        return;
    };
    let line = a_kept_baselines_line();
    let mut baseline = line["baseline"].clone();
    let fields = baseline
        .as_object_mut()
        .unwrap_or_else(|| panic!("the line says what became of its baseline: {line}"));
    fields.remove("dir");
    let hash = fields
        .values()
        .filter_map(Value::as_str)
        .find(|value| value.len() == 64 && value.bytes().all(|b| b.is_ascii_hexdigit()))
        .unwrap_or_else(|| panic!("the line names its kept baseline by a sha256: {line}"))
        .to_owned();

    // THE BEFORE-CONTROL: the same hash, under the name the tool wrote until 2026-10-15.
    let control = RigDir::new("sim-gitleaks-control");
    fs::write(
        control.path().join("regression-set.a1.json"),
        format!("{{\n \"baseline\": {{\n  \"key\": \"{hash}\"\n }}\n}}\n"),
    )
    .expect("write the control");
    assert_eq!(
        gitleaks_finds(&gitleaks, control.path())
            .into_iter()
            .map(|(rule, _)| rule)
            .collect::<Vec<_>>(),
        ["generic-api-key"],
        "the scanner and its configuration read a sha256 named `key` as a credential"
    );

    let sim = Sim::queued(
        "queue-real-gitleaks",
        &with_held_checks(&["regression-set"]),
    );
    fs::remove_file(sim.rig.dir().join("bin/gitleaks")).expect("remove the stand-in");
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"held": {"regression-set": {"status": "green", "baseline": baseline}}})),
    );
    ends_at_its_first_record(&sim, &ran, &candidate, 1, "stop");
    let check = format!("{RUN_DIR}/r1/checks/regression-set.a1.json");
    let kept: Value =
        serde_json::from_str(sim.rig.read(&check).trim_end()).expect("the verdict on record");
    let said = &kept["evidence"]["line"]["baseline"];
    assert_eq!(
        json!([
            said["sha256"],
            said.as_object()
                .map(|o| o.keys().cloned().collect::<Vec<_>>())
        ]),
        json!([
            hash,
            ["detail", "dir", "refused", "sha256", "unkept", "used"]
        ]),
        "the record carries the tool's hash under the tool's name: {kept}"
    );
    assert_eq!(
        gitleaks_finds(&gitleaks, &sim.rig.root.join(RUN_DIR)),
        [],
        "and the run's directory, as committed, is clean to the same scanner"
    );
}

/// The stand-in for gitleaks of the record suite, but naming THE FILES it found the planted
/// string in — each by its path in the repository it scanned, as gitleaks' report does — so
/// that what a refusal of the record script says of them is the shape a real hit has.
fn gitleaks_stub_naming_files() -> String {
    let stub = gitleaks_stub();
    let hit = r#"    if [ -n "$report" ]; then printf '[{"RuleID":"stub-rule","StartLine":3}]' >"$report"; fi"#;
    assert!(
        stub.contains(hit),
        "the record suite's stand-in reports its hit so:\n{stub}"
    );
    stub.replace(
        hit,
        &format!(
            "    if [ -n \"$report\" ]; then\n        sep='['; out=''\n        for found in $(cd \"$repo\" && grep -rl --exclude-dir=.git {STUB_SECRET} .); do\n            out=\"$out$sep{{\\\"RuleID\\\":\\\"stub-rule\\\",\\\"StartLine\\\":3,\\\"File\\\":\\\"${{found#./}}\\\"}}\"; sep=','\n        done\n        printf '%s]' \"$out\" >\"$report\"\n    fi"
        ),
    )
}

/// **A record the public-hygiene scan stops halts the stage saying what stopped it, and
/// where** (the staged canary's part 2, 2026-10-15: the stage halted at its record with
/// *`hygiene`, a word of `dev/stabilize-record` this script has no row for*, and the human
/// had to open the step's own line to learn that a file the record would write held a
/// secret-shaped string, and which). The refusal has its row: the halt says that the scan
/// stopped a file the record would write, and names each file by its path in the repository
/// and the rule — never the string — from the composing step's digest; nothing was written.
#[test]
fn a_record_the_hygiene_scan_stops_halts_naming_each_file_and_its_rule() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-hygiene", &with_held_checks(&["regression-set"]));
    placed_executable::write(
        &sim.rig.dir().join("bin/gitleaks"),
        gitleaks_stub_naming_files(),
    );
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"held": {"regression-set": {"status": "green",
            "baseline": {"detail": format!("the environment held {STUB_SECRET}")}}}})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["refused"]
        ]),
        json!(["halted", "record", "record"]),
        "{result}"
    );
    let reason = text(&result["halted"]["reason"]);
    for said in [
        "refused the composing step `hygiene` — the public-hygiene scan stopped a file the record would write",
        &format!(
            "gitleaks' rule `stub-rule` at line 3 of `{RUN_DIR}/r1/checks/regression-set.a1.json`"
        ),
        ". Nothing was written",
    ] {
        assert!(reason.contains(said), "the halt says `{said}`: {reason}");
    }
    assert!(
        !reason.contains("has no row for") && !reason.contains(STUB_SECRET),
        "the word has its row, and the halt never says what matched: {reason}"
    );
    assert_eq!(sim.rig.rev("HEAD"), candidate, "nothing was recorded");
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
        nobody_verifies(
            small_stage(json!({"agents": {"area-a:review": one_finding(), "triage": one_entry()}})),
            &[INSIDE],
        ),
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

    // AND A RED GATE THAT ANSWERS A START WHOSE FIRST TRY RETURNED NOTHING IS PASSED OVER TOO,
    // WHOEVER STARTED IT (the scoped review before the parts, the harness's finding 1). Here
    // the dead try's OWN job had ended red by the time the step was asked again. The tool says
    // `started` to the one call that started a job, and nothing of who started one that is
    // there — so an answer that says the job has ended cannot be told from an earlier
    // invocation's, and is never taken for this attempt's. THE COST IS ONE GATE MORE: the next
    // name is started and waited for, and — red as the candidate is — is the round's gate.
    // (TURNED: until that repair the retry's answer was the invocation's own, by the count of
    // its tries — this part held `gate=<the first name>`, and that no second gate ran.)
    let sim = Sim::queued("queue-red-gate-own", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let gate = candidates_gate(&candidate);
    let scratch = sim.rig.scratch.display().to_string();
    let own = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "gate": red(&[KNOWN]),
            "record": {"gate": red(&[KNOWN])},
            "endings": {format!("git:hold-start:{gate}"): "acts-and-dies-once"},
            // The dead try's job HAS ENDED before its step is asked again: somebody waits for it.
            "meanwhile": {format!("git:hold-start:{gate}"):
                format!("dev/stabilize-step hold-wait --scratch {scratch} --name {gate}")},
        })),
    );
    let words = ends_at_its_first_record(&sim, &own, &candidate, 1, "stop");
    let next = format!("{gate}-2");
    let gates: Vec<&str> = own
        .agents()
        .into_iter()
        .filter(|label| label.contains(":gate-"))
        .collect();
    assert_eq!(
        gates,
        [
            format!("git:hold-start:{gate}").as_str(),
            format!("git:hold-start:{gate}").as_str(),
            format!("git:hold-start:{next}").as_str(),
            format!("git:hold-wait:{next}").as_str(),
        ],
        "the job that had ended answers the start's retry and is passed over; the next name is started and waited for"
    );
    assert!(
        words.contains(&format!("gate={next}")) && !words.contains(&format!("gate={gate}")),
        "the round's gate is the one that was started under the next name: {words:?}"
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/gate.md"))
            .contains(KNOWN),
        "what the candidate's gate shows red is on record"
    );
    assert_eq!(
        gates_run(&sim),
        3,
        "the dead try's gate, the gate under the next name, and the record's: one gate more"
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

/// **A name whose start left no record is passed over, and the stage holds the next** (the
/// scoped review before the parts, finding 1; DECISIONS.md → 2026-10-13, *Three findings of
/// the scoped review, repaired*). A `hold-start` killed between the job's directory and its
/// record left a name the step tool refused for ever (`missing`) — which this script reads as
/// a step that did not answer (`unread`), and does not pass over: a halt at the build under
/// that commit's name at every later invocation, for the candidate's gate too, and for a
/// record's gate *nothing was committed*, each time. The tool now answers such a name `dead`
/// (`no-record`) — a word this script already reads, so NO LINE OF IT CHANGED: read by its
/// own `readDigest` and `heldOf` here, and passed over by `hold` for what is named by its
/// commit (the build, the candidate's gate) and for what is numbered (a record's gate) alike.
#[test]
fn a_name_whose_start_left_no_record_is_passed_over_and_the_stage_holds_the_next() {
    if !can_run() {
        return;
    }
    // (`tries`: how often each start step is tried. ONCE — the name was left so by an earlier
    // invocation. TWICE — the first try of THIS invocation's start returned nothing: what the
    // tool's repair left to the harness's, *a start retried within one invocation whose first
    // try died between the directory and the record* — that answer was then the invocation's
    // own, a halt at the build, and only the invocation after it passed the name over. An
    // answer that says the job has ended is passed over whatever the tries: the harness's
    // finding 1 of the same review.)
    for (label, tries) in [("queue-no-record", 1), ("queue-no-record-own", 2)] {
        let sim = Sim::queued(label, &one_area());
        let candidate = sim.rig.rev("HEAD");
        let scratch = sim.rig.scratch.display().to_string();
        let (build, gate) = (candidate_build(&candidate), candidates_gate(&candidate));
        let records = "record-test-r1-a1-1";
        // THE STATE: under each name the stage asks for first, a directory and no record —
        // what a start killed before its record leaves, made by hand.
        let mut script = small_stage(json!({"endings": {}}));
        for name in [build.as_str(), gate.as_str(), records] {
            fs::create_dir_all(sim.rig.scratch.join("hold").join(name)).expect("a job's directory");
            if tries == 2 {
                script["endings"][format!("git:hold-start:{name}")] = json!("acts-and-dies-once");
            }
        }

        let ran = sim.invoke(sim.args("test", json!({})), script);
        let result = &ran.result;
        assert_eq!(
            json!([result["status"], result["next"], result["halted"]]),
            json!(["hunted", "stop", null]),
            "{label}: the stage runs to its record: {result}"
        );
        let agents = ran.agents();
        for name in [build.as_str(), gate.as_str(), "record-test-r1-a1"] {
            // (The record's gate is numbered: its first name ends `-1`, and the next is `-2`.)
            let (first, next) = if name.starts_with("record-") {
                (format!("{name}-1"), format!("{name}-2"))
            } else {
                (name.to_owned(), format!("{name}-2"))
            };
            let held: Vec<&str> = agents
                .iter()
                .copied()
                .filter(|label| label.ends_with(&first) || label.ends_with(&next))
                .collect();
            let mut expected = vec![format!("git:hold-start:{first}"); tries];
            expected.push(format!("git:hold-start:{next}"));
            expected.push(format!("git:hold-wait:{next}"));
            assert_eq!(
                held, expected,
                "{label}, `{first}`: the name without a record answers its start and is passed over; the next name is started and waited for"
            );
            assert!(
                ran.logs
                    .iter()
                    .any(|line| line.contains(&format!("`{first}`"))
                        && line.contains("next name is asked")),
                "{label}, `{first}`: {:?}",
                ran.logs
            );
            assert!(
                !sim.rig
                    .scratch
                    .join("hold")
                    .join(&first)
                    .join("job.json")
                    .exists(),
                "{label}, `{first}`: nothing was started under the dead name"
            );
        }
        assert_eq!(
            text(&result["candidate"]["binary"]),
            format!("{scratch}/bin/candidate-{}-2/jigc", &candidate[..12]),
            "{label}: the binary is the next name's, at a path of its own"
        );
    }

    // WHAT THE STEP SAID, AS THIS SCRIPT READS IT: the tool's own digest of that answer,
    // through `readDigest` — every field fits — and `heldOf`: `dead`, by the tool's word.
    let held = Held::new("sim-no-record");
    let root = held.rig.scratch.display().to_string();
    fs::create_dir_all(held.rig.scratch.join("hold/by-hand")).expect("a job's directory");
    for (act, line) in [
        (
            "hold-start",
            held.start_digest("by-hand", &GATE_KIND, &as_recorded("gate-green.txt"), 0),
        ),
        ("hold-wait", held.wait_digest("by-hand", 0)),
    ] {
        let step = harness_pure(&json!([["readDigest", [{"status": "ran", "line": line}, act]]]))
            .remove(0);
        assert!(step.get("relay").is_none(), "a digest of the tool: {step}");
        let dead = harness_pure(&json!([["heldOf", [step, root]]])).remove(0);
        assert_eq!(
            json!([dead["ends"], dead["why"], dead["name"], dead.get("verdict")]),
            json!(["dead", "no-record", "by-hand", null]),
            "{act}: {dead}"
        );
    }
}

/// How many full gates ran under the rig, as their stand-in wrote them down.
fn gates_run(sim: &Sim) -> usize {
    sim.long_commands()
        .iter()
        .filter(|c| *c == "gate --keep-going")
        .count()
}

/// **A job that answers a start as ended is never taken for one the invocation must run
/// itself — whatever its start step did** (the scoped review before the parts, the harness's
/// finding 1: *a record is committed and pushed behind a gate that never ran on its tree*).
/// Whether a held job *was there before this invocation asked* was decided by how often the
/// start step was tried, and not by anything the tool says: where the first try of a start
/// returned nothing, the retry's answer was taken as this invocation's own — an earlier
/// invocation's job among them. THE TOOL SAYS `started` TO THE ONE CALL THAT STARTED A JOB,
/// AND NOTHING OF WHO STARTED ONE THAT IS THERE; so an answer that says the job has ENDED is
/// never a job this invocation's own start may be answered by, and the next name is asked.
/// BOTH FACES. (1) A RECORD'S GATE, whose name every rulings invocation of a round shares:
/// two rulings in one round, the second one's start step acting and returning nothing once —
/// it was answered by the first ruling's gate, no gate ran on the tree with the second
/// ruling's tables, the commit step was handed the first ruling's output, and the return was
/// `ruled`, committed and pushed. (2) WHAT IS NAMED BY ITS COMMIT, here the candidate's
/// gate: a red one an earlier invocation left was the answer, and the round's record named a
/// red nobody measured on this attempt — which every later record's gate is then held to.
#[test]
fn a_job_that_answers_its_start_as_ended_is_never_taken_whatever_the_start_step_did() {
    if !can_run() {
        return;
    }
    // (1) A RECORD'S GATE.
    let (sim, _stage) = off_the_order("hold-rulings", 30);
    let scratch = sim.rig.scratch.display().to_string();
    let put_in = sim.invoke(
        sim.args("test", json!({"rulings": [{"queued": [ELSEWHERE]}]})),
        json!({}),
    );
    assert_eq!(put_in.result["status"], "ruled", "{}", put_in.result);
    let before = gates_run(&sim);
    let ruled = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"order": ["route hunt", "behaviour hunt"]}]}),
        ),
        json!({"endings": {"git:hold-start:record-rulings-r1-1": "acts-and-dies-once"}}),
    );
    assert_eq!(ruled.result["status"], "ruled", "{}", ruled.result);
    let held: Vec<&str> = ruled
        .agents()
        .into_iter()
        .filter(|label| label.contains(":record-rulings-r1-"))
        .collect();
    assert_eq!(
        held,
        [
            "git:hold-start:record-rulings-r1-1",
            "git:hold-start:record-rulings-r1-1",
            "git:hold-start:record-rulings-r1-2",
            "git:hold-wait:record-rulings-r1-2",
        ],
        "the first ruling's gate answers the start — on its retry too — and is passed over; the second ruling's own is started and waited for"
    );
    let commit = ruled
        .commands
        .iter()
        .find(|command| command.starts_with("dev/stabilize-step record "))
        .unwrap_or_else(|| panic!("the commit step ran: {:?}", ruled.commands));
    assert_eq!(
        flag_of(commit, "--gate"),
        format!("{scratch}/hold/record-rulings-r1-2/output"),
        "the commit step is handed the output of the gate that ran on this record's tree"
    );
    assert_eq!(
        gates_run(&sim),
        before + 1,
        "a gate ran on the tree with the second ruling's tables"
    );
    assert!(
        ruled
            .logs
            .iter()
            .any(|line| line.contains("`record-rulings-r1-1`")
                && line.contains("ended already")
                && line.contains("next name is asked")),
        "{:?}",
        ruled.logs
    );

    // (2) THE CANDIDATE'S GATE: a red one of an earlier invocation, and a start whose first
    // try returned nothing. The gate that is measured again is green.
    let sim = Sim::queued("hold-stale-red", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let gate = candidates_gate(&candidate);
    let stopped = sim.invoke(
        sim.args("test", json!({"stopAfter": "preflight"})),
        json!({"gate": red(&[KNOWN]), "scope": one_door(), "agents": {}}),
    );
    assert_eq!(
        json!([stopped.result["status"], stopped.result["gate"]["ends"]]),
        json!(["stopped", "red"]),
        "{}",
        stopped.result
    );
    let mut script = nobody_verifies(
        small_stage(json!({"agents": {"area-a:review": one_finding(), "triage": one_entry()}})),
        &[INSIDE],
    );
    script["endings"][format!("git:hold-start:{gate}")] = json!("acts-and-dies-once");
    let again = sim.invoke(sim.args("test", json!({})), script);
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
            format!("git:hold-start:{gate}").as_str(),
            format!("git:hold-start:{next}").as_str(),
            format!("git:hold-wait:{next}").as_str(),
        ],
        "the red job answers the start — on its retry too — and is passed over; the next name is started and waited for"
    );
    assert!(
        words.contains(&format!("gate={next}")) && !words.contains(&format!("gate={gate}")),
        "the round's gate is the one this attempt measured: {words:?}"
    );
    assert!(
        sim.rig
            .read(&format!("{RUN_DIR}/r1/gate.md"))
            .contains("pass"),
        "and no red an earlier invocation left is on record as the candidate's"
    );
}

/// **A build whose binary was replaced is passed over, and the stage builds under the next
/// name** (what the repair of the step tool left to be driven: *finding 2 through a stage*).
/// A held build stands green, and the file at its path is replaced afterwards: the tool
/// answers the start `void` (`hash-differs`), live — and a job that stands void is passed
/// over as one that stands red is. The stage builds the commit again under the next name, at
/// the path of that name, and every driving agent is handed that file.
#[test]
fn a_build_whose_binary_was_replaced_is_passed_over_and_built_under_the_next_name() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("queue-replaced-binary", &one_area());
    let candidate = sim.rig.rev("HEAD");
    let build = candidate_build(&candidate);
    let stopped = sim.invoke(
        sim.args("test", json!({"stopAfter": "preflight"})),
        json!({"scope": one_door(), "agents": {}}),
    );
    assert_eq!(stopped.result["status"], "stopped", "{}", stopped.result);
    let first = text(&stopped.result["candidate"]["binary"]);
    assert_eq!(
        first,
        format!(
            "{}/{}",
            sim.rig.scratch.display(),
            candidate_bin(&candidate)
        )
    );
    // THE FILE AT THE PATH IS REPLACED: another file, where the binary the build hashed lay.
    fs::remove_file(&first).expect("remove the binary");
    fs::write(&first, "#!/bin/sh\necho \"another file\"\n").expect("another file at its path");

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
        "the build whose binary is another file answers its start void and is passed over; the next name is started and waited for"
    );
    assert!(
        again
            .logs
            .iter()
            .any(|line| line.contains(&format!("`{build}`"))
                && line.contains("void")
                && line.contains("passed over")),
        "{:?}",
        again.logs
    );
    let binary = text(&again.result["candidate"]["binary"]);
    assert_eq!(
        binary,
        format!(
            "{}/bin/candidate-{}-2/jigc",
            sim.rig.scratch.display(),
            &candidate[..12]
        ),
        "the binary is the next name's, at a path of its own"
    );
    assert_eq!(
        again.result["candidate"]["sha256"],
        sha256_of(&fs::read(&binary).expect("the binary the tool built")).as_str()
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
    // WHICH ITEMS ARE VOID IS A LINE OF THE LOG, each with its word; the return says how many
    // (`T7`: a return holds no list of the stage's own making).
    assert_eq!(ran.result["counts"]["voided"], 8, "{}", ran.result);
    let named: Vec<&String> = ran
        .logs
        .iter()
        .filter(|line| line.starts_with("VOID, AND ON RECORD AS VOID: "))
        .collect();
    assert_eq!(named.len(), 1, "{:?}", ran.logs);
    for item in [
        "area-a", "area-b", "arm-a", "cross", "drive-a", "gate", "lint", "tarball",
    ] {
        assert!(
            named[0].contains(&format!("`{item}` (`")),
            "the log names the void item `{item}`, with its word: {}",
            named[0]
        );
    }
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
// A run with a queue: the verify phase (the reshape's plan, `T6a`)
// ---------------------------------------------------------------------------

/// The order the scenarios of the verify phase open a run with: what a verifier hands on
/// under a finding it confirmed stands first, then what the hunt found — a route item ahead
/// of a behaviour item. A class no scenario names is in no queue.
const VERIFY_ORDER: [&str; 4] = [
    "route under-confirmed",
    "behaviour under-confirmed",
    "route hunt",
    "behaviour hunt",
];

impl Sim {
    /// A rig whose run is opened with [`VERIFY_ORDER`], the budget a scenario names, and the
    /// hand-on number 2.
    fn budgeted(label: &str, opening: &Opening, budget: u32) -> Self {
        let sim = Sim::new(label);
        sim.open_with(
            opening,
            &json!({"order": VERIFY_ORDER, "budget": budget, "hand-on": 2}),
        );
        sim
    }

    /// The state document the stage read at its read `n` — the file its step kept it in, under
    /// the scratch root: what the run's state WAS at that point of the stage.
    fn read_at(&self, n: u32) -> Value {
        let path = self.rig.scratch.join(format!("state/test-{n}.json"));
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("the stage's read {n} kept the document ({e})"));
        serde_json::from_str(text.trim_end()).expect("the document is one line of JSON")
    }
}

/// A reviewer's findings, one a title: `s-1`, `s-2`, … at the door every scenario scopes.
fn findings(titles: &[&str]) -> Value {
    let all: Vec<Value> = titles
        .iter()
        .enumerate()
        .map(|(n, title)| {
            finding(
                &format!("s-{}", n + 1),
                title,
                "jigc doc set",
                "audit-clean",
            )
        })
        .collect();
    found(&all)
}

/// The entry a triage keys the reviewer's finding `n` under, graded a break: A ROUTE ITEM, with
/// the route line a refusal printed.
fn a_route(key: &str, n: u32) -> Value {
    let mut entry = keyed(key, "route", "breaks", json!({"area-a-review": [n]}));
    entry["route"] = json!(format!("Run: jigc doc set --repair {key}"));
    entry
}

/// … and A BEHAVIOUR ITEM, with its commands.
fn a_behaviour(key: &str, n: u32) -> Value {
    let mut entry = keyed(key, "behaviour", "breaks", json!({"area-a-review": [n]}));
    entry["commands"] = json!(format!("jigc doc set {key}"));
    entry
}

/// What a verifier that files is scripted with: the members of its filing.
fn files(members: Value) -> Value {
    json!({"files": members})
}

/// A filing that refutes its finding, by one of the three words.
fn refutes(word: &str) -> Value {
    files(json!({"verdict": "refuted", "refutation": word}))
}

/// An item a verifier hands on: a route item with the line it saw printed — or, where `line`
/// is none, a route item WITHOUT its evidence, which is a lead.
fn handed(proposal: &str, line: Option<&str>) -> Value {
    let mut item = json!({"key": proposal, "doctype": "jigc-feedback", "door": "jigc doc set",
                          "clause": "audit-clean", "repro": "the verifier's report, its own heading",
                          "kind": "route"});
    if let Some(line) = line {
        item["route"] = json!(line);
    }
    item
}

/// A row of the state's ledger, by its key.
fn row_of(state: &Value, key: &str) -> Value {
    state["ledger"]
        .as_array()
        .expect("the ledger")
        .iter()
        .find(|row| row["key"] == key)
        .unwrap_or_else(|| panic!("the state has the row `{key}`: {state}"))
        .clone()
}

/// The keys of the queue, in its rank, as a state document has it.
fn queued_keys(state: &Value) -> Vec<String> {
    state["queue"]["ranked"]
        .as_array()
        .unwrap_or_else(|| panic!("the state has a queue: {state}"))
        .iter()
        .map(|entry| text(&entry["key"]))
        .collect()
}

/// The verifiers a stage launched, by the key each was launched for, in order.
fn verified(ran: &Ran) -> Vec<String> {
    ran.agents()
        .iter()
        .filter_map(|label| label.strip_prefix("verify:"))
        .map(str::to_owned)
        .collect()
}

/// The prompt the verifier of `key` was handed.
fn verifier_prompt<'a>(ran: &'a Ran, key: &str) -> &'a str {
    &ran.prompts
        .iter()
        .find(|(label, _)| *label == format!("verify:{key}"))
        .unwrap_or_else(|| panic!("the verifier of `{key}` was launched: {:?}", ran.agents()))
        .1
}

/// The lines of a stage's log that name `key` under `mark` — `NOT VERIFIED`, `NOT HELD`,
/// `CONTESTED` — which is where a stage of a run with a queue says WHICH finding: its return
/// holds how many, and no key.
fn said_of(ran: &Ran, mark: &str, key: &str) -> Vec<String> {
    ran.logs
        .iter()
        .filter(|line| line.starts_with(mark) && line.contains(&format!("`{key}`")))
        .cloned()
        .collect()
}

/// Where the record script files the verdict of `key`, attempt 1, in round 1.
fn filed_at(key: &str) -> String {
    format!("{RUN_DIR}/r1/verdicts/verify-{key}.a1.json")
}

/// The subject of the commit a `test` stage's second record is made by, in round 1.
fn second_record_subject() -> String {
    format!("docs(record): {RUN} r1 - the verdicts of the test stage")
}

/// THE STAGE'S SECOND RECORD (the ruling of 2026-10-11, 1: *a stage records twice*; the harness
/// half of `K3`): ONE more commit, on the first record, pushed, and nothing left pending —
/// composed BY THE TOOL from the filed verdicts and from plain words that name EVERY REPORTER
/// OF THE ATTEMPT THAT LEFT A REPORT and nothing else, held to the hash the harness computed;
/// then its gate, under a name of its own, its one commit and its push. No executor is
/// launched, and no verdict, no handed-on row and no view is a word. Returns the words.
fn ends_at_its_second_record(sim: &Sim, ran: &Ran, candidate: &str) -> Vec<String> {
    let result = &ran.result;
    assert_eq!(
        json!([result["status"], result["halted"]]),
        json!(["verified", null]),
        "{result}"
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([result["verify"]["record"], result["record"]]),
        json!([head, sim.rig.rev("HEAD~1")]),
        "the return names both records: {result}"
    );
    assert_eq!(
        sim.rig.rev("HEAD~2"),
        candidate,
        "two records, each ONE commit, on the candidate"
    );
    assert_eq!(
        sim.rig.remote(LOOP).as_deref(),
        Some(head.as_str()),
        "the second record is pushed"
    );
    assert_eq!(sim.rig.status(), "", "nothing is left pending");
    assert_eq!(head_commit(sim).0, second_record_subject());
    let agents = ran.agents();
    let gate = "record-verdicts-r1-a1-1";
    assert_eq!(
        agents[agents.len() - 6..agents.len() - 1],
        [
            "git:compose:verdicts:r1",
            format!("git:hold-start:{gate}").as_str(),
            format!("git:hold-wait:{gate}").as_str(),
            "git:record:verdicts:r1",
            "git:push:verdicts",
        ],
        "the second record is tool steps only: composed, gated, committed, pushed, read back: {agents:?}"
    );
    // WHAT COMES BEFORE THE COMPOSING STEP IS THE READ OF THE QUEUE AFTER THE LAST WAVE, whose
    // own report check came before that read: the check a record is preceded by is the
    // wave's, and no second one is asked for where a wave ran.
    assert!(
        agents[agents.len() - 7].starts_with("git:state:test-")
            && agents[agents.len() - 8].starts_with("git:check-reports:wave")
            && !agents.contains(&FILED_CHECK),
        "after a wave the record is preceded by that wave's report check, and by no other: {agents:?}"
    );
    assert!(
        agents[agents.len() - 1].starts_with("git:state:test-")
            && !agents.iter().any(|label| label.starts_with("record:"))
            && !ran
                .commands
                .iter()
                .any(|command| command.starts_with("dev/stabilize-record apply ")),
        "no executor is launched and no batch is typed: {agents:?}"
    );
    let composed = composing(ran);
    assert_eq!(
        composed.len(),
        2,
        "one step composes each record: {composed:?}"
    );
    let (flags, words) = &composed[1];
    assert!(
        flags.contains(&format!(
            " --branch {LOOP} --run-dir {RUN_DIR} --round 1 --attempt 1 --subject '{}' --sha256 ",
            second_record_subject()
        )),
        "{flags}"
    );
    assert_eq!(
        flag_of(flags, "--sha256"),
        sha256(&canonical(1, 1, &second_record_subject(), words)),
        "the hash the harness computed is the sha256 of the words' canonical form: {words:?}"
    );
    assert!(
        words.iter().all(|word| word.starts_with("reporter=")),
        "the second record's words name the reporters, and nothing else: no verdict, no row and no view is a word: {words:?}"
    );
    words.clone()
}

/// **Handed-on items are verified in the stage that found them, and the stage ends at its
/// budget** (the reshape's plan, `T6a`; ruling 1 of 2026-10-10: *the queue is ranked by what an
/// item is about … one budget*; ruling 4: *a verdict is recorded when it returns*; the ruling of
/// 2026-10-11, 2: *a verifier hands on at most N*). After its first record the stage VERIFIES,
/// IN WAVES: one step reads the queue's digest; a verifier is launched for each key it prints —
/// never more than the budget has left; each is handed ITS KEY and the read that prints its
/// row, the two binaries, the lookup kit's call and the ONE plain call that files its verdict;
/// the wave's reports are checked; and the queue is read again. WHAT A VERIFIER HANDS ON IS IN
/// THE QUEUE AT THE NEXT READ — a row of the state from the moment it is filed — and is verified
/// in the same stage, while budget is left. THE BUDGET IS THE SCRIPT'S COUNT: with it spent no
/// verifier is launched, and a route item nobody reached is the human's. Then THE SECOND
/// RECORD: the filed verdicts, the handed-on rows and what a stop shows, in one commit.
#[test]
fn handed_on_items_are_verified_in_the_stage_that_found_them_and_the_stage_ends_at_its_budget() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-budget", &one_area(), 3);
    let candidate = sim.rig.rev("HEAD");
    let scratch = sim.rig.scratch.display().to_string();
    let (first, second, third) = (
        "the-repair-route-is-dead",
        "the-next-route-is-dead-too",
        "a-third-route-nobody-reaches",
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "a slot is dropped"]),
            "triage": {"entries": [a_route(ROUTE, 1), a_behaviour(INSIDE, 2)]},
            // The route item is CONFIRMED, and its verifier hands on three: the first two that
            // carry their kind's evidence enter the queue, the third is beyond the number.
            format!("verify:{ROUTE}"): files(json!({
                "verdict": "confirmed", "regression": true,
                "hands_on": [
                    handed("the repair route is dead", Some("Run: jigc doc set --again")),
                    handed("the next route is dead, too", Some("Run: jigc doc set --once-more")),
                    handed("a third route nobody reaches", Some("Run: jigc doc set --third")),
                ],
            })),
            format!("verify:{INSIDE}"): refutes("intended"),
            format!("verify:{first}"): files(json!({"verdict": "confirmed", "regression": false})),
        }})),
    );

    // (1) THE WAVES: the first record, then the two rows the queue prints, in its rank; the
    // wave's reports checked and the queue read again; then, of the two the first verifier
    // handed on, THE ONE THE BUDGET HAS LEFT — and no third wave.
    assert_eq!(verified(&ran), [ROUTE, INSIDE, first], "{:?}", ran.agents());
    let agents = ran.agents();
    let at = |label: &str| {
        agents
            .iter()
            .position(|a| *a == label)
            .unwrap_or_else(|| panic!("`{label}` was launched: {agents:?}"))
    };
    assert!(
        at("git:push") < at(&format!("verify:{ROUTE}")),
        "the first record is committed and pushed before the first verifier starts: {agents:?}"
    );
    assert_eq!(
        agents[at("git:state:test-3")..at("git:compose:verdicts:r1")],
        [
            "git:state:test-3",
            format!("verify:{ROUTE}").as_str(),
            format!("verify:{INSIDE}").as_str(),
            "git:check-reports:wave1",
            "git:state:test-4",
            format!("verify:{first}").as_str(),
            "git:check-reports:wave2",
            "git:state:test-5",
        ],
        "a wave is its verifiers, its report check and the queue read again: {agents:?}"
    );
    for agent in ["advocate:", "proposal:", "triage:p", "record:"] {
        assert!(
            !agents.iter().any(|label| label.starts_with(agent)),
            "`{agent}…` is of the other path: {agents:?}"
        );
    }

    // (2) WHAT A VERIFIER IS HANDED: its key, and nothing of its row; the kit's call, first;
    // the read that prints the row; the two binaries; the ONE call that files its verdict,
    // spelled whole; the number it may hand on; and what its return is held to.
    let prompt = verifier_prompt(&ran, ROUTE);
    let kit = format!("`dev/stabilize-record kit --run {RUN} --role verify`");
    let row = format!("`dev/stabilize-record row --run {RUN} --key {ROUTE}`");
    let call = format!(
        "`dev/stabilize-record verdict --run {RUN} --round 1 --reporter verify-{ROUTE} --attempt 1 --scratch {scratch} --from {scratch}/verdict/r1/verify-{ROUTE}.a1.json`"
    );
    for spelled in [&kit, &row, &call] {
        assert!(
            prompt.contains(spelled.as_str()),
            "the prompt spells {spelled}:\n{prompt}"
        );
    }
    assert!(
        prompt.find(&kit) < prompt.find(&row) && prompt.find(&row) < prompt.find(&call),
        "the kit first, then the row, then the filing:\n{prompt}"
    );
    assert!(
        prompt.contains(&format!("held to `{}`", filed_at(ROUTE)))
            && prompt.contains("AT MOST 2 ENTER THE QUEUE")
            && prompt
                .lines()
                .filter(|line| line.starts_with("REPORT: ") || line.starts_with("BINARY: "))
                .count()
                == 2
            && prompt.contains("; previous release `"),
        "{prompt}"
    );
    for cell in [
        "Run: jigc doc set --repair",
        "the block of",
        "why doc-set",
        "breaks",
    ] {
        assert!(
            !prompt.contains(cell),
            "`{cell}` is text of the finding's row, and the prompt hands the key and the read: {prompt}"
        );
    }
    let (_, _, printed) = ran
        .reads
        .iter()
        .find(|(label, command, _)| {
            *label == format!("verify:{ROUTE}") && command.contains(" row --run ")
        })
        .expect("the verifier ran the read its prompt names");
    let printed: Value = serde_json::from_str(printed).expect("one line of JSON");
    assert_eq!(
        json!([printed["row"]["key"], printed["row"]["route"]]),
        json!([ROUTE, "verify"]),
        "the read prints the row whole, and the row awaits its verdict: {printed}"
    );
    // A HANDED-ON ITEM IS READ BY ITS KEY BEFORE ANY RECORD HOLDS ITS ROW.
    let (_, _, handed_row) = ran
        .reads
        .iter()
        .find(|(label, command, _)| {
            *label == format!("verify:{first}") && command.contains(" row --run ")
        })
        .expect("the verifier of the handed-on item ran the read its prompt names");
    let handed_row: Value = serde_json::from_str(handed_row).expect("one line of JSON");
    assert_eq!(
        json!([
            handed_row["row"]["key"],
            handed_row["row"]["handed_on"]["by"],
            handed_row["row"]["handed_on"]["recorded"]
        ]),
        json!([first, format!("verify-{ROUTE}"), false]),
        "{handed_row}"
    );

    // (3) THE QUEUE, AS THE STAGE READ IT: after the first record the two rows the triage sent
    // on; after the first wave the two the verifier handed on, AND NOT THE THIRD; after the
    // second the one nobody reached, with the budget spent.
    let read = |n: u32| {
        let state = sim.read_at(n);
        json!([
            queued_keys(&state),
            state["budget"]["spent"],
            state["budget"]["left"],
            state["next"]
        ])
    };
    assert_eq!(read(3), json!([[ROUTE, INSIDE], 0, 3, "verify"]));
    assert_eq!(read(4), json!([[first, second], 2, 1, "verify"]));
    // (The step after the last wave is the stage's own record: three verdicts are filed, and
    // no record took one. The human's step follows it — (5).)
    assert_eq!(read(5), json!([[second], 3, 0, "verify"]));

    // (4) THE SECOND RECORD: every reporter of the attempt that left a report, and what it
    // took — three filed verdicts, three handed-on rows, the round's triage, what a stop shows.
    let words = ends_at_its_second_record(&sim, &ran, &candidate);
    assert_eq!(
        words,
        [
            "reporter=attempt".to_owned(),
            "reporter=scope".to_owned(),
            "reporter=preflight".to_owned(),
            "reporter=area-a-review".to_owned(),
            "reporter=triage".to_owned(),
            format!("reporter=verify-{ROUTE}"),
            format!("reporter=verify-{INSIDE}"),
            format!("reporter=verify-{first}"),
        ]
    );
    let (_, paths) = head_commit(&sim);
    for rel in [
        "ledger.md".to_owned(),
        "r1/triage.md".to_owned(),
        "r1/view.md".to_owned(),
        format!("r1/verdicts/verify-{ROUTE}.a1.json"),
        format!("r1/verdicts/verify-{INSIDE}.a1.json"),
        format!("r1/verdicts/verify-{first}.a1.json"),
        format!("r1/reports/test/verify-{ROUTE}.a1.md"),
    ] {
        assert!(
            paths.contains(&format!("{RUN_DIR}/{rel}")),
            "`{rel}` is in the second record's commit: {paths:?}"
        );
    }
    assert!(
        !paths
            .iter()
            .any(|path| path.ends_with("r1/gate.md") || path.ends_with("r1/results.md")),
        "a second record names no gate and no item: {paths:?}"
    );

    // (5) WHAT IS ON RECORD: the three verdicts, by the script's count; the handed-on rows in
    // the ledger — the first verified, the second in the queue and the human's, the third
    // beyond the number and in no queue — and no verdict the budget did not have.
    let state = sim.state();
    assert_eq!(
        json!([state["budget"], state["next"], queued_keys(&state)]),
        json!([{"limit": 3, "spent": 3, "left": 0}, "rule", [second]]),
        "{state}"
    );
    let routes = |key: &str| {
        let row = row_of(&state, key);
        json!([row["grade"], row["route"], row["why"]])
    };
    assert_eq!(
        routes(ROUTE),
        json!(["regression", "fix", "regression"]),
        "{state}"
    );
    assert_eq!(
        routes(INSIDE),
        json!(["refuted", "recorded", "not-a-break"]),
        "{state}"
    );
    assert_eq!(
        routes(first),
        json!(["confirmed", "fix", "inside"]),
        "{state}"
    );
    assert_eq!(
        routes(second),
        json!(["ungraded", "human", "unreached"]),
        "{state}"
    );
    assert_eq!(
        routes(third),
        json!(["ungraded", "listed", "beyond-hand-on"]),
        "{state}"
    );
    assert!(
        state["filed"]
            .as_array()
            .expect("the filed verdicts")
            .iter()
            .all(|filed| filed["counts"] == true && filed["recorded"] == true),
        "every filed verdict counts, and the record holds it: {state}"
    );

    // (6) WHAT THE STAGE RETURNS: that it verified, how many waves and launches, THE SCRIPT'S
    // COUNT of the verdicts, what the budget has left — and `next`, the state's.
    let result = &ran.result;
    assert_eq!(
        json!([
            result["next"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["budget"]["left"],
            result["verify"]["unverified"],
            result["verify"]["unheld"],
            result["verify"]["contested"]
        ]),
        json!(["rule", 2, 3, 3, 0, 0, 0, 0]),
        "{result}"
    );
    assert!(
        ran.trace.iter().any(|line| line.trim() == "phase Verify"),
        "{:?}",
        ran.trace
    );
    let returned = result.to_string();
    for of_a_row in ["Run: jigc doc set", "--again", "the repair route is dead"] {
        assert!(
            !returned.contains(of_a_row),
            "`{of_a_row}` is text of a row or of a filing, and the harness's return holds it: {returned}"
        );
    }
}

/// **An item under a confirmed parent overtakes an older item of a lower class** (`T6a`; ruling
/// 1 of 2026-10-10: *route items come first, those under a confirmed parent ahead of the
/// rest*). THE RANK IS THE QUEUE'S, READ AGAIN AFTER EVERY WAVE — the harness holds no order
/// and no list. Nine rows await a verdict: the first wave is THE EIGHT THE QUEUE PRINTS, though
/// the budget has nine left — a wave is at most eight — and the ninth, a behaviour item, is
/// not in it. One of the eight is confirmed and hands an item on; at the next read THAT ITEM
/// STANDS AHEAD OF THE NINTH, which has waited since the first record, and takes the one
/// verdict the budget has left. The ninth is never launched.
#[test]
fn an_item_under_a_confirmed_parent_overtakes_an_older_item_of_a_lower_class() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-overtakes", &one_area(), 9);
    let candidate = sim.rig.rev("HEAD");
    let routes: Vec<String> = (1..=8).map(|n| format!("a-dead-route-{n}")).collect();
    let (older, child) = ("an-older-behaviour", "the-route-under-the-first");
    let titles: Vec<String> = (1..=9).map(|n| format!("finding {n}")).collect();
    let titles: Vec<&str> = titles.iter().map(String::as_str).collect();
    // The behaviour item is the triage's FIRST entry: the oldest row of the ledger.
    let mut entries = vec![a_behaviour(older, 9)];
    entries.extend(
        routes
            .iter()
            .enumerate()
            .map(|(n, key)| a_route(key, n as u32 + 1)),
    );
    let mut agents = json!({
        "area-a:review": findings(&titles),
        "triage": {"entries": entries},
        format!("verify:{child}"): files(json!({"verdict": "confirmed", "regression": false})),
    });
    agents[format!("verify:{}", routes[0])] = files(json!({
        "verdict": "confirmed", "regression": false,
        "hands_on": [handed("the route under the first", Some("Run: jigc doc set --under"))],
    }));
    for key in &routes[1..] {
        agents[format!("verify:{key}")] = refutes("does-not-reproduce");
    }
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": agents})),
    );

    // (1) THE FIRST WAVE IS THE EIGHT ROUTE ITEMS, in the queue's rank; the second is the
    // item handed on, alone; the behaviour item — older than it — is never launched.
    let mut launched = routes.clone();
    launched.push(child.to_owned());
    assert_eq!(verified(&ran), launched, "{:?}", ran.agents());
    let agents = ran.agents();
    let wave = agents
        .iter()
        .position(|label| *label == "git:check-reports:wave1")
        .expect("the first wave's report check");
    assert_eq!(
        agents[wave - 8..wave]
            .iter()
            .map(|label| label.trim_start_matches("verify:").to_owned())
            .collect::<Vec<_>>(),
        routes,
        "a wave is at most eight: {agents:?}"
    );

    // (2) THE QUEUE, AS THE STAGE READ IT: nine rows, the behaviour item last, the budget at
    // nine; then the handed-on item AHEAD OF IT, one verdict left; then the behaviour item
    // alone, unreached, the budget spent.
    let after_record = sim.read_at(3);
    let mut ranked = routes.clone();
    ranked.push(older.to_owned());
    assert_eq!(
        json!([queued_keys(&after_record), after_record["budget"]["left"]]),
        json!([ranked, 9])
    );
    let after_wave = sim.read_at(4);
    assert_eq!(
        json!([queued_keys(&after_wave), after_wave["budget"]["left"]]),
        json!([[child, older], 1]),
        "the item under the confirmed parent overtook the older item: {after_wave}"
    );
    assert!(
        row_of(&after_record, older)["key"] == older
            && !after_record["ledger"]
                .as_array()
                .expect("the ledger")
                .iter()
                .any(|row| row["key"] == child),
        "the behaviour item is a row since the first record, and the handed-on item is not"
    );

    // (3) AFTER THE STAGE: nine verdicts, the budget spent; the behaviour item nobody
    // reached holds nothing.
    ends_at_its_second_record(&sim, &ran, &candidate);
    let state = sim.state();
    let unreached = row_of(&state, older);
    assert_eq!(
        json!([
            state["budget"]["spent"],
            state["budget"]["left"],
            queued_keys(&state),
            unreached["route"],
            unreached["why"],
            row_of(&state, child)["grade"]
        ]),
        json!([9, 0, [older], "listed", "unreached", "confirmed"]),
        "{state}"
    );
    assert_eq!(
        json!([
            ran.result["verify"]["waves"],
            ran.result["verify"]["launched"],
            ran.result["verify"]["verdicts"]
        ]),
        json!([2, 9, 9]),
        "{}",
        ran.result
    );
}

/// **A lead, and an item under a parent that did not reproduce, are never launched** (`T6a`;
/// ruling 1 of 2026-10-10: *an item without a reproduction is a lead and is not verified …
/// nothing under a parent that did not reproduce is verified*). THE HARNESS LAUNCHES WHAT THE
/// QUEUE PRINTS AND NOTHING ELSE: a lead the triage graded *unclear* is in no queue; a route
/// item a verifier hands on under a finding it refuted as *does not reproduce* is a row at
/// once, in no queue; and an item handed on without its kind's evidence is a lead. One
/// verifier runs, and the phase ends with budget left and nothing to launch.
#[test]
fn a_lead_and_an_item_under_a_parent_that_did_not_reproduce_are_never_launched() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-never", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let (under, bare) = ("a-route-under-a-ghost", "a-route-nobody-quoted");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "a slot may be dropped"]),
            "triage": {"entries": [
                a_route(ROUTE, 1),
                keyed(HUNCH, "lead", "unclear", json!({"area-a-review": [2]})),
            ]},
            format!("verify:{ROUTE}"): files(json!({
                "verdict": "refuted", "refutation": "does-not-reproduce",
                "hands_on": [
                    handed("a route under a ghost", Some("Run: jigc doc set --ghost")),
                    handed("a route nobody quoted", None),
                ],
            })),
        }})),
    );
    assert_eq!(verified(&ran), [ROUTE], "{:?}", ran.agents());
    let agents = ran.agents();
    assert!(
        agents.contains(&"git:check-reports:wave1") && !agents.contains(&"git:check-reports:wave2"),
        "one wave: after it the queue prints nothing, and nothing is launched: {agents:?}"
    );
    let after_record = sim.read_at(3);
    assert_eq!(
        json!([
            queued_keys(&after_record),
            row_of(&after_record, HUNCH)["route"],
            row_of(&after_record, HUNCH)["why"]
        ]),
        json!([[ROUTE], "listed", "lead"]),
        "the lead is graded and in no queue: {after_record}"
    );
    let after_wave = sim.read_at(4);
    assert_eq!(
        json!([
            queued_keys(&after_wave),
            after_wave["budget"]["left"],
            after_wave["next"]
        ]),
        json!([[], 29, "verify"]),
        "the wave's verdict is filed, and until the stage's second record takes it the step is that record: {after_wave}"
    );
    ends_at_its_second_record(&sim, &ran, &candidate);
    let state = sim.state();
    assert_eq!(state["next"], "stop", "recorded: {state}");
    let routes = |key: &str| {
        let row = row_of(&state, key);
        json!([row["route"], row["why"]])
    };
    assert_eq!(routes(HUNCH), json!(["listed", "lead"]), "{state}");
    assert_eq!(
        routes(under),
        json!(["listed", "parent-did-not-reproduce"]),
        "{state}"
    );
    assert_eq!(routes(bare), json!(["listed", "lead"]), "{state}");
    assert_eq!(
        json!([
            ran.result["verify"]["waves"],
            ran.result["verify"]["launched"],
            ran.result["verify"]["verdicts"],
            ran.result["budget"]["left"]
        ]),
        json!([1, 1, 1, 29]),
        "{}",
        ran.result
    );
}

/// **A finding nobody here can drive is not launched again once that is filed** (`T6a`; ruling
/// 4 of 2026-10-10: *marked so once*; the row `wf-unverifiable-relaunch`). `undrivable` IS A
/// FILED VERDICT: it spends one of the budget, the finding is the human's at once, AND THE
/// QUEUE DOES NOT PRINT ITS KEY AGAIN IN THE ROUND — so the waves that follow launch no
/// verifier for it. THE HARNESS HOLDS NO LIST OF SUCH FINDINGS: beside it stands a finding
/// whose verifier HALTED, which the queue prints again, and which the next invocation's
/// verifier is launched for.
#[test]
fn a_finding_nobody_here_can_drive_is_not_launched_again_once_that_is_filed() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-undrivable", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let (undrivable, halted, child) = (
        "setup-needs-a-root-caller",
        "setup-nobody-could-reconstruct",
        "a-route-the-second-wave-drives",
    );
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a hook needs root", "a repro nobody can rebuild", "a refusal prints a dead route"]),
            "triage": {"entries": [a_route(undrivable, 1), a_route(halted, 2), a_route(ROUTE, 3)]},
            format!("verify:{undrivable}"): files(json!({"verdict": "undrivable", "reason": "needs-linux-root"})),
            format!("verify:{halted}"): {"status": "halted", "halt": {
                "root_cause": "the repro can be neither driven nor reconstructed",
                "evidence": "the block names a state it does not build", "tree_state": "clean",
                "recommendation": "a second verifier may reconstruct it"}},
            format!("verify:{ROUTE}"): files(json!({
                "verdict": "confirmed", "regression": false,
                "hands_on": [handed("a route the second wave drives", Some("Run: jigc doc set --second"))],
            })),
            format!("verify:{child}"): refutes("intended"),
        }})),
    );

    // (1) ONE LAUNCH EACH: the second wave is the handed-on item alone.
    assert_eq!(
        verified(&ran),
        [undrivable, halted, ROUTE, child],
        "{:?}",
        ran.agents()
    );

    // (2) AFTER THE FIRST WAVE THE QUEUE PRINTS THE FINDING WHOSE VERIFIER HALTED, AND NOT THE
    // ONE NOBODY CAN DRIVE — which is the human's, with a verdict that spent one of the budget.
    let after_wave = sim.read_at(4);
    let gone = row_of(&after_wave, undrivable);
    assert_eq!(
        json!([
            queued_keys(&after_wave),
            after_wave["budget"]["spent"],
            gone["route"],
            gone["why"]
        ]),
        json!([[child, halted], 2, "human", "undrivable"]),
        "{after_wave}"
    );
    assert!(
        after_wave["human_list"]
            .as_array()
            .expect("the human's list")
            .iter()
            .any(|entry| entry["key"] == undrivable && entry["why"] == "undrivable"),
        "{after_wave}"
    );

    // (3) AFTER THE STAGE: the same — on record. The halted verifier's finding awaits its
    // verdict again, counted by its report; nothing of the budget was spent on it.
    ends_at_its_second_record(&sim, &ran, &candidate);
    let state = sim.state();
    let (gone, waits) = (row_of(&state, undrivable), row_of(&state, halted));
    assert_eq!(
        json!([
            queued_keys(&state),
            state["budget"]["spent"],
            state["next"],
            gone["route"],
            gone["why"],
            waits["route"]
        ]),
        json!([[halted], 3, "verify", "human", "undrivable", "verify"]),
        "{state}"
    );
    assert!(
        sim.reports(1).contains(&format!("verify-{halted}.a1.md"))
            && !sim.rig.root.join(filed_at(halted)).exists()
            && sim.rig.root.join(filed_at(undrivable)).exists(),
        "a halt is reported and files nothing; `undrivable` is filed: {:?}",
        sim.reports(1)
    );
    assert_eq!(
        json!([
            ran.result["next"],
            ran.result["verify"]["verdicts"],
            ran.result["verify"]["unverified"]
        ]),
        json!(["verify", 3, 1]),
        "{}",
        ran.result
    );
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", halted),
        [format!(
            "NOT VERIFIED: `{halted}` — its verifier halted (the repro can be neither driven nor reconstructed), and the queue prints the finding again: no verdict of it counts. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "{:?}",
        ran.logs
    );
    assert!(
        said_of(&ran, "NOT VERIFIED", undrivable).is_empty()
            && said_of(&ran, "NOT HELD", undrivable).is_empty(),
        "a finding nobody can drive HAS its verdict: {:?}",
        ran.logs
    );
}

/// **A verifier that returns a verdict and filed none left no verdict** (`T6a`; the plan's
/// `B2`: *what makes a filed verdict count*). WHAT COUNTS IS WHAT IS FILED: a verifier's return
/// is the path and the word its filing's call printed, and the harness holds it to the path a
/// filing of that reporter and attempt has, to a word a filing says, and to `contested` as a
/// truth value — AND THEN TO THE QUEUE, READ AGAIN: a finding the queue prints again has no
/// verdict, whatever its verifier returned. So a verifier that wrote its report and returns the
/// path and the word of a filing IT NEVER MADE leaves its finding unverified — named, with why
/// — spends nothing, and the record holds no verdict for it; AND A RETURN THAT IS NOT A
/// FILING'S — another path — is named as not held, while the verdict its verifier did file
/// counts all the same: the script's file decides, and never a return.
#[test]
fn a_verifier_that_returns_a_verdict_and_filed_none_left_no_verdict() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-unfiled", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let elsewhere = "another-route-is-dead";
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "another route is dead", "a slot is dropped"]),
            "triage": {"entries": [a_route(ROUTE, 1), a_route(elsewhere, 2), a_behaviour(INSIDE, 3)]},
            // IT FILED NOTHING, AND SAYS IT DID: the path a filing of its name has, and a word.
            format!("verify:{ROUTE}"): {"returns": {"word": "confirmed", "contested": false}},
            // It filed, and returns another path than the one its call printed.
            format!("verify:{elsewhere}"): {"files": {"verdict": "confirmed", "regression": false},
                                           "returns": {"filing": format!("{RUN_DIR}/r1/verdicts/verify-{ROUTE}.a1.json")}},
            format!("verify:{INSIDE}"): refutes("breaks-no-clause"),
        }})),
    );
    assert_eq!(
        verified(&ran),
        [ROUTE, elsewhere, INSIDE],
        "one wave, and no finding is launched twice in an attempt: {:?}",
        ran.agents()
    );
    assert!(
        !ran.agents().contains(&"git:check-reports:wave2"),
        "{:?}",
        ran.agents()
    );

    // (1) AFTER THE WAVE THE QUEUE PRINTS THE FINDING AGAIN, and two verdicts are spent.
    let after_wave = sim.read_at(4);
    assert_eq!(
        json!([
            queued_keys(&after_wave),
            after_wave["budget"]["spent"],
            after_wave["next"]
        ]),
        json!([[ROUTE], 2, "verify"]),
        "{after_wave}"
    );

    // (2) THE STAGE SAYS SO, AND DOES NOT HALT: how many, in its return — the verdicts by the
    // script's count, which is two, never the three that came back — and in its log the
    // finding by its key, with what its verifier returned and that the queue prints it again;
    // and the other return, not held, by its key.
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["verify"]["verdicts"],
            result["verify"]["launched"],
            result["verify"]["unverified"],
            result["verify"]["unheld"]
        ]),
        json!(["verified", "verify", 2, 3, 1, 1]),
        "{result}"
    );
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", ROUTE),
        [format!(
            "NOT VERIFIED: `{ROUTE}` — its verifier returned the path and the word of a filing, and the queue prints the finding again: no verdict of it counts. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "{:?}",
        ran.logs
    );
    let unheld = said_of(&ran, "NOT HELD", elsewhere);
    assert!(
        unheld.len() == 1
            && unheld[0].contains("a path that is not where a verdict of `verify-another-route-is-dead`, attempt 1, is filed")
            && unheld[0].contains(&filed_at(elsewhere))
            && unheld[0].contains("the queue does not print the finding again"),
        "{:?}",
        ran.logs
    );
    assert!(
        said_of(&ran, "NOT", INSIDE).is_empty(),
        "a held return of a finding the queue does not print again is no line: {:?}",
        ran.logs
    );

    // (3) THE RECORD: two verdicts, and none for the finding whose verifier filed nothing —
    // its report is on record, its launch counted, and it awaits its verdict again.
    ends_at_its_second_record(&sim, &ran, &candidate);
    let state = sim.state();
    let (waits, taken) = (row_of(&state, ROUTE), row_of(&state, elsewhere));
    assert_eq!(
        json!([
            state["budget"]["spent"],
            queued_keys(&state),
            waits["route"],
            waits["grade"],
            taken["grade"],
            taken["route"]
        ]),
        json!([2, [ROUTE], "verify", "breaks", "confirmed", "fix"]),
        "{state}"
    );
    assert!(
        !sim.rig.root.join(filed_at(ROUTE)).exists()
            && sim.rig.root.join(filed_at(elsewhere)).exists()
            && sim.reports(1).contains(&format!("verify-{ROUTE}.a1.md")),
        "no file was filed for it, and its report stands: {:?}",
        sim.reports(1)
    );

    // (4) AND WHERE NO VERIFIER OF A STAGE FILED ANYTHING — here its one verifier HALTED, and
    // reported that it did — the second record would change nothing: the record script says so
    // (`no-change`), AND THAT IS NO HALT. The stage returns as it hunted; the halted verifier's
    // report is on disk and in no commit, pending like every report a stage left before a
    // record, and the next record's commit takes it. (A verifier that left no report either
    // is what every scenario scripts whose subject is another: `nobody_verifies`.)
    let sim = Sim::budgeted("verify-nothing-filed", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": one_finding(),
            "triage": one_entry(),
            format!("verify:{INSIDE}"): {"status": "halted", "halt": {
                "root_cause": "the clause the finding names is not in the closing condition",
                "evidence": "the kit's last part", "tree_state": "clean",
                "recommendation": "the finding names another clause"}},
        }})),
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"],
            result["next"],
            result["verify"]["record"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["verify"]["unverified"]
        ]),
        json!(["hunted", null, "verify", null, 1, 0, 1]),
        "{result}"
    );
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", INSIDE),
        [format!(
            "NOT VERIFIED: `{INSIDE}` — its verifier halted (the clause the finding names is not in the closing condition), and the queue prints the finding again: no verdict of it counts. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "{:?}",
        ran.logs
    );
    let composed = composing(&ran);
    let mut reporters: Vec<String> = composed[0]
        .1
        .iter()
        .filter(|word| word.starts_with("reporter="))
        .cloned()
        .collect();
    reporters.push(format!("reporter=verify-{INSIDE}"));
    assert!(
        composed.len() == 2 && composed[1].1 == reporters,
        "the second record was composed over every reporter that left a report, the halted verifier among them: {composed:?}"
    );
    assert!(
        ran.logs.iter().any(|line| line.contains("`no-change`"))
            && ran.agents().last() == Some(&"git:compose:verdicts:r1"),
        "it was refused as one that changes nothing, and nothing followed the step: {:?}",
        ran.agents()
    );
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            head_commit(&sim).0,
            result["record"],
            sim.rig.remote(LOOP)
        ]),
        json!([
            candidate,
            first_record_subject(),
            sim.rig.rev("HEAD"),
            sim.rig.rev("HEAD")
        ]),
        "the first record is the stage's only one"
    );
    assert_eq!(
        sim.rig.status(),
        format!("?? {RUN_DIR}/r1/reports/test/verify-{INSIDE}.a1.md"),
        "the halted verifier's report is pending: on disk, and in no commit yet"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["budget"]["spent"],
            queued_keys(&state),
            row_of(&state, INSIDE)["route"]
        ]),
        json!([0, [INSIDE], "verify"]),
        "{state}"
    );
}

/// **A `test` stage's records are composed by the tool, and no executor is launched** (the
/// plan's `K3`, its harness half — *a named part of `T5b` (the first record) and of `T6a` (the
/// second)*; `T5b` left the scenario to the task that has both records to name it by). The
/// third count priced a record at 118 k output tokens and 13.4 minutes of an executor that
/// re-typed its prompt. IN A RUN WITH A QUEUE BOTH RECORDS OF A STAGE ARE TOOL STEPS ONLY: one
/// step composes each from plain words held to a hash, one starts its gate and one asks after
/// it, one makes its commit, one pushes it — ten steps of `build-git`, and between the two
/// records the verifiers. No `build-executor` is launched in the whole stage, no batch is
/// typed into a file, and no call of `dev/stabilize-record apply` is made by anybody.
#[test]
fn a_test_stages_records_are_composed_by_the_tool_and_no_executor_is_launched() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-two-records", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": one_finding(),
            "triage": one_entry(),
            format!("verify:{INSIDE}"): refutes("does-not-reproduce"),
        }})),
    );
    ends_at_its_second_record(&sim, &ran, &candidate);
    assert!(
        !ran.trace.iter().any(|line| line.contains("build-executor")),
        "no executor is launched in the stage: {:?}",
        ran.trace
    );
    // THE TWO RECORDS, STEP BY STEP — each a `build-git` step of the tool, under the record's
    // own names — and the verifier between them.
    let steps: Vec<&str> = ran
        .agents()
        .into_iter()
        .skip_while(|label| *label != "git:compose:test:r1")
        .collect();
    assert_eq!(
        steps,
        [
            "git:compose:test:r1",
            "git:hold-start:record-test-r1-a1-1",
            "git:hold-wait:record-test-r1-a1-1",
            "git:record:test:r1",
            "git:push",
            "git:state:test-3",
            format!("verify:{INSIDE}").as_str(),
            "git:check-reports:wave1",
            "git:state:test-4",
            "git:compose:verdicts:r1",
            "git:hold-start:record-verdicts-r1-a1-1",
            "git:hold-wait:record-verdicts-r1-a1-1",
            "git:record:verdicts:r1",
            "git:push:verdicts",
            "git:state:test-5",
        ]
    );
    for step in steps.iter().filter(|label| label.starts_with("git:")) {
        let phase = if step.starts_with("git:state:") {
            "State"
        } else if step.starts_with("git:check-reports:") {
            "Verify"
        } else {
            "Record"
        };
        assert!(
            ran.trace
                .iter()
                .any(|line| line.trim() == format!("agent {step} · build-git · sonnet · {phase}")),
            "`{step}` is a step of the tool: {:?}",
            ran.trace
        );
    }
    // WHAT EACH WAS COMPOSED FROM: plain words, each record's own subject, and a hash.
    let composed = composing(&ran);
    assert_eq!(composed.len(), 2, "{composed:?}");
    for ((flags, words), subject) in composed
        .iter()
        .zip([first_record_subject(), second_record_subject()])
    {
        assert!(
            flags.contains(&format!(" --subject '{subject}' ")),
            "{flags}"
        );
        assert_eq!(
            flag_of(flags, "--sha256"),
            sha256(&canonical(1, 1, &subject, words))
        );
    }
    assert!(
        composed[0].1.iter().any(|word| word.starts_with("item="))
            && composed[1]
                .1
                .iter()
                .all(|word| word.starts_with("reporter="))
            && composed[1].1.contains(&format!("reporter=verify-{INSIDE}")),
        "the first says what each item did; the second names the reporters, the verifier among them: {composed:?}"
    );
    // AND BOTH COMMITS ARE THE TOOL'S: the first record's, then the second's, on the candidate.
    assert_eq!(
        sim.rig
            .git(&["log", "--format=%s", &format!("{candidate}..HEAD")])
            .lines()
            .collect::<Vec<_>>(),
        [second_record_subject(), first_record_subject()]
    );
}

/// **A finding no verifier can be named for is passed over, and named** (`T6a`; the builder's
/// naming). A verdict is filed under the reporter `verify-<key>`, which is a slug of at most a
/// hundred characters — and a key a triage minted may itself be that long. For such a row the
/// queue prints a key NO VERIFIER CAN BE LAUNCHED UNDER: the stage does not halt on it, as the
/// other path's pass does — its first record is committed — it passes the key over, launches
/// the rows behind it, and names the finding with why. The row stays in the queue, and `next`
/// says `verify` for as long as budget is left: what becomes of it is the human's to see.
#[test]
fn a_finding_no_verifier_can_be_named_for_is_passed_over_and_named() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-unnamed", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let long = format!("a-key-that-is-a-slug-and-too-long-{}", "x".repeat(62));
    assert!(long.len() == 96 && format!("verify-{long}").len() > 100);
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a route nobody can be sent to", "a slot is dropped"]),
            "triage": {"entries": [a_route(&long, 1), a_behaviour(INSIDE, 2)]},
            format!("verify:{INSIDE}"): refutes("intended"),
        }})),
    );
    assert_eq!(
        verified(&ran),
        [INSIDE],
        "the row behind it is launched, and no verifier for the key that is too long: {:?}",
        ran.agents()
    );
    assert_eq!(queued_keys(&sim.read_at(3)), [long.as_str(), INSIDE]);
    ends_at_its_second_record(&sim, &ran, &candidate);
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["verify"]["unverified"]
        ]),
        json!(["verified", "verify", 1, 1, 1, 1]),
        "{result}"
    );
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", &long),
        [format!(
            "NOT VERIFIED: `{long}` — no verifier can be named for it: a verdict is filed under the reporter `verify-<key>`, which for this key is no slug of at most 100 characters, or is a name this attempt has launched already. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "{:?}",
        ran.logs
    );
    let state = sim.state();
    assert_eq!(
        json!([queued_keys(&state), row_of(&state, &long)["route"]]),
        json!([[long], "verify"]),
        "{state}"
    );
}

/// **What still halts a stage once it verifies — and every such halt leaves the first record
/// standing and the filed verdicts counted** (`T6a`; the builder's naming). Three steps of the
/// phase have no way on: A WAVE'S REPORT CHECK that could not be read (`report-check`) — a
/// record is held to a report from every launched reporter, and nothing says who left one; THE
/// QUEUE, which could not be read after a wave (`read-back`) — nothing says what the wave
/// left; and THE SECOND RECORD, refused by any word but `no-change` (`record`) — here `words`,
/// twice: a step that altered a word, asked again once. In each the stage halts BY NAME, the
/// first record is the branch's head and is pushed, and WHAT THE VERIFIER FILED IS ON DISK AND
/// COUNTS: the state says the finding is verified and one verdict is spent, before any record
/// holds it.
#[test]
fn a_wave_that_cannot_be_checked_or_read_and_a_second_record_that_is_refused_each_halt_by_name() {
    if !can_run() {
        return;
    }
    const WORD: &str = "reporter=area-a-review";
    for (label, more, phase, names) in [
        (
            "verify-halt-check",
            json!({"endings": {"git:check-reports:wave1": "dies"}}),
            "reports",
            "after wave 1 of the stage's verifiers the report check could not be read",
        ),
        (
            "verify-halt-read",
            json!({"endings": {"git:state:test-4": "dies"}}),
            "state",
            "wave 1 of its verifiers ran, and the queue could not be read after it",
        ),
        (
            "verify-halt-record",
            json!({"mistypes": {
                "git:compose:verdicts:r1": [WORD, "reporter=area-a-reviewer"],
                "git:compose:verdicts:r1:again1": [WORD, "reporter=area-a-reviewer"],
            }}),
            "record",
            "the record was not composed: `dev/stabilize-record` refused the composing step `words`",
        ),
    ] {
        let sim = Sim::budgeted(label, &one_area(), 30);
        let candidate = sim.rig.rev("HEAD");
        let mut script = small_stage(json!({"agents": {
            "area-a:review": one_finding(),
            "triage": one_entry(),
            format!("verify:{INSIDE}"): files(json!({"verdict": "confirmed", "regression": false})),
        }}));
        for (name, value) in more.as_object().expect("more of a scenario") {
            script[name] = value.clone();
        }
        let ran = sim.invoke(sim.args("test", json!({})), script);
        let result = &ran.result;
        assert_eq!(
            json!([result["status"], result["halted"]["phase"]]),
            json!(["halted", phase]),
            "{label}: {result}"
        );
        assert!(
            text(&result["halted"]["reason"]).contains(names),
            "{label}: the halt says which step, and of which wave: {result}"
        );
        // THE FIRST RECORD STANDS, pushed — and nothing of a second is applied.
        let head = sim.rig.rev("HEAD");
        assert_eq!(
            json!([
                sim.rig.rev("HEAD~1"),
                head_commit(&sim).0,
                sim.rig.remote(LOOP)
            ]),
            json!([candidate, first_record_subject(), head]),
            "{label}"
        );
        assert!(
            sim.rig.status().lines().all(|line| line.starts_with("?? ")),
            "{label}: nothing is applied — what is untracked is the verifier's report and its filed verdict: {}",
            sim.rig.status()
        );
        // WHAT WAS FILED COUNTS, before any record holds it.
        let state = sim.state();
        let row = row_of(&state, INSIDE);
        assert_eq!(
            json!([
                state["budget"]["spent"],
                row["grade"],
                state["filed"][0]["counts"],
                state["filed"][0]["recorded"],
                sim.rig.root.join(filed_at(INSIDE)).exists()
            ]),
            json!([1, "confirmed", true, false, true]),
            "{label}: {state}"
        );
        if phase == "record" {
            assert_eq!(
                json!([result["halted"]["refused"], result["halted"]["refusal"]]),
                json!(["record", "words"]),
                "{label}: {result}"
            );
            assert_eq!(
                composing(&ran).len(),
                3,
                "{label}: the first record's step, and the second's twice"
            );
            assert!(
                !ran.agents()
                    .iter()
                    .any(|a| a.starts_with("git:hold-start:record-verdicts-")),
                "{label}: no gate is held on a record that was not composed: {:?}",
                ran.agents()
            );
        }
    }
}

/// **A contested verdict is the human's, and no advocate is launched on the new path** (the
/// plan's `T5b`, which could not write it: no verifier was launched; §2.9, ruled: *a contested
/// finding found in `test` reaches the human with its verifier's basis and no advocate's
/// case*). The verifier files `contested` WITH ITS BASIS beside its verdict; the finding is
/// the human's from that moment — never a fixer's on the verifier's word — and the stage
/// launches NO ADVOCATE AND NO INDEPENDENT DRIVE and types no word for it: the record takes
/// the filing with its mark. The stage's return names the finding by its key, as its verifier's
/// return says it. AND A RETURN THAT SAYS `contested` OVER A FILING THAT GAVE NO BASIS filed no
/// basis: that finding is routed by its verdict, and what tells the two apart is the state's
/// list, not the return — which this script cannot hold to the filing.
#[test]
fn a_contested_verdict_is_the_humans_and_no_advocate_is_launched_on_the_new_path() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-contested", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let basis = "the finding argues that a refusal may write: the decision of the write boundary says that it never does";
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a slot is dropped", "a refusal prints a route that leads nowhere"]),
            "triage": {"entries": [a_behaviour(INSIDE, 1), a_route(ROUTE, 2)]},
            format!("verify:{INSIDE}"): files(json!({"verdict": "confirmed", "regression": false, "contested": basis})),
            // It says `contested` in its return, and its filing gives no basis.
            format!("verify:{ROUTE}"): {"files": {"verdict": "confirmed", "regression": false},
                                       "returns": {"contested": true}},
        }})),
    );
    let agents = ran.agents();
    for other in ["advocate:", "proposal:", "triage:p"] {
        assert!(
            !agents.iter().any(|label| label.starts_with(other)),
            "`{other}…` is the other path's: a contested verdict has no case and no drive here: {agents:?}"
        );
    }
    assert_eq!(verified(&ran), [ROUTE, INSIDE], "{agents:?}");
    let (_, _, line) = ran
        .reads
        .iter()
        .find(|(label, command, _)| {
            *label == format!("verify:{INSIDE}") && command.contains(" verdict --run ")
        })
        .expect("the verifier filed");
    let line: Value = serde_json::from_str(line).expect("one line of JSON");
    assert_eq!(
        json!([line["word"], line["contested"]]),
        json!(["confirmed", true])
    );

    // (1) THE FINDING IS THE HUMAN'S FROM THE MOMENT IT IS FILED — before any record holds it.
    let after_wave = sim.read_at(4);
    let contested = row_of(&after_wave, INSIDE);
    assert_eq!(
        json!([
            contested["route"],
            contested["why"],
            after_wave["human_list"]
        ]),
        json!(["human", "contested", [{"key": INSIDE, "why": "contested", "basis": basis}]]),
        "{after_wave}"
    );

    // (2) THE RECORD TAKES IT WITH ITS MARK, and no word says it: the basis stays in the
    // filed file, the round's row carries the mark, and a stop lists it.
    let words = ends_at_its_second_record(&sim, &ran, &candidate);
    assert!(
        !words
            .iter()
            .any(|word| word.contains("contested") || word.contains("fork")),
        "{words:?}"
    );
    let state = sim.state();
    let (contested, by_its_verdict) = (row_of(&state, INSIDE), row_of(&state, ROUTE));
    assert_eq!(
        json!([
            state["next"],
            state["human_list"],
            contested["route"],
            contested["triage"]["fork"],
            contested["triage"]["contested"],
            by_its_verdict["route"],
            by_its_verdict["why"]
        ]),
        json!([
            "rule",
            [{"key": INSIDE, "why": "contested", "basis": basis}],
            "human",
            null,
            {"by": format!("verify-{INSIDE}"), "basis": basis},
            "fix",
            "inside"
        ]),
        "{state}"
    );
    let triage = sim.rig.read(&format!("{RUN_DIR}/r1/triage.md"));
    assert!(
        triage
            .lines()
            .any(|line| line.starts_with(&format!("| `{INSIDE}` |"))
                && line.ends_with("| contested |")),
        "{triage}"
    );
    let view = sim.rig.read(&format!("{RUN_DIR}/r1/view.md"));
    assert!(
        view.contains(&format!(
            "- `{INSIDE}` — confirmed, inside the round's test set — `verify-{INSIDE}`: {basis}"
        )),
        "{view}"
    );

    // (3) WHAT THE STAGE RETURNS: how many findings their verifier's RETURN says are contested
    // — two, each named in the log — and the count the state gives of the human's list,
    // which is one. The finding whose filing gave no basis is among the first and not on the
    // second: it is a fixer's.
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["rule"]["findings"],
            result["verify"]["contested"],
            result["counts"]["blockers"]
        ]),
        json!(["verified", "rule", 1, 2, 1]),
        "{result}"
    );
    for key in [ROUTE, INSIDE] {
        let said = said_of(&ran, "CONTESTED", key);
        assert!(
            said.len() == 1
                && said[0].contains("where that list does not name it so, the filing gave none, and the finding is routed by its verdict"),
            "the log names `{key}` as contested by its verifier's return, and says what holds: {:?}",
            ran.logs
        );
    }
    assert!(
        !result.to_string().contains("write boundary")
            && !ran.logs.iter().any(|line| line.contains("write boundary")),
        "the basis is text of a filing, and the harness holds none of it: {result}"
    );
}

// ---------------------------------------------------------------------------
// A run with a queue: an invocation that continues, and a wave that is the last (the
// reshape's plan, `T6b`)
// ---------------------------------------------------------------------------

/// How AN INVOCATION THAT CONTINUES a `test` stage opens (the reshape's plan, §3 and `T6b`):
/// its two reads, its attempt begun on record, and THE TWO BINARIES — each asked for under
/// its commit's name and answered by the job that stands: a start step, and no wait. No
/// preflight, no scope step, no gate of the candidate's, no held check and no instrument.
fn continuation_opens(sim: &Sim, candidate: &str) -> Vec<String> {
    let mut agents = lines(&["git:state", "git:state:test-1", "git:begin"]);
    agents.push(format!("git:hold-start:{}", candidate_build(candidate)));
    agents.push(format!("git:hold-start:{}", previous_build(sim)));
    agents
}

/// The steps of a record an invocation that continues composes in round 1, under `name` —
/// `graded`: the rows it graded; `verdicts`: what its verifiers filed, and what an earlier
/// invocation's had — in its attempt, and the read of the state that follows the push.
fn continuation_records(name: &str, attempt: u32, read: u32) -> Vec<String> {
    let gate = format!("record-{name}-r1-a{attempt}-1");
    vec![
        format!("git:compose:{name}:r1"),
        format!("git:hold-start:{gate}"),
        format!("git:hold-wait:{gate}"),
        format!("git:record:{name}:r1"),
        format!("git:push:{name}"),
        format!("git:state:test-{read}"),
    ]
}

/// A verifier that HALTS, WITH ITS REPORT: nothing is filed, and its launch is one the record
/// script counts — by that report.
fn halts_reported(why: &str) -> Value {
    json!({"status": "halted", "halt": {"root_cause": why, "evidence": "scripted",
                                        "tree_state": "clean", "recommendation": "none"}})
}

/// How often the agent `label` was tried in an invocation: every try is a line of the trace.
fn tries_of(ran: &Ran, label: &str) -> usize {
    ran.agents().iter().filter(|a| **a == label).count()
}

/// The words the stage's one triage was handed its sources with: the `<source>=<count>` words
/// of the call it made.
fn triaged_from(ran: &Ran) -> Vec<String> {
    let call = ran
        .commands
        .iter()
        .find(|command| command.starts_with("dev/stabilize-record triage "))
        .unwrap_or_else(|| panic!("the triage handed its entries over: {:?}", ran.commands));
    call.split_once(" -- ")
        .expect("the call names its sources")
        .1
        .split(' ')
        .map(str::to_owned)
        .collect()
}

/// **A verdict filed by an invocation that died is not verified again** (the reshape's plan,
/// `T6b`; ruling 4 of 2026-10-10: *a verdict is recorded when it returns … nothing runs twice*;
/// §3: *an invocation that continues … reads the queue, takes the verdicts already filed, and
/// launches only what is missing*). The canary's second invocation lost nineteen verdicts, six
/// of them confirmed, and verified every one of them again. HERE THE FIRST INVOCATION DIES
/// AFTER ITS VERIFIERS FILED — its wave's report check never comes back — with one finding
/// confirmed, one that NOBODY HERE CAN DRIVE, and one whose verifier halted. THE INVOCATION
/// THAT FOLLOWS CONTINUES: no instrument and no triage; the two filed verdicts count as they
/// stand — the state said so before any record held them — and ONE verifier is launched, for
/// the finding that has none: under the continuation's own attempt, so that its report and
/// its filing are names the record script takes. No verifier is launched again for the
/// finding nobody can drive (`B4`: *none after `undrivable` is filed*). Its ONE record takes
/// what both invocations filed, and the report the halted verifier of the dead one left.
#[test]
fn a_verdict_filed_by_an_invocation_that_died_is_not_verified_again() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("continue-filed", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let elsewhere = "another-route-is-dead";
    let mut script = small_stage(json!({"agents": {
        "area-a:review": findings(&["a refusal prints a route that leads nowhere", "another route is dead", "a slot is dropped"]),
        "triage": {"entries": [a_route(ROUTE, 1), a_route(elsewhere, 2), a_behaviour(INSIDE, 3)]},
        format!("verify:{ROUTE}"): files(json!({"verdict": "confirmed", "regression": false})),
        format!("verify:{elsewhere}"): files(json!({"verdict": "undrivable", "reason": "needs-a-linux-host"})),
        format!("verify:{INSIDE}"): halts_reported("the session ended while it drove"),
    }}));
    script["endings"] = json!({"git:check-reports:wave1": "dies"});
    let died = sim.invoke(sim.args("test", json!({})), script);
    assert_eq!(
        json!([died.result["status"], died.result["halted"]["phase"]]),
        json!(["halted", "reports"]),
        "{}",
        died.result
    );
    let first_record = sim.rig.rev("HEAD");
    assert_eq!(
        json!([sim.rig.rev("HEAD~1"), head_commit(&sim).0]),
        json!([candidate, first_record_subject()]),
        "the dead invocation left its first record, and nothing after it"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["budget"]["spent"],
            queued_keys(&state),
            state["position"]["test"],
            state["next"]
        ]),
        json!([2, [INSIDE], {"round": 1, "attempt": 2, "verify": true, "filed": true}, "verify"]),
        "two verdicts are filed and count, in no commit — the position says that they are owed a record; one row awaits its verdict: {state}"
    );
    assert!(
        state["filed"]
            .as_array()
            .is_some_and(|filed| filed.len() == 2
                && filed
                    .iter()
                    .all(|entry| entry["counts"] == true && entry["recorded"] == false)),
        "{state}"
    );

    // THE INVOCATION THAT FOLLOWS CONTINUES: its reads, its attempt, the binaries that stand —
    // and ONE verifier, for the finding that has no verdict.
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {format!("verify:{INSIDE}"): refutes("intended")}}),
    );
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(format!("verify:{INSIDE}"));
    expected.extend(lines(&["git:check-reports:wave1", "git:state:test-2"]));
    expected.extend(continuation_records("verdicts", 2, 3));
    assert_eq!(
        ran.agents(),
        expected,
        "no instrument, no triage, no verifier for a finding whose verdict is filed — the one that nobody here can drive among them"
    );
    assert_eq!(verified(&ran), [INSIDE]);
    assert_eq!(
        ran.result["arrived"]["owed"],
        json!(["filed"]),
        "the first read met the filed verdicts no commit holds, and left them to the stage: {}",
        ran.result
    );
    let scratch = sim.rig.scratch.display().to_string();
    let prompt = verifier_prompt(&ran, INSIDE);
    assert!(
        prompt.contains(&format!(
            "`dev/stabilize-record verdict --run {RUN} --round 1 --reporter verify-{INSIDE} --attempt 2 --scratch {scratch} --from {scratch}/verdict/r1/verify-{INSIDE}.a2.json`"
        )) && prompt.contains(&format!(
            "held to `{RUN_DIR}/r1/verdicts/verify-{INSIDE}.a2.json`"
        )) && prompt.contains(&format!("(commit {candidate}, label c1)")),
        "the verifier files under the continuation's own attempt, and drives the candidate the round tested:\n{prompt}"
    );

    // WHAT IT RETURNS: that it continued, and what it did — one launch, one verdict since it
    // began, by the script's count — and its one record.
    let result = &ran.result;
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            result["status"],
            result["halted"],
            result["continued"],
            result["record"],
            result["verify"]["record"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["verify"]["unverified"]
        ]),
        json!([
            "verified",
            null,
            ["verify", "filed"],
            null,
            head,
            1,
            1,
            1,
            0
        ]),
        "{result}"
    );
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            head_commit(&sim).0,
            sim.rig.remote(LOOP),
            sim.rig.status()
        ]),
        json!([first_record, second_record_subject(), head, ""]),
        "ONE record, on the dead invocation's first, pushed, and nothing left pending"
    );
    let composed = composing(&ran);
    assert!(
        composed.len() == 1
            && composed[0].0.contains(" --round 1 --attempt 2 --subject ")
            && composed[0].1
                == [
                    "reporter=attempt".to_owned(),
                    format!("reporter=verify-{INSIDE}")
                ],
        "its record names its own reporters, and no item, no gate and no fact: {composed:?}"
    );
    let (_, paths) = head_commit(&sim);
    for rel in [
        format!("r1/verdicts/verify-{ROUTE}.a1.json"),
        format!("r1/verdicts/verify-{elsewhere}.a1.json"),
        format!("r1/verdicts/verify-{INSIDE}.a2.json"),
        format!("r1/reports/test/verify-{ROUTE}.a1.md"),
        // THE REPORT A HALTED VERIFIER OF THE DEAD INVOCATION LEFT, which no record had taken.
        format!("r1/reports/test/verify-{INSIDE}.a1.md"),
        format!("r1/reports/test/verify-{INSIDE}.a2.md"),
        "r1/reports/test/attempt.a2.md".to_owned(),
    ] {
        assert!(
            paths.contains(&format!("{RUN_DIR}/{rel}")),
            "`{rel}` is in the continuation's record: {paths:?}"
        );
    }

    // WHAT IS ON RECORD: three verdicts, each filed once; the attempt reached its record.
    let state = sim.state();
    let routes = |key: &str| {
        let row = row_of(&state, key);
        json!([row["grade"], row["route"], row["why"]])
    };
    assert_eq!(
        json!([
            state["budget"]["spent"],
            queued_keys(&state),
            state["rounds"][0]["facts"]["recorded"],
            state["rounds"][0]["test_unrecorded"],
            state["next"],
            state["human_list"]
        ]),
        json!([3, [], ["test a2"], 0, "rule", [{"key": elsewhere, "why": "undrivable"}]]),
        "{state}"
    );
    assert_eq!(
        routes(ROUTE),
        json!(["confirmed", "fix", "inside"]),
        "{state}"
    );
    assert_eq!(
        routes(INSIDE),
        json!(["refuted", "recorded", "not-a-break"]),
        "{state}"
    );
    assert!(
        state["filed"]
            .as_array()
            .is_some_and(|filed| filed.len() == 3
                && filed
                    .iter()
                    .all(|entry| entry["counts"] == true && entry["recorded"] == true)),
        "every filed verdict counts, and the record holds it: {state}"
    );
}

/// **A stage killed after its first record is continued without an instrument** (`T6b`; §3:
/// *an invocation that continues runs no instrument*; ruling 4: *nothing runs twice* — the
/// canary's second invocation ran 45 minutes of gate and regression set again). HERE THE
/// STAGE IS KILLED BETWEEN ITS FIRST RECORD'S BATCH AND ITS COMMIT: the record's gate leaves no
/// verdict. THE NEXT INVOCATION'S FIRST READ FINDS THE BATCH, gates and commits it, pushes, and
/// starts nothing (`finishRecord`, which reads no path of a stage). AND THE ONE AFTER THAT
/// CONTINUES, from the committed state and from git: its two reads, its attempt begun on the
/// candidate the round tested, the two binaries answered by the builds that stand — and then
/// the verifiers the queue prints, and its one record. NO PREFLIGHT, NO SCOPE STEP, NO GATE OF
/// THE CANDIDATE'S, NO HELD CHECK, NO REVIEWER AND NO TRIAGE: the only long command that runs
/// is its own record's gate.
#[test]
fn a_stage_killed_after_its_first_record_is_continued_without_an_instrument() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued("continue-killed", &with_held_checks(&["regression-set"]));
    let candidate = sim.rig.rev("HEAD");
    let killed = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({
            "agents": {"area-a:review": one_finding(), "triage": one_entry()},
            "record": {"gate": {"void": true}},
        })),
    );
    assert_eq!(
        json!([
            killed.result["status"],
            killed.result["halted"]["phase"],
            killed.result["halted"]["held"]["name"],
            sim.rig.rev("HEAD")
        ]),
        json!(["halted", "record", "record-test-r1-a1-1", candidate]),
        "the first record's batch is applied, and no commit holds it: {}",
        killed.result
    );
    assert!(verified(&killed).is_empty(), "{:?}", killed.agents());

    // (1) THE NEXT INVOCATION FINISHES THE RECORD, AND NOTHING ELSE.
    let finished = sim.invoke(sim.args("test", json!({})), json!({}));
    let mut finishing = lines(&["git:state"]);
    finishing.extend(held_steps("record-pending-1"));
    finishing.extend(lines(&[
        "git:record:pending",
        "git:push:pending",
        "git:state:test-1",
    ]));
    assert_eq!(finished.agents(), finishing);
    assert_eq!(
        json!([
            finished.result["status"],
            finished.result["next"],
            sim.rig.rev("HEAD~1"),
            head_commit(&sim).0,
            sim.rig.remote(LOOP)
        ]),
        json!([
            "recorded",
            "verify",
            candidate,
            first_record_subject(),
            sim.rig.rev("HEAD")
        ]),
        "the stage's first record, committed and pushed by the invocation that found it applied: {}",
        finished.result
    );
    let first_record = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            state["position"]["test"],
            state["budget"]["spent"],
            queued_keys(&state)
        ]),
        json!([{"round": 1, "attempt": 2, "verify": true}, 0, [INSIDE]]),
        "{state}"
    );
    // AN INVOCATION THAT WOULD CONTINUE TAKES NO SCOPE, AND ONE THAT ONLY LOOKS BEGINS NOTHING:
    // each ends after the two reads, and the position is what it was.
    let position = state["position"]["test"].clone();
    let refused = sim.invoke(sim.args("test", json!({"scope": "everything"})), json!({}));
    assert_eq!(
        json!([refused.result["status"], refused.result["position"]]),
        json!(["refused", position]),
        "{}",
        refused.result
    );
    assert!(
        text(&refused.result["message"]).contains("it was passed scope")
            && text(&refused.result["message"]).contains("Nothing was run beyond the two reads"),
        "{}",
        refused.result
    );
    let looked = sim.invoke(sim.args("test", json!({"stopAfter": "state"})), json!({}));
    assert_eq!(
        json!([
            looked.result["status"],
            looked.result["after"],
            looked.result["continues"],
            looked.result["attempt"],
            looked.result["candidate"]["sha"]
        ]),
        json!(["stopped", "state", ["verify"], 2, candidate]),
        "{}",
        looked.result
    );
    for ran in [&refused, &looked] {
        assert_eq!(ran.trace, lines(&TWO_READS));
    }
    assert_eq!(
        sim.state()["position"]["test"],
        position,
        "no attempt was begun"
    );
    let before = sim.long_commands();

    // (2) AND THE ONE AFTER IT CONTINUES.
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {format!("verify:{INSIDE}"): files(json!({"verdict": "confirmed", "regression": false}))}}),
    );
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(format!("verify:{INSIDE}"));
    expected.extend(lines(&["git:check-reports:wave1", "git:state:test-2"]));
    expected.extend(continuation_records("verdicts", 2, 3));
    assert_eq!(
        ran.agents(),
        expected,
        "the two reads, the attempt, the binaries that stand, the verifier, its report checked, the queue read, the record"
    );
    assert_eq!(
        ran.reporters,
        ["attempt".to_owned(), format!("verify-{INSIDE}")],
        "the reporters of an invocation that continues are its marker and its verifiers"
    );
    let after = sim.long_commands();
    assert_eq!(
        after[before.len()..],
        ["gate --keep-going".to_owned()],
        "no build, no gate of the candidate's and no regression set runs again: the one long command is the record's own gate"
    );
    let begun = ran
        .commands
        .iter()
        .find(|command| command.starts_with("dev/stabilize-step begin "))
        .expect("the attempt is begun on record");
    assert!(
        begun.contains(&format!(
            " --attempt 2 --reporter attempt --commit {candidate} "
        )) && verifier_prompt(&ran, INSIDE).contains(&format!("(commit {candidate}, label c1)")),
        "the attempt is begun on, and the verifier drives, THE CANDIDATE THE ROUND TESTED — not the record commit {first_record}: {begun}"
    );
    let result = &ran.result;
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["candidate"]["sha"],
            result["verify"]["record"],
            result["verify"]["verdicts"],
            result["next"]
        ]),
        json!(["verified", ["verify"], candidate, head, 1, "fix"]),
        "{result}"
    );
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            head_commit(&sim).0,
            sim.rig.remote(LOOP),
            sim.rig.status()
        ]),
        json!([first_record, second_record_subject(), head, ""])
    );
    let state = sim.state();
    assert_eq!(
        json!([
            row_of(&state, INSIDE)["grade"],
            state["budget"]["spent"],
            standing(&state, "gate"),
            standing(&state, "regression-set")
        ]),
        json!(["confirmed", 1, ["green", null], ["green", null]]),
        "the verdict is on record, and what the stage held stands as its first record left it: {state}"
    );
}

/// **A continuation grades a row nobody graded, and no row a second time** (`T6b`; the plan's
/// review, `B1`: *a row that appears after the one triage has no grader* — a red row a batch
/// filed under its own key while the triage folded the red check under another). The first
/// invocation leaves such a row: `next` is `triage`, and the position says so beside
/// `verify`. THE INVOCATION THAT FOLLOWS GRADES IT FIRST: its triage is handed ONE source —
/// the rows nobody graded, by the read that prints them — and no reporter's finding, no red
/// result and NO ROW A TRIAGE HAS GRADED: the read prints the one row, and the entries the
/// triage hands over are one. THEN ITS RECORD OF WHAT IT GRADED, composed by the tool from
/// that attempt's rendering; then the queue, which now holds the row beside the one the first
/// invocation left unverified; then its verifiers, and the record of what they filed.
#[test]
fn a_continuation_grades_a_row_nobody_graded_and_no_row_a_second_time() {
    if !can_run() {
        return;
    }
    let sim = Sim::queued(
        "continue-triage",
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
    let folded = "the-gate-is-red";
    let mut minted = keyed(folded, "behaviour", "no-break", json!({"red-gate": [1]}));
    minted["commands"] = json!("dev/gate");
    // The triage folds the red gate under a key of its own minting: the batch files the red
    // result's row under ITS key, `red-gate`, and nobody graded that row. The late red check
    // is graded a break under its own key, and its verifier leaves nothing.
    let first = sim.invoke(
        sim.args("test", json!({})),
        nobody_verifies(
            small_stage(json!({
                "checks": {"gate": "red", "ci": "red"},
                "agents": {"area-a:review": nothing(), "triage": {"entries": [
                    minted,
                    found_again("red-ci", "breaks", json!({"red-ci": [1]})),
                ]}},
            })),
            &["red-ci"],
        ),
    );
    ends_at_its_first_record(&sim, &first, &candidate, 1, "triage");
    let first_record = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            state["position"]["test"],
            state["untriaged"],
            row_of(&state, "red-gate")["grade"],
            queued_keys(&state)
        ]),
        json!([
            {"round": 1, "attempt": 2, "triage": true, "verify": true},
            [{"key": "red-gate", "why": "ungraded"}],
            "ungraded",
            ["red-ci"]
        ]),
        "{state}"
    );
    let graded_before = sim.rig.read(&format!("{RUN_DIR}/r1/triage.md"));

    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            "triage": {"entries": [found_again("red-gate", "breaks", json!({"ledger": [1]}))]},
            "verify:red-ci": refutes("intended"),
            "verify:red-gate": files(json!({"verdict": "confirmed", "regression": false})),
        }}),
    );
    let mut expected = continuation_opens(&sim, &candidate);
    expected.extend(lines(&["triage", "git:check-reports:triage"]));
    expected.extend(continuation_records("graded", 2, 2));
    expected.extend(lines(&[
        // (In the queue's rank: two rows of one class, in the ledger's order.)
        "verify:red-gate",
        "verify:red-ci",
        "git:check-reports:wave1",
        "git:state:test-3",
    ]));
    expected.extend(continuation_records("verdicts", 2, 4));
    assert_eq!(
        ran.agents(),
        expected,
        "the triage of the rows nobody graded, its record, the verifiers of what the queue then prints, their record — and no instrument"
    );

    // (1) WHAT THE TRIAGE WAS HANDED: one source, the rows nobody graded, by the read — and
    // that read printed the one row. No graded row is in it, and none is graded again.
    assert_eq!(
        triaged_from(&ran),
        ["ledger=1"],
        "the triage of an invocation that continues is handed the rows nobody graded, and nothing else"
    );
    let prompt = one_triage(&ran);
    assert!(
        !prompt.contains("\n- red-")
            && !prompt.contains("\n- area-a-review")
            && prompt.contains("THE ROWS NOBODY GRADED"),
        "no red result of an earlier invocation and no reporter's finding is handed over again:\n{prompt}"
    );
    let (_, _, printed) = ran
        .reads
        .iter()
        .find(|(label, command, _)| {
            label == "triage" && command.starts_with("dev/stabilize-record untriaged ")
        })
        .expect("the triage ran the read its prompt names");
    let printed: Value = serde_json::from_str(printed).expect("one line of JSON");
    assert_eq!(
        json!([printed["count"], printed["untriaged"][0]["key"]]),
        json!([1, "red-gate"]),
        "the read prints the row nobody graded, and neither row a triage graded: {printed}"
    );
    let entries = sim.rendered(1, 2);
    assert_eq!(
        json!([
            entries["entries"].as_array().map(Vec::len),
            entries["entries"][0]["key"],
            entries["entries"][0]["found"]
        ]),
        json!([1, "red-gate", "again"]),
        "the continuation's rendering holds one entry, under the row's own key: {entries}"
    );
    let graded_after = sim.rig.read(&format!("{RUN_DIR}/r1/triage.md"));
    let row_in = |text: &str, key: &str| -> String {
        text.lines()
            .find(|line| line.starts_with(&format!("| `{key}` |")))
            .unwrap_or_else(|| panic!("the round's triage record holds `{key}`:\n{text}"))
            .to_owned()
    };
    assert_eq!(
        row_in(&graded_before, folded),
        row_in(&graded_after, folded),
        "the row the first invocation's triage graded is as that triage left it"
    );

    // (2) ITS TWO RECORDS: what it graded — composed from that attempt's rendering — and what
    // its verifiers filed.
    let composed = composing(&ran);
    assert_eq!(
        composed
            .iter()
            .map(|(_, words)| words.clone())
            .collect::<Vec<_>>(),
        [
            lines(&["reporter=attempt", "reporter=triage"]),
            lines(&[
                "reporter=attempt",
                "reporter=triage",
                "reporter=verify-red-gate",
                "reporter=verify-red-ci"
            ]),
        ],
        "each record names the reporters the invocation has launched, and nothing else"
    );
    let subject = format!("docs(record): {RUN} r1 - the rows nobody had graded");
    assert!(
        composed[0].0.contains(&format!(
            " --round 1 --attempt 2 --subject '{subject}' --sha256 "
        )) && flag_of(&composed[0].0, "--sha256")
            == sha256(&canonical(1, 2, &subject, &composed[0].1)),
        "{composed:?}"
    );
    let head = sim.rig.rev("HEAD");
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["record"],
            result["verify"]["record"],
            result["triage"]["report"],
            result["counts"]["findings_in"],
            result["verify"]["launched"],
            result["verify"]["verdicts"]
        ]),
        json!([
            "verified",
            ["triage", "verify"],
            sim.rig.rev("HEAD~1"),
            head,
            rendering(1, 2),
            1,
            2,
            2
        ]),
        "{result}"
    );
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~2"),
            sim.rig
                .git(&["show", "--no-patch", "--format=%s", "HEAD~1"])
                .trim(),
            head_commit(&sim).0,
            sim.rig.remote(LOOP),
            sim.rig.status()
        ]),
        json!([first_record, subject, second_record_subject(), head, ""]),
        "two records of the continuation, on the first invocation's one"
    );

    // (3) WHAT IS ON RECORD: the row is graded by the triage that met it, and verified; no
    // row awaits a triage, and the queue is empty.
    let state = sim.state();
    assert_eq!(
        json!([
            state["untriaged"],
            queued_keys(&state),
            row_of(&state, "red-gate")["grade"],
            row_of(&state, "red-ci")["grade"],
            row_of(&state, folded)["grade"],
            state["budget"]["spent"]
        ]),
        json!([[], [], "confirmed", "refuted", "no-break", 2]),
        "{state}"
    );
}

/// **A re-run inside a round hunts its due items, grades once, and spends the round's own
/// budget** (`T6b`; the plan's review, `B1`: *three arms, a re-run's among them*; `E3`: *the
/// budget is the round's `test` stage's, counted on record across its invocations*). An item
/// whose reviewer died is void; the round's other finding is verified — ONE of the budget's
/// two — and closed, the human gives his go, and the state asks for the item's re-run. THE
/// RE-RUN IS A STAGE OF THE PATH LIKE ANY: it hunts THE DUE ITEM AND NO OTHER, with no scope
/// step; ITS ONE TRIAGE IS HANDED WHAT THAT HUNT FOUND, and no row the round's first triage
/// graded; its first record; and then IT VERIFIES ON WHAT THE ROUND'S BUDGET HAS LEFT — one
/// verdict, though two rows are in the queue — so the second, a route item nobody reached, is
/// the human's.
#[test]
fn a_rerun_inside_a_round_hunts_its_due_items_grades_once_and_spends_the_rounds_own_budget() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted(
        "rerun-budget",
        &Opening {
            clauses: vec!["audit-clean", "no-lost-files"],
            items: json!([
                item("area-a", "audit-area", "audit-clean"),
                item("area-b", "audit-area", "no-lost-files"),
            ]),
            bounds: vec![],
            rows: json!([]),
        },
        2,
    );
    let candidate = sim.rig.rev("HEAD");
    let mut script = small_stage(json!({"agents": {
        "area-a:review": findings(&["a refusal prints a route that leads nowhere"]),
        "area-b:review": nothing(),
        "triage": {"entries": [a_route(ROUTE, 1)]},
        format!("verify:{ROUTE}"): refutes("intended"),
    }}));
    script["endings"] = json!({"area-b:review": "dies"});
    let first = sim.invoke(sim.args("test", json!({})), script);
    ends_at_its_second_record(&sim, &first, &candidate);
    let state = sim.state();
    assert_eq!(
        json!([
            state["budget"],
            state["next"],
            state["stop"]["then"],
            standing(&state, "area-b")[0]
        ]),
        json!([{"limit": 2, "spent": 1, "left": 1}, "stop", "retest", "void"]),
        "one verdict of the round's two is spent, nothing is open, and the void item is due again: {state}"
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
    let before = sim.rig.rev("HEAD");

    // THE RE-RUN: the due item, its findings graded once, and one verdict.
    let (reached, unreached) = ("a-lost-file-route-is-dead", "another-lost-file-route");
    let of_b = |key: &str, n: u32| {
        let mut entry = keyed(key, "route", "breaks", json!({"area-b-review": [n]}));
        entry["route"] = json!(format!("Run: jigc doc set --repair {key}"));
        entry
    };
    let ran = sim.invoke(
        sim.args("test", json!({"clause": "no-lost-files"})),
        json!({"agents": {
            "area-b:review": findings(&["a file is lost behind a dead route", "another file is lost"]),
            "triage": {"entries": [of_b(reached, 1), of_b(unreached, 2)]},
            format!("verify:{reached}"): files(json!({"verdict": "confirmed", "regression": false})),
        }}),
    );
    let agents = ran.agents();
    assert_eq!(
        agents
            .iter()
            .filter(|label| !label.starts_with("git:"))
            .copied()
            .collect::<Vec<_>>(),
        [
            "preflight",
            "area-b:review",
            "triage",
            format!("verify:{reached}").as_str()
        ],
        "the due item's hunt and no other, no scope step, ONE triage, and one verifier: {agents:?}"
    );
    assert_eq!(
        triaged_from(&ran),
        ["area-b-review=2"],
        "the re-run's triage is handed what its own hunt found: the row the round's first triage graded is not handed again"
    );
    let entries = sim.rendered(1, 2);
    assert_eq!(
        entries["entries"]
            .as_array()
            .expect("the entries")
            .iter()
            .map(|entry| text(&entry["key"]))
            .collect::<Vec<_>>(),
        [reached, unreached],
        "{entries}"
    );
    // THE ROUND'S OWN BUDGET: at the re-run's first record one verdict is spent already — the
    // first stage's — and one is left; a wave is never more than that.
    let read = |n: u32| {
        let state = sim.read_at(n);
        json!([
            queued_keys(&state),
            state["budget"]["spent"],
            state["budget"]["left"],
            state["next"]
        ])
    };
    assert_eq!(read(3), json!([[reached, unreached], 1, 1, "verify"]));
    // (After its wave the step is the stage's own record of the verdict that wave filed.)
    assert_eq!(read(4), json!([[unreached], 2, 0, "verify"]));
    assert_eq!(verified(&ran), [reached], "{agents:?}");
    let composed = composing(&ran);
    assert_eq!(
        composed[0].1,
        [
            "reporter=attempt".to_owned(),
            "reporter=preflight".to_owned(),
            "reporter=area-b-review".to_owned(),
            "reporter=triage".to_owned(),
            format!("commit={candidate}"),
            "item=area-b:green".to_owned(),
        ],
        "the re-run's first record: its reporters, the round's candidate, and the due item's result"
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["rerun"]["clause"],
            result["candidate"]["sha"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["budget"]["left"],
            result["next"]
        ]),
        json!(["verified", "no-lost-files", candidate, 1, 1, 0, "rule"]),
        "{result}"
    );
    assert_eq!(
        json!([sim.rig.rev("HEAD~2"), sim.rig.status()]),
        json!([before, ""]),
        "the re-run's two records, on the go's"
    );
    let state = sim.state();
    let routes = |key: &str| {
        let row = row_of(&state, key);
        json!([row["grade"], row["route"], row["why"]])
    };
    assert_eq!(
        json!([state["budget"], standing(&state, "area-b")[0]]),
        json!([{"limit": 2, "spent": 2, "left": 0}, "green"]),
        "{state}"
    );
    assert_eq!(
        routes(ROUTE),
        json!(["refuted", "recorded", "not-a-break"]),
        "the row the first stage verified is as it was: {state}"
    );
    assert_eq!(
        routes(reached),
        json!(["confirmed", "fix", "inside"]),
        "{state}"
    );
    assert_eq!(
        routes(unreached),
        json!(["breaks", "human", "unreached"]),
        "{state}"
    );
}

/// The stage both scenarios of a silent wave run: three rows in the queue — two route items
/// and a behaviour item — and the behaviour item's verifier confirms it and hands one on.
fn three_in_a_wave(handed_on: &str) -> (Value, &'static str) {
    let elsewhere = "another-route-is-dead";
    (
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "another route is dead", "a slot is dropped"]),
            "triage": {"entries": [a_route(ROUTE, 1), a_route(elsewhere, 2), a_behaviour(INSIDE, 3)]},
            format!("verify:{INSIDE}"): files(json!({
                "verdict": "confirmed", "regression": false,
                "hands_on": [handed(handed_on, Some("Run: jigc doc set --again"))],
            })),
        }})),
        elsewhere,
    )
}

/// **One silent verifier call leaves its finding unverified, and the phase goes on** (`T6b`;
/// the plan's §2.2 and `E7`: *one stray death costs one unverified finding and no stop — a
/// stop costs a record and a restart*). The account's usage limit never reaches the script as
/// a message: it is handed NOTHING. So A VERIFIER CALL IS NEVER TRIED TWICE — the canary's
/// second invocation tried every dead verifier three times — and one call of a wave that
/// returns nothing is that finding's alone: named, unverified, nothing spent, no launch
/// counted, and THE NEXT WAVE IS LAUNCHED. Beside it, the cell the other path's scenario of a
/// verifier that does not finish held: A VERIFIER THAT RETURNS A FILING AND LEFT NO REPORT has
/// filed nothing — the record script files no verdict before its report — and is named so.
#[test]
fn one_silent_verifier_call_leaves_its_finding_unverified_and_the_phase_goes_on() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-one-silent", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let handed_on = "the-repair-route-is-dead";
    let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
    script["agents"][format!("verify:{elsewhere}")] = json!({});
    script["agents"][format!("verify:{handed_on}")] = refutes("does-not-reproduce");
    script["endings"] = json!({
        format!("verify:{ROUTE}"): "dies",
        format!("verify:{elsewhere}"): "unreported",
    });
    let ran = sim.invoke(sim.args("test", json!({})), script);

    // (1) THE SILENT CALL IS TRIED ONCE, and the phase goes on to its second wave.
    assert_eq!(
        tries_of(&ran, &format!("verify:{ROUTE}")),
        1,
        "a verifier call is never tried twice: {:?}",
        ran.agents()
    );
    assert_eq!(
        verified(&ran),
        [ROUTE, elsewhere, INSIDE, handed_on],
        "the wave, and then the item its one verdict handed on: {:?}",
        ran.agents()
    );
    assert!(
        ran.agents().contains(&"git:check-reports:wave2"),
        "{:?}",
        ran.agents()
    );
    assert!(
        !ran.logs
            .iter()
            .any(|line| line.contains("breaker") || line.contains("exhausted")),
        "one silent verifier call is no exhausted call, and trips nothing: {:?}",
        ran.logs
    );

    // (2) EACH IS NAMED, WITH WHY — and neither is a launch the record script counts.
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", ROUTE),
        [format!(
            "NOT VERIFIED: `{ROUTE}` — its verifier returned nothing, and the queue prints the finding again: no verdict of it counts. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "{:?}",
        ran.logs
    );
    assert_eq!(
        said_of(&ran, "NOT VERIFIED", elsewhere),
        [format!(
            "NOT VERIFIED: `{elsewhere}` — its verifier returned and left no report, and the queue prints the finding again: no verdict of it counts. It awaits its verdict again, and nothing of the budget was spent on it"
        )],
        "{:?}",
        ran.logs
    );
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["verify"]["unverified"],
            result["verify"]["silent"],
            result["verify"]["last"]
        ]),
        json!(["verified", "verify", 2, 4, 2, 2, 1, null]),
        "{result}"
    );
    ends_at_its_second_record(&sim, &ran, &candidate);
    let state = sim.state();
    assert_eq!(
        json!([
            queued_keys(&state),
            state["budget"]["spent"],
            row_of(&state, ROUTE)["awaits"],
            row_of(&state, elsewhere)["awaits"]
        ]),
        json!([
            [ROUTE, elsewhere],
            2,
            {"why": "unverified", "passes": 0, "retry": "due"},
            {"why": "unverified", "passes": 0, "retry": "due"}
        ]),
        "both await their verdict, and no launch of either is counted: {state}"
    );
    assert!(
        !sim.rig.root.join(filed_at(elsewhere)).exists(),
        "a verifier that left no report filed nothing"
    );
}

/// **Two silent calls in a wave make it the last, and none is tried again** (`T6b`; §2.2:
/// *a wave in which two verifier calls return nothing is the last wave; no verifier call is
/// tried a second time*; `E7`: *at a real limit every call of the wave is silent*). Two of a
/// wave's three verifiers return nothing. EACH WAS TRIED ONCE. The wave's third files, and
/// hands an item on — which stands first in the queue at the next read, with twenty-nine
/// verdicts of the budget left: AND NO SECOND WAVE IS LAUNCHED. The stage says so, checks the
/// wave's reports, reads the queue, and RECORDS — every step that follows the wave answers
/// here, so this is no limit, and the stage returns with what it verified on record and
/// `next: verify`.
#[test]
fn two_silent_calls_in_a_wave_make_it_the_last_and_none_is_tried_again() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-two-silent", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let handed_on = "the-repair-route-is-dead";
    let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
    script["endings"] = json!({
        format!("verify:{ROUTE}"): "dies",
        format!("verify:{elsewhere}"): "dies",
    });
    let ran = sim.invoke(sim.args("test", json!({})), script);
    for key in [ROUTE, elsewhere, INSIDE] {
        assert_eq!(
            tries_of(&ran, &format!("verify:{key}")),
            1,
            "`{key}`: no verifier call is tried a second time: {:?}",
            ran.agents()
        );
    }
    let after_wave = sim.read_at(4);
    assert_eq!(
        json!([
            queued_keys(&after_wave),
            after_wave["budget"]["left"],
            after_wave["next"]
        ]),
        json!([[handed_on, ROUTE, elsewhere], 29, "verify"]),
        "a row is in the queue that this invocation has not launched, and budget is left: {after_wave}"
    );
    assert_eq!(
        verified(&ran),
        [ROUTE, elsewhere, INSIDE],
        "and no second wave is launched: {:?}",
        ran.agents()
    );
    assert!(
        !ran.agents().contains(&"git:check-reports:wave2"),
        "{:?}",
        ran.agents()
    );
    let last: Vec<&String> = ran
        .logs
        .iter()
        .filter(|line| line.starts_with("THE LAST WAVE"))
        .collect();
    assert!(
        last.len() == 1
            && last[0].contains("2 verifier call(s) of wave 1 returned nothing")
            && last[0].contains("no further wave is launched"),
        "the stage says that the wave was the last, and why: {:?}",
        ran.logs
    );
    assert!(
        !ran.logs.iter().any(|line| line.contains("breaker")),
        "the breaker is every other role's: {:?}",
        ran.logs
    );

    // THE STEPS THAT FOLLOW ANSWER: the stage records what was filed, and returns.
    ends_at_its_second_record(&sim, &ran, &candidate);
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"],
            result["next"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["verify"]["unverified"],
            result["verify"]["silent"],
            result["verify"]["last"]
        ]),
        json!(["verified", null, "verify", 1, 3, 1, 2, 2, 1]),
        "{result}"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            queued_keys(&state),
            state["budget"]["spent"],
            row_of(&state, INSIDE)["grade"]
        ]),
        json!([[handed_on, ROUTE, elsewhere], 1, "confirmed"]),
        "{state}"
    );
}

/// **A stage whose record step is silent after that halts naming the filed verdicts** (`T6b`;
/// §2.2: *the first message of the account's usage limit cannot be seen*; §6, told with the
/// questions: *a filed verdict is on this machine's disk until the stage's second record*).
/// After a wave that was the last for its silence THE STEP THAT COMPOSES THE RECORD RETURNS
/// NOTHING TOO — on every try: that step is no verifier's, and is tried as every step is. THE
/// STAGE HALTS AT ITS RECORD AND SAYS THAT THIS MAY BE THE ACCOUNT'S LIMIT — never that it is:
/// it was handed nothing, and nothing says why — AND NAMES THE FILED VERDICTS: each verdict a
/// verifier of this invocation filed, by the one path a filing of its name has, held by the
/// queue's own read; how many verdicts the state counts in the round; where they lie; and the
/// state document, by its path and its hash, whose `filed` lists every one with whether it
/// counts. Nothing was applied, the first record stands, and the invocation that follows
/// CONTINUES: it launches what is missing, and its record takes the verdict the halt named.
#[test]
fn a_stage_whose_record_step_is_silent_after_that_halts_naming_the_filed_verdicts() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("verify-limit", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let handed_on = "the-repair-route-is-dead";
    let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
    script["endings"] = json!({
        format!("verify:{ROUTE}"): "dies",
        format!("verify:{elsewhere}"): "dies",
        "git:compose:verdicts:r1": "dies",
    });
    let ran = sim.invoke(sim.args("test", json!({})), script);
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["transient"],
            result["halted"]["limit"]
        ]),
        json!(["halted", "record", true, {"possible": true, "wave": 1, "silent": 2, "step": "compose"}]),
        "{result}"
    );
    assert_eq!(
        tries_of(&ran, "git:compose:verdicts:r1"),
        3,
        "the step that composes the record is no verifier's: it is tried as every step is: {:?}",
        ran.agents()
    );
    let message = text(&result["message"]);
    assert!(
        message.contains("THIS MAY BE THE ACCOUNT'S USAGE LIMIT")
            && message.contains("2 verifier call(s) of wave 1 returned nothing")
            && message.contains(&filed_at(INSIDE))
            && !message.contains("is the account's limit"),
        "the halt says what it may be, and never that it is: {message}"
    );
    // WHAT IS SO AT THIS STEP: the queue was read after the wave, so the state's count and the
    // document the halt names are of now; no commit holds what was filed; and what the next
    // invocation does first is the record step's own sentence, which the halt carries.
    assert!(
        message.contains("the step that composes the record returned nothing too")
            && message.contains("IS IN NO COMMIT YET")
            && message.contains("lists every filed verdict with whether it counts")
            && !message.contains("BEFORE THAT WAVE")
            && !message.contains("it CONTINUES"),
        "{message}"
    );

    // THE FILED VERDICTS, NAMED: by path, by the state's count, by where they lie — and by the
    // state document, which lists each with whether it counts.
    assert_eq!(
        result["halted"]["filed"],
        json!({
            "counted": 1,
            "read": "after-the-wave",
            "commit": "none",
            "record": null,
            "files": [filed_at(INSIDE)],
            "unconfirmed": [],
            "directory": format!("{RUN_DIR}/r1/verdicts"),
        }),
        "{result}"
    );
    let listed = document(result)["filed"].clone();
    assert_eq!(
        json!([
            listed.as_array().map(Vec::len),
            listed[0]["file"],
            listed[0]["counts"],
            listed[0]["recorded"]
        ]),
        json!([1, filed_at(INSIDE), true, false]),
        "the document the halt names lists the filed verdict, and that it counts: {listed}"
    );
    assert!(sim.rig.root.join(filed_at(INSIDE)).exists());
    let first_record = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            head_commit(&sim).0,
            sim.rig.remote(LOOP)
        ]),
        json!([candidate, first_record_subject(), first_record]),
        "the first record stands, pushed, and nothing of a second is applied"
    );
    assert!(
        sim.rig.status().lines().all(|line| line.starts_with("?? ")),
        "{}",
        sim.rig.status()
    );

    // THE INVOCATION THAT FOLLOWS CONTINUES: the three rows the queue holds are launched once
    // more, under the next attempt. HERE ITS VERIFIERS LEAVE NOTHING — AND ITS RECORD IS MADE
    // ALL THE SAME, because the state's position says that a filed verdict is in no record:
    // THE VERDICT THE HALT NAMED IS TAKEN, and is in a commit, pushed.
    let state = sim.state();
    assert_eq!(
        json!([state["position"]["test"], queued_keys(&state)]),
        json!([{"round": 1, "attempt": 2, "verify": true, "filed": true}, [handed_on, ROUTE, elsewhere]]),
        "three rows await their verdicts, and the verdict the halt named is in no record: {state}"
    );
    let again = sim.invoke(
        sim.args("test", json!({})),
        nobody_verifies(json!({}), &[handed_on, ROUTE, elsewhere]),
    );
    assert_eq!(
        verified(&again),
        [handed_on, ROUTE, elsewhere],
        "{:?}",
        again.agents()
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            again.result["status"],
            again.result["continued"],
            again.result["arrived"]["owed"],
            again.result["verify"]["verdicts"],
            again.result["verify"]["record"],
            again.result["halted"]
        ]),
        json!(["verified", ["verify", "filed"], ["filed"], 0, head, null]),
        "{}",
        again.result
    );
    let (subject, paths) = head_commit(&sim);
    assert!(
        subject == second_record_subject()
            && paths.contains(&filed_at(INSIDE))
            && sim.rig.rev("HEAD~1") == first_record
            && sim.rig.remote(LOOP).as_deref() == Some(head.as_str()),
        "the record takes the verdict the halted stage filed: {paths:?}"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["budget"]["spent"],
            state["filed"][0]["recorded"],
            queued_keys(&state),
            sim.rig.status()
        ]),
        json!([1, true, [handed_on, ROUTE, elsewhere], ""]),
        "{state}"
    );

    // AND A STEP THAT REFUSES AFTER SUCH A WAVE IS NO SUCH HALT: the composing step answered —
    // twice, with the record script's refusal of a word its agent altered — so nothing here
    // looks like a limit, and the halt says what a refused record says and no more.
    let sim = Sim::budgeted("verify-silent-refused", &one_area(), 30);
    let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
    script["endings"] = json!({
        format!("verify:{ROUTE}"): "dies",
        format!("verify:{elsewhere}"): "dies",
    });
    const WORD: &str = "reporter=area-a-review";
    script["mistypes"] = json!({
        "git:compose:verdicts:r1": [WORD, "reporter=area-a-reviewer"],
        "git:compose:verdicts:r1:again1": [WORD, "reporter=area-a-reviewer"],
    });
    let refused = sim.invoke(sim.args("test", json!({})), script);
    let result = &refused.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["refusal"],
            result["halted"]["transient"],
            result["halted"]["limit"],
            result["halted"]["filed"]
        ]),
        json!(["halted", "record", "words", false, null, null]),
        "{result}"
    );
    assert!(
        !text(&result["message"]).contains("USAGE LIMIT"),
        "{result}"
    );
}

// ---------------------------------------------------------------------------
// A run with a queue: a filed verdict no record took is recorded before anything else follows
// (a finding `T6b`'s builder reported, read from two scripts and driven by nobody: a stage
// that dies after its last verifier filed, or whose budget that wave spent, was handed to
// nobody — and its filed verdicts waited for whichever record came next, or for none)
// ---------------------------------------------------------------------------

/// Another route item of the scenarios below.
const ELSEWHERE: &str = "another-route-is-dead";

/// A stage that finds `keys` — route items, in that order — whose verifiers file `filings`,
/// and whose invocation DIES BEFORE ITS WAVE'S REPORTS ARE CHECKED: after the filings, and
/// before the stage's second record. Returns the commit of its first record.
fn dies_after_its_wave_filed(sim: &Sim, keys: &[&str], filings: &[(&str, Value)]) -> String {
    let candidate = sim.rig.rev("HEAD");
    let titles: Vec<String> = keys
        .iter()
        .map(|key| format!("the finding {key}"))
        .collect();
    let titles: Vec<&str> = titles.iter().map(String::as_str).collect();
    let entries: Vec<Value> = keys
        .iter()
        .zip(1..)
        .map(|(key, n)| a_route(key, n))
        .collect();
    let mut script = small_stage(json!({"agents": {
        "area-a:review": findings(&titles),
        "triage": {"entries": entries},
    }}));
    for (key, filing) in filings {
        script["agents"][format!("verify:{key}")] = filing.clone();
    }
    script["endings"] = json!({"git:check-reports:wave1": "dies"});
    let died = sim.invoke(sim.args("test", json!({})), script);
    assert_eq!(
        json!([died.result["status"], died.result["halted"]["phase"]]),
        json!(["halted", "reports"]),
        "{}",
        died.result
    );
    assert_eq!(
        verified(&died),
        filings.iter().map(|(key, _)| *key).collect::<Vec<_>>(),
        "the wave: {:?}",
        died.agents()
    );
    let first_record = sim.rig.rev("HEAD");
    assert_eq!(
        json!([sim.rig.rev("HEAD~1"), head_commit(sim).0]),
        json!([candidate, first_record_subject()]),
        "the dead invocation left its first record, and nothing after it"
    );
    first_record
}

/// `[counts, recorded]` of every file among the filed verdicts, as a state document has them.
fn filed_as(state: &Value) -> Vec<Value> {
    state["filed"]
        .as_array()
        .unwrap_or_else(|| panic!("a run with a queue says what is filed: {state}"))
        .iter()
        .map(|entry| json!([entry["counts"], entry["recorded"]]))
        .collect()
}

/// **A stage that died after its last verifier filed is handed its record, and nothing else
/// first** (the finding `T6b`'s builder reported and did not build; the rule for this tooling:
/// what can strand state is repaired before a first run). The state counts a verdict from the
/// moment it is filed. So where an invocation dies after THE LAST ROW OF THE QUEUE was filed,
/// no row awaits a verdict, and the step was the one the verdicts left — here a fixer's —
/// with the `test` stage refused as a round that is open: two verdicts the state counts, in
/// no commit, and nothing of the harness that would ever record them. HERE THE STATE HANDS THE
/// STAGE ITS RECORD: `next` is `verify`, the position says `filed`, and THE INVOCATION THAT
/// FOLLOWS CONTINUES — its reads, its attempt, the binaries that stand, NO VERIFIER, and the
/// one record, composed by the tool, which takes what was filed and every report the dead
/// invocation left. ONCE: the invocation after it is refused as it always was. And AN
/// INVOCATION THAT ONLY LOOKS says that the stage is to be continued, and for what.
#[test]
fn a_stage_that_died_after_its_last_verifier_filed_is_handed_its_record_and_nothing_else_first() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("filed-last", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let first_record = dies_after_its_wave_filed(
        &sim,
        &[ROUTE, ELSEWHERE],
        &[
            (
                ROUTE,
                files(json!({"verdict": "confirmed", "regression": false})),
            ),
            (ELSEWHERE, refutes("intended")),
        ],
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["budget"]["spent"],
            queued_keys(&state),
            filed_as(&state)
        ]),
        json!([2, [], [[true, false], [true, false]]]),
        "the control: both verdicts are filed and count, no record holds one, and no row awaits a verdict: {state}"
    );
    assert_eq!(
        json!([state["next"], state["position"]["test"], state["stop"]]),
        json!(["verify", {"round": 1, "attempt": 2, "filed": true}, null]),
        "the stage is handed its record — not a fixer the findings: {state}"
    );

    // AN INVOCATION THAT ONLY LOOKS reads the state across a filed verdict — nothing is to be
    // finished first — and says what the stage is continued for. It begins nothing.
    let status = sim.rig.status();
    let looked = sim.invoke(sim.args("test", json!({"stopAfter": "state"})), json!({}));
    assert_eq!(
        json!([
            looked.result["status"],
            looked.result["after"],
            looked.result["continues"],
            looked.result["owed"],
            looked.result["attempt"],
            looked.result["candidate"]["sha"]
        ]),
        json!(["stopped", "state", ["filed"], ["filed"], 2, candidate]),
        "{}",
        looked.result
    );
    assert_eq!(looked.trace, lines(&TWO_READS));
    assert!(
        looked.commands[0].contains(" --look "),
        "{:?}",
        looked.commands
    );
    assert_eq!(
        json!([sim.rig.rev("HEAD"), sim.rig.status()]),
        json!([first_record, status]),
        "a look records nothing"
    );

    // THE INVOCATION THAT FOLLOWS CONTINUES, AND ONLY RECORDS.
    let ran = sim.invoke(sim.args("test", json!({})), json!({}));
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(FILED_CHECK.to_owned());
    expected.extend(continuation_records("verdicts", 2, 2));
    assert_eq!(
        ran.agents(),
        expected,
        "its reads, its attempt, the binaries that stand, the report check every record is preceded by, and the record: no verifier, no triage"
    );
    assert!(
        ran.logs.iter().any(|line| line.contains(
            "A CONTINUATION (`next` is `verify`): a filed verdict is in no record — on "
        )),
        "the log says what the stage is continued for: {:?}",
        ran.logs
    );
    let result = &ran.result;
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            result["status"],
            result["halted"],
            result["continued"],
            result["record"],
            result["verify"]["record"],
            result["verify"]["waves"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["next"]
        ]),
        json!(["verified", null, ["filed"], null, head, 0, 0, 0, "fix"]),
        "{result}"
    );
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            head_commit(&sim).0,
            sim.rig.remote(LOOP),
            sim.rig.status()
        ]),
        json!([first_record, second_record_subject(), head, ""]),
        "ONE record, on the dead invocation's first, pushed, and nothing left pending"
    );
    let composed = composing(&ran);
    assert!(
        composed.len() == 1
            && composed[0].0.contains(" --round 1 --attempt 2 --subject ")
            && composed[0].1 == ["reporter=attempt".to_owned()],
        "its record names its marker, and nothing else: {composed:?}"
    );
    let (_, paths) = head_commit(&sim);
    for rel in [
        format!("r1/verdicts/verify-{ROUTE}.a1.json"),
        format!("r1/verdicts/verify-{ELSEWHERE}.a1.json"),
        format!("r1/reports/test/verify-{ROUTE}.a1.md"),
        format!("r1/reports/test/verify-{ELSEWHERE}.a1.md"),
        "r1/reports/test/attempt.a2.md".to_owned(),
        "r1/triage.md".to_owned(),
        "r1/view.md".to_owned(),
    ] {
        assert!(
            paths.contains(&format!("{RUN_DIR}/{rel}")),
            "`{rel}` is in the record: {paths:?}"
        );
    }
    let state = sim.state();
    assert_eq!(
        json!([
            filed_as(&state),
            state["next"],
            state["position"]["test"],
            state["rounds"][0]["facts"]["recorded"],
            state["rounds"][0]["test_unrecorded"],
            row_of(&state, ROUTE)["route"],
            row_of(&state, ELSEWHERE)["route"]
        ]),
        json!([
            [[true, true], [true, true]],
            "fix",
            {"refused": "round-open", "round": 1},
            ["test a2"],
            0,
            "fix",
            "recorded"
        ]),
        "every filed verdict is on record, and the step is the one the verdicts left: {state}"
    );

    // ONCE: nothing is owed a record any more, and the stage is refused as it always was.
    let after = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(
        json!([
            after.result["status"],
            after.result["refused"]["refused"],
            after.result["arrived"]["owed"],
            sim.rig.rev("HEAD")
        ]),
        json!(["refused", "round-open", [], head]),
        "{}",
        after.result
    );
    assert_eq!(after.trace, lines(&TWO_READS));
}

/// **A stage whose last wave spent the budget and filed is handed its record before the
/// human's step** (the same finding, its second shape). The wave that spends the budget
/// files, and the invocation dies before the second record: a route item nobody reached is
/// the human's, so the step was `rule` — over two verdicts no commit holds, with a stop's
/// view on disk that shows neither. HERE THE STEP IS THE STAGE'S RECORD FIRST: the
/// invocation that follows continues, launches NOBODY — the budget has nothing left — and
/// records; and then the step is the human's, with what the budget bought on record and in
/// the view. AND A FILED VERDICT A COMMIT HOLDS AND NO RECORD TOOK — what a filing that
/// arrives between a record's batch and its commit leaves, reached here by a commit made by
/// hand — is recorded the same way: the first read finds nothing owed, and THE STATE'S WORD
/// IS WHAT THE RECORD IS COMPOSED ON.
#[test]
fn a_stage_whose_last_wave_spent_the_budget_is_handed_its_record_before_the_humans_step() {
    if !can_run() {
        return;
    }
    let third = "a-third-route-is-dead";
    let filings = [
        (
            ROUTE,
            files(json!({"verdict": "confirmed", "regression": false})),
        ),
        (ELSEWHERE, refutes("intended")),
    ];
    let sim = Sim::budgeted("filed-spent", &one_area(), 2);
    let candidate = sim.rig.rev("HEAD");
    let first_record = dies_after_its_wave_filed(&sim, &[ROUTE, ELSEWHERE, third], &filings);
    let state = sim.state();
    assert_eq!(
        json!([
            state["budget"],
            queued_keys(&state),
            filed_as(&state),
            state["human_list"],
            state["view"]["on_disk"]
        ]),
        json!([
            {"limit": 2, "spent": 2, "left": 0},
            [third],
            [[true, false], [true, false]],
            [{"key": third, "why": "unreached"}],
            "behind"
        ]),
        "the control: the budget is spent on two verdicts no record holds, one route item is unreached, and the view on disk shows neither verdict: {state}"
    );
    assert_eq!(
        json!([state["next"], state["position"]["test"]]),
        json!(["verify", {"round": 1, "attempt": 2, "filed": true}]),
        "the stage's record comes before the human's step: {state}"
    );

    let ran = sim.invoke(sim.args("test", json!({})), json!({}));
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(FILED_CHECK.to_owned());
    expected.extend(continuation_records("verdicts", 2, 2));
    assert_eq!(
        ran.agents(),
        expected,
        "nobody is launched: the budget has nothing left"
    );
    let result = &ran.result;
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["verify"]["launched"],
            result["budget"]["left"],
            result["verify"]["record"],
            result["next"],
            result["rule"]["findings"]
        ]),
        json!(["verified", ["filed"], 0, 0, head, "rule", 1]),
        "{result}"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            sim.rig.remote(LOOP),
            sim.rig.status(),
            filed_as(&state),
            state["next"],
            state["position"]["test"],
            state["view"]["on_disk"]
        ]),
        json!([
            first_record,
            head,
            "",
            [[true, true], [true, true]],
            "rule",
            {"refused": "round-open", "round": 1},
            "written"
        ]),
        "recorded, pushed, and the step is the human's — with a view that shows what the budget bought: {state}"
    );

    // A FILED VERDICT A COMMIT HOLDS AND NO RECORD TOOK. A commit step adds every pending
    // file of the run, so a verdict filed between a record's batch and its commit rides that
    // commit, and no table holds it. Here a hand makes such a commit.
    let sim = Sim::budgeted("filed-committed", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    dies_after_its_wave_filed(&sim, &[ROUTE, ELSEWHERE], &filings);
    sim.rig.git(&["add", "--", RUN_DIR]);
    sim.rig.git(&[
        "commit",
        "-q",
        "-m",
        "docs(record): a filing that arrived after its record's batch",
    ]);
    sim.rig.git(&["push", "-q", "origin", LOOP]);
    let rode = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            sim.rig.status(),
            filed_as(&state),
            state["next"],
            state["position"]["test"]
        ]),
        json!([
            "",
            [[true, false], [true, false]],
            "verify",
            {"round": 1, "attempt": 2, "filed": true}
        ]),
        "a commit holds both files, and no record took either: {state}"
    );
    let ran = sim.invoke(sim.args("test", json!({})), json!({}));
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(FILED_CHECK.to_owned());
    expected.extend(continuation_records("verdicts", 2, 2));
    assert_eq!(ran.agents(), expected);
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            ran.result["status"],
            ran.result["continued"],
            ran.result["arrived"]["owed"],
            ran.result["verify"]["record"],
            sim.rig.rev("HEAD~1")
        ]),
        json!(["verified", ["filed"], [], head, rode]),
        "the first read found nothing owed, and the record was composed on the state's word: {}",
        ran.result
    );
    assert_eq!(
        filed_as(&sim.state()),
        [json!([true, true]), json!([true, true])]
    );
}

// ---------------------------------------------------------------------------
// A run with a queue: every record the harness composes or applies is preceded, in the same
// invocation, by the report check (the scoped review before the parts, the harness's finding
// 2: a continuation that owes only a record ran no report check, and neither did a record of
// rulings — so a file the tools did not write left no invocation that progresses)
// ---------------------------------------------------------------------------

/// The report check an invocation asks for BEFORE A RECORD IT COMPOSES WITH NO WAVE BEHIND IT:
/// a continuation that owes only a record (`filed`) launches nobody, so no wave's check came
/// before its composing step.
const FILED_CHECK: &str = "git:check-reports:filed";
/// … and before the batch of the human's rulings, in a run with a queue: the report check's
/// vet alone — a record of rulings names no stage's reporters.
const RULINGS_CHECK: &str = "git:check-reports:rulings";

/// A file a hand left among a round's filed verdicts, under a plain name that is no verdict's …
fn stray_among_the_filed() -> String {
    format!("{RUN_DIR}/r1/verdicts/notes.txt")
}

/// … and one beside the `test` stage's reports, under a plain name that is no report's.
fn stray_beside_the_reports() -> String {
    format!("{RUN_DIR}/r1/reports/test/filing.json")
}

/// The lines of a log that say a file was set aside by a report check, by the file.
fn set_aside(ran: &Ran, path: &str) -> usize {
    ran.logs
        .iter()
        .filter(|line| line.starts_with("SET ASIDE") && line.contains(&format!("`{path}`")))
        .count()
}

/// **A record composed with no wave behind it is preceded by its report check, and a stray
/// file strands nothing** (the harness's finding 2, its first half). A stage died after its
/// last verifier filed: the invocation that follows owes only the record (`filed`), launches
/// nobody — and so ran NO report check before its composing step, though the step tool's own
/// order is *the report check, then the record*. With a file a hand left among the filed
/// verdicts the record script refused the record (`check`), the halt said that *the stage's
/// report check came before this step* — none had — and it was refused so at every later
/// invocation: two attempts without a record, then `attempts-spent`, then the human's `again`
/// refused at its own record, and `attempts-spent` again; HEAD never moved, and two counted
/// verdicts stayed in no commit. HERE THE CHECK COMES FIRST: both strays — the one among the
/// filed verdicts, and one beside the reports, which the record script names as pending since
/// its own repair — leave the tree, and the one record takes what was filed.
#[test]
fn a_record_composed_with_no_wave_is_preceded_by_its_report_check_and_a_stray_file_strands_nothing()
{
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("filed-stray", &one_area(), 30);
    let candidate = sim.rig.rev("HEAD");
    let first_record = dies_after_its_wave_filed(
        &sim,
        &[ROUTE, ELSEWHERE],
        &[
            (
                ROUTE,
                files(json!({"verdict": "confirmed", "regression": false})),
            ),
            (ELSEWHERE, refutes("intended")),
        ],
    );
    let strays = [stray_among_the_filed(), stray_beside_the_reports()];
    for stray in &strays {
        fs::write(sim.rig.root.join(stray), "a note somebody left here\n").expect("a stray file");
    }
    let state = sim.state();
    assert_eq!(
        json!([state["next"], state["position"]["test"]]),
        json!(["verify", {"round": 1, "attempt": 2, "filed": true}]),
        "the stage is handed its record: {state}"
    );

    // A CHECK THAT DOES NOT COME BACK HALTS THE INVOCATION BEFORE ANYTHING IS COMPOSED: it is
    // the report check's own row of what halts a stage, and — no wave of this invocation was
    // made the last — no halt that may be the account's limit.
    let unchecked = sim.invoke(
        sim.args("test", json!({})),
        json!({"endings": {FILED_CHECK: "dies"}}),
    );
    let result = &unchecked.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["transient"],
            result["halted"]["limit"],
            tries_of(&unchecked, FILED_CHECK),
            unchecked.agents().contains(&"git:compose:verdicts:r1"),
            sim.rig.rev("HEAD")
        ]),
        json!(["halted", "reports", true, null, 3, false, first_record]),
        "{result}"
    );
    assert!(
        text(&result["message"]).contains("before the record of what was filed")
            && text(&result["message"]).contains("Nothing was composed"),
        "{result}"
    );

    let ran = sim.invoke(sim.args("test", json!({})), json!({}));
    let result = &ran.result;
    assert_eq!(
        json!([result["status"], result["halted"], result["continued"]]),
        json!(["verified", null, ["filed"]]),
        "the record is made past both files: {result}"
    );
    let mut expected = continuation_opens(&sim, &candidate);
    expected.push(FILED_CHECK.to_owned());
    expected.extend(continuation_records("verdicts", 3, 2));
    assert_eq!(
        ran.agents(),
        expected,
        "its reads, its attempt, the binaries that stand, THE REPORT CHECK, and the record"
    );
    let check = ran
        .commands
        .iter()
        .find(|command| command.starts_with("dev/stabilize-step check-reports "))
        .unwrap_or_else(|| panic!("the report check ran: {:?}", ran.commands));
    assert!(
        check.contains(" --round 1 --stage test --attempt 3 ") && check.ends_with(" -- attempt"),
        "the check is the attempt's own, over its one reporter — its marker: {check}"
    );
    for stray in &strays {
        assert!(
            !sim.rig.root.join(stray).exists() && set_aside(&ran, stray) == 1,
            "`{stray}` left the tree, and the log says so: {:?}",
            ran.logs
        );
    }
    let head = sim.rig.rev("HEAD");
    let (subject, paths) = head_commit(&sim);
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            subject,
            sim.rig.remote(LOOP),
            sim.rig.status(),
            result["verify"]["record"]
        ]),
        json!([first_record, second_record_subject(), head, "", head]),
        "ONE record, pushed, and nothing left pending"
    );
    assert!(
        strays.iter().all(|stray| !paths.contains(stray))
            && paths.contains(&filed_at(ROUTE))
            && paths.contains(&filed_at(ELSEWHERE)),
        "the record holds both filed verdicts, and neither stray: {paths:?}"
    );
    let state = sim.state();
    assert_eq!(
        json!([filed_as(&state), state["next"], state["human_stages"]]),
        json!([[[true, true], [true, true]], "fix", []]),
        "every filed verdict is on record, and no attempt was spent on a file: {state}"
    );
}

/// **A record of rulings is preceded by the report check's vet, and a stray file costs it
/// nothing** (the harness's finding 2, its second half; what the record script's repair left:
/// *until the harness sets it aside before a rulings batch, a stray file beside the reports
/// costs a record of rulings a gate*). Every batch takes what was filed first, a ruling's too.
/// (1) A FILE AMONG THE FILED VERDICTS under a plain name: the batch of the rulings was
/// refused where it is applied (`check`) — the record script's line names the report check,
/// which no rulings invocation ran — at every invocation, with no exit the harness takes.
/// (2) A FILE BESIDE THE REPORTS under a plain name: a pending path, so the batch was applied
/// beside it, the gate ran, the commit step staged the file, the vet refused it, and the
/// batch was taken back — one whole gate spent, nothing recorded, and the ruling to be sent
/// again. HERE THE VET COMES BEFORE THE BATCH, in a run with a queue: the file leaves the
/// tree, and the ruling is recorded by that invocation, behind one gate.
#[test]
fn a_record_of_rulings_is_preceded_by_the_report_checks_vet_and_a_stray_file_costs_it_nothing() {
    if !can_run() {
        return;
    }
    for (label, stray) in [
        ("rule-stray-filed", stray_among_the_filed()),
        ("rule-stray-report", stray_beside_the_reports()),
    ] {
        let (sim, _stage) = off_the_order(label, 30);
        let stopped_at = sim.rig.rev("HEAD");
        fs::write(sim.rig.root.join(&stray), "a note somebody left here\n").expect("a stray file");
        let before = gates_run(&sim);
        let ruled = sim.invoke(
            sim.args("test", json!({"rulings": [{"queued": [ELSEWHERE]}]})),
            json!({}),
        );
        let result = &ruled.result;
        assert_eq!(
            json!([result["status"], result["halted"], result["next"]]),
            json!(["ruled", null, "verify"]),
            "{label}: the ruling is recorded by the invocation that carries it: {result}"
        );
        assert_eq!(
            ruled.agents(),
            only_rules(1),
            "{label}: two reads, THE VET, the one step that records rulings, its gate, its commit, its push, the state"
        );
        let check = ruled
            .commands
            .iter()
            .find(|command| command.starts_with("dev/stabilize-step check-reports "))
            .unwrap_or_else(|| panic!("{label}: the vet ran: {:?}", ruled.commands));
        assert!(
            check.contains(&format!(" --run {RUN} "))
                && !check.contains(" --stage ")
                && !check.contains(" --attempt ")
                && !check.contains(" -- "),
            "{label}: the vet names no stage, no attempt and no reporter: {check}"
        );
        assert!(
            !sim.rig.root.join(&stray).exists() && set_aside(&ruled, &stray) == 1,
            "{label}: the file left the tree, and the log says so: {:?}",
            ruled.logs
        );
        let head = sim.rig.rev("HEAD");
        assert_eq!(
            json!([
                sim.rig.rev("HEAD~1"),
                sim.rig.remote(LOOP),
                sim.rig.status(),
                result["record"],
                gates_run(&sim) - before
            ]),
            json!([stopped_at, head, "", head, 1]),
            "{label}: ONE commit, pushed, nothing left pending — behind ONE gate"
        );
        assert_eq!(
            queued_keys(&sim.state()),
            [ELSEWHERE],
            "{label}: what was ruled is on record"
        );
    }

    // A VET THAT DOES NOT COME BACK HALTS THE INVOCATION BEFORE ANYTHING IS APPLIED: no
    // executor is launched, no batch is left, and the ruling is the next invocation's.
    let (sim, _stage) = off_the_order("rule-unvetted", 30);
    let stopped_at = sim.rig.rev("HEAD");
    let unvetted = sim.invoke(
        sim.args("test", json!({"rulings": [{"queued": [ELSEWHERE]}]})),
        json!({"endings": {RULINGS_CHECK: "dies"}}),
    );
    let result = &unvetted.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["transient"],
            sim.record(&["pending", "--run", RUN], "")["batch"],
            sim.rig.rev("HEAD")
        ]),
        json!(["halted", "reports", true, null, stopped_at]),
        "{result}"
    );
    assert_eq!(
        unvetted.agents(),
        [
            "git:state",
            "git:state:test-1",
            RULINGS_CHECK,
            RULINGS_CHECK,
            RULINGS_CHECK
        ],
        "the vet is asked for again as every step is, and nothing follows it"
    );
    assert!(
        text(&result["message"]).contains("the rulings were not recorded")
            && text(&result["message"]).contains("nothing was applied"),
        "{result}"
    );
}

// ---------------------------------------------------------------------------
// A run with a queue: what a stage returns, and the rulings about the queue (the reshape's
// plan, `T7`; the row `wf-return-names`)
// ---------------------------------------------------------------------------

/// Every list a return holds, by where it stands: `a.b` for a member, and an element of a list
/// under its list's own name.
fn lists_of(value: &Value, at: &str, found: &mut std::collections::BTreeSet<String>) {
    match value {
        Value::Array(items) => {
            found.insert(at.to_owned());
            for item in items {
                lists_of(item, at, found);
            }
        }
        Value::Object(members) => {
            for (name, member) in members {
                let below = if at.is_empty() {
                    name.clone()
                } else {
                    format!("{at}.{name}")
                };
                lists_of(member, &below, found);
            }
        }
        _ => {}
    }
}

/// THE LISTS A RETURN OF A RUN WITH A QUEUE MAY HOLD — each one THE STEP TOOL'S, printed by a
/// digest whose shape bounds it, or the position's words: what the first read met, what the
/// stage is continued for, the clauses that forbid the close, the items no round selected, the
/// queue's and the view's classes, the words that keep a row out of the queue, and the human's
/// three lists beside `rule`. NONE IS OF THE HARNESS'S MAKING: what an agent returned and what
/// the script launched are counts in a return, and names in the log.
const THE_TOOLS_LISTS: [&str; 13] = [
    "arrived.finished",
    "arrived.found",
    "arrived.owed",
    "continued",
    "continues",
    "forbids_close.clauses",
    "named.never_selected",
    "queue.classes",
    "rule.clauses",
    "rule.reverify",
    "rule.stages",
    "view.listed",
    "view.rates",
];

/// Hold a return of a run with a queue to HOLDING NO LIST OF ITS OWN: every list in it is one
/// of [`THE_TOOLS_LISTS`] — or lies in the state as the digest has it, which goes back whole
/// where the script does not know `next` — and none of `names`, the keys, reporters, items and
/// doors of the scenario, stands anywhere in it.
fn holds_no_list(result: &Value, names: &[&str]) {
    let mut found = std::collections::BTreeSet::new();
    lists_of(result, "", &mut found);
    let strangers: Vec<&String> = found
        .iter()
        .filter(|at| !THE_TOOLS_LISTS.contains(&at.as_str()) && !at.starts_with("state."))
        .collect();
    assert!(
        strangers.is_empty(),
        "a return of a run with a queue holds a list that is no digest's: {strangers:?}\n{result}"
    );
    let returned = result.to_string();
    for name in names {
        assert!(
            !returned.contains(name),
            "`{name}` is a key, a reporter, an item or a door, and the return holds it: {returned}"
        );
    }
}

/// Hold what a return says of the queue, the budget and the stop's view TO THE STATE DOCUMENT
/// IT NAMES — the record script's, which the harness read a digest of: the budget whole; the
/// queue's length and its length class by class; and the view by its path, its hash and how
/// the file on disk stands to that hash — where it says `written`, THE FILE'S BYTES HASH TO IT.
fn says_what_the_state_says(sim: &Sim, result: &Value) {
    let state = document(result);
    assert_eq!(result["budget"], state["budget"], "{result}");
    assert_eq!(
        json!([
            result["queue"]["length"],
            result["queue"]["classes"],
            result["queue"]["hand_on"]
        ]),
        json!([
            state["queue"]["ranked"].as_array().map(Vec::len),
            state["queue"]["classes"],
            state["facts"]["hand-on"]
        ]),
        "{result}"
    );
    let (said, shown) = (&result["view"], &state["view"]);
    assert_eq!(
        json!([said["round"], said["file"], said["sha256"], said["on_disk"]]),
        json!([
            shown["round"],
            shown["file"],
            shown["sha256"],
            shown["on_disk"]
        ]),
        "{result}"
    );
    assert_eq!(
        json!([said["verified"], said["unreached"]]),
        json!([
            state["budget"]["spent"],
            shown["unreached"].as_array().map(Vec::len)
        ]),
        "the hit rate adds up to what the budget spent, and the list's length is the list's: {result}"
    );
    if said["on_disk"] == "written" {
        assert_eq!(
            said["sha256"],
            sha256(&sim.rig.read(&text(&said["file"]))).as_str(),
            "the file the return names is the view the state renders"
        );
    }
}

/// The names of a return's members.
fn members(result: &Value) -> Vec<String> {
    result
        .as_object()
        .unwrap_or_else(|| panic!("a return is an object: {result}"))
        .keys()
        .cloned()
        .collect()
}

/// **The return names what happened, and holds no list** (the reshape's plan, `T7`; the row
/// `wf-return-names`: *`status: "triaged"` with the triage unfinished; `unverified` naming one
/// row beside `untriaged.count: 118`*). ON THE PATH OF A RUN WITH A QUEUE the status is one of
/// three words, each for what THE INVOCATION did: `hunted` — a stage whose findings are on
/// record, and which made no record of its verify phase; `verified` — the record of the verify
/// phase was made; `continued` — an invocation that continued a stage and made no such record.
/// BESIDE IT the return says what the record script counts, as the state's digest has it: THE
/// BUDGET (its limit, what is spent, what is left), THE QUEUE'S LENGTH, CLASS BY CLASS, THE HIT
/// RATE and THE STOP'S VIEW by its path and its hash — and WHAT AN INVOCATION OF `test`
/// CONTINUES THE STAGE FOR (`continues`), which is how a reader tells verification that is owed
/// from a stage that has nothing left. AND NO LIST OF ITS OWN: what the stage launched, what
/// its agents returned, the items it voided and the doors nobody covers are HOW MANY.
#[test]
fn the_return_names_what_happened_and_holds_no_list() {
    if !can_run() {
        return;
    }
    // (1) A STAGE THAT VERIFIED, AND ENDED AT ITS BUDGET: two route items, one verdict.
    let sim = Sim::budgeted("return-verified", &one_area(), 1);
    let candidate = sim.rig.rev("HEAD");
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "another route is dead"]),
            "triage": {"entries": [a_route(ROUTE, 1), a_route(ELSEWHERE, 2)]},
            format!("verify:{ROUTE}"): refutes("intended"),
        }})),
    );
    let result = &ran.result;
    ends_at_its_second_record(&sim, &ran, &candidate);
    assert_eq!(
        members(result),
        [
            "arrived",
            "budget",
            "candidate",
            "continues",
            "counts",
            "cross_model",
            "forbids_close",
            "named",
            "next",
            "queue",
            "record",
            "rerun",
            "round",
            "rule",
            "run",
            "scope",
            "stage",
            "status",
            "triage",
            "verify",
            "view"
        ],
        "{result}"
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["continues"],
            result["rule"]
        ]),
        json!(["verified", "rule", [], {"findings": 1, "reverify": [], "clauses": [], "stages": []}]),
        "it verified, the budget is spent with a route item unreached, and nothing of the stage is left to continue: {result}"
    );
    assert_eq!(
        result["verify"],
        json!({"waves": 1, "launched": 1, "verdicts": 1, "record": head, "unverified": 0,
               "unheld": 0, "contested": 0, "silent": 0, "last": null}),
        "what the phase did, as counts — and what the budget has left is the budget's to say: {result}"
    );
    assert_eq!(
        json!([
            result["budget"],
            result["queue"]["length"],
            result["queue"]["hand_on"]
        ]),
        json!([{"limit": 1, "spent": 1, "left": 0}, 1, 2]),
        "{result}"
    );
    assert_eq!(
        result["queue"]["classes"],
        json!([
            {"kind": "route", "origin": "under-confirmed", "length": 0},
            {"kind": "behaviour", "origin": "under-confirmed", "length": 0},
            {"kind": "route", "origin": "hunt", "length": 1},
            {"kind": "behaviour", "origin": "hunt", "length": 0},
        ]),
        "the queue's length, class by class, in the order's own order: {result}"
    );
    assert_eq!(
        json!([
            result["view"]["on_disk"],
            result["view"]["file"],
            result["view"]["verified"],
            result["view"]["confirmed"],
            result["view"]["refuted"],
            result["view"]["undrivable"],
            result["view"]["unreached"]
        ]),
        json!(["written", format!("{RUN_DIR}/r1/view.md"), 1, 0, 1, 0, 1]),
        "the hit rate, and the stop's view by its path: {result}"
    );
    says_what_the_state_says(&sim, result);
    assert_eq!(
        json!([
            result["counts"],
            result["scope"],
            result["cross_model"],
            result["rerun"]
        ]),
        json!([
            {"items": 1, "items_run": 1, "reporters": 6, "findings_in": 2, "blockers": 0,
             "for_the_human": 1, "voided": 0, "unasserted": 0, "held": 0},
            {"status": "written", "uncovered": 0, "reached_but_excluded": 0},
            {"named": 0, "ran": 0, "void": 0},
            null
        ]),
        "what it launched, voided and left uncovered is HOW MANY: {result}"
    );
    holds_no_list(
        result,
        &[
            ROUTE,
            ELSEWHERE,
            "verify-",
            "area-a",
            "jigc doc set",
            "Run: ",
        ],
    );

    // (2) A STAGE THAT HUNTED, AND WHOSE VERIFIER LEFT NOTHING: verification is owed, and the
    // return says so — `next` is `verify`, a value the script knows, and the stage is continued
    // for it. THEN THE INVOCATION THAT CONTINUES IT, whose verifier leaves nothing either: it
    // hunted nothing and recorded nothing, and says `continued`.
    let sim = Sim::budgeted("return-hunted", &one_area(), 30);
    let script = nobody_verifies(
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere"]),
            "triage": {"entries": [a_route(ROUTE, 1)]},
        }})),
        &[ROUTE],
    );
    let ran = sim.invoke(sim.args("test", json!({})), script);
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["returned_to_orchestrator"],
            result["continues"],
            result["record"],
            result["verify"]["record"],
            result["verify"]["launched"],
            result["verify"]["verdicts"],
            result["verify"]["unverified"],
            result["budget"],
            result["queue"]["length"]
        ]),
        json!(["hunted", "verify", null, ["verify"], sim.rig.rev("HEAD"), null, 1, 0, 1,
               {"limit": 30, "spent": 0, "left": 30}, 1]),
        "{result}"
    );
    assert!(
        result.get("state").is_none() && result.get("rule").is_none(),
        "{result}"
    );
    says_what_the_state_says(&sim, result);
    holds_no_list(result, &[ROUTE, "verify-", "area-a", "jigc doc set"]);
    let again = sim.invoke(
        sim.args("test", json!({})),
        nobody_verifies(json!({}), &[ROUTE]),
    );
    let result = &again.result;
    assert_eq!(
        members(result),
        [
            "arrived",
            "budget",
            "candidate",
            "continued",
            "continues",
            "counts",
            "forbids_close",
            "named",
            "next",
            "queue",
            "record",
            "round",
            "run",
            "stage",
            "status",
            "triage",
            "verify",
            "view"
        ],
        "{result}"
    );
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["next"],
            result["continues"],
            result["record"],
            result["triage"],
            result["verify"]["record"],
            result["verify"]["launched"],
            result["counts"]
        ]),
        json!(["continued", ["verify"], "verify", ["verify"], null, null, null, 1,
               {"reporters": 1, "findings_in": 0, "blockers": 0, "for_the_human": 0}]),
        "an invocation that continued a stage and recorded no verdict hunted nothing, and does not say that it did: {result}"
    );
    says_what_the_state_says(&sim, result);
    holds_no_list(result, &[ROUTE, "verify-", "area-a", "jigc doc set"]);

    // (3) A STAGE THAT HAD NOTHING TO VERIFY: it hunted, the queue printed no key, and no wave
    // was launched. `next` is a value the script does not act on — the run stops after every
    // round — so the state goes back as the digest has it, and that is where its lists are.
    let sim = Sim::budgeted("return-nothing", &one_area(), 30);
    let ran = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    let result = &ran.result;
    assert_eq!(
        json!([
            result["status"],
            result["next"],
            result["returned_to_orchestrator"],
            result["continues"],
            result["verify"],
            result["budget"],
            result["queue"]["length"],
            result["view"]["on_disk"],
            result["view"]["verified"]
        ]),
        json!(["hunted", "stop", true, [],
               {"waves": 0, "launched": 0, "verdicts": 0, "record": null, "unverified": 0,
                "unheld": 0, "contested": 0, "silent": 0, "last": null},
               {"limit": 30, "spent": 0, "left": 30}, 0, "written", 0]),
        "{result}"
    );
    says_what_the_state_says(&sim, result);
    // (The state that goes back names the test set's items, as the digest does; nothing
    // beside it names one.)
    holds_no_list(result, &["verify-", "jigc doc set"]);
    let mut beside = result.clone();
    beside
        .as_object_mut()
        .expect("a return is an object")
        .remove("state");
    holds_no_list(&beside, &["area-a"]);
}

/// The agents of an invocation that ONLY RECORDS RULINGS, in round 1 of a run with a queue: its
/// two reads, THE REPORT CHECK'S VET every record is preceded by, the one step that records
/// them, its gate — held, under the name `gate` — its commit, its push, and the state read
/// back. NOTHING IS STARTED.
fn only_rules(gate: u32) -> Vec<String> {
    let mut agents = lines(&[
        "git:state",
        "git:state:test-1",
        RULINGS_CHECK,
        "record:rulings:r1",
    ]);
    for earlier in 1..gate {
        agents.push(format!("git:hold-start:record-rulings-r1-{earlier}"));
    }
    agents.extend([
        format!("git:hold-start:record-rulings-r1-{gate}"),
        format!("git:hold-wait:record-rulings-r1-{gate}"),
        "git:record:rulings:r1".to_owned(),
        "git:push:rulings".to_owned(),
        "git:state:test-2".to_owned(),
    ]);
    agents
}

/// The calls of the batch the one rulings step applied, read from the file its `apply` names:
/// each as `[its arguments, what it read]`.
fn ruled_calls(ran: &Ran) -> Vec<Value> {
    let apply = ran
        .commands
        .iter()
        .find(|command| command.starts_with("dev/stabilize-record apply "))
        .unwrap_or_else(|| panic!("the rulings step applied its batch: {:?}", ran.commands));
    let file = flag_of(apply, "--from");
    let batch: Value = serde_json::from_str(
        fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("read {file}: {e}"))
            .trim_end(),
    )
    .expect("a batch is one line of JSON");
    batch
        .as_array()
        .expect("a batch is a list of calls")
        .iter()
        .map(|call| {
            json!([
                call["argv"],
                serde_json::from_str::<Value>(call["stdin"].as_str().expect("what the call reads"))
                    .expect("the call reads JSON")
            ])
        })
        .collect()
}

/// **A budget ruled at a stop is recorded, and starts nothing** (the reshape's plan, `T7`;
/// ruling 1 of 2026-10-10: *one budget*; `T2b`: *a budget the human raises reopens the queue*).
/// A stage ends at its budget with a route item nobody reached, and the step is the human's.
/// THE RAISE TAKES THE ONE ROAD EVERY RULING ABOUT THE RUN TAKES: an invocation that carries it
/// records it — the one step that records rulings, its gate, its commit, its push — STARTS
/// NOTHING, and returns the state's `next`, which is now `verify`: the stage is continued, by
/// the invocation that follows. Where the state does not ask for a raise the invocation is
/// refused after its two reads, IN THE RECORD SCRIPT'S OWN WORDS, and no record step is
/// launched; and THE NUMBER A VERIFIER HANDS ON IS NO RULING — the record script writes it
/// once — so an invocation that carries one is refused BEFORE ANY AGENT.
#[test]
fn a_budget_ruled_at_a_stop_is_recorded_and_starts_nothing() {
    if !can_run() {
        return;
    }
    let sim = Sim::budgeted("rule-budget", &one_area(), 1);
    let stage = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "another route is dead"]),
            "triage": {"entries": [a_route(ROUTE, 1), a_route(ELSEWHERE, 2)]},
            format!("verify:{ROUTE}"): refutes("intended"),
        }})),
    );
    assert_eq!(
        json!([
            stage.result["status"],
            stage.result["next"],
            stage.result["budget"]
        ]),
        json!(["verified", "rule", {"limit": 1, "spent": 1, "left": 0}]),
        "{}",
        stage.result
    );
    let stopped_at = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            state["position"]["test"],
            queued_keys(&state),
            row_of(&state, ELSEWHERE)["route"],
            row_of(&state, ELSEWHERE)["why"]
        ]),
        json!([{"refused": "round-open", "round": 1}, [ELSEWHERE], "human", "unreached"]),
        "the budget is spent, and a route item nobody reached is the human's: {state}"
    );

    // (1) WHAT IS NO RULING IS REFUSED BEFORE ANY AGENT: the number a verifier hands on, alone
    // or beside a raise — with the record script's reason, and no read.
    for rulings in [
        json!([{"hand-on": 3}]),
        json!([{"budget": 3}, {"hand-on": 3}]),
    ] {
        let refused = sim.invoke(sim.args("test", json!({"rulings": rulings})), json!({}));
        let message = text(&refused.result["message"]);
        assert_eq!(refused.result["status"], "refused", "{}", refused.result);
        assert!(
            message.starts_with("`hand-on` is no ruling")
                && message.contains("written ONCE")
                && message.contains("`exists`")
                && message.contains("Nothing was run."),
            "{message}"
        );
        assert_eq!(
            refused.trace,
            Vec::<String>::new(),
            "no agent ran, not a read"
        );
    }
    // … AND A RAISE THE STATE DOES NOT ASK FOR, AFTER THE TWO READS: one that raises nothing.
    let same = sim.invoke(
        sim.args("test", json!({"rulings": [{"budget": 1}]})),
        json!({}),
    );
    assert_eq!(
        json!([same.result["status"], same.result["next"]]),
        json!(["refused", "rule"]),
        "{}",
        same.result
    );
    assert!(
        text(&same.result["message"]).contains("`budget` raises it, and 1 does not")
            && text(&same.result["message"]).ends_with("Nothing was recorded."),
        "{}",
        same.result
    );
    assert_eq!(
        same.trace,
        lines(&TWO_READS),
        "nothing ran beyond the two reads"
    );
    assert_eq!(sim.rig.rev("HEAD"), stopped_at);

    // (2) THE RAISE, AT ITS OWN STOP: recorded by the one step, in ONE commit, pushed — and
    // nothing is started: no attempt is begun, no binary asked for, no verifier launched.
    let ruled = sim.invoke(
        sim.args("test", json!({"rulings": [{"budget": 3}]})),
        json!({}),
    );
    let result = &ruled.result;
    assert_eq!(result["status"], "ruled", "{result}");
    assert_eq!(
        ruled.agents(),
        only_rules(1),
        "two reads, the one step that records rulings, its gate, its commit, its push, the state: nothing is started"
    );
    assert_eq!(
        ruled_calls(&ruled),
        [json!([["run-set", "--run", RUN], {"budget": 3}])],
        "the raise is ONE call of the record script's, in the batch of the one step"
    );
    let recorded = sim.rig.rev("HEAD");
    let (subject, paths) = head_commit(&sim);
    assert_eq!(
        json!([
            subject,
            sim.rig.rev("HEAD~1"),
            sim.rig.remote(LOOP),
            result["record"],
            sim.rig.status()
        ]),
        json!([
            format!("docs(record): {RUN} r1 - the rulings of the human"),
            stopped_at,
            recorded,
            recorded,
            ""
        ]),
        "one commit, on the stage's last record, pushed: {result}"
    );
    assert_eq!(
        sorted(paths),
        [format!("{RUN_DIR}/r1/view.md"), format!("{RUN_DIR}/run.md")],
        "the run's facts, and the view of the round they changed — written by the record, and so no file is behind"
    );
    // WHAT THE RETURN SAYS: the budget as raised, the queue, the view the record wrote — and
    // `next`, now `verify`, with what the invocation that follows continues the stage for.
    assert_eq!(
        json!([
            result["next"],
            result["continues"],
            result["budget"],
            result["queue"]["length"],
            result["view"]["on_disk"]
        ]),
        json!(["verify", ["verify"], {"limit": 3, "spent": 1, "left": 2}, 1, "written"]),
        "{result}"
    );
    says_what_the_state_says(&sim, result);
    let state = sim.state();
    assert_eq!(
        json!([
            state["facts"]["budget"],
            state["next"],
            state["position"]["test"],
            row_of(&state, ELSEWHERE)["route"]
        ]),
        json!([3, "verify", {"round": 1, "attempt": 2, "verify": true}, "verify"]),
        "the state hands the stage that verifies its next attempt: {state}"
    );
    // A RAISE ANYWHERE ELSE IS REFUSED IN THE RECORD SCRIPT'S WORDS: budget is left now.
    let early = sim.invoke(
        sim.args("test", json!({"rulings": [{"budget": 5}]})),
        json!({}),
    );
    assert_eq!(early.result["status"], "refused", "{}", early.result);
    assert!(
        text(&early.result["message"]).contains(
            "raised only while it is spent and the queue holds a row nobody reached (1 of 3 spent, 1 in the queue)"
        ),
        "{}",
        early.result
    );
    assert_eq!(early.trace, lines(&TWO_READS));
    assert_eq!(sim.rig.rev("HEAD"), recorded);

    // (3) THE INVOCATION AFTER THE RULING CONTINUES THE STAGE: no instrument, the one row the
    // raised budget reaches, and its record.
    let again = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            format!("verify:{ELSEWHERE}"): files(json!({"verdict": "confirmed", "regression": false})),
        }}),
    );
    let result = &again.result;
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["budget"],
            result["queue"]["length"],
            result["continues"],
            result["next"]
        ]),
        json!(["verified", ["verify"], {"limit": 3, "spent": 2, "left": 1}, 0, [], "fix"]),
        "{result}"
    );
    assert_eq!(verified(&again), [ELSEWHERE], "{:?}", again.agents());
    assert_eq!(
        again.agents()[..5],
        continuation_opens(&sim, &text(&stage.result["candidate"]["sha"])),
        "{:?}",
        again.agents()
    );

    // (4) A RUN WITHOUT AN ORDER HAS NO QUEUE TO RULE ON: there each of the three is refused
    // after the two reads, and the record script is not asked — what a ruling writes at a
    // run's opening is the opening's to write. Such a run is what it was.
    let plain = Sim::opened("rule-no-queue", &one_area());
    let opened_at = plain.rig.rev("HEAD");
    for ruling in [
        json!({"budget": 3}),
        json!({"order": ["route hunt"]}),
        json!({"queued": [ELSEWHERE]}),
    ] {
        let refused = plain.invoke(
            plain.args("test", json!({"rulings": [ruling.clone()]})),
            json!({}),
        );
        assert_eq!(
            json!([refused.result["status"], refused.result["next"]]),
            json!(["refused", "test"]),
            "{ruling}: {}",
            refused.result
        );
        assert!(
            text(&refused.result["message"])
                .contains("this run has no queue: its opening holds no order")
                && text(&refused.result["message"]).ends_with("Nothing was recorded."),
            "{ruling}: {}",
            refused.result
        );
        assert_eq!(refused.trace, lines(&TWO_READS), "{ruling}");
        assert!(
            refused.result.get("budget").is_none() && refused.result.get("continues").is_none(),
            "a return of a run without an order says nothing of a queue: {}",
            refused.result
        );
    }
    assert_eq!(plain.rig.rev("HEAD"), opened_at);
    assert!(
        plain.state().get("queue").is_none(),
        "and it has none still"
    );
}

/// A rig whose run is opened with an order that ranks BEHAVIOUR ITEMS ONLY, the budget a
/// scenario names and the hand-on number 2 — and the stage both scenarios of an order ruled at
/// a stop run on it: a behaviour item, which is verified and whose verifier contests it, so
/// that the step is the human's; and two route items, which are IN NO QUEUE — their class is
/// off the order — and hold nothing.
fn off_the_order(label: &str, budget: u32) -> (Sim, Ran) {
    let sim = Sim::new(label);
    sim.open_with(
        &one_area(),
        &json!({"order": ["behaviour hunt"], "budget": budget, "hand-on": 2}),
    );
    let basis = "the decision of the write boundary says that a refusal never writes";
    let ran = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a slot is dropped", "a refusal prints a route that leads nowhere", "another route is dead"]),
            "triage": {"entries": [a_behaviour(INSIDE, 1), a_route(ROUTE, 2), a_route(ELSEWHERE, 3)]},
            format!("verify:{INSIDE}"): files(json!({"verdict": "confirmed", "regression": false, "contested": basis})),
        }})),
    );
    let state = sim.state();
    assert_eq!(
        json!([
            ran.result["status"],
            ran.result["next"],
            ran.result["queue"]["length"],
            state["next"],
            state["position"]["test"],
            queued_keys(&state),
            row_of(&state, ROUTE)["route"],
            row_of(&state, ROUTE)["why"],
            row_of(&state, ELSEWHERE)["why"]
        ]),
        json!(["verified", "rule", 0, "rule", {"refused": "round-open", "round": 1},
               [], "listed", "class-off-the-order", "class-off-the-order"]),
        "the step is the human's, and two rows nobody will verify are listed: {}\n{state}",
        ran.result
    );
    (sim, ran)
}

/// **An order ruled at a stop puts its rows into the queue, and the stage is continued** (the
/// reshape's plan, `T7`; `T2b`: *another order is taken at any step*; the ruling of 2026-10-11,
/// 3: *a class the human took off the order … is shown … and the human may put any of them into
/// the queue by key*). A stage returned with `next: rule` and two route items listed as *class
/// off the order*: nobody verifies them. THE HUMAN PUTS ONE IN BY ITS KEY, and then RULES
/// ANOTHER ORDER, which names their class — each by an invocation that RECORDS IT AND STARTS
/// NOTHING. After each the state ranks the row in the queue, `next` is `verify`, and the
/// position hands the stage that verifies its next attempt: THE INVOCATION THAT FOLLOWS IS A
/// CONTINUATION, and it verifies what the rulings put in.
#[test]
fn an_order_ruled_at_a_stop_puts_its_rows_into_the_queue_and_the_stage_is_continued() {
    if !can_run() {
        return;
    }
    let (sim, stage) = off_the_order("rule-order", 30);
    let stopped_at = sim.rig.rev("HEAD");

    // (1) A ROW PUT IN BY ITS KEY: one call, one commit, nothing started — and that row alone
    // is in the queue, ranked after every class the order names.
    let put_in = sim.invoke(
        sim.args("test", json!({"rulings": [{"queued": [ELSEWHERE]}]})),
        json!({}),
    );
    let result = &put_in.result;
    assert_eq!(result["status"], "ruled", "{result}");
    assert_eq!(put_in.agents(), only_rules(1), "nothing is started");
    assert_eq!(
        ruled_calls(&put_in),
        [json!([["run-set", "--run", RUN], {"queued": [ELSEWHERE]}])]
    );
    let state = sim.state();
    assert_eq!(
        json!([
            result["next"],
            result["continues"],
            result["queue"]["length"],
            result["queue"]["classes"],
            state["position"]["test"],
            queued_keys(&state),
            row_of(&state, ROUTE)["route"]
        ]),
        json!(["verify", ["verify"], 1,
               [{"kind": "behaviour", "origin": "hunt", "length": 0},
                {"kind": "route", "origin": "hunt", "length": 1}],
               {"round": 1, "attempt": 2, "verify": true}, [ELSEWHERE], "listed"]),
        "{result}\n{state}"
    );
    says_what_the_state_says(&sim, result);
    assert_eq!(sim.rig.rev("HEAD~1"), stopped_at, "one commit");

    // (2) ANOTHER ORDER: recorded, nothing started — and every row of the class it names is in
    // the queue, the one put in by key in its class's own place.
    let order = json!(["route hunt", "behaviour hunt"]);
    let ruled = sim.invoke(
        sim.args("test", json!({"rulings": [{"order": order}]})),
        json!({}),
    );
    let result = &ruled.result;
    assert_eq!(result["status"], "ruled", "{result}");
    assert_eq!(ruled.agents(), only_rules(2), "nothing is started");
    assert_eq!(
        ruled_calls(&ruled),
        [json!([["run-set", "--run", RUN], {"order": order}])]
    );
    let recorded = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            result["next"],
            result["continues"],
            result["budget"],
            result["queue"]["length"],
            result["queue"]["classes"],
            result["record"],
            sim.rig.remote(LOOP),
            state["facts"]["order"],
            state["next"],
            state["position"]["test"],
            queued_keys(&state)
        ]),
        json!(["verify", ["verify"], {"limit": 30, "spent": 1, "left": 29}, 2,
               [{"kind": "route", "origin": "hunt", "length": 2},
                {"kind": "behaviour", "origin": "hunt", "length": 0}],
               recorded, recorded, order, "verify",
               {"round": 1, "attempt": 2, "verify": true}, [ROUTE, ELSEWHERE]]),
        "{result}\n{state}"
    );
    says_what_the_state_says(&sim, result);
    holds_no_list(
        &json!({"next": result["next"], "continues": result["continues"],
                          "budget": result["budget"], "queue": result["queue"],
                          "view": result["view"], "named": result["named"]}),
        &[ROUTE, ELSEWHERE, INSIDE],
    );

    // (3) THE INVOCATION THAT FOLLOWS CONTINUES THE STAGE'S VERIFICATION: no instrument, the
    // two rows the rulings put into the queue, and its record.
    let again = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            format!("verify:{ROUTE}"): refutes("intended"),
            format!("verify:{ELSEWHERE}"): refutes("does-not-reproduce"),
        }}),
    );
    let result = &again.result;
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["budget"],
            result["queue"]["length"]
        ]),
        json!(["verified", ["verify"], {"limit": 30, "spent": 3, "left": 27}, 0]),
        "{result}"
    );
    assert_eq!(verified(&again), [ROUTE, ELSEWHERE], "{:?}", again.agents());
    assert_eq!(
        again.agents()[..5],
        continuation_opens(&sim, &text(&stage.result["candidate"]["sha"])),
        "{:?}",
        again.agents()
    );
}

/// **An order ruled while the run is stopped after a round puts the round's rows into the
/// queue, and no go is owed first** (the reshape's plan, `T7`; the state the first real use is
/// likeliest to meet). The order a stage ran under names no class any row has: every row is
/// listed as *class off the order*, nothing is open, nothing was verified — and the run stops
/// after every round, so `next` is `stop`, the stop's next step would be the close, and the
/// `test` stage is refused as `stopped`. THE HUMAN RULES ANOTHER ORDER THERE. The invocation is
/// taken, the order is recorded by the one step — and nothing is started. AFTERWARDS THE ROUND
/// IS NOT OVER ANY MORE: the rows are in the queue, the state names no stop, `next` is `verify`,
/// and the position hands the stage that verifies its next attempt. THE INVOCATION THAT
/// FOLLOWS IS A CONTINUATION — it verifies the rows and records — and NO `go` WAS RECORDED OR
/// ASKED FOR: the stop that would have asked for one is not the state's any more.
#[test]
fn an_order_ruled_while_the_run_is_stopped_after_a_round_reopens_its_queue_and_no_go_is_owed() {
    if !can_run() {
        return;
    }
    let sim = Sim::new("rule-order-at-stop");
    sim.open_with(
        &one_area(),
        &json!({"order": ["behaviour hunt"], "budget": 30, "hand-on": 2}),
    );
    let stage = sim.invoke(
        sim.args("test", json!({})),
        small_stage(json!({"agents": {
            "area-a:review": findings(&["a refusal prints a route that leads nowhere", "another route is dead"]),
            "triage": {"entries": [a_route(ROUTE, 1), a_route(ELSEWHERE, 2)]},
        }})),
    );
    let stopped_at = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            stage.result["status"],
            stage.result["next"],
            stage.result["continues"],
            stage.result["verify"]["waves"],
            stage.result["queue"]["length"],
            state["stop"],
            state["position"]["test"],
            row_of(&state, ROUTE)["route"],
            row_of(&state, ROUTE)["why"],
            row_of(&state, ELSEWHERE)["why"]
        ]),
        json!(["hunted", "stop", [], 0, 0,
               {"why": "every-round", "round": 1, "then": "close"},
               {"refused": "stopped", "round": 1},
               "listed", "class-off-the-order", "class-off-the-order"]),
        "every row is listed, nothing is open, and the run is stopped after its round: {}\n{state}",
        stage.result
    );
    assert_eq!(
        stage.result["view"]["listed"]
            .as_array()
            .expect("the words that keep a row out of the queue")
            .iter()
            .find(|word| word["why"] == "class-off-the-order")
            .map(|word| word["rows"].clone()),
        Some(json!(2)),
        "the return says how many rows the order keeps out, and names none: {}",
        stage.result
    );

    // THE ORDER, RULED AT THAT STOP: taken, recorded in one commit, pushed — nothing started.
    let order = json!(["route hunt", "behaviour hunt"]);
    let ruled = sim.invoke(
        sim.args("test", json!({"rulings": [{"order": order}]})),
        json!({}),
    );
    let result = &ruled.result;
    assert_eq!(result["status"], "ruled", "{result}");
    assert_eq!(ruled.agents(), only_rules(1), "nothing is started");
    assert_eq!(
        ruled_calls(&ruled),
        [json!([["run-set", "--run", RUN], {"order": order}])],
        "the order, and no go beside it"
    );
    let recorded = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            sim.rig.remote(LOOP),
            result["record"],
            result["next"],
            result["continues"],
            result["budget"],
            result["queue"]["length"],
            state["facts"]["order"],
            state["next"],
            state["stop"],
            state["position"]["test"],
            state["rounds"][0]["facts"]["go"],
            queued_keys(&state),
            row_of(&state, ROUTE)["route"],
            row_of(&state, ROUTE)["why"]
        ]),
        json!([stopped_at, recorded, recorded, "verify", ["verify"],
               {"limit": 30, "spent": 0, "left": 30}, 2, order, "verify", null,
               {"round": 1, "attempt": 2, "verify": true}, false,
               [ROUTE, ELSEWHERE], "verify", "queued"]),
        "the round is open again, the state names no stop, and no go is on record: {result}\n{state}"
    );
    says_what_the_state_says(&sim, result);

    // THE INVOCATION AFTER IT IS A CONTINUATION, with no go between: no instrument, the two
    // rows the order put into the queue, and the record of what was filed.
    let again = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            format!("verify:{ROUTE}"): refutes("intended"),
            format!("verify:{ELSEWHERE}"): refutes("does-not-reproduce"),
        }}),
    );
    let result = &again.result;
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["budget"],
            result["queue"]["length"],
            result["continues"],
            result["next"]
        ]),
        json!(["verified", ["verify"], {"limit": 30, "spent": 2, "left": 28}, 0, [], "stop"]),
        "{result}"
    );
    assert_eq!(verified(&again), [ROUTE, ELSEWHERE], "{:?}", again.agents());
    assert_eq!(
        again.agents()[..5],
        continuation_opens(&sim, &text(&stage.result["candidate"]["sha"])),
        "{:?}",
        again.agents()
    );
    assert_eq!(
        sim.state()["position"]["test"],
        json!({"refused": "stopped", "round": 1}),
        "with every row verified the round is over, and the run is stopped after it — as it was"
    );
}

/// **An order and a budget ruled in one invocation are one record of two calls** (the
/// reshape's plan, `T7`; an arm the plan does not list, which the first real use opens with).
/// The record script holds a raise to the state as it stands BEFORE its call: the budget
/// spent, and a row of the queue unreached. HERE THE BUDGET IS SPENT AND THE QUEUE IS EMPTY —
/// the rows nobody verified are off the order — so a raise alone is refused, and so would one
/// call that holds both facts be: the order it carries is not written when it is judged. SO
/// THE TWO ARE TWO CALLS OF ONE BATCH, THE ORDER FIRST, WHICHEVER WAS SAID FIRST: a call of a
/// batch reads what the calls before it wrote, the order puts the rows into the queue, and the
/// raise is judged on that. ONE record, one gate, one commit; nothing is started; and the
/// invocation after it continues the stage. WHERE THE RAISE IS REFUSED ON THE STATE THE ORDER
/// LEAVES, THE BATCH IS REFUSED WHOLE: the order is not written either, the halt carries the
/// record script's own line, and nothing is committed or left pending.
#[test]
fn an_order_and_a_budget_ruled_in_one_invocation_are_one_record_of_two_calls() {
    if !can_run() {
        return;
    }
    let (sim, stage) = off_the_order("rule-both", 1);
    let stopped_at = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([state["budget"], queued_keys(&state)]),
        json!([{"limit": 1, "spent": 1, "left": 0}, []]),
        "the budget is spent, and the queue holds no row: {state}"
    );

    // (1) THE RAISE ALONE: refused after the two reads, in the record script's words.
    let alone = sim.invoke(
        sim.args("test", json!({"rulings": [{"budget": 3}]})),
        json!({}),
    );
    assert_eq!(alone.result["status"], "refused", "{}", alone.result);
    assert!(
        text(&alone.result["message"]).contains(
            "raised only while it is spent and the queue holds a row nobody reached (1 of 1 spent, 0 in the queue)"
        ),
        "{}",
        alone.result
    );
    assert_eq!(alone.trace, lines(&TWO_READS));

    // (2) BESIDE AN ORDER THAT PUTS NO ROW INTO THE QUEUE: the harness cannot read the state
    // the order leaves, so the batch is composed — and THE RECORD SCRIPT REFUSES IT WHOLE at
    // its second call. Nothing is written: not the order either.
    let empty = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"budget": 3}, {"order": ["behaviour hunt", "behaviour seed"]}]}),
        ),
        json!({}),
    );
    let result = &empty.result;
    assert_eq!(
        json!([result["status"], result["halted"]["phase"]]),
        json!(["halted", "rulings"]),
        "{result}"
    );
    assert_eq!(
        empty.agents(),
        [
            "git:state",
            "git:state:test-1",
            RULINGS_CHECK,
            "record:rulings:r1"
        ],
        "the batch is refused where it is applied: no gate, no commit step, no push"
    );
    assert!(
        result
            .to_string()
            .contains("the budget: the state does not ask for it"),
        "the halt carries the record script's own line: {result}"
    );
    let state = sim.state();
    assert_eq!(
        json!([
            state["facts"]["order"],
            state["facts"]["budget"],
            sim.rig.rev("HEAD"),
            sim.record(&["pending", "--run", RUN], "")["batch"],
            sim.rig.status()
        ]),
        json!([["behaviour hunt"], 1, stopped_at, null, ""]),
        "a batch is whole or nothing: the order its first call wrote is on no disk: {state}"
    );

    // (3) BESIDE THE ORDER THAT NAMES THE ROWS' CLASS — the raise said FIRST: one record, its
    // two calls the order and then the raise.
    let order = json!(["route hunt", "behaviour hunt"]);
    let ruled = sim.invoke(
        sim.args(
            "test",
            json!({"rulings": [{"budget": 3}, {"order": order}]}),
        ),
        json!({}),
    );
    let result = &ruled.result;
    assert_eq!(result["status"], "ruled", "{result}");
    assert_eq!(
        ruled_calls(&ruled),
        [
            json!([["run-set", "--run", RUN], {"order": order}]),
            json!([["run-set", "--run", RUN], {"budget": 3}]),
        ],
        "the order first, whichever was said first: the raise is judged on the state it leaves"
    );
    assert_eq!(
        ruled.agents(),
        only_rules(1),
        "ONE record — one gate, one commit, one push — and nothing is started"
    );
    let recorded = sim.rig.rev("HEAD");
    let state = sim.state();
    assert_eq!(
        json!([
            sim.rig.rev("HEAD~1"),
            sim.rig.remote(LOOP),
            result["record"],
            result["next"],
            result["continues"],
            result["budget"],
            result["queue"]["length"],
            state["facts"]["order"],
            state["facts"]["budget"],
            state["position"]["test"],
            queued_keys(&state)
        ]),
        json!([stopped_at, recorded, recorded, "verify", ["verify"],
               {"limit": 3, "spent": 1, "left": 2}, 2, order, 3,
               {"round": 1, "attempt": 2, "verify": true}, [ROUTE, ELSEWHERE]]),
        "{result}\n{state}"
    );
    says_what_the_state_says(&sim, result);

    // (4) THE INVOCATION AFTER IT CONTINUES THE STAGE: the two rows, inside the raised budget.
    let again = sim.invoke(
        sim.args("test", json!({})),
        json!({"agents": {
            format!("verify:{ROUTE}"): refutes("intended"),
            format!("verify:{ELSEWHERE}"): files(json!({"verdict": "confirmed", "regression": false})),
        }}),
    );
    let result = &again.result;
    assert_eq!(
        json!([
            result["status"],
            result["continued"],
            result["budget"],
            result["queue"]["length"]
        ]),
        json!(["verified", ["verify"], {"limit": 3, "spent": 3, "left": 0}, 0]),
        "{result}"
    );
    assert_eq!(verified(&again), [ROUTE, ELSEWHERE], "{:?}", again.agents());
    assert_eq!(
        again.agents()[..5],
        continuation_opens(&sim, &text(&stage.result["candidate"]["sha"])),
        "{:?}",
        again.agents()
    );

    // (5) THE SAME TWO RULINGS, SAID IN THE OTHER ORDER, ARE THE SAME BATCH, BYTE FOR BYTE: a
    // second rig, the order said first.
    let (other, _) = off_the_order("rule-both-reversed", 1);
    let reversed = other.invoke(
        other.args(
            "test",
            json!({"rulings": [{"order": order}, {"budget": 3}]}),
        ),
        json!({}),
    );
    assert_eq!(reversed.result["status"], "ruled", "{}", reversed.result);
    assert_eq!(ruled_calls(&reversed), ruled_calls(&ruled));
    let batch_of = |ran: &Ran, scratch: &Path| {
        let apply = ran
            .commands
            .iter()
            .find(|command| command.starts_with("dev/stabilize-record apply "))
            .expect("the batch was applied")
            .clone();
        let file = flag_of(&apply, "--from").to_owned();
        (
            file.trim_start_matches(&scratch.display().to_string())
                .to_owned(),
            fs::read(&file).expect("the batch"),
            flag_of(&apply, "--sha256").to_owned(),
        )
    };
    assert_eq!(
        batch_of(&reversed, &other.rig.scratch),
        batch_of(&ruled, &sim.rig.scratch),
        "one file under one name, the same bytes and the same hash, whichever ruling was said first"
    );
    let state = other.state();
    assert_eq!(
        json!([
            state["facts"]["order"],
            state["facts"]["budget"],
            queued_keys(&state),
            state["next"]
        ]),
        json!([order, 3, [ROUTE, ELSEWHERE], "verify"]),
        "{state}"
    );
}

/// **A halt after silent calls says that it may be the limit** (the reshape's plan, `T7`, for
/// what `T6b` built: §2.2 — *the account's usage limit reaches the script as a null return*; the
/// row `wf-return-names`: *a halt the usage limit caused that says "the report check could not
/// be read"*). Two verifier calls of a wave return nothing, the wave's third files — AND THEN
/// THE WAVE'S OWN REPORT CHECK RETURNS NOTHING TOO, or the read of the queue after it. THE HALT
/// SAYS WHAT IT MAY BE, in a member of its own (`limit`), and NAMES WHAT WAS FILED: here by the
/// path the verifier returned, HELD BY NOTHING (`unconfirmed`) — neither the report check nor
/// the queue came back to hold it — with the state's count AS IT WAS LAST READ, which is before
/// the wave. And it carries what every return of a run with a queue carries, as last read: the
/// budget, the queue and the stop's view. The verdict is on disk, the state counts it, and the
/// invocation that follows is handed the stage's record.
#[test]
fn a_halt_after_silent_calls_says_that_it_may_be_the_limit() {
    if !can_run() {
        return;
    }
    for (label, silent, phase, step, named) in [
        (
            "limit-check",
            "git:check-reports:wave1",
            "reports",
            "report-check",
            "the report check after that wave",
        ),
        (
            "limit-read",
            "git:state:test-4",
            "state",
            "queue",
            "the read of the queue after that wave",
        ),
    ] {
        let tried = silent;
        let sim = Sim::budgeted(label, &one_area(), 30);
        let handed_on = "the-repair-route-is-dead";
        let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
        script["endings"] = json!({
            format!("verify:{ROUTE}"): "dies",
            format!("verify:{elsewhere}"): "dies",
            silent: "dies",
        });
        let ran = sim.invoke(sim.args("test", json!({})), script);
        let result = &ran.result;
        assert_eq!(
            json!([
                result["status"],
                result["halted"]["phase"],
                result["halted"]["transient"],
                result["halted"]["limit"],
                result["halted"]["filed"]
            ]),
            json!(["halted", phase, true, {"possible": true, "wave": 1, "silent": 2, "step": step},
                   {"counted": 0, "read": "before-the-wave", "commit": "none", "record": null,
                    "files": [], "unconfirmed": [filed_at(INSIDE)],
                    "directory": format!("{RUN_DIR}/r1/verdicts")}]),
            "{label}: {result}"
        );
        assert_eq!(tries_of(&ran, tried), 3, "{label}: {:?}", ran.agents());
        let message = text(&result["message"]);
        assert!(
            message.contains("THIS MAY BE THE ACCOUNT'S USAGE LIMIT")
                && message.contains("2 verifier call(s) of wave 1 returned nothing")
                && message.contains(&format!("held by nothing: {}", filed_at(INSIDE)))
                && !message.contains("is the account's limit"),
            "{label}: the halt says what it may be, never that it is, and names the path nothing held: {message}"
        );
        // WHAT IS SO AT THIS STEP (the scoped review before the parts, the harness's finding 3):
        // the state was last read BEFORE the wave, so the document the halt names lists nothing
        // the wave filed, and the count is of then — the halt says so, and names the ONE PLAIN
        // CALL that prints the state as it stands now. (Until that repair it said that *the
        // state document this return names lists every filed verdict* — of a document whose
        // `filed` is empty, as this scenario held, and holds, below.)
        assert!(
            message.contains(&format!("and then {named} returned nothing too"))
                && message.contains("THE STATE WAS LAST READ BEFORE THAT WAVE")
                && message.contains("it counted 0 verdict(s) in round 1 then")
                && message.contains(&format!(
                    "`dev/stabilize-record state --run {RUN}` prints the state as it stands now"
                ))
                && message.contains("IS IN NO COMMIT YET")
                && message.contains("it CONTINUES")
                && !message.contains("lists every filed verdict"),
            "{label}: {message}"
        );
        // WHAT EVERY RETURN OF A RUN WITH A QUEUE CARRIES, AS THE STATE WAS LAST READ — before
        // the wave: nothing spent, three rows in the queue, the view the first record wrote.
        assert_eq!(
            json!([
                result["budget"],
                result["queue"]["length"],
                result["view"]["on_disk"],
                result["view"]["verified"],
                result["continues"]
            ]),
            json!([{"limit": 30, "spent": 0, "left": 30}, 3, "written", 0, ["verify"]]),
            "{label}: {result}"
        );
        says_what_the_state_says(&sim, result);
        assert!(
            document(result)["filed"] == json!([]),
            "{label}: the document the halt names is the state as it was last read, and lists no filing"
        );
        // THE VERDICT IS ON DISK, AND THE STATE COUNTS IT: the invocation that follows is
        // handed the stage — its verification, and the record of what was filed.
        assert!(sim.rig.root.join(filed_at(INSIDE)).exists(), "{label}");
        let state = sim.state();
        assert_eq!(
            json!([
                state["budget"]["spent"],
                state["next"],
                state["position"]["test"],
                queued_keys(&state)
            ]),
            json!([1, "verify", {"round": 1, "attempt": 2, "verify": true, "filed": true},
                   [handed_on, ROUTE, elsewhere]]),
            "{label}: {state}"
        );
    }
}

/// **The limit's halt says what is so at each step of the record** (the scoped review before
/// the parts, the harness's finding 3, its second face). After a wave that was the last for its
/// silence the stage's second record is composed — and then a step of it returns nothing: the
/// step that starts its gate, its commit step, its push, or the read of the state after it.
/// The same sentences were appended at each: at the push the halt said *the record of what the
/// verifiers filed is committed on <branch> (<sha>)* and, in the same message, *WHAT WAS FILED
/// IS NOT LOST, AND IS IN NO COMMIT YET … <a file>* — of a file that commit holds; and *it
/// CONTINUES*, where the next invocation's first read finishes a batch or makes a push first.
/// HERE THE HALT IS HANDED WHICH STEP HALTED. At the gate no commit holds what was filed, and
/// it says so; at the commit step it says that IT CANNOT TELL whether the commit was made; at
/// the push and at the read-back it names the commit that took what was filed, and whether it
/// is pushed; and at all four it leaves what the next invocation does first to the sentence
/// the halt of that step already has. AND A GATE WHOSE START THE TOOL REFUSED IS NO SUCH HALT:
/// that step answered.
#[test]
fn the_limits_halt_says_what_is_so_at_each_step_of_the_record() {
    if !can_run() {
        return;
    }
    let gate = "record-verdicts-r1-a1-1";
    for (label, silent, phase, step, named, commit) in [
        (
            "limit-gate",
            format!("git:hold-start:{gate}"),
            "record",
            "gate",
            "a step of the record's gate",
            "none",
        ),
        (
            "limit-commit",
            "git:record:verdicts:r1".to_owned(),
            "record",
            "commit",
            "the record's commit step",
            "unknown",
        ),
        (
            "limit-push",
            "git:push:verdicts".to_owned(),
            "push",
            "push",
            "the push of the record",
            "made",
        ),
        (
            "limit-read-back",
            "git:state:test-5".to_owned(),
            "state",
            "read-back",
            "the read of the state after the record",
            "pushed",
        ),
    ] {
        let sim = Sim::budgeted(label, &one_area(), 30);
        let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
        script["endings"] = json!({
            format!("verify:{ROUTE}"): "dies",
            format!("verify:{elsewhere}"): "dies",
            silent.as_str(): "dies",
        });
        let ran = sim.invoke(sim.args("test", json!({})), script);
        let result = &ran.result;
        let head = sim.rig.rev("HEAD");
        let (subject, paths) = head_commit(&sim);
        let committed = commit == "made" || commit == "pushed";
        assert_eq!(
            json!([
                subject == second_record_subject(),
                paths.contains(&filed_at(INSIDE))
            ]),
            json!([committed, committed]),
            "{label}: the control — whether the second record is committed, with the filed verdict in it: {subject} {paths:?}"
        );
        assert_eq!(
            json!([
                result["status"],
                result["halted"]["phase"],
                result["halted"]["transient"],
                result["halted"]["limit"],
                result["halted"]["filed"]
            ]),
            json!(["halted", phase, true, {"possible": true, "wave": 1, "silent": 2, "step": step},
                   {"counted": 1, "read": "after-the-wave", "commit": commit,
                    "record": if committed { json!(head) } else { Value::Null },
                    "files": [filed_at(INSIDE)], "unconfirmed": [],
                    "directory": format!("{RUN_DIR}/r1/verdicts")}]),
            "{label}: {result}"
        );
        assert_eq!(tries_of(&ran, &silent), 3, "{label}: {:?}", ran.agents());
        let message = text(&result["message"]);
        assert!(
            message.contains("THIS MAY BE THE ACCOUNT'S USAGE LIMIT")
                && message.contains(&format!("and then {named} returned nothing too"))
                && message.contains(&filed_at(INSIDE))
                && !message.contains("it CONTINUES")
                && !message.contains("BEFORE THAT WAVE")
                && !message.contains("is the account's limit"),
            "{label}: {message}"
        );
        // WHAT IT SAYS OF THE FILED VERDICTS IS WHAT THE RECORD MADE OF THEM BY THAT STEP …
        assert_eq!(
            json!([
                message.contains("WHAT WAS FILED IS NOT LOST, AND IS IN NO COMMIT YET"),
                message.contains("WHETHER A COMMIT HOLDS IT THIS SCRIPT CANNOT SAY"),
                message.contains("WHAT WAS FILED IS NOT LOST, AND IS IN A COMMIT")
                    && message.contains(&head)
            ]),
            json!([commit == "none", commit == "unknown", committed]),
            "{label}: {message}"
        );
        // … and whether that commit is pushed, as the remote stands.
        let pushed = sim.rig.remote(LOOP).as_deref() == Some(head.as_str());
        assert_eq!(
            json!([
                committed && pushed,
                message.contains("and is pushed"),
                message.contains("THE PUSH IS OWED")
            ]),
            json!([commit == "pushed", commit == "pushed", commit == "made"]),
            "{label}: {message}"
        );
    }

    // A GATE WHOSE START THE TOOL REFUSED IS NO SUCH HALT: a command of another kind stands
    // under the record's gate's name, so the start is refused (`taken`) — the step answered,
    // and nothing here looks like a limit. (Until that repair every end of a record's gate
    // that left no verdict — a start refused, a gate still running — was said to be a step
    // that *returned nothing too*.)
    let sim = Sim::budgeted("limit-gate-refused", &one_area(), 30);
    let scratch = sim.rig.scratch.display().to_string();
    let other = sim.step(&[
        "hold-start",
        "--scratch",
        &scratch,
        "--name",
        gate,
        "--kind",
        "probe",
        "--seconds",
        "1",
    ]);
    assert_eq!(other["status"], "started", "{other}");
    let (mut script, elsewhere) = three_in_a_wave("the repair route is dead");
    script["endings"] = json!({
        format!("verify:{ROUTE}"): "dies",
        format!("verify:{elsewhere}"): "dies",
    });
    let refused = sim.invoke(sim.args("test", json!({})), script);
    let result = &refused.result;
    assert_eq!(
        json!([
            result["status"],
            result["halted"]["phase"],
            result["halted"]["held"]["name"],
            result["halted"]["limit"],
            result["halted"]["filed"]
        ]),
        json!(["halted", "record", gate, null, null]),
        "{result}"
    );
    assert!(
        text(&result["message"]).contains("`taken`")
            && !text(&result["message"]).contains("USAGE LIMIT"),
        "{result}"
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
// A machine without gitleaks
// ---------------------------------------------------------------------------

/// The arm that scans a composed record with the machine's own gitleaks, on a machine
/// without gitleaks, driven: this binary is run again for that one test, with a `PATH` on
/// which only `node` answers. It passes, and NOT SILENTLY — the skip is on stderr and in the
/// file the gate names, under CI as anywhere else (CI's tooling legs carry no gitleaks:
/// [`gitleaks_or_skip`]).
#[test]
fn without_gitleaks_the_record_scan_arm_passes_and_tells_the_gate() {
    if !can_run() {
        return;
    }
    let here = RigDir::new("stabilize-sim-no-gitleaks");
    let bin = here.path().join("bin");
    fs::create_dir_all(&bin).expect("create the bin directory");
    let node = std::env::split_paths(&std::env::var_os("PATH").expect("a PATH"))
        .map(|dir| dir.join("node"))
        .find(|candidate| candidate.is_file())
        .expect("node is on PATH");
    std::os::unix::fs::symlink(&node, bin.join("node")).expect("link node");
    for ci in [None, Some("true")] {
        let skips = here.path().join(format!("skips-{}", ci.unwrap_or("local")));
        let mut command = Command::new(std::env::current_exe().expect("this test binary"));
        command
            .args([
                "--exact",
                "stabilize_simulation::a_record_composed_from_the_regression_tools_own_line_passes_the_machines_own_gitleaks",
                "--nocapture",
            ])
            .env("PATH", &bin)
            .env("JIGC_GATE_SKIPS", &skips)
            .env_remove("CI");
        if let Some(ci) = ci {
            command.env("CI", ci);
        }
        let out = command.output().expect("run this test binary");
        let said = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(out.status.success(), "CI {ci:?}: the test passes: {said}");
        let skipped = "SKIPPED: `gitleaks` is not on PATH, so no record composed from the regression tool's own line was scanned by gitleaks";
        assert!(
            said.contains(skipped),
            "CI {ci:?}: and says so on stderr: {said}"
        );
        assert_eq!(
            fs::read_to_string(&skips).expect("the gate's skip file"),
            format!("{skipped}\n"),
            "CI {ci:?}: and in the file the gate names, which is where somebody sees it"
        );
    }
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
