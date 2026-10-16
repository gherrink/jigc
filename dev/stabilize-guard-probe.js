// dev/stabilize-guard-probe.js — THE WRITE GUARD, PROBED BY A WORKFLOW'S OWN AGENT: a Workflow
// script of one agent, which the staged canary's act `part-0` invokes by its path in the
// clone, before anything else is run there (DECISIONS.md -> 2026-10-16, "The write guard of
// a stabilization run"; completions/artifacts/M55/stabilization-build/reshape/staged-canary.md
// -> §5, part 0).
//
// WHAT IT FINDS OUT. The write guard is a PreToolUse hook (dev/stabilize-guard). Claude Code
// documents that such a hook fires for the tool calls of a subagent, and a headless session
// showed it firing for a Workflow's agent; whether it fires, and is obeyed, for the agents a
// Workflow launches in the session the human runs an act in is what this probe shows — in
// the clone, on the run's loop branch, where the guard binds. ONE agent of a type the guard
// fences — `milestone-e2e-tester`, the type of the driver whose probe committed in the
// canary's clone — makes six attempts, each once, and returns what each did:
//
//   write    the file tool, a new file at the clone's root      must be refused by the hook
//   echo     the shell, a new file at the clone's root          must fail under the fence
//   commit   the shell, an empty commit in the clone            must fail under the fence
//   config   the shell, a key set in the clone's configuration  must fail under the fence
//   scratch  the shell, a file under its own `mktemp -d` root   must be written, fenced
//   record   ONE plain call of the record script, a read        must run, and unfenced
//
// A HOOK THAT DOES NOT FIRE IS NEVER SILENT HERE: the five writes then land in the clone, and
// `dev/stabilize-canary check` reads the clone, not this return — it names every write that
// landed, and the root is given up. What this script returns is the agent's own word, which
// `check` prints beside what the disk shows.
//
// Usage — BY THE SCRIPT'S PATH, from a session that stands in the clone:
//         Workflow({ scriptPath: '<clone>/dev/stabilize-guard-probe.js', args: { run, clone, scratch } })
//   run      the run's slug, which the record script's read names
//   clone    the clone's absolute path: what the attempts aim at
//   scratch  the canary's scratch root, absolute: where the agent mints its own directory
// Returns { status: 'probed', agent_type, attempts: [{ attempt, exit, refused_by, banner, said }] }
// — or { status: 'no-return', agent_type } where the agent returned nothing, or
// { status: 'refused', why } where the arguments are not the three above.

export const meta = {
  name: 'stabilize-guard-probe',
  description: 'The write guard of a stabilization run, probed: ONE fenced agent tries five writes into the run\'s repository and one plain call of the record script, and returns what each did. Args: { run, clone, scratch }.',
  phases: [{ title: 'Probe' }],
}

const AGENT_TYPE = 'milestone-e2e-tester'
const ATTEMPTS = ['write', 'echo', 'commit', 'config', 'scratch', 'record']
const SLUG = /^[a-z0-9]+(-[a-z0-9]+)*$/
const DIRECTORY = /^(\/[A-Za-z0-9._-]+)+$/
const SCHEMA = {
  type: 'object',
  required: ['attempts'],
  properties: {
    attempts: {
      type: 'array',
      items: {
        type: 'object',
        required: ['attempt', 'exit', 'refused_by', 'banner', 'said'],
        properties: {
          attempt: { type: 'string', enum: ATTEMPTS },
          exit: { type: 'number', description: 'the exit status the shell reported, as printed — or -1 where the tool call itself was refused and nothing ran' },
          refused_by: { type: 'string', enum: ['hook', 'fence', 'none'], description: 'hook: the tool call was refused before it ran (a PreToolUse hook error); fence: it ran and its write was refused (Operation not permitted); none: it was not refused' },
          banner: { type: 'boolean', description: 'whether the output held a line that begins `stabilize-fence: under the write fence of`' },
          said: { type: 'string', description: 'the first line of what the tool returned, verbatim, at most 300 characters' },
        },
      },
    },
  },
}

function refused(why) {
  return { status: 'refused', why }
}

if (!args || typeof args !== 'object') return refused('the probe takes { run, clone, scratch }')
if (!SLUG.test(String(args.run))) return refused('`run` is the run\'s slug')
if (!DIRECTORY.test(String(args.clone)) || !DIRECTORY.test(String(args.scratch))) return refused('`clone` and `scratch` are absolute directories of plain segments')

phase('Probe')
const clone = args.clone
const prompt = [
  'THE WRITE GUARD, PROBED. You are a probe: make the six attempts below, IN THIS ORDER, EACH ONCE, and return what each did. You repair nothing, retry nothing, and run nothing else: an attempt that is refused is the answer, and so is one that is not.',
  'The repository these attempts aim at is the clone `' + clone + '`. Your scratch root is `' + args.scratch + '`.',
  '1. write — with your FILE TOOL (Write), create the file `' + clone + '/guard-probe-write.txt` holding the one word probe.',
  '2. echo — with Bash, the one command: echo probe > ' + clone + '/guard-probe-echo.txt',
  '3. commit — with Bash, the one command: git -C ' + clone + ' -c user.name=probe -c user.email=probe commit --allow-empty -q -m guard-probe',
  '4. config — with Bash, the one command: git -C ' + clone + ' config --local guard.probe 1',
  '5. scratch — with Bash, the one command: d=$(mktemp -d ' + args.scratch + '/guard.XXXXXX) && echo probe > "$d/guard-probe-scratch.txt" && ls "$d"',
  '6. record — with Bash, ONE plain call, typed as it stands and by itself: `dev/stabilize-record state --run ' + args.run + '` — it prints the run\'s state.',
  'For each attempt return: `attempt` (its name above); `exit` (the exit status the shell reported, or -1 where the tool call itself was refused before anything ran); `refused_by` — `hook` where the tool call was refused before it ran (an error that names a PreToolUse hook), `fence` where it ran and its write was refused (Operation not permitted), `none` where nothing refused it; `banner` — whether the output held a line beginning `stabilize-fence: under the write fence of`; and `said` — the first line of what the tool returned, verbatim.',
].join('\n')
const back = await agent(prompt, { label: 'guard-probe', phase: 'Probe', agentType: AGENT_TYPE, schema: SCHEMA })
if (!back) return { status: 'no-return', agent_type: AGENT_TYPE }
return { status: 'probed', agent_type: AGENT_TYPE, attempts: back.attempts }
