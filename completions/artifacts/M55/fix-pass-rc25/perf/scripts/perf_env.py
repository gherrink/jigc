#!/usr/bin/env python3
"""Shared start-up of the instrument's scripts: every input that is a place or an identity of ONE measured
session is read here from the environment, so no script carries a host path, a session id or a run id.

Reads: the environment variables below and, for RUNS_FILE, that one small JSON file.
Prints: nothing of its own — a script's opening comment on --help, and a usage error (exit 2) when a
variable or an argument the script needs is missing.

  SESSION_DIR  the measured Claude Code session's transcript directory — the one holding subagents/ and
               workflows/; the session's own transcript is the file <SESSION_DIR>.jsonl beside it
  OUT_DIR      the directory the derived datasets are written to and read back from (agents.json,
               calls.jsonl, gate_logs.json, …). Keep it outside the repository: calls.jsonl holds command
               heads taken from private transcripts
  RUNS_FILE    a JSON object naming the session's workflow runs in run order, run id -> the short name the
               tables use for it: {"wf_<run id>": "round1", "wf_<run id>": "round2"}
  CUTOFF_UTC   the instant the measured run ended, ISO 8601 (2026-01-31T07:05:00Z); calls, agents and
               gates after it are left out
  WORK_DIR     the scratch directory of the fresh suite run (clone/, suite/, shim/ — see measure_suite.sh)
"""
import os,sys,json,datetime
VARS={'SESSION_DIR':"the session's transcript directory (holds subagents/ and workflows/)",
      'OUT_DIR':"where the derived datasets are written and read back; keep it outside the repository",
      'RUNS_FILE':'JSON object, in run order: {"wf_<run id>": "<short name>", ...}',
      'CUTOFF_UTC':"ISO 8601 instant the measured run ended; everything later is left out",
      'WORK_DIR':"scratch directory of the fresh suite run (clone/, suite/, shim/)"}
def _usage(write,env,usage):
    write(("usage: "+"".join(v+"=... " for v in env)+os.path.basename(sys.argv[0])+" "+usage).rstrip()+"\n")
    for v in env: write("  %-11s %s\n"%(v,VARS[v]))
def start(doc,env=(),args=0,usage=''):
    """--help prints the calling script's opening comment and its inputs; a missing input is a usage error."""
    if '-h' in sys.argv[1:] or '--help' in sys.argv[1:]:
        print((doc or '').strip()); _usage(sys.stdout.write,env,usage); sys.exit(0)
    missing=[v for v in env if not os.environ.get(v)]
    if missing or len(sys.argv)-1<args:
        sys.stderr.write("%s: %s\n"%(os.path.basename(sys.argv[0]),("not set: "+", ".join(missing)) if missing else "missing argument"))
        _usage(sys.stderr.write,env,usage); sys.exit(2)
def need(var):
    v=os.environ.get(var)
    if not v:
        sys.stderr.write("%s: not set: %s\n"%(os.path.basename(sys.argv[0]),var)); _usage(sys.stderr.write,(var,),''); sys.exit(2)
    return v
def session_dir(): return os.path.abspath(need('SESSION_DIR'))
def out_dir(): return os.path.abspath(need('OUT_DIR'))
def work_dir(): return os.path.abspath(need('WORK_DIR'))
def runs():
    """(NAMES, ORDER): run id -> short name, and the run ids in run order; the single subagents and the
    orchestrating session follow the workflow runs, as 'single' and 'main'."""
    m=json.load(open(need('RUNS_FILE')))
    names=dict(m); names.update(single='singles',main='orchestrator')
    return names,list(m)+['single','main']
def cutoff():
    d=datetime.datetime.fromisoformat(need('CUTOFF_UTC').replace('Z','+00:00'))
    if d.tzinfo is None: d=d.replace(tzinfo=datetime.timezone.utc)
    return d.timestamp()
