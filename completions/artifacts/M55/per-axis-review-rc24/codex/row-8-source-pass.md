<!-- The unseeded Codex source pass for ROW 8 (composed surfaces), verbatim below this line. Codex CLI `codex-cli 0.146.0`, source read at the published `jigc 1.0.0-rc.24`, 2026-10-03; exit code 0. It drove nothing. -->

# Row 8 · composed surfaces — source pass

Reviewed `jigc-v1.0.0-rc.24` source only; no binary drives and no writes. The checkout’s two later commits do not change production source.

## Claims

1. **A project-layer step can name the refused per-task finalize door literally and survive sub-task composition because the omission derives only catalog references, while the supplementary prose fence scans shipped packs only.**

   - Evidence: `sub_task_omission_set` classifies finalize doors only through `command_ref_ids_in` followed by catalog lookup; literal commands or equivalent prose do not affect membership ([start.rs:3235](crates/cli/src/start.rs:3235), [start.rs:3241](crates/cli/src/start.rs:3241)). Project shadows are deliberately read through the live `CascadeStepSource` ([start.rs:3052](crates/cli/src/start.rs:3052)), but the only literal-door fence enumerates the two on-disk shipped packs and explicitly declares project-layer shadows outside its coverage ([sub_task_composition.rs:1653](crates/cli/tests/sub_task_composition.rs:1653), [sub_task_composition.rs:1660](crates/cli/tests/sub_task_composition.rs:1660), [sub_task_composition.rs:1673](crates/cli/tests/sub_task_composition.rs:1673)). The existing project-shadow test covers only adding `{{ cli.finalize-task }}` ([sub_task_composition.rs:557](crates/cli/tests/sub_task_composition.rs:557)).
   - Proposed reproduction: create a milestone sub-task using `park-idea`; run `jigc config fork workflow:park-idea#author-idea`; append prose such as `Run jigc task finalize <the task id>` without a catalog ref; provision, then execute its emitted `Spawn:` command. Expected wrong behavior: the literal per-task instruction remains in the composed sub-task text; following it exits 3 with `finalize.milestone-sub-task`, despite the trailer saying the milestone is its only commit boundary.
   - Confidence: **high from source**. This is a completeness/usability gap, not a demonstrated tier-1 exit-0 loss.

No source-grounded tier-1 finding was found inside the M55 code.

## rc.19 baseline dispositions

- **`(6, D-1)` — STILL OPEN (1.x expected).** Orientation collects task content findings but has no repository-posture sweep ([orient.rs:202](crates/cli/src/orient.rs:202)); `task validate` still owns that posture check. M54/M55 did not close the mismatch.
- **`(6, D-2)` — STILL OPEN (1.x expected).** `fix-task` still combines its never-finalize discipline with an ordinary commit-author/finalize path; M55’s omission applies only when `owning_milestone` is present ([start.rs:3111](crates/cli/src/start.rs:3111), [start.rs:3124](crates/cli/src/start.rs:3124)).
- **`(6, D-3)` — STILL OPEN (1.x expected).** Orientation still says every mint-nothing workflow composes through `start --workflow`, although verb-routed ones refuse ([render.rs:305](crates/cli/src/render.rs:305)); the corrected `describe` wording remains separate ([render.rs:6298](crates/cli/src/render.rs:6298)).
- **`(6, D-4)` — STILL OPEN (1.x expected).** Empty fan-out still intentionally emits zero `Spawn:` lines ([compose.rs:9204](crates/engine/src/compose.rs:9204)); no empty-case narration was added, while finalize retains `milestone.zero-contribution` ([milestone.rs:6249](crates/cli/src/milestone.rs:6249)).

Open leads:

- **`(6, L-1)` — STILL OPEN.** The non-UTF-8 fallback still drops the absolute worktree and constructs `SubTask::new` ([milestone.rs:2708](crates/cli/src/milestone.rs:2708), [milestone.rs:2715](crates/cli/src/milestone.rs:2715)).
- **`(6, L-2)` — STILL OPEN, declared bound.** Suppression completeness remains manifest-scoped; M55’s three hidden workflows do carry the required reason and `expires: never`.
- **`(6, L-3)` — STILL OPEN.** Absolute `cd … && jigc workflow …` remains a distinct composed route-span; spawn generation still owns that form ([compose.rs:1719](crates/engine/src/compose.rs:1719)).
- **`(6, L-4)` — STILL OPEN, declared convention.** Committed-store composition still binds to `jigc_home`, not necessarily the reader’s checkout ([start.rs:1843](crates/cli/src/start.rs:1843)).
- **`(6, L-5)` — STILL OPEN as its three declared bounds.** Nothing in M54/M55 changes the pinned absolute `findings_unavailable` value, migrate-over-open-work behavior, or code-less malformed-address refusal.
- **`(6, L-6)` — STILL OPEN / not re-driven.** Nothing in this range changes `JIGC_PACK_DIR` or the `pack-resource-missing` seam.
- **`(6, L-7)` — STILL OPEN / not driven.** Project packs listed through `packs.yaml` remain outside the baseline binary-built fixture coverage.

## Consistent coverage

- All five producer families converge on `render::composed`: start/workflow arms ([cli.rs:1763](crates/cli/src/cli.rs:1763)), migrate ([migrate.rs:76](crates/cli/src/migrate.rs:76)), milestone execute ([milestone.rs:4952](crates/cli/src/milestone.rs:4952)), and task amend ([task.rs:624](crates/cli/src/task.rs:624)). No producer bypass was found.
- Fresh, minted, resumed, and finalize-time workflow reads all apply project structural deltas through `apply_project_structure` ([start.rs:2013](crates/cli/src/start.rs:2013)).
- The omission walks the resolved include tree and closes finalize-door membership under inclusion ([start.rs:3228](crates/cli/src/start.rs:3228)); commit-doc authors are omitted only under squash ([start.rs:3245](crates/cli/src/start.rs:3245)).
- Compose and join share the sole squash reader ([milestone.rs:6364](crates/cli/src/milestone.rs:6364)).
- The sub-task trailer correctly names re-entry and the milestone-only boundary ([render.rs:529](crates/cli/src/render.rs:529)).
- `report-inconsistency` is selectable with a `when:` hint; the other three findings workflows are hidden, have no door, remain name-composable, and expose their suppression reasons. All four end in `finalize-doc-only`.
- No stale command or flag was found in either current step tree.
- The declared squash=false docs-only and `task validate <sub>` bounds remain open as documented.

## Schema boundary and bounds

The boundary is intact: M55 adds exactly `inconsistency` and `jigc-feedback`, both schema-version 1 ([schema-manifest.yaml:125](crates/cli/packs/methodology/config/schema-manifest.yaml:125)); no other schema hash, schema version, or pinned `--format json` key moved. The co-author trailer is applied at the git-message seam and does not alter the commit doctype ([assistant-adapter.md:157](design/assistant-adapter.md:157)).

This was a source-completeness pass only; runtime prose equivalence and the declared binary-only cells belong to the independent driver.