// GENERATED — never edit this file by hand. It is .claude/workflow-sources/stabilize.js
// with its comments removed, written by `dev/stabilize-harness`: edit the source, then
// run that command (implementation/stabilization-workflow.md -> The harness's source).

export const meta = {
  name: 'stabilize',
  description: 'One stage of a stabilization run. test: preflight, scope, the round\'s instruments, triage, verify-real, the record. fix: the human\'s rulings, fixers, the audit of the fix diff (at most 3 cycles), land — or the exits at a bound: continue, drop, land a part. Reads the round, the cycle and the next step from dev/stabilize-record state. Args: { stage, run, scratch, scope?, clause?, crossModel?, rulings?, raise?, exit?, stopAfter?, selfTest?, model? }. Or one runtime probe, on no run: { probe, scratch, seconds? }.',
  phases: [
    { title: 'Probe' },
    { title: 'State' },
    { title: 'Preflight and scope' },
    { title: 'Instruments' },
    { title: 'Fix' },
    { title: 'Triage' },
    { title: 'Verify' },
    { title: 'Record' },
    { title: 'Land' },
  ],
}

const STAGES = ['test', 'fix']
const ARGS = ['stage', 'run', 'scratch', 'scope', 'clause', 'crossModel', 'rulings', 'raise', 'exit', 'stopAfter', 'selfTest', 'model', 'probe', 'seconds']
const STOPS = { test: ['state', 'preflight', 'instruments', 'triage'], fix: ['state', 'rulings', 'fixers', 'audit'] }
const SLUG_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/
const SLUG_MAX = 100
const SCRATCH_RE = /^(\/[A-Za-z0-9._-]+)+$/
const SHA_RE = /^[0-9a-f]{40}$/
const SHORT_SHA_RE = /^[0-9a-f]{7,40}$/
const SHA256_RE = /^[0-9a-f]{64}$/
const RANGE_RE = /^[0-9a-f]{7,40}\.\.[0-9a-f]{7,40}$/
const MODEL_RE = /^[a-z][a-z0-9.-]*$/
const DEFAULT_CYCLES = 3
const MAX_CYCLES = 99
const MAX_ROUNDS = 99
const KNOWN_NEXT = ['fix', 'rule', 'close', 'verify']
const VERIFY_PASSES = 3
const WAVE = 8
const SILENT_LAST = 2
const FILED_WORDS = ['confirmed', 'refuted', 'undrivable']
const GIT_MODEL = 'sonnet'
const STEP_TOOL = 'dev/stabilize-step'
const RECORD_TOOL = 'dev/stabilize-record'
function typed(tool, words) {
  return '`' + tool + (words ? ' ' + words : '') + '`'
}
const REGRESSION_LIST = 'completions/artifacts/M55/stabilization-build/regression-set/intended-changes.tsv'
function baselineOf(previous) {
  return 'baseline/previous-' + previous.slice(0, 12)
}
function regressionFlags(previous, candidate) {
  return '--previous ' + previous + ' --candidate ' + candidate + ' --list ' + REGRESSION_LIST + ' --baseline ' + baselineOf(previous)
}
const HOLD_LAPS = 3
const HOLD_NAMES = 9
const WORDS_AGAIN = 1
const RUNS_ROOT = 'completions/artifacts'
const PRODUCT_PATHS = ['Cargo.toml', 'Cargo.lock', 'crates']
const SYNC_LOGS = ['DECISIONS.md', 'implementation/project-history.md']
const LABELS = { report: 'REPORT:', binary: 'BINARY:', area: 'AREA:', record: 'RECORD STEP', branch: 'BRANCH:' }
const HUMAN_RULINGS = ['admitted', 'bound', 'later']
const PAYLOAD_ENDS = 'STABILIZE_PAYLOAD'
const PROBE_TOOL = 'dev/stabilize-probe'
const PROBES = ['required', 'relay', 'relay-document', 'payload', 'hold']
const PROBE_ROWS = [1, 20, 60, 120]
const PROBE_RELAYS = 3
const PROBE_ENTRIES = [20, 60, 120, 300]
const PROBE_HOLD = 720
const PROBE_HOLD_MAX = 3600
const PROBE_SLICE = 90
const LEDGER_SOURCE = 'ledger'
const RED_PREFIX = 'red-'
const ATTEMPT = 'attempt'
const STEP_REFUSALS = {
  'usage': { whose: 'the harness\'s', leaves: 'the command this script composed is not one the tool takes, or names a scratch root the tool cannot use: nothing ran. It is a defect of the harness or of the invocation\'s `scratch`, and the step\'s own line says which argument' },
  'wrong-branch': { whose: 'the orchestrator\'s', leaves: 'the branch the step names is not the one checked out: nothing was read past its name, and nothing was written, fetched or pushed. The branch the step names is checked out, and the stage invoked again' },
  'dirty': { whose: 'the human\'s', leaves: 'the tree holds what is no pending write of the run — a tracked file changed by hand, a staged path that is no open record\'s, a stray file — or a file of an applied batch that a hand changed: the human reconciles it, and no step is repeated on a guess. For a changed batch `dev/stabilize-step discard` takes the batch back; the step\'s own line names the file' },
  'remote-ahead': { whose: 'the human\'s', leaves: 'the pushed branch has commits the local one lacks: the human reconciles the two — no step pulls, merges or rebases' },
  'product-path': { whose: 'the human\'s', leaves: 'a commit changed a product path directly on the loop branch: a product path moves only through an audited round\'s merge' },
  'foreign-merge': { whose: 'the human\'s', leaves: 'a merge on the loop branch is not a round\'s: only a round\'s branch is merged into it' },
  'merge-in-round': { whose: 'the human\'s', leaves: 'a round\'s branch holds a merge: its commits are a fixer\'s and a record\'s, one by one' },
  'conflict': { whose: 'the human\'s', leaves: 'a merge stopped on a path that is no append-only log: it was aborted, and the conflict is the human\'s' },
  'merge-logs': { whose: 'the human\'s', leaves: '`dev/merge-logs` could not resolve the logs a merge stopped on: the merge was aborted' },
  'unresolved': { whose: 'the human\'s', leaves: 'a merge is left in progress with a path still conflicted: it was not aborted, and the human finishes or aborts it' },
  'merge-differs': { whose: 'the human\'s', leaves: 'the merge commit is not the merge that was asked for — its parents, its tree or what it moved: nothing was undone' },
  'pick-stopped': { whose: 'the human\'s', leaves: 'a cherry-pick stopped, and was aborted' },
  'carry-differs': { whose: 'the human\'s', leaves: 'what was carried over is not exactly the commits named: nothing was undone' },
  'push-rejected': { whose: 'the orchestrator\'s', leaves: 'the remote did not take the push — or none was made, because a tag or another ref answers to the branch\'s name: nothing is lost — the commit is local — and the push is owed' },
  'remote-differs': { whose: 'the orchestrator\'s', leaves: 'the remote, or the branch, did not stand where the push held it. One of them moved while the range was vetted, and nothing was pushed; or the commit that was vetted IS published — by its sha, and nothing later — and the remote stands elsewhere, or a commit that landed on the branch meanwhile is not on the remote. The push is owed, and the next first read looks again' },
  'record': { whose: 'the orchestrator\'s', leaves: '`dev/stabilize-record` refused, or its answer could not be read: the step\'s own line holds that script\'s refusal — the tree is as that script left it, which is whole or untouched' },
  'git': { whose: 'the orchestrator\'s', leaves: 'a git command of the step failed: the step\'s own line holds the command and what it printed, and nothing after it ran' },
  'gate-red': { whose: 'the orchestrator\'s', leaves: 'the gate that ran on the tree with the records shows red that the candidate\'s own gate did not: nothing was committed or undone, and the batch stays applied — the step\'s own line names what is new' },
  'no-batch': { whose: 'the orchestrator\'s', leaves: 'the run has no applied batch of the calls and checks this script composed — none was applied, or the one that is there is another\'s: nothing was committed' },
  'position': { whose: 'the orchestrator\'s', leaves: 'the run\'s state does not hand the stage this round and this attempt — another invocation holds it, or the state moved since it was read: nothing was written, and the stage invoked again reads the state as it stands' },
  'unvetted': { whose: 'the human\'s at a push; at a record\'s commit, the next attempt\'s', leaves: 'a file, a subject or a commit is not what `dev/stabilize-record` would have written. At a record\'s commit nothing is committed and the batch is taken back; at a push nothing is pushed, and the commit is local and the human\'s to take back or rewrite — `dev/stabilize-record vet --range -- <branch> --not --remotes=origin` names every commit and place again' },
  'did-not-run': { whose: 'the orchestrator\'s', leaves: 'a scanner could not run — gitleaks, or the denylist: that is a refusal and never a pass, and everything is as it was. The scanner is provided, and the stage invoked again' },
  'locked': { whose: 'the orchestrator\'s', leaves: 'git\'s own lock stands in the repository, and the tool never removes one. With no git process running there, the orchestrator removes the file the step\'s own line names and invokes the stage again; while one runs, once it is done' },
  'fence-moved': { whose: 'the human\'s', leaves: 'THE REPOSITORY WAS WRITTEN PAST THE WRITE GUARD while the stage ran — its configuration, a hook, an info file, a local ref or a reflog\'s newest entry is not what the stage\'s first read saw, and no step of a stage writes any of them before its record: nothing was committed or undone, and the batch stays applied. The human finds what moved — `git reflog -5`, `git config --local --list`, the hooks directory, `git for-each-ref` — puts back what must be put back, and invokes the stage again: its first read takes the guarded state as it then stands' },
  'head-moved': { whose: 'the orchestrator\'s', leaves: 'the branch is not at the commit the stage began on — somebody committed while it ran: nothing was committed or undone, and the batch stays applied. The stage is invoked again: its first read meets that commit, then gates the batch on the tree as it now stands and commits it there' },
  'foreign-commit': { whose: 'its author\'s', leaves: 'the loop branch is ahead of its remote by a commit that changes a path outside the run\'s directory, or by a merge: some task\'s commit and no record\'s. It is NEITHER THE HUMAN\'S NOR A STAGE\'S: its author — or the orchestrator, for that task — pushes it by name, `git push origin <branch>`, behind that task\'s own gate, and the stage is invoked again. Nothing was pushed, not the records beside it either' },
  'committed': { whose: 'the orchestrator\'s', leaves: 'the applied batch is in a commit already and is not taken back: the next first read settles it' },
  'taken': { whose: 'the harness\'s', leaves: 'a held command of that name was started with other arguments, or its work directory holds a build: a name is minted once per attempt' },
  'missing': { whose: 'the harness\'s', leaves: 'no held command of that name was started under this scratch root, or the file asked for is not there' },
  'build': { whose: 'the orchestrator\'s', leaves: 'the commit did not build, or its binary is not of the version asked: no binary was written' },
}
function stepRefusal(word, branch) {
  const row = STEP_REFUSALS[word]
  return row ? '`' + word + '` — ' + row.whose + ': ' + row.leaves.replace('<branch>', branch || '<branch>') : '`' + word + '`, a word of `' + STEP_TOOL + '` this script has no row for'
}
const HELD_PREFIX = 'held-'
const HELD_CHECKS = ['gate', 'regression']
function heldKind(kind) {
  const id = typeof kind === 'string' && kind.startsWith(HELD_PREFIX) ? kind.slice(HELD_PREFIX.length) : null
  return id && HELD_CHECKS.includes(id) ? id : null
}
const HELD_ENDS = {
  running: 'every agent that asked after it in turn ended its turn while the command still ran',
  green: 'the command ran to its end, and the tool read its verdict green',
  red: 'the command ran to its end, and the tool read its verdict red',
  void: 'the command ran to its end, and its output holds no verdict the tool takes',
  dead: 'the command\'s process is gone and nothing says how it ended: it has no verdict, whatever its output holds, and is never started again under its name',
  unread: 'the step that asked did not answer with a job of the tool\'s, and nothing is known of how the command stands',
}
const DEAD_WHY = {
  'no-record': 'the name holds no record of the tool\'s — a start under it was killed before it wrote one, or no start made its directory: nothing says what was asked under it, so nothing of it is read and it has no verdict, and nothing is ever started under it again. A command MAY STILL RUN under it, unrecorded — the step\'s own line says whether its supervisor lives (`supervised`)',
}
function heldEnded(job) {
  return (job.ends === 'dead' && DEAD_WHY[job.why]) || HELD_ENDS[job.ends] || 'it ended `' + job.ends + '`'
}
const HELD_TAKES = {
  build: ['green'],
  gate: ['green', 'red'],
  check: ['green', 'red', 'void'],
  record: ['green', 'red'],
  probe: ['green'],
}
const HELD_VOID_RE = /^[a-z0-9]+(?:-[a-z0-9]+)*(?::[A-Za-z0-9._-]+)?$/
function heldVoid(job) {
  const word = job.ends === 'not-run' ? 'off-tree' : job.refused || job.ends
  return word + (job.name ? ':' + job.name : '')
}
function mayRun(job) {
  return !!job && !!job.name && (job.ends === 'running' || (job.ends === 'unread' && job.started === true))
}
function heldFault(what, job) {
  if (job.exhausted) return what + ' was not held: ' + job.why + '.'
  return what + (job.name ? ' (the held command `' + job.name + '`)' : '') + ' did not end as the stage needs it: ' + heldEnded(job) + (job.why ? ' — `' + job.why + '`' : '') + '.'
}
function heldThen(scratch, job) {
  if (!mayRun(job)) return null
  return ' THE COMMAND MAY STILL RUN, in a session of its own, and nothing ends it but its own end: ' + typed(STEP_TOOL, 'hold-wait --scratch ' + scratch + ' --name ' + job.name) + ' answers once it is over' + (job.output ? ', and its whole output is ' + job.output : '') + '. Invoke the stage again AFTER THAT, with the same args — two gates at once contend for one lock: the next attempt names and starts its own command, and takes nothing over from this one.'
}
function heldSaid(job) {
  return { name: job.name || null, kind: job.kind || null, ends: job.ends, why: job.why || null, output: job.output || null }
}
const NEVER = "**Never push to or merge into `main`, force-push any branch, merge a pull request, push a tag, approve or reject a deployment, or yank a crate.** Those acts are the human's ([release.md](../../implementation/release.md) → *What agents may not do*). You may merge an increment branch into its milestone branch locally, and push `milestone/*`, `fix/*` and `work/*` branches — always by name (`git push origin <branch>`), never a bare `git push`, and never forced (no `--force`, `--force-with-lease`, `-f` or `+` refspec). `.claude/settings.json` denies the commands that perform the human's acts; a denial is the answer, never something to route around."

const GUARD = 'THIS RUN\'S WRITE GUARD: no agent of it but its git steps and its fixers writes this repository or its git directory. A file you write goes under your scratch root, and what reaches the run\'s record reaches it through the record script\'s one plain call. A call of ' + STEP_TOOL + ', ' + RECORD_TOOL + ' or ' + PROBE_TOOL + ' is taken only as ONE plain invocation, typed by itself, from the repository\'s root, exactly as this prompt spells it — nothing before it and nothing after it on the line, its exit status its own; any other command of yours runs under the write fence, where every write under this repository is refused.'

const ROLES = {
  preflight: { agentType: 'stabilize-preflight', labels: ['report'], drives: false },
  scope: { agentType: 'stabilize-scope', labels: ['report'], drives: false },
  review: { agentType: 'milestone-code-reviewer', labels: ['report', 'binary'], drives: true },
  drive: { agentType: 'milestone-e2e-tester', labels: ['report', 'binary'], drives: true },
  triage: { agentType: 'finding-triage', labels: ['report'], drives: false },
  verify: { agentType: 'finding-verifier', labels: ['report', 'binary'], drives: true },
  advocate: { agentType: 'robust-advocate', labels: ['report', 'binary'], drives: true },
  proposal: { agentType: 'general-purpose', labels: [], drives: true },
  crossModel: { agentType: 'general-purpose', labels: [], drives: false },
  fixer: { agentType: 'build-fixer', labels: ['report', 'area', 'branch'], drives: false },
  record: { agentType: 'build-executor', labels: ['record', 'branch'], drives: false },
}

const CHECK_KIND = 'check'
const RED_DOCTYPE = 'jigc-feedback'
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
function chainOf(kind, crossModel) {
  const chain = (CHAINS[kind] || []).map((stepList) => stepList.filter((s) => !s.crossModel || crossModel === true))
  const present = chain.reduce((all, stepList) => all.concat(stepList.map((s) => s.as)), [])
  return chain.map((stepList) => stepList.map((s) => (s.hands ? Object.assign({}, s, { hands: s.hands.filter((h) => present.includes(h)) }) : s)))
}
const FIX_AUDIT = [[{ as: 'review', role: 'review', task: 'the review of this round\'s FIX DIFF' }, { as: 'drive', role: 'drive', task: 'the drive of this round\'s FIX DIFF: every door it can change' }]]

const ASSERTED = 'the hash it asserted for the binary it drove: without it nothing it drove is evidence about this candidate'
const NEEDS = {
  review: [{ field: 'asserted_sha256', then: 'void', why: ASSERTED }],
  drive: [{ field: 'asserted_sha256', then: 'void', why: ASSERTED }],
  verify: [
    { field: 'verdict', then: 'unverified', why: 'what it found: a verifier that returns none verified nothing' },
    { field: 'asserted_sha256', then: 'unverified', why: ASSERTED },
    { field: 'regression', with: 'confirmed', then: 'unknown', why: 'whether the finding it confirmed is a regression: the verifier\'s fact, and nobody else\'s' },
    { field: 'ran_on.previous', with: 'confirmed', then: 'unknown', why: 'the hash it asserted for the previous release\'s binary: what a regression fact is backed by' },
  ],
  advocate: [{ field: 'asserted_sha256', then: 'no-fork', why: ASSERTED }],
  proposal: [{ field: 'asserted_sha256', then: 'no-fork', why: ASSERTED }],
}
const HALTS_AFTER = {
  'names': { phase: 'state', when: 'a reporter name cannot be made for a step of an item the round runs: the item\'s id is too long to carry its step inside a slug' },
  'report-check': { phase: 'reports', when: 'the report check after the instruments could not be read — its step died, refused, or the breaker is tripped — or it names a file no launched reporter wrote. IN A RUN WITH A QUEUE, the same of the report check after a wave of verifiers: the first record is committed, and what was filed is on disk — and of the report check that comes before a record no wave came before, in an invocation that continues a stage only for its record: nothing was composed' },
  'scope-aside': { phase: 'scope', when: 'the round\'s scope was set aside by the report check: the file at its path is not what the record script would have written' },
  'stood-on': { phase: 'reports', when: 'a reporter the stage stands on left no report: the attempt\'s marker, the first preflight, the scope step, a second preflight that provided, the preflight of a finishing lap' },
  'triage': { phase: 'triage', when: 'ANY ending of triage but its grades with its report — no result, a halt, counts that do not balance, a key that is none or is given twice, no report — and, inside a pass, a verifier\'s name that cannot be made and a report check that could not be read. IN A RUN WITH A QUEUE, any ending of the one triage but its entries handed over, with their rendered report: a halt, a return that names another path than that report\'s or no hash, a call typed with other counts than the stage handed — and the same of the triage an invocation that continues such a stage launches over the rows nobody graded' },
  'record': { phase: 'record', when: 'the record step: the executor applied nothing or its batch was refused, the record\'s gate left no verdict, or the commit step refused — the batch stays applied where it was, and the next invocation finishes it. IN A RUN WITH A QUEUE, where the tool composes the batch: the composing step returned nothing, or was refused — by a word of the record script (COMPOSE_REFUSALS; `words` only once it was asked for again), with nothing written, or by a word of the tool. Of such a run\'s SECOND record the same, but for `no-change`, which is no halt: nothing was filed, and there is nothing to record. Of the records of an invocation that continues the same: of what it graded as of a first, of what its verifiers filed as of a second' },
  'push': { phase: 'push', when: 'the push of the record\'s commit failed or was refused: the record is committed, the push is owed, and the next invocation\'s first read makes it' },
  'read-back': { phase: 'state', when: 'the state could not be read back once the record was committed and pushed: nothing is lost, and the next invocation reads it. IN A RUN WITH A QUEUE, also the read of the queue after a wave of verifiers: what was filed is on disk, and the state the next invocation reads counts it' },
}

const VOID_WORDS = {
  'silent': 'a step of the item\'s chain returned nothing, after its retries',
  'halted': 'a step of the item\'s chain halted',
  'unasserted': 'a step of the item\'s chain drove, and did not return the hash of the candidate\'s binary as the one it asserted',
  'unreported': 'a step of the item\'s chain returned, and left no report',
  'set-aside': 'the file at the report\'s path of a step of the item\'s chain was no report, and was set aside',
  'chain-lost': 'the item\'s chain did not run: nothing came back of it',
  'no-image': 'a trial arm, and the second preflight provided no verified trial image',
  'off-tree': 'a check that reads the working tree, in a re-run: the tree is no longer the round\'s candidate, and the check was not run',
  'other-commit': 'the check ran on another commit than the candidate, or on one the preflight did not name',
  'could-not-run': 'the preflight could not run the check, and says so',
  'not-provided': 'a check the scope selected, and the second preflight did not provide it',
  'unanswered': 'the preflight returned nothing for the check',
}
function voidWord(word, key) {
  return word + (key ? ':' + key : '')
}
const COMPOSE_REFUSALS = {
  'words': { again: true, says: 'the words the step typed are not the words this script hashed: one was altered, dropped, added or moved on its way through the step\'s agent — also when the step was asked for again', then: null },
  'check': { again: false, says: 'a check of the batch does not hold — a launched reporter\'s report is missing, a file nobody launched lies among the reports, a key is no row — or a file among the filed verdicts is no verdict. The stage\'s report check came before this step, so something changed between the two', then: null },
  'pending': { again: false, says: 'another batch is applied and not committed — one this stage did not compose: it appeared while the stage ran', then: 'pending' },
  'no-change': { again: false, says: 'every file the record would write is what the run holds already — and an attempt\'s first record always writes that the attempt reached it', then: ' Nothing was written and NO BATCH IS LEFT. No discard is asked for.' },
  'hygiene': { again: false, says: 'the public-hygiene scan stopped a file the record would write — gitleaks found a secret-shaped string in it, or the denylist matched: a credential is never committed, even a revoked one, and the scan is never widened to let one through', then: ' What was committed stands, and the reports the stage filed are still untracked under the run\'s directory. What wrote the string is changed — a report its reporter rewrites, a field a tool names — and the stage is invoked again; the step\'s own line holds the record script\'s whole refusal.' },
}
function scanHits(hits) {
  const named = Array.isArray(hits) ? hits.filter((hit) => plain(hit) && isText(hit.rule)) : []
  return named.length ? '. It stopped ' + named.map((hit) => (hit.rule === 'denylist' ? 'a match of the denylist' : 'gitleaks\' rule `' + hit.rule + '`') + (Number.isInteger(hit.line) ? ' at line ' + hit.line : '') + ' of ' + (isText(hit.path) ? '`' + hit.path + '`' : 'a file the step\'s line names')).join('; ') : ''
}


function branchName(run, round, cut) {
  const loop = 'fix/' + run
  if (round == null) return loop
  return loop + '-r' + round + (cut > 1 ? '-part' + (cut - 1) : '')
}

function runDir(run) {
  return RUNS_ROOT + '/' + run
}

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

const QUEUE_RULINGS = ['order', 'queued', 'budget']
const RUN_RULINGS = ['go', 'rerun', 'rounds', 'cycles', 'reverify', 'again'].concat(QUEUE_RULINGS)
const ONCE = '`hand-on` is no ruling: how many items a verifier hands on to the queue is written ONCE, at the run\'s opening, and `' + RECORD_TOOL + ' run-set` refuses a second write of it (`exists`) — no ruling changes it, and nothing was recorded. What the human rules about a run\'s queue is its budget, raised (`budget`), another order of its classes (`order`), and the rows he puts into it by key (`queued`)'
const MAX_NUMBER = 999999
const CLASS_RE = /^[a-z]+(-[a-z]+)* [a-z]+(-[a-z]+)*$/
function runRuling(r) {
  return plain(r) && RUN_RULINGS.some((name) => r[name] != null)
}
function validateRulings(rulings) {
  if (!Array.isArray(rulings) || rulings.length === 0) return 'args.rulings must be a non-empty list of rulings'
  const keys = []
  const about = []
  for (const r of rulings) {
    if (!plain(r)) return 'a ruling must be an object, not ' + JSON.stringify(r)
    if (r['hand-on'] != null) return ONCE
    if (runRuling(r)) {
      const named = Object.keys(r)
      if (named.length !== 1) return 'a ruling about the run names one thing — ' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + ' — and nothing beside it: ' + JSON.stringify(r)
      if (r.go != null && r.go !== true) return 'the human\'s go is `go` as true, or is not passed at all: ' + JSON.stringify(r)
      if (r.rerun != null && !isSlug(r.rerun)) return '`rerun` names the clause that is granted one more re-run, a slug as the clause table spells it: ' + JSON.stringify(r)
      if (r.rounds != null && !(Number.isInteger(r.rounds) && r.rounds >= 1 && r.rounds <= MAX_ROUNDS)) return '`rounds` is the bound across rounds, raised: a whole number from 1 to ' + MAX_ROUNDS + ', not ' + JSON.stringify(r.rounds)
      if (r.cycles != null && !(Number.isInteger(r.cycles) && r.cycles > DEFAULT_CYCLES && r.cycles <= MAX_CYCLES)) return '`cycles` is the bound on a round\'s fix cycles, raised: a whole number above ' + DEFAULT_CYCLES + ' and at most ' + MAX_CYCLES + ', not ' + JSON.stringify(r.cycles)
      if (r.reverify != null && !isSlug(r.reverify)) return '`reverify` names the finding that is granted one more triage, by its ledger key: ' + JSON.stringify(r)
      if (r.again != null && !STAGES.includes(r.again)) return '`again` names the stage that is granted one more attempt — ' + STAGES.join(' or ') + ': ' + JSON.stringify(r)
      if (r.budget != null && !(Number.isInteger(r.budget) && r.budget >= 1 && r.budget <= MAX_NUMBER)) return '`budget` is how many verdicts a round\'s `test` stage may spend, raised: a whole number from 1 to ' + MAX_NUMBER + ', not ' + JSON.stringify(r.budget)
      if (r.order != null && !(Array.isArray(r.order) && r.order.length > 0 && r.order.every((entry) => typeof entry === 'string' && CLASS_RE.test(entry)) && distinct(r.order))) return '`order` is the order of classes the queue is ranked by: a non-empty list of classes, each `<kind> <origin>` and none twice, not ' + JSON.stringify(r.order)
      if (r.queued != null && !(Array.isArray(r.queued) && r.queued.length > 0 && r.queued.every(isSlug) && distinct(r.queued))) return '`queued` is the rows the human puts into the queue: a non-empty list of ledger keys, none twice, not ' + JSON.stringify(r.queued)
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

const BUILD_RECORD = 'completions/artifacts/M55/stabilization-build/README.md'
const NOT_FIT = {
  fix: 'its repair and the re-review of that repair are not recorded',
}
function notFit(a) {
  if (!NOT_FIT[a.stage]) return null
  if (a.rulings != null && a.rulings.every(runRuling)) return null
  return 'the `' + a.stage + '` stage is NOT FIT FOR USE and refuses to start: ' + NOT_FIT[a.stage] + '. The build of the stabilization workflow was reviewed red — its record is ' + BUILD_RECORD + ' — and a stage is used only once its half is repaired and re-reviewed. Nothing was run: no agent, no read. What is taken meanwhile: ' + STAGES.filter((stage) => !NOT_FIT[stage]).map((stage) => 'the `' + stage + '` stage').concat(['what the human rules about the run (args.rulings: ' + RUN_RULINGS.map((name) => '`' + name + '`').join(', ') + '), which either stage records and which starts nothing']).join('; and ') + '. A ruling on a finding or a declared bound is this stage\'s own first step and waits with it — until then an invocation of `test` that carries only such rulings records them on the loop branch, and starts nothing'
}

function scratchFault(value) {
  return typeof value !== 'string' || !SCRATCH_RE.test(value) || /\/\.\.?(\/|$)/.test(value) ? 'args.scratch ' + JSON.stringify(value) + ' is not an absolute directory path of plain segments ([A-Za-z0-9._-]): every agent works under it, and it reaches a shell' : null
}
function probeFault(a) {
  if (!PROBES.includes(a.probe)) return 'args.probe ' + JSON.stringify(a.probe) + ' is not a runtime probe: ' + PROBES.join(', ')
  const beside = Object.keys(a).filter((name) => !['probe', 'scratch', 'seconds'].includes(name))
  if (beside.length) return 'a probe works on no run and no stage: it takes `scratch` — and `seconds`, for `hold` — and nothing else, not ' + beside.join(', ')
  if (scratchFault(a.scratch)) return scratchFault(a.scratch)
  if (a.seconds != null && (a.probe !== 'hold' || !Number.isInteger(a.seconds) || a.seconds < 1 || a.seconds > PROBE_HOLD_MAX)) return 'args.seconds is how long the `hold` probe holds its command — a whole number from 1 to ' + PROBE_HOLD_MAX + ' — and no argument of another probe: not ' + JSON.stringify(a.seconds)
  return null
}

function validateArgs(a) {
  if (!plain(a)) return 'args must be an object: { stage, run, scratch, … }'
  const unknown = Object.keys(a).filter((name) => !ARGS.includes(name))
  if (unknown.length) return 'unknown arg(s) ' + unknown.join(', ') + ' — the args are ' + ARGS.join(', ')
  if (a.selfTest != null && typeof a.selfTest !== 'boolean') return 'args.selfTest must be true or false'
  if (a.selfTest) return null
  if (a.probe != null) return probeFault(a)
  if (!STAGES.includes(a.stage)) return 'args.stage ' + JSON.stringify(a.stage) + ' is not one of ' + STAGES.join(', ')
  if (!isSlug(a.run)) return 'args.run ' + JSON.stringify(a.run) + ' is not a slug: lowercase a-z0-9 words joined by single dashes, at most ' + SLUG_MAX + ' characters — it names the run\'s directory and its branches'
  if (scratchFault(a.scratch)) return scratchFault(a.scratch)
  if (a.seconds != null) return 'args.seconds is the `hold` probe\'s, and no argument of a stage'
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

function reporterName(parts) {
  const name = parts.join('-')
  return isSlug(name) ? name : null
}

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
    drop(name) {
      if (names.includes(name)) names.splice(names.indexOf(name), 1)
    },
    line(name) {
      return reportLine(ctx, name)
    },
  }
}

function stageFlags(ctx) {
  return '--run ' + ctx.run + ' --round ' + ctx.round + ' --stage ' + ctx.stage + (ctx.stage === 'fix' ? ' --cycle ' + ctx.cycle : '')
}
function reportFile(ctx, name) {
  return ctx.scratch + '/report/r' + ctx.round + '/' + ctx.stage + (ctx.stage === 'fix' ? '.c' + ctx.cycle : '') + '/' + name + '.a' + ctx.attempt + '.md'
}
function reportCall(ctx, name) {
  return typed(RECORD_TOOL, 'report ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + ' --from ' + reportFile(ctx, name))
}
function reportLine(ctx, name) {
  return LABELS.report + ' ' + stageFlags(ctx) + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' — write your report ONCE, when what it reports is done, to the file `' + reportFile(ctx, name) + '` WITH YOUR FILE TOOL; a correction is made in that file WITH YOUR EDIT TOOL, never by writing it whole again; and hand it over by ONE plain call, typed as it stands: ' + reportCall(ctx, name) + ' (with one more `--scratch` for each scratch root of your own outside that one).'
}
function below(scratch, path) {
  return path.startsWith(scratch + '/') ? path.slice(scratch.length + 1) : path
}
function binaryLine(built, withImage, scratch) {
  return LABELS.binary + ' candidate `' + built.candidate.binary + '` sha256 ' + built.candidate.sha256 + ' (commit ' + built.candidate.sha + ', label ' + built.candidate.label + ')'
    + (built.previous ? '; previous release `' + built.previous.binary + '` sha256 ' + built.previous.sha256 + ' (version ' + built.previous.version + ')' : '')
    + (withImage && built.image ? '; trial image `' + built.image.tag + '`' : '')
    + '.' + (scratch ? ' ASSERT THE CANDIDATE\'S HASH BEFORE YOU DRIVE ANYTHING, by ONE plain call typed as it stands, and never by a hash command of your own: ' + typed(STEP_TOOL, 'hash --scratch ' + scratch + ' --file ' + below(scratch, built.candidate.binary)) + ' prints ONE line of JSON whose `content_sha256` must be that sha256' + (built.previous ? ' — and the same call with `' + below(scratch, built.previous.binary) + '` gives the previous release\'s' : '') + '.' : '')
    + ' Return the sha256 you asserted for the candidate as `asserted_sha256`.'
}
function branchLine(branch, loop) {
  return LABELS.branch + ' ' + branch + (branch === loop ? '' : ' (forked from ' + loop + ')') + ', already checked out by the harness. Before you commit, the branch checked out must be exactly `' + branch + '` — if it is not, commit nothing and halt. Never create, switch, merge or push a branch; the harness\'s git steps own them.'
}

function hashMismatch(expected, returns) {
  return returns.filter((r) => r.drives && r.result && r.result.status !== 'halted' && r.result.asserted_sha256 !== expected).map((r) => ({ reporter: r.name, asserted: r.result.asserted_sha256 == null ? null : String(r.result.asserted_sha256) }))
}

function lacks(role, result) {
  const held = (path) => path.split('.').reduce((at, name) => (at == null ? undefined : at[name]), result)
  return (NEEDS[role] || []).filter((n) => (!n.with || result.verdict === n.with) && held(n.field) == null)
}
function lacksWhy(reporter, need) {
  return '(' + reporter + ') returned no `' + need.field + '` — ' + need.why
}
function unasserted(role, reporter, result, expected) {
  const gap = lacks(role, result).find((n) => n.field === 'asserted_sha256')
  if (gap) return lacksWhy(reporter, gap)
  if (!ROLES[role].drives || result.asserted_sha256 === expected) return null
  return '(' + reporter + ') returned ' + (SHA256_RE.test(String(result.asserted_sha256)) ? 'the `asserted_sha256` ' + result.asserted_sha256 : 'an `asserted_sha256` that is no sha256') + ', which is not the candidate\'s binary (' + expected + '): nothing it drove is evidence about this candidate'
}

const NOT_COMPARABLE = 'not-comparable'
function regressionOf(reporter, result, previous) {
  const gap = lacks('verify', result).find((n) => n.then === 'unknown')
  const ran = result.ran_on || {}
  const why = gap ? lacksWhy(reporter, gap)
    : typeof result.regression !== 'boolean' && result.regression !== NOT_COMPARABLE ? '(' + reporter + ') returned a `regression` that is neither true, false nor `' + NOT_COMPARABLE + '`'
      : ran.previous !== previous ? '(' + reporter + ') returned ' + (SHA256_RE.test(String(ran.previous)) ? 'the hash ' + ran.previous : 'a hash that is no sha256') + ' for the previous release\'s binary, which is not the one it was handed (' + previous + '): what its regression fact rests on was not that release'
        : null
  return why ? { regression: null, unknown: why } : { regression: result.regression, unknown: null }
}

function reportsRead(step) {
  const seen = step && step.status === 'checked' && plain(step.check) ? step.check : null
  if (!seen || !Array.isArray(seen.missing) || !Number.isInteger(seen.extra) || !Array.isArray(step.aside)) return { fault: 'the report check could not be read' + (step && step.refused ? ': it refused ' + stepRefusal(step.refused) : ''), transient: !step, file: fileOf(step) }
  if (seen.extra) return { fault: 'the stage\'s report directory holds ' + seen.extra + ' file(s) no launched reporter wrote — a file nobody launched is nobody\'s report; the step\'s own line names each', file: fileOf(step) }
  return { missing: seen.missing, aside: step.aside }
}
function fileOf(step) {
  return step && isText(step.file) && SHA256_RE.test(String(step.file_sha256)) ? { file: step.file, sha256: step.file_sha256 } : null
}
function setAside(aside, name) {
  const moved = (aside || []).find((a) => plain(a) && typeof a.path === 'string' && a.path.slice(a.path.lastIndexOf('/') + 1).replace(/\.a[0-9]+\.md$/, '') === name)
  return moved ? String(moved.why) : null
}
function endingOf(result) {
  return result ? 'halted' + (result.halt && result.halt.root_cause ? ' (' + result.halt.root_cause + ')' : '') : 'returned nothing'
}

function unreported(ran, missing, aside) {
  for (const unit of ran) {
    for (const r of unit.reporters.filter((x) => missing.includes(x.name))) {
      const moved = setAside(aside, r.name)
      if (r.crossModel) unit.crossModel = 'void'
      else if (unit.status !== 'void') Object.assign(unit, { status: 'void', reason: 'its `' + r.as + '` step ' + (moved ? 'left a file at its report\'s path that is no report (`' + moved + '`): it was set aside, and the step left none' : r.result && r.result.status === 'reported' ? 'returned and left no report' : endingOf(r.result) + ', and left no report'), word: voidWord(moved ? 'set-aside' : !r.result ? 'silent' : r.result.status === 'reported' ? 'unreported' : 'halted', r.name) })
      r.result = null
      r.report = null
    }
  }
}

function outcomeOf(state) {
  const out = { next: state.next, known: KNOWN_NEXT.includes(state.next) }
  if (out.next === 'rule') out.rule = { findings: state.human_list || 0, reverify: state.reverify || [], clauses: state.human_clauses || [], stages: state.human_stages || [] }
  return out
}

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

function runRulingsFault(rulings, state) {
  const facts = state.facts || {}
  const queue = hasQueue(state) ? state.queue : null
  const moves = rulings.some((r) => r.order != null || r.queued != null)
  for (const r of rulings) {
    const about = QUEUE_RULINGS.find((name) => r[name] != null)
    if (about && !queue) return '`' + about + '` rules on the queue of a run, and this run has no queue: its opening holds no order. The order, the budget and the number a verifier hands on are written at a run\'s opening (`' + RECORD_TOOL + ' run-set`)'
    if (r.budget != null && queue.budget != null && r.budget <= queue.budget) return 'the budget is ' + queue.budget + ': `budget` raises it, and ' + r.budget + ' does not'
    if (r.budget != null && queue.budget != null && !moves && !(queue.left === 0 && queue.length > 0)) return 'the budget: the state does not ask for it — it is written at the run\'s opening, and raised only while it is spent and the queue holds a row nobody reached (' + queue.spent + ' of ' + queue.budget + ' spent, ' + queue.length + ' in the queue)'
    if (r.go != null && !(state.next === 'stop' && state.stop && state.stop.why === 'every-round')) return 'the run is not stopped after a round for the human\'s go — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': a go is recorded for that stop and for no other state'
    if (r.rerun != null && !(state.human_clauses || []).some((c) => c.clause === r.rerun)) return 'the clause `' + r.rerun + '` is not the human\'s: one more re-run is granted to a clause that is still not green after its re-run — the state\'s `human_clauses` names ' + ((state.human_clauses || []).map((c) => c.clause).join(', ') || 'none')
    if (r.rounds != null && facts.rounds != null && r.rounds <= facts.rounds) return 'the bound across rounds is ' + facts.rounds + ': `rounds` raises it, and ' + r.rounds + ' does not'
    if (r.rounds != null && (state.rounds || []).length && !(state.not_ready || []).length && !(state.stop && state.stop.why === 'round-bound')) return 'the run is not stopped at its bound across rounds — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': once a round is begun that bound is raised at its own stop, and at no other state'
    if (r.cycles != null && !(state.next === 'stop' && state.stop && state.stop.why === 'cycle-bound')) return 'the run is not stopped at the bound on a round\'s fix cycles — its `next` is `' + state.next + '`' + (state.stop ? ' (' + state.stop.why + ')' : '') + ': that bound is raised at its own stop, and at no other state'
    if (r.cycles != null && facts.cycles != null && r.cycles <= facts.cycles) return 'the bound on a round\'s fix cycles is ' + facts.cycles + ': `cycles` raises it, and ' + r.cycles + ' does not'
    if (r.reverify != null && !(state.reverify || []).includes(r.reverify)) return 'the finding `' + r.reverify + '` is not the human\'s to grant one more triage: that is granted to a finding the state\'s `human_list` names as still ungraded or unverified after its retry — its `reverify` names ' + ((state.reverify || []).join(', ') || 'none')
    if (r.again != null && !(state.human_stages || []).some((h) => h.stage === r.again)) return 'the `' + r.again + '` stage is not the human\'s to grant one more attempt: that is granted to a stage the state\'s `human_stages` names — it names ' + ((state.human_stages || []).map((h) => h.stage).join(', ') || 'none')
  }
  return null
}


function ledgerSource(run, state) {
  const awaiting = plain(state.untriaged) && Number.isInteger(state.untriaged.count) ? state.untriaged.count : 0
  if (!awaiting) return []
  return [{ reporter: LEDGER_SOURCE, report: runDir(run) + '/ledger.md', count: awaiting, read: typed(RECORD_TOOL, 'untriaged --run ' + run), findings: [] }]
}
function findingsOf(source) {
  return source.read ? source.count : source.findings.length
}

function hasQueue(state) {
  return plain(state.queue)
}
function testReport(ctx, name) {
  return runDir(ctx.run) + '/r' + ctx.round + '/reports/test/' + name + '.a' + ctx.attempt + '.md'
}
function entriesFile(ctx) {
  return ctx.scratch + '/triage/r' + ctx.round + '.a' + ctx.attempt + '.json'
}
function redSources(reds) {
  return reds.map((r) => ({ reporter: RED_PREFIX + r.item, red: r, findings: ['the check `' + r.item + '` is red · clause: ' + r.clause + ' · the door its row is filed with: the check `' + r.item + '`'] }))
}
function filedFault(ctx, name, sources, t) {
  if (t.report !== testReport(ctx, name)) return 'triage returned ' + (isText(t.report) ? 'a path' : 'no path') + ' that is not where its entries are rendered — `' + testReport(ctx, name) + '`, the path a report of `' + name + '`, attempt ' + ctx.attempt + ', has: the call was typed under another name, round or attempt, or was not made'
  if (!SHA256_RE.test(String(t.sha256))) return 'triage returned no sha256 of its rendered entries — the hash its call printed beside their path: nothing says which text it handed over'
  const said = Array.isArray(t.handed) ? t.handed.filter(plain) : []
  const wrong = sources.filter((s) => !said.some((h) => h.source === s.reporter && h.count === findingsOf(s))).map((s) => {
    const typedWith = said.find((h) => h.source === s.reporter)
    return '`' + s.reporter + '`: handed ' + findingsOf(s) + ', and the call ' + (typedWith && Number.isInteger(typedWith.count) ? 'was typed with ' + typedWith.count : 'named no count for it')
  })
  const strangers = said.filter((h) => !sources.some((s) => s.reporter === h.source)).length
  if (wrong.length || strangers) return 'the call triage handed its entries over by was not typed with the sources and counts this stage handed it — ' + wrong.concat(strangers ? [strangers + ' source(s) of the call that nobody handed'] : []).join('; ') + '. The record script holds the partition to the counts of the call, so a finding of a source named here may be in no entry: nothing is recorded on that'
  return null
}

function verdictFile(ctx, name) {
  return runDir(ctx.run) + '/r' + ctx.round + '/verdicts/' + name + '.a' + ctx.attempt + '.json'
}
function filingFile(ctx, name) {
  return ctx.scratch + '/verdict/r' + ctx.round + '/' + name + '.a' + ctx.attempt + '.json'
}
function verdictFault(ctx, name, r, expected) {
  if (r.filing !== verdictFile(ctx, name)) return 'returned ' + (isText(r.filing) ? 'a path' : 'no path') + ' that is not where a verdict of `' + name + '`, attempt ' + ctx.attempt + ', is filed — `' + verdictFile(ctx, name) + '`: the call was typed under another name, round or attempt, or was not made'
  if (!FILED_WORDS.includes(r.word)) return 'returned ' + (r.word == null ? 'no word' : 'a word') + ' that is no verdict a filing says — ' + FILED_WORDS.join(', ') + ' — which the call prints as `word`'
  if (typeof r.contested !== 'boolean') return 'did not return `contested` as true or false: whether its filing gave a basis, as the call prints it'
  if (r.asserted_sha256 !== expected) return 'returned ' + (r.asserted_sha256 == null ? 'no `asserted_sha256`' : SHA256_RE.test(String(r.asserted_sha256)) ? 'the `asserted_sha256` ' + r.asserted_sha256 : 'an `asserted_sha256` that is no sha256') + ', which is not the candidate\'s binary (' + expected + ')'
  return null
}
const CONTINUES = { triage: 'a row nobody graded is left', verify: 'a row of the queue is unverified, and budget is left', filed: 'a filed verdict is in no record' }
function continuing(state, at) {
  return hasQueue(state) ? Object.keys(CONTINUES).filter((word) => at[word] === true) : []
}
function continueFault(a) {
  const passed = ['scope', 'clause', 'crossModel'].filter((name) => a[name] != null).concat(a.stopAfter != null && a.stopAfter !== 'state' ? ['stopAfter: \'' + a.stopAfter + '\''] : [])
  return passed.length ? 'this invocation CONTINUES the round\'s `test` stage — it runs no instrument, resolves no scope and grades only rows nobody graded — so it takes no scope, clause or crossModel, and can be stopped after nothing but its first read (stopAfter: \'state\'): it was passed ' + passed.join(', ') : null
}
const LIMIT_STEPS = {
  'report-check': { step: 'the report check after that wave', read: 'before', commit: 'none' },
  'queue': { step: 'the read of the queue after that wave', read: 'before', commit: 'none' },
  'compose': { step: 'the step that composes the record', read: 'after', commit: 'none' },
  'gate': { step: 'a step of the record\'s gate', read: 'after', commit: 'none' },
  'commit': { step: 'the record\'s commit step', read: 'after', commit: 'unknown' },
  'push': { step: 'the push of the record', read: 'after', commit: 'made' },
  'read-back': { step: 'the read of the state after the record', read: 'after', commit: 'pushed' },
}
function limitSaid(ctx, last, counted, files, unconfirmed, step, record) {
  const at = LIMIT_STEPS[step]
  const directory = runDir(ctx.run) + '/r' + ctx.round + '/verdicts'
  const committed = at.commit === 'made' || at.commit === 'pushed'
  const named = (files.length ? ' — of this invocation\'s verifiers: ' + files.join(', ') : ' — no verifier of this invocation filed one') + (unconfirmed.length ? '; returned as filed in the wave whose report check or whose read of the queue did not come back, and held by nothing: ' + unconfirmed.join(', ') : '')
  const lies = 'each verdict that is filed is a file under ' + directory + '/ on this machine\'s disk' + named + '.'
  const filed = committed ? ' WHAT WAS FILED IS NOT LOST, AND IS IN A COMMIT: the record this invocation made (' + record + ') took it' + (at.commit === 'pushed' ? ', and is pushed' : ' — a local commit until the push that is owed is made') + '. In that commit, ' + lies
    : at.commit === 'unknown' ? ' WHAT WAS FILED IS NOT LOST: ' + lies + ' WHETHER A COMMIT HOLDS IT THIS SCRIPT CANNOT SAY — the commit step may have made its commit before it returned nothing: the next invocation\'s first read says which, and finishes the record either way.'
      : ' WHAT WAS FILED IS NOT LOST, AND IS IN NO COMMIT YET: ' + lies
  const state = at.read === 'before' ? ' THE STATE WAS LAST READ BEFORE THAT WAVE: it counted ' + counted + ' verdict(s) in round ' + ctx.round + ' then, and the state document this return names is that read\'s — it lists nothing wave ' + last.wave + ' filed. ' + typed(RECORD_TOOL, 'state --run ' + ctx.run) + ' prints the state as it stands now, on one line: every filed verdict, with whether it counts (`filed`).'
    : ' The state counts ' + counted + ' verdict(s) in round ' + ctx.round + ', and the state document this return names lists every filed verdict with whether it counts (`filed`)' + (at.commit === 'none' ? '' : ' — as it was read before that record\'s commit: what it says of which record took a verdict (`recorded`) is of then') + '.'
  const then = at.read === 'before' ? ' Wait for the window, then invoke the stage again with the same args: where the state\'s position still hands it the stage, it CONTINUES — no instrument, the filed verdicts taken as they stand, and only what is missing launched; where it does not, its return says why, and whichever record the run makes next takes what is filed.'
    : ' Wait for the window, then invoke the stage again with the same args: what that invocation does first is said above, with this halt.'
  return {
    more: { limit: { possible: true, wave: last.wave, silent: last.silent, step }, filed: { counted, read: at.read + '-the-wave', commit: at.commit, record: committed ? record : null, files, unconfirmed, directory } },
    says: ' THIS MAY BE THE ACCOUNT\'S USAGE LIMIT — this script cannot tell: a limit never reaches it as a message, it is handed nothing. ' + last.silent + ' verifier call(s) of wave ' + last.wave + ' returned nothing, so that wave was made the last, and then ' + at.step + ' returned nothing too.' + filed + state + then,
  }
}

function previousOf(state) {
  const facts = state.facts || {}
  return { version: facts.previous, commit: facts['previous-commit'] }
}

function evidenceOf(state) {
  const at = (value) => (value == null ? null : value)
  return (state.clauses || []).map((c) => ({ clause: c.clause, status: c.status, round: c.round, commit: c.commit, behind: c.behind, items: (state.items || []).filter((i) => i.clause === c.clause).map((i) => ({ item: i.item, round: at(i.round), attempt: at(i.attempt), standing: at(i.standing), why: at(i.why) })) }))
}

function namedOf(state, round) {
  if (!state) return null
  const within = round == null ? state.round : round
  const bare = (state.uncovered || []).find((u) => u.round === within)
  return { never_selected: state.never_selected || [], uncovered: { round: within == null ? null : within, doors: bare ? bare.doors : 0 }, regression_unknown: (state.regression_unknown || []).length, document: state.document || null }
}

function queueSaid(state) {
  if (!state || !hasQueue(state)) return {}
  const queue = state.queue
  return {
    continues: continuing(state, state.position.test),
    budget: { limit: queue.budget, spent: queue.spent, left: queue.left },
    queue: { length: queue.length, hand_on: queue.hand_on, classes: queue.classes },
    view: state.view || null,
  }
}
function statusOf(words, second) {
  return second ? 'verified' : words.length ? 'continued' : 'hunted'
}

const STEP_SHA_RE = /, "sha256": "([0-9a-f]{64})"\}$/
function lostStep(r, why) {
  return { status: 'halted', relay: why, halt: { root_cause: why, evidence: typeof r.line === 'string' ? '(a line of ' + r.line.length + ' characters came back; it is not passed on)' : '(the step returned no line)', tree_state: '(not read: the step\'s line was not the command\'s)', recommendation: 'read the tree and the branches before the step is asked for again: its command may have run' } }
}
function readStep(r, act, exact) {
  return stepRead(r, act, exact).said
}
function stepRead(r, act, exact) {
  if (!r) return { said: null }
  if (r.status !== 'ran') return { said: { status: 'halted', halt: r.halt || null } }
  const lost = (why) => ({ said: lostStep(r, why) })
  if (typeof r.line !== 'string') return lost('the step returned no line')
  const held = asPrinted(r.line, exact)
  if (!held) return lost('the relayed line does not end with the sha256 of itself, as `' + STEP_TOOL + '` prints it: it was altered on its way, or is not the command\'s')
  let said = null
  try {
    said = JSON.parse(held.line)
  } catch (e) {
    return lost('the relayed line is not JSON: ' + ((e && e.message) || e))
  }
  if (!plain(said) || said.act !== act || typeof said.status !== 'string') return lost('the relayed line is not the line of `' + STEP_TOOL + ' ' + act + '`')
  return { said, line: held.line, mended: held.mended }
}
function asPrinted(relayed, exact) {
  const line = relayed.replace(/\n$/, '')
  const tail = STEP_SHA_RE.exec(line)
  if (!tail) return null
  const body = line.slice(0, tail.index)
  if (sha256(body + '}') === tail[1]) return { line, mended: null }
  const mended = exact ? null : mendSlip(body, tail[1])
  return mended ? { line: mended.body + line.slice(tail.index), mended: mended.field } : null
}
const MEND_TRIES = 4096
function mendSlip(body, want) {
  const fields = Array.from(body.matchAll(/"([^"\\]+)":("[^"\\]*"|-?\d+|true|false|null)(?=[,}\]])/g), (m) => ({ key: m[1], value: m[2], at: m.index + m[0].length - m[2].length }))
  const under = {}
  for (const f of fields) {
    if (!Object.prototype.hasOwnProperty.call(under, f.key)) under[f.key] = []
    if (!under[f.key].includes(f.value)) under[f.key].push(f.value)
  }
  let tries = 0
  for (const f of fields) {
    for (const value of under[f.key]) {
      if (value === f.value) continue
      if (++tries > MEND_TRIES) return null
      const text = body.slice(0, f.at) + value + body.slice(f.at + f.value.length)
      if (sha256(text + '}') === want) return { body: text, field: { key: f.key, relayed: f.value, printed: value } }
    }
  }
  return null
}
const DIGEST_RE = /^[\x20-\x7e]*$/
function readDigest(r, act, label) {
  const read = stepRead(r, act)
  const said = read.said
  if (!said || r.status !== 'ran' || said.relay) return said
  const line = read.line
  if (!Array.isArray(said.unfit) || !('file' in said) || !('file_sha256' in said)) return lostStep(r, 'the relayed line is no digest of `' + STEP_TOOL + ' ' + act + '` — it names neither what did not fit nor the file that holds the rest: it is the line the tool prints unasked, or another\'s')
  if (!DIGEST_RE.test(line) || line.includes('\\')) return lostStep(r, 'the relayed line holds a backslash or a character outside printable ASCII, and a digest of `' + STEP_TOOL + '` holds neither: it was altered on its way, or is not the digest that was asked for')
  if (read.mended && label) log('the relayed line of `' + label + '` came back with ONE value altered — `' + read.mended.key + '` as ' + read.mended.relayed + ' where the tool printed ' + read.mended.printed + ' — and is read as the tool printed it: that value put back, the line hashes to the sha256 it ends with')
  if (said.unfit.length) return Object.assign(lostStep(r, 'the digest of `' + STEP_TOOL + ' ' + act + '` could not print ' + said.unfit.filter((name) => typeof name === 'string' && /^[a-z0-9_.-]+$/.test(name)).join(', ') + ': a value did not fit the shape its field declares, and this script acts on no line with a field struck out. THE ACT MAY HAVE RUN — its own line, whole, is in the file the digest names'), { unfit: true, said: said.status, file: said.file, file_sha256: said.file_sha256 })
  return said
}
function heldOf(step, scratch) {
  if (!step) return { ends: 'unread', why: 'the step returned no result', silent: true }
  if (step.status === 'halted') return { ends: 'unread', why: step.refused ? stepRefusal(step.refused) : step.relay || 'the step halted', refused: step.refused || null }
  const job = { name: step.name, kind: step.kind, output: isText(step.output) ? scratch + '/' + step.output : null, fresh: step.status === 'started' }
  if (step.status === 'started' || step.status === 'running') return Object.assign({ ends: 'running' }, job)
  if (step.status === 'dead') return Object.assign({ ends: 'dead', why: step.why || null }, job)
  if (step.status === 'done' && ['green', 'red', 'void'].includes(step.verdict) && isText(step.verdict_file) && SHA256_RE.test(String(step.verdict_sha256))) return Object.assign({ ends: step.verdict, why: step.why || null, verdict: scratch + '/' + step.verdict_file, verdict_sha256: step.verdict_sha256, facts: plain(step.facts) ? step.facts : {} }, job)
  return Object.assign({ ends: 'unread', why: 'the step said `' + step.status + '` and named no verdict of the tool\'s' }, job)
}
function passedOver(job, at, tries) {
  if (at.ends === 'unread' || at.fresh) return null
  if (at.ends === 'running') return job.sequence && tries < 2 ? 'running' : null
  if (job.sequence) return 'ended'
  return job.stands && at.ends !== 'green' ? 'ended' : null
}

function resultRows(units) {
  return units.map((unit) => {
    if (unit.verdict) return { item: unit.item, verdict: unit.verdict }
    if (unit.status === 'void') return { item: unit.item, outcome: 'void', reason: unit.reason || 'it did not run to its end' }
    if (unit.status === 'red') return { item: unit.item, outcome: 'red', doctype: RED_DOCTYPE, door: 'the check `' + unit.item + '`', repro: unit.evidence || 'the check returned red, and no evidence beside it' }
    return { item: unit.item, outcome: 'green' }
  })
}

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
function reopenPatches(ledger, gone) {
  return ledger.filter((row) => row.disposition === 'fixed' && gone.some((sha) => sameCommit(sha, row.detail))).map((row) => ({ key: row.key, disposition: 'open' }))
}
function repointPatches(ledger, picked) {
  const out = []
  for (const row of ledger) {
    const pick = row.disposition === 'fixed' ? picked.find((p) => sameCommit(p.from, row.detail)) : null
    if (pick) out.push({ key: row.key, disposition: 'fixed', detail: pick.to })
  }
  return out
}

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

function currentCut(run, round, local) {
  let cut = 0
  for (let c = 1; c <= MAX_CYCLES; c++) if (local.includes(branchName(run, round, c))) cut = c
  return cut
}

function triageRecord(ctx, entries) {
  return {
    rows: entries.filter((e) => e.new).map((e) => ({ key: e.key, doctype: e.doctype, round: ctx.round, source: e.source, door: e.door, clause: e.clause, repro: e.repro })),
    triage: entries.map((e) => {
      const t = { key: e.key, grade: e.grade }
      if (e.grade === 'out-of-scope') t.bound = e.bound
      if (e.verdict) t.verdict = e.verdict
      if (e.verdict === 'confirmed' && (typeof e.regression === 'boolean' || e.regression === NOT_COMPARABLE)) t.regression = e.regression
      if (e.verdict && e.fork) t.fork = e.fork
      return t
    }),
  }
}

function discardCommand(run, branch) {
  return '`' + STEP_TOOL + ' discard --branch ' + branch + ' --run-dir ' + runDir(run) + '`'
}
function recordThen(run, branch, word, discarded) {
  const stands = ' What was committed stands.'
  const takeBack = ' To take the batch back instead — the orchestrator\'s decision, a subagent\'s act — ' + discardCommand(run, branch) + ' asks git first, puts every table back as it was before the record step, and keeps the reports and the round\'s scope.'
  if (word === 'gate-red') return stands + ' THE BATCH IS STILL APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree. Invoke the stage again: that invocation runs the full gate on the batch once more, commits and pushes it, and does nothing else.' + takeBack
  if (word === 'unvetted') return stands + (discarded === true ? ' THE BATCH IS TAKEN BACK ALREADY, and a report or a scope that was no such file has left the tree: nothing is pending, and no discard is asked for. Invoke the stage again: it reads the state as it stands and works the next attempt.' : ' The batch was NOT taken back — a file of it is in a commit already — and it is the human\'s: the step\'s own line says which file.')
  if (word === 'fence-moved') return stands + ' Nothing was committed or undone: THE BATCH IS STILL APPLIED — AND SOMETHING WROTE THE REPOSITORY PAST THE WRITE GUARD while the stage ran: its configuration, a hook, an info file, a local ref or a reflog\'s newest entry moved. It is the human\'s: what moved is found and put back first. Then invoke the stage again: its first read takes the repository as it then stands, gates the batch and commits it.' + takeBack
  if (word === 'head-moved') return stands + ' Nothing was committed or undone: THE BATCH IS STILL APPLIED. Invoke the stage again: its first read meets the commit somebody made while the stage ran — one the remote lacks is its author\'s to push — then gates the batch on the tree as it now stands, and commits it there.' + takeBack
  if (word === 'no-gate') return stands + ' THE BATCH IS STILL APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree: its gate left no verdict, and no commit step ran. Invoke the stage again: that invocation holds the full gate on the batch once more, under the next name, commits and pushes it, and does nothing else.' + takeBack
  if (word === 'no-name') return stands + ' THE BATCH IS STILL APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree — AND NO NAME IS LEFT TO HOLD ITS GATE UNDER: the gate of a pending batch is held under one of ' + HOLD_NAMES + ' names for the life of a scratch root, a name is one command\'s, and every one of them holds a command already. INVOKING THE STAGE AGAIN CHANGES NOTHING: that invocation asks for the same names, and halts here. THE EXIT THAT WORKS is the batch taken back — the orchestrator\'s decision, a subagent\'s act: ' + discardCommand(run, branch) + ' asks git first, puts every table back as it was before the record step, and keeps the reports and the round\'s scope — and a filed verdict, which is no file of a batch. The stage invoked after that reads the state as it stands: what the batch recorded is recorded again by the step that owes it — a stage\'s record under its attempt\'s own name, the human\'s rulings by the invocation that carries them.'
  if (word === 'did-not-run' || word === 'locked') return stands + ' Everything is as it was: THE BATCH IS STILL APPLIED. Once what the refusal names is dealt with, invoke the stage again: that invocation gates the batch once more, commits and pushes it, and does nothing else.' + takeBack
  return stands + ' A record step that stopped may have left its batch APPLIED AND NOT COMMITTED — the tables written, held by the record script as pending, and no dirty tree. Invoke the stage again: its first read says what is there. Where a batch is pending, that invocation runs the full gate on it once more, commits and pushes it, and does nothing else; where the commit was made and nothing else, it settles and pushes it; where none is, it reads the state as it stands and works the next attempt.' + takeBack
}
const NO_CHANGE = 'no-change'
function pushThen(branch, word) {
  if (word === 'unvetted') return ' What was committed stands, AND THE PUSH IS NOT OWED TO ANY LATER INVOCATION: a commit it would publish is not what `dev/stabilize-record` would have written, so every push of it is refused again — a stage\'s first read included — until the human has taken that commit back or rewritten it. `dev/stabilize-record vet --range -- ' + branch + ' --not --remotes=origin` names every commit and place.'
  if (word === 'foreign-commit') return ' What was committed stands. The record is not pushed beside a commit that is no record\'s: its author pushes that commit by name — `git push origin ' + branch + '` — and the stage is invoked again; its first read then makes the push that is owed.'
  return ' What was committed stands, and THE PUSH IS OWED: invoke the stage again with the same args — its first read finds the record the remote lacks, vets it and pushes it, and the stage goes on from the state as it then stands.'
}
const LOOK_AGAIN = 'RETRY after a transient failure of an earlier attempt at this same call. Before anything else look at what that attempt left — the tree\'s status, the last commit, and whether your report already stands at its path — and go on from it: never clean the tree (the untracked files under the run\'s directory are other agents\' reports), never make a commit or hand over a report that exists already. If you cannot tell what the dead attempt did, halt and say so.'
const RUN_AGAIN = 'RETRY: an earlier try of this same step returned nothing. Run the ONE command above again, exactly as it is written, and relay its line: the command says itself what an earlier run of it left — it finishes that, answers again what it answered, or refuses — and you look at nothing else and repair nothing.'
function relayAgain(n) {
  return 'AGAIN (' + n + '): the line an earlier attempt returned was not the line the command printed. Copy the line from the command\'s output character for character.'
}

const HALT = {
  type: 'object',
  description: 'the halt report — fill every field, so that nobody needs your transcript',
  properties: {
    root_cause: { type: 'string' },
    evidence: { type: 'string', description: 'the command and its full output, or the refusal line verbatim' },
    tree_state: { type: 'string', description: 'the tree\'s status, the branch checked out, and which commits landed' },
    recommendation: { type: 'string' },
  },
}
const STRINGS = { type: 'array', items: { type: 'string' } }
const STEP_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['ran', 'halted'] },
    halt: HALT,
    line: { type: 'string', description: 'the ONE line the command printed on stdout, whole and as printed: every character copied, nothing re-ordered, summarised, re-indented or re-escaped' },
  },
}
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
    image: { type: 'object', properties: { tag: { type: 'string' }, verified: { type: 'boolean' }, failed: STRINGS } },
    checks: { type: 'array', items: { type: 'object', required: ['check', 'status'], properties: { check: { type: 'string', description: 'the id of the item the prompt listed the check under' }, status: { type: 'string', enum: ['green', 'red', 'void'] }, commit: { type: 'string', description: 'the full sha the check ran on: a green or a red is evidence about that commit and no other' }, evidence: { type: 'string' } } } },
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
const PROBE_UNIT_SCHEMA = Object.assign({}, UNIT_SCHEMA, { required: UNIT_SCHEMA.required.concat(['asserted_sha256']) })
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
const FILED_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['filed', 'halted'] },
    halt: HALT,
    report: { type: 'string', description: 'the path the call printed as `triage`: your report, rendered from your entries' },
    sha256: { type: 'string', description: 'the hash that call printed beside the path, as `sha256`' },
    handed: { type: 'array', description: 'what that call printed as `handed`: one entry a source, with the count the call named for it', items: { type: 'object', required: ['source', 'count'], properties: { source: { type: 'string' }, count: { type: 'number' } } } },
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
    regression: { enum: [true, false, 'not-comparable'], description: 'with confirmed only: true where the repro is green on the previous release, false where it is red there too, not-comparable where it cannot run there because its door is one that release does not have' },
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
const VERDICT_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['filed', 'halted'] },
    halt: HALT,
    filing: { type: 'string', description: 'the path the call that filed your verdict printed as `verdict`' },
    word: { type: 'string', description: 'the verdict that call printed as `word`: confirmed, refuted or undrivable' },
    contested: { type: 'boolean', description: 'what that call printed as `contested`: true where your filing gave a basis, false where it gave none' },
    asserted_sha256: { type: 'string', description: 'the sha256 you asserted for the candidate before you drove it' },
    report: { type: 'string', description: 'the path the call that handed your report over printed' },
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
const RECORD_SCHEMA = {
  type: 'object',
  required: ['status'],
  properties: {
    status: { type: 'string', enum: ['applied', 'halted'] },
    halt: HALT,
    refused: { type: 'string', description: 'when the batch of step 2 was refused: the ONE word that follows `refused` on the line `dev/stabilize-record` printed on stderr — `no-change`, `check`, `hygiene`, …' },
  },
}

const STEP_RULES = 'Run exactly the ONE command below, from the repository\'s root, typed as it stands, and nothing else: no command before it, after it or around it, no branch, no commit, no file edit, no stash, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. The command does the whole step and checks it. It prints ONE line of JSON on stdout — also when it refuses, which it says with an exit status that is not 0 and one line on stderr. A refusal is the step\'s answer: you never repair what it names, never run the command a second time, and never do by hand what it did not do.'
const ASK_RULES = 'Run exactly the ONE command below, from the repository\'s root, typed as it stands, and nothing else: no command before it, after it or around it, no branch, no commit, no file edit, no stash, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. It asks the tool what became of a long command the tool itself started and holds, and it answers by itself within a minute and a half, with ONE line of JSON on stdout — also when it refuses, which it says with an exit status that is not 0 and one line on stderr. WHILE THAT LINE\'S `status` IS `running`, RUN THE SAME COMMAND AGAIN, typed as it stands, as often as it takes: the long command can take the better part of an hour, and asking again is the whole of this step — nothing else is run in between, and you keep no time of your own. The first line whose `status` is anything else — `done`, `dead`, or a refusal — is the step\'s answer: you never repair what it names, and never start the long command yourself.'
const AS_PRINTED = 'the ONE line the command printed on stdout, WHOLE and AS PRINTED, whatever it exited with: every character copied, no key re-ordered or dropped, nothing summarised, no `\\u…` escape rewritten into its character'
function stepPrompt(what, act, flags, asks) {
  return [
    'GIT STEP — ' + what + '. ' + (asks ? ASK_RULES : STEP_RULES),
    '1. ' + typed(STEP_TOOL, act + ' ' + flags),
    '2. Report status = ran and line = ' + AS_PRINTED + (asks ? ' — of the LAST run of it: the first line whose `status` is not `running`' : '') + '. The harness holds the line to the hash it ends with, so a line that is not the command\'s is caught. Only when the command printed no such line: status = halted, and the halt report (root_cause, evidence = the command and everything it printed, tree_state = the tree\'s status and the branch checked out).',
  ].join('\n')
}
function holdStartPrompt(v, name, kind, flags, what) {
  return stepPrompt('START ' + what + ' as a command the tool holds (`' + name + '`): the tool starts it in a session of its own and answers at once — the command runs on, and another step asks after it', 'hold-start', '--scratch ' + v.scratch + ' --name ' + name + ' --kind ' + kind + ' ' + flags + ' ' + digestFlag(v.scratch))
}
function holdWaitPrompt(v, name, what) {
  return stepPrompt('what became of ' + what + ', a command the tool holds (`' + name + '`)', 'hold-wait', '--scratch ' + v.scratch + ' --name ' + name + ' ' + digestFlag(v.scratch), true)
}
function listFlags(flag, values) {
  return values.map((value) => '--' + flag + ' ' + value).join(' ')
}
function digestFlag(scratch) {
  return '--digest ' + scratch
}
function gitStatePrompt(v, look) {
  return stepPrompt((look ? 'LOOKED AT ONLY — nothing is finished, and what is owed is said: ' : '') + 'the state of the stabilization run `' + v.run + '` before its `' + v.stage + '` stage: the branch, the tree, the pushed loop branch, and the path-class assert', 'git-state', '--stage ' + v.stage + ' --loop ' + branchName(v.run) + ' --rounds ' + branchName(v.run, '') + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + (look ? ' --look' : '') + ' ' + digestFlag(v.scratch))
}
function statePrompt(v, tag) {
  return stepPrompt('read the record of the stabilization run `' + v.run + '` (read ' + tag + ')', 'state', '--run ' + v.run + ' --scratch ' + v.scratch + ' --tag ' + tag + ' ' + digestFlag(v.scratch))
}
function beginPrompt(ctx, sha) {
  return stepPrompt('attempt ' + ctx.attempt + ' of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', begun on record: the state must hand the stage exactly this round and attempt, and the attempt\'s marker is written', 'begin', stageFlags(ctx) + ' --attempt ' + ctx.attempt + ' --reporter ' + ATTEMPT + ' --commit ' + sha + ' --scratch ' + ctx.scratch + ' ' + digestFlag(ctx.scratch))
}
function checkReportsPrompt(ctx, names) {
  const vet = names == null
  return stepPrompt(vet ? 'what of the stabilization run `' + ctx.run + '` is written once and in no commit, VETTED before a record that names no stage\'s reporters — the batch of the human\'s rulings: a file there that the record script would not have written leaves the tree' : 'the reports of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', held to the reporters that were launched', 'check-reports', (vet ? '--run ' + ctx.run : stageFlags(ctx) + ' --attempt ' + ctx.attempt) + ' --scratch ' + ctx.scratch + ' ' + digestFlag(ctx.scratch) + (vet ? '' : ' -- ' + names.join(' ')))
}
function findRoundPrompt(v, round) {
  return stepPrompt('find the branch of round ' + round + ' of `' + v.run + '`', 'find-round', '--prefix ' + branchName(v.run, round) + ' ' + digestFlag(v.scratch))
}
function openRoundPrompt(v, round, branch) {
  return stepPrompt('the branch `' + branch + '` of round ' + round + ' of `' + v.run + '`: switch to it, or open it from `' + branchName(v.run) + '`', 'open-round', '--loop ' + branchName(v.run) + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + digestFlag(v.scratch))
}
function pushPrompt(v, branch, record) {
  return stepPrompt((record ? 'a RECORD\'S push — what it publishes is the run\'s records and nothing beside them: ' : '') + 'push `' + branch + '` of the stabilization run `' + v.run + '` by name', 'push', '--branch ' + branch + (record ? ' --run-dir ' + runDir(v.run) : '') + ' ' + digestFlag(v.scratch))
}
function recordCommitPrompt(v, branch, gate, expect, head, fence) {
  return stepPrompt('the ONE commit of a record step of `' + v.run + '` on `' + branch + '`: the applied batch, held to the gate that ran on it', 'record', '--branch ' + branch + ' --run-dir ' + runDir(v.run) + ' --gate ' + gate + ' --calls ' + expect.calls + ' --checks ' + expect.checks + ' --scratch ' + v.scratch + (head ? ' --head ' + head : '') + (fence ? ' --fence ' + fence : '') + ' ' + digestFlag(v.scratch))
}
function wordsText(run, round, attempt, subject, words) {
  return ['run=' + run, 'round=' + round, 'attempt=' + attempt, 'subject=' + subject].concat(words).join('\n') + '\n'
}
function recordWords(rec) {
  const words = rec.reporters.map((name) => 'reporter=' + name).concat(['commit=' + rec.commit], rec.gate ? ['gate=' + rec.gate] : [])
  for (const u of rec.did) words.push('item=' + u.item + ':' + (u.job ? 'held:' + u.job : u.status === 'void' ? 'void:' + u.word : u.status === 'red' ? 'red:' + u.by : 'green'))
  for (const [name, value] of rec.facts) for (const one of Array.isArray(value) ? value : [value]) words.push('fact=' + name + ':' + one)
  return words
}
function composePrompt(ctx, branch, subject, words) {
  return stepPrompt('the record of the `' + ctx.stage + '` stage of `' + ctx.run + '`, round ' + ctx.round + ', attempt ' + ctx.attempt + ', COMPOSED BY THE TOOL from what the stage\'s agents filed and from the ' + words.length + ' plain word(s) after the `--` — and applied as ONE batch. THE WORDS ARE HELD TO THE HASH THE COMMAND NAMES: one word typed otherwise — a status, a name, their order — and the tool refuses, with nothing written', 'compose', '--branch ' + branch + ' --run-dir ' + runDir(ctx.run) + ' --round ' + ctx.round + ' --attempt ' + ctx.attempt + ' --subject ' + shq(subject) + ' --sha256 ' + sha256(wordsText(ctx.run, ctx.round, ctx.attempt, subject, words)) + ' --scratch ' + ctx.scratch + ' ' + digestFlag(ctx.scratch) + ' -- ' + words.join(' '))
}
function wordsAgain(n) {
  return 'AGAIN (' + n + '): an earlier step ran this command and the tool refused it — a word of it was not typed as it stands, so the words did not hash to the sha256 the command names, and nothing was written. Run the ONE command above typed as it stands, word for word and in its order: copy it, and change nothing in it.'
}
function landPrompt(v, round, branch) {
  const loop = branchName(v.run)
  return stepPrompt('land round ' + round + ' of `' + v.run + '`, ONE act: merge `' + branch + '` into `' + loop + '` with a merge commit, then push `' + loop + '` by name. The audit of its fix diff left nothing open', 'land', '--loop ' + loop + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + ' ' + listFlags('log', SYNC_LOGS) + ' ' + digestFlag(v.scratch))
}
function roundCommitsPrompt(v, round, branch) {
  return stepPrompt('the commits of round ' + round + ' of `' + v.run + '` (`' + branch + '`), and which of them touch the run\'s directory', 'round-commits', '--loop ' + branchName(v.run) + ' --branch ' + branch + ' --run-dir ' + runDir(v.run) + ' ' + digestFlag(v.scratch))
}
const CARRY_RULES = 'Run exactly the commands below, in order, and nothing else: the only commits you make are the cherry-picks this step names, one by one. No other branch, no file edit, no `git stash`, no reset, no rebase, no pull, never `--force`, and `main` is never checked out, merged into or pushed. Any check that fails, or any command that fails, is a HALT: stop, leave everything as the step says, and fill the halt report (root_cause = which check, evidence = the command and its output, tree_state = `git status` and `git branch --show-current`).'
function carryPrompt(v, round, shas, part) {
  const loop = branchName(v.run)
  if (!part) return stepPrompt('round ' + round + ' of `' + v.run + '` is dropped: carry exactly its record commits over to `' + loop + '`, so that the round keeps its record', 'carry', '--loop ' + loop + ' --run-dir ' + runDir(v.run) + ' ' + listFlags('product', PRODUCT_PATHS) + ' ' + digestFlag(v.scratch) + ' -- ' + shas.join(' '))
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
function syncMainPrompt(run, scratch) {
  const m = branchName(run)
  return stepPrompt('merge `origin/main` into `' + m + '` before its pull request to `main` is opened (the close of the stabilization run `' + run + '`). Nothing is pushed and no pull request is opened: the gate runs on the merged tree first', 'sync-main', '--loop ' + m + ' ' + listFlags('log', SYNC_LOGS) + ' ' + digestFlag(scratch))
}

function opening(v) {
  return 'The run\'s opening record is `' + runDir(v.run) + '/opening.md`; its state is ' + typed(RECORD_TOOL, 'state --run ' + v.run) + '. ' + GUARD
}
function preflightPrompt(ctx, launch, name, plan) {
  const steps = ['the environment asserts — the tree may hold untracked files under `' + runDir(ctx.run) + '/` and nothing else: what the record script wrote and no record step has committed yet — reports, the round\'s scope, its journal' + (plan.tested ? '. THE CANDIDATE IS NOT `HEAD` HERE: it is the commit the round tested, and the round\'s record commits lie on top of it — commit ' + plan.sha + ' must be an ancestor of `HEAD`, and nothing this call covers reads the working tree' : '')]
  const asked = plan.crossModel ? crossModelAssert() : null
  if (plan.image) steps.push('the trial image, built and verified from the candidate\'s commit')
  if (asked) steps.push(asked)
  if (plan.checks.length) steps.push('the deterministic checks, one per item below, each returned in `checks` under `check` = the item\'s id; a CI run that is still in progress 60 minutes on is void:\n' + plan.checks.map((c) => '   - ' + c.item + ': its brief is the `brief` of the item\'s row — ' + typed(RECORD_TOOL, itemRead(ctx.run, c.item))).join('\n'))
  return [
    'PREFLIGHT — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage. ' + opening(ctx),
    launch.line(name),
    'Candidate: label ' + plan.label + ', commit ' + plan.sha + ' — ' + (plan.tested ? 'the commit round ' + ctx.round + ' tested, an ancestor of' : 'the tip of') + ' `' + plan.branch + '`, which is checked out. Scratch root: `' + ctx.scratch + '`.',
    'NO BUILD AND NO GATE IS YOURS, whatever your definition says of either: the candidate\'s binary, the previous release\'s, the candidate\'s full gate and every scripted check are commands the harness has `' + STEP_TOOL + '` start, hold and judge, by steps of its own. You build nothing, run no gate, and return no binary.',
    'The steps this call covers, in this order, and nothing else:',
  ].concat(steps.map((s2, i) => (i + 1) + '. ' + s2)).join('\n')
}
function scopeText(scope, fallback) {
  if (scope == null) return 'the run\'s default scope, a fact of its record: ' + (fallback === 'everything' ? 'everything — every door of every registry is inside' : 'the derived delta')
  if (typeof scope === 'string') return scope === 'everything' ? 'everything — set by the human: every door of every registry is inside' : 'the derived delta'
  return scope.range ? 'the commit range ' + scope.range + ' — set by the human' : 'the named doors, set by the human: ' + scope.doors.join(' · ')
}
function scopeFile(ctx) {
  return ctx.scratch + '/scope/r' + ctx.round + '.a' + ctx.attempt + '.json'
}
function scopePrompt(ctx, launch, name, plan) {
  return [
    'SCOPE — stabilization run `' + ctx.run + '`, round ' + ctx.round + '. ' + opening(ctx),
    launch.line(name),
    'The round\'s change lies between: base = ' + (plan.base ? plan.base + ' (the candidate round ' + (ctx.round - 1) + ' tested)' : plan.previous.commit + ' (the previous release, ' + plan.previous.version + ', as the run\'s record names it)') + '; tip = ' + plan.sha + ' (label ' + plan.label + ').',
    'The scope this stage was started with: ' + scopeText(plan.scope, plan.fallback) + '.',
    plan.earlier ? 'The door lists of round ' + (ctx.round - 1) + '\'s fixers and fix-diff auditors — inputs, never the result: the `doors_affected` of every report under `' + runDir(ctx.run) + '/r' + (ctx.round - 1) + '/reports/fix/`, which this prompt hands you.' : 'There is no earlier round whose door lists could be an input.',
    'The units of the test set are the rows of `' + runDir(ctx.run) + '/test-set.md` (the state document\'s `items`).',
    'Write the round\'s scope — ONE JSON object, as that script\'s help has it — to the file `' + scopeFile(ctx) + '` WITH YOUR FILE TOOL, and hand it over, once, by ONE plain call, typed as it stands: ' + typed(RECORD_TOOL, 'scope-set --run ' + ctx.run + ' --round ' + ctx.round + ' --scratch ' + ctx.scratch + ' --from ' + scopeFile(ctx)) + '.',
  ].join('\n')
}
function doorList(doors) {
  return doors.length ? doors.map((d) => '   - ' + d.door + '   [' + d.registry + ']').join('\n') : '   (none)'
}
function itemRead(run, item) {
  return 'item --run ' + run + ' --item ' + item
}
function doorsRead(run, round, item) {
  return 'item-doors --run ' + run + ' --round ' + round + ' --item ' + item
}
function kitRead(run, role) {
  return 'kit --run ' + run + ' --role ' + role
}
function kitLine(run, role) {
  return 'YOUR FIRST CALL, before anything else: ' + typed(RECORD_TOOL, kitRead(run, role)) + ' — it prints what every agent of your role looks up: the rules of a report, the slug rule, ' + (ROLES[role].drives ? 'the rig and how a binary is named to it, ' : '') + 'the format of a repro block, and the run\'s closing condition, each read from the file that owns it. Read them there, and look none of them up again. A part it says is NOT PRINTED you read at the file it names; the call then exits 27, which is an answer and no halt.'
}
function rowRead(run, key) {
  return 'row --run ' + run + ' --key ' + key
}
function briefLine(ctx, unit) {
  return unit.brief != null ? 'Your brief: ' + unit.brief : 'Your brief is the `brief` of your item\'s row in the test set — read it: ' + typed(RECORD_TOOL, itemRead(ctx.run, unit.item)) + '.'
}
function doorLines(ctx, unit) {
  return Array.isArray(unit.doors) ? 'The door list — a finding names its door in these exact words:\n' + doorList(unit.doors) : 'The door list — a finding names its door in these exact words — is what ' + typed(RECORD_TOOL, doorsRead(ctx.run, unit.round, unit.item)) + ' prints: ' + (unit.doors == null ? 'the doors' : unit.doors + ' door(s)') + ' of round ' + unit.round + '\'s test set that your item covers, each with its registry and its derivation.'
}
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
  if (role.drives) lines.unshift(kitLine(ctx.run, step.role))
  if (role.drives) lines.push(binaryLine(built, step.image, ctx.scratch))
  lines.push(briefLine(ctx, unit))
  lines.push('Candidate: label ' + built.candidate.label + ', commit ' + built.candidate.sha + '. Scratch root: `' + ctx.scratch + '` — mint your own directory under it.')
  if (unit.range) lines.push('The unit is the diff `' + unit.range + '` over the product paths (' + PRODUCT_PATHS.join(', ') + ').')
  lines.push(doorLines(ctx, unit))
  if (handed.length) lines.push('This prompt hands you these reports of the same item, and no other:\n' + handed.map((h) => '   - ' + h.as + ': ' + (h.report ? '`' + h.report + '`' : '(it left no report)')).join('\n'))
  return lines.join('\n')
}
function triagePrompt(ctx, launch, name, sources, pass) {
  const others = launch.names.filter((n) => n !== name)
  return [
    kitLine(ctx.run, 'triage'),
    'TRIAGE — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage, pass ' + pass + '. ' + opening(ctx),
    launch.line(name),
    'The reporters this stage has launched so far, each with one report of attempt ' + ctx.attempt + ' — a report of the stage beside these is the partial stage you halt on: ' + (others.join(' · ') || '(none)') + '.' + (pass > 1 ? ' This is pass ' + pass + ': it grades only what the agents of the pass before left open; every other finding of the stage was graded then, and is not handed to you again.' : ''),
    'Grade every finding below — ' + sources.reduce((n, s) => n + findingsOf(s), 0) + ' in all, from ' + sources.length + ' reporter(s). The reports are the files named; each list is that reporter\'s structured return, and an entry marked `left open` is what an agent left open, triaged like any finding.' + (sources.some((s) => s.reporter === LEDGER_SOURCE) ? ' The source `' + LEDGER_SOURCE + '` is no reporter of this stage: its entries are rows the ledger already holds, whose triage nobody finished. Grade each under its own key, found again — from the report its repro names, which this prompt hands you with it — and count the source in `findings_in` like any other.' : ''),
  ].concat(sources.map((s) => '- ' + s.reporter + ' — report: ' + (s.report ? '`' + s.report + '`' : '(none)') + ' — ' + findingsOf(s) + ' finding(s):' + (s.read ? ' the rows ' + s.read + ' prints — each whole, under its key, with why its triage is not finished (`ungraded`: nobody graded it; `unverified`: graded, and nobody verified it). Read them there: this prompt carries none of them.' : '\n' + s.findings.map((f) => '   - ' + f).join('\n')))).join('\n')
}
function oneTriagePrompt(ctx, launch, name, sources) {
  const others = launch.names.filter((n) => n !== name)
  const said = (s) => (s.red
    ? '- ' + s.reporter + ' — A RED RESULT, and no reporter: the check `' + s.red.item + '` of this invocation came back red. The stage\'s record files its row under the key `' + s.reporter + '`, in the batch that records your grade — so it gets ONE entry, under that key, found again; you mint no key for it. Grade it from ' + (s.red.verdict ? 'the tool\'s verdict of the held command, the file `' + s.red.verdict + '`' + (s.red.output ? ' (the command\'s whole output is `' + s.red.output + '`)' : '') : 'the report of the preflight that read it red, `' + s.red.report + '`') + ' — 1 finding:'
    : s.read ? '- ' + s.reporter + ' — THE ROWS NOBODY GRADED, and no reporter of this stage: rows the ledger already holds — ' + findingsOf(s) + ' finding(s): the rows ' + s.read + ' prints, NUMBERED 1 TO ' + findingsOf(s) + ' IN THE ORDER THAT READ PRINTS THEM — each whole, under its key. Each is found again, under its own key, graded from the report its repro names. Read them there: this prompt carries none of them.'
      : '- ' + s.reporter + ' — report: ' + (s.report ? '`' + s.report + '`' : '(none)') + ' — ' + findingsOf(s) + ' finding(s):')
    + (s.read ? '' : '\n' + s.findings.map((f, i) => '   ' + (i + 1) + '. ' + f).join('\n'))
  return [
    kitLine(ctx.run, 'triage'),
    'TRIAGE — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', the `' + ctx.stage + '` stage: THE STAGE\'S ONE TRIAGE. ' + opening(ctx),
    launch.line(name) + ' UNDER THIS PROMPT THAT IS A HALT\'S ROAD, AND YOU TAKE IT ONLY WHERE YOU HALT: where you grade, the `triage` call below writes the file at the path a report of your name and attempt has — your report, rendered from your entries — and a report handed over beside it for the same attempt would be refused.',
    'The reporters this stage has launched so far, each with one report of attempt ' + ctx.attempt + ' — a report of the stage beside these is the partial stage you halt on: ' + (others.join(' · ') || '(none)') + '.',
    'Grade every finding below — ' + sources.reduce((n, s) => n + findingsOf(s), 0) + ' in all, from ' + sources.length + ' source(s). EVERY FINDING HAS A NUMBER, from 1 to its source\'s count, under its source\'s name, and an entry says what it folds by the source and those numbers. A reporter\'s report is the file named, its list that reporter\'s structured return, and an entry marked `left open` is what an agent left open, triaged like any finding.',
  ].concat(sources.map(said)).concat([
    'YOUR ENTRIES ARE ONE FILE, HANDED OVER ONCE. Write them — a JSON array, one object an entry — to the file `' + entriesFile(ctx) + '` WITH YOUR FILE TOOL, and hand it over by ONE plain call, typed as it stands: ' + typed(RECORD_TOOL, 'triage --run ' + ctx.run + ' --round ' + ctx.round + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + ' --from ' + entriesFile(ctx) + ' -- ' + sources.map((s) => s.reporter + '=' + findingsOf(s)).join(' ')) + '. Its last words say how many findings each source above handed you, and the record script holds every number of every source to being in exactly one entry.',
    'WHAT YOUR RETURN IS HELD TO, and it holds no entry. That call prints ONE line of JSON. Return status = filed with three things of it: `report`, the path it printed as `triage` — held to `' + testReport(ctx, name) + '`, the path a report of your name and attempt has; `sha256`, the hash it printed beside that path — held to being one; and `handed`, what it printed under that name, one entry a source with its count — held to the sources and the counts of the call above, each by name: a call typed with another count held another partition, and the stage halts on it. Nothing else is read from your return: no entry, no grade and no key is typed a second time, and you name nothing to verify — what is verified is the record script\'s queue. Where you halt, return status = halted with the halt report, a refusal\'s line verbatim in it.',
  ]).join('\n')
}
function verifyPrompt(ctx, launch, name, entry, redrive, built) {
  return [
    kitLine(ctx.run, 'verify'),
    'VERIFY-REAL — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', ONE finding. ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false, ctx.scratch),
    'The finding: ledger key `' + entry.key + '` · door: ' + entry.door + ' · the clause it is said to break: ' + entry.clause + ' · triage\'s grade: ' + entry.grade + ' · its repro block: ' + entry.repro + '.',
    redrive ? 'What triage asks you to re-drive: ' + redrive : '',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].filter(Boolean).join('\n')
}
const PRINTED_ROUTE = 'A ROUTE YOU SAW PRINTED AND DID NOT RUN AS PRINTED IS AN ENTRY OF `hands_on`, whether or not it looks right to you: a route is confirmed only by being run as printed, and one you ran as printed belongs to your verdict.'
function verdictPrompt(ctx, launch, name, key, built, handOn) {
  return [
    kitLine(ctx.run, 'verify'),
    'VERIFY-REAL — stabilization run `' + ctx.run + '`, round ' + ctx.round + ', ONE finding: the row `' + key + '` of the run\'s ledger. ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false, ctx.scratch),
    'THE FINDING IS A ROW, AND YOU READ IT BY ITS KEY: ' + typed(RECORD_TOOL, rowRead(ctx.run, key)) + ' prints it whole — its door, the clause it is said to break, its repro, its grade where triage graded it, its class, and for an item a verifier handed on, who handed it on and under which finding. This prompt carries none of it.',
    'YOUR VERDICT IS ONE FILE, AND YOU FILE IT ONCE — after your report is handed over, and before you return. Write it — ONE JSON object, its members as your definition has them — to the file `' + filingFile(ctx, name) + '` WITH YOUR FILE TOOL, and file it by ONE plain call, typed as it stands: ' + typed(RECORD_TOOL, 'verdict --run ' + ctx.run + ' --round ' + ctx.round + ' --reporter ' + name + ' --attempt ' + ctx.attempt + ' --scratch ' + ctx.scratch + ' --from ' + filingFile(ctx, name)) + ' (with one more `--scratch` for each scratch root of your own outside that one). THE TWO HASHES ARE MEMBERS OF THAT FILE, and no word of the call: `candidate` is the sha256 you asserted for the candidate\'s binary under the `' + LABELS.binary + '` line, and `previous` the one you asserted for the previous release\'s, wherever you drove it.',
    (handOn == null
      ? 'WHAT YOU HAND ON ENTERS NO QUEUE: the run\'s opening names no number of items a verifier hands on, so every entry of `hands_on` is recorded as what it is, in no queue, and the human may put one in by its key.'
      : 'WHAT YOU HAND ON — AT MOST ' + handOn + ' ENTER THE QUEUE: that is the number of the run\'s opening. `hands_on` is in the order you would have them verified: the first ' + handOn + ' that carry the evidence of their kind are handed on to the queue, and a later one is recorded as what it is, in no queue.')
      + ' ' + PRINTED_ROUTE,
    'WHAT YOUR RETURN IS HELD TO, and it holds no verdict. That call prints ONE line of JSON. Return status = filed with three things of it: `filing`, the path it printed as `verdict` — held to `' + verdictFile(ctx, name) + '`, the one path a verdict of your name and attempt is filed at; `word`, the verdict as it printed it — held to ' + FILED_WORDS.join(', ') + '; and `contested`, true or false as it printed it. Nothing else of your verdict is read from your return: no basis, no block and no item you hand on is typed a second time. WHAT COUNTS IS WHAT IS FILED: once your wave is over the harness reads the run\'s queue again, and a finding the queue prints again has no verdict, whatever its verifier returned. Where you halt, return status = halted with the halt report, a refusal\'s line verbatim in it — and file nothing.',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].join('\n')
}
function advocatePrompt(ctx, launch, name, fork, built) {
  return [
    kitLine(ctx.run, 'advocate'),
    'A FORK of the stabilization run `' + ctx.run + '`, round ' + ctx.round + ' — finding `' + fork.key + '` (' + fork.kind + '). ' + opening(ctx),
    launch.line(name),
    binaryLine(built, false, ctx.scratch),
    'The finding: door: ' + fork.door + ' · clause: ' + fork.clause + ' · repro: ' + fork.repro + '.',
    'What raised the fork: ' + fork.statement,
    'The cheap cut is what that statement would leave as it is; argue the robust case, and drive what you propose. Return `proposal` as a change somebody else can apply to a clone of commit ' + built.candidate.sha + ': another agent drives it after you, without your spike.',
    'Scratch root: `' + ctx.scratch + '` — mint your own directory under it.',
  ].join('\n')
}
function reportHandOver(ctx, name) {
  return 'Write it ONCE, when what it reports is done, to the file `' + reportFile(ctx, name) + '` WITH YOUR FILE TOOL — its last line is `<!-- end of report -->`; a correction is made in that file WITH YOUR EDIT TOOL, never by writing it whole again — and hand it over by ONE plain call, typed as it stands: ' + reportCall(ctx, name) + '; return the path it prints as `report`. A refusal about your text you repair in the file, and hand it over again; any other refusal you return as your halt, verbatim. Never remove, recursively, a path built from variables, and read every exit status from the command itself, bare.'
}
function proposalPrompt(ctx, name, fork, advocate, built) {
  return [
    kitLine(ctx.run, 'proposal'),
    'You drive a PROPOSAL you did not write. A fork of the stabilization run `' + ctx.run + '` (round ' + ctx.round + ', finding `' + fork.key + '`) goes to the human with an advocate\'s case; a proposal is a claim, and before the human sees it somebody other than its author runs it end to end — through every OTHER party\'s next step (implementation/milestone-planning-workflow.md, Settle: the claim-driven rule). You are that somebody. Your brief is to REFUTE it: assume a step the advocate reports as passing does not pass.',
    'The proposal, as its author returned it: ' + (advocate.proposal || '(none returned — read it from the report)'),
    'The advocate\'s report, handed to you: ' + (advocate.report ? '`' + advocate.report + '`' : '(none)') + '. The steps it says it drove:\n' + ((advocate.driven || []).map((d) => '   - ' + d.step + ' — ' + d.command + ' — ' + d.result).join('\n') || '   (none)'),
    'HOW. In a directory of your own, minted with `mktemp -d` under `' + ctx.scratch + '`: clone commit ' + built.candidate.sha + ' (`git clone` this repository there and check that commit out, detached — never the working tree, never a branch of this repository), apply the proposal as written, build it there (`cargo build --release --locked`, a target directory of its own), and run every next step the proposal names, each as printed, each exit status read bare. Where the proposal names a step you cannot run, say so in `undriven`, with why.',
    'THE BEFORE of each step is the candidate itself, and its hash is ASKED OF THE TOOL, by ONE plain call typed as it stands, and never computed by a command of your own: ' + typed(STEP_TOOL, 'hash --scratch ' + ctx.scratch + ' --file ' + below(ctx.scratch, built.candidate.binary)) + ' prints ONE line of JSON whose `content_sha256` must be ' + built.candidate.sha256 + ', and you return that hash as `asserted_sha256`.',
    'WHAT YOU RETURN. One entry of `steps` per next step: the command, what it printed and exited, and `agrees` — whether that is what the advocate reported. `holds` is true only when every step ran as the proposal says. What you hit that is not this proposal goes in `left_open`.',
    'WHAT YOU MAY TOUCH. Nothing in this repository: no edit, no commit, no branch, no push, no install. One file reaches it — your report. ' + reportHandOver(ctx, name) + ' ' + GUARD,
    NEVER,
  ].join('\n')
}
function crossModelTool() {
  return 'codex'
}
function crossModelAssert() {
  return 'whether the cross-model pass\'s tool answers, returned in `checks` under `check` = `' + CROSS_CHECK + '`: green when `command -v ' + crossModelTool() + '` answers and `' + crossModelTool() + ' --version` exits 0, void when either does not — with what it printed. It is NOT one of the environment asserts: void is an answer, never a halt, and nothing is installed'
}
function crossModelPrompt(ctx, name, unit) {
  const tool = crossModelTool()
  return [
    'A CROSS-MODEL SOURCE PASS of review row `' + unit.item + '` — stabilization run `' + ctx.run + '`, round ' + ctx.round + '. This invocation named this item for it (args.crossModel): a cross-model second opinion is a human-approved suggestion, never auto-run (implementation/milestone-planning-workflow.md, Review), and naming the item was the approval. You relay what a model of another family claims about this row\'s source; you judge none of it and drive none of it.',
    'HOW. `command -v ' + tool + '` must answer — if it does not, halt and say so: nothing is installed. In a directory of your own, minted with `mktemp -d` under `' + ctx.scratch + '`, write the pass\'s prompt to a file: the row\'s brief (the `brief` of the item\'s row, which ' + typed(RECORD_TOOL, itemRead(ctx.run, unit.item)) + ' prints — read it, and hand over what it says to review), the door list below, and the instruction to return each claim with its `file:line` and the door it stands at. Then run `' + tool + ' review - < <the prompt file> > <an output file> 2>&1`, its exit status read bare, and read the output file.',
    'WHAT ITS CLAIMS ARE. Leads, never findings: a claim one model makes that nobody reproduced is a lead, and the row\'s reconciler drives every one. Return each as a finding with `lead` set to true, its door in the exact words of the list below (or named plainly and marked unlisted), the clause it would break as the claim reads, and `repro` = the heading you give the claim in your report. A pass that returned nothing usable is a halt, with its output as the evidence.',
    doorLines(ctx, unit),
    'WHAT YOU MAY TOUCH. Nothing in this repository: no edit, no commit, no branch, no push, no install. One file reaches it — your report, the pass\'s output whole and your list of leads. ' + reportHandOver(ctx, name) + ' ' + GUARD,
    NEVER,
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

function payload(dir, name, value) {
  const text = JSON.stringify(value)
  const file = dir + '/' + name
  return {
    file,
    sha256: sha256(text + '\n'),
    write: 'Write the file `' + file + '` WITH YOUR FILE TOOL — the tool that writes a file, and no shell: its content is exactly the ONE line that stands between the two marker lines below — neither marker, nothing before it, and ONE newline after it.\n' + PAYLOAD_ENDS + '\n' + text + '\n' + PAYLOAD_ENDS,
  }
}
const RECORD_RUNS = 'Run these from the repository\'s root, in this order, each exactly as written and each exit status read bare. A refusal is a halt — its one line the evidence — and nothing it names is repaired by hand:'
const RECORD_RETURNS = 'YOU MAKE NO COMMIT, stage nothing, and RUN NO GATE — whatever your definition says of one: the harness holds the full gate itself, as a command of `' + STEP_TOOL + '`, by steps of its own that follow this one, and then makes the one commit. Return status = applied as soon as step 2 printed its line. A halt is a refusal of step 2 — and with it return refused = the ONE word that follows `refused` on the line it printed on stderr. One refusal is yours to repair, once: `bad-value`, naming the file as not the batch that was hashed — write the file again, exactly, and run step 2 again; refused a second time, it is your halt.'
function subjectOf(run, round, what) {
  return run + ' r' + round + ' - ' + what
}
function call(flags, more, stdin) {
  const made = { argv: flags.split(' ').concat(more || []) }
  if (stdin !== undefined) made.stdin = JSON.stringify(stdin)
  return made
}
function isCheck(made) {
  return made.argv[0].startsWith('check-')
}
function recordHold(dir) {
  return 'record-' + dir.slice(dir.lastIndexOf('/') + 1)
}
function recordPrompt(v, what, branch, dir, round, calls, subject) {
  const batch = payload(dir, 'batch-' + sha256(JSON.stringify(calls) + '\n').slice(0, 12) + '.json', calls)
  const text = [
    LABELS.record + ' — stabilization run `' + v.run + '`: ' + what + '.',
    branchLine(branch, branchName(v.run)),
    RECORD_RUNS,
    '1. ' + batch.write,
    '2. ' + typed(RECORD_TOOL, 'apply --run ' + v.run + (round ? ' --round ' + round : '') + ' --subject ' + shq('docs(record): ' + subject) + ' --scratch ' + v.scratch + ' --from ' + batch.file + ' --sha256 ' + batch.sha256) + ' — the ' + calls.length + ' call(s) of this record as ONE batch, read from the file of step 1 and held to its hash: every table is written, or none is. It prints one line of JSON.',
    RECORD_RETURNS,
  ].join('\n')
  return { text, hold: recordHold(dir), expect: { calls: calls.length, checks: calls.filter(isCheck).length } }
}
function pendingRecord(dir, pending) {
  return { text: null, hold: recordHold(dir), pending: true, expect: { calls: pending.calls, checks: pending.checks } }
}
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
function rulingsRecordPrompt(v, round, branch, rulings, ran, at) {
  const calls = []
  const about = rulings.filter((r) => runRuling(r))
  for (const r of about.filter((x) => !QUEUE_RULINGS.some((name) => x[name] != null))) {
    const facts = r.go != null ? [{ value: { go: true }, call: 'round-set --run ' + v.run + ' --round ' + round }]
      : r.rerun != null ? ran[r.rerun].map((spentIn) => ({ value: { granted: r.rerun }, call: 'round-set --run ' + v.run + ' --round ' + spentIn }))
        : r.reverify != null ? [{ value: { reverify: r.reverify }, call: 'round-set --run ' + v.run + ' --round ' + at.round }]
          : r.again != null ? [{ value: { again: r.again }, call: 'round-set --run ' + v.run + ' --round ' + at.stages[r.again] }]
            : r.cycles != null ? [{ value: { cycles: r.cycles }, call: 'run-set --run ' + v.run }]
              : [{ value: { rounds: r.rounds }, call: 'run-set --run ' + v.run }]
    for (const fact of facts) calls.push(call(fact.call, [], fact.value))
  }
  for (const name of QUEUE_RULINGS) for (const r of about.filter((x) => x[name] != null)) calls.push(call('run-set --run ' + v.run, [], { [name]: r[name] }))
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
  return recordPrompt(v, 'the human\'s rulings — ' + patches.length + ' on a finding, ' + bounds.length + ' declared bound(s), ' + about.length + ' about the run. They are the human\'s, relayed: record them as given, and judge none of them', branch, v.scratch + '/record/rulings-r' + round, round, calls, subjectOf(v.run, round, 'the rulings of the human'))
}

function probeStep(what, act, flags) {
  return stepPrompt('a PROBE of the stabilization harness, of no run — ' + what, act, flags).replace('1. `' + STEP_TOOL + ' ', '1. `' + PROBE_TOOL + ' ')
}
function probeReviewPrompt(id, binary) {
  const lines = [
    'A PROBE of the stabilization harness\'s runtime — the `required` probe, case ' + id + '. It is no stage of any run: there is no round, no test set and NO REPORT — you write no file, and you run no command of `dev/stabilize-record`.',
    binaryLine({ candidate: { binary: binary.file, sha256: binary.sha256, sha: '0'.repeat(40), label: 'probe' } }, false),
    'Your brief: one review row\'s SOURCE PASS, and a small one — read the comment block `' + PROBE_TOOL + '` opens with, and return what you find wrong in it as findings. None is expected, and an empty `findings` is an answer. YOU DRIVE NOTHING: the file on the line above is a stand-in and no build of any commit — the commit that line names is forty zeros and names nothing — and no command of it is run.',
    'The door list — a finding names its door in these exact words:\n' + doorList([{ door: PROBE_TOOL, registry: 'tooling' }]),
  ]
  if (id === 'a') lines.push('THIS CASE PROBES WHAT THE RUNTIME DOES WITH A RETURN THAT LACKS A FIELD ITS SCHEMA REQUIRES. Leave `asserted_sha256` OUT of your return: the field is not there at all — not empty, not null, not a placeholder — whatever the schema of the return says of it. If your return is refused for lacking it, return the same object again, still without it: you never fill it in.')
  return lines.join('\n')
}
function probeField(back) {
  if (!back) return 'none'
  if (back.status === 'halted') return 'halted'
  if (back.asserted_sha256 == null) return 'absent'
  return typeof back.asserted_sha256 === 'string' && SHA256_RE.test(back.asserted_sha256) ? back.asserted_sha256 : 'malformed'
}
function triesOf(tries) {
  return tries.map((t) => t.ended[0]).join('')
}
function utf8Length(text) {
  let n = 0
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i)
    if (c >= 0xd800 && c < 0xdc00 && i + 1 < text.length) {
      n += 4
      i++
    } else n += c < 0x80 ? 1 : c < 0x800 ? 2 : 3
  }
  return n
}
function probeRelayed(raw, read) {
  const line = raw && typeof raw.line === 'string' ? raw.line.replace(/\n$/, '') : null
  if (line == null) return 'none:0'
  if (read.relay || !SHA256_RE.test(String(read.sha256))) return 'altered:' + utf8Length(line)
  return read.sha256 + ':' + utf8Length(line)
}
function probeBatch(scratch, n) {
  const ctx = { run: 'probe', round: 1, stage: 'test', attempt: 1, scratch }
  const entries = []
  for (let i = 1; i <= n; i++) {
    const id = String(i).padStart(3, '0')
    entries.push({ key: 'probe-finding-' + id, new: true, doctype: RED_DOCTYPE, source: 'row-' + (1 + i % 12) + '-review', door: 'jigc doc set', clause: 'no-lost-files', repro: 'F' + id + ' — `jigc doc set` drops the slot it was handed → the read that follows says "ok" and lacks it', grade: 'breaks', verdict: 'confirmed', regression: false })
  }
  const rec = triageRecord(ctx, entries)
  const calls = stageRecordCommands(ctx, { reporters: [ATTEMPT, 'triage-p1'], gate: null, results: null, rows: rec.rows, patches: [], triage: rec.triage, facts: null, keys: rec.rows.map((r) => r.key) })
  return payload(scratch + '/probe/payload/e' + n, 'batch.json', calls)
}
function probePayloadPrompt(scratch, n, batch) {
  return [
    LABELS.record + ' — a PROBE of the stabilization harness, of no run (the `payload` probe: a record\'s batch of ' + n + ' entries, written and hashed). NOTHING IS APPLIED, GATED OR COMMITTED: no command of `dev/stabilize-record` and no gate is yours here, you make no commit, and no file of the repository is written.',
    RECORD_RUNS,
    '1. ' + batch.write,
    '2. ' + typed(PROBE_TOOL, 'hash --scratch ' + scratch + ' --file ' + batch.file) + ' — it prints ONE line of JSON.',
    'Report status = ran and line = ' + AS_PRINTED + ' — the line of step 2. Only when step 2 printed no such line: status = halted, and the halt report.',
  ].join('\n')
}
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
    ['a budget of no verdict', Object.assign({}, base, { rulings: [{ budget: 0 }] })], ['a budget that is no number', Object.assign({}, base, { rulings: [{ budget: '40' }] })], ['half a verdict', Object.assign({}, base, { rulings: [{ budget: 2.5 }] })], ['a budget said twice', Object.assign({}, base, { rulings: [{ budget: 40 }, { budget: 41 }] })], ['a budget beside a finding\'s ruling', Object.assign({}, base, { rulings: [{ budget: 40 }, { key: 'f-1', ruling: later }] })], ['a budget beside a scope', Object.assign({}, base, { scope: 'everything', rulings: [{ budget: 40 }] })],
    ['an order of no class', Object.assign({}, base, { rulings: [{ order: [] }] })], ['an order that is no list', Object.assign({}, base, { rulings: [{ order: 'route hunt' }] })], ['an order that names a class twice', Object.assign({}, base, { rulings: [{ order: ['route hunt', 'route hunt'] }] })], ['an order of half a class', Object.assign({}, base, { rulings: [{ order: ['route'] }] })], ['an order and a budget in one entry', Object.assign({}, base, { rulings: [{ order: ['route hunt'], budget: 40 }] })], ['an order beside a tuning stop', Object.assign({}, base, { stopAfter: 'state', rulings: [{ order: ['route hunt'] }] })],
    ['a hand-on number, which no ruling changes', Object.assign({}, base, { rulings: [{ 'hand-on': 3 }] })], ['a hand-on number on fix', Object.assign({}, fix, { rulings: [{ 'hand-on': 2 }] })], ['a hand-on number beside a budget', Object.assign({}, base, { rulings: [{ budget: 40 }, { 'hand-on': 2 }] })], ['a hand-on number in a budget\'s entry', Object.assign({}, base, { rulings: [{ budget: 40, 'hand-on': 2 }] })],
    ['no row put into the queue', Object.assign({}, base, { rulings: [{ queued: [] }] })], ['a row put into the queue that is no key', Object.assign({}, base, { rulings: [{ queued: ['Not a key'] }] })], ['a row put into the queue twice', Object.assign({}, base, { rulings: [{ queued: ['f-1', 'f-1'] }] })],
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
    ['a probe nobody has', { probe: 'everything', scratch: '/tmp/scratch-1' }], ['a probe that is no name', { probe: true, scratch: '/tmp/scratch-1' }], ['a probe of no name', { probe: '', scratch: '/tmp/scratch-1' }],
    ['a probe with no scratch', { probe: 'relay' }], ['a probe with a relative scratch', { probe: 'relay', scratch: 'tmp/x' }], ['a probe with a scratch that climbs', { probe: 'hold', scratch: '/tmp/../etc' }], ['a probe with a quote in its scratch', { probe: 'payload', scratch: '/tmp/a\'b' }],
    ['a probe beside a stage', { probe: 'relay', stage: 'test', scratch: '/tmp/scratch-1' }], ['a probe beside a run', { probe: 'relay', run: 'rc24-tier1', scratch: '/tmp/scratch-1' }], ['a probe beside a stage and a run', Object.assign({ probe: 'payload' }, base)], ['a probe beside a stage that refuses', Object.assign({ probe: 'hold' }, fix)],
    ['a probe beside a model', { probe: 'required', scratch: '/tmp/scratch-1', model: 'sonnet' }], ['a probe beside a ruling about the run', { probe: 'hold', scratch: '/tmp/scratch-1', rulings: [{ go: true }] }], ['a probe beside a tuning stop', { probe: 'relay', scratch: '/tmp/scratch-1', stopAfter: 'state' }],
    ['seconds on a probe that holds nothing', { probe: 'relay', scratch: '/tmp/scratch-1', seconds: 60 }], ['a hold of no time', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: 0 }], ['a hold of half a second', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: 0.5 }],
    ['a hold that is no number', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: '720' }], ['a hold past its bound', { probe: 'hold', scratch: '/tmp/scratch-1', seconds: PROBE_HOLD_MAX + 1 }], ['seconds on a stage', Object.assign({}, base, { seconds: 60 })],
  ].concat([true, false, 'all', '*', 'every', 'row-3', 1, [], {}, { all: true }, ['*'], ['all rows'], [true]].map((every) => ['a cross-model pass for every item: ' + JSON.stringify(every), Object.assign({}, base, { crossModel: every })]))
  for (const [name, given] of refused) check('refused: ' + name, typeof validateArgs(given) === 'string')
  const taken = PROBES.map((probe) => ({ probe, scratch: '/tmp/scratch-1' })).concat([
    { probe: 'hold', scratch: '/tmp/scratch-1', seconds: 1 }, { probe: 'hold', scratch: '/tmp/scratch-1', seconds: PROBE_HOLD_MAX },
    base, fix, { selfTest: true }, Object.assign({}, base, { scope: 'everything' }), Object.assign({}, base, { scope: { range: 'abcdef1..1234567' } }),
    Object.assign({}, base, { scope: { doors: ['jigc setup', 'jigc doc show'] } }), Object.assign({}, base, { stopAfter: 'preflight', model: 'sonnet' }),
    Object.assign({}, base, { clause: 'no-lost-files' }), Object.assign({}, base, { crossModel: ['row-3'] }), Object.assign({}, base, { crossModel: ['row-3', 'row-7'] }),
    Object.assign({}, fix, { raise: { cycles: DEFAULT_CYCLES + 1 } }), Object.assign({}, fix, { exit: 'drop' }), Object.assign({}, fix, { exit: { part: ['abcdef1', '0123456789abcdef0123456789abcdef01234567'] } }),
    Object.assign({}, base, { rulings: [{ go: true }] }), Object.assign({}, fix, { rulings: [{ go: true }, { rerun: 'no-lost-files' }, { rerun: 'no-regression' }] }), Object.assign({}, fix, { rulings: [{ rounds: 4 }] }), Object.assign({}, base, { rulings: [{ rounds: 4 }, { rerun: 'no-lost-files' }] }), Object.assign({}, base, { rulings: [{ reverify: 'f-1' }, { reverify: 'f-2' }, { again: 'test' }] }), Object.assign({}, fix, { rulings: [{ cycles: 4 }, { again: 'fix' }] }),
    Object.assign({}, base, { rulings: [{ budget: 40 }] }), Object.assign({}, base, { rulings: [{ order: ['route under-confirmed', 'behaviour hunt'] }, { budget: 40 }] }), Object.assign({}, fix, { rulings: [{ order: ['route hunt'] }] }), Object.assign({}, base, { rulings: [{ queued: ['f-1', 'f-2'] }, { budget: 40 }, { order: ['route hunt'] }, { go: true }] }),
    Object.assign({}, base, { rulings: [{ key: 'f-1', ruling: later }] }), Object.assign({}, base, { rulings: [{ key: 'f-1', ruling: admitted, note: 'n' }, { bound: 'b', reach: 'x', where: 'y', pin: 'unpinned' }] }),
    Object.assign({}, fix, { rulings: [{ key: 'f-1', ruling: admitted, note: 'build the robust path' }, { key: 'f-2', ruling: later }, { key: 'f-3', ruling: bound, bound: 'non-jigc-writer', reach: 'races against a writer that is not jigc', where: 'the stop after round 1, item 3', pin: 'unpinned' }, { bound: 'planted-state', reach: 'a state nobody reaches', where: 'the stop after round 1', pin: 'flow12::planted' }] }),
  ])
  for (const given of taken) check('taken: ' + JSON.stringify(given), validateArgs(given) === null)

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

  check('the loop branch', branchName('rc24-tier1') === 'fix/rc24-tier1')
  check('a round branch', branchName('rc24-tier1', 2) === 'fix/rc24-tier1-r2' && branchName('rc24-tier1', 2, 1) === 'fix/rc24-tier1-r2')
  check('a part', branchName('rc24-tier1', 2, 3) === 'fix/rc24-tier1-r2-part2')
  check('the prefix of every round', branchName('rc24-tier1', 1).startsWith(branchName('rc24-tier1', '')) && branchName('rc24-tier1', 12, 2).startsWith(branchName('rc24-tier1', '')))
  check('a round is not a prefix of the loop', !branchName('rc24-tier1').startsWith(branchName('rc24-tier1', '')))
  check('the current cut', currentCut('x', 1, []) === 0 && currentCut('x', 1, ['fix/x-r1']) === 1 && currentCut('x', 1, ['fix/x-r1', 'fix/x-r1-part1', 'fix/x-r11']) === 2 && currentCut('x', 2, ['fix/x-r1']) === 0)

  check('sha256 of nothing', sha256('') === 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855')
  check('sha256 of abc', sha256('abc') === 'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad')
  check('sha256 over two blocks', sha256('abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq') === '248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1')
  check('sha256 of a million a', sha256('a'.repeat(1000000)) === 'cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0')
  check('sha256 outside ASCII', sha256('\u00e9') === '4a99557e4033c3539de2eb65472017cad5f9557f7a0625a09f1c3f6e2ba69c4c' && sha256('\u2014 \u00e9 \ud83d\ude00') === 'ce9b24e5ffe38e22059636aef28334557e630cf2206d2af214d779e63f7b0b51')

  const doc = { run: 'rc24-tier1', opened: true, next: 'fix', position: { test: { round: 1, attempt: 1 } } }
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
  const clauseTable = printed({ act: 'state', status: 'read', state: { clauses: [['no-lost-files', 'never-selected'], ['working-product', 'never-selected'], ['usable-by-agents', 'no-instrument'], ['migration-works', 'no-instrument']].map(([clause, retry]) => ({ clause, status: 'void', round: null, commit: null, behind: null, retry })) } })
  const slipped = clauseTable.replace('"working-product","status":"void","round":null,"commit":null,"behind":null,"retry":"never-selected"', '"working-product","status":"void","round":null,"commit":null,"behind":null,"retry":"no-instrument"')
  check('a relay with one value slipped to another the line holds under that name is read as the tool printed it, and says which value', slipped !== clauseTable && relay(slipped).status === 'read' && JSON.stringify(relay(slipped)) === JSON.stringify(relay(clauseTable)) && relay(slipped).state.clauses[1].retry === 'never-selected' && JSON.stringify(stepRead({ status: 'ran', line: slipped + '\n' }, 'state').mended) === JSON.stringify({ key: 'retry', relayed: '"no-instrument"', printed: '"never-selected"' }) && stepRead({ status: 'ran', line: clauseTable }, 'state').mended === null)
  check('… and no line is read that does not hash: two slips, a value that stands nowhere else in the line under its name, a read that is exact, a hash that is not the line\'s', !!relay(slipped.replace('"migration-works","status":"void"', '"migration-works","status":"read"')).relay && relay(clauseTable.replace('"retry":"never-selected"', '"retry":"due"')).state.clauses[0].retry === 'never-selected' && !!relay(clauseTable.replace('"status":"read"', '"status":"void"')).relay && !!readStep({ status: 'ran', line: slipped }, 'state', true).relay && !!relay(slipped.replace(/"sha256": "[0-9a-f]{64}"/, '"sha256": "' + 'f'.repeat(64) + '"')).relay)
  const refusedLine = printed({ act: 'land', status: 'halted', refused: 'merge-differs', merge_commit: 'a'.repeat(40), halt: { root_cause: 'merge-differs: x', evidence: 'e', tree_state: 't', recommendation: 'r' } })
  check('a refusal is read as a halted step that says why, with what the act had established', relay(refusedLine, 'land').status === 'halted' && relay(refusedLine, 'land').halt.root_cause === 'merge-differs: x' && relay(refusedLine, 'land').merge_commit === 'a'.repeat(40) && !relay(refusedLine, 'land').relay)
  for (const next of ['fix', 'rule', 'close', 'verify', 'unsettled', 'retest', 'stop', 'test', 'triage', 'not-ready', '', null, 7]) {
    const out = outcomeOf({ next })
    check('next relayed verbatim: ' + JSON.stringify(next), out.next === next && out.known === ['fix', 'rule', 'close', 'verify'].includes(next))
  }
  const ruled = outcomeOf({ next: 'rule', human_list: 2, reverify: ['f-1'], human_clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }] })
  check('`rule` goes back with all three of the human\'s lists', JSON.stringify(ruled.rule) === JSON.stringify({ findings: 2, reverify: ['f-1'], clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }], stages: [] }) && JSON.stringify(outcomeOf({ next: 'rule', human_stages: [{ stage: 'test', round: 1 }] }).rule) === JSON.stringify({ findings: 0, reverify: [], clauses: [], stages: [{ stage: 'test', round: 1 }] }) && outcomeOf({ next: 'fix', human_list: 1 }).rule === undefined)
  for (const word of ['not-ready', 'no-round', 'not-tested', 'round-open', 'round-over', 'round-bound', 'cycle-bound', 'attempts-spent', 'stopped']) check('a refusal the orchestrator can act on: ' + word, isText(refusalOf({ refused: word, round: 1 })) && !refusalOf({ refused: word, round: 1 }).includes('no sentence'))
  check('a refusal nobody defined is said to be one', refusalOf({ refused: 'round-closed', round: 1 }).includes('no sentence') && refusalOf({ refused: 'not-ready', round: null }).includes('run-set') && refusalOf({ refused: 'round-bound', round: 3 }).includes('run-set'))
  check('a refusal the human lifts names the ruling that lifts it', refusalOf({ refused: 'stopped', round: 1 }).includes('args.rulings, `go`') && refusalOf({ refused: 'round-bound', round: 3 }).includes('args.rulings, `rounds`') && refusalOf({ refused: 'cycle-bound', round: 1 }).includes('args.rulings, `cycles`') && refusalOf({ refused: 'attempts-spent', round: 1 }).includes('args.rulings, `again`') && refusalOf({ refused: 'round-open', round: 1 }).includes('`triage`'))

  const stoppedState = { next: 'stop', stop: { why: 'every-round', round: 2, then: 'test' }, human_clauses: [{ clause: 'no-lost-files', why: 'not-green-after-its-rerun' }], facts: { stop: 'every-round', rounds: 3 } }
  const boundState = { next: 'stop', stop: { why: 'round-bound', round: 4, then: 'fix' }, human_clauses: [], facts: { stop: 'at-the-bound', rounds: 3 } }
  check('a go is taken at the stop after a round', runRulingsFault([{ go: true }], stoppedState) === null && runRulingsFault([{ go: true }, { rerun: 'no-lost-files' }, { rounds: 4 }], stoppedState) === null)
  check('a go is taken at no other state', [boundState, { next: 'close', stop: null }, { next: 'fix', stop: null }, { next: 'stop', stop: null }, {}].every((state) => typeof runRulingsFault([{ go: true }], state) === 'string'))
  check('one more re-run is granted to a clause that is the human\'s, and to no other', typeof runRulingsFault([{ rerun: 'no-regression' }], stoppedState) === 'string' && typeof runRulingsFault([{ rerun: 'no-lost-files' }], boundState) === 'string' && typeof runRulingsFault([{ rerun: 'no-lost-files' }], {}) === 'string')
  check('the bound is raised, never lowered or said again', runRulingsFault([{ rounds: 4 }], boundState) === null && typeof runRulingsFault([{ rounds: 3 }], boundState) === 'string' && typeof runRulingsFault([{ rounds: 2 }], boundState) === 'string' && runRulingsFault([{ rounds: 1 }], { facts: { stop: 'every-round', rounds: null } }) === null)
  const cycleState = { next: 'stop', stop: { why: 'cycle-bound', round: 1, then: 'fix' }, facts: { stop: 'at-the-bound', rounds: 3, cycles: null } }
  const retriedState = { next: 'rule', stop: null, human_list: 2, reverify: ['f-1'], human_stages: [{ stage: 'test', round: 2 }] }
  const begun = { rounds: [{ round: 1 }], not_ready: [] }
  check('the bound across rounds is raised at its own stop once a round is begun, and written before that', runRulingsFault([{ rounds: 4 }], Object.assign({}, boundState, begun)) === null && typeof runRulingsFault([{ rounds: 4 }], Object.assign({}, stoppedState, begun)) === 'string' && typeof runRulingsFault([{ rounds: 4 }], Object.assign({ next: 'fix', stop: null, facts: { rounds: 3 } }, begun)) === 'string' && runRulingsFault([{ rounds: 4 }], { next: 'test', stop: null, facts: { rounds: 3 }, rounds: [], not_ready: [] }) === null)
  check('the bound on a round\'s cycles is raised at its own stop, and at no other', runRulingsFault([{ cycles: 4 }], cycleState) === null && typeof runRulingsFault([{ cycles: 4 }], boundState) === 'string' && typeof runRulingsFault([{ cycles: 4 }], stoppedState) === 'string' && typeof runRulingsFault([{ cycles: 4 }], Object.assign({}, cycleState, { facts: { cycles: 5 } })) === 'string')
  check('one more triage is granted to a finding that is the human\'s after its retry, and to no other', runRulingsFault([{ reverify: 'f-1' }], retriedState) === null && typeof runRulingsFault([{ reverify: 'f-2' }], retriedState) === 'string' && typeof runRulingsFault([{ reverify: 'f-1' }], stoppedState) === 'string' && typeof runRulingsFault([{ reverify: 'f-1' }], {}) === 'string')
  check('one more attempt is granted to a stage that is the human\'s, and to no other', runRulingsFault([{ again: 'test' }], retriedState) === null && typeof runRulingsFault([{ again: 'fix' }], retriedState) === 'string' && typeof runRulingsFault([{ again: 'test' }], stoppedState) === 'string')
  check('which rulings are about the run', runRuling({ reverify: 'f-1' }) && runRuling({ again: 'test' }) && runRuling({ cycles: 4 }) && runRuling({ go: true }) && runRuling({ rerun: 'x' }) && runRuling({ rounds: 4 }) && !runRuling({ key: 'f-1', ruling: later }) && !runRuling({ bound: 'b', reach: 'x', where: 'y', pin: 'z' }) && !runRuling(null) && !runRuling('go'))
  const whyNot = (rulings) => String(validateRulings(rulings))
  check('which rulings are about the queue of a run', runRuling({ budget: 40 }) && runRuling({ order: ['route hunt'] }) && runRuling({ queued: ['f-1'] }) && JSON.stringify(RUN_RULINGS.slice(-3)) === '["order","queued","budget"]')
  check('a budget is refused before any agent as anything but a whole number of verdicts', [0, -1, 2.5, '40', true, 1000000].every((n) => whyNot([{ budget: n }]).startsWith('`budget` is ')) && validateRulings([{ budget: 1 }]) === null && validateRulings([{ budget: 999999 }]) === null)
  check('an order is refused before any agent as anything but a list of classes, each a kind and where it comes from, none twice', [[], 'route hunt', ['route'], ['route hunt', 'route hunt'], ['Route hunt'], [3], ['route  hunt'], ['route hunt '], ['route hunt seed']].every((order) => whyNot([{ order }]).startsWith('`order` is ')) && validateRulings([{ order: ['route under-confirmed', 'behaviour under-not-a-break', 'route hunt'] }]) === null)
  check('the rows put into the queue are refused before any agent as anything but ledger keys, none twice', [[], 'f-1', ['Not a key'], ['f-1', 'f-1'], [7]].every((queued) => whyNot([{ queued }]).startsWith('`queued` is ')) && validateRulings([{ queued: ['f-1', 'f-2'] }]) === null)
  check('the number a verifier hands on is refused before any agent, whatever it says and whatever stands beside it: it is written once', [[{ 'hand-on': 2 }], [{ 'hand-on': 0 }], [{ 'hand-on': '2' }], [{ budget: 40 }, { 'hand-on': 3 }], [{ budget: 40, 'hand-on': 3 }], [{ key: 'f-1', ruling: later, 'hand-on': 3 }]].every((rulings) => whyNot(rulings).startsWith('`hand-on` is no ruling') && whyNot(rulings).includes('written ONCE') && whyNot(rulings).includes('`exists`')) && !runRuling({ 'hand-on': 2 }))
  check('each ruling about the queue is said once, in an entry of its own, and never beside a ruling on a finding', whyNot([{ budget: 40 }, { budget: 41 }]).includes('more than once') && whyNot([{ budget: 40, order: ['route hunt'] }]).includes('names one thing') && whyNot([{ order: ['route hunt'] }, { key: 'f-1', ruling: later }]).includes('mixes') && validateRulings([{ order: ['route hunt'] }, { budget: 40 }, { queued: ['f-1'] }]) === null)
  const queueAt = (queue) => ({ next: 'rule', stop: null, queue: Object.assign({ budget: 3, hand_on: 2, spent: 3, left: 0, length: 1, first: ['f-9'], classes: [] }, queue) })
  check('a ruling about the queue is refused where the run has none', [{ budget: 40 }, { order: ['route hunt'] }, { queued: ['f-1'] }].every((r) => String(runRulingsFault([r], { next: 'rule', stop: null, queue: null })).includes('has no queue') && String(runRulingsFault([r], {})).includes('has no queue')))
  check('a raise with no order and no key beside it is taken only where the digest shows the budget spent and a row of the queue unreached, and is never a lower number or the same', runRulingsFault([{ budget: 4 }], queueAt({})) === null && String(runRulingsFault([{ budget: 3 }], queueAt({}))).includes('`budget` raises it, and 3 does not') && String(runRulingsFault([{ budget: 2 }], queueAt({}))).includes('does not') && String(runRulingsFault([{ budget: 4 }], queueAt({ spent: 2, left: 1 }))).includes('raised only while it is spent and the queue holds a row nobody reached (2 of 3 spent, 1 in the queue)') && String(runRulingsFault([{ budget: 4 }], queueAt({ length: 0, first: [] }))).includes('(3 of 3 spent, 0 in the queue)'))
  check('a raise beside another order or a row put into the queue is the record script\'s to judge, on the state those leave', runRulingsFault([{ budget: 4 }, { order: ['route hunt'] }], queueAt({ length: 0, first: [] })) === null && runRulingsFault([{ queued: ['f-1'] }, { budget: 4 }], queueAt({ length: 0, first: [] })) === null && String(runRulingsFault([{ order: ['route hunt'] }, { budget: 3 }], queueAt({ length: 0 }))).includes('does not'))
  check('a budget is written at any state where the run holds none', runRulingsFault([{ budget: 4 }], queueAt({ budget: null, left: null, spent: 0, length: 0 })) === null)
  check('another order, and a row put into the queue, are taken at any state of a run with a queue', [queueAt({}), queueAt({ spent: 1, left: 2 }), Object.assign(queueAt({}), { next: 'verify' }), Object.assign(queueAt({}), { next: 'stop', stop: { why: 'every-round', round: 1, then: 'test' } })].every((state) => runRulingsFault([{ order: ['route hunt'] }, { queued: ['f-1'] }], state) === null))

  const awaiting = { untriaged: { count: 2, why: [{ why: 'ungraded', count: 1 }, { why: 'unverified', count: 1 }] }, ledger: 3 }
  const handed = ledgerSource('rc24-tier1', awaiting)
  check('the rows whose triage is unfinished are one source, named for the ledger', handed.length === 1 && handed[0].reporter === LEDGER_SOURCE && handed[0].report === 'completions/artifacts/rc24-tier1/ledger.md' && findingsOf(handed[0]) === 2)
  check('the rows are handed over by the read that prints them, and none by this script', handed[0].findings.length === 0 && handed[0].read === '`dev/stabilize-record untriaged --run rc24-tier1`' && findingsOf({ reporter: 'x', findings: ['1 — a', '2 — b', '3 — c'] }) === 3)
  check('nothing awaits triage: no source', ledgerSource('x', { untriaged: { count: 0, why: [] } }).length === 0 && ledgerSource('x', {}).length === 0 && ledgerSource('x', { untriaged: [{ key: 'a document\'s list', why: 'ungraded' }] }).length === 0)
  check('no reporter can be named as the ledger is', !Object.keys(CHAINS).some((kind) => CHAINS[kind].some((stepList) => stepList.some((step) => reporterName(['x', step.as]) === LEDGER_SOURCE))) && !LEDGER_SOURCE.includes('-'))

  const facts = { stop: 'every-round', rounds: null, previous: '1.0.0-rc.24', 'previous-commit': 'e'.repeat(40), scope: 'delta' }
  check('the previous release is the record\'s', JSON.stringify(previousOf({ facts })) === JSON.stringify({ version: '1.0.0-rc.24', commit: 'e'.repeat(40) }))
  check('the evidence that goes back with close', JSON.stringify(evidenceOf({ clauses: [{ clause: 'no-lost-files', commit: 'c'.repeat(40), status: 'green', round: 2, behind: 1, retry: null }], items: [{ item: 'gate', kind: 'check', clause: 'no-lost-files', runs: 'every-candidate', selected: true, doors: 2, round: 2, attempt: 1, standing: 'green' }, { item: 'row-a', kind: 'review-row', clause: 'no-lost-files', runs: 'in-scope', selected: false, doors: 0, standing: 'not-selected' }, { item: 'other', kind: 'check', clause: 'no-regression', runs: 'every-candidate', selected: true, doors: 2, standing: 'green' }] })) === JSON.stringify([{ clause: 'no-lost-files', status: 'green', round: 2, commit: 'c'.repeat(40), behind: 1, items: [{ item: 'gate', round: 2, attempt: 1, standing: 'green', why: null }, { item: 'row-a', round: null, attempt: null, standing: 'not-selected', why: null }] }]) && evidenceOf({}).length === 0)

  const ctx = { run: 'rc24-tier1', round: 2, stage: 'fix', cycle: 3, attempt: 4, scratch: '/tmp/scratch-1' }
  const built = { candidate: { label: 'c2', sha: 'c'.repeat(40), binary: '/tmp/scratch-1/bin/c2/jigc', sha256: 'b'.repeat(64) }, previous: { version: '1.0.0-rc.24', binary: '/tmp/scratch-1/bin/previous/jigc', sha256: 'a'.repeat(64) }, image: { tag: 'jigc-trial:c2', verified: true } }
  const launch = launcher(ctx)
  const carries = (text, label) => text.split('\n').some((l) => l.startsWith(LABELS[label]))
  const unit = { item: 'row-3', kind: 'review-row', clause: 'no-lost-files', doors: 1, round: 2 }
  const entry = { key: 'f-1', door: 'jigc setup', clause: 'no-lost-files', grade: 'breaks', repro: 'r2/reports/test/row-3-driver.a1.md, block 2' }
  const fork = { key: 'f-1', kind: 'contested', door: 'jigc setup', clause: 'no-lost-files', repro: 'x', statement: 'the verifier' }
  const prompts = {
    preflight: preflightPrompt(ctx, launch, launch.add(['preflight']), { image: true, sha: 'c'.repeat(40), label: 'c2', branch: 'fix/rc24-tier1', checks: [{ item: 'ci' }], crossModel: false }),
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
  const kitFirst = (role) => 'YOUR FIRST CALL, before anything else: `dev/stabilize-record kit --run rc24-tier1 --role ' + role + '` — it prints what every agent of your role looks up: '
  check('every prompt of a role that drives, and the triage\'s, opens with the lookup kit\'s call, under its own role, and no other prompt names it', Object.keys(ROLES).every((role) => (ROLES[role].drives || role === 'triage' ? prompts[role].startsWith(kitFirst(role)) && prompts[role].split('`dev/stabilize-record kit ').length === 2 : !prompts[role].includes('stabilize-record kit'))) && prompts.verify.includes('the rig and how a binary is named to it') && !prompts.triage.includes('the rig'))
  check('a record step opens with its label', prompts.record.startsWith(LABELS.record) && rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text.startsWith(LABELS.record))
  check('the preflight builds nothing and holds no gate: it is told so, and handed neither', prompts.preflight.includes('NO BUILD AND NO GATE IS YOURS') && prompts.preflight.includes('the tip of `fix/rc24-tier1`') && !/git archive|dev\/gate|cargo|built from commit|path_check/.test(prompts.preflight) && prompts.preflight.includes('   - ci: its brief is the `brief` of the item\'s row — `dev/stabilize-record item --run rc24-tier1 --item ci`') && PREFLIGHT_SCHEMA.properties.candidate === undefined && PREFLIGHT_SCHEMA.properties.previous === undefined)
  check('a round\'s base is the earlier candidate, or the previous release\'s commit', prompts.scope.includes('base = ' + 'd'.repeat(40)) && scopePrompt(ctx, launcher(ctx), 'scope', { sha: 'c'.repeat(40), label: 'c1', base: null, earlier: false, scope: null, previous: previousOf({ facts }), fallback: 'everything' }).includes('base = ' + 'e'.repeat(40) + ' (the previous release, 1.0.0-rc.24'))
  check('a triage is told what the ledger\'s rows are', triagePrompt(ctx, launcher(ctx), 'triage-p1', handed, 1).includes('The source `' + LEDGER_SOURCE + '` is no reporter') && !prompts.triage.includes('is no reporter'))
  check('which run has a queue is what its state\'s digest says', hasQueue({ queue: { budget: 30, hand_on: 2, spent: 0, left: 30, length: 0, first: [], classes: [] } }) && hasQueue({ queue: {} }) && !hasQueue({ queue: null }) && !hasQueue({}) && !hasQueue({ queue: [] }))
  const tested = Object.assign({}, ctx, { stage: 'test' })
  const reds = redSources([{ item: 'regression-set', clause: 'working-product', verdict: '/tmp/scratch-1/hold/regression-set-c2-a4/verdict.json', output: '/tmp/scratch-1/hold/regression-set-c2-a4/output' }, { item: 'ci', clause: 'gate-green', report: testReport(tested, 'preflight-second') }])
  const kinds = [{ reporter: 'row-3-source', report: 'a.md', findings: ['F1 — x', 'F2 — y'] }, leftOpenSource('scope', 's.md', ['a door nobody listed'])].concat(reds, handed)
  const one = oneTriagePrompt(tested, launcher(tested), 'triage', kinds)
  check('the one triage opens with the lookup kit\'s call', one.startsWith(kitFirst('triage')))
  const under = (name) => one.split('\n').filter((line, i, all) => line.startsWith('- ' + name + ' — ') || (line.startsWith('   ') && all.slice(0, i).reverse().find((l) => l.startsWith('- ')).startsWith('- ' + name + ' — ')))
  check('a red result is a source under the key its row is filed with, with where its evidence lies', reds.length === 2 && reds[0].reporter === RED_PREFIX + 'regression-set' && reds[1].reporter === 'red-ci' && reds.every((r) => findingsOf(r) === 1 && isSlug(r.reporter)) && testReport(tested, 'preflight-second') === 'completions/artifacts/rc24-tier1/r2/reports/test/preflight-second.a4.md')
  check('the one triage\'s prompt names every source kind: a reporter, what was left open, a red held check, a red check an agent read, the rows nobody graded', ['row-3-source', 'scope', 'red-regression-set', 'red-ci', LEDGER_SOURCE].every((name) => under(name).length >= 1) && one.includes('Grade every finding below — 7 in all, from 5 source(s).'))
  check('every handed finding has a number under its source\'s name, from 1', under('row-3-source').join('|') === '- row-3-source — report: `a.md` — 2 finding(s):|   1. F1 — x|   2. F2 — y' && under('scope').join('|') === '- scope — report: `s.md` — 1 finding(s):|   1. left open 1 — a door nobody listed' && under('red-ci').length === 2 && under('red-ci')[1].startsWith('   1. the check `ci` is red · clause: gate-green'))
  check('a red held check is graded from the file the tool kept its verdict in, and a check an agent read from that preflight\'s report', under('red-regression-set')[0].includes('A RED RESULT') && under('red-regression-set')[0].includes('under the key `red-regression-set`') && under('red-regression-set')[0].includes('the tool\'s verdict of the held command, the file `/tmp/scratch-1/hold/regression-set-c2-a4/verdict.json`') && under('red-ci')[0].includes('the report of the preflight that read it red, `completions/artifacts/rc24-tier1/r2/reports/test/preflight-second.a4.md`') && !under('red-ci')[0].includes('verdict'))
  check('the rows nobody graded are handed over as the read, numbered in the order it prints them, and none by this script', under(LEDGER_SOURCE).length === 1 && under(LEDGER_SOURCE)[0].includes('the rows `dev/stabilize-record untriaged --run rc24-tier1` prints, NUMBERED 1 TO 2 IN THE ORDER THAT READ PRINTS THEM'))
  check('the call that takes the entries is spelled whole: the run, the round, the reporter, the attempt, the file, and one word a source', one.includes('to the file `/tmp/scratch-1/triage/r2.a4.json` WITH YOUR FILE TOOL') && one.includes('`dev/stabilize-record triage --run rc24-tier1 --round 2 --reporter triage --attempt 4 --scratch /tmp/scratch-1 --from /tmp/scratch-1/triage/r2.a4.json -- row-3-source=2 scope=1 red-regression-set=1 red-ci=1 ' + LEDGER_SOURCE + '=2`'))
  check('the one triage carries its report line, as a halt\'s road, and is told what its return is held to', carries(one, 'report') && !carries(one, 'binary') && one.includes('YOU TAKE IT ONLY WHERE YOU HALT') && one.includes('WHAT YOUR RETURN IS HELD TO') && one.includes('held to `completions/artifacts/rc24-tier1/r2/reports/test/triage.a4.md`') && !one.includes('pass ') && !one.includes('to_verify'))
  const back = { status: 'filed', report: testReport(tested, 'triage'), sha256: 'f'.repeat(64), handed: kinds.map((s) => ({ source: s.reporter, count: findingsOf(s) })) }
  const backWith = (more) => filedFault(tested, 'triage', kinds, Object.assign({}, back, more))
  check('a return of the one triage is taken where it is its call\'s line: the path of its report, a hash, what each source handed', backWith({}) === null && backWith({ handed: back.handed.slice().reverse() }) === null)
  check('a return that names another path, or none, is not taken', backWith({ report: testReport(tested, 'triage-p1') }).includes('a path that is not where its entries are rendered') && backWith({ report: undefined }).includes('no path that is not where') && backWith({ report: testReport(Object.assign({}, tested, { attempt: 5 }), 'triage') }).includes('attempt 4'))
  check('a return without a hash is not taken', [undefined, '', 'f'.repeat(63), 'F'.repeat(64)].every((hash) => backWith({ sha256: hash }).includes('no sha256')))
  const typedWith = (source, count) => back.handed.map((h) => (h.source === source ? { source, count } : h))
  check('a call typed with another count is named by its source, with both counts', backWith({ handed: typedWith('row-3-source', 1) }).includes('`row-3-source`: handed 2, and the call was typed with 1') && !backWith({ handed: typedWith('row-3-source', 1) }).includes('`scope`') && backWith({ handed: back.handed.filter((h) => h.source !== 'red-ci') }).includes('`red-ci`: handed 1, and the call named no count for it') && backWith({ handed: undefined }).includes('`' + LEDGER_SOURCE + '`: handed 2, and the call named no count for it') && backWith({ handed: back.handed.concat([{ source: 'nobody', count: 1 }]) }).includes('1 source(s) of the call that nobody handed') && backWith({ handed: typedWith('scope', '1') }).includes('`scope`: handed 1, and the call named no count for it'))
  check('the one triage returns no entry, and its schema requires nothing but its status', JSON.stringify(Object.keys(FILED_SCHEMA.properties)) === JSON.stringify(['status', 'halt', 'report', 'sha256', 'handed']) && JSON.stringify(FILED_SCHEMA.required) === JSON.stringify(['status']))
  const subjectSaid = 'docs(record): rc24 r1 - the record of the test stage'
  const two = ['reporter=attempt', 'reporter=triage']
  check('the canonical form of a record\'s words is the record script\'s — the run, the round, the attempt, the subject, then a line a word, each ended by a line feed — and hashes to what that script prints for its own example', wordsText('rc24', 1, 1, subjectSaid, two) === 'run=rc24\nround=1\nattempt=1\nsubject=' + subjectSaid + '\nreporter=attempt\nreporter=triage\n' && sha256(wordsText('rc24', 1, 1, subjectSaid, two)) === '38653f65999e4d356ac8bb701f36c22fd6cf84bd3dc4b2c5ef969153a0be4c95')
  check('words in another order, another attempt, another round and another subject are other bytes, and nothing is sorted', [wordsText('rc24', 1, 1, subjectSaid, two.slice().reverse()), wordsText('rc24', 1, 2, subjectSaid, two), wordsText('rc24', 2, 1, subjectSaid, two), wordsText('rc24', 1, 1, subjectSaid + '.', two), wordsText('rc24', 1, 1, subjectSaid, two.concat(['reporter=scope']))].every((text, i, all) => sha256(text) !== sha256(wordsText('rc24', 1, 1, subjectSaid, two)) && all.indexOf(text) === i) && wordsText('rc24', 1, 1, subjectSaid, []).endsWith('subject=' + subjectSaid + '\n'))
  const did = [{ item: 'area-a', status: 'green' }, { item: 'gate', status: 'green', verdict: '/tmp/scratch-1/hold/gate-0123456789ab/verdict.json', job: 'gate-0123456789ab' }, { item: 'regression-set', status: 'red', verdict: '/tmp/scratch-1/hold/regression-set-0123456789ab-a1/verdict.json', job: 'regression-set-0123456789ab-a1' }, { item: 'ci', status: 'red', evidence: 'a sentence of the preflight\'s', by: 'preflight-second', word: voidWord('unanswered', 'preflight-second') }, { item: 'arm-a', status: 'void', reason: 'no trial image: a sentence', word: voidWord('no-image', 'preflight-second') }, { item: 'held-b', status: 'void', reason: 'dead:held-b-0123456789ab-a1', word: heldVoid({ ends: 'dead', name: 'held-b-0123456789ab-a1' }) }]
  const said = recordWords({ reporters: ['attempt', 'scope', 'triage'], commit: 'c'.repeat(40), gate: 'gate-0123456789ab', did, facts: [['candidate', 'c'.repeat(40)], ['binary', 'b'.repeat(64)], ['cross-model', ['row-a', 'row-b']], ['cross-model-void', []]] })
  check('a record\'s words say the reporters, the commit, the gate\'s job, what each item did and the round\'s facts, in that order — a held check by its job whatever its verdict, a red check by the preflight that read it, a void by its word, a list one word an item and an empty list none', JSON.stringify(said) === JSON.stringify(['reporter=attempt', 'reporter=scope', 'reporter=triage', 'commit=' + 'c'.repeat(40), 'gate=gate-0123456789ab', 'item=area-a:green', 'item=gate:held:gate-0123456789ab', 'item=regression-set:held:regression-set-0123456789ab-a1', 'item=ci:red:preflight-second', 'item=arm-a:void:no-image:preflight-second', 'item=held-b:void:dead:held-b-0123456789ab-a1', 'fact=candidate:' + 'c'.repeat(40), 'fact=binary:' + 'b'.repeat(64), 'fact=cross-model:row-a', 'fact=cross-model:row-b']))
  check('a re-run\'s words name the commit, and no gate and no fact of the round', JSON.stringify(recordWords({ reporters: ['attempt'], commit: 'c'.repeat(40), gate: null, did: [did[2]], facts: [] })) === JSON.stringify(['reporter=attempt', 'commit=' + 'c'.repeat(40), 'item=regression-set:held:regression-set-0123456789ab-a1']))
  check('no sentence is a word: a void says why in one word of the shape the record script takes, and no word of a record holds a space', Object.keys(VOID_WORDS).length === 12 && Object.keys(VOID_WORDS).every((word) => isSlug(word) && HELD_VOID_RE.test(voidWord(word)) && HELD_VOID_RE.test(voidWord(word, 'row-3-reconciler')) && VOID_WORDS[word].split(' ').length >= 6) && voidWord('silent', 'area-a-review') === 'silent:area-a-review' && voidWord('off-tree') === 'off-tree' && said.every((word) => /^[A-Za-z0-9_.:=-]+$/.test(word)) && !said.join(' ').includes('sentence'))
  const composing = composePrompt(tested, 'fix/rc24-tier1', subjectSaid, said)
  check('the composing step is ONE command of the tool: the branch, the run\'s directory, the round, the attempt, the subject as one quoted word, the sha256 of the words\' canonical form, and the words after `--`', composing.includes('\n1. `dev/stabilize-step compose --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --round 2 --attempt 4 --subject \'' + subjectSaid + '\' --sha256 ' + sha256(wordsText('rc24-tier1', 2, 4, subjectSaid, said)) + ' --scratch /tmp/scratch-1 --digest /tmp/scratch-1 -- ' + said.join(' ') + '`\n') && composing.startsWith('GIT STEP — ') && !composing.includes(LABELS.record) && !composing.includes(PAYLOAD_ENDS))
  check('a composing step that is asked for again is told that the tool refused the words, and to type the command as it stands', wordsAgain(1).startsWith('AGAIN (1): ') && wordsAgain(1).includes('typed as it stands') && wordsAgain(1).includes('nothing was written') && !wordsAgain(1).includes('`'))
  check('of the words a composed record is refused with, `words` is asked for again — once — and every other halts, each saying what it means', WORDS_AGAIN === 1 && Object.keys(COMPOSE_REFUSALS).filter((word) => COMPOSE_REFUSALS[word].again).join() === 'words' && ['words', 'check', 'pending', 'no-change', 'hygiene'].every((word) => COMPOSE_REFUSALS[word] && COMPOSE_REFUSALS[word].says.split(' ').length >= 10) && COMPOSE_REFUSALS['no-change'].then.includes('NO BATCH IS LEFT') && COMPOSE_REFUSALS.pending.then === 'pending')
  const filer = 'verify-f-1'
  const files = verdictPrompt(tested, launcher(tested), filer, 'f-1', built, 2)
  const [kitCall, rowCall, filingCall] = ['`dev/stabilize-record kit --run rc24-tier1 --role verify`', '`dev/stabilize-record row --run rc24-tier1 --key f-1`', '`dev/stabilize-record verdict --run rc24-tier1 --round 2 --reporter verify-f-1 --attempt 4 --scratch /tmp/scratch-1 --from /tmp/scratch-1/verdict/r2/verify-f-1.a4.json`']
  check('a wave is as many verifiers as the digest prints keys of the queue, and a filing says one of three verdicts', WAVE === 8 && FILED_WORDS.join() === 'confirmed,refuted,undrivable')
  check('a verdict is filed where the record script files one, and written first to a file under the scratch root', verdictFile(tested, filer) === 'completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json' && filingFile(tested, filer) === '/tmp/scratch-1/verdict/r2/verify-f-1.a4.json' && verdictFile(Object.assign({}, tested, { attempt: 5 }), filer) !== verdictFile(tested, filer))
  check('a verifier that files is handed the kit\'s call, the read of its row and the call that files its verdict, each spelled whole and in that order — the kit\'s its first line', files.startsWith(kitFirst('verify')) && [kitCall, rowCall, filingCall].every((call) => files.includes(call)) && files.indexOf(kitCall) < files.indexOf(rowCall) && files.indexOf(rowCall) < files.indexOf(filingCall) && files.includes('to the file `/tmp/scratch-1/verdict/r2/verify-f-1.a4.json` WITH YOUR FILE TOOL'))
  check('it is handed its key, and nothing of its row: no door, no clause, no grade and no repro is in the prompt', files.includes('ONE finding: the row `f-1` of the run\'s ledger') && files.includes('This prompt carries none of it.') && !files.includes(entry.door) && !files.includes(entry.clause) && !files.includes(entry.repro) && !files.includes('triage\'s grade'))
  check('it carries its report line and its binary line with both binaries, and the two hashes are members of its file and no word of its call', carries(files, 'report') && carries(files, 'binary') && files.includes('; previous release `/tmp/scratch-1/bin/previous/jigc`') && files.includes('THE TWO HASHES ARE MEMBERS OF THAT FILE, and no word of the call') && !filingCall.includes('b'.repeat(64)) && !filingCall.includes('a'.repeat(64)))
  check('it is told how many of what it hands on enter the queue — the opening\'s number — or that none does where the opening names none', files.includes('AT MOST 2 ENTER THE QUEUE') && files.includes('the first 2 that carry the evidence of their kind') && !files.includes('ENTERS NO QUEUE') && verdictPrompt(tested, launcher(tested), filer, 'f-1', built, null).includes('WHAT YOU HAND ON ENTERS NO QUEUE') && !verdictPrompt(tested, launcher(tested), filer, 'f-1', built, null).includes('AT MOST') && verdictPrompt(tested, launcher(tested), filer, 'f-1', built, 0).includes('AT MOST 0 ENTER THE QUEUE'))
  check('it is told what its return is held to, and that what counts is what is filed', files.includes('WHAT YOUR RETURN IS HELD TO') && files.includes('held to `completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json`') && files.includes('held to confirmed, refuted, undrivable') && files.includes('a finding the queue prints again has no verdict') && !files.includes('left_open') && !files.includes('pass '))
  const filedBack = { status: 'filed', filing: verdictFile(tested, filer), word: 'confirmed', contested: false, asserted_sha256: 'b'.repeat(64) }
  const heldWith = (more) => verdictFault(tested, filer, Object.assign({}, filedBack, more), 'b'.repeat(64))
  check('a return of a verifier that files is held where it is its call\'s line: the path a filing of its name has, a verdict a filing says, `contested` as a truth value, and the hash it was handed', heldWith({}) === null && FILED_WORDS.every((word) => heldWith({ word }) === null) && heldWith({ contested: true }) === null && heldWith({ report: 'anything', basis: 'anything' }) === null)
  check('a return that names another path, or none, is not held, and says which path a filing has', heldWith({ filing: verdictFile(tested, 'verify-f-2') }).includes('a path that is not where a verdict of `verify-f-1`, attempt 4, is filed — `completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json`') && heldWith({ filing: undefined }).startsWith('returned no path that is not where') && isText(heldWith({ filing: verdictFile(Object.assign({}, tested, { attempt: 5 }), filer) })) && isText(heldWith({ filing: testReport(tested, filer) })))
  check('a return whose word is no verdict a filing says is not held, and what came back is not printed', [undefined, '', 'verified', 'Confirmed', 'regression', true].every((word) => isText(heldWith({ word })) && heldWith({ word }).includes('no verdict a filing says') && !heldWith({ word }).includes('`' + word + '`')) && heldWith({ word: undefined }).startsWith('returned no word'))
  check('a return that does not say `contested` as true or false is not held', [undefined, null, 'true', 'a basis', 1].every((contested) => isText(heldWith({ contested })) && heldWith({ contested }).includes('`contested` as true or false')))
  check('a return without the hash it asserted, or with another, is not held', heldWith({ asserted_sha256: undefined }).startsWith('returned no `asserted_sha256`') && heldWith({ asserted_sha256: 'a'.repeat(64) }).includes('the `asserted_sha256` ' + 'a'.repeat(64) + ', which is not the candidate\'s binary (' + 'b'.repeat(64) + ')') && heldWith({ asserted_sha256: 'a sentence' }).includes('an `asserted_sha256` that is no sha256') && !heldWith({ asserted_sha256: 'a sentence' }).includes('a sentence'))
  check('a verifier that files returns no verdict, and its schema requires nothing but its status', JSON.stringify(Object.keys(VERDICT_SCHEMA.properties)) === JSON.stringify(['status', 'halt', 'filing', 'word', 'contested', 'asserted_sha256', 'report']) && JSON.stringify(VERDICT_SCHEMA.required) === JSON.stringify(['status']) && VERDICT_SCHEMA.properties.word.enum === undefined && !('verdict' in VERDICT_SCHEMA.properties) && !('left_open' in VERDICT_SCHEMA.properties))
  const withQueue = { queue: { budget: 30, hand_on: 2, spent: 1, left: 29, length: 1, first: ['f-1'], classes: [] } }
  check('an invocation continues a stage where the run has a queue and the position says `triage`, `verify` or `filed` — worked in that order — and nowhere else', JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, verify: true })) === '["verify"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, verify: true, triage: true })) === '["triage","verify"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, triage: true })) === '["triage"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, filed: true })) === '["filed"]' && JSON.stringify(continuing(withQueue, { round: 2, attempt: 2, filed: true, verify: true, triage: true })) === '["triage","verify","filed"]' && continuing({ queue: null }, { round: 2, attempt: 2, filed: true }).length === 0 && continuing(withQueue, { round: 3, attempt: 1 }).length === 0 && continuing(withQueue, { round: 2, attempt: 2, rerun: { clause: 'c' } }).length === 0 && continuing({ queue: null }, { round: 2, attempt: 2, triage: true }).length === 0 && continuing({}, { round: 2, attempt: 2, verify: true }).length === 0)
  check('an invocation that continues takes no scope, clause or cross-model pass, and is stopped after nothing but its first read', continueFault(base) === null && continueFault(Object.assign({}, base, { stopAfter: 'state' })) === null && [['scope', 'everything'], ['clause', 'no-lost-files'], ['crossModel', ['row-3']]].every(([name, value]) => isText(continueFault(Object.assign({}, base, { [name]: value }))) && continueFault(Object.assign({}, base, { [name]: value })).endsWith('it was passed ' + name)) && ['preflight', 'instruments', 'triage'].every((stop) => continueFault(Object.assign({}, base, { stopAfter: stop })).endsWith('it was passed stopAfter: \'' + stop + '\'')))
  check('two verifier calls of one wave that return nothing make it the last', SILENT_LAST === 2)
  const limitAt = (step, record) => limitSaid(tested, { wave: 3, silent: 2 }, 7, [verdictFile(tested, filer)], [verdictFile(tested, 'verify-f-2')], step, record || null)
  const limit = limitAt('compose')
  check('a halt that may be the account\'s limit says what it saw and never that it is one, and names the filed verdicts: the state\'s count, each path, what nothing held, and where they lie', JSON.stringify(limit.more) === JSON.stringify({ limit: { possible: true, wave: 3, silent: 2, step: 'compose' }, filed: { counted: 7, read: 'after-the-wave', commit: 'none', record: null, files: ['completions/artifacts/rc24-tier1/r2/verdicts/verify-f-1.a4.json'], unconfirmed: ['completions/artifacts/rc24-tier1/r2/verdicts/verify-f-2.a4.json'], directory: 'completions/artifacts/rc24-tier1/r2/verdicts' } }) && limit.says.includes('THIS MAY BE THE ACCOUNT\'S USAGE LIMIT') && limit.says.includes('2 verifier call(s) of wave 3 returned nothing') && limit.says.includes('The state counts 7 verdict(s) in round 2') && limit.says.includes('held by nothing: completions/artifacts/rc24-tier1/r2/verdicts/verify-f-2.a4.json') && limitSaid(tested, { wave: 1, silent: 8 }, 0, [], [], 'queue', null).says.includes('no verifier of this invocation filed one') && !limitSaid(tested, { wave: 1, silent: 8 }, 0, [], [], 'queue', null).says.includes('held by nothing'))
  check('the limit\'s halt can be of seven steps, each with whether the state was read after the wave and what became of the record\'s commit', JSON.stringify(Object.keys(LIMIT_STEPS)) === JSON.stringify(['report-check', 'queue', 'compose', 'gate', 'commit', 'push', 'read-back']) && JSON.stringify(Object.keys(LIMIT_STEPS).map((step) => [LIMIT_STEPS[step].read, LIMIT_STEPS[step].commit])) === JSON.stringify([['before', 'none'], ['before', 'none'], ['after', 'none'], ['after', 'none'], ['after', 'unknown'], ['after', 'made'], ['after', 'pushed']]) && Object.keys(LIMIT_STEPS).every((step) => isText(LIMIT_STEPS[step].step) && limitAt(step, 'c'.repeat(40)).says.includes('and then ' + LIMIT_STEPS[step].step + ' returned nothing too.')))
  const stateNowCall = '`dev/stabilize-record state --run rc24-tier1` prints the state as it stands now'
  check('before a read after the wave the halt says that the named document predates the wave, and names the one plain call that prints the state now — and after one it does not', ['report-check', 'queue'].every((step) => limitAt(step).says.includes('THE STATE WAS LAST READ BEFORE THAT WAVE: it counted 7 verdict(s) in round 2 then') && limitAt(step).says.includes('it lists nothing wave 3 filed') && limitAt(step).says.includes(stateNowCall) && !limitAt(step).says.includes('lists every filed verdict') && limitAt(step).more.filed.read === 'before-the-wave') && ['compose', 'gate', 'commit', 'push', 'read-back'].every((step) => !limitAt(step, 'c'.repeat(40)).says.includes('BEFORE THAT WAVE') && !limitAt(step, 'c'.repeat(40)).says.includes(stateNowCall) && limitAt(step, 'c'.repeat(40)).says.includes('the state document this return names lists every filed verdict with whether it counts (`filed`)') && limitAt(step, 'c'.repeat(40)).more.filed.read === 'after-the-wave'))
  check('the halt says *in no commit yet* only where no commit can hold what was filed; where the commit step returned nothing it says that it cannot tell; and from the push on it names the commit', ['report-check', 'queue', 'compose', 'gate'].every((step) => limitAt(step).says.includes('WHAT WAS FILED IS NOT LOST, AND IS IN NO COMMIT YET: ') && limitAt(step).more.filed.commit === 'none' && limitAt(step).more.filed.record === null) && !limitAt('commit').says.includes('IN NO COMMIT YET') && !limitAt('commit').says.includes('IS IN A COMMIT') && limitAt('commit').says.includes('WHETHER A COMMIT HOLDS IT THIS SCRIPT CANNOT SAY') && limitAt('commit').more.filed.commit === 'unknown' && limitAt('commit', 'c'.repeat(40)).more.filed.record === null && ['push', 'read-back'].every((step) => !limitAt(step, 'c'.repeat(40)).says.includes('IN NO COMMIT YET') && limitAt(step, 'c'.repeat(40)).says.includes('WHAT WAS FILED IS NOT LOST, AND IS IN A COMMIT: the record this invocation made (' + 'c'.repeat(40) + ') took it') && limitAt(step, 'c'.repeat(40)).more.filed.record === 'c'.repeat(40)) && limitAt('push', 'c'.repeat(40)).says.includes('a local commit until the push that is owed is made') && !limitAt('push', 'c'.repeat(40)).says.includes('and is pushed') && limitAt('read-back', 'c'.repeat(40)).says.includes(', and is pushed.') && ['commit', 'push', 'read-back'].every((step) => limitAt(step, 'c'.repeat(40)).says.includes('as it was read before that record\'s commit')) && ['compose', 'gate'].every((step) => !limitAt(step).says.includes('as it was read before that record\'s commit')))
  check('the halt says that the stage is continued only at the two steps before its record: from the composing step on, what the next invocation does first is the record step\'s, the push\'s and the read-back\'s own sentence', ['report-check', 'queue'].every((step) => limitAt(step).says.includes('it CONTINUES') && limitAt(step).says.endsWith('whichever record the run makes next takes what is filed.')) && ['compose', 'gate', 'commit', 'push', 'read-back'].every((step) => !limitAt(step, 'c'.repeat(40)).says.includes('it CONTINUES') && limitAt(step, 'c'.repeat(40)).says.endsWith('what that invocation does first is said above, with this halt.')))
  const aRecordsGate = { sequence: true }
  const byItsCommit = { stands: true }
  const oneName = {}
  const answered = (ends, fresh) => ({ ends, fresh: fresh === true })
  const overOf = (job, ends, tries, fresh) => passedOver(job, answered(ends, fresh), tries)
  check('a job that answers a start as ENDED is never the answer for a record\'s gate, whatever the tries; for what is named by its commit only green is', ['green', 'red', 'void', 'dead'].every((ends) => [1, 2, 3].every((tries) => overOf(aRecordsGate, ends, tries) === 'ended')) && ['red', 'void', 'dead'].every((ends) => [1, 2, 3].every((tries) => overOf(byItsCommit, ends, tries) === 'ended')) && [1, 2, 3].every((tries) => overOf(byItsCommit, 'green', tries) === null))
  check('a job that still runs is passed over for a record\'s gate on the start\'s first try, is the dead try\'s own on a retry, and is taken over for what is named by its commit', overOf(aRecordsGate, 'running', 1) === 'running' && overOf(aRecordsGate, 'running', 2) === null && overOf(aRecordsGate, 'running', 3) === null && [1, 2].every((tries) => overOf(byItsCommit, 'running', tries) === null))
  check('the job this call started, and a start that was not read, are never passed over — and a hold of one name passes nothing over', [aRecordsGate, byItsCommit, oneName].every((job) => [1, 2].every((tries) => overOf(job, 'running', tries, true) === null && overOf(job, 'unread', tries) === null)) && ['running', 'green', 'red', 'void', 'dead'].every((ends) => [1, 2].every((tries) => overOf(oneName, ends, tries) === null)))
  check('a name without a record is said to be what the tool\'s answer says of it — never that its command\'s process is gone', heldEnded({ ends: 'dead', why: 'no-record' }).includes('MAY STILL RUN') && !heldEnded({ ends: 'dead', why: 'no-record' }).includes('process is gone') && heldEnded({ ends: 'dead', why: 'no-record' }).includes('`supervised`') && heldEnded({ ends: 'dead', why: 'no-exit' }) === HELD_ENDS.dead && heldEnded({ ends: 'dead', why: 'killed' }) === HELD_ENDS.dead && heldEnded({ ends: 'red' }) === HELD_ENDS.red && heldFault('the candidate\'s binary', { name: 'build-c1', ends: 'dead', why: 'no-record' }).includes('MAY STILL RUN') && JSON.stringify(Object.keys(DEAD_WHY)) === '["no-record"]')
  check('a step that returned nothing is told from one that answered: only the first is what the limit\'s halt is said of', heldOf(null, '/tmp/scratch-1').silent === true && heldOf({ status: 'halted', refused: 'taken' }, '/tmp/scratch-1').silent === undefined && heldOf({ status: 'running', name: 'g', kind: 'gate' }, '/tmp/scratch-1').silent === undefined)
  const noNameSaid = recordThen('rc24-tier1', 'fix/rc24-tier1', 'no-name')
  check('where no name is left for a pending batch\'s gate the halt says so, never that the next invocation gates it under the next name, and names the exit that works', noNameSaid.includes('NO NAME IS LEFT TO HOLD ITS GATE UNDER') && noNameSaid.includes('one of ' + HOLD_NAMES + ' names') && noNameSaid.includes('INVOKING THE STAGE AGAIN CHANGES NOTHING') && noNameSaid.includes('`dev/stabilize-step discard --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1`') && !noNameSaid.includes('under the next name') && !noNameSaid.includes('once more') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'no-gate').includes('under the next name') && heldFault('the record\'s gate', { name: 'record-pending-9', ends: 'unread', exhausted: true, why: 'none is left' }) === 'the record\'s gate was not held: none is left.' && pendingRecord('/tmp/scratch-1/record/pending', { calls: 2, checks: 1 }).pending === true && pendingRecord('/tmp/scratch-1/record/pending', { calls: 2, checks: 1 }).hold === 'record-pending')
  const vetText = checkReportsPrompt(base, null)
  check('the vet before a record of rulings is the report check without a stage: one plain call that names the run and the scratch root, and no round, stage, attempt or reporter', vetText.includes('1. `dev/stabilize-step check-reports --run rc24-tier1 --scratch /tmp/scratch-1 --digest /tmp/scratch-1`') && !vetText.includes('--round') && !vetText.includes('--stage') && !vetText.includes('--attempt') && !vetText.includes(' -- '))
  const finishing = preflightPrompt(ctx, launcher(ctx), 'preflight', { image: false, sha: 'c'.repeat(40), label: 'c2', branch: 'fix/rc24-tier1', checks: [], crossModel: false, tested: true })
  check('a finishing preflight is told that the candidate is the commit the round tested, and not HEAD', finishing.includes('the commit round 2 tested, an ancestor of `fix/rc24-tier1`') && finishing.includes('THE CANDIDATE IS NOT `HEAD` HERE') && finishing.includes('commit ' + 'c'.repeat(40) + ' must be an ancestor of `HEAD`') && !finishing.includes('`git ') && !prompts.preflight.includes('NOT `HEAD`'))
  check('a fork is recorded with the verdict it rides on, and with no other entry', JSON.stringify(triageRecord(ctx, [{ key: 'f-1', new: false, grade: 'breaks', verdict: 'confirmed', regression: false, fork: { kind: 'contested', case: 'robust-now', drive: 'holds' } }, { key: 'f-2', new: false, grade: 'breaks', fork: { kind: 'contested', case: 'robust-now', drive: 'holds' } }]).triage) === JSON.stringify([{ key: 'f-1', grade: 'breaks', verdict: 'confirmed', regression: false, fork: { kind: 'contested', case: 'robust-now', drive: 'holds' } }, { key: 'f-2', grade: 'breaks' }]))
  check('a report line gives the flags', launch.line('scope') === reportLine(ctx, 'scope') && reportLine(ctx, 'scope').startsWith('REPORT: --run rc24-tier1 --round 2 --stage fix --cycle 3 --reporter scope --attempt 4 ') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').startsWith('REPORT: --run rc24-tier1 --round 2 --stage test --reporter x --attempt 4 ') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').includes('write your report ONCE, when what it reports is done, to the file `/tmp/scratch-1/report/r2/test/x.a4.md` WITH YOUR FILE TOOL; a correction is made in that file WITH YOUR EDIT TOOL, never by writing it whole again;') && reportLine(Object.assign({}, ctx, { stage: 'test' }), 'x').includes('`dev/stabilize-record report --run rc24-tier1 --round 2 --stage test --reporter x --attempt 4 --scratch /tmp/scratch-1 --from /tmp/scratch-1/report/r2/test/x.a4.md`') && reportFile(ctx, 'scope') === '/tmp/scratch-1/report/r2/fix.c3/scope.a4.md')
  check('a binary line gives the path and the hash, and the one plain call that asserts it', binaryLine(built, true, '/tmp/scratch-1').includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc` prints ONE line of JSON whose `content_sha256` must be that sha256') && binaryLine(built, true, '/tmp/scratch-1').endsWith(' Return the sha256 you asserted for the candidate as `asserted_sha256`.') && !binaryLine(built, true).includes('stabilize-step') && prompts.review.includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc`') && prompts.verify.includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc`') && binaryLine(built, true).startsWith('BINARY: candidate `/tmp/scratch-1/bin/c2/jigc` sha256 ' + 'b'.repeat(64)) && binaryLine(built, true).includes('jigc-trial:c2') && !binaryLine(built, false).includes('jigc-trial:c2'))
  check('every chain is staffed from the roles', Object.keys(CHAINS).concat(['fix-diff']).every((kind) => (CHAINS[kind] || FIX_AUDIT).every((stepList) => stepList.every((s) => ROLES[s.role] && isSlug(s.as) && !s.as.includes('-') && (s.hands || []).every((h) => (CHAINS[kind] || FIX_AUDIT).some((earlier) => earlier.some((e) => e.as === h)))))))

  const names = new RegExp(crossModelTool(), 'i')
  const asOf = (chain) => chain.reduce((all, stepList) => all.concat(stepList.map((s) => s.as + (s.hands ? '<' + s.hands.join('+') : ''))), []).join(' ')
  for (const kind of Object.keys(CHAINS)) {
    for (const given of [undefined, null, false, 'true', 1, [], ['row-3'], 'all']) check('no cross-model step in ' + kind + ' under ' + JSON.stringify(given), chainOf(kind, given).every((stepList) => stepList.every((s) => !s.crossModel && ROLES[s.role] !== ROLES.crossModel && !(s.hands || []).includes('crossmodel'))))
  }
  check('the review row without the opt-in, and with it', asOf(chainOf('review-row', false)) === 'source driver reconciler<source+driver' && asOf(chainOf('review-row', true)) === 'source driver crossmodel reconciler<source+driver+crossmodel' && chainOf('no-such-kind', true).length === 0)
  const steps = { begin: beginPrompt(Object.assign({}, ctx, { stage: 'test' }), 'c'.repeat(40)), 'git-state': gitStatePrompt(base), state: statePrompt(base, 't'), 'find-round': findRoundPrompt(fix, 1), 'open-round': openRoundPrompt(fix, 1, 'fix/rc24-tier1-r1'), push: pushPrompt(fix, 'fix/rc24-tier1-r1'), land: landPrompt(fix, 1, 'fix/rc24-tier1-r1'), 'round-commits': roundCommitsPrompt(fix, 1, 'fix/rc24-tier1-r1'), carry: carryPrompt(fix, 1, ['a'.repeat(40)], null), 'sync-main': syncMainPrompt('rc24-tier1', '/tmp/scratch-1'), 'check-reports': checkReportsPrompt(ctx, ['x']), record: recordCommitPrompt(fix, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }) }
  const recut = carryPrompt(fix, 1, ['a'.repeat(40)], 'fix/rc24-tier1-r1-part1')
  for (const act of Object.keys(steps)) {
    const numbered = steps[act].split('\n')
    check('the `' + act + '` step is one command of the tool, and its line relayed', numbered.length === 3 && numbered[0].startsWith('GIT STEP — ') && numbered[0].endsWith(STEP_RULES) && numbered[1].startsWith('1. `' + STEP_TOOL + ' ' + act + ' ') && numbered[1].endsWith('`') && numbered[2].startsWith('2. Report status = ran and line = ') && !steps[act].includes('`git '))
    check('the `' + act + '` step asks for its digest, once, under the scratch root — and before any list of names', numbered[1].split(' --digest /tmp/scratch-1').length === 2 && (!numbered[1].includes(' -- ') || numbered[1].indexOf(' --digest ') < numbered[1].indexOf(' -- ')))
  }
  check('the names a step hands the tool are the harness\'s', steps['git-state'].includes('`' + STEP_TOOL + ' git-state --stage test --loop fix/rc24-tier1 --rounds fix/rc24-tier1-r --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --digest /tmp/scratch-1`') && steps.land.includes(' land --loop fix/rc24-tier1 --branch fix/rc24-tier1-r1 --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --log DECISIONS.md --log implementation/project-history.md --digest /tmp/scratch-1`') && steps.carry.endsWith(STEP_RULES + '\n1. `' + STEP_TOOL + ' carry --loop fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --product Cargo.toml --product Cargo.lock --product crates --digest /tmp/scratch-1 -- ' + 'a'.repeat(40) + '`\n' + steps.carry.split('\n')[2]) && steps['sync-main'].includes(' sync-main --loop fix/rc24-tier1 --log DECISIONS.md --log implementation/project-history.md --digest /tmp/scratch-1`') && steps.state.includes(' state --run rc24-tier1 --scratch /tmp/scratch-1 --tag t --digest /tmp/scratch-1`') && steps['find-round'].includes(' find-round --prefix fix/rc24-tier1-r1 --digest /tmp/scratch-1`') && steps.push.includes(' push --branch fix/rc24-tier1-r1 --digest /tmp/scratch-1`') && steps.record.includes(' record --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --gate /tmp/scratch-1/record/x/gate.txt --calls 3 --checks 2 --scratch /tmp/scratch-1 --digest /tmp/scratch-1`'))
  check('an attempt is begun under the script\'s own reporter, on the commit it runs on', steps.begin.includes('`' + STEP_TOOL + ' begin --run rc24-tier1 --round 2 --stage test --attempt 4 --reporter ' + ATTEMPT + ' --commit ' + 'c'.repeat(40) + ' --scratch /tmp/scratch-1 --digest /tmp/scratch-1`') && beginPrompt(ctx, 'c'.repeat(40)).includes(' begin --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 --reporter '))
  check('no agent can be named as the attempt\'s marker is', isSlug(ATTEMPT) && !ATTEMPT.includes('-') && ATTEMPT !== LEDGER_SOURCE && !Object.keys(CHAINS).some((kind) => CHAINS[kind].some((stepList) => stepList.some((step) => reporterName(['x', step.as]) === ATTEMPT))) && !['preflight', 'scope'].includes(ATTEMPT))
  check('the re-cut of a part is still the list of commands it was', recut.startsWith('GIT STEP — re-cut a part of round 1 of `rc24-tier1` as a branch of its own, `fix/rc24-tier1-r1-part1`') && recut.includes(CARRY_RULES) && recut.includes('`git cherry-pick -x <sha>`') && recut.includes('`git push origin fix/rc24-tier1-r1-part1`') && !recut.includes(STEP_TOOL) && recut.split('\n').length === 8)
  const gitPrompts = Object.keys(steps).map((act) => steps[act]).concat([recut])
  check('no prompt names the tool without the opt-in', Object.keys(prompts).filter((role) => names.test(prompts[role])).join(' ') === 'crossModel' && !names.test(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', taken[taken.length - 1].rulings, {}).text) && !names.test(gitPrompts.join('\n')))
  const asserts = (crossModel) => preflightPrompt(ctx, launcher(ctx), 'preflight', { image: false, sha: 'c'.repeat(40), label: 'c2', branch: 'b', checks: [], crossModel })
  check('the preflight is asked about the tool only when an item is named', names.test(asserts(true)) && asserts(true).includes('`' + CROSS_CHECK + '`') && !names.test(asserts(false)) && !names.test(asserts(undefined)) && !asserts(false).includes(CROSS_CHECK))
  const dropping = launcher(ctx)
  dropping.add(['row-3', 'source'])
  dropping.drop(dropping.add(['row-3', 'crossmodel']))
  check('a cross-model reporter that left no report is not held to one', dropping.names.join(' ') === 'row-3-source')
  check('a round with no scope of its own takes the run\'s default, as its record names it', scopeText(undefined, 'delta').endsWith('the derived delta') && scopeText(null, 'everything').includes('everything') && scopeText('delta', 'everything') === 'the derived delta')
  check('a driving chain role is handed the binary', unitPrompt(ctx, launcher(ctx), 'x', CHAINS['trial-arm'][1][0], unit, built, []).includes('jigc-trial:c2'))

  const checked = (check, aside) => reportsRead({ status: 'checked', check, aside: aside || [], file: 'lines/check-reports.0123456789abcdef.json', file_sha256: 'f'.repeat(64) })
  check('a report check that holds names nothing missing', JSON.stringify(checked({ ok: true, missing: [], extra: 0 })) === '{"missing":[],"aside":[]}')
  check('a launched reporter with no report is named, and is no fault', JSON.stringify(checked({ ok: false, missing: ['verify-p1-f-1'], extra: 0 })) === '{"missing":["verify-p1-f-1"],"aside":[]}')
  check('a file nobody launched is a fault that says how many, and names the file that names them — never the names', isText(checked({ ok: false, missing: [], extra: 1 }).fault) && checked({ ok: false, missing: ['a'], extra: 2 }).fault.includes('2 file(s)') && checked({ ok: false, missing: ['a'], extra: 1 }).missing === undefined && checked({ ok: false, missing: [], extra: 1 }).file.file === 'lines/check-reports.0123456789abcdef.json')
  check('a report check that cannot be read is a fault, and transient only when nothing came back', reportsRead(null).transient === true && isText(reportsRead(null).fault) && reportsRead({ status: 'halted' }).transient === false && isText(reportsRead({ status: 'checked', check: { ok: true }, aside: [] }).fault) && isText(reportsRead({ status: 'checked', check: 'ok', aside: [] }).fault) && isText(reportsRead({ status: 'checked', check: { ok: true, missing: [], extra: [] }, aside: [] }).fault) && isText(reportsRead({ status: 'checked', check: { ok: true, missing: [], extra: 0 } }).fault) && reportsRead({ status: 'halted', refused: 'did-not-run' }).fault.includes('`did-not-run`'))
  const moved = [{ path: 'completions/artifacts/rc24-tier1/r2/reports/test/a-source.a1.md', why: 'hygiene' }, { path: 'completions/artifacts/rc24-tier1/r2/scope.md', why: 'host-path' }]
  check('a report that was set aside is its reporter\'s, by the file\'s name, with the word of what refused it', JSON.stringify(checked({ ok: false, missing: ['a-source'], extra: 0 }, moved)) === JSON.stringify({ missing: ['a-source'], aside: moved }) && setAside(moved, 'a-source') === 'hygiene' && setAside(moved, 'a') === null && setAside(moved, 'scope') === null && setAside(undefined, 'a-source') === null)
  check('how a reporter that did not report ended', endingOf(null) === 'returned nothing' && endingOf({ status: 'halted', halt: { root_cause: 'no rig' } }) === 'halted (no rig)' && endingOf({ status: 'halted' }) === 'halted')
  const twoUnits = () => [
    { item: 'a', status: 'green', reason: null, crossModel: 'ran', reporters: [{ as: 'source', name: 'a-source', crossModel: false, result: { status: 'reported', findings: [1] }, report: 'a.md' }, { as: 'crossmodel', name: 'a-crossmodel', crossModel: true, result: { status: 'reported', findings: [2] }, report: null }] },
    { item: 'b', status: 'void', reason: 'its `drive` step returned nothing', crossModel: null, reporters: [{ as: 'drive', name: 'b-drive', crossModel: false, result: null, report: null }] },
  ]
  const lostCross = twoUnits()
  unreported(lostCross, ['a-crossmodel', 'b-drive'])
  check('a cross-model pass with no report is void for that pass, and never voids its item', lostCross[0].status === 'green' && lostCross[0].crossModel === 'void' && lostCross[0].reporters[1].result === null && lostCross[0].reporters[0].result.findings.length === 1 && lostCross[1].reason === 'its `drive` step returned nothing')
  const lostSource = twoUnits()
  unreported(lostSource, ['a-source'])
  check('a step of an item\'s own chain with no report voids the item, and its result counts for nothing', lostSource[0].status === 'void' && lostSource[0].reason === 'its `source` step returned and left no report' && lostSource[0].reporters[0].result === null && lostSource[0].crossModel === 'ran')
  const asideSource = twoUnits()
  unreported(asideSource, ['a-source'], moved)
  check('a step whose report was set aside voids its item, and the reason says that the file was no report', asideSource[0].status === 'void' && asideSource[0].reason === 'its `source` step left a file at its report\'s path that is no report (`hygiene`): it was set aside, and the step left none' && asideSource[0].reporters[0].result === null)
  const kept = twoUnits()
  unreported(kept, [])
  check('a reporter whose report is there keeps what it returned', JSON.stringify(kept) === JSON.stringify(twoUnits()))
  check('the roles with no definition are handed the paragraph every definition carries', prompts.proposal.endsWith('\n' + NEVER) && prompts.crossModel.endsWith('\n' + NEVER) && NEVER.startsWith('**Never push to or merge into `main`') && Object.keys(prompts).filter((role) => prompts[role].includes(NEVER)).join(' ') === 'proposal crossModel')
  check('a role is handed the binary exactly when it is held to the hash it asserts', Object.keys(ROLES).every((role) => ROLES[role].drives === (ROLES[role].labels.includes('binary') || role === 'proposal')) && ROLES.review.drives && carries(prompts.review, 'binary'))

  check('the reporter list', launch.names.join(' ') === 'preflight scope row-3-source row-3-reconciler triage-p1 verify-p1-f-1 advocate-f-1 proposal-f-1 row-3-crossmodel fix-area-1')
  let twice = false
  try { launch.add(['scope']) } catch (e) { twice = true }
  let tooLong = false
  try { launch.add(['verify', 'p1', 'k'.repeat(100)]) } catch (e) { tooLong = true }
  check('a reporter launched twice, or with no name', twice && tooLong && launch.names.length === 10 && reporterName(['a', 'B']) === null)
  const batchOf = (text) => JSON.parse(text.split('\n').find((l) => l.startsWith('[{"argv"')))
  const spelled = (made) => made.argv.join(' ')
  const recordCalls = batchOf(prompts.record)
  check('the check names every reporter launched', launch.names.every((n) => checkReportsPrompt(ctx, launch.names).includes(' ' + n)) && checkReportsPrompt(ctx, launch.names).includes('`' + STEP_TOOL + ' check-reports --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 --scratch /tmp/scratch-1 --digest /tmp/scratch-1 -- ' + launch.names.join(' ') + '`') && spelled(recordCalls[0]) === 'check-reports --run rc24-tier1 --round 2 --stage fix --cycle 3 --attempt 4 -- ' + launch.names.join(' '))
  check('the ledger check names every key', spelled(recordCalls[recordCalls.length - 1]) === 'check-ledger --run rc24-tier1 -- f-1')
  check('a record\'s calls are one payload, written by a file tool and held to its hash by the writer', prompts.record.includes(' --sha256 ' + sha256(JSON.stringify(recordCalls) + '\n') + '`') && prompts.record.includes('\n1. Write the file `/tmp/scratch-1/record/x/batch-' + sha256(JSON.stringify(recordCalls) + '\n').slice(0, 12) + '.json` WITH YOUR FILE TOOL') && prompts.record.includes('\n' + PAYLOAD_ENDS + '\n' + JSON.stringify(recordCalls) + '\n' + PAYLOAD_ENDS + '\n') && !/cat |mkdir|shasum|<<|here-document/.test(prompts.record) && spelled(recordCalls[1]) === 'round-set --run rc24-tier1 --round 2' && recordCalls[1].stdin === '{"cycles":3}' && recordCalls.length === 3)
  check('a record step applies its calls as one batch from a file, RUNS NO GATE, and makes no commit', prompts.record.includes('\n2. `dev/stabilize-record apply --run rc24-tier1 --round 2 --subject \'docs(record): s\' --scratch /tmp/scratch-1 --from /tmp/scratch-1/record/x/batch-' + sha256(JSON.stringify(recordCalls) + '\n').slice(0, 12) + '.json --sha256 ' + sha256(JSON.stringify(recordCalls) + '\n') + '`') && !prompts.record.includes('\n3. ') && !prompts.record.includes('dev/gate') && prompts.record.endsWith(RECORD_RETURNS) && RECORD_RETURNS.includes('RUN NO GATE') && RECORD_RETURNS.includes('Return status = applied') && JSON.stringify(RECORD_SCHEMA.properties.status.enum) === JSON.stringify(['applied', 'halted']) && RECORD_SCHEMA.properties.gate === undefined && !/`git (add|commit)/.test(prompts.record))
  const stageRecord = recordPrompt(ctx, 'x', 'fix/rc24-tier1', '/tmp/scratch-1/record/z', 0, stageRecordCommands(Object.assign({}, ctx, { stage: 'test' }), { reporters: ['a'], gate: { commit: 'c'.repeat(40), file: '/tmp/scratch-1/gate/c2.a4.txt' }, results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }, { item: 'it\'s-b', status: 'void', reason: 'it\'s chain died' }]) }, rows: [], triage: [], patches: [], facts: { candidate: 'c'.repeat(40) }, keys: [] }), 's')
  check('what the commit step is held to is what was composed', JSON.stringify(stageRecord.expect) === JSON.stringify({ calls: 4, checks: 1 }) && stageRecord.hold === 'record-z' && stageRecord.gate === undefined && !stageRecord.text.includes(' --round 0') && JSON.stringify(recordPrompt(ctx, 'x', 'b', '/d', 2, recordCalls, 's').expect) === JSON.stringify({ calls: 3, checks: 2 }))
  check('a test stage\'s record puts the candidate\'s gate on record, from the file the tool kept its output in', spelled(batchOf(stageRecord.text)[1]) === 'gate-set --run rc24-tier1 --round 2 --commit ' + 'c'.repeat(40) + ' --summary /tmp/scratch-1/gate/c2.a4.txt --scratch /tmp/scratch-1' && !recordCalls.some((made) => made.argv[0] === 'gate-set'))
  check('what each item did is one call, its rows data whatever they hold', spelled(batchOf(stageRecord.text)[2]) === 'result-set --run rc24-tier1 --round 2 --commit ' + 'c'.repeat(40) + ' --scratch /tmp/scratch-1' && batchOf(stageRecord.text)[2].stdin === JSON.stringify([{ item: 'a', outcome: 'green' }, { item: 'it\'s-b', outcome: 'void', reason: 'it\'s chain died' }]))
  const resumed = pendingRecord('/tmp/scratch-1/record/pending', { calls: 5, checks: 2, files: 4, round: 1 })
  check('a pending batch is gated again by the tool: no executor runs, and nothing is applied', resumed.text === null && resumed.hold === 'record-pending' && JSON.stringify(resumed.expect) === JSON.stringify({ calls: 5, checks: 2 }))
  const cycleRecord = stageRecordCommands(ctx, { reporters: [], rows: [{ key: 'f-2' }], triage: [{ key: 'f-1', grade: 'breaks' }], patches: [{ key: 'f-1', disposition: 'fixed', detail: 'c'.repeat(40) }], clauses: [], facts: null, keys: ['f-1', 'f-2'] }).map((made) => made.argv[0])
  const at = (name) => cycleRecord.indexOf(name)
  check('a cycle\'s fixes are on the rows before its audit\'s triage reads them', at('ledger-add') >= 0 && at('ledger-add') < at('ledger-set') && at('ledger-set') < at('triage-set') && at('triage-set') < at('check-ledger'))

  const held = { status: 'recorded', commit: 'a'.repeat(40), gate: { ok: true, verdict: 'pass', red: 0, known: 0, new: 0 }, applied: { calls: 4, checks: 2, failed: 0 } }
  const composed = { calls: 4, checks: 2 }
  const lacking = (change) => recordFault(Object.assign({}, held, change), composed)
  check('a record whose line holds is recorded', recordFault(held, composed) === null)
  check('a record step that returns nothing, or halts, is not recorded', isText(recordFault(null, composed)) && recordFault({ status: 'halted', halt: { root_cause: 'gate-red: x' } }, composed) === 'gate-red: x' && isText(recordFault({ status: 'halted' }, composed)))
  check('a record without a commit, or without a gate check that holds, is not recorded', isText(lacking({ commit: 'abc' })) && isText(lacking({ gate: undefined })) && isText(lacking({ gate: { ok: false, new: 1 } })) && isText(lacking({ gate: 'GATE: PASS' })))
  check('a record that returns no evidence of its checks is not recorded', isText(lacking({ applied: undefined })) && isText(lacking({ applied: { calls: 4 } })) && isText(lacking({ applied: { calls: 4, checks: 0, failed: 0 } })) && isText(lacking({ applied: { calls: 4, checks: 1, failed: 0 } })) && isText(lacking({ applied: { calls: 3, checks: 2, failed: 0 } })) && isText(lacking({ applied: { calls: 4, checks: [{ check: 'reports', ok: true }, { check: 'ledger', ok: true }] } })))
  check('a record one of whose checks does not hold is not recorded', isText(lacking({ applied: { calls: 4, checks: 2, failed: 1 } })) && isText(lacking({ applied: { calls: 4, checks: 2 } })) && isText(lacking({ applied: { calls: 4, checks: 2, failed: null } })))
  check('a commit step that refused is said in the sentence of its word', recordFault({ status: 'halted', refused: 'gate-red', branch: 'fix/rc24-tier1' }, composed).includes('refused `gate-red` — the orchestrator\'s: ') && recordFault({ status: 'halted', relay: 'the relayed line is no digest' }, composed) === 'the relayed line is no digest')

  const heldStart = holdStartPrompt(base, 'gate-c2-a4', 'gate', '--run rc24-tier1', 'the full gate')
  const heldWait = holdWaitPrompt(base, 'gate-c2-a4', 'the full gate')
  check('the gate of a run is a command the tool holds, started by one plain call and asked after by another', heldStart.includes('\n1. `dev/stabilize-step hold-start --scratch /tmp/scratch-1 --name gate-c2-a4 --kind gate --run rc24-tier1 --digest /tmp/scratch-1`\n') && heldStart.split('\n')[0].endsWith(STEP_RULES) && heldWait.includes('\n1. `dev/stabilize-step hold-wait --scratch /tmp/scratch-1 --name gate-c2-a4 --digest /tmp/scratch-1`\n') && heldWait.split('\n')[0].endsWith(ASK_RULES) && ASK_RULES.includes('RUN THE SAME COMMAND AGAIN') && !STEP_RULES.includes('RUN THE SAME COMMAND AGAIN') && STEP_RULES.includes('never run the command a second time') && heldWait.includes('of the LAST run of it') && !heldStart.includes('of the LAST run of it'))

  const good = { status: 'reported', asserted_sha256: 'b'.repeat(64) }
  check('the hash comparison', hashMismatch('b'.repeat(64), [{ name: 'a', drives: true, result: good }, { name: 'b', drives: false, result: { status: 'reported' } }, { name: 'c', drives: true, result: { status: 'halted' } }, { name: 'd', drives: true, result: null }]).length === 0)
  check('a hash that differs, and one never returned', JSON.stringify(hashMismatch('b'.repeat(64), [{ name: 'a', drives: true, result: { status: 'reported', asserted_sha256: 'c'.repeat(64) } }, { name: 'b', drives: true, result: { status: 'reported' } }, { name: 'c', drives: true, result: good }])) === JSON.stringify([{ reporter: 'a', asserted: 'c'.repeat(64) }, { reporter: 'b', asserted: null }]))
  check('a needed field that is not there is the row of its role, and one that is there is none', JSON.stringify(lacks('review', { status: 'reported', findings: [] }).map((n) => n.field + ':' + n.then)) === JSON.stringify(['asserted_sha256:void']) && lacks('review', good).length === 0 && lacks('crossModel', { status: 'reported' }).length === 0 && JSON.stringify(lacks('verify', { status: 'verified', key: 'k' }).map((n) => n.field)) === JSON.stringify(['verdict', 'asserted_sha256']) && JSON.stringify(lacks('verify', { status: 'verified', key: 'k', verdict: 'confirmed', asserted_sha256: 'b'.repeat(64), ran_on: { candidate: 'b'.repeat(64) } }).map((n) => n.field + ':' + n.then)) === JSON.stringify(['regression:unknown', 'ran_on.previous:unknown']) && lacks('verify', { status: 'verified', key: 'k', verdict: 'refuted', asserted_sha256: 'b'.repeat(64) }).length === 0)
  check('why a return is no evidence about the candidate: no hash, another hash, and text that is none', unasserted('drive', 'drive-a-drive', { status: 'reported' }, 'b'.repeat(64)) === '(drive-a-drive) returned no `asserted_sha256` — ' + ASSERTED && unasserted('drive', 'd', { status: 'reported', asserted_sha256: 'c'.repeat(64) }, 'b'.repeat(64)) === '(d) returned the `asserted_sha256` ' + 'c'.repeat(64) + ', which is not the candidate\'s binary (' + 'b'.repeat(64) + '): nothing it drove is evidence about this candidate' && unasserted('drive', 'd', { status: 'reported', asserted_sha256: 'x`; rm -r /' }, 'b'.repeat(64)).includes('an `asserted_sha256` that is no sha256') && !unasserted('drive', 'd', { status: 'reported', asserted_sha256: 'x`; rm -r /' }, 'b'.repeat(64)).includes('rm -r') && unasserted('drive', 'd', good, 'b'.repeat(64)) === null && unasserted('crossModel', 'x', { status: 'reported' }, 'b'.repeat(64)) === null)
  check('every cell of what a stage needs is one of five words, and every row of what still halts names its phase', Object.values(NEEDS).every((rows) => rows.every((n) => ['void', 'unverified', 'no-fork', 'unknown', 'halts'].includes(n.then) && isText(n.field) && isText(n.why))) && Object.keys(NEEDS).every((role) => ROLES[role] && ROLES[role].drives) && Object.values(HALTS_AFTER).every((row) => isText(row.phase) && isText(row.when)) && Object.values(NEEDS).some((rows) => rows.some((n) => n.then === 'halts')) === ('regression-fact' in HALTS_AFTER))

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
  const ofTheQueue = (rulings) => rulingsRecordPrompt(base, 1, 'fix/rc24-tier1', rulings, {}, { round: 1, stages: {} })
  const raisedAndReordered = [{ budget: 40 }, { order: ['route under-confirmed', 'route hunt'] }]
  check('a budget and an order ruled in one invocation are ONE record of two calls: the order, and then the raise', JSON.stringify(batchOf(ofTheQueue(raisedAndReordered).text)) === JSON.stringify([{ argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"order":["route under-confirmed","route hunt"]}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"budget":40}' }]) && JSON.stringify(ofTheQueue(raisedAndReordered).expect) === JSON.stringify({ calls: 2, checks: 0 }))
  check('the record is the same, byte for byte, whichever ruling was said first', ofTheQueue(raisedAndReordered).text === ofTheQueue(raisedAndReordered.slice().reverse()).text && ofTheQueue([{ queued: ['f-2', 'f-1'] }, { order: ['route hunt'] }, { budget: 40 }]).text === ofTheQueue([{ budget: 40 }, { queued: ['f-2', 'f-1'] }, { order: ['route hunt'] }]).text)
  check('the rows put in by key stand between the two, and the calls of the other rulings about the run before all three', JSON.stringify(batchOf(rulingsRecordPrompt(base, 2, 'fix/rc24-tier1', [{ budget: 40 }, { queued: ['f-2', 'f-1'] }, { go: true }, { rounds: 4 }, { order: ['route hunt'] }], {}, { round: 2, stages: {} }).text)) === JSON.stringify([{ argv: ['round-set', '--run', 'rc24-tier1', '--round', '2'], stdin: '{"go":true}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"rounds":4}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"order":["route hunt"]}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"queued":["f-2","f-1"]}' }, { argv: ['run-set', '--run', 'rc24-tier1'], stdin: '{"budget":40}' }]) && ofTheQueue(raisedAndReordered).text.includes('2 about the run'))

  const results = (units) => JSON.stringify(resultRows(units))
  check('a result per item, as it ran', results([{ item: 'a', status: 'green', reason: 'r' }, { item: 'b', status: 'green' }]) === JSON.stringify([{ item: 'a', outcome: 'green' }, { item: 'b', outcome: 'green' }]) && results([]) === '[]')
  check('a void item says why', results([{ item: 'a', status: 'void', reason: 'a step of its chain did not report' }, { item: 'b', status: 'void' }]) === JSON.stringify([{ item: 'a', outcome: 'void', reason: 'a step of its chain did not report' }, { item: 'b', outcome: 'void', reason: 'it did not run to its end' }]))
  check('a red check brings what its finding needs', results([{ item: 'gate', status: 'red', reason: 'r', door: 'jigc setup', evidence: 'two tests red' }, { item: 'ci', status: 'red', evidence: null }]) === JSON.stringify([{ item: 'gate', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `gate`', repro: 'two tests red' }, { item: 'ci', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `ci`', repro: 'the check returned red, and no evidence beside it' }]))
  check('no result names a clause, and no call sets one', !resultRows([{ item: 'a', clause: 'x', status: 'green' }]).some((row) => 'clause' in row) && !stageRecordCommands(ctx, { reporters: [], results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }]) }, rows: [], triage: [], patches: [], facts: null, keys: [] }).some((made) => made.argv[0].startsWith('clause')))
  const reRecord = stageRecordCommands(Object.assign({}, ctx, { stage: 'test' }), { reporters: ['a'], gate: null, results: { commit: 'c'.repeat(40), rows: resultRows([{ item: 'a', status: 'green' }]) }, rows: [{ key: 'f-1' }], triage: [{ key: 'f-1', grade: 'breaks' }], patches: [], facts: null, keys: ['f-1'] })
  check('a re-run\'s record is its results before anything of the ledger, and no fact of the round', JSON.stringify(reRecord.map((made) => made.argv[0])) === JSON.stringify(['check-reports', 'result-set', 'ledger-add', 'triage-set', 'check-ledger']))

  const classes = classifyCommits(['a1', 'b2', 'c3', 'd4', 'e5'], ['b2', 'd4', 'e5'], ['a1', 'c3', 'e5'])
  check('a round\'s commits by class', JSON.stringify(classes) === JSON.stringify({ records: ['b2', 'd4'], fixes: ['a1', 'c3'], mixed: ['e5'] }) && classifyCommits(['z9'], [], []).mixed.length === 1)
  const ledger = [{ key: 'f-1', disposition: 'fixed', detail: 'aaaaaaa1' }, { key: 'f-2', disposition: 'fixed', detail: 'bbbbbbb2' + '0'.repeat(32) }, { key: 'f-3', disposition: 'open', detail: null }, { key: 'f-4', disposition: 'fixed', detail: 'ccccccc3' }]
  check('a dropped round\'s fixes are open again', JSON.stringify(reopenPatches(ledger, ['aaaaaaa1' + '0'.repeat(32), 'bbbbbbb2' + '0'.repeat(32)])) === JSON.stringify([{ key: 'f-1', disposition: 'open' }, { key: 'f-2', disposition: 'open' }]) && reopenPatches(ledger, []).length === 0)
  check('a part\'s kept fixes name their new commits', JSON.stringify(repointPatches(ledger, [{ from: 'aaaaaaa1' + '0'.repeat(32), to: 'd'.repeat(40) }])) === JSON.stringify([{ key: 'f-1', disposition: 'fixed', detail: 'd'.repeat(40) }]))
  check('two commits are one by their prefix only', sameCommit('abcdef1', 'abcdef1234') && !sameCommit('abcdef1', 'abcdef2') && !sameCommit('abc', 'abcdef1') && !sameCommit('abcdef1', null))

  const areas = areasOf([{ key: 'f-1', door: 'jigc setup' }, { key: 'f-2', door: 'nowhere' }, { key: 'f-3', door: 'jigc doc show' }, { key: 'f-4', door: 'finalize.dirty' }], { included: [{ door: 'jigc setup', registry: 'verbs' }, { door: 'finalize.dirty', registry: 'codes' }], excluded: [{ door: 'jigc doc show', registry: 'verbs' }] })
  check('the areas', areas.map((x) => x.n + ':' + x.findings.map((f) => f.key).join('+')).join(' ') === '1:f-1+f-3 2:f-2 3:f-4' && areasOf([], null).length === 0 && areasOf([{ key: 'f', door: 'd' }], null).length === 1)
  check('a unit is handed its brief and its doors as the reads that print them, and no text of either', prompts.review.includes('\nYour brief is the `brief` of your item\'s row in the test set — read it: `dev/stabilize-record item --run rc24-tier1 --item row-3`.\n') && prompts.review.includes('is what `dev/stabilize-record item-doors --run rc24-tier1 --round 2 --item row-3` prints: 1 door(s) of round 2\'s test set') && !prompts.review.includes('Your brief: ') && !prompts.review.includes('   - ') && prompts.crossModel.includes('`dev/stabilize-record item --run rc24-tier1 --item row-3`') && prompts.crossModel.includes('`dev/stabilize-record item-doors --run rc24-tier1 --round 2 --item row-3`') && doorsRead('rc24-tier1', 1, 'row-3').endsWith(' --round 1 --item row-3'))
  check('the one unit that is this script\'s own is handed its brief and its doors as given', unitPrompt(ctx, launcher(ctx), 'x', FIX_AUDIT[0][0], { item: 'audit', brief: 'The fix diff.', doors: [{ door: 'jigc setup', registry: 'verbs' }], range: 'a..b' }, built, []).includes('\nYour brief: The fix diff.\n') && briefLine(ctx, { item: 'audit', brief: '' }) === 'Your brief: ' && doorLines(ctx, { item: 'a', doors: [] }).endsWith('\n   (none)') && doorLines(ctx, { item: 'a', doors: null, round: 3 }).includes('prints: the doors of round 3\'s test set'))
  check('a check is listed for the preflight by its item and the read that prints its brief', prompts.preflight.includes('\n   - ci: its brief is the `brief` of the item\'s row — `dev/stabilize-record item --run rc24-tier1 --item ci`'))
  check('triage is handed the ledger\'s rows as the read that prints them, held to their count', triagePrompt(ctx, launcher(ctx), 'triage-p1', handed.concat([{ reporter: 'row-3-source', report: 'a.md', findings: ['1 — x'] }]), 1).includes('Grade every finding below — 3 in all, from 2 reporter(s).') && triagePrompt(ctx, launcher(ctx), 'triage-p1', handed, 1).includes('\n- ledger — report: `completions/artifacts/rc24-tier1/ledger.md` — 2 finding(s): the rows `dev/stabilize-record untriaged --run rc24-tier1` prints') && prompts.triage.includes('\n- row-3-source — report: `a.md` — 1 finding(s):\n   - 1 — x'))

  const digest = (said) => {
    const body = JSON.stringify(Object.assign({}, said, { unfit: said.unfit || [], file: said.file === undefined ? 'lines/' + said.act + '.0123456789abcdef.json' : said.file, file_sha256: 'f'.repeat(64) }))
    return body.slice(0, -1) + ', "sha256": "' + sha256(body) + '"}'
  }
  const asDigest = (text, act) => readDigest({ status: 'ran', line: text }, act || 'state')
  const stateDigest = digest({ act: 'state', status: 'read', branch: 'fix/rc24-tier1', state: { run: 'rc24-tier1', opened: true, next: 'fix', ledger: 3 }, file: 'state/test-1.json' })
  check('a digest is read: what the tool printed, and the file that holds the rest', asDigest(stateDigest).status === 'read' && asDigest(stateDigest + '\n').state.ledger === 3 && asDigest(stateDigest).file === 'state/test-1.json' && !asDigest(stateDigest).relay && JSON.stringify(fileOf(asDigest(stateDigest))) === JSON.stringify({ file: 'state/test-1.json', sha256: 'f'.repeat(64) }) && fileOf({ file: null }) === null && fileOf(null) === null)
  check('the line the tool prints unasked is no digest, though it hashes: the document is not read', relay(line).status === 'read' && asDigest(line).status === 'halted' && asDigest(line).relay.includes('is no digest') && asDigest(refusedLine, 'land').relay.includes('is no digest') && !JSON.stringify(asDigest(line)).includes('rc24-tier1'))
  const dashed = printed({ act: 'state', status: 'read', state: { run: 'a — b' }, unfit: [], file: 'state/t.json', file_sha256: 'f'.repeat(64) })
  const escaped = printed({ act: 'state', status: 'read', state: { run: 'a \\ b' }, unfit: [], file: 'state/t.json', file_sha256: 'f'.repeat(64) })
  check('a line with a character outside ASCII, or a backslash, is no digest, though it hashes', relay(dashed).status === 'read' && asDigest(dashed).relay.includes('outside printable ASCII') && relay(escaped).status === 'read' && asDigest(escaped).relay.includes('a backslash') && !!asDigest(stateDigest.replace('"fix"', '"fox"')).relay && !!asDigest(stateDigest.replace('rc24-tier1', 'rc24—tier1')).relay)
  const struck = asDigest(digest({ act: 'push', status: 'ready', branch: null, head: null, unfit: ['branch', 'head'] }), 'push')
  check('a digest with a field struck out is a step that is not acted on, and names the file its own line is in', struck.status === 'halted' && struck.unfit === true && struck.said === 'ready' && struck.relay.includes('could not print branch, head') && struck.relay.includes('THE ACT MAY HAVE RUN') && struck.file === 'lines/push.0123456789abcdef.json')
  check('what is no line at all, and a step whose agent halted, are read as before', readDigest(null, 'state') === null && !!readDigest({ status: 'ran' }, 'state').relay && readDigest({ status: 'halted', halt: { root_cause: 'x' } }, 'state').halt.root_cause === 'x' && !!asDigest(stateDigest, 'push').relay && !JSON.stringify(relay(line.replace('"fix"', '"fox"'))).includes('fox'))
  const waited = digest({ act: 'hold-wait', status: 'done', name: 'gate-c1-a1', facts: { steps: [{ step: 'build', exit: 0 }, { step: 'test', exit: 1 }, { step: 'fmt', exit: 0 }] } })
  check('a digest of any act with one value slipped is read as the tool printed it', asDigest(waited.replace('"step":"fmt","exit":0', '"step":"fmt","exit":1'), 'hold-wait').status === 'done' && asDigest(waited.replace('"step":"fmt","exit":0', '"step":"fmt","exit":1'), 'hold-wait').facts.steps[2].exit === 0 && !!asDigest(waited.replace('"step":"test","exit":1', '"step":"test","exit":0'), 'hold-wait').relay)

  check('every refusal of the tool has a row that says whose it is and what leaves it', Object.keys(STEP_REFUSALS).length === 30 && Object.keys(STEP_REFUSALS).every((word) => /^[a-z][a-z-]*$/.test(word) && isText(STEP_REFUSALS[word].whose) && isText(STEP_REFUSALS[word].leaves) && stepRefusal(word).startsWith('`' + word + '` — ' + STEP_REFUSALS[word].whose + ': ')))
  check('a word the table lacks is said to be one', stepRefusal('no-such-word').includes('has no row') && !stepRefusal('locked').includes('has no row'))
  check('a commit that is no record\'s is its author\'s, a lock the orchestrator\'s, and what would not have been written the human\'s', STEP_REFUSALS['foreign-commit'].whose === 'its author\'s' && stepRefusal('foreign-commit', 'fix/rc24-tier1').includes('`git push origin fix/rc24-tier1`') && STEP_REFUSALS.locked.whose === 'the orchestrator\'s' && STEP_REFUSALS.locked.leaves.includes('never removes') && STEP_REFUSALS['head-moved'].whose === 'the orchestrator\'s' && STEP_REFUSALS.unvetted.whose.startsWith('the human\'s at a push') && stepRefusal('unvetted', 'fix/rc24-tier1').includes('`dev/stabilize-record vet --range -- fix/rc24-tier1 --not --remotes=origin`'))
  const discards = '`dev/stabilize-step discard --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1`'
  check('a record step that stopped says what the next invocation does, and names the act that takes a batch back', [null, 'gate-red', 'head-moved', 'did-not-run', 'locked', 'no-batch'].every((word) => recordThen('rc24-tier1', 'fix/rc24-tier1', word).includes(discards) && !recordThen('rc24-tier1', 'fix/rc24-tier1', word).includes('stabilize-record discard')) && recordThen('rc24-tier1', 'fix/rc24-tier1', 'gate-red').includes('THE BATCH IS STILL APPLIED') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'head-moved').includes('its first read meets the commit') && recordThen('rc24-tier1', 'fix/rc24-tier1', null).includes('may have left its batch APPLIED AND NOT COMMITTED'))
  check('a commit refused as unvetted has taken its batch back, and asks for no discard', recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', true).includes('THE BATCH IS TAKEN BACK ALREADY') && !recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', true).includes(discards) && !recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', true).includes('pending, and no dirty tree') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted', false).includes('was NOT taken back') && recordThen('rc24-tier1', 'fix/rc24-tier1', 'unvetted').includes('was NOT taken back'))
  check('a push that failed is owed to the next first read — but never one refused as unvetted, and not beside a commit that is no record\'s', pushThen('fix/rc24-tier1', null).includes('THE PUSH IS OWED') && pushThen('fix/rc24-tier1', 'push-rejected').includes('its first read finds the record the remote lacks') && pushThen('fix/rc24-tier1', 'unvetted').includes('THE PUSH IS NOT OWED TO ANY LATER INVOCATION') && !pushThen('fix/rc24-tier1', 'unvetted').includes('THE PUSH IS OWED:') && pushThen('fix/rc24-tier1', 'foreign-commit').includes('`git push origin fix/rc24-tier1`') && !pushThen('fix/rc24-tier1', 'foreign-commit').includes('THE PUSH IS OWED:'))
  check('a step that returned nothing is asked for again as its one command, and no agent of a step is told to look around', RUN_AGAIN.includes('Run the ONE command above again') && !RUN_AGAIN.includes('git status') && !RUN_AGAIN.includes('git log') && LOOK_AGAIN.includes('the tree\'s status') && !LOOK_AGAIN.includes('`git ') && RUN_AGAIN !== LOOK_AGAIN)
  check('an invocation that only looks says so to its first read, and to no other step', gitStatePrompt(base, true).includes(' --product crates --look --digest /tmp/scratch-1`') && !steps['git-state'].includes('--look') && Object.keys(steps).every((act) => !steps[act].includes('--look')))
  check('a record\'s push on the loop branch publishes records only, and its commit is held to the commit the stage began on', pushPrompt(base, 'fix/rc24-tier1', true).includes(' push --branch fix/rc24-tier1 --run-dir completions/artifacts/rc24-tier1 --digest /tmp/scratch-1`') && !steps.push.includes('--run-dir') && recordCommitPrompt(base, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }, 'c'.repeat(40)).includes(' --calls 3 --checks 2 --scratch /tmp/scratch-1 --head ' + 'c'.repeat(40) + ' --digest /tmp/scratch-1`') && !steps.record.includes('--head') && recordCommitPrompt(base, 'fix/rc24-tier1', '/tmp/scratch-1/record/x/gate.txt', { calls: 3, checks: 2 }, 'c'.repeat(40), 'd'.repeat(64)).includes(' --head ' + 'c'.repeat(40) + ' --fence ' + 'd'.repeat(64) + ' --digest /tmp/scratch-1`') && !steps.record.includes('--fence') && STEP_REFUSALS['fence-moved'].whose === 'the human\'s' && recordThen('rc24-tier1', 'fix/rc24-tier1', 'fence-moved').includes('PAST THE WRITE GUARD'))

  const namedState = { round: 2, never_selected: ['row-b'], uncovered: [{ round: 1, doors: 3 }, { round: 2, doors: 1 }], document: { file: '/tmp/scratch-1/state/test-2.json', sha256: 'f'.repeat(64) }, ledger: 60, blockers: 2, human_list: 5, untriaged: { count: 1, why: [{ why: 'ungraded', count: 1 }] } }
  check('every return names the items no round selected, the doors of the round no item names, and the file', JSON.stringify(namedOf(namedState)) === JSON.stringify({ never_selected: ['row-b'], uncovered: { round: 2, doors: 1 }, regression_unknown: 0, document: namedState.document }) && namedOf(Object.assign({}, namedState, { regression_unknown: ['f-1', 'f-2'] })).regression_unknown === 2 && namedOf(namedState, 1).uncovered.doors === 3 && namedOf(namedState, 3).uncovered.doors === 0 && namedOf(null) === null && JSON.stringify(namedOf({ opened: false })) === JSON.stringify({ never_selected: [], uncovered: { round: null, doors: 0 }, regression_unknown: 0, document: null }))
  check('what a return holds of the state is counts and words, never a list of rows', attached(namedState).ledger === 60 && attached(namedState).blockers === 2 && attached(namedState).human_list === 5 && attached(namedState).untriaged.count === 1 && !('document' in attached(namedState)) && !('items' in attached(namedState)))
  const shown = { round: 1, file: 'completions/artifacts/rc24-tier1/r1/view.md', sha256: 'c'.repeat(64), on_disk: 'written', verified: 3, confirmed: 2, refuted: 1, undrivable: 0, rates: [{ kind: 'route', origin: 'hunt', verified: 3, confirmed: 2, refuted: 1, undrivable: 0, queued: 2 }], inside: 1, outside: 1, unreached: 2, listed: [{ why: 'lead', rows: 4 }], other: 6, codes: 1, sites: 0 }
  const queuedState = { next: 'verify', queue: { budget: 30, hand_on: 2, spent: 3, left: 27, length: 2, first: ['f-1', 'f-2'], classes: [{ kind: 'route', origin: 'hunt', length: 2 }] }, view: shown, position: { test: { round: 1, attempt: 2, verify: true, filed: true } } }
  check('a return of a run with a queue says the budget, the queue by class and the stop\'s view as the digest has them, and what the stage is continued for', JSON.stringify(queueSaid(queuedState)) === JSON.stringify({ continues: ['verify', 'filed'], budget: { limit: 30, spent: 3, left: 27 }, queue: { length: 2, hand_on: 2, classes: [{ kind: 'route', origin: 'hunt', length: 2 }] }, view: shown }))
  check('no key of the queue is in it, and a run without an order says none of it', !JSON.stringify(queueSaid(queuedState)).includes('f-1') && JSON.stringify(queueSaid({ next: 'fix', queue: null, position: { test: { refused: 'round-open', round: 1 } } })) === '{}' && JSON.stringify(queueSaid(null)) === '{}')
  check('only a record is owed where the stage is continued for a filed verdict and nothing else, and nothing where the position hands no continuation', JSON.stringify(queueSaid(Object.assign({}, queuedState, { position: { test: { round: 1, attempt: 2, filed: true } } })).continues) === '["filed"]' && JSON.stringify(queueSaid(Object.assign({}, queuedState, { position: { test: { refused: 'round-open', round: 1 } } })).continues) === '[]' && queueSaid(Object.assign({}, queuedState, { view: undefined })).view === null)
  check('what an invocation says happened: it hunted, it verified, or it continued a stage and recorded no verdict', statusOf([], null) === 'hunted' && statusOf([], 'a'.repeat(40)) === 'verified' && statusOf(['verify'], 'a'.repeat(40)) === 'verified' && statusOf(['filed'], 'a'.repeat(40)) === 'verified' && statusOf(['verify'], null) === 'continued' && statusOf(['triage', 'verify'], null) === 'continued')

  const job = { act: 'hold-wait', name: 'gate-c1-a1', kind: 'gate', output: 'hold/gate-c1-a1/output' }
  const doneAs = (verdict, why) => heldOf(Object.assign({ status: 'done', verdict, why: why || null, verdict_file: 'hold/gate-c1-a1/verdict.json', verdict_sha256: 'e'.repeat(64) }, job), '/tmp/scratch-1')
  check('a held check is an item whose kind names a held command of the tool', heldKind('held-regression') === 'regression' && heldKind('held-gate') === 'gate' && heldKind('held-build') === null && heldKind('held-') === null && heldKind('check') === null && heldKind('regression') === null && heldKind(null) === null && HELD_CHECKS.every((id) => !CHAINS[HELD_PREFIX + id] && HELD_PREFIX + id !== CHECK_KIND))
  const startedWith = (previous, candidate) => holdStartPrompt(base, 'regression-set-' + candidate.slice(0, 12) + '-a1', 'regression', regressionFlags(previous, candidate), 'the regression set')
  check('a held regression set is started with the two commits, the list and the previous release\'s kept baseline — a path below the scratch root named by that release\'s commit and by nothing else: two candidates name one, two previous releases two, and it lies in no job\'s directory', startedWith('c'.repeat(40), 'd'.repeat(40)).includes('\n1. `dev/stabilize-step hold-start --scratch /tmp/scratch-1 --name regression-set-dddddddddddd-a1 --kind regression --previous ' + 'c'.repeat(40) + ' --candidate ' + 'd'.repeat(40) + ' --list ' + REGRESSION_LIST + ' --baseline baseline/previous-cccccccccccc --digest /tmp/scratch-1`\n') && startedWith('c'.repeat(40), 'e'.repeat(40)).includes(' --baseline baseline/previous-cccccccccccc ') && startedWith('f'.repeat(40), 'd'.repeat(40)).includes(' --baseline baseline/previous-ffffffffffff ') && baselineOf.length === 1 && !baselineOf('c'.repeat(40)).startsWith('hold/') && !baselineOf('c'.repeat(40)).startsWith('/') && baselineOf('c'.repeat(40)).split('/').every((part) => /^[A-Za-z0-9_][A-Za-z0-9._-]*$/.test(part)))
  check('a held command that still runs is asked for again', heldOf(Object.assign({ status: 'running' }, job), '/tmp/scratch-1').ends === 'running' && heldOf(Object.assign({ status: 'started' }, job, { act: 'hold-start' }), '/tmp/scratch-1').ends === 'running' && heldOf(Object.assign({ status: 'running' }, job), '/tmp/scratch-1').output === '/tmp/scratch-1/hold/gate-c1-a1/output' && heldOf(Object.assign({ status: 'started' }, job), '/tmp/scratch-1').fresh === true && heldOf(Object.assign({ status: 'running' }, job), '/tmp/scratch-1').fresh === false && doneAs('green').fresh === false)
  check('a held command that ran to its end is the tool\'s verdict, by the file the tool kept it in', ['green', 'red', 'void'].every((verdict) => doneAs(verdict).ends === verdict && doneAs(verdict).verdict === '/tmp/scratch-1/hold/gate-c1-a1/verdict.json' && doneAs(verdict).verdict_sha256 === 'e'.repeat(64)) && doneAs('void', 'no-verdict').why === 'no-verdict' && doneAs('green').why === null)
  check('a held command that is dead has no verdict, and one that is done without the tool\'s verdict is not read', heldOf(Object.assign({ status: 'dead', why: 'no-exit' }, job), '/tmp/scratch-1').ends === 'dead' && heldOf(Object.assign({ status: 'dead', why: 'killed', verdict: 'green', verdict_file: 'hold/x/verdict.json', verdict_sha256: 'e'.repeat(64) }, job), '/tmp/scratch-1').verdict === undefined && doneAs('passed').ends === 'unread' && heldOf(Object.assign({ status: 'done', verdict: 'green' }, job), '/tmp/scratch-1').ends === 'unread' && heldOf(null, '/tmp/scratch-1').ends === 'unread' && heldOf({ status: 'halted', refused: 'missing' }, '/tmp/scratch-1').refused === 'missing' && heldOf({ status: 'halted', relay: 'altered' }, '/tmp/scratch-1').why === 'altered')
  check('every way a held command can stand has its sentence', JSON.stringify(Object.keys(HELD_ENDS)) === JSON.stringify(['running', 'green', 'red', 'void', 'dead', 'unread']) && Object.keys(HELD_ENDS).every((ends) => isText(HELD_ENDS[ends])))
  check('a held check that left no verdict is void by ONE word of the shape the record script takes: the tool\'s own word where it refused the start, how the job stood otherwise, and the job\'s name', heldVoid({ ends: 'not-run', kind: 'gate', why: 'off-tree' }) === 'off-tree' && heldVoid({ ends: 'dead', why: 'no-exit', name: 'regression-set-c1-a1' }) === 'dead:regression-set-c1-a1' && heldVoid({ ends: 'running', name: 'reg-c1-a2' }) === 'running:reg-c1-a2' && heldVoid({ ends: 'unread', refused: 'taken', name: 'reg-c1-a1' }) === 'taken:reg-c1-a1' && heldVoid({ ends: 'unread', why: 'the step returned no result', name: 'reg-c1-a1' }) === 'unread:reg-c1-a1' && heldVoid({ ends: 'unread', kind: 'regression', why: 'no name of a held command can be made' }) === 'unread' && Object.keys(HELD_ENDS).filter((ends) => !HELD_TAKES.check.includes(ends)).concat(['not-run']).every((ends) => HELD_VOID_RE.test(heldVoid({ ends, name: 'x'.repeat(SLUG_MAX) })) && heldVoid({ ends, name: 'x'.repeat(SLUG_MAX) }).length <= 200) && Object.keys(STEP_REFUSALS).every((word) => HELD_VOID_RE.test(heldVoid({ ends: 'unread', refused: word, name: 'a-b' }))) && !HELD_VOID_RE.test('it did not run to its end') && !HELD_VOID_RE.test('dead:/tmp/x') && JSON.stringify(resultRows([{ item: 'reg', status: 'void', reason: heldVoid({ ends: 'dead', name: 'reg-c1-a1' }) }])) === JSON.stringify([{ item: 'reg', outcome: 'void', reason: 'dead:reg-c1-a1' }]))
  const knows = { status: 'verified', verdict: 'confirmed', regression: true, asserted_sha256: 'b'.repeat(64), ran_on: { candidate: 'b'.repeat(64), previous: 'c'.repeat(64) } }
  const without = (drop) => { const r = JSON.parse(JSON.stringify(knows)); drop(r); return r }
  check('a regression fact is taken only as a boolean with the previous binary\'s hash behind it, and is unknown otherwise — never false', JSON.stringify(regressionOf('v', knows, 'c'.repeat(64))) === JSON.stringify({ regression: true, unknown: null }) && regressionOf('v', without((r) => { r.regression = false }), 'c'.repeat(64)).regression === false && [without((r) => { delete r.regression }), without((r) => { delete r.ran_on.previous }), without((r) => { delete r.ran_on }), without((r) => { r.regression = 'no' }), without((r) => { r.ran_on.previous = 'd'.repeat(64) }), without((r) => { r.regression = null })].every((r) => regressionOf('v', r, 'c'.repeat(64)).regression === null && isText(regressionOf('v', r, 'c'.repeat(64)).unknown)) && regressionOf('verify-p1-k', without((r) => { delete r.regression }), 'c'.repeat(64)).unknown.startsWith('(verify-p1-k) returned no `regression` — ') && regressionOf('v', without((r) => { delete r.ran_on.previous }), 'c'.repeat(64)).unknown.startsWith('(v) returned no `ran_on.previous` — ') && regressionOf('v', without((r) => { r.ran_on.previous = 'x`; rm -r /' }), 'c'.repeat(64)).unknown.includes('a hash that is no sha256') && !regressionOf('v', without((r) => { r.ran_on.previous = 'x`; rm -r /' }), 'c'.repeat(64)).unknown.includes('rm -r'))
  check('a confirmed finding whose regression fact is unknown is recorded with the field LEFT OUT, and one that is known with its boolean', JSON.stringify(triageRecord(ctx, [{ key: 'f-1', new: false, grade: 'breaks', verdict: 'confirmed', regression: null }, { key: 'f-2', new: false, grade: 'breaks', verdict: 'confirmed', regression: false }, { key: 'f-3', new: false, grade: 'breaks', verdict: 'confirmed' }, { key: 'f-4', new: false, grade: 'breaks', verdict: 'refuted', regression: true }]).triage) === JSON.stringify([{ key: 'f-1', grade: 'breaks', verdict: 'confirmed' }, { key: 'f-2', grade: 'breaks', verdict: 'confirmed', regression: false }, { key: 'f-3', grade: 'breaks', verdict: 'confirmed' }, { key: 'f-4', grade: 'breaks', verdict: 'refuted' }]))
  check('a door the previous release does not have is NOT COMPARABLE: taken with that binary\'s hash behind it, unknown without, and recorded as it is', regressionOf('v', without((r) => { r.regression = NOT_COMPARABLE }), 'c'.repeat(64)).regression === NOT_COMPARABLE && regressionOf('v', without((r) => { r.regression = NOT_COMPARABLE; delete r.ran_on.previous }), 'c'.repeat(64)).regression === null && regressionOf('v', without((r) => { r.regression = 'not comparable' }), 'c'.repeat(64)).regression === null && JSON.stringify(triageRecord(ctx, [{ key: 'f-1', new: false, grade: 'breaks', verdict: 'confirmed', regression: NOT_COMPARABLE }]).triage) === JSON.stringify([{ key: 'f-1', grade: 'breaks', verdict: 'confirmed', regression: NOT_COMPARABLE }]))
  check('what a stage holds takes the ends its table names, and no other', JSON.stringify(HELD_TAKES) === JSON.stringify({ build: ['green'], gate: ['green', 'red'], check: ['green', 'red', 'void'], record: ['green', 'red'], probe: ['green'] }) && Object.keys(HELD_TAKES).every((use) => HELD_TAKES[use].every((ends) => HELD_ENDS[ends])) && !Object.keys(HELD_TAKES).some((use) => ['running', 'dead', 'unread'].some((ends) => HELD_TAKES[use].includes(ends))))
  check('a held command that may still run is named with the one call that answers once it is over', mayRun({ name: 'gate-c1-a1', ends: 'running' }) && mayRun({ name: 'gate-c1-a1', ends: 'unread', started: true }) && !mayRun({ name: 'gate-c1-a1', ends: 'unread', started: false }) && !mayRun({ name: 'gate-c1-a1', ends: 'dead' }) && !mayRun({ name: 'gate-c1-a1', ends: 'void' }) && !mayRun({ ends: 'running' }) && heldThen('/tmp/scratch-1', { name: 'gate-c1-a1', ends: 'running', output: '/tmp/scratch-1/hold/gate-c1-a1/output' }).includes('`dev/stabilize-step hold-wait --scratch /tmp/scratch-1 --name gate-c1-a1`') && heldThen('/tmp/scratch-1', { name: 'gate-c1-a1', ends: 'running' }).includes('MAY STILL RUN') && heldThen('/tmp/scratch-1', { name: 'gate-c1-a1', ends: 'dead' }) === null)
  check('a held command that did not end as the stage needs it is said by its job and the tool\'s word', heldFault('the candidate\'s gate', { name: 'gate-c1-a1', ends: 'void', why: 'no-verdict' }).includes('`gate-c1-a1`') && heldFault('the candidate\'s gate', { name: 'gate-c1-a1', ends: 'void', why: 'no-verdict' }).includes('`no-verdict`') && heldFault('x', { ends: 'unread' }).includes(HELD_ENDS.unread) && JSON.stringify(heldSaid({ name: 'n', kind: 'gate', ends: 'dead', why: 'no-exit', output: '/o', verdict: '/v', facts: {} })) === JSON.stringify({ name: 'n', kind: 'gate', ends: 'dead', why: 'no-exit', output: '/o' }))
  check('a command an agent types is rendered by one function, as one code span', typed(STEP_TOOL, 'table') === '`dev/stabilize-step table`' && typed(RECORD_TOOL, '') === '`dev/stabilize-record`' && RECORD_TOOL === 'dev/stabilize-record' && isText(REGRESSION_LIST) && !REGRESSION_LIST.startsWith('/') && HOLD_LAPS >= 1 && HOLD_NAMES >= 2)
  check('the subject of a record is plain words, and so is every word of the call that applies it', /^[A-Za-z0-9 _.\/:=@%+,()-]+$/.test('docs(record): ' + subjectOf('rc24-tier1', 2, 'the record of the test stage')) && subjectOf('rc24-tier1', 2, 'a re-run of no-lost-files') === 'rc24-tier1 r2 - a re-run of no-lost-files' && /--subject 'docs\(record\): rc24-tier1 r1 - the rulings of the human' /.test(rulingsRecordPrompt(fix, 1, 'fix/rc24-tier1', [{ go: true }], {}, { round: 1, stages: {} }).text))
  check('the scope and a report reach their writer from a file the agent writes with its file tool', prompts.scope.includes('to the file `/tmp/scratch-1/scope/r2.a4.json` WITH YOUR FILE TOOL') && prompts.scope.includes('`dev/stabilize-record scope-set --run rc24-tier1 --round 2 --scratch /tmp/scratch-1 --from /tmp/scratch-1/scope/r2.a4.json`') && prompts.proposal.includes(reportCall(ctx, 'proposal-f-1')) && prompts.crossModel.includes(reportCall(ctx, 'row-3-crossmodel')) && [prompts.proposal, prompts.crossModel, prompts.scope, prompts.review].every((text) => !text.includes('here-document')))
  check('the one driving role with no definition asks the tool for the binary\'s hash', prompts.proposal.includes('`dev/stabilize-step hash --scratch /tmp/scratch-1 --file bin/c2/jigc`') && prompts.proposal.includes('`content_sha256` must be ' + 'b'.repeat(64)) && !prompts.proposal.includes('shasum') && below('/tmp/scratch-1', '/tmp/scratch-1/bin/c2/jigc') === 'bin/c2/jigc' && below('/tmp/scratch-1', '/elsewhere/jigc') === '/elsewhere/jigc')
  check('a held check\'s result is the file of the tool\'s verdict, and no word', results([{ item: 'regression-set', status: 'green', verdict: '/tmp/scratch-1/hold/regression-c1-a1/verdict.json' }, { item: 'gate', status: 'red', evidence: 'e' }]) === JSON.stringify([{ item: 'regression-set', verdict: '/tmp/scratch-1/hold/regression-c1-a1/verdict.json' }, { item: 'gate', outcome: 'red', doctype: 'jigc-feedback', door: 'the check `gate`', repro: 'e' }]))
  check('a refusal of a batch as one that changes nothing is the record script\'s word', NO_CHANGE === 'no-change' && RECORD_SCHEMA.properties.refused.type === 'string' && !RECORD_SCHEMA.required.includes('refused') && RECORD_RETURNS.includes('return refused = the ONE word'))

  const probeBegin =probeStep('its first act', 'begin', '--scratch /tmp/scratch-1 --probe relay')
  check('a probe\'s step is the harness\'s own step prompt, with the probe tool where the step tool stands', probeBegin === stepPrompt('a PROBE of the stabilization harness, of no run — its first act', 'begin', '--scratch /tmp/scratch-1 --probe relay').split(STEP_TOOL).join(PROBE_TOOL) && probeBegin.includes('\n1. `dev/stabilize-probe begin --scratch /tmp/scratch-1 --probe relay`\n') && probeBegin.includes(STEP_RULES) && probeBegin.includes(AS_PRINTED) && !probeBegin.includes(STEP_TOOL))
  check('the field a reviewer came back with, as a cell — and nothing an agent wrote', probeField(null) === 'none' && probeField({ status: 'halted', asserted_sha256: 'a'.repeat(64) }) === 'halted' && probeField({ status: 'reported', findings: [] }) === 'absent' && probeField({ status: 'reported', asserted_sha256: null }) === 'absent' && probeField({ status: 'reported', asserted_sha256: 'a'.repeat(64) }) === 'a'.repeat(64) && probeField({ status: 'reported', asserted_sha256: 'x\'; rm -r /' }) === 'malformed' && probeField({ status: 'reported', asserted_sha256: 'A'.repeat(64) }) === 'malformed' && probeField({ status: 'reported', asserted_sha256: 7 }) === 'malformed')
  check('how the tries of a call ended', triesOf([{ ended: 'nothing' }, { ended: 'threw', message: 'm' }, { ended: 'returned' }]) === 'ntr' && triesOf([{ ended: 'returned' }]) === 'r' && triesOf([{ ended: 'nothing' }, { ended: 'nothing' }, { ended: 'nothing' }]) === 'nnn')
  check('the bytes of a text', utf8Length('') === 0 && utf8Length('a\u00e9\u2014\ud83d\ude00') === 1 + 2 + 3 + 4)
  const probeLine = printed({ act: 'state', status: 'read', state: { run: 'probe', door: 'a \u2014 b' } })
  const probeOwn = sha256(JSON.stringify({ act: 'state', status: 'read', state: { run: 'probe', door: 'a \u2014 b' } }))
  const relayed = (text) => probeRelayed({ status: 'ran', line: text }, readStep({ status: 'ran', line: text }, 'state', true))
  check('a relayed line that holds is its own hash, and its bytes', relayed(probeLine) === probeOwn + ':' + utf8Length(probeLine) && relayed(probeLine + '\n') === probeOwn + ':' + utf8Length(probeLine))
  check('a relayed line that does not hold is altered, and its bytes', relayed(probeLine.replace('probe', 'prob')) === 'altered:' + (utf8Length(probeLine) - 1) && relayed('not json') === 'altered:8' && relayed(probeLine.replace('\u2014', '-')) === 'altered:' + (utf8Length(probeLine) - 2))
  check('a probe reads the relay exactly: a line with one value slipped is altered there, though a stage reads it', relayed(slipped) === 'altered:' + utf8Length(slipped) && relay(slipped).status === 'read')
  check('no line came back', probeRelayed(null, null) === 'none:0' && probeRelayed({ status: 'ran' }, readStep({ status: 'ran' }, 'state')) === 'none:0' && probeRelayed({ status: 'halted', halt: { root_cause: 'x' } }, readStep({ status: 'halted', halt: { root_cause: 'x' } }, 'state')) === 'none:0')
  const probeReview = (id) => probeReviewPrompt(id, { file: '/tmp/scratch-1/probe/required/bin/jigc', sha256: 'c'.repeat(64) })
  check('a probe\'s reviewer is handed the binary line a stage\'s reviewer is, and no report', probeReview('b').includes('\nBINARY: candidate `/tmp/scratch-1/probe/required/bin/jigc` sha256 ' + 'c'.repeat(64) + ' (commit ' + '0'.repeat(40) + ', label probe). Return the sha256 you asserted for the candidate as `asserted_sha256`.\n') && !probeReview('b').includes('\n' + LABELS.report) && !probeReview('a').includes('\n' + LABELS.report))
  check('only case a is told to leave the field out', probeReview('a').includes('Leave `asserted_sha256` OUT of your return') && !probeReview('b').includes('OUT of your return') && probeReview('a').replace('case a', 'case b').startsWith(probeReview('b') + '\n'))
  check('the probe\'s schema requires the hash, and a stage\'s does not', PROBE_UNIT_SCHEMA.required.includes('asserted_sha256') && !UNIT_SCHEMA.required.includes('asserted_sha256') && PROBE_UNIT_SCHEMA.properties === UNIT_SCHEMA.properties)
  const probed = PROBE_ENTRIES.map((n) => probeBatch('/tmp/scratch-1', n))
  const probedCalls = (batch) => JSON.parse(batch.write.split('\n')[2])
  check('a probe\'s batch is a stage\'s record of that many findings, under the scratch root', probed.every((batch, i) => batch.file === '/tmp/scratch-1/probe/payload/e' + PROBE_ENTRIES[i] + '/batch.json' && JSON.stringify(probedCalls(batch).map((made) => made.argv[0])) === JSON.stringify(['check-reports', 'ledger-add', 'triage-set', 'check-ledger']) && JSON.parse(probedCalls(batch)[1].stdin).length === PROBE_ENTRIES[i] && JSON.parse(probedCalls(batch)[2].stdin).length === PROBE_ENTRIES[i] && probedCalls(batch)[3].argv.length === 4 + PROBE_ENTRIES[i]))
  check('a probe\'s batch is held to the hash of its text, and grows with its entries', probed.every((batch) => batch.sha256 === sha256(batch.write.split('\n')[2] + '\n') && batch.write.split('\n')[1] === PAYLOAD_ENDS && batch.write.split('\n')[3] === PAYLOAD_ENDS && batch.write.startsWith('Write the file `' + batch.file + '` WITH YOUR FILE TOOL')) && probed.every((batch, i) => i === 0 || batch.write.length > probed[i - 1].write.length) && distinct(probed.map((batch) => batch.sha256)))
  const probePayload = probePayloadPrompt('/tmp/scratch-1', 20, probed[0])
  check('a probe\'s executor writes the payload as a record step does, hashes it, and applies nothing', probePayload.startsWith(LABELS.record + ' — a PROBE') && probePayload.includes('\n' + RECORD_RUNS + '\n1. ' + probed[0].write + '\n2. `dev/stabilize-probe hash --scratch /tmp/scratch-1 --file /tmp/scratch-1/probe/payload/e20/batch.json`') && !probePayload.includes('stabilize-record apply') && !probePayload.includes('dev/gate') && !probePayload.includes('shasum') && !probePayload.includes(LABELS.branch))
  const probeHeld = holdStartPrompt({ scratch: '/tmp/scratch-1' }, 'a', 'probe', '--seconds 720', 'the hold probe\'s long command, case a, of 720 seconds')
  check('a probe\'s long command is held as a stage\'s gate is: started by one plain call of the step tool, and asked after by another', probeHeld.includes('\n1. `dev/stabilize-step hold-start --scratch /tmp/scratch-1 --name a --kind probe --seconds 720 --digest /tmp/scratch-1`\n') && !probeHeld.includes('\n' + LABELS.report) && !/background|slices|2>&1/.test(probeHeld) && holdWaitPrompt({ scratch: '/tmp/scratch-1' }, 'a', 'x').includes('`dev/stabilize-step hold-wait --scratch /tmp/scratch-1 --name a --digest /tmp/scratch-1`'))
  check('a slice of a read is under the tool\'s default timeout, and a hold past its ceiling', PROBE_SLICE < 120 && PROBE_HOLD > 600 && PROBE_HOLD <= PROBE_HOLD_MAX)
  check('no prompt of a probe names a run, a round or a branch', [probeBegin, probeReview('a'), probePayload, probeHeld].every((text) => !text.includes('fix/') && !text.includes(RUNS_ROOT) && !text.includes('--run ') && !text.includes('--round ') && !text.includes('--branch ') && !text.includes('--loop ')))

  return { status: failed.length ? 'self-test-failed' : 'self-test-passed', checks, failed }
}

let parsedArgs = args
if (typeof args === 'string') {
  const s = args.trim()
  if (s.startsWith('{')) {
    try {
      parsedArgs = JSON.parse(s)
    } catch (_) {   }
  }
}
const refusal = validateArgs(parsedArgs)
if (refusal) {
  return { status: 'refused', message: refusal + '. Nothing was run. Usage: Workflow({ scriptPath: \'<the repository\'s root>/.claude/workflows/stabilize.js\', args: { stage: \'test\' | \'fix\', run: \'<run>\', scratch: \'<absolute dir>\' } }) — by the script\'s path, never by its name; the script\'s header has the rest.' }
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

const TRANSIENT_RETRIES = 2
let exhaustedLabels = []
let breakerTripped = false
let triesSeen = null
async function agentR(prompt, opts, again, seen, once) {
  const lbl = (opts && opts.label) ? opts.label : 'agent'
  const tries = once ? 1 : TRANSIENT_RETRIES + 1
  let lastErr
  for (let attempt = 0; attempt < tries; attempt++) {
    if (breakerTripped) {
      log('rate-limit breaker is tripped — NOT spawning ' + lbl + (attempt ? ' again (try ' + (attempt + 1) + ' of ' + tries + ')' : ''))
      return null
    }
    const note = attempt === 0 ? '' : '\n\n' + (again || LOOK_AGAIN)
    if (seen) seen.tries = attempt + 1
    try {
      const result = await agent(prompt + note, Object.assign({ model }, opts))
      if (triesSeen) triesSeen.push({ ended: result != null ? 'returned' : 'nothing' })
      if (result != null) return result
      lastErr = new Error('agent returned null')
      log('null return on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + tries + ')')
    } catch (e) {
      lastErr = e
      if (triesSeen) triesSeen.push({ ended: 'threw', message: String((e && e.message) || e) })
      log('transient failure on ' + lbl + ' (attempt ' + (attempt + 1) + '/' + tries + ') — ' + ((e && e.message) || e))
    }
  }
  if (once) return null
  log('exhausted ' + (TRANSIENT_RETRIES + 1) + ' attempts on ' + lbl + ' (' + ((lastErr && lastErr.message) || lastErr) + ')')
  if (!exhaustedLabels.includes(lbl)) exhaustedLabels.push(lbl)
  if (exhaustedLabels.length >= 2 && !breakerTripped) {
    breakerTripped = true
    log('TWO different agents exhausted their retries (' + exhaustedLabels.join(', ') + ') — the shape of an account-level rate limit, not of a fault in the run. No further agent is spawned.')
  }
  return null
}
function gitStep(label, phaseTitle, prompt, schema, again, seen) {
  return agentR(prompt, { label: 'git:' + label, phase: phaseTitle, agentType: 'build-git', schema, model: GIT_MODEL }, again, seen)
}
async function toolStep(label, phaseTitle, act, prompt, seen) {
  return readDigest(await gitStep(label, phaseTitle, prompt, STEP_SCHEMA, RUN_AGAIN, seen), act, 'git:' + label)
}
async function hold(job) {
  let started = null
  let name = job.name
  let asked = 0
  for (let n = 1; n <= (job.sequence || job.stands ? HOLD_NAMES : 1) && !started; n++) {
    name = job.sequence || (job.stands && n > 1) ? job.name + '-' + n : job.name
    asked = n
    if (!isSlug(name)) return { ends: 'unread', kind: job.kind, why: 'no name of a held command can be made of `' + name + '`: it must be a slug of at most ' + SLUG_MAX + ' characters' }
    const seen = { tries: 0 }
    const at = heldOf(await toolStep('hold-start:' + name, job.phase, 'hold-start', holdStartPrompt(v, name, job.kind, typeof job.flags === 'function' ? job.flags(n) : job.flags, job.what), seen), v.scratch)
    const over = passedOver(job, at, seen.tries)
    if (over) {
      log(over === 'running'
        ? 'the held command `' + name + '` was running before this invocation asked for it — an earlier invocation\'s: it is not read as this one\'s, and the next name is asked'
        : 'the held command `' + name + '` answered its start as one that had ended already (' + at.ends + (at.why ? ', `' + at.why + '`' : '') + ') — this call did not start it, and the tool does not say which call did: ' + (job.sequence ? 'it is not read as the gate of this record\'s tree' : 'it is passed over — a job that stands is the answer only while it runs or where it is green') + ', and the next name is asked')
      continue
    }
    started = at
  }
  if (!started) return { ends: 'unread', name, kind: job.kind, exhausted: true, why: 'every name from `' + job.name + (job.sequence ? '-1' : '') + '` to `' + name + '` answered its start with a command that was there already under this scratch root' + (job.stands ? ', each ended red, void or dead' : '') + ': a name is one command\'s, and none is left' }
  if (started.ends === 'unread') return Object.assign({ name, kind: job.kind }, started, { started: !started.refused })
  let now = started
  for (let lap = 1; now.ends === 'running' && lap <= HOLD_LAPS; lap++) now = Object.assign({ name, kind: job.kind, output: started.output }, heldOf(await toolStep('hold-wait:' + name + (lap > 1 ? ':lap' + lap : ''), job.phase, 'hold-wait', holdWaitPrompt(v, name, job.what)), v.scratch))
  return Object.assign(now, { started: true, asked })
}
async function buildOf(whose, sha, version, what, phaseTitle) {
  const short = sha.slice(0, 12)
  const at = (n) => 'bin/' + whose + '-' + short + (n > 1 ? '-' + n : '') + '/jigc'
  const job = await hold({ name: 'build-' + whose + '-' + short, stands: true, kind: 'build', flags: (n) => '--commit ' + sha + ' --to ' + at(n) + (version ? ' --version ' + version : ''), what, phase: phaseTitle })
  if (!HELD_TAKES.build.includes(job.ends)) return { fault: heldFault(what, job) + ' No instrument runs without the binary.', phase: 'build', held: heldSaid(job), transient: job.ends === 'unread' || job.ends === 'running', then: heldThen(v.scratch, job) }
  const to = at(job.asked)
  const f = job.facts
  if (f.commit !== sha || f.binary !== to || !SHA256_RE.test(String(f.content_sha256 || '')) || f.resolves !== true || (version && f.version !== version)) return { fault: what + ' is not the binary that was asked for: the tool\'s verdict names ' + JSON.stringify({ commit: f.commit, binary: f.binary, version: f.version, resolves: f.resolves }), phase: 'build', held: heldSaid(job) }
  return { binary: v.scratch + '/' + to, sha256: f.content_sha256, version: f.version }
}
async function buildBoth(label, sha, previous, phaseTitle) {
  const built = await buildOf('candidate', sha, null, 'the candidate\'s binary, label ' + label, phaseTitle)
  if (built.fault) return built
  const before = await buildOf('previous', String(previous.commit), previous.version, 'the previous release\'s binary, version ' + previous.version, phaseTitle)
  if (before.fault) return before
  return { candidate: { label, sha, binary: built.binary, sha256: built.sha256 }, previous: { version: before.version, binary: before.binary, sha256: before.sha256 }, image: null }
}
function roleStep(role, label, phaseTitle, prompt, schema, once) {
  return agentR(prompt, { label, phase: phaseTitle, agentType: ROLES[role].agentType, schema }, null, null, once)
}

function halt(phaseName, why, more) {
  const transient = !!(more && more.transient)
  return Object.assign({
    status: 'halted',
    stage: v.stage,
    run: v.run,
    halted: Object.assign({ phase: phaseName, reason: why, transient }, more || {}),
    arrived,
    named: namedOf(lastState),
    message: 'stabilize ' + v.stage + ' of `' + v.run + '` HALTED at ' + phaseName + ': ' + why + (breakerTripped ? ' The rate-limit breaker is tripped (' + exhaustedLabels.join(', ') + '): wait for the window before invoking again.' : transient ? ' An agent returned no result after retries: that is infrastructure, not a verdict about the run.' : '') + ((more && more.then) || ' What was committed stands; a report a reporter wrote and no record step committed is still untracked under ' + runDir(v.run) + '/ and stays there. Once the cause is dealt with, invoke the stage again with the same args: it reads the state as it stands and works the next attempt.'),
  }, queueSaid(lastState))
}
function haltAfter(row, why, more) {
  return halt(HALTS_AFTER[row].phase, why, more)
}
function gitHalt(phaseName, r, what, more) {
  if (!r) return halt(phaseName, what + ': the step returned no result', Object.assign({ transient: true }, more || {}))
  const word = typeof r.refused === 'string' ? r.refused : null
  const said = word ? 'the step refused ' + stepRefusal(word, (more && more.branch) || r.branch || loopBranch) : r.relay || (r.halt && r.halt.root_cause) || 'the step reported ' + JSON.stringify({ status: r.status, branch: r.branch, head: r.head, remote_head: r.remote_head, merge_commit: r.merge_commit })
  return halt(phaseName, what + ': ' + said, Object.assign({ refused: word, whose: word && STEP_REFUSALS[word] ? STEP_REFUSALS[word].whose : null, step: stepFile(r), halt: word || r.relay ? null : r.halt || null }, more || {}))
}
function stepFile(r) {
  const kept = fileOf(r)
  return kept ? { file: v.scratch + '/' + kept.file, sha256: kept.sha256 } : null
}
function onIt(r, branch, pushed) {
  return !!r && r.status === 'ready' && r.branch === branch && SHA_RE.test(String(r.head || '')) && (!pushed || r.remote_head === r.head)
}

let stateReads = 0
let lastState = null
let arrived = null
async function readState() {
  const tag = v.stage + '-' + (++stateReads)
  let why = 'the step returned no result'
  for (let n = 0; n < 3; n++) {
    const again = n === 0 ? '' : '\n\n' + relayAgain(n)
    const r = await toolStep('state:' + tag + (n ? ':again' + n : ''), 'State', 'state', statePrompt(v, tag) + again)
    if (!r) return { error: why, transient: true }
    if (!r.relay) {
      if (r.status !== 'read') return { error: r.refused ? 'the step refused ' + stepRefusal(r.refused) : (r.halt && r.halt.root_cause) || 'the step halted', halt: r.refused ? null : r.halt, step: stepFile(r) }
      if (plain(r.state) && typeof r.state.opened === 'boolean') {
        lastState = Object.assign(r.state, { document: stepFile(r) })
        return { state: lastState, branch: r.branch }
      }
    }
    why = r.relay || 'the relayed line holds no state'
    log('state relay rejected (' + why + ') — asking again')
  }
  return { error: why }
}

async function settleReports(ctx, launch, label, phaseTitle) {
  const read = reportsRead(await toolStep(label, phaseTitle, 'check-reports', checkReportsPrompt(ctx, launch.names)))
  for (const name of read.missing || []) launch.drop(name)
  return read
}
function asideSaid(aside) {
  for (const moved of aside) {
    if (!plain(moved) || typeof moved.path !== 'string') continue
    log('SET ASIDE by the report check: `' + moved.path + '` (`' + String(moved.why) + '`) — the file is not what `' + RECORD_TOOL + '` would have written at its path, and it left the tree for the scratch root: the step\'s own line says where it lies now')
  }
}

function recordFault(r, expect) {
  if (!r) return 'the record\'s commit step returned no result'
  if (r.status !== 'recorded') return r.refused ? 'the record\'s commit step refused ' + stepRefusal(r.refused, r.branch) : r.relay || (r.halt && r.halt.root_cause) || 'the record\'s commit step halted'
  if (!SHA_RE.test(String(r.commit || ''))) return 'the record\'s commit step returned no commit sha'
  if (!plain(r.gate) || r.gate.ok !== true) return 'the record\'s commit step returned no gate check that holds'
  const applied = r.applied
  if (!plain(applied) || applied.calls !== expect.calls || applied.checks !== expect.checks) return 'the batch that was committed is not the one this stage composed: ' + expect.calls + ' call(s) and ' + expect.checks + ' check(s) were composed, and the commit step read ' + JSON.stringify(applied == null ? null : { calls: applied.calls, checks: applied.checks })
  if (applied.failed !== 0) return 'a check of the record does not hold: ' + JSON.stringify(applied.failed == null ? null : applied.failed) + ' of ' + applied.checks + ' — the step\'s own line names each'
  return null
}
function pushHalt(pushed, branch, what) {
  return gitHalt('push', pushed, what, { branch, then: pushThen(branch, pushed && typeof pushed.refused === 'string' ? pushed.refused : null) })
}
let stageHead = null
let stageFence = null
async function recordStep(label, branch, rec) {
  if (rec.text) {
    const applied = await roleStep('record', 'record:' + label, 'Record', rec.text, RECORD_SCHEMA)
    if (!applied) return { fault: 'the record step returned no result', result: null, transient: true, then: recordThen(v.run, branch, null) }
    if (applied.status !== 'applied') return { fault: (applied.halt && applied.halt.root_cause) || 'the record step halted', result: null, halt: applied.halt || null, unchanged: applied.refused === NO_CHANGE, then: applied.refused === NO_CHANGE ? ' Nothing was written and NO BATCH IS LEFT: the record script refused the batch as one that changes nothing (`' + NO_CHANGE + '`) — every file it would write is what the run holds already. No discard is asked for.' : recordThen(v.run, branch, null) }
  }
  const gate = await hold({ name: rec.hold, sequence: true, kind: 'gate', flags: '--run ' + v.run, what: 'the full gate on the tree with the records', phase: 'Record' })
  if (gate.exhausted) return { fault: heldFault('the record\'s gate', gate) + ' Nothing was committed.', result: null, transient: false, at: 'gate', silent: false, held: heldSaid(gate), then: recordThen(v.run, branch, rec.pending ? 'no-name' : 'no-gate') }
  if (!HELD_TAKES.record.includes(gate.ends)) return { fault: heldFault('the record\'s gate', gate) + ' Nothing was committed.', result: null, transient: gate.ends === 'unread' || gate.ends === 'running', at: 'gate', silent: gate.silent === true, held: heldSaid(gate), then: recordThen(v.run, branch, 'no-gate') + (heldThen(v.scratch, gate) || '') }
  const r = await toolStep('record:' + label, 'Record', 'record', recordCommitPrompt(v, branch, gate.output, rec.expect, stageHead, stageFence))
  const found = recordFault(r, rec.expect)
  if (!found) {
    if (stageFence) stageFence = SHA256_RE.test(String(r.fence)) ? String(r.fence) : null
    return { result: r }
  }
  const fault = found.replace('<branch>', branch)
  const word = r && typeof r.refused === 'string' ? r.refused : null
  return { fault, result: r, transient: !r, at: 'commit', silent: !r, refused: word, step: stepFile(r), gate: r && plain(r.gate) ? r.gate : null, then: recordThen(v.run, branch, word, r ? r.discarded : null) }
}
async function composeRecord(label, branch, ctx, dir, subject, words) {
  let c = null
  for (let n = 0; n <= WORDS_AGAIN; n++) {
    c = await toolStep('compose:' + label + (n ? ':again' + n : ''), 'Record', 'compose', composePrompt(ctx, branch, subject, words) + (n ? '\n\n' + wordsAgain(n) : ''))
    const row = c && c.status === 'halted' && c.refused === 'record' ? COMPOSE_REFUSALS[c.refusal] : null
    if (!row || !row.again) break
    log('the composing step was refused `' + c.refusal + '` — ' + row.says + (n < WORDS_AGAIN ? ': the step is asked for again, exactly' : ': it was asked for again already'))
  }
  if (!c) return { fault: 'the step that composes the record returned no result', transient: true, then: recordThen(v.run, branch, null) }
  if (c.status === 'applied' && plain(c.applied) && Number.isInteger(c.applied.calls) && Number.isInteger(c.applied.checks)) return { rec: { text: null, hold: recordHold(dir), expect: { calls: c.applied.calls, checks: c.applied.checks } } }
  const word = typeof c.refused === 'string' ? c.refused : null
  const said = word === 'record' && typeof c.refusal === 'string' ? c.refusal : null
  const row = said ? COMPOSE_REFUSALS[said] || { says: 'a word of `' + RECORD_TOOL + '` this script has no row for: the step\'s own line holds that script\'s refusal', then: null } : null
  if (row) return { fault: 'the record was not composed: `' + RECORD_TOOL + '` refused the composing step `' + said + '` — ' + row.says + scanHits(c.hits) + '. Nothing was written', refused: word, refusal: said, step: stepFile(c), then: row.then === 'pending' ? recordThen(v.run, branch, null) : row.then }
  if (word) return { fault: 'the step that composes the record refused ' + stepRefusal(word, branch), refused: word, refusal: null, step: stepFile(c), then: recordThen(v.run, branch, null) }
  return { fault: 'the step that composes the record did not answer with a batch it applied: ' + (c.relay || (c.halt && c.halt.root_cause) || 'it said `' + c.status + '`'), refused: null, refusal: null, step: stepFile(c), halt: c.relay ? null : c.halt || null, then: recordThen(v.run, branch, null) }
}
function recordHalt(phaseName, recorded, more) {
  return halt(phaseName, recorded.fault, Object.assign({ transient: !!recorded.transient, refused: recorded.refused || null, whose: recorded.refused && STEP_REFUSALS[recorded.refused] ? STEP_REFUSALS[recorded.refused].whose : null, step: recorded.step || null, gate: recorded.gate || null, held: recorded.held || null, halt: recorded.halt || null, then: recorded.then }, more || {}))
}
async function pushRecord(label, branch) {
  const pushed = await toolStep(label, 'Record', 'push', pushPrompt(v, branch, branch === loopBranch))
  if (onIt(pushed, branch, true) && stageHead) stageHead = String(pushed.head)
  return pushed
}
async function finishRecord(gs) {
  phase('Record')
  const pending = gs.pending
  log('a record step left its batch applied and not committed on ' + gs.branch + ' — ' + pending.calls + ' call(s), ' + pending.files + ' file(s): this invocation gates and commits it, and starts nothing')
  const recorded = await recordStep('pending', gs.branch, pendingRecord(v.scratch + '/record/pending', pending))
  if (recorded.fault) return recordHalt('record', recorded, { pending })
  const pushed = await pushRecord('push:pending', gs.branch)
  if (!onIt(pushed, gs.branch, true)) return pushHalt(pushed, gs.branch, 'the pending record is committed on ' + gs.branch + ' (' + recorded.result.commit + ') and the branch was not pushed')
  const read = await readState()
  if (!read.state) return halt('state', 'the pending record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient, step: read.step || null })
  return Object.assign({ status: 'recorded', stage: v.stage, run: v.run, pending, record: recorded.result.commit, gate: recorded.result.gate, arrived, message: 'a record step of an earlier invocation had applied its batch and not committed it: this invocation ran the gate on it, committed and pushed it, and did nothing else — `next` names the step.' }, nextOf(read.state))
}

async function beginAttempt(ctx, launch, sha) {
  launch.add([ATTEMPT])
  const begun = await toolStep('begin', 'State', 'begin', beginPrompt(ctx, sha))
  if (!begun || begun.status !== 'begun' || String(begun.attempt) !== String(ctx.attempt)) return gitHalt('begin', begun, 'attempt ' + ctx.attempt + ' of the `' + ctx.stage + '` stage, round ' + ctx.round + ', was not put on record, and no agent of it was launched')
  return null
}

function findingLines(findings) {
  return findings.map((f) => f.id + ' — ' + f.title + ' · door: ' + f.door + ' · clause: ' + f.clause + (f.severity ? ' · severity: ' + f.severity : '') + (f.lead ? ' · a lead' : '') + ' · repro: ' + f.repro)
}
function leftOpenSource(reporter, report, leftOpen) {
  return { reporter, report, findings: (leftOpen || []).map((text, i) => 'left open ' + (i + 1) + ' — ' + text) }
}

async function runUnits(ctx, launch, built, units, phaseTitle) {
  const out = await parallel(units.map((unit) => async () => {
    const done = []
    let reason = null
    let word = null
    let crossModel = null
    for (const stepList of unit.chain) {
      const named = stepList.map((step) => ({ step, name: launch.add([unit.item, step.as]) }))
      const promptOf = (step, name) => (step.crossModel ? crossModelPrompt(ctx, name, unit) : unitPrompt(ctx, launch, name, step, unit, built, (step.hands || []).map((h) => ({ as: h, report: (done.find((d) => d.as === h) || {}).report }))))
      const results = await parallel(named.map(({ step, name }) => () => roleStep(step.role, unit.item + ':' + step.as, phaseTitle, promptOf(step, name), UNIT_SCHEMA)))
      named.forEach(({ step, name }, i) => {
        const r = results[i]
        const reported = !!r && r.status === 'reported'
        const unheld = reported ? unasserted(step.role, name, r, built.candidate.sha256) : null
        if (step.crossModel) crossModel = reported ? 'ran' : 'void'
        else if (!reported && !reason) [reason, word] = ['its `' + step.as + '` step ' + endingOf(r), voidWord(r ? 'halted' : 'silent', name)]
        else if (unheld && !reason) [reason, word] = ['its `' + step.as + '` step ' + unheld, voidWord('unasserted', name)]
        done.push({ as: step.as, name, drives: ROLES[step.role].drives, crossModel: !!step.crossModel, result: reported || !step.crossModel ? r : null, report: r && r.report ? r.report : null })
      })
      if (reason) break
    }
    return { item: unit.item, clause: unit.clause, status: reason ? 'void' : 'green', reason, word, crossModel, reporters: done }
  }))
  return units.map((unit, i) => out[i] || { item: unit.item, clause: unit.clause, status: 'void', reason: 'its chain did not run', word: voidWord('chain-lost'), crossModel: null, reporters: [] })
}
async function triagePasses(ctx, launch, built, sources, forksIn) {
  const entries = []
  const forks = forksIn.slice()
  const unverified = []
  const unreportedBy = []
  let pending = sources.filter((s) => findingsOf(s))
  let handed = 0
  for (let pass = 1; pending.length; pass++) {
    const count = pending.reduce((n, s) => n + findingsOf(s), 0)
    handed += count
    const tName = launch.add(['triage', 'p' + pass])
    log('triage pass ' + pass + ': ' + count + ' finding(s) from ' + pending.length + ' reporter(s)')
    const t = await roleStep('triage', 'triage:p' + pass, 'Triage', triagePrompt(ctx, launch, tName, pending, pass), TRIAGE_SCHEMA)
    if (!t || t.status !== 'graded') return { fault: t ? 'triage halted: ' + ((t.halt && t.halt.root_cause) || 'no reason given') : 'triage returned no result', transient: !t, halt: t ? t.halt : null }
    const counted = (t.counts.findings_in || []).reduce((n, c) => n + c.count, 0)
    if (counted !== count || t.entries.length + t.counts.merged !== count) return { fault: 'triage pass ' + pass + ' was handed ' + count + ' finding(s) and accounts for ' + counted + ' in, ' + t.entries.length + ' entries and ' + t.counts.merged + ' merged: a finding in no entry and in no named merge is lost, and nothing is recorded on that' }
    const strangers = t.entries.filter((e) => !isSlug(e.key)).map((e) => JSON.stringify(e.key))
    if (strangers.length) return { fault: 'triage returned the key ' + strangers.join(', ') + ', which is not a ledger key' }
    const twice = t.entries.map((e) => e.key).filter((key, i, all) => all.indexOf(key) !== i)
    if (twice.length) return { fault: 'triage pass ' + pass + ' returned more than one entry under the key ' + twice.filter((key, i) => twice.indexOf(key) === i).join(', ') + ': one finding has one entry, and a merge is named — nothing is recorded on that' }
    for (const e of t.entries) {
      const held = entries.find((x) => x.key === e.key)
      if (held) Object.assign(held, e, { new: held.new })
      else entries.push(Object.assign({}, e))
    }
    pending = []
    const verified = []
    const driven = []
    const toVerify = pass > VERIFY_PASSES ? [] : t.entries.filter((e) => e.grade === 'breaks' || e.grade === 'unclear')
    if (pass > VERIFY_PASSES) log('NOT VERIFIED: pass ' + pass + ' is past the ' + VERIFY_PASSES + ' passes that are verified — its entries get their rows, and those graded breaks or unclear stay unverified: the state\'s `next` then says that the round\'s triage is not finished (`triage`), and names them')
    if (toVerify.length) {
      const hints = {}
      for (const h of t.to_verify || []) hints[h.key] = h.redrive
      const unnamed = toVerify.filter((e) => !forkNames('p' + pass, e.key)).map((e) => e.key)
      if (unnamed.length) return { fault: 'no reporter name can be made for the verifier of ' + unnamed.join(', ') + ': the key is too long to carry a prefix inside ' + SLUG_MAX + ' characters' }
      const named = toVerify.map((e) => ({ e, name: launch.add(['verify', 'p' + pass, e.key]) }))
      log('verify-real, pass ' + pass + ': ' + named.length + ' finding(s)')
      const verdicts = await parallel(named.map(({ e, name }) => () => roleStep('verify', 'verify:' + e.key, 'Triage', verifyPrompt(ctx, launch, name, e, hints[e.key], built), VERIFY_SCHEMA)))
      named.forEach(({ e, name }, i) => {
        const r = verdicts[i]
        const noVerdict = r && r.status === 'verified' ? lacks('verify', r).find((n) => n.field === 'verdict') : null
        if (!r || r.status !== 'verified' || noVerdict || r.key !== e.key) {
          unverified.push({ key: e.key, why: 'its verifier ' + (!r || r.status !== 'verified' ? endingOf(r) : noVerdict ? lacksWhy(name, noVerdict) : 'returned a verdict for `' + r.key + '`, another finding') })
          return
        }
        const unheld = unasserted('verify', name, r, built.candidate.sha256)
        verified.push({ e, name, r, unheld })
      })
      for (const { e, r } of verified.filter((v) => !v.unheld && v.r.contested)) driven.push(await driveFork(ctx, launch, built, { key: e.key, kind: 'contested', door: e.door, clause: e.clause, repro: e.repro, statement: 'the verifier found the finding to contest a settled decision — ' + (r.basis || '(no basis returned)') }, 'p' + pass))
    }
    const settled = await settleReports(ctx, launch, 'check-reports:p' + pass, 'Triage')
    if (settled.fault) return { fault: settled.fault, transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null }
    for (const name of settled.missing) unreportedBy.push(name)
    if (settled.missing.includes(tName)) return { fault: 'triage pass ' + pass + ' returned its grades and left no report: nothing is recorded on a grader\'s word with no report behind it' }
    for (const { e, name, r, unheld } of verified) {
      const fork = driven.find((d) => d.fork.key === e.key)
      const lacking = settled.missing.includes(name) ? 'its verifier returned a verdict and left no report'
        : unheld ? 'its verifier ' + unheld
          : !fork ? null
            : !fork.argued || settled.missing.includes(fork.aName) ? 'it is contested, and its advocate ' + (fork.argued ? 'left no report' : endingOf(fork.fork.advocate))
              : fork.unheld ? 'it is contested, and ' + fork.unheld
                : !fork.drove || settled.missing.includes(fork.pName) ? 'it is contested, and the independent drive of the advocate\'s proposal ' + (fork.drove ? 'left no report' : endingOf(fork.fork.independent_drive))
                  : null
      if (!settled.missing.includes(name) && r.left_open && r.left_open.length) pending.push(leftOpenSource(name, r.report, r.left_open))
      for (const source of fork ? fork.leftOpen.filter((x) => !settled.missing.includes(x.reporter)) : []) pending.push(source)
      if (lacking) {
        unverified.push({ key: e.key, why: lacking })
        continue
      }
      const entry = entries.find((x) => x.key === e.key)
      entry.verdict = r.verdict
      if (r.verdict === 'confirmed') {
        const fact = regressionOf(name, r, built.previous.sha256)
        entry.regression = fact.regression
        if (fact.unknown) log('REGRESSION UNKNOWN: `' + e.key + '` is confirmed, and its verifier ' + fact.unknown + '. It is recorded as confirmed with that fact unknown — never as not a regression — and the state names it')
      }
      entry.basis = r.basis
      if (fork) {
        entry.fork = { kind: fork.fork.kind, case: fork.fork.advocate.verdict, drive: fork.fork.independent_drive.holds ? 'holds' : 'differs' }
        forks.push(fork.fork)
      }
    }
    if (pass > VERIFY_PASSES) break
  }
  for (const left of unverified) log('NOT VERIFIED: `' + left.key + '` — ' + left.why + '. It stays unverified: the state counts the pass, and after one more the finding is the human\'s')
  return { entries, forks, handed, unverified, unreportedBy }
}

async function triageOnce(ctx, launch, sources) {
  const handed = sources.filter((s) => findingsOf(s))
  const count = handed.reduce((n, s) => n + findingsOf(s), 0)
  if (!handed.length) return { triage: null, handed: 0 }
  const tName = launch.add(['triage'])
  log('the stage\'s one triage: ' + count + ' finding(s) from ' + handed.length + ' source(s) — ' + handed.map((s) => s.reporter + '=' + findingsOf(s)).join(' '))
  const t = await roleStep('triage', 'triage', 'Triage', oneTriagePrompt(ctx, launch, tName, handed), FILED_SCHEMA)
  if (!t || t.status !== 'filed') return { fault: t ? 'triage halted: ' + ((t.halt && t.halt.root_cause) || 'no reason given') : 'triage returned no result', transient: !t, halt: t ? t.halt : null }
  const unheld = filedFault(ctx, tName, handed, t)
  if (unheld) return { fault: unheld }
  const settled = await settleReports(ctx, launch, 'check-reports:triage', 'Triage')
  if (settled.fault) return { fault: settled.fault, transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null }
  if (settled.missing.includes(tName)) return { fault: 'triage returned the path and the hash of its rendered entries, and the report check finds no report of it there' + (setAside(settled.aside, tName) ? ' — the file at that path is not the text its entries render (`' + setAside(settled.aside, tName) + '`), and was set aside' : '') + ': nothing is recorded on a return with no file behind it' }
  return { triage: { reporter: tName, report: t.report, sha256: t.sha256 }, handed: count }
}

async function verifyWaves(ctx, launch, built, from) {
  const out = { state: from, waves: 0, launched: 0, unverified: 0, unheld: 0, contested: 0, silent: 0, last: null, filings: [], unconfirmed: [] }
  const seen = []
  const unverified = (key, why) => {
    out.unverified++
    log('NOT VERIFIED: `' + key + '` — ' + why + '. It awaits its verdict again, and nothing of the budget was spent on it')
  }
  for (;;) {
    const queue = out.state.queue
    const room = queue.left == null ? WAVE : Math.min(WAVE, queue.left)
    const named = []
    for (const key of queue.first || []) {
      if (named.length >= room) break
      if (seen.includes(key)) continue
      seen.push(key)
      const name = reporterName(['verify', key])
      if (name && !launch.names.includes(name)) named.push({ key, name: launch.add(['verify', key]) })
      else unverified(key, 'no verifier can be named for it: a verdict is filed under the reporter `verify-<key>`, which for this key is no slug of at most ' + SLUG_MAX + ' characters, or is a name this attempt has launched already')
    }
    if (!named.length) return out
    out.waves++
    out.launched += named.length
    log('verify-real, wave ' + out.waves + ': ' + named.length + ' of the ' + queue.length + ' finding(s) the queue holds — ' + (queue.left == null ? 'the run holds no budget' : queue.left + ' verdict(s) of the budget left'))
    const back = await parallel(named.map(({ key, name }) => () => roleStep('verify', 'verify:' + key, 'Verify', verdictPrompt(ctx, launch, name, key, built, queue.hand_on), VERDICT_SCHEMA, true)))
    const silent = back.filter((r) => r == null).length
    out.silent += silent
    if (silent >= SILENT_LAST) out.last = { wave: out.waves, silent }
    const returned = named.filter(({ name }, i) => back[i] && back[i].status === 'filed' && !verdictFault(ctx, name, back[i], built.candidate.sha256)).map(({ name }) => verdictFile(ctx, name))
    const settled = await settleReports(ctx, launch, 'check-reports:wave' + out.waves, 'Verify')
    if (settled.fault) return Object.assign(out, { fault: settled.fault, transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null, unconfirmed: returned })
    const read = await readState()
    if (!read.state) return Object.assign(out, { unread: true, fault: read.error, transient: !!read.transient, step: read.step || null, unconfirmed: returned })
    out.state = read.state
    named.forEach(({ key, name }, i) => {
      const r = back[i]
      const filed = !!r && r.status === 'filed'
      const moved = setAside(settled.aside, name)
      const ended = !filed ? endingOf(r)
        : settled.missing.includes(name) ? (moved ? 'left a file at its report\'s path that is no report (`' + moved + '`), which was set aside' : 'returned and left no report')
          : verdictFault(ctx, name, r, built.candidate.sha256)
      if ((out.state.queue.first || []).includes(key)) unverified(key, 'its verifier ' + (ended || 'returned the path and the word of a filing') + ', and the queue prints the finding again: no verdict of it counts')
      else if (ended) {
        out.unheld++
        log('NOT HELD: `' + key + '` — its verifier ' + ended + ' — and the queue does not print the finding again: the state says where it stands, and no return decides it')
      } else {
        out.filings.push(verdictFile(ctx, name))
        if (r.contested) {
          out.contested++
          log('CONTESTED, BY ITS VERIFIER\'S RETURN: `' + key + '` — where its filing gave a basis the finding is the human\'s, and the state\'s list of what is the human\'s names it with that basis; where that list does not name it so, the filing gave none, and the finding is routed by its verdict. This script reads no filing, and cannot tell the two apart')
        }
      }
    })
    if (out.last) {
      log('THE LAST WAVE: ' + silent + ' verifier call(s) of wave ' + out.waves + ' returned nothing — each was tried once, and none is tried again. That is how the account\'s usage limit shows, which never reaches this script as a message, and it is also what ' + silent + ' stray deaths look like: no further wave is launched, though the queue holds ' + out.state.queue.length + ' finding(s) and ' + (out.state.queue.left == null ? 'the run holds no budget' : out.state.queue.left + ' verdict(s) of the budget are left') + '. What was filed is recorded now, and the invocation that follows continues')
      return out
    }
  }
}

async function verifyThenRecord(ctx, launch, built, from, made) {
  phase('Verify')
  const spent = from.queue.spent
  const ver = await verifyWaves(ctx, launch, built, from)
  const limited = (h, step, silent, record) => {
    if (!ver.last || !silent) return h
    const said = limitSaid(ctx, ver.last, ver.state.queue.spent, ver.filings, ver.unconfirmed, step, record || null)
    return Object.assign(h, { halted: Object.assign(h.halted, said.more), message: h.message + said.says })
  }
  if (ver.fault && ver.unread) return { halted: limited(haltAfter('read-back', made.says + ', wave ' + ver.waves + ' of its verifiers ran, and the queue could not be read after it: ' + ver.fault + '. What a verifier filed is on disk, and the state counts it', { transient: !!ver.transient, step: ver.step, launched: launch.names }), 'queue', !!ver.transient) }
  if (ver.fault) return { halted: limited(haltAfter('report-check', 'after wave ' + ver.waves + ' of the stage\'s verifiers ' + ver.fault + '. ' + made.says.charAt(0).toUpperCase() + made.says.slice(1) + ', and what a verifier filed is on disk', { transient: !!ver.transient, step: ver.step, launched: launch.names }), 'report-check', !!ver.transient) }
  let state = ver.state
  let second = null
  const records = made.commit ? ver.launched > 0 : made.owed || launch.names.some((name) => name !== ATTEMPT)
  if (!records && !made.commit) log('NOTHING TO RECORD: no verifier of this invocation left a report, and no filed verdict is owed a record — so no record is composed, and attempt ' + ctx.attempt + ' is one that left its marker and no record. The record script counts those: after two in a row the stage is the human\'s, who grants one more')
  if (records) {
    phase('Record')
    if (!ver.waves) {
      const settled = await settleReports(ctx, launch, 'check-reports:filed', 'Record')
      if (settled.fault) return { halted: limited(haltAfter('report-check', 'before the record of what was filed — no verifier was launched, so no wave\'s report check had come before it — ' + settled.fault + '. Nothing was composed, and what was filed is on disk', { transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null, launched: launch.names }), 'report-check', false) }
      asideSaid(settled.aside)
    }
    const again = await composeRecord('verdicts:r' + ctx.round, loopBranch, ctx, v.scratch + '/record/verdicts-r' + ctx.round + '-a' + ctx.attempt, 'docs(record): ' + subjectOf(v.run, ctx.round, 'the verdicts of the test stage'), launch.names.map((name) => 'reporter=' + name))
    if (again.fault && again.refusal !== NO_CHANGE) return { halted: limited(recordHalt('record', again, { refusal: again.refusal || null, launched: launch.names, first: made.commit }), 'compose', !!again.transient) }
    if (again.fault) log('NOTHING TO RECORD after the verifiers: the record script refused the record as one that changes nothing (`' + NO_CHANGE + '`) — no verdict was filed since the record before it, and no batch is left. That is no halt')
    else {
      const taken = await recordStep('verdicts:r' + ctx.round, loopBranch, again.rec)
      if (taken.fault) return { halted: limited(recordHalt('record', taken, { launched: launch.names, first: made.commit }), taken.at, taken.silent === true) }
      const published = await pushRecord('push:verdicts', loopBranch)
      if (!onIt(published, loopBranch, true)) return { halted: limited(pushHalt(published, loopBranch, 'the record of what the verifiers filed is committed on ' + loopBranch + ' (' + taken.result.commit + ') and the branch was not pushed'), 'push', !published, taken.result.commit) }
      const after = await readState()
      if (!after.state) return { halted: limited(haltAfter('read-back', 'the record of what the verifiers filed is committed and pushed (' + taken.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, step: after.step || null }), 'read-back', !!after.transient, taken.result.commit) }
      state = after.state
      second = taken.result.commit
    }
  }
  return { state, second, verify: { waves: ver.waves, launched: ver.launched, verdicts: state.queue.spent - spent, record: second, unverified: ver.unverified, unheld: ver.unheld, contested: ver.contested, silent: ver.silent, last: ver.last ? ver.last.wave : null } }
}

async function continueTest(ctx, state, words) {
  const label = 'c' + ctx.round
  const said = '(`next` is `' + state.next + '`): ' + words.map((word) => CONTINUES[word]).join(', and ')
  const why = continueFault(v)
  if (why) return { status: 'refused', stage: 'test', run: v.run, message: 'round ' + ctx.round + ' is tested and its `test` stage is not finished ' + said + ' — ' + why + '. Nothing was run beyond the two reads.', next: state.next, untriaged: state.untriaged, position: state.position.test, named: namedOf(state), arrived }
  const tested = String((state.rounds.find((r) => r.round === ctx.round) || {}).candidate || '')
  if (!SHA_RE.test(tested)) return halt('state', 'the state says that round ' + ctx.round + '\'s `test` stage is to be continued and names no candidate for that round: a verdict is of the candidate its round tested, and of no other commit')
  log('round ' + ctx.round + ', attempt ' + ctx.attempt + ' of the test stage: A CONTINUATION ' + said + ' — on ' + tested + ', the candidate the round tested. No instrument runs; the verdicts that are filed stand, and only what is missing is launched')
  if (v.stopAfter === 'state') return { status: 'stopped', after: 'state', stage: 'test', run: v.run, round: ctx.round, attempt: ctx.attempt, continues: words, candidate: { label, sha: tested }, owed: arrived.owed, state: attached(state), named: namedOf(state), arrived }
  const launch = launcher(ctx)
  const unbegun = await beginAttempt(ctx, launch, tested)
  if (unbegun) return unbegun
  const built = await buildBoth(label, tested, previousOf(state), 'State')
  if (built.fault) return halt(built.phase, built.fault, { transient: !!built.transient, held: built.held || null, then: built.then })
  let made = { commit: null, owed: words.includes('filed'), says: 'the stage\'s first record stands, committed by an earlier invocation' }
  let once = { triage: null, handed: 0 }
  if (words.includes('triage')) {
    phase('Triage')
    once = await triageOnce(ctx, launch, ledgerSource(v.run, state))
    if (once.fault) return haltAfter('triage', once.fault, { transient: !!once.transient, halt: once.halt || null, step: once.step || null })
    if (once.triage) {
      phase('Record')
      const composed = await composeRecord('graded:r' + ctx.round, loopBranch, ctx, v.scratch + '/record/graded-r' + ctx.round + '-a' + ctx.attempt, 'docs(record): ' + subjectOf(v.run, ctx.round, 'the rows nobody had graded'), launch.names.map((name) => 'reporter=' + name))
      if (composed.fault) return recordHalt('record', composed, { refusal: composed.refusal || null, launched: launch.names, triage: once.triage })
      const recorded = await recordStep('graded:r' + ctx.round, loopBranch, composed.rec)
      if (recorded.fault) return recordHalt('record', recorded, { launched: launch.names, triage: once.triage })
      const pushed = await pushRecord('push:graded', loopBranch)
      if (!onIt(pushed, loopBranch, true)) return pushHalt(pushed, loopBranch, 'the record of the rows this invocation graded is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
      const read = await readState()
      if (!read.state) return haltAfter('read-back', 'the record of the rows this invocation graded is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient, step: read.step || null })
      state = read.state
      made = { commit: recorded.result.commit, says: 'the record of the rows this invocation graded is committed and pushed (' + recorded.result.commit + ')' }
    }
  }
  const closing = await verifyThenRecord(ctx, launch, built, state, made)
  if (closing.halted) return closing.halted
  state = closing.state
  return Object.assign({
    status: statusOf(words, closing.second),
    stage: 'test',
    run: v.run,
    round: ctx.round,
    continued: words,
    candidate: built.candidate,
    record: made.commit,
    triage: once.triage,
    verify: closing.verify,
    counts: { reporters: launch.names.length, findings_in: once.handed, blockers: state.blockers, for_the_human: state.human_list },
    forbids_close: state.forbids_close,
  }, nextOf(state))
}

async function driveFork(ctx, launch, built, fork, tag) {
  const aName = launch.add(['advocate', tag, fork.key])
  const adv = await roleStep('advocate', 'advocate:' + fork.key, 'Triage', advocatePrompt(ctx, launch, aName, fork, built), ADVOCATE_SCHEMA)
  const out = { fork: Object.assign({}, fork, { advocate: adv, independent_drive: null, driven_by: null }), aName, pName: null, argued: !!adv && adv.status === 'argued', drove: false, leftOpen: [], unheld: null }
  if (!out.argued) return out
  if (adv.left_open && adv.left_open.length) out.leftOpen.push(leftOpenSource(aName, adv.report, adv.left_open))
  const noCase = unasserted('advocate', aName, adv, built.candidate.sha256)
  if (noCase) {
    out.unheld = 'its advocate ' + noCase
    return out
  }
  out.pName = launch.add(['proposal', tag, fork.key])
  const drive = await roleStep('proposal', 'proposal:' + fork.key, 'Triage', proposalPrompt(ctx, out.pName, fork, adv, built), PROPOSAL_SCHEMA)
  out.fork.independent_drive = drive
  out.fork.driven_by = ROLES.proposal.agentType
  out.drove = !!drive && drive.status === 'driven'
  if (out.drove) {
    const noDrive = unasserted('proposal', out.pName, drive, built.candidate.sha256)
    if (noDrive) out.unheld = 'the independent drive of the advocate\'s proposal ' + noDrive
    if (drive.left_open && drive.left_open.length) out.leftOpen.push(leftOpenSource(out.pName, drive.report, drive.left_open))
  }
  return out
}

async function preflightOf(ctx, launch, parts, plan, phaseTitle) {
  const name = launch.add(parts)
  const r = await roleStep('preflight', name, phaseTitle, preflightPrompt(ctx, launch, name, plan), PREFLIGHT_SCHEMA)
  if (!r || r.status !== 'ready') return { name, fault: r ? 'the preflight halted: ' + ((r.halt && r.halt.root_cause) || 'no reason given') : 'the preflight returned no result', transient: !r, halt: r ? r.halt : null }
  if (plan.image && !(r.image && r.image.verified && r.image.tag)) return { name, fault: 'the trial image is not verified: ' + JSON.stringify(r.image || null) }
  return { name, result: r }
}

function attached(state) {
  return { next: state.next, stop: state.stop, not_ready: state.not_ready, candidate: state.candidate, fix_rounds: state.fix_rounds, evidence: evidenceOf(state), ledger: state.ledger, blockers: state.blockers, human_list: state.human_list, reverify: state.reverify, untriaged: state.untriaged, retest: state.retest, human_clauses: state.human_clauses, human_stages: state.human_stages, unsettled: state.unsettled, forbids_close: state.forbids_close, position: state.position }
}
function nextOf(state) {
  const o = outcomeOf(state)
  const named = namedOf(state)
  const queued = queueSaid(state)
  if (!o.known) return Object.assign({ next: o.next, returned_to_orchestrator: true, state: attached(state), named, arrived }, queued)
  if (o.rule) return Object.assign({ next: o.next, rule: o.rule, named, arrived }, queued)
  return Object.assign(o.next === 'close' ? { next: o.next, close: closeOf(), evidence: evidenceOf(state), named, arrived } : { next: o.next, named, arrived }, queued)
}

async function rulingsStep(round, branch, ran, at) {
  phase('Record')
  const ruled = await recordStep('rulings:r' + round, branch, rulingsRecordPrompt(v, round, branch, v.rulings, ran, at))
  if (ruled.unchanged) return { halted: { status: 'refused', stage: v.stage, run: v.run, refused: { refused: NO_CHANGE }, halt: ruled.halt, arrived, named: namedOf(lastState), message: 'the rulings are on record already: the record script refused their batch as one that changes nothing (`' + NO_CHANGE + '`).' + ruled.then + ' Nothing was recorded, committed or pushed.' } }
  if (ruled.fault) return { halted: recordHalt('rulings', ruled) }
  const pushed = await pushRecord('push:rulings', branch)
  if (!onIt(pushed, branch, true)) return { halted: pushHalt(pushed, branch, 'the rulings are recorded on ' + branch + ' (' + ruled.result.commit + ') and the branch was not pushed') }
  const read = await readState()
  if (!read.state) return { halted: halt('state', 'the rulings are recorded (' + ruled.result.commit + '), and the state could not be read back: ' + read.error, { transient: !!read.transient, step: read.step || null }) }
  return { state: read.state, record: ruled.result.commit }
}

async function ruleTheRun(state, checkedOut) {
  const told = { status: 'refused', stage: v.stage, run: v.run, next: state.next, stop: state.stop, human_clauses: state.human_clauses, human_stages: state.human_stages, reverify: state.reverify, named: namedOf(state), arrived }
  if (checkedOut !== loopBranch) return Object.assign(told, { message: 'an invocation that only records rulings records them on the loop branch ' + loopBranch + ', and `' + checkedOut + '` is checked out. Nothing was run beyond the two reads.' })
  const about = v.rulings.every(runRuling)
  const why = about ? runRulingsFault(v.rulings, state) : null
  if (why) return Object.assign(told, { message: why + '. Nothing was recorded.' })
  const ran = {}
  for (const c of state.clauses || []) ran[c.clause] = (state.items || []).filter((i) => i.clause === c.clause && i.rerun === 'spent').map((i) => i.round).filter((n, at, all) => all.indexOf(n) === at)
  const stages = {}
  for (const h of state.human_stages || []) stages[h.stage] = h.round
  if (hasQueue(state)) {
    phase('Record')
    const vetted = await toolStep('check-reports:rulings', 'Record', 'check-reports', checkReportsPrompt(v, null))
    if (!vetted || vetted.status !== 'vetted' || !Array.isArray(vetted.aside)) return gitHalt('reports', vetted, 'the rulings were not recorded: the report check\'s vet, which comes before their batch, did not answer with what it set aside — nothing was applied')
    log('the report check\'s vet, before the batch of the rulings: ' + (Number.isInteger(vetted.vetted) ? vetted.vetted : 'the') + ' pending file(s) of the run vetted, ' + vetted.aside.length + ' set aside')
    asideSaid(vetted.aside)
  }
  const ruled = await rulingsStep((about && state.stop ? state.stop.round : state.round) || 0, loopBranch, ran, { round: state.round, stages })
  if (ruled.halted) return ruled.halted
  return Object.assign({ status: 'ruled', stage: v.stage, run: v.run, rulings: v.rulings, record: ruled.record, message: 'the human\'s rulings ' + (about ? 'about the run' : 'on the round\'s findings and bounds') + ' are recorded, and nothing was started: `next` names the step.' }, nextOf(ruled.state))
}

async function finishTriage(ctx, launch, state, tip, branch) {
  phase('Triage')
  const sources = ledgerSource(v.run, state)
  log('round ' + ctx.round + ': the ' + ctx.stage + ' stage left its triage unfinished — ' + findingsOf(sources[0] || { findings: [] }) + ' row(s) are graded and verified now, attempt ' + ctx.attempt + '; no instrument runs')
  const pre = await preflightOf(ctx, launch, ['preflight'], { image: false, sha: tip.sha, label: tip.label, branch, checks: [], crossModel: false, tested: tip.tested }, 'Triage')
  if (pre.fault) return { halted: halt('preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, branch }) }
  const built = await buildBoth(tip.label, tip.sha, previousOf(state), 'Triage')
  if (built.fault) return { halted: halt(built.phase, built.fault, { transient: !!built.transient, held: built.held || null, branch, then: built.then }) }
  const tri = await triagePasses(ctx, launch, built, sources, [])
  if (tri.fault) return { halted: haltAfter('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, step: tri.step || null, branch }) }
  if (tri.unreportedBy.includes(pre.name)) return { halted: haltAfter('stood-on', 'the preflight of this lap returned and left no report: what was driven on its binaries has nothing on record behind it', { branch, launched: launch.names }) }
  phase('Record')
  const rec = triageRecord(ctx, tri.entries)
  const dir = v.scratch + '/record/triage-r' + ctx.round + (ctx.stage === 'fix' ? '-c' + ctx.cycle : '') + '-a' + ctx.attempt
  const commands = stageRecordCommands(ctx, { reporters: launch.names, rows: rec.rows, triage: rec.triage, patches: [], clauses: [], facts: null, keys: tri.entries.map((e) => e.key) })
  const recorded = await recordStep('triage:r' + ctx.round, branch, recordPrompt(v, 'round ' + ctx.round + '\'s triage, finished — ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s) graded; no instrument ran', branch, dir, ctx.round, commands, subjectOf(v.run, ctx.round, 'the triage of the round, finished')))
  if (recorded.fault) return { halted: recordHalt('record', recorded, { launched: launch.names, branch, forks: tri.forks }) }
  const pushed = await pushRecord('push:triage', branch)
  if (!onIt(pushed, branch, true)) return { halted: pushHalt(pushed, branch, 'the triage is recorded on ' + branch + ' (' + recorded.result.commit + ') and the branch was not pushed') }
  const after = await readState()
  if (!after.state) return { halted: haltAfter('read-back', 'the triage is recorded and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + after.error, { transient: !!after.transient, step: after.step || null, branch }) }
  return { state: after.state, entries: tri.entries, forks: tri.forks, unverified: tri.unverified, handed: tri.handed, record: recorded.result.commit, reporters: launch.names, candidate: built.candidate }
}
function closeOf() {
  return {
    order: ['the close of the run, as the workflow\'s doc has it', 'sync: close.sync, spawned verbatim on model close.sync.model', 'dev/gate green on the merged tree', 'git push origin ' + loopBranch, 'gh pr create --base main --head ' + loopBranch],
    sync: { agentType: 'build-git', model: GIT_MODEL, label: 'git:sync-main', prompt: syncMainPrompt(v.run, v.scratch), schema: STEP_SCHEMA, reads: 'its `line` is the ONE line `' + STEP_TOOL + ' sync-main` printed, a digest — JSON: status `merged` or `up-to-date` with head, origin_main and resolved_logs, or `halted` with the word it refused with (`refused`); and `file`, the path under ' + v.scratch + ' of the line the tool prints unasked, its halt report in it' },
  }
}

async function watched(call) {
  triesSeen = []
  const back = await call()
  const tries = triesSeen
  triesSeen = null
  exhaustedLabels = []
  breakerTripped = false
  return { back, tries }
}
async function probeAct(label, act, what, flags) {
  const raw = await gitStep('probe:' + label, 'Probe', probeStep(what, act, flags), STEP_SCHEMA)
  return { raw, read: readStep(raw, act, true) }
}
function faultOf(read) {
  if (!read) return 'nothing came back'
  if (read.relay) return read.relay
  return read.status === 'halted' ? (read.halt && read.halt.root_cause) || 'the step halted' : null
}
async function runProbe() {
  phase('Probe')
  const s = v.scratch
  const stopped = (at, why, more) => Object.assign({ status: 'halted', probe: v.probe, scratch: s, halted: Object.assign({ phase: at, reason: why }, more || {}), message: 'the `' + v.probe + '` probe HALTED at ' + at + ': ' + why + '. No file of the repository was written; what the probe left lies under ' + s + '/probe/' + v.probe + '/. Invoke it again under a FRESH scratch root: a probe\'s root is made once.' })
  const begun = (await probeAct('begin', 'begin', 'its first act: the root of the `' + v.probe + '` probe under the scratch root, and what the probe stands on', '--scratch ' + s + ' --probe ' + v.probe)).read
  if (!begun || begun.status !== 'begun') return stopped('begin', 'the probe was not begun: ' + faultOf(begun), { refused: (begun && begun.refused) || null, halt: (begun && begun.halt) || null })
  const observed = []
  const cases = []
  if (v.probe === 'required') {
    for (const id of ['a', 'b']) {
      const { back, tries } = await watched(() => roleStep('review', 'probe:required:' + id, 'Probe', probeReviewPrompt(id, begun.binary), PROBE_UNIT_SCHEMA))
      observed.push({ case: id, tries, returned: back })
      cases.push(id + ':' + triesOf(tries) + ':' + probeField(back))
    }
  }
  if (v.probe === 'relay' || v.probe === 'relay-document') {
    for (const rows of PROBE_ROWS) {
      for (let n = 1; n <= PROBE_RELAYS; n++) {
        const tag = 'r' + rows + '-' + n
        const { back, tries } = await watched(() => probeAct('state:' + tag, 'state', 'the ONE line of the state of a throwaway run of ' + rows + ' ledger row(s), relayed — relay ' + n + ' of ' + PROBE_RELAYS, '--scratch ' + s + ' --probe ' + v.probe + ' --rows ' + rows + ' --tag ' + tag))
        const cell = probeRelayed(back.raw, back.read)
        observed.push({ case: tag, tries, back: cell, fault: faultOf(back.read) })
        cases.push(tag + ':' + triesOf(tries) + ':' + cell)
      }
    }
  }
  if (v.probe === 'payload') {
    for (const n of PROBE_ENTRIES) {
      const batch = probeBatch(s, n)
      const { back, tries } = await watched(() => roleStep('record', 'probe:payload:e' + n, 'Probe', probePayloadPrompt(s, n, batch), STEP_SCHEMA))
      const read = readStep(back, 'hash', true)
      const relayed = read && read.status === 'hashed' && SHA256_RE.test(String(read.file_sha256)) ? read.file_sha256 : 'none'
      observed.push({ case: 'e' + n, tries, composed: { file: batch.file, sha256: batch.sha256 }, relayed, fault: faultOf(read) })
      cases.push('e' + n + ':' + triesOf(tries) + ':' + batch.sha256 + ':' + relayed)
    }
  }
  if (v.probe === 'hold') {
    const seconds = v.seconds || PROBE_HOLD
    const begunA = heldOf(await toolStep('hold-start:a', 'Probe', 'hold-start', holdStartPrompt(v, 'a', 'probe', '--seconds ' + seconds, 'the hold probe\'s long command, case a, of ' + seconds + ' seconds')), s)
    const a = await watched(() => toolStep('hold-wait:a', 'Probe', 'hold-wait', holdWaitPrompt(v, 'a', 'the hold probe\'s long command, case a')))
    const held = heldOf(a.back, s)
    const line = HELD_TAKES.probe.includes(held.ends) && held.kind === 'probe' && held.name === 'a' ? 'held' : a.back && !a.back.relay && held.name === 'a' ? 'other' : 'none'
    observed.push({ case: 'a', seconds, started: begunA.ends, tries: a.tries, line, ends: held.ends, why: held.why || null })
    cases.push('a:' + triesOf(a.tries) + ':' + line)
    const started = (await watched(() => probeAct('hold:b', 'hold', 'the long command, STARTED in a session of its own: this step returns at once, and the command runs on for ' + seconds + ' seconds', '--scratch ' + s + ' --name b --seconds ' + seconds + ' --detach'))).back.read
    const most = Math.ceil(seconds / PROBE_SLICE) + 2
    let reads = 0
    let last = 'none'
    while (reads < most && last !== 'done' && last !== 'dead') {
      reads++
      const answer = (await watched(() => probeAct('held:b:' + reads, 'held', 'what became of the long command another step started, asked within a slice of ' + PROBE_SLICE + ' seconds — read ' + reads, '--scratch ' + s + ' --name b --slice ' + PROBE_SLICE))).back.read
      last = answer && ['done', 'running', 'dead'].includes(answer.status) ? answer.status : 'none'
      if (answer && answer.refused) break
    }
    observed.push({ case: 'b', seconds, started: started ? started.status : null, reads, last, fault: faultOf(started) })
    cases.push('b:' + reads + ':' + last)
  }
  const asked = '--scratch ' + s + ' --probe ' + v.probe + ' --sum ' + sha256(cases.join(' ')) + ' -- ' + cases.join(' ')
  const judged = (await probeAct('verdict', 'verdict', 'the judgement: what the harness observed of each case, held by the tool to what it knows itself, and written under the scratch root', asked)).read
  if (!judged || judged.status !== 'judged' || !Array.isArray(judged.cases)) return stopped('verdict', 'what the probe observed was not judged: ' + faultOf(judged) + '. The observations stand: `halted.command`, run as it is from the repository\'s root, judges them', { refused: (judged && judged.refused) || null, halt: (judged && judged.halt) || null, command: PROBE_TOOL + ' verdict ' + asked, observed })
  return { status: 'probed', probe: v.probe, scratch: s, result: judged.file, cases: judged.cases, observed, message: 'the `' + v.probe + '` probe ran: `cases` is what ' + PROBE_TOOL + ' judged, one entry per case with its `verdict`, as it wrote it to ' + judged.file + '; `observed` is what this script saw of each call. What a verdict decides is implementation/stabilization-workflow.md -> The runtime probes.' }
}

async function runTest() {
  phase('State')
  const looks = v.stopAfter === 'state'
  const gs = await toolStep('state', 'State', 'git-state', gitStatePrompt(v, looks))
  if (!onIt(gs, loopBranch, false)) return gitHalt('git', gs, 'the tree and the branches are not in the state the `test` stage starts from — the loop branch ' + loopBranch + ' checked out, holding nothing but what the record script wrote and no commit holds yet, no product change outside a round\'s merge, and no commit the remote lacks that is no record\'s')
  arrived = { found: gs.found || [], finished: gs.finished || [], owed: gs.owed || [] }
  if (arrived.finished.length) log('the first read finished what an earlier invocation left: ' + arrived.finished.join(', '))
  stageHead = String(gs.head)
  stageFence = SHA256_RE.test(String(gs.fence)) ? String(gs.fence) : null
  if (looks && arrived.owed.some((word) => word !== 'filed')) return { status: 'stopped', after: 'state', stage: 'test', run: v.run, owed: arrived.owed, pending: gs.pending || null, head: gs.head, remote_head: gs.remote_head || null, arrived, message: 'this invocation only looked (stopAfter: \'state\'), and FINISHED NOTHING: no write was finished, no batch gated or committed, nothing pushed, and the state was not read across it. What an invocation without that stop finishes before it does anything else is `owed` — ' + arrived.owed.join(', ') + ' — as `' + STEP_TOOL + ' table` names each.' }
  if (gs.pending) return await finishRecord(gs)
  const first = await readState()
  if (!first.state) return halt('state', 'the run\'s state could not be read: ' + first.error, { transient: !!first.transient, halt: first.halt || null, step: first.step || null })
  let state = first.state
  if (!state.opened) return halt('state', 'the run `' + v.run + '` has no opening record (' + runDir(v.run) + '/opening.md): a run opens with the human-led step, and `test` does not start before it')
  if (v.rulings) return await ruleTheRun(state, gs.branch)
  const at = state.position.test
  if (at.refused) return { status: 'refused', stage: 'test', run: v.run, refused: at, message: 'the `test` stage is refused (' + at.refused + ', round ' + at.round + '): ' + refusalOf(at) + '. Nothing was run beyond the two reads.', next: state.next, not_ready: state.not_ready, stop: state.stop, named: namedOf(state), arrived }
  const ctx = { run: v.run, round: at.round, stage: 'test', attempt: at.attempt, scratch: v.scratch }
  const label = 'c' + ctx.round
  const continues = continuing(state, at)
  if (continues.length) return await continueTest(ctx, state, continues)
  if (at.triage) {
    if (v.scope != null || v.clause != null || v.crossModel != null) return { status: 'refused', stage: 'test', run: v.run, message: 'round ' + at.round + '\'s triage is not finished (`next` is `' + state.next + '`): this invocation finishes it and runs no instrument, so it takes no scope, clause or crossModel. Nothing was run beyond the two reads.', next: state.next, untriaged: state.untriaged, named: namedOf(state), arrived }
    const tested = String(state.rounds.find((r) => r.round === at.round).candidate)
    const lap = launcher(ctx)
    const unbegun = await beginAttempt(ctx, lap, tested)
    if (unbegun) return unbegun
    const done = await finishTriage(ctx, lap, state, { sha: tested, label, tested: true }, loopBranch)
    if (done.halted) return done.halted
    return Object.assign({ status: 'triaged', stage: 'test', run: v.run, round: ctx.round, triage_only: true, candidate: done.candidate, record: done.record, counts: { reporters: done.reporters.length, findings_in: done.handed, entries: done.entries.length, blockers: done.state.blockers, for_the_human: done.state.human_list }, forks: done.forks, unverified: done.unverified, forbids_close: done.state.forbids_close }, nextOf(done.state))
  }
  const rerun = at.rerun || null
  if (rerun && v.clause !== rerun.clause) return { status: 'refused', stage: 'test', run: v.run, message: 'the state asks for a re-run (`next` is `' + state.next + '`): the items of clause `' + rerun.clause + '` that are due in round ' + rerun.round + ' — ' + rerun.items.map((i) => i.item + ' (attempt ' + i.attempt + ')').join(', ') + '. Invoke `test` with clause: \'' + rerun.clause + '\'; an invocation without it would begin a round nobody asked for. Nothing was run beyond the two reads.', next: state.next, retest: state.retest, position: at, named: namedOf(state, rerun.round), arrived }
  if (!rerun && v.clause != null) return { status: 'refused', stage: 'test', run: v.run, message: 'args.clause asks for a re-run, and the state does not (`next` is `' + state.next + '`): an item is run again only where the record script lists it as due, as an attempt of the round that selected it. Nothing was run beyond the two reads.', next: state.next, retest: state.retest, named: namedOf(state), arrived }
  const sha = rerun ? String((state.rounds.find((r) => r.round === rerun.round) || {}).candidate || '') : String(gs.head)
  if (!SHA_RE.test(sha)) return halt('state', 'the state asks for a re-run inside round ' + at.round + ' and names no candidate for that round: a result is of the candidate its round tested, and of no other commit')
  const moved = sha !== String(gs.head)
  const launch = launcher(ctx)
  const items = state.items || []
  const crossNamed = v.crossModel || []
  const crossStrangers = crossNamed.filter((id) => !items.some((i) => i.item === id && chainOf(i.kind, true).some((stepList) => stepList.some((s) => s.crossModel))))
  if (crossStrangers.length) return halt('state', 'args.crossModel names ' + crossStrangers.join(', ') + ', which is no item of the test set whose chain has a cross-model pass — the items that have one are ' + (items.filter((i) => chainOf(i.kind, true).some((stepList) => stepList.some((s) => s.crossModel))).map((i) => i.item).join(', ') || '(none)'))
  const runs = (i) => (rerun ? rerun.items.some((x) => x.item === i.item) : i.selected === true)
  log('round ' + ctx.round + ', attempt ' + ctx.attempt + ' of the test stage: ' + (rerun ? 'A RE-RUN inside it, on ' + sha + ' — ' + rerun.items.map((i) => i.item + ' (its attempt ' + i.attempt + ')').join(', ') + ' of clause `' + rerun.clause + '`, and nothing else' : 'candidate ' + label + ' = ' + sha + '; ' + items.length + ' item(s) in the test set'))
  if (looks) return { status: 'stopped', after: 'state', stage: 'test', run: v.run, round: ctx.round, attempt: ctx.attempt, candidate: { label, sha }, owed: arrived.owed, state: attached(state), named: namedOf(state, rerun ? rerun.round : null), arrived }
  const unbegun = await beginAttempt(ctx, launch, sha)
  if (unbegun) return unbegun

  phase('Preflight and scope')
  const earlier = (state.rounds || []).find((r) => r.round === ctx.round - 1)
  const always = items.filter((i) => i.kind === CHECK_KIND && runs(i))
  const scopeName = rerun ? null : launch.add(['scope'])
  const steps = [async () => {
    const asserted = await preflightOf(ctx, launch, ['preflight'], { image: false, sha, label, branch: loopBranch, checks: moved ? [] : always, crossModel: crossNamed.length > 0, tested: moved }, 'Preflight and scope')
    if (asserted.fault) return asserted
    const binaries = await buildBoth(label, sha, previousOf(state), 'Preflight and scope')
    if (binaries.fault) return Object.assign({ name: asserted.name }, binaries)
    const gate = rerun ? null : await hold({ name: 'gate-' + sha.slice(0, 12), stands: true, kind: 'gate', flags: '--run ' + v.run, what: 'the full gate of the candidate, label ' + label, phase: 'Preflight and scope' })
    if (gate && !HELD_TAKES.gate.includes(gate.ends)) return { name: asserted.name, fault: heldFault('the candidate\'s gate', gate) + ' A record commit is held to the candidate\'s own gate, so nothing of this attempt could be recorded against one.', phase: 'gate', held: heldSaid(gate), transient: gate.ends === 'unread' || gate.ends === 'running', then: heldThen(v.scratch, gate) }
    return { name: asserted.name, result: asserted.result, built: binaries, gate }
  }]
  if (!rerun) steps.push(() => roleStep('scope', 'scope', 'Preflight and scope', scopePrompt(ctx, launch, scopeName, { sha, label, base: earlier && earlier.candidate ? earlier.candidate : null, earlier: !!earlier, scope: v.scope, previous: previousOf(state), fallback: state.facts.scope }), SCOPE_SCHEMA))
  const both = await parallel(steps)
  const pre = both[0] || { fault: 'the preflight returned no result', transient: true }
  const sc = rerun ? { status: 'stands' } : both[1]
  if (pre.fault) return halt(pre.phase || 'preflight', pre.fault, { transient: !!pre.transient, halt: pre.halt || null, held: pre.held || null, then: pre.then })
  if (!sc || (sc.status !== 'written' && sc.status !== 'stands')) return halt('scope', sc ? 'the scope step halted: ' + ((sc.halt && sc.halt.root_cause) || 'no reason given') : 'the scope step returned no result', { transient: !sc, halt: sc ? sc.halt : null })
  const built = pre.built
  const second = await readState()
  if (!second.state) return halt('state', 'the run\'s state could not be read after the scope step: ' + second.error, { transient: !!second.transient, halt: second.halt || null, step: second.step || null })
  state = second.state
  if (!rerun && (!state.doors || state.round !== ctx.round)) return halt('scope', 'the scope step reported ' + sc.status + ', and the state holds no doors for round ' + ctx.round)
  const doorsOf = rerun ? rerun.round : ctx.round
  const selected = state.items.filter(runs)
  const held = selected.filter((i) => heldKind(i.kind))
  const hunting = selected.filter((i) => i.kind !== CHECK_KIND && !heldKind(i.kind))
  const late = selected.filter((i) => i.kind === CHECK_KIND && !always.some((x) => x.item === i.item))
  const unknownKinds = hunting.filter((i) => !CHAINS[i.kind]).map((i) => i.item + ' (' + i.kind + ')')
  if (unknownKinds.length) return halt('state', 'this round runs item(s) of a kind the harness has no chain for: ' + unknownKinds.join(', ') + ' — a kind is added by adding its chain to CHAINS, never by guessing one', { launched: launch.names })
  const runOf = (i) => (rerun ? (rerun.items.find((x) => x.item === i.item) || {}).attempt : 1)
  const heldRan = []
  for (const i of held) {
    const kind = heldKind(i.kind)
    if (kind === 'gate') {
      heldRan.push(Object.assign({ item: i.item }, pre.gate || { ends: 'not-run', kind, why: 'off-tree' }))
      continue
    }
    heldRan.push(Object.assign({ item: i.item }, await hold({ name: i.item + '-' + sha.slice(0, 12) + '-a' + runOf(i), stands: true, kind, flags: regressionFlags(previousOf(state).commit, sha), what: 'the held check `' + i.item + '`: the regression set, the previous release against the candidate', phase: 'Preflight and scope' })))
  }
  for (const h of heldRan.filter((x) => !HELD_TAKES.check.includes(x.ends))) log('VOID BY A WORD: the held check `' + h.item + '` — ' + (h.ends === 'not-run' ? 'a gate reads the working tree, which is no longer the round\'s candidate: it is not run again inside its round' : heldEnded(h) + (h.why ? ' (`' + h.why + '`)' : '')) + '. It left no verdict, and is recorded void — `' + heldVoid(h) + '` — a run that counts')
  const needsImage = hunting.some((i) => chainOf(i.kind, false).some((stepList) => stepList.some((s) => s.image)))
  let checks = pre.result.checks || []
  let secondName = null
  let unprovided = null
  if (needsImage || late.length) {
    const more = await preflightOf(ctx, launch, ['preflight', 'second'], { image: needsImage, sha, label, branch: loopBranch, checks: late, crossModel: false, tested: moved }, 'Preflight and scope')
    secondName = more.name
    if (more.fault) {
      unprovided = 'the second preflight did not provide it — ' + more.fault
      log('NOT PROVIDED by the second preflight (' + more.fault + '): ' + (needsImage ? 'no trial arm runs, and ' : '') + late.length + ' check(s) are void — every other item goes on')
    } else {
      built.image = needsImage ? more.result.image : null
      checks = checks.concat(more.result.checks || [])
    }
  }
  const imageless = (i) => !!unprovided && chainOf(i.kind, false).some((stepList) => stepList.some((s) => s.image))
  log('scope ' + sc.status + ': ' + (rerun ? 'the doors of round ' + doorsOf : state.doors.included + ' door(s)') + ' inside; ' + hunting.length + ' of ' + state.items.filter((i) => i.kind !== CHECK_KIND).length + ' hunting item(s) run ' + (rerun ? 'again' : 'this round') + ', and ' + (always.length + late.length) + ' check(s)')
  if (sc.uncovered && sc.uncovered.length) log('NOT COVERED: ' + sc.uncovered.length + ' door(s) of the round\'s test set are reached by no item — ' + sc.uncovered.join(' · '))
  if (v.stopAfter === 'preflight') return { status: 'stopped', after: 'preflight', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, previous: built.previous, gate: pre.gate ? heldSaid(pre.gate) : null, held: heldRan.map((h) => Object.assign({ item: h.item }, heldSaid(h))), checks, scope: { status: sc.status, uncovered: sc.uncovered || [], reached_but_excluded: sc.reached_but_excluded || [] }, reporters: launch.names, named: namedOf(state, doorsOf), arrived }

  phase('Instruments')
  const crossTool = crossNamed.length > 0 && (pre.result.checks || []).some((c) => c.check === CROSS_CHECK && c.status === 'green')
  const crossVoid = crossNamed.filter((id) => !crossTool || !hunting.some((i) => i.item === id)).map((id) => ({ item: id, why: crossTool ? 'the item does not run in this round' : 'the tool of the cross-model pass did not answer on this machine' }))
  for (const lost of crossVoid) log('CROSS-MODEL PASS VOID for `' + lost.item + '`: ' + lost.why + ' — its other passes run regardless')
  const covers = (i) => (rerun ? (rerun.items.find((x) => x.item === i.item) || {}).doors : i.doors)
  const units = hunting.filter((i) => !imageless(i)).map((i) => ({ item: i.item, kind: i.kind, clause: i.clause, chain: chainOf(i.kind, crossTool && crossNamed.includes(i.item)), doors: covers(i), round: doorsOf, range: null }))
  const nameable = unitNames(units, launch.names)
  if (nameable.fault) return haltAfter('names', nameable.fault)
  const ran = await runUnits(ctx, launch, built, units, 'Instruments')
  const reporters = ran.reduce((all, u) => all.concat(u.reporters), [])
  const unassertedBy = hashMismatch(built.candidate.sha256, reporters)
  const launchedSoFar = launch.names.slice()
  const settled = await settleReports(ctx, launch, 'check-reports', 'Instruments')
  if (settled.fault) return haltAfter('report-check', settled.fault, { transient: !!settled.transient, step: settled.file ? { file: v.scratch + '/' + settled.file.file, sha256: settled.file.sha256 } : null, launched: launchedSoFar })
  const scopeFile = runDir(v.run) + '/r' + ctx.round + '/scope.md'
  const noScope = settled.aside.find((a) => plain(a) && a.path === scopeFile)
  if (noScope) return haltAfter('scope-aside', 'the round\'s scope (' + scopeFile + ') was set aside by the report check: the file at its path is not what `dev/stabilize-record` would have written (`' + noScope.why + '`), so round ' + ctx.round + ' has no scope any more, and what this attempt\'s instruments found cannot be recorded against one', { launched: launchedSoFar, aside: settled.aside })
  const stoodOn = [ATTEMPT, pre.name, scopeName].concat(secondName && !unprovided ? [secondName] : []).filter((name) => settled.missing.includes(name))
  if (stoodOn.length) return haltAfter('stood-on', 'a reporter this stage stands on ' + (stoodOn.some((name) => setAside(settled.aside, name)) ? 'left a file at its report\'s path that is no report, and it was set aside' : 'returned and left no report') + ': ' + stoodOn.join(', ') + ' — what it established has nothing on record behind it', { launched: launchedSoFar, aside: settled.aside })
  unreported(ran, settled.missing, settled.aside)
  for (const u of ran) {
    if (u.status === 'void') log('VOID: `' + u.item + '` — ' + u.reason + '. Its result is recorded as void, and the state asks for its re-run')
    if (u.crossModel === 'void') crossVoid.push({ item: u.item, why: 'the pass was launched and did not report' })
  }
  const crossRan = ran.filter((u) => u.crossModel === 'ran').map((u) => u.item)
  if (v.stopAfter === 'instruments') return { status: 'stopped', after: 'instruments', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, units: ran.map((u) => ({ item: u.item, status: u.status, findings: u.reporters.reduce((n, r) => n + (r.result && r.result.findings ? r.result.findings.length : 0), 0) })), reporters: launch.names }

  const unitStatus = ran.map((u) => ({ item: u.item, status: u.status, reason: u.reason, word: u.word })).concat(hunting.filter(imageless).map((i) => ({ item: i.item, status: 'void', reason: 'no trial image: ' + unprovided, word: voidWord('no-image', secondName) })))
  const offTree = moved ? 'the round\'s candidate ' + sha + ' is no longer the tree checked out — the round\'s record commits lie on it — and a check reads the working tree: it was not run, because what it would show is evidence about no commit of this round' : null
  for (const item of always.concat(late)) {
    const c = offTree ? null : checks.find((x) => x.check === item.item)
    const elsewhere = c && c.status !== 'void' && c.commit !== sha ? 'the check ran on ' + (c.commit ? 'the commit ' + c.commit : 'a commit the preflight did not name') + ', not on the candidate' : null
    const by = late.includes(item) ? secondName : pre.name
    unitStatus.push({ item: item.item, status: !c || elsewhere ? 'void' : c.status, reason: elsewhere || offTree || (c ? c.evidence || 'the check could not run, and the preflight returned no reason' : late.includes(item) && unprovided ? unprovided : 'the preflight returned nothing for it'), evidence: c && c.evidence ? c.evidence + ' — the preflight\'s report: ' + pre.result.report : null, by, word: offTree ? voidWord('off-tree') : voidWord(elsewhere ? 'other-commit' : c ? 'could-not-run' : late.includes(item) && unprovided ? 'not-provided' : 'unanswered', by) })
  }
  for (const h of heldRan) unitStatus.push(HELD_TAKES.check.includes(h.ends) ? { item: h.item, status: h.ends, verdict: h.verdict, job: h.name } : { item: h.item, status: 'void', reason: heldVoid(h), word: heldVoid(h) })

  phase('Triage')
  const sources = reporters.filter((r) => r.result && r.result.findings).map((r) => ({ reporter: r.name, report: r.report, findings: findingLines(r.result.findings) }))
  if (sc.left_open && sc.left_open.length) sources.push(leftOpenSource(scopeName, sc.report, sc.left_open))
  if (hasQueue(state)) {
    const reds = unitStatus.filter((u) => u.status === 'red').map((u) => {
      const h = heldRan.find((x) => x.item === u.item)
      return Object.assign({ item: u.item, clause: (selected.find((i) => i.item === u.item) || {}).clause }, h ? { verdict: h.verdict, output: h.output } : { report: testReport(ctx, u.by) })
    })
    const once = await triageOnce(ctx, launch, sources.concat(redSources(reds), ledgerSource(v.run, state)))
    if (once.fault) return haltAfter('triage', once.fault, { transient: !!once.transient, halt: once.halt || null, step: once.step || null })
    if (v.stopAfter === 'triage') return { status: 'stopped', after: 'triage', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, triage: once.triage, findings_in: once.handed, reporters: launch.names, named: namedOf(state, doorsOf), arrived }
    phase('Record')
    const facts = rerun ? [] : [['candidate', sha], ['binary', built.candidate.sha256]].concat(sc.base && SHORT_SHA_RE.test(sc.base) ? [['base', sc.base]] : [], crossNamed.length ? [['cross-model', crossRan], ['cross-model-void', crossVoid.map((x) => x.item)]] : [])
    const words = recordWords({ reporters: launch.names, commit: sha, gate: pre.gate ? pre.gate.name : null, did: unitStatus, facts })
    const composed = await composeRecord('test:r' + ctx.round, loopBranch, ctx, v.scratch + '/record/test-r' + ctx.round + '-a' + ctx.attempt, 'docs(record): ' + subjectOf(v.run, ctx.round, rerun ? 'a re-run of ' + rerun.clause : 'the findings of the test stage'), words)
    if (composed.fault) return recordHalt('record', composed, { refusal: composed.refusal || null, launched: launch.names, triage: once.triage })
    const recorded = await recordStep('test:r' + ctx.round, loopBranch, composed.rec)
    if (recorded.fault) return recordHalt('record', recorded, { launched: launch.names, triage: once.triage })
    const pushed = await pushRecord('push', loopBranch)
    if (!onIt(pushed, loopBranch, true)) return pushHalt(pushed, loopBranch, 'the stage\'s first record is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
    const last = await readState()
    if (!last.state) return haltAfter('read-back', 'the stage\'s first record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + last.error, { transient: !!last.transient, step: last.step || null })
    state = last.state
    const closing = await verifyThenRecord(ctx, launch, built, state, { commit: recorded.result.commit, says: 'the stage\'s first record is committed and pushed (' + recorded.result.commit + ')' })
    if (closing.halted) return closing.halted
    const second = closing.second
    state = closing.state
    const voided = unitStatus.filter((u) => u.status === 'void')
    if (voided.length) log('VOID, AND ON RECORD AS VOID: ' + voided.map((u) => '`' + u.item + '` (`' + (u.word || u.reason) + '`)').join(', ') + ' — the state asks for the re-run of each where it owes one')
    if (heldRan.length) log('the held checks: ' + heldRan.map((h) => '`' + h.item + '` ' + (h.ends === 'not-run' ? 'not run' : 'ended ' + h.ends) + (h.name ? ' (the held command `' + h.name + '`' + (h.output ? ', its output ' + h.output : '') + ')' : '')).join('; '))
    return Object.assign({
      status: statusOf([], second),
      stage: 'test',
      run: v.run,
      round: ctx.round,
      rerun: rerun ? { clause: rerun.clause, round: rerun.round, items: rerun.items.length } : null,
      cross_model: { named: crossNamed.length, ran: crossRan.length, void: crossVoid.length },
      candidate: built.candidate,
      record: recorded.result.commit,
      triage: once.triage,
      verify: closing.verify,
      counts: { items: state.items.length, items_run: hunting.length + always.length + late.length + held.length, reporters: launch.names.length, findings_in: once.handed, blockers: state.blockers, for_the_human: state.human_list, voided: voided.length, unasserted: unassertedBy.length, held: heldRan.length },
      scope: { status: sc.status, uncovered: (sc.uncovered || []).length, reached_but_excluded: (sc.reached_but_excluded || []).length },
      forbids_close: state.forbids_close,
    }, nextOf(state), rerun ? { named: namedOf(state, rerun.round) } : {})
  }
  for (const source of ledgerSource(v.run, state)) sources.push(source)
  const tri = await triagePasses(ctx, launch, built, sources, [])
  if (tri.fault) return haltAfter('triage', tri.fault, { transient: !!tri.transient, halt: tri.halt || null, step: tri.step || null })
  if (v.stopAfter === 'triage') return { status: 'stopped', after: 'triage', stage: 'test', run: v.run, round: ctx.round, candidate: built.candidate, entries: tri.entries, forks: tri.forks, reporters: launch.names, named: namedOf(state, doorsOf), arrived }

  phase('Record')
  const rec = triageRecord(ctx, tri.entries)
  const facts = rerun ? null : { candidate: sha, binary: built.candidate.sha256 }
  if (crossNamed.length) Object.assign(facts, { 'cross-model': crossRan, 'cross-model-void': crossVoid.map((x) => x.item) })
  if (facts && sc.base && SHORT_SHA_RE.test(sc.base)) facts.base = sc.base
  const dir = v.scratch + '/record/test-r' + ctx.round + '-a' + ctx.attempt
  const commands = stageRecordCommands(ctx, { reporters: launch.names, gate: pre.gate ? { commit: sha, file: pre.gate.output } : null, results: { commit: sha, rows: resultRows(unitStatus) }, rows: rec.rows, triage: rec.triage, patches: [], facts, keys: tri.entries.map((e) => e.key) })
  const recorded = await recordStep('test:r' + ctx.round, loopBranch, recordPrompt(v, rerun ? 'the record of a re-run inside round ' + ctx.round + ' — ' + rerun.items.length + ' item(s) of clause `' + rerun.clause + '` run again, ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s)' : 'the record of round ' + ctx.round + '\'s test stage — ' + launch.names.length + ' report(s), ' + tri.entries.length + ' finding(s)', loopBranch, dir, rerun ? state.candidate.round : ctx.round, commands, subjectOf(v.run, ctx.round, rerun ? 'a re-run of ' + rerun.clause : 'the record of the test stage')))
  if (recorded.fault) return recordHalt('record', recorded, { launched: launch.names, forks: tri.forks })
  const pushed = await pushRecord('push', loopBranch)
  if (!onIt(pushed, loopBranch, true)) return pushHalt(pushed, loopBranch, 'the record is committed on ' + loopBranch + ' (' + recorded.result.commit + ') and the branch was not pushed')
  const last = await readState()
  if (!last.state) return haltAfter('read-back', 'the record is committed and pushed (' + recorded.result.commit + '), and the state could not be read back: ' + last.error, { transient: !!last.transient, step: last.step || null })
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
    counts: { items: state.items.length, items_run: hunting.length + always.length + late.length + held.length, reporters: launch.names.length, findings_in: tri.handed, entries: tri.entries.length, by_grade: byGrade, blockers: state.blockers, for_the_human: state.human_list, voided: unitStatus.filter((u) => u.status === 'void').map((u) => u.item) },
    forks: tri.forks,
    unverified: tri.unverified,
    unasserted: unassertedBy,
    held: heldRan.map((h) => Object.assign({ item: h.item }, heldSaid(h))),
    scope: { status: sc.status, uncovered: sc.uncovered || [], reached_but_excluded: sc.reached_but_excluded || [] },
    forbids_close: state.forbids_close,
  }, out, rerun ? { named: namedOf(state, rerun.round) } : {})
}

async function runFix() {
  phase('State')
  const gs = await toolStep('state', 'State', 'git-state', gitStatePrompt(v))
  if (!gs || gs.status !== 'ready') return gitHalt('git', gs, 'the tree or the branches are not in the state the `fix` stage starts from')
  arrived = { found: gs.found || [], finished: gs.finished || [], owed: gs.owed || [] }
  stageHead = String(gs.head)
  stageFence = SHA256_RE.test(String(gs.fence)) ? String(gs.fence) : null
  if (gs.pending) return await finishRecord(gs)
  let read = await readState()
  if (!read.state) return halt('state', 'the run\'s state could not be read: ' + read.error, { transient: !!read.transient, halt: read.halt || null, step: read.step || null })
  let state = read.state
  if (!state.opened) return halt('state', 'the run `' + v.run + '` has no opening record (' + runDir(v.run) + '/opening.md)')
  if (v.rulings && v.rulings.every(runRuling)) return await ruleTheRun(state, gs.branch)
  stageHead = null
  stageFence = null
  if (state.position.fix.refused) return { status: 'refused', stage: 'fix', run: v.run, refused: state.position.fix, message: 'the `fix` stage is refused (' + state.position.fix.refused + ', round ' + state.position.fix.round + '): ' + refusalOf(state.position.fix) + '. Nothing was run beyond the two reads.', next: state.next, not_ready: state.not_ready }
  const round = state.position.fix.round

  const found = await toolStep('find:r' + round, 'State', 'find-round', findRoundPrompt(v, round))
  if (!found || found.status !== 'ready') return gitHalt('git', found, 'the round\'s branches could not be listed')
  const local = found.local || []
  const stray = (found.remote || []).filter((b) => currentCut(v.run, round, [b]) && !local.includes(b))
  if (stray.length) return halt('git', 'origin has a branch of round ' + round + ' that is not here: ' + stray.join(', ') + ' — fetch it; this stage does not guess which branch the round is on')
  let cut = currentCut(v.run, round, local)
  let branch = cut ? branchName(v.run, round, cut) : loopBranch
  let checkedOut = found.branch
  let tipSha = String(found.head || '')
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

  async function secondHalf(ctx, launch, fixerSources, patches, fixerForks, doors, isPart) {
    phase('Fix')
    const head = await toolStep('push:' + branch + ':c' + ctx.cycle, 'Fix', 'push', pushPrompt(v, branch))
    if (!onIt(head, branch, true)) return { halted: gitHalt('push', head, 'the round\'s branch ' + branch + ' was not pushed') }
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
        if (d.unheld) return { halted: halt('binary', 'the fork of `' + fork.key + '`: ' + d.unheld, { branch }) }
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
  function unlanded(phaseName, why) {
    return Object.assign(halt(phaseName, why, { branch, cycles: cyclesDone(), limit: cycleLimit, blockers: state.blockers, human_list: state.human_list, forks, could_not_fix: couldNot }), { round }, phaseName === 'bound' ? { exits: { continue: 'args.raise = { cycles: ' + (Math.max(cycleLimit, cyclesDone()) + 1) + ' } — or more; passed on every later `fix` of this round', drop: 'args.exit = \'drop\'', part: 'args.exit = { part: [<the fix commits of ' + branch + ' to keep>] }' } } : {})
  }

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

  let finished = false
  for (;;) {
    const at = state.position.fix
    if (at.refused) return Object.assign({ status: 'refused', refused: at, message: 'the `fix` stage is refused (' + at.refused + ', round ' + at.round + '): ' + refusalOf(at) + '.' }, report, nextOf(state))
    const open = { branch: cut ? branch : null, cycles: cyclesDone(), blockers: state.blockers, human_list: state.human_list, untriaged: state.untriaged, forks }
    if (at.triage) {
      if (finished) return Object.assign({ status: 'cycle', message: 'round ' + round + '\'s triage was finished in this invocation and the state still names findings without a grade or a verdict: it is not run again on a guess — `next` says what is owed.' }, open, report, nextOf(state))
      if (!cut) return halt('git', 'the state says that the `fix` stage left round ' + round + '\'s triage unfinished, and the round has no branch: there is no tip a verifier could drive')
      const stopped = await onBranch(branch)
      if (stopped) return stopped
      const lap = { run: v.run, round, stage: 'fix', cycle: at.cycle, attempt: at.attempt, scratch: v.scratch }
      const done = await finishTriage(lap, launcher(lap), state, { sha: tipSha, label: 'r' + round + 'c' + at.cycle, tested: false }, branch)
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

return await (v.probe ? runProbe() : v.stage === 'test' ? runTest() : runFix())
