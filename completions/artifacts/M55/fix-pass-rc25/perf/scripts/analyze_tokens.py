#!/usr/bin/env python3
"""Tokens per API request, read from each assistant message's `usage` (one record per message id):
uncached input, cache writes, cache reads, output — per run and per agent — and how much of the cache-write
volume followed a pause longer than the 5-minute cache lifetime (a full-context rewrite).
Cost weights (relative to one uncached input token): cache write 1.25, cache read 0.1, output 5.
Reads: every transcript under SESSION_DIR (the `usage` objects and timestamps only), OUT_DIR/agents.json.
Writes OUT_DIR/tokens_by_agent.json. Prints: token sums per run, and per agent label with -a."""
import json,glob,os,collections,datetime,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('SESSION_DIR','OUT_DIR','RUNS_FILE','CUTOFF_UTC',),usage='[-a]')
B=E.session_dir()
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
NAMES,ORDER=E.runs()
CUT=E.cutoff()
def ts(s): return datetime.datetime.fromisoformat(s.replace('Z','+00:00')).timestamp()
files=glob.glob(B+"/subagents/workflows/*/agent-*.jsonl")+glob.glob(B+"/subagents/agent-*.jsonl")+[B+".jsonl"]
per=collections.defaultdict(collections.Counter); pa=collections.defaultdict(collections.Counter)
for f in files:
    aid='main' if f.endswith(B.split('/')[-1]+'.jsonl') else os.path.basename(f)[6:-6]
    if aid not in A: continue
    run=NAMES[A[aid]['run']]
    msgs={}; order=[]
    for l in open(f):
        try: d=json.loads(l)
        except Exception: continue
        if d.get('type')!='assistant' or 'timestamp' not in d: continue
        m=d['message']; mid=m.get('id'); u=m.get('usage') or {}
        tt=ts(d['timestamp'])
        if mid not in msgs: msgs[mid]=dict(t0=tt,t1=tt); order.append(mid)
        e=msgs[mid]; e['t1']=tt
        e['inp']=u.get('input_tokens',0) or 0; e['cc']=u.get('cache_creation_input_tokens',0) or 0; e['cr']=u.get('cache_read_input_tokens',0) or 0
        e['out']=max(e.get('out',0),u.get('output_tokens',0) or 0)
        cc=u.get('cache_creation') or {}
        e['cc1h']=cc.get('ephemeral_1h_input_tokens',0) or 0
    prev=None
    for mid in order:
        e=msgs[mid]
        if e['t0']>CUT: continue
        gap=(e['t0']-prev) if prev else 0
        prev=e['t1']
        for tgt in(per[run],pa[aid]):
            tgt['req']+=1; tgt['inp']+=e['inp']; tgt['cc']+=e['cc']; tgt['cr']+=e['cr']; tgt['out']+=e['out']; tgt['cc1h']+=e['cc1h']
            if gap>=300:
                tgt['req_after_pause']+=1; tgt['cc_after_pause']+=e['cc']; tgt['cr_after_pause']+=e['cr']
                if e['cr']<0.5*(e['cc']+e['cr']): tgt['full_rewrites']+=1; tgt['cc_full_rewrite']+=e['cc']
def w(c): return c['inp']+1.25*c['cc']+0.1*c['cr']+5*c['out']
print("%-14s %7s %9s %9s %10s %8s | %9s | %8s %10s %8s"%('run','requests','uncached','cacheWr','cacheRd','output','cost-units','pauses>5m','rewritten','share'))
T=collections.Counter()
for r in [NAMES[k] for k in ORDER]:
    c=per[r]; T.update(c)
    print("%-14s %7d %8.2fM %8.2fM %9.1fM %7.2fM | %8.1fM | %8d %9.2fM %7.0f%%"%(r,c['req'],c['inp']/1e6,c['cc']/1e6,c['cr']/1e6,c['out']/1e6,w(c)/1e6,c['full_rewrites'],c['cc_full_rewrite']/1e6,100*1.25*c['cc_full_rewrite']/w(c) if w(c) else 0))
c=T
print("%-14s %7d %8.2fM %8.2fM %9.1fM %7.2fM | %8.1fM | %8d %9.2fM %7.0f%%"%('TOTAL',c['req'],c['inp']/1e6,c['cc']/1e6,c['cr']/1e6,c['out']/1e6,w(c)/1e6,c['full_rewrites'],c['cc_full_rewrite']/1e6,100*1.25*c['cc_full_rewrite']/w(c)))
print("cost-unit composition: cache writes %.0f%%, cache reads %.0f%%, output %.0f%%, uncached %.0f%%; 1-hour cache writes: %d"%(100*1.25*c['cc']/w(c),100*0.1*c['cr']/w(c),100*5*c['out']/w(c),100*c['inp']/w(c),c['cc1h']))
if '-a' in sys.argv:
    print("\nper agent (fix rounds + audit): requests, peak context, output, cost-units, full rewrites, rewritten tokens, cost-units per request")
    for aid,c in sorted(pa.items(),key=lambda kv:A[kv[0]]['start']):
        a=A[aid]
        if NAMES[a['run']] not in('round1','round2','round3','round4','audit'): continue
        print("  %-7s %-36s req %4d peak %4dk out %4dk cost %6.1fM rewrites %3d (%5.2fM) %5.0fk/req"%(NAMES[a['run']],(a.get('label') or '')[:36],c['req'],a['peak_ctx']/1000,c['out']/1000,w(c)/1e6,c['full_rewrites'],c['cc_full_rewrite']/1e6,w(c)/c['req']/1000))
json.dump({k:dict(v) for k,v in pa.items()},open(D+"/tokens_by_agent.json","w"))
