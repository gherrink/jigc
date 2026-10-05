#!/usr/bin/env python3
"""Per-test timings from a nextest log run with --status-level pass: totals per binary and per suite (module),
the slowest tests, the share the fix pass added (OUT_DIR/new_tests.json), and the duration distribution.
Reads: the nextest log named, OUT_DIR/new_tests.json when present. Writes OUT_DIR/per_test_<name of the log's
directory>.json. Prints: sums, shares and test names.
  usage: analyze_suite.py <nextest-log> [-n N]"""
import re,sys,json,os,collections
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR',),args=1,usage='<nextest-log> [-n N]')
log=sys.argv[1]; N=int(sys.argv[sys.argv.index('-n')+1]) if '-n' in sys.argv else 25
D=E.out_dir()
new=json.load(open(D+"/new_tests.json"))['tests'] if os.path.exists(D+"/new_tests.json") else {}
rx=re.compile(r'^\s+(PASS|FAIL|TIMEOUT|ABORT|LEAK|LEAK-FAIL|FLAKY|SIG[A-Z]+)\s+\[\s*>?\s*([\d.]+)s\]\s+(?:\(\s*\d+/\d+\)\s+)?(\S+)\s+(\S+)')
tests={}; status=collections.Counter()
summary=None
for l in open(log,errors='replace'):
    m=rx.match(l)
    if m:
        st,d,b,n=m.group(1),float(m.group(2)),m.group(3),m.group(4)
        if (b,n) not in tests: status[st]+=1
        tests[(b,n)]=(d,st)
    elif 'Summary [' in l: summary=l.strip()
    elif 'Starting ' in l and ' tests across ' in l: start=l.strip()
print(summary); print("parsed %d tests; statuses %s"%(len(tests),dict(status)))
tot=sum(d for d,_ in tests.values())
print("sum of per-test durations: %.0f s (= %.1f test-threads busy for the summary wall)"%(tot,tot/float(re.search(r'\[\s*([\d.]+)s\]',summary).group(1)) if summary else 0))
def suite(b,n):
    parts=n.split('::')
    if b.startswith('jigc::g_'): return parts[0]
    return b+' (unit) '+('::'.join(parts[:-1]).replace('::tests','') or '-')
byb=collections.defaultdict(lambda:[0,0.0,0.0]); bys=collections.defaultdict(lambda:[0,0.0,0.0,None])
isnew=lambda b,n: new.get(suite(b,n)+'::'+n.split('::')[-1]) or new.get(n.split('::')[0]+'::'+n.split('::')[-1])
newtot=0; newn=0; newkinds=collections.Counter()
for (b,n),(d,st) in tests.items():
    e=byb[b]; e[0]+=1; e[1]+=d; e[2]=max(e[2],d)
    s=suite(b,n); e=bys[(b,s)]; e[0]+=1; e[1]+=d; e[2]=max(e[2],d)
    k=isnew(b,n)
    if not k and not b.startswith('jigc::g_'):
        # unit tests: new_tests keys are "<path>::<test>"
        for key in new:
            if key.endswith('::'+n.split('::')[-1]) and key.startswith('crates/'): k=new[key]; break
    if k: newtot+=d; newn+=1; newkinds[k]+=d; e[3]=k if e[3] in(None,k) else 'grown-suite'
print("\nper binary: tests, summed seconds, share, mean s/test, max")
for b,(c,s,m) in sorted(byb.items(),key=lambda kv:-kv[1][1]): print("  %-32s %5d %8.0f %5.1f%%  %5.2f  %6.1f"%(b,c,s,100*s/tot,s/c,m))
print("\ntests the fix pass added: %d of %d (%.1f%%) carrying %.0f s of %.0f s (%.1f%%); mean %.2f s vs %.2f s for the rest; by kind %s"%(newn,len(tests),100*newn/len(tests),newtot,tot,100*newtot/tot,newtot/max(1,newn),(tot-newtot)/max(1,len(tests)-newn),{k:round(v) for k,v in newkinds.items()}))
print("\ntop %d suites by summed seconds: suite, binary, tests, sum s, share, mean, max, [fix pass]"%N)
for (b,s),(c,sm,m,k) in sorted(bys.items(),key=lambda kv:-kv[1][1])[:N]:
    print("  %-46s %-18s %4d %7.0f %5.1f%% %6.2f %6.1f  %s"%(s[:46],b.replace('jigc::',''),c,sm,100*sm/tot,sm/c,m,k or ''))
cum=0; ranked=sorted(bys.items(),key=lambda kv:-kv[1][1]); n10=None
for i,(k,v) in enumerate(ranked):
    cum+=v[1]
    if cum>=0.5*tot and n10 is None: n10=i+1
print("  %d suites of %d carry half of the summed time"%(n10,len(ranked)))
print("\ntop %d tests: seconds, binary, test, [fix pass]"%N)
for (b,n),(d,st) in sorted(tests.items(),key=lambda kv:-kv[1][0])[:N]:
    print("  %7.1f %-14s %s %s"%(d,b.replace('jigc::',''),n[:118],'['+isnew(b,n)+']' if isnew(b,n) else ''))
ds=sorted(d for d,_ in tests.values())
def pct(p): return ds[min(len(ds)-1,int(p*len(ds)))]
print("\ndistribution: median %.2fs p90 %.2fs p99 %.1fs max %.1fs; tests >=10s: %d carrying %.0f%%; tests <0.5s: %d carrying %.1f%%"%(pct(.5),pct(.9),pct(.99),ds[-1],sum(1 for d in ds if d>=10),100*sum(d for d in ds if d>=10)/tot,sum(1 for d in ds if d<0.5),100*sum(d for d in ds if d<0.5)/tot))
json.dump({b+' '+n:d for (b,n),(d,_) in tests.items()},open(D+"/per_test_"+os.path.basename(os.path.dirname(os.path.abspath(log)))+".json","w"))
