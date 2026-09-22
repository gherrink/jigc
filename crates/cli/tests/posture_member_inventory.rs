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
//! exit 128. [`InProgress::ALL`] now carries a member per operation with a per-variant
//! `detect`, noun and abandoning command, and this suite is the fence that keeps both
//! prose homes equal to it.
//!
//! **The mold is `crates/cli/tests/doctype_map_versions.rs`' — read the registry, assert
//! the prose.** The subject is **derived, never listed**: the expected inventory is
//! [`InProgress::ALL`] mapped through the shipped [`InProgress::noun`] and
//! [`InProgress::abandon`], so a new member reddens both homes until both name it, and
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

/// **The counts of [`InProgress::ALL`] struck at M53 Increment 4's fix**, each paired with
/// the datum that falsified it.
///
/// The increment landed the family's tenth member across three commits, and the one that
/// reworded the family's prose (`f1883722`, *"the posture family's prose homes stop stating
/// a count"*) **deliberately** left the enum's own doc-comments to the commit that would
/// move them: *"the enum's own doc-comment, the variant doc and `ALL`'s probe-order doc
/// stay as they are — they change with the member."* The member landed at `13c621f8` and
/// they did not, so `cli::repo` shipped *"never a menu of nine"* and *"Eight members read
/// … The ninth asks git"* around a `pub const ALL: [InProgress; 10]`, and
/// `repo_posture.rs`' composition arm went on naming the member it was waiting for *the
/// tenth*. Nothing caught it: [`no_design_doc_counts_the_family_by_its_own_vocabulary`]
/// scans `design/`, and both prose homes there are **derived** from the enum by arm 1, so
/// the one place a hand-written count survived was the crate that defines the set.
///
/// **[Extended 2026-09-22 (the independent review of `986d5e0a`, MEDIUM 3).** The M53
/// post-review fix put *"nine of the ten members"* back into `crates/cli/src/repo.rs` — a
/// [`COUNT_HOMES`] member — and this arm stayed green, because the doc-comment **wrapped**
/// between `**nine**` and `of the ten` and interpolated a `` [`InProgress`] `` link before
/// `members`. A literal `contains` is not a fence against prose; it is a fence against one
/// spelling of prose. The scan now normalizes first ([`normalized`]) and matches the claim's
/// words in order with a bounded gap ([`states`]), so the wrap and the link no longer hide
/// it. The count itself was wrong in a second way worth recording here: its stated reason —
/// *`Unborn` cannot occur in a linked worktree* — names a `PostureMember`, **not** an
/// `InProgress` member, so it subtracted nothing; the reviewer drove the tenth member
/// landing at exit 0 on `1.0.0-rc.17` too.**]**
const STRUCK_COUNTS: [(&str, &str); 6] = [
    (
        "menu of nine",
        "`InProgress`' own doc-comment, three lines above `ALL`'s `[InProgress; 10]`",
    ),
    (
        "Eight members read",
        "`InProgress::detect`'s doc-comment — nine members read the worktree's git dir",
    ),
    (
        "The ninth asks git",
        "that doc-comment's other half — `UnmergedIndex` is the tenth, not the ninth",
    ),
    (
        "nine of the ten members",
        "`abandon_qualifier`'s doc — true the day it was written and one member from false",
    ),
    (
        "tenth member declaring one",
        "`repo_posture.rs`' composition arm, whose *next* member is no longer the tenth",
    ),
    (
        "nine of these cells",
        "`commit_seam_posture.rs`' git-state axis arm — the axis has fifteen refusing \
         cells, so the numeral was a count of a third set again",
    ),
];

/// The homes scanned for them: the module that defines the family, the suite that runs every
/// member's rendered route out of the emitted bytes, and — since the M53 post-review fix put
/// a count in each — the milestone boundary that asks the family about a *second* checkout
/// and the suite that drives it.
///
/// **Widened because the class was, not because the list was short.** The review found the
/// struck count in **six** homes; four are reachable from a file scan of this crate and the
/// `design/` part-docs (arm 3 takes the two `design/` ones), and the sixth is `DECISIONS.md`.
///
/// **`DECISIONS.md` is deliberately not a member**, for the same reason this file is not: the
/// dated log's convention is *strike with the datum that falsifies it*, so a corrected entry
/// **must** carry the false phrase as quoted text, and a scan of it would match the
/// correction. `design/` docs are corrected in place and are therefore scannable; the record
/// is not.
///
/// **And `crates/cli/tests/posture_member_inventory.rs` is not a member** — this file must
/// carry every struck phrase above as a literal, so a scan of it would match itself. Its own
/// overtaken ordinal is reworded rather than fenced, which is stated here rather than left
/// for a reader to notice.
///
/// Each row carries the **token that proves the scan is looking at a home of this family**,
/// per home rather than one shared literal: the two `repo*` homes and the boundary name the
/// enum, and `commit_seam_posture.rs` — which drives the family through the real binary and
/// never names the type — names the finding code instead. A home that stops carrying its
/// token reddens rather than passing vacuously.
const COUNT_HOMES: [(&str, &str); 4] = [
    ("crates/cli/src/repo.rs", "InProgress"),
    ("crates/cli/tests/repo_posture.rs", "InProgress"),
    ("crates/cli/src/milestone.rs", "InProgress"),
    (
        "crates/cli/tests/commit_seam_posture.rs",
        "repo.operation-in-progress",
    ),
];

/// `body`, as the count fence compares it: doc-comment markers dropped, emphasis / code-span
/// / doc-link / strikethrough punctuation removed, and **all whitespace collapsed**, so a
/// claim the source wrapped across two lines is one string again.
///
/// The wrap is not hypothetical — it is how the M53 post-review fix put a struck count back
/// into `crates/cli/src/repo.rs` under a green fence.
fn normalized(body: &str) -> String {
    let mut joined = String::with_capacity(body.len());
    for line in body.lines() {
        let line = line.trim_start();
        let line = line
            .strip_prefix("//!")
            .or_else(|| line.strip_prefix("///"))
            .or_else(|| line.strip_prefix("//"))
            .unwrap_or(line);
        joined.push_str(line);
        joined.push(' ');
    }
    joined
        .chars()
        .filter(|c| !matches!(c, '`' | '*' | '[' | ']' | '_' | '~'))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// How many words may sit **between** two consecutive words of a struck claim before the scan
/// stops calling it the same claim.
///
/// Two, which is what the evasion cost: `` **nine**\n//! of the ten [`InProgress`] members ``
/// interpolates exactly one token. A larger window buys nothing and starts matching sentences
/// that merely share vocabulary; zero is the `contains` this replaces.
const MAX_GAP: usize = 2;

/// Split into comparison words: lowercased, outer ASCII punctuation trimmed, so `ten,` and
/// `nine.` compare as the words they are.
fn words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| {
            word.trim_matches(|c: char| c.is_ascii_punctuation())
                .to_ascii_lowercase()
        })
        .filter(|word| !word.is_empty())
        .collect()
}

/// Whether `body` states `claim` — the claim's words, in order, each within [`MAX_GAP`] words
/// of the last. `body` is normalized first, so the caller hands in raw source.
fn states(body: &str, claim: &str) -> bool {
    let haystack = words(&normalized(body));
    let needle = words(claim);
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    (0..haystack.len()).any(|start| {
        if haystack[start] != needle[0] {
            return false;
        }
        let mut at = start;
        for want in &needle[1..] {
            match (at + 1..=(at + 1 + MAX_GAP).min(haystack.len().saturating_sub(1)))
                .find(|&i| &haystack[i] == want)
            {
                Some(found) => at = found,
                None => return false,
            }
        }
        true
    })
}

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
        // `STRUCK_CLAIM` stays on the **exact** match it shipped with, and the reason is a
        // measurement: run through [`states`]' bounded gap it fires on four `design/` docs,
        // because *three*, *of* and *four* are three ordinary words and a gap-tolerant match
        // over them is a match on vocabulary rather than on a claim. The loose matcher is for
        // the counts below, whose phrases are specific enough to survive it.
        if body.contains(STRUCK_CLAIM) {
            carriers.push(path.display().to_string());
        }
        // …and the counts arm 4 keeps out of the crate. Two of the six homes the M53
        // post-review review found were `design/` part-docs, so the two arms scan for both
        // shapes over the surfaces each already walks rather than one of them being the
        // place a count is allowed to live.
        for (claim, datum) in STRUCK_COUNTS {
            if states(&body, claim) {
                carriers.push(format!("{} states {claim:?} — {datum}", path.display()));
            }
        }
    }
    assert!(
        scanned > 1,
        "the scan must reach the design/ part-docs; it read {scanned}"
    );
    assert!(
        carriers.is_empty(),
        "no `design/` doc states the posture family's size — not as {STRUCK_CLAIM:?} (the \
         family has three members and the failure the bound was about is a count of \
         repository states, of which the baseline drove eleven), and not as any of the \
         counts of `InProgress::ALL` struck with it, which the M53 post-review fix put into \
         two of these docs. Carried at: {carriers:#?}"
    );
}

/// **Arm 4 — no home of the family states a count of it.**
///
/// The sibling of arm 3, one layer in: arm 3 keeps a struck claim out of `design/`, where
/// the inventory is fenced by derivation and a numeral beside it would be the only
/// hand-written statement of the set's size. This arm keeps the same shape out of the two
/// homes the enum's **own crate** carries, which is where M53 Increment 4's tenth member
/// left the last hand-written ones standing (see [`STRUCK_COUNTS`]).
///
/// **The predicate is the phrase, not the numeral.** A blanket ban on spelled numerals in
/// `crates/cli/src/repo.rs` would fire on accurate prose about three other sets this
/// family's size does not move — [`cli::repo::PostureMember`]'s three, the six markers
/// [`InProgress::UncommittedCherryPick`]'s predicate negates, and the members that name no
/// concluding command — and an ordinal naming **one** member is not a statement of the
/// set's size at all (`crates/cli/tests/count_fences.rs`' own rule), which is why
/// [`InProgress::conclude`]'s *"this family's tenth member"* stays.
#[test]
fn no_home_of_the_family_states_a_count_it_can_move() {
    let mut carried = Vec::new();
    for (home, token) in COUNT_HOMES {
        let body = read(home);
        assert!(
            body.contains(token),
            "{home} must be a home of the posture family; it never names `{token}`"
        );
        for (claim, datum) in STRUCK_COUNTS {
            if states(&body, claim) {
                carried.push(format!("{home} states {claim:?} — {datum}"));
            }
        }
    }
    assert!(
        carried.is_empty(),
        "no prose home of the posture family states a count of `cli::repo::InProgress::ALL` \
         — the inventory is fenced by derivation (arm 1) and a numeral beside it is a \
         sentence one member away from being false, which is what the seven prose homes \
         reworded at `f1883722` were reworded for. The family carries {} members today. \
         Carried at: {carried:#?}",
        InProgress::ALL.len()
    );
}
