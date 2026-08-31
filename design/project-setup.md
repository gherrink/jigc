# Project setup — new & existing

How `jigc` wires into a real project from zero: the two **agent-followed setup workflows** that sit *on top of* the already-built `jigc setup` install command. This is M9 ([roadmap.md](../implementation/roadmap.md) → M9), the last spine milestone.

Builds on [bootstrap.md](bootstrap.md) (the orientation front door + the "this project isn't set up" state it routes from), [assistant-adapter.md](assistant-adapter.md) (the `jigc setup` *install* this composes over), [reconciliation.md](reconciliation.md) (the `file-state` substrate the ingestion classifier reuses — and the inverse problem it does **not** solve), [workflow-dialect.md](workflow-dialect.md) (the workflow front-matter + compose contract both flows ride), and [write-commands.md](write-commands.md) (the create-gate the new-project flow authors through). For the *why*, see [DECISIONS.md](../DECISIONS.md) (2026-06-06). Notation is **illustrative** ([what that disclaims](../CLAUDE.md#how-we-work-together)).

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
#   step:author-commit    — the pack's one commit-doc solicit (type + summary)
#   step:project-finalize — finalize: validate → promote prd to prds/ → one docs(prd) commit
```

`step:author-commit` sits at the **workflow** level, not inside `project-finalize.yaml` — that file is *only* `{{ include: step:finalize }}`, and the 12 migrate workflows reach the same `step:finalize` through `migration-finalize.yaml`, whose mint already pre-fills the commit doc. Pushing the solicit down into the shared commit half would hand every migration a second, redundant solicitation, and the dialect has no conditionals (M47 Increment 5; [DECISIONS.md](../DECISIONS.md) → 2026-07-26 the Settle, Decision 5).

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

### The secrets-floor `.gitignore` — fresh-repo install only (M36)

`jigc setup` seeds a **root `.gitignore` secrets floor** so a brand-new repo can't commit credentials before the human has thought about it. It is a **safety floor, not a general `.gitignore`** — the same secret set the adapter's `deny` floor blocks ([assistant-adapter.md](assistant-adapter.md) → the `deny` safety floor), one list with two enforcement points:

```gitignore
# jigc secrets floor — safe defaults; edit freely
.env
.env.*
!.env.example        # keep a committed template
*.pem
*.key
id_rsa
id_rsa.*
id_ed25519
id_ed25519.*
credentials
.npmrc
```

**Fresh-repo path only — never touch an existing project's `.gitignore`.** Two rules pin this:

- **The discriminator is "no commits yet," not "no `.gitignore`."** Setup seeds the floor **iff the repo has zero commits** (`git rev-list --count HEAD == 0` — an unborn or as-yet-empty HEAD). Setup has **no fresh-vs-existing discriminator today** (the [boundary table](#the-boundary-that-frames-everything-command-vs-workflow) lists only unconditional installs), so this signal is net-new. The tempting proxy — *root `.gitignore` absent → fresh* — is **wrong and unsafe**: an established project that merely lacks a `.gitignore` would get a secrets file dropped into it, a direct violation of *never touch existing*. Commit-count is the signal that actually separates greenfield from established. **Conservative by design:** a repo with a single bootstrap commit reads as *existing* under this signal and is left alone — an intentional false-negative (a missed floor is cheap; seeding into someone's established repo is the scope breach we refuse). "Fresh" ≠ "unborn HEAD" strictly — it is "no commits," which a bootstrap-only repo *fails*, and that is the safe side to err toward. Any case where the signal can't be read cleanly — not a git repo, `git rev-list` errors — resolves the same conservative way: **do not seed** (a missing floor never breaks anything; a wrongful seed is the failure we avoid).
- **On the fresh path, still merge — never clobber.** A fresh repo rarely carries a root `.gitignore`, but if one exists (bootstrap tooling wrote it), setup **appends the missing floor lines under a sentinel-marked block** — the same idempotent, non-destructive discipline as the host-file merges ([Idempotency & irreversibility](#idempotency--irreversibility--the-acceptance-obligation)) — never rewriting the human's lines. Absent → create; present → append-if-absent; re-run → byte-stable no-op.

This is the **third assistant-neutral install responsibility** ([assistant-adapter.md](assistant-adapter.md) → `jigc setup` has assistant-*neutral* install responsibilities too: the project-layer init + the doc↔code `pre-commit` backstop are the other two) — a Cursor or Codex project wants it identically, so it lives on `setup`'s neutral path, not in the per-assistant profile.

## Flow 2 — existing project: bounded detect-and-route ingestion

The brownfield on-ramp: a repo whose docs are in inconsistent states. The M9/M21 slice is **detect-and-route** — discover candidate docs, classify each against the managed schemas, and route the verdict — **never rewrite**. The **transform** arm (rewriting a non-conformant doc into conformant shape + adopting it) ships as **auto-migration (G1), M23** — changelog-first ([auto-migration.md](auto-migration.md)). It does **not** relax the determinism boundary: the canonical parser stays strict-by-design and the CLI never fuzzy-maps a foreign heading; instead the **LLM rewrites** the foreign prose to canonical shape through the existing write verbs and the **CLI strict-parses + adopts iff conformant** (Framing A — [VISION.md](../VISION.md#the-determinism-boundary) table unchanged).

### Why this is net-new, not a reconciliation reuse

[reconciliation.md](reconciliation.md)'s `file-state` machinery solves the **inverse** problem: drift of a doc `jigc` *already minted* (known type, known path, recorded baseline). Ingestion is the forward problem: an **arbitrary foreign `.md`** with no type binding and no baseline. Two facts make the reuse a trap (both M9-baseline-verified):

1. **The `UNKNOWN → baseline-adopt` path is a safety hole.** Today a doc with no recorded hash dropped under a schema's `location:` is **silently adopted as managed without a schema check** (a freeform `notes.md` in `decisions/` was adopted as an ADR). Ingestion must **gate adoption behind a conformance check** — adopt only what classifies conformant; never silently absorb. **Two paths reach this branch, and M9 closed only one (the G4 correction — M21):** the `jigc ingest` adopt action *does* gate on conformance (M9, below), but the **`finalize`/`validate` committed-store sweep** (`reconcile_committed_store` → `reconcile_committed`'s `UNKNOWN` arm, `crates/engine/src/file_state.rs`) still records the hash of *any* `.md` under a `location:` dir **with no schema check** — so a foreign file dropped into `decisions/` is silently baseline-adopted at the next finalize. **M21 closes this second path** (the G4 fix): the `UNKNOWN` arm runs `parse_sections` + `schema_conformance` (the same call its sibling DRIFTED arm already makes) before recording, and routes a non-conformant file as an **advisory** finding instead of adopting it — advisory, not blocking, so a conformant jigc-minted doc on a fresh checkout (the legitimate `UNKNOWN` case the branch was built for) still baselines cleanly and the M20 clean-store guarantee holds.
2. **Discovery is location-scoped.** `jigc` only globs declared `location:` dirs (`decisions/`, `specs/`, `prds/`); it never scans root / `docs/` / `README.md`. Repo-wide candidate discovery is net-new.

### The bounded slice — three net-new but tractable pieces

The reusable substrate is exactly two public engine functions — `parse::parse_sections(schema, src)` and `validate::schema_conformance` — which already answer *"does this doc conform to schema X, and if not, precisely why?"* ([validation.md](validation.md), [parsing.md](../implementation/parsing.md)). The slice composes them:

1. **Repo-wide discovery** — enumerate candidate markdown files beyond the declared `location:` dirs (root, `docs/`, the location dirs themselves). Bounded, conventional file-walking, **sorted** (the existing dir-read sort discipline, so the verdict report's row order is deterministic — same repo in → same verdicts out).

   **M40 re-bases the candidate set on git** ([DECISIONS.md](../DECISIONS.md) → 2026-07-10 M40 planning: Settle): the CLI computes candidates via **`git ls-files -z --cached --others --exclude-standard -- '*.md'`** (NUL-terminated, so a non-ASCII filename is never `core.quotepath`-C-quoted into a bogus literal path) — gitignored files **excluded** (the adoption trial listed `node_modules` `.md`s as candidates — 91% of the logged triage output), untracked-but-not-ignored files **included**. The internals prune stays on top (`.jigc/AGENT.md` *does* appear in `ls-files`), and the output is unsorted, so the `BTreeSet` re-sort re-imposes the same-repo-in → same-verdicts-out determinism. **No no-git fallback**: ingest already hard-requires a git repo (the repo-root discovery bails without one), so a raw-walk fallback would be dead code with divergent semantics; the engine's filesystem walk survives as **engine-test substrate only**. Git stays CLI-only — the engine shells nothing; the one production call site already lives in the CLI.
2. **The N-candidate classifier** — for each discovered file, run it against each persisted schema (`parse_sections` + `schema_conformance`) and reduce to a verdict. **Location is part of the discriminator**, because the committed store only ever *finds* a managed doc by globbing its schema's `location:` dir (`reconcile_committed_store`), so a doc registered outside its location is invisible to every later sweep:
   - **`adoptable`** — parses conformant against exactly one schema **and already sits under that schema's `location:` dir** (e.g. a conformant `adr` at `decisions/x.md`).
   - **`needs-reconcile`** — under a schema's `location:` dir but **fails parse-or-conformance** (a near-miss claiming to be that type), **or** conformant against a schema but **sitting at the wrong location** (a conformant `adr` at `docs/x.md` — claims the type, wrong home). Routed to a human; never auto-relocated.
   - **`unmanaged`** — outside every `location:` dir and parses-conformant against nothing. Left untouched.

   No fuzzy mapping; a file either parses canonically against a schema or it does not (the two gates `parse_sections` → `Err(findings)` and `schema_conformance` → `findings` stay distinct). Location is the discriminator that makes the reduction closed and decidable.
3. **The adopt action — net-new, NOT reconciliation's `baseline-adopt`.** Reconciliation's `UNKNOWN → baseline-adopt` ([reconciliation.md](reconciliation.md)) records a hash with **no schema check and no edge-index population** (it assumes a jigc-minted doc whose edges were indexed at create-time) — that is exactly the safety hole ingestion must *not* reuse. Adopting a foreign `adoptable` doc is a distinct action: **parse → conformance-gate → `index.absorb_doc` (so an adopted `adr`'s `supersedes` edge is indexed) → record the file-state hash**. Adopt is **register-only — it never moves or rewrites a file** (consistent with detect-and-route: a misplaced-but-conformant doc is `needs-reconcile`, a human relocates it, jigc never auto-moves). 
4. **The verdict/triage surface** — a report of `file × best-match type × verdict`, with `needs-reconcile` rows carrying a routed [finding](validation.md) (the existing finding/route shape — severity + located message + route), so a human resolves each exactly as OOB conflicts route today.

   **M40 adds the adopt-time triage-annotation vocabulary** ([validation.md](validation.md) → Hollow and surplus adoption): an adopted row may carry an **advisory annotation** — *"adopted — structurally empty: 0 \<items\>"* (`schema-conformance.repeatable-populated`) or *"adopted — N surplus trailing sections"* (`schema-conformance.surplus-sections-absent`). Annotated rows still adopt — the annotation is visibility, never a gate — and it is **serialized in the triage row in every output format**, not just the human render (the agent path is the dominant consumer).

The agent-facing composition is the **`ingest-existing` workflow** (pure pack data, `creates-task: false` orient/route shape — like `router`): it walks the agent through running the scan, reading the verdicts, adopting the conformant docs, and routing the non-conformant ones to the human. The scan itself is a CLI command (`jigc ingest`, the engine classifier + discovery + triage report) — its exact verb surface, the adopt-confirmation shape, and whether discover/classify and adopt are one verb or two are **illustrative here and pinned at the build's acceptance-flow spike** against the built command grammar ([worked-examples.md](worked-examples.md) → flow 12).

### Idempotency & irreversibility — the acceptance obligation

Setup and ingest write to a user's **real repo**, so the proofs that matter are *non-destructive* and *idempotent*:
- `jigc setup` re-run is a byte-identical no-op; a pre-existing non-trivial `CLAUDE.md` / `.claude/settings.json` is **merged, never clobbered** (structure-aware) — proven at the unit level today, lifted to a **binary-level acceptance test** in M9 (its cheapest irreversibility proof).
- `jigc ingest` **adopts nothing it has not schema-checked** (closing the `baseline-adopt` hole) and **rewrites no prose** (detect-and-route, never auto-migrate). The non-conformant case routes to a human; the unmanaged case is left untouched.

## Flow 2 hardening — full ingest/adopt/cleanup (M21)

M9 shipped the **detect-and-route** core; M21 hardens Flow 2 into a real existing-project on-ramp for the live test ([DECISIONS.md](../DECISIONS.md) 2026-06-14), with one piece (auto-migration) explicitly held back to its own milestone. What M21 adds:

- **The G4 conformance gate** on the finalize/validate store sweep (above) — the second `UNKNOWN → baseline-adopt` path now conformance-checks before adopting, routing non-conformant as advisory. This is the one correctness bug that bites **both** flows.
- **Recursive scan + scan-root (G3).** M9's discovery is non-recursive over a fixed dir set (root + `docs/` + `location:` dirs, top-level only), so docs in `wiki/`, nested `docs/sub/`, `rfcs/`, etc. are invisible. M21 makes discovery **recursive** (keeping the `.jigc/` exclusion and the `BTreeSet` sort-determinism) so a real repo's docs-elsewhere corpus is reachable. *(M40 re-bases the enumeration itself on `git ls-files` — see the Flow-2 discovery item above; the recursive raw walk survives as engine-test substrate only.)*
- **Off-catalog discoverability (G6).** Two entry workflows are reachable today only if you already know the verb: `ingest-existing` (`creates-task: false`) and **`planning`** (`creates-task: true, selectable: false` — deliberately off the router catalog by the M16 invariant, [methodology-docs.md](methodology-docs.md)). The router catalog lists only `selectable` work-workflows, so **a bare `jigc start` never surfaces `planning`** — which means "milestones out of the box" is *not* delivered by the router alone. M21 surfaces **both** from orientation (a route-prose line naming the verb, the same shape for each), so a human/agent on a fresh setup finds *both* the existing-project on-ramp **and the milestone-planning verb** without already knowing them — **without** flipping `planning` selectable (the M16 off-router invariant holds). This is what actually delivers the milestone half of the done-picture; the router default (`router`) delivers vision + MVP.
- **Teardown / cleanup (G5) — two verbs, both repo-local.** No teardown exists today (only `task discard` removes a working area). M21 builds: **(a) un-manage a doc** — drop a single managed doc from the file-state record + its `edges.json` entries, leaving the file on disk (the inverse of `adopt`'s register-only mutation); and **(b) `jigc uninstall`** — **repo-local** teardown of *this project's* jigc install. It reverses **every setup-created repo-local artifact**: remove `.jigc/` (which carries the setup composition marker inside `config/`), unwire the `CLAUDE.md` import line, drop **every setup-created entry from `.claude/settings.json`** — the `Bash(jigc:*)` allowlist permit, **the `SessionStart` hook**, **and the `deny` safety floor** ([assistant-adapter.md](assistant-adapter.md) → the `deny` safety floor) — and **prune the git `pre-commit` hook**.

  **The set is what `setup` writes, not a fixed list (M48 Increment 10).** `setup` now also writes the adapter's **owned guide artifact** at the profile-declared path ([assistant-adapter.md](assistant-adapter.md) → The adapter's owned artifacts), and it is a repo-local setup-created file like any other, so it joins this set — jigc wrote it, jigc takes it back out. It is the **seventh** member, and it comes out under **one condition**: jigc's own copy is removed, and a copy the user has edited is **left in place and reported** — the same `adapter-guide.user-modified` advisory the writing doors raise, routed at this door's own exits (keep it, delete it yourself, or re-run with `--force`), because a door that deletes authored bytes at exit 0 is exactly the class this verb's guards below close, and this file is not exempt from it. The **directories jigc created to hold it** go with it once they empty, stopping at any directory that houses another host file — `.claude/` holds the settings file the teardown just edited *surgically*, so taking it would be the same clobber one level up. Refusing the one file never refuses the teardown: the rest of the install still comes out, and the ledger reports `guide: false` rather than a removal it did not make.

  **Both hooks must come out — the M36 symmetry fix.** M21's teardown reversed only three of these artifacts, silently leaving **both hooks** behind: the `SessionStart` entry and the git `pre-commit` script survived teardown **byte-identical** and then fired against a removed install (a session-start `jigc start` and a per-commit `jigc validate` with no `.jigc/` present). M36 closes this. Removal is **surgical, not a clobber**, because both hooks carry identifiers `setup` already writes: the `SessionStart` entry is the settings-hook matcher whose `command == <profile hook.run>` (drop only that one entry from a possibly-shared `hooks` object, mirroring how `remove_allowlist` drops one permit from a shared `allow` array), and the `pre-commit` block is bracketed by the `PRECOMMIT_SENTINEL`/`…END` markers (prune the marked block; a foreign hook that `setup` *wrapped* is restored, not deleted). The `deny` floor comes out the same structure-aware way as the permit ([assistant-adapter.md](assistant-adapter.md) → merged, never clobbered). **`uninstall` still does NOT remove the `doc-code` probe sibling** — that is the one **machine-global** setup write (`current_exe().parent()/doc-code`, shared by every jigc project on the machine — [module-layout.md](../implementation/module-layout.md) → Probe distribution); deleting it would break doc↔code validation for *other* repos, so it is correctly outside the per-project set. Machine-global removal is `cargo uninstall jigc` + manual probe removal, never the per-project verb.

  Both verbs write a user's real repo, so they carry an explicit **idempotency + non-destructive** acceptance: un-manage touches only jigc's own index/state (asserted: the doc's bytes on disk are unchanged) and is idempotent (re-run on an already-unmanaged doc is a clean no-op, not an error); **`uninstall` removes exactly the enumerated repo-local setup-created set — now *complete*: `.jigc/`, the import line, all three `.claude/settings.json` writes (permit + `SessionStart` hook + `deny` floor), the `pre-commit` hook, and the adapter's owned guide artifact** — and is idempotent (a second run is a clean no-op). The machine-global `doc-code` probe is deliberately *outside* that set, so "exactly the setup-created set" excludes it by design, not by omission. **Two states refuse the teardown outright (2026-08-09, widened 2026-08-13):** `.jigc/` is the only copy of two kinds of work, so `uninstall` blocks rather than take either — **(a)** a worktree-shaped path under `.jigc/worktrees/` that holds content blocks with `uninstall.dirty-worktree` (the subject is the *path*, classified fail-closed, not the registered set: a copied or moved repo's live worktrees are registered at the *source's* path, so a registered-set guard is inert exactly where the live work is), and **(b)** an open task under `.jigc/tasks/` holding a staged `docs/*.md` blocks with `uninstall.staged-prose` — bytes in no object DB at all, the reproduced pre-1.0.0 loss. **The subject there is *staging*, not authorship**, and the refusal says so: the probe cannot tell prose someone typed from the pristine skeleton `jigc start` mints, the skeleton is in fact the dominant cell, and it refuses over both on the one ground it can prove — no commit has a copy — rather than claiming authored prose it never saw ([surface-contract.md](surface-contract.md) → law 1). Its route leads with the exit that state can reach (`jigc task discard`), since `finalize` cannot land an empty skeleton. Both remove nothing, name what they found, and route at the honest exits ([team-ready-state.md](team-ready-state.md) → Abandon refuses on a dirty worktree, the sibling guard; [DECISIONS.md](../DECISIONS.md) → 2026-08-13 the Settle, F3). "Non-destructive" is what makes that a *requirement* of this acceptance rather than an exception to it — and the one way past it is **`--force`**, the operator's explicit consent to delete, so the destruction is asked for rather than assumed.

**The honest Flow-B bound (scope clarity).** As of M21 (before G1/G2), `adopt` reached **only docs already conformant to a shipped doctype** (`adr`/`prd`/`spec`/`arch-doc` + the methodology running-docs); for a foreign project *not* built with jigc that conformant subset is often **near-empty** — its README/CHANGELOG/runbooks/freeform-ADRs map to nothing and route as `unmanaged`/`needs-reconcile`. So M21 Flow-B delivered **recursive scan + classify + route + the G4 safety gate + teardown + discoverability** — an honest "here is your doc landscape and the safe on/off ramps" — **not** broad "track every doc." **M22 (G2)** added the `changelog` doctype (a *new* project's authoring target) and **M23 (G1)** adds the **transform** arm — an existing `CHANGELOG.md` is now **migratable** (rewritten to conformant shape + adopted, [auto-migration.md](auto-migration.md)). So the Flow-B bound now reads: classify + route the full corpus, **adopt the conformant subset + migrate a foreign `CHANGELOG.md`**, tear down — broadening toward "track every doc" doctype-by-doctype (the location-bearing doctypes generalize at M25, after M24 hardens the changelog reference). The existing-project **live test is designed against this real capability**, not an overclaimed tracking surface. README stays **never a doctype**. **Post-M38 (placement) — root files can be managed.** The classifier no longer treats a repo-root `.md` as inherently unmanageable: a **placement** doctype's declared literal home *is* a managed canonical path (root `VISION.md`, root `CHANGELOG.md`, or `docs/roadmap.md`), so a foreign file sitting at that exact path is an in-location squatter — **migratable / adopt-in-place**, not left-untouched-unmanaged — while every *other* root `.md` (`README.md`, `CLAUDE.md`) stays unmanaged by exact-path ownership (a literal home is not a dir-glob; [storage.md](storage.md) → Placement).

A second consequence of the G4 gate, stated so it isn't a surprise: a non-conformant doc squatting in a `location:` dir is **routed (advisory) but not recorded**, so the advisory **re-fires at every finalize** until the human resolves it (ingest if it conforms / move it out of the location dir, then `migrate` the moved file / move it out).

**Which advisory splits on the managed-vs-foreign discriminator (M48 Inc 4).** The gate is one outcome — routed, not recorded, recurring — but the *file* is one of two things, and until M48 this arm called both of them the same thing. A **never-adopted foreign** file (no schema-version stamp, parsing against no shipped version of the doctype) is the **adoption** case, so it now draws the store sweep's `schema-conformance.unadopted-instance` and the route naming `jigc ingest` / `jigc migrate <path> --as <doctype>` with its own path substituted in — the same code and the same bytes `jigc validate` already served for it, so the two doors stop telling two stories about one file ([validation.md](validation.md) → The discriminator's third consumer). A **managed** doc that has merely gone non-conformant keeps `reconciliation.conformance-block`. Neither is recorded, so the recurrence below is unchanged for both. *Note: in M23 `jigc migrate` on a file already **at** the canonical managed path did not author end-to-end (move it out first); the in-location-squatter case is fixed in M24 ([auto-migration.md](auto-migration.md) → Hardening).* That recurrence is intended — it is the honest "an unvetted doc is sitting in `decisions/`" signal — and the advisory's route names the corrective verb. The finding renders distinctly in the finalize report (not swallowed under a clean exit).

**Split from M21 into two separate post-M21 milestones, now both shipped/scheduled (split 2026-06-14, [DECISIONS.md](../DECISIONS.md) → 2026-06-14 split):** **doctype coverage** (G2) shipped **first** as **M22** — the `changelog` doctype + driver, serving Flow A (a *new* project authors the managed doc through jigc — no migration needed); **auto-migration** (G1) ships **after** as **M23** — the transform arm (the G2 doctypes are its migration targets), settled under **Framing A** (boundary intact, no CLI fuzzy-mapping — [auto-migration.md](auto-migration.md)), changelog-first. Together they make Flow B's "track an existing `CHANGELOG.md`" real (a `changelog` doctype can't track an existing file without migration to conformant shape). README stays **never a doctype** (bespoke front-page prose, no schema-able structure).

## Scope boundary (what M9 does **not** do — partially superseded by M21)

- **No auto-migration** of non-conformant docs (deferred past M21 → **ships as M23, changelog-first**, Framing A; M24 hardens the changelog reference; the location-bearing doctypes generalize at M25 — [auto-migration.md](auto-migration.md)).
- ~~**No multi-pack / pack-choice** selector~~ — **superseded at M21**: a real project now composes **dev + methodology** by default at setup ([multi-pack.md](multi-pack.md) → Embedded second pack + setup auto-wiring). The packs are *auto-wired, not chosen* — there is still **no pack-choice selector** (that stays deferred with the public pack-authoring surface).
- **No `prd` repeatable-requirements authoring** / `add-item` verb (deferred; prd uses prose slots).
- **No second assistant profile** (Claude Code only — [assistant-adapter.md](assistant-adapter.md) → Open questions stays deferred).
- **No new fan-out / override / severity surface** — both flows are ordinary compose + create/author + finalize (new) or scan + classify + route (existing). *(M21's teardown verbs are read/state-only — no new override or severity surface either.)*

## Open questions

- **Auto-migration of inconsistent docs** — ships **M23** (changelog-first, Framing A — [auto-migration.md](auto-migration.md)); **M24** hardens the changelog reference; the location-bearing doctypes (`adr`/`spec`/`prd`/`arch-doc`) and code-inferred docs generalize at **M25**.
- **`jigc ingest` exact verb surface + adopt-confirmation** — pinned at the build's acceptance-flow spike against the built command grammar.
- **`prd` repeatable requirements + the `decomposes-into` edge** — when a flow must author structured, individually-addressable requirements (needs the deferred `add-item` verb + the multi-word-heading parser fix).
