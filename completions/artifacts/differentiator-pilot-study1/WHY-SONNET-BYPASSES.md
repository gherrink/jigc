# Why Sonnet bypasses jigc (and Opus doesn't) — diagnosed

**Written 2026-06-22.** The replication showed 0/32 jigc engagement for Sonnet.
Question raised: *why* — and *is Sonnet even reading the "use jigc" instruction?*
Three direct probes in the jigc container (full context, both models) answer it.

## Probe 1 — does the instruction reach the model? **Yes, fully.**

Asked (no task) "how does this project expect you to make changes?", **Sonnet
quoted `.jigc/AGENT.md` verbatim** — "`jigc` is your interface… never read or edit
managed docs directly. Start every task with `jigc start`." Opus did too, and noted
it was "reinforced by the SessionStart orientation." **It is not an awareness gap:**
the bootstrap loads (via `CLAUDE.md` → `@.jigc/AGENT.md`) and the SessionStart hook
fires, for both models, in headless mode.

## Probe 2 — asked to PLAN the rename, what does each choose?

Both *intend* to use jigc — but differently:

- **Sonnet** → "I'll use `jigc start --workflow quick-fix`… a symbol rename has no
  architectural ambiguity, nothing to plan or spec." It picks **quick-fix — the one
  workflow that skips managed-doc maintenance** (commit-only, no arch-doc).
- **Opus** → explicitly **rejects quick-fix**: "it affects the public API surface,
  which the recent commits show is documented," picks a doc-maintaining workflow,
  and plans step 6 "update any managed docs (e.g. the arch doc's core public API)."

So even when reflecting, Sonnet mis-models the rename as doc-irrelevant; Opus
reasons rename → touches documented API → must maintain docs. **That is the "Opus
recognizes the value" you observed** — consequence-modeling, not awareness.

## Probe 3 — under the BARE task (no "state your plan"), does jigc enter Sonnet's reasoning? **No.**

Full transcript of the real bare-task Sonnet run (`stream-json`): **0 jigc mentions
in any assistant text**, tools used = `grep, grep, Read, Write`. It pattern-matches
"rename a class" → grep-and-edit and **never consults the interface instruction it
holds.** The instruction is in context but is not *retrieved/applied* under task
execution — a knowing–doing gap.

## Root cause (two compounding capability gaps, not awareness)

1. **Retrieval-under-execution:** Sonnet holds "use jigc" but, given a bare task,
   never surfaces it — it defaults to its strong code-edit prior. Opus applies the
   meta-instruction proactively.
2. **Consequence-modeling:** even when it does reflect, Sonnet judges a rename
   "architecturally trivial" and picks the doc-skipping `quick-fix`. Opus connects
   the rename to the documented public API and routes to a doc-maintaining workflow.

## Implications

- **Instruction is provably insufficient for the weaker model.** Sonnet *has* the
  instruction (verbatim) and still bypasses; the A″ *proactive* bootstrap didn't
  move it either. So "write a better 'use jigc' instruction" is a dead end for
  Sonnet. This **vindicates enforce-don't-instruct as *necessary* for weaker
  models** — not a stylistic preference.
- **The robust fix is workflow-agnostic enforcement that doesn't depend on the
  agent choosing to engage** — i.e. the always-on `Stop`/commit **gate** (validate
  doc↔code at finish regardless of how the edit was made). It covers *both* Sonnet
  failure modes: full bypass *and* the quick-fix mis-selection (a quick-fix rename
  that dangles an anchor is still caught at stop). The replication's gate fired 0/16
  only because that arm's runs happened not to leave drift-at-stop (variance over a
  ~19% base) — its value is real but tail-confined, so it needs scale or the
  long-horizon regime to *measure*.
- A just-in-time nudge (a `UserPromptSubmit` hook injecting "run `jigc start` first"
  at task time, not just session start) might improve Sonnet *retrieval* — but it is
  still instruction, so treat it as ergonomics, not a guarantee; the gate is the
  guarantee.
