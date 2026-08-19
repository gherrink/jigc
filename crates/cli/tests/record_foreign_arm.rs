//! The record's own fence for M46 Increment 3 — the design of record gains the arm the
//! verb never had, and stops stating the exit condition the increment retired (T3).
//!
//! Increment 3 changed two doors and therefore falsified the record in two directions:
//!
//!   * **`migrate-corpus` stops claiming a file that is not its subject** (T2). The verb
//!     upgrades the *managed* corpus; a never-adopted foreign file at a managed home is
//!     excluded before the fold and reported as the store door's advisory, verbatim.
//!     [corpus-migration.md](../../../design/corpus-migration.md) had **no foreign arm at
//!     all** — the walk section finds the file, and nothing said what the fold then does
//!     with it. That gap is what let the verb block on it for four waves.
//!   * **The store sweep's exit stops calling a never-adopted file harmless** (T1).
//!     `schema-conformance.unadopted-instance` is `render::STORE_EXIT_FLIPS`' fifth
//!     member, so a stock brownfield repo carrying an un-adopted file at a managed home
//!     **exits non-zero**. M42 wrote the opposite here, with a rationale — and that
//!     rationale is **engaged and withdrawn where it was written**, never annotated
//!     around: with the foreign file out of `migrate-corpus`'s blocking set, the store
//!     exit is the only surface left that makes a squatter audible at all.
//!
//! These are doc-content assertions by nature — the deliverable *is* the prose. The
//! behaviour it describes is proven through the real binary elsewhere
//! (`crates/cli/tests/managed_vs_foreign.rs` for the sweep's exit,
//! `crates/cli/tests/migrate_corpus_foreign.rs` for the verb's exclusion axis), which is
//! why this file asserts only that the record says what those suites drive.
//!
//! **The sweep is stated as one thing and asserted as one thing**: every falsified
//! statement is named with the bytes it carried, so a green here means the record no
//! longer carries them — and every replacement is asserted **exactly once**, because a
//! correction restated in two homes is the rot this repo's own cross-reference rule
//! exists to prevent.
//!
//! **The third arm, added on the increment's completion audit: the envelope key T2 minted is
//! declared where every sibling is declared.** T2 put a **new serialized key** (`unadopted`)
//! on the `migrate-corpus` `--format json` report — an envelope the driver contract pins, and
//! whose every prior additive key (`carried-over`, `hook_output`, `hook_file`, `committed`,
//! `copied_in`, `committed_identity`, `milestone`/`no_docs_from`, `checked`) carries its own
//! declaration paragraph in [command-output-contract.md](../../../design/command-output-contract.md)
//! → Evolution posture. It had none — one wave after M48 refused a key on *this very envelope*
//! on exactly that ground (`DECISIONS.md` → 2026-08-13 M48 Increment 9 / T4: *"No envelope key
//! is minted — the additive window is spent"*). The key itself is proven on the wire by
//! `crates/cli/tests/migrate_corpus_foreign.rs`; what was missing was the declaration, and the
//! **premise the discharge closed the window on** — *M48 is the last pre-1.0 wave* — which M46
//! falsified by existing. Both are asserted here, in both homes that carried the premise.
//!
//! `worked-examples.md`'s flow-43 arm 2 is in the sweep although the increment brief's
//! file list omitted it: it states the identical retired green, in the very doc whose
//! acceptance suite T1 had already corrected. Scoping the sweep around it would have
//! left the design of record contradicting its own acceptance test — the masking shape,
//! not a scope boundary (`DECISIONS.md` → 2026-08-19 M46 Increment 3 / T3).

use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root is two levels above crates/cli")
        .to_path_buf()
}

fn read_doc(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("{rel} must exist: {path:?}"))
}

fn count(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

/// The `## <heading>` section body — up to the next `## ` heading or EOF.
fn section<'a>(body: &'a str, heading: &str) -> &'a str {
    let marker = format!("## {heading}\n");
    let start = body
        .find(&marker)
        .unwrap_or_else(|| panic!("the doc must carry a `## {heading}` section"));
    let rest = &body[start + marker.len()..];
    match rest.find("\n## ") {
        Some(end) => &rest[..end],
        None => rest,
    }
}

/// The falsified statements, each with the file it lives in and why it is now false.
/// Named with the bytes they carried so this list cannot quietly become a paraphrase.
const FALSIFIED: &[(&str, &str, &str)] = &[
    (
        "design/validation.md",
        "a foreign brownfield doc never flips it",
        "M42's managed-arm condition decided which CODE fires; since M46 Inc 3 / T1 the \
         exit no longer follows from it",
    ),
    (
        "design/validation.md",
        "| `jigc ingest` / `jigc migrate <path> --as <doctype>` | **0** |",
        "the discriminator table's foreign row: the run's exit flips, though the finding \
         stays advisory and gates nowhere",
    ),
    (
        "design/validation.md",
        "report-only, with three exit-flipping exceptions",
        "the hand-count stood at three while the code carried five; the enumeration is \
         `render::STORE_EXIT_FLIPS`, and prose must point at it rather than re-count",
    ),
    (
        "design/validation.md",
        "only the rename structural-integrity finding and the M42 version-currency break, \
         below, flip the exit",
        "the same stale hand-enumeration, one sentence further down the same paragraph",
    ),
    (
        "design/storage.md",
        "`changelog → changelog/`",
        "R3: `changelog` left the located set at M38 for `placement: { file: CHANGELOG.md }` \
         — the sentence a fixture author reads before planting a squatter",
    ),
    (
        "design/worked-examples.md",
        "the brownfield first run stays GREEN",
        "flow 43's transcript, whose acceptance suite T1 already corrected",
    ),
    (
        "design/worked-examples.md",
        "`validate` **stays exit 0**",
        "flow 43 arm 2's prose, the same retired green",
    ),
];

/// The retired **fact**, keyed as a fact rather than as one of its phrasings.
///
/// `FALSIFIED` above names byte-forms, which is what makes it readable — and what made it
/// maskable: M46 Increment 3 asserted zero hits for the property census's phrasing of the
/// retired exit-0 while the *same doc* stated the identical fact twice more, in the bullet
/// that actually **owns** the property and in the detect flow's brownfield guard. So the
/// retired token is swept over the whole file, and the **only** admissible occurrence is
/// inside the sentence that withdraws it — a withdrawal must be free to quote the bytes it
/// retires (`DECISIONS.md` → 2026-08-19; `design/validation.md` → the managed-arm bullet).
///
/// `(file, retired token, the withdrawal sentence that may carry it, why it is retired)`.
const RETIRED_FACTS: &[(&str, &str, &str, &str)] = &[
    (
        "design/corpus-migration.md",
        "stays exit-0",
        "the *stays exit-0* this row carried is withdrawn",
        "`schema-conformance.unadopted-instance` is `render::STORE_EXIT_FLIPS`' fifth \
         member since M46 Inc 3 / T1 — a never-adopted file at a managed home flips the \
         sweep's exit, in every home this doc states the property",
    ),
    (
        "design/validation.md",
        "stays exit-0",
        "*and stays exit-0* — is withdrawn at M46",
        "the same fact in the doc that owns the exit predicate; keyed here so a future \
         pass cannot re-state it in a second home the way `corpus-migration.md` did",
    ),
    (
        "design/command-output-contract.md",
        "M48 is the last pre-1.0 wave",
        "*M48 is the last pre-1.0 wave* was this paragraph's premise, and it is **withdrawn**",
        "M46 — the combined pre-1.0 wave — follows M48, so a close keyed to a wave NAME was \
         false the day that wave was chartered; the window closes at the 1.0 pin, and until \
         then every spend is declared in its own paragraph",
    ),
    (
        "design/doc-read-surface.md",
        "M48 is the last pre-1.0 wave",
        "*M48 is the last pre-1.0 wave* was its premise here too, and is **withdrawn**",
        "the identical premise in the read side's half of the same discharge — retired in \
         both homes, or the record contradicts itself across a cross-reference it already \
         carries",
    ),
];

/// **The sweep, arm 3 — the retired fact appears nowhere but its own withdrawal.**
///
/// Strip the withdrawal sentences, then the token must be gone: not one phrasing of the
/// fact, every occurrence of it.
#[test]
fn the_retired_fact_appears_nowhere_but_its_withdrawal() {
    for (file, token, withdrawal, why) in RETIRED_FACTS {
        let body = read_doc(file);
        assert_eq!(
            count(&body, withdrawal),
            1,
            "{file} must carry the withdrawal sentence `{withdrawal}` exactly once — a \
             fact is retired where it was written, never annotated around",
        );
        let stripped = body.replace(withdrawal, "");
        assert_eq!(
            count(&stripped, token),
            0,
            "{file} still states the retired fact `{token}` outside its withdrawal — {why}",
        );
    }
}

/// The replacements, each asserted **exactly once** — a correction stated twice is the
/// restatement rot, and a correction stated zero times is the swap never landing.
const REPLACEMENTS: &[(&str, &str)] = &[
    (
        "design/validation.md",
        "not about the exit, and that is a M46 revision of what M42 wrote here",
    ),
    (
        "design/validation.md",
        "| `jigc ingest` / `jigc migrate <path> --as <doctype>` | **flips** (M46) |",
    ),
    (
        "design/validation.md",
        "with the exit-flipping exceptions `render::STORE_EXIT_FLIPS` enumerates",
    ),
    ("design/storage.md", "`changelog` is **not** among them"),
    (
        "design/corpus-migration.md",
        "**gates nowhere**, and **flips the sweep's exit**",
    ),
    (
        "design/corpus-migration.md",
        "## The foreign arm — the file the verb is not for (M46)",
    ),
    ("design/corpus-migration.md", "flips the sweep's own exit"),
    (
        "design/corpus-migration.md",
        "the sweep's verdict on its own run rather than a gate the advisory carries",
    ),
    ("design/worked-examples.md", "# M46: the GREEN is withdrawn"),
    (
        "design/worked-examples.md",
        "`validate` raises **zero** blocking findings",
    ),
];

/// **The sweep, arm 1 — nothing states the retired exit condition.**
#[test]
fn the_record_no_longer_states_the_exit_condition_the_increment_retired() {
    for (file, needle, why) in FALSIFIED {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            0,
            "{file} still carries the falsified statement `{needle}` — {why}",
        );
    }
}

/// **The sweep, arm 2 — each replacement is stated once, in one home.**
#[test]
fn each_replacement_is_stated_exactly_once() {
    for (file, needle) in REPLACEMENTS {
        let body = read_doc(file);
        assert_eq!(
            count(&body, needle),
            1,
            "{file} must state `{needle}` exactly once (cross-reference, never restate)",
        );
    }
}

/// **R1 — the foreign arm exists, and cross-references the discriminator rather than
/// restating it.** The verb's subject is stated once; the classifier's arms stay owned by
/// `validation.md`, which is the property that keeps the two doors from disagreeing again.
#[test]
fn corpus_migration_states_the_foreign_arm_and_defers_the_discriminator_to_validation() {
    let body = read_doc("design/corpus-migration.md");
    let arm = section(
        &body,
        "The foreign arm — the file the verb is not for (M46)",
    );

    for needle in [
        // The verb's subject, stated once.
        "upgrades the *managed* corpus",
        // The shipped discriminator, asked before the fold — named, not re-implemented.
        "is_unadopted_foreign",
        // Ownership of the arms stays with the other doc.
        "The managed-vs-foreign discriminator",
        // Reported, never silently skipped, and never in `blocked`.
        "never a silent already-current",
        "`blocked`, whose emptiness *is* the exit rule",
        // The one-producer property that makes the two surfaces agree by construction.
        "verbatim, from the one producer",
    ] {
        assert!(
            arm.contains(needle),
            "the foreign arm must carry `{needle}`; it reads:\n{arm}",
        );
    }

    // It defers the arms rather than restating them: the three-arm table's own conditions
    // live in `validation.md` and must not be copied here.
    for restated in [
        "parses against **no** known schema version",
        "schema-version-ahead",
    ] {
        assert!(
            !arm.contains(restated),
            "the foreign arm restates `{restated}`, which `validation.md` owns",
        );
    }

    // And `validation.md`'s claim about the other verb now names where it is enforced.
    assert!(
        read_doc("design/validation.md")
            .contains("Since M46 that is true of the verb, not only of this route"),
        "validation.md's `never to migrate-corpus` claim must name the verb-side enforcement",
    );
}

/// **The declared behaviour change carries a TRIGGER, not a mention.** A stock brownfield
/// repo that exits 0 on `jigc validate` today exits non-zero once un-adopted files sit at
/// managed homes — which the next trial's protocol must read as a designed change rather
/// than rediscover as a regression. `decisions-pending.md` is the home where an owed thing
/// carries the condition that resurfaces it; a bare sentence anywhere else is the failure
/// mode that file's own preamble names.
#[test]
fn the_declared_behaviour_change_is_keyed_to_the_next_trials_protocol() {
    let body = read_doc("implementation/decisions-pending.md");
    let heading = "### The trial that follows M46 — protocol inputs";
    assert_eq!(
        count(&body, heading),
        1,
        "decisions-pending.md must carry exactly one `{heading}` entry",
    );

    let start = body.find(heading).expect("the heading was just counted");
    let rest = &body[start + heading.len()..];
    let entry = match rest.find("\n### ") {
        Some(end) => &rest[..end],
        None => rest,
    };

    for needle in [
        // The change itself, stated as the protocol will meet it.
        "exits non-zero once un-adopted files sit at managed homes",
        // The trigger — the whole point of this home.
        "*Trigger:*",
        // Named so the protocol can tell a designed change from a regression.
        "a declared behaviour change, not a defect",
    ] {
        assert!(
            entry.contains(needle),
            "the entry must carry `{needle}`; it reads:\n{entry}",
        );
    }
}

/// **The sweep, arm 4 — T2's new envelope key is declared where every sibling is declared.**
///
/// The `migrate-corpus` `--format json` report is governed by the pinned driver contract, and
/// that contract's rule since M48 is that an addition to a pinned envelope is a **declaration**,
/// never a field that merely ships. `unadopted` is that declaration; it must state what the key
/// carries, why the set is not re-derivable from the three arrays already on the envelope, and
/// why it is deliberately not `blocked`.
#[test]
fn the_new_envelope_key_is_declared_where_every_sibling_is_declared() {
    let body = read_doc("design/command-output-contract.md");
    let posture = section(&body, "Evolution posture (declared)");

    for needle in [
        // The declaration itself, in the sibling paragraphs' own form.
        "**The M46 additive key: `unadopted` on the `migrate-corpus` report",
        // What the key carries — the producer, named, so the two surfaces stay one fact.
        "AdoptionInputs::unadopted",
        // Why it is not re-derivable: an excluded file is in none of the three arrays.
        "`migrated`, `already_current` and `blocked`",
        "is in none of them",
        // Why it is deliberately its own key rather than a member of `blocked`.
        "whose emptiness *is* the exit rule",
        // The window is re-keyed to the pin, and the rule that keeps it from being an open
        // ledger until then — which is what this finding is an instance of.
        "an undeclared key on a pinned envelope is a defect, not an addition",
    ] {
        assert!(
            posture.contains(needle),
            "the evolution posture must carry `{needle}` — a key on a pinned envelope is \
             declared where its siblings are, or it is undeclared surface",
        );
    }

    // The discharge's enumeration is M48's spend and stays M48's — the M46 key is declared in
    // its own paragraph, not smuggled into the sentence that closed the previous wave.
    assert_eq!(
        count(
            &body,
            "plus one this paragraph declares as it ships: **`checked`**"
        ),
        1,
        "M48's spend enumeration must survive intact — this fix re-keys the CLOSE, it does \
         not rewrite what an earlier wave declared",
    );
}
