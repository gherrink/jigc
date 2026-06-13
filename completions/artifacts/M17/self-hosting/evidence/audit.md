# Audit — decided-task increment (M17 self-hosting dogfood)

Independent review of the increment that added the `decided-task` workflow (commits
`56c9158` feat + `5c280c9` docs), conducted as the completion-workflow audit phase.

## Scope reviewed
- `packs/methodology/workflows/decided-task.yaml` — new workflow.
- `packs/methodology/steps/author-decision.yaml` — new step.
- `crates/cli/tests/methodology_pack_compose.rs` — new compose test.
- `crates/cli/tests/methodology_increment_off_router.rs` — extended catalog guard.
- `decisions-log/decisions-log.md` — the two promoted decisions.

## Verdict: green
The increment is sound and minimal. Test-first discipline held (compose test was
observed red — `no workflow decided-task` — then green). Full gate green (fmt ·
clippy -D · 77 suites · build). Scope honest: `decided-task` is `selectable:false`,
preserving the deliberate single-default catalog invariant, and the off-router guard
gained the matching `decided-task` negatives (catalog + JSON + enum). The grain gap
(decisions-pending.md:77) is closed: a sub-milestone task now has a lightweight managed
decision-recording path that does not invoke milestone-planning ceremony.

## Findings
1. **advisory / deferred** — The `decided-task` spine carries `implement` + `gate`
   steps even for a pure decision-recording task (the recording run itself made no code
   change, so those steps were vacuous). The workflow targets "one code change that also
   decides something"; a pure-decision task runs the dev steps as no-ops. Acceptable for
   the gap it closes; a dedicated decision-only spine is not earned by one occurrence.
   Evidence: the `record-the-decided-task-design-decisions-its` task ran scope/implement/
   gate with no source diff before `author-decision`.
