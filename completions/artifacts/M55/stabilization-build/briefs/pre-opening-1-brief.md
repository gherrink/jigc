# Brief — before the opening, task 1: a door the previous release did not have, and the stale line (2026-10-27)

One builder, on `fix/rc24-tier1` in `~/projects/gherrink-jigc`, from HEAD `495991e3` (`git log -3`; a
dirty tree stops you). **`implementation/working-principles.md` first** — neither the doc nor the code is the oracle.

## The human's ruling, 2026-10-27 → A

The opening's fact sheet (gathered by `stabilize-opening`, section 0) found four things owed before the run's opening.
The human ruled that they come first, in this order: (1) `wf-regression-not-comparable` and the stale line 706 of
`implementation/stabilization-workflow.md`; (2) the route hunter, `wf-route-walk` (ruled 2026-10-10, trigger *before
the run's opening*); (3) a review of the `test` stage as built, for a real run, once the hunter is in — the doc's own
condition (lines 13, 639, 669), which no record satisfies; (4) the human's decision on `wf-whole-canary-after-parts`;
then the opening. Why: the 2026-10-10 trigger stands, and round 1 runs with every instrument it is meant to have; a
false regression fact would reach the ledger whatever was chosen. **Record this ruling** in `DECISIONS.md` at the top
(newest first), dated 2026-10-27, in this task's commit.

## What to build

1. **`wf-regression-not-comparable`** (`implementation/decisions-pending.md`, its row): `finding-verifier.md` step 4
   says `regression: false` with *not comparable* in the basis; the `dev/stabilize-record` header (`triage-set`:
   `regression`) says the fact is absent where the door is one the previous release does not have; a filed verdict
   holds no basis, so under the filing it reads as *red there too* and nothing outside the report says *not
   comparable*. Make **one meaning, carried end to end**: from the verifier's filing through the record script into the
   ledger row and the state, so that *not comparable* is never read as *not a regression*, nor as *red there too*.
   Read how ruling 4 and ruling 5 of *2026-10-05 — The stabilization workflow, as ruled* use *a regression of the run*
   (a fact: green on the previous release's binary, red on the candidate) and what the route of such a finding is, and
   choose the form that fits them — a value of its own is likely; prove it, do not assume it. **Red first** in the
   record script's suite (a filing for a door the previous release lacks, read back through the state). If the right
   form needs a new stored file, registry or ledger column, or the choice is a genuine fork between two sound readings,
   **halt and return both sides** — it is the human's.
2. **`finding-verifier.md` is not yours to edit** (subagents never edit `.claude/agents/`). Return the exact
   replacement text for its step 4 sentence(s) — old and new, byte for byte — and stop before the gate. The main
   session applies it and resumes you; then gate, commit, push.
3. **The stale line**: `implementation/stabilization-workflow.md` ~706 says the reshaped `test` stage is "not built
   yet". By the code (the sheet's 0.2): the queue, the budget, the hand-on, the order and the verify waves are built
   (`dev/stabilize-record`, `.claude/workflow-sources/stabilize.js`, the 2026-10-12 entries); the route hunter is not,
   and `VERIFY_PASSES = 3` remains on the path of a run without an order. Say exactly that, pointing at the code and the
   entries; verify each claim yourself before writing it.
4. **`decisions-pending.md`**: `wf-regression-not-comparable` struck as built by this commit; `wf-route-walk`,
   `wf-whole-canary-after-parts` and the real-run condition annotated with the ruling's order (not struck).

## Rules

Red first. The full `dev/gate` in the background, read unpiped (never two gates at once; a failed gate stops the
commit). `git commit -F <file>` with the trailers:

```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <session>
```

Message ends "(before the stabilization run's opening: the first of what is owed)". Push by name. No stash, no `rm -rf`
on a variable path, never print an environment variable's value, no host path / name / id in a file, the human is
"they", no edits to `.claude/` or `CLAUDE.md`. If the harness source changes, run `dev/stabilize-harness` and commit
both files.

## Self-check — report it

1. *Not comparable* traced end to end: name each place it is written and read (filing → record → ledger → state), and
   the test that holds each.
2. No reading where *not comparable* counts as a regression of the run, or as *not a regression*, survives — `command
   grep` for `regression` over `dev/stabilize-record`, `dev/stabilize-step`, the harness source and the definitions;
   the count and what each hit does.
3. Every claim of the corrected line 706 verified against the code at your commit.

## Return

Compact. First return (before the gate): the verifier text, old and new, and what you built. Final: the sha, the
gate's `passed=` line, the pushed remote head, the self-check, anything left open with why.
