# route self-check — every refusal a customized project can meet prints a route that works there

**Status: parked 2026-10-11.** The human's idea, during the build of the stabilization workflow's reshaped `test` stage ([DECISIONS.md](../DECISIONS.md) → *2026-10-12 — The triage's definition follows the reshaped `test` stage*, where the parking is recorded). Not scheduled; nothing here is built. Indexed from [VISION.md](../VISION.md) → Open questions.

> **Notation is illustrative.** This is a parked idea: the verb, its name and its output are sketches, and the docs it names own their rules.

## The gap

The route catalog follows the route of every blocking refusal of a command in a throwaway repository and says whether doing what the route prints ends the refusal. It is a suite of this repository's product tests, over the committed list of refusal sites ([sites.tsv](../crates/cli/tests/route_catalog/sites.tsv)), so it protects the packs and the configuration jigc ships. *Its pilot — two commands, `setup` and `uninstall` — is built on the branch `fix/rc24-route-catalog` and is not in this tree yet: the suite, the pilot's record (`route-catalog-pilot.md`, beside the build's plan) and the section of [pinning.md](../implementation/pinning.md) that owns the catalog arrive with that branch's merge.*

A project that reshapes jigc — its own doctypes and workflows, shadowed definitions, a team layer, later a pack of its own — is outside it. A route jigc prints there names things the project defined: a workflow, a doctype, an address, a command reference. The class of defect the pilot found in the shipped surface can be made by a customization just as well — **when the pilot's record was written, on 2026-10-11, 170 of its 353 cases were red**; the count has moved since and will move with every fix, and that record is its home — and the project's author has no instrument for it.

## The direction

**jigc carries the check itself**: a verb (a `doctor`-shaped read, or a mode of `validate`) that, for the resolved cascade of the project it runs in, provokes the refusals that cascade can produce, in a sandbox, follows each printed route, and reports the ones that do not work. Deterministic, no model, nothing written to the project.

What of it can be fully automated, and what cannot:

- **Can — refusals whose provoking state follows from a definition.** A required slot left empty, a malformed value, a reference that dangles, a create the workflow does not allow, a command reference that does not resolve: for each doctype and workflow the cascade resolves, the state is derivable from the schema or the workflow's front-matter, so a case can be *generated* — one per definition and refusal kind — and followed. This is the part a customization changes, and the part that needs no hand.
- **Can — running cases that are data.** A case of the catalog is data: the state, the refusing argv, the acts whose operand is a span of the printed route, the outcome, the witnesses. Shipped with the binary, the built-in cases run against the project's configuration as they run against the default one.
- **Cannot — finding the causes.** For a refusal that depends on the environment — what git reports, a path of the wrong kind, a broken install — the causes were found by a builder reading the site's callee, site by site — minutes each, and the pilot's record has the figures. That stays authored, in this repository, and ships as data.
- **Cannot — judging prose.** 187 of 241 routed sites print prose and no argv ([route-registry-facts.md](../completions/artifacts/M55/stabilization-build/reshape/route-registry-facts.md)). The check can follow only what a route prints as an act; a route that prints no operand is followed by a natural act the case's author chose.

## What it would cost

- The frame out of the test tree into something the binary can run: the sandbox (an isolated home and git configuration), the state steps, the classifier. Today it is test code that drives the built binary.
- Case generation from a resolved cascade — a new mechanism, beside `workflow-refs` and the store-scope checks it resembles.
- A contract for its output — a findings envelope like every other ([command-output-contract.md](../design/command-output-contract.md)) — and a decision whether it is a pack author's tool only or also something `setup` or an upgrade runs.

## Why parked

Before 1.0.0 nothing new is built that is not a fix ([DECISIONS.md](../DECISIONS.md) → *2026-10-04 — The exit rule, revised*). It also presupposes what does not exist yet: a pack-authoring surface for third parties ([CLAUDE.md](../CLAUDE.md) → Non-goals), and a catalog that covers more than two commands — the rest of the shipped families is scheduled before the port begins ([decisions-pending.md](../implementation/decisions-pending.md) → `wf-route-catalog-rest`).

## Trigger

The first external or project-local pack whose author reports a route that led nowhere; or the port (M56), if migrating this repository onto jigc shows a customized cascade printing routes the shipped catalog cannot see.
