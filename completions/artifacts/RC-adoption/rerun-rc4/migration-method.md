# Migrating a GSD `.planning/` tree to jigc — the method (rerun, 2026-07-11)

Captured verbatim from the migrating agent's write-up of the rc.4 rerun in `~/ideas/project-alpha-2.0` (Claude Code `ultracode` orchestration). Archived as provenance for [trial-record.md](trial-record.md); the human's assessment is that this approach beat the first run's.

---

## The premise that shaped everything

The old planning system claimed the milestone had shipped and passed audit. The project owner said it hadn't. We treated every status claim in the old docs as an unverified assertion and re-checked it against the code. That single decision is what made this a migration rather than a reformatting job — and it's what turned up the fact that several shipped features report success while doing nothing.

Corollary: don't migrate what you haven't verified. A false claim carried into a new system is worse than a deleted one, because the new system lends it credibility.

## 1. Triage — let the tool classify before you decide

`jigc ingest` scanned 220 files and classified each against the managed schemas. Result: one file was conformant-but-misplaced, 219 parsed against no schema. That told us immediately that this was not an adoption job (nothing could be adopted in place) but a re-authoring job — every doc had to be rewritten through the write verbs. Knowing that up front set the whole plan's cost.

## 2. Assessment — fan out, read-only, code-checked

18 parallel agents, none of them permitted to write anything or touch the tool:

- 5 assessed the living docs (charter, backlog, codebase brief, research corpus, status files), each checking claims against the code.
- 13 mined the completed history (13 phase directories + the quick-task pile) for the only four things worth keeping: design decisions whose rationale isn't recoverable from the code, still-open debt, architecture knowledge, and work claimed complete that the code doesn't support.

Everything else — plan-by-plan summaries, validation reports, velocity tables, UI spec tables — was named as process exhaust and dropped.

## 3. Synthesis, then an adversarial critic

One agent deduped and resolved conflicts into a migration plan. Then a separate critic was asked the inverse question: what is being lost, what is being carried that shouldn't be, and is the reality-check honest? It ran the test suites itself and recovered five durable items the miners had missed — including a production defect (every wizard 404s in a built bundle) that nothing else had caught.

The critic earned its keep. Don't skip it. The synthesis step is where a plausible-but-incomplete plan looks finished.

## 4. Put the genuine forks to the human

Not "should I proceed" — the actual decisions: mine-then-delete vs. delete outright; is the charter still live; where does the backlog land. Two answers changed the work materially (one corrected a "rotate these credentials" finding into "this command is still needed").

## 5. Apply — parallel drafting, sequential writing

The split that made 40 docs tractable:

- Drafting was parallel (26 + 10 agents), because prose and code-verification are the expensive part.
- Writing was strictly sequential in one process, because jigc mints a task and makes a git commit per document. Parallel agents would race on tool state and on the index.

Agents returned structured data, never YAML — the payload rendering lived in one place, so format bugs got fixed once instead of 36 times.

## 6. Drain, then sweep

`jigc migrate` retires each foreign source in the same commit as the promoted doc, so `.planning/` drained itself. The residue was deleted in one commit whose message records why — including a warning that one deleted file contained a claim that's provably false, so nobody re-derives it from git history.

Then the sweep for what the migration itself broke: a README section pointing at the deleted directory, a CLAUDE.md that described a codebase three versions out of date, and a ledger entry telling someone to do work the migration had already done.

## What we got wrong

- Ordering. Arch-docs cite ADRs, and jigc fails the commit on a citation to a doc that doesn't exist. The natural authoring order is the reverse of the required one. Cost: one wasted cycle.
- Silent prose mangling. The payload templates demonstrate double-quoted YAML, which folds newlines. A doc committed clean with its bullet list collapsed into one line. Caught only by diffing the staged output.

## The result

41 managed docs, ~55 commits, `.planning/` gone, `jigc validate` clean. The 11 specs carrying 153 acceptance criteria are the real artifact: they're written to catch the original defects, because most of those bugs survived precisely because a test mocked a shape the real code never emits.

The load-bearing property: every architecture doc anchors to `path#symbol`, and jigc re-checks those anchors at commit time. A doc here cannot claim code that doesn't exist — which is exactly how the old paperwork went fictional.
