#!/usr/bin/env python3
"""Wall time per command class, per run (and per agent with -a); background commands listed apart;
the N longest single commands (-top N). Durations: foreground = tool_use → tool_result;
background = tool_use → task-notification.
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: seconds and counts per class; with -top, each row carries the
command's own description and the sanitised 170-character head of the command, as stored in calls.jsonl."""
import json,os,collections,datetime,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),usage='[-a|-aa] [-top N]')
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
EXCL={'a'+x for x in []}
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M:%S') if x else '-'
NAMES,ORDER=E.runs()
CUTOFF=E.cutoff()  # the measured run's end
C=[c for c in C if c['t0']<CUTOFF]
classes=collections.Counter(); per=collections.defaultdict(lambda:collections.Counter()); cnt=collections.defaultdict(lambda:collections.Counter())
bgs=[]
for c in C:
    d=c.get('dur') or 0
    k=c.get('cls')
    if c.get('bg'):
        bgs.append(c); k=k+'(bg-launch)'
    per[c['run']][k]+=d; cnt[c['run']][k]+=1
allk=collections.Counter()
for r in per:
    if r!='main':
        for k,v in per[r].items(): allk[k]+=v
keys=[k for k,_ in allk.most_common()]
print("FOREGROUND seconds per class per run (count in parentheses); main = the orchestrating session's own calls")
print("%-26s"%'class'+"".join("%14s"%NAMES[r] for r in ORDER)+"%14s"%'ALL-subagents')
for k in keys:
    print("%-26s"%k+"".join("%14s"%("%s(%d)"%(hm(per[r][k])[:-3] if per[r][k]>=60 else "%ds"%per[r][k],cnt[r][k]) if cnt[r][k] else '.') for r in ORDER)+"%14s"%hm(allk[k]))
print("%-26s"%'TOTAL fg tool'+"".join("%14s"%hm(sum(per[r].values())) for r in ORDER))
print("\nBACKGROUND commands: %d launched; real duration = launch → notification"%len(bgs))
bk=collections.defaultdict(lambda:[0,0,0.0])
for c in bgs:
    e=bk[(NAMES[c['run']],c['cls'])]; e[0]+=1
    if c.get('bg_dur') is not None: e[1]+=1; e[2]+=c['bg_dur']
for (r,k),e in sorted(bk.items()): print("  %-13s %-22s launched %3d  notified %3d  real %s"%(r,k,e[0],e[1],hm(e[2])))
if '-a' in sys.argv:
    pa=collections.defaultdict(lambda:collections.Counter())
    for c in C:
        k=c.get('cls')
        if c.get('bg') and c.get('bg_dur'): pa[c['agent']][k+'*bg']+=c['bg_dur']
        pa[c['agent']][k]+=c.get('dur') or 0
    for aid,cn in sorted(pa.items(),key=lambda kv:A[kv[0]]['start']):
        a=A[aid]
        if NAMES[a['run']] in('review','trial-score','singles','orchestrator') and '-aa' not in sys.argv: continue
        print("\n%s %s wall %s model %s"%(NAMES[a['run']],a.get('label'),hm(a['wall']),hm(a['model'])))
        print("   "+"  ".join("%s=%s"%(k,hm(v)[1:] if v>=60 else '%ds'%v) for k,v in cn.most_common(14)))
if '-top' in sys.argv:
    n=int(sys.argv[sys.argv.index('-top')+1])
    rows=[]
    for c in C:
        if c['tool']!='Bash': continue
        d=c.get('bg_dur') if c.get('bg') and c.get('bg_dur') else (c.get('dur') or 0)
        rows.append((d,c))
    rows.sort(key=lambda x:-x[0])
    print("\nTOP %d longest single commands"%n)
    for d,c in rows[:n]:
        a=A[c['agent']]
        print("%s  %-9s %-12s %-22s %s%s | %s | %s"%(hm(d),c['cls'][:9],NAMES[c['run']],(a.get('label') or '')[:22],t(c['t0']),' BG' if c.get('bg') else '',c.get('desc','')[:70],' '.join(c['cmd'].split())[:170]))
