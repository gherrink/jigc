#!/usr/bin/env bash
# 21-migrate-continue-on-rc13.sh — protocol.md §5 arm 21, SECOND HALF of the
# MIGRATION PAIR (`completions/artifacts/RC-m50/protocol.md` §5 arm 21; pairs with 14).
#
# Numbered 21 so it sorts after every single-binary arm: it must run AFTER arm 14,
# on a DIFFERENT binary, over the corpus that half left behind. See
# `14-migrate-author-on-rc12.sh` for the three-command sequence.
#
# WHICH SET THIS ARM ITERATES: the **declared change-set of the M49 → rc.13 bump**,
# named from the two shipped manifests rather than from memory — every
# `schema-version` that moved between `5ff85ea` and HEAD in
# `packs/methodology/config/schema-manifest.yaml`:
#
#   completion-record  1 → 2   (EnumWidened: severity gains HIGH/MEDIUM/LOW; AddedItemSlot: detail)
#   milestone-record   2 → 3   (the per-task `workflow` leaf)
#   planning-record    new     (no committed instance can predate it — nothing to migrate)
#
# plus the ONE behaviour tightening that landed after M49 closed, `d854e25`: the
# milestone boundary gates a sub-task's transient commit doc as hard as the task
# door does. The dev pack's frozen set did not move, so it is not on the axis.
#
# It compares against MEASUREMENTS, not memory: every flip below is asserted
# against the number the first half wrote into `.upgrade-baseline` on rc.12.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
B=/work/.upgrade-baseline
base() { sed -n "s/^$1=//p" "$B" | head -1; }
# `doc show --format json`'s top-level `schema-version` — the M49 read-contract
# number; read on rc.13 only (rc.12's envelope has no such key, see arm 14).
json_sv() { jigc doc show "$1" --format json 2>/dev/null \
  | node -e 'const d=JSON.parse(require("fs").readFileSync(0,"utf8"));process.stdout.write(String(d["schema-version"]))'; }
file_sv() { sed -n 's/^schema-version: //p' "$1" | head -1; }

if [ ! -f "$B" ]; then
  cat <<'SKIP'

=== SKIPPED — no baseline from the first half in this corpus

This arm continues the corpus `14-migrate-author-on-rc12.sh` left behind. Run:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc12 --only 14
  python3 run.py  carry <out-a>/14-migrate-author-on-rc12 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc13 --only 21
SKIP
  echo "ARM 21 SKIPPED (no .upgrade-baseline — the rc.12 half has not run)"
  exit 0
fi
if ! jigc --version | grep -q 'rc.13'; then
  echo
  echo "=== SKIPPED — this half needs the NEW binary (rc.13), and this pass is not it"
  echo "ARM 21 SKIPPED (needs jigc-gate:rc13; got $(jigc --version))"
  exit 0
fi

say "0 · we are the NEW binary, over a corpus the OLD one authored"
step jigc --version
step cat "$B"
M="$(base rc12-milestone)"; SUB="$(base rc12-subtask)"
CR="$(base rc12-completion-record)"; CR_PATH="$(base rc12-completion-record-path)"
MR="milestone-record:$M"; MR_PATH="$(base rc12-milestone-record-path)"
ADR="$(base rc12-adr-first)"
bar "this half really is running rc.13"  "jigc --version | grep -q 'rc.13'"
bar "the first half really ran rc.12"    "test \"\$(base rc12-version)\" = 1.0.0-rc.12"
bar "the corpus arrived at the first half's HEAD" "test \"\$(git rev-parse HEAD)\" = \"\$(base rc12-head)\""
bar "…with the milestone-record and completion-record it committed" \
    "git ls-files --error-unmatch '$MR_PATH' '$CR_PATH'"
bar "the tree arrived clean, with no rig evidence carried in" \
    "test -z \"\$(git status --porcelain=v1 | grep -v '^?? .upgrade-baseline')\""
bar "…and specifically no PROVENANCE.txt from the first half" "test ! -e PROVENANCE.txt"
bar "the OLD stamps are the ones the first half measured (file)" \
    "test \"\$(file_sv '$MR_PATH')\" = \"\$(base rc12-milestone-record-schema-version)\" && test \"\$(file_sv '$CR_PATH')\" = \"\$(base rc12-completion-record-schema-version)\""

# ---------------------------------------------------------------------------
say "1 · (a) validate flips: the two stale stamps and the binary mismatch are announced"
step jigc validate
V="$(jigc validate 2>&1)"; jigc validate >/dev/null 2>&1; VRC=$?
bar "rc.12 validated this corpus at exit 0 (baseline) …" "test \"\$(base rc12-validate-exit)\" = 0"
bar "… and rc.13 exits NON-ZERO over the same bytes"     "test $VRC -ne 0"
bar "the code that flipped it is schema-conformance.schema-version-current" \
    "printf '%s' \"\$V\" | grep -q 'schema-conformance.schema-version-current'"
bar "…on the milestone-record"  "printf '%s' \"\$V\" | grep 'schema-version-current' | grep -q '$MR_PATH'"
bar "…on the completion-record" "printf '%s' \"\$V\" | grep 'schema-version-current' | grep -q '$CR_PATH'"
bar "…each routed at jigc migrate-corpus" \
    "test \$(printf '%s' \"\$V\" | grep -c 'jigc migrate-corpus') -ge 2"
bar "the store says it was last written by the older binary (store-version.binary-mismatch)" \
    "printf '%s' \"\$V\" | grep -q 'store-version.binary-mismatch'"
bar "…and names both versions" \
    "printf '%s' \"\$V\" | grep -q 'rc.12' && printf '%s' \"\$V\" | grep -q 'rc.13'"
bar "no adr / research / changelog is flagged — the frozen dev set did not move" \
    "! printf '%s' \"\$V\" | grep 'schema-version-current' | grep -qE 'docs/decisions/|docs/research/|CHANGELOG.md'"

say "1b · the WRONG order, on a throwaway copy: setup FIRST, then validate — recorded, not asserted"
# MIGRATING.md says migrate-corpus first, setup second. What the binary says when an
# adopter does it the other way round is recorded here so the record carries it.
C="$(mktemp -d /tmp/setup-first.XXXXXX)"
cp -R /work/. "$C/"
( cd "$C" && step jigc setup && step jigc validate )
( cd "$C" && jigc validate >/dev/null 2>&1 ); C_VRC=$?
echo "setup-first copy: validate exits $C_VRC afterwards"
bar "setup-first does not paper over the stale stamps (validate still non-zero on the copy)" "test $C_VRC -ne 0"

# ---------------------------------------------------------------------------
say "2 · (b) migrate-corpus: lists both, lands STAMP-ONLY, and validate returns to 0"
step jigc migrate-corpus --dry-run
DRY="$(jigc migrate-corpus --dry-run 2>&1)"
bar "the dry run names the milestone-record"  "printf '%s' \"\$DRY\" | grep -q '$MR_PATH'"
bar "the dry run names the completion-record" "printf '%s' \"\$DRY\" | grep -q '$CR_PATH'"
# The report lists EVERY managed doc with its verdict (`would migrate` / `current`), so
# the dev set is named — as current. The first run of this bar grepped for the paths
# alone and reddened on a correct report; the predicate is the verdict line.
bar "…and nothing from the frozen dev set is a would-migrate line" \
    "! printf '%s' \"\$DRY\" | grep 'would migrate' | grep -qE 'docs/decisions/|docs/research/|CHANGELOG.md'"
bar "the v3 leaf the migration cannot invent (\`workflow\`, set: on-transition) is NAMED as left unfilled …" \
    "printf '%s' \"\$DRY\" | grep -q 'migrate-corpus.set-field-unfilled'"
bar "… with a route that says no action is needed" \
    "printf '%s' \"\$DRY\" | grep -A1 'set-field-unfilled' | grep -q 'no action needed'"
bar "a dry run moves nothing" "test -z \"\$(git status --porcelain=v1 | grep -v '^?? .upgrade-baseline')\" && test \"\$(git rev-parse HEAD)\" = \"\$(base rc12-head)\""

HEAD0="$(git rev-parse HEAD)"
# Captured, not `step`ped: `step` returns its own printf's status, and the exit bar
# below would be inert.
printf '\n$ jigc migrate-corpus\n'
MIG_OUT="$(jigc migrate-corpus 2>&1)"; MIG_RC=$?
printf '%s\n[exit %s]\n' "$MIG_OUT" "$MIG_RC"
git log --oneline -3
git status --porcelain=v1
# Whether migrate-corpus commits or stages is measured, not assumed: the diff is taken
# against the pre-migration HEAD either way.
if [ "$(git rev-parse HEAD)" != "$HEAD0" ]; then
  MIG_COMMITTED=1; DIFF="$(git diff "$HEAD0" HEAD -U0)"; STAT="$(git diff "$HEAD0" HEAD --stat)"
else
  MIG_COMMITTED=0; DIFF="$(git diff "$HEAD0" -U0)"; STAT="$(git diff "$HEAD0" --stat)"
fi
echo "migrate-corpus committed on its own: $MIG_COMMITTED"
printf '%s\n' "$STAT"
printf '%s\n' "$DIFF"
CHANGED_LINES="$(printf '%s' "$DIFF" | grep -E '^[-+]' | grep -vE '^(\+\+\+|---)')"
bar "migrate-corpus exits 0" "test $MIG_RC -eq 0"
bar "exactly the two stale docs changed" \
    "test \$(printf '%s' \"\$STAT\" | grep -c '\\.md') -eq 2 && printf '%s' \"\$STAT\" | grep -q '$MR_PATH' && printf '%s' \"\$STAT\" | grep -q '$CR_PATH'"
bar "the migration is STAMP-ONLY: every changed line is a schema-version line" \
    "test -n \"\$CHANGED_LINES\" && ! printf '%s' \"\$CHANGED_LINES\" | grep -qv 'schema-version:'"
bar "the milestone-record file is stamped 3 (was $(base rc12-milestone-record-schema-version))" \
    "test \"\$(file_sv '$MR_PATH')\" = 3"
bar "the completion-record file is stamped 2 (was $(base rc12-completion-record-schema-version))" \
    "test \"\$(file_sv '$CR_PATH')\" = 2"
bar "doc show --format json reports 3 on the milestone-record"  "test \"\$(json_sv '$MR')\" = 3"
bar "doc show --format json reports 2 on the completion-record" "test \"\$(json_sv '$CR')\" = 2"
bar "the record's authored content survived (status $(base rc12-milestone-record-status), severity blocking)" \
    "grep -q 'status: $(base rc12-milestone-record-status)' '$MR_PATH' && grep -q 'severity: blocking' '$CR_PATH'"
if [ "$MIG_COMMITTED" -eq 0 ]; then
  echo "(migrate-corpus left its writes uncommitted — landing them so the boundary below starts clean)"
  git add -- "$MR_PATH" "$CR_PATH"
  git -c user.name='Corpus Owner' -c user.email='owner@example.invalid' \
      commit -qm 'chore(jigc): migrate the corpus to the rc.13 schemas'
fi
step jigc validate
jigc validate >/dev/null 2>&1; V2RC=$?
bar "validate returns to exit 0 after the migration" "test $V2RC -eq 0"
V2="$(jigc validate 2>&1)"
bar "…and schema-version-current is gone" "! printf '%s' \"\$V2\" | grep -q 'schema-version-current'"

# ---------------------------------------------------------------------------
say "3 · (c) the widened enum: severity HIGH now lands (EnumWidened)"
jigc start --workflow completion "M2" >/dev/null 2>&1
TC="$(newtask)"
CR2="$(jigc doc create completion-record --title "M2" --task "$TC" 2>&1 | grep -oE '^completion-record:[a-z0-9-]+$' | head -1)"
jigc doc set-field "$CR2#meta/verdict" --value green --task "$TC" >/dev/null 2>&1
mkdir -p completions/artifacts/M2 && echo 'the verdict' > completions/artifacts/M2/VERDICT.md
jigc doc set-field "$CR2#meta/owner-artifact" --value completions/artifacts/M2/VERDICT.md --task "$TC" >/dev/null 2>&1
jigc doc add-item "$CR2#findings" --title "Stray finding" --task "$TC" >/dev/null 2>&1
step jigc doc set-field "$CR2#findings/stray-finding/severity" --value HIGH --task "$TC"
jigc doc set-field "$CR2#findings/stray-finding/severity" --value HIGH --task "$TC" >/dev/null 2>&1; H_RC=$?
bar "rc.12 refused HIGH at exit $(base rc12-severity-high-exit) (baseline) …" "test \"\$(base rc12-severity-high-exit)\" -ne 0"
bar "… and rc.13 lands it at exit 0" "test $H_RC -eq 0"
SHOWN="$(jigc doc show "$CR2#findings/stray-finding/severity" --task "$TC" --format json 2>&1)"
bar "the staged read carries HIGH" "printf '%s' \"\$SHOWN\" | grep -q 'HIGH'"
step jigc doc set-field "$CR2#findings/stray-finding/severity" --value CRITICAL --task "$TC"
jigc doc set-field "$CR2#findings/stray-finding/severity" --value CRITICAL --task "$TC" >/dev/null 2>&1; NM_RC=$?
bar "a non-member is still refused — it is a wider enum, not an open string" "test $NM_RC -ne 0"
jigc doc set-field "$CR2#findings/stray-finding/disposition" --value fixed --task "$TC" >/dev/null 2>&1
jigc doc set-field "$CR2#findings/stray-finding/evidence" --value 'audit.log:12' --task "$TC" >/dev/null 2>&1
jigc doc set-field "commit:$TC#header/type" --value docs --task "$TC" >/dev/null 2>&1
printf 'record the M2 completion\n' | jigc doc set-slot "commit:$TC#summary" --from-file - --task "$TC" >/dev/null 2>&1
step jigc task finalize "$TC"
bar "the HIGH-graded record lands" "git ls-files --error-unmatch docs/completions/m2.md && grep -q 'severity: HIGH' docs/completions/m2.md"
bar "…stamped 2 at birth" "test \"\$(file_sv docs/completions/m2.md)\" = 2"

# ---------------------------------------------------------------------------
say "4 · (d) the d854e25 tightening: the boundary now gates the sub-task's commit doc"
# The same construction the first half landed on rc.12 (baseline: landed=$(base rc12-milestone-finalize-typeless-landed),
# exit $(base rc12-milestone-finalize-typeless-exit), subject "$(base rc12-milestone-finalize-typeless-subject)").
# Both author-required leaves of `commit` — `type` and `summary` — are left as the
# skeleton leaves them, so the arm iterates the whole two-leaf axis the fix names.
step jigc config get finalize.fan-out.squash
bar "squash=false travelled with the corpus (committed manifest)" \
    "jigc config get finalize.fan-out.squash 2>&1 | grep -q false"
M2=bound-the-store-again
SUB2=prune-on-overflow
step jigc milestone create "bound the store again"
step jigc milestone add-task "$M2" "prune on overflow"
step jigc milestone provision "$M2"
SPAWN="$(jigc milestone execute "$M2" 2>&1 | sed -n 's/^Spawn: `\(.*\)`$/\1/p' | head -1)"
echo "launch line from \`jigc milestone execute\`: $SPAWN"
( eval "$SPAWN" ) >/dev/null 2>&1; echo "[launch line exit $?]"
WT2=".jigc/worktrees/$SUB2"
( cd "$WT2" && echo 'export const prune = true;' > src/prune.ts && git add src/prune.ts )
( cd "$WT2" && step jigc doc list --task "$SUB2" )
( cd "$WT2" && step jigc task validate "$SUB2" )

HEAD1="$(git rev-parse HEAD)"
printf '\n$ jigc milestone finalize %s   (type AND summary unset)\n' "$M2"
MF1="$(jigc milestone finalize "$M2" 2>&1)"; MF1_RC=$?
printf '%s\n[exit %s]\n' "$MF1" "$MF1_RC"
bar "rc.12 LANDED this shape (baseline landed=$(base rc12-milestone-finalize-typeless-landed)) …" \
    "test \"\$(base rc12-milestone-finalize-typeless-landed)\" = 1"
bar "… and rc.13 BLOCKS it (exit non-zero; measured $MF1_RC)" "test $MF1_RC -ne 0"
bar "the block names the transient commit doc"    "printf '%s' \"\$MF1\" | grep -q 'commit:$SUB2'"
bar "…and names the unset \`type\`"               "printf '%s' \"\$MF1\" | grep -q 'type'"
bar "…and the unset \`summary\`"                  "printf '%s' \"\$MF1\" | grep -qi 'summary'"
bar "…with a route carrying --task $SUB2"         "printf '%s' \"\$MF1\" | grep 'route' | grep -q -- '--task $SUB2'"
bar "it commits NOTHING"                          "test \"\$(git rev-parse HEAD)\" = '$HEAD1'"
bar "…and no \`: …\` subject entered the history" "! git log --format=%s -3 | grep -q '^: '"
# MEASURED on 1.0.0-rc.13 (f266770), 2026-09-04, and left as a FAILing bar rather than
# re-worded: the task door prints each of these findings as
# `blocking · schema-conformance.field-value-conformant — …`; the milestone door
# prints the SAME findings as bare lines — no severity, no code — so the stable
# `(code, target)` key M42 promised on every finding is absent from this door's
# text surface. The JSON envelope is driven right below so the record says whether
# the key exists on the machine surface and only the text drops it.
bar "the block lines carry a severity · code prefix, as the task door's do" \
    "printf '%s' \"\$MF1\" | grep -q 'blocking · schema-conformance'"
printf '\n$ jigc milestone finalize %s --format json   (same state — the machine surface)\n' "$M2"
MF1J="$(jigc milestone finalize "$M2" --format json 2>&1)"; MF1J_RC=$?
printf '%s\n[exit %s]\n' "$MF1J" "$MF1J_RC"
bar "the JSON envelope is valid JSON and still blocks" \
    "test $MF1J_RC -ne 0 && printf '%s' \"\$MF1J\" | node -e 'JSON.parse(require(\"fs\").readFileSync(0,\"utf8\"))'"
bar "…and its findings carry a code (the key the text surface dropped)" \
    "printf '%s' \"\$MF1J\" | grep -q 'schema-conformance'"
bar "…and still nothing committed after the JSON drive" "test \"\$(git rev-parse HEAD)\" = '$HEAD1'"

say "4 · the route for type, run VERBATIM from inside the worktree"
ROUTE="$(printf '%s' "$MF1" | sed -n 's/.*route: `\([^`]*\)`.*/\1/p' | grep 'header/type' | head -1)"
ROUTE="${ROUTE//<value>/feat}"
echo "route as printed (with <value> → feat): $ROUTE"
( cd "$WT2" && eval "$ROUTE" 2>&1; echo "[route exit $?]" )
( cd "$WT2" && eval "$ROUTE" >/dev/null 2>&1 ); R_RC=$?
bar "the printed route runs as printed" "test -n '$ROUTE' && test $R_RC -eq 0"

printf '\n$ jigc milestone finalize %s   (type set, summary still unset)\n' "$M2"
MF2="$(jigc milestone finalize "$M2" 2>&1)"; MF2_RC=$?
printf '%s\n[exit %s]\n' "$MF2" "$MF2_RC"
bar "still blocked on the second leaf" "test $MF2_RC -ne 0"
bar "…which is summary, not type"     "printf '%s' \"\$MF2\" | grep -qi 'summary' && ! printf '%s' \"\$MF2\" | grep -q 'header/type'"
bar "…and still nothing committed"    "test \"\$(git rev-parse HEAD)\" = '$HEAD1'"
( cd "$WT2" && printf 'prune on overflow\n' | jigc doc set-slot "commit:$SUB2#summary" --from-file - --task "$SUB2" 2>&1; echo "[set-slot exit $?]" )

printf '\n$ jigc milestone finalize %s   (both leaves filled)\n' "$M2"
MF3="$(jigc milestone finalize "$M2" 2>&1)"; MF3_RC=$?
printf '%s\n[exit %s]\n' "$MF3" "$MF3_RC"
git log --oneline -3
LOG3="$(git log --stat --format=%s -3)"
bar "the boundary lands once both leaves are filled" "test $MF3_RC -eq 0"
bar "one conventional commit per sub-task, with the authored subject" \
    "git log --format=%s -3 | grep -q '^feat: prune on overflow$'"
bar "…carrying the sub-task's code" "printf '%s' \"\$LOG3\" | grep -q 'src/prune.ts'"
bar "the new milestone-record is stamped 3 at birth" "test \"\$(file_sv docs/milestone-records/$M2.md)\" = 3"
bar "…and records the sub-task's workflow (the v3 leaf)" "grep -q 'workflow: sub-task' docs/milestone-records/$M2.md"

# ---------------------------------------------------------------------------
say "5 · (e) setup AFTER the migration installs the guide stamped with this build"
step jigc setup
bar "the guide artifact is installed" "test -f .claude/skills/jigc/SKILL.md"
bar "…and is stamped with this build" "grep -q 'rc.13' .claude/skills/jigc/SKILL.md"
bar "…and the version stamp now names rc.13" "grep -q 'rc.13' .jigc/version"
step jigc validate
jigc validate >/dev/null 2>&1; V3RC=$?
bar "validate is exit 0 after setup" "test $V3RC -eq 0"
V3="$(jigc validate 2>&1)"
bar "…and the binary mismatch is gone" "! printf '%s' \"\$V3\" | grep -q 'binary-mismatch'"

# ---------------------------------------------------------------------------
say "6 · (f) what the OLD binary wrote reads back through the new one"
step jigc doc show "adr:$ADR"
SHOW_ADR="$(jigc doc show "adr:$ADR" 2>&1)"
bar "an adr the OLD binary wrote reads back"        "printf '%s' \"\$SHOW_ADR\" | grep -q '^# '"
LIST="$(jigc doc list 2>&1)"
bar "…and is managed in the index read"             "printf '%s' \"\$LIST\" | grep 'adr:$ADR' | grep -q managed"
bar "the migrated milestone-record is managed"     "printf '%s' \"\$LIST\" | grep '$MR' | grep -q managed"
bar "the migrated completion-record is managed"    "printf '%s' \"\$LIST\" | grep '$CR' | grep -q managed"
bar "the old changelog's nested change-group reads back" \
    "jigc doc show 'changelog:changelog#releases/0-1-0/changes/added' --format json 2>&1 | node -e 'const d=JSON.parse(require(\"fs\").readFileSync(0,\"utf8\"));process.exit(JSON.stringify(d).includes(\"ring buffer\")?0:1)'"
step jigc doc show "$CR" --format json
bar "the migrated completion-record's finding is intact (severity blocking, disposition fixed)" \
    "jigc doc show '$CR' --format json 2>&1 | node -e 'const d=JSON.parse(require(\"fs\").readFileSync(0,\"utf8\"));const f=d.sections.findings[0];process.exit(f.severity===\"blocking\"&&f.disposition===\"fixed\"?0:1)'"
bar "the working tree ends clean (baseline aside)" \
    "test -z \"\$(git status --porcelain=v1 | grep -v '^?? .upgrade-baseline')\""

say "SUMMARY"
echo "  the migration path: a corpus rc.12 wrote — two stale stamps, a HIGH refusal and"
echo "  a type-less sub-task commit the old boundary landed — validated, migrated"
echo "  stamp-only, continued and re-gated on rc.13, every flip asserted against the"
echo "  first half's MEASURED baseline rather than against memory."
if [ "$FAIL" -eq 0 ]; then echo "ARM 21 PASS"; else echo "ARM 21 FAIL"; fi
exit "$FAIL"
