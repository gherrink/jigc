# Coverage — every changed surface in exactly one column

[protocol.md](protocol.md) §6, applied to M46's diff (`1d2f146..33b3baa`).

**Three columns, never two.** M47's rule: a two-way split has no way to say *"fenced by a test,
not by the trial"*, which inflates the probe count and leaves changed lines unexamined. Each
surface lands in **trial-reached** · **test-fenced (naming the suite)** · **neither** — and the
*neither* column is empty or each member is explained.

**Derived from the code, not from the charter.** [pinning.md](../../../implementation/pinning.md)
§5: *a coverage classification is a claim about the code.* The changed-surface set comes from
`git diff --name-only` over the wave's own range; the fences come from `git diff --name-status`
over `crates/cli/tests`; and **every fence below was verified by reading the test functions it
contains**, never by its name looking apt. `reachable` is not `exercised`.

---

## The table

| # | Changed surface | Trial-reached | Test-fenced | Neither |
|---|---|---|---|---|
| 1 | the base-relative merge + save-scoped lock (`state.rs`, `file_state.rs`, `index.rs`) | — | `file_state_merge_hand_off.rs`, `milestone_task_list_concurrency.rs` | — *(needs concurrent processes; see below)* |
| 1 | its **observable consequence** — a lost baseline turning a conflict block into a silent merge | **walk 06 §3** | `reconciliation_baseline_contrast.rs::a_lost_file_state_baseline_turns_a_conflict_block_into_a_silent_merge` | — |
| 2 | the four destroying doors: 3 refusal cells + narration at all four | **walk 02** (25 bars) | `flow49_acceptance.rs` (iterates `DESTROYING_DOORS`) | — |
| 2 | the `--ignored` axis and its container-not-members rule | **walk 02** | `flow49_acceptance.rs` | — |
| 2/7 | the boundary's on-disk-path subject over own / `cp -R` / `mv` linkage | **walk 05** (12 bars) | `milestone_path_subject.rs` (4 cells, incl. the genuinely-absent path walk 05 does not drive) | — |
| 3 | `jigc validate`'s exit flip over a never-adopted file | **walk 04**; every blind adoption arm meets it | `record_foreign_arm.rs` | — |
| 3 | `migrate-corpus`'s foreign exclusion + the declared `unadopted[]` key | **walk 04** | `migrate_corpus_foreign.rs` (4 cells) | — |
| 4 | the transform's no-op fold, `set-field-unfilled`, `fold-refused` | — | `migrate_corpus_set_fields.rs`, `migrate_corpus_halt_causes.rs` | — *(schema-author-reachable only; see below)* |
| 5 | `conformance.item-heading-unanchored`'s in-message repair | **walk 06** (13 bars) | `pre_guard_repair_route.rs::no_carrier_of_the_diagnosis_names_a_verb_the_state_refuses` | — |
| 5 | `reconciliation.conflict-block`'s migration arm (`jigc unmanage <source>`) | — | `pre_guard_repair_route.rs` | — *(declared in walk 06's own output; see below)* |
| 6 | the changelog gate's write-touch predicate | **walk 07** | `changelog_write_touch.rs::every_doc_write_verb_reads_as_a_write_touch` | — |
| 6 | it joining `preview_gates` — the advisory at `jigc task validate`, both severities | **walk 07** (13 bars) | `validate_previews_the_gate.rs`, `gate_coverage_fence.rs` | — |
| 7 | `doc-code.criterion-maps-to-test`'s diagnosis + the file-only fallback | — | `maps_to_test_caveat_fence.rs`, `spec_read_back_arms.rs` | — *(see below)* |
| 8 | six of the seven surface repairs (T2–T7) | **walk 09** (17 bars) | `flow49_acceptance.rs`, `record_set_splice_retired.rs` | — |
| 8 | T1, the survivable hook-rejection frame on a docs-only finalize | **R2 rehearsal**; B1 blind arm | — | — |
| 9 | `doctype-map.md`'s stated schema-versions; three corrected ledger claims | — | `doctype_map_versions.rs`, `ledger_record_truth.rs`, `ledger_entry_seven_discharged.rs` | — *(carries no verb, finding or route)* |
| 10 | flow 49, the 612-golden regeneration, the project-state paragraph | — | `flow49_acceptance.rs`; the golden regen, whose **empty diff is the assertion** | — *(doc + test only)* |
| — | **the rc.11 → rc.12 upgrade path** | **walk 03 + 10** (7 + 20 bars) | **nothing** — every fixture builder drives the CURRENT binary | — |

---

## The *neither* column, member by member

It is not empty, and each member is here because it **cannot** be trial-reached, not because
nobody got to it.

**Increment 1's lock and merge.** Reachable only by concurrent `jigc` processes — nothing a
worker types drives it, and the wave itself reproduced the symptom with a 2000-round harness
(215 torn reads). Test-fenced. Walk 06 prints this as a DECLARED GAP in its own output rather
than leaving it to this table.

**Increment 4.** `migrate-corpus.set-field-unfilled` and `migrate-corpus.fold-refused` need a
**pack schema-version bump that adds a leaf**. Every driven cell in M46's own record used a
`JIGC_PACK_DIR` dev-pack copy, so this is **schema-author-reachable, not adopter-reachable**. A
walk arm could be written against a mutated pack; it is deliberately not, because the arm would
exercise a state no adopter of 1.0.0 can reach, and coverage bought that way flatters the table.

**Increment 5's conflict-block migration arm.** Needs a migration whose source is already a
**managed** doc carrying a baseline. Driven live, `jigc migrate <path> --as adr` over a *foreign*
file never engages the reconciliation guard — the task door reports
`schema-conformance.unadopted-instance` instead. Declared in walk 06's output, not faked.

**Increment 7.** `doc-code.criterion-maps-to-test`'s repaired diagnosis needs a spec whose
`maps-to-test` points at a closure-registered test. **B3's blind arc could reach it
opportunistically** — it writes a spec and implements it against a `node:test` corpus — but no
arm *designs* for it, so it is counted test-fenced and any blind reach is recorded as a bonus,
never as the reason the surface was covered.

**Increments 9 and 10.** These carry **no verb, no finding and no route**. There is nothing for a
done-picture walk to reach, and the wave's own decomposition says so in as many words. This is
the same honest disposition M48's manifest fence took.

---

## What this table does not yet contain

**The blind sessions have not run.** Three rows above name a blind arm as a *second* reacher
(Increment 3 on every adoption arm, Increment 6 on every gate-granting task, Increment 8's T1 on
B1). Those are predictions from the arm designs, not measurements, and they are marked as the
walk's rows are not: **the walk's cells are driven and counted; the blind cells are pending.**

This file is revised after the sessions, and a blind arm that fails to reach what its design
predicts is itself a finding about the arm — recorded, not quietly dropped.

## The count

- **trial-reached:** 11 surfaces, across 11 walk arms, all driven on `1.0.0-rc.12`
- **test-fenced:** 17 suites named, each verified by reading its test functions
- **neither:** 5 members, each explained above, and **three of them are declared inside the arms
  themselves** so an unreached cell is visibly blank in the walk record rather than absent from it
