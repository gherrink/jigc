# Branches and planned releases — the repo's branch model, as jigc features

**Status: parked 2026-10-01, keyed to the port (M56).** Parked by the human's decision 1 of the branching switch ([DECISIONS.md](../DECISIONS.md) → *the branching switch*): the branch model is this repository's **convention now** — [CLAUDE.md](../CLAUDE.md) → Branches, built into the [`milestone-build`](../.claude/workflows/milestone-build.js) harness — and whether any of it becomes **product** is for the port to show. Indexed from [VISION.md](../VISION.md) → Open questions; the trigger row is [decisions-pending.md](../implementation/decisions-pending.md) → *The branching switch*.

## What exists, as convention

- **Names from a closed set** — `main`, `milestone/<slug>/main`, `milestone/<slug>/<increment-slug>`, `fix/<slug>`, `work/<slug>`, release-plz's `release-plz-*` — each slug in jigc's slug grammar, held by [`dev/branch-name`](../dev/branch-name) in CI.
- **A branch lifecycle per work unit** — the milestone branch forks from `main`; each increment forks from the milestone branch, is built and validated there, and lands back with `--no-ff`; the milestone reaches `main` as one pull request the human merges ([increment-workflow.md](../implementation/increment-workflow.md) → Branches).
- **A release boundary that is a decision** — the release PR accumulates milestone merges and is merged when the planned set is in, with `release-plz set-version` when the derived number differs from the plan ([release.md](../implementation/release.md) → The release PR).

None of it is jigc. The harness mints the names, an agent step runs the git commands, and the release is a human reading a release PR.

## The two directions

**1 · Branching as a jigc capability.** The `milestone > increment > task` hierarchy already exists as jigc work-units ([structural-grammar.md](../design/structural-grammar.md) → Work-units), and the `commit` doctype already shows the shape of a structured, schema-held artifact whose sink is git. A branch-naming **pattern** declared beside it — the unit's kind and slug rendered into a ref name, checked the way a doctype's fields are — and **workflow steps that create and join branches** at a unit's open and close would move the convention into the CLI, on the same terms as everything else it owns: the CLI orchestrates, git executes ([VISION.md](../VISION.md) → the determinism boundary). The fan-out already isolates sub-agent code in per-task worktrees (M31), so the join rule is not new ground; [conflict-free-parallel-writing.md](conflict-free-parallel-writing.md) is the harness-side sibling, and [sequential-milestone.md](sequential-milestone.md) is the brownfield finding that a dependency-spined milestone fits a branch-per-increment spine better than a fan-out.

**2 · Planned releases as a layer above milestones.** A **release** that owns its own milestone list — the set that ships together — would make the release boundary a managed fact rather than a human's reading of a release PR: which milestones are planned into it, which have landed, whether the derived version matches the planned one. Today that lives in the human's head and in release-plz's PR.

## Rationale for parking, not building

- **The governing rule** ([CLAUDE.md](../CLAUDE.md) → Project state): *we do not bend the CLI to fit the project; we migrate the project to fit the product.* A branch model invented for this repository and then shipped as product would be exactly the inversion that rule forbids. So it is lived as convention first.
- **The port is the test.** M56 migrates this repository onto jigc. If running milestones through jigc wants the branch steps and the names inside the CLI, the port will hit it, and every agent doing the port reports through M55's findings channel — the residue arrives as ledger rows, not as an argument.
- **Nothing here is a one-way door yet.** Branch names are not in any frozen schema, and a release layer adds a work-unit rather than changing one.

## Trigger

**M56's Settle**, reading the findings ledger for branch- and release-shaped residue from the port. Absent any, the convention stays a convention and this file says so.
