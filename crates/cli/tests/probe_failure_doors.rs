//! M54 Increment 2 / T7 — **every door that spawns the `doc-code` probe says what went
//! wrong when the probe does not run**
//! ([module-layout.md](../../../implementation/module-layout.md) → Probe boundary, the
//! acceptance per path; [assistant-adapter.md](../../../design/assistant-adapter.md) → the
//! hook's cases (iv) and (v)).
//!
//! `jigc` runs the probe by spawning itself (T4), and a probe that fails reports *why* (T2):
//! a spawn that cannot start reads *could not start: `<io error>`*, and a child that refuses —
//! a skewed build — has its own stderr carried on the message. Both are one
//! `pack-probe-integrity.probe-failure`, check `crash`. T2 proved that at one door. This suite
//! proves it at **all five**, through the real binary:
//!
//! - `jigc validate` (the store sweep, `cli.rs`),
//! - `jigc task validate` and `jigc task finalize` (the task gate, `task.rs`),
//! - `jigc milestone finalize` (the merged-state join, `milestone.rs`),
//! - orientation — a bare `jigc start` over an anchored active task (`orient.rs`),
//! - a real `git commit` through the pre-commit hook `jigc setup` installed.
//!
//! Every door runs over the [`State::Vendored`] corpus with its anchored symbol
//! ([`VENDORED_CODE_SYMBOL`]) **renamed away**, in three arms:
//!
//! - **self-exec** — no override. The real probe runs, so the door reports the drifted anchor
//!   (`doc-code.symbol-exists`) and no probe failure. This is the control that makes the other
//!   two arms mean something: the probe does run at this door.
//! - **could not start** — `JIGC_DOC_CODE_PROBE` names an existing, non-executable regular
//!   file. It passes the override's presence pre-flight and fails at `spawn` with `EACCES`.
//! - **skewed build** — the suite first drives the **real** child with a skewed build id
//!   (`jigc __probe doc-code --build 0.0.0-skew`) and captures its stderr and exit code. The
//!   override is then a `rustc`-built stub that replays exactly those bytes and that code. It
//!   never execs `jigc`, so it is not a shell-wrapper shim onto the binary. *Why a replay:* one
//!   build has one package version, so a genuinely mixed-version pair cannot be built in-tree.
//!   T4 proves the child's reason; this proves every door carries it.
//!
//! In both failure arms the probe never ran, so the door must report no
//! `doc-code.symbol-exists` either.
//!
//! **At the hook door the finding is read from the invocation log.** The hook prints nothing
//! for a `pack-probe-integrity` finding, by design (case (v)), so the observable is the
//! `finding_codes` of the one record the hook's `jigc validate` writes. Its message comes from
//! the same invoker the other four doors assert on. The control there is case (iv): the hook
//! warns on the drifted anchor, which a relocated `jigc` with no sibling probe could not do
//! before self-exec.
//!
//! **The task-scoped doors cover the blast radius.** Each drifts a file a *committed* anchor
//! cites, so the gate probes twice, once over the staged index and once over the base. A probe
//! that cannot run fails both times, and the two failures share one key; the report carries
//! it once (`engine::validate`, the same M54 Inc 2 T7 fix).

use crate::support::run_then_parse::stdout_json;
use crate::support::trial_corpus::{State, TrialCorpus, VENDORED_CODE_FILE, VENDORED_CODE_SYMBOL};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// The corpus's tracked code file with its anchored symbol renamed away.
const DRIFTED: &str = "export function renamed(s: string): string {\n  return s;\n}\n";

/// The build id the skew arm asks the real child for.
const SKEWED_BUILD: &str = "0.0.0-skew";

/// The code a door reports when the probe ran and found the drifted anchor.
const SYMBOL_EXISTS: &str = "doc-code.symbol-exists";

/// The code a door reports when the probe did not run.
const PROBE_FAILURE: &str = "pack-probe-integrity.probe-failure";

/// The hook's warning on a blocking `doc-code` finding (case (iv)).
const DRIFT_WARNING: &str = "doc<->code drift detected";

/// What one arm's door must report.
enum Expect {
    /// The real self-exec probe ran: the drifted anchor is reported, and nothing failed.
    ProbeRan,
    /// The probe did not run: one `probe-failure`/`crash` whose message carries every needle.
    ProbeFailure(Vec<String>),
}

/// One arm: the probe the door's spawn resolves, and what the door must report.
struct Arm<'a> {
    label: &'static str,
    /// `None` removes `JIGC_DOC_CODE_PROBE`; `Some` points it at a failing probe.
    probe: Option<&'a Path>,
    expect: Expect,
}

/// The two failing probes the failure arms point the override at.
struct FailingProbes {
    /// An existing regular file with no exec bit.
    not_executable: PathBuf,
    /// A stub that replays the real skewed child's stderr and exit code.
    replay: PathBuf,
    /// The real child's stderr, trimmed as a finding message carries it.
    skew_reason: String,
    /// The real child's exit code.
    skew_code: i32,
}

impl FailingProbes {
    /// Build both under `<corpus home>/probes`, so the corpus's own drop removes them.
    fn build(corpus: &TrialCorpus) -> Self {
        let dir = corpus.home().join("probes");
        fs::create_dir_all(&dir).expect("create the stub dir");

        let not_executable = dir.join("doc-code-not-executable");
        fs::write(&not_executable, "not a program\n").expect("write the non-executable probe");
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&not_executable, fs::Permissions::from_mode(0o644))
                .expect("clear the exec bits");
        }

        let (stderr, skew_code) = real_skewed_child();
        let replay = build_replay_stub(&dir, &stderr, skew_code);
        FailingProbes {
            not_executable,
            replay,
            skew_reason: String::from_utf8_lossy(&stderr).trim().to_string(),
            skew_code,
        }
    }

    /// The three arms, in the order every door runs them.
    fn arms(&self) -> Vec<Arm<'_>> {
        vec![
            Arm {
                label: "self-exec (no override)",
                probe: None,
                expect: Expect::ProbeRan,
            },
            Arm {
                label: "could not start (non-executable override)",
                probe: Some(&self.not_executable),
                expect: Expect::ProbeFailure(vec![
                    "could not start".to_string(),
                    "os error".to_string(),
                ]),
            },
            Arm {
                label: "skewed build (the real child's refusal, replayed)",
                probe: Some(&self.replay),
                expect: Expect::ProbeFailure(vec![
                    format!("exited non-zero (exit-code {})", self.skew_code),
                    self.skew_reason.clone(),
                ]),
            },
        ]
    }
}

/// Drive the **real** child with a skewed build id — the argv spelled by the builder `jigc`
/// itself uses, with only the build id replaced — and return its stderr and exit code.
fn real_skewed_child() -> (Vec<u8>, i32) {
    let mut argv = cli::invoke::doc_code_probe_args();
    *argv
        .last_mut()
        .expect("the probe argv ends with the build id") = SKEWED_BUILD.to_string();
    let out = Command::new(env!("CARGO_BIN_EXE_jigc"))
        .args(&argv)
        .stdin(Stdio::null())
        .output()
        .expect("spawn the real probe child");
    let reason = String::from_utf8_lossy(&out.stderr);
    let code = out
        .status
        .code()
        .expect("the skewed child exits with a code");
    assert!(
        code != 0 && code != 2,
        "the skewed child refuses with a non-zero, non-usage exit; got {code}, stderr:\n{reason}",
    );
    assert!(
        out.stdout.is_empty(),
        "the skewed child writes nothing on stdout; got:\n{}",
        String::from_utf8_lossy(&out.stdout),
    );
    assert!(
        reason.contains(SKEWED_BUILD) && reason.contains(env!("CARGO_PKG_VERSION")),
        "the child's reason names both builds; stderr:\n{reason}",
    );
    (out.stderr, code)
}

/// Compile a stub that writes `stderr` verbatim and exits `code`, the real child's refusal
/// byte for byte. It never execs `jigc`.
fn build_replay_stub(dir: &Path, stderr: &[u8], code: i32) -> PathBuf {
    let src = dir.join("replay.rs");
    fs::write(
        &src,
        format!(
            "use std::io::Write;\n\
             const STDERR: &[u8] = &{stderr:?};\n\
             fn main() {{\n    \
                 let _ = std::io::stderr().write_all(STDERR);\n    \
                 std::process::exit({code});\n\
             }}\n"
        ),
    )
    .expect("write the replay stub source");
    let bin = dir.join("doc-code-replay");
    let out = Command::new("rustc")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .arg("--edition")
        .arg("2021")
        .output()
        .expect("invoke rustc");
    assert!(
        out.status.success(),
        "rustc failed to build the replay stub:\n{}",
        String::from_utf8_lossy(&out.stderr),
    );
    bin
}

/// Point one child at `probe`, or remove the override so the production self-exec path
/// runs (an ambient value must not mask it).
fn with_probe(command: &mut Command, probe: Option<&Path>) {
    match probe {
        Some(path) => command.env("JIGC_DOC_CODE_PROBE", path),
        None => command.env_remove("JIGC_DOC_CODE_PROBE"),
    };
}

/// Run `jigc <args>` against the corpus, over the embedded packs, with `probe`.
fn jigc(corpus: &TrialCorpus, args: &[&str], probe: Option<&Path>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_jigc"));
    command
        .args(args)
        .current_dir(corpus.repo())
        .env("HOME", corpus.home())
        .env_remove("JIGC_PACK_DIR")
        .stdin(Stdio::null());
    with_probe(&mut command, probe);
    command.output().expect("spawn jigc")
}

/// The `findings` array of a JSON envelope.
fn findings_of<'v>(envelope: &'v Value, door: &str) -> &'v [Value] {
    envelope["findings"]
        .as_array()
        .unwrap_or_else(|| panic!("{door}: the envelope carries a findings array:\n{envelope}"))
}

/// The drifted anchor, as the probe's finding names it.
fn drifted_anchor() -> String {
    format!("{VENDORED_CODE_FILE}#{VENDORED_CODE_SYMBOL}")
}

/// Assert one door's findings against its arm.
fn assert_findings(door: &str, arm: &Arm<'_>, findings: &[Value]) {
    let anchor = drifted_anchor();
    let integrity: Vec<&Value> = findings
        .iter()
        .filter(|f| f["probe"] == "pack-probe-integrity")
        .collect();
    let ran = findings.iter().any(|f| {
        f["code"] == SYMBOL_EXISTS && f["message"].as_str().is_some_and(|m| m.contains(&anchor))
    });
    let label = arm.label;
    match &arm.expect {
        Expect::ProbeRan => {
            assert!(
                integrity.is_empty(),
                "{door} · {label}: the self-exec probe runs, so no probe-integrity finding; \
                 got {integrity:?}",
            );
            assert!(
                ran,
                "{door} · {label}: the probe ran and reports the drifted `{anchor}`; got \
                 {findings:?}",
            );
        }
        Expect::ProbeFailure(needles) => {
            assert_eq!(
                integrity.len(),
                1,
                "{door} · {label}: exactly one probe-integrity finding; got {integrity:?}",
            );
            let failure = integrity[0];
            assert_eq!(
                failure["code"], PROBE_FAILURE,
                "{door} · {label}: {failure}"
            );
            assert_eq!(failure["check"], "crash", "{door} · {label}: {failure}");
            assert_eq!(
                failure["severity"], "blocking",
                "{door} · {label}: {failure}"
            );
            let message = failure["message"].as_str().unwrap_or_default();
            for needle in needles {
                assert!(
                    message.contains(needle.as_str()),
                    "{door} · {label}: the message carries `{needle}`; got: {message}",
                );
            }
            assert!(
                !ran,
                "{door} · {label}: the probe never ran, so no anchor finding; got {findings:?}",
            );
        }
    }
}

/// A Vendored corpus with the anchored symbol renamed away in the working tree.
fn drifted_corpus() -> TrialCorpus {
    let corpus = TrialCorpus::build(State::Vendored);
    fs::write(corpus.repo().join(VENDORED_CODE_FILE), DRIFTED).expect("drift the anchored symbol");
    corpus
}

/// [`drifted_corpus`] plus a live `single-task` task whose commit doc is filled and whose
/// **staged** change is the drift, so its gate re-resolves the committed anchor the drift
/// dangles (the blast radius). Returns the task id the binary printed.
fn drifted_task(corpus: &TrialCorpus) -> String {
    let task = corpus.start_workflow("single-task", "rename the pad");
    corpus.set_field(&format!("commit:{task}#type"), &task, "refactor");
    corpus.set_slot(&format!("commit:{task}#summary"), &task, "rename the pad");
    corpus.git(&["add", VENDORED_CODE_FILE]);
    task
}

/// HEAD's commit count.
fn head_count(corpus: &TrialCorpus) -> u32 {
    corpus
        .git(&["rev-list", "--count", "HEAD"])
        .parse()
        .expect("a commit count")
}

/// Door 1 — `jigc validate`, the store sweep.
#[test]
fn jigc_validate_reports_each_probe_failure() {
    let corpus = drifted_corpus();
    let probes = FailingProbes::build(&corpus);
    for arm in probes.arms() {
        let out = jigc(&corpus, &["validate", "--format", "json"], arm.probe);
        let envelope: Value = stdout_json(&out, &[0, 1], "`jigc validate --format json`");
        assert_findings(
            "jigc validate",
            &arm,
            findings_of(&envelope, "jigc validate"),
        );
    }
}

/// Door 2 — `jigc task validate` and `jigc task finalize`, the task gate. Every arm blocks
/// (the drifted anchor, or the probe that did not run), so the task is unchanged between arms
/// and nothing commits.
#[test]
fn task_validate_and_finalize_report_each_probe_failure() {
    let corpus = drifted_corpus();
    let task = drifted_task(&corpus);
    let probes = FailingProbes::build(&corpus);
    for arm in probes.arms() {
        for verb in ["validate", "finalize"] {
            let door = format!("jigc task {verb}");
            let before = head_count(&corpus);
            let out = jigc(
                &corpus,
                &["--format", "json", "task", verb, &task],
                arm.probe,
            );
            let envelope: Value = stdout_json(&out, &[3], &format!("`{door}`"));
            assert_findings(&door, &arm, findings_of(&envelope, &door));
            assert_eq!(
                head_count(&corpus),
                before,
                "{door} · {}: a blocked gate commits nothing",
                arm.label,
            );
        }
    }
}

/// Door 3 — orientation: a bare `jigc start` sweeps each active task, and the anchored task's
/// row carries what its gate would report.
#[test]
fn orientation_reports_each_probe_failure() {
    let corpus = drifted_corpus();
    let task = drifted_task(&corpus);
    let probes = FailingProbes::build(&corpus);
    for arm in probes.arms() {
        let out = jigc(&corpus, &["--format", "json", "start"], arm.probe);
        let envelope: Value = stdout_json(&out, &[0], "`jigc --format json start`");
        let row = envelope["tasks"]
            .as_array()
            .and_then(|rows| rows.iter().find(|r| r["id"] == task.as_str()))
            .unwrap_or_else(|| panic!("orientation lists the task `{task}`:\n{envelope}"));
        assert!(
            row["findings_unavailable"].is_null(),
            "orientation · {}: the sweep ran — a probe failure is a finding, not an unavailable \
             sweep; row:\n{row}",
            arm.label,
        );
        assert_findings("orientation", &arm, findings_of(row, "orientation"));
    }
}

/// The last line of `stdout` that starts with `prefix`, with the prefix removed.
fn printed_after<'s>(stdout: &'s str, prefix: &str) -> &'s str {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("the output prints `{prefix}…`; got:\n{stdout}"))
}

/// Door 4 — `jigc milestone finalize`, the merged-state join. The one sub-task's worktree
/// renames the symbol a committed anchor cites, so the join's gate re-resolves it.
#[test]
fn milestone_join_reports_each_probe_failure() {
    let corpus = TrialCorpus::build(State::Vendored);
    let created = corpus.jigc_ok(&["milestone", "create", "Pad rework"]);
    let milestone = printed_after(&created, "minted milestone:")
        .split_whitespace()
        .next()
        .expect("the minted milestone id")
        .to_string();
    let added = corpus.jigc_ok(&["milestone", "add-task", &milestone, "Rename the pad"]);
    let sub = printed_after(&added, "added task:")
        .split_whitespace()
        .next()
        .expect("the added sub-task id")
        .to_string();
    corpus.jigc_ok(&["milestone", "provision", &milestone]);

    // The sub-task's worktree, as git lists it — never a reconstructed path.
    let worktrees = corpus.git(&["worktree", "list", "--porcelain"]);
    let worktree = worktrees
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .find(|path| path.file_name().is_some_and(|name| name == sub.as_str()))
        .unwrap_or_else(|| panic!("provision checked out a worktree for `{sub}`:\n{worktrees}"));
    fs::write(worktree.join(VENDORED_CODE_FILE), DRIFTED).expect("drift in the worktree");
    let staged = Command::new("git")
        .args(["add", VENDORED_CODE_FILE])
        .current_dir(&worktree)
        .env("HOME", corpus.home())
        .output()
        .expect("spawn git add");
    assert!(
        staged.status.success(),
        "git add in the worktree: {}",
        String::from_utf8_lossy(&staged.stderr),
    );

    let probes = FailingProbes::build(&corpus);
    for arm in probes.arms() {
        let before = head_count(&corpus);
        let out = jigc(
            &corpus,
            &["--format", "json", "milestone", "finalize", &milestone],
            arm.probe,
        );
        let envelope: Value = stdout_json(&out, &[3], "`jigc milestone finalize`");
        assert_findings(
            "milestone finalize",
            &arm,
            findings_of(&envelope, "milestone"),
        );
        assert_eq!(
            head_count(&corpus),
            before,
            "milestone finalize · {}: a blocked join commits nothing",
            arm.label,
        );
    }
}

/// The parsed records of the corpus's invocation log.
fn log_records(corpus: &TrialCorpus) -> Vec<Value> {
    let path = corpus
        .repo()
        .join(".jigc")
        .join("logs")
        .join("invocations.jsonl");
    fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("each log line is JSON"))
        .collect()
}

/// Door 5 — a real `git commit` through the pre-commit hook `jigc setup` installed. The hook's
/// `jigc validate` inherits the commit's environment, so the override reaches it exactly as it
/// would reach an agent's.
#[test]
fn the_precommit_hook_records_each_probe_failure() {
    let corpus = drifted_corpus();
    corpus.jigc_ok(&["config", "set", "invocation-log", "true"]);
    let probes = FailingProbes::build(&corpus);
    for (n, arm) in probes.arms().iter().enumerate() {
        let label = arm.label;
        let note = format!("notes/arm-{n}.txt");
        let path = corpus.repo().join(&note);
        fs::create_dir_all(path.parent().expect("a parent")).expect("create notes/");
        fs::write(&path, format!("{label}\n")).expect("write the arm's note");
        corpus.git(&["add", &note]);

        let records_before = log_records(&corpus).len();
        let before = head_count(&corpus);
        let mut command = Command::new("git");
        command
            .args(["commit", "-q", "-m", &format!("arm {n}")])
            .current_dir(corpus.repo())
            .env("HOME", corpus.home())
            .env_remove("JIGC_PACK_DIR");
        with_probe(&mut command, arm.probe);
        let out = command.output().expect("spawn git commit");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            out.status.success(),
            "hook · {label}: the hook is warn-only, the commit lands; stderr:\n{stderr}",
        );
        assert_eq!(
            head_count(&corpus),
            before + 1,
            "hook · {label}: one commit"
        );

        let records = log_records(&corpus);
        assert_eq!(
            records.len(),
            records_before + 1,
            "hook · {label}: the hook's `jigc validate` writes exactly one record; new: {:?}",
            &records[records_before.min(records.len())..],
        );
        let record = &records[records_before];
        assert_eq!(
            record["argv"],
            serde_json::json!(["validate", "--format", "json"]),
            "hook · {label}: the new record is the hook's sweep",
        );
        let codes: Vec<&str> = record["finding_codes"]
            .as_array()
            .unwrap_or_else(|| panic!("hook · {label}: a finding_codes array: {record}"))
            .iter()
            .filter_map(Value::as_str)
            .collect();
        match &arm.expect {
            Expect::ProbeRan => {
                assert!(
                    codes.contains(&SYMBOL_EXISTS)
                        && !codes.iter().any(|c| c.starts_with("pack-probe-integrity")),
                    "hook · {label}: the self-exec probe ran and found the drift; codes \
                     {codes:?}",
                );
                assert!(
                    stderr.contains(DRIFT_WARNING),
                    "hook · {label}: case (iv) — the hook warns on the drifted anchor; \
                     stderr:\n{stderr}",
                );
            }
            Expect::ProbeFailure(_) => {
                assert_eq!(
                    codes.iter().filter(|c| **c == PROBE_FAILURE).count(),
                    1,
                    "hook · {label}: one probe-failure; codes {codes:?}",
                );
                assert!(
                    !codes.contains(&SYMBOL_EXISTS),
                    "hook · {label}: the probe never ran; codes {codes:?}",
                );
                assert!(
                    !stderr.contains(DRIFT_WARNING),
                    "hook · {label}: case (v) — the hook prints nothing for a probe-integrity \
                     finding; stderr:\n{stderr}",
                );
            }
        }
    }
}
