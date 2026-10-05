#!/usr/bin/env python3
"""Shape exploration 2: tool_result envelope keys, queued_command shape, background results.
Reads: every subagent transcript under SESSION_DIR. Prints: key sets with counts and the shapes of up to two
entries of each kind — and, beyond aggregates, every string of 40 characters or fewer verbatim and the first 40
characters of each kind of queued prompt. For the operator's eyes: none of its output belongs in a committed file."""
import json,glob,os,collections
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('SESSION_DIR',))
B=E.session_dir()
def shape(o,depth=0,maxd=3):
    if isinstance(o,dict): return {k:shape(v,depth+1,maxd) for k,v in list(o.items())[:25]} if depth<maxd else '{..}'
    if isinstance(o,list): return [shape(x,depth+1,maxd) for x in o[:1]]+(['x%d'%len(o)] if len(o)>1 else []) if o else []
    if isinstance(o,str): return 'str%d'%len(o) if len(o)>40 else 's:'+o
    return repr(o)
turkeys=collections.Counter(); qc=[]; bgres=[]; hook=[]; qprefix=collections.Counter()
for f in glob.glob(B+"/subagents/**/agent-*.jsonl",recursive=True):
    bgids={}
    for l in open(f):
        d=json.loads(l)
        t=d.get('type')
        if t=='assistant':
            for b in d['message']['content']:
                if b['type']=='tool_use' and b['input'].get('run_in_background'): bgids[b['id']]=1
        if t=='user' and 'toolUseResult' in d:
            r=d['toolUseResult']
            if isinstance(r,dict):
                turkeys[tuple(sorted(r.keys()))]+=1
                c=d['message']['content']
                if isinstance(c,list) and c and c[0].get('tool_use_id') in bgids and len(bgres)<2: bgres.append(shape(d,0,3))
            else: turkeys['<'+type(r).__name__+'>']+=1
        if t=='attachment':
            a=d['attachment']
            if a['type']=='queued_command':
                if len(qc)<2: qc.append(shape(d,0,3))
                p=a.get('prompt')
                if isinstance(p,str): qprefix[p[:40].replace('\n',' ')]+=1
                else: qprefix[str(type(p))]+=1
            if a['type']=='hook_success' and len(hook)<1: hook.append(shape(d,0,3))
for k,v in turkeys.most_common(25): print(v,k)
print(json.dumps(qc)[:2500]); print(qprefix.most_common(10)); print(json.dumps(bgres)[:2500]); print(json.dumps(hook)[:1200])
