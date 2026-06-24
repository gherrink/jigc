# Cross-doc forward-ref integrity study — verdict ("the harder claim")

**Written 2026-06-24**, after the full pre-registered matrix (5 arms × Sonnet R=3 + 2 arms ×
Opus R=2 = **19 valid sequences, 152 cold-agent runs**) ran through isolated evolving-twin
containers, measured mechanically on every committed HEAD by a **two-path edge-walking
oracle** (a store-graph file-resolution walk + a `jigc validate` `schema-conformance.
ref-resolves` cross-check, required to agree — they agreed on every sequence). Clean run: **0
control-edge violations, 0 oracle disagreements, 0 auth failures** (the one transient 401 was
discarded and re-run per protocol). This rules on the question the doc↔code study left open:
**does jigc beat a static `CLAUDE.md` on a differentiator a static file *structurally cannot
replicate* — cross-document forward-reference integrity?**

## One-paragraph verdict

**No — the harder claim is refuted, cleanly and on both models.** The differentiator's
premise was that verifying *every committed forward reference still resolves across the whole
store* is not an instruction a cold agent can follow — it requires walking a graph the agent
was never asked to open. **That premise is false for both Sonnet and Opus.** Given a static
rule that says "when you delete/rename/restructure a decision, search `docs/` for the old slug
and fix every reference," the agents **did exactly that** — fixing referrers one and two hops
from the edit site, in the same commit as the rename, at *every* dilution level including a
452-line file with the rule buried at 75% depth. Every static arm and the jigc arm held
dangling references flat at **0** across all 8 edits; only the **plain** arm (no rule) drifted,
compounding to **4** dangling refs. jigc's blocking hook reaches the same 0 — but only via the
block→recovery loop, making it the **most expensive arm by far** ($3.63 Sonnet / $7.66 Opus
per sequence, **2.9–3× the equivalent static rule**). jigc's enforcement here is *correct but
non-differentiating*: it guarantees an outcome a plain instruction already secures.

## The gate (from the pre-registration §11): NOT met — the refuting branch

> Capability-gap win = arm A ends with strictly fewer live dangling cross-doc refs than ≥1
> static arm at the same model, hook-attributable, **and** the static failures are ones a
> cold agent could not have been *instructed* to avoid.

**Not met, and decisively so.** No static arm leaves *any* residual drift to beat: A-Sonnet
final dangling **0** vs C40 **0**, C160 **0**, C550 **0** (and identically on Opus). There is
no gap to attribute to the hook. The pre-registration named this exact branch:

> **Hypothesis-refuting outcome (honest):** static arms reliably catch the dangle (grep the
> old slug, fix referrers) and tie A. Then "cross-doc ref integrity is non-instruction-
> replicable" is **refuted at the tested scale** — reported plainly.

That is what happened. The static arms grep the old slug and fix referrers; they tie A.

## Why this surface is instruction-replicable (where doc↔code was not)

The contrast with the doc↔code study (where jigc *won* for Sonnet) is the real lesson, and it
sharpens the project's working thesis rather than denting it:

- **doc↔code drift is a *separate-artifact* problem.** The code and the doc are different
  files with different purposes; while doing a code-rename ticket, updating the architecture
  doc is an easy-to-forget side errand, and a *capable* model (Opus) even cleared the anchor
  gate and left stale prose. The enforced surface caught what salience missed → jigc won.
- **cross-doc ref integrity is a *same-intent* problem.** "I renamed this decision" and "I fix
  what points at the old name" are the *same action* in the agent's mind — a single
  grep-and-edit that rides along with the rename. Once the rule says "fix references," a
  capable agent does it as one motion. There is no salience gap for enforcement to fill.

So the differentiator jigc enforces here is real (plain drifts to 4), but it sits **inside the
envelope a static instruction already covers** — the opposite of the doc↔code title-drift case.

## What this means for the value gate

- **The doc↔code win does not generalize to "any checkable surface beats static."** It was
  specific to a surface where the check is *non-local in intent*. Cross-doc refs are local in
  intent, so the static rule suffices. **jigc's edge over a static `CLAUDE.md` is not "it
  enforces more things" — it is "it enforces the things instruction-salience reliably
  misses."** This study maps a boundary of that edge.
- **jigc's value is not zero on this surface** — it converts "depends on the agent obeying an
  instruction" into "cannot be committed." That still matters where the instruction is *not*
  reliably followed: a weaker/cheaper model than Sonnet, an adversarial or fatigued agent, a
  genuinely non-local check (a ref whose target is *not* named in the triggering edit), or a
  team that wants a hard guarantee rather than a behavioral tendency. The finding is that *for
  this surface and these two capable models* the instruction is reliably followed, so the
  guarantee is **redundant, not wrong**.
- **The value gate is NOT cleared by this study.** The doc↔code study cleared it
  *partially/in-regime*; this study aimed to clear it *decisively* via a capability gap and
  **did not find one**. The honest standing of jigc-vs-static after both studies: **proven
  superior on non-locally-salient surfaces (doc↔code, weak model); tied on locally-salient
  ones (cross-doc refs, capable models); always more expensive.**

## Cost (the recurring tax, reconfirmed)

Arm A is the most expensive arm in every cell: Sonnet $3.63 vs $1.22–1.55 static; **Opus
$7.66 vs $2.67 static (2.9×)**. Root cause unchanged from the doc↔code study — the
block→recovery loop, heavier on a capable model that re-investigates each block. Here, where
the hook buys *nothing over a static rule*, the tax is pure overhead. This strengthens the
case for the deferred cost-of-enforcement work ([ideas/cost-of-enforcement.md](../../../ideas/cost-of-enforcement.md)):
the salience-independent guarantee should be **cheap and rarely-firing**, so it costs little
on the surfaces where it is redundant and still backstops the ones where it is not.

## Honesty bounds

- **Directional first pass, not powered statistics** (the sign-off framing). N=8 × R=3/2. The
  pattern is unusually clean (plain exactly 4 every rep; every rule/hook arm exactly 0 across
  19 sequences), so the *direction* is unambiguous even un-powered.
- **Scoped to this surface and regime** — doc-level `supersedes`/`cites` on a synthetic
  decision graph; renames/deletes/split where the broken referrer is 1–2 hops from a named
  edit. A genuinely non-local variant (the triggering edit does not name the slug that breaks)
  was **not** tested and is the place a capability gap could still live — see "further work."
- **No blind judge run.** The objective two-path oracle is unambiguous and the result is a
  *refutation*; a blind judge mainly guards against over-claiming a *win* (the doc↔code study's
  risk), not under-claiming one. Skipped deliberately, not by oversight.
- **The validity checks held** (results §2): the renames are real `git mv` slug changes (the
  dangle opportunity was live), plain genuinely drifts, static genuinely walks the graph, A
  genuinely recovers from real blocks (no `--no-verify` anywhere). The tie is real, not an
  artifact of edits that never created a dangle.

## Further work this points to (the "decide where to go deeper" fork)

1. **The genuinely non-local variant** — edits whose triggering ticket does *not* name the
   slug that ends up dangling (e.g. "consolidate the caching decisions" with no slug given, or
   a transitive supersedes chain where fixing B re-dangles C). If a static rule *can't* be
   followed because the agent can't know what to grep for, the capability gap may reappear.
   This is the sharpest next test of jigc's differentiator.
2. **A weaker/cheaper model** — Haiku or a small open model, where instruction-following
   degrades and the guarantee stops being redundant. The doc↔code study already showed the
   win tracks model capability *inversely*; this would map it on the cross-doc surface.
3. **Cost-of-enforcement** — make the block rare and cheap so the redundant-here / essential-
   there guarantee is affordable to leave on always.
