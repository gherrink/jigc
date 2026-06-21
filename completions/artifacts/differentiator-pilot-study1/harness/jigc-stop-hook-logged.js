#!/usr/bin/env node
// Always-on jigc doc<->code enforcement (Stop hook).
// Folds in pilot Study-1 learning #1: jigc's validation only fired at `finalize`,
// which agents bypassed — so the gate never bit. This runs `jigc validate` when the
// agent tries to FINISH and blocks the stop while a managed doc references a code
// symbol that no longer exists, feeding the finding back. Uses jigc's own detection;
// only the activation point changes (Stop, not finalize). Caps re-prompts to avoid loops.
const fs = require('fs');
const { execSync } = require('child_process');

let report = '';
try {
  report = execSync('jigc validate 2>&1', { encoding: 'utf8' });
} catch (e) {
  report = (e.stdout || '') + (e.stderr || '');
}

// A real doc<->code drift: a `doc-code` finding marked blocking. (file-state /
// other probes are intentionally NOT enforced here — only doc<->code honesty.)
const hasDrift = /doc-code/.test(report) && /blocking/.test(report);

const counter = '/tmp/jigc-stop-count';
let n = 0;
try { n = parseInt(fs.readFileSync(counter, 'utf8'), 10) || 0; } catch (e) {}

if (hasDrift && n < 4) {
  fs.writeFileSync(counter, String(n + 1));
  const reason =
    'jigc validation: a managed document references a code symbol that no longer ' +
    'exists (doc<->code drift). You must update every documentation reference to the ' +
    'renamed/removed symbol — the architecture doc AND the README (section headings, ' +
    'prose, cross-references, code examples) — so no document names an absent symbol, ' +
    'before finishing. Details:\n\n' + report.trim();
  try { fs.appendFileSync("/out/stop-hook.log", "BLOCK doc-code drift\n"); } catch (e) {}
  process.stdout.write(JSON.stringify({ decision: 'block', reason }));
  process.exit(0);
}

try { fs.appendFileSync("/out/stop-hook.log", "ALLOW (clean or cap)\n"); } catch (e) {}
// Clean, or re-prompt cap reached: allow the stop.
try { fs.unlinkSync(counter); } catch (e) {}
process.exit(0);
