# VISION

> Working name: TBD. The command is referred to throughout as `jigc` as a placeholder.

## Thesis

**Take every *structural* operation away from the LLM and give it to a CLI. Leave the LLM only the prose.**

A coding agent doing project work has two kinds of jobs: deciding *where things go and how they connect* (structure), and *writing the actual content* (prose). Today's tools let the LLM do both — and the structural half is exactly where they become non-reproducible, unmaintainable, and impossible to correct. This project draws a hard line: the CLI owns all structure deterministically; the LLM only authors content into slots the CLI controls.

**What is proven, and what isn't (M17–M19; the scale boundary settled by the long-horizon study, 2026-06-23).** The *architectural* thesis above — structure to the CLI, prose to the LLM — is built end-to-end and design-proven. The stronger *empirical* claim it was meant to buy — that jigc makes a coding agent **measurably more correct/effective than a static `CLAUDE.md`** — is, as of M17 (the dogfood-and-measurement milestone), **not demonstrated**: the one controlled jigc-vs-static comparison **tied**, and the self-hosting run is an uncontrolled existence proof that the machinery *runs*, not that it *wins*. The live, **scope-bound hypothesis** — that the differentiating value concentrates at the methodology's native (milestone/increment) grain *with a fitting domain pack*, where structural operations are actually in play — is **relocated, not earned**; settling it needs a native-grain comparison with a control and an independent judge. See [completions/artifacts/M17/VERDICT.md](completions/artifacts/M17/VERDICT.md). **M18** built the missing over-time detector (the store-wide `jigc validate` doc↔code sweep) and ran the maintenance/drift comparison — the founding thesis's *best remaining shot*. It too returned **H0 (a tie at zero stale citations, both arms)**, but under a pre-registered **session-compression** bound that suppresses the very forgetting the detector guards against — so the over-time advantage is **undemonstrated, not refuted**; the owed test needs genuine cross-session separation. M18 also observed jigc's **behavioral blind spot in the wild**: `jigc validate` can be green while a doc's prose has drifted from the code it still correctly cites — structural-anchor honesty is a narrower thing than documentation honesty. See [completions/artifacts/M18/maintenance-test/results.md](completions/artifacts/M18/maintenance-test/results.md). **M19** then made the detector *fire automatically* (a warn-only `pre-commit` backstop installed by `jigc setup`, its warning relayed through `finalize`) and ran the comparison under **genuine cross-session forgetting** (a fresh cold agent per step). It too returned **H0 — a tie at zero stale citations** — and sharpened the picture: the **static arm stayed clean at every commit** (right-first-try), while the **jigc arm was the only one that drifted** and recovered via later detector-prompted cleanup. So the backstop **demonstrably fired and drove repair** (the loop M18 never exercised) — a real capability gain — **but on drift that arose in jigc's own arm, and it did not beat diligent static maintenance** on either the end-state or the per-commit timeline; the regime where it would strictly win (diligence leaving *residual* drift) wasn't reached. The over-time advantage remains **undemonstrated**, and the backstop's value is **advisory** (it closes the loop only when an agent heeds the warning). See [completions/artifacts/M19/cross-session-test/results.md](completions/artifacts/M19/cross-session-test/results.md). **Why the three ties may not hold at real scale (mechanism, not measurement).** All three comparisons held one variable fixed that dominates in practice: **instruction salience**. The static arm was given a *clean, on-point* standing rule — "keep citations honest," with nothing competing for it. That is the *best case* for static conventions, and the tests required it: **symmetry demanded the same clean instruction in both arms**, so a fair comparison could only ever idealize the static control. Real `CLAUDE.md` files are not clean — they are long, half-stale, and internally competing, and the relevant rule loses the attention budget as the file grows. The two mechanisms diverge exactly here: **static conventions are salience-dependent** (they work only as well as the agent attends to the relevant line, which decays with rules-file size), while **jigc's backstop is salience-independent** (the `pre-commit` sweep fires identically whether the rules file is five lines or five thousand — it is a mechanism, not an instruction, so it never competes for attention). This predicts that jigc's advantage *appears* precisely in the regime the tests excluded — and that the regime is **structurally hard to test fairly**: reproducing it means handing the static arm a realistically bloated rules file, which breaks symmetry and confounds the result (did the control lose because conventions degrade at scale, or because *that* file was badly written?). So this is stated as what it is — **predicted by how the two mechanisms work, not yet measured, and not cheaply measurable** — and it is the sharpest honest statement of the scope boundary: the ties are real *at small, high-salience scale*; the mechanism says the gap opens as the instruction surface grows, but that claim is owed a test no one yet knows how to run cleanly. **Update — the test was run, and the prediction holds for the weaker model.** The **long-horizon many-edit study** (2026-06-23) built the fair-control design the paragraph above calls structurally hard: 8 sequential edits, a fresh cold agent each, an evolving twin, salience-independent enforcement (a *blocking* pre-commit hook — a one-line delta from the shipped warn-only backstop) against a **dilution ladder** (the same byte-identical rule at 42 / 157 / 452 lines, turning the confound into the measurement). For **Sonnet** it is the project's **first controlled, blind-confirmed (Codex) jigc-beats-static result**: jigc held doc↔code drift **flat at zero** across all 8 edits while plain compounded to 5.0 dangling anchors and *every* static arm shipped **residual** drift — decisively the *file-move* case (edit 3), where the symbol name is unchanged so a name-based convention (and a human, and a symbol-checking judge) sees nothing wrong but the recorded path is dead (6/9 Sonnet static reps shipped it; jigc 0/3). The win is **hook-driven, not engagement-driven** (jigc forced repair on edits where the agent ran zero jigc commands) — *enforce, don't instruct*, demonstrated. **But the same study refutes the claim for Opus**: the hook governs anchors, not prose, and the capable model surgically cleared the gate while leaving component **titles** naming dead symbols — ending *dirtier* than a plain instruction (blind judge 0/2 vs static 2/2) at ~3× the cost. So the M18 prose blind spot is **outcome-determining** for a capable model, and the honest scope boundary is now *measured, not predicted*: jigc's enforcement strictly wins **where instruction degrades** (weaker model · long horizon · move/path drift) and loses **where the enforced surface is narrower than the instructed one**. The clear next move the data points at — **anchor the component titles** (close the prose gap inside the existing `doc-code` mechanism), then re-test the capable model. See [completions/artifacts/long-horizon-study/VERDICT.md](completions/artifacts/long-horizon-study/VERDICT.md).

## The problem

The failure mode shared by today's approaches is that **structural operations are performed by the LLM as freeform edits instead of by a program as deterministic operations.** It shows up everywhere:

- **GSD** lets the LLM rewrite the project skeleton. Adding a phase means the LLM injects it "in a weird way, with weird numbers, never the same way twice." It generates a mass of files nobody reads, doesn't keep them updated, and "just does things" — you throw in one command and it assumes the rest, leaving a project you must heavily correct later.
- **Spec-Kit** has the right philosophy (intent before implementation, multi-step refinement, guardrails) but individualization rots: you override one thing, the underlying system changes, and it silently bites you months later.
- **Static rules files** (`CLAUDE.md`, Cursor rules) are noisy, static, and unverifiable — they can't compose, can't validate, and can't stay current.

All three are symptoms of the same root cause. Fix the root cause and the symptoms go away.

**Closest alternatives we don't name above.** Doc-as-code pipelines (Markdown + schemas + linting + ADR tooling + CI), LSP-style structured editing, structured knowledge bases (Notion/Confluence/Linear), and agent harnesses that generate prompts and constrain file ops share our diagnosis — structural operations belong to deterministic tools — and each delivers parts of what we claim: schema validation, refactoring, cross-references, prompt assembly. They're real competition, not a straw man. The gap isn't capability, it's packaging: **composition** (workflow from steps + cascade), **just-in-time context-assembly** for an agent with no editor in the loop, and **cross-doc graph integrity enforced at transaction boundaries** (not batched at CI) — all designed around a single agent-facing transactional channel. That packaging is the bet.

## What it is

A **context compiler** for coding agents. The agent's context window is the scarce resource; instead of dumping everything in or letting the agent search ad hoc, a deterministic CLI assembles *exactly* the instructions and document slices needed for a specific task, just-in-time, and is the sole channel through which the agent reads and writes managed documents.

- **Engine + CLI, kept separate.** Core logic (config resolution, document registry, workflow composition, validation, state) is independent of any frontend. The CLI is the first frontend; an MCP adapter could come later over the same engine. We bet on CLI now because coding agents work well with a single hierarchical command tree (one entrypoint, subcommands) and humans can use the exact same tool — versus MCP's flat, ever-growing list of tools and single level of nesting.
- **Wires into one specific project.** No one-size-fits-all — jigc ships strong defaults and a setup workflow that customizes them per project. Config cascades: **project > team > pack-default** (`team` = a cross-project layer, not per-developer).
- **Engine vs domain pack.** The engine is domain-neutral and ships **empty** — no documents, workflows, or commands of its own. A **domain pack** supplies the bottom (default) layer of the cascade: its doc types, workflows, steps, domain commands, default values, and validation probes. **Development is the first and only pack — the wedge and the proof, not the ceiling.** Generality is protected by keeping domain content *out* of the engine, not by building it as a feature yet.
- **Deterministic by design.** Composition is assembled from config, never generated by an LLM. The CLI itself makes no LLM calls. Given the same resolved cascade, the same workflow comes out.

## Design principles

Each principle is tagged with the pain it eliminates.

### 1. Deterministic skeleton, LLM-filled slots
Structure — which documents exist, their sections, cross-references, ordering, workflow steps — is config-driven and reproducible. Content — the prose of a spec — is LLM-authored into *named slots* the CLI controls. The CLI guarantees the wiring; it never touches the prose.
*Kills:* non-reproducibility; "the LLM assumes the rest."
*Honest boundary:* "reproducible" applies to **structure**, never to LLM prose. The structure/prose split isn't a natural law — many decisions are mixed (whether to create a new ADR, which doc to cite, how to name an entity). The CLI splits these by what it can mechanically check: **fields** are structured values like slugs, enums, and relation targets; **slots** are prose only the LLM/human can judge. Mixed decisions stay LLM judgment, but the *act* is a CLI command (workflow-gated `create`, typed-field `set`). The agent reasons; the CLI executes. See [The determinism boundary](#the-determinism-boundary).

### 2. Stable IDs, never positions
Every phase, document, step, and slot has a stable opaque ID. Ordering lives in a separate ordered list. "Insert a phase after X" is one deterministic command that mints an ID and edits one ordering array — nothing renumbers, and cross-references (which point at IDs) never break on reorder. **"Never positions" bans IDs that *encode* order** (`01-`, `02.5-` prefixes; numeric positions used as references) — *not* physical-order serialization of an ordered list whose items carry their own non-positional IDs. Physical-order serialization is used uniformly: doc instances' repeatable items ([storage.md](design/storage.md) → "Order = physical order. Reordering is moving a block — a clean diff move, never a renumber") and workflow bodies' include lines ([workflow-dialect.md](design/workflow-dialect.md) → On-disk definition format) follow the same pattern — a line move is a reorder, nothing renumbers, cross-refs by ID keep resolving.
*Kills:* GSD's `01-`/`02.5-` renumbering mess; "never the same way twice."
*Honest boundary:* IDs survive **reorder** and **rename** because minted IDs are content-slugs *frozen at creation* — the title can change, the id can't. Splits and merges are **explicit CLI ops**. Copy/paste is caught when it creates file-state drift, duplicate IDs, or schema/conformance errors; structurally valid duplicate concepts across docs remain a prose/review problem. IDs also don't prevent semantic drift — the same id meaning something subtly different over time — because the CLI never adjudicates prose.

### 3. The CLI is the only interface — reads *and* writes
Files are storage; the CLI is the interface. Reads return assembled *views*, not raw files, so the document store can grow large without overwhelming anyone. Writes are routed: the LLM hands the CLI content for a named slot, and the CLI owns placement, cross-ref wiring, versioning, and commit. The LLM cannot misplace anything because it places nothing. Non-determinism is quarantined to slot contents.
- The LLM writes **only** through the CLI.
- Humans *should* use the CLI but will edit files directly anyway — so out-of-band edits are **detected and routed, never forbidden**. **Files are truth**: a conformant external edit imports automatically via the canonical-Markdown parse; a nonconformant edit blocks with a precise conformance error; a true conflict routes to the human, and discard is explicit, never silent. No silent data loss; three-way merge is deferred. See [storage](design/storage.md).
- **Agent compliance is adapter-enforced, not sandboxed.** "The LLM writes only through the CLI" holds because the [assistant adapter](#sub-agents--assistant-integration) makes the CLI the path of least resistance — bootstrap routing, allowlisted commands, the right tool for managed docs — not because the agent is prevented from editing files. An agent that ignores the adapter contract can bypass us; the bet is on ergonomics + the bootstrap's *advertise + demonstrate* discipline doing the work.
- Storage stays **human-readable and diff-friendly**: documents live in the repo as plain files that read cleanly in a normal PR diff, because humans review (and edit) through git regardless of the CLI. If the on-disk format isn't legible in a diff, review breaks and adoption dies.
- Write-time validation is **transactional**: writes land in a working state; integrity must hold at a `finalize`/commit boundary, not on every write (otherwise bootstrapping deadlocks).
- Under concurrency (sub-agents), each writer gets an **isolated working area keyed by task ID**; the CLI merges them at a **deterministically-ordered join** (by task ID, not completion order) so parallel work stays reproducible.

*Given a compliant agent*, principle #1 is enforced *by construction* rather than by hope: the LLM cannot misplace a section, drop a cross-reference, or "just do things," because the CLI performs the placement and the LLM never sees positions to misplace into. And because nobody navigates the directory directly, the "files nobody reads" problem and silent structural drift both lose their hiding place.

### 4. Workflows are composed, not authored
A workflow is not a monolithic prompt — its **definition** is an include-list file the CLI assembles from reusable steps, with placeholders resolved deterministically against config + cascade + live state. Three placeholder kinds, all CLI-filled before the LLM sees the text:
- **Command references** — `{{ cli.add_phase_command }}` → the literal invocation to run next (routing the LLM back into the CLI for the structural op). Resolves through the cascade, so project overrides flow through.
- **Data values** — `{{ task.commit#summary }}` for the address, `{{ @task.spec#criteria }}` for the content at it (the relevant doc slice, pulling info together — see [workflow-dialect.md](design/workflow-dialect.md) → Leaves for the `@` rule).
- **Includes** — `{{ include: step:validate-cross-refs }}` (composition/reuse).

Placeholders (CLI-filled, read path) are the opposite of slots (LLM-filled, write path) and must stay visually distinct in syntax. The emitted workflow is structured markdown that makes unmistakably clear which lines are "run this exact command," which are "author this slot," and which are "reason about X." A `--explain` mode shows the resolution tree.

Steps may also be marked **`fan-out`/`join`** — the single bounded concurrency construct (see [Sub-agents & assistant integration](#sub-agents--assistant-integration)). The CLI resolves the fan-out list and emits spawn instructions deterministically; the agent does not improvise the topology.
*Kills:* "throw in one command and the LLM assumes the rest"; inconsistent placement across runs.

### 5. Override with deltas, never an untracked fork
The enemy is not forking — it is the *untracked* fork that discards its ancestor, making upgrades a guess. Every customization is a recorded delta against a known base version. A ladder, lightest rung that fits:
1. **Scalar/key override** (cascade) — settings, conventions. Most customization lives here.
2. **Structural ops by ID** — `insert-step --after X`, `replace-step`, `remove-step`. Robust because they target IDs (principle #2), not positions.
3. **Slot fills** — override a named extension point; never the surrounding prose.
4. **Tracked fork** (last resort) — copy a step but record the base version, so a three-way merge can detect conflicts on upgrade.

On upgrade (defaults v1 → v2) the engine re-applies each delta and reports per-delta: **clean** (you inherit every other v2 improvement for free), **conflict** (target changed upstream — review), or **orphaned** (target gone — loud failure). No upstream change is silently lost; no override silently breaks. Drift becomes an explicit item at a known moment.
*Decisions locked:* the CLI records deltas (hand-editing allowed only in delta format, never as forked copies); IDs exist at doc/step/workflow level always, slots only where the author deliberately anticipates customization; conflict *detection* is deterministic, the LLM may *propose* a merge, the human *confirms* (the *agent* drafts the proposal through the CLI — the CLI itself never calls a model).
*Kills:* "individualize and it bites you later."
*Honest boundary:* detects **structural/syntactic** conflicts, not **semantic** ones (upstream changing the meaning around an untouched override). And great defaults matter more than the override machinery — if a project must override heavily to be productive, the defaults are wrong.

### 6. Validate against reality — the integration advantage
Normalized, cross-referenced documents are only valuable if referential integrity is *enforced*. One deterministic validation engine, many targets:
- **doc ↔ code** — an ADR cites a module that no longer exists; a SPEC's criteria map to no test; referenced code changed after the doc's timestamp.
- **override ↔ default** — the upgrade reconciliation above.
- **file ↔ CLI-state** — out-of-band human edits.
- **workflow ↔ references** — every placeholder/include/command-ref resolves, so broken workflows are caught at validate-time, not at the LLM's runtime.

The engine provides the validation *framework*; the domain-specific **probes** are pack-provided. `doc ↔ code` (does this symbol exist? does a test cover this criterion?) is *development*-pack content; `override ↔ default`, `file ↔ CLI-state`, and `workflow ↔ references` are engine-native.

**North-star — `doc ↔ code` must reach every language a real project uses.** This advantage is only real if it works on the project's *actual* stack — Python, PHP, bash, CSS, TypeScript, JS, and onward — not just Rust. A Rust-only validator is a Rust tool, not a context compiler for coding agents. M27 shipped six languages and proved the extension model (grammar-by-extension → per-language addressable-unit allowlist → static parse, probe-internal); the path to "every language" is shaped in [ideas/multi-language-doc-code.md](ideas/multi-language-doc-code.md) and tracked under Open questions below.

*Kills:* documentation drift; the "files nobody keeps updated" problem.
*Honest boundary:* "keep files updated" means the CLI **detects** drift deterministically and **routes** the LLM to repair it in a fixed shape — not that the CLI auto-authors fixes. Knowing what's stale, always, is the achievable and valuable promise. No single one of the four targets is novel — linters do workflow-style ref-checks, CMS systems enforce referential integrity, package managers reconcile overrides. The advantage is the *combination*: all four validated by one deterministic engine, with failures surfaced at the task's transaction boundary instead of later in CI or scattered across separate tools.

## The determinism boundary

State this as a contract, because conflating the two halves leads to chasing a reproducibility that cannot exist.

| Deterministic — owned by the CLI | Non-deterministic — authored by the LLM |
|---|---|
| Which documents exist and where | The prose inside a slot |
| Document structure, sections, ordering | A spec's actual wording |
| Cross-references and their integrity | |
| Workflow composition and placeholder resolution | |
| Placement of every write | |
| Validation results | |

The CLI guarantees the wiring; it never guarantees the prose.

## Document model

Documents are a normalized database: **one document, one purpose, small footprint, cross-reference instead of duplicate.** Each document type is *defined* — its purpose, location, sections, required cross-references, template, and validation rules. The file mass that overwhelms GSD users is a non-problem here because **nobody navigates the directory** — the CLI assembles views on read and places content on write.

Document types to define (starting set): commit messages, architecture documentation, PRDs, ADRs, SPECs — now **complete, every member ships** (`arch-doc` was the last, driven at M13; see [implementation/doctype-map.md](implementation/doctype-map.md)). **Each is a managed document** — even a commit message is a doc type (hence addresses like `commit:add-rate-limiter`) — so each gets a single clear purpose and explicit cross-reference obligations rather than duplicated context.

## Primary flows

- **Compose / read loop:** LLM asks for a workflow → CLI composes from config + cascade + live state → resolves all placeholders deterministically → emits the composed instruction set → LLM *follows* it (including running the CLI commands it names for structural ops).
- **Write / finalize loop:** LLM reads via CLI → drafts content into a named slot via the CLI (stages in the per-task working area with write-time field-type/slug checks) → `jigc task validate` / `diff` previews the changeset and findings → `jigc task finalize` re-runs validation and commits. `finalize` defaults to autonomous (opt-in confirm-gate is a cascade setting); git/PR review is the durable correction point.
- **Validate loop:** `jigc validate [target]` checks integrity across all four targets and reports what is stale or broken, deterministically.

### Workflows to ship
`project setup (existing project)` · `project setup (new project with idea development)` · `project planning` · `milestone planning` · `milestone execution` · `single task execution`.

The **first MVP loop** is `single task execution` end-to-end (discover → compose → execute → validate), because it is the cheapest way to *test whether* the core loop beats a plain `CLAUDE.md` (the verdict on that test — undemonstrated as of M17 — is in the Thesis note above). **Validation is in the MVP, not deferred** — but specifically the validation *framework* plus the two **engine-native** probes that need no pack content or pre-existing docs: `workflow-refs` and `file-state` (see [validation.md](design/validation.md)). The pack-provided `doc ↔ code` probes land *after* the doc-creation flows exist, since they require real ADRs/SPECs to check against. This keeps the MVP **focused** — narrow in feature scope, substantial in foundation, because the differentiators rest on a shared substrate (round-trip parser, edge index, file-state hashing, config cascade, workflow composition, read-view assembly, write path, finalize gate). It also dissolves a bootstrap circularity (meaningful `doc ↔ code` validation would otherwise require the very doc-creation flows the MVP excludes).

## Worked example

A composed `single-task-execution` workflow, shown with placeholders unresolved so you can see the wiring (notation **illustrative**; the delimiters are settled — `{{…}}` is CLI-resolved before the agent sees it, replaced inline with the resolved value; `<<author: addr>>` points the agent at a doc slot to fill through the write path):

```text
# single-task-execution · task add-rate-limiter · 3 steps

## 1 · Locate
Read the spec for this task:
{{ @task.spec#criteria }}         # the SPEC slice content, resolved in (the `@` derefs the path's address — see workflow-dialect.md)

## 2 · Implement, then hand back the commit prose
Run: `jigc doc set-field commit:add-rate-limiter#type --value feat`
Run: `jigc doc set-slot  commit:add-rate-limiter#summary --from-file -`
     <<author: commit:add-rate-limiter#summary>>   # ← you write this; the CLI places & wires it

## Finalize
Run: `jigc task finalize add-rate-limiter`
  ⚠ "SPEC criterion 'limit=100/min' maps to no test — add coverage before finalize."
```

What the agent actually sees has every `{{…}}` replaced inline with the deterministically-resolved value (a literal command, a doc slice, the included text); only `<<author: addr>>` survives into the agent's view, naming a *document* slot the agent fills through the write path — the only thing the agent originates is that prose. The `⚠` is the validation engine speaking, not the agent. (The task itself was born at `jigc start`; structural ops like registering a milestone phase are other workflows, out of this single-task scope.) This illustrates the general, **spec-driven** shape; the **MVP** `single-task` is **spec-less** — its `locate` reads the human `intent` + the codebase, not a SPEC (see [CLAUDE.md](CLAUDE.md) → MVP scope).

## Sub-agents & assistant integration

Sub-agents are first-class — and they ride on the architecture rather than fighting it.

### Sub-agents are just additional CLI consumers
A sub-agent is an LLM that needs context, so it gets context the same way the main agent does: it calls `jigc workflow <x> --task <id>` and the CLI composes its instructions. The CLI doesn't know or care whether it's a main agent or a sub-agent. On the **read/compose side, sub-agents need zero new machinery.**

### Coordination is a control plane; the CLI is the data plane
Agents coordinate **through the CLI, not through each other.** The agent-to-agent channel carries only control; the CLI carries data and is the single source of truth.
- **Dispatch** is minimal: `task_id` + "call the CLI." The main agent never paraphrases instructions — that would reintroduce a lossy, non-reproducible telephone game. The sub-agent gets the *exact same deterministic composed workflow* it would get as a main agent.
- **Acknowledgement** is minimal: `status + task_id` (e.g., "done — task 7, succeeded").
- **The message is a notification, never the source of truth.** The actual outcome lives in CLI state; the main agent always **re-derives state from the CLI**, so even a lying "finished" is caught when the task is queried.
- **Errors and blocks route the same way:** a sub-agent writes the detail to CLI state keyed by its task and returns "failed/blocked — task 7"; resolution surfaces through the normal propose-to-human path. Nothing important ever rides the channel.

This is the **blackboard pattern** (agents read and write a shared store — here the CLI — and never message each other directly) — proven and robust — and it makes sub-agents cheap, stateless, and reproducible (their only memory is the task ID; same task ID in → same instructions out).

### Parallelism is planned, bounded, and deterministic
The structural decision "what runs in parallel" is deterministic and lives in the workflow as a `fan-out`/`join` step — *not* improvised by the agent. The CLI resolves the fan-out list and emits "spawn N sub-agents for these task IDs." Each sub-agent writes to an isolated working area keyed by its task ID; the CLI merges at a **deterministically-ordered join** (by task ID, not completion order) — the *ordering* is deterministic, though the *content* each sub-agent authored is still LLM prose, per the determinism boundary. Keep this the *only* concurrency primitive — concurrency is where determinism and simplicity go to die.

### Assistant integration: neutral core, thin adapter
The engine and CLI are **assistant-neutral**; only a thin adapter is assistant-specific. For Claude Code that adapter is a one-line pointer in root `CLAUDE.md` plus a small set of skill/command files that each just call the CLI. We replace the bulky static rules file with a one-line **routing** pointer to a dynamic, validated context source — the pointer is *routing, not content*. That routing pointer plus the thin file layout *is* the entire integration surface. The same engine serves Cursor, Codex, etc. through equally thin adapters.

The seam where sub-agents meet integration:
> **The CLI owns the sub-agent's instructions (the payload); the coding assistant owns the launch (the mechanism).**

*What* a sub-agent receives is core and neutral; *how* it's spawned (Claude Code's Task tool vs. another assistant's) is the adapter. Design that boundary once and both questions are answered.

## Outcomes (success criteria)

- A coding agent can do project work knowing only **one entrypoint sentence**.
- **Same *resolved cascade* in → same composed workflow out** (structural reproducibility; the cascade is a declared input — repo-reproducible config lives at project level).
- Customize one thing and **inherit every other upstream improvement for free**; every divergence surfaces at a known moment.
- Document staleness is **always known**; nothing drifts silently.
- The document store can **grow large** without overwhelming anyone.
- The human stays in control: the LLM **proposes**, the human **confirms**, corrections are **direct**.

## Non-goals

- **Not a general-purpose workflow engine.** Ordered steps + includes, plus a single bounded concurrency primitive (`fan-out`/`join`) and a `checkpoint` step that halts for a human (M15) — but no DAGs, conditionals, cross-sub-agent messaging, or a runtime. A `checkpoint` is not a revision of this line: it *always composes* and emits a halt directive the agent honors, adding **halting** (which the invariant never barred — it is `finalize`'s blocking gate lifted to an arbitrary step) without adding **branching** (a step that appears/disappears on runtime state — still barred). Resumption stays the stateless restart-from-scratch re-compose; no progress cursor, no runtime. Add branching only if real workflows demand it.
- **Not an auto-doc-writer.** The CLI detects and routes; it does not author prose or silently auto-fix drift.
- **Not one-size-fits-all.** It wires into one project via strong defaults + a setup workflow.
- **Not (yet) a public pack platform — but no longer single-domain.** The engine/pack boundary is **proven architecture, not just internal discipline**: M12 minted a second (methodology) pack, M14 composes it with the dev pack under deterministic collision resolution, and the mechanism generalizes to N domains ([multi-pack.md](design/multi-pack.md)). What stays **future work — earned when a real *external* domain needs it, not ruled out** — is a *stabilized, documented public pack-authoring surface* (a third-party contract + versioning); the composition mechanism is internal until then.
- **Not an LLM wrapper.** The CLI core makes no LLM calls; composition is deterministic. (The *coding agent* may use the CLI to propose merges — that's the agent acting, not the CLI.)
- **Not a replacement for the agent's reasoning.** It constrains placement, not thinking.

## Open questions

Each part-doc tracks its own opens in its `## Open questions` section; this is the **categorized index** across the doc set.

**Cross-cutting (VISION-level):**
- **Multi-language doc↔code — reach every language a project uses** — the principle-#6 north-star: doc↔code validation must work on the project's *actual* stack (Python, PHP, bash, CSS, TS, JS, Go/Java/Ruby/…), not just Rust, or the integration advantage is a Rust demo. *Hard design settled 2026-06-19, shaped in [ideas/multi-language-doc-code.md](ideas/multi-language-doc-code.md); unscheduled.* M27 shipped six languages' `symbol-exists` + proved the extension model; **M28 shipped CSS** — the first breadth target, proving the HD1 *addressable-unit* keystone on the most-different model (no `.name`-field declarations); **M29 shipped YAML/docker-compose** — the second breadth target and the first language to *ride* the finished seam (mapping keys as addressable units), proving the keystone generalizes cheaply. The shaped path: **"symbol" generalizes to "addressable unit"** (the keystone — CSS selectors / YAML keys / declarations, one uniform model, zero schema change); `symbol-exists` breadth scales as a grammar-add (CSS first); the `is-a-test` predicate tiers by confidence (attribute-based blocks, name-convention advises, JS/TS exact label-match blocks); grammars bundle-all until binary size forces pluggability. Promotes into [validation.md](design/validation.md) → Multi-language resolution milestone by milestone. Subsumes the CSS+docker-compose and non-Rust-`is-a-test` deferrals ([decisions-pending.md](implementation/decisions-pending.md)).
- **Multi-pack composition** — can one project use more than one domain pack at once (e.g. dev + docs-writing), and how do packs compose in the cascade? *Scheduled M14 (the post-spine arc terminus); design promoted to [multi-pack.md](design/multi-pack.md) at M14 planning 2026-06-08.* The pack-default layer becomes an ordered set of packs with deterministic precedence-override collision resolution and pack-local include resolution; bounded to our two packs (dev + methodology), mechanism general-but-internal (not a public pack-authoring API).
- **Legacy ingestion / migration** — the `project setup (existing project)` flow must ingest docs in inconsistent states. *Detect-and-route shipped M9/M21; the **transform** arm (rewrite a foreign doc to conformant shape + adopt) is scheduled — changelog-first as **M23**, design promoted to [auto-migration.md](design/auto-migration.md); location-bearing doctypes + code-inferred docs generalize at M24.* **Settled less research-grade than first feared:** under **Framing A** the LLM rewrites the foreign prose and the CLI strict-parses + adopts iff conformant — **no CLI fuzzy heading-mapping**, so the determinism boundary stays intact (the boundary table above is unchanged). The residue is migration *fidelity*, handled by a human review gate.
- **Self-hosting — distill the harness into a jigc pack, dogfood it on a fresh project** — *scheduled M12 (exploratory); design promoted to [self-hosting.md](design/self-hosting.md) at M12 planning 2026-06-07 (from `ideas/self-hosting.md`).* Graduate the hand-run methodology (the [planning](implementation/milestone-planning-workflow.md)/[increment](implementation/increment-workflow.md)/[dev](implementation/dev-workflow.md)/[completion](implementation/milestone-completion-workflow.md) workflows + their folded-in learnings) into a **jigc methodology pack**, then bootstrap a new project with jigc *driving* it. The project's designed terminus (the workflows already call themselves the dogfood for the product workflow). Encoding-as-jigc is a forcing function sorting each lesson into a **three-way cut** — *mechanizable* (→ step/probe) · *irreducible judgment* (stays the agent's) · *owner-assigned recorded artifact* (the M8 genuine-spawn class). M12's bounded first slice encodes the **dev-workflow only, reduced-linear**; the dialect surface for the harder workflows is **discovered by attempting the encode** (a dedicated dialect-extension milestone is the expected outcome).

- **Differentiator-engaging twin pilot — clear the value gate, reversibly** — the M17-relocated hypothesis: does jigc beat a static `CLAUDE.md` when the **differentiators** (managed docs, validation, drift, forward-refs, supersession) are actually engaged, with a fitting domain pack? M17 tied on a bare `dev-task` (no `allows-create` → no differentiator touched) and named two un-run cases (P2 existing-docs/doc↔code, P3 greenfield). *Shaped 2026-06-21 in [ideas/differentiator-pilot.md](ideas/differentiator-pilot.md); unscheduled.* Runs on a **twin** with plain-file export — **zero lock-in**, so it yields the value signal *before* any productive commitment. Pairs with the productive-go gate (M33 schema/format freeze + M34 corpus-migration; [READINESS-ASSESSMENT.md](READINESS-ASSESSMENT.md), [DECISIONS.md](DECISIONS.md) → 2026-06-21 Readiness assessment).

**MVP-blocking:** *none currently open — the emitted-format micro-syntax was settled 2026-05-28 ([workflow-dialect.md](design/workflow-dialect.md#emitted-format)).*

**Per part-doc opens (linked):**
- [structural-grammar.md](design/structural-grammar.md#open-questions) — multi-level repetition; minting mechanics (slug normalization, collision-suffix form).
- [document-type-schema.md](design/document-type-schema.md#open-questions) — inline references in slot prose.
- [write-commands.md](design/write-commands.md#open-questions) — form-marker syntax; `import` three-way merge; blocked/error payload.
- [workflow-dialect.md](design/workflow-dialect.md#open-questions) — workflow progress/resumption; `milestone-execution` orchestration.
- [reconciliation.md](design/reconciliation.md#open-questions) — parser-tolerant vs parser-strict for cosmetic drift; concurrent OOB edits during a task; external-edit notification surface.
- [finalize.md](design/finalize.md#open-questions) — multi-doc promotion ordering; `finalize --dry-run`; commit-msg hook output capture.
- [command-catalog.md](design/command-catalog.md#open-questions) — per-workflow command-ref scoping; stdin as data-value binding; multi-target / variant commands.
- [storage.md](design/storage.md#open-questions) — multi-slot sub-label syntax; config internal structure. *(Milestone worktree orchestration is settled — closed with worktrees, for **code** only; M31.)*
- [validation.md](design/validation.md#open-questions) — pack-probe sandboxing; `doc-code` logic; findings recomputed vs cached.
- [overrides.md](design/overrides.md#open-questions) — per-developer `local` layer; team-layer distribution; committed-config-dir layout.
- [assistant-adapter.md](design/assistant-adapter.md#open-questions) — profiles beyond Claude Code; hook events per assistant.
- [implementation/parsing.md](implementation/parsing.md#open-questions) — span precision under stress; multi-slot sub-label syntax (cross-ref); mentions-in-prose scanning.
- [implementation/module-layout.md](implementation/module-layout.md#open-questions) — engine module → crate splits; embedded-resource mechanism; subprocess probe contract.

**Resolved here, moved to `design/`** (with the *why* in [DECISIONS.md](DECISIONS.md)):
- Bootstrap sentence → [bootstrap.md](design/bootstrap.md)
- Write-command vocabulary · content handoff · proposal staging → [write-commands.md](design/write-commands.md)
- State location & concurrency · sub-agent state-merge · on-disk diff-review format → [storage.md](design/storage.md)
- Composition · sub-agent spawn/ack · placeholder vs slot delimiters · address-vs-content (`@` rule) → [workflow-dialect.md](design/workflow-dialect.md) (slots also in [document-type-schema.md](design/document-type-schema.md))
- Override deltas · the cascade · defaults-versioning discipline · the 9-phase resolution algorithm (by-id shadowing → scalar deltas → structural deltas → slot-fills → cycle check → expansion → resolve → emit) → [overrides.md](design/overrides.md)
- Doc-type schema definition format · cross-references as `ref` fields with relation metadata (`to`, `card`, `inverse`, `inverse-card`) → [document-type-schema.md](design/document-type-schema.md)
- Addressing grammar (variable-depth, one grammar for every reference) · work-unit family identity · ID-source = title field rendered as heading → [structural-grammar.md](design/structural-grammar.md)
- `jigc start` semantics (four forms; orthogonal `creates-task` + `default-workflow` knobs) · task-id collision policy (reject in serial, suffix in parallel) → [write-commands.md](design/write-commands.md)
- Emitted format — four-class micro-syntax (`Run:` · `>` · `<<author:>>` · prose) → [workflow-dialect.md](design/workflow-dialect.md)
- `--explain` resolution-tree output contract → [workflow-dialect.md](design/workflow-dialect.md)
- `describe` self-description surface (facts-not-advice author-enforced · non-contractual enforced by format · cascade reflection via whole-file definition shadow · authored `description:`/`usage:` on workflows + doctypes) → [introspection.md](design/introspection.md) *(M11; promoted from `ideas/describe.md`)*
- Multi-pack composition (pack-default = an ordered set of packs · precedence-override whole-definition collision resolution · pack-local include resolution · multi-pack provenance · single-pack byte-identity floor) → [multi-pack.md](design/multi-pack.md) *(M14)*
- Finalize transaction (seven phases, atomicity rules, rollback discipline, dirty-tree policy) · commit-doc → git-message rendering → [finalize.md](design/finalize.md)
- Out-of-band reconciliation (state machine, parse classifier, hash re-baselining, strict-MVP auto-repair scope) → [reconciliation.md](design/reconciliation.md)
- Validation severity inventory (every MVP check; intrinsic vs tunable; cascade-key convention; locked-demotion rule) → [validation.md](design/validation.md)
- Command catalog (typed args list: literal / `from:` / `agent:`; `<NAME>` agent-substitution markers; POSIX shell-safety; cascade-override behavior) → [command-catalog.md](design/command-catalog.md)
- Workflow-gated `create` (the `allows-create: [<doctype-id>, ...]` workflow front-matter knob; default empty; cascade-overridable at `workflows.<id>.allows-create`; three rejection cases with structured error+route) → [write-commands.md](design/write-commands.md) and [workflow-dialect.md](design/workflow-dialect.md)
- Edge-index lifecycle (five sites: committed rebuild, working overlay, OOB absorb, fan-out join, finalize commit; the "never persisted with task deltas in it" rule) → [storage.md](design/storage.md)
- Orientation output examples (four states: unset / clean / active task / blocked task) → [bootstrap.md](design/bootstrap.md)
- Worked examples (MVP single-task with optional ADR · OOB reconciliation · override application at compose time · finalize-to-git) → [worked-examples.md](design/worked-examples.md)
- Assistant adapter (neutral core + per-assistant profile) → [assistant-adapter.md](design/assistant-adapter.md)
- Slot / field-group boundary discipline (sentinel-marked field groups via `<!-- fields -->`; slot heading-depth ceiling at `##`/`###`) — eliminates content-sniffing at the parse-time slot boundary → [storage.md](design/storage.md) and [implementation/parsing.md](implementation/parsing.md)
- OOB rename detection (path-rename is identity-change; two-tier signal: strong = path missing + content-hash match, weak = path missing alone; MVP routes to revert, `jigc doc rename` post-MVP) → [reconciliation.md](design/reconciliation.md) and [write-commands.md](design/write-commands.md)
- Pack-probe determinism contract (six rules + four meta-finding failure modes; locked now, OS-level sandboxing implementation post-MVP) → [validation.md](design/validation.md)
- Bootstrap compaction resilience (universal routing footer on every CLI agent-text output + per-assistant resume hook seam) → [bootstrap.md](design/bootstrap.md), [workflow-dialect.md](design/workflow-dialect.md), [assistant-adapter.md](design/assistant-adapter.md)
