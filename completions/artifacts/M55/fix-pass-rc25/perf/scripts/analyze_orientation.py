#!/usr/bin/env python3
"""What fixers did before their first source edit: calls and seconds per class, result characters read,
and the context size reached at that point.
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: counts, seconds and result sizes per class, and the tails of the
paths of the files most often read."""
import json,os,collections,re,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR',))
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
SRC=re.compile(r'(^|[\s\'"=/])(crates/|design/|dev/|implementation/|DECISIONS\.md|completions/)')
cnt=collections.Counter(); dur=collections.Counter(); chars=collections.Counter(); n=0; tot=0; mt=0
files=collections.Counter()
for aid,a in A.items():
    if a.get('agent_type')!='build-fixer' or not a['run'].startswith('wf_'): continue
    cs=sorted([c for c in C if c['agent']==aid],key=lambda c:c['t0'])
    first=None
    for i,c in enumerate(cs):
        if c['tool'] in('Edit','Write') and SRC.search(c['cmd']) and '/scratchpad' not in c['cmd'] and not c['cmd'].startswith('$S'): first=i; break
        if c['tool']=='Bash' and re.search(r"open\(p,\s*'w'\)|\.write\(|cat >>? *crates/|cp \S+ crates/|git apply|patch -p",c['cmd']) and SRC.search(c['cmd']): first=i; break
    if first is None: continue
    n+=1; pre=cs[:first]; span=cs[first]['t0']-a['start']; tot+=span
    td=0
    for c in pre:
        k=c['cls']; cnt[k]+=1; dur[k]+=c.get('dur') or 0; chars[k]+=c.get('rlen',0); td+=c.get('dur') or 0
        if c['tool']=='Read': files[re.sub(r'^.*/(crates|design|implementation|completions|scratchpad|\$S)/',r'\1/',c['cmd'])[-70:]]+=1
    mt+=span-td
print("%d fixers; %d s before the first source edit in total; tool time %d s, model time %d s (%.0f%%)"%(n,tot,tot-mt,mt,100*mt/tot))
print("class: calls, seconds, result kchars")
for k,v in cnt.most_common(12): print("  %-20s %4d %6.0f %7.0f"%(k,v,dur[k],chars[k]/1000))
print("total result chars before first edit: %.1f M (≈ %.0f k tokens per fixer)"%(sum(chars.values())/1e6,sum(chars.values())/4/n/1000))
print("files most often Read before the first edit (count over %d fixers):"%n)
for f,c in files.most_common(14): print("  %2d %s"%(c,f))
