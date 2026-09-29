//! **A refusal is homed with a trigger, or it is an untracked punt.**
//!
//! The M49 Settle refused three frozen methodology-doctype bumps at **leg 2** of the
//! wave's razor — the beneficiary of each is *our* document shape, so the corpus changes
//! and the schema does not ([settle-record.md](../../../completions/artifacts/M49/settle-record.md)
//! → the Tier 3 disposition table). The wave's baseline ledger then had to say so per row,
//! and the first pass said something else: all three `UNPINNED` reasons called the bump
//! **owed** and named *increment 12 / T3* as the task carrying it into
//! `implementation/decisions-pending.md`. That task's own decomposition scoped itself to
//! *"a sweep over the eleven increment entries"* and recorded that *"the M49 deferrals the
//! Settle named are already in the ledger"* — so it neither carried nor could have carried
//! them, and it landed no such entry. Three owed items pointed at a landing that does not
//! exist, in the artifact the human's 1.0.0 gate reads.
//!
//! This suite fences the repair on `ledger_record_truth.rs`'s precedent — these are
//! record-content assertions because the deliverable *is* the record — and it takes its
//! subject from the settle record rather than from a hand-written list of three, so a
//! fourth doctype joining that refusal joins this fence with it:
//!
//!   * the three doctypes are **read out of** the Settle's own `OUT — leg 2` row;
//!   * each one's baseline-ledger disposition names the leg the refusal stands on and
//!     stops claiming an in-wave task will land it;
//!   * one `decisions-pending.md` entry names all three and carries a `*Trigger:*`, which
//!     is the deferral discipline's own rule ([CLAUDE.md](../../../CLAUDE.md) → *A deferral
//!     left only in prose has no trigger and will be forgotten*);
//!   * and the manifest is asked, rather than believed, for the *did not ship* half — so
//!     the day one of the three bumps actually lands, this fence reddens and the prose
//!     built on its absence is re-read instead of silently rotting.

use std::fs;
use std::path::{Path, PathBuf};

const LEDGER: &str = "completions/artifacts/M49/baseline-ledger.md";
const SETTLE: &str = "completions/artifacts/M49/settle-record.md";
const PENDING: &str = "implementation/decisions-pending.md";
const MANIFEST: &str = cli::pack_path!(methodology, "config/schema-manifest.yaml");

fn repo_file(rel: &str) -> String {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .join(rel);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("{rel} must exist: {path:?}"))
}

/// The subject set, **derived**: the doctypes named in the first cell of the Settle's
/// `OUT — leg 2` disposition row, paired with the version the row says they stay at.
///
/// The row reads ``| **`deferral-ledger` 2→3 · `vision` 1→2 · `idea` 1→2** | **OUT — leg
/// 2.** … |``, so each `·`-separated item carries the doctype in backticks and the bump it
/// refuses as `<from>→<to>`; `<from>` is what the manifest must still read.
fn leg_two_refusals() -> Vec<(String, u32)> {
    let settle = repo_file(SETTLE);
    let row = settle
        .lines()
        .find(|line| line.starts_with("| ") && line.contains("OUT — leg 2"))
        .unwrap_or_else(|| {
            panic!("{SETTLE} must carry the Tier 3 `OUT — leg 2` disposition row — the subject of this fence")
        });
    let cell = row
        .split('|')
        .nth(1)
        .expect("a table row has a first cell")
        .trim()
        .trim_matches('*');

    let refusals: Vec<(String, u32)> = cell
        .split('·')
        .filter_map(|item| {
            let doctype = item.split('`').nth(1)?.to_string();
            let bump = item.rsplit('`').next()?.trim();
            let from: u32 = bump.split('→').next()?.trim().parse().ok()?;
            Some((doctype, from))
        })
        .collect();

    assert!(
        refusals.len() >= 3,
        "the `OUT — leg 2` row must name at least the three refused bumps this fence \
         exists for; parsed {refusals:?} out of {cell:?}"
    );
    refusals
}

/// The `- **<id>** · …` disposition line whose reason names `doctype`, in the ledger's
/// disposition section (the Tier 3 rows are `T3-*`).
fn disposition_for<'a>(ledger: &'a str, doctype: &str) -> &'a str {
    let needle = format!("`{doctype}`");
    ledger
        .lines()
        .find(|line| {
            line.starts_with("- **T3-")
                && line.contains("UNPINNED:")
                && line.contains(&needle)
                && line.contains("did not ship")
        })
        .unwrap_or_else(|| {
            panic!("{LEDGER} must carry an `UNPINNED` disposition for the refused `{doctype}` bump")
        })
}

/// **No disposition punts an owed item to a task inside the wave that wrote it.** The
/// dispositions were authored at the wave's close; a task of that same wave cannot be a
/// future landing, so naming one is false by construction.
#[test]
fn no_leg_two_disposition_names_an_in_wave_task_as_its_landing() {
    let ledger = repo_file(LEDGER);
    for (doctype, _) in leg_two_refusals() {
        let line = disposition_for(&ledger, &doctype);
        let lowered = line.to_lowercase();
        for punt in ["increment 12", "m49 increment"] {
            assert!(
                !lowered.contains(punt),
                "{LEDGER}'s `{doctype}` disposition names {punt:?} as what carries it. \
                 Increment 12 / T3 scoped itself to the eleven increment entries and \
                 landed no such entry, so this is a landing that does not exist — name \
                 the refusal that actually holds the row instead:\n{line}"
            );
        }
    }
}

/// **Each row names the leg its refusal stands on, and the record of that refusal.** The
/// bump is not owed-and-unscheduled; it was decided against, and the row has to say which.
#[test]
fn every_leg_two_disposition_names_the_refusal_that_holds_it() {
    let ledger = repo_file(LEDGER);
    for (doctype, _) in leg_two_refusals() {
        let line = disposition_for(&ledger, &doctype);
        assert!(
            line.contains("leg 2"),
            "{LEDGER}'s `{doctype}` disposition must name the leg the Settle refused it \
             at, so the row states a decision rather than an unscheduled debt:\n{line}"
        );
        assert!(
            line.contains("settle-record.md"),
            "{LEDGER}'s `{doctype}` disposition must cite the record of that refusal:\n{line}"
        );
        assert!(
            line.contains("decisions-pending.md"),
            "{LEDGER}'s `{doctype}` disposition must name the home where the refusal's \
             own trigger is written, so a reader can reach it:\n{line}"
        );
    }
}

/// **One `decisions-pending.md` entry covers all three, and it carries a written trigger.**
/// The ledger's own rule: a deferral or refusal that lives only in prose has no trigger and
/// will be forgotten.
#[test]
fn the_leg_two_refusal_is_homed_with_one_written_trigger() {
    let pending = repo_file(PENDING);
    let refusals = leg_two_refusals();

    let entry = pending
        .lines()
        .find(|line| {
            line.starts_with("- **")
                && refusals
                    .iter()
                    .all(|(doctype, _)| line.contains(&format!("`{doctype}`")))
        })
        .unwrap_or_else(|| {
            panic!(
                "{PENDING} must carry one entry naming every doctype the Settle refused at \
                 leg 2 ({:?}) — one home for one refusal, not three scattered notes",
                refusals.iter().map(|(d, _)| d).collect::<Vec<_>>()
            )
        });

    assert!(
        entry.contains("*Trigger:*"),
        "{PENDING}'s leg-2 entry must carry a written `*Trigger:*` — the condition that \
         re-opens it — or it is the untracked punt this file exists to prevent:\n{entry}"
    );
}

/// **The *did not ship* half is asked of the manifest, not believed.** Every refused bump's
/// doctype still stands at the version the Settle's row says it stays at — which is what
/// makes "the manifest standing still is the decision holding" a checked claim, and what
/// reddens this fence on the day one of the three actually lands.
#[test]
fn the_manifest_still_stands_where_the_refusal_left_it() {
    let manifest =
        fs::read_to_string(MANIFEST).unwrap_or_else(|e| panic!("{MANIFEST} must exist: {e}"));
    for (doctype, from) in leg_two_refusals() {
        let entry = format!("- type: {doctype}\n");
        let at = manifest
            .find(&entry)
            .unwrap_or_else(|| panic!("{MANIFEST} must carry an entry for `{doctype}`"));
        let version_line = manifest[at + entry.len()..]
            .lines()
            .find(|line| line.trim_start().starts_with("schema-version:"))
            .unwrap_or_else(|| panic!("`{doctype}`'s manifest entry must carry a schema-version"));
        let shipped: u32 = version_line
            .trim()
            .trim_start_matches("schema-version:")
            .trim()
            .parse()
            .expect("schema-version is a number");
        assert_eq!(
            shipped, from,
            "`{doctype}` reads schema-version {shipped}, but the leg-2 refusal — and every \
             disposition resting on it — is written for {from}. A bump landed: re-read the \
             ledger rows and {PENDING}'s leg-2 entry rather than letting them rot"
        );
    }
}
