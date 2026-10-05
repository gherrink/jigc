#!/usr/bin/env python3
"""The gate ledger: join each surviving gate log to the transcript call that launched it (by time),
classify each run (full green / full red / aborted mid-test / quick), list every red run's failing
steps and tests, and sum gate wall time per agent and per run. Writes OUT_DIR/gate_ledger.json.
Reads: OUT_DIR/agents.json, calls.jsonl, gate_logs.json. Prints: counts and wall sums per agent and per run; each
red run with its failing test names, the first compiler error lines of its log, and the launching command's
own description."""
import json,os,collections,datetime,sys,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),usage='[-steps]')
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
L=json.load(open(D+"/gate_logs.json"))
NAMES,_=E.runs()
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M:%S')
gcalls=[c for c in C if c['tool']=='Bash' and (c['cls'] in('gate-full','gate-quick') or 'gate-full' in c.get('classes',[]) or 'gate-quick' in c.get('classes',[]))]
gcalls.sort(key=lambda c:c['t0'])
led=[]
for g in L:
    best=None
    for c in gcalls:
        end=c.get('bg_end') or c.get('t1') or c['t0']
        if c['t0']-2<=g['start']<=max(end,c['t0']+30)+5:
            if best is None or c['t0']>best['t0']: best=c
    r=dict(g); r['dur']=g['end']-g['start']
    if best:
        a=A[best['agent']]
        r.update(agent=best['agent'],label=a.get('label'),run=NAMES[a['run']],bg=bool(best.get('bg')),desc=best.get('desc'),call_t0=best['t0'],call_dur=best.get('dur'),pre_gate_s=g['start']-best['t0'])
    else:
        # fallback: a gate detached with `nohup sh -c 'dev/gate …' &` hides behind another command class;
        # attribute it to the serial fix-round agent whose lifetime contains the log's start.
        own=[a for a in A.values() if a['run'].startswith('wf_') and a.get('agent_type') in('build-fixer','build-executor') and a['start']<=g['start']<=a['end']]
        if len(own)==1:
            a=own[0]; r.update(agent=a['agent'],label=a.get('label'),run=NAMES[a['run']],bg=True,desc='(detached with nohup … &)',call_t0=g['start'],detached=1)
        else: r.update(agent=None,label='(no launching call found)',run='?',bg=None,desc='')
    steps=r.get('steps',{}); full='test' in r['step_names']
    red=[k for k,v in steps.items() if v]
    sig=sum(1 for k in r['fails'])
    if not full: kind='quick-red' if red else 'quick'
    elif 'test' not in steps: kind='aborted'
    elif red: kind='red'
    else: kind='green'
    r['kind']=kind; r['red_steps']=red
    led.append(r)
json.dump(led,open(D+"/gate_ledger.json","w"))
CUT=E.cutoff()
led=[r for r in led if r['start']<CUT]
print("gate runs with a surviving log: %d  (kinds: %s)"%(len(led),dict(collections.Counter(r['kind'] for r in led))))
print("total gate wall: %s   full-run wall: %s   red full-run wall: %s   aborted: %s   quick: %s"%(hm(sum(r['dur'] for r in led)),hm(sum(r['dur'] for r in led if r['kind'] in('green','red'))),hm(sum(r['dur'] for r in led if r['kind']=='red')),hm(sum(r['dur'] for r in led if r['kind']=='aborted')),hm(sum(r['dur'] for r in led if r['kind'].startswith('quick')))))
by=collections.defaultdict(lambda:collections.Counter()); dur=collections.defaultdict(float)
order=[]
for r in led:
    k=(r['run'],r['label'])
    if k not in by: order.append(k)
    by[k][r['kind']]+=1; dur[k]+=r['dur']
print("\nper agent: green / red / aborted / quick — gate wall — share of the agent's wall")
runsum=collections.defaultdict(lambda:[collections.Counter(),0.0])
for k in order:
    b=by[k]; ag=[a for a in A.values() if a.get('label')==k[1] and NAMES[a['run']]==k[0]]
    w=ag[0]['wall'] if ag else 0
    print("  %-9s %-40s %2d / %2d / %2d / %2d   %s   %s"%(k[0],(k[1] or '')[:40],b['green'],b['red'],b['aborted'],b['quick']+b['quick-red'],hm(dur[k]),("%.0f%%"%(100*dur[k]/w)) if w else '-'))
    runsum[k[0]][0].update(b); runsum[k[0]][1]+=dur[k]
print()
for r,(b,d) in runsum.items(): print("  RUN %-12s green %2d red %2d aborted %2d quick %2d  wall %s"%(r,b['green'],b['red'],b['aborted'],b['quick']+b['quick-red'],hm(d)))
print("\nRED / ABORTED runs:")
for r in led:
    if r['kind'] in('red','aborted','quick-red'):
        fl=sorted(r['fails'].items())
        print("- %s %s %-8s %-9s %-28s %4.0fs red=%s fmtdiff=%d  %s"%(r['log'][-6:],t(r['start']),r['kind'],r['run'],(r['label'] or '')[:28],r['dur'],','.join(r['red_steps']),r['fmt_diff'],(r.get('desc') or '')[:60]))
        if r['kind']=='aborted': print("      (killed mid-test after %ss of nextest; %d in-flight tests reported)"%(r.get('nextest_s'),len(fl)))
        else:
            for n,_ in fl[:8]: print("      FAIL "+n[:150])
            if len(fl)>8: print("      … +%d more"%(len(fl)-8))
            for e in r['errors'][:2]:
                if 'test run failed' not in e: print("      "+e[:150])
if '-steps' in sys.argv:
    print("\nstep anatomy of FULL runs (s): log, tests, total, hygiene, pre-test(fmt+clippy+build), test-compile, nextest, doctest+tail")
    for r in led:
        if r['kind'] in('green','red'):
            comp=r['compile']; tc=comp.get('test',0) or 0
            pre=(r['end']-r['start'])-r.get('hygiene_s',0)-tc-(r.get('nextest_s') or 0)
            print("  %s %s %-7s %-22s tests=%d total=%4.0f hyg=%4.1f other=%5.1f test-compile=%5.1f nextest=%6.1f clippyC=%s buildC=%s"%(r['log'][-6:],t(r['start']),r['run'],(r['label'] or '')[:22],r.get('tests') or 0,r['dur'],r.get('hygiene_s',0),pre,tc,r.get('nextest_s') or 0,comp.get('clippy'),comp.get('build')))
