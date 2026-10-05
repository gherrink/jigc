#!/usr/bin/env bash
# Unit cost of the suite's two child processes, in CPU time (user+sys, less sensitive to machine load than wall):
# N runs of `git --version` (real git and the /usr/bin trampoline), `jigc --version`, and two real jigc verbs in a
# fresh rig repo built by the clone's own dev/jigc-rig with the clone's debug binary.
# Reads: <work-dir>/clone (built by measure_suite.sh). Writes: the rig's stderr to <work-dir>/suite/unit-rig.err.
# Prints: one `real/user/sys` line per measured command.
#   usage: spawn_unit_cost.sh <work-dir> [N=100]
[ $# -ge 1 ] || { echo "usage: spawn_unit_cost.sh <work-dir> [N]" >&2; exit 2; }
perf=$1; N=${2:-100}; clone="$perf/clone"; J="$clone/target/debug/jigc"
real_git=$(xcrun --find git); export SDKROOT=$(xcrun --show-sdk-path)
loop() { local i=0; while [ $i -lt "$N" ]; do "$@" >/dev/null 2>&1; i=$((i+1)); done; }
t() { label=$1; shift; TIMEFORMAT="$label: real %R user %U sys %S (x$N)"; time loop "$@"; }
t "real git --version" "$real_git" --version
t "/usr/bin/git --version" /usr/bin/git --version
t "jigc(debug) --version" "$J" --version
export PATH="$(dirname "$real_git"):$PATH"
rig=$("$clone/dev/jigc-rig" fresh --binary "$J" 2>"$perf/suite/unit-rig.err") || { echo "rig failed"; exit 0; }
eval "$rig"
[ -n "${REPO:-}" ] || { echo "no REPO"; exit 0; }
cd "$REPO" || exit 0
t "jigc(debug) validate (fresh rig)" "$JIGC" validate
t "jigc(debug) describe (fresh rig)" "$JIGC" describe
t "real git status (fresh rig)" "$real_git" status --porcelain
