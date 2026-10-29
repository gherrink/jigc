# Brief — fixes after the review of `a7c2d66a` (2026-10-27)

One fixer, one commit on `fix/rc24-tier1` in `~/projects/gherrink-jigc`, starting from HEAD `a7c2d66a`
(`git log -3`). The review is
`~/.claude/projects/<project>/stabilization-briefs/opening-facts-review.md` —
read it whole, and the first brief beside it (`opening-facts-brief.md`; its *Rules* hold for you). The tree holds one
modified file, `.claude/agents/stabilize-opening.md`, which the main session edited for H1 (it points at the doc's
list), H2 (the "impossible" sentence gone), notes 3, 4, 5 and 7, and the human's answer below. **You never edit it**; if
it disagrees with the doc you write, or a fence fails on it, stop and return the line it needs. Any other dirt stops you.

## The human's answer, 2026-10-27 (the review's question)

**A:** the gatherer may name what exists beyond the listed options. The sheet has a section *what exists beyond the
listed options* in each decision — a capability that exists and bears on the decision but is none of the values the doc
and code frame, each as a fact with its source, unranked, never a proposal. Why: the ruling's own reason, planning's
M54 fork, was an unlisted option a fact-gathering pass surfaced; a gatherer held to the listed options could never
surface one, leaving that only to the main session, whose framing the sheet exists to check.

## What to write

1. **H1 — `implementation/stabilization-workflow.md` → *Opening a run*.** The doc's list of what the opening decides
   and its data table carry every fact written at the opening: `budget`, `hand-on` and `order` (the queue — read
   `dev/stabilize-record --help` → `run-set` and the doc's own section on the queue for when a run has one; enumerate
   `run-set`'s fields yourself and say in your return that each is covered or why not), and the run's name with its
   loop branch, which are a section of the sheet (the definition now says so).
2. **H3 — the same section:** who places the sheet in the run's directory, and when — the main session, or a subagent
   before `opening.md` is first written, because the write guard binds from that file on (point at the doc's section on
   the write guard).
3. **Note 1:** the exception's second clause carried — *widening that one request into settling other forks early is
   the failure, not the exception* — by pointing at or quoting planning, not paraphrasing.
4. **The human's answer** in the doc where the sheet's content is described, as a pointer to the definition.
5. **`DECISIONS.md`:** a new entry at the top (not an edit of `a7c2d66a`'s), dated 2026-10-27: the review's three holes
   and how each closed (doc or definition, which line), the notes taken, and the human's answer with its why.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, the `run-set` field enumeration in one line, any
note left with why.
