// A stand-in for the Workflow runtime, for ONE invocation of the stabilization harness
// (.claude/workflows/stabilize.js) — what tooling-tests/stabilize_simulation.rs runs a
// stage under. It hands the committed script, unmodified, the globals the runtime hands a
// workflow script; every agent the script launches is a function here; and it prints what
// the invocation did as one line of JSON.
//
//   node stabilize-runtime.mjs <the harness script> <a scenario file>
//
// run from the root of the repository the stage works on. The scenario is the suite's:
// { args, checks, gate, held, builds, unreleased, scope, agents, record, wrongBinary,
// wrongPrevious, endings, omits, violation, garbles, relays, overwrites, meanwhile } — the
// invocation's arguments, and what the scripted agents say (below). WHAT A HELD COMMAND
// PRINTS is the scenario's too, and nothing else of one is: `gate` — what the candidate's
// gate shows red ({ steps, tests }), or { void: true } for an output that holds no verdict;
// `record.gate` — the same of a record's gate; `held` — by a held check's item, what the
// regression tool's ONE line says ({ status: green | red | void, reason }); `builds` —
// { fails: [candidate | previous] }; `unreleased` — the names of the held commands that do
// not end until the suite releases them. `wrongBinary` names the
// reporters that return another hash than the one they were handed: a driver that drove
// something else; `wrongPrevious` the verifiers that return another hash for the previous
// release's binary. `omits` names, by an agent's label, the fields it LEAVES OUT of its
// return — whatever its stand-in would have put there; a dotted path names a field
// BENEATH an object (`ran_on.previous`), and that field alone; where the call's schema requires
// such a field, what the runtime then does is not known to this stand-in and is the
// scenario's to say, as `violation`: `handed` (the script is handed the return as it is),
// `nothing` (it is handed nothing, on every try) or `throws` (the call throws, on every
// try). `garbles` names the git steps that relay their line with one character changed.
// `relays` says, by a git step's label, WHAT ELSE THAN THE LINE ITS COMMAND PRINTED the step
// hands back: `document` — the line the tool prints UNASKED, the state document or a
// refusal's prose in it: the prompt's command run without its `--digest` flag, which is what
// a stage read until 2026-10-07; `byte` — the digest with one character changed;
// `non-ascii` — the digest with one character replaced by one outside ASCII AND ITS HASH MADE
// TO FIT, so that nothing but the harness's own look at the line can tell it.
// `overwrites` gives, by an agent's label, a text that agent writes OVER its own report once
// the record script has written it — by a plain file write, as a shell's redirect does: a
// reporter has a shell, and a file at a report's path is whatever its last writer left.
// `meanwhile` gives, by an agent's label, a shell command that is run from the repository's
// root once that agent has done its work and before it returns: WHAT SOMEBODY ELSE DID WHILE
// THE STAGE RAN — a commit on the branch, a file written over.
// `endings` says how an agent ENDS where that is not "it returns its
// result and has written its report", by its label:
//   dies               it returns nothing, every time it is tried, and wrote no report
//   dies-after-report  it wrote its report — once — and returns nothing
//   halts-unreported   it returns the halt the scenario scripts, and wrote no report
//   unreported         it returns the result the scenario scripts, and wrote no report
//   acts-and-dies-once it did what it was launched for — a git step ran its one command —
//                      and returns nothing; tried again, it does it again and returns
//   returns-running    a step that asks after a held command, whose agent ends its turn
//                      on the FIRST line it was printed, whatever that line says
// (A halt WITH its report needs no ending: it is a scripted return like any other.)
//
// WHAT AN AGENT IS HERE. A function chosen by the call's label. Four kinds do what their
// definition gives the role to do in the repository, by running the commands their prompt
// spells — taken out of the prompt, never rebuilt here:
//
//   git:*      build-git           the ONE command of the step, run; its one line relayed —
//                                  AS THE RUNTIME RELAYS ONE (observed 2026-10-07, the relay
//                                  probe): an agent's structured return DECODES the line's
//                                  `\uXXXX` escapes, so a line that holds one comes back with
//                                  other bytes than it was printed with. A digest holds none.
//                                  A retry is the same command run again, and a retry that
//                                  tells a step's agent to look around the tree fails the run
//                                  A STEP THAT ASKS AFTER A HELD COMMAND (`hold-wait`) runs
//                                  its one command AGAIN while the line says `running`, as
//                                  its prompt has it — three times at most here
//   git:hold-start:*  build-git    the step's one command, and WHAT THE HELD COMMAND IS: the
//                                  step tool is real, and so are its supervisor, its lock
//                                  and its verdict; the long command it runs is the rig's
//                                  stand-in — `dev/gate`, `dev/regression-set` and `cargo`
//                                  print what this stand-in hands them by the environment
//                                  of that one call, from the scenario (above)
//   preflight  stabilize-preflight the two environment asserts a test can run, held as
//                                  the definition words them — the candidate is `HEAD`,
//                                  or, where the prompt says it is not, a commit `HEAD`
//                                  holds, and then no step that reads the working tree
//                                  may be listed; each check answered as the scenario
//                                  scripts it. IT BUILDS NOTHING AND HOLDS NO GATE: a
//                                  prompt that hands it either fails the run
//   scope      stabilize-scope     the round's doors written to the file the prompt names
//                                  and handed over by the prompt's `scope-set`
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
//   record:*   build-executor      the steps the prompt lists, in its order: the payload
//                                  written to the file the prompt names — by a file write,
//                                  as an agent's file tool writes one — and the batch
//                                  applied by the prompt's own `apply`, which holds the
//                                  file to its hash. IT RUNS NO GATE and makes no commit:
//                                  both are git steps (`git:hold-*:record-*`, `git:record:*`)
//
// AND EVERY AGENT RUNS THE READS ITS PROMPT NAMES — `dev/stabilize-record item`, `item-doors`,
// `untriaged`, `pending`, each as the prompt spells it: what a brief, a door list, the
// untriaged rows and a pending batch's subject are, an agent reads from the record by key,
// and a read that fails fails the run. What each read printed is reported (`reads`), and
// every prompt an agent was handed (`prompts`): the suite holds both.
//
// AND EVERY COMMAND AN AGENT RUNS IS TAKEN OUT OF ITS PROMPT, AS IT IS SPELLED THERE: a
// prompt that names a command the tool does not have, or a flag it does not take, fails
// the run at that command. EVERY PROMPT IS HELD, TOO: a code span that opens with `dev/`
// is one plain invocation of `dev/stabilize-step`, `dev/stabilize-record` or
// `dev/stabilize-probe` — plain words, nothing a shell composes with — or the run fails.
//
// Every other label is a reporter the scenario scripts by that label: its structured
// return is the scenario's, its report is written to the file its prompt names and handed
// to the real `dev/stabilize-record report` by the ONE command its prompt spells, and where
// the prompt carries a binary line the hash it returns is measured from the file at that
// path — or, for the role that has no definition, asked of the tool by the command its
// prompt spells.
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
const prompts = []
const reads = []
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
function sh(command, more) {
  // What ran on an agent's behalf, as its prompt spells it.
  ran.push(command)
  const out = spawnSync('sh', ['-c', command], { encoding: 'utf8', input: '', env: Object.assign({}, process.env, more || {}) })
  return { code: out.status, stdout: out.stdout || '', stderr: out.stderr || '' }
}
// A file, written as an agent's file tool writes one: no shell, and its directory made.
function fileTool(path, text) {
  mkdirSync(dirname(path), { recursive: true })
  writeFileSync(path, text)
}
// A PLAIN INVOCATION: a tool of the three, and words a shell reads as words and nothing else.
const PLAIN = /^dev\/stabilize-(step|record|probe)(?: (?:[A-Za-z0-9_.\/:=@%+,-]+|'[A-Za-z0-9 _.\/:=@%+,()-]*'))*$/
function heldToPlain(label, prompt) {
  for (const [, named] of prompt.split('\nSTABILIZE_PAYLOAD\n').filter((_, i) => i % 2 === 0).join('\n').matchAll(/`(dev\/[^`]*)`/g)) {
    if (!PLAIN.test(named)) fail('the prompt of `' + label + '` names `' + named + '`, which is no plain invocation of dev/stabilize-step, dev/stabilize-record or dev/stabilize-probe')
  }
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
// WHAT A HELD COMMAND PRINTS, AND HOW IT ENDS — handed to the rig's stand-ins for the long
// commands by the environment of the ONE call that starts them (`hold-start`): the step
// tool, its supervisor and its verdict are real, and what the gate, the regression set and
// cargo would have taken an hour to print is the scenario's.
const REGRESSION_VOID = { 'did-not-run': 10, 'build-previous': 11, 'build-candidate': 12, baseline: 13, swap: 14, incomplete: 15, evidence: 16 }
function heldEnv(command) {
  const flag = (name) => (new RegExp(' --' + name + ' (\\S+)').exec(command) || [])[1]
  const [name, kind] = [flag('name'), flag('kind')]
  const dir = process.env.SIM_HELD
  if (!dir) fail('the simulation names no directory for what a held command prints (SIM_HELD)')
  let prints = ''
  let exit = 0
  const env = { HOLD_RAN: dir + '/ran', HOLD_RELEASE: (scenario.unreleased || []).includes(name) ? dir + '/release-' + name : dir + '/released' }
  if (kind === 'gate') {
    const said = name.startsWith('record-') ? (scenario.record || {}).gate : scenario.gate
    prints = said && said.void ? 'gate: mode   quick (scripted by the simulation: no gate ran)\nPRE-CHECK: PASS\n' : gateOutput(said)
    exit = said && !said.void && (said.steps || []).length ? 1 : 0
  } else if (kind === 'regression') {
    const said = (scenario.held || {})[name.replace(/-c[0-9]+-a[0-9]+$/, '')] || { status: 'green' }
    const line = { tool: 'regression-set', status: said.status }
    if (said.status === 'void') Object.assign(line, { reason: said.reason, detail: 'scripted by the simulation: no test ran' })
    else Object.assign(line, { previous: { commit: flag('previous') }, candidate: { commit: flag('candidate') }, list: { path: flag('list'), commit: flag('candidate'), sha256: 'c'.repeat(64) }, counts: { differences: said.status === 'red' ? 1 : 0, not_on_list: said.status === 'red' ? 1 : 0, excluded: 0, excluded_by_no_row: 0 } })
    prints = JSON.stringify(line) + '\n'
    exit = said.status === 'green' ? 0 : said.status === 'red' ? 1 : REGRESSION_VOID[said.reason]
    if (exit == null) fail('the scenario scripts a regression set that is void for `' + said.reason + '`, which is no reason of that tool')
  } else if (kind === 'build') {
    const which = name.startsWith('build-previous') ? 'previous' : 'candidate'
    Object.assign(env, { SIM_BUILD_VERSION: flag('version') || '1.0.0-rc.25', SIM_BUILD_FAILS: ((scenario.builds || {}).fails || []).includes(which) ? 'yes' : '' })
  } else if (kind !== 'probe') fail('`' + command + '` starts a held command of a kind this stand-in has no command for')
  fileTool(dir + '/prints-' + name, prints)
  return Object.assign(env, { HOLD_PRINTS: dir + '/prints-' + name, HOLD_EXIT: String(exit) })
}
function treeState() {
  return 'branch ' + git('branch', '--show-current').stdout + '\n' + git('status', '--porcelain').stdout
}

// ---- a report, through the record script ----
// The command is the prompt's, whole: the ONE call that hands the report over, and in it the
// file the report is written to first (`--from`).
function writeReport(label, prompt, said) {
  const command = span(prompt, 'dev/stabilize-record report --run ')
  const file = command && / --from (\S+)/.exec(command)
  if (!file) fail('the prompt of `' + label + '` names no report to write, or no file to write it to')
  const name = / --reporter (\S+)/.exec(command)[1]
  // An agent that ends with no report wrote none — whichever kind it is.
  if (endings[label] === 'unreported' || endings[label] === 'halts-unreported') return null
  // A try after a transient failure writes no report that stands already — as the retry's
  // prompt has it.
  if (prompt.includes('RETRY after a transient failure') && reporters.includes(name)) return null
  fileTool(file[1], '# ' + label + '\n\nScripted by the simulation: no agent wrote this.\n\n' + said + '\n\n<!-- end of report -->\n')
  const out = sh(command)
  if (out.code !== 0) fail('the report of `' + label + '` was refused: ' + out.stderr)
  reporters.push(name)
  if ((scenario.overwrites || {})[label] != null) writeFileSync(out.stdout.trim(), scenario.overwrites[label])
  return out.stdout.trim()
}

// ---- what an agent reads by key ----
// Every read of the record its prompt names, run as spelled: the commands are the prompt's.
function readByKey(label, prompt) {
  for (const [, command] of prompt.matchAll(/`(dev\/stabilize-record (?:item|item-doors|untriaged|pending) [^`]+)`/g)) {
    const out = sh(command)
    if (out.code !== 0) fail('the read `' + command + '` that the prompt of `' + label + '` names was refused: ' + out.stderr)
    reads.push({ label, command, printed: out.stdout.trim() })
  }
}

// ---- the four kinds that act ----
// relayed — a line as an agent's structured return hands it back: every `\uXXXX` escape of
// it decoded into its character. What was observed of the runtime, and all that was.
function relayed(line) {
  return line.replace(/\\u([0-9a-fA-F]{4})/g, (_, hex) => String.fromCharCode(parseInt(hex, 16)))
}
function gitStep(label, prompt) {
  const first = prompt.split('\n').find((line) => line.startsWith('1. `')) || ''
  const command = first.slice(4, first.lastIndexOf('`'))
  // A probe's step is a command of the probe tool, and of no other; a stage's, of the step tool.
  const tool = label.startsWith('git:probe:') ? 'dev/stabilize-probe ' : 'dev/stabilize-step '
  if (!command.startsWith(tool)) fail('the git step `' + label + '` is not ONE command of ' + tool.trim() + ': ' + first)
  // A step's retry is its one command, run again — never a look around the tree.
  const again = prompt.split('\n\nRETRY')[1]
  if (again != null && !label.startsWith('git:probe:') && (/git (status|log|clean)/.test(again) || !again.includes('Run the ONE command above again'))) fail('the retry of the git step `' + label + '` is not told to run its one command again: ' + again)
  const how = (scenario.relays || {})[label]
  if (how) used.add(label)
  // The line the tool prints unasked: the command, without the flag that asks for a digest.
  const starts = command.startsWith('dev/stabilize-step hold-start ') ? heldEnv(command) : null
  let out = sh(how === 'document' ? command.replace(/ --digest \S+/, '') : command, starts)
  // A STEP THAT ASKS AFTER A HELD COMMAND asks again while its line says `running` — unless
  // its agent's turn ends on the first line.
  if (command.startsWith('dev/stabilize-step hold-wait ')) {
    if (!prompt.includes('RUN THE SAME COMMAND AGAIN')) fail('the step `' + label + '` asks after a held command, and is not told to ask again while it runs')
    if (endings[label] === 'returns-running') used.add(label)
    else for (let asked = 1; asked < 3 && /"status": ?"running"/.test(out.stdout); asked++) out = sh(command)
  }
  if (how === 'document' && out.stdout === sh(command, starts).stdout) fail('the git step `' + label + '` asks for no digest: its line is the same without `--digest`')
  // Handed back byte for byte: a relay that happened to be faithful is still not a digest.
  if (how === 'document') return { status: 'ran', line: out.stdout }
  if (how === 'byte') return { status: 'ran', line: out.stdout.replace('"status":"', '"status":"x') }
  if (how === 'non-ascii') {
    const body = out.stdout.replace(/\n$/, '').replace(/, "sha256": "[0-9a-f]{64}"\}$/, '}').replace('"status":"', '"status":"\u2014')
    return { status: 'ran', line: body.slice(0, -1) + ', "sha256": "' + createHash('sha256').update(body).digest('hex') + '"}\n' }
  }
  if ((scenario.garbles || []).includes(label)) {
    used.add(label)
    return { status: 'ran', line: out.stdout.replace('"status"', '"Status"') }
  }
  if (!out.stdout) return { status: 'halted', halt: { root_cause: 'the command printed no line', evidence: command + '\n' + out.stderr, tree_state: treeState(), recommendation: 'read the tree before the step is asked for again' } }
  // The step that begins an attempt writes the attempt's marker: a report the harness
  // launched, under the reporter the command names.
  if (command.startsWith('dev/stabilize-step begin ') && out.code === 0) reporters.push(/ --reporter (\S+)/.exec(command)[1])
  // A PROBE'S STEP IS RELAYED BYTE FOR BYTE, as it always was here: its lines are the probe
  // tool's, which are no digests, and what the runtime does to one is what the probes
  // themselves went and observed.
  return { status: 'ran', line: label.startsWith('git:probe:') ? out.stdout : relayed(out.stdout) }
}

function preflight(label, prompt) {
  const asked = /^Candidate: label (\S+), commit ([0-9a-f]{40}) /m.exec(prompt)
  const runDir = /may hold untracked files under `([^`]+)\/` and nothing else/.exec(prompt)
  if (!asked || !runDir) fail('the preflight\'s prompt names no candidate, or no run directory')
  const [, candidate, sha] = asked
  // NO BUILD AND NO GATE IS THE PREFLIGHT'S: both are commands the step tool holds.
  if (/git archive|dev\/gate|cargo build/.test(prompt)) fail('the preflight\'s prompt hands it a build or a gate: both are held by steps of the tool')
  // The environment asserts (stabilize-preflight.md): the candidate is `HEAD`, and the
  // tree's status shows what the prompt says to expect and no more. What it prints for an
  // untracked directory is the directory, so this does not tell a report from another
  // untracked file under the run's directory.
  // The candidate is `HEAD` — or, where the prompt says that it is not, a commit `HEAD`
  // holds: then a step that reads the working tree (a check) is not one the definition
  // lets this call list.
  const head = git('rev-parse', 'HEAD').stdout
  const status = git('status', '--porcelain').stdout
  const stray = status.split('\n').filter((line) => line && !line.startsWith('?? ' + runDir[1] + '/'))
  const notHead = prompt.includes('THE CANDIDATE IS NOT `HEAD` HERE') && prompt.includes('commit ' + sha + ' must be an ancestor of `HEAD`')
  const readsTheTree = /^ {3}- [a-z0-9-]+: /m.test(prompt)
  const failed = notHead && git('merge-base', '--is-ancestor', sha, 'HEAD').code !== 0 ? 'the candidate ' + sha + ' is no commit that `HEAD` (' + head + ') holds'
    : notHead && readsTheTree ? 'the candidate ' + sha + ' is not `HEAD`, and this call lists a step that reads the working tree'
      : !notHead && head !== sha ? '`git rev-parse HEAD` is ' + head + ', and the candidate is ' + sha + ': the tree is not the candidate'
        : stray.length ? '`git status --porcelain` shows more than untracked files under ' + runDir[1] + '/: ' + stray.join(' · ') : null
  if (failed) {
    return { status: 'halted', halt: { root_cause: 'an environment assert failed: ' + failed, evidence: 'git rev-parse HEAD: ' + head + '\ngit status --porcelain:\n' + status, tree_state: treeState(), recommendation: 'nothing built on an unasserted environment is evidence' }, report: writeReport(label, prompt, 'Halted: ' + failed) }
  }
  const back = { status: 'ready', checks: [] }
  if (/\d+\. the trial image, built and verified/.test(prompt)) back.image = { tag: 'jigc-trial:' + candidate, verified: true, failed: [] }
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
  const file = command && / --from (\S+)/.exec(command)
  if (!file || !between || !scenario.scope) fail('the scope step is not scripted, or its prompt names no `scope-set` fed from a file and no two commits')
  fileTool(file[1], JSON.stringify({ included: scenario.scope.included, excluded: scenario.scope.excluded }))
  const out = sh(command)
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
// The payload: the text that stands between the two marker lines, written to the file the
// step names as a file tool writes one — and ONE newline after it. `altered`: an agent
// that did not write the text it was handed.
function writePayload(step, altered) {
  const file = span(step[0], '/')
  const [from, to] = [step.indexOf('STABILIZE_PAYLOAD'), step.lastIndexOf('STABILIZE_PAYLOAD')]
  if (!file || !step[0].startsWith('Write the file `') || from < 1 || to === from) fail('a payload names no file, or does not stand between its two marker lines: ' + step[0])
  const text = step.slice(from + 1, to).join('\n')
  fileTool(file, (altered ? text.replace('"', '" ') : text) + '\n')
  return file
}
function recordStep(label, prompt) {
  const lines = prompt.split('\n')
  const from = lines.findIndex((line) => line.startsWith('Run th'))
  const to = lines.findIndex((line) => line.startsWith('YOU MAKE NO COMMIT'))
  if (from < 0 || to < 0 || !/^BRANCH: (\S+?)[ ,]/m.test(prompt)) fail('the record step `' + label + '` is not the prompt a record step is handed')
  const steps = numbered(lines, from, to)
  const halted = (cause, evidence, refused) => Object.assign({ status: 'halted', halt: { root_cause: cause, evidence, tree_state: treeState(), recommendation: 'the record step stopped where it stood: it commits nothing' } }, refused ? { refused } : {})
  const ruled = scenario.record || {}
  let applied = false
  for (const step of steps) {
    if (step[0].startsWith('Write ')) {
      writePayload(step, ruled.payload === 'altered')
      continue
    }
    const command = span(step[0], 'dev/')
    if (!command) fail('a step of the record step `' + label + '` spells no command: ' + step[0])
    // THE GATE IS NOT THE EXECUTOR'S: the harness holds it by steps of its own.
    if (!command.startsWith('dev/stabilize-record apply ')) fail('the record step `' + label + '` runs a command that is not its batch: ' + command)
    applied = true
    // A record step that did nothing and says it did: the batch is never applied.
    if (ruled.apply === 'skipped') continue
    const out = sh(command)
    // The word of the refusal, read off the one line the script printed on stderr.
    if (out.code !== 0) return halted('the batch of the record step was refused: ' + out.stderr.trim(), command, (/: refused ([a-z-]+): /.exec(out.stderr) || [])[1])
  }
  if (!applied) fail('the record step `' + label + '` names no batch to apply')
  return { status: 'applied' }
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
      writePayload(step, false)
      continue
    }
    const command = span(step[0], 'dev/stabilize-probe hash ')
    if (!command) fail('a step of the probe\'s `' + label + '` is neither its payload nor its hash: ' + step[0])
    line = sh(command).stdout
  }
  return line ? { status: 'ran', line } : fail('the probe\'s `' + label + '` hashed nothing')
}
// ---- a scripted reporter ----
function reporter(label, prompt, opts) {
  used.add(label)
  const back = Object.assign({}, scripted[label])
  // The binary it is handed: on its BINARY: line — and the hash it asserts is ASKED OF THE
  // TOOL, by the ONE command its prompt spells, on that line or — for the role that has no
  // definition — on a line of its own. The hash it returns is that tool's, not copied.
  const binary = /^BINARY: candidate `([^`]+)` sha256 [0-9a-f]{64} \(commit [0-9a-f]{40}, label [^)]+\)(?:; previous release `([^`]+)`)?/m.exec(prompt)
  const spelled = span(prompt, 'dev/stabilize-step hash ')
  if (binary && !spelled) fail('the prompt of `' + label + '` hands it a binary, and no call that asserts its hash')
  const hashed = spelled ? sh(spelled) : null
  if (hashed && hashed.code !== 0) fail('the hash the prompt of `' + label + '` asks of the tool was refused: ' + hashed.stderr)
  // The call asserts THE FILE THE AGENT IS HANDED: one that names another would pass on a hash of something else.
  if (hashed && binary && JSON.parse(hashed.stdout).content_sha256 !== sha256Of(binary[1])) fail('the hash call the prompt of `' + label + '` spells is not of the binary on its BINARY: line')
  if (spelled) back.asserted_sha256 = (scenario.wrongBinary || []).includes(label) ? '0'.repeat(64) : JSON.parse(hashed.stdout).content_sha256
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
  prompts.push({ label: String(o.label), prompt: String(prompt) })
  if (fatal.length) throw new Error('the simulation has failed already: ' + fatal[0])
  heldToPlain(String(o.label), String(prompt))
  if (!o.schema) fail('`' + o.label + '` was launched without a schema: the stand-in models no free-text return')
  const kind = KINDS.find((k) => k.labels.test(String(o.label)))
  if (kind && kind.agentType !== o.agentType) fail('`' + o.label + '` was launched as ' + o.agentType + ', not as ' + kind.agentType)
  if (!kind && !(o.label in scripted)) fail('no agent is scripted for the label `' + o.label + '`')
  // An agent that dies returns nothing — whichever kind it is, and however often it is
  // tried. One that dies after its report did its work once, and returns nothing of it.
  const ends = endings[o.label]
  if (ends) used.add(o.label)
  if (ends === 'dies' || (ends === 'dies-after-report' && tried.has(o.label))) return null
  const firstTry = !tried.has(o.label)
  tried.add(o.label)
  let back
  try {
    readByKey(String(o.label), String(prompt).split('\n\nRETRY')[0])
    back = (kind ? kind.play : reporter)(o.label, prompt, o)
  } catch (e) {
    if (!fatal.length) fatal.push('the stand-in for `' + o.label + '` threw: ' + ((e && e.stack) || e))
    throw e
  }
  const others = (scenario.meanwhile || {})[o.label]
  if (others && firstTry) {
    used.add(o.label)
    const did = sh(others)
    if (did.code !== 0) fail('what the scenario has somebody do while `' + o.label + '` ran failed: ' + did.stderr)
  }
  if (ends === 'dies-after-report' || (ends === 'acts-and-dies-once' && firstTry)) return null
  // A field the scenario has this agent leave out — whatever its stand-in put there. A
  // dotted path is a field beneath an object: that field alone leaves, the object stays.
  const left = omits[o.label] || []
  if (left.length) used.add(o.label)
  for (const name of left) {
    const path = name.split('.')
    const last = path.pop()
    const within = path.reduce((at, part) => (at == null ? at : at[part]), back)
    if (within != null) delete within[last]
  }
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
const unused = Object.keys(scripted).concat(Object.keys(endings), Object.keys(omits), Object.keys(scenario.relays || {}), Object.keys(scenario.meanwhile || {}), scenario.garbles || []).filter((label) => !used.has(label))
process.stdout.write(JSON.stringify({ result: result === undefined ? null : result, trace, prompts, reads, ran, logs, reporters, swallowed, unused, fatal }) + '\n')
if (fatal.length) {
  process.stderr.write(fatal.join('\n') + '\n')
  process.exitCode = 1
}
