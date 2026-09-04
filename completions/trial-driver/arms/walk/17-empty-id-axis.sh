#!/usr/bin/env bash
# 17-empty-id-axis.sh — protocol.md §5 arm 17 (the T1-a class, routed to M50).
#
# THIS ARM IS EXPECTED RED ON 1.0.0-rc.13. It is a MEASUREMENT for M50, not a
# regression check: pre-trial-findings.md PT-1 already records that on the release
# binary `jigc task validate ""` prints "no findings — the task validates clean" at
# exit 0 (a false green) and `jigc task discard ""` acks "discarded task " at exit 0.
# Those cells are expected to FAIL the bars below; the bars stay, because a table in
# which the known-bad cells are green would be the instrument lying. The arm's overall
# FAIL is the correct result. M50 receives the table, not this arm's verdict.
#
# WHICH SET THIS ARM ITERATES: the id-taking doors — every leaf verb whose positional
# is a task or milestone id, plus every leaf that takes `--task <id>`. The handover
# sized the class as "seven doors"; this arm DERIVES the set from the binary's own
# help (`jigc --help` · `jigc task --help` · `jigc milestone --help` · `jigc doc --help`
# · `jigc workflow --help` · `jigc start --help`, read on rc.13) and finds 25:
#
#   task-id positional   task validate · task diff · task finalize · task bind · task discard   (5)
#   task-id option       start --task · workflow <W> --task                                     (2)
#   task-id option       doc show · doc list · doc create · doc add-item · doc remove-item ·
#                        doc retitle-item · doc rename · doc set-field · doc set-slot · doc author (10)
#   milestone-id pos.    milestone execute · provision · join · finalize · discard ·
#                        list-tasks · add-task · add-from-spec                                  (8)
#
# It is a hand-enumerated set, said to be one; the ten `doc` write doors share one
# `--task` resolver by design but are driven individually, because "share a resolver"
# is a claim about the source and this arm measures the binary. `task finalize` is one
# door (its `--dry-run` is a flag on it, driven as the safer form).
#
# TWO COLUMNS: `""` (the empty id — the reported repro) and `no-such-task` (a plausible
# non-existent id — the control column: what the door says when the id is merely
# wrong tells us whether the empty cell is the SAME refusal or a different code path).
#
# ORDER IS LOAD-BEARING: `task discard` runs LAST in each column. A live task with
# staged prose is minted first, so the arm can ask afterwards whether it survived —
# `discard ""` addresses `.jigc/tasks/` + `""`, which is the tasks directory itself.
#
# PASS CONDITION, stated before the run, per cell:
#   1. the door does not ACK SUCCESS — exit 0 on a task/milestone that does not exist;
#   2. the door does not PANIC — exit 101;
#   3. the refusal carries a CODE (`blocking · <family>.<check>` or a clap `error:`);
#   4. the refusal carries a ROUTE (a `route:` line, or a runnable `jigc …` in its text);
# and, once per column: the live task and its staged doc are untouched afterwards.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
addr() { grep -oE "^$1:[a-z0-9-]+" | head -1; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

say "FIXTURE · one live task holding a staged doc, so destruction is measurable"
jigc start --workflow record-decision "keep one task live" >/dev/null 2>&1
LIVE="$(newtask)"
A="$(jigc doc create adr --title 'Keep the oldest sample' --task "$LIVE" 2>&1 | addr adr)"
printf 'Staged prose no commit holds.\n' | jigc doc set-slot "$A#context" --from-file - --task "$LIVE" >/dev/null 2>&1
step jigc task list
step jigc doc list --task "$LIVE"
bar "the live task exists"                 "test -n \"$LIVE\" && test -d .jigc/tasks/$LIVE"
bar "…and stages a doc"                    "jigc doc list --task $LIVE | grep -q 'adr:'"

# ---------------------------------------------------------------------------------
# The drive. Each cell: run the door, capture combined output + exit, classify, record
# one JSON line. Classification is on the captured bytes, never on a guessed string.
# ---------------------------------------------------------------------------------
ROWS="$(mktemp "${TMPDIR:-/tmp}/arm17-rows.XXXXXX")"
cell() { # <column-label> <door-label> <argv…>
  local col="$1" door="$2"; shift 2
  local out rc first code route ack panic
  printf '\n$ %s\n' "$*"
  out="$("$@" 2>&1)"; rc=$?
  printf '%s\n[exit %s]\n' "$out" "$rc"
  first="$(printf '%s' "$out" | head -1)"
  if printf '%s' "$out" | grep -qE '^(blocking|advisory).*· [a-z-]+\.[a-z-]+|^error:'; then code=1; else code=0; fi
  if printf '%s' "$out" | grep -qE 'route:|`jigc [a-z]'; then route=1; else route=0; fi
  if [ "$rc" -eq 0 ]; then ack=1; else ack=0; fi
  if [ "$rc" -eq 101 ]; then panic=1; else panic=0; fi
  node -e '
    const [col,door,rc,first,code,route,ack,panic]=process.argv.slice(1);
    console.log(JSON.stringify({col,door,rc:+rc,first,code:+code,route:+route,ack:+ack,panic:+panic}));
  ' "$col" "$door" "$rc" "$first" "$code" "$route" "$ack" "$panic" >> "$ROWS"
  bar "[$col] $door · refuses: no success ack, no panic (exit $rc)" "test $ack -eq 0 && test $panic -eq 0"
  bar "[$col] $door · the refusal carries a code AND a route"       "test $code -eq 1 && test $route -eq 1"
}

drive_column() { # <label> <id>
  local col="$1" id="$2"
  say "COLUMN · id=[$id]"
  cell "$col" "task validate"           jigc task validate "$id"
  cell "$col" "task diff"               jigc task diff "$id"
  cell "$col" "task finalize --dry-run" jigc task finalize "$id" --dry-run
  cell "$col" "task bind"               jigc task bind decision adr:nothing "$id"
  cell "$col" "start --task"            jigc start --task "$id"
  cell "$col" "workflow --task"         jigc workflow sub-task --task "$id"
  cell "$col" "doc show --task"         jigc doc show "$A" --task "$id"
  cell "$col" "doc list --task"         jigc doc list --task "$id"
  cell "$col" "doc create --task"       jigc doc create adr --title Probe --task "$id"
  cell "$col" "doc add-item --task"     jigc doc add-item 'spec:nothing#criteria' --title Probe --task "$id"
  cell "$col" "doc remove-item --task"  jigc doc remove-item 'spec:nothing#criteria/probe' --task "$id"
  cell "$col" "doc retitle-item --task" jigc doc retitle-item 'spec:nothing#criteria/probe' --title Probe --task "$id"
  cell "$col" "doc rename --task"       jigc doc rename "$A" --to 'Probe' --task "$id"
  cell "$col" "doc set-field --task"    jigc doc set-field "$A#header/status" --value accepted --task "$id"
  cell "$col" "doc set-slot --task"     sh -c "echo x | jigc doc set-slot '$A#context' --from-file - --task '$id'"
  cell "$col" "doc author --task"       sh -c "printf 'title: Probe\nsections: []\n' | jigc doc author adr --from-file - --task '$id'"
  cell "$col" "milestone execute"       jigc milestone execute "$id"
  cell "$col" "milestone provision"     jigc milestone provision "$id"
  cell "$col" "milestone join"          jigc milestone join "$id"
  cell "$col" "milestone finalize"      jigc milestone finalize "$id"
  cell "$col" "milestone list-tasks"    jigc milestone list-tasks "$id"
  cell "$col" "milestone add-task"      jigc milestone add-task "$id" "some intent"
  cell "$col" "milestone add-from-spec" jigc milestone add-from-spec "$id" spec:nothing
  cell "$col" "milestone discard"       jigc milestone discard "$id"
  # LAST, on purpose — see the header.
  cell "$col" "task discard"            jigc task discard "$id"
  say "AFTER the column · did the live task survive?"
  step jigc task list
  step ls -la .jigc/tasks
  bar "[$col] the live task \`$LIVE\` is still listed"          "jigc task list | grep -q \"^  $LIVE\""
  bar "[$col] …its working area is still on disk"               "test -d .jigc/tasks/$LIVE"
  bar "[$col] …and its staged doc is still readable"            "jigc doc show '$A#context' --task $LIVE | grep -q 'Staged prose'"
  echo "  OBSERVE  if the three bars above FAIL in the [$col] column: the last door of the column,"
  echo "           \`jigc task discard '$id'\`, took the LIVE task's working area with it — \`.jigc/tasks/\`"
  echo "           joined to an empty id is the tasks directory itself — and acked at exit 0."
  echo "           That is loss of staged prose no commit holds, on a non-destructive-looking path."
}

drive_column "nonexistent" "no-such-task"
drive_column "empty" ""

say "THE TABLE · what M50 receives (release binary $(jigc --version 2>&1))"
node -e '
const fs=require("fs");
const rows=fs.readFileSync(process.argv[1],"utf8").trim().split("\n").filter(Boolean).map(JSON.parse);
const doors=[...new Set(rows.map(r=>r.door))];
const by={}; for(const r of rows) by[r.col+"|"+r.door]=r;
const flag=r=>r?(r.panic?"PANIC":r.ack?"ACK(exit 0)":`exit ${r.rc}`)+(r.code?" code":" NOCODE")+(r.route?" route":" NOROUTE"):"(unrun)";
const pad=(s,n)=>String(s).padEnd(n);
console.log("  "+pad("door",26)+pad("id=\"\"",34)+"id=no-such-task");
for(const d of doors){
  const e=by["empty|"+d], n=by["nonexistent|"+d];
  console.log("  "+pad(d,26)+pad(flag(e),34)+flag(n));
}
const bad=rows.filter(r=>r.ack||r.panic||!r.code||!r.route);
console.log("");
console.log("  cells failing the pass condition: "+bad.length+" of "+rows.length);
for(const r of bad) console.log(`    [${r.col}] ${r.door}: ${flag(r)} — "${r.first.slice(0,110)}"`);
' "$ROWS"

say "SUMMARY"
echo "  25 id-taking doors derived from the binary's help × 2 columns; the table above is the"
echo "  deliverable. Expected RED on rc.13 (PT-1): the bars that FAIL are the measurement."
if [ "$FAIL" -eq 0 ]; then echo "ARM 17 PASS"; else echo "ARM 17 FAIL"; fi
exit "$FAIL"
