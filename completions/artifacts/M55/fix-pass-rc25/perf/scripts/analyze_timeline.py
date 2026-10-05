#!/usr/bin/env python3
"""The session's wall clock as a timeline: when a subagent/workflow was running, when only the orchestrator
was working, and when nothing was (waiting on the human or on nothing). Lists the gaps > 10 min.
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: times, wall sums and counts."""
import json,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',))
D=E.out_dir()
AG=json.load(open(D+"/agents.json"))
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
NAMES,_=E.runs()
CUT=E.cutoff()
def hm(s): return "%d:%02d"%(s//3600,(s%3600)//60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M')
main=[a for a in AG if a['run']=='main'][0]
subs=[a for a in AG if a['run']!='main' and a['start']<CUT]
S0=main['start']; E0=CUT
def union(iv):
    iv=sorted(iv); m=[]
    for a,b in iv:
        if m and a<=m[-1][1]: m[-1][1]=max(m[-1][1],b)
        else: m.append([a,b])
    return m
sub_u=union([(a['start'],min(a['end'],E0)) for a in subs])
sub_t=sum(b-a for a,b in sub_u)
# orchestrator activity: its own tool calls + the model time adjacent (approximate by its non-idle time)
print("session start %s — end of the run (round-4 push) %s : wall %s"%(t(S0),t(E0),hm(E0-S0)))
print("some subagent/workflow running: %s (%.0f%%)"%(hm(sub_t),100*sub_t/(E0-S0)))
print("sum of subagent walls: %s  → average concurrency while anything ran: %.2f"%(hm(sum(min(a['end'],E0)-a['start'] for a in subs)),sum(min(a['end'],E0)-a['start'] for a in subs)/sub_t))
# gaps
gaps=[]; prev=S0
for a,b in sub_u:
    if a-prev>0: gaps.append((prev,a))
    prev=b
if E0-prev>0: gaps.append((prev,E0))
print("no subagent running: %s in %d gaps; the ones > 10 min:"%(hm(sum(b-a for a,b in gaps)),len(gaps)))
mc=sorted([c for c in C if c['run']=='main'],key=lambda c:c['t0'])
for a,b in gaps:
    if b-a>600:
        n=sum(1 for c in mc if a<=c['t0']<b)
        print("   %s → %s  %s   orchestrator tool calls in the gap: %d"%(t(a),t(b),hm(b-a),n))
print("\nruns in order (start, end, wall) and the gap before each:")
runs=collections.defaultdict(list)
for a in subs:
    if a['run'].startswith('wf_'): runs[a['run']].append(a)
prev=None
for r,l in sorted(runs.items(),key=lambda kv:min(a['start'] for a in kv[1])):
    s=min(a['start'] for a in l); e=max(a['end'] for a in l)
    print("   %-14s %s → %s  wall %s  gap before %s"%(NAMES[r],t(s),t(e),hm(e-s),hm(max(0,s-prev)) if prev else '-'))
    prev=max(prev or 0,e)
