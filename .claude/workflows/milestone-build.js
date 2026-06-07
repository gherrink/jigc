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
const a = typeof args === 'string' ? { milestone: args } : (args || {})
const milestone = a.milestone ? String(a.milestone) : 'the next milestone'
const base = a.base ? String(a.base) : null
// skipThrough — the deterministic, cache-independent resume (see RESUMING note 6).
// The highest increment number ALREADY built AND independently validated CLEAN on
// `main`; the harness skips plan/execute/validate for increments 1..skipThrough and
// starts real work at skipThrough+1. The audit still diffs from `base` (whole
// milestone). Use a FRESH run (no resumeFromRunId) — this path does not rely on the
// agent-call cache at all, so it is immune to a cache-replay that won't fast-forward.
const skipThrough = a.skipThrough != null ? Number(a.skipThrough) : 0

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
  required: ['gate_green', 'deliverable_holds', 'blocking', 'advisory', 'verdict'],
  properties: {
    gate_green: { type: 'boolean' },
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
  return [milestone + ' — ' + header(inc), '', 'Validate this increment against that roadmap spec per your validator role; exercise every grouped-scope bullet through the real binary or tests.'].join('\n')
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
const TRANSIENT_RETRIES = 2
async function agentR(prompt, opts) {
  let lastErr
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
      return await agent(prompt + note, opts)
    } catch (e) {
      lastErr = e
      log('transient failure on ' + (opts && opts.label ? opts.label : 'agent') + ' (attempt ' + (attempt + 1) + '/' + (TRANSIENT_RETRIES + 1) + ') — ' + ((e && e.message) || e))
    }
  }
  throw lastErr
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
  return { status: 'halted', halted: { phase: 'read', reason: 'no roadmap decomposition for ' + milestone + ' — run the milestone-planning workflow first.' }, note: read ? read.note : null }
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
    if (blocking.length === 0 && v && v.gate_green) { log('Increment ' + inc.n + ' — validated CLEAN'); break }
    if (round >= 3) { halted = { increment: inc.n, phase: 'validate', reason: blocking.length + ' blocking finding(s) remain after 3 fix rounds', blocking }; break }
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
  return { status: 'halted', halted, message: builtMilestone + ' build HALTED — human attention needed before continuing. Prior committed work stands.', milestone: builtMilestone, incrementReports }
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
  message: builtMilestone + ' fully built and independently validated clean. Milestone-completion audit complete — fix-now is the default (the dev-workflow gate per finding); the human gate fires only for too-big (→ its own increment) or contested (→ would revise a settled decision) findings.',
  milestone: builtMilestone,
  base: base || '(omitted — the auditors auto-discovered it from git log)',
  incrementReports,
  audit: { code_review: audit[0], e2e: audit[1] },
}
