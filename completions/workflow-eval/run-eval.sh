#!/usr/bin/env bash
# run-eval.sh — one isolated agent-in-the-loop workflow-eval run.
#
# Generalizes the pilot's run-rep.sh (completions/artifacts/differentiator-pilot-study1/
# harness/run-rep.sh): no hardcoded arm→image map, and it captures the TOOL-CALL STREAM
# (--output-format stream-json --verbose) so the behavioral signals (engage / select /
# complete) are recoverable — the pilot's --output-format json kept only the final result,
# which is why its select/engage/complete numbers came from an ad-hoc stream-json probe.
#
# Usage:
#   run-eval.sh <image> <model> <prompt-file> <out-dir>
#
# Env (defaults match the pilot RUNBOOK — override per host):
#   CLAUDE_BIN   path to the claude CLI binary mounted into the container
#                (default: newest under ~/.local/share/claude/versions/)
#   CREDS        OAuth creds mounted read-only (default: ~/.claude/.credentials.json)
#   TIMEOUT      per-run wall-clock cap in seconds (default: 1200)
#
# Captures into <out-dir>: transcript.jsonl (the stream), stderr.log, changes.diff
# (staged vs base), jigc-validate.after.txt (the Phase-2 floor's store sweep — the
# outcome oracle), commits.txt. Analyze with analyze.py; aggregate with eval.py.
#
# The container gotchas the pilot RUNBOOK documents are baked in: USER node (root
# refuses bypassPermissions), --permission-mode bypassPermissions (else every jigc /
# Edit is permission-gated and the run is garbage), and the prompt passed via
# -e PILOT_PROMPT (never inlined — task prompts contain backticks the container shell
# would command-substitute).
set -u

if [ "$#" -ne 4 ]; then
  sed -n '2,30p' "$0"; exit 2
fi
IMAGE="$1"; MODEL="$2"; PROMPT_FILE="$3"; OUT="$4"

CLAUDE_BIN="${CLAUDE_BIN:-$(ls -d ~/.local/share/claude/versions/* 2>/dev/null | sort -V | tail -1)}"
CREDS="${CREDS:-$HOME/.claude/.credentials.json}"
TIMEOUT="${TIMEOUT:-1200}"

if [ ! -e "$CLAUDE_BIN" ]; then echo "no claude binary at CLAUDE_BIN=$CLAUDE_BIN" >&2; exit 2; fi
if [ ! -e "$CREDS" ]; then echo "no creds at CREDS=$CREDS" >&2; exit 2; fi
if [ ! -e "$PROMPT_FILE" ]; then echo "no prompt file at $PROMPT_FILE" >&2; exit 2; fi

mkdir -p "$OUT"
PROMPT="$(cat "$PROMPT_FILE")"
echo "[$(date +%H:%M:%S)] image=$IMAGE model=$MODEL → $OUT"

timeout "$TIMEOUT" docker run --rm \
  -v "$CLAUDE_BIN":/usr/local/bin/claude:ro \
  -v "$CREDS":/home/node/.claude/.credentials.json:ro \
  -v "$OUT":/out \
  -e PILOT_PROMPT="$PROMPT" \
  -e MODEL="$MODEL" \
  "$IMAGE" bash -c '
    set -e
    BASE=$(git rev-parse HEAD)
    claude -p "$PILOT_PROMPT" --model "$MODEL" \
      --output-format stream-json --verbose \
      --permission-mode bypassPermissions \
      > /out/transcript.jsonl 2> /out/stderr.log
    git add -A 2>/dev/null || true
    git diff --cached "$BASE" > /out/changes.diff 2>/dev/null || true
    git log --oneline "$BASE"..HEAD > /out/commits.txt 2>/dev/null || true
    command -v jigc >/dev/null 2>&1 && jigc validate > /out/jigc-validate.after.txt 2>&1 || true
    echo done > /out/meta.txt
  ' 2>>"$OUT/stderr.log"

if [ -s "$OUT/transcript.jsonl" ]; then
  echo "  captured $(wc -l < "$OUT/transcript.jsonl") stream events"
else
  echo "  WARNING: empty transcript — check $OUT/stderr.log (missing flag? auth?)" >&2
fi
