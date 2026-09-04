#!/usr/bin/env bash
# 16-pinned-contracts.sh — protocol.md §5 arm 16 (M49 Increment 8: N1, N2, D1 and the
# `--task` gate route).
#
# WHICH SET THIS ARM ITERATES: the **`DOCTYPE_DOORS` address rows** — the code-side
# registry `pub const DOCTYPE_DOORS` in `crates/cli/src/cli.rs`, the one
# `flow50_acceptance.rs::the_pinned_contracts_answer_in_the_shape_a_driver_consumes`
# derives its N1 cells from. This arm drives a STATED SUBSET and says which:
#
#   driven here    the six item-addressing WRITE doors — the `doc` rows taking
#                  `DoctypeArg::Address`, minus the read verb (`show`), minus `doc rename`
#                  (its address names a document identity, so a nested-section hop is not
#                  a miss it can have): `add-item` · `remove-item` · `retitle-item` ·
#                  `set-field --value` · `set-field --unset` · `set-slot`. `set-field` is
#                  one row driven in both shapes because on rc.12 the `--unset` shape
#                  answered a fifth code of its own.
#   not driven     the Bare rows and the remaining Address rows (`migrate`, `relocate`,
#                  `rename`, `doc create/author/schema/list/show`, `task bind`) — those
#                  are the unknown-DOCTYPE axis, fenced by
#                  `crates/cli/tests/unknown_doctype_axis.rs`, not the nested-section miss.
#
# Cells:
#   N1   one nested-section hop (`<release>/bogus/xyz`) at all six doors answers
#        `write.unknown-section`, the typed address verbatim in `key.target`, and a
#        `jigc doc schema` route — rc.12 answered FOUR different codes across them
#        (the distinct-code count is MEASURED below, not assumed)
#   N2   `doc show --format json` carries a top-level INTEGER `schema-version` — staged
#        via `--task`, and committed after finalize; `commit:<id>` serves `null`; `fields`
#        stays all-strings; a `#section` slice gains nothing; `doc list` has no such key
#   D1   a located conformance finding's agent text names the address AND a line
#   gate route   with TWO open tasks, a `required-slot-present` finding at `task validate`
#        emits a `jigc doc set-slot …` route carrying `--task <id>`; run verbatim, it lands
#
# Pass condition, stated before the run: every bar OK. Codes, targets and routes are read
# off the JSON envelope of each refusal; the gate route is run as the bytes the binary
# printed, with prose on stdin as its own `--from-file -` asks.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
# cap: like step, but the combined output is ALSO kept in a file for the bars to read and
# the exit code lands in $RC — a bar asserts on what was captured, never on a guessed string.
W=$(mktemp -d /tmp/arm16.XXXXXX)
cap()  { local f="$1"; shift; printf '\n$ %s\n' "$*"; "$@" > "$f" 2>&1; RC=$?; cat "$f"; printf '[exit %s]\n' "$RC"; }
# jchk <json-file> <js-expr>: exit 0 iff the expression is truthy over the parsed file as
# `j`. The image carries node, not python3.
jchk() { node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8")); process.exit(eval(process.argv[2])?0:1)' "$1" "$2"; }
# route_cmd <json-file> <code>: the backticked command inside that finding's route.
route_cmd() { node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8")); const f=(j.findings||[]).find(f=>f.code===process.argv[2]); const m=((f&&f.route)||"").match(/`([^`]+)`/); process.stdout.write(m?m[1]:"")' "$1" "$2"; }

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

# ---------------------------------------------------------------------------
say "mint · one record-change task, the changelog singleton and one release"
step jigc start --workflow record-change "record the probe release"
T="$(newtask)"; echo "  T=$T"
cap "$W/create.txt" jigc doc create changelog --title Changelog --task "$T"
cap "$W/rel.txt" jigc doc add-item 'changelog:changelog#releases' --title '1.0.0' --task "$T"
REL="$(tr -d '\n' < "$W/rel.txt")"; echo "  REL=$REL"
bar "the release address the binary emitted is the one this arm hops from" \
    "test '$REL' = 'changelog:changelog#releases/1-0-0'"

# ---------------------------------------------------------------------------
say "N1 · one nested-section hop, ONE code, at every item-addressing write door"
HOP="$REL/bogus"; LEAF="$HOP/xyz"
printf 'prose\n' > "$W/prose.md"
# n1 <label> <typed-address> <doc-verb argv…>: drives one door and bars its envelope.
n1() { local label="$1" addr="$2"; shift 2
  local f="$W/n1-$label.json"
  cap "$f" jigc doc "$@" --task "$T" --format json
  bar "$label · blocks" "test $RC -ne 0"
  bar "$label · key.code is write.unknown-section" "jchk $f 'j.findings[0].key.code===\"write.unknown-section\"'"
  bar "$label · key.target is the address typed, verbatim" "jchk $f 'j.findings[0].key.target===\"$addr\"'"
  bar "$label · the route names jigc doc schema changelog" "jchk $f '/jigc doc schema changelog/.test(j.findings[0].route||\"\")'"
}
n1 set-slot         "$LEAF" set-slot     "$LEAF" --from-file "$W/prose.md"
n1 set-field--value "$LEAF" set-field    "$LEAF" --value v
n1 set-field--unset "$LEAF" set-field    "$LEAF" --unset
n1 remove-item      "$LEAF" remove-item  "$LEAF"
n1 retitle-item     "$LEAF" retitle-item "$LEAF" --title Y
# `add-item`'s destination IS the undeclared segment; every other verb addresses a leaf beneath it.
n1 add-item         "$HOP"  add-item     "$HOP"  --title X
echo
echo "  MEASURED · distinct finding codes across the six doors (rc.12: 4; the contract: 1):"
node -e '
const fs=require("fs"),dir=process.argv[1];const codes=new Set();
for(const f of fs.readdirSync(dir).filter(f=>f.startsWith("n1-"))){try{codes.add(JSON.parse(fs.readFileSync(dir+"/"+f,"utf8")).findings[0].key.code)}catch{codes.add("<unparsable "+f+">")}}
console.log("           "+codes.size+" → "+[...codes].join(", "))' "$W" | tee "$W/codes.txt"
bar "one code over the whole set" "grep -q '^           1 → write.unknown-section$' $W/codes.txt"

# ---------------------------------------------------------------------------
say "N2 · the doc's own stamp as a top-level integer — staged"
cap "$W/show-staged.json" jigc doc show changelog:changelog --task "$T" --format json
bar "schema-version is a top-level INTEGER on the staged serve" \
    "jchk $W/show-staged.json 'Number.isInteger(j[\"schema-version\"])'"
bar "…equal to the string in fields, which stays all-strings" \
    "jchk $W/show-staged.json 'String(j[\"schema-version\"])===j.fields[\"schema-version\"] && Object.values(j.fields).every(v=>typeof v===\"string\")'"
bar "…and the serve says which task staged it" "jchk $W/show-staged.json 'j.staged===\"$T\"'"
cap "$W/schema.json" jigc doc schema changelog --format json
bar "…and equal to doc schema's number — the upgrade check a driver automates needs no cast" \
    "node -e 'const r=f=>JSON.parse(require(\"fs\").readFileSync(f,\"utf8\"));process.exit(r(\"$W/show-staged.json\")[\"schema-version\"]===r(\"$W/schema.json\")[\"schema-version\"]?0:1)'"
cap "$W/show-commit.json" jigc doc show "commit:$T" --task "$T" --format json
bar "the transient commit doc serves schema-version: null — the key present, the value null" \
    "jchk $W/show-commit.json '\"schema-version\" in j && j[\"schema-version\"]===null'"
cap "$W/show-slice.json" jigc doc show 'changelog:changelog#releases' --task "$T" --format json
bar "a #section slice is the section's value (an item array) and gains no stamp" \
    "jchk $W/show-slice.json 'Array.isArray(j)' && ! grep -q 'schema-version' $W/show-slice.json"
cap "$W/list.json" jigc doc list --task "$T" --format json
bar "doc list carries no schema-version key anywhere" \
    "jchk $W/list.json 'Array.isArray(j.docs)' && ! grep -q 'schema-version' $W/list.json"

# ---------------------------------------------------------------------------
say "D1 · a located finding says WHERE — address and line — on the agent-text surface"
STAGED=".jigc/tasks/$T/docs/changelog:changelog.md"
cp "$STAGED" "$W/good.md"
# The plant: an undeclared field bullet in the first release's field group — the parse
# diagnostic that the located message is the whole repair for (an enumerated route
# exemption, D5), so its text has to carry the address and the line or it is a dead end.
node -e 'const fs=require("fs"),f=process.argv[1];fs.writeFileSync(f,fs.readFileSync(f,"utf8").replace("- date:","- bogusfield: x\n- date:"))' "$STAGED"
step grep -n 'bogusfield' "$STAGED"
cap "$W/d1.txt" jigc task validate "$T"
bar "task validate blocks" "test $RC -ne 0"
bar "the parse diagnostic conformance.unknown-field reaches the agent text" "grep -q 'conformance.unknown-field' $W/d1.txt"
bar "…carrying the address it located" "grep -q 'at: $REL/bogusfield' $W/d1.txt"
bar "…and the line it read it at" "grep -q 'at: $REL/bogusfield · line [0-9]' $W/d1.txt"
cap "$W/d1.json" jigc task validate "$T" --format json
bar "…and in the envelope that exempt finding carries route: null — the located message IS the repair" \
    "jchk $W/d1.json 'j.findings.some(f=>f.code===\"conformance.unknown-field\"&&f.route===null&&f.location.address===\"$REL/bogusfield\"&&Number.isInteger(f.location.line))'"
cp "$W/good.md" "$STAGED"
cap "$W/d1-restored.json" jigc task validate "$T" --format json
bar "restoring the bytes clears it" "jchk $W/d1-restored.json '!j.findings.some(f=>f.code===\"conformance.unknown-field\")'"

# ---------------------------------------------------------------------------
say "the gate route · with TWO open tasks, a required-slot-present route carries --task"
jigc start --workflow record-decision "a second open task" > "$W/second.txt" 2>&1
T2="$(awk '/^task minted: /{print $3}' "$W/second.txt")"; echo "  T2=$T2"
step jigc task list
bar "two tasks are open — the single-active-task default can no longer resolve" \
    "test -n '$T2' && test \"\$(jigc task list | grep -c '^  [a-z0-9]')\" -eq 2"
cap "$W/gate.json" jigc task validate "$T" --format json
bar "task validate on the first task carries required-slot-present on its commit summary" \
    "test $RC -ne 0 && jchk $W/gate.json 'j.findings.some(f=>f.code===\"schema-conformance.required-slot-present\"&&f.location.address===\"commit:$T#summary\")'"
GROUTE="$(node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));const f=j.findings.find(f=>f.code==="schema-conformance.required-slot-present"&&f.location.address==="commit:"+process.argv[2]+"#summary");const m=((f&&f.route)||"").match(/`([^`]+)`/);process.stdout.write(m?m[1]:"")' "$W/gate.json" "$T")"
printf '%s' "$GROUTE" > "$W/gate.cmd"; echo "  ROUTE=$GROUTE"
bar "…the route is a jigc doc set-slot at that address" "grep -q '^jigc doc set-slot commit:$T#summary' $W/gate.cmd"
bar "…and it carries --task $T" "grep -q -- '--task $T' $W/gate.cmd"
cap "$W/gate-run.txt" sh -c "printf 'record the probe release\n' | $GROUTE"
bar "run VERBATIM (prose on stdin, as its --from-file - asks) it succeeds" \
    "test $RC -eq 0 && grep -q 'set slot commit:$T#summary' $W/gate-run.txt"
cap "$W/gate2.json" jigc task validate "$T" --format json
bar "…and the finding is gone at that target" \
    "jchk $W/gate2.json '!j.findings.some(f=>f.code===\"schema-conformance.required-slot-present\"&&f.location.address===\"commit:$T#summary\")'"

# ---------------------------------------------------------------------------
say "N2 · committed — the same integer after finalize"
jigc doc set-field "commit:$T#type" --value docs --task "$T" >/dev/null 2>&1
jigc doc set-field "commit:$T#scope" --value changelog --task "$T" >/dev/null 2>&1
cap "$W/fin.txt" jigc task finalize "$T"
bar "the finalize lands, promoting CHANGELOG.md" \
    "test $RC -eq 0 && grep -q 'finalized' $W/fin.txt && grep -q 'promoted CHANGELOG.md' $W/fin.txt"
cap "$W/show-committed.json" jigc doc show changelog:changelog --format json
bar "the committed serve carries the integer too, and no staged key" \
    "jchk $W/show-committed.json 'Number.isInteger(j[\"schema-version\"]) && !(\"staged\" in j)'"
bar "…the same number the staged serve carried" \
    "node -e 'const r=f=>JSON.parse(require(\"fs\").readFileSync(f,\"utf8\"));process.exit(r(\"$W/show-staged.json\")[\"schema-version\"]===r(\"$W/show-committed.json\")[\"schema-version\"]?0:1)'"
# Stdout only: with the second task still open, the committed listing rides a
# stderr "note: docs are also staged in open task …" — stream discipline the contract
# pins (stdout is the JSON and nothing else), and exactly the line that broke this
# cell's first run when both streams were folded into one file.
printf '\n$ jigc doc list --format json   (stdout → list2.json; stderr shown below)\n'
jigc doc list --format json > "$W/list2.json" 2> "$W/list2.err"; RC=$?
cat "$W/list2.json"; cat "$W/list2.err"; printf '[exit %s]\n' "$RC"
bar "the committed doc list has no schema-version key either" \
    "jchk $W/list2.json 'j.docs.some(d=>d.id===\"changelog:changelog\"&&d.state===\"managed\")' && ! grep -q 'schema-version' $W/list2.json"

say "SUMMARY"
echo "  six of DOCTYPE_DOORS' Address rows driven over the nested-section miss (the write"
echo "  doors); the rest are the unknown-doctype axis, fenced elsewhere. N2 driven staged"
echo "  and committed; D1 on a planted parse diagnostic; the gate route run verbatim with"
echo "  two tasks open."
if [ "$FAIL" -eq 0 ]; then echo "ARM 16 PASS"; else echo "ARM 16 FAIL"; fi
exit "$FAIL"
