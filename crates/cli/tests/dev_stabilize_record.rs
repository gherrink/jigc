//! **`dev/stabilize-record` — the one script through which a stabilization run's records
//! reach the repository** ([DECISIONS.md](../../../DECISIONS.md) → *2026-10-05 — The
//! stabilization workflow, as ruled*, rulings 12, 13 and 16; the builder's choices are the
//! entry of the same date, *The record script, as built*).
//!
//! A reporter of a stage has a shell and no write tool, and what it writes is public on the
//! next push and read by fences on the next gate. So one script decides where a report
//! lands, what a ledger row may say and what is refused, and this suite holds it to that by
//! **driving the script's own bytes** — copied into a throwaway repository, because the
//! script finds its repository from where it lies — and reading what it left on disk.
//!
//! **Every arm runs under a shell-hostile root** (a space, a `'`, a `"` and a `#` in the
//! repository's path — [dev-workflow.md](../../../implementation/dev-workflow.md), the rule
//! on a path that reaches a shell): the rig has no other kind of root, so no arm can pass
//! only because its path was tame.
//!
//! **The axes, iterated rather than sampled.**
//!
//! - *The identifier sites.* Every identifier the script takes — a run, a round, a stage,
//!   a cycle, a reporter, an attempt, a ledger key, a cited bound, a clause, each name a
//!   check is handed — is driven with every hostile value ([`HOSTILE_IDS`]): each is
//!   refused as `bad-id` and leaves the tree as it was.
//! - *The writers.* Four subcommands put a caller's free text into a file ([`WRITERS`]):
//!   a report's body, a ledger row's cells as it is added and as it is set, a clause
//!   row's cells. The host-path replacement, the hygiene stop, the scanner that could not
//!   run and the killed write are each driven through all four. The list is held to the
//!   script's own parser — every subcommand it names is a writer on the list or one of
//!   the two checks ([`every_subcommand_is_a_listed_writer_or_a_check`]) — so a writer
//!   added to the script meets every one of those arms or reddens that one.
//! - *The scanner's ways of not running* ([`SCAN_BREAKS`]): each is `did-not-run`, never a
//!   pass, and writes nothing.
//!
//! **Two instruments this suite reads instead of respelling.** The install-line fence is
//! named by the script's refusal, so the arm builds its hostile line from that fence's own
//! [`INSTALL_COMMAND`] and asks that fence's own census ([`install_line_carriers`]) what a
//! written file made of the tree. And the denylist half of the scan is the real
//! `dev/hygiene-scan`, unmodified, over a denylist this suite writes; only gitleaks is
//! stood in for — a test runner has none — by a stub that keeps gitleaks' exit contract,
//! with one arm ([`the_machines_own_gitleaks_never_lets_a_credential_through`]) that runs
//! whatever the machine really has.
//!
//! Proved red on applied mutants, each against the arm that names its property: the scan
//! skipped; a scanner's failure read as a pass; the scan run after the write; the body
//! streamed to its path; the cell escape, the line-break rendering, the host-path
//! replacement, the undeclared-scratch refusal and the fence check each dropped; the
//! opening record not required, and the run directory minted on demand; the slug, the
//! link and the taken-attempt checks each dropped; a key that exists appended again; a
//! value outside its vocabulary taken; a damaged table rewritten; a missing and an extra
//! report each read as a pass; the caller's `GIT_*` environment handed to the scan; and
//! the table writers' lock dropped, the lock taken before the input is read, and the
//! output's encoding left to the caller's locale.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::support::child_stdin;
use crate::support::install_line::{INSTALL_COMMAND, install_line_carriers};
use crate::support::scratch::ScratchDir;

/// The script under test, and the scanner it runs, repository-relative.
const SCRIPT: &str = "dev/stabilize-record";
const SCANNER: &str = "dev/hygiene-scan";
const GITLEAKS_CONFIG: &str = ".gitleaks.toml";

/// The one run every rig opens.
const RUN: &str = "rc24";

/// The throwaway repository's directory name: a space, a `'`, a `"` and a `#`.
const HOSTILE_ROOT: &str = "it's a \"repo\" #1";

/// The denylist's one synthetic term — a slug, so that it can also arrive as an identifier.
const DENY_TERM: &str = "zzyzxhost";

/// What the stub gitleaks reports as a secret.
const STUB_SECRET: &str = "STUB-SECRET-SHAPE";

// The exit statuses, as the script's header states them.
const OK: i32 = 0;
const USAGE: i32 = 2;
const NOT_OPENED: i32 = 3;
const BAD_ID: i32 = 4;
const OUTSIDE: i32 = 5;
const EXISTS: i32 = 6;
const BAD_VALUE: i32 = 7;
const NO_SUCH_ROW: i32 = 8;
const FENCE: i32 = 9;
const HOST_PATH: i32 = 10;
const HYGIENE: i32 = 11;
const DID_NOT_RUN: i32 = 12;
const CORRUPT: i32 = 13;
const REPORTS_MISMATCH: i32 = 20;
const LEDGER_MISMATCH: i32 = 21;

/// Every refusal class with its status, as `--help` must list them.
const REFUSALS: &[(i32, &str)] = &[
    (USAGE, "usage"),
    (NOT_OPENED, "not-opened"),
    (BAD_ID, "bad-id"),
    (OUTSIDE, "outside"),
    (EXISTS, "exists"),
    (BAD_VALUE, "bad-value"),
    (NO_SUCH_ROW, "no-such-row"),
    (FENCE, "fence"),
    (HOST_PATH, "host-path"),
    (HYGIENE, "hygiene"),
    (DID_NOT_RUN, "did-not-run"),
    (CORRUPT, "corrupt"),
];

const LEDGER_COLUMNS: [&str; 10] = [
    "key",
    "doctype",
    "round",
    "source",
    "door",
    "clause broken",
    "grade",
    "graded by",
    "disposition",
    "repro pointer",
];

const CLAUSE_COLUMNS: [&str; 5] = ["clause", "instrument", "last commit", "scope", "status"];

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

/// A stand-in for gitleaks that keeps its exit contract: 0 clean, the `--exit-code` value
/// on a finding, anything else when it could not run. `STUB_GITLEAKS` picks a failure:
/// `crash` exits 1 as gitleaks does on an error of its own, `kill-parent` kills the script
/// that launched it, which is the deterministic way to kill a write at its last step
/// before the file exists.
const GITLEAKS_STUB: &str = r#"#!/bin/sh
code=1
report=
repo=
while [ $# -gt 0 ]; do
    case "$1" in
        --exit-code) code=$2; shift ;;
        --exit-code=*) code=${1#*=} ;;
        --report-path) report=$2; shift ;;
        --report-path=*) report=${1#*=} ;;
        --config|--report-format|--log-opts) shift ;;
        -*|git) ;;
        *) repo=$1 ;;
    esac
    shift
done
case "${STUB_GITLEAKS:-scan}" in
    crash) echo "stub gitleaks: could not load its config" >&2; exit 1 ;;
    kill-parent) kill -9 "$PPID"; exit 0 ;;
esac
if grep -rq --exclude-dir=.git @SECRET@ "$repo"; then
    if [ -n "$report" ]; then printf '[{"RuleID":"stub-rule","StartLine":3}]' >"$report"; fi
    exit "$code"
fi
if [ -n "$report" ]; then printf '[]' >"$report"; fi
exit 0
"#;

/// A throwaway repository holding the script's own bytes, one opened run, and the
/// environment a call runs in: its own home, its own temp directory, a denylist, and a
/// `PATH` on which the stub is the first `gitleaks`.
struct Rig {
    dir: ScratchDir,
    root: PathBuf,
    home: PathBuf,
    tmp: PathBuf,
    bin: PathBuf,
    denylist: PathBuf,
    /// What follows the rig's own `bin` on `PATH`: the ambient `PATH`.
    path_tail: String,
    env: Vec<(&'static str, String)>,
}

impl Rig {
    fn new(label: &str) -> Self {
        let dir = ScratchDir::new(&format!("stabilize-{label}"));
        let root = dir.path().join(HOSTILE_ROOT);
        fs::create_dir_all(root.join("dev")).expect("create the rig's dev/");
        for file in [SCRIPT, SCANNER, GITLEAKS_CONFIG] {
            fs::copy(repo_root().join(file), root.join(file))
                .unwrap_or_else(|e| panic!("copy `{file}` into the rig: {e}"));
        }
        let run_dir = root.join("completions/artifacts").join(RUN);
        fs::create_dir_all(&run_dir).expect("create the opened run's directory");
        fs::write(run_dir.join("opening.md"), "# the opening record\n")
            .expect("write the opening record");

        let home = dir.path().join("home/hostile-login");
        let tmp = dir.path().join("tmp");
        let bin = dir.path().join("bin");
        for made in [&home, &tmp, &bin] {
            fs::create_dir_all(made).expect("create a rig directory");
        }
        let stub = bin.join("gitleaks");
        fs::write(&stub, GITLEAKS_STUB.replace("@SECRET@", STUB_SECRET))
            .expect("write the stub gitleaks");
        fs::set_permissions(&stub, fs::Permissions::from_mode(0o755)).expect("chmod the stub");
        let denylist = dir.path().join("denylist");
        fs::write(&denylist, format!("# a private term\n\n{DENY_TERM}\n"))
            .expect("write the denylist");

        let path_tail = std::env::var("PATH").expect("the suite runs with a UTF-8 PATH");
        Rig {
            dir,
            root,
            home,
            tmp,
            bin,
            denylist,
            path_tail,
            env: Vec::new(),
        }
    }

    /// Make the rig's root a repository, so that the install-line fence's census
    /// (`git ls-files`) can be asked what a written file made of this tree.
    fn init_repository(&self) {
        let init = self
            .hermetic(Command::new("git"))
            .args(["init", "-q"])
            .current_dir(&self.root)
            .output()
            .expect("run git init");
        assert!(init.status.success(), "git init the rig: {init:?}");
    }

    /// Take gitleaks off the rig's `PATH` altogether: the stub, and any real one the
    /// machine has — the shadows lie in the rig's temp directory, which no snapshot walks.
    fn without_gitleaks(&mut self) {
        fs::remove_file(self.bin.join("gitleaks")).expect("remove the stub");
        self.path_tail = ambient_path_without("gitleaks", &self.tmp.join("shadow"));
    }

    /// The environment every child of the rig runs in.
    fn hermetic(&self, mut command: Command) -> Command {
        command
            .env("PATH", format!("{}:{}", self.bin.display(), self.path_tail))
            .env("HOME", &self.home)
            .env("TMPDIR", format!("{}/", self.tmp.display()))
            .env("JIGC_DENYLIST_FILE", &self.denylist)
            // The interpreter's own bytecode cache, which macOS's python3 otherwise keeps
            // under the home directory: in the rig's temp directory, where no snapshot
            // reads the interpreter's writes as the script's.
            .env("PYTHONPYCACHEPREFIX", self.tmp.join("pycache"))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .env_remove("STUB_GITLEAKS");
        for (key, value) in &self.env {
            command.env(key, value);
        }
        command
    }

    /// The script, called from a directory that is not the repository: it must find its
    /// repository from where it lies, never from where it was called.
    fn command(&self, args: &[String]) -> Command {
        let mut command = self.hermetic(Command::new(self.root.join(SCRIPT)));
        command
            .args(args)
            .current_dir(self.dir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command
    }

    fn run(&self, args: &[String], stdin: &str) -> Seen {
        let mut child = self.command(args).spawn().expect("spawn the script");
        child_stdin::feed(&mut child, stdin);
        let out = child.wait_with_output().expect("the script exits");
        Seen {
            code: out.status.code(),
            signal: out.status.signal(),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }

    fn run_dir(&self) -> PathBuf {
        self.root.join("completions/artifacts").join(RUN)
    }

    fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.root.join(rel)).unwrap_or_else(|e| panic!("read `{rel}`: {e}"))
    }

    /// Everything on disk the script could have touched: the whole scratch directory but
    /// the rig's own temp directory (where a scan works) and the repository's `.git`.
    fn snapshot(&self) -> BTreeMap<String, String> {
        let mut seen = BTreeMap::new();
        walk(self.dir.path(), self.dir.path(), &self.tmp, &mut seen);
        seen
    }

    /// Seeded ledger rows, one call, for the arms that set or check them.
    fn seed_rows(&self, keys: &[&str]) {
        let rows: Vec<Value> = keys.iter().map(|key| row(key)).collect();
        self.run(&ledger_add(RUN), &json!(rows).to_string())
            .must(OK, &format!("seed the ledger rows {keys:?}"));
    }
}

fn walk(base: &Path, dir: &Path, skip: &Path, seen: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(dir).expect("read a rig directory") {
        let path = entry.expect("a rig directory entry").path();
        if path == skip || path.file_name().is_some_and(|name| name == ".git") {
            continue;
        }
        let rel = path
            .strip_prefix(base)
            .expect("a walked path sits under its base")
            .to_string_lossy()
            .into_owned();
        let meta = fs::symlink_metadata(&path).expect("stat a rig entry");
        if meta.file_type().is_symlink() {
            let to = fs::read_link(&path).expect("read a link");
            seen.insert(rel, format!("link -> {}", to.display()));
        } else if meta.is_dir() {
            seen.insert(rel, "dir".to_owned());
            walk(base, &path, skip, seen);
        } else {
            let bytes = fs::read(&path).expect("read a rig file");
            seen.insert(rel, format!("file: {}", String::from_utf8_lossy(&bytes)));
        }
    }
}

/// The ambient `PATH`, with every directory that holds `tool` replaced by a directory of
/// links to everything else in it — so the tool is gone and nothing beside it is.
fn ambient_path_without(tool: &str, shadows: &Path) -> String {
    let ambient = std::env::var_os("PATH").expect("the suite runs with a PATH");
    let mut kept = Vec::new();
    for (n, dir) in std::env::split_paths(&ambient).enumerate() {
        if !dir.join(tool).exists() {
            kept.push(dir);
            continue;
        }
        let shadow = shadows.join(n.to_string());
        fs::create_dir_all(&shadow).expect("create a shadow directory");
        for entry in fs::read_dir(&dir).expect("read a PATH directory").flatten() {
            if entry.file_name() != tool {
                symlink(entry.path(), shadow.join(entry.file_name())).expect("shadow a tool");
            }
        }
        kept.push(shadow);
    }
    std::env::join_paths(kept)
        .expect("a PATH joins")
        .into_string()
        .expect("a UTF-8 PATH")
}

/// What one call did.
#[derive(Debug)]
struct Seen {
    code: Option<i32>,
    signal: Option<i32>,
    stdout: String,
    stderr: String,
}

impl Seen {
    /// The call exited `code`.
    fn must(&self, code: i32, what: &str) -> &Self {
        assert_eq!(
            self.code,
            Some(code),
            "{what}: expected exit {code}\nstdout: {}\nstderr: {}",
            self.stdout,
            self.stderr
        );
        self
    }

    /// The call was refused as `class`: its status, and one line on stderr that opens with
    /// the class — short, specific and distinguishable, because a script reads it.
    fn refused(&self, code: i32, what: &str) -> &Self {
        self.must(code, what);
        let class = REFUSALS
            .iter()
            .find(|(status, _)| *status == code)
            .map(|(_, class)| *class)
            .expect("a refusal status has a class");
        let opening = format!("stabilize-record: refused {class}: ");
        assert!(
            self.stderr.starts_with(&opening),
            "{what}: the refusal must open `{opening}`; got: {}",
            self.stderr
        );
        assert_eq!(
            self.stderr.trim_end().lines().count(),
            1,
            "{what}: a refusal is one line; got: {}",
            self.stderr
        );
        assert!(
            self.stdout.is_empty(),
            "{what}: a refusal prints nothing on stdout; got: {}",
            self.stdout
        );
        self
    }

    fn json(&self) -> Value {
        serde_json::from_str(self.stdout.trim())
            .unwrap_or_else(|e| panic!("stdout must be one JSON value ({e}): {}", self.stdout))
    }
}

fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|arg| (*arg).to_owned()).collect()
}

/// `report` for a `test`-stage reporter.
fn report(run: &str, round: &str, reporter: &str, attempt: &str) -> Vec<String> {
    vec![
        "report".to_owned(),
        format!("--run={run}"),
        format!("--round={round}"),
        "--stage=test".to_owned(),
        format!("--reporter={reporter}"),
        format!("--attempt={attempt}"),
    ]
}

fn ledger_add(run: &str) -> Vec<String> {
    vec!["ledger-add".to_owned(), format!("--run={run}")]
}

fn ledger_set(run: &str) -> Vec<String> {
    vec!["ledger-set".to_owned(), format!("--run={run}")]
}

fn clause_set(run: &str, clause: &str, rest: &[&str]) -> Vec<String> {
    let mut args = vec![
        "clause-set".to_owned(),
        format!("--run={run}"),
        format!("--clause={clause}"),
    ];
    args.extend(strings(rest));
    args
}

fn check_reports(run: &str, round: &str, attempt: &str, launched: &[&str]) -> Vec<String> {
    let mut args = vec![
        "check-reports".to_owned(),
        format!("--run={run}"),
        format!("--round={round}"),
        "--stage=test".to_owned(),
        format!("--attempt={attempt}"),
        "--".to_owned(),
    ];
    args.extend(strings(launched));
    args
}

fn check_ledger(run: &str, keys: &[&str]) -> Vec<String> {
    let mut args = vec![
        "check-ledger".to_owned(),
        format!("--run={run}"),
        "--".to_owned(),
    ];
    args.extend(strings(keys));
    args
}

/// A well-formed ledger row.
fn row(key: &str) -> Value {
    json!({
        "key": key,
        "doctype": "jigc-feedback",
        "round": 1,
        "source": "audit-install, finding 3",
        "door": "jigc setup",
        "clause": "no-lost-files",
        "repro": "r1/reports/test/audit-install.a1.md, the third block",
    })
}

/// A markdown table row split as a reader splits it: on every `|` no backslash escapes.
fn cells(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cell = String::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'|') => {
                cell.push('|');
                chars.next();
            }
            '|' => out.push(std::mem::take(&mut cell).trim().to_owned()),
            other => cell.push(other),
        }
    }
    assert!(
        line.starts_with('|') && cell.trim().is_empty(),
        "a table row opens and closes with a pipe: {line}"
    );
    out.remove(0);
    out
}

/// The rows of the one table `text` holds, under the header `columns`: every line after
/// the header and its rule is a row of that many cells, and nothing follows the table.
fn table(text: &str, columns: &[&str]) -> Vec<Vec<String>> {
    let lines: Vec<&str> = text.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.starts_with('|') && cells(line) == columns)
        .unwrap_or_else(|| panic!("no header row {columns:?} in:\n{text}"));
    assert!(
        cells(lines[at + 1]).iter().all(|cell| cell == "---"),
        "the header's rule follows it:\n{text}"
    );
    lines[at + 2..]
        .iter()
        .map(|line| {
            let row = cells(line);
            assert_eq!(
                row.len(),
                columns.len(),
                "a row of {} cells, not {}: {line}\nin:\n{text}",
                columns.len(),
                row.len()
            );
            row
        })
        .collect()
}

fn ledger_path() -> String {
    format!("completions/artifacts/{RUN}/ledger.md")
}

fn clauses_path() -> String {
    format!("completions/artifacts/{RUN}/clauses.md")
}

fn report_path(round: &str, reporter: &str, attempt: &str) -> String {
    format!("completions/artifacts/{RUN}/r{round}/reports/test/{reporter}.a{attempt}.md")
}

// ---------------------------------------------------------------------------
// The writers: every way a caller's free text reaches a file
// ---------------------------------------------------------------------------

/// A subcommand that puts free text into a file of the run.
struct Writer {
    name: &'static str,
    /// Anything the writer needs on disk first.
    prepare: fn(&Rig),
    /// The call that writes `text`, with `extra` flags, and the file it writes.
    call: fn(&Rig, text: &str, extra: &[String]) -> (Seen, String),
}

const WRITERS: &[Writer] = &[
    Writer {
        name: "report (the body)",
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut args = report(RUN, "1", "audit-install", "1");
            args.extend_from_slice(extra);
            (
                rig.run(&args, &format!("# a report\n\n{text}\n")),
                report_path("1", "audit-install", "1"),
            )
        },
    },
    Writer {
        name: "ledger-add (a row's `source` cell)",
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut entry = row("audit-f3");
            entry["source"] = json!(text);
            let mut args = ledger_add(RUN);
            args.extend_from_slice(extra);
            (rig.run(&args, &entry.to_string()), ledger_path())
        },
    },
    Writer {
        name: "ledger-set (a disposition's detail)",
        prepare: |rig| rig.seed_rows(&["audit-f3"]),
        call: |rig, text, extra| {
            let patch = json!({"key": "audit-f3", "disposition": "bound", "detail": text});
            let mut args = ledger_set(RUN);
            args.extend_from_slice(extra);
            (rig.run(&args, &patch.to_string()), ledger_path())
        },
    },
    Writer {
        name: "clause-set (the `instrument` cell)",
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut args = clause_set(
                RUN,
                "no-lost-files",
                &["--scope=the delta", "--status=void"],
            );
            args.push(format!("--instrument={text}"));
            args.extend_from_slice(extra);
            (rig.run(&args, ""), clauses_path())
        },
    },
];

/// The two subcommands that write nothing.
const CHECKS: [&str; 2] = ["check-reports", "check-ledger"];

/// The subcommands, as the script's own parser names them when it is handed none of them.
fn subcommands(rig: &Rig) -> Vec<String> {
    let seen = rig.run(&strings(&["no-such-subcommand"]), "");
    seen.refused(USAGE, "a subcommand nobody knows");
    let (_, listed) = seen
        .stderr
        .split_once("the subcommands are ")
        .unwrap_or_else(|| panic!("the refusal lists the subcommands: {}", seen.stderr));
    let (listed, _) = listed.split_once(" — ").expect("the list ends at a dash");
    listed.split(", ").map(str::to_owned).collect()
}

/// A subcommand added to the script is a writer — and then every arm that iterates
/// [`WRITERS`] drives it — or a check, or this arm is red.
#[test]
fn every_subcommand_is_a_listed_writer_or_a_check() {
    let rig = Rig::new("subcommands");
    let named: BTreeSet<String> = subcommands(&rig).into_iter().collect();
    let known: BTreeSet<String> = WRITERS
        .iter()
        .map(|writer| writer.name.split(' ').next().expect("a name").to_owned())
        .chain(CHECKS.map(str::to_owned))
        .collect();
    assert_eq!(
        named, known,
        "the script's subcommands (left) are the listed writers and the two checks (right)"
    );
}

/// Run `arm` once per writer, each in a rig of its own with the writer prepared — and on
/// a thread of its own, because a write is a scan and a scan is a dozen processes.
fn for_every_writer(label: &str, arm: impl Fn(&Writer, &mut Rig) + Sync) {
    std::thread::scope(|scope| {
        for (n, writer) in WRITERS.iter().enumerate() {
            let arm = &arm;
            scope.spawn(move || {
                let mut rig = Rig::new(&format!("{label}-{n}"));
                (writer.prepare)(&rig);
                arm(writer, &mut rig);
            });
        }
    });
}

// ---------------------------------------------------------------------------
// 1 · A report: the path is the script's, and it prints it
// ---------------------------------------------------------------------------

#[test]
fn a_report_lands_at_the_path_the_script_decides_and_prints_it() {
    let rig = Rig::new("report-path");
    rig.init_repository();
    let body = "# audit of the install doors\n\nNothing found.\n";

    let seen = rig.run(&report(RUN, "1", "audit-install", "1"), body);
    seen.must(OK, "a test-stage report");
    let path = report_path("1", "audit-install", "1");
    assert_eq!(seen.stdout, format!("{path}\n"), "the path it wrote, alone");
    assert_eq!(rig.read(&path), body, "the report, byte for byte");

    // A fix-stage report belongs to a cycle of its round.
    let mut fix = strings(&["report", "--round=2", "--stage=fix", "--cycle=3"]);
    fix.extend(strings(&["--reporter=fixer-install", "--attempt=1"]));
    fix.push(format!("--run={RUN}"));
    let seen = rig.run(&fix, body);
    seen.must(OK, "a fix-stage report");
    let path = format!("completions/artifacts/{RUN}/r2/reports/fix/c3/fixer-install.a1.md");
    assert_eq!(seen.stdout, format!("{path}\n"));
    assert_eq!(rig.read(&path), body);

    // The cycle is the fix stage's and only the fix stage's.
    let mut stray = report(RUN, "1", "audit-install", "2");
    stray.push("--cycle=1".to_owned());
    rig.run(&stray, body)
        .refused(USAGE, "a cycle on a test-stage report");
    let cycleless = strings(&[
        "report",
        "--run=rc24",
        "--round=1",
        "--stage=fix",
        "--reporter=fixer-install",
        "--attempt=1",
    ]);
    rig.run(&cycleless, body)
        .refused(USAGE, "a fix-stage report with no cycle");

    // No fence that reads every markdown file was given a new carrier.
    assert_eq!(
        install_line_carriers(&rig.root),
        BTreeSet::new(),
        "the install-line fence's census over the tree the script wrote"
    );
}

#[test]
fn a_taken_attempt_is_refused_and_the_next_attempt_lands_beside_it() {
    let rig = Rig::new("attempts");
    let first = "# the first attempt\n";
    rig.run(&report(RUN, "1", "audit-install", "1"), first)
        .must(OK, "the first attempt");

    let before = rig.snapshot();
    rig.run(&report(RUN, "1", "audit-install", "1"), "# a re-run\n")
        .refused(EXISTS, "the same reporter and attempt again");
    assert_eq!(rig.snapshot(), before, "the stale report is not replaced");

    rig.run(&report(RUN, "1", "audit-install", "2"), "# a re-run\n")
        .must(OK, "the next attempt");
    assert_eq!(rig.read(&report_path("1", "audit-install", "1")), first);
    assert_eq!(
        rig.read(&report_path("1", "audit-install", "2")),
        "# a re-run\n"
    );

    // A link lying at the path is not a free path, wherever it points.
    let outside = rig.dir.path().join("elsewhere.md");
    symlink(
        &outside,
        rig.root.join(report_path("1", "audit-install", "3")),
    )
    .expect("plant a link");
    rig.run(
        &report(RUN, "1", "audit-install", "3"),
        "# through a link\n",
    )
    .refused(OUTSIDE, "a dangling link at the report's path");
    assert!(!outside.exists(), "nothing was written through the link");
}

#[test]
fn a_report_that_is_not_text_is_refused() {
    let rig = Rig::new("not-text");
    let before = rig.snapshot();
    for (body, why) in [
        ("", "an empty report"),
        ("  \n\t\n", "a report of whitespace"),
        ("a NUL \0 byte", "a NUL byte"),
    ] {
        rig.run(&report(RUN, "1", "audit-install", "1"), body)
            .refused(BAD_VALUE, why);
    }
    let mut child = rig
        .command(&report(RUN, "1", "audit-install", "1"))
        .spawn()
        .expect("spawn the script");
    child_stdin::feed(&mut child, [0xff, 0xfe, b'\n']);
    let out = child.wait_with_output().expect("the script exits");
    assert_eq!(
        out.status.code(),
        Some(BAD_VALUE),
        "bytes that are not UTF-8"
    );
    assert_eq!(rig.snapshot(), before);
}

// ---------------------------------------------------------------------------
// 2 · Hostile identifiers
// ---------------------------------------------------------------------------

/// Every way an identifier can stop being one path segment, one shell word or one slug.
const HOSTILE_IDS: &[(&str, &str)] = &[
    ("..", "the parent directory"),
    ("../escape", "a climb out of the run directory"),
    (".", "the directory itself"),
    ("a/b", "a slash"),
    ("/etc", "an absolute path"),
    ("a b", "a space"),
    ("a'b", "a single quote"),
    ("a\"b", "a double quote"),
    ("a#b", "a `#`"),
    ("", "the empty identifier"),
    ("a\nb", "a newline"),
    ("a.md", "a dot"),
    ("A", "upper case"),
    ("a--b", "a doubled hyphen"),
    ("-a", "a leading hyphen"),
    ("a-", "a trailing hyphen"),
    ("a$b", "a `$`"),
    ("a`id`", "backticks"),
    ("a;b", "a `;`"),
    ("a|b", "a pipe"),
    ("a\\b", "a backslash"),
    ("é", "a letter outside a-z"),
];

/// The values a number must not be, beside every member of [`HOSTILE_IDS`].
const HOSTILE_NUMBERS: &[(&str, &str)] = &[
    ("0", "zero"),
    ("-1", "a negative number"),
    ("01", "a leading zero"),
    ("1.5", "a fraction"),
    ("r1", "the rendered form"),
    (" 1", "a padded number"),
    ("1e3", "an exponent"),
];

/// A place the script takes an identifier, and the call that puts `value` there.
struct IdSite {
    name: &'static str,
    numeric: bool,
    call: fn(&Rig, value: &str) -> Seen,
}

const BODY: &str = "# a report\n\nNothing found.\n";

const ID_SITES: &[IdSite] = &[
    IdSite {
        name: "report --run",
        numeric: false,
        call: |rig, v| rig.run(&report(v, "1", "audit-install", "1"), BODY),
    },
    IdSite {
        name: "report --round",
        numeric: true,
        call: |rig, v| rig.run(&report(RUN, v, "audit-install", "1"), BODY),
    },
    IdSite {
        name: "report --stage",
        numeric: false,
        call: |rig, v| {
            let mut args = strings(&["report", "--run=rc24", "--round=1", "--attempt=1"]);
            args.push("--reporter=audit-install".to_owned());
            args.push(format!("--stage={v}"));
            rig.run(&args, BODY)
        },
    },
    IdSite {
        name: "report --cycle",
        numeric: true,
        call: |rig, v| {
            let mut args = strings(&["report", "--run=rc24", "--round=1", "--stage=fix"]);
            args.extend(strings(&["--reporter=fixer-install", "--attempt=1"]));
            args.push(format!("--cycle={v}"));
            rig.run(&args, BODY)
        },
    },
    IdSite {
        name: "report --reporter",
        numeric: false,
        call: |rig, v| rig.run(&report(RUN, "1", v, "1"), BODY),
    },
    IdSite {
        name: "report --attempt",
        numeric: true,
        call: |rig, v| rig.run(&report(RUN, "1", "audit-install", v), BODY),
    },
    IdSite {
        name: "ledger-add --run",
        numeric: false,
        call: |rig, v| rig.run(&ledger_add(v), &row("audit-f3").to_string()),
    },
    IdSite {
        name: "ledger-add, a row's key",
        numeric: false,
        call: |rig, v| rig.run(&ledger_add(RUN), &row(v).to_string()),
    },
    IdSite {
        name: "ledger-add, a row's cited bound",
        numeric: false,
        call: |rig, v| {
            let mut entry = row("audit-f3");
            entry["grade"] = json!("out-of-scope");
            entry["graded_by"] = json!("triage");
            entry["bound"] = json!(v);
            rig.run(&ledger_add(RUN), &entry.to_string())
        },
    },
    IdSite {
        name: "ledger-set --run",
        numeric: false,
        call: |rig, v| {
            let patch = json!({"key": "seeded", "disposition": "later"});
            rig.run(&ledger_set(v), &patch.to_string())
        },
    },
    IdSite {
        name: "ledger-set, a patch's key",
        numeric: false,
        call: |rig, v| {
            let patch = json!({"key": v, "disposition": "later"});
            rig.run(&ledger_set(RUN), &patch.to_string())
        },
    },
    IdSite {
        name: "clause-set --run",
        numeric: false,
        call: |rig, v| {
            let rest = [
                "--instrument=the audit",
                "--scope=the delta",
                "--status=void",
            ];
            rig.run(&clause_set(v, "no-lost-files", &rest), "")
        },
    },
    IdSite {
        name: "clause-set --clause",
        numeric: false,
        call: |rig, v| {
            let rest = [
                "--instrument=the audit",
                "--scope=the delta",
                "--status=void",
            ];
            rig.run(&clause_set(RUN, v, &rest), "")
        },
    },
    IdSite {
        name: "check-reports --run",
        numeric: false,
        call: |rig, v| rig.run(&check_reports(v, "1", "1", &["audit-install"]), ""),
    },
    IdSite {
        name: "check-reports --round",
        numeric: true,
        call: |rig, v| rig.run(&check_reports(RUN, v, "1", &["audit-install"]), ""),
    },
    IdSite {
        name: "check-reports --attempt",
        numeric: true,
        call: |rig, v| rig.run(&check_reports(RUN, "1", v, &["audit-install"]), ""),
    },
    IdSite {
        name: "check-reports, a launched reporter",
        numeric: false,
        call: |rig, v| rig.run(&check_reports(RUN, "1", "1", &["audit-install", v]), ""),
    },
    IdSite {
        name: "check-ledger --run",
        numeric: false,
        call: |rig, v| rig.run(&check_ledger(v, &["seeded"]), ""),
    },
    IdSite {
        name: "check-ledger, a finding id",
        numeric: false,
        call: |rig, v| rig.run(&check_ledger(RUN, &["seeded", v]), ""),
    },
];

#[test]
fn a_hostile_identifier_is_refused_at_every_site_and_nothing_is_written() {
    let rig = Rig::new("hostile-ids");
    rig.seed_rows(&["seeded"]);
    let before = rig.snapshot();
    // A few threads over the sites: no call may write, so the calls cannot disturb each
    // other, and the axis is some 450 launches of an interpreter.
    let driven: usize = std::thread::scope(|scope| {
        let workers: Vec<_> = ID_SITES
            .chunks(5)
            .map(|sites| {
                let (rig, before) = (&rig, &before);
                scope.spawn(move || {
                    let mut driven = 0;
                    for site in sites {
                        let numbers: &[(&str, &str)] =
                            if site.numeric { HOSTILE_NUMBERS } else { &[] };
                        for (value, why) in HOSTILE_IDS.iter().chain(numbers) {
                            let what = format!("{} given {value:?} ({why})", site.name);
                            (site.call)(rig, value).refused(BAD_ID, &what);
                            assert_eq!(
                                &rig.snapshot(),
                                before,
                                "{what}: the tree must be as it was"
                            );
                            driven += 1;
                        }
                    }
                    driven
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().expect("a worker thread"))
            .sum()
    });
    assert!(driven > 400, "the axis collapsed to {driven} cells");
}

/// A slug has a length the file system can carry: a name of a thousand letters is not one.
#[test]
fn an_identifier_too_long_for_a_file_name_is_refused() {
    let rig = Rig::new("long-id");
    let long = "a".repeat(300);
    rig.run(&report(RUN, "1", &long, "1"), BODY)
        .refused(BAD_ID, "a reporter of 300 letters");
    rig.run(&report(RUN, "1", "audit-install", &"9".repeat(40)), BODY)
        .refused(BAD_ID, "an attempt of 40 digits");
}

// ---------------------------------------------------------------------------
// 3 · A run that was never opened
// ---------------------------------------------------------------------------

/// Every subcommand, aimed at `run`.
fn every_subcommand(rig: &Rig, run: &str) -> Vec<(&'static str, Seen)> {
    let patch = json!({"key": "seeded", "disposition": "later"});
    let rest = [
        "--instrument=the audit",
        "--scope=the delta",
        "--status=void",
    ];
    vec![
        (
            "report",
            rig.run(&report(run, "1", "audit-install", "1"), BODY),
        ),
        (
            "ledger-add",
            rig.run(&ledger_add(run), &row("audit-f3").to_string()),
        ),
        ("ledger-set", rig.run(&ledger_set(run), &patch.to_string())),
        (
            "clause-set",
            rig.run(&clause_set(run, "no-lost-files", &rest), ""),
        ),
        (
            "check-reports",
            rig.run(&check_reports(run, "1", "1", &["audit-install"]), ""),
        ),
        ("check-ledger", rig.run(&check_ledger(run, &["seeded"]), "")),
    ]
}

#[test]
fn a_run_that_was_never_opened_is_refused_and_no_directory_is_minted() {
    let rig = Rig::new("never-opened");
    let artifacts = rig.root.join("completions/artifacts");

    // A mistyped run name: no directory.
    let before = rig.snapshot();
    for (name, seen) in every_subcommand(&rig, "rc42") {
        seen.refused(NOT_OPENED, &format!("{name} on a run with no directory"));
    }
    assert_eq!(rig.snapshot(), before, "no second run directory was minted");

    // A directory is not an opening: without its opening record the run is not open.
    fs::create_dir_all(artifacts.join("unopened/r1")).expect("a directory with no record");
    // Nor is a directory, or a link, lying where the record should be.
    fs::create_dir_all(artifacts.join("dir-record/opening.md")).expect("a directory as record");
    fs::create_dir_all(artifacts.join("link-record")).expect("a run directory");
    symlink(
        rig.run_dir().join("opening.md"),
        artifacts.join("link-record/opening.md"),
    )
    .expect("a link as record");
    let before = rig.snapshot();
    for run in ["unopened", "dir-record", "link-record"] {
        for (name, seen) in every_subcommand(&rig, run) {
            seen.refused(NOT_OPENED, &format!("{name} on `{run}`"));
        }
    }
    assert_eq!(rig.snapshot(), before);

    // The control: the opened run takes the same calls.
    rig.run(&report(RUN, "1", "audit-install", "1"), BODY)
        .must(OK, "the opened run");
}

// ---------------------------------------------------------------------------
// 4 · Nothing lands outside the run directory
// ---------------------------------------------------------------------------

#[test]
fn a_link_inside_the_run_directory_never_carries_a_write_outside_it() {
    let rest = [
        "--instrument=the audit",
        "--scope=the delta",
        "--status=void",
    ];

    // Each directory a report's path passes through, replaced by a link out of the run.
    for (n, linked) in ["r1", "r1/reports", "r1/reports/test"].iter().enumerate() {
        let rig = Rig::new(&format!("outside-dir-{n}"));
        let outside = rig.dir.path().join("outside");
        fs::create_dir_all(&outside).expect("a directory outside the run");
        let link = rig.run_dir().join(linked);
        fs::create_dir_all(link.parent().expect("a parent")).expect("the link's parent");
        symlink(&outside, &link).expect("plant the link");
        let before = rig.snapshot();
        rig.run(&report(RUN, "1", "audit-install", "1"), BODY)
            .refused(OUTSIDE, &format!("a report through a linked `{linked}`"));
        rig.run(&check_reports(RUN, "1", "1", &["audit-install"]), "")
            .refused(OUTSIDE, &format!("a check through a linked `{linked}`"));
        assert_eq!(rig.snapshot(), before);
    }

    // The two tables, each a link to a file outside the run.
    let rig = Rig::new("outside-table");
    let outside = rig.dir.path().join("outside.md");
    fs::write(&outside, "not the run's\n").expect("a file outside the run");
    symlink(&outside, rig.run_dir().join("ledger.md")).expect("link the ledger");
    symlink(&outside, rig.run_dir().join("clauses.md")).expect("link the clause table");
    let before = rig.snapshot();
    rig.run(&ledger_add(RUN), &row("audit-f3").to_string())
        .refused(OUTSIDE, "a ledger that is a link");
    rig.run(&check_ledger(RUN, &["audit-f3"]), "")
        .refused(OUTSIDE, "a check of a ledger that is a link");
    rig.run(&clause_set(RUN, "no-lost-files", &rest), "")
        .refused(OUTSIDE, "a clause table that is a link");
    assert_eq!(rig.snapshot(), before);

    // The run directory itself, a link to an opened-looking directory elsewhere.
    let rig = Rig::new("outside-run");
    let elsewhere = rig.dir.path().join("elsewhere");
    fs::create_dir_all(&elsewhere).expect("a directory elsewhere");
    fs::write(elsewhere.join("opening.md"), "# an opening record\n").expect("its record");
    symlink(&elsewhere, rig.root.join("completions/artifacts/linked")).expect("link a run");
    let before = rig.snapshot();
    for (name, seen) in every_subcommand(&rig, "linked") {
        seen.refused(
            OUTSIDE,
            &format!("{name} on a run directory that is a link"),
        );
    }
    assert_eq!(rig.snapshot(), before);
}

// ---------------------------------------------------------------------------
// 5 · Host paths become the public placeholders
// ---------------------------------------------------------------------------

/// A home directory's path as an agent harness spells it inside a session's scratch area:
/// every character that is not a letter or a digit flattened to `-`.
fn flattened(path: &Path) -> String {
    path.to_string_lossy()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

#[test]
fn a_report_carries_no_host_path_only_the_public_placeholders() {
    let rig = Rig::new("placeholders");
    let root = rig.root.to_string_lossy().into_owned();
    let real_root = fs::canonicalize(&rig.root).expect("the root resolves");
    let real_root = real_root.to_string_lossy();
    let home = rig.home.to_string_lossy().into_owned();
    let tmp = rig.tmp.to_string_lossy().into_owned();
    let real_tmp = fs::canonicalize(&rig.tmp).expect("the temp directory resolves");
    let real_tmp = real_tmp.to_string_lossy();
    // A session's scratch area lies under the system temp directory; it is the more
    // specific of the two and must win.
    let scratch = format!("{tmp}/agent/session-1/scratchpad");

    let lines: Vec<(String, &str)> = vec![
        // A path into this repository: its repository-relative form.
        (
            format!("edited {root}/crates/cli/src/task.rs:12"),
            "edited crates/cli/src/task.rs:12",
        ),
        (
            format!("and, resolved, {real_root}/dev/gate"),
            "and, resolved, dev/gate",
        ),
        (
            format!("ran `git -C {root} status` and --root={root}/"),
            "ran `git -C . status` and --root=./",
        ),
        // A root of this machine's own is a host path wherever it stands.
        (
            format!("flags: -I{home}/include -L{root}/target KEY={root}"),
            "flags: -I~/include -Ltarget KEY=.",
        ),
        // A home directory: this machine's, and anyone's.
        (
            format!("the denylist at {home}/.config/jigc/denylist"),
            "the denylist at ~/.config/jigc/denylist",
        ),
        (
            "elsewhere: /Users/somebody/work/x, /home/runner/work/jigc.".to_owned(),
            "elsewhere: ~/work/x, ~/work/jigc.",
        ),
        (
            "a url: file:///Users/somebody/x.html".to_owned(),
            "a url: file://~/x.html",
        ),
        // The system temp directory, in each of its spellings.
        (
            format!("a rig at {tmp}/jigc-rig.Ab12/repo"),
            "a rig at <tmp>/jigc-rig.Ab12/repo",
        ),
        (
            format!("and, resolved, {real_tmp}/gate.log"),
            "and, resolved, <tmp>/gate.log",
        ),
        (
            "logs: /tmp/gate.log, /private/tmp/b, /var/tmp/c; a sentence ends at /tmp.".to_owned(),
            "logs: <tmp>/gate.log, <tmp>/b, <tmp>/c; a sentence ends at <tmp>.",
        ),
        (
            "macOS: /var/folders/nj/abc_123/T/jigc-gate and /private/var/folders/nj/abc_123/T"
                .to_owned(),
            "macOS: <tmp>/jigc-gate and <tmp>",
        ),
        // The session's scratch area.
        (
            format!("scripts in {scratch}/fin1/routes.sh, under {scratch}"),
            "scripts in <scratch>/fin1/routes.sh, under <scratch>",
        ),
        // Not host paths: a longer name, a path segment, a placeholder already there.
        (
            "kept: /tmpfile /tmp.bak crates/tmp/x ~/tmp/y docs/home/z <scratch>/tmp/q /usr/bin/git"
                .to_owned(),
            "kept: /tmpfile /tmp.bak crates/tmp/x ~/tmp/y docs/home/z <scratch>/tmp/q /usr/bin/git",
        ),
    ];
    let body: String = lines.iter().map(|(raw, _)| format!("{raw}\n")).collect();
    let want: String = lines
        .iter()
        .map(|(_, clean)| format!("{clean}\n"))
        .collect();

    let mut args = report(RUN, "1", "audit-install", "1");
    args.push(format!("--scratch={scratch}"));
    rig.run(&args, &body).must(OK, "a report naming host paths");
    assert_eq!(rig.read(&report_path("1", "audit-install", "1")), want);
}

#[test]
fn every_writer_replaces_a_host_path() {
    for_every_writer("writer-paths", |writer, rig| {
        let scratch = format!("{}/agent/scratchpad", rig.tmp.display());
        let text = format!(
            "see {}/dev/gate, {}/notes and {scratch}/run.sh",
            rig.root.display(),
            rig.home.display(),
        );
        let (seen, path) = (writer.call)(rig, &text, &[format!("--scratch={scratch}")]);
        seen.must(OK, writer.name);
        let written = rig.read(&path);
        assert!(
            written.contains("see dev/gate, ~/notes and <scratch>/run.sh"),
            "{}: the placeholders, in {written}",
            writer.name
        );
        for raw in [&rig.root, &rig.home, &rig.tmp] {
            assert!(
                !written.contains(&*raw.to_string_lossy()),
                "{}: a host path survived in {written}",
                writer.name
            );
        }
    });
}

/// A session's scratch area that nobody declared lies under the temp directory with the
/// home directory's path flattened into one of its segments. Replaced as a temp path it
/// would still name the home, so it is refused with what to pass — never written.
#[test]
fn an_undeclared_scratch_area_is_refused_rather_than_half_replaced() {
    for_every_writer("writer-scratch", |writer, rig| {
        let scratch = format!(
            "{}/agent-501/{}-projects-jigc/0a1b/scratchpad",
            rig.tmp.display(),
            flattened(&rig.home)
        );
        let text = format!("the repro is {scratch}/fin1/routes.sh");
        let before = rig.snapshot();
        let (seen, _) = (writer.call)(rig, &text, &[]);
        seen.refused(HOST_PATH, writer.name);
        assert!(
            seen.stderr.contains("--scratch"),
            "{}: the refusal says what to pass: {}",
            writer.name,
            seen.stderr
        );
        assert_eq!(rig.snapshot(), before, "{}: nothing written", writer.name);

        let (seen, path) = (writer.call)(rig, &text, &[format!("--scratch={scratch}")]);
        seen.must(OK, writer.name);
        assert!(
            rig.read(&path)
                .contains("the repro is <scratch>/fin1/routes.sh"),
            "{}: declared, it is replaced",
            writer.name
        );
    });
}

#[test]
fn a_scratch_root_that_is_not_an_absolute_path_is_refused() {
    let rig = Rig::new("scratch-relative");
    let mut args = report(RUN, "1", "audit-install", "1");
    args.push("--scratch=scratchpad".to_owned());
    rig.run(&args, BODY)
        .refused(BAD_VALUE, "a relative scratch root");
}

// ---------------------------------------------------------------------------
// 6 · The hygiene stop
// ---------------------------------------------------------------------------

#[test]
fn every_writer_stops_hard_on_a_denylist_hit_and_never_says_what_matched() {
    for_every_writer("writer-denylist", |writer, rig| {
        // Case-insensitively, as the scanner matches.
        let term = DENY_TERM.to_uppercase();
        let before = rig.snapshot();
        let (seen, _) = (writer.call)(rig, &format!("a host called {term}"), &[]);
        seen.refused(HYGIENE, writer.name);
        assert!(
            !seen.stderr.to_lowercase().contains(DENY_TERM),
            "{}: a hit is named by where it is, never by what matched: {}",
            writer.name,
            seen.stderr
        );
        assert_eq!(rig.snapshot(), before, "{}: nothing written", writer.name);
    });
}

#[test]
fn every_writer_stops_hard_on_a_secret_shaped_string() {
    for_every_writer("writer-secret", |writer, rig| {
        let before = rig.snapshot();
        let (seen, _) = (writer.call)(rig, &format!("the token was {STUB_SECRET}"), &[]);
        seen.refused(HYGIENE, writer.name);
        assert!(
            seen.stderr.contains("stub-rule"),
            "{}: the rule that fired is named: {}",
            writer.name,
            seen.stderr
        );
        assert_eq!(rig.snapshot(), before, "{}: nothing written", writer.name);
    });
}

/// A path can leak a name as surely as a line can: a reporter named for a private term is
/// a hit, although its report says nothing.
#[test]
fn a_denylisted_term_in_the_reports_own_path_is_a_hit() {
    let rig = Rig::new("denylist-path");
    let before = rig.snapshot();
    rig.run(&report(RUN, "1", DENY_TERM, "1"), BODY)
        .refused(HYGIENE, "a reporter named for a denylisted term");
    assert_eq!(rig.snapshot(), before);
}

/// Called from inside a git hook, the environment names the caller's repository and its
/// index. The scan commits into a repository of its own, never into that one.
#[test]
fn the_scan_never_commits_into_the_callers_repository() {
    let mut rig = Rig::new("git-env");
    rig.init_repository();
    let git_dir = rig.root.join(".git");
    rig.env = vec![
        ("GIT_DIR", git_dir.to_string_lossy().into_owned()),
        ("GIT_WORK_TREE", rig.root.to_string_lossy().into_owned()),
        (
            "GIT_INDEX_FILE",
            git_dir.join("index").to_string_lossy().into_owned(),
        ),
    ];
    rig.run(&report(RUN, "1", "audit-install", "1"), BODY)
        .must(OK, "a report written from inside a hook's environment");

    rig.env.clear();
    let git = |args: &[&str]| {
        rig.hermetic(Command::new("git"))
            .args(args)
            .current_dir(&rig.root)
            .output()
            .expect("run git")
    };
    assert!(
        !git(&["rev-parse", "--verify", "--quiet", "HEAD"])
            .status
            .success(),
        "the caller's repository has no commit: the scan's went elsewhere"
    );
    assert!(
        git(&["ls-files", "--cached"]).stdout.is_empty(),
        "and nothing was staged in it"
    );
}

/// A way the scan can fail to run, and how the rig is brought there.
type ScanBreak = (&'static str, fn(&mut Rig));

const SCAN_BREAKS: &[ScanBreak] = &[
    ("gitleaks is not installed", Rig::without_gitleaks),
    ("gitleaks exits on an error of its own", |rig| {
        rig.env.push(("STUB_GITLEAKS", "crash".to_owned()));
    }),
    ("the machine has no denylist", |rig| {
        fs::remove_file(&rig.denylist).expect("remove the denylist");
    }),
    ("the denylist holds no pattern", |rig| {
        fs::write(&rig.denylist, "# nothing yet\n\n").expect("empty the denylist");
    }),
    ("a denylist line is not a regex", |rig| {
        fs::write(&rig.denylist, format!("{DENY_TERM}\na(b\n")).expect("break the denylist");
    }),
    ("the denylist scanner is gone", |rig| {
        fs::remove_file(rig.root.join(SCANNER)).expect("remove the scanner");
    }),
    ("the denylist scanner cannot be executed", |rig| {
        fs::set_permissions(rig.root.join(SCANNER), fs::Permissions::from_mode(0o644))
            .expect("chmod the scanner");
    }),
    ("git cannot build the repository the scan reads", |rig| {
        let git = rig.bin.join("git");
        fs::write(&git, "#!/bin/sh\necho 'stub git: no' >&2\nexit 128\n").expect("stub git");
        fs::set_permissions(&git, fs::Permissions::from_mode(0o755)).expect("chmod stub git");
    }),
];

#[test]
fn a_scanner_that_could_not_run_did_not_run_and_never_passed() {
    for (n, (why, breaker)) in SCAN_BREAKS.iter().enumerate() {
        for_every_writer(&format!("did-not-run-{n}"), |writer, rig| {
            breaker(rig);
            let what = format!("{}, when {why}", writer.name);
            let before = rig.snapshot();
            let (seen, _) = (writer.call)(rig, "nothing a scanner would stop", &[]);
            seen.refused(DID_NOT_RUN, &what);
            assert_eq!(rig.snapshot(), before, "{what}: nothing written");
        });
    }
}

/// The stub stands in for gitleaks' exit contract and says nothing about its rules. This
/// arm runs whatever the machine has: with gitleaks installed, a credential-shaped string
/// is a hit; without it, the scan did not run. On no machine is the report written.
#[test]
fn the_machines_own_gitleaks_never_lets_a_credential_through() {
    let rig = Rig::new("real-gitleaks");
    fs::remove_file(rig.bin.join("gitleaks")).expect("remove the stub");
    let installed = std::env::split_paths(&rig.path_tail).any(|dir| dir.join("gitleaks").is_file());

    // Built here, never spelled: a literal of this shape in a tracked file is a finding of
    // the very scan this arm drives.
    let mut state = 0x2545_f491_4f6c_dd1d_u64;
    let tail: String = (0..36)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            alphabet[(state >> 33) as usize % alphabet.len()] as char
        })
        .collect();
    let token = format!("{}{}_{tail}", "gh", 'p');

    let before = rig.snapshot();
    let seen = rig.run(
        &report(RUN, "1", "audit-install", "1"),
        &format!("# a report\n\nthe environment held {token}\n"),
    );
    let (status, what) = if installed {
        (HYGIENE, "gitleaks is installed: a credential is a hit")
    } else {
        (
            DID_NOT_RUN,
            "no gitleaks on this machine: the scan did not run",
        )
    };
    seen.refused(status, what);
    assert_eq!(rig.snapshot(), before, "{what}: nothing written");
}

// ---------------------------------------------------------------------------
// 7 · No file that would redden a standing fence
// ---------------------------------------------------------------------------

#[test]
fn a_report_that_would_redden_the_install_line_fence_is_refused_by_that_fences_name() {
    let rig = Rig::new("fence");
    rig.init_repository();
    let before = rig.snapshot();
    for (line, why) in [
        (
            format!("{INSTALL_COMMAND} --version '^1.0.0-rc.1' --locked"),
            "the line",
        ),
        (format!("    {INSTALL_COMMAND}"), "the line, indented"),
        (
            format!("\t{INSTALL_COMMAND} --locked\r"),
            "the line, with a tab and a CR",
        ),
        (
            format!("```sh\n{INSTALL_COMMAND} --locked\n```"),
            "the line, in a block",
        ),
    ] {
        let seen = rig.run(
            &report(RUN, "1", "audit-install", "1"),
            &format!("# the install proof\n\n{line}\n"),
        );
        seen.refused(FENCE, why);
        assert!(
            seen.stderr.contains(
                "install_line::the_install_line_has_one_owner_one_copy_and_one_derived_file"
            ),
            "{why}: the refusal names the fence: {}",
            seen.stderr
        );
        assert_eq!(rig.snapshot(), before, "{why}: nothing written");
    }

    // The same command where the fence does not read it is a report like any other — and
    // the fence's own census says so, over the tree the script wrote.
    let body =
        format!("# the install proof\n\nRan `{INSTALL_COMMAND} --locked`.\n$ {INSTALL_COMMAND}\n");
    rig.run(&report(RUN, "1", "audit-install", "1"), &body)
        .must(OK, "the command inside a line");
    assert_eq!(install_line_carriers(&rig.root), BTreeSet::new());
}

/// A table's line opens with a pipe, so a cell cannot give the fence a carrier whatever it
/// holds: the writers of cells take the line, and the census stays empty.
#[test]
fn no_table_cell_can_redden_the_install_line_fence() {
    for_every_writer("fence-cells", |writer, rig| {
        if writer.name.starts_with("report") {
            return;
        }
        rig.init_repository();
        let text = format!("ran:\n{INSTALL_COMMAND} --locked\n");
        let (seen, _) = (writer.call)(rig, &text, &[]);
        seen.must(OK, writer.name);
        assert_eq!(
            install_line_carriers(&rig.root),
            BTreeSet::new(),
            "{}: the census over the written table",
            writer.name
        );
    });
}

// ---------------------------------------------------------------------------
// 8 · A killed write leaves no partial file
// ---------------------------------------------------------------------------

/// Killed while its input is still arriving: nothing of the half that arrived is on disk.
#[test]
fn a_write_killed_while_its_input_arrives_leaves_nothing() {
    for (n, (args, half)) in [
        (
            report(RUN, "1", "audit-install", "1"),
            "# a report\n\nthe first half of a long",
        ),
        (ledger_add(RUN), "[{\"key\": \"audit-f3\", \"doctype\""),
    ]
    .into_iter()
    .enumerate()
    {
        let rig = Rig::new(&format!("killed-input-{n}"));
        let before = rig.snapshot();
        let mut child = rig.command(&args).spawn().expect("spawn the script");
        let mut stdin = child.stdin.take().expect("a piped stdin");
        stdin
            .write_all(half.as_bytes())
            .expect("write half the input");
        stdin.flush().expect("flush half the input");
        // Long enough for the script to be reading; a script that had streamed its input
        // to the path would have written the half by now.
        std::thread::sleep(Duration::from_millis(700));
        child.kill().expect("kill the script");
        let status = child.wait().expect("the script is gone");
        assert_eq!(status.signal(), Some(9), "the script was killed");
        drop(stdin);
        assert_eq!(rig.snapshot(), before, "`{}`: nothing on disk", args[0]);
    }
}

/// Killed at its last step before the file exists — the scan — with the whole content in
/// hand: no file at the path, no temporary beside it, and a table that existed is the
/// table it was.
#[test]
fn a_write_killed_at_its_last_step_leaves_nothing() {
    for_every_writer("killed-scan", |writer, rig| {
        rig.env.push(("STUB_GITLEAKS", "kill-parent".to_owned()));
        let before = rig.snapshot();
        let (seen, _) = (writer.call)(rig, "a whole and harmless text", &[]);
        assert_eq!(
            (seen.code, seen.signal),
            (None, Some(9)),
            "{}: the script was killed mid-run: {seen:?}",
            writer.name
        );
        assert_eq!(rig.snapshot(), before, "{}: nothing on disk", writer.name);
    });
}

// ---------------------------------------------------------------------------
// 9 · The ledger
// ---------------------------------------------------------------------------

#[test]
fn the_ledger_takes_one_row_per_finding_under_the_columns_the_ruling_names() {
    let rig = Rig::new("ledger-rows");
    let mut second = row("audit-f4");
    second["doctype"] = json!("inconsistency");
    second["round"] = json!(0);
    let batch = json!([row("audit-f3"), second]);

    let seen = rig.run(&ledger_add(RUN), &batch.to_string());
    seen.must(OK, "two rows");
    assert_eq!(
        seen.json(),
        json!({"ledger": ledger_path(), "appended": ["audit-f3", "audit-f4"], "existing": []})
    );
    let ledger = rig.read(&ledger_path());
    let rows = table(&ledger, &LEDGER_COLUMNS);
    assert_eq!(
        rows,
        vec![
            vec![
                "`audit-f3`",
                "`jigc-feedback`",
                "1",
                "audit-install, finding 3",
                "jigc setup",
                "no-lost-files",
                "ungraded",
                "-",
                "open",
                "r1/reports/test/audit-install.a1.md, the third block",
            ],
            vec![
                "`audit-f4`",
                "`inconsistency`",
                "0",
                "audit-install, finding 3",
                "jigc setup",
                "no-lost-files",
                "ungraded",
                "-",
                "open",
                "r1/reports/test/audit-install.a1.md, the third block",
            ],
        ]
    );

    // A key that exists is not appended again, whatever the second row says: a finding
    // found again inherits its row.
    let mut again = row("audit-f3");
    again["source"] = json!("found again by another auditor");
    let batch = json!([again, row("audit-f5"), row("audit-f5")]);
    let seen = rig.run(&ledger_add(RUN), &batch.to_string());
    seen.must(OK, "a key that exists, and one given twice");
    assert_eq!(
        seen.json(),
        json!({
            "ledger": ledger_path(),
            "appended": ["audit-f5"],
            "existing": ["audit-f3", "audit-f5"],
        })
    );
    let after = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    assert_eq!(after.len(), 3, "one row per key");
    assert_eq!(after[..2], rows[..], "the rows that existed are untouched");

    // A batch of nothing new writes nothing.
    let before = rig.snapshot();
    rig.run(&ledger_add(RUN), &row("audit-f3").to_string())
        .must(OK, "a row that exists, alone");
    assert_eq!(rig.snapshot(), before);
}

/// Every free-text position of a row, with the text that would break a table.
#[test]
fn no_cell_can_break_the_table() {
    let hostile = "a | b\nsecond line\r\n| `injected` | x |\n|---|---|\ttab \\| and \\\\| done \\";
    let rig = Rig::new("ledger-cells");
    let mut expected = Vec::new();
    for (n, field) in ["source", "door", "clause", "repro"].iter().enumerate() {
        let key = format!("hostile-{n}");
        let mut entry = row(&key);
        entry[*field] = json!(hostile);
        rig.run(&ledger_add(RUN), &entry.to_string())
            .must(OK, &format!("a hostile `{field}`"));
        expected.push(key);
    }
    // A disposition's detail and a clause row's two cells are free text too.
    let patch = json!({"key": "hostile-0", "disposition": "bound", "detail": hostile});
    rig.run(&ledger_set(RUN), &patch.to_string())
        .must(OK, "a hostile disposition detail");
    let mut clause = clause_set(RUN, "no-lost-files", &["--status=void"]);
    clause.push(format!("--instrument={hostile}"));
    clause.push(format!("--scope={hostile}"));
    rig.run(&clause, "").must(OK, "hostile clause cells");

    let rows = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    let keys: Vec<String> = rows
        .iter()
        .map(|r| r[0].trim_matches('`').to_owned())
        .collect();
    assert_eq!(
        keys, expected,
        "one row per finding, and no row a cell invented"
    );
    for (n, field) in [3usize, 4, 5, 9].iter().enumerate() {
        let cell = &rows[n][*field];
        assert!(
            cell.starts_with("a | b<br>second line<br>| `injected` | x |<br>|---|---| tab "),
            "the cell keeps what it was given, its line breaks as `<br>`: {cell}"
        );
    }
    let clauses = table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS);
    assert_eq!(clauses.len(), 1, "one clause row");

    // And the script reads its own table back the same way.
    let refs: Vec<&str> = expected.iter().map(String::as_str).collect();
    let seen = rig.run(&check_ledger(RUN, &refs), "");
    seen.must(OK, "every hostile row is one row");
    rig.run(&check_ledger(RUN, &["injected"]), "").must(
        LEDGER_MISMATCH,
        "the row a cell tried to inject does not exist",
    );
}

#[test]
fn a_rows_grade_and_disposition_are_set_later_from_a_closed_vocabulary() {
    let rig = Rig::new("ledger-set");

    // Every grade its grader may give, and every disposition, with the cell it becomes.
    let sayable: &[(Value, usize, &str)] = &[
        (
            json!({"grade": "unclear", "graded_by": "triage"}),
            6,
            "unclear",
        ),
        (
            json!({"grade": "no-break", "graded_by": "triage"}),
            6,
            "no-break",
        ),
        (
            json!({"grade": "out-of-scope", "graded_by": "triage", "bound": "non-jigc-writer"}),
            6,
            "out-of-scope: non-jigc-writer",
        ),
        (
            json!({"grade": "confirmed", "graded_by": "verify-real"}),
            6,
            "confirmed",
        ),
        (
            json!({"grade": "refuted", "graded_by": "verify-real"}),
            6,
            "refuted",
        ),
        (json!({"grade": "breaks", "graded_by": "human"}), 7, "human"),
        (
            json!({"grade": "refuted", "graded_by": "human"}),
            7,
            "human",
        ),
        (json!({"grade": "ungraded"}), 7, "-"),
        (json!({"disposition": "admitted"}), 8, "admitted"),
        (
            json!({"disposition": "bound", "detail": "races against a writer that is not jigc"}),
            8,
            "bound: races against a writer that is not jigc",
        ),
        (json!({"disposition": "later"}), 8, "later"),
        (
            json!({"disposition": "later", "detail": "the 1.x fix pass"}),
            8,
            "later: the 1.x fix pass",
        ),
        (json!({"disposition": "no-action"}), 8, "no-action"),
        (json!({"disposition": "open"}), 8, "open"),
    ];

    // The rows: one to walk through a finding's life, one per sayable value — each already
    // graded and disposed of, so that every patch is seen to change its cell — and one
    // that nothing touches.
    let mut seeds = vec![row("audit-f3")];
    for n in 0..sayable.len() {
        let mut seed = row(&format!("say-{n}"));
        seed["grade"] = json!("breaks");
        seed["graded_by"] = json!("triage");
        seed["disposition"] = json!("no-action");
        seed["detail"] = json!("as seeded");
        seeds.push(seed);
    }
    seeds.push(row("audit-f5"));
    rig.run(&ledger_add(RUN), &json!(seeds).to_string()).must(
        OK,
        "the rows, some graded and disposed of as they are added",
    );
    let seeded = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    assert_eq!(
        seeded[1][6..9],
        ["breaks", "triage", "no-action: as seeded"],
        "a row may be graded and disposed of as it is added"
    );
    let untouched = seeded.last().expect("the last row").clone();

    // Triage grades; the verifier's verdict replaces the grade; a fix's sha closes the row.
    let steps: &[(Value, &str, &str, &str)] = &[
        (
            json!({"key": "audit-f3", "grade": "breaks", "graded_by": "triage"}),
            "breaks",
            "triage",
            "open",
        ),
        (
            json!({"key": "audit-f3", "grade": "regression", "graded_by": "verify-real"}),
            "regression",
            "verify-real",
            "open",
        ),
        (
            json!({"key": "audit-f3", "disposition": "fixed", "detail": "0f34d8f0"}),
            "regression",
            "verify-real",
            "fixed: 0f34d8f0",
        ),
    ];
    for (patch, grade, by, disposition) in steps {
        let seen = rig.run(&ledger_set(RUN), &patch.to_string());
        seen.must(OK, &patch.to_string());
        assert_eq!(
            seen.json(),
            json!({"ledger": ledger_path(), "updated": ["audit-f3"]})
        );
        let rows = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
        assert_eq!(
            (
                rows[0][6].as_str(),
                rows[0][7].as_str(),
                rows[0][8].as_str()
            ),
            (*grade, *by, *disposition)
        );
        assert_eq!(
            rows.last(),
            Some(&untouched),
            "another row is never touched"
        );
    }

    // Every sayable value, each on a row of its own, as one batch.
    let keys: Vec<String> = (0..sayable.len()).map(|n| format!("say-{n}")).collect();
    let batch: Vec<Value> = sayable
        .iter()
        .zip(&keys)
        .map(|((fields, _, _), key)| {
            let mut patch = fields.clone();
            patch["key"] = json!(key);
            patch
        })
        .collect();
    let seen = rig.run(&ledger_set(RUN), &json!(batch).to_string());
    seen.must(OK, "every sayable grade and disposition");
    assert_eq!(
        seen.json(),
        json!({"ledger": ledger_path(), "updated": keys})
    );
    let rows = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    for (n, (fields, column, cell)) in sayable.iter().enumerate() {
        assert_eq!(rows[n + 1][*column], *cell, "after {fields}");
    }
    assert_eq!(
        rows.last(),
        Some(&untouched),
        "another row is never touched"
    );

    // A patch for a row the ledger lacks changes nothing, and neither does its batch.
    let before = rig.snapshot();
    let batch = json!([
        {"key": "audit-f5", "disposition": "later"},
        {"key": "never-filed", "disposition": "later"},
    ]);
    rig.run(&ledger_set(RUN), &batch.to_string())
        .refused(NO_SUCH_ROW, "a patch for a key the ledger lacks");
    assert_eq!(rig.snapshot(), before, "all of a batch, or none of it");
}

/// What a row may not say. Each is refused as `bad-value`, and its whole batch with it.
#[test]
fn a_value_outside_the_vocabulary_is_refused_and_nothing_of_its_batch_is_written() {
    let rig = Rig::new("ledger-vocabulary");
    rig.seed_rows(&["seeded"]);
    let before = rig.snapshot();

    let with = |field: &str, value: Value| {
        let mut entry = row("audit-f3");
        entry[field] = value;
        entry
    };
    let without = |field: &str| {
        let mut entry = row("audit-f3");
        entry.as_object_mut().expect("a row object").remove(field);
        entry
    };
    let graded = |grade: &str, by: Value| {
        let mut entry = row("audit-f3");
        entry["grade"] = json!(grade);
        entry["graded_by"] = by;
        entry
    };
    let rows: Vec<(Value, &str)> = vec![
        (
            with("doctype", json!("adr")),
            "a doctype that is neither of the two",
        ),
        (with("round", json!(-1)), "a negative round"),
        (with("round", json!("1")), "a round that is a string"),
        (with("round", json!(1.5)), "a round that is a fraction"),
        (with("round", json!(true)), "a round that is a boolean"),
        (with("source", json!(7)), "a cell that is not text"),
        (with("dispositon", json!("open")), "a field nobody knows"),
        (without("door"), "a row with no door"),
        (without("repro"), "a row with no repro pointer"),
        (
            graded("severe", json!("triage")),
            "a grade outside the vocabulary",
        ),
        (
            graded("breaks", json!("the-fixer")),
            "a grader outside the vocabulary",
        ),
        (graded("breaks", Value::Null), "a grade nobody gave"),
        (
            graded("confirmed", json!("triage")),
            "a verdict given by triage",
        ),
        (
            graded("breaks", json!("verify-real")),
            "a triage grade given by the verifier",
        ),
        (
            graded("ungraded", json!("triage")),
            "nothing, graded by somebody",
        ),
        (
            graded("out-of-scope", json!("triage")),
            "out of scope, citing no bound",
        ),
        (
            {
                let mut entry = graded("breaks", json!("triage"));
                entry["bound"] = json!("non-jigc-writer");
                entry
            },
            "a bound cited by a grade that is not out of scope",
        ),
        (
            with("disposition", json!("wontfix")),
            "a disposition outside the vocabulary",
        ),
        (with("disposition", json!("fixed")), "fixed, by no commit"),
        (
            {
                let mut entry = with("disposition", json!("fixed"));
                entry["detail"] = json!("the last commit");
                entry
            },
            "fixed, by something that is not a sha",
        ),
        (with("disposition", json!("bound")), "a bound with no reach"),
        (
            {
                let mut entry = with("disposition", json!("open"));
                entry["detail"] = json!("why");
                entry
            },
            "open, with a detail",
        ),
        (
            with("detail", json!("of nothing")),
            "a detail with no disposition",
        ),
    ];
    for (entry, why) in &rows {
        let batch = json!([row("audit-f9"), entry]);
        rig.run(&ledger_add(RUN), &batch.to_string())
            .refused(BAD_VALUE, why);
        assert_eq!(rig.snapshot(), before, "{why}: nothing of the batch");
    }
    for (input, why) in [
        ("", "no input"),
        ("not json", "input that is not JSON"),
        ("[]", "an empty batch"),
        ("[1]", "a row that is not an object"),
        ("\"audit-f3\"", "a batch that is a string"),
    ] {
        rig.run(&ledger_add(RUN), input).refused(BAD_VALUE, why);
        rig.run(&ledger_set(RUN), input).refused(BAD_VALUE, why);
    }

    let patches: Vec<(Value, &str)> = vec![
        (json!({"key": "seeded"}), "a patch that sets nothing"),
        (
            json!({"key": "seeded", "grade": "breaks"}),
            "a grade nobody gave",
        ),
        (
            json!({"key": "seeded", "graded_by": "triage"}),
            "a grader of nothing",
        ),
        (
            json!({"key": "seeded", "grade": "severe", "graded_by": "human"}),
            "a grade",
        ),
        (
            json!({"key": "seeded", "disposition": "fixed", "detail": "xyz"}),
            "a sha",
        ),
        (
            json!({"key": "seeded", "disposition": "later", "source": "x"}),
            "a column not set later",
        ),
        (json!({"disposition": "later"}), "a patch with no key"),
    ];
    for (patch, why) in &patches {
        rig.run(&ledger_set(RUN), &patch.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);
}

/// A table is read, changed and replaced. Writers that arrive at once queue, so each
/// one's rows are there when all are done — whichever order they ran in.
#[test]
fn rows_written_at_once_are_all_written() {
    let rig = Rig::new("at-once");
    let keys: Vec<String> = (0..6).map(|n| format!("audit-f{n}")).collect();
    let clauses: Vec<String> = (0..3).map(|n| format!("clause-{n}")).collect();
    std::thread::scope(|scope| {
        for key in &keys {
            let rig = &rig;
            scope.spawn(move || {
                rig.run(&ledger_add(RUN), &row(key).to_string())
                    .must(OK, key);
            });
        }
        for clause in &clauses {
            let rig = &rig;
            scope.spawn(move || {
                let rest = [
                    "--instrument=the audit",
                    "--scope=the delta",
                    "--status=void",
                ];
                rig.run(&clause_set(RUN, clause, &rest), "")
                    .must(OK, clause);
            });
        }
    });
    let first_cells = |text: String, columns: &[&str]| -> BTreeSet<String> {
        let rows = table(&text, columns);
        let cells: BTreeSet<String> = rows
            .iter()
            .map(|r| r[0].trim_matches('`').to_owned())
            .collect();
        assert_eq!(cells.len(), rows.len(), "one row each:\n{text}");
        cells
    };
    assert_eq!(
        first_cells(rig.read(&ledger_path()), &LEDGER_COLUMNS),
        keys.into_iter().collect()
    );
    assert_eq!(
        first_cells(rig.read(&clauses_path()), &CLAUSE_COLUMNS),
        clauses.into_iter().collect()
    );
}

/// The lock is taken once a writer has its input, never while it waits for it: a caller
/// that is slow to send its rows holds nobody up. Held the other way round, two writers
/// of which each holds the other's input pipe open wait on each other for good — driven
/// by hand, and the one explanation found for a hang of the arm above, seen once.
#[test]
fn a_writer_waiting_for_its_input_holds_nobody_up() {
    let rig = Rig::new("slow-input");
    let mut slow = rig
        .command(&ledger_add(RUN))
        .spawn()
        .expect("spawn the slow writer");
    let mut input = slow.stdin.take().expect("a piped stdin");
    input.write_all(b"[").expect("open the batch");
    input.flush().expect("flush the batch's opening");
    // Long enough for the slow writer to be waiting for the rest.
    std::thread::sleep(Duration::from_millis(700));

    let mut quick = rig
        .command(&ledger_add(RUN))
        .spawn()
        .expect("spawn the quick writer");
    child_stdin::feed(&mut quick, row("audit-f4").to_string());
    let patience = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = quick.try_wait().expect("ask after the quick writer") {
            break status;
        }
        if Instant::now() > patience {
            let _ = quick.kill();
            let _ = slow.kill();
            panic!("a writer still waiting for its input held the lock for 30 s");
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    assert!(status.success(), "the quick writer wrote: {status:?}");

    input
        .write_all(format!("{}]", row("audit-f3")).as_bytes())
        .expect("send the rest of the batch");
    drop(input);
    let out = slow.wait_with_output().expect("the slow writer exits");
    assert!(out.status.success(), "the slow writer wrote: {out:?}");
    let rows = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    let keys: Vec<&str> = rows.iter().map(|r| r[0].as_str()).collect();
    assert_eq!(
        keys,
        ["`audit-f4`", "`audit-f3`"],
        "both, in the order they wrote"
    );
}

/// A ledger the script cannot read as its own table is never rewritten on a guess.
#[test]
fn a_table_the_script_did_not_write_is_refused_as_corrupt_and_left_alone() {
    type Damage = (&'static str, fn(&str) -> String);
    let damage: &[Damage] = &[
        ("a row that lost a cell", |text| {
            text.replacen("| `jigc-feedback` ", "", 1)
        }),
        ("prose after the table", |text| {
            format!("{text}\nA note somebody added by hand.\n")
        }),
        ("a header somebody renamed", |text| {
            text.replacen("| clause broken |", "| clause |", 1)
        }),
        ("no table at all", |_| "# a ledger\n".to_owned()),
    ];
    // One ledger as the script writes it, damaged in each way.
    let written = {
        let rig = Rig::new("corrupt-source");
        rig.seed_rows(&["seeded"]);
        rig.read(&ledger_path())
    };
    for (n, (why, damaged)) in damage.iter().enumerate() {
        let rig = Rig::new(&format!("corrupt-{n}"));
        fs::write(rig.root.join(ledger_path()), damaged(&written)).expect("a damaged ledger");
        let before = rig.snapshot();
        let patch = json!({"key": "seeded", "disposition": "later"});
        rig.run(&ledger_add(RUN), &row("audit-f3").to_string())
            .refused(CORRUPT, &format!("ledger-add over {why}"));
        rig.run(&ledger_set(RUN), &patch.to_string())
            .refused(CORRUPT, &format!("ledger-set over {why}"));
        rig.run(&check_ledger(RUN, &["seeded"]), "")
            .refused(CORRUPT, &format!("check-ledger over {why}"));
        assert_eq!(rig.snapshot(), before, "{why}: the file is left as it was");
    }
}

// ---------------------------------------------------------------------------
// 10 · The per-clause table
// ---------------------------------------------------------------------------

#[test]
fn the_clause_table_is_written_and_updated_row_by_row() {
    let rig = Rig::new("clauses");
    let opening = [
        "--instrument=the audit of the fix diff",
        "--scope=product paths of the delta",
        "--status=void",
    ];
    let seen = rig.run(&clause_set(RUN, "no-lost-files", &opening), "");
    seen.must(OK, "a clause's first row");
    assert_eq!(
        seen.json(),
        json!({"clauses": clauses_path(), "clause": "no-lost-files", "status": "void"})
    );
    let rest = [
        "--instrument=the scripted regression set",
        "--scope=everything",
        "--status=void",
    ];
    rig.run(&clause_set(RUN, "no-regression", &rest), "")
        .must(OK, "a second clause");
    assert_eq!(
        table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS),
        vec![
            vec![
                "`no-lost-files`",
                "the audit of the fix diff",
                "-",
                "product paths of the delta",
                "void",
            ],
            vec![
                "`no-regression`",
                "the scripted regression set",
                "-",
                "everything",
                "void",
            ],
        ]
    );

    // The instrument ran: its commit and its verdict, and nothing else of the row.
    rig.run(
        &clause_set(RUN, "no-lost-files", &["--commit=0f34d8f0", "--status=red"]),
        "",
    )
    .must(OK, "the instrument ran red");
    rig.run(
        &clause_set(
            RUN,
            "no-lost-files",
            &[
                "--commit=967ca491",
                "--status=green",
                "--scope=the doors of round 2",
            ],
        ),
        "",
    )
    .must(OK, "and then green, over another scope");
    assert_eq!(
        table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS),
        vec![
            vec![
                "`no-lost-files`",
                "the audit of the fix diff",
                "`967ca491`",
                "the doors of round 2",
                "green",
            ],
            vec![
                "`no-regression`",
                "the scripted regression set",
                "-",
                "everything",
                "void",
            ],
        ]
    );

    let before = rig.snapshot();
    let refused: &[(&[&str], &str, i32)] = &[
        (
            &["--status=passed"],
            "a status outside green, red and void",
            BAD_VALUE,
        ),
        (
            &["--commit=HEAD", "--status=red"],
            "a commit that is not a sha",
            BAD_VALUE,
        ),
        (
            &["--commit=-", "--status=green"],
            "green on no commit",
            BAD_VALUE,
        ),
        (&[], "an update that sets nothing", USAGE),
    ];
    for (rest, why, status) in refused {
        rig.run(&clause_set(RUN, "no-lost-files", rest), "")
            .refused(*status, why);
    }
    // A verdict needs a commit it was reached on, and a new row needs all it must say.
    rig.run(&clause_set(RUN, "no-regression", &["--status=green"]), "")
        .refused(BAD_VALUE, "green, on a row that never ran");
    rig.run(&clause_set(RUN, "a-third", &["--status=void"]), "")
        .refused(BAD_VALUE, "a new row with no instrument and no scope");
    assert_eq!(rig.snapshot(), before);

    // Void is sayable of a commit: the instrument could not run on it.
    rig.run(
        &clause_set(
            RUN,
            "no-regression",
            &["--commit=967ca491", "--status=void"],
        ),
        "",
    )
    .must(OK, "could not run on a commit");
}

// ---------------------------------------------------------------------------
// 11 · The two one-to-one checks
// ---------------------------------------------------------------------------

#[test]
fn check_reports_holds_a_stage_to_one_report_per_launched_reporter() {
    let rig = Rig::new("check-reports");
    let launched = ["audit-install", "audit-worktree", "review-source"];
    let dir = format!("completions/artifacts/{RUN}/r1/reports/test");

    // Nothing written yet: every reporter is missing.
    let seen = rig.run(&check_reports(RUN, "1", "1", &launched), "");
    seen.must(REPORTS_MISMATCH, "before any report");
    assert_eq!(seen.json()["missing"], json!(launched));

    for reporter in launched {
        rig.run(&report(RUN, "1", reporter, "1"), BODY)
            .must(OK, reporter);
    }
    let seen = rig.run(&check_reports(RUN, "1", "1", &launched), "");
    seen.must(OK, "one report each");
    assert_eq!(
        seen.json(),
        json!({
            "check": "reports",
            "ok": true,
            "dir": dir,
            "attempt": 1,
            "missing": [],
            "extra": [],
            "other_attempts": [],
        })
    );

    // One reporter fewer was launched: its report is an extra one, and it is named.
    let seen = rig.run(&check_reports(RUN, "1", "1", &launched[..2]), "");
    seen.must(REPORTS_MISMATCH, "a report nobody launched");
    assert_eq!(seen.json()["ok"], json!(false));
    assert_eq!(seen.json()["extra"], json!(["review-source.a1.md"]));
    assert_eq!(seen.json()["missing"], json!([]));

    // One more was launched and wrote nothing: it is missing, and it is named.
    let mut more = launched.to_vec();
    more.push("trial-arm");
    let seen = rig.run(&check_reports(RUN, "1", "1", &more), "");
    seen.must(REPORTS_MISMATCH, "a reporter that wrote nothing");
    assert_eq!(seen.json()["missing"], json!(["trial-arm"]));
    assert_eq!(seen.json()["extra"], json!([]));

    // A re-run's attempt is judged on its own files; the earlier attempt's are listed, and
    // are neither a pass for it nor a failure of it.
    let seen = rig.run(&check_reports(RUN, "1", "2", &launched), "");
    seen.must(REPORTS_MISMATCH, "attempt 2, before its reports");
    assert_eq!(seen.json()["missing"], json!(launched));
    assert_eq!(
        seen.json()["other_attempts"].as_array().map(Vec::len),
        Some(3)
    );
    for reporter in launched {
        rig.run(&report(RUN, "1", reporter, "2"), BODY)
            .must(OK, reporter);
    }
    rig.run(&check_reports(RUN, "1", "2", &launched), "")
        .must(OK, "attempt 2, complete");

    // Whatever else lies in the stage's directory is an extra file: a note by hand, a
    // directory, the temporary of a write that was killed.
    let stage = rig.root.join(&dir);
    fs::write(stage.join("notes.txt"), "by hand\n").expect("a stray file");
    fs::write(stage.join(".stabilize-record.x1.tmp"), "half").expect("a stray temporary");
    fs::create_dir(stage.join("audit-extra.a2.md")).expect("a directory named as a report");
    let seen = rig.run(&check_reports(RUN, "1", "2", &launched), "");
    seen.must(REPORTS_MISMATCH, "strays in the stage's directory");
    assert_eq!(
        seen.json()["extra"],
        json!([".stabilize-record.x1.tmp", "audit-extra.a2.md", "notes.txt"])
    );

    // A stage that launched nobody passes on an empty directory, and a reporter named
    // twice is not a set of reporters.
    rig.run(&check_reports(RUN, "7", "1", &[]), "")
        .must(OK, "nobody launched, nothing written");
    rig.run(
        &check_reports(RUN, "1", "1", &["audit-install", "audit-install"]),
        "",
    )
    .refused(BAD_VALUE, "a reporter launched twice");
}

#[test]
fn check_ledger_holds_a_stage_to_one_row_per_finding_id() {
    let rig = Rig::new("check-ledger");

    // No ledger yet: every finding is missing.
    let seen = rig.run(&check_ledger(RUN, &["audit-f3"]), "");
    seen.must(LEDGER_MISMATCH, "before any row");
    assert_eq!(seen.json()["missing"], json!(["audit-f3"]));

    rig.seed_rows(&["audit-f3", "audit-f4", "audit-f5"]);
    let seen = rig.run(&check_ledger(RUN, &["audit-f3", "audit-f5"]), "");
    seen.must(OK, "one row each");
    assert_eq!(
        seen.json(),
        json!({"check": "ledger", "ok": true, "missing": [], "duplicated": []})
    );

    let seen = rig.run(
        &check_ledger(RUN, &["audit-f3", "audit-f6", "audit-f7"]),
        "",
    );
    seen.must(LEDGER_MISMATCH, "two findings with no row");
    assert_eq!(
        seen.json(),
        json!({
            "check": "ledger",
            "ok": false,
            "missing": ["audit-f6", "audit-f7"],
            "duplicated": [],
        })
    );

    // A second row for one finding, put there by hand: named, and nothing the script
    // writes over.
    let path = rig.root.join(ledger_path());
    let text = fs::read_to_string(&path).expect("read the ledger");
    let twin = text
        .lines()
        .find(|line| line.starts_with("| `audit-f4`"))
        .expect("the row to double")
        .to_owned();
    fs::write(&path, format!("{text}{twin}\n")).expect("double a row");
    let seen = rig.run(&check_ledger(RUN, &["audit-f3", "audit-f4"]), "");
    seen.must(LEDGER_MISMATCH, "a finding with two rows");
    assert_eq!(seen.json()["duplicated"], json!(["audit-f4"]));
    assert_eq!(seen.json()["missing"], json!([]));
    let before = rig.snapshot();
    rig.run(&ledger_add(RUN), &row("audit-f9").to_string())
        .refused(CORRUPT, "an append to a ledger with a doubled key");
    assert_eq!(rig.snapshot(), before);

    rig.run(&check_ledger(RUN, &["audit-f3", "audit-f3"]), "")
        .refused(BAD_VALUE, "a finding id given twice");
}

// ---------------------------------------------------------------------------
// 12 · The header is the documentation
// ---------------------------------------------------------------------------

#[test]
fn help_is_the_header_and_states_every_subcommand_and_every_exit_status() {
    let mut rig = Rig::new("help");
    let seen = rig.run(&strings(&["--help"]), "");
    seen.must(OK, "--help");
    let subcommands = subcommands(&rig);
    assert!(subcommands.len() >= 6, "the parser names its subcommands");
    for subcommand in subcommands {
        assert!(
            seen.stdout
                .contains(&format!("dev/stabilize-record {subcommand} ")),
            "--help shows `{subcommand}`'s usage:\n{}",
            seen.stdout
        );
    }
    let mut statuses: Vec<(i32, &str)> = REFUSALS.to_vec();
    statuses.push((REPORTS_MISMATCH, "check-reports"));
    statuses.push((LEDGER_MISMATCH, "check-ledger"));
    for (status, class) in statuses {
        assert!(
            seen.stdout
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{status}  {class}"))),
            "--help states exit {status} as `{class}`:\n{}",
            seen.stdout
        );
    }
    assert!(
        seen.stdout.contains("opening.md"),
        "--help says what an opened run is"
    );

    for (args, why) in [
        (strings(&[]), "no subcommand"),
        (strings(&["report", "--run=rc24"]), "a report with no round"),
        (
            strings(&["ledger-add", "--run=rc24", "--force"]),
            "a flag nobody knows",
        ),
    ] {
        rig.run(&args, "").refused(USAGE, why);
    }

    // The same bytes whatever encoding the caller's terminal claims: a result is read by
    // a script, and the header and a refusal both carry characters ASCII lacks.
    rig.env.push(("PYTHONIOENCODING", "ascii".to_owned()));
    let ascii = rig.run(&strings(&["--help"]), "");
    ascii.must(OK, "--help under a terminal that claims ASCII");
    assert_eq!(ascii.stdout, seen.stdout, "the header, byte for byte");
    let refusal = rig.run(&strings(&["no-such-subcommand"]), "");
    refusal.refused(USAGE, "a refusal under a terminal that claims ASCII");
    assert!(
        refusal.stderr.contains(" \u{2014} "),
        "the refusal's dash arrives as itself: {}",
        refusal.stderr
    );
}
