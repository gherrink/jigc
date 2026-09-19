//! Introspection — the `describe` whole-menu projection assembler.
//!
//! Assembles a [`Description`]: a prose projection of the resolved definitions
//! (every workflow, every doctype, plus the command-ref `hint`s) for the
//! `jigc describe` self-description surface (`design/introspection.md`). The
//! engine **assembles, never generates** — it makes no LLM call; the prose is
//! the human-authored `description:` / `usage:` fields carried on the
//! definitions themselves, woven into a structured sentence here, and the
//! command-ref `hint`s carried verbatim — one entry per **declaring pack**, so a
//! composed pack-set's command surface is the union of its catalogs rather than the
//! precedence winner's alone ([`CommandHint`]). Every narrated definition carries the
//! origin its caller resolved, so the unioned menu says which pack provided each entry
//! ([`DefinitionProse::origin_pack`]). The result is presentation-free: it
//! carries the woven prose, not a rendered surface (the `cli::render` free-prose
//! renderer frames it — `module-layout.md` → CLI renders).
//!
//! The weave (`introspection.md` → The authored fields): a definition with both
//! fields becomes "*X* is `<description>`. Reach for it when `<usage>`."; with
//! only one of the two it narrates *that one*; with **both absent** it is
//! **skipped** (skip-on-absent — describe is a menu, not a gate). Enumeration is
//! over the **unfiltered** definition set (every workflow, not the
//! `creates-task && selectable` catalog), sorted by id so the projection is
//! deterministic regardless of input order. A **hidden** workflow's narration
//! additionally carries its suppression clause — it is hidden from the router
//! catalog, plus the declared `suppressed.reason` (M43 law 2; [`weave_workflow`]).

use serde::{Deserialize, Serialize};

use crate::compose::{CommandCatalog, WorkflowDef};
use crate::result::SCHEMA_VERSION;
use crate::schema::Schema;

/// Which kind of definition a [`DefinitionProse`] narrates — so the renderer can
/// group "the workflows you can compose" apart from "the doc-types you can
/// create" without parsing the prose back out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DefinitionKind {
    /// A workflow definition (the unfiltered set, not the selectable catalog).
    Workflow,
    /// A doc-type schema definition.
    Doctype,
}

/// One narrated definition: its stable `id`, its `kind`, the woven prose sentence
/// assembled from the authored `description:` / `usage:` fields, and — for a hidden
/// workflow — the router-suppression the prose narrates, carried structurally.
///
/// Only definitions that carry at least one of the two fields produce a
/// `DefinitionProse` (skip-on-absent); `prose` is therefore always a non-empty
/// narration, never a placeholder for "nothing authored".
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionProse {
    /// Which definition kind this narrates.
    pub kind: DefinitionKind,
    /// The definition's stable id (e.g. `single-task`, `adr`).
    pub id: String,
    /// The woven prose sentence (the `description:` / `usage:` weave).
    pub prose: String,
    /// `Some(reason)` when this definition is **hidden from the router catalog** —
    /// the declared `suppressed.reason` [`weave_workflow`] narrates in prose; `None`
    /// when it is not hidden (every doctype, and every selectable workflow).
    ///
    /// The **shape** half of a fact the prose has stated since M43's suppression
    /// fence (`surface-contract.md` → The suppression fence): the narration reads
    /// "… It is hidden from the router catalog: `<reason>`.", so a driver could
    /// recover the state only by substring-matching a sentence — on the one surface
    /// whose prose is deliberately non-contractual, which made the substring a
    /// promise nobody had made. Carried here so `describe --format json` states it
    /// structurally (M48 Inc 7 / T5, the judgment-tier census's close under the
    /// pre-1.0 additive-key window). The engine still **assembles, never generates**:
    /// the reason is the pack-authored string, carried verbatim exactly as
    /// `description:` / `usage:` are.
    #[serde(default)]
    pub router_hidden: Option<String>,
    /// The `pack-id` of the pack that **actually provided** this definition, or `None`
    /// when no pack did — the caller resolves both, and the assembler carries the answer
    /// verbatim (the engine holds no cascade and no pack-set).
    ///
    /// `None` is a real answer, not a missing one: the **project layer** outranks every
    /// pack and replaces a definition file whole (`overrides.md` → Authored metadata on a
    /// definition resolves by whole-file shadow), so a shadowed definition is provided by
    /// the project, and naming a pack there would point the reader at a file whose prose
    /// the projection no longer carries.
    ///
    /// The sibling of [`CommandHint::pack`], one definition kind over, and for the same
    /// reason: a composed pack-set unions the definitions of every constituent
    /// (`multi-pack.md` → Composition), so an unattributed entry cannot say which pack
    /// ships it — which is the pack a reader has to edit, vendor or drop (M50 Increment 5
    /// / T1; `introspection.md` → The authored fields).
    #[serde(default)]
    pub origin_pack: Option<String>,
}

/// One command-ref's projected `hint` — the command's `{{cli.<id>}}` id, the pack
/// that declares it, and its authored one-line `hint`, carried verbatim as prose
/// (`hint`'s first projection consumer — `introspection.md` → Command surface).
///
/// **The id alone does not identify a command-ref under a composed pack-set.** A
/// workflow resolves `{{cli.<id>}}` against *its own* origin pack's catalog, so two
/// packs may each declare the same id with a different argv and a different `hint`,
/// and both are genuinely reachable. The projection therefore carries one entry per
/// **declaring pack**, keyed `(id, pack)` — the union, attributed — rather than the
/// precedence winner alone (M49 Increment 11 / T6; `introspection.md` → Command
/// surface).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandHint {
    /// The command-ref's `{{cli.<id>}}` id.
    pub id: String,
    /// The `pack-id` of the pack whose catalog declares this entry — the attribution
    /// that makes a repeated id readable. Empty only for a pack that declares no
    /// `pack-id` (provenance is a display surface, never a hard-fail path).
    pub pack: String,
    /// The authored one-line `hint`, carried verbatim.
    pub hint: String,
}

/// The whole-menu projection — the versioned result substrate `jigc describe`
/// renders into discursive prose.
///
/// Presentation-free: it carries the **woven prose** (not a rendered surface),
/// so the `cli::render` free-prose renderer owns all formatting. The lists are
/// **sorted by id** (workflows then doctypes within `definitions`; command-refs
/// within `commands`), so the projection is deterministic across a re-run or a
/// scrambled input set.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Description {
    /// The result-contract schema version (see [`SCHEMA_VERSION`]).
    pub schema_version: u32,
    /// The narrated workflow + doctype definitions, in (kind, id) order —
    /// workflows first, doctypes second, each id-sorted. Both-absent
    /// definitions are omitted (skip-on-absent).
    pub definitions: Vec<DefinitionProse>,
    /// The projected command-ref hints, in id order.
    pub commands: Vec<CommandHint>,
}

impl Description {
    /// Assemble the whole-menu projection from the resolved definitions.
    ///
    /// `workflows` is the **unfiltered** set (`(id, def, origin pack-id)` triples — every
    /// workflow, not the `creates-task && selectable` catalog); `schemas` is the full
    /// doctype set, each schema paired with its own origin pack-id; `catalogs` is one
    /// `(pack-id, catalog)` pair per **declaring pack**, whose `hint`s are projected as
    /// their union — see [`CommandHint`] for why the winner alone would hide reachable
    /// command-refs. Inputs may arrive in any order — the assembler sorts every output
    /// list (commands by `(id, pack)`), so the projection is deterministic. No LLM call:
    /// the prose is assembled from the authored fields, never generated.
    ///
    /// The origin is **resolved by the caller and carried verbatim**, `None` when no pack
    /// provided the definition ([`DefinitionProse::origin_pack`]): the engine holds
    /// neither the cascade nor the pack-set, so it cannot compute that answer — and does
    /// not guess one.
    pub fn assemble<'a>(
        workflows: impl IntoIterator<Item = (&'a str, &'a WorkflowDef, Option<&'a str>)>,
        schemas: impl IntoIterator<Item = (&'a Schema, Option<&'a str>)>,
        catalogs: impl IntoIterator<Item = (&'a str, &'a CommandCatalog)>,
    ) -> Self {
        let mut definitions: Vec<DefinitionProse> = Vec::new();

        let mut workflow_proses: Vec<DefinitionProse> = workflows
            .into_iter()
            .filter_map(|(id, def, origin_pack)| {
                weave_workflow(id, def).map(|prose| DefinitionProse {
                    kind: DefinitionKind::Workflow,
                    id: id.to_owned(),
                    prose,
                    // The same reason, read through the same predicate the narration uses,
                    // so the prose clause and the structured key cannot disagree.
                    router_hidden: suppression_reason(def).map(str::to_owned),
                    origin_pack: origin_pack.map(str::to_owned),
                })
            })
            .collect();
        workflow_proses.sort_by(|a, b| a.id.cmp(&b.id));

        let mut doctype_proses: Vec<DefinitionProse> = schemas
            .into_iter()
            .filter_map(|(schema, origin_pack)| {
                weave(
                    &schema.ty,
                    schema.description.as_deref(),
                    schema.usage.as_deref(),
                )
                .map(|prose| DefinitionProse {
                    kind: DefinitionKind::Doctype,
                    id: schema.ty.clone(),
                    prose,
                    // A doctype has no router catalog to be hidden from.
                    router_hidden: None,
                    origin_pack: origin_pack.map(str::to_owned),
                })
            })
            .collect();
        doctype_proses.sort_by(|a, b| a.id.cmp(&b.id));

        definitions.append(&mut workflow_proses);
        definitions.append(&mut doctype_proses);

        // Each catalog is a `BTreeMap` (id-sorted within a pack), but the union spans
        // packs, so the merged list is sorted by `(id, pack)` — the key the projection
        // is stable under, and the order a reader scanning for one id wants.
        let mut commands: Vec<CommandHint> = catalogs
            .into_iter()
            .flat_map(|(pack, catalog)| {
                catalog
                    .commands
                    .iter()
                    .map(move |(id, command_ref)| CommandHint {
                        id: id.clone(),
                        pack: pack.to_owned(),
                        hint: command_ref.hint.clone(),
                    })
            })
            .collect();
        commands.sort_by(|a, b| a.id.cmp(&b.id).then_with(|| a.pack.cmp(&b.pack)));

        Self {
            schema_version: SCHEMA_VERSION,
            definitions,
            commands,
        }
    }
}

/// Weave one workflow's full narration: the authored `description:` / `usage:`
/// weave ([`weave`]) plus — for a hidden workflow — the **suppression clause**
/// carrying the declared reason (M43 law 2, `surface-contract.md` → The
/// suppression fence: "`jigc describe` prints the reason for a hidden workflow,
/// so `describe` and the orient catalog stop contradicting each other"):
///
/// - suppressed + narrated → "`<weave>`. It is hidden from the router catalog:
///   `<reason>`."
/// - suppressed with neither authored field → "*id* is hidden from the router
///   catalog: `<reason>`." — the declared reason counts as an authored field, so
///   it never vanishes behind the both-absent skip.
/// - not suppressed → the plain [`weave`], byte-unchanged (inert).
///
/// The clause boundary is controlled the same way [`weave`] controls its own:
/// strip at most one trailing period off each side and rejoin with `. `, so prose
/// that already ends in a period is unchanged and prose that doesn't is fixed. A
/// blank reason is treated as absent (the loader rejects one on a shipped pack;
/// the assembler stays graceful on hand-built defs).
fn weave_workflow(id: &str, def: &WorkflowDef) -> Option<String> {
    let base = weave(id, def.description.as_deref(), def.usage.as_deref());
    let reason = suppression_reason(def);
    match (base, reason) {
        (Some(base), Some(reason)) => {
            let base = base.strip_suffix('.').unwrap_or(&base);
            let reason = reason.strip_suffix('.').unwrap_or(reason);
            Some(format!(
                "{base}. It is hidden from the router catalog: {reason}."
            ))
        }
        (None, Some(reason)) => {
            let reason = reason.strip_suffix('.').unwrap_or(reason);
            Some(format!("{id} is hidden from the router catalog: {reason}."))
        }
        (base, None) => base,
    }
}

/// The declared router-suppression reason of a workflow, or `None` when it is not
/// hidden — the **one** predicate both the prose clause ([`weave_workflow`]) and the
/// structured [`DefinitionProse::router_hidden`] key read, so the two renderings of
/// the same fact cannot drift. A blank reason counts as absent (the loader rejects
/// one on a shipped pack; the assembler stays graceful on hand-built defs).
fn suppression_reason(def: &WorkflowDef) -> Option<&str> {
    def.suppressed
        .as_ref()
        .map(|s| s.reason.trim())
        .filter(|reason| !reason.is_empty())
}

/// Weave one definition's authored `description:` / `usage:` into a structured
/// prose sentence (`introspection.md` → The authored fields):
///
/// - both present → "*id* is `<description>`. Reach for it when `<usage>`."
/// - description only → "*id* is `<description>`."
/// - usage only → "Reach for `id` when `<usage>`."
/// - both absent → `None` (skip-on-absent).
///
/// A field that is present but blank (whitespace-only) is treated as absent, so a
/// `description: ""` does not narrate an empty clause.
fn weave(id: &str, description: Option<&str>, usage: Option<&str>) -> Option<String> {
    let description = description.map(str::trim).filter(|d| !d.is_empty());
    let usage = usage.map(str::trim).filter(|u| !u.is_empty());

    match (description, usage) {
        (Some(description), Some(usage)) => {
            // Control the clause boundary ourselves rather than depending on the
            // authored description ending in a period — a period-less description
            // would otherwise run on into "Reach for it when". Strip one trailing
            // period (if any) and always rejoin with ". ", so prose that already
            // ends in a period is byte-unchanged and prose that doesn't is fixed.
            let description = description.strip_suffix('.').unwrap_or(description);
            Some(format!("{id} is {description}. Reach for it when {usage}"))
        }
        (Some(description), None) => Some(format!("{id} is {description}")),
        (None, Some(usage)) => Some(format!("Reach for {id} when {usage}")),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compose::{CommandArg, CommandRef, Suppressed};
    use std::collections::BTreeMap;

    /// A minimal `WorkflowDef` carrying only the two authored fields the weave
    /// reads — the other fields are inert for this projection.
    fn workflow(description: Option<&str>, usage: Option<&str>) -> WorkflowDef {
        WorkflowDef {
            when: None,
            description: description.map(str::to_owned),
            usage: usage.map(str::to_owned),
            creates_task: true,
            selectable: true,
            suppressed: None,
            allows_create: Vec::new(),
            reads: Vec::new(),
            includes: Vec::new(),
        }
    }

    /// A minimal `Schema` carrying only `type` + the two authored fields.
    fn schema(ty: &str, description: Option<&str>, usage: Option<&str>) -> Schema {
        Schema {
            ty: ty.to_owned(),
            location: None,
            id_from: None,
            description: description.map(str::to_owned),
            usage: usage.map(str::to_owned),
            display_title: None,
            placement: None,
            singleton: false,
            sections: Vec::new(),
        }
    }

    fn catalog_with(entries: &[(&str, &str)]) -> CommandCatalog {
        let mut commands = BTreeMap::new();
        for (id, hint) in entries {
            commands.insert(
                (*id).to_owned(),
                CommandRef {
                    command: "jigc".to_owned(),
                    args: vec![CommandArg::Literal {
                        literal: "noop".to_owned(),
                    }],
                    stdin: None,
                    hint: (*hint).to_owned(),
                },
            );
        }
        CommandCatalog { commands }
    }

    /// A both-fields workflow weaves the two-clause sentence containing both
    /// authored strings, in `is … Reach for it when …` order.
    #[test]
    fn both_fields_weave_a_two_clause_sentence() {
        let def = workflow(
            Some("one end-to-end scoped change."),
            Some("the work is one coherent change you can hold in your head."),
        );
        let description = Description::assemble(
            [("single-task", &def, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(description.definitions.len(), 1);
        let prose = &description.definitions[0].prose;
        assert!(
            prose.contains("one end-to-end scoped change."),
            "the woven sentence carries the description string: {prose:?}"
        );
        assert!(
            prose.contains("the work is one coherent change you can hold in your head."),
            "the woven sentence carries the usage string: {prose:?}"
        );
        // The two are two semantic clauses, in order, not a concatenated blob.
        let is_at = prose.find("is ").expect("an `is` clause");
        let reach_at = prose
            .find("Reach for it when ")
            .expect("a `Reach for it when` clause");
        assert!(
            is_at < reach_at,
            "the `is` clause precedes the `Reach for it when` clause: {prose:?}"
        );
        assert!(
            prose.starts_with("single-task is "),
            "the weave names the definition first: {prose:?}"
        );
        // The weave supplies the "Reach for it when " lead itself — exactly once.
        // The authored `usage:` is a **bare clause** (no own lead), so a single
        // occurrence is the only well-formed shape. Asserting the *count* (not just
        // presence) is what would catch a weave that doubled the lead — the
        // production defect a presence-only `find` masks.
        assert_eq!(
            prose.matches("Reach for it when ").count(),
            1,
            "the weave supplies the usage lead exactly once: {prose:?}"
        );
    }

    /// The weave must not double its own lead when handed real pack-shaped prose.
    ///
    /// The pack authors `usage:` as a **bare clause** (`design/introspection.md` →
    /// the weave example carries usage WITHOUT the "Reach for it when" prefix), and
    /// the weave prepends the fixed lead. Fed verbatim copies of two real shipped
    /// `usage:` strings, the woven sentence must carry the lead exactly once and
    /// must never produce the doubled "Reach for it when Reach for it when …" shape.
    /// This is the unit-level counterpart of the emitted-bytes guard in
    /// `crates/cli/tests/describe.rs` — it pins the weave's contract at the source
    /// so a regression is caught here, not only at the binary boundary.
    #[test]
    fn weave_does_not_double_the_lead_on_real_pack_prose() {
        // Verbatim from the shipped pack (single-task.yaml / adr.yaml `usage:`).
        let single_task_usage = "the work is one coherent change you can hold in your head and carry from \
             intent to commit in a single pass.";
        let adr_usage = "a choice is worth preserving with its rationale, so a later \
                         reader can recover why the call was made or supersede it on \
                         the record.";

        for (id, usage) in [("single-task", single_task_usage), ("adr", adr_usage)] {
            let prose =
                weave(id, Some("an end-to-end scoped change."), Some(usage)).expect("woven");
            assert_eq!(
                prose.matches("Reach for it when ").count(),
                1,
                "real pack prose weaves the lead exactly once: {prose:?}"
            );
            assert!(
                !prose.contains("Reach for it when Reach for it"),
                "the doubled lead must never appear: {prose:?}"
            );
        }
    }

    /// A description-only definition narrates "X is …" and omits the usage clause.
    #[test]
    fn description_only_narrates_is_clause() {
        let def = workflow(Some("one end-to-end scoped change."), None);
        let description = Description::assemble(
            [("single-task", &def, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(description.definitions.len(), 1);
        let prose = &description.definitions[0].prose;
        assert_eq!(prose, "single-task is one end-to-end scoped change.");
        assert!(
            !prose.contains("Reach for"),
            "no usage clause when usage is absent: {prose:?}"
        );
    }

    /// A usage-only definition narrates "Reach for … when …" and omits the
    /// identity clause.
    #[test]
    fn usage_only_narrates_reach_for_when_clause() {
        let def = workflow(None, Some("a decision is worth preserving."));
        let description = Description::assemble(
            [("adr", &def, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(description.definitions.len(), 1);
        let prose = &description.definitions[0].prose;
        assert_eq!(prose, "Reach for adr when a decision is worth preserving.");
        assert!(
            !prose.starts_with("adr is"),
            "no identity clause when description is absent: {prose:?}"
        );
    }

    /// A both-absent definition is omitted entirely (skip-on-absent) — describe
    /// is a menu, not a gate.
    #[test]
    fn both_absent_definition_is_skipped() {
        let narrated = workflow(Some("a narrated workflow."), None);
        let silent = workflow(None, None);
        let description = Description::assemble(
            [("narrated", &narrated, None), ("silent", &silent, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(
            description.definitions.len(),
            1,
            "the both-absent definition is omitted"
        );
        assert_eq!(description.definitions[0].id, "narrated");
    }

    /// A whitespace-only field is treated as absent — a blank `description:`
    /// narrates only the usage clause, not an empty identity clause.
    #[test]
    fn blank_field_is_treated_as_absent() {
        let def = workflow(Some("   "), Some("you need the menu."));
        let description =
            Description::assemble([("x", &def, None)], std::iter::empty(), std::iter::empty());

        assert_eq!(
            description.definitions[0].prose,
            "Reach for x when you need the menu."
        );
    }

    /// A hidden (`selectable: false`) workflow's narration carries the suppression
    /// clause — it says the workflow is hidden from the router catalog and names
    /// the declared reason (M43 T6, `surface-contract.md` → The suppression fence:
    /// "`jigc describe` prints the reason for a hidden workflow"). The clause joins
    /// *after* the authored weave, with a controlled sentence boundary.
    #[test]
    fn suppressed_workflow_narrates_hidden_from_the_router_catalog_with_reason() {
        let mut def = workflow(
            Some("one sub-task of a milestone."),
            Some("a milestone execution fans out."),
        );
        def.selectable = false;
        def.suppressed = Some(Suppressed {
            reason: "spawned by fan-out, never picked".to_owned(),
            expires: "never".to_owned(),
            door: None,
        });
        let description = Description::assemble(
            [("sub-task", &def, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(description.definitions.len(), 1);
        assert_eq!(
            description.definitions[0].prose,
            "sub-task is one sub-task of a milestone. Reach for it when a milestone \
             execution fans out. It is hidden from the router catalog: spawned by \
             fan-out, never picked.",
        );
    }

    /// The omitting context is inert: a workflow WITHOUT a `suppressed:` block
    /// narrates exactly the pre-M43 weave — no hidden clause, no changed bytes.
    #[test]
    fn unsuppressed_workflow_narration_carries_no_hidden_clause() {
        let def = workflow(
            Some("one sub-task of a milestone."),
            Some("a milestone execution fans out."),
        );
        let description = Description::assemble(
            [("sub-task", &def, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(
            description.definitions[0].prose,
            "sub-task is one sub-task of a milestone. Reach for it when a milestone execution fans out.",
        );
        assert!(
            !description.definitions[0]
                .prose
                .contains("hidden from the router catalog"),
            "the hidden clause must not appear on an unsuppressed workflow",
        );
    }

    /// A suppressed workflow with NEITHER authored field is still narrated — the
    /// declared reason is an authored field for skip-on-absent purposes, so law 2's
    /// "describe prints the reason for a hidden workflow" holds unconditionally
    /// (the reason must not vanish behind the both-absent skip).
    #[test]
    fn suppressed_only_workflow_is_still_narrated() {
        let mut def = workflow(None, None);
        def.selectable = false;
        def.suppressed = Some(Suppressed {
            reason: "verb-routed — reached only through `jigc migrate`".to_owned(),
            expires: "never".to_owned(),
            door: None,
        });
        let description = Description::assemble(
            [("migrate-spec", &def, None)],
            std::iter::empty(),
            std::iter::empty(),
        );

        assert_eq!(description.definitions.len(), 1);
        assert_eq!(
            description.definitions[0].prose,
            "migrate-spec is hidden from the router catalog: verb-routed — reached \
             only through `jigc migrate`.",
        );
    }

    /// Command-ref hints are assembled as prose: a known hint is projected
    /// verbatim under its id (`hint`'s first projection consumer).
    #[test]
    fn command_ref_hints_are_assembled() {
        let catalog = catalog_with(&[
            (
                "run-ingest",
                "Ingest the agent's drafted prose into the named slot.",
            ),
            (
                "finalize",
                "Validate, render the commit, and commit the task.",
            ),
        ]);
        let description =
            Description::assemble(std::iter::empty(), std::iter::empty(), [("dev", &catalog)]);

        assert_eq!(description.commands.len(), 2);
        let ingest = description
            .commands
            .iter()
            .find(|c| c.id == "run-ingest")
            .expect("the run-ingest hint is projected");
        assert_eq!(
            ingest.hint,
            "Ingest the agent's drafted prose into the named slot."
        );
        assert_eq!(
            ingest.pack, "dev",
            "the projected entry names the pack whose catalog declares it"
        );
    }

    /// The union across packs, and the reason it is one: two packs may each declare
    /// the **same** command-ref id with a different `hint`, and a workflow resolves
    /// `{{cli.<id>}}` against its *own* origin pack — so both are reachable and both
    /// are projected, attributed. Projecting the precedence winner alone would hide a
    /// reachable command-ref (M49 Increment 11 / T6).
    #[test]
    fn a_collided_id_is_projected_once_per_declaring_pack() {
        let dev = catalog_with(&[("show-doc", "read a managed doc"), ("run-ingest", "scan")]);
        let methodology = catalog_with(&[("show-doc", "read a work-doc back")]);

        let description = Description::assemble(
            std::iter::empty(),
            std::iter::empty(),
            [("dev", &dev), ("methodology", &methodology)],
        );

        let projected: Vec<(&str, &str, &str)> = description
            .commands
            .iter()
            .map(|c| (c.id.as_str(), c.pack.as_str(), c.hint.as_str()))
            .collect();
        assert_eq!(
            projected,
            vec![
                ("run-ingest", "dev", "scan"),
                ("show-doc", "dev", "read a managed doc"),
                ("show-doc", "methodology", "read a work-doc back"),
            ],
            "the union carries each declaring pack's own entry, sorted by (id, pack)",
        );
    }

    /// Every narrated definition carries the origin its caller resolved — verbatim, per
    /// definition, across both kinds — and a definition the caller attributes to **no**
    /// pack carries `None` in the same projection, so the key discriminates rather than
    /// merely existing (M50 Increment 5 / T1).
    ///
    /// The engine holds neither cascade nor pack-set: it cannot compute this answer, so
    /// the assembler's whole obligation is to carry the caller's, unaltered and unmixed.
    #[test]
    fn each_definition_carries_the_origin_its_caller_resolved() {
        let shipped = workflow(Some("a workflow the base pack ships."), None);
        let shadowed = workflow(Some("a workflow the project layer replaced."), None);
        let doctype = schema("adr", Some("a dated decision record."), None);

        let description = Description::assemble(
            [
                ("shipped", &shipped, Some("dev")),
                ("shadowed", &shadowed, None),
            ],
            [(&doctype, Some("methodology"))],
            std::iter::empty(),
        );

        let origins: Vec<(&str, Option<&str>)> = description
            .definitions
            .iter()
            .map(|d| (d.id.as_str(), d.origin_pack.as_deref()))
            .collect();
        assert_eq!(
            origins,
            vec![
                ("shadowed", None),
                ("shipped", Some("dev")),
                ("adr", Some("methodology")),
            ],
            "each definition carries its OWN caller-resolved origin — the un-attributed one \
             stays null rather than borrowing a neighbour's pack",
        );
    }

    /// The projection is deterministic across a scrambled input: the same
    /// definitions fed under two divergent orders produce byte-identical output
    /// (sorted by id within kind; workflows before doctypes).
    #[test]
    fn projection_is_deterministic_across_scrambled_input() {
        let zebra = workflow(Some("the last workflow alphabetically."), None);
        let alpha = workflow(Some("the first workflow alphabetically."), None);
        let yak = schema("yak", Some("a shaggy doctype."), None);
        let bison = schema("bison", Some("a sturdy doctype."), None);
        // Two catalogs with a genuinely COLLIDING id (`a-cmd` in both, different
        // hints) — a non-overlapping pair would prove nothing about the merge.
        let dev = catalog_with(&[("z-cmd", "last command"), ("a-cmd", "first command")]);
        let methodology = catalog_with(&[("a-cmd", "the methodology reading")]);

        // Id order.
        let forward = Description::assemble(
            [("alpha", &alpha, None), ("zebra", &zebra, None)],
            [(&bison, None), (&yak, None)],
            [("dev", &dev), ("methodology", &methodology)],
        );
        // Reverse order — the same inputs scrambled, the catalogs included.
        let reverse = Description::assemble(
            [("zebra", &zebra, None), ("alpha", &alpha, None)],
            [(&yak, None), (&bison, None)],
            [("methodology", &methodology), ("dev", &dev)],
        );

        assert_eq!(
            forward, reverse,
            "byte-identical output across divergent input orders"
        );

        // The sort is the documented (workflows-then-doctypes, each id-sorted)
        // order, not input order.
        let ids: Vec<(DefinitionKind, &str)> = forward
            .definitions
            .iter()
            .map(|d| (d.kind, d.id.as_str()))
            .collect();
        assert_eq!(
            ids,
            vec![
                (DefinitionKind::Workflow, "alpha"),
                (DefinitionKind::Workflow, "zebra"),
                (DefinitionKind::Doctype, "bison"),
                (DefinitionKind::Doctype, "yak"),
            ],
        );
        // The merged command list is keyed `(id, pack)` — the colliding id keeps both
        // declaring packs' entries, in one order regardless of the input order.
        let cmd_keys: Vec<(&str, &str)> = forward
            .commands
            .iter()
            .map(|c| (c.id.as_str(), c.pack.as_str()))
            .collect();
        assert_eq!(
            cmd_keys,
            vec![("a-cmd", "dev"), ("a-cmd", "methodology"), ("z-cmd", "dev"),],
        );
    }
}
