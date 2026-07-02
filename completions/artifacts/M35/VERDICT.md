# M35 CLI-owned-rename cost+completeness study — verdict

**Written 2026-07-02**, after the full pre-registered matrix ran through isolated evolving-twin
containers: **22 valid sequences × 8 edits = 176 cold-agent runs** (pinned matrix: Sonnet 4.6
3 arms × 3 reps + Opus 4.8 cost-extremes × 2 reps; plus the [Amendment 1](pre-registration.md)
Sonnet 5 supplementary cell, 3 arms × 3 reps), scored by the arm-agnostic two-path oracle.
Clean run: **0 control-edge violations, 0 oracle disagreements, 0 auth failures** across every
cell (`study_valid: true`; one Opus plain sequence died on a transient `ConnectionRefused`,
was **discarded whole and re-run** per protocol — kept under
`.discarded-plain-8-rep2-connrefused` in the runs dir for the audit trail). Matrix spend
$30.60 (+ $0.69 certification). Full numbers: [results.md](results.md).

## One-paragraph verdict

**WIN — the pre-registered win condition holds, on both Sonnet generations.** jigc is
**strictly cheaper per rename than plain** (Sonnet 4.6: $0.080 vs $0.125, −36%; Sonnet 5:
$0.129 vs $0.262, −51%) **and ≥ static on completeness** (1.0 vs 0.875 on Sonnet 4.6; 1.0 vs
1.0 on Sonnet 5) — the first study in the series where jigc **beats plain on effort**, turning
the enforcement tax the prior studies measured into a labor saving. The mechanism is directly
visible in the turns column: the jigc arm resolves each rename in **3.5–4.6 turns** (one
`jigc rename` + verification) where plain/static burn **8.3–14.4 turns** hand-chasing
referrers. The two pre-registered confounds cleared decisively: **verb-engagement was 100%**
(64/64 jigc-arm edits used `jigc rename`; availability *did* induce usage — the adapter
advertisement sufficed) and **rename-error→recovery cycles were 0** (the verb never errored
into the re-orient loop the cross-doc study identified as the cost driver).

## The gate (pre-registration §3): MET

> **WIN** iff jigc is strictly cheaper per rename than plain **AND** jigc completeness ≥
> static — per model, only when the study is valid.

| model | jigc cheaper than plain | jigc completeness ≥ static | verdict |
|---|---|---|---|
| **Sonnet 4.6** (pinned) | ✅ $0.0795 < $0.1248 | ✅ 1.0 ≥ 0.875 | **WIN** |
| **Sonnet 5** (Amendment 1) | ✅ $0.1286 < $0.2621 | ✅ 1.0 ≥ 1.0 | **WIN** |
| Opus 4.8 (pinned, cost-extremes) | ✅ $0.1934 < $0.2655 | *(static arm not run — by design)* | cost clause holds |

The Opus cell was pre-registered as the **cost-extremes pair only** (jigc vs plain — the
matrix design in `run-matrix-refint.sh`), so its static clause was never testable; the formal
per-model verdict is `indeterminate — missing arm(s)`, but the clause it *was* designed to
test — the headline cost comparison — holds (−27% cost, 4.6 vs 10.4 turns/rename).

## The headline numbers

| arm | model | cost/rename | turns/rename | completeness | engagement | final dangling |
|---|---|--:|--:|--:|--:|--:|
| **jigc** | sonnet-4-6 | **$0.0795** | **3.46** | **1.0** | 1.0 | 0 |
| static | sonnet-4-6 | $0.1524 | 10.42 | 0.875 | — | 1 |
| plain | sonnet-4-6 | $0.1248 | 8.25 | 0.25 | — | 6 |
| **jigc** | sonnet-5 | **$0.1286** | **4.13** | **1.0** | 1.0 | 0 |
| static | sonnet-5 | $0.2150 | 10.00 | 1.0 | — | 0 |
| plain | sonnet-5 | $0.2621 | 14.38 | 1.0 | — | 0 |
| **jigc** | opus-4-8 | **$0.1934** | **4.56** | **1.0** | 1.0 | 0 |
| plain | opus-4-8 | $0.2655 | 10.43 | 0.625 | — | 3 |

Three regularities worth naming:

1. **jigc is the cheapest arm in every cell — including cheaper than the static one-liner.**
   The cross-doc study's finding was jigc as the *most expensive* arm (2.9–3× static) because
   its enforcement only fired *after* the mistake; owning the operation inverts the sign. The
   static arm's one-liner still costs 1.7–1.9× jigc, because `git mv` + `sd`/`grep` + verify
   is itself a multi-turn errand.
2. **The turns gap is the mechanism, observed.** 3.5–4.6 turns (jigc) vs 8.3–14.4 (manual) —
   exactly the "one command vs N turns of grep-and-edit" the design predicted
   ([ideas/cli-owned-rename.md](../../../ideas/cli-owned-rename.md)).
3. **The non-greppable referrer behaved as constructed on the pinned model.** Sonnet 4.6's
   static arm dropped **exactly 1 ref in every rep** — the edit-4 2-hop `cites` referrer whose
   slug the prompt withholds. jigc's edge-index walk repointed it every time (and the
   certification transcript shows the agent deriving the slug semantically, then issuing one
   `jigc rename`).

## The Amendment-1 finding — capability closes the correctness gap, and *widens* the cost gap

The Sonnet 5 supplementary cell (run because Sonnet 5 shipped between handoff and the run) is
the study's most instructive secondary result. On the current-generation model, **plain and
static both reached completeness 1.0** — Sonnet 5 chases referrers thoroughly even
uninstructed, including recovering the constructed non-greppable case (once the agent renames
the file it has *derived* the old slug, so post-derivation grepping becomes possible; the
construction bounds prompt-side non-greppability only, and a sufficiently thorough agent
closes it). This is the cross-doc study's lesson repeating on schedule: **detection/correctness
differentiators decay as model capability rises.**

But the cost gap did not decay — it **widened**: plain on Sonnet 5 pays $0.262/rename and
**14.4 turns/rename** (the most turns in the whole matrix) to achieve that thoroughness,
while jigc pays $0.129 and 4.1 turns for the same completeness. The more capable and diligent
the agent, the more manual labor the rename costs it — and the more one transaction-bound
command saves. **The cost win is the durable differentiator; the correctness win is the
capability-dependent backstop.** This is precisely the reframe M35 was chartered on, now with
its sharpest empirical support.

## Confounds (pre-registered §4) — all controlled

- **Verb-engagement (primary):** `rename_engagement = 1.0` in all three jigc cells — 64/64
  edits used the verb; zero hand-edit bypasses. Availability ≠ induced-usage was the risk;
  induced-usage is what happened (the one-line adapter advert + the orientation footer
  sufficed).
- **Rename-error→recovery:** 0 cycles in every cell. The verb never errored on collision,
  dirty tree, OOB drift, or fan-out context; the Inc-2 backstop hook never fired
  (`hook_blocked_seen: false` throughout) because the agent never went out of band.
- **Static arm is a real baseline:** the one-liner rule was followed (the static arm's 10
  turns/rename are it *executing* the scripted rename faithfully) — the cost win is against a
  genuine scripted baseline, not a straw.
- **Small-N:** every cell is flagged `underpowered` (2–3 reps vs the `min_reps=5` bar) — this
  is a **directional first pass, not powered statistics**. The direction is unambiguous:
  per-cell spreads are tight (jigc 4.6: $0.622–0.646; plain 4.6: $0.934–1.077) and the
  ordering is identical in every rep of every cell.

## Honesty bounds

- **Directional, not powered** (above). The pattern is clean and rep-consistent, but N is
  small and per-rename cost is the noisiest metric measured.
- **Synthetic seed corpus** — a 9-adr + 1-arch-doc decision graph purpose-built for
  forward-ref density. Generalizes to multi-doc `supersedes`/`cites` graphs, not to any
  specific corpus.
- **The non-greppable construction is prompt-side only.** It proves the agent cannot be
  *instructed* to grep for a slug it was never given; it does not prevent a thorough agent
  from deriving the slug mid-edit and grepping afterward (Sonnet 5 did exactly that). The
  genuinely-unreachable-referrer case remains untested — and after this result, likely moot:
  the cost win no longer leans on it.
- **Completeness scored over structured managed refs only** (the determinism boundary) —
  prose/unmanaged mentions were reported by the verb but never counted.
- **The win requires the agent to use the verb** — engagement was 100% *in this harness*
  (fresh cold agents, adapter-advertised). A differently-primed agent could bypass; the Inc-2
  backstop covers correctness in that case but the cost win would shrink toward the bypass
  rate.
- **Model-generation scope** — the verdict is proven on Sonnet 4.6, Sonnet 5, and (cost
  clause) Opus 4.8. The Sonnet 5 cell is the current generation at run time.

## What this means for the project

- **The M35 acceptance gate is met.** The differentiator the cross-doc study said detection
  couldn't prove — jigc cheaper per rename than an unassisted agent — is proven, on the
  pinned pre-registered matrix and again on the current-generation robustness cell.
- **The [cost-of-enforcement](../../../ideas/cost-of-enforcement.md) thesis flips on this
  surface:** the enforcement tax measured in the doc↔code and cross-doc studies becomes a
  ~2× labor *saving* when the CLI owns the operation instead of policing its aftermath.
  "Own the verb, don't police the mistake" is now the empirically-backed design rule for
  future differentiators.
- **The honest boundary stands:** a static project can still script the effect; jigc's edge
  is shipping it as the canonical, transaction-bound managed-doc interface — one command,
  every structured ref, atomic, engaged 100% of the time in practice.

With this verdict the M35 milestone is **complete** — verb + backstop shipped and audited,
acceptance study run and won. The roadmap spine (M1–M35) is closed.
