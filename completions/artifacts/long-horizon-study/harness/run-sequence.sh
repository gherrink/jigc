#!/usr/bin/env bash
# run-sequence.sh — ONE long-horizon sequence rep: a fresh cold agent per edit,
# against an EVOLVING twin. Generalizes the pilot's single-shot run-rep.sh.
#
#   run-sequence.sh <arm> <model> <out-dir>
#     arm    : A | C40 | C160 | C550 | P   (template under $ROOT/templates/<arm>)
#     model  : claude-sonnet-4-6 | claude-opus-4-8
#     out-dir: receives repo/ (the evolving twin) + edit<N>/ per-edit artifacts
#
# Per edit N (1..8): a FRESH --rm container with the evolving repo bind-mounted rw at
# /work; the agent edits + `git commit`s (the pre-commit hook fires on arm A); we
# capture the tool-call stream, record whether a commit landed + any blocked attempt,
# MEASURE the committed HEAD (the deliverable, via measure.py), then reset the tree to
# HEAD so the next edit starts from the last landed commit.
#
# Gotchas baked in (pilot RUNBOOK): USER node, --permission-mode bypassPermissions,
# prompt via -e PILOT_PROMPT (never inlined — backticks).
set -u
ROOT="${ROOT:-$HOME/lh-study}"; H="$ROOT/harness"
IMAGE="${IMAGE:-lh-toolchain}"
CLAUDE_BIN="${CLAUDE_BIN:-$(readlink -f ~/.local/bin/claude)}"
CREDS="${CREDS:-$HOME/.claude/.credentials.json}"
JIGC="${JIGC:-$HOME/.local/bin/jigc}"; DOCCODE="${DOCCODE:-$HOME/.local/bin/doc-code}"
TIMEOUT="${TIMEOUT:-900}"; NEDITS="${NEDITS:-8}"

[ "$#" -eq 3 ] || { sed -n '2,14p' "$0"; exit 2; }
ARM="$1"; MODEL="$2"; OUT="$3"; TPL="$ROOT/templates/$ARM"
[ -d "$TPL" ] || { echo "no template $TPL" >&2; exit 2; }
for f in "$CLAUDE_BIN" "$CREDS"; do [ -e "$f" ] || { echo "missing $f" >&2; exit 2; }; done

mkdir -p "$OUT"; OUT=$(cd "$OUT" && pwd)   # docker bind-mounts require absolute paths
REPO="$OUT/repo"
rm -rf "$REPO"; cp -a "$TPL" "$REPO"
git -C "$REPO" config user.email agent@study.local
git -C "$REPO" config user.name  study-agent

EXTRA=()
[ "$ARM" = "A" ] && EXTRA=(-v "$JIGC":/usr/local/bin/jigc:ro -v "$DOCCODE":/usr/local/bin/doc-code:ro)

echo "[$(date +%H:%M:%S)] arm=$ARM model=$MODEL → $OUT ($NEDITS edits)"
for n in $(seq 1 "$NEDITS"); do
  ED="$OUT/edit$n"; mkdir -p "$ED"
  PROMPT="$(cat "$H/prompts/edit$n.txt")"
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
  # commit accounting BEFORE reset
  git -C "$REPO" log --oneline "$BEFORE..$AFTER" > "$ED/commits.txt" 2>/dev/null || true
  NCOMMITS=$(git -C "$REPO" rev-list --count "$BEFORE..$AFTER" 2>/dev/null); NCOMMITS=${NCOMMITS:-0}
  git -C "$REPO" diff HEAD > "$ED/leftover.diff" 2>/dev/null || true   # uncommitted (e.g. blocked)
  LEFTOVER=$([ -s "$ED/leftover.diff" ] && echo 1 || echo 0)
  # discard any uncommitted leftover; next edit starts from the landed HEAD
  git -C "$REPO" reset --hard -q HEAD; git -C "$REPO" clean -fdq

  # measure the committed deliverable (working tree == HEAD now)
  python3 "$H/measure.py" --repo "$REPO" --seq "$H/sequence.json" --edit "$n" > "$ED/measure.json" 2>"$ED/measure.err" || true
  # behavioral analysis of the transcript (engage/select/finalize/turns/cost)
  python3 "$H/analyze.py" "$ED/transcript.jsonl" > "$ED/analyze.json" 2>/dev/null || echo '{}' > "$ED/analyze.json"

  # per-edit one-line record
  python3 - "$ED" "$n" "$NCOMMITS" "$LEFTOVER" "$ARM" <<'PY' > "$ED/record.json"
import json,sys,re
ed,n,ncommits,leftover,arm=sys.argv[1:6]
m=json.load(open(f"{ed}/measure.json")) if __import__("os").path.exists(f"{ed}/measure.json") else {}
a=json.load(open(f"{ed}/analyze.json")) if __import__("os").path.exists(f"{ed}/analyze.json") else {}
try: trans=open(f"{ed}/transcript.jsonl").read()
except Exception: trans=""
blocked = ("COMMIT BLOCKED" in trans) or ("doc-code" in trans and arm=="A")
noverify = bool(re.search(r"--no-verify|--no\s*verify", trans))
rec=dict(edit=int(n), arm=arm,
         n_commits=int(ncommits), leftover_uncommitted=bool(int(leftover)),
         drift=m.get("drift"), n_dangling=m.get("n_dangling"), n_stale=m.get("n_stale_names"),
         control_ok=m.get("control_ok"),
         engaged=a.get("engaged"), finalized=a.get("finalized"),
         jigc_verbs=a.get("jigc_verbs"), turns=a.get("num_turns"), cost=a.get("cost_usd"),
         hook_blocked_seen=blocked, no_verify_seen=noverify)
print(json.dumps(rec))
PY
  python3 - "$ED/record.json" "$n" <<'PY'
import json,sys
try: d=json.load(open(sys.argv[1]))
except Exception as e: print(f"  edit {sys.argv[2]}: (record error: {e})"); sys.exit()
print(f"  edit {d.get('edit')}: commits={d.get('n_commits')} drift={d.get('drift')} "
      f"dangling={d.get('n_dangling')} stale={d.get('n_stale')} engaged={d.get('engaged')} "
      f"blocked={d.get('hook_blocked_seen')} cost={d.get('cost')}")
PY
done

# assemble the sequence timeline
python3 - "$OUT" "$NEDITS" "$ARM" "$MODEL" <<'PY' > "$OUT/sequence.json"
import json,sys,os
out,nedits,arm,model=sys.argv[1],int(sys.argv[2]),sys.argv[3],sys.argv[4]
edits=[]
for n in range(1,nedits+1):
    p=f"{out}/edit{n}/record.json"
    edits.append(json.load(open(p)) if os.path.exists(p) else {"edit":n})
final=edits[-1] if edits else {}
summary=dict(arm=arm, model=model, n_edits=nedits,
             final_drift=final.get("drift"),
             final_dangling=final.get("n_dangling"),
             final_stale=final.get("n_stale"),
             commits_total=sum((e.get("n_commits") or 0) for e in edits),
             edits_with_drift=sum(1 for e in edits if e.get("drift")),
             any_engaged=any(e.get("engaged") for e in edits),
             any_blocked=any(e.get("hook_blocked_seen") for e in edits),
             any_no_verify=any(e.get("no_verify_seen") for e in edits),
             total_cost=sum((e.get("cost") or 0) for e in edits),
             timeline=edits)
print(json.dumps(summary,indent=2))
PY
echo "[$(date +%H:%M:%S)] done $ARM/$MODEL → $OUT/sequence.json"
