//! **`dev/stabilize-record` — the one script through which a stabilization run's records
//! reach the repository, and the one place its state is read back and decided**
//! ([DECISIONS.md](../DECISIONS.md) → *2026-10-05 — The stabilization workflow, as
//! ruled*, rulings 4, 5, 6, 9, 10, 12, 13 and 16; the builders' choices are the two entries
//! of the same date, *The record script, as built* and *The record script reads the run
//! back*, and the three entries of 2026-10-06, *The record script holds what the harness
//! reads its work from*, *The run's decision table, settled* and *The decision table's
//! holes, closed* — and, for what a clause's status is, *Evidence belongs to a test-set
//! item*).
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
//! wrote by hand, and read off the state document. The six cells the rulings left open were
//! decided on 2026-10-06, and each is a row under its number; so are the seven decisions
//! that closed that table's holes — close only from a tested candidate, a bound that counts
//! fix rounds, the human's go and one more re-run as recorded facts, the stage that
//! finishes a triage, the opening's last facts; and the two states the decisions still do
//! not cover are rows too, as `unsettled` — the script's word for *not decided here*.
//! **Since the repair's task 4 a clause's status is DERIVED, and the table says what was
//! done, never what was concluded**: a row of [`ROUNDS`] names what each item of the test
//! set did in which round — the script's `result-set` — and, by hand, the status each clause
//! must then have; the suite holds the state document AND the rendered clause table to it.
//! No expectation of a clause's status is computed from the fixture by a rule of this
//! suite's own.
//! **Two more hold what the harness reads its work from** ([`SELECTIONS`], and
//! [`POSITIONS`] with [`BOUND_POSITIONS`]): which items of the test set the latest round
//! runs, and the round, the cycle and the attempt an invocation of each stage works on — or
//! the word it is refused with, which for a run whose opening is not done, for a round
//! after a stop, and for one more fix round past the run's bound, is the human's to lift.
//!
//! **Every arm runs under a shell-hostile root** (a space, a `'`, a `"` and a `#` in the
//! repository's path — [dev-workflow.md](../implementation/dev-workflow.md), the rule
//! on a path that reaches a shell): the rig has no other kind of root, so no arm can pass
//! only because its path was tame.
//!
//! **The axes, iterated rather than sampled.**
//!
//! - *The identifier sites.* Every identifier the script takes — a run, a round, a stage,
//!   a cycle, a reporter, an attempt, a ledger key, a cited bound, a clause, each name a
//!   check is handed — is driven with every hostile value ([`HOSTILE_IDS`]): each is
//!   refused as `bad-id` and leaves the tree as it was.
//! - *The writers.* Ten subcommands write a file ([`WRITERS`]), and seven of them put a
//!   caller's free text into it: a report's body, a ledger row's cells as it is added and
//!   as it is set, a void run's reason, a bound's, a door's derivation, a test-set item's
//!   brief. The scanner that could not run and the killed write are driven through all
//!   ten; the host-path replacement and the hygiene stop through the seven. The list is
//!   held to the script's
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
//!
//! And on sixty-three more, for the cells decided on 2026-10-06 and the two bounds. *Cell 1:* an
//! ungraded finding recorded; an unverified one sent to a fixer, or put on the human's
//! list; the step called `unsettled` again; the human's list, or a fixer, taken before the
//! triage; the findings not named. *Cell 2:* `fixed` inherited; a finding re-opened by any
//! fix, or by a round that refuted it; a finding that needs a bound, or one sent on, not
//! standing; the round counting a refuted one, or nothing; the finding not saying so.
//! *Cells 5 and 6:* the first round that triaged a finding placing it; `no-action` a ruling.
//! *Cell 4:* an empty table, a missing stop mode and a missing bound each ready; the step,
//! the position, and the `fix` stage alone each ignoring it. *Cell 3:* the step called
//! `unsettled` again; a retry never spent, always spent, spent by any clause's re-run, by
//! the latest round's; the first round a clause ran in taken for its last, and its commit
//! matched whole; a due retry, or a clause no item judges, taken before a spent one; a
//! clause no item judges run again; a re-run while something is open, and before round 1;
//! a red row no re-run; the clauses, the human's clauses and the unsettled one each left
//! unnamed. *The first stage:* an untested round skipped; a round that is over handed to a
//! fixer. *The stops:* no stop after a round; a stop inside one; the bound stopping a
//! close; the bound one round late and one early, in the step and in the position; no
//! stop at the bound, and no refusal; what follows a stop, its round and the word `stop`
//! each wrong; the bound applied in the mode that stops after every round; the bound's
//! word said before the round's own. *The writers:* a bound that goes down; a mode, a fact
//! and a bound of any kind; the run's facts read though written by hand; the clause a
//! round ran alone written twice, of any shape, and read though written by hand.
//!
//! And on sixty-eight more, for the seven decisions that closed the table's holes.
//! *Decision 1, close only from a tested candidate:* a landed round closing; the candidate
//! current whatever was fixed, and the clause table asked on one nobody tested; a check's
//! green counting on any candidate; a hunt's green counting though the fixes' doors reach
//! it, though nobody resolved them, though only one of two landings was resolved, and
//! never counting; a round that ran one clause alone resolving the doors; a green row no
//! round tested, and one nobody judges, counting; no row ever stale; a stale row, and an
//! untested candidate, no bar to closing; `behind` counted over every round, from the
//! round after the row's, and in rounds; a stale green row the human's; a fix stage on
//! record by its cycles alone, and by its outcome alone; the candidate taken for the latest
//! round's, tested or not; the stale cell called by the other's name. *Decision 2, the
//! bound counts fix rounds:* rounds counted; the round that spent the bound stopped;
//! every step but `close` stopped; the bound a fix round late and one early; applied in
//! the other mode; no stop at a spent bound, and no refusal of the fix stage; a `test`
//! stage refused past the bound; every round counted. *Decision 3, the go:* a go that
//! lifts nothing, and one that lifts every later stop; a go taken whatever the state, for
//! any round of a stopped run, and at a stop at the bound; a stage started past a stop
//! nobody lifted, and a stop keeping a round from landing; a go taken back, of any value
//! that reads as true, not read back, and read though written by hand. *Decision 4, one
//! more re-run:* a grant that earns nothing, and one never spent; a grant for any clause,
//! in any round, written twice, and read though written by hand. *Decision 5, the stage
//! that finishes a triage:* no stage; always `test`; always `fix`; a fix cycle nobody
//! counted, and a counted one with no report on disk, each taken for the test stage's; the
//! attempt on disk taken again; any open finding read as an unfinished triage; the stage
//! not saying so. *Decision 6, the opening's facts:* a run ready without its previous
//! release, with a version and no commit, and without its default scope; the release
//! rewritten, and only its version written once; a release and a scope of any shape; each
//! of the three read though written by hand. Two survived at first — a go of any truthy
//! value, where the one cell that tried it stood in a state that refuses every go, and a
//! counted cycle with no report on disk — and each got the cell it lacked.
//!
//! **And on thirty-six more, for the derived clause** (the repair's task 4; the arms the
//! lists above name for a clause row's `stale`, for a round run `alone` and for a row set
//! by hand went with those mechanisms). *The derivation:* an item a round selected and never
//! ran owing nothing; the round that owes an item taken for the FIRST that selected it; a
//! void run standing as green; a run current whatever was fixed since; an item's run taken
//! for its first attempt; a clause that is owed nothing green; a red item not reddening its
//! clause; a clause never selected no state for the human; a census with no clause ready.
//! *The re-run:* two automatic; never spent; a grant that allows nothing, in any round, for
//! a clause that is not the human's; a re-run taken whatever the state asks, and whatever
//! the step is; the position naming none, and naming the latest round. *The results:* an
//! item the round's scope does not select; a second result in a round that is not tested; a
//! candidate with no scope, and one that is not the commit the results are of. *The view:*
//! read whatever it says; rendered over a hand's change; not rendered by each of its four
//! writers; the census written again. *The red finding:* none filed; `fixed` inherited by a
//! check red after its fix; a red result read though its finding is gone. *The state:* a
//! tested round with no scope, a result whose earlier attempt is gone, an outcome and a
//! granted run of any shape, each read. Every one turned an arm that names it red.
//!
//! **And on thirty-nine more, for the counters, the exits and the writers' table** (the
//! repair's task 6: every loop has a counter on record and an exit to the human, and a
//! write is taken only where the state asks for it — [`PLACES`], [`HERE`], [`NEXT`]: every
//! writer and every fact of the script's own lists at every position, each cell a letter
//! written by hand). *The triage passes:* a second pass without a verdict not the human's;
//! two automatic; a grant never spent; a record that does not name a finding counting no
//! pass against it; ungraded-then-unverified going on counting; the passes taken from a
//! caller; one more triage granted to a finding that is not the human's. *The attempts of
//! a stage:* never spent; an attempt that reached its record counted; a batch that names
//! none; one more granted to any stage, and never spent; the step still naming the spent
//! stage; the fix stage refused for spent attempts where a bound is what stops it, and
//! where the round still has to land; the reports of a recorded attempt not missed. *The bounds and the go:* no stop at
//! the cycle bound, and no refusal of the fix stage there; a bound raised at any state, and
//! at the other bound's stop; a go lifting every later stop of its round. *The writers'
//! table:* a round begun by any write, past a stop, and two rounds ahead; the stop mode and
//! the default scope rewritten; what a test stage records taken in a tested round; a fix
//! cycle counted in a round that is not tested, and where the position refuses a fix
//! stage; the round's outcome taken in any round; the table not held to the script's own
//! lists. *What is deleted:* a ledger row the census names, and one added by hand; the
//! ledger's own writers writing past a row that is gone; an item's last result, and a
//! result added by hand; a finding the round's triage left, not held to the ledger by that
//! record. *The fix:* a fix that is its sha's text. *The scan:* a batch scanned file by file,
//! where it is one scan. Every one turned an arm that names it
//! red, and none survived — one, as first written, changed nothing (a condition read before
//! the call it was meant to follow), and was written again.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
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
const INTERRUPTED: i32 = 17;
const PENDING: i32 = 18;
const CHECK: i32 = 19;
const REPORTS_MISMATCH: i32 = 20;
const LEDGER_MISMATCH: i32 = 21;
const GATE_MISMATCH: i32 = 22;
const FAULT: i32 = 23;

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
    (INTERRUPTED, "interrupted"),
    (PENDING, "pending"),
    (CHECK, "check"),
    (FAULT, "fault"),
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
/// The table of a round's results, as `result-set` writes it.
const RESULT_COLUMNS: [&str; 5] = ["item", "attempt", "commit", "outcome", "detail"];

const BOUND_COLUMNS: [&str; 4] = ["bound", "reach", "ruling", "pin"];

const SCOPE_COLUMNS: [&str; 4] = ["door", "test set", "registry", "derivation"];
/// The test set's seven columns, and the round record's two.
const ITEM_COLUMNS: [&str; 7] = [
    "item",
    "kind",
    "clause",
    "runs",
    "doors",
    "registries",
    "brief",
];
const ROUND_COLUMNS: [&str; 2] = ["fact", "value"];

const TRIAGE_COLUMNS: [&str; 7] = [
    "key",
    "inside",
    "triage",
    "verdict",
    "regression",
    "found with",
    "fork",
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
/// before the file exists — `slow` scans as ever, two seconds late, which holds a
/// write in flight for as long — and `once` scans once and cannot run a second time: a
/// call's files are one scan, so it is the NEXT call that meets a scanner that cannot run.
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
    once)
        if [ -e "${TMPDIR}stub-gitleaks-ran" ]; then
            echo "stub gitleaks: could not run a second time" >&2; exit 1
        fi
        : >"${TMPDIR}stub-gitleaks-ran" ;;
esac
if grep -rq --exclude-dir=.git @SECRET@ "$repo"; then
    if [ -n "$report" ]; then printf '[{"RuleID":"stub-rule","StartLine":3}]' >"$report"; fi
    exit "$code"
fi
if [ -n "$report" ]; then printf '[]' >"$report"; fi
exit 0
"#;

/// The stand-in for gitleaks, as an executable's text — this suite's, and the simulation's
/// of a stage ([`stabilize_simulation`](super::stabilize_simulation)), whose every record is
/// written through the script.
pub(crate) fn gitleaks_stub() -> String {
    GITLEAKS_STUB.replace("@SECRET@", STUB_SECRET)
}

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
        fs::write(&stub, gitleaks_stub()).expect("write the stub gitleaks");
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

    /// A file holding what a full gate printed, in the rig's temp directory — which no
    /// snapshot walks: the gate's output is the step's, never the run's.
    fn summary(&self, name: &str, steps: &[&str], tests: &[&str]) -> PathBuf {
        self.summary_of(name, &gate_output(steps, tests))
    }

    fn summary_of(&self, name: &str, text: &str) -> PathBuf {
        let path = self.tmp.join(format!("gate-{name}.txt"));
        fs::write(&path, text).expect("write a gate's output");
        path
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

fn result_set(run: &str, round: &str, commit: &str) -> Vec<String> {
    vec![
        "result-set".to_owned(),
        format!("--run={run}"),
        format!("--round={round}"),
        format!("--commit={commit}"),
    ]
}

/// One item's run, as `result-set` takes it: green; void, with a reason; or red, with what
/// the finding it is filed as needs.
fn ran(item: &str, outcome: &str) -> Value {
    match outcome {
        "void" => json!({"item": item, "outcome": outcome, "reason": "its driver died"}),
        "red" => json!({"item": item, "outcome": outcome, "doctype": "jigc-feedback",
                        "door": "jigc setup", "repro": "what the check printed"}),
        _ => json!({"item": item, "outcome": outcome}),
    }
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

fn gate_set(run: &str, round: &str, commit: &str, summary: &Path) -> Vec<String> {
    strings(&[
        "gate-set",
        &format!("--run={run}"),
        &format!("--round={round}"),
        &format!("--commit={commit}"),
        &format!("--summary={}", summary.display()),
    ])
}

fn gate_check(run: &str, round: Option<&str>, summary: &Path) -> Vec<String> {
    let mut args = strings(&[
        "gate-check",
        &format!("--run={run}"),
        &format!("--summary={}", summary.display()),
    ]);
    if let Some(round) = round {
        args.push(format!("--round={round}"));
    }
    args
}

fn apply(run: &str, round: Option<&str>) -> Vec<String> {
    let mut args = strings(&[
        "apply",
        &format!("--run={run}"),
        "--subject=docs(record): a record step",
    ]);
    if let Some(round) = round {
        args.push(format!("--round={round}"));
    }
    args
}

fn keeper(name: &str, run: &str) -> Vec<String> {
    strings(&[name, &format!("--run={run}")])
}

fn gate_path(round: &str) -> String {
    format!("completions/artifacts/{RUN}/r{round}/gate.md")
}

/// What a full `dev/gate` prints, as much of it as the script reads: the totals line, the
/// verdict line, and the failing tests by name. No step failed: a green gate.
fn gate_output(steps: &[&str], tests: &[&str]) -> String {
    let mut text = format!(
        "gate: log    <tmp>/jigc-gate-x\n==> fmt     ok        (1s)\n\n--- summary ---\ntests   passed=4700 failed={}  (over 15 test binaries)\n",
        tests.len()
    );
    if steps.is_empty() {
        text.push_str("\nGATE: PASS\nfull output: <tmp>/jigc-gate-x\n");
        return text;
    }
    text.push_str(&format!("\nGATE: FAIL (step: {})\n", steps.join(" ")));
    if !tests.is_empty() {
        text.push_str("\nfailing tests:\n");
        for test in tests {
            text.push_str(&format!("  {test}\n"));
        }
    }
    text.push_str("\nfull output: <tmp>/jigc-gate-x\nthe failing step is x\n");
    text
}

fn state(run: &str) -> Vec<String> {
    vec!["state".to_owned(), format!("--run={run}")]
}

fn item_set(run: &str) -> Vec<String> {
    vec!["item-set".to_owned(), format!("--run={run}")]
}

fn round_set(run: &str, round: &str) -> Vec<String> {
    vec![
        "round-set".to_owned(),
        format!("--run={run}"),
        format!("--round={round}"),
    ]
}

fn run_set(run: &str) -> Vec<String> {
    vec!["run-set".to_owned(), format!("--run={run}")]
}

/// A well-formed item of the test set: one review row, run where the round reaches it.
fn item(id: &str) -> Value {
    json!({
        "item": id,
        "kind": "review-row",
        "clause": "no-lost-files",
        "runs": "in-scope",
        "brief": "the row's brief, as the opening record names it",
        "doors": ["jigc setup"],
    })
}

/// A full sha and a sha256 nobody computed: `digit` repeated.
fn sha(digit: char) -> String {
    digit.to_string().repeat(40)
}

fn sha256(digit: char) -> String {
    digit.to_string().repeat(64)
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

fn test_set_path() -> String {
    format!("completions/artifacts/{RUN}/test-set.md")
}

fn round_path(round: &str) -> String {
    format!("completions/artifacts/{RUN}/r{round}/round.md")
}

fn results_path(round: &str) -> String {
    format!("completions/artifacts/{RUN}/r{round}/results.md")
}

fn run_path() -> String {
    format!("completions/artifacts/{RUN}/run.md")
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
        name: "result-set (a void run's `reason`)",
        free_text: true,
        prepare: |rig| {
            rig.run(&item_set(RUN), &item("row-setup").to_string())
                .must(OK, "the item");
            rig.run(
                &scope_set(RUN, "1"),
                &scope(&["jigc setup"], &[]).to_string(),
            )
            .must(OK, "the round's scope");
        },
        call: |rig, text, extra| {
            let mut given = ran("row-setup", "void");
            given["reason"] = json!(text);
            let mut args = result_set(RUN, "1", &sha('a'));
            args.extend_from_slice(extra);
            (rig.run(&args, &given.to_string()), results_path("1"))
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
    Writer {
        name: "item-set (an item's `brief`)",
        free_text: true,
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut given = item("row-setup");
            given["brief"] = json!(text);
            let mut args = item_set(RUN);
            args.extend_from_slice(extra);
            (rig.run(&args, &given.to_string()), test_set_path())
        },
    },
    Writer {
        name: "round-set (no free text)",
        free_text: false,
        prepare: |rig| {
            rig.run(
                &scope_set(RUN, "1"),
                &scope(&["jigc setup"], &[]).to_string(),
            )
            .must(OK, "the round's scope");
        },
        call: |rig, _, extra| {
            let mut args = round_set(RUN, "1");
            args.extend_from_slice(extra);
            (
                rig.run(&args, &json!({"candidate": sha('a')}).to_string()),
                round_path("1"),
            )
        },
    },
    Writer {
        name: "run-set (no free text)",
        free_text: false,
        prepare: |_| {},
        call: |rig, _, extra| {
            let mut args = run_set(RUN);
            args.extend_from_slice(extra);
            (
                rig.run(&args, &json!({"stop": "every-round"}).to_string()),
                run_path(),
            )
        },
    },
    Writer {
        name: "gate-set (a failing test's name)",
        free_text: true,
        prepare: |_| {},
        call: |rig, text, extra| {
            let summary = rig.summary("writer", &["tier1"], &[text]);
            let mut args = gate_set(RUN, "1", &sha('a'), &summary);
            args.extend_from_slice(extra);
            (rig.run(&args, ""), gate_path("1"))
        },
    },
    Writer {
        name: "apply (a row's `source` cell, in a batch of one call)",
        free_text: true,
        prepare: |_| {},
        call: |rig, text, extra| {
            let mut entry = row("audit-f3");
            entry["source"] = json!(text);
            let mut inner = ledger_add(RUN);
            inner.extend_from_slice(extra);
            let mut args = apply(RUN, Some("1"));
            args.extend_from_slice(extra);
            let batch = json!([{"argv": inner, "stdin": entry.to_string()}]);
            (rig.run(&args, &batch.to_string()), ledger_path())
        },
    },
];

/// The subcommands that write nothing.
const READERS: [&str; 5] = [
    "check-reports",
    "check-ledger",
    "state",
    "gate-check",
    "pending",
];

/// The subcommands that settle a batch and take no text of a caller's: they finish a write
/// that was killed, take an applied batch back, or forget it once it is committed.
const KEEPERS: [&str; 3] = ["discard", "settle", "recover"];

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
        .chain(KEEPERS.map(str::to_owned))
        .collect();
    assert_eq!(
        named, known,
        "the script's subcommands (left) are the listed writers, the readers and the three that settle a batch (right)"
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
    let mut fix = strings(&["report", "--round=1", "--stage=fix", "--cycle=3"]);
    fix.extend(strings(&["--reporter=fixer-install", "--attempt=1"]));
    fix.push(format!("--run={RUN}"));
    let seen = rig.run(&fix, body);
    seen.must(OK, "a fix-stage report");
    let path = format!("completions/artifacts/{RUN}/r1/reports/fix/c3/fixer-install.a1.md");
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
        name: "result-set --run",
        numeric: false,
        call: |rig, v| {
            rig.run(
                &result_set(v, "1", &sha('a')),
                &ran("row-setup", "green").to_string(),
            )
        },
    },
    IdSite {
        name: "result-set --round",
        numeric: true,
        call: |rig, v| {
            rig.run(
                &result_set(RUN, v, &sha('a')),
                &ran("row-setup", "green").to_string(),
            )
        },
    },
    IdSite {
        name: "result-set, a result's item",
        numeric: false,
        call: |rig, v| {
            rig.run(
                &result_set(RUN, "1", &sha('a')),
                &ran(v, "green").to_string(),
            )
        },
    },
    IdSite {
        name: "run-set, a clause of the census",
        numeric: false,
        call: |rig, v| rig.run(&run_set(RUN), &json!({"clauses": [v]}).to_string()),
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
    IdSite {
        name: "item-set --run",
        numeric: false,
        call: |rig, v| rig.run(&item_set(v), &item("row-setup").to_string()),
    },
    IdSite {
        name: "item-set, an item's id",
        numeric: false,
        call: |rig, v| rig.run(&item_set(RUN), &item(v).to_string()),
    },
    IdSite {
        name: "item-set, an item's kind",
        numeric: false,
        call: |rig, v| {
            let mut given = item("row-setup");
            given["kind"] = json!(v);
            rig.run(&item_set(RUN), &given.to_string())
        },
    },
    IdSite {
        name: "item-set, an item's clause",
        numeric: false,
        call: |rig, v| {
            let mut given = item("row-setup");
            given["clause"] = json!(v);
            rig.run(&item_set(RUN), &given.to_string())
        },
    },
    IdSite {
        name: "round-set --run",
        numeric: false,
        call: |rig, v| rig.run(&round_set(v, "1"), &json!({"cycles": 1}).to_string()),
    },
    IdSite {
        name: "round-set --round",
        numeric: true,
        call: |rig, v| rig.run(&round_set(RUN, v), &json!({"cycles": 1}).to_string()),
    },
    IdSite {
        name: "round-set, the clause granted one more re-run",
        numeric: false,
        call: |rig, v| rig.run(&round_set(RUN, "1"), &json!({"granted": v}).to_string()),
    },
    IdSite {
        name: "run-set --run",
        numeric: false,
        call: |rig, v| rig.run(&run_set(v), &json!({"rounds": 3}).to_string()),
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
    assert!(driven > 780, "the axis collapsed to {driven} cells");
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
            "result-set",
            rig.run(
                &result_set(run, "1", &sha('a')),
                &ran("row-setup", "green").to_string(),
            ),
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
        (
            "item-set",
            rig.run(&item_set(run), &item("row-setup").to_string()),
        ),
        (
            "round-set",
            rig.run(&round_set(run, "1"), &json!({"cycles": 1}).to_string()),
        ),
        (
            "run-set",
            rig.run(&run_set(run), &json!({"rounds": 3}).to_string()),
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
    symlink(&outside, rig.run_dir().join("run.md")).expect("link the run's facts");
    fs::create_dir_all(rig.run_dir().join("r1")).expect("a round directory");
    symlink(&outside, rig.run_dir().join("r1/scope.md")).expect("link a round's scope");
    let before = rig.snapshot();
    rig.run(&ledger_add(RUN), &row("audit-f3").to_string())
        .refused(OUTSIDE, "a ledger that is a link");
    rig.run(&check_ledger(RUN, &["audit-f3"]), "")
        .refused(OUTSIDE, "a check of a ledger that is a link");
    rig.run(&item_set(RUN), &item("row-setup").to_string())
        .refused(
            OUTSIDE,
            "a test-set row under a clause table that is a link: the table is read first",
        );
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .refused(OUTSIDE, "a bounds list that is a link");
    rig.run(&run_set(RUN), &json!({"rounds": 3}).to_string())
        .refused(OUTSIDE, "the run's facts, a link");
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
    // A disposition's detail is free text too — and so is the reason of a void run, which
    // the clause table is rendered with: the view says it again, inside a cell of its own.
    let patch = json!({"key": "hostile-0", "disposition": "bound", "detail": hostile});
    rig.run(&ledger_set(RUN), &patch.to_string())
        .must(OK, "a hostile disposition detail");
    rig.run(
        &run_set(RUN),
        &json!({"clauses": ["no-lost-files"]}).to_string(),
    )
    .must(OK, "the census");
    rig.run(&item_set(RUN), &item("row-setup").to_string())
        .must(OK, "the item");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&["jigc setup"], &[]).to_string(),
    )
    .must(OK, "the round's scope");
    let mut void = ran("row-setup", "void");
    void["reason"] = json!(hostile);
    rig.run(&result_set(RUN, "1", &sha('a')), &void.to_string())
        .must(OK, "a hostile reason");
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": sha('a')}).to_string(),
    )
    .must(
        OK,
        "the round, tested: the view is rendered with the reason",
    );

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
    let results = table(&rig.read(&results_path("1")), &RESULT_COLUMNS);
    assert_eq!(results.len(), 1, "one result row");
    assert!(
        results[0][4].starts_with("a | b<br>second line<br>| `injected` | x |<br>|---|---| tab "),
        "the reason keeps what it was given: {results:?}"
    );
    let clauses = table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS);
    assert_eq!(clauses.len(), 1, "one clause row");
    assert!(
        clauses[0][1].contains(&results[0][4]) && clauses[0][4] == "void",
        "the view says the reason again, and is one row still: {clauses:?}"
    );
    // The view is the one the results give, read back cell for cell: the state is read —
    // and gives the cell back as data, its pipes unescaped.
    let read = rig.state();
    let text = |value: &Value| value.as_str().expect("text").to_owned();
    assert_eq!(read["clauses"][0]["status"], json!("void"));
    assert!(
        text(&read["clauses"][0]["instrument"])
            .contains(": a | b<br>second line<br>| `injected` | x |")
            && text(&read["clauses"][0]["items"][0]["reason"]).starts_with("a | b<br>second line")
            && text(&read["rounds"][0]["results"][0]["reason"]).starts_with("a | b<br>second line"),
        "{read}"
    );

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
    let items: Vec<String> = (0..3).map(|n| format!("row-{n}")).collect();
    std::thread::scope(|scope| {
        for key in &keys {
            let rig = &rig;
            scope.spawn(move || {
                rig.run(&ledger_add(RUN), &row(key).to_string())
                    .must(OK, key);
            });
        }
        for id in &items {
            let rig = &rig;
            scope.spawn(move || {
                rig.run(&item_set(RUN), &item(id).to_string()).must(OK, id);
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
        first_cells(rig.read(&test_set_path()), &ITEM_COLUMNS),
        items.into_iter().collect()
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
// 10 · An item's results, and the per-clause table that is rendered from them
// ---------------------------------------------------------------------------

/// The run of the review's first HIGH finding, opened: one clause judged by a check and by
/// two hunts, each behind a door of its own.
fn three_item_run(label: &str) -> Rig {
    let rig = Rig::new(label);
    rig.run(
        &run_set(RUN),
        &format!(r#"{{{BOUNDED}, {RELEASE}, "clauses": ["no-lost-files"]}}"#),
    )
    .must(OK, "the run's facts, and its census");
    let mut gate = item("gate");
    gate["kind"] = json!("check");
    gate["runs"] = json!("every-candidate");
    gate["doors"] = json!([]);
    let setup = item("review-setup");
    let mut show = item("review-show");
    show["doors"] = json!([EXCLUDED]);
    rig.run(&item_set(RUN), &json!([gate, setup, show]).to_string())
        .must(OK, "the test set");
    rig
}

/// A round of [`three_item_run`], recorded as its `test` stage does: its doors, what each
/// item did, its candidate.
fn stage(rig: &Rig, n: usize, doors: &Value, results: &[(&str, &str)]) {
    rig.run(&scope_set(RUN, &n.to_string()), &doors.to_string())
        .must(OK, "a round's scope");
    let rows: Vec<Value> = results
        .iter()
        .map(|(item, outcome)| ran(item, outcome))
        .collect();
    rig.run(
        &result_set(RUN, &n.to_string(), &candidate(n)),
        &json!(rows).to_string(),
    )
    .must(OK, "what the round's items did");
    rig.run(
        &round_set(RUN, &n.to_string()),
        &json!({"candidate": candidate(n)}).to_string(),
    )
    .must(OK, "the round's candidate");
}

/// A result is recorded per test-set item, per round: which item ran, which attempt of it,
/// on which commit, with which outcome — green, void with its reason, red with the finding
/// it is filed as. The attempt is computed; and what is no run of an item the round's
/// scope selected is refused, leaving nothing.
#[test]
fn an_items_result_is_recorded_per_round_with_its_attempt_computed() {
    let rig = three_item_run("results");
    let before = rig.snapshot();
    rig.run(
        &result_set(RUN, "1", &sha('a')),
        &ran("gate", "green").to_string(),
    )
    .refused(NO_SCOPE, "a result in a round that has no scope");
    assert_eq!(rig.snapshot(), before);
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");

    let before = rig.snapshot();
    let commit = sha('a');
    let on = commit.as_str();
    let refused: &[(Value, &str, &str, i32)] = &[
        (
            json!({"item": "gate", "outcome": "passed"}),
            on,
            "an outcome outside green, red and void",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "void"}),
            on,
            "void, and no reason",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "void", "reason": "  "}),
            on,
            "void, with a reason that says nothing",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "green", "reason": "why not"}),
            on,
            "a reason beside green",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "red"}),
            on,
            "red, and nothing of its finding",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "red", "doctype": "jigc-feedback", "door": "the gate"}),
            on,
            "red, and no repro",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "red", "doctype": "a-note", "door": "the gate", "repro": "r"}),
            on,
            "red, filed under a doctype nobody defined",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "green", "repro": "r"}),
            on,
            "a finding's field beside green",
            BAD_VALUE,
        ),
        (
            json!({"item": "gate", "outcome": "green", "attempt": 1}),
            on,
            "an attempt supplied: it is computed",
            BAD_VALUE,
        ),
        (
            json!([ran("gate", "green"), ran("gate", "green")]),
            on,
            "an item twice in one call",
            BAD_VALUE,
        ),
        (
            ran("gate", "green"),
            "0f34d8f0",
            "a commit that is not a full sha",
            BAD_VALUE,
        ),
        (
            ran("review-row-nobody-listed", "green"),
            on,
            "an item the test set lacks",
            NO_SUCH_ROW,
        ),
        (
            ran("review-show", "green"),
            on,
            "an item the round's scope does not select",
            BAD_VALUE,
        ),
    ];
    for (given, commit, why, status) in refused {
        rig.run(&result_set(RUN, "1", commit), &given.to_string())
            .refused(*status, why);
    }
    assert_eq!(rig.snapshot(), before, "a refused result writes nothing");

    let seen = rig.run(
        &result_set(RUN, "1", on),
        &json!([ran("gate", "green"), ran("review-setup", "void")]).to_string(),
    );
    seen.must(OK, "what round 1's items did");
    assert_eq!(
        seen.json(),
        json!({"results": results_path("1"), "findings": [], "recorded": [
            {"item": "gate", "attempt": 1, "outcome": "green"},
            {"item": "review-setup", "attempt": 1, "outcome": "void"}]})
    );
    let ticked = format!("`{commit}`");
    assert_eq!(
        table(&rig.read(&results_path("1")), &RESULT_COLUMNS),
        [
            ["`gate`", "1", ticked.as_str(), "green", "-"],
            [
                "`review-setup`",
                "1",
                ticked.as_str(),
                "void",
                "its driver died"
            ],
        ]
    );
    // A round that is not tested yet has one result per item: its test stage's.
    let before = rig.snapshot();
    rig.run(&result_set(RUN, "1", on), &ran("gate", "green").to_string())
        .refused(
            EXISTS,
            "a second result of an item in a round that is not tested",
        );
    // And a round tests one candidate: the commit its results are of.
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": sha('b')}).to_string(),
    )
    .refused(
        BAD_VALUE,
        "a candidate that is not the commit the results are of",
    );
    assert_eq!(rig.snapshot(), before);
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": commit}).to_string(),
    )
    .must(OK, "the round's candidate");
    assert_eq!(
        rig.state()["rounds"][0]["results"],
        json!([
            {"item": "gate", "attempt": 1, "commit": commit, "outcome": "green", "reason": null, "finding": null},
            {"item": "review-setup", "attempt": 1, "commit": commit, "outcome": "void",
             "reason": "its driver died", "finding": null},
        ])
    );

    // A candidate is the candidate of a round that has a scope.
    fs::create_dir_all(rig.run_dir().join("r2")).expect("a round directory");
    let before = rig.snapshot();
    rig.run(
        &round_set(RUN, "2"),
        &json!({"candidate": sha('b')}).to_string(),
    )
    .refused(NO_SCOPE, "a candidate for a round with no scope");
    assert_eq!(rig.snapshot(), before);
}

/// **The first of the review's HIGH findings, and the repair plan's state 1.** Three items
/// judge one clause. Round 1 leaves one hunt void; its fix lands; round 2's scope does not
/// select that hunt, and everything round 2 ran is green. The run answers the re-run of
/// that hunt — an attempt of round 1 — and never `close`: an instrument that never ran to
/// its end is not turned green by a later round that ran the others.
#[test]
fn an_instrument_that_never_ran_to_its_end_is_not_turned_green_by_a_later_round_that_ran_the_others()
 {
    let rig = three_item_run("f1");
    stage(
        &rig,
        1,
        &scope(&[INSIDE, EXCLUDED], &[]),
        &[
            ("gate", "green"),
            ("review-setup", "void"),
            ("review-show", "green"),
        ],
    );
    rig.run(&ledger_add(RUN), &row_at("f-1", EXCLUDED).to_string())
        .must(OK, "a finding of round 1");
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "f-1", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "its triage");
    assert_eq!(rig.state()["next"], json!("fix"));
    rig.run(
        &ledger_set(RUN),
        &json!({"key": "f-1", "disposition": "fixed", "detail": sha('f')}).to_string(),
    )
    .must(OK, "its fix");
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "the fix cycle");
    assert_eq!(rig.state()["next"], json!("test"));

    // Round 2: the fix's door only. It selects the check and one hunt, and both are green.
    stage(
        &rig,
        2,
        &scope(&[EXCLUDED], &[INSIDE]),
        &[("gate", "green"), ("review-show", "green")],
    );
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["retest"], &read["forbids_close"]),
        (
            &json!("retest"),
            &json!(["no-lost-files"]),
            &json!([{"clause": "no-lost-files", "status": "void"}])
        ),
        "never `close`: {read}"
    );
    assert_eq!(
        read["position"]["test"],
        json!({"round": 1, "attempt": 1, "rerun": {
            "clause": "no-lost-files", "round": 1,
            "items": [{"item": "review-setup", "attempt": 2}],
            "doors": [door(INSIDE), door(EXCLUDED)]}}),
        "the re-run of that hunt, inside the round that selected it: {read}"
    );
    // The clause table says so, item by item — each one's own run, in its own round.
    let commit = format!("`{}`", candidate(2));
    assert_eq!(
        table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS),
        [[
            "`no-lost-files`",
            "`gate` — green (round 2, attempt 1)<br>`review-setup` — void (round 1, attempt 1): its driver died<br>`review-show` — green (round 2, attempt 1)",
            commit.as_str(),
            "round 1, round 2",
            "void",
        ]]
    );

    // The hunt runs to its end on the candidate, as attempt 2 of round 1: now it closes.
    rerun(&rig, 1, 2, "review-setup", "green").must(OK, "the hunt's re-run");
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["forbids_close"], &read["round"]),
        (&json!("close"), &json!([]), &json!(2)),
        "{read}"
    );
}

/// **The per-clause table is a view, rendered by the script from the item results, and
/// nothing sets a clause's status.** It holds a row per clause of the census, written once
/// at the opening; and a table that differs from what the results give — a status a hand
/// changed, a row or the file deleted, a row added, a cell reworded — is refused by `state`
/// and by every writer that renders it, naming what differs: never read as it stands, and
/// never quietly rendered over.
#[test]
fn the_clause_table_is_rendered_from_the_item_results_and_a_table_that_differs_is_refused() {
    let rig = three_item_run("view");
    // The census is written once, and the table has its rows from that moment on.
    let before = rig.snapshot();
    rig.run(
        &run_set(RUN),
        &json!({"clauses": ["no-lost-files", "no-regression"]}).to_string(),
    )
    .refused(EXISTS, "a second census");
    for (said, why) in [
        (json!({"clauses": []}), "a census of no clause"),
        (
            json!({"clauses": "no-lost-files"}),
            "a census that is no list",
        ),
        (json!({"clauses": ["a", "a"]}), "a clause named twice"),
    ] {
        rig.run(&run_set(RUN), &said.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);
    rig.run(
        &run_set(RUN),
        &json!({"clauses": ["no-lost-files"]}).to_string(),
    )
    .must(OK, "the census said again, as it stands");
    assert_eq!(
        table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS),
        [[
            "`no-lost-files`",
            "`gate` — not selected by any tested round: it owes nothing<br>`review-setup` — not selected by any tested round: it owes nothing<br>`review-show` — not selected by any tested round: it owes nothing",
            "-",
            "-",
            "void",
        ]],
        "before any round is tested no item is owed, and the clause is void"
    );
    assert!(
        !subcommands(&rig)
            .iter()
            .any(|name| name.starts_with("clause")),
        "no subcommand writes a clause's row"
    );

    let all_green = [
        ("gate", "green"),
        ("review-setup", "green"),
        ("review-show", "green"),
    ];
    stage(&rig, 1, &scope(&[INSIDE, EXCLUDED], &[]), &all_green);
    let commit = format!("`{}`", candidate(1));
    let green = [[
        "`no-lost-files`",
        "`gate` — green (round 1, attempt 1)<br>`review-setup` — green (round 1, attempt 1)<br>`review-show` — green (round 1, attempt 1)",
        commit.as_str(),
        "round 1",
        "green",
    ]];
    assert_eq!(table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS), green);
    assert_eq!(rig.state()["next"], json!("close"));

    // What a hand does to it. Each is refused by the state, by its own word for it — and
    // by each writer that would otherwise render the table again, which leaves the tree.
    let held = rig.read(&clauses_path());
    let path = rig.root.join(clauses_path());
    let row = held.lines().last().expect("the clause's row").to_owned();
    let damaged: &[(String, &str, &str)] = &[
        (
            held.replace("| green |", "| void |"),
            "says \"void\" of `no-lost-files` where the results give \"green\"",
            "a status changed by hand",
        ),
        (
            held.replace(&format!("{row}\n"), ""),
            "has no row for `no-lost-files`",
            "the clause's row deleted",
        ),
        (
            format!("{held}| `no-regression` | - | - | - | green |\n"),
            "holds a row for `no-regression`, which is no clause of the run",
            "a row added by hand",
        ),
        (
            held.replace("| round 1 |", "| every door |"),
            "holds a row for `no-lost-files` whose cells are not the ones the results give",
            "a cell reworded by hand",
        ),
    ];
    for (text, names, why) in damaged {
        assert_ne!(text, &held, "{why}: the damage is done");
        fs::write(&path, text).expect("damage the clause table");
        let before = rig.snapshot();
        let seen = rig.run(&state(RUN), "");
        seen.refused(CORRUPT, why);
        assert!(seen.stderr.contains(names), "{why}: {}", seen.stderr);
        for (args, body, writer) in [
            (run_set(RUN), json!({"rounds": 4}), "run-set"),
            (item_set(RUN), item("review-rename"), "item-set"),
            (round_set(RUN, "1"), json!({"cycles": 1}), "round-set"),
            (
                result_set(RUN, "1", &candidate(1)),
                ran("gate", "green"),
                "result-set",
            ),
        ] {
            rig.run(&args, &body.to_string())
                .refused(CORRUPT, &format!("{writer}, over {why}"));
        }
        assert_eq!(rig.snapshot(), before, "{why}: nothing renders over it");
    }
    fs::remove_file(&path).expect("delete the clause table");
    let seen = rig.run(&state(RUN), "");
    seen.refused(CORRUPT, "the clause table deleted");
    assert!(
        seen.stderr.contains("has no row for `no-lost-files`"),
        "a deleted table is a table without its rows: {}",
        seen.stderr
    );
    fs::write(&path, &held).expect("take the table back");
    assert_eq!(rig.state()["next"], json!("close"));

    // And every writer whose table the derivation reads renders it in its own batch: a
    // fix cycle on record, and no run is current.
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "a fix cycle");
    let said = table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS);
    assert_eq!(
        (
            said[0][4].as_str(),
            said[0][1].matches("void: not current").count()
        ),
        ("void", 3),
        "{said:?}"
    );
    // An item that takes its row later owes a run where a tested round selects it.
    stage(&rig, 2, &scope(&[INSIDE, EXCLUDED], &[]), &all_green);
    assert_eq!(rig.state()["next"], json!("close"));
    let mut late = item("regression-set");
    late["kind"] = json!("check");
    late["runs"] = json!("every-candidate");
    late["doors"] = json!([]);
    rig.run(&item_set(RUN), &late.to_string())
        .must(OK, "an instrument that was built later");
    let read = rig.state();
    assert_eq!(
        (
            &read["next"],
            &read["clauses"][0]["status"],
            &read["position"]["test"]["rerun"]["items"]
        ),
        (
            &json!("retest"),
            &json!("void"),
            &json!([{"item": "regression-set", "attempt": 1}])
        ),
        "{read}"
    );
}

/// **A red deterministic check becomes a finding with a ledger row** — by the call that
/// records the red, so that no caller can record one and file nothing: ungraded and open,
/// under a key of the item's own, it is the next triage's to grade and then routed, ruled
/// or fixed like any other. A red result whose finding the ledger lacks is refused; and a
/// check that is red again after its finding was fixed opens that finding again.
#[test]
fn a_red_run_is_filed_as_a_finding_and_is_open_again_when_it_is_red_after_its_fix() {
    let rig = three_item_run("red");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");
    let mut red = ran("gate", "red");
    red["repro"] = json!("the gate's own output: two tests red");
    let seen = rig.run(
        &result_set(RUN, "1", &candidate(1)),
        &json!([red, ran("review-setup", "green")]).to_string(),
    );
    seen.must(OK, "a red check, and a green hunt");
    assert_eq!(seen.json()["findings"], json!(["red-gate"]));
    assert_eq!(
        table(&rig.read(&ledger_path()), &LEDGER_COLUMNS),
        [[
            "`red-gate`",
            "`jigc-feedback`",
            "1",
            "the item `gate` of the test set, red in round 1 (attempt 1)",
            INSIDE,
            "no-lost-files",
            "ungraded",
            "-",
            "open",
            "the gate's own output: two tests red",
        ]],
        "the finding's row, as the call that recorded the red wrote it"
    );
    assert_eq!(
        table(&rig.read(&results_path("1")), &RESULT_COLUMNS)[0][3..],
        ["red", "`red-gate`"]
    );
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": candidate(1)}).to_string(),
    )
    .must(OK, "the round's candidate");
    // It is handed to somebody: the round's triage is not finished until it is graded.
    let read = rig.state();
    assert_eq!(
        (
            &read["next"],
            &read["untriaged"],
            &read["clauses"][0]["status"]
        ),
        (
            &json!("triage"),
            &json!([{"key": "red-gate", "why": "ungraded"}]),
            &json!("red")
        ),
        "{read}"
    );
    // The finding is the ledger's to hold: its row deleted is a state that is not read.
    let ledger = rig.read(&ledger_path());
    let kept: String = ledger
        .lines()
        .filter(|line| !line.contains("`red-gate`"))
        .map(|line| format!("{line}\n"))
        .collect();
    fs::write(rig.root.join(ledger_path()), kept).expect("delete the finding's row");
    let seen = rig.run(&state(RUN), "");
    seen.refused(CORRUPT, "a red result whose finding is gone");
    assert!(seen.stderr.contains("lacks red-gate"), "{}", seen.stderr);
    fs::write(rig.root.join(ledger_path()), ledger).expect("take the ledger back");

    // Graded, verified, fixed like any other: inside the round's test set, so a fixer's.
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "red-gate", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "its triage");
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["blockers"]),
        (&json!("fix"), &json!(["red-gate"])),
        "{read}"
    );
    rig.run(
        &ledger_set(RUN),
        &json!({"key": "red-gate", "disposition": "fixed", "detail": sha('f')}).to_string(),
    )
    .must(OK, "its fix");
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "the fix cycle");

    // Round 2: the check is red again. Its finding keeps its row and does not inherit
    // `fixed` — the fix did not hold — so it is a fixer's again, and nothing closes.
    rig.run(
        &scope_set(RUN, "2"),
        &scope(&[EXCLUDED], &[INSIDE]).to_string(),
    )
    .must(OK, "round 2's scope");
    let mut again = ran("gate", "red");
    again["repro"] = json!("the gate's own output, on the next candidate");
    let seen = rig.run(
        &result_set(RUN, "2", &candidate(2)),
        &json!([again, ran("review-show", "green")]).to_string(),
    );
    seen.must(OK, "red again");
    assert_eq!(seen.json()["findings"], json!(["red-gate"]));
    rig.run(
        &round_set(RUN, "2"),
        &json!({"candidate": candidate(2)}).to_string(),
    )
    .must(OK, "round 2's candidate");
    let rows = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    assert_eq!(
        (rows.len(), rows[0][8].as_str(), rows[0][9].as_str()),
        (1, "open", "the gate's own output: two tests red"),
        "one row per finding: found again, it keeps its row and is open again"
    );
    let read = rig.state();
    assert_eq!(
        (
            &read["next"],
            &read["blockers"],
            &read["clauses"][0]["status"]
        ),
        (&json!("fix"), &json!(["red-gate"]), &json!("red")),
        "{read}"
    );
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
    assert!(subcommands.len() >= 13, "the parser names its subcommands");
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
    statuses.push((GATE_MISMATCH, "gate-check"));
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

    let seen = rig.run(&scope_set(RUN, "1"), &given.to_string());
    seen.must(OK, "a round's scope");
    assert_eq!(
        seen.json(),
        json!({"scope": scope_path("1"), "included": 2, "excluded": 1})
    );
    assert_eq!(
        table(&rig.read(&scope_path("1")), &SCOPE_COLUMNS),
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
    assert_eq!(read["round"], json!(1));
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
    rig.run(&scope_set(RUN, "1"), &scope(&[], &[]).to_string())
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
        rig.run(&scope_set(RUN, "1"), input).refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);

    // A round that tests nothing says so, with both sides empty.
    let empty = Rig::new("scope-empty");
    empty
        .run(&scope_set(RUN, "1"), &scope(&[], &[]).to_string())
        .must(OK, "a scope with no door");
    assert_eq!(
        empty.state()["doors"],
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
            vec!["`f-in`", "inside", "breaks", "confirmed", "no", "open", "-"],
            vec![
                "`f-out`",
                "outside: excluded",
                "unclear",
                "confirmed",
                "yes",
                "open",
                "-"
            ],
            vec![
                "`f-unlisted`",
                "outside: unlisted",
                "breaks",
                "refuted",
                "-",
                "open",
                "-"
            ],
            vec![
                "`f-bounded`",
                "inside",
                "out-of-scope: non-jigc-writer",
                "-",
                "-",
                "open",
                "-",
            ],
            vec!["`f-waiting`", "inside", "unclear", "-", "-", "open", "-"],
            vec![
                "`f-fixed`",
                "inside",
                "needs-bound",
                "-",
                "-",
                "fixed: 0f34d8f0",
                "-"
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
        [
            "`f-waiting`",
            "inside",
            "unclear",
            "refuted",
            "-",
            "open",
            "-"
        ]
    );

    // A FORK IS A CELL OF THE ENTRY — the harness review's `M3`: the verifier found the
    // finding to contest a settled decision, an advocate argued the case, and an agent that
    // is not the advocate drove the proposal. It is recorded with the verdict it rides on,
    // replaces the row like any entry recorded again, and is gone when the entry is
    // recorded again without it.
    let contested = |fork: Value| json!({"key": "f-in", "grade": "breaks", "verdict": "confirmed", "regression": false, "fork": fork});
    let fork = json!({"kind": "contested", "case": "robust-now", "drive": "differs"});
    let before = rig.snapshot();
    for (what, entry) in [
        (
            "a fork on a finding nobody verified",
            json!({"key": "f-fixed", "grade": "needs-bound", "fork": fork}),
        ),
        (
            "a fork of no kind",
            contested(json!({"case": "robust-now", "drive": "holds"})),
        ),
        (
            "a fork of a kind nobody raises here",
            contested(json!({"kind": "disputed", "case": "robust-now", "drive": "holds"})),
        ),
        (
            "a case that is no verdict of an advocate",
            contested(json!({"kind": "contested", "case": "maybe", "drive": "holds"})),
        ),
        (
            "a drive that says neither",
            contested(json!({"kind": "contested", "case": "robust-now", "drive": "undriven"})),
        ),
        (
            "a fork with a field nobody defined",
            contested(
                json!({"kind": "contested", "case": "robust-now", "drive": "holds", "note": "x"}),
            ),
        ),
        ("a fork that is a word", contested(json!("contested"))),
    ] {
        rig.run(&triage_set(RUN, "1"), &entry.to_string())
            .refused(BAD_VALUE, what);
        assert_eq!(rig.snapshot(), before, "{what}: nothing was written");
    }
    rig.run(&triage_set(RUN, "1"), &contested(fork.clone()).to_string())
        .must(OK, "a contested finding, with its fork");
    let rows = table(&rig.read(&triage_path("1")), &TRIAGE_COLUMNS);
    assert_eq!(
        rows[0],
        [
            "`f-in`",
            "inside",
            "breaks",
            "confirmed",
            "no",
            "open",
            "contested: robust-now, differs"
        ]
    );
    let read = rig.state();
    let row = |read: &Value, key: &str| -> Value {
        read["ledger"]
            .as_array()
            .expect("the ledger's rows")
            .iter()
            .find(|entry| entry["key"] == key)
            .unwrap_or_else(|| panic!("no row `{key}` in {read}"))
            .clone()
    };
    assert_eq!(
        row(&read, "f-in")["triage"]["fork"],
        fork,
        "the fork, read back as data"
    );
    assert_eq!(row(&read, "f-out")["triage"]["fork"], Value::Null);
    assert_eq!(
        json!([
            row(&read, "f-in")["route"],
            row(&read, "f-in")["why"],
            read["next"]
        ]),
        json!(["human", "fork", "not-ready"]),
        "a fork nobody ruled is the human's, whatever its door"
    );
    // A cell a hand wrote is no fork.
    let text = rig.read(&triage_path("1"));
    fs::write(
        rig.root.join(triage_path("1")),
        text.replacen("contested: robust-now, differs", "contested: surely", 1),
    )
    .expect("edit the triage record");
    rig.run(&state(RUN), "")
        .refused(CORRUPT, "a fork cell somebody wrote by hand");
    fs::write(rig.root.join(triage_path("1")), text).expect("put the record back");
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "f-in", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "the same entry again, without a fork");
    assert_eq!(
        table(&rig.read(&triage_path("1")), &TRIAGE_COLUMNS)[0][6],
        "-"
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
            "fork": null,
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
    assert_eq!(rig.snapshot(), before);
    let scopeless = Rig::new("triage-no-scope");
    scopeless.seed_rows(&["f-in"]);
    let before = scopeless.snapshot();
    scopeless
        .run(
            &triage_set(RUN, "1"),
            &json!({"key": "f-in", "grade": "no-break"}).to_string(),
        )
        .refused(NO_SCOPE, "a round whose scope was never written");
    assert_eq!(scopeless.snapshot(), before);
}

// ---------------------------------------------------------------------------
// 17 · The state, as one document
// ---------------------------------------------------------------------------

#[test]
fn state_is_the_runs_committed_state_as_one_json_document() {
    let rig = Rig::new("state");

    // An opened run with nothing in it — and reading it writes nothing. Its opening owes
    // the clause rows and every fact of the run, so the run is not ready and no stage
    // starts.
    let before = rig.snapshot();
    assert_eq!(
        rig.state(),
        json!({
            "run": RUN,
            "opened": true,
            "facts": {
                "stop": null,
                "rounds": null,
                "cycles": null,
                "previous": null,
                "previous-commit": null,
                "scope": null,
                "clauses": null,
                "findings": null,
            },
            "not_ready": [
                "no-clause-row",
                "no-stop-mode",
                "no-previous-release",
                "no-default-scope",
            ],
            "rounds": [],
            "round": null,
            // Round 1 is the round a write may begin: the opening's own order is not held.
            "begins": 1,
            "doors": null,
            "bounds": [],
            "clauses": [],
            "items": [],
            "ledger": [],
            "blockers": [],
            "human_list": [],
            "human_stages": [],
            "candidate": {"round": null, "commit": null, "current": false},
            "fix_rounds": 0,
            "untriaged": [],
            "retest": [],
            "human_clauses": [],
            "unsettled": [],
            "forbids_close": [
                {"clauses": "no row"},
                {"candidate": "not-tested", "round": null},
            ],
            "next": "not-ready",
            "stop": null,
            "position": {
                "test": {"refused": "not-ready", "round": null},
                "fix": {"refused": "not-ready", "round": null},
            },
        })
    );
    assert_eq!(rig.snapshot(), before, "the state is read, never written");

    // Round 1: two test-stage reports and a re-run of one, two fix cycles, a scope, a
    // triage record and a round record — and it was dropped. Round 2: one report, and
    // nothing else yet; it is begun only once round 1 is over.
    for (round, reporter, attempt) in [
        ("1", "audit-install", "1"),
        ("1", "review-source", "1"),
        ("1", "review-source", "2"),
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
    let mut check = item("the-gate");
    check["kind"] = json!("check");
    check["runs"] = json!("every-candidate");
    check["doors"] = json!([]);
    rig.run(
        &item_set(RUN),
        &json!([check, item("row-setup")]).to_string(),
    )
    .must(OK, "the test set");
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "a declared bound");
    rig.run(
        &run_set(RUN),
        &format!(r#"{{{BOUNDED}, {RELEASE}, "clauses": ["no-lost-files", "no-regression"]}}"#),
    )
    .must(OK, "the run's facts, and its census");
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");
    // What round 1's two items did — the check green, the review row void — and then the
    // round's record: its candidate, its binary, and two fix cycles.
    let mut void = ran("row-setup", "void");
    void["reason"] = json!("its driver | died");
    rig.run(
        &result_set(RUN, "1", &sha('c')),
        &json!([ran("the-gate", "green"), void]).to_string(),
    )
    .must(OK, "round 1's results");
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": sha('c'), "binary": sha256('b')}).to_string(),
    )
    .must(OK, "round 1's record");
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
    let before = rig.snapshot();
    rig.run(&report(RUN, "2", "audit-install", "1"), BODY)
        .refused(
            BAD_VALUE,
            "a report of round 2, while round 1 is still open",
        );
    assert_eq!(rig.snapshot(), before);
    rig.run(
        &round_set(RUN, "1"),
        &json!({"cycles": 2, "outcome": "dropped"}).to_string(),
    )
    .must(OK, "round 1's two fix cycles, and the human's exit from it");
    rig.run(&report(RUN, "2", "audit-install", "1"), BODY)
        .must(OK, "round 2's first report");

    assert_eq!(
        rig.state(),
        json!({
            "run": RUN,
            "opened": true,
            "facts": {
                "stop": "at-the-bound",
                "rounds": 3,
                "cycles": null,
                "previous": "1.0.0-rc.24",
                "previous-commit": sha('e'),
                "scope": "delta",
                "clauses": ["no-lost-files", "no-regression"],
                "findings": ["audit-f3", "audit-f4"],
            },
            "not_ready": [],
            "rounds": [
                {
                    "round": 1,
                    "scope": true,
                    "triage": true,
                    "record": true,
                    "facts": {
                        "candidate": sha('c'),
                        "base": null,
                        "binary": sha256('b'),
                        "cycles": 2,
                        "outcome": "dropped",
                        "go": false,
                        "granted": [],
                        "reverify": [],
                        "again": [],
                        "cross-model": [],
                        "cross-model-void": [],
                        "ran": ["the-gate a1", "row-setup a1"],
                        "awaiting": [],
                        "recorded": [],
                    },
                    "results": [
                        {"item": "the-gate", "attempt": 1, "commit": sha('c'), "outcome": "green",
                         "reason": null, "finding": null},
                        {"item": "row-setup", "attempt": 1, "commit": sha('c'), "outcome": "void",
                         "reason": "its driver | died", "finding": null},
                    ],
                    "reopened": [],
                    "test_reports": 3,
                    "test_attempt": 2,
                    // No attempt reached its record through a batch: each one on disk is
                    // an attempt that left reports and nothing else.
                    "test_unrecorded": 2,
                    "fix_cycles": [
                        {"cycle": 1, "reports": 1, "attempt": 1, "unrecorded": 1},
                        {"cycle": 2, "reports": 1, "attempt": 1, "unrecorded": 1},
                    ],
                },
                {
                    "round": 2,
                    "scope": false,
                    "triage": false,
                    "record": false,
                    "facts": {
                        "candidate": null,
                        "base": null,
                        "binary": null,
                        "cycles": 0,
                        "outcome": null,
                        "go": false,
                        "granted": [],
                        "reverify": [],
                        "again": [],
                        "cross-model": [],
                        "cross-model-void": [],
                        "ran": [],
                        "awaiting": [],
                        "recorded": [],
                    },
                    "results": [],
                    "reopened": [],
                    "test_reports": 1,
                    "test_attempt": 1,
                    "test_unrecorded": 1,
                    "fix_cycles": [],
                },
            ],
            "round": 2,
            "begins": null,
            "doors": null,
            "bounds": [{
                "bound": "non-jigc-writer",
                "reach": "races against a writer that is not jigc",
                "ruling": "the opening record, declared bounds",
                "pin": "unpinned",
            }],
            // Round 1 selected both items of the first clause, and a fix stage is on
            // record in it that no tested round has followed: neither run is current, so
            // the clause is void, one fix round behind, and nothing is run again yet — a
            // round that is begun and not tested changes nothing here. The second clause
            // has no item of the test set at all.
            "clauses": [
                {
                    "clause": "no-lost-files",
                    "instrument": "`the-gate` — void: not current — round 1 selected it, a fix stage is on record since, and no tested round has followed that<br>`row-setup` — void: not current — round 1 selected it, a fix stage is on record since, and no tested round has followed that",
                    "commit": sha('c'),
                    "scope": "round 1",
                    "status": "void",
                    "round": 1,
                    "behind": 1,
                    "retry": null,
                    "items": [
                        {"item": "the-gate", "runs": "every-candidate", "round": 1, "attempt": 1,
                         "commit": sha('c'), "outcome": "green", "reason": null,
                         "standing": "void", "why": "not-current", "rerun": null},
                        {"item": "row-setup", "runs": "in-scope", "round": 1, "attempt": 1,
                         "commit": sha('c'), "outcome": "void", "reason": "its driver | died",
                         "standing": "void", "why": "not-current", "rerun": null},
                    ],
                },
                {
                    "clause": "no-regression",
                    "instrument": "-",
                    "commit": null,
                    "scope": "-",
                    "status": "void",
                    "round": null,
                    "behind": null,
                    "retry": "no-instrument",
                    "items": [],
                },
            ],
            // Round 2 is the latest and has no scope yet: nobody can say whether it
            // reaches the review row, and the check runs on every candidate.
            "items": [
                {
                    "item": "the-gate",
                    "kind": "check",
                    "clause": "no-lost-files",
                    "runs": "every-candidate",
                    "doors": [],
                    "registries": [],
                    "brief": "the row's brief, as the opening record names it",
                    "selected": true,
                },
                {
                    "item": "row-setup",
                    "kind": "review-row",
                    "clause": "no-lost-files",
                    "runs": "in-scope",
                    "doors": ["jigc setup"],
                    "registries": [],
                    "brief": "the row's brief, as the opening record names it",
                    "selected": null,
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
                        "fork": null,
                    },
                    "reopened": false,
                    "route": "fix",
                    "why": "inside",
                    "awaits": null,
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
                    "reopened": false,
                    "route": "recorded",
                    "why": "ruled",
                    "awaits": null,
                },
            ],
            "blockers": ["audit-f3"],
            "human_list": [],
            "human_stages": [],
            // Round 1 is the latest that tested a candidate, and round 2 is begun.
            "candidate": {"round": 1, "commit": sha('c'), "current": false},
            "fix_rounds": 1,
            "untriaged": [],
            "retest": [],
            "human_clauses": [],
            "unsettled": [],
            "forbids_close": [
                {"clause": "no-lost-files", "status": "void"},
                {"clause": "no-regression", "status": "void"},
                {"finding": "audit-f3", "route": "fix"},
                {"candidate": "not-tested", "round": 2},
            ],
            // Round 2 has a report and no record of its test stage: that stage is run
            // again, as the next attempt, and nothing of round 2 can be fixed yet — so
            // the step is `test`, whatever stands open.
            "next": "test",
            "stop": null,
            "position": {
                "test": {"round": 2, "attempt": 2},
                "fix": {"refused": "not-tested", "round": 2},
            },
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

/// The state is read from eight tables, and none of them is read on a guess: a table that
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
            |text| text.replacen("| void |", "| passed |", 1),
        ),
        (
            "an outcome somebody wrote by hand",
            || results_path("1"),
            |text| text.replacen("| green | - |", "| passed | - |", 1),
        ),
        (
            "a green run with a reason beside it",
            || results_path("1"),
            |text| text.replacen("| green | - |", "| green | it went well |", 1),
        ),
        (
            "a run whose attempt a hand renumbered: the one before it is gone",
            || results_path("1"),
            |text| text.replacen("| `row-setup` | 1 |", "| `row-setup` | 2 |", 1),
        ),
        (
            "a run cut loose from its commit",
            || results_path("1"),
            |text| {
                text.replacen(
                    "| `cccccccccccccccccccccccccccccccccccccccc` |",
                    "| `ccccccc` |",
                    1,
                )
            },
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
        (
            "a round record somebody wrote by hand",
            || round_path("1"),
            |_| "# round 1\n\nTested, and all of it fine.\n".to_owned(),
        ),
        (
            "a fact nobody defined",
            || round_path("1"),
            |text| text.replacen("| `cycles` |", "| `landed` |", 1),
        ),
        (
            "an outcome somebody wrote by hand",
            || round_path("1"),
            |text| text.replacen("| `dropped` |", "| `abandoned` |", 1),
        ),
        (
            "an item nobody could have named",
            || round_path("1"),
            |text| text.replacen("| `row-setup` |", "| `Row Setup, row-setup` |", 1),
        ),
        ("a clause nobody could have named", run_path, |text| {
            text.replacen("| `no-lost-files` |", "| `No lost files` |", 1)
        }),
        ("a clause the census names twice", run_path, |text| {
            text.replacen(
                "| `no-lost-files` |",
                "| `no-lost-files, no-lost-files` |",
                1,
            )
        }),
        ("a stop mode somebody wrote by hand", run_path, |text| {
            text.replacen("| `at-the-bound` |", "| `whenever` |", 1)
        }),
        ("a bound that is no number", run_path, |text| {
            text.replacen("| `3` |", "| `three` |", 1)
        }),
        ("a fact of the run nobody defined", run_path, |text| {
            text.replacen("| `rounds` |", "| `laps` |", 1)
        }),
        (
            "a previous release somebody wrote by hand",
            run_path,
            |text| text.replacen("| `1.0.0-rc.24` |", "| `the last one` |", 1),
        ),
        ("a release's commit cut short", run_path, |text| {
            text.replacen(
                "| `eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |",
                "| `eeeeeee` |",
                1,
            )
        }),
        ("a default scope somebody wrote by hand", run_path, |text| {
            text.replacen("| `delta` |", "| `most of it` |", 1)
        }),
        (
            "a go somebody wrote by hand",
            || round_path("1"),
            |text| format!("{text}| `go` | `maybe` |\n"),
        ),
        (
            "a granted run nobody could have named",
            || round_path("1"),
            |text| format!("{text}| `granted` | `no-lost-files` |\n"),
        ),
        ("an item that runs on a whim", test_set_path, |text| {
            text.replacen("| in-scope |", "| sometimes |", 1)
        }),
        ("an item listed twice", test_set_path, |text| {
            let row = text.lines().last().expect("a row");
            format!("{text}{row}\n")
        }),
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
                rig.run(
                    &run_set(RUN),
                    &format!(r#"{{{BOUNDED}, {RELEASE}, "clauses": ["no-lost-files"]}}"#),
                )
                .must(OK, "the run's facts, and its census");
                rig.run(&item_set(RUN), &item("row-setup").to_string())
                    .must(OK, "the test set");
                rig.run(
                    &result_set(RUN, "1", &sha('c')),
                    &ran("row-setup", "green").to_string(),
                )
                .must(OK, "what the round's item did");
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
                rig.run(
                    &round_set(RUN, "1"),
                    &json!({"candidate": sha('c'), "cross-model": ["row-setup"]}).to_string(),
                )
                .must(OK, "round 1's record");
                rig.run(
                    &round_set(RUN, "1"),
                    &json!({"cycles": 1, "outcome": "dropped"}).to_string(),
                )
                .must(OK, "a fix cycle, and the human's exit from the round");
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

    // And a tested round is tested over its doors: which items it owes a run is computed
    // from them, so a candidate with no scope beside it is no state to read.
    let rig = Rig::new("state-corrupt-tested");
    rig.run(&scope_set(RUN, "1"), &scope(&[INSIDE], &[]).to_string())
        .must(OK, "round 1's scope");
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": sha('c')}).to_string(),
    )
    .must(OK, "round 1's candidate");
    rig.run(&state(RUN), "").must(OK, "the control");
    fs::remove_file(rig.root.join(scope_path("1"))).expect("remove the scope");
    rig.run(&state(RUN), "")
        .refused(CORRUPT, "a tested round whose scope is gone");
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
    /// Whether the row's fix did not hold: the finding is open again.
    reopened: bool,
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
            reopened: false,
        }
    }

    fn reopened(mut self) -> Self {
        self.reopened = true;
        self
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

/// A verified entry the verifier contested, with the advocate's case and what the
/// independent drive of the proposal found.
fn forked(mut entry: Value) -> Value {
    entry["fork"] = json!({"kind": "contested", "case": "cheap-cut-is-correct", "drive": "holds"});
    entry
}

fn disposed(disposition: &str) -> Value {
    match disposition {
        "fixed" => json!({"disposition": "fixed", "detail": "0f34d8f0"}),
        "bound" => json!({"disposition": "bound", "detail": "a planted state"}),
        other => json!({"disposition": other}),
    }
}

/// **The per-finding truth table.** Every cell the rulings settle — and they settle every
/// one: no finding is routed `unsettled`. Three cells of it were, and are marked with the
/// number the decision of 2026-10-06 gave them: a finding nobody graded or verified goes
/// back to the round's triage (cell 1), a finding found again after its fix is open again
/// and routed by its door (cell 2), and a verified break under `no-action` is still open
/// (cell 6). Cell 5 — the latest round that triaged a finding places it — is the second
/// half of [`every_finding_is_routed_by_its_grade_its_disposition_and_the_rounds_doors`].
fn findings() -> Vec<Finding> {
    const FIX_INSIDE: (&str, &str) = ("fix", "inside");
    const FIX_REGRESSION: (&str, &str) = ("fix", "regression");
    const FIX_ADMITTED: (&str, &str) = ("fix", "admitted");
    const HUMAN_OUTSIDE: (&str, &str) = ("human", "outside");
    const NOT_A_BREAK: (&str, &str) = ("recorded", "not-a-break");
    const RULED: (&str, &str) = ("recorded", "ruled");
    const FIXED: (&str, &str) = ("recorded", "fixed");
    const UNVERIFIED: (&str, &str) = ("triage", "unverified");
    const FORK: (&str, &str) = ("human", "fork");
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
        // A FORK — a verdict the verifier found to contest a settled decision, with an
        // advocate's case and an independent drive: the human's until the human rules it,
        // wherever it was found, a regression and a refuted finding included. The ruling is
        // one of the three dispositions; `no-action` is none.
        f("fork-inside", INSIDE, FORK).triaged(forked(confirmed())),
        f("fork-outside", EXCLUDED, FORK).triaged(forked(confirmed())),
        f("fork-regression", INSIDE, FORK).triaged(forked(regression())),
        f("fork-refuted", INSIDE, FORK).triaged(forked(refuted())),
        f("fork-no-action", INSIDE, FORK)
            .triaged(forked(confirmed()))
            .then(disposed("no-action")),
        f("fork-admitted", INSIDE, FIX_ADMITTED)
            .triaged(forked(confirmed()))
            .then(disposed("admitted")),
        f("fork-later", INSIDE, RULED)
            .triaged(forked(regression()))
            .then(disposed("later")),
        f("fork-bound", EXCLUDED, RULED)
            .triaged(forked(confirmed()))
            .then(disposed("bound")),
        f("fork-fixed", INSIDE, FIXED)
            .triaged(forked(confirmed()))
            .then(disposed("fixed")),
        // Cell 1 — triage that is not finished: back to the round's triage, never to a
        // fixer and never to the human's list.
        f("in-unverified", INSIDE, UNVERIFIED).triaged(graded("breaks")),
        f("in-unclear", EXCLUDED, UNVERIFIED).triaged(graded("unclear")),
        f("never-triaged", INSIDE, ("triage", "ungraded")),
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
        // Cell 6 — `no-action` is neither a fix nor a ruling: a verified break under it
        // is open.
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
        // Cell 2 — …and `fixed` is not inherited: a fix that did not hold leaves the
        // finding open again, routed by its grade and its door like any other.
        f("again-fixed", INSIDE, FIX_INSIDE)
            .said(disposed("fixed"))
            .triaged(confirmed())
            .reopened(),
        f("again-fixed-out", EXCLUDED, HUMAN_OUTSIDE)
            .said(disposed("fixed"))
            .triaged(confirmed())
            .reopened(),
        f("again-fixed-regression", EXCLUDED, FIX_REGRESSION)
            .said(disposed("fixed"))
            .triaged(regression())
            .reopened(),
        f("again-fixed-unverified", INSIDE, UNVERIFIED)
            .said(disposed("fixed"))
            .triaged(graded("breaks"))
            .reopened(),
        f("again-fixed-needs-bound", INSIDE, ("human", "needs-bound"))
            .said(disposed("fixed"))
            .triaged(graded("needs-bound"))
            .reopened(),
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

/// Bring `findings` into the run through the script's own writers: the rows, then round
/// 1's triage, then what became of a row afterwards.
fn plant(rig: &Rig, findings: &[&Finding]) {
    plant_in(rig, findings, "1");
}

/// … in the round whose triage found them.
fn plant_in(rig: &Rig, findings: &[&Finding], round: &str) {
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
        (triage_set(RUN, round), entries, "the round's triage"),
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
        truth.len() >= 41,
        "the table collapsed to {} cells",
        truth.len()
    );
    assert!(
        truth.iter().all(|finding| finding.route != "unsettled"),
        "no finding is routed `unsettled`: every cell of this table is settled"
    );

    // A fix that did not hold: the finding says so, and so does the round that found it
    // again — there even after a later fix, because it counts what the round found.
    for finding in &truth {
        let entry = &read["ledger"]
            .as_array()
            .expect("the ledger's rows")
            .iter()
            .find(|entry| entry["key"] == finding.key)
            .expect("the finding's row")["reopened"];
        assert_eq!(
            entry,
            &json!(finding.reopened),
            "whether `{}` is open again",
            finding.key
        );
    }
    let mut found_again: Vec<&str> = truth
        .iter()
        .filter(|finding| finding.reopened)
        .map(|finding| finding.key)
        .collect();
    found_again.push("again-refixed");
    assert_eq!(
        read["rounds"][0]["reopened"],
        json!(found_again),
        "the fixes round 1 found not to have held: {read}"
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
    assert_eq!(
        read["untriaged"],
        json!(
            routed_as("triage")
                .iter()
                .map(|f| json!({"key": f.key, "why": f.why}))
                .collect::<Vec<_>>()
        )
    );
    assert_eq!(read["unsettled"], json!([]));

    // A later round has doors of its own, and the latest round that triaged a finding is
    // the one that places it; a finding it did not triage stays where its round put it.
    // (Round 1 is tested and dropped first: with a finding open, a round is begun only
    // after the human's exit from the one before it.)
    for (facts, what) in [
        (json!({"candidate": sha('c')}), "round 1's candidate"),
        (
            json!({"outcome": "dropped"}),
            "the human's exit from round 1",
        ),
    ] {
        rig.run(&round_set(RUN, "1"), &facts.to_string())
            .must(OK, what);
    }
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

/// One state of a run — its opening's facts, its clause table, its rounds and the findings
/// standing in its ledger, by their keys in [`findings`] — and the step that must follow.
#[derive(Clone, Copy)]
/// What is done once every round of a [`Round`] stands, in order — each one call of the
/// script, taken only while the state asks for it.
enum Then {
    /// An item run again: one more result in the round that selected it.
    Rerun(usize, &'static str, &'static str),
    /// A fact written to a round's record: one more re-run granted by the human.
    Fact(usize, &'static str),
    /// The human's ruling on a finding: its disposition.
    Rule(&'static str, &'static str),
    /// One more record of a round's triage: an entry for a finding, less its key.
    Triage(usize, &'static str, &'static str),
    /// A report of a round's `test` stage, by its attempt: an attempt that left a report
    /// and reached no record.
    Report(usize, &'static str),
    /// A ledger patch, less its key.
    Patch(&'static str, &'static str),
    /// A fact of the run, written again: a bound, raised.
    Run(&'static str),
}

struct Round {
    name: &'static str,
    /// The run's facts as the opening wrote them — the inside of a JSON object; empty
    /// for an opening that wrote none.
    run: &'static str,
    /// The clauses of the census, each with WHAT ITS ITEMS DID, as shorthand for the
    /// results: `(clause, outcome, round)` — in that round's test stage every item that
    /// judges the clause and that the round's scope selects ran, with that outcome.
    /// Round 0 is no round: the clause's items have no result anywhere.
    clauses: &'static [(&'static str, &'static str, usize)],
    /// The clauses an item of the test set judges — an in-scope item, a hunt, whose one
    /// door is [`INSIDE`].
    instruments: &'static [&'static str],
    /// The clauses a deterministic check judges: an item that runs on every candidate.
    checks: &'static [&'static str],
    /// The clauses a hunt judges whose one door is [`UNLISTED`]: no round's scope names it.
    beyond: &'static [&'static str],
    /// Whether the opening wrote the previous release and the default scope.
    release: bool,
    /// The rounds, in order: what each one's record holds beside its candidate, as the
    /// inside of a JSON object — or `None` for a round that has a scope and no record.
    rounds: &'static [Option<&'static str>],
    /// The rounds whose scope leaves [`INSIDE`] out: their doors reach no hunt.
    away: &'static [usize],
    /// Facts written to a round's record as soon as it stands — the human's go — as
    /// `(round, the inside of a JSON object)`.
    after: &'static [(usize, &'static str)],
    findings: &'static [&'static str],
    /// What follows once every round stands: re-runs, grants, the human's rulings.
    then: &'static [Then],
    /// Findings planted after that: found by what ran last.
    later: &'static [&'static str],
    /// The findings a red result filed, and where each is routed — `(key, route)`.
    filed: &'static [(&'static str, &'static str)],
    /// Where a standing finding is routed once `then` is done, where that is not where
    /// [`findings`] routes it as planted — `(key, route)`, by hand.
    rerouted: &'static [(&'static str, &'static str)],
    /// WHAT THE DERIVATION GIVES, written by hand for each cell and never computed: the
    /// status of each clause named here. A clause not named has the status its shorthand
    /// in `clauses` says — its items' one outcome, where that is also what is derived.
    derived: &'static [(&'static str, &'static str)],
    next: &'static str,
    /// What the document says beside `next`, as the inside of a JSON object.
    says: &'static str,
}

/// The run's two clauses, both green on round 1's candidate, and one of them not.
const GREEN: &[(&str, &str, usize)] = &[("clause-a", "green", 1), ("clause-b", "green", 1)];
const ONE_VOID: &[(&str, &str, usize)] = &[("clause-a", "green", 1), ("clause-b", "void", 1)];
/// A run that stops at its bound of three fix rounds, and one that stops after every
/// round.
const BOUNDED: &str = r#""stop": "at-the-bound", "rounds": 3"#;
const EVERY_ROUND: &str = r#""stop": "every-round""#;
/// The rest of what an opening writes: the previous release, and the default scope.
const RELEASE: &str = r#""previous": "1.0.0-rc.24", "previous-commit": "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", "scope": "delta""#;
/// One round, tested; and what its record says when it also ran a fix cycle.
const TESTED_ONCE: &[Option<&str>] = &[Some("")];
const LANDED: &[Option<&str>] = &[Some(r#""cycles": 1"#)];
const DROPPED: &[Option<&str>] = &[Some(r#""cycles": 3, "outcome": "dropped""#)];
/// Round 1's fixes landed, and round 2 tested what they left.
const CONFIRMED: &[Option<&str>] = &[Some(r#""cycles": 1"#), Some("")];
/// Three rounds with a fix stage on record — the bound of [`BOUNDED`], spent — and the
/// round that tests what the third left.
const SPENT: &[Option<&str>] = &[
    Some(r#""cycles": 1"#),
    Some(r#""cycles": 2"#),
    Some(r#""cycles": 1"#),
    Some(""),
];
/// What both clauses are once a fix stage is on record in the latest tested round: void.
/// No run is current — what that fix stage left is tested by nobody.
const BOTH_VOID: &[(&str, &str)] = &[("clause-a", "void"), ("clause-b", "void")];
/// Both clauses green on round 2's candidate, and on round 4's.
const GREEN_2: &[(&str, &str, usize)] = &[("clause-a", "green", 2), ("clause-b", "green", 2)];
const GREEN_4: &[(&str, &str, usize)] = &[("clause-a", "green", 4), ("clause-b", "green", 4)];

/// The state every cell is a variation of: a ready run that stops at its bound, both
/// clauses judged by an item and green, one round tested, nothing found.
const ROUND: Round = Round {
    name: "",
    run: BOUNDED,
    clauses: GREEN,
    instruments: &["clause-a", "clause-b"],
    checks: &[],
    beyond: &[],
    release: true,
    rounds: TESTED_ONCE,
    away: &[],
    after: &[],
    findings: &[],
    then: &[],
    later: &[],
    filed: &[],
    rerouted: &[],
    derived: &[],
    next: "",
    says: "{}",
};

/// **The run's truth table.** The cells the decision of 2026-10-06 settled carry its
/// numbers; the stops are the two bounds of ruling 6.
const ROUNDS: &[Round] = &[
    Round {
        name: "every clause green, and no finding",
        next: "close",
        says: r#"{"stop": null, "retest": [], "human_clauses": [], "unsettled": [], "not_ready": []}"#,
        ..ROUND
    },
    Round {
        name: "every clause green, and nothing that breaks one",
        findings: &["in-refuted", "in-no-break", "oos-listed"],
        next: "close",
        ..ROUND
    },
    Round {
        name: "every blocker fixed, every item ruled, one found again",
        findings: &["in-fixed", "out-later", "out-bound", "again-later"],
        next: "close",
        ..ROUND
    },
    // Cell 4 — the opening writes a row per clause, and a run without one is not ready:
    // an empty table is never "nothing forbids close", and no stage starts over it. The
    // same holds of the facts the stops are computed from.
    Round {
        name: "no clause row at all",
        clauses: &[],
        next: "not-ready",
        says: r#"{"not_ready": ["no-clause-row"], "stop": null}"#,
        ..ROUND
    },
    Round {
        name: "no clause row, and an open blocker: nothing is fixed past an opening that is not done",
        clauses: &[],
        findings: &["in-confirmed"],
        next: "not-ready",
        says: r#"{"not_ready": ["no-clause-row"], "blockers": ["in-confirmed"]}"#,
        ..ROUND
    },
    Round {
        name: "no stop mode",
        run: "",
        next: "not-ready",
        says: r#"{"not_ready": ["no-stop-mode"]}"#,
        ..ROUND
    },
    Round {
        name: "a run that stops at its bound, and no bound",
        run: r#""stop": "at-the-bound""#,
        next: "not-ready",
        says: r#"{"not_ready": ["no-round-bound"]}"#,
        ..ROUND
    },
    Round {
        name: "a bound, and no stop mode",
        run: r#""rounds": 3"#,
        next: "not-ready",
        says: r#"{"not_ready": ["no-stop-mode"]}"#,
        ..ROUND
    },
    Round {
        name: "an opening that wrote nothing",
        run: "",
        release: false,
        clauses: &[],
        rounds: &[],
        next: "not-ready",
        says: r#"{"not_ready": ["no-clause-row", "no-stop-mode", "no-previous-release", "no-default-scope"]}"#,
        ..ROUND
    },
    // The opening's remaining machine-read facts are data: the release the run measures
    // against, and the scope a round is started with when its invocation names none.
    Round {
        name: "no previous release, and no default scope",
        release: false,
        next: "not-ready",
        says: r#"{"not_ready": ["no-previous-release", "no-default-scope"], "stop": null}"#,
        ..ROUND
    },
    Round {
        name: "a previous release with no commit, and a default scope",
        run: r#""stop": "at-the-bound", "rounds": 3, "previous": "1.0.0-rc.24", "scope": "everything""#,
        release: false,
        next: "not-ready",
        says: r#"{"not_ready": ["no-previous-release"]}"#,
        ..ROUND
    },
    Round {
        name: "a previous release, and no default scope",
        run: r#""stop": "every-round", "previous": "1.0.0", "previous-commit": "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee""#,
        release: false,
        findings: &["in-confirmed"],
        next: "not-ready",
        says: r#"{"not_ready": ["no-default-scope"], "stop": null}"#,
        ..ROUND
    },
    // The `test` stage comes first: before round 1, while a round's own is not recorded,
    // and after a round that is over with something still open.
    Round {
        name: "a ready run and no round yet: a row not yet run is no re-run",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        rounds: &[],
        next: "test",
        says: r#"{"retest": [], "stop": null}"#,
        ..ROUND
    },
    Round {
        name: "no round yet, and a clause no item judges: nothing is asked of the table yet",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        instruments: &["clause-a"],
        rounds: &[],
        next: "test",
        says: r#"{"unsettled": [], "retest": [], "human_clauses": []}"#,
        ..ROUND
    },
    Round {
        name: "a round begun and not recorded",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        rounds: &[None],
        next: "test",
        ..ROUND
    },
    Round {
        name: "round 1 landed, round 2 begun and not recorded: every row green closes nothing",
        rounds: &[Some(r#""cycles": 1"#), None],
        findings: &["in-fixed"],
        next: "test",
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a dropped round, its blocker open again: the next round's test, not a fixer",
        rounds: DROPPED,
        findings: &["in-confirmed"],
        next: "test",
        says: r#"{"blockers": ["in-confirmed"], "stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a part landed, an item left for the human",
        rounds: &[Some(r#""cycles": 2, "outcome": "part""#)],
        findings: &["out-confirmed"],
        next: "test",
        derived: BOTH_VOID,
        ..ROUND
    },
    // Cell 1 — a finding with no grade, or a break nobody verified: the round's triage
    // is not finished. It forbids close, and it comes before the human's list and before
    // any fixer.
    Round {
        name: "a finding nobody verified",
        findings: &["in-unverified"],
        next: "triage",
        says: r#"{"untriaged": [{"key": "in-unverified", "why": "unverified"}], "blockers": [], "human_list": []}"#,
        ..ROUND
    },
    Round {
        name: "a finding nobody graded",
        findings: &["never-triaged"],
        next: "triage",
        says: r#"{"untriaged": [{"key": "never-triaged", "why": "ungraded"}], "blockers": [], "human_list": []}"#,
        ..ROUND
    },
    Round {
        name: "a finding nobody verified beside an open blocker",
        findings: &["in-unverified", "in-confirmed"],
        next: "triage",
        ..ROUND
    },
    Round {
        name: "a finding nobody verified beside an unruled item: the list is ruled when it is whole",
        findings: &["in-unverified", "out-confirmed"],
        next: "triage",
        ..ROUND
    },
    Round {
        name: "found again after its fix, and nobody verified it",
        findings: &["again-fixed-unverified"],
        next: "triage",
        ..ROUND
    },
    // The human's list.
    Round {
        name: "an unruled item on the human's list",
        findings: &["out-confirmed"],
        next: "rule",
        ..ROUND
    },
    Round {
        name: "a finding that needs a bound nobody declared",
        findings: &["needs-bound"],
        next: "rule",
        ..ROUND
    },
    Round {
        name: "an unruled item beside an open blocker: the ruling comes first",
        findings: &["in-confirmed", "out-confirmed"],
        next: "rule",
        ..ROUND
    },
    Round {
        name: "an unruled item under a void clause",
        clauses: ONE_VOID,
        findings: &["out-confirmed"],
        next: "rule",
        says: r#"{"retest": [], "human_clauses": []}"#,
        ..ROUND
    },
    // Fixed without asking.
    Round {
        name: "an open blocker inside the test set",
        findings: &["in-confirmed"],
        next: "fix",
        ..ROUND
    },
    Round {
        name: "a regression found outside the test set",
        findings: &["out-regression"],
        next: "fix",
        ..ROUND
    },
    Round {
        name: "an item the human admitted to the run",
        findings: &["out-admitted"],
        next: "fix",
        ..ROUND
    },
    Round {
        name: "an open blocker under a void clause: the clause's re-run waits",
        clauses: ONE_VOID,
        findings: &["in-confirmed", "in-fixed"],
        next: "fix",
        says: r#"{"retest": []}"#,
        ..ROUND
    },
    // Cell 2 — a finding found again after its fix is open again, and routed by its door.
    Round {
        name: "a finding found again after its fix, inside the test set",
        findings: &["again-fixed", "in-fixed"],
        next: "fix",
        says: r#"{"blockers": ["again-fixed"]}"#,
        ..ROUND
    },
    Round {
        name: "a finding found again after its fix, outside the test set",
        findings: &["again-fixed-out"],
        next: "rule",
        says: r#"{"human_list": [{"key": "again-fixed-out", "why": "outside"}]}"#,
        ..ROUND
    },
    // Cell 3 — nothing open, and a clause without a green row: its instrument is run
    // again once, alone; still not green after that, the clause is the human's.
    Round {
        name: "a void clause row",
        clauses: ONE_VOID,
        next: "retest",
        says: r#"{"retest": ["clause-b"], "human_clauses": [], "unsettled": []}"#,
        ..ROUND
    },
    Round {
        name: "a void clause row, every finding fixed",
        clauses: &[("clause-a", "void", 1), ("clause-b", "green", 1)],
        findings: &["in-fixed"],
        next: "retest",
        says: r#"{"retest": ["clause-a"]}"#,
        ..ROUND
    },
    Round {
        name: "a red check is a finding: nobody's yet, so the round's triage is not finished",
        clauses: &[("clause-a", "green", 1), ("clause-b", "red", 1)],
        next: "triage",
        says: r#"{"untriaged": [{"key": "red-check-clause-b", "why": "ungraded"}], "retest": [], "blockers": []}"#,
        instruments: &["clause-a"],
        checks: &["clause-b"],
        filed: &[("red-check-clause-b", "triage")],
        ..ROUND
    },
    Round {
        name: "a red check whose finding the human ruled for later: nothing is open, and the check is run again once",
        instruments: &["clause-a"],
        checks: &["clause-b"],
        clauses: &[("clause-a", "green", 1), ("clause-b", "red", 1)],
        then: &[Then::Rule("red-check-clause-b", "later")],
        filed: &[("red-check-clause-b", "recorded")],
        next: "retest",
        says: r#"{"retest": ["clause-b"], "untriaged": []}"#,
        ..ROUND
    },
    Round {
        name: "a red check, red again on its one re-run, its finding ruled: the clause is the human's",
        instruments: &["clause-a"],
        checks: &["clause-b"],
        clauses: &[("clause-a", "green", 1), ("clause-b", "red", 1)],
        then: &[
            Then::Rule("red-check-clause-b", "later"),
            Then::Rerun(1, "check-clause-b", "red"),
        ],
        filed: &[("red-check-clause-b", "recorded")],
        next: "rule",
        says: r#"{"human_clauses": [{"clause": "clause-b", "why": "not-green-after-its-rerun"}], "retest": []}"#,
        ..ROUND
    },
    Round {
        name: "an item round 1 selected and holds no run of: it never ran to its end",
        clauses: &[("clause-a", "green", 1), ("clause-b", "void", 0)],
        next: "retest",
        says: r#"{"retest": ["clause-b"]}"#,
        ..ROUND
    },
    Round {
        name: "two clauses without a green row: both are named",
        clauses: &[("clause-a", "void", 1), ("clause-b", "void", 1)],
        next: "retest",
        says: r#"{"retest": ["clause-a", "clause-b"]}"#,
        ..ROUND
    },
    Round {
        name: "the one re-run turned the row green",
        clauses: ONE_VOID,
        next: "close",
        then: &[Then::Rerun(1, "row-clause-b", "green")],
        derived: &[("clause-b", "green")],
        ..ROUND
    },
    Round {
        name: "still void after its one re-run: the human's",
        clauses: ONE_VOID,
        next: "rule",
        says: r#"{"human_clauses": [{"clause": "clause-b", "why": "not-green-after-its-rerun"}], "retest": [], "human_list": []}"#,
        then: &[Then::Rerun(1, "row-clause-b", "void")],
        ..ROUND
    },
    Round {
        name: "the re-run was another clause's item's: an attempt is counted per item",
        clauses: &[("clause-a", "void", 1), ("clause-b", "void", 1)],
        next: "retest",
        says: r#"{"retest": ["clause-a"], "human_clauses": []}"#,
        then: &[Then::Rerun(1, "row-clause-b", "green")],
        derived: &[("clause-b", "green")],
        ..ROUND
    },
    Round {
        name: "one clause's retry spent, another's due: the human's comes first",
        clauses: &[("clause-a", "void", 1), ("clause-b", "void", 1)],
        next: "rule",
        says: r#"{"human_clauses": [{"clause": "clause-b", "why": "not-green-after-its-rerun"}], "retest": []}"#,
        then: &[Then::Rerun(1, "row-clause-b", "void")],
        ..ROUND
    },
    Round {
        name: "a spent retry beside an open blocker: the clause is the human's once nothing is open",
        clauses: ONE_VOID,
        next: "fix",
        says: r#"{"human_clauses": [], "retest": [], "unsettled": []}"#,
        then: &[Then::Rerun(1, "row-clause-b", "void")],
        later: &["in-confirmed"],
        ..ROUND
    },
    // What the rulings still do not settle: a clause without a green row that no item of
    // the test set judges. There is no instrument to run again.
    Round {
        name: "a void clause that no item judges",
        clauses: ONE_VOID,
        instruments: &["clause-a"],
        next: "unsettled",
        says: r#"{"unsettled": [{"cell": "not-green-and-no-instrument", "clauses": ["clause-b"]}], "retest": []}"#,
        ..ROUND
    },
    // And the second cell: items judge the clause, and no tested round's scope has selected
    // one. An item no scope selected owes nothing — and a clause with no green run on
    // record does not close: the rulings name no step, so it is the human's.
    Round {
        name: "a clause whose items no scope has selected: it owes nothing and has no evidence",
        clauses: &[("clause-a", "green", 1), ("clause-b", "void", 0)],
        instruments: &["clause-a"],
        beyond: &["clause-b"],
        next: "unsettled",
        says: r#"{"unsettled": [{"cell": "judged-and-never-selected", "clauses": ["clause-b"]}], "retest": [], "human_clauses": []}"#,
        ..ROUND
    },
    Round {
        name: "a clause never selected beside an open blocker: the blocker is fixed first",
        clauses: &[("clause-a", "green", 1), ("clause-b", "void", 0)],
        instruments: &["clause-a"],
        beyond: &["clause-b"],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"unsettled": []}"#,
        ..ROUND
    },
    Round {
        name: "a clause no item judges beside one whose retry is due: nothing is run on a guess",
        clauses: &[("clause-a", "void", 1), ("clause-b", "void", 0)],
        instruments: &["clause-a"],
        next: "unsettled",
        says: r#"{"unsettled": [{"cell": "not-green-and-no-instrument", "clauses": ["clause-b"]}], "retest": []}"#,
        ..ROUND
    },
    // The stops (ruling 6). A run that stops after every round stops once its latest
    // round is over, whatever follows — and says what follows.
    Round {
        name: "every round a stop: a round that left nothing open, every clause green",
        run: EVERY_ROUND,
        next: "stop",
        says: r#"{"stop": {"why": "every-round", "round": 1, "then": "close"}}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop: a round that landed is followed by its test",
        run: EVERY_ROUND,
        rounds: LANDED,
        findings: &["in-fixed"],
        next: "stop",
        says: r#"{"stop": {"why": "every-round", "round": 1, "then": "test"}}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "every round a stop: the re-run of a void clause waits for the human's go",
        run: EVERY_ROUND,
        clauses: ONE_VOID,
        next: "stop",
        says: r#"{"stop": {"why": "every-round", "round": 1, "then": "retest"}, "retest": ["clause-b"]}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop: a dropped round",
        run: EVERY_ROUND,
        rounds: DROPPED,
        findings: &["in-confirmed"],
        next: "stop",
        says: r#"{"stop": {"why": "every-round", "round": 1, "then": "test"}}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "every round a stop, and the round is not over: a blocker is fixed first",
        run: EVERY_ROUND,
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop, the go given and the next round begun: no stop inside a round",
        run: EVERY_ROUND,
        rounds: &[Some(""), None],
        after: &[(1, r#""go": true"#)],
        next: "test",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop, and a bound beside it: the bound is the other mode's",
        run: r#""stop": "every-round", "rounds": 1"#,
        clauses: ONE_VOID,
        next: "stop",
        says: r#"{"stop": {"why": "every-round", "round": 1, "then": "retest"}}"#,
        ..ROUND
    },
    // The human's go after a stop is a recorded fact: `next` answers the step the stop
    // named — and the round that follows stops again.
    Round {
        name: "every round a stop, and the human's go on record: the run closes",
        run: EVERY_ROUND,
        after: &[(1, r#""go": true"#)],
        next: "close",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop, and the go on record: a void clause's re-run is the step",
        run: EVERY_ROUND,
        clauses: ONE_VOID,
        after: &[(1, r#""go": true"#)],
        next: "retest",
        says: r#"{"stop": null, "retest": ["clause-b"]}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop, and the go on record after a dropped round: its test",
        run: EVERY_ROUND,
        rounds: DROPPED,
        findings: &["in-confirmed"],
        after: &[(1, r#""go": true"#)],
        next: "test",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "the go was round 1's: round 2 is over, and the run stops again",
        run: EVERY_ROUND,
        clauses: GREEN_2,
        rounds: &[Some(""), Some("")],
        after: &[(1, r#""go": true"#)],
        next: "stop",
        says: r#"{"stop": {"why": "every-round", "round": 2, "then": "close"}}"#,
        ..ROUND
    },
    // Close is reachable only from a tested candidate: once a fix stage is on record in
    // the latest round, the next step is that candidate's `test` round, whatever the
    // clause table says — and never `close`.
    Round {
        name: "round 1's fixes landed, every clause row green, nothing open: the candidate is not tested",
        rounds: LANDED,
        findings: &["in-fixed"],
        next: "test",
        says: r#"{"stop": null, "retest": [], "human_clauses": [], "unsettled": [], "candidate": {"round": 1, "commit": "1111111111111111111111111111111111111111", "current": false}, "fix_rounds": 1}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a landed round and a ledger with no row at all",
        rounds: LANDED,
        next: "test",
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a landed round under a void clause: the round's test, not the clause's re-run",
        rounds: LANDED,
        clauses: ONE_VOID,
        next: "test",
        says: r#"{"retest": []}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a landed round and a clause no item judges: nothing is asked of the table yet",
        rounds: LANDED,
        clauses: ONE_VOID,
        instruments: &["clause-a"],
        next: "test",
        says: r#"{"unsettled": []}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a part landed, and nothing is left open",
        rounds: &[Some(r#""cycles": 2, "outcome": "part""#)],
        findings: &["in-fixed"],
        next: "test",
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "a dropped round, everything ruled since: the latest stage on record is a fix stage",
        rounds: DROPPED,
        findings: &["out-later"],
        next: "test",
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "the round after a landing, tested, every row from it: the run closes",
        rounds: CONFIRMED,
        clauses: GREEN_2,
        findings: &["in-fixed"],
        next: "close",
        says: r#"{"candidate": {"round": 2, "commit": "2222222222222222222222222222222222222222", "current": true}, "fix_rounds": 1}"#,
        ..ROUND
    },
    // Hunt once per change, check always. A deterministic check's green is evidence about
    // the candidate it ran on; a hunt's green from before a landing counts when the doors
    // of the fixes do not reach it.
    Round {
        name: "a deterministic check green on an earlier candidate does not count on this one",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 1), ("clause-b", "green", 2)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        next: "retest",
        says: r#"{"retest": ["clause-a"]}"#,
        derived: &[("clause-a", "void")],
        ..ROUND
    },
    Round {
        name: "a check and a hunt judge the clause, and the check has no run on this candidate: the hunt's green does not stand in for it",
        rounds: CONFIRMED,
        clauses: GREEN,
        checks: &["clause-a"],
        away: &[2],
        next: "retest",
        says: r#"{"retest": ["clause-a"]}"#,
        derived: &[("clause-a", "void")],
        ..ROUND
    },
    Round {
        name: "a hunt's green from before a landing counts: the fixes' doors do not reach it",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 2), ("clause-b", "green", 1)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        away: &[2],
        next: "close",
        ..ROUND
    },
    Round {
        name: "a hunt's green from before a landing, and the fixes' doors reach it",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 2), ("clause-b", "green", 1)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        next: "retest",
        says: r#"{"retest": ["clause-b"]}"#,
        derived: &[("clause-b", "void")],
        ..ROUND
    },
    // An item round 1's scope selected and round 2's does not, under each of its outcomes:
    // what it owes is round 1's, and a later round that ran the items beside it settles
    // nothing of it. (Green is the row above the last: the run closes.)
    Round {
        name: "a hunt void in round 1 that round 2 does not select: its void stands, and nothing closes over it",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 2), ("clause-b", "void", 1)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        away: &[2],
        next: "retest",
        says: r#"{"retest": ["clause-b"], "position": {"test": {"round": 1, "attempt": 1, "rerun": {"clause": "clause-b", "round": 1, "items": [{"item": "row-clause-b", "attempt": 2}], "doors": [{"door": "jigc setup", "registry": "the verb table", "derivation": "a changed symbol is read by `jigc setup`"}]}}, "fix": {"round": 2, "cycle": 1, "attempt": 1, "land": false}}}"#,
        ..ROUND
    },
    Round {
        name: "a hunt round 1 selected and never ran, that round 2 does not select: it is owed still",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 2), ("clause-b", "void", 0)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        away: &[2],
        next: "retest",
        says: r#"{"retest": ["clause-b"]}"#,
        ..ROUND
    },
    Round {
        name: "an item red in round 1, which was dropped, that round 2 does not select: its finding is still nobody's",
        rounds: &[Some(r#""cycles": 1, "outcome": "dropped""#), Some("")],
        clauses: &[("clause-a", "green", 2), ("clause-b", "red", 1)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        away: &[2],
        filed: &[("red-row-clause-b", "triage")],
        next: "triage",
        ..ROUND
    },
    Round {
        name: "two landings, both resolved, neither reaching the hunt: its green counts",
        rounds: &[Some(r#""cycles": 1"#), Some(r#""cycles": 1"#), Some("")],
        clauses: &[("clause-a", "green", 3), ("clause-b", "green", 1)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        away: &[2, 3],
        next: "close",
        ..ROUND
    },
    Round {
        name: "two landings, and the first one's doors reached the hunt",
        rounds: &[Some(r#""cycles": 1"#), Some(r#""cycles": 1"#), Some("")],
        clauses: &[("clause-a", "green", 3), ("clause-b", "green", 1)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        away: &[3],
        next: "retest",
        derived: &[("clause-b", "void")],
        ..ROUND
    },
    Round {
        name: "a hunt's green from before a round that fixed nothing stands while that round's scope does not select it",
        rounds: &[Some(""), Some("")],
        clauses: &[("clause-a", "green", 2), ("clause-b", "green", 1)],
        checks: &["clause-a"],
        next: "close",
        instruments: &["clause-b"],
        away: &[2],
        ..ROUND
    },
    Round {
        name: "a check with no run on this candidate beside a void hunt: both are named",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 1), ("clause-b", "void", 2)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        next: "retest",
        says: r#"{"retest": ["clause-a", "clause-b"]}"#,
        derived: &[("clause-a", "void")],
        ..ROUND
    },
    Round {
        name: "a check with no run on this candidate beside an open blocker: the blocker is fixed first",
        rounds: CONFIRMED,
        clauses: &[("clause-a", "green", 1), ("clause-b", "green", 2)],
        instruments: &["clause-b"],
        checks: &["clause-a"],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"retest": []}"#,
        derived: &[("clause-a", "void")],
        ..ROUND
    },
    // What the rulings still do not settle, the second cell: a green row from before a
    // fix round that no item judges. Nothing can say whether it still holds.
    Round {
        name: "both cells at once: a clause no item judges, and one whose items no scope has selected",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        instruments: &[],
        next: "unsettled",
        says: r#"{"unsettled": [{"cell": "not-green-and-no-instrument", "clauses": ["clause-a"]}, {"cell": "judged-and-never-selected", "clauses": ["clause-b"]}], "retest": [], "human_clauses": []}"#,
        beyond: &["clause-b"],
        ..ROUND
    },
    // A clause still not green after its one re-run is the human's, and the one ruling
    // with a recorded form is one more re-run of it.
    Round {
        name: "still void after its re-run, and the human grants one more",
        clauses: ONE_VOID,
        next: "retest",
        says: r#"{"retest": ["clause-b"], "human_clauses": []}"#,
        then: &[
            Then::Rerun(1, "row-clause-b", "void"),
            Then::Fact(1, r#""granted": "clause-b""#),
        ],
        ..ROUND
    },
    Round {
        name: "the granted re-run left it void again: the human's again",
        clauses: ONE_VOID,
        next: "rule",
        says: r#"{"human_clauses": [{"clause": "clause-b", "why": "not-green-after-its-rerun"}], "retest": []}"#,
        then: &[
            Then::Rerun(1, "row-clause-b", "void"),
            Then::Fact(1, r#""granted": "clause-b""#),
            Then::Rerun(1, "row-clause-b", "void"),
        ],
        ..ROUND
    },
    Round {
        name: "the granted re-run turned the row green",
        clauses: ONE_VOID,
        next: "close",
        then: &[
            Then::Rerun(1, "row-clause-b", "void"),
            Then::Fact(1, r#""granted": "clause-b""#),
            Then::Rerun(1, "row-clause-b", "green"),
        ],
        derived: &[("clause-b", "green")],
        ..ROUND
    },
    Round {
        name: "granted, spent again, and granted again: each grant is one more attempt",
        clauses: ONE_VOID,
        then: &[
            Then::Rerun(1, "row-clause-b", "void"),
            Then::Fact(1, r#""granted": "clause-b""#),
            Then::Rerun(1, "row-clause-b", "void"),
            Then::Fact(1, r#""granted": "clause-b""#),
        ],
        next: "retest",
        says: r#"{"retest": ["clause-b"], "human_clauses": []}"#,
        ..ROUND
    },
    // The bound across rounds (ruling 6) counts FIX ROUNDS, never test stages: a run that
    // stops at its bound stops when one more fix round would start and the bound is spent.
    Round {
        name: "the bound spent, and the confirming test leaves a blocker",
        rounds: SPENT,
        clauses: GREEN_4,
        findings: &["in-confirmed"],
        next: "stop",
        says: r#"{"stop": {"why": "round-bound", "round": 4, "then": "fix"}, "blockers": ["in-confirmed"], "fix_rounds": 3}"#,
        ..ROUND
    },
    Round {
        name: "the bound spent, and the confirming test leaves nothing: the run closes",
        rounds: SPENT,
        clauses: GREEN_4,
        findings: &["in-fixed"],
        next: "close",
        says: r#"{"stop": null, "fix_rounds": 3}"#,
        ..ROUND
    },
    Round {
        name: "the bound spent, the last allowed fix round landed: its test is always allowed",
        rounds: &[
            Some(r#""cycles": 1"#),
            Some(r#""cycles": 2"#),
            Some(r#""cycles": 1"#),
        ],
        findings: &["in-fixed"],
        next: "test",
        says: r#"{"stop": null, "fix_rounds": 3}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "the bound spent, in the round that spent it: its own blocker is still fixed",
        rounds: &[
            Some(r#""cycles": 1"#),
            Some(r#""cycles": 2"#),
            Some(r#""cycles": 1"#),
        ],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "the bound spent, an item for the human: the ruling is no fix round",
        rounds: SPENT,
        clauses: GREEN_4,
        findings: &["out-confirmed", "in-confirmed"],
        next: "rule",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    Round {
        name: "the bound spent, a finding nobody verified: the triage is finished first",
        rounds: SPENT,
        clauses: GREEN_4,
        findings: &["in-unverified", "in-confirmed"],
        next: "triage",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    Round {
        name: "the bound spent, a clause without a green row: its re-run counts against no bound",
        rounds: SPENT,
        clauses: &[("clause-a", "green", 4), ("clause-b", "void", 4)],
        next: "retest",
        says: r#"{"stop": null, "retest": ["clause-b"]}"#,
        ..ROUND
    },
    Round {
        name: "one fix round under the bound: the blocker is fixed",
        rounds: &[Some(r#""cycles": 1"#), Some(r#""cycles": 1"#), Some("")],
        clauses: &[("clause-a", "green", 3), ("clause-b", "green", 3)],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null, "fix_rounds": 2}"#,
        ..ROUND
    },
    Round {
        name: "three rounds and no fix stage in any: the bound counts no test stage",
        rounds: &[Some(""), Some(""), Some(""), Some("")],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null, "fix_rounds": 0}"#,
        clauses: GREEN_4,
        ..ROUND
    },
    Round {
        name: "a dropped round is a fix round: a fix stage ran in it",
        rounds: &[
            Some(r#""cycles": 1"#),
            Some(r#""outcome": "dropped""#),
            Some(r#""cycles": 1"#),
            Some(""),
        ],
        clauses: GREEN_4,
        findings: &["in-confirmed"],
        next: "stop",
        says: r#"{"stop": {"why": "round-bound", "round": 4, "then": "fix"}, "fix_rounds": 3}"#,
        ..ROUND
    },
    Round {
        name: "the bound spent, a dropped round with its blocker open again: its test, then the stop",
        rounds: &[
            Some(r#""cycles": 1"#),
            Some(r#""cycles": 1"#),
            Some(r#""cycles": 3, "outcome": "dropped""#),
        ],
        findings: &["in-confirmed"],
        next: "test",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "the bound raised by the human: the step is taken",
        run: r#""stop": "at-the-bound", "rounds": 4"#,
        rounds: SPENT,
        clauses: GREEN_4,
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    Round {
        name: "every round a stop, three fix rounds and a bound of one: the bound is the other mode's",
        run: r#""stop": "every-round", "rounds": 1"#,
        rounds: SPENT,
        after: &[
            (1, r#""go": true"#),
            (2, r#""go": true"#),
            (3, r#""go": true"#),
        ],
        clauses: GREEN_4,
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null}"#,
        ..ROUND
    },
    // `triage` CANNOT SPIN (the review's `F3`, the repair plan's state 3). A finding the
    // round's triage leaves without a grade or a verdict is tried once more without the
    // human — the passes are counted in the round's record, by the call that records a
    // triage — and after that it is the human's: to rule, or to grant one more.
    Round {
        name: "a finding left unverified by a second record of the round's triage: it is the human's",
        findings: &["in-unverified"],
        then: &[Then::Triage(1, "in-unverified", r#""grade": "breaks""#)],
        rerouted: &[("in-unverified", "human")],
        next: "rule",
        says: r#"{"human_list": [{"key": "in-unverified", "why": "unverified-after-retry"}], "untriaged": [], "blockers": [], "human_stages": []}"#,
        ..ROUND
    },
    Round {
        name: "left unverified twice, and the human grants one more triage: the round's triage is finished once more",
        findings: &["in-unverified"],
        then: &[
            Then::Triage(1, "in-unverified", r#""grade": "breaks""#),
            Then::Fact(1, r#""reverify": "in-unverified""#),
        ],
        next: "triage",
        says: r#"{"untriaged": [{"key": "in-unverified", "why": "unverified"}], "human_list": []}"#,
        ..ROUND
    },
    Round {
        name: "granted, and left unverified a third time: one grant is one more triage, and it is the human's again",
        findings: &["in-unverified"],
        then: &[
            Then::Triage(1, "in-unverified", r#""grade": "breaks""#),
            Then::Fact(1, r#""reverify": "in-unverified""#),
            Then::Triage(1, "in-unverified", r#""grade": "unclear""#),
        ],
        rerouted: &[("in-unverified", "human")],
        next: "rule",
        says: r#"{"human_list": [{"key": "in-unverified", "why": "unverified-after-retry"}]}"#,
        ..ROUND
    },
    Round {
        name: "left unverified once, and confirmed by the one triage that is automatic: a fixer's, and nobody is asked",
        findings: &["in-unverified"],
        then: &[Then::Triage(
            1,
            "in-unverified",
            r#""grade": "breaks", "verdict": "confirmed", "regression": false"#,
        )],
        rerouted: &[("in-unverified", "fix")],
        next: "fix",
        says: r#"{"blockers": ["in-unverified"], "human_list": [], "untriaged": []}"#,
        ..ROUND
    },
    Round {
        name: "left unverified twice, and ruled for a later release: one of the human's three, and it is recorded",
        findings: &["in-unverified"],
        then: &[
            Then::Triage(1, "in-unverified", r#""grade": "breaks""#),
            Then::Rule("in-unverified", "later"),
        ],
        rerouted: &[("in-unverified", "recorded")],
        next: "close",
        says: r#"{"human_list": [], "untriaged": []}"#,
        ..ROUND
    },
    Round {
        name: "left unverified twice, and admitted by the human: a fixer's",
        findings: &["in-unverified"],
        then: &[
            Then::Triage(1, "in-unverified", r#""grade": "breaks""#),
            Then::Rule("in-unverified", "admitted"),
        ],
        rerouted: &[("in-unverified", "fix")],
        next: "fix",
        ..ROUND
    },
    Round {
        name: "a finding nobody graded, left so by two records of the round's triage: it is the human's",
        findings: &["never-triaged", "in-refuted"],
        then: &[Then::Triage(
            1,
            "in-refuted",
            r#""grade": "breaks", "verdict": "refuted""#,
        )],
        rerouted: &[("never-triaged", "human")],
        next: "rule",
        says: r#"{"human_list": [{"key": "never-triaged", "why": "ungraded-after-retry"}], "untriaged": []}"#,
        ..ROUND
    },
    Round {
        name: "left ungraded once, then graded and left unverified: that is its first pass without a verdict",
        findings: &["never-triaged", "in-refuted"],
        then: &[Then::Triage(1, "never-triaged", r#""grade": "unclear""#)],
        next: "triage",
        says: r#"{"untriaged": [{"key": "never-triaged", "why": "unverified"}], "human_list": []}"#,
        ..ROUND
    },
    // THE ATTEMPTS OF A STAGE: one that leaves reports and no record is made once more
    // without the human, and then the next one is the human's to grant.
    Round {
        name: "a round's test stage left reports and no record, once: it is run again",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        rounds: &[None],
        then: &[Then::Report(1, "1")],
        next: "test",
        says: r#"{"human_stages": [], "position": {"test": {"round": 1, "attempt": 2}, "fix": {"refused": "not-tested", "round": 1}}}"#,
        ..ROUND
    },
    Round {
        name: "a round's test stage left reports and no record, twice: the next attempt is the human's",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        rounds: &[None],
        then: &[Then::Report(1, "1"), Then::Report(1, "2")],
        next: "rule",
        says: r#"{"human_stages": [{"stage": "test", "round": 1, "cycle": null, "at": "test", "attempt": 3, "attempts": 2, "why": "not-recorded-after-retry"}], "human_list": [], "human_clauses": [], "position": {"test": {"refused": "attempts-spent", "round": 1}, "fix": {"refused": "not-tested", "round": 1}}}"#,
        ..ROUND
    },
    Round {
        name: "twice without a record, and the human grants one more attempt: the stage is run again",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        rounds: &[None],
        then: &[
            Then::Report(1, "1"),
            Then::Report(1, "2"),
            Then::Fact(1, r#""again": "test""#),
        ],
        next: "test",
        says: r#"{"human_stages": [], "position": {"test": {"round": 1, "attempt": 3}, "fix": {"refused": "not-tested", "round": 1}}}"#,
        ..ROUND
    },
    Round {
        name: "the granted attempt left a report and no record too: one grant is one attempt",
        clauses: &[("clause-a", "void", 0), ("clause-b", "void", 0)],
        rounds: &[None],
        then: &[
            Then::Report(1, "1"),
            Then::Report(1, "2"),
            Then::Fact(1, r#""again": "test""#),
            Then::Report(1, "3"),
        ],
        next: "rule",
        says: r#"{"human_stages": [{"stage": "test", "round": 1, "cycle": null, "at": "test", "attempt": 4, "attempts": 3, "why": "not-recorded-after-retry"}]}"#,
        ..ROUND
    },
    Round {
        name: "the stage that finishes a triage left reports and no record twice: its next attempt is the human's",
        findings: &["in-unverified"],
        then: &[Then::Report(1, "1"), Then::Report(1, "2")],
        next: "rule",
        says: r#"{"human_stages": [{"stage": "test", "round": 1, "cycle": null, "at": "test", "attempt": 3, "attempts": 2, "why": "not-recorded-after-retry"}], "untriaged": [{"key": "in-unverified", "why": "unverified"}], "human_list": []}"#,
        ..ROUND
    },
    Round {
        name: "an item's re-run left reports and no record twice: the next attempt of the stage is the human's",
        clauses: ONE_VOID,
        then: &[Then::Report(1, "1"), Then::Report(1, "2")],
        next: "rule",
        says: r#"{"retest": [], "human_clauses": [], "human_stages": [{"stage": "test", "round": 1, "cycle": null, "at": "test", "attempt": 3, "attempts": 2, "why": "not-recorded-after-retry"}]}"#,
        ..ROUND
    },
    // THE BOUND INSIDE A ROUND (ruling 6) is a fact, and the script answers at it: at most
    // three fix -> audit cycles, until the human raises it.
    Round {
        name: "two fix cycles in a round and a blocker left: under the bound, a fixer's",
        rounds: &[Some(r#""cycles": 2"#)],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "three fix cycles in a round and a blocker left: the bound on a round's cycles is spent",
        rounds: &[Some(r#""cycles": 3"#)],
        findings: &["in-confirmed"],
        next: "stop",
        says: r#"{"stop": {"why": "cycle-bound", "round": 1, "then": "fix"}, "position": {"test": {"refused": "round-open", "round": 1}, "fix": {"refused": "cycle-bound", "round": 1}}}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "three fix cycles and nothing left open: no bound stops a round that lands",
        rounds: &[Some(r#""cycles": 3"#)],
        findings: &["in-fixed"],
        next: "test",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "the cycle bound written as four at the opening: the third cycle's blocker is a fixer's",
        run: r#""stop": "at-the-bound", "rounds": 3, "cycles": 4"#,
        rounds: &[Some(r#""cycles": 3"#)],
        findings: &["in-confirmed"],
        next: "fix",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "at the cycle bound, and the human raises it: one more cycle",
        rounds: &[Some(r#""cycles": 3"#)],
        findings: &["in-confirmed"],
        then: &[Then::Run(r#""cycles": 4"#)],
        next: "fix",
        says: r#"{"stop": null, "position": {"test": {"refused": "round-open", "round": 1}, "fix": {"round": 1, "cycle": 4, "attempt": 1, "land": false}}}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "at the cycle bound, and the round is dropped: the next round's test",
        rounds: &[Some(r#""cycles": 3"#)],
        findings: &["in-confirmed"],
        then: &[Then::Fact(1, r#""outcome": "dropped""#)],
        next: "test",
        says: r#"{"stop": null}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    Round {
        name: "every round a stop, three cycles and a blocker: the cycle bound stops inside a round",
        run: EVERY_ROUND,
        rounds: &[Some(r#""cycles": 3"#)],
        findings: &["in-confirmed"],
        next: "stop",
        says: r#"{"stop": {"why": "cycle-bound", "round": 1, "then": "fix"}}"#,
        derived: BOTH_VOID,
        ..ROUND
    },
    // The review's `F12`: a fix is a commit, at whatever length its sha is written.
    Round {
        name: "a fix that did not hold, named again by the same commit cut shorter: the finding is still open",
        findings: &["again-fixed"],
        then: &[Then::Patch(
            "again-fixed",
            r#""disposition": "fixed", "detail": "0f34d8f""#,
        )],
        next: "fix",
        says: r#"{"blockers": ["again-fixed"]}"#,
        ..ROUND
    },
    Round {
        name: "a fix that did not hold, named again by the same commit written in full: the finding is still open",
        findings: &["again-fixed"],
        then: &[Then::Patch(
            "again-fixed",
            r#""disposition": "fixed", "detail": "0f34d8f0aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa""#,
        )],
        next: "fix",
        says: r#"{"blockers": ["again-fixed"]}"#,
        ..ROUND
    },
    Round {
        name: "a fix that did not hold, and another commit on its row: that one records it",
        findings: &["again-fixed"],
        then: &[Then::Patch(
            "again-fixed",
            r#""disposition": "fixed", "detail": "0f34d8e""#,
        )],
        rerouted: &[("again-fixed", "recorded")],
        next: "close",
        ..ROUND
    },
];

/// The candidate round `n` of a [`Round`] tested: its digit, forty times.
fn candidate(n: usize) -> String {
    let digit = u32::try_from(n).expect("a round's number");
    sha(char::from_digit(digit, 10).expect("a round below ten"))
}

/// Stand `round` up in a rig of its own and hold the state document to it.
fn drive_round(label: &str, round: &Round, truth: &[Finding]) {
    let rig = Rig::new(label);
    let what = round.name;
    if !round.run.is_empty() {
        rig.run(&run_set(RUN), &format!("{{{}}}", round.run))
            .must(OK, "the run's facts");
    }
    if round.release {
        rig.run(&run_set(RUN), &format!("{{{RELEASE}}}"))
            .must(OK, "the previous release and the default scope");
    }
    let census: Vec<&str> = round.clauses.iter().map(|(clause, _, _)| *clause).collect();
    if !census.is_empty() {
        rig.run(&run_set(RUN), &json!({"clauses": census}).to_string())
            .must(OK, "the census");
    }
    let hunt = |clause: &&str, door: &str| {
        let mut judged = item(&format!("row-{clause}"));
        judged["clause"] = json!(clause);
        judged["doors"] = json!([door]);
        judged
    };
    let hunts = round.instruments.iter().map(|clause| hunt(clause, INSIDE));
    let beyond = round.beyond.iter().map(|clause| hunt(clause, UNLISTED));
    let checks = round.checks.iter().map(|clause| {
        let mut judged = item(&format!("check-{clause}"));
        judged["clause"] = json!(clause);
        judged["kind"] = json!("check");
        judged["runs"] = json!("every-candidate");
        judged["doors"] = json!([]);
        judged
    });
    let items: Vec<Value> = hunts.chain(beyond).chain(checks).collect();
    if !items.is_empty() {
        rig.run(&item_set(RUN), &json!(items).to_string())
            .must(OK, "the test set");
    }
    let found = |keys: &[&'static str]| -> Vec<&Finding> {
        keys.iter()
            .map(|key| {
                truth
                    .iter()
                    .find(|finding| finding.key == *key)
                    .unwrap_or_else(|| panic!("no finding `{key}` in the table"))
            })
            .collect()
    };
    let standing = found(round.findings);
    let later = found(round.later);
    if standing.iter().any(|finding| finding.key == "oos-listed") {
        rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
            .must(OK, "the declared bound");
    }
    // The rounds, one after the other: a round's scope, the findings once round 1 has
    // doors to place them by, what its items did in its test stage, its record — and then
    // what the human ruled after it, which the script takes only while the state asks.
    for (n, record) in round.rounds.iter().enumerate() {
        let number = (n + 1).to_string();
        let away = round.away.contains(&(n + 1));
        let doors = if away {
            scope(&[EXCLUDED], &[INSIDE])
        } else {
            scope(&[INSIDE], &[EXCLUDED])
        };
        rig.run(&scope_set(RUN, &number), &doors.to_string())
            .must(OK, &format!("{what}: round {number}'s scope"));
        // The findings stand in the LATEST round: it is that round's triage that found
        // them — a round is begun only once the one before it left nothing open, or was
        // dropped.
        if n + 1 == round.rounds.len() {
            plant_in(&rig, &standing, &number);
        }
        // Every item that judges the clause and that this round's scope selects: a hunt
        // behind `INSIDE` unless the round is away, a check always. In a round BEFORE the
        // one the row names for a clause, those items ran green: a round is begun only
        // where the one before it owes no run — unless a fix stage followed it, after which
        // the next round's `test` comes whatever the clause table says.
        let mut results = Vec::new();
        let fixed =
            record.is_some_and(|facts| facts.contains("cycles") || facts.contains("outcome"));
        for (clause, outcome, at) in round.clauses {
            let outcome = match (n + 1).cmp(at) {
                std::cmp::Ordering::Equal => outcome,
                std::cmp::Ordering::Less if *at > 0 && !fixed && record.is_some() => &"green",
                _ => continue,
            };
            if round.instruments.contains(clause) && !away {
                results.push(ran(&format!("row-{clause}"), outcome));
            }
            if round.checks.contains(clause) {
                results.push(ran(&format!("check-{clause}"), outcome));
            }
        }
        if !results.is_empty() {
            rig.run(
                &result_set(RUN, &number, &candidate(n + 1)),
                &json!(results).to_string(),
            )
            .must(OK, "what the round's items did");
        }
        if let Some(facts) = record {
            let mut given: Value =
                serde_json::from_str(&format!("{{{facts}}}")).expect("a round's facts");
            given["candidate"] = json!(candidate(n + 1));
            write_round(&rig, &number, &given.to_string());
        }
        for (_, facts) in round.after.iter().filter(|(at, _)| *at == n + 1) {
            rig.run(&round_set(RUN, &number), &format!("{{{facts}}}"))
                .must(OK, "what the human ruled after the round");
        }
    }
    let latest = candidate(round.rounds.len());
    for step in round.then {
        match step {
            Then::Rerun(at, item, outcome) => rig
                .run(
                    &result_set(RUN, &at.to_string(), &latest),
                    &ran(item, outcome).to_string(),
                )
                .must(OK, "an item run again, where the state asks for it"),
            Then::Fact(at, facts) => rig
                .run(&round_set(RUN, &at.to_string()), &format!("{{{facts}}}"))
                .must(OK, "what the human granted"),
            Then::Rule(key, disposition) => rig
                .run(
                    &ledger_set(RUN),
                    &json!({"key": key, "disposition": disposition}).to_string(),
                )
                .must(OK, "what the human ruled on a finding"),
            Then::Triage(at, key, entry) => rig
                .run(
                    &triage_set(RUN, &at.to_string()),
                    &format!(r#"{{"key": "{key}", {entry}}}"#),
                )
                .must(OK, "one more record of the round's triage"),
            Then::Report(at, attempt) => rig
                .run(&report(RUN, &at.to_string(), "preflight", attempt), BODY)
                .must(OK, "the report of an attempt that reached no record"),
            Then::Patch(key, patch) => rig
                .run(&ledger_set(RUN), &format!(r#"{{"key": "{key}", {patch}}}"#))
                .must(OK, "a ledger patch"),
            Then::Run(facts) => rig
                .run(&run_set(RUN), &format!("{{{facts}}}"))
                .must(OK, "a bound, raised by the human"),
        };
    }
    plant_in(&rig, &later, &round.rounds.len().to_string());
    assert!(
        (!round.away.contains(&round.rounds.len()) && !round.rounds.is_empty())
            || (standing.is_empty() && later.is_empty()),
        "{what}: the latest round is where the findings are placed, by the doors of [`ground`]"
    );

    let read = rig.state();
    assert_eq!(read["next"], json!(round.next), "{what}: {read}");
    let says: Value = serde_json::from_str(round.says).expect("what the document says");
    for (field, value) in says.as_object().expect("an object") {
        assert_eq!(&read[field], value, "{what}: `{field}` in {read}");
    }
    // `stop` is the step exactly when the document says why, and then it names the step
    // that follows.
    assert_eq!(
        round.next == "stop",
        !read["stop"].is_null(),
        "{what}: {read}"
    );

    // What the derivation gives each clause: the status this cell's row names by hand —
    // or, where it names none, the one outcome its items had. And the clause table on
    // disk says the same, row for row: it is the view of exactly this.
    let status_of = |clause: &str, shorthand: &'static str| {
        round
            .derived
            .iter()
            .find(|(named, _)| *named == clause)
            .map_or(shorthand, |(_, status)| *status)
    };
    let derived: Vec<(&str, &str)> = round
        .clauses
        .iter()
        .map(|(clause, outcome, _)| (*clause, status_of(clause, outcome)))
        .collect();
    let said: Vec<(String, String)> = read["clauses"]
        .as_array()
        .expect("the clause rows")
        .iter()
        .map(|row| {
            let text = |field: &str| row[field].as_str().expect("text").to_owned();
            (text("clause"), text("status"))
        })
        .collect();
    assert_eq!(
        said,
        derived
            .iter()
            .map(|(clause, status)| ((*clause).to_owned(), (*status).to_owned()))
            .collect::<Vec<_>>(),
        "{what}: what the item results give each clause: {read}"
    );
    if !census.is_empty() {
        let view: Vec<(String, String)> = table(&rig.read(&clauses_path()), &CLAUSE_COLUMNS)
            .into_iter()
            .map(|row| (row[0].trim_matches('`').to_owned(), row[4].clone()))
            .collect();
        assert_eq!(view, said, "{what}: the clause table is the view of it");
    }

    // What forbids closing, by the clause table, by the ledger and by the candidate — and
    // `close`, now or after a stop, only when nothing does.
    let mut forbids: Vec<Value> = derived
        .iter()
        .filter(|(_, status)| *status != "green")
        .map(|(clause, status)| json!({"clause": clause, "status": status}))
        .collect();
    if round.clauses.is_empty() {
        forbids.push(json!({"clauses": "no row"}));
    }
    let routed_to = |finding: &Finding| {
        round
            .rerouted
            .iter()
            .find(|(key, _)| *key == finding.key)
            .map_or(finding.route, |(_, route)| *route)
    };
    forbids.extend(
        standing
            .iter()
            .filter(|finding| routed_to(finding) != "recorded")
            .map(|finding| json!({"finding": finding.key, "route": routed_to(finding)})),
    );
    forbids.extend(
        round
            .filed
            .iter()
            .filter(|(_, route)| *route != "recorded")
            .map(|(key, route)| json!({"finding": key, "route": route})),
    );
    forbids.extend(
        later
            .iter()
            .filter(|finding| finding.route != "recorded")
            .map(|finding| json!({"finding": finding.key, "route": finding.route})),
    );
    // The candidate is tested while the latest round has a record and no fix stage in it:
    // each cell's `rounds` says so in the fixture's own words.
    let current = round.rounds.last().is_some_and(|last| {
        last.is_some_and(|facts| !facts.contains("cycles") && !facts.contains("outcome"))
    });
    if !current {
        let latest = (!round.rounds.is_empty()).then_some(round.rounds.len());
        forbids.push(json!({"candidate": "not-tested", "round": latest}));
    }
    assert_eq!(
        read["candidate"]["current"],
        json!(current),
        "{what}: {read}"
    );
    assert_eq!(read["forbids_close"], json!(forbids), "{what}: {read}");
    if round.next == "close" || read["stop"]["then"] == "close" {
        assert!(
            forbids.is_empty(),
            "{what}: close, only when nothing forbids it"
        );
    }
    // A stage `next` names is one its position does not refuse.
    for (step, stage) in [("fix", "fix"), ("test", "test"), ("retest", "test")] {
        if round.next == step {
            assert!(
                read["position"][stage]["refused"].is_null(),
                "{what}: `{step}` names a stage that is refused: {read}"
            );
        }
    }
    // A re-run is the `test` stage's position exactly when it is the step.
    assert_eq!(
        round.next == "retest",
        !read["position"]["test"]["rerun"].is_null(),
        "{what}: the position names a re-run when it is the step, and only then: {read}"
    );
    // And an unfinished triage has exactly one stage that finishes it.
    let finishes = ["test", "fix"]
        .iter()
        .filter(|stage| read["position"][**stage]["triage"] == json!(true))
        .count();
    assert_eq!(
        finishes,
        usize::from(round.next == "triage"),
        "{what}: the stage that finishes the round's triage: {read}"
    );
}

/// The table is driven in three parts, so that no one test is a hundred rigs long.
fn drive_rounds(part: usize) {
    const PARTS: usize = 3;
    let truth = findings();
    let truth = &truth;
    let cells: Vec<(usize, &Round)> = ROUNDS
        .iter()
        .enumerate()
        .filter(|(n, _)| n % PARTS == part)
        .collect();
    let driven: usize = std::thread::scope(|threads| {
        let workers: Vec<_> = cells
            .chunks(2)
            .map(|rounds| {
                threads.spawn(move || {
                    for (n, round) in rounds {
                        drive_round(&format!("next-{n}"), round, truth);
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
    assert!(
        driven >= 40 && ROUNDS.len() >= 121,
        "the table collapsed to {driven} cells of {}",
        ROUNDS.len()
    );
}

#[test]
fn the_next_step_is_computed_from_the_runs_recorded_state_part_1_of_3() {
    drive_rounds(0);
}

#[test]
fn the_next_step_is_computed_from_the_runs_recorded_state_part_2_of_3() {
    drive_rounds(1);
}

#[test]
fn the_next_step_is_computed_from_the_runs_recorded_state_part_3_of_3() {
    drive_rounds(2);
}

/// A run of two clauses — `clause-a` judged by a check that runs on every candidate,
/// `clause-b` by a hunt whose one door is [`INSIDE`] — opened with `facts`, and no round yet.
fn two_clause_run(label: &str, facts: &str) -> Rig {
    let rig = Rig::new(label);
    rig.run(
        &run_set(RUN),
        &format!(r#"{{{facts}, {RELEASE}, "clauses": ["clause-a", "clause-b"]}}"#),
    )
    .must(OK, "the run's facts, and its census");
    let mut check = item("check-clause-a");
    check["clause"] = json!("clause-a");
    check["kind"] = json!("check");
    check["runs"] = json!("every-candidate");
    check["doors"] = json!([]);
    let mut hunt = item("row-clause-b");
    hunt["clause"] = json!("clause-b");
    rig.run(&item_set(RUN), &json!([check, hunt]).to_string())
        .must(OK, "the test set");
    rig
}

/// Record round `n` as its `test` stage does — over doors that reach the hunt or leave it
/// out: what each of `results` did, as `(item, outcome)`, and then its candidate, with
/// `more` beside it.
fn tested_round(rig: &Rig, n: usize, reaches: bool, results: &[(&str, &str)], more: Value) {
    let doors = if reaches {
        scope(&[INSIDE], &[EXCLUDED])
    } else {
        scope(&[EXCLUDED], &[INSIDE])
    };
    rig.run(&scope_set(RUN, &n.to_string()), &doors.to_string())
        .must(OK, "a round's scope");
    if !results.is_empty() {
        let rows: Vec<Value> = results
            .iter()
            .map(|(item, outcome)| ran(item, outcome))
            .collect();
        rig.run(
            &result_set(RUN, &n.to_string(), &candidate(n)),
            &json!(rows).to_string(),
        )
        .must(OK, "what the round's items did");
    }
    let mut facts = more;
    facts["candidate"] = json!(candidate(n));
    rig.run(&round_set(RUN, &n.to_string()), &facts.to_string())
        .must(OK, "a round's record");
}

/// One item run again in round `n`, on the candidate round `on` tested.
fn rerun(rig: &Rig, n: usize, on: usize, item: &str, outcome: &str) -> Seen {
    rig.run(
        &result_set(RUN, &n.to_string(), &candidate(on)),
        &ran(item, outcome).to_string(),
    )
}

/// `(round, behind, status, retry)` of each clause, in the census's order.
fn worth(read: &Value) -> Vec<Value> {
    read["clauses"]
        .as_array()
        .expect("the clause rows")
        .iter()
        .map(|row| json!([row["round"], row["behind"], row["status"], row["retry"]]))
        .collect()
}

/// What the dissent on scoping by door was answered with: at closing the record shows,
/// per clause, how far behind the current candidate its last evidence is. The unit is the
/// one the committed record supports — fix rounds on record since the round of its latest
/// run. And what that evidence is worth: a check's run is current only on the present
/// candidate, a hunt's while no landed fix reaches its doors.
#[test]
fn a_clauses_evidence_says_how_many_fix_rounds_behind_the_candidate_it_is() {
    let rig = two_clause_run("behind", r#""stop": "at-the-bound", "rounds": 9"#);
    assert_eq!(
        worth(&rig.state()),
        [
            json!([null, null, "void", "never-selected"]),
            json!([null, null, "void", "never-selected"])
        ],
        "before any round is tested no item is owed, and nothing is behind anything"
    );

    // Round 1 runs both items. While no fix stage is on record its candidate is the run's.
    let both = [("check-clause-a", "green"), ("row-clause-b", "green")];
    tested_round(&rig, 1, true, &both, json!({}));
    let read = rig.state();
    assert_eq!(
        worth(&read),
        [json!([1, 0, "green", null]), json!([1, 0, "green", null])]
    );
    assert_eq!(
        (&read["next"], &read["candidate"]),
        (
            &json!("close"),
            &json!({"round": 1, "commit": candidate(1), "current": true})
        ),
        "{read}"
    );

    // A fix cycle is recorded in round 1: the candidate it tested is no longer the run's.
    // No run is current — the clause table says void, of both — and nothing closes.
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "a recorded fix cycle");
    let read = rig.state();
    assert_eq!(
        worth(&read),
        [json!([1, 1, "void", null]), json!([1, 1, "void", null])]
    );
    assert_eq!(
        (&read["next"], &read["candidate"], &read["fix_rounds"]),
        (
            &json!("test"),
            &json!({"round": 1, "commit": candidate(1), "current": false}),
            &json!(1)
        ),
        "{read}"
    );
    assert_eq!(
        read["forbids_close"],
        json!([
            {"clause": "clause-a", "status": "void"},
            {"clause": "clause-b", "status": "void"},
            {"candidate": "not-tested", "round": 1},
        ]),
        "{read}"
    );
    assert_eq!(
        (
            &read["clauses"][0]["items"][0]["why"],
            &read["clauses"][1]["items"][0]["why"]
        ),
        (&json!("not-current"), &json!("not-current")),
        "{read}"
    );

    // Round 2 tests what the fixes left, over doors that do not reach the hunt — and its
    // record holds no run of the check. The hunt's green from before the landing stands,
    // one fix round behind; the check's does not: it is of an earlier candidate.
    tested_round(&rig, 2, false, &[], json!({}));
    let read = rig.state();
    assert_eq!(
        (worth(&read), &read["next"], &read["retest"]),
        (
            vec![
                json!([null, null, "void", "due"]),
                json!([1, 1, "green", null])
            ],
            &json!("retest"),
            &json!(["clause-a"])
        ),
        "a check green on an earlier candidate has no say on this one: {read}"
    );
    assert_eq!(
        read["clauses"][0]["items"][0],
        json!({"item": "check-clause-a", "runs": "every-candidate", "round": 2, "attempt": null,
               "commit": null, "outcome": null, "reason": null, "standing": "void",
               "why": "not-run", "rerun": "due"}),
        "{read}"
    );
    rerun(&rig, 2, 2, "check-clause-a", "green").must(OK, "the check, run on this candidate");
    let read = rig.state();
    assert_eq!(
        worth(&read),
        [json!([2, 0, "green", null]), json!([1, 1, "green", null])]
    );
    assert_eq!(
        (&read["next"], &read["forbids_close"]),
        (&json!("close"), &json!([])),
        "{read}"
    );

    // A second landing, and a round 3 whose doors do reach the hunt: round 3 owes its run,
    // and the green of round 1 is no longer what stands for it.
    rig.run(&round_set(RUN, "2"), &json!({"cycles": 2}).to_string())
        .must(OK, "round 2's fix cycles");
    tested_round(&rig, 3, true, &[("check-clause-a", "green")], json!({}));
    let read = rig.state();
    assert_eq!(
        (worth(&read), &read["next"], &read["fix_rounds"]),
        (
            vec![
                json!([3, 0, "green", null]),
                json!([null, null, "void", "due"])
            ],
            &json!("retest"),
            &json!(2)
        ),
        "{read}"
    );
    rerun(&rig, 3, 3, "row-clause-b", "green").must(OK, "the hunt, run where the fixes reach");
    assert_eq!(rig.state()["next"], json!("close"));
}

/// The human's go after a stop is a recorded fact, and a fact of exactly that stop: the
/// script takes it for the round the run is stopped after, and at no other time — a go
/// written early would lift a stop nobody has seen.
#[test]
fn the_humans_go_is_a_recorded_fact_taken_only_for_the_stop_it_answers() {
    let rig = two_clause_run("go", EVERY_ROUND);
    let go = json!({"go": true}).to_string();
    let stopped = |read: &Value| read["stop"].clone();

    // Before round 1 there is no stop, and no round the go could be a fact of.
    let before = rig.snapshot();
    rig.run(&round_set(RUN, "1"), &go)
        .refused(BAD_VALUE, "a go before any round");
    assert_eq!(rig.snapshot(), before, "a refused go writes nothing");

    // Round 1 is tested and leaves a blocker: the round is not over, and nothing stops.
    let both = [("check-clause-a", "green"), ("row-clause-b", "green")];
    tested_round(&rig, 1, true, &both, json!({}));
    rig.seed_rows(&["blocker"]);
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "blocker", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "round 1's triage");
    let read = rig.state();
    assert_eq!(
        (&read["next"], stopped(&read)),
        (&json!("fix"), json!(null))
    );
    let before = rig.snapshot();
    rig.run(&round_set(RUN, "1"), &go)
        .refused(BAD_VALUE, "a go inside a round");
    assert_eq!(rig.snapshot(), before);

    // Its fix lands: the round is over, the run stops, and the next round waits.
    rig.run(
        &ledger_set(RUN),
        &json!({"key": "blocker", "disposition": "fixed", "detail": "0f34d8f0"}).to_string(),
    )
    .must(OK, "the fix");
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "the fix cycle");
    let read = rig.state();
    assert_eq!(
        (&read["next"], stopped(&read), &read["position"]["test"]),
        (
            &json!("stop"),
            json!({"why": "every-round", "round": 1, "then": "test"}),
            &json!({"refused": "stopped", "round": 1})
        ),
        "{read}"
    );
    assert_eq!(
        read["position"]["fix"]["land"],
        json!(true),
        "a stop never keeps a round from landing: {read}"
    );
    rig.run(&round_set(RUN, "2"), &go)
        .refused(BAD_VALUE, "a go for a round that does not exist yet");
    // Even here, where a go is asked for, it is `true` and nothing that merely reads as it.
    let before = rig.snapshot();
    for (said, why) in [
        (json!({"go": "yes"}), "a go that is text, at the stop"),
        (json!({"go": 1}), "a go that is a number, at the stop"),
        (json!({"go": [true]}), "a go that is a list, at the stop"),
    ] {
        rig.run(&round_set(RUN, "1"), &said.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);

    // The go, written: `next` answers what the stop named, and the stage is allowed.
    let seen = rig.run(&round_set(RUN, "1"), &go);
    seen.must(OK, "the human's go after round 1");
    assert_eq!(seen.json()["facts"]["go"], json!(true));
    assert!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS)
            .contains(&vec!["`go`".to_owned(), "`c1`".to_owned()]),
        "the go is a row of the round's record"
    );
    rig.run(&round_set(RUN, "1"), &go)
        .must(OK, "a go said again, as it stands");
    let read = rig.state();
    assert_eq!(
        (&read["next"], stopped(&read), &read["position"]["test"]),
        (
            &json!("test"),
            json!(null),
            &json!({"round": 2, "attempt": 1})
        ),
        "{read}"
    );
    assert_eq!(read["rounds"][0]["facts"]["go"], json!(true));

    // Round 2 confirms the candidate and leaves nothing: over, and the run stops again —
    // round 1's go is round 1's.
    tested_round(&rig, 2, false, &[("check-clause-a", "green")], json!({}));
    let read = rig.state();
    assert_eq!(
        (&read["next"], stopped(&read)),
        (
            &json!("stop"),
            json!({"why": "every-round", "round": 2, "then": "close"})
        ),
        "{read}"
    );
    rig.run(&round_set(RUN, "2"), &go)
        .must(OK, "the human's go after round 2");
    assert_eq!(rig.state()["next"], json!("close"));

    // A run that stops at its bound has no such stop — and the stop at a spent bound is
    // not one a go lifts: that go is the raised bound, and only the human raises it.
    let other = two_clause_run("go-bounded", r#""stop": "at-the-bound", "rounds": 1"#);
    tested_round(&other, 1, true, &both, json!({}));
    assert_eq!(other.state()["next"], json!("close"));
    other
        .run(&round_set(RUN, "1"), &go)
        .refused(BAD_VALUE, "a go in a run that stops at its bound");
    other
        .run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "round 1's fix cycle: the bound of one is spent");
    tested_round(&other, 2, true, &both, json!({}));
    other.seed_rows(&["blocker"]);
    other
        .run(
            &triage_set(RUN, "2"),
            &json!({"key": "blocker", "grade": "breaks", "verdict": "confirmed", "regression": false})
                .to_string(),
        )
        .must(OK, "round 2's triage");
    let read = other.state();
    assert_eq!(
        (&read["next"], stopped(&read), &read["position"]["fix"]),
        (
            &json!("stop"),
            json!({"why": "round-bound", "round": 2, "then": "fix"}),
            &json!({"refused": "round-bound", "round": 2})
        ),
        "{read}"
    );
    let before = other.snapshot();
    other
        .run(&round_set(RUN, "2"), &go)
        .refused(BAD_VALUE, "a go for a stop at the bound");
    assert_eq!(other.snapshot(), before);
    other
        .run(&run_set(RUN), &json!({"rounds": 2}).to_string())
        .must(OK, "the bound, raised by the human");
    let read = other.state();
    assert_eq!(
        (&read["next"], &read["position"]["fix"]["cycle"]),
        (&json!("fix"), &json!(1)),
        "{read}"
    );
}

/// **A re-run is an attempt inside the round that selected the item — one automatic
/// attempt, on record, then the human.** It is a result like any other, taken only where
/// the state asks for it (the review's `F5`, the repair plan's state 9); it begins no round,
/// so it resolves no scope and counts against no bound; and the one ruling with a recorded
/// form is one more of it, granted to the clause whose item is spent — in the round that
/// selected that item, and to no other. Nothing removes a clause: that is the run's opening.
#[test]
fn a_rerun_is_an_attempt_inside_its_round_taken_only_where_the_state_asks_and_granted_by_the_human()
{
    let rig = two_clause_run("rerun", r#""stop": "at-the-bound", "rounds": 9"#);
    let grant = |clause: &str| json!({"granted": clause}).to_string();
    tested_round(
        &rig,
        1,
        true,
        &[("check-clause-a", "green"), ("row-clause-b", "void")],
        json!({}),
    );

    // The hunt's one re-run is due: the position of the `test` stage IS that re-run — its
    // round, the attempt it will be, the doors it runs over again — and no new round.
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["retest"], &read["position"]["test"]),
        (
            &json!("retest"),
            &json!(["clause-b"]),
            &json!({"round": 1, "attempt": 1, "rerun": {
                "clause": "clause-b", "round": 1,
                "items": [{"item": "row-clause-b", "attempt": 2}],
                "doors": [door(INSIDE)]}})
        ),
        "{read}"
    );
    // Nothing is the human's yet: there is nothing to grant. And a re-run nobody is asked
    // for is refused — of an item that is green, in a round that is not the item's, in a
    // round that does not exist.
    let before = rig.snapshot();
    rig.run(&round_set(RUN, "1"), &grant("clause-b"))
        .refused(BAD_VALUE, "a grant before the one re-run was spent");
    rerun(&rig, 1, 1, "check-clause-a", "green")
        .refused(BAD_VALUE, "a re-run of an item that is green");
    rerun(&rig, 2, 1, "row-clause-b", "green")
        .refused(BAD_VALUE, "a re-run in a round the run has not begun");
    assert_eq!(
        rig.snapshot(),
        before,
        "a refused re-run, or grant, writes nothing"
    );

    // The re-run leaves it void: attempt 2 of round 1, and the clause is the human's.
    let seen = rerun(&rig, 1, 1, "row-clause-b", "void");
    seen.must(OK, "the one re-run that is automatic");
    assert_eq!(
        seen.json()["recorded"],
        json!([{"item": "row-clause-b", "attempt": 2, "outcome": "void"}])
    );
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["human_clauses"], &read["round"]),
        (
            &json!("rule"),
            &json!([{"clause": "clause-b", "why": "not-green-after-its-rerun"}]),
            &json!(1)
        ),
        "a re-run began no round: {read}"
    );
    assert!(read["position"]["test"]["rerun"].is_null(), "{read}");
    let before = rig.snapshot();
    rerun(&rig, 1, 1, "row-clause-b", "green").refused(BAD_VALUE, "a third attempt nobody granted");
    for (round, clause, why) in [
        ("1", "clause-a", "a grant for a clause that is green"),
        ("1", "clause-c", "a grant for a clause the census lacks"),
        (
            "2",
            "clause-b",
            "a grant in a round that selected no item of it",
        ),
    ] {
        rig.run(&round_set(RUN, round), &grant(clause))
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);

    // Granted: the record holds the attempt it allows, one more re-run is due, and a
    // second grant is not asked for — one grant, one attempt.
    let seen = rig.run(&round_set(RUN, "1"), &grant("clause-b"));
    seen.must(OK, "one more re-run, granted");
    assert_eq!(seen.json()["facts"]["granted"], json!(["row-clause-b a3"]));
    rig.run(&round_set(RUN, "1"), &grant("clause-b"))
        .refused(BAD_VALUE, "a second grant while the first is not spent");
    let read = rig.state();
    assert_eq!(
        (
            &read["next"],
            &read["retest"],
            &read["human_clauses"],
            &read["position"]["test"]["rerun"]["items"]
        ),
        (
            &json!("retest"),
            &json!(["clause-b"]),
            &json!([]),
            &json!([{"item": "row-clause-b", "attempt": 3}])
        ),
        "{read}"
    );

    // That one leaves it void again: the human's again, and one more may be granted. Then
    // it runs green, and the run closes — on four attempts of one round.
    rerun(&rig, 1, 1, "row-clause-b", "void").must(OK, "the granted re-run");
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["clauses"][1]["retry"]),
        (&json!("rule"), &json!("spent")),
        "{read}"
    );
    rig.run(&round_set(RUN, "1"), &grant("clause-b"))
        .must(OK, "and one more");
    assert_eq!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS)[1],
        ["`granted`", "`row-clause-b a3, row-clause-b a4`"],
        "each grant is on record, as the attempt it allowed"
    );
    rerun(&rig, 1, 1, "row-clause-b", "green").must(OK, "the second granted re-run");
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["fix_rounds"], &read["round"]),
        (&json!("close"), &json!(0), &json!(1)),
        "three re-runs are no round and no fix round: {read}"
    );
    let commit = format!("`{}`", candidate(1));
    assert_eq!(
        table(&rig.read(&results_path("1")), &RESULT_COLUMNS),
        [
            ["`check-clause-a`", "1", commit.as_str(), "green", "-"],
            [
                "`row-clause-b`",
                "1",
                commit.as_str(),
                "void",
                "its driver died"
            ],
            [
                "`row-clause-b`",
                "2",
                commit.as_str(),
                "void",
                "its driver died"
            ],
            [
                "`row-clause-b`",
                "3",
                commit.as_str(),
                "void",
                "its driver died"
            ],
            ["`row-clause-b`", "4", commit.as_str(), "green", "-"],
        ],
        "which item ran, which attempt, on which commit, with which outcome"
    );

    // A fix lands, and round 2's doors reach the hunt again: its attempts are round 2's,
    // counted anew — round 1's grants allow nothing here.
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "round 1's fix cycle");
    tested_round(
        &rig,
        2,
        true,
        &[("check-clause-a", "green"), ("row-clause-b", "void")],
        json!({}),
    );
    assert_eq!(
        rig.state()["position"]["test"]["rerun"]["items"],
        json!([{"item": "row-clause-b", "attempt": 2}])
    );
    rerun(&rig, 1, 2, "row-clause-b", "green").refused(
        BAD_VALUE,
        "a re-run in round 1, which is no longer the round that owes the item",
    );
    rerun(&rig, 2, 2, "row-clause-b", "void").must(OK, "round 2's one re-run");
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["rounds"][1]["facts"]["granted"]),
        (&json!("rule"), &json!([])),
        "{read}"
    );

    // And while a finding is open the state asks for no re-run at all, granted or not.
    rig.run(&round_set(RUN, "2"), &grant("clause-b"))
        .must(OK, "one more, granted in round 2");
    rig.seed_rows(&["blocker"]);
    rig.run(
        &triage_set(RUN, "2"),
        &json!({"key": "blocker", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "round 2's triage");
    assert_eq!(rig.state()["next"], json!("fix"));
    let before = rig.snapshot();
    rerun(&rig, 2, 2, "row-clause-b", "green")
        .refused(BAD_VALUE, "a re-run while a finding is open");
    assert_eq!(rig.snapshot(), before);

    // The script has no call that removes a clause or an item of the test set.
    let help = rig.run(&strings(&["--help"]), "");
    help.must(OK, "the header");
    assert!(
        help.stdout.contains("NOTHING REMOVES A CLAUSE"),
        "the header says why no removal is offered"
    );
    assert!(
        subcommands(&rig).iter().all(|name| !name.contains("remove")
            && !name.contains("drop")
            && !name.contains("delete")),
        "no subcommand removes anything"
    );
}

// ---------------------------------------------------------------------------
// 19 · The test set: instrument items, as data
// ---------------------------------------------------------------------------

/// The instruments of a run are rows, never a list in the harness: an item carries its
/// kind, the clause it judges, when it runs, what it covers and its brief. The kind is a
/// slug and no vocabulary of the script's — the harness has a chain per kind, and a kind
/// it has none for is the harness's halt.
#[test]
fn the_test_set_is_written_item_by_item_and_an_item_is_replaced_whole() {
    let rig = Rig::new("test-set");
    let mut gate = item("the-gate");
    gate["kind"] = json!("check");
    gate["clause"] = json!("no-regression");
    gate["runs"] = json!("every-candidate");
    gate["brief"] = json!("the full `dev/gate` | on the candidate");
    gate.as_object_mut().expect("an object").remove("doors");
    let mut row3 = item("row-3");
    row3["doors"] = json!(["jigc setup", "jigc doc show"]);
    row3["registries"] = json!(["the verb table"]);
    let seen = rig.run(&item_set(RUN), &json!([gate, row3]).to_string());
    seen.must(OK, "two items");
    assert_eq!(
        seen.json(),
        json!({"test_set": test_set_path(), "written": ["the-gate", "row-3"]})
    );
    // A kind nobody has built a chain for yet is a row like any other.
    let mut later = item("port-rehearsal");
    later["kind"] = json!("rehearsal");
    later["doors"] = json!([]);
    later["registries"] = json!(["the migration verbs"]);
    rig.run(&item_set(RUN), &later.to_string())
        .must(OK, "an item of a kind that came later");
    // An item the set holds is replaced whole, where it stands.
    row3["brief"] = json!("the row's brief, corrected");
    row3["doors"] = json!(["jigc setup"]);
    row3.as_object_mut()
        .expect("an object")
        .remove("registries");
    rig.run(&item_set(RUN), &row3.to_string())
        .must(OK, "an item, again");
    assert_eq!(
        table(&rig.read(&test_set_path()), &ITEM_COLUMNS),
        vec![
            vec![
                "`the-gate`",
                "`check`",
                "`no-regression`",
                "every-candidate",
                "-",
                "-",
                "the full `dev/gate` | on the candidate",
            ],
            vec![
                "`row-3`",
                "`review-row`",
                "`no-lost-files`",
                "in-scope",
                "jigc setup",
                "-",
                "the row's brief, corrected",
            ],
            vec![
                "`port-rehearsal`",
                "`rehearsal`",
                "`no-lost-files`",
                "in-scope",
                "-",
                "the migration verbs",
                "the row's brief, as the opening record names it",
            ],
        ]
    );
    assert_eq!(
        rig.state()["items"][0],
        json!({
            "item": "the-gate",
            "kind": "check",
            "clause": "no-regression",
            "runs": "every-candidate",
            "doors": [],
            "registries": [],
            "brief": "the full `dev/gate` | on the candidate",
            "selected": true,
        }),
        "an item, read back as data"
    );

    let before = rig.snapshot();
    type Spoil = (&'static str, fn(&mut Value));
    let spoils: &[Spoil] = &[
        ("an item that runs on a whim", |i| {
            i["runs"] = json!("sometimes")
        }),
        ("an in-scope item that covers nothing", |i| {
            i["doors"] = json!([])
        }),
        ("a blank brief", |i| i["brief"] = json!("  ")),
        ("a brief that is not text", |i| i["brief"] = json!(3)),
        ("doors that are no list", |i| {
            i["doors"] = json!("jigc setup")
        }),
        ("a door that is not text", |i| i["doors"] = json!([1])),
        ("a blank door", |i| i["doors"] = json!(["jigc setup", " "])),
        ("a door of two lines", |i| {
            i["doors"] = json!(["jigc\nsetup"])
        }),
        ("a door that holds the list's break", |i| {
            i["doors"] = json!(["jigc<br>setup"])
        }),
        ("a door that reads as none", |i| i["doors"] = json!(["-"])),
        ("a door named twice", |i| {
            i["doors"] = json!(["jigc setup", "jigc setup"])
        }),
        ("a registry of two lines", |i| {
            i["registries"] = json!(["the verb\ntable"])
        }),
        ("a field nobody defined", |i| i["selected"] = json!(true)),
        ("no clause", |i| {
            i.as_object_mut().expect("an object").remove("clause");
        }),
    ];
    for (why, spoil) in spoils {
        let mut given = item("row-9");
        spoil(&mut given);
        rig.run(&item_set(RUN), &given.to_string())
            .refused(BAD_VALUE, why);
    }
    rig.run(
        &item_set(RUN),
        &json!([item("row-9"), item("row-9")]).to_string(),
    )
    .refused(BAD_VALUE, "an item given twice in one batch");
    rig.run(&item_set(RUN), "[]")
        .refused(BAD_VALUE, "a batch of no item");
    assert_eq!(
        rig.snapshot(),
        before,
        "nothing of a refused batch is written"
    );
}

/// One item of [`SELECTIONS`]: how it runs and what it covers, and whether a round whose
/// scope is [`selection_scope`] runs it.
struct Selection {
    id: &'static str,
    runs: &'static str,
    doors: &'static [&'static str],
    registries: &'static [&'static str],
    selected: bool,
}

/// **Which items a round runs** — *hunt once per change, check always*. The round's scope
/// includes `jigc setup` of the verb table and `finalize.dirty` of the finding codes, and
/// excludes `jigc doc show` of the verb table and `the pack loader` of the load paths.
const SELECTIONS: &[Selection] = &[
    Selection {
        id: "check-always",
        runs: "every-candidate",
        doors: &[],
        registries: &[],
        selected: true,
    },
    Selection {
        id: "check-always-covering-an-excluded-door",
        runs: "every-candidate",
        doors: &["jigc doc show"],
        registries: &[],
        selected: true,
    },
    Selection {
        id: "an-included-door",
        runs: "in-scope",
        doors: &["jigc setup"],
        registries: &[],
        selected: true,
    },
    Selection {
        id: "an-included-door-among-others",
        runs: "in-scope",
        doors: &["jigc doc show", "jigc rename", "finalize.dirty"],
        registries: &[],
        selected: true,
    },
    Selection {
        id: "an-excluded-door",
        runs: "in-scope",
        doors: &["jigc doc show"],
        registries: &[],
        selected: false,
    },
    Selection {
        id: "a-door-the-scope-never-listed",
        runs: "in-scope",
        doors: &["jigc rename"],
        registries: &[],
        selected: false,
    },
    Selection {
        id: "a-door-that-only-resembles-one",
        runs: "in-scope",
        doors: &["jigc setup "],
        registries: &[],
        selected: true,
    },
    Selection {
        id: "a-longer-door",
        runs: "in-scope",
        doors: &["jigc setup --force"],
        registries: &[],
        selected: false,
    },
    Selection {
        id: "the-registry-of-an-included-door",
        runs: "in-scope",
        doors: &[],
        registries: &["the finding codes"],
        selected: true,
    },
    Selection {
        id: "a-registry-with-an-included-and-an-excluded-door",
        runs: "in-scope",
        doors: &[],
        registries: &["the verb table"],
        selected: true,
    },
    Selection {
        id: "a-registry-whose-doors-are-all-excluded",
        runs: "in-scope",
        doors: &[],
        registries: &["the load paths"],
        selected: false,
    },
    Selection {
        id: "a-registry-the-scope-never-read",
        runs: "in-scope",
        doors: &[],
        registries: &["the adapters"],
        selected: false,
    },
    Selection {
        id: "a-door-that-is-a-registrys-name",
        runs: "in-scope",
        doors: &["the finding codes"],
        registries: &[],
        selected: false,
    },
    Selection {
        id: "a-registry-that-is-an-included-doors-name",
        runs: "in-scope",
        doors: &[],
        registries: &["jigc setup"],
        selected: false,
    },
    Selection {
        id: "an-excluded-door-and-an-included-registry",
        runs: "in-scope",
        doors: &["jigc doc show"],
        registries: &["the finding codes"],
        selected: true,
    },
];

fn selection_scope() -> Value {
    let at = |door: &str, registry: &str| json!({"door": door, "registry": registry, "derivation": "read off the registry"});
    json!({
        "included": [at("jigc setup", "the verb table"), at("finalize.dirty", "the finding codes")],
        "excluded": [at("jigc doc show", "the verb table"), at("the pack loader", "the load paths")],
    })
}

fn selected(read: &Value) -> BTreeMap<String, Value> {
    read["items"]
        .as_array()
        .expect("the items")
        .iter()
        .map(|item| {
            (
                item["item"].as_str().expect("an id").to_owned(),
                item["selected"].clone(),
            )
        })
        .collect()
}

#[test]
fn which_items_a_round_runs_is_computed_from_the_test_set_and_the_rounds_doors() {
    let rig = Rig::new("selection");
    let items: Vec<Value> = SELECTIONS
        .iter()
        .map(|s| {
            json!({
                "item": s.id,
                "kind": "review-row",
                "clause": "no-lost-files",
                "runs": s.runs,
                "brief": "a brief",
                "doors": s.doors,
                "registries": s.registries,
            })
        })
        .collect();
    assert!(items.len() >= 15, "the table collapsed");
    rig.run(&item_set(RUN), &json!(items).to_string())
        .must(OK, "the test set");

    // Before any round, and in a round with no scope: what runs on every candidate is
    // selected, and of the rest nobody can say.
    let unknown = |read: &Value, when: &str| {
        for s in SELECTIONS {
            let expected = if s.runs == "every-candidate" {
                json!(true)
            } else {
                Value::Null
            };
            assert_eq!(selected(read)[s.id], expected, "{when}: `{}`", s.id);
        }
    };
    unknown(&rig.state(), "before any round");
    rig.run(&report(RUN, "1", "preflight", "1"), BODY)
        .must(OK, "round 1's first report");
    unknown(&rig.state(), "a round with no scope yet");

    rig.run(&scope_set(RUN, "1"), &selection_scope().to_string())
        .must(OK, "round 1's scope");
    let read = rig.state();
    for s in SELECTIONS {
        assert_eq!(
            selected(&read)[s.id],
            json!(s.selected),
            "`{}` ({}, doors {:?}, registries {:?})",
            s.id,
            s.runs,
            s.doors,
            s.registries
        );
    }

    // It is the LATEST round's doors that are asked: a round 2 that includes nothing runs
    // only what runs on every candidate. (Round 1 is tested first, and left nothing open:
    // a round is begun where the state hands it to a `test` stage.)
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": sha('c')}).to_string(),
    )
    .must(OK, "round 1's candidate");
    rig.run(
        &scope_set(RUN, "2"),
        &scope(&[], &["jigc setup"]).to_string(),
    )
    .must(OK, "round 2's scope");
    let read = rig.state();
    for s in SELECTIONS {
        assert_eq!(
            selected(&read)[s.id],
            json!(s.runs == "every-candidate"),
            "round 2: `{}`",
            s.id
        );
    }
}

// ---------------------------------------------------------------------------
// 20 · The round record: its facts
// ---------------------------------------------------------------------------

#[test]
fn a_round_record_holds_its_facts_and_what_is_written_once_stands() {
    let rig = Rig::new("round-record");
    rig.run(&scope_set(RUN, "1"), &scope(&[INSIDE], &[]).to_string())
        .must(OK, "the round's scope: a round is tested over its doors");
    let seen = rig.run(
        &round_set(RUN, "1"),
        &json!({"binary": sha256('b'), "candidate": sha('c'), "base": "0f34d8f0"}).to_string(),
    );
    seen.must(OK, "the test stage's facts");
    assert_eq!(
        seen.json(),
        json!({
            "round": round_path("1"),
            "facts": {
                "candidate": sha('c'),
                "base": "0f34d8f0",
                "binary": sha256('b'),
                "cycles": 0,
                "outcome": null,
                "go": false,
                "granted": [],
                "reverify": [],
                "again": [],
                "cross-model": [],
                "cross-model-void": [],
                "ran": [],
                "awaiting": [],
                "recorded": [],
            },
        })
    );
    for cycles in [1, 2, 2] {
        rig.run(&round_set(RUN, "1"), &json!({"cycles": cycles}).to_string())
            .must(OK, "a recorded cycle");
    }
    // A fact written once may be said again, as it stands.
    rig.run(
        &round_set(RUN, "1"),
        &json!({"candidate": sha('c'), "outcome": "part"}).to_string(),
    )
    .must(OK, "the candidate again, and the round's outcome");
    assert_eq!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS),
        vec![
            vec!["`candidate`".to_owned(), format!("`{}`", sha('c'))],
            vec!["`base`".to_owned(), "`0f34d8f0`".to_owned()],
            vec!["`binary`".to_owned(), format!("`{}`", sha256('b'))],
            vec!["`cycles`".to_owned(), "`2`".to_owned()],
            vec!["`outcome`".to_owned(), "`part`".to_owned()],
        ],
        "the facts, in the record's own order whatever order they came in"
    );
    assert_eq!(
        rig.state()["rounds"][0]["facts"],
        json!({
            "candidate": sha('c'),
            "base": "0f34d8f0",
            "binary": sha256('b'),
            "cycles": 2,
            "outcome": "part",
            "go": false,
            "granted": [],
            "reverify": [],
            "again": [],
            "cross-model": [],
            "cross-model-void": [],
            "ran": [],
            "awaiting": [],
            "recorded": [],
        })
    );

    let before = rig.snapshot();
    let stands: &[(Value, &str)] = &[
        (json!({"candidate": sha('d')}), "another candidate"),
        (json!({"base": "0f34d8f1"}), "another base"),
        (json!({"binary": sha256('d')}), "another binary"),
        (json!({"outcome": "dropped"}), "another outcome"),
        (
            json!({"cycles": 3, "outcome": "dropped"}),
            "another outcome beside a fact that would be taken",
        ),
    ];
    for (given, why) in stands {
        rig.run(&round_set(RUN, "1"), &given.to_string())
            .refused(EXISTS, why);
    }
    let bad: &[(Value, &str)] = &[
        (json!({"cycles": 1}), "a count that goes down"),
        (json!({"cycles": 0}), "no cycle at all"),
        (json!({"cycles": "3"}), "a count that is text"),
        (json!({"cycles": 2.5}), "half a cycle"),
        (json!({"cycles": true}), "a count that is a truth value"),
        (json!({"candidate": "0f34d8f0"}), "a candidate cut short"),
        (
            json!({"candidate": sha('c').to_uppercase()}),
            "a candidate in upper case",
        ),
        (json!({"base": "HEAD~3"}), "a base that is no sha"),
        (
            json!({"binary": sha('b')}),
            "a binary hash of the wrong length",
        ),
        (json!({"outcome": "landed"}), "an outcome nobody defined"),
        (json!({"go": false}), "a go taken back"),
        (json!({"go": "yes"}), "a go that is text"),
        (json!({"go": 1}), "a go that is a number"),
        (
            json!({"go": true}),
            "a go for a round the run is not stopped after",
        ),
        (
            json!({"granted": "no-lost-files"}),
            "one more re-run for a clause that is not the human's",
        ),
        (
            json!({"cycles": 3, "go": true}),
            "a go nobody asked for beside a fact that would be taken",
        ),
        (json!({"landed": sha('c')}), "a fact nobody defined"),
        (
            json!({"alone": "no-lost-files"}),
            "a clause run alone: a re-run is no round of its own, and the fact is gone",
        ),
        (json!({}), "a record that says nothing"),
        (json!([{"cycles": 3}]), "a list of records"),
    ];
    for (given, why) in bad {
        rig.run(&round_set(RUN, "1"), &given.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before, "a refused record writes nothing");

    // Which items a cross-model pass read, and which were named for one and got none:
    // two lists of item ids, replaced whole, and gone from the table when empty.
    rig.run(
        &round_set(RUN, "2"),
        &json!({"cross-model": ["row-3", "row-setup"], "cross-model-void": ["row-7"]}).to_string(),
    )
    .must(OK, "the round's cross-model passes");
    assert_eq!(
        table(&rig.read(&round_path("2")), &ROUND_COLUMNS),
        vec![
            vec!["`cross-model`", "`row-3, row-setup`"],
            vec!["`cross-model-void`", "`row-7`"],
        ]
    );
    let read = rig.state();
    assert_eq!(
        (
            &read["rounds"][1]["facts"]["cross-model"],
            &read["rounds"][1]["facts"]["cross-model-void"]
        ),
        (&json!(["row-3", "row-setup"]), &json!(["row-7"])),
        "the two lists, read back as data: {read}"
    );
    rig.run(
        &round_set(RUN, "2"),
        &json!({"cross-model": ["row-3"], "cross-model-void": []}).to_string(),
    )
    .must(OK, "the lists, replaced");
    assert_eq!(
        table(&rig.read(&round_path("2")), &ROUND_COLUMNS),
        vec![vec!["`cross-model`", "`row-3`"]]
    );
    let before = rig.snapshot();
    let lists: &[(Value, i32, &str)] = &[
        (
            json!({"cross-model": "row-3"}),
            BAD_VALUE,
            "a list that is one text",
        ),
        (
            json!({"cross-model": true}),
            BAD_VALUE,
            "a list that is a truth value",
        ),
        (
            json!({"cross-model": ["row-3", "row-3"]}),
            BAD_VALUE,
            "an item twice",
        ),
        (
            json!({"cross-model-void": ["Row 3"]}),
            BAD_ID,
            "an item that is no id",
        ),
        (
            json!({"cross-model": ["row-3, row-4"]}),
            BAD_ID,
            "two items in one entry",
        ),
        (
            json!({"cross-model": [3]}),
            BAD_ID,
            "an item that is a number",
        ),
        (
            json!({"granted": "No lost files"}),
            BAD_ID,
            "a granted clause that is no slug",
        ),
    ];
    for (given, status, why) in lists {
        rig.run(&round_set(RUN, "2"), &given.to_string())
            .refused(*status, why);
    }
    assert_eq!(rig.snapshot(), before, "a refused list writes nothing");

    // A round record somebody wrote by hand is not rewritten on a guess.
    fs::write(rig.root.join(round_path("2")), "# round 2\n\nAll fine.\n").expect("a note");
    let before = rig.snapshot();
    rig.run(&round_set(RUN, "2"), &json!({"cycles": 1}).to_string())
        .refused(CORRUPT, "a round record that is not the script's");
    assert_eq!(rig.snapshot(), before);
}

// ---------------------------------------------------------------------------
// 21 · The position: the round, the cycle and the attempt of each stage
// ---------------------------------------------------------------------------

/// One round of a [`Position`]: the facts its record holds (`None`: a directory and no
/// record), and its reports as `(stage directory, attempt)`.
type RoundOnDisk = (
    Option<&'static str>,
    &'static [(&'static str, &'static str)],
);

/// One state of a run's rounds, and what an invocation of each stage would work on.
struct Position {
    name: &'static str,
    /// The rounds, in order.
    rounds: &'static [RoundOnDisk],
    /// Whether a finding stands open in the ledger — by its key in [`findings`].
    finding: Option<&'static str>,
    test: &'static str,
    fix: &'static str,
}

/// The candidate every tested round of [`POSITIONS`] names.
const TESTED: &str = r#""candidate": "cccccccccccccccccccccccccccccccccccccccc""#;

/// **The position's truth table.** A fact list is the inside of a JSON object.
const POSITIONS: &[Position] = &[
    Position {
        name: "no round yet",
        rounds: &[],
        finding: None,
        test: r#"{"round": 1, "attempt": 1}"#,
        fix: r#"{"refused": "no-round", "round": null}"#,
    },
    Position {
        name: "no round yet, and a finding from the opening",
        rounds: &[],
        finding: Some("in-confirmed"),
        test: r#"{"round": 1, "attempt": 1}"#,
        fix: r#"{"refused": "no-round", "round": null}"#,
    },
    Position {
        name: "a round whose test stage left reports and no record: it is run again, once",
        rounds: &[(None, &[("test", "1")])],
        finding: None,
        test: r#"{"round": 1, "attempt": 2}"#,
        fix: r#"{"refused": "not-tested", "round": 1}"#,
    },
    Position {
        name: "a round whose test stage left reports and no record twice: the next attempt is the human's",
        rounds: &[(None, &[("test", "1"), ("test", "2")])],
        finding: None,
        test: r#"{"refused": "attempts-spent", "round": 1}"#,
        fix: r#"{"refused": "not-tested", "round": 1}"#,
    },
    Position {
        name: "a round with a record that names no candidate",
        rounds: &[(Some(r#""base": "0f34d8f0""#), &[])],
        finding: Some("in-confirmed"),
        test: r#"{"round": 1, "attempt": 1}"#,
        fix: r#"{"refused": "not-tested", "round": 1}"#,
    },
    Position {
        name: "a tested round with a blocker",
        rounds: &[(Some(TESTED), &[("test", "1")])],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
    },
    Position {
        name: "a tested round with an item for the human",
        rounds: &[(Some(TESTED), &[("test", "1")])],
        finding: Some("out-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
    },
    // An unfinished triage is finished by the stage that left it: the position says
    // which, and that invocation runs no instrument.
    Position {
        name: "a tested round with a finding nobody verified: the test stage finishes its triage",
        rounds: &[(Some(TESTED), &[("test", "1")])],
        finding: Some("in-unverified"),
        test: r#"{"round": 1, "attempt": 2, "triage": true}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
    },
    Position {
        name: "a tested round with a finding nobody graded, and no report on disk",
        rounds: &[(Some(TESTED), &[])],
        finding: Some("never-triaged"),
        test: r#"{"round": 1, "attempt": 1, "triage": true}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
    },
    Position {
        name: "a counted fix cycle left a finding nobody verified: the fix stage finishes it",
        rounds: &[(
            Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
            &[("test", "1")],
        )],
        finding: Some("in-unverified"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 2, "attempt": 1, "land": false, "triage": true}"#,
    },
    Position {
        name: "a fix cycle nobody counted left a report and a finding nobody verified",
        rounds: &[(Some(TESTED), &[("test", "1"), ("fix/c1", "1")])],
        finding: Some("in-unverified"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 2, "land": false, "triage": true}"#,
    },
    Position {
        name: "a round that is over with a finding nobody verified: the next round's test takes it",
        rounds: &[(
            Some(
                r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 3, "outcome": "dropped""#,
            ),
            &[("fix/c3", "1")],
        )],
        finding: Some("in-unverified"),
        test: r#"{"round": 2, "attempt": 1}"#,
        fix: r#"{"refused": "round-over", "round": 1}"#,
    },
    Position {
        name: "a tested round that left nothing open",
        rounds: &[(Some(TESTED), &[("test", "1")])],
        finding: Some("in-refuted"),
        test: r#"{"round": 2, "attempt": 1}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
    },
    Position {
        name: "a fix cycle that was started and never recorded: it is run again, once",
        rounds: &[(Some(TESTED), &[("fix/c1", "1")])],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 1, "attempt": 2, "land": false}"#,
    },
    Position {
        name: "a fix cycle that was started twice and never recorded: the next attempt is the human's",
        rounds: &[(Some(TESTED), &[("fix/c1", "1"), ("fix/c1", "2")])],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"refused": "attempts-spent", "round": 1}"#,
    },
    Position {
        name: "one cycle recorded, a blocker left",
        rounds: &[(
            Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
            &[("fix/c1", "2")],
        )],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 2, "attempt": 1, "land": false}"#,
    },
    Position {
        name: "two cycles recorded, the third started once",
        rounds: &[(
            Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 2"#),
            &[("fix/c1", "1"), ("fix/c2", "1"), ("fix/c3", "1")],
        )],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"round": 1, "cycle": 3, "attempt": 2, "land": false}"#,
    },
    Position {
        name: "a cycle recorded and nothing left open: the round lands",
        rounds: &[(
            Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
            &[("fix/c1", "1")],
        )],
        finding: Some("in-fixed"),
        test: r#"{"round": 2, "attempt": 1}"#,
        fix: r#"{"round": 1, "cycle": 2, "attempt": 1, "land": true}"#,
    },
    Position {
        name: "a cycle recorded, nothing left open, and two attempts of the next cycle without a record: the round still lands",
        rounds: &[(
            Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
            &[("fix/c2", "1"), ("fix/c2", "2")],
        )],
        finding: Some("in-fixed"),
        test: r#"{"round": 2, "attempt": 1}"#,
        fix: r#"{"round": 1, "cycle": 2, "attempt": 3, "land": true}"#,
    },
    Position {
        name: "three cycles recorded, a blocker left, and two attempts of the fourth without a record: the bound is said first",
        rounds: &[(
            Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 3"#),
            &[("fix/c4", "1"), ("fix/c4", "2")],
        )],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 1}"#,
        fix: r#"{"refused": "cycle-bound", "round": 1}"#,
    },
    Position {
        name: "a round that was dropped, its blocker open again",
        rounds: &[(
            Some(
                r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 3, "outcome": "dropped""#,
            ),
            &[("fix/c3", "1")],
        )],
        finding: Some("in-confirmed"),
        test: r#"{"round": 2, "attempt": 1}"#,
        fix: r#"{"refused": "round-over", "round": 1}"#,
    },
    Position {
        name: "a round of which a part landed, a blocker left for the next",
        rounds: &[(
            Some(
                r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 4, "outcome": "part""#,
            ),
            &[],
        )],
        finding: Some("in-confirmed"),
        test: r#"{"round": 2, "attempt": 1}"#,
        fix: r#"{"refused": "round-over", "round": 1}"#,
    },
    Position {
        name: "it is the latest round that is asked: round 1 over, round 2 tested",
        rounds: &[
            (
                Some(
                    r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 2, "outcome": "dropped""#,
                ),
                &[("fix/c3", "5")],
            ),
            (Some(TESTED), &[("test", "1")]),
        ],
        finding: Some("in-confirmed"),
        test: r#"{"refused": "round-open", "round": 2}"#,
        fix: r#"{"round": 2, "cycle": 1, "attempt": 1, "land": false}"#,
    },
    Position {
        name: "round 1 landed, round 2 begun and not recorded",
        rounds: &[
            (
                Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
                &[],
            ),
            (None, &[("test", "1")]),
        ],
        finding: Some("in-fixed"),
        test: r#"{"round": 2, "attempt": 2}"#,
        fix: r#"{"refused": "not-tested", "round": 2}"#,
    },
];

/// What a run's opening wrote, for a [`Position`]: its facts, as the inside of a JSON
/// object (empty: none), and whether its clause table has a row.
type Opening = (&'static str, bool);

/// An opening that is done, with a bound no cell of [`POSITIONS`] reaches.
const OPENED: Opening = (r#""stop": "at-the-bound", "rounds": 9"#, true);

/// **The position under the run's facts.** A run that is not ready starts no stage; one
/// more fix round of a run whose bound is spent is the human's to allow, and its `test`
/// stage never is; and round R+1 of a run that stops after every round waits for the
/// human's go.
const BOUND_POSITIONS: &[(Opening, Position)] = &[
    (
        ("", false),
        Position {
            name: "an opening that wrote nothing, and no round",
            rounds: &[],
            finding: None,
            test: r#"{"refused": "not-ready", "round": null}"#,
            fix: r#"{"refused": "not-ready", "round": null}"#,
        },
    ),
    (
        (r#""stop": "every-round""#, false),
        Position {
            name: "no clause row, though a round is tested and a blocker stands",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-confirmed"),
            test: r#"{"refused": "not-ready", "round": 1}"#,
            fix: r#"{"refused": "not-ready", "round": 1}"#,
        },
    ),
    (
        ("", true),
        Position {
            name: "no stop mode, though a round is tested and left nothing open",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-refuted"),
            test: r#"{"refused": "not-ready", "round": 1}"#,
            fix: r#"{"refused": "not-ready", "round": 1}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound""#, true),
        Position {
            name: "a run that stops at its bound, and no bound",
            rounds: &[],
            finding: None,
            test: r#"{"refused": "not-ready", "round": null}"#,
            fix: r#"{"refused": "not-ready", "round": null}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "a bound of one round, and no round yet",
            rounds: &[],
            finding: None,
            test: r#"{"round": 1, "attempt": 1}"#,
            fix: r#"{"refused": "no-round", "round": null}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "at the bound, the round's own test stage not recorded: it is run again",
            rounds: &[(None, &[("test", "1")])],
            finding: None,
            test: r#"{"round": 1, "attempt": 2}"#,
            fix: r#"{"refused": "not-tested", "round": 1}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "the bound spent by a round that landed: its test is always allowed",
            rounds: &[(
                Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
                &[("fix/c1", "1")],
            )],
            finding: Some("in-fixed"),
            test: r#"{"round": 2, "attempt": 1}"#,
            fix: r#"{"round": 1, "cycle": 2, "attempt": 1, "land": true}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "the bound spent by a dropped round: the next round's test is allowed too",
            rounds: &[(
                Some(
                    r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 3, "outcome": "dropped""#,
                ),
                &[],
            )],
            finding: Some("in-confirmed"),
            test: r#"{"round": 2, "attempt": 1}"#,
            fix: r#"{"refused": "round-over", "round": 1}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "the bound spent, the confirming test leaves a blocker: one more fix round is the human's",
            rounds: &[
                (
                    Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
                    &[],
                ),
                (Some(TESTED), &[("test", "1")]),
            ],
            finding: Some("in-confirmed"),
            test: r#"{"refused": "round-open", "round": 2}"#,
            fix: r#"{"refused": "round-bound", "round": 2}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "the bound spent, an item for the human in the confirming round: a ruling is no fix round",
            rounds: &[
                (
                    Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
                    &[],
                ),
                (Some(TESTED), &[("test", "1")]),
            ],
            finding: Some("out-confirmed"),
            test: r#"{"refused": "round-open", "round": 2}"#,
            fix: r#"{"round": 2, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "the bound spent, a finding nobody verified in the confirming round: its triage is finished",
            rounds: &[
                (
                    Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
                    &[],
                ),
                (Some(TESTED), &[("test", "1")]),
            ],
            finding: Some("in-unverified"),
            test: r#"{"round": 2, "attempt": 2, "triage": true}"#,
            fix: r#"{"round": 2, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "the bound spent in this very round: its own fix cycles go on",
            rounds: &[(
                Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 1"#),
                &[("fix/c1", "1")],
            )],
            finding: Some("in-confirmed"),
            test: r#"{"refused": "round-open", "round": 1}"#,
            fix: r#"{"round": 1, "cycle": 2, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "a bound of one, and no fix round yet: the first is allowed",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-confirmed"),
            test: r#"{"refused": "round-open", "round": 1}"#,
            fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 1"#, true),
        Position {
            name: "a bound of one, a round that left nothing open: a further test is no fix round",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-refuted"),
            test: r#"{"round": 2, "attempt": 1}"#,
            fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "at-the-bound", "rounds": 2"#, true),
        Position {
            name: "under the bound, a round that left nothing open",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-refuted"),
            test: r#"{"round": 2, "attempt": 1}"#,
            fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "every-round", "rounds": 1"#, true),
        Position {
            name: "every round a stop, a round that left nothing open: the next waits for the go",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-refuted"),
            test: r#"{"refused": "stopped", "round": 1}"#,
            fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
    (
        (r#""stop": "every-round""#, true),
        Position {
            name: "every round a stop, a dropped round: no test before the human's go",
            rounds: &[(
                Some(
                    r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 3, "outcome": "dropped""#,
                ),
                &[],
            )],
            finding: Some("in-confirmed"),
            test: r#"{"refused": "stopped", "round": 1}"#,
            fix: r#"{"refused": "round-over", "round": 1}"#,
        },
    ),
    (
        (r#""stop": "every-round""#, true),
        Position {
            name: "every round a stop, a round whose last cycle left nothing open: it still lands",
            rounds: &[(
                Some(r#""candidate": "cccccccccccccccccccccccccccccccccccccccc", "cycles": 2"#),
                &[("fix/c2", "1")],
            )],
            finding: Some("in-fixed"),
            test: r#"{"refused": "stopped", "round": 1}"#,
            fix: r#"{"round": 1, "cycle": 3, "attempt": 1, "land": true}"#,
        },
    ),
    (
        (r#""stop": "every-round""#, true),
        Position {
            name: "every round a stop, and the round is not over: no stop inside a round",
            rounds: &[(Some(TESTED), &[("test", "1")])],
            finding: Some("in-unverified"),
            test: r#"{"round": 1, "attempt": 2, "triage": true}"#,
            fix: r#"{"round": 1, "cycle": 1, "attempt": 1, "land": false}"#,
        },
    ),
];

/// A round's facts, written as the stages write them: what its `test` stage records — the
/// candidate, and what is written with it — and then, in a call of its own, what a fix
/// stage or the human's exit adds to a tested round. The script takes each only there.
fn write_round(rig: &Rig, round: &str, facts: &str) {
    let given: Value = serde_json::from_str(facts).expect("a round's facts");
    let (mut tested, mut after) = (json!({}), json!({}));
    for (name, value) in given.as_object().expect("an object") {
        if ["cycles", "outcome"].contains(&name.as_str()) {
            after[name] = value.clone();
        } else {
            tested[name] = value.clone();
        }
    }
    for (facts, what) in [
        (tested, "the round's record"),
        (after, "what a fix stage left in it"),
    ] {
        if facts.as_object().is_some_and(|facts| !facts.is_empty()) {
            rig.run(&round_set(RUN, round), &facts.to_string())
                .must(OK, what);
        }
    }
}

fn drive_position(label: &str, opening: Opening, position: &Position, truth: &[Finding]) {
    let rig = Rig::new(label);
    let (facts, clause_row) = opening;
    if !facts.is_empty() {
        rig.run(&run_set(RUN), &format!("{{{facts}, {RELEASE}}}"))
            .must(OK, "the run's facts");
    }
    if clause_row {
        rig.run(
            &run_set(RUN),
            &json!({"clauses": ["no-lost-files"]}).to_string(),
        )
        .must(OK, "the census: a clause, not yet run");
    }
    match position.finding {
        // Before any round nothing was triaged: the finding is a row of the opening's.
        Some(key) if position.rounds.is_empty() => rig.seed_rows(&[key]),
        Some(key) => {
            ground(&rig);
            let finding = truth
                .iter()
                .find(|finding| finding.key == key)
                .unwrap_or_else(|| panic!("`{key}` is a finding of the table"));
            plant(&rig, &[finding]);
        }
        None => {}
    }
    for (n, (facts, reports)) in position.rounds.iter().enumerate() {
        // A round has a directory before it has anything else.
        let round = (n + 1).to_string();
        fs::create_dir_all(rig.run_dir().join(format!("r{round}"))).expect("a round directory");
        if let Some(facts) = facts {
            // A round is tested over its doors: its record names a candidate once it has
            // a scope — which round 1 has already, where a finding was placed by it.
            if facts.contains("candidate")
                && !rig.run_dir().join(format!("r{round}/scope.md")).exists()
            {
                rig.run(
                    &scope_set(RUN, &round),
                    &scope(&[INSIDE], &[EXCLUDED]).to_string(),
                )
                .must(OK, "the round's scope");
            }
            write_round(&rig, &round, &format!("{{{facts}}}"));
        }
        for (stage, attempt) in *reports {
            let mut args = strings(&["report", "--reporter=preflight"]);
            args.extend([
                format!("--run={RUN}"),
                format!("--round={round}"),
                format!("--attempt={attempt}"),
            ]);
            match stage.strip_prefix("fix/c") {
                Some(cycle) => args.extend(["--stage=fix".to_owned(), format!("--cycle={cycle}")]),
                None => args.push("--stage=test".to_owned()),
            }
            rig.run(&args, BODY).must(OK, "a report");
        }
    }
    let read = rig.state();
    let what = format!("`{}`", position.name);
    let parsed = |text: &str| -> Value { serde_json::from_str(text).expect("a JSON cell") };
    let ready = !facts.contains("at-the-bound") || facts.contains("rounds");
    assert_eq!(
        read["not_ready"] == json!([]),
        clause_row && !facts.is_empty() && ready,
        "{what}: {read}"
    );
    assert_eq!(
        read["position"],
        json!({"test": parsed(position.test), "fix": parsed(position.fix)}),
        "{what}: {read}"
    );
}

/// Drive `cells` — each an opening and a position — a few rigs to a thread.
fn drive_positions(label: &str, cells: &[(Opening, &Position)]) -> usize {
    let truth = findings();
    let truth = &truth;
    std::thread::scope(|threads| {
        let workers: Vec<_> = cells
            .chunks(2)
            .enumerate()
            .map(|(n, positions)| {
                threads.spawn(move || {
                    for (m, (opening, position)) in positions.iter().enumerate() {
                        drive_position(&format!("{label}-{n}-{m}"), *opening, position, truth);
                    }
                    positions.len()
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().expect("a worker thread"))
            .sum()
    })
}

#[test]
fn the_position_of_each_stage_is_computed_from_the_rounds_records_and_the_routes() {
    let cells: Vec<(Opening, &Position)> = POSITIONS
        .iter()
        .map(|position| (OPENED, position))
        .collect();
    let driven = drive_positions("position", &cells);
    assert!(driven >= 24, "the table collapsed to {driven} cells");
}

#[test]
fn a_run_that_is_not_ready_starts_no_stage_and_the_bound_and_the_stop_are_the_humans() {
    let cells: Vec<(Opening, &Position)> = BOUND_POSITIONS
        .iter()
        .map(|(opening, position)| (*opening, position))
        .collect();
    let driven = drive_positions("bound-position", &cells);
    assert!(driven >= 18, "the table collapsed to {driven} cells");
}

// ---------------------------------------------------------------------------
// 22 · The run's facts: what the opening settled for every stage to read
// ---------------------------------------------------------------------------

/// The opening record is the human-led step's prose, which the script neither writes nor
/// reads. What a stage must read of it — when the run stops of its own accord, the bound
/// across rounds, the previous release and the default scope — gets in through `run-set`;
/// the bound is only ever raised, and the release a run measures against is written once.
#[test]
fn the_runs_facts_are_written_through_the_script_and_the_bound_is_only_raised() {
    let rig = Rig::new("run-facts");
    let facts = |stop: Value, rounds: Value, previous: Value, commit: Value, scope: Value| {
        json!({
            "stop": stop,
            "rounds": rounds,
            "cycles": null,
            "previous": previous,
            "previous-commit": commit,
            "scope": scope,
            "clauses": null,
            "findings": null,
        })
    };
    let none = Value::Null;
    assert_eq!(
        rig.state()["facts"],
        facts(
            none.clone(),
            none.clone(),
            none.clone(),
            none.clone(),
            none.clone()
        ),
        "an opening that wrote none"
    );
    let seen = rig.run(&run_set(RUN), &json!({"rounds": 3}).to_string());
    seen.must(OK, "the bound across rounds");
    assert_eq!(
        seen.json(),
        json!({
            "run": run_path(),
            "facts": facts(none.clone(), json!(3), none.clone(), none.clone(), none.clone()),
        })
    );
    rig.run(&run_set(RUN), &json!({"stop": "at-the-bound"}).to_string())
        .must(OK, "the stop mode");
    assert_eq!(
        table(&rig.read(&run_path()), &ROUND_COLUMNS),
        vec![vec!["`stop`", "`at-the-bound`"], vec!["`rounds`", "`3`"]],
        "the facts, in the table's own order whatever order they came in"
    );
    assert_eq!(
        rig.state()["not_ready"],
        json!(["no-clause-row", "no-previous-release", "no-default-scope"]),
        "the stop mode and its bound are not all an opening owes"
    );

    // The previous release — its version, and the commit it was released from — and the
    // default scope. Half a release is none.
    rig.run(
        &run_set(RUN),
        &json!({"scope": "delta", "previous": "1.0.0-rc.24"}).to_string(),
    )
    .must(OK, "the default scope, and the previous release's version");
    assert_eq!(
        rig.state()["not_ready"],
        json!(["no-clause-row", "no-previous-release"]),
        "a version with no commit is no release to build"
    );
    rig.run(
        &run_set(RUN),
        &json!({"previous-commit": sha('e')}).to_string(),
    )
    .must(OK, "the commit it was released from");
    assert_eq!(
        table(&rig.read(&run_path()), &ROUND_COLUMNS),
        vec![
            vec!["`stop`".to_owned(), "`at-the-bound`".to_owned()],
            vec!["`rounds`".to_owned(), "`3`".to_owned()],
            vec!["`previous`".to_owned(), "`1.0.0-rc.24`".to_owned()],
            vec!["`previous-commit`".to_owned(), format!("`{}`", sha('e'))],
            vec!["`scope`".to_owned(), "`delta`".to_owned()],
        ]
    );
    assert_eq!(rig.state()["not_ready"], json!(["no-clause-row"]));

    // While the opening is being written the bound may still go up, and a raise is the
    // same call; a bound said again stands, and so does every other fact said again as it
    // is. (Once a round is begun, a bound is raised only at its own stop: the writers'
    // table, below.)
    for rounds in [5, 5] {
        rig.run(&run_set(RUN), &json!({"rounds": rounds}).to_string())
            .must(OK, "the bound, raised and said again");
    }
    rig.run(
        &run_set(RUN),
        &json!({
            "stop": "at-the-bound",
            "rounds": 12,
            "scope": "delta",
            "previous": "1.0.0-rc.24",
            "previous-commit": sha('e'),
        })
        .to_string(),
    )
    .must(
        OK,
        "the mode, the scope and the release again, and a higher bound",
    );
    assert_eq!(
        rig.state()["facts"],
        facts(
            json!("at-the-bound"),
            json!(12),
            json!("1.0.0-rc.24"),
            json!(sha('e')),
            json!("delta")
        )
    );

    let before = rig.snapshot();
    let stands: &[(Value, &str)] = &[
        (
            json!({"previous": "1.0.0-rc.23"}),
            "another previous release",
        ),
        (
            json!({"previous-commit": sha('f')}),
            "another commit for the previous release",
        ),
        (
            json!({"rounds": 13, "previous": "1.0.0"}),
            "another release beside a fact that would be taken",
        ),
        // WHEN A RUN STOPS IS WRITTEN ONCE (the review's `F7`, its third way): the mode
        // rewritten would take a stop at a spent bound away without the bound being raised.
        (json!({"stop": "every-round"}), "another stop mode"),
        (json!({"scope": "everything"}), "another default scope"),
    ];
    for (given, why) in stands {
        rig.run(&run_set(RUN), &given.to_string())
            .refused(EXISTS, why);
    }
    let bad: &[(Value, &str)] = &[
        (json!({"rounds": 11}), "a bound that goes down"),
        (
            json!({"stop": "at-the-bound", "rounds": 3}),
            "a bound that goes down beside a fact that would be taken",
        ),
        (json!({"rounds": 0}), "no round at all"),
        (json!({"rounds": "13"}), "a bound that is text"),
        (json!({"rounds": 12.5}), "half a round"),
        (json!({"rounds": true}), "a bound that is a truth value"),
        (json!({"stop": "sometimes"}), "a mode nobody defined"),
        (json!({"stop": true}), "a mode that is a truth value"),
        (json!({"laps": 3}), "a fact nobody defined"),
        (
            json!({"cycles": 2}),
            "a bound on a round's fix cycles below the three it starts at",
        ),
        (
            json!({"findings": ["audit-f3"]}),
            "the ledger's census, which the script counts and no caller writes",
        ),
        (
            json!({"stop": "every-round", "release": "1.0.0"}),
            "a fact nobody defined beside one that would be taken",
        ),
        (
            json!({"previous": "the last one"}),
            "a release that is prose",
        ),
        (json!({"previous": "v1.0.0"}), "a release that is a tag"),
        (json!({"previous": 1}), "a release that is a number"),
        (
            json!({"previous-commit": "0f34d8f0"}),
            "a release's commit cut short",
        ),
        (
            json!({"previous-commit": "HEAD~3"}),
            "a release's commit that is no sha",
        ),
        (json!({"scope": "the delta"}), "a scope that is prose"),
        (
            json!({"scope": ["jigc setup"]}),
            "a scope that is a list of doors",
        ),
        (json!({"scope": true}), "a scope that is a truth value"),
        (json!({}), "facts that say nothing"),
        (json!([{"rounds": 13}]), "a list of facts"),
    ];
    for (given, why) in bad {
        rig.run(&run_set(RUN), &given.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before, "a refused fact writes nothing");

    // Facts somebody wrote by hand are not rewritten on a guess.
    let other = Rig::new("run-facts-by-hand");
    fs::write(
        other.root.join(run_path()),
        "# the run\n\nThree rounds, then stop.\n",
    )
    .expect("a note");
    let before = other.snapshot();
    other
        .run(&run_set(RUN), &json!({"rounds": 3}).to_string())
        .refused(CORRUPT, "facts that are not the script's");
    other
        .run(&state(RUN), "")
        .refused(CORRUPT, "the state over facts that are not the script's");
    assert_eq!(other.snapshot(), before);
}

// ---------------------------------------------------------------------------
// The one write: a batch lands whole or not at all, and a killed one is seen
// ---------------------------------------------------------------------------

/// A rig with a finding triaged once: a ledger and a triage record that both exist, so that
/// a second triage of it rewrites two files.
fn triaged_once(label: &str) -> Rig {
    let rig = Rig::new(label);
    rig.seed_rows(&["audit-f3"]);
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&["jigc setup"], &[]).to_string(),
    )
    .must(OK, "the round's scope");
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "audit-f3", "grade": "breaks", "verdict": "refuted"}).to_string(),
    )
    .must(OK, "the first triage");
    rig
}

fn found_again() -> String {
    json!({"key": "audit-f3", "grade": "breaks", "verdict": "confirmed", "regression": true})
        .to_string()
}

/// The ledger's grade of the one finding, and the round's triage row of it.
fn two_files(rig: &Rig) -> (String, String) {
    let ledger = table(&rig.read(&ledger_path()), &LEDGER_COLUMNS);
    let triage = table(&rig.read(&triage_path("1")), &TRIAGE_COLUMNS);
    (ledger[0][6].clone(), triage[0][3].clone())
}

/// State 11 of the repair's truth table, the state-machine review's `F4`: `triage-set`
/// writes two files, and the scanner answers for the first and cannot run for the second.
/// It said *nothing was written* with the triage record written — and `state` then read
/// the stale grade without a word.
#[test]
fn a_call_that_writes_two_files_writes_both_or_neither() {
    let mut rig = triaged_once("two-files");
    let before = rig.snapshot();
    let read = rig.state();
    rig.env.push(("STUB_GITLEAKS", "crash".to_owned()));
    rig.run(&triage_set(RUN, "1"), &found_again()).refused(
        DID_NOT_RUN,
        "a scanner that cannot run, under a call that writes more than one file",
    );
    assert_eq!(
        rig.snapshot(),
        before,
        "the refusal says that nothing was written, and nothing was"
    );
    rig.env.pop();
    assert_eq!(rig.state(), read, "and the state is the state it was");

    // A CALL'S FILES ARE SCANNED AS ONE BATCH, before any of them is written: a scanner
    // that answers once and can never run again scans the whole call. (The review's `F4`
    // had it die between the call's two files — which no call can meet any more: there is
    // one scan, however many files the call writes.)
    rig.env.push(("STUB_GITLEAKS", "once".to_owned()));
    rig.run(&triage_set(RUN, "1"), &found_again())
        .must(OK, "the triage: every file of it, in one scan");
    assert_eq!(
        two_files(&rig),
        ("regression".to_owned(), "confirmed".to_owned()),
        "the ledger's grade and the triage record's verdict, from one entry"
    );
    assert!(
        !rig.snapshot()
            .keys()
            .any(|path| path.ends_with(".pending.json")),
        "a batch that landed leaves no journal"
    );
    let written = rig.snapshot();
    rig.run(&triage_set(RUN, "1"), &found_again())
        .refused(DID_NOT_RUN, "the next call, whose scan cannot run");
    assert_eq!(rig.snapshot(), written, "and that call wrote nothing");
}

/// A stand-in for a kill at a point no signal can be timed to: the interpreter is handed a
/// `sitecustomize` that kills the script — or raises in it — at its Nth `os.replace`. The
/// script makes one to write its journal and one per table it puts in place.
const KILL_SWITCH: &str = r#"import os

_kill = int(os.environ.get("STUB_KILL_AT_REPLACE", "0"))
_fault = int(os.environ.get("STUB_FAULT_AT_REPLACE", "0"))
_real = os.replace
_seen = [0]


def _replace(*args, **kwargs):
    _seen[0] += 1
    if _seen[0] == _kill:
        os.kill(os.getpid(), 9)
    if _seen[0] == _fault:
        raise RuntimeError("a fault nobody foresaw")
    return _real(*args, **kwargs)


os.replace = _replace
"#;

fn with_kill_switch(rig: &mut Rig, name: &'static str, at: u32) {
    let hooks = rig.tmp.join("hooks");
    fs::create_dir_all(&hooks).expect("create the hook directory");
    fs::write(hooks.join("sitecustomize.py"), KILL_SWITCH).expect("write the kill switch");
    rig.env
        .push(("PYTHONPATH", hooks.to_string_lossy().into_owned()));
    rig.env.push((name, at.to_string()));
}

fn temporaries(rig: &Rig) -> Vec<String> {
    rig.snapshot()
        .into_keys()
        .filter(|path| path.ends_with(".tmp") || path.ends_with(".pending.json"))
        .collect()
}

/// A call killed between its two files leaves an INTERRUPTED write: the state is not read
/// across it, and the next writer finishes the batch before it does anything else. Killed
/// before its journal is written, it leaves no file of the batch, and the next writer
/// removes what it left.
#[test]
fn a_write_killed_between_its_files_is_never_read_as_a_finished_one() {
    // (1) Killed with the first file in place and the second not.
    let mut rig = triaged_once("killed-between");
    let read = rig.state();
    with_kill_switch(&mut rig, "STUB_KILL_AT_REPLACE", 3);
    let seen = rig.run(&triage_set(RUN, "1"), &found_again());
    assert_eq!((seen.code, seen.signal), (None, Some(9)), "{seen:?}");
    rig.env.truncate(0);
    assert_eq!(
        two_files(&rig),
        ("refuted".to_owned(), "confirmed".to_owned()),
        "the triage record is written and the ledger is not: half a batch"
    );
    let refused = rig.run(&state(RUN), "");
    refused.refused(INTERRUPTED, "the state over half a batch");
    assert!(
        refused.stderr.contains("recover --run rc24"),
        "the refusal names what finishes it: {}",
        refused.stderr
    );
    let pending = rig.run(&keeper("pending", RUN), "");
    pending.must(OK, "pending");
    assert_eq!(pending.json()["interrupted"], true);
    let recovered = rig.run(&keeper("recover", RUN), "");
    recovered.must(OK, "recover");
    assert_eq!(
        recovered.json()["recovered"],
        json!([triage_path("1"), ledger_path()])
    );
    assert_eq!(
        two_files(&rig),
        ("regression".to_owned(), "confirmed".to_owned()),
        "the batch is whole"
    );
    assert_eq!(temporaries(&rig), Vec::<String>::new());
    assert_ne!(rig.state(), read, "and the state reads what it says");

    // (2) Killed with the journal written and no file in place: the next WRITER — any
    // writer — finishes the batch first.
    let mut rig = triaged_once("killed-journaled");
    with_kill_switch(&mut rig, "STUB_KILL_AT_REPLACE", 2);
    let seen = rig.run(&triage_set(RUN, "1"), &found_again());
    assert_eq!((seen.code, seen.signal), (None, Some(9)), "{seen:?}");
    rig.env.truncate(0);
    assert_eq!(
        two_files(&rig),
        ("refuted".to_owned(), "refuted".to_owned())
    );
    rig.run(&state(RUN), "").refused(
        INTERRUPTED,
        "the state over a batch no file of which landed",
    );
    rig.run(&bound_set(RUN, "non-jigc-writer", &BOUND), "")
        .must(OK, "the next writer");
    assert_eq!(
        two_files(&rig),
        ("regression".to_owned(), "confirmed".to_owned()),
        "it finished the batch before its own write"
    );
    assert_eq!(temporaries(&rig), Vec::<String>::new());
    rig.state();

    // (3) Killed before the journal: no file of the batch, and what it left is removed.
    let mut rig = triaged_once("killed-unjournaled");
    let before = rig.snapshot();
    let read = rig.state();
    with_kill_switch(&mut rig, "STUB_KILL_AT_REPLACE", 1);
    let seen = rig.run(&triage_set(RUN, "1"), &found_again());
    assert_eq!((seen.code, seen.signal), (None, Some(9)), "{seen:?}");
    rig.env.truncate(0);
    assert_eq!(rig.state(), read, "no file of the batch is in place");
    assert_eq!(
        temporaries(&rig).len(),
        3,
        "its temporaries are what is left"
    );
    rig.run(&keeper("recover", RUN), "").must(OK, "recover");
    assert_eq!(rig.snapshot(), before, "and they are removed");

    // A journal somebody wrote by hand is not read on a guess.
    fs::write(rig.run_dir().join(".pending.json"), "{\"placing\": 1}\n").expect("a journal");
    for name in ["state", "pending", "recover", "discard", "settle"] {
        rig.run(&keeper(name, RUN), "").refused(
            CORRUPT,
            &format!("`{name}` over a journal that is not the script's"),
        );
    }
}

// ---------------------------------------------------------------------------
// A record step's writes, as one batch
// ---------------------------------------------------------------------------

fn call(argv: Vec<String>, stdin: &str) -> Value {
    json!({"argv": argv, "stdin": stdin})
}

/// The calls of a small `test` stage's record: the candidate's gate, a row, its triage, an
/// item's result, the round's facts, and the ledger checked.
fn stage_batch(rig: &Rig) -> Value {
    let summary = rig.summary("candidate", &[], &[]);
    json!([
        call(gate_set(RUN, "1", &sha('a'), &summary), ""),
        call(ledger_add(RUN), &row("audit-f4").to_string()),
        call(
            triage_set(RUN, "1"),
            &json!([{"key": "audit-f4", "grade": "no-break"}]).to_string()
        ),
        call(
            result_set(RUN, "1", &sha('a')),
            &ran("row-setup", "green").to_string()
        ),
        call(
            round_set(RUN, "1"),
            &json!({"candidate": sha('a')}).to_string()
        ),
        call(check_ledger(RUN, &["audit-f3", "audit-f4"]), ""),
    ])
}

#[test]
fn a_record_steps_batch_is_written_whole_held_as_pending_and_can_be_taken_back() {
    let rig = triaged_once("apply");
    rig.run(
        &run_set(RUN),
        &json!({"clauses": ["no-lost-files"]}).to_string(),
    )
    .must(OK, "the census");
    rig.run(&item_set(RUN), &item("row-setup").to_string())
        .must(OK, "the item the batch records a result of");
    let before = rig.snapshot();
    let read = rig.state();

    // A BATCH THAT IS REFUSED WRITES NOTHING — whichever call of it is refused, and a
    // check that does not hold refuses it too.
    let mut batch = stage_batch(&rig);
    batch[5] = call(check_ledger(RUN, &["audit-f3", "no-such-row"]), "");
    let refused = rig.run(&apply(RUN, Some("1")), &batch.to_string());
    refused.refused(CHECK, "a batch whose check does not hold");
    assert!(
        refused
            .stderr
            .contains("call 6 of the batch (`check-ledger`)")
            && refused.stderr.contains("\"missing\": [\"no-such-row\"]"),
        "it names the call and what the check found: {}",
        refused.stderr
    );
    let mut batch = stage_batch(&rig);
    batch[2] = call(
        triage_set(RUN, "1"),
        &json!([{"key": "audit-f9", "grade": "no-break"}]).to_string(),
    );
    let refused = rig.run(&apply(RUN, Some("1")), &batch.to_string());
    refused.refused(NO_SUCH_ROW, "a batch whose third call is refused");
    assert!(
        refused
            .stderr
            .contains("call 3 of the batch (`triage-set`)"),
        "{}",
        refused.stderr
    );
    for (given, why) in [
        (json!([]), "a batch of no call"),
        (json!({"argv": ["ledger-add"]}), "a batch that is no list"),
        (
            json!([{"argv": "ledger-add"}]),
            "a call whose argv is no list",
        ),
        (
            json!([{"argv": ["state", "--run=rc24"]}]),
            "a call that is no writer of a table",
        ),
        (
            json!([{"argv": ["apply", "--run=rc24", "--subject=x"]}]),
            "a batch inside a batch",
        ),
        (
            json!([{"argv": ["report", "--run=rc24"]}]),
            "a report inside a batch",
        ),
        (
            json!([{"argv": ["ledger-add", "--run=other"], "stdin": row("x").to_string()}]),
            "a call for another run",
        ),
        (
            json!([{"argv": ["ledger-add", "--run=rc24"], "stdin": 3}]),
            "an input that is no text",
        ),
        (
            json!([{"argv": ["ledger-add", "--run=rc24"], "more": 1}]),
            "a field nobody defined",
        ),
    ] {
        rig.run(&apply(RUN, Some("1")), &given.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before, "no refused batch wrote anything");
    assert_eq!(rig.state(), read);

    // A BATCH THAT IS TAKEN: one result per call, every file written, and the journal
    // holds it as the applied batch.
    let applied = rig.run(&apply(RUN, Some("1")), &stage_batch(&rig).to_string());
    applied.must(OK, "the batch");
    let said = applied.json();
    assert_eq!(said["applied"], 6);
    assert_eq!(said["results"].as_array().map(Vec::len), Some(6));
    assert_eq!(
        said["results"][5],
        json!({"check": "ledger", "ok": true, "missing": [], "duplicated": []})
    );
    // The clause table is one of them: the round's candidate made the item's run count,
    // and the writer that recorded it rendered the view in the same batch.
    let files = json!([
        gate_path("1"),
        ledger_path(),
        run_path(),
        triage_path("1"),
        results_path("1"),
        round_path("1"),
        clauses_path()
    ]);
    assert_eq!(said["files"], files);
    let pending = rig.run(&keeper("pending", RUN), "").json();
    assert_eq!(pending["interrupted"], false);
    assert_eq!(
        json!([
            pending["batch"]["round"],
            pending["batch"]["subject"],
            pending["batch"]["calls"],
            pending["batch"]["checks"]
        ]),
        json!([1, "docs(record): a record step", 6,
               [{"check": "ledger", "ok": true, "missing": [], "duplicated": []}]])
    );
    let held: Vec<&String> = pending["batch"]["files"]
        .as_object()
        .expect("the batch's files")
        .keys()
        .collect();
    assert_eq!(held.len(), 7, "{pending}");
    assert_eq!(pending["once"], json!([scope_path("1")]));
    let state = rig.state();
    assert_eq!(state["rounds"][0]["facts"]["candidate"], sha('a').as_str());
    assert_eq!(
        (
            &state["clauses"][0]["status"],
            &read["clauses"][0]["status"]
        ),
        (&json!("green"), &json!("void")),
        "the batch turned the clause green, in the table and in the state"
    );
    assert_eq!(state["ledger"].as_array().map(Vec::len), Some(2));

    // ONE APPLIED BATCH AT A TIME.
    let written = rig.snapshot();
    rig.run(&apply(RUN, Some("1")), &stage_batch(&rig).to_string())
        .refused(PENDING, "a second batch over one that no commit holds");
    assert_eq!(rig.snapshot(), written);

    // TAKEN BACK: every file as it was, the made ones gone, the journal with them — and
    // what is no part of a batch, the round's scope, stays.
    let discarded = rig.run(&keeper("discard", RUN), "");
    discarded.must(OK, "discard");
    assert_eq!(discarded.json()["discarded"], files);
    assert_eq!(
        rig.snapshot(),
        before,
        "the tree before the batch, byte for byte"
    );
    assert_eq!(rig.state(), read);
    assert_eq!(
        rig.run(&keeper("discard", RUN), "").json()["discarded"],
        json!([]),
        "nothing to take back is an answer"
    );

    // COMMITTED: the step that made the commit says so, and the batch is forgotten with
    // its files where they are.
    rig.run(&apply(RUN, Some("1")), &stage_batch(&rig).to_string())
        .must(OK, "the batch again");
    let settled = rig.run(&keeper("settle", RUN), "");
    settled.must(OK, "settle");
    assert_eq!(settled.json()["settled"].as_array().map(Vec::len), Some(7));
    assert_eq!(
        rig.run(&keeper("pending", RUN), "").json()["batch"],
        Value::Null
    );
    assert_eq!(rig.state(), state);
    assert_eq!(temporaries(&rig), Vec::<String>::new());

    // A run that was never opened has nothing pending, and says so as `state` does.
    let nobody = rig.run(&keeper("pending", "no-such-run"), "").json();
    assert_eq!(
        json!([nobody["opened"], nobody["batch"], nobody["once"]]),
        json!([false, null, []])
    );
}

// ---------------------------------------------------------------------------
// The candidate's gate, and a record commit's gate held to it
// ---------------------------------------------------------------------------

#[test]
fn a_record_commits_gate_is_held_to_what_the_candidates_gate_showed_red() {
    let a = "jigc::g_a suite::one";
    let b = "jigc::g_a suite::two";
    let c = "jigc::g_b other::new";
    struct Row {
        why: &'static str,
        candidate: Option<(&'static [&'static str], Vec<&'static str>)>,
        record: (&'static [&'static str], Vec<&'static str>),
        new: Vec<String>,
    }
    let rows = vec![
        Row {
            why: "a green candidate, a green record gate: the rule as it was",
            candidate: Some((&[], vec![])),
            record: (&[], vec![]),
            new: vec![],
        },
        Row {
            why: "a green candidate, a red record gate",
            candidate: Some((&[], vec![])),
            record: (&["tier1"], vec![a]),
            new: vec!["step tier1".into(), format!("test {a}")],
        },
        Row {
            why: "a red candidate, the same red",
            candidate: Some((&["tier1"], vec![a, b])),
            record: (&["tier1"], vec![a, b]),
            new: vec![],
        },
        Row {
            why: "a red candidate, less red",
            candidate: Some((&["tier1"], vec![a, b])),
            record: (&["tier1"], vec![b]),
            new: vec![],
        },
        Row {
            why: "a red candidate, and a test red only with the records",
            candidate: Some((&["tier1"], vec![a])),
            record: (&["tier1"], vec![a, c]),
            new: vec![format!("test {c}")],
        },
        Row {
            why: "a candidate red in the second tier, a record gate red in the first",
            candidate: Some((&["tier2"], vec![a])),
            record: (&["tier1"], vec![a]),
            new: vec!["step tier1".into()],
        },
        Row {
            why: "a candidate whose lint is red, and no test ran on either",
            candidate: Some((&["clippy"], vec![])),
            record: (&["clippy"], vec![]),
            new: vec![],
        },
        Row {
            why: "a red candidate, and a lint red only with the records",
            candidate: Some((&["tier1"], vec![a])),
            record: (&["fmt", "clippy"], vec![]),
            new: vec!["step fmt".into(), "step clippy".into()],
        },
        Row {
            why: "a test step that failed and names no test",
            candidate: Some((&["tier1"], vec![a])),
            record: (&["tier1"], vec![]),
            new: vec!["step tier1 (it names no failing test)".into()],
        },
        Row {
            why: "no candidate's gate on record: nothing may be red",
            candidate: None,
            record: (&["tier1"], vec![a]),
            new: vec!["step tier1".into(), format!("test {a}")],
        },
        Row {
            why: "no candidate's gate on record, and a green gate",
            candidate: None,
            record: (&[], vec![]),
            new: vec![],
        },
    ];
    for (n, row) in rows.iter().enumerate() {
        let rig = Rig::new(&format!("gate-{n}"));
        if let Some((steps, tests)) = &row.candidate {
            let set = rig.run(
                &gate_set(RUN, "1", &sha('a'), &rig.summary("candidate", steps, tests)),
                "",
            );
            set.must(OK, row.why);
            assert_eq!(
                set.json(),
                json!({"gate": gate_path("1"), "commit": sha('a'),
                       "verdict": if steps.is_empty() { "pass" } else { "fail" },
                       "red": steps.len() + tests.len()}),
                "{}",
                row.why
            );
        }
        let seen = rig.run(
            &gate_check(
                RUN,
                Some("1"),
                &rig.summary("record", row.record.0, &row.record.1),
            ),
            "",
        );
        seen.must(
            if row.new.is_empty() {
                OK
            } else {
                GATE_MISMATCH
            },
            row.why,
        );
        let said = seen.json();
        assert_eq!(said["check"], "gate", "{}", row.why);
        assert_eq!(said["ok"], row.new.is_empty(), "{}: {said}", row.why);
        assert_eq!(said["new"], json!(row.new), "{}: {said}", row.why);
        assert_eq!(
            said["candidate"],
            if row.candidate.is_some() {
                json!(sha('a'))
            } else {
                Value::Null
            },
            "{}",
            row.why
        );
    }

    // THE TABLE, as it is written: the commit, the verdict, the gate's own totals line,
    // and a row per step and per test it showed red.
    let rig = Rig::new("gate-table");
    let hostile = "jigc::g_a a|b `c` \\| d";
    rig.run(
        &gate_set(
            RUN,
            "1",
            &sha('b'),
            &rig.summary("candidate", &["tier1", "doctest"], &[a, hostile]),
        ),
        "",
    )
    .must(OK, "the candidate's gate");
    let rows = table(&rig.read(&gate_path("1")), &["what", "value"]);
    assert_eq!(rows[0], ["commit", &format!("`{}`", sha('b'))]);
    assert_eq!(rows[1], ["verdict", "fail"]);
    assert_eq!(
        rows[2],
        [
            "totals",
            "`tests   passed=4700 failed=2  (over 15 test binaries)`"
        ]
    );
    assert_eq!(rows[3..5], [["step", "`tier1`"], ["step", "`doctest`"]]);
    assert_eq!(rows.len(), 7, "{rows:?}");
    // A name no cell could hold as it is still compares as itself.
    rig.run(
        &gate_check(
            RUN,
            Some("1"),
            &rig.summary("record", &["tier1"], &[hostile]),
        ),
        "",
    )
    .must(OK, "a hostile test name, red on both");
    // Without a round, nothing may be red — whatever a round holds.
    rig.run(
        &gate_check(RUN, None, &rig.summary("record", &["tier1"], &[a])),
        "",
    )
    .must(GATE_MISMATCH, "a record commit that names no round");
    // A round tests one candidate; the same candidate's gate may be read again.
    let before = rig.snapshot();
    rig.run(
        &gate_set(RUN, "1", &sha('c'), &rig.summary("candidate", &[], &[])),
        "",
    )
    .refused(EXISTS, "the gate of another commit for the same round");
    assert_eq!(rig.snapshot(), before);
    rig.run(
        &gate_set(RUN, "1", &sha('b'), &rig.summary("candidate", &[], &[])),
        "",
    )
    .must(OK, "the same candidate's gate, run again");

    // WHAT IS NOT ONE FULL GATE'S OUTPUT is evidence about no tree.
    let before = rig.snapshot();
    let green = gate_output(&[], &[]);
    for (text, why) in [
        ("fast tier  12 passed, 0 failed -- the second tier and the doctests were not run\n\nPRE-CHECK: PASS -- fmt, clippy, build and the fast test tier. NOT a gate: never commit on it\n".to_owned(), "a pre-check"),
        ("==> fmt     ok        (1s)\n\nGATE: PASS\n".to_owned(), "a gate without its tests: no totals line"),
        (format!("{green}{green}"), "two gates in one file"),
        (green.replace("GATE: PASS", "  GATE: PASS"), "a verdict that is quoted, not printed"),
        (green.replace("failed=0", "failed=2"), "a green verdict over failed tests"),
        (green.replace("\nGATE: PASS\n", "\n"), "a gate cut off before its verdict"),
        (String::new(), "nothing"),
    ] {
        let file = rig.summary_of("bad", &text);
        rig.run(&gate_check(RUN, Some("1"), &file), "")
            .refused(BAD_VALUE, why);
        rig.run(&gate_set(RUN, "1", &sha('d'), &file), "")
            .refused(BAD_VALUE, why);
    }
    rig.run(
        &gate_check(RUN, Some("1"), &rig.tmp.join("no-such-file")),
        "",
    )
    .refused(BAD_VALUE, "a gate's output that is not there");
    rig.run(
        &gate_set(RUN, "1", "abcdef1", &rig.summary("candidate", &[], &[])),
        "",
    )
    .refused(BAD_VALUE, "a candidate named by a short sha");
    assert_eq!(rig.snapshot(), before, "no refusal wrote anything");

    // A gate somebody wrote by hand is not compared against on a guess.
    fs::write(
        rig.root.join(gate_path("1")),
        "# the gate\n\n| what | value |\n|---|---|\n| verdict | pass |\n| test | `x` |\n",
    )
    .expect("a gate by hand");
    rig.run(
        &gate_check(RUN, Some("1"), &rig.summary("record", &[], &[])),
        "",
    )
    .refused(CORRUPT, "a candidate's gate that is not the script's");
}

// ---------------------------------------------------------------------------
// Nothing the script is handed ends in a traceback
// ---------------------------------------------------------------------------

/// The state-machine review's `F11`, as an axis: every input the script parses — a JSON
/// body, an argument, a table cell — is refused in one line, by the class it belongs to.
#[test]
fn no_input_ends_in_a_traceback() {
    let rig = triaged_once("tracebacks");
    let before = rig.snapshot();
    let one_line = |seen: &Seen, code: i32, why: &str| {
        seen.refused(code, why);
        assert!(!seen.stderr.contains("Traceback"), "{why}: {}", seen.stderr);
    };

    // A JSON BODY, at every subcommand that reads one: nested past what the interpreter
    // parses; and holding a text that JSON can spell and no file can hold.
    let bodies: Vec<Vec<String>> = vec![
        ledger_add(RUN),
        ledger_set(RUN),
        triage_set(RUN, "1"),
        scope_set(RUN, "2"),
        item_set(RUN),
        round_set(RUN, "1"),
        run_set(RUN),
        apply(RUN, Some("1")),
    ];
    let deep = "[".repeat(200_000);
    let lone = r#"{"key": "audit-f5", "source": "a \ud800 b", "note": ["\udfff"]}"#;
    for args in &bodies {
        one_line(
            &rig.run(args, &deep),
            BAD_VALUE,
            &format!("`{}`: a body nested 200 000 deep", args[0]),
        );
        one_line(
            &rig.run(args, lone),
            BAD_VALUE,
            &format!("`{}`: a lone surrogate", args[0]),
        );
        one_line(
            &rig.run(args, "\u{feff}{}"),
            BAD_VALUE,
            &format!("`{}`: a byte-order mark", args[0]),
        );
    }
    // The same text in a body that is otherwise one the call takes: the cell it would have
    // been written into is where it ended in a traceback.
    let mut entry = row("audit-f5");
    entry["source"] = json!("a @@ b");
    let mut doors = scope(&["jigc setup"], &[]);
    doors["included"][0]["derivation"] = json!("a @@ b");
    for (args, body) in [
        (ledger_add(RUN), entry.to_string()),
        (
            ledger_set(RUN),
            json!({"key": "audit-f3", "disposition": "later", "detail": "a @@ b"}).to_string(),
        ),
        (scope_set(RUN, "2"), doors.to_string()),
    ] {
        one_line(
            &rig.run(&args, &body.replace("@@", "\\ud800")),
            BAD_VALUE,
            &format!("`{}`: a lone surrogate in a cell it would write", args[0]),
        );
    }
    // … and inside a batch, where a call's input is the batch's text.
    let inner = json!([{"argv": ledger_add(RUN), "stdin": deep}]);
    one_line(
        &rig.run(&apply(RUN, Some("1")), &inner.to_string()),
        BAD_VALUE,
        "a call of a batch whose body is nested 200 000 deep",
    );

    // AN ARGUMENT that is not UTF-8 text.
    let mut command = rig.command(&bound_set(RUN, "non-jigc-writer", &BOUND[1..]));
    command.arg(std::ffi::OsStr::from_bytes(b"--reach=a \xff b"));
    let out = command
        .stdin(Stdio::null())
        .output()
        .expect("the script exits");
    let seen = Seen {
        code: out.status.code(),
        signal: out.status.signal(),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    };
    one_line(&seen, BAD_VALUE, "an argument that is not UTF-8");
    assert_eq!(rig.snapshot(), before, "no refused input wrote anything");

    // A TABLE CELL a hand turned into something that only looks like a number: the three
    // cells the script reads as one.
    for (path, from, to, why) in [
        (
            ledger_path(),
            "| 1 | audit-install, finding 3 |",
            "| \u{b2} | audit-install, finding 3 |",
            "a superscript two as a ledger row's round",
        ),
        (
            ledger_path(),
            "| 1 | audit-install, finding 3 |",
            "| \u{661} | audit-install, finding 3 |",
            "an Arabic-Indic one as a ledger row's round",
        ),
        (
            ledger_path(),
            "| 1 | audit-install, finding 3 |",
            "| 1\u{b2} | audit-install, finding 3 |",
            "a round with a superscript",
        ),
    ] {
        let other = triaged_once("tracebacks-cell");
        let text = other.read(&path);
        assert!(text.contains(from), "the fixture's row: {text}");
        fs::write(other.root.join(&path), text.replace(from, to)).expect("a cell by hand");
        one_line(&other.run(&state(RUN), ""), CORRUPT, why);
    }
    for (facts, call, path, from, to, why) in [
        (
            json!({"cycles": 2}),
            round_set(RUN, "1"),
            round_path("1"),
            "`2`",
            "`\u{b2}`",
            "a superscript two as a round's cycles",
        ),
        (
            json!({"rounds": 3}),
            run_set(RUN),
            run_path(),
            "`3`",
            "`\u{663}`",
            "an Arabic-Indic three as the bound across rounds",
        ),
    ] {
        let other = triaged_once("tracebacks-fact");
        // A fix cycle is counted in a tested round.
        other
            .run(
                &round_set(RUN, "1"),
                &json!({"candidate": sha('c')}).to_string(),
            )
            .must(OK, "the round's candidate");
        other.run(&call, &facts.to_string()).must(OK, why);
        let text = other.read(&path);
        assert!(text.contains(from), "the fixture's row: {text}");
        fs::write(other.root.join(&path), text.replace(from, to)).expect("a cell by hand");
        one_line(&other.run(&state(RUN), ""), CORRUPT, why);
    }

    // AND WHAT NO REFUSAL NAMES is still one line, with a status of its own — never a
    // traceback, and never a status another refusal has.
    let mut faulty = triaged_once("tracebacks-fault");
    with_kill_switch(&mut faulty, "STUB_FAULT_AT_REPLACE", 1);
    let seen = faulty.run(&triage_set(RUN, "1"), &found_again());
    one_line(&seen, FAULT, "a fault nobody foresaw");
    assert!(
        seen.stderr.contains("RuntimeError at line ")
            && seen.stderr.contains("a fault nobody foresaw"),
        "it says what was raised, and where: {}",
        seen.stderr
    );
}

// ---------------------------------------------------------------------------
// 27 · Every loop has a counter on record and an exit to the human
// ---------------------------------------------------------------------------

/// The passes of a round's triage are counted BY THE CALL THAT RECORDS ONE, in the round's
/// own record — never by an invocation, and never supplied: every finding a triage record
/// leaves without a grade or a verdict is named there with how many of the round's records
/// in a row have left it so. One more pass is automatic; after it the finding is the
/// human's, with the reason — and what the human may do is rule it, or grant one more.
#[test]
fn a_finding_the_triage_cannot_settle_is_tried_once_more_and_then_it_is_the_humans() {
    let rig = two_clause_run("passes", r#""stop": "at-the-bound", "rounds": 9"#);
    let both = [("check-clause-a", "green"), ("row-clause-b", "green")];
    tested_round(&rig, 1, true, &both, json!({}));
    rig.run(
        &ledger_add(RUN),
        &json!([row_at("f-open", INSIDE), row_at("f-seeded", INSIDE)]).to_string(),
    )
    .must(OK, "two findings");
    let unverified = json!({"key": "f-open", "grade": "breaks"}).to_string();
    let awaiting = |rig: &Rig| rig.state()["rounds"][0]["facts"]["awaiting"].clone();
    let awaits = |rig: &Rig, key: &str| {
        let read = rig.state();
        let row = read["ledger"]
            .as_array()
            .expect("the ledger")
            .iter()
            .find(|row| row["key"] == key)
            .expect("the row")
            .clone();
        json!([row["route"], row["why"], row["awaits"]])
    };

    // The stage's own triage: one finding graded and sent on, the verifier settled
    // nothing; the other was never graded. Both are counted, once.
    rig.run(&triage_set(RUN, "1"), &unverified)
        .must(OK, "the stage's own triage");
    assert_eq!(
        awaiting(&rig),
        json!(["f-open unverified 1", "f-seeded ungraded 1"])
    );
    assert_eq!(
        awaits(&rig, "f-open"),
        json!(["triage", "unverified", {"why": "unverified", "passes": 1, "retry": "due"}])
    );
    assert!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS).contains(&vec![
            "`awaiting`".to_owned(),
            "`f-open unverified 1, f-seeded ungraded 1`".to_owned()
        ]),
        "the count is a row of the round's record"
    );
    assert_eq!(rig.state()["next"], json!("triage"));
    // Nothing is the human's yet, and nothing is granted before it is.
    let before = rig.snapshot();
    rig.run(
        &round_set(RUN, "1"),
        &json!({"reverify": "f-open"}).to_string(),
    )
    .refused(
        BAD_VALUE,
        "one more triage before the automatic one is spent",
    );
    // The count is the script's: no caller writes it.
    for (given, why) in [
        (
            json!({"awaiting": ["f-open unverified 1"]}),
            "the passes, supplied",
        ),
        (json!({"awaiting": []}), "the passes, taken back"),
        (json!({"ran": ["row-clause-b a1"]}), "the runs, supplied"),
        (
            json!({"recorded": ["test a1"]}),
            "an attempt that reached its record, supplied",
        ),
    ] {
        rig.run(&round_set(RUN, "1"), &given.to_string())
            .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);

    // The one pass that is automatic grades the seeded finding and still cannot verify
    // the other: that one is the human's, with the reason — and the seeded one, graded
    // now and unverified for the first time, is not.
    rig.run(
        &triage_set(RUN, "1"),
        &json!([{"key": "f-open", "grade": "unclear"}, {"key": "f-seeded", "grade": "breaks"}])
            .to_string(),
    )
    .must(OK, "the finishing triage");
    assert_eq!(
        awaiting(&rig),
        json!(["f-open unverified 2", "f-seeded unverified 1"])
    );
    assert_eq!(
        awaits(&rig, "f-open"),
        json!(["human", "unverified-after-retry",
               {"why": "unverified", "passes": 2, "retry": "spent"}])
    );
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["human_list"], &read["untriaged"]),
        (
            &json!("triage"),
            &json!([{"key": "f-open", "why": "unverified-after-retry"}]),
            &json!([{"key": "f-seeded", "why": "unverified"}])
        ),
        "the other finding's triage is finished first: {read}"
    );
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "f-seeded", "grade": "breaks", "verdict": "refuted"}).to_string(),
    )
    .must(OK, "the seeded finding, refuted");
    assert_eq!(
        awaiting(&rig),
        json!(["f-open unverified 3"]),
        "a record that settles a finding names it no more, and one that leaves a finding counts"
    );
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["position"]["test"]),
        (
            &json!("rule"),
            &json!({"refused": "round-open", "round": 1})
        ),
        "no stage finishes this triage again unasked: {read}"
    );

    // Granted: the record holds the pass it allows, and one grant is one pass.
    let before = rig.snapshot();
    for (round, key, why) in [
        (
            "1",
            "f-seeded",
            "one more triage of a finding that is settled",
        ),
        (
            "1",
            "f-nowhere",
            "one more triage of a finding the ledger lacks",
        ),
    ] {
        rig.run(
            &round_set(RUN, round),
            &json!({"reverify": key}).to_string(),
        )
        .refused(BAD_VALUE, why);
    }
    assert_eq!(rig.snapshot(), before);
    let seen = rig.run(
        &round_set(RUN, "1"),
        &json!({"reverify": "f-open"}).to_string(),
    );
    seen.must(OK, "one more triage, granted");
    assert_eq!(seen.json()["facts"]["reverify"], json!(["f-open p4"]));
    rig.run(
        &round_set(RUN, "1"),
        &json!({"reverify": "f-open"}).to_string(),
    )
    .refused(BAD_VALUE, "a second grant while the first is not spent");
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["position"]["test"]),
        (
            &json!("triage"),
            &json!({"round": 1, "attempt": 1, "triage": true})
        ),
        "{read}"
    );
    // … and the granted pass confirms it: a fixer's, counted no more.
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "f-open", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "the granted triage");
    assert_eq!(awaiting(&rig), json!([]));
    assert_eq!(rig.state()["next"], json!("fix"));

    // A red check's finding is filed ungraded by the call that records the result: the
    // record that files it counts one pass without a grade, and the first triage that
    // grades it and cannot verify it counts its FIRST pass without a verdict.
    let other = two_clause_run("passes-red", r#""stop": "at-the-bound", "rounds": 9"#);
    other
        .run(
            &scope_set(RUN, "1"),
            &scope(&[INSIDE], &[EXCLUDED]).to_string(),
        )
        .must(OK, "the scope");
    let mut red = ran("check-clause-a", "red");
    for (field, value) in [
        ("doctype", "jigc-feedback"),
        ("door", INSIDE),
        ("repro", "the gate"),
    ] {
        red[field] = json!(value);
    }
    other
        .run(
            &result_set(RUN, "1", &candidate(1)),
            &json!([red, ran("row-clause-b", "green")]).to_string(),
        )
        .must(OK, "a red check");
    other.seed_rows(&["beside"]);
    other
        .run(
            &triage_set(RUN, "1"),
            &json!({"key": "beside", "grade": "no-break"}).to_string(),
        )
        .must(
            OK,
            "the stage's triage, which was never handed the red check",
        );
    other
        .run(
            &round_set(RUN, "1"),
            &json!({"candidate": candidate(1)}).to_string(),
        )
        .must(OK, "the candidate");
    assert_eq!(awaiting(&other), json!(["red-check-clause-a ungraded 1"]));
    other
        .run(
            &triage_set(RUN, "1"),
            &json!({"key": "red-check-clause-a", "grade": "breaks"}).to_string(),
        )
        .must(OK, "the finishing triage grades it");
    assert_eq!(awaiting(&other), json!(["red-check-clause-a unverified 1"]));
    assert_eq!(
        other.state()["next"],
        json!("triage"),
        "the verifier gets its one more attempt"
    );
}

/// **A stop is lifted by its go, and by nothing else** (the review's `F7`, three ways, and
/// `F9`; the repair plan's state 10). A write that names a round the run has not begun is
/// taken only where the state hands that round to a `test` stage; when a run stops is
/// written once; a bound is raised only at its own stop; and a go answers the stop it was
/// given at — a round that is over again after one more fix cycle stops again.
#[test]
fn a_stop_is_lifted_by_its_go_and_by_no_other_write() {
    let rig = two_clause_run("stops", EVERY_ROUND);
    let both = [("check-clause-a", "green"), ("row-clause-b", "green")];
    tested_round(&rig, 1, true, &both, json!({}));
    let stopped = json!({"why": "every-round", "round": 1, "then": "close"});
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["stop"], &read["begins"]),
        (&json!("stop"), &stopped, &json!(null)),
        "{read}"
    );

    // The three ways, and every other write that names the round after: each refused,
    // each leaving the tree and the stop as they were.
    let before = rig.snapshot();
    let summary = rig.summary("stops", &[], &[]);
    let lifts: Vec<(Seen, &str)> = vec![
        (
            rig.run(
                &round_set(RUN, "2"),
                &json!({"base": "abcdef1"}).to_string(),
            ),
            "a fact of round 2's record",
        ),
        (
            rig.run(&report(RUN, "2", "scope", "1"), BODY),
            "a report of round 2",
        ),
        (
            rig.run(
                &scope_set(RUN, "2"),
                &scope(&[INSIDE], &[EXCLUDED]).to_string(),
            ),
            "round 2's scope",
        ),
        (
            rig.run(&gate_set(RUN, "2", &candidate(2), &summary), ""),
            "the gate of round 2's candidate",
        ),
        (
            rig.run(
                &result_set(RUN, "2", &candidate(2)),
                &ran("check-clause-a", "green").to_string(),
            ),
            "a result of round 2",
        ),
        (
            rig.run(
                &triage_set(RUN, "2"),
                &json!({"key": "nobody", "grade": "no-break"}).to_string(),
            ),
            "round 2's triage",
        ),
        (
            rig.run(&run_set(RUN), &json!({"rounds": 50}).to_string()),
            "a bound across rounds, written past the opening and at no stop of its own",
        ),
        (
            rig.run(&run_set(RUN), &json!({"cycles": 50}).to_string()),
            "a bound on a round's cycles, raised at no stop of its own",
        ),
    ];
    for (seen, why) in &lifts {
        seen.refused(BAD_VALUE, why);
        assert!(
            seen.stderr.contains("the state does not ask for it"),
            "{why}: the refusal says that the state does not ask for the write: {}",
            seen.stderr
        );
    }
    rig.run(&run_set(RUN), &json!({"stop": "at-the-bound"}).to_string())
        .refused(EXISTS, "the stop mode, rewritten");
    assert_eq!(rig.snapshot(), before, "a refused write writes nothing");
    assert_eq!(rig.state()["stop"], stopped, "and the stop stands");

    // The go lifts it: the record holds the count of fix cycles it was given at.
    let go = json!({"go": true}).to_string();
    rig.run(&round_set(RUN, "1"), &go).must(OK, "the go");
    assert!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS)
            .contains(&vec!["`go`".to_owned(), "`c0`".to_owned()])
    );
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["stop"], &read["begins"]),
        (&json!("close"), &json!(null), &json!(2)),
        "{read}"
    );

    // F9 — the human does not close: a finding is admitted and fixed, and a fix cycle is
    // recorded. The round is over AGAIN, and the go given before that cycle is no answer
    // to this stop: the next round's `test` does not start unasked.
    rig.seed_rows(&["admitted-late"]);
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "admitted-late", "grade": "breaks", "verdict": "confirmed", "regression": false})
            .to_string(),
    )
    .must(OK, "a finding, confirmed");
    for patch in [
        json!({"key": "admitted-late", "disposition": "admitted"}),
        json!({"key": "admitted-late", "disposition": "fixed", "detail": "0f34d8f0"}),
    ] {
        rig.run(&ledger_set(RUN), &patch.to_string())
            .must(OK, "admitted, then fixed");
    }
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "the fix cycle");
    let read = rig.state();
    assert_eq!(
        (
            &read["next"],
            &read["stop"],
            &read["rounds"][0]["facts"]["go"],
            &read["position"]["test"],
            &read["begins"]
        ),
        (
            &json!("stop"),
            &json!({"why": "every-round", "round": 1, "then": "test"}),
            &json!(false),
            &json!({"refused": "stopped", "round": 1}),
            &json!(null)
        ),
        "a go lifts one stop, and not the next: {read}"
    );
    rig.run(&round_set(RUN, "1"), &go)
        .must(OK, "the go after the cycle");
    assert!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS)
            .contains(&vec!["`go`".to_owned(), "`c1`".to_owned()])
    );
    let read = rig.state();
    assert_eq!(
        (&read["next"], &read["begins"]),
        (&json!("test"), &json!(2)),
        "{read}"
    );
    // Round 2 is begun where the state hands it out — and round 3 is not.
    rig.run(&report(RUN, "3", "scope", "1"), BODY)
        .refused(BAD_VALUE, "a report two rounds ahead");
    rig.run(&report(RUN, "2", "scope", "1"), BODY)
        .must(OK, "round 2's first report");
    rig.run(&report(RUN, "1", "late", "9"), BODY)
        .must(OK, "a report of a round the run has begun");

    // At a spent bound across rounds: the mode stands, the other bound is not the one that
    // is asked for, and the raise is the go.
    let other = two_clause_run("stops-bound", r#""stop": "at-the-bound", "rounds": 1"#);
    tested_round(&other, 1, true, &both, json!({}));
    other
        .run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .must(OK, "round 1's fix cycle: the bound of one is spent");
    tested_round(&other, 2, true, &both, json!({}));
    other.seed_rows(&["blocker"]);
    other
        .run(
            &triage_set(RUN, "2"),
            &json!({"key": "blocker", "grade": "breaks", "verdict": "confirmed", "regression": false})
                .to_string(),
        )
        .must(OK, "round 2's triage");
    let at_bound = json!({"why": "round-bound", "round": 2, "then": "fix"});
    assert_eq!(other.state()["stop"], at_bound);
    let before = other.snapshot();
    other
        .run(&run_set(RUN), &json!({"stop": "every-round"}).to_string())
        .refused(
            EXISTS,
            "the other stop mode, in which the bound is not applied",
        );
    other
        .run(&run_set(RUN), &json!({"cycles": 4}).to_string())
        .refused(
            BAD_VALUE,
            "the bound on a round's cycles, at the stop of the other bound",
        );
    other
        .run(&round_set(RUN, "2"), &json!({"cycles": 1}).to_string())
        .refused(
            BAD_VALUE,
            "a fix cycle counted past the bound across rounds",
        );
    assert_eq!(other.snapshot(), before);
    assert_eq!(other.state()["stop"], at_bound);
    other
        .run(&run_set(RUN), &json!({"rounds": 2}).to_string())
        .must(OK, "the bound, raised at its own stop");
    assert_eq!(other.state()["next"], json!("fix"));
    other
        .run(&run_set(RUN), &json!({"rounds": 3}).to_string())
        .refused(
            BAD_VALUE,
            "the bound, raised again where nothing stops at it",
        );
}

/// **What is deleted is missed** (the review's `F6`, and the item's last result the entry
/// *Evidence belongs to a test-set item* left uncaught). A ledger row, a result and a
/// report that a record names and that are gone are each a refusal that names what is
/// missing — never a quieter state.
#[test]
fn what_is_deleted_is_missed_and_named() {
    let build = |label: &str| {
        let rig = two_clause_run(label, r#""stop": "at-the-bound", "rounds": 9"#);
        tested_round(
            &rig,
            1,
            true,
            &[("check-clause-a", "green"), ("row-clause-b", "void")],
            json!({}),
        );
        rerun(&rig, 1, 1, "row-clause-b", "green").must(OK, "the hunt's re-run");
        rig.run(
            &ledger_add(RUN),
            &json!([
                row_at("f-triaged", INSIDE),
                row_at("f-left", INSIDE),
                row_at("f-opening", INSIDE)
            ])
            .to_string(),
        )
        .must(OK, "three findings");
        rig
    };
    let without = |rig: &Rig, rel: &str, marker: &str| {
        let path = rig.root.join(rel);
        let text = fs::read_to_string(&path).expect("read the table");
        let kept: String = text
            .lines()
            .filter(|line| !line.contains(marker))
            .map(|line| format!("{line}\n"))
            .collect();
        assert_ne!(kept, text, "the row `{marker}` was there");
        fs::write(&path, kept).expect("delete the row");
    };
    let refusal = |rig: &Rig, names: &[&str], why: &str| {
        let seen = rig.run(&state(RUN), "");
        seen.refused(CORRUPT, why);
        for name in names {
            assert!(
                seen.stderr.contains(name),
                "{why}: the refusal names `{name}`: {}",
                seen.stderr
            );
        }
    };

    // The review's own repro: a finding nobody triaged yet — an opening row — deleted from
    // the ledger. The run's record names every finding the ledger was handed.
    let rig = build("deleted-untriaged");
    assert_eq!(rig.state()["next"], json!("triage"));
    assert_eq!(
        rig.state()["facts"]["findings"],
        json!(["f-triaged", "f-left", "f-opening"])
    );
    without(&rig, &ledger_path(), "`f-opening`");
    refusal(
        &rig,
        &["f-opening", "run.md"],
        "a ledger row no round has triaged, deleted",
    );
    // … and no writer writes past it: not the ledger's own, and not another table's.
    let before = rig.snapshot();
    rig.run(&ledger_add(RUN), &row_at("f-new", INSIDE).to_string())
        .refused(CORRUPT, "a row added to a ledger with a row gone");
    rig.run(
        &ledger_set(RUN),
        &json!({"key": "f-left", "disposition": "later"}).to_string(),
    )
    .refused(CORRUPT, "a ruling on a ledger with a row gone");
    rig.run(&round_set(RUN, "1"), &json!({"cycles": 1}).to_string())
        .refused(
            CORRUPT,
            "a fact of the round, over a ledger with a row gone",
        );
    assert_eq!(rig.snapshot(), before);

    // A row a recorded triage names — by its entry, and by the count of what it left.
    let rig = build("deleted-triaged");
    rig.run(
        &triage_set(RUN, "1"),
        &json!({"key": "f-triaged", "grade": "no-break"}).to_string(),
    )
    .must(OK, "round 1's triage: one graded, two left");
    for (key, by) in [("f-triaged", "triages"), ("f-left", "left without a grade")] {
        let other = build(&format!("deleted-{key}"));
        other
            .run(
                &triage_set(RUN, "1"),
                &json!({"key": "f-triaged", "grade": "no-break"}).to_string(),
            )
            .must(OK, "round 1's triage");
        without(&other, &ledger_path(), &format!("`{key}`"));
        refusal(
            &other,
            &[key, by],
            "a ledger row a recorded triage names, deleted",
        );
    }
    // A row a hand ADDED is no finding of the run either.
    let text = rig.read(&ledger_path());
    let added = text
        .lines()
        .find(|line| line.contains("`f-left`"))
        .expect("the row")
        .replace("`f-left`", "`f-by-hand`");
    fs::write(rig.root.join(ledger_path()), format!("{text}{added}\n")).expect("add a row");
    refusal(&rig, &["f-by-hand"], "a ledger row added by hand");

    // An item's LAST result: its re-run's row, which made the clause green. Without the
    // round's count of its runs the state would read "void, and its one re-run is due".
    let rig = build("deleted-result");
    assert!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS).contains(&vec![
            "`ran`".to_owned(),
            "`check-clause-a a1, row-clause-b a2`".to_owned()
        ]),
        "the round's record counts the runs of each item"
    );
    let results = rig.root.join(results_path("1"));
    let text = fs::read_to_string(&results).expect("the results");
    let last = text.lines().last().expect("a row");
    assert!(last.contains("`row-clause-b` | 2 |"), "{last}");
    fs::write(&results, text.replace(&format!("{last}\n"), "")).expect("delete the last result");
    refusal(
        &rig,
        &["attempt 2 of row-clause-b", "round.md"],
        "an item's last result, deleted",
    );
    rerun(&rig, 1, 1, "row-clause-b", "green").refused(
        CORRUPT,
        "the re-run a quieter state would have asked for again",
    );
    // An item's ONLY result, and a result a hand added.
    let rig = build("deleted-only-result");
    without(&rig, &results_path("1"), "`check-clause-a`");
    refusal(
        &rig,
        &["attempt 1 of check-clause-a"],
        "an item's only result, deleted",
    );
    let rig = build("added-result");
    let text = rig.read(&results_path("1"));
    let extra = text
        .lines()
        .last()
        .expect("a row")
        .replace("| 2 |", "| 3 |");
    fs::write(rig.root.join(results_path("1")), format!("{text}{extra}\n")).expect("add a result");
    refusal(
        &rig,
        &["attempt 3 of row-clause-b", "does not count"],
        "a result added by hand",
    );
}

/// **The attempts of a stage are counted from the last one that reached its record.** A
/// batch that holds a stage's report check is the record of that attempt, and `apply` says
/// so in the round's record; an attempt after it that left reports and nothing else is
/// counted, one more is made without the human, and then the stage is the human's — who
/// grants one more attempt, one grant one attempt.
#[test]
fn the_attempts_of_a_stage_are_counted_from_the_last_that_reached_its_record() {
    let rig = two_clause_run("attempts", r#""stop": "at-the-bound", "rounds": 9"#);
    rig.run(
        &scope_set(RUN, "1"),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
    .must(OK, "round 1's scope");
    rig.seed_rows(&["f-open"]);
    // Attempt 1 halted after its preflight; attempt 2 reaches its record, as a stage's
    // batch: the reports checked, what each item did, the triage, the candidate.
    rig.run(&report(RUN, "1", "preflight", "1"), BODY)
        .must(OK, "attempt 1's report");
    rig.run(&report(RUN, "1", "preflight", "2"), BODY)
        .must(OK, "attempt 2's report");
    let unrecorded = |rig: &Rig| rig.state()["rounds"][0]["test_unrecorded"].clone();
    assert_eq!(unrecorded(&rig), json!(2));
    let batch = json!([
        call(check_reports(RUN, "1", "2", &["preflight"]), ""),
        call(
            result_set(RUN, "1", &candidate(1)),
            &json!([ran("check-clause-a", "green"), ran("row-clause-b", "green")]).to_string()
        ),
        call(
            triage_set(RUN, "1"),
            &json!({"key": "f-open", "grade": "breaks"}).to_string()
        ),
        call(
            round_set(RUN, "1"),
            &json!({"candidate": candidate(1)}).to_string()
        ),
    ]);
    rig.run(&apply(RUN, Some("1")), &batch.to_string())
        .must(OK, "the stage's record");
    rig.run(&keeper("settle", RUN), "").must(OK, "its commit");
    assert!(
        table(&rig.read(&round_path("1")), &ROUND_COLUMNS)
            .contains(&vec!["`recorded`".to_owned(), "`test a2`".to_owned()]),
        "the batch that held the report check is the record of that attempt"
    );
    let read = rig.state();
    assert_eq!(
        (
            unrecorded(&rig),
            &read["next"],
            &read["position"]["test"],
            &read["human_stages"]
        ),
        (
            json!(0),
            &json!("triage"),
            &json!({"round": 1, "attempt": 3, "triage": true}),
            &json!([])
        ),
        "an attempt that reached its record is counted against nothing: {read}"
    );

    // The finishing invocation halts at its preflight: a report, and no record. Once more
    // is automatic.
    rig.run(&report(RUN, "1", "preflight", "3"), BODY)
        .must(OK, "the halted finishing attempt's report");
    let read = rig.state();
    assert_eq!(
        (unrecorded(&rig), &read["next"], &read["position"]["test"]),
        (
            json!(1),
            &json!("triage"),
            &json!({"round": 1, "attempt": 4, "triage": true})
        ),
        "{read}"
    );
    let before = rig.snapshot();
    rig.run(&round_set(RUN, "1"), &json!({"again": "test"}).to_string())
        .refused(
            BAD_VALUE,
            "one more attempt before the automatic one is spent",
        );
    assert_eq!(rig.snapshot(), before);
    // It halts again: the stage is the human's, with the reason, and it is refused.
    rig.run(&report(RUN, "1", "preflight", "4"), BODY)
        .must(OK, "the second halted attempt's report");
    let read = rig.state();
    let spent = json!([{"stage": "test", "round": 1, "cycle": null, "at": "test",
                        "attempt": 5, "attempts": 2, "why": "not-recorded-after-retry"}]);
    assert_eq!(
        (
            &read["next"],
            &read["human_stages"],
            &read["position"]["test"],
            &read["untriaged"]
        ),
        (
            &json!("rule"),
            &spent,
            &json!({"refused": "attempts-spent", "round": 1}),
            &json!([{"key": "f-open", "why": "unverified"}])
        ),
        "{read}"
    );
    let before = rig.snapshot();
    for (round, stage, why) in [
        ("1", "fix", "one more attempt of the other stage"),
        (
            "2",
            "test",
            "one more attempt in a round the run has not begun",
        ),
    ] {
        rig.run(&round_set(RUN, round), &json!({"again": stage}).to_string())
            .refused(BAD_VALUE, why);
    }
    rig.run(&round_set(RUN, "1"), &json!({"again": "both"}).to_string())
        .refused(BAD_VALUE, "one more attempt of no stage");
    assert_eq!(rig.snapshot(), before);
    let seen = rig.run(&round_set(RUN, "1"), &json!({"again": "test"}).to_string());
    seen.must(OK, "one more attempt, granted");
    assert_eq!(seen.json()["facts"]["again"], json!(["test a5"]));
    let read = rig.state();
    assert_eq!(
        (
            &read["next"],
            &read["position"]["test"],
            &read["human_stages"]
        ),
        (
            &json!("triage"),
            &json!({"round": 1, "attempt": 5, "triage": true}),
            &json!([])
        ),
        "{read}"
    );
    // The granted attempt reaches its record: the count starts again from it.
    rig.run(&report(RUN, "1", "preflight", "5"), BODY)
        .must(OK, "attempt 5's report");
    let finished = json!([
        call(check_reports(RUN, "1", "5", &["preflight"]), ""),
        call(
            triage_set(RUN, "1"),
            &json!({"key": "f-open", "grade": "breaks", "verdict": "refuted"}).to_string()
        ),
    ]);
    rig.run(&apply(RUN, Some("1")), &finished.to_string())
        .must(OK, "the finished triage's record");
    rig.run(&keeper("settle", RUN), "").must(OK, "its commit");
    let read = rig.state();
    assert_eq!(
        (
            unrecorded(&rig),
            &read["next"],
            &read["rounds"][0]["facts"]["recorded"]
        ),
        (json!(0), &json!("close"), &json!(["test a5"])),
        "{read}"
    );

    // WHAT IS DELETED IS MISSED: the reports of an attempt that reached its record.
    fs::remove_file(rig.root.join(report_path("1", "preflight", "5"))).expect("delete a report");
    let seen = rig.run(&state(RUN), "");
    seen.refused(CORRUPT, "the reports of a recorded attempt, deleted");
    assert!(
        seen.stderr.contains("attempt 5 of test"),
        "it names the attempt: {}",
        seen.stderr
    );
}

// ---------------------------------------------------------------------------
// 28 · The writer × state table: a write is taken only where the state asks for it
// ---------------------------------------------------------------------------

/// The positions of a run the table is driven at, in the order of every row's cells —
/// each stood up through the script's own writers by [`place`].
const PLACES: [&str; 17] = [
    "the opening: nothing written, no round",
    "ready, and no round yet",
    "round 1 begun: a scope, and no record",
    "tested, a blocker open: a fixer's",
    "tested, a finding unverified: the triage is finished once more",
    "tested, the finding unverified after its retry: the human's",
    "tested, a finding outside the test set: the human's list",
    "tested, nothing open, every clause green: close",
    "tested, an item void: its re-run",
    "tested, the re-run spent: the human's clause",
    "every round a stop: stopped after round 1",
    "every round a stop: the go given",
    "a round that landed: the next round's test",
    "stopped at the bound across rounds, in round 2",
    "stopped at the bound on a round's cycles",
    "round 1 begun, two attempts without a record: the human's stage",
    "a dropped round: the next round's test",
];

/// Stand position `n` of [`PLACES`] up in a rig of its own.
fn place(n: usize) -> Rig {
    let label = format!("place-{n}");
    let both = [("check-clause-a", "green"), ("row-clause-b", "green")];
    let voided = [("check-clause-a", "green"), ("row-clause-b", "void")];
    let rig = match n {
        0 => Rig::new(&label),
        10 | 11 => two_clause_run(&label, EVERY_ROUND),
        13 => two_clause_run(&label, r#""stop": "at-the-bound", "rounds": 1"#),
        _ => two_clause_run(&label, BOUNDED),
    };
    // A finding that is recorded everywhere: a row for the writers of the ledger to name.
    let mut prop = row_at("prop", INSIDE);
    prop["grade"] = json!("no-break");
    prop["graded_by"] = json!("human");
    rig.run(&ledger_add(RUN), &prop.to_string())
        .must(OK, "a recorded finding");
    let begun = |rig: &Rig| {
        rig.run(
            &scope_set(RUN, "1"),
            &scope(&[INSIDE], &[EXCLUDED]).to_string(),
        )
        .must(OK, "round 1's scope");
    };
    let found = |rig: &Rig, round: &str, door: &str, entry: Value| {
        rig.run(&ledger_add(RUN), &row_at("blocker", door).to_string())
            .must(OK, "a finding");
        rig.run(&triage_set(RUN, round), &entry.to_string())
            .must(OK, "its triage");
    };
    let confirmed =
        json!({"key": "blocker", "grade": "breaks", "verdict": "confirmed", "regression": false});
    let unverified = json!({"key": "blocker", "grade": "breaks"});
    let fact = |rig: &Rig, round: &str, facts: Value| {
        rig.run(&round_set(RUN, round), &facts.to_string())
            .must(OK, "a fact of the round");
    };
    match n {
        0 | 1 => {}
        2 => begun(&rig),
        3 | 14 | 16 => {
            tested_round(&rig, 1, true, &both, json!({}));
            found(&rig, "1", INSIDE, confirmed);
            if n == 14 {
                fact(&rig, "1", json!({"cycles": 3}));
            }
            if n == 16 {
                fact(&rig, "1", json!({"outcome": "dropped"}));
            }
        }
        4 | 5 => {
            tested_round(&rig, 1, true, &both, json!({}));
            found(&rig, "1", INSIDE, unverified.clone());
            if n == 5 {
                rig.run(&triage_set(RUN, "1"), &unverified.to_string())
                    .must(OK, "the triage, once more");
            }
        }
        6 => {
            tested_round(&rig, 1, true, &both, json!({}));
            found(&rig, "1", EXCLUDED, confirmed);
        }
        7 | 10 | 11 | 12 => {
            tested_round(&rig, 1, true, &both, json!({}));
            if n == 11 {
                fact(&rig, "1", json!({"go": true}));
            }
            if n == 12 {
                fact(&rig, "1", json!({"cycles": 1}));
            }
        }
        8 | 9 => {
            tested_round(&rig, 1, true, &voided, json!({}));
            if n == 9 {
                rerun(&rig, 1, 1, "row-clause-b", "void").must(OK, "the one re-run");
            }
        }
        13 => {
            tested_round(&rig, 1, true, &both, json!({}));
            fact(&rig, "1", json!({"cycles": 1}));
            tested_round(&rig, 2, true, &both, json!({}));
            found(&rig, "2", INSIDE, confirmed);
        }
        15 => {
            begun(&rig);
            for attempt in ["1", "2"] {
                rig.run(&report(RUN, "1", "preflight", attempt), BODY)
                    .must(OK, "an attempt's report");
            }
        }
        _ => unreachable!("a place of the table"),
    }
    rig
}

/// What each place answers, by hand: `(next, stop.why, begins)`.
const PLACE_SAYS: [(&str, Option<&str>, Option<u64>); 17] = [
    ("not-ready", None, Some(1)),
    ("test", None, Some(1)),
    ("test", None, None),
    ("fix", None, None),
    ("triage", None, None),
    ("rule", None, None),
    ("rule", None, None),
    ("close", None, Some(2)),
    ("retest", None, None),
    ("rule", None, Some(2)),
    ("stop", Some("every-round"), None),
    ("close", None, Some(2)),
    ("test", None, Some(2)),
    ("stop", Some("round-bound"), None),
    ("stop", Some("cycle-bound"), None),
    ("rule", None, None),
    ("test", None, Some(2)),
];

/// One write of the table: a writer — where it names a round, for the latest round the
/// run has begun (`here`; round 1 where it has begun none) and for the round after it
/// (`next`) — or one fact of a writer that takes facts. `takes` is a cell per place of
/// [`PLACES`], WRITTEN BY HAND: `a` the state takes it (exit 0), `r` it is refused as not
/// asked for or as written once, and the tree is as it was, `s` the table takes it and
/// the round has no scope to write it under.
struct Taken {
    name: &'static str,
    takes: &'static str,
    call: fn(&Rig, &Value, usize) -> Seen,
}

/// The round's candidate where it is tested, else one of its own.
fn commit_of(read: &Value, round: usize) -> String {
    read["rounds"]
        .as_array()
        .expect("the rounds")
        .iter()
        .find(|held| held["round"] == json!(round))
        .and_then(|held| held["facts"]["candidate"].as_str())
        .map_or_else(|| candidate(round), str::to_owned)
}

fn fact_of_round(rig: &Rig, round: usize, facts: Value) -> Seen {
    rig.run(&round_set(RUN, &round.to_string()), &facts.to_string())
}

fn fact_of_run(rig: &Rig, facts: Value) -> Seen {
    rig.run(&run_set(RUN), &facts.to_string())
}

fn written_report(rig: &Rig, _: &Value, round: usize) -> Seen {
    rig.run(&report(RUN, &round.to_string(), "prop", "9"), BODY)
}

fn written_scope(rig: &Rig, _: &Value, round: usize) -> Seen {
    rig.run(
        &scope_set(RUN, &round.to_string()),
        &scope(&[INSIDE], &[EXCLUDED]).to_string(),
    )
}

fn written_triage(rig: &Rig, _: &Value, round: usize) -> Seen {
    rig.run(
        &triage_set(RUN, &round.to_string()),
        &json!({"key": "prop", "grade": "no-break"}).to_string(),
    )
}

fn written_result(rig: &Rig, read: &Value, round: usize) -> Seen {
    rig.run(
        &result_set(RUN, &round.to_string(), &commit_of(read, round)),
        &ran("row-clause-b", "green").to_string(),
    )
}

fn written_gate(rig: &Rig, read: &Value, round: usize) -> Seen {
    let summary = rig.summary("place", &[], &[]);
    rig.run(
        &gate_set(RUN, &round.to_string(), &commit_of(read, round), &summary),
        "",
    )
}

const ANYWHERE: &str = "aaaaaaaaaaaaaaaaa";
const NOWHERE: &str = "rrrrrrrrrrrrrrrrr";
/// A round the run has not begun is written only where the state hands it to a `test`
/// stage: after a round that left nothing open, was dropped, or landed — and never past
/// a stop, past an open round, or while a re-run is what is asked.
const WHERE_A_ROUND_BEGINS: &str = "rrrrrrrararaarrra";
/// … where the write then meets a round with no scope yet.
const WHERE_A_ROUND_BEGINS_UNSCOPED: &str = "rrrrrrrsrsrssrrrs";
/// What a round's test stage records is taken while the round is not tested.
const WHILE_NOT_TESTED: &str = "aaarrrrrrrrrrrrar";
/// What the opening writes once is taken where the run does not hold it yet.
const AT_THE_OPENING: &str = "arrrrrrrrrrrrrrrr";

const HERE: &[Taken] = &[
    Taken {
        name: "report",
        takes: ANYWHERE,
        call: written_report,
    },
    Taken {
        name: "scope-set",
        takes: "aarrrrrrrrrrrrrrr",
        call: written_scope,
    },
    Taken {
        name: "triage-set",
        takes: "ssaaaaaaaaaaaaaaa",
        call: written_triage,
    },
    Taken {
        name: "result-set",
        takes: "ssarrrrrarrrrrrar",
        call: written_result,
    },
    Taken {
        name: "gate-set",
        takes: WHILE_NOT_TESTED,
        call: written_gate,
    },
    Taken {
        name: "ledger-add",
        takes: ANYWHERE,
        call: |rig, _, _| {
            let mut new = row_at("prop-new", INSIDE);
            new["grade"] = json!("no-break");
            new["graded_by"] = json!("human");
            rig.run(&ledger_add(RUN), &new.to_string())
        },
    },
    Taken {
        name: "ledger-set",
        takes: ANYWHERE,
        call: |rig, _, _| {
            rig.run(
                &ledger_set(RUN),
                &json!({"key": "prop", "disposition": "later"}).to_string(),
            )
        },
    },
    Taken {
        name: "bound-set",
        takes: ANYWHERE,
        call: |rig, _, _| rig.run(&bound_set(RUN, "prop-bound", &BOUND), ""),
    },
    Taken {
        name: "item-set",
        takes: ANYWHERE,
        call: |rig, _, _| rig.run(&item_set(RUN), &item("prop-item").to_string()),
    },
    Taken {
        name: "apply",
        takes: ANYWHERE,
        call: |rig, _, _| {
            let mut new = row_at("prop-new", INSIDE);
            new["grade"] = json!("no-break");
            new["graded_by"] = json!("human");
            let batch = json!([call(ledger_add(RUN), &new.to_string())]);
            rig.run(&apply(RUN, None), &batch.to_string())
        },
    },
    Taken {
        name: "round-set candidate",
        takes: "ssarrrrrrrrrrrrar",
        call: |rig, _, round| fact_of_round(rig, round, json!({"candidate": sha('9')})),
    },
    Taken {
        name: "round-set base",
        takes: WHILE_NOT_TESTED,
        call: |rig, _, round| fact_of_round(rig, round, json!({"base": "abcdef1"})),
    },
    Taken {
        name: "round-set binary",
        takes: WHILE_NOT_TESTED,
        call: |rig, _, round| fact_of_round(rig, round, json!({"binary": sha256('9')})),
    },
    Taken {
        name: "round-set cross-model",
        takes: WHILE_NOT_TESTED,
        call: |rig, _, round| fact_of_round(rig, round, json!({"cross-model": ["row-clause-b"]})),
    },
    Taken {
        name: "round-set cross-model-void",
        takes: WHILE_NOT_TESTED,
        call: |rig, _, round| {
            fact_of_round(rig, round, json!({"cross-model-void": ["row-clause-b"]}))
        },
    },
    Taken {
        // One more fix cycle: in the latest round, tested, where a fix stage works — not
        // past either bound, and not in a round the human left.
        name: "round-set cycles",
        takes: "rrraaaaaaaaaarrrr",
        call: |rig, read, round| {
            let held = read["rounds"]
                .as_array()
                .expect("the rounds")
                .iter()
                .find(|held| held["round"] == json!(round))
                .map_or(0, |held| held["facts"]["cycles"].as_u64().expect("a count"));
            fact_of_round(rig, round, json!({"cycles": held + 1}))
        },
    },
    Taken {
        // The human's exit from the latest round, once it is tested — at a bound too.
        name: "round-set outcome",
        takes: "rrraaaaaaaaaaaarr",
        call: |rig, _, round| fact_of_round(rig, round, json!({"outcome": "part"})),
    },
    Taken {
        // The go: at the stop after a round — and said again where it stands.
        name: "round-set go",
        takes: "rrrrrrrrrraarrrrr",
        call: |rig, _, round| fact_of_round(rig, round, json!({"go": true})),
    },
    Taken {
        name: "round-set granted",
        takes: "rrrrrrrrrarrrrrrr",
        call: |rig, _, round| fact_of_round(rig, round, json!({"granted": "clause-b"})),
    },
    Taken {
        name: "round-set reverify",
        takes: "rrrrrarrrrrrrrrrr",
        call: |rig, _, round| fact_of_round(rig, round, json!({"reverify": "blocker"})),
    },
    Taken {
        name: "round-set again",
        takes: "rrrrrrrrrrrrrrrar",
        call: |rig, _, round| fact_of_round(rig, round, json!({"again": "test"})),
    },
    Taken {
        name: "round-set ran",
        takes: NOWHERE,
        call: |rig, _, round| fact_of_round(rig, round, json!({"ran": ["row-clause-b a1"]})),
    },
    Taken {
        name: "round-set awaiting",
        takes: NOWHERE,
        call: |rig, _, round| fact_of_round(rig, round, json!({"awaiting": ["prop ungraded 1"]})),
    },
    Taken {
        name: "round-set recorded",
        takes: NOWHERE,
        call: |rig, _, round| fact_of_round(rig, round, json!({"recorded": ["test a1"]})),
    },
    Taken {
        name: "run-set stop",
        takes: AT_THE_OPENING,
        call: |rig, read, _| {
            let other = if read["facts"]["stop"] == "every-round" {
                "at-the-bound"
            } else {
                "every-round"
            };
            fact_of_run(rig, json!({"stop": other}))
        },
    },
    Taken {
        name: "run-set previous",
        takes: AT_THE_OPENING,
        call: |rig, _, _| fact_of_run(rig, json!({"previous": "9.9.9"})),
    },
    Taken {
        name: "run-set previous-commit",
        takes: AT_THE_OPENING,
        call: |rig, _, _| fact_of_run(rig, json!({"previous-commit": sha('f')})),
    },
    Taken {
        name: "run-set scope",
        takes: AT_THE_OPENING,
        call: |rig, _, _| fact_of_run(rig, json!({"scope": "everything"})),
    },
    Taken {
        name: "run-set clauses",
        takes: AT_THE_OPENING,
        call: |rig, _, _| fact_of_run(rig, json!({"clauses": ["another-clause"]})),
    },
    Taken {
        // The bound across rounds: at the opening and before any round — and raised only
        // at the stop it names.
        name: "run-set rounds",
        takes: "aarrrrrrrrrrrarrr",
        call: |rig, read, _| {
            let held = read["facts"]["rounds"].as_u64().unwrap_or(4);
            fact_of_run(rig, json!({"rounds": held + 1}))
        },
    },
    Taken {
        // The bound on a round's cycles: the same, at its own stop.
        name: "run-set cycles",
        takes: "aarrrrrrrrrrrrarr",
        call: |rig, _, _| fact_of_run(rig, json!({"cycles": 4})),
    },
    Taken {
        name: "run-set findings",
        takes: NOWHERE,
        call: |rig, _, _| fact_of_run(rig, json!({"findings": ["prop"]})),
    },
];

/// The writes that name a round, for the round AFTER the latest one the run has begun.
const NEXT: &[Taken] = &[
    Taken {
        name: "report",
        takes: WHERE_A_ROUND_BEGINS,
        call: written_report,
    },
    Taken {
        name: "scope-set",
        takes: WHERE_A_ROUND_BEGINS,
        call: written_scope,
    },
    Taken {
        name: "triage-set",
        takes: WHERE_A_ROUND_BEGINS_UNSCOPED,
        call: written_triage,
    },
    Taken {
        name: "result-set",
        takes: WHERE_A_ROUND_BEGINS_UNSCOPED,
        call: written_result,
    },
    Taken {
        name: "gate-set",
        takes: WHERE_A_ROUND_BEGINS,
        call: written_gate,
    },
    Taken {
        name: "round-set base",
        takes: WHERE_A_ROUND_BEGINS,
        call: |rig, _, round| fact_of_round(rig, round, json!({"base": "abcdef1"})),
    },
];

/// What a refusal of the script lists after `opens`, up to the next ` —` or the line's end.
fn listed_in(seen: &Seen, opens: &str) -> BTreeSet<String> {
    let (_, rest) = seen
        .stderr
        .split_once(opens)
        .unwrap_or_else(|| panic!("the refusal lists them after `{opens}`: {}", seen.stderr));
    rest.trim_end()
        .split(" — ")
        .next()
        .expect("a list")
        .split(", ")
        .map(str::to_owned)
        .collect()
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("create a directory");
    for entry in fs::read_dir(from).expect("read a directory") {
        let entry = entry.expect("an entry");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("its type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy a file");
        }
    }
}

/// **A recorded fact is accepted only where the state asks for it** (the review's `F7` is
/// the case in hand; the repair plan's change C5). The table is the script's own —
/// `WRITERS`, `ROUND_TAKES`, `RUN_TAKES` — and this arm holds it cell by cell: EVERY
/// writer, and EVERY fact of a round's record and of the run's, driven at EVERY position
/// of [`PLACES`], each cell's expectation written by hand in the row. And the rows are
/// held to the script's own lists — its subcommands, and the facts its two fact writers
/// name — so a writer or a fact added later has a row here, or this arm is red; and the
/// script itself takes no call while its own table lacks one.
#[test]
fn every_writer_and_every_fact_is_taken_only_where_the_state_asks_for_it() {
    // The rows are the script's own lists, whole.
    let rig = Rig::new("table-lists");
    let named: BTreeSet<String> = subcommands(&rig).into_iter().collect();
    let facts_of = |args: Vec<String>, opens: &str| {
        let seen = rig.run(&args, "{}");
        seen.refused(BAD_VALUE, "facts that say nothing");
        listed_in(&seen, opens)
    };
    let round_facts = facts_of(round_set(RUN, "1"), "naming at least one of ");
    let run_facts = facts_of(run_set(RUN), "naming at least one of ");
    assert!(
        round_facts.len() >= 14 && run_facts.len() >= 8,
        "the script lists its facts: {round_facts:?} {run_facts:?}"
    );
    let rows: BTreeSet<String> = HERE.iter().map(|write| write.name.to_owned()).collect();
    let mut owed: BTreeSet<String> = named
        .iter()
        .filter(|name| {
            !READERS.contains(&name.as_str())
                && !KEEPERS.contains(&name.as_str())
                && !["round-set", "run-set"].contains(&name.as_str())
        })
        .cloned()
        .collect();
    owed.extend(round_facts.iter().map(|fact| format!("round-set {fact}")));
    owed.extend(run_facts.iter().map(|fact| format!("run-set {fact}")));
    assert_eq!(
        rows, owed,
        "the table's rows (left) are every writer and every fact the script names (right)"
    );
    assert_eq!(HERE.len(), rows.len(), "and no row stands twice");
    for write in HERE.iter().chain(NEXT) {
        assert!(
            write.takes.len() == PLACES.len()
                && write.takes.chars().all(|cell| "ars".contains(cell)),
            "`{}` has a cell per place",
            write.name
        );
    }

    // The table, cell by cell: a thread per place, the place stood up once and put back
    // as it was before every write.
    let cells: usize = std::thread::scope(|threads| {
        let workers: Vec<_> = (0..PLACES.len())
            .map(|n| {
                threads.spawn(move || {
                    let rig = place(n);
                    let read = rig.state();
                    let (next, stop, begins) = PLACE_SAYS[n];
                    assert_eq!(
                        json!([read["next"], read["stop"]["why"], read["begins"]]),
                        json!([next, stop, begins]),
                        "the place `{}`: {read}",
                        PLACES[n]
                    );
                    let pristine = rig.tmp.join("pristine");
                    copy_tree(&rig.run_dir(), &pristine);
                    let here = usize::try_from(read["round"].as_u64().unwrap_or(1))
                        .expect("a round");
                    let mut driven = 0;
                    for (writes, round) in [(HERE, here), (NEXT, here + 1)] {
                        for write in writes {
                            fs::remove_dir_all(rig.run_dir()).expect("clear the run");
                            copy_tree(&pristine, &rig.run_dir());
                            let before = rig.snapshot();
                            let seen = (write.call)(&rig, &read, round);
                            let what = format!(
                                "`{}` for round {round} at `{}`",
                                write.name, PLACES[n]
                            );
                            match write.takes.as_bytes()[n] {
                                b'a' => {
                                    seen.must(OK, &what);
                                }
                                b's' => {
                                    seen.refused(NO_SCOPE, &what);
                                }
                                _ => {
                                    assert!(
                                        [Some(BAD_VALUE), Some(EXISTS)].contains(&seen.code),
                                        "{what}: refused as not asked for, or as written once: {seen:?}"
                                    );
                                    assert_eq!(rig.snapshot(), before, "{what}: nothing written");
                                }
                            }
                            driven += 1;
                        }
                    }
                    driven
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().expect("a place's thread"))
            .sum()
    });
    assert_eq!(cells, (HERE.len() + NEXT.len()) * PLACES.len());

    // COVERED BY CONSTRUCTION: a fact or a subcommand added to the script without its row
    // in the script's own table is no call the script takes — every call says so.
    for (from, to, names) in [
        (
            "ROUND_FACTS = (\"candidate\",",
            "ROUND_FACTS = (\"landed\", \"candidate\",",
            "landed",
        ),
        (
            "RUN_FACTS = (\"stop\",",
            "RUN_FACTS = (\"laps\", \"stop\",",
            "laps",
        ),
        (
            "    command(\"recover\", recover_, [])\n",
            "    command(\"recover\", recover_, [])\n    command(\"forget\", recover_, [])\n",
            "forget",
        ),
    ] {
        let other = Rig::new(&format!("table-{names}"));
        let script = other.root.join(SCRIPT);
        let text = fs::read_to_string(&script).expect("the rig's script");
        assert!(text.contains(from), "the script's own list: `{from}`");
        fs::write(&script, text.replace(from, to)).expect("add one without its row");
        for args in [state(RUN), ledger_add(RUN), run_set(RUN)] {
            let seen = other.run(&args, &row("audit-f3").to_string());
            seen.refused(FAULT, "a call of a script whose table lacks a row");
            assert!(
                seen.stderr.contains(names) && seen.stderr.contains("writer x state table"),
                "it names what has no row: {}",
                seen.stderr
            );
        }
    }
}
