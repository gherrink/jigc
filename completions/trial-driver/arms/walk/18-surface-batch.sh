#!/usr/bin/env bash
# 18-surface-batch.sh — protocol.md §5 arm 18 (M49 Increment 11, T1–T8, + the two
# items owed after RC-1.0-final, S-1 and S-4, + the M46 changelog-gate carry).
#
# WHICH SET THIS ARM ITERATES: two code-side registries, each driven over a STATED
# SUBSET, plus a hand-enumerated remainder:
#
#   (a) `DOCTYPE_DOORS` — 15 doors (`crates/cli/src/cli.rs`, the `DOCTYPE_DOORS` const)
#       take a doctype; flow50 iterates all 15. This arm drives 8 of them: the two
#       create-gate doors (`create.unknown-doctype`) and six existence doors
#       (`store.unknown-type`). Subset, said to be one.
#   (d) `RefusalKind::ALL` — 11 members since M50 Increment 2 (`crates/cli/src/rename.rs`);
#       `flow37_rename::every_rename_refusal_carries_an_identity_and_an_exit` iterates all
#       11 with their repairs. This arm REACHES nine of them through the real binary from
#       a store it builds, and records the order the door checks them in. Subset, said to
#       be one: `write.malformed-slug` is driven whole by `slug_override_axis.rs` over its
#       six doors, and `write.untrackable-destination` needs an embedded repository
#       planted at the doctype's home — a corpus mutation that would poison every cell
#       after it in this arm.
#   (b)(c)(e)(f)(g)(h) — hand-enumerated: the non-git-dir answer over two verbs; the
#       `write.unknown-section` cell over four write verbs; the non-UTF-8 argv byte;
#       `describe --commands` in both formats; S-1 and S-4; the changelog gate.
#
# CAPTURE, THEN GREP. This arm runs under `pipefail`, so `jigc … | grep -q` reports
# jigc's exit, and a refusal that matched would read as FAIL. Every assertion greps a
# captured variable (learned on this arm's first container run: ten green cells red).
#
# PASS CONDITION, stated before the run:
#   (a) unknown doctype `nosuch` → ONE code per door kind, a route, and NO `{:?}` Debug
#       leak (the output never contains `PackResourceKind` or `Schemas(`);
#   (b) `jigc validate` and `jigc start "x"` from a non-git dir → the SAME one text on
#       both verbs, exit ≠ 0, naming `git init`;
#   (c) an undeclared section at `set-slot`, `set-field --value`, `set-field --unset`,
#       and a `doc author` payload → `write.unknown-section` + a `jigc doc schema <t>`
#       route (rc.12 printed a bare `{"error": …}`);
#   (d) every reachable rename refusal carries its code AND exits non-zero;
#   (e) a non-UTF-8 argv byte, invocation log ON → exit ≠ 101 (rc.12 panicked);
#   (f) `describe --commands --format json` has > 16 entries each with a `pack` key,
#       and the prose form renders `<id> (<pack> pack)`;
#   (g) S-1: the composed text of a `--from-file -` workflow names the permitted
#       heredoc form `<<'EOF'`; S-4: `jigc rename … --task` names `jigc doc rename`;
#   (h) `task validate` on a `single-task` that wrote no changelog draws
#       `changelog-recording.gate-granted-unused` at exit 0 (M46, unchanged).
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
addr() { grep -oE "^$1:[a-z0-9-]+" | head -1; }             # capture, never tail -1
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
# Run once, print the transcript, keep the bytes + exit in OUT / RC for the bars.
cap()  { printf '\n$ %s\n' "$*"; OUT="$("$@" 2>&1)"; RC=$?; printf '%s\n[exit %s]\n' "$OUT" "$RC"; }
# One rename refusal cell: capture, then assert code + non-zero on the SAME run.
refusal() { # <label> <expected-code> <argv…>
  local label="$1" want="$2"; shift 2
  cap "$@"
  bar "$label · code $want"     "printf '%s' \"\$OUT\" | grep -q '$want'"
  bar "$label · exits non-zero" "test $RC -ne 0"
}

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
git add -A >/dev/null 2>&1; git commit -qm "walk 18: install + log on" >/dev/null 2>&1
jigc --version

say "FIXTURE · two committed adrs and a committed vision — what the rename cells need"
A1="$(land_adr 'Drop the oldest sample')"; A2="$(land_adr 'Cap distinct series')"
jigc start --workflow form-vision "form the project vision" >/dev/null 2>&1
TV="$(newtask)"
jigc doc create vision --title Vision --task "$TV" >/dev/null 2>&1
for s in thesis invariants open-questions; do
  printf 'Written for walk arm 18.\n' | jigc doc set-slot "vision:vision#$s" --from-file - --task "$TV" >/dev/null 2>&1
done
jigc doc set-field "commit:$TV#header/type" --value docs --task "$TV" >/dev/null 2>&1
printf 'form the vision\n' | jigc doc set-slot "commit:$TV#summary" --from-file - --task "$TV" >/dev/null 2>&1
jigc task finalize "$TV" >/dev/null 2>&1
step git ls-files docs/decisions VISION.md
TL="$(jigc task list 2>&1)"
bar "two adrs + the vision are committed, no task is live, the tree is clean" \
    "test \$(git ls-files docs/decisions/ | wc -l | tr -d ' ') -eq 2 && git ls-files --error-unmatch VISION.md && ! printf '%s' \"\$TL\" | grep -q '^  [a-z0-9]' && test -z \"\$(git status --porcelain)\""

say "(a) · unknown doctype \`nosuch\` — one answer per door kind, no Debug leak"
LEAK="$(mktemp "${TMPDIR:-/tmp}/arm18-a.XXXXXX")"
door() { # <expected-code> <argv…> — one captured run: code + route + no leak
  local want="$1"; shift
  cap "$@"; printf '%s\n' "$OUT" >>"$LEAK"
  bar "$* → $want + a route" "printf '%s' \"\$OUT\" | grep -q '$want' && printf '%s' \"\$OUT\" | grep -q 'route:'"
}
jigc start --workflow record-decision "probe the doctype doors" >/dev/null 2>&1
T="$(newtask)"
door create.unknown-doctype jigc doc create nosuch --title X --task "$T"
bar "…and it names the authorable set, routing at jigc describe" "printf '%s' \"\$OUT\" | grep -q 'known doctypes' && printf '%s' \"\$OUT\" | grep -q 'jigc describe'"
door create.unknown-doctype sh -c "printf 'title: X\nsections: []\n' | jigc doc author nosuch --from-file - --task $T"
jigc task discard "$T" >/dev/null 2>&1
door store.unknown-type jigc doc show nosuch:x
door store.unknown-type jigc doc schema nosuch
door store.unknown-type jigc doc list nosuch
door store.unknown-type jigc migrate README.md --as nosuch
bar "…migrate's narrower set is said in the MESSAGE, the code stays store.unknown-type" "printf '%s' \"\$OUT\" | grep -q 'migratable doctypes:'"
door store.unknown-type jigc relocate nosuch --from x/
door store.unknown-type jigc rename nosuch:x --to Y
bar "no {:?} Debug leak on any of the eight doors" "! grep -qE 'PackResourceKind|Schemas\(' \"$LEAK\""

say "(b) · jigc from a NON-git directory — one text, one route, on both verbs"
NG="$(mktemp -d "${TMPDIR:-/tmp}/arm18-nogit.XXXXXX")"
VOUT="$(cd "$NG" && jigc validate 2>&1)"; (cd "$NG" && jigc validate >/dev/null 2>&1); VRC=$?
SOUT="$(cd "$NG" && jigc start "x" 2>&1)"; (cd "$NG" && jigc start "x" >/dev/null 2>&1); SRC=$?
printf '\n$ (cd %s && jigc validate)\n%s\n[exit %s]\n' "$NG" "$VOUT" "$VRC"
printf '\n$ (cd %s && jigc start "x")\n%s\n[exit %s]\n' "$NG" "$SOUT" "$SRC"
bar "validate refuses (non-zero)"                  "test $VRC -ne 0"
bar "start refuses (non-zero)"                     "test $SRC -ne 0"
bar "both name the state and the route: git init here first" \
    "printf '%s' \"\$VOUT\" | grep -q 'not inside a git repository' && printf '%s' \"\$VOUT\" | grep -q 'git init'"
bar "…and the two verbs print the SAME text"       "test \"\$VOUT\" = \"\$SOUT\""
cd /work

say "(c) · write.unknown-section at four write verbs — the code + the \`jigc doc schema\` route"
jigc start --workflow record-decision "probe the section miss" >/dev/null 2>&1
T="$(newtask)"
A="$(jigc doc create adr --title 'Probe decision' --task "$T" 2>&1 | addr adr)"
echo "staged: $A"
sec() { # <label> <argv…> — asserts code + route on one captured run
  local label="$1"; shift
  cap "$@"
  bar "$label · write.unknown-section"            "printf '%s' \"\$OUT\" | grep -q 'write.unknown-section'"
  bar "$label · routes at jigc doc schema adr"    "printf '%s' \"\$OUT\" | grep -q 'jigc doc schema adr'"
  bar "$label · non-zero"                          "test $RC -ne 0"
}
sec "set-slot"          sh -c "echo x | jigc doc set-slot '$A#nosection' --from-file - --task $T"
sec "set-field --value" jigc doc set-field "$A#nosection" --value x --task "$T"
sec "set-field --unset" jigc doc set-field "$A#nosection" --unset --task "$T"
sec "author payload"    sh -c "printf 'title: Probe decision\nsections:\n  - id: nosection\n    set:\n      context: |\n        <<x>>\n' | jigc doc author adr --from-file - --task $T"
# The contrast cell, recorded not required: the same miss addressed one level deeper.
cap jigc doc set-field "$A#nosection/status" --value x --task "$T"
echo "  OBSERVE  if the two set-field cells above FAIL while this deeper form carries the code,"
echo "           the miss is the SECTION-level address shape at set-field only (M49 Increment 11's"
echo "           'four producers of the bare form' claim, re-measured on the binary)."
jigc task discard "$T" >/dev/null 2>&1

say "(e) · a non-UTF-8 argv byte with the invocation log ON — must not panic"
NOUT="$(jigc doc show $'\xff' 2>&1)"; jigc doc show $'\xff' >/dev/null 2>&1; NRC=$?
printf '\n$ jigc doc show <the single byte 0xff>\n%s\n[exit %s]\n' "$NOUT" "$NRC"
bar "exit is not 101 (no panic)"                   "test $NRC -ne 101"
bar "…and it is a refusal, not a success"          "test $NRC -ne 0"
LOGT="$(tail -3 .jigc/logs/invocations.jsonl 2>&1)"
bar "…and the invocation log recorded it (lossy argv, not a crash)" "printf '%s' \"\$LOGT\" | grep -q '\"doc\",\"show\"'"
step tail -1 .jigc/logs/invocations.jsonl

say "(f) · describe --commands — every entry carries its pack, in both formats"
CJ="$(mktemp "${TMPDIR:-/tmp}/arm18-cmds.XXXXXX")"
jigc describe --commands --format json > "$CJ" 2>&1
step node -e '
const d=JSON.parse(require("fs").readFileSync(process.argv[1],"utf8"));
const c=d.commands||[]; const nopack=c.filter(x=>!x.pack);
console.log("entries: "+c.length+" · without pack: "+nopack.length+" · packs: "+[...new Set(c.map(x=>x.pack))].join(","));
process.exit(c.length>16&&nopack.length===0?0:1);' "$CJ"
bar "JSON: > 16 entries, each with a pack key" \
    "node -e 'const d=JSON.parse(require(\"fs\").readFileSync(process.argv[1],\"utf8\"));const c=d.commands||[];process.exit(c.length>16&&c.every(x=>x.pack)?0:1)' \"$CJ\""
DP="$(jigc describe --commands 2>&1)"
bar "prose renders <id> (<pack> pack)"              "printf '%s' \"\$DP\" | grep -qE '[a-z-]+ \((dev|methodology) pack\)'"
bar "…for BOTH packs"                              "printf '%s' \"\$DP\" | grep -q '(dev pack)' && printf '%s' \"\$DP\" | grep -q '(methodology pack)'"

say "(g) · S-1: the composed step names the permitted heredoc form · S-4: rename --task names doc rename"
jigc start --workflow record-decision "x" > /tmp/composed.txt 2>&1
TX="$(newtask)"
step grep -n "from-file -\|EOF\|heredoc" /tmp/composed.txt
bar "S-1 · the composed text names the heredoc form <<'EOF' next to --from-file -" \
    "grep -q \"from-file - --task .* <<'EOF'\" /tmp/composed.txt"
bar "S-1 · …and says in prose that stdin is the payload channel" "grep -qi 'heredoc' /tmp/composed.txt"
cap jigc rename "$A1" --to Y --task "$TX"; S4="$OUT"; S4RC=$RC
bar "S-4 · the wrong-turn refuses"                  "test $S4RC -ne 0"
bar "S-4 · …and its tip names jigc doc rename with the argv shape" \
    "printf '%s' \"\$S4\" | grep -q 'jigc doc rename <address> --to <title> --task <task-id>'"
bar "S-4 · …and says WHY rename takes no --task (task-less, self-committing)" "printf '%s' \"\$S4\" | grep -q 'task-less'"

say "(d) · jigc rename — nine of the eleven refusals, reached from the store the fixture built"
# In-flight first: the task S-1 minted is still open, which is the state the guard
# names — on an EXISTING doc, because `store.not-found` is checked before the guard.
refusal "in-flight (task $TX open)" "rename.in-flight" jigc rename "$A1" --to "Drop the newest sample"
bar "…the in-flight route names BOTH exits for that task" "printf '%s' \"\$OUT\" | grep -q \"jigc task finalize $TX\" && printf '%s' \"\$OUT\" | grep -q \"jigc task discard $TX\""
jigc task discard "$TX" >/dev/null 2>&1
TL="$(jigc task list 2>&1)"
bar "the store is settled again: no live task, clean tree" "! printf '%s' \"\$TL\" | grep -q '^  [a-z0-9]' && test -z \"\$(git status --porcelain)\""
refusal "unknown type"       "store.unknown-type"     jigc rename nosuch:x --to "Y"
refusal "transient commit"   "store.transient-type"   jigc rename commit:x --to "Y"
refusal "not-found"          "store.not-found"        jigc rename adr:no-such-doc --to "Y"
refusal "unslugable title"   "write.unslugable-title" jigc rename "$A1" --to "???"
refusal "already-present"    "write.already-present"  jigc rename "$A2" --to "Drop the oldest sample"
refusal "identity-change · singleton" "write.identity-change" jigc rename vision:vision --to "New Vision"
bar "…routes at the --slug form that keeps the fixed identity" "printf '%s' \"\$OUT\" | grep -q -- '--slug vision'"
# The work-unit identity needs a committed milestone-record: `milestone create` lands
# one in a record-only commit; `discard` settles it (still committed, still a record).
jigc milestone create "bound the store" >/dev/null 2>&1
jigc milestone discard bound-the-store >/dev/null 2>&1
step git ls-files docs/milestone-records
refusal "identity-change · milestone-record" "write.identity-change" jigc rename milestone-record:bound-the-store --to "Bound"
bar "…routes at the --slug form that keeps the work-unit identity" "printf '%s' \"\$OUT\" | grep -q -- '--slug bound-the-store'"
echo x >> README.md
refusal "dirty-tree"         "rename.dirty-tree"      jigc rename "$A1" --to "Drop the newest sample"
git checkout -- README.md >/dev/null 2>&1
# Recorded, not a refusal: the idempotent no-op is the tenth cell and exits 0 by design.
cap jigc rename "$A1" --to "Drop the oldest sample"
bar "the idempotent rename is a no-op ack at exit 0, saying nothing moved" \
    "test $RC -eq 0 && printf '%s' \"\$OUT\" | grep -q 'nothing renamed, nothing committed'"
# Recorded: the bare singleton form `jigc rename vision --to …` is what `doc show` accepts.
cap jigc rename vision --to "New Vision"
echo "  OBSERVE  a bare singleton address at the top-level verb: recorded above — it carries a route"
echo "           but no finding code (compare \`jigc doc show vision\`, which accepts the bare form)."

say "(h) · changelog-recording.gate-granted-unused at exit 0 on a single-task with no changelog (M46 carry)"
jigc start --workflow single-task "tidy the ingest path" >/dev/null 2>&1
TS="$(newtask)"
jigc doc set-field "commit:$TS#header/type" --value chore --task "$TS" >/dev/null 2>&1
printf 'tidy the ingest path\n' | jigc doc set-slot "commit:$TS#summary" --from-file - --task "$TS" >/dev/null 2>&1
cap jigc task validate "$TS"; H="$OUT"; HRC=$RC
bar "the advisory fires"                            "printf '%s' \"\$H\" | grep -q 'advisory · changelog-recording.gate-granted-unused'"
bar "…names the granting workflow"                  "printf '%s' \"\$H\" | grep -q 'workflow \`single-task\` grants'"
bar "…carries a route naming the create + add-item pair AND the no-action exit" \
    "printf '%s' \"\$H\" | grep -q 'jigc doc create changelog' && printf '%s' \"\$H\" | grep -q 'no action is needed'"
bar "…at exit 0 — advisory, not a block"            "test $HRC -eq 0"
jigc task discard "$TS" >/dev/null 2>&1

say "SUMMARY"
echo "  8 of DOCTYPE_DOORS' 15 · 9 of RefusalKind::ALL's 11 (+ the no-op) · the non-git pair ·"
echo "  the four-verb section miss · non-UTF-8 argv · describe in both formats · S-1 · S-4 · the"
echo "  M46 changelog gate. Both registries are fenced in full by flow50; this arm drives a stated subset."
if [ "$FAIL" -eq 0 ]; then echo "ARM 18 PASS"; else echo "ARM 18 FAIL"; fi
exit "$FAIL"
