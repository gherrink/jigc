# Assistant adapter

The thin, per-assistant shim between the neutral CLI and a specific coding assistant (Claude Code, Cursor, Codex). It is the *entire* integration surface — kept tiny and **CLI-generated** so it can never become the next bloated rules-pile.

Builds on [VISION.md](../VISION.md) (assistant-neutral core + thin adapter; the sub-agent seam), [bootstrap.md](bootstrap.md) (the sentence it injects), [workflow-dialect.md](workflow-dialect.md) (the fan-out dispatch it binds), and [overrides.md](overrides.md) (config-family YAML). For the *why*, see [DECISIONS.md](../DECISIONS.md). Notation is **illustrative**.

## Neutral core, per-assistant profile

The core CLI knows nothing about any assistant. **All** assistant-specific knowledge lives in a small **adapter profile** — *how to inject* the bootstrap, *how to allowlist* `tool`, *how to spawn* sub-agents. Swap the profile → new assistant, zero core changes.

This is the second of two **orthogonal extension axes**, the core neutral to both:

- **domain** → a **pack** (development is the first).
- **assistant** → an **adapter profile** (Claude Code is the first).

The profile is a **config-family artifact** (YAML, like schemas and manifests): config is YAML, content is its native format.

## Generated, minimal, regenerated

`tool setup` (or `tool adapter install --assistant claude-code`) **generates** the adapter from the profile and **regenerates it on upgrade**, so the integration stays current and can't rot — the thesis applied to integration itself: structure is CLI-owned, not hand-maintained.

There are **no per-command wrappers**. The agent shell-calls `tool` and learns each command just-in-time from composed output (command-refs); a wrapper-per-command would be a static, partial surface that undercuts the "one entrypoint" promise. (Optional human-facing slash-commands could exist as sugar, but they are not the agent's path.)

## The three responsibilities

### 1. Inject the bootstrap — hook (primary) + line (floor)

The bootstrap reaches the agent by the assistant's *best available* mechanism, declared in the profile:

- **A hook that calls the CLI** — injects the bootstrap plus a *thin live nudge* at session start (e.g. Claude Code's `SessionStart` running the front door). Fresher and higher-salience than a buried rules line, and it makes the bootstrap's "advertise *and* demonstrate" automatic: the agent's first context already shows live orientation. **Primary** where supported.
- **A static line** in the always-loaded file (`CLAUDE.md` / `AGENT.md` / Cursor rules) — the **universal floor**: works where there's no hook system, and cheap insurance if the hook fails.

**Discipline:** the hook obeys the same *routing, not content* rule — it injects the bootstrap + at most a one-line current-state nudge, **never a content dump**. Its value is freshness and salience, not volume; the agent still *pulls* real context JIT via `tool`. (Capability-dependent: the profile declares which mechanisms the assistant supports; the line is the floor.)

### 2. Make `tool` frictionless

Allowlist `tool` in the assistant's permission/settings so the agent runs it without friction. This is the **path-of-least-resistance** the bootstrap *depends on* — if `tool` prompts every time, the agent routes around it and the whole bet fails. Generating this makes it a guaranteed setup step, not something a human must remember.

### 3. Bind the spawn mechanism

The profile carries a **launch template**. The CLI renders its fan-out dispatch (`task_id` + entrypoint) *through* the template into the assistant's launch primitive — for Claude Code, a Task-tool invocation running `tool workflow W --task <sub>`. The locked seam holds: **CLI owns the payload; the adapter owns the launch** — and the template is the *only* assistant-specific bit.

## The adapter profile

A small YAML spec of "how to wire into assistant X" (illustrative):

```yaml
# adapters/claude-code.yaml
assistant: claude-code
inject:
  - line: { file: CLAUDE.md, scope: project-root }            # universal floor
  - hook: { event: SessionStart, run: "tool start --orient" } # primary; calls the CLI
allowlist:
  file: .claude/settings.json
  permit: ["tool *"]
spawn:
  template: "Use your Task tool to run: `tool workflow {{workflow}} --task {{task_id}}`"
```

Profiles for known assistants **ship with the CLI** (Claude Code first); additional profiles are **installable** later — a swappable layer parallel to packs.

## Open questions

- **Profiles beyond Claude Code** — Cursor, Codex, and others follow the same model; their concrete profiles (events, files, spawn primitive) are incremental.
- **Hook event(s) + nudge content** — the exact session-start (and any other) event per assistant, and the precise shape of the thin live nudge.
