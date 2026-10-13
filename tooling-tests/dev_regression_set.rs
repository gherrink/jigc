//! **`dev/regression-set` — the previous release's test suite, run against the candidate's
//! binary** ([DECISIONS.md](../DECISIONS.md) → *2026-10-06 — The regression set's first
//! part, as a tool*; [stabilization-workflow.md](../implementation/stabilization-workflow.md)
//! → The regression set).
//!
//! The tool builds a release's tests, runs them on their own binary, puts the candidate's
//! binary where they look for theirs and runs them again; a test that passed and now fails
//! must be on a committed list of intended changes. A real run builds two workspaces and
//! takes twenty minutes, so **nothing here builds one**. What is held instead:
//!
//! - *The verdict, over recorded evidence.* `verdict` is the function a run ends with, and
//!   it reads a directory: two reports of the runner's and `run.json`. Each arm writes that
//!   directory and reads the line — as [`dev_gate_report`](super::dev_gate_report) hands
//!   the gate a log. The comparison, the list matching, the stale rows, the tests excluded
//!   for failing on their own binary, every void reason — and, the arm the tool exists
//!   for, [`a_binary_that_was_put_back_is_void_and_never_green`]: evidence that is green
//!   in every report and whose hash after the run is the previous release's.
//! - *The list's form*, and that a row pointing at nothing refuses the whole list — against
//!   a throwaway repository with a commit before the release, the release, an intended
//!   change, the candidate, and a commit after it.
//! - *Which list a verdict rests on.* The list is a path **in the candidate's commit**, so
//!   every arm's candidate is a commit that holds its list ([`Rig::candidate_with`]); a file
//!   the commit does not hold is refused, whatever the working tree has at the path
//!   ([`a_list_that_is_not_in_the_candidates_commit_is_refused`]); a run records the list
//!   it read, and a verdict over another one is void
//!   ([`a_verdict_over_another_list_than_the_run_read_is_void_and_never_green`]).
//! - *What drops out of the comparison.* A test that fails on its own binary is excluded
//!   only by a row of the list that names it and points at the committed line of a run
//!   that showed it ([`an_exclusion_row_points_at_the_line_of_a_run_that_showed_the_test_failing`]);
//!   on no row it voids the run and is named
//!   ([`a_test_that_fails_on_its_own_binary_and_is_on_no_row_is_void_and_named`]), however
//!   few such tests there are
//!   ([`no_number_of_tests_drops_out_unnamed_and_none_is_too_many_with_its_row`]).
//! - *The run's own order*, with a stand-in for cargo ([`FAKE_CARGO`]) that builds nothing
//!   and is **faithful where the trap is**: asked for a run that is not made from the
//!   recorded build, it copies the previous release's binary back over the path, as cargo
//!   does; and the report it writes is chosen by the bytes it finds at the path, as the
//!   real tests' outcomes are. So a run that lost its swap reads green in its reports —
//!   and is answered void by the hashes, or the arm is red.
//! - *The source*: no run after the build goes through cargo's build
//!   ([`no_run_goes_through_cargos_build`]), with a plant.
//!
//! **Every arm runs under a shell-hostile root** — a space, both quotes and a `#` in the
//! repository's path and in the scratch directory's.
//!
//! **What no arm here can show** is the real thing: that cargo-nextest's reuse-build path
//! leaves the swapped file alone, and that a real suite's reports parse. The first is the
//! measurement the tool was built from, and every real run re-proves it with its hashes;
//! [`a_report_is_read_as_the_runner_writes_it`] holds the parser to a fragment of a real
//! report.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{Value, json};

use crate::support::scratch::ScratchDir;

use super::placed_executable;

const TOOL: &str = "dev/regression-set";

/// The list's path in the rig's repository, as a call names it.
const LIST: &str = "record/intended-changes.tsv";

/// Where the rig's exclusion rows point: the committed line of an earlier run of the tool.
const RECORD: &str = "record/first-run.json";

/// A space, both quotes and a `#`: a path that breaks any command that is not quoted, and
/// any TOML string that is not escaped.
const HOSTILE_REPO: &str = "it's a \"repo\" #1";
const HOSTILE_SCRATCH: &str = "a \"scratch\" it's #2";

/// What the tool exits with: the two verdicts, the word of each refusal, the reason of
/// each void. [`the_header_is_the_help_and_its_status_table_is_this_one`] holds the tool's
/// header to it.
const STATUSES: &[(&str, i32)] = &[
    ("green", 0),
    ("red", 1),
    ("usage", 2),
    ("list", 3),
    ("commit", 4),
    ("scratch", 5),
    ("did-not-run", 10),
    ("build-previous", 11),
    ("build-candidate", 12),
    ("baseline", 13),
    ("swap", 14),
    ("incomplete", 15),
    ("evidence", 16),
];

fn status_of(word: &str) -> i32 {
    STATUSES
        .iter()
        .find(|(known, _)| *known == word)
        .unwrap_or_else(|| panic!("`{word}` is a status of the tool"))
        .1
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

// ---------------------------------------------------------------------------
// The rig
// ---------------------------------------------------------------------------

/// A throwaway repository holding the tool's own bytes, and a scratch directory beside it.
///
/// Its history is one line: a commit [`before`](Rig::before) the release, the
/// [`previous`](Rig::previous) release, an [`intended`](Rig::intended) change, the
/// [`candidate`](Rig::candidate) — whose `DECISIONS.md` has one heading that opens *The
/// cap is ruled* and two that open *Twice* — and a commit [`later`](Rig::later) than it.
///
/// **The candidate a call names is never that commit itself**: the list is read out of the
/// candidate's commit, so each arm's candidate is one commit on top of it that holds the
/// arm's list at [`LIST`] ([`candidate_with`](Rig::candidate_with)).
struct Rig {
    dir: ScratchDir,
    root: PathBuf,
    scratch: PathBuf,
    path: String,
    before: String,
    previous: String,
    intended: String,
    candidate: String,
    later: String,
}

/// What a call of the tool left.
struct Called {
    code: i32,
    raw: String,
    line: Value,
    stderr: String,
}

impl Called {
    /// The call ended in the verdict `status` — `green` or `red`.
    fn verdict(&self, status: &str) -> &Value {
        assert_eq!(
            (self.code, self.line["status"].as_str()),
            (status_of(status), Some(status)),
            "the verdict is `{status}`\nline: {}\nstderr: {}",
            self.raw,
            self.stderr
        );
        assert_eq!(
            self.line["reason"],
            Value::Null,
            "a verdict has no reason: {}",
            self.raw
        );
        &self.line
    }

    /// The call ended void for `reason`, and its sentence holds `saying`.
    fn void(&self, reason: &str, saying: &str) -> &Value {
        assert_eq!(
            (
                self.code,
                self.line["status"].as_str(),
                self.line["reason"].as_str()
            ),
            (status_of(reason), Some("void"), Some(reason)),
            "void for `{reason}`\nline: {}\nstderr: {}",
            self.raw,
            self.stderr
        );
        let detail = self.line["detail"]
            .as_str()
            .expect("a void has its sentence");
        assert!(
            detail.contains(saying),
            "void `{reason}` says `{saying}`: {detail}"
        );
        for verdict in ["counts", "not_on_list", "on_list"] {
            assert_eq!(
                self.line[verdict],
                Value::Null,
                "a void run has no `{verdict}`: {}",
                self.raw
            );
        }
        &self.line
    }

    /// The call was refused with `word`, and its sentence holds `saying`.
    fn refused(&self, word: &str, saying: &str) {
        assert_eq!(
            (
                self.code,
                self.line["status"].as_str(),
                self.line["refused"].as_str()
            ),
            (status_of(word), Some("refused"), Some(word)),
            "refused `{word}`\nline: {}\nstderr: {}",
            self.raw,
            self.stderr
        );
        let opening = format!("regression-set: refused {word}: ");
        assert!(
            self.stderr.starts_with(&opening) && self.stderr.trim_end().lines().count() == 1,
            "a refusal is one line on stderr that opens `{opening}`: {}",
            self.stderr
        );
        assert!(
            self.stderr.contains(saying),
            "refused `{word}` says `{saying}`: {}",
            self.stderr
        );
    }
}

impl Rig {
    fn new() -> Rig {
        let dir = ScratchDir::new("regression-set");
        let root = dir.path().join(HOSTILE_REPO);
        let scratch = dir.path().join(HOSTILE_SCRATCH);
        for made in [
            &root,
            &scratch,
            &dir.path().join("home"),
            &dir.path().join("bin"),
        ] {
            fs::create_dir_all(made).expect("create the rig's directories");
        }
        // As the tool names it: it resolves the scratch directory before it mints in it.
        let scratch = scratch
            .canonicalize()
            .expect("the scratch directory exists");
        let path = format!(
            "{}:{}",
            dir.path().join("bin").display(),
            std::env::var("PATH").expect("the suite runs with a UTF-8 PATH")
        );
        let mut rig = Rig {
            dir,
            root,
            scratch,
            path,
            before: String::new(),
            previous: String::new(),
            intended: String::new(),
            candidate: String::new(),
            later: String::new(),
        };
        rig.git(&["init", "-q", "-b", "trunk"]);
        rig.before = rig.change("README.md", "a product\n", "chore: the first commit");
        rig.previous = rig.change(
            "Cargo.toml",
            "version = \"1.0.0\"\n",
            "chore: release 1.0.0",
        );
        rig.intended = rig.change("src.rs", "fn refuses() {}\n", "fix: it refuses now");
        rig.candidate = rig.change(
            "DECISIONS.md",
            "# Decisions\n\n## 2026-10-06 — The cap is ruled: ten thousand (the human's)\n\n\
             The cap is ruled in prose too.\n\n## Twice — the first\n\n## Twice — the second\n",
            "docs: the ruling",
        );
        rig.later = rig.change("later.md", "after the candidate\n", "docs: later");
        fs::create_dir_all(rig.root.join("dev")).expect("create the tool's directory");
        placed_executable::copy(&repo_root().join(TOOL), &rig.root.join(TOOL));
        rig
    }

    /// The environment every child of the rig runs in: its own home, no git configuration
    /// of the machine's, a fixed identity and fixed dates.
    fn hermetic(&self, mut command: Command) -> Command {
        command
            .env("PATH", &self.path)
            .env("HOME", self.dir.path().join("home"))
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
            .env_remove("CARGO_TARGET_DIR")
            .env_remove("SDKROOT");
        command
    }

    fn git(&self, args: &[&str]) -> String {
        let out = self
            .hermetic(Command::new("git"))
            .args(args)
            .current_dir(&self.root)
            .output()
            .expect("run git");
        assert!(
            out.status.success(),
            "`git {}` in the rig: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim_end().to_owned()
    }

    /// One file written and committed; the commit's sha.
    fn change(&self, rel: &str, text: &str, subject: &str) -> String {
        self.commit(&[(rel, text.as_bytes())], subject)
    }

    /// These files written and committed as one commit on the commit checked out; its sha.
    fn commit(&self, files: &[(&str, &[u8])], subject: &str) -> String {
        for (rel, bytes) in files {
            let path = self.root.join(rel);
            fs::create_dir_all(path.parent().expect("a file has a directory"))
                .unwrap_or_else(|e| panic!("create the directory of `{rel}`: {e}"));
            fs::write(path, bytes).unwrap_or_else(|e| panic!("write `{rel}`: {e}"));
            self.git(&["add", "--", rel]);
        }
        self.git(&["commit", "-q", "-m", subject]);
        self.git(&["rev-parse", "HEAD"])
    }

    /// A candidate that holds these files: one commit on top of the rig's
    /// [`candidate`](Rig::candidate). The rig's dates and identity are fixed, so the same
    /// files give the same commit however often it is asked for.
    fn candidate_holding(&self, files: &[(&str, &[u8])]) -> String {
        self.git(&["checkout", "-q", "--detach", &self.candidate]);
        self.commit(files, "docs: the list of intended changes")
    }

    /// The candidate whose commit holds a list of this text at [`LIST`] — and, at
    /// [`RECORD`], the line of the run its exclusion rows point at.
    fn candidate_with(&self, list: impl AsRef<[u8]>) -> String {
        let record = self.record_of(&String::from_utf8_lossy(list.as_ref()));
        self.candidate_holding(&[(LIST, list.as_ref()), (RECORD, record.as_bytes())])
    }

    /// The line of a run against the rig's release in which every test failed on its own
    /// binary that a row of `list` excludes by pointing at [`RECORD`]: the first of them
    /// named as a line names a test that had its row, the others as tests that had none.
    fn record_of(&self, list: &str) -> String {
        let pointer = format!("excluded:{RECORD}");
        let failed: Vec<Value> = list
            .lines()
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .filter(|fields| fields.len() == 4 && fields[3] == pointer)
            .map(|fields| json!({"binary": fields[0], "test": fields[1]}))
            .collect();
        let (with_a_row, with_none) = failed.split_at(failed.len().min(1));
        json!({
            "tool": "regression-set",
            "status": "red",
            "previous": {"commit": self.previous, "sha256": OLD_HASH},
            "excluded": with_a_row,
            "excluded_by_no_row": with_none,
        })
        .to_string()
    }

    /// The tool, called from outside its repository — it finds that from where it lies.
    fn tool_as(&self, args: &[&str], shape: impl FnOnce(&mut Command)) -> Called {
        let mut command = self.hermetic(Command::new(self.root.join(TOOL)));
        command.args(args).current_dir(self.dir.path());
        shape(&mut command);
        let out = command.output().expect("spawn the tool");
        let raw = String::from_utf8(out.stdout).expect("the tool prints UTF-8");
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            raw.ends_with('\n') && raw.trim_end().lines().count() == 1,
            "the tool prints ONE line on stdout: {raw:?}\nstderr: {stderr}"
        );
        let line: Value = serde_json::from_str(raw.trim_end())
            .unwrap_or_else(|e| panic!("the line is JSON ({e}): {raw}"));
        assert_eq!(
            line["tool"], "regression-set",
            "the line names its tool: {raw}"
        );
        Called {
            code: out.status.code().expect("the tool exits"),
            raw,
            line,
            stderr,
        }
    }

    fn tool(&self, args: &[&str]) -> Called {
        self.tool_as(args, |_| {})
    }

    /// A file with this text outside the repository — what no commit holds.
    fn file_outside(&self, text: impl AsRef<[u8]>) -> String {
        let path = self.dir.path().join("intended-changes.tsv");
        fs::write(&path, text).expect("write the file");
        path.to_str().expect("a UTF-8 path").to_owned()
    }

    /// A row of the list: the test, and a pointer at the rig's intended change.
    fn row(&self, binary: &str, test: &str) -> String {
        format!(
            "{binary}\t{test}\tit refuses now\tcommit:{}\n",
            self.intended
        )
    }

    /// An exclusion row of the list: the test, and a pointer at [`RECORD`].
    fn excluding(&self, binary: &str, test: &str) -> String {
        format!("{binary}\t{test}\t{NO_REPOSITORY}\texcluded:{RECORD}\n")
    }

    /// `check-list` over a list with this text, between the rig's release and the
    /// candidate that holds it.
    fn check_list(&self, text: impl AsRef<[u8]>) -> Called {
        let candidate = self.candidate_with(text);
        self.tool(&[
            "check-list",
            "--previous",
            &self.previous,
            "--candidate",
            &candidate,
            "--list",
            LIST,
        ])
    }

    /// The facts of a run that held, over an empty list.
    fn facts(&self) -> Value {
        self.facts_over("")
    }

    /// The facts of a run that held, over a list of this text: two binaries, a path a test
    /// binary names, the previous release's hash around the baseline and the candidate's
    /// around its run — and the list the run read, by its path and its hash, in the
    /// candidate that holds it.
    fn facts_over(&self, list: &str) -> Value {
        json!({
            "previous": {"commit": self.previous, "sha256": OLD_HASH},
            "candidate": {"commit": self.candidate_with(list), "sha256": NEW_HASH},
            "list": {"path": LIST, "sha256": sha256_of(list.as_bytes())},
            "baked_path": "/work/target-previous/debug/jigc",
            "named_by": 12,
            "at_baked_path": {
                "baseline_before": OLD_HASH,
                "baseline_after": OLD_HASH,
                "candidate_before": NEW_HASH,
                "candidate_after": NEW_HASH,
            },
            "failed": null,
            "seconds": {"baseline": 511, "candidate": 589},
        })
    }

    /// An evidence directory: `run.json` and the two reports, each as given.
    fn evidence(
        &self,
        facts: Option<&str>,
        baseline: Option<&str>,
        candidate: Option<&str>,
    ) -> String {
        let at = self.dir.path().join("evidence");
        let _ = fs::remove_dir_all(&at);
        fs::create_dir_all(&at).expect("create the evidence directory");
        for (name, text) in [
            ("run.json", facts),
            ("baseline-junit.xml", baseline),
            ("candidate-junit.xml", candidate),
        ] {
            if let Some(text) = text {
                fs::write(at.join(name), text).expect("write a file of the evidence");
            }
        }
        at.to_str().expect("a UTF-8 path").to_owned()
    }

    /// `verdict` over the run `suite` describes, with `facts` — against the list at
    /// [`LIST`] in the candidate the facts name.
    fn verdict_with(&self, facts: &Value, suite: &[Ran]) -> Called {
        let (baseline, candidate) = reports(suite);
        let evidence = self.evidence(Some(&facts.to_string()), Some(&baseline), Some(&candidate));
        self.verdict_over(&evidence)
    }

    /// `verdict` over the run `suite` describes, made over a list of this text.
    fn verdict(&self, suite: &[Ran], list: &str) -> Called {
        self.verdict_with(&self.facts_over(list), suite)
    }

    fn verdict_over(&self, evidence: &str) -> Called {
        self.tool(&["verdict", "--evidence", evidence, "--list", LIST])
    }
}

/// Why the rig's excluded tests fail on their own binary.
const NO_REPOSITORY: &str = "it asks git about a tree that is no repository";

const OLD_HASH: &str = "a906cd49003ebedd95f0cb96f3fd06d591b7561150e01a042909b0da2fc9835f";
const NEW_HASH: &str = "7fdc1355e52a5be2e5a489c4535cac0239d3eeda08d91f8c1fca9c6afeb737da";
const OTHER_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

// ---------------------------------------------------------------------------
// Reports
// ---------------------------------------------------------------------------

/// How a test ended in one run.
#[derive(Clone, Copy, PartialEq)]
enum End {
    Pass,
    /// Failed, then passed on its retry.
    Flaky,
    /// Failed twice.
    Fail,
}
use End::{Fail, Flaky, Pass};

/// One test of the previous release's suite: its binary, its name, and how it ended on
/// the previous release's binary and on the candidate's.
struct Ran(&'static str, String, End, End);

fn ran(binary: &'static str, test: &str, on_previous: End, on_candidate: End) -> Ran {
    Ran(binary, test.to_owned(), on_previous, on_candidate)
}

/// `count` tests that pass on both binaries and reach neither — the unit tests.
fn quiet(count: usize) -> Vec<Ran> {
    (0..count)
        .map(|n| ran("jigc-engine", &format!("unit::holds_{n}"), Pass, Pass))
        .collect()
}

/// A report in the shape cargo-nextest writes: one `<testsuite>` per binary, a
/// `<failure>` under a test that failed, a `<flakyFailure>` under one that passed on its
/// retry.
fn junit(cases: &[(&str, &str, End)]) -> String {
    let mut binaries: Vec<&str> = cases.iter().map(|case| case.0).collect();
    binaries.sort_unstable();
    binaries.dedup();
    let mut out = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites name=\"nextest-run\" tests=\"{}\">\n",
        cases.len()
    );
    for binary in binaries {
        out.push_str(&format!("    <testsuite name=\"{binary}\">\n"));
        for (_, test, end) in cases.iter().filter(|case| case.0 == binary) {
            let open =
                format!("        <testcase name=\"{test}\" classname=\"{binary}\" time=\"0.020\"");
            out.push_str(&match end {
                Pass => format!("{open}/>\n"),
                Flaky => format!(
                    "{open}>\n            <flakyFailure type=\"test failure with exit code 101\">once</flakyFailure>\n        </testcase>\n"
                ),
                Fail => format!(
                    "{open}>\n            <failure type=\"test failure with exit code 101\">twice</failure>\n            <rerunFailure type=\"test failure with exit code 101\">once</rerunFailure>\n        </testcase>\n"
                ),
            });
        }
        out.push_str("    </testsuite>\n");
    }
    out.push_str("</testsuites>\n");
    out
}

/// The two reports of a run: the baseline's and the candidate's.
fn reports(suite: &[Ran]) -> (String, String) {
    let on = |pick: fn(&Ran) -> End| {
        junit(
            &suite
                .iter()
                .map(|test| (test.0, test.1.as_str(), pick(test)))
                .collect::<Vec<_>>(),
        )
    };
    (on(|test| test.2), on(|test| test.3))
}

/// The `binary test` pairs of an array of the line.
fn pairs(named: &Value) -> Vec<String> {
    named
        .as_array()
        .unwrap_or_else(|| panic!("an array of tests: {named}"))
        .iter()
        .map(|test| {
            format!(
                "{} {}",
                test["binary"].as_str().expect("a test has its binary"),
                test["test"].as_str().expect("a test has its name")
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The verdict, over recorded evidence
// ---------------------------------------------------------------------------

/// The suite most arms compare: two differences, one test that fails on its own binary
/// and on the candidate, one that fails on its own binary only, one flaky in each run.
fn a_suite() -> Vec<Ran> {
    let mut suite = quiet(400);
    suite.extend([
        ran("jigc::g_flow", "setup::refuses_a_seeded_file", Pass, Fail),
        ran("jigc::g_migrate", "uninstall::says_dropped", Pass, Fail),
        ran("jigc::g_flow", "fences::reads_the_git_history", Fail, Fail),
        ran("jigc::g_flow", "discard::names_its_commit", Fail, Pass),
        ran("jigc::g_flow", "corpus::copies_a_tree", Flaky, Pass),
        ran("jigc::g_item", "milestone::lands", Pass, Flaky),
        ran("jigc::g_item", "task::finalizes", Pass, Pass),
    ]);
    suite
}

#[test]
fn every_difference_on_the_list_is_green() {
    let rig = Rig::new();
    let list = format!(
        "# intended changes since 1.0.0\n\n{}jigc::g_migrate\tuninstall::says_dropped\tthe ack says dropped\truling:DECISIONS.md#2026-10-06 — The cap is ruled\n\n# what fails on its own binary\n{}{}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"),
        rig.excluding("jigc::g_flow", "fences::reads_the_git_history"),
        rig.excluding("jigc::g_flow", "discard::names_its_commit"),
    );
    let called = rig.verdict(&a_suite(), &list);
    let line = called.verdict("green");

    assert_eq!(
        line["counts"],
        json!({
            "tests": 407,
            "baseline_passed": 405,
            "baseline_failed": 2,
            "candidate_passed": 404,
            "candidate_failed": 3,
            "differences": 2,
            "on_list": 2,
            "not_on_list": 0,
            "excluded": 2,
            "excluded_by_no_row": 0,
            "stale": 0,
            "fixed": 1,
        }),
        "the counts: {}",
        called.raw
    );
    assert_eq!(pairs(&line["not_on_list"]), Vec::<String>::new());
    assert_eq!(
        line["on_list"],
        json!([
            {
                "binary": "jigc::g_flow",
                "test": "setup::refuses_a_seeded_file",
                "change": "it refuses now",
                "pointer": format!("commit:{}", rig.intended),
            },
            {
                "binary": "jigc::g_migrate",
                "test": "uninstall::says_dropped",
                "change": "the ack says dropped",
                "pointer": "ruling:DECISIONS.md#2026-10-06 — The cap is ruled",
            },
        ]),
        "each difference with the row that lists it"
    );
    let excluded = format!("excluded:{RECORD}");
    assert_eq!(
        line["excluded"],
        json!([
            {
                "binary": "jigc::g_flow",
                "test": "discard::names_its_commit",
                "change": NO_REPOSITORY,
                "pointer": excluded,
            },
            {
                "binary": "jigc::g_flow",
                "test": "fences::reads_the_git_history",
                "change": NO_REPOSITORY,
                "pointer": excluded,
            },
        ]),
        "each test that failed on its own binary with the row that excludes it — exactly \
         the exclusion rows"
    );
    assert_eq!(line["excluded_by_no_row"], json!([]));
    assert_eq!(line["stale"], json!([]));
    let candidate = rig.candidate_with(&list);
    assert_eq!(
        line["list"],
        json!({
            "path": LIST,
            "commit": candidate,
            "sha256": sha256_of(list.as_bytes()),
            "rows": 4,
        }),
        "the line says which list the verdict rests on: its path, the commit it was read \
         out of, and its hash"
    );
    assert_eq!(
        line["previous"],
        json!({"commit": rig.previous, "sha256": OLD_HASH})
    );
    assert_eq!(
        line["candidate"],
        json!({"commit": candidate, "sha256": NEW_HASH})
    );
    assert_eq!(
        line["proof"],
        json!({
            "baked_path": "/work/target-previous/debug/jigc",
            "named_by": 12,
            "baseline_before": OLD_HASH,
            "baseline_after": OLD_HASH,
            "candidate_before": NEW_HASH,
            "candidate_after": NEW_HASH,
        }),
        "the line carries what proves which binary each run drove"
    );
}

#[test]
fn no_difference_at_all_is_green_under_an_empty_list() {
    let rig = Rig::new();
    let called = rig.verdict(&quiet(3), "");
    let line = called.verdict("green");
    assert_eq!(line["counts"]["differences"], 0);
    assert_eq!(line["counts"]["tests"], 3);
}

#[test]
fn a_difference_that_is_not_on_the_list_is_red_and_named() {
    let rig = Rig::new();
    let list = rig.row("jigc::g_flow", "setup::refuses_a_seeded_file");
    let called = rig.verdict(&a_suite(), &list);
    let line = called.verdict("red");

    assert_eq!(
        pairs(&line["not_on_list"]),
        ["jigc::g_migrate uninstall::says_dropped"],
        "the unlisted difference, by binary and name: {}",
        called.raw
    );
    assert_eq!(
        pairs(&line["on_list"]),
        ["jigc::g_flow setup::refuses_a_seeded_file"],
        "and the listed one beside it"
    );
    assert_eq!(line["counts"]["not_on_list"], 1);
    assert_eq!(line["counts"]["on_list"], 1);
}

/// A row is matched by binary **and** name: the same test name in another binary is
/// another test.
#[test]
fn a_row_for_the_same_name_in_another_binary_lists_nothing() {
    let rig = Rig::new();
    let suite = [
        ran("jigc", "tests::render_holds", Pass, Pass),
        ran("jigc::bin/jigc", "tests::render_holds", Pass, Fail),
    ];
    let called = rig.verdict(&suite, &rig.row("jigc", "tests::render_holds"));
    let line = called.verdict("red");
    assert_eq!(
        pairs(&line["not_on_list"]),
        ["jigc::bin/jigc tests::render_holds"]
    );
    assert_eq!(line["stale"][0]["why"], "passed-on-the-candidate");
}

/// **A test drops out of the comparison only by a row that names it** (the re-review's
/// `R-M7`: up to one test in a hundred dropped out with the verdict green, and no list
/// said which may). A test that fails on its own binary and is on no exclusion row voids
/// the run, and the line names it. With a difference off the list as well the run is red —
/// that much is established whatever the baseline lacks — and the line names both.
#[test]
fn a_test_that_fails_on_its_own_binary_and_is_on_no_row_is_void_and_named() {
    let rig = Rig::new();
    let changes = format!(
        "{}{}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"),
        rig.row("jigc::g_migrate", "uninstall::says_dropped"),
    );
    let one_row = rig.excluding("jigc::g_flow", "fences::reads_the_git_history");
    let both_rows = format!(
        "{one_row}{}",
        rig.excluding("jigc::g_flow", "discard::names_its_commit")
    );

    // Neither has a row: void, and both are named.
    let called = rig.verdict(&a_suite(), &changes);
    let line = called.void("baseline", "2 of the 407 tests failed on their own binary");
    assert_eq!(
        pairs(&line["excluded_by_no_row"]),
        [
            "jigc::g_flow discard::names_its_commit",
            "jigc::g_flow fences::reads_the_git_history",
        ]
    );

    // One has none — the one that passes on the candidate's binary, so nothing else moved.
    let called = rig.verdict(&a_suite(), &format!("{changes}{one_row}"));
    let line = called.void(
        "baseline",
        "1 of the 407 tests failed on their own binary and no exclusion row of the list names \
         them, `jigc::g_flow` `discard::names_its_commit` among them",
    );
    assert_eq!(
        pairs(&line["excluded_by_no_row"]),
        ["jigc::g_flow discard::names_its_commit"],
        "the void line names the test no row excludes: {}",
        called.raw
    );

    // Each of the two has its row: green, and what is excluded is exactly the rows.
    let called = rig.verdict(&a_suite(), &format!("{changes}{both_rows}"));
    let line = called.verdict("green");
    assert_eq!(
        pairs(&line["excluded"]),
        [
            "jigc::g_flow discard::names_its_commit",
            "jigc::g_flow fences::reads_the_git_history",
        ],
        "a test that failed on the previous release's binary is no difference, whatever it \
         did on the candidate's — and is named: {}",
        called.raw
    );
    assert_eq!(line["excluded_by_no_row"], json!([]));

    // And a difference off the list beside it: red, with both named.
    let list = format!(
        "{}{one_row}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file")
    );
    let called = rig.verdict(&a_suite(), &list);
    let line = called.verdict("red");
    assert_eq!(
        pairs(&line["not_on_list"]),
        ["jigc::g_migrate uninstall::says_dropped"]
    );
    assert_eq!(
        pairs(&line["excluded"]),
        ["jigc::g_flow fences::reads_the_git_history"]
    );
    assert_eq!(
        pairs(&line["excluded_by_no_row"]),
        ["jigc::g_flow discard::names_its_commit"],
        "a red line names what no row excludes too: {}",
        called.raw
    );
    assert_eq!(
        (
            &line["counts"]["baseline_failed"],
            &line["counts"]["excluded"],
            &line["counts"]["excluded_by_no_row"],
        ),
        (&json!(2), &json!(1), &json!(1))
    );
}

#[test]
fn a_test_that_passed_on_its_retry_passed_and_is_named_flaky() {
    let rig = Rig::new();
    let mut suite = quiet(5);
    suite.extend([
        ran("jigc::g_flow", "corpus::copies_a_tree", Flaky, Pass),
        ran("jigc::g_item", "milestone::lands", Pass, Flaky),
        // Flaky on its own binary is still a pass there: failing on the candidate differs.
        ran("jigc::g_item", "task::races", Flaky, Fail),
    ]);
    let called = rig.verdict(&suite, "");
    let line = called.verdict("red");
    assert_eq!(pairs(&line["not_on_list"]), ["jigc::g_item task::races"]);
    assert_eq!(pairs(&line["excluded"]), Vec::<String>::new());
    assert_eq!(
        pairs(&line["flaky"]["baseline"]),
        [
            "jigc::g_flow corpus::copies_a_tree",
            "jigc::g_item task::races"
        ]
    );
    assert_eq!(
        pairs(&line["flaky"]["candidate"]),
        ["jigc::g_item milestone::lands"]
    );
}

#[test]
fn a_row_that_matches_nothing_is_stale_and_says_why() {
    let rig = Rig::new();
    let held = format!(
        "{}{}{}{}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"),
        rig.row("jigc::g_migrate", "uninstall::says_dropped"),
        rig.excluding("jigc::g_flow", "fences::reads_the_git_history"),
        rig.excluding("jigc::g_flow", "discard::names_its_commit"),
    );
    let list = format!(
        "{held}{}{}{}{}",
        rig.row("jigc::g_flow", "setup::a_test_nobody_wrote"),
        rig.row("jigc::g_item", "task::finalizes"),
        rig.excluding("jigc::g_flow", "fences::a_test_nobody_wrote"),
        // Flaky on its own binary is a pass there.
        rig.excluding("jigc::g_flow", "corpus::copies_a_tree"),
    );
    let called = rig.verdict(&a_suite(), &list);
    // Stale rows are reported and change no verdict: every difference is still listed,
    // and every test that failed on its own binary still has its row.
    let line = called.verdict("green");
    let commit = format!("commit:{}", rig.intended);
    let excluded = format!("excluded:{RECORD}");
    assert_eq!(
        line["stale"],
        json!([
            {"binary": "jigc::g_flow", "test": "corpus::copies_a_tree",
             "pointer": excluded, "why": "passed-on-its-own-binary"},
            {"binary": "jigc::g_flow", "test": "fences::a_test_nobody_wrote",
             "pointer": excluded, "why": "no-such-test"},
            {"binary": "jigc::g_flow", "test": "setup::a_test_nobody_wrote",
             "pointer": commit, "why": "no-such-test"},
            {"binary": "jigc::g_item", "test": "task::finalizes",
             "pointer": commit, "why": "passed-on-the-candidate"},
        ]),
        "each stale row with why: {}",
        called.raw
    );
    assert_eq!(line["counts"]["stale"], 4);
    assert_eq!(line["counts"]["on_list"], 2);
    assert_eq!(line["counts"]["excluded"], 2);

    // The two kinds of row do not stand in for each other. A difference under an
    // exclusion row is not listed, and a test that failed on its own binary under a row
    // for a change is not excluded: each row is stale, and each test is still owed its own.
    let crossed = format!(
        "{}{}{}{}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"),
        rig.excluding("jigc::g_migrate", "uninstall::says_dropped"),
        rig.row("jigc::g_flow", "fences::reads_the_git_history"),
        rig.excluding("jigc::g_flow", "discard::names_its_commit"),
    );
    let called = rig.verdict(&a_suite(), &crossed);
    let line = called.verdict("red");
    assert_eq!(
        pairs(&line["not_on_list"]),
        ["jigc::g_migrate uninstall::says_dropped"],
        "{}",
        called.raw
    );
    assert_eq!(
        pairs(&line["excluded_by_no_row"]),
        ["jigc::g_flow fences::reads_the_git_history"]
    );
    assert_eq!(
        line["stale"],
        json!([
            {"binary": "jigc::g_flow", "test": "fences::reads_the_git_history",
             "pointer": commit, "why": "failed-on-its-own-binary"},
            {"binary": "jigc::g_migrate", "test": "uninstall::says_dropped",
             "pointer": excluded, "why": "passed-on-its-own-binary"},
        ])
    );
}

/// **The arm the tool exists for.** Every report is green — the candidate's run shows no
/// difference at all, which is exactly what a run on the previous release's binary shows —
/// and the hash taken at the path after the run is the previous release's.
#[test]
fn a_binary_that_was_put_back_is_void_and_never_green() {
    let rig = Rig::new();
    let mut facts = rig.facts();
    facts["at_baked_path"]["candidate_after"] = json!(OLD_HASH);
    let called = rig.verdict_with(&facts, &quiet(40));
    let line = called.void("swap", "the previous release's binary, put back");
    assert_eq!(
        line["proof"]["candidate_after"], OLD_HASH,
        "the void line carries the hash that voided it"
    );

    // The control: the same evidence with the candidate's hash after the run is green.
    rig.verdict(&quiet(40), "").verdict("green");
}

/// **No number of tests drops out unnamed, and none is too many once each has its row.**
/// The tool compared against a baseline while at most one test in a hundred failed on its
/// own binary; 2 of 200, on no row, were green (the re-review's `R-M7`, whose block this
/// is).
#[test]
fn no_number_of_tests_drops_out_unnamed_and_none_is_too_many_with_its_row() {
    let rig = Rig::new();
    let mut suite = quiet(198);
    suite.extend([
        ran("jigc::g_flow", "fences::one", Fail, Fail),
        ran("jigc::g_flow", "fences::two", Fail, Fail),
    ]);
    let called = rig.verdict(&suite, "");
    let line = called.void(
        "baseline",
        "2 of the 200 tests failed on their own binary and no exclusion row",
    );
    assert_eq!(
        pairs(&line["excluded_by_no_row"]),
        ["jigc::g_flow fences::one", "jigc::g_flow fences::two"]
    );

    // Most of a suite failing on its own binary, each test on its row: nothing is capped.
    let mut suite = quiet(10);
    let mut list = String::new();
    for n in 0..60 {
        let test = format!("fences::asks_git_{n:02}");
        suite.push(ran("jigc::g_flow", &test, Fail, Fail));
        list.push_str(&rig.excluding("jigc::g_flow", &test));
    }
    let called = rig.verdict(&suite, &list);
    let line = called.verdict("green");
    assert_eq!(
        (
            &line["counts"]["excluded"],
            &line["counts"]["excluded_by_no_row"]
        ),
        (&json!(60), &json!(0)),
        "{}",
        called.raw
    );
}

/// One row per way a run fails to establish a verdict: what is changed in evidence that
/// is otherwise green, the reason, and words of its sentence.
#[test]
fn each_way_a_run_establishes_nothing_is_void_with_its_reason() {
    let rig = Rig::new();
    let suite = quiet(40);
    let (baseline, candidate) = reports(&suite);
    let (baseline, candidate) = (baseline.as_str(), candidate.as_str());
    let facts_with = |change: &dyn Fn(&mut Value)| {
        let mut facts = rig.facts();
        change(&mut facts);
        facts.to_string()
    };
    let good = rig.facts().to_string();
    let short = junit(&[("jigc-engine", "unit::holds_0", Pass)]);
    let mut longer = suite
        .iter()
        .map(|t| (t.0, t.1.as_str(), t.3))
        .collect::<Vec<_>>();
    longer.push(("jigc::g_flow", "setup::a_test_only_this_run_ran", Pass));
    let longer = junit(&longer);
    let twice = junit(&[
        ("jigc-engine", "unit::holds_0", Pass),
        ("jigc-engine", "unit::holds_0", Pass),
    ]);
    let recorded = |reason: &str| {
        facts_with(&|facts: &mut Value| {
            facts["failed"] =
                json!({"reason": reason, "detail": "`cargo build -p jigc` exited 101"});
        })
    };

    type Row<'a> = (
        &'a str,
        Option<String>,
        Option<&'a str>,
        Option<&'a str>,
        &'a str,
        &'a str,
    );
    let rows: Vec<Row> = vec![
        // The swap.
        (
            "the file at the path was not the candidate's before its run",
            Some(facts_with(&|f: &mut Value| {
                f["at_baked_path"]["candidate_before"] = json!(OLD_HASH)
            })),
            Some(baseline),
            Some(candidate),
            "swap",
            "before the run the file at the path was the previous release's binary",
        ),
        (
            "the file at the path was neither binary after the run",
            Some(facts_with(&|f: &mut Value| {
                f["at_baked_path"]["candidate_after"] = json!(OTHER_HASH)
            })),
            Some(baseline),
            Some(candidate),
            "swap",
            "after the run the file at the path was another file",
        ),
        (
            "the two binaries are the same bytes",
            Some(facts_with(&|f: &mut Value| {
                f["candidate"]["sha256"] = json!(OLD_HASH);
                f["at_baked_path"]["candidate_before"] = json!(OLD_HASH);
                f["at_baked_path"]["candidate_after"] = json!(OLD_HASH);
            })),
            Some(baseline),
            Some(candidate),
            "swap",
            "byte for byte the previous release's",
        ),
        (
            "no test binary names the path",
            Some(facts_with(&|f: &mut Value| f["named_by"] = json!(0))),
            Some(baseline),
            Some(candidate),
            "swap",
            "no test binary of the recorded build names the path",
        ),
        // The baseline.
        (
            "the file at the path was not the previous release's before the baseline",
            Some(facts_with(&|f: &mut Value| {
                f["at_baked_path"]["baseline_before"] = json!(OTHER_HASH)
            })),
            Some(baseline),
            Some(candidate),
            "baseline",
            "not the previous release's binary before the baseline",
        ),
        (
            "the file at the path was not the previous release's after the baseline",
            Some(facts_with(&|f: &mut Value| {
                f["at_baked_path"]["baseline_after"] = json!(NEW_HASH)
            })),
            Some(baseline),
            Some(candidate),
            "baseline",
            "not the previous release's binary after the baseline",
        ),
        (
            "the baseline left no report",
            Some(good.clone()),
            None,
            Some(candidate),
            "baseline",
            "no readable report `baseline-junit.xml`",
        ),
        (
            "the baseline's report is cut off",
            Some(good.clone()),
            Some(&baseline[..baseline.len() / 2]),
            Some(candidate),
            "baseline",
            "no readable report `baseline-junit.xml`",
        ),
        (
            "the baseline's report names a test twice",
            Some(good.clone()),
            Some(&twice),
            Some(&twice),
            "baseline",
            "reported twice",
        ),
        (
            "the baseline ran no test",
            Some(good.clone()),
            Some("<testsuites name=\"nextest-run\" tests=\"0\"/>\n"),
            Some("<testsuites name=\"nextest-run\" tests=\"0\"/>\n"),
            "baseline",
            "the baseline ran no test",
        ),
        // The second run.
        (
            "the candidate's run left no report",
            Some(good.clone()),
            Some(baseline),
            None,
            "incomplete",
            "no readable report `candidate-junit.xml`",
        ),
        (
            "the candidate's run stopped early",
            Some(good.clone()),
            Some(baseline),
            Some(&short),
            "incomplete",
            "39 are in one report only",
        ),
        (
            "the candidate's run ran a test the baseline did not",
            Some(good.clone()),
            Some(baseline),
            Some(&longer),
            "incomplete",
            "`jigc::g_flow` `setup::a_test_only_this_run_ran`",
        ),
        // The record of the run.
        (
            "there is no run.json",
            None,
            Some(baseline),
            Some(candidate),
            "evidence",
            "no readable `run.json`",
        ),
        (
            "run.json is cut off",
            Some(good[..good.len() / 2].to_owned()),
            Some(baseline),
            Some(candidate),
            "evidence",
            "no readable `run.json`",
        ),
        (
            "run.json is no object",
            Some("[]".to_owned()),
            Some(baseline),
            Some(candidate),
            "evidence",
            "`run.json` is not an object",
        ),
        (
            "no hash was taken after the candidate's run",
            Some(facts_with(&|f: &mut Value| {
                f["at_baked_path"]
                    .as_object_mut()
                    .expect("an object")
                    .remove("candidate_after");
            })),
            Some(baseline),
            Some(candidate),
            "evidence",
            "`run.json` has no `at_baked_path.candidate_after`",
        ),
        (
            "a hash is not a string",
            Some(facts_with(&|f: &mut Value| {
                f["at_baked_path"]["candidate_after"] = json!(7)
            })),
            Some(baseline),
            Some(candidate),
            "evidence",
            "`run.json` has no `at_baked_path.candidate_after`",
        ),
        (
            "the count of test binaries naming the path is no number",
            Some(facts_with(&|f: &mut Value| f["named_by"] = json!("twelve"))),
            Some(baseline),
            Some(candidate),
            "evidence",
            "`run.json` has no `named_by`",
        ),
        // A step the run recorded as failed — the reports beside it do not rescue it.
        (
            "a tool was missing",
            Some(recorded("did-not-run")),
            Some(baseline),
            Some(candidate),
            "did-not-run",
            "exited 101",
        ),
        (
            "the previous release did not build",
            Some(recorded("build-previous")),
            Some(baseline),
            Some(candidate),
            "build-previous",
            "exited 101",
        ),
        (
            "the candidate did not build",
            Some(recorded("build-candidate")),
            Some(baseline),
            Some(candidate),
            "build-candidate",
            "exited 101",
        ),
        (
            "a failure of no known kind",
            Some(recorded("weather")),
            Some(baseline),
            Some(candidate),
            "evidence",
            "a failure of no known kind",
        ),
    ];

    for (what, facts, baseline, candidate, reason, saying) in rows {
        // A cut-off or non-object `run.json` names no commits, so no list is read at all.
        let evidence = rig.evidence(facts.as_deref(), baseline, candidate);
        let called = rig.verdict_over(&evidence);
        assert_eq!(
            (called.code, called.line["reason"].as_str()),
            (status_of(reason), Some(reason)),
            "{what}: void `{reason}`\nline: {}",
            called.raw
        );
        called.void(reason, saying);
    }
}

/// A fragment of a real report — cargo-nextest 0.9.143, the run of 2026-10-06 that the
/// tool was built from — with its attributes and its child elements as written.
#[test]
fn a_report_is_read_as_the_runner_writes_it() {
    const REAL: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="nextest-run" tests="3" skipped="0" failures="1" errors="0" uuid="8213d863-990e-49bd-8dab-6ae98e14538b" timestamp="2026-10-06T12:25:14.277+02:00" time="589.012">
    <testsuite name="jigc::g_config" tests="2" skipped="0" errors="0" failures="1">
        <testcase name="placement_override::the_store_conformance_sweep_adjudicates_at_the_overridden_home" classname="jigc::g_config" timestamp="2026-10-06T12:25:38.820+02:00" time="1.395"/>
        <testcase name="flow19_planning_encode::flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize" classname="jigc::g_config" timestamp="2026-10-06T12:25:29.361+02:00" time="11.173">
            <failure message="thread &apos;flow19_planning_encode::flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize&apos; (85057626) panicked at crates/cli/tests/groups/../flow19_planning_encode.rs:794:5" type="test failure with exit code 101">thread &apos;flow19_planning_encode::flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize&apos; (85057626) panicked at crates/cli/tests/groups/../flow19_planning_encode.rs:794:5:
a DRIFTED+TOUCHED singleton must conflict-block finalize non-zero</failure>
            <system-out>
running 1 test
test flow19_planning_encode::flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize ... FAILED
</system-out>
            <system-err>thread panicked</system-err>
        </testcase>
    </testsuite>
    <testsuite name="jigc-engine" tests="1" skipped="0" errors="0" failures="0">
        <testcase name="write::splice_prop_tests::set_slot_into_decision_is_byte_stable" classname="jigc-engine" timestamp="2026-10-06T12:34:10.098+02:00" time="0.308"/>
    </testsuite>
</testsuites>
"#;
    let rig = Rig::new();
    let baseline = REAL
        .split("<failure")
        .next()
        .expect("the report up to its failure")
        .to_owned()
        + "</testcase>\n    </testsuite>\n    <testsuite name=\"jigc-engine\">\n        <testcase name=\"write::splice_prop_tests::set_slot_into_decision_is_byte_stable\" classname=\"jigc-engine\" time=\"0.308\"/>\n    </testsuite>\n</testsuites>\n";
    let evidence = rig.evidence(Some(&rig.facts().to_string()), Some(&baseline), Some(REAL));
    let called = rig.verdict_over(&evidence);
    let line = called.verdict("red");
    assert_eq!(line["counts"]["tests"], 3, "{}", called.raw);
    assert_eq!(
        pairs(&line["not_on_list"]),
        [
            "jigc::g_config flow19_planning_encode::flow19_warm_append_over_an_oob_drifted_singleton_conflict_blocks_at_finalize"
        ]
    );
}

// ---------------------------------------------------------------------------
// The list of intended changes
// ---------------------------------------------------------------------------

#[test]
fn a_list_in_its_form_is_read() {
    let rig = Rig::new();
    let text = format!(
        "# Intended changes since 1.0.0 — one row per test, four fields, one tab between each.\n\
         \n\
         jigc::g_flow\tsetup::refuses_a_seeded_file\t`setup` refuses a file it did not write — exit 0 → 3\tcommit:{}\n\
         # a comment between two rows\n\
         jigc::g_migrate\tuninstall::says_dropped\tthe ack says dropped\truling:DECISIONS.md#2026-10-06 — The cap is ruled\n",
        rig.intended
    );
    let called = rig.check_list(&text);
    assert_eq!(
        (called.code, called.line["status"].as_str()),
        (0, Some("listed")),
        "{}",
        called.raw
    );
    assert_eq!(
        called.line["list"]["rows"],
        json!([
            {
                "binary": "jigc::g_flow",
                "test": "setup::refuses_a_seeded_file",
                "change": "`setup` refuses a file it did not write — exit 0 → 3",
                "pointer": format!("commit:{}", rig.intended),
            },
            {
                "binary": "jigc::g_migrate",
                "test": "uninstall::says_dropped",
                "change": "the ack says dropped",
                "pointer": "ruling:DECISIONS.md#2026-10-06 — The cap is ruled",
            },
        ])
    );
    assert_eq!(
        (&called.line["list"]["path"], &called.line["list"]["commit"]),
        (&json!(LIST), &json!(rig.candidate_with(&text))),
        "and the line says where it was read: the path, in the candidate's commit"
    );
    assert_eq!(called.line["list"]["sha256"], sha256_of(text.as_bytes()));
    // The newest commit a row can point at is the one the list's own was made on.
    let newest = format!("jigc\tt::a\tx\tcommit:{}\n", rig.candidate);
    assert_eq!(rig.check_list(&newest).code, 0);
}

/// One row per way a list is not in its form or points at nothing: each refuses the whole
/// list, naming the line.
#[test]
fn a_list_out_of_its_form_or_pointing_at_nothing_is_refused() {
    let rig = Rig::new();
    let good = rig.row("jigc::g_flow", "setup::refuses_a_seeded_file");
    let pointing =
        |pointer: &str| format!("jigc::g_flow\tsetup::refuses\tit refuses now\t{pointer}\n");
    let commit = |sha: &str| pointing(&format!("commit:{sha}"));
    let rows: Vec<(&str, String, &str)> = vec![
        // The form.
        (
            "three fields",
            "jigc::g_flow\tsetup::refuses\tit refuses now\n".into(),
            "line 1 has 3 fields",
        ),
        (
            "five fields",
            format!("{}\tone more\n", good.trim_end()),
            "line 1 has 5 fields",
        ),
        (
            "spaces for tabs",
            good.replace('\t', "    "),
            "line 1 has 1 fields",
        ),
        (
            "no test",
            commit(&rig.intended).replace("setup::refuses", ""),
            "line 1: `test` is empty",
        ),
        (
            "no change",
            commit(&rig.intended).replace("it refuses now", ""),
            "line 1: `change` is empty",
        ),
        (
            "no pointer",
            "jigc::g_flow\tsetup::refuses\tit refuses now\t\n".into(),
            "line 1: `pointer` is empty",
        ),
        (
            "space around a field",
            commit(&rig.intended).replace("jigc::g_flow", " jigc::g_flow"),
            "line 1: `binary` is empty or has space around it",
        ),
        (
            "a carriage return",
            good.replace('\n', "\r\n"),
            "line 1: `pointer` is empty or has space around it",
        ),
        (
            "a line of spaces",
            format!("{good}   \n"),
            "line 2 has 1 fields",
        ),
        (
            "a test listed twice",
            format!("{good}# between\n{good}"),
            "line 3 lists a test that line 1 lists already",
        ),
        // A pointer that is none.
        (
            "a bare sha",
            pointing(&rig.intended),
            "a pointer is `commit:<sha>`, `ruling:<path>#<heading>` or `excluded:<path>`",
        ),
        (
            "another kind of pointer",
            pointing("ticket:TKT-1"),
            "a pointer is `commit:<sha>`, `ruling:<path>#<heading>` or `excluded:<path>`",
        ),
        (
            "a short sha",
            commit(&rig.intended[..12]),
            "a commit pointer is `commit:` and a full sha",
        ),
        (
            "a ruling without a heading",
            pointing("ruling:DECISIONS.md"),
            "a ruling pointer is",
        ),
        (
            "a ruling with an empty heading",
            pointing("ruling:DECISIONS.md#"),
            "a ruling pointer is",
        ),
        (
            "a ruling outside the repository",
            pointing("ruling:../DECISIONS.md#The cap"),
            "a ruling pointer is",
        ),
        (
            "a ruling by an absolute path",
            pointing("ruling:/etc/hosts#The cap"),
            "a ruling pointer is",
        ),
        // A pointer at nothing.
        (
            "a sha that is no commit",
            commit(&"0".repeat(40)),
            "line 1 points at nothing: `0000000000000000000000000000000000000000` is no commit",
        ),
        (
            "a commit after the candidate",
            commit(&rig.later),
            "line 1 points at nothing: the candidate does not hold",
        ),
        (
            "the previous release's own commit",
            commit(&rig.previous),
            "line 1 points at nothing: the previous release holds",
        ),
        (
            "a commit before the previous release",
            commit(&rig.before),
            "line 1 points at nothing: the previous release holds",
        ),
        (
            "a ruling in a file the candidate lacks",
            pointing("ruling:later.md#after"),
            "line 1 points at nothing: the candidate has no file `later.md`",
        ),
        (
            "a heading nothing opens with",
            pointing("ruling:DECISIONS.md#The cap is not ruled"),
            "line 1 points at nothing: 0 headings",
        ),
        (
            "words of the prose, not of a heading",
            pointing("ruling:DECISIONS.md#The cap is ruled in prose"),
            "line 1 points at nothing: 0 headings",
        ),
        (
            "words a heading holds and does not open with",
            pointing("ruling:DECISIONS.md#The cap is ruled: ten thousand"),
            "line 1 points at nothing: 0 headings",
        ),
        (
            "a heading two lines open with",
            pointing("ruling:DECISIONS.md#Twice"),
            "line 1 points at more than one: 2 headings",
        ),
        // The second row is held as the first is.
        (
            "a bad row after a good one",
            format!("{good}{}", commit(&rig.later)),
            "line 2 points at nothing",
        ),
    ];
    for (what, text, saying) in rows {
        let called = rig.check_list(&text);
        assert_eq!(
            called.code,
            status_of("list"),
            "{what}: refused\nline: {}",
            called.raw
        );
        called.refused("list", saying);
    }

    let called = rig.check_list([b'j', 0xff, b'\n']);
    called.refused("list", "is not UTF-8");
}

/// **An exclusion row points at the record of a run that showed the test failing on its
/// own binary**: a file of the candidate's commit that is a line of this tool, of a run
/// against the same previous release, which names the test among those that failed there.
/// Anything less is a row that points at nothing, and the whole list is refused.
#[test]
fn an_exclusion_row_points_at_the_line_of_a_run_that_showed_the_test_failing() {
    let rig = Rig::new();
    // In its form it is read, as a row of the same list. The record names the first test
    // as a line names one that had its row, the second as one that had none.
    let text = format!(
        "{}{}{}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"),
        rig.excluding("jigc::g_flow", "fences::reads_the_git_history"),
        rig.excluding("jigc::g_flow", "discard::names_its_commit"),
    );
    let called = rig.check_list(&text);
    assert_eq!(
        (called.code, called.line["status"].as_str()),
        (0, Some("listed")),
        "{}",
        called.raw
    );
    assert_eq!(
        called.line["list"]["rows"][1],
        json!({
            "binary": "jigc::g_flow",
            "test": "fences::reads_the_git_history",
            "change": NO_REPOSITORY,
            "pointer": format!("excluded:{RECORD}"),
        })
    );
    assert_eq!(
        called.line["list"]["rows"][2]["test"],
        "discard::names_its_commit"
    );

    const RUN: &str = "record/run.json";
    let pointing = |pointer: &str| {
        format!("jigc::g_flow\tfences::reads_the_git_history\t{NO_REPOSITORY}\t{pointer}\n")
    };
    let check = |record: Option<&str>, pointer: &str| {
        let list = pointing(pointer);
        let mut files = vec![(LIST, list.as_bytes())];
        files.extend(record.map(|record| (RUN, record.as_bytes())));
        let candidate = rig.candidate_holding(&files);
        rig.tool(&[
            "check-list",
            "--previous",
            &rig.previous,
            "--candidate",
            &candidate,
            "--list",
            LIST,
        ])
    };
    let naming = |binary: &str, test: &str| {
        json!({
            "tool": "regression-set",
            "status": "green",
            "previous": {"commit": rig.previous, "sha256": OLD_HASH},
            "excluded": [{"binary": binary, "test": test}],
        })
        .to_string()
    };
    let good = naming("jigc::g_flow", "fences::reads_the_git_history");
    let at_run = format!("excluded:{RUN}");
    let at_run = at_run.as_str();
    assert_eq!(check(Some(&good), at_run).code, 0, "the control: read");

    let rows: Vec<(&str, Option<String>, &str, &str)> = vec![
        // A pointer that is none.
        (
            "no path",
            Some(good.clone()),
            "excluded:",
            "an exclusion pointer is `excluded:<path in the repository>`",
        ),
        (
            "a path that leaves the repository",
            Some(good.clone()),
            "excluded:../record/run.json",
            "an exclusion pointer is",
        ),
        (
            "an absolute path",
            Some(good.clone()),
            "excluded:/etc/hosts",
            "an exclusion pointer is",
        ),
        // A pointer at nothing.
        (
            "a file the candidate lacks",
            None,
            at_run,
            "line 1 points at nothing: the candidate has no file `record/run.json`",
        ),
        (
            "a file that is no JSON",
            Some("# The record of a run\n".to_owned()),
            at_run,
            "is not the line of a run of this tool",
        ),
        (
            "JSON that is no object",
            Some("[]".to_owned()),
            at_run,
            "is not the line of a run of this tool",
        ),
        (
            "the line of another tool",
            Some(good.replace("regression-set", "gate")),
            at_run,
            "is not the line of a run of this tool",
        ),
        (
            "a line that names no previous release",
            Some(json!({"tool": "regression-set", "excluded": []}).to_string()),
            at_run,
            "is not the line of a run of this tool",
        ),
        (
            "a line whose tests are bare names",
            Some(
                json!({
                    "tool": "regression-set",
                    "previous": {"commit": rig.previous},
                    "excluded": ["fences::reads_the_git_history"],
                })
                .to_string(),
            ),
            at_run,
            "is not the line of a run of this tool",
        ),
        (
            "a run against another release",
            Some(good.replace(&rig.previous, &rig.before)),
            at_run,
            "is the line of a run against",
        ),
        (
            "a line that names another test",
            Some(naming("jigc::g_flow", "fences::another")),
            at_run,
            "does not name `jigc::g_flow` `fences::reads_the_git_history`",
        ),
        (
            "the same name in another binary",
            Some(naming("jigc::g_item", "fences::reads_the_git_history")),
            at_run,
            "does not name `jigc::g_flow` `fences::reads_the_git_history`",
        ),
        (
            "a line in which no test failed on its own binary",
            Some(
                json!({"tool": "regression-set", "previous": {"commit": rig.previous}}).to_string(),
            ),
            at_run,
            "does not name",
        ),
    ];
    for (what, record, pointer, saying) in rows {
        let called = check(record.as_deref(), pointer);
        assert_eq!(
            called.code,
            status_of("list"),
            "{what}: refused\nline: {}",
            called.raw
        );
        called.refused("list", saying);
    }
}

/// **What a green rests on is a fact of the candidate's commit** (the re-review's `R-M6`:
/// the first green on record was given over a file no commit held). The list is named by
/// its path in the repository and read out of the candidate's commit — never from the
/// working tree, and never from a file somewhere else.
#[test]
fn a_list_that_is_not_in_the_candidates_commit_is_refused() {
    let rig = Rig::new();
    let unruled = rig.row("jigc::g_flow", "some_suite::a_test_a_fixer_broke");
    let check = |candidate: &str, list: &str| {
        // Called from the repository's root, where a path in the repository names the
        // working tree's file too: what a caller's shell would hand a tool that opens it.
        rig.tool_as(
            &[
                "check-list",
                "--previous",
                &rig.previous,
                "--candidate",
                candidate,
                "--list",
                list,
            ],
            |command| {
                command.current_dir(&rig.root);
            },
        )
    };

    // The candidate holds an empty list. The working tree's file at the same path is then
    // given a row nobody committed: the list that is read is the commit's.
    let candidate = rig.candidate_with("");
    fs::write(rig.root.join(LIST), &unruled).expect("write the working tree's file");
    let called = check(&candidate, LIST);
    assert_eq!(
        (called.code, called.line["status"].as_str()),
        (0, Some("listed")),
        "{}",
        called.raw
    );
    assert_eq!(
        (&called.line["list"]["rows"], &called.line["list"]["sha256"]),
        (&json!([]), &json!(sha256_of(b""))),
        "the list is the candidate's commit's, whatever the working tree has at the path: {}",
        called.raw
    );

    // A candidate that has no file at the path, while the working tree still has one.
    let called = check(&rig.candidate, LIST);
    called.refused("list", "the candidate's commit holds no file");

    // A file no commit holds, named as a caller's shell would name it.
    let outside = rig.file_outside(&unruled);
    for (what, named) in [
        ("an absolute path", outside.as_str()),
        (
            "a path that leaves the repository",
            "../intended-changes.tsv",
        ),
        (
            "a path with an empty segment",
            "record//intended-changes.tsv",
        ),
    ] {
        let called = check(&candidate, named);
        assert_eq!(called.code, status_of("list"), "{what}: {}", called.raw);
        called.refused("list", "is named by its path in the repository");
    }

    // A directory of the commit is no list, and neither is a path the commit lacks.
    check(&candidate, "record").refused("list", "the candidate's commit holds no file");
    check(&candidate, "no/such/list.tsv").refused("list", "the candidate's commit holds no file");
}

/// **A verdict is given over the list its run read, or it is void — never green.** A run
/// records the list's path and hash; evidence that names another list than the one the
/// verdict is asked over establishes nothing about it.
#[test]
fn a_verdict_over_another_list_than_the_run_read_is_void_and_never_green() {
    let rig = Rig::new();
    let suite = [
        ran("jigc-engine", "unit::holds", Pass, Pass),
        ran("jigc::g_flow", "setup::refuses_a_seeded_file", Pass, Fail),
    ];
    let row = rig.row("jigc::g_flow", "setup::refuses_a_seeded_file");
    const OTHER: &str = "record/another-list.tsv";
    let held = rig.facts();
    // The candidate holds two lists: an empty one, and one with the row that would make
    // the run green. It is the commit checked out, so the working tree has both too.
    let candidate = rig.candidate_holding(&[(LIST, b""), (OTHER, row.as_bytes())]);
    let facts_naming = |path: &str, sha256: String| {
        let mut facts = held.clone();
        facts["candidate"]["commit"] = json!(candidate);
        facts["list"] = json!({"path": path, "sha256": sha256});
        facts
    };
    let verdict = |facts: &Value, list: &str| {
        let (baseline, candidate) = reports(&suite);
        let evidence = rig.evidence(Some(&facts.to_string()), Some(&baseline), Some(&candidate));
        // From the repository's root, as in the arm above.
        rig.tool_as(
            &["verdict", "--evidence", &evidence, "--list", list],
            |command| {
                command.current_dir(&rig.root);
            },
        )
    };

    // The run read the empty list; the verdict is asked over the one with the row.
    let read_the_empty_one = facts_naming(LIST, sha256_of(b""));
    let line = verdict(&read_the_empty_one, OTHER)
        .void(
            "evidence",
            "the run read the list `record/intended-changes.tsv`",
        )
        .clone();
    assert_eq!(
        line["list"],
        json!({
            "path": OTHER,
            "commit": candidate,
            "sha256": sha256_of(row.as_bytes()),
            "rows": 1,
        }),
        "the void line names the list it was asked over"
    );
    // The control: over the list the run read, the difference is unlisted — red.
    verdict(&read_the_empty_one, LIST).verdict("red");

    // The path is the same and the bytes are not: the evidence names a hash the commit's
    // file does not have.
    let another_hash = facts_naming(OTHER, sha256_of(b"another list\n"));
    verdict(&another_hash, OTHER).void("evidence", "the run read the list");
    // The control: the same evidence naming the hash of the file is green.
    verdict(&facts_naming(OTHER, sha256_of(row.as_bytes())), OTHER).verdict("green");

    // Evidence that does not say which list its run read — a run of the tool before it
    // recorded one — cannot be told from either.
    let mut silent = read_the_empty_one.clone();
    silent.as_object_mut().expect("an object").remove("list");
    verdict(&silent, LIST).void("evidence", "`run.json` has no `list.path`");
}

/// Two builds of one commit differ in bytes wherever a build embeds its path, so no hash
/// refuses them (the re-review's `R-M6`): the two commits are held apart before anything
/// is read or built.
#[test]
fn two_commits_that_are_one_are_refused() {
    let rig = Rig::new();
    // A list no row of which can be wrong, at a path every form of the call can open.
    let outside = rig.file_outside("");
    let candidate = rig.candidate_with("");
    let scratch = rig.scratch.to_str().expect("a UTF-8 path");
    let saying = "the candidate's commit is the previous release's";
    for list in [outside.as_str(), LIST] {
        for commit in [rig.previous.as_str(), candidate.as_str()] {
            let named = ["--previous", commit, "--candidate", commit, "--list", list];
            let called = rig.tool(&[&["check-list"], &named[..]].concat());
            assert_eq!(called.code, status_of("commit"), "{}", called.raw);
            called.refused("commit", saying);
            let called = rig.tool(&[&["run"], &named[..], &["--scratch", scratch]].concat());
            assert_eq!(called.code, status_of("commit"), "{}", called.raw);
            called.refused("commit", saying);
        }
    }
    assert_eq!(
        fs::read_dir(&rig.scratch)
            .expect("read the scratch directory")
            .count(),
        0,
        "a refused run mints nothing"
    );

    // Evidence of such a run is refused the same way.
    let mut facts = rig.facts();
    facts["previous"]["commit"] = facts["candidate"]["commit"].clone();
    rig.verdict_with(&facts, &quiet(3))
        .refused("commit", saying);
}

/// The list is held to the commits the evidence names: a row that points outside them is
/// refused by `verdict` as by `run`.
#[test]
fn a_verdict_is_given_over_no_list_that_is_refused() {
    let rig = Rig::new();
    let list = format!(
        "jigc::g_flow\tsetup::refuses\tit refuses now\tcommit:{}\n",
        rig.later
    );
    rig.verdict(&quiet(3), &list)
        .refused("list", "the candidate does not hold");
}

// ---------------------------------------------------------------------------
// Refusals before anything is read
// ---------------------------------------------------------------------------

#[test]
fn what_a_call_names_wrongly_is_refused_with_its_word() {
    let rig = Rig::new();
    let candidate = rig.candidate_with("");
    let list = LIST;
    let scratch = rig.scratch.to_str().expect("a UTF-8 path");
    let inside = rig.root.join("dev");
    fs::create_dir_all(&inside).expect("a directory inside the repository");
    let (previous, candidate) = (rig.previous.as_str(), candidate.as_str());
    let short = &rig.previous[..12];
    let nowhere = "f".repeat(40);
    let absent = format!("{scratch}/absent");
    let inside = inside.to_str().expect("a UTF-8 path");
    let repo = rig.root.to_str().expect("a UTF-8 path");
    let kept_inside = format!("{repo}/kept");
    let run = |previous, candidate, scratch| {
        vec![
            "run",
            "--previous",
            previous,
            "--candidate",
            candidate,
            "--list",
            list,
            "--scratch",
            scratch,
        ]
    };
    let rows: Vec<(Vec<&str>, &str, &str)> = vec![
        (vec![], "usage", "act"),
        (vec!["compare"], "usage", "invalid choice"),
        (
            vec![
                "run",
                "--previous",
                previous,
                "--candidate",
                candidate,
                "--list",
                list,
            ],
            "usage",
            "--scratch",
        ),
        (vec!["verdict", "--list", list], "usage", "--evidence"),
        (
            vec![
                "check-list",
                "--previous",
                previous,
                "--candidate",
                candidate,
                "--list",
                list,
                "--force",
            ],
            "usage",
            "--force",
        ),
        (
            run(short, candidate, scratch),
            "commit",
            "the previous release's commit",
        ),
        (
            run(previous, "trunk", scratch),
            "commit",
            "the candidate's commit `trunk` is not a full commit sha",
        ),
        (
            run(previous, &nowhere, scratch),
            "commit",
            "is not a commit this repository holds",
        ),
        (
            run(previous, candidate, &absent),
            "scratch",
            "does not exist",
        ),
        (
            run(previous, candidate, inside),
            "scratch",
            "lies inside the repository",
        ),
        (
            run(previous, candidate, repo),
            "scratch",
            "lies inside the repository",
        ),
        (
            [run(previous, candidate, scratch), vec!["--baseline"]].concat(),
            "usage",
            "--baseline",
        ),
        (
            [
                run(previous, candidate, scratch),
                vec!["--baseline", &kept_inside],
            ]
            .concat(),
            "scratch",
            "the directory of the kept baseline",
        ),
    ];
    for (args, word, saying) in rows {
        let called = rig.tool(&args);
        assert_eq!(
            called.code,
            status_of(word),
            "{args:?}: refused `{word}`\n{}",
            called.raw
        );
        called.refused(word, saying);
    }
    assert_eq!(
        fs::read_dir(&rig.scratch)
            .expect("read the scratch directory")
            .count(),
        0,
        "a refused run mints nothing"
    );
    assert!(
        !Path::new(&kept_inside).exists(),
        "and makes no directory for a baseline it refused to keep there"
    );
}

// ---------------------------------------------------------------------------
// The run, with a stand-in for cargo
// ---------------------------------------------------------------------------

/// A stand-in for cargo that builds nothing and runs no test.
///
/// *A build* writes a file: the previous release's "binary" at `debug/deps/jigc-…` and,
/// uplifted, at `debug/jigc`; a "test binary" whose bytes hold that path; the candidate's
/// at its own target. *The recorded build* names them as cargo-nextest does.
///
/// *A run* is where it is faithful to what was measured. **A run that is not made from
/// the recorded build copies `deps/jigc-…` back over `debug/jigc`** — cargo finds the unit
/// fresh and uplifts it again. A run that is made from it reads the target directory out
/// of the record it is handed, as the runner does: on a kept baseline this call built
/// nothing. Then the report: **chosen by the bytes at the path**,
/// `on-previous.xml` or `on-candidate.xml` of the directory `FAKE_CARGO` names, written
/// where the tool config's store says — which it reads as a strict string, so a path the
/// tool did not escape is a run with no report.
///
/// `FAKE_CARGO_MODE` makes one thing go wrong: `fail-previous`, `fail-candidate`,
/// `same-binary`, `unnamed`, `disturbed` — another file at the path from the candidate's
/// build until the baseline's run puts the right one back — `put-back` — the previous
/// release's binary copied back during the candidate's run, whatever path the run took —
/// and `cut-short` — THE BASELINE'S RUNNER SIGNALLED WHILE IT RAN, as the real one answers
/// it (cargo-nextest 0.9.143, sent SIGTERM and SIGINT two tests into six on 2026-10-13: a
/// report of the two tests that had run, neither failed, and exit status 100; killed
/// outright it leaves no report at all): the report holds the tests of the first binary
/// only, none of them failed, and the exit status is 100.
///
/// *Asked what the runner is* it says what `FAKE_RUNNER` holds — a call only a run that
/// names a kept baseline makes.
const FAKE_CARGO: &str = r#"#!/usr/bin/env python3
import json, os, shutil, sys

state = os.environ["FAKE_CARGO"]
mode = os.environ.get("FAKE_CARGO_MODE", "").split(",")
argv = sys.argv[1:]
with open(os.path.join(state, "calls"), "a") as calls:
    calls.write(" ".join(argv) + "\n")

PREVIOUS = b"the previous release's binary\n"
CANDIDATE = b"the candidate's binary\n"


def flag(name):
    return argv[argv.index(name) + 1]


def target():
    return os.environ["CARGO_TARGET_DIR"]


def built():
    return open(os.path.join(state, "target")).read()


def fail(what):
    sys.stderr.write("error: could not compile `%s`\n" % what)
    sys.exit(101)


if argv[:2] == ["nextest", "run"] and "--no-run" in argv:
    if "fail-previous" in mode:
        fail("jigc (test)")
    deps = os.path.join(target(), "debug", "deps")
    os.makedirs(deps)
    with open(os.path.join(deps, "jigc-0f0f"), "wb") as binary:
        binary.write(PREVIOUS)
    shutil.copyfile(os.path.join(deps, "jigc-0f0f"), os.path.join(target(), "debug", "jigc"))
    names = b"" if "unnamed" in mode else os.fsencode(os.path.join(target(), "debug", "jigc"))
    with open(os.path.join(deps, "g_flow-1a1a"), "wb") as test:
        test.write(b"\x7fELF a test binary that runs " + names + b"\x00")
    with open(os.path.join(state, "target"), "w") as remembered:
        remembered.write(target())
elif argv[:2] == ["nextest", "list"]:
    print(json.dumps({
        "rust-build-meta": {
            "target-directory": built(),
            "non-test-binaries": {
                "path+file:///work/previous/crates/cli#jigc@1.0.0": [
                    {"name": "jigc", "kind": "bin-exe", "path": "debug/jigc"},
                    {"name": "jigc", "kind": "dylib", "path": "debug/libjigc.dylib"},
                ],
                "path+file:///work/previous/crates/other#other@1.0.0": [
                    {"name": "other", "kind": "bin-exe", "path": "debug/other"},
                ],
            },
        },
        "rust-binaries": {
            "jigc::g_flow": {
                "binary-id": "jigc::g_flow",
                "binary-path": os.path.join(built(), "debug", "deps", "g_flow-1a1a"),
            },
        },
    }))
elif argv[:1] == ["metadata"]:
    print("{}")
elif argv == ["nextest", "--version"]:
    print(os.environ["FAKE_RUNNER"])
elif argv[:1] == ["build"]:
    if "fail-candidate" in mode:
        fail("jigc")
    os.makedirs(os.path.join(target(), "debug"))
    with open(os.path.join(target(), "debug", "jigc"), "wb") as binary:
        binary.write(PREVIOUS if "same-binary" in mode else CANDIDATE)
    if "disturbed" in mode:
        with open(os.path.join(built(), "debug", "jigc"), "wb") as binary:
            binary.write(b"something else\n")
elif argv[:2] == ["nextest", "run"]:
    if "--binaries-metadata" in argv:
        with open(flag("--binaries-metadata")) as handed:
            recorded = json.load(handed)["rust-build-meta"]["target-directory"]
    else:
        recorded = built()
    baked = os.path.join(recorded, "debug", "jigc")
    put_back = "put-back" in mode and flag("--profile") == "regression-candidate"
    if "--binaries-metadata" not in argv or put_back or "disturbed" in mode:
        shutil.copyfile(os.path.join(recorded, "debug", "deps", "jigc-0f0f"), baked)
    store = None
    for line in open(flag("--tool-config-file").split(":", 1)[1]):
        if line.startswith("dir = "):
            store = json.loads(line[len("dir = "):])
    drove = "previous" if open(baked, "rb").read() == PREVIOUS else "candidate"
    report = os.path.join(state, "on-%s.xml" % drove)
    out = os.path.join(store, flag("--profile"))
    os.makedirs(out)
    if "cut-short" in mode and flag("--profile") == "regression-baseline":
        whole = open(report).read()
        end = whole.index("</testsuite>") + len("</testsuite>")
        with open(os.path.join(out, "junit.xml"), "w") as cut:
            cut.write(whole[:end] + "\n</testsuites>\n")
        sys.exit(100)
    shutil.copyfile(report, os.path.join(out, "junit.xml"))
    sys.exit(100 if b"<failure" in open(report, "rb").read() else 0)
else:
    sys.stderr.write("the stand-in does not know `cargo %s`\n" % " ".join(argv))
    sys.exit(2)
"#;

/// What a run of the tool under the stand-in left.
struct Driven {
    called: Called,
    /// The stand-in's calls, in order: each `cargo` command line without `cargo`.
    calls: Vec<String>,
}

/// A stand-in for the compiler: it says what `FAKE_RUSTC` holds, and writes down beside
/// cargo's calls that it was asked, and in which directory — only a run that names a kept
/// baseline asks, and it asks in the previous release's tree, where that tree's toolchain
/// file is what resolves the compiler.
const FAKE_RUSTC: &str =
    "#!/bin/sh\necho \"rustc $* in ${PWD##*/}\" >> \"$FAKE_CARGO/calls\"\necho \"$FAKE_RUSTC\"\n";

/// What the two stand-ins say of themselves unless an arm says otherwise.
const RUNNER: &str = "cargo-nextest 0.9.143 (60fa45f63 2026-08-04)";
const COMPILER: &str = "rustc 1.95.0 (59807616e 2026-04-14)";

/// How one call of `run` under the stand-ins is made.
struct Asked {
    previous: String,
    list: String,
    mode: String,
    more: Vec<String>,
    runner: String,
    compiler: String,
}

impl Rig {
    /// `run`, with the stand-in first on `PATH`, over a suite whose reports the stand-in
    /// hands out by the binary it finds at the path — the candidate the one that holds a
    /// list of this text.
    fn run(&self, suite: &[Ran], list: &str, mode: &str, more: &[&str]) -> Driven {
        self.ask(
            suite,
            &Asked {
                previous: self.previous.clone(),
                list: list.to_owned(),
                mode: mode.to_owned(),
                more: more.iter().map(|flag| (*flag).to_owned()).collect(),
                runner: RUNNER.to_owned(),
                compiler: COMPILER.to_owned(),
            },
        )
    }

    /// `run` as `asked`, over `suite`.
    fn ask(&self, suite: &[Ran], asked: &Asked) -> Driven {
        let bin = self.dir.path().join("bin");
        placed_executable::write(&bin.join("cargo"), FAKE_CARGO);
        placed_executable::write(&bin.join("cargo-nextest"), "#!/bin/sh\nexit 2\n");
        placed_executable::write(&bin.join("rustc"), FAKE_RUSTC);
        let state = self.dir.path().join("fake-cargo");
        let _ = fs::remove_dir_all(&state);
        fs::create_dir_all(&state).expect("create the stand-in's directory");
        let (on_previous, on_candidate) = reports(suite);
        fs::write(state.join("on-previous.xml"), on_previous).expect("write a report");
        fs::write(state.join("on-candidate.xml"), on_candidate).expect("write a report");
        let candidate = self.candidate_with(&asked.list);
        let mut args = vec![
            "run",
            "--previous",
            &asked.previous,
            "--candidate",
            &candidate,
            "--list",
            LIST,
            "--scratch",
            self.scratch.to_str().expect("a UTF-8 path"),
        ];
        args.extend(asked.more.iter().map(String::as_str));
        let called = self.tool_as(&args, |command| {
            command
                .env("FAKE_CARGO", &state)
                .env("FAKE_CARGO_MODE", &asked.mode)
                .env("FAKE_RUNNER", &asked.runner)
                .env("FAKE_RUSTC", &asked.compiler);
        });
        let calls = fs::read_to_string(state.join("calls"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect();
        Driven { called, calls }
    }
}

/// The two differences of a run under the stand-in, and the list that holds them.
fn a_run() -> Vec<Ran> {
    let mut suite = quiet(20);
    suite.extend([
        ran("jigc::g_flow", "setup::refuses_a_seeded_file", Pass, Fail),
        ran("jigc::g_migrate", "uninstall::says_dropped", Pass, Fail),
    ]);
    suite
}

fn both_rows(rig: &Rig) -> String {
    format!(
        "{}{}",
        rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"),
        rig.row("jigc::g_migrate", "uninstall::says_dropped"),
    )
}

fn sha256_of(bytes: &[u8]) -> String {
    let dir = ScratchDir::new("regression-set-hash");
    let file = dir.path().join("bytes");
    fs::write(&file, bytes).expect("write the bytes");
    let out = Command::new("python3")
        .args(["-c", "import hashlib, sys; sys.stdout.write(hashlib.sha256(open(sys.argv[1], 'rb').read()).hexdigest())"])
        .arg(&file)
        .output()
        .expect("spawn python3");
    assert!(out.status.success(), "python3 hashes the bytes");
    String::from_utf8(out.stdout).expect("a hex digest")
}

#[test]
fn a_run_builds_once_records_the_build_and_makes_both_runs_from_the_record() {
    let rig = Rig::new();
    let run = rig.run(&a_run(), &both_rows(&rig), "", &[]);
    let line = run.called.verdict("green");
    assert_eq!(line["counts"]["differences"], 2, "{}", run.called.raw);
    assert_eq!(line["counts"]["on_list"], 2);

    // Everything is done in one directory minted under the scratch directory.
    let work = PathBuf::from(line["work"].as_str().expect("the line names its directory"));
    assert_eq!(
        work.parent(),
        Some(rig.scratch.as_path()),
        "minted under the scratch directory"
    );
    let evidence = work.join("evidence");
    assert_eq!(line["evidence"].as_str(), evidence.to_str());

    // The order: one build of the tests, the build recorded, the candidate's binary, and
    // two runs — each made from the record, the second no differently from the first.
    let recorded = format!(
        "--binaries-metadata {0}/binaries-metadata.json --cargo-metadata {0}/cargo-metadata.json \
         --tool-config-file regression-set:{0}/nextest.toml",
        evidence.display()
    );
    assert_eq!(
        run.calls,
        [
            "nextest run --workspace --locked --no-run".to_owned(),
            "nextest list --workspace --locked --list-type binaries-only --message-format json"
                .to_owned(),
            "metadata --format-version=1 --all-features --locked".to_owned(),
            "build -p jigc --locked".to_owned(),
            format!(
                "nextest run --no-fail-fast --retries 1 {recorded} --profile regression-baseline"
            ),
            format!(
                "nextest run --no-fail-fast --retries 1 {recorded} --profile regression-candidate"
            ),
        ],
        "what the tool asked cargo for, in order"
    );

    // Which binary each run drove, as the line proves it.
    let old = sha256_of(b"the previous release's binary\n");
    let new = sha256_of(b"the candidate's binary\n");
    assert_eq!(
        line["previous"],
        json!({"commit": rig.previous, "sha256": old})
    );
    let candidate = rig.candidate_with(both_rows(&rig));
    assert_eq!(
        line["candidate"],
        json!({"commit": candidate, "sha256": new})
    );
    assert_eq!(
        line["proof"],
        json!({
            "baked_path": work.join("target-previous/debug/jigc"),
            "named_by": 1,
            "baseline_before": old,
            "baseline_after": old,
            "candidate_before": new,
            "candidate_after": new,
        })
    );

    // The evidence stays, and `verdict` gives the same verdict over it; what was built
    // does not.
    for kept in [
        "run.json",
        "baseline-junit.xml",
        "candidate-junit.xml",
        "binaries-metadata.json",
        "baseline.log",
    ] {
        assert!(evidence.join(kept).is_file(), "the evidence keeps `{kept}`");
    }
    for built in [
        "previous",
        "candidate",
        "target-previous",
        "target-candidate",
    ] {
        assert!(
            !work.join(built).exists(),
            "`{built}` is removed at the end of a run"
        );
    }
    let again = rig.verdict_over(evidence.to_str().expect("a UTF-8 path"));
    assert_eq!(again.verdict("green")["counts"], line["counts"]);

    // And what the runner exited with, each time: the baseline green, the second run not.
    let facts: Value = serde_json::from_str(
        &fs::read_to_string(evidence.join("run.json")).expect("read run.json"),
    )
    .expect("run.json is JSON");
    assert_eq!(facts["exits"], json!({"baseline": 0, "candidate": 100}));

    // Which list the run read is a fact of its evidence and of its line: the path, the
    // candidate's commit it was read out of, and the hash of its bytes.
    let hash = sha256_of(both_rows(&rig).as_bytes());
    assert_eq!(facts["list"], json!({"path": LIST, "sha256": hash}));
    assert_eq!(
        line["list"],
        json!({"path": LIST, "commit": candidate, "sha256": hash, "rows": 2})
    );
}

#[test]
fn a_run_keeps_what_it_built_when_asked_to() {
    let rig = Rig::new();
    let run = rig.run(&a_run(), "", "", &["--keep"]);
    let line = run.called.verdict("red");
    let work = PathBuf::from(line["work"].as_str().expect("the line names its directory"));
    // The trees are the commits', from `git archive`: the candidate's has the intended
    // change and the previous release's does not, and neither has what came later.
    assert!(work.join("candidate/src.rs").is_file());
    assert!(!work.join("previous/src.rs").exists());
    assert!(work.join("previous/Cargo.toml").is_file());
    assert!(!work.join("candidate/later.md").exists());
    assert_eq!(
        fs::read(work.join("target-previous/debug/jigc")).expect("the swapped file"),
        b"the candidate's binary\n",
        "the candidate's binary is at the path the tests name"
    );
}

/// **The trap, driven.** During the candidate's run the previous release's binary is
/// copied back — so the tests run on it, and the report the run leaves shows no
/// difference at all. The hash after the run is what says so.
#[test]
fn a_run_whose_binary_was_put_back_is_void() {
    let rig = Rig::new();
    let run = rig.run(&a_run(), &both_rows(&rig), "put-back", &[]);
    let line = run
        .called
        .void("swap", "the previous release's binary, put back");
    assert_eq!(
        line["proof"]["candidate_before"],
        sha256_of(b"the candidate's binary\n")
    );
    assert_eq!(
        line["proof"]["candidate_after"],
        sha256_of(b"the previous release's binary\n")
    );
    assert_eq!(run.calls.len(), 6, "both runs were made: {:?}", run.calls);
}

/// One row per step that ends a run before a verdict: the mode of the stand-in, the
/// reason, words of its sentence, and how many commands cargo was asked for.
#[test]
fn a_step_that_cannot_be_done_ends_the_run_there_void() {
    let rig = Rig::new();
    let rows = [
        (
            "fail-previous",
            "build-previous",
            "`cargo nextest run --workspace` exited 101",
            1,
        ),
        (
            "fail-candidate",
            "build-candidate",
            "`cargo build -p jigc` exited 101",
            4,
        ),
        (
            "same-binary",
            "swap",
            "byte for byte the previous release's",
            4,
        ),
        (
            "unnamed",
            "swap",
            "no test binary of the recorded build names the path",
            3,
        ),
        // The baseline's own report is green, and its binary is the right one afterwards.
        (
            "disturbed",
            "baseline",
            "not the previous release's binary before the baseline",
            5,
        ),
    ];
    for (mode, reason, saying, calls) in rows {
        let run = rig.run(&a_run(), "", mode, &[]);
        run.called.void(reason, saying);
        assert_eq!(
            run.calls.len(),
            calls,
            "{mode}: nothing runs after the step: {:?}",
            run.calls
        );
    }
}

/// A test that fails on its own binary and is on no row does not end the run where the
/// baseline ends: the second run is made, so that ONE run names every difference and every
/// such test — which is what the rows are then written from.
#[test]
fn a_run_names_what_no_row_excludes_after_both_runs_are_made() {
    let rig = Rig::new();
    let mut suite = a_run();
    suite.push(ran(
        "jigc::g_flow",
        "fences::reads_the_git_history",
        Fail,
        Fail,
    ));
    let asks_git = ["jigc::g_flow fences::reads_the_git_history"];

    // A difference is off the list too: red, and both are named.
    let one_row = rig.row("jigc::g_flow", "setup::refuses_a_seeded_file");
    let run = rig.run(&suite, &one_row, "", &[]);
    let line = run.called.verdict("red");
    assert_eq!(
        pairs(&line["not_on_list"]),
        ["jigc::g_migrate uninstall::says_dropped"]
    );
    assert_eq!(pairs(&line["excluded_by_no_row"]), asks_git);
    assert_eq!(run.calls.len(), 6, "both runs were made: {:?}", run.calls);

    // Every difference is listed: void, and the test is named.
    let run = rig.run(&suite, &both_rows(&rig), "", &[]);
    let line = run.called.void(
        "baseline",
        "1 of the 23 tests failed on their own binary and no exclusion row",
    );
    assert_eq!(pairs(&line["excluded_by_no_row"]), asks_git);
    assert_eq!(run.calls.len(), 6, "both runs were made: {:?}", run.calls);

    // With its row: green, and what is excluded is the row.
    let listed = format!(
        "{}{}",
        both_rows(&rig),
        rig.excluding("jigc::g_flow", "fences::reads_the_git_history")
    );
    let run = rig.run(&suite, &listed, "", &[]);
    let line = run.called.verdict("green");
    assert_eq!(pairs(&line["excluded"]), asks_git, "{}", run.called.raw);
    assert_eq!(line["excluded_by_no_row"], json!([]));
}

#[test]
fn a_run_without_cargo_did_not_run() {
    let rig = Rig::new();
    // A PATH that has what the tool itself needs and no cargo.
    let bin = rig.dir.path().join("no-cargo");
    fs::create_dir_all(&bin).expect("create the directory");
    for needed in ["python3", "git", "tar"] {
        let real = std::env::var("PATH")
            .expect("a UTF-8 PATH")
            .split(':')
            .map(|dir| Path::new(dir).join(needed))
            .find(|candidate| candidate.is_file())
            .unwrap_or_else(|| panic!("`{needed}` is on PATH"));
        std::os::unix::fs::symlink(real, bin.join(needed)).expect("link the tool");
    }
    let candidate = rig.candidate_with("");
    let called = rig.tool_as(
        &[
            "run",
            "--previous",
            &rig.previous,
            "--candidate",
            &candidate,
            "--list",
            LIST,
            "--scratch",
            rig.scratch.to_str().expect("a UTF-8 path"),
        ],
        |command| {
            command.env("PATH", &bin);
        },
    );
    called.void("did-not-run", "cargo is not on PATH");
}

#[test]
fn a_list_that_is_refused_costs_no_build() {
    let rig = Rig::new();
    let list = format!(
        "jigc::g_flow\tsetup::refuses\tit refuses now\tcommit:{}\n",
        rig.later
    );
    let run = rig.run(&a_run(), &list, "", &[]);
    run.called.refused("list", "line 1 points at nothing");
    assert_eq!(
        run.calls,
        Vec::<String>::new(),
        "cargo was asked for nothing"
    );
    assert_eq!(
        fs::read_dir(&rig.scratch)
            .expect("read the scratch directory")
            .count(),
        0,
        "and nothing was minted"
    );
}

// ---------------------------------------------------------------------------
// A kept baseline
// ---------------------------------------------------------------------------
//
// `run --baseline DIR` builds the previous release in DIR and keeps what the baseline is
// made of there — the tree, the built tests, the record of that build, the report — and a
// later run against the same previous release uses it instead of building. **It is of the
// class that records falsely**: a baseline that is not the one this run would have made,
// used as if it were, is a comparison against nothing. So what is held here is the guard:
// the key the baseline is kept with, member by member, and that nothing is used because a
// directory is there.

/// The directory a call names for the baseline — shell-hostile, like the rig's other two.
const HOSTILE_KEPT: &str = "a \"kept\" it's #3";

/// What the tool owns in that directory: the baseline, and the lock two runs do not share
/// it under.
const KEPT: &str = "regression-set.kept";
const LOCK: &str = "regression-set.lock";

/// The path the stand-in's tests name, inside the kept baseline.
const BAKED: &str = "target-previous/debug/jigc";

/// The two binaries the stand-in for cargo builds.
const PREVIOUS_BINARY: &[u8] = b"the previous release's binary\n";
const CANDIDATE_BINARY: &[u8] = b"the candidate's binary\n";

impl Rig {
    /// The directory the rig's baseline is kept in, as the tool names it: resolved.
    fn kept(&self) -> PathBuf {
        self.scratch
            .parent()
            .expect("the scratch directory lies in the rig's")
            .join(HOSTILE_KEPT)
    }

    /// A file of the kept baseline.
    fn in_kept(&self, rel: &str) -> PathBuf {
        self.kept().join(KEPT).join(rel)
    }

    /// A call of `run` that names the directory `kept`, over a list of this text.
    fn keeping_in(&self, kept: &Path, list: &str) -> Asked {
        Asked {
            previous: self.previous.clone(),
            list: list.to_owned(),
            mode: String::new(),
            more: vec![
                "--baseline".to_owned(),
                kept.to_str().expect("a UTF-8 path").to_owned(),
            ],
            runner: RUNNER.to_owned(),
            compiler: COMPILER.to_owned(),
        }
    }

    /// A call of `run` that names the rig's kept baseline, over a list of this text.
    fn keeping(&self, list: &str) -> Asked {
        self.keeping_in(&self.kept(), list)
    }

    /// The rig's copy of the tool with one more line of comment: the same tool, and other
    /// bytes.
    fn another_tool(&self) {
        let path = self.root.join(TOOL);
        let mut source = fs::read(&path).expect("read the rig's copy of the tool");
        source.extend_from_slice(b"# one more line\n");
        placed_executable::write(&path, source);
    }

    /// Another git first on the rig's `PATH`: the host's own in everything but what it says
    /// of itself.
    fn another_git(&self) {
        let real = std::env::var("PATH")
            .expect("a UTF-8 PATH")
            .split(':')
            .map(|dir| Path::new(dir).join("git"))
            .find(|candidate| candidate.is_file())
            .expect("git is on PATH");
        placed_executable::write(
            &self.dir.path().join("bin/git"),
            format!(
                "#!/bin/sh\nif [ \"$1\" = --version ]; then echo 'git version 0.0.0 (another)'; exit 0; fi\nexec '{}' \"$@\"\n",
                real.display()
            ),
        );
    }
}

impl Driven {
    /// What the line says of the kept baseline, held to its form: the directory, whether
    /// the baseline there was used, the word one was refused with and its sentence, and the
    /// key of the baseline the run left there.
    fn baseline(&self) -> &Value {
        let said = &self.called.line["baseline"];
        let fields: Vec<&str> = said
            .as_object()
            .unwrap_or_else(|| {
                panic!(
                    "the line says what became of the baseline: {}",
                    self.called.raw
                )
            })
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            fields,
            ["detail", "dir", "key", "refused", "unkept", "used"],
            "{}",
            self.called.raw
        );
        said
    }

    /// The key of the baseline the run left in the directory: a sha256.
    fn key(&self) -> String {
        let key = self.baseline()["key"]
            .as_str()
            .unwrap_or_else(|| panic!("the run left a kept baseline: {}", self.called.raw));
        assert!(
            key.len() == 64 && key.bytes().all(|b| b.is_ascii_hexdigit()),
            "a key is named by a sha256: {key}"
        );
        key.to_owned()
    }

    /// Whether cargo was asked to build the previous release's tests, and to run them on
    /// their own binary.
    fn built_the_previous_release(&self) -> bool {
        let asked = |saying: &str| self.calls.iter().any(|call| call.contains(saying));
        asked("--no-run") && asked("--profile regression-baseline")
    }

    /// The run used the baseline kept in `kept` under `key`: nothing was refused, and the
    /// previous release was neither built nor run.
    fn used(&self, kept: &Path, key: &str) {
        assert_eq!(
            self.baseline(),
            &json!({"dir": kept, "used": true, "refused": null, "detail": null, "key": key,
                    "unkept": null}),
            "the kept baseline is used: {}",
            self.called.raw
        );
        assert!(
            !self.calls.iter().any(|call| {
                call.contains("--no-run")
                    || call.contains("regression-baseline")
                    || call.starts_with("nextest list")
                    || call.starts_with("metadata")
            }),
            "the previous release is not built and its baseline is not run again: {:?}",
            self.calls
        );
    }

    /// The run refused the baseline kept in `kept` with `word`, said `saying` of it, built
    /// the previous release afresh — and kept that in its place.
    fn refused_for(&self, kept: &Path, word: &str, saying: &str) {
        let said = self.baseline();
        assert_eq!(
            (&said["dir"], &said["used"], said["refused"].as_str()),
            (&json!(kept), &json!(false), Some(word)),
            "the kept baseline is refused with `{word}`: {}",
            self.called.raw
        );
        let detail = said["detail"].as_str().expect("a refusal has its sentence");
        assert!(
            detail.contains(saying),
            "refused `{word}` says `{saying}`: {detail}"
        );
        assert!(
            self.built_the_previous_release(),
            "what is refused is built afresh: {:?}",
            self.calls
        );
        self.key();
    }

    /// The run left NO baseline in `kept` and used none: the line says by which word what
    /// it built there was not kept — `why` — and in a sentence that holds `saying`.
    fn not_kept(&self, kept: &Path, why: &str, saying: &str) {
        let said = self.baseline();
        assert_eq!(
            (
                &said["dir"],
                &said["used"],
                &said["key"],
                said["unkept"]["why"].as_str()
            ),
            (&json!(kept), &json!(false), &Value::Null, Some(why)),
            "what the run built is not kept, by `{why}`: {}",
            self.called.raw
        );
        let detail = said["unkept"]["detail"]
            .as_str()
            .expect("what is not kept has its sentence");
        assert!(
            detail.contains(saying),
            "not kept by `{why}` says `{saying}`: {detail}"
        );
    }

    /// Cargo's calls, with the run's own directory named `<work>`.
    fn steps(&self) -> Vec<String> {
        let work = self.called.line["work"]
            .as_str()
            .expect("the line names its directory");
        self.calls
            .iter()
            .map(|call| call.replace(work, "<work>"))
            .collect()
    }
}

/// What a line says of the two commits — the line without what is of the run that printed
/// it: its directory, its seconds, and what it says of a kept baseline.
fn said(line: &Value) -> Value {
    let mut said = line.clone();
    let fields = said.as_object_mut().expect("the line is an object");
    for of_the_run in ["work", "evidence", "seconds", "baseline"] {
        fields.remove(of_the_run);
    }
    said["proof"]
        .as_object_mut()
        .expect("the line carries its proof")
        .remove("baked_path");
    said
}

/// The second run of the stand-in's suite, made from the record: what a run on a kept
/// baseline asks cargo for, after the two things it asks for the key.
fn the_candidates_run() -> [String; 2] {
    [
        "build -p jigc --locked".to_owned(),
        "nextest run --no-fail-fast --retries 1 \
         --binaries-metadata <work>/evidence/binaries-metadata.json \
         --cargo-metadata <work>/evidence/cargo-metadata.json \
         --tool-config-file regression-set:<work>/evidence/nextest.toml \
         --profile regression-candidate"
            .to_owned(),
    ]
}

/// What a run that names a kept baseline asks before anything else: what the runner is,
/// and what the compiler is — in the previous release's tree.
const ASKED_FOR_THE_KEY: [&str; 2] = ["nextest --version", "rustc -vV in previous"];

#[test]
fn a_kept_baseline_is_used_and_the_previous_release_is_not_built_again() {
    let rig = Rig::new();
    let list = both_rows(&rig);
    let kept = rig.kept();
    let unkept = rig.run(&a_run(), &list, "", &[]);
    unkept.called.verdict("green");

    // Nothing is kept there yet: the run asks what the key is made of, builds the previous
    // release in the directory as a run builds it anywhere, and keeps the baseline.
    let first = rig.ask(&a_run(), &rig.keeping(&list));
    first.called.verdict("green");
    let key = first.key();
    assert_eq!(
        first.baseline(),
        &json!({"dir": kept, "used": false, "refused": null, "detail": null, "key": key,
                "unkept": null}),
        "nothing was kept, so nothing is refused: {}",
        first.called.raw
    );
    assert_eq!(first.steps()[..2], ASKED_FOR_THE_KEY);
    assert_eq!(
        first.steps()[2..],
        unkept.steps()[..],
        "the run that keeps asks cargo for what a run that keeps nothing asks"
    );

    // The second run: the candidate's binary, the swap and the candidate's run — and
    // nothing of the previous release.
    let second = rig.ask(&a_run(), &rig.keeping(&list));
    let line = second.called.verdict("green");
    second.used(&kept, &key);
    assert_eq!(
        second.steps(),
        [
            &ASKED_FOR_THE_KEY.map(str::to_owned)[..],
            &the_candidates_run()[..]
        ]
        .concat(),
        "what a run on a kept baseline asks cargo for"
    );
    for step in ["unpack-previous", "build-previous", "baseline"] {
        assert_eq!(
            line["seconds"][step],
            Value::Null,
            "a run on a kept baseline spends nothing on `{step}`: {}",
            second.called.raw
        );
    }

    // The three lines say the same of the two commits: the verdict, the counts, every test
    // named, the list, the two binaries, and the four hashes of the proof.
    assert_eq!(said(&first.called.line), said(&unkept.called.line));
    assert_eq!(said(line), said(&unkept.called.line));
    let baked = rig.in_kept(BAKED);
    assert_eq!(
        line["proof"]["baked_path"].as_str(),
        baked.to_str(),
        "the path the tests name lies in the kept baseline"
    );

    // The evidence of the run is whole — the baseline's report and the record of the build
    // lie in it, and the kept record beside them — and `verdict` says the same over it.
    let work = PathBuf::from(line["work"].as_str().expect("the line names its directory"));
    for held in [
        "run.json",
        "baseline-junit.xml",
        "candidate-junit.xml",
        "binaries-metadata.json",
        "cargo-metadata.json",
        "baseline.json",
    ] {
        assert!(
            work.join("evidence").join(held).is_file(),
            "the evidence of a run on a kept baseline holds `{held}`"
        );
    }
    let again = rig.verdict_over(
        line["evidence"]
            .as_str()
            .expect("the line names its evidence"),
    );
    assert_eq!(said(again.verdict("green")), said(line));
    assert_eq!(again.line["baseline"], line["baseline"]);
    assert!(
        !work.join("previous").exists() && !work.join("target-previous").exists(),
        "the previous release lies in the kept baseline, not in the run's directory"
    );
    assert!(rig.in_kept("previous/Cargo.toml").is_file());

    // At rest the kept baseline is as its own baseline left it: the previous release's
    // binary at the path the tests name.
    assert_eq!(
        fs::read(&baked).expect("the file at the path"),
        PREVIOUS_BINARY
    );

    // And it is the previous release's, not a candidate's: another candidate is judged
    // against the same kept baseline, on its own list.
    let other = rig.ask(
        &a_run(),
        &rig.keeping(&rig.row("jigc::g_flow", "setup::refuses_a_seeded_file")),
    );
    let line = other.called.verdict("red");
    other.used(&kept, &key);
    assert_eq!(
        pairs(&line["not_on_list"]),
        ["jigc::g_migrate uninstall::says_dropped"]
    );
}

/// **A run that names no baseline is the run it was** — the line, byte for byte but for
/// the run's own directory and its seconds; what it asks cargo for; and where it writes.
#[test]
fn a_run_that_names_no_baseline_is_byte_for_byte_what_it_was() {
    let rig = Rig::new();
    let list = both_rows(&rig);
    let run = rig.run(&a_run(), &list, "", &[]);
    let work = run.called.line["work"]
        .as_str()
        .expect("the line names its directory");
    // The directory as the line spells it, and the seconds, which are a clock's.
    let spelled = serde_json::to_string(work).expect("a path is a string");
    let line = run
        .called
        .raw
        .replace(&spelled[1..spelled.len() - 1], "<work>");
    let seconds = line
        .find("\"seconds\": {")
        .expect("the line has its seconds");
    let end = seconds + line[seconds..].find('}').expect("the seconds close");
    let line = format!(
        "{}\"seconds\": <seconds>{}",
        &line[..seconds],
        &line[end + 1..]
    );

    let old = sha256_of(PREVIOUS_BINARY);
    let new = sha256_of(CANDIDATE_BINARY);
    let candidate = rig.candidate_with(&list);
    let row = |binary: &str, test: &str| {
        format!(
            "{{\"binary\": \"{binary}\", \"test\": \"{test}\", \"change\": \"it refuses now\", \"pointer\": \"commit:{}\"}}",
            rig.intended
        )
    };
    assert_eq!(
        line,
        format!(
            "{{\"tool\": \"regression-set\", \"status\": \"green\", \"reason\": null, \"detail\": null, \
             \"list\": {{\"path\": \"{LIST}\", \"commit\": \"{candidate}\", \"sha256\": \"{}\", \"rows\": 2}}, \
             \"previous\": {{\"commit\": \"{}\", \"sha256\": \"{old}\"}}, \
             \"candidate\": {{\"commit\": \"{candidate}\", \"sha256\": \"{new}\"}}, \
             \"seconds\": <seconds>, \
             \"proof\": {{\"baked_path\": \"<work>/target-previous/debug/jigc\", \"named_by\": 1, \
             \"baseline_after\": \"{old}\", \"baseline_before\": \"{old}\", \
             \"candidate_after\": \"{new}\", \"candidate_before\": \"{new}\"}}, \
             \"excluded_by_no_row\": [], \
             \"counts\": {{\"tests\": 22, \"baseline_passed\": 22, \"baseline_failed\": 0, \
             \"candidate_passed\": 20, \"candidate_failed\": 2, \"differences\": 2, \"on_list\": 2, \
             \"not_on_list\": 0, \"excluded\": 0, \"excluded_by_no_row\": 0, \"stale\": 0, \"fixed\": 0}}, \
             \"not_on_list\": [], \"on_list\": [{}, {}], \"excluded\": [], \"stale\": [], \
             \"flaky\": {{\"baseline\": [], \"candidate\": []}}, \
             \"evidence\": \"<work>/evidence\", \"work\": \"<work>\"}}\n",
            sha256_of(list.as_bytes()),
            rig.previous,
            row("jigc::g_flow", "setup::refuses_a_seeded_file"),
            row("jigc::g_migrate", "uninstall::says_dropped"),
        ),
        "the line of a run that names no baseline"
    );
    assert_eq!(run.called.stderr, "", "and it says nothing beside its line");

    // It asks nothing for a key, and nobody but cargo is asked anything.
    assert_eq!(run.steps().len(), 6, "{:?}", run.calls);
    assert!(
        !run.calls
            .iter()
            .any(|call| call.contains("--version") || call.starts_with("rustc")),
        "a run that keeps nothing asks for no key: {:?}",
        run.calls
    );

    // It writes in the directory it minted under the scratch directory, and nowhere else.
    let minted: Vec<PathBuf> = fs::read_dir(&rig.scratch)
        .expect("read the scratch directory")
        .map(|entry| entry.expect("an entry").path())
        .collect();
    assert_eq!(minted, [PathBuf::from(work)]);
    assert!(!rig.kept().exists(), "no baseline is kept unasked");
    let facts: Value = serde_json::from_str(
        &fs::read_to_string(Path::new(work).join("evidence/run.json")).expect("read run.json"),
    )
    .expect("run.json is JSON");
    assert_eq!(
        facts["baseline"],
        Value::Null,
        "and its evidence says nothing of one"
    );
}

/// **The key, member by member.** A kept baseline is used only while each of the eight
/// things it was kept with is what it was. One arm a member: a baseline is kept, the member
/// changes, and the next run refuses the baseline by that member's word, says what it was
/// kept with and what it is now, builds the previous release afresh and keeps that in its
/// place — which the run after it uses.
#[test]
fn a_kept_baseline_is_refused_for_each_thing_it_was_kept_with() {
    type Differs = fn(&Rig, &mut Asked);
    let members: [(&str, &str, Differs); 8] = [
        ("previous", "the previous release's commit", |rig, asked| {
            asked.previous = rig.before.clone()
        }),
        ("exclusions", "the list's exclusion rows", |rig, asked| {
            asked
                .list
                .push_str(&rig.excluding("jigc-engine", "unit::holds_0"));
        }),
        ("tool", "the tool's own bytes", |rig, _| rig.another_tool()),
        ("runner", "`cargo-nextest 0.9.144 (another)`", |_, asked| {
            asked.runner = "cargo-nextest 0.9.144 (another)".to_owned()
        }),
        ("compiler", "`rustc 1.96.0 (another)`", |_, asked| {
            asked.compiler = "rustc 1.96.0 (another)".to_owned()
        }),
        ("git", "`git version 0.0.0 (another)`", |rig, _| {
            rig.another_git()
        }),
        ("tests", "the test binary `jigc::g_flow`", |rig, _| {
            let test = rig.in_kept("target-previous/debug/deps/g_flow-1a1a");
            let mut bytes = fs::read(&test).expect("read the kept test binary");
            bytes.push(b'\n');
            fs::write(&test, bytes).expect("write the kept test binary");
        }),
        ("jigc", "the baseline's own `jigc`", |rig, _| {
            fs::write(rig.in_kept(BAKED), "something else\n").expect("write at the path")
        }),
    ];
    for (word, saying, differs) in members {
        let rig = Rig::new();
        let kept = rig.kept();
        let mut asked = rig.keeping(&both_rows(&rig));
        let first = rig.ask(&a_run(), &asked);
        first.called.verdict("green");
        let key = first.key();

        differs(&rig, &mut asked);
        let refused = rig.ask(&a_run(), &asked);
        // Refused is not void: the run builds what it could not use, and gives its verdict.
        let line = refused.called.verdict("green");
        refused.refused_for(&kept, word, saying);
        assert_eq!(
            (&line["counts"]["differences"], &line["counts"]["on_list"]),
            (&json!(2), &json!(2)),
            "{word}: {}",
            refused.called.raw
        );
        // What the tests and the binary are is what the build made them: those two are
        // the members a fresh build of the same commit gives again.
        let rebuilt = refused.key();
        assert_eq!(
            rebuilt == key,
            matches!(word, "tests" | "jigc"),
            "{word}: the key of what was built afresh, against the refused one"
        );

        let next = rig.ask(&a_run(), &asked);
        next.called.verdict("green");
        next.used(&kept, &rebuilt);
    }
}

/// **A change that is no member of the key refuses nothing**: another candidate, a row of
/// the list that is no exclusion row — and a file in the kept tree that the previous
/// release's commit does not hold, which is what its own tests leave there.
#[test]
fn a_kept_baseline_is_held_to_its_key_and_to_nothing_else() {
    let rig = Rig::new();
    let kept = rig.kept();
    let first = rig.ask(&a_run(), &rig.keeping(""));
    first.called.verdict("red");
    let key = first.key();
    fs::create_dir_all(rig.in_kept("previous/target")).expect("a directory in the tree");
    fs::write(
        rig.in_kept("previous/target/CACHEDIR.TAG"),
        "left by a run\n",
    )
    .expect("a file in the tree");
    // The list gains its two rows — rows for changes — and the candidate is another commit.
    let next = rig.ask(&a_run(), &rig.keeping(&both_rows(&rig)));
    next.called.verdict("green");
    next.used(&kept, &key);
}

/// **The baseline's own binary.** At rest a kept baseline holds the previous release's
/// binary at the path its tests name, as its baseline left it, and a copy of it. A file
/// there that is another one — what a run killed after its swap leaves, the candidate's —
/// refuses the baseline, and so does a copy that is another file.
#[test]
fn a_kept_baseline_whose_binary_was_replaced_is_refused() {
    let rig = Rig::new();
    let kept = rig.kept();
    let asked = rig.keeping(&both_rows(&rig));
    let baked = rig.in_kept(BAKED);
    let old = sha256_of(PREVIOUS_BINARY);
    let new = sha256_of(CANDIDATE_BINARY);
    let first = rig.ask(&a_run(), &asked);
    first.called.verdict("green");
    let key = first.key();

    // The same bytes written again are the same binary: it is held by its hash.
    fs::write(&baked, PREVIOUS_BINARY).expect("write at the path");
    rig.ask(&a_run(), &asked).used(&kept, &key);

    // The candidate's binary at the path.
    fs::write(&baked, CANDIDATE_BINARY).expect("write at the path");
    let run = rig.ask(&a_run(), &asked);
    let line = run.called.verdict("green");
    run.refused_for(
        &kept,
        "jigc",
        &format!("the file at the path the tests name is sha256 `{new}`, and the baseline was kept with `{old}`"),
    );
    assert_eq!(
        (
            &line["proof"]["baseline_before"],
            &line["proof"]["baseline_after"]
        ),
        (&json!(old), &json!(old)),
        "the baseline was made again, on the previous release's binary: {}",
        run.called.raw
    );
    assert_eq!(
        fs::read(&baked).expect("the file at the path"),
        PREVIOUS_BINARY,
        "and the baseline built afresh is at rest"
    );
    rig.ask(&a_run(), &asked).used(&kept, &key);

    // The copy the path is restored from.
    fs::write(rig.in_kept("jigc"), "something else\n").expect("write the copy");
    let run = rig.ask(&a_run(), &asked);
    run.called.verdict("green");
    run.refused_for(&kept, "jigc", "the kept copy of it is sha256 `");
    rig.ask(&a_run(), &asked).used(&kept, &key);
}

/// **A baseline that was not finished is not used.** The record is written last, and a
/// directory without one — a run that ended, or was killed, before its baseline held — is
/// refused and built afresh. So is one whose record is not what the tool wrote, or whose
/// files are not the ones the record names: the report above all, which is the baseline.
#[test]
fn a_kept_baseline_that_was_not_finished_is_refused_and_built_afresh() {
    let rig = Rig::new();
    let kept = rig.kept();
    // Red on this list: one of the two differences is on no row.
    let list = rig.row("jigc::g_flow", "setup::refuses_a_seeded_file");
    let unlisted = ["jigc::g_migrate uninstall::says_dropped"];

    // A run that ends before its baseline is made keeps none: the tree and the build lie
    // in the directory, and no record.
    let mut failing = rig.keeping(&list);
    failing.mode = "fail-candidate".to_owned();
    let ended = rig.ask(&a_run(), &failing);
    ended.called.void("build-candidate", "exited 101");
    assert_eq!(
        (&ended.baseline()["refused"], &ended.baseline()["detail"]),
        (&Value::Null, &Value::Null),
        "{}",
        ended.called.raw
    );
    ended.not_kept(
        &kept,
        "build-candidate",
        "kept only by the run that compared against it whole",
    );
    assert!(rig.in_kept(BAKED).is_file() && !rig.in_kept("baseline.json").exists());

    // Nor does a run whose baseline did not hold — the file at the path was another one
    // before it: what it found unfinished it built afresh, and it left that unfinished too.
    let mut disturbed = rig.keeping(&list);
    disturbed.mode = "disturbed".to_owned();
    let ended = rig.ask(&a_run(), &disturbed);
    ended.called.void(
        "baseline",
        "not the previous release's binary before the baseline",
    );
    assert_eq!(
        (&ended.baseline()["refused"], &ended.baseline()["key"]),
        (&json!("incomplete"), &Value::Null),
        "{}",
        ended.called.raw
    );
    ended.not_kept(
        &kept,
        "baseline",
        "kept only by the run that compared against it whole",
    );
    assert!(rig.in_kept(BAKED).is_file() && !rig.in_kept("baseline.json").exists());

    let asked = rig.keeping(&list);
    let run = rig.ask(&a_run(), &asked);
    run.called.verdict("red");
    run.refused_for(&kept, "incomplete", "no finished record");
    let key = run.key();
    rig.ask(&a_run(), &asked).used(&kept, &key);

    // The candidate's report: in it both tests fail. Taken for the baseline's, both would
    // have failed on their own binary and the run would name no difference at all.
    let (_, on_candidate) = reports(&a_run());
    type Undoes<'a> = Box<dyn Fn() + 'a>;
    let record = rig.in_kept("baseline.json");
    let rows: Vec<(&str, Undoes, &str, &str)> = vec![
        (
            "its record is cut off",
            Box::new(|| {
                let whole = fs::read(&record).expect("read the record");
                fs::write(&record, &whole[..whole.len() / 2]).expect("write the record");
            }),
            "incomplete",
            "no finished record",
        ),
        (
            "its record is gone",
            Box::new(|| fs::remove_file(&record).expect("remove the record")),
            "incomplete",
            "no finished record",
        ),
        (
            "its record is another tool's",
            Box::new(|| {
                let whole = fs::read_to_string(&record).expect("read the record");
                let named = "\"tool\": \"regression-set\"";
                assert!(whole.contains(named), "the record names its tool: {whole}");
                fs::write(&record, whole.replace(named, "\"tool\": \"another\""))
                    .expect("write the record");
            }),
            "incomplete",
            "no finished record",
        ),
        (
            "the report is another one",
            Box::new(|| {
                fs::write(rig.in_kept("baseline-junit.xml"), &on_candidate)
                    .expect("write a report");
            }),
            "record",
            "`baseline-junit.xml` is not the file",
        ),
        (
            "the report is gone",
            Box::new(|| {
                fs::remove_file(rig.in_kept("baseline-junit.xml")).expect("remove the report");
            }),
            "record",
            "`baseline-junit.xml` is not the file",
        ),
        (
            "the record of the build is another one",
            Box::new(|| {
                let path = rig.in_kept("binaries-metadata.json");
                let mut bytes = fs::read(&path).expect("read the recorded build");
                bytes.push(b'\n');
                fs::write(&path, bytes).expect("write the recorded build");
            }),
            "record",
            "`binaries-metadata.json` is not the file",
        ),
        (
            "the previous release's tree is gone",
            Box::new(|| fs::remove_dir_all(rig.in_kept("previous")).expect("remove the tree")),
            "record",
            "the previous release's tree does not hold `Cargo.toml`",
        ),
        (
            "a file of the tree is another one",
            Box::new(|| {
                fs::write(rig.in_kept("previous/README.md"), "another product\n")
                    .expect("write a file of the tree");
            }),
            "record",
            "the previous release's tree does not hold `README.md` as the commit",
        ),
    ];
    for (what, undoes, word, saying) in rows {
        undoes();
        let run = rig.ask(&a_run(), &asked);
        let line = run.called.verdict("red");
        assert_eq!(pairs(&line["not_on_list"]), unlisted, "{what}");
        run.refused_for(&kept, word, saying);
        // Built afresh it is the same baseline, and finished: the next row starts from it.
        assert_eq!(run.key(), key, "{what}");
    }
    rig.ask(&a_run(), &asked).used(&kept, &key);

    // A kept baseline is used where it was built: its tests name a path in it. Moved, it
    // is refused — the directory's name is nothing it is trusted by.
    let moved = kept.with_file_name("moved");
    fs::rename(&kept, &moved).expect("move the kept baseline");
    let run = rig.ask(&a_run(), &rig.keeping_in(&moved, &list));
    run.called.verdict("red");
    run.refused_for(&moved, "record", "is used where it was built");
}

/// **A baseline is kept only by the run that compared against it whole** (the scoped review
/// before the parts, finding 3; DECISIONS.md → 2026-10-13, *Three findings of the scoped
/// review, repaired*). Until this a baseline was kept as soon as its own two hashes held and
/// its report was readable and not empty — so a baseline whose run had gone wrong was kept
/// with its failure, held by its key like any other, and EVERY LATER RUN WAS VOID ON IT:
///
/// - a test that failed on its own binary and is on no exclusion row — a machine under load
///   — voided each later run by `baseline`, the same test named, though it passed again;
/// - the same test beside a difference that is off the list — a red run — did the same to
///   the run after the list was completed;
/// - a runner that was signalled while it ran kept a report of the tests that had run, and
///   each later run was void `incomplete`.
///
/// Now what a run built is kept only where that run ended green or red and named no test
/// under `excluded_by_no_row`: the record is written after the verdict. Left unkept, the
/// directory holds no finished record, the next run builds afresh — and keeps. The line
/// says it: `unkept: {why, detail}`, the word the run was void by. AND WHAT IS FIT IS STILL
/// KEPT AND USED: a baseline in which a test fails on its own binary UNDER ITS ROW is kept
/// by the run that made it and used by the next, which builds nothing of the release.
#[test]
fn a_baseline_is_kept_only_by_the_run_that_compared_against_it_whole() {
    let asks_git = "jigc::g_flow fences::reads_the_git_history";
    let with = |on_previous: End, on_candidate: End| {
        let mut suite = a_run();
        suite.push(ran(
            "jigc::g_flow",
            "fences::reads_the_git_history",
            on_previous,
            on_candidate,
        ));
        suite
    };
    // The run after one that kept nothing: it finds no finished record, builds the previous
    // release afresh, gives its verdict — and keeps what it built, which the run after uses.
    let builds_afresh_and_keeps = |rig: &Rig, suite: &[Ran], asked: &Asked| {
        let kept = rig.kept();
        let next = rig.ask(suite, asked);
        next.called.verdict("green");
        next.refused_for(&kept, "incomplete", "no finished record");
        assert_eq!(
            next.baseline()["unkept"],
            Value::Null,
            "{}",
            next.called.raw
        );
        let key = next.key();
        let after = rig.ask(suite, asked);
        after.called.verdict("green");
        after.used(&kept, &key);
    };

    // (1) A TEST FAILED ON ITS OWN BINARY, AND NO ROW NAMES IT: void — and the next run, in
    // which it passes, is not void on a baseline that was kept with the failure.
    let rig = Rig::new();
    let kept = rig.kept();
    let asked = rig.keeping(&both_rows(&rig));
    let first = rig.ask(&with(Fail, Pass), &asked);
    let line = first.called.void(
        "baseline",
        "1 of the 23 tests failed on their own binary and no exclusion row",
    );
    assert_eq!(pairs(&line["excluded_by_no_row"]), [asks_git]);
    builds_afresh_and_keeps(&rig, &with(Pass, Pass), &asked);
    first.not_kept(
        &kept,
        "baseline",
        "kept only by the run that compared against it whole",
    );

    // (2) THE SAME TEST BESIDE A DIFFERENCE THAT IS OFF THE LIST: red, and named — and not
    // kept either: with the list completed, the next run would be void by it.
    let rig = Rig::new();
    let kept = rig.kept();
    let one_row = rig.keeping(&rig.row("jigc::g_flow", "setup::refuses_a_seeded_file"));
    let first = rig.ask(&with(Fail, Pass), &one_row);
    let line = first.called.verdict("red");
    assert_eq!(pairs(&line["excluded_by_no_row"]), [asks_git]);
    builds_afresh_and_keeps(&rig, &with(Pass, Pass), &rig.keeping(&both_rows(&rig)));
    first.not_kept(
        &kept,
        "baseline",
        asks_git.split(' ').nth(1).expect("the test"),
    );
    assert!(
        first.baseline()["unkept"]["detail"]
            .as_str()
            .is_some_and(|detail| detail.contains("failed on its own binary")),
        "{}",
        first.called.raw
    );

    // (3) THE BASELINE'S RUNNER WAS CUT SHORT: its report holds the tests that had run. The
    // run is void — its second run ran them all — and the next run is not void on it.
    let rig = Rig::new();
    let kept = rig.kept();
    let asked = rig.keeping(&both_rows(&rig));
    let mut cut = rig.keeping(&both_rows(&rig));
    cut.mode = "cut-short".to_owned();
    let first = rig.ask(&a_run(), &cut);
    first.called.void(
        "incomplete",
        "the two runs did not run the same tests: 2 are",
    );
    // What the line says of it is in the run's evidence: `verdict` over it says the same.
    let again = rig.verdict_over(
        first.called.line["evidence"]
            .as_str()
            .expect("the line names its evidence"),
    );
    again.void("incomplete", "did not run the same tests");
    assert_eq!(again.line["baseline"], first.called.line["baseline"]);
    builds_afresh_and_keeps(&rig, &a_run(), &asked);
    first.not_kept(
        &kept,
        "incomplete",
        "kept only by the run that compared against it whole",
    );

    // (4) AND WHAT IS FIT IS KEPT, AND USED: the test fails on its own binary under its row.
    let rig = Rig::new();
    let kept = rig.kept();
    let listed = rig.keeping(&format!(
        "{}{}",
        both_rows(&rig),
        rig.excluding("jigc::g_flow", "fences::reads_the_git_history")
    ));
    let first = rig.ask(&with(Fail, Fail), &listed);
    first.called.verdict("green");
    let key = first.key();
    assert_eq!(
        first.baseline(),
        &json!({"dir": kept, "used": false, "refused": null, "detail": null, "key": key,
                "unkept": null}),
        "a baseline that is fit is kept: {}",
        first.called.raw
    );
    let second = rig.ask(&with(Fail, Fail), &listed);
    let line = second.called.verdict("green");
    second.used(&kept, &key);
    assert_eq!(
        pairs(&line["excluded"]),
        [asks_git],
        "{}",
        second.called.raw
    );
}

/// A process that holds the lock of a kept baseline's directory, as a run of the tool
/// holds it, until its stdin closes — which a test that ends, however it ends, does for it.
fn holding(lock: &Path) -> std::process::Child {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;
    fs::create_dir_all(lock.parent().expect("a lock lies in a directory"))
        .expect("create the directory of the lock");
    let mut holder = Command::new("python3")
        .args([
            "-c",
            "import fcntl, sys\nheld = open(sys.argv[1], 'a')\nfcntl.flock(held, fcntl.LOCK_EX)\nprint('held', flush=True)\nsys.stdin.read()\n",
        ])
        .arg(lock)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn python3");
    let mut said = String::new();
    BufReader::new(holder.stdout.as_mut().expect("the holder's stdout"))
        .read_line(&mut said)
        .expect("read the holder's line");
    assert_eq!(said, "held\n", "the holder holds the lock");
    holder
}

/// The holder lets go, and is gone.
fn released(mut holder: std::process::Child) {
    drop(holder.stdin.take());
    assert!(holder.wait().expect("wait for the holder").success());
}

/// **Two runs do not share one kept baseline at a time.** A run swaps the candidate's
/// binary in at a path inside the kept baseline, so a second run there would drive the
/// first one's candidate. The directory has a lock; a run that does not get it says so in
/// its line and runs as a run that names no baseline runs — in its own directory, with a
/// build of its own, writing nothing where the other one works.
#[test]
fn a_kept_baseline_another_run_holds_is_not_shared() {
    let rig = Rig::new();
    let kept = rig.kept();
    let list = both_rows(&rig);
    let asked = rig.keeping(&list);
    let unkept = rig.run(&a_run(), &list, "", &[]);
    let shut_out = |holder: std::process::Child| {
        let run = rig.ask(&a_run(), &asked);
        let line = run.called.verdict("green");
        let of_it = run.baseline();
        assert_eq!(
            (
                &of_it["dir"],
                &of_it["used"],
                &of_it["refused"],
                &of_it["key"]
            ),
            (&json!(kept), &json!(false), &json!("locked"), &Value::Null),
            "{}",
            run.called.raw
        );
        assert!(
            of_it["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("another run holds")),
            "{}",
            run.called.raw
        );
        assert_eq!(
            run.steps(),
            unkept.steps(),
            "a run shut out is a run that names no baseline"
        );
        let work = Path::new(line["work"].as_str().expect("the line names its directory"));
        assert_eq!(
            line["proof"]["baked_path"].as_str(),
            work.join(BAKED).to_str(),
            "it builds in its own directory"
        );
        assert_eq!(said(line), said(&unkept.called.line));
        released(holder);
    };

    // Nothing is kept yet: the run shut out keeps nothing, and the directory holds the
    // lock and nothing else.
    shut_out(holding(&kept.join(LOCK)));
    let held: Vec<String> = fs::read_dir(&kept)
        .expect("read the directory")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(held, [LOCK]);

    // A baseline is kept: the run shut out leaves every file of it as it was.
    let first = rig.ask(&a_run(), &asked);
    first.called.verdict("green");
    let key = first.key();
    let record = fs::read(rig.in_kept("baseline.json")).expect("read the record");
    shut_out(holding(&kept.join(LOCK)));
    assert_eq!(
        fs::read(rig.in_kept(BAKED)).expect("the file at the path"),
        PREVIOUS_BINARY
    );
    assert_eq!(
        fs::read(rig.in_kept("baseline.json")).expect("read the record"),
        record
    );

    // The lock is the holder's while it lives, and nobody's after: the next run uses the
    // baseline.
    rig.ask(&a_run(), &asked).used(&kept, &key);
}

/// **The swap and its proof are on a kept baseline what they are on a fresh one.** The
/// previous release's binary put back during the candidate's run is void by the hash taken
/// after it, and a candidate that is byte for byte the previous release is void before any
/// run — and neither costs the kept baseline, which the next run uses.
#[test]
fn the_swap_is_proven_on_a_kept_baseline_as_on_a_fresh_one() {
    let rig = Rig::new();
    let kept = rig.kept();
    let list = both_rows(&rig);
    let first = rig.ask(&a_run(), &rig.keeping(&list));
    first.called.verdict("green");
    let key = first.key();

    let mut put_back = rig.keeping(&list);
    put_back.mode = "put-back".to_owned();
    let run = rig.ask(&a_run(), &put_back);
    let line = run
        .called
        .void("swap", "the previous release's binary, put back");
    run.used(&kept, &key);
    assert_eq!(
        (
            &line["proof"]["candidate_before"],
            &line["proof"]["candidate_after"]
        ),
        (
            &json!(sha256_of(CANDIDATE_BINARY)),
            &json!(sha256_of(PREVIOUS_BINARY))
        ),
        "{}",
        run.called.raw
    );

    let mut same = rig.keeping(&list);
    same.mode = "same-binary".to_owned();
    let run = rig.ask(&a_run(), &same);
    run.called
        .void("swap", "byte for byte the previous release's");
    run.used(&kept, &key);
    assert_eq!(
        run.steps(),
        [
            &ASKED_FOR_THE_KEY.map(str::to_owned)[..],
            &the_candidates_run()[..1]
        ]
        .concat(),
        "no run is made of a candidate no hash tells from the previous release"
    );

    let next = rig.ask(&a_run(), &rig.keeping(&list));
    next.called.verdict("green");
    next.used(&kept, &key);
}

// ---------------------------------------------------------------------------
// The source, and the header
// ---------------------------------------------------------------------------

/// Every `cargo nextest run` the source spells that is neither the build (`--no-run`) nor
/// made from the recorded build (`--binaries-metadata`): a run that would go through
/// cargo's build, which puts the previous release's binary back.
fn runs_through_cargos_build(source: &str) -> Vec<String> {
    const SPELLED: &str = "\"cargo\", \"nextest\", \"run\"";
    source
        .match_indices(SPELLED)
        .map(|(at, _)| {
            let rest = &source[at..];
            &rest[..rest.find(']').expect("an argument list closes")]
        })
        .filter(|argv| !argv.contains("\"--no-run\"") && !argv.contains("\"--binaries-metadata\""))
        .map(|argv| argv.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}

#[test]
fn no_run_goes_through_cargos_build() {
    let source = fs::read_to_string(repo_root().join(TOOL)).expect("read the tool");
    assert_eq!(
        source.matches("\"nextest\", \"run\"").count(),
        2,
        "the tool spells two `cargo nextest run`: the build, and the run both runs are made by"
    );
    assert_eq!(
        runs_through_cargos_build(&source),
        Vec::<String>::new(),
        "after the swap a run through cargo's build puts the previous release's binary back"
    );

    // The plant: the run, made as the gate makes its own.
    let planted = source.replace(
        "\"--binaries-metadata\", os.path.join(self.evidence, \"binaries-metadata.json\"),",
        "\"--workspace\",",
    );
    assert_ne!(planted, source, "the plant found the run's argument list");
    assert_eq!(
        runs_through_cargos_build(&planted).len(),
        1,
        "the scan names the planted run"
    );
}

#[test]
fn the_header_is_the_help_and_its_status_table_is_this_one() {
    let source = fs::read_to_string(repo_root().join(TOOL)).expect("read the tool");
    let header: Vec<&str> = source
        .lines()
        .skip(1)
        .take_while(|line| line.starts_with('#'))
        .map(|line| line.strip_prefix("# ").unwrap_or(&line[1..]))
        .collect();

    let rig = Rig::new();
    for flag in ["--help", "-h"] {
        let out = rig
            .hermetic(Command::new(rig.root.join(TOOL)))
            .arg(flag)
            .output()
            .expect("spawn the tool");
        assert!(out.status.success(), "`{flag}` exits 0");
        assert_eq!(
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .collect::<Vec<_>>(),
            header,
            "`{flag}` prints the header"
        );
    }

    // The table: every line of the header that opens with a status and a word.
    let table: Vec<(String, i32)> = header
        .iter()
        .skip_while(|line| !line.starts_with("Exit status"))
        .filter_map(|line| {
            let mut words = line.split_whitespace();
            let status = words.next()?.parse::<i32>().ok()?;
            Some((words.next()?.to_owned(), status))
        })
        .collect();
    let expected: Vec<(String, i32)> = STATUSES
        .iter()
        .map(|(word, status)| ((*word).to_owned(), *status))
        .collect();
    assert_eq!(
        table, expected,
        "the header's exit statuses are the ones this suite drives"
    );
}
