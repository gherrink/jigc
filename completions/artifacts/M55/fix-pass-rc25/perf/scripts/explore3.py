#!/usr/bin/env python3
"""Shape exploration 3: background-result texts, task-notification texts (home path masked), timeout markers.
Reads: the agent transcripts of ONE workflow run under SESSION_DIR. Prints: the marker counts — and, beyond
aggregates, the first 400 characters of two background results and the first 700 of two task notifications, raw
but for the home path. This one echoes transcript text by design: run it only where its output stays private.
  usage: explore3.py <run id>"""
import json,glob,os,collections,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('SESSION_DIR',),args=1,usage='<run id>')
B=E.session_dir()
H=os.path.expanduser("~")
def txt(c):
    if isinstance(c,str): return c
    return "\n".join(b.get('text','') for b in c if isinstance(b,dict) and b.get('type')=='text')
shown=0; shownq=0; markers=collections.Counter()
for f in glob.glob(B+"/subagents/workflows/"+sys.argv[1]+"/agent-*.jsonl"):
    bg={}
    for l in open(f):
        d=json.loads(l); t=d.get('type')
        if t=='assistant':
            for b in d['message']['content']:
                if b['type']=='tool_use' and b['name']=='Bash':
                    bg[b['id']]=b['input'].get('run_in_background',False)
        elif t=='user' and isinstance(d['message']['content'],list):
            for b in d['message']['content']:
                if b['type']=='tool_result':
                    s=txt(b.get('content') or '')
                    if bg.get(b['tool_use_id']) and shown<2:
                        shown+=1; print('BGRESULT:',s[:400].replace(H,'~'))
                    for m in ('timed out','Command running in background','was moved to the background','Exit code','<persisted-output>','Output too large','blocked','claude-shell-guard','[lacon'):
                        if m in s: markers[m]+=1
        elif t=='attachment' and d['attachment']['type']=='queued_command' and shownq<2:
            shownq+=1; print('NOTIF:',str(d['attachment'].get('prompt'))[:700].replace(H,'~'))
print(markers)
