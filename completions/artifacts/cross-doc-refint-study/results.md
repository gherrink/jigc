# Cross-doc forward-ref integrity study — results

Run 2026-06-24. Protocol: [pre-registration.md](pre-registration.md) (signed off as a
directional first pass). Harness: `harness/` (canonical), mirrored to `~/lh-study/`. All
sequences clean: **0 control-edge violations, 0 oracle (path-a vs path-b) disagreements, 0
authentication failures** across the whole matrix — the run is valid, not voided.

> **Headline (both models, decisive): the harder claim is REFUTED.** The problem is real —
> the plain arm ships 4 dangling cross-doc refs by the end of every sequence. But a **static
> `CLAUDE.md` cross-reference rule fully prevents it, at every dilution level including the
> 452-line heavy-burial arm, on both Sonnet and Opus**, and **jigc's hook only ties that
> rule, at 2.9–3× the cost**. Store-wide ref integrity turns out to be *instruction-
> replicable* for both models tested — contra the §2 premise the study was built to test.

## 1. The matrix (Sonnet primary, R=3 per arm)

Cumulative dangling cross-doc refs (`supersedes` + `cites`) in the committed HEAD after edit
*k*, mean of 3 reps — **the headline curve**:

| arm | e1 | e2 | e3 | e4 | e5 | e6 | e7 | e8 | final | tickets landed | hook-blocked | cost/seq |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--:|--:|
| **P** · plain        | 0.67 | 1 | 2 | 3 | 4 | 4 | 4 | 4 | **4.0** | 8.3 | — | $0.87 |
| **C40** · rule, prominent | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** | 8 | — | $1.22 |
| **C160** · rule, diluted  | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** | 8 | — | $1.32 |
| **C550** · rule, heavy (452 ln) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** | 8 | — | $1.55 |
| **A** · jigc hook    | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | **0** | 8 | **3/3** | **$3.63** |

Per-rep curves (consistency was near-total):

```
P    rep1 [1,1,2,3,4,4,4,4]=4   rep2 [1,1,2,3,4,4,4,4]=4   rep3 [0,1,2,3,4,4,4,4]=4
C40  rep1..3 all [0,0,0,0,0,0,0,0]=0
C160 rep1..3 all [0,0,0,0,0,0,0,0]=0
C550 rep1..3 all [0,0,0,0,0,0,0,0]=0
A    rep1..3 all [0,0,0,0,0,0,0,0]=0   (3/3 hit >=1 hook block -> recovery)
```

## 2. What each arm actually did (the validity checks)

The result only means something if the edits genuinely created cross-doc dangle
opportunities and the arms genuinely differed in how they handled them. Hand-verified:

- **The edits are real slug changes, not title-only edits.** On a "rename" ticket every arm
  performed a true `git mv` (e.g. `csrf-double-submit-cookie.md => csrf-origin-validation.md`),
  so the referrer (`cites`/`supersedes` two hops away) genuinely dangled unless fixed. The
  split (edit 5) created two new slugs and removed the original. So the dangle opportunity
  was live on every edit.
- **Plain (P) ignores the graph.** The plain agent did each *local* task correctly but never
  walked the store: its final arch-doc still cites all four ORIGINAL slugs
  (`redis-cluster-sessions, csrf-double-submit-cookie, ip-rate-limiting,
  append-only-audit-log`), every one now dangling → final 4. Drift compounds monotonically,
  exactly the predicted shape — for the arm with no rule.
- **Static rule arms (C40/C160/C550) walk the graph because told to.** Each fixed the
  referrer in the SAME commit as the rename — e.g. C40 edit-3's commit touches both the
  renamed ADR and `session-management.md`'s `cites`. Final cites on every static rep:
  `[adr:csrf-strict-origin, adr:token-bucket-rate-limiting, adr:audit-write-log,
  adr:audit-read-log]` — every entry the correctly-updated current slug, including the
  two-hop renames chained across edits 3→7 and 4→8, and the split correctly expanded the one
  audit cite into two. **Heavy dilution (C550, rule at ~75% depth of a 452-line file) did not
  degrade this.**
- **jigc (A) reaches the same end state via enforcement, not foresight.** All 3 reps hit
  >=1 blocking `schema-conformance.ref-resolves` at `git commit`, then recovered (re-pointed
  or removed the dangling ref) and committed clean — the block→recovery loop, verified
  legitimate (no `--no-verify` in any rep; `any_no_verify` 0/3). Final state identical to the
  static arms.

## 3. Reading

1. **The problem is real.** Cold agents do not maintain cross-document reference integrity
   unprompted: plain ships 4 dangling refs, every rep, drift compounding edit over edit.
2. **A static instruction is *sufficient* AND *dilution-proof* here.** Every static arm held
   at 0 — including C550. Unlike the doc↔code study (where a buried rule decayed and a capable
   model left stale prose), the cross-ref rule did not decay at all. The likely reason: the
   cross-ref fix is a **single, self-evident grep-and-edit that rides along with the rename
   itself** ("I renamed this file, so I update what points at it") — co-located in intent with
   the edit, unlike doc↔code where the doc is a separate artifact easy to forget. So the §2
   premise — that store-wide ref integrity is *not* instruction-followable for a cold agent —
   **does not hold for Sonnet.**
3. **jigc ties static, at a cost.** Arm A reaches the same 0 dangling, but only via the
   block→recovery loop, making it the **most expensive arm ($3.63 vs $1.22–1.55 static, $0.87
   plain)** — 2.3–3× the static rule. The enforcement is *correct* (it genuinely prevents the
   drift plain ships) but **non-differentiating against a good static rule**, and it carries
   the ceremony tax the doc↔code study already flagged.

This is the **hypothesis-refuting** pre-registered outcome (§11), reported straight: a real
bound on jigc's differentiator, not massaged into a win.

## 4. Opus (A · C550 × R=2) — the stronger-model confirmation

The refutation **holds identically for Opus**, and the ceremony tax is larger:

| arm | model | e1→e8 | final | tickets | hook-blocked | cost/seq |
|---|---|---|--:|--:|:--:|--:|
| **C550** · rule, heavy | opus | flat **0** | 0 | 8 | — | $2.67 |
| **A** · jigc hook | opus | flat **0** | 0 | 8 | **2/2** | **$7.66** |

Per-rep: `A-opus rep1 [0×8]=0 ($7.68); rep2 [0×8]=0 ($7.64); C550-opus rep1/rep2 [0×8]=0
($2.69/$2.65)`. Both arms reach 0; the stronger model follows the buried 452-line-file rule
just as reliably as Sonnet (C550 flat at 0, 8/8 tickets, final cites fully correct). The only
difference Opus makes is **cost**: arm A at **$7.66/seq is 2.9× the static C550 ($2.67)** and
~2.1× the Opus-on-Sonnet ratio — the heavier block→recovery tax the doc↔code study predicted
for a capable model that re-investigates every block. **Opus changes the price, not the
verdict.**

### One 401, handled per protocol
A-opus-rep1's **first** run hit a transient `authentication_failed` at edit 8 (OAuth token
expired mid-batch — the known short-lived-token behavior; the token recovered on its own, so
every later sequence was clean). Per the pre-registration ("never let a 401 reach the
verdict"), that sequence was **discarded** (preserved as `A-8-rep1.401-discarded/` with a
note) and **re-run on fresh auth** — the clean re-run ([0×8], 8 tickets, $7.68) is what the
table above uses. No 401-touched data reaches the verdict.

## 5. Honesty bounds (carried from the pre-registration)

- **Directional first pass, not powered statistics** — N=8 edits × R=3 (Sonnet). The pattern
  is unusually clean (plain exactly 4 every rep; all rule/hook arms exactly 0), so the
  direction is unambiguous even un-powered.
- **The refutation is scoped to this surface and this regime** — doc-level `supersedes`/`cites`
  on a synthetic decision graph, renames/deletes/split. It does **not** generalize to
  differentiators a static rule genuinely cannot express (the doc↔code title-drift the prior
  study found, multi-pack composition, reconciliation) — those remain open.
- **jigc's value is not *zero* here** — it still converts "depends on the agent obeying an
  instruction" into "cannot be committed," which matters where the instruction is *not*
  reliably followed (a weaker model, a genuinely non-local check, an adversarial/forgetful
  agent). The finding is that *for this surface and this model* the instruction IS reliably
  followed, so the guarantee is redundant.
- **Blocking hook ≠ jigc default** (a 1-line config delta from shipped warn-only; keyed on
  `schema-conformance`). `jigc validate` is read-only and exits 0; the study hook greps the
  finding and blocks.
