#!/usr/bin/env node
// UserPromptSubmit hook — just-in-time routing nudge (targets Sonnet's
// retrieval-under-execution gap: it holds the AGENT.md instruction but doesn't
// surface it under a bare task). Injects, at prompt time, a directive to route
// through jigc. Tool-routing only (not the doc-consistency rule) — to keep the
// arm distinct from the static-instruction arm.
const fs = require('fs');
try { JSON.parse(fs.readFileSync(0, 'utf8') || '{}'); } catch (e) {}
const ctx =
  "REMINDER — this project is managed by `jigc`. Before editing any files for this " +
  "task, FIRST run `jigc start \"<intent>\"` and make the change through the composed " +
  "jigc workflow; do not edit files directly. jigc assembles the correct workflow and " +
  "validates the change, including keeping the project's managed documentation " +
  "consistent with the code.";
process.stdout.write(JSON.stringify({
  hookSpecificOutput: { hookEventName: "UserPromptSubmit", additionalContext: ctx }
}));
process.exit(0);
