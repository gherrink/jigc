#!/usr/bin/env bash
# run-matrix-refint.sh — the full pre-registered cross-doc matrix, bounded-concurrency.
#   Sonnet: arms A C40 C160 C550 P  x REPS_S reps
#   Opus:   arms A C550             x REPS_O reps   (the extremes, the cost-tax arm)
# Each (arm,model,rep) is one independent 8-edit sequence (own repo, fresh containers).
# Sequences run concurrently up to CONC; edits within a sequence stay serial (evolving twin).
#
# AUTH PROTOCOL (the OAuth token expires mid-run; see the study HANDOVER):
#   - verify auth LIVE immediately before launch (the cheap haiku probe);
#   - prefer launching the OPUS batch separately, in small groups, right after a fresh
#     re-login — set MODELS=opus and CONC=1 for it. An expiry then truncates one sequence,
#     not five. Always integrity-check after (authentication_failed / turns==1,cost==0).
set -u
STUDY="${STUDY:-$(cd "$(dirname "$0")/.." && pwd)}"
H="${H:-$STUDY/harness}"
RUNS="${RUNS:-$HOME/lh-study/runs/refint-matrix}"
CONC="${CONC:-3}"
REPS_S="${REPS_S:-3}"; REPS_O="${REPS_O:-2}"
MODELS="${MODELS:-both}"   # both | sonnet | opus  (split the paid run for auth safety)
SONNET=claude-sonnet-4-6; OPUS=claude-opus-4-8
mkdir -p "$RUNS"

jobs=()
if [ "$MODELS" != "opus" ]; then
  for r in $(seq 1 "$REPS_S"); do for arm in A C40 C160 C550 P; do jobs+=("$arm $SONNET $r"); done; done
fi
if [ "$MODELS" != "sonnet" ]; then
  for r in $(seq 1 "$REPS_O"); do for arm in A C550;            do jobs+=("$arm $OPUS  $r"); done; done
fi

echo "matrix ($MODELS): ${#jobs[@]} sequences x ${NEDITS:-8} edits, concurrency $CONC → $RUNS"
printf '%s\n' "${jobs[@]}" | nl

run_one() {
  read -r arm model rep <<< "$1"
  local tag="${arm}-${model##*-}-rep${rep}"
  STUDY="$STUDY" H="$H" "$H/run-sequence-refint.sh" "$arm" "$model" "$RUNS/$tag" > "$RUNS/$tag.log" 2>&1
  echo "[$(date +%H:%M:%S)] FINISHED $tag"
}
export -f run_one; export H RUNS STUDY

active=0
for j in "${jobs[@]}"; do
  run_one "$j" &
  active=$((active+1))
  if [ "$active" -ge "$CONC" ]; then wait -n 2>/dev/null || wait; active=$((active-1)); fi
done
wait
echo "[$(date +%H:%M:%S)] MATRIX COMPLETE → $RUNS"
