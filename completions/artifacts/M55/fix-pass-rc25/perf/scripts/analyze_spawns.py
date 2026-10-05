#!/usr/bin/env python3
"""Summarize spawn-shim logs: per program and verb — calls, summed seconds, mean ms — at depth 0 (spawned by the
test) and depth 1 (git spawned from inside jigc).
Reads: the .spawns files spawn_probe.sh wrote. Prints: calls and seconds per program and verb.
  usage: analyze_spawns.py <file.spawns>..."""
import sys,collections,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,args=1,usage='<file.spawns>...')
tot=collections.Counter(); n=collections.Counter()
for f in sys.argv[1:]:
    for l in open(f,errors='replace'):
        p=l.split()
        if len(p)<3: continue
        name,depth,sec=p[0],p[1],float(p[2]); args=p[3:]
        # verb = first arg that is not an option or a masked path
        verb=None; i=0
        while i<len(args):
            a=args[i]
            if a in('-C','-c','--git-dir','--work-tree','--format'): i+=2; continue
            if a.startswith('-') or a=='_': i+=1; continue
            verb=a; break
        if name=='jigc' and verb in('task','doc','milestone','config','item') and i+1<len(args): verb=verb+' '+args[i+1]
        k=(name,depth,verb or '?'); tot[k]+=sec; n[k]+=1
T0=sum(v for (nm,d,_),v in tot.items() if d=='0')
print("depth-0 child time %.1f s over %d calls; nested git %.1f s over %d calls"%(T0,sum(c for (nm,d,_),c in n.items() if d=='0'),sum(v for (nm,d,_),v in tot.items() if d=='1'),sum(c for (nm,d,_),c in n.items() if d=='1')))
for d,label in(('0','spawned by the test'),('1','git spawned inside jigc')):
    print(label+':')
    for k,v in sorted(((k,v) for k,v in tot.items() if k[1]==d),key=lambda kv:-kv[1])[:16]:
        print("   %-5s %-22s %6d calls %8.1f s %6.0f ms/call %5.1f%%"%(k[0],k[2][:22],n[k],v,1000*v/n[k],100*v/T0 if d=='0' else 0))
