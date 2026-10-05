#!/usr/bin/env python3
"""Assemble the report: copy a source file to stdout, replacing each line `<!--include <path>-->` with the content
of that file (paths relative to the source file's directory), and refuse to emit a line that carries a host path.
Reads: the source file and the table files it includes. Prints: the assembled report, or nothing but the refusal.
  usage: build_report.py <source> > run-performance.md"""
import sys,os,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,args=1,usage='<source> > run-performance.md')
here=os.path.dirname(os.path.abspath(sys.argv[1]))
# each alternative is spelled with a group, so a plain-text scan of this directory for a host path finds only real ones
bad=re.compile(r'/(?:Users)/|/(?:private)/tmp|/(?:var)/folders|/home/[a-z]')
out=[]
for line in open(sys.argv[1]):
    m=re.match(r'\s*<!--include (\S+)-->\s*$',line)
    if m: out+=open(os.path.join(here,m.group(1))).read().rstrip('\n').split('\n')
    else: out.append(line.rstrip('\n'))
hits=[i+1 for i,l in enumerate(out) if bad.search(l)]
if hits: sys.exit("build_report: host path on output line(s) %s — not emitted"%hits[:10])
print('\n'.join(out))
