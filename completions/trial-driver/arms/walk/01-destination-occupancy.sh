#!/usr/bin/env bash
# 01-destination-occupancy.sh — cue-card-postmortem.md §6 step 5.
#
#   The destination-occupancy guard (F7) gets an arm on the walk too — two
#   `doc rename`s onto one identity, in both homes. It is three commands and it
#   converts an [R]/test-fenced row into a walk-reached one.
#
# Why the walk and not a blind session: this needs a specific wrong command typed
# on purpose, and §6 is explicit that "the blind sessions were never the right
# instrument for a refusal that needs a specific wrong command typed on purpose."
#
# PASS CONDITION, stated before the run and deliberately weak on wording:
#   every step records an exit code and its output, and the rename onto an
#   occupied identity does not silently succeed. The exact ack text is CAPTURED,
#   never asserted — this arm exists to find out what the binary says, not to pin
#   a string its author guessed. A refusal, a route, or a documented no-op all
#   pass; a silent success does not.
#
# Two things this arm learned about itself on its first two runs, kept because a
# walk arm that has never been driven is a plan, not an instrument:
#   * the runtime image is node-based and has NO python3;
#   * `record-decision` declares ONE `allows-create` adr role, so a second
#     `doc create adr` in the same task is not a second document. Every step now
#     records its own exit code, so "printed nothing" can be told apart from
#     "failed", which it could not be on run 2.
set -uo pipefail
cd /work

FAIL=0
say()  { printf '\n=== %s\n' "$1"; }
note() { echo "  !! $1"; FAIL=1; }

# Run one command, echoing it, its combined output and its exit code. The exit
# code is the point: on this arm's second run a step printed nothing and the
# record could not say whether it had succeeded quietly or failed quietly.
step() {
  echo "--- \$ $*"
  local out rc
  out="$("$@" 2>&1)"; rc=$?
  [ -n "$out" ] && printf '%s\n' "$out" | sed 's/^/    /'
  echo "--- exit: $rc"
  LAST_RC=$rc
  LAST_OUT="$out"
}

say "0 · adopt, so there is a managed store to rename inside"
step jigc setup
[ "$LAST_RC" -eq 0 ] || { note "setup failed — nothing below is readable"; exit 1; }
step jigc config set invocation-log true
git add -A
git -c user.name='Walk' -c user.email='walk@example.invalid' \
    commit -q -m 'chore: adopt jigc' || true

say "1 · mint a task"
# `node`, not `python3`: the runtime image carries no python. Found by running it.
TASK="$(jigc start --workflow record-decision "decisions that will collide" \
        --format json 2>/dev/null \
        | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{process.stdout.write(String(JSON.parse(s).task||""))}catch(e){}})')"
echo "task: ${TASK:-<none>}"
[ -n "$TASK" ] || { note "no task id came back from start --format json"; exit 1; }

say "2 · the doc this task's create-gate allows"
step jigc doc create adr --title "Bound the number of distinct series" --task "$TASK"
FIRST_ACK="$LAST_OUT"

say "2b · a SECOND create of the same type in the same task"
# Not a second document: `record-decision` declares one `allows-create` adr role.
# Captured with its exit code so the behaviour is on the record either way — M43
# settled that a same-identity collision acks `existed` and binds, and what a
# DIFFERENT title does at the same gate is what this step reads.
step jigc doc create adr --title "Cap the retention window at seven days" --task "$TASK"
SECOND_RC="$LAST_RC"
echo "second-create exit: $SECOND_RC"

say "3 · what the task actually holds"
step jigc doc list --task "$TASK"

say "4 · rename onto an identity that is already taken — the occupancy probe"
# Whatever step 2b left, the FIRST doc's identity is occupied. Renaming any doc
# onto it is the probe; if 2b produced no second doc, this reads the guard from
# the one-doc side instead, and says so.
step jigc doc rename adr:bound-the-number-of-distinct \
     --to "Bound the number of distinct series" --task "$TASK"
echo "self-rename (same identity) exit: $LAST_RC"

say "5 · rename onto a DIFFERENT existing identity, if there are two docs"
if printf '%s' "$LAST_OUT" >/dev/null && jigc doc list --task "$TASK" 2>/dev/null | grep -q 'cap-the-retention'; then
  step jigc doc rename adr:cap-the-retention-window \
       --to "Bound the number of distinct series" --task "$TASK"
  [ "$LAST_RC" -eq 0 ] && note "rename onto an OCCUPIED identity exited 0 — read the ack above"
else
  echo "    only one adr exists in this task, so the two-doc collision is not reachable here."
  echo "    That is a finding about the create-gate, not about the occupancy guard:"
  echo "    a one-role workflow cannot hold two docs of that type."
fi

say "6 · the committed home"
step jigc task finalize "$TASK"
step jigc doc list

say "7 · top-level rename onto an existing identity"
step jigc rename adr:bound-the-number-of-distinct --to "Bound the number of distinct series"
echo "top-level self-rename exit: $LAST_RC"

say "done"
echo "arm exit: $FAIL   (0 = every step recorded an exit code and nothing silently succeeded)"
exit "$FAIL"
