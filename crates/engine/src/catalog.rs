//! Building the workflow [`Catalog`] from a [`PackSource`].
//!
//! Orientation's catalog is *derived*, not stored: for each workflow the pack
//! provides, the engine reads its bytes, parses the **config-family YAML
//! front-matter** (the `---`-fenced block at the top of a workflow definition,
//! per `design/workflow-dialect.md` → On-disk definition format) and lifts the
//! one-line `when` selection hint out of it. The entry's **id is the workflow's
//! [`ResourceId`]** — the filename stem the pack assigns — never a front-matter
//! field (the id is the filename; the front-matter carries only metadata).
//!
//! This is the engine resolving over a fed-in layer (feed-layers-in /
//! assert-results-out): it compiles in no pack content of its own.

use crate::packsource::{PackResourceKind, PackSource, ResourceId};
use crate::result::{Catalog, CatalogEntry};
use serde::Deserialize;
use thiserror::Error;

/// Why building the catalog from a pack failed.
#[derive(Debug, Error)]
pub enum CatalogError {
    /// A workflow's bytes were not valid UTF-8 (definitions are text).
    #[error("workflow `{id}` is not valid UTF-8")]
    NotUtf8 { id: ResourceId },

    /// A workflow had no `---`-fenced front-matter block to parse.
    #[error("workflow `{id}` has no `---`-fenced front-matter block")]
    MissingFrontMatter { id: ResourceId },

    /// A workflow's front-matter was not parseable config-family YAML.
    #[error("workflow `{id}` has malformed front-matter: {source}")]
    MalformedFrontMatter {
        id: ResourceId,
        source: serde_yaml_ng::Error,
    },

    /// A workflow's front-matter parsed but carried no `when` selection hint.
    #[error("workflow `{id}` is missing the required `when` selection hint")]
    MissingWhen { id: ResourceId },
}

/// The subset of workflow front-matter the catalog reads: just the `when` hint.
///
/// The full metadata set (`creates-task`, `allows-create`, …) lands when
/// composition needs it; the catalog needs only the selection hint.
#[derive(Deserialize)]
struct WorkflowFrontMatter {
    when: Option<String>,
}

/// Build the workflow [`Catalog`] from `pack`, in the pack's `list` order.
///
/// Reads every [`PackResourceKind::Workflows`] resource, parses its front-matter
/// for the `when` hint, and pairs that with the resource's id. Any workflow that
/// can't be read or whose front-matter lacks `when` is a hard [`CatalogError`] —
/// a workflow earns its place in the catalog by declaring its hint, so a missing
/// one is a definition bug, not a silently-skipped entry.
pub fn build_catalog(pack: &dyn PackSource) -> Result<Catalog, CatalogError> {
    let mut entries = Vec::new();
    for id in pack.list(PackResourceKind::Workflows) {
        let bytes = pack
            .read(PackResourceKind::Workflows, &id)
            .expect("listed workflow id must be readable");
        let when = parse_when(&id, &bytes)?;
        entries.push(CatalogEntry::new(id.as_str(), when));
    }
    Ok(Catalog::new(entries))
}

/// Extract the `when` hint from one workflow's raw definition bytes.
fn parse_when(id: &ResourceId, bytes: &[u8]) -> Result<String, CatalogError> {
    let text = std::str::from_utf8(bytes).map_err(|_| CatalogError::NotUtf8 { id: id.clone() })?;
    let front_matter =
        front_matter(text).ok_or_else(|| CatalogError::MissingFrontMatter { id: id.clone() })?;
    let parsed: WorkflowFrontMatter = serde_yaml_ng::from_str(front_matter).map_err(|source| {
        CatalogError::MalformedFrontMatter {
            id: id.clone(),
            source,
        }
    })?;
    parsed
        .when
        .filter(|w| !w.trim().is_empty())
        .ok_or_else(|| CatalogError::MissingWhen { id: id.clone() })
}

/// Slice out the YAML inside the leading `---`-fenced front-matter block.
///
/// Recognizes a `---` fence on the first line and the matching closing `---`
/// line (a line that is exactly `---`); returns the bytes between them. Returns
/// `None` if the text does not open with a fence (no front-matter block). The
/// canonical writer emits LF (`parsing.md` → Round-trip guarantees), so this
/// reads the LF form.
///
/// Shared with [`crate::compose`], which loads the full workflow front-matter
/// (not just `when`) over the same slicer — one front-matter recognizer, reused.
pub(crate) fn front_matter(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---\n")?;
    // The closing fence is a line that is exactly `---`.
    let end = rest
        .match_indices("---")
        .find(|(i, _)| {
            let at_line_start = *i == 0 || rest[..*i].ends_with('\n');
            let after = &rest[i + 3..];
            let at_line_end = after.is_empty() || after.starts_with('\n');
            at_line_start && at_line_end
        })
        .map(|(i, _)| i)?;
    Some(&rest[..end])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// A minimal in-memory `PackSource` seeded with workflow bytes only — proves
    /// the catalog builder resolves over a fed-in layer with no engine-embedded
    /// pack content.
    struct FakePack {
        workflows: HashMap<ResourceId, Vec<u8>>,
    }

    impl FakePack {
        fn with(pairs: Vec<(&str, &str)>) -> Self {
            let workflows = pairs
                .into_iter()
                .map(|(id, body)| (ResourceId::from(id), body.as_bytes().to_vec()))
                .collect();
            Self { workflows }
        }
    }

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            "0.0.0".to_owned()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            if kind != PackResourceKind::Workflows {
                return Vec::new();
            }
            let mut ids: Vec<ResourceId> = self.workflows.keys().cloned().collect();
            ids.sort();
            ids
        }

        fn read(
            &self,
            kind: PackResourceKind,
            id: &ResourceId,
        ) -> Result<Vec<u8>, crate::packsource::PackError> {
            if kind == PackResourceKind::Workflows
                && let Some(bytes) = self.workflows.get(id)
            {
                return Ok(bytes.clone());
            }
            Err(crate::packsource::PackError::NotFound {
                kind,
                id: id.clone(),
            })
        }
    }

    /// The core done-criterion: a workflow whose `---`-fenced front-matter carries
    /// a `when` hint yields a catalog entry whose id is the resource id and whose
    /// `when` is the exact front-matter text.
    #[test]
    fn catalog_entry_carries_resource_id_and_front_matter_when() {
        let pack = FakePack::with(vec![(
            "single-task",
            "---\nwhen: \"Implement one well-scoped change against an existing spec.\"\n---\n{{ include: step:locate }}\n",
        )]);

        let catalog = build_catalog(&pack).expect("builds");

        assert_eq!(catalog.entries().len(), 1);
        let entry = &catalog.entries()[0];
        assert_eq!(entry.id, "single-task");
        assert_eq!(
            entry.when,
            "Implement one well-scoped change against an existing spec."
        );
    }

    /// Front-matter carrying nested config-family keys (e.g. `allows-create`)
    /// alongside `when` still parses — the catalog reads only `when` and ignores
    /// the rest, so richer definitions don't break it.
    #[test]
    fn catalog_ignores_other_front_matter_keys() {
        let pack = FakePack::with(vec![(
            "single-task",
            "---\nwhen: \"Implement one scoped change.\"\nallows-create: [{type: adr, as: decision}]\ncreates-task: true\n---\nbody\n",
        )]);

        let catalog = build_catalog(&pack).expect("builds despite extra keys");

        assert_eq!(catalog.entries()[0].when, "Implement one scoped change.");
    }

    /// Multiple workflows surface in the pack's `list` order, each with its own id.
    #[test]
    fn catalog_has_one_entry_per_workflow_in_list_order() {
        let pack = FakePack::with(vec![
            (
                "router",
                "---\nwhen: \"Help me pick a workflow.\"\n---\nbody\n",
            ),
            ("single-task", "---\nwhen: \"Do one change.\"\n---\nbody\n"),
        ]);

        let catalog = build_catalog(&pack).expect("builds");

        let ids: Vec<&str> = catalog.entries().iter().map(|e| e.id.as_str()).collect();
        assert_eq!(ids, vec!["router", "single-task"]);
    }

    /// A workflow whose front-matter omits `when` is a definition bug: a clear,
    /// id-bearing error, never a silently-dropped entry.
    #[test]
    fn workflow_without_when_is_a_clear_error() {
        let pack = FakePack::with(vec![("broken", "---\ncreates-task: true\n---\nbody\n")]);

        let err = build_catalog(&pack).expect_err("missing `when` errors");
        assert!(
            matches!(err, CatalogError::MissingWhen { ref id } if id.as_str() == "broken"),
            "expected MissingWhen for `broken`, got {err:?}",
        );
    }

    /// A workflow with no `---`-fenced front-matter block at all is a clear error.
    #[test]
    fn workflow_without_front_matter_is_a_clear_error() {
        let pack = FakePack::with(vec![("plain", "just a body, no fences\n")]);

        let err = build_catalog(&pack).expect_err("missing front-matter errors");
        assert!(
            matches!(err, CatalogError::MissingFrontMatter { ref id } if id.as_str() == "plain"),
            "expected MissingFrontMatter for `plain`, got {err:?}",
        );
    }
}
