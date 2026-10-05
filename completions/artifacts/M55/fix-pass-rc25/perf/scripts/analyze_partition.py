#!/usr/bin/env python3
"""Exclusive partition of every agent's wall clock: each second goes to exactly one bucket, by priority
  full gate running (from the gate logs' own start/end)  >  a foreground tool call (by class)  >  idle after end_turn  >  model.
Printed per run, plus the session-wide totals. `-a` adds one line per fix-round agent.
Reads: OUT_DIR/agents.json, calls.jsonl, gate_ledger.json. Writes OUT_DIR/partition_by_agent.json.
Prints: agent-hours per bucket and run."""
import json,os,collections,datetime,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),usage='[-a]')
D=E.out_dir()
AG=json.load(open(D+"/agents.json")); A={a['agent']:a for a in AG}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
led=json.load(open(D+"/gate_ledger.json"))
NAMES,ORDER=E.runs()
CUT=E.cutoff()
GROUP={'gate-full':'gate (not in a surviving log)','gate-quick':'quick gate','gate-report':'other','cargo-build-release':'cargo build','cargo-build-debug':'cargo build','cargo-install':'cargo build','cargo-test':'scoped tests','cargo-clippy':'clippy/fmt','cargo-fmt':'clippy/fmt','cargo-other':'cargo build','poll':'sleep/poll (no gate running)','rig':'rig + driving the binary','drive':'rig + driving the binary','script':'rig + driving the binary','trial-tooling':'trial tooling','docker':'docker','codex':'codex','git':'git','python':'python (patch scripts)','read':'shell reads','file-read':'Read/Write/Edit','file-write':'Read/Write/Edit','file-edit':'Read/Write/Edit','spawn':'waiting on a spawned agent/workflow'}
def hm(s): return "%d:%02d"%(s//3600,(s%3600)//60)
gates=collections.defaultdict(list)
for g in led:
    if g.get('agent') and g['kind'] in('green','red','aborted'): gates[g['agent']].append((g['start'],g['end']))
calls=collections.defaultdict(list)
for c in C:
    if c.get('t1'): calls[c['agent']].append(c)
PR=['gate']+list(dict.fromkeys(GROUP.values()))+['other tool','idle','model']
def part(a):
    s=a['start']; e=min(a['end'],CUT)
    if e<=s: return collections.Counter()
    iv=[(max(s,x),min(e,y),'gate') for x,y in gates[a['agent']]]
    for c in calls[a['agent']]:
        k=GROUP.get(c['cls'],'other tool')
        if c['cls']=='poll' and False: pass
        iv.append((max(s,c['t0']),min(e,c['t1']),k))
    for t0,g in a.get('idle_gaps',[]): iv.append((max(s,t0),min(e,t0+g),'idle'))
    iv=[x for x in iv if x[1]>x[0]]
    pts=sorted(set([s,e]+[p for x in iv for p in x[:2]]))
    out=collections.Counter()
    for x,y in zip(pts,pts[1:]):
        ks=set(k for (i,j,k) in iv if i<=x and j>=y)
        k='model'
        for p in PR:
            if p in ks: k=p; break
        out[k]+=y-x
    return out
tot=collections.Counter(); per=collections.defaultdict(collections.Counter); pa={}
for a in AG:
    if a['start']>=CUT: continue
    p=part(a); pa[a['agent']]=p
    per[NAMES[a['run']]].update(p)
    if a['run']!='main': tot.update(p)
runs=[NAMES[k] for k in ORDER if k!='main']
keys=[k for k,_ in tot.most_common()]
print("%-36s"%'bucket (agent-hours, exclusive)'+"".join("%9s"%r[:9] for r in runs)+"%10s %6s"%('ALL','share'))
T=sum(tot.values())
for k in keys:
    print("%-36s"%k+"".join("%9s"%(hm(per[r][k]) if per[r][k]>=30 else '.') for r in runs)+"%10s %5.1f%%"%(hm(tot[k]),100*tot[k]/T))
print("%-36s"%'TOTAL agent wall'+"".join("%9s"%hm(sum(per[r].values())) for r in runs)+"%10s"%hm(T))
fx=collections.Counter()
for r in('round1','round2','round3','round4'): fx.update(per[r])
F=sum(fx.values())
print("\nfix rounds 1-4 only (serial, so agent-hours = wall): %s"%hm(F))
for k,v in fx.most_common(): print("   %-36s %7s %5.1f%%"%(k,hm(v),100*v/F))
print("\norchestrator's own session:"," | ".join("%s=%s"%(k,hm(v)) for k,v in per['orchestrator'].most_common(6)))
if '-a' in sys.argv:
    for a in sorted(AG,key=lambda a:a['start']):
        if NAMES[a['run']] in('round1','round2','round3','round4') and a['agent'] in pa:
            p=pa[a['agent']]; w=sum(p.values())
            print("  %-7s %-34s %6s | "%(NAMES[a['run']],(a.get('label') or '')[:34],hm(w))+"  ".join("%s %s (%.0f%%)"%(k.split(' ')[0],hm(v),100*v/w) for k,v in p.most_common(5)))
json.dump({k:dict(v) for k,v in pa.items()},open(D+"/partition_by_agent.json","w"))
