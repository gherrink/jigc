# Handover — next session: the long-horizon many-edit study (prove *superiority*, not parity)

Written 2026-06-22. HEAD = `0a4e215` on `main`, tree clean, **pushed** to
`github.com/gherrink/gherrink-jigc` (private). The previous session built the
foundation this study needs — the universal finalize floor (hardened through two
Codex review rounds) and the behavioral workflow-eval harness. This session runs
**the study those were the prerequisites for**: does jigc make a coding agent
*measurably more correct over many edits than a static `CLAUDE.md`* — the one claim
the project has never demonstrated.

## The one question this session exists to answer

Every controlled jigc-vs-static comparison to date has **tied** (M17, M18, M19 — all
returned H0, "no difference," at zero stale citations). The architectural thesis is
built and design-proven; the **empirical superiority claim is not**. This study is its
best remaining shot. Read [VISION.md](../../../VISION.md) → "What is proven, and what
isn't (M17–M19)" first — it is the precise statement of where we stand and why the
ties may not hold at real scale.

## Why the ties happened, and the regime that should break them

The mechanism, straight from VISION (not new analysis):

- All three studies held **instruction salience** fixed: the static arm got a *clean,
  on-point* standing rule ("keep citations honest") with nothing competing for it.
  That is the **best case for static conventions**, and symmetry *required* it (a fair
  comparison gives both arms the same clean instruction).
- Real `CLAUDE.md` files are **not** clean — they are long, half-stale, internally
  competing, and the relevant rule **loses the attention budget as the file grows**.
- The two mechanisms diverge exactly there: **static conventions are
  salience-dependent** (work only as well as the agent attends to the right line, which
  decays with rules-file size); **jigc's enforcement is salience-independent** (a
  mechanism fires identically whether the rules file is 5 lines or 5,000).

So jigc's advantage is *predicted* to appear in the regime the prior tests **excluded**:
**many edits, over a long/forgetful horizon, with a large competing instruction
surface.** That is what "long-horizon many-edit" means. The catch VISION names
honestly: this regime is **structurally hard to test fairly** — handing the static arm
a realistically bloated rules file breaks symmetry and confounds the result (did the
control lose because conventions degrade at scale, or because *that* file was badly
written?). **Cracking that fair-control design is the intellectual core of this
session.** Do not hand-wave it.

## What is now in place that M17–M19 did not have

1. **A *blocking* finalize floor (this session, `af26db1` + remediations).** M18/M19
   had only the store-wide `jigc validate` sweep + a **warn-only** pre-commit backstop
   (advisory — "closes the loop only when an agent heeds the warning"). Now jigc
   **blocks** doc↔code anchor drift at the commit boundary under *every* workflow
   (`design/validation.md` → Scope = effective state). This is a genuinely stronger
   differentiator to test: *prevention*, not *advice*. It has been through two
   adversarial cross-model reviews (`DECISIONS.md` 2026-06-22 → the two "Floor
   remediation" entries) and is verified to block newly-caused drift while not
   false-attributing pre-existing drift.
2. **A behavioral workflow-eval harness (`completions/workflow-eval/`).** Reusable,
   per-workflow, agent-in-the-loop: `run-eval.sh` (one isolated container run,
   capturing **stream-json** — the tool-call trace), `analyze.py` (one transcript →
   engage/select/complete + outcome), `eval.py` (N runs → rates + markdown),
   `selftest.sh` (verifies the analysis layer, zero API spend). This is the
   **measurement substrate** for this study. Read its `README.md`.

## Three things the floor changes about the study design

- **The floor only fires if the agent routes through `jigc`.** It lives at `jigc task
  finalize`. An agent that edits files and commits via **plain git bypasses it** (the
  warn-only pre-commit hook is the only backstop on that path). The pilot's hardest
  finding stands: **Sonnet ran a jigc command in 0/32 bare-task runs.** So the study
  must *measure and/or control* how the agent commits — jigc-finalize vs git-bypass —
  or the floor's effect is invisible regardless of its strength. "Enforce, don't
  instruct" is only real if routing is forced; see the pilot
  [REPLICATION.md](REPLICATION.md) → Implication.
- **The floor catches anchor drift, not prose drift.** `symbol-exists` knows a symbol
  *exists*, not that the prose still *describes* it (the M18 blind spot, unchanged —
  `VISION.md` M18). Pick a drift measure the floor actually governs (symbol/anchor
  honesty), or you will measure a thing jigc does not claim to fix.
- **The win condition is narrow and must be hit on purpose.** jigc strictly wins only
  where a *diligent* static arm still leaves **residual** drift — i.e. where
  instruction degrades (long session / bloated rules / weaker model under load) and the
  mechanism keeps firing. M19 never reached that regime; design the task sequence so it
  *does*.

## A concrete protocol sketch (refine, then pre-register before running)

This is a starting point, not gospel — the previous sessions' rigor (pre-registration
*before* any run, blind cross-model judge) is mandatory; see
[design/measurement.md](../../../design/measurement.md) and
[pre-registration.md](pre-registration.md) for the template the pilot used.

1. **Twin + corpus.** A real repo twin with managed docs whose `code-anchor`s tie to
   code that the edit sequence will churn (the pilot used `gherrink-ui-doc @ 542b3206`;
   reuse or pick something with more surface). Author the managed arch-doc/ADRs as the
   pilot RUNBOOK §2 shows.
2. **A long edit sequence (the "many-edit" axis).** N sequential tickets (rename / move
   / split / delete a documented symbol, add a new documented component, …), each a
   **fresh cold agent** (genuine cross-session forgetting — M19's design), run one after
   another against the *evolving* repo. N large enough that instruction salience can
   decay and residual drift can accumulate (M19 was too short).
3. **Arms (the fair-control crux).** At minimum: **(A) jigc** (routed, floor active) vs
   **(C) static `CLAUDE.md`**. The honest hard part is C's rules file. Options to weigh
   — *name the tradeoff explicitly in the pre-registration*:
   - **C-clean** (idealized, on-point rule) — the symmetric control the prior ties used;
     re-running it is the "did anything change with the *blocking* floor?" baseline.
   - **C-realistic** (a long, multi-rule file where the doc-consistency rule competes
     for attention) — the regime that should favor jigc, but the confound VISION names.
     If you run it, you must defend *why this file is representative* (e.g. derive it
     from a real open-source `CLAUDE.md`, not author a strawman) so "the control lost
     because the file was bad" is not a live alternative explanation.
   - Consider a **dilution ladder** (rule at 40 / 150 / 500+ lines) to *measure* the
     salience-decay curve rather than pick one point — that turns the confound into the
     finding.
4. **Models.** Sonnet (the weaker model where instruction provably degrades — the pilot
   showed it) is the primary signal; Opus as the "too capable to drift on small tasks"
   control. The over-time regime is where even capable models may degrade.
5. **Measures (objective first).** Per edit and at end-state: anchor-drift rate (the
   floor's domain), floor block-rate + recovery, **jigc engagement/route** (from the
   harness — did the agent even use jigc?), turns/cost (ceremony tax). Plus a **blind
   cross-model judge** over final docs (the pilot's `judge/` pattern) for the prose
   honesty the floor *doesn't* govern.

## Tools & how to run

- **Harness:** `completions/workflow-eval/` — `run-eval.sh <image> <model> <prompt>
  <out>` per step, `eval.py` to aggregate, `selftest.sh` to keep the analyzers honest.
  **Extend `analyze.py`/`eval.py` for multi-edit *sequences*** (per-edit outcome over an
  evolving repo) — today it scores independent single runs; the long-horizon axis needs
  a per-step timeline. That extension is itself a deliverable.
- **Container harness gotchas:** [RUNBOOK.md](RUNBOOK.md) — `USER node`,
  `--permission-mode bypassPermissions`, prompt via `-e PILOT_PROMPT` (baked into
  `run-eval.sh`). The pilot's `~/diff-pilot/` workspace + images are **gone**; rebuild
  per RUNBOOK §1–3 (glibc ≥ 2.39 base — `node:20-trixie-slim`).
- **The floor / binary:** `jigc 0.0.0` at `~/.local/bin/jigc` (+ `doc-code` probe).
  Rebuild + reinstall (`cargo build --release -p cli && cp target/release/jigc
  ~/.local/bin/`) if you touch jigc-core; `jigc setup` re-extracts the probe.
- **First, certify the harness on real data.** The Phase-3 honest boundary: the
  stream-json parser is pinned only to synthetic fixtures. **Do one live `run-eval.sh`
  run and spot-check the `transcript.jsonl` against `analyze.py`'s record** before
  trusting any matrix — the one thing fixtures can't certify
  (`completions/workflow-eval/README.md` → Honest boundaries).

## Honest boundaries / risks to hold

- **A fourth tie is a real possible outcome** — and would be informative (it would say
  the *blocking* floor, too, doesn't beat diligent static maintenance in the regime
  tested). Pre-register so a null result is publishable, not massaged.
- **Cost & time.** Each edit is a full cold-agent session; a many-edit × multi-arm ×
  multi-model matrix is large. Scope N and reps to the question; lean on `selftest.sh`
  (free) to keep iterating the harness between paid runs.
- **Don't let routing failure masquerade as a floor failure.** If the agent bypasses
  jigc (0/32 precedent), the floor can't fire — that is a *routing* result, not a
  *mechanism* result. Measure engagement explicitly and report it separately.

## Explicitly OUT of scope

- **Productive corpus / self-hosting** — unchanged: do **not**. M33 (schema/format
  freeze) + M34 (corpus migration) + a *cleared* value gate still gate it, and this
  study is what *might* clear the value gate. Twin-only, plain-file export, zero lock-in.
- **Re-opening the floor design** — it is built, reviewed (twice, cross-model), and
  pushed. Use it; don't re-litigate it unless the study surfaces a concrete defect.

## Reading order

This file → [VISION.md](../../../VISION.md) → "What is proven (M17–M19)" → the M17/M18/M19
`results.md` under `completions/artifacts/` → [REPLICATION.md](REPLICATION.md) +
[NUDGE-ARM.md](NUDGE-ARM.md) (the engagement/selection findings) →
[completions/workflow-eval/README.md](../../workflow-eval/README.md) →
[design/measurement.md](../../../design/measurement.md) +
[pre-registration.md](pre-registration.md) → `DECISIONS.md` 2026-06-22 entries (Phases
0–3 + the two floor remediations).

## Success criterion for the session

A **pre-registered, fairly-controlled long-horizon comparison run end-to-end** with an
objective drift measure the floor governs, an explicit engagement measure, and a blind
judge — yielding a defensible verdict on **superiority vs parity vs the salience
confound**, whichever way it falls. The deliverable is *a credible answer to the
founding empirical question*, not a win.
