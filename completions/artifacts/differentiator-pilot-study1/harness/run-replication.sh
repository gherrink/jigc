#!/usr/bin/env bash
# Replication: 4 arms × Sonnet × 2 tasks × N reps, concurrency-capped.
set -u
cd ~/diff-pilot
MODEL=claude-sonnet-4-6
REPS=${1:-8}
MAX=3
declare -a CELLS
for task in task1 task2; do
  for arm in plain static jigc jigcgate; do
    for rep in $(seq 1 "$REPS"); do
      CELLS+=("$arm|$task|$rep")
    done
  done
done
echo "replication: ${#CELLS[@]} runs (4 arms × 2 tasks × $REPS reps), concurrency $MAX"
running=0
for cell in "${CELLS[@]}"; do
  IFS='|' read -r arm task rep <<< "$cell"
  ./run-rep.sh "$arm" "$MODEL" "$task" "prompts/$task.txt" "$rep" \
    >> "runs-rep/$task-$arm-rep$rep.runlog" 2>&1 &
  running=$((running+1))
  if (( running >= MAX )); then wait -n; running=$((running-1)); fi
done
wait
echo "===== REPLICATION COMPLETE: ${#CELLS[@]} runs ====="
