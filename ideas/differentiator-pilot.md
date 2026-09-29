# The differentiator-engaging twin pilot — clearing the value gate, reversibly

**Status:** planned, ready to execute (plan finalized 2026-06-21; **execution
deferred to the next session**). The operational spec for the **value gate** the
[readiness assessment](../completions/artifacts/READINESS-ASSESSMENT.md) named: the test that would tell
us whether a productive jigc commitment is worth its lock-in cost. Protocol design
home is [measurement.md](../design/measurement.md) (don't restate it); this file is
the *delta* M17 left un-run + the **environment-isolation harness** + the concrete
study-1 spec.

## Why this exists (the M17-relocated hypothesis)

M17's one controlled jigc-vs-static comparison **tied**, and it tied for a
diagnosable reason: the comparison used a bare `dev-task` (no `allows-create`) that
**engaged none of the differentiators** (managed docs, validation over them,
drift/reconciliation, forward-ref integrity, supersession). M17 named the un-run
cases (P2 existing-docs/doc↔code — "the differentiator most likely to show a
measurable correctness win"; P3 greenfield) and *why* they were skipped: a foreign
project needs an **authored domain pack** to reach the differentiators. This pilot
runs that regime — **reversibly** (twins + plain-file export, zero corpus lock-in),
so it yields the value signal *before* the productive-go gate (M33 freeze + M34
corpus-migration) is even reached.

## The hard problem M17 actually died on: experiment validity

The threat is **contamination**, not effort. Three Claude "methods" (plain / GSD /
jigc) each install different hooks, skills, and config; a shared environment, a
shared memory, or the always-loaded global `CLAUDE.md` makes the arms incomparable —
you'd be comparing three blends, not three methods. M17's run was interrupted on
exactly this. **The isolation IS the experiment.** Confirmed lesson (human,
2026-06-21): last attempt, the **global `~/.claude/CLAUDE.md` loaded anyway from the
shared claude env** — so `HOME`-redirect alone is insufficient; only full
filesystem isolation defeats a sticky global config.

### The isolation harness (mandatory)

- **One container per arm** (devcontainer/Docker) — *mandatory*, not `HOME`-redirect.
  Full filesystem isolation is the only thing that defeats the global-`CLAUDE.md`
  leak. Each container ships **only its own method**: *plain* = bare Claude, nothing
  installed; *GSD* = the GSD skill + its hooks; *jigc* = `jigc` + the domain pack +
  `jigc setup`.
- **Kill ambient inputs in every arm:** disable auto-memory and disable `CLAUDE.md`
  loading for the clean arms (so *plain* is genuinely plain — no `PRINCIPLES.md`/
  `LACON.md`, no memory). *(Exact CC flags/env vars — the guide named
  `CLAUDE_CODE_DISABLE_AUTO_MEMORY`, `CLAUDE_CODE_DISABLE_CLAUDE_MDS`, `--safe-mode`,
  `--bare`, but was uncertain; **verify empirically on the actual CC version at
  build time** before trusting them. There is **no** confirmed `CLAUDE_CONFIG_DIR`.)*
- **Separate git clones, never worktrees** — auto-memory is keyed per-repo and shared
  across worktrees; clones keep memory from leaking arm→arm.
- **Headless, byte-identical prompt:** drive each arm with `claude -p "<pre-registered
  prompt>" --model <pinned> --output-format json`. This removes the thing that broke
  M17 — *a human juggling environments and steering differently per arm*. Build the
  containers, fire the same prompt headless at each, collect transcripts. The model
  has no cross-session memory between independent invocations, so three fresh headless
  runs are genuinely independent.
- **No managed-policy leak:** confirm no `/etc/claude-code/` org policy is present
  (it cannot be disabled and would hit all arms).

## Two studies (don't conflate them)

- **Study 1 — clean-baseline controlled A/B/C (the primary, spec'd below).** A
  hand-crafted, no-AI project → three isolated container-twins → identical
  differentiator-engaging task → blind-judged. The variable is *purely the method*.
- **Study 2 — pairwise-migrated adoption (later, confounded).** For a project already
  on GSD/plain: "continue on incumbent" vs "jigc adopted-from-incumbent" (via
  `jigc migrate` foreign-doc adoption — built; *not* the unbuilt M34 corpus
  migration). Answers the real *adoption* question, but the migration step + incumbent
  state are confounds — so it's a second study, framed as "adoption," never "method."

## Study 1 — the concrete spec

- **Baseline project:** `~/Projects/gherrink-ui-doc` @ **`542b3206`** (2024-10-03) —
  the HEAD of the pre-AI era (next commit `ed15959` introduces Claude Code). TS pnpm
  monorepo; a tag-comment→component-docs generator (5 packages: `core`,
  `html-renderer`, `node`/`rollup`/`vite` plugins). **Contamination guard:** the rich
  `docs/` tree is AI-era — build the twin from `542b3206` where it does **not** exist;
  the neutral doc↔code surface is the **hand-written `packages/core/README.md`**.
- **The doc↔code contract (all hand-crafted at the baseline):** `core/README.md`'s
  "Available Tags" table + per-tag `### @<tag>` sections + cross-reference prose ↔
  the `tag-transformers/index.ts` registry ↔ each `tag-transformers/<name>.ts`
  `name:` field.
- **The matched task (differentiator = doc↔code consistency):** **rename the
  `@hideCode` tag to `@hide-code`** (the lone camelCase outlier; the README's own
  convention is kebab). Correct completion must consistently touch **all five**: (1)
  `tag-transformers/hide-code.ts` `name`, (2) the README table row, (3) the `###
  @hideCode` section, (4) the `@code`/`@example` cross-reference prose, (5) the tests.
  **The measured win is a correctness outcome, not feature-presence:** a freeform
  method renames the code + obvious heading but **misses the table row or a prose
  cross-ref → silent doc↔code drift**; the validating method (jigc, over the anchored
  tag-docs) **catches the dangling reference**. It does *not* ask any arm to "produce
  a doc," so it does not rig toward the doc tool. *(Fallback task: add a tag
  transformer — same differentiator, weaker because no cross-references to break.)*
- **The jigc treatment (the domain pack — the real prep work):** author a minimal
  pack that models the **tag documentation as a managed doctype**, one managed entry
  per tag, each carrying a **`code-anchor` to its transformer `name` field** (jigc's
  `symbol-exists` doc↔code predicate, M27/M29 — TS supported). Adopt the README's tag
  table into that managed shape on the jigc twin (one-time setup, pre-registered as
  adoption cost). Then a rename that breaks the anchor **fires jigc's doc↔code
  validation at finalize** — the differentiator engaging organically.
- **The three arms:** **A** = jigc (+ pack + setup), **B** = GSD (the incumbent
  skill), **C** = plain (bare). *(Optional 4th — M17's "static-methodology": the
  jigc/methodology content frozen flat, isolating dynamic-composition+write-channel
  from content. Add only if the A/B/C result warrants it.)*
- **Measurement:** reuse [measurement.md](../design/measurement.md) verbatim — the
  `dogfood-record` doctype, the three facts, the **pilot gate** (verdict must name ≥1
  **non-seeded** observation), the seeded-failure instrument check, the honesty
  bounds. Judge **blind** (the judge sees the three diffs without knowing which arm)
  via a cross-model judge (Codex, as M17 did). Export records + transcripts as plain
  files to `completions/artifacts/<pilot>/`.
- **Reversibility:** everything on throwaway container-twins; nothing touches the real
  `gherrink-ui-doc` or jigc's own repo. Pure measurement, zero lock-in.

## Honesty bounds (carried from M17)

- **n=1 per task** — a structured pilot, not statistics. Pre-register **2–3**
  differentiator-engaging tasks (the rename + the add-a-tag + one more) so a *pattern*
  across them is the signal, not a single run.
- **The win must be a non-seeded correctness observation** (a real doc↔code drift the
  validating arm caught and another arm shipped), never "jigc has a feature."
- **Blind judging + pre-registration** of intent, acceptance, the prompt, the arm
  order, model + settings — fixed before any arm runs.

## Next-session execution checklist

1. **Verify the CC isolation flags** empirically on the live version (memory-off,
   CLAUDE.md-off, headless, model-pin) — don't trust the guessed names.
2. **Build the three container-twins** from `gherrink-ui-doc@542b3206`, each with only
   its method; confirm `/etc/claude-code/` is absent.
3. **Author the domain pack** (tag doctype + per-tag `code-anchor`) and set up the
   jigc twin (adopt the README tag table as managed, anchored).
4. **Pre-register** the 2–3 tasks, the acceptance criteria, the prompt, the rubric,
   the blind-judge protocol, the seeded failure.
5. **Run headless ×3 (×4 if static-methodology arm)** per task; collect transcripts.
6. **Judge blind** (cross-model), assemble the `dogfood-record`s, check the pilot
   gate, write the verdict; **export plain files** to `completions/artifacts/`.

## Open build-time decisions

- Exact CC isolation mechanism (containers confirmed; flags TBD-by-verification).
- Domain-pack scope — keep minimal (the one tag doctype + anchor is enough to make the
  differentiator bite; resist building more).
- Whether to add the 4th static-methodology arm (decide after the A/B/C read).

## Success criterion

A pilot-gate-passing run whose verdict names **≥1 non-seeded** observation where the
doc↔code differentiator **changed an outcome** the GSD/plain arms got wrong — the
first evidence that jigc **>** static, not **≈** static. That is the signal the
productive-go decision (M33 + M34 + this) waits on.
