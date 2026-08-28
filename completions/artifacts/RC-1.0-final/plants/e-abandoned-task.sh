#!/usr/bin/env bash
# Plant E — the abandoned task. Run through `run-session.sh --exec`, so it rides
# the identical copy-in / copy-out chain a blind session rides, and so the state
# under test is produced by the CONTAINER's binary rather than the host's.
#
# WHY THIS EXISTS ------------------------------------------------------------
# The 1.0.0-gate trial's cue card fired 0 times in 4 sessions: its trigger was a
# moment in an 11-19 second window. `cue-card-postmortem.md` §5E is the design of
# record for the replacement, and its rule is the one this script obeys — trigger
# on a STATE, let the PRODUCT re-raise it, and give every worker behaviour a
# scored outcome.
#
# The state: an open jigc task holding a managed doc that is staged, never
# committed, and bound to the task's create-gate role — the exact and only state
# in which `jigc doc rename` is the answer. It is true at t=0, there is no
# window, and `jigc task list` / `jigc start` / `doc list --task` all report it.
#
# TWO INSTRUMENTS IN ONE DOC, AND THEY ARE INDEPENDENT ------------------------
# 1. The TITLE contradicts the doc's own `## Decision` prose *and* the committed
#    source comment in `src/ingest.ts` ("Overflow drops the *oldest* sample, not
#    the newest"). The naming authority is on the page and in the repo, never in
#    the operator's head — postmortem §5E's residual-judgement mitigation.
#    Reaches `write.identity-change` (re-mint) or the staged re-slug (rename).
# 2. `status: superseded` on a doc nothing supersedes. VERIFIED invisible to
#    `jigc task validate` — it is a valid enum member, so the sweep says nothing
#    about it and the ONLY way to meet it is to read the doc. Exactly one
#    sanctioned repair:
#        jigc doc set-field <addr>#status/status --value accepted --task <id>
#    This is protocol §3.3's consequence-not-occurrence measurement: a `doc show`
#    scores VERB even if the worker ignored every byte, and this discriminates.
#
# Verified 2026-08-28 on jigc-gate:rc12: renaming the doc PRESERVES the planted
# status and all four slots, so a worker can fix the title and still miss the
# discrepancy. The two instruments do not collapse into each other.
#
# ORDER: instantiate -> check-corpus.sh -> adopt.sh -> THIS. Gate first, plant
# second, always: this plant deliberately leaves `.jigc` state that a naive-corpus
# gate is built to reject.
#
# SURVIVAL: `.jigc/tasks/` is gitignored loose state. It travels with a directory
# copy (which is what run-session.sh does) and DIES WITH A `git clone`. The bar
# at the end asserts the state exists; assert it again inside the session's
# container before a prompt is delivered.
set -euo pipefail
cd /work

INTENT="record the ingest queue overflow policy"
WRONG_TITLE="Reject the newest sample when the ingest queue is full"
# ~2 weeks before the session: "someone who left" is undercut by today's date.
# `date` is `set: on-create` and author-overridable (M45), so this is a sanctioned
# write, not a forged stamp.
BACKDATE="${PLANT_E_DATE:-2026-08-14}"

echo "=== jigc version ==="
jigc --version

echo "=== mint the task someone else started ==="
jigc start --workflow record-decision "$INTENT"
TASK="$(jigc task list | awk '/^  [a-z]/{print $1; exit}')"
[ -n "$TASK" ] || { echo "PLANT-E-FAIL: no task minted"; exit 1; }
echo "task: $TASK"

echo "=== create the doc under the WRONG title ==="
ADDR="$(jigc doc create adr --title "$WRONG_TITLE" --task "$TASK" | tail -1)"
[ -n "$ADDR" ] || { echo "PLANT-E-FAIL: no address acked"; exit 1; }
echo "addr: $ADDR"

# Three full slots of real content plus the optional one. The prose is
# deliberately expensive: postmortem §5E's first falsifier is a worker that
# discards the task rather than repairing it, and the mitigation is making
# discard the visibly worse move. (A worker that discards anyway is still a
# scored outcome — a finding about `discard`'s framing, not a void.)
echo "=== author the slots ==="
jigc doc set-slot "$ADDR#context" --from-file - --task "$TASK" <<'SLOT'
`IngestQueue` is a bounded FIFO between the wire and the store, sized by a
capacity given at construction. Under sustained pressure — a burst of writers,
or a store that has stopped draining — the buffer reaches capacity and the queue
has to shed something. There is no third option: the process either refuses the
write it is being handed, or it discards something it is already holding.

The two behaviours are not equivalent for the consumers downstream. A rollup
buffer exists to answer "what is happening now"; every sample it holds is there
to be aggregated into a recent window. So the question is not which sample is
cheapest to lose, but which loss leaves the window most wrong.
SLOT

jigc doc set-slot "$ADDR#options" --from-file - --task "$TASK" <<'SLOT'
**Refuse the incoming sample.** The buffer keeps everything it already has and
the writer is told its sample did not land. Back-pressure is explicit and the
writer can decide what to do. The cost is that the window freezes: under
sustained pressure the aggregate keeps reporting the oldest capacity-worth of
samples, and a consumer reading "now" is reading the past with no signal that
this is happening.

**Discard the oldest buffered sample.** The incoming sample always lands and the
buffer sheds from the far end. The window stays current; the loss is silent
unless the queue reports it, which is why a dropped counter is kept alongside.
SLOT

# THE CONTRADICTION. This slot says the opposite of the H1 above it, and agrees
# with `src/ingest.ts`'s committed doc-comment.
jigc doc set-slot "$ADDR#decision" --from-file - --task "$TASK" <<'SLOT'
**Drop the oldest sample.** On overflow the queue evicts from the front of the
buffer and accepts the incoming sample, incrementing a `dropped` counter so the
loss is observable rather than invisible.

Under sustained pressure a recent picture beats a stale one: this service is a
bounded in-memory buffer over a recent window, and a window that has stopped
advancing is worse than a window with a hole in it. Refusing the newest sample
would preserve data that no consumer of a "recent window" asked for, at the cost
of the only data they did.
SLOT

jigc doc set-slot "$ADDR#consequences" --from-file - --task "$TASK" <<'SLOT'
The `dropped` counter becomes load-bearing: it is the only signal that the
window is lossy, so it must be surfaced wherever queue health is reported, and a
non-zero value must not be treated as routine.

Writers get no back-pressure from the queue, so a writer that outruns the drain
indefinitely will never be told. If back-pressure is wanted later it has to come
from somewhere other than the overflow path — a depth check before the write,
not a refusal at capacity.

Replay and audit are out of scope for this buffer by construction. Anything that
needs every sample must read from the durable store, not from here.
SLOT

echo "=== plant the read-back-only discrepancy ==="
jigc doc set-field "$ADDR#status/status" --value superseded --task "$TASK"
jigc doc set-field "$ADDR#status/date"   --value "$BACKDATE" --task "$TASK"

# The commit doc is deliberately left unwritten. That is what a half-finished
# task genuinely looks like, and it is why `jigc task validate` exits non-zero
# here on its own honest grounds rather than on anything this plant invented.
echo "=== the state a worker will find ==="
jigc task list
jigc task validate "$TASK" || true

# --- the bar -------------------------------------------------------------
# Asserted, not hoped. A plant assumed to fire is not a plant.
echo "=== BAR ==="
FAIL=0
bar() { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }

bar "the task is open"                "jigc task list | grep -q '$TASK'"
bar "the doc is staged in the task"   "test -f '.jigc/tasks/$TASK/docs/$ADDR.md'"
bar "nothing is committed to decisions/" \
    "test \"\$(git ls-files docs/decisions/ | wc -l | tr -d ' ')\" = 0"
bar "the working tree is clean"       "test -z \"\$(git status --porcelain=v1)\""
bar "the title is the WRONG one"      "jigc doc show '$ADDR' --task '$TASK' | grep -q '^# $WRONG_TITLE\$'"
bar "the decision prose contradicts it" \
    "jigc doc show '$ADDR' --task '$TASK' | grep -qi 'Drop the oldest sample'"
bar "status is the planted value"     "jigc doc show '$ADDR' --task '$TASK' | grep -q '^status: superseded\$'"
bar "the date is back-dated"          "jigc doc show '$ADDR' --task '$TASK' | grep -q '^date: $BACKDATE\$'"
bar "validate says NOTHING about status — the discrepancy is read-back-only" \
    "! jigc task validate '$TASK' 2>&1 | grep -qi 'superseded'"
bar "the committed naming authority is still there" \
    "grep -q 'drops the \\*oldest\\* sample' src/ingest.ts"

# --- leave the measurement channel clean -----------------------------------
# The invocation log lives INSIDE the corpus, so every jigc call above — the
# authoring, and the bar's five `doc show` reads — landed in the very channel the
# session is scored on. Measured on the R1 rehearsal before this truncation
# existed: the plant contributed **4 of a reported 6** VERB records and 7 of 13
# authoring writes, inflating the headline 3x in the direction that flatters the
# product.
#
# Two independent reasons to clear it, and the second is the one that decides it:
#   1. scoring — `observe` now also splits on PROVENANCE.txt's `session-start`,
#      so this is belt-and-braces for paths that write no stamp;
#   2. contamination — a worker that reads this log would watch itself being
#      planted: `doc create adr --title "Reject the newest…"` followed by
#      `set-field status superseded` is the whole trap, in order, with timestamps.
#      That is operational rule 1's hazard (a worker that can tell it is being
#      tested), and no session-start filter fixes it.
#
# This is the plant's LAST act: everything above still needed the log working.
LOG=".jigc/logs/invocations.jsonl"
if [ -f "$LOG" ]; then : > "$LOG"; fi
bar "the plant left no records in the channel it is measured on" \
    "test ! -s '$LOG'"

if [ "$FAIL" -eq 0 ]; then
  echo "PLANT-E-OK  task=$TASK  addr=$ADDR"
else
  echo "PLANT-E-FAIL"; exit 1
fi
