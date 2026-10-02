// milestone-build — the reusable build harness for one whole milestone.
//
// Captures the increment-workflow + milestone-completion loops as fixed
// orchestration (the by-hand process, promoted from interpreted docs). Each
// phase is its own agent role, defined under .claude/agents/:
//   build-planner · build-executor · increment-validator · build-fixer ·
//   milestone-code-reviewer · milestone-e2e-tester · milestone-reader · build-git.
//
// BRANCHES (the branching switch, 2026-10-01 — CLAUDE.md -> Branches is the model;
// implementation/increment-workflow.md -> Branches carries the lifecycle). The milestone
// is built on `milestone/<slug>/main`, forked from `main`; each increment on its own
// `milestone/<slug>/<increment-slug>`, forked from the milestone branch. Every git act is
// a small `build-git` agent step with a schema — this script has no shell — and no build
// agent creates, switches, merges or pushes a branch:
//   milestone branch  ensure `milestone/<slug>/main` (reuse a local or pushed one, else
//                     create it from `origin/main`), check it out; its fork point from
//                     `origin/main` is the audit base. Runs BEFORE the reader, because the
//                     decomposition the reader enumerates lives on that branch.
//   per increment     open `milestone/<slug>/<increment-slug>` from the milestone branch ->
//                     plan -> execute (one agent per task, full dev-workflow, one commit,
//                     all on the increment branch) -> validate (independent, read-only) ->
//                     fix (one agent per blocking finding, bounded 3 rounds) -> LAND:
//                     merge it back `--no-ff`, delete it locally, then PUSH the milestone
//                     branch (one CI run per increment; increment branches stay local).
//   planned halt      an increment whose roadmap entry ends in a human halt (`Ends in H2
//                     (the human's)`) is a STAGE BOUNDARY: once it has landed and been
//                     pushed, the run RETURNS `{status:'planned-halt'}` with the human's
//                     checklist. Workflows take no mid-run input, so the halt is a return,
//                     never an agent waiting. Resume per PLANNED HALTS below.
//   close             the milestone-completion audit (code review + e2e, parallel) over
//                     `<fork point>..milestone/<slug>/main`; its findings come back for
//                     triage, the fixes land on the milestone branch, and then, in order:
//                     the SYNC step merges `origin/main` into the milestone branch (a merge
//                     commit, never a rebase; a conflict confined to the append-only logs is
//                     resolved keeping both sides by `dev/merge-logs`, any other conflict
//                     halts to the human) ->
//                     `dev/gate` green on the merged tree -> push by name -> the ORCHESTRATOR
//                     opens the PR `milestone/<slug>/main -> main`. The sync runs AFTER triage,
//                     which happens outside this script, so the script does not run it: it
//                     RETURNS the step (`close.sync` — a build-git prompt with its exact
//                     command list, and its schema) for the orchestrator to spawn verbatim
//                     (implementation/dev-workflow.md -> Before a pull request to main). The
//                     human merges the PR; agents never do (release.md -> What agents may not
//                     do).
// The increment slug is derived here, deterministically, from the roadmap increment
// title (`incrementSlug`, below); the ORDER of increments comes from the roadmap, never
// from the name.
//
// Halts (a new fork at plan, a blocked task at execute, still-blocking after 3 rounds at
// validate, a git step that refuses) stop the run and surface a structured reason —
// prior committed work stands.
//
// RUNS STARTED BEFORE THE BRANCHING SWITCH ARE NOT RESUMABLE UNDER THIS SCRIPT. Every
// build prompt now names its branch and the reader returns planned halts, so every
// agent call's cache key changed; and a pre-switch run built on `main`, which agents may
// no longer push. Finish such a run on its own script snapshot, or start a fresh run of
// this one with `skipThrough`.
//
// THE MILESTONE-BRANCH STEP'S PROMPT CHANGED ON 2026-10-02 (a new branch starts from the
// current remote main; a diverged local main halts). Its cache key changed with it, so a run
// started before that change misses at its first call: resume such a run with the fresh-run
// `skipThrough` fallback (note 6), not resumeFromRunId. The close's sync step, added the same
// day, is returned rather than run and changed no other call's prompt.
//
// RESUMING — RULE 0 (the M13 root cause, get this right or nothing replays): ALWAYS
//   re-pass the SAME `args` ({ milestone, slug, model? }) on EVERY resume invocation. The
//   agent-call cache key is a CONTENT HASH that includes each call's prompt, and the
//   very first agents (the milestone-branch step and the milestone-reader) embed the slug
//   and the milestone id in their prompts. Omit `args` and `slug` is missing → the run is
//   REFUSED before any agent; omit only `milestone` and it defaults to 'the next
//   milestone' → the reader's prompt changes → its hash misses the cache → the ENTIRE
//   prefix re-runs live (and the first increment's planner then halts on already-built
//   work). On M13 this looked like "resume won't fast-forward"; it was actually a
//   dropped-args cache miss. So every resume below is `Workflow({ scriptPath, args: {
//   milestone, slug, cleared? }, resumeFromRunId })` — args ALWAYS present
//   (resumeFromRunId does NOT restore args). `cleared` is the one arg that GROWS across
//   resumes (PLANNED HALTS); no prompt embeds it, so growing it misses no cache entry.
//   `base` is no longer an arg: the audit base is the milestone branch's fork point, and
//   a passed `base` is ignored with a log line.
//
// PLANNED HALTS — the DEFAULT resume is resumeFromRunId. After the human has done the
//   checklist, re-invoke `Workflow({ scriptPath: <the snapshot>, args: { milestone, slug,
//   cleared: [<every halt cleared so far>, '<this halt id>'] }, resumeFromRunId: <this
//   run id> })`. Every agent call before the boundary replays from cache, the boundary is
//   passed because its id is in `cleared`, and the next increment's branch step, planner
//   and builders run live — its planner opens with the entry-gate task that verifies the
//   human's act by API (roadmap convention). The FALLBACK, cache-independent: a fresh run
//   (no resumeFromRunId) with `skipThrough: <the increment the halt ended>` and the same
//   `cleared`. Each halt return prints both lines, ready to paste.
//
// RESUMING — FIRST distinguish a HALT from an INTERRUPTION (they resume differently):
//   - INTERRUPTION (the run was killed mid-flight — process died, session dropped):
//     the in-flight agent() call never returned, so it is NOT cached — it is a natural
//     cache-miss that re-runs live on a plain resume. But a killed agent may have left a
//     DIRTY TREE — and a dirty tree has THREE states, not two. NEVER resume onto one, and
//     never assume it is junk: a killed executor can die between GREEN and COMMIT (M48 lost
//     511 insertions across four files that compiled and whose acceptance suite passed).
//     (PRECEDENCE — this rule is stated in three places and they must not drift: canonical is
//     implementation/increment-workflow.md -> Halt and resume; note 7(b) below restates it;
//     this is its short form. If they ever disagree, increment-workflow.md wins.)
//     M52 (2026-09-20) datum: a killed run can stay REGISTERED as running — TaskStop reports
//     it killed but its loop never exits, and every resume is refused ("would run two copies
//     against the same journal"). Do not wait it out: finish the interrupted increment outside
//     the harness (executors → validator → fixers, one at a time, on its increment branch,
//     then land it by hand per note 6(a)) and start a FRESH run with skipThrough (rule 6
//     below). increment-workflow.md → Halt and resume carries the same note.
//     So CLASSIFY before acting: (a) does it build, (b) does the halted task's own
//     done-criterion pass, (c) does the FULL unscoped gate pass (`dev/gate`). All three =>
//     the work is FINISHED and only the commit is owed — COMMIT IT on the increment branch,
//     and say in the message that it was recovered rather than authored. Any of the three
//     failing (or an obvious half-write, e.g. a planner's uncommitted DECISIONS entry) =>
//     revert it and let the agent re-do it from a clean base. Then plain-resume
//     `Workflow({scriptPath: <snapshot>, args: { milestone, slug }, resumeFromRunId})`
//     (args per RULE 0) — NO script surgery (the killed call cache-misses on its own;
//     the committed prefix replays from cache). Leave the increment branch checked out:
//     every build prompt names its branch and refuses to commit anywhere else.
//   - HALT (the run returned `{status:'halted'}` cleanly, tree CLEAN): the halted call
//     COMPLETED and its halt-result IS cached — a plain resume replays the cached halt
//     and re-halts. This case needs the script-snapshot surgery in steps 1–5 below.
//   (Tell them apart: a halt left a clean tree + a returned halt report; an interruption
//   left no return value and often a dirty tree. M8 hit both — an interruption mid-inc-5
//   planner, then later a genuine halt at inc-5 Plan.)
//   A PLANNED halt (`{status:'planned-halt'}`) is neither: nothing is wrong and nothing is
//   cached as a halt — see PLANNED HALTS above.
//
// RESUMING AFTER A HALT (read this before re-invoking — there is a sharp edge):
//   1. Resolve the blocker ON THE HALTED INCREMENT'S BRANCH (`milestone/<slug>/<increment-
//      slug>`, still checked out — the halt return names it), outside the workflow, but
//      DELEGATE — the orchestrator decides, it does not code. The orchestrator (with the
//      human) owns only the JUDGMENT: diagnose the fork, pick the approach. Executing it
//      — edit, full gate green, COMMIT on that branch — goes to a `build-fixer` SUBAGENT
//      (Agent tool, agentType 'build-fixer'), a sibling of the workflow; for a genuine
//      fork, surface its options to the human first, then have it apply the choice.
//      The fixer's report carries the commit sha + gate/emitted-command evidence,
//      so the orchestrator resumes from the report WITHOUT re-reading code — keeping
//      the main-session window holding summaries + shas, not file bodies. (Even the
//      diagnosis can be a subagent that returns a tight root-cause summary.) Coding
//      inline here is the mistake to avoid: it bloats the orchestrator's context and
//      can exhaust it before the milestone finishes. (The halted agent left a clean
//      tree, so the fixer starts from a known base.) A halt in a GIT step (phase
//      'branch', 'land' or 'push') has no code blocker: its report says what git refused —
//      a moved milestone branch, a rejected push — and the human reconciles the branches.
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
//          ? '\n\nRESUME — <what was fixed on the increment branch, with commit sha>; '
//            + '<verified how>; write the test for this proven path and commit; do NOT re-diagnose.'
//          : ''
//        await agent(execPrompt(inc, task, plan.tasks) + resumeNote, { ... })
//      The changed call (and everything after, which never ran) goes live; the
//      unchanged prefix (every prior increment/task) replays instantly from cache.
//   4. KEEP ANY EARLIER RESUME NOTE BYTE-IDENTICAL across re-invocations — a prior
//      halt's note must stay unchanged or that call cache-misses too and re-runs
//      (risking re-doing already-committed work). Add the new condition; never edit
//      the old one. Then (args per RULE 0):
//      Workflow({ scriptPath: <snapshot>, args: { milestone, slug }, resumeFromRunId: <id> }).
//   5. The RESUME note must state what was fixed (with the commit sha), how it was
//      verified, and "do NOT re-diagnose" — so the re-run executor writes the test
//      for the now-working path instead of re-halting on the same diagnosis.
//   6. IF THE CACHE-REPLAY RESUME STILL DOES NOT FAST-FORWARD after RULE 0 is satisfied
//      (it re-runs from the top and the first increment's planner HALTS on already-built
//      work): RULE 0 — a dropped `args` — is the cause to rule out FIRST (it was the M13
//      culprit; the resume had been invoked without `args`, so the milestone-reader's
//      prompt changed and missed the cache at call #1). If args are correctly re-passed
//      and it STILL won't replay, fall back to the DETERMINISTIC, cache-independent path:
//      (a) if the halted increment is only PARTLY done, finish its remaining tasks on its
//      increment branch with direct `build-executor` subagents (Agent tool) + one
//      `increment-validator`, exactly as the harness would — bringing that increment to
//      fully-built + validated-clean — then LAND it by hand, the three git acts the harness
//      would run: `git switch milestone/<slug>/main && git merge --no-ff --no-edit
//      <increment branch> && git branch -d <increment branch> && git push origin
//      milestone/<slug>/main`; (b) then re-invoke a FRESH run (NO resumeFromRunId) with
//      `args: { milestone, slug, skipThrough: <highest increment built, validated AND
//      merged into the milestone branch>, cleared }`. The harness skips the done prefix
//      (no re-plan, so no spurious already-built halt) and builds only the remainder; the
//      audit still diffs from the fork point (whole milestone). This needs NO script
//      surgery and does not depend on the agent-call cache at all. Prefer steps 1–5
//      (cheaper) once RULE 0 is honored; this skipThrough path is the reliable fallback if
//      cache-replay still misbehaves.
//
//   7. THE RUN DIED WITH THE PROCESS (rate limit, crash, closed shell) — do these IN ORDER,
//      derived by hand twice on 2026-08-09 and written down so the third time is cheap:
//      (a) `git push origin milestone/<slug>/main` FIRST, before diagnosing anything — the
//          branch named in full (a bare `git push` is denied). Since the switch the harness
//          pushes after every landed increment, so this is usually a no-op; but the
//          in-flight increment's commits live on its LOCAL increment branch, unpushed by
//          design (increment branches stay local), and diagnosis is worthless if the disk
//          dies while you do it — say so to the human rather than assuming it is safe.
//      (b) `git status --porcelain` — a NON-empty tree is a killed agent's in-flight work.
//          Do NOT assume it is junk and do NOT assume it is finished: run the full gate
//          over it. Green + coherent => commit it (this recovered a complete fix on
//          2026-08-09); red or half-written => discard it and let its task re-run.
//      (c) Establish the last FULLY-BUILT, VALIDATED AND MERGED increment — three different
//          facts. `git log --oneline --first-parent <fork point>..milestone/<slug>/main`
//          shows one merge commit per landed increment (`Merge branch 'milestone/<slug>/
//          <increment-slug>'`); an unmerged increment branch (`git branch --list
//          'milestone/<slug>/*'`) is an increment that did not land. The run journal
//          (`<transcriptDir>/journal.jsonl`, one {type:'result'} per agent) shows which
//          increments a validator actually returned CLEAN for. An increment whose commits
//          all landed but whose validator never ran, or ran BEFORE its fixes landed, is
//          NOT validated — validate it with one `increment-validator` before counting it,
//          and land it per 6(a).
//      (d) Resume with `args: { milestone, slug, skipThrough: <that number>, cleared }`.
//      Diagnosing a mass agent death: many agents failing inside a few SECONDS of each
//      other is an account-level usage limit, not a code fault — grep the transcript dir
//      for '"error":"rate_limit"' / apiErrorStatus 429. Nothing in the run is wrong; wait
//      for the window and resume. A genuine blocker fails ONE agent, repeatedly.
//
// Usage:  Workflow({ name: 'milestone-build', args: { milestone: 'M55', slug: 'findings-channel' } })
//   milestone — the roadmap milestone id whose decomposition to build (e.g. 'M55').
//   slug      — REQUIRED: the milestone's branch slug (jigc's slug grammar, lowercase a-z0-9
//               words joined by single `-`), chosen at planning and recorded in the
//               roadmap decomposition (milestone-planning-workflow.md -> Decompose). The
//               milestone branch is `milestone/<slug>/main`. A run without it is REFUSED
//               before any agent is spawned.
//   skipThrough — OPTIONAL deterministic resume (note 6): the highest increment number
//               already built, independently validated CLEAN and merged into the milestone
//               branch. The harness skips increments 1..skipThrough and starts at
//               skipThrough+1; the audit still covers the whole milestone from the fork
//               point. Use a fresh run (no resumeFromRunId). Default 0 (build everything).
//   cleared   — OPTIONAL list of planned-halt ids the human has cleared (e.g. ['H1']); a
//               planned halt whose id is listed is passed instead of returned.
//   model     — OPTIONAL model class pinned onto every agent() call ('opus' by default).
//               Part of each call's cache key: re-pass the same value on every resume.

export const meta = {
  name: 'milestone-build',
  description: 'Build a whole milestone on milestone/<slug>/main from its roadmap decomposition: per increment, on milestone/<slug>/<increment-slug>, plan -> execute (one agent/task) -> validate -> fix (bounded 3 rounds) -> merge back --no-ff and push the milestone branch; return at each planned human halt; then the milestone-completion audit. Args: { milestone, slug, skipThrough?, cleared?, model? }.',
  phases: [
    { title: 'Milestone branch' },
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
// slug — the milestone's branch slug, REQUIRED (see Usage). jigc's slug grammar, the same
// one dev/branch-name and CI hold every branch to: runs of [a-z0-9] joined by single `-`.
const SLUG_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/
const slug = a.slug != null ? String(a.slug) : null
const milestoneBranch = slug ? 'milestone/' + slug + '/main' : null
// cleared — the planned-halt ids the human has cleared (PLANNED HALTS). Accepts a list or
// a comma-separated string. No prompt embeds it, so growing it misses no cache entry.
const cleared = (Array.isArray(a.cleared) ? a.cleared : a.cleared != null ? String(a.cleared).split(',') : [])
  .map((id) => String(id).trim()).filter(Boolean)
// skipThrough — the deterministic, cache-independent resume (see RESUMING note 6).
// The highest increment number ALREADY built, independently validated CLEAN AND merged
// into the milestone branch; the harness skips open/plan/execute/validate/land for
// increments 1..skipThrough (and their planned halts) and starts real work at
// skipThrough+1. The audit still diffs from the fork point (whole milestone). Use a FRESH run (no resumeFromRunId) — this path does not rely on the
// agent-call cache at all, so it is immune to a cache-replay that won't fast-forward.
const skipThrough = a.skipThrough != null ? Number(a.skipThrough) : 0
// model — the model class EVERY agent call in this run is pinned to. Default 'opus'.
// The `model:` frontmatter key in .claude/agents/*.md is NOT honored when the Workflow
// runtime resolves a role by `agentType` (measured 2026-09-14 on the first M51 launch: the
// milestone-reader ran on the session model with `model: opus` present in its definition),
// so the pin has to ride the agent() opts. It is part of the (prompt, opts) cache key, so a
// resume must re-pass the same value (RULE 0 applies to `model` exactly as to `milestone`).
const model = a.model ? String(a.model) : 'opus'

// The two refusals that run BEFORE any agent: no slug (a fresh run that forgot it, or a
// resume of a run started before the branching switch, whose args had none), or a slug
// outside the grammar (the branch would fail CI's branch-name check on its first push).
if (!slug || !SLUG_RE.test(slug)) {
  return {
    status: 'refused',
    message: (slug
      ? 'args.slug ' + JSON.stringify(slug) + ' is not a slug: it must be lowercase a-z0-9 words joined by single dashes (jigc\'s slug grammar), because the milestone branch is milestone/<slug>/main and CI refuses any other shape. '
      : 'args.slug is missing, and it is REQUIRED: the milestone is built on milestone/<slug>/main. Pass the slug the roadmap decomposition records for ' + milestone + ' (e.g. M55 -> findings-channel). ')
      + 'Nothing was run. A resume of a run started BEFORE the branching switch (2026-10-01) lands here too — those runs are not resumable under this script (see its header); finish one on its own snapshot, or start a fresh run of this script with slug and skipThrough.',
  }
}
// `base` is no longer read: the audit base is the milestone branch's fork point from
// origin/main, which the milestone-branch step reports. Said once, so a caller still
// passing it is not left believing it took effect.
if (a.base != null) log('args.base (' + String(a.base) + ') is ignored since the branching switch: the audit base is the fork point of ' + milestoneBranch + ' from origin/main.')

// ---- branch names ----
// incrementSlug — the increment's branch slug, derived from its roadmap TITLE alone, so
// the same decomposition always yields the same branches (the reader's result is cached,
// so a resume derives them again identically). It yields jigc's slug GRAMMAR (`is_slug`:
// the shape dev/branch-name and CI accept) by construction, and borrows the mint rule's
// legibility moves — a five-word cap, edge filler dropped — without claiming to be the
// engine's `slugify` (no transliteration table, no hyphen-glue provenance):
//   1. fold accents (NFKD, combining marks dropped) and case;
//   2. delete apostrophes and code ticks, which join a word (`run's` -> `runs`) rather
//      than split it;
//   3. split on every other non-[a-z0-9] run;
//   4. drop leading and trailing filler words, cap at five words, drop trailing filler
//      again (the cap can expose some) — so `the pre-public audit, and the before run's
//      instruments` is `pre-public-audit`, `the close` is `close`;
//   5. cap at 50 characters on a word boundary.
// An empty result is `increment-<n>`. Uniqueness within the milestone is the caller's
// (`incrementBranches`): `main` is reserved for the milestone branch itself.
const SLUG_FILLER = ['a', 'an', 'the', 'of', 'to', 'in', 'on', 'at', 'by', 'for', 'and', 'or']
const SLUG_MAX_WORDS = 5
const SLUG_MAX_CHARS = 50
function incrementSlug(title) {
  let words = String(title || '').normalize('NFKD').replace(/[\u0300-\u036f]/g, '').toLowerCase()
    .replace(/['\u2019`]/g, '')
    .split(/[^a-z0-9]+/).filter(Boolean)
  const trim = (w) => {
    while (w.length && SLUG_FILLER.includes(w[0])) w.shift()
    while (w.length && SLUG_FILLER.includes(w[w.length - 1])) w.pop()
    return w
  }
  words = trim(words).slice(0, SLUG_MAX_WORDS)
  words = trim(words)
  while (words.length > 1 && words.join('-').length > SLUG_MAX_CHARS) words.pop()
  let out = words.join('-')
  if (out.length > SLUG_MAX_CHARS) out = out.slice(0, SLUG_MAX_CHARS).replace(/-+$/, '')
  return out
}
// incrementBranches — n -> `milestone/<slug>/<increment-slug>`, unique within the
// milestone in ROADMAP ORDER: the first increment to claim a slug keeps it bare, each later
// one takes `-2`, `-3`, … (jigc's collision-suffix convention), and `main` is pre-claimed
// by the milestone branch.
function incrementBranches(incs) {
  const taken = new Set(['main'])
  const byN = {}
  for (const inc of incs) {
    const bare = incrementSlug(inc.title) || ('increment-' + inc.n)
    let candidate = bare
    for (let k = 2; taken.has(candidate); k++) candidate = bare + '-' + k
    taken.add(candidate)
    byN[inc.n] = 'milestone/' + slug + '/' + candidate
  }
  return byN
}

// resumeLine — the EXACT correct resume invocation, surfaced IN every halt return so the
// operator sees it at the moment they need it (not buried in the RULE 0 header they won't
// re-read mid-run). The M30 resume failed precisely because `args` was dropped on the
// re-invoke — `milestone` then defaulted to 'the next milestone' and the run halted at the
// read phase. So this spells out: ALWAYS re-pass args (resumeFromRunId does NOT restore
// them), and gives the cache-independent skipThrough fallback. `id`/`baseRef` are
// interpolated so the line is copy-paste-ready.
//
// `transient` (M43 lesson, 2026-07-17): an agent that "returned no result" died on
// infrastructure (API 529/kill after retries), NOT on a design blocker — so there is no
// blocker to RESOLVE. (It does not follow that there is nothing to DO: the tree still has
// the three states of the RESUMING block above.) The old one-size message said "first resolve the
// blocker via a build-fixer", which for this class sends the operator hunting for a
// blocker that does not exist. The halt return is the surface that produces the
// "how do I resume?" state, so it carries the right recovery for its own cause
// (the surface-contract law-2 discipline, applied to this harness).
//
// TWO printed surfaces, ONE mode, ONE tree rule. `haltMode` classifies a halt once;
// `haltMessage` and `resumeLine` each take that mode plus the halt's PHASE, and both defer
// the tree rule to the single `treeGuidance` below. Before this split, `msg` checked
// `breakerTripped` first while `resume` read the raw `transient` flag — so a transient halt
// during a TRIPPED breaker (they are simultaneously true whenever the breaker trips on a null
// return) put "wait for the window" and "resume immediately" in the same return object.
// Following the resume line inside a limit window is exactly the thrash the breaker exists
// to prevent, so the breaker's guidance WINS whenever it is tripped.
// (`breakerTripped`/`exhaustedLabels` are declared further down with the breaker itself;
// these functions only read them at call time, which is always after.)
function haltMode(transient) {
  return breakerTripped ? 'breaker' : transient === true ? 'transient' : 'blocker'
}

// treeGuidance — the ONE statement of the M48 three-state tree rule on the printed surfaces,
// so the message and the resume line cannot drift from each other. Canonical is
// implementation/increment-workflow.md -> Halt and resume; the RESUMING header block above
// restates it; this is the surface form. If they disagree, increment-workflow.md wins.
//
// PHASE-AWARE, because the generic three-state text is WRONG for a phase whose agent has no
// code deliverable. At `plan` the only thing that can be in the tree is the planner's
// uncommitted DECISIONS entry: `dev/gate` over a docs-only change is green BY CONSTRUCTION,
// and a half-written entry reads as satisfying "record the decomposition" — so all three
// printed conditions pass and the operator commits a half-write whose real product (the
// ordered task list) died with the agent anyway. The header block already carries the
// discriminating clause ("or an obvious half-write, e.g. a planner's uncommitted DECISIONS
// entry => revert it"); this file's own rationale is that the header is NOT re-read mid-run,
// so it is carried onto the surfaces that ARE read at halt time.
//
// It states what to do to the TREE and never when to resume — the caller owns the timing
// (the breaker's is "not until the window resets", everyone else's is "now").
function treeGuidance(haltPhase) {
  if (haltPhase === 'branch' || haltPhase === 'land' || haltPhase === 'push') {
    return 'The git steps edit no file and commit nothing but a merge, and each checks the tree is clean before it acts, so a dirty tree here is not theirs — explain it before anything else. If `git status` reports a merge in progress, the land step stopped inside one: `git merge --abort` returns the milestone branch to where it was. Never resume onto a dirty tree or a half-done merge. '
  }
  if (haltPhase === 'read') {
    return 'The milestone-reader is READ-ONLY, so nothing in the tree is its work: a dirty tree here predates this run and is yours to explain — the harness has nothing to commit or revert on its behalf. '
  }
  if (haltPhase === 'plan') {
    return 'CLASSIFY THE TREE before anything else: clean => nothing to do to it; dirty => it is the planner\'s uncommitted DECISIONS entry, and a finished planner commits that ITSELF, so uncommitted means it did not finish — that is a HALF-WRITE, so REVERT it and let the planner re-do it from a clean base. Do NOT run `dev/gate` over it and read green as done (a docs-only change is green by construction), and do NOT read a half-written entry as satisfying "record the decomposition": the planner\'s real product, the ordered task list, is a structured return that died with the agent. Never resume onto a dirty tree. '
  }
  if (haltPhase === 'validate') {
    return 'CLASSIFY THE TREE before anything else — and note the increment-validator is READ-ONLY, so anything dirty is a `build-fixer`\'s work, not the halted call\'s: clean => nothing to do to it; dirty AND the full `dev/gate` is green AND the finding that fixer was handed is genuinely fixed => it died between green and commit, so COMMIT that work (say it was recovered, not authored); dirty otherwise, or an obvious half-write => REVERT it and let it re-run from a clean base. Never resume onto a dirty tree. '
  }
  return 'CLASSIFY THE TREE before anything else, it has three states: clean => nothing to do to it; dirty AND the full `dev/gate` is green AND the halted call\'s own done-criterion holds => the agent died between green and commit, so COMMIT that work (say it was recovered, not authored); dirty otherwise — any of the three failing, or an obvious half-write (e.g. a planner\'s uncommitted DECISIONS entry) => REVERT it and let the call re-do it from a clean base. Never resume onto a dirty tree. '
}

// haltMessage — the ONE producer of a halt's headline message, for every halt construction
// (the read-phase return included, which used to carry no `message` key at all).
function haltMessage(id, mode, haltPhase) {
  if (mode === 'breaker') {
    return id + ' build STOPPED by the rate-limit breaker: two different agents exhausted their retries (' + exhaustedLabels.join(', ') + '), so the run stopped spawning rather than thrash through the limit window. THIS IS NOT A CODE FAULT and there is no blocker to resolve — confirm by grepping the run transcript dir for \'"error":"rate_limit"\' / apiErrorStatus 429. Prior committed work stands (the in-flight increment\'s commits are on its LOCAL increment branch, unpushed by design; `git push origin ' + milestoneBranch + '` before diagnosing if a landed increment was not pushed). ' + treeGuidance(haltPhase) + 'THEN WAIT for the window to reset before resuming — a resume inside it just re-spends it — and resume with args: { milestone, slug, skipThrough: <highest increment that is built, validated clean AND merged into ' + milestoneBranch + '>, cleared } — note those are different: an increment whose validator ran before its fixes landed is not validated, and one not merged did not land.'
  }
  if (mode === 'transient') {
    return id + ' build HALTED on a transient infrastructure failure (an agent returned no result after retries). Prior committed work stands and there is no blocker to resolve. ' + treeGuidance(haltPhase)
  }
  return id + ' build HALTED — human attention needed before continuing. Prior committed work stands.'
}

// argsLiteral — the run's args as a copy-paste-ready literal: the SAME milestone, slug and
// model (RULE 0), `cleared` as given (plus `extraCleared`, for a planned halt), and
// `skipThrough` only on the fallback line.
function argsLiteral(id, extraCleared, skip) {
  const ids = cleared.concat(extraCleared ? [extraCleared] : [])
  return "{ milestone: '" + id + "', slug: '" + slug + "'"
    + (a.model ? ", model: '" + model + "'" : '')
    + (skip != null ? ', skipThrough: ' + skip : '')
    + (ids.length ? ', cleared: [' + ids.map((c) => "'" + c + "'").join(', ') + ']' : '')
    + ' }'
}

function resumeLine(id, mode, haltPhase, branch) {
  const argsObj = argsLiteral(id, null, null)
  // A git step's refusal is a CACHED result like any halt, so a plain resume replays it and
  // refuses again; and there is no agent call to cache-bust, because the fix is the
  // human's, on the branches. So the deterministic path is the primary one here.
  if (mode === 'blocker' && (haltPhase === 'branch' || haltPhase === 'land' || haltPhase === 'push')) {
    return (
      'TO RESUME — there is no CODE blocker and a `build-fixer` is the wrong instrument: a git step refused (its halt report says what — a dirty tree, a moved or diverged branch, a rejected push), and reconciling the branches is the human\'s. A plain resumeFromRunId would REPLAY this cached refusal. So: reconcile, leave the tree CLEAN; if the halted increment is validated but not yet merged, land it by hand (header note 6(a)); push ' + milestoneBranch + ' by name; then start a FRESH run (no resumeFromRunId) with args: '
      + argsLiteral(id, null, '<highest increment built, validated AND merged into ' + milestoneBranch + '>') + ' — the same milestone, slug and cleared (RULE 0).'
    )
  }
  const prep = mode === 'breaker'
    ? 'The rate-limit breaker is tripped, so this is NOT a blocker and there is nothing to resolve — but do NOT resume yet: a resume inside the usage window just re-spends it. ' + treeGuidance(haltPhase) + 'Then, once the window has reset and the tree is clean: '
    : mode === 'transient'
    ? 'This halt is transient-infrastructure-shaped (an agent returned no result after retries — API overload/kill, not a design blocker), so there is no blocker to resolve. ' + treeGuidance(haltPhase) + 'The failed call cache-misses and re-runs live while completed work replays from cache. Then, with the tree clean: '
    : haltPhase === 'read'
    ? 'There is no CODE blocker here and a `build-fixer` is the wrong instrument: the roadmap on ' + milestoneBranch + ' simply carries no decomposition for this milestone, so run the MILESTONE-PLANNING workflow to produce one there (or re-invoke with the correct milestone id), leave the tree CLEAN, then: '
    : 'First resolve the blocker on ' + (branch ? '`' + branch + '`' : 'the halted increment\'s branch') + ' (still checked out) via a build-fixer subagent, committing there, and leave the tree CLEAN, then: '
  return (
    'TO RESUME — re-pass args ALWAYS (RULE 0: resumeFromRunId does NOT restore args; omit them and the run is ' +
    "REFUSED for want of a slug, and drop only `milestone` and it resets to 'the next milestone' and every cached call misses). " +
    prep +
    'Workflow({ scriptPath: <the snapshot path printed at launch>, args: ' + argsObj + ', resumeFromRunId: <this run id> }). ' +
    'If cache-replay will not fast-forward, use the deterministic fallback — a FRESH run (no resumeFromRunId) with ' +
    'args: ' + argsLiteral(id, null, '<highest increment built, validated AND merged into ' + milestoneBranch + '>') + '.'
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
          halt_after: {
            type: 'object',
            description: 'ONLY when the roadmap ends this increment in a planned human halt (a bullet like `**Ends in H2 (the human\'s).**`): the halt, so the harness can stop at it. Omit the field entirely for an increment with no such bullet.',
            required: ['id', 'checklist'],
            properties: {
              id: { type: 'string', description: 'the halt id exactly as the roadmap names it, e.g. "H2"' },
              checklist: { type: 'array', items: { type: 'string' }, description: 'the human\'s acts the bullet lists, one item per act, verbatim, in order — including any closing sentence about what the orchestrator does next' },
            },
          },
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
// The three git steps' reports. `status: 'halted'` carries the HALT report like any other
// role; the git steps halt rather than improvise whenever the branches are not in the shape
// the step expects.
const BRANCH_SCHEMA = {
  type: 'object',
  required: ['status', 'branch', 'head'],
  properties: {
    status: { type: 'string', enum: ['ready', 'halted'] },
    halt: HALT,
    branch: { type: 'string', description: 'the output of `git branch --show-current` after the step' },
    head: { type: 'string', description: 'the full sha of HEAD after the step' },
    created: { type: 'boolean', description: 'true when this step created the branch, false when it reused an existing one' },
    fork_point: { type: 'string', description: 'milestone-branch step only: the full sha `git merge-base origin/main HEAD` printed' },
  },
}
const LAND_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['merged', 'halted'] },
    halt: HALT,
    merge_commit: { type: 'string', description: 'the full sha of the --no-ff merge commit (empty if halted)' },
  },
}
const PUSH_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['pushed', 'halted'] },
    halt: HALT,
    remote_head: { type: 'string', description: 'the full sha `git ls-remote` reports for the pushed branch (empty if halted)' },
  },
}
// The close's sync step (syncMainPrompt) — returned to the orchestrator in `close.sync`, not
// run here, because it follows the triage that happens after this script returns.
const SYNC_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['merged', 'up-to-date', 'halted'] },
    halt: HALT,
    head: { type: 'string', description: 'the full sha `git rev-parse HEAD` prints after the step — the merge commit when status is merged' },
    origin_main: { type: 'string', description: 'the full sha `git rev-parse origin/main` prints after the fetch' },
    resolved_logs: { type: 'array', items: { type: 'string' }, description: 'the append-only logs whose conflict the step resolved by keeping both sides (empty when the merge was clean)' },
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
    gate_evidence: { type: 'string', description: 'PROOF the FULL gate was run, not a claim: paste `dev/gate`\'s own summary block VERBATIM from ONE full run (no --quick) — at minimum its `tests   passed=N failed=0  (over B test binaries)` totals line AND its `GATE: PASS` line. gate_green=true is INVALID without BOTH: the totals line is printed only in full mode (--quick omits it) and `GATE: PASS` only when every step exited 0. Raw `cargo test` output does NOT count — `test result:` says nothing about scope (a -p-scoped run prints it too, and the string also matches test NAMES like `test result::tests::…`); dev/gate takes no scope arguments, so its own verdict — not cargo\'s — is what one full run leaves behind. This checks WHAT YOU PASTE, not provenance: `--report <log>` prints the totals line over any log without running cargo, and `--quick` prints `GATE: PASS` without running tests, so pasting either in place of one real full run is a deliberate falsification rather than a slip.' },
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
// `branches` is filled once the reader has returned (incrementBranches); every build prompt
// names its increment's branch through `header`, so an agent can refuse to commit anywhere
// else even after a resume replayed the step that checked the branch out.
let branches = {}
function header(inc) {
  const bullets = (inc.scope || []).map((s) => '  - ' + s).join('\n')
  return ['Increment ' + inc.n + ': ' + inc.title, 'Deliverable: ' + inc.deliverable, 'Grouped scope:', bullets, 'Proves: ' + inc.proves, '', branchLine(branches[inc.n])].join('\n')
}
function branchLine(branch) {
  return 'BRANCH: ' + branch + ' (forked from ' + milestoneBranch + '), already checked out by the harness. Every commit of this increment lands on it: before you commit, `git branch --show-current` must print exactly `' + branch + '` — if it does not, commit nothing and halt. Never create, switch, merge or push a branch; the harness\'s git steps own them. `git log --oneline ' + milestoneBranch + '..' + branch + '` is this increment\'s work so far.'
}

// ---- the git steps (build-git) — exact command sequences, nothing improvised ----
const GIT_RULES = 'Run exactly the commands below, in order, and nothing else: no other branch, no commit beyond the merge a step names, no file edit, no `git stash`, no reset, never `--force`, and `main` is never checked out, merged into or pushed (reading `origin/main` is all a step does with it). Any check that fails, or any command that fails, is a HALT: stop, leave everything as it is, and fill the halt report (root_cause = which check, evidence = the command and its output, tree_state = `git status` and `git branch --show-current`).'
function milestoneBranchPrompt() {
  const b = milestoneBranch
  return [
    'GIT STEP — the milestone branch `' + b + '` for ' + milestone + '. ' + GIT_RULES,
    '1. `git status --porcelain` must print nothing (a dirty tree is never carried across a branch switch).',
    '2. `git fetch origin main`. A new branch starts from the CURRENT remote `main`, never from a stale local one: if `git rev-parse --verify --quiet refs/heads/main` succeeds, `git merge-base --is-ancestor main origin/main` must succeed too — else local `main` has diverged from `origin/main`: HALT, and never reset, merge or rebase it (it is the human\'s to reconcile).',
    '3. Find the branch: local = `git rev-parse --verify --quiet refs/heads/' + b + '` succeeds; remote = `git ls-remote --exit-code --heads origin ' + b + '` exits 0 (exit 2 means absent).',
    '   - local exists: `git switch ' + b + '`. If remote exists too, `git fetch origin ' + b + '`, then `git merge-base --is-ancestor origin/' + b + ' ' + b + '` must succeed (else the pushed branch has commits the local one lacks — HALT; never pull, merge or rebase it here). created = false.',
    '   - only remote exists: `git fetch origin ' + b + '` then `git switch --track -c ' + b + ' origin/' + b + '`. created = false.',
    '   - neither exists: `git switch --no-track -c ' + b + ' origin/main`, then `git rev-parse HEAD` and `git merge-base origin/main HEAD` must both equal `git rev-parse origin/main` — the new branch starts exactly at the remote `main` just fetched. created = true. Do NOT push it; the harness pushes it after its first landed increment.',
    '4. `git branch --show-current` must print `' + b + '`.',
    '5. Report branch, head = `git rev-parse HEAD`, fork_point = `git merge-base origin/main HEAD`, created.',
  ].join('\n')
}
function openIncrementPrompt(inc) {
  const m = milestoneBranch
  const b = branches[inc.n]
  return [
    'GIT STEP — open increment ' + inc.n + ' (' + inc.title + ') of ' + milestone + ' on `' + b + '`, forked from `' + m + '`. ' + GIT_RULES,
    '1. `git status --porcelain` must print nothing.',
    '2. `git switch ' + m + '`.',
    '3. If `git rev-parse --verify --quiet refs/heads/' + b + '` succeeds (a resumed increment): `git merge-base --is-ancestor ' + m + ' ' + b + '` must succeed, then `git switch ' + b + '`; created = false. Otherwise: `git switch --no-track -c ' + b + ' ' + m + '`; created = true.',
    '4. `git branch --show-current` must print `' + b + '`. Never push it: increment branches stay local.',
    '5. Report branch, head = `git rev-parse HEAD`, created.',
  ].join('\n')
}
function landPrompt(inc) {
  const m = milestoneBranch
  const b = branches[inc.n]
  return [
    'GIT STEP — land increment ' + inc.n + ' (' + inc.title + ') of ' + milestone + ': merge `' + b + '` into `' + m + '` with a merge commit, then delete it. It has been validated clean. ' + GIT_RULES,
    '1. `git status --porcelain` must print nothing.',
    '2. `git switch ' + m + '`.',
    '3. `git merge-base --is-ancestor ' + m + ' ' + b + '` must succeed — the increment was built on the milestone branch\'s current tip, so the merge cannot conflict; if it fails, the milestone branch moved under the increment and the human reconciles it.',
    '4. `git rev-list --count ' + m + '..' + b + '` must print a number above 0.',
    '5. `git merge --no-ff --no-edit ' + b + '`. If it fails: `git merge --abort`, then HALT.',
    '6. Check the merge: `git rev-parse \'HEAD^{tree}\'` must equal `git rev-parse \'' + b + '^{tree}\'`, and `git rev-list --parents -n 1 HEAD` must list exactly two parents, the second equal to `git rev-parse ' + b + '`. On a mismatch HALT and undo nothing.',
    '7. `git branch -d ' + b + '` — the safe delete, which refuses an unmerged branch; never `-D`.',
    '8. Report merge_commit = `git rev-parse HEAD`.',
  ].join('\n')
}
function pushPrompt(inc) {
  const m = milestoneBranch
  return [
    'GIT STEP — push `' + m + '` after increment ' + inc.n + ' of ' + milestone + ' landed. ' + GIT_RULES,
    '1. `git branch --show-current` must print `' + m + '` and `git status --porcelain` must print nothing.',
    '2. `git push origin ' + m + '` — the branch named in full, exactly so: a bare `git push`, `HEAD`, `main` or any force flag is denied or forbidden. A rejected push (someone else pushed the branch) is a HALT — never force, pull or rebase.',
    '3. `git ls-remote --exit-code --heads origin ' + m + '` must report the sha `git rev-parse ' + m + '` prints.',
    '4. Report remote_head.',
  ].join('\n')
}
// The close's SYNC step — the fixed step before any pull request to main (the human's
// decision of 2026-10-02; implementation/dev-workflow.md -> Before a pull request to main is
// its home). It has its own rules line rather than GIT_RULES, because it merges `origin/main`
// INTO the branch; GIT_RULES is left byte-identical, so no other step's prompt — or
// cache key — moves with it. A conflict confined to the append-only logs is resolved by
// `dev/merge-logs`, deterministically (both sides kept, in the file's date order; anything
// but a pure append refused), never by the agent's own edit. SYNC_LOGS mirrors that tool's
// LOGS, and crates/cli/tests/merge_logs_fence.rs holds the two equal.
const SYNC_LOGS = ['DECISIONS.md', 'implementation/project-history.md']
const SYNC_RULES = 'Run exactly the commands below, in order, and nothing else: no other branch, no commit beyond the merge this step names, no file edit (step 6\'s `dev/merge-logs` is the only thing that writes a file), no `git stash`, no reset, no rebase, never `--force`, and `main` is never checked out, merged into or pushed — `origin/main` is merged INTO the branch, which is all this step does with it. Any check that fails, or any command that fails, is a HALT: stop, leave everything as the step says, and fill the halt report (root_cause = which check, evidence = the command and its output, tree_state = `git status` and `git branch --show-current`).'
function syncMainPrompt() {
  const m = milestoneBranch
  const logs = SYNC_LOGS.map((p) => '`' + p + '`').join(' or ')
  return [
    'GIT STEP — merge `origin/main` into `' + m + '` before its pull request to `main` is opened (the close of ' + milestone + '). ' + SYNC_RULES,
    '1. `git branch --show-current` must print `' + m + '` and `git status --porcelain` must print nothing; pre = `git rev-parse HEAD`.',
    '2. `git fetch origin main`, then origin_main = `git rev-parse origin/main`.',
    '3. If `git merge-base --is-ancestor origin/main ' + m + '` succeeds, `main` has nothing the branch lacks: status = up-to-date, head = `git rev-parse HEAD`, and stop — steps 4–8 do not run.',
    '4. `git merge --no-ff --no-edit origin/main` — a merge commit, never a rebase. If it exits 0, go to step 7.',
    '5. It stopped. `git diff --name-only --diff-filter=U` lists the conflicted paths. If that list is empty, or names any path other than ' + logs + ': `git merge --abort`, then HALT — a conflict outside the append-only logs is the human\'s, never resolved here.',
    '6. `dev/merge-logs` — it resolves each conflicted log by keeping both sides\' entries whole in the file\'s date order, and stages it; it refuses, writing nothing, a hunk where either side changed text that was already there. If it exits non-zero: `git merge --abort`, then HALT with its stderr as evidence. If it exits 0: `git diff --name-only --diff-filter=U` must print nothing, then `git commit --no-edit`; resolved_logs = the paths step 5 listed.',
    '7. Check the merge: `git rev-list --parents -n 1 HEAD` must list exactly two parents, the first equal to pre and the second equal to `git rev-parse origin/main`; `git status --porcelain` must print nothing. On a mismatch HALT and undo nothing.',
    '8. Report status = merged, head = `git rev-parse HEAD`, origin_main, resolved_logs (empty after a clean merge). Do NOT push and do NOT open the pull request: the gate runs on the merged tree first.',
  ].join('\n')
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
// `addendum` — a corrective for the NEXT round only (empty on round 0, so that call stays
// cache-key-identical on resume). Appended, never interleaved, so the base prompt is byte-
// identical to what it has always been.
function validatePrompt(inc, addendum) {
  return [milestone + ' — ' + header(inc), '', 'Validate this increment against that roadmap spec per your validator role; exercise every grouped-scope bullet through the real binary or tests.', '', 'GATE IS A FACT, NOT A CLAIM: run the FULL gate yourself with `dev/gate` — never `--quick`, which skips the tests. It runs fmt/clippy/build/test each BARE with its exit code captured, and it accepts no scope arguments at all, so a dev/gate run cannot be the scoped `cargo test` that has landed a red gate before (M23 inc-1). Paste its summary block VERBATIM into `gate_evidence` — at minimum the `tests   passed=… failed=…  (over N test binaries)` totals line AND the `GATE: PASS` line. gate_green=true is INVALID without both pasted lines (a `--quick` run omits the totals line and does not count); any `GATE: FAIL` / any non-zero exit / any `FAILED` is a BLOCKING finding. Do not trust the executor\'s claim — re-run it.'].join('\n') + (addendum ? '\n\n' + addendum : '')
}
function fixPrompt(inc, f) {
  return [
    'Fix ONE blocking finding from ' + milestone + ' increment ' + inc.n + ':',
    'TITLE: ' + f.title,
    'EVIDENCE: ' + f.evidence,
    f.fix_hint ? 'HINT: ' + f.fix_hint : '',
    '',
    branchLine(branches[inc.n]),
    '',
    'Resolve it via your fixer role (red reproduces the defect -> minimal green -> full gate -> one commit).',
  ].join('\n')
}
function auditBranchLine() {
  return 'The milestone is `' + milestoneBranch + '`. Confirm `git branch --show-current` prints it; if it does not and `git status --porcelain` prints nothing, `git switch ' + milestoneBranch + '` first (that is the one git act you may perform). Commit nothing.'
}
function reviewPrompt(baseRef) {
  return ['Milestone: ' + milestone + '. Review everything on `' + milestoneBranch + '` after its fork point from main, the base commit: ' + baseRef + ' (`git log --oneline ' + baseRef + '..' + milestoneBranch + '`).', '', auditBranchLine(), '', 'Review the whole milestone diff per your code-reviewer role.'].join('\n')
}
function e2ePrompt() {
  return ['Milestone: ' + milestone + '.', '', auditBranchLine(), '', 'Drive the milestone acceptance flows end-to-end through the real binary in throwaway repos per your e2e role.'].join('\n')
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
      // `model` is merged UNDER opts, so a call that names its own model still wins.
      const result = await agent(prompt + note, Object.assign({ model }, opts))
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

// ---- the milestone branch: ensured and checked out BEFORE the reader ----
// The decomposition the reader enumerates was committed on this branch at planning
// (milestone-planning-workflow.md -> Decompose), so the reader must read it here, not on
// main. Its fork point from origin/main is the audit base.
phase('Milestone branch')
log('Ensuring ' + milestoneBranch + ' (reused if it exists locally or on origin, else created from origin/main)')
const mb = await agentR(milestoneBranchPrompt(), { label: 'git:milestone-branch', phase: 'Milestone branch', agentType: 'build-git', schema: BRANCH_SCHEMA })
if (!mb || mb.status !== 'ready' || mb.branch !== milestoneBranch || !mb.fork_point) {
  const transient = !mb
  const mode = haltMode(transient)
  return {
    status: 'halted',
    halted: {
      phase: 'branch',
      transient,
      branch: milestoneBranch,
      halt: mb && mb.halt ? mb.halt : { root_cause: mb ? 'the milestone-branch step reported ' + JSON.stringify({ status: mb.status, branch: mb.branch, fork_point: mb.fork_point }) + ' — not the ready ' + milestoneBranch + ' with a fork point' : 'the milestone-branch step returned no result' },
    },
    message: haltMessage(milestone, mode, 'branch'),
    resume: resumeLine(milestone, mode, 'branch', milestoneBranch),
  }
}
const forkPoint = String(mb.fork_point)
log(milestoneBranch + (mb.created ? ' created' : ' reused') + ' at ' + mb.head + '; fork point from origin/main ' + forkPoint)

// ---- read the milestone's increments from the roadmap ----
phase('Read milestone')
log('Reading ' + milestone + ' increment decomposition from implementation/roadmap.md')
const read = await agentR('Enumerate the ordered increments of milestone "' + milestone + '" from implementation/roadmap.md, as it stands on the checked-out branch `' + milestoneBranch + '`. If the request is "the next milestone", resolve it to the next milestone whose decomposition is present but status is planned-not-built, and return its canonical id (e.g. "M11") in the `milestone` field. For every increment the roadmap ends in a planned human halt, return it as `halt_after`.', { label: 'read:' + milestone, phase: 'Read milestone', agentType: 'milestone-reader', schema: INCREMENTS_SCHEMA })
const increments = read && read.increments ? read.increments : []
// The id the reader actually resolved — so the result names what was built even
// when the caller passed no args and the workflow auto-selected the next milestone.
const builtMilestone = read && read.milestone ? String(read.milestone) : milestone
if (increments.length === 0) {
  // TWO causes reach here and they are NOT interchangeable — the fourth halt construction,
  // and the one the `transient` flag was never wired into:
  //   * the reader RETURNED NOTHING (agentR exhausted its retries — 529, kill, user skip).
  //     The roadmap was never read, so nothing here is a statement about the decomposition.
  //   * the reader READ a roadmap that carries no decomposition for this milestone. That is
  //     the genuine "go plan it first" (or, on a resume, the dropped-args symptom).
  // Reporting the first AS the second told the operator to re-plan an already-planned
  // milestone and to clear a blocker that does not exist. The fail-safe default stands:
  // `transient` is set only on the one condition that means it, so unset => non-transient.
  const transient = !read
  // The dropped-args reading only applies when a roadmap was actually read: a null reader
  // says nothing about which milestone was asked for.
  const droppedArgs = !transient && milestone === 'the next milestone'
  // The breaker cannot be tripped here — it needs a SECOND distinct label to exhaust and the
  // reader is the first call of the run — so `haltMode` can only return transient/blocker.
  const mode = haltMode(transient)
  return {
    status: 'halted',
    halted: {
      phase: 'read',
      transient,
      reason: transient
        ? 'the milestone-reader returned no result after retries for ' + milestone + ' — the roadmap was never read, so this is NOT a verdict about the decomposition: it is the transient/rate-limit shape (see the failures channel above). Re-run the reader before concluding anything about the roadmap.'
        : 'no roadmap decomposition for ' + milestone + ' — '
          + (droppedArgs
              ? 'this is almost certainly a DROPPED-ARGS resume (RULE 0): re-invoke with args: { milestone: "<id>", base } explicitly.'
              : 'run the milestone-planning workflow first.'),
    },
    message: haltMessage(milestone, mode, 'read'),
    resume: resumeLine(milestone, mode, 'read', milestoneBranch),
    note: read ? read.note : null,
  }
}
branches = incrementBranches(increments)
log(builtMilestone + ' has ' + increments.length + ' increment(s): ' + increments.map((i) => 'I' + i.n + ' -> ' + branches[i.n] + (i.halt_after && i.halt_after.id ? ' [then ' + i.halt_after.id + ']' : '')).join(', '))

// ---- per increment: plan -> execute(per task) -> validate -> fix(bounded 3) ----
let halted = null
const incrementReports = []
for (const inc of increments) {
  phase('Build increments')

  // Deterministic resume: skip increments already built, validated CLEAN and merged into
  // the milestone branch (see RESUMING note 6). Cache-independent — the skipped increments never re-plan,
  // so the planner can't halt on already-built work.
  if (skipThrough && inc.n <= skipThrough) {
    log('Increment ' + inc.n + ' — already built, validated and merged into ' + milestoneBranch + '; skipping (skipThrough=' + skipThrough + ').')
    continue
  }

  // Open the increment's branch. For a never-run increment this call is new, so it runs
  // live even on a resume — which is what puts a resumed run back on the right branch
  // after a planned halt, whatever the human had checked out in between.
  const ob = await agentR(openIncrementPrompt(inc), { label: 'git:open:inc' + inc.n, phase: 'Build increments', agentType: 'build-git', schema: BRANCH_SCHEMA })
  if (!ob || ob.status !== 'ready' || ob.branch !== branches[inc.n]) {
    halted = { increment: inc.n, phase: 'branch', transient: !ob, branch: branches[inc.n], halt: ob && ob.halt ? ob.halt : { root_cause: ob ? 'the open-increment step reported branch ' + JSON.stringify(ob.branch) + ', status ' + ob.status + ' — not ' + branches[inc.n] : 'the open-increment step returned no result' } }
    incrementReports.push({ increment: inc.n, branch: ob })
    break
  }
  log('Increment ' + inc.n + ' — on ' + branches[inc.n] + (ob.created ? ' (created)' : ' (reused: a resumed increment)'))

  log('Increment ' + inc.n + ' — planning (' + inc.title + ')')
  const plan = await agentR(planPrompt(inc), { label: 'plan:inc' + inc.n, phase: 'Build increments', agentType: 'build-planner', schema: PLAN_SCHEMA })
  if (!plan || plan.status === 'halted' || !plan.tasks || plan.tasks.length === 0) {
    // `transient` is set HERE, by the harness, on the one condition that means it
    // (agentR exhausted its retries and returned null) — never re-derived downstream
    // from `halt.root_cause`, which is agent-written prose.
    halted = { increment: inc.n, phase: 'plan', transient: !plan, branch: branches[inc.n], halt: plan && plan.halt ? plan.halt : { root_cause: plan ? 'planner produced no tasks' : 'planner returned no result', tree_state: 'clean (planner writes only the DECISIONS entry, nothing on halt)' } }
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
      halted = { increment: inc.n, phase: 'execute', task: task.id, transient: !r, branch: branches[inc.n], halt: r && r.halt ? r.halt : { root_cause: r ? 'executor halted without detail' : 'executor returned no result' } }
      break
    }
  }
  if (halted) { incrementReports.push({ increment: inc.n, plan, execResults }); break }

  let round = 0
  let lastValidation = null
  let nullRounds = 0
  let fixerNulls = 0
  let fixerUnfixed = 0
  let gateAddendum = ''
  while (true) {
    log('Increment ' + inc.n + ' — independent validation (after ' + round + ' fix round(s))')
    const v = await agentR(validatePrompt(inc, gateAddendum), { label: 'validate:inc' + inc.n + ':r' + round, phase: 'Build increments', agentType: 'increment-validator', schema: VALIDATION_SCHEMA })
    lastValidation = v
    if (!v) nullRounds++
    const blocking = v && v.blocking ? v.blocking : []
    // gate_green must be BACKED by pasted evidence, not a bare boolean — and the evidence
    // is `dev/gate`'s OWN VERDICT, never raw cargo output. dev/gate is unscopeable (its arg
    // loop takes only --quick/--private-target/--report/--help and exits 2 on anything else),
    // it prints `GATE: PASS` only after every step exited 0, and it prints the
    // `tests   passed=… failed=…  (over N test binaries)` totals line ONLY in full mode
    // (--quick omits it entirely). So demanding BOTH lines rejects a red gate, a bare
    // --quick run, and the ACCIDENTAL scoped paste this check exists to catch (M23: inc-1
    // landed a red gate because the validator claimed green off a scoped/lacon-trimmed run).
    // HONEST BOUND — this is NOT proof of provenance, and must not be read as one: the
    // predicate tests PASTED TEXT, not that `dev/gate` ran unscoped. Both lines are
    // assemblable from two sanctioned fast invocations. `dev/gate --report <log>` runs NO
    // cargo at all (dev/gate:139-146) and prints the totals line over ANY log — driven over
    // a `cargo test -p jigc-engine` log it printed `tests   passed=948 failed=0  (over 1 test
    // binaries)`, which satisfies this regex — and `dev/gate --quick` reaches `GATE: PASS`
    // (dev/gate:282) without running tests. So the check raises the forgery cost from a
    // careless paste to a deliberate two-command assembly; it cannot detect the latter.
    // The OLD predicate, /test result:/ over pasted cargo output, caught NONE of that:
    //   * a scoped `cargo test -p jigc-engine` prints a genuine `test result: ok. …` summary too,
    //     so it passed on exactly the input it was written to reject; and
    //   * the pattern is UNDELIMITED, so it also matches test NAMES — engine's `pub mod
    //     result` has a `mod tests`, so cargo prints `test result::tests::alpha ... ok`.
    //     Measured on `cargo test -p jigc-engine --lib`: 17 matches, 16 of them names and 1 a
    //     real summary — i.e. evidence with zero real summaries could still pass.
    // Same undelimited-match bug, second site: 03abf34 fixed it in dev/gate's own counting
    // awk (33 binaries reported for a 17-binary run) by matching fields instead.
    // The binary count is deliberately NOT hard-coded here — it is derived inside dev/gate
    // from the run itself; pinning it would only mint a drift point against tests/groups/.
    // The leading `(?:[>*+-][ \t]*)*` tolerates a PREFIX-decorated paste and nothing wider:
    // a quote marker or a list bullet in front of the line (`> tests   passed=…`, `- GATE: PASS`,
    // `  * GATE: PASS`), plus leading indent/tabs. It deliberately does NOT admit the WRAPPING
    // forms — `**GATE: PASS**`, `` `GATE: PASS` `` and `1. GATE: PASS` are all REJECTED — because
    // the property that rejects prose is that NOTHING trailing is tolerated (`The run reported
    // GATE: PASS.` does not match), and every wrapping form needs exactly that property loosened.
    // Failing closed on them is cheap now: a gate_green=true paste that misses lands in `missing`
    // below and comes back as a targeted addendum on the very next round.
    const gatePassRe = /^[ \t]*(?:[>*+-][ \t]*)*GATE: PASS[ \t]*$/m
    const gateTotalsRe = /^[ \t]*(?:[>*+-][ \t]*)*tests[ \t]+passed=\d+[ \t]+failed=0[ \t]+\(over [1-9]\d* test binaries\)[ \t]*$/m
    const ev = v && typeof v.gate_evidence === 'string' ? v.gate_evidence : ''
    const gateProven = !!(v && v.gate_green && gatePassRe.test(ev) && gateTotalsRe.test(ev))
    if (blocking.length === 0 && gateProven) { log('Increment ' + inc.n + ' — validated CLEAN'); break }
    // The diagnosis must reach the NEXT ROUND'S PROMPT, not only the log: with zero blocking
    // findings no fixer spawns, so the tree does not change either — and validatePrompt is a
    // pure function of `inc`, so an un-addended retry is byte-identical and fails identically
    // for rounds 0..3, burning four full gate runs to reach a halt. Empty => nothing appended.
    //
    // TWO ways a round can leave the gate unproven with NOTHING to fix, and both need their
    // own addendum — the second is the sibling arm of this ternary, and it was the one left
    // un-swept: a validator that honestly reports gate_green=FALSE while filing the red gate
    // as *advisory* (or nowhere) yields no `missing`, no addendum and no fixer, so the round
    // is a pure no-op that repeats to the 3-round halt — whose reason then blamed the
    // EVIDENCE, the same confident-wrong-cause shape the `nullRounds` branch exists to kill.
    const missing = (v && v.gate_green && !gateProven)
      ? [gatePassRe.test(ev) ? null : 'the `GATE: PASS` verdict line', gateTotalsRe.test(ev) ? null : 'the full-run `tests   passed=… failed=0  (over N test binaries)` totals line (a --quick run omits it)'].filter(Boolean).join(' and ')
      : ''
    const redGateUnfiled = !!(v && !v.gate_green && blocking.length === 0)
    if (blocking.length === 0 && missing) {
      log('Increment ' + inc.n + ' — gate claimed green but gate_evidence is missing ' + missing + ' from a full `dev/gate` run; treating as unverified → fix round')
    }
    if (redGateUnfiled) {
      log('Increment ' + inc.n + ' — validator reported gate_green=false yet filed ZERO blocking findings, so no fixer can spawn; re-asking with the "a non-green gate is BLOCKING" addendum')
    }
    gateAddendum = missing
      ? 'PREVIOUS ROUND REJECTED: you reported gate_green=true but `gate_evidence` was missing ' + missing + ', so the gate is unverified and this increment is still unvalidated. Re-run the FULL gate yourself — `dev/gate` with NO flags (`--quick` skips the tests and omits the totals line; `--report <log>` runs no cargo at all) — and paste its summary block VERBATIM and UNEDITED: both the totals line and the `GATE: PASS` line, each on its own line. Do not paraphrase it, do not substitute raw `cargo test` output.'
      : redGateUnfiled
      ? 'PREVIOUS ROUND REJECTED: you reported gate_green=false and filed ZERO blocking findings. That combination cannot make progress — a fixer only ever spawns for a BLOCKING finding, so nothing changed in the tree and this prompt would otherwise repeat unaltered to the round limit. Your role contract is explicit: any `GATE: FAIL`, any non-zero exit, any `FAILED` is a BLOCKING finding, never advisory. So this round: re-run the FULL gate (`dev/gate`, no flags), and if it is still not green FILE IT AS BLOCKING — one finding whose `evidence` names dev/gate\'s failing step and the failing test name(s) it prints. If it IS green, report gate_green=true and paste the summary block verbatim (both the totals line and the `GATE: PASS` line).'
      : ''
    if (round >= 3) {
      // A null validator result is NOT a statement about the gate — it never ran. Saying
      // "gate_evidence never carried both lines" there is a confident wrong cause. Same for a
      // round the fixers never worked, and same for a red gate the validator filed as advisory.
      const reason = blocking.length > 0
        ? blocking.length + ' blocking finding(s) remain after 3 fix rounds'
          + (fixerNulls ? ' — but ' + fixerNulls + ' fixer call(s) returned NO RESULT, so the findings may never have been worked at all: that is the transient/rate-limit shape, not a verdict that they are unfixable. Re-run the fixers before triaging, and note anything a dead fixer left is UNCOMMITTED in the tree.' : '')
          + (fixerUnfixed ? ' — ' + fixerUnfixed + ' fixer call(s) reported could-not-fix; read their notes (they carry the trace showing the finding is not real) before assuming the findings stand.' : '')
        : !v
        ? 'the validator returned NO RESULT on the last of 4 rounds (' + nullRounds + ' of 4 returned none) — the increment was never actually validated, so this is not a verdict about the code: it is the transient/rate-limit shape (see the failures channel and the breaker log above). Re-run the validator before triaging anything.'
        : !v.gate_green
        ? 'the validator reported gate_green=FALSE on the last of 4 rounds while filing ZERO blocking findings — so the real cause is a RED GATE recorded as advisory (or not recorded at all), NOT missing evidence and NOT the code being unfixable: no fixer ever spawned, because a fixer only spawns for a blocking finding. Run `dev/gate` by hand, read its failing step and test names, and fix that.'
        : 'gate green could not be verified after 3 validation rounds (gate_evidence never carried both `GATE: PASS` and the full-run `tests … (over N test binaries)` totals line from one `dev/gate` run) — verify the gate by hand'
      halted = { increment: inc.n, phase: 'validate', transient: !v && blocking.length === 0, branch: branches[inc.n], reason, blocking }
      break
    }
    round++
    // Only a round that actually spawns a fixer is a FIX round. Logging "fix round N: 0
    // blocking finding(s)" three times over described a loop that fixed nothing as if it had
    // been fixing — the log is the operator's only live view of the loop, so it says which
    // no-op this is and that no fixer will spawn.
    log('Increment ' + inc.n + ' — ' + (blocking.length > 0
      ? 'fix round ' + round + ': ' + blocking.length + ' blocking finding(s)'
      : 're-validation round ' + round + ': NOTHING TO FIX (0 blocking findings), so no fixer spawns — '
        + (missing ? 'the gate evidence was short ' + missing : redGateUnfiled ? 'the validator reported the gate NOT green but filed no blocking finding' : 'the gate was not proven')
        + '; re-asking with a corrective addendum'))
    for (const f of blocking) {
      // The fixer's result is CAPTURED, not discarded. A null return means the fixer landed
      // nothing (agentR exhausted its retries, or the breaker is tripped) — and it may have
      // died between gate-green and commit, leaving that work loose in the tree, which is
      // precisely the third tree state the validate-phase guidance now names. Deliberately
      // NOT a halt of its own: the finding still stands, re-validation is the honest next
      // step, and the bounded round budget still ends the run — but the ROUND-LIMIT REASON
      // must not report "3 fix rounds could not fix it" when zero fixers ever ran.
      const fr = await agentR(fixPrompt(inc, f), { label: 'fix:inc' + inc.n + ':r' + round, phase: 'Build increments', agentType: 'build-fixer', schema: FIX_SCHEMA })
      if (!fr) {
        fixerNulls++
        log('Increment ' + inc.n + ' — fixer returned NO RESULT on round ' + round + ' (' + f.title + '); the finding stands and anything it left is UNCOMMITTED in the tree')
      } else if (fr.status === 'could-not-fix') {
        fixerUnfixed++
        log('Increment ' + inc.n + ' — fixer reported could-not-fix on round ' + round + ' (' + f.title + '): ' + (fr.notes || '(no notes)'))
      }
    }
  }

  if (halted) {
    incrementReports.push({ increment: inc.n, plan, execResults, validation: lastValidation, fixRounds: round })
    break
  }

  // ---- land: merge back --no-ff, delete the increment branch, push the milestone branch ----
  log('Increment ' + inc.n + ' — landing ' + branches[inc.n] + ' on ' + milestoneBranch + ' (--no-ff)')
  const landed = await agentR(landPrompt(inc), { label: 'git:land:inc' + inc.n, phase: 'Build increments', agentType: 'build-git', schema: LAND_SCHEMA })
  if (!landed || landed.status !== 'merged' || !landed.merge_commit) {
    halted = { increment: inc.n, phase: 'land', transient: !landed, branch: branches[inc.n], halt: landed && landed.halt ? landed.halt : { root_cause: landed ? 'the land step reported status ' + landed.status + ' with no merge commit' : 'the land step returned no result' } }
    incrementReports.push({ increment: inc.n, plan, execResults, validation: lastValidation, fixRounds: round, land: landed })
    break
  }
  const pushed = await agentR(pushPrompt(inc), { label: 'git:push:inc' + inc.n, phase: 'Build increments', agentType: 'build-git', schema: PUSH_SCHEMA })
  if (!pushed || pushed.status !== 'pushed' || pushed.remote_head !== landed.merge_commit) {
    halted = { increment: inc.n, phase: 'push', transient: !pushed, branch: milestoneBranch, halt: pushed && pushed.halt ? pushed.halt : { root_cause: pushed ? 'the push step reported status ' + pushed.status + ', remote head ' + JSON.stringify(pushed.remote_head) + ' against merge commit ' + landed.merge_commit : 'the push step returned no result', recommendation: 'increment ' + inc.n + ' IS merged into ' + milestoneBranch + ' (' + landed.merge_commit + '); push it by name and resume with skipThrough: ' + inc.n } }
    incrementReports.push({ increment: inc.n, plan, execResults, validation: lastValidation, fixRounds: round, land: landed, push: pushed })
    break
  }
  log('Increment ' + inc.n + ' — landed as ' + landed.merge_commit + ' and pushed ' + milestoneBranch)
  incrementReports.push({ increment: inc.n, branch: branches[inc.n], plan, execResults, validation: lastValidation, fixRounds: round, merge_commit: landed.merge_commit })

  // ---- a planned human halt is a stage boundary: return, with the human's checklist ----
  const h = inc.halt_after
  if (h && h.id) {
    if (cleared.includes(String(h.id))) {
      log('Increment ' + inc.n + ' — planned halt ' + h.id + ' is cleared (args.cleared); carrying on')
    } else {
      return {
        status: 'planned-halt',
        halt: { id: String(h.id), after_increment: inc.n, checklist: h.checklist || [] },
        message: builtMilestone + ' increment ' + inc.n + ' is built, validated clean, merged into ' + milestoneBranch + ' (' + landed.merge_commit + ') and pushed. The roadmap ends it in ' + h.id + ', the human\'s: the run stops here BY DESIGN, not on a fault. Nothing is owed by an agent. The human works the checklist; then resume, and the next increment\'s planner opens with the entry-gate task that verifies it.',
        checklist: h.checklist || [],
        resume: 'TO RESUME once the checklist is done (the default, per the script header → PLANNED HALTS): Workflow({ scriptPath: <the snapshot path printed at launch>, args: ' + argsLiteral(builtMilestone, String(h.id), null) + ', resumeFromRunId: <this run id> }) — every call so far replays from cache and the boundary is passed because ' + h.id + ' is in `cleared`. Fallback, cache-independent: a FRESH run with args: ' + argsLiteral(builtMilestone, String(h.id), inc.n) + '.',
        milestone: builtMilestone,
        branch: milestoneBranch,
        forkPoint,
        incrementReports,
      }
    }
  }
}

if (halted) {
  // Transient-infrastructure halts (an agent returned no result after retries) get the
  // no-blocker-to-resolve resume message; genuine blockers keep the resolve-first one.
  // The discriminator is the flag the HARNESS set at halt construction, never a pattern over
  // `halt.root_cause` — that is a field an LLM fills with prose, and the old
  // /returned no result/ test matched real design forks verbatim ("the census grep for
  // existing `slot_ceiling` callers returned no result…", "`jigc doc show …` returned no
  // results" — note the plural matches an undelimited substring test too), printing
  // "nothing to fix, resume immediately" over an unresolved fork at exit 0.
  // ONE mode for BOTH surfaces — the message and the resume line are generated from the same
  // classification, so they cannot hand the operator two instructions. The breaker wins over
  // `transient` (they are simultaneously true whenever the breaker trips on a null return),
  // and the tree rule comes from the single phase-aware `treeGuidance` in both.
  const mode = haltMode(halted.transient)
  return { status: 'halted', halted, message: haltMessage(builtMilestone, mode, halted.phase), resume: resumeLine(builtMilestone, mode, halted.phase, halted.branch), milestone: builtMilestone, branch: milestoneBranch, incrementReports }
}

// ---- milestone-completion audit (independent, adversarial, parallel) ----
phase('Milestone audit')
const baseRef = forkPoint
log('All increments validated clean and landed on ' + milestoneBranch + ' — running the milestone-completion audit (code review + e2e) over ' + baseRef + '..' + milestoneBranch)
const audit = await parallel([
  () => agentR(reviewPrompt(baseRef), { label: 'audit:code-review', phase: 'Milestone audit', agentType: 'milestone-code-reviewer', schema: REVIEW_SCHEMA }),
  () => agentR(e2ePrompt(), { label: 'audit:e2e', phase: 'Milestone audit', agentType: 'milestone-e2e-tester', schema: E2E_SCHEMA }),
])

return {
  status: 'built-and-audited',
  message: builtMilestone + ' fully built and independently validated clean, every increment merged into ' + milestoneBranch + ' and pushed. Milestone-completion audit complete. THE CLOSE (milestone-completion-workflow.md → The loop, and → Close): the triage fixes below land on ' + milestoneBranch + ' — each fixer is told that branch and commits there. THEN, IN THIS ORDER, before any pull request (implementation/dev-workflow.md → Before a pull request to main): (1) SYNC — spawn a `build-git` subagent (Agent tool, agentType build-git) with `close.sync.prompt` VERBATIM; it merges `origin/main` into ' + milestoneBranch + ' with a merge commit (never a rebase — agents may not force-push), resolving a conflict confined to the append-only logs by keeping both sides (`dev/merge-logs`), and its report has the shape of `close.sync.schema`. A sync that HALTS (a conflict outside the logs) goes to the human — open no PR. (2) GATE — `dev/gate` green on the merged tree (a red one is a defect `main` brought in: a `build-fixer`, committing on ' + milestoneBranch + '). (3) PUSH it by name (`git push origin ' + milestoneBranch + '`). (4) Only then does the ORCHESTRATOR open the pull request: `gh pr create --base main --head ' + milestoneBranch + '`. While that PR touches the shared logs, start no second branch that touches them (CLAUDE.md → Branches). The human merges it (agents never merge a PR or push main); the release PR updates from that merge. NOW: verify each finding is real (reproduce it), then AUTO-FIX every confirmed finding — delegate each to a `build-fixer` subagent (dev-workflow, one commit), the SAME autonomy the build phase has. Do NOT ask the human per finding and do NOT present a fix-vs-defer menu: leaving a confirmed finding unfixed degrades the milestone, so "defer / known-limitation" is NOT a default disposition. The human gate fires for EXACTLY two cases, and only after you have confirmed the finding: (a) too-big — the fix genuinely warrants its own increment (still scheduled, never dropped); (b) contested — the fix would revise a settled decision or change intended behavior. Size, not severity, decides the lane: a HIGH that is a bounded fix is still fix-now, and a pre-existing defect the milestone’s own flow exercises + a builder test masked is fix-now (not defer). CHEAP-VS-ROBUST FIRES AT TRIAGE TOO: a defer that leaves a KNOWN HOLE in the milestone declared/goal-complete surface (a capability reachable only through the engine/tests, not the shipped verb/CLI — the deliverable-reachable lens), or trips a ONE-WAY-DOOR tell, is NOT a valid defer even when large — it means the declared deliverable is hollow and the milestone IS NOT DONE, so the robust fix is fix-now (or, if genuinely huge, the milestone is BLOCKED, never quietly shipped hollow). Two rationalizations are BARRED: "no live case to test" is NOT "not needed" (a reconstructed/synthetic case proves a now-needed capability), and "premature generality" holds only if the trajectory does not commit. AND this fork is NEVER self-framed: before you surface any too-big->defer OR contested->document-it recommendation, spawn a `robust-advocate` subagent (Agent tool, agentType robust-advocate) to argue the vision-robust case at full strength, and present ITS case beside the cheap one — never your lone cheap recommendation (the M34 failure: the orchestrator self-framed the fork and led the human to defer the milestone declared deliverable). Everything else is a tested commit the human reviews AFTER. See milestone-completion-workflow.md → Plan (triage); methodology-docs.md → the independent robust-case advocate.',
  milestone: builtMilestone,
  branch: milestoneBranch,
  base: forkPoint,
  incrementReports,
  audit: { code_review: audit[0], e2e: audit[1] },
  close: {
    order: ['triage fixes on ' + milestoneBranch, 'sync: close.sync, spawned verbatim', 'dev/gate green on the merged tree', 'git push origin ' + milestoneBranch, 'gh pr create --base main --head ' + milestoneBranch],
    sync: { agentType: 'build-git', label: 'git:sync-main', prompt: syncMainPrompt(), schema: SYNC_SCHEMA },
  },
}
