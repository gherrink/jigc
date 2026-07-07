# Team-ready state externalization — shareable work state out of `.jigc`

**Status: ✅ shipped M39** (parked 2026-07-06; built as the `milestone-record` doctype under the `.jigc`-is-the-workbench principle — design of record in [design/team-ready-state.md](../design/team-ready-state.md)). The standalone single-task sibling stays parked in [task-record-graduation](task-record-graduation.md). From RC greenfield trial 1, finding A6 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Work-unit state (milestones, task state) lives under `.jigc/` — fine for a single developer, but a teammate cloning the repo cannot see or continue the in-flight work. The trial's framing: pull the information others need (milestones, project state) out into the project so the work is *team-ready*. M38 solved this for *docs* (managed at legible homes: root `VISION.md`, `docs/*.md`); work-unit state is the remainder.

**Evidence (2026-07-06):** the human's own side experiment found that good structured meta documents make coding agents measurably more efficient when starting fresh sessions — the value of the hidden state is real, not hypothetical. And the trial log analysis measured the hidden set concretely: `.jigc/.gitignore` hides `tasks/`, `milestones/`, `index/`, `state/`, `logs/` — of which `index`/`state` are **re-derived** on a state-less clone (the baseline-adopt path proves it), so the genuinely-lost-on-clone set is **work-unit state: tasks and milestones** ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md) → Invocation-log analysis).

## The shape — direction settled 2026-07-06 (human call): `.jigc/` is the workbench, the repo is the record

The classification axis is **working vs record**, and it assigns each side a *home*, not just a gitignore line:

- **The record lives OUTSIDE `.jigc/`**, at legible homes close to the docs that reference it — the M38 placement move, extended from docs to work-unit records. Milestones and the roadmap-shaped state a teammate continues from become project files (the methodology doctypes — M16 `roadmap`, `deferral-ledger`, … — already prove work-state-as-managed-doc; this is largely routing work-unit state through that proven surface, and it settles *where*: near the referencing docs, **not** inside `.jigc/`).
- **`.jigc/` is a pure working directory** — things currently being worked on that must not be committed, plus internal config: task working areas, derived caches (`index`/`state` re-derive on clone — proven by the baseline-adopt path), per-machine logs. Gitignored as a matter of *identity*, not convenience.
- **New workbench use — move-INTO for collisions:** when a working state or a colliding file would otherwise sit in the record's path (e.g. the changelog-migration in-location collision: a foreign file squatting on a migration destination), the CLI can **relocate it into the `.jigc/` work area** — gitignored, so a working state or the wrong file *cannot* be accidentally committed. This gives detect+route a new resolution arm ("parked in the workbench") and pairs naturally with the M39 freeze-exempt relocation floor.

Distinct from (but adjacent to) the condition-keyed **per-developer `local` layer / team-layer distribution** deferral ([decisions-pending.md](../implementation/decisions-pending.md)) — that is *config* distribution; this is *state* legibility. Both fire on the same condition.

## Trigger

**Scheduled: M39 candidate scope** ([decisions-pending.md](../implementation/decisions-pending.md) → M39) — pre-1.0 because the committed-vs-ignored layout split is a one-way door once external adopters exist. The full doctype-graduation depth is a fork for M39's Settle; the team-layer *config* sibling still waits for real multi-developer adoption.
