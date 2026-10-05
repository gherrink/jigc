#!/usr/bin/env python3
"""Classify each red full gate by cause: fmt/clippy only, static fence/registry tests only, behaviour tests,
or mixed; check that an edit happened between the red and the next gate (else it would be a flake).
Reads: OUT_DIR/gate_ledger.json, calls.jsonl. Prints: one line per red gate (agent label, cause class, counts)
and the sums per cause class."""
import json,os,collections,datetime,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','CUTOFF_UTC',))
D=E.out_dir()
led=json.load(open(D+"/gate_ledger.json"))
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
CUT=E.cutoff()
led=[r for r in led if r['start']<CUT]
# suites whose failing test is a source/registry/help-text scan (verified by reading the suite heads and by their sub-1.5 s runtime)
FENCE={'git_span_aim','task_area_writer_registry','migrate_route_family','message_whitespace_fence','rollback_population_registry','repo_relative_paths','dry_run_findings_equal_set','count_fences','foldback_truth','help_truth','flow52_acceptance'}
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
cat=collections.Counter(); wall=collections.Counter(); rows=[]
for i,r in enumerate(led):
    if r['kind']!='red': continue
    suites=[n.split(' ')[1].split('::')[0] for n in r['fails']]
    f=[s for s in suites if s in FENCE]; b=[s for s in suites if s not in FENCE]
    lint=[s for s in r['red_steps'] if s in('fmt','clippy')]
    if lint and not suites: k='fmt/clippy only'
    elif suites and not b and not lint: k='static fence/registry only'
    elif suites and not b and lint: k='fence + fmt/clippy'
    elif b and not f and not lint: k='behaviour test only'
    else: k='mixed (behaviour + fence or lint)'
    nxt=[x for x in led[i+1:] if x.get('agent')==r.get('agent') and x['kind'] in('green','red','aborted')]
    edits=0
    if nxt:
        for c in C:
            if c['agent']==r['agent'] and r['end']-5<=c['t0']<=nxt[0]['start']+5:
                if c['tool'] in('Edit','Write') or (c['tool']=='Bash' and re.search(r"cargo fmt(?! --check)|open\(p|\.write\(|cp \S+ crates|cat >|sed -i|git apply|perl -pi",c['cmd'])): edits+=1
    cat[k]+=1; wall[k]+=r['dur']; rows.append((r['log'][-6:],r['run'],r['label'],k,len(f),len(b),lint,edits if nxt else None))
for l,run,lab,k,nf,nb,lint,ed in rows: print("  %s %-7s %-28s %-34s fence=%d behaviour=%d lint=%s edits-before-rerun=%s"%(l,run,(lab or '')[:28],k,nf,nb,','.join(lint) or '-',ed))
print()
for k,n in cat.most_common(): print("  %-36s %2d red gates  %s"%(k,n,hm(wall[k])))
cheap=sum(n for k,n in cat.items() if k!='behaviour test only' and not k.startswith('mixed'))
print("  red gates a seconds-long pre-check (fmt + clippy + the fence suites) would have caught entirely: %d of %d, %s"%(cheap,sum(cat.values()),hm(sum(v for k,v in wall.items() if k!='behaviour test only' and not k.startswith('mixed')))))
