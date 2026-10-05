#!/usr/bin/env python3
"""Parse every transcript of the session into two datasets under OUT_DIR:

  agents.json  one record per agent (run, label, phase, type, start, end, tokens, call counts,
               model/tool/idle seconds)
  calls.jsonl  one record per tool call (agent, tool, class, start, end, seconds, background flag,
               background completion, result size, error flag, a SANITIZED 600-char command head)

Reads: SESSION_DIR — the session's own transcript (<SESSION_DIR>.jsonl), every subagent transcript under
subagents/, each workflow run's journal under subagents/workflows/<run id>/ and its run file under workflows/.
Prints: counts only — records per dataset, calls per class, and the first words of the commands it could not
class. Commands are sanitized (home path -> ~, NAME=value for secret-shaped names -> NAME=<redacted>,
token-shaped strings -> <redacted>) before they are stored; calls.jsonl still holds command heads from private
transcripts and is not for publication.
"""
import json,glob,os,re,sys,datetime,collections
import sys; sys.dont_write_bytecode=True; import perf_env as E
E.start(__doc__,env=('SESSION_DIR','OUT_DIR',))
HOME=os.path.expanduser("~")
B=E.session_dir()
OUT=E.out_dir()
os.makedirs(OUT,exist_ok=True)

def ts(s): return datetime.datetime.fromisoformat(s.replace('Z','+00:00')).timestamp()
SECRET_NAME=re.compile(r'\b([A-Za-z0-9_]*(?:TOKEN|SECRET|PASSWORD|PASSWD|API_?KEY|CREDENTIAL|AUTH)[A-Za-z0-9_]*)=(\S+)',re.I)
TOKENISH=re.compile(r'\b(?:sk-[A-Za-z0-9_-]{8,}|gh[pousr]_[A-Za-z0-9]{8,}|cio[A-Za-z0-9]{20,}|xox[abprs]-[A-Za-z0-9-]{8,}|eyJ[A-Za-z0-9_-]{20,}|[A-Fa-f0-9]{48,})\b')
# the path alternatives below are spelled with a group, so a plain-text scan of this directory for a host path finds only real ones
SCR=re.compile(r'/(?:private)/tmp/claude-\d+/[^/\s"\']+/[0-9a-f-]{36}/scratchpad')
TMPD=re.compile(r'/(?:private/)?var/folders/[^/\s]+/[^/\s]+/T')
def clean(s):
    s=SCR.sub('$S',s); s=TMPD.sub('$TMPDIR',s)
    s=s.replace(HOME,'~')
    s=SECRET_NAME.sub(lambda m:m.group(1)+'=<redacted>',s)
    s=TOKENISH.sub('<redacted>',s)
    s=re.sub(r'/(?:Users)/[^/\s]+','~',s)
    return s
def txt(c):
    if isinstance(c,str): return c
    if not isinstance(c,list): return ''
    return "\n".join(b.get('text','') for b in c if isinstance(b,dict) and b.get('type')=='text')

# ---------------- command classification ----------------
READERS={'cat','sed','head','tail','grep','rg','ugrep','wc','ls','find','awk','jq','cut','sort','uniq','less','bat','stat','file','diff','cmp','tree','du','df','nl','tr','xxd','od','strings','comm','shasum','md5','sha256sum','basename','dirname','realpath','readlink','test','[','echo','printf','true','false','date','pwd','which','type','env','printenv','export','set','unset','local','read','mkdir','touch','cp','mv','ln','chmod','tee','mktemp','rm','rmdir','id','uname','sw_vers','nproc','sysctl','ps','pgrep','lsof','top','kill','pkill','exit','return','break','continue','then','else','fi','do','done','esac','case','if','for','while','until','in','{','}','(',')','[[','eval','source','.','trap','shift','local','typeset','declare','IFS','command','builtin','xargs','time','timeout','gtimeout','nohup','exec','setopt','unsetopt','alias','cd','pushd','popd','wait','sleep','open','tput','column','paste','fold','rev','seq','yes','hexdump','base64','zcat','gzip','gunzip','tar','unzip','zip','curl','wget','ssh','scp','rsync','make','sh','bash','zsh','python','python3','node','perl','ruby'}
PRIO=['gate-full','docker','codex','cargo-build-release','cargo-install','cargo-build-debug','cargo-test','cargo-clippy','cargo-fmt','gate-quick','cargo-other','poll','trial-tooling','rig','drive','script','gate-report','git','python','read','other']
def first_words(seg):
    """Yield the command word of a simple-command segment, skipping assignments and wrappers."""
    toks=seg.strip().split()
    i=0; out=[]
    while i<len(toks):
        t=toks[i].lstrip('({!').strip()
        if not t: i+=1; continue
        if re.match(r'^[A-Za-z_][A-Za-z0-9_]*=',t):
            # VAR=$(cmd ...) — the command is inside the substitution
            m=re.match(r'^[A-Za-z_][A-Za-z0-9_]*=["\']?\$\((.*)',t)
            if m and m.group(1): out.append(m.group(1)); return out,toks[i+1:]
            i+=1; continue
        if t in ('time','nohup','command','exec','builtin','then','else','do','if','while','until','!','{','sudo','env','caffeinate'):
            if t in('while','until'): out.append(t)
            i+=1; continue
        if t in('timeout','gtimeout'):
            i+=2; continue
        out.append(t); return out,toks[i+1:]
    return out,[]
def split_segments(c):
    """Quote-aware split into simple-command segments: unquoted ; && || | newline, and $( / backtick
    (also inside double quotes). Nothing inside single quotes is split. Quoted text stays in the segment."""
    segs=[]; cur=[]; i=0; n=len(c); q=None
    while i<n:
        ch=c[i]
        if q=="'":
            cur.append(ch)
            if ch=="'": q=None
            i+=1; continue
        if ch=='\\' and i+1<n:
            cur.append(c[i:i+2]); i+=2; continue
        if q=='"':
            if ch=='"': q=None; cur.append(ch); i+=1; continue
            if ch=='$' and i+1<n and c[i+1]=='(' and not (i+2<n and c[i+2]=='('):
                segs.append(''.join(cur)); cur=[]; i+=2; q=None; continue   # leave quote context: approximation
            cur.append(ch); i+=1; continue
        if ch in('"',"'"):
            q=ch; cur.append(ch); i+=1; continue
        if ch=='$' and i+1<n and c[i+1]=='(' and not (i+2<n and c[i+2]=='('):
            segs.append(''.join(cur)); cur=[]; i+=2; continue
        if ch=='`' or ch==';' or ch=='\n':
            segs.append(''.join(cur)); cur=[]; i+=1; continue
        if ch=='&' and i+1<n and c[i+1]=='&':
            segs.append(''.join(cur)); cur=[]; i+=2; continue
        if ch=='|':
            segs.append(''.join(cur)); cur=[]; i+=2 if (i+1<n and c[i+1]=='|') else 1; continue
        cur.append(ch); i+=1
    segs.append(''.join(cur))
    return segs
def classify(cmd):
    classes=set(); detail={}
    c=cmd
    # strip heredoc bodies so their text is not read as commands
    c=re.sub(r"<<-?\s*['\"]?(\w+)['\"]?.*?\n.*?\n\s*\1\b",' ',c,flags=re.S)
    has_sleep=False
    funcs=set(re.findall(r'(?:^|[\s;])(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*\(\)\s*\{',c))
    sources=bool(re.search(r'(?:^|[\s;&])(?:source|\.)\s+\S',c))
    for seg in split_segments(c):
        seg=seg.strip()
        if not seg: continue
        fw,rest=first_words(seg)
        if not fw: continue
        w=fw[-1]; base=w.strip('"\'').split('/')[-1]
        full=w.strip('"\'')
        r=' '.join(rest)
        if 'while' in fw[:-1] or 'until' in fw[:-1]: detail['loop']=1
        if base=='gate' and 'dev/gate' in full or full in('dev/gate','./dev/gate'):
            if '--report' in r: classes.add('gate-report')
            elif '--quick' in r: classes.add('gate-quick')
            elif '--help' in r or '-h' in rest: classes.add('read')
            else:
                classes.add('gate-full')
            if '--private-target' in r or 'JIGC_GATE_TARGET' in seg or 'CARGO_TARGET_DIR=' in seg: detail['private_target']=1
        elif base in('bash','sh','zsh') and rest and re.search(r'dev/gate$',rest[0].strip('"\'')):
            rr=' '.join(rest[1:])
            classes.add('gate-report' if '--report' in rr else 'gate-quick' if '--quick' in rr else 'gate-full')
        elif base=='cargo':
            sub=None
            for x in rest:
                if x.startswith('+') or x.startswith('-'): continue
                sub=x; break
            if 'CARGO_TARGET_DIR=' in seg or '--target-dir' in r: detail['private_target']=1
            if sub=='build': classes.add('cargo-build-release' if ('--release' in r or '--profile release' in r) else 'cargo-build-debug')
            elif sub in('test','nextest'):
                classes.add('cargo-test')
                if '--release' in r: detail['test_release']=1
                if '--no-run' in r: detail['no_run']=1
            elif sub=='clippy': classes.add('cargo-clippy')
            elif sub=='fmt': classes.add('cargo-fmt')
            elif sub=='install': classes.add('cargo-install')
            elif sub in('check','clean','metadata','tree','doc','run','publish','package','llvm-cov','expand','--version','-V',None): classes.add('cargo-other'); detail['cargo_sub']=sub
            else: classes.add('cargo-other'); detail['cargo_sub']=sub
        elif base in('docker','runner-faithful','podman','colima'): classes.add('docker')
        elif base=='codex': classes.add('codex')
        elif base=='jigc-rig': classes.add('rig')
        elif base in('jigc','$JIGC','${JIGC}','$BIN','$J','$JIGC_BIN','${BIN}','$jigc','$bin') or re.fullmatch(r'\$\{?(JIGC|BIN|J|REL|RELBIN|DBG|JB|JR|JD)[A-Z0-9_]*\}?',base or ''):
            classes.add('drive')
        elif base=='git': classes.add('git')
        elif base=='gh': classes.add('git')
        elif base=='sleep': has_sleep=True
        elif base=='wait': has_sleep=True
        elif base in('python','python3'):
            classes.add('python')
        elif base in('bash','sh','zsh'):
            tgt=rest[0].strip('"\'') if rest else ''
            tb=tgt.split('/')[-1]
            if re.search(r'(build-image|verify-image|verify-pair|run-session|instantiate|check-corpus)',tb): classes.add('trial-tooling')
            elif tgt.startswith('-c') or tgt=='-lc': classes.add('other')
            else: classes.add('script')
        elif re.search(r'(build-image|verify-image|verify-pair|run-session|instantiate|check-corpus)\.sh$',full) or re.search(r'trial-driver/(run|walk)\.py',seg): classes.add('trial-tooling')
        elif base in READERS or base.startswith('$') and False: classes.add('read')
        elif full.endswith('.sh') or full.startswith('./') or full.startswith('$S/') or '/scratchpad/' in full: classes.add('script')
        elif re.fullmatch(r'[A-Za-z_][A-Za-z0-9_]*(\(\))?',base or '') and (base.rstrip('()') in funcs or sources):
            b0=base.rstrip('()')
            if base.endswith('()'): pass
            elif 'rig' in b0.lower(): classes.add('rig')
            else: classes.add('drive'); detail['via_helper']=1
        else:
            classes.add('other'); detail.setdefault('other_words',[]).append(base[:30])
    if re.search(r'trial-driver/(run|walk)\.py',c) or re.search(r'trial-harness/[a-z-]+\.sh',c): classes.add('trial-tooling')
    if has_sleep and not (classes & {'gate-full','docker','codex','cargo-build-release','cargo-install','cargo-build-debug','cargo-test','cargo-clippy','cargo-fmt','gate-quick','trial-tooling'}):
        classes.add('poll')
    if has_sleep: detail['has_sleep']=1
    for p in PRIO:
        if p in classes: return p,sorted(classes),detail
    return 'other',sorted(classes),detail

def tokens_of(usage):
    return dict(inp=usage.get('input_tokens',0) or 0, cc=usage.get('cache_creation_input_tokens',0) or 0,
                cr=usage.get('cache_read_input_tokens',0) or 0, out=usage.get('output_tokens',0) or 0)

def parse(path,run,agent_id,meta):
    entries=[]
    with open(path) as fh:
        for l in fh:
            try: entries.append(json.loads(l))
            except Exception: pass
    uses={}; order=[]; msgs={}; notif={}
    first=None; last=None
    timeline=[]  # (t, kind) kind in 'a' assistant block, 'u' user/tool_result, 'x' other
    end_turn_ts=[]
    prompt_len=0; ntext=0
    final_text_len=0; so_len=0
    for d in entries:
        t=d.get('type')
        if 'timestamp' not in d: continue
        tt=ts(d['timestamp'])
        if first is None: first=tt
        last=tt if last is None else max(last,tt)
        if t=='assistant':
            m=d['message']; mid=m.get('id')
            u=m.get('usage') or {}
            tk=tokens_of(u)
            if mid:
                prev=msgs.get(mid)
                if prev is None or tk['out']>=prev['out']: msgs[mid]=dict(tk,t=tt,model=m.get('model'))
            timeline.append((tt,'a',m.get('stop_reason')))
            if m.get('stop_reason')=='end_turn': end_turn_ts.append(tt)
            for b in m.get('content',[]):
                if b.get('type')=='tool_use':
                    inp=b.get('input') or {}
                    uses[b['id']]=dict(id=b['id'],name=b['name'],t0=tt,inp=inp)
                    order.append(b['id'])
                    if b['name']=='StructuredOutput': so_len=len(json.dumps(inp))
                elif b.get('type')=='text':
                    final_text_len=len(b.get('text',''))
        elif t=='user':
            c=d['message']['content']
            if isinstance(c,str):
                if prompt_len==0: prompt_len=len(c)
                timeline.append((tt,'p',None))
            else:
                got=False
                for b in c:
                    if b.get('type')=='tool_result':
                        got=True
                        u=uses.get(b['tool_use_id'])
                        if u is None: continue
                        s=txt(b.get('content') or '')
                        u['t1']=tt; u['rlen']=len(s); u['err']=bool(b.get('is_error'))
                        u['rtext']=s
                        tur=d.get('toolUseResult')
                        if isinstance(tur,dict) and tur.get('backgroundTaskId'): u['bgid']=tur['backgroundTaskId']
                        m=re.search(r'Command running in background with ID: (\w+)',s[:200])
                        if m: u['bgid']=m.group(1)
                        if 'was moved to the background' in s[:400]:
                            u['auto_bg']=1
                            m=re.search(r'ID:? (\w{6,12})',s[:400])
                            if m: u['bgid']=m.group(1)
                    elif b.get('type')=='text' and prompt_len==0: prompt_len=len(b.get('text',''))
                timeline.append((tt,'u' if got else 'p',None))
        elif t=='attachment':
            a=d['attachment']
            if a.get('type')=='queued_command' and a.get('commandMode')=='task-notification':
                p=str(a.get('prompt'))
                m=re.search(r'<task-id>(\w+)</task-id>',p)
                m2=re.search(r'<tool-use-id>(\w+)</tool-use-id>',p)
                st=re.search(r'<status>(\w+)</status>',p)
                ex=re.search(r'exit code (-?\d+)',p)
                at=ts(a['timestamp']) if a.get('timestamp') else tt
                rec=dict(t=min(at,tt),status=st.group(1) if st else None,exit=int(ex.group(1)) if ex else None)
                if m: notif[m.group(1)]=rec
                if m2: notif[m2.group(1)]=rec
                timeline.append((tt,'n',None))
    calls=[]
    for uid in order:
        u=uses[uid]
        inp=u['inp']
        rec=dict(run=run,agent=agent_id,tool=u['name'],t0=u['t0'],t1=u.get('t1'),err=u.get('err',False),rlen=u.get('rlen',0))
        rec['dur']=(u['t1']-u['t0']) if u.get('t1') else None
        rt=u.get('rtext','')
        if u['name']=='Bash':
            cmd=inp.get('command','') or ''
            cls,classes,detail=classify(cmd)
            rec.update(cls=cls,classes=classes,detail=detail,cmd=clean(cmd)[:600],cmdlen=len(cmd),desc=clean(inp.get('description','') or '')[:120])
            rec['timeout_ms']=inp.get('timeout')
            rec['bg']=bool(inp.get('run_in_background')) or bool(u.get('auto_bg'))
            if u.get('auto_bg'): rec['auto_bg']=1
            n=notif.get(uid) or notif.get(u.get('bgid',''))
            if rec['bg']:
                rec['bgid']=u.get('bgid')
                if n: rec.update(bg_end=n['t'],bg_dur=n['t']-u['t0'],bg_status=n['status'],bg_exit=n['exit'])
            # result flags
            head=rt[:600]
            if 'claude-shell-guard' in rt or re.search(r'blocked',head,re.I) and 'hook' in head.lower(): rec['blocked']=1
            m=re.search(r'Exit code (\d+)',head)
            if m: rec['exit']=int(m.group(1))
            if re.search(r'timed out',head): rec['timedout']=1
            if '<persisted-output>' in rt or 'Output too large' in rt: rec['persisted']=1
            # gate report extraction (stdout of a foreground gate or a tail of its redirect)
            if 'GATE: PASS' in rt or 'GATE: FAIL' in rt or '==> fmt' in rt or re.search(r'==> \w+ +(ok|FAILED)',rt):
                g={}
                for sm in re.finditer(r'==> (\w+)\s+(ok|FAILED\s+\d+|DID NOT RUN)[^\n(]*\((\d+)s\)',rt): g[sm.group(1)]=(sm.group(2).split()[0],int(sm.group(3)))
                gm=re.search(r'GATE: (PASS|FAIL)(?: \(step:([^)]*)\))?',rt)
                tm=re.search(r'tests\s+passed=(\d+) failed=(\d+)\s+\(over (\d+) test binaries\)',rt)
                lm=re.search(r'(?:gate: log\s+|full output: )\S*?(jigc-gate-[A-Za-z0-9]{6})\b',rt)
                fails=[]
                fm=re.search(r'failing tests:\n((?:  .*\n?)+)',rt)
                if fm: fails=[x.strip() for x in fm.group(1).strip().split('\n')][:40]
                rec['gate']=dict(steps=g,verdict=gm.group(1) if gm else None,failed_steps=(gm.group(2) or '').split() if gm else [],
                                 passed=int(tm.group(1)) if tm else None,failed=int(tm.group(2)) if tm else None,log=lm.group(1) if lm else None,failing=fails,
                                 mode='quick' if 'mode   quick' in rt else ('full' if 'mode   full' in rt else None))
            lm=re.findall(r'jigc-gate-[A-Za-z0-9]{6}\b',cmd)
            if lm: rec['cmd_gate_logs']=sorted(set(lm))
        elif u['name'] in('Read','Write','Edit'):
            fp=inp.get('file_path','') or ''
            rec.update(cls='file-'+u['name'].lower(),cmd=clean(fp)[:300])
            if u['name']=='Read': rec['limit']=inp.get('limit'); rec['offset']=inp.get('offset')
            if u['name']=='Write': rec['wlen']=len(inp.get('content','') or '')
        elif u['name'] in('Agent','Task','Workflow'):
            rec.update(cls='spawn',cmd=clean(str(inp.get('description') or inp.get('name') or inp.get('script_path') or ''))[:200],bg=bool(inp.get('run_in_background')))
            rec['subagent_type']=inp.get('subagent_type')
        else:
            rec.update(cls='tool-'+u['name'],cmd='')
            if u['name']=='StructuredOutput': rec['wlen']=len(json.dumps(inp))
        calls.append(rec)
    # ----- time partition: union of tool intervals vs gaps
    iv=sorted([(c['t0'],c['t1']) for c in calls if c.get('t1')])
    merged=[]
    for a,b in iv:
        if merged and a<=merged[-1][1]: merged[-1][1]=max(merged[-1][1],b)
        else: merged.append([a,b])
    tool_union=sum(b-a for a,b in merged)
    # idle = gap after an end_turn assistant block until the next entry
    tl=sorted(timeline,key=lambda x:x[0])
    idle=0.0; idle_gaps=[]
    for i,(tt,k,sr) in enumerate(tl[:-1]):
        if k=='a' and sr=='end_turn':
            g=tl[i+1][0]-tt
            if g>0: idle+=g; idle_gaps.append((tt,g))
    wall=(last-first) if first is not None else 0
    model=max(0.0,wall-tool_union-idle)
    tk=collections.Counter()
    for m in msgs.values():
        for k in('inp','cc','cr','out'): tk[k]+=m[k]
    peak_ctx=max([m['inp']+m['cc']+m['cr'] for m in msgs.values()],default=0)
    ag=dict(run=run,agent=agent_id,start=first,end=last,wall=wall,tool_union=tool_union,idle=idle,model=model,
            n_calls=len(calls),n_bash=sum(1 for c in calls if c['tool']=='Bash'),n_msgs=len(msgs),
            tok_in=tk['inp'],tok_cache_create=tk['cc'],tok_cache_read=tk['cr'],tok_out=tk['out'],peak_ctx=peak_ctx,
            prompt_len=prompt_len,final_text_len=final_text_len,structured_len=so_len,idle_gaps=idle_gaps[:200])
    ag.update(meta)
    return ag,calls

def main():
    agents=[]; allcalls=[]
    # workflow agents
    for wd in sorted(glob.glob(B+"/subagents/workflows/wf_*")):
        run=os.path.basename(wd)
        labels={}
        rf=os.path.join(B,"workflows",run+".json")
        runmeta={}
        if os.path.exists(rf):
            rd=json.load(open(rf))
            for p in rd.get('workflowProgress',[]):
                if p.get('type')=='workflow_agent':
                    labels[p['agentId']]=dict(label=p.get('label'),phase=p.get('phaseTitle'),agent_type=p.get('agentType'),
                        wf_tokens=p.get('tokens'),wf_tool_calls=p.get('toolCalls'),wf_dur=(p.get('durationMs') or 0)/1000,
                        wf_started=(p.get('startedAt') or 0)/1000,wf_queued=(p.get('queuedAt') or 0)/1000,wf_state=p.get('state'),attempt=p.get('attempt'))
        jp=os.path.join(wd,"journal.jsonl")
        if os.path.exists(jp):
            for l in open(jp):
                d=json.loads(l)
                if d.get('type')=='started' and d.get('agentId') not in labels:
                    labels[d['agentId']]=dict(label=d.get('label'),phase=d.get('phase'))
                if d.get('type')=='result' and d.get('agentId') in labels:
                    labels[d['agentId']]['journal_result_len']=len(json.dumps(d.get('result')))
        for f in sorted(glob.glob(wd+"/agent-*.jsonl")):
            aid=os.path.basename(f)[6:-6]
            meta=dict(labels.get(aid,{})); meta.setdefault('label','?')
            mf=f[:-6]+'.meta.json'
            if os.path.exists(mf):
                md=json.load(open(mf)); meta.setdefault('agent_type',md.get('agentType')); meta['description']=clean(md.get('description','') or '')[:120]
            ag,calls=parse(f,run,aid,meta); agents.append(ag); allcalls+=calls
    # single subagents
    for f in sorted(glob.glob(B+"/subagents/agent-*.jsonl")):
        aid=os.path.basename(f)[6:-6]
        meta={}
        mf=f[:-6]+'.meta.json'
        if os.path.exists(mf):
            md=json.load(open(mf)); meta=dict(agent_type=md.get('agentType'),label=clean(md.get('description','') or '')[:120],phase='single')
        ag,calls=parse(f,'single',aid,meta); agents.append(ag); allcalls+=calls
    # the orchestrating session itself
    mainf=B+".jsonl"
    if os.path.exists(mainf):
        ag,calls=parse(mainf,'main','main',dict(label='orchestrator',phase='main',agent_type='main'))
        agents.append(ag); allcalls+=calls
    json.dump(agents,open(os.path.join(OUT,"agents.json"),"w"))
    with open(os.path.join(OUT,"calls.jsonl"),"w") as fh:
        for c in allcalls: fh.write(json.dumps(c)+"\n")
    print("agents",len(agents),"calls",len(allcalls),"bash",sum(1 for c in allcalls if c['tool']=='Bash'))
    cc=collections.Counter(c.get('cls') for c in allcalls)
    print(cc.most_common())
    ow=collections.Counter()
    for c in allcalls:
        if c.get('cls')=='other':
            for w in c.get('detail',{}).get('other_words',[]): ow[w]+=1
    print('other first-words:',ow.most_common(60))
if __name__=="__main__": main()
