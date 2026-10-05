#!/usr/bin/env python3
"""Waiting and waste: poll loops, refused commands, repeated commands, what filled the context
(result characters per class), big reads, files read by many agents, time-to-first-edit per fixer, rig use.
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: counts and sums; the twelve largest results each carry the
sanitised 110-character head of their command, and the file listings the last 80 characters of a sanitised path."""
import json,os,collections,datetime,re,sys
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',))
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
NAMES,_=E.runs()
CUT=E.cutoff()
C=[c for c in C if c['t0']<CUT]
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
def real(c): return c.get('bg_dur') if c.get('bg') and c.get('bg_dur') is not None else (c.get('dur') or 0)
bash=[c for c in C if c['tool']=='Bash']
# 1 polls
polls=[c for c in bash if c['cls']=='poll']
fg=[c for c in polls if not c.get('bg')]; bg=[c for c in polls if c.get('bg')]
print("1. poll/sleep commands: %d foreground (%s), %d background (%s real)"%(len(fg),hm(sum(c['dur'] or 0 for c in fg)),len(bg),hm(sum(real(c) for c in bg))))
what=collections.Counter()
for c in fg:
    cmd=c['cmd']
    k='gate log' if re.search(r'gate|GATE',cmd) else 'build/test log' if re.search(r'build|test|\.log|rc=',cmd) else 'pid/other'
    what[k]+=c['dur'] or 0
print("   foreground polls wait on:", ", ".join("%s=%s"%(k,hm(v)) for k,v in what.most_common()))
hit=[c for c in fg if (c['dur'] or 0)>=595]
print("   foreground polls that ran into the 10-minute tool ceiling (>=595 s): %d"%len(hit))
dbl=0
for c in bg:
    e=c.get('bg_end') or c['t0']
    if any(x['agent']==c['agent'] and x['t0']>=c['t0'] and x['t0']<e for x in fg): dbl+=1
print("   background polls shadowed by a foreground poll on the same thing: %d of %d"%(dbl,len(bg)))
# 2 refused
bl=[c for c in bash if c.get('blocked')]
print("\n2. commands refused by a hook (shell-guard etc.): %d ; by run: %s"%(len(bl),dict(collections.Counter(NAMES[c['run']] for c in bl))))
shapes=collections.Counter()
for c in bl:
    cmd=c['cmd']
    k='rm -r on a variable path' if re.search(r'\brm\s+-[a-zA-Z]*r',cmd) else 'exit code through a pipe' if re.search(r'\|\s*(tail|head|grep)',cmd) else 'other'
    shapes[k]+=1
print("   shapes:",dict(shapes))
to=[c for c in bash if c.get('timedout')]
print("   commands that hit their timeout: %d (%s)"%(len(to),hm(sum(c['dur'] or 0 for c in to))))
# 3 repeated identical commands
rep=collections.Counter(); repd=collections.Counter()
for c in bash:
    if c['cls'] in('poll',): continue
    k=(c['agent'],re.sub(r'\s+',' ',c['cmd'])[:400])
    rep[k]+=1; repd[k]+=real(c)
dups=[(k,n) for k,n in rep.items() if n>1]
print("\n3. byte-identical commands repeated by the same agent: %d distinct commands, %d extra executions, %s wall in the extras (upper bound)"%(len(dups),sum(n-1 for _,n in dups),hm(sum(repd[k]*(n-1)/n for k,n in dups))))
bycls=collections.Counter()
cl={ (c['agent'],re.sub(r'\s+',' ',c['cmd'])[:400]):c['cls'] for c in bash}
for k,n in dups: bycls[cl[k]]+=repd[k]*(n-1)/n
print("   by class:",", ".join("%s=%s"%(k,hm(v)) for k,v in bycls.most_common(8)))
# 4 context fill
fill=collections.Counter(); nfill=collections.Counter()
for c in C:
    if c['run']=='main': continue
    fill[c.get('cls')]+=c.get('rlen',0); nfill[c.get('cls')]+=1
tot=sum(fill.values())
print("\n4. tool-result characters returned to subagents, by class (total %.1f M chars ≈ %.1f M tokens at 4 chars/token):"%(tot/1e6,tot/4e6))
for k,v in fill.most_common(12): print("   %-22s %6.2f M chars  %4.1f%%  n=%d  mean %d"%(k,v/1e6,100*v/tot,nfill[k],v/max(1,nfill[k])))
big=sorted([c for c in C if c['run']!='main'],key=lambda c:-c.get('rlen',0))[:12]
print("   largest single results:")
for c in big: print("     %6dk chars %-10s %-9s %-24s %s"%(c['rlen']/1000,c['cls'][:10],NAMES[c['run']],(A[c['agent']].get('label') or '')[:24],' '.join(c['cmd'].split())[:110]))
pers=[c for c in bash if c.get('persisted')]
print("   results too large to return inline (persisted to a file): %d"%len(pers))
# 5 files read by many agents
rd=collections.defaultdict(set); rdc=collections.Counter(); rdn=collections.Counter()
for c in C:
    if c['tool']=='Read' and c['run']!='main':
        f=c['cmd']; rd[f].add(c['agent']); rdc[f]+=c.get('rlen',0); rdn[f]+=1
print("\n5. files read (Read tool) by the most distinct subagents: file — agents — reads — chars returned")
for f,s in sorted(rd.items(),key=lambda kv:-len(kv[1]))[:14]: print("   %-80s %3d %4d %6dk"%(f[-80:],len(s),rdn[f],rdc[f]/1000))
print("   most chars returned by one file across all reads:")
for f,v in rdc.most_common(10): print("   %-80s agents %3d reads %4d %6dk"%(f[-80:],len(rd[f]),rdn[f],v/1000))
# 6 time to first source edit
print("\n6. fixers: time and calls before the first edit to the repository's source (crates/, design/, dev/, …)")
SRC=re.compile(r'(^|[\s\'"=/])(crates/|design/|dev/|implementation/|DECISIONS\.md|completions/)')
rows=[]
for aid,a in A.items():
    if a.get('agent_type')!='build-fixer' or not a['run'].startswith('wf_'): continue
    cs=sorted([c for c in C if c['agent']==aid],key=lambda c:c['t0'])
    first=None
    for i,c in enumerate(cs):
        if c['tool'] in('Edit','Write') and SRC.search(c['cmd']) and '/scratchpad' not in c['cmd'] and not c['cmd'].startswith('$S'): first=(c['t0'],i); break
        if c['tool']=='Bash' and re.search(r"open\(p,\s*'w'\)|\.write\(|cat >>? *crates/|cp \S+ crates/|git apply|patch -p",c['cmd']) and SRC.search(c['cmd']): first=(c['t0'],i); break
    if first: rows.append((a['start'],NAMES[a['run']],a.get('label'),first[0]-a['start'],first[1],a['wall']))
for s,r,l,d,n,w in sorted(rows): print("   %-7s %-40s first edit after %s (%2.0f%% of its wall), %3d calls"%(r,(l or '')[:40],hm(d),100*d/w,n))
if rows: print("   sum of time-before-first-edit: %s over %d fixers; median %s"%(hm(sum(r[3] for r in rows)),len(rows),hm(sorted(r[3] for r in rows)[len(rows)//2])))
# 7 rig
rig=[c for c in bash if 'rig' in c.get('classes',[]) and re.search(r'jigc-rig',c['cmd'])]
st=collections.Counter()
for c in rig:
    for m in re.finditer(r'jigc-rig\s+(?:--[\w-]+(?:\s+\S+)?\s+)*([a-z][a-z-]+)',c['cmd']): st[m.group(1)]+=1
print("\n7. dev/jigc-rig: %d commands invoke it (%s wall for those whole commands; median %.1fs); states: %s"%(len(rig),hm(sum(real(c) for c in rig)),sorted(real(c) for c in rig)[len(rig)//2] if rig else 0,dict(st.most_common(8))))
print("   by run:",dict(collections.Counter(NAMES[c['run']] for c in rig)))
