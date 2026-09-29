# M19 handover — next session: the thesis is thrice-H0; decide whether to chase the win-regime or accept scope-bound and pivot

**Written 2026-06-13.** Branch `main`, tree clean, gate green (fmt · clippy -D · 427 lib tests +
suites · build). HEAD `1c38a93`. Pinned binary `~/.local/bin/jigc` sha `5d64adc…` (M19 release —
hook-installing + finalize-relaying), `doc-code` sibling beside it. **M19 is fully closed** (built,
audited, tested, recorded). This handover is the running start for whatever comes next — and the
honest truth is that **the next move is a strategic decision, not a foregone task.**

## Read first — where the founding thesis actually stands (don't re-derive this)

The founding thesis is *jigc makes a coding agent measurably more correct/effective than a static
`CLAUDE.md`*. As of M19 it has been tested **three times, all H0** (no demonstrated comparative
advantage) — and the understanding has *sharpened* each time, which is the real value:

1. **M17 (pilot + native-grain).** A capable model **hand-structures correctly from good static
   conventions** — diligence ties jigc's machine-guarantee at authoring grain.
   ([../M17/VERDICT.md](../M17/VERDICT.md), [../M17/native-grain-test/](../M17/native-grain-test/))
2. **M18 (maintenance/drift, on-demand detector).** The detector **never fired** — single-session
   diligence kept docs honest, so the over-time advantage went untested; H0 under a session-compression
   bound. Also surfaced the **behavioral blind spot** (validate green over drifted prose).
   ([../M18/maintenance-test/results.md](../M18/maintenance-test/results.md))
3. **M19 (auto-firing backstop, genuine cross-session forgetting).** The backstop **fired in anger and
   drove repair** (cold agents missed citations, the warn-only sweep surfaced the drift, a later cold
   agent fixed it — crediting `jigc validate` in the commit messages) — the loop M18 never exercised.
   **But:** the **static arm stayed clean at every commit** (right-first-try), while the **jigc arm was
   the only one that drifted** and cleaned up. End-state tie (0 stale, all 6 chains); neither a
   comparative end-state nor timeline win. ([cross-session-test/results.md](cross-session-test/results.md))

**The pattern that matters:** jigc reaches a clean end-state via *drift→detect→repair*; a capable agent
reaches it via *right-first-try* diligence. **They tie because diligence keeps not-failing.** jigc's
backstop earns a *comparative* win only when first-try diligence leaves **residual** drift in the final
state — and across three tests, at the scales tested, **it never has.** The architectural thesis
(structure→CLI, prose→LLM) is **built and proven**; the **empirical superiority claim is undemonstrated
and scope-bound** — now stated honestly in [VISION.md](../../VISION.md) → "What is proven (M17–M19)".

## The strategic fork the next session owns (surface it to the human first)

Don't default into "run another test." Three H0s is itself a finding. The genuine options:

- **(A) Chase the win-regime — one more, harder comparative test.** The owed regime (named in the M19
  verdict): a setting where first-try diligence **predictably leaves residual drift** the detector
  catches and the control ships. Concretely — a **far larger/denser citation graph** (dozens of
  citation sites per symbol, many docs, so a cold agent reliably misses *some*), and/or **truly
  separated sessions** (the agent changing code is *not* the one who knows the docs — a real
  multi-author / weeks-apart split, which cold subagents in one orchestration only approximate). This
  is the honest shot at converting the backstop's *demonstrated loop-closure* into a *comparative win*.
  Risk: it may tie again (capable agents are stubbornly diligent), and it's expensive.
- **(B) Accept scope-bound; pivot off the comparative-superiority chase.** Three controlled tests say
  the value isn't a measurable correctness delta over a capable agent + good static conventions at
  these scales. The honest product story may not be "more correct" but something else already proven:
  *deterministic structure*, *machine-guaranteed integrity* (can't dangle past the gate — even if
  diligence also gets there), *the detect→repair loop that demonstrably closes*, *living docs that
  can't silently rot once the backstop is installed*. Pivot to **hardening/shipping what's built**
  (the four-target `jigc validate` envelope, the `cargo install` probe-distribution gap, multi-assistant
  profiles, the deferred product gaps) and **reframe the value claim** to what the evidence supports.
- **(C) Attack the behavioral blind spot** (M18's finding) — the gap where *every* test showed jigc is
  weak: `symbol-exists` is green while prose lies. This is a genuinely **unbuilt** capability, not a
  re-test of a tied one. But it's hard (it needs semantic/LLM judgment, which the CLI-core can't do
  without crossing the no-LLM-in-core invariant — likely an *agent*-driven check the CLI composes, not
  an engine probe). Could be the most honest "what would actually make docs more trustworthy" direction.

**Recommendation to put to the human:** lead with the **standing** of the thesis (B's framing is the
honest baseline), and offer A as a bounded, pre-registered last attempt *only if* the human wants the
comparative win badly enough to fund a denser/multi-author regime — with eyes open that it may tie a
fourth time. C is the most novel and the least re-tread, if the appetite is for building over proving.
This is the human's call; don't pick it silently.

## What M19 shipped (the built surface, for whoever builds next)

- **`jigc validate`** (M18) — store-wide, read-only, report-only doc↔code sweep; content findings exit
  0, probe-integrity exits non-zero; the four-target VISION envelope is **named but only doc↔code is
  built**. ([../../design/validation.md](../../design/validation.md) → Store-scope re-validation.)
- **The auto-firing backstop** (M19) — `jigc setup` installs an **assistant-neutral, warn-only**
  `.git/hooks/pre-commit` running the sweep (keys on `doc-code` content findings via `--format json`,
  **not** the exit code; silent + exit 0 on clean/not-a-project/probe-missing; idempotent;
  non-destructive; honors `core.hooksPath` + worktrees). It's an **install artifact**, not a managed
  doc. ([../../design/assistant-adapter.md](../../design/assistant-adapter.md) → neutral install.)
- **finalize success-relay** (M19) — `git_commit` relays hook stdout/stderr to the agent **on success**
  (delimited, not in the footer/JSON envelope; aggregate-commit-only in fan-out). Resolved the
  `finalize.md` "commit-msg hook output capture" open question.
- **No new doctypes**; pure integration over the shipped sweep.

## Reusable assets (don't rebuild)

- **The M19 seed twins** at `/home/maurice/jigc-dogfood/mdt19-seed-{J,S}` — the **extended** `ratelimit`
  toy project (16 citations across an 8-component arch-doc + 4 ADRs + 4-criterion spec; J set up *with*
  the backstop, S static). The chains `mdt19-{J,S}{1,2,3}` are the run outputs (mutated). For a denser
  test (option A) extend this seed further; the construction pattern is in
  [cross-session-test/appendix-sequence.md](cross-session-test/appendix-sequence.md).
- **The M18 seed** at `mdt-seed-{J,S}` (the original 9-citation ratelimit) — the lighter base.
- **The pre-registration + appendix templates** at [cross-session-test/](cross-session-test/) and
  [../M18/maintenance-test/](../M18/maintenance-test/) — falsifiable H1/H0, honest-bounds-up-front,
  build-contract-vs-test-hypothesis split, no-cue slips, net-value scoring.
- **The orchestration recipe (proven this session):** clone N twins → a **Workflow** runs the chains
  (cold blind subagent per step, sequential within / parallel across — each `agent()` call is a fresh
  cold context) → score **mechanically** from git history (per-commit citation resolution; **use a
  correct resolver** — see the trap below) → one **independent review subagent** re-verifies + challenges
  → **Codex-pass the verdict** prose. The workflow script is at the session's workflow scripts dir.
- **The milestone-plan → milestone-build → test-half loop** is now well-grooved (M18 + M19 both ran it
  clean); reuse it for any new milestone.

## Mechanics to re-establish

1. **Binary.** `cargo build --release`; copy `target/release/{jigc,doc-code}` to `~/.local/bin/`.
   Current pin `5d64adc…`. **Re-pin after any build-half change, between build and test** (a mid-test
   binary change contaminates).
2. **Twins on real disk** (`/home/maurice/jigc-dogfood/`); `/tmp` is a small tmpfs — too small for
   `target/`. The dev pack is **embedded** — `jigc setup` with **no `JIGC_PACK_DIR`**. Re-running
   `setup` with a newer binary adds the new install steps (e.g. the hook) over existing docs —
   idempotent, non-destructive (verified).
3. **Cold-per-step = genuine forgetting.** A fresh `general-purpose` subagent per step, given **only**
   its step's pure-code task + the twin + "follow CLAUDE.md, commit when done." Blind to the comparison,
   the metric, and the docs being the point. Bound: cold subagents ≈ but ≠ real separated human sessions.
4. **Symmetry discipline (the fairness crux).** Identical seed + identical project-facts `CLAUDE.md` +
   identical "keep citations honest" standing rule in both arms; they differ ONLY in mechanism (jigc
   adapter + backstop vs frozen-conventions + no jigc). Verify code byte-identical + citation sets
   identical before any run.

## Traps (paid for this session — don't re-pay)

1. **The mechanical scorer's resolver bug bit twice.** A naïve "symbol in file" check, or a regex with
   a literal `{}` placeholder, gives **every citation stale at every commit** (M18 first pass) — a
   glaring tell it's broken (the *seed* must score clean). Use
   `\b(fn|struct|trait|enum|type|const|static|mod|macro_rules!)\s+<symbol>\b` and **sanity-check against
   the known-clean seed** before trusting any timeline.
2. **The verdict will flatter jigc by omission.** Both M18 and M19 verdicts' first drafts were "too
   kind" — M19's buried that *jigc's own arm was the only one that drifted* behind a rhetorically
   symmetric "different paths" framing. **Codex-pass every verdict** and foreground the result that's
   *bad* for jigc, in the same sentence as the good one.
3. **Don't let "the backstop fired" become "jigc won."** It fired and drove repair (real, M18 lacked
   it) — but on **jigc's own drift**, to a tie. Loop-closure ≠ comparative advantage. The independent
   re-verifier earned its keep catching this; keep using one.
4. **Capable agents are stubbornly diligent** — the recurring reason every test ties. Cold-per-step +
   no-cue *still* didn't make the static arm slip. Any option-A re-test must make diligence
   *predictably* fail (scale / density / true session separation) or it will tie a fourth time. Don't run it
   expecting a win by default.
5. **The warn-only backstop is advisory, not enforced** — the loop closes only when an agent *heeds*
   the warning (ergonomic-not-sandboxed, [VISION.md](../../VISION.md) principle #3). Don't claim
   enforcement.

## State at handoff

- `main` clean, gate green, HEAD `1c38a93`. M19's commits span the planning (`16c0c2e`-era forward),
  build (`211dce8…3990b54`), and test-half/record commits through `1c38a93`.
- **Environment normal:** `jigc` un-wrapped real binary (`5d64adc`) + `doc-code` sibling at
  `~/.local/bin/`; claude-mem on; global `~/.claude/CLAUDE.md` intact. No rollback owed.
- **M19 fully closed:** plan + design-review + build + audit (3 LOW fixed) + the cross-session test
  (H0, Codex-passed) — all recorded in DECISIONS, VISION (M17–M19), and the roadmap M19 Outcome.
- **Deferred product gaps still open** (need-driven, in [decisions-pending.md](../../implementation/decisions-pending.md)):
  the `cargo install` probe-distribution gap; the non-`.rs` silent-pass; multi-valued code-anchor (guard
  only); the other three `jigc validate` targets (override↔default / file↔CLI-state / workflow↔refs);
  multi-assistant adapter profiles; and the **behavioral blind spot** (option C) is unbuilt.
- **No milestone is currently scoped beyond M19.** The next milestone is created only after the human
  settles the strategic fork above.
