# Project setup — new & existing

How `jigc` wires into a real project from zero: the two **agent-followed setup workflows** that sit *on top of* the already-built `jigc setup` install command. This is M9 ([roadmap.md](../implementation/roadmap.md) → M9), the last spine milestone.

Builds on [bootstrap.md](bootstrap.md) (the orientation front door + the "this project isn't set up" state it routes from), [assistant-adapter.md](assistant-adapter.md) (the `jigc setup` *install* this composes over), [reconciliation.md](reconciliation.md) (the `file-state` substrate the ingestion classifier reuses — and the inverse problem it does **not** solve), [workflow-dialect.md](workflow-dialect.md) (the workflow front-matter + compose contract both flows ride), and [write-commands.md](write-commands.md) (the create-gate the new-project flow authors through). For the *why*, see [DECISIONS.md](../DECISIONS.md) (2026-06-06). Notation is **illustrative**.

## The boundary that frames everything: command vs workflow

There are two distinct things, and conflating them is the milestone's central trap:

| | What it is | Status |
|---|---|---|
| **`jigc setup`** — the *install command* | *Structure the CLI does:* generate the Claude Code adapter — the `@.jigc/AGENT.md` import into `CLAUDE.md`, the managed `.jigc/AGENT.md`, the `SessionStart` hook + `jigc` allowlist into `.claude/settings.json`, the `.jigc/config/` project-layer init, the spawn launch-template + its install-time validation. | **Built & proven** (M1 inc-6 + M8) |
| **the *setup workflows*** — `project-setup` (new) + `ingest-existing` (existing) | *Prose the agent follows:* the composed bootstrap/ingestion flow that runs **after** the install — develop an idea into the first managed doc, or detect-and-route an existing repo's docs. | **M9 — this doc** |

M9 does **not** re-touch the install machinery (idempotency, the structure-aware host-file merge, the adapter regeneration) — that is done and proven ([assistant-adapter.md](assistant-adapter.md)). M9 adds the *human-facing flows* the command deliberately does not do.

### What "set up" means (resolving the bootstrap.md over-promise)

"Set up" stays the predicate it is today: **`.jigc/config/` exists** — i.e. the bare `jigc setup` command has run ([bootstrap.md](bootstrap.md) → orientation states; the discriminator is the project-layer's presence). Running the *command* flips orientation from "unset" to "clean"; the setup **workflows** are then ordinary catalog workflows surfaced in the clean state, not a new terminal "fully-bootstrapped" state. This keeps the orientation model unchanged (no third state) and is consistent with the spike that a setup workflow composes cleanly on an already-"clean" project (nothing assumes "set up" is terminal).

The unset-project orientation copy is corrected accordingly ([bootstrap.md](bootstrap.md) → orientation state 1): it routes to `jigc setup` for the install and names the real `project-setup` / `ingest-existing` workflows as the next step — it no longer advertises an install command that "walks the pack choice" (**vacuous in a single-pack MVP** — there is one pack; pack *choice* stays deferred with multi-pack composition, [VISION.md](../VISION.md) → Open questions).

## Flow 1 — new project + idea development

The greenfield on-ramp: a fresh repo, `jigc setup` installs the adapter, then the agent develops a raw idea into the project's first managed document — a **`prd`** (product requirements).

### The `project-setup` workflow

A normal work-workflow, **pure pack data** (the proven work-workflow-add path — [workflow-dialect.md](workflow-dialect.md) → On-disk definition format; M9 baseline-verified that adding a workflow needs no engine change):

```yaml
# workflows/project-setup.yaml   (illustrative)
when: "Bootstrap a brand-new project — develop the idea into its first product requirements."
creates-task: true
allows-create: [{ type: prd, as: brief }]
# body: include-list
#   step:develop-idea     — reason about the idea; shape vision / requirements / context
#   step:author-prd       — create the prd via the create-gate, set its slots
#   step:project-finalize — finalize: validate → promote prd to prds/ → one docs(prd) commit
```

The `as: brief` binding only gates the `create` (it admits `prd` into this workflow and binds the doc to `task.brief`); **no M9 step reads `task.brief`** — unlike the MVP's `task.decision`, which a context-slice consumes — so the role name is for create-gate admission alone. It mirrors the proven `plan` shape (`creates-task: true`, create-gate author, code-less finalize — [roadmap.md](../implementation/roadmap.md) → M3 increment 3) — `plan` assumes you know the change and authors a `spec` for it; `project-setup` is the open-ended *idea-development* altitude above that, authoring the `prd` a later `plan`/`implement-from-spec` decomposes into. Reached via the front door once the project reads as set up: `jigc start --workflow project-setup "<idea>"` mints the task (or bare `jigc start` routes to it through the catalog, post-flip).

### The `prd` doctype — driven here, earned here

`prd` was named-but-unscheduled ([doctype-map.md](../implementation/doctype-map.md)); M9's idea-development flow is its real driver, so it earns its schema **from this workflow, now** (not pre-defined). Per the M9 scope decision (G2, [DECISIONS.md](../DECISIONS.md) 2026-06-06), requirements are **fixed prose slots, not a repeatable section** — this keeps `prd` **pure pack data with zero engine work** (a repeatable `requirements` section would hit the stubbed `add-item` authoring wall; deferred — see Deferred below):

```yaml
# schemas/prd.yaml   (illustrative)
type: prd
location: prds/
id-from: title
sections:
  - id: vision        # single-word ids ONLY (see the heading note below)
    slot: { hint: "The product vision in a sentence or two." }
  - id: requirements
    slot: { hint: "The product requirements, as a prose bullet list." }
  - id: context
    slot: { hint: "Constraints, audience, and what's out of scope." }
```

**Section ids must be single words.** A multi-word id (`out-of-scope`, `success-metrics`) trips a latent writer/parser round-trip defect that makes the whole doc non-reparseable (the writer title-cases `out-of-scope` → "Out Of Scope"; the parser does a flat case-compare against `out-of-scope` and fails). The defect is **logged with a trigger** ([decisions-pending.md](../implementation/decisions-pending.md)), **not fixed in M9** — `prd`'s single-word slots route around it. This is a real constraint on the schema, recorded so the build doesn't discover it mid-task.

**No `decomposes-into` edge authored here.** `prd —decomposes-into→ spec` ([doctype-map.md](../implementation/doctype-map.md)) is the *spec-side* `derived-from` relation (inverse derived, never stored — [document-type-schema.md](document-type-schema.md)); with prose-slot requirements there are no per-requirement anchors to point from, so M9's prd authors no ref edge. The edge activates if/when a spec-side flow writes `derived-from` (unscheduled).

### Finalize

Exactly the code-less, doc-only finalize the `plan` flow already proves ([finalize.md](finalize.md) → Promote; M9 baseline-verified end-to-end for a brand-new `prd`): the promoted `prds/<slug>.md` is the non-empty diff (empty-commit guard satisfied), commit `type: docs`, one `docs(prd): …` commit lands.

## Flow 2 — existing project: bounded detect-and-route ingestion

The brownfield on-ramp: a repo whose docs are in inconsistent states. The honest, acceptance-testable slice is **detect-and-route** — discover candidate docs, classify each against the managed schemas, and route the verdict — **never auto-migrate**. Auto-migration (rewriting a non-conformant doc into conformant shape) is the research-grade core flagged in [VISION.md](../VISION.md) → Open questions; it is **deferred with a trigger** ([decisions-pending.md](../implementation/decisions-pending.md)), because the canonical parser is strict-by-design and cannot fuzzy-map a foreign heading onto a schema section without machinery the determinism boundary deliberately withholds.

### Why this is net-new, not a reconciliation reuse

[reconciliation.md](reconciliation.md)'s `file-state` machinery solves the **inverse** problem: drift of a doc `jigc` *already minted* (known type, known path, recorded baseline). Ingestion is the forward problem: an **arbitrary foreign `.md`** with no type binding and no baseline. Two facts make the reuse a trap (both M9-baseline-verified):

1. **The `UNKNOWN → baseline-adopt` path is a safety hole.** Today a doc with no recorded hash dropped under a schema's `location:` is **silently adopted as managed without a schema check** (a freeform `notes.md` in `decisions/` was adopted as an ADR). Ingestion must **gate adoption behind a conformance check** — adopt only what classifies conformant; never silently absorb. **Two paths reach this branch, and M9 closed only one (the G4 correction — M21):** the `jigc ingest` adopt action *does* gate on conformance (M9, below), but the **`finalize`/`validate` committed-store sweep** (`reconcile_committed_store` → `reconcile_committed`'s `UNKNOWN` arm, `crates/engine/src/file_state.rs`) still records the hash of *any* `.md` under a `location:` dir **with no schema check** — so a foreign file dropped into `decisions/` is silently baseline-adopted at the next finalize. **M21 closes this second path** (the G4 fix): the `UNKNOWN` arm runs `parse_sections` + `schema_conformance` (the same call its sibling DRIFTED arm already makes) before recording, and routes a non-conformant file as an **advisory** finding instead of adopting it — advisory, not blocking, so a conformant jigc-minted doc on a fresh checkout (the legitimate `UNKNOWN` case the branch was built for) still baselines cleanly and the M20 clean-store guarantee holds.
2. **Discovery is location-scoped.** `jigc` only globs declared `location:` dirs (`decisions/`, `specs/`, `prds/`); it never scans root / `docs/` / `README.md`. Repo-wide candidate discovery is net-new.

### The bounded slice — three net-new but tractable pieces

The reusable substrate is exactly two public engine functions — `parse::parse_sections(schema, src)` and `validate::schema_conformance` — which already answer *"does this doc conform to schema X, and if not, precisely why?"* ([validation.md](validation.md), [parsing.md](../implementation/parsing.md)). The slice composes them:

1. **Repo-wide discovery** — enumerate candidate markdown files beyond the declared `location:` dirs (root, `docs/`, the location dirs themselves). Bounded, conventional file-walking, **sorted** (the existing dir-read sort discipline, so the verdict report's row order is deterministic — same repo in → same verdicts out).
2. **The N-candidate classifier** — for each discovered file, run it against each persisted schema (`parse_sections` + `schema_conformance`) and reduce to a verdict. **Location is part of the discriminator**, because the committed store only ever *finds* a managed doc by globbing its schema's `location:` dir (`reconcile_committed_store`), so a doc registered outside its location is invisible to every later sweep:
   - **`adoptable`** — parses conformant against exactly one schema **and already sits under that schema's `location:` dir** (e.g. a conformant `adr` at `decisions/x.md`).
   - **`needs-reconcile`** — under a schema's `location:` dir but **fails parse-or-conformance** (a near-miss claiming to be that type), **or** conformant against a schema but **sitting at the wrong location** (a conformant `adr` at `docs/x.md` — claims the type, wrong home). Routed to a human; never auto-relocated.
   - **`unmanaged`** — outside every `location:` dir and parses-conformant against nothing. Left untouched.

   No fuzzy mapping; a file either parses canonically against a schema or it does not (the two gates `parse_sections` → `Err(findings)` and `schema_conformance` → `findings` stay distinct). Location is the discriminator that makes the reduction closed and decidable.
3. **The adopt action — net-new, NOT reconciliation's `baseline-adopt`.** Reconciliation's `UNKNOWN → baseline-adopt` ([reconciliation.md](reconciliation.md)) records a hash with **no schema check and no edge-index population** (it assumes a jigc-minted doc whose edges were indexed at create-time) — that is exactly the safety hole ingestion must *not* reuse. Adopting a foreign `adoptable` doc is a distinct action: **parse → conformance-gate → `index.absorb_doc` (so an adopted `adr`'s `supersedes` edge is indexed) → record the file-state hash**. Adopt is **register-only — it never moves or rewrites a file** (consistent with detect-and-route: a misplaced-but-conformant doc is `needs-reconcile`, a human relocates it, jigc never auto-moves). 
4. **The verdict/triage surface** — a report of `file × best-match type × verdict`, with `needs-reconcile` rows carrying a routed [finding](validation.md) (the existing finding/route shape — severity + located message + route), so a human resolves each exactly as OOB conflicts route today.

The agent-facing composition is the **`ingest-existing` workflow** (pure pack data, `creates-task: false` orient/route shape — like `router`): it walks the agent through running the scan, reading the verdicts, adopting the conformant docs, and routing the non-conformant ones to the human. The scan itself is a CLI command (`jigc ingest`, the engine classifier + discovery + triage report) — its exact verb surface, the adopt-confirmation shape, and whether discover/classify and adopt are one verb or two are **illustrative here and pinned at the build's acceptance-flow spike** against the built command grammar ([worked-examples.md](worked-examples.md) → flow 12).

### Idempotency & irreversibility — the acceptance obligation

Setup and ingest write to a user's **real repo**, so the proofs that matter are *non-destructive* and *idempotent*:
- `jigc setup` re-run is a byte-identical no-op; a pre-existing non-trivial `CLAUDE.md` / `.claude/settings.json` is **merged, never clobbered** (structure-aware) — proven at the unit level today, lifted to a **binary-level acceptance test** in M9 (its cheapest irreversibility proof).
- `jigc ingest` **adopts nothing it has not schema-checked** (closing the `baseline-adopt` hole) and **rewrites no prose** (detect-and-route, never auto-migrate). The non-conformant case routes to a human; the unmanaged case is left untouched.

## Flow 2 hardening — full ingest/adopt/cleanup (M21)

M9 shipped the **detect-and-route** core; M21 hardens Flow 2 into a real existing-project on-ramp for the live test ([DECISIONS.md](../DECISIONS.md) 2026-06-14), with one piece (auto-migration) explicitly held back to its own milestone. What M21 adds:

- **The G4 conformance gate** on the finalize/validate store sweep (above) — the second `UNKNOWN → baseline-adopt` path now conformance-checks before adopting, routing non-conformant as advisory. This is the one correctness bug that bites **both** flows.
- **Recursive scan + scan-root (G3).** M9's discovery is non-recursive over a fixed dir set (root + `docs/` + `location:` dirs, top-level only), so docs in `wiki/`, nested `docs/sub/`, `rfcs/`, etc. are invisible. M21 makes discovery **recursive** (keeping the `.jigc/` exclusion and the `BTreeSet` sort-determinism) so a real repo's docs-elsewhere corpus is reachable.
- **Off-catalog discoverability (G6).** Two entry workflows are reachable today only if you already know the verb: `ingest-existing` (`creates-task: false`) and **`planning`** (`creates-task: true, selectable: false` — deliberately off the router catalog by the M16 invariant, [methodology-docs.md](methodology-docs.md)). The router catalog lists only `selectable` work-workflows, so **a bare `jigc start` never surfaces `planning`** — which means "milestones out of the box" is *not* delivered by the router alone. M21 surfaces **both** from orientation (a route-prose line naming the verb, the same shape for each), so a human/agent on a fresh setup finds *both* the existing-project on-ramp **and the milestone-planning verb** without already knowing them — **without** flipping `planning` selectable (the M16 off-router invariant holds). This is what actually delivers the milestone half of the done-picture; the router default (`router`) delivers vision + MVP.
- **Teardown / cleanup (G5) — two verbs, both repo-local.** No teardown exists today (only `task discard` removes a working area). M21 builds: **(a) un-manage a doc** — drop a single managed doc from the file-state record + its `edges.json` entries, leaving the file on disk (the inverse of `adopt`'s register-only mutation); and **(b) `jigc uninstall`** — **repo-local** teardown of *this project's* jigc install (remove `.jigc/`, unwire the `CLAUDE.md` import line + the `.claude/settings.json` allowlist entry + the setup composition marker). **`uninstall` does NOT remove the `doc-code` probe sibling** — that is the one **machine-global** setup write (`current_exe().parent()/doc-code`, shared by every jigc project on the machine — [module-layout.md](../implementation/module-layout.md) → Probe distribution); deleting it would break doc↔code validation for *other* repos. Machine-global removal is `cargo uninstall jigc` + manual probe removal, never the per-project verb. Both verbs write a user's real repo, so they carry an explicit **idempotency + non-destructive** acceptance: un-manage touches only jigc's own index/state (asserted: the doc's bytes on disk are unchanged) and is idempotent (re-run on an already-unmanaged doc is a clean no-op, not an error); `uninstall` removes exactly the enumerated repo-local setup-created set and is idempotent (a second run is a clean no-op).

**The honest Flow-B bound (scope clarity).** With G1 (auto-migration) and G2 (doctype coverage) deferred, M21's `adopt` still reaches **only docs already conformant to a shipped doctype** (`adr`/`prd`/`spec`/`arch-doc` + the methodology running-docs). For a foreign project *not* built with jigc, that conformant subset is often **near-empty** — its README/CHANGELOG/runbooks/freeform-ADRs map to nothing and route as `unmanaged`/`needs-reconcile`. So M21 Flow-B delivers **recursive scan + classify + route + the G4 safety gate + teardown + discoverability** — an honest "here is your doc landscape and the safe on/off ramps" — **not** broad "track every doc" (that arrives with the deferred doctype-expansion + auto-migration milestones). The existing-project **live test is designed against this real capability** (cleanly classify and route a foreign corpus, adopt the conformant subset, tear down), not against an overclaimed tracking surface.

A second consequence of the G4 gate, stated so it isn't a surprise: a non-conformant foreign doc squatting in a `location:` dir is **routed (advisory) but not recorded**, so the advisory **re-fires at every finalize** until the human resolves it (ingest / migrate / move it out of the location dir). That recurrence is intended — it is the honest "an unvetted doc is sitting in `decisions/`" signal — and the advisory's route names the corrective verb. The finding renders distinctly in the finalize report (not swallowed under a clean exit).

**Deferred from M21 to two separate post-M21 milestones (split 2026-06-14, [DECISIONS.md](../DECISIONS.md) → 2026-06-14 split):** **doctype coverage** (G2 — new doctypes like `changelog` for README/changelog/runbook-class docs, each driven by a real workflow) ships **first**, serving Flow A (a *new* project authors the managed doc through jigc — no migration needed); **auto-migration** (G1 — rewriting a foreign doc into conformant shape; research-grade, pressures the determinism boundary) ships **after**, since the G2 doctypes are its migration targets and it carries the isolated determinism-boundary risk. Together they make Flow B's "track an existing `CHANGELOG.md`" real (a `changelog` doctype can't track an existing file without migration to conformant shape) — but each earns its own verdict ([decisions-pending.md](../implementation/decisions-pending.md)). README stays **never a doctype** (bespoke front-page prose, no schema-able structure).

## Scope boundary (what M9 does **not** do — partially superseded by M21)

- **No auto-migration** of non-conformant docs (research-grade — deferred with trigger; **still deferred past M21**).
- ~~**No multi-pack / pack-choice** selector~~ — **superseded at M21**: a real project now composes **dev + methodology** by default at setup ([multi-pack.md](multi-pack.md) → Embedded second pack + setup auto-wiring). The packs are *auto-wired, not chosen* — there is still **no pack-choice selector** (that stays deferred with the public pack-authoring surface).
- **No `prd` repeatable-requirements authoring** / `add-item` verb (deferred; prd uses prose slots).
- **No second assistant profile** (Claude Code only — [assistant-adapter.md](assistant-adapter.md) → Open questions stays deferred).
- **No new fan-out / override / severity surface** — both flows are ordinary compose + create/author + finalize (new) or scan + classify + route (existing). *(M21's teardown verbs are read/state-only — no new override or severity surface either.)*

## Open questions

- **Auto-migration of inconsistent docs** — the deferred research-grade core; fires when a real corpus + a measured migration-quality baseline justify the tolerant/fuzzy-mapping machinery ([VISION.md](../VISION.md) → Open questions; [decisions-pending.md](../implementation/decisions-pending.md)).
- **`jigc ingest` exact verb surface + adopt-confirmation** — pinned at the build's acceptance-flow spike against the built command grammar.
- **`prd` repeatable requirements + the `decomposes-into` edge** — when a flow must author structured, individually-addressable requirements (needs the deferred `add-item` verb + the multi-word-heading parser fix).
