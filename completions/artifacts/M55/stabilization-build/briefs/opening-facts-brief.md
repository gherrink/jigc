# Brief — the opening gathers its facts before the first question (2026-10-27)

One builder, one commit on `fix/rc24-tier1` in `~/projects/gherrink-jigc`, starting from HEAD `186bb9e7`
(`git log -3`). The tree holds exactly one untracked file, `.claude/agents/stabilize-opening.md`, which the main session
wrote. **You never edit it** (subagents never edit `.claude/agents/`, `.claude/settings.json` or `CLAUDE.md`); if a fence
fails on it, stop and return the exact failure and the line it needs. Any other dirt stops you.

## The human's ruling, 2026-10-27

A stabilization run's opening **gathers its facts before the human is asked anything**, as milestone planning does: a
read-only agent, `stabilize-opening`, writes a fact sheet — one section per decision the opening takes, every fact with
its source and how it is known, no recommendation — and **no question of the opening goes to the human before the sheet
is back**. The recommendation comes with each question, from the main session. Why: planning's Settle rule
(`implementation/milestone-planning-workflow.md` → Settle: *"No design fork goes to the human before the baseline ledger
and all four gap-detectors are back"*, and why — M54's reopened fork); `stabilization-workflow.md` → *Opening a run* says
it runs "as milestone planning does" yet had no gather step, so the opening was the one point of a run where the human
was asked without gathered facts (the end of a round already brings triage, verify-real and the advocate's case first).
The human asked for it as a general step of the workflow, not a one-off sheet for this run; this run's opening is its
first use.

## What to write

Read first: `implementation/working-principles.md`, `implementation/stabilization-workflow.md` → *Opening a run* and
→ *Orchestration*, the planning doc's Settle paragraph, the new definition, and the top of `DECISIONS.md` for the form an
entry takes.

1. **`implementation/stabilization-workflow.md` → *Opening a run*:** the gather step at the head of the section — who
   (`stabilize-opening`, launched by the main session, not by the harness), what (the sheet, its sections being the
   doc's own list of what the opening decides), the rule (no question before the sheet is back; the one exception,
   mirroring planning, a question the human asks to take first, taken and flagged *may re-open once the sheet is
   back*), and where the sheet goes: committed beside `opening.md` as `opening-facts.md` in the opening's one commit,
   vetted by the same scanners as every other write. Point at the definition; restate none of it.
2. **→ *Orchestration*:** one sentence where the roles are listed, that the opening's gatherer is `stabilize-opening`,
   launched by the main session — so it is in neither the harness's `ROLES` table nor the guard's fenced list.
3. **`DECISIONS.md`**, at the top, dated 2026-10-27: the ruling, its why, the new definition and why no existing one
   fits (`capability-auditor` audits built capability for milestone planning; `stabilize-scope` resolves a round's
   doors; neither gathers what an opening decides).

**Check, before you write the doc's sentence about where the sheet goes,** that a file `opening-facts.md` in a run's
directory is accepted by everything that reads that directory — `dev/stabilize-record` (`state` and its writes), the
harness's path-class assert, the record step's check of untracked files, any fence that enumerates a run's files. Show
it by a command or a test, not by reading alone. **If anything would refuse it or needs a code change, stop and return
what and where** — that is a fork for the human, not part of this commit. If the record script's `--help` listing of a
run's files is what a reader goes by, say whether it should name the sheet; change it only if no fence holds that text
and it is one line — else return it.

## Rules

Red first where a fence applies. Run `doc_link_fence`, `release_pipeline_fence` (arm q, the *Never* paragraph),
`stabilize_harness_fence`, `dev_stabilize_guard` and any fence that enumerates `.claude/agents/` before the gate. Then
the full `dev/gate` in the background, read unpiped (never two gates at once; a failed gate stops the commit). Commit
via `git commit -F <file>`, the new definition included; message ends "(the stabilization workflow: the opening gathers
its facts first)" and the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Push by name (`git push origin fix/rc24-tier1`). No stash, no `rm -rf` on a variable path, never print an environment
variable's value, no host path / name / id in a file, the human is "they".

## Self-check — report it, one line each

1. Every sentence you wrote points rather than restates (the definition, planning's rule, the record script).
2. Every path you name resolves (`doc_link_fence`).
3. The sheet's place was shown accepted by a command or test you name.
4. Nothing in the doc says the harness launches the gatherer.
5. The `DECISIONS.md` entry quotes the planning rule exactly and names this run's opening as the first use.

## Return

Compact: the sha, the gate's `passed=` line, the pushed remote head, the self-check, anything left open with why.
