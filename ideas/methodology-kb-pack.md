# The KB as a jigc pack — a non-code domain stress test for the pack platform

**Status: parked 2026-07-02, unscheduled.** From the KB/research comparison. Indexed from [VISION.md](../VISION.md) → Open questions. Explicitly behind the "not yet a public pack platform" non-goal — this is a candidate *first tenant*, not a schedule.

## The shape

The human's project-structure KB (`~/ideas/claude-project-structure/kb`) is itself jigc-shaped: rule files with a strict per-rule format (kebab-slug + imperative + `Why:` + `Src:` provenance), a rebut-through-the-why protocol backed by decision records, filenames-as-index. As a jigc pack: a rule-file doctype with a one-level repeatable `rules` section (title + imperative body slot + why slot + src field + optional tags enum — expressible in the frozen schema format); the rebuttal protocol as a workflow with a create-gate for a decision-record doctype, so "never edit a rule silently" becomes **structurally enforced** (edit blocks at finalize without the paired decision doc — a real upgrade over prose discipline).

## What it would need (the honest bill)

(i) A **home decision** — the KB is cross-project; jigc manages per-repo (the KB repo becomes the managed store; "read at every project start" then needs the team-layer or a pack-distribution story). (ii) Corpus migration, including re-slugging the `docs/decisions/NNNN` numbered ids via `jigc rename` (the no-renumbering identity model). (iii) `Src:` as managed ref only if decision records co-manage, else string. (iv) It is a third pack — i.e., exactly the deferred pack-authoring surface.

## Trigger

The pack-platform non-goal's own release condition fires (an external domain earns a pack + the authoring API stabilizes) — and this KB volunteers as the non-code proving ground.
