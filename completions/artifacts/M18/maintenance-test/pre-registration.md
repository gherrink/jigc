# Pre-registration — the maintenance / drift-over-time thesis test (M18)

> **STATUS: FROZEN at sign-off (2026-06-13).** The design forks are resolved (see "Resolved at
> sign-off" below); this is the frozen design. The seed + the exact per-step edit sequence
> (`appendix-sequence.md`) are constructed next and committed before any arm runs. Deviations after
> this freeze are recorded as timestamped amendments.

This is the test M18 was built for: jigc's strongest *distinctive* claim — **"living documentation
stays honest against the code"** — tested **over time**, the one ground the one-shot comparisons
(pilot, native-grain) couldn't reach. M18 shipped the passive detector (`jigc validate`, a read-only
store-wide doc↔code sweep); this test asks whether that detector makes a coding agent's documentation
**measurably more correct over a maintenance sequence** than the same agent working from a static
doc set with no detector.

## The design problem this test must avoid (the reviewer's S1)

The naïve framing — "jigc's catch-rate vs. the static arm's silent rot" — is **tautological and
rigged**: the static arm has no detector, so it "catches" zero by construction; jigc catches what
its probe can see. That restates that jigc has a feature the control lacks; it proves nothing about
*value*. The native-grain test already taught the real lesson: a capable agent can do by **diligence**
what jigc does by **machine** — and at moderate scale, diligence ties. So this test is built around
the question diligence-vs-machine actually poses:

> **Over a realistic code-evolution sequence where keeping docs in sync is a *secondary* concern,
> does the agent *with the detector* end with a more-honest doc set than the agent who must
> *remember* to keep docs honest by hand?**

Drift happens in the real world because people change code and **forget** the docs — not because they
lack a grep. So the metric is the **honesty of each arm's final committed doc set**, with *both* arms
told to keep docs honest and *both* able to — one with `jigc validate`, one by manual diligence.
That comparison can genuinely go either way, which is what makes it a test and not a demonstration.

## Hypothesis (falsifiable, with its null)

- **H1:** after an identical pre-registered code-evolution sequence, **arm J** (jigc, with
  `jigc validate`) commits a final state with **fewer stale doc↔code citations** than **arm S**
  (static doc set + frozen conventions, no detector) — because the detector surfaces the drift the
  agent forgot, and the agent repairs it before finalizing.
- **H0 (must be returnable):** the two arms' final states carry a statistically indistinguishable
  number of stale citations — a capable agent keeps docs in sync by hand as well as the tool prompts
  it to, so the detector adds no end-state correctness at this grain/scale. **A tie or arm-S-win is a
  real, reportable result.**

## Honest bounds — carried in from the start, named in the verdict

1. **Structural drift only, never behavioral.** `jigc validate`'s `symbol-exists` knows a cited
   symbol *exists*, not that the prose still *describes what the code does*. A renamed/deleted/moved
   symbol is in scope; a component whose description rots while its symbol survives is **invisible to
   both arms** and is **not scored**. jigc's claim is bounded to structural anchor drift.
2. **Session-compression under-estimates the real effect (the load-bearing bound).** "Over time" in
   the wild means *across sessions, weeks apart, by people who've forgotten the doc exists*. This test
   compresses the sequence into **one focused subagent session** — the most *favorable* case for the
   static arm (everything is in working memory). So: **a tie here does NOT refute the over-time claim
   at true timescale** (the forgetting the detector guards against is suppressed by compression); **a
   jigc win here is a strong result** (it beat the static arm even on the static arm's best day). The
   verdict must state which way the bound cuts.
3. **Subagents ≈ not = human sessions; n = 2 per arm is small; orchestrator built both environments.**
   The judge is a **same-model-family** review agent (independent context + blind + de-identified, but
   not cross-model as the native-grain test's Codex judge was) — acceptable because the metric is a
   mechanical symbol-resolution check, with Codex reserved as the cross-model escalation/verdict pass.
4. **Non-Rust silent-pass is out of frame** — the seed is a Rust project, so every citation is
   genuinely checkable; the polyglot blind spot is named but not exercised here.

## The arms

| | Arm J (jigc) | Arm S (static) |
|---|---|---|
| Doc store | jigc-managed (`decisions/`, `architecture/`, `specs/` + `.jigc/` state, dev pack via `jigc setup`) | identical-content static markdown in the same dirs, no `.jigc/`, no pack, jigc not referenced |
| Detector | `jigc validate` (the M18 store-wide sweep) | none — manual diligence only |
| `CLAUDE.md` | identical minimal project-facts base **+ the jigc adapter** (routes doc work through jigc; advertises `jigc validate`) | identical minimal project-facts base **+ the frozen conventions block** (same doc shape, by hand) + a standing "after code changes, keep the architecture docs' code citations honest" rule |
| Differs **only** in | the doc-authoring + drift-detection **mechanism** | — |

**Symmetry discipline (the fairness crux).** Both arms get the *same* project-facts `CLAUDE.md` base
and the *same* standing instruction to keep docs honest. The frozen-conventions block (arm S) encodes
the same semantic doc shape jigc enforces (no more, no less) — verified before the run. The arms
differ **only** in: arm J has the jigc adapter making `jigc validate` the path of least resistance;
arm S must keep docs honest by reading/grepping. *That asymmetry is precisely the thesis* and is fair
so long as the **intent** ("keep docs honest") is identical.

## The seed — a small, self-contained Rust project (identical content in both twins)

A purpose-built ~4-module Rust library (NOT jigc itself — jigc's 418 MB target and load-bearing
symbols make a controlled mutation sequence impractical). The seed ships:

- **Code:** a small lib with ~6–8 named public symbols across ~3–4 files (e.g. a toy rate-limiter /
  token-bucket: `TokenBucket`, `refill`, `try_acquire`, a `Clock` trait, a test `burst_is_rejected`),
  all real and compiling (`cargo test` green).
- **Committed docs citing that code** (the starting honest state, byte-identical content across twins):
  - one **arch-doc** (`architecture/`) — overview + ≥3 components, each with an `implemented-by`
    anchor to a real symbol;
  - two **adr**s (`decisions/`) — at least one with a `cites-code` anchor to a real symbol;
  - one **spec** (`specs/`) — ≥2 criteria, each with a `maps-to-test` anchor to a real test fn.
  - Every citation resolves clean at the start (arm J: a `jigc validate` on the seed reports zero
    findings; arm S: every cited symbol greps to a real definition).

Arm J's twin is produced by authoring these docs *through jigc* (so the `.jigc/` state + canonical
bytes are real); arm S's twin holds the *same content* as static markdown. The seed-construction
(mine) is symmetric; the arms run blind on top of it.

## The pre-registered code-evolution sequence (identical, ordered, given to both arms)

A list of **plausible code-change tasks** — the *primary* work; doc-honesty is the standing rule, not
the spotlight (this is how drift actually happens). Each step names a code change; **none** says
"update the docs." Frozen composition:

1. **Rename a cited symbol** — rename a documented public fn for clarity (e.g. `try_acquire` →
   `try_take`), update call sites so the build stays green. *(Breaks ≥1 `implemented-by`/`cites-code`.)*
2. **Delete a cited symbol, fold its logic** — remove a documented helper, inline it into its caller.
   *(Breaks the citation that named it.)*
3. **Move a cited symbol to a new module** — relocate a documented struct/fn to a new file.
   *(Breaks the `path#symbol` whose path changed.)*
4. **Rename a cited test** — rename the test fn a spec criterion maps to. *(Breaks a `maps-to-test`.)*
5. **Control step — a change that touches NO cited symbol** — add a new unrelated helper + a small
   internal refactor of an *un*-cited function. *(Breaks nothing; tests over-editing / false flags:
   does either arm spuriously rewrite docs? does `jigc validate` correctly stay quiet?)*

The exact symbol names + edits are fixed in an appendix to this doc before freeze, so both arms get
byte-identical task lists.

## What's measured (judged by the independent review agents on each arm's committed history)

**Primary metric A — final stale-citation count.** For each arm's final committed docs, the judge
checks every doc↔code citation (`implemented-by`, `cites-code`, `maps-to-test`) against the arm's
final code: a citation is **stale** iff its `path#symbol` does not resolve to a real symbol/test in
the final tree. **Lower is more-honest.** (This is exactly what `jigc validate` checks — but the
judge computes it *independently* by parsing the code, not by running jigc, so arm S is scored on the
same objective footing.)

**Primary metric B — the per-step caught-vs-shipped timeline.** Because each step is committed
separately (see Run mechanism), the judge reconstructs, **per step**, for each citation the step's
code change broke: was it **caught** (the citation resolves again by that step's own commit — the
agent noticed and repaired in-step) or **shipped-stale** (left dangling in that commit, surfacing in
a later commit or the final state)? This is an *objective per-commit* check (does `path#symbol`
resolve at commit N?), so it is mechanically reproducible from each twin's git history — it shows
*where* drift entered and whether the detector closed the loop *at the moment of breakage* vs. drift
accumulating. The timeline is the over-time signal the final count alone flattens.

**Secondary / observational (recorded, not the headline):**
- **Did arm J run `jigc validate`** — and did it act on the findings (repair the flagged citations)?
- **Repair friction / capability gaps** — e.g. did arm J hit the M17-surfaced "no committed-field-edit
  verb" gap when repairing a citation? How smooth was the detect→repair loop?
- **Over-editing** — docs changed when no citation broke (step 5); spurious churn.
- **Code integrity** — did either arm leave the build broken / code inconsistent (a confound).

## Run mechanism

- **n = 2 independent blind subagents per arm** (drift is stochastic — one agent may remember docs,
  another forget; two per arm reduces the single-draw lottery). Each subagent: fresh
  `general-purpose` context, given **only** its twin path + the ordered code-task list + "work in
  this repo, follow its `CLAUDE.md`." Blind to the other arm, to the comparison, and to the metric.
  Arm J's environment has `jigc`; arm S's does not and isn't told it exists.
- **One commit per step.** Each subagent is instructed to commit after completing each step (a normal
  dev practice given identically to both arms — arm J via `jigc … finalize`, arm S via a plain git
  commit). This is what makes the per-step caught-vs-shipped timeline reconstructable; it is not a
  doc-honesty hint (the standing rule is the only doc instruction).
- **Twins on real disk** (`/home/maurice/jigc-dogfood/mdt-J{1,2}`, `mdt-S{1,2}`), cloned fresh from
  the seed for each subagent (no cross-run carryover).
- **Pinned binary:** `jigc` sha `00e67f0f…` (the M18 release), `doc-code` sibling `f56032c2…`, both
  at `~/.local/bin/`. Frozen for the whole run; no mid-test rebuild.

## The judge

**Independent review subagents** (not Codex — the human's call at sign-off; Codex is held as
escalation, below). For each arm's run, a **fresh review subagent** that authored none of the
environments and is **blind to arm identity** is given the arm's **de-identified** committed history
(jigc tells — `.jigc/`, tool names, commit trailers — stripped/neutralized) + the final code + the
objective rubric: *(A) for each doc↔code citation in the final state, does its `path#symbol` resolve
in the final code? count the stale ones, with evidence (a confirming grep); (B) at each step-commit,
which broken citations were repaired in-step (caught) vs left dangling (shipped-stale)?*

- **Two review agents score each arm independently**; disagreement on a citation's status is resolved
  by re-checking the objective grep (the count is mechanical, so agreement should be high — divergence
  flags a judging error, not a real ambiguity). The orchestrator de-anonymizes only after scores are in.
- **Why same-model review here is acceptable:** the metric is a *mechanical* symbol-resolution check
  (does `X` exist in the tree), not a subjective quality call, so the cross-model independence the
  native-grain test needed (for a defect-judgment rubric) buys little — a same-context-fresh agent
  grepping the code is sufficient and verifiable.
- **Codex escalation (reserved):** if the two review agents diverge irreconcilably, or the final
  margin between arms is within one citation (too close to call), OR for the **verdict-prose pass**
  (the M17 lesson — green-but-flattering applies to verdicts), escalate to `codex exec
  --sandbox read-only` as the cross-model tie-breaker / verdict reviewer.

## Success criteria (what makes the test *valid*, regardless of outcome)

1. Both arms produced a final committed state from the identical seed + task list.
2. The judge scored stale citations on de-identified final states with evidence.
3. The result is reported **with its bounds** — including a tie / arm-S-win (H0), and including the
   session-compression bound's direction.
4. The verdict distinguishes **structural** drift (tested) from **behavioral** drift (out of frame)
   and does not let "jigc catches drift" overclaim past what `symbol-exists` actually sees.

## Resolved at sign-off (2026-06-13) — freezing the design

1. **Seed** → purpose-built toy Rust project (controllable, unambiguous citations, fast clones).
2. **Replicates** → n = 2 independent blind subagents per arm (4 arm-runs total).
3. **Framing** → doc-honesty is a standing `CLAUDE.md` rule; code-work is the primary per-step task
   (the realistic drift scenario, keeps the comparison refutable).
4. **Metric** → final stale-citation count **and** the per-step caught-vs-shipped timeline (both
   primary).
5. **Judge** → independent same-model-family review subagents (two per arm, blind, de-identified),
   **not** Codex; Codex reserved as the cross-model escalation (irreconcilable divergence, a within-
   one-citation margin, or the verdict-prose pass).

**Frozen at this commit.** The exact symbol names + per-step edits land in `appendix-sequence.md`
(committed with the seed) before any arm runs; deviations after that are timestamped amendments.
