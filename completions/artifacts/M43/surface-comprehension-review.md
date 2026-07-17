# The pre-trial surface-comprehension review — rc.7, 2026-07-17

**Method (human-directed, 2026-07-17):** before spending the rc.7 verification trial, the whole emitted CLI surface was captured verbatim (49 captures through the installed `1.0.0-rc.7` binary in throwaway repos — [surface-corpus-rc7.md](surface-corpus-rc7.md)) and put through **two independent comprehension reads**: (a) a **blind fresh-eyes agent** (Claude-family, zero project context, only the corpus, working the trial's real decision tasks — routing, recovery, the write path, the commit boundary), and (b) a **cross-model review** (OpenAI codex, gpt-5.5) grading the corpus against [surface-contract.md](../../../design/surface-contract.md)'s style guide. Rationale: the rc.5 precedent — running a trial on known defects pollutes it with findings already in hand; and a cross-model read catches phrasing one model family happens to tolerate.

## Convergent findings (both reviews independently)

1. **The decided/single/dev-task catalog tie (would-cause-wrong-action).** The blind agent genuinely guessed on "add caching — decide the eviction policy" and flip-flopped after `describe`. Root cause (both reviews): the discriminator that matters — the decision's **destination** (decisions-log entry vs ADR vs none) — is invisible at the router tier; the words that differ ("test-first"/"end-to-end", "decision"/"decisions") discriminate least. The documented-symbol caveat likewise lives only in `describe`.
2. **The `docs: docs:` doubled subject (ambush, reproduced).** No surface states the subject renders `<type>: <summary>` or warns against a self-prefixed summary; the doubled subject committed clean with no finding. The blind agent: *"I would have written that commit."*
3. **The heading-depth ceiling reaches only the migrate path.** The seam-generated statement rides the `{{schema:}}` generator (migration templates); ordinary `set-slot` solicits and help are silent — a law-3 hole in the M43 batch's own coverage.
4. **The required-slot route is not copy-runnable and claims falsely.** "this finding's target is the address" while the printed target is a file path + section name; both reviewers reconstructed `adr:<slug>#<section>` by convention.
5. Smaller: the staged-read route omits `jigc task list` · the post-finalize changelog advisory leads with dead in-task commands · slug caps absent from `doc create --help`.

## Blind-only findings (credible, accepted)

- AGENT.md's "managed docs are exactly the `jigc doc list` set" does not compose with staged docs (not yet in the set → literally read-free) or `unregistered` rows (membership undecidable).
- `doc author`-after-incremental-verbs rejection stated only in one compose, not `author --help`.
- "Retire the foreign original" mechanics unstated (deleted? same commit?).
- `task discard` silently drops staged docs — the style guide's own ack rule ("what a discard threw away") was never implemented.
- No exit-code taxonomy on any surface (1 error · 2 usage · 3 blocking findings · 4 review hold — reverse-engineered by both readers).

## Certified clean (both reviews)

The task loop (resume / what's-left footers on every compose — "best-in-corpus"; the blind agent answered the whole task-model section with total confidence) · the errors-that-teach routes (`store.not-found`, enum rejection, wrong-id, carried-staged — recovery was "almost entirely route-following") · the write/read split · the migrate compose's self-sufficiency (the `--approve` gate explained before it can fail) · staging honesty.

## Disposition (human call, 2026-07-17)

**Fix all confirmed + cheap before the trial** — the full batch (convergent 1–5 + the five blind-only items), all prose/route/help tier plus one small route-address render. Landed as the pre-trial surface-polish commits on main; the trial then runs on the polished binary.
