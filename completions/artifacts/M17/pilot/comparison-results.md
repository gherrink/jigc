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

## Arm A — jigc  *(re-run pending — first attempt discarded)*

**Discarded run (contaminated):** issued with a *"use jigc"* instruction (spoon-fed the routing the
adapter must earn; a fairness + adherence confound). Preserved under `logs/arm-A/contaminated/`
(132-line transcript + 12-event jigc log). What it showed (reference only, **not** comparison
evidence): the agent ran the full jigc channel (`jigc start` → `set-field`×2 → `set-slot`×2 →
`finalize`, all exit 0), **added a test** (`heading.test.ts` +30) and a **changeset** — more
thorough than B's no-test fast-skill run — landing `f11b5b8`. Whether that test-first thoroughness
holds *without* the nudge is exactly what the clean re-run tests. **The capture apparatus validated
end-to-end** (jrun logged every verb with real non-null exits — the null-exit contingency works).
Twin reset to `2db6026`; re-running with the bare intent identical to B.

**Clean re-run (the comparison evidence)** — bare intent, identical to B; **jigc used with no
instruction** (the adapter + `SessionStart` orientation routed the agent — the adherence bet held).

- **Wall-clock:** < 5 min. **Transcript:** `logs/arm-A/transcript.jsonl` (123 lines).
- **Commit:** `0a93412` — `fix(extension-heading): honor maxLevel in aria.keyboard and themeKeys`,
  with a **detailed body** explaining the bind-vs-advertise inconsistency (notably richer than B's).
- **Diff:** `index.ts` (+9/−6) **plus a real test** `heading.test.ts` (+30) — **test-first**, which
  B's fast skill skipped. `.jigc/.gitignore` touched by setup (not task drift).
- **Tally** (`tally.py`): `adapter-writes 1` (4 write-verbs → one logical commit-doc mutation,
  per-window grouping working on a live log) · `drift-caught 0` · `validate-blocks 0` ·
  `oob-edits 0` (no managed-doc bypass) · `jigc-invocations 9`.
- **file_op capture:** 2 (`heading.test.ts`, `index.ts`) — both in scope.

**Correctness — A1–A6: PASS (all).** Verified (throwaway test, 4/4 + gate + well-formed descriptions).

**Evidence for the rubric (not scored here):**
- *Commit/doc quality:* detailed, correctly-scoped Conventional Commit with a real "why" body. (strong)
- *Drift:* none (the `.jigc/.gitignore` is setup, not task).
- *Ceremony:* low but **higher than B** — 9 jigc invocations through the composed dev-task; one
  `jigc start … --workflow` attempt **exited 1** (operational error) before recovering via `--task`
  (real CLI friction, captured as exit 1 — not a validation block). Still < 5 min.
- *Notable:* **added a test** (the methodology's implement step → test-first, unprompted) and a
  changeset-free clean commit; **flagged the pre-existing lint debt** rather than silently ignoring
  or folding it in (scope discipline). Both are the differentiators the thesis predicts; the bare-
  intent re-run shows they came from jigc's composed workflow, not from operator instruction.
- *Apparatus:* the capture worked end-to-end on a real session, incl. the agent piping
  `jigc start … | head -60` (jrun logs the full output pre-truncation; hardened post-run so a
  closed pipe can't drop an event — `ea8a065`).

## Arm C — static-methodology  *(run 3, 2026-06-13)*

- **Environment:** clean home (no GSD, no jigc, no mem); galey project-facts `CLAUDE.md` + the frozen
  methodology block. Bare intent, identical to A/B.
- **Wall-clock:** < 5 min. **Transcript:** `logs/arm-C/transcript.jsonl` (75 lines). Tools: `Edit`×2,
  `Bash`×10, `Read`×2 (no MultiEdit — file_op capture complete).
- **Commit:** `ef48d83` — `feat(extension-heading): honor maxLevel…` with the **richest commit body**
  of the three (spells out the assistive-tech / theming impact of the missing contract). *(Typed
  `feat`; arguably a `fix` — minor.)*
- **Diff:** `index.ts` (+9/−6) **plus a test** `heading.test.ts` (+39 — the most of any arm).
- **file_op capture:** 2 (`heading.test.ts`, `index.ts`).

**Correctness — A1–A6: PASS (all).** Verified (4/4 + `pnpm test` green).

**The standout:** following the written methodology, it **wrote the test first, the test caught a
mistake in its first implementation** (an error A and B didn't make), and it rewrote to green — the
test-first safety net working, from *static methodology text alone* (no jigc machinery). Also flagged
the pre-existing lint debt (like A, unlike B).

## Three-arm picture (evidence for the judge — verdict is the human's)

| | **B — GSD** | **A — jigc** | **C — static methodology** |
|---|---|---|---|
| Correct A1–A6 | ✅ | ✅ | ✅ |
| Test added (test-first) | ✗ | ✅ (+30) | ✅ (+39) |
| Test caught a bug | — | no | **yes** |
| Commit body | terse | detailed | richest |
| Flagged lint debt | ✗ | ✅ | ✅ |
| Routing | GSD fast-skill, unprompted | jigc adapter, **unprompted** | static text |
| Ceremony | lowest (1 edit) | jigc workflow (9 calls, 1 exit-1 fumble) | plain edits + tests |
| Wall-clock | <5 min | <5 min | <5 min |

**The non-seeded thesis observation (gate requirement):** on this task **C ≈ A** — the methodology
*written as static text* produced the same test-first thoroughness and commit quality as jigc's
composed workflow, and both out-discipline GSD's fast skill (which skipped the test). **Reading:**
for a small, spec-less code fix, the *methodology content* drives the quality, not jigc's dynamic
machinery — because this task never exercises jigc's actual differentiators (managed docs, the write
channel, validation/drift, forward-ref integrity). jigc (A) matched but did **not** beat the same
methodology as a flat file (C), and carried slightly more ceremony (the dev-task workflow + one CLI
fumble). **Honest implication:** the differentiators live in the *surrounding spine* (flow 21:
planning → increment → completion with ADRs, validation, supersession), not in a lone code task —
which is exactly what the pilot's flow-21 run must exercise to test the thesis where it actually bites.
