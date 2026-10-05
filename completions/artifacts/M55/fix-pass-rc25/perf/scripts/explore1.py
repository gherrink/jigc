#!/usr/bin/env python3
"""Shape exploration 1: the first workflow run file, and the tool, attachment, stop-reason and user-entry kinds of
every subagent transcript.
Reads: SESSION_DIR/workflows and every subagent transcript. Prints: key names, counts and type tags — and, beyond
aggregates, every string of 30 characters or fewer verbatim, the names of the workflow script files, and the first
25 characters of each kind of user message. For the operator's eyes: none of its output belongs in a committed file."""
import json,glob,os,collections,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('SESSION_DIR',))
B=E.session_dir()
def shape(o,depth=0,maxd=3):
    if isinstance(o,dict): return {k:shape(v,depth+1,maxd) for k,v in list(o.items())[:25]} if depth<maxd else '{..}'
    if isinstance(o,list): return [shape(x,depth+1,maxd) for x in o[:1]]+(['x%d'%len(o)] if len(o)>1 else []) if o else []
    if isinstance(o,str): return 'str%d'%len(o) if len(o)>30 else 's:'+o
    return repr(o)
wf=sorted(glob.glob(B+"/workflows/wf_*.json"))
d=json.load(open(wf[0]))
print(os.path.basename(wf[0]), json.dumps(shape(d,0,4))[:3000])
print(os.listdir(B+"/workflows/scripts")[:40])
tools=collections.Counter(); att=collections.Counter(); ukind=collections.Counter(); bg=0; stop=collections.Counter()
for f in glob.glob(B+"/subagents/**/agent-*.jsonl",recursive=True):
    for l in open(f):
        d=json.loads(l)
        t=d.get('type')
        if t=='attachment': att[d['attachment'].get('type')]+=1
        elif t=='assistant':
            stop[d['message'].get('stop_reason')]+=1
            for b in d['message']['content']:
                if b['type']=='tool_use':
                    tools[b['name']]+=1
                    if b['input'].get('run_in_background'): bg+=1
        elif t=='user':
            c=d['message']['content']
            if isinstance(c,str): ukind['str:'+c[:25].replace('\n',' ')]+=1
            else:
                for b in c:
                    k=b['type']
                    if k=='text': k='text:'+b['text'][:25].replace('\n',' ')
                    ukind[k]+=1
        else: ukind['TYPE:'+str(t)]+=1
print(tools.most_common()); print(att.most_common()); print(ukind.most_common(40)); print('bg',bg); print(stop)
