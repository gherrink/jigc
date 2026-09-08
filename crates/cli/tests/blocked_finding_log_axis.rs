//! **A refusal that names itself on the surface names itself in the log — at every door**
//! (M50 completion audit, Finding 2 part 3).
//!
//! `render::finding_error` exists so a verb whose only failure channel is the operational
//! funnel still refuses with an identity: it wraps the whole [`engine::finding::Finding`] in
//! a `render::BlockedFinding` carrier, and M49 Increment 11 / T4 states the reason in as many
//! words — *"so a dispatch handler can log the identity it prints"*, because before it every
//! such refusal was `exit 1, finding_codes: [], error_code: null` in the invocation log,
//! indistinguishable from every other.
//!
//! **That promise was installed at two hand-listed handlers.** `render::blocked_finding` had
//! exactly two production callers — `cli::run_rename` and `task discard`'s arm — so
//! `jigc rename 'research:../../x'` logged `store.malformed-slug` while
//! `jigc doc show 'adr:../../x'`, driven at the same corpus in the same second, logged
//! nothing. One code, logged at one door of its family and silently absent at the rest: *a
//! guard that has to ask to be called is not a guard*
//! ([dev-workflow.md](../../../implementation/dev-workflow.md) → *a grep is not a fence*).
//!
//! The axis is **not** "the four codes M50 minted" — it is **every production site that
//! renders an operational error**, because that is where a carried finding is either read
//! back or dropped. So the two arms below are:
//!
//! 1. the **source fence** — `render::operational_error` has no production caller outside
//!    the one funnel that pairs it with the [`Outcome`](crate::support) it returns, so a
//!    door added tomorrow cannot re-introduce the drop by forgetting to ask;
//! 2. the **driven** arm — one row per verb family, through the real binary with the
//!    `invocation-log` knob ON, asserting the log record carries the code the surface
//!    printed. The two doors that already worked ride the same table as controls, so the
//!    fix is visible as *the table going uniform* rather than as new rows appearing.
//!
//! **What this suite deliberately does not claim.** The `--format json` envelope at these
//! doors is still the flattened `{"error": "…"}`, not the findings envelope — the declared
//! bound on `render::finding_error` and
//! `design/command-output-contract.md` → *the complement*. This suite is about the log
//! record, which is the half the contract already promised and did not deliver.

use std::path::{Path, PathBuf};

use crate::support::rust_source;
use crate::support::trial_corpus::{State, TrialCorpus};

/// One door of the axis: the argv that provokes a flattened blocking refusal, the finding
/// code the surface prints for it, and why the row is in the table.
struct DoorRow {
    /// The argv after `jigc`.
    argv: &'static [&'static str],
    /// The finding code the refusal carries.
    code: &'static str,
    /// Why this row is in the table — the door family it stands for.
    why: &'static str,
}

/// One row per verb family that can raise a `render::BlockedFinding`, plus the two doors that
/// already read it back at HEAD (the controls).
///
/// The codes are M50's four new ones (`work-unit.malformed-id`, `store.malformed-slug`,
/// `config.workbench-root`, `config.unusable-root`) because they are the wave's own, but the
/// property under test is the carrier's, not theirs: every one of these rows reaches the same
/// funnel, and the source fence beside this table is what makes the table a witness rather
/// than the guarantee.
const DOORS: &[DoorRow] = &[
    DoorRow {
        argv: &["task", "validate", ""],
        code: "work-unit.malformed-id",
        why: "the task verb family's resolve seam",
    },
    DoorRow {
        argv: &["task", "diff", ""],
        code: "work-unit.malformed-id",
        why: "a second task verb — the seam is shared, the dispatch arm is not",
    },
    DoorRow {
        argv: &["task", "finalize", ""],
        code: "work-unit.malformed-id",
        why: "the committing task door, whose failure path is `surface_commit_rejection`",
    },
    DoorRow {
        argv: &["task", "discard", ""],
        code: "work-unit.malformed-id",
        why: "CONTROL — one of the two doors that read the carrier back at HEAD",
    },
    DoorRow {
        argv: &["doc", "list", "--task", ""],
        code: "work-unit.malformed-id",
        why: "the doc verb family's `--task` resolution",
    },
    DoorRow {
        argv: &["milestone", "list-tasks", ""],
        code: "work-unit.malformed-id",
        why: "the milestone verb family",
    },
    DoorRow {
        argv: &["doc", "show", "adr:../../x"],
        code: "store.malformed-slug",
        why: "the doc read door's address parse",
    },
    DoorRow {
        argv: &["rename", "research:../../x", "--to", "New Title"],
        code: "store.malformed-slug",
        why: "CONTROL — the other door that read the carrier back at HEAD",
    },
    DoorRow {
        argv: &["config", "set", "docs-root", ".jigc"],
        code: "config.workbench-root",
        why: "the config write door — the `config.*` family flattens whole",
    },
    DoorRow {
        argv: &["config", "set", "placement-root", "/tmp/elsewhere"],
        code: "config.unusable-root",
        why: "the config write door's second M50 refusal, on the other root knob",
    },
];

/// The JSONL invocation-log records at `.jigc/logs/invocations.jsonl` (empty when absent).
fn log_records(repo: &Path) -> Vec<serde_json::Value> {
    let path = repo.join(".jigc").join("logs").join("invocations.jsonl");
    match std::fs::read_to_string(&path) {
        Ok(body) => body
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).expect("each log line is valid JSON"))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// **Arm 1 — the driven witness.** Every row's refusal reaches the invocation log carrying the
/// code its own stderr printed.
///
/// The stderr assertion is not decoration: it cross-reads the identity from the surface the
/// agent sees in the *same run*, so the log assertion cannot pass over a code the door never
/// actually printed.
#[test]
fn every_flattened_refusal_records_the_code_it_printed() {
    let corpus = TrialCorpus::build(State::CommittedSingletons);
    let repo = corpus.repo();
    corpus.jigc_ok(&["config", "set", "invocation-log", "true"]);

    let mut missing = Vec::new();
    for row in DOORS {
        let before = log_records(&repo).len();
        let out = corpus.jigc(row.argv);
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        assert!(
            stderr.contains(row.code),
            "`jigc {}` ({}) must print `{}` on stderr; stderr:\n{stderr}",
            row.argv.join(" "),
            row.why,
            row.code,
        );

        let records = log_records(&repo);
        assert_eq!(
            records.len(),
            before + 1,
            "`jigc {}` must append exactly one invocation-log record",
            row.argv.join(" "),
        );
        let record = records.last().expect("one record");
        let codes: Vec<String> = record["finding_codes"]
            .as_array()
            .expect("finding_codes is an array")
            .iter()
            .map(|value| value.as_str().expect("a code is a string").to_string())
            .collect();
        if !codes.iter().any(|code| code == row.code) {
            missing.push(format!(
                "  jigc {} → printed `{}`, logged {codes:?}   ({})",
                row.argv.join(" "),
                row.code,
                row.why,
            ));
        }
    }

    assert!(
        missing.is_empty(),
        "a refusal that names itself on the surface must name itself in the invocation log — \
         `render::finding_error` carries the whole finding for exactly that reason. \
         {} of {} doors dropped it:\n{}",
        missing.len(),
        DOORS.len(),
        missing.join("\n"),
    );
}

/// The `crates/cli/src` directory.
fn cli_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// **Arm 2 — the fence.** `render::operational_error` has no production caller outside the one
/// funnel that pairs it with an [`Outcome`]-returning read of the carrier.
///
/// Arm 1 is a table, and a table is a *finder*: it covers the doors someone remembered. This
/// is the *fence* — it puts the check where membership is decided (the act of rendering an
/// operational error), so a dispatch arm added tomorrow either goes through the funnel or
/// reddens this test. It is the same shape `located_finding_text`'s sweep uses, and it exists
/// because the property it guards was already installed once as a two-entry census.
#[test]
fn operational_error_has_no_production_caller_outside_the_funnel() {
    /// The funnel's own name — the one production fn allowed to render an operational error.
    const FUNNEL: &str = "operational_failure";

    let mut offenders = Vec::new();
    for path in rust_source::rust_files(&cli_src()) {
        let body = std::fs::read_to_string(&path).expect("read a cli source file");
        let code = rust_source::code_only(&body);
        let regions = rust_source::cfg_test_regions(&code);
        for (at, _) in code.match_indices("operational_error(") {
            if rust_source::is_test_domain(&path, &regions, at) {
                continue;
            }
            let owner = rust_source::enclosing_fn(&code, at).unwrap_or("<top level>");
            if owner == FUNNEL || owner == "operational_error" {
                continue;
            }
            let line = code[..at].bytes().filter(|b| *b == b'\n').count() + 1;
            offenders.push(format!(
                "  {}:{line} — in `{owner}`",
                path.file_name().unwrap_or_default().to_string_lossy(),
            ));
        }
    }

    assert!(
        offenders.is_empty(),
        "`render::operational_error` renders a failure but decides no `Outcome`, so a caller \
         that reaches for it directly drops the carried finding's identity from the invocation \
         log — which is exactly how the promise on `render::finding_error` came to be kept at \
         two doors of ~30. Call `invocation_log::{FUNNEL}` instead (it renders the same bytes \
         and returns the outcome). Direct callers:\n{}",
        offenders.join("\n"),
    );
}
