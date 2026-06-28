#!/usr/bin/env bash
# run-sequence-refint.sh — ONE M35 rename-cost sequence rep: a fresh cold agent per edit,
# against an EVOLVING decision-record twin. The M35 analog of the cross-doc study's
# run-sequence (same isolation, same per-edit accounting), with the arm set collapsed to the
# three rename-study arms and the jigc arm able to PERFORM the rename.
#
#   run-sequence-refint.sh <arm> <model> <out-dir>
#     arm    : jigc | static | plain   (template under $TPLROOT/templates/<arm>)
#     model  : claude-sonnet-4-6 | claude-opus-4-8
#
# Per edit N (1..8 — every edit is a rename, so the per-rename cost denominator is well
# powered): a FRESH --rm container with the evolving repo bind-mounted rw at /work; the agent
# performs the rename + `git commit`s (the jigc arm's Inc-2 backstop hook blocks a bare
# `git mv`, so a hand-edit pays block->recovery — the verb-engagement confound). We capture
# the tool-call stream, record whether a commit landed + any blocked attempt, MEASURE the
# committed HEAD via measure-refint.py (the deliverable), then reset the tree to HEAD so the
# next edit starts from the last landed commit.
#
# Gotchas baked in (cross-doc RUNBOOK): USER node, --permission-mode bypassPermissions,
# prompt via -e PILOT_PROMPT (never inlined). The jigc arm mounts jigc at the container path
# the setup-written hook + adapter expect (/usr/local/bin).
set -u
STUDY="${STUDY:-$(cd "$(dirname "$0")/.." && pwd)}"
H="${H:-$STUDY/harness}"
PROMPTS="${PROMPTS:-$STUDY/prompts}"
ANALYZE="${ANALYZE:-$HOME/lh-study/harness/analyze.py}"   # internalized into $H by Inc-3 T3
TPLROOT="${TPLROOT:-$HOME/lh-study}"
IMAGE="${IMAGE:-lh-toolchain}"
CLAUDE_BIN="${CLAUDE_BIN:-$(readlink -f ~/.local/bin/claude)}"
CREDS="${CREDS:-$HOME/.claude/.credentials.json}"
JIGC="${JIGC:-$(readlink -f ~/.local/bin/jigc)}"
TIMEOUT="${TIMEOUT:-900}"; NEDITS="${NEDITS:-8}"
CONTROL_EDGE="adr:stateless-jwt-sessions"   # the never-edited control target slug

[ "$#" -eq 3 ] || { sed -n '2,13p' "$0"; exit 2; }
ARM="$1"; MODEL="$2"; OUT="$3"; TPL="$TPLROOT/templates/$ARM"
case "$ARM" in jigc|static|plain) ;; *) echo "unknown arm '$ARM' (jigc|static|plain)" >&2; exit 2;; esac
[ -d "$TPL" ] || { echo "no template $TPL (run build-templates-refint.sh)" >&2; exit 2; }
for f in "$CLAUDE_BIN" "$CREDS"; do [ -e "$f" ] || { echo "missing $f" >&2; exit 2; }; done

mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd)
REPO="$OUT/repo"; rm -rf "$REPO"; cp -a "$TPL" "$REPO"
git -C "$REPO" config user.email agent@study.local
git -C "$REPO" config user.name  study-agent

# Only the jigc arm needs the binary mounted (for `jigc rename` + the backstop hook). The
# static/plain arms are tool-free by construction — the omitting case runs inert.
EXTRA=()
[ "$ARM" = "jigc" ] && EXTRA=(-v "$JIGC":/usr/local/bin/jigc:ro)

echo "[$(date +%H:%M:%S)] arm=$ARM model=$MODEL → $OUT ($NEDITS edits)"
for n in $(seq 1 "$NEDITS"); do
  ED="$OUT/edit$n"; mkdir -p "$ED"
  PROMPT="$(cat "$PROMPTS/edit-$n.txt")"
  BEFORE=$(git -C "$REPO" rev-parse HEAD)

  timeout "$TIMEOUT" docker run --rm \
    -v "$CLAUDE_BIN":/usr/local/bin/claude:ro \
    -v "$CREDS":/home/node/.claude/.credentials.json:ro \
    "${EXTRA[@]}" \
    -v "$REPO":/work -w /work -v "$ED":/out \
    -e PILOT_PROMPT="$PROMPT" -e MODEL="$MODEL" \
    "$IMAGE" bash -c '
      git config --global --add safe.directory /work 2>/dev/null
      claude -p "$PILOT_PROMPT" --model "$MODEL" \
        --output-format stream-json --verbose \
        --permission-mode bypassPermissions \
        > /out/transcript.jsonl 2> /out/stderr.log
    ' </dev/null >>"$ED/docker.log" 2>&1

  AFTER=$(git -C "$REPO" rev-parse HEAD)
  git -C "$REPO" log --oneline "$BEFORE..$AFTER" > "$ED/commits.txt" 2>/dev/null || true
  NCOMMITS=$(git -C "$REPO" rev-list --count "$BEFORE..$AFTER" 2>/dev/null); NCOMMITS=${NCOMMITS:-0}
  git -C "$REPO" diff HEAD > "$ED/leftover.diff" 2>/dev/null || true
  LEFTOVER=$([ -s "$ED/leftover.diff" ] && echo 1 || echo 0)
  git -C "$REPO" reset --hard -q HEAD; git -C "$REPO" clean -fdq

  # measure the committed deliverable (working tree == HEAD now). On the jigc arm the .jigc
  # lets measure-refint cross-check path (b); on static/plain it walks path (a) only.
  NOJIGC=""; [ "$ARM" = "jigc" ] || NOJIGC="--no-jigc"
  python3 "$H/measure-refint.py" --repo "$REPO" --jigc "$JIGC" $NOJIGC > "$ED/measure.json" 2>"$ED/measure.err" || true
  python3 "$ANALYZE" "$ED/transcript.jsonl" > "$ED/analyze.json" 2>/dev/null || echo '{}' > "$ED/analyze.json"

  python3 - "$ED" "$n" "$NCOMMITS" "$LEFTOVER" "$ARM" "$CONTROL_EDGE" <<'PY' > "$ED/record.json"
import json,sys,re,os
ed,n,ncommits,leftover,arm,control=sys.argv[1:7]
def load(p):
    try: return json.load(open(p))
    except Exception: return {}
m=load(f"{ed}/measure.json"); a=load(f"{ed}/analyze.json")
try: trans=open(f"{ed}/transcript.jsonl").read()
except Exception: trans=""
ndang=m.get("n_dangling")
dangling=m.get("dangling") or []
control_ok=(control not in " ".join(dangling)) if ndang is not None else None
blocked = ("COMMIT BLOCKED" in trans) or ("commit blocked" in trans) or ("out-of-band managed-doc rename" in trans)
noverify = bool(re.search(r"--no-verify|--no\s*verify", trans))
rec=dict(edit=int(n), arm=arm,
         n_commits=int(ncommits), leftover_uncommitted=bool(int(leftover)),
         n_dangling=ndang, dangling=dangling, drift=(bool(ndang) if ndang is not None else None),
         oracle_agrees=m.get("oracle_agrees"), edges_total=m.get("edges_total"),
         control_ok=control_ok,
         engaged=a.get("engaged"), finalized=a.get("finalized"),
         rename_engaged=a.get("rename_engaged"),
         jigc_verbs=a.get("jigc_verbs"), turns=a.get("num_turns"), cost=a.get("cost_usd"),
         hook_blocked_seen=blocked, no_verify_seen=noverify)
print(json.dumps(rec))
PY
  python3 - "$ED/record.json" "$n" <<'PY'
import json,sys
try: d=json.load(open(sys.argv[1]))
except Exception as e: print(f"  edit {sys.argv[2]}: (record error: {e})"); sys.exit()
print(f"  edit {d.get('edit')}: commits={d.get('n_commits')} dangling={d.get('n_dangling')} "
      f"drift={d.get('drift')} agrees={d.get('oracle_agrees')} control_ok={d.get('control_ok')} "
      f"rename_engaged={d.get('rename_engaged')} blocked={d.get('hook_blocked_seen')} cost={d.get('cost')}")
PY
done

python3 - "$OUT" "$NEDITS" "$ARM" "$MODEL" <<'PY' > "$OUT/sequence.json"
import json,sys,os
out,nedits,arm,model=sys.argv[1],int(sys.argv[2]),sys.argv[3],sys.argv[4]
edits=[]
for n in range(1,nedits+1):
    p=f"{out}/edit{n}/record.json"
    edits.append(json.load(open(p)) if os.path.exists(p) else {"edit":n})
final=edits[-1] if edits else {}
summary=dict(arm=arm, model=model, n_edits=nedits,
             final_dangling=final.get("n_dangling"),
             commits_total=sum((e.get("n_commits") or 0) for e in edits),
             edits_with_drift=sum(1 for e in edits if e.get("drift")),
             control_violations=sum(1 for e in edits if e.get("control_ok") is False),
             oracle_disagreements=sum(1 for e in edits if e.get("oracle_agrees") is False),
             any_engaged=any(e.get("engaged") for e in edits),
             renames_via_verb=sum(1 for e in edits if e.get("rename_engaged") is True),
             any_blocked=any(e.get("hook_blocked_seen") for e in edits),
             any_no_verify=any(e.get("no_verify_seen") for e in edits),
             total_cost=sum((e.get("cost") or 0) for e in edits),
             dangling_curve=[e.get("n_dangling") for e in edits],
             timeline=edits)
print(json.dumps(summary,indent=2))
PY
echo "[$(date +%H:%M:%S)] done $ARM/$MODEL → $OUT/sequence.json"
