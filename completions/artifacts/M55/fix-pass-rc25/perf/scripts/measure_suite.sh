#!/usr/bin/env bash
# Measure per-test timings of the whole suite WITHOUT touching the working repository:
# clone the committed HEAD into <work-dir>/clone (its own target/), build, and run the gate's own test command
# with nextest's per-test status lines on. Mirrors dev/gate's git/SDKROOT setup. No teardown: nothing is removed.
# Reads: the source repository (git clone only). Writes: <work-dir>/clone and the logs under <work-dir>/suite
# (timeline.txt, head.txt, build-debug.log, test-compile.log, nextest-full.log, load-during-run.txt).
# Prints: nothing. <work-dir> is a scratch directory outside the repository.
#   usage: measure_suite.sh <source-repo> <work-dir>
set -u
[ $# -ge 2 ] || { echo "usage: measure_suite.sh <source-repo> <work-dir>" >&2; exit 2; }
src=$1; perf=$2
clone="$perf/clone"; out="$perf/suite"
mkdir -p "$out" || exit 2
tl="$out/timeline.txt"
stamp() { printf '%s %s load=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" "$(sysctl -n vm.loadavg)" >>"$tl"; }
if [ ! -d "$clone/.git" ]; then
    git clone --quiet --no-hardlinks "$src" "$clone" >"$out/clone.log" 2>&1 || { stamp clone-failed; exit 2; }
fi
cd "$clone" || exit 2
git rev-parse HEAD >"$out/head.txt"
if [ "$(command -v git)" = /usr/bin/git ] && real_git=$(xcrun --find git 2>/dev/null); then
    PATH="$(dirname "$real_git"):$PATH"; export PATH
    SDKROOT=$(xcrun --show-sdk-path 2>/dev/null) && export SDKROOT
fi
export JIGC_GATE_HYGIENE=off
others() { pgrep -f 'cargo-nextest|nextest run' | grep -v "^$$\$" | wc -l | tr -d ' '; }
stamp build-debug-start
s=$(date +%s); cargo build >"$out/build-debug.log" 2>&1; echo "build-debug rc=$? secs=$(( $(date +%s)-s ))" >>"$tl"
stamp test-compile-start
s=$(date +%s); cargo nextest run --workspace --no-fail-fast --no-run >"$out/test-compile.log" 2>&1; echo "test-compile rc=$? secs=$(( $(date +%s)-s ))" >>"$tl"
# do not start while somebody else's suite is running (up to 20 min), so neither measurement is polluted
w=0; while [ "$(others)" != 0 ] && [ $w -lt 80 ]; do sleep 15; w=$((w+1)); done
echo "waited_for_other_suites=$((w*15))s" >>"$tl"
( while :; do printf '%s others=%s load=%s\n' "$(date -u +%H:%M:%S)" "$(pgrep -f 'cargo-nextest' | wc -l | tr -d ' ')" "$(sysctl -n vm.loadavg)"; sleep 20; done ) >"$out/load-during-run.txt" 2>&1 &
sampler=$!
stamp nextest-start
s=$(date +%s)
cargo nextest run --workspace --no-fail-fast --status-level pass --final-status-level fail >"$out/nextest-full.log" 2>&1
echo "nextest rc=$? secs=$(( $(date +%s)-s ))" >>"$tl"
stamp nextest-end
kill "$sampler" 2>/dev/null
echo done >>"$tl"
