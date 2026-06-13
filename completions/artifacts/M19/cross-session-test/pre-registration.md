# Pre-registration — the cross-session drift test (M19)

> **STATUS: FROZEN at sign-off (2026-06-13).** Forks resolved (see "Resolved at sign-off" below).
> The seed + the exact longer/denser no-cue sequence are in
> [appendix-sequence.md](appendix-sequence.md), constructed + verified before any arm runs.
> Deviations after this freeze are timestamped amendments.

The test M19 was built for. M18 built the store-wide `jigc validate` detector but it stayed on-demand
and **never fired** — a capable single session kept docs honest by diligence, and the over-time
advantage came back **H0 under the session-compression bound**. M19 closed the *firing* gap (a
`jigc setup`-installed **warn-only git `pre-commit` hook** that runs the sweep automatically, with its
warning relayed to an agent committing via `finalize`). This test puts that backstop in the regime
M18 couldn't reach: **genuine cross-session forgetting**, where diligence actually fails.

## What carries in from M18 + the M19 review (read before the hypothesis)

- **Build contract ≠ test hypothesis (M19 review S2).** The backstop's *contract* is that it
  **surfaces** store drift automatically; it is **warn-only and does NOT force repair** (it can't —
  report-only + no-auto-author invariants — and the agent can ignore a warning). The *hypothesis* is
  whether automatic surfacing **changes a cold agent's end-state doc honesty** vs no detector. These
  are different claims. **Firing-without-forcing can still reproduce M18's null** if the cold agent
  ignores the relayed warning — so **a tie is an honest, reportable outcome, not a failure of the
  test.**
- **No-cue slips are fair, and scored on repair (M19 review S3).** "Code changed; the doc that cites
  it wasn't in this session's scope" *is* the realistic over-time drift case. The test must (a) treat
  it as fair drift, not static-arm-rigging, and (b) **score the repair, not just the catch** — a
  finding the backstop *surfaced* but the agent did **not** repair counts as **no-win** for jigc.

## Hypothesis (falsifiable, with its null)

- **H1:** under genuine cross-session forgetting (a fresh cold agent per step) over a no-cue induced-
  slip sequence, the **jigc arm** (warn-only backstop installed) ends with **fewer stale doc↔code
  citations** than the **static arm** (no backstop) — because the hook surfaces accumulated drift at
  commit time, the relayed warning reaches the agent, and the agent repairs it; the static arm's slips
  persist silently.
- **H0 (must be returnable):** no difference — either (a) both arms' cold agents slip the same no-cue
  citations and the jigc arm's agent **ignores** the relayed warning (firing-without-forcing = M18's
  null, one level up), or (b) cold agents are diligent enough to catch their own slips in both arms.
  **A tie or static-win is real and reportable.**

## The arms (differ ONLY in mechanism)

| | Arm J (jigc) | Arm S (static) |
|---|---|---|
| Doc store | jigc-managed (same authored arch-doc + 2 adr + spec as M18) | identical-content static markdown |
| **Backstop** | **`jigc setup`-installed warn-only `pre-commit` hook** runs `jigc validate` on every commit; the warning is relayed to the agent (finalize success-relay) | **none** — no hook, no detector |
| Commit path | `jigc finalize` (hook fires; relay surfaces drift) | plain `git commit` |
| `CLAUDE.md` | identical project-facts base + the standing "keep citations honest" rule + the jigc adapter | identical base + the same standing rule + the frozen conventions block |

The seed is the M18 `ratelimit` toy project **extended** into a **longer, denser** citation surface
(**16 citations** across an 8-component arch-doc, **4** ADRs with `cites-code`, and a 4-criterion
spec), driven by an **8-step** no-cue code sequence (**10** breakable citations across 4 docs, **6**
stable controls). Full verbatim seed + sequence in [appendix-sequence.md](appendix-sequence.md). Every
step is a pure code change that **never mentions** `architecture/`/`decisions/`/`specs/` — the no-cue
property — so a cold agent has no cue to audit the docs its change silently breaks. The denser surface
multiplies slip opportunities and later-step catch opportunities vs M18's 5 steps / 9 citations.

## The apparatus — genuine cross-session forgetting (the M18→M19 change)

M18 ran **one** subagent through all 5 steps with full continuous context — so it remembered the docs
and fixed each break in-step (the compression that produced H0). M19 runs **a fresh, cold, blind
subagent per step**:

- Each step's subagent (fresh `general-purpose` context) is given **only**: its twin path, its
  **single** step's code task (verbatim from the M18 appendix — pure code, no doc mention), and "work
  in this repo, follow its `CLAUDE.md`; commit your change when done." It has **no memory** of prior
  steps, **does not know** prior steps touched docs, and is blind to the comparison/metric.
- It sees the current repo state — carrying prior steps' code **and any drift prior cold agents
  slipped**. This is "the developer who changed the code weeks ago and has forgotten the doc exists."
- **Arm J:** committing via `jigc finalize` fires the pre-commit hook → `jigc validate` reports
  accumulated `doc-code` drift → finalize **relays** the warning to this cold agent → following
  `CLAUDE.md`'s keep-citations-honest rule, the agent *may* repair (a follow-up doc-fix task). Whether
  it does is the hypothesis.
- **Arm S:** plain `git commit` → no hook → no signal → any slip ships and accumulates.
- **n = 3 independent chains per arm** (6 chains total; drift/diligence is stochastic across cold
  agents). Each chain is the ordered sequence of per-step cold subagents on its own fresh clone of the
  arm's seed (6 chains × 8 steps ≈ 48 cold-subagent runs).

**Fairness crux:** identical seed, identical tasks, identical "keep citations honest" `CLAUDE.md`
intent; both arms' cold agents are equally likely to slip the same no-cue citations. The **only**
difference is arm J's automatic warning. That isolates the backstop's effect.

## What's measured (independent judges, M18 pattern)

- **Primary A — final stale-citation count** per chain (judge checks each `path#symbol` against that
  chain's final code; lower = more honest).
- **Primary B — per-step caught-vs-shipped timeline** (objective per-commit: when each slip entered;
  and for arm J, whether a backstop-surfaced slip was **repaired at a later step** or persisted).
- **Net value (the headline, M18 framing):** true catches **−** false-positive friction **−** repair
  cost. **Repair is scored, not just the catch:** a drift the hook surfaced but the agent left
  unrepaired is **no-win**.
- **Observational:** did the hook fire (it should, every arm-J commit)? did the relayed warning reach
  the agent, and did the agent **act** on it (the build-contract-vs-hypothesis question — the crux of
  whether warn-only is enough)? over-editing on the control step.

## Run mechanism + pinned binary

- Pinned `jigc` sha `5d64adc…` (the M19 release, hook-installing + finalize-relaying), `doc-code`
  sibling at `~/.local/bin/`. Frozen for the whole run; no mid-test rebuild.
- Seeds: copy the clean M18 `mdt-seed-{J,S}`; **re-run `jigc setup` in the J seed with the M19 binary**
  so the pre-commit hook installs over the existing authored docs (verified: setup is idempotent +
  adds the neutral hook step, docs intact). Confirm: arm-J seed has `.git/hooks/pre-commit` running
  the sweep; `jigc validate` clean (0 findings); arm-S seed has no hook, citations grep clean. Clone
  each seed fresh per chain (`mdt19-J{1,2}`, `mdt19-S{1,2}`).
- Judge: independent same-model **review subagents** (≥1 per chain, blind, de-identified over the
  M18-style bundles), confirming the stale count + the timeline; **Codex escalation** for an
  irreconcilable margin or the verdict-prose pass (the M18 green-but-flattering lesson).

## Honest bounds (carried into the verdict)

1. **Warn-only surfaces, never forces** — a tie (agent ignores the warning) is honest, not refutation.
2. **No-cue slips are the realistic over-time case** — fair, scored on repair (surfaced-but-unrepaired = no-win).
3. **Cold-per-step ≈ but ≠ true cross-session** — it's still one orchestration; a real developer's
   weeks-apart forgetting is approximated, not reproduced. Subagents ≈ not = human sessions; n=2.
4. **Structural drift only** (`symbol-exists`) — behavioral prose drift out of frame (M18 observed it
   leaking in; not scored here).
5. **The hook only fires on a commit, and the relay only reaches an agent committing via `finalize`**
   — arm J must commit through jigc for the loop to close (its `CLAUDE.md` routes it there).

## Success criteria (valid regardless of outcome)

1. All 4 chains produced a final committed state from the identical seed + cold-per-step sequence.
2. Judges scored final stale citations + the caught-vs-shipped timeline on de-identified states.
3. Reported **with bounds** — including a tie/static-win (H0) and whether the backstop fired-but-
   wasn't-acted-on (the firing-without-forcing case).
4. The verdict separates **surfaced** from **repaired**, and **structural** from behavioral, and does
   not overclaim warn-only firing as enforced honesty.

## Resolved at sign-off (2026-06-13) — freezing the design

1. **n = 3 chains per arm** (6 chains; the strongest of the offered options against cold-agent
   stochasticity).
2. **A longer/denser no-cue sequence** — the extended `ratelimit` seed (16 citations, 4 docs) + the
   8-step sequence in [appendix-sequence.md](appendix-sequence.md); more slip + catch opportunities
   than M18's 5/9.
3. **Arm-J repair = agent discretion under `CLAUDE.md`** — the cold agent decides whether to act on a
   relayed warning, guided only by the standing keep-citations-honest rule; keeps the hypothesis
   falsifiable (it can ignore the warning → H0).

**Frozen at this commit** alongside the appendix. The seed twins are built + verified before any arm
runs (re-`setup` the J seed with the M19 binary so the pre-commit hook installs; confirm `jigc
validate` clean + hook present; arm-S static + no hook).
