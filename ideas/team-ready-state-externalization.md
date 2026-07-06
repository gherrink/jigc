# Team-ready state externalization — shareable work state out of `.jigc`

**Status: parked 2026-07-06, unscheduled.** From RC greenfield trial 1, finding A6 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Work-unit state (milestones, task state) lives under `.jigc/` — fine for a single developer, but a teammate cloning the repo cannot see or continue the in-flight work. The trial's framing: pull the information others need (milestones, project state) out into the project so the work is *team-ready*. M38 solved this for *docs* (managed at legible homes: root `VISION.md`, `docs/*.md`); work-unit state is the remainder.

**Evidence (2026-07-06):** the human's own side experiment found that good structured meta documents make coding agents measurably more efficient when starting fresh sessions — the value of the hidden state is real, not hypothetical. And the trial log analysis measured the hidden set concretely: `.jigc/.gitignore` hides `tasks/`, `milestones/`, `index/`, `state/`, `logs/` — of which `index`/`state` are **re-derived** on a state-less clone (the baseline-adopt path proves it), so the genuinely-lost-on-clone set is **work-unit state: tasks and milestones** ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md) → Invocation-log analysis).

## The shape

Classify `.jigc/` contents along the share axis, not the format axis:

- **Shareable / committed** — the work-unit record a teammate continues from: milestones, task intent + status, the roadmap-shaped state. The methodology doctypes (M16: `roadmap`, `deferral-ledger`, …) already prove work-state-as-managed-doc; this idea is largely "route work-unit state through that proven surface" rather than a new mechanism.
- **Local / ignored** — derived caches (index), per-machine logs (`invocations.jsonl` already gitignored), transient working areas.

Distinct from (but adjacent to) the condition-keyed **per-developer `local` layer / team-layer distribution** deferral ([decisions-pending.md](../implementation/decisions-pending.md)) — that is *config* distribution; this is *state* legibility. Both fire on the same condition.

## Trigger

Multi-developer / team adoption is real — the same condition as the team-layer deferral; settle them together.
