# State-aware compose — a checklist with done/pending marks, not a verbatim re-print

**Status: parked 2026-07-06, unscheduled.** From RC greenfield trial 1, finding F2 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Every re-compose reprints the full command template verbatim — including "Create it" for a doc that already exists and `set-slot` commands for slots already filled. The trial had to *silently skip a printed instruction*, which an interface should never make necessary. The state that decides done/pending (doc exists? slot filled? step's write landed?) is CLI-owned store state — the compiler is reprinting instructions whose completion it can already see.

## The shape

Deterministic: compose consults the store (file-state, the task's working area, slot presence) and renders each instruction with a done/pending mark — done steps collapse to one checked line, pending steps keep their full command text. Same resolved cascade + same store state in → same output out; the determinism contract is untouched because store state was always an input to the loop, just not to the *render*.

Side effects that come free: output size roughly halves on re-compose (the trial's estimate), which feeds [composed-context-token-budget](composed-context-token-budget.md); and the `checkpoint` step kind (M15) gets a natural rendering (checked up to the checkpoint).

Fold-in from the same trial: the **two-step routing redundancy** — `jigc start "<intent>"` prints the workflow list, then you re-run with `--workflow`; since the first call already accepts `--workflow` and the SessionStart hook already showed the menu, the intent-only form added little. A state-aware surface should also reconsider what the *first* compose needs to print.

## Trigger

Next milestone planning (the post-RC fix wave) — pairs naturally with [doc-read-surface](doc-read-surface.md); or the adoption trial reproducing the skip-a-printed-instruction moment.
