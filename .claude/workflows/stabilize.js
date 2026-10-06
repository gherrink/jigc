// stabilize — the harness of the stabilization workflow: one script, two stages, each a
// fresh invocation (DECISIONS.md -> 2026-10-05, "The stabilization workflow, as ruled";
// what this script settles inside those rulings is the entry of 2026-10-06, "The
// stabilization harness, as built", and what the entry "The decision table's holes,
// closed" changed in it). It takes a run that the human opened — a build and a
// closing condition — and works one round of it: `test` finds and grades, `fix` fixes,
// audits the fix diff and lands. Every point where the human may rule lies BETWEEN two
// invocations, so a stop is a return, never an agent waiting.
//
// WHAT THIS SCRIPT DECIDES, AND WHAT IT DOES NOT. It has no shell, no file system and no
// clock: it sees what an agent returns to it. The run's committed state is read by
// `dev/stabilize-record state`, relayed by a git step inside the ONE line its command prints
// and checked here against the hash that line ends with — and whether a finding is inside
// the round's test set, where it is routed, which items a round runs, which round, cycle and
// attempt an invocation works on, and what happens next are computed THERE, where a suite
// holds them to truth tables on every gate. This script computes none of them again. `next`
// is returned to the orchestrator verbatim; `unsettled`, and any value this script does not
// know, is a return with the state attached — never a guess and never a loop. Nothing a
// previous invocation knew is used: an invocation starts from the state document and from
// git.
//
// THE BOUND ACROSS ROUNDS AND THE STOP MODE (ruling 6) are the record script's too: the
// run's opening writes them (`dev/stabilize-record run-set`), `next` is `stop` where the
// human is asked before the step that follows, and the position refuses a stage of a run
// whose opening is not done (`not-ready`), a round after a stop the human has not lifted
// (`stopped`) and one more fix round once the bound is spent (`round-bound`). This script
// holds neither number and starts no stage the position refuses.
//
// WHAT THE HUMAN RULES ABOUT THE RUN is recorded like every ruling: by the one step that
// records `args.rulings`. A go after a stop, one more re-run of a clause that is still not
// green, and a raised bound are facts of the record (`round-set`, `run-set`) — an invocation
// that carries them records them on the loop branch, starts nothing, and returns the state's
// `next`, which then names the step the stop was holding.
//
// AN UNFINISHED TRIAGE (`next: 'triage'`) is finished by the stage that left it: the position
// marks that stage `triage: true`, and its next invocation grades, verifies and records the
// rows the state lists as untriaged and runs NO instrument — every report of the round is on
// record already. And every stage's triage is handed those rows beside its own reporters'
// findings (`ledgerSource`): a row seeded at the opening, and an entry an earlier stage left
// without a verdict, never wait for a stage of their own.
//
// A STAGE THAT IS NOT FIT FOR USE REFUSES TO START (the human's ruling of 2026-10-06 on the
// order of the repair: the `test` half first). The build was reviewed red, and the `fix`
// stage is named in NOT_FIT: an invocation of it returns `refused` before any agent runs,
// whatever its arguments — an exit at a bound included — until its own repair and the
// re-review of that repair are recorded. The one invocation that is not refused starts no
// stage: one that only records what the human ruled about the run (below).
//
// THE STAGES (ruling 9).
//   test   git state (the path-class assert) -> state -> preflight ∥ scope -> state ->
//          the round's instruments, in parallel -> the reports checked -> triage ->
//          verify-real -> per fork an advocate and an independent drive -> the record
//          step (its batch applied, the full gate) -> the record's one commit on the loop
//          branch, held to the candidate's gate -> push -> state.
//          Returns {status: 'triaged', round, candidate, counts, human_list, forks, next}.
//   fix    git state -> state -> the round's branch found -> [the human's rulings
//          recorded] -> per cycle: the round branch opened -> fixers, one area at a time,
//          serial, one finding per commit -> push -> the round-tip binary -> the audit of
//          the fix diff, in parallel -> triage -> verify-real -> the record step on the
//          round branch -> push -> state -> land when nothing is left open, else the next
//          cycle while blockers remain and the bound allows.
//          Returns {status: 'landed', candidate, cycles, counts, doors_for_retest}; at the
//          bound {status: 'halted', halted: {phase: 'bound'}}, unlanded; on a fixer's
//          new-mechanism halt {status: 'halted', halted: {phase: 'fork'}} with the
//          advocate's case and the independent drive.
//
// THE BOUND AND ITS EXITS (ruling 6). A round has at most DEFAULT_CYCLES fix -> audit
// cycles; the next one is not started, and the stage returns `phase: 'bound'` with the
// round unlanded. The human's exits, each an invocation of `fix`:
//   continue      args.raise = { cycles: N }   — N above DEFAULT_CYCLES, passed again on every
//                                                later `fix` of that round (nothing is
//                                                remembered between invocations)
//   drop          args.exit  = 'drop'          — the round's record says so, its fixes are
//                                                marked open again, and exactly its record
//                                                commits are carried over to the loop branch
//   land a part   args.exit  = { part: [sha…] } — the fix commits the human keeps are re-cut
//                                                as a branch tip of their own, gated and
//                                                audited once more as exactly that diff,
//                                                and landed if that audit finds no new blocker
//
// THE GIT ACTS. Every git act of a run — and every read of its record — is ONE command of
// `dev/stabilize-step`, which does the act, checks it and prints ONE line: a git step's
// prompt is that command and "relay its line", and `readStep` holds the relayed line to the
// hash it ends with before anything here reads it. What an act runs and what it refuses is
// that tool's (its header); which act a stage asks for, in which order and with which names,
// is this script's. ONE step is still a list of commands the agent follows: the re-cut of a
// part (`carryPrompt` with a part), because that exit is ruled to be rebuilt as a revert
// (DECISIONS.md -> 2026-10-06) and a mechanism about to go is not ported.
//
// BRANCHES (ruling 7). Every branch name is minted in ONE function, `branchName`. A round's
// branch is opened from the loop branch, pushed by name and landed `--no-ff`; product paths
// (PRODUCT_PATHS) move only through such a merge, and a git step asserts it by path class.
// The `test` stage's records land on the loop branch; the `fix` stage's ride the round
// branch and reach the loop branch when the round lands, or by the carry-over when it is
// dropped (adjustment iv). No step force-pushes, rebases or touches `main`.
//
// REPORTS (ruling 12). Every reporter writes its report through `dev/stabilize-record`,
// under the name this script hands it on its `REPORT:` line. The script keeps the list of
// every reporter it launched, has `check-reports` run on that list before a record commit,
// and halts on a missing or an extra one.
//
// ONE BINARY PER CANDIDATE (ruling 11). Preflight builds it; every driving agent is handed
// its path and hash on a `BINARY:` line and returns the hash it asserted; this script
// compares the two itself and halts on a mismatch.
//
// THE LABELS. The contracts of the agent definitions bind on lines of the prompt — LABELS,
// below. Each is spelled here exactly as the definitions spell it, and
// tooling-tests/stabilize_harness_fence.rs holds the two together.
//
// THE RECORD STEP (the repair's change C4; DECISIONS.md -> 2026-10-06, "A stabilization
// run's record commit under a red candidate"). A stage's tables are written in ONE place and
// in three acts. (1) A `build-executor` applies the record's calls as ONE batch
// (`dev/stabilize-record apply`): every table is written or none is, and the record script
// holds the batch as pending. (2) The same agent runs the FULL gate on the tree with the
// records and keeps its whole output. (3) A git step makes the one commit
// (`dev/stabilize-step record`): it holds that gate to the candidate's own — which the
// preflight ran, and whose red steps and tests the batch itself puts on record — and commits
// when nothing is red that the candidate's gate did not show red. On a green candidate that
// is the rule that a commit follows a green gate. Red that is new is a halt that names it,
// with the batch still applied: THE NEXT INVOCATION OF EITHER STAGE FINDS THE PENDING BATCH,
// runs the gate on it again and commits it, and does nothing else (`finishRecord`). What is
// accepted as recorded is the commit step's own line, held to its hash: the commit, the gate
// check that holds, and as many calls and checks as this script composed.
//
// RESUMING. A stage that returned — halted, stopped at a bound, or finished — is not
// resumed: the next invocation is a fresh one and reads the state as it stands. What a
// halted stage wrote through the record script and no record step committed — its reports,
// the round's scope — is pending, not a dirty tree, and stays where it is; the next
// attempt's reports carry the next attempt number, which the state document gives. `resumeFromRunId` is for a KILLED run
// only, and then with the SAME args: every agent call's cache key is its prompt.
//
// Usage:  Workflow({ name: 'stabilize', args: { stage: 'test', run: '<run>', scratch: '<dir>' } })
//   stage     — REQUIRED: 'test' or 'fix'.
//   run       — REQUIRED: the run's slug. Its directory is completions/artifacts/<run>/ and
//               its branches are named from it (`branchName`).
//   scratch   — REQUIRED: an absolute directory, outside the repository, that every agent of
//               the invocation works under and passes to the record script as --scratch.
//   scope     — OPTIONAL, `test` only: the scope the round is started with — 'delta',
//               'everything', { range: '<sha>..<sha>' } or { doors: [ … ] }. Absent: the
//               run's default scope, a fact of its record. A round's scope is written once.
//   clause    — `test` only, and REQUIRED EXACTLY WHEN THE STATE ASKS FOR A RE-RUN (`next` is
//               `retest`): the first clause its `retest` names. The invocation runs the
//               items of that clause the state lists as due — its position's `rerun` — and
//               nothing else, as ONE MORE ATTEMPT INSIDE THE ROUND THAT SELECTED THEM, over
//               that round's doors: no round is begun, no scope resolved, and the record is
//               one result per item (the human's ruling of 2026-10-06: that instrument is
//               run again once; the record script counts the attempts, and takes a re-run
//               only where its state asks for one). A `clause` the state does not ask for,
//               and a `retest` answered without it, are refused before any agent runs.
//               Never together with `scope` or `crossModel`.
//   crossModel — OPTIONAL, `test` only: [ '<item>', … ] — the items of the test set whose
//               source pass is ALSO read by a model of another family, named one by one.
//               There is no value that means every item: heavy use is a deliberate act,
//               item by item. Absent, no stage names, calls or needs that tool, and a
//               machine that lacks it runs the whole workflow. A named item whose pass
//               could not run is returned and recorded as void for that pass; the source
//               pass a definition of this repository runs is never failed by it. (The
//               human's rulings of 2026-10-06; a cross-model second opinion is a human-
//               approved suggestion, never auto-run — implementation/milestone-planning-
//               workflow.md -> Review — and naming the item is the approval.)
//   rulings   — OPTIONAL: the human's rulings, recorded by ONE step.
//               On a finding or a bound:
//               [{ key, ruling: 'admitted' | 'later', note? } | { key, ruling: 'bound',
//               bound, reach, where, pin } | { bound, reach, where, pin }] — `where` is where
//               the human ruled it, `pin` the test that pins the bound or the word `unpinned`.
//               On `fix` they are recorded before anything is fixed, and the stage goes on.
//               On `test` the invocation records them on the loop branch and starts nothing
//               — the road they take while the `fix` stage refuses to start.
//               About the run, either stage — the invocation records them and starts nothing:
//               [{ go: true } | { rerun: '<clause>' } | { rounds: N } | { cycles: N }
//               | { reverify: '<key>' } | { again: 'test' | 'fix' }] — the go after the
//               stop the state names (`stop.why: 'every-round'`), which answers that stop and
//               no later one; one more re-run of a clause the state lists in `human_clauses`;
//               the bound across rounds, raised (the go after `stop.why: 'round-bound'`), and
//               the bound on a round's fix cycles, raised (after `stop.why: 'cycle-bound'`);
//               one more triage of a finding the state's `human_list` names as still ungraded
//               or unverified after its retry; one more attempt of a stage the state lists in
//               `human_stages`. The record script takes each only while its state asks for it.
//               The two kinds are two invocations.
//   raise     — OPTIONAL, `fix` only: { cycles: N }, the human's raise of the cycle bound.
//   exit      — OPTIONAL, `fix` only: 'drop' or { part: [ … ] }, the human's exit at a bound.
//   stopAfter — OPTIONAL: return after a named step (STOPS), for tuning. Nothing is recorded.
//   selfTest  — OPTIONAL: true runs zero agents and checks this script's own logic.
//   model     — OPTIONAL: the model every agent call but the git steps is pinned to ('opus').

export const meta = {
  name: 'stabilize',
  description: 'One stage of a stabilization run. test: preflight, scope, the round\'s instruments, triage, verify-real, the record. fix: the human\'s rulings, fixers, the audit of the fix diff (at most 3 cycles), land — or the exits at a bound: continue, drop, land a part. Reads the round, the cycle and the next step from dev/stabilize-record state. Args: { stage, run, scratch, scope?, clause?, crossModel?, rulings?, raise?, exit?, stopAfter?, selfTest?, model? }.',
  phases: [
    { title: 'State' },
    { title: 'Preflight and scope' },
    { title: 'Instruments' },
    { title: 'Fix' },
    { title: 'Triage' },
    { title: 'Record' },
    { title: 'Land' },
  ],
}

// ---- constants ----
const STAGES = ['test', 'fix']
const ARGS = ['stage', 'run', 'scratch', 'scope', 'clause', 'crossModel', 'rulings', 'raise', 'exit', 'stopAfter', 'selfTest', 'model']
// The steps a stage can be stopped after, for tuning.
const STOPS = { test: ['state', 'preflight', 'instruments', 'triage'], fix: ['state', 'rulings', 'fixers', 'audit'] }
const SLUG_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/
const SLUG_MAX = 100
const SCRATCH_RE = /^(\/[A-Za-z0-9._-]+)+$/
const SHA_RE = /^[0-9a-f]{40}$/
const SHORT_SHA_RE = /^[0-9a-f]{7,40}$/
const SHA256_RE = /^[0-9a-f]{64}$/
const RANGE_RE = /^[0-9a-f]{7,40}\.\.[0-9a-f]{7,40}$/
const MODEL_RE = /^[a-z][a-z0-9.-]*$/
// Ruling 6: at most three fix -> audit cycles in a round. Only the human raises it, per run.
const DEFAULT_CYCLES = 3
const MAX_CYCLES = 99
const MAX_ROUNDS = 99
// The values of the state document's `next` this script knows what to say about. Any other
// value — the script's own `unsettled` among them — goes back to the orchestrator with the
// state attached. The set grows in dev/stabilize-record, never here first.
const KNOWN_NEXT = ['fix', 'rule', 'close']
// What an agent leaves open is triaged like any finding (ruling 5), and what triage sends on
// is verified. A verifier leaves things open too, so the two alternate — this many times with
// verification, then once more to give every entry its row.
const VERIFY_PASSES = 3
const GIT_MODEL = 'sonnet'
// The one tool a git step runs a command of, and relays the line of.
const STEP_TOOL = 'dev/stabilize-step'
const RUNS_ROOT = 'completions/artifacts'
// Adjustment (ii): the paths the published crates carry, and their tests. They move only
// through an audited round's merge; every other path commits directly behind the gate. The
// acts that assert it are handed the list.
const PRODUCT_PATHS = ['Cargo.toml', 'Cargo.lock', 'crates']
// The append-only logs a merge may conflict in — milestone-build.js's list, held equal by
// tooling-tests/merge_logs_fence.rs. `dev/merge-logs` resolves exactly these; the two acts
// that merge are handed the list.
const SYNC_LOGS = ['DECISIONS.md', 'implementation/project-history.md']
// The lines the agent definitions' contracts bind on, spelled as they spell them.
const LABELS = { report: 'REPORT:', binary: 'BINARY:', area: 'AREA:', record: 'RECORD STEP', branch: 'BRANCH:' }
// Ruling 4's three: the dispositions that are the human's to give.
const HUMAN_RULINGS = ['admitted', 'bound', 'later']
const PAYLOAD_ENDS = 'STABILIZE_PAYLOAD'
// The name under which a triage is handed the rows of the ledger that still await it. No
// reporter can carry it: an item's reporters are `<item>-<step>`.
const LEDGER_SOURCE = 'ledger'

// ---- the roles: who is launched, and which labelled lines its definition binds on ----
// `drives` is a role that is handed the binary and returns the hash it asserted.
// `proposal` drives an advocate's proposal independently of the advocate. No definition
// fits that role (the entry of 2026-10-06 says what each lacks), so it has none, and its
// prompt is its whole contract. `crossModel` is the source pass by a model of another
// family: it has no definition either, and is launched only for an item args.crossModel names.
const ROLES = {
  preflight: { agentType: 'stabilize-preflight', labels: ['report'], drives: false },
  scope: { agentType: 'stabilize-scope', labels: ['report'], drives: false },
  review: { agentType: 'milestone-code-reviewer', labels: ['report'], drives: false },
  drive: { agentType: 'milestone-e2e-tester', labels: ['report', 'binary'], drives: true },
  triage: { agentType: 'finding-triage', labels: ['report'], drives: false },
  verify: { agentType: 'finding-verifier', labels: ['report', 'binary'], drives: true },
  advocate: { agentType: 'robust-advocate', labels: ['report', 'binary'], drives: true },
  proposal: { agentType: 'general-purpose', labels: [], drives: true },
  crossModel: { agentType: 'general-purpose', labels: [], drives: false },
  fixer: { agentType: 'build-fixer', labels: ['report', 'area', 'branch'], drives: false },
  record: { agentType: 'build-executor', labels: ['record', 'branch'], drives: false },
}

// ---- the chains: per KIND of instrument item, the fixed chain of agents that runs it ----
// A chain is a list of steps run in order; a step is the roles run in parallel. `as` names
// the reporter (`<item>-<as>`), `hands` the earlier steps whose reports the role is handed.
// An item whose kind has no chain here halts the stage and names the kind: the test set is
// data (dev/stabilize-record item-set), and a kind is added by adding its row here.
// `check` has no chain of its own: a deterministic check is run by the preflight.
const CHECK_KIND = 'check'
// The doctype a red deterministic check is filed under: the candidate fails a check.
const RED_DOCTYPE = 'jigc-feedback'
// The id under which the preflight returns whether the cross-model pass's tool answers.
const CROSS_CHECK = 'cross-model-tool'
const CHAINS = {
  'review-row': [
    [{ as: 'source', role: 'review', task: 'the SOURCE PASS of this review row' }, { as: 'driver', role: 'drive', task: 'DRIVE this review row\'s table' }, { as: 'crossmodel', role: 'crossModel', crossModel: true }],
    [{ as: 'reconciler', role: 'drive', task: 'RECONCILE this review row: the table another agent drove, against the source pass — a claim of one that the other cannot reproduce is a lead, and you drive every lead', hands: ['source', 'driver', 'crossmodel'] }],
  ],
  'trial-arm': [
    [{ as: 'rehearse', role: 'drive', task: 'REHEARSE this trial arm: prove its occasion fires, before the arm is run', image: true }],
    [{ as: 'run', role: 'drive', task: 'RUN this trial arm', hands: ['rehearse'], image: true }],
    [{ as: 'score', role: 'drive', task: 'SCORE this trial arm from the evidence its run left', hands: ['rehearse', 'run'], image: true }],
  ],
  'audit-area': [[{ as: 'review', role: 'review', task: 'the audit of this AREA' }]],
  'audit-cross-cutting': [[{ as: 'review', role: 'review', task: 'the CROSS-CUTTING pass of the audit' }]],
  'audit-drive': [[{ as: 'drive', role: 'drive', task: 'the audit\'s DRIVE: the doors of this unit, through the binary' }]],
}
// chainOf — a kind's chain as ONE item runs it: a step marked `crossModel` is in it only
// when the invocation named that item for the cross-model pass (and the pass can run), and
// nobody is handed the report of a step that is not.
function chainOf(kind, crossModel) {
  const chain = (CHAINS[kind] || []).map((stepList) => stepList.filter((s) => !s.crossModel || crossModel === true))
  const present = chain.reduce((all, stepList) => all.concat(stepList.map((s) => s.as)), [])
  return chain.map((stepList) => stepList.map((s) => (s.hands ? Object.assign({}, s, { hands: s.hands.filter((h) => present.includes(h)) }) : s)))
}
// The audit of a round's fix diff — not an item of the test set, the `fix` stage's own.
const FIX_AUDIT = [[{ as: 'review', role: 'review', task: 'the review of this round\'s FIX DIFF' }, { as: 'drive', role: 'drive', task: 'the drive of this round\'s FIX DIFF: every door it can change' }]]

// ---- pure helpers (everything from here to `selfTest` runs no agent) ----

// branchName — THE one place a branch name of a stabilization run is minted (ruling 7).
// `round` absent: the loop branch. With a round: its branch; `cut` above 1 is a part of the
// round re-cut as a tip of its own (ruling 6). `round` as '' gives the prefix every branch
// of every round shares, for the git steps that list them. The branch type of its own
// (`stabilize/<run>/main`, `stabilize/<run>/r<N>`) is this function's change and no other's.
function branchName(run, round, cut) {
  const loop = 'fix/' + run
  if (round == null) return loop
  return loop + '-r' + round + (cut > 1 ? '-part' + (cut - 1) : '')
}

function runDir(run) {
  return RUNS_ROOT + '/' + run
}

// sha256 — of a text's UTF-8 bytes, as `shasum -a 256` prints it. What an agent relays —
// the state document — and what an agent is told to write — a record step's payload — is
// held to a hash, so that a line retyped with one character changed is caught here and not
// three steps later.
function sha256(text) {
  const bytes = []
  for (let i = 0; i < text.length; i++) {
    let c = text.charCodeAt(i)
    if (c >= 0xd800 && c < 0xdc00 && i + 1 < text.length) c = 0x10000 + ((c - 0xd800) << 10) + (text.charCodeAt(++i) - 0xdc00)
    if (c < 0x80) bytes.push(c)
    else if (c < 0x800) bytes.push(0xc0 | (c >> 6), 0x80 | (c & 63))
    else if (c < 0x10000) bytes.push(0xe0 | (c >> 12), 0x80 | ((c >> 6) & 63), 0x80 | (c & 63))
    else bytes.push(0xf0 | (c >> 18), 0x80 | ((c >> 12) & 63), 0x80 | ((c >> 6) & 63), 0x80 | (c & 63))
  }
  const bits = bytes.length * 8
  bytes.push(0x80)
  while (bytes.length % 64 !== 56) bytes.push(0)
  for (let shift = 56; shift >= 0; shift -= 8) bytes.push(shift >= 32 ? Math.floor(bits / 4294967296) >>> (shift - 32) & 255 : bits >>> shift & 255)
  // The first 32 bits of the fractional parts of the square and cube roots of the primes.
  const h = []
  const k = []
  for (let n = 2; k.length < 64; n++) {
    let prime = true
    for (let d = 2; d * d <= n; d++) if (n % d === 0) prime = false
    if (!prime) continue
    if (h.length < 8) h.push(Math.floor((Math.sqrt(n) % 1) * 4294967296))
    k.push(Math.floor((Math.cbrt(n) % 1) * 4294967296))
  }
  const rotr = (x, n) => (x >>> n) | (x << (32 - n))
  for (let at = 0; at < bytes.length; at += 64) {
    const w = []
    for (let i = 0; i < 16; i++) w.push((bytes[at + 4 * i] << 24) | (bytes[at + 4 * i + 1] << 16) | (bytes[at + 4 * i + 2] << 8) | bytes[at + 4 * i + 3])
    for (let i = 16; i < 64; i++) {
      const s0 = rotr(w[i - 15], 7) ^ rotr(w[i - 15], 18) ^ (w[i - 15] >>> 3)
      const s1 = rotr(w[i - 2], 17) ^ rotr(w[i - 2], 19) ^ (w[i - 2] >>> 10)
      w.push((w[i - 16] + s0 + w[i - 7] + s1) | 0)
    }
    let [a0, b0, c0, d0, e0, f0, g0, h0] = h
    for (let i = 0; i < 64; i++) {
      const t1 = (h0 + (rotr(e0, 6) ^ rotr(e0, 11) ^ rotr(e0, 25)) + ((e0 & f0) ^ (~e0 & g0)) + k[i] + w[i]) | 0
      const t2 = ((rotr(a0, 2) ^ rotr(a0, 13) ^ rotr(a0, 22)) + ((a0 & b0) ^ (a0 & c0) ^ (b0 & c0))) | 0
      h0 = g0; g0 = f0; f0 = e0; e0 = (d0 + t1) | 0; d0 = c0; c0 = b0; b0 = a0; a0 = (t1 + t2) | 0
    }
    const add = [a0, b0, c0, d0, e0, f0, g0, h0]
    for (let i = 0; i < 8; i++) h[i] = (h[i] + add[i]) | 0
  }
  return h.map((x) => (x >>> 0).toString(16).padStart(8, '0')).join('')
}

function isText(value) {
  return typeof value === 'string' && value.trim() !== '' && !/[\r\n]/.test(value)
}
function isSlug(value) {
  return typeof value === 'string' && value.length <= SLUG_MAX && SLUG_RE.test(value)
}
function shq(text) {
  return "'" + String(text).replace(/'/g, "'\\''") + "'"
}
function plain(value) {
  return value && typeof value === 'object' && !Array.isArray(value)
}
function distinct(list) {
  return new Set(list).size === list.length
}

// validateRulings — the `rulings` argument, or why it is refused. A ruling is one of the
// human's three dispositions on a finding, or a declared bound; nothing else is one.
// runRuling — whether a ruling is about the RUN and not about a finding or a bound: the go
// after a stop, one more re-run of a clause, one more triage of a finding, one more attempt
// of a stage, and either bound raised.
const RUN_RULINGS = ['go', 'rerun', 'rounds', 'cycles', 'reverify', 'again']
function runRuling(r) {
  return plain(r) && RUN_RULINGS.some((name) => r[name] != null)
}
function validateRulings(rulings) {
  if (!Array.isArray(rulings) || rulings.length === 0) return 'args.rulings must be a non-empty list of rulings'
  const keys = []
  const about = []
  for (const r of rulings) {
    if (!plain(r)) return 'a ruling must be an object, not ' + JSON.stringify(r)
    if (runRuling(r)) {
      const named = Object.keys(r)
      if (named.length !== 1) return 'a ruling about the run names one thing — ' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + ' — and nothing beside it: ' + JSON.stringify(r)
      if (r.go != null && r.go !== true) return 'the human\'s go is `go` as true, or is not passed at all: ' + JSON.stringify(r)
      if (r.rerun != null && !isSlug(r.rerun)) return '`rerun` names the clause that is granted one more re-run, a slug as the clause table spells it: ' + JSON.stringify(r)
      if (r.rounds != null && !(Number.isInteger(r.rounds) && r.rounds >= 1 && r.rounds <= MAX_ROUNDS)) return '`rounds` is the bound across rounds, raised: a whole number from 1 to ' + MAX_ROUNDS + ', not ' + JSON.stringify(r.rounds)
      if (r.cycles != null && !(Number.isInteger(r.cycles) && r.cycles > DEFAULT_CYCLES && r.cycles <= MAX_CYCLES)) return '`cycles` is the bound on a round\'s fix cycles, raised: a whole number above ' + DEFAULT_CYCLES + ' and at most ' + MAX_CYCLES + ', not ' + JSON.stringify(r.cycles)
      if (r.reverify != null && !isSlug(r.reverify)) return '`reverify` names the finding that is granted one more triage, by its ledger key: ' + JSON.stringify(r)
      if (r.again != null && !STAGES.includes(r.again)) return '`again` names the stage that is granted one more attempt — ' + STAGES.join(' or ') + ': ' + JSON.stringify(r)
      about.push(named[0] + ':' + (r.rerun || r.reverify || r.again || ''))
      continue
    }
    const declares = r.bound != null
    if (r.key == null && !declares) return 'a ruling names a finding by `key`, or declares a bound by `bound`: ' + JSON.stringify(r)
    if (r.key != null) {
      if (!isSlug(r.key)) return 'a ruling\'s key ' + JSON.stringify(r.key) + ' is not a ledger key'
      if (!HUMAN_RULINGS.includes(r.ruling)) return 'ruling ' + JSON.stringify(r.ruling) + ' on ' + r.key + ' is not one of the human\'s: ' + HUMAN_RULINGS.join(', ')
      if (declares !== (r.ruling === 'bound')) return 'ruling on ' + r.key + ': `bound` names the declared bound of a `bound` ruling, and of no other'
      if (r.note != null && (declares || !isText(r.note))) return 'ruling on ' + r.key + ': `note` is one line of text, for `admitted` and `later`'
      keys.push(r.key)
    } else if (r.ruling != null || r.note != null) {
      return 'a declared bound without a finding carries no `ruling` and no `note`: ' + JSON.stringify(r)
    }
    if (declares) {
      if (!isSlug(r.bound)) return 'the bound ' + JSON.stringify(r.bound) + ' is not a slug'
      for (const cell of ['reach', 'where', 'pin']) if (!isText(r[cell])) return 'the bound ' + r.bound + ' needs `' + cell + '` as one line of text (`pin`: the test that pins it, or the word unpinned)'
    }
    const known = r.key != null ? (declares ? ['key', 'ruling', 'bound', 'reach', 'where', 'pin'] : ['key', 'ruling', 'note']) : ['bound', 'reach', 'where', 'pin']
    const unknown = Object.keys(r).filter((name) => !known.includes(name))
    if (unknown.length) return 'a ruling has no field ' + unknown.join(', ') + ': ' + JSON.stringify(r)
  }
  if (!distinct(keys)) return 'args.rulings rules on a finding more than once'
  if (!distinct(about)) return 'args.rulings says the same thing about the run more than once'
  if (about.length && about.length !== rulings.length) return 'args.rulings mixes rulings about the run (' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + ') with rulings on a finding or a bound: the first are recorded on the loop branch by an invocation that starts nothing, the others ride the round — two invocations'
  return null
}

// NOT_FIT — the stages that refuse to start, each with why. The build of this workflow was
// reviewed red; a stage is used on a real run only once the repair of its half and the
// re-review of that repair are recorded in DECISIONS.md, the `test` half first (the human's
// ruling of 2026-10-06 — DECISIONS.md, "The stabilization workflow's build, reviewed red",
// ruling 3). LIFTING A REFUSAL IS ONE EDIT: the stage's line is deleted from this table, by
// the commit that records that re-review under a DECISIONS.md heading with the words
// "the `<stage>` half of the stabilization workflow, repaired and re-reviewed" —
// tooling-tests/stabilize_harness_fence.rs, arm (m), takes the deletion in a tree that has
// that heading and in no other, and neither it nor the self-test is edited with it.
const BUILD_RECORD = 'completions/artifacts/M55/stabilization-build/README.md'
const NOT_FIT = {
  fix: 'its repair and the re-review of that repair are not recorded',
}
// notFit — why an invocation's stage refuses to start, or null. Asked before any agent runs.
// An invocation that only records what the human ruled ABOUT THE RUN (RUN_RULINGS) starts
// no stage — it runs two reads and the one record step, on the loop branch,
// exactly as the other stage's does — and is not refused. A ruling on a finding, or a
// declared bound, is the stage's own first step and is refused with it.
function notFit(a) {
  if (!NOT_FIT[a.stage]) return null
  if (a.rulings != null && a.rulings.every(runRuling)) return null
  return 'the `' + a.stage + '` stage is NOT FIT FOR USE and refuses to start: ' + NOT_FIT[a.stage] + '. The build of the stabilization workflow was reviewed red — its record is ' + BUILD_RECORD + ' — and a stage is used only once its half is repaired and re-reviewed. Nothing was run: no agent, no read. What is taken meanwhile: ' + STAGES.filter((stage) => !NOT_FIT[stage]).map((stage) => 'the `' + stage + '` stage').concat(['what the human rules about the run (args.rulings: ' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + '), which either stage records and which starts nothing']).join('; and ') + '. A ruling on a finding or a declared bound is this stage\'s own first step and waits with it — until then an invocation of `test` that carries only such rulings records them on the loop branch, and starts nothing'
}

// validateArgs — every refusal that precedes the first agent. Returns the message of the
// refusal, or null.
function validateArgs(a) {
  if (!plain(a)) return 'args must be an object: { stage, run, scratch, … }'
  const unknown = Object.keys(a).filter((name) => !ARGS.includes(name))
  if (unknown.length) return 'unknown arg(s) ' + unknown.join(', ') + ' — the args are ' + ARGS.join(', ')
  if (a.selfTest != null && typeof a.selfTest !== 'boolean') return 'args.selfTest must be true or false'
  if (a.selfTest) return null
  if (!STAGES.includes(a.stage)) return 'args.stage ' + JSON.stringify(a.stage) + ' is not one of ' + STAGES.join(', ')
  if (!isSlug(a.run)) return 'args.run ' + JSON.stringify(a.run) + ' is not a slug: lowercase a-z0-9 words joined by single dashes, at most ' + SLUG_MAX + ' characters — it names the run\'s directory and its branches'
  if (typeof a.scratch !== 'string' || !SCRATCH_RE.test(a.scratch) || /\/\.\.?(\/|$)/.test(a.scratch)) return 'args.scratch ' + JSON.stringify(a.scratch) + ' is not an absolute directory path of plain segments ([A-Za-z0-9._-]): every agent works under it, and it reaches a shell'
  if (a.model != null && (typeof a.model !== 'string' || !MODEL_RE.test(a.model))) return 'args.model ' + JSON.stringify(a.model) + ' is not a model name'
  if (a.stopAfter != null && !STOPS[a.stage].includes(a.stopAfter)) return 'args.stopAfter ' + JSON.stringify(a.stopAfter) + ' is not a step of `' + a.stage + '`: ' + STOPS[a.stage].join(', ')
  const only = (name, stage) => (a[name] != null && a.stage !== stage ? 'args.' + name + ' belongs to the `' + stage + '` stage' : null)
  const misplaced = only('scope', 'test') || only('clause', 'test') || only('crossModel', 'test') || only('raise', 'fix') || only('exit', 'fix')
  if (misplaced) return misplaced
  if (a.crossModel != null && !(Array.isArray(a.crossModel) && a.crossModel.length > 0 && a.crossModel.every(isSlug) && distinct(a.crossModel))) return 'args.crossModel names the items of the test set that get a cross-model source pass, one by one: [ \'<item>\', … ] — not ' + JSON.stringify(a.crossModel) + '. There is no value that turns it on for every item'
  if (a.clause != null) {
    if (!isSlug(a.clause)) return 'args.clause ' + JSON.stringify(a.clause) + ' is not a clause of the closing condition: a slug, as the clause table spells it'
    if (a.scope != null || a.crossModel != null) return 'args.clause runs the items of one clause again, inside the round that selected them and over that round\'s doors: it takes no args.scope and no args.crossModel'
  }
  if (a.scope != null) {
    const s = a.scope
    const named = plain(s) ? Object.keys(s) : []
    const ok = s === 'delta' || s === 'everything'
      || (named.length === 1 && named[0] === 'range' && typeof s.range === 'string' && RANGE_RE.test(s.range))
      || (named.length === 1 && named[0] === 'doors' && Array.isArray(s.doors) && s.doors.length > 0 && s.doors.every(isText) && distinct(s.doors))
    if (!ok) return 'args.scope must be \'delta\', \'everything\', { range: \'<sha>..<sha>\' } or { doors: [ … ] }, not ' + JSON.stringify(s)
  }
  if (a.rulings != null) {
    const why = validateRulings(a.rulings)
    if (why) return why
    const run = a.rulings.every(runRuling)
    if ((run || a.stage === 'test') && ['scope', 'clause', 'crossModel', 'raise', 'exit', 'stopAfter'].some((name) => a[name] != null)) return 'an invocation that only records rulings — about the run, on either stage; on a finding or a bound, on `test` — records them and starts nothing: it takes no scope, clause, crossModel, raise, exit or stopAfter — invoke the step the returned `next` names afterwards'
  }
  if (a.raise != null) {
    const named = plain(a.raise) ? Object.keys(a.raise) : []
    if (named.length !== 1 || named[0] !== 'cycles' || !Number.isInteger(a.raise.cycles) || a.raise.cycles <= DEFAULT_CYCLES || a.raise.cycles > MAX_CYCLES) return 'args.raise must be { cycles: N } with N above the bound of ' + DEFAULT_CYCLES + ' (and at most ' + MAX_CYCLES + '), not ' + JSON.stringify(a.raise)
  }
  if (a.exit != null) {
    const e = a.exit
    const part = plain(e) && Object.keys(e).length === 1 && Array.isArray(e.part) && e.part.length > 0 && e.part.every((s) => typeof s === 'string' && SHORT_SHA_RE.test(s)) && distinct(e.part)
    if (e !== 'drop' && !part) return 'args.exit must be \'drop\' or { part: [<the fix commits to keep>] }, not ' + JSON.stringify(e)
    if (a.raise != null) return 'args.exit and args.raise are two different exits at a bound: pass one'
  }
  return null
}

// reporterName — the name a reporter writes its report under: a slug, or null when the
// parts do not make one (the caller halts, naming them — never a truncated name).
function reporterName(parts) {
  const name = parts.join('-')
  return isSlug(name) ? name : null
}

// The reporters of one stage of one attempt: every name this script hands out, once.
function launcher(ctx) {
  const names = []
  return {
    names,
    add(parts) {
      const name = reporterName(parts)
      if (!name) throw new Error('no reporter name can be made of ' + JSON.stringify(parts) + ': it must be a slug of at most ' + SLUG_MAX + ' characters')
      if (names.includes(name)) throw new Error('the reporter ' + name + ' would be launched twice')
      names.push(name)
      return name
    },
    // A reporter that was launched and is known to have left no report, where that is
    // an answer and not a halt: the record is not held to it.
    drop(name) {
      names.splice(names.indexOf(name), 1)
    },
    line(name) {
      return reportLine(ctx, name)
    },
  }
}

function stageFlags(ctx) {
  return '--run ' + ctx.run + ' --round ' + ctx.round + ' --stage ' + ctx.stage + (ctx.stage === 'fix' ? ' --cycle ' + ctx.cycle : '')
}
function reportLine(ctx, name) {
  return LABELS.report + ' ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' — write your report through `dev/stabilize-record report` with exactly these flags, and `--scratch ' + ctx.scratch + '` (and one more `--scratch` for each scratch root of your own outside it).'
}
function binaryLine(built, withImage) {
  return LABELS.binary + ' candidate `' + built.candidate.binary + '` sha256 ' + built.candidate.sha256 + ' (commit ' + built.candidate.sha + ', label ' + built.candidate.label + ')'
    + (built.previous ? '; previous release `' + built.previous.binary + '` sha256 ' + built.previous.sha256 + ' (version ' + built.previous.version + ')' : '')
    + (withImage && built.image ? '; trial image `' + built.image.tag + '`' : '')
    + '. Return the sha256 you asserted for the candidate as `asserted_sha256`.'
}
function branchLine(branch, loop) {
  return LABELS.branch + ' ' + branch + (branch === loop ? '' : ' (forked from ' + loop + ')') + ', already checked out by the harness. Before you commit, `git branch --show-current` must print exactly `' + branch + '` — if it does not, commit nothing and halt. Never create, switch, merge or push a branch; the harness\'s git steps own them.'
}

// hashMismatch — ruling 11's comparison, made here and by no agent: the reporters that
// drove something and did not return the candidate's hash as the one they asserted.
function hashMismatch(expected, returns) {
  return returns.filter((r) => r.drives && r.result && r.result.status !== 'halted' && r.result.asserted_sha256 !== expected).map((r) => ({ reporter: r.name, asserted: r.result.asserted_sha256 == null ? null : String(r.result.asserted_sha256) }))
}

// outcomeOf — `next` exactly as the state document gives it, and whether this script knows
// the value. It knows nothing else about it — but `rule` is the human's step for a finding
// on the human's list (one that is still ungraded or unverified after its retry among them:
// its `why` says so), for a clause that is still not green after its one re-run, and for a
// stage whose attempts did not reach their record, so all three lists go back with it.
function outcomeOf(state) {
  const out = { next: state.next, known: KNOWN_NEXT.includes(state.next) }
  if (out.next === 'rule') out.rule = { findings: state.human_list || [], clauses: state.human_clauses || [], stages: state.human_stages || [] }
  return out
}

// refusalOf — the word the state's position refuses a stage with, as what the orchestrator
// does about it. The words are dev/stabilize-record's (its header: THE POSITION), and
// tooling-tests/stabilize_harness_fence.rs holds this table to them.
const REFUSALS = {
  'not-ready': 'the run\'s opening is not done, and no stage starts before it is — the state\'s `not_ready` names what it owes, each a fact of the run: the clauses of the closing condition, the stop mode, the bound across rounds, the previous release and the default scope (`dev/stabilize-record run-set`)',
  'no-round': 'there is no tested round to fix — the `test` stage comes first, and records its triage',
  'not-tested': 'the round\'s `test` stage has not reached its record — it is run again first, as the next attempt',
  'round-open': 'the round is tested and a finding of it is still open — run `fix`, record the human\'s rulings with it, or drop the round (args.exit = \'drop\'); where the state\'s `next` is `triage`, the stage whose position says `triage` finishes the round\'s triage first',
  'round-over': 'the round is over — the next stage is `test`',
  'cycle-bound': 'the bound on a round\'s fix cycles is spent — as many fix -> audit cycles are recorded in this round as it allows — and the next step is the human\'s: one more cycle is allowed by the raised bound, passed as a ruling about the run (args.rulings, `cycles`), which records it (`dev/stabilize-record run-set`) and starts nothing; the other exits are to rule on what is still open, or to leave the round',
  'attempts-spent': 'this stage has left reports and no record in this round as many times in a row as are tried without the human — the state\'s `human_stages` names it — and one more attempt is the human\'s to grant: pass it as a ruling about the run (args.rulings, `again`), which records it and starts nothing; what the earlier attempts halted on is in their returns and their reports',
  'round-bound': 'the bound across rounds is spent — as many rounds have a fix stage on record as it allows — and one more fix round is the human\'s to allow: pass the raised bound as a ruling about the run (args.rulings, `rounds`), which records it (`dev/stabilize-record run-set`) and starts nothing',
  'stopped': 'the run stops after every round, and the round is over: the next one waits for the human\'s go — pass it as a ruling about the run (args.rulings, `go`), which records it and starts nothing; the state\'s `stop.then` names the step that follows',
}
function refusalOf(at) {
  return REFUSALS[at.refused] || 'the record script refuses it with a word this script has no sentence for'
}

// runRulingsFault — why the state does not ask the human for what the rulings about the run
// say, or null. The record script refuses the same (its `round-set`: go, granted); asked
// here first, so that a ruling nobody was asked for costs no record step.
function runRulingsFault(rulings, state) {
  const facts = state.facts || {}
  for (const r of rulings) {
    if (r.go != null && !(state.next === 'stop' && state.stop && state.stop.why === 'every-round')) return 'the run is not stopped after a round for the human\'s go — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': a go is recorded for that stop and for no other state'
    if (r.rerun != null && !(state.human_clauses || []).some((c) => c.clause === r.rerun)) return 'the clause `' + r.rerun + '` is not the human\'s: one more re-run is granted to a clause that is still not green after its re-run — the state\'s `human_clauses` names ' + ((state.human_clauses || []).map((c) => c.clause).join(', ') || 'none')
    if (r.rounds != null && facts.rounds != null && r.rounds <= facts.rounds) return 'the bound across rounds is ' + facts.rounds + ': `rounds` raises it, and ' + r.rounds + ' does not'
    if (r.rounds != null && (state.rounds || []).length && !(state.not_ready || []).length && !(state.stop && state.stop.why === 'round-bound')) return 'the run is not stopped at its bound across rounds — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': once a round is begun that bound is raised at its own stop, and at no other state'
    if (r.cycles != null && !(state.next === 'stop' && state.stop && state.stop.why === 'cycle-bound')) return 'the run is not stopped at the bound on a round\'s fix cycles — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': that bound is raised at its own stop, and at no other state'
    if (r.cycles != null && facts.cycles != null && r.cycles <= facts.cycles) return 'the bound on a round\'s fix cycles is ' + facts.cycles + ': `cycles` raises it, and ' + r.cycles + ' does not'
    if (r.reverify != null && !(state.human_list || []).some((f) => f.key === r.reverify && /-after-retry$/.test(String(f.why)))) return 'the finding `' + r.reverify + '` is not the human\'s to grant one more triage: that is granted to a finding the state\'s `human_list` names as still ungraded or unverified after its retry — it names ' + ((state.human_list || []).filter((f) => /-after-retry$/.test(String(f.why))).map((f) => f.key).join(', ') || 'none')
    if (r.again != null && !(state.human_stages || []).some((h) => h.stage === r.again)) return 'the `' + r.again + '` stage is not the human\'s to grant one more attempt: that is granted to a stage the state\'s `human_stages` names — it names ' + ((state.human_stages || []).map((h) => h.stage).join(', ') || 'none')
  }
  return null
}

// findingRulingsFault — why rulings on findings and bounds cannot be recorded by an
// invocation that starts nothing, or null: a ruling names a row the ledger holds. Whether a
// ruling is the right one is the human's; the record script takes the three dispositions on
// any row (its header: WHAT THIS SCRIPT CANNOT KNOW).
function findingRulingsFault(rulings, state) {
  const held = (state.ledger || []).map((row) => row.key)
  const strangers = rulings.filter((r) => r.key != null && !held.includes(r.key)).map((r) => r.key)
  return strangers.length ? 'the ledger has no row ' + strangers.join(', ') + ': a ruling is on a finding the run has recorded' : null
}

// ledgerSource — the rows of the ledger whose triage is not finished (the state's
// `untriaged`: nobody graded them, or nobody verified them), as one more source EVERY stage's
// triage is handed beside its own reporters' findings. A row seeded at the opening, and an
// entry an earlier stage left without a verdict, are graded and verified by the next triage
// that runs — never left for a stage of their own, and never for the human as if verified.
function ledgerSource(run, state) {
  const rows = (state.untriaged || []).map((u) => Object.assign({ why: u.why }, (state.ledger || []).find((row) => row.key === u.key))).filter((row) => row.key)
  if (!rows.length) return []
  return [{ reporter: LEDGER_SOURCE, report: runDir(run) + '/ledger.md', findings: rows.map((row) => row.key + ' — a row the ledger holds under this key, ' + (row.why === 'ungraded' ? 'never graded' : 'graded ' + row.grade + ' and never verified') + ' · door: ' + row.door + ' · clause: ' + row.clause + ' · repro: ' + row.repro) }]
}

// previousOf — the release the run measures against, as its record names it: what the
// second binary is built from, and what "a regression of the run" is green on.
function previousOf(state) {
  const facts = state.facts || {}
  return { version: facts.previous, commit: facts['previous-commit'] }
}

// evidenceOf — per clause, what the record script derived for it: its status, the round and
// the commit of the latest run among its items, how many fix rounds behind the candidate
// that is, and where each item's own run stands. It goes back with `close`, so that the
// human closes with that number in front of them.
function evidenceOf(state) {
  return (state.clauses || []).map((c) => ({ clause: c.clause, status: c.status, round: c.round, commit: c.commit, behind: c.behind, items: (c.items || []).map((i) => ({ item: i.item, round: i.round, attempt: i.attempt, standing: i.standing, why: i.why })) }))
}

// readStep — a git step's return as the object its command printed. The relayed line must
// end with the sha256 of itself without that field, as the tool writes it (dev/stabilize-step:
// "The line"), parse as JSON and be the line of the act that was asked for; then it is what
// the step says, a refusal included (`status: 'halted'`, with the tool's halt report). A line
// that is not the command's is a halted step too, and `relay` says why. A step whose agent
// halted without a line is passed on as that halt.
const STEP_SHA_RE = /, "sha256": "([0-9a-f]{64})"\}$/
function readStep(r, act) {
  if (!r) return null
  if (r.status !== 'ran') return { status: 'halted', halt: r.halt || null }
  const lost = (why) => ({ status: 'halted', relay: why, halt: { root_cause: why, evidence: typeof r.line === 'string' ? r.line : '(the step returned no line)', tree_state: '(not read: the step\'s line was not the command\'s)', recommendation: 'read the tree and the branches before the step is asked for again: its command may have run' } })
  if (typeof r.line !== 'string') return lost('the step returned no line')
  const line = r.line.replace(/\n$/, '')
  const tail = STEP_SHA_RE.exec(line)
  if (!tail || sha256(line.slice(0, tail.index) + '}') !== tail[1]) return lost('the relayed line does not end with the sha256 of itself, as `' + STEP_TOOL + '` prints it: it was altered on its way, or is not the command\'s')
  let said = null
  try {
    said = JSON.parse(line)
  } catch (e) {
    return lost('the relayed line is not JSON: ' + ((e && e.message) || e))
  }
  if (!plain(said) || said.act !== act || typeof said.status !== 'string') return lost('the relayed line is not the line of `' + STEP_TOOL + ' ' + act + '`')
  return said
}

// resultRows — what a `test` stage records of its items: ONE RESULT PER ITEM it ran, and
// nothing about a clause — a clause's status is derived by the record script from these
// rows, and no code of this script composes one. `void` with its reason when the item did
// not run to its end; `red` for a deterministic check that is red — which the record script
// files as a finding in the same call, so every red brings what that ledger row needs: a
// doctype, the door it stands at (the check's own first door, or its name), and where the
// evidence lies; else `green`: the item ran, on this commit, over the round's doors. What a
// hunting item FOUND is the ledger's, and forbids closing there.
function resultRows(units) {
  return units.map((unit) => {
    if (unit.status === 'void') return { item: unit.item, outcome: 'void', reason: unit.reason || 'it did not run to its end' }
    if (unit.status === 'red') return { item: unit.item, outcome: 'red', doctype: RED_DOCTYPE, door: unit.door || 'the check `' + unit.item + '`', repro: unit.evidence || 'the check returned red, and no evidence beside it' }
    return { item: unit.item, outcome: 'green' }
  })
}

// classifyCommits — a round's commits, by what a carry-over may take: a commit confined to
// the run's directory is a record; one that touches nothing in it is a fix; one that does
// both, or neither, is neither — and stops the step that asked.
function classifyCommits(all, inside, outside) {
  const records = []
  const fixes = []
  const mixed = []
  for (const sha of all) {
    const i = inside.includes(sha)
    const o = outside.includes(sha)
    if (i && !o) records.push(sha)
    else if (o && !i) fixes.push(sha)
    else mixed.push(sha)
  }
  return { records, fixes, mixed }
}
function sameCommit(x, y) {
  return typeof x === 'string' && typeof y === 'string' && x.length >= 7 && y.length >= 7 && (x.startsWith(y) || y.startsWith(x))
}
// reopenPatches — the ledger patches of a dropped round, or of the fixes a part leaves out:
// a row whose `fixed` names one of `gone` is open again, because that commit never lands.
function reopenPatches(ledger, gone) {
  return ledger.filter((row) => row.disposition === 'fixed' && gone.some((sha) => sameCommit(sha, row.detail))).map((row) => ({ key: row.key, disposition: 'open' }))
}
// repointPatches — a part's kept fixes were cherry-picked, so their rows name the new commits.
function repointPatches(ledger, picked) {
  const out = []
  for (const row of ledger) {
    const pick = row.disposition === 'fixed' ? picked.find((p) => sameCommit(p.from, row.detail)) : null
    if (pick) out.push({ key: row.key, disposition: 'fixed', detail: pick.to })
  }
  return out
}

// areasOf — lever 8, how `fix` partitions: the blockers by the registry their door comes
// from in the round's scope, in the scope's order; a door the scope does not list is an
// area of its own. One fixer per area, serial.
function areasOf(blockers, doors) {
  const rows = doors ? doors.included.concat(doors.excluded) : []
  const areas = []
  for (const finding of blockers) {
    const at = rows.find((d) => d.door === finding.door)
    const registry = at ? at.registry : '(a door the round\'s scope does not list)'
    let area = areas.find((x) => x.registry === registry)
    if (!area) areas.push(area = { registry, findings: [] })
    area.findings.push(finding)
  }
  return areas.map((area, i) => Object.assign({ n: i + 1 }, area))
}

// currentCut — which of a round's branches is its branch now: the highest cut that exists.
function currentCut(run, round, local) {
  let cut = 0
  for (let c = 1; c <= MAX_CYCLES; c++) if (local.includes(branchName(run, round, c))) cut = c
  return cut
}

// ---- structured-output schemas ----
const HALT = {
  type: 'object',
  description: 'the halt report — fill every field, so that nobody needs your transcript',
  properties: {
    root_cause: { type: 'string' },
    evidence: { type: 'string', description: 'the command and its full output, or the refusal line verbatim' },
    tree_state: { type: 'string', description: '`git status`, `git branch --show-current`, and which commits landed' },
    recommendation: { type: 'string' },
  },
}
const STRINGS = { type: 'array', items: { type: 'string' } }
// What a git step returns: the ONE line its command printed. What the line holds is the
// tool's to say (dev/stabilize-step: its header); `readStep` reads it.
const STEP_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['ran', 'halted'] },
    halt: HALT,
    line: { type: 'string', description: 'the ONE line the command printed on stdout, whole and as printed: every character copied, nothing re-ordered, summarised, re-indented or re-escaped' },
  },
}
// The one step that is still a list of commands: the re-cut of a part.
const CARRY_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['carried', 'halted'] },
    halt: HALT,
    branch: { type: 'string' },
    head: { type: 'string' },
    remote_head: { type: 'string' },
    picked: { type: 'array', description: 'one entry per commit carried, in order', items: { type: 'object', required: ['from', 'to'], properties: { from: { type: 'string', description: 'the full sha the step named' }, to: { type: 'string', description: 'the full sha of the commit the cherry-pick made' } } } },
  },
}
const PREFLIGHT_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['ready', 'halted'] },
    halt: HALT,
    candidate: { type: 'object', properties: { label: { type: 'string' }, sha: { type: 'string' }, binary: { type: 'string' }, sha256: { type: 'string' }, version_string: { type: 'string' }, path_check: { type: 'string' } } },
    previous: { type: 'object', properties: { version: { type: 'string' }, binary: { type: 'string' }, sha256: { type: 'string' } } },
    image: { type: 'object', properties: { tag: { type: 'string' }, verified: { type: 'boolean' }, failed: STRINGS } },
    checks: { type: 'array', items: { type: 'object', required: ['check', 'status'], properties: { check: { type: 'string', description: 'the id of the item the prompt listed the check under' }, status: { type: 'string', enum: ['green', 'red', 'void'] }, commit: { type: 'string' }, evidence: { type: 'string' } } } },
    report: { type: 'string', description: 'the path `dev/stabilize-record report` printed' },
  },
}
const DOORS = { type: 'array', items: { type: 'object', required: ['door', 'registry', 'derivation'], properties: { door: { type: 'string' }, registry: { type: 'string' }, derivation: { type: 'string' } } } }
const SCOPE_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['written', 'stands', 'halted'] },
    halt: HALT,
    base: { type: 'string', description: 'the full sha the round\'s change is measured from' },
    tip: { type: 'string' },
    included: DOORS,
    excluded: DOORS,
    registries: { type: 'array', items: { type: 'object', properties: { registry: { type: 'string' }, read_at: { type: 'string' }, doors: { type: 'number' }, included: { type: 'number' }, excluded: { type: 'number' }, whole: { type: 'boolean' } } } },
    by_unit: { type: 'array', items: { type: 'object', properties: { unit: { type: 'string' }, doors: STRINGS } } },
    uncovered: STRINGS,
    reached_but_excluded: STRINGS,
    left_open: STRINGS,
    scope: { type: 'string' },
    report: { type: 'string' },
  },
}
const FINDING = {
  type: 'object',
  required: ['id', 'title', 'door', 'clause', 'repro'],
  properties: {
    id: { type: 'string', description: 'your own id for it, unique in your report' },
    title: { type: 'string' },
    severity: { type: 'string', description: 'information for triage, never a filter' },
    door: { type: 'string', description: 'in the exact words of the door list you were handed, or named plainly and marked unlisted' },
    clause: { type: 'string' },
    class: { type: 'string' },
    count_derivation: { type: 'string' },
    repro: { type: 'string', description: 'the heading of its repro block in your report' },
    lead: { type: 'boolean', description: 'true for what you noticed and did not pursue' },
  },
}
const UNIT_SCHEMA = {
  type: 'object',
  required: ['status', 'findings'],
  properties: {
    status: { type: 'string', enum: ['reported', 'halted'] },
    halt: HALT,
    findings: { type: 'array', items: FINDING },
    doors_affected: Object.assign({ description: 'when you were handed a fix diff: every door it can change' }, STRINGS),
    asserted_sha256: { type: 'string', description: 'when your prompt carries a binary line: the sha256 you asserted for the candidate before you drove it' },
    report: { type: 'string', description: 'the path `dev/stabilize-record report` printed' },
    summary: { type: 'string', description: '2-4 sentences' },
  },
}
const TRIAGE_SCHEMA = {
  type: 'object',
  required: ['status', 'entries', 'counts'],
  properties: {
    status: { type: 'string', enum: ['graded', 'halted'] },
    halt: HALT,
    entries: {
      type: 'array',
      items: {
        type: 'object',
        required: ['key', 'new', 'doctype', 'source', 'door', 'clause', 'repro', 'grade'],
        properties: {
          key: { type: 'string' },
          new: { type: 'boolean', description: 'true for a key you minted; false for a finding found again, whose row the ledger holds' },
          doctype: { type: 'string', enum: ['jigc-feedback', 'inconsistency'] },
          source: { type: 'string' },
          door: { type: 'string' },
          clause: { type: 'string' },
          repro: { type: 'string' },
          grade: { type: 'string', enum: ['breaks', 'unclear', 'no-break', 'out-of-scope', 'needs-bound'] },
          bound: { type: 'string', description: 'with out-of-scope only: the row of the declared-bounds list it cites' },
          why: { type: 'string' },
        },
      },
    },
    to_verify: { type: 'array', items: { type: 'object', required: ['key'], properties: { key: { type: 'string' }, redrive: { type: 'string', description: 'what the verifier must re-drive' } } } },
    counts: {
      type: 'object',
      required: ['findings_in', 'entries', 'merged'],
      properties: {
        findings_in: { type: 'array', items: { type: 'object', required: ['reporter', 'count'], properties: { reporter: { type: 'string' }, count: { type: 'number' } } } },
        entries: { type: 'number' },
        new: { type: 'number' },
        found_again: { type: 'number' },
        merged: { type: 'number', description: 'findings folded into another entry, each merge named in your report' },
      },
    },
    report: { type: 'string' },
  },
}
const VERIFY_SCHEMA = {
  type: 'object',
  required: ['status', 'key'],
  properties: {
    status: { type: 'string', enum: ['verified', 'halted'] },
    halt: HALT,
    key: { type: 'string' },
    verdict: { type: 'string', enum: ['confirmed', 'refuted'] },
    regression: { type: 'boolean', description: 'with confirmed only' },
    basis: { type: 'string' },
    contested: { type: 'boolean' },
    asserted_sha256: { type: 'string', description: 'the sha256 you asserted for the candidate before you drove it' },
    ran_on: { type: 'object', properties: { candidate: { type: 'string', description: 'the sha256 you asserted for the candidate' }, previous: { type: 'string', description: 'the sha256 you asserted for the previous release, where step 4 ran' } } },
    repro: { type: 'string' },
    pinnable: { type: 'boolean' },
    left_open: STRINGS,
    report: { type: 'string' },
  },
}
const DRIVEN = { type: 'array', items: { type: 'object', required: ['step', 'command', 'result'], properties: { step: { type: 'string' }, command: { type: 'string' }, result: { type: 'string' } } } }
const ADVOCATE_SCHEMA = {
  type: 'object',
  required: ['status', 'verdict', 'case'],
  properties: {
    status: { type: 'string', enum: ['argued', 'halted'] },
    halt: HALT,
    verdict: { type: 'string', enum: ['robust-now', 'cheap-cut-is-correct'] },
    case: { type: 'string', description: 'the case, whole: the robust position, the tells, the priced cost, the robust scope' },
    proposal: { type: 'string', description: 'what you propose the human accept, as a change somebody else could apply to a clone of the candidate\'s commit' },
    driven: DRIVEN,
    undriven: STRINGS,
    asserted_sha256: { type: 'string' },
    left_open: STRINGS,
    report: { type: 'string' },
  },
}
const PROPOSAL_SCHEMA = {
  type: 'object',
  required: ['status', 'holds', 'steps'],
  properties: {
    status: { type: 'string', enum: ['driven', 'halted'] },
    halt: HALT,
    holds: { type: 'boolean', description: 'true only when every next step ran as the proposal says it does' },
    steps: { type: 'array', items: { type: 'object', required: ['step', 'command', 'result', 'agrees'], properties: { step: { type: 'string' }, command: { type: 'string' }, result: { type: 'string' }, agrees: { type: 'boolean', description: 'whether what you observed is what the advocate reported for this step' } } } },
    undriven: STRINGS,
    asserted_sha256: { type: 'string' },
    left_open: STRINGS,
    report: { type: 'string' },
  },
}
const FIXER_SCHEMA = {
  type: 'object',
  required: ['status', 'entries'],
  properties: {
    status: { type: 'string', enum: ['worked', 'halted'] },
    halt: HALT,
    entries: {
      type: 'array',
      items: {
        type: 'object',
        required: ['key', 'status'],
        properties: {
          key: { type: 'string' },
          status: { type: 'string', enum: ['fixed', 'could-not-fix', 'halted'] },
          commit: { type: 'string', description: 'the full sha' },
          gate: { type: 'string', description: 'the totals line and the GATE: PASS of the gate that preceded that commit' },
          notes: { type: 'string', description: 'on could-not-fix: the trace showing the finding is not real, or what stood in the way' },
          class: { type: 'string' },
          count_derivation: { type: 'string' },
          doors_affected: STRINGS,
          left_open: STRINGS,
          halt: HALT,
        },
      },
    },
    report: { type: 'string' },
  },
}
// What the record step's executor returns: that the gate ran to its verdict, red or green.
// It makes no commit and returns no evidence of its own: the commit step reads the batch
// and the gate's output itself.
const RECORD_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['gated', 'halted'] },
    halt: HALT,
    gate: { type: 'string', description: 'the file the gate\'s whole output was kept in, as the prompt names it' },
  },
}

// ---- the git steps (build-git) — each ONE command of dev/stabilize-step, its line relayed ----
const STEP_RULES = 'Run exactly the ONE command below, from the repository\'s root, and nothing else: no command before it or after it, no branch, no commit, no file edit, no `git stash`, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. The command does the whole step and checks it. It prints ONE line of JSON on stdout — also when it refuses, which it says with an exit status that is not 0 and one line on stderr. A refusal is the step\'s answer: you never repair what it names, never run the command a second time, and never do by hand what it did not do.'
function stepPrompt(what, act, flags) {
  return [
    'GIT STEP — ' + what + '. ' + STEP_RULES,
    '1. `' + STEP_TOOL + ' ' + act + ' ' + flags + '`',
    '2. Report status = ran and line = the ONE line the command printed on stdout, WHOLE and AS PRINTED, whatever it exited with: every character copied, no key re-ordered or dropped, nothing summarised, no `\\u…` escape rewritten into its character. The harness holds the line to the hash it ends with, so a line that is not the command\'s is caught. Only when the command printed no such line: status = halted, and the halt report (root_cause, evidence = the command and everything it printed, tree_state = `git status` and `git branch --show-current`).',
  ].join('\n')
}
// A list the harness owns, as the flags that hand it to an act.
function listFlags(flag, values) {
  return values.map((value) => '--' + flag + ' ' + value).join(' ')
}
function gitStatePrompt(v) {
  return stepPrompt('the state of the stabilization run `' + v.run + '` before its `' + v.stage + '` stage: the branch, the tree, the pushed loop branch, and the path-class assert', 'git-state', '--stage ' + v.stage + ' --loop ' + branchName(v.run) + ' --rounds ' + branchName(v.run, '') + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS))
}
function statePrompt(v, tag) {
  return stepPrompt('read the record of the stabilization run `' + v.run + '` (read ' + tag + ')', 'state', '--run ' + v.run + ' --scratch ' + v.scratch + ' --tag ' + tag)
}
function checkReportsPrompt(ctx, names) {
  return stepPrompt('the reports of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', held to the reporters that were launched', 'check-reports', stageFlags(ctx) + ' --attempt ' + ctx.attempt + ' -- ' + names.join(' '))
}
function findRoundPrompt(v, round) {
  return stepPrompt('find the branch of round ' + round + ' of `' + v.run + '`', 'find-round', '--prefix ' + branchName(v.run, round))
}
function openRoundPrompt(v, round, branch) {
  return stepPrompt('the branch `' + branch + '` of round ' + round + ' of `' + v.run + '`: switch to it, or open it from `' + branchName(v.run) + '`', 'open-round', '--loop ' + branchName(v.run) + ' --branch ' + branch + ' --run-dir ' + runDir(v.run))
}
function pushPrompt(v, branch) {
  return stepPrompt('push `' + branch + '` of the stabilization run `' + v.run + '` by name', 'push', '--branch ' + branch)
}
// The RECORD step's commit: the applied batch committed as ONE commit, when the gate that
// ran on the tree with the records shows nothing red that the candidate's own gate did not.
// The act is handed how many calls and checks the batch was composed of, and refuses a
// batch that is another's.
function recordCommitPrompt(v, branch, gate, expect) {
  return stepPrompt('the ONE commit of a record step of `' + v.run + '` on `' + branch + '`: the applied batch, held to the gate that ran on it', 'record', '--branch ' + branch + ' --run-dir ' + runDir(v.run) + ' --gate ' + gate + ' --calls ' + expect.calls + ' --checks ' + expect.checks)
}
// The LAND step: a round's branch merged `--no-ff` into the loop branch and the loop branch
// pushed, as one act.
function landPrompt(v, round, branch) {
  const loop = branchName(v.run)
  return stepPrompt('land round ' + round + ' of `' + v.run + '`, ONE act: merge `' + branch + '` into `' + loop + '` with a merge commit, then push `' + loop + '` by name. The audit of its fix diff left nothing open', 'land', '--loop ' + loop + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + ' ' + listFlags('log', SYNC_LOGS))
}
function roundCommitsPrompt(v, round, branch) {
  return stepPrompt('the commits of round ' + round + ' of `' + v.run + '` (`' + branch + '`), and which of them touch the run\'s directory', 'round-commits', '--loop ' + branchName(v.run) + ' --branch ' + branch + ' --run-dir ' + runDir(v.run))
}
// The CARRY step: named commits cherry-picked, one by one. Onto the loop branch — a dropped
// round keeps its record, adjustment iv — it is the tool's act. Onto a part's own branch
// (ruling 6) it is NOT: that exit is ruled to be rebuilt as a revert, so its re-cut stays the
// list of commands it was, to the byte, until that rebuild deletes it.
const CARRY_RULES = 'Run exactly the commands below, in order, and nothing else: the only commits you make are the cherry-picks this step names, one by one. No other branch, no file edit, no `git stash`, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. Any check that fails, or any command that fails, is a HALT: stop, leave everything as the step says, and fill the halt report (root_cause = which check, evidence = the command and its output, tree_state = `git status` and `git branch --show-current`).'
function carryPrompt(v, round, shas, part) {
  const loop = branchName(v.run)
  if (!part) return stepPrompt('round ' + round + ' of `' + v.run + '` is dropped: carry exactly its record commits over to `' + loop + '`, so that the round keeps its record', 'carry', '--loop ' + loop + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + ' -- ' + shas.join(' '))
  return [
    'GIT STEP — re-cut a part of round ' + round + ' of `' + v.run + '` as a branch of its own, `' + part + '`, from the tip of `' + loop + '`: the fix commits the human keeps, and the round\'s record commits. ' + CARRY_RULES,
    '1. `git status --porcelain --untracked-files=all` must print nothing.',
    '2. `git rev-parse --verify --quiet refs/heads/' + part + '` must FAIL and `git ls-remote --exit-code --heads origin ' + part + '` must exit 2 — the part\'s branch does not exist yet. `git switch ' + loop + '`, then `git switch --no-track -c ' + part + ' ' + loop + '`; pre = `git rev-parse HEAD`.',
    '3. Each of these ' + shas.length + ' commit(s), in this order, with `git cherry-pick -x <sha>` — each must exit 0; one that stops is `git cherry-pick --abort`, then HALT:',
  ].concat(shas.map((sha) => '   - ' + sha)).concat([
    '4. `git rev-list --count pre..HEAD` must print ' + shas.length + '.',
    '5. `git push origin ' + part + '` — the branch named in full; a rejected push is a HALT, never forced. Then `git ls-remote --exit-code --heads origin ' + part + '` must report the sha `git rev-parse ' + part + '` prints.',
    '6. Report status = carried, branch = `git branch --show-current`, head = `git rev-parse HEAD`, remote_head, and picked: for each commit named in step 3, in order, `from` = that sha and `to` = the sha of the commit its cherry-pick made (`git log --reverse --format=%H pre..HEAD` prints them in the same order).',
  ]).join('\n')
}
// The close's SYNC step — the fixed step before any pull request to main
// (implementation/dev-workflow.md -> Before a pull request to main). Returned to the
// orchestrator with `next: 'close'`, never run here: the close's own acts come first.
function syncMainPrompt(run) {
  const m = branchName(run)
  return stepPrompt('merge `origin/main` into `' + m + '` before its pull request to `main` is opened (the close of the stabilization run `' + run + '`). Nothing is pushed and no pull request is opened: the gate runs on the merged tree first', 'sync-main', '--loop ' + m + ' ' + listFlags('log', SYNC_LOGS))
}

// ---- the agents' prompts — thin: the role and its contract live in the definition ----
function opening(v) {
  return 'The run\'s opening record is `' + runDir(v.run) + '/opening.md`; its state is `dev/stabilize-record state --run ' + v.run + '`.'
}
function preflightPrompt(ctx, launch, name, plan) {
  const steps = ['the environment asserts — `git status --porcelain` may show untracked files under `' + runDir(ctx.run) + '/` and nothing else: what the record script wrote and no record step has committed yet — reports, the round\'s scope, its journal']
  const asked = plan.crossModel ? crossModelAssert() : null
  if (plan.build) {
    steps.push('the build: the candidate\'s binary, from `git archive ' + plan.sha + '`, copied to `' + plan.binary + '`')
    steps.push('the previous release\'s binary: version `' + plan.previous.version + '`, built from commit ' + plan.previous.commit + ' (`git rev-parse --verify ' + plan.previous.commit + '^{commit}` must resolve it) with `git archive`, exactly as the candidate is built, at `' + ctx.scratch + '/bin/previous/jigc` — one that is there already, read-only, is kept once its `--version` prints that version. Both are facts of the run\'s record (`facts.previous`, `facts.previous-commit` of its state), and neither is read from prose')
  }
  if (plan.gate) steps.push('the candidate\'s full gate, ONCE: `dev/gate > ' + plan.gate + ' 2>&1` (`mkdir -p` its directory first) — in the background, waited for in this same turn in slices of under five minutes, until that file holds the gate\'s verdict line; never `--fast`, never `--quick`, never piped. It is a FACT about the candidate, not an assert: `GATE: FAIL` is an answer and never your halt. The stage\'s record step reads which steps and tests it shows red out of that file, and a record commit is held to exactly that list — so the file is the gate\'s whole output, unedited. A check below whose brief is the full gate is answered from this same run: the gate is not run twice')
  if (plan.image) steps.push('the trial image, built and verified from the candidate\'s commit')
  if (asked) steps.push(asked)
  if (plan.checks.length) steps.push('the deterministic checks, one per item below, each returned in `checks` under `check` = the item\'s id; a CI run still in progress is waited for at most 60 minutes:\n' + plan.checks.map((c) => '   - ' + c.item + ': ' + c.brief).join('\n'))
  return [
    'PREFLIGHT — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage. ' + opening(ctx),
    launch.line(name),
    'Candidate: label ' + plan.label + ', commit ' + plan.sha + ' — ' + (plan.tested ? 'the commit round ' + ctx.round + ' tested, an ancestor of' : 'the tip of') + ' `' + plan.branch + '`, which is checked out. Scratch root: `' + ctx.scratch + '`.',
    'The steps this call covers, in this order, and nothing else:',
  ].concat(steps.map((s, i) => (i + 1) + '. ' + s)).join('\n')
}
function scopeText(scope, fallback) {
  if (scope == null) return 'the run\'s default scope, a fact of its record: ' + (fallback === 'everything' ? 'everything — every door of every registry is inside' : 'the derived delta')
  if (typeof scope === 'string') return scope === 'everything' ? 'everything — set by the human: every door of every registry is inside' : 'the derived delta'
  return scope.range ? 'the commit range ' + scope.range + ' — set by the human' : 'the named doors, set by the human: ' + scope.doors.map((d) => '`' + d + '`').join(' · ')
}
function scopePrompt(ctx, launch, name, plan) {
  return [
    'SCOPE — stabilization run `' + ctx.run + '`, round ' + ctx.round + '. ' + opening(ctx),
    launch.line(name),
    'The round\'s change lies between: base = ' + (plan.base ? plan.base + ' (the candidate round ' + (ctx.round - 1) + ' tested)' : plan.previous.commit + ' (the previous release, ' + plan.previous.version + ', as the run\'s record names it)') + '; tip = ' + plan.sha + ' (label ' + plan.label + ').',
    'The scope this stage was started with: ' + scopeText(plan.scope, plan.fallback) + '.',
    plan.earlier ? 'The door lists of round ' + (ctx.round - 1) + '\'s fixers and fix-diff auditors — inputs, never the result: the `doors_affected` of every report under `' + runDir(ctx.run) + '/r' + (ctx.round - 1) + '/reports/fix/`, which this prompt hands you.' : 'There is no earlier round whose door lists could be an input.',
    'The units of the test set are the rows of `' + runDir(ctx.run) + '/test-set.md` (the state document\'s `items`).',
    'Write the round\'s scope with `dev/stabilize-record scope-set --run ' + ctx.run + ' --round ' + ctx.round + ' --scratch ' + ctx.scratch + '`, once.',
  ].join('\n')
}
function doorList(doors) {
  return doors.length ? doors.map((d) => '   - ' + d.door + '   [' + d.registry + ']').join('\n') : '   (none)'
}
// The doors of the round's test set a unit covers: its own doors, and those of its registries.
function unitDoors(item, included) {
  if (item.runs === 'every-candidate' && !item.doors.length && !item.registries.length) return included
  return included.filter((d) => item.doors.includes(d.door) || item.registries.includes(d.registry))
}
// forkNames / unitNames — the reporter names a fork and a list of units will ask for, or
// why one cannot be made: found before anything is launched, never inside a running chain.
function forkNames(tag, key) {
  const names = [['verify', tag, key], ['advocate', tag, key], ['proposal', tag, key]]
  return names.every((parts) => reporterName(parts)) ? names : null
}
function unitNames(units, taken) {
  const names = taken.slice()
  for (const unit of units) {
    for (const stepList of unit.chain) {
      for (const step of stepList) {
        const name = reporterName([unit.item, step.as])
        if (!name || names.includes(name)) return { fault: 'no reporter name of its own can be made for `' + unit.item + '`, ' + step.as + (name ? ' (`' + name + '` is taken)' : ' (not a slug of at most ' + SLUG_MAX + ' characters)') }
        names.push(name)
      }
    }
  }
  return { names }
}
function unitPrompt(ctx, launch, name, step, unit, built, handed) {
  const role = ROLES[step.role]
  const lines = [
    'STABILIZATION RUN `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage — ' + step.task + ': `' + unit.item + '`. ' + opening(ctx),
    launch.line(name),
  ]
  if (role.drives) lines.push(binaryLine(built, step.image))
  lines.push('Your brief: ' + unit.brief)
  lines.push('Candidate: label ' + built.candidate.label + ', commit ' + built.candidate.sha + '. Scratch root: `' + ctx.scratch + '` — mint your own directory under it.')
  if (unit.range) lines.push('The unit is the diff `' + unit.range + '` over the product paths (' + PRODUCT_PATHS.join(', ') + ').')
  lines.push('The door list — a finding names its door in these exact words:\n' + doorList(unit.doors))
  if (handed.length) lines.push('This prompt hands you these reports of the same item, and no other:\n' + handed.map((h) => '   - ' + h.as + ': ' + (h.report ? '`' + h.report + '`' : '(it left no report)')).join('\n'))
  return lines.join('\n')
}
function triagePrompt(ctx, launch, name, sources, pass) {
  const others = launch.names.filter((n) => n !== name)
  return [
    'TRIAGE — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage, pass ' + pass + '. ' + opening(ctx),
    launch.line(name),
    'The reporters this stage has launched so far, each with one report of attempt ' + ctx.attempt + ' — a report of the stage beside these is the partial stage you halt on: ' + (others.join(' · ') || '(none)') + '.' + (pass > 1 ? ' This is pass ' + pass + ': it grades only what the agents of the pass before left open; every other finding of the stage was graded then, and is not handed to you again.' : ''),
    'Grade every finding below — ' + sources.reduce((n, s) => n + s.findings.length, 0) + ' in all, from ' + sources.length + ' reporter(s). The reports are the files named; each list is that reporter\'s structured return, and an entry marked `left open` is what an agent left open, triaged like any finding.' + (sources.some((s) => s.reporter === LEDGER_SOURCE) ? ' The source `' + LEDGER_SOURCE + '` is no reporter of this stage: its entries are rows the ledger already holds, whose triage nobody finished. Grade each under its own key, found again — from the report its repro names, which this prompt hands you with it — and count the source in `findings_in` like any other.' : ''),
  ].concat(sources.map((s) => '- ' + s.reporter + ' — report: ' + (s.report ? '`' + s.report + '`' : '(none)') + ' — ' + s.findings.length + ' finding(s):\n' + s.findings.map((f) => '   - ' + f).join('\n'))).join('\n')
}
function verifyPrompt(ctx, launch, name, entry, redrive, built) {
  return [
    'VERIFY-REAL — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', ONE finding. ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false),
    'The finding: ledger key `' + entry.key + '` · door: ' + entry.door + ' · the clause it is said to break: ' + entry.clause + ' · triage\'s grade: ' + entry.grade + ' · its repro block: ' + entry.repro + '.',
    redrive ? 'What triage asks you to re-drive: ' + redrive : '',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].filter(Boolean).join('\n')
}
function advocatePrompt(ctx, launch, name, fork, built) {
  return [
    'A FORK of the stabilization run `' + ctx.run + '`, round ' + ctx.round + ' — finding `' + fork.key + '` (' + fork.kind + '). ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false),
    'The finding: door: ' + fork.door + ' · clause: ' + fork.clause + ' · repro: ' + fork.repro + '.',
    'What raised the fork: ' + fork.statement,
    'The cheap cut is what that statement would leave as it is; argue the robust case, and drive what you propose. Return `proposal` as a change somebody else can apply to a clone of commit ' + built.candidate.sha + ': another agent drives it after you, without your spike.',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].join('\n')
}
// The independent drive of an advocate's proposal. Its agent has no definition, so this
// prompt is its whole contract — and so it is long where the others are thin.
function proposalPrompt(ctx, name, fork, advocate, built) {
  return [
    'You drive a PROPOSAL you did not write. A fork of the stabilization run `' + ctx.run + '` (round ' + ctx.round + ', finding `' + fork.key + '`) goes to the human with an advocate\'s case; a proposal is a claim, and before the human sees it somebody other than its author runs it end to end — through every OTHER party\'s next step (implementation/milestone-planning-workflow.md -> Settle, the claim-driven rule). You are that somebody. Your brief is to REFUTE it: assume a step the advocate reports as passing does not pass.',
    'The proposal, as its author returned it: ' + (advocate.proposal || '(none returned — read it from the report)'),
    'The advocate\'s report, handed to you: ' + (advocate.report ? '`' + advocate.report + '`' : '(none)') + '. The steps it says it drove:\n' + ((advocate.driven || []).map((d) => '   - ' + d.step + ' — `' + d.command + '` — ' + d.result).join('\n') || '   (none)'),
    'HOW. In a directory of your own, minted with `mktemp -d` under `' + ctx.scratch + '`: clone commit ' + built.candidate.sha + ' (`git clone` this repository there and check that commit out, detached — never the working tree, never a branch of this repository), apply the proposal as written, build it there (`cargo build --release --locked`, a target directory of its own), and run every next step the proposal names, each as printed, each exit status read bare. Where the proposal names a step you cannot run, say so in `undriven`, with why. For the BEFORE of each step drive the candidate itself: `shasum -a 256 ' + built.candidate.binary + '` must print ' + built.candidate.sha256 + ', and you return that hash as `asserted_sha256`.',
    'WHAT YOU RETURN. One entry of `steps` per next step: the command, what it printed and exited, and `agrees` — whether that is what the advocate reported. `holds` is true only when every step ran as the proposal says. What you hit that is not this proposal goes in `left_open`.',
    'WHAT YOU MAY TOUCH. Nothing in this repository: no edit, no commit, no branch, no push, no install. One file reaches it — your report, written with `dev/stabilize-record report ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + '` as a single here-document whose last line is `<!-- end of report -->`; return the path it prints as `report`. A refusal about your text you repair in the text; any other refusal you return as your halt, verbatim. Never `rm -rf` a path built from variables, and never pipe a command whose exit status you read.',
  ].join('\n')
}
// The cross-model pass — everything this script says about it is in these three functions,
// and each is called only where the invocation's opt-in is tested
// (tooling-tests/stabilize_harness_fence.rs holds both).
function crossModelTool() {
  return 'codex'
}
function crossModelAssert() {
  return 'whether the cross-model pass\'s tool answers, returned in `checks` under `check` = `' + CROSS_CHECK + '`: green when `command -v ' + crossModelTool() + '` answers and `' + crossModelTool() + ' --version` exits 0, void when either does not — with what it printed. It is NOT one of the environment asserts: void is an answer, never a halt, and nothing is installed'
}
function crossModelPrompt(ctx, name, unit) {
  const tool = crossModelTool()
  return [
    'A CROSS-MODEL SOURCE PASS of review row `' + unit.item + '` — stabilization run `' + ctx.run + '`, round ' + ctx.round + '. This invocation named this item for it (args.crossModel): a cross-model second opinion is a human-approved suggestion, never auto-run (implementation/milestone-planning-workflow.md -> Review), and naming the item was the approval. You relay what a model of another family claims about this row\'s source; you judge none of it and drive none of it.',
    'HOW. `command -v ' + tool + '` must answer — if it does not, halt and say so: nothing is installed. In a directory of your own, minted with `mktemp -d` under `' + ctx.scratch + '`, write the pass\'s prompt to a file: the row\'s brief (' + unit.brief + ' — read it, and hand over what it says to review), the door list below, and the instruction to return each claim with its `file:line` and the door it stands at. Then run `' + tool + ' review - < <the prompt file> > <an output file> 2>&1`, its exit status read bare, and read the output file.',
    'WHAT ITS CLAIMS ARE. Leads, never findings: a claim one model makes that nobody reproduced is a lead, and the row\'s reconciler drives every one. Return each as a finding with `lead: true`, its door in the exact words of the list below (or named plainly and marked unlisted), the clause it would break as the claim reads, and `repro` = the heading you give the claim in your report. A pass that returned nothing usable is a halt, with its output as the evidence.',
    'The door list:\n' + doorList(unit.doors),
    'WHAT YOU MAY TOUCH. Nothing in this repository: no edit, no commit, no branch, no push, no install. One file reaches it — your report, the pass\'s output whole and your list of leads, written with `dev/stabilize-record report ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + '` as a single here-document whose last line is `<!-- end of report -->`; return the path it prints as `report`. A refusal about your text you repair in the text; any other refusal you return as your halt, verbatim. Never `rm -rf` a path built from variables, and never pipe a command whose exit status you read.',
  ].join('\n')
}
function fixerPrompt(ctx, launch, name, area, branch) {
  return [
    'FIX — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', cycle ' + ctx.cycle + '. ' + opening(ctx),
    launch.line(name),
    branchLine(branch, branchName(ctx.run)),
    LABELS.area + ' ' + area.registry + ' — ' + area.findings.length + ' finding(s), in this order, each by its ledger key:',
  ].concat(area.findings.map((f) => '- `' + f.key + '` · door: ' + f.door + ' · clause: ' + f.clause + ' · repro: ' + f.repro + (f.detail ? ' · the human\'s ruling: ' + f.detail : ''))).join('\n')
}

// A record step's payload: one line, written to a file of the step's own and held to its
// hash before the script reads it — so that a row is the row this script composed.
function payload(dir, name, value, lines) {
  const text = lines ? value.join('\n') : JSON.stringify(value)
  const file = dir + '/' + name
  return {
    file,
    write: 'Write ' + (lines ? 'these lines' : 'this ONE line') + ', exactly, to `' + file + '` (`mkdir -p ' + dir + '` first) — `cat > ' + file + ' <<\'' + PAYLOAD_ENDS + '\'`, the text, then `' + PAYLOAD_ENDS + '` on a line of its own:\n' + text + '\n   Then `shasum -a 256 ' + file + '` must print `' + sha256(text + '\n') + '` — if it does not, the text was altered on its way: write it again, and halt if it still differs.',
  }
}
// gateStep — the full gate of a record step, its whole output kept where the commit step
// reads it.
function gateStep(file) {
  return 'The FULL gate on the tree as it now stands, its WHOLE output kept: `dev/gate > ' + file + ' 2>&1` — in the background, waited for in this same turn in slices of under five minutes, until that file holds the gate\'s verdict line. Never `--fast`, never `--quick`, never piped, and the file is never edited.'
}
const RECORD_RETURNS = 'YOU MAKE NO COMMIT, and stage nothing: the harness\'s next step reads the gate\'s output, holds it to the gate of the round\'s candidate, and makes the one commit. So a RED gate is not your halt: return status = gated as soon as the file holds the gate\'s verdict line — `GATE: PASS` or `GATE: FAIL` alike — and gate = the file. A halt is a refusal of step 2, or a gate that printed no verdict line.'
// call — one call of a record's batch: a subcommand of the record script with its flags, as
// its argument list, and what it reads on stdin.
function call(flags, more, stdin) {
  const made = { argv: flags.split(' ').concat(more || []) }
  if (stdin !== undefined) made.stdin = JSON.stringify(stdin)
  return made
}
function isCheck(made) {
  return made.argv[0].startsWith('check-')
}
// recordPrompt — a record step: the calls as ONE batch, then the gate. Returns the prompt,
// the file the gate's output is kept in, and what the commit step is held to.
function recordPrompt(v, what, branch, dir, round, calls, subject) {
  const batch = payload(dir, 'batch.json', calls)
  const gate = dir + '/gate.txt'
  const text = [
    LABELS.record + ' — stabilization run `' + v.run + '`: ' + what + '.',
    branchLine(branch, branchName(v.run)),
    'Run these from the repository\'s root, in this order, each exactly as written and each exit status read bare. A refusal is a halt — its one line the evidence — and nothing it names is repaired by hand:',
    '1. ' + batch.write,
    '2. `dev/stabilize-record apply --run ' + v.run + (round ? ' --round ' + round : '') + ' --subject ' + shq('docs(record): ' + subject) + ' --scratch ' + v.scratch + ' < ' + batch.file + '` — the ' + calls.length + ' call(s) of this record as ONE batch: every table is written, or none is. It prints one line of JSON.',
    '3. ' + gateStep(gate),
    RECORD_RETURNS,
  ].join('\n')
  return { text, gate, expect: { calls: calls.length, checks: calls.filter(isCheck).length } }
}
// pendingRecordPrompt — the record step of a batch an earlier invocation applied and did not
// commit: the gate again, and nothing else.
function pendingRecordPrompt(v, branch, dir, pending) {
  const gate = dir + '/gate.txt'
  const text = [
    LABELS.record + ' — stabilization run `' + v.run + '`: a record step of an earlier invocation applied its batch — ' + pending.calls + ' call(s), `' + pending.subject + '` — and did not commit it. The batch is on disk and pending; you apply NOTHING and write no table.',
    branchLine(branch, branchName(v.run)),
    'Run this from the repository\'s root, exactly as written (`mkdir -p ' + dir + '` first):',
    '1. ' + gateStep(gate),
    RECORD_RETURNS,
  ].join('\n')
  return { text, gate, expect: { calls: pending.calls, checks: (pending.checks || []).length } }
}
// stageRecordCommands — a stage's record, as the calls of its ONE batch: the reports checked
// first, then the candidate's gate (a `test` stage's), then what each item did, then the
// rows, then the ledger checked. Every row is composed here, from structured returns. THE
// RESULTS COME BEFORE THE LEDGER'S ROWS: the record script takes a re-run only while its
// state asks for one, and a finding the re-run found would end that. THE DISPOSITIONS ARE WRITTEN
// BEFORE THE TRIAGE: a round's triage records the disposition a row carried when it was
// found, and a fix cycle's audit finds a finding AFTER that cycle's fix — written the other
// way round, a fix that did not hold would be read as a finding followed by its fix.
function stageRecordCommands(ctx, rec) {
  const calls = []
  if (rec.reporters.length) calls.push(call('check-reports ' + stageFlags(ctx) + ' --attempt ' + ctx.attempt + ' --', rec.reporters))
  if (rec.gate) calls.push(call('gate-set --run ' + ctx.run + ' --round ' + ctx.round + ' --commit ' + rec.gate.commit + ' --summary ' + rec.gate.file + ' --scratch ' + ctx.scratch))
  if (rec.results && rec.results.rows.length) calls.push(call('result-set --run ' + ctx.run + ' --round ' + ctx.round + ' --commit ' + rec.results.commit + ' --scratch ' + ctx.scratch, [], rec.results.rows))
  if (rec.rows.length) calls.push(call('ledger-add --run ' + ctx.run + ' --scratch ' + ctx.scratch, [], rec.rows))
  if (rec.patches.length) calls.push(call('ledger-set --run ' + ctx.run + ' --scratch ' + ctx.scratch, [], rec.patches))
  if (rec.triage.length) calls.push(call('triage-set --run ' + ctx.run + ' --round ' + ctx.round, [], rec.triage))
  if (rec.facts) calls.push(call('round-set --run ' + ctx.run + ' --round ' + ctx.round, [], rec.facts))
  if (rec.keys.length) calls.push(call('check-ledger --run ' + ctx.run + ' --', rec.keys))
  return calls
}
// rulingsRecordPrompt — THE one step through which the human's rulings reach the record
// (ruling 4): the three dispositions that are the human's, and the rows of the declared-
// bounds list. No other function of this script composes either call with those values.
function rulingsRecordPrompt(v, round, branch, rulings, ran, at) {
  const calls = []
  // What the human ruled about the run: the go after the stop that follows `round`; one
  // more re-run of a clause — a fact of every round that selected an item of it whose
  // re-run is spent; one more triage of a finding — a fact of the latest round, whose
  // record counts them; one more attempt of a stage — a fact of the round it is spent in;
  // and either bound, raised. The record script takes each only while the state asks for it.
  const about = rulings.filter((r) => runRuling(r))
  for (const r of about) {
    const facts = r.go != null ? [{ value: { go: true }, call: 'round-set --run ' + v.run + ' --round ' + round }]
      : r.rerun != null ? ran[r.rerun].map((spentIn) => ({ value: { granted: r.rerun }, call: 'round-set --run ' + v.run + ' --round ' + spentIn }))
        : r.reverify != null ? [{ value: { reverify: r.reverify }, call: 'round-set --run ' + v.run + ' --round ' + at.round }]
          : r.again != null ? [{ value: { again: r.again }, call: 'round-set --run ' + v.run + ' --round ' + at.stages[r.again] }]
            : r.cycles != null ? [{ value: { cycles: r.cycles }, call: 'run-set --run ' + v.run }]
              : [{ value: { rounds: r.rounds }, call: 'run-set --run ' + v.run }]
    for (const fact of facts) calls.push(call(fact.call, [], fact.value))
  }
  const bounds = rulings.filter((r) => r.bound != null)
  for (const r of bounds) calls.push(call('bound-set --run ' + v.run + ' --bound ' + r.bound + ' --scratch ' + v.scratch, ['--reach', r.reach, '--ruling', r.where, '--pin', r.pin]))
  const patches = rulings.filter((r) => r.key != null).map((r) => {
    const patch = { key: r.key, disposition: r.ruling }
    if (r.ruling === 'bound') patch.detail = r.reach
    else if (r.note != null) patch.detail = r.note
    return patch
  })
  if (patches.length) {
    calls.push(call('ledger-set --run ' + v.run + ' --scratch ' + v.scratch, [], patches))
    calls.push(call('check-ledger --run ' + v.run + ' --', patches.map((p2) => p2.key)))
  }
  return recordPrompt(v, 'the human\'s rulings — ' + patches.length + ' on a finding, ' + bounds.length + ' declared bound(s), ' + about.length + ' about the run. They are the human\'s, relayed: record them as given, and judge none of them', branch, v.scratch + '/record/rulings-r' + round, round, calls, v.run + ' r' + round + ' — the human\'s rulings')
}

// ---- self-test: this script's own logic, with no agent ----
function selfTest() {
  const failed = []
  let checks = 0
  const check = (name, ok) => {
    checks++
    if (!ok) failed.push(name)
  }
  const base = { stage: 'test', run: 'rc24-tier1', scratch: '/tmp/scratch-1' }
  const fix = { stage: 'fix', run: 'rc24-tier1', scratch: '/tmp/scratch-1' }
  const [admitted, bound, later] = HUMAN_RULINGS

  // The refusals that precede every agent.
  const refused = [
    ['no args', undefined], ['a string', 'test'], ['a list', []], ['empty args', {}],
    ['no stage', { run: 'r', scratch: '/s' }], ['a stage nobody has', Object.assign({}, base, { stage: 'both' })],
    ['no run', { stage: 'test', scratch: '/s' }], ['a run in upper case', Object.assign({}, base, { run: 'RC24' })],
    ['a run with a slash', Object.assign({}, base, { run: 'a/b' })], ['a run with a doubled dash', Object.assign({}, base, { run: 'a--b' })],
    ['a run of 101 characters', Object.assign({}, base, { run: 'a'.repeat(101) })],
    ['no scratch', { stage: 'test', run: 'r' }], ['a relative scratch', Object.assign({}, base, { scratch: 'tmp/x' })],
    ['a scratch with a space', Object.assign({}, base, { scratch: '/tmp/a b' })], ['a scratch with a quote', Object.assign({}, base, { scratch: '/tmp/a\'b' })],
    ['a scratch that climbs', Object.assign({}, base, { scratch: '/tmp/../etc' })], ['a scratch with a trailing slash', Object.assign({}, base, { scratch: '/tmp/x/' })],
    ['an arg nobody defined', Object.assign({}, base, { stopAfer: 'state' })], ['a selfTest that is no truth value', { selfTest: 'yes' }],
    ['a stop that is another stage\'s', Object.assign({}, base, { stopAfter: 'fixers' })], ['a model with a space', Object.assign({}, base, { model: 'opus 4' })],
    ['a scope on fix', Object.assign({}, fix, { scope: 'everything' })], ['a scope nobody defined', Object.assign({}, base, { scope: 'all' })],
    ['a range that is no range', Object.assign({}, base, { scope: { range: 'main..HEAD' } })], ['doors that are no list', Object.assign({}, base, { scope: { doors: 'jigc setup' } })],
    ['no door at all', Object.assign({}, base, { scope: { doors: [] } })], ['a door of two lines', Object.assign({}, base, { scope: { doors: ['a\nb'] } })],
    ['a finding\'s ruling on test beside a scope', Object.assign({}, base, { scope: 'everything', rulings: [{ key: 'f-1', ruling: later }] })], ['a declared bound on test beside a tuning stop', Object.assign({}, base, { stopAfter: 'state', rulings: [{ bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] })], ['a finding\'s ruling on test beside a clause', Object.assign({}, base, { clause: 'no-lost-files', rulings: [{ key: 'f-1', ruling: admitted }] })], ['a raise on test', Object.assign({}, base, { raise: { cycles: 5 } })],
    ['a go that is no truth', Object.assign({}, base, { rulings: [{ go: 'yes' }] })], ['a go taken back', Object.assign({}, fix, { rulings: [{ go: false }] })], ['a go said twice', Object.assign({}, base, { rulings: [{ go: true }, { go: true }] })],
    ['a go beside a finding\'s ruling', Object.assign({}, fix, { rulings: [{ go: true }, { key: 'f-1', ruling: later }] })], ['a go with a note', Object.assign({}, base, { rulings: [{ go: true, note: 'on' }] })], ['a go and a re-run in one entry', Object.assign({}, base, { rulings: [{ go: true, rerun: 'no-lost-files' }] })],
    ['a re-run of no clause', Object.assign({}, base, { rulings: [{ rerun: 'No lost files' }] })], ['a re-run granted twice', Object.assign({}, fix, { rulings: [{ rerun: 'no-lost-files' }, { rerun: 'no-lost-files' }] })],
    ['a bound that is no number', Object.assign({}, fix, { rulings: [{ rounds: '4' }] })], ['a bound of no round', Object.assign({}, base, { rulings: [{ rounds: 0 }] })], ['half a round', Object.assign({}, base, { rulings: [{ rounds: 3.5 }] })],
    ['a cycle bound that raises nothing', Object.assign({}, base, { rulings: [{ cycles: DEFAULT_CYCLES }] })], ['a cycle bound that is no number', Object.assign({}, fix, { rulings: [{ cycles: '4' }] })], ['one more triage of no finding', Object.assign({}, base, { rulings: [{ reverify: 'Not a key' }] })], ['one more triage granted twice', Object.assign({}, base, { rulings: [{ reverify: 'f-1' }, { reverify: 'f-1' }] })], ['one more attempt of no stage', Object.assign({}, base, { rulings: [{ again: 'triage' }] })], ['one more attempt beside a finding\'s ruling', Object.assign({}, base, { rulings: [{ again: 'test' }, { key: 'f-1', ruling: later }] })], ['one more triage and one more attempt in one entry', Object.assign({}, base, { rulings: [{ reverify: 'f-1', again: 'test' }] })],
    ['a go beside a scope', Object.assign({}, base, { scope: 'everything', rulings: [{ go: true }] })], ['a go beside a clause', Object.assign({}, base, { clause: 'no-lost-files', rulings: [{ go: true }] })], ['a raised bound beside an exit', Object.assign({}, fix, { exit: 'drop', rulings: [{ rounds: 4 }] })], ['a go beside a tuning stop', Object.assign({}, fix, { stopAfter: 'rulings', rulings: [{ go: true }] })],
    ['an exit on test', Object.assign({}, base, { exit: 'drop' })], ['no ruling at all', Object.assign({}, fix, { rulings: [] })],
    ['a ruling that is not the human\'s', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: 'fixed' }] })],
    ['a ruling that reopens', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: 'open' }] })],
    ['a ruling on no finding', Object.assign({}, fix, { rulings: [{ ruling: later }] })],
    ['a bound with no reach', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: bound, bound: 'b', where: 'stop 1', pin: 'unpinned' }] })],
    ['a bound with no word on where', Object.assign({}, fix, { rulings: [{ bound: 'b', reach: 'x', pin: 'unpinned' }] })],
    ['a bound with no pin', Object.assign({}, fix, { rulings: [{ bound: 'b', reach: 'x', where: 'stop 1' }] })],
    ['a bound cited by another ruling', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: later, bound: 'b', reach: 'x', where: 'y', pin: 'z' }] })],
    ['a bound ruling that names no bound', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: bound }] })],
    ['a note of two lines', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: admitted, note: 'a\nb' }] })],
    ['a finding ruled twice', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: later }, { key: 'f-1', ruling: admitted }] })],
    ['a ruling with a field nobody defined', Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: later, grade: 'refuted' }] })],
    ['a raise to the bound itself', Object.assign({}, fix, { raise: { cycles: DEFAULT_CYCLES } })], ['a raise that is no number', Object.assign({}, fix, { raise: { cycles: '5' } })],
    ['a raise without cycles', Object.assign({}, fix, { raise: 5 })], ['an exit nobody defined', Object.assign({}, fix, { exit: 'continue' })],
    ['a part of no commit', Object.assign({}, fix, { exit: { part: [] } })], ['a part that names a branch', Object.assign({}, fix, { exit: { part: ['HEAD~1'] } })],
    ['an exit beside a raise', Object.assign({}, fix, { exit: 'drop', raise: { cycles: 5 } })],
    ['a clause on fix', Object.assign({}, fix, { clause: 'no-lost-files' })], ['a clause that is no slug', Object.assign({}, base, { clause: 'No lost files' })],
    ['a clause beside a scope', Object.assign({}, base, { clause: 'no-lost-files', scope: 'everything' })], ['a clause beside a cross-model pass', Object.assign({}, base, { clause: 'no-lost-files', crossModel: ['row-3'] })],
    ['a cross-model pass on fix', Object.assign({}, fix, { crossModel: ['row-3'] })], ['a cross-model item that is no id', Object.assign({}, base, { crossModel: ['Row 3'] })],
    ['a cross-model item named twice', Object.assign({}, base, { crossModel: ['row-3', 'row-3'] })],
  ].concat([true, false, 'all', '*', 'every', 'row-3', 1, [], {}, { all: true }, ['*'], ['all rows'], [true]].map((every) => ['a cross-model pass for every item: ' + JSON.stringify(every), Object.assign({}, base, { crossModel: every })]))
  for (const [name, given] of refused) check('refused: ' + name, typeof validateArgs(given) === 'string')
  const taken = [
    base, fix, { selfTest: true }, Object.assign({}, base, { scope: 'everything' }), Object.assign({}, base, { scope: { range: 'abcdef1..1234567' } }),
    Object.assign({}, base, { scope: { doors: ['jigc setup', 'jigc doc show'] } }), Object.assign({}, base, { stopAfter: 'preflight', model: 'sonnet' }),
    Object.assign({}, base, { clause: 'no-lost-files' }), Object.assign({}, base, { crossModel: ['row-3'] }), Object.assign({}, base, { crossModel: ['row-3', 'row-7'] }),
    Object.assign({}, fix, { raise: { cycles: DEFAULT_CYCLES + 1 } }), Object.assign({}, fix, { exit: 'drop' }), Object.assign({}, fix, { exit: { part: ['abcdef1', '0123456789abcdef0123456789abcdef01234567'] } }),
    Object.assign({}, base, { rulings: [{ go: true }] }), Object.assign({}, fix, { rulings: [{ go: true }, { rerun: 'no-lost-files' }, { rerun: 'no-regression' }] }), Object.assign({}, fix, { rulings: [{ rounds: 4 }] }), Object.assign({}, base, { rulings: [{ rounds: 4 }, { rerun: 'no-lost-files' }] }), Object.assign({}, base, { rulings: [{ reverify: 'f-1' }, { reverify: 'f-2' }, { again: 'test' }] }), Object.assign({}, fix, { rulings: [{ cycles: 4 }, { again: 'fix' }] }),
    Object.assign({}, base, { rulings: [{ key: 'f-1', ruling: later }] }), Object.assign({}, base, { rulings: [{ key: 'f-1', ruling: admitted, note: 'n' }, { bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] }),
    Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: admitted, note: 'build the robust path' }, { key: 'f-2', ruling: later }, { key: 'f-3', ruling: bound, bound: 'non-jigc-writer', reach: 'races against a writer that is not jigc', where: 'the stop after round 1, item 3', pin: 'unpinned' }, { bound: 'planted-state', reach: 'a state nobody reaches', where: 'the stop after round 1', pin: 'flow12::planted' }] }),
  ]
  for (const given of taken) check('taken: ' + JSON.stringify(given), validateArgs(given) === null)

  // A stage that is not fit for use refuses to start — every invocation of it but the one
  // that only records what the human ruled about the run. Read off the table: a stage that
  // is not in it refuses nothing.
  const aboutTheRunOnly = (given) => given.rulings != null && given.rulings.every(runRuling)
  for (const given of taken.filter((t) => !t.selfTest)) {
    const refusedAsUnfit = !!NOT_FIT[given.stage] && !aboutTheRunOnly(given)
    check('a stage that is not fit for use refuses to start: ' + JSON.stringify(given), (typeof notFit(given) === 'string') === refusedAsUnfit)
  }
  for (const stage of Object.keys(NOT_FIT)) {
    const of = (more) => notFit(Object.assign({ stage, run: 'rc24-tier1', scratch: '/tmp/scratch-1' }, more))
    check('the `' + stage + '` stage is known, and says why it is not fit', STAGES.includes(stage) && isText(NOT_FIT[stage]))
    check('the `' + stage + '` stage refuses whatever it is asked for', [{}, { stopAfter: STOPS[stage][0] }, { model: 'sonnet' }, { rulings: [{ key: 'f-1', ruling: later }] }, { rulings: [{ bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] }].every((more) => isText(of(more))))
    check('the refusal of `' + stage + '` names the stage, the build\'s record and that nothing ran', of({}).includes('the `' + stage + '` stage is NOT FIT FOR USE') && of({}).includes(BUILD_RECORD) && of({}).includes('Nothing was run') && of({}).includes(NOT_FIT[stage]))
    check('what the human rules about the run is recorded while `' + stage + '` refuses', of({ rulings: [{ go: true }] }) === null && of({ rulings: [{ rounds: 4 }] }) === null && of({ rulings: [{ rerun: 'no-lost-files' }, { go: true }] }) === null)
  }
  check('an exit at a bound and a raised bound do not get past a `fix` stage that is not fit', !NOT_FIT.fix || [{ exit: 'drop' }, { exit: { part: ['abcdef1'] } }, { raise: { cycles: DEFAULT_CYCLES + 1 } }].every((more) => isText(notFit(Object.assign({}, fix, more)))))
  check('a stage the table does not name refuses nothing', STAGES.filter((stage) => !NOT_FIT[stage]).every((stage) => notFit({ stage, run: 'rc24-tier1', scratch: '/tmp/scratch-1' }) === null) && notFit({ stage: 'no-such-stage' }) === null)

  // The one function that mints a branch name.
  check('the loop branch', branchName('rc24-tier1') === 'fix/rc24-tier1')
  check('a round branch', branchName('rc24-tier1', 2) === 'fix/rc24-tier1-r2' && branchName('rc24-tier1', 2, 1) === 'fix/rc24-tier1-r2')
  check('a part', branchName('rc24-tier1', 2, 3) === 'fix/rc24-tier1-r2-part2')
  check('the prefix of every round', branchName('rc24-tier1', 1).startsWith(branchName('rc24-tier1', '')) && branchName('rc24-tier1', 12, 2).startsWith(branchName('rc24-tier1', '')))
  check('a round is not a prefix of the loop', !branchName('rc24-tier1').startsWith(branchName('rc24-tier1', '')))
  check('the current cut', currentCut('x', 1, []) === 0 && currentCut('x', 1, ['fix/x-r1']) === 1 && currentCut('x', 1, ['fix/x-r1', 'fix/x-r1-part1', 'fix/x-r11']) === 2 && currentCut('x', 2, ['fix/x-r1']) === 0)

  // The hash a relay and a payload are held to.
  check('sha256 of nothing', sha256('') === 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855')
  check('sha256 of abc', sha256('abc') === 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad')
  check('sha256 over two blocks', sha256('abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq') === '248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1')
  check('sha256 of a million a', sha256('a'.repeat(1000000)) === 'cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0')
  check('sha256 outside ASCII', sha256('\u00e9') === '4a99557e4033c3539de2eb65472017cad5f9557f7a0625a09f1c3f6e2ba69c4c' && sha256('\u2014 \u00e9 \ud83d\ude00') === 'ce9b24e5ffe38e22059636aef28334557e630cf2206d2af214d779e63f7b0b51')

  // The relay of a git step's line — the state document inside it — and of `next`.
  const doc = { run: 'rc24-tier1', opened: true, next: 'fix', position: { test: { round: 1, attempt: 1 } } }
  // A line as the tool prints it: the object, and last the sha256 of the object's own text.
  const printed = (said) => JSON.stringify(said).slice(0, -1) + ', "sha256": "' + sha256(JSON.stringify(said)) + '"}'
  const line = printed({ act: 'state', status: 'read', branch: 'fix/rc24-tier1', state: doc })
  const relay = (text, act) => readStep({ status: 'ran', line: text }, act || 'state')
  check('a relay that hashes', relay(line).status === 'read' && relay(line + '\n').state.next === 'fix' && relay(line).branch === 'fix/rc24-tier1' && !relay(line).relay)
  check('a relay with one character changed', relay(line.replace('"fix"', '"fox"')).status === 'halted' && isText(relay(line.replace('"fix"', '"fox"')).relay) && !!relay(line.replace('rc24-tier1', 'rc24-tier2')).relay)
  check('a relay with a key re-ordered', !!relay(printed({ status: 'read', act: 'state' }).replace('"status":"read","act":"state"', '"act":"state","status":"read"')).relay && !!relay(line.replace(/, "sha256": "[0-9a-f]{64}"\}$/, '}')).relay)
  check('a relay that is no JSON', !!relay('not json').relay && !!relay('not json, "sha256": "' + sha256('not json}') + '"}').relay)
  check('a relay that is another act\'s line, or no act\'s', !!relay(line, 'push').relay && !!relay(printed([1])).relay && !!relay(printed({ status: 'read' })).relay && !!relay(printed({ act: 'state' })).relay)
  check('no relay', readStep(null, 'state') === null && !!readStep({ status: 'ran' }, 'state').relay && readStep({ status: 'ran' }, 'state').status === 'halted')
  check('a step that halted without a line is passed on as its halt', readStep({ status: 'halted', halt: { root_cause: 'x' } }, 'state').halt.root_cause === 'x' && !readStep({ status: 'halted' }, 'state').relay && readStep({ status: 'halted' }, 'state').halt === null)
  const refusedLine = printed({ act: 'land', status: 'halted', refused: 'merge-differs', merge_commit: 'a'.repeat(40), halt: { root_cause: 'merge-differs: x', evidence: 'e', tree_state: 't', recommendation: 'r' } })
  check('a refusal is read as a halted step that says why, with what the act had established', relay(refusedLine, 'land').status === 'halted' && relay(refusedLine, 'land').halt.root_cause === 'merge-differs: x' && relay(refusedLine, 'land').merge_commit === 'a'.repeat(40) && !relay(refusedLine, 'land').relay)
  for (const next of ['fix', 'rule', 'close', 'unsettled', 'retest', 'stop', 'test', 'triage', 'not-ready', '', null, 7]) {
    const out = outcomeOf({ next })
    check('next relayed verbatim: ' + JSON.stringify(next), out.next === next && out.known === ['fix', 'rule', 'close'].includes(next))
  }
  const ruled = outcomeOf({ next: 'rule', human_list: [{ key: 'f-1', why: 'outside' }], human_clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }] })
  check('`rule` goes back with all three of the human\'s lists', JSON.stringify(ruled.rule) === JSON.stringify({ findings: [{ key: 'f-1', why: 'outside' }], clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }], stages: [] }) && JSON.stringify(outcomeOf({ next: 'rule', human_stages: [{ stage: 'test', round: 1 }] }).rule) === JSON.stringify({ findings: [], clauses: [], stages: [{ stage: 'test', round: 1 }] }) && outcomeOf({ next: 'fix', human_list: [{ key: 'f-1' }] }).rule === undefined)
  for (const word of ['not-ready', 'no-round', 'not-tested', 'round-open', 'round-over', 'round-bound', 'cycle-bound', 'attempts-spent', 'stopped']) check('a refusal the orchestrator can act on: ' + word, isText(refusalOf({ refused: word, round: 1 })) && !refusalOf({ refused: word, round: 1 }).includes('no sentence'))
  check('a refusal nobody defined is said to be one', refusalOf({ refused: 'round-closed', round: 1 }).includes('no sentence') && refusalOf({ refused: 'not-ready', round: null }).includes('run-set') && refusalOf({ refused: 'round-bound', round: 3 }).includes('run-set'))
  check('a refusal the human lifts names the ruling that lifts it', refusalOf({ refused: 'stopped', round: 1 }).includes('args.rulings, `go`') && refusalOf({ refused: 'round-bound', round: 3 }).includes('args.rulings, `rounds`') && refusalOf({ refused: 'cycle-bound', round: 1 }).includes('args.rulings, `cycles`') && refusalOf({ refused: 'attempts-spent', round: 1 }).includes('args.rulings, `again`') && refusalOf({ refused: 'round-open', round: 1 }).includes('`triage`'))

  // What the human rules about the run is taken only where the state asks for it.
  const stoppedState = { next: 'stop', stop: { why: 'every-round', round: 2, then: 'test' }, human_clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }], facts: { stop: 'every-round', rounds: 3 } }
  const boundState = { next: 'stop', stop: { why: 'round-bound', round: 4, then: 'fix' }, human_clauses: [], facts: { stop: 'at-the-bound', rounds: 3 } }
  check('a go is taken at the stop after a round', runRulingsFault([{ go: true }], stoppedState) === null && runRulingsFault([{ go: true }, { rerun: 'no-lost-files' }, { rounds: 4 }], stoppedState) === null)
  check('a go is taken at no other state', [boundState, { next: 'close', stop: null }, { next: 'fix', stop: null }, { next: 'stop', stop: null }, {}].every((state) => typeof runRulingsFault([{ go: true }], state) === 'string'))
  check('one more re-run is granted to a clause that is the human\'s, and to no other', typeof runRulingsFault([{ rerun: 'no-regression' }], stoppedState) === 'string' && typeof runRulingsFault([{ rerun: 'no-lost-files' }], boundState) === 'string' && typeof runRulingsFault([{ rerun: 'no-lost-files' }], {}) === 'string')
  check('the bound is raised, never lowered or said again', runRulingsFault([{ rounds: 4 }], boundState) === null && typeof runRulingsFault([{ rounds: 3 }], boundState) === 'string' && typeof runRulingsFault([{ rounds: 2 }], boundState) === 'string' && runRulingsFault([{ rounds: 1 }], { facts: { stop: 'every-round', rounds: null } }) === null)
  check('a ruling on a finding is recorded only on a row the ledger holds', findingRulingsFault([{ key: 'f-1', ruling: later }, { bound: 'b', reach: 'x', where: 'y', pin: 'z' }], { ledger: [{ key: 'f-1' }] }) === null && typeof findingRulingsFault([{ key: 'f-9', ruling: later }], { ledger: [{ key: 'f-1' }] }) === 'string' && typeof findingRulingsFault([{ key: 'f-1', ruling: later }], {}) === 'string')
  const cycleState = { next: 'stop', stop: { why: 'cycle-bound', round: 1, then: 'fix' }, facts: { stop: 'at-the-bound', rounds: 3, cycles: null } }
  const retriedState = { next: 'rule', stop: null, human_list: [{ key: 'f-1', why: 'unverified-after-retry' }, { key: 'f-2', why: 'outside' }], human_stages: [{ stage: 'test', round: 2 }] }
  const begun = { rounds: [{ round: 1 }], not_ready: [] }
  check('the bound across rounds is raised at its own stop once a round is begun, and written before that', runRulingsFault([{ rounds: 4 }], Object.assign({}, boundState, begun)) === null && typeof runRulingsFault([{ rounds: 4 }], Object.assign({}, stoppedState, begun)) === 'string' && typeof runRulingsFault([{ rounds: 4 }], Object.assign({ next: 'fix', stop: null, facts: { rounds: 3 } }, begun)) === 'string' && runRulingsFault([{ rounds: 4 }], { next: 'test', stop: null, facts: { rounds: 3 }, rounds: [], not_ready: [] }) === null)
  check('the bound on a round\'s cycles is raised at its own stop, and at no other', runRulingsFault([{ cycles: 4 }], cycleState) === null && typeof runRulingsFault([{ cycles: 4 }], boundState) === 'string' && typeof runRulingsFault([{ cycles: 4 }], stoppedState) === 'string' && typeof runRulingsFault([{ cycles: 4 }], Object.assign({}, cycleState, { facts: { cycles: 5 } })) === 'string')
  check('one more triage is granted to a finding that is the human\'s after its retry, and to no other', runRulingsFault([{ reverify: 'f-1' }], retriedState) === null && typeof runRulingsFault([{ reverify: 'f-2' }], retriedState) === 'string' && typeof runRulingsFault([{ reverify: 'f-1' }], stoppedState) === 'string' && typeof runRulingsFault([{ reverify: 'f-1' }], {}) === 'string')
  check('one more attempt is granted to a stage that is the human\'s, and to no other', runRulingsFault([{ again: 'test' }], retriedState) === null && typeof runRulingsFault([{ again: 'fix' }], retriedState) === 'string' && typeof runRulingsFault([{ again: 'test' }], stoppedState) === 'string')
  check('which rulings are about the run', runRuling({ reverify: 'f-1' }) && runRuling({ again: 'test' }) && runRuling({ cycles: 4 }) && runRuling({ go: true }) && runRuling({ rerun: 'x' }) && runRuling({ rounds: 4 }) && !runRuling({ key: 'f-1', ruling: later }) && !runRuling({ bound: 'b', reach: 'x', where: 'y', pin: 'z' }) && !runRuling(null) && !runRuling('go'))

  // Every triage is handed the rows of the ledger whose triage nobody finished.
  const awaiting = { untriaged: [{ key: 'seeded-1', why: 'ungraded' }, { key: 'f-9', why: 'unverified' }], ledger: [{ key: 'f-1', grade: 'confirmed', door: 'd', clause: 'c', repro: 'r' }, { key: 'seeded-1', grade: 'ungraded', door: 'jigc setup', clause: 'no-lost-files', repro: 'the opening record, row 3' }, { key: 'f-9', grade: 'unclear', door: 'jigc rename', clause: 'no-regression', repro: 'r1/reports/fix/c1/fix-area-1.a1.md, left open 2' }] }
  const handed = ledgerSource('rc24-tier1', awaiting)
  check('the rows whose triage is unfinished are one source, named for the ledger', handed.length === 1 && handed[0].reporter === LEDGER_SOURCE && handed[0].report === 'completions/artifacts/rc24-tier1/ledger.md' && handed[0].findings.length === 2)
  check('a row nobody graded and a row nobody verified are both handed over, by their keys', handed[0].findings[0].startsWith('seeded-1 — ') && handed[0].findings[0].includes('never graded') && handed[0].findings[0].includes('door: jigc setup') && handed[0].findings[1].startsWith('f-9 — ') && handed[0].findings[1].includes('graded unclear and never verified') && !handed[0].findings.join('\n').includes('f-1 '))
  check('nothing awaits triage: no source', ledgerSource('x', { untriaged: [], ledger: awaiting.ledger }).length === 0 && ledgerSource('x', {}).length === 0 && ledgerSource('x', { untriaged: [{ key: 'gone', why: 'ungraded' }], ledger: [] }).length === 0)
  check('no reporter can be named as the ledger is', !Object.keys(CHAINS).some((kind) => CHAINS[kind].some((stepList) => stepList.some((step) => reporterName(['x', step.as]) === LEDGER_SOURCE))) && !LEDGER_SOURCE.includes('-'))

  // The release a run measures against, and what a clause's row is worth, are read off the state.
  const facts = { stop: 'every-round', rounds: null, previous: '1.0.0-rc.24', 'previous-commit': 'e'.repeat(40), scope: 'delta' }
  check('the previous release is the record\'s', JSON.stringify(previousOf({ facts })) === JSON.stringify({ version: '1.0.0-rc.24', commit: 'e'.repeat(40) }))
  check('the evidence that goes back with close', JSON.stringify(evidenceOf({ clauses: [{ clause: 'no-lost-files', instrument: 'i', commit: 'c'.repeat(40), scope: 's', status: 'green', round: 2, behind: 1, retry: null, items: [{ item: 'gate', runs: 'every-candidate', round: 2, attempt: 1, commit: 'c'.repeat(40), outcome: 'green', reason: null, standing: 'green', why: null, rerun: null }, { item: 'row-a', runs: 'in-scope', round: null, attempt: null, commit: null, outcome: null, reason: null, standing: 'not-selected', why: null, rerun: null }] }] })) === JSON.stringify([{ clause: 'no-lost-files', status: 'green', round: 2, commit: 'c'.repeat(40), behind: 1, items: [{ item: 'gate', round: 2, attempt: 1, standing: 'green', why: null }, { item: 'row-a', round: null, attempt: null, standing: 'not-selected', why: null }] }]) && evidenceOf({}).length === 0)

  // Every prompt carries the labels its definition binds on, spelled as LABELS spells them.
  const ctx = { run: 'rc24-tier1', round: 2, stage: 'fix', cycle: 3, attempt: 4, scratch: '/tmp/scratch-1' }
  const built = { candidate: { label: 'c2', sha: 'c'.repeat(40), binary: '/tmp/scratch-1/bin/c2/jigc', sha256: 'b'.repeat(64) }, previous: { version: '1.0.0-rc.24', binary: '/tmp/scratch-1/bin/previous/jigc', sha256: 'a'.repeat(64) }, image: { tag: 'jigc-trial:c2', verified: true } }
  const launch = launcher(ctx)
  const carries = (text, label) => text.split('\n').some((l) => l.startsWith(LABELS[label]))
  const unit = { item: 'row-3', kind: 'review-row', clause: 'no-lost-files', brief: 'briefs/row-3.md', doors: [{ door: 'jigc setup', registry: 'the verb table' }] }
  const entry = { key: 'f-1', door: 'jigc setup', clause: 'no-lost-files', grade: 'breaks', repro: 'r2/reports/test/row-3-driver.a1.md, block 2' }
  const fork = { key: 'f-1', kind: 'contested', door: 'jigc setup', clause: 'no-lost-files', repro: 'x', statement: 'the verifier' }
  const prompts = {
    preflight: preflightPrompt(ctx, launch, launch.add(['preflight']), { build: true, image: true, sha: 'c'.repeat(40), label: 'c2', branch: 'fix/rc24-tier1', binary: '/tmp/scratch-1/bin/c2/jigc', checks: [{ item: 'the-gate', brief: 'the full gate' }], crossModel: false, previous: previousOf({ facts }) }),
    scope: scopePrompt(ctx, launch, launch.add(['scope']), { sha: 'c'.repeat(40), label: 'c2', base: 'd'.repeat(40), earlier: true, scope: { doors: ['jigc setup'] }, previous: previousOf({ facts }), fallback: facts.scope }),
    review: unitPrompt(ctx, launch, launch.add(['row-3', 'source']), CHAINS['review-row'][0][0], unit, built, []),
    drive: unitPrompt(ctx, launch, launch.add(['row-3', 'reconciler']), CHAINS['review-row'][1][0], unit, built, [{ as: 'source', report: 'a.md' }]),
    triage: triagePrompt(ctx, launch, launch.add(['triage', 'p1']), [{ reporter: 'row-3-source', report: 'a.md', findings: ['1 — x'] }], 1),
    verify: verifyPrompt(ctx, launch, launch.add(['verify', 'p1', 'f-1']), entry, 'the block', built),
    advocate: advocatePrompt(ctx, launch, launch.add(['advocate', 'f-1']), fork, built),
    proposal: proposalPrompt(ctx, launch.add(['proposal', 'f-1']), fork, { proposal: 'p', report: 'r.md', driven: [] }, built),
    crossModel: crossModelPrompt(ctx, launch.add(['row-3', 'crossmodel']), unit),
    fixer: fixerPrompt(ctx, launch, launch.add(['fix', 'area', '1']), { n: 1, registry: 'the verb table', findings: [entry] }, 'fix/rc24-tier1-r2'),
    record: recordPrompt(ctx, 'x', 'fix/rc24-tier1-r2', '/tmp/scratch-1/record/x', 2, stageRecordCommands(ctx, { reporters: launch.names, rows: [], triage: [], patches: [], clauses: [], facts: { cycles: 3 }, keys: ['f-1'] }), 's').text,
  }
  check('a prompt for every role', Object.keys(ROLES).every((role) => typeof prompts[role] === 'string') && Object.keys(prompts).length === Object.keys(ROLES).length)
  for (const role of Object.keys(ROLES)) {
    for (const label of Object.keys(LABELS)) check('the ' + role + ' prompt and ' + LABELS[label], carries(prompts[role], label) === ROLES[role].labels.includes(label))
  }
  check('a record step opens with its label', prompts.record.startsWith(LABELS.record) && rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text.startsWith(LABELS.record) && pendingRecordPrompt(fix, 'fix/rc24-tier1', '/tmp/scratch-1/record/pending-r1', { calls: 3, checks: [{}], subject: 's', round: 1 }).text.startsWith(LABELS.record))
  check('the preflight is handed the previous release as data', prompts.preflight.includes('version `1.0.0-rc.24`') && prompts.preflight.includes('built from commit ' + 'e'.repeat(40)) && prompts.preflight.includes('the tip of `fix/rc24-tier1`'))
  check('a round\'s base is the earlier candidate, or the previous release\'s commit', prompts.scope.includes('base = ' + 'd'.repeat(40)) && scopePrompt(ctx, launcher(ctx), 'scope', { sha: 'c'.repeat(40), label: 'c1', base: null, earlier: false, scope: null, previous: previousOf({ facts }), fallback: 'everything' }).includes('base = ' + 'e'.repeat(40) + ' (the previous release, 1.0.0-rc.24'))
  check('a triage is told what the ledger\'s rows are', triagePrompt(ctx, launcher(ctx), 'triage-p1', handed, 1).includes('The source `' + LEDGER_SOURCE + '` is no reporter') && !prompts.triage.includes('is no reporter'))
  check('a finishing preflight builds the commit the round tested', preflightPrompt(ctx, launcher(ctx), 'preflight', { build: true, image: false, sha: 'c'.repeat(40), label: 'c2', branch: 'fix/rc24-tier1', binary: 'x', checks: [], crossModel: false, previous: previousOf({ facts }), tested: true }).includes('the commit round 2 tested, an ancestor of `fix/rc24-tier1`'))
  check('a report line gives the flags', launch.line('scope') === reportLine(ctx, 'scope') && reportLine(ctx, 'scope').startsWith('REPORT: --run rc24-tier1 --round 2 --stage fix --cycle 3 --reporter scope --attempt 4 ') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').startsWith('REPORT: --run rc24-tier1 --round 2 --stage test --reporter x --attempt 4 '))
  check('a binary line gives the path and the hash', binaryLine(built, true).startsWith('BINARY: candidate `/tmp/scratch-1/bin/c2/jigc` sha256 ' + 'b'.repeat(64)) && binaryLine(built, true).includes('jigc-trial:c2') && !binaryLine(built, false).includes('jigc-trial:c2'))
  check('every chain is staffed from the roles', Object.keys(CHAINS).concat(['fix-diff']).every((kind) => (CHAINS[kind] || FIX_AUDIT).every((stepList) => stepList.every((s) => ROLES[s.role] && isSlug(s.as) && !s.as.includes('-') && (s.hands || []).every((h) => (CHAINS[kind] || FIX_AUDIT).some((earlier) => earlier.some((e) => e.as === h)))))))

  // The cross-model pass runs for the items an invocation names, and for no other: an
  // item that is not named has no such step, nobody is handed its report, and no prompt
  // names its tool. (That no argument value names every item is among the refusals above.)
  const names = new RegExp(crossModelTool(), 'i')
  const asOf = (chain) => chain.reduce((all, stepList) => all.concat(stepList.map((s) => s.as + (s.hands ? '<' + s.hands.join('+') : ''))), []).join(' ')
  for (const kind of Object.keys(CHAINS)) {
    for (const given of [undefined, null, false, 'true', 1, [], ['row-3'], 'all']) check('no cross-model step in ' + kind + ' under ' + JSON.stringify(given), chainOf(kind, given).every((stepList) => stepList.every((s) => !s.crossModel && ROLES[s.role] !== ROLES.crossModel && !(s.hands || []).includes('crossmodel'))))
  }
  check('the review row without the opt-in, and with it', asOf(chainOf('review-row', false)) === 'source driver reconciler<source+driver' && asOf(chainOf('review-row', true)) === 'source driver crossmodel reconciler<source+driver+crossmodel' && chainOf('no-such-kind', true).length === 0)
  // Every git step is ONE command of the tool, and its prompt is that command and "relay
  // its line" — all but the re-cut of a part, which is the list of commands it was.
  const steps = { 'git-state': gitStatePrompt(base), state: statePrompt(base, 't'), 'find-round': findRoundPrompt(fix, 1), 'open-round': openRoundPrompt(fix, 1, 'fix/rc24-tier1-r1'), push: pushPrompt(fix, 'fix/rc24-tier1-r1'), land: landPrompt(fix, 1, 'fix/rc24-tier1-r1'), 'round-commits': roundCommitsPrompt(fix, 1, 'fix/rc24-tier1-r1'), carry: carryPrompt(fix, 1, ['a'.repeat(40)], null), 'sync-main': syncMainPrompt('rc24-tier1'), 'check-reports': checkReportsPrompt(ctx, ['x']), record: recordCommitPrompt(fix, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }) }
  const recut = carryPrompt(fix, 1, ['a'.repeat(40)], 'fix/rc24-tier1-r1-part1')
  for (const act of Object.keys(steps)) {
    const numbered = steps[act].split('\n')
    check('the `' + act + '` step is one command of the tool, and its line relayed', numbered.length === 3 && numbered[0].startsWith('GIT STEP — ') && numbered[0].endsWith(STEP_RULES) && numbered[1].startsWith('1. `' + STEP_TOOL + ' ' + act + ' ') && numbered[1].endsWith('`') && numbered[2].startsWith('2. Report status = ran and line = ') && !/`git (?!stash|status|branch --show-current)/.test(steps[act]))
  }
  check('the names a step hands the tool are the harness\'s', steps['git-state'].includes('`' + STEP_TOOL + ' git-state --stage test --loop fix/rc24-tier1 --rounds fix/rc24-tier1-r --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates`') && steps.land.includes(' land --loop fix/rc24-tier1 --branch fix/rc24-tier1-r1 --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --log DECISIONS.md --log implementation/project-history.md`') && steps.carry.endsWith(STEP_RULES + '\n1. `' + STEP_TOOL + ' carry --loop fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates -- ' + 'a'.repeat(40) + '`\n' + steps.carry.split('\n')[2]) && steps['sync-main'].includes(' sync-main --loop fix/rc24-tier1 --log DECISIONS.md --log implementation/project-history.md`') && steps.state.includes(' state --run rc24-tier1 --scratch /tmp/scratch-1 --tag t`') && steps['find-round'].includes(' find-round --prefix fix/rc24-tier1-r1`') && steps.push.includes(' push --branch fix/rc24-tier1-r1`') && steps.record.includes(' record --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --gate /tmp/scratch-1/record/x/gate.txt --calls 3 --checks 2`'))
  check('the re-cut of a part is still the list of commands it was', recut.startsWith('GIT STEP — re-cut a part of round 1 of `rc24-tier1` as a branch of its own, `fix/rc24-tier1-r1-part1`') && recut.includes(CARRY_RULES) && recut.includes('`git cherry-pick -x <sha>`') && recut.includes('`git push origin fix/rc24-tier1-r1-part1`') && !recut.includes(STEP_TOOL) && recut.split('\n').length === 8)
  const gitPrompts = Object.keys(steps).map((act) => steps[act]).concat([recut])
  check('no prompt names the tool without the opt-in', Object.keys(prompts).filter((role) => names.test(prompts[role])).join(' ') === 'crossModel' && !names.test(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text) && !names.test(gitPrompts.join('\n')))
  const asserts = (crossModel) => preflightPrompt(ctx, launcher(ctx), 'preflight', { build: false, image: false, sha: 'c'.repeat(40), label: 'c2', branch: 'b', binary: 'x', checks: [], crossModel })
  check('the preflight is asked about the tool only when an item is named', names.test(asserts(true)) && asserts(true).includes('`' + CROSS_CHECK + '`') && !names.test(asserts(false)) && !names.test(asserts(undefined)) && !asserts(false).includes(CROSS_CHECK))
  const dropping = launcher(ctx)
  dropping.add(['row-3', 'source'])
  dropping.drop(dropping.add(['row-3', 'crossmodel']))
  check('a cross-model reporter that left no report is not held to one', dropping.names.join(' ') === 'row-3-source')
  check('a round with no scope of its own takes the run\'s default, as its record names it', scopeText(undefined, 'delta').endsWith('the derived delta') && scopeText(null, 'everything').includes('everything') && scopeText('delta', 'everything') === 'the derived delta')
  check('a driving chain role is handed the binary', unitPrompt(ctx, launcher(ctx), 'x', CHAINS['trial-arm'][1][0], unit, built, []).includes('jigc-trial:c2'))

  // The reporters: every name once, and the check names every one of them.
  check('the reporter list', launch.names.join(' ') === 'preflight scope row-3-source row-3-reconciler triage-p1 verify-p1-f-1 advocate-f-1 proposal-f-1 row-3-crossmodel fix-area-1')
  let twice = false
  try { launch.add(['scope']) } catch (e) { twice = true }
  let tooLong = false
  try { launch.add(['verify', 'p1', 'k'.repeat(100)]) } catch (e) { tooLong = true }
  check('a reporter launched twice, or with no name', twice && tooLong && launch.names.length === 10 && reporterName(['a', 'B']) === null)
  // A record step: its calls as ONE batch, the gate, and no commit of the executor's.
  const batchOf = (text) => JSON.parse(text.split('\n').find((l) => l.startsWith('[{"argv"')))
  const spelled = (made) => made.argv.join(' ')
  const recordCalls = batchOf(prompts.record)
  check('the check names every reporter launched', launch.names.every((n) => checkReportsPrompt(ctx, launch.names).includes(' ' + n)) && checkReportsPrompt(ctx, launch.names).includes('`' + STEP_TOOL + ' check-reports --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 -- ' + launch.names.join(' ') + '`') && spelled(recordCalls[0]) === 'check-reports --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 -- ' + launch.names.join(' '))
  check('the ledger check names every key', spelled(recordCalls[recordCalls.length - 1]) === 'check-ledger --run rc24-tier1 -- f-1')
  check('a record\'s calls are one payload, held to its hash', prompts.record.includes(sha256(JSON.stringify(recordCalls) + '\n')) && prompts.record.includes('`cat > /tmp/scratch-1/record/x/batch.json <<\'' + PAYLOAD_ENDS + '\'`') && spelled(recordCalls[1]) === 'round-set --run rc24-tier1 --round 2' && recordCalls[1].stdin === '{"cycles":3}' && recordCalls.length === 3)
  check('a record step applies its calls as one batch, runs the full gate, and makes no commit', prompts.record.includes('\n2. `dev/stabilize-record apply --run rc24-tier1 --round 2 --subject \'docs(record): s\' --scratch /tmp/scratch-1 < /tmp/scratch-1/record/x/batch.json`') && prompts.record.includes('\n3. The FULL gate') && prompts.record.includes('`dev/gate > /tmp/scratch-1/record/x/gate.txt 2>&1`') && prompts.record.endsWith(RECORD_RETURNS) && !/`git (add|commit)/.test(prompts.record) && prompts.record.includes('Never `--fast`, never `--quick`'))
  const stageRecord = recordPrompt(ctx, 'x', 'fix/rc24-tier1', '/tmp/scratch-1/record/z', 0, stageRecordCommands(Object.assign({}, ctx, { stage: 'test' }), { reporters: ['a'], gate: { commit: 'c'.repeat(40), file: '/tmp/scratch-1/gate/c2.a4.txt' }, results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }, { item: 'it\'s-b', status: 'void', reason: 'it\'s chain died' }]) }, rows: [], triage: [], patches: [], facts: { candidate: 'c'.repeat(40) }, keys: [] }), 's')
  check('what the commit step is held to is what was composed', JSON.stringify(stageRecord.expect) === JSON.stringify({ calls: 4, checks: 1 }) && stageRecord.gate === '/tmp/scratch-1/record/z/gate.txt' && !stageRecord.text.includes(' --round 0') && JSON.stringify(recordPrompt(ctx, 'x', 'b', '/d', 2, recordCalls, 's').expect) === JSON.stringify({ calls: 3, checks: 2 }))
  check('a test stage\'s record puts the candidate\'s gate on record, from the file the preflight kept', spelled(batchOf(stageRecord.text)[1]) === 'gate-set --run rc24-tier1 --round 2 --commit ' + 'c'.repeat(40) + ' --summary /tmp/scratch-1/gate/c2.a4.txt --scratch /tmp/scratch-1' && !recordCalls.some((made) => made.argv[0] === 'gate-set'))
  check('what each item did is one call, its rows data whatever they hold', spelled(batchOf(stageRecord.text)[2]) === 'result-set --run rc24-tier1 --round 2 --commit ' + 'c'.repeat(40) + ' --scratch /tmp/scratch-1' && batchOf(stageRecord.text)[2].stdin === JSON.stringify([{ item: 'a', outcome: 'green' }, { item: 'it\'s-b', outcome: 'void', reason: 'it\'s chain died' }]))
  const resumed = pendingRecordPrompt(fix, 'fix/rc24-tier1', '/tmp/scratch-1/record/pending-r1', { calls: 5, checks: [{ ok: true }, { ok: true }], subject: 'docs(record): x', round: 1 })
  check('a pending batch is gated again and nothing is applied', !resumed.text.includes('stabilize-record apply') && !resumed.text.includes(PAYLOAD_ENDS) && resumed.text.includes('`dev/gate > /tmp/scratch-1/record/pending-r1/gate.txt 2>&1`') && resumed.text.endsWith(RECORD_RETURNS) && JSON.stringify(resumed.expect) === JSON.stringify({ calls: 5, checks: 2 }))
  const cycleRecord = stageRecordCommands(ctx, { reporters: [], rows: [{ key: 'f-2' }], triage: [{ key: 'f-1', grade: 'breaks' }], patches: [{ key: 'f-1', disposition: 'fixed', detail: 'c'.repeat(40) }], clauses: [], facts: null, keys: ['f-1', 'f-2'] }).map((made) => made.argv[0])
  const at = (name) => cycleRecord.indexOf(name)
  check('a cycle\'s fixes are on the rows before its audit\'s triage reads them', at('ledger-add') >= 0 && at('ledger-add') < at('ledger-set') && at('ledger-set') < at('triage-set') && at('triage-set') < at('check-ledger'))

  // What is accepted as recorded: the commit step's own line, with one result per check.
  const held = { status: 'recorded', commit: 'a'.repeat(40), gate: { check: 'gate', ok: true, new: [] }, applied: { calls: 4, checks: [{ check: 'reports', ok: true }, { check: 'ledger', ok: true }] } }
  const composed = { calls: 4, checks: 2 }
  const lacking = (change) => recordFault(Object.assign({}, held, change), composed)
  check('a record whose line holds is recorded', recordFault(held, composed) === null)
  check('a record step that returns nothing, or halts, is not recorded', isText(recordFault(null, composed)) && recordFault({ status: 'halted', halt: { root_cause: 'gate-red: x' } }, composed) === 'gate-red: x' && isText(recordFault({ status: 'halted' }, composed)))
  check('a record without a commit, or without a gate check that holds, is not recorded', isText(lacking({ commit: 'abc' })) && isText(lacking({ gate: undefined })) && isText(lacking({ gate: { ok: false, new: ['test x'] } })) && isText(lacking({ gate: 'GATE: PASS' })))
  check('a record that returns no evidence of its checks is not recorded', isText(lacking({ applied: undefined })) && isText(lacking({ applied: { calls: 4 } })) && isText(lacking({ applied: { calls: 4, checks: [] } })) && isText(lacking({ applied: { calls: 4, checks: [{ check: 'reports', ok: true }] } })) && isText(lacking({ applied: { calls: 3, checks: held.applied.checks } })))
  check('a record one of whose checks does not hold is not recorded', isText(lacking({ applied: { calls: 4, checks: [{ check: 'reports', ok: true }, { check: 'ledger', ok: false }] } })) && isText(lacking({ applied: { calls: 4, checks: [{ check: 'reports', ok: true }, 'ok'] } })))

  // The candidate's gate is run by the first preflight of a `test` stage, and by no other.
  const gated = (gate) => preflightPrompt(ctx, launcher(ctx), 'preflight', { build: false, image: false, gate, sha: 'c'.repeat(40), label: 'c2', branch: 'b', binary: 'x', checks: [], crossModel: false })
  check('the preflight keeps the candidate\'s gate where the record step reads it', gated('/tmp/scratch-1/gate/c2.a4.txt').includes('`dev/gate > /tmp/scratch-1/gate/c2.a4.txt 2>&1`') && gated('/tmp/scratch-1/gate/c2.a4.txt').includes('never your halt') && !gated(undefined).includes('dev/gate') && !prompts.preflight.includes('dev/gate >'))

  // Ruling 11's comparison.
  const good = { status: 'reported', asserted_sha256: 'b'.repeat(64) }
  check('the hash comparison', hashMismatch('b'.repeat(64), [{ name: 'a', drives: true, result: good }, { name: 'b', drives: false, result: { status: 'reported' } }, { name: 'c', drives: true, result: { status: 'halted' } }, { name: 'd', drives: true, result: null }]).length === 0)
  check('a hash that differs, and one never returned', JSON.stringify(hashMismatch('b'.repeat(64), [{ name: 'a', drives: true, result: { status: 'reported', asserted_sha256: 'c'.repeat(64) } }, { name: 'b', drives: true, result: { status: 'reported' } }, { name: 'c', drives: true, result: good }])) === JSON.stringify([{ reporter: 'a', asserted: 'c'.repeat(64) }, { reporter: 'b', asserted: null }]))

  // The human's rulings enter in one step.
  const rulingsCalls = batchOf(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text)
  const rulingsText = rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text
  const aboutTheRun = rulingsRecordPrompt(base, 2, 'fix/rc24-tier1', [{ go: true }, { rerun: 'no-lost-files' }, { rounds: 4 }], { 'no-lost-files': [1, 2] })
  const aboutCalls = batchOf(aboutTheRun.text)
  check('the rulings step writes the go, the granted re-run — in every round an item of the clause is spent in — and the raised bound', JSON.stringify(aboutCalls) === JSON.stringify([{ argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"go":true}' }, { argv: ['round-set', '--run', 'rc24-tier1', '--round', '1'], stdin: '{"granted":"no-lost-files"}' }, { argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"granted":"no-lost-files"}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"rounds":4}' }]) && aboutTheRun.text.includes(' apply --run rc24-tier1 --round 2 --subject '))
  const granted = batchOf(rulingsRecordPrompt(base, 3, 'fix/rc24-tier1', [{ reverify: 'f-1' }, { again: 'test' }, { cycles: 4 }], {}, { round: 2, stages: { test: 1 } }).text)
  check('the rulings step writes one more triage in the latest round, one more attempt in the round the stage is spent in, and the raised cycle bound', JSON.stringify(granted) === JSON.stringify([{ argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"reverify":"f-1"}' }, { argv: ['round-set', '--run', 'rc24-tier1', '--round', '1'], stdin: '{"again":"test"}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"cycles":4}' }]))
  check('a ruling about the run writes no bound and no disposition', !aboutCalls.some((made) => ['bound-set', 'ledger-set'].includes(made.argv[0])) && aboutTheRun.text.includes('3 about the run') && rulingsText.includes('0 about the run') && !rulingsCalls.some((made) => ['round-set', 'run-set'].includes(made.argv[0])) && JSON.stringify(aboutTheRun.expect) === JSON.stringify({ calls: 4, checks: 0 }))
  check('the rulings step writes the bounds and the dispositions', JSON.stringify(rulingsCalls[0].argv) === JSON.stringify(['bound-set', '--run', 'rc24-tier1', '--bound', 'non-jigc-writer', '--scratch', '/tmp/scratch-1', '--reach', 'races against a writer that is not jigc', '--ruling', 'the stop after round 1, item 3', '--pin', 'unpinned']) && spelled(rulingsCalls[1]).startsWith('bound-set --run rc24-tier1 --bound planted-state ') && rulingsCalls[2].stdin === JSON.stringify([{ key: 'f-1', disposition: admitted, detail: 'build the robust path' }, { key: 'f-2', disposition: later }, { key: 'f-3', disposition: bound, detail: 'races against a writer that is not jigc' }]) && spelled(rulingsCalls[3]) === 'check-ledger --run rc24-tier1 -- f-1 f-2 f-3' && rulingsCalls.length === 4)
  check('a quote in a ruling cannot leave its argument', shq('it\'s a "bound" $(x) `y`') === '\'it\'\\\'\'s a "bound" $(x) `y`\'')

  // What a test stage records of its items: one result each, and nothing about a clause.
  const results = (units) => JSON.stringify(resultRows(units))
  check('a result per item, as it ran', results([{ item: 'a', status: 'green', reason: 'r' }, { item: 'b', status: 'green' }]) === JSON.stringify([{ item: 'a', outcome: 'green' }, { item: 'b', outcome: 'green' }]) && results([]) === '[]')
  check('a void item says why', results([{ item: 'a', status: 'void', reason: 'a step of its chain did not report' }, { item: 'b', status: 'void' }]) === JSON.stringify([{ item: 'a', outcome: 'void', reason: 'a step of its chain did not report' }, { item: 'b', outcome: 'void', reason: 'it did not run to its end' }]))
  check('a red check brings what its finding needs', results([{ item: 'gate', status: 'red', reason: 'r', door: 'jigc setup', evidence: 'two tests red' }, { item: 'ci', status: 'red', evidence: null }]) === JSON.stringify([{ item: 'gate', outcome: 'red', doctype: 'jigc-feedback', door: 'jigc setup', repro: 'two tests red' }, { item: 'ci', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `ci`', repro: 'the check returned red, and no evidence beside it' }]))
  check('no result names a clause, and no call sets one', !resultRows([{ item: 'a', clause: 'x', status: 'green' }]).some((row) => 'clause' in row) && !stageRecordCommands(ctx, { reporters: [], results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }]) }, rows: [], triage: [], patches: [], facts: null, keys: [] }).some((made) => made.argv[0].startsWith('clause')))
  const reRecord = stageRecordCommands(Object.assign({}, ctx, { stage: 'test' }), { reporters: ['a'], gate: null, results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }]) }, rows: [{ key: 'f-1' }], triage: [{ key: 'f-1', grade: 'breaks' }], patches: [], facts: null, keys: ['f-1'] })
  check('a re-run\'s record is its results before anything of the ledger, and no fact of the round', JSON.stringify(reRecord.map((made) => made.argv[0])) === JSON.stringify(['check-reports', 'result-set', 'ledger-add', 'triage-set', 'check-ledger']))

  // A dropped round, and a part.
  const classes = classifyCommits(['a1', 'b2', 'c3', 'd4', 'e5'], ['b2', 'd4', 'e5'], ['a1', 'c3', 'e5'])
  check('a round\'s commits by class', JSON.stringify(classes) === JSON.stringify({ records: ['b2', 'd4'], fixes: ['a1', 'c3'], mixed: ['e5'] }) && classifyCommits(['z9'], [], []).mixed.length === 1)
  const ledger = [{ key: 'f-1', disposition: 'fixed', detail: 'aaaaaaa1' }, { key: 'f-2', disposition: 'fixed', detail: 'bbbbbbb2' + '0'.repeat(32) }, { key: 'f-3', disposition: 'open', detail: null }, { key: 'f-4', disposition: 'fixed', detail: 'ccccccc3' }]
  check('a dropped round\'s fixes are open again', JSON.stringify(reopenPatches(ledger, ['aaaaaaa1' + '0'.repeat(32), 'bbbbbbb2' + '0'.repeat(32)])) === JSON.stringify([{ key: 'f-1', disposition: 'open' }, { key: 'f-2', disposition: 'open' }]) && reopenPatches(ledger, []).length === 0)
  check('a part\'s kept fixes name their new commits', JSON.stringify(repointPatches(ledger, [{ from: 'aaaaaaa1' + '0'.repeat(32), to: 'd'.repeat(40) }])) === JSON.stringify([{ key: 'f-1', disposition: 'fixed', detail: 'd'.repeat(40) }]))
  check('two commits are one by their prefix only', sameCommit('abcdef1', 'abcdef1234') && !sameCommit('abcdef1', 'abcdef2') && !sameCommit('abc', 'abcdef1') && !sameCommit('abcdef1', null))

  // The areas of a fix cycle.
  const areas = areasOf([{ key: 'f-1', door: 'jigc setup' }, { key: 'f-2', door: 'nowhere' }, { key: 'f-3', door: 'jigc doc show' }, { key: 'f-4', door: 'finalize.dirty' }], { included: [{ door: 'jigc setup', registry: 'verbs' }, { door: 'finalize.dirty', registry: 'codes' }], excluded: [{ door: 'jigc doc show', registry: 'verbs' }] })
  check('the areas', areas.map((x) => x.n + ':' + x.findings.map((f) => f.key).join('+')).join(' ') === '1:f-1+f-3 2:f-2 3:f-4' && areasOf([], null).length === 0 && areasOf([{ key: 'f', door: 'd' }], null).length === 1)
  check('a unit\'s doors', unitDoors({ doors: ['jigc setup'], registries: ['codes'] }, [{ door: 'jigc setup', registry: 'verbs' }, { door: 'jigc rename', registry: 'verbs' }, { door: 'finalize.dirty', registry: 'codes' }]).map((d) => d.door).join(',') === 'jigc setup,finalize.dirty')

  return { status: failed.length ? 'self-test-failed' : 'self-test-passed', checks, failed }
}

// ---- args: parsed, and refused before any agent ----
// args sometimes arrives JSON-stringified (milestone-build.js's M24 guard): a string that
// parses to an object is that object.
let parsedArgs = args
if (typeof args === 'string') {
  const s = args.trim()
  if (s.startsWith('{')) {
    try {
      parsedArgs = JSON.parse(s)
    } catch (_) { /* not JSON: refused below, as a string */ }
  }
}
const refusal = validateArgs(parsedArgs)
if (refusal) {
  return { status: 'refused', message: refusal + '. Nothing was run. Usage: Workflow({ name: \'stabilize\', args: { stage: \'test\' | \'fix\', run: \'<run>\', scratch: \'<absolute dir>\' } }) — the script\'s header has the rest.' }
}
if (parsedArgs.selfTest) return selfTest()
const unfit = notFit(parsedArgs)
if (unfit) {
  return { status: 'refused', stage: parsedArgs.stage, run: parsedArgs.run, not_fit: { stage: parsedArgs.stage, record: BUILD_RECORD }, message: unfit + '.' }
}
const v = parsedArgs
const model = v.model ? String(v.model) : 'opus'
const loopBranch = branchName(v.run)
const cycleLimit = v.raise ? v.raise.cycles : DEFAULT_CYCLES

// ---- running agents ----
// agentR — an agent() call retried on a transient failure (a throw, or a null return: a
// subagent that died on a terminal API error), with milestone-build.js's rate-limit breaker:
// once two DIFFERENT calls have exhausted their retries the run stops spawning. A retry is
// told what a dead attempt may have left; it is never told to clean the tree, because the
// untracked files under the run's directory are other agents' reports.
const TRANSIENT_RETRIES = 2
let exhaustedLabels = []
let breakerTripped = false
async function agentR(prompt, opts) {
  const lbl = (opts && opts.label) ? opts.label : 'agent'
  if (breakerTripped) {
    log('rate-limit breaker is tripped — NOT spawning ' + lbl)
    return null
  }
  let lastErr
  for (let attempt = 0; attempt <= TRANSIENT_RETRIES; attempt++) {
    const note = attempt === 0 ? '' : '\n\nRETRY after a transient failure of an earlier attempt at this same call. Before anything else look at what that attempt left — `git status --porcelain`, `git log -1`, and whether your report already stands at its path — and go on from it: never `git clean` (the untracked files under the run\'s directory are other agents\' reports), never make a commit or write a report that exists already. If you cannot tell what the dead attempt did, halt and say so.'
    try {
      const result = await agent(prompt + note, Object.assign({ model }, opts))
      if (result != null) return result
      lastErr = new Error('agent returned null')
      log('null return on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + (TRANSIENT_RETRIES + 1) + ')')
    } catch (e) {
      lastErr = e
      log('transient failure on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + (TRANSIENT_RETRIES + 1) + ') — ' + ((e && e.message) || e))
    }
  }
  log('exhausted ' + (TRANSIENT_RETRIES + 1) + ' attempts on ' + lbl + ' (' + ((lastErr && lastErr.message) || lastErr) + ')')
  if (!exhaustedLabels.includes(lbl)) exhaustedLabels.push(lbl)
  if (exhaustedLabels.length >= 2 && !breakerTripped) {
    breakerTripped = true
    log('TWO different agents exhausted their retries (' + exhaustedLabels.join(', ') + ') — the shape of an account-level rate limit, not of a fault in the run. No further agent is spawned.')
  }
  return null
}
// Every git step, and every read of the record, is a build-git call on Sonnet.
function gitStep(label, phaseTitle, prompt, schema) {
  return agentR(prompt, { label: 'git:' + label, phase: phaseTitle, agentType: 'build-git', schema, model: GIT_MODEL })
}
// toolStep — a git step that is ONE command of the tool: what the command `act` printed,
// read off the relayed line. Null when the agent returned nothing.
async function toolStep(label, phaseTitle, act, prompt) {
  return readStep(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA), act)
}
function roleStep(role, label, phaseTitle, prompt, schema) {
  return agentR(prompt, { label, phase: phaseTitle, agentType: ROLES[role].agentType, schema })
}

// halt — every way a stage stops short, as one shape. Committed work stands; the next
// invocation is a fresh one.
function halt(phaseName, why, more) {
  const transient = !!(more && more.transient)
  return {
    status: 'halted',
    stage: v.stage,
    run: v.run,
    halted: Object.assign({ phase: phaseName, reason: why, transient }, more || {}),
    message: 'stabilize ' + v.stage + ' of `' + v.run + '` HALTED at ' + phaseName + ': ' + why + (breakerTripped ? ' The rate-limit breaker is tripped (' + exhaustedLabels.join(', ') + '): wait for the window before invoking again.' : transient ? ' An agent returned no result after retries: that is infrastructure, not a verdict about the run.' : '') + ((more && more.then) || ' What was committed stands; a report a reporter wrote and no record step committed is still untracked under ' + runDir(v.run) + '/ and stays there. Once the cause is dealt with, invoke the stage again with the same args: it reads the state as it stands and works the next attempt.'),
  }
}
// gitHalt — a git step that did not do what the stage needs: it returned nothing, it
// halted, or it reported something else than the step's end state.
function gitHalt(phaseName, r, what) {
  if (!r) return halt(phaseName, what + ': the step returned no result', { transient: true })
  return halt(phaseName, what + ': ' + ((r.halt && r.halt.root_cause) || 'the step reported ' + JSON.stringify({ status: r.status, branch: r.branch, head: r.head, remote_head: r.remote_head, merge_commit: r.merge_commit })), { halt: r.halt || null })
}
// A branch step ended where the stage needs it: on `branch`, and — for a push — with the
// remote at the local head.
function onIt(r, branch, pushed) {
  return !!r && r.status === 'ready' && r.branch === branch && SHA_RE.test(String(r.head || '')) && (!pushed || r.remote_head === r.head)
}

// readState — the state document, relayed by a git step inside its command's line and held
// to that line's hash. A relay that does not hash, or holds no state document, is asked for
// again, twice; then there is no state, and the stage halts.
let stateReads = 0
async function readState() {
  const tag = v.stage + '-' + (++stateReads)
  let why = 'the step returned no result'
  for (let n = 0; n < 3; n++) {
    const again = n === 0 ? '' : '\n\nAGAIN (' + n + '): the line an earlier attempt returned was not the line the command printed. Copy the line from the command\'s output character for character.'
    const r = await toolStep('state:' + tag + (n ? ':again' + n : ''), 'State', 'state', statePrompt(v, tag) + again)
    if (!r) return { error: why, transient: true }
    if (!r.relay) {
      if (r.status !== 'read') return { error: (r.halt && r.halt.root_cause) || 'the step halted', halt: r.halt }
      if (plain(r.state) && typeof r.state.opened === 'boolean') return { state: r.state, branch: r.branch }
    }
    why = r.relay || 'the relayed line holds no state document'
    log('state relay rejected (' + why + ') — asking again')
  }
  return { error: why }
}

// recordFault — why a record is NOT accepted as recorded, or null. What is accepted is the
// commit step's own line — held to its hash by `readStep` — and nothing an agent says of
// it: the commit, the gate check that holds, and ONE RESULT PER CHECK THIS SCRIPT COMPOSED,
// each of which holds, in a batch of as many calls as were composed. A step that returns no
// such evidence did not record.
function recordFault(r, expect) {
  if (!r) return 'the record\'s commit step returned no result'
  if (r.status !== 'recorded') return (r.halt && r.halt.root_cause) || 'the record\'s commit step halted'
  if (!SHA_RE.test(String(r.commit || ''))) return 'the record\'s commit step returned no commit sha'
  if (!plain(r.gate) || r.gate.ok !== true) return 'the record\'s commit step returned no gate check that holds'
  const applied = r.applied
  if (!plain(applied) || applied.calls !== expect.calls || !Array.isArray(applied.checks) || applied.checks.length !== expect.checks) return 'the batch that was committed is not the one this stage composed: ' + expect.calls + ' call(s) and ' + expect.checks + ' check(s) were composed, and the commit step read ' + JSON.stringify(applied == null ? null : applied)
  if (applied.checks.some((c) => !plain(c) || c.ok !== true)) return 'a check of the record does not hold: ' + JSON.stringify(applied.checks)
  return null
}
// RECORD_THEN — what a halt of a record step says is to be done, in place of the sentence
// every other halt ends with: it is true whether or not the batch was applied.
const RECORD_THEN = ' What was committed stands. A record step that stopped may have left its batch APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree. Invoke the stage again: where a batch is pending, that invocation runs the full gate on it once more, commits and pushes it, and does nothing else; where none is, it reads the state as it stands and works the next attempt. To take a pending batch back instead — the orchestrator\'s decision, a subagent\'s act — `dev/stabilize-record discard --run ' + v.run + '` puts every table back as it was before the record step, and keeps the reports and the round\'s scope.'
// recordStep — a record step, whole: the executor applies the batch and runs the gate; then
// the ONE commit, a git step. `rec` is what `recordPrompt` returned.
async function recordStep(label, branch, rec) {
  const gated = await roleStep('record', 'record:' + label, 'Record', rec.text, RECORD_SCHEMA)
  if (!gated) return { fault: 'the record step returned no result', result: null }
  if (gated.status !== 'gated') return { fault: (gated.halt && gated.halt.root_cause) || 'the record step halted', result: gated }
  const r = await toolStep('record:' + label, 'Record', 'record', recordCommitPrompt(v, branch, rec.gate, rec.expect))
  const fault = recordFault(r, rec.expect)
  return fault ? { fault, result: r } : { result: r }
}
// finishRecord — an invocation that finds a batch a record step applied and did not commit:
// the gate on it again, the one commit, the push, the state — and NOTHING ELSE of a stage.
async function finishRecord(gs) {
  phase('Record')
  const pending = gs.pending
  log('a record step left its batch applied and not committed on ' + gs.branch + ' — ' + pending.calls + ' call(s), `' + pending.subject + '`: this invocation gates and commits it, and starts nothing')
  const recorded = await recordStep('pending', gs.branch, pendingRecordPrompt(v, gs.branch, v.scratch + '/record/pending', pending))
  if (recorded.fault) return halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null, gate: recorded.result ? recorded.result.gate : null, pending, then: RECORD_THEN })
  const pushed = await toolStep('push:pending', 'Record', 'push', pushPrompt(v, gs.branch))
  if (!onIt(pushed, gs.branch, true)) return gitHalt('push', pushed, 'the pending record is committed on ' + gs.branch + ' (' + recorded.result.commit + ') and the branch was not pushed')
  const read = await readState()
  if (!read.state) return halt('state', 'the pending record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient })
  return Object.assign({ status: 'recorded', stage: v.stage, run: v.run, pending, record: recorded.result.commit, gate: recorded.result.gate, message: 'a record step of an earlier invocation had applied its batch and not committed it: this invocation ran the gate on it, committed and pushed it, and did nothing else — `next` names the step.' }, nextOf(read.state))
}

// findingLines — a reporter's structured findings as the lines triage is handed.
function findingLines(findings) {
  return findings.map((f) => f.id + ' — ' + f.title + ' · door: ' + f.door + ' · clause: ' + f.clause + (f.severity ? ' · severity: ' + f.severity : '') + (f.lead ? ' · a lead' : '') + ' · repro: ' + f.repro)
}
function leftOpenSource(reporter, report, leftOpen) {
  return { reporter, report, findings: (leftOpen || []).map((text, i) => 'left open ' + (i + 1) + ' — ' + text) }
}

// runUnits — the chains of a list of units, in parallel under the runtime's cap. Returns,
// per unit, its status (`green` once every step of its chain reported; `void` when one did
// not) and every reporter it launched with what that reporter returned.
async function runUnits(ctx, launch, built, units, phaseTitle) {
  const out = await parallel(units.map((unit) => async () => {
    const done = []
    let status = 'green'
    let crossModel = null
    for (const stepList of unit.chain) {
      const named = stepList.map((step) => ({ step, name: launch.add([unit.item, step.as]) }))
      const promptOf = (step, name) => (step.crossModel ? crossModelPrompt(ctx, name, unit) : unitPrompt(ctx, launch, name, step, unit, built, (step.hands || []).map((h) => ({ as: h, report: (done.find((d) => d.as === h) || {}).report }))))
      const results = await parallel(named.map(({ step, name }) => () => roleStep(step.role, unit.item + ':' + step.as, phaseTitle, promptOf(step, name), UNIT_SCHEMA)))
      let stops = false
      named.forEach(({ step, name }, i) => {
        const r = results[i]
        const reported = !!r && r.status === 'reported'
        if (step.crossModel) {
          // The cross-model pass that did not run is void FOR THAT PASS: the unit goes on,
          // and a reporter that left no report is not one the record is held to.
          crossModel = reported ? 'ran' : 'void'
          if (!r || !r.report) return launch.drop(name)
        } else if (!reported) {
          stops = true
        }
        done.push({ as: step.as, name, drives: ROLES[step.role].drives, result: reported || !step.crossModel ? r : null, report: r ? r.report : null })
      })
      if (stops) {
        status = 'void'
        break
      }
    }
    return { item: unit.item, clause: unit.clause, status, crossModel, reporters: done }
  }))
  return units.map((unit, i) => out[i] || { item: unit.item, clause: unit.clause, status: 'void', crossModel: null, reporters: [] })
}

// triagePasses — triage of every finding, verify-real on everything graded breaks or
// unclear, and per contested finding an advocate and an independent drive; then again over
// what those agents left open. Returns the entries by key, with the verdicts beside them.
async function triagePasses(ctx, launch, built, sources, forksIn) {
  const entries = []
  const forks = forksIn.slice()
  const faults = []
  let pending = sources.filter((s) => s.findings.length)
  let handed = 0
  for (let pass = 1; pending.length; pass++) {
    const count = pending.reduce((n, s) => n + s.findings.length, 0)
    handed += count
    const tName = launch.add(['triage', 'p' + pass])
    log('triage pass ' + pass + ': ' + count + ' finding(s) from ' + pending.length + ' reporter(s)')
    const t = await roleStep('triage', 'triage:p' + pass, 'Triage', triagePrompt(ctx, launch, tName, pending, pass), TRIAGE_SCHEMA)
    if (!t || t.status !== 'graded') return { fault: t ? 'triage halted: ' + ((t.halt && t.halt.root_cause) || 'no reason given') : 'triage returned no result', transient: !t, halt: t ? t.halt : null }
    const counted = (t.counts.findings_in || []).reduce((n, c) => n + c.count, 0)
    if (counted !== count || t.entries.length + t.counts.merged !== count) return { fault: 'triage pass ' + pass + ' was handed ' + count + ' finding(s) and accounts for ' + counted + ' in, ' + t.entries.length + ' entries and ' + t.counts.merged + ' merged: a finding in no entry and in no named merge is lost, and nothing is recorded on that' }
    for (const e of t.entries) {
      if (!isSlug(e.key)) return { fault: 'triage returned the key ' + JSON.stringify(e.key) + ', which is not a ledger key' }
      const held = entries.find((x) => x.key === e.key)
      if (held) Object.assign(held, e, { new: held.new })
      else entries.push(Object.assign({}, e))
    }
    pending = []
    if (pass > VERIFY_PASSES) {
      log('NOT VERIFIED: pass ' + pass + ' is past the ' + VERIFY_PASSES + ' passes that are verified — its entries get their rows, and those graded breaks or unclear stay unverified: the state\'s `next` then says that the round\'s triage is not finished (`triage`), and names them')
      break
    }
    const hints = {}
    for (const h of t.to_verify || []) hints[h.key] = h.redrive
    const toVerify = t.entries.filter((e) => e.grade === 'breaks' || e.grade === 'unclear')
    if (!toVerify.length) continue
    const unnamed = toVerify.filter((e) => !forkNames('p' + pass, e.key)).map((e) => e.key)
    if (unnamed.length) return { fault: 'no reporter name can be made for the verifier of ' + unnamed.join(', ') + ': the key is too long to carry a prefix inside ' + SLUG_MAX + ' characters' }
    const named = toVerify.map((e) => ({ e, name: launch.add(['verify', 'p' + pass, e.key]) }))
    log('verify-real, pass ' + pass + ': ' + named.length + ' finding(s)')
    const verdicts = await parallel(named.map(({ e, name }) => () => roleStep('verify', 'verify:' + e.key, 'Triage', verifyPrompt(ctx, launch, name, e, hints[e.key], built), VERIFY_SCHEMA)))
    const contested = []
    named.forEach(({ e, name }, i) => {
      const r = verdicts[i]
      const entry = entries.find((x) => x.key === e.key)
      if (!r || r.status !== 'verified' || !r.verdict) {
        log('verify-real left `' + e.key + '` without a verdict' + (r && r.halt ? ': ' + r.halt.root_cause : '') + ' — it stays unverified')
        return
      }
      const ran = r.ran_on || {}
      if (r.asserted_sha256 !== built.candidate.sha256 || (r.verdict === 'confirmed' && r.regression === true && ran.previous !== built.previous.sha256)) {
        faults.push('the verifier of `' + e.key + '` asserted ' + JSON.stringify({ candidate: r.asserted_sha256 == null ? null : r.asserted_sha256, previous: ran.previous == null ? null : ran.previous }) + ', not the binaries it was handed')
        return
      }
      if (r.verdict === 'confirmed' && typeof r.regression !== 'boolean') {
        faults.push('the verifier of `' + e.key + '` confirmed it and did not say whether it is a regression')
        return
      }
      entry.verdict = r.verdict
      if (r.verdict === 'confirmed') entry.regression = r.regression
      entry.basis = r.basis
      if (r.left_open && r.left_open.length) pending.push(leftOpenSource(name, r.report, r.left_open))
      if (r.contested) contested.push({ key: e.key, kind: 'contested', door: e.door, clause: e.clause, repro: e.repro, statement: 'the verifier found the finding to contest a settled decision — ' + (r.basis || '(no basis returned)') })
    })
    for (const fork of contested) {
      const driven = await driveFork(ctx, launch, built, fork, 'p' + pass)
      forks.push(driven.fork)
      pending = pending.concat(driven.leftOpen)
      if (driven.fault) faults.push(driven.fault)
    }
  }
  return { entries, forks, faults, handed }
}

// driveFork — ruling 5: a contested fix, or one that needs a new mechanism, goes to the
// human with a robust-advocate's case, and the proposal is driven before the human sees it —
// by the advocate, as its definition has it, and then by an agent that is not the advocate.
async function driveFork(ctx, launch, built, fork, tag) {
  const aName = launch.add(['advocate', tag, fork.key])
  const adv = await roleStep('advocate', 'advocate:' + fork.key, 'Triage', advocatePrompt(ctx, launch, aName, fork, built), ADVOCATE_SCHEMA)
  const out = { fork: Object.assign({}, fork, { advocate: adv, independent_drive: null, driven_by: null }), leftOpen: [], fault: null }
  if (!adv || adv.status !== 'argued') return out
  if (adv.asserted_sha256 !== built.candidate.sha256) out.fault = 'the advocate of `' + fork.key + '` asserted ' + JSON.stringify(adv.asserted_sha256) + ', not the candidate\'s hash'
  if (adv.left_open && adv.left_open.length) out.leftOpen.push(leftOpenSource(aName, adv.report, adv.left_open))
  const pName = launch.add(['proposal', tag, fork.key])
  const drive = await roleStep('proposal', 'proposal:' + fork.key, 'Triage', proposalPrompt(ctx, pName, fork, adv, built), PROPOSAL_SCHEMA)
  out.fork.independent_drive = drive
  out.fork.driven_by = ROLES.proposal.agentType
  if (drive && drive.status === 'driven') {
    if (drive.asserted_sha256 !== built.candidate.sha256) out.fault = 'the independent drive of `' + fork.key + '` asserted ' + JSON.stringify(drive.asserted_sha256) + ', not the candidate\'s hash'
    if (drive.left_open && drive.left_open.length) out.leftOpen.push(leftOpenSource(pName, drive.report, drive.left_open))
  }
  return out
}

// What a stage's triage becomes in the record: the new rows, and the round's triage entries.
function triageRecord(ctx, entries) {
  return {
    rows: entries.filter((e) => e.new).map((e) => ({ key: e.key, doctype: e.doctype, round: ctx.round, source: e.source, door: e.door, clause: e.clause, repro: e.repro })),
    triage: entries.map((e) => {
      const t = { key: e.key, grade: e.grade }
      if (e.grade === 'out-of-scope') t.bound = e.bound
      if (e.verdict) t.verdict = e.verdict
      if (e.verdict === 'confirmed') t.regression = e.regression
      return t
    }),
  }
}

// preflightOf — one preflight call, and what must hold of its return before anything drives it.
async function preflightOf(ctx, launch, parts, plan, phaseTitle) {
  const name = launch.add(parts)
  const r = await roleStep('preflight', name, phaseTitle, preflightPrompt(ctx, launch, name, plan), PREFLIGHT_SCHEMA)
  if (!r || r.status !== 'ready') return { fault: r ? 'the preflight halted: ' + ((r.halt && r.halt.root_cause) || 'no reason given') : 'the preflight returned no result', transient: !r, halt: r ? r.halt : null }
  if (plan.build) {
    const c = r.candidate || {}
    if (c.sha !== plan.sha || !SHA256_RE.test(String(c.sha256 || '')) || c.binary !== plan.binary) return { fault: 'the preflight did not return the candidate it was asked for: ' + JSON.stringify(c) }
    if (!r.previous || !SHA256_RE.test(String(r.previous.sha256 || '')) || !r.previous.binary) return { fault: 'the preflight returned no previous release\'s binary: ' + JSON.stringify(r.previous || null) }
    if (r.previous.version !== plan.previous.version) return { fault: 'the preflight returned the binary of ' + JSON.stringify(r.previous.version) + ', and the previous release the run\'s record names is ' + plan.previous.version + ': every regression fact would be measured against the wrong release' }
  }
  if (plan.image && !(r.image && r.image.verified && r.image.tag)) return { fault: 'the trial image is not verified: ' + JSON.stringify(r.image || null) }
  return { result: r }
}

function attached(state) {
  return { next: state.next, stop: state.stop, not_ready: state.not_ready, candidate: state.candidate, fix_rounds: state.fix_rounds, evidence: evidenceOf(state), blockers: state.blockers, human_list: state.human_list, untriaged: state.untriaged, retest: state.retest, human_clauses: state.human_clauses, human_stages: state.human_stages, unsettled: state.unsettled, forbids_close: state.forbids_close, position: state.position }
}
// What a stage returns of `next`: the value itself, always; the whole state beside it when
// this script does not know the value; and with `close` the step the close owes first, and
// per clause how far behind the candidate its last evidence is.
function nextOf(state) {
  const o = outcomeOf(state)
  if (!o.known) return { next: o.next, returned_to_orchestrator: true, state }
  if (o.rule) return { next: o.next, rule: o.rule }
  return o.next === 'close' ? { next: o.next, close: closeOf(), evidence: evidenceOf(state) } : { next: o.next }
}

// rulingsStep — the ONE call of the step that records what the human ruled: its record, the
// push of the branch it is on, and the state read back.
async function rulingsStep(round, branch, ran, at) {
  phase('Record')
  const ruled = await recordStep('rulings:r' + round, branch, rulingsRecordPrompt(v, round, branch, v.rulings, ran, at))
  if (ruled.fault) return { halted: halt('rulings', ruled.fault, { transient: !ruled.result, halt: ruled.result ? ruled.result.halt : null, gate: ruled.result ? ruled.result.gate : null, then: RECORD_THEN }) }
  const pushed = await toolStep('push:rulings', 'Record', 'push', pushPrompt(v, branch))
  if (!onIt(pushed, branch, true)) return { halted: gitHalt('push', pushed, 'the rulings are recorded on ' + branch + ' (' + ruled.result.commit + ') and the branch was not pushed') }
  const read = await readState()
  if (!read.state) return { halted: halt('state', 'the rulings are recorded (' + ruled.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient }) }
  return { state: read.state, record: ruled.result.commit }
}

// ruleTheRun — an invocation that ONLY RECORDS RULINGS: what the human ruled about the run
// (a go after a stop, one more re-run of a clause, the bound raised), which either stage
// takes — or, on the `test` stage, what the human ruled on findings and bounds. They are
// recorded on the loop branch by the one step that may write them, NOTHING is started, and
// the state's `next` goes back — which now names the step the human was asked about.
async function ruleTheRun(state, checkedOut) {
  const told = { status: 'refused', stage: v.stage, run: v.run, next: state.next, stop: state.stop, human_clauses: state.human_clauses, human_stages: state.human_stages }
  if (checkedOut !== loopBranch) return Object.assign(told, { message: 'an invocation that only records rulings records them on the loop branch ' + loopBranch + ', and `' + checkedOut + '` is checked out. Nothing was run beyond the two reads.' })
  const about = v.rulings.every(runRuling)
  const why = about ? runRulingsFault(v.rulings, state) : findingRulingsFault(v.rulings, state)
  if (why) return Object.assign(told, { message: why + '. Nothing was recorded.' })
  // The rounds in which a clause has an item whose re-run is spent: where a grant is a fact.
  const ran = {}
  for (const c of state.clauses || []) ran[c.clause] = (c.items || []).filter((i) => i.rerun === 'spent').map((i) => i.round).filter((n, at, all) => all.indexOf(n) === at)
  // The round a finding's passes are counted in, and the round each stage is spent in.
  const stages = {}
  for (const h of state.human_stages || []) stages[h.stage] = h.round
  const ruled = await rulingsStep((about && state.stop ? state.stop.round : state.round) || 0, loopBranch, ran, { round: state.round, stages })
  if (ruled.halted) return ruled.halted
  return Object.assign({ status: 'ruled', stage: v.stage, run: v.run, rulings: v.rulings, record: ruled.record, message: 'the human\'s rulings ' + (about ? 'about the run' : 'on the round\'s findings and bounds') + ' are recorded, and nothing was started: `next` names the step.' }, nextOf(ruled.state))
}

// finishTriage — a round's triage that a stage left unfinished (`next: 'triage'`), finished
// by that stage's next invocation: the rows the state lists as untriaged are graded, verified
// and recorded, and NO INSTRUMENT RUNS — every report of the round is on record already.
// Which stage, which round and which attempt is the position's (`triage: true`), read from
// committed state; nothing here is an argument. `tip` is the commit the verifiers drive.
async function finishTriage(ctx, state, tip, branch) {
  phase('Triage')
  const launch = launcher(ctx)
  const sources = ledgerSource(v.run, state)
  const binary = v.scratch + '/bin/' + tip.label + '.a' + ctx.attempt + '/jigc'
  log('round ' + ctx.round + ': the ' + ctx.stage + ' stage left its triage unfinished — ' + (state.untriaged || []).length + ' row(s) are graded and verified now, attempt ' + ctx.attempt + '; no instrument runs')
  const pre = await preflightOf(ctx, launch, ['preflight'], { build: true, image: false, sha: tip.sha, label: tip.label, branch, binary, checks: [], crossModel: false, previous: previousOf(state), tested: tip.tested }, 'Triage')
  if (pre.fault) return { halted: halt('preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, branch }) }
  const built = { candidate: { label: tip.label, sha: tip.sha, binary, sha256: pre.result.candidate.sha256 }, previous: pre.result.previous, image: null }
  const tri = await triagePasses(ctx, launch, built, sources, [])
  if (tri.fault) return { halted: halt('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, branch }) }
  if (tri.faults.length) return { halted: halt('binary', tri.faults.join('; '), { branch }) }
  phase('Record')
  const rec = triageRecord(ctx, tri.entries)
  const dir = v.scratch + '/record/triage-r' + ctx.round + (ctx.stage === 'fix' ? '-c' + ctx.cycle : '') + '-a' + ctx.attempt
  const commands = stageRecordCommands(ctx, { reporters: launch.names, rows: rec.rows, triage: rec.triage, patches: [], clauses: [], facts: null, keys: tri.entries.map((e) => e.key) })
  const recorded = await recordStep('triage:r' + ctx.round, branch, recordPrompt(v, 'round ' + ctx.round + '\'s triage, finished — ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s) graded; no instrument ran', branch, dir, ctx.round, commands, v.run + ' r' + ctx.round + ' — the round\'s triage, finished'))
  if (recorded.fault) return { halted: halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null, gate: recorded.result ? recorded.result.gate : null, launched: launch.names, branch, forks: tri.forks, then: RECORD_THEN }) }
  const pushed = await toolStep('push:triage', 'Record', 'push', pushPrompt(v, branch))
  if (!onIt(pushed, branch, true)) return { halted: gitHalt('push', pushed, 'the triage is recorded on ' + branch + ' (' + recorded.result.commit + ') and the branch was not pushed') }
  const after = await readState()
  if (!after.state) return { halted: halt('state', 'the triage is recorded and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, branch }) }
  return { state: after.state, entries: tri.entries, forks: tri.forks, handed: tri.handed, record: recorded.result.commit, reporters: launch.names, candidate: built.candidate }
}
function closeOf() {
  return {
    order: ['the close of the run, as the workflow\'s doc has it', 'sync: close.sync, spawned verbatim on model close.sync.model', 'dev/gate green on the merged tree', 'git push origin ' + loopBranch, 'gh pr create --base main --head ' + loopBranch],
    sync: { agentType: 'build-git', model: GIT_MODEL, label: 'git:sync-main', prompt: syncMainPrompt(v.run), schema: STEP_SCHEMA, reads: 'its `line` is the ONE line `' + STEP_TOOL + ' sync-main` printed — JSON: status `merged` or `up-to-date` with head, origin_main and resolved_logs, or `halted` with the word it refused with and its halt report' },
  }
}

// ---- the `test` stage ----
async function runTest() {
  phase('State')
  const gs = await toolStep('state', 'State', 'git-state', gitStatePrompt(v))
  if (!onIt(gs, loopBranch, false)) return gitHalt('git', gs, 'the tree and the branches are not in the state the `test` stage starts from — the loop branch ' + loopBranch + ' checked out, holding nothing but what the record script wrote and no commit holds yet, and no product change outside a round\'s merge')
  if (gs.pending) return await finishRecord(gs)
  const first = await readState()
  if (!first.state) return halt('state', 'the run\'s state could not be read: ' + first.error, { transient: !!first.transient, halt: first.halt || null })
  let state = first.state
  if (!state.opened) return halt('state', 'the run `' + v.run + '` has no opening record (' + runDir(v.run) + '/opening.md): a run opens with the human-led step, and `test` does not start before it')
  if (v.rulings) return await ruleTheRun(state, gs.branch)
  const at = state.position.test
  if (at.refused) return { status: 'refused', stage: 'test', run: v.run, refused: at, message: 'the `test` stage is refused (' + at.refused + ', round ' + at.round + '): ' + refusalOf(at) + '. Nothing was run beyond the two reads.', next: state.next, not_ready: state.not_ready, stop: state.stop }
  const ctx = { run: v.run, round: at.round, stage: 'test', attempt: at.attempt, scratch: v.scratch }
  const label = 'c' + ctx.round
  // The round's triage is not finished, and this stage left it: finish it, and run nothing.
  if (at.triage) {
    if (v.scope != null || v.clause != null || v.crossModel != null) return { status: 'refused', stage: 'test', run: v.run, message: 'round ' + at.round + '\'s triage is not finished (`next` is `' + state.next + '`): this invocation finishes it and runs no instrument, so it takes no scope, clause or crossModel. Nothing was run beyond the two reads.', next: state.next, untriaged: state.untriaged }
    const done = await finishTriage(ctx, state, { sha: String(state.rounds.find((r) => r.round === at.round).facts.candidate), label, tested: true }, loopBranch)
    if (done.halted) return done.halted
    return Object.assign({ status: 'triaged', stage: 'test', run: v.run, round: ctx.round, triage_only: true, candidate: done.candidate, record: done.record, counts: { reporters: done.reporters.length, findings_in: done.handed, entries: done.entries.length, blockers: done.state.blockers.length, for_the_human: done.state.human_list.length }, human_list: done.state.human_list, forks: done.forks, blockers: done.state.blockers, forbids_close: done.state.forbids_close }, nextOf(done.state))
  }
  // A RE-RUN is the position's, never the invocation's to decide: the state asks for it
  // (`rerun`) and names the clause, the round that selected its due items, and the attempt
  // each one's run will be. `args.clause` says that the orchestrator read the same answer.
  const rerun = at.rerun || null
  if (rerun && v.clause !== rerun.clause) return { status: 'refused', stage: 'test', run: v.run, message: 'the state asks for a re-run (`next` is `' + state.next + '`): the items of clause `' + rerun.clause + '` that are due in round ' + rerun.round + ' — ' + rerun.items.map((i) => i.item + ' (attempt ' + i.attempt + ')').join(', ') + '. Invoke `test` with clause: \'' + rerun.clause + '\'; an invocation without it would begin a round nobody asked for. Nothing was run beyond the two reads.', next: state.next, retest: state.retest, position: at }
  if (!rerun && v.clause != null) return { status: 'refused', stage: 'test', run: v.run, message: 'args.clause asks for a re-run, and the state does not (`next` is `' + state.next + '`): an item is run again only where the record script lists it as due, as an attempt of the round that selected it. Nothing was run beyond the two reads.', next: state.next, retest: state.retest }
  const sha = String(gs.head)
  const launch = launcher(ctx)
  const items = state.items || []
  // The items this invocation names for a cross-model source pass — none, unless it names them.
  const crossNamed = v.crossModel || []
  const crossStrangers = crossNamed.filter((id) => !items.some((i) => i.item === id && chainOf(i.kind, true).some((stepList) => stepList.some((s) => s.crossModel))))
  if (crossStrangers.length) return halt('state', 'args.crossModel names ' + crossStrangers.join(', ') + ', which is no item of the test set whose chain has a cross-model pass — the items that have one are ' + (items.filter((i) => chainOf(i.kind, true).some((stepList) => stepList.some((s) => s.crossModel))).map((i) => i.item).join(', ') || '(none)'))
  const runs = (i) => (rerun ? rerun.items.some((x) => x.item === i.item) : i.selected === true)
  log('round ' + ctx.round + ', attempt ' + ctx.attempt + ' of the test stage: ' + (rerun ? 'A RE-RUN inside it, on ' + sha + ' — ' + rerun.items.map((i) => i.item + ' (its attempt ' + i.attempt + ')').join(', ') + ' of clause `' + rerun.clause + '`, and nothing else' : 'candidate ' + label + ' = ' + sha + '; ' + items.length + ' item(s) in the test set'))
  if (v.stopAfter === 'state') return { status: 'stopped', after: 'state', stage: 'test', run: v.run, round: ctx.round, attempt: ctx.attempt, candidate: { label, sha }, state: attached(state) }

  // Preflight (the asserts, the one build, the checks that run on every candidate) beside
  // the scope step: neither reads what the other writes. A RE-RUN has no scope step — the
  // round's scope stands, and its doors are what the items run over again — and no gate of
  // the candidate's to record: its round has one, and the record commit is held to the
  // gate of the candidate the run tested last.
  phase('Preflight and scope')
  const earlier = (state.rounds || []).find((r) => r.round === ctx.round - 1)
  const binary = v.scratch + '/bin/' + label + '.a' + ctx.attempt + '/jigc'
  // The candidate's own gate: the preflight runs it once and keeps its whole output, and
  // the record's batch puts what it shows red on record — what a record commit is held to.
  const candidateGate = rerun ? null : v.scratch + '/gate/' + label + '.a' + ctx.attempt + '.txt'
  const always = items.filter((i) => i.kind === CHECK_KIND && runs(i))
  const scopeName = rerun ? null : launch.add(['scope'])
  const steps = [() => preflightOf(ctx, launch, ['preflight'], { build: true, image: false, gate: candidateGate, sha, label, branch: loopBranch, binary, checks: always, crossModel: crossNamed.length > 0, previous: previousOf(state) }, 'Preflight and scope')]
  if (!rerun) steps.push(() => roleStep('scope', 'scope', 'Preflight and scope', scopePrompt(ctx, launch, scopeName, { sha, label, base: earlier && earlier.facts ? earlier.facts.candidate : null, earlier: !!earlier, scope: v.scope, previous: previousOf(state), fallback: state.facts.scope }), SCOPE_SCHEMA))
  const both = await parallel(steps)
  const pre = both[0] || { fault: 'the preflight returned no result', transient: true }
  const sc = rerun ? { status: 'stands' } : both[1]
  if (pre.fault) return halt('preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null })
  if (!sc || (sc.status !== 'written' && sc.status !== 'stands')) return halt('scope', sc ? 'the scope step halted: ' + ((sc.halt && sc.halt.root_cause) || 'no reason given') : 'the scope step returned no result', { transient: !sc, halt: sc ? sc.halt : null })
  const built = { candidate: { label, sha, binary, sha256: pre.result.candidate.sha256 }, previous: pre.result.previous, image: null }
  const second = await readState()
  if (!second.state) return halt('state', 'the run\'s state could not be read after the scope step: ' + second.error, { transient: !!second.transient, halt: second.halt || null })
  state = second.state
  if (!rerun && (!state.doors || state.round !== ctx.round)) return halt('scope', 'the scope step reported ' + sc.status + ', and the state holds no doors for round ' + ctx.round)
  // The doors the round's items run over: its own scope's — as the state reads them back, or
  // as the re-run's position names them again.
  const inside = rerun ? rerun.doors : state.doors.included
  const selected = state.items.filter(runs)
  const hunting = selected.filter((i) => i.kind !== CHECK_KIND)
  const late = selected.filter((i) => i.kind === CHECK_KIND && !always.some((x) => x.item === i.item))
  const unknownKinds = hunting.filter((i) => !CHAINS[i.kind]).map((i) => i.item + ' (' + i.kind + ')')
  if (unknownKinds.length) return halt('state', 'this round runs item(s) of a kind the harness has no chain for: ' + unknownKinds.join(', ') + ' — a kind is added by adding its chain to CHAINS, never by guessing one', { launched: launch.names })
  const needsImage = hunting.some((i) => chainOf(i.kind, false).some((stepList) => stepList.some((s) => s.image)))
  let checks = pre.result.checks || []
  if (needsImage || late.length) {
    const more = await preflightOf(ctx, launch, ['preflight', 'second'], { build: false, image: needsImage, sha, label, branch: loopBranch, binary, checks: late, crossModel: false, previous: previousOf(state) }, 'Preflight and scope')
    if (more.fault) return halt('preflight', more.fault, { transient: !!more.transient, halt: more.halt || null })
    built.image = needsImage ? more.result.image : null
    checks = checks.concat(more.result.checks || [])
  }
  log('scope ' + sc.status + ': ' + inside.length + ' door(s) inside; ' + hunting.length + ' of ' + state.items.filter((i) => i.kind !== CHECK_KIND).length + ' hunting item(s) run ' + (rerun ? 'again' : 'this round') + ', and ' + (always.length + late.length) + ' check(s)')
  if (sc.uncovered && sc.uncovered.length) log('NOT COVERED: ' + sc.uncovered.length + ' door(s) of the round\'s test set are reached by no item — ' + sc.uncovered.join(' · '))
  if (v.stopAfter === 'preflight') return { status: 'stopped', after: 'preflight', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, previous: built.previous, checks, scope: { status: sc.status, uncovered: sc.uncovered || [], reached_but_excluded: sc.reached_but_excluded || [] }, reporters: launch.names }

  // The round's instruments.
  phase('Instruments')
  const crossTool = crossNamed.length > 0 && (pre.result.checks || []).some((c) => c.check === CROSS_CHECK && c.status === 'green')
  const crossVoid = crossNamed.filter((id) => !crossTool || !hunting.some((i) => i.item === id)).map((id) => ({ item: id, why: crossTool ? 'the item does not run in this round' : 'the tool of the cross-model pass did not answer on this machine' }))
  for (const lost of crossVoid) log('CROSS-MODEL PASS VOID for `' + lost.item + '`: ' + lost.why + ' — its other passes run regardless')
  const units = hunting.map((i) => ({ item: i.item, kind: i.kind, clause: i.clause, brief: i.brief, chain: chainOf(i.kind, crossTool && crossNamed.includes(i.item)), doors: unitDoors(i, inside), range: null }))
  const nameable = unitNames(units, launch.names)
  if (nameable.fault) return halt('state', nameable.fault)
  const ran = await runUnits(ctx, launch, built, units, 'Instruments')
  const reporters = ran.reduce((all, u) => all.concat(u.reporters), [])
  for (const u of ran) if (u.crossModel === 'void') crossVoid.push({ item: u.item, why: 'the pass was launched and did not report' })
  const crossRan = ran.filter((u) => u.crossModel === 'ran').map((u) => u.item)
  const wrong = hashMismatch(built.candidate.sha256, reporters)
  if (wrong.length) return halt('binary', 'a driving agent did not assert the candidate\'s binary (' + built.candidate.sha256 + '): ' + wrong.map((w) => w.reporter + ' asserted ' + JSON.stringify(w.asserted)).join('; ') + ' — nothing it drove is evidence about this candidate', { mismatched: wrong })
  const cr = await toolStep('check-reports', 'Instruments', 'check-reports', checkReportsPrompt(ctx, launch.names))
  const seen = cr && cr.status === 'checked' && plain(cr.check) ? cr.check : null
  if (!seen || seen.ok !== true) return halt('reports', seen ? 'not every launched reporter left exactly one report — missing: ' + JSON.stringify(seen.missing) + '; extra: ' + JSON.stringify(seen.extra) : 'the report check could not be read', { transient: !cr, check: seen, launched: launch.names })
  if (v.stopAfter === 'instruments') return { status: 'stopped', after: 'instruments', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, units: ran.map((u) => ({ item: u.item, status: u.status, findings: u.reporters.reduce((n, r) => n + (r.result && r.result.findings ? r.result.findings.length : 0), 0) })), reporters: launch.names }

  // Triage of every finding; verify-real; the forks.
  phase('Triage')
  const sources = reporters.filter((r) => r.result && r.result.findings).map((r) => ({ reporter: r.name, report: r.report, findings: findingLines(r.result.findings) }))
  if (sc.left_open && sc.left_open.length) sources.push(leftOpenSource(scopeName, sc.report, sc.left_open))
  // And the rows whose triage nobody finished — seeded at the opening, or left without a
  // verdict by an earlier stage: this triage is the next one that runs.
  for (const source of ledgerSource(v.run, state)) sources.push(source)
  const tri = await triagePasses(ctx, launch, built, sources, [])
  if (tri.fault) return halt('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null })
  if (tri.faults.length) return halt('binary', tri.faults.join('; '))
  if (v.stopAfter === 'triage') return { status: 'stopped', after: 'triage', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, entries: tri.entries, forks: tri.forks, reporters: launch.names }

  // The record step: the reports checked, what each item did, the rows, the round's facts.
  // ONE RESULT PER ITEM THIS INVOCATION RAN — the record script derives every clause's
  // status from them, and files a red check as a finding in the same call.
  phase('Record')
  const unitStatus = ran.map((u) => ({ item: u.item, status: u.status, reason: 'a step of its chain did not report' }))
  for (const item of always.concat(late)) {
    const c = checks.find((x) => x.check === item.item)
    unitStatus.push({ item: item.item, status: c ? c.status : 'void', reason: c ? c.evidence || 'the check could not run, and the preflight returned no reason' : 'the preflight returned nothing for it', door: item.doors[0], evidence: c && c.evidence ? c.evidence + ' — the preflight\'s report: ' + pre.result.report : null })
  }
  const rec = triageRecord(ctx, tri.entries)
  // A re-run writes no fact of the round: the round is tested, and its record stands.
  const facts = rerun ? null : { candidate: sha, binary: built.candidate.sha256 }
  if (crossNamed.length) Object.assign(facts, { 'cross-model': crossRan, 'cross-model-void': crossVoid.map((x) => x.item) })
  if (facts && sc.base && SHORT_SHA_RE.test(sc.base)) facts.base = sc.base
  const dir = v.scratch + '/record/test-r' + ctx.round + '-a' + ctx.attempt
  const commands = stageRecordCommands(ctx, { reporters: launch.names, gate: rerun ? null : { commit: sha, file: candidateGate }, results: { commit: sha, rows: resultRows(unitStatus) }, rows: rec.rows, triage: rec.triage, patches: [], facts, keys: tri.entries.map((e) => e.key) })
  const recorded = await recordStep('test:r' + ctx.round, loopBranch, recordPrompt(v, rerun ? 'the record of a re-run inside round ' + ctx.round + ' — ' + rerun.items.length + ' item(s) of clause `' + rerun.clause + '` run again, ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s)' : 'the record of round ' + ctx.round + '\'s test stage — ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s)', loopBranch, dir, rerun ? state.candidate.round : ctx.round, commands, v.run + ' r' + ctx.round + (rerun ? ' — a re-run of ' + rerun.clause : ' — the test stage\'s record')))
  if (recorded.fault) return halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null, gate: recorded.result ? recorded.result.gate : null, launched: launch.names, forks: tri.forks, then: RECORD_THEN })
  const pushed = await toolStep('push', 'Record', 'push', pushPrompt(v, loopBranch))
  if (!onIt(pushed, loopBranch, true)) return gitHalt('push', pushed, 'the record is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
  const last = await readState()
  if (!last.state) return halt('state', 'the record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + last.error, { transient: !!last.transient })
  state = last.state

  const out = nextOf(state)
  const byGrade = {}
  for (const e of tri.entries) byGrade[e.verdict || e.grade] = (byGrade[e.verdict || e.grade] || 0) + 1
  return Object.assign({
    status: 'triaged',
    stage: 'test',
    run: v.run,
    round: ctx.round,
    rerun,
    cross_model: { named: crossNamed, ran: crossRan, void: crossVoid },
    candidate: built.candidate,
    record: recorded.result.commit,
    counts: { items: state.items.length, items_run: units.length + always.length + late.length, reporters: launch.names.length, findings_in: tri.handed, entries: tri.entries.length, by_grade: byGrade, blockers: state.blockers.length, for_the_human: state.human_list.length, voided: unitStatus.filter((u) => u.status === 'void').map((u) => u.item) },
    human_list: state.human_list,
    forks: tri.forks,
    scope: { status: sc.status, uncovered: sc.uncovered || [], reached_but_excluded: sc.reached_but_excluded || [] },
    blockers: state.blockers,
    forbids_close: state.forbids_close,
  }, out)
}

// ---- the `fix` stage ----
async function runFix() {
  phase('State')
  const gs = await toolStep('state', 'State', 'git-state', gitStatePrompt(v))
  if (!gs || gs.status !== 'ready') return gitHalt('git', gs, 'the tree or the branches are not in the state the `fix` stage starts from')
  if (gs.pending) return await finishRecord(gs)
  let read = await readState()
  if (!read.state) return halt('state', 'the run\'s state could not be read: ' + read.error, { transient: !!read.transient, halt: read.halt || null })
  let state = read.state
  if (!state.opened) return halt('state', 'the run `' + v.run + '` has no opening record (' + runDir(v.run) + '/opening.md)')
  if (v.rulings && v.rulings.every(runRuling)) return await ruleTheRun(state, gs.branch)
  if (state.position.fix.refused) return { status: 'refused', stage: 'fix', run: v.run, refused: state.position.fix, message: 'the `fix` stage is refused (' + state.position.fix.refused + ', round ' + state.position.fix.round + '): ' + refusalOf(state.position.fix) + '. Nothing was run beyond the two reads.', next: state.next, not_ready: state.not_ready }
  const round = state.position.fix.round

  // The round's branch: the highest cut that exists, or none yet.
  const found = await toolStep('find:r' + round, 'State', 'find-round', findRoundPrompt(v, round))
  if (!found || found.status !== 'ready') return gitHalt('git', found, 'the round\'s branches could not be listed')
  const local = found.local || []
  const stray = (found.remote || []).filter((b) => currentCut(v.run, round, [b]) && !local.includes(b))
  if (stray.length) return halt('git', 'origin has a branch of round ' + round + ' that is not here: ' + stray.join(', ') + ' — fetch it; this stage does not guess which branch the round is on')
  let cut = currentCut(v.run, round, local)
  let branch = cut ? branchName(v.run, round, cut) : loopBranch
  let checkedOut = found.branch
  let tipSha = String(found.head || '')
  // onBranch — the round's branch checked out: switched to, or opened from the loop branch.
  async function onBranch(target) {
    if (checkedOut === target) return null
    const ob = await toolStep('open:' + target, 'State', 'open-round', openRoundPrompt(v, round, target))
    if (!onIt(ob, target, false)) return gitHalt('git', ob, 'the branch ' + target + ' could not be checked out')
    checkedOut = target
    tipSha = String(ob.head)
    return null
  }
  if (checkedOut !== branch) {
    if (!cut) return halt('git', 'round ' + round + ' has no branch yet, and `' + checkedOut + '` is checked out — not the loop branch ' + loopBranch)
    const stopped = await onBranch(branch)
    if (stopped) return stopped
    read = await readState()
    if (!read.state) return halt('state', 'the run\'s state could not be read on ' + branch + ': ' + read.error, { transient: !!read.transient })
    state = read.state
    if (state.position.fix.refused) return { status: 'refused', stage: 'fix', run: v.run, refused: state.position.fix, message: 'on `' + branch + '` the `fix` stage is refused (' + state.position.fix.refused + ').', next: state.next }
  }
  if (v.stopAfter === 'state') return { status: 'stopped', after: 'state', stage: 'fix', run: v.run, round, branch, state: attached(state) }

  // The human's rulings: ONE step, before anything is fixed.
  if (v.rulings) {
    const ruled = await rulingsStep(round, branch, {})
    if (ruled.halted) return ruled.halted
    state = ruled.state
  }
  if (v.stopAfter === 'rulings') return { status: 'stopped', after: 'rulings', stage: 'fix', run: v.run, round, branch, state: attached(state) }

  const report = { stage: 'fix', run: v.run, round }
  const blockersOf = (s) => s.ledger.filter((row) => s.blockers.includes(row.key))
  const cyclesDone = () => state.rounds.find((r) => r.round === round).facts.cycles
  const doorsForRetest = []
  const forks = []
  const couldNot = []

  // The exit `drop`: the round's record says so, its fixes are open again, and its record
  // commits — exactly those — are carried over to the loop branch.
  async function drop() {
    phase('Record')
    let commits = { records: [], fixes: [], mixed: [] }
    if (cut) {
      const listed = await toolStep('commits:r' + round, 'Record', 'round-commits', roundCommitsPrompt(v, round, branch))
      if (!listed || listed.status !== 'listed') return gitHalt('git', listed, 'the round\'s commits could not be listed')
      commits = classifyCommits(listed.all || [], listed.inside || [], listed.outside || [])
      if (commits.mixed.length) return halt('git', 'commit(s) of the round touch the run\'s directory and other paths at once, or nothing: ' + commits.mixed.join(', ') + ' — no carry-over can take them, and none is guessed')
    }
    const ctx = { run: v.run, round, stage: 'fix', cycle: state.position.fix.cycle, attempt: state.position.fix.attempt, scratch: v.scratch }
    const patches = reopenPatches(state.ledger, commits.fixes)
    const commands = stageRecordCommands(ctx, { reporters: [], rows: [], triage: [], patches, clauses: [], facts: { outcome: 'dropped' }, keys: patches.map((p) => p.key) })
    const recorded = await recordStep('drop:r' + round, branch, recordPrompt(v, 'round ' + round + ' is DROPPED, by the human\'s ruling at a bound — ' + patches.length + ' finding(s) whose fix does not land are open again', branch, v.scratch + '/record/drop-r' + round, round, commands, v.run + ' r' + round + ' — the round is dropped'))
    if (recorded.fault) return halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null })
    phase('Land')
    let carried = []
    if (cut) {
      const pushedRound = await toolStep('push:' + branch, 'Land', 'push', pushPrompt(v, branch))
      if (!onIt(pushedRound, branch, true)) return gitHalt('push', pushedRound, 'the dropped round\'s branch was not pushed')
      const take = commits.records.concat([recorded.result.commit])
      const carry = await toolStep('carry:r' + round, 'Land', 'carry', carryPrompt(v, round, take, null))
      if (!carry || carry.status !== 'carried' || carry.branch !== loopBranch || carry.remote_head !== carry.head || (carry.picked || []).length !== take.length) return gitHalt('carry', carry, 'the dropped round\'s record commits were not carried over to ' + loopBranch)
      carried = carry.picked
    } else {
      const pushed = await toolStep('push', 'Land', 'push', pushPrompt(v, loopBranch))
      if (!onIt(pushed, loopBranch, true)) return gitHalt('push', pushed, 'the drop is recorded on ' + loopBranch + ' and the branch was not pushed')
    }
    const after = await readState()
    if (!after.state) return halt('state', 'round ' + round + ' is dropped and its record is on ' + loopBranch + ', and the state could not be read back: ' + after.error, { transient: !!after.transient })
    return Object.assign({ status: 'dropped', dropped_branch: cut ? branch : null, reopened: patches.map((p) => p.key), carried, blockers: after.state.blockers, human_list: after.state.human_list }, report, nextOf(after.state))
  }
  if (v.exit === 'drop') return await drop()

  // The second half of a cycle, from the fixers' return on: the round's tip pushed and
  // built; per fork of the fixers an advocate and an independent drive — or, when there is
  // none, the audit of the fix diff; then triage, verify-real, the record on the round's
  // branch, the push, and the state read back. A cycle with a fork, and a cycle that fixed
  // nothing, is not audited in this invocation and is not counted: there is no new diff.
  async function secondHalf(ctx, launch, fixerSources, patches, fixerForks, doors, isPart) {
    phase('Fix')
    const head = await toolStep('push:' + branch + ':c' + ctx.cycle, 'Fix', 'push', pushPrompt(v, branch))
    if (!onIt(head, branch, true)) return { halted: gitHalt('push', head, 'the round\'s branch ' + branch + ' was not pushed') }
    // The fixers' own, and the rows whose triage nobody finished: this triage is the next.
    const sources = fixerSources.concat(ledgerSource(v.run, state))
    const driven = []
    const range = loopBranch + '..' + branch
    const audited = !fixerForks.length && (isPart || patches.length > 0)
    let built = null
    if (audited || fixerForks.length || sources.length) {
      const label = 'r' + round + 'c' + ctx.cycle
      const binary = v.scratch + '/bin/' + label + '.a' + ctx.attempt + '/jigc'
      const pre = await preflightOf(ctx, launch, ['preflight'], { build: true, image: false, sha: head.head, label, branch, binary, checks: [], crossModel: false, previous: previousOf(state) }, 'Fix')
      if (pre.fault) return { halted: halt('preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, branch }) }
      built = { candidate: { label, sha: head.head, binary, sha256: pre.result.candidate.sha256 }, previous: pre.result.previous, image: null }
    }
    if (fixerForks.length) {
      log('cycle ' + ctx.cycle + ': ' + fixerForks.length + ' fork(s) of the fixers — the fix diff is NOT audited in this invocation, and the cycle is not counted')
      for (const fork of fixerForks) {
        const d = await driveFork(ctx, launch, built, fork, 'fix')
        if (d.fault) return { halted: halt('binary', d.fault, { branch }) }
        driven.push(d.fork)
        for (const source of d.leftOpen) sources.push(source)
      }
    } else if (!audited) {
      log('cycle ' + ctx.cycle + ': NOTHING WAS FIXED — there is no new diff to audit, and the cycle is not counted')
    } else {
      const unit = { item: 'audit', clause: null, chain: FIX_AUDIT, doors, range, brief: 'The fix diff of round ' + round + ' as it stands at ' + head.head + ': every commit of `' + range + '` over the product paths. Whether each fix closes its finding\'s class, and what it breaks beside it.' }
      const ran = (await runUnits(ctx, launch, built, [unit], 'Fix'))[0]
      const wrong = hashMismatch(built.candidate.sha256, ran.reporters)
      if (wrong.length) return { halted: halt('binary', 'a driving agent of the audit did not assert the round tip\'s binary (' + built.candidate.sha256 + '): ' + wrong.map((w) => w.reporter + ' asserted ' + JSON.stringify(w.asserted)).join('; '), { mismatched: wrong, branch }) }
      if (ran.status !== 'green') return { halted: halt('audit', 'the audit of the fix diff did not run to its end (' + (ran.reporters.filter((r) => !r.result || r.result.status !== 'reported').map((r) => r.name).join(', ') || 'no reporter returned') + '): a round is not landed, and a cycle is not counted, on half an audit', { transient: ran.reporters.some((r) => !r.result), launched: launch.names, branch }) }
      for (const r of ran.reporters) {
        sources.push({ reporter: r.name, report: r.report, findings: findingLines(r.result.findings) })
        for (const d of r.result.doors_affected || []) if (!doorsForRetest.includes(d)) doorsForRetest.push(d)
      }
    }
    if (v.stopAfter === 'audit') return { halted: Object.assign({ status: 'stopped', after: 'audit', branch, candidate: built ? built.candidate : null, reporters: launch.names, sources: sources.map((x) => ({ reporter: x.reporter, findings: x.findings.length })), forks: driven }, report) }
    phase('Triage')
    const tri = await triagePasses(ctx, launch, built, sources, [])
    if (tri.fault) return { halted: halt('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, branch }) }
    if (tri.faults.length) return { halted: halt('binary', tri.faults.join('; '), { branch }) }
    phase('Record')
    const rec = triageRecord(ctx, tri.entries)
    const keys = tri.entries.map((e) => e.key)
    for (const patch of patches) if (!keys.includes(patch.key)) keys.push(patch.key)
    const commands = stageRecordCommands(ctx, { reporters: launch.names, rows: rec.rows, triage: rec.triage, patches, clauses: [], facts: audited ? { cycles: ctx.cycle } : null, keys })
    const recorded = await recordStep('fix:r' + round + ':c' + ctx.cycle, branch, recordPrompt(v, 'the record of round ' + round + '\'s fix cycle ' + ctx.cycle + ' — ' + launch.names.length + ' report(s), ' + patches.length + ' ledger patch(es), ' + tri.entries.length + ' finding(s) triaged' + (audited ? '' : '; the fix diff was not audited, and the cycle is not counted'), branch, v.scratch + '/record/fix-r' + round + '-c' + ctx.cycle + '-a' + ctx.attempt, round, commands, v.run + ' r' + round + ' c' + ctx.cycle + ' — the fix cycle\'s record'))
    if (recorded.fault) return { halted: halt('record', recorded.fault, { transient: !recorded.result, halt: recorded.result ? recorded.result.halt : null, launched: launch.names, branch }) }
    const pushed = await toolStep('push:' + branch + ':c' + ctx.cycle + ':record', 'Record', 'push', pushPrompt(v, branch))
    if (!onIt(pushed, branch, true)) return { halted: gitHalt('push', pushed, 'cycle ' + ctx.cycle + ' is recorded on ' + branch + ' (' + recorded.result.commit + ') and the branch was not pushed') }
    const after = await readState()
    if (!after.state) return { halted: halt('state', 'cycle ' + ctx.cycle + ' is recorded (' + recorded.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, branch }) }
    state = after.state
    for (const fork of driven.concat(tri.forks)) forks.push(fork)
    return { audited, fixerForks: driven, record: recorded.result.commit }
  }

  // land — the round's branch merged into the loop branch `--no-ff`, and the loop branch pushed.
  async function land(extra) {
    phase('Land')
    const landed = await toolStep('land:r' + round, 'Land', 'land', landPrompt(v, round, branch))
    const merged = !!landed && landed.status === 'merged' && SHA_RE.test(String(landed.merge_commit || '')) && landed.remote_head === landed.merge_commit
    const before = !!landed && landed.status === 'landed-before' && SHA_RE.test(String(landed.head || ''))
    if (!merged && !before) return gitHalt('land', landed, landed && landed.merge_commit ? 'round ' + round + ' IS merged into ' + loopBranch + ' locally (' + landed.merge_commit + '), and what followed the merge stopped' : 'round ' + round + ' was not landed; it is unlanded on ' + branch)
    const after = await readState()
    if (!after.state) return halt('state', 'round ' + round + ' is landed (' + (landed.merge_commit || landed.head) + '), and the state could not be read back: ' + after.error, { transient: !!after.transient })
    return Object.assign({ status: 'landed', candidate: { label: 'c' + (round + 1), sha: merged ? landed.merge_commit : landed.head }, landed_before: before, landed_branch: branch, cycles: after.state.rounds.find((r) => r.round === round).facts.cycles, counts: { blockers: after.state.blockers.length, for_the_human: after.state.human_list.length, forks: forks.length }, doors_for_retest: doorsForRetest, tip_moved: !!landed.tip_moved, resolved_logs: landed.resolved_logs || [], moved_outside: landed.moved_outside || [], forks }, extra || {}, report, nextOf(after.state))
  }
  // unlanded — a stage that stops with the round not landed: at the bound, or on a fork.
  function unlanded(phaseName, why) {
    return Object.assign(halt(phaseName, why, { branch, cycles: cyclesDone(), limit: cycleLimit, blockers: state.blockers, human_list: state.human_list, forks, could_not_fix: couldNot }), { round }, phaseName === 'bound' ? { exits: { continue: 'args.raise = { cycles: ' + (Math.max(cycleLimit, cyclesDone()) + 1) + ' } — or more; passed on every later `fix` of this round', drop: 'args.exit = \'drop\'', part: 'args.exit = { part: [<the fix commits of ' + branch + ' to keep>] }' } } : {})
  }

  // The exit `part`: the fix commits the human keeps, and the round's record commits, re-cut
  // as a branch tip of their own; then gated — the record step's gate runs on exactly that
  // tree — and audited once more as exactly that diff, and landed if that audit is clean.
  if (v.exit) {
    if (!cut) return halt('git', 'round ' + round + ' has no branch: there is nothing a part could be cut from')
    phase('Fix')
    const listed = await toolStep('commits:r' + round, 'Fix', 'round-commits', roundCommitsPrompt(v, round, branch))
    if (!listed || listed.status !== 'listed') return gitHalt('git', listed, 'the round\'s commits could not be listed')
    const commits = classifyCommits(listed.all || [], listed.inside || [], listed.outside || [])
    if (commits.mixed.length) return halt('git', 'commit(s) of the round touch the run\'s directory and other paths at once, or nothing: ' + commits.mixed.join(', ') + ' — no part can be cut around them, and none is guessed')
    const strangers = v.exit.part.filter((sha) => commits.fixes.filter((f) => sameCommit(f, sha)).length !== 1)
    if (strangers.length) return halt('git', 'args.exit.part names ' + strangers.join(', ') + ', which is not exactly one fix commit of `' + branch + '` — its fix commits are: ' + (commits.fixes.join(', ') || '(none)'))
    const kept = commits.fixes.filter((f) => v.exit.part.some((sha) => sameCommit(f, sha)))
    const left = commits.fixes.filter((f) => !kept.includes(f))
    const take = (listed.all || []).filter((sha) => kept.includes(sha) || commits.records.includes(sha))
    const part = branchName(v.run, round, cut + 1)
    const carry = await gitStep('cut:' + part, 'Fix', carryPrompt(v, round, take, part), CARRY_SCHEMA)
    if (!carry || carry.status !== 'carried' || carry.branch !== part || carry.remote_head !== carry.head || (carry.picked || []).length !== take.length) return gitHalt('carry', carry, 'the part was not cut as ' + part)
    // What the part changes in the ledger: a kept fix names its new commit; a fix left out
    // never lands, so its finding is open again. Those are the human's choice, and are not
    // what the part's audit is asked about: a blocker it must not add is one that is new.
    const reopen = reopenPatches(state.ledger, left)
    const patches = repointPatches(state.ledger, carry.picked).concat(reopen)
    const known = state.blockers.concat(reopen.map((p) => p.key))
    const doors = (state.doors ? state.doors.included.concat(state.doors.excluded) : []).filter((d) => state.ledger.some((row) => row.door === d.door && (row.disposition === 'fixed' || state.blockers.includes(row.key))))
    cut = cut + 1
    branch = part
    checkedOut = part
    const ctx = { run: v.run, round, stage: 'fix', cycle: state.position.fix.cycle, attempt: state.position.fix.attempt, scratch: v.scratch }
    const done = await secondHalf(ctx, launcher(ctx), [], patches, [], doors, true)
    if (done.halted) return done.halted
    const fresh = state.blockers.filter((key) => !known.includes(key))
    const cutAs = { branch: part, kept, left, reopened: reopen.map((p) => p.key) }
    if (fresh.length || state.human_list.length || forks.length) return Object.assign(unlanded('bound', 'the part was audited as exactly its diff, and the audit is not clean: ' + fresh.length + ' new blocker(s)' + (fresh.length ? ' (' + fresh.join(', ') + ')' : '') + ', ' + state.human_list.length + ' item(s) for the human, ' + forks.length + ' fork(s) — it is unlanded, on ' + part), { part: Object.assign({ new_blockers: fresh }, cutAs), record: done.record }, nextOf(state))
    const closing = stageRecordCommands(ctx, { reporters: [], rows: [], triage: [], patches: [], clauses: [], facts: { outcome: 'part' }, keys: [] })
    const said = await recordStep('part:r' + round, part, recordPrompt(v, 'a PART of round ' + round + ' lands, by the human\'s ruling at a bound — ' + kept.length + ' fix commit(s) kept, ' + left.length + ' left out and their finding(s) open again', part, v.scratch + '/record/part-r' + round, round, closing, v.run + ' r' + round + ' — a part of the round lands'))
    if (said.fault) return halt('record', said.fault, { transient: !said.result, halt: said.result ? said.result.halt : null, branch: part })
    return await land({ part: cutAs })
  }

  // The cycles.
  let finished = false
  for (;;) {
    const at = state.position.fix
    if (at.refused) return Object.assign({ status: 'refused', refused: at, message: 'the `fix` stage is refused (' + at.refused + ', round ' + at.round + '): ' + refusalOf(at) + '.' }, report, nextOf(state))
    const open = { branch: cut ? branch : null, cycles: cyclesDone(), blockers: state.blockers, human_list: state.human_list, untriaged: state.untriaged, forks }
    // The round's triage is not finished, and this stage left it: finish it — once — and
    // go on from the state that leaves. No fixer and no auditor runs for it.
    if (at.triage) {
      if (finished) return Object.assign({ status: 'cycle', message: 'round ' + round + '\'s triage was finished in this invocation and the state still names findings without a grade or a verdict: it is not run again on a guess — `next` says what is owed.' }, open, report, nextOf(state))
      if (!cut) return halt('git', 'the state says that the `fix` stage left round ' + round + '\'s triage unfinished, and the round has no branch: there is no tip a verifier could drive')
      const stopped = await onBranch(branch)
      if (stopped) return stopped
      const done = await finishTriage({ run: v.run, round, stage: 'fix', cycle: at.cycle, attempt: at.attempt, scratch: v.scratch }, state, { sha: tipSha, label: 'r' + round + 'c' + at.cycle, tested: false }, branch)
      if (done.halted) return done.halted
      state = done.state
      for (const fork of done.forks) forks.push(fork)
      finished = true
      continue
    }
    if (at.land) {
      if (!cut) return Object.assign({ status: 'nothing-to-fix', message: 'round ' + round + ' has nothing open and no branch: nothing is fixed and nothing lands.' }, open, report, nextOf(state))
      const stopped = await onBranch(branch)
      if (stopped) return stopped
      return await land()
    }
    if (!state.blockers.length) return Object.assign({ status: cut ? 'cycle' : 'nothing-to-fix', message: 'no finding of round ' + round + ' is routed `fix`' + (cut ? ', and the round is not landed: something of it is still open, and it is not a fixer\'s' : '') + ' — `next` says what is owed.' }, open, report, nextOf(state))
    if (state.next !== 'fix') return Object.assign({ status: 'cycle', message: 'round ' + round + ' has ' + state.blockers.length + ' blocker(s), and the state\'s `next` is not `fix`: what it names comes first.' }, open, report, nextOf(state))
    if (at.cycle > cycleLimit) return unlanded('bound', 'round ' + round + ' has ' + cyclesDone() + ' recorded fix -> audit cycle(s) and ' + state.blockers.length + ' blocker(s) left: the bound of ' + cycleLimit + ' is reached, and the round is unlanded' + (cut ? ' on ' + branch : '') + ' (ruling 6)')

    // Cycle `at.cycle`: the round's branch, then the fixers — one area at a time, serial.
    phase('Fix')
    if (!cut) {
      cut = 1
      branch = branchName(v.run, round, cut)
    }
    const stopped = await onBranch(branch)
    if (stopped) return stopped
    const ctx = { run: v.run, round, stage: 'fix', cycle: at.cycle, attempt: at.attempt, scratch: v.scratch }
    const launch = launcher(ctx)
    const blockers = blockersOf(state)
    const unnamed = blockers.filter((b) => !forkNames('fix', b.key)).map((b) => b.key)
    if (unnamed.length) return halt('state', 'no reporter name can be made for a fork of ' + unnamed.join(', ') + ': the key is too long to carry a prefix inside ' + SLUG_MAX + ' characters', { branch })
    const areas = areasOf(blockers, state.doors)
    log('round ' + round + ', cycle ' + ctx.cycle + ' of at most ' + cycleLimit + ' (attempt ' + ctx.attempt + '): ' + blockers.length + ' blocker(s) in ' + areas.length + ' area(s), on ' + branch)
    const fixerSources = []
    const patches = []
    const fixerForks = []
    for (const area of areas) {
      const name = launch.add(['fix', 'area', String(area.n)])
      const r = await roleStep('fixer', 'fix:r' + round + ':c' + ctx.cycle + ':area' + area.n, 'Fix', fixerPrompt(ctx, launch, name, area, branch), FIXER_SCHEMA)
      if (!r || r.status !== 'worked') return halt('fix', r ? 'the fixer of area ' + area.n + ' (' + area.registry + ') halted: ' + ((r.halt && r.halt.root_cause) || 'no reason given') : 'the fixer of area ' + area.n + ' (' + area.registry + ') returned no result — what it left is UNCOMMITTED in the tree, and what it committed is on ' + branch, { transient: !r, halt: r ? r.halt : null, branch, launched: launch.names })
      const handedKeys = area.findings.map((f) => f.key)
      const returned = r.entries.map((e) => e.key)
      if (returned.length !== handedKeys.length || !distinct(returned) || returned.some((key) => !handedKeys.includes(key))) return halt('fix', 'the fixer of area ' + area.n + ' was handed ' + handedKeys.join(', ') + ' and returned entries for ' + (returned.join(', ') || '(none)') + ' — one entry per finding handed, and no other', { branch, launched: launch.names })
      const leftOpen = []
      for (const e of r.entries) {
        const finding = area.findings.find((f) => f.key === e.key)
        if (e.status === 'fixed') {
          if (!SHA_RE.test(String(e.commit || ''))) return halt('fix', 'the fixer of area ' + area.n + ' reported `' + e.key + '` fixed and named no commit by its full sha', { branch, launched: launch.names })
          patches.push({ key: e.key, disposition: 'fixed', detail: e.commit })
        } else if (e.status === 'could-not-fix') {
          couldNot.push({ key: e.key, cycle: ctx.cycle, notes: e.notes || null })
        }
        if (e.status === 'halted') {
          fixerForks.push({ key: e.key, kind: 'new-mechanism', door: finding.door, clause: finding.clause, repro: finding.repro, statement: 'the fixer halted on it — ' + ((e.halt && e.halt.root_cause ? e.halt.root_cause + (e.halt.recommendation ? ' · ' + e.halt.recommendation : '') : '') || '(no halt report returned)') })
        }
        for (const d of e.doors_affected || []) if (!doorsForRetest.includes(d)) doorsForRetest.push(d)
        for (const text of e.left_open || []) leftOpen.push('`' + e.key + '`: ' + text)
      }
      if (leftOpen.length) fixerSources.push(leftOpenSource(name, r.report, leftOpen))
    }
    if (v.stopAfter === 'fixers') return Object.assign({ status: 'stopped', after: 'fixers', branch, fixed: patches, forks: fixerForks, reporters: launch.names }, report)

    const fixDoors = (state.doors ? state.doors.included.concat(state.doors.excluded) : []).filter((d) => blockers.some((b) => b.door === d.door) || doorsForRetest.includes(d.door))
    const done = await secondHalf(ctx, launch, fixerSources, patches, fixerForks, fixDoors, false)
    if (done.halted) return done.halted
    if (!patches.length && !done.fixerForks.length) return Object.assign(unlanded('fix', 'no fixer fixed anything in cycle ' + ctx.cycle + ' of round ' + round + ' (' + couldNot.filter((c) => c.cycle === ctx.cycle).map((c) => c.key).join(', ') + ' could not be fixed) — the same fixers on the same findings are not sent again; each `could_not_fix` entry carries its fixer\'s notes, and the cycle is not counted'), { record: done.record }, nextOf(state))
    if (done.fixerForks.length) return Object.assign(unlanded('fork', 'a fixer halted on a fix that needs a new mechanism, or that would revise a settled decision: ' + done.fixerForks.map((f) => f.key).join(', ') + ' — each comes with an advocate\'s case and an independent drive of the proposal. The human rules; the next `fix` carries the ruling in args.rulings'), { fixed: patches.map((p) => p.key), record: done.record }, nextOf(state))
  }
}

return await (v.stage === 'test' ? runTest() : runFix())
