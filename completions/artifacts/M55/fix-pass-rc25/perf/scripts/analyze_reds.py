#!/usr/bin/env python3
"""Red gates: every failing test with its own duration (a fence that fails in <1 s is a static scan),
how often each test reddened a gate, and the red→next-green cycle per red run.
Reads: OUT_DIR/gate_ledger.json. Prints: test names with counts and seconds; one line per red cycle."""
import json,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','CUTOFF_UTC',))
D=E.out_dir()
led=json.load(open(D+"/gate_ledger.json"))
CUT=E.cutoff()
led=[r for r in led if r['start']<CUT]
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
cnt=collections.Counter(); durs=collections.defaultdict(list)
for r in led:
    if r['kind']=='red':
        for n,d in r['fails'].items(): cnt[n]+=1; durs[n].append(d)
print("failing test → times it reddened a full gate, its own duration(s)")
for n,c in cnt.most_common(): print("  %2d  %-120s %s"%(c,n[:120],' '.join('%.2fs'%d for d in durs[n])))
print("\nred cycles (same agent: red → next green):")
tot=0; totfix=0
for i,r in enumerate(led):
    if r['kind'] not in('red',): continue
    nxt=[x for x in led[i+1:] if x.get('agent')==r.get('agent') and x['kind'] in('green','red')]
    if nxt:
        gap=nxt[0]['start']-r['end']; cyc=nxt[0]['end']-r['start']
        print("  %s %-9s %-26s red %4.0fs  → fix %4.0fs → next %s %4.0fs  (cycle %s)  steps=%s nfail=%d maxfail=%.1fs"%(r['log'][-6:],r['run'],(r['label'] or '')[:26],r['dur'],gap,nxt[0]['kind'],nxt[0]['dur'],hm(cyc),','.join(r['red_steps']),len(r['fails']),max(r['fails'].values() or [0])))
        tot+=r['dur']; totfix+=gap
    else:
        print("  %s %-9s %-26s red %4.0fs  → no later gate by this agent"%(r['log'][-6:],r['run'],(r['label'] or '')[:26],r['dur'])); tot+=r['dur']
print("red gate wall %s ; fix time between red and rerun %s"%(hm(tot),hm(totfix)))
