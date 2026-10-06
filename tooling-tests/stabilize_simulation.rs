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
//! - *The full gate a record step runs.* The gate runs this suite; the record step's
//!   stand-in returns the two lines the harness reads for a green gate, or halts where a
//!   scenario scripts a red one.
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
//! finding by its id; the task that repairs it turns the assertion. They are `M2`, `M3`,
//! `M4` and `M6` of the harness review and `F3` of the state machine's, the lead *a red
//! check without a ledger row*, and the first of the build record's *Found while the
//! workflow's doc was written*.
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
    /// opening record, the run's facts, a row per clause, the test set, the declared
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
                    "previous-commit": previous, "scope": "delta"})
            .to_string(),
        );
        for clause in &opening.clauses {
            self.record(
                &[
                    "clause-set",
                    "--run",
                    RUN,
                    "--clause",
                    clause,
                    "--instrument",
                    "not yet run",
                    "--scope",
                    "not yet run",
                    "--status",
                    "void",
                ],
                "",
            );
        }
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

#[test]
fn a_record_step_under_a_red_gate_leaves_a_tree_no_stage_starts_from() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("red-gate", &one_area());
    let opened = sim.rig.rev("HEAD");

    // PINNED — M4 (harness-agents-gate.md): the record step writes its tables, then gates,
    // then commits. Under a red gate the stage halts with the tables written and nothing
    // committed. (What an agent does to the tree after a red gate is the canary's to show;
    // the stand-in stops where it stands.)
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "record": {"gate": "red"}}),
    );
    let halted = &ran.result;
    assert_eq!(halted["status"], "halted", "{halted}");
    assert_eq!(halted["halted"]["phase"], "record", "{halted}");
    assert_eq!(halted["halted"]["reason"], "the full gate is red");
    assert_eq!(sim.rig.rev("HEAD"), opened, "nothing was committed");
    assert_eq!(
        sim.rig.remote(LOOP),
        Some(opened.clone()),
        "and nothing pushed"
    );
    let status = sim.rig.status();
    assert!(
        status.contains(&format!(" M {RUN_DIR}/clauses.md"))
            && status.contains(&format!("?? {RUN_DIR}/r1/round.md")),
        "M4: the tables are written and uncommitted:\n{status}"
    );

    // PINNED — M4, its other half: the halt says to invoke the stage again with the same
    // args (the sentence every halt ends with, which `L2` finds wrong elsewhere too) …
    assert!(
        halted["message"]
            .as_str()
            .is_some_and(|m| m.contains("invoke the stage again with the same args")),
        "M4: {halted}"
    );
    // … and the stage that is invoked again halts on that tree, before any read. A
    // candidate whose gate is red cannot have its `test` stage recorded at all.
    let again = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(again.result["status"], "halted", "{}", again.result);
    assert_eq!(
        again.result["halted"]["phase"], "git",
        "M4: {}",
        again.result
    );
    assert!(
        again.result["halted"]["halt"]["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("dirty: ")),
        "M4: {}",
        again.result
    );
    assert_eq!(again.trace, lines(&TWO_READS[..2]));
}

#[test]
fn a_record_step_that_returns_no_check_is_taken_as_recorded() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("no-checks", &one_area());
    // PINNED — M2 (harness-agents-gate.md): the harness composes two checks for this
    // record step and reads the return of neither. A record step that hands back no check
    // line at all is taken as a record whose checks held.
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"scope": one_door(), "agents": {"area-a:review": nothing()},
               "record": {"checks": "omitted"}}),
    );
    assert_eq!(ran.result["status"], "triaged", "M2: {}", ran.result);
    assert_eq!(ran.result["record"], sim.rig.rev("HEAD").as_str());
}

#[test]
fn a_stage_stopped_after_its_scope_step_halts_the_invocation_that_follows() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened("stopped", &one_area());
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

    // PINNED — the build record's *Found while the workflow's doc was written*, item 1
    // (completions/artifacts/M55/stabilization-build/README.md; named by neither review and
    // by no task of the plan): the scope step wrote the round's scope, which is no report,
    // and the step that opens every stage admits untracked files only where they are
    // reports. So the invocation that follows a stage stopped after its scope step — by
    // `stopAfter`, or by any halt — halts on the scope file.
    let again = sim.invoke(sim.args("test", json!({})), json!({}));
    assert_eq!(again.result["status"], "halted", "{}", again.result);
    assert_eq!(again.result["halted"]["phase"], "git", "{}", again.result);
    let halt = &again.result["halted"]["halt"];
    assert!(
        halt["root_cause"]
            .as_str()
            .is_some_and(|cause| cause.starts_with("dirty: "))
            && halt.to_string().contains(&format!("{RUN_DIR}/r1/scope.md")),
        "the scope file is what the tree is refused for: {halt}"
    );
    assert_eq!(again.trace, lines(&TWO_READS[..2]));
}

#[test]
fn a_red_check_reaches_nobody_and_a_go_is_recorded_without_starting_anything() {
    if !can_run() {
        return;
    }
    let sim = Sim::opened(
        "red-check",
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
    let ran = sim.invoke(
        sim.args("test", json!({})),
        json!({"checks": {"gate": "red"}, "scope": one_door(),
               "agents": {"area-a:review": nothing()}}),
    );
    let result = &ran.result;
    assert_eq!(result["status"], "triaged", "{result}");

    // PINNED — the lead *a red check without a ledger row* (repair-plan.md §2;
    // stabilization-workflow.md → *A red deterministic check*): the check's clause row is
    // red, and that is all. No finding, no row, no blocker — nothing hands a red check to
    // triage or to a fixer. What the state asks for is the check run again.
    assert_eq!(result["state"]["ledger"], json!([]), "{result}");
    assert_eq!(result["blockers"], json!([]));
    assert_eq!(
        result["forbids_close"],
        json!([{"clause": "gate-green", "status": "red"}])
    );
    assert_eq!(result["state"]["retest"], json!(["gate-green"]));

    // THE RUN STOPS AFTER EVERY ROUND, and `stop` is a value the script only relays.
    assert_eq!(result["next"], "stop", "{result}");
    assert_eq!(result["returned_to_orchestrator"], true);
    assert_eq!(
        result["state"]["stop"],
        json!({"why": "every-round", "round": 1, "then": "retest"})
    );
    let tested = sim.rig.rev("HEAD");

    // THE HUMAN'S GO is its own invocation: recorded on the loop branch, pushed, and
    // nothing started — `next` then names the step the stop was holding.
    let go = sim.invoke(
        sim.args("test", json!({"rulings": [{"go": true}]})),
        json!({}),
    );
    assert_eq!(go.result["status"], "ruled", "{}", go.result);
    assert_eq!(go.result["next"], "retest");
    assert_eq!(
        go.agents(),
        [
            "git:state",
            "git:state:test-1",
            "record:rulings:r1",
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
