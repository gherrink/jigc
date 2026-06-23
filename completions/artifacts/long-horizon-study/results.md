# Long-horizon many-edit study — results

**Matrix:** 5 arms × Sonnet (R=3) + 2 arms × Opus (R=2) × 8 sequential edits =
**19 sequences, 152 cold-agent runs.** Each edit is a fresh container against the
*evolving* twin; drift is measured on the committed HEAD by the real `doc-code`
probe (anchor resolution) + a dead-symbol grep over titles/prose. Run 2026-06-22/23
(Opus arm re-run after an OAuth-credential expiry truncated the first attempt — see
*Data integrity* below). Objective oracle + blind cross-model judge (Codex) agree.

## Headline: the cumulative dangling-anchor curve

Mean dangling anchors in committed HEAD after edit N (the surface jigc's hook governs):

| arm | model | e1 | e2 | e3 | e4 | e5 | e6 | e7 | e8 |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| **P** plain | Sonnet | 0.33 | 0.33 | 1.33 | 2 | 3 | 4 | 4.67 | **5.0** |
| **C40** clean rule | Sonnet | 0 | 0 | 0.33 | 0.33 | 0.33 | 0.33 | 0.33 | **0.33** |
| **C160** 157-line | Sonnet | 0 | 0 | 1 | 1 | 1 | 1 | 1 | **1.0** |
| **C550** 452-line | Sonnet | 0 | 0 | 0.67 | 0.67 | 0.67 | 0.67 | 0.67 | **0.67** |
| **A** jigc-hook | Sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** |
| **C550** 452-line | Opus | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** |
| **A** jigc-hook | Opus | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** |

Stale-prose curve (dead symbol names in titles/prose — the surface the hook does *not* govern):

| arm | model | e1 | e2 | e3 | e4 | e5 | e6 | e7 | e8 |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| **P** plain | Sonnet | 0.33 | 0.33 | 0.33 | 1 | 2 | 3 | 3.67 | **4.0** |
| C40 / C160 / C550 | Sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** |
| C550 | Opus | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** |
| **A** jigc-hook | **Opus** | 0.5 | 1 | 1 | 1.5 | 1.5 | 2 | 2 | **3.0** |
| A jigc-hook | Sonnet | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** |

## Final-state + behavioral rates (mean over reps)

| arm | model | reps | final dangling | final stale | cost/seq | engaged | blocked | judge consistent |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| P | Sonnet | 3 | 5.0 | 4.0 | $2.77 | 0/3 | — | **0/3** |
| C40 | Sonnet | 3 | 0.33 | 0 | $3.87 | 0/3 | — | 3/3 |
| C160 | Sonnet | 3 | 1.0 | 0 | $4.34 | 0/3 | — | 3/3 |
| C550 | Sonnet | 3 | 0.67 | 0 | $4.68 | 0/3 | — | 3/3 |
| **A** | **Sonnet** | 3 | **0** | **0** | $4.52 | 3/3* | 3/3 | **3/3** |
| C550 | Opus | 2 | 0 | 0 | $8.71 | 0/2 | — | 2/2 |
| **A** | **Opus** | 2 | **0** | **3.0** | **$27.64** | 2/2 | 2/2 | **0/2** |

\* "engaged" = the agent ran ≥1 `jigc` command somewhere in the sequence. Per-edit it
was *variable and often absent*: A-Sonnet recovered from hook blocks on edits 1 & 3 via
**direct file edits, zero jigc commands** — the clean outcome is **hook-driven, not
engagement-driven** (the "enforce, don't instruct" thesis: the block forces repair
whether or not the agent chooses the tool). Plain/static agents ran jigc 0 times (not installed).

## The three diagnosed mechanisms

### 1. Plain compounds catastrophically — drift is the norm at scale
Unguided, dangling anchors rise monotonically 0.33 → **5.0** and stale prose → **4.0**
over 8 edits. A fresh cold agent does only *its* ticket; an earlier edit's missed doc
update is never revisited. The 88%-style base rate of the pilot, compounded.

### 2. jigc-Sonnet: a clean strict win — and it catches a drift class instruction structurally misses
A-Sonnet holds **flat at 0** dangling and **0** stale across all 8 edits, 3/3 reps,
blind-judge 3/3 consistent, at cost ($4.52) comparable to static. Two sub-findings:

- **Diligence gap on the move.** Every static arm's residual dangling anchor is
  **edit 3, the file *move*** (`errors/BlockParseError.ts` → `errors/parse/…`). A move
  does not change the symbol *name*, so the static rule's method ("search the project
  for the old name") finds nothing, and a human — and the symbol-checking blind judge —
  see a doc that *looks* correct. But the recorded **file path is dead**. jigc's probe
  resolves the full `path#symbol` against the tree, caught it, and the blocking hook
  forced repair. **6/9 Sonnet static reps shipped a dead move-path; jigc shipped 0/3.**
  This is "validate against reality" beating "remember to check" — the integration
  advantage, demonstrated.
- **Hook-driven, not tool-driven.** The win does not require the agent to adopt jigc:
  on most edits Sonnet edited files directly, hit the hook on `git commit`, and fixed
  the anchor to get past it. Salience-independent enforcement, as predicted.

### 3. jigc-Opus: the prose blind spot backfires — jigc ends *dirtier* than static, at 3× cost
A capable model inverts the result. Opus *engages* jigc (2/2) and the hook keeps
**anchors** perfectly clean (dangling 0) — but Opus **surgically fixes only the anchor
to clear the gate** and commits, leaving the component **title** naming the dead symbol.
Stale prose compounds to **3.0** mean (5 in the worst rep — blind judge **0/2
consistent**), the *worst doc-honesty of any arm except plain*. Meanwhile **static-Opus
stays perfectly clean (0/0, judge 2/2)**: a capable model simply follows the broad
written instruction ("update every documentation reference, including headings"), which
covers prose the hook cannot. And jigc-Opus cost **$27.64/sequence** (Opus thrashing
against the gate) vs **$8.71** static — ~3×.

## Reconciliation: probe oracle vs blind judge (they measure different surfaces)
The objective probe and the Codex judge agree on every cell *except* the static-Sonnet
move case, and the disagreement is itself the finding: the **probe** resolves the full
`path#symbol` (catches the dead move-path), while the **judge** (and a human) check
symbol *names* in titles/anchors (miss a stale path whose name is unchanged). Net: the
probe is the stricter, correct oracle for anchor honesty; the judge is the authority on
prose honesty (the surface the probe ignores). The verdict rests on both, each in its
domain.

## Data integrity
The first Opus pass was truncated by an OAuth-credential expiry at ~23:21 (every
subsequent agent invocation failed with HTTP 401 `authentication_failed`). **All 15
Sonnet sequences completed before the expiry (0 × 401) and are intact**; one Opus
sequence (`C550-8-rep1`) also beat it. The 3 affected Opus sequences were **re-run from
scratch after credential refresh** (0 × 401, 0 dead edits) and the truncated originals
preserved under `runs/opus-truncated-401/`. No Sonnet (headline) data was affected.

## Bounds
n = 2–3 per cell (a structured study, not powered statistics — the per-edit *curve* and
the *ladder* are the signal). One task family (symbol rename/move/delete on a TS repo),
doc↔code only. Agents ran without `node_modules` (build/test unavailable — uniform across
arms). The blocking hook is a **1-line config delta** from jigc's shipped *warn-only*
default, not its out-of-the-box behavior. The hook is bypassable (`--no-verify`); it was
**never bypassed on arm A** (0/5), though C160-Sonnet agents used `--no-verify` 2/3
(on a repo with no hook — habitually, not to defeat enforcement).
