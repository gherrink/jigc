#!/usr/bin/env python3
"""Per-run and per-agent overview: start, end, wall, tokens, call counts, tool/model/idle split.
Reads: OUT_DIR/agents.json. Prints: one block per run (its run id, times, token sums) and per agent label."""
import json,os,collections,datetime,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE',),usage='[-v]')
D=E.out_dir()
A=json.load(open(D+"/agents.json"))
def hm(s): return "%d:%02d"%(s//3600,(s%3600)//60) if s>=3600 else "%dm%02ds"%(s//60,s%60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M') if x else '-'
_,ORDER=E.runs()
byrun=collections.defaultdict(list)
for a in A: byrun[a['run']].append(a)
print("token definitions vs the workflow file's per-agent `tokens` (sum over workflow agents):")
s=collections.Counter()
for a in A:
    if a.get('wf_tokens'):
        s['wf']+=a['wf_tokens']; s['in+out']+=a['tok_in']+a['tok_out']; s['in+cc+out']+=a['tok_in']+a['tok_cache_create']+a['tok_out']
        s['all']+=a['tok_in']+a['tok_cache_create']+a['tok_cache_read']+a['tok_out']; s['peak']+=a['peak_ctx']; s['out']+=a['tok_out']; s['cr']+=a['tok_cache_read']
print(dict(s))
for run in ORDER:
    ags=sorted(byrun[run],key=lambda a:a['start'] or 0)
    if not ags: continue
    st=min(a['start'] for a in ags); en=max(a['end'] for a in ags)
    print("\n== %s  %s → %s  wall %s  agents %d  sum-agent-wall %s  model %s  tool %s  idle %s  out-tok %.2fM  wf-tok %.2fM  fresh-in(cc) %.2fM cache-read %.1fM"%(
        run,t(st),t(en),hm(en-st),len(ags),hm(sum(a['wall'] for a in ags)),hm(sum(a['model'] for a in ags)),hm(sum(a['tool_union'] for a in ags)),hm(sum(a['idle'] for a in ags)),
        sum(a['tok_out'] for a in ags)/1e6,sum(a.get('wf_tokens') or 0 for a in ags)/1e6,sum(a['tok_cache_create']+a['tok_in'] for a in ags)/1e6,sum(a['tok_cache_read'] for a in ags)/1e6))
    if '-v' in sys.argv or len(ags)<=12 or run in('single',):
        for a in ags:
            print("   %-34s %-22s %s→%s %8s  model %7s tool %7s idle %6s calls %4d bash %4d  wfTok %7s out %6dk peak %4dk ret %5d"%(
                (a.get('label') or '?')[:34],(a.get('agent_type') or '')[:22],t(a['start']),t(a['end'])[6:],hm(a['wall']),hm(a['model']),hm(a['tool_union']),hm(a['idle']),a['n_calls'],a['n_bash'],
                ('%dk'%(a['wf_tokens']/1000)) if a.get('wf_tokens') else '-',a['tok_out']/1000,a['peak_ctx']/1000,a.get('journal_result_len') or a.get('structured_len') or a.get('final_text_len') or 0))
    else:
        ph=collections.defaultdict(list)
        for a in ags: ph[a.get('phase')].append(a)
        for p,l in ph.items():
            print("   phase %-12s n=%2d  %s→%s span %s  agent-wall sum %s median %s max %s | model %s tool %s | wfTok %.2fM"%(p,len(l),t(min(a['start'] for a in l)),t(max(a['end'] for a in l))[6:],hm(max(a['end'] for a in l)-min(a['start'] for a in l)),
                hm(sum(a['wall'] for a in l)),hm(sorted(a['wall'] for a in l)[len(l)//2]),hm(max(a['wall'] for a in l)),hm(sum(a['model'] for a in l)),hm(sum(a['tool_union'] for a in l)),sum(a.get('wf_tokens') or 0 for a in l)/1e6))
