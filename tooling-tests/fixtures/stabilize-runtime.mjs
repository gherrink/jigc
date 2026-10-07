// A stand-in for the Workflow runtime, for ONE invocation of the stabilization harness
// (.claude/workflows/stabilize.js) — what tooling-tests/stabilize_simulation.rs runs a
// stage under. It hands the committed script, unmodified, the globals the runtime hands a
// workflow script; every agent the script launches is a function here; and it prints what
// the invocation did as one line of JSON.
//
//   node stabilize-runtime.mjs <the harness script> <a scenario file>
//
// run from the root of the repository the stage works on. The scenario is the suite's:
// { args, checks, gate, scope, agents, record, wrongBinary, wrongPrevious, endings, omits,
// violation, garbles } — the
// invocation's arguments, and what the scripted agents say (below). `wrongBinary` names the
// reporters that return another hash than the one they were handed: a driver that drove
// something else; `wrongPrevious` the verifiers that return another hash for the previous
// release's binary. `omits` names, by an agent's label, the fields it LEAVES OUT of its
// return — whatever its stand-in would have put there; where the call's schema requires
// such a field, what the runtime then does is not known to this stand-in and is the
// scenario's to say, as `violation`: `handed` (the script is handed the return as it is),
// `nothing` (it is handed nothing, on every try) or `throws` (the call throws, on every
// try). `garbles` names the git steps that relay their line with one character changed.
// `endings` says how an agent ENDS where that is not "it returns its
// result and has written its report", by its label:
//   dies               it returns nothing, every time it is tried, and wrote no report
//   dies-after-report  it wrote its report — once — and returns nothing
//   halts-unreported   it returns the halt the scenario scripts, and wrote no report
//   unreported         it returns the result the scenario scripts, and wrote no report
// (A halt WITH its report needs no ending: it is a scripted return like any other.)
//
// WHAT AN AGENT IS HERE. A function chosen by the call's label. Four kinds do what their
// definition gives the role to do in the repository, by running the commands their prompt
// spells — taken out of the prompt, never rebuilt here:
//
//   git:*      build-git           the ONE command of the step, run; its one line relayed
//   preflight  stabilize-preflight the two environment asserts a test can run, held as
//                                  the definition words them — the candidate is `HEAD`,
//                                  or, where the prompt says it is not, a commit `HEAD`
//                                  holds, and then no step that reads the working tree
//                                  may be listed; each binary a FILE at the
//                                  path the prompt names (no build), its sha256 measured;
//                                  each check answered as the scenario scripts it; and
//                                  the candidate's gate NOT RUN — what a full gate prints,
//                                  green or as the scenario scripts it red (`gate`),
//                                  written to the file the prompt names
//   scope      stabilize-scope     the round's doors written with the prompt's `scope-set`
//                                  — or `stands`, where an earlier attempt wrote them
//   git:probe:* build-git          the ONE command of a probe's step — a command of
//                                  dev/stabilize-probe, and of no other tool
//   probe:required:*  milestone-code-reviewer  the return the scenario scripts, with the
//                                  hash of the file on its binary line, measured; no
//                                  report — a probe that hands its reviewer one fails the run
//   probe:payload:*   build-executor  the payload written as the prompt says, then the
//                                  prompt's `dev/stabilize-probe hash`, its line relayed;
//                                  a prompt that names the record script or a gate fails
//                                  the run
//   probe:hold:*      stabilize-preflight  the prompt's `dev/stabilize-probe hold`, run to
//                                  its end — in the foreground: nothing here has a
//                                  background, a slice or a timeout — and the one line its
//                                  output file then holds, relayed
//   record:*   build-executor      the steps the prompt lists, in its order: the payload
//                                  written as the prompt says and held to its hash, the
//                                  batch applied by the prompt's own `apply`, and the gate
//                                  NOT RUN — its output, green or as scripted
//                                  (`record.gate`), written to the file the prompt names.
//                                  It makes no commit: that is a git step (`git:record:*`)
//
// Every other label is a reporter the scenario scripts by that label: its structured
// return is the scenario's, its report is written through the real
// `dev/stabilize-record report` with the flags its prompt hands it, and where the prompt
// carries a binary line the hash it returns is measured from the file at that path.
// A LABEL NOBODY SCRIPTED FAILS THE RUN, and so does a return its schema would not take.
//
// WHERE THIS IS NOT THE RUNTIME — the suite's header says what follows from each:
//   - `parallel` runs its thunks ONE AFTER ANOTHER, in order; nothing here is concurrent,
//     and no cap on concurrent agents exists.
//   - an agent's return is checked against the call's schema by a small validator
//     (type, required, properties, items, enum); the runtime's own validation, and a
//     model's retry on a mismatch, are not modelled.
//   - an agent dies only where a scenario scripts it (`endings`), and then on every try
//     the harness makes of it; nothing dies half-way through a command, there is no
//     journal and no resume.
//   - the script's top-level `return` works here because the script is the body of an
//     async function; how the runtime runs it is its own.
//   - `pipeline`, `workflow` and `budget` are there and unused by the harness: the first
//     two fail the run if called.

import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs'
import { dirname } from 'node:path'
import { spawnSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import vm from 'node:vm'

const [script, scenarioFile] = process.argv.slice(2)
const scenario = JSON.parse(readFileSync(scenarioFile, 'utf8'))
const scripted = scenario.agents || {}
const endings = scenario.endings || {}

const omits = scenario.omits || {}
const trace = []
const ran = []
const logs = []
const reporters = []
const swallowed = []
const fatal = []
const used = new Set()
const tried = new Set()
let depth = 0

function note(line) {
  trace.push('  '.repeat(depth) + line)
}
// fail — the run is not one the simulation can stand behind. The harness catches what an
// agent throws and tries again, so the reason is kept here, out of its reach.
function fail(why) {
  fatal.push(why)
  throw new Error(why)
}
function sh(command, input) {
  // What ran on an agent's behalf, by its first line: a payload's text is not a command.
  ran.push(command.split('\n')[0])
  const out = spawnSync('sh', ['-c', command], { encoding: 'utf8', input: input || '' })
  return { code: out.status, stdout: out.stdout || '', stderr: out.stderr || '' }
}
function git(...argv) {
  const out = spawnSync('git', argv, { encoding: 'utf8' })
  return { code: out.status, stdout: (out.stdout || '').replace(/\n$/, ''), stderr: out.stderr || '' }
}
function sha256Of(path) {
  return createHash('sha256').update(readFileSync(path)).digest('hex')
}
function span(text, opens) {
  const at = text.indexOf('`' + opens)
  return at < 0 ? null : text.slice(at + 1, text.indexOf('`', at + 1))
}
// What a full `dev/gate --keep-going` prints, as much of it as anybody reads: its mode
// line, its totals line, its verdict line, and the failing tests by name. `red` is
// { steps, tests }, or nothing.
function gateOutput(red) {
  const steps = (red && red.steps) || []
  const tests = (red && red.tests) || []
  return ['gate: log    (scripted by the simulation: no gate ran)', 'gate: mode   full, keep-going (no red step stops the run but a red build; the suite is ONE step, `test` -- what is named red is all that is red)', '', '--- summary ---', 'tests   passed=1 failed=' + tests.length + '  (over 1 test binaries)', '']
    .concat(steps.length ? ['GATE: FAIL (step: ' + steps.join(' ') + ')', ''].concat(tests.length ? ['failing tests:'].concat(tests.map((t) => '  ' + t), ['']) : []) : ['GATE: PASS'])
    .join('\n') + '\n'
}
function keepGate(command, red) {
  // THE GATE OF A STABILIZATION RUN KEEPS GOING: a stage that spells a plain `dev/gate`
  // has no gate this stand-in keeps, and its record step halts on the file that is not there.
  const file = /^dev\/gate --keep-going > (\S+) 2>&1$/.exec(command)[1]
  mkdirSync(dirname(file), { recursive: true })
  writeFileSync(file, gateOutput(red))
  return file
}
function treeState() {
  return 'branch ' + git('branch', '--show-current').stdout + '\n' + git('status', '--porcelain').stdout
}

// ---- a report, through the record script ----
// The command is the prompt's: the flags of its REPORT: line and its scratch root, or — for
// a role that has no definition — the command its prompt spells whole.
function reportCommand(prompt) {
  const labelled = /^REPORT: (--run .+?) — write your report through `dev\/stabilize-record report` with exactly these flags, and `(--scratch [^`]+)`/m.exec(prompt)
  if (labelled) return 'dev/stabilize-record report ' + labelled[1] + ' ' + labelled[2]
  return span(prompt, 'dev/stabilize-record report ')
}
function writeReport(label, prompt, said) {
  const command = reportCommand(prompt)
  if (!command) fail('the prompt of `' + label + '` names no report to write')
  const name = / --reporter (\S+)/.exec(command)[1]
  // An agent that ends with no report wrote none — whichever kind it is.
  if (endings[label] === 'unreported' || endings[label] === 'halts-unreported') return null
  // A try after a transient failure writes no report that stands already — as the retry's
  // prompt has it.
  if (prompt.includes('RETRY after a transient failure') && reporters.includes(name)) return null
  const out = sh(command, '# ' + label + '\n\nScripted by the simulation: no agent wrote this.\n\n' + said + '\n\n<!-- end of report -->\n')
  if (out.code !== 0) fail('the report of `' + label + '` was refused: ' + out.stderr)
  reporters.push(name)
  return out.stdout.trim()
}

// ---- the four kinds that act ----
function gitStep(label, prompt) {
  const first = prompt.split('\n').find((line) => line.startsWith('1. `')) || ''
  const command = first.slice(4, first.lastIndexOf('`'))
  // A probe's step is a command of the probe tool, and of no other; a stage's, of the step tool.
  const tool = label.startsWith('git:probe:') ? 'dev/stabilize-probe ' : 'dev/stabilize-step '
  if (!command.startsWith(tool)) fail('the git step `' + label + '` is not ONE command of ' + tool.trim() + ': ' + first)
  const out = sh(command)
  if ((scenario.garbles || []).includes(label)) {
    used.add(label)
    return { status: 'ran', line: out.stdout.replace('"status"', '"Status"') }
  }
  if (!out.stdout) return { status: 'halted', halt: { root_cause: 'the command printed no line', evidence: command + '\n' + out.stderr, tree_state: treeState(), recommendation: 'read the tree before the step is asked for again' } }
  // The step that begins an attempt writes the attempt's marker: a report the harness
  // launched, under the reporter the command names.
  if (command.startsWith('dev/stabilize-step begin ') && out.code === 0) reporters.push(/ --reporter (\S+)/.exec(command)[1])
  return { status: 'ran', line: out.stdout }
}

function preflight(label, prompt) {
  const asked = /^Candidate: label (\S+), commit ([0-9a-f]{40}) /m.exec(prompt)
  const runDir = /may show untracked files under `([^`]+)\/` and nothing else/.exec(prompt)
  if (!asked || !runDir) fail('the preflight\'s prompt names no candidate, or no run directory')
  const [, candidate, sha] = asked
  // The environment asserts (stabilize-preflight.md): `git rev-parse HEAD` is the
  // candidate's sha, and `git status --porcelain` shows what the prompt says to expect and
  // no more. What it prints for an untracked directory is the directory, so this does not
  // tell a report from another untracked file under the run's directory.
  // The candidate is `HEAD` — or, where the prompt says that it is not, a commit `HEAD`
  // holds: then both builds are from the commit, and a step that reads the working tree
  // (the gate, a check) is not one the definition lets this call list.
  const head = git('rev-parse', 'HEAD').stdout
  const status = git('status', '--porcelain').stdout
  const stray = status.split('\n').filter((line) => line && !line.startsWith('?? ' + runDir[1] + '/'))
  const notHead = prompt.includes('THE CANDIDATE IS NOT `HEAD` HERE') && new RegExp('`git merge-base --is-ancestor ' + sha + ' HEAD` must hold').test(prompt)
  const readsTheTree = prompt.includes('dev/gate --keep-going > ') || /^ {3}- [a-z0-9-]+: /m.test(prompt)
  const failed = notHead && git('merge-base', '--is-ancestor', sha, 'HEAD').code !== 0 ? 'the candidate ' + sha + ' is no commit that `HEAD` (' + head + ') holds'
    : notHead && readsTheTree ? 'the candidate ' + sha + ' is not `HEAD`, and this call lists a step that reads the working tree'
      : !notHead && head !== sha ? '`git rev-parse HEAD` is ' + head + ', and the candidate is ' + sha + ': the tree is not the candidate'
        : stray.length ? '`git status --porcelain` shows more than untracked files under ' + runDir[1] + '/: ' + stray.join(' · ') : null
  if (failed) {
    return { status: 'halted', halt: { root_cause: 'an environment assert failed: ' + failed, evidence: 'git rev-parse HEAD: ' + head + '\ngit status --porcelain:\n' + status, tree_state: treeState(), recommendation: 'nothing built on an unasserted environment is evidence' }, report: writeReport(label, prompt, 'Halted: ' + failed) }
  }
  const back = { status: 'ready', checks: [] }
  const standIn = (path, what) => {
    if (!existsSync(path)) {
      mkdirSync(dirname(path), { recursive: true })
      writeFileSync(path, 'a stand-in for the binary of ' + what + '\n')
    }
    return sha256Of(path)
  }
  const built = /the build: the candidate's binary, from `git archive ([0-9a-f]{40})`, copied to `([^`]+)`/.exec(prompt)
  if (built) back.candidate = { label: candidate, sha: built[1], binary: built[2], sha256: standIn(built[2], 'commit ' + built[1]), path_check: scenario.pathCheck || built[2] }
  const previous = /the previous release's binary: version `([^`]+)`, built from commit [0-9a-f]{40} .*? at `([^`]+)`/.exec(prompt)
  if (previous) back.previous = { version: previous[1], binary: previous[2], sha256: standIn(previous[2], 'release ' + previous[1]) }
  if (/\d+\. the trial image, built and verified/.test(prompt)) back.image = { tag: 'jigc-trial:' + candidate, verified: true, failed: [] }
  const gate = span(prompt, 'dev/gate --keep-going > ')
  if (gate) keepGate(gate, scenario.gate)
  // Whether the cross-model pass's tool answers: one more entry of `checks`, under the id
  // the prompt names — an answer, never an assert.
  const tool = /returned in `checks` under `check` = `([a-z0-9-]+)`/.exec(prompt)
  if (tool) {
    const answers = (scenario.checks || {})[tool[1]]
    if (!answers) fail('the scenario does not say whether the cross-model pass\'s tool answers (`checks.' + tool[1] + '`)')
    back.checks.push({ check: tool[1], status: answers, evidence: 'scripted by the simulation' })
  }
  for (const [, item] of prompt.matchAll(/^ {3}- ([a-z0-9-]+): /gm)) {
    const status = (scenario.checks || {})[item]
    if (!status) fail('no result is scripted for the deterministic check `' + item + '`')
    back.checks.push(Object.assign({ check: item, status, evidence: 'scripted by the simulation' }, (scenario.checkCommits || {})[item] === 'none' ? {} : { commit: (scenario.checkCommits || {})[item] || sha }))
  }
  const report = writeReport(label, prompt, 'Ready.')
  if (report) back.report = report
  return back
}

function scopeStep(label, prompt) {
  const command = span(prompt, 'dev/stabilize-record scope-set ')
  const between = /base = ([0-9a-f]{40}) .*; tip = ([0-9a-f]{40}) /.exec(prompt)
  if (!command || !between || !scenario.scope) fail('the scope step is not scripted, or its prompt names no `scope-set` and no two commits')
  const out = sh(command, JSON.stringify({ included: scenario.scope.included, excluded: scenario.scope.excluded }))
  // A round's scope is written once: where an earlier attempt of the stage wrote it, it
  // stands (stabilize-scope.md), and the script's refusal is the evidence.
  const stands = out.code === 6 && out.stderr.includes('refused exists: ')
  if (out.code !== 0 && !stands) fail('`scope-set` refused the scripted scope: ' + out.stderr)
  return Object.assign({ status: stands ? 'stands' : 'written', base: between[1], tip: between[2], report: writeReport(label, prompt, stands ? 'Stands.' : 'Written.') }, stands ? {} : { scope: JSON.parse(out.stdout).scope }, scenario.scope)
}

// The record step (build-executor.md, the RECORD STEP paragraph): the numbered steps of
// the prompt, in order, each exactly as written — the payload, the batch applied, the gate —
// and NO commit. A refusal is a halt, its one line the evidence.
// The numbered steps of an executor's prompt, between the line that says how they are run
// and the line the prompt ends with: each as its lines.
function numbered(lines, from, to) {
  const steps = []
  for (const line of lines.slice(from + 1, to)) {
    if (line.startsWith(steps.length + 1 + '. ')) steps.push([line.slice(String(steps.length + 1).length + 2)])
    else steps[steps.length - 1].push(line)
  }
  return steps
}
// The payload: the text, by the here-document the prompt spells, held to its hash. What
// went wrong, or null.
function writePayload(step) {
  const file = span(step[0], '/')
  const text = step.slice(1, -1).join('\n')
  const hash = /must print `([0-9a-f]{64})`/.exec(step[step.length - 1])
  const wrote = sh(span(step[0], 'mkdir -p ') + ' && ' + span(step[0], 'cat > ') + '\n' + text + '\nSTABILIZE_PAYLOAD\n')
  return wrote.code !== 0 || !hash || sha256Of(file) !== hash[1] ? { file, stderr: wrote.stderr } : null
}
function recordStep(label, prompt) {
  const lines = prompt.split('\n')
  const from = lines.findIndex((line) => line.startsWith('Run th'))
  const to = lines.findIndex((line) => line.startsWith('YOU MAKE NO COMMIT'))
  if (from < 0 || to < 0 || !/^BRANCH: (\S+?)[ ,]/m.test(prompt)) fail('the record step `' + label + '` is not the prompt a record step is handed')
  const steps = numbered(lines, from, to)
  const halted = (cause, evidence) => ({ status: 'halted', halt: { root_cause: cause, evidence, tree_state: treeState(), recommendation: 'the record step stopped where it stood: it commits nothing' } })
  const ruled = scenario.record || {}
  let gate = null
  for (const step of steps) {
    if (step[0].startsWith('Write ')) {
      const lost = writePayload(step)
      if (lost) return halted('the payload of the record step is not the text it was handed: ' + lost.file, lost.stderr)
      continue
    }
    const command = span(step[0], 'dev/')
    if (!command) fail('a step of the record step `' + label + '` spells no command: ' + step[0])
    if (command.startsWith('dev/gate --keep-going > ')) {
      // THE GATE IS NOT RUN: a test inside the gate cannot run the gate.
      gate = keepGate(command, ruled.gate)
      continue
    }
    if (!command.startsWith('dev/stabilize-record apply ')) fail('the record step `' + label + '` runs a command that is neither its batch nor its gate: ' + command)
    // A record step that did nothing and says it did: the batch is never applied.
    if (ruled.apply === 'skipped') continue
    const out = sh(command)
    if (out.code !== 0) return halted('the batch of the record step was refused: ' + out.stderr.trim(), command)
  }
  if (!gate) fail('the record step `' + label + '` names no gate')
  return { status: 'gated', gate }
}

// ---- a probe's agents ----
// What no probe's prompt may carry: a report to write, a command of the record script or of
// the step tool, a gate, a git command, a branch.
function probeOnly(label, whole) {
  // What a retry is told on top — to look at the tree a dead try left — is every call's,
  // and no part of the probe's own prompt.
  const prompt = whole.split('\n\nRETRY after a transient failure')[0]
  const carried = [['a report to write', /^REPORT: /m], ['a command of the record script', /`dev\/stabilize-record [a-z]/], ['a command of the step tool', /dev\/stabilize-step/], ['a gate', /dev\/gate/], ['a git command', /`git /], ['a branch', /^BRANCH: |fix\//m]].filter(([, pattern]) => pattern.test(prompt)).map(([what]) => what)
  if (carried.length) fail('the prompt of the probe\'s `' + label + '` carries ' + carried.join(', ') + ': a probe works on no run and writes nothing of the repository')
}
// The `required` probe's reviewer: what the scenario scripts, with the hash of the file its
// binary line names — measured. It writes no report.
function probeReviewer(label, prompt) {
  probeOnly(label, prompt)
  used.add(label)
  if (!(label in scripted)) fail('no agent is scripted for the label `' + label + '`')
  const binary = /^BINARY: candidate `([^`]+)` sha256 [0-9a-f]{64} \(commit [0-9a-f]{40}, label [^)]+\)/m.exec(prompt)
  if (!binary) fail('the prompt of `' + label + '` carries no binary line')
  return Object.assign({}, scripted[label], { asserted_sha256: (scenario.wrongBinary || []).includes(label) ? '0'.repeat(64) : sha256Of(binary[1]) })
}
// The `payload` probe's executor: the payload written as the prompt says, then the one
// command that hashes it — its line relayed.
function probePayload(label, prompt) {
  probeOnly(label, prompt)
  const lines = prompt.split('\n')
  const from = lines.findIndex((line) => line.startsWith('Run th'))
  const to = lines.findIndex((line) => line.startsWith('Report status = ran'))
  if (!prompt.startsWith('RECORD STEP — a PROBE') || from < 0 || to < 0) fail('the probe\'s `' + label + '` is not handed a record step\'s payload to write')
  let line = null
  for (const step of numbered(lines, from, to)) {
    if (step[0].startsWith('Write ')) {
      const lost = writePayload(step)
      if (lost) return { status: 'halted', halt: { root_cause: 'the payload is not the text it was handed: ' + lost.file, evidence: lost.stderr, tree_state: treeState(), recommendation: 'nothing was applied' } }
      continue
    }
    const command = span(step[0], 'dev/stabilize-probe hash ')
    if (!command) fail('a step of the probe\'s `' + label + '` is neither its payload nor its hash: ' + step[0])
    line = sh(command).stdout
  }
  return line ? { status: 'ran', line } : fail('the probe\'s `' + label + '` hashed nothing')
}
// The `hold` probe's agent: the long command, run to its end, and the one line its output
// file then holds.
function probeHold(label, prompt) {
  probeOnly(label, prompt)
  const command = span(prompt, 'dev/stabilize-probe hold ')
  const out = command && / > (\S+) 2>&1$/.exec(command)
  if (!out) fail('the probe\'s `' + label + '` is handed no long command with its output in a file')
  sh(command)
  return { status: 'ran', line: readFileSync(out[1], 'utf8') }
}

// ---- a scripted reporter ----
function reporter(label, prompt, opts) {
  used.add(label)
  const back = Object.assign({}, scripted[label])
  // The binary it is handed: on its BINARY: line, or — for the role that has no definition
  // — in the `shasum` its prompt spells. The hash it returns is measured, not copied.
  const binary = /^BINARY: candidate `([^`]+)` sha256 [0-9a-f]{64} \(commit [0-9a-f]{40}, label [^)]+\)(?:; previous release `([^`]+)`)?/m.exec(prompt)
  const spelled = span(prompt, 'shasum -a 256 ')
  if (binary || spelled) back.asserted_sha256 = (scenario.wrongBinary || []).includes(label) ? '0'.repeat(64) : sha256Of(binary ? binary[1] : spelled.slice('shasum -a 256 '.length))
  if (binary && opts.agentType === 'finding-verifier') back.ran_on = { candidate: back.asserted_sha256, previous: (scenario.wrongPrevious || []).includes(label) ? '0'.repeat(64) : sha256Of(binary[2]) }
  // It ends with its report written and its result returned — or as the scenario says.
  const report = writeReport(label, prompt, 'Returned: ' + JSON.stringify(scripted[label]))
  if (report) back.report = report
  return back
}

const KINDS = [
  { labels: /^git:/, agentType: 'build-git', play: gitStep },
  { labels: /^preflight(-second)?$/, agentType: 'stabilize-preflight', play: preflight },
  { labels: /^scope$/, agentType: 'stabilize-scope', play: scopeStep },
  { labels: /^record:/, agentType: 'build-executor', play: recordStep },
  { labels: /^probe:required:/, agentType: 'milestone-code-reviewer', play: probeReviewer },
  { labels: /^probe:payload:/, agentType: 'build-executor', play: probePayload },
  { labels: /^probe:hold:/, agentType: 'stabilize-preflight', play: probeHold },
]

// ---- what a schema asks of a return ----
function conforms(schema, value, at) {
  const type = Array.isArray(value) ? 'array' : value === null ? 'null' : typeof value
  if (schema.type && schema.type !== type) return at + ' is ' + type + ', not ' + schema.type
  if (schema.enum && !schema.enum.includes(value)) return at + ' is ' + JSON.stringify(value) + ', not one of ' + schema.enum.join(', ')
  if (type === 'object') {
    for (const name of schema.required || []) if (!(name in value)) return at + ' lacks `' + name + '`'
    for (const [name, inner] of Object.entries(schema.properties || {})) {
      const why = name in value ? conforms(inner, value[name], at + '.' + name) : null
      if (why) return why
    }
  }
  if (type === 'array' && schema.items) {
    for (let i = 0; i < value.length; i++) {
      const why = conforms(schema.items, value[i], at + '[' + i + ']')
      if (why) return why
    }
  }
  return null
}

// ---- the globals the runtime hands a workflow script ----
const context = vm.createContext({})
// A workflow script has the language's built-ins and nothing else — no clock, no randomness.
vm.runInContext('Date.now = () => { throw new Error(\'Date.now() is unavailable in a workflow script\') }\n'
  + 'Math.random = () => { throw new Error(\'Math.random() is unavailable in a workflow script\') }\n'
  + 'Date = new Proxy(Date, { construct(target, argv) { if (!argv.length) throw new Error(\'new Date() is unavailable in a workflow script\'); return new target(...argv) } })', context)
// A value from here, as a value of the script's own realm: what an agent returns is data.
const into = (value) => (value === undefined ? undefined : vm.runInContext('JSON.parse', context)(JSON.stringify(value)))

async function agent(prompt, opts) {
  const o = opts || {}
  note('agent ' + o.label + ' · ' + o.agentType + ' · ' + o.model + ' · ' + o.phase)
  if (fatal.length) throw new Error('the simulation has failed already: ' + fatal[0])
  if (!o.schema) fail('`' + o.label + '` was launched without a schema: the stand-in models no free-text return')
  const kind = KINDS.find((k) => k.labels.test(String(o.label)))
  if (kind && kind.agentType !== o.agentType) fail('`' + o.label + '` was launched as ' + o.agentType + ', not as ' + kind.agentType)
  if (!kind && !(o.label in scripted)) fail('no agent is scripted for the label `' + o.label + '`')
  // An agent that dies returns nothing — whichever kind it is, and however often it is
  // tried. One that dies after its report did its work once, and returns nothing of it.
  const ends = endings[o.label]
  if (ends) used.add(o.label)
  if (ends === 'dies' || (ends === 'dies-after-report' && tried.has(o.label))) return null
  tried.add(o.label)
  let back
  try {
    back = (kind ? kind.play : reporter)(o.label, prompt, o)
  } catch (e) {
    if (!fatal.length) fatal.push('the stand-in for `' + o.label + '` threw: ' + ((e && e.stack) || e))
    throw e
  }
  if (ends === 'dies-after-report') return null
  // A field the scenario has this agent leave out — whatever its stand-in put there.
  const left = omits[o.label] || []
  if (left.length) used.add(o.label)
  for (const name of left) delete back[name]
  const why = conforms(o.schema, back, 'the return of `' + o.label + '`')
  // A return that lacks a field its schema REQUIRES, by a scripted omission: what the
  // runtime does then is what a probe exists to find out, so the scenario says it.
  if (why && left.some((name) => why.endsWith(' lacks `' + name + '`'))) {
    if (scenario.violation === 'handed') return into(back)
    if (scenario.violation === 'nothing') return null
    if (scenario.violation === 'throws') throw new Error('scripted: the runtime refused a return — ' + why)
    fail('`' + o.label + '` leaves out a field its schema requires, and the scenario does not say what the runtime does then (`violation`: handed, nothing or throws)')
  }
  if (why) fail(why + ': the runtime would not hand the script this return')
  return into(back)
}
async function parallel(thunks) {
  note('parallel ' + thunks.length + ' {')
  depth++
  const out = []
  for (const thunk of thunks) {
    try {
      out.push(await thunk())
    } catch (e) {
      // As the runtime has it: a thunk that throws is null in the result, and nothing rejects.
      swallowed.push(String((e && e.message) || e))
      out.push(null)
    }
  }
  depth--
  note('}')
  return out
}
const unavailable = (name) => () => fail('`' + name + '` is not modelled by the stand-in')

const source = readFileSync(script, 'utf8').replace(/^export const meta = /m, 'const meta = ')
const body = vm.runInContext('(async function (agent, parallel, pipeline, log, phase, args, budget, workflow) {\n' + source + '\n})', context, { filename: script })
let result = null
try {
  result = await body(agent, parallel, unavailable('pipeline'), (message) => { logs.push(String(message)) }, (title) => { note('phase ' + title) }, into(scenario.args), into({ total: null }), unavailable('workflow'))
} catch (e) {
  fatal.push('the script threw: ' + ((e && e.stack) || e))
}
const unused = Object.keys(scripted).concat(Object.keys(endings), Object.keys(omits), scenario.garbles || []).filter((label) => !used.has(label))
process.stdout.write(JSON.stringify({ result: result === undefined ? null : result, trace, ran, logs, reporters, swallowed, unused, fatal }) + '\n')
if (fatal.length) {
  process.stderr.write(fatal.join('\n') + '\n')
  process.exitCode = 1
}
