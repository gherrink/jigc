#!/usr/bin/env python3
"""Could a fast tier have caught the red gates? Uses the per-test timings of the private full-suite run and the
gate ledger: for each duration threshold, how many tests fall under it, what they cost (summed seconds and wall at
16 threads), and how many of the red gates had ALL their failing tests under it. Also the cost of the named fence suites.
Reads: OUT_DIR/per_test_suite.json, gate_ledger.json. Prints: counts, seconds and test names."""
import json,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','CUTOFF_UTC',))
D=E.out_dir()
T=json.load(open(D+"/per_test_suite.json"))
led=json.load(open(D+"/gate_ledger.json"))
CUT=E.cutoff()
reds=[r for r in led if r['kind']=='red' and r['start']<CUT]
tot=sum(T.values())
FENCE=['git_span_aim','task_area_writer_registry','migrate_route_family','message_whitespace_fence','rollback_population_registry','repo_relative_paths','dry_run_findings_equal_set','count_fences','foldback_truth','help_truth','flow52_acceptance','test_target_registration']
print("threshold: tests under it, summed s, wall at 16 threads, red gates fully caught (test failures only; fmt/clippy reds are caught by the lint steps)")
for th in(0.25,0.5,1,2,5,10):
    sub={k:v for k,v in T.items() if v<th}
    caught=0; lintonly=0
    for r in reds:
        if not r['fails']: lintonly+=1; continue
        if all(T.get(n,99)<th for n in r['fails']): caught+=1
    print("  <%5.2fs  %4d tests (%2.0f%%)  %6.0f s summed  ≈%4.0f s wall   catches %2d of %d test-red gates (+%d lint-only reds)"%(th,len(sub),100*len(sub)/len(T),sum(sub.values()),sum(sub.values())/16,caught,len(reds)-lintonly,lintonly))
fs=collections.Counter(); fn=collections.Counter()
for k,v in T.items():
    s=k.split(' ')[1].split('::')[0]
    if s in FENCE: fs[s]+=v; fn[s]+=1
print("\nthe fence suites that reddened gates, whole suites: %d tests, %.0f s summed (≈%.0f s wall at 16 threads)"%(sum(fn.values()),sum(fs.values()),sum(fs.values())/16))
for s,v in fs.most_common(): print("   %-34s %3d tests %6.1f s"%(s,fn[s],v))
# the specific failing tests' durations in the clean run
print("\nthe 25 distinct tests that reddened a gate, their duration in the clean run:")
names=collections.Counter()
for r in reds:
    for n in r['fails']: names[n]+=1
for n,c in names.most_common(): print("   %2d× %6.2fs %s"%(c,T.get(n,-1),n[:110]))
# group-level: which group binaries would a touched-area pre-check need
byb=collections.Counter()
for k,v in T.items(): byb[k.split(' ')[0]]+=v
print("\nsummed seconds per group (wall if run alone at 16 threads):")
for b,v in byb.most_common(12): print("   %-28s %6.0f s  ≈%4.0f s"%(b,v,v/16))
