#!/usr/bin/env bash
# run-matrix-refint.sh — the full pre-registered M35 rename-cost matrix, bounded-concurrency.
#   Sonnet: arms jigc static plain  x REPS_S reps   (the full 3-arm set)
#   Opus:   arms jigc plain         x REPS_O reps   (the cost extremes — the headline
#                                                    jigc-cheaper-than-plain comparison)
# Each (arm,model,rep) is one independent 8-edit rename sequence (own repo, fresh containers).
# Sequences run concurrently up to CONC; edits within a sequence stay serial (evolving twin).
#
# AUTH PROTOCOL (the OAuth token expires mid-run; see the study RUNBOOK):
#   - verify auth LIVE immediately before launch (the cheap haiku probe);
#   - prefer launching the OPUS batch separately, in small groups, right after a fresh
#     re-login — set MODELS=opus and CONC=1 for it. An expiry then truncates one sequence,
#     not several. Always integrity-check after (authentication_failed / turns==1,cost==0).
set -u
STUDY="${STUDY:-$(cd "$(dirname "$0")/.." && pwd)}"
H="${H:-$STUDY/harness}"
RUNS="${RUNS:-$HOME/lh-study/runs/refint-matrix-m35}"
CONC="${CONC:-3}"
REPS_S="${REPS_S:-3}"; REPS_O="${REPS_O:-2}"
MODELS="${MODELS:-both}"   # both | sonnet | opus  (split the paid run for auth safety)
# Env-overridable (amendment 2026-07-02: the supplementary Sonnet-5 cell runs as
# MODELS=sonnet SONNET=claude-sonnet-5 — the pre-registered verdict still rides the pinned ids).
SONNET="${SONNET:-claude-sonnet-4-6}"; OPUS="${OPUS:-claude-opus-4-8}"
mkdir -p "$RUNS"

jobs=()
if [ "$MODELS" != "opus" ]; then
  for r in $(seq 1 "$REPS_S"); do for arm in jigc static plain; do jobs+=("$arm $SONNET $r"); done; done
fi
if [ "$MODELS" != "sonnet" ]; then
  for r in $(seq 1 "$REPS_O"); do for arm in jigc plain;          do jobs+=("$arm $OPUS  $r"); done; done
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
