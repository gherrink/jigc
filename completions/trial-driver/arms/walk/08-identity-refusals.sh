#!/usr/bin/env bash
# 08-identity-refusals.sh — protocol.md §5 arm 08.
#
# WHICH SET THIS ARM ITERATES: the identity surface's **refusal set**, which
# cue-card-postmortem.md §6 enumerates as T7–T11 and which the 1.0.0-gate walk
# lost. That loss is the reason walk arms are scripts now:
#
#   "§5 arm 4 already chartered a committed-identity `doc rename` and the walk
#    record does not carry it — a narrative walk lost a chartered probe."
#
# The structural fact the whole set turns on, stated once:
#
#   `jigc doc rename`'s subject is a STAGED, never-committed doc bound to an open
#   task's role. A COMMITTED doc's rename is the top-level `jigc rename`.
#
# So each cell below is one (subject-state x verb) pair, and the pairs that must
# refuse are exactly the ones where those two homes are confused for each other.
#
# ARM 01's DECLARED BOUND IS DISCHARGED HERE. It could not reach the two-doc
# collision, because `record-decision` declares ONE `allows-create` adr role and a
# second `doc create` is refused outright — a one-role workflow cannot hold two
# docs to collide. Two COMMITTED docs and the top-level verb can, and do.
#
# CAPTURE, DO NOT GUESS: every address is extracted by pattern from the ack, never
# by `tail -1`. Learned by getting it wrong — `doc create` sometimes prints the
# standing trailer after the address, so `tail -1` silently yields
# "— jigc · run `jigc start` …" and every later step addresses garbage.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
addr() { grep -oE "^$1:[a-z0-9-]+$" | head -1; }          # capture, never tail -1
newtask() { jigc task list | awk '/^  [a-z]/{print $1; exit}'; }

land_adr() { # <title> -> echoes the committed address
  jigc start --workflow record-decision "record $1" >/dev/null 2>&1
  local t a s; t="$(newtask)"
  a="$(jigc doc create adr --title "$1" --task "$t" 2>&1 | addr adr)"
  for s in context decision consequences; do
    echo 'Recorded for the walk.' \
      | jigc doc set-slot "$a#$s" --from-file - --task "$t" >/dev/null 2>&1
  done
  jigc doc set-field "commit:$t#header/type" --value docs --task "$t" >/dev/null 2>&1
  printf 'record %s\n' "$1" | jigc doc set-slot "commit:$t#summary" --from-file - --task "$t" >/dev/null 2>&1
  jigc task finalize "$t" >/dev/null 2>&1
  echo "$a"
}

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

say "T10 · doc rename against a COMMITTED identity — the refusal, not exit 0"
A1="$(land_adr 'Drop the oldest sample on overflow')"
echo "committed: $A1"
jigc start --workflow record-decision "revise the overflow policy" >/dev/null 2>&1
TR="$(newtask)"
step jigc doc rename "$A1" --to "Evict the oldest sample on overflow" --task "$TR"
R10="$(jigc doc rename "$A1" --to 'Evict the oldest sample on overflow' --task "$TR" 2>&1)"
bar "it refuses with write.identity-change" "printf '%s' \"\$R10\" | grep -q 'write.identity-change'"
bar "it explains WHY a committed doc is different — the path IS the identity" \
    "printf '%s' \"\$R10\" | grep -q \"committed doc's path IS its identity\""
bar "it routes at the top-level, task-less verb" \
    "printf '%s' \"\$R10\" | grep -q 'jigc rename $A1'"
bar "…and states the precondition rather than handing over an argv that would fail" \
    "printf '%s' \"\$R10\" | grep -q 'once this task is finalized or discarded'"
bar "the same-slug retitle is named as the thing that IS supported" \
    "printf '%s' \"\$R10\" | grep -q 'same-slug retitle'"
jigc task discard "$TR" >/dev/null 2>&1

say "T11 · doc rename against a SINGLETON doctype — nothing to rename at all"
jigc start --workflow form-vision "the project vision" >/dev/null 2>&1
TV="$(newtask)"
AV="$(jigc doc create vision --title Vision --task "$TV" 2>&1 | addr vision)"
echo "created: $AV"
step jigc doc rename "$AV" --to "Product Vision" --task "$TV"
R11="$(jigc doc rename "$AV" --to 'Product Vision' --task "$TV" 2>&1)"
bar "the singleton refuses with write.identity-change"  "printf '%s' \"\$R11\" | grep -q 'write.identity-change'"
bar "it says the H1 is the SCHEMA's, not the author's"  "printf '%s' \"\$R11\" | grep -q 'supplied by the schema'"
bar "its route says there is nothing to rename — a pack change, not a write" \
    "printf '%s' \"\$R11\" | grep -q 'nothing to rename'"
bar "…and points at the verb that DOES edit it"         "printf '%s' \"\$R11\" | grep -q 'jigc doc set-slot'"
jigc task discard "$TV" >/dev/null 2>&1

say "OCCUPANCY · two committed docs, one identity — arm 01's declared bound, discharged"
A2="$(land_adr 'Cap distinct series at a ceiling')"
echo "second committed: $A2"
bar "both docs really are committed" \
    "test \$(git ls-files docs/decisions/ | wc -l | tr -d ' ') -eq 2"
step jigc rename "$A2" --to "Drop the oldest sample on overflow"
ROC="$(jigc rename "$A2" --to 'Drop the oldest sample on overflow' 2>&1)"
jigc rename "$A2" --to 'Drop the oldest sample on overflow' >/dev/null 2>&1; OCRC=$?
bar "the destination-occupancy guard refuses"      "test $OCRC -ne 0"
bar "it names the occupied destination"            "printf '%s' \"\$ROC\" | grep -q 'already exists at docs/decisions/drop-the-oldest-sample.md'"
bar "nothing was moved — the loser is intact"      "test -f docs/decisions/cap-distinct-series.md"
bar "…and so is the incumbent"                     "test -f docs/decisions/drop-the-oldest-sample.md"

say "OCCUPANCY · the sibling refusal on the SAME verb, for comparison"
# Recorded, not asserted as correct: this is the comparison that makes the
# observation in pre-trial-findings.md legible rather than a lone complaint.
step jigc rename adr:no-such-doc --to "Something else"
RUN="$(jigc rename adr:no-such-doc --to 'Something else' 2>&1)"
bar "the unknown-doc refusal on this same verb DOES carry a route" \
    "printf '%s' \"\$RUN\" | grep -q 'route:'"
echo "  OBSERVE  the occupancy refusal above carries NO route line:"
printf '%s' "$ROC" | sed 's/^/           | /'
echo "           -> recorded in pre-trial-findings.md as a candidate finding."
echo "           -> NOT adjudicated here: this arm builds the instrument."

say "T-idempotent · a same-title rename is a no-op ack, not a dressed-up failure"
step jigc rename "$A2" --to "Cap distinct series at a ceiling"
RNO="$(jigc rename "$A2" --to 'Cap distinct series at a ceiling' 2>&1)"
jigc rename "$A2" --to 'Cap distinct series at a ceiling' >/dev/null 2>&1; NRC=$?
bar "the idempotent rename exits 0"        "test $NRC -eq 0"
bar "…and says plainly that nothing moved" "printf '%s' \"\$RNO\" | grep -q 'nothing renamed, nothing committed'"

say "SUMMARY"
echo "  T10 committed-identity refusal · T11 singleton refusal · destination occupancy"
echo "  in the committed home · the idempotent no-op. Arm 01's declared bound is"
echo "  discharged: the two-doc collision needs two COMMITTED docs and the top-level verb."
if [ "$FAIL" -eq 0 ]; then echo "ARM 08 PASS"; else echo "ARM 08 FAIL"; fi
exit "$FAIL"
