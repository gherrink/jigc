#!/usr/bin/env bash
# run-matrix.sh — the full pre-registered matrix, bounded-concurrency.
#   Sonnet: arms A C40 C160 C550 P  x REPS_S reps
#   Opus:   arms A C550             x REPS_O reps
# Each (arm,model,rep) is one independent 8-edit sequence (own repo, fresh containers).
# Sequences run concurrently up to CONC; edits within a sequence stay serial (evolving twin).
set -u
ROOT="${ROOT:-$HOME/lh-study}"; H="$ROOT/harness"
RUNS="${RUNS:-$ROOT/runs/matrix}"
CONC="${CONC:-3}"
REPS_S="${REPS_S:-3}"; REPS_O="${REPS_O:-2}"
SONNET=claude-sonnet-4-6; OPUS=claude-opus-4-8
mkdir -p "$RUNS"

jobs=()
for r in $(seq 1 "$REPS_S"); do for arm in A C40 C160 C550 P; do jobs+=("$arm $SONNET $r"); done; done
for r in $(seq 1 "$REPS_O"); do for arm in A C550;            do jobs+=("$arm $OPUS  $r"); done; done

echo "matrix: ${#jobs[@]} sequences x 8 edits, concurrency $CONC → $RUNS"
printf '%s\n' "${jobs[@]}" | nl

run_one() {
  read -r arm model rep <<< "$1"
  local tag="${arm}-${model##*-}-rep${rep}"
  "$H/run-sequence.sh" "$arm" "$model" "$RUNS/$tag" > "$RUNS/$tag.log" 2>&1
  echo "[$(date +%H:%M:%S)] FINISHED $tag"
}
export -f run_one; export H RUNS

# bounded concurrency
active=0
for j in "${jobs[@]}"; do
  run_one "$j" &
  active=$((active+1))
  if [ "$active" -ge "$CONC" ]; then wait -n 2>/dev/null || wait; active=$((active-1)); fi
done
wait
echo "[$(date +%H:%M:%S)] MATRIX COMPLETE → $RUNS"
