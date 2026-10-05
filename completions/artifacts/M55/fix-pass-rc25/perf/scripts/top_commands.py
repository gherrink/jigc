#!/usr/bin/env python3
"""The N longest single Bash commands of the run as a markdown table (duration, class, run, agent, start UTC, what),
and the N longest that are neither a gate nor a wait on one. Text is the command's own description, sanitized.
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: two markdown tables separated by a blank line; each row carries
the agent label and the first 80 characters of the description the command was launched with.
  usage: top_commands.py [N=30]"""
import json,os,datetime,sys,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),usage='[N=30]')
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
NAMES=dict(E.runs()[0],single='single')
CUT=E.cutoff()
N=int(sys.argv[1]) if len(sys.argv)>1 else 30
def real(c): return c.get('bg_dur') if c.get('bg') and c.get('bg_dur') is not None else (c.get('dur') or 0)
def ms(s): return "%d:%02d"%(s//60,s%60)
rows=sorted([c for c in C if c['tool']=='Bash' and c['t0']<CUT],key=lambda c:-real(c))
def table(rs):
    print("| # | min:s | class | run | agent | start (UTC) | what the command said it was doing |\n|---|---|---|---|---|---|---|")
    for i,c in enumerate(rs,1):
        a=A[c['agent']]
        print("| %d | %s | %s%s | %s | %s | %s | %s |"%(i,ms(real(c)),c['cls'],' (bg)' if c.get('bg') else '',NAMES[c['run']],(a.get('label') or '').replace('|','/')[:30],datetime.datetime.utcfromtimestamp(c['t0']).strftime('%d %H:%M'),(c.get('desc') or '').replace('|','/')[:80]))
table(rows[:N])
print()
table([c for c in rows if c['cls'] not in('gate-full','poll') and not re.search(r'gate',c.get('desc','') ,re.I)][:N//2])
