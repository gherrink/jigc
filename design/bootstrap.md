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

The sentence's whole value is that it **routes** and never **contains**. Naming the *categories* the tool is the source for is routing; embedding a convention, a workflow, or a doc list would be content — and the instant it does, we've reinvented the bloated `CLAUDE.md` it exists to kill. Categories are stable; contents rot. The line carries no rule that the CLI couldn't compose fresh on demand.

## The state-aware front door

The agent's *entire* a-priori knowledge is "run the front door and follow it." Running it requires no memorized vocabulary: the CLI composes whatever the project state warrants — orientation, the right workflow for the task, or *"this project isn't set up — here's setup."* Every other command the agent ever needs is learned **just-in-time**, exactly when a composed step names it (a command-ref resolved through the [command catalog](command-catalog.md) into a literal `Run: ` directive). One door; the CLI teaches the rest.

The front door is also where a **task is born**: `jigc start "<intent>"` composes the cascade's default workflow ([write-commands.md](write-commands.md) → Task origination), and the workflow's own **`creates-task`** declaration ([workflow-dialect.md](workflow-dialect.md)) determines whether running it mints a task. The MVP's default workflow is `single-task` (`creates-task: true`), so one call mints; post-MVP the cascade default may flip to a router (`creates-task: false`) that emits an explicit `jigc start --workflow X "<intent>"` call, which then mints. Either way, "start every task with `jigc start`" is literal — it is both the entry point and the task-create path.

### Orientation output examples

Concrete shapes the four orientation states render to. All are read-only — bare `jigc start` never mints, never stages, never composes a side-effectful workflow. The CLI prints orientation as plain text; the `Run:` directives below follow the [emitted format](workflow-dialect.md#emitted-format) so the agent can spot next-step commands without parsing structure.

**1. Unset project** — no pack installed, no cascade resolved:

```text
jigc — orientation

This project isn't set up. No domain pack is installed; the cascade has only engine defaults.

Run: `jigc setup`

The setup workflow walks the pack choice, the project config dir, and the first workflow.
```

**2. Clean project, no active task** — pack installed, no in-progress work:

```text
jigc — orientation

Pack: dev/v0.3.0 · Project config: .jigc-config/ · Branch: main (HEAD a3f9c2)

Available workflows:
  - single-task    — Implement one well-scoped change against an existing spec.
  - project-setup  — Set up the development pack on a fresh repo.

Recent: 3 finalizations on this branch · last: add-rate-limiter (2026-05-27)

Run: `jigc start "<intent>"`   — composes the default workflow (single-task)
```

**3. Active task** — a task in progress, not blocked:

```text
jigc — orientation

Pack: dev/v0.3.0 · Project config: .jigc-config/ · Branch: main (HEAD a3f9c2)

Active task: add-rate-limiter
  workflow: single-task · step 2/3 (implement) · base: a3f9c2
  staged:   commit:add-rate-limiter#type, commit:add-rate-limiter#summary
  findings: none

Run: `jigc task validate add-rate-limiter`   — preview blockers
Run: `jigc task finalize add-rate-limiter`   — validate + commit
Run: `jigc task discard add-rate-limiter`    — abandon
```

**4. Blocked task** — a task with a blocking finding:

```text
jigc — orientation

Pack: dev/v0.3.0 · Project config: .jigc-config/ · Branch: main (HEAD a3f9c2)

Active task: add-rate-limiter
  workflow: single-task · step 3/3 (finalize) · base: a3f9c2
  findings: 1 blocking

> workflow-refs · blocking
> {{ task.spec#criteria }} resolves to no spec — task.spec binding missing.

Run: `jigc task bind --role spec --addr spec:auth-flow`   — set the binding
Run: `jigc task discard add-rate-limiter`                 — abandon
```

The four shapes share a header (pack/cascade summary or active-task line), a status block, and at most a small set of `Run:` directives for next steps. The blockquoted finding in example 4 is Content-class — the engine's voice. Author-class (`<<author: …>>`) never appears — orientation never asks the agent to write.

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
