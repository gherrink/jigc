# Long-horizon many-edit study — pre-registration

**Status:** SIGNED OFF 2026-06-22 (user: "proceed as written"). The protocol below is
now fixed; deviations are recorded as amendments with reasons. This is the study the pilot's [VERDICT](../differentiator-pilot-study1/VERDICT.md)
and [REPLICATION](../differentiator-pilot-study1/REPLICATION.md) named as the one
remaining shot at jigc's empirical superiority claim, and the study
[HANDOVER-long-horizon-study.md](../differentiator-pilot-study1/HANDOVER-long-horizon-study.md)
was written to set up.

**Reversibility:** everything runs on throwaway container twins of a baseline commit
(`gherrink-ui-doc @ 542b3206`). Nothing touches the real repo or the jigc repo.
Records export to this directory as plain files. **Zero corpus lock-in** — no
productive-go commitment is made or implied (M33/M34 still gate that; this study is
what *might* clear the value gate that precedes them).

---

## 1. The question (precise)

Does jigc make a coding agent **measurably more correct over a long sequence of
edits** than a static `CLAUDE.md` — specifically, does jigc's **salience-independent
enforcement** beat a **salience-dependent instruction** in the regime the prior three
ties (M17/M18/M19) and the pilot all *excluded*: many edits, fresh cold agent per
edit, against an evolving repo, with a rules file large enough that the relevant rule
loses the attention budget?

The single differentiator engaged: **doc↔code anchor consistency** — a managed
architecture doc whose `implemented-by` anchors must keep resolving as the documented
symbols are renamed / moved / deleted across the edit sequence. (The pilot proved this
is jigc's real shipped predicate, `doc-code.symbol-exists`; nothing new is built in
jigc-core for this study.)

## 2. The mechanism this study is built to expose (the long-horizon insight)

A single edit cannot probe it; that is why M17–M19 and the pilot tied/lost. Two
compounding effects only appear over a sequence:

1. **Drift compounds monotonically in the static arm.** Each edit is a *fresh cold
   agent* doing only *that edit's* ticket. If edit 3's agent renames a documented
   symbol but misses the doc, the doc is now stale — and edit 7's agent, asked only to
   do edit 7, **never revisits edit 3's miss**. Static drift can only accumulate.
2. **Static salience decays with rules-file size.** The pilot showed a clean rule = 0%
   and *moderate* dilution (157 lines) = still 0%; the untested regime is *heavy*
   dilution. We test a **ladder** to measure the decay curve rather than guess a point.

Against both, **jigc's blocking enforcement is flat**: a `pre-commit` hook running
`jigc validate` **rejects any commit that leaves a dangling anchor**, so drift cannot
be committed and cannot compound — independent of rules-file size, session length, or
whether the agent "chose" to use jigc. That flat-line-vs-rising-curve contrast is the
predicted finding.

## 3. Enforcement model (the make-or-break design decision — settled)

**Decision (user-approved 2026-06-22): a blocking `pre-commit` hook.** The pilot's
decisive negative was **0/32 tool engagement** — Sonnet never routes a "rename a
class" ticket through `jigc finalize`, so the finalize-gate floor never fires. The
*only* genuinely salience-independent enforcement point is a git `pre-commit` hook: it
fires on **every** `git commit` regardless of how the agent edited or committed.

jigc ships this hook **warn-only** (it `exit 0`s, because `jigc validate`'s exit code
is wrong-way-round — see `crates/cli/src/setup.rs`). The study twin uses a **blocking
variant**: the *same* shipped detection (`grep '"probe":"doc-code"'` on
`jigc validate --format json`) with `exit 1` instead of `exit 0` on a finding.

- **Honest framing (pre-registered):** this is **jigc's general `validate` mechanism,
  configured to block — a one-line config delta from the shipped warn-only default**,
  not a jigc-core change and not jigc's out-of-the-box behavior. No jigc source is
  modified; the OUT-OF-SCOPE "don't re-open the floor design" boundary is respected.
- **The honest bound:** `git commit --no-verify` bypasses any pre-commit hook, and an
  agent committing via a path that skips git hooks bypasses it too. **Both are measured
  and reported** (§7); a bypass is a *routing/ergonomics* result, never counted as a
  mechanism win.
- **Why the static arm gets no hook:** the realistic static alternative people actually
  use is an instruction, not a hand-rolled per-project grep hook. jigc's hook runs the
  *general* `validate` (any documented symbol, any doctype) — that generality is the
  differentiator. Granting the static arm a grep-hook would be rebuilding a worse jigc;
  it is a *separate* question, named here, not run.

## 4. Fair control (settled): the dilution ladder

**Decision (user-approved 2026-06-22): a dilution ladder**, so "a bloated rules file
is an unfair confound" becomes the *measured finding* instead of a thumb on the scale.
Three static arms, identical doc-consistency rule, increasing file size and burial:

| Arm | Rules file | Rule placement | Source |
|---|---|---|---|
| **C40** | ~40 lines | prominent, ~1 of ~5 sections | pilot `static-methodology-CLAUDE.md` (reused) |
| **C160** | ~157 lines | mid-file, §7 of 14 (~23%) | pilot `bigstatic-methodology-CLAUDE.md` (reused = pilot arm E) |
| **C550** | ~550 lines | deep, ~1 of ~22 sections (~5%) | **new**, extended from C160 with realistic sections **derived from real OSS `CLAUDE.md` files**, not a strawman (representativeness defense) |

The **rule text is byte-identical** across C40/C160/C550; only the surrounding file
grows. Predicted: drift-rate rises C40 ≤ C160 < C550.

## 5. Arms (full set)

All arms are container twins of `gherrink-ui-doc @ 542b3206`. The doc under measurement
is **byte-identical** across all arms: the expanded `core-public-api` arch-doc (§6),
documenting ~8 components each with an `implemented-by` anchor that resolves at
baseline. In arm A it is a **managed** jigc doc (validated); in the static/plain arms
it is the same bytes as a **plain `docs/` file** the rule says to keep honest.

| Arm | Method | The only variable |
|---|---|---|
| **A · jigc-hook** | `jigc setup` + arch-doc as a managed doc + **blocking** pre-commit hook (§3) | enforcement is a **salience-independent mechanism** |
| **C40** | flat ~40-line `CLAUDE.md`, prominent doc-rule, no jigc | the rule as **high-salience instruction** |
| **C160** | ~157-line `CLAUDE.md`, rule mid-file | instruction at **moderate** dilution |
| **C550** | ~550-line `CLAUDE.md`, rule deep | instruction at **heavy** dilution |
| **P · plain** | bare agent, no `CLAUDE.md`, no rule | base-rate drift anchor (pilot: ~88%) |

**Isolation (proven in pilot):** one fresh `--rm` container per *edit*, full filesystem
isolation, read-only mounted OAuth credential. The *repo* persists across the N edits
of a sequence (the evolving twin); each *agent* is fresh (cross-session forgetting).

## 6. The twin & the documented surface

Baseline `gherrink-ui-doc @ 542b3206`. The arch-doc is **expanded** from the pilot's 3
components to **~8 documented symbols** (the 3 existing — `UIDoc`,
`CommentBlockParser`, `MarkdownDescriptionParser` — plus ~5 more real public symbols
selected by inspecting the repo at build time), each carrying `implemented-by:
<path>#<Symbol>` that resolves at baseline. ~8 anchors give the 8-edit sequence a live
surface to churn throughout. The expanded arch-doc is authored once and committed
byte-identical into every arm's twin.

## 7. The edit sequence (N=8, fixed, pre-registered)

A fixed list of 8 plain refactor tickets, each **operating on a documented symbol** so
each edit is in the floor's domain (a dangling-anchor opportunity). Each ticket is a
**byte-identical prompt across all arms** with **no doc/test/workflow hints** — a plain
ticket, so the *method* determines whether the doc stays honest. Each ticket is issued
to a **fresh cold agent** against the repo as evolved by edits 1..k−1. The exact 8
(symbols finalized at build once the expanded arch-doc is fixed) follow this shape:

1. rename documented class **#1**
2. rename documented class **#2**
3. **move** a documented symbol to a new module (anchor *path* changes)
4. rename documented class **#3**
5. **delete** a documented helper (its anchor must be *removed* from the doc)
6. rename documented class **#4**
7. **re-rename** the symbol from edit 1 (compounding — re-touches an evolved anchor)
8. rename documented class **#5**

Ticket form (illustrative): `Rename the public class \`Foo\` to \`Bar\` throughout
this project.` — no mention of docs.

## 8. Matrix, models, reps, cost

- **Models:** `claude-sonnet-4-6` (primary — where instruction provably degrades) on
  all arms; `claude-opus-4-8` (the "too capable to drift on small tasks" control — does
  long-horizon load break it?) on the extremes **A** and **C550** only.
- **A rep = one full 8-edit sequence** (8 fresh-agent invocations against one evolving
  twin).
- **Reps:** Sonnet R=3 per arm (5 arms); Opus R=2 per arm (2 arms).
- **Run count:** Sonnet 5×3×8 = 120 + Opus 2×2×8 = 32 → **152 agent invocations**.
- **Settings (identical):** `claude -p "<ticket>" --model <pinned> --output-format
  stream-json --verbose --permission-mode bypassPermissions` (stream-json = the tool-
  call trace the engagement measure needs). One fresh container per edit.
- **Cost estimate:** ~$60–120 total (pilot: Sonnet runs cheap; Opus ~$0.6–1.6/run).
  Scope knobs if cost must drop: drop Opus (−$~38), drop Plain (−$~6/rep), R=2 Sonnet.

## 9. Measures (objective first)

Scored **out-of-band on every arm** (running `jigc validate` as a *measurement* on the
static/plain arms too is fair — it is the oracle, not enforcement):

- **Per-edit drift (objective):** after the agent commits edit *k*, does the arch-doc
  contain a **dangling anchor or a dead symbol name** (an `implemented-by` that resolves
  to no symbol, or prose/title naming a now-absent symbol)? Measured by (a) the
  `jigc validate` doc-code oracle and (b) an old-symbol grep — both, agreement required.
- **Cumulative live-drift curve:** count of live dangling anchors in the doc after edit
  *k*, *k*=1..8 — **the headline**. Predicted: static curves rise (steeper at higher
  dilution), arm A stays ~0.
- **Arm-A enforcement trace:** hook **block** events (commit rejected), **recovery**
  (agent fixes → clean re-commit), **`--no-verify` / hook-bypass** count, and
  **jigc-verb engagement** (from stream-json).
- **Ceremony cost:** turns + `total_cost_usd` per edit (jigc/static overhead is *data*).
- **Blind judge (prose, the floor's blind spot):** §10.

## 10. Blind judging

A cross-model judge (Codex if quota permits; else a same-family blind judge, as the
pilot fell back to) receives the **final doc + final diff of each sequence, arm-labels
coded/stripped**, and per sequence rates: doc↔code consistency (count stale refs),
**prose honesty** (does the prose still *describe* the current code — the M18 blind
spot the anchor oracle misses), ceremony cost, collateral. The analyst unblinds only
after the judge returns verdicts.

## 11. Pre-registered outcomes (so a null is publishable, not massaged)

- **Superiority (clears the value gate):** arm A ends sequences with **strictly fewer
  live dangling anchors** than ≥1 static arm at the same model, **and** the difference
  is **attributable to the hook** — ≥1 hook-block event is in the causal chain (a
  blocked commit forced a fix, or prevented a drift the static arm shipped). The
  cumulative curve shows A flat while a static arm rises.
- **Salience-decay finding (valuable even if A ties C40):** static drift rises with
  dilution (C40 ≤ C160 < C550) over the sequence — turns the confound into a result.
- **Null / parity:** all arms end clean, or A ≥ best static. Reported as the **fourth
  tie** — informative (the blocking floor, too, doesn't beat diligent static at the
  scale tested) and honest about the regime's limits.
- **Routing-failure outcome (distinct from mechanism failure):** if agents bypass the
  hook (`--no-verify` / hook-skipping commits) at a rate that makes A's enforcement
  invisible, that is reported **separately** as an ergonomics result — the pilot's 0/32
  precedent makes this a live possibility.

## 12. Honesty bounds

- **Structured pilot, not powered statistics:** N=8 edits × R=3 (Sonnet) / R=2 (Opus).
  The *pattern across the sequence and across the dilution ladder* is the signal.
- **Blocking hook ≠ jigc default** (a 1-line config delta from shipped warn-only) —
  stated wherever the result is reported.
- **The hook is bypassable** (`--no-verify`); measured, never assumed away.
- **The floor governs anchors, not prose** (M18 blind spot, unchanged) — the blind
  judge, not the oracle, scores prose honesty.
- **Static arm gets no hook** by design (§3) — the realistic instruction-only baseline.
- **C550 representativeness** rests on deriving it from real OSS `CLAUDE.md` files; the
  derivation is recorded so "the control lost because the file was a strawman" is not a
  live alternative explanation.
- **Same-family judge bound** if Codex quota blocks (as it did in the pilot); the
  headline measure is the objective oracle+grep, model-independent.

## 13. What this study does NOT test

Greenfield; existing-doc adoption/migration; the other differentiators (supersession,
cross-doc forward-ref integrity, multi-pack composition, reconciliation); a static arm
*with* its own enforcement hook; any productive-corpus commitment; statistical
generalization beyond the symbol-churn task family.

---

## Amendments (recorded during the build, before the matrix ran)

Deviations from the signed-off protocol, each with its reason — logged for honesty,
not silently absorbed:

1. **Documented surface = 7 real classes + 1 control interface (not 8 classes).** At
   baseline `542b320`, `EventEmitterBase.ts` and `Logger.ts` do not yet exist (added
   later upstream). The 7 classes that *do* exist are documented; the 8th anchor is the
   stable `BlockParser` **interface** (`BlockParser.types.ts#BlockParser`), never edited
   — the never-touched **control anchor** (a sanity tripwire: it must resolve at every
   edit on every arm). The "move" edit (3) retargets to `BlockParseError`.
2. **Dilution ladder is 42 / 157 / 452 lines (not 40 / 157 / 550).** C550 came to 452
   lines of realistic prose (markdown is less dense than estimated); the rule sits at
   ~75% depth and ~8% of content — a clear heavy point ~10× the small file, well
   separated from C160. The **rule text is byte-identical** across C40/C160/C550
   (verified), so the ladder varies only the competing surface, not the rule. Renamed
   the arm `C550→C550` label kept for continuity; actual size recorded here.
3. **Every ticket ends "…and commit the change."** The blocking hook is a `pre-commit`
   hook — it only fires on an actual `git commit`. Committing is made part of every
   ticket, **identically across all arms** (symmetric), so the block→recovery loop is
   exercised on arm A and the static arms commit their result too. Without this the
   enforcement point is never reached.
4. **Drift is measured on the committed HEAD; the tree resets to HEAD between edits.**
   The deliverable is what *ships* (the commit), so each edit's drift is measured on
   HEAD after the agent exits, and the next edit starts from the last landed commit. A
   blocked-and-unrecovered arm-A edit therefore lands *no* commit (drift never enters
   HEAD) at the faithful, measured cost of an unlanded ticket (tracked as tickets-landed).
5. **Agents run without installed `node_modules`.** The refactor (edit source + doc +
   commit) and the **static AST oracle** are build-independent; omitting the 406 MB dep
   tree is symmetric across all arms and avoids a heavy per-rep copy. Agents cannot run
   the build/tests in-container — a uniform limitation, not an arm asymmetry.
6. **Blind judge = Codex (cross-model), available this run** (the pilot's was
   quota-blocked). The objective oracle (`measure.py`, doc-code probe + dead-name grep)
   remains the headline; Codex corroborates and scores prose honesty independently.

## Deliverables this study produces (the build, gated behind sign-off)

1. **Sequence harness** — extend `completions/workflow-eval/` for **multi-edit
   sequences over a persistent evolving twin**: a sequence-runner (loop edits 1..N,
   fresh agent per edit, repo persists), `analyze.py` per-edit records, `eval.py`
   per-sequence timeline + cumulative-drift curve, selftest fixtures for the sequence
   path. (The handover names this extension as itself a deliverable.)
2. **Twin assets** — expanded arch-doc (~8 anchors), the C550 rules file, the 8 fixed
   tickets, the blocking-hook variant, the per-arm Dockerfiles (adapted from the pilot
   `harness/`).
3. **Harness certification on real data** — one live `run-eval` and a hand spot-check
   of `transcript.jsonl` vs the parsed record **before** trusting any matrix (the
   Phase-3 honest boundary the harness README names).
4. **Results + verdict** — `results.md` (the curves + rates), the blind-judge pass, and
   a `VERDICT.md` giving a defensible ruling on **superiority vs the salience-decay
   finding vs a fourth tie**.
