# Bootstrap sentence

The single permanent-context line that wires an agent into the system — the thesis's punchline. Every other tool stuffs the context window with a bloated static rules file; this replaces all of it with **one routing pointer**, because the real content is composed just-in-time ([workflow-dialect.md](workflow-dialect.md)). It resolves [VISION.md](../VISION.md)'s "the agent knows only **one entrypoint sentence**" outcome.

For the *why*, see [DECISIONS.md](../DECISIONS.md). The command tokens below (`tool`, `tool start`) are placeholders pending the product/command name.

## The sentence

> **`tool` is your interface to this project — your single, current source for the workflow for your task, the project's state, and the doc context you need, all assembled and validated for you. The files are storage, not your interface: never read or edit managed docs directly. Start every task with `tool start`; write every change back through `tool`.**

## Its three jobs — and nothing more

1. **Source + benefit** — establishes `tool` as the *interface* and the single, *current, assembled, validated* source for the agent's needs. It names the **categories** (workflow · project state · doc context), never their contents.
2. **Prohibition** — *"files are storage, not your interface."* Scoped to **managed docs** (specs/ADRs/PRDs/…); the agent still reads and writes **code** normally.
3. **Front door + write channel** — one state-aware entry (`tool start`); every change to managed docs goes back through `tool`.

## Routing, not content — the discipline

The sentence's whole value is that it **routes** and never **contains**. Naming the *categories* the tool is the source for is routing; embedding a convention, a workflow, or a doc list would be content — and the instant it does, we've reinvented the bloated `CLAUDE.md` it exists to kill. Categories are stable; contents rot. The line carries no rule that the CLI couldn't compose fresh on demand.

## The state-aware front door

The agent's *entire* a-priori knowledge is "run the front door and follow it." Running it requires no memorized vocabulary: the CLI composes whatever the project state warrants — orientation, the right workflow for the task, or *"this project isn't set up — here's setup."* Every other command the agent ever needs is learned **just-in-time**, exactly when a composed step names it (a command-ref). One door; the CLI teaches the rest.

The front door is also where a **task is born**: `tool start "<intent>"` routes the agent to a workflow (via the router) and `--workflow <X>` mints the task and composes it ([write-commands.md](write-commands.md) → Task origination). So "start every task with `tool start`" is literal — it is both the entry point and the task-create verb.

## Benefit detection — why it advertises, not just instructs

A thin *"run `tool start` for instructions"* loses to the agent's instinct to grep. So the sentence makes the agent **perceive** `tool` as the superior source: **authoritative + current + assembled-for-this-task**, versus files which are raw, scattered, and possibly stale. The first `tool` call then **demonstrates** that breadth (its output is self-describing — what workflows exist, the project's status, the task's context). Advertise in the line, prove on first use.

*Honest boundary:* one permanent line is **necessary but not sufficient** — compliance also rests on the front door being the genuine **path of least resistance**. If `tool` returns better context faster than grepping, the agent routes through it; wording carries weight, ergonomics enforce it.

## Placement is the adapter

The sentence is **assistant-neutral**. *How and where* it reaches the agent — a static line in the always-loaded file (`CLAUDE.md` / `AGENT.md` / Cursor rules) and/or a **hook that calls the CLI** to inject it with a thin live nudge — is the thin, assistant-specific **adapter**, generated from a per-assistant profile. The adapter (injection + a `tool` allowlist + the spawn binding) is the entire integration surface, and it is *routing, not content*. See [assistant-adapter.md](assistant-adapter.md).

## Open questions

- **Command / product name** — the `tool` / `tool start` tokens resolve once the product is named.
- **Per-assistant adapter** — the model and the Claude Code profile are designed in [assistant-adapter.md](assistant-adapter.md); profiles for other assistants follow the same model.
