//! **`dev/stabilize-record` — the one script through which a stabilization run's records
//! reach the repository, and the one place its state is read back and decided**
//! ([DECISIONS.md](../../../DECISIONS.md) → *2026-10-05 — The stabilization workflow, as
//! ruled*, rulings 4, 5, 9, 10, 12, 13 and 16; the builders' choices are the two entries of
//! the same date, *The record script, as built* and *The record script reads the run
//! back*).
//!
//! A reporter of a stage has a shell and no write tool, and what it writes is public on the
//! next push and read by fences on the next gate. So one script decides where a report
//! lands, what a ledger row may say and what is refused, and this suite holds it to that by
//! **driving the script's own bytes** — copied into a throwaway repository, because the
//! script finds its repository from where it lies — and reading what it left on disk.
//!
//! And the workflow's harness sees only what an agent returns to it. So the run's state
//! leaves the script as one JSON document, and what follows from it — whether a finding is
//! inside the round's test set, where it is routed, what the round does next — is computed
//! by the script. **Two truth tables hold those decisions** ([`findings`], [`ROUNDS`]):
//! each cell is stood up through the script's own writers, never as a table this suite
//! wrote by hand, and read off the state document. A cell the rulings do not settle is in
//! the tables too, as `unsettled` — the script's word for *not decided here*.
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
//! - *The writers.* Seven subcommands write a file ([`WRITERS`]), and six of them put a
//!   caller's free text into it: a report's body, a ledger row's cells as it is added and
//!   as it is set, a clause row's cells, a bound's, a door's derivation. The scanner that
//!   could not run and the killed write are driven through all seven; the host-path
//!   replacement and the hygiene stop through the six. The list is held to the script's
//!   own parser — every subcommand it names is a writer on the list or one of the three
//!   that write nothing ([`every_subcommand_is_a_listed_writer_or_a_check`]) — so a writer
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
//!
//! And on sixty-two more, for what the script reads back and decides. *The route:* a
//! regression not fixed unasked; an inside finding sent to the human and an outside one
//! fixed unasked; a break no round placed fixed unasked; `admitted` not fixed; `bound`,
//! `later` each no ruling, and `no-action` one; `fixed` inherited by a finding found again,
//! in each of its five shapes; an ungraded and an unverified finding recorded, or fixed;
//! *refuted* and *no-break* each read as a break; *out-of-scope* recorded without a listed
//! bound, or never; `needs-bound` recorded; the first round that triaged a finding placing
//! it. *The next step:* each pair of its precedence swapped; a void, a red and a missing
//! clause row each no bar to closing, and an open finding none; the state with no step
//! closed, and left unnamed. *Inside:* an excluded and an unlisted door each inside;
//! `inside` supplied; a triage record not held to its doors. *The writers:* a bound cited
//! off the list, through each of the three that take a grade; a bound entered with no
//! ruling, or a blank cell; a scope replaced, a door on both sides, a blank door cell; a
//! round with no scope and a finding with no row triaged; the inherited disposition not
//! recorded; a regression recorded as confirmed; the ledger's grade not following; a
//! verdict on any grade, with no word on regression, twice in a batch; a cut-off report
//! taken, and its last line taken from anywhere. *The state:* a run never opened refused;
//! a write in flight not waited for; a cell returned escaped; a grade, a status, a
//! disposition, a side written by hand each read, and a triage record with no scope.

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
const NO_SUCH_BOUND: i32 = 14;
const NO_SCOPE: i32 = 15;
const TRUNCATED: i32 = 16;
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
    (NO_SUCH_BOUND, "no-such-bound"),
    (NO_SCOPE, "no-scope"),
    (TRUNCATED, "truncated"),
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

const BOUND_COLUMNS: [&str; 4] = ["bound", "reach", "ruling", "pin"];

const SCOPE_COLUMNS: [&str; 4] = ["door", "test set", "registry", "derivation"];

const TRIAGE_COLUMNS: [&str; 6] = [
    "key",
    "inside",
    "triage",
    "verdict",
    "regression",
    "found with",
];

/// The line a report ends with: a report that does not was cut off on its way.
const ENDS: &str = "<!-- end of report -->";

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
/// before the file exists — and `slow` scans as ever, two seconds late, which holds a
/// write in flight for as long.
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
    slow) sleep 2 ;;
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

    /// The run's state document.
    fn state(&self) -> Value {
        let seen = self.run(&state(RUN), "");
        seen.must(OK, "state");
        assert_eq!(
            seen.stdout.lines().count(),
            1,
            "the state is one line of JSON: {}",
            seen.stdout
        );
        seen.json()
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

/// `bound-set` for the bound `bound`, with the cells `rest` names.
fn bound_set(run: &str, bound: &str, rest: &[&str]) -> Vec<String> {
    let mut args = vec![
        "bound-set".to_owned(),
        format!("--run={run}"),
        format!("--bound={bound}"),
    ];
    args.extend(strings(rest));
    args
}

/// The cells a bound's first row must name.
const BOUND: [&str; 3] = [
    "--reach=races against a writer that is not jigc",
    "--ruling=the opening record, declared bounds",
    "--pin=unpinned",
];

fn scope_set(run: &str, round: &str) -> Vec<String> {
    vec![
        "scope-set".to_owned(),
        format!("--run={run}"),
        format!("--round={round}"),
    ]
}

fn triage_set(run: &str, round: &str) -> Vec<String> {
    vec![
        "triage-set".to_owned(),
        format!("--run={run}"),
        format!("--round={round}"),
    ]
}

fn state(run: &str) -> Vec<String> {
    vec!["state".to_owned(), format!("--run={run}")]
}

/// A door as the scope step hands it over.
fn door(name: &str) -> Value {
    json!({
        "door": name,
        "registry": "the verb table",
        "derivation": format!("a changed symbol is read by `{name}`"),
    })
}

/// A round's scope: the doors inside its test set, and the doors left out of it.
fn scope(included: &[&str], excluded: &[&str]) -> Value {
    json!({
        "included": included.iter().map(|name| door(name)).collect::<Vec<_>>(),
        "excluded": excluded.iter().map(|name| door(name)).collect::<Vec<_>>(),
    })
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

fn bounds_path() -> String {
    format!("completions/artifacts/{RUN}/bounds.md")
}

fn scope_path(round: &str) -> String {
    format!("completions/artifacts/{RUN}/r{round}/scope.md")
}

fn triage_path(round: &str) -> String {
    format!("completions/artifacts/{RUN}/r{round}/triage.md")
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
    /// Whether the call puts `text` into the file. `triage-set` takes identifiers and
    /// closed vocabularies only: it meets the arms about a write, not the arms about text.
    free_text: bool,
    /// Anything the writer needs on disk first.
    prepare: fn(&Rig),
    /// The call that writes `text`, with `extra` flags, and the file it writes.
    call: fn(&Rig, text: &str, extra: &[String]) -> (Seen, String),
}

const WRITERS: &[Writer] = &[
    Writer {
        name: "report (the body)",
        free_text: true,
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut args = report(RUN, "1", "audit-install", "1");
            args.extend_from_slice(extra);
            (
                rig.run(&args, &format!("# a report\n\n{text}\n{ENDS}\n")),
                report_path("1", "audit-install", "1"),
            )
        },
    },
    Writer {
        name: "ledger-add (a row's `source` cell)",
        free_text: true,
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
        free_text: true,
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
        free_text: true,
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
    Writer {
        name: "bound-set (the `reach` cell)",
        free_text: true,
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut args = bound_set(RUN, "non-jigc-writer", &BOUND[1..]);
            args.push(format!("--reach={text}"));
            args.extend_from_slice(extra);
            (rig.run(&args, ""), bounds_path())
        },
    },
    Writer {
        name: "scope-set (a door's derivation)",
        free_text: true,
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut given = scope(&["jigc setup"], &[]);
            given["included"][0]["derivation"] = json!(text);
            let mut args = scope_set(RUN, "1");
            args.extend_from_slice(extra);
            (rig.run(&args, &given.to_string()), scope_path("1"))
        },
    },
    Writer {
        name: "triage-set (no free text)",
        free_text: false,
        prepare: |rig| {
            rig.seed_rows(&["audit-f3"]);
            rig.run(
                &scope_set(RUN, "1"),
                &scope(&["jigc setup"], &[]).to_string(),
            )
            .must(OK, "the round's scope");
        },
        call: |rig, _, extra| {
            let entry = json!({"key": "audit-f3", "grade": "no-break"});
            let mut args = triage_set(RUN, "1");
            args.extend_from_slice(extra);
            (rig.run(&args, &entry.to_string()), triage_path("1"))
        },
    },
];

/// The three subcommands that write nothing.
const READERS: [&str; 3] = ["check-reports", "check-ledger", "state"];

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
/// [`WRITERS`] drives it — or one of the three that write nothing, or this arm is red.
#[test]
fn every_subcommand_is_a_listed_writer_or_a_check() {
    let rig = Rig::new("subcommands");
    let named: BTreeSet<String> = subcommands(&rig).into_iter().collect();
    let known: BTreeSet<String> = WRITERS
        .iter()
        .map(|writer| writer.name.split(' ').next().expect("a name").to_owned())
        .chain(READERS.map(str::to_owned))
        .collect();
    assert_eq!(
        named, known,
        "the script's subcommands (left) are the listed writers and the three readers (right)"
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
    let body = &format!("# audit of the install doors\n\nNothing found.\n{ENDS}\n");

    let seen = rig.run(&report(RUN, "1", "audit-install", "1"), body);
    seen.must(OK, "a test-stage report");
    let path = report_path("1", "audit-install", "1");
    assert_eq!(seen.stdout, format!("{path}\n"), "the path it wrote, alone");
    assert_eq!(&rig.read(&path), body, "the report, byte for byte");

    // A fix-stage report belongs to a cycle of its round.
    let mut fix = strings(&["report", "--round=2", "--stage=fix", "--cycle=3"]);
    fix.extend(strings(&["--reporter=fixer-install", "--attempt=1"]));
    fix.push(format!("--run={RUN}"));
    let seen = rig.run(&fix, body);
    seen.must(OK, "a fix-stage report");
    let path = format!("completions/artifacts/{RUN}/r2/reports/fix/c3/fixer-install.a1.md");
    assert_eq!(seen.stdout, format!("{path}\n"));
    assert_eq!(&rig.read(&path), body);

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
    let first = &format!("# the first attempt\n{ENDS}\n");
    rig.run(&report(RUN, "1", "audit-install", "1"), first)
        .must(OK, "the first attempt");

    let before = rig.snapshot();
    let again = &format!("# a re-run\n{ENDS}\n");
    rig.run(&report(RUN, "1", "audit-install", "1"), again)
        .refused(EXISTS, "the same reporter and attempt again");
    assert_eq!(rig.snapshot(), before, "the stale report is not replaced");

    rig.run(&report(RUN, "1", "audit-install", "2"), again)
        .must(OK, "the next attempt");
    assert_eq!(&rig.read(&report_path("1", "audit-install", "1")), first);
    assert_eq!(&rig.read(&report_path("1", "audit-install", "2")), again);

    // A link lying at the path is not a free path, wherever it points.
    let outside = rig.dir.path().join("elsewhere.md");
    symlink(
        &outside,
        rig.root.join(report_path("1", "audit-install", "3")),
    )
    .expect("plant a link");
    rig.run(
        &report(RUN, "1", "audit-install", "3"),
        &format!("# through a link\n{ENDS}\n"),
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

const BODY: &str = "# a report\n\nNothing found.\n<!-- end of report -->\n";

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
    IdSite {
        name: "bound-set --run",
        numeric: false,
        call: |rig, v| rig.run(&bound_set(v, "non-jigc-writer", &BOUND), ""),
    },
    IdSite {
        name: "bound-set --bound",
        numeric: false,
        call: |rig, v| rig.run(&bound_set(RUN, v, &BOUND), ""),
    },
    IdSite {
        name: "scope-set --run",
        numeric: false,
        call: |rig, v| rig.run(&scope_set(v, "1"), &scope(&["jigc setup"], &[]).to_string()),
    },
    IdSite {
        name: "scope-set --round",
        numeric: true,
        call: |rig, v| rig.run(&scope_set(RUN, v), &scope(&["jigc setup"], &[]).to_string()),
    },
    IdSite {
        name: "triage-set --run",
        numeric: false,
        call: |rig, v| {
            let entry = json!({"key": "seeded", "grade": "no-break"});
            rig.run(&triage_set(v, "1"), &entry.to_string())
        },
    },
    IdSite {
        name: "triage-set --round",
        numeric: true,
        call: |rig, v| {
            let entry = json!({"key": "seeded", "grade": "no-break"});
            rig.run(&triage_set(RUN, v), &entry.to_string())
        },
    },
    IdSite {
        name: "triage-set, an entry's key",
        numeric: false,
        call: |rig, v| {
            let entry = json!({"key": v, "grade": "no-break"});
            rig.run(&triage_set(RUN, "1"), &entry.to_string())
        },
    },
    IdSite {
        name: "triage-set, an entry's cited bound",
        numeric: false,
        call: |rig, v| {
            let entry = json!({"key": "seeded", "grade": "out-of-scope", "bound": v});
            rig.run(&triage_set(RUN, "1"), &entry.to_string())
        },
    },
    IdSite {
        name: "state --run",
        numeric: false,
        call: |rig, v| rig.run(&state(v), ""),
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
    assert!(driven > 600, "the axis collapsed to {driven} cells");
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

/// Every subcommand but `state` — for which a run that was never opened is an answer and
/// not a refusal — aimed at `run`.
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
        (
            "bound-set",
            rig.run(&bound_set(run, "non-jigc-writer", &BOUND), ""),
        ),
        (
            "scope-set",
            rig.run(
                &scope_set(run, "1"),
                &scope(&["jigc setup"], &[]).to_string(),
            ),
        ),
        (
            "triage-set",
            rig.run(
                &triage_set(run, "1"),
                &json!({"key": "seeded", "grade": "no-break"}).to_string(),
            ),
        ),
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
    // To the one subcommand that only reads, "not opened" is the answer — as data.
    let seen = rig.run(&state("rc42"), "");
    seen.must(OK, "the state of a run with no directory");
    assert_eq!(seen.json(), json!({"run": "rc42", "opened": false}));
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
        assert_eq!(
            rig.run(&state(run), "").json(),
            json!({"run": run, "opened": false}),
            "the state of `{run}`"
        );
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
        rig.run(&state(RUN), "")
            .refused(OUTSIDE, &format!("the state through a linked `{linked}`"));
        if n == 0 {
            let given = scope(&["jigc setup"], &[]).to_string();
            rig.run(&scope_set(RUN, "1"), &given)
                .refused(OUTSIDE, "a scope through a linked round directory");
        }
        assert_eq!(rig.snapshot(), before);
    }

    // The two tables, each a link to a file outside the run.
    let rig = Rig::new("outside-table");
    let outside = rig.dir.path().join("outside.md");
    fs::write(&outside, "not the run's\n").expect("a file outside the run");
    symlink(&outside, rig.run_dir().join("ledger.md")).expect("link the ledger");
    symlink(&outside, rig.run_dir().join("clauses.md")).expect("link the clause table");
    symlink(&outside, rig.run_dir().join("bounds.md")).expect("link the bounds list");
    fs::create_dir_all(rig.run_dir().join("r1")).expect("a round directory");
    symlink(&outside, rig.run_dir().join("r1/scope.md")).expect("link a round's scope");
    let before = rig.snapshot();
    rig.run(&ledger_add(RUN), &row("audit-f3").to_string())
        .refused(OUTSIDE, "a ledger that is a link");
    rig.run(&check_ledger(RUN, &["audit-f3"]), "")
        .refused(OUTSIDE, "a check of a ledger that is a link");
    rig.run(&clause_set(RUN, "no-lost-files", &rest), "")
        .refused(OUTSIDE, "a clause table that is a link");
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .refused(OUTSIDE, "a bounds list that is a link");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&["jigc setup"], &[]).to_string(),
    )
    .refused(OUTSIDE, "a scope that is a link");
    rig.run(&state(RUN), "")
        .refused(OUTSIDE, "the state of a run whose tables are links");
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
    rig.run(&state("linked"), "")
        .refused(OUTSIDE, "the state of a run directory that is a link");
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
    let body = format!("{body}{ENDS}\n");
    let want: String = lines
        .iter()
        .map(|(_, clean)| format!("{clean}\n"))
        .collect();
    let want = format!("{want}{ENDS}\n");

    let mut args = report(RUN, "1", "audit-install", "1");
    args.push(format!("--scratch={scratch}"));
    rig.run(&args, &body).must(OK, "a report naming host paths");
    assert_eq!(rig.read(&report_path("1", "audit-install", "1")), want);
}

#[test]
fn every_writer_replaces_a_host_path() {
    for_every_writer("writer-paths", |writer, rig| {
        if !writer.free_text {
            return;
        }
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
        if !writer.free_text {
            return;
        }
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
        if !writer.free_text {
            return;
        }
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
        if !writer.free_text {
            return;
        }
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
        &format!("# a report\n\nthe environment held {token}\n{ENDS}\n"),
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
            &format!("# the install proof\n\n{line}\n{ENDS}\n"),
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
    let body = format!(
        "# the install proof\n\nRan `{INSTALL_COMMAND} --locked`.\n$ {INSTALL_COMMAND}\n{ENDS}\n"
    );
    rig.run(&report(RUN, "1", "audit-install", "1"), &body)
        .must(OK, "the command inside a line");
    assert_eq!(install_line_carriers(&rig.root), BTreeSet::new());
}

/// A table's line opens with a pipe, so a cell cannot give the fence a carrier whatever it
/// holds: the writers of cells take the line, and the census stays empty.
#[test]
fn no_table_cell_can_redden_the_install_line_fence() {
    for_every_writer("fence-cells", |writer, rig| {
        if writer.name.starts_with("report") || !writer.free_text {
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
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "the bound a grade below cites");

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
            json!({"grade": "needs-bound", "graded_by": "triage"}),
            6,
            "needs-bound",
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
            graded("needs-bound", json!("verify-real")),
            "a triage grade given by the verifier",
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
    assert!(subcommands.len() >= 10, "the parser names its subcommands");
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
    assert!(
        seen.stdout.contains(ENDS),
        "--help spells the line a report ends with"
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

// ---------------------------------------------------------------------------
// 13 · A report that was cut off is not a short report
// ---------------------------------------------------------------------------

/// A report piped from a command that died arrives as a shorter text, and a shorter text
/// is still text. So a report ends with one line saying that it ended, and what lacks it
/// is refused.
#[test]
fn a_report_that_was_cut_off_is_refused_and_never_written_as_a_short_report() {
    let rig = Rig::new("cut-off");
    let whole = format!("# a long report\n\nfinding 1: lost a file\nfinding 2: none\n{ENDS}\n");
    let at = |needle: &str| whole.find(needle).expect("the needle is in the report");
    let before = rig.snapshot();

    let cut: Vec<(String, &str)> = vec![
        (
            whole[..at("finding 2")].to_owned(),
            "the pipe closed between two findings",
        ),
        (
            whole[..at("finding 2") + 4].to_owned(),
            "the pipe closed inside a line",
        ),
        (
            whole[..at(ENDS) + 9].to_owned(),
            "the pipe closed inside the last line",
        ),
        (
            format!("{whole}and a line after the end\n"),
            "text after the line that ends the report",
        ),
        (
            format!("# a report\n\nIt ends with `{ENDS}`, always.\n"),
            "the line only quoted inside another",
        ),
    ];
    for (text, why) in &cut {
        let seen = rig.run(&report(RUN, "1", "audit-install", "1"), text);
        seen.refused(TRUNCATED, why);
        assert!(
            seen.stderr.contains(ENDS),
            "{why}: the refusal spells the line: {}",
            seen.stderr
        );
        assert_eq!(rig.snapshot(), before, "{why}: nothing written");
    }

    // The whole report, however its last line is terminated.
    for (n, (text, why)) in [
        (whole.clone(), "the whole report"),
        (whole.trim_end().to_owned(), "with no final line break"),
        (
            format!("{}\r\n\n\n", whole.trim_end()),
            "with a CR and blank lines after it",
        ),
    ]
    .iter()
    .enumerate()
    {
        let attempt = (n + 1).to_string();
        rig.run(&report(RUN, "1", "audit-install", &attempt), text)
            .must(OK, why);
        assert!(
            rig.read(&report_path("1", "audit-install", &attempt))
                .starts_with(&whole[..at(ENDS)]),
            "{why}: every finding is in the file"
        );
    }
}

// ---------------------------------------------------------------------------
// 14 · The declared-bounds list
// ---------------------------------------------------------------------------

#[test]
fn a_bound_enters_the_list_only_as_the_record_of_a_ruling() {
    let rig = Rig::new("bounds");
    let seen = rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "");
    seen.must(OK, "a bound's first row");
    assert_eq!(
        seen.json(),
        json!({"bounds": bounds_path(), "bound": "non-jigc-writer"})
    );
    rig.run(
        &bound_set(
            RUN,
            "planted-state",
            &[
                "--reach=a state nobody reaches without planting it",
                "--ruling=the stop after round 1, item 3",
                "--pin=flow12::a_planted_state_is_refused",
            ],
        ),
        "",
    )
    .must(OK, "a second bound");
    // A bound is pinned later: the cell named, and nothing else of the row.
    rig.run(
        &bound_set(RUN, "non-jigc-writer", &["--pin=flow9::two_writers"]),
        "",
    )
    .must(OK, "the pin, set later");
    assert_eq!(
        table(&rig.read(&bounds_path()), &BOUND_COLUMNS),
        vec![
            vec![
                "`non-jigc-writer`",
                "races against a writer that is not jigc",
                "the opening record, declared bounds",
                "flow9::two_writers",
            ],
            vec![
                "`planted-state`",
                "a state nobody reaches without planting it",
                "the stop after round 1, item 3",
                "flow12::a_planted_state_is_refused",
            ],
        ]
    );
    assert_eq!(
        rig.state()["bounds"],
        json!([
            {
                "bound": "non-jigc-writer",
                "reach": "races against a writer that is not jigc",
                "ruling": "the opening record, declared bounds",
                "pin": "flow9::two_writers",
            },
            {
                "bound": "planted-state",
                "reach": "a state nobody reaches without planting it",
                "ruling": "the stop after round 1, item 3",
                "pin": "flow12::a_planted_state_is_refused",
            },
        ]),
        "the list, read back as data"
    );

    // A row with no ruling behind it, no reach or no word on its pin is no row.
    let before = rig.snapshot();
    let refused: &[(&[&str], &str, i32)] = &[
        (
            &BOUND[..2],
            "a new bound with no word on its pin",
            BAD_VALUE,
        ),
        (&BOUND[1..], "a new bound with no reach", BAD_VALUE),
        (
            &[BOUND[0], BOUND[2]],
            "a new bound with no ruling behind it",
            BAD_VALUE,
        ),
        (
            &[BOUND[0], "--ruling=  ", BOUND[2]],
            "a ruling of whitespace",
            BAD_VALUE,
        ),
        (&[], "an update that sets nothing", USAGE),
    ];
    for (rest, why, status) in refused {
        rig.run(&bound_set(RUN, "a-third", rest), "")
            .refused(*status, why);
    }
    rig.run(&bound_set(RUN, "planted-state", &["--reach="]), "")
        .refused(BAD_VALUE, "a reach emptied later");
    assert_eq!(rig.snapshot(), before);
}

/// Every way a grade reaches the ledger, citing `bound`.
fn grade_citers(rig: &Rig, bound: &str) -> Vec<(&'static str, Seen)> {
    let mut entry = row("audit-f9");
    entry["grade"] = json!("out-of-scope");
    entry["graded_by"] = json!("triage");
    entry["bound"] = json!(bound);
    let patch =
        json!({"key": "seeded", "grade": "out-of-scope", "graded_by": "human", "bound": bound});
    let triaged = json!({"key": "seeded", "grade": "out-of-scope", "bound": bound});
    vec![
        (
            "ledger-add",
            rig.run(
                &ledger_add(RUN),
                &json!([row("audit-f8"), entry]).to_string(),
            ),
        ),
        ("ledger-set", rig.run(&ledger_set(RUN), &patch.to_string())),
        (
            "triage-set",
            rig.run(&triage_set(RUN, "1"), &triaged.to_string()),
        ),
    ]
}

/// "Out of scope" is the one grade that takes a finding off the human's list unseen, so it
/// is sayable only of a bound the human declared.
#[test]
fn out_of_scope_is_refused_unless_it_cites_a_bound_on_the_list() {
    let rig = Rig::new("bound-cited");
    rig.seed_rows(&["seeded"]);
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&["jigc setup"], &[]).to_string(),
    )
    .must(OK, "the round's scope");

    // No list at all, and then a list that holds another bound.
    for listed in [None, Some("planted-state")] {
        if let Some(bound) = listed {
            rig.run(&bound_set(RUN, bound, &BOUND), "")
                .must(OK, "another bound");
        }
        let before = rig.snapshot();
        for (name, seen) in grade_citers(&rig, "non-jigc-writer") {
            let what = format!("{name} citing a bound the list lacks (listed: {listed:?})");
            seen.refused(NO_SUCH_BOUND, &what);
            assert!(
                seen.stderr.contains("non-jigc-writer"),
                "{what}: the refusal names the bound: {}",
                seen.stderr
            );
        }
        assert_eq!(rig.snapshot(), before, "nothing of any batch was written");
    }

    // The control: once the human's ruling is on the list, each of the three takes it.
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "the bound, declared");
    for (name, seen) in grade_citers(&rig, "non-jigc-writer") {
        seen.must(OK, &format!("{name} citing a listed bound"));
    }
    let rows = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    assert_eq!(rows[0][6], "out-of-scope: non-jigc-writer");
}

// ---------------------------------------------------------------------------
// 15 · A round's doors
// ---------------------------------------------------------------------------

#[test]
fn a_rounds_doors_are_written_once_from_structured_input_and_read_back() {
    let rig = Rig::new("scope");
    let mut given = scope(&["jigc setup", "jigc task finalize"], &["jigc doc show"]);
    given["excluded"][0]["derivation"] = json!(format!(
        "no changed symbol reaches it | see {}/dev/gate",
        rig.root.display()
    ));

    let seen = rig.run(&scope_set(RUN, "2"), &given.to_string());
    seen.must(OK, "a round's scope");
    assert_eq!(
        seen.json(),
        json!({"scope": scope_path("2"), "included": 2, "excluded": 1})
    );
    assert_eq!(
        table(&rig.read(&scope_path("2")), &SCOPE_COLUMNS),
        vec![
            vec![
                "jigc setup",
                "included",
                "the verb table",
                "a changed symbol is read by `jigc setup`",
            ],
            vec![
                "jigc task finalize",
                "included",
                "the verb table",
                "a changed symbol is read by `jigc task finalize`",
            ],
            vec![
                "jigc doc show",
                "excluded",
                "the verb table",
                "no changed symbol reaches it | see dev/gate",
            ],
        ]
    );

    // Read back as data: the doors of the highest round, as they were handed over.
    let read = rig.state();
    assert_eq!(read["round"], json!(2));
    assert_eq!(
        read["doors"],
        json!({
            "included": [
                {
                    "door": "jigc setup",
                    "registry": "the verb table",
                    "derivation": "a changed symbol is read by `jigc setup`",
                },
                {
                    "door": "jigc task finalize",
                    "registry": "the verb table",
                    "derivation": "a changed symbol is read by `jigc task finalize`",
                },
            ],
            "excluded": [
                {
                    "door": "jigc doc show",
                    "registry": "the verb table",
                    "derivation": "no changed symbol reaches it | see dev/gate",
                },
            ],
        })
    );

    // A round's test set is written once: a second scope is refused, whatever it says.
    let before = rig.snapshot();
    rig.run(&scope_set(RUN, "2"), &scope(&[], &[]).to_string())
        .refused(EXISTS, "a second scope for the round");

    // And a scope says, of every door, which side it is on, where it comes from and why.
    let with = |side: &str, field: &str, value: Value| {
        let mut given = scope(&["jigc setup"], &["jigc doc show"]);
        given[side][0][field] = value;
        given
    };
    let without = |side: &str, field: &str| {
        let mut given = scope(&["jigc setup"], &["jigc doc show"]);
        given[side][0]
            .as_object_mut()
            .expect("a door object")
            .remove(field);
        given
    };
    let malformed: Vec<(String, &str)> = vec![
        (String::new(), "no input"),
        (
            "{\"included\": [{\"door\"".to_owned(),
            "input that was cut off",
        ),
        ("[]".to_owned(), "a scope that is a list"),
        (
            json!({"included": []}).to_string(),
            "no excluded doors named",
        ),
        (
            json!({"excluded": []}).to_string(),
            "no included doors named",
        ),
        (
            json!({"included": {}, "excluded": []}).to_string(),
            "a side that is not a list",
        ),
        (
            json!({"included": ["jigc setup"], "excluded": []}).to_string(),
            "a door that is only a name",
        ),
        (
            without("included", "derivation").to_string(),
            "a door with no derivation",
        ),
        (
            without("excluded", "registry").to_string(),
            "a door with no registry",
        ),
        (
            with("excluded", "derivation", json!("  ")).to_string(),
            "a derivation of whitespace",
        ),
        (
            with("included", "door", json!("")).to_string(),
            "a door with no name",
        ),
        (
            with("included", "door", json!(7)).to_string(),
            "a door that is not text",
        ),
        (
            with("included", "inside", json!(true)).to_string(),
            "a field nobody knows",
        ),
        (
            scope(&["jigc setup", "jigc setup"], &[]).to_string(),
            "a door listed twice",
        ),
        (
            scope(&["jigc setup"], &["jigc setup"]).to_string(),
            "a door on both sides",
        ),
    ];
    for (input, why) in &malformed {
        rig.run(&scope_set(RUN, "3"), input).refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);

    // A round that tests nothing says so, with both sides empty.
    rig.run(&scope_set(RUN, "3"), &scope(&[], &[]).to_string())
        .must(OK, "a scope with no door");
    assert_eq!(
        rig.state()["doors"],
        json!({"included": [], "excluded": []})
    );
}

// ---------------------------------------------------------------------------
// 16 · A round's triage record
// ---------------------------------------------------------------------------

/// The three doors of the truth tables: one inside the round's test set, one the scope
/// step left out of it, and one it never listed.
const INSIDE: &str = "jigc setup";
const EXCLUDED: &str = "jigc doc show";
const UNLISTED: &str = "jigc rename";

/// A ledger row at `door`.
fn row_at(key: &str, door: &str) -> Value {
    let mut entry = row(key);
    entry["door"] = json!(door);
    entry
}

#[test]
fn a_rounds_triage_is_recorded_with_inside_computed_from_the_rounds_doors() {
    let rig = Rig::new("triage");
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "a declared bound");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");
    let mut fixed = row_at("f-fixed", INSIDE);
    fixed["disposition"] = json!("fixed");
    fixed["detail"] = json!("0f34d8f0");
    let rows = json!([
        row_at("f-in", INSIDE),
        row_at("f-out", EXCLUDED),
        row_at("f-unlisted", UNLISTED),
        row_at("f-bounded", INSIDE),
        row_at("f-waiting", INSIDE),
        fixed,
        row_at("f-untouched", INSIDE),
    ]);
    rig.run(&ledger_add(RUN), &rows.to_string())
        .must(OK, "the round's findings");

    let entries = json!([
        {"key": "f-in", "grade": "breaks", "verdict": "confirmed", "regression": false},
        {"key": "f-out", "grade": "unclear", "verdict": "confirmed", "regression": true},
        {"key": "f-unlisted", "grade": "breaks", "verdict": "refuted"},
        {"key": "f-bounded", "grade": "out-of-scope", "bound": "non-jigc-writer"},
        {"key": "f-waiting", "grade": "unclear"},
        {"key": "f-fixed", "grade": "needs-bound"},
    ]);
    let seen = rig.run(&triage_set(RUN, "1"), &entries.to_string());
    seen.must(OK, "the round's triage");
    assert_eq!(
        seen.json(),
        json!({
            "triage": triage_path("1"),
            "ledger": ledger_path(),
            "recorded": ["f-in", "f-out", "f-unlisted", "f-bounded", "f-waiting", "f-fixed"],
        })
    );

    // The round's record: what triage and the verifier established, where the script put
    // each finding, and the disposition each row carried into the round.
    assert_eq!(
        table(&rig.read(&triage_path("1")), &TRIAGE_COLUMNS),
        vec![
            vec!["`f-in`", "inside", "breaks", "confirmed", "no", "open"],
            vec![
                "`f-out`",
                "outside: excluded",
                "unclear",
                "confirmed",
                "yes",
                "open"
            ],
            vec![
                "`f-unlisted`",
                "outside: unlisted",
                "breaks",
                "refuted",
                "-",
                "open"
            ],
            vec![
                "`f-bounded`",
                "inside",
                "out-of-scope: non-jigc-writer",
                "-",
                "-",
                "open",
            ],
            vec!["`f-waiting`", "inside", "unclear", "-", "-", "open"],
            vec![
                "`f-fixed`",
                "inside",
                "needs-bound",
                "-",
                "-",
                "fixed: 0f34d8f0"
            ],
        ]
    );
    // And the ledger's grade follows from it — one entry, never two that could disagree.
    let ledger = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    let graded: Vec<(&str, &str, &str)> = ledger
        .iter()
        .map(|r| (r[0].as_str(), r[6].as_str(), r[7].as_str()))
        .collect();
    assert_eq!(
        graded,
        [
            ("`f-in`", "confirmed", "verify-real"),
            ("`f-out`", "regression", "verify-real"),
            ("`f-unlisted`", "refuted", "verify-real"),
            ("`f-bounded`", "out-of-scope: non-jigc-writer", "triage"),
            ("`f-waiting`", "unclear", "triage"),
            ("`f-fixed`", "needs-bound", "triage"),
            ("`f-untouched`", "ungraded", "-"),
        ]
    );
    assert_eq!(
        ledger[5][8], "fixed: 0f34d8f0",
        "a disposition is never touched"
    );

    // The verdict arrives later: the entry is recorded again and replaces its row.
    let verdict = json!({"key": "f-waiting", "grade": "unclear", "verdict": "refuted"});
    rig.run(&triage_set(RUN, "1"), &verdict.to_string())
        .must(OK, "the verdict, later");
    let rows = table(&rig.read(&triage_path("1")), &TRIAGE_COLUMNS);
    assert_eq!(rows.len(), 6, "one row per finding of the round");
    assert_eq!(
        rows[4],
        ["`f-waiting`", "inside", "unclear", "refuted", "-", "open"]
    );

    // Read back as data, beside the row it belongs to.
    let read = rig.state();
    let found = |key: &str| {
        read["ledger"]
            .as_array()
            .expect("the ledger's rows")
            .iter()
            .find(|entry| entry["key"] == key)
            .unwrap_or_else(|| panic!("no row `{key}` in {read}"))
            .clone()
    };
    assert_eq!(
        found("f-out")["triage"],
        json!({
            "round": 1,
            "inside": false,
            "door": "excluded",
            "grade": "unclear",
            "bound": null,
            "verdict": "confirmed",
            "regression": true,
            "found_with": "open",
        })
    );
    assert_eq!(found("f-untouched")["triage"], Value::Null);

    // What an entry may not say. `inside` above all: it is computed, never supplied.
    let before = rig.snapshot();
    let entry = |extra: Value| {
        let mut entry = json!({"key": "f-untouched", "grade": "breaks"});
        for (field, value) in extra.as_object().expect("an object") {
            entry[field] = value.clone();
        }
        entry
    };
    let malformed: Vec<(Value, &str)> = vec![
        (entry(json!({"inside": true})), "`inside`, supplied"),
        (entry(json!({"door": INSIDE})), "a door, supplied"),
        (entry(json!({"graded_by": "human"})), "a grader, supplied"),
        (
            entry(json!({"grade": "confirmed"})),
            "a verdict as the triage grade",
        ),
        (
            entry(json!({"grade": "severe"})),
            "a grade outside the vocabulary",
        ),
        (
            entry(json!({"verdict": "confirmed"})),
            "confirmed, with no word on regression",
        ),
        (
            entry(json!({"verdict": "confirmed", "regression": "yes"})),
            "a regression that is not a boolean",
        ),
        (
            entry(json!({"verdict": "refuted", "regression": false})),
            "a regression of a refuted finding",
        ),
        (
            entry(json!({"regression": true})),
            "a regression with no verdict",
        ),
        (
            entry(json!({"verdict": "regression"})),
            "a verdict outside the two",
        ),
        (
            entry(json!({"grade": "no-break", "verdict": "refuted"})),
            "a verdict on a finding triage did not send to the verifier",
        ),
        (
            entry(json!({"grade": "out-of-scope"})),
            "out of scope, citing no bound",
        ),
        (
            entry(json!({"bound": "non-jigc-writer"})),
            "a bound cited by another grade",
        ),
        (json!({"grade": "breaks"}), "an entry with no key"),
        (json!({"key": "f-untouched"}), "an entry with no grade"),
    ];
    for (entry, why) in &malformed {
        let batch = json!([{"key": "f-in", "grade": "no-break"}, entry]);
        rig.run(&triage_set(RUN, "1"), &batch.to_string())
            .refused(BAD_VALUE, why);
        assert_eq!(rig.snapshot(), before, "{why}: nothing of the batch");
    }
    let twice = json!([
        {"key": "f-in", "grade": "no-break"},
        {"key": "f-in", "grade": "breaks"},
    ]);
    rig.run(&triage_set(RUN, "1"), &twice.to_string())
        .refused(BAD_VALUE, "a finding triaged twice in one batch");
    for (input, why) in [
        ("", "no input"),
        ("[{\"key\": \"f-in\", \"gra", "a batch cut off"),
    ] {
        rig.run(&triage_set(RUN, "1"), input)
            .refused(BAD_VALUE, why);
    }

    // A finding with no ledger row has no door to compute from; a round with no scope has
    // no doors to compute against.
    let stranger = json!([
        {"key": "f-in", "grade": "no-break"},
        {"key": "never-filed", "grade": "no-break"},
    ]);
    rig.run(&triage_set(RUN, "1"), &stranger.to_string())
        .refused(NO_SUCH_ROW, "a finding the ledger lacks");
    rig.run(
        &triage_set(RUN, "2"),
        &json!({"key": "f-in", "grade": "no-break"}).to_string(),
    )
    .refused(NO_SCOPE, "a round whose scope was never written");
    assert_eq!(rig.snapshot(), before);
}

// ---------------------------------------------------------------------------
// 17 · The state, as one document
// ---------------------------------------------------------------------------

#[test]
fn state_is_the_runs_committed_state_as_one_json_document() {
    let rig = Rig::new("state");

    // An opened run with nothing in it — and reading it writes nothing.
    let before = rig.snapshot();
    assert_eq!(
        rig.state(),
        json!({
            "run": RUN,
            "opened": true,
            "rounds": [],
            "round": null,
            "doors": null,
            "bounds": [],
            "clauses": [],
            "ledger": [],
            "blockers": [],
            "human_list": [],
            "unsettled": [{"cell": "nothing-open-and-no-close", "keys": []}],
            "forbids_close": [{"clauses": "no row"}],
            "next": "unsettled",
        })
    );
    assert_eq!(rig.snapshot(), before, "the state is read, never written");

    // Round 1: two test-stage reports and a re-run of one, two fix cycles, a scope, a
    // triage record and a round record. Round 2: one report, and nothing else yet.
    for (round, reporter, attempt) in [
        ("1", "audit-install", "1"),
        ("1", "review-source", "1"),
        ("1", "review-source", "2"),
        ("2", "audit-install", "1"),
    ] {
        rig.run(&report(RUN, round, reporter, attempt), BODY)
            .must(OK, "a test-stage report");
    }
    for cycle in ["1", "2"] {
        let mut fix = strings(&["report", "--round=1", "--stage=fix", "--attempt=1"]);
        fix.extend([
            format!("--run={RUN}"),
            format!("--cycle={cycle}"),
            "--reporter=fixer-install".to_owned(),
        ]);
        rig.run(&fix, BODY).must(OK, "a fix-stage report");
    }
    fs::write(rig.run_dir().join("r1/round.md"), "# round 1\n").expect("a round record");
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "a declared bound");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");
    rig.run(
        &clause_set(
            RUN,
            "no-lost-files",
            &[
                "--instrument=the audit | of the fix diff",
                "--scope=the delta",
                "--commit=0f34d8f0",
                "--status=green",
            ],
        ),
        "",
    )
    .must(OK, "a clause that ran");
    rig.run(
        &clause_set(
            RUN,
            "no-regression",
            &[
                "--instrument=the scripted regression set",
                "--scope=everything",
                "--status=void",
            ],
        ),
        "",
    )
    .must(OK, "a clause that never ran");
    let mut second = row_at("audit-f4", EXCLUDED);
    second["doctype"] = json!("inconsistency");
    second["round"] = json!(0);
    second["grade"] = json!("out-of-scope");
    second["graded_by"] = json!("human");
    second["bound"] = json!("non-jigc-writer");
    second["disposition"] = json!("later");
    second["detail"] = json!("the 1.x fix pass");
    rig.run(
        &ledger_add(RUN),
        &json!([row_at("audit-f3", INSIDE), second]).to_string(),
    )
    .must(OK, "two findings");
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "audit-f3", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "round 1's triage");

    assert_eq!(
        rig.state(),
        json!({
            "run": RUN,
            "opened": true,
            "rounds": [
                {
                    "round": 1,
                    "scope": true,
                    "triage": true,
                    "record": true,
                    "test_reports": 3,
                    "fix_cycles": [
                        {"cycle": 1, "reports": 1},
                        {"cycle": 2, "reports": 1},
                    ],
                },
                {
                    "round": 2,
                    "scope": false,
                    "triage": false,
                    "record": false,
                    "test_reports": 1,
                    "fix_cycles": [],
                },
            ],
            "round": 2,
            "doors": null,
            "bounds": [{
                "bound": "non-jigc-writer",
                "reach": "races against a writer that is not jigc",
                "ruling": "the opening record, declared bounds",
                "pin": "unpinned",
            }],
            "clauses": [
                {
                    "clause": "no-lost-files",
                    "instrument": "the audit | of the fix diff",
                    "commit": "0f34d8f0",
                    "scope": "the delta",
                    "status": "green",
                },
                {
                    "clause": "no-regression",
                    "instrument": "the scripted regression set",
                    "commit": null,
                    "scope": "everything",
                    "status": "void",
                },
            ],
            "ledger": [
                {
                    "key": "audit-f3",
                    "doctype": "jigc-feedback",
                    "round": 1,
                    "source": "audit-install, finding 3",
                    "door": INSIDE,
                    "clause": "no-lost-files",
                    "grade": "confirmed",
                    "bound": null,
                    "graded_by": "verify-real",
                    "disposition": "open",
                    "detail": null,
                    "repro": "r1/reports/test/audit-install.a1.md, the third block",
                    "triage": {
                        "round": 1,
                        "inside": true,
                        "door": "included",
                        "grade": "breaks",
                        "bound": null,
                        "verdict": "confirmed",
                        "regression": false,
                        "found_with": "open",
                    },
                    "route": "fix",
                    "why": "inside",
                },
                {
                    "key": "audit-f4",
                    "doctype": "inconsistency",
                    "round": 0,
                    "source": "audit-install, finding 3",
                    "door": EXCLUDED,
                    "clause": "no-lost-files",
                    "grade": "out-of-scope",
                    "bound": "non-jigc-writer",
                    "graded_by": "human",
                    "disposition": "later",
                    "detail": "the 1.x fix pass",
                    "repro": "r1/reports/test/audit-install.a1.md, the third block",
                    "triage": null,
                    "route": "recorded",
                    "why": "ruled",
                },
            ],
            "blockers": ["audit-f3"],
            "human_list": [],
            "unsettled": [],
            "forbids_close": [
                {"clause": "no-regression", "status": "void"},
                {"finding": "audit-f3", "route": "fix"},
            ],
            "next": "fix",
        })
    );
}

/// A table writer replaces its file, and `triage-set` replaces two. The state is read
/// between two writes, never across one: it waits for the write in flight.
#[test]
fn state_waits_for_a_write_in_flight_and_reads_what_it_wrote() {
    let mut rig = Rig::new("state-lock");
    rig.env.push(("STUB_GITLEAKS", "slow".to_owned()));
    let mut writer = rig
        .command(&ledger_add(RUN))
        .spawn()
        .expect("spawn the writer");
    child_stdin::feed(&mut writer, row("audit-f3").to_string());
    // The writer is in its scan — it holds the lock, and will for two seconds more —
    // once the scan's working directory is there.
    let patience = Instant::now() + Duration::from_secs(30);
    let scanning = |tmp: &Path| {
        fs::read_dir(tmp)
            .expect("read the rig's temp directory")
            .flatten()
            .any(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("jigc-stabilize-scan-")
            })
    };
    while !scanning(&rig.tmp) {
        assert!(
            Instant::now() < patience,
            "the writer never reached its scan"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(
        !rig.root.join(ledger_path()).exists(),
        "the control: nothing is written yet"
    );

    rig.env.clear();
    let read = rig.state();
    assert_eq!(
        read["ledger"][0]["key"],
        json!("audit-f3"),
        "the state was read after the write, not during it: {read}"
    );
    let status = writer.wait().expect("the writer exits");
    assert!(status.success(), "the writer wrote: {status:?}");
}

/// The state is read from five tables, and none of them is read on a guess: a table that
/// is not the script's, a cell outside its vocabulary, and a triage record that disagrees
/// with the doors it was computed from are each refused.
#[test]
fn state_refuses_a_record_it_cannot_read_as_the_scripts_own() {
    type Damage = (&'static str, fn() -> String, fn(&str) -> String);
    let damage: &[Damage] = &[
        ("a ledger row that lost a cell", ledger_path, |text| {
            text.replacen("| `jigc-feedback` ", "", 1)
        }),
        ("a grade somebody wrote by hand", ledger_path, |text| {
            text.replacen("| confirmed |", "| severe |", 1)
        }),
        (
            "a disposition somebody wrote by hand",
            ledger_path,
            |text| text.replacen("| open |", "| wontfix |", 1),
        ),
        (
            "a clause status somebody wrote by hand",
            clauses_path,
            |text| text.replacen("| green |", "| passed |", 1),
        ),
        ("prose after the bounds list", bounds_path, |text| {
            format!("{text}\nA bound somebody thought of.\n")
        }),
        (
            "a door on a side nobody knows",
            || scope_path("1"),
            |text| text.replacen("| excluded |", "| maybe |", 1),
        ),
        (
            "a triage row moved inside by hand",
            || triage_path("1"),
            |text| text.replacen("| outside: excluded |", "| inside |", 1),
        ),
        (
            "a triage row for a finding the ledger lacks",
            || triage_path("1"),
            |text| text.replacen("`f-out`", "`f-gone`", 1),
        ),
        (
            "a verdict somebody wrote by hand",
            || triage_path("1"),
            |text| text.replacen("| confirmed | no |", "| likely | no |", 1),
        ),
    ];
    std::thread::scope(|threads| {
        for (n, (why, file, damaged)) in damage.iter().enumerate() {
            threads.spawn(move || {
                let rig = Rig::new(&format!("state-corrupt-{n}"));
                rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
                    .must(OK, "a declared bound");
                // The first excluded door is one no finding stands at.
                rig.run(
                    &scope_set(RUN, "1"),
                    &scope(&[INSIDE], &["jigc task finalize", EXCLUDED]).to_string(),
                )
                .must(OK, "round 1's scope");
                let rest = ["--instrument=the audit", "--scope=the delta"];
                let mut green = clause_set(RUN, "no-lost-files", &rest);
                green.extend(strings(&["--commit=0f34d8f0", "--status=green"]));
                rig.run(&green, "").must(OK, "a clause");
                rig.run(
                    &ledger_add(RUN),
                    &json!([row_at("f-in", INSIDE), row_at("f-out", EXCLUDED)]).to_string(),
                )
                .must(OK, "two findings");
                let entries = json!([
                    {"key": "f-in", "grade": "breaks", "verdict": "confirmed", "regression": false},
                    {"key": "f-out", "grade": "breaks", "verdict": "confirmed", "regression": false},
                ]);
                rig.run(&triage_set(RUN, "1"), &entries.to_string())
                    .must(OK, "round 1's triage");
                rig.run(&state(RUN), "")
                    .must(OK, "the control: the state as written");

                let path = rig.root.join(file());
                let text = fs::read_to_string(&path).expect("read the table");
                let hurt = damaged(&text);
                assert_ne!(hurt, text, "{why}: the damage must land");
                fs::write(&path, hurt).expect("damage the table");
                let before = rig.snapshot();
                rig.run(&state(RUN), "").refused(CORRUPT, why);
                assert_eq!(rig.snapshot(), before, "{why}: the file is left as it was");
            });
        }
    });

    // A triage record with no scope beside it has nothing it was computed from — though
    // its row says what an empty scope would have said.
    let rig = Rig::new("state-corrupt-scope");
    rig.seed_rows(&["seeded"]);
    rig.run(&scope_set(RUN, "1"), &scope(&[], &[]).to_string())
        .must(OK, "round 1's scope");
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "seeded", "grade": "no-break"}).to_string(),
    )
    .must(OK, "round 1's triage");
    fs::remove_file(rig.root.join(scope_path("1"))).expect("remove the scope");
    rig.run(&state(RUN), "")
        .refused(CORRUPT, "a triage record whose scope is gone");
}

// ---------------------------------------------------------------------------
// 18 · The routing and the next step: two truth tables
// ---------------------------------------------------------------------------

/// One finding of the truth table: how it comes to stand in the run's records, and where
/// the script must route it.
struct Finding {
    key: &'static str,
    door: &'static str,
    /// What the row says as it is added, beyond the columns every row has: a disposition
    /// it carries into the round's triage, or a grade the human gave it.
    said: Value,
    /// The round's triage entry, less its key; `None` for a finding the round never saw.
    triage: Option<Value>,
    /// A ledger patch made after the triage, less its key.
    then: Option<Value>,
    route: &'static str,
    why: &'static str,
}

impl Finding {
    fn new(key: &'static str, door: &'static str, routed: (&'static str, &'static str)) -> Self {
        Finding {
            key,
            door,
            said: json!({}),
            triage: None,
            then: None,
            route: routed.0,
            why: routed.1,
        }
    }

    fn said(mut self, said: Value) -> Self {
        self.said = said;
        self
    }

    fn triaged(mut self, entry: Value) -> Self {
        self.triage = Some(entry);
        self
    }

    fn then(mut self, patch: Value) -> Self {
        self.then = Some(patch);
        self
    }
}

fn confirmed() -> Value {
    json!({"grade": "breaks", "verdict": "confirmed", "regression": false})
}

fn regression() -> Value {
    json!({"grade": "unclear", "verdict": "confirmed", "regression": true})
}

fn refuted() -> Value {
    json!({"grade": "breaks", "verdict": "refuted"})
}

fn graded(grade: &str) -> Value {
    json!({"grade": grade})
}

fn disposed(disposition: &str) -> Value {
    match disposition {
        "fixed" => json!({"disposition": "fixed", "detail": "0f34d8f0"}),
        "bound" => json!({"disposition": "bound", "detail": "a planted state"}),
        other => json!({"disposition": other}),
    }
}

/// **The per-finding truth table.** Every cell the rulings settle, and the three they do
/// not — routed `unsettled`, to the human, and never guessed.
fn findings() -> Vec<Finding> {
    const FIX_INSIDE: (&str, &str) = ("fix", "inside");
    const FIX_REGRESSION: (&str, &str) = ("fix", "regression");
    const FIX_ADMITTED: (&str, &str) = ("fix", "admitted");
    const HUMAN_OUTSIDE: (&str, &str) = ("human", "outside");
    const NOT_A_BREAK: (&str, &str) = ("recorded", "not-a-break");
    const RULED: (&str, &str) = ("recorded", "ruled");
    const FIXED: (&str, &str) = ("recorded", "fixed");
    const UNVERIFIED: (&str, &str) = ("unsettled", "unverified");
    const FOUND_AGAIN: (&str, &str) = ("unsettled", "found-again-after-its-fix");
    let f = Finding::new;
    vec![
        // A verified break nobody has disposed of: by its door, unless it is a regression.
        f("in-confirmed", INSIDE, FIX_INSIDE).triaged(confirmed()),
        f("out-confirmed", EXCLUDED, HUMAN_OUTSIDE).triaged(confirmed()),
        f("unlisted-confirmed", UNLISTED, HUMAN_OUTSIDE).triaged(confirmed()),
        f("in-regression", INSIDE, FIX_REGRESSION).triaged(regression()),
        f("out-regression", EXCLUDED, FIX_REGRESSION).triaged(regression()),
        // Not a break, by the verifier or by triage.
        f("in-refuted", INSIDE, NOT_A_BREAK).triaged(refuted()),
        f("in-no-break", INSIDE, NOT_A_BREAK).triaged(graded("no-break")),
        // Out of scope: recorded under a listed bound, the human's without one.
        f("oos-listed", INSIDE, ("recorded", "out-of-scope"))
            .triaged(json!({"grade": "out-of-scope", "bound": "non-jigc-writer"})),
        f("needs-bound", INSIDE, ("human", "needs-bound")).triaged(graded("needs-bound")),
        // Triage that is not finished.
        f("in-unverified", INSIDE, UNVERIFIED).triaged(graded("breaks")),
        f("in-unclear", EXCLUDED, UNVERIFIED).triaged(graded("unclear")),
        f("never-triaged", INSIDE, ("unsettled", "ungraded")),
        // A break the human graded, which no round's triage placed.
        f("human-confirmed", INSIDE, ("human", "untriaged"))
            .said(json!({"grade": "confirmed", "graded_by": "human"})),
        f("human-regression", EXCLUDED, FIX_REGRESSION)
            .said(json!({"grade": "regression", "graded_by": "human"})),
        // The human's three rulings, and a fix.
        f("out-admitted", EXCLUDED, FIX_ADMITTED)
            .triaged(confirmed())
            .then(disposed("admitted")),
        f("out-bound", EXCLUDED, RULED)
            .triaged(confirmed())
            .then(disposed("bound")),
        f("out-later", EXCLUDED, RULED)
            .triaged(confirmed())
            .then(disposed("later")),
        f("in-fixed", INSIDE, FIXED)
            .triaged(confirmed())
            .then(disposed("fixed")),
        f("needs-bound-later", INSIDE, RULED)
            .triaged(graded("needs-bound"))
            .then(disposed("later")),
        // `no-action` is neither a fix nor a ruling: a verified break under it is open.
        f("in-no-action", INSIDE, FIX_INSIDE)
            .triaged(confirmed())
            .then(disposed("no-action")),
        f("out-no-action", EXCLUDED, HUMAN_OUTSIDE)
            .triaged(confirmed())
            .then(disposed("no-action")),
        // A finding found again inherits its row's disposition.
        f("again-later", EXCLUDED, RULED)
            .said(disposed("later"))
            .triaged(confirmed()),
        f("again-bound", EXCLUDED, RULED)
            .said(disposed("bound"))
            .triaged(confirmed()),
        f("again-admitted", EXCLUDED, FIX_ADMITTED)
            .said(disposed("admitted"))
            .triaged(confirmed()),
        f("again-no-action", EXCLUDED, HUMAN_OUTSIDE)
            .said(disposed("no-action"))
            .triaged(confirmed()),
        // …and of `fixed` the rulings do not say whether it is inherited.
        f("again-fixed", INSIDE, FOUND_AGAIN)
            .said(disposed("fixed"))
            .triaged(confirmed()),
        f("again-fixed-unverified", INSIDE, FOUND_AGAIN)
            .said(disposed("fixed"))
            .triaged(graded("breaks")),
        f("again-fixed-needs-bound", INSIDE, FOUND_AGAIN)
            .said(disposed("fixed"))
            .triaged(graded("needs-bound")),
        f("again-fixed-refuted", INSIDE, FIXED)
            .said(disposed("fixed"))
            .triaged(refuted()),
        f("again-refixed", INSIDE, FIXED)
            .said(disposed("fixed"))
            .triaged(confirmed())
            .then(json!({"disposition": "fixed", "detail": "967ca491"})),
    ]
}

/// The ground every cell stands on: one declared bound, and round 1's doors.
fn ground(rig: &Rig) {
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "the declared bound");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");
}

/// Bring `findings` into the run through the script's own writers: the rows, then the
/// round's triage, then what became of a row afterwards.
fn plant(rig: &Rig, findings: &[&Finding]) {
    let keyed = |fields: &Value, key: &str| {
        let mut entry = fields.clone();
        entry["key"] = json!(key);
        entry
    };
    let rows: Vec<Value> = findings
        .iter()
        .map(|finding| {
            let mut entry = row_at(finding.key, finding.door);
            for (field, value) in finding.said.as_object().expect("an object") {
                entry[field] = value.clone();
            }
            entry
        })
        .collect();
    let entries: Vec<Value> = findings
        .iter()
        .filter_map(|finding| Some(keyed(finding.triage.as_ref()?, finding.key)))
        .collect();
    let patches: Vec<Value> = findings
        .iter()
        .filter_map(|finding| Some(keyed(finding.then.as_ref()?, finding.key)))
        .collect();
    for (args, batch, what) in [
        (ledger_add(RUN), rows, "the rows"),
        (triage_set(RUN, "1"), entries, "the round's triage"),
        (ledger_set(RUN), patches, "what became of a row afterwards"),
    ] {
        if !batch.is_empty() {
            rig.run(&args, &json!(batch).to_string()).must(OK, what);
        }
    }
}

/// `(route, why)` of the row `key` in a state document.
fn routed(read: &Value, key: &str) -> (String, String) {
    let entry = read["ledger"]
        .as_array()
        .expect("the ledger's rows")
        .iter()
        .find(|entry| entry["key"] == key)
        .unwrap_or_else(|| panic!("no row `{key}` in {read}"));
    let text = |field: &str| entry[field].as_str().expect("text").to_owned();
    (text("route"), text("why"))
}

#[test]
fn every_finding_is_routed_by_its_grade_its_disposition_and_the_rounds_doors() {
    let rig = Rig::new("routes");
    ground(&rig);
    let truth = findings();
    plant(&rig, &truth.iter().collect::<Vec<_>>());

    let read = rig.state();
    for finding in &truth {
        assert_eq!(
            routed(&read, finding.key),
            (finding.route.to_owned(), finding.why.to_owned()),
            "the route of `{}`",
            finding.key
        );
    }
    assert!(
        truth.len() >= 30,
        "the table collapsed to {} cells",
        truth.len()
    );

    // The three lists a stage acts on are the routes, and nothing an agent compiled.
    let routed_as = |route: &str| -> Vec<&Finding> {
        truth
            .iter()
            .filter(|finding| finding.route == route)
            .collect()
    };
    assert_eq!(
        read["blockers"],
        json!(routed_as("fix").iter().map(|f| f.key).collect::<Vec<_>>())
    );
    assert_eq!(
        read["human_list"],
        json!(
            routed_as("human")
                .iter()
                .map(|f| json!({"key": f.key, "why": f.why}))
                .collect::<Vec<_>>()
        )
    );
    let cell = |why: &str| -> Value {
        let keys: Vec<&str> = routed_as("unsettled")
            .iter()
            .filter(|f| f.why == why)
            .map(|f| f.key)
            .collect();
        json!({"cell": why, "keys": keys})
    };
    assert_eq!(
        read["unsettled"],
        json!([
            cell("ungraded"),
            cell("unverified"),
            cell("found-again-after-its-fix"),
        ])
    );

    // A later round has doors of its own, and the latest round that triaged a finding is
    // the one that places it; a finding it did not triage stays where its round put it.
    rig.run(
        &scope_set(RUN, "2"),
        &scope(&[EXCLUDED], &[INSIDE]).to_string(),
    )
    .must(OK, "round 2's scope, the other way round");
    let again = json!([
        {"key": "in-confirmed", "grade": "breaks", "verdict": "confirmed", "regression": false},
        {"key": "out-confirmed", "grade": "breaks", "verdict": "confirmed", "regression": false},
    ]);
    rig.run(&triage_set(RUN, "2"), &again.to_string())
        .must(OK, "round 2's triage of two of them");
    let read = rig.state();
    let placed = |key: &str, route: &str, why: &str| {
        assert_eq!(
            routed(&read, key),
            (route.to_owned(), why.to_owned()),
            "the route of `{key}` after round 2"
        );
    };
    placed("in-confirmed", "human", "outside");
    placed("out-confirmed", "fix", "inside");
    placed("in-no-action", "fix", "inside");
    placed("unlisted-confirmed", "human", "outside");

    // A bound that left the list takes its findings back to the human: an out-of-scope
    // grade without a listed bound never reaches `recorded` — unless the human has ruled.
    let out_of_scope = |key: &str, disposition: &str| {
        let mut entry = row_at(key, INSIDE);
        for (field, value) in [
            ("grade", "out-of-scope"),
            ("graded_by", "triage"),
            ("bound", "non-jigc-writer"),
            ("disposition", disposition),
        ] {
            entry[field] = json!(value);
        }
        entry
    };
    let more = json!([
        out_of_scope("oos-ruled", "later"),
        out_of_scope("oos-no-action", "no-action"),
    ]);
    rig.run(&ledger_add(RUN), &more.to_string())
        .must(OK, "two more out-of-scope rows");
    let recorded = ("recorded".to_owned(), "out-of-scope".to_owned());
    let read = rig.state();
    assert_eq!(routed(&read, "oos-listed"), recorded);
    assert_eq!(routed(&read, "oos-no-action"), recorded);

    let bounds = rig.root.join(bounds_path());
    let text = fs::read_to_string(&bounds).expect("read the bounds list");
    let without: String = text
        .lines()
        .filter(|line| !line.starts_with("| `non-jigc-writer`"))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(without, text, "the bound's row is gone");
    fs::write(&bounds, without).expect("take the bound off the list");
    let read = rig.state();
    let human = ("human".to_owned(), "bound-not-listed".to_owned());
    assert_eq!(routed(&read, "oos-listed"), human);
    assert_eq!(routed(&read, "oos-no-action"), human);
    assert_eq!(
        routed(&read, "oos-ruled"),
        ("recorded".to_owned(), "ruled".to_owned()),
        "the human's ruling records it, never the grade"
    );
}

/// One state of a round: its clause table and the findings standing in its ledger, by
/// their keys in [`findings`] — and the step that must follow.
struct Round {
    name: &'static str,
    clauses: &'static [(&'static str, &'static str)],
    findings: &'static [&'static str],
    next: &'static str,
}

/// The run's two clauses, both green, and one of them not.
const GREEN: &[(&str, &str)] = &[("clause-a", "green"), ("clause-b", "green")];
const ONE_VOID: &[(&str, &str)] = &[("clause-a", "green"), ("clause-b", "void")];

/// **The round's truth table.**
const ROUNDS: &[Round] = &[
    Round {
        name: "every clause green, and no finding",
        clauses: GREEN,
        findings: &[],
        next: "close",
    },
    Round {
        name: "every clause green, and nothing that breaks one",
        clauses: GREEN,
        findings: &["in-refuted", "in-no-break", "oos-listed"],
        next: "close",
    },
    Round {
        name: "every blocker fixed, every item ruled, one found again",
        clauses: GREEN,
        findings: &["in-fixed", "out-later", "out-bound", "again-later"],
        next: "close",
    },
    // A clause row that is void, red or missing forbids close — and with nothing open the
    // rulings name no step.
    Round {
        name: "a void clause row",
        clauses: ONE_VOID,
        findings: &[],
        next: "unsettled",
    },
    Round {
        name: "a void clause row, every finding fixed",
        clauses: &[("clause-a", "void"), ("clause-b", "green")],
        findings: &["in-fixed"],
        next: "unsettled",
    },
    Round {
        name: "a red clause row",
        clauses: &[("clause-a", "green"), ("clause-b", "red")],
        findings: &[],
        next: "unsettled",
    },
    Round {
        name: "no clause row at all",
        clauses: &[],
        findings: &[],
        next: "unsettled",
    },
    // The human's list.
    Round {
        name: "an unruled item on the human's list",
        clauses: GREEN,
        findings: &["out-confirmed"],
        next: "rule",
    },
    Round {
        name: "a finding that needs a bound nobody declared",
        clauses: GREEN,
        findings: &["needs-bound"],
        next: "rule",
    },
    Round {
        name: "an unruled item beside an open blocker: the ruling comes first",
        clauses: GREEN,
        findings: &["in-confirmed", "out-confirmed"],
        next: "rule",
    },
    Round {
        name: "an unruled item under a void clause",
        clauses: ONE_VOID,
        findings: &["out-confirmed"],
        next: "rule",
    },
    Round {
        name: "an unruled item beside a finding nobody verified",
        clauses: GREEN,
        findings: &["in-unverified", "out-confirmed"],
        next: "rule",
    },
    // Fixed without asking.
    Round {
        name: "an open blocker inside the test set",
        clauses: GREEN,
        findings: &["in-confirmed"],
        next: "fix",
    },
    Round {
        name: "a regression found outside the test set",
        clauses: GREEN,
        findings: &["out-regression"],
        next: "fix",
    },
    Round {
        name: "an item the human admitted to the run",
        clauses: GREEN,
        findings: &["out-admitted"],
        next: "fix",
    },
    Round {
        name: "an open blocker under a void clause",
        clauses: ONE_VOID,
        findings: &["in-confirmed", "in-fixed"],
        next: "fix",
    },
    // What the rulings do not settle is never fixed past, and never closed over.
    Round {
        name: "a finding nobody verified",
        clauses: GREEN,
        findings: &["in-unverified"],
        next: "unsettled",
    },
    Round {
        name: "a finding nobody graded",
        clauses: GREEN,
        findings: &["never-triaged"],
        next: "unsettled",
    },
    Round {
        name: "a finding nobody verified beside an open blocker",
        clauses: GREEN,
        findings: &["in-unverified", "in-confirmed"],
        next: "unsettled",
    },
    Round {
        name: "a finding found again after its fix",
        clauses: GREEN,
        findings: &["again-fixed", "in-fixed"],
        next: "unsettled",
    },
];

/// Stand `round` up in a rig of its own and hold the state document to it.
fn drive_round(label: &str, round: &Round, truth: &[Finding]) {
    let rig = Rig::new(label);
    ground(&rig);
    for (clause, status) in round.clauses {
        let rest = ["--instrument=its instrument", "--scope=the delta"];
        let mut args = clause_set(RUN, clause, &rest);
        args.push(format!("--status={status}"));
        if *status != "void" {
            args.push("--commit=0f34d8f0".to_owned());
        }
        rig.run(&args, "").must(OK, "a clause row");
    }
    let standing: Vec<&Finding> = round
        .findings
        .iter()
        .map(|key| {
            truth
                .iter()
                .find(|finding| finding.key == *key)
                .unwrap_or_else(|| panic!("no finding `{key}` in the table"))
        })
        .collect();
    plant(&rig, &standing);

    let read = rig.state();
    let what = round.name;
    assert_eq!(read["next"], json!(round.next), "{what}: {read}");

    // What forbids closing, by the clause table and by the ledger — and `close` exactly
    // when nothing does.
    let mut forbids: Vec<Value> = round
        .clauses
        .iter()
        .filter(|(_, status)| *status != "green")
        .map(|(clause, status)| json!({"clause": clause, "status": status}))
        .collect();
    if round.clauses.is_empty() {
        forbids.push(json!({"clauses": "no row"}));
    }
    let open: Vec<&&Finding> = standing
        .iter()
        .filter(|finding| finding.route != "recorded")
        .collect();
    forbids.extend(
        open.iter()
            .map(|finding| json!({"finding": finding.key, "route": finding.route})),
    );
    assert_eq!(read["forbids_close"], json!(forbids), "{what}: {read}");
    assert_eq!(
        round.next == "close",
        forbids.is_empty(),
        "{what}: close, exactly when nothing forbids it"
    );
    let no_step = json!({"cell": "nothing-open-and-no-close", "keys": []});
    assert_eq!(
        read["unsettled"]
            .as_array()
            .expect("the unsettled cells")
            .contains(&no_step),
        open.is_empty() && round.next != "close",
        "{what}: the state the rulings name no step for is named: {read}"
    );
}

#[test]
fn the_next_step_is_computed_from_the_clause_table_and_the_ledger() {
    let truth = findings();
    let truth = &truth;
    let driven: usize = std::thread::scope(|threads| {
        let workers: Vec<_> = ROUNDS
            .chunks(4)
            .enumerate()
            .map(|(n, rounds)| {
                threads.spawn(move || {
                    for (m, round) in rounds.iter().enumerate() {
                        drive_round(&format!("next-{n}-{m}"), round, truth);
                    }
                    rounds.len()
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().expect("a worker thread"))
            .sum()
    });
    assert!(driven >= 20, "the table collapsed to {driven} cells");
}
