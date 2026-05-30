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

There are **no per-command wrappers**. The agent shell-calls `tool` and learns each command just-in-time from composed output (command-refs); a wrapper-per-command would be a static, partial surface that undercuts the "one entrypoint" promise. What the adapter *does* generate is a small, **catalog-derived** set of **human-facing workflow launchers** (e.g. a Claude Code slash-command per workflow), each just calling `tool start --workflow <X>` — the *human's* path to kick off a specific workflow directly. That stays consistent with the rule: it's bounded and **regenerated from the workflow catalog** (not a hand-maintained pile), and it's not the agent's path (the agent still shell-calls the front door).

## The three responsibilities

### 1. Inject the bootstrap — hook (primary) + line (floor)

The bootstrap reaches the agent by the assistant's *best available* mechanism, declared in the profile:

- **A hook that calls the CLI** — injects the bootstrap plus a *thin live nudge* at session start (e.g. Claude Code's `SessionStart` running the front door). Fresher and higher-salience than a buried rules line, and it makes the bootstrap's "advertise *and* demonstrate" automatic: the agent's first context already shows live orientation. **Primary** where supported.
- **A static line** in the always-loaded file (`CLAUDE.md` / `AGENT.md` / Cursor rules) — the **universal floor**: works where there's no hook system, and cheap insurance if the hook fails.
- **A resume / post-compaction hook** (capability-dependent) — *if* the assistant exposes a "context resumed" or "compaction completed" event, the adapter re-injects the bootstrap on that event so a mid-task compaction doesn't lose the routing pointer. The profile's `resume` event slot declares whether the assistant supports it; Claude Code ships this when the API exposes such an event. The universal compaction baseline (independent of any hook capability) is the routing footer the CLI appends to every agent-facing output ([workflow-dialect.md](workflow-dialect.md#emitted-format) → Routing footer; [bootstrap.md](bootstrap.md) → Context compaction resilience).

**Discipline:** the hook obeys the same *routing, not content* rule — it injects the bootstrap + at most a one-line current-state nudge, **never a content dump**. Its value is freshness and salience, not volume; the agent still *pulls* real context JIT via `tool`. (Capability-dependent: the profile declares which mechanisms the assistant supports; the line is the floor.)

### 2. Make `tool` frictionless

Allowlist `tool` in the assistant's permission/settings so the agent runs it without friction. This is the **path-of-least-resistance** the bootstrap *depends on* — if `tool` prompts every time, the agent routes around it and the whole bet fails. Generating this makes it a guaranteed setup step, not something a human must remember.

*Honest boundary: this is enforcement by **ergonomics**, not by sandbox.* A non-compliant agent that decides to ignore the adapter and edit files directly bypasses us — there is no kernel-level block, and the architecture explicitly does not promise one ([VISION.md](../VISION.md) principle #3). The bet: frictionless `tool` access + bootstrap *advertise+demonstrate* + "the CLI is the only path that knows the wiring" make compliance the cheaper path. Out-of-band edits, when they happen, are detected and reconciled via the engine-native `file-state` probe ([write-commands.md](write-commands.md) → Out-of-band reconciliation), not prevented.

### 3. Bind the spawn mechanism

The profile carries a **launch template**. The CLI renders its fan-out dispatch (`task_id` + entrypoint) *through* the template into the assistant's launch primitive — for Claude Code, a Task-tool invocation running `tool workflow W --task <sub>`. The locked seam holds: **CLI owns the payload; the adapter owns the launch** — and the template is the *only* assistant-specific bit.

**Launch templates are typed-schema validated at install.** The template is the actual surface that decides whether the sub-agent ever sees the CLI's payload, so an unvalidated template is a determinism-boundary leak: a stale profile could omit the `{{task_id}}` placeholder, paraphrase inline instructions, or fail to invoke `tool workflow` — and the sub-agent's writes would never reach the working area the join expects. The profile's launch template carries a schema declaring:

- **required placeholders** the CLI fills (`{{task_id}}`, `{{workflow}}`);
- **the required invocation pattern** — the template must invoke `tool workflow --task <id>` (the rendered command, however the assistant primitive surfaces it, must shell out to the CLI; no inline-paraphrased instructions to the sub-agent);
- **forbidden patterns** — long inline blocks that would suggest paraphrased step prose, anything that would look like a workflow body re-authored in the template.

Validation runs at `tool setup` / `tool adapter install` / `tool adapter regenerate`. **A broken template is an install-time error** with a precise pointer to the schema violation; the install does not complete. The runtime backstop — the join's **never-started detection** ([workflow-dialect.md](workflow-dialect.md) → `fan-out` / `join`) — catches the case where a template *passes* schema but somehow still fails to reach the CLI (the assistant changed an API, the user mis-wrapped the template, the spawn primitive misfires). Two complementary layers, each catching a failure mode the other can't: install-time catches the broken template before any sub-agent runs; the runtime backstop surfaces the silent "sub-agent never started" case as a distinct routing outcome rather than a deadlock.

## The adapter profile

A small YAML spec of "how to wire into assistant X" (illustrative):

```yaml
# adapters/claude-code.yaml
assistant: claude-code
inject:
  - line: { file: CLAUDE.md, scope: project-root }            # universal floor
  - hook: { event: SessionStart, run: "tool start" }          # primary; calls the CLI — bare `tool start` is read-only orientation
  - hook: { event: Resume, run: "tool start", when: supports(resume) }  # post-compaction re-injection (ships when the assistant API exposes the event)
allowlist:
  file: .claude/settings.json
  permit: ["tool *"]
spawn:
  template: "Use your Task tool to run: `tool workflow {{workflow}} --task {{task_id}}`"
```

Profiles for known assistants **ship with the CLI** (Claude Code first); additional profiles are **installable** later — a swappable layer parallel to packs.

## Open questions

- **Profiles beyond Claude Code** — Cursor, Codex, and others follow the same model; their concrete profiles (events, files, spawn primitive) are incremental.
- **Hook event(s) per assistant** — the precise session-start (and any other) hook event for each assistant; for Claude Code, `SessionStart` running bare `tool start` is settled (the orientation output *is* the nudge).
