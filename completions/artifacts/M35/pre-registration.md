# M35 CLI-owned-rename cost+completeness study — pre-registration

**Status:** **PRE-REGISTERED, designed + verified no-API at handoff (2026-06-28).** The
protocol below is fixed *before* the paid matrix runs; the build is certified runnable at
**zero API cost** (see [RUNBOOK.md](RUNBOOK.md) → State at handoff). The **paid matrix run
+ the verdict are the post-build HUMAN step** (RUNBOOK steps 3–4) — design-now / run-post-build
is the settled M35 decision ([DECISIONS.md](../../../DECISIONS.md) → 2026-06-28 M35 planning,
*Acceptance*). Deviations after this point are recorded as amendments with reasons.

**What this study is — the first study designed for jigc to beat plain on *effort*.** The
cross-doc forward-ref study
([VERDICT](../cross-doc-refint-study/VERDICT.md)) **refuted** the *detection* claim: a
dangling slug in a plain-file store is *greppable*, so an instructed static agent ties jigc
on catching it. That study tested catching the dangle **after** the fact. This study tests
the **reframe** ([ideas/cli-owned-rename.md](../../../ideas/cli-owned-rename.md)): own the
rename as a deterministic, transaction-bound CLI operation, so jigc is **cheaper per rename**
than an agent hand-chasing referrers — the cost-win the detection study said detection alone
could not prove. A win here turns the enforcement *tax* into a labor *saving*.

**Dependency (built, committed).** `jigc rename <old-slug> --to "<New Title>"` ships
(M35 Inc-1: the verb + the atomic committed-in-place transaction with `rollback_rename`;
`crates/cli/src/rename.rs`, `crates/cli/tests/flow37_rename.rs`) **and** the OOB backstop
ships (M35 Inc-2: store-scope rename detection, the exit-flip, the blocking pre-commit hook).
No further jigc-core change is made for this study; the harness only *drives* the shipped
binary.

**Reversibility:** everything runs on throwaway container twins (the cross-doc harness's
`--rm`-per-edit isolation). Nothing touches the real repo or a productive corpus. Records
export to this directory as plain files. **Zero corpus lock-in.**

---

## 1. The arms

Three arms over the **byte-identical** seed decision-record graph (`harness/build-seed.sh`;
the `docs/` subtree is sha256-identical across all three — verified by
`harness/build-templates-refint.sh`). The cross-doc study's 5-arm dilution ladder
(A/C40/C160/C550/P) **collapses to three** here — salience-decay is *not* what the rename
study measures; the **cost-win** is. The arm set is **jigc · plain · static**:

| Arm | Method | The only variable |
|---|---|---|
| **jigc** | `jigc setup` + `jigc ingest` (managed, baselined) + the `jigc rename` verb **available and adapter-advertised** (`arms/jigc-CLAUDE.md` appended to the project `CLAUDE.md`) + the **real Inc-2 backstop hook** (native `jigc setup` hook — blocks a this-commit OOB `git mv`) | the rename is **one owned, atomic CLI operation** |
| **plain** | seed only — no `CLAUDE.md`, no rule, no jigc, no hook | base-rate cost/drift anchor (the omitting case — runs **inert**, never error) |
| **static** | seed + a root `CLAUDE.md` carrying a **real shell one-liner** rename rule (`git mv` + `sd`/`grep` to repoint refs + a post-rename `grep` verification — `arms/static-CLAUDE.md`) | the rename as an **ad-hoc scripted instruction** |

The **static arm is a genuine baseline, not a straw**: it is allowed the shell one-liner an
ad-hoc static project would realistically write, exactly as a Codex cross-review required
([ideas/cli-owned-rename.md](../../../ideas/cli-owned-rename.md) → "Static arm must be allowed
a shell one-liner"). Each agent is a **fresh cold container per edit** against the evolving
twin; the store persists across the N edits of a sequence.

## 2. Metrics (headline first)

Scored out-of-band on every arm (running the ref-graph oracle on static/plain is fair — it
is the oracle, not enforcement). The aggregator is `harness/eval-sequence-refint.py`; the
oracle is `harness/measure-refint.py`; verb-engagement is `harness/analyze.py`.

- **THE HEADLINE — turns/cost per rename.** `total_cost_usd ÷ renames-performed` and
  `turns ÷ renames-performed`, where a rename is *performed* when an edit landed a commit.
  jigc should be **cheaper** (one `jigc rename` command) where plain/static burn turns
  hand-editing and chasing referrers. This is the new headline — the first study where jigc
  can beat plain on *effort*, not just tie static on correctness.
- **Dangling refs — over structured managed refs ONLY.** Count of committed forward edges
  (`supersedes`: adr→adr; `cites`: arch-doc→adr; `derived-from`: spec→prd) whose `to`
  resolves to no committed doc at the final HEAD. Scored over **structured managed refs
  only** — the determinism boundary's honest line: a **prose** mention or an
  **unmanaged-store** reference (code, README, tests) is **never** a measured ref, so it can
  never count as CLI-owned (in)completeness ([ideas/cli-owned-rename.md](../../../ideas/cli-owned-rename.md)
  → "score completeness only over structured managed refs").
- **Completeness on the non-greppable referrer.** `resolved ÷ total` structured managed edges
  at the final HEAD, with the study constructed around **≥1 deliberately non-greppable
  referrer**: seed edit 4 renames `sticky-lb-affinity` via a **semantic ticket**
  (`prompts/edit-4.txt`) that names the decision only by behaviour and **never prints the
  slug** — while the `arch-doc session-management` **cites** it at **2 hops** (a structured
  managed referrer). A plain/static agent that never derived the old slug has **nothing to
  grep**; `jigc rename` resolves the slug and walks the edge index to repoint the cite. A
  transitive `supersedes` chain is **greppable** (the old slug still appears textually) and is
  explicitly **not** the construction. `harness/check-seed.sh` *proves* the non-greppability
  by failing if the slug leaks into the prompt.

## 3. The pre-registered win

**jigc strictly beats plain on cost AND is ≥ static on completeness.** Concretely (rendered
by `eval-sequence-refint.py`'s `verdict`, per model, only when the study is valid):

- **Cost:** jigc's `cost_per_rename` is **strictly cheaper per rename than plain**.
- **Completeness:** jigc's `completeness >= static`'s completeness (jigc ≥ static, scored over
  structured managed refs only — `>= static`).

Both clauses must hold for a **WIN**. This is the first surface where jigc can win on
*effort*, turning the enforcement tax into a labor saving. Pre-registered alternative
outcomes (so a null is publishable, not massaged):

- **Cost-win refuted (honest):** the static one-liner or plain hand-editing matches jigc's
  per-rename cost — reported plainly as a bound on the cost differentiator.
- **Engagement-failure (routing, not mechanism):** the agent ignores the verb and hand-edits,
  re-incurring the block→recovery cost — reported **separately** via the verb-engagement rate
  (§4), never massaged into a mechanism result.

## 4. Confounds (pre-registered — control these or the win is not valid)

- **Verb-engagement is a PRIMARY measured quantity** (availability ≠ induced-usage). If the
  agent ignores `jigc rename` and hand-edits, jigc pays the *same* block→recovery cost and the
  cost win evaporates. `harness/analyze.py` classifies, per transcript, **used `jigc rename`**
  (`rename_engaged: true`) vs **hand-edited** (`git mv`/`sd`/file edit → `rename_engaged:
  false`) — keyed on the `rename` **verb** specifically, *not* on "any `jigc` ran". The
  aggregator surfaces it as the `rename_engagement` **rate**, a primary measured quantity, not
  a footnote.
- **The rename command erroring → its own recovery loop.** A `rename` that errors on
  collision, a dirty tree, OOB drift, or fan-out context can trigger the exact re-orient loop
  the study aims to avoid. The **rename-error→recovery** cycle count (`hook_blocked_seen`
  edits) is aggregated as `rename_error_recoveries`.
- **The static arm is allowed a real shell one-liner — not a straw baseline** (`git mv` +
  `sd`/`grep`; `arms/static-CLAUDE.md`), so any jigc cost-win is against a genuine baseline.
- **Small-N effort noise.** Per-rename cost is noisy on short sequences; the aggregator
  surfaces the `reps` count and flags any cell below `--min-reps` as `underpowered`, and the
  matrix is sized so the signal dominates the small-N noise.

## 5. Void-tripwires (study-integrity gates — NOT results)

Any cell with either tripwire is **voided** and the study renders **no verdict** (the
metrics are not trustworthy). Both fire loudly (stderr + `study_valid: false`) rather than
silently reporting a corrupted number:

- **control-violation = 0** — the never-edited, never-cited **control edge**
  (`adr:stateless-jwt-sessions#supersedes → adr:server-side-session-store`) must resolve at
  **every** edit on **every** arm. `control✗ = 0` is required; `control✗ > 0` voids the cell.
- **oracle-disagreement = 0** — the oracle's two independent paths (a: the
  `measure-refint.py` edge-walker; b: `jigc validate --format json` `ref-resolves` count, arm
  jigc only) must **agree**. `oracle≠ = 0` is required; `oracle≠ > 0` voids the cell.

Either tripwire `> 0` voids the cell **and** the study — an integrity tripwire, never a
result. `eval-sequence-refint.py` enforces both and withholds the verdict on a void.

---

## Honesty bounds

- **Structured study, not powered statistics** — the pattern across the sequence and the arms
  is the signal; per-rename cost is small-N noisy and flagged as such.
- **The win requires the agent to *use* the verb** — adapter-advertised, not sandboxed. If it
  hand-edits anyway, the Inc-2 backstop (the blocking hook + store-scope detection) still
  keeps correctness, but the **cost win is forfeited** — measured by the verb-engagement rate,
  never assumed away.
- **Completeness/dangling are scored over structured managed refs only** — a prose or
  unmanaged mention of the old slug is **never** counted as CLI-owned (in)completeness (the
  determinism boundary).
- **The non-greppable construction is a single, precisely-defined case** (the
  slug-withholding semantic ticket at edit 4, ≥2 hops via the arch-doc cite) — it generalizes
  to the *shape* (a referrer whose target slug the agent never derived), not to any corpus.
- **Synthetic seed store** — purpose-built for forward-ref density; the result generalizes to
  multi-doc supersedes/cites graphs, not to a specific corpus.
