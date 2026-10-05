#!/usr/bin/env python3
"""Per-step timings of the trial operator run from its .start/.end/.rc marker files.
Reads: the *.start, *.end and *.rc files of the directory named. Prints: one line per step (its name, start,
duration, exit code) and the sum.
  usage: trial_steps.py <directory of the marker files>"""
import os,sys,glob,datetime,re
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,args=1,usage='<directory of the marker files>')
L=sys.argv[1]
def rd(p):
    try: return open(p).read().strip()
    except Exception: return None
def ts(s):
    try: return datetime.datetime.fromisoformat(s.replace('Z','+00:00')).timestamp()
    except Exception: return None
rows=[]
for st in sorted(glob.glob(L+"/*.start")):
    base=st[:-6]; s=ts(rd(st) or ''); e=ts(rd(base+'.end') or ''); rc=rd(base+'.rc')
    rows.append((os.path.basename(base),s,e,rc))
tot=0
for n,s,e,rc in rows:
    d=(e-s) if s and e else None
    if d: tot+=d
    print("  %-34s %s  %8s  rc=%s"%(n,datetime.datetime.utcfromtimestamp(s).strftime('%m-%d %H:%M:%S') if s else '?', ("%dm%02ds"%(d//60,d%60)) if d is not None else '?',rc))
print("  sum of step durations: %dm%02ds; first start → last end: %s"%(tot//60,tot%60, ("%dm"%((max(e for _,_,e,_ in rows if e)-min(s for _,s,_,_ in rows if s))/60)) if rows else '-'))
