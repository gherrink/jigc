// milestone-build — the reusable build harness for one whole milestone.
//
// Captures the increment-workflow + milestone-completion loops as fixed
// orchestration (the by-hand process, promoted from interpreted docs). Each
// phase is its own agent role, defined under .claude/agents/:
//   build-planner · build-executor · increment-validator · build-fixer ·
//   milestone-code-reviewer · milestone-e2e-tester · milestone-reader.
//
// Per increment (just-in-time, after the prior one lands): plan -> execute
// (one agent per task, full dev-workflow, one commit) -> validate (independent,
// read-only) -> fix (one agent per blocking finding, bounded 3 rounds). Then the
// milestone-completion audit (code review + e2e, parallel) — its findings come
// back for HUMAN TRIAGE; fixes + re-verify run after triage.
//
// Halts (a new fork at plan, a blocked task at execute, still-blocking after 3
// rounds at validate) stop the run and surface a structured reason — prior
// committed work stands.
//
// RESUMING — RULE 0 (the M13 root cause, get this right or nothing replays): ALWAYS
//   re-pass the SAME `args` ({ milestone, base }) on EVERY resume invocation. The
//   agent-call cache key is a CONTENT HASH that includes each call's prompt, and the
//   very first agent (the milestone-reader) embeds the milestone id in its prompt. Omit
//   `args` and `args` is undefined → `milestone` defaults to 'the next milestone' → the
//   reader's prompt changes → its hash misses the cache at call #1 → the ENTIRE prefix
//   re-runs live (and the first increment's planner then halts on already-built work).
//   On M13 this looked like "resume won't fast-forward"; it was actually a dropped-args
//   cache miss. So every resume below is `Workflow({ scriptPath, args: { milestone, base },
//   resumeFromRunId })` — args ALWAYS present. (resumeFromRunId does NOT restore args.)
//
// RESUMING — FIRST distinguish a HALT from an INTERRUPTION (they resume differently):
//   - INTERRUPTION (the run was killed mid-flight — process died, session dropped):
//     the in-flight agent() call never returned, so it is NOT cached — it is a natural
//     cache-miss that re-runs live on a plain resume. But a killed agent may have left a
//     DIRTY TREE (a partial write it hadn't committed, e.g. a planner's uncommitted
//     DECISIONS entry). Before resuming: inspect `git status`, REVERT the partial/
//     uncommitted work (the agent re-does it from a clean base), then plain-resume
//     `Workflow({scriptPath: <snapshot>, args: { milestone, base }, resumeFromRunId})`
//     (args per RULE 0) — NO script surgery (the killed call cache-misses on its own;
//     the committed prefix replays from cache).
//   - HALT (the run returned `{status:'halted'}` cleanly, tree CLEAN): the halted call
//     COMPLETED and its halt-result IS cached — a plain resume replays the cached halt
//     and re-halts. This case needs the script-snapshot surgery in steps 1–5 below.
//   (Tell them apart: a halt left a clean tree + a returned halt report; an interruption
//   left no return value and often a dirty tree. M8 hit both — an interruption mid-inc-5
//   planner, then later a genuine halt at inc-5 Plan.)
//
// RESUMING AFTER A HALT (read this before re-invoking — there is a sharp edge):
//   1. Resolve the blocker ON MAIN, outside the workflow, but DELEGATE — the
//      orchestrator decides, it does not code. The orchestrator (with the human)
//      owns only the JUDGMENT: diagnose the fork, pick the approach. Executing it
//      — edit, full gate green, COMMIT — goes to a `build-fixer` SUBAGENT (Agent
//      tool, agentType 'build-fixer'), a sibling of the workflow; for a genuine
//      fork, surface its options to the human first, then have it apply the choice.
//      The fixer's report carries the commit sha + gate/emitted-command evidence,
//      so the orchestrator resumes from the report WITHOUT re-reading code — keeping
//      the main-session window holding summaries + shas, not file bodies. (Even the
//      diagnosis can be a subagent that returns a tight root-cause summary.) Coding
//      inline here is the mistake to avoid: it bloats the orchestrator's context and
//      can exhaust it before the milestone finishes. (The halted agent left a clean
//      tree, so the fixer starts from a known base.)
//   2. A naive resume REPLAYS THE CACHED HALT. resumeFromRunId returns each prior
//      agent() call's cached result for an unchanged (prompt, opts) — and the
//      halted executor's cached result *is* the halt, so it re-halts immediately
//      at the same spot. You must make that ONE call a cache miss so it re-runs
//      live against the now-fixed tree.
//   3. Do it surgically: in the run's SCRIPT SNAPSHOT (the path the Workflow tool
//      printed at launch, under the session dir — NOT this canonical file), append
//      a short RESUME note to ONLY the halted call's prompt, via a condition keyed
//      on its increment+task, e.g.:
//        const resumeNote = (inc.n === 4 && task.id === 'T5')
//          ? '\n\nRESUME — <what was fixed on main, with commit sha>; <verified how>; '
//            + 'write the test for this proven path and commit; do NOT re-diagnose.'
//          : ''
//        await agent(execPrompt(inc, task, plan.tasks) + resumeNote, { ... })
//      The changed call (and everything after, which never ran) goes live; the
//      unchanged prefix (every prior increment/task) replays instantly from cache.
//   4. KEEP ANY EARLIER RESUME NOTE BYTE-IDENTICAL across re-invocations — a prior
//      halt's note must stay unchanged or that call cache-misses too and re-runs
//      (risking re-doing already-committed work). Add the new condition; never edit
//      the old one. Then (args per RULE 0):
//      Workflow({ scriptPath: <snapshot>, args: { milestone, base }, resumeFromRunId: <id> }).
//   5. The RESUME note must state what was fixed (with the commit sha), how it was
//      verified, and "do NOT re-diagnose" — so the re-run executor writes the test
//      for the now-working path instead of re-halting on the same diagnosis.
//   6. IF THE CACHE-REPLAY RESUME STILL DOES NOT FAST-FORWARD after RULE 0 is satisfied
//      (it re-runs from the top and the first increment's planner HALTS on already-built
//      work): RULE 0 — a dropped `args` — is the cause to rule out FIRST (it was the M13
//      culprit; the resume had been invoked without `args`, so the milestone-reader's
//      prompt changed and missed the cache at call #1). If args are correctly re-passed
//      and it STILL won't replay, fall back to the DETERMINISTIC, cache-independent path:
//      (a) if the halted increment is only PARTLY done, finish its remaining tasks on
//      `main` with direct `build-executor` subagents (Agent tool) + one `increment-
//      validator`, exactly as the harness would — bringing that increment to fully-
//      built + validated-clean; (b) then re-invoke a FRESH run (NO resumeFromRunId)
//      with `args: { milestone, base, skipThrough: <highest fully-done+validated
//      increment> }`. The harness skips the done prefix (no re-plan, so no spurious
//      already-built halt) and builds only the remainder; the audit still diffs from
//      `base` (whole milestone). This needs NO script surgery and does not depend on the
//      agent-call cache at all. Prefer steps 1–5 (cheaper) once RULE 0 is honored;
//      this skipThrough path is the reliable fallback if cache-replay still misbehaves.
//
//   7. THE RUN DIED WITH THE PROCESS (rate limit, crash, closed shell) — do these IN ORDER,
//      derived by hand twice on 2026-08-09 and written down so the third time is cheap:
//      (a) `git push origin main` FIRST, before diagnosing anything. A killed run leaves
//          committed work unpushed (the 2026-08-09 rate limit stranded 26 commits), and
//          diagnosis is worthless if the disk dies while you do it.
//      (b) `git status --porcelain` — a NON-empty tree is a killed agent's in-flight work.
//          Do NOT assume it is junk and do NOT assume it is finished: run the full gate
//          over it. Green + coherent => commit it (this recovered a complete fix on
//          2026-08-09); red or half-written => discard it and let its task re-run.
//      (c) Establish the last FULLY-BUILT AND VALIDATED increment — the two are different.
//          `git log --oneline <base>..HEAD` shows the `design(mNN): increment N task
//          decomposition` markers and the task commits; the run journal
//          (`<transcriptDir>/journal.jsonl`, one {type:'result'} per agent) shows which
//          increments a validator actually returned CLEAN for. An increment whose commits
//          all landed but whose validator never ran, or ran BEFORE its fixes landed, is
//          NOT validated — validate it with one `increment-validator` before counting it.
//      (d) Resume with `args: { milestone, base, skipThrough: <that number> }`.
//      Diagnosing a mass agent death: many agents failing inside a few SECONDS of each
//      other is an account-level usage limit, not a code fault — grep the transcript dir
//      for '"error":"rate_limit"' / apiErrorStatus 429. Nothing in the run is wrong; wait
//      for the window and resume. A genuine blocker fails ONE agent, repeatedly.
//
// Usage:  Workflow({ name: 'milestone-build', args: { milestone: 'M3', base: '<sha>' } })
//   milestone — the roadmap milestone id whose decomposition to build (e.g. 'M3').
//   base      — the commit immediately before this milestone's first increment,
//               used as the audit diff base. Optional; omit and the auditors find it.
//   skipThrough — OPTIONAL deterministic resume (note 6): the highest increment number
//               already built + independently validated CLEAN on `main`. The harness
//               skips increments 1..skipThrough and starts at skipThrough+1; the audit
//               still covers the whole milestone via `base`. Use a fresh run (no
//               resumeFromRunId). Default 0 (build everything).

export const meta = {
  name: 'milestone-build',
  description: 'Build a whole milestone from its roadmap decomposition: per increment plan -> execute (one agent/task) -> validate -> fix (bounded 3 rounds); then the milestone-completion audit. Args: { milestone, base, skipThrough? }.',
  phases: [
    { title: 'Read milestone' },
    { title: 'Build increments' },
    { title: 'Milestone audit' },
  ],
}

// args may arrive as the object { milestone, base } or as a bare milestone string
// (the /milestone-build slash command passes its positional arg as a string, which
// would otherwise drop the milestone — and silently take the auto-find base path).
// GUARD (M24/M25): args sometimes arrives JSON-STRINGIFIED — a string that is itself
// a serialized object `{"milestone":...,"base":...,"skipThrough":...}`. Without this
// parse, the `typeof args === 'string'` branch below would take the WHOLE JSON blob as
// the milestone and silently drop base/skipThrough — which broke a skipThrough resume
// in M24. So: if args is a string that parses to an object, use the parsed object.
let parsedArgs = args
if (typeof args === 'string') {
  const s = args.trim()
  if (s.startsWith('{') || s.startsWith('[')) {
    try {
      const p = JSON.parse(s)
      if (p && typeof p === 'object') parsedArgs = p
    } catch (_) { /* not JSON — keep the bare-milestone-string handling below */ }
  }
}
const a = typeof parsedArgs === 'string' ? { milestone: parsedArgs } : (parsedArgs || {})
const milestone = a.milestone ? String(a.milestone) : 'the next milestone'
const base = a.base ? String(a.base) : null
// skipThrough — the deterministic, cache-independent resume (see RESUMING note 6).
// The highest increment number ALREADY built AND independently validated CLEAN on
// `main`; the harness skips plan/execute/validate for increments 1..skipThrough and
// starts real work at skipThrough+1. The audit still diffs from `base` (whole
// milestone). Use a FRESH run (no resumeFromRunId) — this path does not rely on the
// agent-call cache at all, so it is immune to a cache-replay that won't fast-forward.
const skipThrough = a.skipThrough != null ? Number(a.skipThrough) : 0

// resumeLine — the EXACT correct resume invocation, surfaced IN every halt return so the
// operator sees it at the moment they need it (not buried in the RULE 0 header they won't
// re-read mid-run). The M30 resume failed precisely because `args` was dropped on the
// re-invoke — `milestone` then defaulted to 'the next milestone' and the run halted at the
// read phase. So this spells out: ALWAYS re-pass args (resumeFromRunId does NOT restore
// them), and gives the cache-independent skipThrough fallback. `id`/`baseRef` are
// interpolated so the line is copy-paste-ready.
//
// `transient` (M43 lesson, 2026-07-17): an agent that "returned no result" died on
// infrastructure (API 529/kill after retries), NOT on a design blocker — the tree is
// clean and there is nothing to fix. The old one-size message said "first resolve the
// blocker via a build-fixer", which for this class sends the operator hunting for a
// blocker that does not exist. The halt return is the surface that produces the
// "how do I resume?" state, so it carries the right recovery for its own cause
// (the surface-contract law-2 discipline, applied to this harness).
function resumeLine(id, baseRef, transient) {
  const argsObj = "{ milestone: '" + id + "'" + (baseRef ? ", base: '" + baseRef + "'" : '') + ' }'
  const prep = transient
    ? 'This halt is transient-infrastructure-shaped (an agent returned no result after retries — API overload/kill, not a design blocker): if `git status` is clean there is NOTHING to fix — resume immediately; the failed call cache-misses and re-runs live while completed work replays from cache. Then: '
    : 'First resolve the blocker on `main` via a build-fixer subagent and leave the tree CLEAN, then: '
  return (
    'TO RESUME — re-pass args ALWAYS (RULE 0: resumeFromRunId does NOT restore args; omit them and ' +
    "`milestone` resets to 'the next milestone' and the resume dies at the read phase). " +
    prep +
    'Workflow({ scriptPath: <the snapshot path printed at launch>, args: ' + argsObj + ', resumeFromRunId: <this run id> }). ' +
    'If cache-replay will not fast-forward, use the deterministic fallback — a FRESH run (no resumeFromRunId) with ' +
    'args: { milestone: ' + "'" + id + "'" + ', base, skipThrough: <highest fully-built+validated increment> }.'
  )
}

// ---- structured-output schemas ----
const INCREMENTS_SCHEMA = {
  type: 'object',
  required: ['milestone', 'increments'],
  properties: {
    milestone: { type: 'string', description: 'the canonical milestone id this decomposition belongs to, e.g. "M11" — resolved from the roadmap decomposition heading, even when the request said "the next milestone"' },
    increments: {
      type: 'array',
      items: {
        type: 'object',
        required: ['n', 'title', 'deliverable', 'scope', 'proves'],
        properties: {
          n: { type: 'number' },
          title: { type: 'string' },
          deliverable: { type: 'string' },
          scope: { type: 'array', items: { type: 'string' } },
          proves: { type: 'string' },
        },
      },
    },
    note: { type: 'string' },
  },
}
const HALT = {
  type: 'object',
  description: 'the halt report — fill every field so triage needs nothing from your transcript (which is never read back)',
  properties: {
    root_cause: { type: 'string', description: 'the precise fork/blocker: what is undecided and why it is not resolvable from the locked docs or the milestone settled decisions' },
    evidence: { type: 'string', description: 'concrete proof: failing tests/commands, file:line, or the specific design gap' },
    tree_state: { type: 'string', description: 'the working-tree + commit state you leave: which commits landed; confirm the tree is CLEAN (you reverted your uncommitted changes)' },
    recommendation: { type: 'string', description: 'the suggested resolution — e.g. a task to insert before this one, or the decision the human must make' },
  },
}
const PLAN_SCHEMA = {
  type: 'object',
  required: ['status', 'tasks'],
  properties: {
    status: { type: 'string', enum: ['planned', 'halted'] },
    halt: HALT,
    tasks: {
      type: 'array',
      items: {
        type: 'object',
        required: ['id', 'subject', 'done_criterion'],
        properties: {
          id: { type: 'string' },
          subject: { type: 'string' },
          done_criterion: { type: 'string' },
          files: { type: 'array', items: { type: 'string' } },
          design_refs: { type: 'string' },
          notes: { type: 'string' },
        },
      },
    },
  },
}
const EXEC_TASK_SCHEMA = {
  type: 'object',
  required: ['status', 'notes'],
  properties: {
    status: { type: 'string', enum: ['completed', 'halted'] },
    halt: HALT,
    commit: { type: 'string', description: 'the conventional commit subject you committed (empty if halted)' },
    notes: { type: 'string', description: '1-2 lines: what shipped + any DECISIONS pin id logged. No essay.' },
  },
}
const VALIDATION_SCHEMA = {
  type: 'object',
  required: ['gate_green', 'gate_evidence', 'deliverable_holds', 'blocking', 'advisory', 'verdict'],
  properties: {
    gate_green: { type: 'boolean' },
    gate_evidence: { type: 'string', description: 'PROOF the FULL gate was run, not a claim: paste the verbatim `test result: ok. N passed; 0 failed` summary line for EVERY test binary from one UNSCOPED `cargo test` run — the WHOLE suite, no -p, no name filter, no single test — plus the fmt/clippy/build exit confirmations. gate_green=true is INVALID without this evidence; a scoped run does not count — the pack/describe goldens live in the `--bin jigc` target a scoped run misses.' },
    deliverable_holds: { type: 'boolean' },
    blocking: { type: 'array', items: { type: 'object', required: ['title', 'evidence'], properties: { title: { type: 'string' }, evidence: { type: 'string', description: 'file:line + the exact command and its observed output that proves the finding' }, fix_hint: { type: 'string' } } } },
    advisory: { type: 'array', items: { type: 'object', required: ['title', 'evidence'], properties: { title: { type: 'string' }, evidence: { type: 'string', description: 'file:line + the proof' } } } },
    verdict: { type: 'string', description: '1-2 sentences: does the increment genuinely hold, and the decisive reason' },
  },
}
const FIX_SCHEMA = {
  type: 'object',
  required: ['status', 'notes'],
  properties: { status: { type: 'string', enum: ['fixed', 'could-not-fix'] }, commit: { type: 'string', description: 'the fix commit subject (empty if could-not-fix)' }, notes: { type: 'string', description: '1-2 lines; on could-not-fix, the trace showing the finding is not real' } },
}
const REVIEW_SCHEMA = {
  type: 'object',
  required: ['findings', 'summary'],
  properties: {
    findings: { type: 'array', items: { type: 'object', required: ['severity', 'title', 'location', 'evidence'], properties: { severity: { type: 'string', enum: ['critical', 'high', 'medium', 'low'] }, title: { type: 'string' }, location: { type: 'string', description: 'file:line' }, evidence: { type: 'string', description: 'the trace proving it is real — verify before reporting (the audit is a hypothesis generator, not an oracle)' }, confidence: { type: 'string' } } } },
    summary: { type: 'string', description: '2-4 sentences: the verdict on whether the milestone deliverable holds + the headline concern, if any' },
  },
}
const E2E_SCHEMA = {
  type: 'object',
  required: ['scenarios', 'overall_pass', 'summary'],
  properties: {
    scenarios: { type: 'array', items: { type: 'object', required: ['name', 'passed', 'detail'], properties: { name: { type: 'string' }, passed: { type: 'boolean' }, detail: { type: 'string', description: 'the exact repro command(s) + observed output; required on failure' } } } },
    overall_pass: { type: 'boolean' },
    summary: { type: 'string', description: '2-4 sentences: overall pass/fail + any failure headline' },
  },
}

// ---- thin prompts (the role + project knowledge live in the agent defs) ----
function header(inc) {
  const bullets = (inc.scope || []).map((s) => '  - ' + s).join('\n')
  return ['Increment ' + inc.n + ': ' + inc.title, 'Deliverable: ' + inc.deliverable, 'Grouped scope:', bullets, 'Proves: ' + inc.proves].join('\n')
}
function planPrompt(inc) {
  return [milestone + ' — ' + header(inc), '', 'Read the milestone settled decisions in DECISIONS.md + the roadmap and the design sections the scope names, ground the cut in the current code, then produce the ordered single-concern task cut and record it to DECISIONS.md per your role.'].join('\n')
}
function execPrompt(inc, task, all) {
  const others = all.map((t) => '  - ' + t.id + ': ' + t.subject).join('\n')
  return [
    milestone + ' — ' + header(inc),
    '',
    'Other tasks in this increment (context only — do NOT do them):',
    others,
    '',
    'YOUR TASK — ' + task.id + ': ' + task.subject,
    'Done-criterion: ' + task.done_criterion,
    'Files: ' + (task.files && task.files.length ? task.files.join(', ') : '(discover from the codebase)'),
    'Design: ' + (task.design_refs || '(see the scope above)'),
    task.notes ? 'Planner notes: ' + task.notes : '',
    '',
    'Run the full dev-workflow for THIS ONE task per your role; earlier tasks of this increment are already committed.',
  ].join('\n')
}
function validatePrompt(inc) {
  return [milestone + ' — ' + header(inc), '', 'Validate this increment against that roadmap spec per your validator role; exercise every grouped-scope bullet through the real binary or tests.', '', 'GATE IS A FACT, NOT A CLAIM: run the FULL gate yourself, UNSCOPED — `cargo test` over the WHOLE suite (no -p, no name filter, no single test — the pack/describe goldens live in the `--bin jigc` target a scoped run silently misses), plus `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo build`. Paste the verbatim `test result:` summary line for EVERY binary into `gate_evidence`. gate_green=true is INVALID without that pasted evidence; any non-zero exit / any `FAILED` is a BLOCKING finding. Do not trust the executor\'s claim — re-run it.'].join('\n')
}
function fixPrompt(inc, f) {
  return [
    'Fix ONE blocking finding from ' + milestone + ' increment ' + inc.n + ':',
    'TITLE: ' + f.title,
    'EVIDENCE: ' + f.evidence,
    f.fix_hint ? 'HINT: ' + f.fix_hint : '',
    '',
    'Resolve it via your fixer role (red reproduces the defect -> minimal green -> full gate -> one commit).',
  ].join('\n')
}
function reviewPrompt(baseRef) {
  return ['Milestone: ' + milestone + '. Review everything after the base commit: ' + baseRef + '.', '', 'Review the whole milestone diff per your code-reviewer role.'].join('\n')
}
function e2ePrompt() {
  return ['Milestone: ' + milestone + '.', '', 'Drive the milestone acceptance flows end-to-end through the real binary in throwaway repos per your e2e role.'].join('\n')
}

// agentR — run an agent() call, retrying on a TRANSIENT failure (API overload /
// a subagent that finished without emitting its StructuredOutput — usually a momentary
// 529 inside the agent loop). A single transient blip should not forfeit a multi-hour
// run (M10 lost a ~69-min run to one 529 at a planner's first API call). This AUTOMATES
// the documented INTERRUPTION recovery above: each retry tells the agent to restore a
// clean committed base first (discard the failed attempt's uncommitted partial work),
// then re-do its task from that base — exactly what a manual revert-then-resume does, so
// it is no riskier than the resume mechanism the harness already relies on. The "if your
// commit already landed, don't duplicate it" clause covers the rare commit-then-fail edge.
// Read-only agents (reader/validator/code-reviewer/e2e) treat the reset as a harmless no-op.
// Domain-agnostic: no milestone/project specifics — keep it that way (this harness is the
// self-hosting distill target).
//
// TWO failure SHAPES, both transient, both must retry (M30 root cause — get this right):
//   1. a THROW — agent() rejects (an error escaped the agent loop). Caught below.
//   2. a NULL RETURN — agent() RETURNS `null` when the subagent dies on a terminal API
//      error AFTER its own internal retries (e.g. a sustained 529), or the user skips it.
//      This does NOT throw — so the original `try/return` let it sail straight through
//      WITHOUT retrying, and the caller's `if (!r)` then halted the whole run on what was
//      just a momentary overload (M30 lost an inc-4 executor to exactly this — a 529 null
//      return that bypassed the 2 retries entirely). So a null return is retried here too.
// ON EXHAUSTION we RETURN null (not throw): every caller already treats a falsy result as
// a clean, structured halt ("returned no result") with prior committed work standing — far
// better than throwing, which crashes the run and forfeits every committed increment. The
// harness surfaces the underlying error separately in its failures channel, so visibility
// is not lost. (Bonus: on a later RESUME, attempt 0 replays the cached null but the live
// retry below then re-runs it — so a transient-failed call self-heals on resume.)
// RATE-LIMIT CIRCUIT BREAKER. A transient 529 is one agent stumbling; an account-level
// usage limit (429) kills EVERY agent at once — on 2026-08-09 twelve died inside a
// six-second window. The retry logic above is exactly wrong for that case: it answers
// each death by spawning MORE agents into the same wall, so one limit burns three
// attempts per in-flight call and the run thrashes instead of stopping.
//
// Detection is structural, not temporal — the script has no clock (`Date.now()` throws;
// it would break resume). The discriminator is WHICH calls fail: a genuine blocker fails
// ONE agent repeatedly (same label, retried in place), while a usage limit fails whatever
// is spawned next, whoever it is. So: once any call has exhausted its retries, the next
// exhaustion of a DIFFERENT call trips the breaker, and from then on `agentR` returns null
// WITHOUT SPAWNING. The run unwinds through the halt paths it already has, prior committed
// work standing, instead of spending the rest of the limit window failing.
//
// Deliberately NOT done here: waiting out the window, or lowering concurrency. A limit is
// the human's to wait out, and the recovery is cheap — resume with `skipThrough`. Losing
// one run is survivable; thrashing through a limit window is just waste.
const TRANSIENT_RETRIES = 2
let exhaustedLabels = []
let breakerTripped = false
async function agentR(prompt, opts) {
  let lastErr
  const lbl = (opts && opts.label) ? opts.label : 'agent'
  if (breakerTripped) {
    log('rate-limit breaker is tripped — NOT spawning ' + lbl + '; unwinding the run')
    return null
  }
  for (let attempt = 0; attempt <= TRANSIENT_RETRIES; attempt++) {
    // attempt 0 uses the prompt verbatim, so its (prompt, opts) stays cache-key-identical
    // on resume; only live retries carry the reset note (and are inherently uncached).
    const note = attempt === 0 ? '' : (
      '\n\nRETRY after a transient failure (API overload / no StructuredOutput) of a prior attempt. ' +
      'FIRST restore a clean committed base — discard any uncommitted partial work the failed attempt left ' +
      '(`git reset --hard HEAD` then `git clean -fd`) — then proceed from that clean base as if starting fresh. ' +
      'If your task\'s commit ALREADY exists at HEAD (the failure struck after committing), do NOT duplicate it: ' +
      're-derive and report your structured result from the existing commit.'
    )
    try {
      const result = await agent(prompt + note, opts)
      if (result != null) return result
      // null return = the subagent died on a terminal error after its own retries, or was
      // skipped. Same transient class as a throw — fall through to retry rather than return it.
      lastErr = new Error('agent returned null (subagent died on a terminal error, or was skipped)')
      log('null return on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + (TRANSIENT_RETRIES + 1) + ') — retrying')
    } catch (e) {
      lastErr = e
      log('transient failure on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + (TRANSIENT_RETRIES + 1) + ') — ' + ((e && e.message) || e))
    }
  }
  // Retries exhausted. Return null so the caller's existing halt path fires with a clean,
  // structured reason and prior committed work stands — instead of throwing (which would
  // crash the run and forfeit every committed increment).
  log('exhausted ' + (TRANSIENT_RETRIES + 1) + ' attempts on ' + lbl + ' — halting cleanly (' + ((lastErr && lastErr.message) || lastErr) + ')')
  // Trip the breaker on the SECOND distinct call to exhaust: one label failing is a
  // blocker, two different ones failing back-to-back is the environment.
  if (!exhaustedLabels.includes(lbl)) exhaustedLabels.push(lbl)
  if (exhaustedLabels.length >= 2 && !breakerTripped) {
    breakerTripped = true
    log('TWO different agents exhausted their retries (' + exhaustedLabels.join(', ') + ') — '
      + 'this is the shape of an account-level rate limit, not a code fault. Tripping the breaker: '
      + 'no further agents will be spawned. Confirm by grepping the run transcript dir for '
      + '\'"error":"rate_limit"\' / apiErrorStatus 429; if so, wait for the window and resume '
      + 'with args: { milestone, base, skipThrough: <highest built+validated increment> }.')
  }
  return null
}

// ---- read the milestone's increments from the roadmap ----
phase('Read milestone')
log('Reading ' + milestone + ' increment decomposition from implementation/roadmap.md')
const read = await agentR('Enumerate the ordered increments of milestone "' + milestone + '" from implementation/roadmap.md. If the request is "the next milestone", resolve it to the next milestone whose decomposition is present but status is planned-not-built, and return its canonical id (e.g. "M11") in the `milestone` field.', { label: 'read:' + milestone, phase: 'Read milestone', agentType: 'milestone-reader', schema: INCREMENTS_SCHEMA })
const increments = read && read.increments ? read.increments : []
// The id the reader actually resolved — so the result names what was built even
// when the caller passed no args and the workflow auto-selected the next milestone.
const builtMilestone = read && read.milestone ? String(read.milestone) : milestone
if (increments.length === 0) {
  // A read-phase halt is the classic DROPPED-ARGS symptom on a resume: if `milestone` is
  // 'the next milestone', the caller almost certainly omitted args on the re-invoke (RULE 0).
  const droppedArgs = milestone === 'the next milestone'
  return {
    status: 'halted',
    halted: {
      phase: 'read',
      reason: 'no roadmap decomposition for ' + milestone + ' — '
        + (droppedArgs
            ? 'this is almost certainly a DROPPED-ARGS resume (RULE 0): re-invoke with args: { milestone: "<id>", base } explicitly.'
            : 'run the milestone-planning workflow first.'),
    },
    resume: resumeLine(milestone, base),
    note: read ? read.note : null,
  }
}
log(builtMilestone + ' has ' + increments.length + ' increment(s): ' + increments.map((i) => 'I' + i.n).join(', '))

// ---- per increment: plan -> execute(per task) -> validate -> fix(bounded 3) ----
let halted = null
const incrementReports = []
for (const inc of increments) {
  phase('Build increments')

  // Deterministic resume: skip increments already built + validated CLEAN on `main`
  // (see RESUMING note 6). Cache-independent — the skipped increments never re-plan,
  // so the planner can't halt on already-built work.
  if (skipThrough && inc.n <= skipThrough) {
    log('Increment ' + inc.n + ' — already built + validated on main; skipping (skipThrough=' + skipThrough + ').')
    continue
  }

  log('Increment ' + inc.n + ' — planning (' + inc.title + ')')
  const plan = await agentR(planPrompt(inc), { label: 'plan:inc' + inc.n, phase: 'Build increments', agentType: 'build-planner', schema: PLAN_SCHEMA })
  if (!plan || plan.status === 'halted' || !plan.tasks || plan.tasks.length === 0) {
    halted = { increment: inc.n, phase: 'plan', halt: plan && plan.halt ? plan.halt : { root_cause: plan ? 'planner produced no tasks' : 'planner returned no result', tree_state: 'clean (planner writes only the DECISIONS entry, nothing on halt)' } }
    incrementReports.push({ increment: inc.n, plan })
    break
  }
  log('Increment ' + inc.n + ' — plan: ' + plan.tasks.length + ' task(s) [' + plan.tasks.map((t) => t.id).join(', ') + ']')

  const execResults = []
  for (const task of plan.tasks) {
    log('Increment ' + inc.n + ' — execute ' + task.id + ': ' + task.subject)
    const r = await agentR(execPrompt(inc, task, plan.tasks), { label: 'exec:inc' + inc.n + ':' + task.id, phase: 'Build increments', agentType: 'build-executor', schema: EXEC_TASK_SCHEMA })
    execResults.push({ task: task.id, result: r })
    if (!r || r.status === 'halted') {
      halted = { increment: inc.n, phase: 'execute', task: task.id, halt: r && r.halt ? r.halt : { root_cause: r ? 'executor halted without detail' : 'executor returned no result' } }
      break
    }
  }
  if (halted) { incrementReports.push({ increment: inc.n, plan, execResults }); break }

  let round = 0
  let lastValidation = null
  while (true) {
    log('Increment ' + inc.n + ' — independent validation (after ' + round + ' fix round(s))')
    const v = await agentR(validatePrompt(inc), { label: 'validate:inc' + inc.n + ':r' + round, phase: 'Build increments', agentType: 'increment-validator', schema: VALIDATION_SCHEMA })
    lastValidation = v
    const blocking = v && v.blocking ? v.blocking : []
    // gate_green must be BACKED by pasted evidence (a real `test result:` summary line),
    // not a bare boolean — an evidence-less green is treated as not-green (M23: inc-1
    // landed a red gate because the validator claimed green off a scoped/lacon-trimmed run).
    const gateProven = v && v.gate_green && typeof v.gate_evidence === 'string' && /test result:/.test(v.gate_evidence)
    if (blocking.length === 0 && gateProven) { log('Increment ' + inc.n + ' — validated CLEAN'); break }
    if (blocking.length === 0 && v && v.gate_green && !gateProven) { log('Increment ' + inc.n + ' — gate claimed green WITHOUT pasted `test result:` evidence; treating as unverified → fix round') }
    if (round >= 3) { halted = { increment: inc.n, phase: 'validate', reason: blocking.length > 0 ? blocking.length + ' blocking finding(s) remain after 3 fix rounds' : 'gate green could not be verified (no pasted `test result:` evidence from a full unscoped run) after 3 validation rounds — verify the gate by hand', blocking }; break }
    round++
    log('Increment ' + inc.n + ' — fix round ' + round + ': ' + blocking.length + ' blocking finding(s)')
    for (const f of blocking) {
      await agentR(fixPrompt(inc, f), { label: 'fix:inc' + inc.n + ':r' + round, phase: 'Build increments', agentType: 'build-fixer', schema: FIX_SCHEMA })
    }
  }

  incrementReports.push({ increment: inc.n, plan, execResults, validation: lastValidation, fixRounds: round })
  if (halted) break
}

if (halted) {
  // Transient-infrastructure halts (an agent returned no result after retries) get the
  // nothing-to-fix resume message; genuine blockers keep the resolve-first one.
  const transient = !!(halted.halt && /returned no result/.test(halted.halt.root_cause || ''))
  const msg = breakerTripped
    ? builtMilestone + ' build STOPPED by the rate-limit breaker: two different agents exhausted their retries (' + exhaustedLabels.join(', ') + '), so the run stopped spawning rather than thrash through the limit window. THIS IS NOT A CODE FAULT and nothing needs fixing — confirm by grepping the run transcript dir for \'"error":"rate_limit"\' / apiErrorStatus 429. Prior committed work stands (it may be UNPUSHED — push before diagnosing). Wait for the window, then resume with args: { milestone, base, skipThrough: <highest increment that is both built AND validated clean> } — note those are different: an increment whose validator ran before its fixes landed is not validated.'
    : transient
    ? builtMilestone + ' build HALTED on a transient infrastructure failure (an agent returned no result after retries). Prior committed work stands — if the tree is clean, resume immediately; nothing needs fixing.'
    : builtMilestone + ' build HALTED — human attention needed before continuing. Prior committed work stands.'
  return { status: 'halted', halted, message: msg, resume: resumeLine(builtMilestone, base, transient), milestone: builtMilestone, incrementReports }
}

// ---- milestone-completion audit (independent, adversarial, parallel) ----
phase('Milestone audit')
const baseRef = base || "the commit immediately before this milestone's first increment (find it via git log)"
log('All increments validated clean — running the milestone-completion audit (code review + e2e)')
const audit = await parallel([
  () => agentR(reviewPrompt(baseRef), { label: 'audit:code-review', phase: 'Milestone audit', agentType: 'milestone-code-reviewer', schema: REVIEW_SCHEMA }),
  () => agentR(e2ePrompt(), { label: 'audit:e2e', phase: 'Milestone audit', agentType: 'milestone-e2e-tester', schema: E2E_SCHEMA }),
])

return {
  status: 'built-and-audited',
  message: builtMilestone + ' fully built and independently validated clean. Milestone-completion audit complete. NOW: verify each finding is real (reproduce it), then AUTO-FIX every confirmed finding — delegate each to a `build-fixer` subagent (dev-workflow, one commit), the SAME autonomy the build phase has. Do NOT ask the human per finding and do NOT present a fix-vs-defer menu: leaving a confirmed finding unfixed degrades the milestone, so "defer / known-limitation" is NOT a default disposition. The human gate fires for EXACTLY two cases, and only after you have confirmed the finding: (a) too-big — the fix genuinely warrants its own increment (still scheduled, never dropped); (b) contested — the fix would revise a settled decision or change intended behavior. Size, not severity, decides the lane: a HIGH that is a bounded fix is still fix-now, and a pre-existing defect the milestone’s own flow exercises + a builder test masked is fix-now (not defer). CHEAP-VS-ROBUST FIRES AT TRIAGE TOO: a defer that leaves a KNOWN HOLE in the milestone declared/goal-complete surface (a capability reachable only through the engine/tests, not the shipped verb/CLI — the deliverable-reachable lens), or trips a ONE-WAY-DOOR tell, is NOT a valid defer even when large — it means the declared deliverable is hollow and the milestone IS NOT DONE, so the robust fix is fix-now (or, if genuinely huge, the milestone is BLOCKED, never quietly shipped hollow). Two rationalizations are BARRED: "no live case to test" is NOT "not needed" (a reconstructed/synthetic case proves a now-needed capability), and "premature generality" holds only if the trajectory does not commit. AND this fork is NEVER self-framed: before you surface any too-big->defer OR contested->document-it recommendation, spawn a `robust-advocate` subagent (Agent tool, agentType robust-advocate) to argue the vision-robust case at full strength, and present ITS case beside the cheap one — never your lone cheap recommendation (the M34 failure: the orchestrator self-framed the fork and led the human to defer the milestone declared deliverable). Everything else is a tested commit the human reviews AFTER. See milestone-completion-workflow.md → Plan (triage); methodology-docs.md → the independent robust-case advocate.',
  milestone: builtMilestone,
  base: base || '(omitted — the auditors auto-discovered it from git log)',
  incrementReports,
  audit: { code_review: audit[0], e2e: audit[1] },
}
