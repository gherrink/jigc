#!/usr/bin/env python3
"""Emit the report's markdown tables from the datasets under OUT_DIR (so the report's numbers are the scripts' numbers).
Reads: OUT_DIR/agents.json, calls.jsonl, gate_ledger.json, partition_by_agent.json, tokens_by_agent.json, commits.txt.
Prints: one markdown table — agent labels, times, counts; `runs` also prints each workflow run id.
  usage: make_tables.py <table-name>   names: runs agents instruments singles"""
import json,os,collections,datetime,sys,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),args=1,usage='runs|agents|instruments|singles')
D=E.out_dir()
AG=json.load(open(D+"/agents.json")); A={a['agent']:a for a in AG}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
led=json.load(open(D+"/gate_ledger.json"))
part=json.load(open(D+"/partition_by_agent.json"))
tok=json.load(open(D+"/tokens_by_agent.json"))
NAMES,ORDER=E.runs()
CUT=E.cutoff()
def hm(s): return "%d:%02d"%(s//3600,(s%3600)//60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%d %H:%M')
def w(c): return c.get('inp',0)+1.25*c.get('cc',0)+0.1*c.get('cr',0)+5*c.get('out',0)
AG=[a for a in AG if a['start']<CUT]
commits=[]
for l in open(D+"/commits.txt"):
    h,iso,subj=l.rstrip('\n').split(' ',2); commits.append(datetime.datetime.fromisoformat(iso).timestamp())
def ncommits(a): return sum(1 for x in commits if a['start']<=x<=a['end']+5)
gk=collections.defaultdict(collections.Counter); gw=collections.Counter()
for g in led:
    if g.get('agent') and g['start']<CUT: gk[g['agent']][g['kind']]+=1; gw[g['agent']]+=g['dur'] if g['kind'] in('green','red','aborted') else 0
name=sys.argv[1]
if name=='runs':
    print("| run | what | start → end (UTC) | wall | agents | agent-hours | model | gate | other tools | context-sum \"tokens\" | cost units |\n|---|---|---|---|---|---|---|---|---|---|---|")
    WHAT={'review':'per-axis review','trial-score':'trial scoring','tier1-redrive':'tier-1 re-drive','fix-plan':'fix planning','round1':'fix round 1','round2':'fix round 2','round3':'fix round 3','audit':'completion audit','round4':'fix round 4','singles':'single subagents','orchestrator':'the orchestrating session'}
    for r in ORDER:
        ags=[a for a in AG if a['run']==r]
        if not ags: continue
        s=min(a['start'] for a in ags); e=min(CUT,max(a['end'] for a in ags))
        p=collections.Counter()
        for a in ags: p.update(part.get(a['agent'],{}))
        tc=collections.Counter()
        for a in ags: tc.update(tok.get(a['agent'],{}))
        aw=sum(p.values()); other=aw-p['model']-p['gate']-p.get('idle',0)
        print("| `%s` | %s | %s → %s | %s | %d | %s | %s | %s | %s | %s | %.1f M |"%(r if r.startswith('wf_') else '—',WHAT[NAMES[r]],t(s),t(e),hm(e-s) if r!='single' else '—',len(ags),hm(aw-p.get('idle',0)),hm(p['model']),hm(p['gate']) if p['gate'] else '—',hm(other),("%.2f M"%(sum(a.get('wf_tokens') or 0 for a in ags)/1e6)) if r.startswith('wf_') else '—',w(tc)/1e6))
elif name=='agents':
    print("| run | agent | start (UTC) | wall | model | gate | scoped tests | other tools | calls | commits | gates g/r/a | peak context | output | cost units |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for a in sorted(AG,key=lambda a:a['start']):
        if NAMES[a['run']] not in('round1','round2','round3','round4','audit','fix-plan','tier1-redrive'): continue
        p=collections.Counter(part.get(a['agent'],{})); aw=sum(p.values()); k=gk[a['agent']]
        other=aw-p['model']-p['gate']-p['scoped tests']
        fix=NAMES[a['run']].startswith('round')
        print("| %s | %s | %s | %s | %s | %s | %s | %s | %d | %s | %s | %dk | %dk | %.1f M |"%(NAMES[a['run']],(a.get('label') or '').replace('|','/')[:44],t(a['start']),hm(aw),hm(p['model']),hm(p['gate']) if p['gate'] else '—',hm(p['scoped tests']) if p['scoped tests']>=30 else '—',hm(other),a['n_calls'],ncommits(a) if fix else '—',("%d/%d/%d"%(k['green'],k['red'],k['aborted'])) if fix else '—',a['peak_ctx']/1000,a['tok_out']/1000,w(tok.get(a['agent'],{}))/1e6))
elif name=='instruments':
    print("| run | phase | agents | span | sum of agent walls | longest agent | max alive at once | model | tools | context-sum \"tokens\" | cost units |\n|---|---|---|---|---|---|---|---|---|---|---|")
    for r in [k for k in ORDER if NAMES[k] in('review','trial-score','tier1-redrive','fix-plan','audit')]:
        ph=collections.defaultdict(list)
        for a in AG:
            if a['run']==r: ph[a.get('phase')].append(a)
        for p,l in ph.items():
            ev=sorted([(a['start'],1) for a in l]+[(a['end'],-1) for a in l]); cur=mx=0
            for _,d in ev: cur+=d; mx=max(mx,cur)
            tc=collections.Counter()
            for a in l: tc.update(tok.get(a['agent'],{}))
            print("| %s | %s | %d | %s | %s | %s | %d | %s | %s | %.2f M | %.1f M |"%(NAMES[r],p,len(l),hm(max(a['end'] for a in l)-min(a['start'] for a in l)),hm(sum(a['wall'] for a in l)),hm(max(a['wall'] for a in l)),mx,hm(sum(a['model'] for a in l)),hm(sum(a['tool_union'] for a in l)),sum(a.get('wf_tokens') or 0 for a in l)/1e6,w(tc)/1e6))
elif name=='singles':
    print("| single subagent | type | start (UTC) | wall | model | tools | idle | calls | peak context | cost units |\n|---|---|---|---|---|---|---|---|---|---|")
    for a in sorted(AG,key=lambda a:a['start']):
        if a['run']!='single': continue
        print("| %s | %s | %s | %s | %s | %s | %s | %d | %dk | %.1f M |"%((a.get('label') or '')[:44],a.get('agent_type'),t(a['start']),hm(a['wall']),hm(a['model']),hm(a['tool_union']),hm(a['idle']),a['n_calls'],a['peak_ctx']/1000,w(tok.get(a['agent'],{}))/1e6))
