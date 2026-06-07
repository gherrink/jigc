# Self-hosting — distill the harness into a methodology pack, dogfood it on a fresh project

**Status: settled for M12's bounded first slice (2026-06-07); exploratory beyond it.** The project's designed terminus — the workflow docs already call the hand-run loops *"the dogfood for the product workflow we will eventually compose."* De-parked from `ideas/self-hosting.md` (removed) at M12 planning (the M11 `describe`→[introspection.md](introspection.md) precedent). This doc is the locked spec M12 decomposes against; the dialect surface *beyond* the bounded slice is deliberately **discovered by attempting the encode**, not designed here.

Reading-order note: this is the terminus doc — it reads after [worked-examples.md](worked-examples.md), because the methodology pack is the product turned on its own process and presupposes every prior surface.

## The idea (unchanged from the park)

Distill the accumulated harness — the hand-run [milestone-planning](../implementation/milestone-planning-workflow.md) / [increment](../implementation/increment-workflow.md) / [dev](../implementation/dev-workflow.md) / [milestone-completion](../implementation/milestone-completion-workflow.md) workflows + their folded learnings — into a jigc **methodology pack**, then bootstrap a fresh project with jigc *driving* that pack, and measure how it performs. Three moves: **distill** (graduate prose-and-agents-we-run-by-hand into portable, project-agnostic form) · **encode** (workflows as jigc workflow-definitions, gates as probes/steps) · **dogfood + measure** (run a real new project through jigc, compare build-health to the hand-run baseline).

**Why on-thesis:** distilling-into-jigc is a *forcing function*. Every operator-discipline lesson hits a binary when you try to encode it: it either **becomes a mechanical step/probe** (now enforced) or is revealed as **irreducible LLM judgment** (stays the agent's). Both are wins — the exercise sorts the harness into the [determinism boundary](../VISION.md) the product exists to draw. M8 forced a **third** bucket (below).

## The honest caveat (the acceptance bar, load-bearing)

**Distilling-into-jigc ≠ automating judgment.** jigc owns the *structure* (compose the workflow, place every write, walk the deterministic gates, resolve the cascade); the agent fills the prose and the judgment calls. "The methodology becomes a pack" means *jigc orchestrates the loop and the agent fills the judgment slots* — the thesis, not a compromise.

The bar cuts **both ways**, and M12 fails if it crosses either edge:
- **Hollowing a judgment slot** — turning gap-detection / design-review / settle (or the "question-the-docs-you-lean-on" and "forward-difficulty-is-a-claim" folds) into a checklist probe. Looks like more automation; actually destroys the judgment.
- **Prose-washing a mechanizable structure** — handing a genuinely *mechanizable* constraint back to the agent as un-enforced prose and calling it a faithful encode. The dev-workflow's **test-first ordering** (red-before-green) is *structure*, not judgment; a flat encode that emits it as prose has prose-washed it (see the three-way sort).

A faithful encode keeps each lesson on the side the forcing-function actually sorts it to — not the side that flatters the milestone.

## The three-way determinism cut (M12's central forcing function)

Every harness lesson sorts into exactly one of three buckets. M12 *does this sort for real* on the dev-workflow's steps:

1. **Mechanizable step** → becomes a probe / lint / structural gate. *Enforced, no longer fragile.* (M7's determinism-test obligations were the first clean example.)
2. **Irreducible judgment** → stays the agent's. A model-free composer cannot resolve a genuine design fork; the halt-on-fork is preserved, never linted away. (M8's inc-5 fork-refusal.)
3. **Owner-assigned recorded artifact** → real but un-automatable because it lives on the LLM/assistant side of the boundary: a named owner runs it, records a blocking artifact, and the automated half declares *"I did not run this."* (M8's genuine-spawn proof — a headless subagent cannot spawn the Task tool.)

The third bucket is forced by the determinism boundary recursing onto the harness's own proofs. A distilled methodology pack must be able to compose a gate of each shape — not just probes and judgment slots.

## The dev-workflow sort (done for real — the executable graduation spec)

M12's bounded first slice encodes **the dev-workflow only** ([dev-workflow.md](../implementation/dev-workflow.md): scope → red → green → refactor → gate → commit). Verified at planning (HEAD `8929f7d`): the dev-workflow *proper* is a **flat, single-pass linear sequence** — each step runs once. The bounded-loop / halt-resume / conditional primitives the roadmap feared live in the **increment-workflow wrapper**, *not* the dev-workflow. So the reduced-linear encode runs **today, with zero dialect extension** (spiked end-to-end against the real binary on a non-Rust repo — see Acceptance flow).

Each dev-workflow step, sorted:

| Step | Sort | Encoded as | Honest note |
|---|---|---|---|
| **Scope** — restate intent + observable done-criterion | judgment | prose step (embeds `{{task.intent}}`) | the restatement is irreducibly the agent's |
| **Scope** — "stop and check" if scope drifted | judgment (lightweight human-touch) | prose step | a structural **human-gate** step kind is absent — degraded to prose for the dev-workflow (it only needs the prose form); the *structural* human-gate is a named dialect-extension trigger |
| **Red** — test-first, fail-for-the-right-reason | **mechanizable (ordering) — but unencodable today** | prose step | red-before-green is *structure*, not judgment; the dialect has no ordering/assertion gate, so it is **prose-washed** in the first slice. **This is the headline named trigger** — the one primitive that would convert it to structure |
| **Green** — minimal implementation | judgment | prose step + `<<author>>` slots | — |
| **Refactor** — tidy while green | judgment | prose step | — |
| **Gate** — `fmt`/`clippy`/`test`/`build` all pass | **partly mechanizable** (see note) | **prose** ("run your project's configured test+lint+build gate") | two stacked reasons, with different sorts: (i) the literal `cargo` commands are jigc's own stack — a portable pack can't hardcode them, *solvable* by a **gate-command knob** (bucket-1, named trigger #3); (ii) there is no jigc verb to *run* a foreign process and the product may never want one — so "the agent runs the project's own gate" is plausibly **irreducibly the agent's job** (bucket-2). The knob mechanizes *which command*; it does not mechanize *jigc shelling out to run it*. Degraded to prose either way |
| **Commit** — one logical commit | **mechanizable — built** | `finalize` step (`{{cli.finalize-task}}` + commit doctype) | the one step that *is* genuinely structural today: finalize renders the commit doctype to the git message and lands exactly one commit, git-only, no cargo |

**What this proves and what it does not.** The reduced-linear encode proves **workflow-composition fidelity** — jigc composes the dev-workflow's *structure* deterministically and runs a real task end-to-end, the agent filling the judgment slots. It does **not** prove jigc owns the dev-workflow's *characteristic* discipline (test-first ordering, the mechanized gate) — those degrade to prose. **That degradation is not a failure to hide; it is the milestone's primary finding** — the measured shape of the dialect-extension milestone M12 surfaces (below). Recording it honestly *is* the graduation pass's "enforced-vs-trusted" sort, done for real.

**The honest M1 delta (don't over-read "self-hosting").** The reduced dev-task is *structurally the same shape* as M1's `single-task` — a flat `{{include}}` spine, `{{task.intent}}` resolution, a `commit` doctype, finalize-to-git. So M12 adds **no new engine behaviour and no new workflow structure** over M1. Its genuinely-new proofs are narrow and worth naming exactly: (a) a **sole `JIGC_PACK_DIR` pack that is *not* the embedded dev pack** composes and runs (the subsume / compose-alone proof); (b) it runs on a **non-Rust** repo, git-only (portability); (c) the **graduation sort is performed for real** and the dialect-extension triggers are named (a design artifact, not a runtime proof). "Composition fidelity" is the headline, but it largely *re-exercises M1's machinery on a foreign pack + a foreign project* — that is the milestone's honest worth, not new structural ownership.

## The four-part graduation pass (the Distill move, scoped to the dev-workflow)

A one-time pass over the harness, here executed for the dev-workflow only (the wider harness folds back later):
1. **Portability** — each step reads principle-first, not jigc-coupled. The concrete proof: the gate step must *not* hardcode `cargo` (it is portable prose); step ids stay single-word (the multi-word section-id defect).
2. **Rationale index** — each gate ↔ the failure class it prevents. For the dev-workflow: the gate-commands prevent "green that doesn't compile/lint/test"; test-first prevents "a vague task with no red step" / implementation-without-a-falsifiable-target.
3. **Enforced-vs-trusted** — the sort table above *is* this part: Commit is enforced (structural); Gate + Red are *trusted* today (prose) and named as the mechanization candidates.
4. **One forward-looking adversarial review of the process itself** — repeatable per-novel-milestone (M7 confirmed it pays off *before* the build). For the dev-workflow encode the forward review *is* the dialect-extension naming: "what does a faithful encode of the *next* workflow (increment/completion) force that this one routed around?" — answer: the bounded fix-loop, the hard halts, the owner-artifact gate.

## The named dialect-extension trigger (the expected M12 outcome)

M12 is **exploratory**: surfacing a dedicated workflow-dialect-extension milestone is an acceptable, expected result — not a failure. The bounded slice routes around the missing surface by degrading to prose and **naming** what a faithful encode needs, ranked by leverage:

1. **A mechanizable ordering/assertion gate** (assert "a test was observed failing before the implementation") — the single primitive that converts the dev-workflow's test-first discipline from prose → structure. Highest value; the dev-workflow's defining structure.
2. **A structural human-gate / checkpoint step kind** — the most-reused missing primitive across the *wider* methodology (planning's Settle, increment's halts). The dev-workflow needs only its prose form.
3. **A portable gate-command knob** — lets a pack express *which* gate command structurally instead of hardcoding a stack. Bounded leverage: it mechanizes the *command string*, not the *running* of a foreign process — "the agent runs the project's own gate" stays the agent's job (bucket-2). So this trigger is real but smaller than #1/#2.
4. **A bounded fix-loop primitive** + **halt→re-run resumption** — needed by the *increment-workflow* encode (out of the dev-workflow's scope); pairs with the `(D)` resumption decision and the never-started-fan-out backstop ([decisions-pending.md](../implementation/decisions-pending.md)).
5. **An owner-assigned-artifact gate step** — the third-bucket shape; needed by the *milestone-completion* encode (out of scope for the dev-workflow).

These are **named, not built** in M12. Conditional steps stay *barred* by the workflow-dialect task-independence invariant — the harness's genuinely conditional structure (the novelty-keyed forward review, the findings-keyed fix phase) resolves to **agent-judgment prose**, never a dialect step; that is the correct sort, not a gap.

## Seams M12 must not cross

- **M13 seam — the methodology's own working docs stay plain markdown.** The encoded workflow *reads* the roadmap / deferral-ledger / decisions-log at fixed paths; it does **not** manage them as doctypes (no jigc-placed writes there). The dev-workflow only *reads* such docs, so it needs no managed methodology doctype. Graduating them to doctypes is M13 (a fold-back can't precede the thing it enables). **Consequence, read honestly:** M12 measures *workflow-composition fidelity*, not self-managed methodology docs.
- **M14 seam — the methodology pack composes alone.** M12's dogfood runs the methodology pack as the **sole composed pack**; it **subsumes** the dev surfaces it needs (see the subsume decision). True dev + methodology multi-pack co-composition is M14's distinct proof. This breaks the M12↔M14 circularity (M14 needs the pack M12 mints, so M12 must not need M14's composition to run).

## Settled planning forks (2026-06-07)

- **Encode scope → reduced-linear only.** Encode the flat prose spine; name the test-first/gate degradation as the dialect-extension trigger. **Zero engine/dialect code in M12** — pure pack authoring + design + dogfood.
- **Subsume vs thin → subsume.** The baseline kills "thin": any `creates-task: true` pack must vendor a `commit` schema (finalize hardcodes `COMMIT_TYPE`) and reproduce the full intrinsic-knob severity surface floored at blocking. A truly-thin pack needs engine changes M12 won't make. So the methodology pack **vendors** `commit` (+ the cascade config + the dev steps) and runs alone. **Build note (verified):** "reproduce the knob surface" is *copy-and-edit*, not copy — the embedded `knobs.yaml` hardcodes `default-workflow: { of: [router, single-task, …], default: router }`, and the knob enum **constrains** the `defaults.yaml` value, so the pack must rewrite `of:`/`default:` to its own workflow id(s) (e.g. `of: [dev-task], default: dev-task`) or `start` fails `no workflow 'router'`.
- **Which workflow first → the dev-workflow** (verified simplest: flat single-pass).
- **Success bar → composition-fidelity + qualitative judgment check** (below). The quantitative build-health comparison is **deferred** — its tally lives in the external `milestone-build.js` orchestration harness, not the binary or the pack, so it has no runnable home on a foreign-project dogfood. Defer rather than build measurement scaffolding (that would balloon the bounded slice).
- **`(D)` workflow progress/resumption → defers cleanly.** Under reduced-linear, the existing restart-from-scratch policy holds (matching the fan-out resumption policy); the structural primitive is named (trigger #4) for the dialect-extension milestone, not built. Graduated out of [decisions-pending.md](../implementation/decisions-pending.md) → "Before planning M12".
- **Dogfood target → `gherrink-galey`** (a TypeScript pnpm monorepo — genuinely unlike jigc, so jigc-shaped gates can't flatter it; has planned-but-unbuilt work = a real dev task). `gherrink-lacon` (Rust) is the fallback. **Constraint: never touch the original** — the dogfood runs on a local `/tmp` copy.

## The dogfood — acceptance flow (spiked against the real binary, 2026-06-07)

Verified end-to-end on a fresh non-Rust git repo via `JIGC_PACK_DIR`. The flow (full walk in [worked-examples.md](worked-examples.md) → flow 15):

0. **`jigc setup`** (step 0 — `start` hard-fails without a `.jigc/config/` cascade layer). Commit its adapter artifacts *before* the work task, so the work commit is code-only.
1. **`jigc start "<intent>"`** composes the methodology `dev-task` workflow → emits the flat prose spine with `{{task.intent}}` resolved. Deterministic: recomposing via `start --task <id>` is byte-identical.
2. The agent works the task on the foreign project — runs the project's own gate (prose-instructed), self-polices test-first.
3. **`jigc task finalize <id>`** — fills the commit slots (`type`/`scope`/`body` + `summary`, step-prose-instructed), renders the commit doctype to the git message, lands **exactly one** git commit. Git-only — no cargo.

**Verified friction (build-time knowledge — all confirmed against the real binary at planning):**
- **`setup` is mandatory step-0** and honors `JIGC_PACK_DIR` (`start` hard-fails without a `.jigc/config/` layer).
- **finalize *renders*, it does not *fill*.** The vendored `commit` schema requires `type`/`scope`/`body` **non-empty** (`scope` is a required `string` field — rejected empty). The agent must run, *before* finalize: `jigc doc set-field <commit>#type --value <t>`, `… set-field <commit>#scope --value <s>`, `… set-slot <commit>#body` (and `#summary`). Note `set-field --value` (for `type`/`scope`) is a **different verb** than `set-slot` (for `body`/`summary`). The methodology `finalize` step must therefore wire command-refs (or spell the literal invocations) for **all four**, not just `summary` as the embedded pack does — *not* an engine change; the optional-field capability stays deferred.
- **Placeholders resolve ONLY as a lone line.** A `{{task.intent}}` / `{{cli.…}}` placeholder resolves only when it is the *entire trimmed line*; inline-in-a-sentence emits the literal `{{…}}` — and a "deterministically broken" compose still passes the byte-identical determinism bar. Every authored step must put each placeholder on its own line (the embedded `locate.yaml` does this; the notation in this doc is illustrative, not line-faithful).
- **Determinism via `--task` recompose** — capture the stdout of the *original successful* `start`, then `start --task <id>` and diff; re-minting the same intent correctly fails "already active" (don't diff against a re-mint's error output).
- **Sequence setup's commit ahead of the work commit** for a clean code-only commit.

## What "done / good" looks like (the success bar)

The bounded slice succeeds when, on a `/tmp` copy of the dogfood project:
1. **Composition fidelity** — the methodology pack composes the dev-workflow **deterministically** (same pack + cascade → byte-identical workflow) and **runs a real task end-to-end through the binary** (`setup → start → finalize → one commit`), git-only, on a non-Rust project.
2. **Judgment slots genuinely exercised, not hollowed** — scope-restatement, test-first discipline, and the gate stay agent-authored prose; the encode mechanized **nothing** that is judgment. (The qualitative check: read the encoded steps and confirm each judgment slot is still the agent's, and each *prose-washed mechanizable* one is **named** as a trigger, not silently dropped.) The bar has a **falsifiable edge** — two concrete FAIL exemplars: (a) *hollowing* — the `scope` step emits a `gap-count ≥ N` lint or a structured checklist the agent must satisfy (it has mechanized the judgment of "is this scoped right" → **FAIL**); (b) *prose-washing-without-naming* — the encode ships the test-first ordering as prose but the milestone record does **not** name it as a dialect-extension trigger, presenting the flat encode as a faithful TDD loop (→ **FAIL**). An encode that triggers neither, and whose every prose-washed mechanizable appears in the named-trigger list, passes.
3. **The dialect-extension milestone is named honestly** — the test-first-ordering + gate + human-gate + fix-loop + owner-artifact triggers are recorded with the failure-class each would prevent, ready to scope as their own milestone.

The quantitative "matches/beats the hand-run build-health trend" is **explicitly out of this slice** (no runnable home; deferred).

## Honest risks this idea still carries

- **Reactive-only hardening** — a fresh project inherits our post-mortems but learns *its* novel failure the hard way unless the forward review (part 4) probes first. Run it per-novel-milestone.
- **A clean run is ambiguous evidence** — "jigc ran it clean" is not "jigc ran it well" without a hard milestone to stress it. The composition-fidelity bar mitigates this (it asks "did it run + stay honest," not "was the run easy").
- **The dogfood proves fidelity, not the mechanized gate** — by the reduced-linear choice, the gate + doc↔code stay prose/off on a foreign (non-Rust) project. That is the honest bound, stated, not hidden.

## What the M12 run added (2026-06-07) — the two-half dogfood pattern, for the next encode

The first slice ran clean (0 halts / 0 fix-rounds), and the build surfaced one reusable structural lesson the *next* workflow-encode (increment / completion — the dialect-extension milestone) will need:

- **The dogfood splits into two honestly-different halves, and conflating them is the trap.** **Half A — the automatable regression gate:** the deterministic CLI walk (`setup → compose → byte-identical recompose → fill → finalize-lands-one-commit`) over a `/tmp` copy with *fixed inputs* — a `cargo test` a headless validator can re-run. **Half B — the recorded owner-artifact:** the genuine live-agent run exercising the *judgment* slots (real failing test observed before impl, the project's own gate run by hand), which by the determinism boundary a headless subagent can no more author than it can spawn the Task tool — so it is a **recorded milestone-completion artifact** (the bucket-3 shape), *not* an increment-loop test. The **hollow-dogfood trap** is passing Half A's fixed-input walk off as the genuine run — the sibling of M8's hollow-spawn trap. The milestone is not shippable until the Half-B artifact exists. This split is the direct analogue of flow 10's Half-A/Half-B (automated determinism gate vs recorded genuine spawn); the next encode's dogfood reuses it verbatim.
- **Vendoring a doctype inherits its required-field gates.** Subsuming `commit` meant the workflow's finalize step had to satisfy every required field (`type`/`scope`/`body`) the schema enforces — a happy-path spike misses this; spike the *required-field/error* paths of any vendored doctype, and bake what you learn into the acceptance flow's walk, not just a prose note (the design-review re-spike caught exactly this gap here).

## Open threads (beyond the bounded slice)

- **Where the line falls** between "jigc composes the *structure* of a judgment step" and "the step is pure agent judgment jigc only points at" — the encode of the *next* workflow (increment/completion) will surface concrete cases.
- **The dialect-extension surface** — the five named triggers above, to be shaped into a dedicated milestone when their joint shape is clear.
- **Whether a doctype must promote into M12** — if a faithful dogfood turns out to need the Scope step to *mechanically* read a *managed* deferral-ledger (vs plain markdown), that one doctype promotes from M13 into M12 (the roadmap escape hatch). The dev-workflow as scoped does **not** trip this (it only reads, at fixed paths).
- **The Rust-only `doc↔code` probe** — if a later dogfood wants to *prove* doc↔code on a foreign project, the M10 probe is Rust-grammar-only and the build-tree-sibling placement won't survive `cargo install`; both are `unverified-reuse` for a non-Rust target. Out of scope for the reduced slice (the methodology pack ships no `code-anchor`, so the probe is gated off).
