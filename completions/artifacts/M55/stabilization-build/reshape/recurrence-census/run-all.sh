#!/bin/sh
# usage: CENSUS_REPO=<repository root> sh run-all.sh   — rebuilds every table of the census; writes *.json and *.out here
set -e
here=$(dirname "$0")
: "${CENSUS_REPO:?set CENSUS_REPO to the repository root}"
# several tables print sets and take the first of two codes a row carries; with the seed fixed a re-run prints the committed *.out byte for byte
PYTHONHASHSEED=0; export PYTHONHASHSEED
python3 "$here/items.py"   > "$here/items.out"
python3 "$here/bullets.py" > "$here/bullets.out"
for q in keys q1_coverage q2_recurrence q3_predict q3b_extras q3c_reverse q4_duplicates q5_nocode q6_confound; do
  python3 "$here/$q.py" > "$here/$q.out"
done
