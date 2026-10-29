# The build's briefs and reviews — what steered each task, kept

**Kept 2026-10-29.** Until this directory was committed, every file here existed only outside the repository: in the build's briefs folder under `~/.claude/projects/<project>/stabilization-briefs/`, in a plan folder under `~/.claude/plans/`, or in the session scratch directory, which a restart removes and whose text files were copied out once before one (786 of them). The human's rule of that day is why it is here: what we will need again lives in the repository, never only on the machine ([working-principles.md](../../../../../implementation/working-principles.md) → *What we will need again lives in the repository*). The plan before the opening and its reviews have their own directory, [../pre-opening-plan/history/](../pre-opening-plan/history/README.md).

**What a brief is.** The text the orchestrating session handed one builder, fixer or reviewer of the stabilization workflow's build: the task, its rulings, its write-set, what binds it. A review is a read-only reviewer's report on what a builder committed; a *fixes* brief hands that review's findings to the fixer. **What each task did is its commit's and its `DECISIONS.md` entry's to say** — these files are how the work was steered, and are read as a record, never as instruction. A path in them is the path as it was when the brief was written; a brief names worktrees and scratch directories that no longer exist.

## The files

45 files besides this one, about 660 KB in all; the largest, the draft fact sheet, 77 KB.

| group | files | when |
|---|---|---|
| **The stabilization workflow's plan** — the settled shape and its build, as the human approved it: the rulings, four adjustments after two reviews, the build's order, the reviews' findings and what became of each. `DECISIONS.md` → *2026-10-05 — The stabilization workflow, as ruled* is its record; this is the plan it was written from | [stabilization-workflow-plan.md](stabilization-workflow-plan.md) | 2026-10-05 |
| **What binds a tooling builder** (stream T of the reshape plan) | [t-rules.md](t-rules.md) | 2026-10-17 |
| **Two repairs after the canary's fifth root** | [fix-handon-route-and-arrival-brief.md](fix-handon-route-and-arrival-brief.md) | 2026-10-18 |
| **The byte table, tasks B6–B10** — per family: the builder's brief, the review, the fixes brief; B6 and B7 in two parts each (`a`, `b`); B10 the whole table's record, with its review's own brief | `doc` family: [b6-doc-family-brief.md](b6-doc-family-brief.md), [b6a-review.md](b6a-review.md), [b6a-fixes-brief.md](b6a-fixes-brief.md), [b6b-doc-family-brief.md](b6b-doc-family-brief.md), [b6b-review.md](b6b-review.md), [b6b-fixes-brief.md](b6b-fixes-brief.md) · `milestone` family: [b7-milestone-family-brief.md](b7-milestone-family-brief.md), [b7a-review.md](b7a-review.md), [b7a-fixes-brief.md](b7a-fixes-brief.md), [b7b-review.md](b7b-review.md), [b7b-fixes-brief.md](b7b-fixes-brief.md) · movers: [b8-movers-family-brief.md](b8-movers-family-brief.md), [b8-review.md](b8-review.md), [b8-fixes-brief.md](b8-fixes-brief.md) · mint: [b9-mint-family-brief.md](b9-mint-family-brief.md), [b9-review.md](b9-review.md), [b9-fixes-brief.md](b9-fixes-brief.md) · the record: [b10-record-brief.md](b10-record-brief.md), [b10-review-brief.md](b10-review-brief.md), [b10-review.md](b10-review.md), [b10-fixes-brief.md](b10-fixes-brief.md) | 2026-10-18 to 2026-10-27 |
| **The opening's fact sheet** — the brief that made the opening gather its facts first, the review of that commit, its fixes brief, and **the draft fact sheet itself**, gathered by `stabilize-opening` at `495991e3` and never committed until now | [opening-facts-brief.md](opening-facts-brief.md), [opening-facts-review.md](opening-facts-review.md), [opening-facts-fixes-brief.md](opening-facts-fixes-brief.md), [opening-facts-draft.md](opening-facts-draft.md) | 2026-10-27 |
| **The route hunter's build plan**, and its review — the plan the human's order of 2026-10-27 put second, folded into the plan before the opening | [route-hunter-plan.md](route-hunter-plan.md), [route-hunter-plan-review.md](route-hunter-plan-review.md) | 2026-10-27 |
| **The reshape plan's `K1b`** (the lookup kit as every prompt's first call) | [reshape-k1b-brief.md](reshape-k1b-brief.md) | 2026-10-27 |
| **The tool index** and *deterministic first* | [tooling-index-brief.md](tooling-index-brief.md) | 2026-10-27 |
| **Before the opening, task 1** (*not comparable*, the stale line), and its review | [pre-opening-1-brief.md](pre-opening-1-brief.md), [pre-opening-1-review.md](pre-opening-1-review.md) | 2026-10-27 |
| **The CI fixes** — the Linux red that was the test's, the two byte-table cells that flipped by git version, and the run ids of the red stretch (the logs are not kept; each is re-fetchable by its id) | [ci-linux-setup-brief.md](ci-linux-setup-brief.md), [ci-flaky-byte-table-brief.md](ci-flaky-byte-table-brief.md), [ci-runs.md](ci-runs.md) | 2026-10-27 |
| **The link-on-the-way fix** — the brief under the human's ruling B, and the four review cycles: the review and three re-reviews, the fourth cycle allowed by the human | [link-on-the-way-brief.md](link-on-the-way-brief.md), [link-review.md](link-review.md), [link-rereview.md](link-rereview.md), [link-rereview2.md](link-rereview2.md), [link-rereview3.md](link-rereview3.md) | 2026-10-28 |
| **The readable plan, and this move** — the two briefs of 2026-10-29, the second this directory's own | [readable-plan-brief.md](readable-plan-brief.md), [repo-move-brief.md](repo-move-brief.md) | 2026-10-29 |
| **The scripts the build ran more than once**, kept as text with the question each answers, each with its row in [tooling.md](../../../../../implementation/tooling.md) → *Candidates* | [harvested-scripts.md](harvested-scripts.md) | 2026-10-27 to 2026-10-29 |

## Not kept — each group, and why

From the briefs folder and the plan folder, nothing was dropped: every file there is a brief, a review or a plan. From the scratch directory's 786 files, these were dropped, each checked against what the repository holds:

- **Clones of this repository** (four trees, about 450 files) **and single copies of its files** — the harness's source and generated form, the record script, the workflow doc, the record script's help, a section of `decisions-pending.md`, a file list: every one is in git at a commit.
- **A rig** (`dev/jigc-rig committed-singletons`): rebuilt by the rig in seconds.
- **Drafts of `DECISIONS.md` entries** (ten): each landed — its heading is in `DECISIONS.md`.
- **Drafts of commit messages** (twenty-one): each landed — its subject is in `git log --all`.
- **Drafts of committed records** — the byte table's record sections, its counts, `pinning.md` §7, the findings' sections, the opening sheet's §0 (line for line in the kept draft): the committed record is the version that stands.
- **One-off edit scripts** (`*_edit.py`, `e_*.py`, `rows.py` and the like): each applied one text replacement once, and its result is in a commit.
- **Earlier versions of a kept script, and one-off analyses** — four splitters and comparers of the byte table's print before `record-print.py`, the `B10` count and key analyses, an `awk` recipe for the record's counts: `record-print.py` prints every count the record keeps; the analyses' results are in the committed records.
- **The link reviews' probe trees** (four copies, 11 to 51 scripts each) but the harvested ones, **and the `B10` review's three rig scripts**: one rig per finding, each body the repro block in its review's text, and every finding since either fixed by a red-first test or recorded as a key for the run's ledger.
- **Build output** — the byte table's prints and their splits, per-cell and per-test seconds, the route catalog's site joins, key lists, grep and diff output, `VERB_KINDS` extracts: re-printable by the instrument that made each, at the commit it names. The measured sums the build relied on are in the committed byte-table record.
- **CI output**: none was in the scratch directory but the run-id lists, kept as [ci-runs.md](ci-runs.md).
- **Noise**: a 404 page from `gh`, a one-line path note, a sentence the main session applied to `CLAUDE.md` in `26474b80`.

## Sanitized

Every file is its source with these changes and no other: an absolute host path is written as a repository-relative path when it is one, otherwise a home path with `~`; a scratch path as `<scratch>`, another temporary path as `<tmp>`; the briefs folder's project directory as `<project>`; a session link or id as `<session>`. The scripts in [harvested-scripts.md](harvested-scripts.md) carry the same replacement, and [ci-runs.md](ci-runs.md) is two lists joined into a table. The guard's two scanners ran over every file before the commit (`dev/hygiene-scan --tree` with this machine's private denylist, and gitleaks), clean.
