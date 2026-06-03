//! The `override-default` classifier — maps each recorded delta against a given
//! [`PackSource`] to one of **clean / conflict / orphaned / needs-rebasing**.
//!
//! This is M5's core logic, run by the bespoke upgrade-time path (`jigc upgrade`),
//! not the per-task `validate_task` seam ([validation.md](../../../design/validation.md)
//! → `override-default`). When pack-default goes `v1 → v2`, the classifier re-applies
//! each recorded delta and asks up to three deterministic questions against v2
//! ([overrides.md](../../../design/overrides.md) → Upgrade reconciliation):
//!
//! 1. **Does the target still exist?** No → **`orphaned`**.
//! 2. A content-bearing `replace`/`remove` with no recorded base-hash (an M4-era
//!    manifest) → **`needs-rebasing`** — distinct from `clean`, never re-derived from
//!    the current pack.
//! 3. The target's content changed upstream → **`conflict`**, else **`clean`**.
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
/// Per content-bearing delta (each `replace` / `remove` [`StructuralDelta`] and each
/// [`TrackedForkDelta`]), against v2:
///
/// 1. **Existence (→ `orphaned`).** The target's step id (`#<step-id>`, an
///    [`Anchor::At`]) must still resolve: a pack-direct `pack.read(Steps, <step-id>)`
///    that is `Ok` means the target exists; an `Err(NotFound)` orphans it (one blocking
///    `orphaned` [`Finding`] with the delta's target string + a remove/re-target route).
/// 2. **No basis (→ `needs-rebasing`).** A still-present `replace`/`remove` with **no**
///    recorded [`StructuralBasis`] (an M4-era manifest) raises one blocking
///    `needs-rebasing` [`Finding`] routed to re-record — distinct from `clean` and never
///    re-derived from the current pack. (Asked before the content compare.)
/// 3. **Content (→ `conflict` / `clean`).** For a still-present target that carries
///    a recorded `base_hash` (a fork's `base_hash`, or a `replace`/`remove` with a
///    matching [`StructuralBasis`]), re-read the pack unit and compare: equal → `clean`
///    (no finding); differs → one blocking `conflict` [`Finding`] with a keep /
///    re-target / drop route.
///
/// Every read is **pack-direct** (`PackSource::read`), never the cascade-resolved owner
/// — the B3 shadow-bypass the fork case depends on: a shadow-aware read would compare a
/// fork to its own copy and falsely say `clean` (`overrides.md` → The probe re-reads the
/// pack-default unit).
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
            continue; // an absent target is orphaned; no content compare follows.
        }
        // The target exists. Question 2 (asked before the content compare): a
        // content-bearing `replace`/`remove` with **no** recorded basis is an
        // M4-era manifest → `needs-rebasing`, distinct from `clean` and never
        // silently re-derived from the v2 pack (`overrides.md` → Backward-compat).
        match recorded_basis(target, deltas.bases) {
            None => findings.push(needs_rebasing(target)),
            // A recorded basis drives the content compare: equal → clean, differs → conflict.
            Some(recorded) => {
                if let Some(finding) = conflict_if_changed(target, recorded, pack) {
                    findings.push(finding);
                }
            }
        }
    }

    for fork in deltas.forks {
        if let Some(finding) = orphaned_if_absent(&fork.target, pack) {
            findings.push(finding);
            continue;
        }
        // A fork always carries its recorded `base_hash` (M4), so the content
        // compare always applies. The read is pack-direct (`orphaned_if_absent`
        // / `conflict_if_changed` both go through `pack.read`), bypassing the
        // fork's own shadow — the B3 shadow-bypass headline.
        if let Some(finding) = conflict_if_changed(&fork.target, &fork.base_hash, pack) {
            findings.push(finding);
        }
    }

    findings
}

/// The recorded [`StructuralBasis::base_hash`] for a `replace`/`remove` target, if one
/// was recorded — keyed by the delta's [`StructuralTarget`] (a delta has no stable id;
/// its target is its identity — `overrides.md` → On-disk vs in-memory representation).
/// `None` is the basis-less legacy case — the `needs-rebasing` branch in [`classify`].
fn recorded_basis<'a>(target: &StructuralTarget, bases: &'a [StructuralBasis]) -> Option<&'a str> {
    bases
        .iter()
        .find(|b| &b.target == target)
        .map(|b| b.base_hash.as_str())
}

/// Question 3 (`overrides.md` → Upgrade reconciliation): for a content-bearing target
/// that *exists* and carries a recorded `base_hash`, re-read the **pack-default** unit
/// and compare. Equal → clean (`None`); differs → one blocking `conflict` [`Finding`].
///
/// The read is **pack-direct** — `pack.read(Steps, <step-id>)`, never the
/// cascade-resolved owner — so a `tracked-fork` that shadows the same id is compared
/// against the pack's bytes, not its own copy. A shadow-aware read would return the
/// fork's copy (always == `base_hash`) and falsely say `clean` (the B3 shadow-bypass —
/// `overrides.md` → The probe re-reads the pack-default unit).
fn conflict_if_changed(
    target: &StructuralTarget,
    base_hash: &str,
    pack: &dyn PackSource,
) -> Option<Finding> {
    let step_id = at_step_id(target);
    // The existence question already ran for this target, so the read resolves; a
    // racing `NotFound` is the orphaned case, not a content change.
    let bytes = pack
        .read(PackResourceKind::Steps, &ResourceId::from(step_id))
        .ok()?;
    if crate::file_state::hash_bytes(&bytes) == base_hash {
        None // unchanged upstream → clean, you inherit the v2 pack free.
    } else {
        Some(conflict(target))
    }
}

/// One blocking `conflict` [`Finding`] for a content-bearing delta whose target changed
/// upstream, identified by its target string and routed to the keep / re-target / drop
/// review the human acts on (`overrides.md` → conflict → blocks with a review route).
fn conflict(target: &StructuralTarget) -> Finding {
    let target_str = render_target(target);
    Finding::block(
        "override-default.content-changed",
        format!("override target `{target_str}` changed in the current pack since it was recorded"),
        format!(
            "review the change on `{target_str}`: keep your override, re-target it, or drop it"
        ),
    )
}

/// One blocking `needs-rebasing` [`Finding`] (`overrides.md` → Upgrade reconciliation,
/// question 2) for a content-bearing `replace`/`remove` whose target *exists* but
/// carries **no** recorded [`StructuralBasis`] (an M4-era manifest). Routed to re-record
/// the delta — which pins a basis against the now-current pack — and **distinct from
/// `clean`**: a missing basis is never re-derived from the v2 pack (that would compare
/// equal and silently mask every conflict — `overrides.md` → Backward-compat).
fn needs_rebasing(target: &StructuralTarget) -> Finding {
    let target_str = render_target(target);
    Finding::block(
        "override-default.needs-rebasing",
        format!(
            "override target `{target_str}` has no recorded base-hash (an older manifest); its basis cannot be compared"
        ),
        format!(
            "re-record the delta on `{target_str}` (e.g. `jigc config replace-step …`) to pin a basis against the current pack"
        ),
    )
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

    /// A `remove` delta over `workflow:single-task#<step-id>`.
    fn remove(workflow: &str, step: &str) -> StructuralDelta {
        StructuralDelta::Remove {
            target: StructuralTarget {
                workflow_id: workflow.to_owned(),
                anchor: Anchor::At(step.to_owned()),
            },
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

    /// A recorded [`StructuralBasis`] for a `replace`/`remove` delta over
    /// `workflow:single-task#<step-id>` — the M5 base-hash basis the content
    /// compare reads.
    fn basis(workflow: &str, step: &str, base_hash: &str) -> StructuralBasis {
        StructuralBasis {
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

    /// A delta whose target the v2 pack **still carries** *unchanged* raises **no**
    /// finding (the negative half of question 1). The `replace` carries a basis that
    /// matches the present `locate` bytes, and the fork carries a basis that matches
    /// the present `implement` bytes, so the content compare says clean — a present,
    /// unchanged target raises nothing at all (no spurious `orphaned`/`conflict`).
    #[test]
    fn target_present_in_v2_yields_no_finding() {
        let locate_v2 = "locate body v2\n";
        let implement_v2 = "implement body v2\n";
        let v2 = FakePack::new("v2", &[("locate", locate_v2), ("implement", implement_v2)]);

        let structural = vec![replace("single-task", "locate", "find")];
        let forks = vec![fork(
            "single-task",
            "implement",
            &crate::file_state::hash_bytes(implement_v2.as_bytes()),
        )];
        let bases = vec![basis(
            "single-task",
            "locate",
            &crate::file_state::hash_bytes(locate_v2.as_bytes()),
        )];
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

    /// **Clean** — a content-bearing delta whose target the v2 pack still carries
    /// *unchanged* (the v2 step bytes hash equal to the recorded basis) raises **no**
    /// finding, for each content-bearing kind: a `replace`/`remove` with a matching
    /// [`StructuralBasis`], and a `tracked-fork` with a matching `base_hash`. The
    /// override inherits the v2 pack free (`overrides.md` → question 3: clean).
    #[test]
    fn unchanged_unit_classifies_clean_for_each_content_bearing_kind() {
        // v1 and v2 carry byte-identical step bodies for the overridden targets.
        let locate_v1 = "locate body v1\n";
        let implement_v1 = "implement body v1\n";
        let validate_v1 = "validate body v1\n";
        let v2 = FakePack::new(
            "v2",
            &[
                ("locate", locate_v1),
                ("implement", implement_v1),
                ("validate", validate_v1),
            ],
        );

        // The recorded basis is the blake3 of the v1 (== v2) pack-default bytes.
        let structural = vec![
            replace("single-task", "locate", "find"),
            remove("single-task", "implement"),
        ];
        let forks = vec![fork(
            "single-task",
            "validate",
            &crate::file_state::hash_bytes(validate_v1.as_bytes()),
        )];
        let bases = vec![
            basis(
                "single-task",
                "locate",
                &crate::file_state::hash_bytes(locate_v1.as_bytes()),
            ),
            basis(
                "single-task",
                "implement",
                &crate::file_state::hash_bytes(implement_v1.as_bytes()),
            ),
        ];
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        assert!(
            findings.is_empty(),
            "an unchanged unit (v2 bytes == recorded basis) is clean — no finding: {findings:?}"
        );
    }

    /// **Conflict** — a `replace` and a `remove` whose target *content changed*
    /// upstream (the v2 step bytes hash differs from the recorded basis) each
    /// classify `conflict`: a blocking [`Finding`] identified by the delta's target
    /// string and carrying a keep / re-target / drop review route
    /// (`overrides.md` → question 3: conflict → blocks with a review route).
    #[test]
    fn changed_replace_and_remove_classify_conflict() {
        // v1 basis recorded against these bytes; v2 ships *different* bytes.
        let locate_v1 = "locate body v1\n";
        let implement_v1 = "implement body v1\n";
        let v2 = FakePack::new(
            "v2",
            &[
                ("locate", "locate body v2 — changed\n"),
                ("implement", "implement body v2 — changed\n"),
            ],
        );

        let structural = vec![
            replace("single-task", "locate", "find"),
            remove("single-task", "implement"),
        ];
        let forks = Vec::new();
        let bases = vec![
            basis(
                "single-task",
                "locate",
                &crate::file_state::hash_bytes(locate_v1.as_bytes()),
            ),
            basis(
                "single-task",
                "implement",
                &crate::file_state::hash_bytes(implement_v1.as_bytes()),
            ),
        ];
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        assert_eq!(findings.len(), 2, "both changed targets conflict");
        for (finding, step) in findings.iter().zip(["locate", "implement"]) {
            assert_eq!(finding.severity, Severity::Blocking);
            assert_eq!(finding.code, "override-default.content-changed");
            let target_str = format!("workflow:single-task#{step}");
            assert!(
                finding.message.contains(&target_str),
                "conflict identifies the delta by its target string: {}",
                finding.message
            );
            let route = finding.route.as_deref().expect("conflict carries a route");
            assert!(
                route.contains("keep") && route.contains("re-target") && route.contains("drop"),
                "the conflict route offers keep / re-target / drop: {route}"
            );
            assert!(
                route.contains(&target_str),
                "the route names the target string: {route}"
            );
        }
    }

    /// **The headline — the fork shadow-bypass (B3).** A `tracked-fork` *shadows the
    /// same step id*: the project's forked copy lives at `steps/<id>.yaml`, so a
    /// shadow-aware re-resolution would read the fork's **own** bytes (always == the
    /// recorded `base_hash`) and falsely say `clean`. The probe instead re-reads the
    /// **pack-default** unit via [`PackSource::read`]; when those v2 pack bytes differ
    /// from the recorded basis, the fork classifies **`conflict`** — a blocking
    /// [`Finding`] with a keep / re-target / drop route. The `FakePack` carries *only*
    /// the pack bytes, so a passing conflict proves the compare hit the pack unit, not
    /// the shadow (`overrides.md` → The probe re-reads the pack-default unit).
    #[test]
    fn changed_fork_classifies_conflict_via_pack_direct_read() {
        // The recorded basis: the v1 pack-default bytes the fork was taken from.
        let validate_v1 = "validate body v1\n";
        let recorded = crate::file_state::hash_bytes(validate_v1.as_bytes());

        // v2's *pack-default* `validate` step changed upstream. A naive shadow-aware
        // read would return the fork's own (recorded-hash) copy and say `clean`; the
        // pack-direct read sees the changed pack bytes and conflicts.
        let v2 = FakePack::new("v2", &[("validate", "validate body v2 — changed\n")]);

        let structural = Vec::new();
        let forks = vec![fork("single-task", "validate", &recorded)];
        let bases = Vec::new();
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        assert_eq!(findings.len(), 1, "the changed fork conflicts");
        let finding = &findings[0];
        assert_eq!(finding.severity, Severity::Blocking);
        assert_eq!(finding.code, "override-default.content-changed");
        assert!(
            finding.message.contains("workflow:single-task#validate"),
            "the fork conflict identifies its target string: {}",
            finding.message
        );
        let route = finding
            .route
            .as_deref()
            .expect("fork conflict carries a route");
        assert!(
            route.contains("keep") && route.contains("re-target") && route.contains("drop"),
            "the fork conflict route offers keep / re-target / drop: {route}"
        );
    }

    /// **`needs-rebasing` is distinct from `clean`.** Two `replace`/`remove` deltas
    /// over *present* targets carrying byte-identical v2 bytes — one with a recorded
    /// [`StructuralBasis`] equal to the present bytes, one with **no** basis (an
    /// M4-era manifest). The basis-present-and-equal pair is `clean` (no finding); the
    /// basis-less pair is **`needs-rebasing`** — a blocking [`Finding`] routed to
    /// re-record. The two are **not confused**: a present-and-equal basis is never a
    /// missing one (`overrides.md` → Backward-compat).
    #[test]
    fn basis_less_replace_remove_classify_needs_rebasing_distinct_from_clean() {
        // Both the basis-bearing and the basis-less targets carry identical v2 bytes;
        // only the *presence of a recorded basis* distinguishes clean from needs-rebasing.
        let locate_v2 = "locate body v2\n";
        let implement_v2 = "implement body v2\n";
        let extra_v2 = "extra body v2\n";
        let probe_v2 = "probe body v2\n";
        let v2 = FakePack::new(
            "v2",
            &[
                ("locate", locate_v2),
                ("implement", implement_v2),
                ("extra", extra_v2),
                ("probe", probe_v2),
            ],
        );

        // `locate` (replace) + `implement` (remove) have a basis EQUAL to the v2 bytes
        // → clean. `extra` (replace) + `probe` (remove) have NO basis → needs-rebasing.
        let structural = vec![
            replace("single-task", "locate", "find"),
            remove("single-task", "implement"),
            replace("single-task", "extra", "more"),
            remove("single-task", "probe"),
        ];
        let forks = Vec::new();
        let bases = vec![
            basis(
                "single-task",
                "locate",
                &crate::file_state::hash_bytes(locate_v2.as_bytes()),
            ),
            basis(
                "single-task",
                "implement",
                &crate::file_state::hash_bytes(implement_v2.as_bytes()),
            ),
        ];
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        // The two basis-bearing deltas are clean (silent); only the two basis-less
        // deltas surface — needs-rebasing is never silently re-derived from the v2 pack.
        assert_eq!(
            findings.len(),
            2,
            "only the basis-less deltas surface; the equal-basis pair stays clean: {findings:?}"
        );
        for (finding, step) in findings.iter().zip(["extra", "probe"]) {
            assert_eq!(finding.severity, Severity::Blocking);
            assert_eq!(finding.code, "override-default.needs-rebasing");
            let target_str = format!("workflow:single-task#{step}");
            assert!(
                finding.message.contains(&target_str),
                "needs-rebasing identifies the delta by its target string: {}",
                finding.message
            );
            let route = finding
                .route
                .as_deref()
                .expect("needs-rebasing carries a re-record route");
            assert!(
                route.contains("re-record"),
                "the route directs a re-record: {route}"
            );
            assert!(
                route.contains(&target_str),
                "the route names the target string: {route}"
            );
        }
    }

    /// **Validation hardening #5 — the omitting-context case.** A basis-less
    /// `replace`/`remove` (the needs-rebasing input) classified against a pack that
    /// **OMITS** the target must take the *absent-target* path — `orphaned`, the
    /// existence question — never a panic and never a false `clean`/`needs-rebasing`.
    /// Exercising the needs-rebasing branch only against a pack that *carries* the
    /// target would hide the case where the target is gone: existence is asked first,
    /// so a basis-less delta over a dropped step orphans, it does not reach the
    /// no-basis branch.
    #[test]
    fn basis_less_delta_over_omitted_target_classifies_orphaned_not_clean() {
        // v2 omits `locate` and `implement` entirely — only `implement2` survives.
        let v2 = FakePack::new("v2", &[("implement2", "implement2 body v2\n")]);

        // Basis-less replace + remove (would be needs-rebasing if present) over the
        // *dropped* steps.
        let structural = vec![
            replace("single-task", "locate", "find"),
            remove("single-task", "implement"),
        ];
        let forks = Vec::new();
        let bases = Vec::new();
        let deltas = RecordedDeltas {
            structural: &structural,
            forks: &forks,
            bases: &bases,
        };

        let findings = classify(deltas, &v2);

        assert_eq!(
            findings.len(),
            2,
            "an omitted target orphans even a basis-less delta — never a false clean: {findings:?}"
        );
        for finding in &findings {
            assert_eq!(finding.severity, Severity::Blocking);
            assert_eq!(
                finding.code, "override-default.target-exists",
                "the absent-target path wins over the no-basis branch (existence is asked first)"
            );
        }
    }
}
