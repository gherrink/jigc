# The plan before the opening — every version, and every review of one

**Kept 2026-10-29.** Until this directory was committed, each file here existed only outside the repository — the build's briefs folder under `~/.claude/projects/<project>/stabilization-briefs/`, beside the session scratch directory a restart removes. The human's rule of that day is why it is here: a plan is tracked in the repository, never only on the machine ([working-principles.md](../../../../../../implementation/working-principles.md) → *What we will need again lives in the repository*).

**What the plan is.** *Everything owed before the first stabilization run opens* — the work the opening's fact sheet found owed on 2026-10-27 ([`DECISIONS.md`](../../../../../../DECISIONS.md) → *2026-10-27 — Four things are owed before the run's opening, in the human's order*), cut into lanes and tasks by a read-only planning agent, reviewed adversarially after each version, and revised under the human's rulings on the reviews' forks. **This directory is the plan's history, not its current form**: the readable plan is an entry in [implementation/roadmap.md](../../../../../../implementation/roadmap.md) and its builders' detail, progress and gate answers sit one directory up, each written by a separate task. The rulings the versions fold in are recorded in `DECISIONS.md` → the entries of 2026-10-29 that record them.

**Read it for the order of events, never as instruction.** Each version names the version and the reviews it works in at its head; each review names the commit it read. Line numbers in them are at the commit each names.

## The files, in order

| # | file | what | read against |
|---|---|---|---|
| 1 | [pre-opening-plan.md](pre-opening-plan.md) | **v1**, 2026-10-27 — read-only planning, after the human's order of 2026-10-27 and their rulings of that day | `0bd8a5b5` |
| 2 | [pre-opening-plan-review.md](pre-opening-plan-review.md) | v1's review, adversarial and read-only: 5 blocking, 11 should-fix, 9 notes; the seven forks, one line each | `0bd8a5b5` |
| 3 | [pre-opening-plan-v2.md](pre-opening-plan-v2.md) | **v2**, 2026-10-28 — works in v1's review, the review of `f99658b8` ([../../briefs/pre-opening-1-review.md](../../briefs/pre-opening-1-review.md)) and the human's rulings (a)–(e), its §0.3 | `7c34a222` |
| 4 | [pre-opening-plan-v2-review.md](pre-opening-plan-v2-review.md) | v2's review: 3 blocking, 16 should-fix, 13 notes, and three forks for the human — F-A, F-B, F-C | `7c34a222` |
| 5 | [pre-opening-plan-v3.md](pre-opening-plan-v3.md) | **v3**, 2026-10-28 — works in v2's review and the human's rulings F-A → B, F-B → A, F-C → A, its §0.1 | `7c34a222` |
| 6 | [pre-opening-plan-v3-review.md](pre-opening-plan-v3-review.md) | v3's focused re-review: 2 blocking, 7 should-fix, 6 notes, one question — T-NR | `7c34a222` |
| 7 | [pre-opening-plan-v4.md](pre-opening-plan-v4.md) | **v4**, 2026-10-29 — works in v3's re-review, the human's ruling T-NR → B and the main session's engineering calls | `7c34a222` |
| 8 | [pre-opening-plan-v5-as-drafted.md](pre-opening-plan-v5-as-drafted.md) | **v5 as its planner returned it**, 2026-10-29 — works in a light check of v4 (2 blocking, 5 should-fix, 3 notes), which has no file of its own: v5's head and §0.3 carry it | `7c34a222` |
| 9 | [pre-opening-plan-v5.md](pre-opening-plan-v5.md) | **v5 as the main session corrected it** after a check of v5 — the as-drafted text with the check's corrections applied in a few places and stated in a closing §9 that overrides the sections above it; that check, too, has no file but §9 | `7c34a222` |

*A count of a review's findings is the count its own head or table gives.* The human read v5 on 2026-10-29 and found it too hard to read; what followed is the readable plan above ([../../briefs/readable-plan-brief.md](../../briefs/readable-plan-brief.md)).

## Not kept

- **No other version existed outside these files**: the briefs folder held each version once, and v5 twice (as drafted, and as corrected).
- **The plan's first draft of the route hunter**, its review and the briefs of the tasks the plan schedules are build briefs, not plan versions: they are in [../../briefs/](../../briefs/README.md).

## Sanitized

Each file is its source with one change: an absolute host path is written as a repository-relative path, a home path with `~`, a scratch path as `<scratch>`, and the briefs folder's project directory as `<project>` — `pre-opening-plan.md` held five such paths, the other eight none. Nothing else was changed.
