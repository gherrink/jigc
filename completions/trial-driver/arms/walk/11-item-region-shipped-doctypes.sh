#!/usr/bin/env bash
# 11-item-region-shipped-doctypes.sh — protocol.md §5 arm 11 (M49 Increment 1, the
# item-region class, plus the M49 completion audit's fix 2 — `1cf9295`).
#
# WHICH SET THIS ARM ITERATES: a **derivation over the shipped doctypes' item blocks**,
# read off the binary rather than remembered — section 0 re-runs the census
# (`jigc doc schema <t> --format json` for every doctype `describe --doctypes` lists)
# and prints it, so the rows this arm drives are checked against the packs THIS binary
# ships:
#
#   changelog#releases               string id · optional settable `link` · nested `changes`
#   changelog#releases/<id>/changes  ENUM id · single slot `notes`
#   changelog#unreleased-changes     ENUM id
#   spec#criteria                    string id · single slot `statement` · optional settable `maps-to-test`
#   roadmap#milestones               the only MULTI-SLOT block either pack ships (`proves`,
#                                    `decomposition`) — and it carries no settable field
#
# The eighteen-cell manufactured shape space `{slotless, single-slot, multi-slot} ×
# {nested, ¬nested} × {insert, update, unset}` (widened by slot position and payload
# shape after the audit) is DECLARED TEST-FENCED — `crates/cli/tests/item_region_shape_space.rs`
# BUILDS those shapes precisely because the shipped registry populates 2 of 18 — and is
# NOT re-driven here. This arm drives what a shipped doctype can reach:
#
#   (a) `set-field <item>/<field> --unset` on an ABSENT optional field acks the no-op
#       (`already_absent: true`, exit 0) — rc.12 refused it `write.not-present`
#   (b) two hand-planted `- <field>:` bullets in one staged item → `jigc task validate`
#       reports `conformance.duplicate-field` and EXITS NON-ZERO — rc.12 exited 0 while
#       `doc show` returned the first, stale value (the wave's silent-corruption centrepiece)
#   (c) slot prose carrying a heading at the very depth `write.slot-heading-depth` names
#       as free lands intact — audit fix 2: the message said `#####` was free and the next
#       write refused it. Driven at the faces of that fix a shipped doctype reaches: the
#       nested single-slot (`changes/<id>/notes`), the multi-slot NON-LAST slot
#       (`roadmap#milestones/<id>/proves`, updated over heading-bearing prose with the next
#       sub-label intact), and the writer's half on `spec#criteria` (set-field insert +
#       update over heading-bearing prose lands ONE bullet, never two). The slot∧nested
#       face — the audit's silent-loss cell — is reachable by NO shipped block (section 0
#       measures that) and stays with the fenced suite.
#   (d) `add-item --slug` decouples id from title; is refused `write.identity-change` where
#       the block's id-from is an enum; a colliding slug draws `write.already-present`
#       whose route is the `--slug <id>-N` mint, run here VERBATIM
#
# Pass condition, stated before the run: every bar below OK. The depth in (c) is READ
# OFF the rejection message, never written into this file — a constant here would
# re-enact the statement==constant failure the audit found in the acceptance suite.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
# cap: like step, but the combined output is ALSO kept in a file for the bars to read and
# the exit code lands in $RC — a bar asserts on what was captured, never on a string its
# author guessed.
W=$(mktemp -d /tmp/arm11.XXXXXX)
cap()  { local f="$1"; shift; printf '\n$ %s\n' "$*"; "$@" > "$f" 2>&1; RC=$?; cat "$f"; printf '[exit %s]\n' "$RC"; }
# jchk <json-file> <js-expr>: exit 0 iff the expression is truthy over the parsed file as
# `j`. The image carries node, not python3.
jchk() { node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8")); process.exit(eval(process.argv[2])?0:1)' "$1" "$2"; }
# route_cmd <json-file> <code>: the backticked command inside that finding's route.
route_cmd() { node -e 'const j=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8")); const f=(j.findings||[]).find(f=>f.code===process.argv[2]); const m=((f&&f.route)||"").match(/`([^`]+)`/); process.stdout.write(m?m[1]:"")' "$1" "$2"; }
# free_depth <text-file>: the depth a `write.slot-heading-depth` rejection names as free.
free_depth() { grep -o '`#*` is the shallowest' "$1" | head -1 | tr -d '`' | awk '{print $1}'; }

jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
jigc --version

# ---------------------------------------------------------------------------
say "0 · the set, re-derived from this binary — every shipped item block"
for t in $(jigc describe --doctypes 2>/dev/null | awk '/^[a-z][a-z0-9-]* is /{print $1}'); do
  jigc doc schema "$t" --format json > "$W/schema-$t.json" 2>/dev/null
done
node -e '
const fs=require("fs"), dir=process.argv[1], rows=[];
for (const f of fs.readdirSync(dir).filter(f=>f.startsWith("schema-"))) {
  let s; try { s=JSON.parse(fs.readFileSync(dir+"/"+f,"utf8")); } catch { continue; }
  const walk=(secs,prefix)=>{ for (const sec of secs||[]) { const it=sec.item; if(!it) continue;
    const idf=(it.fields||[]).find(x=>x["write-key"]==="--title");
    const settable=(it.fields||[]).filter(x=>x["set-field"]&&!x.required).map(x=>x.id);
    rows.push(`  ${s.type}#${prefix}${sec.id}  slots=${(it.slots||[]).length} nested=${(it.nested||[]).length} id-from=${idf?idf.type:"?"} optional-settable=[${settable}]`);
    for (const n of it.nested||[]) walk([n], prefix+sec.id+"/<id>/"); } };
  walk(s.sections,"");
}
console.log(rows.sort().join("\n"));
const multi=rows.filter(r=>/slots=[2-9]/.test(r)), slotnested=rows.filter(r=>/slots=[1-9]/.test(r)&&/nested=[1-9]/.test(r));
console.log(`  MEASURED · ${rows.length} item blocks · multi-slot: ${multi.length} · slot∧nested: ${slotnested.length}`);
' "$W" | tee "$W/census.txt"
bar "the census reaches the five blocks this arm drives" \
    "grep -q 'changelog#releases ' $W/census.txt && grep -q 'changelog#releases/<id>/changes' $W/census.txt && grep -q 'changelog#unreleased-changes' $W/census.txt && grep -q 'spec#criteria' $W/census.txt && grep -q 'roadmap#milestones' $W/census.txt"
bar "…and no shipped block is slot∧nested — the audit's silent-loss face is fenced, not reachable here" \
    "grep -q 'slot∧nested: 0' $W/census.txt"

# ---------------------------------------------------------------------------
say "mint · one task per create-gate the drive needs (three open tasks; every write is --task-explicit)"
step jigc start --workflow record-change "record the probe release"
T="$(newtask)"; echo "  T=$T"
jigc start --workflow plan "bound the ingest queue" > "$W/plan.txt" 2>&1
TS="$(awk '/^task minted: /{print $3}' "$W/plan.txt")"; echo "  TS=$TS  (plan — the spec create-gate)"
jigc start --workflow planning "plan the first milestone" > "$W/planning.txt" 2>&1
TR="$(awk '/^task minted: /{print $3}' "$W/planning.txt")"; echo "  TR=$TR  (planning — the roadmap create-gate)"
bar "three tasks minted" "test -n '$T' && test -n '$TS' && test -n '$TR'"

# ---------------------------------------------------------------------------
say "(d) · --slug decouples id from title; refused on an enum id-from; the collision route runs"
cap "$W/create.txt" jigc doc create changelog --title Changelog --task "$T"
cap "$W/rel.txt" jigc doc add-item 'changelog:changelog#releases' --title '1.0.0' --task "$T"
REL="$(tr -d '\n' < "$W/rel.txt")"; echo "  REL=$REL"
bar "the release address the binary emitted is the one this arm drives" \
    "test '$REL' = 'changelog:changelog#releases/1-0-0'"
cap "$W/slug.txt" jigc doc add-item 'changelog:changelog#releases' --title '2.0.0' --slug my-slug --task "$T"
bar "--slug my-slug mints the item under my-slug, not under the title's 2-0-0" \
    "test $RC -eq 0 && grep -qx 'changelog:changelog#releases/my-slug' $W/slug.txt"
cap "$W/enum.json" jigc doc add-item 'changelog:changelog#unreleased-changes' --title added --slug my-slug --task "$T" --format json
bar "on an enum id-from block the same --slug is refused write.identity-change" \
    "test $RC -ne 0 && jchk $W/enum.json 'j.findings[0].key.code===\"write.identity-change\"'"
bar "…whose route mints the member itself (the heading IS the member)" \
    "jchk $W/enum.json '/add-item changelog:changelog#unreleased-changes --title added/.test(j.findings[0].route||\"\")'"
cap "$W/collide.json" jigc doc add-item 'changelog:changelog#releases' --title 'Two point oh' --slug my-slug --task "$T" --format json
bar "a colliding --slug draws write.already-present" \
    "test $RC -ne 0 && jchk $W/collide.json 'j.findings[0].key.code===\"write.already-present\"'"
ROUTE="$(route_cmd "$W/collide.json" write.already-present)"; printf '%s' "$ROUTE" > "$W/route.cmd"
echo "  ROUTE=$ROUTE"
bar "…whose route is the mechanical --slug <id>-N mint, argv-complete with --task" \
    "grep -q -- '--slug my-slug-2' $W/route.cmd && grep -q -- '--task $T' $W/route.cmd"
cap "$W/route.txt" sh -c "$ROUTE"
bar "…and run VERBATIM it succeeds, landing my-slug-2" \
    "test $RC -eq 0 && grep -qx 'changelog:changelog#releases/my-slug-2' $W/route.txt"

# ---------------------------------------------------------------------------
say "(a) · --unset of an ABSENT optional field acks the no-op — rc.12 refused it write.not-present"
cap "$W/unset.json" jigc doc set-field "$REL/link" --unset --task "$T" --format json
bar "exit 0 with already_absent: true at changelog#releases/<id>/link" \
    "test $RC -eq 0 && jchk $W/unset.json 'j.already_absent===true && j.unset===true && j.target.leaf===\"link\"'"
cap "$W/unset.txt" jigc doc set-field "$REL/link" --unset --task "$T"
bar "…and the agent text says so rather than blocking" "test $RC -eq 0 && grep -q 'already absent' $W/unset.txt"
cap "$W/spec.txt" jigc doc create spec --title 'Bounded ingest' --task "$TS"
cap "$W/crit.txt" jigc doc add-item 'spec:bounded-ingest#criteria' --title 'Queue is bounded' --task "$TS"
CRIT="$(tr -d '\n' < "$W/crit.txt")"; echo "  CRIT=$CRIT"
cap "$W/unset2.json" jigc doc set-field "$CRIT/maps-to-test" --unset --task "$TS" --format json
bar "the same at spec#criteria/<id>/maps-to-test — the other optional settable item field" \
    "test $RC -eq 0 && jchk $W/unset2.json 'j.already_absent===true'"

# ---------------------------------------------------------------------------
say "(c) · a heading at the depth the rejection itself names lands intact — audit fix 2"
printf 'Intro\n\n# Too shallow\n\nmore\n' > "$W/shallow.md"

echo "  -- face 1: the nested single-slot, changelog#releases/<id>/changes/<id>/notes"
cap "$W/nested.txt" jigc doc add-item "$REL/changes" --title added --task "$T"
cap "$W/depth1.txt" jigc doc set-slot "$REL/changes/added/notes" --from-file "$W/shallow.md" --task "$T"
D1="$(free_depth "$W/depth1.txt")"; echo "  prescribed free depth at changes/added/notes: '$D1'"
bar "a '#' heading is refused write.slot-heading-depth, naming the free depth" \
    "test $RC -ne 0 && grep -q 'write.slot-heading-depth' $W/depth1.txt && test -n '$D1'"
printf 'Intro\n\n%s Deep enough\n\nmore\n' "$D1" > "$W/notes.md"
cap "$W/notes.txt" jigc doc set-slot "$REL/changes/added/notes" --from-file "$W/notes.md" --task "$T"
bar "prose carrying a heading at exactly that depth is accepted" "test $RC -eq 0"
cap "$W/notes.json" jigc doc show "$REL/changes/added/notes" --task "$T" --format json
bar "…and reads back VERBATIM through the pinned read" \
    "jchk $W/notes.json 'j===require(\"fs\").readFileSync(\"$W/notes.md\",\"utf8\").trimEnd()'"

echo "  -- face 2: the multi-slot block, NON-LAST slot — roadmap#milestones/<id>/proves"
cap "$W/rm.txt" jigc doc create roadmap --title Roadmap --task "$TR"
cap "$W/ms.txt" jigc doc add-item 'roadmap:roadmap#milestones' --title 'M1 the probe' --task "$TR"
MS="$(tr -d '\n' < "$W/ms.txt")"; echo "  MS=$MS"
cap "$W/depth2.txt" jigc doc set-slot "$MS/proves" --from-file "$W/shallow.md" --task "$TR"
D2="$(free_depth "$W/depth2.txt")"; echo "  prescribed free depth at milestones/<id>/proves: '$D2'"
bar "the multi-slot block names its own free depth" "test $RC -ne 0 && test -n '$D2'"
printf 'Proves intro\n\n%s Proves heading\n\nafter\n' "$D2" > "$W/proves.md"
printf 'Decomp intro\n\n%s Decomp heading\n\ntail\n' "$D2" > "$W/decomp.md"
printf 'Proves v2\n\n%s Proves heading v2\n\nafter2\n' "$D2" > "$W/proves2.md"
cap "$W/p1.txt" jigc doc set-slot "$MS/proves" --from-file "$W/proves.md" --task "$TR"; P1=$RC
cap "$W/p2.txt" jigc doc set-slot "$MS/decomposition" --from-file "$W/decomp.md" --task "$TR"; P2=$RC
cap "$W/p3.txt" jigc doc set-slot "$MS/proves" --from-file "$W/proves2.md" --task "$TR"; P3=$RC
bar "both slots accept a heading at the named depth, and the NON-LAST slot accepts an UPDATE over it" \
    "test $P1 -eq 0 && test $P2 -eq 0 && test $P3 -eq 0"
cap "$W/ms.json" jigc doc show "$MS" --task "$TR" --format json
bar "the item reads back with BOTH slots intact — the next '#### Decomposition' sub-label was not swallowed" \
    "jchk $W/ms.json 'j.proves===require(\"fs\").readFileSync(\"$W/proves2.md\",\"utf8\").trimEnd() && j.decomposition===require(\"fs\").readFileSync(\"$W/decomp.md\",\"utf8\").trimEnd()'"
step sed -n '/^## Milestones/,$p' ".jigc/tasks/$TR/docs/roadmap:roadmap.md"

echo "  -- face 3: the writer's half — spec#criteria, set-field insert+update over heading-bearing prose"
cap "$W/depth3.txt" jigc doc set-slot "$CRIT/statement" --from-file "$W/shallow.md" --task "$TS"
D3="$(free_depth "$W/depth3.txt")"; echo "  prescribed free depth at criteria/<id>/statement: '$D3'"
printf 'Given a queue\n\n%s Then it is bounded\n\ntail\n' "$D3" > "$W/stmt.md"
cap "$W/st.txt" jigc doc set-slot "$CRIT/statement" --from-file "$W/stmt.md" --task "$TS"; S1=$RC
cap "$W/f1.txt" jigc doc set-field "$CRIT/maps-to-test" --value 'test/ingest.test.ts' --task "$TS"; F1=$RC
cap "$W/f2.txt" jigc doc set-field "$CRIT/maps-to-test" --value 'test/rollup.test.ts' --task "$TS"; F2=$RC
SPEC=".jigc/tasks/$TS/docs/spec:bounded-ingest.md"
N_BULLETS="$(grep -c '^- maps-to-test:' "$SPEC")"; echo "  maps-to-test bullets on disk: $N_BULLETS"
step sed -n '/^## Criteria/,$p' "$SPEC"
bar "insert then update over the heading-bearing prose lands ONE bullet, not two" \
    "test $S1 -eq 0 && test $F1 -eq 0 && test $F2 -eq 0 && test $N_BULLETS -eq 1"
cap "$W/crit.json" jigc doc show "$CRIT" --task "$TS" --format json
bar "…and the read returns the SECOND value, never the stale first, with the prose intact" \
    "jchk $W/crit.json 'j[\"maps-to-test\"]===\"test/rollup.test.ts\" && j.statement===require(\"fs\").readFileSync(\"$W/stmt.md\",\"utf8\").trimEnd()'"

# ---------------------------------------------------------------------------
say "(b) · two hand-planted field bullets → task validate blocks conformance.duplicate-field — rc.12 exited 0"
jigc doc set-field "commit:$T#type" --value docs --task "$T" >/dev/null 2>&1
jigc doc set-field "commit:$T#scope" --value changelog --task "$T" >/dev/null 2>&1
printf 'record the probe release\n' > "$W/summary.md"
jigc doc set-slot "commit:$T#summary" --from-file "$W/summary.md" --task "$T" >/dev/null 2>&1
cap "$W/base.txt" jigc task validate "$T"
bar "baseline: the task validates at exit 0 BEFORE the plant" "test $RC -eq 0"
STAGED=".jigc/tasks/$T/docs/changelog:changelog.md"
cp "$STAGED" "$W/changelog-good.md"
# The plant is the out-of-band edit the finding is about: a second `- date:` bullet in
# the first release's field group. `date` is a declared, required item field.
node -e 'const fs=require("fs"),f=process.argv[1];fs.writeFileSync(f,fs.readFileSync(f,"utf8").replace(/- date: (\S+)/,"- date: $1\n- date: 2020-01-01"))' "$STAGED"
step grep -n '^- date:' "$STAGED"
cap "$W/dup.txt" jigc task validate "$T"; DRC=$RC
cap "$W/dup.json" jigc task validate "$T" --format json
bar "task validate EXITS NON-ZERO over the duplicate — the exit flip (rc.12: exit 0)" "test $DRC -ne 0"
bar "…reporting conformance.duplicate-field as blocking, located at the item's field" \
    "jchk $W/dup.json 'j.findings.some(f=>f.code===\"conformance.duplicate-field\"&&f.severity===\"blocking\"&&f.location.address===\"$REL/date\")'"
bar "…and the text names the line and the hand-edit repair" \
    "grep -q 'conformance.duplicate-field' $W/dup.txt && grep -q '· line ' $W/dup.txt && grep -q 'delete the repeated' $W/dup.txt"
cp "$W/changelog-good.md" "$STAGED"
cap "$W/restored.txt" jigc task validate "$T"
bar "restoring the bytes returns the task to exit 0 — the plant, not the doc, was the fault" "test $RC -eq 0"

say "SUMMARY"
echo "  driven on the shipped blocks the census names: changelog (releases · nested changes ·"
echo "  the enum unreleased-changes), spec#criteria, roadmap#milestones. The 18-cell shape"
echo "  space is test-fenced (item_region_shape_space.rs) and was not re-driven; the slot∧nested"
echo "  face is unreachable on any shipped block, measured above."
if [ "$FAIL" -eq 0 ]; then echo "ARM 11 PASS"; else echo "ARM 11 FAIL"; fi
exit "$FAIL"
