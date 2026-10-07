//! **`dev/stabilize-probe` — the script side of the stabilization harness's runtime probes**
//! ([DECISIONS.md](../DECISIONS.md) → *2026-10-07 — The runtime probes of the stabilization
//! harness*; the second repair plan's task `K0`,
//! [plan.md](../completions/artifacts/M55/stabilization-build/re-review-test-half/plan.md)
//! → §5).
//!
//! Four facts about the runtime the harness runs in were guessed at, and tasks were cut on
//! the guesses (the plan review's `B7`). A probe is an invocation of the harness that
//! observes one of them; **what it found is judged by this script and never by an agent's
//! sentence** — what was sent, what came back, and the verdict of comparing the two, written
//! under the scratch root the invocation was handed. This suite is where that judging is
//! executed, on fixture data, with no runtime and no agent:
//!
//! - *relay* — a throwaway run under the scratch root whose `state` line is the real step
//!   tool's over the real record script, at four sizes
//!   ([`a_throwaway_runs_state_line_is_at_least_the_size_asked_at_four_sizes`]), and a line
//!   that came back judged against the line that was sent
//!   ([`a_relayed_line_is_judged_against_the_line_that_was_sent`]);
//! - *payload* — a batch of N entries on disk judged by its sha256, and a file's hash as
//!   one hashed line ([`a_batch_of_n_entries_is_judged_by_the_sha256_of_what_the_disk_holds`]);
//! - *hold* — a command that writes its last line only after its seconds, is started once
//!   and is answered `running` within a slice
//!   ([`a_hold_ends_only_after_its_seconds_is_started_once_and_is_answered_within_a_slice`]),
//!   and what became of it judged from its own files
//!   ([`a_held_command_is_judged_from_its_own_files`]);
//! - *required* — the stand-in binary and its hash, and a return judged by the field it
//!   came back with ([`a_return_is_judged_by_the_field_it_came_back_with`]).
//!
//! **A probe touches no run, no branch, no remote and no file of the repository.** Every
//! act refuses a scratch root that is missing, no directory, or inside the repository —
//! judged on the real path, so a link into the repository is inside it — and after every
//! act has run to its end the repository's tree, its `git status`, its head and its
//! remote are what they were
//! ([`every_act_refuses_a_scratch_root_inside_the_repository_and_changes_no_file_of_it`]).
//!
//! **Proved red on applied mutants**, each against the arm that names its decision: the
//! `inside` refusal dropped; a relayed line taken as whole without the comparison; a batch
//! judged by the hash an agent relayed instead of the disk's; a returned hash taken as the
//! binary's unread; a hold that prints its line before it waits, one that can be started
//! twice, and a dead one read as running; a verdict that does not hold its cases to their
//! sum; a file hashed wherever it lies; a hold read as outlived whatever its reads said;
//! and a killed hold judged by what its agent said. Eleven, each applied alone; none
//! survived.
//!
//! **What this suite cannot show** is everything a probe exists to find out: what a real
//! agent relays, writes or holds, and what the runtime hands the harness. Those are a real
//! invocation's ([stabilization-workflow.md](../implementation/stabilization-workflow.md) →
//! *The runtime probes*). The harness's half — which steps a probe launches, and what it
//! returns — is driven in [`stabilize_simulation`](super::stabilize_simulation).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::support::child_stdin;

use super::dev_stabilize_record::gitleaks_stub;
use super::dev_stabilize_step::StepRig;
use super::placed_executable;

const TOOL: &str = "dev/stabilize-probe";
const SCANNER: &str = "dev/hygiene-scan";
const GITLEAKS_CONFIG: &str = ".gitleaks.toml";

/// The word a refusal opens with, and its exit status.
const REFUSALS: &[(&str, i32)] = &[
    ("usage", 2),
    ("inside", 3),
    ("no-scratch", 4),
    ("exists", 5),
    ("missing", 6),
    ("outside", 7),
    ("tool", 8),
    ("garbled", 9),
];

/// The four sizes of the relay probe: ledger rows, doors of the round's scope, and the
/// least its ONE line measures — the sizes the re-review measured a real run's state at
/// (3, 25, 75 and 143 KB).
const SIZES: [(u64, u64, usize); 4] = [
    (1, 2, 3_000),
    (20, 40, 25_000),
    (60, 150, 74_000),
    (120, 300, 142_000),
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

/// What a call of the tool left: its exit status, the ONE line it printed — held to the
/// hash it ends with before it is read — and its stderr.
struct Probed {
    code: i32,
    raw: String,
    line: Value,
    stderr: String,
}

impl Probed {
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
        let opening = format!("stabilize-probe: refused {word}: ");
        assert!(
            self.stderr.starts_with(&opening) && self.stderr.trim_end().lines().count() == 1,
            "a refusal is one line on stderr that opens `{opening}`: {}",
            self.stderr
        );
        assert_eq!(self.line["status"], "halted", "the line: {}", self.raw);
        assert_eq!(self.line["refused"], word, "the line: {}", self.raw);
        let halt = &self.line["halt"];
        assert!(
            halt["root_cause"]
                .as_str()
                .is_some_and(|cause| cause.starts_with(&format!("{word}: "))),
            "the halt report's root cause opens with the word: {}",
            self.raw
        );
        for field in ["evidence", "tree_state", "recommendation"] {
            assert!(halt[field].is_string(), "the halt report has `{field}`");
        }
        &self.line
    }
}

/// The one line of a call's stdout, as the object it is — after the line is held to the
/// sha256 it ends with, which is the sha256 of the line without that field.
fn line_of(stdout: &str) -> Value {
    assert!(
        stdout.ends_with('\n') && stdout.trim_end().lines().count() == 1,
        "the tool prints ONE line on stdout: {stdout:?}"
    );
    let line = stdout.trim_end_matches('\n');
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

/// A throwaway repository that holds the probe tool beside the scripts it copies into a
/// throwaway run, a bare remote, and the scratch roots a probe is handed — outside it.
struct ProbeRig {
    rig: StepRig,
}

impl ProbeRig {
    fn new(label: &str) -> Self {
        let rig = StepRig::unopened(&format!("probe-{label}"));
        for script in [TOOL, SCANNER] {
            placed_executable::copy(&repo_root().join(script), &rig.root.join(script));
        }
        let config = fs::read(repo_root().join(GITLEAKS_CONFIG)).expect("read the gitleaks config");
        fs::write(rig.root.join(GITLEAKS_CONFIG), config).expect("write the rig's gitleaks config");
        rig.commit("chore: the probe tool and the hygiene scan");
        rig.git(&["push", "-q", "origin", "main"]);
        rig.on_path("gitleaks", &gitleaks_stub());
        fs::write(
            rig.dir().join("denylist"),
            "# a private term\n\nzzyzxhost\n",
        )
        .expect("write the denylist");
        fs::create_dir_all(rig.dir().join("tmp")).expect("create the rig's temp directory");
        ProbeRig { rig }
    }

    /// A scratch root of the rig's own, made: outside the repository, of plain segments.
    fn scratch(&self, name: &str) -> String {
        let scratch = self.rig.dir().join(name);
        fs::create_dir_all(&scratch).expect("create a scratch root");
        scratch.display().to_string()
    }

    /// A child of the rig: its environment, a denylist and a temp directory of its own.
    fn command(&self, program: &Path) -> Command {
        let mut command = self.rig.hermetic(Command::new(program));
        command
            .env("JIGC_DENYLIST_FILE", self.rig.dir().join("denylist"))
            .env(
                "TMPDIR",
                format!("{}/", self.rig.dir().join("tmp").display()),
            );
        command
    }

    /// The tool, called by its path from a directory that is not the repository: it finds
    /// its repository from where it lies.
    fn probe(&self, args: &[&str]) -> Probed {
        let out = self
            .command(&self.rig.root.join(TOOL))
            .args(args)
            .current_dir(self.rig.dir())
            .stdin(Stdio::null())
            .output()
            .expect("run the probe tool");
        let raw = String::from_utf8_lossy(&out.stdout).into_owned();
        Probed {
            code: out.status.code().expect("the tool exits"),
            line: line_of(&raw),
            raw,
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }

    /// The verdict of `probe` over `cases`, with the sum the harness would hand it.
    fn verdict(&self, scratch: &str, probe: &str, cases: &[String]) -> Probed {
        let sum = sha256(cases.join(" ").as_bytes());
        let mut args = vec![
            "verdict",
            "--scratch",
            scratch,
            "--probe",
            probe,
            "--sum",
            sum.as_str(),
            "--",
        ];
        args.extend(cases.iter().map(String::as_str));
        self.probe(&args)
    }

    /// Everything of the repository a probe could change: its head, its branch, what
    /// `git status` shows, every file of its tree with its bytes' hash, and its remote.
    fn repository(&self) -> BTreeMap<String, String> {
        let mut seen = BTreeMap::new();
        seen.insert("HEAD".to_owned(), self.rig.rev("HEAD"));
        seen.insert("branch".to_owned(), self.rig.branch());
        seen.insert("status".to_owned(), self.rig.status());
        seen.insert(
            "refs".to_owned(),
            self.rig
                .git(&["for-each-ref", "--format=%(refname) %(objectname)"]),
        );
        seen.insert(
            "remote main".to_owned(),
            self.rig.remote("main").unwrap_or_default(),
        );
        files_under(&self.rig.root, &self.rig.root, &mut seen);
        seen
    }
}

/// Every file under `dir` but git's own, by its path from `root`, with its bytes' length
/// and what it is.
fn files_under(root: &Path, dir: &Path, seen: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
        let path = entry.expect("an entry").path();
        let name = path
            .strip_prefix(root)
            .expect("under the root")
            .display()
            .to_string();
        if name == ".git" {
            continue;
        }
        let meta = fs::symlink_metadata(&path).expect("stat an entry");
        if meta.is_dir() {
            seen.insert(format!("{name}/"), "a directory".to_owned());
            files_under(root, &path, seen);
        } else if meta.is_file() {
            seen.insert(name, sha256(&fs::read(&path).expect("read a file")));
        } else {
            seen.insert(name, "a link".to_owned());
        }
    }
}

fn case<'a>(line: &'a Value, id: &str) -> &'a Value {
    line["cases"]
        .as_array()
        .unwrap_or_else(|| panic!("the verdict lists its cases: {line}"))
        .iter()
        .find(|c| c["case"] == id)
        .unwrap_or_else(|| panic!("the verdict has the case `{id}`: {line}"))
}

fn verdicts(line: &Value) -> Vec<(String, String)> {
    line["cases"]
        .as_array()
        .expect("the verdict lists its cases")
        .iter()
        .map(|c| {
            (
                c["case"].as_str().expect("a case").to_owned(),
                c["verdict"].as_str().expect("a verdict").to_owned(),
            )
        })
        .collect()
}

/// The result a verdict wrote, read back from the file its line names: the same cases.
fn written(line: &Value) -> Value {
    let file = line["file"].as_str().expect("the verdict names its file");
    serde_json::from_str(&fs::read_to_string(file).expect("read the result"))
        .expect("the result is JSON")
}

// ---------------------------------------------------------------------------
// relay
// ---------------------------------------------------------------------------

fn relay_tags() -> Vec<String> {
    SIZES
        .iter()
        .flat_map(|(rows, _, _)| (1..=3).map(move |n| format!("r{rows}-{n}")))
        .collect()
}

#[test]
fn a_throwaway_runs_state_line_is_at_least_the_size_asked_at_four_sizes() {
    let rig = ProbeRig::new("sizes");
    let scratch = rig.scratch("scratch-relay");
    let begun = rig.probe(&["begin", "--scratch", &scratch, "--probe", "relay"]);
    let line = begun.done("begin", "begun");
    let runs = line["runs"].as_array().expect("the runs it built");
    assert_eq!(
        runs.len(),
        SIZES.len(),
        "one throwaway run per size: {line}"
    );
    for ((rows, doors, at_least), run) in SIZES.iter().zip(runs) {
        assert_eq!(run["rows"], *rows, "{run}");
        assert_eq!(run["doors"], *doors, "{run}");
        assert!(
            run["bytes"].as_u64().is_some_and(|n| n >= *at_least as u64),
            "the run of {rows} rows over {doors} doors states itself in at least {at_least} bytes: {run}"
        );
        // The line a relay step is handed: the real step tool's, over the real record
        // script, whole — and kept by the tool as what was sent.
        let tag = format!("r{rows}-1");
        let stated = rig.probe(&[
            "state",
            "--scratch",
            &scratch,
            "--rows",
            &rows.to_string(),
            "--tag",
            &tag,
        ]);
        let said = stated.done("state", "read");
        assert!(
            stated.raw.len() > *at_least,
            "the ONE line of {rows} rows is at least {at_least} bytes: {}",
            stated.raw.len()
        );
        assert_eq!(
            said["state"]["ledger"].as_array().map(Vec::len),
            Some(*rows as usize),
            "the run's ledger has the rows asked"
        );
        assert_eq!(
            said["state"]["doors"]["included"].as_array().map(Vec::len),
            Some(*doors as usize),
            "the round's scope has the doors asked"
        );
        assert!(
            stated.raw.contains("\\u"),
            "the line carries what a real one carries: escapes"
        );
        let sent =
            fs::read_to_string(Path::new(&scratch).join(format!("probe/relay/sent/{tag}.line")))
                .expect("the tool keeps what it sent");
        assert_eq!(sent, stated.raw, "what was sent is the line as printed");
    }
    // A run is written once, and a size nobody built is no size.
    rig.probe(&["begin", "--scratch", &scratch, "--probe", "relay"])
        .refused("exists");
    rig.probe(&[
        "state",
        "--scratch",
        &scratch,
        "--rows",
        "7",
        "--tag",
        "r7-1",
    ])
    .refused("usage");
    let unbuilt = rig.scratch("scratch-unbuilt");
    rig.probe(&[
        "state",
        "--scratch",
        &unbuilt,
        "--rows",
        "20",
        "--tag",
        "r20-1",
    ])
    .refused("missing");
}

#[test]
fn a_relayed_line_is_judged_against_the_line_that_was_sent() {
    let rig = ProbeRig::new("relay");
    let scratch = rig.scratch("scratch-relay");
    rig.probe(&["begin", "--scratch", &scratch, "--probe", "relay"])
        .done("begin", "begun");
    // Every case but one is sent; the hash each line ends with is what a harness that
    // took the line whole hands back.
    let mut own: BTreeMap<String, String> = BTreeMap::new();
    for tag in relay_tags().iter().filter(|tag| *tag != "r120-3") {
        let rows = tag[1..tag.find('-').expect("a tag")].to_owned();
        let stated = rig.probe(&[
            "state",
            "--scratch",
            &scratch,
            "--rows",
            &rows,
            "--tag",
            tag,
        ]);
        let said = stated.done("state", "read");
        own.insert(
            tag.clone(),
            said["sha256"].as_str().expect("its own hash").to_owned(),
        );
    }
    let cases: Vec<String> = relay_tags()
        .iter()
        .map(|tag| match tag.as_str() {
            // came back retyped: the harness could not hold it to its hash
            "r1-2" => format!("{tag}:r:altered:3170"),
            // never came back: the agent returned nothing, three times
            "r1-3" => format!("{tag}:nnn:none:0"),
            // a line that hashes, and is another's: the line of another tag
            "r20-1" => format!("{tag}:r:{}:0", own["r20-2"]),
            // nobody ran the command
            "r120-3" => format!("{tag}:r:{}:0", "0".repeat(64)),
            _ => format!("{tag}:r:{}:0", own[tag]),
        })
        .collect();
    let judged = rig.verdict(&scratch, "relay", &cases);
    let line = judged.done("verdict", "judged");
    let expected: Vec<(String, String)> = relay_tags()
        .into_iter()
        .map(|tag| {
            let verdict = match tag.as_str() {
                "r1-2" => "altered",
                "r1-3" => "missing",
                "r20-1" => "other",
                "r120-3" => "unsent",
                _ => "whole",
            };
            (tag, verdict.to_owned())
        })
        .collect();
    assert_eq!(verdicts(line), expected, "{line}");
    assert_eq!(case(line, "r1-2")["bytes_back"], 3170, "{line}");
    assert!(
        case(line, "r120-1")["bytes_sent"]
            .as_u64()
            .is_some_and(|n| n >= 142_000),
        "a case says how much was sent: {line}"
    );
    assert_eq!(case(line, "r1-3")["tries"], 3, "{line}");
    assert_eq!(written(line)["cases"], line["cases"], "the result on disk");
    assert_eq!(written(line)["probe"], "relay");
}

// ---------------------------------------------------------------------------
// payload
// ---------------------------------------------------------------------------

/// A record's batch of `n` entries as a here-document leaves it: one line and its break.
fn batch(n: usize) -> String {
    let entries: Vec<Value> = (1..=n)
        .map(|i| json!({"argv": ["ledger-add", "--run", "probe"], "stdin": format!("the entry {i} — `jigc doc set` → \"lost\"")}))
        .collect();
    format!("{}\n", Value::Array(entries))
}

#[test]
fn a_batch_of_n_entries_is_judged_by_the_sha256_of_what_the_disk_holds() {
    let rig = ProbeRig::new("payload");
    let scratch = rig.scratch("scratch-payload");
    let begun = rig.probe(&["begin", "--scratch", &scratch, "--probe", "payload"]);
    let root = PathBuf::from(
        begun.done("begin", "begun")["root"]
            .as_str()
            .expect("the probe's root"),
    );
    assert_eq!(root, Path::new(&scratch).join("probe/payload"));
    let place = |n: usize, text: &str| -> String {
        let file = root.join(format!("e{n}/batch.json"));
        fs::create_dir_all(file.parent().expect("its directory")).expect("create it");
        fs::write(&file, text).expect("write the batch");
        file.display().to_string()
    };
    // e20: whole. e60: one character off on the disk — and the hash that came back is the
    // one the harness expected, which is what an agent that retyped it would return.
    // e120: never written. e300: whole, and the line that came back says otherwise.
    let whole = place(20, &batch(20));
    place(60, &batch(60).replacen("entry 7 ", "entry 7  ", 1));
    place(300, &batch(300));

    // A file's hash, as one hashed line.
    let hashed = rig.probe(&["hash", "--scratch", &scratch, "--file", &whole]);
    let said = hashed.done("hash", "hashed");
    assert_eq!(said["file_sha256"], sha256(batch(20).as_bytes()).as_str());
    assert_eq!(said["bytes"], batch(20).len());
    assert_eq!(said["file"], whole.as_str());
    // … of a file under the scratch root, that is there.
    let elsewhere = rig.rig.dir().join("elsewhere.json");
    fs::write(&elsewhere, "{}\n").expect("write a file outside the scratch root");
    rig.probe(&[
        "hash",
        "--scratch",
        &scratch,
        "--file",
        &elsewhere.display().to_string(),
    ])
    .refused("outside");
    rig.probe(&[
        "hash",
        "--scratch",
        &scratch,
        "--file",
        &root.join("e120/batch.json").display().to_string(),
    ])
    .refused("missing");

    let expected = |n: usize| sha256(batch(n).as_bytes());
    let cases = vec![
        format!("e20:r:{}:{}", expected(20), expected(20)),
        format!("e60:r:{}:{}", expected(60), expected(60)),
        format!("e120:nnn:{}:none", expected(120)),
        format!("e300:r:{}:{}", expected(300), "0".repeat(64)),
    ];
    let judged = rig.verdict(&scratch, "payload", &cases);
    let line = judged.done("verdict", "judged");
    assert_eq!(
        verdicts(line),
        [
            ("e20", "whole"),
            ("e60", "altered"),
            ("e120", "missing"),
            ("e300", "whole")
        ]
        .map(|(c, v)| (c.to_owned(), v.to_owned())),
        "{line}"
    );
    assert_eq!(case(line, "e20")["relayed"], "agrees", "{line}");
    assert_eq!(
        case(line, "e60")["relayed"],
        "differs",
        "the hash that came back is not the disk's: {line}"
    );
    assert_eq!(case(line, "e120")["relayed"], "none", "{line}");
    assert_eq!(case(line, "e300")["relayed"], "differs", "{line}");
    assert_eq!(case(line, "e300")["bytes"], batch(300).len(), "{line}");
    assert_eq!(case(line, "e20")["entries"], 20, "{line}");
    assert_eq!(written(line)["cases"], line["cases"], "the result on disk");
}

// ---------------------------------------------------------------------------
// hold
// ---------------------------------------------------------------------------

fn kill(pid: &Value) {
    let pid = pid.as_u64().expect("a pid").to_string();
    let out = Command::new("kill").arg(&pid).output().expect("run kill");
    assert!(out.status.success(), "kill {pid}: {out:?}");
}

#[test]
fn a_hold_ends_only_after_its_seconds_is_started_once_and_is_answered_within_a_slice() {
    let rig = ProbeRig::new("hold");
    let scratch = rig.scratch("scratch-hold");
    let hold = |more: &[&str]| {
        let mut args = vec!["hold", "--scratch", scratch.as_str()];
        args.extend_from_slice(more);
        rig.probe(&args)
    };
    let held = |more: &[&str]| {
        let mut args = vec!["held", "--scratch", scratch.as_str()];
        args.extend_from_slice(more);
        rig.probe(&args)
    };
    // Before the probe is begun there is nowhere to hold.
    hold(&["--name", "a", "--seconds", "1"]).refused("missing");
    rig.probe(&["begin", "--scratch", &scratch, "--probe", "hold"])
        .done("begin", "begun");
    held(&["--name", "a"]).refused("missing");

    // In the foreground: its ONE line, only after its seconds.
    let clock = Instant::now();
    let ran = hold(&["--name", "a", "--seconds", "2"]);
    assert!(
        clock.elapsed() >= Duration::from_secs(2),
        "a hold of two seconds prints its line after them: {:?}",
        clock.elapsed()
    );
    let said = ran.done("hold", "held");
    assert_eq!(said["name"], "a");
    assert_eq!(said["seconds"], 2);
    assert!(
        said["ended"].as_f64().expect("ended") - said["started"].as_f64().expect("started") >= 2.0,
        "{said}"
    );
    // Started once.
    hold(&["--name", "a", "--seconds", "1"]).refused("exists");
    hold(&["--name", "a", "--seconds", "1", "--detach"]).refused("exists");
    let read = held(&["--name", "a"]);
    let read = read.done("held", "done");
    assert!(
        read["held_for"].as_f64().is_some_and(|s| s >= 2.0),
        "{read}"
    );

    // Detached: started at once, answered `running` within a slice, and then `done`.
    let clock = Instant::now();
    let started = hold(&["--name", "b", "--seconds", "4", "--detach"]);
    let started = started.done("hold", "started");
    assert!(
        clock.elapsed() < Duration::from_secs(4),
        "a detached hold answers before it ends: {:?}",
        clock.elapsed()
    );
    assert!(started["pid"].as_u64().is_some(), "{started}");
    let within = held(&["--name", "b", "--slice", "1"]);
    assert_eq!(
        within.done("held", "running")["seconds"],
        4,
        "one second into four it is running"
    );
    let ended = held(&["--name", "b", "--slice", "60"]);
    let ended = ended.done("held", "done");
    assert!(
        ended["held_for"].as_f64().is_some_and(|s| s >= 4.0),
        "{ended}"
    );

    // A hold that was killed is dead — never `running`, and never `done`.
    let doomed = hold(&["--name", "c", "--seconds", "600", "--detach"]);
    kill(&doomed.done("hold", "started")["pid"]);
    let clock = Instant::now();
    held(&["--name", "c", "--slice", "60"]).done("held", "dead");
    assert!(
        clock.elapsed() < Duration::from_secs(60),
        "a dead hold is answered before the slice is over"
    );
    // A slice is under five minutes, and a hold has a whole number of seconds.
    held(&["--name", "b", "--slice", "300"]).refused("usage");
    hold(&["--name", "d", "--seconds", "soon"]).refused("usage");
}

#[test]
fn a_held_command_is_judged_from_its_own_files() {
    let rig = ProbeRig::new("held");
    let begin = |scratch: &str| {
        rig.probe(&["begin", "--scratch", scratch, "--probe", "hold"])
            .done("begin", "begun");
    };
    let hold = |scratch: &str, name: &str, seconds: &str| -> Value {
        rig.probe(&[
            "hold",
            "--scratch",
            scratch,
            "--name",
            name,
            "--seconds",
            seconds,
            "--detach",
        ])
        .done("hold", "started")
        .clone()
    };
    let wait = |scratch: &str, name: &str, status: &str| {
        rig.probe(&[
            "held",
            "--scratch",
            scratch,
            "--name",
            name,
            "--slice",
            "60",
        ])
        .done("held", status);
    };
    let judge = |scratch: &str, cases: &[&str]| -> Value {
        let cases: Vec<String> = cases.iter().map(|c| (*c).to_owned()).collect();
        rig.verdict(scratch, "hold", &cases)
            .done("verdict", "judged")
            .clone()
    };
    let said = |line: &Value| -> Vec<(String, String)> { verdicts(line) };
    let pair = |a: &str, b: &str| {
        vec![
            ("a".to_owned(), a.to_owned()),
            ("b".to_owned(), b.to_owned()),
        ]
    };

    // Both ran to their last line.
    let both = rig.scratch("scratch-both");
    begin(&both);
    hold(&both, "a", "1");
    hold(&both, "b", "1");
    wait(&both, "a", "done");
    wait(&both, "b", "done");
    // … and the agent of (a) returned that line, and the last read of (b) said `done`.
    let line = judge(&both, &["a:r:held", "b:2:done"]);
    assert_eq!(said(&line), pair("held", "outlived"), "{line}");
    assert_eq!(case(&line, "a")["process"], "done", "{line}");
    assert_eq!(case(&line, "b")["reads"], 2, "{line}");
    assert_eq!(written(&line)["cases"], line["cases"], "the result on disk");
    // … and the agent of (a) returned nothing, and no read of (b) came back: the command
    // ran on, and nobody was there for its line.
    let line = judge(&both, &["a:nnn:none", "b:3:none"]);
    assert_eq!(said(&line), pair("not-held", "unread"), "{line}");
    let line = judge(&both, &["a:r:other", "b:1:running"]);
    assert_eq!(said(&line), pair("not-held", "unread"), "{line}");

    // (a) was never started; (b) was killed.
    let killed = rig.scratch("scratch-killed");
    begin(&killed);
    kill(&hold(&killed, "b", "600")["pid"]);
    wait(&killed, "b", "dead");
    let line = judge(&killed, &["a:nnn:none", "b:1:dead"]);
    assert_eq!(said(&line), pair("unstarted", "killed"), "{line}");

    // (a) was killed — whatever its agent said; (b) is still running when it is judged.
    let running = rig.scratch("scratch-running");
    begin(&running);
    kill(&hold(&running, "a", "600")["pid"]);
    wait(&running, "a", "dead");
    let still = hold(&running, "b", "600");
    let line = judge(&running, &["a:r:held", "b:4:running"]);
    assert_eq!(said(&line), pair("killed", "running"), "{line}");
    kill(&still["pid"]);
}

// ---------------------------------------------------------------------------
// required
// ---------------------------------------------------------------------------

#[test]
fn a_return_is_judged_by_the_field_it_came_back_with() {
    let rig = ProbeRig::new("required");
    let scratch = rig.scratch("scratch-required");
    let begun = rig.probe(&["begin", "--scratch", &scratch, "--probe", "required"]);
    let binary = begun.done("begin", "begun")["binary"].clone();
    let file = binary["file"].as_str().expect("the stand-in binary");
    let hash = binary["sha256"].as_str().expect("its hash").to_owned();
    assert!(Path::new(file).starts_with(Path::new(&scratch).join("probe/required")));
    assert_eq!(
        sha256(&fs::read(file).expect("read the stand-in")),
        hash,
        "the hash a reviewer is handed is the file's"
    );
    let other = "0".repeat(64);
    let judge = |a: &str, b: &str| -> Vec<(String, String)> {
        let line = rig
            .verdict(&scratch, "required", &[a.to_owned(), b.to_owned()])
            .done("verdict", "judged")
            .clone();
        assert_eq!(written(&line)["cases"], line["cases"], "the result on disk");
        verdicts(&line)
    };
    let pair = |a: &str, b: &str| {
        vec![
            ("a".to_owned(), a.to_owned()),
            ("b".to_owned(), b.to_owned()),
        ]
    };
    // The object without the field; and the field, as the file's hash.
    assert_eq!(
        judge("a:r:absent", &format!("b:r:{hash}")),
        pair("absent", "returned")
    );
    // Nothing, after three tries; a throw among the tries.
    assert_eq!(judge("a:nnn:none", "b:ntt:none"), pair("nothing", "threw"));
    // A field that is not the file's hash — made up — in either spelling.
    assert_eq!(
        judge(&format!("a:r:{other}"), "b:r:malformed"),
        pair("other", "other")
    );
    // A return after a retry is a return; an agent that halted gave no answer.
    assert_eq!(
        judge(&format!("a:nr:{hash}"), "b:r:halted"),
        pair("returned", "halted")
    );
    let line = rig
        .verdict(
            &scratch,
            "required",
            &["a:nnn:none".to_owned(), "b:tr:absent".to_owned()],
        )
        .done("verdict", "judged")
        .clone();
    assert_eq!(case(&line, "a")["tries"], 3, "{line}");
    assert_eq!(case(&line, "b")["tries"], 2, "{line}");
    assert_eq!(case(&line, "b")["handed"], hash.as_str(), "{line}");
}

// ---------------------------------------------------------------------------
// refusals, and what a probe leaves of the repository
// ---------------------------------------------------------------------------

#[test]
fn a_verdict_whose_cases_do_not_hash_to_their_sum_or_are_not_the_probes_is_refused() {
    let rig = ProbeRig::new("garbled");
    let scratch = rig.scratch("scratch-garbled");
    rig.verdict(
        &scratch,
        "required",
        &["a:r:absent".to_owned(), "b:r:absent".to_owned()],
    )
    .refused("missing");
    rig.probe(&["begin", "--scratch", &scratch, "--probe", "required"])
        .done("begin", "begun");
    // One character of one case changed on its way: the sum is another's.
    let sum = sha256(b"a:r:absent b:r:absent");
    rig.probe(&[
        "verdict",
        "--scratch",
        &scratch,
        "--probe",
        "required",
        "--sum",
        &sum,
        "--",
        "a:r:absent",
        "b:r:halted",
    ])
    .refused("garbled");
    let refused = |cases: &[&str]| {
        let cases: Vec<String> = cases.iter().map(|c| (*c).to_owned()).collect();
        rig.verdict(&scratch, "required", &cases).refused("garbled");
    };
    // A case short, a case twice, a case of another probe, a case outside its grammar,
    // and two cells that contradict each other.
    refused(&["a:r:absent"]);
    refused(&["a:r:absent", "a:r:absent"]);
    refused(&["a:r:absent", "b:r:absent", "e20:r:absent"]);
    refused(&["a:r:absent", "b:rrrr:absent"]);
    refused(&["a:r:none", "b:r:absent"]);
    refused(&["a:nnn:absent", "b:r:absent"]);
    assert!(
        !Path::new(&scratch)
            .join("probe/required/result.json")
            .exists(),
        "a refused verdict writes no result"
    );
    // A probe nobody has, and an act nobody has.
    rig.probe(&["begin", "--scratch", &scratch, "--probe", "everything"])
        .refused("usage");
    rig.probe(&["probe", "--scratch", &scratch])
        .refused("usage");
    rig.probe(&[]).refused("usage");
}

#[test]
fn every_act_refuses_a_scratch_root_inside_the_repository_and_changes_no_file_of_it() {
    let rig = ProbeRig::new("outside");
    let before = rig.repository();
    assert_eq!(before["status"], "", "the rig's tree is clean");

    // Inside: a link of plain segments into the repository — judged on the real path.
    let inside = rig.rig.dir().join("a-link-inside");
    std::os::unix::fs::symlink(rig.rig.root.join("crates"), &inside)
        .expect("link into the repository");
    let inside = inside.display().to_string();
    // Missing, and no directory.
    let absent = rig.rig.dir().join("absent").display().to_string();
    let a_file = rig.rig.dir().join("a-file");
    fs::write(&a_file, "no directory\n").expect("write a file");
    let a_file = a_file.display().to_string();

    let sum = sha256(b"a:r:absent b:r:absent");
    let acts = |scratch: &str| -> Vec<Vec<String>> {
        let s = scratch.to_owned();
        let own = |args: &[&str]| -> Vec<String> { args.iter().map(|a| (*a).to_owned()).collect() };
        let mut all: Vec<Vec<String>> = ["required", "relay", "payload", "hold"]
            .iter()
            .map(|probe| own(&["begin", "--scratch", &s, "--probe", probe]))
            .collect();
        all.push(own(&[
            "state",
            "--scratch",
            &s,
            "--rows",
            "1",
            "--tag",
            "r1-1",
        ]));
        all.push(own(&[
            "hash",
            "--scratch",
            &s,
            "--file",
            &format!("{s}/a.json"),
        ]));
        all.push(own(&[
            "hold",
            "--scratch",
            &s,
            "--name",
            "a",
            "--seconds",
            "1",
        ]));
        all.push(own(&[
            "hold",
            "--scratch",
            &s,
            "--name",
            "b",
            "--seconds",
            "1",
            "--detach",
        ]));
        all.push(own(&["held", "--scratch", &s, "--name", "a"]));
        all.push(own(&[
            "verdict",
            "--scratch",
            &s,
            "--probe",
            "required",
            "--sum",
            &sum,
            "--",
            "a:r:absent",
            "b:r:absent",
        ]));
        all
    };
    for (scratch, word) in [
        (&inside, "inside"),
        (&absent, "no-scratch"),
        (&a_file, "no-scratch"),
    ] {
        for act in acts(scratch) {
            let args: Vec<&str> = act.iter().map(String::as_str).collect();
            rig.probe(&args).refused(word);
        }
    }
    // A scratch root that is the repository itself, or no path of plain segments, is no
    // argument at all.
    rig.probe(&["begin", "--scratch", "relative/dir", "--probe", "hold"])
        .refused("usage");
    rig.probe(&["begin", "--scratch", "/tmp/a b", "--probe", "hold"])
        .refused("usage");
    assert!(!Path::new(&absent).exists(), "a refusal makes no directory");
    assert_eq!(
        fs::read_to_string(&a_file).expect("read it"),
        "no directory\n"
    );
    assert_eq!(
        rig.repository(),
        before,
        "a refused act changed the repository"
    );

    // And every act, run to its end under a scratch root outside: the repository's tree,
    // its `git status`, its head and its remote are what they were.
    let scratch = rig.scratch("scratch-all");
    for probe in ["required", "relay", "payload", "hold"] {
        rig.probe(&["begin", "--scratch", &scratch, "--probe", probe])
            .done("begin", "begun");
    }
    rig.probe(&[
        "state",
        "--scratch",
        &scratch,
        "--rows",
        "20",
        "--tag",
        "r20-1",
    ])
    .done("state", "read");
    let batch_file = Path::new(&scratch).join("probe/payload/e20/batch.json");
    fs::create_dir_all(batch_file.parent().expect("its directory")).expect("create it");
    fs::write(&batch_file, batch(20)).expect("write the batch");
    rig.probe(&[
        "hash",
        "--scratch",
        &scratch,
        "--file",
        &batch_file.display().to_string(),
    ])
    .done("hash", "hashed");
    rig.probe(&[
        "hold",
        "--scratch",
        &scratch,
        "--name",
        "a",
        "--seconds",
        "1",
    ])
    .done("hold", "held");
    rig.probe(&[
        "hold",
        "--scratch",
        &scratch,
        "--name",
        "b",
        "--seconds",
        "1",
        "--detach",
    ])
    .done("hold", "started");
    rig.probe(&[
        "held",
        "--scratch",
        &scratch,
        "--name",
        "b",
        "--slice",
        "60",
    ])
    .done("held", "done");
    rig.verdict(
        &scratch,
        "hold",
        &["a:r:held".to_owned(), "b:1:done".to_owned()],
    )
    .done("verdict", "judged");
    rig.verdict(
        &scratch,
        "required",
        &["a:r:absent".to_owned(), "b:r:absent".to_owned()],
    )
    .done("verdict", "judged");
    assert_eq!(rig.repository(), before, "an act changed the repository");
    // Everything it wrote lies under `<scratch>/probe/`.
    let made: Vec<String> = fs::read_dir(&scratch)
        .expect("read the scratch root")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(made, ["probe"], "what the acts left in the scratch root");
}

#[test]
fn the_tools_header_its_statuses_and_this_suites_table_agree() {
    let source = fs::read_to_string(repo_root().join(TOOL)).expect("read the probe tool");
    // The header's table: `#   NN  word   …`.
    let header: BTreeMap<String, i32> = source
        .lines()
        .take_while(|line| line.starts_with('#'))
        .filter_map(|line| {
            let mut words = line.trim_start_matches('#').split_whitespace();
            let status: i32 = words.next()?.parse().ok()?;
            let word = words.next()?;
            (status >= 2).then(|| (word.to_owned(), status))
        })
        .collect();
    // The code's: `    "word": NN,`.
    let at = source.find("\nSTATUS = {\n").expect("the tool's statuses");
    let table = &source[at..];
    let table = &table[..table.find("\n}\n").expect("the table closes")];
    let code: BTreeMap<String, i32> = table
        .lines()
        .filter_map(|line| line.trim().strip_prefix('"'))
        .map(|line| {
            let (word, status) = line.split_once("\": ").expect("a word and its status");
            (
                word.to_owned(),
                status.trim_end_matches(',').parse().expect("a status"),
            )
        })
        .collect();
    let suite: BTreeMap<String, i32> = REFUSALS
        .iter()
        .map(|(word, status)| ((*word).to_owned(), *status))
        .collect();
    assert_eq!(
        header, code,
        "the header's refusals (left) and the code's (right)"
    );
    assert_eq!(
        code, suite,
        "the code's refusals (left) and this suite's (right)"
    );
    // The sizes of the relay probe are the tool's and this suite's alike.
    let sizes = source
        .lines()
        .find(|line| line.starts_with("SIZES = "))
        .expect("the tool's sizes");
    assert_eq!(
        sizes,
        format!(
            "SIZES = ({})",
            SIZES
                .iter()
                .map(|(rows, doors, _)| format!("({rows}, {doors})"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        "the four sizes"
    );
}
