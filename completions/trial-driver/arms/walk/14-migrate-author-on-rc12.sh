#!/usr/bin/env bash
# 14-migrate-author-on-rc12.sh — protocol.md §5 arm 14, FIRST HALF of the
# MIGRATION PAIR (`completions/artifacts/RC-m50/protocol.md` §5 arm 14; pairs with 21).
#
# WHICH SET THIS ARM ITERATES: none — it is a MEASUREMENT. It authors a corpus
# through the OLD binary (rc.12, the last binary before M49's three schema bumps
# and the post-M49 boundary tightening) and writes what that binary did into
# `/work/.upgrade-baseline`, so the second half asserts every flip against a
# number rather than against a remembered claim.
#
# A two-binary arm, like 03/10, so walk.py cannot drive it in one pass:
#
#   python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc12 --only 14
#   python3 run.py  carry <out-a>/14-migrate-author-on-rc12 <corpus-b>
#   python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc13 --only 21
#
# `run.py carry` between the halves is load-bearing: run-session.sh writes its own
# evidence into the out-dir, and handing that straight on plants the rig's
# droppings in the corpus the second half reads.
#
# WHY A SEPARATE ARM FROM 03/10: the handover's own warning — "a schema bump makes
# the following trial about migration rather than about the fixes" — and M49
# shipped three bumps plus a new doctype. The blind sessions run on fresh corpora
# and never meet a migration; this pair is where the migration is measured.
#
# THE SUBJECTS this half authors on rc.12, each recorded as the binary did it:
#   (1) a milestone under `finalize.fan-out.squash false` whose provisioned sub-task
#       holds staged code AND a commit doc with `type` left unset — the `d854e25`
#       baseline: rc.12's `jigc milestone finalize` did not gate a sub-task's
#       transient commit doc, so it LANDS a subject like `: …` at exit 0
#   (2) a `completion-record` whose `findings` item is graded `HIGH` — refused on
#       rc.12 (the enum was `[blocking, advisory]`), then set to `blocking` so the
#       record lands at schema-version 1
#   (3) the committed `milestone-record` from (1), stamped schema-version 2
# plus the ordinary corpus around them: two adrs, a research doc, a changelog with
# one release and one nested change-group.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z0-9]/{print $1; exit}'; }
B=/work/.upgrade-baseline
rec() { printf '%s=%s\n' "$1" "$2" >> "$B"; }

# PRECONDITION, checked rather than assumed: this half must run on the OLD binary.
# A full-walk pass hands it the new one, and it SKIPS loudly rather than failing.
if ! jigc --version | grep -q 'rc.12'; then
  cat <<'SKIP'

=== SKIPPED — this arm needs the OLD binary (rc.12), and this pass is not it

The migration pair is driven as an explicit three-command sequence:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc12 --only 14
  python3 run.py  carry <out-a>/14-migrate-author-on-rc12 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc13 --only 21

`run.py carry` between the halves is load-bearing: run-session.sh writes its own
evidence into the out-dir, and handing that straight on would plant the rig's
droppings in the corpus the second half reads.
SKIP
  echo "ARM 14 SKIPPED (needs jigc-gate:rc12; got $(jigc --version))"
  exit 0
fi

say "0 · adopt with the OLD binary"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
step jigc --version
bar "this half really is running rc.12" "jigc --version | grep -q 'rc.12'"
: > "$B"
rec rc12-version "$(jigc --version | awk '{print $2}')"

# ---------------------------------------------------------------------------
say "1 · the ordinary corpus, authored THROUGH the old binary"
land_adr() {
  jigc start --workflow record-decision "record $1" >/dev/null 2>&1
  local t a s; t="$(newtask)"
  a="$(jigc doc create adr --title "$1" --task "$t" 2>&1 | grep -oE '^adr:[a-z0-9-]+$' | head -1)"
  for s in context decision consequences; do
    echo 'Authored on rc.12, before the migration.' \
      | jigc doc set-slot "$a#$s" --from-file - --task "$t" >/dev/null 2>&1
  done
  jigc doc set-field "commit:$t#header/type" --value docs --task "$t" >/dev/null 2>&1
  printf 'record %s\n' "$1" | jigc doc set-slot "commit:$t#summary" --from-file - --task "$t" >/dev/null 2>&1
  jigc task finalize "$t" >/dev/null 2>&1
}
land_adr 'Drop the oldest sample on overflow'
land_adr 'Cap distinct series at a ceiling'

# a research doc — a methodology-pack doctype, at a different home
jigc start --workflow do-research "how other buffers shed load" >/dev/null 2>&1
TR="$(newtask)"
RA="$(jigc doc create research --title 'How other buffers shed load' --task "$TR" 2>&1 \
      | grep -oE '^research:[a-z0-9-]+$' | head -1)"
for s in question findings sources; do
  echo 'Authored on rc.12.' | jigc doc set-slot "$RA#$s" --from-file - --task "$TR" >/dev/null 2>&1
done
jigc doc set-field "commit:$TR#header/type" --value docs --task "$TR" >/dev/null 2>&1
printf 'record the buffer research\n' | jigc doc set-slot "commit:$TR#summary" --from-file - --task "$TR" >/dev/null 2>&1
jigc task finalize "$TR" >/dev/null 2>&1

# a changelog with one release and one NESTED change-group, via record-change —
# the shipped nested repeatable no migration can reshape (handover → owed item 2)
jigc start --workflow record-change "record the first release" >/dev/null 2>&1
TL="$(newtask)"
jigc doc create changelog --title Changelog --task "$TL" >/dev/null 2>&1
jigc doc add-item "changelog:changelog#releases" --title "0.1.0" --task "$TL" >/dev/null 2>&1
jigc doc add-item "changelog:changelog#releases/0-1-0/changes" --title "Added" --task "$TL" >/dev/null 2>&1
printf -- '- the ring buffer\n' \
  | jigc doc set-slot "changelog:changelog#releases/0-1-0/changes/added/notes" --from-file - --task "$TL" >/dev/null 2>&1
jigc doc set-field "commit:$TL#header/type" --value docs --task "$TL" >/dev/null 2>&1
printf 'record the first release\n' | jigc doc set-slot "commit:$TL#summary" --from-file - --task "$TL" >/dev/null 2>&1
step jigc task finalize "$TL"

step git log --oneline
bar "the corpus carries adrs"        "test \$(git ls-files docs/decisions/ | wc -l | tr -d ' ') -ge 2"
bar "…a research doc"                "test \$(git ls-files docs/research/ | wc -l | tr -d ' ') -ge 1"
bar "…and a changelog at its placement home" "git ls-files --error-unmatch CHANGELOG.md"
# Captured first, then grepped: under `pipefail`, `cmd | grep -q` can report grep's
# early close as 141 and turn a true bar red.
NESTED="$(jigc doc show 'changelog:changelog#releases/0-1-0/changes/added' 2>&1)"
bar "the changelog carries the release AND the nested change-group" \
    "printf '%s' \"\$NESTED\" | grep -q 'ring buffer'"
rec rc12-adr-count "$(git ls-files docs/decisions/ | wc -l | tr -d ' ')"
rec rc12-adr-first "$(git ls-files docs/decisions/ | head -1 | sed 's|docs/decisions/||; s|\.md$||')"

# ---------------------------------------------------------------------------
say "2 · SUBJECT (2) · a completion-record graded HIGH — what rc.12 says"
# Taken BEFORE the milestone so the task list holds nothing but this task when
# `newtask` reads it; the milestone's sub-task is minted below and settled by the
# boundary, and if that boundary does NOT land on rc.12 the sub-task stays active.
jigc start --workflow completion "M1" >/dev/null 2>&1
TC="$(newtask)"
CR="$(jigc doc create completion-record --title "M1" --task "$TC" 2>&1 | grep -oE '^completion-record:[a-z0-9-]+$' | head -1)"
echo "completion-record: $CR (task $TC)"
jigc doc set-field "$CR#meta/verdict" --value green --task "$TC" >/dev/null 2>&1
mkdir -p completions/artifacts/M1 && echo 'the verdict' > completions/artifacts/M1/VERDICT.md
jigc doc set-field "$CR#meta/owner-artifact" --value completions/artifacts/M1/VERDICT.md --task "$TC" >/dev/null 2>&1
jigc doc add-item "$CR#findings" --title "Stray finding" --task "$TC" >/dev/null 2>&1

step jigc doc set-field "$CR#findings/stray-finding/severity" --value HIGH --task "$TC"
HIGH_OUT="$(jigc doc set-field "$CR#findings/stray-finding/severity" --value HIGH --task "$TC" 2>&1)"; HIGH_RC=$?
rec rc12-severity-high-exit "$HIGH_RC"
rec rc12-severity-high-refused-at-field-value-conformant \
    "$(printf '%s' "$HIGH_OUT" | grep -c 'schema-conformance.field-value-conformant')"
bar "rc.12 REFUSES severity HIGH (exit non-zero)" "test $HIGH_RC -ne 0"
bar "…at schema-conformance.field-value-conformant" \
    "printf '%s' \"\$HIGH_OUT\" | grep -q 'schema-conformance.field-value-conformant'"
bar "…and names the enum it holds" "printf '%s' \"\$HIGH_OUT\" | grep -q 'blocking'"

jigc doc set-field "$CR#findings/stray-finding/severity" --value blocking --task "$TC" >/dev/null 2>&1
jigc doc set-field "$CR#findings/stray-finding/disposition" --value fixed --task "$TC" >/dev/null 2>&1
jigc doc set-field "$CR#findings/stray-finding/evidence" --value 'audit.log:12' --task "$TC" >/dev/null 2>&1
jigc doc set-field "commit:$TC#header/type" --value docs --task "$TC" >/dev/null 2>&1
printf 'record the M1 completion\n' | jigc doc set-slot "commit:$TC#summary" --from-file - --task "$TC" >/dev/null 2>&1
step jigc task finalize "$TC"
CR_PATH="$(git ls-files docs/completions/ | head -1)"
bar "the completion-record lands at its home with severity blocking" \
    "test -n '$CR_PATH' && grep -q 'severity: blocking' '$CR_PATH'"
# The stamp is read from the committed FILE, the byte the migration will move. (The
# first run of this arm read `doc show --format json`'s top-level `schema-version`
# and got `undefined`: that key is M49's — rc.12's pinned read carries no such
# top-level number. Recorded here so the second half compares stamps, not contracts.)
step jigc doc show "$CR" --format json
CR_SV="$(sed -n 's/^schema-version: //p' "$CR_PATH" | head -1)"
echo "completion-record schema-version as rc.12 stamps it (file): $CR_SV"
rec rc12-completion-record "$CR"
rec rc12-completion-record-path "$CR_PATH"
rec rc12-completion-record-schema-version "$CR_SV"
bar "the completion-record is stamped schema-version 1 on rc.12" "test '$CR_SV' = 1"

# ---------------------------------------------------------------------------
say "3 · SUBJECT (1) · the milestone boundary over a type-less sub-task commit doc"
# The sub-task's doors run from inside its worktree (a sub-task is pinned to the
# milestone's base and refuses the parent checkout by design). Its commit doc is
# provisioned on the sub-task's first re-entry — the launch line `milestone execute`
# prints — so that line is extracted from the composed output and run VERBATIM,
# rather than typed from memory of what one binary printed.
jigc config set finalize.fan-out.squash false >/dev/null 2>&1
M=bound-the-store
SUB=cap-distinct-series
step jigc milestone create "bound the store"
step jigc milestone add-task "$M" "cap distinct series"
step jigc milestone provision "$M"
SPAWN="$(jigc milestone execute "$M" 2>&1 | sed -n 's/^Spawn: `\(.*\)`$/\1/p' | head -1)"
echo "launch line from \`jigc milestone execute\`: $SPAWN"
rec rc12-spawn-line "$SPAWN"
( eval "$SPAWN" ) >/dev/null 2>&1; echo "[launch line exit $?]"
WT=".jigc/worktrees/$SUB"
( cd "$WT" && echo 'export const cap = 10_000;' > src/cap.ts && git add src/cap.ts )
( cd "$WT" && step jigc doc list --task "$SUB" )
# `summary` filled, `type` deliberately left as the skeleton leaves it
( cd "$WT" && printf 'cap distinct series\n' \
    | jigc doc set-slot "commit:$SUB#summary" --from-file - --task "$SUB" 2>&1; echo "[set-slot exit $?]" )
( cd "$WT" && step jigc task validate "$SUB" )
( cd "$WT" && jigc task validate "$SUB" >/dev/null 2>&1 ); SV_RC=$?
rec rc12-subtask-validate-typeless-exit "$SV_RC"
bar "the TASK door sees the unset type (task validate exits non-zero)" "test $SV_RC -ne 0"

HEAD_BEFORE="$(git rev-parse HEAD)"
# Driven ONCE, captured: a second drive would be a no-op over a settled milestone.
printf '\n$ jigc milestone finalize %s\n' "$M"
MF_OUT="$(jigc milestone finalize "$M" 2>&1)"; MF_EXIT=$?
printf '%s\n[exit %s]\n' "$MF_OUT" "$MF_EXIT"
git log --oneline -3
if [ "$(git rev-parse HEAD)" != "$HEAD_BEFORE" ]; then MF_LANDED=1; else MF_LANDED=0; fi
SUBJ="$(git log --format=%s -3 | grep -E '^[a-z]*: ' | head -1)"
rec rc12-milestone-finalize-typeless-landed "$MF_LANDED"
rec rc12-milestone-finalize-typeless-exit "$MF_EXIT"
rec rc12-milestone-finalize-typeless-subject "$SUBJ"
bar "the d854e25 baseline: rc.12 LANDS the milestone over the type-less commit doc" "test $MF_LANDED -eq 1"
bar "…with a type-less subject (\`: …\`) in the history" "git log --format=%s -3 | grep -q '^: '"
LOG3="$(git log --stat --format= -3)"
bar "…and the sub-task's code is in it" "printf '%s' \"\$LOG3\" | grep -q 'src/cap.ts'"

MR="milestone-record:$M"
step jigc doc show "$MR" --format json
MR_SV="$(sed -n 's/^schema-version: //p' "docs/milestone-records/$M.md" | head -1)"
MR_ST="$(jigc doc show "$MR" --format json 2>/dev/null \
         | node -e 'const d=JSON.parse(require("fs").readFileSync(0,"utf8"));process.stdout.write(String(d.fields.status))')"
echo "milestone-record schema-version as rc.12 stamps it (file): $MR_SV (status $MR_ST)"
rec rc12-milestone "$M"
rec rc12-subtask "$SUB"
rec rc12-milestone-record-path "docs/milestone-records/$M.md"
rec rc12-milestone-record-schema-version "$MR_SV"
rec rc12-milestone-record-status "$MR_ST"
bar "SUBJECT (3) · the milestone-record is committed" "git ls-files --error-unmatch docs/milestone-records/$M.md"
bar "…stamped schema-version 2 on rc.12" "test '$MR_SV' = 2"

# ---------------------------------------------------------------------------
say "4 · the store as rc.12 leaves it"
step jigc validate
jigc validate >/dev/null 2>&1; VRC=$?
rec rc12-validate-exit "$VRC"
rec rc12-head "$(git rev-parse HEAD)"
rec rc12-commit-count "$(git rev-list --count HEAD)"
bar "rc.12 validates its own corpus at exit 0" "test $VRC -eq 0"
bar "the working tree is clean (the baseline is the only untracked file)" \
    "test -z \"\$(git status --porcelain=v1 | grep -v '^?? .upgrade-baseline')\""
bar "no task is left open" "! jigc task list | grep -qE '^  [a-z0-9]'"

# The baseline is deliberately UNTRACKED: it is the rig's note to its second half,
# and committing it would put the instrument inside the thing being measured.
step cat "$B"
bar "the baseline is present" "test -s $B"

say "SUMMARY"
echo "  a corpus authored on rc.12 — two adrs, a research doc, a nested changelog, a"
echo "  completion-record at v1 (HIGH refused, blocking landed) and a milestone whose"
echo "  type-less sub-task commit doc the old boundary landed — every number in"
echo "  .upgrade-baseline for arm 21 to assert the flips against."
if [ "$FAIL" -eq 0 ]; then echo "ARM 14 PASS"; else echo "ARM 14 FAIL"; fi
exit "$FAIL"
