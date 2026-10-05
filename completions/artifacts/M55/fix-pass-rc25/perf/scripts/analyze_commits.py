#!/usr/bin/env python3
"""Commits versus gates: for each full green gate, did a commit follow before the same agent's next gate?
A green gate with no commit after it was superseded (the tree changed again and was re-gated).
Also commits per round and gate wall per commit.
Reads: OUT_DIR/gate_ledger.json, agents.json, calls.jsonl, and OUT_DIR/commits.txt — one line per commit of the
measured branch, "<hash> <committer date, strict ISO 8601> <subject>". Prints: counts, seconds and agent labels.
The round walls in `rw` (minutes) are the measured run's own, typed in from analyze_timeline.py's output."""
import json,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',))
D=E.out_dir()
led=json.load(open(D+"/gate_ledger.json"))
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
NAMES,_=E.runs()
CUT=E.cutoff()
commits=[]
for l in open(D+"/commits.txt"):
    h,iso,subj=l.rstrip('\n').split(' ',2)
    commits.append((datetime.datetime.fromisoformat(iso).timestamp(),h,subj))
commits=[c for c in commits if c[0]<CUT]
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
def owner(t):
    for a in A.values():
        if a['run'].startswith('wf_') and a.get('agent_type') in('build-fixer','build-executor','build-git') and a['start']<=t<=a['end']+5: return a
    return None
per=collections.Counter(); perag=collections.Counter()
for t,h,s in commits:
    a=owner(t)
    per[NAMES[a['run']] if a else 'outside the fix rounds']+=1
    if a: perag[a['agent']]+=1
print("commits on the branch up to the end of round 4: %d  →"%len(commits),dict(per))
full=[g for g in led if g['kind'] in('green','red','aborted') and g['start']<CUT and g.get('agent')]
sup=[]; used=0
byag=collections.defaultdict(list)
for g in full: byag[g['agent']].append(g)
for ag,gs in byag.items():
    gs.sort(key=lambda g:g['start'])
    for i,g in enumerate(gs):
        if g['kind']!='green': continue
        nxt=gs[i+1]['start'] if i+1<len(gs) else A[ag]['end']+60
        if any(g['end']-1<=t<=nxt for t,_,_ in commits): used+=1
        else: sup.append(g)
print("green full gates: %d followed by a commit, %d superseded (no commit before the agent's next gate) = %s"%(used,len(sup),hm(sum(g['dur'] for g in sup))))
for g in sup: print("   %s %-7s %-34s %s %4.0fs"%(g['log'][-6:],g['run'],(g['label'] or '')[:34],datetime.datetime.utcfromtimestamp(g['start']).strftime('%m-%d %H:%M'),g['dur']))
print("\nper round: commits, full-gate launches (green/red/aborted), gate wall, gate wall per commit, round wall per commit")
rw={'round1':87.3,'round2':337.8,'round3':272.4,'round4':715.4}
for r in('round1','round2','round3','round4'):
    gs=[g for g in full if g['run']==r]; k=collections.Counter(g['kind'] for g in gs); w=sum(g['dur'] for g in gs)
    print("   %-7s commits %2d  gates %2d (%d/%d/%d)  gate wall %s  → %.1f min gate per commit, %.1f min wall per commit"%(r,per[r],len(gs),k['green'],k['red'],k['aborted'],hm(w),w/60/max(1,per[r]),rw[r]/max(1,per[r])))
print("\nround 4 per fixer: commits, green/red gates, wall per commit")
for ag,n in perag.items():
    a=A[ag]
    if NAMES[a['run']]!='round4': continue
    k=collections.Counter(g['kind'] for g in byag[ag])
    print("   %-8s commits %2d  green %2d red %2d aborted %d  wall %s → %.1f min per commit"%(a['label'],n,k['green'],k['red'],k['aborted'],hm(a['wall']),a['wall']/60/n))
