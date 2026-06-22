# The UserPromptSubmit nudge arm — fixes retrieval, but it's instruction again

**Written 2026-06-22.** Tests whether a *just-in-time* nudge (a `UserPromptSubmit`
hook injecting "run `jigc start` first" at task-submit time) fixes Sonnet's
retrieval-under-execution gap — where session-start instruction (`AGENT.md`) failed.

## Result (n=16/arm, Sonnet, reliable drift measure)

| arm | drift-rate |
|---|---|
| plain | 88% |
| jigc (bootstrap, bypassable) | 19% |
| **jigc + UserPromptSubmit nudge** | **0%** |
| static | 0% |

The nudge drove drift to 0%. But the *mechanism* matters — and a stream-json sample
(n=6, with `bypassPermissions`) plus an adversarial finalize test show it is **not**
jigc's validation doing the work.

## What actually happens under the nudge (stream-json, n=6)

Every run: **engaged jigc (5 jigc cmds), picked `quick-fix` (6/6), ran `jigc task
finalize` (6/6), AND made 6–13 direct file edits.**

1. **Retrieval is fixed** — Sonnet now runs `jigc start` (vs 0/16 bare-task
   engagement). The just-in-time nudge surfaces jigc where the static `AGENT.md`
   didn't.
2. **But Sonnet picks `quick-fix` every time** — the consequence-modeling gap
   ([WHY-SONNET-BYPASSES.md](WHY-SONNET-BYPASSES.md)) at scale: it judges a rename
   "trivial" and takes the lightweight, doc-free workflow.
3. **`quick-fix` finalize does NOT validate doc↔code.** Adversarial test: drove
   `jigc start --workflow quick-fix`, renamed the code leaving the committed
   arch-doc anchor dangling, staged, `jigc task finalize` → **committed clean**
   (`fix: rename class`), no `doc-code.symbol-exists` block. jigc's anchor check is
   **workflow-scoped** and quick-fix omits it.

⟹ **jigc's validation never fired in any nudge run.** The 0% drift came from the
nudge's own doc-consistency clause (it told the agent to keep docs consistent) plus
the agent editing the docs by hand — **instruction, not mechanism.** Same result as
every prior arm: jigc's *differentiator* stayed dark; an instruction did the work.

## The architectural insight (the one durable takeaway)

jigc's doc↔code gate is **workflow-scoped**, and the weaker model reliably picks the
one workflow (`quick-fix`) that omits it. So enforcement that depends on the agent
choosing the right workflow is defeated by the agent's workflow choice. **Only a
workflow-agnostic, always-on check catches drift regardless of workflow:** the
`Stop`-hook gate, a *blocking* pre-commit hook, or a finalize that *always* runs
store-wide `jigc validate`. This is the concrete form "enforce-don't-instruct" must
take — and it's a fixable product change.

## Honesty note — harness bugs caught mid-investigation

Two one-off diagnostic runners (not the main `run-rep.sh`, which was correct) were
invalid and re-run: (1) a prompt with backticks was command-substituted by the
container's `bash -c` (mangled the class names); (2) a stream-json sample omitted
`--permission-mode bypassPermissions`, so jigc was permission-blocked and the agent
fumbled. Both were detected by reading the transcripts, corrected, and re-run; the
**n=16 drift result used `run-rep.sh` throughout and is unaffected.** Recorded so a
re-runner doesn't repeat them.

## Bounds

n=16 drift / n=6 mechanism, Sonnet, one task family, doc↔code only. The nudge text
deliberately mentioned doc-consistency (so it doubles as an instruction); a
pure-routing nudge would still hit quick-fix → no validation, so it would not reach
0% on its own. The finding is robust to that: jigc's mechanism didn't fire either way.
