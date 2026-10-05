#!/usr/bin/env python3
"""Markdown table: wall per Bash command class per run (foreground call time; a background command counts its
launch→notification time), with call counts. Non-exclusive: a foreground poll and the background gate it waits on both count.
Reads: OUT_DIR/calls.jsonl. Prints: one markdown table of times and counts."""
import json,os,collections,datetime
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('OUT_DIR','RUNS_FILE','CUTOFF_UTC',))
D=E.out_dir()
C=[json.loads(l) for l in open(D+"/calls.jsonl")]
NAMES,ORDER=E.runs()
RUNS=[NAMES[k] for k in ORDER if k!='main']
CUT=E.cutoff()
LAB=[('gate-full','`dev/gate` (full)'),('gate-quick','`dev/gate --quick`'),('poll','sleep / wait / poll loops'),('cargo-test','`cargo test` / `nextest`, scoped'),('cargo-build-release','`cargo build --release`'),('cargo-build-debug','`cargo build` (debug)'),('cargo-clippy','`cargo clippy`'),('cargo-fmt','`cargo fmt`'),('rig','`dev/jigc-rig` (+ what the same command drove)'),('drive','driving the binary directly'),('script','scratch scripts (mostly route drives)'),('trial-tooling','trial harness / driver'),('docker','docker / `dev/runner-faithful`'),('codex','codex'),('git','git / gh'),('python','python (patch and analysis scripts)'),('read','file reads via the shell'),('cargo-install','`cargo install`'),('cargo-other','other cargo')]
def real(c): return c.get('bg_dur') if c.get('bg') and c.get('bg_dur') is not None else (c.get('dur') or 0)
s=collections.defaultdict(collections.Counter); n=collections.defaultdict(collections.Counter)
for c in C:
    if c['tool']!='Bash' or c['t0']>=CUT or c['run']=='main': continue
    s[c['cls']][NAMES[c['run']]]+=real(c); n[c['cls']][NAMES[c['run']]]+=1
def f(x): return ("%d:%02d"%(x//3600,(x%3600)//60)) if x>=60 else ("%ds"%x if x>=1 else '·')
print("| class | "+" | ".join(RUNS)+" | all | calls |\n|---|"+"---|"*(len(RUNS)+2))
for k,l in LAB:
    if not n[k]: continue
    print("| %s | "%l+" | ".join(f(s[k][r]) for r in RUNS)+" | %s | %d |"%(f(sum(s[k].values())),sum(n[k].values())))
