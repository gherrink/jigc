//! The `override-default` classifier — maps each recorded delta against a given
//! [`PackSource`] to one of **clean / conflict / orphaned / needs-rebasing**.
//!
//! This is M5's core logic, run by the bespoke upgrade-time path (`jigc upgrade`),
//! not the per-task `validate_task` seam ([validation.md](../../../design/validation.md)
//! → `override-default`). When pack-default goes `v1 → v2`, the classifier re-applies
//! each recorded delta and asks up to three deterministic questions against v2
//! ([overrides.md](../../../design/overrides.md) → Upgrade reconciliation):
//!
//! 1. **Does the target still exist?** No → **`orphaned`** (this task, T1).
//! 2. *(T3)* a content-bearing delta with no recorded base-hash → **`needs-rebasing`**.
//! 3. *(T2)* the target's content changed upstream → **`conflict`**, else **`clean`**.
//!
//! **The probe re-reads the pack-default unit via [`PackSource::read`], never the
//! cascade-resolved owner** — the tracked-fork shadow-bypass (`overrides.md` →
//! "The probe re-reads the *pack-default* unit"). A shadow-aware read would compare
//! a fork to itself and falsely say `clean`. The existence question (T1) inherits the
//! same pack-direct read: a content-bearing target *exists in v2* iff
//! `pack.read(Steps, <step-id>)` is `Ok`.
//!
//! Findings ship **blocking-by-default** in M5 (cascade-tunability is M6) and carry a
//! human-readable `String` route; each delta is identified by its **target string**
//! `workflow:<id>#<step-id>` — a delta has no stable id, so its target is its identity.

use crate::cascade::{
    Anchor, StructuralBasis, StructuralDelta, StructuralTarget, TrackedForkDelta,
};
use crate::finding::Finding;
use crate::packsource::{PackError, PackResourceKind, PackSource, ResourceId};

/// The recorded delta surfaces the classifier reconciles — the M5-relevant members
/// of `load_project_layer`'s 5-tuple, borrowed for the duration of the classify call.
///
/// The classifier consults the **content-bearing** surfaces (`structural` filtered to
/// `replace`/`remove`, `forks`) for the existence + content questions; `bases` pairs a
/// recorded base-hash to a content-bearing target (read by the T2/T3 branches). The
/// `scalar`/`slot-fill` surfaces ride along for completeness (their existence questions
/// land in later tasks) and are not yet consulted here.
#[derive(Clone, Copy)]
pub struct RecordedDeltas<'a> {
    /// The phase-4 `structural-op` deltas (`insert` / `replace` / `remove`).
    pub structural: &'a [StructuralDelta],
    /// The `tracked-fork` recording deltas — content-bearing, shadow-bypass read.
    pub forks: &'a [TrackedForkDelta],
    /// The M5 base-hash basis records, keyed by a content-bearing delta's target.
    pub bases: &'a [StructuralBasis],
}

/// Classify each recorded delta against the current `pack`, returning one
/// [`Finding`] per non-`clean` delta (`clean` deltas emit nothing —
/// [overrides.md](../../../design/overrides.md) → Report).
///
/// **T1 scope — question 1 (existence → `orphaned`) for the content-bearing kinds.**
/// For each `replace` / `remove` [`StructuralDelta`] and each [`TrackedForkDelta`], the
/// target's step id (`#<step-id>`, an [`Anchor::At`]) must still resolve in v2: a
/// pack-direct `pack.read(Steps, <step-id>)` that is `Ok` means the target exists (no
/// orphaned finding — and **no** content compare yet, so no spurious `conflict`); an
/// `Err(NotFound)` means the v2 pack omits the target → one blocking `orphaned`
/// [`Finding`] carrying the delta's target string and a remove/re-target route.
///
/// The read is **pack-direct** (`PackSource::read`), never the cascade-resolved owner —
/// the shadow-bypass the fork case depends on (`overrides.md` → the probe re-reads the
/// pack-default unit). T2/T3 add the content compare and the `needs-rebasing` branch.
pub fn classify(deltas: RecordedDeltas<'_>, pack: &dyn PackSource) -> Vec<Finding> {
    let mut findings = Vec::new();

    for delta in deltas.structural {
        // `insert` depends on anchor existence (a different question, a later task);
        // only the content-bearing `replace` / `remove` reach the step-existence read.
        let target = match delta {
            StructuralDelta::Replace { target, .. } | StructuralDelta::Remove { target } => target,
            StructuralDelta::Insert { .. } => continue,
        };
        if let Some(finding) = orphaned_if_absent(target, pack) {
            findings.push(finding);
        }
    }

    for fork in deltas.forks {
        if let Some(finding) = orphaned_if_absent(&fork.target, pack) {
            findings.push(finding);
        }
    }

    findings
}

/// Question 1 for a content-bearing target: a blocking `orphaned` [`Finding`] when the
/// current pack omits the target's step, or `None` when it still carries it.
///
/// The existence read is **pack-direct** — `pack.read(Steps, <step-id>)` — so a project
/// shadow of the same id never masks an upstream removal (the shadow-bypass). A target
/// present in v2 yields `None`: T1 makes no content compare, so a still-present target
/// raises **no** finding (no spurious `conflict`).
fn orphaned_if_absent(target: &StructuralTarget, pack: &dyn PackSource) -> Option<Finding> {
    let step_id = at_step_id(target);
    match pack.read(PackResourceKind::Steps, &ResourceId::from(step_id)) {
        Ok(_) => None,
        Err(PackError::NotFound { .. }) => Some(orphaned(target)),
    }
}

/// One blocking `orphaned` [`Finding`] for a content-bearing delta whose target the
/// current pack omits, identified by its target string and routed to remove/re-target.
fn orphaned(target: &StructuralTarget) -> Finding {
    let target_str = render_target(target);
    Finding::block(
        "override-default.target-exists",
        format!("override target `{target_str}` no longer exists in the current pack"),
        format!("remove or re-target the delta on `{target_str}`"),
    )
}

/// The `#<step-id>` of a content-bearing target — its [`Anchor::At`] step id. Content
/// kinds (`replace` / `remove` / `tracked-fork`) always carry an `At` anchor; the other
/// anchors are insert-only, never reached here.
fn at_step_id(target: &StructuralTarget) -> &str {
    match &target.anchor {
        Anchor::At(id) | Anchor::After(id) | Anchor::Before(id) => id,
    }
}

/// Render a [`StructuralTarget`] as its identity string `workflow:<id>#<step-id>` — the
/// engine-side spelling of the delta's target (the cli spells the same at
/// `config.rs`'s `at_step` / `append_fork`; the engine gains its own so the classifier
/// has no cli dependency). A delta has no stable id, so this string is its identity in
/// every finding ([overrides.md](../../../design/overrides.md) → Classify).
fn render_target(target: &StructuralTarget) -> String {
    format!("workflow:{}#{}", target.workflow_id, at_step_id(target))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cascade::Anchor;
    use crate::finding::Severity;
    use crate::packsource::PackResourceKind;
    use std::collections::HashMap;

    /// A trivial in-memory `PackSource` — the [`crate::packsource`] test-double
    /// pattern, here seeded as a two-version pack so the classifier can be exercised
    /// against `v2` after `v1`.
    struct FakePack {
        version: String,
        resources: HashMap<(PackResourceKind, ResourceId), Vec<u8>>,
    }

    impl PackSource for FakePack {
        fn pack_version(&self) -> String {
            self.version.clone()
        }

        fn list(&self, kind: PackResourceKind) -> Vec<ResourceId> {
            let mut ids: Vec<ResourceId> = self
                .resources
                .keys()
                .filter(|(k, _)| *k == kind)
                .map(|(_, id)| id.clone())
                .collect();
            ids.sort();
            ids
        }

        fn read(&self, kind: PackResourceKind, id: &ResourceId) -> Result<Vec<u8>, PackError> {
            self.resources
                .get(&(kind, id.clone()))
                .cloned()
                .ok_or_else(|| PackError::NotFound {
                    kind,
                    id: id.clone(),
                })
        }
    }

    impl FakePack {
        fn new(version: &str, steps: &[(&str, &str)]) -> Self {
            let mut resources = HashMap::new();
            for (id, body) in steps {
                resources.insert(
                    (PackResourceKind::Steps, ResourceId::from(*id)),
                    body.as_bytes().to_vec(),
                );
            }
            FakePack {
                version: version.to_owned(),
                resources,
            }
        }
    }

    /// A `replace` delta over `workflow:single-task#<step-id>`.
    fn replace(workflow: &str, step: &str, with: &str) -> StructuralDelta {
        StructuralDelta::Replace {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::At(step.to_owned()),
            },
            step: with.to_owned(),
        }
    }

    /// A `tracked-fork` delta over `workflow:single-task#<step-id>`.
    fn fork(workflow: &str, step: &str, base_hash: &str) -> TrackedForkDelta {
        TrackedForkDelta {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::At(step.to_owned()),
            },
            base_version: "v1".to_owned(),
            base_hash: base_hash.to_owned(),
        }
    }

    /// The v2 pack **omits** the `locate` step both a `replace` and a `tracked-fork`
    /// delta target → each classifies `orphaned`: a blocking [`Finding::block`]
    /// carrying the delta's target string `workflow:single-task#locate` and a
    /// remove/re-target route. Question 1, the existence question.
    #[test]
    fn target_absent_in_v2_classifies_orphaned() {
        // v1 had `locate` + `implement`; v2 dropped `locate`.
        let v2 = FakePack::new("v2", &[("implement", "implement body v2\n")]);

        let structural = vec![replace("single-task", "locate", "find")];
        let forks = vec![fork("single-task", "locate", "abc123")];
        let bases = Vec::new();
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        assert_eq!(findings.len(), 2, "both deltas on the dropped step orphan");
        for finding in &findings {
            assert_eq!(finding.severity, Severity::Blocking);
            assert_eq!(finding.code, "override-default.target-exists");
            assert!(
                finding.message.contains("workflow:single-task#locate"),
                "finding identifies the delta by its target string: {}",
                finding.message
            );
            let route = finding.route.as_deref().expect("orphaned carries a route");
            assert!(
                route.contains("remove") || route.contains("re-target"),
                "route directs remove/re-target: {route}"
            );
            assert!(
                route.contains("workflow:single-task#locate"),
                "route names the target string: {route}"
            );
        }
    }

    /// A delta whose target the v2 pack **still carries** yields **no** orphaned
    /// finding — and T1 makes no content compare, so a present target raises nothing
    /// at all (no spurious `conflict`). The negative half of question 1.
    #[test]
    fn target_present_in_v2_yields_no_finding() {
        let v2 = FakePack::new(
            "v2",
            &[
                ("locate", "locate body v2\n"),
                ("implement", "implement body v2\n"),
            ],
        );

        let structural = vec![replace("single-task", "locate", "find")];
        let forks = vec![fork("single-task", "implement", "abc123")];
        let bases = Vec::new();
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        assert!(
            findings.is_empty(),
            "present targets raise no orphaned finding and no spurious conflict: {findings:?}"
        );
    }

    /// An `insert` delta is not content-bearing — it depends on anchor existence (a
    /// later task's question), so the existence read does not apply and it never
    /// orphans here, even when the v2 pack omits its anchor step.
    #[test]
    fn insert_delta_is_not_subject_to_the_content_existence_question() {
        let v2 = FakePack::new("v2", &[("implement", "implement body v2\n")]);

        let structural = vec![StructuralDelta::Insert {
            target: StructuralTarget {
                workflow_id: "single-task".to_owned(),
                anchor: Anchor::After("locate".to_owned()),
            },
            step: "extra".to_owned(),
        }];
        let forks = Vec::new();
        let bases = Vec::new();
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        assert!(
            classify(deltas, &v2).is_empty(),
            "insert is not the content-existence question T1 answers"
        );
    }
}
