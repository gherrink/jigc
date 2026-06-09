//! Introspection — the `describe` whole-menu projection assembler.
//!
//! Assembles a [`Description`]: a prose projection of the resolved definitions
//! (every workflow, every doctype, plus the command-ref `hint`s) for the
//! `jigc describe` self-description surface (`design/introspection.md`). The
//! engine **assembles, never generates** — it makes no LLM call; the prose is
//! the human-authored `description:` / `usage:` fields carried on the
//! definitions themselves, woven into a structured sentence here, and the
//! command-ref `hint`s carried verbatim. The result is presentation-free: it
//! carries the woven prose, not a rendered surface (the `cli::render` free-prose
//! renderer frames it — `module-layout.md` → CLI renders).
//!
//! The weave (`introspection.md` → The authored fields): a definition with both
//! fields becomes "*X* is `<description>`. Reach for it when `<usage>`."; with
//! only one of the two it narrates *that one*; with **both absent** it is
//! **skipped** (skip-on-absent — describe is a menu, not a gate). Enumeration is
//! over the **unfiltered** definition set (every workflow, not the
//! `creates-task && selectable` catalog), sorted by id so the projection is
//! deterministic regardless of input order.

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

/// One narrated definition: its stable `id`, its `kind`, and the woven prose
/// sentence assembled from the authored `description:` / `usage:` fields.
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
}

/// One command-ref's projected `hint` — the command's `{{cli.<id>}}` id and its
/// authored one-line `hint`, carried verbatim as prose (`hint`'s first
/// projection consumer — `introspection.md` → Command surface).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandHint {
    /// The command-ref's `{{cli.<id>}}` id.
    pub id: String,
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
    /// `workflows` is the **unfiltered** set (`(id, def)` pairs — every workflow,
    /// not the `creates-task && selectable` catalog); `schemas` is the full
    /// doctype set; `catalog` is the command catalog whose `hint`s are projected.
    /// Inputs may arrive in any order — the assembler sorts every output list by
    /// id, so the projection is deterministic. No LLM call: the prose is
    /// assembled from the authored fields, never generated.
    pub fn assemble<'a>(
        workflows: impl IntoIterator<Item = (&'a str, &'a WorkflowDef)>,
        schemas: impl IntoIterator<Item = &'a Schema>,
        catalog: &CommandCatalog,
    ) -> Self {
        let mut definitions: Vec<DefinitionProse> = Vec::new();

        let mut workflow_proses: Vec<DefinitionProse> = workflows
            .into_iter()
            .filter_map(|(id, def)| {
                weave(id, def.description.as_deref(), def.usage.as_deref()).map(|prose| {
                    DefinitionProse {
                        kind: DefinitionKind::Workflow,
                        id: id.to_owned(),
                        prose,
                    }
                })
            })
            .collect();
        workflow_proses.sort_by(|a, b| a.id.cmp(&b.id));

        let mut doctype_proses: Vec<DefinitionProse> = schemas
            .into_iter()
            .filter_map(|schema| {
                weave(
                    &schema.ty,
                    schema.description.as_deref(),
                    schema.usage.as_deref(),
                )
                .map(|prose| DefinitionProse {
                    kind: DefinitionKind::Doctype,
                    id: schema.ty.clone(),
                    prose,
                })
            })
            .collect();
        doctype_proses.sort_by(|a, b| a.id.cmp(&b.id));

        definitions.append(&mut workflow_proses);
        definitions.append(&mut doctype_proses);

        // The catalog is a `BTreeMap`, so iteration is already id-sorted; collect
        // it into the projection's stable shape.
        let commands = catalog
            .commands
            .iter()
            .map(|(id, command_ref)| CommandHint {
                id: id.clone(),
                hint: command_ref.hint.clone(),
            })
            .collect();

        Self {
            schema_version: SCHEMA_VERSION,
            definitions,
            commands,
        }
    }
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
    use crate::compose::{CommandArg, CommandRef};
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
            singleton: false,
            sections: Vec::new(),
        }
    }

    fn empty_catalog() -> CommandCatalog {
        CommandCatalog {
            commands: BTreeMap::new(),
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
            [("single-task", &def)],
            std::iter::empty(),
            &empty_catalog(),
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
            [("single-task", &def)],
            std::iter::empty(),
            &empty_catalog(),
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
        let description =
            Description::assemble([("adr", &def)], std::iter::empty(), &empty_catalog());

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
            [("narrated", &narrated), ("silent", &silent)],
            std::iter::empty(),
            &empty_catalog(),
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
            Description::assemble([("x", &def)], std::iter::empty(), &empty_catalog());

        assert_eq!(
            description.definitions[0].prose,
            "Reach for x when you need the menu."
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
        let description = Description::assemble(std::iter::empty(), std::iter::empty(), &catalog);

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
        let catalog = catalog_with(&[("z-cmd", "last command"), ("a-cmd", "first command")]);

        // Id order.
        let forward = Description::assemble(
            [("alpha", &alpha), ("zebra", &zebra)],
            [&bison, &yak],
            &catalog,
        );
        // Reverse order — the same inputs scrambled.
        let reverse = Description::assemble(
            [("zebra", &zebra), ("alpha", &alpha)],
            [&yak, &bison],
            &catalog,
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
        let cmd_ids: Vec<&str> = forward.commands.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(cmd_ids, vec!["a-cmd", "z-cmd"]);
    }
}
