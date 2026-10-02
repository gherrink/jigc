# M55 robust-advocates — the cases as argued (2026-10-02)

Two independent `robust-advocate` subagents argued the vision-robust side of two Settle forks before the human decided. Their full outputs lived in the planning session; this file preserves each case's substance so the gate-record's citation (O4) rests on a committed record.

## 1 · Fork S1 — the findings doctype's cardinality

**Advocated: D, one document per reporting batch.** Spiked against the rc.22 debug binary with a scratch `finding-batch` doctype + `report-finding` workflow; no new engine mechanism.
- **Against A (singleton ledger):** fails at measured doors on the committed trajectory — fan-out `join.same-doc-clash` (recovery discards a sub-task's entry), parallel branches (the PR #6/#5 datum; a union resolve silently dropped an item field while `validate` exited 0), L1's false conflict-block after every pull, and M57 triage colliding with M56 reports. No singleton→per-doc transform exists, so A is a one-way door.
- **Against B (doc per finding):** right concurrency, wrong unit — one created instance per task (`write.identity-change`), so an auditor with five findings needs five tasks; seeding costs one task per row; the exact-title silent overwrite (S1/F2) is worst there; reads cost list + N shows.
- **D's spike:** one task, three findings, one commit; two parallel branches (a new batch + a status edit on a committed batch) merged with no conflict and `validate` clean; a title collision degraded to a misfile, never a loss.
- **Declared costs of D:** reads (list + one show per batch); items lose identity in managed refs (as A); append-only is item-level convention.

**The human's decision:** B — D holds many findings in one file, against *one file, one purpose*; B reduces collisions and lets a finding be referenced by its own file (settle-log S1).

## 2 · Fork S4 — what "record-only commit" may mean

**Advocated: (b), a guard keyed on a composed step, inside `crates/cli` — no engine change.** Re-ran the mid-task spike's state 3: the report task's finalize exited 0 and committed the code task's `README.md` + `greet.sh` under `docs(ideas): …`, no advisory.
- **Against (a) convention-only:** two contradicting instructions in one agent's context (`dev-task` says stage as you work; the report says don't), no signal on violation; the wrong commit lands in pushed history agents may not force-push, release-plz misreads a `feat` inside a `docs` commit, the code task's message is lost.
- **(d) the general fix is unavailable:** finalize cannot tell which open task staged a path (shared index, no per-task path claims — new mechanism); `decide_carryover` keys on a mint-time snapshot, so state 3 is invisible to `carried-staged`.
- **Proposed shape:** a new pack step `step:finalize-doc-only`; a CLI constant + guard keyed on the composed body including it (the `composes_review_hold` / `MIGRATION_FINALIZE_STEP` precedent); body mirroring `amend_index_findings`; a new blocking finding with a `git restore --staged` route; vocabulary "the doc-only finalize step", never "record-only".

**The human's decision:** (b) (settle-log S4) — **revised at the design review (R1):** the refusal's route un-staged the code task's work (which then failed `finalize.nothing-staged`, driven by the design-reviewer), so the step commits **path-scoped** instead (`git commit -- <doc>`, the milestone-record door mechanism) with no refusal.
