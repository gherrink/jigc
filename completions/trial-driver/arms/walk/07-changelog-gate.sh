#!/usr/bin/env bash
# 07-changelog-gate.sh — protocol.md §5 arm 07 (M46 Increment 6).
#
# WHICH SET THIS ARM ITERATES: the **declared behaviour change** of protocol §0.2,
# across the two doors the advisory now fires at and the two severities it can
# carry. Not a registry — a 2x2 the wave created by moving one member into the
# previewed set:
#
#                        default (advisory)      promoted (blocking)
#   jigc task validate   fires, exit 0           fires, exit 3      <- NEW at this door
#   jigc task finalize   fires, commits anyway   refuses
#
# THIS ARM CARRIES §5's STANDARD FOR §0.2, which §1 row 2 defers to: (a) names what
# it objects to; (b) says the consequence; (c) names the route; (d) the route, run
# verbatim, works.
#
# Why the door matters: before M46 the advisory was computed only at `finalize`,
# where its in-task route was already dead — after a landed commit the task is
# gone and `jigc doc create changelog --task <id>` answers `no task <id>`. Moving
# it into `preview_gates` puts it where its route can still be followed.
#
# THE COMMIT DOC IS FILLED FIRST, deliberately. An unfilled `commit` draws two
# blocking findings of its own that dominate the output and force exit 3, and the
# cell under test is "the ONLY finding, at exit 0" — the exact byte-shape that
# replaced rc.11's "no findings — the task validates clean".
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }

KEY=validation.changelog-recording.gate-granted-unused.severity

say "0 · adopt, mint a gate-granting task, and fill its commit doc"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version
jigc start --workflow single-task "add a rate limiter" >/dev/null 2>&1
T="$(jigc task list | awk '/^  [a-z]/{print $1; exit}')"
[ -n "$T" ] || { echo "no task minted"; exit 1; }
jigc doc set-field "commit:$T#header/type"  --value feat   --task "$T" >/dev/null 2>&1
jigc doc set-field "commit:$T#header/scope" --value ingest --task "$T" >/dev/null 2>&1
printf 'add a rate limiter to the ingest path\n' \
  | jigc doc set-slot "commit:$T#summary" --from-file - --task "$T" >/dev/null 2>&1
echo "task: $T"

say "A · the declared change: a conformant task no longer validates silent"
step jigc task validate "$T"
V="$(jigc task validate "$T" 2>&1)"; VRC=$?

bar "the advisory fires at jigc task validate — NEW at this door" \
    "printf '%s' \"\$V\" | grep -q 'changelog-recording.gate-granted-unused'"
bar "(a) it names the workflow and the gate it granted" \
    "printf '%s' \"\$V\" | grep -q 'workflow .single-task. grants the .changelog. create-gate'"
bar "(b) it says the consequence — this task recorded no entry" \
    "printf '%s' \"\$V\" | grep -q 'recorded no changelog entry'"
bar "(c) it names a route with real argv, scoped to THIS task" \
    "printf '%s' \"\$V\" | grep -q -- \"--task $T\""
bar "(c) …and names the not-user-facing branch too, so it is not a demand" \
    "printf '%s' \"\$V\" | grep -q 'no action is needed'"
bar "exit stays 0 at the default severity" "test \$VRC -eq 0"
bar "rc.11's byte-shape is gone: it does NOT say the task validates clean" \
    "! printf '%s' \"\$V\" | grep -q 'no findings'"
# Count FINDING lines, not lines containing "·": the standing trailer
# ("— jigc · run `jigc start` …") carries one too, so the naive count is 2 and the
# bar fails against correct output. Findings are prefixed by their severity.
bar "it is the ONLY finding — nothing else was disturbed" \
    "test \$(printf '%s\n' \"\$V\" | grep -cE '^(advisory|blocking) · ') -eq 1"

say "B · the suppressing event is a WRITE-TOUCH, not the doc existing"
# M46 moved the predicate off an item count and onto a staged write measured
# against the un-authored baseline. `doc create` alone lands exactly the pristine
# skeleton, so it is NOT a touch — and this cell is what proves the predicate is
# not "a changelog doc exists in this task".
jigc doc create changelog --title Changelog --task "$T" >/dev/null 2>&1
V2="$(jigc task validate "$T" 2>&1)"
bar "creating the changelog and stopping does NOT suppress it" \
    "printf '%s' \"\$V2\" | grep -q 'gate-granted-unused'"

say "B · (d) now follow the printed route VERBATIM"
step jigc doc add-item "changelog:changelog#unreleased-changes" --title Added --task "$T"
V3="$(jigc task validate "$T" 2>&1)"
bar "(d) the route runs and the advisory is suppressed" \
    "! printf '%s' \"\$V3\" | grep -q 'gate-granted-unused'"
bar "…and what remains is honest unfinished work, not the gate" \
    "printf '%s' \"\$V3\" | grep -q 'required-slot-present'"

# DECLARED BOUND, stated rather than glossed: this cell separates the write-touch
# predicate from "the doc exists", but NOT from an item count — adding an item
# moves both. The discriminating cell would be a staged write to the changelog
# that creates no item, and no shipped verb produces one, so it is not reached
# here. `changelog_gate_advisory.rs` and M46's own write-touch suite carry that.

say "C · the promoted severity — the door parity M46 shipped this for"
jigc config set "$KEY" blocking >/dev/null 2>&1
jigc task validate "$T" >/dev/null 2>&1; PROMOTED_CLEAN=$?
# put the task back into the un-recorded state the gate objects to
# `--force` is the consent this door refuses without since M50 Increment 3: minting a
# task stages its commit doc, so an ordinary discard is refused from the moment the task
# exists (`task-discard.staged-prose`). This is CLEANUP, not a subject under test — the
# refusal itself is driven, both sides, in pre-trial-findings.md PT-8.
jigc task discard "$T" --force >/dev/null 2>&1
jigc start --workflow single-task "add a second rate limiter" >/dev/null 2>&1
T2="$(jigc task list | awk '/^  [a-z]/{print $1; exit}')"
jigc doc set-field "commit:$T2#header/type" --value feat --task "$T2" >/dev/null 2>&1
printf 'second limiter\n' | jigc doc set-slot "commit:$T2#summary" --from-file - --task "$T2" >/dev/null 2>&1
step jigc task validate "$T2"
jigc task validate "$T2" >/dev/null 2>&1; PRC=$?
bar "promoted to blocking, task validate exits 3 on the state finalize refuses" \
    "test $PRC -eq 3"
jigc config set "$KEY" advisory >/dev/null 2>&1
jigc task validate "$T2" >/dev/null 2>&1; DRC=$?
bar "…and back at the default severity the same state exits 0 again" "test $DRC -eq 0"

say "SUMMARY"
echo "  §5 standard for the declared change §0.2: (a) names the gate · (b) says the"
echo "  consequence · (c) routes with live in-task argv · (d) the route suppresses it"
if [ "$FAIL" -eq 0 ]; then echo "ARM 07 PASS"; else echo "ARM 07 FAIL"; fi
exit "$FAIL"
