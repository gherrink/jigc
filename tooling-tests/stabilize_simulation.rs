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
//! finding by its id; the task that repairs it turns the assertion. They are `M3` and `M6`
//! of the harness review and `F3` of the state machine's. **Turned by the repair's task 3,
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
        for file in [SCANNER, GITLEAKS_CONFIG] {
            fs::copy(repo_root().join(file), rig.root.join(file))
                .unwrap_or_else(|e| panic!("copy `{file}` into the rig: {e}"));
        }
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
        18,
        "the reporters this stage launched: {launched:?}"
    );
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
    assert_eq!(result["counts"]["reporters"], 18, "{result}");
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
            "fix",
            "inside"
        ])
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
        json!([true, true, true, 18])
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
        json!([{"key": OUTSIDE, "why": "outside"}])
    );
    assert_eq!(result["human_list"], state["human_list"]);
    assert_eq!(result["blockers"], json!([INSIDE]));
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
        json!({"items": 7, "items_run": 6, "reporters": 18, "findings_in": 4, "entries": 4,
               "by_grade": {"confirmed": 2, "refuted": 1, "out-of-scope": 1},
               "blockers": 1, "for_the_human": 1, "voided": []})
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

    // PINNED — M3 (harness-agents-gate.md): a fork has no committed form. The contested
    // finding comes back in `forks`, with the advocate's case and the independent drive —
    // and in the state it is a blocker like any other: routed to a fixer, in no list the
    // human is asked about, and nothing the state computes waits for the human's answer.
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
        json!([INSIDE]),
        "M3: the fork is a fixer's, by the state"
    );
    assert!(
        !state.to_string().contains("contested"),
        "M3: no word of the fork is in the state the next invocation starts from"
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
    assert_eq!(sim.rig.rev("HEAD~1"), head, "one commit");
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

#[test]
fn an_unverified_finding_goes_back_as_triage_and_the_invocation_that_finishes_it_halts() {
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
            format!("verify:{key}"): {
                "status": "halted", "key": key,
                "halt": {"root_cause": "the repro can be neither driven nor reconstructed",
                         "evidence": "scripted", "tree_state": "clean", "recommendation": "none"},
            },
        },
    });
    let ran = sim.invoke(sim.args("test", json!({})), script);
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");
    assert_eq!(result["counts"]["by_grade"], json!({"breaks": 1}));

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

    // PINNED — M6 (harness-agents-gate.md), which the repair plan's §7 reads as a halt:
    // the invocation that finishes the triage asks the preflight for the commit the round
    // tested, and the record commit has moved HEAD off it. The preflight's definition
    // asserts `git rev-parse HEAD` is the candidate, and the stand-in holds that assert —
    // so the finishing invocation halts at the preflight, before anything is graded.
    let finishing = sim.invoke(sim.args("test", json!({})), json!({}));
    let halted = &finishing.result;
    assert_eq!(halted["status"], "halted", "{halted}");
    assert_eq!(halted["halted"]["phase"], "preflight", "M6: {halted}");
    let reason = halted["halted"]["reason"].as_str().expect("a reason");
    assert!(
        reason.contains(&format!(
            "`git rev-parse HEAD` is {recorded}, and the candidate is {candidate}"
        )),
        "M6: {reason}"
    );
    assert_eq!(
        finishing.trace,
        lines(&[
            TWO_READS[0],
            TWO_READS[1],
            TWO_READS[2],
            "phase Triage",
            "agent preflight · stabilize-preflight · opus · Triage",
        ]),
        "no instrument runs, and no triage either"
    );

    // PINNED — F3 (state-machine.md): nothing counts the laps. The halted lap left a
    // report, so the state hands the same stage the next attempt, and `next` is `triage`
    // as before — with no bound and no way to the human.
    let state = sim.state();
    assert_eq!(state["next"], "triage", "F3: {state}");
    assert_eq!(
        state["position"]["test"],
        json!({"round": 1, "attempt": 3, "triage": true}),
        "F3: {state}"
    );
    assert_eq!(
        sim.rig.rev("HEAD"),
        recorded,
        "the halted lap committed nothing"
    );
}

/// A scenario's red gate: the fast tier failed, on these tests.
fn red(tests: &[&str]) -> Value {
    json!({"steps": ["tier1"], "tests": tests})
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
        json!(["step tier1", format!("test {FLAKY}")])
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
        "| step | `tier1` |".to_owned(),
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
        json!(["step tier1", format!("test {KNOWN}")])
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
    assert_eq!(again.result["counts"]["reporters"], 3);
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
        "tests   passed=8 failed=1  (over 2 test binaries)\n\nGATE: FAIL (step: tier1)\n\nfailing tests:\n  jigc::g_a s::one\n",
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
        json!(["step tier1", "test jigc::g_a s::one"]),
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
        lines(&["preflight", "scope"])
    );
    assert_eq!(sim.reports(1), lines(&["preflight.a1.md", "scope.a1.md"]));
    assert_eq!(
        ran.agents(),
        [
            "git:state",
            "git:state:test-1",
            "preflight",
            "scope",
            "git:state:test-2"
        ]
    );
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
        5 + 5,
        "five reports, the scope, the gate, the items' results, the round's facts and the clause table: {paths:?}"
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
            &format!("{RUN_DIR}/r1/reports/test/preflight.a2.md"),
            &format!("{RUN_DIR}/r1/results.md"),
        ]),
        "the re-run's one report, its result, and the clause table the result rendered"
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
