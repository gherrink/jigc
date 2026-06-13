# M17 Pilot — Comparison results (flow 22)

Objective facts per arm, captured as each completes. The **verdict** (rubric scores + prose +
`green|red`) is the human judge's after all three arms run — this file holds the evidence, not the
ruling. Acceptance A1–A6 and the rubric are in `pre-registration.md`. Arm order: **B → A → C**.

## Arm B — control = GSD  *(run 1, 2026-06-13)*

- **Environment:** `~/.claude` (GSD active, mem off), galey untouched (full GSD `CLAUDE.md`).
- **Approach:** GSD "fast" skill (`/gsd-quick`-style) — ran the task in one shot, minimal changes;
  typecheck + tests executed by the agent.
- **Wall-clock:** < 5 min. **Transcript:** `logs/arm-B/transcript.jsonl` (61 lines).
- **Commit:** `1c47bcc` — `fix: honor maxLevel in heading aria.keyboard and themeKeys`.
- **Diff:** 1 file (`packages/extension-heading/src/index.ts`), +9 / −6. `Array.from({length:maxLevel})`
  for both `aria.keyboard` and `themeKeys` — the clean, idiomatic fix.
- **file_op capture:** 1 event (`Edit` on the task file). Zero touches outside scope.

**Correctness — A1–A6: PASS (all).** Objectively verified (throwaway acceptance test, 5/5):
A1 `maxLevel:6`→6 `aria.keyboard` entries · A2 →6 `themeKeys` · A3 default→3 each · A4 `maxLevel:1`→1 ·
A5 `pnpm test` green (existing suite) · A6 descriptions well-formed (`set heading level N`).

**Evidence for the rubric (not scored here):**
- *Commit/doc quality:* clear, correctly-scoped Conventional Commit. (good)
- *Drift:* none — one file, one edit, exactly the task.
- *Ceremony:* very low — one-shot via the GSD fast skill, < 5 min.
- *Notable:* **added no test** for the new `maxLevel` behaviour — the fix is correct but the
  regression is uncovered (no test-first; the GSD fast path optimizes for speed, not test-first
  discipline). A signal worth weighing against arm A/C, which the methodology pushes toward a test.

## Arm A — jigc  *(pending)*

## Arm C — static-methodology  *(pending)*
