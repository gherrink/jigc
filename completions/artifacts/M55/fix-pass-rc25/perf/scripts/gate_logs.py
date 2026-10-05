#!/usr/bin/env python3
"""Read every surviving dev/gate log in $TMPDIR (jigc-gate-XXXXXX + its -steps- and -hygiene- siblings,
matched by creation time) and write OUT_DIR/gate_logs.json: start (file birth), end (mtime), per-step
exit codes, the hygiene advisory's duration, compile seconds per step, nextest summary, SLOW and
failing tests. Prints a one-line-per-log table.
Reads: the gate logs where dev/gate left them — $TMPDIR, /tmp when it is unset, the same rule dev/gate follows.
A log's start is its file birth time, so the logs are read in place: a copy does not carry it.
Prints: per log its name's suffix, times, step results, counts and the head of its first compiler error line."""
import os,re,glob,json,datetime,sys
T=os.environ.get('TMPDIR','/tmp').rstrip('/')
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR',))
OUT=E.out_dir()
def st(p):
    s=os.stat(p); return s.st_birthtime,s.st_mtime,s.st_size
logs=[p for p in glob.glob(T+"/jigc-gate-??????") ]
steps=sorted([(st(p)[0],st(p)[1],p) for p in glob.glob(T+"/jigc-gate-steps-??????")])
hyg=sorted([(st(p)[0],st(p)[1],p) for p in glob.glob(T+"/jigc-gate-hygiene-??????")])
leaks=sorted([(st(p)[0],st(p)[1],p) for p in glob.glob(T+"/jigc-gate-gitleaks-??????")])
rows=[]
for p in logs:
    b,m,sz=st(p)
    txt=open(p,errors='replace').read()
    r=dict(log=os.path.basename(p),start=b,end=m,size=sz)
    # sibling steps file: first one born at/after this log's birth and before its end
    cand=[s for s in steps if b-0.5<=s[0]<=m+2]
    if cand:
        s=cand[0]; r['steps_birth']=s[0]; r['hygiene_s']=s[0]-b
        r['steps']={l.split()[0]:int(l.split()[1]) for l in open(s[2]) if len(l.split())==2}
    lk=[s for s in leaks if b-0.5<=s[0]<=m+2]
    hy=[s for s in hyg if b-0.5<=s[0]<=m+2]
    if hy: r['denylist_s']=hy[0][1]-hy[0][0]
    if lk: r['gitleaks_s']=lk[0][1]-lk[0][0]
    blocks=re.split(r'\n===== STEP (\w+) =====\n',txt)
    sb={blocks[i]:blocks[i+1] for i in range(1,len(blocks)-1,2)}
    r['step_names']=list(sb.keys())
    comp={}
    for k,v in sb.items():
        fm=re.findall(r'Finished `\w+` profile.*? in (?:(\d+)m )?([\d.]+)s',v)
        if fm: comp[k]=sum((int(a or 0)*60+float(s)) for a,s in fm)
        comp[k+'_compiled']=len(re.findall(r'^\s+Compiling ',v,re.M))
    r['compile']=comp
    t=sb.get('test','')
    sm=re.search(r'Starting (\d+) tests? across (\d+) binar',t)
    if sm: r['tests']=int(sm.group(1)); r['binaries']=int(sm.group(2))
    su=re.search(r'Summary \[\s*([\d.]+)s\]\s+(\S+) tests? run: (\d+) passed(?: \((\d+) slow\))?(?:, (\d+) failed)?',t)
    if su: r['nextest_s']=float(su.group(1)); r['passed']=int(su.group(3)); r['slow_n']=int(su.group(4) or 0); r['failed']=int(su.group(5) or 0)
    slow={}; fails={}
    for mm in re.finditer(r'^\s+(SLOW|FAIL|TIMEOUT|ABORT|LEAK-FAIL|SIG[A-Z]+) \[\s*>?\s*([\d.]+)s\] (?:\(\s*\d+/\d+\) +)?(\S+) (\S+)',t,re.M):
        d=slow if mm.group(1)=='SLOW' else fails
        d[mm.group(3)+' '+mm.group(4)]=max(d.get(mm.group(3)+' '+mm.group(4),0),float(mm.group(2)))
    r['slow']=slow; r['fails']=fails
    r['errors']=sorted(set(re.findall(r'^(error(?:\[E\d+\])?: .{0,110})',txt,re.M)))[:6]
    r['fmt_diff']=len(re.findall(r'^Diff in ',sb.get('fmt',''),re.M))
    rows.append(r)
rows.sort(key=lambda r:r['start'])
json.dump(rows,open(OUT+"/gate_logs.json","w"))
def t(x): return datetime.datetime.utcfromtimestamp(x).strftime('%m-%d %H:%M:%S')
print("%d logs"%len(rows))
for r in rows:
    s=r.get('steps',{})
    bad=[k for k,v in s.items() if v]
    print("%s %s %5.0fs hyg %4.1fs steps=%-28s tests=%s nextest=%s slow=%s fail=%d comp(test)=%s %s %s"%(r['log'][-6:],t(r['start']),r['end']-r['start'],r.get('hygiene_s',-1),
        ','.join(k for k in s)+(' RED:'+','.join(bad) if bad else ''),r.get('tests'),r.get('nextest_s'),r.get('slow_n'),len(r['fails']),r['compile'].get('test'),
        ('fmtdiff=%d'%r['fmt_diff']) if r['fmt_diff'] else '',(r['errors'][0][:60] if r['errors'] else '')))
