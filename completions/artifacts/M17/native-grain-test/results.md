# Native-grain thesis test — results & verdict

**Run 2026-06-13**, per the frozen [pre-registration](pre-registration.md) (+ amendments 1–2). Two
blind, independent subagents authored the same structure-rich deliverable (an `arch-doc` for the
reconciliation/file-state subsystem + a superseding ADR) from an identical bare intent — **arm J**
through jigc, **arm S** by hand under a static `CLAUDE.md` freezing the same doctype conventions.
An independent judge (**OpenAI Codex / gpt-5.5**) scored the committed artifacts against the frozen
defect rubric, blind to which arm was which (sets labeled A/B; mapping revealed only after).

## Headline result — the null was not rejected; if anything the static arm edged it

| | Arm J (jigc) = judge's Set B | Arm S (static) = judge's Set A |
|---|---|---|
| Committed structural-integrity defects | **1 / 7** (D3) | **0 / 7** |
| D2 supersedes resolves & correct | CLEAN | CLEAN |
| D3 superseded ADR's status updated | **PRESENT (defect)** — left `accepted` | CLEAN — flipped to `superseded` |
| D4 cites resolve | CLEAN | CLEAN |
| D5 implemented-by symbols exist | CLEAN (7/7) | CLEAN (7/7) |
| D1 / D6 / D7 | CLEAN | CLEAN |

**On the headline metric, arm S (static, by hand) was cleaner: 0 defects vs jigc's 1.** **H1 — "jigc
commits fewer structural-integrity defects at native grain" — is *not* supported by this run.** The
null (no jigc advantage on the count; a capable model hand-structures correctly from good static
conventions) stands, and the one committed defect was jigc's.

## The nuance the bare count misses (both directions — stated, not spun)

The count is real, but three things complicate reading it as "static beats jigc":

1. **Jigc's clean cross-refs were *machine-guaranteed*; arm S's were *diligence-dependent*.** Arm J
   *could not* have committed a dangling `cites`, a wrong `supersedes`, or a non-existent
   `implemented-by` symbol — jigc's finalize gate (edge index + `doc-code` probe) blocks them. Arm J
   even reported the gate catching its own mistakes mid-run (an unfilled commit doc; see below). Arm
   S got every cross-ref right by **hand-grepping to verify** before committing — correct *this run*,
   but nothing *prevented* a dangling ref; on a less careful run or a larger task that diligence is
   what would slip. The count says they tied on D2/D4/D5; the *mechanism* differs (guaranteed vs.
   careful-this-time).
2. **Jigc's one defect (D3) is a capability gap, not an LLM structural fumble.** Ironically, the tool
   whose whole point is owning structure *couldn't perform a structural update*: the MVP exposes no
   verb to flip a field on an already-committed doc, and the supersede flow didn't carry the old
   ADR's `status → superseded`. The by-hand arm just edited the file. So jigc lost the one point on a
   **missing feature**, not on the error class the thesis is about (misplacement / dangling refs).
3. **The task was within a careful model's hand-structuring ability** — the pilot's lesson, one grain
   up. Even structure-rich (7 components, 3 cross-refs, a supersession), a moderate, well-scoped task
   did not push the static arm into the dangling-ref / misplacement errors jigc prevents by
   construction. Where jigc's guaranteed integrity would plausibly *win on the count* is a task big or
   interconnected enough that hand-diligence breaks down — many docs, symbol-moving refactors that
   invalidate `implemented-by` anchors, cross-refs across dozens of files. This run did not reach
   that regime.

## Product gaps this test surfaced (real, actionable — the test paid for itself here)

- **No verb to update a field on a committed doc.** The cause of jigc's only defect (D3). Supersession
  is left structurally incomplete: jigc records the new `supersedes` edge but cannot mark the
  superseded ADR `superseded`. (Workaround exists via idempotent-create + re-promote, but the
  composed supersede flow neither does it nor guides the agent to it.) *Owed.*
- **`set-field` on a `0..*` ref is silently last-write-wins.** Arm J set `cites` "one `set-field` per
  ADR" (as the guidance suggested) and silently kept only the last; it caught this at preview and
  corrected via the inline-list form. A repeated `set-field` on a list-cardinality ref should
  accumulate or warn, not silently overwrite. *Owed.*

## Verdict

**This run does not demonstrate jigc's superiority at native grain; on the chosen metric the static
arm was marginally cleaner, and jigc's lone defect was a fixable capability gap.** What the run *does*
show is narrower and honest: jigc's structural-integrity guarantees are **real and enforced**
(dangling refs and bad anchors are impossible past the gate, where the static arm merely happened to
get them right by hand), but on a moderate, careful run those guarantees **did not convert into a
lower defect count** — and jigc's own feature gaps cost it a point. The thesis's mechanism is
visible; its *measurable* payoff is not, at this task size.

**Consistent with the M17 verdict's conclusion:** jigc's value is not a universal "fewer defects than
a static file." If it concentrates anywhere, it is in **guaranteed** integrity under conditions where
hand-diligence fails — larger, messier, refactor-heavy work — which this test did not reach. That is
the next test, if pursued: scale the task until hand-structuring breaks.

## Bounds

- **n=1 per arm, one task, one subsystem.** A point, not a distribution.
- **Blinding imperfect** — the arms' on-disk formats differ (jigc renders H1 = slug, `<!-- fields -->`
  markers; the static arm wrote conventional headings), so the judge could likely infer origin. The
  rubric is objective and evidence-backed (every verdict quotes a line or a grep), which is the
  mitigation; but it is not true blinding.
- **Subagents ≈ not = human sessions.** Independent fresh contexts (no cross-arm carryover), full
  tools — the property the comparison needed — but not a human at a real terminal.
- **Orchestrator authored both environments** (the arms ran blind; the setup is mine). Symmetric
  project-facts base verified; the jigc-vs-hand authoring mechanism is the only intended difference.
- **One metric, one dimension** — structural-integrity defects. Says nothing about prose quality,
  speed, or maintenance-over-time.

> **Post-test probe correction (2026-06-13):** scoping the *maintenance/drift-over-time* follow-up
> empirically corrected an assumption stated in an earlier draft of this doc — that jigc "re-checks
> doc↔code on every later finalize." It does **not**. Three probes in the jigc twin: a stale
> `implemented-by` anchor (a symbol the committed arch-doc names was renamed in the code) is **not**
> caught at an unrelated task's finalize, and **not** caught when the doc itself drifts and is
> absorbed; it is caught **only** when the doc is re-authored through its own workflow — and there is
> **no store-wide `jigc validate`** to sweep committed docs against current code. So jigc enforces
> doc↔code integrity at **authoring time, not over time**: a committed doc silently rots under moving
> code until deliberately re-authored. This gap — and the milestone to close it (build the store-wide
> re-validation, then run the maintenance/drift comparison) — is recorded in
> [decisions-pending.md](../../../implementation/decisions-pending.md) and the roadmap (M18).

## Disposition

The native-grain self-hosting test ran clean and returned an honest, judge-adjudicated result that
**did not confirm the thesis and surfaced two real product gaps**. The "jigc wins under
hand-diligence-breaking scale" hypothesis is now the sharpest open question — and the maintenance/
drift-over-time advantage remains entirely untested by any one-shot comparison.
