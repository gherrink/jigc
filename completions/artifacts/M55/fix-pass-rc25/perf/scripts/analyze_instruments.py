#!/usr/bin/env python3
"""The review and trial-scoring workflows per agent: queued → started → ended, queue wait, wall, model/tool split,
tokens, and the concurrency actually reached (max agents alive at once).
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: per phase and per agent label, times and token counts.
  usage: analyze_instruments.py <run id>... [-v]"""
import json,os,collections,datetime,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR',),args=1,usage='<run id>... [-v]')
D=E.out_dir()
AG=json.load(open(D+"/agents.json"))
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
def hm(s): return "%dm%02ds"%(s//60,s%60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%H:%M:%S')
cls=collections.defaultdict(collections.Counter)
for c in C:
    d=c.get('bg_dur') if c.get('bg') and c.get('bg_dur') is not None else (c.get('dur') or 0)
    cls[c['agent']][c['cls']]+=d
for run in sys.argv[1:]:
    ags=sorted([a for a in AG if a['run']==run],key=lambda a:a['start'])
    print("\n== %s"%run)
    ev=sorted([(a['start'],1) for a in ags]+[(a['end'],-1) for a in ags]); cur=mx=0
    for _,d in ev: cur+=d; mx=max(mx,cur)
    print("   max agents alive at once: %d"%mx)
    ph=collections.defaultdict(list)
    for a in ags: ph[a.get('phase')].append(a)
    for p,l in ph.items():
        ev=sorted([(a['start'],1) for a in l]+[(a['end'],-1) for a in l]); cur=m2=0
        for _,d in ev: cur+=d; m2=max(m2,cur)
        print("   phase %-10s n=%2d span %s (first start %s, last end %s) max concurrent %d; sum wall %s; if all had started together: %s"%(p,len(l),hm(max(a['end'] for a in l)-min(a['start'] for a in l)),t(min(a['start'] for a in l)),t(max(a['end'] for a in l)),m2,hm(sum(a['wall'] for a in l)),hm(max(a['wall'] for a in l))))
    if '-v' in sys.argv: continue
    for a in ags:
        top=", ".join("%s=%s"%(k,hm(v)) for k,v in cls[a['agent']].most_common(3) if v>=20)
        print("   %-22s %-9s start %s (queued→start %4.0fs) wall %7s model %7s tool %7s ctx %4dk out %4dk ret %5d | %s"%((a.get('label') or '')[:22],(a.get('phase') or '')[:9],t(a['start']),(a.get('wf_started',0)-a.get('wf_queued',0)) if a.get('wf_queued') else 0,hm(a['wall']),hm(a['model']),hm(a['tool_union']),a['peak_ctx']/1000,a['tok_out']/1000,a.get('journal_result_len') or 0,top))
