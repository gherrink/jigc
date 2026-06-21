#!/usr/bin/env bash
# run-arm.sh <arm> <model> <task-id> <prompt-file>
# Runs ONE (arm × model × task) in a fresh, isolated container and captures
# the transcript JSON + the complete diff the agent produced. Read-only mounts;
# nothing touches the host repos. The container exits and is removed (--rm).
set -u

ARM="$1"; MODEL="$2"; TASK="$3"; PROMPT_FILE="$4"
case "$ARM" in
  jigc)   IMG=pilot-jigc ;;
  gsd)    IMG=pilot-gsd ;;
  static) IMG=pilot-static ;;
  plain)  IMG=pilot-deps ;;
  *) echo "unknown arm: $ARM" >&2; exit 2 ;;
esac

OUT=~/diff-pilot/runs/$TASK/$ARM-$MODEL
mkdir -p "$OUT"
PROMPT="$(cat "$PROMPT_FILE")"
CLAUDE_BIN=~/.local/share/claude/versions/2.1.185

echo "[$(date +%H:%M:%S)] RUN arm=$ARM model=$MODEL task=$TASK -> $OUT"

timeout 1200 docker run --rm \
  -v "$CLAUDE_BIN":/usr/local/bin/claude:ro \
  -v ~/.claude/.credentials.json:/home/node/.claude/.credentials.json:ro \
  -v "$OUT":/out \
  -e PILOT_PROMPT="$PROMPT" \
  "$IMG" bash -c '
    set -e
    BASE=$(git rev-parse HEAD)
    echo "$BASE" > /out/base.sha
    # Fire the byte-identical prompt headless, model pinned, autonomous.
    claude -p "$PILOT_PROMPT" \
      --model '"$MODEL"' \
      --output-format json \
      --permission-mode bypassPermissions \
      > /out/transcript.json 2> /out/stderr.log
    echo "claude_exit=$?" > /out/meta.txt
    # Capture the complete change set vs the pre-run state (tracked + untracked).
    git add -A 2>/dev/null || true
    git diff --cached "$BASE" > /out/changes.diff 2>/dev/null || true
    git -c core.pager=cat log --oneline "$BASE"..HEAD > /out/commits.txt 2>/dev/null || true
    # Snapshot the two doc surfaces + the renamed code for the judge.
    cp packages/core/README.md /out/README.after.md 2>/dev/null || true
    cp docs/architecture/core-public-api.md /out/arch-doc.after.md 2>/dev/null || true
    # jigc arm: record whether the managed store still validates (drift left behind?).
    if command -v jigc >/dev/null 2>&1; then
      jigc validate > /out/jigc-validate.after.txt 2>&1 || true
    fi
  ' 2>>"$OUT/stderr.log"

echo "[$(date +%H:%M:%S)] DONE arm=$ARM model=$MODEL task=$TASK (host exit $?)"
