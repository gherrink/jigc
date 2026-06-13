# M18 Handover — next session: close the doc↔code-over-time gap, then test the thesis where it should win

**Written 2026-06-13.** Branch `main`, tree clean, gate green (fmt · clippy -D · 77 suites · build).
M17 is closed (honestly — see below); two controlled thesis tests ran and **tied/favored static** at
moderate scale; the **strongest remaining shot** at the founding thesis is M18, now scoped on the
[roadmap](../../../implementation/roadmap.md). The next session **plans M18** (milestone-planning
workflow), then runs its two halves: **(1) build** a store-wide doc↔code re-validation, **(2) test**
the maintenance/drift-over-time thesis. This handover is the running start.

## Read first — where the thesis actually stands (don't re-derive this)

The founding thesis is *jigc makes a coding agent measurably more correct/effective than a static
`CLAUDE.md`*. As of M17 it is **not demonstrated, and relocated** ([../M17/VERDICT.md](../M17/VERDICT.md)):

1. **Two controlled comparisons, both negative-or-tie.** The M17 **single-task pilot** tied (jigc ≈
   static methodology; both out-disciplined GSD's no-test fast skill). The **native-grain test**
   ([../M17/native-grain-test/](../M17/native-grain-test/)) — a structure-rich arch-doc + superseding
   ADR, judged blind by Codex on structural-integrity defects — came back **static 0 defects, jigc 1**
   (jigc left a superseded ADR's status unflipped: a capability gap). At moderate task size a capable
   model hand-structures correctly; jigc's integrity wins were *machine-guaranteed* but didn't convert
   to a lower defect **count**.
2. **The scoping probe that created M18 (the key finding).** Scoping the drift test, three probes on a
   jigc twin established: **jigc enforces doc↔code integrity at *authoring* time, not over time.** A
   committed doc's `implemented-by` anchor whose symbol the code renames is:
   - **NOT** caught at an unrelated task's `finalize`,
   - **NOT** caught when the doc itself drifts and is absorbed,
   - **caught ONLY** when the doc is re-authored through its own workflow (verified: `doc-code.symbol-exists`
     fires then),
   - and there is **no store-wide `jigc validate`** to sweep committed docs against current code.

   So "living documentation that stays honest against the code" — VISION's strongest distinctive claim —
   is authoring-time only; a committed doc **silently rots** under moving code. **This is the gap M18
   closes, and the reason the one-shot tests couldn't reach jigc's best argument.**

The honest standing: jigc's value is **scope-bound**, and its strongest *untested* claim is
**maintenance over time** — which can only show once the passive detector exists. That's M18.

## What the next session does

### Phase 0 — plan M18 (milestone-planning workflow)

M18 is **milestone-level only** on the roadmap; decompose it at pickup via the
[milestone-planning workflow](../../../implementation/milestone-planning-workflow.md) (the
`/milestone-plan` skill from the main session: scope → detect gaps → settle → review → decompose).
Confirm the two-half shape with the human at Settle. **Forks the planning must settle** (listed in
[decisions-pending.md](../../../implementation/decisions-pending.md) → the doc↔code-over-time entry):

- **The command shape.** A new top-level `jigc validate` (store-wide) vs. extending `task validate`
  with a `--store`/`--all` mode vs. a `jigc check`. Today only **task-scoped** `jigc task validate <id>`
  exists. Decide the verb, its output (per-stale-anchor findings), and exit-code semantics (reuse the
  exit-3 validation-blocked convention? or advisory by default — a report, not a gate?).
- **Severity + where it sits relative to the task-scoped gate.** Is store-wide drift *blocking*
  (you can't finalize while any committed doc's anchor is stale) or *advisory/on-demand* (a `jigc
  validate` you run, like a linter)? The CLAUDE.md invariant — *integrity holds at a finalize/commit
  boundary, not on every write* — and the "no auto-author" non-goal both bear here. A whole-store
  blocking gate on every finalize would be heavy and could block unrelated work; an on-demand report
  is lighter and matches "detect-and-route, never auto-fix." Recommend on-demand report first.
- **Scope of the sweep.** All `code-anchor` fields (`implemented-by` on `arch-doc`, `cites-code` on
  `adr`) across every committed doc in every active schema `location:`. Reuse the existing
  `doc-code` probe (the `~/.local/bin/doc-code` sibling) over the committed-store set — the engine
  already has `reconcile_committed_store`; the gap is invoking `doc-code` over committed docs (today
  it runs only on task-staged docs).
- **The test's evolution sequence** (Phase 2, pre-register it): which code mutations (rename / delete /
  move a symbol), how many, and the matched static arm.

### Phase 1 — build the store-wide doc↔code re-validation

The passive detector the claim needs. Likely a thin command over existing machinery (`doc-code`
probe + `reconcile_committed_store` walk); the real design work is the command surface + severity
decided at planning. Build test-first per the dev-workflow; it's engine+CLI work (not pack-only),
so the binary **does** change — rebuild + re-pin for the Phase 2 test.

### Phase 2 — the maintenance/drift-over-time test (pre-registered, independent judge)

The thesis test where jigc *should* win. **jigc-with-store-validate vs. a static doc set**, both
starting from an authored arch-doc + ADRs (reuse the NGT twins' end state, below), run over a
**pre-registered code-evolution sequence** (symbol renames/deletes/moves). Metric: jigc's
catch-rate + timing (does the new `validate` surface every stale anchor?) vs. the static arm's
**silent rot** (no detector — 100% by construction). **Carry the honest bound from the start:**
`symbol-exists` catches **structural** anchor drift (renamed/deleted symbols), **never behavioral**
drift (the prose lies while the symbol still exists) — so even a perfect store-validate only covers
half of "honest against the code." Frame it to **prove or refute**, like the NGT test. Judge: Codex
on de-identified artifacts/reports (the NGT pattern).

## Mechanics to re-establish (proven this session)

1. **Binary + probe.** `cargo build --release`; the pinned `jigc` is sha `8d7ab262…` at
   `~/.local/bin/jigc` (real ELF — **already restored**, not the jrun wrapper). The **`doc-code`
   probe** is at `~/.local/bin/doc-code` (sibling of `jigc`; jigc resolves it by default, or set
   `JIGC_DOC_CODE_PROBE`). **M18 Phase 1 changes the binary** → rebuild + re-pin before Phase 2, and
   land binary changes *between* runs only (a mid-test binary change contaminates → re-run).
2. **Twin pattern.** `git clone <repo> /home/maurice/jigc-dogfood/<twin>` on real disk (`/` has room;
   `/tmp` is a 15G tmpfs — too small for a `target/`). The methodology pack is external
   (`JIGC_PACK_DIR`); the **dev pack is embedded** (`crates/cli/pack`) and is the domain pack that
   owns `arch-doc`/`adr` + the `architecture-documentation`/`single-task` workflows — so a doc-authoring
   twin uses **`jigc setup` with NO `JIGC_PACK_DIR`** (embedded dev pack alone).
3. **Blind-subagent arms.** Run each comparison arm as its own `general-purpose` subagent (fresh
   independent context, given only the bare intent + its twin + "follow CLAUDE.md", blind to the
   comparison) — the inline way to get independent "sessions" without cross-arm carryover. Bound:
   subagents ≈ not = human sessions.
4. **Independent judge.** `codex exec --sandbox read-only` with an objective, evidence-demanding
   rubric over **de-identified** artifacts (the NGT judging prompt is a good template — it scored
   blind and correctly resolved both arms' ref formats). The Codex pass on the *verdict prose* also
   earned its keep this session (caught flattering) — Codex-pass the M18 verdict too.
5. **Symmetry discipline (the fairness crux).** Both arms get an **identical minimal project-facts
   `CLAUDE.md`** (strip jigc's real product CLAUDE.md — it would tell the static arm about jigc and
   over-enrich the jigc arm); they differ ONLY in the authoring mechanism (jigc adapter via `jigc
   setup` vs. a frozen-conventions block). Pre-seed identical committed docs as the shared starting
   surface. See [../M17/native-grain-test/pre-registration.md](../M17/native-grain-test/pre-registration.md)
   amendments 1–2.

## Reusable assets (don't rebuild)

- **The NGT twins** at `/home/maurice/jigc-dogfood/ngt-J` (jigc-managed arch-doc + ADRs) and
  `ngt-S` (hand-written) — **disposable, and already mutated by the scoping probes** (ngt-J has a
  renamed symbol + a discarded task). The clean *starting state* for Phase 2 is reproducible from the
  pre-registration + the seed files at `/home/maurice/jigc-dogfood/ngt-seed/` (the project-facts base
  `CLAUDE-base.md` + the two pre-seeded ADRs). Re-clone fresh for the measured test.
- **The pre-registration + results templates** at [../M17/native-grain-test/](../M17/native-grain-test/)
  — adapt the structure (hypothesis-with-null, frozen rubric, blind-subagent run mechanism, Codex
  judging, honest bounds) for the drift test.
- **The dogfood capture apparatus** at `implementation/dogfood/` (jrun + log-event.py + tally.py) —
  available if the drift test wants adapter-adherence/hook capture, though the drift metric is
  judge-counted, not hook-counted.
- **The Codex judging prompt** (in this session's transcript / the NGT results' evidence) — a proven
  objective-rubric template.

## State at handoff

- `main` clean, gate green, HEAD `3e3f3a9`. This session's 9 commits: `fc1ba3b`…`3e3f3a9`.
- **Environment normal:** `jigc` un-wrapped (real binary), `doc-code` probe in place, claude-mem on,
  global `~/.claude/CLAUDE.md` intact. No rollback owed.
- **M17 fully closed:** verdict (`VERDICT.md`, Codex-revised) + completion audit (PASS) + the
  self-hosting dogfood (green) + the native-grain test (H1 not supported). `decided-task` workflow
  shipped to main (closed decisions-pending:77).
- **M18 scoped** on the roadmap (milestone-level); the gap + forks in `decisions-pending.md`.
- **Four product gaps logged** in `decisions-pending.md` for need-driven pickup: no committed-field
  edit verb; silent last-write-wins on `set-field` for a `0..*` ref; (M16's) `arch-doc` bind/required
  read-role; and **the M18 driver** — no store-wide doc↔code re-validation.

## Traps

1. **Don't let M18 become a foregone conclusion.** The static arm has no detector → it rots 100% by
   construction. The *informative* content is jigc's **catch-rate, timing, friction, and the
   behavioral-drift blind spot** — plus whether the store-validate is genuinely usable (not just
   present). Frame to prove-or-refute; report a tie/limitation honestly.
2. **The behavioral-drift bound is load-bearing.** `symbol-exists` only knows a symbol exists, not
   that the prose still describes what the code does. Don't let "jigc catches drift" overclaim past
   structural anchor drift. Name it in the pre-registration *and* the verdict.
3. **Binary changes mid-test contaminate.** Phase 1 builds the command; freeze + re-pin before
   Phase 2; fixes land between runs only.
4. **Keep the determinism-boundary + no-auto-author invariants.** A store-wide check *detects and
   reports* stale anchors; it must not auto-rewrite docs (that's the auto-doc-writer non-goal). Lean
   on-demand-report over always-blocking-gate unless planning decides otherwise.
5. **Codex-pass the M18 verdict prose** — the green-but-meaningless / flattering blind spot applies
   to verdicts too (it bit the M17 verdict's first draft).
