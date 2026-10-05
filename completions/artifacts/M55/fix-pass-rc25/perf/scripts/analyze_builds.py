#!/usr/bin/env python3
"""Builds: every cargo build (debug/release), cargo install, and scoped cargo test/nextest call — count, wall,
private-target flag — per run and per agent; the long ones listed.
Reads: OUT_DIR/agents.json, calls.jsonl. Prints: counts and wall sums; each listed long build carries the
sanitised 150-character head of its command, as stored in calls.jsonl."""
import json,os,collections,datetime,sys,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',),usage='[-v]')
D=E.out_dir()
A={a['agent']:a for a in json.load(open(D+"/agents.json"))}
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
NAMES,_=E.runs()
CUT=E.cutoff()
def hm(s): return "%d:%02d:%02d"%(s//3600,(s%3600)//60,s%60)
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M:%S')
def real(c): return c.get('bg_dur') if c.get('bg') and c.get('bg_dur') is not None else (c.get('dur') or 0)
B=[c for c in C if c['tool']=='Bash' and c['t0']<CUT and any(k in c.get('classes',[]) for k in('cargo-build-release','cargo-build-debug','cargo-install','cargo-test','cargo-clippy','cargo-fmt','cargo-other'))]
for kind in('cargo-build-release','cargo-build-debug','cargo-install','cargo-test','cargo-clippy','cargo-fmt'):
    rows=[c for c in B if kind in c['classes'] and c['cls'] not in('gate-full','gate-quick','docker','codex')]
    tot=sum(real(c) for c in rows)
    pt=[c for c in rows if c.get('detail',{}).get('private_target')]
    print("\n%s: %d calls, wall %s; private-target %d calls / %s; >=60s: %d calls / %s; <5s: %d"%(kind,len(rows),hm(tot),len(pt),hm(sum(real(c) for c in pt)),sum(1 for c in rows if real(c)>=60),hm(sum(real(c) for c in rows if real(c)>=60)),sum(1 for c in rows if real(c)<5)))
    pr=collections.defaultdict(lambda:[0,0.0])
    for c in rows:
        e=pr[NAMES[c['run']]]; e[0]+=1; e[1]+=real(c)
    print("   per run: "+"  ".join("%s=%d/%s"%(k,v[0],hm(v[1])) for k,v in pr.items()))
    if kind in('cargo-build-release','cargo-build-debug','cargo-install') or '-v' in sys.argv:
        for c in sorted(rows,key=lambda c:-real(c))[:22 if kind!='cargo-test' else 30]:
            if real(c)<20: break
            print("   %s %-8s %-24s %s %s%s | %s"%(hm(real(c)),NAMES[c['run']],(A[c['agent']].get('label') or '')[:24],t(c['t0']),'PT ' if c.get('detail',{}).get('private_target') else '','BG' if c.get('bg') else '',' '.join(c['cmd'].split())[:150]))
# release builds per fixer
print("\nrelease builds per agent (count, wall, max):")
pa=collections.defaultdict(list)
for c in B:
    if 'cargo-build-release' in c['classes'] and c['cls'] not in('gate-full','docker'): pa[c['agent']].append(real(c))
for a,v in sorted(pa.items(),key=lambda kv:A[kv[0]]['start']): print("   %-8s %-40s n=%2d wall %s max %3.0fs median %3.0fs"%(NAMES[A[a]['run']],(A[a].get('label') or '')[:40],len(v),hm(sum(v)),max(v),sorted(v)[len(v)//2]))
