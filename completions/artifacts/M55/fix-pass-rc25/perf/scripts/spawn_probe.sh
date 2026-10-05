#!/usr/bin/env bash
# For one test in the private clone: (1) run it alone, unshimmed, and time it; (2) run it again with the
# spawn shim standing in for `git` on PATH and for the jigc binary the suite drives, and summarize the
# children: how many git / jigc processes, their summed wall time, and git time nested inside jigc.
# Reads: <work-dir>/clone (built by measure_suite.sh) and the compiled shim at <work-dir>/shim/shim, with a copy
# of it named <work-dir>/shim/git (shim/shim.c beside this script is its source). Writes: the test's two logs and
# its .spawns file under <work-dir>/suite. Prints: the test's name and one summary line of counts and seconds.
#   usage: spawn_probe.sh <work-dir> <group-binary e.g. g_milestone> <suite::test>
# Swaps clone/target/debug/jigc for the shim and back (rename only, inside the clone; nothing is removed).
set -u
[ $# -ge 3 ] || { echo "usage: spawn_probe.sh <work-dir> <group-binary> <suite::test>" >&2; exit 2; }
perf=$1; grp=$2; test=$3
clone="$perf/clone"; shim="$perf/shim"
bin=$(find "$clone/target/debug/deps" -maxdepth 1 -type f -perm +111 -name "${grp}-*" ! -name '*.d' | head -1)
[ -x "$bin" ] || { echo "no test binary for $grp"; exit 2; }
real_git=$(xcrun --find git); gitdir=$(dirname "$real_git")
export SDKROOT=$(xcrun --show-sdk-path) CARGO_MANIFEST_DIR="$clone/crates/cli" CLAUDECODE=""
jig="$clone/target/debug/jigc"
out="$perf/suite/probe-$(echo "$test" | tr ':' '_' | cut -c1-80)"
# 1. alone, unshimmed
s=$(perl -MTime::HiRes=time -e 'print time')
PATH="$gitdir:$PATH" "$bin" --exact "$test" >"$out.plain.log" 2>&1; rc1=$?
e=$(perl -MTime::HiRes=time -e 'print time')
plain=$(perl -e "printf '%.2f', $e-$s")
# 2. shimmed
: >"$out.spawns"
mv "$jig" "$jig.real" && cp "$shim/shim" "$jig" || exit 2
s=$(perl -MTime::HiRes=time -e 'print time')
SPAWN_LOG="$out.spawns" SHIM_REAL_GIT="$real_git" SHIM_REAL_JIGC="$jig.real" PATH="$shim:$gitdir:$PATH" "$bin" --exact "$test" >"$out.shim.log" 2>&1; rc2=$?
e=$(perl -MTime::HiRes=time -e 'print time')
mv "$jig" "$jig.shim-used" && mv "$jig.real" "$jig"
shimmed=$(perl -e "printf '%.2f', $e-$s")
awk -v t="$test" -v plain="$plain" -v sh="$shimmed" -v rc1="$rc1" -v rc2="$rc2" '
  $1=="git" && $2==0 {g0++; gt0+=$3} $1=="git" && $2==1 {g1++; gt1+=$3} $1=="jigc" {j++; jt+=$3}
  END { top=gt0+jt; printf "%s\n  alone %.1fs (rc %s) | shimmed %.1fs (rc %s) | git direct %d calls %.1fs | jigc %d calls %.1fs (of which nested git %d calls %.1fs) | children %.1fs = %.0f%% of the shimmed wall | in-process %.1fs\n", t, plain, rc1, sh, rc2, g0, gt0, j, jt, g1, gt1, top, 100*top/sh, sh-top }' "$out.spawns"
