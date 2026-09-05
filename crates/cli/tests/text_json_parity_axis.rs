//! M48 Increment 7 / T4 — **the standing text/JSON parity fence** (`DECISIONS.md` →
//! 2026-08-13 the Settle, *the pre-1.0 additive-key window*;
//! [command-output-contract.md](../../../design/command-output-contract.md) → Evolution
//! posture; `implementation/pinning.md` §2 — a contract property suite).
//!
//! The rule the Settle fixed, so the sweep is plannable and testable:
//!
//! > **A value the human/agent text already prints, but the `--format json` envelope
//! > withholds, is a gap.**
//!
//! Two locked contracts permit **additive keys pre-1.0 only** — from the 1.0 pin the
//! shape evolves solely by an explicitly versioned extension. M48 is the last pre-1.0
//! wave, so the window closes here, and it closes **fenced**: this suite is a standing
//! fence, not a one-time census, so a verb added after 1.0 cannot quietly print a fact it
//! withholds from its driver.
//!
//! **The enumeration is the clap tree** — the axis `format_json_success_axis.rs`
//! established: *every machine-output surface* enumerates as *every leaf verb that speaks
//! `--format json`*. [`REGISTRY`] must **biject** that set, so a new verb reddens this
//! suite until someone classifies it.
//!
//! **The two tiers, split at the seam that makes parity decidable.**
//!
//!   * **Fenced** — the verb's agent/human text renders from a **structured Rust value**,
//!     so *field vs envelope key* is a mechanical question. Its check destructures the
//!     value **exhaustively — no `..`** — which makes the compiler the field enumerator:
//!     a field added to (or dropped from) the value fails this suite to *compile*, which
//!     is the strongest form of "the fence cannot fall behind the surface it fences".
//!     Each bound field then either names the envelope key that carries it, names the
//!     keys it is **decomposed** into, or is a **declared exclusion** carrying its
//!     recorded reason inline.
//!   * **Judgment** — the verb's text is composed **prose** (an orientation, a composed
//!     workflow, a validation report, a menu, a per-run-mode summary sentence). No
//!     field↔key correspondence exists to check mechanically, so these get the one-time
//!     enumerated census instead; each member below carries the reason it is there.
//!
//! **The judgment tier's census (M48 Inc 7 / T5).** The census is the judgment tier's own
//! deliverable, and a census that only *classifies* decides nothing — so every judgment
//! member additionally carries a stated [`Disposition`]: the gap this wave **closed** (with
//! the check that proves the key on the rendered bytes), or the reason it is **declared
//! out** of the window. Two closes, both under the enumeration rule: `describe`'s
//! router-hidden suppression (structured key, not a substring of `prose`) and `upgrade`'s
//! `checked` delta count (printed in the clean line, absent from the envelope). One
//! disposition is **derived rather than hand-listed** — `validate`'s, which is a claim about
//! *every* exit-flipping condition and so is built from `cli::render::STORE_EXIT_FLIPS`
//! itself (promoted `pub` for exactly this), with the claim re-proven per member against the
//! real renderer, so a fifth condition joins the entry the day it joins the table.
//!
//! **Declared bound, carried from the Settle:** this fence does **not** cover the prose
//! tier, and a key whose value nothing computes is **not** a gap ("re-derivable state,
//! counts, internal identifiers never rendered" — adding those is inventing contract
//! surface, not closing a gap). The judgment tier stays a census by that same Settle: its
//! members get *stated dispositions*, not mechanical per-field checks — the closes' arms and
//! the derivation are the only behaviour asserted here.
//!
//! **Proven non-vacuous by applied mutation** (M48 Inc 7 T4, recorded in `DECISIONS.md`):
//! deleting the shipped `"hook_file"` key from `render::setup_success`'s JSON arm reddens
//! [`every_fenced_renderer_carries_on_the_wire_what_its_text_prints`] with
//! *"`jigc setup`'s envelope withholds `hook_file`"*; restoring it greens. The census's
//! derivation is non-vacuous the same way (M48 Inc 7 T5): blanking one
//! `STORE_EXIT_FLIPS` member's `cause` reddens
//! [`the_validate_census_entry_is_derived_from_the_exit_flip_table`] on that member.

use clap::CommandFactory;
use cli::cli::{Cli, Format};
use cli::ingest::IngestReport;
use cli::relocate::RelocationReport;
use cli::rename::RenameReport;
use cli::render::{
    AckTarget, ConfigAck, DiscardState, DiscardedWork, DocAck, DroppedStaged, KnobReading,
    ManifestEntry, ManifestKind, MilestoneLanded, RejectedSet, StagedDoc, SubTaskContribution,
    TaskAck, TaskDiffView, config_ack, config_get, config_list, describe, doc_ack,
    freeze_exempt_relocation, ingest, milestone_finalized, milestone_join, rename, setup_success,
    task_ack, task_diff, task_list, uninstall_success, unmanage, validation_upgrade,
};
use cli::setup::{InstallCommit, RemovedArtifacts, SetupSummary, UninstallSummary};
use cli::task::TaskListRow;
use cli::unmanage::UnmanageReport;
use engine::compose::{CommandCatalog, Suppressed, WorkflowDef};
use engine::finding::Findings;
use engine::introspect::Description;
use engine::milestone::JoinOutcome;
use engine::state::BasePin;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

// ─────────────────────────────── the registry ───────────────────────────────

/// How a leaf verb's `--format json` envelope relates to what its text prints.
enum Tier {
    /// The text renders from a structured Rust value: the named renderer's check
    /// destructures it exhaustively and asserts each field reaches the wire.
    Fenced(&'static str),
    /// The text is composed prose: no mechanical field↔key correspondence. The reason is
    /// recorded here, together with the census's stated [`Disposition`] — the judgment
    /// tier's own deliverable (T5).
    Judgment(&'static str, Disposition),
}

/// **What the census decided about one judgment-tier member** (M48 Inc 7 / T5). A member
/// is either a gap this wave closed or a gap declared out with its reason: an
/// unclassified member would be the "census that decides nothing" the Settle's window is
/// meant to end.
enum Disposition {
    /// The census found a gap and **this wave closed it**: the envelope key(s) that joined,
    /// and the check that proves them on the rendered bytes.
    Closed {
        /// The keys that joined the envelope — a slice because one member's text can print
        /// more than one withheld fact (`upgrade`'s delta count and, since Increment 10,
        /// the adapter artifact its clean line names).
        keys: &'static [&'static str],
        /// The check that drives the real renderer in both surfaces.
        proof: fn(),
    },
    /// **Declared out** of the additive-key window, with the recorded reason.
    DeclaredOut(&'static str),
    /// Declared out with the reason **derived from a code-side registry** rather than
    /// hand-listed — for a member whose disposition is a claim about every member of some
    /// enumerable axis, so the entry cannot fall behind the axis it speaks for.
    Derived(fn() -> String),
}

/// **Every leaf verb, classified.** Bijected against the clap tree by
/// [`the_parity_registry_bijects_the_clap_leaf_verbs`], so a verb added anywhere reddens
/// this suite until it is classified — the property that makes this a standing fence
/// rather than a census with an expiry date.
const REGISTRY: &[(&[&str], Tier)] = &[
    // ── top-level ───────────────────────────────────────────────────────────────
    (
        &["start"],
        Tier::Judgment(
            "the orientation / composed-workflow surface — prose; composed output pinned as \
             {task, text}, orientation projected as OrientationView",
            Disposition::DeclaredOut(
                "TWO surfaces, declared out for two different reasons. (1) The COMPOSED arm's \
                 envelope is PINNED at exactly `{task, text}` (`command-output-contract.md` §1), \
                 which declares the agent surface's mint announcement, task-state affordances, \
                 create-gate list and (M50) already-open block to be presentation that adds no \
                 key — a prior contract decision \
                 the window does not reopen; the composed text itself rides `text` whole. (2) The \
                 bare-ORIENTATION arm renders from `engine::result::OrientationView`, whose every \
                 variant already carries on the wire each fact its text prints — the provenance \
                 header, the catalog, the off-catalog next steps, and (M50) the active set's id, \
                 workflow, intent, base pin, staged ids and findings-as-data. What the text adds \
                 and the envelope withholds is the `Run:` directives and the routing footer, which \
                 are routes rather than values and are presentation by the same declaration. There \
                 is no gap to close, which is why this stays DeclaredOut rather than becoming a \
                 Closed entry",
            ),
        ),
    ),
    (
        &["workflow"],
        Tier::Judgment(
            "the composed-workflow surface — prose, pinned as {task, text}",
            Disposition::DeclaredOut(
                "the same pinned `{task, text}` projection, byte-identical to `start`'s with \
                 `task: null` — the preview banner is presentation by the same declaration",
            ),
        ),
    ),
    (&["setup"], Tier::Fenced("setup_success")),
    (&["uninstall"], Tier::Fenced("uninstall_success")),
    (
        &["upgrade"],
        Tier::Judgment(
            "the upgrade validation report — prose findings + a checked count + the adapter \
             artifact the sweep read",
            // The census's second close. `checked` is read by the dispatch
            // (`upgrade::recorded_delta_count`) and printed by the clean line — "no findings —
            // N recorded config delta(s) re-apply clean" — while the envelope carried the
            // report alone. It is not re-derivable from `findings[]` (a clean sweep over zero
            // recorded deltas and a clean sweep over twelve serialize identically), so a driver
            // could not tell "nothing to check" from "everything checks out": the gap shape the
            // window exists to close. `guide` joins it at Increment 10, for the same reason and
            // in the same line: the sweep widened to read the adapter's owned artifact, so the
            // clean line names it — and a fact printed there is owed to the wire too.
            Disposition::Closed {
                keys: &["checked", "guide"],
                proof: upgrade_checked_close,
            },
        ),
    ),
    (&["ingest"], Tier::Fenced("ingest")),
    (
        &["migrate"],
        Tier::Judgment(
            "the migration review — the rewritten prose itself, plus its fidelity narration",
            Disposition::DeclaredOut(
                "the review's one computed-and-printed value is the fidelity delta \
                 (`render::dropped_release_versions`), which is DISPLAY-ONLY by decision \
                 (DECISIONS C4, Framing A: a labeled fuzzy scan that feeds no gate and is never \
                 a second structural authority) and is re-derivable besides — the envelope \
                 carries `source` and every `rewrites[].rendered` whole, which are the scan's \
                 only two inputs. Promoting a heuristic to a contract key is inventing surface, \
                 not closing a gap",
            ),
        ),
    ),
    (
        &["migrate-corpus"],
        Tier::Judgment(
            "the corpus-migration report — its run-mode sentences (dry-run / committed / \
             `--no-commit`) and its recovery clause (M48 F11 — the headline and the per-doc \
             line a run that LANDS an earlier run's unlanded migration prints) are prose, and \
             the report type carries crate-private commit-boundary fields, so no witness \
             exists outside the crate",
            Disposition::DeclaredOut(
                "the whole report serializes (`json(report)`), and the two text-only fields are \
                 `no_commit` and `unlanded` — each declared `#[serde(skip)]` at the field \
                 itself, with its recorded reason: the envelope already discriminates the run \
                 mode by `dry_run: false` + `commit: null`, and the recovery by `dry_run: \
                 false` + `commit: <sha>` + `migrated: []` — whose sha names the tree carrying \
                 exactly the recovered paths. Re-derivable, and recorded where they are skipped",
            ),
        ),
    ),
    (&["unmanage"], Tier::Fenced("unmanage")),
    (&["rename"], Tier::Fenced("rename")),
    (&["relocate"], Tier::Fenced("freeze_exempt_relocation")),
    (
        &["describe"],
        Tier::Judgment(
            "the introspection menu — prose one-liners over the projected catalog",
            // The census's headline close. The router-hidden state (and the declared reason for
            // it) has been narrated by the prose surface since M43's suppression fence, while a
            // driver could recover it only by substring-matching a sentence — on the one
            // surface whose prose is deliberately non-contractual, so the substring was never
            // a promise. The key joins the envelope; the prose tier is untouched.
            Disposition::Closed {
                keys: &["router_hidden"],
                proof: describe_router_hidden_close,
            },
        ),
    ),
    (
        &["validate"],
        Tier::Judgment(
            "the store validation report — prose finding lines + the store trailer",
            // Derived, never hand-listed: the disposition is a claim about EVERY exit-flipping
            // condition, so it is built from the code-side table and re-proven per member.
            Disposition::Derived(validate_census_entry),
        ),
    ),
    // ── doc ─────────────────────────────────────────────────────────────────────
    (&["doc", "create"], Tier::Fenced("doc_ack")),
    (&["doc", "add-item"], Tier::Fenced("doc_ack")),
    (&["doc", "remove-item"], Tier::Fenced("doc_ack")),
    (&["doc", "retitle-item"], Tier::Fenced("doc_ack")),
    (&["doc", "rename"], Tier::Fenced("doc_ack")),
    (&["doc", "set-field"], Tier::Fenced("doc_ack")),
    (&["doc", "set-slot"], Tier::Fenced("doc_ack")),
    (&["doc", "author"], Tier::Fenced("doc_ack")),
    (
        &["doc", "show"],
        Tier::Judgment(
            "a separately versioned read contract — the text arm renders the document, the \
             envelope its own `contract-version`ed projection",
            Disposition::DeclaredOut(
                "governed by its OWN pinned contract (`doc-read-surface.md`), whose additive-key \
                 window closes at the 1.0 pin — M49 spends it once more, on the top-level \
                 `schema-version` number — and the plain arm renders the document's own bytes, \
                 so there is no rendered-fact-vs-key axis for the parity rule to run over",
            ),
        ),
    ),
    (
        &["doc", "schema"],
        Tier::Judgment(
            "a separately versioned read contract — the text arm is the agent listing, the \
             envelope its own `contract-version`ed projection",
            Disposition::DeclaredOut(
                "governed by its own explicitly versioned contract, bumped 4→5 by this very \
                 increment (T1, the id-source `write-key`) — its evolution is a version bump, \
                 which is what the window's successor regime asks for, not a silent key",
            ),
        ),
    ),
    (
        &["doc", "list"],
        Tier::Judgment(
            "a separately versioned read contract — the text arm is the index listing, the \
             envelope its own `contract-version`ed projection",
            Disposition::DeclaredOut(
                "governed by its own pinned contract; the index row's every rendered fact \
                 (identity, registration state, item count) is already a key of that \
                 projection — the text arm is a rendering OF the envelope, not a sibling of it",
            ),
        ),
    ),
    // ── task ────────────────────────────────────────────────────────────────────
    (&["task", "list"], Tier::Fenced("task_list")),
    (&["task", "diff"], Tier::Fenced("task_diff")),
    (
        &["task", "validate"],
        Tier::Judgment(
            "the task-scoped validation report — prose finding lines + the verdict",
            Disposition::DeclaredOut(
                "the text renders the report and nothing else — one line per finding, each from \
                 a `Finding` the envelope carries whole — and the verdict it previews IS the \
                 exit code, which no key may restate without becoming a second authority over \
                 the same fact",
            ),
        ),
    ),
    (&["task", "discard"], Tier::Fenced("task_ack")),
    (
        &["task", "finalize"],
        Tier::Judgment(
            "the validation report plus the landed summary — prose; its landed facts ride the \
             whole-value `committed` object",
            Disposition::DeclaredOut(
                "the success section renders from `Landed` alone (hash, subject, manifest, file \
                 count, the left-out residue) and the envelope carries that value WHOLE under \
                 `committed` beside the report — a whole-value carry is the strongest form of \
                 the parity rule, not an exception to it",
            ),
        ),
    ),
    (&["task", "bind"], Tier::Fenced("task_ack")),
    // ── config ──────────────────────────────────────────────────────────────────
    (&["config", "get"], Tier::Fenced("config_get")),
    (&["config", "list"], Tier::Fenced("config_list")),
    (&["config", "set"], Tier::Fenced("config_ack")),
    (&["config", "insert-step"], Tier::Fenced("config_ack")),
    (&["config", "replace-step"], Tier::Fenced("config_ack")),
    (&["config", "remove-step"], Tier::Fenced("config_ack")),
    (&["config", "fill"], Tier::Fenced("config_ack")),
    (&["config", "fork"], Tier::Fenced("config_ack")),
    // ── milestone ───────────────────────────────────────────────────────────────
    // The seven verbs rendered by `render::milestone` hand it a **prose summary string**
    // (`milestone_created`'s multi-line ack among them); the envelope is pinned
    // `{text, hook_output}` and the growth rides inside `text` by declaration.
    (
        &["milestone", "create"],
        Tier::Judgment(
            "the mint ack — a prose summary carried whole inside `text`",
            MILESTONE_PROSE_SUMMARY,
        ),
    ),
    (
        &["milestone", "add-task"],
        Tier::Judgment(
            "a prose summary carried whole inside `text`",
            MILESTONE_PROSE_SUMMARY,
        ),
    ),
    (
        &["milestone", "add-from-spec"],
        Tier::Judgment(
            "a prose summary carried whole inside `text`",
            MILESTONE_PROSE_SUMMARY,
        ),
    ),
    (
        &["milestone", "list-tasks"],
        Tier::Judgment(
            "a prose summary carried whole inside `text`",
            MILESTONE_PROSE_SUMMARY,
        ),
    ),
    (
        &["milestone", "provision"],
        Tier::Judgment(
            "a prose summary carried whole inside `text`",
            MILESTONE_PROSE_SUMMARY,
        ),
    ),
    (
        &["milestone", "execute"],
        Tier::Judgment(
            "the composed milestone-execution walk — the pinned `{task, text}` projection",
            Disposition::DeclaredOut(
                "`execute` renders through `render::composed`, not \
                 `render::milestone`: stdout IS the pinned `{task, text}` contract, and \
                 the agent surface differs from it by the routing footer alone. Its one \
                 computed side channel — the partially-provisioned advisory (M49 \
                 Increment 10 / T5) — is not withheld from the driver but MOVED by \
                 stream discipline: under `--format json` it is emitted on stderr, \
                 exactly as the finalize `left-out` / carried-over advisories and the \
                 migrate byte-floor nudge are, so both surfaces state it and the pinned \
                 document stays one JSON value \
                 (`milestone_provision_handoff::the_partial_advisory_leaves_the_json_document_alone`)",
            ),
        ),
    ),
    (&["milestone", "join"], Tier::Fenced("milestone_join")),
    (
        &["milestone", "finalize"],
        Tier::Fenced("milestone_finalized"),
    ),
    (
        &["milestone", "discard"],
        Tier::Judgment(
            "a prose summary carried whole inside `text`",
            MILESTONE_PROSE_SUMMARY,
        ),
    ),
];

/// The six prose-summary `milestone` verbs' shared census disposition — one statement
/// because they share one renderer (`render::milestone`), and a per-verb copy would be
/// six places for the same fact to rot in. `milestone execute` left this set at M49
/// Increment 10 / T5: it never rendered through `render::milestone` at all, and it now has
/// a computed side channel of its own, so it carries its own disposition above.
const MILESTONE_PROSE_SUMMARY: Disposition = Disposition::DeclaredOut(
    "`render::milestone` puts the ENTIRE agent summary on the wire as `text` (beside \
     `hook_output`); the agent surface differs from it only by the routing footer, which is \
     framing every surface carries. Nothing is computed, printed, and withheld",
);

/// The per-renderer parity checks the fenced tier names. Bijected against the renderer
/// names [`REGISTRY`] uses, so a fenced row cannot name a check that does not exist and a
/// check cannot outlive the row that justified it.
const FENCES: &[(&str, fn())] = &[
    ("setup_success", setup_success_parity),
    ("uninstall_success", uninstall_success_parity),
    ("ingest", ingest_parity),
    ("unmanage", unmanage_parity),
    ("rename", rename_parity),
    ("freeze_exempt_relocation", relocation_parity),
    ("doc_ack", doc_ack_parity),
    ("task_list", task_list_parity),
    ("task_diff", task_diff_parity),
    ("task_ack", task_ack_parity),
    ("config_get", config_get_parity),
    ("config_list", config_list_parity),
    ("config_ack", config_ack_parity),
    ("milestone_join", milestone_join_parity),
    ("milestone_finalized", milestone_finalized_parity),
];

// ────────────────────────────── shared machinery ──────────────────────────────

/// Every **leaf** verb's argv path, walked from the clap `Command` tree — the shared
/// enumeration seam (`format_json_success_axis.rs` / `machine_output.rs` idiom), so this
/// fence and the success-path sweep derive their axis from the same tree.
fn leaf_verb_paths() -> Vec<Vec<String>> {
    fn walk(cmd: &clap::Command, prefix: Vec<String>, out: &mut Vec<Vec<String>>) {
        let mut had_child = false;
        for sub in cmd.get_subcommands() {
            if sub.get_name() == "help" {
                continue;
            }
            had_child = true;
            let mut child = prefix.clone();
            child.push(sub.get_name().to_string());
            walk(sub, child, out);
        }
        if !had_child && !prefix.is_empty() {
            out.push(prefix);
        }
    }
    let mut out = Vec::new();
    walk(&Cli::command(), Vec::new(), &mut out);
    out
}

/// The rendered JSON arm, parsed — the envelope a driver reads.
fn envelope(rendered: &str, label: &str) -> Value {
    serde_json::from_str(rendered).unwrap_or_else(|err| {
        panic!("`{label}`'s JSON arm is one document ({err}); got:\n{rendered}")
    })
}

/// **The parity assertion.** The envelope carries `key`, and its value is exactly the
/// field's own serialization — so the fact the text printed is on the wire, not merely a
/// key of the same name holding something else.
fn carries<T: serde::Serialize>(doc: &Value, key: &str, field: &T, label: &str, field_name: &str) {
    let expected = serde_json::to_value(field).expect("the rendered field serializes");
    let got = doc.get(key).unwrap_or_else(|| {
        panic!(
            "`{label}`'s envelope withholds `{key}` — its text renders `{field_name}`, and a value \
             the text prints but the envelope withholds is a gap (M48, the pre-1.0 additive-key \
             window). Envelope:\n{doc:#}",
        )
    });
    assert_eq!(
        got, &expected,
        "`{label}`'s `{key}` must carry `{field_name}`'s own value, not a same-named different \
         fact. Envelope:\n{doc:#}",
    );
}

/// The same, for a field the envelope carries **decomposed** into several keys (the
/// write address decomposed into `target`, say): every named key is present, and the
/// declared decomposition is recorded at the call site.
fn carries_decomposed(doc: &Value, keys: &[&str], label: &str, field_name: &str) {
    for key in keys {
        assert!(
            doc.get(*key).is_some(),
            "`{label}`'s envelope withholds `{key}`, one of the keys `{field_name}` is decomposed \
             into. Envelope:\n{doc:#}",
        );
    }
}

/// The agent text really does print `value` — the other half of the rule (a fact the
/// envelope carries that the text never prints is not this fence's business, but a
/// declared *gap* must be a gap: the text has to print it).
fn text_prints(text: &str, value: &str, label: &str, field_name: &str) {
    assert!(
        text.contains(value),
        "`{label}`'s agent text must print `{field_name}` — the fence asserts parity with what \
         the text says, so a field it never says has no gap to close. Text:\n{text}",
    );
}

// ─────────────────────────────── the fenced checks ───────────────────────────────

/// `jigc setup` — the install summary. Seven fields, seven keys (`hook_file` is M48's own
/// close, T2 of Increment 7; `hook_committed` its completion over the hooks-dir shape the
/// first sweep left silent; `guide_file` + `findings` the adapter-owned guide artifact of
/// Increment 10), plus the constant `installed` discriminator.
///
/// **Two witnesses, because the summary has two shapes and each withholds what the other
/// prints.** The ordinary install writes the artifact and reports nothing; the refused
/// install leaves a **user-modified** copy alone, so it prints an advisory and prints *no*
/// installed-guide line (`guide_file: None` — the line it would print says "stamped with
/// this build", which of a user's copy would be a lie). A single-witness fence would leave
/// whichever half it omitted unchecked.
fn setup_success_parity() {
    let guide = ".claude/skills/jigc/SKILL.md";
    let installed = SetupSummary {
        line_file: "CLAUDE.md".to_owned(),
        allowlist_file: ".claude/settings.json".to_owned(),
        hook_file: ".git/hooks/pre-commit".to_owned(),
        // The default hooks dir: git cannot track it, so the hook is in no commit — the
        // shape the summary must SAY something about, and the one the wire must carry.
        hook_committed: false,
        guide_file: Some(guide.to_owned()),
        findings: Vec::new().into(),
        install_commit: InstallCommit::Committed("a1b2c3d".to_owned()),
    };
    let refused = SetupSummary {
        line_file: "CLAUDE.md".to_owned(),
        allowlist_file: ".claude/settings.json".to_owned(),
        hook_file: ".git/hooks/pre-commit".to_owned(),
        hook_committed: false,
        guide_file: None,
        findings: vec![cli::setup::guide_modified_finding(guide)].into(),
        install_commit: InstallCommit::Committed("a1b2c3d".to_owned()),
    };

    for summary in [&installed, &refused] {
        // Exhaustive — no `..`: an eighth field fails to compile here.
        let SetupSummary {
            line_file,
            allowlist_file,
            hook_file,
            hook_committed,
            guide_file,
            findings,
            install_commit,
        } = summary;

        let text = setup_success(Format::Agent, summary);
        let doc = envelope(&setup_success(Format::Json, summary), "jigc setup");

        for (field_name, value) in [
            ("line_file", line_file),
            ("allowlist_file", allowlist_file),
            ("hook_file", hook_file),
        ] {
            text_prints(&text, value, "jigc setup", field_name);
            carries(&doc, field_name, value, "jigc setup", field_name);
        }
        // A boolean the text renders as a whole clause rather than as a printed value, so
        // the two halves are asserted apart: the wire carries the fact, and the text says
        // it in words. Both witnesses are the uncommittable shape, so the clause is owed.
        carries(
            &doc,
            "hook_committed",
            hook_committed,
            "jigc setup",
            "hook_committed",
        );
        assert!(
            !*hook_committed,
            "both witnesses are built on the default hooks dir — the assertion below reads \
             the `false` half",
        );
        text_prints(
            &text,
            "not in the install commit",
            "jigc setup",
            "hook_committed",
        );
        // Carried either way — a `null` is the honest answer when this run installed none.
        if let Some(guide_file) = guide_file {
            text_prints(&text, guide_file, "jigc setup", "guide_file");
        }
        carries(&doc, "guide_file", guide_file, "jigc setup", "guide_file");
        // Findings-as-data: every advisory the summary prints rides the wire whole.
        for finding in findings {
            text_prints(&text, &finding.message, "jigc setup", "findings[].message");
        }
        carries(&doc, "findings", findings, "jigc setup", "findings");

        let InstallCommit::Committed(sha) = install_commit else {
            unreachable!("both witnesses commit")
        };
        text_prints(&text, sha, "jigc setup", "install_commit");
        carries(&doc, "install_commit", sha, "jigc setup", "install_commit");
    }
}

/// `jigc uninstall` — the teardown summary. The **seven**-flag removal ledger rides the wire
/// whole under `removed` (the seventh is the adapter's owned guide artifact, M48 Increment
/// 10), so every flag the text turns into a bullet reaches a driver.
///
/// **Two witnesses, because the teardown has two shapes over that seventh artifact** — the
/// same split `setup` carries. The full teardown removes it and reports nothing; a teardown
/// that met a **user-modified** copy leaves it in place, so it removes nothing there and
/// prints an advisory instead. A single-witness fence would leave whichever half it omitted
/// unchecked — and the `findings` half is exactly where a refusal becomes visible.
fn uninstall_success_parity() {
    let guide = ".claude/skills/jigc/SKILL.md";
    let torn_down = UninstallSummary {
        line_file: "CLAUDE.md".to_owned(),
        allowlist_file: ".claude/settings.json".to_owned(),
        removed: RemovedArtifacts {
            jigc_dir: true,
            reference: true,
            allowlist: true,
            hook: true,
            deny: true,
            precommit: true,
            guide: true,
        },
        findings: Vec::new().into(),
    };
    let kept = UninstallSummary {
        line_file: "CLAUDE.md".to_owned(),
        allowlist_file: ".claude/settings.json".to_owned(),
        removed: RemovedArtifacts {
            guide: false,
            ..torn_down.removed
        },
        findings: vec![cli::setup::guide_kept_finding(guide)].into(),
    };

    for summary in [&torn_down, &kept] {
        // Exhaustive — no `..`: a fourth field fails to compile here.
        let UninstallSummary {
            line_file,
            allowlist_file,
            removed,
            findings,
        } = summary;
        let RemovedArtifacts {
            jigc_dir,
            reference,
            allowlist,
            hook,
            deny,
            precommit,
            guide,
        } = removed;

        let text = uninstall_success(Format::Agent, summary);
        let doc = envelope(&uninstall_success(Format::Json, summary), "jigc uninstall");

        text_prints(&text, line_file, "jigc uninstall", "line_file");
        carries(&doc, "line_file", line_file, "jigc uninstall", "line_file");
        text_prints(&text, allowlist_file, "jigc uninstall", "allowlist_file");
        carries(
            &doc,
            "allowlist_file",
            allowlist_file,
            "jigc uninstall",
            "allowlist_file",
        );

        let removed_doc = doc
            .get("removed")
            .unwrap_or_else(|| panic!("`jigc uninstall`'s envelope withholds `removed`:\n{doc:#}"));
        for (field_name, flag) in [
            ("jigc_dir", jigc_dir),
            ("reference", reference),
            ("allowlist", allowlist),
            ("hook", hook),
            ("deny", deny),
            ("precommit", precommit),
            ("guide", guide),
        ] {
            carries(
                removed_doc,
                field_name,
                flag,
                "jigc uninstall",
                &format!("removed.{field_name}"),
            );
        }

        // Findings-as-data: an artifact the teardown declined to take is *said* on both
        // surfaces — the text prints it, and a driver reads it rather than grepping prose.
        for finding in findings {
            text_prints(
                &text,
                &finding.message,
                "jigc uninstall",
                "findings[].message",
            );
        }
        carries(&doc, "findings", findings, "jigc uninstall", "findings");
    }
}

/// `jigc ingest` — the triage report. Its one field rides the wire whole (the envelope
/// adds the derived `summary` rollup beside it, which is projection, not parity).
fn ingest_parity() {
    let report = IngestReport { rows: Vec::new() };
    let IngestReport { rows } = &report;

    let doc = envelope(&ingest(Format::Json, &report), "jigc ingest");
    carries(&doc, "rows", rows, "jigc ingest", "rows");
}

/// `jigc unmanage` — the drop report, carried whole.
fn unmanage_parity() {
    let report = UnmanageReport {
        path: "docs/decisions/use-sqlite.md".to_owned(),
        identity: Some("adr:use-sqlite".to_owned()),
        dropped: true,
    };
    let UnmanageReport {
        path,
        identity,
        dropped,
    } = &report;

    let text = unmanage(Format::Agent, &report);
    let doc = envelope(&unmanage(Format::Json, &report), "jigc unmanage");

    text_prints(&text, path, "jigc unmanage", "path");
    carries(&doc, "path", path, "jigc unmanage", "path");
    carries(&doc, "identity", identity, "jigc unmanage", "identity");
    carries(&doc, "dropped", dropped, "jigc unmanage", "dropped");
}

/// `jigc rename` — the atomic-identity-move report, carried whole.
fn rename_parity() {
    let report = RenameReport {
        from: "adr:use-sqlite".to_owned(),
        to: "adr:use-postgres".to_owned(),
        old_path: "docs/decisions/use-sqlite.md".to_owned(),
        new_path: "docs/decisions/use-postgres.md".to_owned(),
        title: "Use Postgres".to_owned(),
        referrers: vec!["adr:cache-strategy#supersedes".to_owned()],
        prose_mentions: vec!["README.md:12".to_owned()],
        commit: Some("a1b2c3d".to_owned()),
        hook_output: String::new(),
    };
    let RenameReport {
        from,
        to,
        old_path,
        new_path,
        title,
        referrers,
        prose_mentions,
        commit,
        hook_output,
    } = &report;

    let doc = envelope(&rename(Format::Json, &report), "jigc rename");
    for (field_name, value) in [
        ("from", from),
        ("to", to),
        ("old_path", old_path),
        ("new_path", new_path),
        ("title", title),
        ("hook_output", hook_output),
    ] {
        carries(&doc, field_name, value, "jigc rename", field_name);
    }
    // The no-op discriminator (M48 Increment 8): the landed commit's sha, `null` when nothing
    // was committed — the fact the **text** states as the no-op ack, so the envelope owes it.
    carries(&doc, "commit", commit, "jigc rename", "commit");
    carries(&doc, "referrers", referrers, "jigc rename", "referrers");
    carries(
        &doc,
        "prose_mentions",
        prose_mentions,
        "jigc rename",
        "prose_mentions",
    );
}

/// `jigc relocate` — the freeze-exempt relocation report, carried whole.
fn relocation_parity() {
    let report = RelocationReport {
        moved: vec![("docs/legacy/n.md".to_owned(), "notes/n.md".to_owned())],
        blocked: vec![("docs/legacy/b.md".to_owned(), "unreadable".to_owned())],
        displaced: vec![("notes/n.md".to_owned(), ".jigc/displaced/n.md".to_owned())],
    };
    let RelocationReport {
        moved,
        blocked,
        displaced,
    } = &report;

    let doc = envelope(
        &freeze_exempt_relocation(Format::Json, &report),
        "jigc relocate",
    );
    carries(&doc, "moved", moved, "jigc relocate", "moved");
    carries(&doc, "blocked", blocked, "jigc relocate", "blocked");
    carries(&doc, "displaced", displaced, "jigc relocate", "displaced");
}

/// `jigc task list` — the active-task roster. The rows are the envelope.
fn task_list_parity() {
    let rows = vec![TaskListRow {
        id: "harden-the-cache".to_owned(),
        workflow: Some("single-task".to_owned()),
        intent: "harden the cache".to_owned(),
    }];
    let TaskListRow {
        id,
        workflow,
        intent,
    } = &rows[0];

    let text = task_list(Format::Agent, &rows);
    let doc = envelope(&task_list(Format::Json, &rows), "jigc task list");
    let row = doc
        .get(0)
        .unwrap_or_else(|| panic!("`jigc task list`'s envelope is the row array:\n{doc:#}"));

    text_prints(&text, id, "jigc task list", "id");
    carries(row, "id", id, "jigc task list", "id");
    carries(row, "workflow", workflow, "jigc task list", "workflow");
    text_prints(&text, intent, "jigc task list", "intent");
    carries(row, "intent", intent, "jigc task list", "intent");
}

/// `jigc task diff` — the work-unit reader.
///
/// **Declared exclusion: `StagedDoc.body`.** The text prints the staged bytes verbatim;
/// the envelope carries the identity only, because echoing bodies would mint a second,
/// unversioned managed-doc content-read path beside the separately-versioned `doc show`
/// (`design/doc-read-surface.md` → The version/posture map). Not a gap — a *routed*
/// read: the identity **is** the address `jigc doc show <id> --task <id>` takes.
fn task_diff_parity() {
    let base = BasePin::new("0123456789abcdef", "0123456");
    let staged = vec![StagedDoc {
        id: "adr:cache-strategy".to_owned(),
        body: "# Cache strategy\n".to_owned(),
    }];
    let view = TaskDiffView {
        task: "harden-the-cache",
        base: &base,
        code_diff: "diff --git a/src/lib.rs b/src/lib.rs\n",
        staged: &staged,
    };
    let TaskDiffView {
        task,
        base,
        code_diff,
        staged,
    } = &view;
    let StagedDoc {
        id,
        // DECLARED EXCLUSION — see this function's doc comment.
        body: _body,
    } = &staged[0];

    let doc = envelope(&task_diff(Format::Json, &view), "jigc task diff");
    carries(&doc, "task", task, "jigc task diff", "task");
    carries(&doc, "base", base, "jigc task diff", "base");
    carries(&doc, "code_diff", code_diff, "jigc task diff", "code_diff");
    let staged_doc = doc
        .get("staged_docs")
        .and_then(|docs| docs.get(0))
        .unwrap_or_else(|| panic!("`jigc task diff`'s envelope carries `staged_docs`:\n{doc:#}"));
    carries(staged_doc, "id", id, "jigc task diff", "staged.id");
}

/// The nine `doc` write acks — every variant, exhaustively destructured.
///
/// **Declared decomposition: `address`.** The text leads with the write address; the
/// envelope carries it decomposed into `target` (`doctype`/`slug`/`section`/`item`/`leaf`
/// — the same depth ladder `doc show`'s `#fragment` projects), so the value is on the
/// wire in the contract's own normal form, never withheld.
fn doc_ack_parity() {
    let target = || AckTarget {
        doctype: "adr".to_owned(),
        slug: "cache-strategy".to_owned(),
        section: Some("decision".to_owned()),
        item: None,
        leaf: None,
    };
    let head = || AckTarget {
        doctype: "adr".to_owned(),
        slug: "cache-strategy".to_owned(),
        section: None,
        item: None,
        leaf: None,
    };
    let address = "adr:cache-strategy#decision".to_owned();

    let acks = vec![
        DocAck::Field {
            address: address.clone(),
            target: target(),
            value: Value::String("accepted".to_owned()),
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::UnsetField {
            address: address.clone(),
            target: target(),
            already_absent: true,
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::Slot {
            address: address.clone(),
            target: target(),
            chars: 42,
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::RemovedItem {
            address: address.clone(),
            target: target(),
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::RetitledItem {
            address: address.clone(),
            target: target(),
            title: "Limits by client".to_owned(),
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::Renamed {
            address: "adr:cache-policy".to_owned(),
            target: head(),
            title: "Cache policy".to_owned(),
            from: "adr:cache-strategy".to_owned(),
            reslugged: true,
            committed_identity: false,
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::Renamed {
            address: "adr:cache-strategy".to_owned(),
            target: head(),
            title: "Cache policy".to_owned(),
            from: "adr:cache-strategy".to_owned(),
            reslugged: false,
            committed_identity: true,
            findings: Findings::from(Vec::new()),
            copied_in: false,
        },
        DocAck::Created {
            address: "adr:cache-strategy".to_owned(),
            target: head(),
            existed: true,
            findings: Findings::from(Vec::new()),
        },
        DocAck::AddedItem {
            address: "spec:rate-limiting#criteria/limits-per-ip".to_owned(),
            target: target(),
            findings: Findings::from(Vec::new()),
            copied_in: true,
        },
        DocAck::Authored {
            address: "adr:cache-strategy".to_owned(),
            target: head(),
            findings: Findings::from(Vec::new()),
        },
    ];

    for ack in &acks {
        let doc = envelope(&doc_ack(Format::Json, ack), "jigc doc <write>");
        let text = doc_ack(Format::Agent, ack);
        let label = doc["op"].as_str().unwrap_or("<no op>").to_owned();
        let label = format!("jigc doc {label}");
        let decomposed = &["doctype", "slug"];

        // Exhaustive per variant — no `..`: a field added to any arm fails to compile.
        match ack {
            DocAck::Field {
                address,
                target,
                value,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "value", value, &label, "value");
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::UnsetField {
                address,
                target,
                already_absent,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(
                    &doc,
                    "already_absent",
                    already_absent,
                    &label,
                    "already_absent",
                );
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::Slot {
                address,
                target,
                chars,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "chars", chars, &label, "chars");
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::RemovedItem {
                address,
                target,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::RetitledItem {
                address,
                target,
                title,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                text_prints(&text, title, &label, "title");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "title", title, &label, "title");
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::Renamed {
                address,
                target,
                title,
                from,
                reslugged,
                committed_identity,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                text_prints(&text, title, &label, "title");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "title", title, &label, "title");
                carries(&doc, "from", from, &label, "from");
                carries(&doc, "reslugged", reslugged, &label, "reslugged");
                carries(
                    &doc,
                    "committed_identity",
                    committed_identity,
                    &label,
                    "committed_identity",
                );
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::Created {
                address,
                target,
                existed,
                findings,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "existed", existed, &label, "existed");
                carries(&doc, "findings", findings, &label, "findings");
            }
            DocAck::AddedItem {
                address,
                target,
                findings,
                copied_in,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "findings", findings, &label, "findings");
                carries(&doc, "copied_in", copied_in, &label, "copied_in");
            }
            DocAck::Authored {
                address,
                target,
                findings,
            } => {
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], decomposed, &label, "address");
                carries(&doc, "target", target, &label, "target");
                carries(&doc, "findings", findings, &label, "findings");
            }
        }
    }
}

/// The two task-state acks.
///
/// **Declared decomposition: `Bound.address`** — carried as `target`, the same normal
/// form the doc acks use. **Declared exclusion: `DroppedStaged.transient`** — the text
/// marks a transient instance `(transient)`; the envelope carries the bare identities
/// because transience is a **schema** fact a driver reads from `jigc doc schema`
/// (`design/command-output-contract.md` §2, the recorded posture), i.e. re-derivable
/// state, which the Settle's exclusion covers.
fn task_ack_parity() {
    let acks = vec![
        TaskAck::Bound {
            task: "enforce-the-rate-limit".to_owned(),
            role: "spec".to_owned(),
            address: "spec:rate-limiting".to_owned(),
            target: AckTarget {
                doctype: "spec".to_owned(),
                slug: "rate-limiting".to_owned(),
                section: None,
                item: None,
                leaf: None,
            },
        },
        TaskAck::Discarded {
            task: "harden-the-cache".to_owned(),
            dropped: vec![DroppedStaged {
                doc: "adr:cache-strategy".to_owned(),
                transient: false,
            }],
        },
    ];

    for ack in &acks {
        let doc = envelope(&task_ack(Format::Json, ack), "jigc task <state verb>");
        let text = task_ack(Format::Agent, ack);
        let label = format!("jigc {}", doc["op"].as_str().unwrap_or("<no op>"));

        match ack {
            TaskAck::Bound {
                task,
                role,
                address,
                target,
            } => {
                text_prints(&text, task, &label, "task");
                carries(&doc, "task", task, &label, "task");
                text_prints(&text, role, &label, "role");
                carries(&doc, "role", role, &label, "role");
                text_prints(&text, address, &label, "address");
                carries_decomposed(&doc["target"], &["doctype", "slug"], &label, "address");
                carries(&doc, "target", target, &label, "target");
            }
            TaskAck::Discarded { task, dropped } => {
                text_prints(&text, task, &label, "task");
                carries(&doc, "task", task, &label, "task");
                let DroppedStaged {
                    doc: staged_doc,
                    // DECLARED EXCLUSION — see this function's doc comment.
                    transient: _transient,
                } = &dropped[0];
                text_prints(&text, staged_doc, &label, "dropped.doc");
                carries(&doc, "dropped", &vec![staged_doc], &label, "dropped[].doc");
            }
        }
    }
}

/// One `KnobReading` witness — the row shape both cascade read verbs render.
fn knob_reading() -> KnobReading {
    KnobReading {
        key: "docs-root".to_owned(),
        value: "docs".to_owned(),
        layer: "project",
        rejected: Some(RejectedSet {
            attempted: "documentation".to_owned(),
            floor: "docs".to_owned(),
            layer: "team",
        }),
    }
}

/// Assert one knob-reading row object carries every field the reading line prints.
fn assert_knob_row(row: &Value, reading: &KnobReading, text: &str, label: &str) {
    let KnobReading {
        key,
        value,
        layer,
        rejected,
    } = reading;
    text_prints(text, key, label, "key");
    carries(row, "key", key, label, "key");
    text_prints(text, value, label, "value");
    carries(row, "value", value, label, "value");
    text_prints(text, layer, label, "layer");
    carries(row, "layer", layer, label, "layer");

    let RejectedSet {
        attempted,
        floor,
        layer: rejected_layer,
    } = rejected.as_ref().expect("the witness carries a rejection");
    let rejected_row = row
        .get("rejected")
        .unwrap_or_else(|| panic!("`{label}`'s envelope withholds `rejected`:\n{row:#}"));
    text_prints(text, attempted, label, "rejected.attempted");
    carries(
        rejected_row,
        "attempted",
        attempted,
        label,
        "rejected.attempted",
    );
    text_prints(text, floor, label, "rejected.floor");
    carries(rejected_row, "floor", floor, label, "rejected.floor");
    text_prints(text, rejected_layer, label, "rejected.layer");
    carries(
        rejected_row,
        "layer",
        rejected_layer,
        label,
        "rejected.layer",
    );
}

/// `jigc config get` — one knob's resolved reading.
fn config_get_parity() {
    let reading = knob_reading();
    let text = config_get(Format::Agent, &reading);
    let doc = envelope(&config_get(Format::Json, &reading), "jigc config get");
    assert_knob_row(&doc, &reading, &text, "jigc config get");
}

/// `jigc config list` — the whole declared surface, one row per knob.
fn config_list_parity() {
    let readings = vec![knob_reading()];
    let text = config_list(Format::Agent, &readings);
    let doc = envelope(&config_list(Format::Json, &readings), "jigc config list");
    let row = doc
        .get("knobs")
        .and_then(|knobs| knobs.get(0))
        .unwrap_or_else(|| panic!("`jigc config list`'s envelope carries `knobs`:\n{doc:#}"));
    assert_knob_row(row, &readings[0], &text, "jigc config list");
}

/// The six cascade-authoring acks — iterated from [`ConfigAck::ALL`], the code-side axis
/// T3 minted, so a seventh authoring verb joins this fence with its arm.
fn config_ack_parity() {
    for arm in ConfigAck::ALL {
        let ack = (arm.witness)();
        let label = format!("jigc config {}", arm.verb);
        let text = config_ack(Format::Agent, &ack);
        let doc = envelope(&config_ack(Format::Json, &ack), &label);

        match &ack {
            ConfigAck::Set { key, value } => {
                text_prints(&text, key, &label, "key");
                carries(&doc, "key", key, &label, "key");
                text_prints(&text, value, &label, "value");
                carries(&doc, "value", value, &label, "value");
            }
            ConfigAck::InsertStep {
                workflow,
                step,
                side,
                anchor,
            } => {
                text_prints(&text, workflow, &label, "workflow");
                carries(&doc, "workflow", workflow, &label, "workflow");
                text_prints(&text, step, &label, "step");
                carries(&doc, "step", step, &label, "step");
                text_prints(&text, side, &label, "side");
                carries(&doc, "side", side, &label, "side");
                text_prints(&text, anchor, &label, "anchor");
                carries(&doc, "anchor", anchor, &label, "anchor");
            }
            ConfigAck::ReplaceStep { target, step } => {
                text_prints(&text, target, &label, "target");
                carries(&doc, "target", target, &label, "target");
                text_prints(&text, step, &label, "step");
                carries(&doc, "step", step, &label, "step");
            }
            ConfigAck::RemoveStep { target } => {
                text_prints(&text, target, &label, "target");
                carries(&doc, "target", target, &label, "target");
            }
            ConfigAck::Fill { target } => {
                text_prints(&text, target, &label, "target");
                carries(&doc, "target", target, &label, "target");
            }
            ConfigAck::Fork { target, path, base } => {
                text_prints(&text, target, &label, "target");
                carries(&doc, "target", target, &label, "target");
                text_prints(&text, path, &label, "path");
                carries(&doc, "path", path, &label, "path");
                text_prints(&text, base, &label, "base");
                carries(&doc, "base", base, &label, "base");
            }
        }
    }
}

/// `jigc milestone join` — the merged-overlay ack. Its text renders from **three**
/// inputs, not one: the milestone id it echoes, the merge outcome, and the milestone's
/// full sub-task list, from which it derives the `no docs staged from:` line naming every
/// sub-task that contributed nothing.
fn milestone_join_parity() {
    let outcome = JoinOutcome::default();
    let milestone_id = "cache-rework";
    let sub_tasks = vec!["warm-the-read-cache".to_owned()];

    let text = milestone_join(Format::Agent, milestone_id, &outcome, &sub_tasks);
    let doc = envelope(
        &milestone_join(Format::Json, milestone_id, &outcome, &sub_tasks),
        "jigc milestone join",
    );
    let JoinOutcome { overlay, findings } = &outcome;

    text_prints(&text, milestone_id, "jigc milestone join", "milestone_id");
    carries(
        &doc,
        "milestone",
        &milestone_id,
        "jigc milestone join",
        "milestone_id",
    );
    carries(&doc, "overlay", overlay, "jigc milestone join", "overlay");
    carries(
        &doc,
        "findings",
        findings,
        "jigc milestone join",
        "findings",
    );
    // The doc-less sub-tasks the text names — derived from `sub_tasks` minus the overlay's
    // contributors, so the fact the text states is the one the envelope must carry.
    text_prints(
        &text,
        &sub_tasks[0],
        "jigc milestone join",
        "sub_tasks (the doc-less set)",
    );
    carries(
        &doc,
        "no_docs_from",
        &sub_tasks,
        "jigc milestone join",
        "sub_tasks (the doc-less set)",
    );
}

/// `jigc milestone finalize` — the landed boundary, carried whole under `committed`.
fn milestone_finalized_parity() {
    let landed = MilestoneLanded {
        hash: "a1b2c3d".to_owned(),
        subject: "Finalize milestone cache-rework (1 sub-task)".to_owned(),
        files: 1,
        manifest: vec![ManifestEntry {
            path: "docs/decisions/cache-strategy.md".to_owned(),
            kind: ManifestKind::Promoted,
        }],
        sub_tasks: vec![SubTaskContribution {
            id: "warm-the-read-cache".to_owned(),
            docs: 1,
            code_files: 0,
            provisioned: true,
            worktree_unreadable: false,
            discarded: vec![DiscardedWork {
                path: "src/scratch.rs".to_owned(),
                state: DiscardState::NeverStaged,
            }],
        }],
        hook_output: String::new(),
    };
    let MilestoneLanded {
        hash,
        subject,
        files,
        manifest,
        sub_tasks,
        hook_output,
    } = &landed;

    let text = milestone_finalized(Format::Agent, &landed);
    let doc = envelope(
        &milestone_finalized(Format::Json, &landed),
        "jigc milestone finalize",
    );
    let committed = doc.get("committed").unwrap_or_else(|| {
        panic!("`jigc milestone finalize`'s envelope carries `committed`:\n{doc:#}")
    });

    let label = "jigc milestone finalize";
    text_prints(&text, hash, label, "hash");
    carries(committed, "hash", hash, label, "hash");
    text_prints(&text, subject, label, "subject");
    carries(committed, "subject", subject, label, "subject");
    carries(committed, "files", files, label, "files");
    carries(committed, "manifest", manifest, label, "manifest");
    carries(committed, "sub_tasks", sub_tasks, label, "sub_tasks");
    carries(committed, "hook_output", hook_output, label, "hook_output");
}

// ────────────────────────── the judgment-tier census (T5) ──────────────────────────

/// A no-delta resolved cascade — the honest input for a report whose findings carry no
/// inventory row, so the engine's severity post-pass is a guaranteed no-op over them
/// (the `cli::cascade_util::no_delta_resolved` shape, which is crate-private).
fn no_delta_resolved() -> engine::cascade::Resolved {
    let pack = engine::cascade::PackDefaultLayer::new("", "", BTreeMap::new(), Vec::new());
    engine::cascade::resolve(&pack, None, None).expect("a no-delta cascade resolves")
}

/// **The `describe` close.** A hidden workflow's suppression reaches the envelope as the
/// structured `router_hidden` key, carrying the declared reason the prose sentence names —
/// asserted on the rendered bytes of the real renderer, over the engine's real assembler,
/// with a non-hidden sibling in the same projection so the key discriminates rather than
/// merely existing.
///
/// The end-to-end twin over the **shipped packs** lives in
/// `crates/cli/tests/describe.rs::describe_json_carries_the_router_hidden_suppression_as_a_key`.
fn describe_router_hidden_close() {
    let reason = "spawned by the fan-out, never picked from the catalog";
    let hidden = WorkflowDef {
        when: None,
        description: Some("one sub-task of a milestone fan-out".to_owned()),
        usage: None,
        creates_task: true,
        selectable: false,
        suppressed: Some(Suppressed {
            reason: reason.to_owned(),
            expires: "never".to_owned(),
        }),
        allows_create: Vec::new(),
        reads: Vec::new(),
        includes: Vec::new(),
    };
    let plain = WorkflowDef {
        description: Some("one well-scoped change, intent to commit".to_owned()),
        selectable: true,
        suppressed: None,
        ..hidden.clone()
    };
    let catalog = CommandCatalog {
        commands: BTreeMap::new(),
    };
    let description = Description::assemble(
        [("sub-task", &hidden, None), ("single-task", &plain, None)],
        std::iter::empty(),
        [("dev", &catalog)],
    );

    let text = describe(Format::Agent, &description);
    let doc = envelope(&describe(Format::Json, &description), "jigc describe");
    let definitions = doc["definitions"]
        .as_array()
        .expect("the projection carries `definitions`");

    text_prints(&text, reason, "jigc describe", "suppressed.reason");
    for definition in definitions {
        let id = definition["id"].as_str().expect("each entry carries an id");
        let key = definition.get("router_hidden").unwrap_or_else(|| {
            panic!(
                "`jigc describe`'s envelope withholds `router_hidden` for `{id}` — its prose \
                 states the router-hidden clause, and a value the text prints but the envelope \
                 withholds is a gap. Envelope:\n{doc:#}"
            )
        });
        match id {
            "sub-task" => assert_eq!(
                key.as_str(),
                Some(reason),
                "the hidden workflow's key carries its DECLARED REASON:\n{doc:#}",
            ),
            _ => assert!(
                key.is_null(),
                "`{id}` is not hidden, so its key must be null — a key that lies is worse than \
                 one that is absent:\n{doc:#}",
            ),
        }
    }
}

/// **The `upgrade` close.** The delta count the clean line names — *"N recorded config
/// delta(s) re-apply clean"* — reaches the envelope as `checked`, on a clean sweep (where
/// the text prints it) and with findings present alike (where a driver still needs to know
/// how wide the sweep was). Since Increment 10 the same clean line also names the adapter's
/// **owned guide artifact** the sweep read, and that path reaches the envelope as `guide`.
fn upgrade_checked_close() {
    let resolved = no_delta_resolved();
    let report = engine::result::ValidationReport::new(Vec::new(), &resolved);
    let checked = 3usize;
    let guide = ".claude/skills/jigc/SKILL.md";

    let text = validation_upgrade(Format::Agent, &report, checked, Some(guide));
    let doc = envelope(
        &validation_upgrade(Format::Json, &report, checked, Some(guide)),
        "jigc upgrade",
    );
    text_prints(&text, "3", "jigc upgrade", "checked");
    carries(&doc, "checked", &checked, "jigc upgrade", "checked");
    text_prints(&text, guide, "jigc upgrade", "guide");
    carries(&doc, "guide", &guide, "jigc upgrade", "guide");

    // The zero case is the one a driver most needs: "no recorded deltas to check" and
    // "twelve deltas re-apply clean" carry identical `findings: []`.
    let none = envelope(
        &validation_upgrade(Format::Json, &report, 0, None),
        "jigc upgrade",
    );
    carries(&none, "checked", &0usize, "jigc upgrade", "checked");
    // The omitting context: no artifact read, so the key is `null` and the clean line
    // claims nothing about one (the fence's own inert-on-omission arm).
    carries(
        &none,
        "guide",
        &Option::<&str>::None,
        "jigc upgrade",
        "guide",
    );
    let bare = validation_upgrade(Format::Agent, &report, 0, None);
    assert!(
        !bare.contains("guide artifact"),
        "with no artifact read the clean line must claim no guide check; got:\n{bare}",
    );
}

/// **The `validate` member's census entry, DERIVED from the code-side registry.** Its
/// disposition is a claim about *which condition fired*, and that is a claim about every
/// member of [`STORE_EXIT_FLIPS`] — so the entry is built from the table rather than
/// hand-listed, and a fifth condition joins it the day it joins the table (the `pub`
/// promotion is what lets a `tests/` suite read the table at all).
///
/// The disposition itself: **declared out under the recorded re-derivable exclusion.** The
/// trailer names which condition fired in prose; the envelope carries `report_only: false`
/// plus the very finding that *is* the condition inside `findings[]`, so the identity is
/// re-derivable from the wire rather than withheld — and
/// [`the_validate_census_entry_is_derived_from_the_exit_flip_table`] re-proves that per
/// member against the real renderer instead of taking this sentence's word for it.
fn validate_census_entry() -> String {
    let conditions: Vec<String> = cli::render::STORE_EXIT_FLIPS
        .iter()
        .map(|flip| format!("`{}` ({})", flip.id, flip.cause))
        .collect();
    format!(
        "declared out under the recorded re-derivable exclusion: the store trailer names WHICH \
         exit-flipping condition fired — {} — and each condition's identifying finding rides \
         `findings[]` on the same envelope beside `report_only: false`, so the identity is \
         re-derivable from the wire, not withheld. Derived from `cli::render::STORE_EXIT_FLIPS`, \
         so a fifth condition joins this entry with the table",
        conditions.join(", "),
    )
}

/// **The derivation, proven per member.** The entry above claims something of *every*
/// exit-flipping condition, so this drives the real store renderer with each member's own
/// witness and holds both halves of the claim: the agent trailer names the condition (the
/// text really does print the fact), and the envelope carries `report_only: false` plus a
/// finding whose code identifies *that* condition and no other (the fact is re-derivable,
/// which is what the exclusion rests on).
///
/// A fifth condition whose identity does **not** reach the wire reddens here rather than
/// silently widening a disposition written for four. Proven non-vacuous by applied mutation
/// (M48 Inc 7 T5): blanking a member's `cause` reddens its first assertion.
#[test]
fn the_validate_census_entry_is_derived_from_the_exit_flip_table() {
    let resolved = no_delta_resolved();
    let entry = validate_census_entry();

    // Every member of the table is named by the derived entry — the derivation, not a
    // hand-list that a fifth member could outlive.
    for flip in cli::render::STORE_EXIT_FLIPS {
        assert!(
            entry.contains(flip.id) && entry.contains(flip.cause),
            "the derived census entry must name `{}` and the words its trailer fires with \
             ({:?}); got:\n{entry}",
            flip.id,
            flip.cause,
        );
    }

    // The identifying codes discriminate: an envelope carrying one member's finding cannot
    // be read as another's.
    let codes: BTreeSet<String> = cli::render::STORE_EXIT_FLIPS
        .iter()
        .map(|flip| (flip.witness)().code.clone())
        .collect();
    assert_eq!(
        codes.len(),
        cli::render::STORE_EXIT_FLIPS.len(),
        "each exit-flipping condition must be identifiable by its own finding code, or the \
         identity is NOT re-derivable from `findings[]` and the exclusion does not hold",
    );

    for flip in cli::render::STORE_EXIT_FLIPS {
        let id = flip.id;
        let witness = (flip.witness)();
        let code = witness.code.clone();
        let report = engine::result::ValidationReport::new(vec![witness], &resolved);
        let unbaselined = BTreeSet::new();

        let text = cli::render::validation_store(Format::Agent, &report, &unbaselined);
        assert!(
            text.contains(flip.cause),
            "{id}: the trailer must name which condition fired ({:?}) — the half of the claim \
             that says the TEXT prints this fact; got:\n{text}",
            flip.cause,
        );

        let doc = envelope(
            &cli::render::validation_store(Format::Json, &report, &unbaselined),
            "jigc validate",
        );
        assert_eq!(
            doc.get("report_only"),
            Some(&Value::Bool(false)),
            "{id}: the envelope must state the flipped exit;\n{doc:#}",
        );
        let carried: Vec<&str> = doc["findings"]
            .as_array()
            .expect("the report carries `findings`")
            .iter()
            .filter_map(|finding| finding["code"].as_str())
            .collect();
        assert!(
            carried.contains(&code.as_str()),
            "{id}: the finding that IS this condition must ride `findings[]`, or the trailer's \
             identity is withheld rather than re-derivable and this member is a GAP, not a \
             declared exclusion. Envelope:\n{doc:#}",
        );
    }
}

/// **Every judgment member states a disposition, and every close proves its key.** The
/// census is the judgment tier's deliverable: a member with no disposition is a verb the
/// sweep looked at and decided nothing about.
#[test]
fn every_judgment_member_states_a_disposition_the_census_can_stand_on() {
    let mut closed = 0usize;
    for (path, tier) in REGISTRY {
        let Tier::Judgment(reason, disposition) = tier else {
            continue;
        };
        let verb = path.join(" ");
        assert!(
            reason.len() > 20,
            "`jigc {verb}` is in the judgment tier, so it must record WHY its text has no \
             field↔key correspondence; got {reason:?}",
        );
        match disposition {
            Disposition::Closed { keys, proof } => {
                assert!(
                    !keys.is_empty(),
                    "`jigc {verb}` claims a close, so it must name the key(s) that joined",
                );
                eprintln!("census close: jigc {verb} → {}", keys.join(", "));
                proof();
                closed += 1;
            }
            Disposition::DeclaredOut(why) => assert!(
                why.len() > 40,
                "`jigc {verb}` is declared out of the additive-key window, so it must carry the \
                 reason — an unreasoned exclusion is the census deciding nothing; got {why:?}",
            ),
            Disposition::Derived(derive) => {
                let derived = derive();
                assert!(
                    derived.len() > 40,
                    "`jigc {verb}`'s derived disposition must state something; got {derived:?}",
                );
            }
        }
    }
    assert!(
        closed >= 2,
        "the census closed two gaps (`describe`'s `router_hidden`, `upgrade`'s `checked`) — a \
         run where neither is claimed means the closes lost their rows; got {closed}",
    );
}

// ─────────────────────────────────── the fence ───────────────────────────────────

/// **The bijection.** [`REGISTRY`] and the clap tree's leaf verbs are the *same* set — no
/// verb unclassified, no stale row — with a floor beneath it so a simultaneous delete on
/// both sides stays visible rather than shrinking the axis in silence.
#[test]
fn the_parity_registry_bijects_the_clap_leaf_verbs() {
    let from_clap: BTreeSet<Vec<String>> = leaf_verb_paths().into_iter().collect();
    let from_registry: BTreeSet<Vec<String>> = REGISTRY
        .iter()
        .map(|(path, _)| path.iter().map(|s| (*s).to_string()).collect())
        .collect();

    assert!(
        from_clap.len() >= 47,
        "the clap tree must still enumerate the whole verb surface (>= 47 leaf verbs); got {}: \
         {from_clap:?}",
        from_clap.len(),
    );

    let unclassified: Vec<_> = from_clap.difference(&from_registry).collect();
    let stale: Vec<_> = from_registry.difference(&from_clap).collect();
    assert!(
        unclassified.is_empty() && stale.is_empty(),
        "the parity registry must be a BIJECTION with the clap leaf verbs — every verb that \
         speaks `--format json` is either FENCED (its text renders from a structured value) or \
         JUDGMENT (its text is prose).\n\
         verbs with no classification: {unclassified:?}\n\
         rows naming no verb (delete them): {stale:?}",
    );
    assert_eq!(
        REGISTRY.len(),
        from_clap.len(),
        "one row per leaf verb — a duplicated path would hide an unclassified verb",
    );
}

/// **Every fenced row names a check, and every check is claimed.** The renderer names are
/// the join between the verb axis and the per-renderer parity assertions, so neither side
/// can drift: a fenced verb whose renderer has no check reddens here, and a check no verb
/// claims reddens here too.
#[test]
fn every_fenced_member_names_a_renderer_the_fence_checks() {
    let named: BTreeSet<&str> = REGISTRY
        .iter()
        .filter_map(|(_, tier)| match tier {
            Tier::Fenced(renderer) => Some(*renderer),
            Tier::Judgment(..) => None,
        })
        .collect();
    let checked: BTreeSet<&str> = FENCES.iter().map(|(name, _)| *name).collect();

    assert_eq!(
        named, checked,
        "every FENCED verb's renderer owns a parity check, and every check is claimed by at \
         least one verb",
    );
    assert_eq!(
        FENCES.len(),
        checked.len(),
        "one check per renderer — a duplicated name would shadow a check",
    );
}

/// **The fence itself.** Every fenced renderer carries, on the wire, each field of the
/// value its agent/human text renders from — asserted against a witness the real renderer
/// formats in both surfaces, with the value destructured exhaustively so the compiler is
/// the field enumerator.
#[test]
fn every_fenced_renderer_carries_on_the_wire_what_its_text_prints() {
    for (name, check) in FENCES {
        eprintln!("parity fence: {name}");
        check();
    }
}
