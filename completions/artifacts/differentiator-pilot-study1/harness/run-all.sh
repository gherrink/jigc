#!/usr/bin/env bash
# Drive the full 4-arm × 2-model × 2-task matrix with a small concurrency cap.
set -u
cd ~/diff-pilot
MAX=2
declare -a CELLS
for task in task1 task2; do
  for arm in plain static gsd jigc; do
    for model in claude-sonnet-4-6 claude-opus-4-8; do
      CELLS+=("$arm|$model|$task")
    done
  done
done
echo "matrix: ${#CELLS[@]} cells, concurrency $MAX"
running=0
for cell in "${CELLS[@]}"; do
  IFS='|' read -r arm model task <<< "$cell"
  ./run-arm.sh "$arm" "$model" "$task" "prompts/$task.txt" >> "runs/$task/$arm-$model.runlog" 2>&1 &
  running=$((running+1))
  if (( running >= MAX )); then wait -n; running=$((running-1)); fi
done
wait
echo "===== ALL ${#CELLS[@]} RUNS COMPLETE ====="
