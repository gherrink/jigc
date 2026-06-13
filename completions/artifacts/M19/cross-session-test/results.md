# Results — the cross-session drift test (M19)

**Run 2026-06-13**, after the pre-registration + appendix were frozen and the binary re-pinned
(`jigc` sha `5d64adc…`, hook-installing + finalize-relaying). 6 chains (3 jigc / 3 static) × 8
cold-blind-subagent steps = 48 cold-subagent runs, sequential within a chain (genuine forgetting —
fresh context per step), parallel across. Scored mechanically (per-commit citation resolution over
each twin's git history) **and** independently re-verified + adversarially challenged by a separate
analyst. **End-state is a tie; the timeline is the finding.**

## Primary metric A — final stale-citation count: 0 in all 6 chains (tie)

| Chain | arm | final stale |
|---|---|---|
| J1 / J2 / J3 | jigc (warn-only backstop installed) | **0 / 0 / 0** |
| S1 / S2 / S3 | static (no detector) | **0 / 0 / 0** |

All six converge to the identical 12-citation final set, every citation resolving to a real
definition (including the `clock.rs`→`time.rs` move). Independently confirmed. **A flat tie on the
end-state — H0 on the comparative "more honest over time" claim.**

## Primary metric B — the per-step timeline: this is where the arms differ (and it surprises)

**The static arms were CLEAN at every commit.** S1/S2/S3 each fixed the citation for the symbol they
renamed **in the same commit** — zero transient drift, ever, across all 24 static steps. Right-first-try.

**The jigc arms transiently drifted, then recovered** (exact shas, independently verified):
- **J1** — `cce33c1` (rename `refill`) shipped `architecture…#refill` **stale**; the next step's commit
  `b2815b2` repaired it. (1 citation, ~1 commit.)
- **J2** — `590fec6` (inline/delete `try_acquire`) shipped `architecture…#try_acquire` **stale**; it
  **persisted 3 commits** (`590fec6`→`186af99`→`4fca400`) until step 5's `538f948` re-pointed it to
  `#tick_and_take`.
- **J3** — accumulated **`#SlidingWindow` + `#count_in_window` + `#select_policy` stale across 4
  commits** (`a2436fd`→`1ba2370`), then a later step's cleanup commit `1aa0c7b` cleared them.

**The backstop demonstrably fired in anger and drove the repair — the thing M18 lacked.** The J repair
commits **credit the detector in their own messages**: `fe148db` — *"point the architecture citation at
the current symbol **so jigc validate passes**"*; `1aa0c7b` — *"fix a pre-existing broken citation …
**which the store-scope doc-code gate flagged**, so jigc validate passes."* So in M19 the detector
**ran, surfaced real accumulated cross-session drift, and an agent acted on it** — exactly the loop M18
never exercised (there the detector never fired).

## Verdict — H0 on the comparative thesis; a genuine positive on the mechanism

**The test does not demonstrate that jigc yields a more-honest doc set over time than diligent static
maintenance.** Both arms ended at **0 stale citations**. The over-time advantage is **undemonstrated —
again** — but for a different, sharper reason than M18:

1. **jigc did not win — and was, in fact, the only arm that drifted.** Two things are both true and
   must be said together: the static arm **stayed clean at every commit**, and the jigc arm **incurred
   transient stale citations and relied on a later step to repair them**. jigc's backstop earns a
   *comparative* win only when diligence leaves **residual** drift in the **final** state for the
   detector to have caught and the control to have shipped — and here the static cold agents reached a
   clean end-state **right-first-try**, even under genuine per-step forgetting and a no-cue sequence, so
   there was no residual static drift to beat. On the **per-commit timeline the static arm strictly
   dominated** (clean throughout vs jigc's up-to-4-commit drift windows). (Bound: static's record is
   **N=3 first-try success on a small, well-telegraphed citation set** — "static never drifts" is a
   property of this easy task, not proven robustness; a denser/longer or less-careful regime could
   still separate the arms.)
2. **What M19 *did* demonstrate, narrowly: the detect→repair mechanism fired and was acted on.** Unlike
   M18 (detector never fired), here the **warn-only store-wide backstop fired automatically, surfaced
   real cross-session drift cold agents had missed (persisting up to 3–4 commits), and an agent acted on
   it** — the J repair commits credit it by name. The auto-firing mechanism + the finalize relay
   **closed the detect→repair loop *in this run, when agents heeded the warning*** (warn-only, so the
   closure is an agent-behavior property, not a guarantee of the mechanism alone). This is a real
   capability gain over M18 — but it was **neither a comparative end-state win nor a comparative
   timeline win** (static dominated per-commit cleanliness). It shows the loop *can* close, on drift
   that **arose in jigc's own arm**.
3. **Honest mechanism — net-dependent cleanup latency, NOT moral hazard.** The tempting read ("jigc
   agents got sloppy because they had a net; static agents were careful because they didn't") is
   **over-read** (independently challenged). The jigc agents *also* attempted per-step honesty — J3's
   slip was **missing a *second* citation site** of a symbol whose first site it did update (partial
   coverage, not negligence). The accurate description: **static was clean at every commit; jigc
   introduced stale citations and relied on later detector-prompted cleanup to land clean.** Phrased as
   paths: **jigc operates drift→detect→repair**, **static operates right-first-try** — but the paths are
   not rhetorically symmetric, because only one arm ever exposed stale citations, and it was the jigc
   arm. They reach the same clean end-state; static reaches it without ever being wrong.

**The load-bearing caveat (the bound the build's contract already named):** the backstop is
**warn-only / advisory** — its value is entirely *"an agent that reads the warning fixes it."* In this
run agents did read and fix (commit messages prove it). But the boundary is **ergonomic, not enforced**
([VISION.md](../../../VISION.md) principle #3): an agent that ignored the relayed warning would ship
the drift, and jigc would tie-or-lose. M19 demonstrates the loop *can* close, not that it *must*.

## Where the comparative advantage could still show (owed, not reached here)

The arms tie because static diligence didn't fail. To separate them honestly you need a regime where
first-try diligence **predictably leaves residual drift** that only an automatic detector catches:
larger/denser citation graphs (dozens of sites per symbol), genuinely separated sessions (days apart,
real context loss — not cold subagents in one orchestration), or a multi-author setting where the
agent changing the code is not the agent who knows the docs. This run deliberately did not reach that,
and says so.

## Bounds (carried into the verdict)

- **End-state tie; jigc's arm was the only one that drifted (static dominated the per-commit
  timeline). The backstop's demonstrated value is loop-closure (it fires + drives repair on drift that
  arose in jigc's own arm), not a comparative end-state or timeline win.**
- **Warn-only / advisory** — contingent on an agent reading + acting on the warning; not enforced.
- **Static's clean record is N=3, first-try, on a small well-telegraphed citation set** — not proven robustness.
- **Cold-per-step ≈ but ≠ true cross-session** (still one orchestration; subagents ≈ not = human sessions).
- **Structural drift only** (`symbol-exists`) — behavioral prose drift out of frame.
- Orchestrator built both environments; the arms ran blind; the timeline is objective git-history fact,
  independently re-verified.

## Artifacts

- Pre-registration + appendix (frozen): `pre-registration.md`, `appendix-sequence.md`.
- Seeds: `/home/maurice/jigc-dogfood/mdt19-seed-{J,S}`; chains: `mdt19-{J,S}{1,2,3}` (git histories carry
  the per-commit timeline — reproducible: does each cited `path#symbol` resolve at commit N).
- Independent verification (final counts + timeline shas + the interpretation challenge) in this
  session's transcript.
