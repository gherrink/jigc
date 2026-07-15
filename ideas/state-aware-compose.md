# State-aware compose — a checklist with done/pending marks, not a verbatim re-print

**Status: parked 2026-07-06, unscheduled.** From RC greenfield trial 1, finding F2 ([trial-record](../completions/artifacts/RC-greenfield/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

Every re-compose reprints the full command template verbatim — including "Create it" for a doc that already exists and `set-slot` commands for slots already filled. The trial had to *silently skip a printed instruction*, which an interface should never make necessary. The state that decides done/pending (doc exists? slot filled? step's write landed?) is CLI-owned store state — the compiler is reprinting instructions whose completion it can already see.

## The shape

Deterministic: compose consults the store (file-state, the task's working area, slot presence) and renders each instruction with a done/pending mark — done steps collapse to one checked line, pending steps keep their full command text. Same resolved cascade + same store state in → same output out; the determinism contract is untouched because store state was always an input to the loop, just not to the *render*.

Side effects that come free: output size roughly halves on re-compose (the trial's estimate), which feeds [composed-context-token-budget](composed-context-token-budget.md); and the `checkpoint` step kind (M15) gets a natural rendering (checked up to the checkpoint).

Fold-in from the same trial: the **two-step routing redundancy** — `jigc start "<intent>"` prints the workflow list, then you re-run with `--workflow`; since the first call already accepts `--workflow` and the SessionStart hook already showed the menu, the intent-only form added little. A state-aware surface should also reconsider what the *first* compose needs to print.

**2026-07-12 update — the trigger fired, three ways, on the implementation-half trial** ([trial-record](../completions/artifacts/RC-adoption/impl-rc5/trial-record.md) → Probe 3): (a) the sharpest instance yet — after `task bind spec` succeeded, re-compose still printed the full **pick-a-spec gate block** *plus* the caption "The spec's criteria — empty until you bind a spec and re-compose:" directly above the fully-populated criteria: static text actively asserting the opposite of store state, read as "the bind silently failed" (the trial's #1-ranked bug); (b) "satisfied gates keep talking" generally; (c) "everything-at-once" — the first compose dumps implement + commit fields + trailers + changelog + ADR + verify + finalize guidance before a line of code exists ("stage it: tell me what to do now"). The bind-gate *contradiction* is fix-shaped (chartered with the rc.6 wave); the general done/pending render stays this idea.

**2026-07-15 update — demanded again, in all four lacon-trial sessions, now with a status-verb face** ([lacon trial](../completions/artifacts/RC-lacon/trial-record.md); verified [findings-verification](../completions/artifacts/RC-lacon/findings-verification.md) → B4/B5): every session independently asked for "where am I / what's next" — `task status` was guessed and doesn't exist (only `list`), the dev-task runbook scrolled out of reach by finalize time, and planning's one long block hid which phase the agent was "in" (the composed `Checkpoint: settle` separator went uncredited). Two affordances **exist and were missed** — `jigc start --task <id>` resumes (re-composes the full runbook) and `task validate <id>` lists exactly what finalize still needs — so the cheap half is *naming those verbs in composed output*; the "you are here" step cursor + done/pending render stays this idea (the M42 "lighter mode stays unbuilt unless demanded again" note has now been demanded again, twice over).

## Trigger

Next milestone planning (the post-RC fix wave) — pairs naturally with [doc-read-surface](doc-read-surface.md); or the adoption trial reproducing the skip-a-printed-instruction moment. *(Fired 2026-07-12 — see above; the minimal contradiction-fix is chartered, the full state-aware render is the remaining idea. Fired again 2026-07-15 — see above.)*
