#!/usr/bin/env python3
"""Shape exploration 4: workflow run files — progress entry shapes, headline numbers.
Reads: SESSION_DIR/workflows/*.json. Prints: per run its id, workflow name, status and totals; for the run named,
the shape of one progress entry of each kind — and, beyond aggregates, every string of 60 characters or fewer
verbatim and the first 200 characters of its first six log lines. For the operator's eyes only.
  usage: explore4.py <run id>"""
import json,glob,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('SESSION_DIR',),args=1,usage='<run id>')
B=E.session_dir()
def shape(o,depth=0,maxd=3):
    if isinstance(o,dict): return {k:shape(v,depth+1,maxd) for k,v in list(o.items())[:25]} if depth<maxd else '{..}'
    if isinstance(o,list): return [shape(x,depth+1,maxd) for x in o[:1]]+(['x%d'%len(o)] if len(o)>1 else []) if o else []
    if isinstance(o,str): return 'str%d'%len(o) if len(o)>60 else 's:'+o
    return repr(o)
for f in sorted(glob.glob(B+"/workflows/wf_*.json")):
    d=json.load(open(f))
    st=datetime.datetime.utcfromtimestamp(d['startTime']/1000).isoformat()
    print(d['runId'],d.get('workflowName'),d.get('status'),'start',st,'dur_min',round(d['durationMs']/60000,1),'agents',d.get('agentCount'),'tok',d.get('totalTokens'),'calls',d.get('totalToolCalls'))
    kinds=collections.Counter(p.get('type') for p in d.get('workflowProgress',[]))
    print('   progress kinds',dict(kinds))
d=json.load(open(B+"/workflows/"+sys.argv[1]+".json"))
seen=set()
for p in d['workflowProgress']:
    if p['type'] not in seen:
        seen.add(p['type']); print(json.dumps(shape(p,0,4))[:900])
print([l[:200] for l in d.get('logs',[])][:6])
print(len(d['script']))
