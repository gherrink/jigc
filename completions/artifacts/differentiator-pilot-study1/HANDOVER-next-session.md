# Handover — next session: improve the workflows (small → big)

Written 2026-06-22. HEAD = `3205968` on `main`, tree clean. The differentiator
pilot (study 1) is complete and committed; this session turns its findings into
**workflow improvements**, sequenced small→big so each step validates a foundation
before the next builds on it. Nothing here re-opens the pilot verdict — it acts on it.

## The one finding everything below rests on

The pilot proved (n=16/arm, Sonnet) that **jigc's doc↔code enforcement is opt-out-able
by workflow choice**, so it never differentiates from a static instruction:

- drift-rates: **plain 88% · jigc 19% · jigc-gate 0% · static 0% · nudge 0%**
- the agent ran a jigc command in **0/32** bare-task jigc runs (it edits directly);
- when *nudged* to engage, Sonnet picks **`quick-fix` 6/6** — and **`quick-fix`
  finalize does NOT validate doc↔code** (adversarially proven: a dangling arch-doc
  anchor finalized **clean** under quick-fix). So jigc's validation never fired in
  any arm; every "clean" result came from an *instruction*, not the mechanism.

Root insight: the taxonomy axis (*"is there a decision to record?"* → quick-fix) is
**orthogonal** to the safety axis (*"does this touch documented code?"*). Fix the
safety axis directly — don't try to make the agent reason onto it.

Full detail: `REPLICATION.md`, `WHY-SONNET-BYPASSES.md`, `NUDGE-ARM.md`, `VERDICT.md`.

## The mission: the three workflow improvements (your call, 2026-06-22)

1. **Descriptions** — hygiene only (instruction lever; proven weak for the weaker model).
2. **Universal finalize floor** — the robust fix: doc↔code as a repo invariant *every*
   workflow enforces at its commit boundary.
3. **Behavioral testing of workflows** — the standing gap: workflows were golden-tested
   for *structure*, never for *agent behavior* (select / engage / complete).

Recorded in DECISIONS.md (2026-06-22 entry). Sequence them small→big:

---

## Phase 0 — Demonstrate the floor works (smallest; no code change; de-risks Phase 2)

**Goal:** show, deterministically, that a *workflow-agnostic* check converts a
would-be-drift into a caught/fixed one — i.e. that the floor concept is sound before
building it into jigc-core.

**Do:** a side-by-side seeded-drift test in a jigc twin (use `pilot-jigcgate` +
`pilot-jigc`):
- Drive the exact Sonnet path — `jigc start --workflow quick-fix`, rename the code
  leaving the committed arch-doc anchor dangling, `git add`, `jigc task finalize`.
- Confirm the **hole**: quick-fix finalize commits **clean** (already shown once —
  reproduce it as the baseline).
- Confirm the **floor closes it**: the workflow-agnostic Stop-hook gate
  (`pilot-jigcgate`) **blocks** the same scenario at session-stop (or run `jigc
  validate` to show the store-wide check fires). 
- *(Optional, if budget:)* a handful more `jigcgate` reps to catch an *organic*
  tail drift and watch the gate fire end-to-end (the replication hit 0/16 by
  variance — the gate is sound but rarely triggered on this small task).

**Success:** a recorded side-by-side — quick-fix-alone ships the dangling anchor;
workflow-agnostic-check blocks it. That is the empirical case for Phase 2.

**Harness:** everything is in `~/diff-pilot/` — `run-rep.sh` (copy its docker
invocation verbatim for any ad-hoc run), images `pilot-jigc` / `pilot-jigcgate`.
**Read `RUNBOOK.md` gotchas first** — esp. `--permission-mode bypassPermissions`
(or jigc gets permission-blocked) and pass prompts via `-e PILOT_PROMPT` (backticks).

---

## Phase 1 — Descriptions hygiene (small; cheap; independent)

**Goal:** sharpen `quick-fix` + the router/selection guidance so the *taxonomy* hints
at the *safety* axis — e.g. quick-fix "for changes that touch no documented public
API." Helps Opus/humans; **not** the safety mechanism (don't over-invest).

**Do:** edit the relevant pack workflow front-matter / descriptions in
`crates/cli/pack/workflows/*.yaml` (+ the router selection guidance). This is a
content tweak, but it lives in jigc-core, so gate it with `cargo test` + the dev
workflow (`fmt`/`clippy`).

**Success:** descriptions updated; `describe` output reads sensibly; tests green.
**Honest framing:** mark in DECISIONS that this is hygiene, measured value ≈ 0 for
the weaker model (per the pilot) — it's not the fix.

---

## Phase 2 — Universal finalize floor (the real fix; jigc-core; DESIGN-WORKFLOW)

**Goal:** make doc↔code validation a **commit-boundary invariant for every
workflow**, so workflow-misselection (and the quick-fix hole) cannot bypass it.

**This is a core behavior change touching the determinism boundary — run it through
the design-workflow** (`implementation/design-workflow.md`: discuss → converge →
decide → record), then the dev-workflow (test-first). Do NOT just code it.

**Design questions to settle first (the discussion):**
- Does *every* `finalize` run the store-wide doc↔code anchor check, or only over
  anchors referencing code the task changed? (correctness vs perf on a big store)
- Blocking vs advisory? (Blocking = the floor; that's the point.)
- Layer: jigc-core `finalize` (holds for any adapter) is the principled home;
  keep the adapter backstops too — make the **warn-only pre-commit hook blocking**
  (catches the total-bypass/git-commit path the Stop-gate covers in the adapter).
- Interaction with the OOB/reconciliation path (a correct direct edit to a managed
  doc shouldn't be pure friction — see the A′ task2-opus note in ADDENDUM).
- Does this change VISION's determinism-boundary statement? (Probably: "finalize
  guarantees doc↔code integrity" becomes a stated invariant.)

**Build (after the design is settled):** test-first — a failing test that a quick-fix
rename dangling an anchor is **blocked** at finalize; then the implementation; then
re-run Phase 0's demo through the real binary to confirm the hole is closed in core
(not just via the hook).

**Success:** the Phase-0 seeded-drift scenario blocks at finalize **under every
workflow including quick-fix**, with golden/unit tests; VISION/DECISIONS updated.

---

## Phase 3 — Behavioral workflow-eval harness (biggest; standing practice)

**Goal:** generalize the pilot's container harness into a reusable
**agent-in-the-loop** evaluation: given a task, does a real agent *select*, *engage*,
and *complete* the right workflow? "The CLI composes it correctly" ≠ "an agent uses
it correctly" — the pilot is the first such test and it failed all three.

**Do (scope to taste — this is infra, can be staged):** lift `run-rep.sh` + the
drift/engagement analyzers into a small harness that runs a workflow against a twin
on a pinned model and reports select-rate / engage-rate / complete-rate / outcome.
Make it re-runnable per workflow. This is the seed of jigc's missing behavioral test
layer; it also becomes the measurement substrate for the long-horizon study.

**Success:** one workflow (e.g. dev-task or the new floor) has an automated
behavioral eval that a future session can re-run; documented as the pattern.

---

## Explicitly OUT of scope next session (deferred, bigger, later)

- **The long-horizon many-edit study** — still the only path to proving jigc
  *superiority* over a static file (not just parity). It needs Phase 2 + Phase 3
  done first (a working floor + a behavioral harness to measure over many edits).
  Do it *after* this foundation.
- **Productive corpus / self-hosting** — unchanged: do not. M33 + M34 + a *cleared*
  value gate still gate it, and the value gate is not cleared.

## Environment / state notes

- **Commits:** the pilot series sits on `main`, **unpushed** (HEAD `3205968`). Push
  when ready.
- **Workspace:** `~/diff-pilot/` (throwaway) — clones, Dockerfiles, run scripts,
  `runs-rep/`. Images: `pilot-{toolchain,deps,jigc,jigcgate,jigcnudge,static,bigstatic,jigcenforced,jigcproactive}` (~4 GB). All disposable; `RUNBOOK.md` reproduces them.
- **jigc binary under test:** `jigc 0.0.0` at `~/.local/bin/jigc` (+ `doc-code`
  probe). Phases 1–2 modify jigc-core (`crates/cli/...`) — rebuild + re-install
  before re-testing arms against the new behavior.
- **Read first:** this file → `REPLICATION.md` → `NUDGE-ARM.md` → `RUNBOOK.md`
  (gotchas) → DECISIONS.md (2026-06-22 entries).

## Success criterion for the session

Phase 0 demonstrated (floor concept proven) + Phase 1 landed (hygiene) + Phase 2
designed-and-prototyped (the universal floor blocks the quick-fix hole in jigc-core,
test-first) — a validated foundation, smallest-first, before the long-horizon study.
