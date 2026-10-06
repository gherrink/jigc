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
//! - *The full gate — the candidate's, which the preflight runs, and the one a record step
//!   runs.* The gate runs this suite; each stand-in writes what a full gate prints — green,
//!   or red with the steps and tests a scenario scripts — to the file its prompt names. What
//!   reads that file is real: `dev/stabilize-record gate-set` and `gate-check`, and the
//!   commit step that holds one to the other.
//! - *gitleaks, of the two hygiene scanners.* A test runner has none; the stand-in is the
//!   record suite's, which keeps gitleaks' exit contract
//!   ([`gitleaks_stub`]). The denylist half is the real `dev/hygiene-scan`, over a denylist
//!   this suite writes.
//! - *The builds of a binary and of a trial image.* A "binary" is a file the preflight's
//!   stand-in writes at the path its prompt names. Its sha256 is real — measured by the
//!   preflight, and measured again by every stand-in that "drives" it — so the comparison
//!   the harness makes of the two is a comparison of two measurements.
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
//! - **Scripted agents are not agents.** That a model runs one command and relays a line
//!   of several kilobytes whole, writes a payload byte for byte, or reads its definition
//!   as this stand-in does, is shown by nothing here.
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
//! finding by its id; the task that repairs it turns the assertion. **No pin of the `test`
//! stage is left.** *Turned by the repair's last task of that stage's harness, and asserted
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
//! **The `fix` stage refuses to start**, and that is all that is asserted of it; the rig,
//! the scenario and the stand-in are shaped so that its half's tasks add its agents here.
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

use super::dev_stabilize_record::gitleaks_stub;
use super::dev_stabilize_step::{LOOP, RUN, RUN_DIR, StepRig, node_or_skip};
use super::placed_executable;

const HARNESS: &str = ".claude/workflows/stabilize.js";
const RUNTIME: &str = "tooling-tests/fixtures/stabilize-runtime.mjs";
const TRACE: &str = "tooling-tests/fixtures/stabilize-test-stage.trace";
const RECORD: &str = "dev/stabilize-record";
const SCANNER: &str = "dev/hygiene-scan";
const GITLEAKS_CONFIG: &str = ".gitleaks.toml";

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

impl Sim {
    /// A repository with a remote, the scanners a write of the record script needs, and
    /// the loop branch of a run nobody has opened yet checked out.
    fn new(label: &str) -> Self {
        let rig = StepRig::unopened(&format!("sim-{label}"));
        placed_executable::copy(&repo_root().join(SCANNER), &rig.root.join(SCANNER));
        let config = fs::read(repo_root().join(GITLEAKS_CONFIG)).expect("read the gitleaks config");
        fs::write(rig.root.join(GITLEAKS_CONFIG), config).expect("write the rig's gitleaks config");
        rig.commit("chore: the hygiene scan");
        rig.git(&["push", "-q", "origin", "main"]);
        rig.on_path("gitleaks", &gitleaks_stub());
        fs::write(
            rig.dir().join("denylist"),
            "# a private term\n\nzzyzxhost\n",
        )
        .expect("write the denylist");
        fs::create_dir_all(rig.dir().join("tmp")).expect("create the rig's temp directory");
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
            .env("JIGC_DENYLIST_FILE", self.rig.dir().join("denylist"))
            .env(
                "TMPDIR",
                format!("{}/", self.rig.dir().join("tmp").display()),
            )
            .current_dir(&self.rig.root);
        command
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
        Ran {
            result: said["result"].clone(),
            trace,
            logs: list("logs"),
            reporters: list("reporters"),
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
    golden(TRACE, &format!("{}\n", ran.trace.join("\n")));

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
        Some("docs(record): rc24 r1 — the test stage's record"),
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

    // (5) THE RETURN THE ORCHESTRATOR READS — `next` as the record script computes it.
    assert_eq!(state["next"], "rule", "{state}");
    assert_eq!(
        result["next"], state["next"],
        "`next` is relayed, never computed: {result}"
    );
    assert_eq!(
        result["rule"]["findings"],
        json!([{"key": INSIDE, "why": "fork"}, {"key": OUTSIDE, "why": "outside"}])
    );
    assert_eq!(result["human_list"], state["human_list"]);
    assert_eq!(result["blockers"], json!([]));
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
    assert_eq!(stranger.result["status"], "refused", "{}", stranger.result);
    assert_eq!(
        stranger.trace,
        lines(&TWO_READS),
        "a ruling on no row records nothing"
    );
    let ruled = sim.invoke(sim.args("test", json!({"rulings": rulings})), json!({}));
    assert_eq!(ruled.result["status"], "ruled", "{}", ruled.result);
    assert_eq!(
        ruled.agents(),
        [
            "git:state",
            "git:state:test-1",
            "record:rulings:r1",
            "git:record:rulings:r1",
            "git:push:rulings",
            "git:state:test-2"
        ],
        "two reads, the one step that records rulings, its commit, the push, the state: nothing is started"
    );
    // M3: THE FORK IS STILL UNRULED, so the next step is still the human's — never `fix`,
    // though nothing else is left to rule on.
    assert_eq!(
        json!([ruled.result["next"], ruled.result["rule"]["findings"]]),
        json!(["rule", [{"key": INSIDE, "why": "fork"}]]),
        "M3: {}",
        ruled.result
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
        fork_ruled.agents()[2..4],
        ["record:rulings:r1", "git:record:rulings:r1"],
        "M3: by the one step that records rulings"
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

/// The agents of an invocation that only finishes a round's triage, in order.
fn finishing_lap(verified: &[&str], last_state: &str) -> Vec<String> {
    let mut agents = lines(&[
        "git:state",
        "git:state:test-1",
        "git:begin",
        "preflight",
        "triage:p1",
    ]);
    agents.extend(verified.iter().map(|key| format!("verify:{key}")));
    agents.extend(lines(&[
        "git:check-reports:p1",
        "record:triage:r1",
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
    assert_eq!(
        result["state"]["untriaged"],
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
        finishing_lap(&[key], "git:state:test-2"),
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
        format!("docs(record): {RUN} r1 — the round's triage, finished")
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
            finished["rule"]["findings"]
        ]),
        json!(["rule", [{"key": key, "why": "unverified-after-retry"}],
               [{"key": key, "why": "unverified-after-retry"}]]),
        "F3: {state}"
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
    assert!(
        reason.starts_with("gate-red: ") && reason.contains(&format!("test {FLAKY}")),
        "the halt names what is red only with the records: {reason}"
    );
    assert_eq!(
        halted["halted"]["gate"]["new"],
        json!(["step test", format!("test {FLAKY}")])
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
        json!(["docs(record): rc24 r1 — the test stage's record", 1, []])
    );

    // THE HALT SAYS WHAT TO DO, AND IT IS TRUE: invoke the stage again — or take the batch
    // back with the one command it names.
    let message = halted["message"].as_str().expect("a message");
    assert!(
        message.contains("Invoke the stage again: where a batch is pending, that invocation runs the full gate on it once more, commits and pushes it, and does nothing else")
            && message.contains("`dev/stabilize-record discard --run rc24`")
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
            "record:pending",
            "git:record:pending",
            "git:push:pending",
            "git:state:test-1"
        ]
    );
    let head = sim.rig.rev("HEAD");
    assert_eq!(sim.rig.rev("HEAD~1"), opened, "one commit");
    assert_eq!(again.result["record"], head.as_str());
    let (subject, paths) = head_commit(&sim);
    assert_eq!(subject, "docs(record): rc24 r1 — the test stage's record");
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
        halted["halted"]["gate"]["new"],
        json!([format!("test {FLAKY}")]),
        "what the candidate's gate showed red is not named: {halted}"
    );
    assert_eq!(
        halted["halted"]["gate"]["known"],
        json!(["step test", format!("test {KNOWN}")])
    );
    assert_eq!(sim.rig.rev("HEAD"), candidate);

    // THE OTHER WAY OUT: the batch is taken back — every table as it was, the reports and
    // the scope kept — and the stage is run again, as the next attempt.
    sim.record(&["discard", "--run", RUN], "");
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
        halted["halted"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.starts_with("no-batch: ")),
        "M2: {halted}"
    );
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert_eq!(
        ran.agents()[ran.agents().len() - 2..],
        ["record:test:r1", "git:record:test:r1"],
        "the commit step ran, and is what refused"
    );
    // Nothing is pending, so the stage invoked again works the next attempt.
    let again = sim.invoke(sim.args("test", json!({})), small_stage(json!({})));
    assert_eq!(again.result["status"], "triaged", "{}", again.result);
    assert_eq!(
        again.result["counts"]["reporters"], 4,
        "the attempt's marker, the preflight, the scope step and the one reviewer"
    );
}

/// The hash comparison of ruling 11, driven: a driver that asserts another binary than the
/// one the preflight built drove something else, and nothing it found is evidence about
/// the candidate. (The whole-stage test could not see this comparison removed: every
/// driver there measures the file it was handed.)
#[test]
fn a_driver_that_drove_another_binary_halts_the_stage_before_anything_is_recorded() {
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
    let halted = &ran.result;
    assert_eq!(halted["status"], "halted", "{halted}");
    assert_eq!(halted["halted"]["phase"], "binary", "{halted}");
    assert_eq!(
        halted["halted"]["mismatched"],
        json!([{"reporter": "drive-a-drive", "asserted": "0".repeat(64)}])
    );
    assert!(
        !ran.agents()
            .iter()
            .any(|label| label.starts_with("record:") || label.starts_with("triage:")),
        "nothing is graded and nothing recorded: {:?}",
        ran.agents()
    );
    assert_eq!(sim.rig.rev("HEAD"), opened);
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
            "status --porcelain --untracked-files=all",
            &format!("add -- {facts}"),
            "commit -q -m docs(record): a record",
            "rev-parse HEAD",
            "status --porcelain --untracked-files=all",
        ]),
        "the commands of the act, in order: nothing is pushed, and no path is added by a directory"
    );
    record(&green, "2", LOOP).refused("no-batch");

    // A write of the record that was killed between its files is no state a step reads.
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
    assert!(
        killed.refused("record")["halt"]["recommendation"]
            .as_str()
            .is_some_and(|what| what.contains("recover --run rc24")),
        "the refusal names what finishes the write: {}",
        killed.raw
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
            String::new()
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
    assert_eq!(
        ran.agents(),
        [
            "git:state",
            "git:state:test-1",
            "git:begin",
            "preflight",
            "scope",
            "git:state:test-2"
        ]
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
    let ledger = result["state"]["ledger"].as_array().expect("the ledger");
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
    assert_eq!(result["blockers"], json!([]));
    assert_eq!(
        result["forbids_close"],
        json!([{"clause": "gate-green", "status": "red"},
               {"finding": "red-gate", "route": "triage"}])
    );
    assert_eq!(
        result["state"]["untriaged"],
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
        finishing_lap(&["red-gate"], "git:state:test-2")
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

    // The re-run: the check, on the loop branch's tip, and nothing else — no scope step, no
    // item beside it. One record commit, pushed.
    let again = sim.invoke(
        sim.args("test", json!({"clause": "gate-green"})),
        json!({"checks": {"gate": "green"}}),
    );
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
        [
            "git:state",
            "git:state:test-1",
            "git:begin",
            "preflight",
            "git:state:test-2",
            "git:check-reports",
            "record:test:r1",
            "git:record:test:r1",
            "git:push",
            "git:state:test-3"
        ],
        "a re-run launches no scope step and no item it was not asked for"
    );
    assert_eq!(sim.rig.rev("HEAD~1"), asked, "one record commit");
    let (subject, paths) = head_commit(&sim);
    assert_eq!(
        subject,
        format!("docs(record): {RUN} r1 — a re-run of gate-green")
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

    // It began no round, wrote no fact of round 1, and is on record as attempt 2 of it — on
    // the commit it ran on. The clause is green, and the run closes.
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
        [
            json!([1, "void", ran.result["candidate"]["sha"]]),
            json!([2, "green", asked]),
        ],
        "{state}"
    );
    assert_eq!(result["next"], "close", "{result}");
    assert_eq!(result["forbids_close"], json!([]));
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
        [
            "git:state",
            "git:state:test-1",
            "git:begin",
            "preflight",
            "git:state:test-2",
            "area-a:review",
            "cross:review",
            "drive-a:drive",
            "git:check-reports",
            "record:test:r1",
            "git:record:test:r1",
            "git:push",
            "git:state:test-3"
        ],
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
    assert_eq!(last.result["forbids_close"], json!([]));
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
/// binary a reviewer was handed (`M7`), the posture the preflight proved and the commit a
/// check ran on (`L1`).
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

    // M7 — a reviewer is handed the candidate's binary and returns the hash it asserted.
    let sim = Sim::opened("reviewer-binary", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "wrongBinary": ["area-a:review"]}),
    );
    assert_eq!(
        ran.result["halted"]["phase"], "binary",
        "M7: {}",
        ran.result
    );
    assert_eq!(
        ran.result["halted"]["mismatched"],
        json!([{"reporter": "area-a-review", "asserted": "0".repeat(64)}]),
        "M7: the reviewer's hash is compared like every driver's"
    );

    // L1 — the posture the preflight proved is read: a bare `jigc` that resolves elsewhere.
    let sim = Sim::opened("path-check", &one_area());
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "pathCheck": "/usr/local/bin/jigc"}),
    );
    assert_eq!(
        ran.result["halted"]["phase"], "preflight",
        "L1: {}",
        ran.result
    );
    assert!(
        ran.result["halted"]["reason"].as_str().is_some_and(
            |reason| reason.contains("`command -v jigc` printed \"/usr/local/bin/jigc\"")
        ),
        "L1: {}",
        ran.result
    );
    // L1 — and so is the commit a check ran on: green on another commit is void here.
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
    assert!(
        dirty.result["halted"]["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("dirty: ")),
        "{}",
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
    assert!(
        elsewhere.result["halted"]["halt"]["root_cause"]
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
