#!/usr/bin/env bash
# arm0-control.sh — protocol.md §5 arm 0, the positive control.
#
# Run INSIDE the container, via `run-session.sh --exec`, so it exercises the same
# copy-in / copy-out / provenance chain a blind session runs on. A control driven any
# other way cannot validate the mechanism the blind sessions actually use, and its whole
# job is to prove that chain before a null result is allowed to mean anything.
#
# Pass condition (§5 arm 0): at least one `jigc doc show … --task …` record in the
# corpus's invocation log, recoverable from $OUT after the container is destroyed.
#
# It proves the CHANNEL records and can be counted. It proves nothing about
# discoverability — the operator already knows the verb.
set -uo pipefail
cd /work

say() { printf '\n=== %s\n' "$1"; }
FAIL=0

say "0 · adopt the corpus with the binary under test"
jigc setup >/dev/null 2>&1 || { echo "setup failed"; exit 1; }
jigc config set invocation-log true >/dev/null 2>&1 || { echo "config set failed"; exit 1; }
jigc --version

say "1 · mint the task"
START="$(jigc start --workflow record-decision \
  "record that the ingest queue drops the oldest sample when it overflows" 2>&1)"
TASK="$(printf '%s\n' "$START" | sed -n 's/^task minted: //p' | head -1)"
[ -n "$TASK" ] || { echo "no task id in start output:"; printf '%s\n' "$START"; exit 1; }
echo "task: $TASK"

say "2 · author a managed doc through the write verbs"
ADDR="$(jigc doc create adr --title "Drop the oldest sample when the ingest queue overflows" \
        --task "$TASK" 2>&1 | tr -d '\r')"
case "$ADDR" in adr:*) echo "created: $ADDR" ;; *) echo "unexpected create ack: $ADDR"; exit 1 ;; esac

for slot in context decision consequences; do
  printf 'Prose for %s, authored by the control arm.\n' "$slot" \
    | jigc doc set-slot "${ADDR}#${slot}" --from-file - --task "$TASK" >/dev/null 2>&1 \
    || { echo "set-slot $slot failed"; FAIL=1; }
done
echo "slots authored"

say "3 · THE CHANNEL — read the staged copy back"
# What comes back is state the author never typed: CLI-owned front matter, the rendered
# empty optional section, the full H1 against a capped slug. That is the arm's honest
# claim — the read serves what the author did not write. It is NOT a claim that the
# author had to run it (protocol §5 arm 0, premise corrected 2026-08-16).
jigc doc show "$ADDR" --task "$TASK" | head -12

say "4 · the pass condition, checked in-container"
LOG=.jigc/logs/invocations.jsonl
if [ ! -f "$LOG" ]; then
  echo "FAIL: no invocation log at $LOG"; FAIL=1
else
  N="$(grep -c '"doc","show".*"--task"' "$LOG" || true)"
  echo "doc show --task records: ${N:-0}"
  [ "${N:-0}" -ge 1 ] || { echo "FAIL: the VERB channel did not record"; FAIL=1; }
fi

say "5 · the VERB-ADJACENT channel also records (protocol §3.3)"
jigc task diff "$TASK" >/dev/null 2>&1
jigc doc list --task "$TASK" >/dev/null 2>&1
grep -cE '"task","diff"|"doc","list"' "$LOG" 2>/dev/null | sed 's/^/adjacent records: /'

say "6 · leave the task open"
# Deliberate: the evidence copied out should show a live task with a staged doc, which is
# the state a blind session is in when its cue card fires.
jigc task list 2>&1 | head -5

echo
[ "$FAIL" -eq 0 ] && echo "ARM 0 PASS — the channel fires and is countable" \
                  || echo "ARM 0 FAIL — fix the instrument before reading any blind session"
exit "$FAIL"
