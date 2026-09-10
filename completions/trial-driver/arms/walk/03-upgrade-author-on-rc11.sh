#!/usr/bin/env bash
# 03a-upgrade-author-on-rc11.sh — protocol.md §5 arm 03, FIRST HALF.
#
# THE ONLY ARM THAT NEEDS TWO BINARIES, so it is the only one walk.py cannot drive
# in a single pass. The sequence, and it must be run in this order:
#
#   python3 walk.py <corpus> <out-a> --tag jigc-gate:rc11 --only 03a
#   python3 run.py  carry <out-a>/03a-upgrade-author-on-rc11 <corpus-b>
#   python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 03b
#
# `run.py carry` is load-bearing between them: `run-session.sh` writes its own
# evidence into the out-dir, and handing that straight on plants the rig's
# droppings in the corpus the second half reads.
#
# WHY THIS ARM EXISTS AT ALL: this is the real 1.0.0 upgrade path, and it is
# covered by nothing else in the suite — every fixture builder constructs its
# states by driving the CURRENT binary, so no test has ever seen a corpus authored
# by an older one.
#
# THIS HALF authors a mixed corpus on rc.11 and records the PRE-CHANGE baseline for
# the two declared behaviour changes, so the second half's assertions are measured
# against something rather than remembered.
set -uo pipefail
cd /work

say()  { printf '\n=== %s\n' "$1"; }
FAIL=0
step() { printf '\n$ %s\n' "$*"; "$@" 2>&1; printf '[exit %s]\n' "$?"; }
bar()  { if eval "$2" >/dev/null 2>&1; then echo "  OK    $1"; else echo "  FAIL  $1"; FAIL=1; fi; }
newtask() { jigc task list | awk '/^  [a-z]/{print $1; exit}'; }

# PRECONDITION, checked rather than assumed. Every other arm in this directory
# runs against the tag under test; this one must run against the OLD binary. A
# full-walk pass will therefore hand it the new one, and it SKIPS loudly rather
# than failing — an arm that cannot run is not an arm that found something.
if ! jigc --version | grep -q 'rc.11'; then
  cat <<'SKIP'

=== SKIPPED — this arm needs the OLD binary, and this pass is not it

The upgrade arm is the only two-binary arm, so it is driven as an explicit
sequence rather than by a normal walk pass:

  python3 walk.py <corpus>   <out-a> --tag jigc-gate:rc11 --only 03
  python3 run.py  carry <out-a>/03-upgrade-author-on-rc11 <corpus-b>
  python3 walk.py <corpus-b> <out-b> --tag jigc-gate:rc12 --only 10

`run.py carry` between the halves is load-bearing: run-session.sh writes its own
evidence into the out-dir, and handing that straight on would plant the rig's
droppings in the corpus the second half reads.
SKIP
  echo "ARM 03 SKIPPED (needs jigc-gate:rc11; got $(jigc --version))"
  exit 0
fi

say "0 · adopt with the OLD binary"
jigc setup >/dev/null 2>&1
jigc config set invocation-log true >/dev/null 2>&1
step jigc --version
bar "this half really is running rc.11" "jigc --version | grep -q 'rc.11'"

say "1 · author a mixed corpus THROUGH the old binary"
land_adr() {
  jigc start --workflow record-decision "record $1" >/dev/null 2>&1
  local t a s; t="$(newtask)"
  a="$(jigc doc create adr --title "$1" --task "$t" 2>&1 | grep -oE '^adr:[a-z0-9-]+$' | head -1)"
  for s in context decision consequences; do
    echo 'Authored on rc.11, before the upgrade.' \
      | jigc doc set-slot "$a#$s" --from-file - --task "$t" >/dev/null 2>&1
  done
  jigc doc set-field "commit:$t#header/type" --value docs --task "$t" >/dev/null 2>&1
  printf 'record %s\n' "$1" | jigc doc set-slot "commit:$t#summary" --from-file - --task "$t" >/dev/null 2>&1
  jigc task finalize "$t" >/dev/null 2>&1
}
land_adr 'Drop the oldest sample on overflow'
land_adr 'Cap distinct series at a ceiling'

# a research doc — a different doctype, a different home
jigc start --workflow do-research "how other buffers shed load" >/dev/null 2>&1
TR="$(newtask)"
RA="$(jigc doc create research --title 'How other buffers shed load' --task "$TR" 2>&1 \
      | grep -oE '^research:[a-z0-9-]+$' | head -1)"
for s in question findings sources; do
  echo 'Authored on rc.11.' | jigc doc set-slot "$RA#$s" --from-file - --task "$TR" >/dev/null 2>&1
done
jigc doc set-field "commit:$TR#header/type" --value docs --task "$TR" >/dev/null 2>&1
printf 'record the buffer research\n' | jigc doc set-slot "commit:$TR#summary" --from-file - --task "$TR" >/dev/null 2>&1
jigc task finalize "$TR" >/dev/null 2>&1

# a milestone record — the doctype whose lifecycle spans commits
jigc milestone create "bound the store" >/dev/null 2>&1
jigc milestone add-task bound-the-store "cap distinct series" >/dev/null 2>&1

step git log --oneline
bar "the corpus carries adrs"            "test \$(git ls-files docs/decisions/ | wc -l | tr -d ' ') -ge 2"
bar "…a research doc"                    "test \$(git ls-files docs/research/ | wc -l | tr -d ' ') -ge 1"
bar "…and a milestone record"            "test \$(git ls-files docs/milestone-records/ | wc -l | tr -d ' ') -ge 1"
bar "the working tree is clean"          "test -z \"\$(git status --porcelain=v1)\""

say "2 · the PRE-CHANGE baseline for the two declared behaviour changes"
# Recorded on the old binary so the second half compares against a measurement.
# ORDER MATTERS, and getting it wrong cost a run: the §0.1 setup plants a foreign
# CHANGELOG.md, whose unadopted advisory rides the TASK door too. Measured after
# that plant, this cell sees a finding and reads as "rc.11 was not clean either" —
# a false baseline that would have made the second half's comparison meaningless.
# So the clean-task baseline is taken FIRST, on an untouched corpus.
jigc start --workflow single-task "add a rate limiter" >/dev/null 2>&1
TS="$(newtask)"
jigc doc set-field "commit:$TS#header/type" --value feat --task "$TS" >/dev/null 2>&1
jigc doc set-field "commit:$TS#header/scope" --value ingest --task "$TS" >/dev/null 2>&1
printf 'add a rate limiter\n' | jigc doc set-slot "commit:$TS#summary" --from-file - --task "$TS" >/dev/null 2>&1
step jigc task validate "$TS"
TV="$(jigc task validate "$TS" 2>&1)"
echo "rc11-task-validate-clean=$(printf '%s' "$TV" | grep -c 'no findings')" > /work/.upgrade-baseline
bar "§0.2 baseline: on rc.11 a conformant task validates CLEAN" \
    "printf '%s' \"\$TV\" | grep -q 'no findings'"
# `--force` is the consent this door refuses without since M50 Increment 3: minting a
# task stages its commit doc, so an ordinary discard is refused from the moment the task
# exists (`task-discard.staged-prose`). This is CLEANUP, not a subject under test — the
# refusal itself is driven, both sides, in pre-trial-findings.md PT-8.
jigc task discard "$TS" --force >/dev/null 2>&1

printf '# Changelog\n\n## [0.1.0] - 2026-01-04\n### Added\n- first release\n' > CHANGELOG.md
git add CHANGELOG.md
git -c user.name='Corpus Owner' -c user.email='owner@example.invalid' \
    commit -qm 'docs: a foreign changelog' -- CHANGELOG.md

step jigc validate
jigc validate >/dev/null 2>&1; VRC=$?
echo "rc11-validate-exit=$VRC" >> /work/.upgrade-baseline
bar "§0.1 baseline: on rc.11 a foreign squatter validates at EXIT 0" "test $VRC -eq 0"

# The baseline file is deliberately left UNTRACKED: it is the rig's note to its
# own second half, not part of the corpus under test, and committing it would put
# the instrument inside the thing being measured.
step cat /work/.upgrade-baseline

say "SUMMARY"
echo "  a mixed corpus authored on rc.11, with both declared changes measured BEFORE."
if [ "$FAIL" -eq 0 ]; then echo "ARM 03a PASS"; else echo "ARM 03a FAIL"; fi
exit "$FAIL"
