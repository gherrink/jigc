#!/usr/bin/env python3
"""More markdown tables for the report: partition, reds, gate-growth, suite-binaries, suite-top, tests-top, probe, tokens.
Reads: OUT_DIR/agents.json and, by table, partition_by_agent.json, gate_ledger.json, per_test_suite.json,
new_tests.json, tokens_by_agent.json; `probe` also reads WORK_DIR/suite/probe-summary.txt.
Prints: one markdown table — agent labels, test and suite names, times, counts.
  usage: make_tables2.py <name>"""
import json,os,collections,datetime,sys,re,glob,statistics as st
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),args=1,usage='partition|reds|gate-growth|suite-binaries|suite-top|tests-top|probe|tokens')
D=E.out_dir()
AG=json.load(open(D+"/agents.json")); A={a['agent']:a for a in AG}
NAMES,ORDER=E.runs()
RUNS=[NAMES[k] for k in ORDER if k!='main']
CUT=E.cutoff()
def hm(s): return "%d:%02d"%(s//3600,(s%3600)//60)
def ms(s): return "%d:%02d"%(s//60,s%60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%d %H:%M')
name=sys.argv[1]
if name=='partition':
    part=json.load(open(D+"/partition_by_agent.json"))
    per=collections.defaultdict(collections.Counter); tot=collections.Counter()
    for a in AG:
        if a['start']>=CUT or a['run']=='main': continue
        p=part.get(a['agent'],{}); per[NAMES[a['run']]].update(p); tot.update(p)
    T=sum(tot.values())
    print("| bucket | "+" | ".join(RUNS)+" | all subagents | share |\n|---|"+"---|"*(len(RUNS)+2))
    for k,v in tot.most_common():
        if v<120: continue
        print("| %s | "%k+" | ".join(hm(per[r][k]) if per[r][k]>=30 else '·' for r in RUNS)+" | %s | %.1f%% |"%(hm(v),100*v/T))
    print("| **total agent wall** | "+" | ".join(hm(sum(per[r].values())) for r in RUNS)+" | **%s** | |"%hm(T))
elif name=='reds':
    led=[r for r in json.load(open(D+"/gate_ledger.json")) if r['start']<CUT]
    FENCE={'git_span_aim','task_area_writer_registry','migrate_route_family','message_whitespace_fence','rollback_population_registry','repo_relative_paths','dry_run_findings_equal_set','count_fences','foldback_truth','help_truth','flow52_acceptance'}
    print("| # | start (UTC) | run | fixer | gate s | red step(s) | failing tests (suite, own seconds) | cause class | until the rerun started |\n|---|---|---|---|---|---|---|---|---|")
    i=0
    for j,r in enumerate(led):
        if r['kind']!='red': continue
        i+=1
        suites=[(n.split(' ')[1].split('::')[0],d) for n,d in r['fails'].items()]
        f=[s for s,_ in suites if s in FENCE]; b=[s for s,_ in suites if s not in FENCE]; lint=[s for s in r['red_steps'] if s in('fmt','clippy')]
        k='lint only' if lint and not suites else 'fence/registry' if suites and not b and not lint else 'fence + lint' if suites and not b else 'behaviour' if b and not f and not lint else 'mixed'
        nxt=[x for x in led[j+1:] if x.get('agent')==r.get('agent') and x['kind'] in('green','red')]
        print("| %d | %s | %s | %s | %d | %s | %s | %s | %s |"%(i,t(r['start']),r['run'],(r['label'] or '').replace('fix:','')[:28],r['dur'],', '.join(r['red_steps']),'; '.join("%s %.2f"%(s,d) for s,d in suites) or '—',k,("%d s"%(nxt[0]['start']-r['end'])) if nxt else '—'))
elif name=='gate-growth':
    led=[r for r in json.load(open(D+"/gate_ledger.json")) if r['start']<CUT and r['kind'] in('green','red')]
    byr=collections.defaultdict(list)
    for r in led: byr[r['run']].append(r)
    print("| run | full gates | tests (first → last) | gate wall, median s | nextest, median s | nextest min–max s | hygiene advisory, median s | test compile, median s |\n|---|---|---|---|---|---|---|---|")
    for k in('singles','round1','round2','round3','round4'):
        v=byr[k]
        if not v: continue
        nx=[x['nextest_s'] for x in v if x.get('nextest_s')]
        print("| %s | %d | %d → %d | %d | %d | %d–%d | %.1f | %.1f |"%(k,len(v),v[0]['tests'],v[-1]['tests'],st.median([x['dur'] for x in v]),st.median(nx),min(nx),max(nx),st.median([x.get('hygiene_s',0) for x in v]),st.median([x['compile'].get('test',0) or 0 for x in v])))
elif name in('suite-binaries','suite-top','tests-top'):
    T=json.load(open(D+"/per_test_suite.json")); new=json.load(open(D+"/new_tests.json"))['tests']
    tot=sum(T.values())
    def isnew(k):
        b,n=k.split(' ',1); return new.get(n.split('::')[0]+'::'+n.split('::')[-1])
    if name=='suite-binaries':
        byb=collections.defaultdict(lambda:[0,0.0,0.0,0,0.0])
        for k,v in T.items():
            e=byb[k.split(' ')[0]]; e[0]+=1; e[1]+=v; e[2]=max(e[2],v)
            if isnew(k): e[3]+=1; e[4]+=v
        print("| test binary | tests | summed s | share | mean s | max s | ≈ wall alone at 16 threads | added by the fix pass (tests / s) |\n|---|---|---|---|---|---|---|---|")
        for b,(c,s,m,nn,ns) in sorted(byb.items(),key=lambda kv:-kv[1][1]): print("| `%s` | %d | %d | %.1f%% | %.2f | %.0f | %d s | %d / %d |"%(b,c,s,100*s/tot,s/c,m,max(m,s/16),nn,ns))
        print("| **all** | %d | %d | | %.2f | | | %d / %d |"%(len(T),tot,tot/len(T),sum(1 for k in T if isnew(k)),sum(v for k,v in T.items() if isnew(k))))
    elif name=='suite-top':
        bys=collections.defaultdict(lambda:[0,0.0,0.0,set(),''])
        for k,v in T.items():
            b,n=k.split(' ',1)
            if not b.startswith('jigc::g_'): continue
            e=bys[n.split('::')[0]]; e[0]+=1; e[1]+=v; e[2]=max(e[2],v); e[4]=b.replace('jigc::','')
            if isnew(k): e[3].add(isnew(k))
        print("| # | suite | group | tests | summed s | share of the step | mean s | max s | fix pass |\n|---|---|---|---|---|---|---|---|---|")
        for i,(s,(c,sm,m,kinds,b)) in enumerate(sorted(bys.items(),key=lambda kv:-kv[1][1])[:25],1):
            print("| %d | `%s` | %s | %d | %d | %.1f%% | %.1f | %.0f | %s |"%(i,s,b,c,sm,100*sm/tot,sm/c,m,'new' if 'new-suite' in kinds else 'grown' if kinds else ''))
    else:
        print("| # | s | group | test | fix pass |\n|---|---|---|---|---|")
        for i,(k,v) in enumerate(sorted(T.items(),key=lambda kv:-kv[1])[:20],1):
            b,n=k.split(' ',1); print("| %d | %.0f | %s | `%s` | %s |"%(i,v,b.replace('jigc::',''),n[:105],'yes' if isnew(k) else ''))
elif name=='probe':
    T=json.load(open(D+"/per_test_suite.json"))
    print("| test | in the full run, s | alone, s | git spawned by the test | jigc invocations | git spawned inside jigc | child-process share of the test's wall | in-process, s |\n|---|---|---|---|---|---|---|---|")
    txt=open(E.work_dir()+"/suite/probe-summary.txt").read().split('\n')
    for i,l in enumerate(txt):
        m=re.match(r'\s+alone ([\d.]+)s .* shimmed ([\d.]+)s .* git direct (\d+) calls ([\d.]+)s \| jigc (\d+) calls ([\d.]+)s \(of which nested git (\d+) calls ([\d.]+)s\) \| children ([\d.]+)s = (\d+)% .* in-process ([\d.]+)s',l)
        if m:
            nm=txt[i-1].strip(); full=[v for k,v in T.items() if k.endswith(' '+nm)]
            print("| `%s` | %.1f | %s | %s | %s | %s | %s%% | %s |"%(nm[:70],full[0] if full else -1,m.group(1),m.group(3),m.group(5),m.group(7),m.group(10),m.group(11)))
elif name=='tokens':
    tok=json.load(open(D+"/tokens_by_agent.json"))
    per=collections.defaultdict(collections.Counter)
    for a in AG:
        if a['start']>=CUT: continue
        per[NAMES[a['run']]].update(tok.get(a['agent'],{}))
    def w(c): return c['inp']+1.25*c['cc']+0.1*c['cr']+5*c['out']
    print("| run | API requests | cache writes | cache reads | output | cost units | share | full-context rewrites after a pause > 5 min | tokens rewritten | rewrites' share of the run's cost |\n|---|---|---|---|---|---|---|---|---|---|")
    T=collections.Counter()
    for r in RUNS+['orchestrator']: T.update(per[r])
    for r in RUNS+['orchestrator']:
        c=per[r]; print("| %s | %d | %.2f M | %.1f M | %.2f M | %.1f M | %.0f%% | %d | %.2f M | %.0f%% |"%(r,c['req'],c['cc']/1e6,c['cr']/1e6,c['out']/1e6,w(c)/1e6,100*w(c)/w(T),c['full_rewrites'],c['cc_full_rewrite']/1e6,100*1.25*c['cc_full_rewrite']/w(c)))
    c=T; print("| **all** | %d | %.2f M | %.1f M | %.2f M | %.1f M | | %d | %.2f M | %.0f%% |"%(c['req'],c['cc']/1e6,c['cr']/1e6,c['out']/1e6,w(c)/1e6,c['full_rewrites'],c['cc_full_rewrite']/1e6,100*1.25*c['cc_full_rewrite']/w(c)))
