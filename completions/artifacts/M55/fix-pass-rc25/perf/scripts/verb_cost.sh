#!/usr/bin/env bash
# Debug versus release jigc, per invocation, in one fresh rig repo: N runs each of three read verbs, and the wall
# and CPU of building the rig itself (a fixed sequence of real write verbs) with each binary.
# Reads: <work-dir>/clone with BOTH target/debug/jigc and target/release/jigc built (measure_suite.sh builds the
# debug one only). Writes: each rig's stderr to <work-dir>/suite/verb-rig-<kind>.err.
# Prints: one `real/user/sys` line per measured command.
#   usage: verb_cost.sh <work-dir> [N=50]
[ $# -ge 1 ] || { echo "usage: verb_cost.sh <work-dir> [N]" >&2; exit 2; }
perf=$1; N=${2:-50}; clone="$perf/clone"
real_git=$(xcrun --find git); export PATH="$(dirname "$real_git"):$PATH" SDKROOT=$(xcrun --show-sdk-path)
loop() { local i=0; while [ $i -lt "$N" ]; do "$@" >/dev/null 2>&1; i=$((i+1)); done; }
for kind in debug release; do
  J="$clone/target/$kind/jigc"
  TIMEFORMAT="$kind: build rig committed-singletons: real %R user %U sys %S"
  time { rig=$("$clone/dev/jigc-rig" committed-singletons --binary "$J" 2>"$perf/suite/verb-rig-$kind.err"); }
  eval "$rig"; [ -n "${REPO:-}" ] || { echo "no REPO for $kind"; continue; }
  cd "$REPO" || continue
  for verb in "--version" "describe" "validate" "doc list"; do
    TIMEFORMAT="$kind: jigc $verb x$N: real %R user %U sys %S"
    # shellcheck disable=SC2086
    time loop "$J" $verb
  done
  cd "$perf" || exit 0
done
