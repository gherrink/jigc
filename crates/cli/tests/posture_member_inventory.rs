//! **The two prose homes of the posture family state its whole member inventory**
//! (M52 Increment 3, T7).
//!
//! `repo.operation-in-progress` shipped with its member set written out by hand in two
//! design docs — [`finalize.md`](../../../design/finalize.md) → 1. Preflight and
//! [`validation.md`](../../../design/validation.md) → the M51 Increment 2 registration
//! table — as *"a merge, rebase or bisect"* over three markers. The M52 baseline drove
//! what a hand-written member list costs: a clean `git merge --squash` writes
//! **`SQUASH_MSG` and nothing else**, a conflicted `git stash pop` writes **no marker at
//! all**, and `rebase-apply/` is written by **two** operations — so the three-marker list
//! named `git am` *a rebase* and routed it at `git rebase --abort`, which git refuses at
//! exit 128. [`InProgress::ALL`] now carries nine members with a per-variant `detect`,
//! noun and abandoning command, and this suite is the fence that keeps both prose homes
//! equal to it.
//!
//! **The mold is `crates/cli/tests/doctype_map_versions.rs`' — read the registry, assert
//! the prose.** The subject is **derived, never listed**: the expected inventory is
//! [`InProgress::ALL`] mapped through the shipped [`InProgress::noun`] and
//! [`InProgress::abandon`], so a tenth member reddens both homes until both name it, and
//! a row dropped from either home reddens naming the member it dropped. Neither direction
//! is a spell-check: the route column is the **runnable** half, and
//! `crates/cli/tests/repo_posture.rs` runs every one of those commands out of the emitted
//! route and asserts git accepts it, so a doc row and a live route cannot drift apart
//! without one of the two suites saying so.
//!
//! **Why the inventory is stated twice at all**, against this repo's *cross-reference,
//! never restate* rule: the two homes answer different questions — `finalize.md` states
//! what the **door** refuses and the route it names, `validation.md` registers the
//! **finding** and its severity class — and both were already stating the member set,
//! wrongly and independently, which is the condition the rule exists to prevent. The
//! answer taken here is the shipped one from `design/surface-contract.md`'s error-code
//! mirror: keep the second statement and **fence it**, so the two homes and the binary
//! cannot disagree. Only the two fenced columns are duplicated; the *why* of each member
//! lives in `validation.md`'s third column alone.

use cli::repo::InProgress;
use std::fs;
use std::path::{Path, PathBuf};

/// The two prose homes, enumerated rather than globbed — a sweep of every `.md` would
/// reach the dated records, whose job is to state the world as it was.
const HOMES: [&str; 2] = ["design/finalize.md", "design/validation.md"];

/// The inventory table's first two columns, verbatim. Matched as a **prefix** so a home
/// may carry further columns of its own prose: `validation.md` explains each member's
/// evidence in a third, and nothing here reads it.
const TABLE_HEADER: &str = "| the operation | the route abandons it with |";

/// The family's own statement, which is what the widening actually changed: the member
/// set is *the operations git can leave un-concluded*, never a list of markers.
const FAMILY_STATEMENT: &str = "any operation git can leave un-concluded";

/// The declared deferral, written **at the doc** and not only in the wave's ledger
/// (`completions/artifacts/M52/settle-record.md` → §16): every marker fact in the table
/// is a property of one git.
const DEFERRAL_SUBJECT: &str = "git 2.54.0";

/// …and its reopening trigger, verbatim in both homes. A bound with no trigger is a
/// silence with a paragraph in front of it.
const DEFERRAL_TRIGGER: &str =
    "the first CI or adopter report on another git major/minor where a marker cell diverges";

/// The claim struck at this task, and the shape that must not come back anywhere under
/// `design/`: a count of the family's own vocabulary standing in for a count of the
/// repository states a caller can reach.
const STRUCK_CLAIM: &str = "three of four";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the repo root sits two levels above crates/cli")
        .to_path_buf()
}

fn read(rel: &str) -> String {
    fs::read_to_string(repo_root().join(rel)).unwrap_or_else(|e| panic!("{rel} is readable: {e}"))
}

/// The inventory every home must state, in probe order: the member's noun and the command
/// its route names to abandon it. **Derived**, so the fence has no list of its own to go
/// stale.
fn expected() -> Vec<(String, String)> {
    InProgress::ALL
        .iter()
        .map(|op| (op.noun().to_string(), op.abandon().to_string()))
        .collect()
}

/// Is this a markdown table separator (`|---|---|`, with or without alignment colons)?
fn is_separator(line: &str) -> bool {
    line.starts_with('|')
        && line.chars().all(|c| matches!(c, '|' | '-' | ':' | ' '))
        && line.contains('-')
}

/// The cells of one table row, outer pipes dropped.
fn cells(line: &str) -> Vec<String> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

/// **The inventory rows `home` states**, in the order it states them.
///
/// Exactly one table per home — a second copy is how a stale inventory hides behind a
/// current one, so two headers is a failure rather than a choice of which to read.
fn stated_rows(home: &str, body: &str) -> Vec<(String, String)> {
    let lines: Vec<&str> = body.lines().collect();
    let headers: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with(TABLE_HEADER))
        .map(|(at, _)| at)
        .collect();
    assert_eq!(
        headers.len(),
        1,
        "{home} must carry exactly ONE posture-member inventory whose header begins \
         `{TABLE_HEADER}` — found {}. The member set is `cli::repo::InProgress::ALL`, and a \
         second table is how a stale copy of it survives beside a current one",
        headers.len()
    );
    let header = headers[0];
    let separator = lines
        .get(header + 1)
        .map(|line| line.trim())
        .unwrap_or_default();
    assert!(
        is_separator(separator),
        "{home}'s inventory header must be followed by a table separator, found {separator:?}"
    );
    lines[header + 2..]
        .iter()
        .map(|line| line.trim())
        .take_while(|line| line.starts_with('|'))
        .map(|line| {
            let cells = cells(line);
            assert!(
                cells.len() >= 2,
                "{home}'s inventory row {line:?} must carry at least the operation and its \
                 abandoning command"
            );
            (
                cells[0].clone(),
                cells[1].trim_matches('`').trim().to_string(),
            )
        })
        .collect()
}

/// **Arm 1 — both homes state every member of the family, and only its members.**
///
/// Equality in both directions and in probe order: a row dropped from either home names
/// the member it dropped, a member minted in `cli::repo` with no row names itself, and a
/// row with nothing behind it names itself too.
#[test]
fn both_prose_homes_state_the_whole_operation_inventory() {
    let expected = expected();
    assert_eq!(
        expected.len(),
        InProgress::ALL.len(),
        "the derivation must not collapse two members onto one row"
    );
    let mut broken = Vec::new();
    for home in HOMES {
        let stated = stated_rows(home, &read(home));
        if stated == expected {
            continue;
        }
        let missing: Vec<&(String, String)> = expected
            .iter()
            .filter(|row| !stated.contains(row))
            .collect();
        let unexpected: Vec<&(String, String)> = stated
            .iter()
            .filter(|row| !expected.contains(row))
            .collect();
        broken.push(format!(
            "{home}: missing {missing:?}; not in `InProgress::ALL` {unexpected:?}; \
             stated {stated:?}"
        ));
    }
    assert!(
        broken.is_empty(),
        "every prose home of the posture family states `cli::repo::InProgress::ALL` \
         member-for-member, in probe order, with each member's own abandoning command — \
         the three-marker list this replaced named `git am` a rebase and routed it at a \
         command git refuses at exit 128. Broken at: {broken:#?}"
    );
}

/// **Arm 2 — the family is stated as *any un-concluded operation*, and the git the
/// inventory was driven on is declared with its reopening trigger.**
///
/// The first half is what the widening changed; the second is the bound that makes the
/// first honest, and it is owed **at the doc** rather than only in the wave's settle
/// record.
#[test]
fn both_prose_homes_state_the_family_and_carry_the_git_version_deferral() {
    let mut broken = Vec::new();
    for home in HOMES {
        let body = read(home);
        for owed in [FAMILY_STATEMENT, DEFERRAL_SUBJECT, DEFERRAL_TRIGGER] {
            if !body.contains(owed) {
                broken.push(format!("{home} does not state {owed:?}"));
            }
        }
    }
    assert!(
        broken.is_empty(),
        "both homes state the family as *{FAMILY_STATEMENT}* and declare that every marker \
         fact in the inventory is a property of {DEFERRAL_SUBJECT}, with its reopening \
         trigger written out — {broken:#?}"
    );
}

/// **Arm 3 — the struck claim does not come back.**
///
/// `design/worked-examples.md` carried flow 52's honest bound as *"three of four driven
/// members"* — a count of `PostureMember` variants standing in for a count of the
/// repository states a caller can reach. It is struck with its datum at T7; this arm is
/// what keeps it struck, across every `design/` doc rather than the one that carried it.
#[test]
fn no_design_doc_counts_the_family_by_its_own_vocabulary() {
    let design = repo_root().join("design");
    let mut carriers = Vec::new();
    let mut scanned = 0usize;
    for entry in fs::read_dir(&design).expect("design/ is readable") {
        let path = entry.expect("a design/ entry").path();
        if path.extension().is_none_or(|ext| ext != "md") {
            continue;
        }
        scanned += 1;
        let body = fs::read_to_string(&path).expect("a design/ doc is readable");
        if body.contains(STRUCK_CLAIM) {
            carriers.push(path.display().to_string());
        }
    }
    assert!(
        scanned > 1,
        "the scan must reach the design/ part-docs; it read {scanned}"
    );
    assert!(
        carriers.is_empty(),
        "no `design/` doc states the posture family's size as {STRUCK_CLAIM:?} — the family \
         has three members and the failure the bound was about is a count of repository \
         states, of which the baseline drove eleven. Carried at: {carriers:#?}"
    );
}
