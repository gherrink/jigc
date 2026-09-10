#!/usr/bin/env bash
# 22-m50-orientation-and-carried.sh — the wave's own claim surface, plus two carried
# defects this trial is the registered trigger for.
#
# WHICH SET THIS ARM ITERATES: `engine::result::OrientationView`, matched against the
# three shipped variants by driving the states that produce them — clean, active-task,
# and the minting form that appends `also open:`. It is the class's defining case-set,
# named from the code, not a hand list: a fourth variant would have no state here and
# the arm would say so by having nothing to drive it with.
#
# TWO CELLS ARE EXPECTED RED, and they are MEASUREMENTS, not regression checks — the
# same shape as arm 17 on rc.13, which is how that trial found its blocking finding:
#
#   N27  `jigc task diff <id>` cold-start answers almost nothing. Its recorded trigger
#        IS this trial's plant-E arm: "if a worker again goes to the filesystem to read
#        its own abandoned working area, this row is the named candidate and is
#        re-argued against its cost, not re-discovered."
#   N15  a `--task` read of an address that resolves nowhere is byte-identical to the
#        task-less one, says *committed*, and its route DROPS `--task` — so following it
#        verbatim serves the committed copy to a reader holding a staged one.
#
# The bars for those two stay green-when-fixed and red-when-not. A table whose
# known-bad cells read green would be the instrument lying.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
note() { printf '  NOTE  %s\n' "$1"; }

say "0 · adopt"
jigc setup >/dev/null 2>&1 || { echo "setup failed"; exit 1; }
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

say "1 · OrientationView::Clean — a repo holding no task"
jigc start --format json > /tmp/o-clean.json 2>&1
CLEAN_STATE="$(node -e 'process.stdout.write(String(require("/tmp/o-clean.json").state))' 2>/dev/null)"
CLEAN_VER="$(node -e 'process.stdout.write(String(require("/tmp/o-clean.json").schema_version))' 2>/dev/null)"
echo "state=$CLEAN_STATE  envelope schema_version=$CLEAN_VER"
bar "a repo with no task is 'clean'"            "[ \"$CLEAN_STATE\" = clean ]"
bar "the envelope carries schema_version 3"      "[ \"$CLEAN_VER\" = 3 ]"

say "2 · mint a task and stage prose in it"
START="$(jigc start --workflow record-decision 'record the ingest queue overflow policy' 2>&1)"
printf '%s\n' "$START"
TASK="$(printf '%s\n' "$START" | sed -n 's/^task minted: //p' | head -1)"
[ -n "$TASK" ] || { echo "no task id"; exit 1; }
ADDR="$(jigc doc create adr --title 'Drop the oldest sample on ingest queue overflow' --task "$TASK" 2>&1 | tail -1)"
printf 'Prose authored by the walk arm.\n' | jigc doc set-slot "${ADDR}#context" --from-file - --task "$TASK" >/dev/null 2>&1

say "3 · OrientationView::ActiveTask — the lie M50 killed"
step jigc start
jigc start --format json > /tmp/o-active.json 2>&1
ACT_STATE="$(node -e 'process.stdout.write(String(require("/tmp/o-active.json").state))' 2>/dev/null)"
echo "state=$ACT_STATE"
bar "a repo holding a live task is NOT reported clean" "[ \"$ACT_STATE\" != clean ]"
bar "it is the active-task view"                        "[ \"$ACT_STATE\" = active-task ]"
ORIENT="$(jigc start 2>&1)"
bar "the view names the task"                    "printf '%s' \"\$ORIENT\" | grep -q '$TASK'"
bar "it names what the task stages"              "printf '%s' \"\$ORIENT\" | grep -q 'staged:'"
bar "the staged line names the doc identity"     "printf '%s' \"\$ORIENT\" | grep -q '$ADDR'"
bar "findings is stated, never silently absent"  "printf '%s' \"\$ORIENT\" | grep -q 'findings:'"
bar "it routes at resume"                        "printf '%s' \"\$ORIENT\" | grep -q 'jigc start --task $TASK'"
bar "it routes at validate"                      "printf '%s' \"\$ORIENT\" | grep -q 'jigc task validate $TASK'"
bar "it routes at finalize"                      "printf '%s' \"\$ORIENT\" | grep -q 'jigc task finalize $TASK'"
bar "the abandon route carries the consent the door needs" \
    "printf '%s' \"\$ORIENT\" | grep -q 'jigc task discard $TASK --force'"
# The claim under test, stated as a bar so the answer is recorded either way. This is
# NOT a defect bar: no design doc promises a read directive here. It is the fact the
# headline's mechanism turns on, and protocol.md §3.1 pre-registers it.
if printf '%s' "$ORIENT" | grep -qE 'doc show|doc list'; then
  note "the active-task view DOES name a read verb"
else
  note "the active-task view names NO read verb — resume/validate/finalize/discard only (protocol.md §3.1)"
fi

say "4 · the 'also open:' block on the minting form"
SECOND="$(jigc start --workflow record-decision 'a second, unrelated decision' 2>&1)"
printf '%s\n' "$SECOND"
bar "a second mint reports the work already open"  "printf '%s' \"\$SECOND\" | grep -q 'also open:'"
bar "it names the open task by id"                 "printf '%s' \"\$SECOND\" | grep -q '$TASK'"
bar "it does not suppress the mint"                "printf '%s' \"\$SECOND\" | grep -q 'task minted:'"
if printf '%s' "$SECOND" | grep -A3 'also open:' | grep -qE 'task validate|task finalize|task discard'; then
  note "the also-open block carries the validate/finalize/discard directives too"
else
  note "the also-open block carries RESUME ONLY — the four directives are the orientation view's (handover correction C3)"
fi

say "5 · N27 (EXPECTED RED) — task diff's cold-start form"
step jigc task diff "$TASK"
DIFF="$(jigc task diff "$TASK" 2>&1)"
bar "N27: the cold-start diff names the task id"       "printf '%s' \"\$DIFF\" | grep -q '$TASK'"
bar "N27: it names the workflow the task was minted from" \
    "printf '%s' \"\$DIFF\" | grep -qi 'record-decision'"
bar "N27: it names the intent"                         "printf '%s' \"\$DIFF\" | grep -qi 'overflow policy'"
jigc task diff "$TASK" --format json > /tmp/diff.json 2>&1
bar "N27: --format json carries the workflow"          "grep -q 'workflow' /tmp/diff.json"
bar "N27: --format json carries the intent"            "grep -q 'intent' /tmp/diff.json"

say "6 · N15 (EXPECTED RED) — a --task read of an address that resolves nowhere"
step jigc doc show "adr:no-such-decision" --task "$TASK"
MISS="$(jigc doc show 'adr:no-such-decision' --task "$TASK" 2>&1)"
BARE="$(jigc doc show 'adr:no-such-decision' 2>&1)"
bar "N15: the task-scoped miss is NOT byte-identical to the task-less one" \
    "[ \"\$MISS\" != \"\$BARE\" ]"
bar "N15: the task-scoped miss does not claim the address names no COMMITTED doc" \
    "! printf '%s' \"\$MISS\" | grep -qi 'committed'"
bar "N15: its route keeps --task rather than dropping it" \
    "! printf '%s' \"\$MISS\" | grep -E 'route:' | grep -q 'doc show' || printf '%s' \"\$MISS\" | grep -E 'route:' | grep -q -- '--task'"

say "7 · nothing above destroyed the task it was reading"
bar "the task is still open"                 "jigc task list | grep -q '$TASK'"
bar "its staged doc is still there"          "test -f '.jigc/tasks/$TASK/docs/${ADDR}.md'"
# Narrowed after the first run: `jigc config set` writes `.jigc/config/manifest.yaml`
# and SAYS it leaves it uncommitted, so a bare "tree is clean" bar fails on the arm's own
# setup rather than on anything the section drove. What this bar is for is that reading a
# task did not move a TRACKED file.
bar "no tracked file was modified by reading" "test -z \"\$(git status --porcelain=v1 --untracked-files=no)\""

echo
if [ "$FAIL" -eq 0 ]; then echo "ARM 22 PASS"; else echo "ARM 22 RED — see the N15/N27 bars, which are MEASUREMENTS (expected red), and any others"; fi
exit 0
