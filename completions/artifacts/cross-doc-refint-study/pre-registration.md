# Cross-doc forward-ref integrity study — pre-registration ("the harder claim")

**Status:** **SIGNED OFF 2026-06-24** (user: "fine to get a good impression from where we
can go further and decide if we need to study more into one specific direction"). The
protocol below is now fixed; deviations are recorded as amendments with reasons. (The
companion doc↔code study's [pre-registration](../long-horizon-study/pre-registration.md)
is the template this mirrors.)

**Intent — a directional first pass.** This is run to get a **good first read** on whether
the cross-doc ref-integrity capability gap is real, *not* as the final powered word. The
outcome decides whether a deeper, more focused follow-up (drilling into one specific
direction — e.g. far-hop-only edits, larger graphs, a specific failure mode) is warranted.
Read the §11 outcomes in that light: a clear signal in any direction is the deliverable,
and "we should study X harder next" is a legitimate, expected result.

**What this study is.** The doc↔code long-horizon study cleared the value gate
*partially* — jigc beat a static `CLAUDE.md`, but on a surface a static rule *could*
express ("keep your citations honest"). This study tests **the harder claim**: does jigc
beat a static `CLAUDE.md` on a differentiator a static file **structurally cannot
replicate** — **cross-document forward-reference integrity** across a whole managed store?
A win here is a **capability gap**, not a regime-bound advantage — decisive for the value
gate in a way the doc↔code win was not.

**Dependency (built, committed).** This study rests on the **store-wide `ref-resolves`**
increment (the 4th store-sweep family), `crates/engine` commit `e01614e` on branch
`study/long-horizon-many-edit`. Before that commit, `schema-conformance.ref-resolves`
fired *only* at `jigc task finalize`, scoped to task-touched edges — so a dangling
`supersedes`/`cites` already committed in the store was invisible to `jigc validate`,
hence to a pre-commit hook keyed on its findings. The increment makes the differentiator
**hook-reachable**; this study measures whether that reach is worth anything. No further
jigc-core change is needed or made for this study.

**Reversibility:** everything runs on throwaway container twins. Nothing touches the real
repo or a productive corpus. Records export to this directory as plain files. **Zero
corpus lock-in** — no productive-go commitment is made or implied (M33/M34 still gate
that; this study is what *might* fully clear the value gate that precedes them).

---

## 1. The question (precise)

Over a sequence of edits, each by a fresh cold agent, that **dangle a forward reference
across documents** (a `supersedes` or `cites` target deleted / renamed / split somewhere
in a managed store), does jigc's **salience-independent, store-wide ref-integrity
enforcement** keep the store's cross-document graph honest where a **static `CLAUDE.md`
instruction cannot** — because verifying that *every* committed forward edge still
resolves across the whole store is **not an instruction-followable act for a cold agent**?

The single differentiator engaged: **cross-doc forward-ref integrity** —
`schema-conformance.ref-resolves` over the committed edge index (`supersedes`: adr→adr;
`cites`: arch-doc→adr). Nothing new is built in jigc-core for this study beyond the
already-committed store-wide sweep.

## 2. Why this is *harder* than doc↔code — and why a static file structurally cannot win

The doc↔code study's differentiator was, in principle, expressible as an instruction: "when
you rename a symbol, update the doc that names it." A diligent enough cold agent, reminded
by a prominent rule, can follow it locally — the symbol and its mention are *co-located* in
the agent's working set.

Cross-doc forward-ref integrity is different in kind:

1. **The referrer is not in the agent's working set.** Edit *k* asks the agent to
   delete/rename/split decision **X**. The thing that breaks is a *different* document —
   ADR **B**'s `supersedes: adr:X`, or the arch-doc's `cites: adr:X` — possibly 1–3 hops
   away in the store graph, which the ticket never mentions and the agent has no reason to
   open. There is no local co-location to make the fix salient.
2. **The check is global, not local.** Catching the dangle requires walking *the whole
   committed edge set* and asking "does every `to` still resolve?" — exactly what
   `index::ref_resolves_store` does deterministically. An instruction can *say* "keep all
   cross-references valid," but executing it means the cold agent must reconstruct and
   traverse the store graph unprompted on an edit that looks purely local. That is the act
   we predict no instruction reliably induces, and that *no rules-file size* fixes.
3. **Drift compounds and hides.** As in the doc↔code study, each edit is a fresh agent
   doing only *its* ticket; a dangle introduced at edit 2 is never revisited by edit 6.
   Static drift can only accumulate; jigc's blocking sweep holds it at 0 by construction.

**The decisive contrast:** jigc's `validate` walks the edge index every commit and blocks a
dangling `to`; the static rule depends on a cold agent *choosing* to perform a global graph
check it was not asked to perform. We predict jigc-flat vs static-rising — and unlike the
doc↔code study, a static win here would require the agent to do something **no instruction
phrasing can reliably command**, so a jigc win is a *capability-gap* result.

**Honest pre-registered threat to decisiveness (§12):** the dangle-introducing edit
(rename/delete X) leaves the old name *greppable*. A sufficiently diligent cold agent could
`grep` for `adr:X` and discover B/the arch-doc. If static arms reliably do this, the
"non-instruction-replicable" hypothesis is *refuted* — a meaningful, honestly-reported
outcome. The study is designed to *measure* that, not assume it (edits place referrers at a
distance, and the dilution ladder measures decay of whatever diligence exists).

## 3. Enforcement model (settled, mirrors the doc↔code study)

**A blocking `pre-commit` hook**, identical in spirit to the doc↔code study: the pilot's
decisive negative was 0/32 tool engagement — a cold agent does not route a "delete a
decision" ticket through `jigc finalize`, so the finalize-gate floor never fires. The only
genuinely salience-independent enforcement point is a git `pre-commit` hook firing on
**every** `git commit`.

- **The detection delta from the doc↔code study.** That study's hook grepped
  `'"probe":"doc-code"'` on `jigc validate --format json`. The dangling-ref finding is
  `schema-conformance.ref-resolves` → `split_code` yields `probe = "schema-conformance"`.
  So the blocking hook for this study greps **`'"probe":"schema-conformance"'`** (or
  `ref-resolves`) — **one extra grep alternation in the study harness's
  `blocking-pre-commit`**, not a jigc change. (Optionally keep the `doc-code` alternation
  too; this study's seed corpus carries no `code-anchor`, so it is inert here.)
- **Honest framing (pre-registered):** the *same* shipped `jigc validate` mechanism,
  configured to **block** — a one-line config delta from the shipped warn-only default
  (which `exit 0`s). jigc's shipped hook stays warn-only and `doc-code`-keyed; **no jigc
  source is modified** for this study (the store-wide sweep it relies on is already
  committed in `e01614e`). The OUT-OF-SCOPE "don't re-open the floor design" boundary holds.
- **The honest bound:** `git commit --no-verify` and hook-skipping commit paths bypass any
  pre-commit hook. **Both are measured and reported** (§9); a bypass is a *routing/
  ergonomics* result, never counted as a mechanism win.
- **Why the static arm gets no hook:** the realistic static alternative is an instruction,
  not a hand-rolled per-project ref-graph-walking hook. jigc's hook runs the *general*
  store-wide `validate` (any forward edge, any doctype). Granting the static arm a
  ref-walking hook would be rebuilding a worse jigc — a separate question, named here, not
  run.

## 4. Fair control (settled): the dilution ladder

Reuse the doc↔code study's **dilution ladder** so "a bloated rules file is an unfair
confound" is a *measured finding*, not a thumb on the scale. Three static arms, an
**identical cross-reference-integrity rule**, increasing file size and burial:

| Arm | Rules file | Rule placement | Source |
|---|---|---|---|
| **C40** | ~42 lines | prominent | doc↔code study `arms/C40` (reused, rule text swapped) |
| **C160** | ~157 lines | mid-file (~23%) | doc↔code study `arms/C160` (reused, rule text swapped) |
| **C550** | ~452 lines | deep (~75% depth, ~8% of content) | doc↔code study `arms/C550` (reused, rule text swapped) |

The **rule text is byte-identical** across C40/C160/C550; only the surrounding file grows.
The rule states (final wording fixed at build): *"This project's decision records cross-
reference each other (`supersedes`) and the architecture doc cites them (`cites`). When you
delete, rename, or restructure any decision, keep every cross-reference across all
documents valid — no reference may point at a decision that no longer exists."* Predicted:
dangling-ref rate rises C40 ≤ C160 < C550.

## 5. Arms (full set)

All arms are container twins of the **same seed store** (§6). The managed store of ADRs +
arch-doc is **byte-identical content** across all arms: in arm A the docs are **managed
jigc docs** (the edge index is live, `jigc validate` walks it); in the static/plain arms
the same bytes are **plain `decisions/`+`architecture/` files** the rule says to keep
consistent.

| Arm | Method | The only variable |
|---|---|---|
| **A · jigc-hook** | `jigc setup` + the managed ADR/arch-doc store + **blocking** pre-commit hook keyed on `schema-conformance` (§3) | enforcement is a **salience-independent, store-wide mechanism** |
| **C40** | flat ~42-line `CLAUDE.md`, prominent ref-integrity rule, no jigc | the rule as **high-salience instruction** |
| **C160** | ~157-line `CLAUDE.md`, rule mid-file | instruction at **moderate** dilution |
| **C550** | ~452-line `CLAUDE.md`, rule deep | instruction at **heavy** dilution |
| **P · plain** | bare agent, no `CLAUDE.md`, no rule | base-rate dangle anchor |

**Isolation (proven in the doc↔code study):** one fresh `--rm` container per *edit*, full
filesystem isolation, read-only mounted OAuth credential. The *store* persists across the N
edits of a sequence (evolving twin); each *agent* is fresh (cross-session forgetting).

## 6. The twin & the documented surface (the doc graph is the subject)

The differentiator is the **doc graph, not the code**, so the seed is a **synthetic managed
store** built for a dense, multi-hop forward-ref graph (simpler and more controllable than
grafting onto `gherrink-ui-doc`; a code twin adds nothing the edge graph needs). Built once
and committed byte-identical into every arm's twin:

- **~8 ADRs** in `decisions/`, forming **`supersedes` chains** — e.g. B supersedes A, C
  supersedes B (a 3-deep chain), E supersedes D, plus standalone F/G/H. (Arm A authors them
  via `jigc doc create adr` + `jigc doc set-field …#supersedes`; static/plain arms get the
  identical bytes as plain files.)
- **1 `arch-doc`** in `architecture/` whose `cites` edges point at several ADRs (e.g.
  cites C, E, F) — the **far-hop referrers** (a `cites` dangle is 2+ hops from a decision
  edit, the hardest-to-find case).
- **Every forward edge resolves at baseline** (verified by `jigc validate` reporting zero
  `ref-resolves` findings on the seed). One **never-edited control edge** (an ADR pair
  whose `supersedes` is never touched) is a sanity tripwire: it must resolve at every edit
  on every arm.

Total seed forward edges: ~6–8 `supersedes` + ~3 `cites`. Exact graph finalized at build
and recorded here as an amendment.

## 7. The edit sequence (N≈8, fixed, pre-registered)

A fixed list of plain "decision-management" tickets, each **operating on a decision that is
the *target* of a forward edge from elsewhere**, so each edit is a cross-doc dangle
opportunity. Each ticket is a **byte-identical prompt across all arms** with **no doc/ref/
workflow hints** — a plain ticket, so the *method* determines whether the graph stays
honest. Each ticket goes to a **fresh cold agent** against the store as evolved by edits
1..k−1, and **ends "…and commit the change."** (the pre-commit hook only fires on an actual
`git commit` — symmetric across all arms). The exact N (symbols/slugs finalized at build)
follow this shape, deliberately placing the broken referrer **1–3 hops from the edit site**:

1. **Delete** a superseded decision (A) — now B's `supersedes: adr:A` dangles (1 hop).
2. **Rename/renumber** a decision (slug change) — referrers to the old slug dangle.
3. **Split** a decision into two new ones — the original slug is gone; its citer dangles.
4. **Delete** another superseded decision — its superseder's `supersedes` dangles.
5. **Rename** a decision the **arch-doc cites** — the `cites` edge dangles (2+ hops, the
   far case the rule is least likely to catch).
6. **Merge** two decisions into one — references to the absorbed slug dangle.
7. **Re-rename** the decision renamed in edit 2 (compounding — re-touches an evolved edge).
8. **Delete** a decision the arch-doc cites (far-hop dangle, compounded store).

Ticket form (illustrative): `Decision \`<slug>\` is obsolete — delete it from the
decisions, and commit the change.` — no mention of who references it.

## 8. Matrix, models, reps, cost

- **Models:** `claude-sonnet-4-6` (primary — carries the verdict) on all 5 arms;
  `claude-opus-4-8` on the extremes **A** and **C550** only.
- **A rep = one full N-edit sequence** (N fresh-agent invocations against one evolving twin).
- **Reps:** Sonnet R=3 per arm (5 arms); Opus R=2 per arm (2 arms) — **same shape as the
  doc↔code study** (the cost decision below).
- **Run count (N=8):** Sonnet 5×3×8 = 120 + Opus 2×2×8 = 32 → **152 agent invocations**.
- **Settings (identical):** `claude -p "<ticket>" --model <pinned> --output-format
  stream-json --verbose --permission-mode bypassPermissions`. One fresh container per edit;
  store persists; tree resets to HEAD between edits; drift measured on committed HEAD.
- **Cost estimate:** comparable to the doc↔code study (~$60–120 Sonnet-dominated), **plus
  the Opus ceremony tax** the doc↔code study measured (Opus ran ~$50/seq on the jigc arm).
  Opus is 2 arms × R=2 = 4 sequences — bounded, run in small post-re-login batches (§auth).

### Cost-of-enforcement: deliberately NOT mitigated before this study (decided 2026-06-23)

The doc↔code study measured a real ceremony tax (jigc ≈ static on Sonnet, **3–6× on
Opus** — the block→recovery loop). We considered fixing it first and **decided not to**,
for three reasons: (1) **wrong axis** — this study measures a correctness/capability claim,
not cost; a high tax doesn't invalidate a dangle-catch win. (2) **Fixing it can only
flatter jigc** — every cost lever reduces thrash, so running the **un-optimized** jigc is
the *conservative* test: a win despite the full tax is robust. (3) **It would confound** —
the biggest cost levers are *prose the agent reads* (block-message wording, bootstrap
nudge), so changing them mid-program means this study runs on a behaviorally different jigc
than the doc↔code one; the harness is held **frozen**. Cost-of-enforcement stays its own
future experiment ([ideas/cost-of-enforcement.md](../../../ideas/cost-of-enforcement.md))
with cost as the *dependent* variable. Operational mitigation only: Opus in small batches
(see auth handling). (`DECISIONS.md` 2026-06-23.)

## 9. Measures (objective first)

Scored **out-of-band on every arm** (running the ref-graph oracle on static/plain arms is
fair — it is the oracle, not enforcement):

- **Per-edit dangle count (objective):** after the agent commits edit *k*, how many
  committed forward edges (`supersedes`, `cites`) have a `to` that resolves to **no
  committed doc** in the store? Measured by **two independent paths, agreement required**:
  (a) the extended `measure.py` **edge-walker** (parse each doc's `supersedes`/`cites`
  values, check each `<type>:<slug>` target file exists in the store), and (b)
  `jigc validate --format json` on a managed copy, counting `schema-conformance.ref-resolves`
  findings (the arm-A oracle, now reporting store-wide after `e01614e`).
- **Cumulative dangling-ref curve:** count of live dangling cross-doc edges after edit *k*,
  *k*=1..N — **the headline**. Predicted: static curves rise (steeper at higher dilution),
  arm A stays ~0.
- **Arm-A enforcement trace:** hook **block** events (commit rejected on a `ref-resolves`
  finding), **recovery** (agent re-points or removes the edge → clean re-commit),
  **`--no-verify`/hook-bypass** count, and **jigc-verb engagement** (from stream-json). The
  win must be **hook-attributable** (a block is in the causal chain), as in the doc↔code
  study.
- **Ceremony cost:** turns + `total_cost_usd` per edit (the tax is *data*, reported, not
  optimized — §8).
- **Blind judge (the graph the oracle can't fully see):** §10.

## 10. Blind judging

A cross-model judge (**Codex**, available in the doc↔code study) receives the **final store
(all docs) + final diff of each sequence, arm-labels coded/stripped**, and per sequence
rates: **cross-reference integrity** (count dangling `supersedes`/`cites`), **graph
plausibility** (did the agent leave the decision history coherent, or orphan/garble it),
ceremony cost, collateral. Objective oracle stays canonical ("objective first"); Codex
corroborates and scores the coherence the edge-walker can't.

- **Judge caveat carried from the doc↔code study:** Codex **mis-flags stable lowercase
  `{#slug}` ids as symbols**. For ref-integrity judging, give the judge **the explicit
  list of doc ids in the store** and ask narrowly: *"does any `supersedes:` / `cites:`
  value name a doc not in this list?"* — not an open "find stale references" prompt that
  trips on slug formatting. The analyst unblinds only after the judge returns verdicts.

## 11. Pre-registered outcomes (so a null is publishable, not massaged)

- **Capability-gap win (clears the value gate decisively):** arm A ends sequences with
  **strictly fewer live dangling cross-doc edges** than ≥1 static arm at the same model,
  **the difference is hook-attributable** (≥1 `ref-resolves` block forced a fix or
  prevented a dangle the static arm shipped), **and** the static failures are ones a cold
  agent could not have been *instructed* to avoid (the broken referrer was hops from the
  edit, not locally greppable-and-fixed). The cumulative curve shows A flat while a static
  arm rises. This is the result that distinguishes a *capability gap* from the doc↔code
  study's regime-bound win.
- **Salience-decay finding (valuable even if A ties C40):** static dangle rate rises with
  dilution (C40 ≤ C160 < C550) — turns the confound into a result.
- **Hypothesis-refuting outcome (honest):** static arms reliably catch the dangle (grep the
  old slug, fix referrers) and tie A. Then "cross-doc ref integrity is non-instruction-
  replicable" is **refuted at the tested scale** — reported plainly, a real bound on jigc's
  differentiator, not massaged.
- **Routing-failure outcome (distinct from mechanism failure):** if agents bypass the hook
  (`--no-verify`/hook-skipping) at a rate that makes A's enforcement invisible, reported
  **separately** as an ergonomics result (the 0/32 precedent makes this live).

## 12. Honesty bounds

- **Structured study, not powered statistics:** N≈8 edits × R=3 (Sonnet) / R=2 (Opus). The
  *pattern across the sequence and the dilution ladder* is the signal.
- **The decisiveness hinges on the dangle being non-locally-greppable** (§2/§11). Edits
  place referrers 1–3 hops from the edit site, with the `cites` (arch-doc) cases at the far
  end; if static still catches them, that is the refuting result, pre-registered.
- **Blocking hook ≠ jigc default** (a 1-line config delta from shipped warn-only; the
  detection grep keys on `schema-conformance`) — stated wherever results are reported.
- **The hook is bypassable** (`--no-verify`); measured, never assumed away.
- **The store-wide sweep is intrinsic-blocking but the hook, not jigc's exit code, gates**
  — `jigc validate` is read-only and exits 0 even on a `ref-resolves` finding (§validation
  design); the study's blocking hook greps the finding and `exit 1`s. Stated so the
  mechanism isn't overclaimed as "jigc blocks out of the box."
- **The jigc under test is the un-optimized one** (cost tax unmitigated, §8) — a
  conservative test; a win is *despite* the tax.
- **C40/C160/C550 representativeness** rests on the doc↔code study's real-OSS-derived
  files (reused, rule text swapped); the swap is recorded so "the control lost because the
  file was a strawman" is not a live alternative.
- **Synthetic seed store** — the graph is purpose-built for forward-ref density; the result
  generalizes to the *shape* (multi-doc supersedes/cites graphs), not to any specific
  corpus. Greenfield adoption/migration is out of scope.

## 13. What this study does NOT test

The other differentiators (doc↔code anchors — that is the prior study; multi-pack
composition; reconciliation of out-of-band edits as its own axis); a static arm *with* its
own ref-walking hook; existing-foreign-doc migration; any productive-corpus commitment;
statistical generalization beyond the decision-graph edit family; **repeatable-item edges**
(`ref-resolves` over a per-entry managed ref inside a repeatable section stays deferred —
the seed uses doc-level `supersedes`/`cites` only, matching what `ref_resolves_store` walks).

---

## Deliverables this study produces (the build, gated behind sign-off)

1. **Edge-walking oracle** — extend `~/lh-study/harness/measure.py` to parse each doc's
   `supersedes`/`cites` refs, check each `<type>:<slug>` target exists in the store, and
   count dangling cross-doc edges per committed HEAD (the cumulative curve, exactly like the
   doc↔code anchor curve). Keep `--selftest`; add fixtures for a known dangling graph.
2. **Blocking hook update** — add `'"probe":"schema-conformance"'` (or `ref-resolves`) to
   `~/lh-study/harness/blocking-pre-commit`'s grep alternation (study config, §3).
3. **Twin assets** — the synthetic seed store (~8 ADRs + 1 arch-doc with the forward-ref
   graph), built via `jigc doc create`/`set-field` for arm A and emitted as identical plain
   files for the static/plain arms; the C40/C160/C550 rules files with the swapped
   ref-integrity rule; the N fixed tickets; the per-arm Dockerfiles (reuse `~/lh-study`).
4. **Harness certification on real data** — one live sequence + a hand spot-check of the
   transcript vs the parsed record **before** trusting any matrix; a live auth probe
   immediately before launch (the OAuth-expiry protocol from the doc↔code handover).
5. **Results + verdict** — `results.md` (the curves + rates), the blind-judge pass, and a
   `VERDICT.md` ruling on **capability-gap win vs salience-decay vs hypothesis-refuted** —
   the answer that gates productive-go.

---

## Amendments (recorded during the build, before the matrix ran)

Deviations from the signed-off protocol, each with its reason — logged for honesty, not
silently absorbed. All verified against the real binary 2026-06-24; the harness is the
durable copy under `harness/` in this directory (canonical), copied into `~/lh-study/` to run.

1. **Seed built by `jigc ingest` of hand-written conformant files, not `jigc doc create`/
   `set-field`.** The deliverable §3 named the create/finalize workflow; ingesting
   hand-authored conformant `adr`/`arch-doc` files is simpler and yields the *same* result —
   genuinely **managed** docs (indexed + file-state-baselined, "adopted register-only"). The
   store-wide `ref-resolves` sweep re-parses committed files index-independently, so the
   edge graph is identical either way. Builder: `harness/build-seed.sh`.
2. **Managed root is `docs/decisions/` + `docs/architecture/`** (not bare `decisions/`).
   Confirmed against the binary: the canonical path is `docs/<location>/<slug>.md`; files
   under bare `decisions/` are invisible to the store sweep. The oracle + seed builder use
   the `docs/` root.
3. **The arch-doc's `components` section is left EMPTY** — so the seed carries **no
   `code-anchor`** and the `doc-code` probe is inert. This keeps the study purely about
   *cross-doc refs* (`supersedes`/`cites`), the differentiator under test; doc↔code is the
   prior study's subject.
4. **The oracle is a sibling `measure-refint.py`, not an in-place extension of the doc↔code
   `measure.py`.** They measure different things (edge-graph resolution over the doc store
   vs tree-sitter over code); a sibling keeps the doc↔code oracle intact and the two
   selftests independent. Path (a) edge-walker is arm-agnostic; path (b) (`jigc validate`
   `ref-resolves` count) runs only where `.jigc/` exists (arm A) and must **agree** with (a)
   — a mismatch is recorded as an oracle-integrity flag, never reconciled. Both paths verified
   to agree on the live seed (clean=0, and on simulated edits 1→2 dangling).
5. **The 8 edits are finalized (`harness/sequence.json` + `prompts/edit-*.txt`).** Deletes
   for superseded ADRs (1-hop `supersedes` dangle), renames for cited ADRs (2-hop `cites`
   dangle — the far case), one split, and edits 7–8 as **compounding re-touches** of docs
   renamed at edits 3–4 (they re-dangle only on an arm that *repaired* the earlier dangle,
   e.g. arm A under the hook). **Caveat:** rename tickets name the exact new slug
   (`…to \`csrf-origin-validation\``) so edits 7–8 can reference it deterministically; this
   leans on the agent using the named slug — a measured caveat, not assumed. Prompts are
   byte-identical across all arms, hint-free, each ending "…and commit the change." (so the
   pre-commit hook is reached symmetrically — the doc↔code study's amendment 3).
6. **On-disk ref serialization confirmed:** inline bare (`supersedes: adr:x`) for a single
   value, inline bracket (`cites: [adr:a, adr:b]`) for a list; the oracle parses both (plus a
   defensive YAML-list form jigc does not currently emit).
7. **Drift is measured on committed HEAD; the tree resets to HEAD between edits** (the
   doc↔code study's amendment 4, unchanged) — the deliverable is what *ships*; a
   blocked-and-unrecovered arm-A edit lands no commit (drift never enters HEAD), at the
   faithful cost of an unlanded ticket (tracked as tickets-landed). A **never-edited,
   never-cited control edge** (`adr:stateless-jwt-sessions#supersedes → adr:server-side-
   session-store`) must resolve at every edit on every arm — a sanity tripwire (verified).
