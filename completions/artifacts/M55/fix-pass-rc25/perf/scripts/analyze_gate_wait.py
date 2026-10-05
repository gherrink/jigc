#!/usr/bin/env python3
"""What the launching agent did while each full gate ran: seconds inside the gate call itself (foreground gate),
inside a foreground poll/sleep loop, inside any other tool call, and between calls (model time).
Reads: OUT_DIR/agents.json, calls.jsonl, gate_ledger.json. Prints: seconds and shares per bucket."""
import json,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','CUTOFF_UTC',))
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
led=json.load(open(D+"/gate_ledger.json"))
CUT=E.cutoff()
by=collections.defaultdict(list)
for c in C:
    if c.get('t1'): by[c['agent']].append(c)
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
tot=collections.Counter(); perrun=collections.defaultdict(collections.Counter); other_cls=collections.Counter(); n=0
for g in led:
    if g['start']>=CUT or g['kind'] not in('green','red') or not g.get('agent'): continue
    n+=1; s,e=g['start'],g['end']
    cov=[]  # (a,b,kind)
    for c in by[g['agent']]:
        a=max(s,c['t0']); b=min(e,c['t1'])
        if b<=a: continue
        k='in-gate-call' if c['cls']=='gate-full' and not c.get('bg') else 'poll' if (c['cls']=='poll' or c.get('detail',{}).get('has_sleep')) else 'other-tool'
        cov.append((a,b,k,c['cls']))
    # resolve overlaps by priority
    pts=sorted(set([s,e]+[x for a,b,_,_ in cov for x in(a,b)]))
    for a,b in zip(pts,pts[1:]):
        ks=[(k,cl) for (x,y,k,cl) in cov if x<=a and y>=b]
        if not ks: k='model'
        elif any(k=='in-gate-call' for k,_ in ks): k='in-gate-call'
        elif any(k=='other-tool' for k,_ in ks):
            k='other-tool'
            for kk,cl in ks:
                if kk=='other-tool': other_cls[cl]+=b-a
        else: k='poll'
        tot[k]+=b-a; perrun[g['run']][k]+=b-a
print("full gates considered: %d, wall %s"%(n,hm(sum(tot.values()))))
for k,v in tot.most_common(): print("  %-14s %s  %4.1f%%"%(k,hm(v),100*v/sum(tot.values())))
for r,cn in perrun.items(): print("  run %-8s "%r+"  ".join("%s=%s"%(k,hm(v)) for k,v in cn.most_common()))
print("  other-tool classes during a gate:",", ".join("%s=%s"%(k,hm(v)) for k,v in other_cls.most_common(8)))
