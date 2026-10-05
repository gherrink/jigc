#!/usr/bin/env python3
"""Step anatomy of the full gate across the pass: hygiene advisory, fmt+clippy+build (residual), test compile,
nextest, and how nextest's wall grew with the test count. Also the SLOW (>60 s) tests each log names.
Reads: OUT_DIR/gate_ledger.json, calls.jsonl. Prints: medians, sums and the names of the SLOW tests."""
import json,os,collections,datetime,statistics as st
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','CUTOFF_UTC',))
D=E.out_dir()
led=json.load(open(D+"/gate_ledger.json"))
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
CUT=E.cutoff()
full=[r for r in led if r['kind'] in('green','red') and r['start']<CUT]
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M')
rows=[]
for r in full:
    tc=r['compile'].get('test',0) or 0; nx=r.get('nextest_s') or 0; hy=r.get('hygiene_s',0)
    pre=r['dur']-hy-tc-nx
    rows.append((r,hy,pre,tc,nx))
def med(x): return st.median(x) if x else 0
print("full gates: %d"%len(rows))
print("median seconds: total %.0f | hygiene %.1f | fmt+clippy+build+doctest+overhead %.1f | test compile %.1f | nextest %.0f"%(med([r['dur'] for r,_,_,_,_ in rows]),med([h for _,h,_,_,_ in rows]),med([p for _,_,p,_,_ in rows]),med([c for _,_,_,c,_ in rows]),med([n for *_,n in rows])))
print("sums (h): total %.2f | hygiene %.2f | pre/other %.2f | test compile %.2f | nextest %.2f"%tuple(x/3600 for x in (sum(r['dur'] for r,*_ in rows),sum(h for _,h,*_ in rows),sum(p for _,_,p,_,_ in rows),sum(c for _,_,_,c,_ in rows),sum(n for *_,n in rows))))
print("nextest share of full-gate wall: %.1f%%"%(100*sum(n for *_,n in rows)/sum(r['dur'] for r,*_ in rows)))
# steps as the gate printed them (==> lines) where a transcript captured them
sd=collections.defaultdict(list)
for c in C:
    g=c.get('gate')
    if g and g.get('steps') and 'test' in g['steps']:
        for k,(v,s) in g['steps'].items(): sd[k].append(s)
print("step seconds as printed by the gate (n captured, min/median/max):")
for k in('fmt','clippy','build','test','doctest'):
    if sd[k]: print("  %-8s n=%3d  %4d / %4d / %4d"%(k,len(sd[k]),min(sd[k]),med(sd[k]),max(sd[k])))
print("\ngrowth of the test step (greens only; first/last five):")
gr=[(r,nx) for r,_,_,_,nx in rows if r['kind']=='green']
for r,nx in gr[:5]+gr[-5:]: print("  %s tests=%d nextest=%.0fs  (%.1f ms wall/test; %.2f thread-s/test at 16 threads)"%(t(r['start']),r['tests'],nx,1000*nx/r['tests'],16*nx/r['tests']))
a,b=gr[0],gr[-1]
dt=b[0]['tests']-a[0]['tests']; dn=b[1]-a[1]
print("  delta: +%d tests, +%.0f s nextest wall → %.2f s wall per added test = %.1f thread-s per added test (suite average before: %.2f thread-s)"%(dt,dn,dn/dt,16*dn/dt,16*a[1]/a[0]['tests']))
# by round
import itertools
print("\nnextest wall by round (median, greens): ")
byr=collections.defaultdict(list)
for r,nx in gr: byr[r['run']].append((r['tests'],nx))
for k,v in byr.items(): print("  %-8s n=%2d tests %d→%d nextest median %.0fs (min %.0f max %.0f)"%(k,len(v),v[0][0],v[-1][0],med([x[1] for x in v]),min(x[1] for x in v),max(x[1] for x in v)))
slow=collections.defaultdict(list)
for r,*_ in rows:
    for n,d in r['slow'].items(): slow[n].append(d)
print("\nSLOW tests (>60 s) named in the logs: n logs, min/median/max s")
for n,v in sorted(slow.items(),key=lambda kv:-med(kv[1])): print("  %3d  %5.0f/%5.0f/%5.0f  %s"%(len(v),min(v),med(v),max(v),n))
