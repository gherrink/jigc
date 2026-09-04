#!/usr/bin/env bash
# 19-planning-record.sh — protocol.md §5 arm 19 (M49 Increment 9: the `planning-record`
# doctype — one required slot per plan-time gate, blocking `finalize` until every gate is
# answered).
#
# WHICH SET THIS ARM ITERATES: the **gate set, read from the shipped schema THROUGH the
# binary** — `jigc doc schema planning-record --format json`, every `kind: slot` section
# not marked optional — and NEVER hand-listed in this file. That is the derivation
# `flow50_acceptance.rs::the_planning_gates_block_the_finalize_until_they_are_filled`
# makes. The settle record fixed the count at 14 in one home
# (`implementation/milestone-planning-workflow.md`); this arm PRINTS the count it
# measured and bars it against that number, so a drift shows up as a red bar and a
# printed list, not as a re-typed enumeration that rots.
#
# Cells:
#   1  the gate set off the binary, with its count
#   2  `jigc start --workflow planning` composes the `{{schema:planning-record}}`
#      projection — the create is named, every gate the schema declares is named, the
#      blocking code is stated; the composed size in LINES is carried as an observation
#      (DECISIONS.md → 2026-08-31, Increment 9: the projection roughly tripled it)
#   3  the record is created through the gate role; every gate but the LAST one the
#      projection lists is filled
#   4  `task validate` AND `task finalize` both block on `schema-conformance.required-slot-present`
#      NAMING the unfilled gate at its own address, and commit nothing
#      (`git rev-list --count HEAD` unchanged, the file not on disk)
#   5  fill it; the IDENTICAL finalize lands one commit; the record is at
#      `docs/planning-records/<slug>.md` (not `completions/`); `doc show` reads it back
#      with `schema-version: 1`; `doc list` shows it managed there
#
# Pass condition, stated before the run: every bar OK. The held-out gate is whichever
# the projection lists LAST, so the arm cannot quietly pick the one gate it knows works.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
# cap: like step, but the combined output is ALSO kept in a file for the bars to read and
# the exit code lands in $RC — a bar asserts on what was captured, never on a guessed string.
W=$(mktemp -d /tmp/arm19.XXXXXX)
cap()  { local f="$1"; shift; printf '\n$ %s\n' "$*"; "$@" > "$f" 2>&1; RC=$?; cat "$f"; printf '[exit %s]\n' "$RC"; }
# jchk <json-file> <js-expr>: exit 0 iff the expression is truthy over the parsed file as
# `j`. The image carries node, not python3.
jchk() { node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8")); process.exit(eval(process.argv[2])?0:1)' "$1" "$2"; }

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

# ---------------------------------------------------------------------------
say "1 · the gate set, read off the binary"
cap "$W/schema.json" jigc doc schema planning-record --format json
GATES="$(node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8")); console.log(j.sections.filter(s=>s.kind==="slot"&&s.optional!==true).map(s=>s.id).join(" "))' "$W/schema.json")"
set -- $GATES; NGATES=$#; HELD="${*: -1}"
echo "  MEASURED · $NGATES required gate slots: $GATES"
echo "  held out (the LAST one the projection lists): $HELD"
bar "the projection yields at least one required slot" "test $NGATES -ge 1"
bar "…and the count is the settled 14 (milestone-planning-workflow.md's one home)" "test $NGATES -eq 14"
bar "planning-record reports schema-version 1 on doc schema" "jchk $W/schema.json 'j[\"schema-version\"]===1'"
bar "…and every gate is a required slot, none optional" \
    "jchk $W/schema.json 'j.sections.filter(s=>s.kind===\"slot\").every(s=>s.optional!==true)'"

# ---------------------------------------------------------------------------
say "2 · jigc start --workflow planning — the composed output renders the schema projection"
jigc start --workflow planning "plan the first milestone" > "$W/compose.txt" 2>&1; CRC=$?
T="$(newtask)"; echo "  T=$T"
LINES="$(wc -l < "$W/compose.txt" | tr -d ' ')"
echo "  MEASURED · composed planning workflow: $LINES lines"
echo "           (observation, not a bar — DECISIONS 2026-08-31 says the projection roughly tripled it)"
step sed -n '/^Run: `jigc doc create planning-record/,/^- `meta`/p' "$W/compose.txt"
bar "the compose minted a task" "test $CRC -eq 0 && test -n '$T'"
bar "it names the create: jigc doc create planning-record --title <TITLE> --task $T" \
    "grep -q 'jigc doc create planning-record --title <TITLE> --task $T' $W/compose.txt"
bar "it renders the planning-record schema (the {{schema:planning-record}} projection) and its batch payload" \
    "grep -q 'The .planning-record. schema' $W/compose.txt && grep -q 'jigc doc author planning-record --from-file - --task $T' $W/compose.txt"
MISSING=""
for g in "$@"; do grep -q -- "- \`$g\`: prose slot" "$W/compose.txt" || MISSING="$MISSING $g"; done
NMISS="$(echo $MISSING | wc -w | tr -d ' ')"
echo "  gates named in the composed output: $((NGATES - NMISS)) of $NGATES${MISSING:+ · missing:$MISSING}"
bar "every gate the schema declares is named in the composed output" "test -z '$MISSING'"
bar "…and it says the finalize BLOCKS on an unfilled gate, naming the code" \
    "grep -q 'schema-conformance.required-slot-present' $W/compose.txt"
bar "…and states the home: docs/planning-records/<slug>.md" "grep -q 'docs/planning-records/<slug>.md' $W/compose.txt"

# ---------------------------------------------------------------------------
say "3 · create the record through the gate role, fill every gate but the held-out one"
cap "$W/create.txt" jigc doc create planning-record --title 'M1 the first milestone' --task "$T"
ADDR="$(tr -d '\n' < "$W/create.txt")"; SLUG="${ADDR#planning-record:}"
echo "  ADDR=$ADDR  SLUG=$SLUG"
bar "the create is accepted at the gate role and emits the identity" \
    "test $RC -eq 0 && grep -q '^planning-record:' $W/create.txt"
printf 'Answered for the probe wave.\n' > "$W/answer.md"
FILLED=0
for g in "$@"; do
  [ "$g" = "$HELD" ] && continue
  if jigc doc set-slot "$ADDR#$g" --from-file "$W/answer.md" --task "$T" >/dev/null 2>&1; then
    FILLED=$((FILLED + 1))
  else
    echo "  set-slot FAILED at $g"
  fi
done
echo "  filled $FILLED of $NGATES gates; $HELD left empty"
bar "every gate but the held-out one accepted its prose" "test $FILLED -eq $((NGATES - 1))"
# The commit doc is filled so that the ONLY blocking finding left is the gate's — the
# arm is about the record, and a commit-doc block would mask what it claims.
jigc doc set-field "commit:$T#type" --value docs --task "$T" >/dev/null 2>&1
jigc doc set-field "commit:$T#scope" --value planning --task "$T" >/dev/null 2>&1
printf 'record the M1 planning gates\n' > "$W/summary.md"
jigc doc set-slot "commit:$T#summary" --from-file "$W/summary.md" --task "$T" >/dev/null 2>&1

# ---------------------------------------------------------------------------
say "4 · task validate and task finalize both block, NAMING the unfilled gate, and commit nothing"
cap "$W/validate.txt" jigc task validate "$T"; VRC=$RC
cap "$W/validate.json" jigc task validate "$T" --format json
bar "task validate exits non-zero" "test $VRC -ne 0"
bar "…with schema-conformance.required-slot-present at $ADDR#$HELD" \
    "jchk $W/validate.json 'j.findings.some(f=>f.code===\"schema-conformance.required-slot-present\"&&f.location.address===\"$ADDR#$HELD\")'"
bar "…and it is the ONLY blocking finding — every other gate is answered" \
    "jchk $W/validate.json 'j.findings.filter(f=>f.severity===\"blocking\").length===1'"
bar "…and the text names the gate" "grep -q 'required slot in section .$HELD. is empty' $W/validate.txt"
bar "…routing at the set-slot that fills it, with --task" \
    "grep -q 'jigc doc set-slot $ADDR#$HELD --task $T --from-file -' $W/validate.txt"
N0="$(git rev-list --count HEAD)"
cap "$W/finalize.txt" jigc task finalize "$T"; FRC=$RC
N1="$(git rev-list --count HEAD)"
bar "task finalize exits non-zero" "test $FRC -ne 0"
bar "…naming the same code and the same gate" \
    "grep -q 'schema-conformance.required-slot-present' $W/finalize.txt && grep -q '$ADDR#$HELD' $W/finalize.txt"
bar "…and commits nothing: git rev-list --count HEAD $N0 -> $N1" "test $N0 -eq $N1"
bar "…and the record is not on disk — still staged, not promoted" "test ! -e docs/planning-records/$SLUG.md"
bar "…and the task is still open" "jigc task list | grep -q '^  $T '"

# ---------------------------------------------------------------------------
say "5 · fill the last gate; the IDENTICAL finalize lands"
cap "$W/last.txt" jigc doc set-slot "$ADDR#$HELD" --from-file "$W/answer.md" --task "$T"
bar "the held-out gate accepts its prose" "test $RC -eq 0"
cap "$W/finalize2.txt" jigc task finalize "$T"; F2=$RC
N2="$(git rev-list --count HEAD)"
bar "the same finalize now lands at exit 0" "test $F2 -eq 0 && grep -q 'finalized' $W/finalize2.txt"
bar "…as exactly one commit: $N1 -> $N2" "test $N2 -eq $((N1 + 1))"
bar "the record is at docs/planning-records/$SLUG.md" \
    "test -f docs/planning-records/$SLUG.md && grep -q 'promoted docs/planning-records/$SLUG.md' $W/finalize2.txt"
bar "…not under completions/" "test ! -e completions/$SLUG.md && ! git show --stat --format= HEAD | grep -q 'completions/'"
step git show --stat --format=%s HEAD
cap "$W/show.json" jigc doc show "$ADDR" --format json
bar "doc show reads it back committed with schema-version: 1 — a top-level integer, no staged key" \
    "jchk $W/show.json 'j[\"schema-version\"]===1 && j.fields[\"schema-version\"]===\"1\" && !(\"staged\" in j)'"
bar "…every gate the schema declares holding its prose" \
    "jchk $W/show.json 'JSON.parse(require(\"fs\").readFileSync(\"$W/schema.json\",\"utf8\")).sections.filter(s=>s.kind===\"slot\"&&s.optional!==true).every(s=>j.sections[s.id]===\"Answered for the probe wave.\")'"
step head -8 "docs/planning-records/$SLUG.md"
bar "…and the file carries the stamp in its front-matter" "grep -q '^schema-version: 1$' docs/planning-records/$SLUG.md"
cap "$W/list.json" jigc doc list planning-record --format json
bar "doc list planning-record lists it managed at that path" \
    "jchk $W/list.json 'j.docs.some(d=>d.id===\"$ADDR\"&&d.state===\"managed\"&&d.path===\"docs/planning-records/$SLUG.md\")'"
cap "$W/validate-store.txt" jigc validate
bar "the committed store validates clean" "test $RC -eq 0 && grep -q 'validates clean' $W/validate-store.txt"

say "SUMMARY"
echo "  the gate set was read off jigc doc schema planning-record ($NGATES gates), the last"
echo "  one held out; validate and finalize both named it and committed nothing; filled, the"
echo "  identical finalize landed docs/planning-records/$SLUG.md at schema-version 1."
echo "  composed planning workflow: $LINES lines (observation)."
if [ "$FAIL" -eq 0 ]; then echo "ARM 19 PASS"; else echo "ARM 19 FAIL"; fi
exit "$FAIL"
