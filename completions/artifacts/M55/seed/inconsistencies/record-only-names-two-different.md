---
kind: doc-doc
status: resolved
date: 2026-10-03
schema-version: 1
---

# Record-only names two different commits

## Sides

### design/finalize.md:218  {#side1}

The rollback table's row for *the five record-only doors*: the commits the milestone record's doors land, enumerated by `cli::rollback::ROLLBACK_POPULATIONS`' `milestone-record` row.

### design/team-ready-state.md:73  {#side2}

`create` and `add-task` each land *a separate record-only path-scoped commit* of the milestone record.

### design/validation.md:753  {#side3}

*Every record-only door writes the committed `milestone-record`, `git add`s it and commits only it.*

### crates/cli/tests/count_fences.rs:901  {#side4}

`RECORD_DOOR_HOMES`: a home that says *<N> record-only doors* must state the registry's door count.

### implementation/decisions-pending.md:105  {#side5}

The port paragraph: a `report-finding` workflow appends one finding *and lands it as a record-only commit*. That is a report task's commit, not a milestone-record door's.

## Description

One term, two meanings. *Record-only* names the commit a milestone-record door lands. There are five such doors, enumerated by a registry and count-fenced over three design docs. The port paragraph in decisions-pending used the same words for a report task's commit, which no milestone-record door lands. The decisions-pending row *What record-only commit is allowed to mean* asked the M55 Settle to say which reading it took. The planning register recorded the clash.

## Evidence

Read at planning: the four record-door homes against the port paragraph, then at `decisions-pending.md:104`.

Re-read at `a7a742d3`: `git grep -n -i record-only` over `design/`, `implementation/` and `crates/` returns 213 lines, with the roadmap and the crate changelog left out. Every use names a milestone-record commit, except two. The port sentence at `decisions-pending.md:105` uses it for a report. `findings-channel.md:181` uses it only to refuse it.

## Resolution

Settled at the M55 Settle (S4, revised by R1; `9fd644b7`). The report task's commit is a new thing with its own name, `step:finalize-doc-only`, *the doc-only finalize step*, and *record-only* stays with the five milestone doors. `findings-channel.md` → 3, *The name*, says so. The decisions-pending fork row graduated in the same words. The port sentence is kept as recorded under a `[Settled differently 2026-10-02 …]` bracket that names the doc-only finalize, so the corrected side is decisions-pending, by annotation. UNPINNED: nothing pins the term's reach. `count_fences::every_record_only_door_home_states_the_registrys_door_count` pins only the door count where the term is used.

Re-driven at `a7a742d3`: the grep in `evidence` holds, and the port sentence carries the bracket.
