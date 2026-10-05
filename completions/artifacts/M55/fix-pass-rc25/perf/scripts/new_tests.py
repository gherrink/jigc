#!/usr/bin/env python3
"""Which tests did the fix pass add? Compare `#[test] fn` names per file between the branch's merge-base with
origin/main and a given revision (read-only `git show`). Writes OUT_DIR/new_tests.json: {"suite::test": "new-suite"|"grown-suite"}.
Reads: the repository named, through read-only git. Prints: counts and the paths of the files that gained tests.
  usage: new_tests.py <repo> <rev>"""
import subprocess,sys,re,json,os
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR',),args=2,usage='<repo> <rev>')
repo,rev=sys.argv[1],sys.argv[2]
def git(*a): return subprocess.run(['git','-C',repo]+list(a),capture_output=True,text=True).stdout
base=git('merge-base','origin/main',rev).strip()
def tests(text):
    out=set(); lines=text.split('\n')
    for i,l in enumerate(lines):
        if l.strip()=='#[test]':
            for j in range(i+1,min(i+6,len(lines))):
                m=re.match(r'\s*(?:pub\s+)?fn\s+([A-Za-z0-9_]+)\s*\(',lines[j])
                if m: out.add(m.group(1)); break
    return out
res={}; stats=[]
for line in git('diff','--name-status',base,rev,'--','crates/cli/tests','crates/cli/src','crates/engine/src').split('\n'):
    if not line.strip(): continue
    st,*paths=line.split('\t'); p=paths[-1]
    if not p.endswith('.rs'): continue
    new=tests(git('show',rev+':'+p))
    old=tests(git('show',base+':'+p)) if not st.startswith('A') else set()
    added=new-old
    if not added: continue
    suite=os.path.basename(p)[:-3] if '/tests/' in p else p
    kind='new-suite' if st.startswith('A') else 'grown-suite'
    for t in added: res[suite+'::'+t]=kind
    stats.append((len(added),kind,p))
D=E.out_dir()
json.dump(dict(base=base,rev=rev,tests=res),open(D+"/new_tests.json","w"))
print("base",base[:8],"rev",rev[:8],"added tests:",len(res),"in new suites:",sum(1 for v in res.values() if v=='new-suite'))
for n,k,p in sorted(stats,reverse=True)[:25]: print("  %3d %-11s %s"%(n,k,p))
