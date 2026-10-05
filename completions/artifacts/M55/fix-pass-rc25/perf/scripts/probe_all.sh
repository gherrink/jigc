#!/usr/bin/env bash
# Run spawn_probe.sh over a fixed sample of tests, one after another; append the summaries to
# <work-dir>/suite/probe-summary.txt.
# Reads: the sample file — one "<group-binary> <suite::test>" per line; by default probe_sample.txt beside this
# script, the ten tests the report's sample used. Prints: nothing (the summaries go to the file).
#   usage: probe_all.sh <work-dir> [<sample-file>]
[ $# -ge 1 ] || { echo "usage: probe_all.sh <work-dir> [<sample-file>]" >&2; exit 2; }
here=$(cd "$(dirname "$0")" && pwd) || exit 2
perf=$1; sample=${2:-$here/probe_sample.txt}; out="$perf/suite/probe-summary.txt"
printf 'start %s load=%s\n' "$(date -u +%H:%M:%S)" "$(sysctl -n vm.loadavg)" >>"$out"
while read -r grp test; do
  [ -n "$grp" ] || continue
  "$here/spawn_probe.sh" "$perf" "$grp" "$test" >>"$out" 2>&1
done <"$sample"
printf 'end %s load=%s\n' "$(date -u +%H:%M:%S)" "$(sysctl -n vm.loadavg)" >>"$out"
