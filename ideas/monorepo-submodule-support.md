# monorepo / submodule support — manage docs and route commits into the correct sub-repo

**Status: parked 2026-07-15.** From the human's trial scratch notes (the lacon-trial batch, [trial-record](../completions/artifacts/RC-lacon/trial-record.md)). Indexed from [VISION.md](../VISION.md) → Open questions.

## The gap

jigc assumes one repo, one store, one `docs-root`, one git boundary. A monorepo with submodules (or workspaces acting as sub-projects) breaks several load-bearing assumptions at once: where `.jigc/` lives, which git tree `finalize` commits into (a change spanning a submodule needs the commit *inside* the submodule plus the pointer bump outside), which `docs-root` a doctype resolves against per sub-project, and whether managed identity (`type:slug`) is repo-global or per-sub-project.

## The shape

Genuinely design-sized — this is not a knob. Sketch axes to settle at pickup: (a) **store topology** — one root store with per-sub-project scoping vs one store per sub-repo (the cascade's `team` layer is the natural cross-project precedent); (b) **finalize routing** — the CLI shells out to git already; committing into the sub-repo that owns the touched paths is mechanical *if* a task is bounded to one sub-repo (a bound worth imposing first — "one task, one repo boundary"); (c) **identity** — per-sub-project doc namespaces or a repo-global namespace with home-scoped resolution. The minimal honest first rung: detect the submodule case and say so (today the behavior is presumably undefined), then the one-task-one-sub-repo bound.

## Trigger

A real monorepo/submodule adopter — do not design ahead of one; the axes above are the questions to ask them.
