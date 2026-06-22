#!/usr/bin/env bash
# run-rep.sh <arm> <model> <task> <prompt-file> <rep>
# One replication run, isolated container, output to runs-rep/<task>/<arm>-rep<rep>.
set -u
ARM="$1"; MODEL="$2"; TASK="$3"; PROMPT_FILE="$4"; REP="$5"
case "$ARM" in
  plain)    IMG=pilot-deps ;;
  static)   IMG=pilot-static ;;
  jigc)     IMG=pilot-jigc ;;
  jigcgate) IMG=pilot-jigcgate ;;
  *) echo "unknown arm: $ARM" >&2; exit 2 ;;
esac
OUT=~/diff-pilot/runs-rep/$TASK/$ARM-rep$REP
mkdir -p "$OUT"
PROMPT="$(cat "$PROMPT_FILE")"
CLAUDE_BIN=~/.local/share/claude/versions/2.1.185
echo "[$(date +%H:%M:%S)] arm=$ARM task=$TASK rep=$REP"
timeout 1200 docker run --rm \
  -v "$CLAUDE_BIN":/usr/local/bin/claude:ro \
  -v ~/.claude/.credentials.json:/home/node/.claude/.credentials.json:ro \
  -v "$OUT":/out \
  -e PILOT_PROMPT="$PROMPT" \
  "$IMG" bash -c '
    set -e
    BASE=$(git rev-parse HEAD)
    claude -p "$PILOT_PROMPT" --model '"$MODEL"' --output-format json \
      --permission-mode bypassPermissions > /out/transcript.json 2> /out/stderr.log
    git add -A 2>/dev/null || true
    git diff --cached "$BASE" > /out/changes.diff 2>/dev/null || true
    cp packages/core/README.md /out/README.after.md 2>/dev/null || true
    cp docs/architecture/core-public-api.md /out/arch-doc.after.md 2>/dev/null || true
    command -v jigc >/dev/null 2>&1 && jigc validate > /out/jigc-validate.after.txt 2>&1 || true
    echo done > /out/meta.txt
  ' 2>>"$OUT/stderr.log"
