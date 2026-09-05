# Bootstrap sentence

The single permanent-context line that wires an agent into the system — the thesis's punchline. Every other tool stuffs the context window with a bloated static rules file; this replaces all of it with **one routing pointer**, because the real content is composed just-in-time ([workflow-dialect.md](workflow-dialect.md)). It resolves [VISION.md](../VISION.md)'s "the agent knows only **one entrypoint sentence**" outcome.

For the *why*, see [DECISIONS.md](../DECISIONS.md). The command tokens below (`jigc`, `jigc start`) are placeholders pending the product/command name.

## The sentence

> **`jigc` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `jigc start`; write every change back through `jigc`.**

## Its three jobs — and nothing more

1. **Source + benefit** — establishes `jigc` as the *interface* and the single, *current, assembled, validated* source for the agent's needs. It names the **categories** (workflow · project state · doc context), never their contents.
2. **Prohibition** — *"files are storage, not your interface."* Scoped to **managed docs** (specs/ADRs/PRDs/…); the agent still reads and writes **code** normally.
3. **Front door + write channel** — one state-aware entry (`jigc start`); every change to managed docs goes back through `jigc`.

## Routing, not content — the discipline

The sentence's whole value is that it **routes** and never **contains**. Naming the *categories* jigc is the source for is routing; embedding a convention, a workflow, or a doc list would be content — and the instant it does, we've reinvented the bloated `CLAUDE.md` it exists to kill. Categories are stable; contents rot. The line carries no rule that the CLI couldn't compose fresh on demand.

## The state-aware front door

The agent's *entire* a-priori knowledge is "run the front door and follow it." Running it requires no memorized vocabulary: the CLI composes whatever the project state warrants — orientation, the right workflow for the task, or *"this project isn't set up — here's setup."* Every other command the agent ever needs is learned **just-in-time**, exactly when a composed step names it (a command-ref resolved through the [command catalog](command-catalog.md) into a literal `Run: ` directive). One door; the CLI teaches the rest.

The front door is also where a **task is born**: `jigc start "<intent>"` composes the cascade's default workflow ([write-commands.md](write-commands.md) → Task origination), and the workflow's own **`creates-task`** declaration ([workflow-dialect.md](workflow-dialect.md)) determines whether running it mints a task. The MVP's default workflow is `single-task` (`creates-task: true`), so one call mints; post-MVP the cascade default may flip to a router (`creates-task: false`) that emits an explicit `jigc start --workflow X "<intent>"` call, which then mints. Either way, "start every task with `jigc start`" is literal — it is both the entry point and the task-create path.

### Orientation output examples

Concrete shapes the orientation states render to. Bare `jigc start` never mints, never stages, never composes a side-effectful workflow — it reports. (Reporting a live task's findings runs the task-scope sweep, so it is not *inert*: it reads git and materializes the derived edge-index cache. See [validation.md](validation.md).) The CLI prints orientation as plain text; the `Run:` directives below follow the [emitted format](workflow-dialect.md#emitted-format) so the agent can spot next-step commands without parsing structure.

**Three states ship, not four.** States 3 and 4 below differ only by whether the findings a task carries are blocking, so one **active-task** view renders both — it carries the *active set*, one block per live task, each with its findings as data (M50 → the Settle, D3). Two live tasks are two blocks under one state, which is legal ([write-commands.md](write-commands.md) → Task origination). **The workflow catalog rides the active state too**, beneath the work in progress and elided from the examples below for length: the `create.gate-blocked` refusal routes with *"`jigc start` lists the catalog"* and can only ever fire while a task is live, so a state that dropped the catalog would break that route in exactly the state that prints it.

**1. Unset project** — no pack installed, no cascade resolved:

```text
jigc — orientation

This project isn't set up. No project config layer is present; the cascade has only pack defaults.

Run: `jigc setup`

`jigc setup` installs jigc into this project (the adapter, the `jigc` allowlist, the `.jigc/config/` layer). Then `jigc start` orients you to the setup workflows — `project-setup` (develop a new project's idea into its first requirements) or `ingest-existing` (bring an existing repo's docs under management). See [project-setup.md](project-setup.md).
```

**2. Clean project, no active task** — pack installed, no in-progress work:

```text
jigc — orientation

Pack: dev/v0.3.0 · Project config: .jigc/config/ · Branch: main (HEAD a3f9c2)

Available workflows:
  - single-task    — Implement one well-scoped change against an existing spec.
  - project-setup  — Bootstrap a brand-new project — develop the idea into its first requirements.

Recent: 3 finalizations on this branch · last: add-rate-limiter (2026-05-27)

Run: `jigc start "<intent>"`   — presents the workflows above; pick one, then re-run with `--workflow <chosen>` to compose it
```

**3. Active task** — a task in progress, not blocked:

```text
jigc — orientation

Pack: dev/v0.3.0 · Project config: .jigc/config/

Active task: add-rate-limiter
  workflow: single-task
  intent:   add a rate limiter to the ingest endpoint
  base:     a3f9c2
  staged:   commit:add-rate-limiter
  findings: none

Run: `jigc start --task add-rate-limiter`   — resume
Run: `jigc task validate add-rate-limiter`   — preview the blockers this side of the commit
Run: `jigc task finalize add-rate-limiter`   — validate + commit
Run: `jigc task discard add-rate-limiter --force`   — abandon
```

The abandon directive carries its **consent** here because the mint stages the task's commit doc, and `jigc task discard` refuses over staged docs no commit has a copy of ([write-commands.md](write-commands.md) → Lifecycle). Over a task staging nothing — a milestone sub-task before its first write — the flag is *not* printed: a route this binary's own guard blocks is a route-floor defect, and so is teaching a consent flag at a door that never asks for it, so the printed argv is the one that runs in each state.

**A milestone sub-task's commit directive names the milestone door.** Its area is an ordinary `.jigc/tasks/<id>/`, so it rides the active set like any other task — but `jigc task finalize <sub>` refuses outright, the milestone boundary being its only commit boundary ([team-ready-state.md](team-ready-state.md) → The lifecycle), so the block routes to `jigc milestone finalize <m>` and says why.

**4. Blocked task** — a task with a blocking finding:

```text
jigc — orientation

Pack: dev/v0.3.0 · Project config: .jigc/config/

Active task: add-rate-limiter
  workflow: single-task
  intent:   add a rate limiter to the ingest endpoint
  base:     a3f9c2
  staged:   commit:add-rate-limiter
  findings: 1 blocking

blocking · workflow-refs.binding-present — {{ task.spec#criteria }} resolves to no spec — task.spec binding missing.
  route: `jigc task bind spec spec:auth-flow add-rate-limiter` to set the binding

Run: `jigc start --task add-rate-limiter`   — resume
Run: `jigc task validate add-rate-limiter`   — preview the blockers this side of the commit
Run: `jigc task finalize add-rate-limiter`   — validate + commit
Run: `jigc task discard add-rate-limiter --force`   — abandon
```

State 4 is state 3 with findings, which is why one view renders both: a blocking finding changes what the block *says*, never which directives are reachable — and orientation reports it at **exit 0**, because orientation is not a gate. The findings render through the same finding renderer every other surface uses, so this surface grows no private finding shape ([command-output-contract.md](command-output-contract.md) → findings-as-data). A task whose sweep cannot run — no `doc-code` probe beside the binary on a fresh clone, say — reads `findings: unknown — <reason>`: the bootstrap door degrades, and *unknown* is never printed as *none*.

The shapes share a header (pack/cascade summary), a status block, and at most a small set of `Run:` directives for next steps. Content-class findings are the engine's voice. Author-class (`<<author: …>>`) never appears — orientation never asks the agent to write.

**Not yet built, keyed to their trigger** ([decisions-pending.md](../implementation/decisions-pending.md)): the header's `Branch: main (HEAD …)` segment, the clean state's `Recent: N finalizations …` line, and the active block's `step N/M (<step>)` progress. Each needs a read this view does not make (git HEAD, the finalize history, a per-task step cursor).

## Benefit detection — why it advertises, not just instructs

A thin *"run `jigc start` for instructions"* loses to the agent's instinct to grep. So the sentence makes the agent **perceive** `jigc` as the superior source: **authoritative + current + assembled-for-this-task**, versus files which are raw, scattered, and possibly stale. The first `jigc` call then **demonstrates** that breadth (its output is self-describing — what workflows exist, the project's status, the task's context). Advertise in the line, prove on first use.

*Honest boundary:* one permanent line is **necessary but not sufficient** — compliance also rests on the front door being the genuine **path of least resistance**. If `jigc` returns better context faster than grepping, the agent routes through it; wording carries weight, ergonomics enforce it.

## Context compaction resilience

The bootstrap is injected at session start, but the agent's context can be compacted mid-task — early messages get summarized or dropped to make room for new ones. A compaction pass can drop or dilute the bootstrap injection (the hook ran once; the static line in `CLAUDE.md` is more durable but compaction can summarize it away too). The agent then resumes without the routing intent — back to grep-and-guess.

Two defenses, complementary:

- **Routing footer on every CLI output** (universal). Every composed workflow / `jigc start` / `jigc task validate` output in agent-text and human-pretty format ends with a one-line footer: `— jigc · run \`jigc start\` for orientation; all writes through \`jigc\`.` Self-reinforcing — every CLI call re-shows the routing pointer, so compaction can drop one injection but can't drop the footer the agent re-sees on every CLI call. JSON-format output (consumed by code, not the agent's reading flow) carries no footer. Stays within the *routing, not content* discipline: names the entrypoint, embeds no rules. Spec lives in [workflow-dialect.md](workflow-dialect.md#emitted-format) → Routing footer.
- **Resume / post-compaction hook** (per-assistant capability). The adapter profile carries a `resume` event slot parallel to `SessionStart` ([assistant-adapter.md](assistant-adapter.md) → Inject the bootstrap). If the assistant supports a "context resumed" or "compaction completed" event, the adapter re-injects the bootstrap on that event. Claude Code: ships when the API supports such an event; the seam is designed, no implementation gated on it.

Together: the footer is the always-on baseline (no assistant capability required); the resume hook is the per-assistant strengthening for assistants that expose the event. Same shape as bootstrap injection itself (universal floor + best-available primary).

## Placement is the adapter

The sentence is **assistant-neutral**. *How and where* it reaches the agent — a static line in the always-loaded file (`CLAUDE.md` / `AGENT.md` / Cursor rules) and/or a **hook that calls the CLI** to inject it with a thin live nudge — is the thin, assistant-specific **adapter**, generated from a per-assistant profile. The adapter (injection + a `jigc` allowlist + the spawn binding) is the entire integration surface, and it is *routing, not content*. See [assistant-adapter.md](assistant-adapter.md).

## Open questions

- **Command / product name** — the `jigc` / `jigc start` tokens resolve once the product is named.
- **Per-assistant adapter** — the model and the Claude Code profile are designed in [assistant-adapter.md](assistant-adapter.md); profiles for other assistants follow the same model.
