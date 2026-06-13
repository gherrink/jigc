# M17 Pilot — Pre-registration (three-arm comparison)

**Written before any arm runs** (measurement.md → The comparison protocol). Carryover is the n=1
killer; this document is the only mitigation. Nothing in the intent, acceptance, interventions,
stop condition, or rubric may change once an arm has started. Arm order is fixed by coin-flip
*after* this is approved, and recorded in § Arm order.

- **Project under test:** `gherrink-galey` (TS/pnpm monorepo, ProseMirror block editor).
- **Twin baseline commit:** `4147a36` (galey `HEAD`, committed; the dirty WIP in the original is
  *not* carried — twins are made from the committed tree). The original is never touched (M12).
- **jigc binary:** pinned `cargo build --release`, sha recorded in the pilot `dogfood-record`.
- **doc↔code:** gated off — galey is non-Rust; recorded, not hidden (measurement.md → case 1).
- **Model / session shape:** same model, one arm per session, same harness.

## The three arms

| Arm | Instruction surface | Isolates |
|---|---|---|
| **A — jigc** | galey `CLAUDE.md` + `jigc setup` 4-line adapter; methodology delivered by **jigc composition** (`jigc start --workflow dev-task "<intent>"`) | dynamic composition + the write channel |
| **B — control** | galey `CLAUDE.md`, **untouched** (the product baseline: jigc vs. what galey has today) | — (baseline) |
| **C — static-methodology** | galey `CLAUDE.md` + the frozen methodology block (`arm-C-static-methodology.md`), static, no jigc | methodology *content*, written down |

A-beats-B alone can't say whether jigc or merely *written-down methodology* did the work; arm C
separates them.

## The matched intent (issued identically to all three arms)

> In `@galey/extension-heading`, the `heading()` factory accepts a `maxLevel` option (1–6,
> default 3). The keymap bindings (`addKeymap`) and the `parseDOM`/`toDOM` tag handling already
> honor `maxLevel`, but the accessibility keyboard contract (`aria.keyboard`) and the theme keys
> (`themeKeys`) are hardcoded to three levels. Make both honor the configured `maxLevel`, so that
> `heading({ maxLevel: 6 })` advertises keyboard shortcuts and theme keys for all six levels (and
> the default `heading()` still exposes exactly three).

Issued to **A** as the `--workflow dev-task` intent; to **B** and **C** as the verbatim session
prompt. No other steering text.

Grounding (verified, jigc session 2026-06-13): `packs`… n/a — in galey,
`packages/extension-heading/src/index.ts`: `addKeymap` loops `1..maxLevel` (L117-122) and `toDOM`
clamps to `maxLevel` (L67), but `aria.keyboard` is literal `Mod-Alt-1..3` (L73-77) and `themeKeys`
is literal `h1..h3` (L80). Existing tests assert only "contains Mod-Alt-1 / h1–h3", never the
`maxLevel` case — so the gap is real and untested.

## Acceptance checks (objective; applied post-hoc by the judge to each arm's landed commit)

- **A1.** `heading({ maxLevel: 6 }).spec.aria.keyboard` has six entries `Mod-Alt-1 … Mod-Alt-6`,
  each with a non-empty description string.
- **A2.** `heading({ maxLevel: 6 }).spec.themeKeys` contains `block.heading.h1 … block.heading.h6`
  (six entries).
- **A3. (no regression)** default `heading()` (maxLevel 3) still yields exactly 3 `aria.keyboard`
  entries and 3 `themeKeys`.
- **A4. (boundary)** `heading({ maxLevel: 1 })` yields exactly 1 of each.
- **A5. (gate)** the project's full gate is green — `pnpm test` (the configured gate) passes,
  including any new test and all pre-existing tests.
- **A6. (contract sanity)** the new `aria.keyboard` descriptions are well-formed (e.g. "set
  heading level N"), not malformed or empty.

"Correct" = A1–A6 all met. Partial = some met. These are scored identically on every arm.

## Allowed human interventions (identical across arms — carryover control)

- The human answers **only** a direct clarifying question the agent itself asks, and **only** with
  the single pre-written clarification below — verbatim, same on every arm. No other hints,
  steering, "did you consider…", or corrections.
- If an agent edits unrelated files, refactors beyond scope, or goes off the rails, the human does
  **not** correct it — that is recorded as drift / ceremony, it is data.
- The human runs nothing on the agent's behalf except what the agent requests through its own
  tools.

**Pre-written clarification (the only allowed answer), used verbatim if asked:**

> Scope is exactly: `aria.keyboard` and `themeKeys` honoring the configured `maxLevel`. Leave the
> toolbar, the message catalog, and everything else unchanged. Use your judgment on the wording of
> the new `aria.keyboard` descriptions.

## Stop condition (when an arm's session ends)

An arm ends at the **first** of:

1. The agent declares the task done **and** has landed its commit (arm A: `finalize`; B/C: `git
   commit`). The arm ends at that first landed commit — no post-commit iteration is scored.
2. The agent is blocked and asks for input that is **not** the pre-registered clarification.
3. A runaway cap: the agent is visibly looping or exceeds ~45 min / ~40 assistant turns on the
   task. Record the reason and the partial state.

## Judge rubric (measurement.md → The judge; prose + a single `green | red` enum, never computed)

Scored per arm, with the pre-registered facts in hand:

- **Correctness** — which of A1–A6 are met (the objective core). pass / partial / fail.
- **Doc / commit-message quality** — is the commit message accurate, correctly scoped, and
  Conventional-Commits-shaped? (1–5)
- **Drift left behind** — unrelated files touched, dead code, anything broken (count + severity).
- **Ceremony cost** — turns, wall-clock, total tool calls, and (arm A) jigc invocation count spent
  on tool friction. Counted on **every** arm; jigc's overhead is *data, not noise*.

The verdict prose must name **≥1 non-seeded thesis observation** (an organic event or a judged
difference between arms) — a gate passed on apparatus-planted events alone has measured nothing.

**Honest bounds carried into the verdict:** n=1 per arm (structured pilot, not statistics); the
judge is unblinded (transcripts self-identify; mitigations are the recorded arm order + the
committed hook log); adherence is gameable (hence the seeded-failure obligation, satisfied in the
surrounding pilot run, not in this matched-task comparison).

## Arm order

*Fixed by coin-flip after approval. To be recorded here before the first arm runs:*

```
ORDER: B → A → C   (control → jigc → static-methodology)
METHOD: shuf --random-source=/dev/urandom over {A,B,C}, recorded at draw time
DATE: 2026-06-13
```

## Approval

- [x] Matched intent approved (human chose "Heading maxLevel consistency", 2026-06-13).
- [x] Arm C frozen content approved (`arm-C-static-methodology.md`).
- [x] Rubric + acceptance + interventions + stop condition approved (human, 2026-06-13).
- [x] Arm order coin-flipped + recorded (B → A → C, 2026-06-13).
